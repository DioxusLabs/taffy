//! Benchmarks for Display::Contents overhead
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use taffy::prelude::*;
use taffy::style::Style;

/// Build a flat flex tree with N children, no contents nodes.
fn build_flat_tree(n: usize) -> (TaffyTree<()>, NodeId) {
    let mut taffy = TaffyTree::new();
    let mut children = Vec::with_capacity(n);
    for _ in 0..n {
        let child = taffy
            .new_leaf(Style {
                size: Size { width: Dimension::from_length(30.0), height: auto() },
                ..Default::default()
            })
            .unwrap();
        children.push(child);
    }
    let root = taffy
        .new_with_children(
            Style {
                display: Display::Flex,
                size: Size {
                    width: Dimension::from_length(1000.0),
                    height: Dimension::from_length(500.0),
                },
                ..Default::default()
            },
            &children,
        )
        .unwrap();
    (taffy, root)
}

/// Build a flex tree with N children, where every 5th child is wrapped in a
/// Display::Contents node (simulating GestureDetector-like wrappers).
fn build_tree_with_contents(n: usize) -> (TaffyTree<()>, NodeId) {
    let mut taffy = TaffyTree::new();
    let mut children = Vec::with_capacity(n);
    for i in 0..n {
        let leaf = taffy
            .new_leaf(Style {
                size: Size { width: Dimension::from_length(30.0), height: auto() },
                ..Default::default()
            })
            .unwrap();
        if i % 5 == 0 {
            // Wrap in contents node
            let contents = taffy
                .new_with_children(
                    Style { display: Display::Contents, ..Default::default() },
                    &[leaf],
                )
                .unwrap();
            children.push(contents);
        } else {
            children.push(leaf);
        }
    }
    let root = taffy
        .new_with_children(
            Style {
                display: Display::Flex,
                size: Size {
                    width: Dimension::from_length(1000.0),
                    height: Dimension::from_length(500.0),
                },
                ..Default::default()
            },
            &children,
        )
        .unwrap();
    (taffy, root)
}

/// Build a deep tree (10 levels) with N leaves at the bottom, no contents.
fn build_deep_tree(depth: usize, breadth: usize) -> (TaffyTree<()>, NodeId) {
    let mut taffy = TaffyTree::new();
    let root = build_deep_node(&mut taffy, depth, breadth);
    (taffy, root)
}

fn build_deep_node(taffy: &mut TaffyTree<()>, depth: usize, breadth: usize) -> NodeId {
    if depth == 0 {
        return taffy
            .new_leaf(Style {
                size: Size { width: Dimension::from_length(10.0), height: Dimension::from_length(10.0) },
                ..Default::default()
            })
            .unwrap();
    }
    let mut children = Vec::with_capacity(breadth);
    for _ in 0..breadth {
        children.push(build_deep_node(taffy, depth - 1, breadth));
    }
    taffy
        .new_with_children(
            Style { display: Display::Flex, ..Default::default() },
            &children,
        )
        .unwrap()
}

fn bench_no_contents(c: &mut Criterion) {
    let mut group = c.benchmark_group("display_contents/no_contents");
    for &n in &[10, 100, 1000] {
        group.bench_with_input(BenchmarkId::new("flat", n), &n, |b, &n| {
            b.iter_batched(
                || build_flat_tree(n),
                |(mut taffy, root)| {
                    taffy.compute_layout(root, Size::MAX_CONTENT).unwrap();
                },
                criterion::BatchSize::SmallInput,
            );
        });
    }
    // Deep tree: 4^6 = 4096 nodes
    group.bench_function("deep_4x6", |b| {
        b.iter_batched(
            || build_deep_tree(6, 4),
            |(mut taffy, root)| {
                taffy.compute_layout(root, Size::MAX_CONTENT).unwrap();
            },
            criterion::BatchSize::SmallInput,
        );
    });
    group.finish();
}

fn bench_with_contents(c: &mut Criterion) {
    let mut group = c.benchmark_group("display_contents/with_contents");
    for &n in &[10, 100, 1000] {
        group.bench_with_input(BenchmarkId::new("flat_20pct_contents", n), &n, |b, &n| {
            b.iter_batched(
                || build_tree_with_contents(n),
                |(mut taffy, root)| {
                    taffy.compute_layout(root, Size::MAX_CONTENT).unwrap();
                },
                criterion::BatchSize::SmallInput,
            );
        });
    }
    group.finish();
}

fn bench_relayout(c: &mut Criterion) {
    let mut group = c.benchmark_group("display_contents/relayout");
    // Measure cost of repeated layouts (resolved_children recomputed each time)
    for &n in &[100, 1000] {
        group.bench_with_input(BenchmarkId::new("no_contents", n), &n, |b, &n| {
            let (mut taffy, root) = build_flat_tree(n);
            taffy.compute_layout(root, Size::MAX_CONTENT).unwrap();
            b.iter(|| {
                taffy.mark_dirty(root).unwrap();
                taffy.compute_layout(root, Size::MAX_CONTENT).unwrap();
            });
        });
        group.bench_with_input(BenchmarkId::new("with_contents", n), &n, |b, &n| {
            let (mut taffy, root) = build_tree_with_contents(n);
            taffy.compute_layout(root, Size::MAX_CONTENT).unwrap();
            b.iter(|| {
                taffy.mark_dirty(root).unwrap();
                taffy.compute_layout(root, Size::MAX_CONTENT).unwrap();
            });
        });
    }
    group.finish();
}

criterion_group!(benches, bench_no_contents, bench_with_contents, bench_relayout);
criterion_main!(benches);
