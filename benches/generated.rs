use criterion::{criterion_group, criterion_main, Criterion};

fn stretch_benchmarks(c: &mut Criterion) {
    c.bench_function("deep hierarchy", |b| {
        b.iter(|| {
            stretch::compute(
                &stretch::style::Node {
                    children: vec![
                        stretch::style::Node {
                            children: vec![
                                stretch::style::Node {
                                    children: vec![
                                        stretch::style::Node {
                                            size: stretch::geometry::Size {
                                                width: stretch::style::Dimension::Points(10.0),
                                                height: stretch::style::Dimension::Points(10.0),
                                            },
                                            ..Default::default()
                                        },
                                        stretch::style::Node {
                                            size: stretch::geometry::Size {
                                                width: stretch::style::Dimension::Points(10.0),
                                                height: stretch::style::Dimension::Points(10.0),
                                            },
                                            ..Default::default()
                                        },
                                    ],
                                    ..Default::default()
                                },
                                stretch::style::Node {
                                    children: vec![
                                        stretch::style::Node {
                                            size: stretch::geometry::Size {
                                                width: stretch::style::Dimension::Points(10.0),
                                                height: stretch::style::Dimension::Points(10.0),
                                            },
                                            ..Default::default()
                                        },
                                        stretch::style::Node {
                                            size: stretch::geometry::Size {
                                                width: stretch::style::Dimension::Points(10.0),
                                                height: stretch::style::Dimension::Points(10.0),
                                            },
                                            ..Default::default()
                                        },
                                    ],
                                    ..Default::default()
                                },
                            ],

                            ..Default::default()
                        },
                        stretch::style::Node {
                            children: vec![
                                stretch::style::Node {
                                    children: vec![
                                        stretch::style::Node {
                                            size: stretch::geometry::Size {
                                                width: stretch::style::Dimension::Points(10.0),
                                                height: stretch::style::Dimension::Points(10.0),
                                            },
                                            ..Default::default()
                                        },
                                        stretch::style::Node {
                                            size: stretch::geometry::Size {
                                                width: stretch::style::Dimension::Points(10.0),
                                                height: stretch::style::Dimension::Points(10.0),
                                            },
                                            ..Default::default()
                                        },
                                    ],
                                    ..Default::default()
                                },
                                stretch::style::Node {
                                    children: vec![
                                        stretch::style::Node {
                                            size: stretch::geometry::Size {
                                                width: stretch::style::Dimension::Points(10.0),
                                                height: stretch::style::Dimension::Points(10.0),
                                            },
                                            ..Default::default()
                                        },
                                        stretch::style::Node {
                                            size: stretch::geometry::Size {
                                                width: stretch::style::Dimension::Points(10.0),
                                                height: stretch::style::Dimension::Points(10.0),
                                            },
                                            ..Default::default()
                                        },
                                    ],
                                    ..Default::default()
                                },
                            ],
                            ..Default::default()
                        },
                    ],

                    ..Default::default()
                },
                stretch::geometry::Size::undefined(),
            )
            .unwrap()
        })
    });
}

criterion_group!(benches, stretch_benchmarks);
criterion_main!(benches);
