use crate::app;
use crate::app::HelpMenuLine;
use crate::app::Node;
use handlebars::Handlebars;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::HashMap;
use tui::backend::Backend;
use tui::layout::Rect;
use tui::layout::{Constraint as TuiConstraint, Direction, Layout};
use tui::style::{Color, Style};
use tui::text::{Span, Spans};
use tui::widgets::{
    Block, Borders, Cell, List, ListItem, ListState, Paragraph, Row, Table, TableState,
};
use tui::Frame;

const TOTAL_ROWS: usize = 50;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct NodeUiMetadata {
    // From Node
    pub parent: String,
    pub relative_path: String,
    pub absolute_path: String,
    pub extension: String,
    pub is_symlink: bool,
    pub is_dir: bool,
    pub is_file: bool,
    pub is_readonly: bool,
    pub mime_essence: String,

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
            parent: node.parent.clone(),
            relative_path: node.relative_path.clone(),
            absolute_path: node.absolute_path.clone(),
            extension: node.extension.clone(),
            is_symlink: node.is_symlink,
            is_dir: node.is_dir,
            is_file: node.is_file,
            is_readonly: node.is_readonly,
            mime_essence: node.mime_essence.clone(),
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

    let rows = app
        .directory_buffer()
        .map(|dir| {
            let offset = (
                dir.focus.max(TOTAL_ROWS) - TOTAL_ROWS,
                dir.focus.max(TOTAL_ROWS),
            );

            dir.nodes
                .iter()
                .enumerate()
                .skip_while(|(i, _)| *i < offset.0)
                .take_while(|(i, _)| *i <= offset.1)
                .map(|(index, node)| {
                    let is_focused = dir.focus == index;

                    // TODO : Optimize
                    let is_selected = app.selection().contains(&node);

                    let ui = if is_focused {
                        &config.general.focus_ui
                    } else if is_selected {
                        &config.general.selection_ui
                    } else {
                        &config.general.default_ui
                    };

                    let is_first = index == 0;
                    let is_last = index == dir.total.max(1) - 1;

                    let tree = config
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

                    let node_type = config
                        .node_types
                        .special
                        .get(&node.relative_path)
                        .or_else(|| config.node_types.extension.get(&node.extension))
                        .or_else(|| config.node_types.mime_essence.get(&node.mime_essence))
                        .unwrap_or_else(|| {
                            if node.is_symlink {
                                &config.node_types.symlink
                            } else if node.is_dir {
                                &config.node_types.directory
                            } else {
                                &config.node_types.file
                            }
                        });

                    let (relative_index, is_before_focus, is_after_focus) =
                        match dir.focus.cmp(&index) {
                            Ordering::Greater => (dir.focus - index, true, false),
                            Ordering::Less => (index - dir.focus, false, true),
                            Ordering::Equal => (0, false, false),
                        };

                    let meta = NodeUiMetadata::new(
                        &node,
                        index,
                        relative_index,
                        is_before_focus,
                        is_after_focus,
                        tree.unwrap_or_default(),
                        ui.prefix.to_owned().unwrap_or_default(),
                        ui.suffix.to_owned().unwrap_or_default(),
                        is_selected,
                        is_focused,
                        dir.total,
                        node_type.meta.clone(),
                    );

                    let cols = hb
                        .render(app::TEMPLATE_TABLE_ROW, &meta)
                        .ok()
                        .unwrap_or_else(|| app::UNSUPPORTED_STR.into())
                        .split('\t')
                        .map(|x| Cell::from(x.to_string()))
                        .collect::<Vec<Cell>>();

                    let style = if is_focused {
                        config.general.focus_ui.style
                    } else if is_selected {
                        config.general.selection_ui.style
                    } else {
                        config
                            .node_types
                            .special
                            .get(&node.relative_path)
                            .or_else(|| config.node_types.extension.get(&node.extension))
                            .or_else(|| config.node_types.mime_essence.get(&node.mime_essence))
                            .unwrap_or_else(|| {
                                if node.is_symlink {
                                    &config.node_types.symlink
                                } else if node.is_dir {
                                    &config.node_types.directory
                                } else {
                                    &config.node_types.file
                                }
                            })
                            .style
                    };

                    Row::new(cols).style(style.into())
                })
                .collect::<Vec<Row>>()
        })
        .unwrap_or_default();

    let table_constraints: Vec<TuiConstraint> = config
        .general
        .table
        .col_widths
        .clone()
        .unwrap_or_default()
        .into_iter()
        .map(|c| c.into())
        .collect();

    let table = Table::new(rows)
        .widths(&table_constraints)
        .style(config.general.table.style.into())
        .highlight_style(config.general.focus_ui.style.into())
        .column_spacing(config.general.table.col_spacing.unwrap_or_default())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!(" {} ", app.pwd())),
        );

    let table = table.clone().header(
        Row::new(
            config
                .general
                .table
                .header
                .cols
                .unwrap_or_default()
                .iter()
                .map(|c| Cell::from(c.format.to_owned().unwrap_or_default()))
                .collect::<Vec<Cell>>(),
        )
        .height(config.general.table.header.height.unwrap_or_default())
        .style(config.general.table.header.style.into()),
    );

    let mut table_state = TableState::default();
    table_state.select(app.directory_buffer().map(|dir| dir.focus));

    f.render_stateful_widget(table, rect, &mut table_state);
}

fn draw_selection<B: Backend>(f: &mut Frame<B>, rect: Rect, app: &app::App, _: &Handlebars) {
    let selection: Vec<ListItem> = app
        .selection()
        .iter()
        .map(|n| n.absolute_path.clone())
        .map(ListItem::new)
        .collect();

    let selection_count = selection.len();

    // Selected items
    let selection_list = List::new(selection).block(
        Block::default()
            .borders(Borders::ALL)
            .title(format!(" Selection ({}) ", selection_count)),
    );

    let mut list_state = ListState::default();
    if selection_count > 0 {
        list_state.select(Some(selection_count.max(1) - 1));
    }
    f.render_stateful_widget(selection_list, rect, &mut list_state);
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
                    .key_bindings
                    .remaps
                    .iter()
                    .filter(|(_, t)| t == &&k)
                    .map(|(f, _)| f.clone())
                    .collect::<Vec<String>>()
                    .join("|");
                Row::new([Cell::from(k), Cell::from(remaps), Cell::from(h)].to_vec())
            }
        })
        .collect::<Vec<Row>>();

    let help_menu = Table::new(help_menu_rows)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!(" Help [{}] ", &app.mode().name)),
        )
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
                .general
                .prompt
                .format
                .clone()
                .unwrap_or_default(),
            app.config().general.prompt.style.into(),
        ),
        Span::raw(app.input_buffer().unwrap_or_else(|| "".into())),
        Span::styled(
            app.config()
                .general
                .cursor
                .format
                .clone()
                .unwrap_or_default(),
            app.config().general.cursor.style.into(),
        ),
    ]))
    .block(Block::default().borders(Borders::ALL).title(" Input "));
    f.render_widget(input_buf, rect);
}

fn draw_logs<B: Backend>(f: &mut Frame<B>, rect: Rect, app: &app::App, _: &Handlebars) {
    let logs = app
        .logs()
        .iter()
        .rev()
        .take(1)
        .rev()
        .map(|l| match &l.level {
            app::LogLevel::Info => {
                ListItem::new(l.message.clone()).style(Style::default().fg(Color::Gray))
            }
            app::LogLevel::Success => {
                ListItem::new(l.message.clone()).style(Style::default().fg(Color::Green))
            }
            app::LogLevel::Error => {
                ListItem::new(l.message.clone()).style(Style::default().fg(Color::Red))
            }
        })
        .collect::<Vec<ListItem>>();

    let logs_list = List::new(logs).block(Block::default().borders(Borders::ALL).title(" Logs "));

    f.render_widget(logs_list, rect);
}

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &app::App, hb: &Handlebars) {
    let rect = f.size();

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([TuiConstraint::Percentage(70), TuiConstraint::Percentage(30)].as_ref())
        .split(rect);

    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                TuiConstraint::Length(rect.height - 3),
                TuiConstraint::Length(3),
            ]
            .as_ref(),
        )
        .split(chunks[0]);

    draw_table(f, left_chunks[0], app, hb);

    if app.input_buffer().is_some() {
        draw_input_buffer(f, left_chunks[1], app, hb);
    } else {
        draw_logs(f, left_chunks[1], app, hb);
    };

    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([TuiConstraint::Percentage(50), TuiConstraint::Percentage(50)].as_ref())
        .split(chunks[1]);

    draw_selection(f, right_chunks[0], app, hb);
    draw_help_menu(f, right_chunks[1], app, hb);
}
