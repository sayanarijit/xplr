use crate::app;
use crate::app::Node;
use handlebars::Handlebars;
use serde::{Deserialize, Serialize};
use tui::backend::Backend;
use tui::layout::Rect;
use tui::layout::{Constraint as TUIConstraint, Direction, Layout};
use tui::widgets::{
    Block, Borders, Cell, List, ListItem, ListState, Paragraph, Row, Table, TableState,
};
use tui::Frame;

const TOTAL_ROWS: usize = 50;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct NodeUIMetadata {
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
    pub icon: String,
    pub prefix: String,
    pub suffix: String,
    pub is_selected: bool,
    pub is_focused: bool,
    pub total: usize,
}

impl NodeUIMetadata {
    fn new(
        node: &Node,
        index: usize,
        relative_index: usize,
        is_before_focus: bool,
        is_after_focus: bool,
        tree: String,
        icon: String,
        prefix: String,
        suffix: String,
        is_selected: bool,
        is_focused: bool,
        total: usize,
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
            icon,
            prefix,
            suffix,
            is_selected,
            is_focused,
            total,
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
                    let is_selected = app.selected().contains(&node);

                    let ui = if is_focused {
                        &config.general.focused_ui
                    } else if is_selected {
                        &config.general.selected_ui
                    } else {
                        &config.general.normal_ui
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
                                t.2.format.clone()
                            } else if is_first {
                                t.0.format.clone()
                            } else {
                                t.1.format.clone()
                            }
                        })
                        .unwrap_or_default();

                    let filetype = config
                        .filetypes
                        .special
                        .get(&node.relative_path)
                        .or_else(|| config.filetypes.extension.get(&node.extension))
                        .or_else(|| config.filetypes.mime_essence.get(&node.mime_essence))
                        .unwrap_or_else(|| {
                            if node.is_symlink {
                                &config.filetypes.symlink
                            } else if node.is_dir {
                                &config.filetypes.directory
                            } else {
                                &config.filetypes.file
                            }
                        });

                    let (relative_index, is_before_focus, is_after_focus) = if dir.focus > index {
                        (dir.focus - index, true, false)
                    } else if dir.focus < index {
                        (index - dir.focus, false, true)
                    } else {
                        (0, false, false)
                    };

                    let meta = NodeUIMetadata::new(
                        &node,
                        index,
                        relative_index,
                        is_before_focus,
                        is_after_focus,
                        tree,
                        filetype.icon.clone(),
                        ui.prefix.clone(),
                        ui.suffix.clone(),
                        is_selected,
                        is_focused,
                        dir.total,
                    );

                    let cols = hb
                        .render(app::TEMPLATE_TABLE_ROW, &meta)
                        .ok()
                        .unwrap_or_else(|| app::UNSUPPORTED_STR.into())
                        .split("\t")
                        .map(|x| Cell::from(x.to_string()))
                        .collect::<Vec<Cell>>();

                    let style = if is_focused {
                        config.general.focused_ui.style
                    } else if is_selected {
                        config.general.selected_ui.style
                    } else {
                        config
                            .filetypes
                            .special
                            .get(&node.relative_path)
                            .or_else(|| config.filetypes.extension.get(&node.extension))
                            .or_else(|| config.filetypes.mime_essence.get(&node.mime_essence))
                            .unwrap_or_else(|| {
                                if node.is_symlink {
                                    &config.filetypes.symlink
                                } else if node.is_dir {
                                    &config.filetypes.directory
                                } else {
                                    &config.filetypes.file
                                }
                            })
                            .style
                    };

                    Row::new(cols).style(style)
                })
                .collect::<Vec<Row>>()
        })
        .unwrap_or_default();

    let table_constraints: Vec<TUIConstraint> = config
        .general
        .table
        .col_widths
        .clone()
        .into_iter()
        .map(|c| c.into())
        .collect();

    let table = Table::new(rows)
        .widths(&table_constraints)
        .style(config.general.table.style)
        .highlight_style(config.general.focused_ui.style)
        .column_spacing(config.general.table.col_spacing)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!(" {} ", app.pwd())),
        );

    let table = config
        .general
        .table
        .header
        .clone()
        .map(|h| {
            table.clone().header(
                Row::new(
                    h.cols
                        .iter()
                        .map(|c| Cell::from(c.format.to_owned()))
                        .collect::<Vec<Cell>>(),
                )
                .height(h.height)
                .style(h.style),
            )
        })
        .unwrap_or_else(|| table.clone());

    let mut table_state = TableState::default();
    table_state.select(app.directory_buffer().map(|dir| dir.focus));

    f.render_stateful_widget(table, rect, &mut table_state);
}

fn draw_selected<B: Backend>(f: &mut Frame<B>, rect: Rect, app: &app::App, _: &Handlebars) {
    let selected: Vec<ListItem> = app
        .selected()
        .iter()
        .map(|n| n.absolute_path.clone())
        .map(ListItem::new)
        .collect();

    let selected_count = selected.len();

    // Selected items
    let selected_list = List::new(selected).block(
        Block::default()
            .borders(Borders::ALL)
            .title(format!(" Selected ({}) ", selected_count)),
    );

    let mut list_state = ListState::default();
    if selected_count > 0 {
        list_state.select(Some(selected_count.max(1) - 1));
    }
    f.render_stateful_widget(selected_list, rect, &mut list_state);
}

fn draw_help_menu<B: Backend>(f: &mut Frame<B>, rect: Rect, app: &app::App, _: &Handlebars) {
    // Help menu
    let mode = app.mode();
    let extra_help_lines = mode
        .extra_help
        .clone()
        .map(|e| e.lines().map(|l| l.to_string()).collect::<Vec<String>>());

    let help_menu_rows: Vec<Row> = mode
        .help
        .clone()
        .map(|h| {
            h.lines()
                .map(|l| Row::new(vec![Cell::from(l.to_string())]))
                .collect()
        })
        .unwrap_or_else(|| {
            extra_help_lines
                .unwrap_or_default()
                .into_iter()
                .map(|l| Row::new(vec![Cell::from(l)]))
                .chain(mode.key_bindings.on_key.iter().filter_map(|(k, a)| {
                    a.help.clone().map(|h| {
                        Row::new(vec![Cell::from(k.to_string()), Cell::from(h.to_string())])
                    })
                }))
                .chain(
                    mode.key_bindings
                        .on_alphabet
                        .iter()
                        .map(|a| ("a-Z", a.help.clone()))
                        .filter_map(|(k, mh)| {
                            mh.map(|h| {
                                Row::new(vec![Cell::from(k.to_string()), Cell::from(h.to_string())])
                            })
                        }),
                )
                .chain(
                    mode.key_bindings
                        .on_number
                        .iter()
                        .map(|a| ("0-9", a.help.clone()))
                        .filter_map(|(k, mh)| {
                            mh.map(|h| {
                                Row::new(vec![Cell::from(k.to_string()), Cell::from(h.to_string())])
                            })
                        }),
                )
                .chain(
                    mode.key_bindings
                        .on_special_character
                        .iter()
                        .map(|a| ("spcl chars", a.help.clone()))
                        .filter_map(|(k, mh)| {
                            mh.map(|h| {
                                Row::new(vec![Cell::from(k.to_string()), Cell::from(h.to_string())])
                            })
                        }),
                )
                .chain(
                    mode.key_bindings
                        .default
                        .iter()
                        .map(|a| ("default", a.help.clone()))
                        .filter_map(|(k, mh)| {
                            mh.map(|h| {
                                Row::new(vec![Cell::from(k.to_string()), Cell::from(h.to_string())])
                            })
                        }),
                )
                .collect::<Vec<Row>>()
        });

    let help_menu = Table::new(help_menu_rows)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!(" Help [{}] ", &mode.name)),
        )
        .widths(&[TUIConstraint::Percentage(40), TUIConstraint::Percentage(60)]);
    f.render_widget(help_menu, rect);
}

fn draw_input_buffer<B: Backend>(f: &mut Frame<B>, rect: Rect, app: &app::App, _: &Handlebars) {
    let input_buf = Paragraph::new(format!("> {}", app.input_buffer().unwrap_or(&"".into())))
        .block(Block::default().borders(Borders::ALL).title(" input "));
    f.render_widget(input_buf, rect);
}

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &app::App, hb: &Handlebars) {
    let rect = f.size();

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([TUIConstraint::Percentage(70), TUIConstraint::Percentage(30)].as_ref())
        .split(rect);

    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([TUIConstraint::Length(rect.height - 3), TUIConstraint::Min(3)].as_ref())
        .split(chunks[0]);


    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([TUIConstraint::Percentage(50), TUIConstraint::Percentage(50)].as_ref())
        .split(chunks[1]);

    draw_table(f, left_chunks[0], app, hb);
    draw_input_buffer(f, left_chunks[1], app, hb);
    draw_selected(f, right_chunks[0], app, hb);
    draw_help_menu(f, right_chunks[1], app, hb);
}
