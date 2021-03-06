use crate::app::VERSION;
use crate::input::Key;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
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

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Explore => {
                write!(f, "explore")
            }

            Self::Select => {
                write!(f, "select")
            }

            Self::ExploreSubmode(s) => {
                write!(f, "explore({})", &s)
            }

            Self::SelectSubmode(s) => {
                write!(f, "select({})", &s)
            }
        }
    }
}

impl Mode {
    pub fn does_support(self, action: &Action) -> bool {
        match (self, action) {
            // Special
            (_, Action::Terminate) => true,

            // Explore mode
            (Self::Explore, Action::Back) => true,
            (Self::Explore, Action::Call(_)) => true,
            (Self::Explore, Action::ChangeDirectory(_)) => true,
            (Self::Explore, Action::Enter) => true,
            (Self::Explore, Action::EnterSubmode(_)) => true,
            (Self::Explore, Action::ExitSubmode) => false,
            (Self::Explore, Action::FocusFirst) => true,
            (Self::Explore, Action::FocusLast) => true,
            (Self::Explore, Action::FocusNext) => true,
            (Self::Explore, Action::FocusPath(_)) => true,
            (Self::Explore, Action::FocusPathByBufferRelativeIndex(_)) => true,
            (Self::Explore, Action::FocusPathByFocusRelativeIndex(_)) => true,
            (Self::Explore, Action::FocusPathByIndex(_)) => true,
            (Self::Explore, Action::FocusPrevious) => true,
            (Self::Explore, Action::PrintAppState) => true,
            (Self::Explore, Action::PrintFocused) => true,
            (Self::Explore, Action::PrintSelected) => false,
            (Self::Explore, Action::Quit) => true,
            (Self::Explore, Action::Select) => true,
            (Self::Explore, Action::ToggleSelection) => false,
            (Self::Explore, Action::ToggleShowHidden) => true,

            // Explore submode
            (Self::ExploreSubmode(_), Action::ExitSubmode) => true,
            (Self::ExploreSubmode(_), a) => Self::does_support(Self::Explore, a),

            // Select mode
            (Self::Select, Action::Back) => true,
            (Self::Select, Action::Call(_)) => true,
            (Self::Select, Action::ChangeDirectory(_)) => true,
            (Self::Select, Action::Enter) => true,
            (Self::Select, Action::EnterSubmode(_)) => true,
            (Self::Select, Action::ExitSubmode) => true,
            (Self::Select, Action::FocusFirst) => true,
            (Self::Select, Action::FocusLast) => true,
            (Self::Select, Action::FocusNext) => true,
            (Self::Select, Action::FocusPath(_)) => true,
            (Self::Select, Action::FocusPathByBufferRelativeIndex(_)) => true,
            (Self::Select, Action::FocusPathByFocusRelativeIndex(_)) => true,
            (Self::Select, Action::FocusPathByIndex(_)) => true,
            (Self::Select, Action::FocusPrevious) => true,
            (Self::Select, Action::PrintAppState) => true,
            (Self::Select, Action::PrintFocused) => false,
            (Self::Select, Action::PrintSelected) => true,
            (Self::Select, Action::Quit) => true,
            (Self::Select, Action::Select) => false,
            (Self::Select, Action::ToggleSelection) => true,
            (Self::Select, Action::ToggleShowHidden) => true,

            // Select submode
            (Self::SelectSubmode(_), Action::ExitSubmode) => true,
            (Self::SelectSubmode(_), a) => Self::does_support(Self::Select, a),
        }
    }
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
pub struct CommandConfig {
    pub command: String,

    #[serde(default)]
    pub args: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Action {
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
    Call(CommandConfig),
    EnterSubmode(String),
    ExitSubmode,
    Select,
    // Unselect,
    // SelectAll,
    // SelectAllRecursive,
    // UnselectAll,
    // UnSelectAllRecursive,
    ToggleSelection,
    // ClearSelectedPaths,

    // Quit options
    PrintFocused,
    PrintSelected,
    PrintAppState,
    Quit,
    Terminate,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ActionMenu {
    #[serde(default)]
    pub help: String,
    pub actions: Vec<Action>,
}

pub type SubmodeActionMenu = HashMap<Key, ActionMenu>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyBindings {
    pub global: HashMap<Key, ActionMenu>,
    #[serde(default)]
    pub explore_mode: HashMap<Key, ActionMenu>,
    #[serde(default)]
    pub explore_submodes: HashMap<String, SubmodeActionMenu>,
    #[serde(default)]
    pub select_mode: HashMap<Key, ActionMenu>,
    #[serde(default)]
    pub select_submodes: HashMap<String, SubmodeActionMenu>,
}

impl KeyBindings {
    pub fn filtered(&self, mode: &Mode) -> HashMap<Key, (String, Vec<Action>)> {
        let mode_bindings: Option<HashMap<Key, ActionMenu>> = match mode {
            Mode::Explore => Some(self.explore_mode.clone()),
            Mode::ExploreSubmode(s) => self.explore_submodes.clone().get(s).map(|a| a.to_owned()),
            Mode::Select => Some(self.select_mode.clone()),
            Mode::SelectSubmode(s) => self.select_submodes.clone().get(s).map(|a| a.to_owned()),
        };

        let kb = self.global.clone().into_iter();

        let kb: HashMap<Key, ActionMenu> = if let Some(modal_kb) = mode_bindings {
            kb.chain(modal_kb.into_iter()).collect()
        } else {
            kb.collect()
        };

        kb.into_iter()
            .map(|(k, am)| {
                (
                    k.clone(),
                    (
                        am.help,
                        am.actions
                            .into_iter()
                            .filter(|a| mode.clone().does_support(a))
                            .collect::<Vec<Action>>(),
                    ),
                )
            })
            .filter(|(_, (_, actions))| !actions.is_empty())
            .collect()
    }
}

impl Default for KeyBindings {
    fn default() -> Self {
        let yaml = r###"
            global:
              ctrl-c:
                help: quit
                actions:
                  - Terminate
              q:
                help: quit
                actions:
                  - Quit
              question-mark:
                help: print debug info
                actions:
                  - PrintAppState
              up:
                help: up
                actions:
                  - FocusPrevious
              down:
                help: down
                actions:
                  - FocusNext
              shift-g:
                help: bottom
                actions:
                  - FocusLast
              tilde:
                help: go home
                actions:
                  - ChangeDirectory: "~"
              dot:
                help: toggle show hidden
                actions:
                  - ToggleShowHidden
              right:
                help: enter
                actions:
                  - Enter
              left:
                help: back
                actions:
                  - Back
              o:
                help: open
                actions:
                  - Call:
                      command: bash
                      args:
                        - "-c"
                        - FILE="{{shellescape relativePath}}" && xdg-open "${FILE:?}" &> /dev/null
              e:
                help: edit
                actions:
                  - Call:
                      command: bash
                      args:
                        - -c
                        - FILE="{{shellescape relativePath}}" && "${EDITOR:-vim}" "${FILE:?}"
              forward-slash:
                help: search
                actions:
                  - Call:
                      command: bash
                      args:
                        - "-c"
                        - FILE="$(ls -a | fzf)" && xplr "${FILE:?}" || xplr "${PWD:?}"
                  - Quit

              s:
                help: shell
                actions:
                  - Call:
                      command: bash

              esc:
                help: quit
                actions:
                  - Quit

            explore_mode:
              g:
                help: go to
                actions:
                  - EnterSubmode: GoTo
              return:
                help: done
                actions:
                  - PrintFocused
              space:
                help: select
                actions:
                  - Select
                  - FocusNext
            explore_submodes:
              GoTo:
                g:
                  help: top
                  actions:
                    - FocusFirst
                    - ExitSubmode
            select_mode:
              space:
                help: toggle selection
                actions:
                  - ToggleSelection
                  - FocusNext
              g:
                help: go to
                actions:
                  - EnterSubmode: GoTo
              return:
                help: done
                actions:
                  - PrintSelected
                  - Quit
            select_submodes:
              GoTo:
                g:
                  help: top
                  actions:
                    - FocusFirst
                    - ExitSubmode
            "###;
        serde_yaml::from_str(yaml).unwrap()
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
        let yaml = r###"
          show_hidden: false
          table:
            header:
              cols:
              - format: "│      path"
              - format: "is symlink"
              - format: "index"
              height: 1
              style:
                add_modifier:
                  bits: 1
                sub_modifier:
                  bits: 0
            row:
              cols:
              - format: "{{tree}}{{prefix}}{{icon}} {{relativePath}}{{#if isDir}}/{{/if}}{{suffix}}"
              - format: "{{isSymlink}}"
              - format: "{{focusRelativeIndex}}/{{bufferRelativeIndex}}/{{index}}/{{total}}"

            col_spacing: 3
            col_widths:
              - percentage: 60
              - percentage: 20
              - percentage: 20

            tree:
            - format: "├─"
            - format: "├─"
            - format: "╰─"

          normal_ui:
            prefix: "  "
            suffix: ""

          focused_ui:
            prefix: "▸["
            suffix: "]"
            style:
              fg: Blue
              add_modifier:
                bits: 1
              sub_modifier:
                bits: 0

          selected_ui:
            prefix: " {"
            suffix: "}"
            style:
              fg: LightGreen
              add_modifier:
                bits: 1
              sub_modifier:
                bits: 0
                "###;
        serde_yaml::from_str(yaml).unwrap()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub version: String,

    #[serde(default)]
    pub general: GeneralConfig,

    #[serde(default)]
    pub filetypes: FileTypesConfig,

    #[serde(default)]
    pub key_bindings: KeyBindings,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            version: VERSION.into(),
            general: Default::default(),
            filetypes: Default::default(),
            key_bindings: Default::default(),
        }
    }
}
