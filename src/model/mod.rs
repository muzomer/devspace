mod git;

pub struct Repository {
    pub git_repo: git2::Repository,
    pub path: String,
}

impl Clone for Repository {
    fn clone(&self) -> Self {
        let clone_repo = git2::Repository::open(self.git_repo.path())
            .unwrap_or_else(|_| panic!("Could not clone the git repo {}", self.path));

        Self {
            path: self.path.to_string(),
            git_repo: clone_repo,
        }
    }
}

pub struct Worktree {
    pub git_worktree: git2::Worktree,
    pub path: String,
}

impl Worktree {
    pub fn from_path(path: &str) -> Result<Self, git2::Error> {
        let repo = git2::Repository::open(path)?;
        let worktree = git2::Worktree::open_from_repository(&repo)?;
        Ok(Self {
            git_worktree: worktree,
            path: path.to_string(),
        })
    }

    pub fn list(path: &str) -> Vec<Self> {
        match git::list_git_dirs(path) {
            Ok(git_dirs) => git_dirs
                .iter()
                .filter_map(|dir| match Self::from_path(dir) {
                    Ok(created_worktree) => Some(created_worktree),
                    Err(err) => {
                        println!(
                            "Could not create worktree from path {}. Error: {}",
                            dir, err
                        );
                        None
                    }
                })
                .collect(),
            Err(err) => {
                println!(
                    "Could not list the directories of the git worktrees: {}",
                    err
                );
                Vec::new()
            }
        }
    }
}

impl Repository {
    pub fn from_path(path: &str) -> Result<Self, git2::Error> {
        let repo = git2::Repository::open(path)?;
        Ok(Self {
            git_repo: repo,
            path: path.to_string(),
        })
    }

    pub fn list(path: &str) -> Vec<Self> {
        match git::list_git_dirs(path) {
            Ok(git_dirs) => git_dirs
                .iter()
                .filter_map(|dir| match Self::from_path(dir) {
                    Ok(created_repo) => Some(created_repo),
                    Err(err) => {
                        println!(
                            "Could not create repository from path {}. Error: {}",
                            dir, err
                        );
                        None
                    }
                })
                .collect(),
            Err(err) => {
                println!(
                    "Could not retrieve the git directories for repositories: {}",
                    err
                );
                Vec::new()
            }
        }
    }
}
