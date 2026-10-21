//! Benchmarks for incremental re-layout, which is dominated by layout cache lookups
//! (unlike the other benchmarks, which lay out freshly built trees with empty caches).
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use std::hint::black_box;
use taffy::prelude::*;
use taffy::{Cache, LayoutInput, LayoutOutput, Line, RequestedAxis, RunMode, SizingMode};
use taffy_benchmarks::compute_layout;

/// The layout modes that the containers of a benchmark tree cycle through, by depth
#[derive(Clone, Copy)]
struct Containers(&'static str, &'static [Display]);

const CONTAINERS: [Containers; 4] = [
    Containers("flex", &[Display::Flex]),
    Containers("block", &[Display::Block]),
    Containers("grid", &[Display::Grid]),
    Containers("mixed", &[Display::Flex, Display::Block, Display::Grid]),
];

fn leaf_style(height: f32) -> Style {
    Style { size: Size { width: length(10.0), height: length(height) }, ..Default::default() }
}

fn container_style(display: Display, depth: usize) -> Style {
    Style {
        display,
        flex_direction: [FlexDirection::Column, FlexDirection::Row][depth % 2],
        grid_template_columns: vec![auto(), auto()],
        padding: Rect::length(1.0),
        ..Default::default()
    }
}

/// Builds a tree of auto-sized containers with `fanout` children each and fixed-size leaves at `depth`.
/// Returns the first leaf.
fn build_subtree(
    taffy: &mut TaffyTree,
    containers: Containers,
    fanout: usize,
    depth: usize,
    max_depth: usize,
    first_leaf: &mut Option<NodeId>,
) -> NodeId {
    if depth == max_depth {
        let leaf = taffy.new_leaf(leaf_style(10.0)).unwrap();
        first_leaf.get_or_insert(leaf);
        return leaf;
    }
    let children: Vec<_> =
        (0..fanout).map(|_| build_subtree(taffy, containers, fanout, depth + 1, max_depth, first_leaf)).collect();
    let display = containers.1[depth % containers.1.len()];
    taffy.new_with_children(container_style(display, depth), &children).unwrap()
}

fn build_tree(containers: Containers, fanout: usize, max_depth: usize) -> (TaffyTree, NodeId, NodeId) {
    let mut taffy = TaffyTree::new();
    // Rounding visits every node of the tree on every layout, which would dominate these benchmarks
    taffy.disable_rounding();
    let mut first_leaf = None;
    let root = build_subtree(&mut taffy, containers, fanout, 0, max_depth, &mut first_leaf);
    compute_layout(&mut taffy, root, Size { width: length(2000.0), height: max_content() });
    (taffy, root, first_leaf.unwrap())
}

/// Lay out a tree, then repeatedly resize one leaf and lay the tree out again. Each ancestor of the
/// leaf is laid out again, and looks its other children up in their caches.
fn relayout_benchmarks(c: &mut Criterion) {
    for (shape, fanout, depth) in [("wide", 100, 2), ("balanced", 10, 4), ("deep", 2, 12)] {
        let mut group = c.benchmark_group(format!("relayout/{shape}"));
        for containers in CONTAINERS {
            let (mut taffy, root, leaf) = build_tree(containers, fanout, depth);
            let node_count = taffy.total_node_count();
            let mut tall = false;
            group.bench_function(BenchmarkId::new(containers.0, node_count), |b| {
                b.iter(|| {
                    tall = !tall;
                    taffy.set_style(leaf, leaf_style(if tall { 20.0 } else { 10.0 })).unwrap();
                    compute_layout(&mut taffy, root, Size { width: length(2000.0), height: max_content() });
                })
            });
        }
        group.finish();
    }

    // Laying out a tree whose caches are all still valid for a different available space: the root is
    // laid out again and every node whose inputs did not change is answered from its cache.
    let mut group = c.benchmark_group("relayout/resize");
    for containers in CONTAINERS {
        let (mut taffy, root, _) = build_tree(containers, 10, 4);
        let node_count = taffy.total_node_count();
        let mut wide = false;
        group.bench_function(BenchmarkId::new(containers.0, node_count), |b| {
            b.iter(|| {
                wide = !wide;
                let width = if wide { 2500.0 } else { 2000.0 };
                compute_layout(&mut taffy, root, Size { width: length(width), height: max_content() });
            })
        });
    }
    group.finish();
}

fn measure_input(width: f32) -> LayoutInput {
    LayoutInput {
        run_mode: RunMode::ComputeSize,
        sizing_mode: SizingMode::InherentSize,
        axis: RequestedAxis::Both,
        known_dimensions: Size { width: Some(width), height: None },
        known_dimensions_are_definite: Size { width: true, height: true },
        parent_size: Size { width: Some(1000.0), height: None },
        available_space: Size { width: AvailableSpace::MaxContent, height: AvailableSpace::MaxContent },
        vertical_margins_are_collapsible: Line::FALSE,
    }
}

fn layout_input(width: f32) -> LayoutInput {
    LayoutInput { run_mode: RunMode::PerformLayout, ..measure_input(width) }
}

fn output(width: f32) -> LayoutOutput {
    LayoutOutput::from_outer_size(Size { width, height: width })
}

/// Micro-benchmarks of the layout cache itself
fn cache_benchmarks(c: &mut Criterion) {
    /// The number of measure entries that a cache holds
    const MEASURE_ENTRIES: usize = 9;

    let mut full_cache = Cache::new();
    full_cache.store(&layout_input(0.0), output(0.0));
    for width in 0..MEASURE_ENTRIES {
        full_cache.store(&measure_input(width as f32), output(width as f32));
    }

    let mut group = c.benchmark_group("cache");

    // Looks up each of the nine measure entries of a full cache once
    group.bench_function("get_measure_hit_x9", |b| {
        let inputs: Vec<_> = (0..MEASURE_ENTRIES).map(|width| measure_input(width as f32)).collect();
        let mut cache = full_cache.clone();
        b.iter(|| {
            for input in &inputs {
                black_box(black_box(&mut cache).get(black_box(input)));
            }
        })
    });

    // A lookup that compares against all nine measure entries of a full cache without finding a match
    group.bench_function("get_measure_miss", |b| {
        let input = measure_input(100.0);
        let mut cache = full_cache.clone();
        b.iter(|| black_box(black_box(&mut cache).get(black_box(&input))))
    });

    group.bench_function("get_layout_hit", |b| {
        let input = layout_input(0.0);
        let mut cache = full_cache.clone();
        b.iter(|| black_box(black_box(&mut cache).get(black_box(&input))))
    });

    // Fills an empty cache: one final layout and nine measurements
    group.bench_function("store_x10", |b| {
        let layout = layout_input(0.0);
        let inputs: Vec<_> = (0..MEASURE_ENTRIES).map(|width| measure_input(width as f32)).collect();
        b.iter(|| {
            let mut cache = Cache::new();
            cache.store(black_box(&layout), output(0.0));
            for input in &inputs {
                cache.store(black_box(input), output(1.0));
            }
            black_box(cache)
        })
    });

    group.finish();
}

// These benchmarks time many iterations per sample (with `Bencher::iter`), as an iteration can take only a few
// nanoseconds. Samples are cheap to collect, so we take more of them than the other benchmarks do.
criterion_group!(
    name = benches;
    config = taffy_benchmarks::criterion_config().sample_size(50);
    targets = relayout_benchmarks, cache_benchmarks
);
criterion_main!(benches);
