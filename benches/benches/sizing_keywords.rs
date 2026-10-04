//! Benchmarks for sizing keywords (`min-content`, `max-content`, `fit-content` and `stretch`) in `min_size`
//! and `max_size`, compared with the same trees without a min or max size and with a length.
//!
//! The number of times that the leaves' measure function is called by each layout is printed alongside
//! the timings, as resolving a content-based keyword may require measuring a node's content.
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use std::sync::atomic::{AtomicUsize, Ordering};
use taffy::prelude::*;
use taffy_benchmarks::{bench_layout, benchmark_group, TaffyLayoutTree};

static MEASURE_COUNT: AtomicUsize = AtomicUsize::new(0);

/// Measures a leaf like text with a min-content width of 20px, a max-content width of 80px and 10px tall lines
fn measure_text(known_dimensions: Size<Option<f32>>, available_space: Size<AvailableSpace>) -> Size<f32> {
    MEASURE_COUNT.fetch_add(1, Ordering::Relaxed);
    let width = known_dimensions.width.unwrap_or(match available_space.width {
        AvailableSpace::MinContent => 20.0,
        AvailableSpace::MaxContent => 80.0,
        AvailableSpace::Definite(width) => width.clamp(20.0, 80.0),
    });
    let height = known_dimensions.height.unwrap_or((80.0 / width.max(20.0)).ceil() * 10.0);
    Size { width, height }
}

/// The min and max sizes that are applied to the nodes of a benchmark tree
#[derive(Clone, Copy)]
struct Variant {
    name: &'static str,
    /// The `size.width` of the leaves
    leaf_width: Dimension,
    /// The `min_size.width` and `max_size.width` of the leaves
    leaf_min_max_width: (Dimension, Dimension),
    /// The `min_size` and `max_size` of the containers (in both axes)
    container_min_max: (Dimension, Dimension),
}

fn variants() -> [Variant; 8] {
    let auto = Dimension::auto();
    let none = (auto, auto);
    let min_max_content = (Dimension::min_content(), Dimension::max_content());
    [
        // Leaves whose width is determined by their content
        Variant { name: "auto/none", leaf_width: auto, leaf_min_max_width: none, container_min_max: none },
        Variant {
            name: "auto/length",
            leaf_width: auto,
            leaf_min_max_width: (length(20.0), length(80.0)),
            container_min_max: (length(1.0), length(100_000.0)),
        },
        Variant {
            name: "auto/min-content..max-content",
            leaf_width: auto,
            leaf_min_max_width: min_max_content,
            container_min_max: none,
        },
        Variant {
            name: "auto/fit-content",
            leaf_width: auto,
            leaf_min_max_width: (auto, Dimension::fit_content()),
            container_min_max: none,
        },
        Variant {
            name: "auto/stretch",
            leaf_width: auto,
            leaf_min_max_width: (auto, Dimension::stretch()),
            container_min_max: (auto, Dimension::stretch()),
        },
        // Leaves with a definite width, whose content has to be measured to resolve a content-based keyword
        Variant { name: "definite/none", leaf_width: length(50.0), leaf_min_max_width: none, container_min_max: none },
        Variant {
            name: "definite/length",
            leaf_width: length(50.0),
            leaf_min_max_width: (length(20.0), length(80.0)),
            container_min_max: none,
        },
        Variant {
            name: "definite/min-content..max-content",
            leaf_width: length(50.0),
            leaf_min_max_width: min_max_content,
            container_min_max: none,
        },
    ]
}

const CONTAINERS: [(&str, &[Display]); 4] = [
    ("flex", &[Display::Flex]),
    ("block", &[Display::Block]),
    ("grid", &[Display::Grid]),
    ("mixed", &[Display::Flex, Display::Block, Display::Grid]),
];

fn build_subtree(taffy: &mut TaffyTree<()>, variant: Variant, displays: &[Display], depth: usize) -> NodeId {
    const FANOUT: usize = 8;
    const MAX_DEPTH: usize = 3;
    if depth == MAX_DEPTH {
        let (min_width, max_width) = variant.leaf_min_max_width;
        return taffy
            .new_leaf_with_context(
                Style {
                    size: Size { width: variant.leaf_width, height: auto() },
                    min_size: Size { width: min_width, height: auto() },
                    max_size: Size { width: max_width, height: auto() },
                    ..Default::default()
                },
                (),
            )
            .unwrap();
    }
    let children: Vec<_> = (0..FANOUT).map(|_| build_subtree(taffy, variant, displays, depth + 1)).collect();
    let (min, max) = variant.container_min_max;
    let style = Style {
        display: displays[depth % displays.len()],
        flex_direction: [FlexDirection::Column, FlexDirection::Row][depth % 2],
        flex_wrap: FlexWrap::Wrap,
        grid_template_columns: vec![auto(), auto()],
        padding: Rect::length(1.0),
        min_size: Size { width: min, height: min },
        max_size: Size { width: max, height: max },
        ..Default::default()
    };
    taffy.new_with_children(style, &children).unwrap()
}

fn build_tree(variant: Variant, displays: &[Display]) -> (TaffyTree<()>, NodeId) {
    let mut taffy = TaffyTree::new();
    let tree = build_subtree(&mut taffy, variant, displays, 0);
    let root = taffy
        .new_with_children(
            Style {
                display: Display::Block,
                size: Size { width: length(2000.0), height: length(2000.0) },
                ..Default::default()
            },
            &[tree],
        )
        .unwrap();
    (taffy, root)
}

fn layout(tree: &mut TaffyLayoutTree<()>) {
    tree.tree
        .compute_layout_with_measure(tree.root, Size::MAX_CONTENT, |inputs, _node, _context, style| {
            taffy::compute_leaf_layout(inputs, style, |_, _| 0.0, measure_text)
        })
        .unwrap()
}

fn sizing_keyword_benchmarks(c: &mut Criterion) {
    for (containers_name, displays) in CONTAINERS {
        let mut group = benchmark_group(c, &format!("min_max_keywords/{containers_name}"));
        for variant in variants() {
            let mut tree = TaffyLayoutTree::new(build_tree(variant, displays));
            let node_count = tree.tree.total_node_count();
            MEASURE_COUNT.store(0, Ordering::Relaxed);
            layout(&mut tree);
            let measure_count = MEASURE_COUNT.load(Ordering::Relaxed);
            println!("min_max_keywords/{containers_name}/{}: {measure_count} leaf measurements", variant.name);

            let mut tree = Some(tree);
            group.bench_function(BenchmarkId::new(variant.name, node_count), |b| {
                bench_layout(b, &mut tree, || unreachable!(), |tree| tree.mark_all_dirty(), layout)
            });
        }
        group.finish();
    }
}

criterion_group!(benches, sizing_keyword_benchmarks);
criterion_main!(benches);
