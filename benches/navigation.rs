use criterion::{criterion_group, criterion_main, Criterion};
use std::fs;
use xplr::*;

fn criterion_benchmark(c: &mut Criterion) {
    let app = app::create()
        .expect("failed to create app")
        .change_directory(&"/tmp/xplr_bench".to_string())
        .unwrap();

    fs::create_dir_all("/tmp/xplr_bench").unwrap();
    (1..10000).for_each(|i| {
        fs::File::create(format!("/tmp/xplr_bench/{}", i)).unwrap();
    });

    c.bench_function("focus next item", |b| {
        b.iter(|| {
            app.clone()
                .handle(&config::Action::Global(config::GlobalAction::FocusNext))
        })
    });

    c.bench_function("focus previous item", |b| {
        b.iter(|| {
            app.clone().handle(&config::Action::Global(
                config::GlobalAction::FocusPrevious,
            ))
        })
    });

    c.bench_function("focus first item", |b| {
        b.iter(|| {
            app.clone()
                .handle(&config::Action::Global(config::GlobalAction::FocusFirst))
        })
    });

    c.bench_function("focus last item", |b| {
        b.iter(|| {
            app.clone()
                .handle(&config::Action::Global(config::GlobalAction::FocusLast))
        })
    });
}
criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
