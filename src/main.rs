mod app;
mod cli;
mod devspace;
mod ui;

use std::{error::Error, io};

use app::{App, CurrentScreen};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    crossterm::{
        event::{self, Event, KeyCode, KeyEventKind},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    Terminal,
};
use ui::ui;

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
            if key_event.kind == KeyEventKind::Release {
                continue;
            }

            match app.current_screen {
                CurrentScreen::ListDevspaces => match key_event.code {
                    KeyCode::Char('q') | KeyCode::Esc => break,
                    KeyCode::Char('j') | KeyCode::Down => app.devspaces.select_next(),
                    KeyCode::Char('k') | KeyCode::Up => app.devspaces.select_previous(),
                    KeyCode::Char('g') | KeyCode::Home => app.devspaces.select_first(),
                    KeyCode::Char('G') | KeyCode::End => app.devspaces.select_last(),
                    KeyCode::Char('n') => app.current_screen = CurrentScreen::ListRepos,
                    KeyCode::Enter => {
                        app.go_to_devspace();
                        break;
                    }
                    _ => {}
                },
                CurrentScreen::ListRepos => match key_event.code {
                    KeyCode::Char('q') | KeyCode::Esc => {
                        app.current_screen = CurrentScreen::ListDevspaces
                    }
                    KeyCode::Char('j') | KeyCode::Down => app.repos.select_next(),
                    KeyCode::Char('k') | KeyCode::Up => app.repos.select_previous(),
                    KeyCode::Char('g') | KeyCode::Home => app.repos.select_first(),
                    KeyCode::Char('G') | KeyCode::End => app.repos.select_last(),
                    _ => {}
                },
            }
        }
    }
    Ok(true)
}
