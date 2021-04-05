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

                    app = app.refresh_selection()?;

                    let selection = app
                        .selection()
                        .iter()
                        .map(|n| n.absolute_path.clone())
                        .collect::<Vec<String>>()
                        .join("\n");

                    fs::write(&app.pipe().selection_out, selection)?;

                    fs::write(&app.pipe().mode_out, &app.mode().name)?;
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

                    let selection = app
                        .selection()
                        .iter()
                        .map(|n| n.absolute_path.clone())
                        .collect::<Vec<String>>()
                        .join("\n");

                    let directory_nodes = app
                        .directory_buffer()
                        .map(|d| {
                            d.nodes
                                .iter()
                                .map(|n| n.absolute_path.clone())
                                .collect::<Vec<String>>()
                                .join("\n")
                        })
                        .unwrap_or_default();

                    let logs = app
                        .logs()
                        .iter()
                        .map(|l| l.to_string())
                        .collect::<Vec<String>>()
                        .join("\n");

                    let help_menu = app
                        .config()
                        .modes
                        .iter()
                        .map(|(name, mode)| {
                            let help = mode
                                .help_menu()
                                .iter()
                                .map(|l| match l {
                                    app::HelpMenuLine::Paragraph(p) => format!("\t{}", p),
                                    app::HelpMenuLine::KeyMap(k, h) => {
                                        format!(" {:15} | {}", k, h)
                                    }
                                })
                                .collect::<Vec<String>>()
                                .join("\n");

                            format!("### {}\n\n key             | action\n --------------- | ------\n{}", name, help)
                        })
                        .collect::<Vec<String>>()
                        .join("\n\n\n");

                    let pipe_msg_in = app.pipe().msg_in.clone();
                    let pipe_focus_out = app.pipe().focus_out.clone();
                    let pipe_selection_out = app.pipe().selection_out.clone();

                    let app_yaml = serde_yaml::to_string(&app)?;
                    let session_path = app.session_path();
                    let result = app.result_str();

                    let status = std::process::Command::new(cmd.command.clone())
                        .current_dir(app.pwd())
                        .env("XPLR_PID", pid)
                        .env("XPLR_INPUT_BUFFER", input_buffer)
                        .env("XPLR_FOCUS_PATH", focus_path)
                        .env("XPLR_FOCUS_INDEX", focus_index)
                        .env("XPLR_SELECTION", selection)
                        .env("XPLR_SESSION_PATH", session_path)
                        .env("XPLR_PIPE_MSG_IN", pipe_msg_in)
                        .env("XPLR_PIPE_SELECTION_OUT", pipe_selection_out)
                        .env("XPLR_PIPE_FOCUS_OUT", pipe_focus_out)
                        .env("XPLR_APP_YAML", app_yaml)
                        .env("XPLR_RESULT", result)
                        .env("XPLR_GLOBAL_HELP_MENU", help_menu)
                        .env("XPLR_DIRECTORY_NODES", directory_nodes)
                        .env("XPLR_LOGS", logs)
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
