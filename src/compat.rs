// Things of the past, mostly bad decisions, which cannot erased, stays in this
// haunted module.

use crate::app;
use crate::lua;
use crate::ui::block;
use crate::ui::string_to_text;
use crate::ui::Constraint;
use crate::ui::ContentRendererArg;
use crate::ui::UI;
use serde::{Deserialize, Serialize};
use tui::layout::Constraint as TuiConstraint;
use tui::layout::Rect as TuiRect;
use tui::widgets::Cell;
use tui::widgets::List;
use tui::widgets::ListItem;
use tui::widgets::Paragraph;
use tui::widgets::Row;
use tui::widgets::Table;
use tui::Frame;

/// A cursed enum from crate::ui.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
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

/// A cursed struct from crate::ui.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct CustomContent {
    pub title: Option<String>,
    pub body: ContentBody,
}

/// A cursed function from crate::ui.
pub fn draw_custom_content(
    ui: &mut UI,
    f: &mut Frame,
    layout_size: TuiRect,
    app: &app::App,
    content: CustomContent,
) {
    let config = app.config.general.panel_ui.default.clone();
    let title = content.title;
    let body = content.body;

    match body {
        ContentBody::StaticParagraph { render } => {
            let render = string_to_text(render);
            let content = Paragraph::new(render).block(block(
                config,
                title.map(|t| format!(" {t} ")).unwrap_or_default(),
            ));
            f.render_widget(content, layout_size);
        }

        ContentBody::DynamicParagraph { render } => {
            let ctx = ContentRendererArg {
                app: app.to_lua_ctx_light(),
                layout_size: layout_size.into(),
                screen_size: ui.screen_size.into(),
                scrolltop: ui.scrolltop as u16,
            };

            let render = lua::serialize(ui.lua, &ctx)
                .map(|arg| {
                    lua::call(ui.lua, &render, arg).unwrap_or_else(|e| format!("{e:?}"))
                })
                .unwrap_or_else(|e| e.to_string());

            let render = string_to_text(render);

            let content = Paragraph::new(render).block(block(
                config,
                title.map(|t| format!(" {t} ")).unwrap_or_default(),
            ));
            f.render_widget(content, layout_size);
        }

        ContentBody::StaticList { render } => {
            let items = render
                .into_iter()
                .map(string_to_text)
                .map(ListItem::new)
                .collect::<Vec<ListItem>>();

            let content = List::new(items).block(block(
                config,
                title.map(|t| format!(" {t} ")).unwrap_or_default(),
            ));
            f.render_widget(content, layout_size);
        }

        ContentBody::DynamicList { render } => {
            let ctx = ContentRendererArg {
                app: app.to_lua_ctx_light(),
                layout_size: layout_size.into(),
                screen_size: ui.screen_size.into(),
                scrolltop: ui.scrolltop as u16,
            };

            let items = lua::serialize(ui.lua, &ctx)
                .map(|arg| {
                    lua::call(ui.lua, &render, arg)
                        .unwrap_or_else(|e| vec![format!("{e:?}")])
                })
                .unwrap_or_else(|e| vec![e.to_string()])
                .into_iter()
                .map(string_to_text)
                .map(ListItem::new)
                .collect::<Vec<ListItem>>();

            let content = List::new(items).block(block(
                config,
                title.map(|t| format!(" {t} ")).unwrap_or_default(),
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
                            .map(string_to_text)
                            .map(Cell::from)
                            .collect::<Vec<Cell>>(),
                    )
                })
                .collect::<Vec<Row>>();

            let widths = widths
                .into_iter()
                .map(|w| w.to_tui(ui.screen_size, layout_size))
                .collect::<Vec<TuiConstraint>>();

            let content = Table::new(rows, widths)
                .column_spacing(col_spacing.unwrap_or(1))
                .block(block(
                    config,
                    title.map(|t| format!(" {t} ")).unwrap_or_default(),
                ));

            f.render_widget(content, layout_size);
        }

        ContentBody::DynamicTable {
            widths,
            col_spacing,
            render,
        } => {
            let ctx = ContentRendererArg {
                app: app.to_lua_ctx_light(),
                layout_size: layout_size.into(),
                screen_size: ui.screen_size.into(),
                scrolltop: ui.scrolltop as u16,
            };

            let rows = lua::serialize(ui.lua, &ctx)
                .map(|arg| {
                    lua::call(ui.lua, &render, arg)
                        .unwrap_or_else(|e| vec![vec![format!("{e:?}")]])
                })
                .unwrap_or_else(|e| vec![vec![e.to_string()]])
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
                .map(|w| w.to_tui(ui.screen_size, layout_size))
                .collect::<Vec<TuiConstraint>>();

            let mut content = Table::new(rows, &widths).block(block(
                config,
                title.map(|t| format!(" {t} ")).unwrap_or_default(),
            ));

            if let Some(col_spacing) = col_spacing {
                content = content.column_spacing(col_spacing);
            };

            f.render_widget(content, layout_size);
        }
    }
}
