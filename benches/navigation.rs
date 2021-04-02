use criterion::{criterion_group, criterion_main, Criterion};
use std::fs;
use xplr::*;

fn criterion_benchmark(c: &mut Criterion) {
    fs::create_dir_all("/tmp/xplr_bench").unwrap();
    (1..10000).for_each(|i| {
        fs::File::create(format!("/tmp/xplr_bench/{}", i)).unwrap();
    });

    let app = app::App::create()
        .expect("failed to create app")
        .enqueue(app::Task::new(
            1,
            app::MsgIn::External(app::ExternalMsg::ChangeDirectory("/tmp/xplr_bench".into())),
            None,
        ))
        .possibly_mutate()
        .unwrap();

    c.bench_function("focus next item", |b| {
        b.iter(|| {
            app.clone()
                .enqueue(app::Task::new(
                    1,
                    app::MsgIn::External(app::ExternalMsg::FocusNext),
                    None,
                ))
                .possibly_mutate()
                .unwrap()
        })
    });

    c.bench_function("focus previous item", |b| {
        b.iter(|| {
            app.clone()
                .enqueue(app::Task::new(
                    1,
                    app::MsgIn::External(app::ExternalMsg::FocusPrevious),
                    None,
                ))
                .possibly_mutate()
                .unwrap()
        })
    });

    c.bench_function("focus first item", |b| {
        b.iter(|| {
            app.clone()
                .enqueue(app::Task::new(
                    1,
                    app::MsgIn::External(app::ExternalMsg::FocusFirst),
                    None,
                ))
                .possibly_mutate()
                .unwrap()
        })
    });

    c.bench_function("focus last item", |b| {
        b.iter(|| {
            app.clone()
                .enqueue(app::Task::new(
                    1,
                    app::MsgIn::External(app::ExternalMsg::FocusLast),
                    None,
                ))
                .possibly_mutate()
                .unwrap()
        })
    });

    c.bench_function("leave and enter directory", |b| {
        b.iter(|| {
            app.clone()
                .enqueue(app::Task::new(
                    1,
                    app::MsgIn::External(app::ExternalMsg::Back),
                    None,
                ))
                .possibly_mutate()
                .unwrap()
                .enqueue(app::Task::new(
                    1,
                    app::MsgIn::External(app::ExternalMsg::Back),
                    None,
                ))
                .possibly_mutate()
                .unwrap()
        })
    });
}
criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
