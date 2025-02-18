use std::ffi::OsStr;
use std::fs::read_dir;
use std::fs::DirEntry;
use std::io;
use std::path::Path;

use ratatui::widgets::ListState;

pub fn list(path: &str) -> io::Result<Vec<String>> {
    let mut devspaces: Vec<String> = vec![];
    for entry in get_git_subdirs(Path::new(path))? {
        if let Some(entry_path) = entry.path().to_str() {
            devspaces.push(entry_path.to_string());
        }
    }

    Ok(devspaces)
}

pub fn is_git_dir(dir: &Path) -> io::Result<bool> {
    let entries = read_dir(dir)?;
    let mut result = false;
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() && path.file_name() == Some(OsStr::new(".git")) {
            result = true;
            break;
        }
    }

    Ok(result)
}

pub fn get_git_subdirs(path: &Path) -> io::Result<Vec<DirEntry>> {
    let mut git_subdirs: Vec<DirEntry> = Vec::new();
    let entries = read_dir(path)?;
    for entry in entries.flatten() {
        if let Ok(true) = is_git_dir(&entry.path()) {
            git_subdirs.push(entry);
        } else {
            for entry in read_dir(entry.path())?.flatten() {
                let entry_git_subdirs = get_git_subdirs(&entry.path())?;
                git_subdirs.extend(entry_git_subdirs);
            }
        }
    }

    Ok(git_subdirs)
}

#[derive(Debug, Clone)]
pub struct DevspaceList {
    pub items: Vec<String>,
    pub state: ListState,
}

impl DevspaceList {
    pub fn new(items: Vec<String>) -> Self {
        let state = ListState::default();

        Self { items, state }
    }
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
                git_subdirs.iter().any(|dir| dir.path() == expected_dir),
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

        let git_subdirs = list(
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
