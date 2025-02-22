use ratatui::widgets::ListState;
use std::ffi::OsStr;
use std::fs::read_dir;
use std::io;
use std::path::Path;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct DevspaceList {
    pub items: Vec<String>,
    pub state: ListState,
    pub filter: String,
    pub filter_character_index: usize,
    pub filtered_items: Vec<String>,
}

impl DevspaceList {
    pub fn new(items: Vec<String>) -> Self {
        let mut new = Self {
            items: items.clone(),
            state: ListState::default(),
            filter: String::new(),
            filter_character_index: 0,
            filtered_items: items.clone(),
        };
        new.state.select_first();
        new
    }
    pub fn select_next(&mut self) {
        if let Some(index) = self.state.selected() {
            if index == self.filtered_items.len() - 1 {
                self.state.select_first();
            } else {
                self.state.select_next();
            }
        }
    }
    pub fn select_previous(&mut self) {
        if let Some(index) = self.state.selected() {
            if index == 0 {
                self.state.select_last();
            } else {
                self.state.select_previous();
            }
        }
    }
    pub fn select_first(&mut self) {
        self.state.select_first();
    }

    pub fn select_last(&mut self) {
        self.state.select_last();
    }

    pub fn move_filter_cursor_right(&mut self) {
        let cursor_moved_right = self.filter_character_index.saturating_add(1);
        self.filter_character_index = self.clamp_cursor(cursor_moved_right);
    }

    pub fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.filter.chars().count())
    }

    pub fn update_filtered_items(&mut self) {
        self.filtered_items = self
            .items
            .iter()
            .filter(|devspace| devspace.contains(&self.filter))
            .cloned()
            .collect();
    }

    pub fn enter_char(&mut self, new_char: char) {
        let index = self.byte_index();
        self.filter.insert(index, new_char);
        self.move_filter_cursor_right();
    }

    pub fn byte_index(&self) -> usize {
        self.filter
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.filter_character_index)
            .unwrap_or(self.filter.len())
    }

    pub fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.filter_character_index != 0;
        if is_not_cursor_leftmost {
            // Method "remove" is not used on the saved text for deleting the selected char.
            // Reason: Using remove on String works on bytes instead of the chars.
            // Using remove would require special care because of char boundaries.

            let current_index = self.filter_character_index;
            let from_left_to_current_index = current_index - 1;

            // Getting all characters before the selected character.
            let before_char_to_delete = self.filter.chars().take(from_left_to_current_index);
            // Getting all characters after selected character.
            let after_char_to_delete = self.filter.chars().skip(current_index);

            // Put all characters together except the selected one.
            // By leaving the selected one out, it is forgotten and therefore deleted.
            self.filter = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }
    pub fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.filter_character_index.saturating_sub(1);
        self.filter_character_index = self.clamp_cursor(cursor_moved_left);
    }
}

pub fn list(path: &str) -> io::Result<Vec<String>> {
    let mut devspaces: Vec<String> = vec![];
    for entry in get_git_subdirs(Path::new(path))? {
        if let Some(entry_path) = entry.to_str() {
            devspaces.push(entry_path.to_string());
        }
    }

    Ok(devspaces)
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
