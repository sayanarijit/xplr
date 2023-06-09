#![allow(clippy::too_many_arguments)]
#![allow(clippy::from_over_into)]
#![allow(clippy::unnecessary_wraps)]

pub mod app;
pub mod cli;
pub mod compat;
pub mod config;
pub mod directory_buffer;
pub mod dirs;
pub mod event_reader;
pub mod explorer;
pub mod input;
pub mod lua;
pub mod msg;
pub mod node;
pub mod path;
pub mod permissions;
pub mod pipe;
pub mod pwd_watcher;
pub mod runner;
pub mod search;
pub mod ui;
pub mod yaml;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_upgrade_guide_has_latest_version() {
        let guide = include_str!("../docs/en/src/upgrade-guide.md");
        assert!(guide.contains(app::VERSION));
    }
}
