fn main() {
    match xplr::runner::Runner::new(xplr::cli::Cli::default()).and_then(|a| a.run()) {
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
