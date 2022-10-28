use crate::{app, yaml};
use anyhow::{bail, Context, Result};
use app::ExternalMsg;
use path_absolutize::*;
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
    pub vroot: Option<PathBuf>,
    pub config: Option<PathBuf>,
    pub extra_config: Vec<PathBuf>,
    pub on_load: Vec<app::ExternalMsg>,
    pub pipe_msg_in: Vec<String>,
    pub print_msg_in: Vec<String>,
    pub paths: Vec<PathBuf>,
}

impl Cli {
    fn read_path(arg: &str) -> Result<PathBuf> {
        if arg.is_empty() {
            bail!("empty string passed")
        };

        let path = PathBuf::from(arg).absolutize()?.to_path_buf();
        if path.exists() {
            Ok(path)
        } else {
            bail!("path doesn't exist: {}", path.to_string_lossy())
        }
    }

    /// Parse arguments from the command-line
    pub fn parse(args: env::Args) -> Result<Self> {
        let mut cli = Self::default();
        let mut args = args.peekable();
        cli.bin = args
            .next()
            .map(which::which)
            .context("failed to parse xplr binary path")?
            .context("failed to find xplr binary path")?
            .absolutize()?
            .to_path_buf()
            .to_string_lossy()
            .to_string();

        let mut flag_ends = false;

        while let Some(arg) = args.next() {
            if flag_ends {
                cli.paths.push(Cli::read_path(&arg)?);
            } else {
                match arg.as_str() {
                    // Flags
                    "-" => {
                        let reader = BufReader::new(std::io::stdin());
                        if cli.read0 {
                            for path in reader.split(b'\0') {
                                cli.paths
                                    .push(Cli::read_path(&String::from_utf8(path?)?)?);
                            }
                        } else {
                            for path in reader.lines() {
                                cli.paths.push(Cli::read_path(&path?)?);
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
                    "-c" | "--config" => {
                        cli.config = Some(
                            args.next().map(|a| Cli::read_path(&a)).with_context(
                                || format!("usage: xplr {} PATH", arg),
                            )??,
                        );
                    }

                    "--vroot" => {
                        cli.vroot = Some(
                            args.next().map(|a| Cli::read_path(&a)).with_context(
                                || format!("usage: xplr {} PATH", arg),
                            )??,
                        );
                    }

                    "-C" | "--extra-config" => {
                        while let Some(path) =
                            args.next_if(|path| !path.starts_with('-'))
                        {
                            cli.extra_config.push(Cli::read_path(&path)?);
                        }
                    }

                    "--read-only" => cli.read_only = true,

                    "--on-load" => {
                        while let Some(msg) = args.next_if(|msg| !msg.starts_with('-')) {
                            cli.on_load.push(yaml::from_str(&msg)?);
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
                            bail!("usage: xplr {} FORMAT [ARGUMENT]...", arg)
                        }
                    }

                    "-M" | "--print-msg-in" => {
                        cli.print_msg_in.extend(args.by_ref());
                        if cli.print_msg_in.is_empty() {
                            bail!("usage: xplr {} FORMAT [ARGUMENT]...", arg)
                        }
                    }

                    // path
                    path => {
                        if path.starts_with('-') && !flag_ends {
                            bail!(
                                "invalid argument: {0:?}, try `-- {0:?}` or `--help`",
                                path
                            )
                        } else {
                            cli.paths.push(Cli::read_path(path)?);
                        }
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

    for (col, ch) in format.chars().enumerate() {
        match (ch, last_char) {
            ('%', Some('%')) => {
                msg.push(ch);
                last_char = None;
            }
            ('%', _) => {
                last_char = Some(ch);
            }
            ('q', Some('%')) => {
                let arg = args.next().context(format!(
                    "argument missing for the placeholder at column {}",
                    col
                ))?;
                msg.push_str(&json::to_string(&arg)?);
                last_char = None;
            }
            ('s', Some('%')) => {
                let arg = args.next().context(format!(
                    "argument missing for the placeholder at column {}",
                    col
                ))?;
                msg.push_str(&arg);
                last_char = None;
            }
            (ch, Some('%')) => {
                bail!(format!(
                    "invalid placeholder '%{}' at column {}, use one of '%s' or '%q', or escape it using '%%'",
                    ch, col
                ));
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
        bail!("too many arguments, not enough placeholders")
    }

    // Since we'll mostly by passing json using `-m`, and json is faster than yaml
    // let's try to validate using json first.
    let msg = if let Ok(val) = json::from_str::<ExternalMsg>(&msg) {
        json::to_string(&val)?
    } else {
        let val: ExternalMsg = yaml::from_str(&msg)?;
        json::to_string(&val)?
    };

    Ok(msg)
}
