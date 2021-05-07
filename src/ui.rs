use crate::app;
use crate::app::HelpMenuLine;
use crate::app::{Node, ResolvedNode};
use crate::config::Layout;
use handlebars::Handlebars;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::env;
use tui::backend::Backend;
use tui::layout::Rect;
use tui::layout::{Constraint as TuiConstraint, Direction, Layout as TuiLayout};
use tui::style::{Color, Modifier, Style as TuiStyle};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, Cell, List, ListItem, Paragraph, Row, Table};
use tui::Frame;

lazy_static! {
    pub static ref NO_COLOR: bool = env::var("NO_COLOR").ok().map(|_| true).unwrap_or(false);
    pub static ref DEFAULT_STYLE: TuiStyle = TuiStyle::default();
}

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
        if *NO_COLOR {
            *DEFAULT_STYLE
        } else {
            TuiStyle {
                fg: self.fg,
                bg: self.bg,
                add_modifier: self.add_modifier.unwrap_or_else(Modifier::empty),
                sub_modifier: self.sub_modifier.unwrap_or_else(Modifier::empty),
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResolvedNodeUiMetadata {
    absolute_path: String,
    extension: String,
    is_dir: bool,
    is_file: bool,
    is_readonly: bool,
    mime_essence: String,
    size: u64,
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
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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

fn draw_table<B: Backend>(f: &mut Frame<B>, rect: Rect, app: &app::App, hb: &Handlebars) {
    let config = app.config().to_owned();
    let header_height = config.general().table().header().height().unwrap_or(1);
    let height: usize = (rect.height.max(header_height + 2) - (header_height + 2)).into();

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

                    let tree = config
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

                    let node_type = config
                        .node_types()
                        .special()
                        .get(node.relative_path())
                        .or_else(|| config.node_types().extension().get(node.extension()))
                        .or_else(|| config.node_types().mime_essence().get(node.mime_essence()))
                        .unwrap_or_else(|| {
                            if node.is_symlink() {
                                &config.node_types().symlink()
                            } else if node.is_dir() {
                                &config.node_types().directory()
                            } else {
                                &config.node_types().file()
                            }
                        });

                    let (relative_index, is_before_focus, is_after_focus) =
                        match dir.focus().cmp(&index) {
                            Ordering::Greater => (dir.focus() - index, true, false),
                            Ordering::Less => (index - dir.focus(), false, true),
                            Ordering::Equal => (0, false, false),
                        };

                    let (mut prefix, mut suffix, mut style) = {
                        let ui = config.general().default_ui().clone();
                        (
                            ui.prefix().clone(),
                            ui.suffix().clone(),
                            ui.style().extend(node_type.style()),
                        )
                    };

                    if is_selected {
                        let ui = config.general().selection_ui().clone();
                        prefix = ui.prefix().clone().or(prefix);
                        suffix = ui.suffix().clone().or(suffix);
                        style = style.extend(ui.style());
                    };

                    if is_focused {
                        let ui = config.general().focus_ui().clone();
                        prefix = ui.prefix().clone().or(prefix);
                        suffix = ui.suffix().clone().or(suffix);
                        style = style.extend(ui.style());
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

                    let cols = hb
                        .render(app::TEMPLATE_TABLE_ROW, &meta)
                        .ok()
                        .unwrap_or_else(|| app::UNSUPPORTED_STR.into())
                        .split('\t')
                        .map(|x| Cell::from(x.to_string()))
                        .collect::<Vec<Cell>>();

                    Row::new(cols).style(style.into())
                })
                .collect::<Vec<Row>>()
        })
        .unwrap_or_default();

    let table_constraints: Vec<TuiConstraint> = config
        .general()
        .table()
        .col_widths()
        .clone()
        .unwrap_or_default()
        .into_iter()
        .map(|c| c.into())
        .collect();

    let table = Table::new(rows)
        .widths(&table_constraints)
        .style(config.general().table().style().into())
        .highlight_style(config.general().focus_ui().style().into())
        .column_spacing(config.general().table().col_spacing().unwrap_or_default())
        .block(Block::default().borders(Borders::ALL).title(format!(
            " {} ({}) ",
            app.pwd(),
            app.directory_buffer().map(|d| d.total()).unwrap_or_default()
        )));

    let table = table.clone().header(
        Row::new(
            config
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
        .style(config.general().table().header().style().into()),
    );

    f.render_widget(table, rect);
}

fn draw_selection<B: Backend>(f: &mut Frame<B>, rect: Rect, app: &app::App, _: &Handlebars) {
    let selection: Vec<ListItem> = app
        .selection()
        .iter()
        .rev()
        .take((rect.height.max(2) - 2).into())
        .rev()
        .map(|n| n.absolute_path().clone())
        .map(ListItem::new)
        .collect();

    let selection_count = selection.len();

    // Selected items
    let selection_list = List::new(selection).block(
        Block::default()
            .borders(Borders::ALL)
            .title(format!(" Selection ({}) ", selection_count)),
    );

    f.render_widget(selection_list, rect);
}

fn draw_help_menu<B: Backend>(f: &mut Frame<B>, rect: Rect, app: &app::App, _: &Handlebars) {
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
                    .filter(|(_, t)| t == &&k)
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
        .block(Block::default().borders(Borders::ALL).title(format!(
            " Help [{}{}] ",
            &app.mode().name(),
            read_only_indicator
        )))
        .widths(&[
            TuiConstraint::Percentage(20),
            TuiConstraint::Percentage(20),
            TuiConstraint::Percentage(60),
        ]);
    f.render_widget(help_menu, rect);
}

fn draw_input_buffer<B: Backend>(f: &mut Frame<B>, rect: Rect, app: &app::App, _: &Handlebars) {
    let input_buf = Paragraph::new(Spans::from(vec![
        Span::styled(
            app.config()
                .general()
                .prompt()
                .format()
                .clone()
                .unwrap_or_default(),
            app.config().general().prompt().style().into(),
        ),
        Span::raw(app.input_buffer().unwrap_or_else(|| "".into())),
        Span::styled(
            app.config()
                .general()
                .cursor()
                .format()
                .clone()
                .unwrap_or_default(),
            app.config().general().cursor().style().into(),
        ),
    ]))
    .block(Block::default().borders(Borders::ALL).title(" Input "));
    f.render_widget(input_buf, rect);
}

fn draw_sort_n_filter<B: Backend>(f: &mut Frame<B>, rect: Rect, app: &app::App, _: &Handlebars) {
    let ui = app.config().general().sort_and_filter_ui().clone();
    let filter_by = app.explorer_config().filters();
    let sort_by = app.explorer_config().sorters();
    let forward = Span::styled(
        ui.sort_direction_identifiers()
            .forward()
            .format()
            .to_owned()
            .unwrap_or_default(),
        ui.sort_direction_identifiers().forward().style().into(),
    );

    let reverse = Span::styled(
        ui.sort_direction_identifiers()
            .reverse()
            .format()
            .to_owned()
            .unwrap_or_default(),
        ui.sort_direction_identifiers().reverse().style().into(),
    );

    let mut spans = filter_by
        .iter()
        .map(|f| {
            ui.filter_identifiers()
                .get(&f.filter())
                .map(|u| {
                    (
                        Span::styled(u.format().to_owned().unwrap_or_default(), u.style().into()),
                        Span::raw(f.input().clone()),
                    )
                })
                .unwrap_or_else(|| (Span::raw("f"), Span::raw("")))
        })
        .chain(sort_by.iter().map(|s| {
            let direction = if s.reverse() {
                reverse.clone()
            } else {
                forward.clone()
            };

            ui.sorter_identifiers()
                .get(&s.sorter())
                .map(|u| {
                    (
                        Span::styled(u.format().to_owned().unwrap_or_default(), u.style().into()),
                        direction.clone(),
                    )
                })
                .unwrap_or_else(|| (Span::raw("s"), direction.clone()))
        }))
        .zip(std::iter::repeat(Span::styled(
            ui.separator().format().to_owned().unwrap_or_default(),
            ui.separator().style().into(),
        )))
        .map(|((a, b), c)| vec![a, b, c])
        .flatten()
        .collect::<Vec<Span>>();
    spans.pop();

    let p = Paragraph::new(Spans::from(spans)).block(Block::default().borders(Borders::ALL).title(
        format!(" Sort & filter ({}) ", filter_by.len() + sort_by.len()),
    ));

    f.render_widget(p, rect);
}

fn draw_logs<B: Backend>(f: &mut Frame<B>, rect: Rect, app: &app::App, _: &Handlebars) {
    let config = app.config().general().logs().clone();
    let logs = app
        .logs()
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
                    &config.info().format().to_owned().unwrap_or_default(),
                    l.message()
                ))
                .style(config.info().style().into()),
                app::LogLevel::Success => ListItem::new(format!(
                    "{} | {} | {}",
                    &time,
                    &config.success().format().to_owned().unwrap_or_default(),
                    l.message()
                ))
                .style(config.success().style().into()),
                app::LogLevel::Error => ListItem::new(format!(
                    "{} | {} | {}",
                    &time,
                    &config.error().format().to_owned().unwrap_or_default(),
                    l.message()
                ))
                .style(config.error().style().into()),
            }
        })
        .collect::<Vec<ListItem>>();

    let logs_list = List::new(logs).block(
        Block::default()
            .borders(Borders::ALL)
            .title(format!(" Logs ({}) ", app.logs().len())),
    );

    f.render_widget(logs_list, rect);
}

pub fn draw_layout<B: Backend>(
    layout: Layout,
    f: &mut Frame<B>,
    rect: Rect,
    app: &app::App,
    hb: &Handlebars,
) {
    match layout {
        Layout::Table => draw_table(f, rect, app, hb),
        Layout::SortAndFilter => draw_sort_n_filter(f, rect, app, hb),
        Layout::HelpMenu => draw_help_menu(f, rect, app, hb),
        Layout::Selection => draw_selection(f, rect, app, hb),
        Layout::InputAndLogs => {
            if app.input_buffer().is_some() {
                draw_input_buffer(f, rect, app, hb);
            } else {
                draw_logs(f, rect, app, hb);
            };
        }
        Layout::Horizontal { config, splits } => {
            let chunks = TuiLayout::default()
                .direction(Direction::Horizontal)
                .constraints(
                    config
                        .constraints()
                        .iter()
                        .map(|c| (*c).into())
                        .collect::<Vec<TuiConstraint>>(),
                )
                .margin(config.margin().unwrap_or_default())
                .horizontal_margin(config.horizontal_margin().unwrap_or_default())
                .vertical_margin(config.vertical_margin().unwrap_or_default())
                .split(rect);
            splits
                .into_iter()
                .enumerate()
                .for_each(|(i, s)| draw_layout(s, f, chunks[i], app, hb));
        }

        Layout::Vertical { config, splits } => {
            let chunks = TuiLayout::default()
                .direction(Direction::Vertical)
                .constraints(
                    config
                        .constraints()
                        .iter()
                        .map(|c| (*c).into())
                        .collect::<Vec<TuiConstraint>>(),
                )
                .margin(config.margin().unwrap_or_default())
                .horizontal_margin(config.horizontal_margin().unwrap_or_default())
                .vertical_margin(config.vertical_margin().unwrap_or_default())
                .split(rect);
            splits
                .into_iter()
                .enumerate()
                .for_each(|(i, s)| draw_layout(s, f, chunks[i], app, hb));
        }
    }
}

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &app::App, hb: &Handlebars) {
    let rect = f.size();
    let layout = app.layout().clone();

    draw_layout(layout, f, rect, app, hb);
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::config;
    use tui::style::Color;
    use tui::style::Modifier;

    #[test]
    fn test_extend_style() {
        let a = Style {
            fg: Some(Color::Red),
            bg: None,
            add_modifier: Some(Modifier::BOLD),
            sub_modifier: None,
        };

        let b = Style {
            fg: None,
            bg: Some(Color::Blue),
            add_modifier: None,
            sub_modifier: Some(Modifier::DIM),
        };

        let c = Style {
            fg: Some(Color::Cyan),
            bg: Some(Color::Magenta),
            add_modifier: Some(Modifier::CROSSED_OUT),
            sub_modifier: Some(Modifier::ITALIC),
        };

        assert_eq!(
            a.clone().extend(b.clone()),
            Style {
                fg: Some(Color::Red),
                bg: Some(Color::Blue),
                add_modifier: Some(Modifier::BOLD),
                sub_modifier: Some(Modifier::DIM),
            }
        );

        assert_eq!(
            b.clone().extend(a.clone()),
            Style {
                fg: Some(Color::Red),
                bg: Some(Color::Blue),
                add_modifier: Some(Modifier::BOLD),
                sub_modifier: Some(Modifier::DIM),
            }
        );

        assert_eq!(
            a.clone().extend(c.clone()),
            Style {
                fg: Some(Color::Cyan),
                bg: Some(Color::Magenta),
                add_modifier: Some(Modifier::CROSSED_OUT),
                sub_modifier: Some(Modifier::ITALIC),
            }
        );

        assert_eq!(
            c.clone().extend(a.clone()),
            Style {
                fg: Some(Color::Red),
                bg: Some(Color::Magenta),
                add_modifier: Some(Modifier::BOLD),
                sub_modifier: Some(Modifier::ITALIC),
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
                add_modifier: Some(Modifier::BOLD),
                sub_modifier: None,
            },
        };

        let b = config::UiConfig {
            prefix: None,
            suffix: Some("b".to_string()),
            style: Style {
                fg: None,
                bg: Some(Color::Blue),
                add_modifier: None,
                sub_modifier: Some(Modifier::DIM),
            },
        };

        let c = config::UiConfig {
            prefix: Some("cp".to_string()),
            suffix: Some("cs".to_string()),
            style: Style {
                fg: Some(Color::Cyan),
                bg: Some(Color::Magenta),
                add_modifier: Some(Modifier::CROSSED_OUT),
                sub_modifier: Some(Modifier::ITALIC),
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
                    add_modifier: Some(Modifier::BOLD),
                    sub_modifier: Some(Modifier::DIM),
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
                    add_modifier: Some(Modifier::BOLD),
                    sub_modifier: Some(Modifier::DIM),
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
                    add_modifier: Some(Modifier::CROSSED_OUT),
                    sub_modifier: Some(Modifier::ITALIC),
                },
            }
        );
    }
}
