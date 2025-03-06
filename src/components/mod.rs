mod filter;
mod list;
mod repositories;
mod worktrees;

pub use repositories::RepositoriesComponent;
pub use worktrees::WorktreesComponent;

#[derive(PartialEq, Debug)]
pub enum EventState {
    Consumed,
    NotConsumed,
}
