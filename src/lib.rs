#![allow(clippy::too_many_arguments)]
#![allow(clippy::from_over_into)]
#![allow(clippy::unnecessary_wraps)]

pub mod app;
pub mod auto_refresher;
pub mod config;
pub mod event_reader;
pub mod explorer;
pub mod input;
pub mod lua;
pub mod permissions;
pub mod pipe_reader;
pub mod pwd_watcher;
pub mod runner;
pub mod ui;

pub use app::runner;
