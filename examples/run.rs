fn main() {
    let pwd = std::path::PathBuf::from("/");
    match xplr::run(pwd, None) {
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
