use xplr::*;

#[test]
fn test_version_incompatibility() {
    assert!(app::is_compatible("v0.1.0", "v0.1.2"));
    assert!(app::is_compatible("v0.2.0", "v0.2.2"));
    assert!(!app::is_compatible("v0.1.0", "v0.2.0"));
    assert!(!app::is_compatible("v0.1.0", "v1.1.0"));
    assert!(app::is_compatible("v1.1.0", "v1.1.0"));
    assert!(app::is_compatible("v1.1.0", "v1.1.1"));
    assert!(app::is_compatible("v1.1.0", "v1.2.1"));
    assert!(app::is_compatible("v1.1.0", "v1.2.1"));
    assert!(!app::is_compatible("v1.1.0", "v2.0.0"));
}
