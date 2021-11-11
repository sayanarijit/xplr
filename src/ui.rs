use crate::app;
use crate::app::{HelpMenuLine, NodeFilterApplicable, NodeSorterApplicable};
use crate::app::{Node, ResolvedNode};
use crate::config::PanelUiConfig;
use crate::lua;
use crate::permissions::Permissions;
use ansi_to_tui::ansi_to_text;
use anyhow::Result;
use indexmap::IndexSet;
use lazy_static::lazy_static;
use mlua::Lua;
use mlua::LuaSerdeExt;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::env;
use std::ops::BitXor;
use tui::backend::Backend;
use tui::layout::Rect as TuiRect;
use tui::layout::{
    Constraint as TuiConstraint, Direction, Layout as TuiLayout,
};
use tui::style::{Color, Modifier as TuiModifier, Style as TuiStyle};
use tui::text::{Span, Spans, Text};
use tui::widgets::{
    Block, Borders as TuiBorders, Cell, List, ListItem, Paragraph, Row, Table,
};
use tui::Frame;

lazy_static! {
    pub static ref NO_COLOR: bool = env::var("NO_COLOR").is_ok();
    pub static ref DEFAULT_STYLE: TuiStyle = TuiStyle::default();
}

fn read_only_indicator(app: &app::App) -> &str {
    if app.config.general.read_only {
        "(r)"
    } else {
        ""
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
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
        self.horizontal_margin =
            other.horizontal_margin.or(self.horizontal_margin);
        self.vertical_margin = other.vertical_margin.or(self.vertical_margin);
        self.constraints = other.constraints.to_owned().or(self.constraints);
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub enum ContentBody {
    /// A paragraph to render
    StaticParagraph { render: String },

    /// A Lua function that returns a paragraph to render
    DynamicParagraph { render: String },

    /// List to render
    StaticList { render: Vec<String> },

    /// A Lua function that returns lines to render
    DynamicList { render: String },

    /// A table to render
    StaticTable {
        widths: Vec<Constraint>,
        col_spacing: Option<u16>,
        render: Vec<Vec<String>>,
    },

    /// A Lua function that returns a table to render
    DynamicTable {
        widths: Vec<Constraint>,
        col_spacing: Option<u16>,
        render: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub enum Layout {
    Nothing,
    Table,
    InputAndLogs,
    Selection,
    HelpMenu,
    SortAndFilter,
    CustomContent {
        title: Option<String>,
        body: ContentBody,
    },
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
                splits: osplits.to_owned(),
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
                splits: osplits.to_owned(),
            },
            (_, other) => other.to_owned(),
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
    pub fn bits(self) -> u32 {
        match self {
            Self::Top => TuiBorders::TOP.bits(),
            Self::Right => TuiBorders::RIGHT.bits(),
            Self::Bottom => TuiBorders::BOTTOM.bits(),
            Self::Left => TuiBorders::LEFT.bits(),
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

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
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
            other.add_modifiers.to_owned().or(self.add_modifiers);
        self.sub_modifiers =
            other.sub_modifiers.to_owned().or(self.sub_modifiers);
        self
    }
}

impl Into<TuiStyle> for Style {
    fn into(self) -> TuiStyle {
        fn xor(modifiers: Option<IndexSet<Modifier>>) -> u16 {
            modifiers
                .unwrap_or_default()
                .into_iter()
                .map(Modifier::bits)
                .fold(0, BitXor::bitxor)
        }
        if *NO_COLOR {
            *DEFAULT_STYLE
        } else {
            TuiStyle {
                fg: self.fg,
                bg: self.bg,
                add_modifier: TuiModifier::from_bits_truncate(xor(
                    self.add_modifiers
                )),
                sub_modifier: TuiModifier::from_bits_truncate(xor(
                    self.sub_modifiers
                )),
            }
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
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
    pub fn to_tui(
        self,
        screen_size: TuiRect,
        layout_size: TuiRect,
    ) -> TuiConstraint {
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
}

impl From<ResolvedNode> for ResolvedNodeUiMetadata {
    fn from(node: ResolvedNode) -> Self {
        Self {
            absolute_path: node.absolute_path.to_owned(),
            extension: node.extension.to_owned(),
            is_dir: node.is_dir,
            is_file: node.is_file,
            is_readonly: node.is_readonly,
            mime_essence: node.mime_essence.to_owned(),
            size: node.size,
            human_size: node.human_size,
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
            parent: node.parent.to_owned(),
            relative_path: node.relative_path.to_owned(),
            absolute_path: node.absolute_path.to_owned(),
            extension: node.extension.to_owned(),
            is_symlink: node.is_symlink,
            is_broken: node.is_broken,
            is_dir: node.is_dir,
            is_file: node.is_file,
            is_readonly: node.is_readonly,
            mime_essence: node.mime_essence.to_owned(),
            size: node.size,
            human_size: node.human_size.to_owned(),
            permissions: node.permissions.to_owned(),
            canonical: node.canonical.to_owned().map(ResolvedNode::into),
            symlink: node.symlink.to_owned().map(ResolvedNode::into),
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
                .borders
                .to_owned()
                .unwrap_or_default()
                .iter()
                .map(|b| b.bits())
                .fold(0, |a, b| (a ^ b)),
        ))
        .title(Span::styled(
            config.title.format.unwrap_or(default_title),
            config.title.style.into(),
        ))
        .style(config.style.into())
}

fn draw_table<B: Backend>(
    f: &mut Frame<B>,
    screen_size: TuiRect,
    layout_size: TuiRect,
    app: &app::App,
    lua: &Lua,
) {
    let panel_config = &app.config.general.panel_ui;
    let config = panel_config.default.to_owned().extend(&panel_config.table);
    let app_config = app.config.to_owned();
    let header_height = app_config.general.table.header.height.unwrap_or(1);
    let height: usize = (layout_size.height.max(header_height + 2)
        - (header_height + 2))
        .into();

    let rows = app
        .directory_buffer
        .as_ref()
        .map(|dir| {
            dir.nodes
                .iter()
                .enumerate()
                .skip(height * (dir.focus / height.max(1)))
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
                        .to_owned()
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

                    let mut me = node.mime_essence.splitn(2, '/');
                    let mimetype: String =
                        me.next().map(|s| s.into()).unwrap_or_default();
                    let mimesub: String =
                        me.next().map(|s| s.into()).unwrap_or_default();

                    let mut node_type = if node.is_symlink {
                        app_config.node_types.symlink.to_owned()
                    } else if node.is_dir {
                        app_config.node_types.directory.to_owned()
                    } else {
                        app_config.node_types.file.to_owned()
                    };

                    if let Some(conf) = app_config
                        .node_types
                        .mime_essence
                        .get(&mimetype)
                        .and_then(|t| t.get(&mimesub).or_else(|| t.get("*")))
                    {
                        node_type = node_type.extend(conf);
                    }

                    if let Some(conf) =
                        app_config.node_types.extension.get(&node.extension)
                    {
                        node_type = node_type.extend(conf);
                    }

                    if let Some(conf) =
                        app_config.node_types.special.get(&node.relative_path)
                    {
                        node_type = node_type.extend(conf);
                    }

                    let (relative_index, is_before_focus, is_after_focus) =
                        match dir.focus.cmp(&index) {
                            Ordering::Greater => {
                                (dir.focus - index, true, false)
                            }
                            Ordering::Less => (index - dir.focus, false, true),
                            Ordering::Equal => (0, false, false),
                        };

                    let (mut prefix, mut suffix, mut style) = {
                        let ui = app_config.general.default_ui.to_owned();
                        (
                            ui.prefix,
                            ui.suffix,
                            ui.style.extend(&node_type.style),
                        )
                    };

                    if is_focused && is_selected {
                        let ui =
                            app_config.general.focus_selection_ui.to_owned();
                        prefix = ui.prefix.to_owned().or(prefix);
                        suffix = ui.suffix.to_owned().or(suffix);
                        style = style.extend(&ui.style);
                    } else if is_selected {
                        let ui = app_config.general.selection_ui.to_owned();
                        prefix = ui.prefix.to_owned().or(prefix);
                        suffix = ui.suffix.to_owned().or(suffix);
                        style = style.extend(&ui.style);
                    } else if is_focused {
                        let ui = app_config.general.focus_ui.to_owned();
                        prefix = ui.prefix.to_owned().or(prefix);
                        suffix = ui.suffix.to_owned().or(suffix);
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
                    );

                    let cols = lua
                        .to_value::<NodeUiMetadata>(&meta)
                        .map(|v| {
                            app_config
                                .general
                                .table
                                .row
                                .cols
                                .to_owned()
                                .unwrap_or_default()
                                .iter()
                                .filter_map(|c| {
                                    c.format.as_ref().map(|f| {
                                        let out: Result<String> =
                                            lua::call(lua, f, v.clone());
                                        match out {
                                            Ok(o) => ansi_to_text(o.bytes())
                                                .unwrap_or_else(|e| {
                                                    Text::raw(format!(
                                                        "{:?}",
                                                        e
                                                    ))
                                                }),
                                            Err(e) => Text::raw(e.to_string()),
                                        }
                                    })
                                })
                                .collect::<Vec<Text>>()
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
        .general
        .table
        .col_widths
        .to_owned()
        .unwrap_or_default()
        .into_iter()
        .map(|c| c.to_tui(screen_size, layout_size))
        .collect();

    let table = Table::new(rows)
        .widths(&table_constraints)
        .style(app_config.general.table.style.to_owned().into())
        .highlight_style(app_config.general.focus_ui.style.to_owned().into())
        .column_spacing(
            app_config.general.table.col_spacing.unwrap_or_default(),
        )
        .block(block(
            config,
            format!(
                " {} ({}) ",
                app.pwd,
                app.directory_buffer
                    .as_ref()
                    .map(|d| d.total)
                    .unwrap_or_default()
            ),
        ));

    let table = table.to_owned().header(
        Row::new(
            app_config
                .general
                .table
                .header
                .cols
                .to_owned()
                .unwrap_or_default()
                .iter()
                .map(|c| Cell::from(c.format.to_owned().unwrap_or_default()))
                .collect::<Vec<Cell>>(),
        )
        .height(header_height)
        .style(app_config.general.table.header.style.to_owned().into()),
    );

    f.render_widget(table, layout_size);
}

fn draw_selection<B: Backend>(
    f: &mut Frame<B>,
    _screen_size: TuiRect,
    layout_size: TuiRect,
    app: &app::App,
    _: &Lua,
) {
    let panel_config = &app.config.general.panel_ui;
    let config = panel_config
        .default
        .to_owned()
        .extend(&panel_config.selection);

    let selection_count = app.selection.len();

    let selection: Vec<ListItem> = app
        .selection
        .iter()
        .rev()
        .take((layout_size.height.max(2) - 2).into())
        .rev()
        .map(|n| n.absolute_path.to_owned())
        .map(ListItem::new)
        .collect();

    // Selected items
    let selection_list = List::new(selection)
        .block(block(config, format!(" Selection ({}) ", selection_count)));

    f.render_widget(selection_list, layout_size);
}

fn draw_help_menu<B: Backend>(
    f: &mut Frame<B>,
    _screen_size: TuiRect,
    layout_size: TuiRect,
    app: &app::App,
    _: &Lua,
) {
    let panel_config = &app.config.general.panel_ui;
    let config = panel_config
        .default
        .to_owned()
        .extend(&panel_config.help_menu);
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

    let help_menu = Table::new(help_menu_rows)
        .block(block(
            config,
            format!(" Help [{}{}] ", &app.mode.name, read_only_indicator(app)),
        ))
        .widths(if app.config.general.hide_remaps_in_help_menu {
            &[TuiConstraint::Percentage(20), TuiConstraint::Percentage(80)]
        } else {
            &[
                TuiConstraint::Percentage(20),
                TuiConstraint::Percentage(20),
                TuiConstraint::Percentage(60),
            ]
        });
    f.render_widget(help_menu, layout_size);
}

fn draw_input_buffer<B: Backend>(
    f: &mut Frame<B>,
    _screen_size: TuiRect,
    layout_size: TuiRect,
    app: &app::App,
    _: &Lua,
) {
    if let Some(input) = app.input.as_ref() {
        let panel_config = &app.config.general.panel_ui;
        let config = panel_config
            .default
            .to_owned()
            .extend(&panel_config.input_and_logs);

        let cursor_offset_left = if config
            .borders
            .as_ref()
            .map(|b| b.contains(&Border::Left))
            .unwrap_or(false)
        {
            3
        } else {
            2
        };

        let cursor_offset_right = if config
            .borders
            .as_ref()
            .map(|b| b.contains(&Border::Right))
            .unwrap_or(false)
        {
            2
        } else {
            1
        };

        let offset_width = cursor_offset_left + cursor_offset_right;
        let width = layout_size.width.max(offset_width) - offset_width;
        let scroll = (input.cursor() as u16).max(width) - width;

        let input_buf = Paragraph::new(Spans::from(vec![
            Span::styled(
                app.config
                    .general
                    .prompt
                    .format
                    .to_owned()
                    .unwrap_or_default(),
                app.config.general.prompt.style.to_owned().into(),
            ),
            Span::raw(input.value()),
        ]))
        .scroll((0, scroll))
        .block(block(
            config,
            format!(" Input [{}{}] ", app.mode.name, read_only_indicator(app)),
        ));

        f.render_widget(input_buf, layout_size);
        f.set_cursor(
            // Put cursor past the end of the input text
            layout_size.x
                + (input.cursor() as u16).min(width)
                + cursor_offset_left,
            // Move one line down, from the border to the input line
            layout_size.y + 1,
        );
    };
}

fn draw_sort_n_filter<B: Backend>(
    f: &mut Frame<B>,
    _screen_size: TuiRect,
    layout_size: TuiRect,
    app: &app::App,
    _: &Lua,
) {
    let panel_config = &app.config.general.panel_ui;
    let config = panel_config
        .default
        .to_owned()
        .extend(&panel_config.sort_and_filter);
    let ui = app.config.general.sort_and_filter_ui.to_owned();
    let filter_by: &IndexSet<NodeFilterApplicable> =
        &app.explorer_config.filters;
    let sort_by: &IndexSet<NodeSorterApplicable> = &app.explorer_config.sorters;
    let defaultui = &ui.default_identifier;
    let forwardui = defaultui
        .to_owned()
        .extend(&ui.sort_direction_identifiers.forward);
    let reverseui = defaultui
        .to_owned()
        .extend(&ui.sort_direction_identifiers.reverse);

    let mut spans = filter_by
        .iter()
        .map(|f| {
            ui.filter_identifiers
                .get(&f.filter)
                .map(|u| {
                    let ui = defaultui.to_owned().extend(u);
                    (
                        Span::styled(
                            ui.format.to_owned().unwrap_or_default(),
                            ui.style.to_owned().into(),
                        ),
                        Span::styled(f.input.to_owned(), ui.style.into()),
                    )
                })
                .unwrap_or((Span::raw("f"), Span::raw("")))
        })
        .chain(sort_by.iter().map(|s| {
            let direction = if s.reverse { &reverseui } else { &forwardui };

            ui.sorter_identifiers
                .get(&s.sorter)
                .map(|u| {
                    let ui = defaultui.to_owned().extend(u);
                    (
                        Span::styled(
                            ui.format.to_owned().unwrap_or_default(),
                            ui.style.into(),
                        ),
                        Span::styled(
                            direction.format.to_owned().unwrap_or_default(),
                            direction.style.to_owned().into(),
                        ),
                    )
                })
                .unwrap_or((Span::raw("s"), Span::raw("")))
        }))
        .zip(std::iter::repeat(Span::styled(
            ui.separator.format.to_owned().unwrap_or_default(),
            ui.separator.style.to_owned().into(),
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
    _screen_size: TuiRect,
    layout_size: TuiRect,
    app: &app::App,
    _: &Lua,
) {
    let panel_config = &app.config.general.panel_ui;
    let config = panel_config
        .default
        .to_owned()
        .extend(&panel_config.input_and_logs);
    let logs_config = app.config.general.logs.to_owned();
    let logs = if app.logs_hidden {
        vec![]
    } else {
        app.logs
            .iter()
            .rev()
            .take(layout_size.height as usize)
            .map(|l| {
                let time = l.created_at.format("%r");
                match l.level {
                    app::LogLevel::Info => ListItem::new(format!(
                        "{} | {} | {}",
                        &time,
                        &logs_config.info.format.to_owned().unwrap_or_default(),
                        l.message
                    ))
                    .style(logs_config.info.style.to_owned().into()),

                    app::LogLevel::Warning => ListItem::new(format!(
                        "{} | {} | {}",
                        &time,
                        &logs_config
                            .warning
                            .format
                            .to_owned()
                            .unwrap_or_default(),
                        l.message
                    ))
                    .style(logs_config.warning.style.to_owned().into()),

                    app::LogLevel::Success => ListItem::new(format!(
                        "{} | {} | {}",
                        &time,
                        &logs_config
                            .success
                            .format
                            .to_owned()
                            .unwrap_or_default(),
                        l.message
                    ))
                    .style(logs_config.success.style.to_owned().into()),

                    app::LogLevel::Error => ListItem::new(format!(
                        "{} | {} | {}",
                        &time,
                        &logs_config
                            .error
                            .format
                            .to_owned()
                            .unwrap_or_default(),
                        l.message
                    ))
                    .style(logs_config.error.style.to_owned().into()),
                }
            })
            .collect::<Vec<ListItem>>()
    };

    let logs_list = List::new(logs).block(block(
        config,
        format!(
            " Logs ({}) [{}{}] ",
            app.logs.len(),
            app.mode.name,
            read_only_indicator(app),
        ),
    ));

    f.render_widget(logs_list, layout_size);
}

pub fn draw_nothing<B: Backend>(
    f: &mut Frame<B>,
    _screen_size: TuiRect,
    layout_size: TuiRect,
    app: &app::App,
    _lua: &Lua,
) {
    let panel_config = &app.config.general.panel_ui;
    let config = panel_config.default.to_owned();
    let nothing = Paragraph::new("").block(block(config, "".into()));
    f.render_widget(nothing, layout_size);
}

pub fn draw_custom_content<B: Backend>(
    f: &mut Frame<B>,
    screen_size: TuiRect,
    layout_size: TuiRect,
    app: &app::App,
    title: Option<String>,
    body: ContentBody,
    lua: &Lua,
) {
    let config = app.config.general.panel_ui.default.clone();

    match body {
        ContentBody::StaticParagraph { render } => {
            let render = ansi_to_text(render.bytes())
                .unwrap_or_else(|e| Text::raw(e.to_string()));
            let content = Paragraph::new(render).block(block(
                config,
                title.map(|t| format!(" {} ", t)).unwrap_or_default(),
            ));
            f.render_widget(content, layout_size);
        }

        ContentBody::DynamicParagraph { render } => {
            let ctx = ContentRendererArg {
                app: app.to_lua_arg(),
                layout_size: layout_size.into(),
                screen_size: screen_size.into(),
            };

            let render = lua
                .to_value(&ctx)
                .map(|arg| {
                    lua::call_with_cache(lua, &render, arg)
                        .unwrap_or_else(|e| format!("{:?}", e))
                })
                .unwrap_or_else(|e| e.to_string());

            let render = ansi_to_text(render.bytes())
                .unwrap_or_else(|e| Text::raw(e.to_string()));

            let content = Paragraph::new(render).block(block(
                config,
                title.map(|t| format!(" {} ", t)).unwrap_or_default(),
            ));
            f.render_widget(content, layout_size);
        }

        ContentBody::StaticList { render } => {
            let items = render
                .into_iter()
                .map(|item| {
                    ansi_to_text(item.bytes())
                        .unwrap_or_else(|e| Text::raw(e.to_string()))
                })
                .map(ListItem::new)
                .collect::<Vec<ListItem>>();

            let content = List::new(items).block(block(
                config,
                title.map(|t| format!(" {} ", t)).unwrap_or_default(),
            ));
            f.render_widget(content, layout_size);
        }

        ContentBody::DynamicList { render } => {
            let ctx = ContentRendererArg {
                app: app.to_lua_arg(),
                layout_size: layout_size.into(),
                screen_size: screen_size.into(),
            };

            let items = lua
                .to_value(&ctx)
                .map(|arg| {
                    lua::call_with_cache(lua, &render, arg)
                        .unwrap_or_else(|e| vec![format!("{:?}", e)])
                })
                .unwrap_or_else(|e| vec![e.to_string()])
                .into_iter()
                .map(|item| {
                    ansi_to_text(item.bytes())
                        .unwrap_or_else(|e| Text::raw(e.to_string()))
                })
                .map(ListItem::new)
                .collect::<Vec<ListItem>>();

            let content = List::new(items).block(block(
                config,
                title.map(|t| format!(" {} ", t)).unwrap_or_default(),
            ));
            f.render_widget(content, layout_size);
        }

        ContentBody::StaticTable {
            widths,
            col_spacing,
            render,
        } => {
            let rows = render
                .into_iter()
                .map(|cols| {
                    Row::new(
                        cols.into_iter()
                            .map(|item| {
                                ansi_to_text(item.bytes()).unwrap_or_else(|e| {
                                    Text::raw(e.to_string())
                                })
                            })
                            .map(Cell::from)
                            .collect::<Vec<Cell>>(),
                    )
                })
                .collect::<Vec<Row>>();

            let widths = widths
                .into_iter()
                .map(|w| w.to_tui(screen_size, layout_size))
                .collect::<Vec<TuiConstraint>>();

            let content = Table::new(rows)
                .widths(&widths)
                .column_spacing(col_spacing.unwrap_or(1))
                .block(block(
                    config,
                    title.map(|t| format!(" {} ", t)).unwrap_or_default(),
                ));

            f.render_widget(content, layout_size);
        }

        ContentBody::DynamicTable {
            widths,
            col_spacing,
            render,
        } => {
            let ctx = ContentRendererArg {
                app: app.to_lua_arg(),
                layout_size: layout_size.into(),
                screen_size: screen_size.into(),
            };

            let rows = lua
                .to_value(&ctx)
                .map(|arg| {
                    lua::call_with_cache(lua, &render, arg)
                        .unwrap_or_else(|e| vec![vec![format!("{:?}", e)]])
                })
                .unwrap_or_else(|e| vec![vec![e.to_string()]])
                .into_iter()
                .map(|cols| {
                    Row::new(
                        cols.into_iter()
                            .map(|item| {
                                ansi_to_text(item.bytes()).unwrap_or_else(|e| {
                                    Text::raw(e.to_string())
                                })
                            })
                            .map(Cell::from)
                            .collect::<Vec<Cell>>(),
                    )
                })
                .collect::<Vec<Row>>();

            let widths = widths
                .into_iter()
                .map(|w| w.to_tui(screen_size, layout_size))
                .collect::<Vec<TuiConstraint>>();

            let mut content = Table::new(rows).widths(&widths).block(block(
                config,
                title.map(|t| format!(" {} ", t)).unwrap_or_default(),
            ));

            if let Some(col_spacing) = col_spacing {
                content = content.column_spacing(col_spacing);
            };

            f.render_widget(content, layout_size);
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Serialize, Deserialize)]
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
    app: app::CallLuaArg,
    screen_size: Rect,
    layout_size: Rect,
}

pub fn draw_layout<B: Backend>(
    layout: Layout,
    f: &mut Frame<B>,
    screen_size: TuiRect,
    layout_size: TuiRect,
    app: &app::App,
    lua: &Lua,
) {
    match layout {
        Layout::Nothing => draw_nothing(f, screen_size, layout_size, app, lua),
        Layout::Table => draw_table(f, screen_size, layout_size, app, lua),
        Layout::SortAndFilter => {
            draw_sort_n_filter(f, screen_size, layout_size, app, lua)
        }
        Layout::HelpMenu => {
            draw_help_menu(f, screen_size, layout_size, app, lua)
        }
        Layout::Selection => {
            draw_selection(f, screen_size, layout_size, app, lua)
        }
        Layout::InputAndLogs => {
            if app.input.is_some() {
                draw_input_buffer(f, screen_size, layout_size, app, lua);
            } else {
                draw_logs(f, screen_size, layout_size, app, lua);
            };
        }
        Layout::CustomContent { title, body } => draw_custom_content(
            f,
            screen_size,
            layout_size,
            app,
            title,
            body,
            lua,
        ),
        Layout::Horizontal { config, splits } => {
            let chunks = TuiLayout::default()
                .direction(Direction::Horizontal)
                .constraints(
                    config
                        .constraints
                        .to_owned()
                        .unwrap_or_default()
                        .iter()
                        .map(|c| c.to_tui(screen_size, layout_size))
                        .collect::<Vec<TuiConstraint>>(),
                )
                .horizontal_margin(
                    config
                        .horizontal_margin
                        .or(config.margin)
                        .unwrap_or_default(),
                )
                .vertical_margin(
                    config
                        .vertical_margin
                        .or(config.margin)
                        .unwrap_or_default(),
                )
                .split(layout_size);

            splits.into_iter().zip(chunks.into_iter()).for_each(
                |(split, chunk)| {
                    draw_layout(split, f, screen_size, chunk, app, lua)
                },
            );
        }

        Layout::Vertical { config, splits } => {
            let chunks = TuiLayout::default()
                .direction(Direction::Vertical)
                .constraints(
                    config
                        .constraints
                        .to_owned()
                        .unwrap_or_default()
                        .iter()
                        .map(|c| c.to_tui(screen_size, layout_size))
                        .collect::<Vec<TuiConstraint>>(),
                )
                .horizontal_margin(
                    config
                        .horizontal_margin
                        .or(config.margin)
                        .unwrap_or_default(),
                )
                .vertical_margin(
                    config
                        .vertical_margin
                        .or(config.margin)
                        .unwrap_or_default(),
                )
                .split(layout_size);

            splits.into_iter().zip(chunks.into_iter()).for_each(
                |(split, chunk)| {
                    draw_layout(split, f, screen_size, chunk, app, lua)
                },
            );
        }
    }
}

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &app::App, lua: &Lua) {
    let screen_size = f.size();
    let layout = app.mode.layout.as_ref().unwrap_or(&app.layout).to_owned();

    draw_layout(layout, f, screen_size, screen_size, app, lua);
}

#[cfg(test)]
mod tests {
    use super::*;
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
            a.to_owned().extend(&b),
            Style {
                fg: Some(Color::Red),
                bg: Some(Color::Blue),
                add_modifiers: modifier(Modifier::Bold),
                sub_modifiers: modifier(Modifier::Dim),
            }
        );

        assert_eq!(
            b.to_owned().extend(&a),
            Style {
                fg: Some(Color::Red),
                bg: Some(Color::Blue),
                add_modifiers: modifier(Modifier::Bold),
                sub_modifiers: modifier(Modifier::Dim),
            }
        );

        assert_eq!(
            a.to_owned().extend(&c),
            Style {
                fg: Some(Color::Cyan),
                bg: Some(Color::Magenta),
                add_modifiers: modifier(Modifier::CrossedOut),
                sub_modifiers: modifier(Modifier::Italic),
            }
        );

        assert_eq!(
            c.to_owned().extend(&a),
            Style {
                fg: Some(Color::Red),
                bg: Some(Color::Magenta),
                add_modifiers: modifier(Modifier::Bold),
                sub_modifiers: modifier(Modifier::Italic),
            }
        );
    }
}
