use std::{
    fs::{self},
    io,
    path::Path,
};

pub struct Worktree {
    pub git_worktree: git2::Worktree,
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
        }
    }
}

pub fn delete_worktree(worktree: &Worktree) -> io::Result<()> {
    let worktree_path = Path::new(worktree.path());
    if worktree_path.exists() {
        fs::remove_dir_all(worktree_path)?;
    }
    worktree.git_worktree.prune(None).unwrap();
    Ok(())
}
