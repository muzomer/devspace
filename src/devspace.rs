use std::fs;
use std::io;

use ratatui::widgets::ListState;

pub fn list(path: &str) -> io::Result<Vec<String>> {
    let mut devspaces: Vec<String> = vec![];
    for entry in list_dir(path)? {
        let mut paths = list_dir(&entry)?;
        devspaces.append(&mut paths);
    }

    Ok(devspaces)
}

pub fn list_dir(path: &str) -> io::Result<Vec<String>> {
    let mut paths: Vec<String> = vec![];
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        if let Some(path_str) = entry.path().to_str() {
            paths.push(path_str.to_string());
        };
    }

    Ok(paths)
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
