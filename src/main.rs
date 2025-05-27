use devspace::{app, logs, run_app};
use std::{error::Error, io};

use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    Terminal,
};

fn main() -> Result<(), Box<dyn Error>> {
    logs::initialize_logging()?;
    let mut app = app::App::new();
    let mut terminal = setup_terminal()?;
    let _ = run_app(&mut terminal, &mut app);
    let _ = restore_terminal(&mut terminal);
    Ok(())
}

fn setup_terminal() -> io::Result<Terminal<CrosstermBackend<io::Stderr>>> {
    enable_raw_mode()?;
    let mut stderr = io::stderr();
    execute!(stderr, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stderr);
    Terminal::new(backend)
}

fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<io::Stderr>>) -> io::Result<()> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen,)?;
    terminal.show_cursor()
}
