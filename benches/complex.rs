use criterion::{criterion_group, criterion_main, Criterion};

fn build_deep_hierarchy(stretch: &mut stretch::node::Stretch) -> stretch::node::Node {
    let node111 = stretch
        .new_node(
            stretch::style::Style {
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(10.0),
                    height: stretch::style::Dimension::Points(10.0),
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node112 = stretch
        .new_node(
            stretch::style::Style {
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(10.0),
                    height: stretch::style::Dimension::Points(10.0),
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();

    let node121 = stretch
        .new_node(
            stretch::style::Style {
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(10.0),
                    height: stretch::style::Dimension::Points(10.0),
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node122 = stretch
        .new_node(
            stretch::style::Style {
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(10.0),
                    height: stretch::style::Dimension::Points(10.0),
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();

    let node11 = stretch.new_node(stretch::style::Style { ..Default::default() }, &[node111, node112]).unwrap();
    let node12 = stretch.new_node(stretch::style::Style { ..Default::default() }, &[node121, node122]).unwrap();
    let node1 = stretch.new_node(stretch::style::Style { ..Default::default() }, &[node11, node12]).unwrap();

    let node211 = stretch
        .new_node(
            stretch::style::Style {
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(10.0),
                    height: stretch::style::Dimension::Points(10.0),
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node212 = stretch
        .new_node(
            stretch::style::Style {
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(10.0),
                    height: stretch::style::Dimension::Points(10.0),
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();

    let node221 = stretch
        .new_node(
            stretch::style::Style {
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(10.0),
                    height: stretch::style::Dimension::Points(10.0),
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node222 = stretch
        .new_node(
            stretch::style::Style {
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(10.0),
                    height: stretch::style::Dimension::Points(10.0),
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();

    let node21 = stretch.new_node(stretch::style::Style { ..Default::default() }, &[node211, node212]).unwrap();
    let node22 = stretch.new_node(stretch::style::Style { ..Default::default() }, &[node221, node222]).unwrap();

    let node2 = stretch.new_node(stretch::style::Style { ..Default::default() }, &[node21, node22]).unwrap();
    let node0 = stretch.new_node(stretch::style::Style { ..Default::default() }, &[node1, node2]).unwrap();

    node0
}

fn stretch_benchmarks(c: &mut Criterion) {
    c.bench_function("deep hierarchy - build", |b| {
        b.iter(|| {
            let mut stretch = stretch::node::Stretch::new();
            build_deep_hierarchy(&mut stretch);
        })
    });

    c.bench_function("deep hierarchy - single", |b| {
        b.iter(|| {
            let mut stretch = stretch::node::Stretch::new();
            let root = build_deep_hierarchy(&mut stretch);
            stretch.compute_layout(root, stretch::geometry::Size::undefined()).unwrap()
        })
    });

    c.bench_function("deep hierarchy - relayout", |b| {
        let mut stretch = stretch::node::Stretch::new();
        let root = build_deep_hierarchy(&mut stretch);

        b.iter(|| {
            stretch.mark_dirty(root).unwrap();
            stretch.compute_layout(root, stretch::geometry::Size::undefined()).unwrap()
        })
    });
}

criterion_group!(benches, stretch_benchmarks);
criterion_main!(benches);
