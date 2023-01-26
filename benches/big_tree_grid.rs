//! This file includes benchmarks for very large, pseudo-randomly generated trees
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use taffy::prelude::*;
use taffy::style::Style;
use std::iter;

/// Build a random leaf node
fn build_random_leaf(taffy: &mut Taffy, _rng: &mut ChaCha8Rng) -> Node {
    taffy.new_with_children(Style { size: points(20.0), ..Default::default() }, &[]).unwrap()
}

fn random_grid_track<R: Rng>(rng: &mut R) -> TrackSizingFunction {
    let switch: f32 = rng.gen_range(0.0..=1.0);
    if switch < 0.1 {
        auto()
    } else if switch < 0.2 {
        min_content()
    } else if switch < 0.3 {
        max_content()
    } else if switch < 0.5 {
        flex(1.0)
    } else if switch < 0.6 {
        minmax(points(0.0), flex(1.0))
    } else if switch < 0.8 {
        points(40.0)
    } else {
        percent(0.3)
    }
}

fn random_3x3_grid_style<R: Rng>(rng: &mut R) -> Style {
    Style {
        display: Display::Grid,
        grid_template_columns: vec![random_grid_track(rng), random_grid_track(rng), random_grid_track(rng)],
        grid_template_rows: vec![random_grid_track(rng), random_grid_track(rng), random_grid_track(rng)],
        ..Default::default()
    }
}

/// A tree with many children that have shallow depth
fn build_grid_flat_hierarchy(col_count: usize, row_count: usize) -> (Taffy, Node) {
    let mut taffy = Taffy::new();
    let mut rng = ChaCha8Rng::seed_from_u64(12345);

    let style = Style {
        grid_template_columns: iter::from_fn(|| Some(random_grid_track(&mut rng))).take(col_count).collect(),
        grid_template_rows: iter::from_fn(|| Some(random_grid_track(&mut rng))).take(row_count).collect(),
        ..Default::default()
    };

    let children: Vec<_> =
        iter::from_fn(|| Some(build_random_leaf(&mut taffy, &mut rng))).take(col_count * row_count as usize).collect();


    let root = taffy.new_with_children(style, children.as_slice()).unwrap();
    (taffy, root)
}

/// A helper function to recursively construct a deep tree
pub fn build_deep_grid_tree(
    tree: &mut Taffy,
    levels: u32,
    create_leaf_node: &mut impl FnMut(&mut Taffy) -> Node,
    create_container_node: &mut impl FnMut(&mut Taffy, Vec<Node>) -> Node,
) -> Vec<Node> {
    if levels == 1 {
        // Build leaf nodes
        return (0..10).map(|_| create_leaf_node(tree)).collect();
    }

    // Add another layer to the tree
    // Each child gets an equal amount of the remaining nodes
    (0..10)
        .map(|_| {
            let sub_children = build_deep_grid_tree(tree, levels - 1, create_leaf_node, create_container_node);
            tree.set_style(sub_children[9], {
                let mut style = tree.style(sub_children[9]).unwrap().clone();
                style.position = Position::Absolute;
                style.size = percent(0.5);
                style
            })
            .unwrap();
            create_container_node(tree, sub_children)
        })
        .collect()
}

/// A tree with a higher depth for a more realistic scenario
fn build_taffy_deep_grid_hierarchy(levels: u32) -> (Taffy, Node) {
    let mut rng = ChaCha8Rng::seed_from_u64(12345);
    let mut build_leaf_node = |taffy: &mut Taffy| build_random_leaf(taffy, &mut rng);
    let mut rng = ChaCha8Rng::seed_from_u64(12345);
    let mut build_flex_node = |taffy: &mut Taffy, children: Vec<Node>| {
        taffy.new_with_children(random_3x3_grid_style(&mut rng), &children).unwrap()
    };

    let mut taffy = Taffy::new();
    let tree = build_deep_grid_tree(&mut taffy, levels, &mut build_leaf_node, &mut build_flex_node);
    let root = taffy.new_with_children(Style::DEFAULT, &tree).unwrap();
    (taffy, root)
}

fn taffy_benchmarks(c: &mut Criterion) {

    let mut group = c.benchmark_group("Grid (wide)");
    for track_count in [31usize, 100, 150].iter() {
        group.bench_with_input(BenchmarkId::new("Taffy", track_count.pow(2)), track_count, |b, &track_count| {
            b.iter_batched(
                || build_grid_flat_hierarchy(track_count, track_count),
                |(mut taffy, root)| taffy.compute_layout(root, points(12000.0)).unwrap(),
                criterion::BatchSize::SmallInput,
            )
        });
    }
    group.finish();

    let mut group = c.benchmark_group("Grid (deep)/random3x3");
    for levels in [3, 4/*, 5*/].iter() {
        group.bench_with_input(BenchmarkId::new("Taffy", 10u32.pow(*levels)), levels, |b, &levels| {
            b.iter_batched(
                || build_taffy_deep_grid_hierarchy(levels),
                |(mut taffy, root)| taffy.compute_layout(root, points(12000.0)).unwrap(),
                criterion::BatchSize::SmallInput,
            )
        });
    }
    group.finish();
}

criterion_group!(benches, taffy_benchmarks);
criterion_main!(benches);
