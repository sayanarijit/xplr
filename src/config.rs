use crate::app::ExternalMsg;
use crate::app::HelpMenuLine;
use crate::default_config;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::collections::HashMap;
use tui::layout::Constraint as TuiConstraint;
use tui::style::Style as TuiStyle;
use tui::style::{Color, Modifier};

#[derive(Debug, Copy, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Style {
    fg: Option<Color>,
    bg: Option<Color>,
    add_modifier: Option<Modifier>,
    sub_modifier: Option<Modifier>,
}

impl Style {
    pub fn extend(mut self, other: Self) -> Self {
        self.fg = other.fg.or(self.fg);
        self.bg = other.bg.or(self.bg);
        self.add_modifier = other.add_modifier.or(self.add_modifier);
        self.sub_modifier = other.sub_modifier.or(self.sub_modifier);
        self
    }
}

impl From<TuiStyle> for Style {
    fn from(s: TuiStyle) -> Self {
        Self {
            fg: s.fg,
            bg: s.bg,
            add_modifier: Some(s.add_modifier),
            sub_modifier: Some(s.sub_modifier),
        }
    }
}

impl Into<TuiStyle> for Style {
    fn into(self) -> TuiStyle {
        TuiStyle {
            fg: self.fg,
            bg: self.bg,
            add_modifier: self.add_modifier.unwrap_or_else(Modifier::empty),
            sub_modifier: self.sub_modifier.unwrap_or_else(Modifier::empty),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Action {
    #[serde(default)]
    pub help: Option<String>,

    #[serde(default)]
    pub messages: Vec<ExternalMsg>,
}

impl Action {
    pub fn extend(mut self, other: Self) -> Self {
        self.help = other.help.or(self.help);
        self.messages = other.messages;
        self
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
    pub fn extend(mut self, mut other: Self) -> Self {
        self.style = other.style.extend(self.style);
        other.meta.extend(self.meta);
        self.meta = other.meta;
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
    pub mime_essence: HashMap<String, NodeTypeConfig>,

    #[serde(default)]
    pub extension: HashMap<String, NodeTypeConfig>,

    #[serde(default)]
    pub special: HashMap<String, NodeTypeConfig>,
}

impl NodeTypesConfig {
    fn extend(mut self, mut other: Self) -> Self {
        self.directory = other.directory.extend(self.directory);
        self.file = other.file.extend(self.file);
        self.symlink = other.symlink.extend(self.symlink);

        other.mime_essence.extend(self.mime_essence);
        self.mime_essence = other.mime_essence;

        other.extension.extend(self.extension);
        self.extension = other.extension;

        other.special.extend(self.special);
        self.special = other.special;

        self
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
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
    fn extend(mut self, other: Self) -> Self {
        self.prefix = other.prefix.or(self.prefix);
        self.suffix = other.suffix.or(self.suffix);
        self.style = other.style.extend(self.style);
        self
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UiElement {
    #[serde(default)]
    pub format: Option<String>,

    #[serde(default)]
    pub style: Style,
}

impl UiElement {
    fn extend(mut self, other: Self) -> Self {
        self.format = other.format.or(self.format);
        self.style = other.style.extend(self.style);
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

impl TableRowConfig {
    fn extend(mut self, other: Self) -> Self {
        self.cols = other.cols.or(self.cols);
        self.style = other.style.extend(self.style);
        self.height = other.height.or(self.height);
        self
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
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

impl Into<TuiConstraint> for Constraint {
    fn into(self) -> TuiConstraint {
        match self {
            Self::Length(n) => TuiConstraint::Length(n),
            Self::Percentage(n) => TuiConstraint::Percentage(n),
            Self::Ratio(x, y) => TuiConstraint::Ratio(x, y),
            Self::Max(n) => TuiConstraint::Max(n),
            Self::Min(n) => TuiConstraint::Min(n),
        }
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
    pub fn extend(mut self, other: Self) -> Self {
        self.header = other.header.extend(self.header);
        self.row = other.row.extend(self.row);
        self.style = other.style.extend(self.style);
        self.tree = other.tree.or(self.tree);
        self.col_spacing = other.col_spacing.or(self.col_spacing);
        self.col_widths = other.col_widths.or(self.col_widths);
        self
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GeneralConfig {
    #[serde(default)]
    pub show_hidden: Option<bool>,

    #[serde(default)]
    pub cursor: UiElement,

    #[serde(default)]
    pub prompt: UiElement,

    #[serde(default)]
    pub table: TableConfig,

    #[serde(default)]
    pub default_ui: UiConfig,

    #[serde(default)]
    pub focus_ui: UiConfig,

    #[serde(default)]
    pub selection_ui: UiConfig,
}

impl GeneralConfig {
    pub fn extend(mut self, other: Self) -> Self {
        self.show_hidden = other.show_hidden.or(self.show_hidden);
        self.cursor = other.cursor.extend(self.cursor);
        self.prompt = other.prompt.extend(self.prompt);
        self.table = other.table.extend(self.table);
        self.default_ui = other.default_ui.extend(self.default_ui);
        self.focus_ui = other.focus_ui.extend(self.focus_ui);
        self.selection_ui = other.selection_ui.extend(self.selection_ui);
        self
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct KeyBindings {
    #[serde(default)]
    pub remaps: BTreeMap<String, String>,

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
    pub fn extend(mut self, mut other: Self) -> Self {
        other.remaps.extend(self.remaps);
        self.remaps = other.remaps;
        other.on_key.extend(self.on_key);
        self.on_key = other.on_key;
        self.on_alphabet = other.on_alphabet.or(self.on_alphabet);
        self.on_number = other.on_number.or(self.on_number);
        self.on_special_character = other.on_special_character.or(self.on_special_character);
        self.default = other.default.or(self.default);
        self
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
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
}

impl Mode {
    pub fn extend(mut self, other: Self) -> Self {
        self.help = other.help.or(self.help);
        self.extra_help = other.extra_help.or(self.extra_help);
        self.key_bindings = other.key_bindings.extend(self.key_bindings);
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
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BuiltinModesConfig {
    #[serde(default)]
    pub default: Mode,

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
}

impl BuiltinModesConfig {
    pub fn extend(mut self, other: Self) -> Self {
        self.default = other.default.extend(self.default);
        self.selection_ops = other.selection_ops.extend(self.selection_ops);
        self.go_to = other.go_to.extend(self.go_to);
        self.create = other.create.extend(self.create);
        self.create_file = other.create_file.extend(self.create_file);
        self.create_directory = other.create_directory.extend(self.create_directory);
        self.rename = other.rename.extend(self.rename);
        self.delete = other.delete.extend(self.delete);
        self.number = other.number.extend(self.number);
        self.action = other.action.extend(self.action);
        self.search = other.search.extend(self.search);
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

    pub fn extend(mut self, mut other: Self) -> Self {
        self.builtin = other.builtin.extend(self.builtin);
        other.custom.extend(self.custom);
        self.custom = other.custom;
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub version: String,

    #[serde(default)]
    pub general: GeneralConfig,

    #[serde(default)]
    pub node_types: NodeTypesConfig,

    #[serde(default)]
    pub modes: ModesConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            version: default_config::version(),
            general: default_config::general(),
            node_types: default_config::node_types(),
            modes: default_config::modes(),
        }
    }
}

impl Config {
    pub fn extended(mut self) -> Self {
        let default = Self::default();
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
            (0, 4, 2) => true,
            (0, 4, 1) => true,
            (0, 4, 0) => true,
            (_, _, _) => false,
        };

        Ok(result)
    }

    pub fn upgrade_notification(&self) -> Result<Option<&str>> {
        let result = match self.parsed_version()? {
            (0, 4, 2) => None,
            (_, _, _) => Some("New version available"),
        };

        Ok(result)
    }
}
