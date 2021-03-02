use crate::input::Key;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tui::layout::Constraint as TUIConstraint;
use tui::style::Color;
use tui::style::Modifier;
use tui::style::Style;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Mode {
    Explore,
    ExploreSubmode(String),
    Select,
    SelectSubmode(String),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Format {
    Line,
    Pretty,
    Yaml,
    YamlPretty,
    Template(String),
}

impl Default for Format {
    fn default() -> Self {
        Self::Line
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GlobalAction {

    // Common actions
    ToggleShowHidden,
    Back,
    Enter,
    FocusPrevious,
    FocusNext,
    FocusFirst,
    FocusLast,
    FocusPath(String),
    FocusPathByIndex(usize),
    FocusPathByBufferRelativeIndex(usize),
    FocusPathByFocusRelativeIndex(isize),
    ChangeDirectory(String),

    // Quit options
    PrintPwdAndQuit,
    PrintAppStateAndQuit,
    Quit,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ExploreModeAction {

    // Common actions
    ToggleShowHidden,
    Back,
    Enter,
    FocusPrevious,
    FocusNext,
    FocusFirst,
    FocusLast,
    FocusPathByIndex(usize),
    FocusPathByBufferRelativeIndex(usize),
    FocusPathByFocusRelativeIndex(isize),
    FocusPath(String),
    ChangeDirectory(String),

    // Explore mode exclusive options
    EnterSubmode(String),
    ExitSubmode,
    Select,
    // Unselect,
    // SelectAll,
    // SelectAllRecursive,

    // Quit options
    PrintFocusedAndQuit,
    PrintPwdAndQuit,
    PrintAppStateAndQuit,
    Quit,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SelectModeAction {

    // Common actions
    ToggleShowHidden,
    Back,
    Enter,
    FocusPrevious,
    FocusNext,
    FocusFirst,
    FocusLast,
    FocusPathByIndex(usize),
    FocusPathByBufferRelativeIndex(usize),
    FocusPathByFocusRelativeIndex(isize),
    FocusPath(String),
    ChangeDirectory(String),

    // Select mode exclusive options
    EnterSubmode(String),
    ExitSubmode,
    // Select,
    // Unselect,
    // SelectAll,
    // SelectAllRecursive,
    // UnselectAll,
    // UnSelectAllRecursive,
    ToggleSelection,
    // ClearSelectedPaths,

    // Quit options
    PrintSelectedAndQuit,
    PrintAppStateAndQuit,
    Quit,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    Global(GlobalAction),
    ExploreMode(ExploreModeAction),
    SelectMode(SelectModeAction),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GlobalActionMenu {
    #[serde(default)]
    pub help: String,
    pub actions: Vec<GlobalAction>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExploreModeActionMenu {
    #[serde(default)]
    pub help: String,
    pub actions: Vec<ExploreModeAction>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SelectModeActionMenu {
    #[serde(default)]
    pub help: String,
    pub actions: Vec<SelectModeAction>,
}

pub type ExploreSubmodeActionMenu = HashMap<Key, ExploreModeActionMenu>;
pub type SelectSubmodeActionMenu = HashMap<Key, SelectModeActionMenu>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyBindings {
    pub global: HashMap<Key, GlobalActionMenu>,
    #[serde(default)]
    pub explore_mode: HashMap<Key, ExploreModeActionMenu>,
    #[serde(default)]
    pub explore_submodes: HashMap<String, ExploreSubmodeActionMenu>,
    #[serde(default)]
    pub select_mode: HashMap<Key, SelectModeActionMenu>,
    #[serde(default)]
    pub select_submodes: HashMap<String, SelectSubmodeActionMenu>,
}

impl Default for KeyBindings {
    fn default() -> Self {
        let mut global: HashMap<Key, GlobalActionMenu> = Default::default();
        let mut explore_mode: HashMap<Key, ExploreModeActionMenu> = Default::default();
        let explore_submodes: HashMap<String, ExploreSubmodeActionMenu> = Default::default();
        let mut select_mode: HashMap<Key, SelectModeActionMenu> = Default::default();
        let select_submodes: HashMap<String, SelectSubmodeActionMenu> = Default::default();

        global.insert(
            Key::CtrlC,
            GlobalActionMenu {
                help: "quit".into(),
                actions: vec![GlobalAction::Quit],
            },
        );

        global.insert(
            Key::Q,
            GlobalActionMenu {
                help: "quit".into(),
                actions: vec![GlobalAction::Quit],
            },
        );

        global.insert(
            Key::Escape,
            GlobalActionMenu {
                help: "quit".into(),
                actions: vec![GlobalAction::Quit],
            },
        );

        global.insert(
            Key::Left,
            GlobalActionMenu {
                help: "left".into(),
                actions: vec![GlobalAction::Back],
            },
        );

        global.insert(
            Key::Dot,
            GlobalActionMenu {
                help: "show/hide hidden files".into(),
                actions: vec![GlobalAction::ToggleShowHidden],
            },
        );

        global.insert(
            Key::Right,
            GlobalActionMenu {
                help: "enter".into(),
                actions: vec![GlobalAction::Enter],
            },
        );

        global.insert(
            Key::Up,
            GlobalActionMenu {
                help: "<prev".into(),
                actions: vec![GlobalAction::FocusPrevious],
            },
        );

        global.insert(
            Key::Down,
            GlobalActionMenu {
                help: "next>".into(),
                actions: vec![GlobalAction::FocusNext],
            },
        );

        global.insert(
            Key::G,
            GlobalActionMenu {
                help: "top".into(),
                actions: vec![GlobalAction::FocusFirst],
            },
        );

        global.insert(
            Key::ShiftG,
            GlobalActionMenu {
                help: "bottom".into(),
                actions: vec![GlobalAction::FocusLast],
            },
        );

        global.insert(
            Key::Tilde,
            GlobalActionMenu {
                help: "home".into(),
                actions: vec![GlobalAction::ChangeDirectory("~".to_string())],
            },
        );

        explore_mode.insert(
            Key::Space,
            ExploreModeActionMenu {
                help: "select".into(),
                actions: vec![ExploreModeAction::Select, ExploreModeAction::FocusNext],
            },
        );

        select_mode.insert(
            Key::Space,
            SelectModeActionMenu {
                help: "select/unselect".into(),
                actions: vec![
                    SelectModeAction::ToggleSelection,
                    SelectModeAction::FocusNext,
                ],
            },
        );

        Self {
            global,
            explore_mode,
            explore_submodes,
            select_mode,
            select_submodes,
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FileTypeConfig {
    #[serde(default)]
    pub icon: String,
    #[serde(default)]
    pub style: Style,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileTypesConfig {
    #[serde(default)]
    pub directory: FileTypeConfig,
    #[serde(default)]
    pub file: FileTypeConfig,
    #[serde(default)]
    pub symlink: FileTypeConfig,
    #[serde(default)]
    pub extension: HashMap<String, FileTypeConfig>,
    #[serde(default)]
    pub special: HashMap<String, FileTypeConfig>,
}

impl Default for FileTypesConfig {
    fn default() -> Self {
        FileTypesConfig {
            directory: FileTypeConfig {
                icon: "d".into(),
                style: Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(Color::Blue),
            },

            file: FileTypeConfig {
                icon: "f".into(),
                style: Default::default(),
            },

            symlink: FileTypeConfig {
                icon: "s".into(),
                style: Style::default()
                    .add_modifier(Modifier::ITALIC)
                    .fg(Color::Cyan),
            },

            extension: Default::default(),
            special: Default::default(),
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UIConfig {
    #[serde(default)]
    pub prefix: String,
    #[serde(default)]
    pub suffix: String,
    #[serde(default)]
    pub style: Style,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UIElement {
    #[serde(default)]
    pub format: String,
    #[serde(default)]
    pub style: Style,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TableRowConfig {
    #[serde(default)]
    pub cols: Vec<UIElement>,
    #[serde(default)]
    pub style: Style,
    #[serde(default)]
    pub height: u16,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Constraint {
    Percentage(u16),
    Ratio(u32, u32),
    Length(u16),
    Max(u16),
    Min(u16),
}

impl Default for Constraint {
    fn default() -> Self {
        Self::Min(1)
    }
}

impl Into<TUIConstraint> for Constraint {
    fn into(self) -> TUIConstraint {
        match self {
            Self::Length(n) => TUIConstraint::Length(n),
            Self::Percentage(n) => TUIConstraint::Percentage(n),
            Self::Ratio(x, y) => TUIConstraint::Ratio(x, y),
            Self::Max(n) => TUIConstraint::Max(n),
            Self::Min(n) => TUIConstraint::Min(n),
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TableConfig {
    #[serde(default)]
    pub header: Option<TableRowConfig>,
    #[serde(default)]
    pub row: TableRowConfig,
    #[serde(default)]
    pub style: Style,
    #[serde(default)]
    pub tree: Option<(UIElement, UIElement, UIElement)>,
    #[serde(default)]
    pub col_spacing: u16,
    #[serde(default)]
    pub col_widths: Vec<Constraint>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralConfig {
    #[serde(default)]
    pub show_hidden: bool,
    #[serde(default)]
    pub table: TableConfig,
    #[serde(default)]
    pub normal_ui: UIConfig,
    #[serde(default)]
    pub focused_ui: UIConfig,
    #[serde(default)]
    pub selected_ui: UIConfig,
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            show_hidden: false,
            table: TableConfig {
                header: None,
                row: TableRowConfig {
                    cols: vec![UIElement {
                        format: "{{tree}}{{prefix}}{{relativePath}}{{#if isDir}}/{{/if}}{{suffix}}"
                            .into(),
                        style: Default::default(),
                    }],
                    style: Default::default(),
                    height: 1,
                },

                style: Default::default(),
                tree: Some((
                    UIElement {
                        format: "├─".into(),
                        style: Default::default(),
                    },
                    UIElement {
                        format: "├─".into(),
                        style: Default::default(),
                    },
                    UIElement {
                        format: "└─".into(),
                        style: Default::default(),
                    },
                )),
                col_spacing: 1,
                col_widths: vec![Constraint::Percentage(100)],
            },
            normal_ui: UIConfig {
                prefix: "   ".into(),
                suffix: " ".into(),
                style: Default::default(),
            },

            focused_ui: UIConfig {
                prefix: "▸ [".into(),
                suffix: "]".into(),
                style: Style::default().add_modifier(Modifier::BOLD),
            },

            selected_ui: UIConfig {
                prefix: "  {".into(),
                suffix: "}".into(),
                style: Style::default().add_modifier(Modifier::BOLD),
            },
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub general: GeneralConfig,

    #[serde(default)]
    pub filetypes: FileTypesConfig,

    #[serde(default)]
    pub key_bindings: KeyBindings,
}
