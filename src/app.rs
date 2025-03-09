use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use log::debug;
use ratatui::{
    layout::{Constraint, Flex, Layout, Rect},
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
    pub repositories: RepositoriesComponent,
    pub focus: Focus,
}

impl App {
    pub fn new(worktrees: Vec<Worktree>, repositories: Vec<Repository>) -> Self {
        Self {
            worktrees: WorktreesComponent::new(worktrees),
            repositories: RepositoriesComponent::new(repositories),
            focus: Focus::Worktrees,
        }
    }

    pub fn draw(&mut self, frame: &mut Frame) {
        let [full_area] = Layout::default()
            .constraints([Constraint::Percentage(100)])
            .areas(frame.area());

        self.worktrees.draw(frame, full_area);

        if let Focus::Repositories = self.focus {
            let popup_area = self.popup_area(full_area, 50, 50);
            self.repositories.draw(frame, popup_area);
        }
    }

    pub fn handle_key(&mut self, key: KeyEvent) -> EventState {
        match self.focus {
            Focus::Worktrees => {
                let result = self.worktrees.handle_key(key);
                if result == EventState::Consumed {
                    result
                } else if key.modifiers == KeyModifiers::CONTROL && key.code == KeyCode::Char('d') {
                    self.focus = Focus::Repositories;
                    EventState::Consumed
                } else {
                    EventState::NotConsumed
                }
            }
            Focus::Repositories => {
                let result = self.repositories.handle_key(key);
                if result == EventState::Consumed {
                    result
                } else if key.modifiers == KeyModifiers::CONTROL && key.code == KeyCode::Char('d') {
                    // self.focus = Focus::Repositories;
                    EventState::Consumed
                } else {
                    EventState::NotConsumed
                }
            }
            Focus::CreateWorktree => todo!(),
        }
    }

    fn popup_area(&self, area: Rect, percent_x: u16, percent_y: u16) -> Rect {
        let vertical = Layout::vertical([Constraint::Percentage(percent_y)]).flex(Flex::Center);
        let horizontal = Layout::horizontal([Constraint::Percentage(percent_x)]).flex(Flex::Center);
        let [area] = vertical.areas(area);
        let [area] = horizontal.areas(area);
        area
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
