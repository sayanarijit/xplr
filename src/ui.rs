use crate::app;
use handlebars::Handlebars;
use tui::backend::Backend;
use tui::layout::{Constraint as TUIConstraint, Direction, Layout};
use tui::widgets::{Block, Borders, Cell, List, ListItem, ListState, Row, Table, TableState};
use tui::Frame;

pub fn draw<B: Backend>(
    app: &app::App,
    hb: &Handlebars,
    f: &mut Frame<B>,
    table_state: &mut TableState,
    list_state: &mut ListState,
) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints([TUIConstraint::Percentage(70), TUIConstraint::Percentage(30)].as_ref())
        .split(f.size());

    let body = app
        .directory_buffer
        .items
        .iter()
        .map(|(_, m)| {
            let txt = hb
                .render(app::TEMPLATE_TABLE_ROW, &m)
                .ok()
                .unwrap_or_else(|| app::UNSUPPORTED_STR.into())
                .split("\t")
                .map(|x| Cell::from(x.to_string()))
                .collect::<Vec<Cell>>();

            let style = if m.is_focused {
                app.config.general.focused_ui.style
            } else if m.is_selected {
                app.config.general.selected_ui.style
            } else {
                app.config
                    .filetypes
                    .special
                    .get(&m.relative_path)
                    .or_else(|| app.config.filetypes.extension.get(&m.extension))
                    .unwrap_or_else(|| {
                        if m.is_symlink {
                            &app.config.filetypes.symlink
                        } else if m.is_dir {
                            &app.config.filetypes.directory
                        } else {
                            &app.config.filetypes.file
                        }
                    })
                    .style
            };
            (txt, style)
        })
        .map(|(t, s)| Row::new(t).style(s))
        .collect::<Vec<Row>>();

    let table_constraints: Vec<TUIConstraint> = app
        .config
        .general
        .table
        .col_widths
        .clone()
        .into_iter()
        .map(|c| c.into())
        .collect();

    let table = Table::new(body)
        .widths(&table_constraints)
        .style(app.config.general.table.style)
        .highlight_style(app.config.general.focused_ui.style)
        .column_spacing(app.config.general.table.col_spacing)
        .block(Block::default().borders(Borders::ALL).title(format!(
            " {} ",
            app.directory_buffer.pwd.to_str().unwrap_or("???")
        )));

    let table = app
        .config
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

    table_state.select(
        app.directory_buffer
            .focus
            .map(app::DirectoryBuffer::relative_focus),
    );

    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([TUIConstraint::Percentage(40), TUIConstraint::Percentage(60)].as_ref())
        .split(chunks[1]);

    let selected: Vec<ListItem> = app
        .selected_paths
        .iter()
        .map(|p| p.to_str().unwrap_or(app::UNSUPPORTED_STR))
        .map(String::from)
        .map(ListItem::new)
        .collect();

    let selected_count = selected.len();

    // Selected items
    let selected_list = List::new(selected).block(
        Block::default()
            .borders(Borders::ALL)
            .title(format!(" Selected ({}) ", selected_count)),
    );

    // Help menu
    let help_menu_rows: Vec<Row> = app
        .parsed_help_menu
        .clone()
        .iter()
        .map(|(h, k)| Row::new(vec![Cell::from(h.to_string()), Cell::from(k.to_string())]))
        .collect();

    let help_menu = Table::new(help_menu_rows)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!(" Help [{}] ", &app.mode)),
        )
        .widths(&[TUIConstraint::Percentage(40), TUIConstraint::Percentage(60)]);

    f.render_stateful_widget(table, chunks[0], table_state);
    f.render_stateful_widget(selected_list, left_chunks[0], list_state);
    f.render_widget(help_menu, left_chunks[1]);
}
