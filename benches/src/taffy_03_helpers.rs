use rand::distributions::uniform::SampleRange;
use rand::Rng;
use rand_chacha::ChaCha8Rng;
use taffy::style::Style as TaffyStyle;

use super::{BuildTree, BuildTreeExt, GenStyle};

pub struct Taffy03TreeBuilder<R: Rng, G: GenStyle<TaffyStyle>> {
    rng: R,
    style_generator: G,
    tree: taffy_03::Taffy,
    root: taffy_03::prelude::Node,
}

// Implement the BuildTree trait
impl<R: Rng, G: GenStyle<TaffyStyle>> BuildTree<R, G> for Taffy03TreeBuilder<R, G> {
    const NAME: &'static str = "Taffy 0.3";
    type Tree = taffy_03::Taffy;
    type Node = taffy_03::prelude::Node;

    fn with_rng(mut rng: R, mut style_generator: G) -> Self {
        let mut tree = taffy_03::Taffy::new();
        let root = tree.new_leaf(convert_style(style_generator.create_root_style(&mut rng))).unwrap();
        Taffy03TreeBuilder { rng, style_generator, tree, root }
    }

    fn compute_layout_inner(&mut self, available_width: Option<f32>, available_height: Option<f32>) {
        let available_space =
            taffy_03::geometry::Size { width: available_width.into(), height: available_height.into() };
        self.tree.compute_layout(self.root, available_space).unwrap();
    }

    fn random_usize(&mut self, range: impl SampleRange<usize>) -> usize {
        self.rng.gen_range(range)
    }

    fn create_leaf_node(&mut self) -> Self::Node {
        let style = self.style_generator.create_leaf_style(&mut self.rng);
        self.tree.new_leaf(convert_style(style)).unwrap()
    }

    fn create_container_node(&mut self, children: &[Self::Node]) -> Self::Node {
        let style = self.style_generator.create_container_style(&mut self.rng);
        self.tree.new_with_children(convert_style(style), children).unwrap()
    }

    fn total_node_count(&mut self) -> usize {
        self.tree.total_node_count()
    }

    fn set_root_children(&mut self, children: &[Self::Node]) {
        self.tree.set_children(self.root, children).unwrap();
    }

    fn into_tree_and_root(self) -> (Self::Tree, Self::Node) {
        (self.tree, self.root)
    }
}

impl<G: GenStyle<TaffyStyle>> BuildTreeExt<G> for Taffy03TreeBuilder<ChaCha8Rng, G> {}

fn convert_style(style: taffy::style::Style) -> taffy_03::style::Style {
    taffy_03::style::Style {
        display: convert_display(style.display),
        position: convert_position(style.position),
        inset: convert_rect(style.inset, convert_length_percentage_auto),
        margin: convert_rect(style.margin, convert_length_percentage_auto),
        padding: convert_rect(style.padding, convert_length_percentage),
        border: convert_rect(style.border, convert_length_percentage),
        size: convert_size(style.size, convert_dimension),
        min_size: convert_size(style.min_size, convert_dimension),
        max_size: convert_size(style.max_size, convert_dimension),
        aspect_ratio: style.aspect_ratio,
        gap: convert_size(style.gap, convert_length_percentage),
        // Alignment
        align_items: None,
        align_self: None,
        justify_items: None,
        justify_self: None,
        align_content: None,
        justify_content: None,
        // Flexbox
        flex_direction: convert_flex_direction(style.flex_direction),
        flex_wrap: convert_flex_wrap(style.flex_wrap),
        flex_grow: style.flex_grow,
        flex_shrink: style.flex_shrink,
        flex_basis: convert_dimension(style.flex_basis),
        // Grid
        grid_template_rows: Vec::new(),
        grid_template_columns: Vec::new(),
        grid_auto_rows: Vec::new(),
        grid_auto_columns: Vec::new(),
        grid_auto_flow: taffy_03::style::GridAutoFlow::Row,
        grid_row: taffy_03::geometry::Line {
            start: taffy_03::style::GridPlacement::Auto,
            end: taffy_03::style::GridPlacement::Auto,
        },
        grid_column: taffy_03::geometry::Line {
            start: taffy_03::style::GridPlacement::Auto,
            end: taffy_03::style::GridPlacement::Auto,
        },
    }
}

fn convert_rect<T, U, F: Fn(T) -> U>(input: taffy::geometry::Rect<T>, map: F) -> taffy_03::geometry::Rect<U> {
    taffy_03::geometry::Rect {
        left: map(input.left),
        right: map(input.right),
        top: map(input.top),
        bottom: map(input.bottom),
    }
}

fn convert_size<T, U, F: Fn(T) -> U>(input: taffy::geometry::Size<T>, map: F) -> taffy_03::geometry::Size<U> {
    taffy_03::geometry::Size { width: map(input.width), height: map(input.height) }
}

fn convert_point<T, U, F: Fn(T) -> U>(input: taffy::geometry::Point<T>, map: F) -> taffy_03::geometry::Point<U> {
    taffy_03::geometry::Point { x: map(input.x), y: map(input.y) }
}

fn convert_dimension(input: taffy::style::Dimension) -> taffy_03::style::Dimension {
    match input {
        taffy::style::Dimension::Length(val) => taffy_03::style::Dimension::Points(val),
        taffy::style::Dimension::Percent(val) => taffy_03::style::Dimension::Percent(val),
        taffy::style::Dimension::Auto => taffy_03::style::Dimension::Auto,
    }
}

fn convert_length_percentage_auto(input: taffy::style::LengthPercentageAuto) -> taffy_03::style::LengthPercentageAuto {
    match input {
        taffy::style::LengthPercentageAuto::Length(val) => taffy_03::style::LengthPercentageAuto::Points(val),
        taffy::style::LengthPercentageAuto::Percent(val) => taffy_03::style::LengthPercentageAuto::Percent(val),
        taffy::style::LengthPercentageAuto::Auto => taffy_03::style::LengthPercentageAuto::Auto,
    }
}

fn convert_length_percentage(input: taffy::style::LengthPercentage) -> taffy_03::style::LengthPercentage {
    match input {
        taffy::style::LengthPercentage::Length(val) => taffy_03::style::LengthPercentage::Points(val),
        taffy::style::LengthPercentage::Percent(val) => taffy_03::style::LengthPercentage::Percent(val),
    }
}

fn convert_display(input: taffy::style::Display) -> taffy_03::style::Display {
    match input {
        taffy::style::Display::None => taffy_03::style::Display::None,
        taffy::style::Display::Flex => taffy_03::style::Display::Flex,
        taffy::style::Display::Grid => taffy_03::style::Display::Grid,
        taffy::style::Display::Block => panic!("Block layout not implemented in taffy 0.3"),
    }
}

fn convert_position(input: taffy::style::Position) -> taffy_03::style::Position {
    match input {
        taffy::style::Position::Relative => taffy_03::style::Position::Relative,
        taffy::style::Position::Absolute => taffy_03::style::Position::Absolute,
    }
}

fn convert_flex_direction(input: taffy::style::FlexDirection) -> taffy_03::style::FlexDirection {
    match input {
        taffy::style::FlexDirection::Row => taffy_03::style::FlexDirection::Row,
        taffy::style::FlexDirection::Column => taffy_03::style::FlexDirection::Column,
        taffy::style::FlexDirection::RowReverse => taffy_03::style::FlexDirection::RowReverse,
        taffy::style::FlexDirection::ColumnReverse => taffy_03::style::FlexDirection::ColumnReverse,
    }
}

fn convert_flex_wrap(input: taffy::style::FlexWrap) -> taffy_03::style::FlexWrap {
    match input {
        taffy::style::FlexWrap::NoWrap => taffy_03::style::FlexWrap::NoWrap,
        taffy::style::FlexWrap::Wrap => taffy_03::style::FlexWrap::Wrap,
        taffy::style::FlexWrap::WrapReverse => taffy_03::style::FlexWrap::WrapReverse,
    }
}
