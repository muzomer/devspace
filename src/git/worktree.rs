use std::{
    fs::{self},
    path::Path,
};
use tracing::debug;

pub struct Worktree {
    pub git_worktree: git2::Worktree,
    pub has_remote_branch: bool,
}
impl Worktree {
    pub fn path(&self) -> &str {
        self.git_worktree
            .path()
            .to_str()
            .expect("Could not get worktree path")
    }

    pub fn name(&self) -> &str {
        self.git_worktree
            .name()
            .expect("Could not get worktree name")
    }
}

pub fn delete_worktree(worktree: &Worktree) {
    let worktree_path = Path::new(worktree.path());
    if let Err(e) = worktree.git_worktree.prune(None) {
        debug!("Failed to prune worktree '{}': {}", worktree.name(), e);
    }

    if worktree_path.exists() {
        if let Err(e) = fs::remove_dir_all(worktree_path) {
            debug!(
                "Failed to remove worktree directory for '{}': {}",
                worktree.name(),
                e
            );
        }
    }
}
