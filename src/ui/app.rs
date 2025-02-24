use crate::model::{Devspace, Repository};

use super::devspaces::DevspaceList;
use super::repositories::RepositoriesList;
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
    pub devspaces: DevspaceList,
    pub repos: RepositoriesList,
    pub exit: bool,
    pub selected_space: String,
    pub current_screen: CurrentScreen,
}

impl App {
    pub fn new(devspaces: Vec<Devspace>, repositories: Vec<Repository>) -> Self {
        Self {
            devspaces: DevspaceList::new(devspaces),
            repos: RepositoriesList::new(repositories),
            exit: false,
            selected_space: String::new(),
            current_screen: CurrentScreen::ListDevspaces(ListingScreenMode::Filtering),
        }
    }
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
