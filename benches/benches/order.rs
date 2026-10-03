//! Benchmarks for the `order` property: flat flex and grid containers whose children either all
//! have the default `order: 0` (the fast path, which must not sort) or a mix of `order` values.
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use taffy::prelude::*;

/// Cycle through a few `order` values so that the items need sorting (and ties are common)
fn mixed_order(index: usize) -> i32 {
    match index % 5 {
        0 => 2,
        1 => -1,
        2 => 0,
        3 => 3,
        _ => -2,
    }
}

fn child_style(index: usize, with_order: bool) -> Style {
    Style {
        size: Size { width: length(10.0), height: length(10.0) },
        order: if with_order { mixed_order(index) } else { 0 },
        ..Default::default()
    }
}

fn build_flex(child_count: usize, with_order: bool) -> (TaffyTree, NodeId) {
    let mut taffy = TaffyTree::new();
    let children: Vec<NodeId> =
        (0..child_count).map(|index| taffy.new_leaf(child_style(index, with_order)).unwrap()).collect();
    let root = taffy
        .new_with_children(
            Style { display: Display::Flex, flex_wrap: FlexWrap::Wrap, size: length(500.0), ..Default::default() },
            &children,
        )
        .unwrap();
    (taffy, root)
}

fn build_grid(track_count: usize, with_order: bool) -> (TaffyTree, NodeId) {
    let mut taffy = TaffyTree::new();
    let children: Vec<NodeId> = (0..track_count * track_count)
        .map(|index| taffy.new_leaf(child_style(index, with_order)).unwrap())
        .collect();
    let root = taffy
        .new_with_children(
            Style {
                display: Display::Grid,
                grid_template_columns: vec![repeat(track_count as u16, vec![length(10.0)])],
                grid_template_rows: vec![repeat(track_count as u16, vec![length(10.0)])],
                ..Default::default()
            },
            &children,
        )
        .unwrap();
    (taffy, root)
}

fn taffy_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("flex/order");
    for child_count in [100usize, 1000, 10_000].iter() {
        for (label, with_order) in [("none", false), ("mixed", true)] {
            group.bench_with_input(BenchmarkId::new(label, child_count), child_count, |b, &child_count| {
                b.iter_batched(
                    || build_flex(child_count, with_order),
                    |(mut taffy, root)| taffy.compute_layout(root, max_content()).unwrap(),
                    criterion::BatchSize::SmallInput,
                )
            });
        }
    }
    group.finish();

    let mut group = c.benchmark_group("grid/order");
    for track_count in [10usize, 31, 100].iter() {
        for (label, with_order) in [("none", false), ("mixed", true)] {
            group.bench_with_input(
                BenchmarkId::new(format!("{label} {c}x{c}", c = track_count), track_count.pow(2)),
                track_count,
                |b, &track_count| {
                    b.iter_batched(
                        || build_grid(track_count, with_order),
                        |(mut taffy, root)| taffy.compute_layout(root, max_content()).unwrap(),
                        criterion::BatchSize::SmallInput,
                    )
                },
            );
        }
    }
    group.finish();
}

criterion_group!(benches, taffy_benchmarks);
criterion_main!(benches);
