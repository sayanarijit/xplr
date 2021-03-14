use crossterm::terminal as term;
use handlebars::{handlebars_helper, Handlebars};
use shellwords;
use std::fs;
use std::io;
use termion::get_tty;
use termion::{input::TermRead, screen::AlternateScreen};
use tui::backend::CrosstermBackend;
use tui::widgets::{ListState, TableState};
use tui::Terminal;
use xplr::app;
use xplr::app::Task;
use xplr::error::Error;
use xplr::input::Key;
use xplr::ui;

handlebars_helper!(shellescape: |v: str| format!("{}", shellwords::escape(v)));
handlebars_helper!(readfile: |v: str| fs::read_to_string(v).unwrap_or_default());

fn main() -> Result<(), Error> {
    let mut app = app::create()?;

    let mut hb = Handlebars::new();
    hb.register_helper("shellescape", Box::new(shellescape));
    hb.register_helper("readfile", Box::new(readfile));
    hb.register_template_string(
        app::TEMPLATE_TABLE_ROW,
        &app.config
            .clone()
            .general
            .table
            .row
            .cols
            .iter()
            .map(|c| c.format.to_string())
            .collect::<Vec<String>>()
            .join("\t"),
    )?;

    let stdin = io::stdin();
    let stdout = get_tty()?;
    // let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let keys = stdin
        .keys()
        .map(|e| e.map_or(Key::NotSupported, |e| Key::from_termion_event(e)));

    let mut table_state = TableState::default();
    let mut list_state = ListState::default();

    term::enable_raw_mode().unwrap();
    terminal.draw(|f| ui::draw(&app, &hb, f, &mut table_state, &mut list_state))?;

    let mut result = Ok(());
    'outer: for key in keys {
        if let Some(actions) = app.actions_from_key(&key) {
            for action in actions.iter() {
                app = match app.handle(action) {
                    Ok(mut a) => {
                        terminal
                            .draw(|f| ui::draw(&a, &hb, f, &mut table_state, &mut list_state))?;

                        match a.task.clone() {
                            Task::NoOp => {}

                            Task::Quit => {
                                term::disable_raw_mode().unwrap();
                                std::mem::drop(terminal);
                                break 'outer;
                            }

                            Task::PrintAndQuit(txt) => {
                                term::disable_raw_mode().unwrap();
                                std::mem::drop(terminal);
                                if !txt.is_empty() {
                                    println!("{}", &txt);
                                };
                                break 'outer;
                            }

                            Task::Call(cmd) => {
                                term::disable_raw_mode().unwrap();
                                std::mem::drop(terminal);
                                if let Some((_, meta)) = a.directory_buffer.focused() {
                                    let _ = std::process::Command::new(cmd.command.clone())
                                        .current_dir(&a.directory_buffer.pwd)
                                        .args(
                                            cmd.args
                                                .iter()
                                                .map(|arg| hb.render_template(arg, &meta).unwrap()),
                                        )
                                        .status();
                                };

                                term::enable_raw_mode().unwrap();
                                let stdout = get_tty()?;
                                let stdout = AlternateScreen::from(stdout);
                                let backend = CrosstermBackend::new(stdout);
                                terminal = Terminal::new(backend)?;
                                a = a.refresh()?;
                                terminal.draw(|f| {
                                    ui::draw(&a, &hb, f, &mut table_state, &mut list_state)
                                })?;
                            }
                        };

                        a.task = Task::NoOp;
                        a
                    }
                    Err(e) => {
                        term::disable_raw_mode().unwrap();
                        std::mem::drop(terminal);
                        result = Err(e);
                        break 'outer;
                    }
                }
            }
        };
    }

    result
}
