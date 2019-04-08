use criterion::{criterion_group, criterion_main, Criterion};

fn stretch_benchmarks(c: &mut Criterion) {
    c.bench_function("deep hierarchy", |b| {
        b.iter(|| {
            stretch::node::Node::new(
                stretch::style::Style { ..Default::default() },
                vec![
                    &stretch::node::Node::new(
                        stretch::style::Style { ..Default::default() },
                        vec![
                            &stretch::node::Node::new(
                                stretch::style::Style { ..Default::default() },
                                vec![
                                    &stretch::node::Node::new(
                                        stretch::style::Style {
                                            size: stretch::geometry::Size {
                                                width: stretch::style::Dimension::Points(10.0),
                                                height: stretch::style::Dimension::Points(10.0),
                                            },
                                            ..Default::default()
                                        },
                                        vec![],
                                    ),
                                    &stretch::node::Node::new(
                                        stretch::style::Style {
                                            size: stretch::geometry::Size {
                                                width: stretch::style::Dimension::Points(10.0),
                                                height: stretch::style::Dimension::Points(10.0),
                                            },
                                            ..Default::default()
                                        },
                                        vec![],
                                    ),
                                ],
                            ),
                            &stretch::node::Node::new(
                                stretch::style::Style { ..Default::default() },
                                vec![
                                    &stretch::node::Node::new(
                                        stretch::style::Style {
                                            size: stretch::geometry::Size {
                                                width: stretch::style::Dimension::Points(10.0),
                                                height: stretch::style::Dimension::Points(10.0),
                                            },
                                            ..Default::default()
                                        },
                                        vec![],
                                    ),
                                    &stretch::node::Node::new(
                                        stretch::style::Style {
                                            size: stretch::geometry::Size {
                                                width: stretch::style::Dimension::Points(10.0),
                                                height: stretch::style::Dimension::Points(10.0),
                                            },
                                            ..Default::default()
                                        },
                                        vec![],
                                    ),
                                ],
                            ),
                        ],
                    ),
                    &stretch::node::Node::new(
                        stretch::style::Style { ..Default::default() },
                        vec![
                            &stretch::node::Node::new(
                                stretch::style::Style { ..Default::default() },
                                vec![
                                    &stretch::node::Node::new(
                                        stretch::style::Style {
                                            size: stretch::geometry::Size {
                                                width: stretch::style::Dimension::Points(10.0),
                                                height: stretch::style::Dimension::Points(10.0),
                                            },
                                            ..Default::default()
                                        },
                                        vec![],
                                    ),
                                    &stretch::node::Node::new(
                                        stretch::style::Style {
                                            size: stretch::geometry::Size {
                                                width: stretch::style::Dimension::Points(10.0),
                                                height: stretch::style::Dimension::Points(10.0),
                                            },
                                            ..Default::default()
                                        },
                                        vec![],
                                    ),
                                ],
                            ),
                            &stretch::node::Node::new(
                                stretch::style::Style { ..Default::default() },
                                vec![
                                    &stretch::node::Node::new(
                                        stretch::style::Style {
                                            size: stretch::geometry::Size {
                                                width: stretch::style::Dimension::Points(10.0),
                                                height: stretch::style::Dimension::Points(10.0),
                                            },
                                            ..Default::default()
                                        },
                                        vec![],
                                    ),
                                    &stretch::node::Node::new(
                                        stretch::style::Style {
                                            size: stretch::geometry::Size {
                                                width: stretch::style::Dimension::Points(10.0),
                                                height: stretch::style::Dimension::Points(10.0),
                                            },
                                            ..Default::default()
                                        },
                                        vec![],
                                    ),
                                ],
                            ),
                        ],
                    ),
                ],
            )
            .compute_layout(stretch::geometry::Size::undefined())
            .unwrap()
        })
    });
}

criterion_group!(benches, stretch_benchmarks);
criterion_main!(benches);
