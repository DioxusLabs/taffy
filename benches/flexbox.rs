//! This file includes benchmarks for very large, pseudo-randomly generated trees
use std::{any::Any, fmt::format};

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use taffy::prelude::*;
use taffy::randomizable::Randomizeable;
use taffy::style::Style;

mod helpers;
use helpers::{build_deep_tree, build_linear_tree_with_n_children_per_level};

#[cfg(feature = "yoga_benchmark")]
use helpers::yoga_helpers;
#[cfg(feature = "yoga_benchmark")]
use slotmap::SlotMap;
#[cfg(feature = "yoga_benchmark")]
use yoga_helpers::yg;

struct FlexAutoStyle(Style);

impl Randomizeable for FlexAutoStyle {
    fn random<R>(rng: &mut R) -> Self
    where
        R: Rng,
    {
        let mut style = Style::DEFAULT;
        style.margin = Rect {
            left: LengthPercentageAuto::Points(rng.gen::<f32>()),
            right: LengthPercentageAuto::Points(rng.gen()),
            top: LengthPercentageAuto::Points(rng.gen()),
            bottom: LengthPercentageAuto::Points(rng.gen()),
        };
        Self(style)
    }
}

impl From<FlexAutoStyle> for Style {
    fn from(flex_auto_style: FlexAutoStyle) -> Self {
        flex_auto_style.0
    }
}

/// Build a random leaf node
fn _build_random_leaf(taffy: &mut Taffy, rng: &mut ChaCha8Rng) -> Node {
    taffy.new_with_children(Style::random(rng), &[]).unwrap()
}

fn build_random_leaf(taffy: &mut Taffy, style: Style) -> Node {
    taffy.new_with_children(style, &[]).unwrap()
}

/// A tree with many children that have shallow depth
fn build_taffy_flat_hierarchy(total_node_count: u32, make_style: fn(&mut ChaCha8Rng) -> Style) -> (Taffy, Node) {
    let mut taffy = Taffy::new();
    let mut rng = ChaCha8Rng::seed_from_u64(12345);
    let mut children = Vec::new();
    let mut node_count = 0;

    while node_count < total_node_count {
        let sub_children_count = rng.gen_range(1..=4);
        let sub_children: Vec<Node> =
            (0..sub_children_count).map(|_| build_random_leaf(&mut taffy, make_style(&mut rng))).collect();
        let node = taffy.new_with_children(make_style(&mut rng), &sub_children).unwrap();

        children.push(node);
        node_count += 1 + sub_children_count;
    }

    let root = taffy.new_with_children(Style::DEFAULT, children.as_slice()).unwrap();
    (taffy, root)
}

#[cfg(feature = "yoga_benchmark")]
/// A tree with many children that have shallow depth
fn build_yoga_flat_hierarchy(total_node_count: u32, make_style: fn(&mut ChaCha8Rng) -> Style) -> (yg::YogaTree, Node) {
    let mut tree = SlotMap::new();
    let mut rng = ChaCha8Rng::seed_from_u64(12345);
    let mut children = Vec::new();
    let mut node_count = 0;

    while node_count < total_node_count {
        let sub_children_count = rng.gen_range(1..=4);
        let sub_children: Vec<Node> = (0..sub_children_count)
            .map(|_| yoga_helpers::new_with_children(&mut tree, &make_style(&mut rng), vec![]))
            .collect();
        let node = yoga_helpers::new_with_children(&mut tree, &make_style(&mut rng), sub_children);

        children.push(node);
        node_count += 1 + sub_children_count;
    }

    let root = yoga_helpers::new_with_children(&mut tree, &Style::DEFAULT, children);
    (tree, root)
}

/// A tree with a higher depth for a more realistic scenario
fn build_taffy_deep_hierarchy(
    node_count: u32,
    branching_factor: u32,
    make_style: impl Fn(&mut ChaCha8Rng) -> Style,
) -> (Taffy, Node) {
    let mut rng = ChaCha8Rng::seed_from_u64(12345);
    let mut build_leaf_node = |taffy: &mut Taffy| build_random_leaf(taffy, make_style(&mut rng));
    let mut rng = ChaCha8Rng::seed_from_u64(12345);
    let mut build_flex_node =
        |taffy: &mut Taffy, children: Vec<Node>| taffy.new_with_children(make_style(&mut rng), &children).unwrap();

    let mut taffy = Taffy::new();
    let tree = build_deep_tree(&mut taffy, node_count, branching_factor, &mut build_leaf_node, &mut build_flex_node);
    let root = taffy.new_with_children(Style::DEFAULT, &tree).unwrap();
    (taffy, root)
}

fn build_taffy_deep_thin_tree(
    node_count: u32,
    children_per_level: u32,
    make_style: impl Fn(&mut ChaCha8Rng) -> Style,
) -> (Taffy, Node) {
    assert!(node_count < 1001);
    let mut rng = ChaCha8Rng::seed_from_u64(12345);
    let mut build_leaf_node = |taffy: &mut Taffy| build_random_leaf(taffy, make_style(&mut rng));
    let mut append_child = |taffy: &mut Taffy, parent: Node, child: Node| {
        taffy.add_child(parent, child).unwrap();
    };
    let mut taffy = Taffy::new();
    let root = build_linear_tree_with_n_children_per_level(
        &mut taffy,
        node_count,
        children_per_level,
        &mut build_leaf_node,
        &mut append_child,
    );
    (taffy, root)
}

#[cfg(feature = "yoga_benchmark")]
/// A tree with a higher depth for a more realistic scenario
fn build_yoga_deep_hierarchy(
    node_count: u32,
    branching_factor: u32,
    make_style: impl Fn(&mut ChaCha8Rng) -> Style,
) -> (yg::YogaTree, Node) {
    let mut rng = ChaCha8Rng::seed_from_u64(12345);
    let mut build_leaf_node =
        |tree: &mut yg::YogaTree| yoga_helpers::new_with_children(tree, &make_style(&mut rng), vec![]);
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

fn yoga_huge_nested_bench(c: &mut Criterion) {
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
}

fn big_trees_wide_bench(c: &mut Criterion, make_style: fn(&mut ChaCha8Rng) -> Style, style_label: &str) {
    let mut group = c.benchmark_group("big trees (wide)");
    group.sample_size(10);
    for node_count in [1_000u32, 10_000, 100_000].iter() {
        #[cfg(feature = "yoga_benchmark")]
        let benchmark_id = BenchmarkId::new(format!("{style_label}: Yoga (2-level hierarchy)"), node_count);
        #[cfg(feature = "yoga_benchmark")]
        group.bench_with_input(benchmark_id, node_count, |b, &node_count| {
            b.iter_batched(
                || build_yoga_flat_hierarchy(node_count, make_style),
                |(mut tree, root)| {
                    tree[root].calculate_layout(f32::INFINITY, f32::INFINITY, yg::Direction::LTR);
                },
                criterion::BatchSize::SmallInput,
            )
        });
        let benchmark_id = BenchmarkId::new(format!("{style_label}: Taffy (2-level hierarchy)"), node_count);
        group.bench_with_input(benchmark_id, node_count, |b, &node_count| {
            b.iter_batched(
                || build_taffy_flat_hierarchy(node_count, make_style),
                |(mut taffy, root)| taffy.compute_layout(root, Size::MAX_CONTENT).unwrap(),
                criterion::BatchSize::SmallInput,
            )
        });
    }
    group.finish();
}

fn big_trees_deep_bench(c: &mut Criterion, make_style: fn(&mut ChaCha8Rng) -> Style, style_label: &str) {
    let mut group = c.benchmark_group("big trees (deep)");
    group.sample_size(10);
    let benches = [(4000, "(12-level hierarchy)"), (10_000, "(14-level hierarchy)"), (100_000, "(17-level hierarchy)")];
    for (node_count, label) in benches.iter() {
        #[cfg(feature = "yoga_benchmark")]
        group.bench_with_input(
            BenchmarkId::new(format!("{style_label}: Yoga {label} "), node_count),
            node_count,
            |b, &node_count| {
                b.iter_batched(
                    || build_yoga_deep_hierarchy(node_count, 2, make_style),
                    |(mut tree, root)| {
                        tree[root].calculate_layout(f32::INFINITY, f32::INFINITY, yg::Direction::LTR);
                    },
                    criterion::BatchSize::SmallInput,
                )
            },
        );
        group.bench_with_input(
            BenchmarkId::new(format!("{style_label}: Taffy {label}"), node_count),
            node_count,
            |b, &node_count| {
                b.iter_batched(
                    || build_taffy_deep_hierarchy(node_count, 2, make_style),
                    |(mut taffy, root)| taffy.compute_layout(root, Size::MAX_CONTENT).unwrap(),
                    criterion::BatchSize::SmallInput,
                )
            },
        );
    }
    group.finish();
}

fn super_deep_bench(c: &mut Criterion, make_style: fn(&mut ChaCha8Rng) -> Style, style_label: &str) {
    let mut group = c.benchmark_group("super deep (1000-level hierarchy)");
    group.sample_size(10);
    for node_count in [1000u32].iter() {
        #[cfg(feature = "yoga_benchmark")]
        group.bench_with_input(
            BenchmarkId::new(format!("{style_label}: Yoga"), node_count),
            node_count,
            |b, &node_count| {
                b.iter_batched(
                    || build_yoga_deep_hierarchy(node_count, 2, make_style),
                    |(mut tree, root)| {
                        tree[root].calculate_layout(f32::INFINITY, f32::INFINITY, yg::Direction::LTR);
                    },
                    criterion::BatchSize::SmallInput,
                )
            },
        );
        group.bench_with_input(
            BenchmarkId::new(format!("{style_label}: Taffy"), node_count),
            node_count,
            |b, &node_count| {
                b.iter_batched(
                    || build_taffy_deep_hierarchy(node_count, 2, make_style),
                    |(mut taffy, root)| taffy.compute_layout(root, Size::MAX_CONTENT).unwrap(),
                    criterion::BatchSize::SmallInput,
                )
            },
        );
    }
    group.finish();
}

fn super_deep_thin_tree_bench(c: &mut Criterion, make_style: fn(&mut ChaCha8Rng) -> Style, style_label: &str) {
    let mut group = c.benchmark_group("Flex Auto tree with 150 level depth (2 siblings per level)");
    group.sample_size(10);
    for node_count in [300u32].iter() {
        #[cfg(feature = "yoga_benchmark")]
        group.bench_with_input(
            BenchmarkId::new(format!("{style_label}: Yoga"), node_count),
            node_count,
            |b, &node_count| {
                b.iter_batched(
                    || build_yoga_deep_hierarchy(node_count, 1, make_style),
                    |(mut tree, root)| {
                        tree[root].calculate_layout(f32::INFINITY, f32::INFINITY, yg::Direction::LTR);
                    },
                    criterion::BatchSize::SmallInput,
                )
            },
        );
        group.bench_with_input(
            BenchmarkId::new(format!("{style_label} Taffy"), node_count),
            node_count,
            |b, &node_count| {
                b.iter_batched(
                    || build_taffy_deep_thin_tree(node_count, 2, make_style),
                    |(mut taffy, root)| taffy.compute_layout(root, Size::MAX_CONTENT).unwrap(),
                    criterion::BatchSize::SmallInput,
                )
            },
        );
    }
    group.finish();
}

fn make_random_fixed_style(rng: &mut ChaCha8Rng) -> Style {
    Style::random(rng)
}

fn make_random_flex_auto_style(rng: &mut ChaCha8Rng) -> Style {
    FlexAutoStyle::random(rng).into()
}

fn make_benchmark_variants(c: &mut Criterion, bench_fn: impl Fn(&mut Criterion, fn(&mut ChaCha8Rng) -> Style, &str)) {
    type StyleConfig = (fn(&mut ChaCha8Rng) -> Style, &'static str);

    let style_configs: [StyleConfig; 2] =
        [(make_random_fixed_style, "Fixed Style"), (make_random_flex_auto_style, "Flex Auto Style")];

    for (make_style, style_label) in style_configs.iter() {
        bench_fn(c, *make_style, style_label);
    }
}

fn taffy_benchmarks(c: &mut Criterion) {
    // Decrease sample size, because the tasks take longer
    // no variants to make because yoga benchmarks use fixed dimensions
    yoga_huge_nested_bench(c);
    make_benchmark_variants(c, big_trees_wide_bench);

    // Decrease sample size, because the tasks take longer
    make_benchmark_variants(c, big_trees_deep_bench);

    make_benchmark_variants(c, super_deep_bench);
    make_benchmark_variants(c, super_deep_thin_tree_bench)
}

criterion_group!(benches, taffy_benchmarks);
criterion_main!(benches);
