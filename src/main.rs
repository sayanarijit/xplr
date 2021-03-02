use crossterm::terminal as term;
use handlebars::{handlebars_helper, Handlebars};
use shellwords;
use std::io;
use termion::get_tty;
use termion::{input::TermRead, screen::AlternateScreen};
use tui::backend::CrosstermBackend;
use tui::widgets::{ListState, TableState};
use tui::Terminal;
use xplr::app;
use xplr::error::Error;
use xplr::input::Key;
use xplr::ui;

handlebars_helper!(hex: |v: i64| format!("0x{:x}", v));
handlebars_helper!(shell_escape: |v: str| format!("{}", shellwords::escape(v)));

fn main() -> Result<(), Error> {
    let mut app = app::create()?;

    let mut hb = Handlebars::new();
    hb.register_helper("shell_escape", Box::new(shell_escape));
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
        if let Some(actions) = app.actions_from_key(key) {
            for action in actions.iter() {
                app = match app.handle(action) {
                    Ok(mut a) => {
                        terminal
                            .draw(|f| ui::draw(&a, &hb, f, &mut table_state, &mut list_state))?;

                        if let Some(result) = a.result.clone() {
                            term::disable_raw_mode().unwrap();
                            std::mem::drop(terminal);
                            if !result.is_empty() {
                                println!("{}", &result);
                            };
                            break 'outer;
                        };

                        if let Some(cmd) = a.call.clone() {
                            term::disable_raw_mode().unwrap();
                            std::mem::drop(terminal);
                            if let Some((_, meta)) = a.directory_buffer.focused_item() {
                                let _ = std::process::Command::new(cmd.command.clone())
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
                            terminal.draw(|f| {
                                ui::draw(&a, &hb, f, &mut table_state, &mut list_state)
                            })?;
                        };

                        a.call = None;
                        a.result = None;
                        a
                    }
                    Err(e) => {
                        result = Err(e);
                        break 'outer;
                    }
                }
            }
        };
    }

    result
}
