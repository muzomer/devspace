use color_eyre::eyre::{self};
use directories::ProjectDirs;
use std::path::PathBuf;
pub fn get_data_dir() -> eyre::Result<PathBuf> {
    let directory = if let Ok(s) = std::env::var("DEVSPACE_DATA") {
        PathBuf::from(s)
    } else if let Some(proj_dirs) = ProjectDirs::from("com", "muzomer", "devspace") {
        proj_dirs.data_local_dir().to_path_buf()
    } else {
        return Err(eyre::eyre!("Unable to find data directory for devspace"));
    };
    Ok(directory)
}
// pub fn get_config_dir() -> eyre::Result<PathBuf> {
//     let directory = if let Ok(s) = std::env::var("DEVSPACE_CONFIG") {
//         PathBuf::from(s)
//     } else if let Some(proj_dirs) = ProjectDirs::from("com", "muzomer", "devspace") {
//         proj_dirs.config_local_dir().to_path_buf()
//     } else {
//         return Err(eyre::eyre!("Unable to find config directory for devspace"));
//     };
//     Ok(directory)
// }
