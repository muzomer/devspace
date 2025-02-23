mod git;

pub struct Repository {
    pub git_repo: git2::Repository,
    pub path: String,
}

pub struct Devspace {
    pub git_worktree: git2::Worktree,
    pub path: String,
}

impl Devspace {
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
                    Ok(created_devspace) => Some(created_devspace),
                    Err(err) => {
                        println!(
                            "Could not create devspace from path {}. Error: {}",
                            dir, err
                        );
                        None
                    }
                })
                .collect(),
            Err(err) => {
                println!(
                    "Could not retrieve the git directories for devspaces: {}",
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
                    Ok(created_devspace) => Some(created_devspace),
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
