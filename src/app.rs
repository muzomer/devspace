use crossterm::event::KeyEvent;
use ratatui::{
    layout::{Constraint, Flex, Layout, Rect},
    Frame,
};

use crate::{
    cli,
    components::{
        Action, ConfirmComponent, CreateWorktreeComponent, EventState, HelpComponent,
        RepositoriesComponent, WorktreesComponent,
    },
    git,
    keymap::{self, InputMode},
};

#[derive(Debug, Clone, Copy)]
pub enum Focus {
    Worktrees,
    Repositories,
    CreateWorktree,
    Confirm,
    Help,
}

pub struct App {
    worktrees_component: WorktreesComponent,
    repositories_component: RepositoriesComponent,
    create_worktree: CreateWorktreeComponent,
    confirm_component: ConfirmComponent,
    help_component: HelpComponent,
    args: cli::Args,
    focus: Focus,
    previous_focus: Focus,
    mode: InputMode,
}

impl App {
    pub fn new() -> App {
        let args = cli::Args::new();
        let repositories = git::list_repositories(&args.repos_dir, args.run_fetch);
        let worktrees = git::worktrees_of_repositories(&repositories);

        let repositories_component = RepositoriesComponent::new(repositories);
        let worktrees_component = WorktreesComponent::new(worktrees, args.worktrees_dir.clone());
        Self {
            worktrees_component,
            repositories_component,
            create_worktree: CreateWorktreeComponent::new(),
            confirm_component: ConfirmComponent::new(String::new()),
            help_component: HelpComponent::new(vec![]),
            focus: Focus::Worktrees,
            previous_focus: Focus::Worktrees,
            args,
            mode: InputMode::Normal,
        }
    }

    pub fn draw(&mut self, frame: &mut Frame) {
        let [full_area] = Layout::default()
            .constraints([Constraint::Percentage(100)])
            .areas(frame.area());

        self.worktrees_component.draw(
            frame,
            full_area,
            self.mode,
            matches!(self.focus, Focus::Worktrees),
        );

        if let Focus::Repositories = self.focus {
            let popup_area = self.popup_area(full_area, 50, 50);
            self.repositories_component.draw(frame, popup_area);
        }

        if let Focus::CreateWorktree = self.focus {
            let popup_area = self.popup_area(full_area, 50, 30);
            self.create_worktree.draw(frame, popup_area);
        }

        if let Focus::Confirm = self.focus {
            let popup_area = self.popup_area(full_area, 60, 30);
            self.confirm_component.draw(frame, popup_area);
        }

        if let Focus::Help = self.focus {
            let (w, h) = self.help_component.dimensions();
            let popup_area = self.popup_area_fixed(full_area, w, h);
            self.help_component.draw(frame, popup_area);
        }
    }

    pub fn handle_key(&mut self, key: KeyEvent) -> EventState {
        let action = match keymap::resolve(self.mode, key) {
            Some(action) => action,
            None => return EventState::NotConsumed,
        };

        match self.focus {
            Focus::Worktrees => self.handle_worktrees_action(action),
            Focus::Repositories => self.handle_repositories_action(action),
            Focus::CreateWorktree => self.handle_create_worktree_action(action),
            Focus::Confirm => self.handle_confirm_action(action),
            Focus::Help => self.handle_help_action(action),
        }
    }

    fn handle_worktrees_action(&mut self, action: Action) -> EventState {
        match action {
            Action::Quit => EventState::Exit,
            Action::ShowHelp => {
                self.previous_focus = self.focus;
                self.help_component =
                    HelpComponent::new(Self::help_bindings_for(self.focus, self.mode));
                self.focus = Focus::Help;
                EventState::Consumed
            }
            Action::OpenRepositories => {
                self.focus = Focus::Repositories;
                EventState::Consumed
            }
            Action::Delete | Action::ForceDelete => {
                match self.worktrees_component.delete_selected_worktree() {
                    Ok(()) => self.worktrees_component.last_error = None,
                    Err(e) => self.worktrees_component.last_error = Some(format!("{:#}", e)),
                }
                EventState::Consumed
            }
            Action::DeleteWithConfirmation => {
                if let Some(path) = self.worktrees_component.selected_worktree_path() {
                    self.confirm_component = ConfirmComponent::new(path);
                    self.focus = Focus::Confirm;
                }
                EventState::Consumed
            }
            Action::EnterInsertMode => {
                self.mode = InputMode::Insert;
                self.worktrees_component.focus_filter();
                EventState::Consumed
            }
            Action::ExitInsertMode => {
                self.mode = InputMode::Normal;
                self.worktrees_component.focus_list();
                EventState::Consumed
            }
            Action::FocusNext => {
                self.worktrees_component.toggle_focus();
                self.mode = if self.worktrees_component.is_filter_focused() {
                    InputMode::Insert
                } else {
                    InputMode::Normal
                };
                EventState::Consumed
            }
            _ => self.worktrees_component.handle_action(action),
        }
    }

    fn handle_repositories_action(&mut self, action: Action) -> EventState {
        match action {
            Action::Quit => EventState::Exit,
            Action::ShowHelp => {
                self.previous_focus = self.focus;
                self.help_component =
                    HelpComponent::new(Self::help_bindings_for(self.focus, self.mode));
                self.focus = Focus::Help;
                EventState::Consumed
            }
            Action::Select => {
                self.create_worktree = CreateWorktreeComponent::new();
                self.focus = Focus::CreateWorktree;
                self.mode = InputMode::Insert;
                EventState::Consumed
            }
            Action::ClosePopup => {
                self.focus = Focus::Worktrees;
                self.mode = InputMode::Normal;
                EventState::Consumed
            }
            Action::ExitInsertMode => {
                self.mode = InputMode::Normal;
                self.repositories_component.focus_list();
                EventState::Consumed
            }
            Action::FocusNext => {
                self.repositories_component.toggle_focus();
                self.mode = if self.repositories_component.is_filter_focused() {
                    InputMode::Insert
                } else {
                    InputMode::Normal
                };
                EventState::Consumed
            }
            Action::EnterInsertMode => {
                self.mode = InputMode::Insert;
                self.repositories_component.focus_filter();
                EventState::Consumed
            }
            _ => self.repositories_component.handle_action(action),
        }
    }

    fn handle_create_worktree_action(&mut self, action: Action) -> EventState {
        match action {
            Action::Quit => EventState::Exit,
            Action::Select => {
                if !self.create_worktree.new_worktree_name.is_empty() {
                    if let Some(selected_repository) =
                        self.repositories_component.selected_repository()
                    {
                        match selected_repository.create_new_worktree(
                            &self.create_worktree.new_worktree_name,
                            &self.args.worktrees_dir,
                        ) {
                            Ok(created_worktree) => {
                                self.worktrees_component.last_error = None;
                                self.worktrees_component.add(created_worktree);
                            }
                            Err(e) => {
                                self.worktrees_component.last_error = Some(format!("{:#}", e));
                            }
                        }
                    }
                }
                self.focus = Focus::Worktrees;
                self.mode = InputMode::Normal;
                EventState::Consumed
            }
            Action::ClosePopup | Action::ExitInsertMode => {
                self.focus = Focus::Worktrees;
                self.mode = InputMode::Normal;
                EventState::Consumed
            }
            _ => self.create_worktree.handle_action(action),
        }
    }

    fn handle_confirm_action(&mut self, action: Action) -> EventState {
        match action {
            Action::Quit => EventState::Exit,
            Action::Select => {
                match self.worktrees_component.delete_selected_worktree() {
                    Ok(()) => self.worktrees_component.last_error = None,
                    Err(e) => self.worktrees_component.last_error = Some(format!("{:#}", e)),
                }
                self.focus = Focus::Worktrees;
                EventState::Consumed
            }
            Action::ClosePopup | Action::ExitInsertMode => {
                self.focus = Focus::Worktrees;
                EventState::Consumed
            }
            _ => self.confirm_component.handle_action(action),
        }
    }

    fn help_bindings_for(focus: Focus, mode: InputMode) -> Vec<(&'static str, &'static str)> {
        match (focus, mode) {
            (Focus::Worktrees, InputMode::Normal) => vec![
                ("j / ↓", "Move down"),
                ("k / ↑", "Move up"),
                ("g / Home", "Go to first"),
                ("G / End", "Go to last"),
                ("i / /", "Enter filter mode"),
                ("Tab", "Toggle filter / list"),
                ("n", "New worktree (pick repo)"),
                ("d", "Delete with confirmation"),
                ("D", "Force delete"),
                ("Enter", "Copy path to clipboard & exit"),
                ("?", "Show this help"),
                ("q / Ctrl+C", "Quit"),
            ],
            (Focus::Worktrees, InputMode::Insert) => vec![
                ("Esc", "Exit filter mode"),
                ("Tab", "Toggle filter / list"),
                ("↑ / Ctrl+K", "Move up in list"),
                ("↓ / Ctrl+J", "Move down in list"),
                ("Backspace", "Delete filter character"),
                ("Enter", "Copy path to clipboard & exit"),
                ("Ctrl+C", "Quit"),
            ],
            (Focus::Repositories, InputMode::Normal) => vec![
                ("j / ↓", "Move down"),
                ("k / ↑", "Move up"),
                ("g / Home", "Go to first"),
                ("G / End", "Go to last"),
                ("i", "Enter filter mode"),
                ("Tab", "Toggle filter / list"),
                ("Enter", "Select repository"),
                ("?", "Show this help"),
                ("Esc", "Close popup"),
                ("q / Ctrl+C", "Quit"),
            ],
            (Focus::Repositories, InputMode::Insert) => vec![
                ("Esc", "Exit filter mode"),
                ("Tab", "Toggle filter / list"),
                ("↑ / Ctrl+K", "Move up in list"),
                ("↓ / Ctrl+J", "Move down in list"),
                ("Backspace", "Delete filter character"),
                ("Enter", "Select repository"),
                ("Ctrl+C", "Quit"),
            ],
            (Focus::CreateWorktree, _) => vec![
                ("Enter", "Create worktree"),
                ("Esc", "Cancel"),
                ("Backspace", "Delete character"),
                ("Ctrl+C", "Quit"),
            ],
            (Focus::Confirm, _) => vec![
                ("Enter", "Confirm delete"),
                ("Esc", "Cancel"),
                ("q / Ctrl+C", "Quit"),
            ],
            _ => vec![],
        }
    }

    fn handle_help_action(&mut self, action: Action) -> EventState {
        match action {
            Action::Quit => EventState::Exit,
            Action::ClosePopup | Action::ExitInsertMode | Action::ShowHelp => {
                self.focus = self.previous_focus;
                EventState::Consumed
            }
            _ => self.help_component.handle_action(action),
        }
    }

    fn popup_area(&self, area: Rect, percent_x: u16, percent_y: u16) -> Rect {
        let vertical = Layout::vertical([Constraint::Percentage(percent_y)]).flex(Flex::Center);
        let horizontal = Layout::horizontal([Constraint::Percentage(percent_x)]).flex(Flex::Center);
        let [area] = vertical.areas(area);
        let [area] = horizontal.areas(area);
        area
    }

    fn popup_area_fixed(&self, area: Rect, width: u16, height: u16) -> Rect {
        let vertical = Layout::vertical([Constraint::Length(height)]).flex(Flex::Center);
        let horizontal = Layout::horizontal([Constraint::Length(width)]).flex(Flex::Center);
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
