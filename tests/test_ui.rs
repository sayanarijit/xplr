use tui::style::Color;
use tui::style::Modifier;
use xplr::*;

#[test]
fn test_extend_style() {
    let a = ui::Style {
        fg: Some(Color::Red),
        bg: None,
        add_modifier: Some(Modifier::BOLD),
        sub_modifier: None,
    };

    let b = ui::Style {
        fg: None,
        bg: Some(Color::Blue),
        add_modifier: None,
        sub_modifier: Some(Modifier::DIM),
    };

    let c = ui::Style {
        fg: Some(Color::Cyan),
        bg: Some(Color::Magenta),
        add_modifier: Some(Modifier::CROSSED_OUT),
        sub_modifier: Some(Modifier::ITALIC),
    };

    assert_eq!(
        a.clone().extend(b.clone()),
        ui::Style {
            fg: Some(Color::Red),
            bg: Some(Color::Blue),
            add_modifier: Some(Modifier::BOLD),
            sub_modifier: Some(Modifier::DIM),
        }
    );

    assert_eq!(
        b.clone().extend(a.clone()),
        ui::Style {
            fg: Some(Color::Red),
            bg: Some(Color::Blue),
            add_modifier: Some(Modifier::BOLD),
            sub_modifier: Some(Modifier::DIM),
        }
    );

    assert_eq!(
        a.clone().extend(c.clone()),
        ui::Style {
            fg: Some(Color::Cyan),
            bg: Some(Color::Magenta),
            add_modifier: Some(Modifier::CROSSED_OUT),
            sub_modifier: Some(Modifier::ITALIC),
        }
    );

    assert_eq!(
        c.clone().extend(a.clone()),
        ui::Style {
            fg: Some(Color::Red),
            bg: Some(Color::Magenta),
            add_modifier: Some(Modifier::BOLD),
            sub_modifier: Some(Modifier::ITALIC),
        }
    );
}
