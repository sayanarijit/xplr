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

// In unix system, the std::env::current_dir() calls libc getcwd() that
// returns physical path. As a workaround, this function tries to use `PWD`
// environment variable that is configured by shell.
pub fn get_current_dir() -> Result<PathBuf, std::io::Error> {
    if let Ok(pwd) = std::env::var("PWD") {
        if pwd.is_empty() {
            std::env::current_dir()
        } else {
            Ok(PathBuf::from(pwd))
        }
    } else {
        std::env::current_dir()
    }
}
