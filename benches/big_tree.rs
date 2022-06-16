//! This file includes benchmarks for very large, pseudo-randomly generated trees
use criterion::{criterion_group, criterion_main, Criterion};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use taffy::prelude::*;
use taffy::style::FlexboxLayout;

/// The number of nodes to include in the trees
const NODE_COUNT: u32 = 100_000;

/// Get a randomly generated dimension
fn get_random_dimension(rng: &mut ChaCha8Rng) -> Dimension {
    let switch: f32 = rng.gen_range(0.0..=1.0);

    match switch {
        0.0..=0.2 => Dimension::Auto,
        0.2..=0.4 => Dimension::Undefined,
        0.4..=0.8 => Dimension::Points(rng.gen_range(0.0..500.0)),
        _ => Dimension::Percent(rng.gen_range(0.0..1.0)),
    }
}

/// Get a randomly generated size
fn get_random_size(rng: &mut ChaCha8Rng) -> Size<Dimension> {
    Size { width: get_random_dimension(rng), height: get_random_dimension(rng) }
}

/// Get a randomly generated style for a node
fn get_random_style(rng: &mut ChaCha8Rng) -> FlexboxLayout {
    // TODO: Add more attributes
    FlexboxLayout { size: get_random_size(rng), ..Default::default() }
}

/// Build a random leaf node
fn build_random_leaf(taffy: &mut Taffy, rng: &mut ChaCha8Rng) -> Node {
    taffy.new_with_children(get_random_style(rng), &[]).unwrap()
}

/// A single root node with many children that have shallow depth
fn build_single_root_flat_hierarchy(taffy: &mut Taffy) -> Node {
    let mut rng = ChaCha8Rng::seed_from_u64(12345);
    let mut children = Vec::new();
    let mut node_count = 0;

    while node_count < NODE_COUNT {
        let sub_children_count = rng.gen_range(0..=3);
        let sub_children: Vec<Node> = (0..sub_children_count).map(|_| build_random_leaf(taffy, &mut rng)).collect();
        let node = taffy.new_with_children(get_random_style(&mut rng), &sub_children).unwrap();

        children.push(node);
        node_count += 1 + sub_children_count;
    }

    taffy.new_with_children(FlexboxLayout { ..Default::default() }, children.as_slice()).unwrap()
}

fn taffy_benchmarks(c: &mut Criterion) {
    c.bench_function("single root, flat hierarchy", |b| {
        b.iter(|| {
            let mut taffy = Taffy::new();
            let root = build_single_root_flat_hierarchy(&mut taffy);
            taffy.compute_layout(root, Size::undefined()).unwrap()
        })
    });
}

criterion_group!(benches, taffy_benchmarks);
criterion_main!(benches);
