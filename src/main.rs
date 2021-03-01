use crossterm::terminal as term;
use handlebars::Handlebars;
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

fn main() -> Result<(), Error> {
    let mut app = app::create()?;

    let mut hb = Handlebars::new();
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

    let stdout = get_tty()?;
    // let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let stdin = io::stdin();
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
                    Ok(a) => {
                        terminal
                            .draw(|f| ui::draw(&a, &hb, f, &mut table_state, &mut list_state))?;
                        if a.result.is_some() {
                            term::disable_raw_mode().unwrap();
                            std::mem::drop(terminal);
                            println!("{}", &a.result.unwrap_or("".into()));
                            break 'outer;
                        };
                        a
                    }
                    Err(e) => {
                        term::disable_raw_mode().unwrap();
                        result = Err(e);
                        break 'outer;
                    }
                }
            }
        };
    }

    result
}
