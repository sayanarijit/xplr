use crate::app::ExternalMsg;
use crate::app::HelpMenuLine;
use crate::app::NodeFilter;
use crate::app::NodeSorter;
use crate::app::NodeSorterApplicable;
use crate::default_config;
use crate::ui::Border;
use crate::ui::Style;
use anyhow::Result;
use indexmap::IndexSet;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::collections::HashMap;
use tui::layout::Constraint as TuiConstraint;
use tui::layout::Rect;

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Action {
    #[serde(default)]
    help: Option<String>,

    #[serde(default)]
    messages: Vec<ExternalMsg>,
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

    pub fn extend(mut self, other: Self) -> Self {
        self.help = other.help.or(self.help);
        self.messages = other.messages;
        self
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
    style: Style,

    #[serde(default)]
    meta: HashMap<String, String>,
}

impl NodeTypeConfig {
    pub fn extend(mut self, other: Self) -> Self {
        self.style = self.style.extend(other.style);
        self.meta.extend(other.meta);
        self
    }

    /// Get a reference to the node type config's style.
    pub fn style(&self) -> &Style {
        &self.style
    }

    /// Get a reference to the node type config's meta.
    pub fn meta(&self) -> &HashMap<String, String> {
        &self.meta
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NodeTypesConfig {
    #[serde(default)]
    directory: NodeTypeConfig,

    #[serde(default)]
    file: NodeTypeConfig,

    #[serde(default)]
    symlink: NodeTypeConfig,

    #[serde(default)]
    mime_essence: HashMap<String, NodeTypeConfig>,

    #[serde(default)]
    extension: HashMap<String, NodeTypeConfig>,

    #[serde(default)]
    special: HashMap<String, NodeTypeConfig>,
}

impl NodeTypesConfig {
    fn extend(mut self, other: Self) -> Self {
        self.directory = self.directory.extend(other.directory);
        self.file = self.file.extend(other.file);
        self.symlink = self.symlink.extend(other.symlink);
        self.mime_essence.extend(other.mime_essence);
        self.extension.extend(other.extension);
        self.special.extend(other.special);
        self
    }

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
    pub fn mime_essence(&self) -> &HashMap<String, NodeTypeConfig> {
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
    pub(crate) prefix: Option<String>,

    #[serde(default)]
    pub(crate) suffix: Option<String>,

    #[serde(default)]
    pub(crate) style: Style,
}

impl UiConfig {
    pub fn extend(mut self, other: Self) -> Self {
        self.prefix = other.prefix.or(self.prefix);
        self.suffix = other.suffix.or(self.suffix);
        self.style = self.style.extend(other.style);
        self
    }

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

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UiElement {
    #[serde(default)]
    format: Option<String>,

    #[serde(default)]
    style: Style,
}

impl UiElement {
    pub fn extend(mut self, other: Self) -> Self {
        self.format = other.format.or(self.format);
        self.style = self.style.extend(other.style);
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
    cols: Option<Vec<UiElement>>,

    #[serde(default)]
    style: Style,

    #[serde(default)]
    height: Option<u16>,
}

impl TableRowConfig {
    fn extend(mut self, other: Self) -> Self {
        self.cols = other.cols.or(self.cols);
        self.style = self.style.extend(other.style);
        self.height = other.height.or(self.height);
        self
    }

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

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum Constraint {
    Percentage(u16),
    Ratio(u32, u32),
    Length(u16),
    LengthLessThanScreenHeight(u16),
    LengthLessThanScreenWidth(u16),
    LengthLessThanLayoutHeight(u16),
    LengthLessThanLayoutWidth(u16),
    Max(u16),
    MaxLessThanScreenHeight(u16),
    MaxLessThanScreenWidth(u16),
    MaxLessThanLayoutHeight(u16),
    MaxthLessThanLayoutWidth(u16),
    Min(u16),
    MinLessThanScreenHeight(u16),
    MinLessThanScreenWidth(u16),
    MinLessThanLayoutHeight(u16),
    MinLessThanLayoutWidth(u16),
}

impl Constraint {
    pub fn to_tui(self, screen_size: Rect, layout_size: Rect) -> TuiConstraint {
        match self {
            Self::Percentage(n) => TuiConstraint::Percentage(n),
            Self::Ratio(x, y) => TuiConstraint::Ratio(x, y),
            Self::Length(n) => TuiConstraint::Length(n),
            Self::LengthLessThanScreenHeight(n) => {
                TuiConstraint::Length(screen_size.height.max(n) - n)
            }
            Self::LengthLessThanScreenWidth(n) => {
                TuiConstraint::Length(screen_size.width.max(n) - n)
            }
            Self::LengthLessThanLayoutHeight(n) => {
                TuiConstraint::Length(layout_size.height.max(n) - n)
            }
            Self::LengthLessThanLayoutWidth(n) => {
                TuiConstraint::Length(layout_size.width.max(n) - n)
            }
            Self::Max(n) => TuiConstraint::Max(n),
            Self::MaxLessThanScreenHeight(n) => TuiConstraint::Max(screen_size.height.max(n) - n),
            Self::MaxLessThanScreenWidth(n) => TuiConstraint::Max(screen_size.width.max(n) - n),
            Self::MaxLessThanLayoutHeight(n) => TuiConstraint::Max(layout_size.height.max(n) - n),
            Self::MaxthLessThanLayoutWidth(n) => TuiConstraint::Max(layout_size.width.max(n) - n),
            Self::Min(n) => TuiConstraint::Min(n),
            Self::MinLessThanScreenHeight(n) => TuiConstraint::Min(screen_size.height.max(n) - n),
            Self::MinLessThanScreenWidth(n) => TuiConstraint::Min(screen_size.width.max(n) - n),
            Self::MinLessThanLayoutHeight(n) => TuiConstraint::Min(layout_size.height.max(n) - n),
            Self::MinLessThanLayoutWidth(n) => TuiConstraint::Min(layout_size.width.max(n) - n),
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TableConfig {
    #[serde(default)]
    header: TableRowConfig,

    #[serde(default)]
    row: TableRowConfig,

    #[serde(default)]
    style: Style,

    #[serde(default)]
    tree: Option<(UiElement, UiElement, UiElement)>,

    #[serde(default)]
    col_spacing: Option<u16>,

    #[serde(default)]
    col_widths: Option<Vec<Constraint>>,
}

impl TableConfig {
    pub fn extend(mut self, other: Self) -> Self {
        self.header = self.header.extend(other.header);
        self.row = self.row.extend(other.row);
        self.style = self.style.extend(other.style);
        self.tree = other.tree.or(self.tree);
        self.col_spacing = other.col_spacing.or(self.col_spacing);
        self.col_widths = other.col_widths.or(self.col_widths);
        self
    }

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
    info: UiElement,

    #[serde(default)]
    success: UiElement,

    #[serde(default)]
    error: UiElement,
}

impl LogsConfig {
    pub fn extend(mut self, other: Self) -> Self {
        self.info = self.info.extend(other.info);
        self.success = self.success.extend(other.success);
        self.error = self.error.extend(other.error);
        self
    }

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
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SortDirectionIdentifiersUi {
    #[serde(default)]
    forward: UiElement,

    #[serde(default)]
    reverse: UiElement,
}

impl SortDirectionIdentifiersUi {
    pub fn extend(mut self, other: Self) -> Self {
        self.forward = self.forward.extend(other.forward);
        self.reverse = self.reverse.extend(other.reverse);
        self
    }

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
    separator: UiElement,

    #[serde(default)]
    default_identifier: UiElement,

    #[serde(default)]
    sort_direction_identifiers: SortDirectionIdentifiersUi,

    #[serde(default)]
    sorter_identifiers: HashMap<NodeSorter, UiElement>,

    #[serde(default)]
    filter_identifiers: HashMap<NodeFilter, UiElement>,
}

impl SortAndFilterUi {
    pub fn extend(mut self, other: Self) -> Self {
        self.separator = self.separator.extend(other.separator);
        self.default_identifier = self.default_identifier.extend(other.default_identifier);
        self.sort_direction_identifiers = self
            .sort_direction_identifiers
            .extend(other.sort_direction_identifiers);
        self.sorter_identifiers.extend(other.sorter_identifiers);
        self.filter_identifiers.extend(other.filter_identifiers);
        self
    }

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
    default: PanelUiConfig,

    #[serde(default)]
    table: PanelUiConfig,

    #[serde(default)]
    sort_and_filter: PanelUiConfig,

    #[serde(default)]
    selection: PanelUiConfig,

    #[serde(default)]
    input_and_logs: PanelUiConfig,

    #[serde(default)]
    help_menu: PanelUiConfig,
}

impl PanelUi {
    fn extend(mut self, other: Self) -> Self {
        self.default = self.default.extend(other.default);
        self.table = self.table.extend(other.table);
        self.sort_and_filter = self.sort_and_filter.extend(other.sort_and_filter);
        self.selection = self.selection.extend(other.selection);
        self.input_and_logs = self.input_and_logs.extend(other.input_and_logs);
        self.help_menu = self.help_menu.extend(other.help_menu);
        self
    }

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
    show_hidden: Option<bool>,

    #[serde(default)]
    read_only: Option<bool>,

    #[serde(default)]
    cursor: UiElement,

    #[serde(default)]
    prompt: UiElement,

    #[serde(default)]
    logs: LogsConfig,

    #[serde(default)]
    table: TableConfig,

    #[serde(default)]
    default_ui: UiConfig,

    #[serde(default)]
    focus_ui: UiConfig,

    #[serde(default)]
    selection_ui: UiConfig,

    #[serde(default)]
    sort_and_filter_ui: SortAndFilterUi,

    #[serde(default)]
    panel_ui: PanelUi,

    #[serde(default)]
    initial_sorting: Option<IndexSet<NodeSorterApplicable>>,

    #[serde(default)]
    initial_mode: Option<String>,

    #[serde(default)]
    initial_layout: Option<String>,
}

impl GeneralConfig {
    fn extend(mut self, other: Self) -> Self {
        self.show_hidden = other.show_hidden.or(self.show_hidden);
        self.read_only = other.read_only.or(self.read_only);
        self.cursor = self.cursor.extend(other.cursor);
        self.prompt = self.prompt.extend(other.prompt);
        self.logs = self.logs.extend(other.logs);
        self.table = self.table.extend(other.table);
        self.default_ui = self.default_ui.extend(other.default_ui);
        self.focus_ui = self.focus_ui.extend(other.focus_ui);
        self.selection_ui = self.selection_ui.extend(other.selection_ui);
        self.sort_and_filter_ui = self.sort_and_filter_ui.extend(other.sort_and_filter_ui);
        self.panel_ui = self.panel_ui.extend(other.panel_ui);
        self.initial_sorting = other.initial_sorting.or(self.initial_sorting);
        self.initial_layout = other.initial_layout.or(self.initial_layout);
        self.initial_mode = other.initial_mode.or(self.initial_mode);
        self
    }

    /// Get a reference to the general config's show hidden.
    pub fn show_hidden(&self) -> Option<bool> {
        self.show_hidden
    }

    /// Get a reference to the general config's read only.
    pub fn read_only(&self) -> Option<bool> {
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
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct KeyBindings {
    #[serde(default)]
    remaps: BTreeMap<String, String>,

    #[serde(default)]
    on_key: BTreeMap<String, Action>,

    #[serde(default)]
    on_alphabet: Option<Action>,

    #[serde(default)]
    on_number: Option<Action>,

    #[serde(default)]
    on_special_character: Option<Action>,

    #[serde(default)]
    default: Option<Action>,
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
            self.remaps = self
                .remaps
                .clone()
                .into_iter()
                .filter(|(_, v)| self.on_key.contains_key(v))
                .collect();
        };
        self
    }

    fn extend(mut self, other: Self) -> Self {
        self.remaps.extend(other.remaps);
        self.on_key.extend(other.on_key);
        self.on_alphabet = other.on_alphabet.or(self.on_alphabet);
        self.on_number = other.on_number.or(self.on_number);
        self.on_special_character = other.on_special_character.or(self.on_special_character);
        self.default = other.default.or(self.default);
        self
    }

    /// Get a reference to the key bindings's remaps.
    pub fn remaps(&self) -> &BTreeMap<String, String> {
        &self.remaps
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

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Mode {
    #[serde(default)]
    name: String,

    #[serde(default)]
    help: Option<String>,

    #[serde(default)]
    extra_help: Option<String>,

    #[serde(default)]
    key_bindings: KeyBindings,
}

impl Mode {
    pub fn sanitized(mut self, read_only: bool) -> Self {
        self.key_bindings = self.key_bindings.sanitized(read_only);
        self
    }

    fn extend(mut self, other: Self) -> Self {
        self.help = other.help.or(self.help);
        self.extra_help = other.extra_help.or(self.extra_help);
        self.key_bindings = self.key_bindings.extend(other.key_bindings);
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
                extra_help_lines
                    .unwrap_or_default()
                    .into_iter()
                    .chain(
                        self.key_bindings
                            .on_key
                            .iter()
                            .filter(|(k, _)| !self.key_bindings.remaps.contains_key(&k.to_string()))
                            .filter_map(|(k, a)| {
                                a.help.clone().map(|h| HelpMenuLine::KeyMap(k.into(), h))
                            }),
                    )
                    .chain(
                        self.key_bindings
                            .on_alphabet
                            .iter()
                            .map(|a| ("[a-Z]", a.help.clone()))
                            .filter_map(|(k, mh)| mh.map(|h| HelpMenuLine::KeyMap(k.into(), h))),
                    )
                    .chain(
                        self.key_bindings
                            .on_number
                            .iter()
                            .map(|a| ("[0-9]", a.help.clone()))
                            .filter_map(|(k, mh)| mh.map(|h| HelpMenuLine::KeyMap(k.into(), h))),
                    )
                    .chain(
                        self.key_bindings
                            .on_special_character
                            .iter()
                            .map(|a| ("[spcl chars]", a.help.clone()))
                            .filter_map(|(k, mh)| mh.map(|h| HelpMenuLine::KeyMap(k.into(), h))),
                    )
                    .chain(
                        self.key_bindings
                            .default
                            .iter()
                            .map(|a| ("[default]", a.help.clone()))
                            .filter_map(|(k, mh)| mh.map(|h| HelpMenuLine::KeyMap(k.into(), h))),
                    )
                    .collect()
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
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BuiltinModesConfig {
    #[serde(default)]
    default: Mode,

    #[serde(default)]
    selection_ops: Mode,

    #[serde(default)]
    create: Mode,

    #[serde(default)]
    create_directory: Mode,

    #[serde(default)]
    create_file: Mode,

    #[serde(default)]
    number: Mode,

    #[serde(default)]
    go_to: Mode,

    #[serde(default)]
    rename: Mode,

    #[serde(default)]
    delete: Mode,

    #[serde(default)]
    action: Mode,

    #[serde(default)]
    search: Mode,

    #[serde(default)]
    filter: Mode,

    #[serde(default)]
    relative_path_does_contain: Mode,

    #[serde(default)]
    relative_path_does_not_contain: Mode,

    #[serde(default)]
    sort: Mode,

    #[serde(default)]
    switch_layout: Mode,
}

impl BuiltinModesConfig {
    pub fn extend(mut self, other: Self) -> Self {
        self.default = self.default.extend(other.default);
        self.selection_ops = self.selection_ops.extend(other.selection_ops);
        self.go_to = self.go_to.extend(other.go_to);
        self.create = self.create.extend(other.create);
        self.create_file = self.create_file.extend(other.create_file);
        self.create_directory = self.create_directory.extend(other.create_directory);
        self.rename = self.rename.extend(other.rename);
        self.delete = self.delete.extend(other.delete);
        self.number = self.number.extend(other.number);
        self.action = self.action.extend(other.action);
        self.search = self.search.extend(other.search);
        self.filter = self.filter.extend(other.filter);
        self.relative_path_does_contain = self
            .relative_path_does_contain
            .extend(other.relative_path_does_contain);
        self.relative_path_does_not_contain = self
            .relative_path_does_not_contain
            .extend(other.relative_path_does_not_contain);
        self.sort = self.sort.extend(other.sort);
        self.switch_layout = self.switch_layout.extend(other.switch_layout);
        self
    }

    pub fn get(&self, name: &str) -> Option<&Mode> {
        match name {
            "default" => Some(&self.default),
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
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ModesConfig {
    #[serde(default)]
    builtin: BuiltinModesConfig,

    #[serde(default)]
    custom: HashMap<String, Mode>,
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

    pub fn extend(mut self, other: Self) -> Self {
        self.builtin = self.builtin.extend(other.builtin);
        self.custom.extend(other.custom);
        self
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

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LayoutOptions {
    #[serde(default)]
    margin: Option<u16>,
    #[serde(default)]
    horizontal_margin: Option<u16>,
    #[serde(default)]
    vertical_margin: Option<u16>,
    #[serde(default)]
    constraints: Option<Vec<Constraint>>,
}

impl LayoutOptions {
    pub fn extend(mut self, other: Self) -> Self {
        self.margin = other.margin.or(self.margin);
        self.horizontal_margin = other.horizontal_margin.or(self.horizontal_margin);
        self.vertical_margin = other.vertical_margin.or(self.vertical_margin);
        self.constraints = other.constraints.or(self.constraints);
        self
    }

    /// Get a reference to the layout options's constraints.
    pub fn constraints(&self) -> &Option<Vec<Constraint>> {
        &self.constraints
    }

    /// Get a reference to the layout options's margin.
    pub fn margin(&self) -> Option<u16> {
        self.margin
    }

    /// Get a reference to the layout options's horizontal margin.
    pub fn horizontal_margin(&self) -> Option<u16> {
        self.horizontal_margin
    }

    /// Get a reference to the layout options's vertical margin.
    pub fn vertical_margin(&self) -> Option<u16> {
        self.vertical_margin
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PanelUiConfig {
    #[serde(default)]
    title: UiElement,

    #[serde(default)]
    borders: Option<IndexSet<Border>>,

    #[serde(default)]
    style: Style,
}

impl PanelUiConfig {
    pub fn extend(mut self, other: Self) -> Self {
        self.title = self.title.extend(other.title);
        self.borders = other.borders.or(self.borders);
        self.style = self.style.extend(other.style);
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum Layout {
    Nothing(Option<PanelUiConfig>),
    Table(Option<PanelUiConfig>),
    InputAndLogs(Option<PanelUiConfig>),
    Selection(Option<PanelUiConfig>),
    HelpMenu(Option<PanelUiConfig>),
    SortAndFilter(Option<PanelUiConfig>),
    Horizontal {
        config: LayoutOptions,
        splits: Vec<Layout>,
    },
    Vertical {
        config: LayoutOptions,
        splits: Vec<Layout>,
    },
}

impl Default for Layout {
    fn default() -> Self {
        Self::Nothing(Default::default())
    }
}

impl Layout {
    pub fn extend(self, other: Self) -> Self {
        match (self, other) {
            (s, Self::Nothing(_)) => s,
            (Self::Table(s), Self::Table(o)) => Self::Table(o.or(s)),
            (Self::InputAndLogs(s), Self::InputAndLogs(o)) => Self::InputAndLogs(o.or(s)),
            (Self::Selection(s), Self::Selection(o)) => Self::Selection(o.or(s)),
            (Self::HelpMenu(s), Self::HelpMenu(o)) => Self::HelpMenu(o.or(s)),
            (Self::SortAndFilter(s), Self::SortAndFilter(o)) => Self::SortAndFilter(o.or(s)),
            (
                Self::Horizontal {
                    config: sconfig,
                    splits: _,
                },
                Self::Horizontal {
                    config: oconfig,
                    splits: osplits,
                },
            ) => Self::Horizontal {
                config: sconfig.extend(oconfig),
                splits: osplits,
            },

            (
                Self::Vertical {
                    config: sconfig,
                    splits: _,
                },
                Self::Vertical {
                    config: oconfig,
                    splits: osplits,
                },
            ) => Self::Vertical {
                config: sconfig.extend(oconfig),
                splits: osplits,
            },
            (_, other) => other,
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BuiltinLayoutsConfig {
    #[serde(default)]
    default: Layout,

    #[serde(default)]
    no_help: Layout,

    #[serde(default)]
    no_selection: Layout,

    #[serde(default)]
    no_help_no_selection: Layout,
}

impl BuiltinLayoutsConfig {
    pub fn extend(mut self, other: Self) -> Self {
        self.default = self.default.extend(other.default);
        self.no_help = self.no_help.extend(other.no_help);
        self.no_selection = self.no_selection.extend(other.no_selection);
        self.no_help_no_selection = self.no_help_no_selection.extend(other.no_help_no_selection);
        self
    }

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
    builtin: BuiltinLayoutsConfig,

    #[serde(default)]
    custom: HashMap<String, Layout>,
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

    pub fn extend(mut self, other: Self) -> Self {
        self.builtin = self.builtin.extend(other.builtin);
        self.custom.extend(other.custom);
        self
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    version: String,

    #[serde(default)]
    layouts: LayoutsConfig,

    #[serde(default)]
    general: GeneralConfig,

    #[serde(default)]
    node_types: NodeTypesConfig,

    #[serde(default)]
    modes: ModesConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            version: default_config::version(),
            layouts: default_config::layouts(),
            general: default_config::general(),
            node_types: default_config::node_types(),
            modes: default_config::modes(),
        }
    }
}

impl Config {
    pub fn extended(mut self) -> Self {
        let default = Self::default();
        self.layouts = default.layouts.extend(self.layouts);
        self.general = default.general.extend(self.general);
        self.node_types = default.node_types.extend(self.node_types);
        self.modes = default.modes.extend(self.modes);
        self
    }

    fn parsed_version(&self) -> Result<(u16, u16, u16)> {
        let mut configv = self
            .version
            .strip_prefix('v')
            .unwrap_or_default()
            .split('.');

        let major = configv.next().unwrap_or_default().parse::<u16>()?;
        let minor = configv.next().unwrap_or_default().parse::<u16>()?;
        let bugfix = configv.next().unwrap_or_default().parse::<u16>()?;

        Ok((major, minor, bugfix))
    }

    pub fn is_compatible(&self) -> Result<bool> {
        let result = match self.parsed_version()? {
            (0, 7, 0) => true,
            (_, _, _) => false,
        };

        Ok(result)
    }

    pub fn upgrade_notification(&self) -> Result<Option<&str>> {
        let result = match self.parsed_version()? {
            (_, _, _) => None,
            // (_, _, _) => Some("App version updated. New: added sort and filter support and some hacks: https://github.com/sayanarijit/xplr/wiki/Hacks"),
        };

        Ok(result)
    }

    /// Get a reference to the config's version.
    pub fn version(&self) -> &String {
        &self.version
    }

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

#[cfg(test)]
mod test {

    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_compatibility() {
        let config = Config::default();
        assert!(config.is_compatible().unwrap());
        assert_eq!(config.upgrade_notification().unwrap(), None);
    }

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
}
