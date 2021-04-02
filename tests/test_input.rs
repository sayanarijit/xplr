use xplr::*;

#[test]
fn test_key_down() {
    let mut app = app::App::create().expect("failed to create app");

    assert_eq!(app.focus(), Some(0));

    let actions = app.actions_from_key(&input::Key::Down).unwrap();

    for action in actions {
        app = app.handle(&action).unwrap()
    }

    assert_eq!(app.directory_buffer.focus, Some(1));
}
