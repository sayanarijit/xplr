use crate::app;
use crate::ui;
use anyhow::Result;
use criterion::{criterion_group, criterion_main, Criterion};
use crossterm::execute;
use crossterm::terminal as term;
use handlebars::{handlebars_helper, Handlebars};
use humansize::{file_size_opts as options, FileSize};
use std::fs;
use std::io::prelude::*;
use termion::get_tty;
use tui::backend::CrosstermBackend;
use tui::Terminal;
use xplr::*;

const PWD: &str = "/tmp/xplr_bench";

handlebars_helper!(to_humansize: |size: i64| size.file_size(options::CONVENTIONAL).unwrap_or_default());

fn navigation_benchmark(c: &mut Criterion) {
    fs::create_dir_all(PWD).unwrap();
    (1..10000).for_each(|i| {
        fs::File::create(std::path::Path::new(PWD).join(i.to_string())).unwrap();
    });

    let mut app = app::App::create(PWD.into()).expect("failed to create app");

    app = app
        .clone()
        .handle_task(app::Task::new(
            app::MsgIn::External(app::ExternalMsg::ChangeDirectory(PWD.into())),
            None,
        ))
        .unwrap();

    c.bench_function("focus next item", |b| {
        b.iter(|| {
            app.clone()
                .handle_task(app::Task::new(
                    app::MsgIn::External(app::ExternalMsg::FocusNext),
                    None,
                ))
                .unwrap()
        })
    });

    c.bench_function("focus previous item", |b| {
        b.iter(|| {
            app.clone()
                .handle_task(app::Task::new(
                    app::MsgIn::External(app::ExternalMsg::FocusPrevious),
                    None,
                ))
                .unwrap()
        })
    });

    c.bench_function("focus first item", |b| {
        b.iter(|| {
            app.clone()
                .handle_task(app::Task::new(
                    app::MsgIn::External(app::ExternalMsg::FocusFirst),
                    None,
                ))
                .unwrap()
        })
    });

    c.bench_function("focus last item", |b| {
        b.iter(|| {
            app.clone()
                .handle_task(app::Task::new(
                    app::MsgIn::External(app::ExternalMsg::FocusLast),
                    None,
                ))
                .unwrap()
        })
    });

    c.bench_function("leave and enter directory", |b| {
        b.iter(|| {
            app.clone()
                .handle_task(app::Task::new(
                    app::MsgIn::External(app::ExternalMsg::Back),
                    None,
                ))
                .unwrap()
                .handle_task(app::Task::new(
                    app::MsgIn::External(app::ExternalMsg::Enter),
                    None,
                ))
                .unwrap()
        })
    });
}

fn draw_benchmark(c: &mut Criterion) {
    fs::create_dir_all(PWD).unwrap();
    (1..10000).for_each(|i| {
        fs::File::create(std::path::Path::new(PWD).join(i.to_string())).unwrap();
    });

    let mut app = app::App::create(PWD.into()).expect("failed to create app");

    app = app
        .clone()
        .handle_task(app::Task::new(
            app::MsgIn::External(app::ExternalMsg::ChangeDirectory(PWD.into())),
            None,
        ))
        .unwrap();

    let mut hb = Handlebars::new();
    hb.register_helper("humansize", Box::new(to_humansize));
    hb.register_template_string(
        app::TEMPLATE_TABLE_ROW,
        &app.config()
            .general()
            .table()
            .row()
            .cols()
            .clone()
            .unwrap_or_default()
            .iter()
            .map(|c| c.format().clone().unwrap_or_default())
            .collect::<Vec<String>>()
            .join("\t"),
    )
    .unwrap();

    term::enable_raw_mode().unwrap();
    let mut stdout = get_tty().unwrap();
    // let mut stdout = stdout.lock();
    execute!(stdout, term::EnterAlternateScreen).unwrap();
    // let stdout = MouseTerminal::from(stdout);
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal.hide_cursor().unwrap();

    c.bench_function("draw on terminal", |b| {
        b.iter(|| {
            terminal.draw(|f| ui::draw(f, &app, &hb)).unwrap();
        })
    });

    terminal.clear().unwrap();
    terminal.set_cursor(0, 0).unwrap();
    execute!(terminal.backend_mut(), term::LeaveAlternateScreen).unwrap();
    term::disable_raw_mode().unwrap();
    terminal.show_cursor().unwrap();
}

criterion_group!(benches, navigation_benchmark, draw_benchmark);
criterion_main!(benches);
