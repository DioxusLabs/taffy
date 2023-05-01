// As each benchmark suite is compiled as a seperate crate and uses different helpers, we end up with a bunch
// of false positives for this lint. So let's just disable it for this code.
#![allow(dead_code)]

pub mod taffy_helpers;
pub use taffy_helpers::TaffyTreeBuilder;

#[cfg(feature = "yoga")]
pub mod yoga_helpers;
#[cfg(feature = "yoga")]
pub use yoga_helpers::YogaTreeBuilder;

use rand::distributions::uniform::SampleRange;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use taffy::style::Style as TaffyStyle;

pub const STANDARD_RNG_SEED: u64 = 12345;

pub trait GenStyle<Style: Default> {
    fn create_leaf_style(&mut self, rng: &mut impl Rng) -> Style;
    fn create_container_style(&mut self, rng: &mut impl Rng) -> Style;
    fn create_root_style(&mut self, _rng: &mut impl Rng) -> Style {
        Default::default()
    }
}

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
    type Tree;
    type Node: Clone;

    fn with_rng(rng: R, style_generator: G) -> Self;

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

    /// A tree with a higher depth for a more realistic scenario
    fn build_deep_hierarchy(mut self, node_count: u32, branching_factor: u32) -> (Self::Tree, Self::Node)
    where
        Self: Sized,
    {
        let children = self.build_deep_tree(node_count, branching_factor);
        self.set_root_children(&children);
        self.into_tree_and_root()
    }

    /// A tree with many children that have shallow depth
    fn build_flat_hierarchy(mut self, target_node_count: u32) -> (Self::Tree, Self::Node)
    where
        Self: Sized,
    {
        let mut children = Vec::new();

        while self.total_node_count() < target_node_count as usize {
            let count = self.random_usize(1..=4);
            let sub_children = self.build_n_leaf_nodes(count);
            let node = self.create_container_node(&sub_children);
            children.push(node);
        }

        self.set_root_children(&children);
        self.into_tree_and_root()
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
}
