use crate::app::ExternalMsg;
use crate::app::HelpMenuLine;
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
                icon: "ð".into(),
                style: Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(Color::Blue),
            },

            file: FileTypeConfig {
                icon: "ƒ".into(),
                style: Default::default(),
            },

            symlink: FileTypeConfig {
                icon: "§".into(),
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
                help: up [k]
                messages:
                  - FocusPrevious

              k:
                messages:
                  - FocusPrevious

              down:
                help: down [j]
                messages:
                  - FocusNext

              j:
                messages:
                  - FocusNext

              right:
                help: enter [l]
                messages:
                  - Enter

              l:
                messages:
                  - Enter

              left:
                help: back [h]
                messages:
                  - Back

              h:
                messages:
                  - Back

              g:
                help: go to
                messages:
                  - SwitchMode: go to

              G:
                help: go to bottom
                messages:
                  - FocusLast

              ctrl-f:
                help: search [/]
                messages:
                  - ResetNodeFilters
                  - SwitchMode: search
                  - SetInputBuffer: ""
                  - Explore

              /:
                messages:
                  - ResetNodeFilters
                  - SwitchMode: search
                  - SetInputBuffer: ""
                  - Explore

              d:
                help: delete
                messages:
                  - SwitchMode: delete

              ":":
                help: action
                messages:
                  - SwitchMode: action

              space:
                help: toggle selection [v]
                messages:
                  - ToggleSelection
                  - FocusNext

              v:
                messages:
                  - ToggleSelection
                  - FocusNext

              ".":
                help: show hidden
                messages:
                  - ToggleNodeFilter:
                      filter: RelativePathDoesNotStartWith
                      input: .
                  - Explore

              enter:
                help: quit with result
                messages:
                  - PrintResultAndQuit

              "#":
                messages:
                  - PrintAppStateAndQuit

              "?":
                help: global help menu
                messages:
                  - Call:
                      command: bash
                      args:
                        - -c
                        - |
                          echo -e "${XPLR_GLOBAL_HELP_MENU}"
                          echo
                          read -p "[enter to continue]"

              ctrl-c:
                help: cancel & quit [q|esc]
                messages:
                  - Terminate

              q:
                messages:
                  - Terminate

              esc:
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
                ExternalMsg::ResetInputBuffer,
                ExternalMsg::SwitchMode("number".into()),
                ExternalMsg::BufferInputFromKey,
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

impl Mode {
    pub fn help_menu(&self) -> Vec<HelpMenuLine> {
        let extra_help_lines = self.extra_help.clone().map(|e| {
            e.lines()
                .map(|l| HelpMenuLine::Paragraph(l.into()))
                .collect::<Vec<HelpMenuLine>>()
        });

        self.help
            .clone()
            .map(|h| {
                h.lines()
                    .map(|l| HelpMenuLine::Paragraph(l.into()))
                    .collect()
            })
            .unwrap_or_else(|| {
                extra_help_lines
                    .unwrap_or_default()
                    .into_iter()
                    .chain(self.key_bindings.on_key.iter().filter_map(|(k, a)| {
                        a.help
                            .clone()
                            .map(|h| HelpMenuLine::KeyMap(k.into(), h.into()))
                    }))
                    .chain(
                        self.key_bindings
                            .on_alphabet
                            .iter()
                            .map(|a| ("[a-Z]", a.help.clone()))
                            .filter_map(|(k, mh)| {
                                mh.map(|h| HelpMenuLine::KeyMap(k.into(), h.into()))
                            }),
                    )
                    .chain(
                        self.key_bindings
                            .on_number
                            .iter()
                            .map(|a| ("[0-9]", a.help.clone()))
                            .filter_map(|(k, mh)| {
                                mh.map(|h| HelpMenuLine::KeyMap(k.into(), h.into()))
                            }),
                    )
                    .chain(
                        self.key_bindings
                            .on_special_character
                            .iter()
                            .map(|a| ("[spcl chars]", a.help.clone()))
                            .filter_map(|(k, mh)| {
                                mh.map(|h| HelpMenuLine::KeyMap(k.into(), h.into()))
                            }),
                    )
                    .chain(
                        self.key_bindings
                            .default
                            .iter()
                            .map(|a| ("[default]", a.help.clone()))
                            .filter_map(|(k, mh)| {
                                mh.map(|h| HelpMenuLine::KeyMap(k.into(), h.into()))
                            }),
                    )
                    .collect()
            })
    }
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
        let search_mode: Mode = serde_yaml::from_str(
            r###"
              name: search
              key_bindings:
                on_key:
                  enter:
                    help: focus
                    messages:
                      - ResetNodeFilters
                      - SwitchMode: default
                      - Explore
                  
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
                      - ResetNodeFilters
                      - Enter
                      - SwitchMode: default
                      - Explore

                  left:
                    help: back
                    messages:
                      - ResetNodeFilters
                      - Back
                      - SwitchMode: default
                      - Explore

                  esc:
                    help: cancel
                    messages:
                      - ResetNodeFilters
                      - SwitchMode: default
                      - Explore

                  backspace:
                    help: clear
                    messages:
                      - SetInputBuffer: ""
                      - ResetNodeFilters
                      - Explore

                  ctrl-c:
                    help: cancel & quit
                    messages:
                      - Terminate

                default:
                  messages:
                    - BufferInputFromKey
                    - ResetNodeFilters
                    - AddNodeFilterFromInput:
                        filter: RelativePathDoesContain
                        case_sensitive: false
                    - Explore
            "###,
        )
        .unwrap();

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

                  x:
                    help: open in gui
                    messages:
                      - Call:
                          command: bash
                          args:
                            - -c
                            - |
                              xdg-open "${XPLR_FOCUS_PATH:?}" &> /dev/null
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

        let action_mode: Mode = serde_yaml::from_str(
            r###"
              name: action to
              key_bindings:
                on_number:
                  help: go to index
                  messages:
                    - ResetInputBuffer
                    - SwitchMode: number
                    - BufferInputFromKey

                on_key:
                  "!":
                    help: shell
                    messages:
                      - Call:
                          command: bash
                      - Explore
                      - SwitchMode: default

                  c:
                    help: create
                    messages:
                      - SwitchMode: create

                  s:
                    help: selection operations
                    messages:
                      - SwitchMode: selection ops

                  l:
                    help: logs
                    messages:
                      - Call:
                          command: bash
                          args:
                            - -c
                            - |
                              echo -e "$XPLR_LOGS"
                              read -p "[enter to continue]"
                      - SwitchMode: default

                  ctrl-c:
                    help: cancel & quit [q]
                    messages:
                      - Terminate

                  q:
                    messages:
                      - Terminate

                default:
                  messages:
                    - SwitchMode: default
            "###,
        )
        .unwrap();

        let selection_ops_mode: Mode = serde_yaml::from_str(
            r###"
              name: selection ops
              key_bindings:
                on_key:
                  c:
                    help: copy here
                    messages:
                      - Call:
                          command: bash
                          args:
                            - -c
                            - |
                              (while IFS= read -r line; do
                                if cp -v "${line:?}" ./; then
                                  echo "LogSuccess: $line copied to $PWD" >> "${XPLR_PIPE_MSG_IN:?}"
                                else
                                  echo "LogError: failed to copy $line to $PWD" >> "${XPLR_PIPE_MSG_IN:?}"
                                fi
                              done <<< "${XPLR_SELECTION:?}")
                              echo Explore >> "${XPLR_PIPE_MSG_IN:?}"
                              echo ClearSelection >> "${XPLR_PIPE_MSG_IN:?}"
                              read -p "[enter to continue]"
                      - SwitchMode: default

                  m:
                    help: move here
                    messages:
                      - Call:
                          command: bash
                          args:
                            - -c
                            - |
                              (while IFS= read -r line; do
                                if mv -v "${line:?}" ./; then
                                  echo "LogSuccess: $line moved to $PWD" >> "${XPLR_PIPE_MSG_IN:?}"
                                else
                                  echo "LogError: failed to move $line to $PWD" >> "${XPLR_PIPE_MSG_IN:?}"
                                fi
                              done <<< "${XPLR_SELECTION:?}")
                              echo Explore >> "${XPLR_PIPE_MSG_IN:?}"
                              read -p "[enter to continue]"
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
                    help: to up [k]
                    messages:
                      - FocusPreviousByRelativeIndexFromInput
                      - SwitchMode: default

                  k:
                    messages:
                      - FocusPreviousByRelativeIndexFromInput
                      - SwitchMode: default

                  down:
                    help: to down [j]
                    messages:
                      - FocusNextByRelativeIndexFromInput
                      - SwitchMode: default

                  j:
                    messages:
                      - FocusNextByRelativeIndexFromInput
                      - SwitchMode: default

                  enter:
                    help: to index
                    messages:
                      - FocusByIndexFromInput
                      - SwitchMode: default

                  backspace:
                    help: clear
                    messages:
                      - ResetInputBuffer

                  ctrl-c:
                    help: cancel & quit
                    messages:
                      - Terminate

                on_number:
                  help: input
                  messages:
                    - BufferInputFromKey

                default:
                  messages:
                    - SwitchMode: default
            "###,
        )
        .unwrap();

        let create_mode: Mode = serde_yaml::from_str(
            r###"
              name: create
              key_bindings:
                on_key:
                  f:
                    help: create file
                    messages:
                      - SwitchMode: create file
                      - SetInputBuffer: ""

                  d:
                    help: create directory
                    messages:
                      - SwitchMode: create directory
                      - SetInputBuffer: ""

                  esc:
                    help: cancel
                    messages:
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

        let create_file_mode: Mode = serde_yaml::from_str(
            r###"
              name: create file
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
                              PTH="${XPLR_INPUT_BUFFER:?}"
                              if touch "${PTH:?}"; then
                                echo "LogSuccess: $PTH created" >> "${XPLR_PIPE_MSG_IN:?}"
                                echo Explore >> "${XPLR_PIPE_MSG_IN:?}"
                              else
                                echo "LogError: failed to create $PTH" >> "${XPLR_PIPE_MSG_IN:?}"
                                echo Refresh >> "${XPLR_PIPE_MSG_IN:?}"
                              fi
                      - SwitchMode: default

                  backspace:
                    help: clear
                    messages:
                      - SetInputBuffer: ""

                  esc:
                    help: cancel
                    messages:
                      - SwitchMode: default

                  ctrl-c:
                    help: cancel & quit
                    messages:
                      - Terminate

                default:
                  messages:
                    - BufferInputFromKey
            "###,
        )
        .unwrap();

        let create_dir_mode: Mode = serde_yaml::from_str(
            r###"
              name: create directory
              key_bindings:
                on_key:
                  enter:
                    help: create directory
                    messages:
                      - Call:
                          command: bash
                          args:
                            - -c
                            - |
                              PTH="${XPLR_INPUT_BUFFER:?}"
                              if mkdir -p "$PTH"; then
                                echo Explore >> "${XPLR_PIPE_MSG_IN:?}"
                                echo "LogSuccess: $PTH created" >> "${XPLR_PIPE_MSG_IN:?}"
                              else
                                echo "LogError: failed to create $PTH" >> "${XPLR_PIPE_MSG_IN:?}"
                              fi
                      - SwitchMode: default

                  backspace:
                    help: clear
                    messages:
                      - SetInputBuffer: ""

                  esc:
                    help: cancel
                    messages:
                      - SwitchMode: default

                  ctrl-c:
                    help: cancel & quit
                    messages:
                      - Terminate

                default:
                  messages:
                    - BufferInputFromKey
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
                              (while IFS= read -r line; do
                                if [ -d "$line" ]; then
                                  if rmdir -v "${line:?}"; then
                                    echo "LogSuccess: $line deleted" >> "${XPLR_PIPE_MSG_IN:?}"
                                  else
                                    echo "LogError: failed to delete $line" >> "${XPLR_PIPE_MSG_IN:?}"
                                  fi
                                else
                                  if rm -v "${line:?}"; then
                                    echo "LogSuccess: $line deleted" >> "${XPLR_PIPE_MSG_IN:?}"
                                  else
                                    echo "LogError: failed to delete $line" >> "${XPLR_PIPE_MSG_IN:?}"
                                  fi
                                fi
                              done <<< "${XPLR_RESULT:?}")
                              echo Explore >> "${XPLR_PIPE_MSG_IN:?}"
                              read -p "[enter to continue]"
                      - SwitchMode: default

                  D:
                    help: force delete
                    messages:
                      - Call:
                          command: bash
                          args:
                            - -c
                            - |
                              (while IFS= read -r line; do
                                if rm -rfv "${line:?}"; then
                                  echo "LogSuccess: $line deleted" >> "${XPLR_PIPE_MSG_IN:?}"
                                else
                                  echo "LogError: failed to delete $line" >> "${XPLR_PIPE_MSG_IN:?}"
                                fi
                              done <<< "${XPLR_RESULT:?}")
                              echo Explore >> "${XPLR_PIPE_MSG_IN:?}"
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
        modes.insert("go to".into(), goto_mode);
        modes.insert("number".into(), number_mode);
        modes.insert("create".into(), create_mode);
        modes.insert("create file".into(), create_file_mode);
        modes.insert("create directory".into(), create_dir_mode);
        modes.insert("delete".into(), delete_mode);
        modes.insert("action".into(), action_mode);
        modes.insert("search".into(), search_mode);
        modes.insert("selection ops".into(), selection_ops_mode);

        Self {
            version: VERSION.into(),
            general: Default::default(),
            filetypes: Default::default(),
            modes,
        }
    }
}
