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
            if self.messages.iter().all(|m| m.is_read_only()) {
                Some(self)
            } else {
                None
            }
        } else {
            Some(self)
        }
    }

    /// Get a reference to the action's help.
    pub fn help(&self) -> &Option<String> {
        &self.help
    }

    /// Get a reference to the action's messages.
    pub fn messages(&self) -> &Vec<ExternalMsg> {
        &self.messages
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
    /// Get a reference to the node type config's style.
    pub fn style(&self) -> &Style {
        &self.style
    }

    /// Get a reference to the node type config's meta.
    pub fn meta(&self) -> &HashMap<String, String> {
        &self.meta
    }

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

impl NodeTypesConfig {
    /// Get a reference to the node types config's directory.
    pub fn directory(&self) -> &NodeTypeConfig {
        &self.directory
    }

    /// Get a reference to the node types config's file.
    pub fn file(&self) -> &NodeTypeConfig {
        &self.file
    }

    /// Get a reference to the node types config's symlink.
    pub fn symlink(&self) -> &NodeTypeConfig {
        &self.symlink
    }

    /// Get a reference to the node types config's mime essence.
    pub fn mime_essence(&self) -> &HashMap<String, HashMap<String, NodeTypeConfig>> {
        &self.mime_essence
    }

    /// Get a reference to the node types config's extension.
    pub fn extension(&self) -> &HashMap<String, NodeTypeConfig> {
        &self.extension
    }

    /// Get a reference to the node types config's special.
    pub fn special(&self) -> &HashMap<String, NodeTypeConfig> {
        &self.special
    }
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

impl UiConfig {
    /// Get a reference to the ui config's prefix.
    pub fn prefix(&self) -> &Option<String> {
        &self.prefix
    }

    /// Get a reference to the ui config's suffix.
    pub fn suffix(&self) -> &Option<String> {
        &self.suffix
    }

    /// Get a reference to the ui config's style.
    pub fn style(&self) -> &Style {
        &self.style
    }
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

    /// Get a reference to the ui element's format.
    pub fn format(&self) -> &Option<String> {
        &self.format
    }

    /// Get a reference to the ui element's style.
    pub fn style(&self) -> &Style {
        &self.style
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

impl TableRowConfig {
    /// Get a reference to the table row config's cols.
    pub fn cols(&self) -> &Option<Vec<UiElement>> {
        &self.cols
    }

    /// Get a reference to the table row config's style.
    pub fn style(&self) -> &Style {
        &self.style
    }

    /// Get a reference to the table row config's height.
    pub fn height(&self) -> Option<u16> {
        self.height
    }
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

impl TableConfig {
    /// Get a reference to the table config's header.
    pub fn header(&self) -> &TableRowConfig {
        &self.header
    }

    /// Get a reference to the table config's row.
    pub fn row(&self) -> &TableRowConfig {
        &self.row
    }

    /// Get a reference to the table config's style.
    pub fn style(&self) -> &Style {
        &self.style
    }

    /// Get a reference to the table config's tree.
    pub fn tree(&self) -> &Option<(UiElement, UiElement, UiElement)> {
        &self.tree
    }

    /// Get a reference to the table config's col spacing.
    pub fn col_spacing(&self) -> Option<u16> {
        self.col_spacing
    }

    /// Get a reference to the table config's col widths.
    pub fn col_widths(&self) -> &Option<Vec<Constraint>> {
        &self.col_widths
    }
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

impl LogsConfig {
    /// Get a reference to the logs config's info.
    pub fn info(&self) -> &UiElement {
        &self.info
    }

    /// Get a reference to the logs config's success.
    pub fn success(&self) -> &UiElement {
        &self.success
    }

    /// Get a reference to the logs config's error.
    pub fn error(&self) -> &UiElement {
        &self.error
    }

    /// Get a reference to the logs config's warning.
    pub fn warning(&self) -> &UiElement {
        &self.warning
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SortDirectionIdentifiersUi {
    #[serde(default)]
    pub forward: UiElement,

    #[serde(default)]
    pub reverse: UiElement,
}

impl SortDirectionIdentifiersUi {
    /// Get a reference to the sort direction identifiers ui's forward.
    pub fn forward(&self) -> &UiElement {
        &self.forward
    }

    /// Get a reference to the sort direction identifiers ui's reverse.
    pub fn reverse(&self) -> &UiElement {
        &self.reverse
    }
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

impl SortAndFilterUi {
    /// Get a reference to the sort and filter ui's separator.
    pub fn separator(&self) -> &UiElement {
        &self.separator
    }

    /// Get a reference to the sort and filter ui's sort direction identifiers.
    pub fn sort_direction_identifiers(&self) -> &SortDirectionIdentifiersUi {
        &self.sort_direction_identifiers
    }

    /// Get a reference to the sort and filter ui's sorter identifiers.
    pub fn sorter_identifiers(&self) -> &HashMap<NodeSorter, UiElement> {
        &self.sorter_identifiers
    }

    /// Get a reference to the sort and filter ui's filter identifiers.
    pub fn filter_identifiers(&self) -> &HashMap<NodeFilter, UiElement> {
        &self.filter_identifiers
    }

    /// Get a reference to the sort and filter ui's default identifier.
    pub fn default_identifier(&self) -> &UiElement {
        &self.default_identifier
    }
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

impl PanelUi {
    /// Get a reference to the panel ui's default.
    pub fn default(&self) -> &PanelUiConfig {
        &self.default
    }

    /// Get a reference to the panel ui's table.
    pub fn table(&self) -> &PanelUiConfig {
        &self.table
    }

    /// Get a reference to the panel ui's sort and filter.
    pub fn sort_and_filter(&self) -> &PanelUiConfig {
        &self.sort_and_filter
    }

    /// Get a reference to the panel ui's selection.
    pub fn selection(&self) -> &PanelUiConfig {
        &self.selection
    }

    /// Get a reference to the panel ui's input and log.
    pub fn input_and_logs(&self) -> &PanelUiConfig {
        &self.input_and_logs
    }

    /// Get a reference to the panel ui's help menu.
    pub fn help_menu(&self) -> &PanelUiConfig {
        &self.help_menu
    }
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
    pub cursor: UiElement,

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

impl GeneralConfig {
    /// Get a reference to the general config's show hidden.
    pub fn show_hidden(&self) -> bool {
        self.show_hidden
    }

    /// Get a reference to the general config's read only.
    pub fn read_only(&self) -> bool {
        self.read_only
    }

    /// Get a reference to the general config's cursor.
    pub fn cursor(&self) -> &UiElement {
        &self.cursor
    }

    /// Get a reference to the general config's prompt.
    pub fn prompt(&self) -> &UiElement {
        &self.prompt
    }

    /// Get a reference to the general config's logs.
    pub fn logs(&self) -> &LogsConfig {
        &self.logs
    }

    /// Get a reference to the general config's table.
    pub fn table(&self) -> &TableConfig {
        &self.table
    }

    /// Get a reference to the general config's default ui.
    pub fn default_ui(&self) -> &UiConfig {
        &self.default_ui
    }

    /// Get a reference to the general config's focus ui.
    pub fn focus_ui(&self) -> &UiConfig {
        &self.focus_ui
    }

    /// Get a reference to the general config's selection ui.
    pub fn selection_ui(&self) -> &UiConfig {
        &self.selection_ui
    }

    /// Get a reference to the general config's sort and filter ui.
    pub fn sort_and_filter_ui(&self) -> &SortAndFilterUi {
        &self.sort_and_filter_ui
    }

    /// Get a reference to the general config's initial sorting.
    pub fn initial_sorting(&self) -> &Option<IndexSet<NodeSorterApplicable>> {
        &self.initial_sorting
    }

    /// Get a reference to the general config's initial mode.
    pub fn initial_mode(&self) -> &Option<String> {
        &self.initial_mode
    }

    /// Get a reference to the general config's initial layout.
    pub fn initial_layout(&self) -> &Option<String> {
        &self.initial_layout
    }

    /// Get a reference to the general config's panel ui.
    pub fn panel_ui(&self) -> &PanelUi {
        &self.panel_ui
    }

    /// Get a reference to the general config's enable mouse.
    pub fn enable_mouse(&self) -> bool {
        self.enable_mouse
    }

    /// Get a reference to the general config's enable recover mode.
    pub fn enable_recover_mode(&self) -> bool {
        self.enable_recover_mode
    }

    /// Set the general config's read only.
    pub fn set_read_only(&mut self, read_only: bool) {
        self.read_only = read_only;
    }

    /// Get a reference to the general config's start fifo.
    pub fn start_fifo(&self) -> Option<&String> {
        self.start_fifo.as_ref()
    }

    /// Get a reference to the general config's focus selection ui.
    pub fn focus_selection_ui(&self) -> &UiConfig {
        &self.focus_selection_ui
    }
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
    pub on_special_character: Option<Action>,

    #[serde(default)]
    pub default: Option<Action>,
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
            self.on_special_character = self
                .on_special_character
                .and_then(|a| a.sanitized(read_only));
            self.default = self.default.and_then(|a| a.sanitized(read_only));
        };

        self
    }

    /// Get a reference to the key bindings's on key.
    pub fn on_key(&self) -> &BTreeMap<String, Action> {
        &self.on_key
    }

    /// Get a reference to the key bindings's on alphabet.
    pub fn on_alphabet(&self) -> &Option<Action> {
        &self.on_alphabet
    }

    /// Get a reference to the key bindings's on number.
    pub fn on_number(&self) -> &Option<Action> {
        &self.on_number
    }

    /// Get a reference to the key bindings's on special character.
    pub fn on_special_character(&self) -> &Option<Action> {
        &self.on_special_character
    }

    /// Get a reference to the key bindings's default.
    pub fn default(&self) -> &Option<Action> {
        &self.default
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
                            .on_special_character
                            .iter()
                            .map(|a| ("[spcl chars]", a.help.clone()))
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
                        HelpMenuLine::Paragraph(p) => result.push(HelpMenuLine::Paragraph(p)),
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

    /// Get a reference to the mode's name.
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Get a reference to the mode's help.
    pub fn help(&self) -> &Option<String> {
        &self.help
    }

    /// Get a reference to the mode's extra help.
    pub fn extra_help(&self) -> &Option<String> {
        &self.extra_help
    }

    /// Get a reference to the mode's key bindings.
    pub fn key_bindings(&self) -> &KeyBindings {
        &self.key_bindings
    }

    /// Get a reference to the mode's layout.
    pub fn layout(&self) -> &Option<Layout> {
        &self.layout
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
            "relative_path_does_contain" => Some(&self.relative_path_does_contain),
            "relative path does contain" => Some(&self.relative_path_does_contain),
            "relative_path_does_not_contain" => Some(&self.relative_path_does_not_contain),
            "relative path does not contain" => Some(&self.relative_path_does_not_contain),
            "switch layout" => Some(&self.switch_layout),
            "switch_layout" => Some(&self.switch_layout),
            "quit" => Some(&self.quit),
            _ => None,
        }
    }

    /// Get a reference to the builtin modes config's default.
    pub fn default(&self) -> &Mode {
        &self.default
    }

    /// Get a reference to the builtin modes config's selection ops.
    pub fn selection_ops(&self) -> &Mode {
        &self.selection_ops
    }

    /// Get a reference to the builtin modes config's create.
    pub fn create(&self) -> &Mode {
        &self.create
    }

    /// Get a reference to the builtin modes config's create directory.
    pub fn create_directory(&self) -> &Mode {
        &self.create_directory
    }

    /// Get a reference to the builtin modes config's create file.
    pub fn create_file(&self) -> &Mode {
        &self.create_file
    }

    /// Get a reference to the builtin modes config's number.
    pub fn number(&self) -> &Mode {
        &self.number
    }

    /// Get a reference to the builtin modes config's go to.
    pub fn go_to(&self) -> &Mode {
        &self.go_to
    }

    /// Get a reference to the builtin modes config's rename.
    pub fn rename(&self) -> &Mode {
        &self.rename
    }

    /// Get a reference to the builtin modes config's delete.
    pub fn delete(&self) -> &Mode {
        &self.delete
    }

    /// Get a reference to the builtin modes config's action.
    pub fn action(&self) -> &Mode {
        &self.action
    }

    /// Get a reference to the builtin modes config's search.
    pub fn search(&self) -> &Mode {
        &self.search
    }

    /// Get a reference to the builtin modes config's filter.
    pub fn filter(&self) -> &Mode {
        &self.filter
    }

    /// Get a reference to the builtin modes config's relative path does contain.
    pub fn relative_path_does_contain(&self) -> &Mode {
        &self.relative_path_does_contain
    }

    /// Get a reference to the builtin modes config's relative path does not contain.
    pub fn relative_path_does_not_contain(&self) -> &Mode {
        &self.relative_path_does_not_contain
    }

    /// Get a reference to the builtin modes config's sort.
    pub fn sort(&self) -> &Mode {
        &self.sort
    }

    /// Get a reference to the builtin modes config's switch layout.
    pub fn switch_layout(&self) -> &Mode {
        &self.switch_layout
    }

    /// Get a reference to the builtin modes config's recover.
    pub fn recover(&self) -> &Mode {
        &self.recover
    }

    /// Get a reference to the builtin modes config's quit.
    pub fn quit(&self) -> &Mode {
        &self.quit
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
    pub fn get_builtin(&self, name: &str) -> Option<&Mode> {
        self.builtin.get(name)
    }

    pub fn get_custom(&self, name: &str) -> Option<&Mode> {
        self.custom.get(name)
    }

    pub fn get(&self, name: &str) -> Option<&Mode> {
        self.get_builtin(name).or_else(|| self.get_custom(name))
    }

    /// Get a reference to the modes config's builtin.
    pub fn builtin(&self) -> &BuiltinModesConfig {
        &self.builtin
    }

    /// Get a reference to the modes config's custom.
    pub fn custom(&self) -> &HashMap<String, Mode> {
        &self.custom
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
    /// Get a reference to the block config's borders.
    pub fn borders(&self) -> &Option<IndexSet<Border>> {
        &self.borders
    }

    /// Get a reference to the block config's title.
    pub fn title(&self) -> &UiElement {
        &self.title
    }

    /// Get a reference to the block config's style.
    pub fn style(&self) -> &Style {
        &self.style
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

    /// Get a reference to the builtin layouts config's default.
    pub fn default(&self) -> &Layout {
        &self.default
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

    /// Get a reference to the layouts config's builtin.
    pub fn builtin(&self) -> &BuiltinLayoutsConfig {
        &self.builtin
    }

    /// Get a reference to the layouts config's custom.
    pub fn custom(&self) -> &HashMap<String, Layout> {
        &self.custom
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

impl Config {
    /// Get a reference to the config's layouts.
    pub fn layouts(&self) -> &LayoutsConfig {
        &self.layouts
    }

    /// Get a reference to the config's general.
    pub fn general(&self) -> &GeneralConfig {
        &self.general
    }

    /// Get a reference to the config's node types.
    pub fn node_types(&self) -> &NodeTypesConfig {
        &self.node_types
    }

    /// Get a reference to the config's modes.
    pub fn modes(&self) -> &ModesConfig {
        &self.modes
    }
}
