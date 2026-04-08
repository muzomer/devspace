use crossterm::event::KeyEvent;
use ratatui::{
    layout::{Constraint, Flex, Layout, Rect},
    Frame,
};

use crate::{
    cli,
    components::{
        Action, ConfirmComponent, CreateWorktreeComponent, EventState, HelpComponent, HelpEntry,
        PrWorktreeComponent, RepositoriesComponent, WorktreesComponent,
    },
    git, github,
    keymap::{self, InputMode},
};

#[derive(Debug, Clone, Copy)]
pub enum Focus {
    Worktrees,
    Repositories,
    CreateWorktree,
    Confirm,
    Help,
    PrWorktree,
}

pub struct App {
    worktrees_component: WorktreesComponent,
    repositories_component: RepositoriesComponent,
    create_worktree: CreateWorktreeComponent,
    confirm_component: ConfirmComponent,
    help_component: HelpComponent,
    pr_worktree_component: PrWorktreeComponent,
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
            create_worktree: CreateWorktreeComponent::new(String::new()),
            confirm_component: ConfirmComponent::new(String::new()),
            help_component: HelpComponent::new(vec![]),
            pr_worktree_component: PrWorktreeComponent::new(),
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

        let show_repos = matches!(self.focus, Focus::Repositories)
            || matches!(
                (self.focus, self.previous_focus),
                (Focus::Help, Focus::Repositories)
            );
        if show_repos {
            let popup_area = self.popup_area(full_area, 50, 50);
            self.repositories_component.draw(frame, popup_area, self.mode);
        }

        if let Focus::CreateWorktree = self.focus {
            let [popup_area] = Layout::vertical([Constraint::Length(9)]).flex(Flex::Center).areas(full_area);
            let [popup_area] = Layout::horizontal([Constraint::Percentage(55)]).flex(Flex::Center).areas(popup_area);
            self.create_worktree.draw(frame, popup_area);
        }

        if let Focus::Confirm = self.focus {
            let [popup_area] = Layout::vertical([Constraint::Length(8)]).flex(Flex::Center).areas(full_area);
            let [popup_area] = Layout::horizontal([Constraint::Percentage(55)]).flex(Flex::Center).areas(popup_area);
            self.confirm_component.draw(frame, popup_area);
        }

        if let Focus::Help = self.focus {
            let (w, h) = self.help_component.dimensions();
            let popup_area = self.popup_area_fixed(full_area, w, h);
            self.help_component.draw(frame, popup_area);
        }

        if let Focus::PrWorktree = self.focus {
            let [popup_area] = Layout::vertical([Constraint::Length(9)]).flex(Flex::Center).areas(full_area);
            let [popup_area] = Layout::horizontal([Constraint::Percentage(70)]).flex(Flex::Center).areas(popup_area);
            self.pr_worktree_component.draw(frame, popup_area);
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
            Focus::PrWorktree => self.handle_pr_worktree_action(action),
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
                self.mode = InputMode::Normal;
                EventState::Consumed
            }
            Action::OpenPrWorktree => {
                self.pr_worktree_component.reset();
                self.focus = Focus::PrWorktree;
                self.mode = InputMode::Insert;
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
                let repo_name = self
                    .repositories_component
                    .selected_repository()
                    .map(|r| r.name())
                    .unwrap_or_default();
                self.create_worktree = CreateWorktreeComponent::new(repo_name);
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
            _ => {
                let result = self.create_worktree.handle_action(action);
                if result == EventState::Consumed {
                    let name = self.create_worktree.new_worktree_name.clone();
                    self.create_worktree.base_branch_hint = if name.is_empty() {
                        None
                    } else {
                        self.repositories_component
                            .selected_repository()
                            .map(|r| r.resolve_base_branch(&name))
                    };
                }
                result
            }
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

    fn handle_pr_worktree_action(&mut self, action: Action) -> EventState {
        match action {
            Action::Quit => EventState::Exit,
            Action::ClosePopup | Action::ExitInsertMode => {
                self.pr_worktree_component.reset();
                self.focus = Focus::Worktrees;
                self.mode = InputMode::Normal;
                EventState::Consumed
            }
            Action::Select => self.handle_pr_url_submission(),
            _ => self.pr_worktree_component.handle_action(action),
        }
    }

    fn handle_pr_url_submission(&mut self) -> EventState {
        let url = self.pr_worktree_component.current_url().to_string();

        let pr_url = match github::parse_pr_url(&url) {
            Ok(p) => p,
            Err(e) => {
                self.pr_worktree_component.set_error(format!("{:#}", e));
                return EventState::Consumed;
            }
        };

        let pr_info = match github::fetch_pr_info(&pr_url) {
            Ok(info) => info,
            Err(e) => {
                self.pr_worktree_component.set_error(format!("{:#}", e));
                return EventState::Consumed;
            }
        };

        if !self.repositories_component.select_repository_by_name(&pr_url.repo) {
            self.pr_worktree_component.set_error(format!(
                "Repository '{}' not found in repos directory",
                pr_url.repo
            ));
            return EventState::Consumed;
        }

        let branch = pr_info.branch_name.clone();

        // Worktree already exists — select it and close popup
        if self.worktrees_component.select_worktree_by_branch(&branch) {
            self.pr_worktree_component.reset();
            self.focus = Focus::Worktrees;
            self.mode = InputMode::Normal;
            if pr_info.is_merged {
                self.worktrees_component.last_error =
                    Some("PR is merged — existing worktree selected".to_string());
            }
            return EventState::Consumed;
        }

        // No worktree yet — open CreateWorktree with branch pre-filled
        let warning = if pr_info.is_merged {
            Some("Warning: PR is merged, branch may be deleted on remote".to_string())
        } else {
            None
        };

        let base_branch_hint = self
            .repositories_component
            .selected_repository()
            .map(|r| r.resolve_base_branch(&branch));

        self.create_worktree =
            CreateWorktreeComponent::new_with_branch(pr_url.repo.clone(), branch, warning);
        self.create_worktree.base_branch_hint = base_branch_hint;

        self.pr_worktree_component.reset();
        self.focus = Focus::CreateWorktree;
        self.mode = InputMode::Insert;
        EventState::Consumed
    }

    fn help_bindings_for(focus: Focus, mode: InputMode) -> Vec<HelpEntry> {
        match (focus, mode) {
            (Focus::Worktrees, InputMode::Normal) => vec![
                HelpEntry::Section("Keybindings"),
                HelpEntry::Binding("j / ↓", "Move down"),
                HelpEntry::Binding("k / ↑", "Move up"),
                HelpEntry::Binding("g / Home", "Go to first"),
                HelpEntry::Binding("G / End", "Go to last"),
                HelpEntry::Binding("i / /", "Enter filter mode"),
                HelpEntry::Binding("Tab", "Toggle filter / list"),
                HelpEntry::Binding("n", "New worktree (pick repo)"),
                HelpEntry::Binding("p", "New worktree from PR URL"),
                HelpEntry::Binding("d", "Delete with confirmation"),
                HelpEntry::Binding("D", "Force delete"),
                HelpEntry::Binding("Enter", "Copy path to clipboard & exit"),
                HelpEntry::Binding("?", "Show this help"),
                HelpEntry::Binding("q / Ctrl+C", "Quit"),
                HelpEntry::Blank,
                HelpEntry::Section("Worktree State"),
                HelpEntry::Binding("✔", "Remote branch exists"),
                HelpEntry::Binding("✘", "Merged / deleted remotely"),
                HelpEntry::Binding("⬆", "Never pushed to remote"),
                HelpEntry::Binding("*", "Dirty working tree"),
            ],
            (Focus::Worktrees, InputMode::Insert) => vec![
                HelpEntry::Section("Keybindings"),
                HelpEntry::Binding("Esc", "Exit filter mode"),
                HelpEntry::Binding("Tab", "Toggle filter / list"),
                HelpEntry::Binding("↑ / Ctrl+K / Ctrl+P", "Move up in list"),
                HelpEntry::Binding("↓ / Ctrl+J / Ctrl+N", "Move down in list"),
                HelpEntry::Binding("Backspace", "Delete filter character"),
                HelpEntry::Binding("Enter", "Copy path to clipboard & exit"),
                HelpEntry::Binding("Ctrl+C", "Quit"),
            ],
            (Focus::Repositories, InputMode::Normal) => vec![
                HelpEntry::Section("Keybindings"),
                HelpEntry::Binding("j / ↓", "Move down"),
                HelpEntry::Binding("k / ↑", "Move up"),
                HelpEntry::Binding("g / Home", "Go to first"),
                HelpEntry::Binding("G / End", "Go to last"),
                HelpEntry::Binding("i", "Enter filter mode"),
                HelpEntry::Binding("Tab", "Toggle filter / list"),
                HelpEntry::Binding("Enter", "Select repository"),
                HelpEntry::Binding("?", "Show this help"),
                HelpEntry::Binding("Esc", "Close popup"),
                HelpEntry::Binding("q / Ctrl+C", "Quit"),
            ],
            (Focus::Repositories, InputMode::Insert) => vec![
                HelpEntry::Section("Keybindings"),
                HelpEntry::Binding("Esc", "Exit filter mode"),
                HelpEntry::Binding("Tab", "Toggle filter / list"),
                HelpEntry::Binding("↑ / Ctrl+K / Ctrl+P", "Move up in list"),
                HelpEntry::Binding("↓ / Ctrl+J / Ctrl+N", "Move down in list"),
                HelpEntry::Binding("Backspace", "Delete filter character"),
                HelpEntry::Binding("Enter", "Select repository"),
                HelpEntry::Binding("Ctrl+C", "Quit"),
            ],
            (Focus::CreateWorktree, _) => vec![
                HelpEntry::Section("Keybindings"),
                HelpEntry::Binding("Enter", "Create worktree"),
                HelpEntry::Binding("Esc", "Cancel"),
                HelpEntry::Binding("Backspace", "Delete character"),
                HelpEntry::Binding("Ctrl+C", "Quit"),
            ],
            (Focus::Confirm, _) => vec![
                HelpEntry::Section("Keybindings"),
                HelpEntry::Binding("Enter", "Confirm delete"),
                HelpEntry::Binding("Esc", "Cancel"),
                HelpEntry::Binding("q / Ctrl+C", "Quit"),
            ],
            (Focus::PrWorktree, _) => vec![
                HelpEntry::Section("Keybindings"),
                HelpEntry::Binding("Enter", "Fetch PR and open worktree"),
                HelpEntry::Binding("Esc", "Cancel"),
                HelpEntry::Binding("Backspace", "Delete character"),
                HelpEntry::Binding("Ctrl+C", "Quit"),
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
