use assert_cmd::{cargo_bin, Command};

#[test]
fn test_cli_version() {
    Command::new(cargo_bin!())
        .arg("--version")
        .assert()
        .success()
        .code(0)
        .stdout(format!("xplr {}\n", xplr::app::VERSION))
        .stderr("");

    Command::new(cargo_bin!())
        .arg("-V")
        .assert()
        .success()
        .code(0)
        .stdout(format!("xplr {}\n", xplr::app::VERSION))
        .stderr("");
}

#[test]
fn test_cli_help() {
    Command::new(cargo_bin!())
        .arg("-h")
        .assert()
        .success()
        .code(0)
        .stderr("");

    Command::new(cargo_bin!())
        .arg("--help")
        .assert()
        .success()
        .code(0)
        .stderr("");
}
