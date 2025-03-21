use crate::app::ExternalMsg;
use crate::app::HelpMenuLine;
use crate::app::NodeFilter;
use crate::app::NodeSorter;
use crate::app::NodeSorterApplicable;
use crate::node::Node;
use crate::search::RankCriteria;
use crate::search::SearchAlgorithm;
use crate::ui::Border;
use crate::ui::BorderType;
use crate::ui::Constraint;
use crate::ui::Layout;
use crate::ui::Style;
use indexmap::IndexSet;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
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
        self.meta.extend(other.meta.clone());
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

impl NodeTypesConfig {
    pub fn get(&self, node: &Node) -> NodeTypeConfig {
        let mut node_type = if node.is_symlink {
            self.symlink.clone()
        } else if node.is_dir {
            self.directory.clone()
        } else {
            self.file.clone()
        };

        let mut me = node.mime_essence.splitn(2, '/');
        let mimetype: String = me.next().map(|s| s.into()).unwrap_or_default();
        let mimesub: String = me.next().map(|s| s.into()).unwrap_or_default();

        if let Some(conf) = self
            .mime_essence
            .get(&mimetype)
            .and_then(|t| t.get(&mimesub).or_else(|| t.get("*")))
        {
            node_type = node_type.extend(conf);
        }

        if let (Some(conf), false) = (self.extension.get(&node.extension), node.is_dir) {
            node_type = node_type.extend(conf);
        }

        if let Some(conf) = self.special.get(&node.relative_path) {
            node_type = node_type.extend(conf);
        }

        node_type
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UiConfig {
    #[serde(default)]
    pub prefix: Option<String>,

    #[serde(default)]
    pub suffix: Option<String>,

    #[serde(default)]
    pub style: Style,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct UiElement {
    #[serde(default)]
    pub format: Option<String>,

    #[serde(default)]
    pub style: Style,
}

impl UiElement {
    pub fn extend(mut self, other: &Self) -> Self {
        self.format = other.format.clone().or(self.format);
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
pub struct SelectionConfig {
    #[serde(default)]
    pub item: UiElement,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SearchConfig {
    #[serde(default)]
    pub algorithm: SearchAlgorithm,

    #[serde(default)]
    pub unordered: bool,

    #[serde(default)]
    pub exact_mode: bool,

    #[serde(default)]
    pub rank_criteria: Option<Vec<RankCriteria>>,
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
pub struct SearchDirectionIdentifiersUi {
    #[serde(default)]
    pub ordered: UiElement,

    #[serde(default)]
    pub unordered: UiElement,
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

    #[serde(default)]
    pub search_direction_identifiers: SearchDirectionIdentifiersUi,

    #[serde(default)]
    pub search_identifiers: HashMap<SearchAlgorithm, UiElement>,
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
    pub disable_debug_error_mode: bool,

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
    pub enforce_bounded_index_navigation: bool,

    #[serde(default)]
    pub prompt: UiElement,

    #[serde(default)]
    pub logs: LogsConfig,

    #[serde(default)]
    pub table: TableConfig,

    #[serde(default)]
    pub selection: SelectionConfig,

    #[serde(default)]
    pub search: SearchConfig,

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

    #[serde(default)]
    pub global_key_bindings: KeyBindings,

    #[serde(default)]
    pub paginated_scrolling: bool,

    #[serde(default)]
    pub scroll_padding: usize,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
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
    // Checklist for adding new field:
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

            self.on_alphabet = self.on_alphabet.and_then(|a| a.sanitized(read_only));
            self.on_number = self.on_number.and_then(|a| a.sanitized(read_only));
            self.on_alphanumeric =
                self.on_alphanumeric.and_then(|a| a.sanitized(read_only));
            self.on_special_character = self
                .on_special_character
                .and_then(|a| a.sanitized(read_only));
            self.on_character = self.on_character.and_then(|a| a.sanitized(read_only));
            self.on_navigation = self.on_navigation.and_then(|a| a.sanitized(read_only));
            self.on_function = self.on_function.and_then(|a| a.sanitized(read_only));
            self.default = self.default.and_then(|a| a.sanitized(read_only));
        };
        self
    }

    pub fn extend(mut self, other: Self) -> Self {
        self.on_key.extend(other.on_key);
        self.on_alphabet = other.on_alphabet.or(self.on_alphabet);
        self.on_number = other.on_number.or(self.on_number);
        self.on_alphanumeric = other.on_alphanumeric.or(self.on_alphanumeric);
        self.on_special_character =
            other.on_special_character.or(self.on_special_character);
        self.on_character = other.on_character.or(self.on_character);
        self.on_navigation = other.on_navigation.or(self.on_navigation);
        self.on_function = other.on_function.or(self.on_function);
        self.default = other.default.or(self.default);
        self
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
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

    #[serde(default)]
    pub prompt: Option<String>,
}

impl Mode {
    pub fn sanitized(
        mut self,
        read_only: bool,
        global_key_bindings: KeyBindings,
    ) -> Self {
        self.key_bindings = global_key_bindings
            .sanitized(read_only)
            .extend(self.key_bindings.sanitized(read_only));
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
                    .chain(self.key_bindings.on_key.iter().filter_map(|(k, a)| {
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
                        a.help
                            .clone()
                            .map(|h| HelpMenuLine::KeyMap(k.into(), remaps, h))
                    }))
                    .chain(
                        self.key_bindings
                            .on_alphabet
                            .iter()
                            .map(|a| ("[a-Z]", a.help.clone()))
                            .filter_map(|(k, mh)| {
                                mh.map(|h| HelpMenuLine::KeyMap(k.into(), vec![], h))
                            }),
                    )
                    .chain(
                        self.key_bindings
                            .on_number
                            .iter()
                            .map(|a| ("[0-9]", a.help.clone()))
                            .filter_map(|(k, mh)| {
                                mh.map(|h| HelpMenuLine::KeyMap(k.into(), vec![], h))
                            }),
                    )
                    .chain(
                        self.key_bindings
                            .on_alphanumeric
                            .iter()
                            .map(|a| ("[0-Z]", a.help.clone()))
                            .filter_map(|(k, mh)| {
                                mh.map(|h| HelpMenuLine::KeyMap(k.into(), vec![], h))
                            }),
                    )
                    .chain(
                        self.key_bindings
                            .on_special_character
                            .iter()
                            .map(|a| ("[^0-Z]", a.help.clone()))
                            .filter_map(|(k, mh)| {
                                mh.map(|h| HelpMenuLine::KeyMap(k.into(), vec![], h))
                            }),
                    )
                    .chain(
                        self.key_bindings
                            .on_character
                            .iter()
                            .map(|a| ("[*]", a.help.clone()))
                            .filter_map(|(k, mh)| {
                                mh.map(|h| HelpMenuLine::KeyMap(k.into(), vec![], h))
                            }),
                    )
                    .chain(
                        self.key_bindings
                            .on_navigation
                            .iter()
                            .map(|a| ("[nav]", a.help.clone()))
                            .filter_map(|(k, mh)| {
                                mh.map(|h| HelpMenuLine::KeyMap(k.into(), vec![], h))
                            }),
                    )
                    .chain(
                        self.key_bindings
                            .on_function
                            .iter()
                            .map(|a| ("[f1-f12]", a.help.clone()))
                            .filter_map(|(k, mh)| {
                                mh.map(|h| HelpMenuLine::KeyMap(k.into(), vec![], h))
                            }),
                    )
                    .chain(
                        self.key_bindings
                            .default
                            .iter()
                            .map(|a| ("[default]", a.help.clone()))
                            .filter_map(|(k, mh)| {
                                mh.map(|h| HelpMenuLine::KeyMap(k.into(), vec![], h))
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
pub struct ModesConfig {
    #[serde(default)]
    pub builtin: HashMap<String, Mode>,

    #[serde(default)]
    pub custom: HashMap<String, Mode>,
}

impl ModesConfig {
    pub fn get(&self, name: &str) -> Option<&Mode> {
        self.builtin.get(name).or_else(|| self.custom.get(name))
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct PanelUiConfig {
    #[serde(default)]
    pub title: UiElement,

    #[serde(default)]
    pub style: Style,

    #[serde(default)]
    pub borders: Option<IndexSet<Border>>,

    #[serde(default)]
    pub border_type: Option<BorderType>,

    #[serde(default)]
    pub border_style: Style,
}

impl PanelUiConfig {
    pub fn extend(mut self, other: &Self) -> Self {
        self.title = self.title.extend(&other.title);
        self.style = self.style.extend(&other.style);
        self.borders = other.borders.clone().or(self.borders);
        self.border_type = other.border_type.or(self.border_type);
        self.border_style = self.border_style.extend(&other.border_style);
        self
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LayoutsConfig {
    #[serde(default)]
    pub builtin: HashMap<String, Layout>,

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
    pub general: GeneralConfig,

    #[serde(default)]
    pub node_types: NodeTypesConfig,

    #[serde(default)]
    pub layouts: LayoutsConfig,

    #[serde(default)]
    pub modes: ModesConfig,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Hooks {
    #[serde(default)]
    pub on_load: Vec<ExternalMsg>,

    #[serde(default)]
    pub on_directory_change: Vec<ExternalMsg>,

    #[serde(default)]
    pub on_focus_change: Vec<ExternalMsg>,

    #[serde(default)]
    pub on_mode_switch: Vec<ExternalMsg>,

    #[serde(default)]
    pub on_layout_switch: Vec<ExternalMsg>,

    #[serde(default)]
    pub on_selection_change: Vec<ExternalMsg>,
    // TODO After cleanup or Runner::run
    // #[serde(default)]
    // pub before_quit: Vec<ExternalMsg>,
}

impl Hooks {
    pub fn extend(mut self, other: Self) -> Self {
        self.on_load.extend(other.on_load);
        self.on_directory_change.extend(other.on_directory_change);
        self.on_focus_change.extend(other.on_focus_change);
        self.on_mode_switch.extend(other.on_mode_switch);
        self.on_layout_switch.extend(other.on_layout_switch);
        self.on_selection_change.extend(other.on_selection_change);
        self
    }
}
