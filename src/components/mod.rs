mod confirm;
mod create_worktree;
mod filter;
mod help;
mod list;
mod pr_worktree;
mod repositories;
mod worktrees;

pub use confirm::ConfirmComponent;
pub use create_worktree::CreateWorktreeComponent;
pub use help::{HelpComponent, HelpEntry};
pub use pr_worktree::PrWorktreeComponent;
use ratatui::style::{
    palette::tailwind::{BLUE, CYAN, SLATE},
    Modifier, Style,
};
pub use repositories::RepositoriesComponent;
pub use worktrees::WorktreesComponent;

/// Selected item: vivid blue matching lazygit's selectedLineBgColor.
const SELECTED_STYLE: Style = Style::new().bg(BLUE.c700).add_modifier(Modifier::BOLD);
/// Vivid cyan borders matching lazygit's activeBorderColor (#22d3ee).
const BORDER_STYLE: Style = Style::new().fg(CYAN.c400);
/// Same vivid cyan border for popups; elevation conveyed via POPUP_BG_STYLE.
const POPUP_BORDER_STYLE: Style = Style::new().fg(CYAN.c400);
/// Slightly lighter dark background that lifts popup / dialog windows above the main panel.
const POPUP_BG_STYLE: Style = Style::new().bg(SLATE.c800);
/// Bright cyan + bold for active input — matches lazygit's [cyan, bold] active border.
const ACTIVE_BORDER_STYLE: Style = Style::new().fg(CYAN.c300).add_modifier(Modifier::BOLD);

#[derive(PartialEq, Debug)]
pub enum EventState {
    Consumed,
    NotConsumed,
    Exit,
}

#[derive(PartialEq, Debug)]
pub enum Action {
    MoveDown,
    MoveUp,
    GoFirst,
    GoLast,
    Select,
    Delete,
    DeleteWithConfirmation,
    ForceDelete,
    OpenRepositories,
    OpenPrWorktree,
    OpenPrWorktreeAutoClone,
    ClosePopup,
    EnterInsertMode,
    ExitInsertMode,
    InsertChar(char),
    DeleteChar,
    FocusNext,
    ShowHelp,
    Quit,
}
