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
use taffy_benchmarks::yoga_helpers::YogaTreeBuilder;

fn random_dimension(rng: &mut impl Rng) -> Dimension {
    match rng.random_range(0.0..=1.0) {
        rand if rand < 0.2 => auto(),
        rand if rand < 0.8 => length(rng.random_range(0.0..500.0)),
        _ => percent(rng.random_range(0.0..1.0)),
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

macro_rules! run_benchmark {
    ($TreeBuilder: ty, $tree_builder_name: literal, $benchmark_name: expr, $group: ident, $builder: ident, $params: expr, $generate_style: expr, $generate_tree: expr) => {
        let benchmark_id = BenchmarkId::new(format!("{} {}", $tree_builder_name, $benchmark_name), $params);
        $group.bench_with_input(benchmark_id, $params, |b, _params| {
            b.iter_batched(
                || -> $TreeBuilder {
                    let mut $builder = <$TreeBuilder>::new($generate_style());
                    $generate_tree;
                    $builder
                },
                |mut builder| builder.compute_layout(None, None),
                criterion::BatchSize::SmallInput,
            )
        });
    };
}

macro_rules! benchmark_each_library {
    ($benchmark_name: expr, $group: ident, $builder: ident, $params: expr, $generate_style: expr, $generate_tree: expr) => {
        #[cfg(feature = "yoga")]
        run_benchmark!(YogaTreeBuilder<_, _>, "Yoga", $benchmark_name, $group, $builder, $params, $generate_style, $generate_tree);
        #[cfg(feature = "taffy03")]
        run_benchmark!(Taffy03TreeBuilder<_, _>, "Taffy 0.3", $benchmark_name, $group, $builder, $params, $generate_style, $generate_tree);

        run_benchmark!(TaffyTreeBuilder<_, _>, "Taffy 0.7", $benchmark_name, $group, $builder, $params, $generate_style, $generate_tree);
    };
}

fn huge_nested_benchmarks(c: &mut Criterion) {
    let node_counts = [
        #[cfg(feature = "small")]
        1_000u32,
        10_000,
        #[cfg(feature = "large")]
        100_000,
    ];

    let mut group = c.benchmark_group("yoga 'huge nested'");
    let style = Style { size: length(10.0), flex_grow: 1.0, ..Default::default() };
    for node_count in node_counts.iter() {
        benchmark_each_library!(
            "",
            group,
            builder,
            node_count,
            || FixedStyleGenerator(style.clone()),
            builder.build_deep_hierarchy(*node_count, 10)
        );
    }
    group.finish();
}

fn wide_benchmarks(c: &mut Criterion) {
    let node_counts = [
        #[cfg(feature = "small")]
        1_000u32,
        10_000,
        #[cfg(feature = "large")]
        100_000,
    ];

    let mut group = c.benchmark_group("Wide tree");
    group.sample_size(10); // Decrease sample size, because the tasks take longer
    for node_count in node_counts.iter() {
        benchmark_each_library!(
            "(2-level hierarchy)",
            group,
            builder,
            node_count,
            || RandomStyleGenerator,
            builder.build_flat_hierarchy(*node_count)
        );
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
        benchmark_each_library!(
            label,
            group,
            builder,
            node_count,
            || RandomStyleGenerator,
            builder.build_deep_hierarchy(*node_count, 2)
        );
    }
    group.finish();
}

fn deep_auto_benchmarks(c: &mut Criterion) {
    let benches = [
        (4000, "(12-level hierarchy)"),
        (10_000, "(14-level hierarchy)"),
        #[cfg(feature = "large")]
        (100_000, "(17-level hierarchy)"),
    ];
    let style = Style { flex_grow: 1.0, margin: length(10.0), ..Default::default() };

    let mut group = c.benchmark_group("Deep tree (auto size)");
    group.sample_size(10); // Decrease sample size, because the tasks take longer
    for (node_count, label) in benches.iter() {
        benchmark_each_library!(
            label,
            group,
            builder,
            node_count,
            || FixedStyleGenerator(style.clone()),
            builder.build_deep_hierarchy(*node_count, 2)
        );
    }
    group.finish();
}

fn super_deep_benchmarks(c: &mut Criterion) {
    let benches = [
        #[cfg(feature = "small")]
        50u32,
        100,
        #[cfg(feature = "large")]
        200,
    ];

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

    let mut group = c.benchmark_group("super deep");
    group.sample_size(10);

    for depth in benches.iter() {
        // Yoga is particularly slow at these benchmarks, so we gate them behind a separate feature flag
        #[cfg(all(feature = "yoga", feature = "yoga-super-deep"))]
        run_benchmark!(
            YogaTreeBuilder<_,_>,
            "Yoga",
            "",
            group,
            builder,
            depth,
            || SuperDeepStyleGen,
            builder.build_super_deep_hierarchy(*depth, 3)
        );
        #[cfg(feature = "taffy03")]
        run_benchmark!(
            Taffy03TreeBuilder<_,_>,
            "Taffy 0.3",
            "",
            group,
            builder,
            depth,
            || SuperDeepStyleGen,
            builder.build_super_deep_hierarchy(*depth, 3)
        );

        run_benchmark!(
            TaffyTreeBuilder<_,_>,
            "Taffy 0.7",
            "",
            group,
            builder,
            depth,
            || SuperDeepStyleGen,
            builder.build_super_deep_hierarchy(*depth, 3)
        );
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
