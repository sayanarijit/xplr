use crate::app::{HelpMenuLine, NodeFilterApplicable, NodeSorterApplicable};
use crate::app::{Node, ResolvedNode};
use crate::compat::{draw_custom_content, CustomContent};
use crate::config::PanelUiConfig;
use crate::lua;
use crate::permissions::Permissions;
use crate::{app, path};
use ansi_to_tui::IntoText;
use indexmap::IndexSet;
use lazy_static::lazy_static;
use lscolors::{Color as LsColorsColor, Style as LsColorsStyle};
use mlua::Lua;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::env;
use std::ops::BitXor;
use time::macros::format_description;
use tui::layout::Rect as TuiRect;
use tui::layout::{Constraint as TuiConstraint, Direction, Layout as TuiLayout};
use tui::style::{Color as TuiColor, Modifier as TuiModifier, Style as TuiStyle};
use tui::text::{Line, Span, Text};
use tui::widgets::{
    Block, BorderType as TuiBorderType, Borders as TuiBorders, Cell, List, ListItem,
    Paragraph, Row, Table,
};
use tui::Frame;

const DEFAULT_STYLE: TuiStyle = TuiStyle::new();

lazy_static! {
    pub static ref NO_COLOR: bool = env::var("NO_COLOR").is_ok();
}

fn read_only_indicator(app: &app::App) -> &str {
    if app.config.general.read_only {
        "(r)"
    } else {
        ""
    }
}

fn selection_indicator(app: &app::App) -> String {
    let count = app.selection.len();
    if count == 0 {
        String::new()
    } else {
        format!(" {{{count} sel}}")
    }
}

pub fn string_to_text<'a>(string: String) -> Text<'a> {
    if *NO_COLOR {
        Text::raw(string)
    } else {
        string
            .as_bytes()
            .into_text()
            .unwrap_or_else(|e| Text::raw(format!("{e:?}")))
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Rect {
    x: u16,
    y: u16,
    height: u16,
    width: u16,
}

impl From<TuiRect> for Rect {
    fn from(tui: TuiRect) -> Self {
        Self {
            x: tui.x,
            y: tui.y,
            height: tui.height,
            width: tui.width,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct ContentRendererArg {
    pub app: app::LuaContextLight,
    pub screen_size: Rect,
    pub layout_size: Rect,
    pub scrolltop: u16,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct LayoutOptions {
    #[serde(default)]
    pub margin: Option<u16>,

    #[serde(default)]
    pub horizontal_margin: Option<u16>,

    #[serde(default)]
    pub vertical_margin: Option<u16>,

    #[serde(default)]
    pub constraints: Option<Vec<Constraint>>,
}

impl LayoutOptions {
    pub fn extend(mut self, other: &Self) -> Self {
        self.margin = other.margin.or(self.margin);
        self.horizontal_margin = other.horizontal_margin.or(self.horizontal_margin);
        self.vertical_margin = other.vertical_margin.or(self.vertical_margin);
        self.constraints = other.constraints.clone().or(self.constraints);
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub enum CustomPanel {
    CustomParagraph {
        #[serde(default)]
        ui: PanelUiConfig,
        body: String,
    },
    CustomList {
        #[serde(default)]
        ui: PanelUiConfig,
        body: Vec<String>,
    },
    CustomTable {
        #[serde(default)]
        ui: PanelUiConfig,
        widths: Vec<Constraint>,
        col_spacing: Option<u16>,
        body: Vec<Vec<String>>,
    },
    CustomLayout(Layout),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub enum Layout {
    Nothing,
    Table,
    InputAndLogs,
    Selection,
    HelpMenu,
    SortAndFilter,
    Static(Box<CustomPanel>),
    Dynamic(String),
    Horizontal {
        config: LayoutOptions,
        splits: Vec<Layout>,
    },
    Vertical {
        config: LayoutOptions,
        splits: Vec<Layout>,
    },

    /// For compatibility only. A better choice is Static or Dynamic layout.
    CustomContent(Box<CustomContent>),
}

impl Default for Layout {
    fn default() -> Self {
        Self::Nothing
    }
}

impl Layout {
    pub fn extend(self, other: &Self) -> Self {
        match (self, other) {
            (s, Self::Nothing) => s,
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
                splits: osplits.clone(),
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
                splits: osplits.clone(),
            },
            (_, other) => other.clone(),
        }
    }

    pub fn replace(self, target: &Self, replacement: &Self) -> Self {
        match self {
            Self::Horizontal { splits, config } => Self::Horizontal {
                splits: splits
                    .into_iter()
                    .map(|s| s.replace(target, replacement))
                    .collect(),
                config,
            },
            Self::Vertical { splits, config } => Self::Vertical {
                splits: splits
                    .into_iter()
                    .map(|s| s.replace(target, replacement))
                    .collect(),
                config,
            },
            other => {
                if other == *target {
                    replacement.clone()
                } else {
                    other
                }
            }
        }
    }
}

#[derive(
    Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Serialize, Deserialize,
)]
#[serde(deny_unknown_fields)]
pub enum Border {
    Top,
    Right,
    Bottom,
    Left,
}

impl Border {
    pub fn bits(self) -> u8 {
        match self {
            Self::Top => TuiBorders::TOP.bits(),
            Self::Right => TuiBorders::RIGHT.bits(),
            Self::Bottom => TuiBorders::BOTTOM.bits(),
            Self::Left => TuiBorders::LEFT.bits(),
        }
    }
}

#[derive(
    Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Serialize, Deserialize,
)]
#[serde(deny_unknown_fields)]
pub enum BorderType {
    Plain,
    Rounded,
    Double,
    Thick,
}

impl Default for BorderType {
    fn default() -> Self {
        Self::Plain
    }
}

impl Into<TuiBorderType> for BorderType {
    fn into(self) -> TuiBorderType {
        match self {
            BorderType::Plain => TuiBorderType::Plain,
            BorderType::Rounded => TuiBorderType::Rounded,
            BorderType::Double => TuiBorderType::Double,
            BorderType::Thick => TuiBorderType::Thick,
        }
    }
}

#[derive(
    Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Serialize, Deserialize,
)]
#[serde(deny_unknown_fields)]
pub enum Modifier {
    Bold,
    Dim,
    Italic,
    Underlined,
    SlowBlink,
    RapidBlink,
    Reversed,
    Hidden,
    CrossedOut,
}

impl Modifier {
    pub fn bits(self) -> u16 {
        match self {
            Self::Bold => TuiModifier::BOLD.bits(),
            Self::Dim => TuiModifier::DIM.bits(),
            Self::Italic => TuiModifier::ITALIC.bits(),
            Self::Underlined => TuiModifier::UNDERLINED.bits(),
            Self::SlowBlink => TuiModifier::SLOW_BLINK.bits(),
            Self::RapidBlink => TuiModifier::RAPID_BLINK.bits(),
            Self::Reversed => TuiModifier::REVERSED.bits(),
            Self::Hidden => TuiModifier::HIDDEN.bits(),
            Self::CrossedOut => TuiModifier::CROSSED_OUT.bits(),
        }
    }
}

fn extend_optional_modifiers(
    a: Option<IndexSet<Modifier>>,
    b: Option<IndexSet<Modifier>>,
) -> Option<IndexSet<Modifier>> {
    match (a, b) {
        (Some(mut a), Some(b)) => {
            a.extend(b);
            Some(a)
        }
        (Some(a), None) => Some(a),
        (None, Some(b)) => Some(b),
        (None, None) => None,
    }
}

// raratui doesn't support directly serializing `Color::Rgb` and
// `Color::Indexed` anymore.
// See https://github.com/ratatui-org/ratatui/pull/934
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum Color {
    #[default]
    Reset,
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    Gray,
    DarkGray,
    LightRed,
    LightGreen,
    LightYellow,
    LightBlue,
    LightMagenta,
    LightCyan,
    White,
    Rgb(u8, u8, u8),
    Indexed(u8),
}

impl From<Color> for TuiColor {
    fn from(value: Color) -> Self {
        match value {
            Color::Reset => TuiColor::Reset,
            Color::Black => TuiColor::Black,
            Color::Red => TuiColor::Red,
            Color::Green => TuiColor::Green,
            Color::Yellow => TuiColor::Yellow,
            Color::Blue => TuiColor::Blue,
            Color::Magenta => TuiColor::Magenta,
            Color::Cyan => TuiColor::Cyan,
            Color::Gray => TuiColor::Gray,
            Color::DarkGray => TuiColor::DarkGray,
            Color::LightRed => TuiColor::LightRed,
            Color::LightGreen => TuiColor::LightGreen,
            Color::LightYellow => TuiColor::LightYellow,
            Color::LightBlue => TuiColor::LightBlue,
            Color::LightMagenta => TuiColor::LightMagenta,
            Color::LightCyan => TuiColor::LightCyan,
            Color::White => TuiColor::White,
            Color::Rgb(r, g, b) => TuiColor::Rgb(r, g, b),
            Color::Indexed(index) => TuiColor::Indexed(index),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Style {
    pub fg: Option<Color>,
    pub bg: Option<Color>,
    pub add_modifiers: Option<IndexSet<Modifier>>,
    pub sub_modifiers: Option<IndexSet<Modifier>>,
}

impl Style {
    pub fn extend(mut self, other: &Self) -> Self {
        self.fg = other.fg.or(self.fg);
        self.bg = other.bg.or(self.bg);
        self.add_modifiers =
            extend_optional_modifiers(self.add_modifiers, other.add_modifiers.clone());
        self.sub_modifiers =
            extend_optional_modifiers(self.sub_modifiers, other.sub_modifiers.clone());
        self
    }
}

impl From<Style> for TuiStyle {
    fn from(val: Style) -> Self {
        fn xor(modifiers: Option<IndexSet<Modifier>>) -> u16 {
            modifiers
                .unwrap_or_default()
                .into_iter()
                .map(Modifier::bits)
                .fold(0, BitXor::bitxor)
        }
        if *NO_COLOR {
            DEFAULT_STYLE
        } else {
            TuiStyle {
                fg: val.fg.map(Into::into),
                bg: val.bg.map(Into::into),
                add_modifier: TuiModifier::from_bits_truncate(xor(val.add_modifiers)),
                sub_modifier: TuiModifier::from_bits_truncate(xor(val.sub_modifiers)),
                underline_color: None,
            }
        }
    }
}

impl From<&LsColorsStyle> for Style {
    fn from(style: &LsColorsStyle) -> Self {
        fn convert_color(color: &LsColorsColor) -> Color {
            match color {
                LsColorsColor::Black => Color::Black,
                LsColorsColor::Red => Color::Red,
                LsColorsColor::Green => Color::Green,
                LsColorsColor::Yellow => Color::Yellow,
                LsColorsColor::Blue => Color::Blue,
                LsColorsColor::Magenta => Color::Magenta,
                LsColorsColor::Cyan => Color::Cyan,
                LsColorsColor::White => Color::Gray,
                LsColorsColor::BrightBlack => Color::DarkGray,
                LsColorsColor::BrightRed => Color::LightRed,
                LsColorsColor::BrightGreen => Color::LightGreen,
                LsColorsColor::BrightYellow => Color::LightYellow,
                LsColorsColor::BrightBlue => Color::LightBlue,
                LsColorsColor::BrightMagenta => Color::LightMagenta,
                LsColorsColor::BrightCyan => Color::LightCyan,
                LsColorsColor::BrightWhite => Color::White,
                LsColorsColor::Fixed(index) => Color::Indexed(*index),
                LsColorsColor::RGB(r, g, b) => Color::Rgb(*r, *g, *b),
            }
        }
        Self {
            fg: style.foreground.as_ref().map(convert_color),
            bg: style.background.as_ref().map(convert_color),
            add_modifiers: None,
            sub_modifiers: None,
        }
    }
}

impl From<Style> for nu_ansi_term::Style {
    fn from(val: Style) -> Self {
        fn convert_color(color: Color) -> Option<nu_ansi_term::Color> {
            match color {
                Color::Black => Some(nu_ansi_term::Color::Black),
                Color::Red => Some(nu_ansi_term::Color::Red),
                Color::Green => Some(nu_ansi_term::Color::Green),
                Color::Yellow => Some(nu_ansi_term::Color::Yellow),
                Color::Blue => Some(nu_ansi_term::Color::Blue),
                Color::Magenta => Some(nu_ansi_term::Color::Purple),
                Color::Cyan => Some(nu_ansi_term::Color::Cyan),
                Color::Gray => Some(nu_ansi_term::Color::LightGray),
                Color::DarkGray => Some(nu_ansi_term::Color::DarkGray),
                Color::LightRed => Some(nu_ansi_term::Color::LightRed),
                Color::LightGreen => Some(nu_ansi_term::Color::LightGreen),
                Color::LightYellow => Some(nu_ansi_term::Color::LightYellow),
                Color::LightBlue => Some(nu_ansi_term::Color::LightBlue),
                Color::LightMagenta => Some(nu_ansi_term::Color::LightMagenta),
                Color::LightCyan => Some(nu_ansi_term::Color::LightCyan),
                Color::White => Some(nu_ansi_term::Color::White),
                Color::Rgb(r, g, b) => Some(nu_ansi_term::Color::Rgb(r, g, b)),
                Color::Indexed(index) => Some(nu_ansi_term::Color::Fixed(index)),
                _ => None,
            }
        }
        fn match_modifiers<F>(style: &Style, f: F) -> bool
        where
            F: Fn(&IndexSet<Modifier>) -> bool,
        {
            style.add_modifiers.as_ref().is_some_and(f)
        }

        let mut style = nu_ansi_term::Style::new();
        style.foreground = val.fg.and_then(convert_color);
        style.background = val.bg.and_then(convert_color);
        style.is_bold = match_modifiers(&val, |m| m.contains(&Modifier::Bold));
        style.is_dimmed = match_modifiers(&val, |m| m.contains(&Modifier::Dim));
        style.is_italic = match_modifiers(&val, |m| m.contains(&Modifier::Italic));
        style.is_underline =
            match_modifiers(&val, |m| m.contains(&Modifier::Underlined));
        style.is_blink = match_modifiers(&val, |m| {
            m.contains(&Modifier::SlowBlink) || m.contains(&Modifier::RapidBlink)
        });
        style.is_reverse = match_modifiers(&val, |m| m.contains(&Modifier::Reversed));
        style.is_hidden = match_modifiers(&val, |m| m.contains(&Modifier::Hidden));
        style.is_strikethrough =
            match_modifiers(&val, |m| m.contains(&Modifier::CrossedOut));
        style
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WrapOptions {
    pub width: usize,
    pub initial_indent: Option<String>,
    pub subsequent_indent: Option<String>,
    pub break_words: Option<bool>,
}

impl WrapOptions {
    pub fn get_options(&self) -> textwrap::Options<'_> {
        let mut options = textwrap::Options::new(self.width);
        if let Some(initial_indent) = &self.initial_indent {
            options = options.initial_indent(initial_indent);
        }
        if let Some(subsequent_indent) = &self.subsequent_indent {
            options = options.subsequent_indent(subsequent_indent);
        }
        if let Some(break_words) = self.break_words {
            options = options.break_words(break_words);
        }
        options
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
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
    MaxLessThanLayoutWidth(u16),
    Min(u16),
    MinLessThanScreenHeight(u16),
    MinLessThanScreenWidth(u16),
    MinLessThanLayoutHeight(u16),
    MinLessThanLayoutWidth(u16),
}

impl Constraint {
    pub fn to_tui(self, screen_size: TuiRect, layout_size: TuiRect) -> TuiConstraint {
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
            Self::MaxLessThanScreenHeight(n) => {
                TuiConstraint::Max(screen_size.height.max(n) - n)
            }
            Self::MaxLessThanScreenWidth(n) => {
                TuiConstraint::Max(screen_size.width.max(n) - n)
            }
            Self::MaxLessThanLayoutHeight(n) => {
                TuiConstraint::Max(layout_size.height.max(n) - n)
            }
            Self::MaxLessThanLayoutWidth(n) => {
                TuiConstraint::Max(layout_size.width.max(n) - n)
            }
            Self::Min(n) => TuiConstraint::Min(n),
            Self::MinLessThanScreenHeight(n) => {
                TuiConstraint::Min(screen_size.height.max(n) - n)
            }
            Self::MinLessThanScreenWidth(n) => {
                TuiConstraint::Min(screen_size.width.max(n) - n)
            }
            Self::MinLessThanLayoutHeight(n) => {
                TuiConstraint::Min(layout_size.height.max(n) - n)
            }
            Self::MinLessThanLayoutWidth(n) => {
                TuiConstraint::Min(layout_size.width.max(n) - n)
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolvedNodeUiMetadata {
    pub absolute_path: String,
    pub extension: String,
    pub is_dir: bool,
    pub is_file: bool,
    pub is_readonly: bool,
    pub mime_essence: String,
    pub size: u64,
    pub human_size: String,
    pub created: Option<u128>,
    pub last_modified: Option<u128>,
}

impl From<ResolvedNode> for ResolvedNodeUiMetadata {
    fn from(node: ResolvedNode) -> Self {
        Self {
            absolute_path: node.absolute_path.clone(),
            extension: node.extension.clone(),
            is_dir: node.is_dir,
            is_file: node.is_file,
            is_readonly: node.is_readonly,
            mime_essence: node.mime_essence.clone(),
            size: node.size,
            human_size: node.human_size,
            created: node.created,
            last_modified: node.last_modified,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeUiMetadata {
    // From Node
    pub parent: String,
    pub relative_path: String,
    pub absolute_path: String,
    pub extension: String,
    pub is_symlink: bool,
    pub is_broken: bool,
    pub is_dir: bool,
    pub is_file: bool,
    pub is_readonly: bool,
    pub mime_essence: String,
    pub size: u64,
    pub human_size: String,
    pub permissions: Permissions,
    pub canonical: Option<ResolvedNodeUiMetadata>,
    pub symlink: Option<ResolvedNodeUiMetadata>,
    pub created: Option<u128>,
    pub last_modified: Option<u128>,
    pub uid: u32,
    pub gid: u32,

    // Extra
    pub index: usize,
    pub relative_index: usize,
    pub is_before_focus: bool,
    pub is_after_focus: bool,
    pub tree: String,
    pub prefix: String,
    pub suffix: String,
    pub is_selected: bool,
    pub is_focused: bool,
    pub total: usize,
    pub meta: HashMap<String, String>,
    pub style: Style,
}

impl NodeUiMetadata {
    fn new(
        node: &Node,
        index: usize,
        relative_index: usize,
        is_before_focus: bool,
        is_after_focus: bool,
        tree: String,
        prefix: String,
        suffix: String,
        is_selected: bool,
        is_focused: bool,
        total: usize,
        meta: HashMap<String, String>,
        style: Style,
    ) -> Self {
        Self {
            parent: node.parent.clone(),
            relative_path: node.relative_path.clone(),
            absolute_path: node.absolute_path.clone(),
            extension: node.extension.clone(),
            is_symlink: node.is_symlink,
            is_broken: node.is_broken,
            is_dir: node.is_dir,
            is_file: node.is_file,
            is_readonly: node.is_readonly,
            mime_essence: node.mime_essence.clone(),
            size: node.size,
            human_size: node.human_size.clone(),
            permissions: node.permissions,
            canonical: node.canonical.clone().map(ResolvedNode::into),
            symlink: node.symlink.clone().map(ResolvedNode::into),
            created: node.created,
            last_modified: node.last_modified,
            uid: node.uid,
            gid: node.gid,
            index,
            relative_index,
            is_before_focus,
            is_after_focus,
            tree,
            prefix,
            suffix,
            is_selected,
            is_focused,
            total,
            meta,
            style,
        }
    }
}

pub fn block<'a>(config: PanelUiConfig, default_title: String) -> Block<'a> {
    Block::default()
        .borders(TuiBorders::from_bits_truncate(
            config
                .borders
                .clone()
                .unwrap_or_default()
                .iter()
                .map(|b| b.bits())
                .fold(0, |a, b| (a ^ b)),
        ))
        .title(Span::styled(
            config.title.format.unwrap_or(default_title),
            config.title.style,
        ))
        .style(config.style)
        .border_type(config.border_type.unwrap_or_default().into())
        .border_style(config.border_style)
}

pub struct UI<'lua> {
    pub lua: &'lua Lua,
    pub screen_size: TuiRect,
    pub scrolltop: usize,
}

impl<'lua> UI<'lua> {
    pub fn new(lua: &'lua Lua) -> Self {
        let screen_size = Default::default();
        let scrolltop = 0;
        Self {
            lua,
            scrolltop,
            screen_size,
        }
    }
}

impl UI<'_> {
    fn draw_table(&mut self, f: &mut Frame, layout_size: TuiRect, app: &app::App) {
        let panel_config = &app.config.general.panel_ui;
        let config = panel_config.default.clone().extend(&panel_config.table);
        let app_config = app.config.clone();
        let header_height = app_config.general.table.header.height.unwrap_or(1);
        let height: usize = layout_size.height.saturating_sub(header_height + 2).into();
        let row_style = app_config.general.table.row.style.clone();

        let rows = app
            .directory_buffer
            .as_ref()
            .map(|dir| {
                // Scroll
                if app.config.general.paginated_scrolling {
                    // Paginated scrolling
                    self.scrolltop = height * (dir.focus / height.max(1))
                } else {
                    // Vim-like-scrolling
                    let padding = app
                        .config
                        .general
                        .scroll_padding
                        .min(height / 2)
                        .saturating_sub(1);
                    if dir.focus >= (self.scrolltop + height).saturating_sub(padding) {
                        // Scrolling down
                        self.scrolltop = (dir.focus + padding + 1)
                            .saturating_sub(height)
                            .min(dir.total.saturating_sub(height));
                    } else if dir.focus < self.scrolltop + padding {
                        // Scrolling up
                        self.scrolltop = dir.focus.saturating_sub(padding);
                    }
                };

                dir.nodes
                    .iter()
                    .enumerate()
                    .skip(self.scrolltop)
                    .take(height)
                    .map(|(index, node)| {
                        let is_focused = dir.focus == index;

                        let is_selected = app
                            .selection
                            .iter()
                            .any(|s| s.absolute_path == node.absolute_path);

                        let is_first = index == 0;
                        let is_last = index == dir.total.max(1) - 1;

                        let tree = app_config
                            .general
                            .table
                            .tree
                            .clone()
                            .map(|t| {
                                if is_last {
                                    t.2.format
                                } else if is_first {
                                    t.0.format
                                } else {
                                    t.1.format
                                }
                            })
                            .unwrap_or_default();

                        let node_type = app_config.node_types.get(node);

                        let (relative_index, is_before_focus, is_after_focus) =
                            match dir.focus.cmp(&index) {
                                Ordering::Greater => (dir.focus - index, true, false),
                                Ordering::Less => (index - dir.focus, false, true),
                                Ordering::Equal => (0, false, false),
                            };

                        let (mut prefix, mut suffix, mut style) = {
                            let ui = app_config.general.default_ui.clone();
                            (ui.prefix, ui.suffix, ui.style.extend(&node_type.style))
                        };

                        if is_focused && is_selected {
                            let ui = app_config.general.focus_selection_ui.clone();
                            prefix = ui.prefix.clone().or(prefix);
                            suffix = ui.suffix.clone().or(suffix);
                            style = style.extend(&ui.style);
                        } else if is_selected {
                            let ui = app_config.general.selection_ui.clone();
                            prefix = ui.prefix.clone().or(prefix);
                            suffix = ui.suffix.clone().or(suffix);
                            style = style.extend(&ui.style);
                        } else if is_focused {
                            let ui = app_config.general.focus_ui.clone();
                            prefix = ui.prefix.clone().or(prefix);
                            suffix = ui.suffix.clone().or(suffix);
                            style = style.extend(&ui.style);
                        };

                        let meta = NodeUiMetadata::new(
                            node,
                            index,
                            relative_index,
                            is_before_focus,
                            is_after_focus,
                            tree.unwrap_or_default(),
                            prefix.unwrap_or_default(),
                            suffix.unwrap_or_default(),
                            is_selected,
                            is_focused,
                            dir.total,
                            node_type.meta,
                            style,
                        );

                        let cols = lua::serialize::<NodeUiMetadata>(self.lua, &meta)
                            .map(|v| {
                                app_config
                                    .general
                                    .table
                                    .row
                                    .cols
                                    .clone()
                                    .unwrap_or_default()
                                    .iter()
                                    .filter_map(|c| {
                                        c.format.as_ref().map(|f| {
                                            let out = lua::call(self.lua, f, v.clone())
                                                .unwrap_or_else(|e| format!("{e:?}"));
                                            (string_to_text(out), c.style.clone())
                                        })
                                    })
                                    .collect::<Vec<(Text, Style)>>()
                            })
                            .unwrap_or_default()
                            .into_iter()
                            .map(|(text, style)| Cell::from(text).style(style))
                            .collect::<Vec<Cell>>();

                        Row::new(cols).style(row_style.clone())
                    })
                    .collect::<Vec<Row>>()
            })
            .unwrap_or_default();

        let table_constraints: Vec<TuiConstraint> = app_config
            .general
            .table
            .col_widths
            .clone()
            .unwrap_or_default()
            .into_iter()
            .map(|c| c.to_tui(self.screen_size, layout_size))
            .collect();

        let pwd = if let Some(vroot) = app.vroot.as_ref() {
            app.pwd.strip_prefix(vroot).unwrap_or(&app.pwd)
        } else {
            &app.pwd
        }
        .trim_matches('/');

        let pwd = path::escape(pwd);

        let vroot_indicator = if app.vroot.is_some() { "vroot:" } else { "" };

        let node_count = app.directory_buffer.as_ref().map(|d| d.total).unwrap_or(0);
        let node_count = if node_count == 0 {
            String::new()
        } else {
            format!("({node_count}) ")
        };

        let table = Table::new(rows, table_constraints)
            .style(app_config.general.table.style.clone())
            .row_highlight_style(app_config.general.focus_ui.style.clone())
            .column_spacing(app_config.general.table.col_spacing.unwrap_or_default())
            .block(block(
                config,
                format!(" {vroot_indicator}/{pwd} {node_count}"),
            ));

        let table = table.clone().header(
            Row::new(
                app_config
                    .general
                    .table
                    .header
                    .cols
                    .clone()
                    .unwrap_or_default()
                    .iter()
                    .map(|c| {
                        Cell::from(c.format.clone().unwrap_or_default())
                            .style(c.style.clone())
                    })
                    .collect::<Vec<Cell>>(),
            )
            .height(header_height)
            .style(app_config.general.table.header.style.clone()),
        );

        f.render_widget(table, layout_size);
    }

    fn draw_selection(&mut self, f: &mut Frame, layout_size: TuiRect, app: &app::App) {
        let panel_config = &app.config.general.panel_ui;
        let config = panel_config.default.clone().extend(&panel_config.selection);

        let selection_count = app.selection.len();

        let selection: Vec<ListItem> = app
            .selection
            .iter()
            .rev()
            .take((layout_size.height.max(2) - 2).into())
            .rev()
            .map(|n| {
                let out = app
                    .config
                    .general
                    .selection
                    .item
                    .format
                    .as_ref()
                    .map(|f| {
                        lua::serialize::<Node>(self.lua, n)
                            .and_then(|n| lua::call(self.lua, f, n))
                            .unwrap_or_else(|e| format!("{e:?}"))
                    })
                    .unwrap_or_else(|| n.absolute_path.clone());
                string_to_text(out)
            })
            .map(|i| {
                ListItem::new(i).style(app.config.general.selection.item.style.clone())
            })
            .collect();

        // Selected items
        let selection_count = if selection_count == 0 {
            String::new()
        } else {
            format!("({selection_count}) ")
        };

        let selection_list = List::new(selection)
            .block(block(config, format!(" Selection {selection_count}")));

        f.render_widget(selection_list, layout_size);
    }

    fn draw_help_menu(&mut self, f: &mut Frame, layout_size: TuiRect, app: &app::App) {
        let panel_config = &app.config.general.panel_ui;

        let config = panel_config.default.clone().extend(&panel_config.help_menu);

        let help_menu_rows = app
            .mode
            .help_menu()
            .into_iter()
            .map(|l| match l {
                HelpMenuLine::Paragraph(p) => Row::new([Cell::from(p)].to_vec()),
                HelpMenuLine::KeyMap(k, remaps, h) => Row::new({
                    if app.config.general.hide_remaps_in_help_menu {
                        [Cell::from(k), Cell::from(h)].to_vec()
                    } else {
                        [Cell::from(k), Cell::from(remaps.join("|")), Cell::from(h)]
                            .to_vec()
                    }
                }),
            })
            .collect::<Vec<Row>>();

        let widths = if app.config.general.hide_remaps_in_help_menu {
            vec![TuiConstraint::Percentage(20), TuiConstraint::Percentage(80)]
        } else {
            vec![
                TuiConstraint::Percentage(20),
                TuiConstraint::Percentage(20),
                TuiConstraint::Percentage(60),
            ]
        };
        let help_menu = Table::new(help_menu_rows, widths).block(block(
            config,
            format!(" Help [{}{}] ", &app.mode.name, read_only_indicator(app)),
        ));
        f.render_widget(help_menu, layout_size);
    }

    fn draw_input_buffer(
        &mut self,
        f: &mut Frame,
        layout_size: TuiRect,
        app: &app::App,
    ) {
        if let Some(input) = app.input.buffer.as_ref() {
            let panel_config = &app.config.general.panel_ui;
            let config = panel_config
                .default
                .clone()
                .extend(&panel_config.input_and_logs);

            let cursor_offset_left = config
                .borders
                .as_ref()
                .map(|b| b.contains(&Border::Left))
                .unwrap_or(false) as u16
                + app.input.prompt.chars().count() as u16;

            let cursor_offset_right = config
                .borders
                .as_ref()
                .map(|b| b.contains(&Border::Right))
                .unwrap_or(false) as u16
                + 1;

            let offset_width = cursor_offset_left + cursor_offset_right;
            let width = layout_size.width.max(offset_width) - offset_width;
            let scroll = input.visual_scroll(width.into()) as u16;

            let input_buf = Paragraph::new(Line::from(vec![
                Span::styled(
                    app.input.prompt.clone(),
                    app.config.general.prompt.style.clone(),
                ),
                Span::raw(input.value()),
            ]))
            .scroll((0, scroll))
            .block(block(
                config,
                format!(
                    " Input [{}{}]{} ",
                    app.mode.name,
                    read_only_indicator(app),
                    selection_indicator(app),
                ),
            ));

            f.render_widget(input_buf, layout_size);
            f.set_cursor_position(
                // Put cursor past the end of the input text
                (
                    layout_size.x
                        + (input.visual_cursor() as u16).min(width)
                        + cursor_offset_left,
                    // Move one line down, from the border to the input line
                    layout_size.y + 1,
                ),
            );
        };
    }

    fn draw_sort_n_filter(
        &mut self,
        f: &mut Frame,
        layout_size: TuiRect,
        app: &app::App,
    ) {
        let panel_config = &app.config.general.panel_ui;
        let config = panel_config
            .default
            .clone()
            .extend(&panel_config.sort_and_filter);
        let ui = app.config.general.sort_and_filter_ui.clone();
        let filter_by: &IndexSet<NodeFilterApplicable> = &app.explorer_config.filters;
        let sort_by: &IndexSet<NodeSorterApplicable> = &app.explorer_config.sorters;
        let search = app.explorer_config.searcher.as_ref();

        let defaultui = &ui.default_identifier;
        let forwardui = defaultui
            .clone()
            .extend(&ui.sort_direction_identifiers.forward);
        let reverseui = defaultui
            .clone()
            .extend(&ui.sort_direction_identifiers.reverse);

        let orderedui = defaultui
            .clone()
            .extend(&ui.search_direction_identifiers.ordered);
        let unorderedui = defaultui
            .clone()
            .extend(&ui.search_direction_identifiers.unordered);

        let is_ordered_search = search.as_ref().map(|s| !s.unordered).unwrap_or(false);

        let mut spans = filter_by
            .iter()
            .map(|f| {
                ui.filter_identifiers
                    .get(&f.filter)
                    .map(|u| {
                        let ui = defaultui.clone().extend(u);
                        (
                            Span::styled(
                                ui.format.clone().unwrap_or_default(),
                                ui.style.clone(),
                            ),
                            Span::styled(f.input.clone(), ui.style),
                        )
                    })
                    .unwrap_or((Span::raw("f"), Span::raw("")))
            })
            .chain(search.iter().map(|s| {
                ui.search_identifiers
                    .get(&s.algorithm)
                    .map(|u| {
                        let direction = if s.unordered {
                            &unorderedui
                        } else {
                            &orderedui
                        };
                        let ui = defaultui.clone().extend(u);
                        let f = ui
                            .format
                            .as_ref()
                            .map(|f| format!("{f}{p}", p = &s.pattern))
                            .unwrap_or_else(|| s.pattern.clone());
                        (
                            Span::styled(f, ui.style),
                            Span::styled(
                                direction.format.clone().unwrap_or_default(),
                                direction.style.clone(),
                            ),
                        )
                    })
                    .unwrap_or((Span::raw("/"), Span::raw(&s.pattern)))
            }))
            .chain(
                sort_by
                    .iter()
                    .map(|s| {
                        let direction = if s.reverse { &reverseui } else { &forwardui };
                        ui.sorter_identifiers
                            .get(&s.sorter)
                            .map(|u| {
                                let ui = defaultui.clone().extend(u);
                                (
                                    Span::styled(
                                        ui.format.clone().unwrap_or_default(),
                                        ui.style,
                                    ),
                                    Span::styled(
                                        direction.format.clone().unwrap_or_default(),
                                        direction.style.clone(),
                                    ),
                                )
                            })
                            .unwrap_or((Span::raw("s"), Span::raw("")))
                    })
                    .take(if !is_ordered_search { sort_by.len() } else { 0 }),
            )
            .zip(std::iter::repeat(Span::styled(
                ui.separator.format.clone().unwrap_or_default(),
                ui.separator.style.clone(),
            )))
            .flat_map(|((a, b), c)| vec![a, b, c])
            .collect::<Vec<Span>>();

        spans.pop();

        let item_count = filter_by.len() + sort_by.len();
        let item_count = if item_count == 0 {
            String::new()
        } else {
            format!("({item_count}) ")
        };

        let p = Paragraph::new(Line::from(spans))
            .block(block(config, format!(" Sort & filter {item_count}")));

        f.render_widget(p, layout_size);
    }

    fn draw_logs(&mut self, f: &mut Frame, layout_size: TuiRect, app: &app::App) {
        let panel_config = &app.config.general.panel_ui;
        let config = panel_config
            .default
            .clone()
            .extend(&panel_config.input_and_logs);
        let logs_config = app.config.general.logs.clone();
        let logs = if app.logs_hidden {
            vec![]
        } else {
            app.logs
                .iter()
                .rev()
                .take(layout_size.height as usize)
                .map(|log| {
                    let fd = format_description!("[hour]:[minute]:[second]");
                    let time =
                        log.created_at.format(fd).unwrap_or_else(|_| "when?".into());
                    let cfg = match log.level {
                        app::LogLevel::Info => &logs_config.info,
                        app::LogLevel::Warning => &logs_config.warning,
                        app::LogLevel::Success => &logs_config.success,
                        app::LogLevel::Error => &logs_config.error,
                    };

                    let prefix =
                        format!("{time}|{0}", cfg.format.clone().unwrap_or_default());

                    let padding = " ".repeat(prefix.chars().count());

                    let txt = log
                        .message
                        .lines()
                        .enumerate()
                        .map(|(i, line)| {
                            if i == 0 {
                                format!("{prefix}) {line}")
                            } else {
                                format!("{padding}  {line}")
                            }
                        })
                        .take(layout_size.height as usize)
                        .collect::<Vec<_>>()
                        .join("\n");

                    ListItem::new(txt).style(cfg.style.clone())
                })
                .collect::<Vec<ListItem>>()
        };

        let logs_count = app.logs.len();
        let logs_count = if logs_count == 0 {
            String::new()
        } else {
            format!(" ({logs_count})")
        };

        let logs_list = List::new(logs).block(block(
            config,
            format!(
                " Logs{} [{}{}]{} ",
                logs_count,
                app.mode.name,
                read_only_indicator(app),
                selection_indicator(app)
            ),
        ));

        f.render_widget(logs_list, layout_size);
    }

    fn draw_nothing(&mut self, f: &mut Frame, layout_size: TuiRect, app: &app::App) {
        let panel_config = &app.config.general.panel_ui;
        let config = panel_config.default.clone();
        let nothing = Paragraph::new("").block(block(config, "".into()));
        f.render_widget(nothing, layout_size);
    }

    fn draw_dynamic(
        &mut self,
        f: &mut Frame,
        layout_size: TuiRect,
        app: &app::App,
        func: &str,
    ) {
        let ctx = ContentRendererArg {
            app: app.to_lua_ctx_light(),
            layout_size: layout_size.into(),
            screen_size: self.screen_size.into(),
            scrolltop: self.scrolltop as u16,
        };

        let panel: CustomPanel = lua::serialize(self.lua, &ctx)
            .and_then(|arg| lua::call(self.lua, func, arg))
            .unwrap_or_else(|e| CustomPanel::CustomParagraph {
                ui: app.config.general.panel_ui.default.clone(),
                body: format!("{e:?}"),
            });

        self.draw_static(f, layout_size, app, panel);
    }

    fn draw_static(
        &mut self,
        f: &mut Frame,
        layout_size: TuiRect,
        app: &app::App,
        panel: CustomPanel,
    ) {
        let defaultui = app.config.general.panel_ui.default.clone();
        match panel {
            CustomPanel::CustomParagraph { ui, body } => {
                let config = defaultui.extend(&ui);
                let body = string_to_text(body);
                let content = Paragraph::new(body).block(block(config, "".into()));
                f.render_widget(content, layout_size);
            }

            CustomPanel::CustomList { ui, body } => {
                let config = defaultui.extend(&ui);

                let items = body
                    .into_iter()
                    .map(string_to_text)
                    .map(ListItem::new)
                    .collect::<Vec<ListItem>>();

                let content = List::new(items).block(block(config, "".into()));
                f.render_widget(content, layout_size);
            }

            CustomPanel::CustomTable {
                ui,
                widths,
                col_spacing,
                body,
            } => {
                let config = defaultui.extend(&ui);
                let rows = body
                    .into_iter()
                    .map(|cols| {
                        Row::new(
                            cols.into_iter()
                                .map(string_to_text)
                                .map(Cell::from)
                                .collect::<Vec<Cell>>(),
                        )
                    })
                    .collect::<Vec<Row>>();

                let widths = widths
                    .into_iter()
                    .map(|w| w.to_tui(self.screen_size, layout_size))
                    .collect::<Vec<TuiConstraint>>();

                let content = Table::new(rows, widths)
                    .column_spacing(col_spacing.unwrap_or(1))
                    .block(block(config, "".into()));

                f.render_widget(content, layout_size);
            }

            CustomPanel::CustomLayout(layout) => {
                self.draw_layout(layout, f, layout_size, app);
            }
        }
    }

    fn draw_layout(
        &mut self,
        layout: Layout,
        f: &mut Frame,
        layout_size: TuiRect,
        app: &app::App,
    ) {
        match layout {
            Layout::Nothing => self.draw_nothing(f, layout_size, app),
            Layout::Table => self.draw_table(f, layout_size, app),
            Layout::SortAndFilter => self.draw_sort_n_filter(f, layout_size, app),
            Layout::HelpMenu => self.draw_help_menu(f, layout_size, app),
            Layout::Selection => self.draw_selection(f, layout_size, app),
            Layout::InputAndLogs => {
                if app.input.buffer.is_some() {
                    self.draw_input_buffer(f, layout_size, app);
                } else {
                    self.draw_logs(f, layout_size, app);
                };
            }
            Layout::Static(panel) => self.draw_static(f, layout_size, app, *panel),
            Layout::Dynamic(ref func) => self.draw_dynamic(f, layout_size, app, func),
            Layout::CustomContent(content) => {
                draw_custom_content(self, f, layout_size, app, *content)
            }
            Layout::Horizontal { config, splits } => {
                let chunks = TuiLayout::default()
                    .direction(Direction::Horizontal)
                    .constraints(
                        config
                            .constraints
                            .clone()
                            .unwrap_or_default()
                            .iter()
                            .map(|c| c.to_tui(self.screen_size, layout_size))
                            .collect::<Vec<TuiConstraint>>(),
                    )
                    .horizontal_margin(
                        config
                            .horizontal_margin
                            .or(config.margin)
                            .unwrap_or_default(),
                    )
                    .vertical_margin(
                        config.vertical_margin.or(config.margin).unwrap_or_default(),
                    )
                    .split(layout_size);

                splits
                    .into_iter()
                    .zip(chunks.iter())
                    .for_each(|(split, chunk)| self.draw_layout(split, f, *chunk, app));
            }

            Layout::Vertical { config, splits } => {
                let chunks = TuiLayout::default()
                    .direction(Direction::Vertical)
                    .constraints(
                        config
                            .constraints
                            .clone()
                            .unwrap_or_default()
                            .iter()
                            .map(|c| c.to_tui(self.screen_size, layout_size))
                            .collect::<Vec<TuiConstraint>>(),
                    )
                    .horizontal_margin(
                        config
                            .horizontal_margin
                            .or(config.margin)
                            .unwrap_or_default(),
                    )
                    .vertical_margin(
                        config.vertical_margin.or(config.margin).unwrap_or_default(),
                    )
                    .split(layout_size);

                splits
                    .into_iter()
                    .zip(chunks.iter())
                    .for_each(|(split, chunk)| self.draw_layout(split, f, *chunk, app));
            }
        }
    }

    pub fn draw(&mut self, f: &mut Frame, app: &app::App) {
        self.screen_size = f.area();
        let layout = app.mode.layout.as_ref().unwrap_or(&app.layout).clone();
        self.draw_layout(layout, f, self.screen_size, app);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn modifier(m: Modifier) -> Option<IndexSet<Modifier>> {
        let mut x = IndexSet::new();
        x.insert(m);
        Some(x)
    }

    #[test]
    fn test_extend_style() {
        let a = Style {
            fg: Some(Color::Red),
            bg: None,
            add_modifiers: modifier(Modifier::Bold),
            sub_modifiers: None,
        };

        let b = Style {
            fg: None,
            bg: Some(Color::Blue),
            add_modifiers: None,
            sub_modifiers: modifier(Modifier::Dim),
        };

        let c = Style {
            fg: Some(Color::Cyan),
            bg: Some(Color::Magenta),
            add_modifiers: modifier(Modifier::CrossedOut),
            sub_modifiers: modifier(Modifier::Italic),
        };

        assert_eq!(
            a.clone().extend(&b),
            Style {
                fg: Some(Color::Red),
                bg: Some(Color::Blue),
                add_modifiers: modifier(Modifier::Bold),
                sub_modifiers: modifier(Modifier::Dim),
            }
        );

        assert_eq!(
            b.extend(&a),
            Style {
                fg: Some(Color::Red),
                bg: Some(Color::Blue),
                add_modifiers: modifier(Modifier::Bold),
                sub_modifiers: modifier(Modifier::Dim),
            }
        );

        assert_eq!(
            a.clone().extend(&c),
            Style {
                fg: Some(Color::Cyan),
                bg: Some(Color::Magenta),
                add_modifiers: Some(
                    vec![Modifier::Bold, Modifier::CrossedOut]
                        .into_iter()
                        .collect()
                ),
                sub_modifiers: modifier(Modifier::Italic),
            }
        );

        assert_eq!(
            c.extend(&a),
            Style {
                fg: Some(Color::Red),
                bg: Some(Color::Magenta),
                add_modifiers: Some(
                    vec![Modifier::Bold, Modifier::CrossedOut]
                        .into_iter()
                        .collect()
                ),
                sub_modifiers: modifier(Modifier::Italic),
            }
        );
    }

    #[test]
    fn test_layout_replace() {
        let layout = Layout::Horizontal {
            config: LayoutOptions {
                margin: Some(2),
                horizontal_margin: Some(3),
                vertical_margin: Some(4),
                constraints: Some(vec![
                    Constraint::Percentage(80),
                    Constraint::Percentage(20),
                ]),
            },
            splits: vec![Layout::Table, Layout::HelpMenu],
        };

        let res = layout.clone().replace(&Layout::Table, &Layout::Selection);

        match (res, layout) {
            (
                Layout::Horizontal {
                    config: res_config,
                    splits: res_splits,
                },
                Layout::Horizontal {
                    config: layout_config,
                    splits: layout_splits,
                },
            ) => {
                assert_eq!(res_config, layout_config);
                assert_eq!(res_splits.len(), layout_splits.len());
                assert_eq!(res_splits[0], Layout::Selection);
                assert_ne!(res_splits[0], layout_splits[0]);
                assert_eq!(res_splits[1], layout_splits[1]);
            }
            _ => panic!("Unexpected layout"),
        }

        let res = Layout::Table.replace(&Layout::Table, &Layout::Selection);
        assert_eq!(res, Layout::Selection);

        let res = Layout::Table.replace(&Layout::Nothing, &Layout::Selection);
        assert_eq!(res, Layout::Table);
    }
}
