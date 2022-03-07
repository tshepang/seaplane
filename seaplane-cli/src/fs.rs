use std::{
    fs::{self, File},
    path::{Path, PathBuf},
};

use directories::ProjectDirs;
use serde::{de::DeserializeOwned, Serialize};

use crate::{
    error::{CliError, CliErrorKind, Context, Result},
    printer::Color,
};

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
#[cfg(not(feature = "ui_tests"))]
#[inline]
pub fn data_dir() -> PathBuf {
    project_dirs()
        .expect("Failed to determine usable directories")
        .data_dir()
        .to_owned()
}

#[cfg(feature = "ui_tests")]
#[cfg_attr(feature = "ui_tests", inline)]
pub fn data_dir() -> PathBuf {
    std::env::current_dir().unwrap()
}

// TODO: make the deserializer generic
pub trait FromDisk {
    /// Allows one to save or deserialize what path the item was loaded from
    fn set_loaded_from<P: AsRef<Path>>(&mut self, _p: P) {}

    /// If saved, get the path the item was loaded from
    fn loaded_from(&self) -> Option<&Path> {
        None
    }

    /// Deserialize from some given path
    fn load<P: AsRef<Path>>(p: P) -> Result<Self>
    where
        Self: Sized + DeserializeOwned,
    {
        let path = p.as_ref();

        let mut item: Self = serde_json::from_str(
            &fs::read_to_string(&path)
                .map_err(CliError::from)
                .context("\n\tpath: ")
                .with_color_context(|| (Color::Yellow, format!("{:?}\n", path)))
                .context("\n(hint: try '")
                .color_context(Color::Green, "seaplane init")
                .context("' if the files are missing)\n")?,
        )?;

        item.set_loaded_from(p);

        Ok(item)
    }
}

// TODO: make the serializer generic
pub trait ToDisk: FromDisk {
    /// Serializes itself to the given path
    fn persist(&self) -> Result<()>
    where
        Self: Sized + Serialize,
    {
        if let Some(path) = self.loaded_from() {
            // TODO: make atomic so that we don't lose or currupt data
            // TODO: long term consider something like SQLite
            serde_json::to_writer(
                File::create(path)
                    .map_err(CliError::from)
                    .context("\n\tpath: ")
                    .with_color_context(|| (Color::Yellow, format!("{:?}\n", path)))?,
                self,
            )
            .map_err(CliError::from)
        } else {
            Err(CliErrorKind::MissingPath.into_err())
        }
    }
}
