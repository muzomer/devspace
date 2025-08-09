mod create_worktree;
mod filter;
mod list;
mod repositories;
mod worktrees;

pub use create_worktree::CreateWorktreeComponent;
use ratatui::style::{palette::tailwind::SLATE, Modifier, Style};
pub use repositories::RepositoriesComponent;
pub use worktrees::WorktreesComponent;

const SELECTED_STYLE: Style = Style::new().bg(SLATE.c800).add_modifier(Modifier::BOLD);

#[derive(PartialEq, Debug)]
pub enum EventState {
    Consumed,
    NotConsumed,
    Exit,
}
