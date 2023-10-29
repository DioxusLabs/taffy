//! This file includes benchmarks for very large, pseudo-randomly generated trees
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use rand::Rng;
use taffy::prelude::*;
use taffy::style::Dimension;
use taffy::style::Style as TaffyStyle;

use taffy_benchmarks::{BuildTreeExt, FixedStyleGenerator, GenStyle, TaffyTreeBuilder};

#[cfg(feature = "taffy03")]
use taffy_benchmarks::taffy_03_helpers::Taffy03TreeBuilder;
#[cfg(feature = "yoga")]
use taffy_benchmarks::yoga_helpers::{yg, YogaTreeBuilder};

fn random_dimension(rng: &mut impl Rng) -> Dimension {
    match rng.gen_range(0.0..=1.0) {
        rand if rand < 0.2 => Dimension::Auto,
        rand if rand < 0.8 => Dimension::Length(rng.gen_range(0.0..500.0)),
        _ => Dimension::Percent(rng.gen_range(0.0..1.0)),
    }
}

#[derive(Clone)]
pub struct RandomStyleGenerator;
impl GenStyle<TaffyStyle> for RandomStyleGenerator {
    fn create_leaf_style(&mut self, rng: &mut impl Rng) -> TaffyStyle {
        TaffyStyle { size: Size { width: random_dimension(rng), height: random_dimension(rng) }, ..Default::default() }
    }
    fn create_container_style(&mut self, rng: &mut impl Rng) -> TaffyStyle {
        TaffyStyle { size: Size { width: random_dimension(rng), height: random_dimension(rng) }, ..Default::default() }
    }
}

// fn with_each_library<G, TreeBuilder, F>(style_generator: G, mut build_tree: F)
// where
//     G: GenStyle<TaffyStyle>,
//     TreeBuilder: BuildTreeExt<G>,
//     F: FnMut(&mut dyn BuildTreeExt<G>)
// {
//     #[cfg(feature = "yoga")]
//     let tree = YogaTreeBuilder::new(style_generator());

//     let mut tree = TaffyTreeBuilder::new(style_generator.clone());
//     build_tree(&mut tree as &mut dyn BuildTreeExt<G>);
// }

/// A deep tree that matches the shape and styling that yoga use on their benchmarks
fn build_flat_hierarchy<G: GenStyle<TaffyStyle>, TreeBuilder: BuildTreeExt<G>>(
    target_node_count: u32,
    style_generator: impl FnOnce() -> G,
) -> TreeBuilder {
    let mut tree_builder = TreeBuilder::new(style_generator());
    tree_builder.build_flat_hierarchy(target_node_count);
    tree_builder
}

/// A deep tree that matches the shape and styling that yoga use on their benchmarks
fn build_deep_hierarchy<G: GenStyle<TaffyStyle>, TreeBuilder: BuildTreeExt<G>>(
    node_count: u32,
    branching_factor: u32,
    style_generator: impl FnOnce() -> G,
) -> TreeBuilder {
    let mut tree_builder = TreeBuilder::new(style_generator());
    tree_builder.build_deep_hierarchy(node_count, branching_factor);
    tree_builder
}

/// A deep tree that matches the shape and styling that yoga use on their benchmarks
fn build_super_deep_hierarchy<G: GenStyle<TaffyStyle>, TreeBuilder: BuildTreeExt<G>>(
    depth: u32,
    nodes_per_level: u32,
    style_generator: impl FnOnce() -> G,
) -> TreeBuilder {
    let mut tree_builder = TreeBuilder::new(style_generator());
    tree_builder.build_super_deep_hierarchy(depth, nodes_per_level);
    tree_builder
}

/// A deep tree that matches the shape and styling that yoga use on their benchmarks
fn build_huge_nested_hierarchy<G: GenStyle<TaffyStyle>, TreeBuilder: BuildTreeExt<G>>(
    node_count: u32,
    branching_factor: u32,
    style_generator: impl FnOnce() -> G,
) -> TreeBuilder {
    let mut tree_builder = TreeBuilder::new(style_generator());
    tree_builder.build_deep_hierarchy(node_count, branching_factor);
    tree_builder
}

fn huge_nested_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("yoga 'huge nested'");
    let style = Style { size: length(10.0), flex_grow: 1.0, ..Default::default() };
    let style_gen = || FixedStyleGenerator(style.clone());
    for node_count in [
        #[cfg(feature = "small")]
        1_000u32,
        10_000,
        #[cfg(feature = "large")]
        100_000,
    ]
    .iter()
    {
        #[cfg(feature = "yoga")]
        group.bench_with_input(BenchmarkId::new("Yoga", node_count), node_count, |b, &node_count| {
            b.iter_batched(
                || build_huge_nested_hierarchy::<_, YogaTreeBuilder<_, _>>(node_count, 10, style_gen),
                |mut builder| builder.compute_layout(None, None),
                criterion::BatchSize::SmallInput,
            )
        });
        #[cfg(feature = "taffy03")]
        group.bench_with_input(BenchmarkId::new("Taffy 0.3", node_count), node_count, |b, &node_count| {
            b.iter_batched(
                || build_huge_nested_hierarchy::<_, Taffy03TreeBuilder<_, _>>(node_count, 10, style_gen),
                |mut builder| builder.compute_layout(None, None),
                criterion::BatchSize::SmallInput,
            )
        });
        group.bench_with_input(BenchmarkId::new("Taffy 0.4", node_count), node_count, |b, &node_count| {
            b.iter_batched(
                || build_huge_nested_hierarchy::<_, TaffyTreeBuilder<_, _>>(node_count, 10, style_gen),
                |mut builder| builder.compute_layout(None, None),
                criterion::BatchSize::SmallInput,
            )
        });
    }
    group.finish();
}

fn wide_benchmarks(c: &mut Criterion) {
    // Decrease sample size, because the tasks take longer
    let mut group = c.benchmark_group("Wide tree");
    group.sample_size(10);
    for node_count in [
        #[cfg(feature = "small")]
        1_000u32,
        10_000,
        #[cfg(feature = "large")]
        100_000,
    ]
    .iter()
    {
        #[cfg(feature = "yoga")]
        let benchmark_id = BenchmarkId::new(format!("Yoga (2-level hierarchy)"), node_count);
        #[cfg(feature = "yoga")]
        group.bench_with_input(benchmark_id, node_count, |b, &node_count| {
            b.iter_batched(
                || build_flat_hierarchy::<_, YogaTreeBuilder<_, _>>(node_count, || RandomStyleGenerator),
                |mut builder| builder.compute_layout(None, None),
                criterion::BatchSize::SmallInput,
            )
        });
        #[cfg(feature = "taffy03")]
        group.bench_with_input(
            BenchmarkId::new("Taffy 0.3 (2-level hierarchy)", node_count),
            node_count,
            |b, &node_count| {
                b.iter_batched(
                    || build_flat_hierarchy::<_, Taffy03TreeBuilder<_, _>>(node_count, || RandomStyleGenerator),
                    |mut builder| builder.compute_layout(None, None),
                    criterion::BatchSize::LargeInput,
                )
            },
        );
        let benchmark_id = BenchmarkId::new(format!("Taffy 0.4 (2-level hierarchy)"), node_count);
        group.bench_with_input(benchmark_id, node_count, |b, &node_count| {
            b.iter_batched(
                || build_flat_hierarchy::<_, TaffyTreeBuilder<_, _>>(node_count, || RandomStyleGenerator),
                |mut builder| builder.compute_layout(None, None),
                criterion::BatchSize::SmallInput,
            )
        });
    }
    group.finish();
}

fn deep_random_benchmarks(c: &mut Criterion) {
    // Decrease sample size, because the tasks take longer
    let mut group = c.benchmark_group("Deep tree (random size)");
    group.sample_size(10);
    let benches = [
        (4000, "(12-level hierarchy)"),
        (10_000, "(14-level hierarchy)"),
        #[cfg(feature = "large")]
        (100_000, "(17-level hierarchy)"),
    ];
    for (node_count, label) in benches.iter() {
        #[cfg(feature = "yoga")]
        group.bench_with_input(BenchmarkId::new(format!("Yoga {label}"), node_count), node_count, |b, &node_count| {
            b.iter_batched(
                || build_deep_hierarchy::<_, YogaTreeBuilder<_, _>>(node_count, 2, || RandomStyleGenerator),
                |mut builder| builder.compute_layout(None, None),
                criterion::BatchSize::SmallInput,
            )
        });
        #[cfg(feature = "taffy03")]
        group.bench_with_input(
            BenchmarkId::new(format!("Taffy 0.3 {label}"), node_count),
            node_count,
            |b, &node_count| {
                b.iter_batched(
                    || build_deep_hierarchy::<_, Taffy03TreeBuilder<_, _>>(node_count, 2, || RandomStyleGenerator),
                    |mut builder| builder.compute_layout(None, None),
                    criterion::BatchSize::SmallInput,
                )
            },
        );
        group.bench_with_input(
            BenchmarkId::new(format!("Taffy 0.4 {label}"), node_count),
            node_count,
            |b, &node_count| {
                b.iter_batched(
                    || build_deep_hierarchy::<_, TaffyTreeBuilder<_, _>>(node_count, 2, || RandomStyleGenerator),
                    |mut builder| builder.compute_layout(None, None),
                    criterion::BatchSize::SmallInput,
                )
            },
        );
    }
    group.finish();
}

fn deep_auto_benchmarks(c: &mut Criterion) {
    // Decrease sample size, because the tasks take longer
    let mut group = c.benchmark_group("Deep tree (auto size)");
    group.sample_size(10);
    let style = Style { flex_grow: 1.0, margin: length(10.0), ..Default::default() };
    let style_gen = || FixedStyleGenerator(style.clone());
    let benches = [
        (4000, "(12-level hierarchy)"),
        (10_000, "(14-level hierarchy)"),
        #[cfg(feature = "large")]
        (100_000, "(17-level hierarchy)"),
    ];
    for (node_count, label) in benches.iter() {
        #[cfg(feature = "yoga")]
        group.bench_with_input(BenchmarkId::new(format!("Yoga {label}"), node_count), node_count, |b, &node_count| {
            b.iter_batched(
                || build_deep_hierarchy::<_, YogaTreeBuilder<_, _>>(node_count, 2, style_gen),
                |mut builder| builder.compute_layout(None, None),
                criterion::BatchSize::SmallInput,
            )
        });
        #[cfg(feature = "taffy03")]
        group.bench_with_input(
            BenchmarkId::new(format!("Taffy 0.3 {label}"), node_count),
            node_count,
            |b, &node_count| {
                b.iter_batched(
                    || build_deep_hierarchy::<_, Taffy03TreeBuilder<_, _>>(node_count, 2, style_gen),
                    |mut builder| builder.compute_layout(None, None),
                    criterion::BatchSize::SmallInput,
                )
            },
        );
        group.bench_with_input(
            BenchmarkId::new(format!("Taffy 0.4 {label}"), node_count),
            node_count,
            |b, &node_count| {
                b.iter_batched(
                    || build_deep_hierarchy::<_, TaffyTreeBuilder<_, _>>(node_count, 2, style_gen),
                    |mut builder| builder.compute_layout(None, None),
                    criterion::BatchSize::SmallInput,
                )
            },
        );
    }
    group.finish();
}

fn super_deep_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("super deep");
    group.sample_size(10);
    #[derive(Clone)]
    struct SuperDeepStyleGen;
    impl GenStyle<TaffyStyle> for SuperDeepStyleGen {
        fn create_leaf_style(&mut self, _rng: &mut impl Rng) -> TaffyStyle {
            // let flex_direction = if rng.gen::<f32>() < 0.5 { FlexDirection::Column } else { FlexDirection::Row };
            let flex_direction = FlexDirection::Row;
            Style { flex_direction, flex_grow: 1.0, margin: length(10.0), ..Default::default() }
        }
        fn create_container_style(&mut self, rng: &mut impl Rng) -> TaffyStyle {
            self.create_leaf_style(rng)
        }
    }
    for depth in [
        #[cfg(feature = "small")]
        50u32,
        100,
        #[cfg(feature = "large")]
        200,
    ]
    .iter()
    {
        // Yoga is particularly slow at these benchmarks, so we gate them behind a separate feature flag
        #[cfg(all(feature = "yoga", feature = "yoga-super-deep"))]
        group.bench_with_input(BenchmarkId::new("Yoga", depth), depth, |b, &depth| {
            b.iter_batched(
                || build_super_deep_hierarchy::<_, YogaTreeBuilder<_, _>>(depth, 3, || SuperDeepStyleGen),
                |mut builder| builder.compute_layout(Some(800.0), Some(800.0)),
                criterion::BatchSize::SmallInput,
            )
        });
        #[cfg(feature = "taffy03")]
        group.bench_with_input(BenchmarkId::new("Taffy 0.3", depth), depth, |b, &depth| {
            b.iter_batched(
                || build_super_deep_hierarchy::<_, Taffy03TreeBuilder<_, _>>(depth, 3, || SuperDeepStyleGen),
                |mut builder| builder.compute_layout(Some(800.0), Some(800.0)),
                criterion::BatchSize::SmallInput,
            )
        });
        group.bench_with_input(BenchmarkId::new("Taffy 0.4", depth), depth, |b, &depth| {
            b.iter_batched(
                || build_super_deep_hierarchy::<_, TaffyTreeBuilder<_, _>>(depth, 3, || SuperDeepStyleGen),
                |mut builder| builder.compute_layout(Some(800.0), Some(800.0)),
                criterion::BatchSize::SmallInput,
            )
        });
    }
    group.finish();
}

fn taffy_benchmarks(c: &mut Criterion) {
    huge_nested_benchmarks(c);
    wide_benchmarks(c);
    deep_auto_benchmarks(c);
    deep_random_benchmarks(c);
    super_deep_benchmarks(c);
}

criterion_group!(benches, taffy_benchmarks);
criterion_main!(benches);
