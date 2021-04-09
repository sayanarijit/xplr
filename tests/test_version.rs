use xplr::*;

#[test]
fn test_version_compatibility() {
    // Config version == app version
    assert!(app::is_compatible("v0.1.0", "v0.1.0"));
    assert!(app::is_compatible("v1.1.0", "v1.1.0"));

    // Config major version < app major version
    assert!(!app::is_compatible("v0.1.0", "v0.2.0"));
    assert!(!app::is_compatible("v0.2.0", "v0.12.0"));
    assert!(!app::is_compatible("v1.0.0", "v2.0.0"));
    assert!(!app::is_compatible("v2.0.0", "v12.0.0"));

    // Config minor version < app minor version
    assert!(app::is_compatible("v0.0.1", "v0.0.2"));
    assert!(app::is_compatible("v0.0.2", "v0.0.12"));
    assert!(app::is_compatible("v1.1.0", "v1.2.0"));
    assert!(app::is_compatible("v1.2.0", "v1.12.0"));

    // Config patch version < app patch version
    assert!(app::is_compatible("v1.1.1", "v1.1.2"));
    assert!(app::is_compatible("v1.1.2", "v1.1.12"));

    // Config major version > app major version
    assert!(!app::is_compatible("v0.2.0", "v0.1.0"));
    assert!(!app::is_compatible("v0.12.0", "v0.2.0"));
    assert!(!app::is_compatible("v2.0.0", "v1.0.0"));
    assert!(!app::is_compatible("v12.0.0", "v2.0.0"));

    // Config minor version > app minor version
    assert!(!app::is_compatible("v0.0.2", "v0.0.1"));
    assert!(!app::is_compatible("v0.0.12", "v0.0.2"));
    assert!(!app::is_compatible("v1.2.0", "v1.1.0"));
    assert!(!app::is_compatible("v1.12.0", "v1.2.0"));

    // Config patch version > app patch version
    assert!(app::is_compatible("v1.1.2", "v1.1.1"));
    assert!(app::is_compatible("v1.1.12", "v1.1.2"));
}
