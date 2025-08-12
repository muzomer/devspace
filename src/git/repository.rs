use rayon::prelude::*;
use std::{
    ffi::OsStr,
    fs::{self, read_dir},
    path::{Path, PathBuf},
};
use tracing::{debug, error};

pub struct Repository(git2::Repository);
impl Repository {
    pub fn from_path(path: &str) -> Result<Self, git2::Error> {
        let repo = git2::Repository::open(path)?;
        // git fetch --prune
        if let Err(err) = repo.remotes().and_then(|remotes| {
            remotes.iter().for_each(|remote| {
                if let Err(e) = fetch_with_prune(&repo, remote.expect("Could not get remote name"))
                {
                    debug!("Could not fetch from remote. Error: {}", e);
                }
            });
            Ok(())
        }) {
            debug!("Could not fetch from remotes. Error: {}", err);
        }
        Ok(Self(repo))
    }
    pub fn create_new_worktree(
        &self,
        worktree_name: &str,
        worktrees_dir: &str,
    ) -> Option<super::Worktree> {
        let repo_worktrees_dir = PathBuf::from(worktrees_dir).join(self.name());
        let new_worktree_dir = PathBuf::from(&repo_worktrees_dir).join(worktree_name);

        let _ = fs::create_dir_all(&repo_worktrees_dir);

        let mut create_worktree_options = git2::WorktreeAddOptions::new();
        create_worktree_options.checkout_existing(true);
        let result = self.0.worktree(
            worktree_name,
            new_worktree_dir.as_path(),
            Some(&create_worktree_options),
        );

        match result {
            Ok(created_worktree) => {
                let branch = self
                    .0
                    .find_branch(worktree_name, git2::BranchType::Local)
                    .unwrap();
                Some(super::Worktree {
                    git_worktree: created_worktree,
                    has_remote_branch: branch.upstream().is_ok(),
                })
            }
            Err(error) => {
                panic!(
                    "Could not create the worktree {}. Error: {}",
                    worktree_name, error
                );
            }
        }
    }

    pub fn name(&self) -> String {
        let path = String::from(self.0.path().to_str().unwrap());
        path.replace("/.git/", "")
            .split("/")
            .last()
            .unwrap()
            .to_string()
    }

    pub fn worktrees(&self) -> Vec<super::Worktree> {
        let mut git_worktrees: Vec<super::Worktree> = Vec::new();
        match self.0.worktrees() {
            Ok(worktrees_arr) => {
                worktrees_arr.iter().for_each(|worktree| {
                    if let Some(worktree_name) = worktree {
                        if let Ok(git_worktree) = self.0.find_worktree(worktree_name) {
                            let branch = self.0.find_branch(worktree_name, git2::BranchType::Local);

                            let has_remote_branch = match branch {
                                Ok(branch) => branch.upstream().is_ok(),
                                Err(_) => false,
                            };

                            git_worktrees.push(super::Worktree {
                                git_worktree,
                                has_remote_branch,
                            });
                        }
                    }
                });
            }
            Err(error) => {
                error!("Could not list the worktrees for repository {}", error);
            }
        };
        git_worktrees
    }
}

fn fetch_with_prune(git_repo: &git2::Repository, remote_name: &str) -> Result<(), git2::Error> {
    let refspecs: Vec<String> = vec![];
    let mut fetch_opts = git2::FetchOptions::new();
    fetch_opts.prune(git2::FetchPrune::On);

    let callbacks = git2::RemoteCallbacks::new();
    fetch_opts.remote_callbacks(callbacks);

    git_repo
        .find_remote(remote_name)?
        .fetch(&refspecs, Some(&mut fetch_opts), None)?;

    Ok(())
}
pub fn list_repositories(path: &str) -> Vec<Repository> {
    debug!("Listing repositories in: {}", path);
    let repositories: Vec<Repository> = find_git_dirs(Path::new(path))
        .par_iter()
        .filter_map(|dir| match Repository::from_path(dir) {
            Ok(created_repo) => Some(created_repo),
            Err(err) => {
                error!(
                    "Could not create repository from path {}. Error: {}",
                    dir, err
                );
                None
            }
        })
        .collect();

    repositories
}

pub fn worktrees_of_repositories(repositories: &Vec<Repository>) -> Vec<super::Worktree> {
    let mut worktrees: Vec<super::Worktree> = Vec::new();
    repositories.iter().for_each(|repo| {
        worktrees.append(&mut repo.worktrees());
    });
    worktrees
}

fn is_git_dir(dir: &Path) -> bool {
    if !dir.is_dir() {
        return false;
    }
    match read_dir(dir) {
        Ok(entries) => {
            let mut result = false;
            for entry in entries.flatten() {
                let path = entry.path();
                if path.file_name() == Some(OsStr::new(".git")) {
                    result = true;
                    break;
                }
            }
            result
        }
        Err(err) => {
            error!("Could not read the directory {}: {}", dir.display(), err);
            return false;
        }
    }
}

fn find_git_dirs(path: &Path) -> Vec<String> {
    let mut git_dirs: Vec<String> = vec![];

    if !path.is_dir() {
        return git_dirs;
    }

    if is_git_dir(path) {
        debug!("Found git repository at: {:?}", path);
        git_dirs.push(path.to_path_buf().display().to_string());
        return git_dirs;
    }

    return match read_dir(path) {
        Err(err) => {
            error!("Could not read the directory {}: {}", path.display(), err);
            return git_dirs;
        }
        Ok(entries) => {
            for entry in entries.flatten() {
                if !entry.path().is_dir() {
                    continue;
                }

                if let true = is_git_dir(&entry.path()) {
                    debug!("Found git repository at: {:?}", entry.path());
                    git_dirs.push(entry.path().to_path_buf().display().to_string());
                } else {
                    debug!(
                        "No git repository found at: {:?}, continuing search",
                        entry.path()
                    );
                    let sub_entries = find_git_dirs(&entry.path());
                    git_dirs.extend(sub_entries);
                }
            }
            git_dirs
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_not_git_dir() {
        let temp_dir = tempdir().expect("Could not create temporary directory");
        assert!(
            !is_git_dir(temp_dir.path()),
            "Expected is_git_dir to be false, but it was true"
        );
    }

    #[test]
    fn test_git_dir() {
        let temp_dir = tempdir().expect("Could not create temporary directory");
        fs::DirBuilder::new()
            .create(temp_dir.path().join(".git"))
            .expect("Could not create .git directory inside the temporary dir");

        assert!(
            is_git_dir(temp_dir.path()),
            "Expected is_git_dir to be true, but it was false"
        );
    }

    #[test]
    fn test_list() {
        let temp_dir = tempdir().expect("Could not create temporary directory");

        for path in [
            "first_git_dir/.git",
            "second_git_dir/.git",
            "third_git_dir/subdir/subdir/",
            "fourth_git_dir/subdir/subdir/.git",
        ] {
            fs::DirBuilder::new()
                .recursive(true)
                .create(temp_dir.path().join(path))
                .unwrap_or_else(|_| {
                    panic!(
                        "Could not create {} directory inside the temporary dir",
                        path
                    )
                });
        }

        for path in [
            "first_git_dir",
            "second_git_dir",
            "fourth_git_dir/subdir/subdir",
        ] {
            let expected_dir = temp_dir.path().join(path);
            assert!(
                find_git_dirs(temp_dir.path())
                    .iter()
                    .any(|dir| dir == expected_dir.to_str().unwrap()),
                "Expected {} to be listed in the git subdirectories, but it was not included",
                expected_dir.to_str().unwrap()
            )
        }
    }
}
