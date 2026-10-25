use anyhow::{Result, bail};
use std::fs;
use std::fs::File;
use tui::Terminal;
use tui::backend::Backend;
use tui::backend::ClearType;
use tui::backend::CrosstermBackend;
use tui::crossterm::event;
use tui::crossterm::execute;
use tui::crossterm::terminal as term;

pub fn get_tty() -> Result<fs::File> {
    let tty = "/dev/tty";
    match fs::OpenOptions::new().read(true).write(true).open(tty) {
        Ok(f) => Ok(f),
        Err(e) => {
            bail!(format!("could not open {tty}. {e}"))
        }
    }
}

pub fn clear(terminal: &mut Terminal<CrosstermBackend<File>>) -> Result<()> {
    // TODO: workaround for https://github.com/sayanarijit/xplr/issues/790
    // Clear AND redraw the terminal surface
    terminal.backend_mut().clear_region(ClearType::All)?;
    terminal.draw(|f| f.render_widget(tui::widgets::Clear, f.area()))?;
    terminal.set_cursor_position((0, 0))?;
    Ok(())
}

pub fn start() -> Result<Terminal<CrosstermBackend<File>>> {
    term::enable_raw_mode()?;
    let stdout = get_tty()?; // Use a tty to enable piping results
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    resume(&mut terminal, true)?;
    Ok(terminal)
}

pub fn stop(
    terminal: &mut Terminal<CrosstermBackend<File>>,
    leave_alt_screen: bool,
) -> Result<()> {
    execute!(terminal.backend_mut(), event::DisableMouseCapture).unwrap_or_default();
    if leave_alt_screen {
        execute!(terminal.backend_mut(), term::LeaveAlternateScreen)?;
    } else {
        clear(terminal)?;
    }
    term::disable_raw_mode()?;
    terminal.show_cursor()?;
    Ok(())
}

pub fn resume(
    terminal: &mut Terminal<CrosstermBackend<File>>,
    enter_alt_screen: bool,
) -> Result<()> {
    term::enable_raw_mode()?;
    if enter_alt_screen {
        execute!(terminal.backend_mut(), term::EnterAlternateScreen)?;
    }
    terminal.hide_cursor()?;
    clear(terminal)?;
    Ok(())
}
