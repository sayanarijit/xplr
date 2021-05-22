use crate::app;
use crate::app::HelpMenuLine;
use crate::app::{Node, ResolvedNode};
use crate::config::PanelUiConfig;
use crate::lua::resolve_fn;
use indexmap::IndexSet;
use lazy_static::lazy_static;
use mlua::Lua;
use mlua::LuaSerdeExt;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::env;
use tui::backend::Backend;
use tui::layout::Rect;
use tui::layout::{Constraint as TuiConstraint, Direction, Layout as TuiLayout};
use tui::style::{Color, Modifier as TuiModifier, Style as TuiStyle};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders as TuiBorders, Cell, List, ListItem, Paragraph, Row, Table};
use tui::Frame;

lazy_static! {
    pub static ref NO_COLOR: bool = env::var("NO_COLOR").ok().map(|_| true).unwrap_or(false);
    pub static ref DEFAULT_STYLE: TuiStyle = TuiStyle::default();
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum Layout {
    Nothing,
    Table,
    InputAndLogs,
    Selection,
    HelpMenu,
    SortAndFilter,
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
        Self::Nothing
    }
}

impl Layout {
    pub fn extend(self, other: Self) -> Self {
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

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum Border {
    Top,
    Right,
    Bottom,
    Left,
}

impl Border {
    pub fn bits(self) -> u32 {
        match self {
            Self::Top => TuiBorders::TOP.bits(),
            Self::Right => TuiBorders::RIGHT.bits(),
            Self::Bottom => TuiBorders::BOTTOM.bits(),
            Self::Left => TuiBorders::LEFT.bits(),
        }
    }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Style {
    fg: Option<Color>,
    bg: Option<Color>,
    add_modifiers: Option<IndexSet<Modifier>>,
    sub_modifiers: Option<IndexSet<Modifier>>,
}

impl PartialEq for Style {
    fn eq(&self, other: &Self) -> bool {
        self.fg == other.fg
            && self.bg == other.bg
            && self.add_modifiers == other.add_modifiers
            && self.sub_modifiers == other.sub_modifiers
    }
}

impl Style {
    pub fn extend(mut self, other: Self) -> Self {
        self.fg = other.fg.or(self.fg);
        self.bg = other.bg.or(self.bg);
        self.add_modifiers = other.add_modifiers.or(self.add_modifiers);
        self.sub_modifiers = other.sub_modifiers.or(self.sub_modifiers);
        self
    }
}

impl Into<TuiStyle> for Style {
    fn into(self) -> TuiStyle {
        if *NO_COLOR {
            *DEFAULT_STYLE
        } else {
            TuiStyle {
                fg: self.fg,
                bg: self.bg,
                add_modifier: TuiModifier::from_bits_truncate(
                    self.add_modifiers
                        .unwrap_or_default()
                        .into_iter()
                        .map(|m| m.bits())
                        .fold(0, |a, b| (a ^ b)),
                ),

                sub_modifier: TuiModifier::from_bits_truncate(
                    self.sub_modifiers
                        .unwrap_or_default()
                        .into_iter()
                        .map(|m| m.bits())
                        .fold(0, |a, b| (a ^ b)),
                ),
            }
        }
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolvedNodeUiMetadata {
    absolute_path: String,
    extension: String,
    is_dir: bool,
    is_file: bool,
    is_readonly: bool,
    mime_essence: String,
    size: u64,
    human_size: String,
}

impl From<ResolvedNode> for ResolvedNodeUiMetadata {
    fn from(node: ResolvedNode) -> Self {
        Self {
            absolute_path: node.absolute_path().clone(),
            extension: node.extension().clone(),
            is_dir: node.is_dir(),
            is_file: node.is_file(),
            is_readonly: node.is_readonly(),
            mime_essence: node.mime_essence().clone(),
            size: node.size(),
            human_size: node.human_size().clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct NodeUiMetadata {
    // From Node
    parent: String,
    relative_path: String,
    absolute_path: String,
    extension: String,
    is_symlink: bool,
    is_broken: bool,
    is_dir: bool,
    is_file: bool,
    is_readonly: bool,
    mime_essence: String,
    size: u64,
    human_size: String,
    canonical: Option<ResolvedNodeUiMetadata>,
    symlink: Option<ResolvedNodeUiMetadata>,

    // Extra
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
    ) -> Self {
        Self {
            parent: node.parent().clone(),
            relative_path: node.relative_path().clone(),
            absolute_path: node.absolute_path().clone(),
            extension: node.extension().clone(),
            is_symlink: node.is_symlink(),
            is_broken: node.is_broken(),
            is_dir: node.is_dir(),
            is_file: node.is_file(),
            is_readonly: node.is_readonly(),
            mime_essence: node.mime_essence().clone(),
            size: node.size(),
            human_size: node.human_size().clone(),
            canonical: node.canonical().to_owned().map(|s| s.into()),
            symlink: node.symlink().to_owned().map(|s| s.into()),
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
        }
    }
}

fn block<'a>(config: PanelUiConfig, default_title: String) -> Block<'a> {
    Block::default()
        .borders(TuiBorders::from_bits_truncate(
            config
                .borders()
                .clone()
                .unwrap_or_default()
                .iter()
                .map(|b| b.bits())
                .fold(0, |a, b| (a ^ b)),
        ))
        .title(Span::styled(
            config.title().format().clone().unwrap_or(default_title),
            config.title().style().clone().into(),
        ))
        .style(config.style().clone().into())
}

fn draw_table<B: Backend>(
    f: &mut Frame<B>,
    screen_size: Rect,
    layout_size: Rect,
    app: &app::App,
    lua: &Lua,
) {
    let panel_config = app.config().general().panel_ui();
    let config = panel_config
        .default()
        .clone()
        .extend(panel_config.table().clone());
    let app_config = app.config().to_owned();
    let header_height = app_config.general().table().header().height().unwrap_or(1);
    let height: usize = (layout_size.height.max(header_height + 2) - (header_height + 2)).into();

    let globals = lua.globals();

    let rows = app
        .directory_buffer()
        .map(|dir| {
            dir.nodes()
                .iter()
                .enumerate()
                .skip(height * (dir.focus() / height.max(1)))
                .take(height)
                .map(|(index, node)| {
                    let is_focused = dir.focus() == index;

                    // TODO : Optimize
                    let is_selected = app.selection().contains(node);

                    let is_first = index == 0;
                    let is_last = index == dir.total().max(1) - 1;

                    let tree = app_config
                        .general()
                        .table()
                        .tree()
                        .clone()
                        .map(|t| {
                            if is_last {
                                t.2.format().clone()
                            } else if is_first {
                                t.0.format().clone()
                            } else {
                                t.1.format().clone()
                            }
                        })
                        .unwrap_or_default();

                    let node_type = app_config
                        .node_types()
                        .special()
                        .get(node.relative_path())
                        .or_else(|| app_config.node_types().extension().get(node.extension()))
                        .or_else(|| {
                            app_config
                                .node_types()
                                .mime_essence()
                                .get(node.mime_essence())
                        })
                        .unwrap_or_else(|| {
                            if node.is_symlink() {
                                &app_config.node_types().symlink()
                            } else if node.is_dir() {
                                &app_config.node_types().directory()
                            } else {
                                &app_config.node_types().file()
                            }
                        });

                    let (relative_index, is_before_focus, is_after_focus) =
                        match dir.focus().cmp(&index) {
                            Ordering::Greater => (dir.focus() - index, true, false),
                            Ordering::Less => (index - dir.focus(), false, true),
                            Ordering::Equal => (0, false, false),
                        };

                    let (mut prefix, mut suffix, mut style) = {
                        let ui = app_config.general().default_ui().clone();
                        (
                            ui.prefix().clone(),
                            ui.suffix().clone(),
                            ui.style().clone().extend(node_type.style().clone()),
                        )
                    };

                    if is_selected {
                        let ui = app_config.general().selection_ui().clone();
                        prefix = ui.prefix().clone().or(prefix);
                        suffix = ui.suffix().clone().or(suffix);
                        style = style.extend(ui.style().clone());
                    };

                    if is_focused {
                        let ui = app_config.general().focus_ui().clone();
                        prefix = ui.prefix().clone().or(prefix);
                        suffix = ui.suffix().clone().or(suffix);
                        style = style.extend(ui.style().clone());
                    };

                    let meta = NodeUiMetadata::new(
                        &node,
                        index,
                        relative_index,
                        is_before_focus,
                        is_after_focus,
                        tree.unwrap_or_default(),
                        prefix.unwrap_or_default(),
                        suffix.unwrap_or_default(),
                        is_selected,
                        is_focused,
                        dir.total(),
                        node_type.meta().clone(),
                    );

                    let cols = lua
                        .to_value::<NodeUiMetadata>(&meta)
                        .map(|v| {
                            app_config
                                .general()
                                .table()
                                .row()
                                .cols()
                                .clone()
                                .unwrap_or_default()
                                .iter()
                                .filter_map(|c| {
                                    c.format()
                                        .to_owned()
                                        .and_then(|f| resolve_fn(&globals, &f).ok())
                                })
                                .map(|f| f.call((v.clone(),)).unwrap_or_else(|e| e.to_string()))
                                .collect::<Vec<String>>()
                        })
                        .unwrap_or_default()
                        .iter()
                        .map(|x| Cell::from(x.to_owned()))
                        .collect::<Vec<Cell>>();

                    Row::new(cols).style(style.into())
                })
                .collect::<Vec<Row>>()
        })
        .unwrap_or_default();

    let table_constraints: Vec<TuiConstraint> = app_config
        .general()
        .table()
        .col_widths()
        .clone()
        .unwrap_or_default()
        .into_iter()
        .map(|c| c.to_tui(screen_size, layout_size))
        .collect();

    let table = Table::new(rows)
        .widths(&table_constraints)
        .style(app_config.general().table().style().clone().into())
        .highlight_style(app_config.general().focus_ui().style().clone().into())
        .column_spacing(
            app_config
                .general()
                .table()
                .col_spacing()
                .unwrap_or_default(),
        )
        .block(block(
            config,
            format!(
                " {} ({}) ",
                app.pwd(),
                app.directory_buffer()
                    .map(|d| d.total())
                    .unwrap_or_default()
            ),
        ));

    let table = table.clone().header(
        Row::new(
            app_config
                .general()
                .table()
                .header()
                .cols()
                .clone()
                .unwrap_or_default()
                .iter()
                .map(|c| Cell::from(c.format().to_owned().unwrap_or_default()))
                .collect::<Vec<Cell>>(),
        )
        .height(header_height)
        .style(app_config.general().table().header().style().clone().into()),
    );

    f.render_widget(table, layout_size);
}

fn draw_selection<B: Backend>(
    f: &mut Frame<B>,
    _screen_size: Rect,
    layout_size: Rect,
    app: &app::App,
    _: &Lua,
) {
    let panel_config = app.config().general().panel_ui();
    let config = panel_config
        .default()
        .clone()
        .extend(panel_config.selection().clone());
    let selection: Vec<ListItem> = app
        .selection()
        .iter()
        .rev()
        .take((layout_size.height.max(2) - 2).into())
        .rev()
        .map(|n| n.absolute_path().clone())
        .map(ListItem::new)
        .collect();

    let selection_count = selection.len();

    // Selected items
    let selection_list =
        List::new(selection).block(block(config, format!(" Selection ({}) ", selection_count)));

    f.render_widget(selection_list, layout_size);
}

fn draw_help_menu<B: Backend>(
    f: &mut Frame<B>,
    _screen_size: Rect,
    layout_size: Rect,
    app: &app::App,
    _: &Lua,
) {
    let panel_config = app.config().general().panel_ui();
    let config = panel_config
        .default()
        .clone()
        .extend(panel_config.help_menu().clone());
    let help_menu_rows = app
        .mode()
        .help_menu()
        .into_iter()
        .map(|l| match l {
            HelpMenuLine::Paragraph(p) => Row::new([Cell::from(p)].to_vec()),
            HelpMenuLine::KeyMap(k, h) => {
                let remaps = app
                    .mode()
                    .key_bindings()
                    .remaps()
                    .iter()
                    .filter(|(_, maybeto)| maybeto.as_ref().map(|to| to == &k).unwrap_or(false))
                    .map(|(f, _)| f.clone())
                    .collect::<Vec<String>>()
                    .join("|");
                Row::new([Cell::from(k), Cell::from(remaps), Cell::from(h)].to_vec())
            }
        })
        .collect::<Vec<Row>>();

    let read_only_indicator = if app.config().general().read_only().unwrap_or_default() {
        "(r)"
    } else {
        ""
    };

    let help_menu = Table::new(help_menu_rows)
        .block(block(
            config,
            format!(" Help [{}{}] ", &app.mode().name(), read_only_indicator),
        ))
        .widths(&[
            TuiConstraint::Percentage(20),
            TuiConstraint::Percentage(20),
            TuiConstraint::Percentage(60),
        ]);
    f.render_widget(help_menu, layout_size);
}

fn draw_input_buffer<B: Backend>(
    f: &mut Frame<B>,
    _screen_size: Rect,
    layout_size: Rect,
    app: &app::App,
    _: &Lua,
) {
    let panel_config = app.config().general().panel_ui();
    let config = panel_config
        .default()
        .clone()
        .extend(panel_config.input_and_logs().clone());
    let input_buf = Paragraph::new(Spans::from(vec![
        Span::styled(
            app.config()
                .general()
                .prompt()
                .format()
                .clone()
                .unwrap_or_default(),
            app.config().general().prompt().style().clone().into(),
        ),
        Span::raw(app.input_buffer().unwrap_or_else(|| "".into())),
        Span::styled(
            app.config()
                .general()
                .cursor()
                .format()
                .clone()
                .unwrap_or_default(),
            app.config().general().cursor().style().clone().into(),
        ),
    ]))
    .block(block(config, " Input ".into()));
    f.render_widget(input_buf, layout_size);
}

fn draw_sort_n_filter<B: Backend>(
    f: &mut Frame<B>,
    _screen_size: Rect,
    layout_size: Rect,
    app: &app::App,
    _: &Lua,
) {
    let panel_config = app.config().general().panel_ui();
    let config = panel_config
        .default()
        .clone()
        .extend(panel_config.sort_and_filter().clone());
    let ui = app.config().general().sort_and_filter_ui().clone();
    let filter_by = app.explorer_config().filters();
    let sort_by = app.explorer_config().sorters();
    let defaultui = ui.default_identifier();
    let forwardui = defaultui
        .clone()
        .extend(ui.sort_direction_identifiers().forward().clone());
    let reverseui = defaultui
        .clone()
        .extend(ui.sort_direction_identifiers().reverse().clone());

    let mut spans = filter_by
        .iter()
        .map(|f| {
            ui.filter_identifiers()
                .get(&f.filter())
                .map(|u| {
                    let ui = defaultui.clone().extend(u.clone());
                    (
                        Span::styled(
                            ui.format().to_owned().unwrap_or_default(),
                            ui.style().clone().into(),
                        ),
                        Span::styled(f.input().clone(), ui.style().clone().into()),
                    )
                })
                .unwrap_or_else(|| (Span::raw("f"), Span::raw("")))
        })
        .chain(sort_by.iter().map(|s| {
            let direction = if s.reverse() { &reverseui } else { &forwardui };

            ui.sorter_identifiers()
                .get(&s.sorter())
                .map(|u| {
                    let ui = defaultui.clone().extend(u.clone());
                    (
                        Span::styled(
                            ui.format().to_owned().unwrap_or_default(),
                            ui.style().clone().into(),
                        ),
                        Span::styled(
                            direction.format().to_owned().unwrap_or_default(),
                            direction.style().clone().into(),
                        ),
                    )
                })
                .unwrap_or_else(|| (Span::raw("s"), Span::raw("")))
        }))
        .zip(std::iter::repeat(Span::styled(
            ui.separator().format().to_owned().unwrap_or_default(),
            ui.separator().style().clone().into(),
        )))
        .map(|((a, b), c)| vec![a, b, c])
        .flatten()
        .collect::<Vec<Span>>();
    spans.pop();

    let p = Paragraph::new(Spans::from(spans)).block(block(
        config,
        format!(" Sort & filter ({}) ", filter_by.len() + sort_by.len()),
    ));

    f.render_widget(p, layout_size);
}

fn draw_logs<B: Backend>(
    f: &mut Frame<B>,
    _screen_size: Rect,
    layout_size: Rect,
    app: &app::App,
    _: &Lua,
) {
    let panel_config = app.config().general().panel_ui();
    let config = panel_config
        .default()
        .clone()
        .extend(panel_config.input_and_logs().clone());
    let logs_config = app.config().general().logs().clone();
    let logs = if app.logs_hidden() {
        vec![]
    } else {
        app.logs()
            .iter()
            .rev()
            .take(1)
            .rev()
            .map(|l| {
                let time = l.created_at().format("%r");
                match l.level() {
                    app::LogLevel::Info => ListItem::new(format!(
                        "{} | {} | {}",
                        &time,
                        &logs_config.info().format().to_owned().unwrap_or_default(),
                        l.message()
                    ))
                    .style(logs_config.info().style().clone().into()),

                    app::LogLevel::Warning => ListItem::new(format!(
                        "{} | {} | {}",
                        &time,
                        &logs_config
                            .warning()
                            .format()
                            .to_owned()
                            .unwrap_or_default(),
                        l.message()
                    ))
                    .style(logs_config.warning().style().clone().into()),

                    app::LogLevel::Success => ListItem::new(format!(
                        "{} | {} | {}",
                        &time,
                        &logs_config
                            .success()
                            .format()
                            .to_owned()
                            .unwrap_or_default(),
                        l.message()
                    ))
                    .style(logs_config.success().style().clone().into()),

                    app::LogLevel::Error => ListItem::new(format!(
                        "{} | {} | {}",
                        &time,
                        &logs_config.error().format().to_owned().unwrap_or_default(),
                        l.message()
                    ))
                    .style(logs_config.error().style().clone().into()),
                }
            })
            .collect::<Vec<ListItem>>()
    };

    let logs_list = List::new(logs).block(block(config, format!(" Logs ({}) ", app.logs().len())));

    f.render_widget(logs_list, layout_size);
}

pub fn draw_nothing<B: Backend>(
    f: &mut Frame<B>,
    _screen_size: Rect,
    layout_size: Rect,
    app: &app::App,
    _lua: &Lua,
) {
    let panel_config = app.config().general().panel_ui();
    let config = panel_config.default().clone();
    let nothing = Paragraph::new("").block(block(config, "".into()));
    f.render_widget(nothing, layout_size);
}

pub fn draw_layout<B: Backend>(
    layout: Layout,
    f: &mut Frame<B>,
    screen_size: Rect,
    layout_size: Rect,
    app: &app::App,
    lua: &Lua,
) {
    match layout {
        Layout::Nothing => draw_nothing(f, screen_size, layout_size, app, lua),
        Layout::Table => draw_table(f, screen_size, layout_size, app, lua),
        Layout::SortAndFilter => draw_sort_n_filter(f, screen_size, layout_size, app, lua),
        Layout::HelpMenu => draw_help_menu(f, screen_size, layout_size, app, lua),
        Layout::Selection => draw_selection(f, screen_size, layout_size, app, lua),
        Layout::InputAndLogs => {
            if app.input_buffer().is_some() {
                draw_input_buffer(f, screen_size, layout_size, app, lua);
            } else {
                draw_logs(f, screen_size, layout_size, app, lua);
            };
        }
        Layout::Horizontal { config, splits } => {
            let chunks = TuiLayout::default()
                .direction(Direction::Horizontal)
                .constraints(
                    config
                        .constraints()
                        .clone()
                        .unwrap_or_default()
                        .iter()
                        .map(|c| c.to_tui(screen_size, layout_size))
                        .collect::<Vec<TuiConstraint>>(),
                )
                .horizontal_margin(
                    config
                        .horizontal_margin()
                        .or_else(|| config.margin())
                        .unwrap_or_default(),
                )
                .vertical_margin(
                    config
                        .vertical_margin()
                        .or_else(|| config.margin())
                        .unwrap_or_default(),
                )
                .split(layout_size);

            splits
                .into_iter()
                .zip(chunks.into_iter())
                .for_each(|(split, chunk)| draw_layout(split, f, screen_size, chunk, app, lua));
        }

        Layout::Vertical { config, splits } => {
            let chunks = TuiLayout::default()
                .direction(Direction::Vertical)
                .constraints(
                    config
                        .constraints()
                        .clone()
                        .unwrap_or_default()
                        .iter()
                        .map(|c| c.to_tui(screen_size, layout_size))
                        .collect::<Vec<TuiConstraint>>(),
                )
                .horizontal_margin(
                    config
                        .horizontal_margin()
                        .or_else(|| config.margin())
                        .unwrap_or_default(),
                )
                .vertical_margin(
                    config
                        .vertical_margin()
                        .or_else(|| config.margin())
                        .unwrap_or_default(),
                )
                .split(layout_size);

            splits
                .into_iter()
                .zip(chunks.into_iter())
                .for_each(|(split, chunk)| draw_layout(split, f, screen_size, chunk, app, lua));
        }
    }
}

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &app::App, lua: &Lua) {
    let screen_size = f.size();
    let layout = app.layout().clone();

    draw_layout(layout, f, screen_size, screen_size, app, lua);
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::config;
    use tui::style::Color;

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
            a.clone().extend(b.clone()),
            Style {
                fg: Some(Color::Red),
                bg: Some(Color::Blue),
                add_modifiers: modifier(Modifier::Bold),
                sub_modifiers: modifier(Modifier::Dim),
            }
        );

        assert_eq!(
            b.clone().extend(a.clone()),
            Style {
                fg: Some(Color::Red),
                bg: Some(Color::Blue),
                add_modifiers: modifier(Modifier::Bold),
                sub_modifiers: modifier(Modifier::Dim),
            }
        );

        assert_eq!(
            a.clone().extend(c.clone()),
            Style {
                fg: Some(Color::Cyan),
                bg: Some(Color::Magenta),
                add_modifiers: modifier(Modifier::CrossedOut),
                sub_modifiers: modifier(Modifier::Italic),
            }
        );

        assert_eq!(
            c.clone().extend(a.clone()),
            Style {
                fg: Some(Color::Red),
                bg: Some(Color::Magenta),
                add_modifiers: modifier(Modifier::Bold),
                sub_modifiers: modifier(Modifier::Italic),
            }
        );
    }

    #[test]
    fn test_extend_ui_config() {
        let a = config::UiConfig {
            prefix: Some("a".to_string()),
            suffix: None,
            style: Style {
                fg: Some(Color::Red),
                bg: None,
                add_modifiers: modifier(Modifier::Bold),
                sub_modifiers: None,
            },
        };

        let b = config::UiConfig {
            prefix: None,
            suffix: Some("b".to_string()),
            style: Style {
                fg: None,
                bg: Some(Color::Blue),
                add_modifiers: None,
                sub_modifiers: modifier(Modifier::Dim),
            },
        };

        let c = config::UiConfig {
            prefix: Some("cp".to_string()),
            suffix: Some("cs".to_string()),
            style: Style {
                fg: Some(Color::Cyan),
                bg: Some(Color::Magenta),
                add_modifiers: modifier(Modifier::CrossedOut),
                sub_modifiers: modifier(Modifier::Italic),
            },
        };

        assert_eq!(
            a.clone().extend(b.clone()),
            config::UiConfig {
                prefix: Some("a".to_string()),
                suffix: Some("b".to_string()),
                style: Style {
                    fg: Some(Color::Red),
                    bg: Some(Color::Blue),
                    add_modifiers: modifier(Modifier::Bold),
                    sub_modifiers: modifier(Modifier::Dim),
                },
            }
        );

        assert_eq!(
            b.clone().extend(a.clone()),
            config::UiConfig {
                prefix: Some("a".to_string()),
                suffix: Some("b".to_string()),
                style: Style {
                    fg: Some(Color::Red),
                    bg: Some(Color::Blue),
                    add_modifiers: modifier(Modifier::Bold),
                    sub_modifiers: modifier(Modifier::Dim),
                },
            }
        );

        assert_eq!(
            a.clone().extend(c.clone()),
            config::UiConfig {
                prefix: Some("cp".to_string()),
                suffix: Some("cs".to_string()),
                style: Style {
                    fg: Some(Color::Cyan),
                    bg: Some(Color::Magenta),
                    add_modifiers: modifier(Modifier::CrossedOut),
                    sub_modifiers: modifier(Modifier::Italic),
                },
            }
        );
    }
}
