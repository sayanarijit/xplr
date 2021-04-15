use std::collections::HashMap;
use tui::style::Color;
use tui::style::Modifier;
use xplr::*;

#[test]
fn test_extend_hashmap() {
    let mut a = HashMap::new();
    let mut b = HashMap::new();

    a.insert("a", "a");
    a.insert("b", "a");

    b.insert("b", "b");
    b.insert("c", "b");

    a.extend(b);

    assert_eq!(a.get("a"), Some(&"a"));
    assert_eq!(a.get("b"), Some(&"b"));
    assert_eq!(a.get("c"), Some(&"b"));
}

#[test]
fn test_extend_ui_config() {
    let a = config::UiConfig {
        prefix: Some("a".to_string()),
        suffix: None,
        style: ui::Style {
            fg: Some(Color::Red),
            bg: None,
            add_modifier: Some(Modifier::BOLD),
            sub_modifier: None,
        },
    };

    let b = config::UiConfig {
        prefix: None,
        suffix: Some("b".to_string()),
        style: ui::Style {
            fg: None,
            bg: Some(Color::Blue),
            add_modifier: None,
            sub_modifier: Some(Modifier::DIM),
        },
    };

    let c = config::UiConfig {
        prefix: Some("cp".to_string()),
        suffix: Some("cs".to_string()),
        style: ui::Style {
            fg: Some(Color::Cyan),
            bg: Some(Color::Magenta),
            add_modifier: Some(Modifier::CROSSED_OUT),
            sub_modifier: Some(Modifier::ITALIC),
        },
    };

    assert_eq!(
        a.clone().extend(b.clone()),
        config::UiConfig {
            prefix: Some("a".to_string()),
            suffix: Some("b".to_string()),
            style: ui::Style {
                fg: Some(Color::Red),
                bg: Some(Color::Blue),
                add_modifier: Some(Modifier::BOLD),
                sub_modifier: Some(Modifier::DIM),
            },
        }
    );

    assert_eq!(
        b.clone().extend(a.clone()),
        config::UiConfig {
            prefix: Some("a".to_string()),
            suffix: Some("b".to_string()),
            style: ui::Style {
                fg: Some(Color::Red),
                bg: Some(Color::Blue),
                add_modifier: Some(Modifier::BOLD),
                sub_modifier: Some(Modifier::DIM),
            },
        }
    );

    assert_eq!(
        a.clone().extend(c.clone()),
        config::UiConfig {
            prefix: Some("cp".to_string()),
            suffix: Some("cs".to_string()),
            style: ui::Style {
                fg: Some(Color::Cyan),
                bg: Some(Color::Magenta),
                add_modifier: Some(Modifier::CROSSED_OUT),
                sub_modifier: Some(Modifier::ITALIC),
            },
        }
    );
}
