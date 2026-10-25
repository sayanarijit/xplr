#![allow(clippy::too_many_arguments)]

use crate::app;
use crate::app::ExternalMsg;
use crate::cli::Cli;
use crate::event_reader::EventReader;
use crate::explorer;
use crate::lua;
use crate::pipe;
use crate::pwd_watcher::PwdWatcher;
use crate::terminal;
use crate::terminal::get_tty;
use crate::ui::NO_COLOR;
use crate::ui::UI;
use crate::yaml;
use anyhow::{Error, Result};
use mlua::LuaSerdeExt;
use mlua::Value;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::mpsc;
use tui::Terminal;
use tui::backend::CrosstermBackend;
use tui::crossterm::event;
use tui::crossterm::execute;
use tui_input::Input;

fn start_fifo(path: &str, focus_path: &str) -> Result<fs::File> {
    match fs::OpenOptions::new().write(true).open(path) {
        Ok(mut file) => {
            writeln!(file, "{focus_path}")?;
            Ok(file)
        }
        Err(e) => Err(e.into()),
    }
}

pub struct Runner {
    app: app::App,
    lua: mlua::Lua,
    ui: UI,
    print_pwd_as_result: bool,
    default_delimiter: char,
    pwd_watcher: PwdWatcher,
    event_reader: EventReader,
    task_sender: mpsc::Sender<app::Task>,
    task_receiver: mpsc::Receiver<app::Task>,
    terminal: Terminal<CrosstermBackend<File>>,
    mouse_enabled: bool,
    fifo: Option<fs::File>,
}

impl Runner {
    /// Create a new runner object passing the default arguments
    pub fn new() -> Result<Self> {
        Self::from_cli(Default::default())
    }

    /// Create a new runner object passing the given arguments
    pub fn from_cli(cli: Cli) -> Result<Self> {
        let currdir = crate::dirs::get_current_dir()?;
        let mut paths = cli.paths.into_iter();
        let mut pwd = paths
            .next()
            .or_else(|| cli.vroot.clone())
            .unwrap_or_else(|| currdir.clone());
        let mut focused_path: Option<PathBuf> = None;

        if cli.force_focus || pwd.is_file() {
            focused_path = pwd.file_name().map(|p| p.into());
            pwd = pwd.parent().map(|p| p.into()).unwrap_or(currdir);
        }

        let (task_sender, task_receiver) = mpsc::channel();
        let pwd_watcher = PwdWatcher::new(task_sender.clone());
        let event_reader = EventReader::new(task_sender.clone());

        let mut terminal = terminal::start()?;

        let lua = unsafe { mlua::Lua::unsafe_new() }; // Safety: https://github.com/sayanarijit/xplr/issues/309

        let ui = UI::new(task_sender.clone());
        let mut app = app::App::create(
            cli.bin.clone(),
            cli.vroot.clone(),
            pwd.clone(),
            &lua,
            cli.config.clone(),
            cli.extra_config.clone(),
        )?;

        if !app.config.general.read_only {
            app.config.general.read_only = cli.read_only;
        }

        app.explore_pwd()?;
        for path in paths {
            app.select_path(path.to_string_lossy().to_string())?;
        }

        if let Some(f) = focused_path
            .clone()
            .map(|f| f.to_string_lossy().to_string())
        {
            app.focus_by_file_name(&f, true)?
        } else {
            app.focus_first(true)?
        };

        explorer::explore_recursive_async(
            app.explorer_config.clone(),
            app.pwd.clone().into(),
            focused_path.clone(),
            app.directory_buffer.as_ref().map(|d| d.focus).unwrap_or(0),
            task_sender.clone(),
        );

        let fifo: Option<fs::File> =
            if let Some(path) = app.config.general.start_fifo.clone() {
                // TODO remove duplicate segment
                match start_fifo(&path, &app.focused_node_str()) {
                    Ok(file) => Some(file),
                    Err(e) => {
                        app.log_error(format!("could not start fifo {path:?}: {e}"))?;
                        None
                    }
                }
            } else {
                None
            };

        terminal.hide_cursor()?;
        // Clear AND redraw the terminal surface
        terminal::clear(&mut terminal)?;

        // Enqueue on_load messages
        for msg in app.hooks.on_load.iter().chain(cli.on_load.iter()) {
            task_sender.send(app::Task::new(app::MsgIn::External(msg.clone()), None))?;
        }

        let mut runner = Self {
            app,
            lua,
            ui,
            print_pwd_as_result: cli.print_pwd_as_result,
            default_delimiter: if cli.write0 { '\0' } else { '\n' },
            task_sender,
            task_receiver,
            pwd_watcher,
            event_reader,
            terminal,
            mouse_enabled: false,
            fifo,
        };

        runner.start()?;
        Ok(runner)
    }

    fn try_enable_mouse(&mut self) -> Result<()> {
        match execute!(self.terminal.backend_mut(), event::EnableMouseCapture) {
            Ok(_) => {
                self.mouse_enabled = true;
            }
            Err(e) => {
                self.mouse_enabled = false;
                self.app.log_error(format!("could not enable mouse: {e}"))?;
            }
        }
        Ok(())
    }

    fn try_disable_mouse(&mut self) -> Result<()> {
        match execute!(self.terminal.backend_mut(), event::DisableMouseCapture) {
            Ok(_) => {
                self.mouse_enabled = false;
            }
            Err(e) => {
                self.mouse_enabled = true;
                self.app
                    .log_error(format!("could not disable mouse: {e}"))?;
            }
        }
        Ok(())
    }

    fn resume_mouse_if_enabled(&mut self) -> Result<()> {
        if self.mouse_enabled {
            self.try_enable_mouse()?;
        }
        Ok(())
    }

    /// Run the instance
    pub fn run(mut self) -> Result<Option<String>> {
        let mut result = Ok(None);
        let mut last_focus: Option<app::Node> = None;
        let mut last_pwd = self.app.pwd.clone();

        'outer: while let Ok(task) = self.task_receiver.recv() {
            match self.app.handle_task(task) {
                Ok(()) => {
                    while let Some(msg) = self.app.msg_out.pop_front() {
                        use app::MsgOut::*;
                        match msg {
                            Enqueue(task) => {
                                self.task_sender.send(task)?;
                            }

                            Quit => {
                                result = Ok(None);
                                break 'outer;
                            }

                            PrintPwdAndQuit => {
                                result =
                                    Ok(Some(self.app.pwd_str(self.default_delimiter)));
                                break 'outer;
                            }

                            PrintFocusPathAndQuit => {
                                result = Ok(self.app.focused_node().map(|n| {
                                    format!(
                                        "{}{}",
                                        n.absolute_path, self.default_delimiter
                                    )
                                }));
                                break 'outer;
                            }

                            PrintSelectionAndQuit => {
                                result = Ok(Some(
                                    self.app.selection_str(self.default_delimiter),
                                ));
                                break 'outer;
                            }

                            PrintResultAndQuit => {
                                result = if self.print_pwd_as_result {
                                    Ok(Some(self.app.pwd_str(self.default_delimiter)))
                                } else {
                                    Ok(Some(self.app.result_str(self.default_delimiter)))
                                };

                                break 'outer;
                            }

                            PrintAppStateAndQuit => {
                                let out = yaml::to_string(&self.app)?;
                                result = Ok(Some(out));
                                break 'outer;
                            }

                            Debug(path) => {
                                fs::write(path, yaml::to_string(&self.app)?)?;
                            }

                            ClearScreen => {
                                // Clear AND redraw the terminal surface
                                terminal::clear(&mut self.terminal)?;
                            }

                            ScrollUp => {
                                self.app.focus_previous_by_relative_index(
                                    self.terminal.size()?.height as usize,
                                )?;
                            }

                            ScrollDown => {
                                self.app.focus_next_by_relative_index(
                                    self.terminal.size()?.height as usize,
                                )?;
                            }

                            ScrollUpHalf => {
                                self.app.focus_previous_by_relative_index(
                                    self.terminal.size()?.height as usize / 2,
                                )?;
                            }

                            ScrollDownHalf => {
                                self.app.focus_next_by_relative_index(
                                    self.terminal.size()?.height as usize / 2,
                                )?;
                            }

                            ExplorePwdAsync => {
                                explorer::explore_async(
                                    self.app.explorer_config.clone(),
                                    self.app.pwd.clone().into(),
                                    self.app
                                        .focused_node()
                                        .map(|n| n.relative_path.clone().into()),
                                    self.app
                                        .directory_buffer
                                        .as_ref()
                                        .map(|d| d.focus)
                                        .unwrap_or(0),
                                    self.task_sender.clone(),
                                );
                                self.pwd_watcher.update_pwd(self.app.pwd.as_ref());
                            }

                            ExploreParentsAsync => {
                                explorer::explore_recursive_async(
                                    self.app.explorer_config.clone(),
                                    self.app.pwd.clone().into(),
                                    self.app
                                        .focused_node()
                                        .map(|n| n.relative_path.clone().into()),
                                    self.app
                                        .directory_buffer
                                        .as_ref()
                                        .map(|d| d.focus)
                                        .unwrap_or(0),
                                    self.task_sender.clone(),
                                );
                                self.pwd_watcher.update_pwd(self.app.pwd.as_ref());
                            }

                            Refresh => {
                                let focus = self.app.focused_node();
                                if focus != last_focus.as_ref() {
                                    last_focus = focus.cloned();

                                    // Fifo
                                    if let Some(ref mut file) = self.fifo {
                                        writeln!(
                                            file,
                                            "{}",
                                            self.app.focused_node_str()
                                        )?;
                                    };

                                    // Hooks
                                    if !self.app.hooks.on_focus_change.is_empty() {
                                        let msgs =
                                            self.app.hooks.on_focus_change.clone();
                                        self.app.handle_batch_external_msgs(msgs)?
                                    }
                                }

                                if self.app.pwd != last_pwd {
                                    last_pwd.clone_from(&self.app.pwd);

                                    // $PWD watcher
                                    self.pwd_watcher.update_pwd(self.app.pwd.as_ref());

                                    // OSC 7: Change CWD
                                    if !(*NO_COLOR) {
                                        write!(
                                            self.terminal.backend_mut(),
                                            "\x1b]7;file://{}{}\x1b\\",
                                            self.app.hostname,
                                            self.app.pwd
                                        )?;
                                    }

                                    // Hooks
                                    if !self.app.hooks.on_directory_change.is_empty() {
                                        let msgs =
                                            self.app.hooks.on_directory_change.clone();
                                        self.app.handle_batch_external_msgs(msgs)?
                                    }
                                }

                                // UI
                                self.terminal
                                    .draw(|f| self.ui.draw(f, &self.app, &self.lua))?;
                            }

                            EnableMouse => {
                                self.try_enable_mouse()?;
                            }

                            ToggleMouse => {
                                if self.mouse_enabled {
                                    self.try_disable_mouse()?;
                                } else {
                                    self.try_enable_mouse()?;
                                };
                            }

                            DisableMouse => {
                                self.try_disable_mouse()?;
                            }

                            StartFifo(path) => {
                                self.fifo =
                                    match start_fifo(&path, &self.app.focused_node_str())
                                    {
                                        Ok(file) => Some(file),
                                        Err(e) => {
                                            self.app.log_error(format!(
                                                "could not start fifo {path:?}: {e}"
                                            ))?;
                                            None
                                        }
                                    }
                            }

                            StopFifo => {
                                if let Some(file) = self.fifo {
                                    self.fifo = None;
                                    std::mem::drop(file);
                                }
                            }

                            ToggleFifo(path) => {
                                if let Some(file) = self.fifo {
                                    self.fifo = None;
                                    std::mem::drop(file);
                                } else {
                                    self.fifo = match start_fifo(
                                        &path,
                                        &self.app.focused_node_str(),
                                    ) {
                                        Ok(file) => Some(file),
                                        Err(e) => {
                                            self.app.log_error(format!(
                                                "could not toggle fifo {path:?}: {e}"
                                            ))?;
                                            None
                                        }
                                    }
                                }
                            }

                            CallLuaSilently(func) => {
                                match self.call_lua_heavy(&func, false) {
                                    Ok(Some(msgs)) => {
                                        self.app.handle_batch_external_msgs(msgs)?;
                                    }
                                    Ok(None) => {}
                                    Err(err) => {
                                        self.app.log_error(err.to_string())?;
                                    }
                                };
                            }

                            CallLua(func) => {
                                self.pause()?;
                                match self.call_lua_heavy(&func, false) {
                                    Ok(Some(msgs)) => {
                                        self.app.handle_batch_external_msgs(msgs)?;
                                    }
                                    Ok(None) => {}
                                    Err(err) => {
                                        self.app.log_error(err.to_string())?;
                                    }
                                };
                                self.resume()?;
                            }

                            LuaEval(code) => {
                                self.pause()?;
                                let res: Result<Value> =
                                    self.lua.load(&code).eval().map_err(Error::from);

                                match res {
                                    Ok(Value::Function(f)) => {
                                        let arg = self.app.to_lua_ctx_heavy();
                                        let res: Result<Option<Vec<ExternalMsg>>> = self
                                            .lua
                                            .to_value(&arg)
                                            .and_then(|a| f.call(a))
                                            .and_then(|v| self.lua.from_value(v))
                                            .map_err(Error::from);
                                        match res {
                                            Ok(Some(msgs)) => {
                                                self.app
                                                    .handle_batch_external_msgs(msgs)?;
                                            }
                                            Ok(None) => {}
                                            Err(err) => {
                                                self.app.log_error(err.to_string())?;
                                            }
                                        }
                                    }
                                    Ok(v) => {
                                        let res: Result<Option<Vec<ExternalMsg>>> =
                                            self.lua.from_value(v).map_err(Error::from);
                                        match res {
                                            Ok(Some(msgs)) => {
                                                self.app
                                                    .handle_batch_external_msgs(msgs)?;
                                            }
                                            Ok(None) => {}
                                            Err(err) => {
                                                self.app.log_error(err.to_string())?;
                                            }
                                        }
                                    }
                                    Err(err) => {
                                        self.app.log_error(err.to_string())?;
                                    }
                                };
                                self.resume()?;
                            }

                            LuaEvalSilently(code) => {
                                let res: Result<Value> =
                                    self.lua.load(&code).eval().map_err(Error::from);

                                match res {
                                    Ok(Value::Function(f)) => {
                                        let arg = self.app.to_lua_ctx_heavy();
                                        let res: Result<Option<Vec<ExternalMsg>>> = self
                                            .lua
                                            .to_value(&arg)
                                            .and_then(|a| f.call(a))
                                            .and_then(|v| self.lua.from_value(v))
                                            .map_err(Error::from);
                                        match res {
                                            Ok(Some(msgs)) => {
                                                self.app
                                                    .handle_batch_external_msgs(msgs)?;
                                            }
                                            Ok(None) => {}
                                            Err(err) => {
                                                self.app.log_error(err.to_string())?;
                                            }
                                        }
                                    }
                                    Ok(v) => {
                                        let res: Result<Option<Vec<ExternalMsg>>> =
                                            self.lua.from_value(v).map_err(Error::from);
                                        match res {
                                            Ok(Some(msgs)) => {
                                                self.app
                                                    .handle_batch_external_msgs(msgs)?;
                                            }
                                            Ok(None) => {}
                                            Err(err) => {
                                                self.app.log_error(err.to_string())?;
                                            }
                                        }
                                    }
                                    Err(err) => {
                                        self.app.log_error(err.to_string())?;
                                    }
                                };
                            }

                            Call(cmd) => {
                                self.call(cmd, false, '\n')?;
                            }

                            Call0(cmd) => {
                                self.call(cmd, false, '\0')?;
                            }

                            CallSilently(cmd) => {
                                self.call(cmd, true, '\n')?;
                            }

                            CallSilently0(cmd) => {
                                self.call(cmd, true, '\0')?;
                            }
                        };
                    }
                }

                Err(e) => {
                    result = Err(e);
                    break;
                }
            }
        }

        self.stop()?;
        result
    }

    fn call_lua_heavy(
        &self,
        func: &str,
        _silent: bool,
    ) -> Result<Option<Vec<app::ExternalMsg>>> {
        let arg = self.app.to_lua_ctx_heavy();
        let arg = lua::serialize(&self.lua, &arg)?;
        lua::call(&self.lua, func, arg)
    }

    fn call(&mut self, cmd: app::Command, silent: bool, delimiter: char) -> Result<()> {
        if !silent {
            self.pause()?;
        }

        self.write_pipes(delimiter)?;
        let focus_index = self
            .app
            .directory_buffer
            .as_ref()
            .map(|d| d.focus)
            .unwrap_or_default()
            .to_string();

        let (stdin, stdout, stderr) = if silent {
            (Stdio::null(), Stdio::null(), Stdio::null())
        } else {
            (get_tty()?.into(), get_tty()?.into(), get_tty()?.into())
        };

        let input_buffer = self
            .app
            .input
            .buffer
            .as_ref()
            .map(Input::to_string)
            .unwrap_or_default();

        let mut command = Command::new(cmd.command.clone());
        command
            .env("XPLR", &self.app.bin)
            .env("XPLR_VROOT", self.app.vroot.clone().unwrap_or_default())
            .env("XPLR_APP_VERSION", &self.app.version)
            .env("XPLR_PID", self.app.pid.to_string())
            .env("XPLR_INPUT_BUFFER", input_buffer)
            .env("XPLR_INITIAL_PWD", &self.app.initial_pwd)
            .env("XPLR_FOCUS_PATH", self.app.focused_node_str())
            .env("XPLR_FOCUS_INDEX", focus_index)
            .env("XPLR_SESSION_PATH", &self.app.session_path)
            .env("XPLR_PIPE_MSG_IN", &self.app.pipe.msg_in)
            .env("XPLR_PIPE_SELECTION_OUT", &self.app.pipe.selection_out)
            .env("XPLR_PIPE_HISTORY_OUT", &self.app.pipe.history_out)
            .env("XPLR_MODE", self.app.mode_str())
            .env("XPLR_PIPE_RESULT_OUT", &self.app.pipe.result_out)
            .env(
                "XPLR_PIPE_GLOBAL_HELP_MENU_OUT",
                &self.app.pipe.global_help_menu_out,
            )
            .env(
                "XPLR_PIPE_DIRECTORY_NODES_OUT",
                &self.app.pipe.directory_nodes_out,
            )
            .env("XPLR_PIPE_LOGS_OUT", &self.app.pipe.logs_out)
            .stdin(stdin)
            .stdout(stdout)
            .stderr(stderr)
            .args(cmd.args);

        // Run silent commands in their own process group so that processes they
        // leave behind (e.g. `cmd &`) are not in xplr's foreground process group
        // and survive the terminal emulator closing after xplr quits (see #764).
        // Interactive commands keep xplr's process group so terminal signals
        // like Ctrl-C are still delivered to them.
        #[cfg(unix)]
        if silent {
            use std::os::unix::process::CommandExt;
            command.process_group(0);
        }

        let status = command
            .status()
            .map(|s| {
                if s.success() {
                    Ok(())
                } else {
                    Err(format!("process exited with code {s}"))
                }
            })
            .unwrap_or_else(|e| Err(e.to_string()));

        match pipe::read_all(&self.app.pipe.msg_in, delimiter) {
            Ok(msgs) => {
                self.app.handle_batch_external_msgs(msgs)?;
            }
            Err(err) => {
                self.app.log_error(err.to_string())?;
            }
        };

        self.cleanup_pipes()?;
        if let Err(e) = status {
            self.app.log_error(e)?;
        };

        if !silent {
            self.resume()?;
        }

        Ok(())
    }

    fn start_threads(&mut self) -> Result<()> {
        self.event_reader.start();
        self.pwd_watcher.start(&self.app.pwd);
        Ok(())
    }

    fn stop_threads(&mut self) -> Result<()> {
        self.pwd_watcher.stop_async();
        self.event_reader.stop();
        Ok(())
    }

    fn pause(&mut self) -> Result<()> {
        self.stop_threads()?;
        terminal::stop(&mut self.terminal, false)?;
        Ok(())
    }

    fn resume(&mut self) -> Result<()> {
        terminal::resume(&mut self.terminal, false)?;
        self.resume_mouse_if_enabled()?;
        self.start_threads()?;
        Ok(())
    }

    fn start(&mut self) -> Result<()> {
        fs::create_dir_all(&self.app.session_path)?;
        self.start_threads()?;
        Ok(())
    }

    fn stop(mut self) -> Result<()> {
        self.stop_threads()?;
        terminal::stop(&mut self.terminal, true)?;
        fs::remove_dir_all(&self.app.session_path)?;
        Ok(())
    }

    pub fn write_pipes(&self, delimiter: char) -> Result<()> {
        fs::create_dir_all(self.app.pipe.path.clone())?;
        fs::write(&self.app.pipe.msg_in, [delimiter as u8])?;

        let selection_str = self.app.selection_str(delimiter);
        fs::write(&self.app.pipe.selection_out, selection_str)?;

        let history_str = self.app.history_str(delimiter);
        fs::write(&self.app.pipe.history_out, history_str)?;

        let directory_nodes_str = self.app.directory_nodes_str(delimiter);
        fs::write(&self.app.pipe.directory_nodes_out, directory_nodes_str)?;

        let logs_str = self.app.logs_str(delimiter);
        fs::write(&self.app.pipe.logs_out, logs_str)?;

        let result_str = self.app.result_str(delimiter);
        fs::write(&self.app.pipe.result_out, result_str)?;

        let global_help_menu_str = self.app.global_help_menu_str(delimiter);
        fs::write(&self.app.pipe.global_help_menu_out, global_help_menu_str)?;

        Ok(())
    }

    pub fn cleanup_pipes(&self) -> Result<()> {
        while !fs::read_to_string(&self.app.pipe.msg_in)?.is_empty() {
            std::thread::sleep(std::time::Duration::from_millis(1));
        }

        fs::remove_file(&self.app.pipe.msg_in)?;
        fs::remove_file(&self.app.pipe.selection_out)?;
        fs::remove_file(&self.app.pipe.result_out)?;
        fs::remove_file(&self.app.pipe.directory_nodes_out)?;
        fs::remove_file(&self.app.pipe.global_help_menu_out)?;
        fs::remove_file(&self.app.pipe.logs_out)?;
        fs::remove_file(&self.app.pipe.history_out)?;
        fs::remove_dir(&self.app.pipe.path)?;
        Ok(())
    }
}

/// Create a new runner object passing the default arguments
pub fn runner() -> Result<Runner> {
    Runner::new()
}

/// Create a new runner object passing the given arguments
pub fn from_cli(cli: Cli) -> Result<Runner> {
    Runner::from_cli(cli)
}
