mod app;
mod cli;
mod ui;

use std::{error::Error, io};

use app::App;
use ratatui::{
    backend::{Backend, CrosstermBackend},
    crossterm::{
        event::{self, Event},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    Terminal,
};
use ui::{events, ui};

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stderr = io::stderr(); // This is a special case. Normally using stdout is fine
    execute!(stderr, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;
    let mut app = App::default();

    // start app
    let res = run_app(&mut terminal, &mut app);
    // restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen,)?;
    terminal.show_cursor()?;

    if let Ok(do_print) = res {
        if do_print {
            app.print_devspace_dir();
        }
    } else if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<bool> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key_event) = event::read()? {
            match events::handle_event(key_event, app) {
                events::HandleEventResult::Continue => continue,
                events::HandleEventResult::Stop => return Ok(true),
            }
        }
    }
}
