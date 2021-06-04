#![allow(clippy::too_many_arguments)]

use std::env;
use std::path::PathBuf;
use xplr::app;

fn main() {
    let mut pwd = PathBuf::from(env::args().nth(1).unwrap_or_else(|| ".".into()))
        .canonicalize()
        .unwrap_or_default();
    let mut focused_path = None;

    if pwd.is_file() {
        focused_path = pwd.file_name().map(|p| p.into());
        pwd = pwd.parent().map(|p| p.into()).unwrap_or_else(|| ".".into());
    }

    match app::run(pwd, focused_path) {
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
