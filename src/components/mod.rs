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

/// Highlight for the currently selected list item.
const SELECTED_STYLE: Style = Style::new().bg(BLUE.c800).add_modifier(Modifier::BOLD);
/// Cyan border matching lazygit's panel border style.
const BORDER_STYLE: Style = Style::new().fg(CYAN.c600);
/// Same cyan border for popup / dialog windows; elevation via POPUP_BG_STYLE.
const POPUP_BORDER_STYLE: Style = Style::new().fg(CYAN.c600);
/// Slightly lighter dark background that lifts popup / dialog windows above the main panel.
const POPUP_BG_STYLE: Style = Style::new().bg(SLATE.c800);
/// Brighter cyan border for an active / focused text input.
const ACTIVE_BORDER_STYLE: Style = Style::new().fg(CYAN.c400);

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
