use color_eyre::eyre;
use color_eyre::eyre::WrapErr;
use git2::{Cred, RemoteCallbacks};
use rayon::prelude::*;
use std::{
    ffi::OsStr,
    fs::{self, read_dir},
    path::{Path, PathBuf},
};
use tracing::{debug, error};

pub struct Repository(git2::Repository);
impl Repository {
    pub fn from_path(path: &str, run_fetch: bool) -> eyre::Result<Self> {
        let repo = git2::Repository::open(path)
            .wrap_err_with(|| format!("Could not open repository at {}", path))?;
        if run_fetch {
            // git fetch --prune
            if let Err(err) = repo.remotes().map(|remotes| {
                remotes.iter().for_each(|remote| {
                    if let Some(name) = remote {
                        if let Err(e) = fetch_with_prune(&repo, name) {
                            debug!("Could not fetch from remote. Error: {}", e);
                        }
                    }
                });
            }) {
                debug!("Could not fetch from remotes. Error: {}", err);
            }
        }
        Ok(Self(repo))
    }
    pub fn create_new_worktree(
        &self,
        worktree_name: &str,
        worktrees_dir: &str,
    ) -> eyre::Result<super::Worktree> {
        let repo_worktrees_dir = PathBuf::from(worktrees_dir).join(self.name());
        let new_worktree_dir = PathBuf::from(&repo_worktrees_dir).join(worktree_name);

        fs::create_dir_all(&repo_worktrees_dir).wrap_err_with(|| {
            format!(
                "Could not create worktrees directory {:?}",
                repo_worktrees_dir
            )
        })?;

        let remote_branch = self.find_remote_branch_oid(worktree_name);

        let created_worktree = if let Some((remote_name, oid)) = remote_branch {
            debug!(
                "Found remote branch '{}/{}', creating tracking branch",
                remote_name, worktree_name
            );
            let commit = self
                .0
                .find_commit(oid)
                .wrap_err("Could not find commit for remote branch")?;
            let mut local_branch = self
                .0
                .branch(worktree_name, &commit, false)
                .wrap_err_with(|| format!("Could not create local branch '{}'", worktree_name))?;
            let upstream_name = format!("{}/{}", remote_name, worktree_name);
            local_branch
                .set_upstream(Some(&upstream_name))
                .wrap_err_with(|| format!("Could not set upstream to '{}'", upstream_name))?;

            let branch_ref = local_branch.into_reference();
            let mut opts = git2::WorktreeAddOptions::new();
            opts.checkout_existing(true);
            opts.reference(Some(&branch_ref));
            self.0
                .worktree(worktree_name, new_worktree_dir.as_path(), Some(&opts))
                .wrap_err_with(|| format!("Could not create worktree '{}'", worktree_name))?
        } else {
            let mut opts = git2::WorktreeAddOptions::new();
            opts.checkout_existing(true);
            self.0
                .worktree(worktree_name, new_worktree_dir.as_path(), Some(&opts))
                .wrap_err_with(|| format!("Could not create worktree '{}'", worktree_name))?
        };

        let branch = self
            .0
            .find_branch(worktree_name, git2::BranchType::Local)
            .wrap_err_with(|| {
                format!(
                    "Could not find branch '{}' after creating worktree",
                    worktree_name
                )
            })?;

        Ok(super::Worktree {
            git_worktree: created_worktree,
            has_remote_branch: branch.upstream().is_ok(),
        })
    }

    /// Searches all remotes for a branch named `branch_name` and returns the
    /// remote name and the OID of its tip commit, if found.
    fn find_remote_branch_oid(&self, branch_name: &str) -> Option<(String, git2::Oid)> {
        let remotes = self.0.remotes().ok()?;
        for remote_name in remotes.iter().flatten() {
            let refname = format!("refs/remotes/{}/{}", remote_name, branch_name);
            if let Ok(reference) = self.0.find_reference(&refname) {
                if let Ok(oid) = reference.peel_to_commit().map(|c| c.id()) {
                    debug!("Found remote branch ref '{}' at {}", refname, oid);
                    return Some((remote_name.to_string(), oid));
                }
            }
        }
        None
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

    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(|_url, username_from_url, _allowed_types| {
        Cred::ssh_key_from_agent(username_from_url.unwrap_or("git"))
    });
    fetch_opts.prune(git2::FetchPrune::On);
    fetch_opts.remote_callbacks(callbacks);
    git_repo
        .find_remote(remote_name)?
        .fetch(&refspecs, Some(&mut fetch_opts), None)?;
    Ok(())
}
pub fn list_repositories(path: &str, run_fetch: bool) -> Vec<Repository> {
    debug!("Listing repositories in: {}", path);
    find_git_dirs(Path::new(path))
        .par_iter()
        .filter_map(|dir| match Repository::from_path(dir, run_fetch) {
            Ok(repo) => Some(repo),
            Err(err) => {
                error!("Could not open repository at {}: {}", dir, err);
                None
            }
        })
        .collect()
}

pub fn worktrees_of_repositories(repositories: &[Repository]) -> Vec<super::Worktree> {
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
            false
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

    match read_dir(path) {
        Err(err) => {
            error!("Could not read the directory {}: {}", path.display(), err);
            git_dirs
        }
        Ok(entries) => {
            for entry in entries.flatten() {
                if !entry.path().is_dir() {
                    continue;
                }

                if is_git_dir(&entry.path()) {
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
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    /// Create a git repository with a single commit and return it along with the commit OID.
    fn init_repo_with_commit(path: &Path) -> (git2::Repository, git2::Oid) {
        let repo = git2::Repository::init(path).expect("Could not init repo");
        let sig = git2::Signature::now("Test", "test@example.com").unwrap();
        let oid = {
            let tree_id = {
                let mut tb = repo.treebuilder(None).unwrap();
                let blob = repo.blob(b"hello").unwrap();
                tb.insert("file.txt", blob, 0o100644).unwrap();
                tb.write().unwrap()
            };
            let tree = repo.find_tree(tree_id).unwrap();
            repo.commit(Some("HEAD"), &sig, &sig, "initial commit", &tree, &[])
                .expect("Could not create initial commit")
            // tree is dropped here before repo is moved
        };
        (repo, oid)
    }

    #[test]
    fn test_find_remote_branch_oid_no_remote() {
        let repo_dir = tempdir().expect("Could not create temporary directory");
        let (_, _) = init_repo_with_commit(repo_dir.path());
        let repo =
            Repository(git2::Repository::open(repo_dir.path()).expect("Could not open repo"));
        assert!(
            repo.find_remote_branch_oid("feature-branch").is_none(),
            "Expected None when no remote branch exists"
        );
    }

    #[test]
    fn test_find_remote_branch_oid_with_remote_ref() {
        let repo_dir = tempdir().expect("Could not create temporary directory");
        let (git_repo, oid) = init_repo_with_commit(repo_dir.path());
        // Simulate a fetched remote ref without actually having a remote URL
        git_repo
            .reference(
                "refs/remotes/origin/feature-branch",
                oid,
                false,
                "simulated fetch",
            )
            .expect("Could not create remote ref");
        // Add an entry in config so git2 treats "origin" as a remote
        {
            let mut config = git_repo.config().unwrap();
            config
                .set_str("remote.origin.url", "git@github.com:example/repo.git")
                .unwrap();
            config
                .set_str("remote.origin.fetch", "+refs/heads/*:refs/remotes/origin/*")
                .unwrap();
        }
        drop(git_repo);

        let repo =
            Repository(git2::Repository::open(repo_dir.path()).expect("Could not open repo"));
        let result = repo.find_remote_branch_oid("feature-branch");
        assert!(
            result.is_some(),
            "Expected Some when remote branch ref exists"
        );
        let (remote_name, found_oid) = result.unwrap();
        assert_eq!(remote_name, "origin");
        assert_eq!(found_oid, oid);
    }

    #[test]
    fn test_create_worktree_from_remote_branch() {
        let repo_dir = tempdir().expect("Could not create repo directory");
        let worktrees_dir = tempdir().expect("Could not create worktrees directory");
        let (git_repo, oid) = init_repo_with_commit(repo_dir.path());

        // Simulate a fetched remote ref
        git_repo
            .reference(
                "refs/remotes/origin/feature-branch",
                oid,
                false,
                "simulated fetch",
            )
            .expect("Could not create remote ref");
        {
            let mut config = git_repo.config().unwrap();
            config
                .set_str("remote.origin.url", "git@github.com:example/repo.git")
                .unwrap();
            config
                .set_str("remote.origin.fetch", "+refs/heads/*:refs/remotes/origin/*")
                .unwrap();
        }
        drop(git_repo);

        let repo =
            Repository(git2::Repository::open(repo_dir.path()).expect("Could not open repo"));
        let worktree = repo
            .create_new_worktree("feature-branch", worktrees_dir.path().to_str().unwrap())
            .expect("Could not create worktree from remote branch");

        assert!(
            worktree.has_remote_branch,
            "Expected has_remote_branch to be true when created from remote branch"
        );

        // Verify the local branch was created with the upstream set
        let git_repo = git2::Repository::open(repo_dir.path()).expect("Could not open repo");
        let branch = git_repo
            .find_branch("feature-branch", git2::BranchType::Local)
            .expect("Local branch should exist");
        assert!(
            branch.upstream().is_ok(),
            "Local branch should track the remote branch"
        );
    }

    #[test]
    fn test_create_worktree_no_remote_branch() {
        let repo_dir = tempdir().expect("Could not create repo directory");
        let worktrees_dir = tempdir().expect("Could not create worktrees directory");
        init_repo_with_commit(repo_dir.path());

        let repo =
            Repository(git2::Repository::open(repo_dir.path()).expect("Could not open repo"));
        let worktree = repo
            .create_new_worktree("local-only-branch", worktrees_dir.path().to_str().unwrap())
            .expect("Could not create worktree");

        assert!(
            !worktree.has_remote_branch,
            "Expected has_remote_branch to be false when no remote branch exists"
        );
    }

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
