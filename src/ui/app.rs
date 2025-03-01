use crate::model::{Repository, Worktree};

use super::create_worktree::CreateWorktreeScreen;
use super::repositories::RepositoriesScreen;
use super::worktrees::WorktreesScreen;
#[derive(Debug, Clone, Copy)]
pub enum ListingScreenMode {
    Filtering,
    Navigating,
}

pub enum Screen {
    ListWorktrees,
    ListRepos,
    CreateWorktree(Repository),
}

pub struct App {
    pub worktrees: WorktreesScreen,
    pub repos: RepositoriesScreen,
    pub new_worktree: CreateWorktreeScreen,
    pub selected_space: String,
    pub current_screen: Screen,
}

impl App {
    pub fn new(worktrees: Vec<Worktree>, repositories: Vec<Repository>) -> Self {
        Self {
            worktrees: WorktreesScreen::new(worktrees),
            repos: RepositoriesScreen::new(repositories),
            new_worktree: CreateWorktreeScreen::new(),
            selected_space: String::new(),
            current_screen: Screen::ListWorktrees,
        }
    }
    pub fn go_to_worktree(&mut self) {
        if let Some(selected_index) = self.worktrees.state.selected() {
            let selected_space = &self.worktrees.items[selected_index];
            self.selected_space = selected_space.clone();
        }
    }

    pub fn print_worktree_dir(&self) {
        println!("{}", self.selected_space);
    }
}
