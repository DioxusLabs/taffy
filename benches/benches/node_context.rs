//! Benchmarks for node context access, which happens once per measured leaf per layout and whenever a
//! context is replaced. A trivial measure function keeps the cost of the lookup itself visible.
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use std::hint::black_box;
use std::time::Instant;
use taffy::prelude::*;
use taffy::LayoutOutput;
use taffy_benchmarks::{benchmark_group, iter_timed};

/// A flex row of rows, each holding measured leaves whose context is their width
fn build_tree(leaf_count: usize) -> (TaffyTree<f32>, NodeId, Vec<NodeId>) {
    let mut taffy = TaffyTree::new();
    let leaves: Vec<NodeId> =
        (0..leaf_count).map(|index| taffy.new_leaf_with_context(Style::default(), index as f32).unwrap()).collect();
    let rows: Vec<NodeId> = leaves
        .chunks(16)
        .map(|chunk| {
            let style = Style { flex_wrap: FlexWrap::Wrap, ..Default::default() };
            taffy.new_with_children(style, chunk).unwrap()
        })
        .collect();
    let root_style = Style { flex_direction: FlexDirection::Column, ..Default::default() };
    let root = taffy.new_with_children(root_style, &rows).unwrap();
    (taffy, root, leaves)
}

fn layout(taffy: &mut TaffyTree<f32>, root: NodeId) {
    taffy
        .compute_layout_with_measure(root, Size::MAX_CONTENT, |input, _, context, _| {
            LayoutOutput::from_outer_size(Size {
                width: input.known_dimensions.width.unwrap_or_else(|| context.copied().unwrap_or(0.0) % 100.0),
                height: input.known_dimensions.height.unwrap_or(10.0),
            })
        })
        .unwrap();
}

fn node_context_benchmarks(c: &mut Criterion) {
    let mut group = benchmark_group(c, "node_context");
    for leaf_count in [1_000, 10_000, 100_000] {
        group.bench_with_input(BenchmarkId::new("create", leaf_count), &leaf_count, |b, &leaf_count| {
            b.iter(|| black_box(build_tree(leaf_count)))
        });

        let (mut taffy, root, leaves) = build_tree(leaf_count);
        group.bench_with_input(BenchmarkId::new("layout", leaf_count), &leaf_count, |b, _| {
            iter_timed(b, || {
                for &leaf in &leaves {
                    taffy.mark_dirty(leaf).unwrap();
                }
                let start = Instant::now();
                layout(&mut taffy, root);
                start.elapsed()
            })
        });

        group.bench_with_input(BenchmarkId::new("replace_and_layout", leaf_count), &leaf_count, |b, _| {
            iter_timed(b, || {
                let start = Instant::now();
                for (index, &leaf) in leaves.iter().enumerate() {
                    taffy.set_node_context(leaf, Some(index as f32 + 1.0)).unwrap();
                }
                layout(&mut taffy, root);
                start.elapsed()
            })
        });
    }
    group.finish();
}

criterion_group!(benches, node_context_benchmarks);
criterion_main!(benches);
