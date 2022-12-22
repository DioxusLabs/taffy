//! This file includes benchmarks for very large, pseudo-randomly generated trees
use criterion::{criterion_group, criterion_main, Criterion};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use taffy::prelude::*;
use taffy::randomizable::Randomizeable;
use taffy::style::Style;

/// Build a random leaf node
fn build_random_leaf(taffy: &mut Taffy, rng: &mut ChaCha8Rng) -> Node {
    taffy.new_with_children(Style::random(rng), &[]).unwrap()
}

/// A tree with many children that have shallow depth
fn build_flat_hierarchy(taffy: &mut Taffy, total_node_count: u32) -> Node {
    let mut rng = ChaCha8Rng::seed_from_u64(12345);
    let mut children = Vec::new();
    let mut node_count = 0;

    while node_count < total_node_count {
        let sub_children_count = rng.gen_range(1..=4);
        let sub_children: Vec<Node> = (0..sub_children_count).map(|_| build_random_leaf(taffy, &mut rng)).collect();
        let node = taffy.new_with_children(Style::random(&mut rng), &sub_children).unwrap();

        children.push(node);
        node_count += 1 + sub_children_count;
    }

    taffy.new_with_children(Style { ..Default::default() }, children.as_slice()).unwrap()
}

/// A helper function to recursively construct a deep tree
fn build_deep_tree(
    taffy: &mut Taffy,
    max_nodes: u32,
    branching_factor: u32,
    create_leaf_node: &mut impl FnMut(&mut Taffy) -> Node,
    create_flex_node: &mut impl FnMut(&mut Taffy, Vec<Node>) -> Node,
) -> Vec<Node> {
    if max_nodes <= branching_factor {
        // Build leaf nodes
        return (0..max_nodes).map(|_| create_leaf_node(taffy)).collect();
    }

    // Add another layer to the tree
    // Each child gets an equal amount of the remaining nodes
    (0..branching_factor)
        .map(|_| {
            let max_nodes = (max_nodes - branching_factor) / branching_factor;
            let sub_children = build_deep_tree(taffy, max_nodes, branching_factor, create_leaf_node, create_flex_node);
            create_flex_node(taffy, sub_children)
        })
        .collect()
}

/// A tree with a higher depth for a more realistic scenario
fn build_deep_hierarchy(taffy: &mut Taffy, node_count: u32, branching_factor: u32) -> Node {
    let mut rng = ChaCha8Rng::seed_from_u64(12345);
    let mut build_leaf_node = |taffy: &mut Taffy| build_random_leaf(taffy, &mut rng);
    let mut rng = ChaCha8Rng::seed_from_u64(12345);
    let mut build_flex_node =
        |taffy: &mut Taffy, children: Vec<Node>| taffy.new_with_children(Style::random(&mut rng), &children).unwrap();

    let tree = build_deep_tree(taffy, node_count, branching_factor, &mut build_leaf_node, &mut build_flex_node);

    taffy.new_with_children(Style { ..Default::default() }, &tree).unwrap()
}

/// A deep tree that matches the shape and styling that yoga use on their benchmarks
fn build_yoga_deep_hierarchy(taffy: &mut Taffy, node_count: u32, branching_factor: u32) -> Node {
    let style = Style {
        size: Size { width: Dimension::Points(10.0), height: Dimension::Points(10.0) },
        flex_grow: 1.0,
        ..Default::default()
    };
    let mut build_leaf_node = |taffy: &mut Taffy| taffy.new_leaf(style.clone()).unwrap();
    let mut build_flex_node =
        |taffy: &mut Taffy, children: Vec<Node>| taffy.new_with_children(style.clone(), &children).unwrap();

    let tree = build_deep_tree(taffy, node_count, branching_factor, &mut build_leaf_node, &mut build_flex_node);

    taffy.new_with_children(Style::DEFAULT, &tree).unwrap()
}

fn taffy_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("yoga benchmarks");
    group.sample_size(10);

    group.bench_function("10 nodes", |b| {
        b.iter_batched(
            || {
                let mut taffy = Taffy::new();
                let root = build_yoga_deep_hierarchy(&mut taffy, 10, 10);
                (taffy, root)
            },
            |(mut taffy, root)| taffy.compute_layout(root, Size::MAX_CONTENT).unwrap(),
            criterion::BatchSize::SmallInput,
        )
    });

    group.bench_function("100 nodes", |b| {
        b.iter_batched(
            || {
                let mut taffy = Taffy::new();
                let root = build_yoga_deep_hierarchy(&mut taffy, 100, 10);
                (taffy, root)
            },
            |(mut taffy, root)| taffy.compute_layout(root, Size::MAX_CONTENT).unwrap(),
            criterion::BatchSize::SmallInput,
        )
    });

    group.bench_function("1_000 nodes", |b| {
        b.iter_batched(
            || {
                let mut taffy = Taffy::new();
                let root = build_yoga_deep_hierarchy(&mut taffy, 1_000, 10);
                (taffy, root)
            },
            |(mut taffy, root)| taffy.compute_layout(root, Size::MAX_CONTENT).unwrap(),
            criterion::BatchSize::SmallInput,
        )
    });

    group.bench_function("10_000 nodes", |b| {
        b.iter_batched(
            || {
                let mut taffy = Taffy::new();
                let root = build_yoga_deep_hierarchy(&mut taffy, 10_000, 10);
                (taffy, root)
            },
            |(mut taffy, root)| taffy.compute_layout(root, Size::MAX_CONTENT).unwrap(),
            criterion::BatchSize::SmallInput,
        )
    });

    group.bench_function("100_000 nodes", |b| {
        b.iter_batched(
            || {
                let mut taffy = Taffy::new();
                let root = build_yoga_deep_hierarchy(&mut taffy, 100_000, 10);
                (taffy, root)
            },
            |(mut taffy, root)| taffy.compute_layout(root, Size::MAX_CONTENT).unwrap(),
            criterion::BatchSize::LargeInput,
        )
    });

    group.bench_function("1_000_000 nodes", |b| {
        b.iter_batched(
            || {
                let mut taffy = Taffy::new();
                let root = build_yoga_deep_hierarchy(&mut taffy, 1_000_000, 10);
                (taffy, root)
            },
            |(mut taffy, root)| taffy.compute_layout(root, Size::MAX_CONTENT).unwrap(),
            criterion::BatchSize::LargeInput,
        )
    });
    drop(group);

    // Decrease sample size, because the tasks take longer
    let mut group = c.benchmark_group("big trees (wide)");
    group.sample_size(10);

    group.bench_function("10 nodes (2-level hierarchy)", |b| {
        b.iter_batched(
            || {
                let mut taffy = Taffy::new();
                let root = build_flat_hierarchy(&mut taffy, 10);
                (taffy, root)
            },
            |(mut taffy, root)| taffy.compute_layout(root, Size::MAX_CONTENT).unwrap(),
            criterion::BatchSize::SmallInput,
        )
    });

    group.bench_function("100 nodes (2-level hierarchy)", |b| {
        b.iter_batched(
            || {
                let mut taffy = Taffy::new();
                let root = build_flat_hierarchy(&mut taffy, 100);
                (taffy, root)
            },
            |(mut taffy, root)| taffy.compute_layout(root, Size::MAX_CONTENT).unwrap(),
            criterion::BatchSize::SmallInput,
        )
    });

    group.bench_function("1_000 nodes (2-level hierarchy)", |b| {
        b.iter_batched(
            || {
                let mut taffy = Taffy::new();
                let root = build_flat_hierarchy(&mut taffy, 1_000);
                (taffy, root)
            },
            |(mut taffy, root)| taffy.compute_layout(root, Size::MAX_CONTENT).unwrap(),
            criterion::BatchSize::SmallInput,
        )
    });

    group.bench_function("10_000 nodes (2-level hierarchy)", |b| {
        b.iter_batched(
            || {
                let mut taffy = Taffy::new();
                let root = build_flat_hierarchy(&mut taffy, 10_000);
                (taffy, root)
            },
            |(mut taffy, root)| taffy.compute_layout(root, Size::MAX_CONTENT).unwrap(),
            criterion::BatchSize::LargeInput,
        )
    });

    group.bench_function("100_000 nodes (2-level hierarchy)", |b| {
        b.iter_batched(
            || {
                let mut taffy = Taffy::new();
                let root = build_flat_hierarchy(&mut taffy, 100_000);
                (taffy, root)
            },
            |(mut taffy, root)| taffy.compute_layout(root, Size::MAX_CONTENT).unwrap(),
            criterion::BatchSize::LargeInput,
        )
    });

    group.bench_function("100_000 nodes (7-level hierarchy)", |b| {
        b.iter_batched(
            || {
                let mut taffy = Taffy::new();
                let root = build_deep_hierarchy(&mut taffy, 100_000, 7);
                (taffy, root)
            },
            |(mut taffy, root)| taffy.compute_layout(root, Size::MAX_CONTENT).unwrap(),
            criterion::BatchSize::LargeInput,
        )
    });

    drop(group);

    // Decrease sample size, because the tasks take longer
    let mut group = c.benchmark_group("big trees (deep)");
    group.sample_size(10);

    group.bench_function("4000 nodes (12-level hierarchy)", |b| {
        b.iter_batched(
            || {
                let mut taffy = Taffy::new();
                let root = build_deep_hierarchy(&mut taffy, 4000, 2);
                (taffy, root)
            },
            |(mut taffy, root)| taffy.compute_layout(root, Size::MAX_CONTENT).unwrap(),
            criterion::BatchSize::SmallInput,
        )
    });

    group.bench_function("10_000 nodes (14-level hierarchy)", |b| {
        b.iter_batched(
            || {
                let mut taffy = Taffy::new();
                let root = build_deep_hierarchy(&mut taffy, 10_000, 2);
                (taffy, root)
            },
            |(mut taffy, root)| taffy.compute_layout(root, Size::MAX_CONTENT).unwrap(),
            criterion::BatchSize::SmallInput,
        )
    });

    group.bench_function("100_000 nodes (17-level hierarchy)", |b| {
        b.iter_batched(
            || {
                let mut taffy = Taffy::new();
                let root = build_deep_hierarchy(&mut taffy, 100_000, 2);
                (taffy, root)
            },
            |(mut taffy, root)| taffy.compute_layout(root, Size::MAX_CONTENT).unwrap(),
            criterion::BatchSize::LargeInput,
        )
    });

    group.bench_function("1_000_000 nodes (20-level hierarchy)", |b| {
        b.iter_batched(
            || {
                let mut taffy = Taffy::new();
                let root = build_deep_hierarchy(&mut taffy, 1_000_000, 2);
                (taffy, root)
            },
            |(mut taffy, root)| taffy.compute_layout(root, Size::MAX_CONTENT).unwrap(),
            criterion::BatchSize::LargeInput,
        )
    });

    drop(group);

    let mut group = c.benchmark_group("super deep trees");
    group.sample_size(10);

    group.bench_function("100 nodes (100-level hierarchy)", |b| {
        b.iter_batched(
            || {
                let mut taffy = Taffy::new();
                let root = build_deep_hierarchy(&mut taffy, 100, 1);
                (taffy, root)
            },
            |(mut taffy, root)| taffy.compute_layout(root, Size::MAX_CONTENT).unwrap(),
            criterion::BatchSize::SmallInput,
        )
    });

    group.bench_function("1_000 nodes (1000-level hierarchy)", |b| {
        b.iter_batched(
            || {
                let mut taffy = Taffy::new();
                let root = build_deep_hierarchy(&mut taffy, 1_000, 1);
                (taffy, root)
            },
            |(mut taffy, root)| taffy.compute_layout(root, Size::MAX_CONTENT).unwrap(),
            criterion::BatchSize::SmallInput,
        )
    });
}

criterion_group!(benches, taffy_benchmarks);
criterion_main!(benches);
