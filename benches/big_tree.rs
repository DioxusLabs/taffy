//! This file includes benchmarks for very large, pseudo-randomly generated trees
use criterion::{criterion_group, criterion_main, Criterion};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use taffy::prelude::*;
use taffy::randomizable::Randomizeable;
use taffy::style::FlexboxLayout;

/// The number of nodes to include in the trees
const NODE_COUNT: u32 = 100_000;

/// Build a random leaf node
fn build_random_leaf(taffy: &mut Taffy, rng: &mut ChaCha8Rng) -> Node {
    taffy.new_with_children(FlexboxLayout::random(rng), &[]).unwrap()
}

/// A tree with many children that have shallow depth
fn build_flat_hierarchy(taffy: &mut Taffy) -> Node {
    let mut rng = ChaCha8Rng::seed_from_u64(12345);
    let mut children = Vec::new();
    let mut node_count = 0;

    while node_count < NODE_COUNT {
        let sub_children_count = rng.gen_range(1..=4);
        let sub_children: Vec<Node> = (0..sub_children_count).map(|_| build_random_leaf(taffy, &mut rng)).collect();
        let node = taffy.new_with_children(FlexboxLayout::random(&mut rng), &sub_children).unwrap();

        children.push(node);
        node_count += 1 + sub_children_count;
    }

    taffy.new_with_children(FlexboxLayout { ..Default::default() }, children.as_slice()).unwrap()
}

/// A helper function to recursively construct a deep tree
fn build_deep_tree(taffy: &mut Taffy, rng: &mut ChaCha8Rng, max_nodes: u32) -> Vec<Node> {
    let branching_factor = 7;

    if max_nodes <= branching_factor {
        // Build leaf nodes
        return (0..max_nodes).map(|_| build_random_leaf(taffy, rng)).collect();
    }

    // Add another layer to the tree
    // Each child gets an equal amount of the remaining nodes
    (0..branching_factor)
        .map(|_| {
            let sub_children = build_deep_tree(taffy, rng, (max_nodes - branching_factor) / branching_factor);
            taffy.new_with_children(FlexboxLayout::random(rng), &sub_children).unwrap()
        })
        .collect()
}

/// A tree with a higher depth for a more realistic scenario
fn build_deep_hierarchy(taffy: &mut Taffy) -> Node {
    let mut rng = ChaCha8Rng::seed_from_u64(12345);

    let tree = build_deep_tree(taffy, &mut rng, NODE_COUNT);

    taffy.new_with_children(FlexboxLayout { ..Default::default() }, &tree).unwrap()
}

fn taffy_benchmarks(c: &mut Criterion) {
    // Decrease sample size, because the tasks take longer
    let mut group = c.benchmark_group("Big tree");
    group.sample_size(10);

    group.bench_function("flat hierarchy", |b| {
        let mut taffy = Taffy::new();
        let root = build_flat_hierarchy(&mut taffy);

        b.iter(|| taffy.compute_layout(root, Size::NONE).unwrap())
    });

    group.bench_function("deep hierarchy", |b| {
        let mut taffy = Taffy::new();
        let root = build_deep_hierarchy(&mut taffy);

        b.iter(|| taffy.compute_layout(root, Size::NONE).unwrap())
    });
}

criterion_group!(benches, taffy_benchmarks);
criterion_main!(benches);
