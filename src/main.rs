#![allow(clippy::too_many_arguments)]

use anyhow::Result;
use crossterm::execute;
use crossterm::terminal as term;
use handlebars::Handlebars;
use std::env;
use std::fs;
use std::io;
use std::io::prelude::*;
use std::path::PathBuf;
use std::process::{Command, ExitStatus, Stdio};
use std::sync::mpsc;
use termion::get_tty;
use tui::backend::CrosstermBackend;
use tui::Terminal;
use xplr::app;
use xplr::auto_refresher;
use xplr::event_reader;
use xplr::explorer;
use xplr::pipe_reader;
use xplr::pwd_watcher;
use xplr::ui;

fn call(app: &app::App, cmd: app::Command, silent: bool) -> io::Result<ExitStatus> {
    let input_buffer = app.input_buffer().unwrap_or_default();

    let focus_index = app
        .directory_buffer()
        .map(|d| d.focus)
        .unwrap_or_default()
        .to_string();

    let pipe_msg_in = app.pipe().msg_in.clone();
    let pipe_mode_out = app.pipe().mode_out.clone();
    let pipe_focus_out = app.pipe().focus_out.clone();
    let pipe_selection_out = app.pipe().selection_out.clone();
    let pipe_result_out = app.pipe().result_out.clone();
    let pipe_directory_nodes_out = app.pipe().directory_nodes_out.clone();
    let pipe_global_help_menu_out = app.pipe().global_help_menu_out.clone();
    let pipe_logs_out = app.pipe().logs_out.clone();
    let session_path = app.session_path();

    let (stdin, stdout, stderr) = if silent {
        (Stdio::null(), Stdio::null(), Stdio::null())
    } else {
        (Stdio::inherit(), Stdio::inherit(), Stdio::inherit())
    };

    Command::new(cmd.command.clone())
        .current_dir(app.pwd())
        .env("XPLR_APP_VERSION", app.version())
        .env("XPLR_CONFIG_VERSION", &app.config().version)
        .env("XPLR_PID", &app.pid().to_string())
        .env("XPLR_INPUT_BUFFER", input_buffer)
        .env("XPLR_FOCUS_PATH", app.focused_node_str())
        .env("XPLR_FOCUS_INDEX", focus_index)
        .env("XPLR_SESSION_PATH", session_path)
        .env("XPLR_PIPE_MSG_IN", pipe_msg_in)
        .env("XPLR_PIPE_SELECTION_OUT", pipe_selection_out)
        .env("XPLR_PIPE_FOCUS_OUT", pipe_focus_out)
        .env("XPLR_PIPE_MODE_OUT", pipe_mode_out)
        .env("XPLR_PIPE_RESULT_OUT", pipe_result_out)
        .env("XPLR_PIPE_GLOBAL_HELP_MENU_OUT", pipe_global_help_menu_out)
        .env("XPLR_PIPE_DIRECTORY_NODES_OUT", pipe_directory_nodes_out)
        .env("XPLR_PIPE_LOGS_OUT", pipe_logs_out)
        .stdin(stdin)
        .stdout(stdout)
        .stderr(stderr)
        .args(cmd.args)
        .status()
}

fn main() -> Result<()> {
    let (tx_msg_in, rx_msg_in) = mpsc::channel();
    let (tx_event_reader, rx_event_reader) = mpsc::channel();
    let (tx_pwd_watcher, rx_pwd_watcher) = mpsc::channel();

    let mut pwd = PathBuf::from(env::args().nth(1).unwrap_or_else(|| ".".into()))
        .canonicalize()
        .unwrap_or_default();
    let mut focused_path = None;

    if pwd.is_file() {
        focused_path = Some(
            pwd.file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string(),
        );
        pwd = pwd.parent().map(|p| p.into()).unwrap_or_default();
    }

    let mut app = app::App::create(pwd)?;

    if app.version() != &app.config().version {
        let msg = format!(
            "version mismatch, to update your config file to {}, visit {}",
            app.version(),
            app::UPGRADE_GUIDE_LINK,
        );

        tx_msg_in.send(app::Task::new(
            app::MsgIn::External(app::ExternalMsg::LogInfo(msg)),
            None,
        ))?;
    };

    fs::write(&app.pipe().global_help_menu_out, app.global_help_menu_str())?;

    explorer::explore(
        app.explorer_config().clone(),
        app.pwd().clone(),
        focused_path,
        tx_msg_in.clone(),
    );

    let mut hb = Handlebars::new();
    hb.register_template_string(
        app::TEMPLATE_TABLE_ROW,
        &app.config()
            .general
            .table
            .row
            .cols
            .iter()
            .map(|c| c.format.to_string())
            .collect::<Vec<String>>()
            .join("\t"),
    )?;

    let mut result = Ok(());
    let mut output = None;

    term::enable_raw_mode()?;
    let mut stdout = get_tty()?;
    // let mut stdout = stdout.lock();
    execute!(stdout, term::EnterAlternateScreen)?;
    // let stdout = MouseTerminal::from(stdout);
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    // Threads
    auto_refresher::start_auto_refreshing(tx_msg_in.clone());
    pipe_reader::keep_reading(app.pipe().msg_in.clone(), tx_msg_in.clone());
    event_reader::keep_reading(tx_msg_in.clone(), rx_event_reader);
    pwd_watcher::keep_watching(app.pwd(), tx_msg_in.clone(), rx_pwd_watcher)?;

    'outer: for task in rx_msg_in {
        let last_app = app.clone();

        let (new_app, new_result) = match app.handle_task(task) {
            Ok(a) => (a, Ok(())),
            Err(err) => (last_app.clone(), Err(err)),
        };

        app = new_app;
        result = new_result;

        if result.is_err() {
            break;
        }

        while let Some(msg) = app.pop_msg_out() {
            match msg {
                app::MsgOut::Enque(task) => {
                    tx_msg_in.send(task)?;
                }

                app::MsgOut::PrintResultAndQuit => {
                    output = Some(app.result_str());
                    break 'outer;
                }

                app::MsgOut::PrintAppStateAndQuit => {
                    let out = serde_yaml::to_string(&app)?;
                    output = Some(out);
                    break 'outer;
                }

                app::MsgOut::Debug(path) => {
                    fs::write(&path, serde_yaml::to_string(&app)?)?;
                }

                app::MsgOut::ClearScreen => {
                    terminal.clear()?;
                }

                app::MsgOut::Explore => {
                    explorer::explore(
                        app.explorer_config().clone(),
                        app.pwd().clone(),
                        app.focused_node().map(|n| n.relative_path.clone()),
                        tx_msg_in.clone(),
                    );
                }

                app::MsgOut::Refresh => {
                    app = app.refresh_selection()?;
                    if app.pwd() != last_app.pwd() {
                        tx_pwd_watcher.send(app.pwd().clone())?;
                        explorer::explore(
                            app.explorer_config().clone(),
                            app.pwd().clone(),
                            app.focused_node().map(|n| n.relative_path.clone()),
                            tx_msg_in.clone(),
                        );
                    };

                    // UI
                    terminal.draw(|f| ui::draw(f, &app, &hb))?;
                }

                app::MsgOut::CallSilently(cmd) => {
                    tx_event_reader.send(true)?;

                    let status = call(&app, cmd, false)
                        .map(|s| {
                            if s.success() {
                                Ok(())
                            } else {
                                Err(format!("process exited with code {}", &s))
                            }
                        })
                        .unwrap_or_else(|e| Err(e.to_string()));

                    if let Err(e) = status {
                        let msg = app::MsgIn::External(app::ExternalMsg::LogError(e));
                        tx_msg_in.send(app::Task::new(msg, None))?;
                    };

                    tx_event_reader.send(false)?;
                }

                app::MsgOut::Call(cmd) => {
                    tx_event_reader.send(true)?;

                    terminal.clear()?;
                    term::disable_raw_mode()?;
                    terminal.set_cursor(0, 0)?;
                    terminal.show_cursor()?;

                    let status = call(&app, cmd, false)
                        .map(|s| {
                            if s.success() {
                                Ok(())
                            } else {
                                Err(format!("process exited with code {}", &s))
                            }
                        })
                        .unwrap_or_else(|e| Err(e.to_string()));

                    if let Err(e) = status {
                        let msg = app::MsgIn::External(app::ExternalMsg::LogError(e));
                        tx_msg_in.send(app::Task::new(msg, None))?;
                    };

                    terminal.hide_cursor()?;
                    terminal.clear()?;
                    term::enable_raw_mode()?;
                    tx_event_reader.send(false)?;
                }
            };
        }

        if app.focused_node() != last_app.focused_node() {
            fs::write(&app.pipe().focus_out, app.focused_node_str())?;
        };

        if app.selection() != last_app.selection() {
            fs::write(&app.pipe().selection_out, app.selection_str())?;
        };

        if app.mode_str() != last_app.mode_str() {
            fs::write(&app.pipe().mode_out, app.mode_str())?;
        };

        if app.directory_buffer() != last_app.directory_buffer() {
            fs::write(&app.pipe().directory_nodes_out, app.directory_nodes_str())?;
        };

        if app.logs() != last_app.logs() {
            fs::write(&app.pipe().logs_out, app.logs_str())?;
        };

        if app.result() != last_app.result() {
            fs::write(&app.pipe().result_out, app.result_str())?;
        };
    }

    terminal.clear()?;
    terminal.set_cursor(0, 0)?;
    term::disable_raw_mode()?;
    execute!(terminal.backend_mut(), term::LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    fs::remove_dir_all(app.session_path())?;

    if let Some(out) = output {
        print!("{}", out);
    }

    result
}
