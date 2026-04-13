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
    palette::tailwind::{BLUE, GREEN, SLATE},
    Modifier, Style,
};
pub use repositories::RepositoriesComponent;
pub use worktrees::WorktreesComponent;

/// Selected item: blue bg matching lazygit's selectedLineBgColor.
const SELECTED_STYLE: Style = Style::new().bg(BLUE.c800).add_modifier(Modifier::BOLD);
/// Muted border for inactive/main panel — lazygit's inactiveBorderColor is terminal default.
const BORDER_STYLE: Style = Style::new().fg(SLATE.c500);
/// Green border for popups (active elements) — lazygit's activeBorderColor is [green, bold].
const POPUP_BORDER_STYLE: Style = Style::new().fg(GREEN.c400).add_modifier(Modifier::BOLD);
/// Slightly lighter dark background that lifts popup / dialog windows above the main panel.
const POPUP_BG_STYLE: Style = Style::new().bg(SLATE.c800);
/// Green bold border for active/focused text input — matches lazygit's [green, bold].
const ACTIVE_BORDER_STYLE: Style = Style::new().fg(GREEN.c400).add_modifier(Modifier::BOLD);

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
