#![allow(clippy::too_many_arguments)]

use std::env;
use xplr::cli::{self, Cli};
use xplr::runner;

fn main() {
    let cli = Cli::parse(env::args()).unwrap_or_else(|e| {
        eprintln!("error: {e}");
        std::process::exit(1);
    });

    if cli.help {
        let usage = r###"
    xplr [FLAG]... [OPTION]... [PATH] [SELECTION]..."###;

        let flags = r###"
  -                            Reads new-line (\n) separated paths from stdin
  --                           Denotes the end of command-line flags and options
      --force-focus            Focuses on the given <PATH>, even if it is a directory
  -h, --help                   Prints help information
  -m, --pipe-msg-in            Helps safely passing messages to the active xplr
                                 session, use %%, %s and %q as the placeholders
  -M, --print-msg-in           Like --pipe-msg-in, but prints the message instead of
                                 passing to the active xplr session
      --print-pwd-as-result    Prints the present working directory when quitting
                                 with `PrintResultAndQuit`
      --read-only              Enables read-only mode (config.general.read_only)
      --read0                  Reads paths separated using the null character (\0)
      --write0                 Prints paths separated using the null character (\0)
  -0  --null                   Combines --read0 and --write0
  -V, --version                Prints version information"###;

        let options = r###"
  -c, --config <PATH>             Specifies a custom config file (default is
                                    "$HOME/.config/xplr/init.lua")
  -C, --extra-config <PATH>...    Specifies extra config files to load
      --on-load <MESSAGE>...      Sends messages when xplr loads
      --vroot <PATH>              Treats the specified path as the virtual root"###;

        let args = r###"
  <PATH>            Path to focus on, or enter if directory, (default is `.`)
  <SELECTION>...    Paths to select, requires <PATH> to be set explicitly"###;

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

        println!("{help}");
    } else if cli.version {
        println!("xplr {}", xplr::app::VERSION);
    } else if !cli.pipe_msg_in.is_empty() {
        if let Err(err) = cli::pipe_msg_in(cli.pipe_msg_in) {
            eprintln!("error: {err}");
            std::process::exit(1);
        }
    } else if !cli.print_msg_in.is_empty() {
        if let Err(err) = cli::print_msg_in(cli.print_msg_in) {
            eprintln!("error: {err}");
            std::process::exit(1);
        }
    } else {
        match runner::from_cli(cli).and_then(|a| a.run()) {
            Ok(Some(out)) => {
                print!("{out}");
            }
            Ok(None) => {}
            Err(err) => {
                if !err.to_string().is_empty() {
                    eprintln!("error: {err}");
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
    fn test_no_debug_in_lib() {
        // for pat in ["print!", "println!"].iter() {
        //     Command::new("grep")
        //         .args(&[
        //             "-R",
        //             pat,
        //             "src",
        //             "--exclude-dir",
        //             "bin/",
        //             "--exclude-dir",
        //             "rustc/",
        //         ])
        //         .assert()
        //         .failure();
        // }
        //
        // TODO: fix github macos runner
    }

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

    // TODO fix GitHub CI failures
    //
    // #[test]
    // fn test_cli_path_arg_valid() {
    //     Command::cargo_bin("xplr")
    //         .unwrap()
    //         .arg("src")
    //         .arg("--on-load")
    //         .arg("PrintResultAndQuit")
    //         .assert()
    //         .success()
    //         .code(0)
    //         .stderr("");
    //
    //     Command::cargo_bin("xplr")
    //         .unwrap()
    //         .arg("src")
    //         .arg("--on-load")
    //         .arg("PrintResultAndQuit")
    //         .assert()
    //         .success()
    //         .code(0)
    //         .stderr("");
    //
    //     Command::cargo_bin("xplr")
    //         .unwrap()
    //         .arg("--on-load")
    //         .arg("PrintResultAndQuit")
    //         .arg("--")
    //         .arg("src")
    //         .assert()
    //         .success()
    //         .code(0)
    //         .stderr("");
    // }
    //
    // #[test]
    // fn test_cli_path_stdin_valid() {
    //     Command::cargo_bin("xplr")
    //         .unwrap()
    //         .arg("-")
    //         .arg("--on-load")
    //         .arg("PrintResultAndQuit")
    //         .write_stdin("src\n")
    //         .assert()
    //         .success()
    //         .code(0)
    //         .stderr("");
    // }
    //
    // #[test]
    // fn test_on_load_yaml_parsing() {
    //     Command::cargo_bin("xplr")
    //         .unwrap()
    //         .arg("--on-load")
    //         .arg("Call: {command: touch, args: [foo]}")
    //         .arg("Quit")
    //         .assert()
    //         .success()
    //         .code(0)
    //         .stderr("");
    //
    //     std::fs::remove_file("foo").unwrap();
    // }
}
