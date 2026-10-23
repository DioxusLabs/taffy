//! This file includes benchmarks for very large, pseudo-randomly generated trees
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::iter;
use taffy::prelude::*;
use taffy::style::Style;
use taffy_benchmarks::{bench_layout, benchmark_group, compute_layout, TaffyLayoutTree};

/// Build a random leaf node
fn build_random_leaf(taffy: &mut TaffyTree, _rng: &mut ChaCha8Rng) -> NodeId {
    taffy.new_with_children(Style { size: length(20.0), ..Default::default() }, &[]).unwrap()
}

fn random_grid_track<R: Rng>(rng: &mut R) -> GridTemplateComponent<String> {
    let switch: f32 = rng.random_range(0.0..=1.0);
    if switch < 0.1 {
        auto()
    } else if switch < 0.2 {
        min_content()
    } else if switch < 0.3 {
        max_content()
    } else if switch < 0.5 {
        fr(1.0)
    } else if switch < 0.6 {
        minmax(length(0.0), fr(1.0))
    } else if switch < 0.8 {
        length(40.0)
    } else {
        percent(0.3)
    }
}

fn random_nxn_grid_style<R: Rng>(rng: &mut R, track_count: usize) -> Style {
    Style {
        display: Display::Grid,
        grid_template_columns: iter::from_fn(|| Some(random_grid_track(rng))).take(track_count).collect(),
        grid_template_rows: iter::from_fn(|| Some(random_grid_track(rng))).take(track_count).collect(),
        ..Default::default()
    }
}

/// A tree with many children that have shallow depth
fn build_grid_flat_hierarchy(col_count: usize, row_count: usize) -> (TaffyTree, NodeId) {
    let mut taffy = TaffyTree::new();
    let mut rng = ChaCha8Rng::seed_from_u64(12345);

    let style = Style {
        display: Display::Grid,
        grid_template_columns: iter::from_fn(|| Some(random_grid_track(&mut rng))).take(col_count).collect(),
        grid_template_rows: iter::from_fn(|| Some(random_grid_track(&mut rng))).take(row_count).collect(),
        ..Default::default()
    };

    let children: Vec<_> =
        iter::from_fn(|| Some(build_random_leaf(&mut taffy, &mut rng))).take(col_count * row_count).collect();

    let root = taffy.new_with_children(style, children.as_slice()).unwrap();
    (taffy, root)
}

#[cfg(feature = "grid-alternating-spans")]
fn build_grid_alternating_spans(item_count: usize, flow: GridAutoFlow) -> (TaffyTree, NodeId) {
    let mut taffy = TaffyTree::new();
    let is_row = matches!(flow, GridAutoFlow::Row | GridAutoFlow::RowDense);
    let children: Vec<_> = (0..item_count)
        .map(|i| {
            let locked_axis = if i % 2 == 0 { Line { start: line(1), end: span(2) } } else { line(2) };
            let mut style = Style::default();
            if is_row {
                style.grid_row = locked_axis;
            } else {
                style.grid_column = locked_axis;
            }
            taffy.new_leaf(style).unwrap()
        })
        .collect();
    let mut style = Style {
        display: Display::Grid,
        grid_auto_flow: flow,
        grid_auto_columns: vec![length(1.0)],
        grid_auto_rows: vec![length(1.0)],
        ..Default::default()
    };
    if is_row {
        style.grid_template_columns = vec![length(1.0); item_count];
    } else {
        style.grid_template_rows = vec![length(1.0); item_count];
    }
    let root = taffy.new_with_children(style, &children).unwrap();
    (taffy, root)
}

/// A helper function to recursively construct a deep tree
pub fn build_deep_grid_tree(
    tree: &mut TaffyTree,
    levels: usize,
    track_count: usize,
    create_leaf_node: &mut impl FnMut(&mut TaffyTree) -> NodeId,
    create_container_node: &mut impl FnMut(&mut TaffyTree, Vec<NodeId>) -> NodeId,
) -> Vec<NodeId> {
    // The extra one is for a position:absolute child
    let child_count = track_count * track_count;

    if levels == 1 {
        // Build leaf nodes
        return (0..child_count).map(|_| create_leaf_node(tree)).collect();
    }

    // Add another layer to the tree
    // Each child gets an equal amount of the remaining nodes
    (0..child_count)
        .map(|_| {
            let sub_children =
                build_deep_grid_tree(tree, levels - 1, track_count, create_leaf_node, create_container_node);
            create_container_node(tree, sub_children)
        })
        .collect()
}

/// A tree with a higher depth for a more realistic scenario
fn build_taffy_deep_grid_hierarchy(levels: usize, track_count: usize) -> (TaffyTree, NodeId) {
    let mut rng = ChaCha8Rng::seed_from_u64(12345);
    let mut build_leaf_node = |taffy: &mut TaffyTree| build_random_leaf(taffy, &mut rng);
    let mut rng = ChaCha8Rng::seed_from_u64(12345);
    let mut build_flex_node = |taffy: &mut TaffyTree, children: Vec<NodeId>| {
        taffy.new_with_children(random_nxn_grid_style(&mut rng, track_count), &children).unwrap()
    };

    let mut taffy = TaffyTree::new();
    let tree = build_deep_grid_tree(&mut taffy, levels, track_count, &mut build_leaf_node, &mut build_flex_node);
    let root = taffy.new_with_children(Style::DEFAULT, &tree).unwrap();
    (taffy, root)
}

fn taffy_benchmarks(c: &mut Criterion) {
    #[cfg(feature = "grid-alternating-spans")]
    {
        let mut group = benchmark_group(c, "grid/alternating_spans");
        for flow in [GridAutoFlow::Row, GridAutoFlow::RowDense, GridAutoFlow::Column, GridAutoFlow::ColumnDense] {
            for item_count in [100, 1000] {
                let mut tree = None;
                group.bench_with_input(
                    BenchmarkId::new(format!("{flow:?}"), item_count),
                    &item_count,
                    |b, &item_count| {
                        bench_layout(
                            b,
                            &mut tree,
                            || TaffyLayoutTree::new(build_grid_alternating_spans(item_count, flow)),
                            |tree| tree.mark_all_dirty(),
                            |tree| tree.tree.compute_layout(tree.root, Size::MAX_CONTENT).unwrap(),
                        )
                    },
                );
            }
        }
        group.finish();
    }

    let mut group = benchmark_group(c, "grid/wide");
    for track_count in [31usize, 100, 316].iter() {
        let mut tree = None;
        group.bench_with_input(
            BenchmarkId::new(format!("{c}x{c}", c = track_count), track_count.pow(2)),
            track_count,
            |b, &track_count| {
                bench_layout(
                    b,
                    &mut tree,
                    || TaffyLayoutTree::new(build_grid_flat_hierarchy(track_count, track_count)),
                    |tree| tree.mark_all_dirty(),
                    |tree| compute_layout(&mut tree.tree, tree.root, length(12000.0)),
                )
            },
        );
    }
    group.finish();

    let mut group = benchmark_group(c, "grid/deep");
    for (tracks, levels) in [(2, 5), (3, 4), (2, 7) /*, (3, 5)*/].iter() {
        let children_per_level: usize = tracks * tracks;
        let mut tree = None;
        group.bench_with_input(
            BenchmarkId::new(format!("{c}x{c}", c = tracks), children_per_level.pow(*levels as u32)),
            &(*levels, *tracks),
            |b, &(levels, tracks)| {
                bench_layout(
                    b,
                    &mut tree,
                    || TaffyLayoutTree::new(build_taffy_deep_grid_hierarchy(levels, tracks)),
                    |tree| tree.mark_all_dirty(),
                    |tree| compute_layout(&mut tree.tree, tree.root, length(12000.0)),
                )
            },
        );
    }
    group.finish();

    let mut group = benchmark_group(c, "grid/superdeep");
    for levels in [100, 1000].iter() {
        let mut tree = None;
        group.bench_with_input(BenchmarkId::new("1x1", levels), levels, |b, &levels| {
            bench_layout(
                b,
                &mut tree,
                || TaffyLayoutTree::new(build_taffy_deep_grid_hierarchy(levels, 1)),
                |tree| tree.mark_all_dirty(),
                |tree| compute_layout(&mut tree.tree, tree.root, max_content()),
            )
        });
    }
    group.finish();
}

criterion_group!(name = benches; config = taffy_benchmarks::criterion_config(); targets = taffy_benchmarks);
criterion_main!(benches);
