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

impl Clone for Worktree {
    fn clone(&self) -> Self {
        let repo = git2::Repository::discover(self.path()).expect("Could not open repository");
        let worktree = repo
            .find_worktree(self.name())
            .expect("Could not find worktree");
        Worktree {
            git_worktree: worktree,
            has_remote_branch: self.has_remote_branch,
        }
    }
}

pub fn delete_worktree(worktree: &Worktree) {
    let worktree_path = Path::new(worktree.path());
    if worktree_path.exists() {
        let _ = fs::remove_dir_all(worktree_path)
            .inspect_err(|_| debug!("Could not delete the worktree {}", worktree.name()));
    } else {
        debug!(
            "Skipping deletion of worktree {} does not exist in the filesystem",
            worktree.name()
        );
    }
}
