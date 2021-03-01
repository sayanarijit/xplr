use handlebars::Handlebars;
use std::io;
use termion::{input::MouseTerminal, input::TermRead, raw::IntoRawMode, screen::AlternateScreen};
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

    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let stdin = io::stdin();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let keys = stdin
        .keys()
        .map(|e| e.map_or(Key::NotSupported, |e| Key::from_termion_event(e)));

    let mut table_state = TableState::default();
    let mut list_state = ListState::default();

    terminal.draw(|f| ui::draw(&app, &hb, f, &mut table_state, &mut list_state))?;

    'outer: for key in keys {
        if let Some(actions) = app.actions_from_key(key) {
            for action in actions.iter() {
                app = app.handle(action)?;
                terminal.draw(|f| ui::draw(&app, &hb, f, &mut table_state, &mut list_state))?;
                if app.result.is_some() {
                    break 'outer;
                }
            }
        };
    }

    std::mem::drop(terminal);
    println!("{}", app.result.unwrap_or("".into()));
    Ok(())
}
