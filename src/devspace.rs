use std::fs;
use std::io;

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
