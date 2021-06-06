#![allow(clippy::too_many_arguments)]

use anyhow::Result;
use std::collections::VecDeque;
use std::env;
use std::io;
use std::path::PathBuf;
use xplr::app;

#[derive(Debug, Clone, Default)]
struct Cli {
    version: bool,
    help: bool,
    read_only: bool,
    path: Option<PathBuf>,
    config: Option<PathBuf>,
    on_load: Vec<app::ExternalMsg>,
}

impl Cli {
    fn parse(args: env::Args) -> Result<Self> {
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

fn main() {
    let cli = Cli::parse(env::args()).unwrap_or_else(|e| {
        println!("error: {}", e);
        std::process::exit(1);
    });

    if cli.help {
        let usage = r###"
    xplr [FLAG]... [OPTION]... [PATH]"###;

        let flags = r###"
    -                  Reads path from stdin
    --                 Denotes the end of command-line flags and options
        --read-only    Enables read-only mode (config.general.read_only)
    -h, --help         Prints help information
    -V, --version      Prints version information"###;

        let options = r###"
    -c, --config <PATH>           Specifies a custom config file (default is
                                    "$HOME/.config/xplr/init.lua")
        --on-load <MESSAGE>...    Sends messages when xplr loads"###;

        let args = r###"
    <PATH>    Path to focus on, or enter if directory"###;

        let help = format!(
            "xplr {}\n{}\n{}\n\nUSAGE:{}\n\nFLAGS:{}\n\nOPTIONS:{}\n\nARGS:{}",
            xplr::app::VERSION,
            env!("CARGO_PKG_AUTHORS"),
            env!("CARGO_PKG_DESCRIPTION"),
            usage,
            flags,
            options,
            args,
        );
        let help = help.trim();

        println!("{}", help);
    } else if cli.version {
        println!("xplr {}", xplr::app::VERSION);
    } else {
        match app::runner(cli.path.clone())
            .map(|a| a.with_on_load(cli.on_load.clone()))
            .map(|a| a.with_config(cli.config.clone()))
            .map(|a| a.with_read_only(cli.read_only))
            .and_then(|a| a.run())
        {
            Ok(Some(out)) => print!("{}", out),
            Ok(None) => {}
            Err(err) => {
                if !err.to_string().is_empty() {
                    eprintln!("error: {}", err);
                };

                std::process::exit(1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use assert_cmd::Command;

    #[test]
    fn test_cli_version() {
        Command::cargo_bin("xplr")
            .unwrap()
            .arg("--version")
            .assert()
            .success()
            .code(0)
            .stdout(format!("xplr {}\n", xplr::app::VERSION))
            .stderr("");

        Command::cargo_bin("xplr")
            .unwrap()
            .arg("-V")
            .assert()
            .success()
            .code(0)
            .stdout(format!("xplr {}\n", xplr::app::VERSION))
            .stderr("");
    }

    #[test]
    fn test_cli_help() {
        Command::cargo_bin("xplr")
            .unwrap()
            .arg("-h")
            .assert()
            .success()
            .code(0)
            .stderr("");

        Command::cargo_bin("xplr")
            .unwrap()
            .arg("--help")
            .assert()
            .success()
            .code(0)
            .stderr("");
    }

    // TODO: Fix running GitHub action
    //
    // #[test]
    // fn test_cli_path_arg_valid() {
    //     Command::cargo_bin("xplr")
    //         .unwrap()
    //         .arg("/tmp")
    //         .arg("--on-load")
    //         .arg("PrintResultAndQuit")
    //         .assert()
    //         .success()
    //         .code(0)
    //         .stderr("");

    //     Command::cargo_bin("xplr")
    //         .unwrap()
    //         .arg("/tmp")
    //         .arg("--on-load")
    //         .arg("PrintResultAndQuit")
    //         .assert()
    //         .success()
    //         .code(0)
    //         .stderr("");

    //     Command::cargo_bin("xplr")
    //         .unwrap()
    //         .arg("--on-load")
    //         .arg("PrintResultAndQuit")
    //         .arg("--")
    //         .arg("/tmp")
    //         .assert()
    //         .success()
    //         .code(0)
    //         .stderr("");
    // }

    // #[test]
    // fn test_cli_path_stdin_valid() {
    //     Command::cargo_bin("xplr")
    //         .unwrap()
    //         .arg("-")
    //         .arg("--on-load")
    //         .arg("PrintResultAndQuit")
    //         .write_stdin("/tmp\n")
    //         .assert()
    //         .success()
    //         .code(0)
    //         .stderr("");
    // }
}
