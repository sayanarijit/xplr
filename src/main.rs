#![allow(clippy::too_many_arguments)]

use anyhow::Result;
use crossterm::execute;
use crossterm::terminal as term;
use handlebars::Handlebars;
use std::env;
use std::fs;
use std::io::prelude::*;
use std::path::PathBuf;
use std::sync::mpsc;
use termion::get_tty;
use tui::backend::CrosstermBackend;
use tui::Terminal;
use xplr::app;
use xplr::auto_refresher;
use xplr::event_reader;
use xplr::explorer;
use xplr::pipe_reader;
use xplr::ui;

fn main() -> Result<()> {
    let (tx_msg_in, rx_msg_in) = mpsc::channel();
    let (tx_event_reader, rx_event_reader) = mpsc::channel();

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

    auto_refresher::start_auto_refreshing(tx_msg_in.clone());
    pipe_reader::keep_reading(app.pipe().msg_in.clone(), tx_msg_in.clone());

    event_reader::keep_reading(tx_msg_in.clone(), rx_event_reader);

    let mut last_pwd = app.pwd().clone();
    'outer: while result.is_ok() {
        while let Some(msg) = app.pop_msg_out() {
            match msg {
                app::MsgOut::Debug(path) => {
                    fs::write(&path, serde_yaml::to_string(&app)?)?;
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
                    if app.pwd() != &last_pwd {
                        explorer::explore(
                            app.explorer_config().clone(),
                            app.pwd().clone(),
                            app.focused_node().map(|n| n.relative_path.clone()),
                            tx_msg_in.clone(),
                        );
                        last_pwd = app.pwd().to_owned();
                    };

                    // UI
                    terminal.draw(|f| ui::draw(f, &app, &hb))?;

                    // Pipes
                    let focused = app
                        .focused_node()
                        .map(|n| n.absolute_path.clone())
                        .unwrap_or_default();

                    fs::write(&app.pipe().focus_out, focused)?;
                    fs::write(&app.pipe().selection_out, app.selection_str())?;
                    fs::write(&app.pipe().mode_out, &app.mode().name)?;
                    fs::write(&app.pipe().directory_nodes_out, app.directory_nodes_str())?;
                    fs::write(&app.pipe().global_help_menu_out, app.global_help_menu_str())?;
                    fs::write(&app.pipe().logs_out, app.logs_str())?;
                    fs::write(&app.pipe().result_out, app.result_str())?;
                }

                app::MsgOut::Call(cmd) => {
                    tx_event_reader.send(true)?;
                    terminal.clear()?;
                    term::disable_raw_mode()?;
                    execute!(terminal.backend_mut(), term::LeaveAlternateScreen)?;
                    terminal.show_cursor()?;

                    let pid = std::process::id().to_string();
                    let input_buffer = app.input_buffer().unwrap_or_default();

                    let focus_path = app
                        .focused_node()
                        .map(|n| n.absolute_path.clone())
                        .unwrap_or_default();

                    let focus_index = app
                        .directory_buffer()
                        .map(|d| d.focus)
                        .unwrap_or_default()
                        .to_string();

                    let pipe_msg_in = app.pipe().msg_in.clone();
                    let pipe_focus_out = app.pipe().focus_out.clone();
                    let pipe_selection_out = app.pipe().selection_out.clone();
                    let pipe_result_out = app.pipe().result_out.clone();
                    let pipe_directory_nodes_out = app.pipe().directory_nodes_out.clone();
                    let pipe_global_help_menu_out = app.pipe().global_help_menu_out.clone();
                    let pipe_logs_out = app.pipe().logs_out.clone();
                    let session_path = app.session_path();

                    let status = std::process::Command::new(cmd.command.clone())
                        .current_dir(app.pwd())
                        .env("XPLR_PID", pid)
                        .env("XPLR_INPUT_BUFFER", input_buffer)
                        .env("XPLR_FOCUS_PATH", focus_path)
                        .env("XPLR_FOCUS_INDEX", focus_index)
                        .env("XPLR_SESSION_PATH", session_path)
                        .env("XPLR_PIPE_MSG_IN", pipe_msg_in)
                        .env("XPLR_PIPE_SELECTION_OUT", pipe_selection_out)
                        .env("XPLR_PIPE_FOCUS_OUT", pipe_focus_out)
                        .env("XPLR_PIPE_RESULT_OUT", pipe_result_out)
                        .env("XPLR_PIPE_GLOBAL_HELP_MENU_OUT", pipe_global_help_menu_out)
                        .env("XPLR_PIPE_DIRECTORY_NODES_OUT", pipe_directory_nodes_out)
                        .env("XPLR_PIPE_LOGS_OUT", pipe_logs_out)
                        .args(cmd.args.clone())
                        .status()
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
                        tx_msg_in.send(app::Task::new(1, msg, None))?;
                    };

                    terminal.hide_cursor()?;
                    execute!(terminal.backend_mut(), term::EnterAlternateScreen)?;
                    term::enable_raw_mode()?;
                    tx_event_reader.send(false)?;
                    terminal.draw(|f| ui::draw(f, &app, &hb))?;
                }
            };
        }

        for task in rx_msg_in.try_iter() {
            app = app.enqueue(task);
        }

        let (new_app, new_result) = match app.clone().possibly_mutate() {
            Ok(a) => (a, Ok(())),
            Err(e) => (app, Err(e)),
        };

        app = new_app;
        result = new_result;

        // thread::sleep(Duration::from_millis(10));
    }

    term::disable_raw_mode()?;
    execute!(terminal.backend_mut(), term::LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    fs::remove_dir_all(app.session_path())?;

    if let Some(out) = output {
        println!("{}", out);
    }

    result
}
