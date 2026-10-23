use std::{env, path::PathBuf, sync::LazyLock};

use xdg::BaseDirectories;

pub static BASE_DIRS: LazyLock<BaseDirectories> = LazyLock::new(BaseDirectories::new);

pub fn home_dir() -> Option<PathBuf> {
    env::home_dir()
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
