use std::path::PathBuf;

use directories::ProjectDirs;

/// A utility function to get the correct "project" directories in a platform specific manner
#[inline]
fn project_dirs() -> Option<ProjectDirs> {
    directories::ProjectDirs::from("io", "Seaplane", "seaplane")
}

/// Finds all appropriate configuration directories in a platform specific manner
pub fn conf_dirs() -> Vec<PathBuf> {
    let mut dirs = Vec::new();
    if let Some(proj_dirs) = project_dirs() {
        dirs.push(proj_dirs.config_dir().to_owned());
    }
    if let Some(base_dirs) = directories::BaseDirs::new() {
        // On Linux ProjectDirs already adds ~/.config/seaplane, but not on macOS or Windows
        if !cfg!(target_os = "linux") {
            dirs.push(base_dirs.home_dir().join(".config/seaplane"));
        }
        dirs.push(base_dirs.home_dir().join(".seaplane"));
    }
    dirs
}

/// A utility function to get the correct data directory
#[inline]
pub fn data_dir() -> PathBuf {
    project_dirs()
        .expect("Failed to determine usable directories")
        .data_dir()
        .to_owned()
}
