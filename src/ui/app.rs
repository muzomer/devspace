use crate::model::{Repository, Worktree};

use super::repositories::RepositoriesList;
use super::worktrees::WorktreesList;
#[derive(Debug, Clone, Copy)]
pub enum ListingScreenMode {
    Filtering,
    Navigating,
}

#[derive(Debug, Clone, Copy)]
pub enum CurrentScreen {
    ListWorktrees(ListingScreenMode),
    ListRepos(ListingScreenMode),
    CreatingWorktree,
}

#[derive(Debug)]
pub struct App {
    pub worktrees: WorktreesList,
    pub repos: RepositoriesList,
    pub exit: bool,
    pub selected_space: String,
    pub current_screen: CurrentScreen,
}

impl App {
    pub fn new(worktrees: Vec<Worktree>, repositories: Vec<Repository>) -> Self {
        Self {
            worktrees: WorktreesList::new(worktrees),
            repos: RepositoriesList::new(repositories),
            exit: false,
            selected_space: String::new(),
            current_screen: CurrentScreen::ListWorktrees(ListingScreenMode::Filtering),
        }
    }
    pub fn go_to_worktree(&mut self) {
        if let Some(selected_index) = self.worktrees.state.selected() {
            let selected_space = &self.worktrees.items[selected_index];
            self.selected_space = selected_space.clone();
        }
        self.exit();
    }

    pub fn exit(&mut self) {
        self.exit = true;
    }

    pub fn print_worktree_dir(&self) {
        println!("{}", self.selected_space);
    }
}
