use std::{env, path::PathBuf};

use lazy_static::lazy_static;
use xdg::BaseDirectories;

lazy_static! {
    pub static ref BASE_DIRS: BaseDirectories = BaseDirectories::new();
}

pub fn home_dir() -> Option<PathBuf> {
    home::home_dir()
}

pub fn config_dir() -> Option<PathBuf> {
    BASE_DIRS.get_config_home()
}

pub fn runtime_dir() -> PathBuf {
    let Some(dir) = BASE_DIRS.get_runtime_directory().ok() else {
        return env::temp_dir();
    };
    dir.clone()
}
