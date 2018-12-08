use criterion::{criterion_group, criterion_main, Criterion};

fn stretch_benchmarks(c: &mut Criterion) {
    c.bench_function("layout", |b| {
        b.iter(|| {
            stretch::compute(&stretch::style::Node {
                align_items: stretch::style::AlignItems::Center,
                width: stretch::style::Dimension::Points(100.0000),
                height: stretch::style::Dimension::Points(100.0000),

                children: vec![stretch::style::Node {
                    width: stretch::style::Dimension::Points(10.0000),
                    height: stretch::style::Dimension::Points(10.0000),
                    ..Default::default()
                }],

                ..Default::default()
            })
        })
    });
}

criterion_group!(benches, stretch_benchmarks);
criterion_main!(benches);
