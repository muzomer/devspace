use git2::WorktreeAddOptions;
use std::{
    ffi::OsStr,
    fs::{self, read_dir},
    io,
    path::{Path, PathBuf},
};
use tracing::error;

pub struct Worktree(git2::Worktree);
impl Worktree {
    pub fn path(&self) -> &str {
        self.0.path().to_str().unwrap()
    }
}

pub struct Repository(git2::Repository);
impl Repository {
    pub fn from_path(path: &str) -> Result<Self, git2::Error> {
        Ok(Self(git2::Repository::open(path)?))
    }
    pub fn create_new_worktree(
        &self,
        worktree_name: &str,
        worktrees_dir: &str,
    ) -> Option<Worktree> {
        let repo_worktrees_dir = PathBuf::from(worktrees_dir).join(self.name());
        let new_worktree_dir = PathBuf::from(&repo_worktrees_dir).join(worktree_name);

        // Create the directory to store the worktrees of the selected repository
        let _ = fs::create_dir_all(&repo_worktrees_dir);

        let mut create_worktree_options = WorktreeAddOptions::new();
        create_worktree_options.checkout_existing(true);
        let result = self.0.worktree(
            worktree_name,
            new_worktree_dir.as_path(),
            Some(&create_worktree_options),
        );

        match result {
            Ok(created_worktree) => Some(Worktree(created_worktree)),
            Err(error) => {
                panic!(
                    "Could not create the worktree {}. Error: {}",
                    worktree_name, error
                );
                // None
            }
        }
    }

    // fn cleanup(&self) {
    // TODO: remove the worktree directory if exists
    // TODO: remove the branch if exists
    // TODO: remove the worktree from the repo/.git/worktree directory
    // }

    pub fn name(&self) -> String {
        let path = String::from(self.0.path().to_str().unwrap());
        path.replace("/.git/", "")
            .split("/")
            .last()
            .unwrap()
            .to_string()
    }

    pub fn worktrees(&self) -> Vec<Worktree> {
        let mut git_worktrees: Vec<Worktree> = Vec::new();
        match self.0.worktrees() {
            Ok(worktrees_arr) => {
                worktrees_arr.iter().for_each(|worktree| {
                    if let Some(worktree_name) = worktree {
                        if let Ok(git_worktree) = self.0.find_worktree(worktree_name) {
                            git_worktrees.push(Worktree(git_worktree));
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

pub fn list_repositories(path: &str) -> Vec<Repository> {
    match list_git_dirs(path) {
        Ok(git_dirs) => git_dirs
            .iter()
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
            .collect(),
        Err(err) => {
            error!(
                "Could not retrieve the git directories for repositories: {}",
                err
            );
            Vec::new()
        }
    }
}

pub fn list_git_dirs(path: &str) -> io::Result<Vec<String>> {
    let mut git_dirs: Vec<String> = vec![];
    for entry in get_git_subdirs(Path::new(path))? {
        if let Some(entry_path) = entry.to_str() {
            git_dirs.push(entry_path.to_string());
        }
    }

    Ok(git_dirs)
}

pub fn is_git_dir(dir: &Path) -> io::Result<bool> {
    if !dir.is_dir() {
        return Ok(false);
    }

    let entries = read_dir(dir)?;
    let mut result = false;
    for entry in entries.flatten() {
        let path = entry.path();
        if path.file_name() == Some(OsStr::new(".git")) {
            result = true;
            break;
        }
    }

    Ok(result)
}

pub fn get_git_subdirs(path: &Path) -> io::Result<Vec<PathBuf>> {
    let mut git_subdirs: Vec<PathBuf> = Vec::new();

    if !path.is_dir() {
        return Ok(git_subdirs);
    }

    if is_git_dir(path)? {
        git_subdirs.push(path.to_path_buf());
        return Ok(git_subdirs);
    }

    let entries = read_dir(path)?;
    for entry in entries.flatten() {
        if !entry.path().is_dir() {
            continue;
        }

        if let Ok(true) = is_git_dir(&entry.path()) {
            git_subdirs.push(entry.path().to_path_buf());
        } else {
            for entry in read_dir(entry.path())?.flatten() {
                let entry_git_subdirs = get_git_subdirs(&entry.path())?;
                git_subdirs.extend(entry_git_subdirs);
            }
        }
    }

    Ok(git_subdirs)
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
            !is_git_dir(temp_dir.path())
                .expect("is_git_dir failed to check whether temporary path is git directory"),
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
            is_git_dir(temp_dir.path())
                .expect("is_git_dir failed to check whether temporary path is git directory"),
            "Expected is_git_dir to be true, but it was false"
        );
    }

    #[test]
    fn test_get_gitsubdirs() {
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

        let git_subdirs = get_git_subdirs(temp_dir.path()).unwrap();

        for path in [
            "first_git_dir/",
            "second_git_dir",
            "fourth_git_dir/subdir/subdir",
        ] {
            let expected_dir = temp_dir.path().join(path);
            assert!(
                git_subdirs
                    .iter()
                    .any(|dir| dir.to_path_buf() == expected_dir),
                "Expected {} to be listed in the git subdirectories, but it was not included",
                expected_dir.to_str().unwrap()
            )
        }
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

        let git_subdirs = list_git_dirs(
            temp_dir
                .path()
                .to_str()
                .expect("Could not convert temporary path to string"),
        )
        .expect("Could not list all git subdirectories");

        for path in [
            "first_git_dir",
            "second_git_dir",
            "fourth_git_dir/subdir/subdir",
        ] {
            let expected_dir = temp_dir.path().join(path);
            assert!(
                git_subdirs
                    .iter()
                    .any(|dir| dir == expected_dir.to_str().unwrap()),
                "Expected {} to be listed in the git subdirectories, but it was not included",
                expected_dir.to_str().unwrap()
            )
        }
    }
}
