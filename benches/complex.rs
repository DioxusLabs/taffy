use criterion::{criterion_group, criterion_main, Criterion};

fn build_deep_hierarchy(sprawl: &mut sprawl::node::Sprawl) -> sprawl::node::Node {
    let node111 = sprawl
        .new_node(
            sprawl::style::FlexboxLayout {
                size: sprawl::geometry::Size {
                    width: sprawl::style::Dimension::Points(10.0),
                    height: sprawl::style::Dimension::Points(10.0),
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node112 = sprawl
        .new_node(
            sprawl::style::FlexboxLayout {
                size: sprawl::geometry::Size {
                    width: sprawl::style::Dimension::Points(10.0),
                    height: sprawl::style::Dimension::Points(10.0),
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();

    let node121 = sprawl
        .new_node(
            sprawl::style::FlexboxLayout {
                size: sprawl::geometry::Size {
                    width: sprawl::style::Dimension::Points(10.0),
                    height: sprawl::style::Dimension::Points(10.0),
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node122 = sprawl
        .new_node(
            sprawl::style::FlexboxLayout {
                size: sprawl::geometry::Size {
                    width: sprawl::style::Dimension::Points(10.0),
                    height: sprawl::style::Dimension::Points(10.0),
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();

    let node11 = sprawl.new_node(sprawl::style::FlexboxLayout { ..Default::default() }, &[node111, node112]).unwrap();
    let node12 = sprawl.new_node(sprawl::style::FlexboxLayout { ..Default::default() }, &[node121, node122]).unwrap();
    let node1 = sprawl.new_node(sprawl::style::FlexboxLayout { ..Default::default() }, &[node11, node12]).unwrap();

    let node211 = sprawl
        .new_node(
            sprawl::style::FlexboxLayout {
                size: sprawl::geometry::Size {
                    width: sprawl::style::Dimension::Points(10.0),
                    height: sprawl::style::Dimension::Points(10.0),
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node212 = sprawl
        .new_node(
            sprawl::style::FlexboxLayout {
                size: sprawl::geometry::Size {
                    width: sprawl::style::Dimension::Points(10.0),
                    height: sprawl::style::Dimension::Points(10.0),
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();

    let node221 = sprawl
        .new_node(
            sprawl::style::FlexboxLayout {
                size: sprawl::geometry::Size {
                    width: sprawl::style::Dimension::Points(10.0),
                    height: sprawl::style::Dimension::Points(10.0),
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node222 = sprawl
        .new_node(
            sprawl::style::FlexboxLayout {
                size: sprawl::geometry::Size {
                    width: sprawl::style::Dimension::Points(10.0),
                    height: sprawl::style::Dimension::Points(10.0),
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();

    let node21 = sprawl.new_node(sprawl::style::FlexboxLayout { ..Default::default() }, &[node211, node212]).unwrap();
    let node22 = sprawl.new_node(sprawl::style::FlexboxLayout { ..Default::default() }, &[node221, node222]).unwrap();

    let node2 = sprawl.new_node(sprawl::style::FlexboxLayout { ..Default::default() }, &[node21, node22]).unwrap();

    sprawl.new_node(sprawl::style::FlexboxLayout { ..Default::default() }, &[node1, node2]).unwrap()
}

fn sprawl_benchmarks(c: &mut Criterion) {
    c.bench_function("deep hierarchy - build", |b| {
        b.iter(|| {
            let mut sprawl = sprawl::node::Sprawl::new();
            build_deep_hierarchy(&mut sprawl);
        })
    });

    c.bench_function("deep hierarchy - single", |b| {
        b.iter(|| {
            let mut sprawl = sprawl::node::Sprawl::new();
            let root = build_deep_hierarchy(&mut sprawl);
            sprawl.compute_layout(root, sprawl::geometry::Size::undefined()).unwrap()
        })
    });

    c.bench_function("deep hierarchy - relayout", |b| {
        let mut sprawl = sprawl::node::Sprawl::new();
        let root = build_deep_hierarchy(&mut sprawl);

        b.iter(|| {
            sprawl.mark_dirty(root).unwrap();
            sprawl.compute_layout(root, sprawl::geometry::Size::undefined()).unwrap()
        })
    });
}

criterion_group!(benches, sprawl_benchmarks);
criterion_main!(benches);
