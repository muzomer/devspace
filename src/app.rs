use ratatui::widgets::ListState;

use crate::cli;
use crate::devspace;

#[derive(Debug)]
pub enum CurrentScreen {
    ListDevspaces,
    ListRepos,
}

#[derive(Debug)]
pub struct App {
    pub devspaces: DevspaceList,
    pub repos: RepositoriesList,
    pub exit: bool,
    pub selected_space: String,
    pub current_screen: CurrentScreen,
}

impl Default for App {
    fn default() -> Self {
        let args = cli::Args::new();
        let devspaces = devspace::list(&args.spaces_dir).unwrap_or_default();
        let repos = devspace::list(&args.repos_dirs).unwrap_or_default();

        Self {
            devspaces: DevspaceList::new(devspaces),
            repos: RepositoriesList::new(repos),
            exit: false,
            selected_space: String::new(),
            current_screen: CurrentScreen::ListDevspaces,
        }
    }
}

impl App {
    pub fn go_to_devspace(&mut self) {
        if let Some(selected_index) = self.devspaces.state.selected() {
            let selected_space = &self.devspaces.items[selected_index];
            self.selected_space = selected_space.clone();
        }
        self.exit();
    }

    pub fn exit(&mut self) {
        self.exit = true;
    }

    pub fn print_devspace_dir(&self) {
        println!("{}", self.selected_space);
    }
}

#[derive(Debug, Clone)]
pub struct DevspaceList {
    pub items: Vec<String>,
    pub state: ListState,
}

impl DevspaceList {
    pub fn new(items: Vec<String>) -> Self {
        let state = ListState::default();

        Self { items, state }
    }
    pub fn select_next(&mut self) {
        self.state.select_next();
    }
    pub fn select_previous(&mut self) {
        self.state.select_previous();
    }
    pub fn select_first(&mut self) {
        self.state.select_first();
    }

    pub fn select_last(&mut self) {
        self.state.select_last();
    }
}

#[derive(Debug, Clone)]
pub struct RepositoriesList {
    pub items: Vec<String>,
    pub state: ListState,
}

impl RepositoriesList {
    pub fn new(items: Vec<String>) -> Self {
        let state = ListState::default();

        Self { items, state }
    }
    pub fn select_next(&mut self) {
        self.state.select_next();
    }
    pub fn select_previous(&mut self) {
        self.state.select_previous();
    }
    pub fn select_first(&mut self) {
        self.state.select_first();
    }

    pub fn select_last(&mut self) {
        self.state.select_last();
    }
}
