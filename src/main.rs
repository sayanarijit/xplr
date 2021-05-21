#![allow(clippy::too_many_arguments)]

use std::env;
use std::path::PathBuf;
use xplr::app;
use xplr::runner;

fn main() {
    let mut pwd = PathBuf::from(env::args().nth(1).unwrap_or_else(|| ".".into()))
        .canonicalize()
        .unwrap_or_default();
    let mut focused_path = None;

    if pwd.is_file() {
        focused_path = Some(
            pwd.file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string(),
        );
        pwd = pwd.parent().map(|p| p.into()).unwrap_or_default();
    }

    let lua = mlua::Lua::new();

    let app = app::App::create(pwd, &lua).unwrap_or_else(|e| {
        eprintln!("error: {}", e);
        std::process::exit(1);
    });

    match runner::run(app, focused_path, lua) {
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
