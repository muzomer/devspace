mod cli;
mod model;
mod ui;

use std::{error::Error, io};

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
    let args = cli::Args::new();
    let mut terminal = setup_terminal()?;
    let mut app = create_app(args);
    let res = run_app(&mut terminal, &mut app);

    let _ = restore_terminal(&mut terminal);

    if let Ok(do_print) = res {
        if do_print {
            app.print_devspace_dir();
        }
    } else if let Err(err) = res {
        println!("{err:?}");
    }

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

fn create_app(args: cli::Args) -> ui::App {
    let devspaces = model::Devspace::list(&args.spaces_dir);
    let repos = model::Repository::list(&args.repos_dirs);
    ui::App::new(devspaces, repos)
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut ui::App) -> io::Result<bool> {
    loop {
        terminal.draw(|f| ui::draw(f, app))?;

        if let Event::Key(key_event) = event::read()? {
            match ui::handle_event(key_event, app) {
                ui::HandleEventResult::Continue => continue,
                ui::HandleEventResult::Stop => return Ok(true),
            }
        }
    }
}
