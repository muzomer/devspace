use crossterm::event::KeyEvent;
use ratatui::{
    layout::{Constraint, Layout},
    Frame,
};

use crate::{
    components::{EventState, RepositoriesComponent, WorktreesComponent},
    git::{Repository, Worktree},
};

#[derive(Debug, Clone, Copy)]
pub enum Focus {
    Worktrees,
    Repositories,
    CreateWorktree,
}

pub struct App {
    pub worktrees: WorktreesComponent,
    pub repos: RepositoriesComponent,
    pub focus: Focus,
}

impl App {
    pub fn new(worktrees: Vec<Worktree>, repositories: Vec<Repository>) -> Self {
        Self {
            worktrees: WorktreesComponent::new(worktrees),
            repos: RepositoriesComponent::new(repositories),
            focus: Focus::Worktrees,
        }
    }

    pub fn draw(&mut self, frame: &mut Frame) {
        let full_area = Layout::default()
            .constraints([Constraint::Percentage(100)])
            .split(frame.area())[0];

        self.worktrees.draw(frame, full_area);
    }

    pub fn handle_key(&mut self, key: KeyEvent) -> EventState {
        match self.focus {
            Focus::Worktrees => self.worktrees.handle_key(key),
            Focus::Repositories => todo!(),
            Focus::CreateWorktree => todo!(),
        }
    }

    // pub fn go_to_worktree(&mut self) {
    //     if let Some(selected_index) = self.worktrees.state.selected() {
    //         let selected_space = &self.worktrees.items[selected_index];
    //         self.selected_space = selected_space.clone();
    //     }
    // }

    // pub fn print_worktree_dir(&self) {
    //     println!("{}", self.selected_space);
    // }
}
