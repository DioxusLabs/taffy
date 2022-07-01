use criterion::{criterion_group, criterion_main, Criterion};

fn build_deep_hierarchy(taffy: &mut taffy::node::Taffy) -> taffy::node::Node {
    let node111 = taffy.new_leaf(taffy::style::FlexboxLayout {
        size: taffy::geometry::Size {
            width: taffy::style::Dimension::Points(10.0),
            height: taffy::style::Dimension::Points(10.0),
        },
        ..Default::default()
    });
    let node112 = taffy.new_leaf(taffy::style::FlexboxLayout {
        size: taffy::geometry::Size {
            width: taffy::style::Dimension::Points(10.0),
            height: taffy::style::Dimension::Points(10.0),
        },
        ..Default::default()
    });

    let node121 = taffy.new_leaf(taffy::style::FlexboxLayout {
        size: taffy::geometry::Size {
            width: taffy::style::Dimension::Points(10.0),
            height: taffy::style::Dimension::Points(10.0),
        },
        ..Default::default()
    });
    let node122 = taffy.new_leaf(taffy::style::FlexboxLayout {
        size: taffy::geometry::Size {
            width: taffy::style::Dimension::Points(10.0),
            height: taffy::style::Dimension::Points(10.0),
        },
        ..Default::default()
    });

    let node11 = taffy.new_with_children(taffy::style::FlexboxLayout { ..Default::default() }, &[node111, node112]);
    let node12 = taffy.new_with_children(taffy::style::FlexboxLayout { ..Default::default() }, &[node121, node122]);
    let node1 = taffy.new_with_children(taffy::style::FlexboxLayout { ..Default::default() }, &[node11, node12]);

    let node211 = taffy.new_leaf(taffy::style::FlexboxLayout {
        size: taffy::geometry::Size {
            width: taffy::style::Dimension::Points(10.0),
            height: taffy::style::Dimension::Points(10.0),
        },
        ..Default::default()
    });
    let node212 = taffy.new_leaf(taffy::style::FlexboxLayout {
        size: taffy::geometry::Size {
            width: taffy::style::Dimension::Points(10.0),
            height: taffy::style::Dimension::Points(10.0),
        },
        ..Default::default()
    });

    let node221 = taffy.new_leaf(taffy::style::FlexboxLayout {
        size: taffy::geometry::Size {
            width: taffy::style::Dimension::Points(10.0),
            height: taffy::style::Dimension::Points(10.0),
        },
        ..Default::default()
    });
    let node222 = taffy.new_leaf(taffy::style::FlexboxLayout {
        size: taffy::geometry::Size {
            width: taffy::style::Dimension::Points(10.0),
            height: taffy::style::Dimension::Points(10.0),
        },
        ..Default::default()
    });

    let node21 = taffy.new_with_children(taffy::style::FlexboxLayout { ..Default::default() }, &[node211, node212]);
    let node22 = taffy.new_with_children(taffy::style::FlexboxLayout { ..Default::default() }, &[node221, node222]);

    let node2 = taffy.new_with_children(taffy::style::FlexboxLayout { ..Default::default() }, &[node21, node22]);

    taffy.new_with_children(taffy::style::FlexboxLayout { ..Default::default() }, &[node1, node2])
}

fn taffy_benchmarks(c: &mut Criterion) {
    c.bench_function("deep hierarchy - build", |b| {
        b.iter(|| {
            let mut taffy = taffy::node::Taffy::new();
            build_deep_hierarchy(&mut taffy);
        })
    });

    c.bench_function("deep hierarchy - single", |b| {
        b.iter(|| {
            let mut taffy = taffy::node::Taffy::new();
            let root = build_deep_hierarchy(&mut taffy);
            taffy.compute_layout(root, taffy::geometry::Size::undefined()).unwrap()
        })
    });

    c.bench_function("deep hierarchy - relayout", |b| {
        let mut taffy = taffy::node::Taffy::new();
        let root = build_deep_hierarchy(&mut taffy);

        b.iter(|| {
            taffy.mark_dirty(root);
            taffy.compute_layout(root, taffy::geometry::Size::undefined()).unwrap()
        })
    });
}

criterion_group!(benches, taffy_benchmarks);
criterion_main!(benches);
