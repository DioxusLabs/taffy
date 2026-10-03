// As each benchmark suite is compiled as a separate crate and uses different helpers, we end up with a bunch
// of false positives for this lint. So let's just disable it for this code.
#![allow(dead_code)]

pub mod taffy_helpers;
pub use taffy_helpers::TaffyTreeBuilder;

#[cfg(feature = "taffy03")]
pub mod taffy_03_helpers;

#[cfg(feature = "yoga")]
pub mod yoga_helpers;
#[cfg(feature = "yoga")]
pub use yoga_helpers::YogaTreeBuilder;

use criterion::measurement::WallTime;
use criterion::{Bencher, BenchmarkGroup, Criterion, SamplingMode};
use rand::distr::uniform::SampleRange;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use std::time::{Duration, Instant};
use taffy::style::Style as TaffyStyle;
use taffy::{Display, NodeId, TaffyTree};

pub const STANDARD_RNG_SEED: u64 = 12345;

/// The Criterion configuration shared by all of the benchmarks.
///
/// The benchmarks are deterministic and each iteration is timed individually (see [`iter_timed`]), so a few short
/// samples are enough. Criterion's defaults (100 samples over a 3s warm-up and 5s of measurement) make the
/// suite take several minutes. These can still be overridden from the command line
/// (e.g. `cargo bench -- --measurement-time 5`).
pub fn criterion_config() -> Criterion {
    Criterion::default()
        .warm_up_time(Duration::from_millis(300))
        .measurement_time(Duration::from_secs(1))
        .sample_size(10)
}

/// Create a benchmark group which runs the same number of iterations in every sample.
///
/// Criterion's default "linear" sampling exists to regress out per-sample overhead. We time every iteration
/// individually (see [`iter_timed`]) so there is no such overhead, and it would only make the samples unequal.
pub fn benchmark_group<'a>(c: &'a mut Criterion, name: &str) -> BenchmarkGroup<'a, WallTime> {
    let mut group = c.benchmark_group(name);
    group.sampling_mode(SamplingMode::Flat);
    group
}

/// Runs `timed_run` once per iteration and reports the mean of the times that it returns.
///
/// `timed_run` returns the time taken by the part of the run that is being benchmarked, which keeps setup and
/// teardown (building, invalidating and dropping trees) out of the measurement.
pub fn iter_timed(b: &mut Bencher, mut timed_run: impl FnMut() -> Duration) {
    b.iter_custom(|iters| (0..iters).map(|_| timed_run()).sum())
}

/// Benchmark laying out a tree from scratch (with no cached layout results).
///
/// The tree is only built once (by `build`, the first time that Criterion runs the benchmark) and is then reused
/// for every iteration, with `invalidate` being called before each iteration to throw away the results of the
/// previous one. Only `layout` is timed.
pub fn bench_layout<T>(
    b: &mut Bencher,
    tree: &mut Option<T>,
    build: impl FnOnce() -> T,
    mut invalidate: impl FnMut(&mut T),
    mut layout: impl FnMut(&mut T),
) {
    let tree = tree.get_or_insert_with(build);
    iter_timed(b, || {
        invalidate(tree);
        let start = Instant::now();
        layout(tree);
        start.elapsed()
    })
}

/// A [`TaffyTree`] along with what is needed to lay it out from scratch repeatedly
pub struct TaffyLayoutTree<NodeContext = ()> {
    pub tree: TaffyTree<NodeContext>,
    pub root: NodeId,
    nodes: Vec<NodeId>,
}

impl<NodeContext> TaffyLayoutTree<NodeContext> {
    pub fn new((tree, root): (TaffyTree<NodeContext>, NodeId)) -> Self {
        let mut nodes = vec![root];
        let mut next = 0;
        while let Some(&node) = nodes.get(next) {
            nodes.extend(tree.children(node).unwrap());
            next += 1;
        }
        Self { tree, root, nodes }
    }

    /// Clear everything that is cached for every node in the tree: both the cached layout of each node and the
    /// cached grid item placement of each grid container, so that the next layout is a layout from scratch
    pub fn mark_all_dirty(&mut self) {
        for &node in &self.nodes {
            // The cached grid item placement of a node survives `mark_dirty`, but not its style being set
            let style = self.tree.style(node).unwrap();
            if style.display == Display::Grid {
                let style = style.clone();
                self.tree.set_style(node, style).unwrap();
            }
            self.tree.mark_dirty(node).unwrap();
        }
    }

    /// Clear the cached layout of every node in the tree, keeping the cached grid item placement of each
    /// grid container (which is what happens when a tree is laid out again after its content has changed)
    pub fn mark_all_layouts_dirty(&mut self) {
        for &node in &self.nodes {
            self.tree.mark_dirty(node).unwrap();
        }
    }
}

pub trait GenStyle<Style: Default>: Clone {
    fn create_leaf_style(&mut self, rng: &mut impl Rng) -> Style;
    fn create_container_style(&mut self, rng: &mut impl Rng) -> Style;
    fn create_root_style(&mut self, _rng: &mut impl Rng) -> Style {
        Default::default()
    }
}

#[derive(Clone)]
pub struct FixedStyleGenerator(pub TaffyStyle);
impl GenStyle<TaffyStyle> for FixedStyleGenerator {
    fn create_leaf_style(&mut self, _rng: &mut impl Rng) -> TaffyStyle {
        self.0.clone()
    }
    fn create_container_style(&mut self, _rng: &mut impl Rng) -> TaffyStyle {
        self.0.clone()
    }
}

pub trait BuildTree<R: Rng, G: GenStyle<TaffyStyle>> {
    const NAME: &'static str;
    type Tree;
    type Node: Clone;

    fn with_rng(rng: R, style_generator: G) -> Self;

    fn compute_layout_inner(&mut self, available_width: Option<f32>, available_height: Option<f32>);
    /// Discard any cached layout results so that the next layout recomputes the entire tree
    fn mark_all_dirty(&mut self);
    fn random_usize(&mut self, range: impl SampleRange<usize>) -> usize;
    fn create_leaf_node(&mut self) -> Self::Node;
    fn create_container_node(&mut self, children: &[Self::Node]) -> Self::Node;
    fn set_root_children(&mut self, children: &[Self::Node]);
    fn total_node_count(&mut self) -> usize;
    fn into_tree_and_root(self) -> (Self::Tree, Self::Node);

    fn build_n_leaf_nodes(&mut self, n: usize) -> Vec<Self::Node> {
        (0..n).map(|_| self.create_leaf_node()).collect()
    }

    /// A helper function to recursively construct a deep tree
    fn build_deep_tree(&mut self, max_nodes: u32, branching_factor: u32) -> Vec<Self::Node> {
        if max_nodes <= branching_factor {
            // Build leaf nodes
            return (0..max_nodes).map(|_| self.create_leaf_node()).collect();
        }

        // Add another layer to the tree
        // Each child gets an equal amount of the remaining nodes
        (0..branching_factor)
            .map(|_| {
                let max_nodes = (max_nodes - branching_factor) / branching_factor;
                let children = self.build_deep_tree(max_nodes, branching_factor);
                self.create_container_node(&children)
            })
            .collect()
    }
}

pub trait BuildTreeExt<G: GenStyle<TaffyStyle>>: BuildTree<ChaCha8Rng, G> {
    fn with_seed(seed: u64, style_generator: G) -> Self
    where
        Self: Sized,
    {
        let rng = ChaCha8Rng::seed_from_u64(seed);
        Self::with_rng(rng, style_generator)
    }

    fn new(style_generator: G) -> Self
    where
        Self: Sized,
    {
        Self::with_seed(STANDARD_RNG_SEED, style_generator)
    }

    /// A helper function to recursively construct a deep tree
    fn build_super_deep_hierarchy(&mut self, depth: u32, nodes_per_level: u32) {
        let mut children = Vec::with_capacity(nodes_per_level as usize);
        for _ in 0..depth {
            let node_with_children = self.create_container_node(&children);

            children.clear();
            children.push(node_with_children);
            for _ in 0..(nodes_per_level - 1) {
                children.push(self.create_leaf_node())
            }
        }
        self.set_root_children(&children);
    }

    /// A tree with a higher depth for a more realistic scenario
    fn build_deep_hierarchy(&mut self, node_count: u32, branching_factor: u32) {
        let children = self.build_deep_tree(node_count, branching_factor);
        self.set_root_children(&children);
    }

    /// A tree with many children that have shallow depth
    fn build_flat_hierarchy(&mut self, target_node_count: u32) {
        let mut children = Vec::new();

        while self.total_node_count() < target_node_count as usize {
            let count = self.random_usize(1..=4);
            let sub_children = self.build_n_leaf_nodes(count);
            let node = self.create_container_node(&sub_children);
            children.push(node);
        }

        self.set_root_children(&children);
    }

    /// Compute layout given a viewport size
    fn compute_layout(&mut self, available_width: Option<f32>, available_height: Option<f32>) {
        self.compute_layout_inner(available_width, available_height);
    }
}
