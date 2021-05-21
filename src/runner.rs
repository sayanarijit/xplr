#![allow(clippy::too_many_arguments)]

use crate::app;
use crate::auto_refresher;
use crate::event_reader;
use crate::explorer;
use crate::pipe_reader;
use crate::pwd_watcher;
use crate::ui;
use anyhow::Result;
use crossterm::event;
use crossterm::execute;
use crossterm::terminal as term;
use std::fs;
use std::io;
use std::io::prelude::*;
use std::process::{Command, ExitStatus, Stdio};
use std::sync::mpsc;
use termion::get_tty;
use tui::backend::CrosstermBackend;
use tui::Terminal;

fn call(app: &app::App, cmd: app::Command, silent: bool) -> io::Result<ExitStatus> {
    let focus_index = app
        .directory_buffer()
        .map(|d| d.focus())
        .unwrap_or_default()
        .to_string();

    let (stdin, stdout, stderr) = if silent {
        (Stdio::null(), Stdio::null(), Stdio::null())
    } else {
        (get_tty()?.into(), get_tty()?.into(), get_tty()?.into())
    };

    Command::new(cmd.command().clone())
        .env("XPLR_APP_VERSION", app.version())
        .env("XPLR_CONFIG_VERSION", app.config().version())
        .env("XPLR_PID", &app.pid().to_string())
        .env("XPLR_INPUT_BUFFER", app.input_buffer().unwrap_or_default())
        .env("XPLR_FOCUS_PATH", app.focused_node_str())
        .env("XPLR_FOCUS_INDEX", focus_index)
        .env("XPLR_SESSION_PATH", app.session_path())
        .env("XPLR_PIPE_MSG_IN", app.pipe().msg_in())
        .env("XPLR_PIPE_SELECTION_OUT", app.pipe().selection_out())
        .env("XPLR_PIPE_HISTORY_OUT", app.pipe().history_out())
        .env("XPLR_MODE", app.mode_str())
        .env("XPLR_PIPE_RESULT_OUT", app.pipe().result_out())
        .env(
            "XPLR_PIPE_GLOBAL_HELP_MENU_OUT",
            app.pipe().global_help_menu_out(),
        )
        .env(
            "XPLR_PIPE_DIRECTORY_NODES_OUT",
            app.pipe().directory_nodes_out(),
        )
        .env("XPLR_PIPE_LOGS_OUT", app.pipe().logs_out())
        .stdin(stdin)
        .stdout(stdout)
        .stderr(stderr)
        .args(cmd.args())
        .status()
}

pub fn run(
    mut app: app::App,
    focused_path: Option<String>,
    lua: mlua::Lua,
) -> Result<Option<String>> {
    let (tx_msg_in, rx_msg_in) = mpsc::channel();
    let (tx_event_reader, rx_event_reader) = mpsc::channel();
    let (tx_pwd_watcher, rx_pwd_watcher) = mpsc::channel();

    app = app.explore_pwd()?;

    app = if let Some(f) = focused_path.clone() {
        app.focus_by_file_name(&f, true)?
    } else {
        app.focus_first(true)?
    };

    explorer::explore_recursive_async(
        app.explorer_config().clone(),
        app.pwd().clone(),
        focused_path,
        tx_msg_in.clone(),
    );
    tx_pwd_watcher.send(app.pwd().clone())?;

    let mut result = Ok(None);
    let session_path = app.session_path().to_owned();

    term::enable_raw_mode()?;
    let mut stdout = get_tty()?;
    // let mut stdout = stdout.lock();
    execute!(stdout, term::EnterAlternateScreen)?;
    execute!(stdout, event::EnableMouseCapture).unwrap_or_default(); // Optional
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    // Threads
    auto_refresher::start_auto_refreshing(tx_msg_in.clone());
    pipe_reader::keep_reading(app.pipe().msg_in().clone(), tx_msg_in.clone());
    event_reader::keep_reading(tx_msg_in.clone(), rx_event_reader);
    pwd_watcher::keep_watching(app.pwd(), tx_msg_in.clone(), rx_pwd_watcher)?;

    'outer: for task in rx_msg_in {
        match app.handle_task(task) {
            Ok(a) => {
                app = a;
                while let Some(msg) = app.pop_msg_out() {
                    match msg {
                        // NOTE: Do not schedule critical tasks via tx_msg_in in this loop.
                        // Try handling them immediately.
                        app::MsgOut::Enque(task) => {
                            tx_msg_in.send(task)?;
                        }

                        app::MsgOut::Quit => {
                            result = Ok(None);
                            break 'outer;
                        }

                        app::MsgOut::PrintResultAndQuit => {
                            result = Ok(Some(app.result_str()));
                            break 'outer;
                        }

                        app::MsgOut::PrintAppStateAndQuit => {
                            let out = serde_yaml::to_string(&app)?;
                            result = Ok(Some(out));
                            break 'outer;
                        }

                        app::MsgOut::Debug(path) => {
                            fs::write(&path, serde_yaml::to_string(&app)?)?;
                        }

                        app::MsgOut::ClearScreen => {
                            terminal.clear()?;
                        }

                        app::MsgOut::ExplorePwdAsync => {
                            explorer::explore_async(
                                app.explorer_config().clone(),
                                app.pwd().clone(),
                                app.focused_node().map(|n| n.relative_path().clone()),
                                tx_msg_in.clone(),
                            );
                            tx_pwd_watcher.send(app.pwd().clone())?;
                        }

                        app::MsgOut::ExploreParentsAsync => {
                            explorer::explore_recursive_async(
                                app.explorer_config().clone(),
                                app.pwd().clone(),
                                app.focused_node().map(|n| n.relative_path().clone()),
                                tx_msg_in.clone(),
                            );
                            tx_pwd_watcher.send(app.pwd().clone())?;
                        }

                        app::MsgOut::Refresh => {
                            // $PWD watcher
                            tx_pwd_watcher.send(app.pwd().clone())?;
                            // UI
                            terminal.draw(|f| ui::draw(f, &app, &lua))?;
                        }

                        app::MsgOut::CallSilently(cmd) => {
                            tx_event_reader.send(true)?;

                            app.write_pipes()?;
                            let status = call(&app, cmd, true)
                                .map(|s| {
                                    if s.success() {
                                        Ok(())
                                    } else {
                                        Err(format!("process exited with code {}", &s))
                                    }
                                })
                                .unwrap_or_else(|e| Err(e.to_string()));

                            if let Err(e) = status {
                                app = app.log_error(e.to_string())?;
                            };

                            tx_event_reader.send(false)?;
                        }

                        app::MsgOut::Call(cmd) => {
                            execute!(terminal.backend_mut(), event::DisableMouseCapture)
                                .unwrap_or_default(); // Optional

                            tx_event_reader.send(true)?;

                            terminal.clear()?;
                            terminal.set_cursor(0, 0)?;
                            term::disable_raw_mode()?;
                            terminal.show_cursor()?;

                            app.write_pipes()?;
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
                                app = app.log_error(e.to_string())?;
                            };

                            terminal.clear()?;
                            term::enable_raw_mode()?;
                            terminal.hide_cursor()?;
                            tx_event_reader.send(false)?;

                            execute!(terminal.backend_mut(), event::EnableMouseCapture)
                                .unwrap_or_default(); // Optional
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

    terminal.clear()?;
    terminal.set_cursor(0, 0)?;
    execute!(terminal.backend_mut(), term::LeaveAlternateScreen)?;
    execute!(terminal.backend_mut(), event::DisableMouseCapture).unwrap_or_default(); // Optional
    term::disable_raw_mode()?;
    terminal.show_cursor()?;

    fs::remove_dir_all(session_path)?;

    result
}
