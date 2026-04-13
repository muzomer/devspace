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
    palette::tailwind::{BLUE, SKY, SLATE},
    Modifier, Style,
};
pub use repositories::RepositoriesComponent;
pub use worktrees::WorktreesComponent;

/// Highlight for the currently selected list item.
const SELECTED_STYLE: Style = Style::new().bg(BLUE.c800).add_modifier(Modifier::BOLD);
/// Subtle blue-gray border for the main worktrees panel.
const BORDER_STYLE: Style = Style::new().fg(SLATE.c500);
/// Sky-blue border used by all popup / dialog windows.
const POPUP_BORDER_STYLE: Style = Style::new().fg(SKY.c600);
/// Bright sky-blue border for an active / focused text input.
const ACTIVE_BORDER_STYLE: Style = Style::new().fg(SKY.c400);

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
