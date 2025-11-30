use cosmic_text::{Attrs, Buffer, FontSystem, Metrics, Shaping};
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::iter;
use taffy::prelude::*;
use taffy::style::Style;

pub const LOREM_IPSUM : &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";

struct CosmicTextContext {
    buffer: cosmic_text::Buffer,
}

impl CosmicTextContext {
    fn new(metrics: Metrics, text: &str, attrs: Attrs, font_system: &mut FontSystem) -> Self {
        let mut buffer = Buffer::new_empty(metrics);
        buffer.set_size(font_system, None, None);
        buffer.set_text(font_system, text, attrs, Shaping::Advanced);
        buffer.shape_until_scroll(font_system, false);
        Self { buffer }
    }

    fn measure(
        &mut self,
        known_dimensions: taffy::Size<Option<f32>>,
        available_space: taffy::Size<taffy::AvailableSpace>,
        font_system: &mut FontSystem,
    ) -> taffy::Size<f32> {
        let width_constraint = known_dimensions.width.or(match available_space.width {
            AvailableSpace::MinContent => Some(0.0),
            AvailableSpace::MaxContent => None,
            AvailableSpace::Definite(width) => Some(width),
        });
        self.buffer.set_size(font_system, width_constraint, None);

        let (width, total_lines) = self
            .buffer
            .layout_runs()
            .fold((0.0, 0usize), |(width, total_lines), run| (run.line_w.max(width), total_lines + 1));
        let height = total_lines as f32 * self.buffer.metrics().line_height;

        taffy::Size { width, height }
    }
}

fn measure_function(
    known_dimensions: taffy::Size<Option<f32>>,
    available_space: taffy::Size<taffy::AvailableSpace>,
    node_context: Option<&mut CosmicTextContext>,
    font_system: &mut FontSystem,
) -> Size<f32> {
    if let Size { width: Some(width), height: Some(height) } = known_dimensions {
        return Size { width, height };
    }

    match node_context {
        None => Size::ZERO,
        Some(text_context) => text_context.measure(known_dimensions, available_space, font_system),
    }
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

fn random_flex_style<R: Rng>(rng: &mut R) -> Style {
    Style {
        display: Display::Flex,
        flex_direction: if rng.random_bool(0.5) { FlexDirection::Row } else { FlexDirection::Column },
        flex_wrap: if rng.random_bool(0.5) { FlexWrap::Wrap } else { FlexWrap::NoWrap },
        ..Default::default()
    }
}

fn build_mixed_tree(
    taffy: &mut TaffyTree<CosmicTextContext>,
    font_system: &mut FontSystem,
    depth: usize,
    width: usize,
    rng: &mut ChaCha8Rng,
    is_grid: bool,
) -> NodeId {
    if depth == 0 {
        let metrics = Metrics { font_size: 14.0, line_height: 16.0 };
        let context = CosmicTextContext::new(metrics, LOREM_IPSUM, Attrs::new(), font_system);
        return taffy.new_leaf_with_context(Style::default(), context).unwrap();
    }

    let children: Vec<NodeId> =
        (0..width).map(|_| build_mixed_tree(taffy, font_system, depth - 1, width, rng, !is_grid)).collect();

    let style = if is_grid {
        random_nxn_grid_style(rng, (width as f32).sqrt().ceil() as usize)
    } else {
        random_flex_style(rng)
    };

    taffy.new_with_children(style, &children).unwrap()
}

fn mixed_benchmark(c: &mut Criterion) {
    let font_system = std::cell::RefCell::new(FontSystem::new());
    let mut group = c.benchmark_group("mixed_flex_grid");

    let depths = [2, 4];
    let widths = [4, 8];

    for &depth in &depths {
        for &width in &widths {
            let benchmark_id = BenchmarkId::new("mixed", format!("depth_{}_width_{}", depth, width));
            group.bench_with_input(benchmark_id, &(depth, width), |b, &(depth, width)| {
                b.iter_batched(
                    || {
                        let mut taffy = TaffyTree::new();
                        let mut rng = ChaCha8Rng::seed_from_u64(12345);
                        let root =
                            build_mixed_tree(&mut taffy, &mut font_system.borrow_mut(), depth, width, &mut rng, true);
                        (taffy, root)
                    },
                    |(mut taffy, root)| {
                        taffy
                            .compute_layout_with_measure(
                                root,
                                Size::MAX_CONTENT,
                                |known_dimensions, available_space, _node_id, node_context, _style| {
                                    measure_function(
                                        known_dimensions,
                                        available_space,
                                        node_context,
                                        &mut font_system.borrow_mut(),
                                    )
                                },
                            )
                            .unwrap();
                    },
                    criterion::BatchSize::SmallInput,
                )
            });
        }
    }
    group.finish();
}

criterion_group!(benches, mixed_benchmark);
criterion_main!(benches);
