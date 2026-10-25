use anyhow::Result;
use std::fs;
use std::io::Write;

pub fn start(path: &str, focus_path: &str) -> Result<fs::File> {
    match fs::OpenOptions::new().write(true).open(path) {
        Ok(mut file) => {
            writeln!(file, "{focus_path}")?;
            Ok(file)
        }
        Err(e) => Err(e.into()),
    }
}
