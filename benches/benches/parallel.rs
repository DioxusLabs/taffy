//! Benchmarks comparing sequential layout of a `TaffyTree` with parallel layout on thread pools of various sizes
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use parley::{style::StyleProperty, FontContext, LayoutContext};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use taffy::prelude::*;
use taffy::{LayoutInput, LayoutOutput};
use taffy_benchmarks::{bench_layout, benchmark_group, TaffyLayoutTree};

const LOREM_IPSUM : &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";

const THREAD_COUNTS: [usize; 4] = [1, 2, 4, 8];

/// The context of a leaf node: either nothing (the leaf is sized by its style) or text that is measured using Parley
enum LeafContext {
    None,
    Text(Box<parley::Layout<[u8; 4]>>),
}

fn measure(inputs: LayoutInput, _node: NodeId, context: Option<&mut LeafContext>, style: &Style) -> LayoutOutput {
    taffy::compute_leaf_layout(
        inputs,
        style,
        |_, _| 0.0,
        |known_dimensions, available_space| {
            if let Size { width: Some(width), height: Some(height) } = known_dimensions {
                return Size { width, height };
            }
            let Some(LeafContext::Text(layout)) = context else { return Size::ZERO };
            let width: f32 = known_dimensions.width.unwrap_or_else(|| {
                let widths = layout.calculate_content_widths();
                match available_space.width {
                    AvailableSpace::MinContent => widths.min,
                    AvailableSpace::MaxContent => widths.max,
                    AvailableSpace::Definite(limit) => limit.min(widths.max).max(widths.min),
                }
                .ceil()
            });
            layout.break_all_lines(Some(width));
            Size { width, height: layout.height() }
        },
    )
}

type Tree = TaffyTree<LeafContext>;

fn fixed_leaf(taffy: &mut Tree, rng: &mut ChaCha8Rng) -> NodeId {
    let size = Size { width: length(rng.random_range(10.0..100.0)), height: length(rng.random_range(10.0..100.0)) };
    taffy.new_leaf_with_context(Style { size, ..Default::default() }, LeafContext::None).unwrap()
}

/// A tree of nodes with the same `display` and `branching` children each, with auto-sized containers
fn build_uniform_tree(
    taffy: &mut Tree,
    rng: &mut ChaCha8Rng,
    depth: usize,
    branching: usize,
    container: &dyn Fn(&mut ChaCha8Rng) -> Style,
) -> NodeId {
    if depth == 0 {
        return fixed_leaf(taffy, rng);
    }
    let children: Vec<NodeId> =
        (0..branching).map(|_| build_uniform_tree(taffy, rng, depth - 1, branching, container)).collect();
    taffy.new_with_children(container(rng), &children).unwrap()
}

fn flex_container(rng: &mut ChaCha8Rng) -> Style {
    Style {
        display: Display::Flex,
        flex_direction: if rng.random_bool(0.5) { FlexDirection::Row } else { FlexDirection::Column },
        flex_wrap: if rng.random_bool(0.5) { FlexWrap::Wrap } else { FlexWrap::NoWrap },
        ..Default::default()
    }
}

fn block_container(rng: &mut ChaCha8Rng) -> Style {
    let padding = length(rng.random_range(0.0..4.0));
    Style { display: Display::Block, padding: Rect { left: padding, right: padding, top: padding, bottom: padding }, ..Default::default() }
}

fn grid_track(rng: &mut ChaCha8Rng) -> GridTemplateComponent<String> {
    match rng.random_range(0.0..1.0) {
        x if x < 0.2 => auto(),
        x if x < 0.3 => min_content(),
        x if x < 0.4 => max_content(),
        x if x < 0.7 => fr(1.0),
        x if x < 0.8 => minmax(length(0.0), fr(1.0)),
        _ => length(40.0),
    }
}

fn grid_container(track_count: usize) -> impl Fn(&mut ChaCha8Rng) -> Style {
    move |rng| Style {
        display: Display::Grid,
        grid_template_columns: (0..track_count).map(|_| grid_track(rng)).collect(),
        grid_template_rows: (0..track_count).map(|_| grid_track(rng)).collect(),
        ..Default::default()
    }
}

/// A tree that alternates between grid and flexbox containers, with leaves that contain a paragraph of text
fn build_text_tree(
    taffy: &mut Tree,
    rng: &mut ChaCha8Rng,
    layout_ctx: &mut LayoutContext<[u8; 4]>,
    font_ctx: &mut FontContext,
    depth: usize,
    branching: usize,
    is_grid: bool,
) -> NodeId {
    if depth == 0 {
        let mut builder = layout_ctx.ranged_builder(font_ctx, LOREM_IPSUM, 1.0, false);
        builder.push_default(StyleProperty::FontSize(14.0));
        let context = LeafContext::Text(Box::new(builder.build(LOREM_IPSUM)));
        return taffy.new_leaf_with_context(Style::default(), context).unwrap();
    }
    let children: Vec<NodeId> = (0..branching)
        .map(|_| build_text_tree(taffy, rng, layout_ctx, font_ctx, depth - 1, branching, !is_grid))
        .collect();
    let style =
        if is_grid { grid_container((branching as f32).sqrt().ceil() as usize)(rng) } else { flex_container(rng) };
    taffy.new_with_children(style, &children).unwrap()
}

fn bench_tree(c: &mut Criterion, name: &str, available_space: Size<AvailableSpace>, build: impl Fn() -> (Tree, NodeId)) {
    let mut group = benchmark_group(c, &format!("parallel/{name}"));
    let mut tree = None;
    let node_count = build().0.total_node_count();

    group.bench_function(BenchmarkId::new("sequential", node_count), |b| {
        bench_layout(
            b,
            &mut tree,
            || TaffyLayoutTree::new(build()),
            |tree| tree.mark_all_dirty(),
            |tree| tree.tree.compute_layout_with_measure(tree.root, available_space, measure).unwrap(),
        )
    });
    for thread_count in THREAD_COUNTS {
        let pool = rayon::ThreadPoolBuilder::new().num_threads(thread_count).build().unwrap();
        group.bench_function(BenchmarkId::new(format!("parallel_{thread_count}_threads"), node_count), |b| {
            bench_layout(
                b,
                &mut tree,
                || TaffyLayoutTree::new(build()),
                |tree| tree.mark_all_dirty(),
                |tree| {
                    pool.install(|| {
                        tree.tree.compute_layout_with_measure_parallel(tree.root, available_space, measure).unwrap()
                    })
                },
            )
        });
    }
    group.finish();
}

fn parallel_benchmarks(c: &mut Criterion) {
    let definite = Size { width: AvailableSpace::Definite(1920.0), height: AvailableSpace::MaxContent };

    for (depth, branching) in [(3, 10), (5, 10)] {
        bench_tree(c, &format!("flex_depth_{depth}_width_{branching}"), definite, || {
            let mut taffy = Tree::new();
            let mut rng = ChaCha8Rng::seed_from_u64(12345);
            let root = build_uniform_tree(&mut taffy, &mut rng, depth, branching, &flex_container);
            (taffy, root)
        });
    }
    for (depth, branching) in [(3, 10), (5, 10)] {
        bench_tree(c, &format!("block_depth_{depth}_width_{branching}"), definite, || {
            let mut taffy = Tree::new();
            let mut rng = ChaCha8Rng::seed_from_u64(12345);
            let root = build_uniform_tree(&mut taffy, &mut rng, depth, branching, &block_container);
            (taffy, root)
        });
    }
    for (depth, tracks) in [(3, 3), (5, 3)] {
        bench_tree(c, &format!("grid_depth_{depth}_{tracks}x{tracks}"), definite, || {
            let mut taffy = Tree::new();
            let mut rng = ChaCha8Rng::seed_from_u64(12345);
            let root = build_uniform_tree(&mut taffy, &mut rng, depth, tracks * tracks, &grid_container(tracks));
            (taffy, root)
        });
    }
    for (depth, branching) in [(2, 8), (4, 8)] {
        bench_tree(c, &format!("text_depth_{depth}_width_{branching}"), Size::MAX_CONTENT, || {
            let mut taffy = Tree::new();
            let mut rng = ChaCha8Rng::seed_from_u64(12345);
            let mut layout_ctx = LayoutContext::new();
            let mut font_ctx = FontContext::new();
            let root = build_text_tree(&mut taffy, &mut rng, &mut layout_ctx, &mut font_ctx, depth, branching, true);
            (taffy, root)
        });
    }
}

criterion_group!(name = benches; config = taffy_benchmarks::criterion_config(); targets = parallel_benchmarks);
criterion_main!(benches);
