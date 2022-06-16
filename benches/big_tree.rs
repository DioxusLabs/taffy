//! This file includes benchmarks for very large, pseudo-randomly generated trees
use criterion::{criterion_group, criterion_main, Criterion};

/// The number of nodes to include in the trees
const NODE_COUNT: u32 = 1_000_000;

/// A single root node with many children that have shallow depth
fn build_large_flat_hierachy(taffy: &mut taffy::node::Taffy) -> taffy::node::Node {
    let mut children = Vec::new();

    taffy.new_with_children(taffy::style::FlexboxLayout { ..Default::default() }, &children).unwrap()
}

fn taffy_benchmarks(c: &mut Criterion) {
    c.bench_function("large flat hierachy", |b| {
        b.iter(|| {
            let mut taffy = taffy::node::Taffy::new();
            let root = build_large_flat_hierachy(&mut taffy);
            taffy.compute_layout(root, taffy::geometry::Size::undefined()).unwrap()
        })
    });
}

criterion_group!(benches, taffy_benchmarks);
criterion_main!(benches);
