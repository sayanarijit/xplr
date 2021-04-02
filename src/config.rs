use crate::app::ExternalMsg;
use crate::app::VERSION;
use serde::{Deserialize, Serialize};
use serde_yaml;
use std::collections::BTreeMap;
use std::collections::HashMap;
use tui::layout::Constraint as TUIConstraint;
use tui::style::Color;
use tui::style::Modifier;
use tui::style::Style;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Action {
    #[serde(default)]
    pub help: Option<String>,

    #[serde(default)]
    pub messages: Vec<ExternalMsg>,
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
    pub mime_essence: HashMap<String, FileTypeConfig>,
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

            mime_essence: Default::default(),
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
    pub selection_ui: UIConfig,
}

impl Default for GeneralConfig {
    fn default() -> Self {
        let yaml = r###"
          show_hidden: false
          table:
            header:
              cols:
              - format: "│     path"
              - format: "type"
              - format: " index"
              height: 1
              style:
                add_modifier:
                  bits: 1
                sub_modifier:
                  bits: 0
            row:
              cols:
              - format: "{{{tree}}}{{{prefix}}}{{{icon}}} {{{relativePath}}}{{#if isDir}}/{{/if}}{{{suffix}}}"
              - format: "{{{mimeEssence}}}"
              - format: "{{#if isBeforeFocus}}-{{else}} {{/if}}{{{relativeIndex}}}/{{{index}}}/{{{total}}}"

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

          selection_ui:
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
pub struct KeyBindings {
    #[serde(default)]
    pub on_key: BTreeMap<String, Action>,

    #[serde(default)]
    pub on_alphabet: Option<Action>,

    #[serde(default)]
    pub on_number: Option<Action>,

    #[serde(default)]
    pub on_special_character: Option<Action>,

    #[serde(default)]
    pub default: Option<Action>,
}

impl Default for KeyBindings {
    fn default() -> Self {
        let on_key: BTreeMap<String, Action> = serde_yaml::from_str(
            r###"
              up:
                help: up
                messages:
                  - FocusPrevious

              down:
                help: down
                messages:
                  - FocusNext

              right:
                help: enter
                messages:
                  - Enter

              left:
                help: back
                messages:
                  - Back

              g:
                help: go to
                messages:
                  - SwitchMode: goto

              G:
                help: bottom
                messages:
                  - FocusLast

              s:
                help: shell
                messages:
                  - Call:
                      command: bash
                      args: []
                  - Explore

              /:
                help: search
                messages:
                  - Call:
                      command: bash
                      args:
                        - "-c"
                        - |
                            PTH=$(echo -e "${XPLR_DIRECTORY_NODES:?}" | fzf)
                            if [ -d "$PTH" ]; then
                                echo "ChangeDirectory: ${PTH:?}" >> "${XPLR_PIPE_MSG_IN:?}"
                            elif [ -f "$PTH" ]; then
                                echo "FocusPath: ${PTH:?}" >> "${XPLR_PIPE_MSG_IN:?}"
                            fi

              space:
                help: toggle selection
                messages:
                  - ToggleSelection
                  - FocusNext

              n:
                help: create new
                messages:
                  - SwitchMode: create

              d:
                help: delete
                messages:
                  - SwitchMode: delete

              c:
                help: copy here
                messages:
                  - Call:
                      command: bash
                      args:
                        - -c
                        - |
                          while IFS= read -r line; do
                            cp -v "${line:?}" ./
                          done <<< "${XPLR_SELECTION:?}"
                          read -p "[enter to continue]"
                  - ClearSelection
                  - Explore

              m:
                help: move here
                messages:
                  - Call:
                      command: bash
                      args:
                        - -c
                        - |
                          while IFS= read -r line; do
                            mv -v "${line:?}" ./
                          done <<< "${XPLR_SELECTION:?}"
                          read -p "[enter to continue]"
                  - Explore

              enter:
                help: quit with result
                messages:
                  - PrintResultAndQuit

              o:
                help: open
                messages:
                  - Call:
                      command: bash
                      args:
                        - -c
                        - |
                          xdg-open "${XPLR_FOCUS_PATH:?}" &> /dev/null

              ctrl-l:
                help: clear
                messages:
                  - ClearScreen
                  - Refresh

              "#":
                help: quit with debug
                messages:
                  - PrintAppStateAndQuit

              esc:
                help: cancel & quit
                messages:
                  - Terminate

              q:
                help: cancel & quit
                messages:
                  - Terminate

              ctrl-c:
                help: cancel & quit
                messages:
                  - Terminate
            "###,
        )
        .unwrap();

        let default = Some(Action {
            help: None,
            messages: vec![ExternalMsg::SwitchMode("default".into())],
        });

        let on_number = Some(Action {
            help: Some("input".to_string()),
            messages: vec![
                ExternalMsg::BufferStringFromKey,
                ExternalMsg::SwitchMode("number".into()),
            ],
        });

        Self {
            on_key,
            on_alphabet: Default::default(),
            on_number,
            on_special_character: Default::default(),
            default,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mode {
    pub name: String,

    #[serde(default)]
    pub help: Option<String>,

    #[serde(default)]
    pub extra_help: Option<String>,

    #[serde(default)]
    pub key_bindings: KeyBindings,
}

impl Default for Mode {
    fn default() -> Self {
        Self {
            name: "default".into(),
            help: Default::default(),
            extra_help: Default::default(),
            key_bindings: Default::default(),
        }
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
    pub modes: HashMap<String, Mode>,
}

impl Default for Config {
    fn default() -> Self {
        let goto_mode: Mode = serde_yaml::from_str(
            r###"
              name: go to
              key_bindings:
                on_key:
                  g:
                    help: top
                    messages:
                      - FocusFirst
                      - SwitchMode: default

                  ctrl-c:
                    help: cancel & quit
                    messages:
                      - Terminate

                default:
                  messages:
                    - SwitchMode: default
            "###,
        )
        .unwrap();

        let number_mode: Mode = serde_yaml::from_str(
            r###"
              name: number
              key_bindings:
                on_key:
                  up:
                    help: go up
                    messages:
                      - FocusPreviousByRelativeIndexFromInput
                      - ResetInputBuffer
                      - SwitchMode: default

                  down:
                    help: go down
                    messages:
                      - FocusNextByRelativeIndexFromInput
                      - ResetInputBuffer
                      - SwitchMode: default

                  enter:
                    help: to index
                    messages:
                      - FocusByIndexFromInput
                      - ResetInputBuffer
                      - SwitchMode: default

                  ctrl-c:
                    help: cancel & quit
                    messages:
                      - Terminate

                on_number:
                  help: input
                  messages:
                    - BufferStringFromKey

                default:
                  messages:
                    - ResetInputBuffer
                    - SwitchMode: default
            "###,
        )
        .unwrap();

        let create_mode: Mode = serde_yaml::from_str(
            r###"
              name: create
              key_bindings:
                on_key:
                  enter:
                    help: create file
                    messages:
                      - Call:
                          command: bash
                          args:
                            - -c
                            - |
                              touch "${XPLR_INPUT_BUFFER:?}"
                      - ResetInputBuffer
                      - SwitchMode: default
                      - Explore

                  ctrl-d:
                    help: create directory
                    messages:
                      - Call:
                          command: bash
                          args:
                            - -c
                            - |
                              mkdir -p "${XPLR_INPUT_BUFFER:?}"
                      - ResetInputBuffer
                      - SwitchMode: default
                      - Explore

                  esc:
                    help: cancel
                    messages:
                      - ResetInputBuffer
                      - SwitchMode: default

                  ctrl-c:
                    help: cancel & quit
                    messages:
                      - Terminate

                default:
                  messages:
                    - BufferStringFromKey
            "###,
        )
        .unwrap();

        let delete_mode: Mode = serde_yaml::from_str(
            r###"
              name: delete
              key_bindings:
                on_key:
                  d:
                    help: delete
                    messages:
                      - Call:
                          command: bash
                          args:
                            - -c
                            - |
                              while IFS= read -r line; do
                                if [ -d "$line" ]; then
                                  rmdir -v "${line:?}"
                                else
                                  rm -v "${line:?}"
                                fi
                              done <<< "${XPLR_RESULT:?}"
                              read -p "[Enter to continue]"
                      - SwitchMode: default
                      - Explore

                  D:
                    help: force delete
                    messages:
                      - Call:
                          command: bash
                          args:
                            - -c
                            - |
                              echo -e "${XPLR_RESULT:?}" | xargs -l rm -rfv
                              read -p "[enter to continue]"
                      - SwitchMode: default
                      - Explore

                  ctrl-c:
                    help: cancel & quit
                    messages:
                      - Terminate
                
                default:
                  messages:
                    - SwitchMode: default
            "###,
        )
        .unwrap();

        let mut modes: HashMap<String, Mode> = Default::default();
        modes.insert("default".into(), Mode::default());
        modes.insert("goto".into(), goto_mode);
        modes.insert("number".into(), number_mode);
        modes.insert("create".into(), create_mode);
        modes.insert("delete".into(), delete_mode);

        Self {
            version: VERSION.into(),
            general: Default::default(),
            filetypes: Default::default(),
            modes,
        }
    }
}
