//! This file includes benchmarks for very large, pseudo-randomly generated trees
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use taffy::prelude::*;
use taffy::randomizable::Randomizeable;
use taffy::style::Style;

mod helpers;
use helpers::build_deep_tree;

#[cfg(feature = "yoga_benchmark")]
use helpers::yoga_helpers;
#[cfg(feature = "yoga_benchmark")]
use slotmap::SlotMap;
#[cfg(feature = "yoga_benchmark")]
use yoga_helpers::yg;

/// Build a random leaf node
fn build_random_leaf(taffy: &mut Taffy, rng: &mut ChaCha8Rng) -> Node {
    taffy.new_with_children(Style::random(rng), &[]).unwrap()
}

/// A tree with many children that have shallow depth
fn build_taffy_flat_hierarchy(total_node_count: u32) -> (Taffy, Node) {
    let mut taffy = Taffy::new();
    let mut rng = ChaCha8Rng::seed_from_u64(12345);
    let mut children = Vec::new();
    let mut node_count = 0;

    while node_count < total_node_count {
        let sub_children_count = rng.gen_range(1..=4);
        let sub_children: Vec<Node> =
            (0..sub_children_count).map(|_| build_random_leaf(&mut taffy, &mut rng)).collect();
        let node = taffy.new_with_children(Style::random(&mut rng), &sub_children).unwrap();

        children.push(node);
        node_count += 1 + sub_children_count;
    }

    let root = taffy.new_with_children(Style::DEFAULT, children.as_slice()).unwrap();
    (taffy, root)
}

#[cfg(feature = "yoga_benchmark")]
/// A tree with many children that have shallow depth
fn build_yoga_flat_hierarchy(total_node_count: u32) -> (yg::YogaTree, Node) {
    let mut tree = SlotMap::new();
    let mut rng = ChaCha8Rng::seed_from_u64(12345);
    let mut children = Vec::new();
    let mut node_count = 0;

    while node_count < total_node_count {
        let sub_children_count = rng.gen_range(1..=4);
        let sub_children: Vec<Node> = (0..sub_children_count)
            .map(|_| yoga_helpers::new_with_children(&mut tree, &Style::random(&mut rng), vec![]))
            .collect();
        let node = yoga_helpers::new_with_children(&mut tree, &Style::random(&mut rng), sub_children);

        children.push(node);
        node_count += 1 + sub_children_count;
    }

    let root = yoga_helpers::new_with_children(&mut tree, &Style::DEFAULT, children);
    (tree, root)
}

/// A tree with a higher depth for a more realistic scenario
fn build_taffy_deep_hierarchy(node_count: u32, branching_factor: u32) -> (Taffy, Node) {
    let mut rng = ChaCha8Rng::seed_from_u64(12345);
    let mut build_leaf_node = |taffy: &mut Taffy| build_random_leaf(taffy, &mut rng);
    let mut rng = ChaCha8Rng::seed_from_u64(12345);
    let mut build_flex_node =
        |taffy: &mut Taffy, children: Vec<Node>| taffy.new_with_children(Style::random(&mut rng), &children).unwrap();

    let mut taffy = Taffy::new();
    let tree = build_deep_tree(&mut taffy, node_count, branching_factor, &mut build_leaf_node, &mut build_flex_node);
    let root = taffy.new_with_children(Style::DEFAULT, &tree).unwrap();
    (taffy, root)
}

#[cfg(feature = "yoga_benchmark")]
/// A tree with a higher depth for a more realistic scenario
fn build_yoga_deep_hierarchy(node_count: u32, branching_factor: u32) -> (yg::YogaTree, Node) {
    let mut rng = ChaCha8Rng::seed_from_u64(12345);
    let mut build_leaf_node =
        |tree: &mut yg::YogaTree| yoga_helpers::new_with_children(tree, &Style::random(&mut rng), vec![]);
    let mut rng = ChaCha8Rng::seed_from_u64(12345);
    let mut build_flex_node = |tree: &mut yg::YogaTree, children: Vec<Node>| {
        yoga_helpers::new_with_children(tree, &Style::random(&mut rng), children)
    };

    let mut tree = SlotMap::new();
    let children = build_deep_tree(&mut tree, node_count, branching_factor, &mut build_leaf_node, &mut build_flex_node);
    let root = yoga_helpers::new_with_children(&mut tree, &Style::DEFAULT, children);

    (tree, root)
}

/// A deep tree that matches the shape and styling that yoga use on their benchmarks
fn build_taffy_huge_nested_hierarchy(node_count: u32, branching_factor: u32) -> (Taffy, Node) {
    let style = Style {
        size: Size { width: Dimension::Points(10.0), height: Dimension::Points(10.0) },
        flex_grow: 1.0,
        ..Default::default()
    };
    let mut build_leaf_node = |taffy: &mut Taffy| taffy.new_leaf(style.clone()).unwrap();
    let mut build_flex_node =
        |taffy: &mut Taffy, children: Vec<Node>| taffy.new_with_children(style.clone(), &children).unwrap();

    let mut taffy = Taffy::new();
    let tree = build_deep_tree(&mut taffy, node_count, branching_factor, &mut build_leaf_node, &mut build_flex_node);
    let root = taffy.new_with_children(Style::DEFAULT, &tree).unwrap();
    (taffy, root)
}

#[cfg(feature = "yoga_benchmark")]
/// A deep tree that matches the shape and styling that yoga use on their benchmarks
fn build_yoga_huge_nested_hierarchy(node_count: u32, branching_factor: u32) -> (yg::YogaTree, Node) {
    let style = Style {
        size: Size { width: Dimension::Points(10.0), height: Dimension::Points(10.0) },
        flex_grow: 1.0,
        ..Default::default()
    };
    let mut build_leaf_node = |tree: &mut yg::YogaTree| -> Node {
        let mut node = yg::Node::new();
        yoga_helpers::apply_taffy_style(&mut node, &style.clone());
        tree.insert(node)
    };
    let mut build_flex_node = |tree: &mut yg::YogaTree, children: Vec<Node>| -> Node {
        let mut node = yg::Node::new();
        yoga_helpers::apply_taffy_style(&mut node, &style.clone());
        for (i, child) in children.into_iter().enumerate() {
            node.insert_child(&mut tree[child], i as u32);
        }
        tree.insert(node)
    };

    let mut tree = SlotMap::new();
    let children = build_deep_tree(&mut tree, node_count, branching_factor, &mut build_leaf_node, &mut build_flex_node);
    let mut root = yg::Node::new();
    for (i, child) in children.into_iter().enumerate() {
        root.insert_child(&mut tree[child], i as u32);
    }
    let root = tree.insert(root);
    (tree, root)
}

fn taffy_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("yoga 'huge nested'");
    for node_count in [1_000u32, 10_000, 100_000].iter() {
        #[cfg(feature = "yoga_benchmark")]
        group.bench_with_input(BenchmarkId::new("Yoga", node_count), node_count, |b, &node_count| {
            b.iter_batched(
                || build_yoga_huge_nested_hierarchy(node_count, 10),
                |(mut tree, root)| {
                    tree[root].calculate_layout(f32::INFINITY, f32::INFINITY, yg::Direction::LTR);
                },
                criterion::BatchSize::SmallInput,
            )
        });
        group.bench_with_input(BenchmarkId::new("Taffy", node_count), node_count, |b, &node_count| {
            b.iter_batched(
                || build_taffy_huge_nested_hierarchy(node_count, 10),
                |(mut taffy, root)| taffy.compute_layout(root, Size::MAX_CONTENT).unwrap(),
                criterion::BatchSize::SmallInput,
            )
        });
    }
    group.finish();

    // Decrease sample size, because the tasks take longer
    let mut group = c.benchmark_group("big trees (wide)");
    group.sample_size(10);
    for node_count in [1_000u32, 10_000, 100_000].iter() {
        #[cfg(feature = "yoga_benchmark")]
        let benchmark_id = BenchmarkId::new(format!("Yoga (2-level hierarchy)"), node_count);
        #[cfg(feature = "yoga_benchmark")]
        group.bench_with_input(benchmark_id, node_count, |b, &node_count| {
            b.iter_batched(
                || build_yoga_flat_hierarchy(node_count),
                |(mut tree, root)| {
                    tree[root].calculate_layout(f32::INFINITY, f32::INFINITY, yg::Direction::LTR);
                },
                criterion::BatchSize::SmallInput,
            )
        });
        let benchmark_id = BenchmarkId::new(format!("Taffy (2-level hierarchy)"), node_count);
        group.bench_with_input(benchmark_id, node_count, |b, &node_count| {
            b.iter_batched(
                || build_taffy_flat_hierarchy(node_count),
                |(mut taffy, root)| taffy.compute_layout(root, Size::MAX_CONTENT).unwrap(),
                criterion::BatchSize::SmallInput,
            )
        });
    }
    group.finish();

    // Decrease sample size, because the tasks take longer
    let mut group = c.benchmark_group("big trees (deep)");
    group.sample_size(10);
    let benches = [(4000, "(12-level hierarchy)"), (10_000, "(14-level hierarchy)"), (100_000, "(17-level hierarchy)")];
    for (node_count, label) in benches.iter() {
        #[cfg(feature = "yoga_benchmark")]
        group.bench_with_input(BenchmarkId::new(format!("Yoga {label}"), node_count), node_count, |b, &node_count| {
            b.iter_batched(
                || build_yoga_deep_hierarchy(node_count, 2),
                |(mut tree, root)| {
                    tree[root].calculate_layout(f32::INFINITY, f32::INFINITY, yg::Direction::LTR);
                },
                criterion::BatchSize::SmallInput,
            )
        });
        group.bench_with_input(BenchmarkId::new(format!("Taffy {label}"), node_count), node_count, |b, &node_count| {
            b.iter_batched(
                || build_taffy_deep_hierarchy(node_count, 2),
                |(mut taffy, root)| taffy.compute_layout(root, Size::MAX_CONTENT).unwrap(),
                criterion::BatchSize::SmallInput,
            )
        });
    }
    group.finish();

    let mut group = c.benchmark_group("super deep (1000-level hierarchy)");
    group.sample_size(10);
    for node_count in [1000u32].iter() {
        #[cfg(feature = "yoga_benchmark")]
        group.bench_with_input(BenchmarkId::new("Yoga", node_count), node_count, |b, &node_count| {
            b.iter_batched(
                || build_yoga_deep_hierarchy(node_count, 2),
                |(mut tree, root)| {
                    tree[root].calculate_layout(f32::INFINITY, f32::INFINITY, yg::Direction::LTR);
                },
                criterion::BatchSize::SmallInput,
            )
        });
        group.bench_with_input(BenchmarkId::new("Taffy", node_count), node_count, |b, &node_count| {
            b.iter_batched(
                || build_taffy_deep_hierarchy(node_count, 2),
                |(mut taffy, root)| taffy.compute_layout(root, Size::MAX_CONTENT).unwrap(),
                criterion::BatchSize::SmallInput,
            )
        });
    }
    group.finish();
}

criterion_group!(benches, taffy_benchmarks);
criterion_main!(benches);
