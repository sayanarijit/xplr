use crate::app;
use anyhow::{bail, Context, Result};
use app::ExternalMsg;
use serde_json as json;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;
use std::{env, fs};

/// The arguments to pass
#[derive(Debug, Clone, Default)]
pub struct Cli {
    pub bin: String,
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
    pub pipe_msg_in: Vec<String>,
    pub print_msg_in: Vec<String>,
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
        let mut cli = Self::default();
        let mut args = args.peekable();
        cli.bin = args.next().context("failed to parse xplr binary path")?;

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

                    "-m" | "--pipe-msg-in" => {
                        cli.pipe_msg_in.extend(args.by_ref());
                        if cli.pipe_msg_in.is_empty() {
                            bail!("usage: {} {} FORMAT [ARGUMENT]...", cli.bin, arg)
                        }
                    }

                    "-M" | "--print-msg-in" => {
                        cli.print_msg_in.extend(args.by_ref());
                        if cli.print_msg_in.is_empty() {
                            bail!("usage: {} {} FORMAT [ARGUMENT]...", cli.bin, arg)
                        }
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

pub fn pipe_msg_in(args: Vec<String>) -> Result<()> {
    let mut msg = fmt_msg_in(args)?;

    if let Ok(path) = std::env::var("XPLR_PIPE_MSG_IN") {
        let delimiter = fs::read(&path)?
            .first()
            .cloned()
            .context("failed to detect delimmiter")?;

        msg.push(delimiter.try_into()?);
        File::options()
            .append(true)
            .open(&path)?
            .write_all(msg.as_bytes())?;
    } else {
        println!("{}", msg);
    };

    Ok(())
}

pub fn print_msg_in(args: Vec<String>) -> Result<()> {
    let msg = fmt_msg_in(args)?;
    print!("{}", msg);
    Ok(())
}

fn fmt_msg_in(args: Vec<String>) -> Result<String> {
    let mut args = args.into_iter();
    let format = args.next().context("FORMAT is missing")?;
    let mut msg = "".to_string();
    let mut last_char = None;

    for ch in format.chars() {
        match (ch, last_char) {
            ('%', Some('%')) => {
                msg.push(ch);
                last_char = None;
            }
            ('%', _) => {
                last_char = Some(ch);
            }
            ('q', Some('%')) => {
                let arg = args.next().context("not enough arguments")?;
                msg.push_str(&json::to_string(&arg)?);
                last_char = None;
            }
            ('s', Some('%')) => {
                let arg = args.next().context("not enough arguments")?;
                msg.push_str(&arg);
                last_char = None;
            }
            (ch, Some('%')) => {
                bail!(format!("invalid placeholder: %{}, use %s, %q or %%", ch));
            }
            (ch, _) => {
                msg.push(ch);
                last_char = Some(ch);
            }
        }
    }

    if last_char == Some('%') {
        bail!("message ended with incomplete placeholder");
    }

    if args.count() != 0 {
        bail!("too many arguments")
    }

    // Validate
    let msg = json::to_string(&ExternalMsg::try_from(msg.as_str())?)?;
    Ok(msg)
}
