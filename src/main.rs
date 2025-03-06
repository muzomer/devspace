mod app;
mod cli;
mod components;
mod git;

use std::{error::Error, io};

use components::EventState;
use ratatui::{
    backend::{Backend, CrosstermBackend},
    crossterm::{
        event::{self, Event},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    Terminal,
};

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    let args = cli::Args::new();
    let worktrees = git::Worktree::list(&args.worktrees_dir);
    let repos = git::Repository::list(&args.repos_dirs);
    let mut app = app::App::new(worktrees, repos);
    let mut terminal = setup_terminal()?;
    let res = run_app(&mut terminal, &mut app);
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

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut app::App) -> io::Result<bool> {
    loop {
        terminal.draw(|f| app.draw(f))?;

        if let Event::Key(key) = event::read()? {
            if app.handle_key(key) == EventState::NotConsumed {
                break Ok(false);
            }
        };

        // if let Event::Key(key_event) = event::read()? {
        //     match ui::handle_event(key_event, app) {
        //         ui::HandleEventResult::Stop => return Ok(true),
        //         ui::HandleEventResult::NewScreen(screen) => app.current_screen = screen,
        //         _ => {}
        //     }
        // }
    }
}
