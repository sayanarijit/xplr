use crate::app::ExternalMsg;
use crate::app::HelpMenuLine;
use crate::app::NodeFilter;
use crate::app::NodeSorter;
use crate::app::NodeSorterApplicable;
use crate::ui::Border;
use crate::ui::Constraint;
use crate::ui::Layout;
use crate::ui::Style;
use indexmap::IndexSet;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Action {
    #[serde(default)]
    pub help: Option<String>,

    #[serde(default)]
    pub messages: Vec<ExternalMsg>,
}

impl Action {
    pub fn sanitized(self, read_only: bool) -> Option<Self> {
        if self.messages.is_empty() {
            None
        } else if read_only {
            if self.messages.iter().all(ExternalMsg::is_read_only) {
                Some(self)
            } else {
                None
            }
        } else {
            Some(self)
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NodeTypeConfig {
    #[serde(default)]
    pub style: Style,

    #[serde(default)]
    pub meta: HashMap<String, String>,
}

impl NodeTypeConfig {
    pub fn extend(mut self, other: &Self) -> Self {
        self.style = self.style.extend(&other.style);
        self.meta.extend(other.meta.to_owned());
        self
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NodeTypesConfig {
    #[serde(default)]
    pub directory: NodeTypeConfig,

    #[serde(default)]
    pub file: NodeTypeConfig,

    #[serde(default)]
    pub symlink: NodeTypeConfig,

    #[serde(default)]
    pub mime_essence: HashMap<String, HashMap<String, NodeTypeConfig>>,

    #[serde(default)]
    pub extension: HashMap<String, NodeTypeConfig>,

    #[serde(default)]
    pub special: HashMap<String, NodeTypeConfig>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UiConfig {
    #[serde(default)]
    pub prefix: Option<String>,

    #[serde(default)]
    pub suffix: Option<String>,

    #[serde(default)]
    pub style: Style,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct UiElement {
    #[serde(default)]
    pub format: Option<String>,

    #[serde(default)]
    pub style: Style,
}

impl UiElement {
    pub fn extend(mut self, other: &Self) -> Self {
        self.format = other.format.to_owned().or(self.format);
        self.style = self.style.extend(&other.style);
        self
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TableRowConfig {
    #[serde(default)]
    pub cols: Option<Vec<UiElement>>,

    #[serde(default)]
    pub style: Style,

    #[serde(default)]
    pub height: Option<u16>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TableConfig {
    #[serde(default)]
    pub header: TableRowConfig,

    #[serde(default)]
    pub row: TableRowConfig,

    #[serde(default)]
    pub style: Style,

    #[serde(default)]
    pub tree: Option<(UiElement, UiElement, UiElement)>,

    #[serde(default)]
    pub col_spacing: Option<u16>,

    #[serde(default)]
    pub col_widths: Option<Vec<Constraint>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LogsConfig {
    #[serde(default)]
    pub info: UiElement,

    #[serde(default)]
    pub success: UiElement,

    #[serde(default)]
    pub warning: UiElement,

    #[serde(default)]
    pub error: UiElement,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SortDirectionIdentifiersUi {
    #[serde(default)]
    pub forward: UiElement,

    #[serde(default)]
    pub reverse: UiElement,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SortAndFilterUi {
    #[serde(default)]
    pub separator: UiElement,

    #[serde(default)]
    pub default_identifier: UiElement,

    #[serde(default)]
    pub sort_direction_identifiers: SortDirectionIdentifiersUi,

    #[serde(default)]
    pub sorter_identifiers: HashMap<NodeSorter, UiElement>,

    #[serde(default)]
    pub filter_identifiers: HashMap<NodeFilter, UiElement>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PanelUi {
    #[serde(default)]
    pub default: PanelUiConfig,

    #[serde(default)]
    pub table: PanelUiConfig,

    #[serde(default)]
    pub sort_and_filter: PanelUiConfig,

    #[serde(default)]
    pub selection: PanelUiConfig,

    #[serde(default)]
    pub input_and_logs: PanelUiConfig,

    #[serde(default)]
    pub help_menu: PanelUiConfig,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GeneralConfig {
    #[serde(default)]
    pub enable_mouse: bool,

    #[serde(default)]
    pub show_hidden: bool,

    #[serde(default)]
    pub read_only: bool,

    #[serde(default)]
    pub enable_recover_mode: bool,

    #[serde(default)]
    pub hide_remaps_in_help_menu: bool,

    #[serde(default)]
    pub prompt: UiElement,

    #[serde(default)]
    pub logs: LogsConfig,

    #[serde(default)]
    pub table: TableConfig,

    #[serde(default)]
    pub default_ui: UiConfig,

    #[serde(default)]
    pub focus_ui: UiConfig,

    #[serde(default)]
    pub selection_ui: UiConfig,

    #[serde(default)]
    pub focus_selection_ui: UiConfig,

    #[serde(default)]
    pub sort_and_filter_ui: SortAndFilterUi,

    #[serde(default)]
    pub panel_ui: PanelUi,

    #[serde(default)]
    pub initial_sorting: Option<IndexSet<NodeSorterApplicable>>,

    #[serde(default)]
    pub initial_mode: Option<String>,

    #[serde(default)]
    pub initial_layout: Option<String>,

    #[serde(default)]
    pub start_fifo: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct KeyBindings {
    #[serde(default)]
    pub on_key: BTreeMap<String, Action>,

    #[serde(default)]
    pub on_alphabet: Option<Action>,

    #[serde(default)]
    pub on_number: Option<Action>,

    #[serde(default)]
    pub on_alphanumeric: Option<Action>,

    #[serde(default)]
    pub on_special_character: Option<Action>,

    #[serde(default)]
    pub on_character: Option<Action>,

    #[serde(default)]
    pub on_navigation: Option<Action>,

    #[serde(default)]
    pub on_function: Option<Action>,

    #[serde(default)]
    pub default: Option<Action>,
    // Checklist for adding field:
    // - [ ] Update App::handle_key
    // - [ ] Update KeyBindings::sanitized
    // - [ ] Update Mode::help_menu
    // - [ ] Update configure-key-bindings.md
    // - [ ] Update debug-key-bindings.md
}

impl KeyBindings {
    pub fn sanitized(mut self, read_only: bool) -> Self {
        if read_only {
            self.on_key = self
                .on_key
                .into_iter()
                .filter_map(|(k, a)| a.sanitized(read_only).map(|a| (k, a)))
                .collect();

            self.on_alphabet =
                self.on_alphabet.and_then(|a| a.sanitized(read_only));
            self.on_number =
                self.on_number.and_then(|a| a.sanitized(read_only));
            self.on_alphanumeric =
                self.on_alphanumeric.and_then(|a| a.sanitized(read_only));
            self.on_special_character = self
                .on_special_character
                .and_then(|a| a.sanitized(read_only));
            self.on_character =
                self.on_character.and_then(|a| a.sanitized(read_only));
            self.on_navigation =
                self.on_navigation.and_then(|a| a.sanitized(read_only));
            self.on_function =
                self.on_function.and_then(|a| a.sanitized(read_only));
            self.default = self.default.and_then(|a| a.sanitized(read_only));
        };
        self
    }
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Mode {
    #[serde(default)]
    pub name: String,

    #[serde(default)]
    pub help: Option<String>,

    #[serde(default)]
    pub extra_help: Option<String>,

    #[serde(default)]
    pub key_bindings: KeyBindings,

    #[serde(default)]
    pub layout: Option<Layout>,
}

impl Mode {
    pub fn sanitized(mut self, read_only: bool) -> Self {
        self.key_bindings = self.key_bindings.sanitized(read_only);
        self
    }

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
                let lines = extra_help_lines
                    .unwrap_or_default()
                    .into_iter()
                    .chain(self.key_bindings.on_key.iter().filter_map(
                        |(k, a)| {
                            let remaps = self
                                .key_bindings
                                .on_key
                                .iter()
                                .filter_map(|(rk, ra)| {
                                    if rk == k {
                                        None
                                    } else if a == ra {
                                        Some(rk.clone())
                                    } else {
                                        None
                                    }
                                })
                                .collect::<Vec<String>>();
                            a.help.clone().map(|h| {
                                HelpMenuLine::KeyMap(k.into(), remaps, h)
                            })
                        },
                    ))
                    .chain(
                        self.key_bindings
                            .on_alphabet
                            .iter()
                            .map(|a| ("[a-Z]", a.help.clone()))
                            .filter_map(|(k, mh)| {
                                mh.map(|h| {
                                    HelpMenuLine::KeyMap(k.into(), vec![], h)
                                })
                            }),
                    )
                    .chain(
                        self.key_bindings
                            .on_number
                            .iter()
                            .map(|a| ("[0-9]", a.help.clone()))
                            .filter_map(|(k, mh)| {
                                mh.map(|h| {
                                    HelpMenuLine::KeyMap(k.into(), vec![], h)
                                })
                            }),
                    )
                    .chain(
                        self.key_bindings
                            .on_alphanumeric
                            .iter()
                            .map(|a| ("[0-Z]", a.help.clone()))
                            .filter_map(|(k, mh)| {
                                mh.map(|h| {
                                    HelpMenuLine::KeyMap(k.into(), vec![], h)
                                })
                            }),
                    )
                    .chain(
                        self.key_bindings
                            .on_special_character
                            .iter()
                            .map(|a| ("[^0-Z]", a.help.clone()))
                            .filter_map(|(k, mh)| {
                                mh.map(|h| {
                                    HelpMenuLine::KeyMap(k.into(), vec![], h)
                                })
                            }),
                    )
                    .chain(
                        self.key_bindings
                            .on_character
                            .iter()
                            .map(|a| ("[*]", a.help.clone()))
                            .filter_map(|(k, mh)| {
                                mh.map(|h| {
                                    HelpMenuLine::KeyMap(k.into(), vec![], h)
                                })
                            }),
                    )
                    .chain(
                        self.key_bindings
                            .on_navigation
                            .iter()
                            .map(|a| ("[nav]", a.help.clone()))
                            .filter_map(|(k, mh)| {
                                mh.map(|h| {
                                    HelpMenuLine::KeyMap(k.into(), vec![], h)
                                })
                            }),
                    )
                    .chain(
                        self.key_bindings
                            .on_function
                            .iter()
                            .map(|a| ("[f1-f12]", a.help.clone()))
                            .filter_map(|(k, mh)| {
                                mh.map(|h| {
                                    HelpMenuLine::KeyMap(k.into(), vec![], h)
                                })
                            }),
                    )
                    .chain(
                        self.key_bindings
                            .default
                            .iter()
                            .map(|a| ("[default]", a.help.clone()))
                            .filter_map(|(k, mh)| {
                                mh.map(|h| {
                                    HelpMenuLine::KeyMap(k.into(), vec![], h)
                                })
                            }),
                    );

                let mut remapped = HashSet::new();
                let mut result = vec![];

                for line in lines {
                    match line {
                        HelpMenuLine::Paragraph(p) => {
                            result.push(HelpMenuLine::Paragraph(p))
                        }
                        HelpMenuLine::KeyMap(k, r, d) => {
                            if !remapped.contains(&k) {
                                for k in r.iter() {
                                    remapped.insert(k.clone());
                                }
                                result.push(HelpMenuLine::KeyMap(k, r, d));
                            }
                        }
                    }
                }

                result
            })
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BuiltinModesConfig {
    #[serde(default)]
    pub default: Mode,

    #[serde(default)]
    pub recover: Mode,

    #[serde(default)]
    pub selection_ops: Mode,

    #[serde(default)]
    pub create: Mode,

    #[serde(default)]
    pub create_directory: Mode,

    #[serde(default)]
    pub create_file: Mode,

    #[serde(default)]
    pub number: Mode,

    #[serde(default)]
    pub go_to: Mode,

    #[serde(default)]
    pub rename: Mode,

    #[serde(default)]
    pub delete: Mode,

    #[serde(default)]
    pub action: Mode,

    #[serde(default)]
    pub search: Mode,

    #[serde(default)]
    pub filter: Mode,

    #[serde(default)]
    pub relative_path_does_contain: Mode,

    #[serde(default)]
    pub relative_path_does_not_contain: Mode,

    #[serde(default)]
    pub sort: Mode,

    #[serde(default)]
    pub switch_layout: Mode,

    #[serde(default)]
    pub quit: Mode,
}

impl BuiltinModesConfig {
    pub fn get(&self, name: &str) -> Option<&Mode> {
        match name {
            "default" => Some(&self.default),
            "recover" => Some(&self.recover),
            "selection ops" => Some(&self.selection_ops),
            "selection_ops" => Some(&self.selection_ops),
            "create" => Some(&self.create),
            "create file" => Some(&self.create_file),
            "create_file" => Some(&self.create_file),
            "create directory" => Some(&self.create_directory),
            "create_directory" => Some(&self.create_directory),
            "number" => Some(&self.number),
            "go to" => Some(&self.go_to),
            "go_to" => Some(&self.go_to),
            "rename" => Some(&self.rename),
            "delete" => Some(&self.delete),
            "action" => Some(&self.action),
            "search" => Some(&self.search),
            "sort" => Some(&self.sort),
            "filter" => Some(&self.filter),
            "relative_path_does_contain" => {
                Some(&self.relative_path_does_contain)
            }
            "relative path does contain" => {
                Some(&self.relative_path_does_contain)
            }
            "relative_path_does_not_contain" => {
                Some(&self.relative_path_does_not_contain)
            }
            "relative path does not contain" => {
                Some(&self.relative_path_does_not_contain)
            }
            "switch layout" => Some(&self.switch_layout),
            "switch_layout" => Some(&self.switch_layout),
            "quit" => Some(&self.quit),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ModesConfig {
    #[serde(default)]
    pub builtin: BuiltinModesConfig,

    #[serde(default)]
    pub custom: HashMap<String, Mode>,
}

impl ModesConfig {
    pub fn get(&self, name: &str) -> Option<&Mode> {
        self.builtin.get(name).or_else(|| self.custom.get(name))
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct PanelUiConfig {
    #[serde(default)]
    pub title: UiElement,

    #[serde(default)]
    pub borders: Option<IndexSet<Border>>,

    #[serde(default)]
    pub style: Style,
}

impl PanelUiConfig {
    pub fn extend(mut self, other: &Self) -> Self {
        self.title = self.title.extend(&other.title);
        self.borders = other.borders.to_owned().or(self.borders);
        self.style = self.style.extend(&other.style);
        self
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BuiltinLayoutsConfig {
    #[serde(default)]
    pub default: Layout,

    #[serde(default)]
    pub no_help: Layout,

    #[serde(default)]
    pub no_selection: Layout,

    #[serde(default)]
    pub no_help_no_selection: Layout,
}

impl BuiltinLayoutsConfig {
    pub fn get(&self, name: &str) -> Option<&Layout> {
        match name {
            "default" => Some(&self.default),
            "no_help" => Some(&self.no_help),
            "no help" => Some(&self.no_help),
            "no_selection" => Some(&self.no_selection),
            "no selection" => Some(&self.no_selection),
            "no_help_no_selection" => Some(&self.no_help_no_selection),
            "no help no selection" => Some(&self.no_help_no_selection),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LayoutsConfig {
    #[serde(default)]
    pub builtin: BuiltinLayoutsConfig,

    #[serde(default)]
    pub custom: HashMap<String, Layout>,
}

impl LayoutsConfig {
    pub fn get_builtin(&self, name: &str) -> Option<&Layout> {
        self.builtin.get(name)
    }

    pub fn get_custom(&self, name: &str) -> Option<&Layout> {
        self.custom.get(name)
    }

    pub fn get(&self, name: &str) -> Option<&Layout> {
        self.get_builtin(name).or_else(|| self.get_custom(name))
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    #[serde(default)]
    pub layouts: LayoutsConfig,

    #[serde(default)]
    pub general: GeneralConfig,

    #[serde(default)]
    pub node_types: NodeTypesConfig,

    #[serde(default)]
    pub modes: ModesConfig,
}
