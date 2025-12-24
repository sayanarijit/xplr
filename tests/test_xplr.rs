use assert_cmd::{cargo_bin, Command};

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

// TODO fix GitHub CI failures
//
// #[test]
// fn test_cli_path_arg_valid() {
//     Command::new(cargo_bin!())
//         .arg("src")
//         .arg("--on-load")
//         .arg("PrintResultAndQuit")
//         .assert()
//         .success()
//         .code(0)
//         .stderr("");
//
//     Command::new(cargo_bin!())
//         .arg("src")
//         .arg("--on-load")
//         .arg("PrintResultAndQuit")
//         .assert()
//         .success()
//         .code(0)
//         .stderr("");
//
//     Command::new(cargo_bin!())
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
//     Command::new(cargo_bin!())
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
//     Command::new(cargo_bin!())
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
