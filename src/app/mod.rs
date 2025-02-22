use crate::cli;

mod devspace;
mod repo;

#[derive(Debug, Clone, Copy)]
pub enum ListingScreenMode {
    Filtering,
    Navigating,
}

#[derive(Debug, Clone, Copy)]
pub enum CurrentScreen {
    ListDevspaces(ListingScreenMode),
    ListRepos(ListingScreenMode),
}

#[derive(Debug)]
pub struct App {
    pub devspaces: devspace::DevspaceList,
    pub repos: repo::RepositoriesList,
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
            devspaces: devspace::DevspaceList::new(devspaces),
            repos: repo::RepositoriesList::new(repos),
            exit: false,
            selected_space: String::new(),
            current_screen: CurrentScreen::ListDevspaces(ListingScreenMode::Filtering),
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
