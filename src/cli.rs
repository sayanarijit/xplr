use anyhow::{bail, Result};
use std::env;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use crate::app;

/// The arguments to pass
#[derive(Debug, Clone, Default)]
pub struct Cli {
    pub version: bool,
    pub help: bool,
    pub read_only: bool,
    pub force_focus: bool,
    pub print_pwd_as_result: bool,
    pub read0: bool,
    pub write0: bool,
    pub config: Option<PathBuf>,
    pub extra_config: Vec<PathBuf>,
    pub on_load: Vec<app::ExternalMsg>,
    pub paths: Vec<PathBuf>,
}

impl Cli {
    fn read_path(&mut self, arg: &str) -> Result<()> {
        if arg.is_empty() {
            bail!("empty string passed")
        };

        let path = PathBuf::from(arg);
        if path.exists() {
            self.paths.push(path);
            Ok(())
        } else {
            bail!("path doesn't exist: {}", path.to_string_lossy().to_string())
        }
    }

    /// Parse arguments from the command-line
    pub fn parse(args: env::Args) -> Result<Self> {
        let mut args = args.skip(1).peekable();
        let mut cli = Self::default();

        let mut flag_ends = false;

        while let Some(arg) = args.next() {
            if flag_ends {
                cli.read_path(&arg)?;
            } else {
                match arg.as_str() {
                    // Flags
                    "-" => {
                        let reader = BufReader::new(std::io::stdin());
                        if cli.read0 {
                            for path in reader.split(b'\0') {
                                cli.read_path(&String::from_utf8(path?)?)?;
                            }
                        } else {
                            for path in reader.lines() {
                                cli.read_path(&path?)?;
                            }
                        };
                    }

                    "-h" | "--help" => {
                        cli.help = true;
                    }

                    "-V" | "--version" => {
                        cli.version = true;
                    }

                    "--read0" => {
                        cli.read0 = true;
                    }

                    "--write0" => {
                        cli.write0 = true;
                    }

                    "-0" | "--null" => {
                        cli.read0 = true;
                        cli.write0 = true;
                    }

                    "--" => {
                        flag_ends = true;
                    }

                    // Options
                    "-c" | "--config" => cli.config = args.next().map(PathBuf::from),

                    "-C" | "--extra-config" => {
                        while let Some(path) =
                            args.next_if(|path| !path.starts_with('-'))
                        {
                            cli.extra_config.push(PathBuf::from(path));
                        }
                    }

                    "--read-only" => cli.read_only = true,

                    "--on-load" => {
                        while let Some(msg) = args.next_if(|msg| !msg.starts_with('-')) {
                            cli.on_load.push(msg.trim().try_into()?);
                        }
                    }

                    "--force-focus" => {
                        cli.force_focus = true;
                    }

                    "--print-pwd-as-result" => {
                        cli.print_pwd_as_result = true;
                    }

                    // path
                    path => {
                        cli.read_path(path)?;
                    }
                }
            }
        }
        Ok(cli)
    }
}
