#![allow(dead_code)]

use rand::distributions::uniform::SampleRange;
use rand::Rng;
use rand_chacha::ChaCha8Rng;
use slotmap::{DefaultKey, SlotMap};

use super::{BuildTree, BuildTreeExt, GenStyle};

pub mod yg {
    pub use ordered_float::OrderedFloat;
    pub use slotmap::{DefaultKey, SlotMap};
    pub use yoga::types::*;
    pub use yoga::Node;

    pub type NodeId = DefaultKey;
    pub type YogaTree = SlotMap<DefaultKey, Node>;
}
mod tf {
    pub use taffy::prelude::*;
}
use tf::Style as TaffyStyle;

pub struct YogaTreeBuilder<R: Rng, G: GenStyle<TaffyStyle>> {
    rng: R,
    style_generator: G,
    tree: yg::YogaTree,
    root: yg::DefaultKey,
}

// Implement the BuildTree trait
impl<R: Rng, G: GenStyle<TaffyStyle>> BuildTree<R, G> for YogaTreeBuilder<R, G> {
    const NAME: &'static str = "Yoga";
    type Tree = yg::YogaTree;
    type Node = DefaultKey;

    fn with_rng(mut rng: R, mut style_generator: G) -> Self {
        let mut tree = SlotMap::new();
        let root = create_yg_node(&mut tree, &style_generator.create_root_style(&mut rng), &[]);
        YogaTreeBuilder { rng, style_generator, tree, root }
    }

    fn compute_layout(&mut self, available_width: Option<f32>, available_height: Option<f32>) {
        self.tree[self.root].calculate_layout(
            available_width.unwrap_or(f32::INFINITY),
            available_height.unwrap_or(f32::INFINITY),
            yg::Direction::LTR,
        )
    }

    fn random_usize(&mut self, range: impl SampleRange<usize>) -> usize {
        self.rng.gen_range(range)
    }

    fn create_leaf_node(&mut self) -> Self::Node {
        let style = self.style_generator.create_leaf_style(&mut self.rng);
        create_yg_node(&mut self.tree, &style, &[])
    }

    fn create_container_node(&mut self, children: &[Self::Node]) -> Self::Node {
        let style = self.style_generator.create_container_style(&mut self.rng);
        create_yg_node(&mut self.tree, &style, &children)
    }

    fn set_root_children(&mut self, children: &[Self::Node]) {
        set_node_children(&mut self.tree, self.root, &children);
    }

    fn total_node_count(&mut self) -> usize {
        self.tree.len()
    }

    fn into_tree_and_root(self) -> (Self::Tree, Self::Node) {
        (self.tree, self.root)
    }
}

impl<G: GenStyle<TaffyStyle>> BuildTreeExt<G> for YogaTreeBuilder<ChaCha8Rng, G> {}

// impl<R: Rng, G: GenStyle<TaffyStyle>> YogaTreeBuilder<R, G> {
//     /// Create a YogaTreeBuilder with a standard rng from a style generator
//     fn new<NG: GenStyle<TaffyStyle>>(mut style_generator: NG) -> YogaTreeBuilder<ChaCha8Rng, NG> {
//         let mut rng = ChaCha8Rng::seed_from_u64(STANDARD_RNG_SEED);
//         let mut tree = SlotMap::new();
//         let root = create_yg_node(&mut tree, &style_generator.create_root_style(&mut rng), &[]);
//         YogaTreeBuilder { rng, style_generator, tree, root }
//     }

//     /// Create a YogaTreeBuilder with a standard rng from a style generator
//     fn with_seed<NG: GenStyle<TaffyStyle>>(seed: u64, mut style_generator: NG) -> YogaTreeBuilder<ChaCha8Rng, NG> {
//         let mut rng = ChaCha8Rng::seed_from_u64(seed);
//         let mut tree = SlotMap::new();
//         let root = create_yg_node(&mut tree, &style_generator.create_root_style(&mut rng), &[]);
//         YogaTreeBuilder { rng, style_generator, tree, root }
//     }

//     /// Create a YogaTreeBuilder from a random number generator and a style generator
//     fn with_rng<NR: Rng, NG: GenStyle<TaffyStyle>>(mut rng: NR, mut style_generator: NG) -> YogaTreeBuilder<NR, NG> {
//         let mut tree = SlotMap::new();
//         let root = create_yg_node(&mut tree, &style_generator.create_root_style(&mut rng), &[]);
//         YogaTreeBuilder { rng, style_generator, tree, root }
//     }
// }

fn create_yg_node(tree: &mut yg::YogaTree, style: &tf::Style, children: &[yg::DefaultKey]) -> yg::DefaultKey {
    let mut node = yg::Node::new();
    apply_taffy_style(&mut node, &style);
    for (i, child) in children.into_iter().enumerate() {
        node.insert_child(&mut tree[*child], i as u32);
    }
    tree.insert(node)
}

pub fn new_default_style_with_children(tree: &mut yg::YogaTree, children: &[yg::DefaultKey]) -> yg::DefaultKey {
    let mut node = yg::Node::new();
    for (i, child) in children.into_iter().enumerate() {
        node.insert_child(&mut tree[*child], i as u32);
    }
    tree.insert(node)
}

fn set_node_children(tree: &mut yg::YogaTree, node_id: yg::DefaultKey, children: &[yg::DefaultKey]) {
    // TODO: clear existing children.
    for (i, child_id) in children.into_iter().enumerate() {
        let [node, child] = tree.get_disjoint_mut([node_id, *child_id]).unwrap();
        node.insert_child(child, i as u32);
    }
}

fn into_yg_units(dim: impl Into<tf::Dimension>) -> yg::StyleUnit {
    match dim.into() {
        tf::Dimension::Auto => yg::StyleUnit::Auto,
        tf::Dimension::Length(val) => yg::StyleUnit::Point(yg::OrderedFloat(val)),
        tf::Dimension::Percent(val) => yg::StyleUnit::Percent(yg::OrderedFloat(val)),
    }
}

fn into_pixels(dim: impl Into<tf::Dimension>) -> f32 {
    dim.into().into_option().unwrap_or(0.0)
}

fn items_into_align(align: Option<tf::AlignSelf>) -> yg::Align {
    match align {
        None => yg::Align::Auto,
        Some(tf::AlignSelf::FlexStart) => yg::Align::FlexStart,
        Some(tf::AlignSelf::FlexEnd) => yg::Align::FlexEnd,
        Some(tf::AlignSelf::Center) => yg::Align::Center,
        Some(tf::AlignSelf::Baseline) => yg::Align::Baseline,
        Some(tf::AlignSelf::Stretch) => yg::Align::Stretch,
        Some(tf::AlignSelf::Start) => unimplemented!(),
        Some(tf::AlignSelf::End) => unimplemented!(),
    }
}

fn content_into_align(align: Option<tf::AlignContent>) -> yg::Align {
    match align {
        None => yg::Align::Auto,
        Some(tf::AlignContent::FlexStart) => yg::Align::FlexStart,
        Some(tf::AlignContent::Start) => yg::Align::FlexStart,
        Some(tf::AlignContent::FlexEnd) => yg::Align::FlexEnd,
        Some(tf::AlignContent::End) => yg::Align::FlexEnd,
        Some(tf::AlignContent::Center) => yg::Align::Center,
        Some(tf::AlignContent::Stretch) => yg::Align::Stretch,
        Some(tf::AlignContent::SpaceBetween) => yg::Align::SpaceBetween,
        Some(tf::AlignContent::SpaceAround) => yg::Align::SpaceAround,
        Some(tf::AlignContent::SpaceEvenly) => unimplemented!(),
    }
}

fn content_into_justify(align: Option<tf::JustifyContent>) -> yg::Justify {
    match align {
        None => yg::Justify::FlexStart,
        Some(tf::JustifyContent::FlexStart) => yg::Justify::FlexStart,
        Some(tf::JustifyContent::Start) => yg::Justify::FlexStart,
        Some(tf::JustifyContent::FlexEnd) => yg::Justify::FlexEnd,
        Some(tf::JustifyContent::End) => yg::Justify::FlexEnd,
        Some(tf::JustifyContent::Center) => yg::Justify::Center,
        Some(tf::JustifyContent::SpaceBetween) => yg::Justify::SpaceBetween,
        Some(tf::JustifyContent::SpaceAround) => yg::Justify::SpaceAround,
        Some(tf::JustifyContent::Stretch) => unimplemented!(),
        Some(tf::JustifyContent::SpaceEvenly) => unimplemented!(),
    }
}

fn apply_taffy_style(node: &mut yg::Node, style: &tf::Style) {
    // display
    node.set_display(match style.display {
        tf::Display::None => yg::Display::None,
        tf::Display::Flex => yg::Display::Flex,
        tf::Display::Grid => panic!("Yoga does not support CSS Grid layout"),
        tf::Display::Block => panic!("Yoga does not support CSS Block layout"),
    });

    // position
    node.set_position_type(match style.position {
        tf::Position::Relative => yg::PositionType::Relative,
        tf::Position::Absolute => yg::PositionType::Absolute,
    });
    // inset
    node.set_position(yg::Edge::Left, into_yg_units(style.inset.left));
    node.set_position(yg::Edge::Right, into_yg_units(style.inset.right));
    node.set_position(yg::Edge::Top, into_yg_units(style.inset.top));
    node.set_position(yg::Edge::Bottom, into_yg_units(style.inset.bottom));

    // sizes
    node.set_width(into_yg_units(style.size.width));
    node.set_height(into_yg_units(style.size.height));
    node.set_min_width(into_yg_units(style.min_size.width));
    node.set_min_height(into_yg_units(style.min_size.height));
    node.set_max_width(into_yg_units(style.max_size.width));
    node.set_max_height(into_yg_units(style.max_size.height));

    // aspect_ratio
    if let Some(aspect_ratio) = style.aspect_ratio {
        node.set_aspect_ratio(aspect_ratio);
    }

    // spacing
    node.set_padding(yg::Edge::Left, into_yg_units(style.padding.left));
    node.set_padding(yg::Edge::Right, into_yg_units(style.padding.right));
    node.set_padding(yg::Edge::Top, into_yg_units(style.padding.top));
    node.set_padding(yg::Edge::Bottom, into_yg_units(style.padding.bottom));
    node.set_margin(yg::Edge::Left, into_yg_units(style.margin.left));
    node.set_margin(yg::Edge::Right, into_yg_units(style.margin.right));
    node.set_margin(yg::Edge::Top, into_yg_units(style.margin.top));
    node.set_margin(yg::Edge::Bottom, into_yg_units(style.margin.bottom));
    node.set_border(yg::Edge::Left, into_pixels(style.border.left));
    node.set_border(yg::Edge::Right, into_pixels(style.border.right));
    node.set_border(yg::Edge::Top, into_pixels(style.border.top));
    node.set_border(yg::Edge::Bottom, into_pixels(style.border.bottom));

    // alignment
    node.set_align_items(items_into_align(style.align_items));
    node.set_align_self(items_into_align(style.align_self));
    node.set_align_content(content_into_align(style.align_content));
    node.set_justify_content(content_into_justify(style.justify_content));

    // gap
    node.set_column_gap(into_pixels(style.gap.width));
    node.set_row_gap(into_pixels(style.gap.height));

    // flex
    node.set_flex_direction(match style.flex_direction {
        tf::FlexDirection::Row => yg::FlexDirection::Row,
        tf::FlexDirection::Column => yg::FlexDirection::Column,
        tf::FlexDirection::RowReverse => yg::FlexDirection::RowReverse,
        tf::FlexDirection::ColumnReverse => yg::FlexDirection::ColumnReverse,
    });
    node.set_flex_wrap(match style.flex_wrap {
        tf::FlexWrap::NoWrap => yg::Wrap::NoWrap,
        tf::FlexWrap::Wrap => yg::Wrap::Wrap,
        tf::FlexWrap::WrapReverse => yg::Wrap::WrapReverse,
    });
    node.set_flex_basis(into_yg_units(style.flex_basis));
    node.set_flex_grow(style.flex_grow);
    node.set_flex_shrink(style.flex_shrink);
}
