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
use ratatui::style::{palette::tailwind::SLATE, Color, Modifier, Style};
pub use repositories::RepositoriesComponent;
pub use worktrees::WorktreesComponent;

const SELECTED_STYLE: Style = Style::new().bg(SLATE.c800).add_modifier(Modifier::BOLD);
const BORDER_STYLE: Style = Style::new().fg(Color::DarkGray);

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
