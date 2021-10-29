use anyhow::Result;
use std::collections::VecDeque;
use std::path::PathBuf;
use std::{env, io};

use crate::app;

/// The arguments to pass
#[derive(Debug, Clone, Default)]
pub struct Cli {
    pub version: bool,
    pub help: bool,
    pub read_only: bool,
    pub path: Option<PathBuf>,
    pub config: Option<PathBuf>,
    pub extra_config: Vec<PathBuf>,
    pub on_load: Vec<app::ExternalMsg>,
    pub select: Vec<PathBuf>,
}

impl Cli {
    /// Parse arguments from the command-line
    pub fn parse(args: env::Args) -> Result<Self> {
        let mut args: VecDeque<String> = args.skip(1).collect();
        let mut cli = Self::default();

        while let Some(arg) = args.pop_front() {
            match arg.as_str() {
                // Flags
                "-" => {
                    let mut path = String::new();
                    if io::stdin().read_line(&mut path).is_ok() {
                        cli.path =
                            Some(path.trim_end_matches("\r\n").trim_end_matches('\n').into());
                    };
                }

                "-h" | "--help" => {
                    cli.help = true;
                }

                "-V" | "--version" => {
                    cli.version = true;
                }

                "--" => {
                    if cli.path.is_none() {
                        cli.path = args.pop_front().map(PathBuf::from);
                    }
                    return Ok(cli);
                }

                // Options
                "-c" | "--config" => cli.config = args.pop_front().map(PathBuf::from),

                "-C" | "--extra-config" => {
                    while let Some(path) = args.pop_front() {
                        if path.starts_with('-') {
                            args.push_front(path);
                            break;
                        } else {
                            cli.extra_config.push(PathBuf::from(path));
                        }
                    }
                }

                "--read-only" => cli.read_only = true,

                "--on-load" => {
                    while let Some(msg) = args.pop_front() {
                        if msg.starts_with('-') {
                            args.push_front(msg);
                            break;
                        } else {
                            cli.on_load.push(serde_yaml::from_str(&msg)?);
                        }
                    }
                }
                "--select" => {
                    while let Some(path) = args.pop_front() {
                        if path.starts_with('-') && path != "-" {
                            args.push_front(path);
                            break;
                        } else {
                            cli.select.push(PathBuf::from(path));
                        }
                    }
                }

                "--force-focus" => {
                    let path = cli
                        .path
                        .as_ref()
                        .map(|x| x.clone())
                        .or(args.pop_front().map(PathBuf::from))
                        .unwrap_or(PathBuf::new());
                    cli.on_load.push(app::ExternalMsg::FocusPath(
                        path.to_string_lossy().to_string(),
                    ));
                }

                // path
                path => {
                    if cli.path.is_none() {
                        cli.path = Some(path.into());
                    }
                }
            }
        }
        Ok(cli)
    }
}
