use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    layout::{Constraint, Flex, Layout, Rect},
    Frame,
};

use crate::{
    cli::Args,
    components::{CreateWorktreeComponent, EventState, RepositoriesComponent, WorktreesComponent},
    git,
};

#[derive(Debug, Clone, Copy)]
pub enum Focus {
    Worktrees,
    Repositories,
    CreateWorktree,
}

pub struct App {
    worktrees: WorktreesComponent,
    repositories: RepositoriesComponent,
    create_worktree: CreateWorktreeComponent,
    args: Args,
    focus: Focus,
}

impl App {
    pub fn new() -> App {
        let args = Args::new();
        let repositories = RepositoriesComponent::new(git::list_repositories(&args.repos_dir));
        let worktrees = WorktreesComponent::new(repositories.worktrees());
        Self {
            worktrees,
            repositories,
            create_worktree: CreateWorktreeComponent::new(),
            focus: Focus::Worktrees,
            args,
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

        if let Focus::CreateWorktree = self.focus {
            let popup_area = self.popup_area(full_area, 50, 30);
            self.create_worktree.draw(frame, popup_area);
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
                } else if (key.modifiers == KeyModifiers::CONTROL && key.code == KeyCode::Char('d'))
                    || (key.code == KeyCode::Enter)
                {
                    self.create_worktree = CreateWorktreeComponent::new();
                    self.focus = Focus::CreateWorktree;
                    EventState::Consumed
                } else {
                    self.focus = Focus::Worktrees;
                    EventState::Consumed
                }
            }
            Focus::CreateWorktree => {
                let result = self.create_worktree.handle_key(key);
                if result == EventState::Consumed {
                    result
                } else {
                    if key.code == KeyCode::Enter
                        && !self.create_worktree.new_worktree_name.is_empty()
                    {
                        if let Some(selected_repository) = self.repositories.selected_repository() {
                            if let Some(created_worktree) = selected_repository.create_new_worktree(
                                &self.create_worktree.new_worktree_name,
                                &self.args.worktrees_dir,
                            ) {
                                self.worktrees.add(created_worktree);
                            }
                        }
                    }
                    self.focus = Focus::Worktrees;
                    EventState::Consumed
                }
            }
        }
    }

    fn popup_area(&self, area: Rect, percent_x: u16, percent_y: u16) -> Rect {
        let vertical = Layout::vertical([Constraint::Percentage(percent_y)]).flex(Flex::Center);
        let horizontal = Layout::horizontal([Constraint::Percentage(percent_x)]).flex(Flex::Center);
        let [area] = vertical.areas(area);
        let [area] = horizontal.areas(area);
        area
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
