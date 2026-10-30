//! Translation of the styles used by the comparison benchmarks.
//! Cephal's taffy-compat module only converts in the opposite direction.
use cephal::{style as c, AvailableSpace, NodeId, Size, Style, Tree};
use rand::distr::uniform::SampleRange;
use rand::Rng;
use rand_chacha::ChaCha8Rng;
use taffy::style as t;

use super::{BuildTree, BuildTreeExt, GenStyle};

fn length(value: taffy::CompactLength) -> c::Length {
    use taffy::CompactLength as L;
    match value.tag() {
        L::LENGTH_TAG => c::Length::length(value.value()),
        L::PERCENT_TAG => c::Length::percent(value.value()),
        L::AUTO_TAG => c::Length::AUTO,
        L::FR_TAG => c::Length::fr(value.value()),
        L::MIN_CONTENT_TAG => c::Length::MIN_CONTENT,
        L::MAX_CONTENT_TAG => c::Length::MAX_CONTENT,
        L::FIT_CONTENT_PX_TAG => c::Length::fit_content_px(value.value()),
        L::FIT_CONTENT_PERCENT_TAG => c::Length::fit_content_percent(value.value()),
        L::FIT_CONTENT_KEYWORD_TAG => c::Length::FIT_CONTENT,
        L::STRETCH_TAG => c::Length::STRETCH,
        L::CONTENT_TAG => c::Length::CONTENT,
        _ => panic!("calc lengths are not supported by the comparison benchmarks"),
    }
}

fn display(value: t::Display) -> c::Display {
    match value {
        t::Display::Block => c::Display::Block,
        t::Display::FlowRoot => c::Display::FlowRoot,
        t::Display::Flex => c::Display::Flex,
        t::Display::Grid => c::Display::Grid,
        t::Display::None => c::Display::None,
    }
}

fn position(value: t::Position) -> c::Position {
    match value {
        t::Position::Static => c::Position::Static,
        t::Position::Relative => c::Position::Relative,
        t::Position::Absolute => c::Position::Absolute,
        t::Position::Fixed => c::Position::Fixed,
        t::Position::Sticky => c::Position::Sticky,
    }
}

fn box_sizing(value: t::BoxSizing) -> c::BoxSizing {
    match value {
        t::BoxSizing::BorderBox => c::BoxSizing::BorderBox,
        t::BoxSizing::ContentBox => c::BoxSizing::ContentBox,
    }
}

fn direction(value: t::Direction) -> c::Direction {
    match value {
        t::Direction::Ltr => c::Direction::Ltr,
        t::Direction::Rtl => c::Direction::Rtl,
    }
}

fn flex_direction(value: t::FlexDirection) -> c::FlexDirection {
    match value {
        t::FlexDirection::Row => c::FlexDirection::Row,
        t::FlexDirection::Column => c::FlexDirection::Column,
        t::FlexDirection::RowReverse => c::FlexDirection::RowReverse,
        t::FlexDirection::ColumnReverse => c::FlexDirection::ColumnReverse,
    }
}

fn flex_wrap(value: t::FlexWrap) -> c::FlexWrap {
    match value {
        t::FlexWrap::NoWrap => c::FlexWrap::NoWrap,
        t::FlexWrap::Wrap => c::FlexWrap::Wrap,
        t::FlexWrap::WrapReverse => c::FlexWrap::WrapReverse,
        t::FlexWrap::Balance => c::FlexWrap::Balance,
        t::FlexWrap::BalanceReverse => c::FlexWrap::BalanceReverse,
    }
}

fn grid_auto_flow(value: t::GridAutoFlow) -> c::GridAutoFlow {
    match value {
        t::GridAutoFlow::Row => c::GridAutoFlow::Row,
        t::GridAutoFlow::Column => c::GridAutoFlow::Column,
        t::GridAutoFlow::RowDense => c::GridAutoFlow::RowDense,
        t::GridAutoFlow::ColumnDense => c::GridAutoFlow::ColumnDense,
    }
}

fn alignment_safety(value: t::AlignmentSafety) -> c::AlignmentSafety {
    match value {
        t::AlignmentSafety::Default => c::AlignmentSafety::Default,
        t::AlignmentSafety::Unsafe => c::AlignmentSafety::Unsafe,
        t::AlignmentSafety::Safe => c::AlignmentSafety::Safe,
    }
}

fn align_items(value: t::AlignItems) -> Option<c::AlignItems> {
    let keyword = match value.keyword {
        t::AlignItemsKeyword::Normal => return None,
        t::AlignItemsKeyword::Start => c::AlignItemsKeyword::Start,
        t::AlignItemsKeyword::End => c::AlignItemsKeyword::End,
        t::AlignItemsKeyword::FlexStart => c::AlignItemsKeyword::FlexStart,
        t::AlignItemsKeyword::FlexEnd => c::AlignItemsKeyword::FlexEnd,
        t::AlignItemsKeyword::SelfStart => c::AlignItemsKeyword::SelfStart,
        t::AlignItemsKeyword::SelfEnd => c::AlignItemsKeyword::SelfEnd,
        t::AlignItemsKeyword::Center => c::AlignItemsKeyword::Center,
        t::AlignItemsKeyword::Baseline => c::AlignItemsKeyword::Baseline,
        t::AlignItemsKeyword::Stretch => c::AlignItemsKeyword::Stretch,
    };
    Some(c::AlignItems { keyword, safety: alignment_safety(value.safety) })
}

fn align_content(value: t::AlignContent) -> Option<c::AlignContent> {
    let keyword = match value.keyword {
        t::AlignContentKeyword::Normal => return None,
        t::AlignContentKeyword::Start => c::AlignContentKeyword::Start,
        t::AlignContentKeyword::End => c::AlignContentKeyword::End,
        t::AlignContentKeyword::FlexStart => c::AlignContentKeyword::FlexStart,
        t::AlignContentKeyword::FlexEnd => c::AlignContentKeyword::FlexEnd,
        t::AlignContentKeyword::Center => c::AlignContentKeyword::Center,
        t::AlignContentKeyword::Stretch => c::AlignContentKeyword::Stretch,
        t::AlignContentKeyword::SpaceBetween => c::AlignContentKeyword::SpaceBetween,
        t::AlignContentKeyword::SpaceEvenly => c::AlignContentKeyword::SpaceEvenly,
        t::AlignContentKeyword::SpaceAround => c::AlignContentKeyword::SpaceAround,
    };
    Some(c::AlignContent { keyword, safety: alignment_safety(value.safety) })
}

fn track(value: t::TrackSizingFunction) -> c::TrackSizingFunction {
    c::TrackSizingFunction {
        min: c::MinTrackSizingFunction(length(value.min.into_raw())),
        max: c::MaxTrackSizingFunction(length(value.max.into_raw())),
    }
}

fn component(value: &t::GridTemplateComponent<String>) -> c::GridTemplateComponent {
    match value {
        t::GridTemplateComponent::Single(value) => c::GridTemplateComponent::Single(track(*value)),
        _ => panic!("repeated tracks are not used by the comparison benchmarks"),
    }
}

fn placement(value: &t::GridPlacement) -> c::GridPlacement {
    match value {
        t::GridPlacement::Auto => c::GridPlacement::Auto,
        t::GridPlacement::Line(value) => c::GridPlacement::Line(value.as_i16()),
        t::GridPlacement::Span(value) => c::GridPlacement::Span(*value),
        _ => panic!("named grid lines are not used by the comparison benchmarks"),
    }
}

/// Convert the properties exercised by the benchmark generators.
pub fn style(value: &taffy::Style) -> Style {
    let mut result = Style {
        display: display(value.display),
        position: position(value.position),
        box_sizing: box_sizing(value.box_sizing),
        direction: direction(value.direction),
        aspect_ratio: value.aspect_ratio,
        flex_direction: flex_direction(value.flex_direction),
        flex_wrap: flex_wrap(value.flex_wrap),
        flex_basis: c::Dimension(length(value.flex_basis.into_raw())),
        flex_grow: value.flex_grow,
        flex_shrink: value.flex_shrink,
        flex_line_count: value.flex_line_count,
        align_items: align_items(value.align_items),
        align_self: value.align_self.and_then(align_items),
        justify_items: align_items(value.justify_items),
        justify_self: value.justify_self.and_then(align_items),
        align_content: align_content(value.align_content),
        justify_content: align_content(value.justify_content),
        ..Style::DEFAULT
    };
    result.size = Size {
        width: c::Dimension(length(value.size.width.into_raw())),
        height: c::Dimension(length(value.size.height.into_raw())),
    };
    result.min_size = Size {
        width: c::LengthPercentageAuto(length(value.min_size.width.into_raw())),
        height: c::LengthPercentageAuto(length(value.min_size.height.into_raw())),
    };
    result.max_size = Size {
        width: c::LengthPercentageAuto(length(value.max_size.width.into_raw())),
        height: c::LengthPercentageAuto(length(value.max_size.height.into_raw())),
    };
    result.gap = Size {
        width: c::LengthPercentage(length(value.gap.width.into_raw())),
        height: c::LengthPercentage(length(value.gap.height.into_raw())),
    };
    result.inset = cephal::Rect {
        left: c::LengthPercentageAuto(length(value.inset.left.into_raw())),
        right: c::LengthPercentageAuto(length(value.inset.right.into_raw())),
        top: c::LengthPercentageAuto(length(value.inset.top.into_raw())),
        bottom: c::LengthPercentageAuto(length(value.inset.bottom.into_raw())),
    };
    result.margin = cephal::Rect {
        left: c::LengthPercentageAuto(length(value.margin.left.into_raw())),
        right: c::LengthPercentageAuto(length(value.margin.right.into_raw())),
        top: c::LengthPercentageAuto(length(value.margin.top.into_raw())),
        bottom: c::LengthPercentageAuto(length(value.margin.bottom.into_raw())),
    };
    result.padding = cephal::Rect {
        left: c::LengthPercentage(length(value.padding.left.into_raw())),
        right: c::LengthPercentage(length(value.padding.right.into_raw())),
        top: c::LengthPercentage(length(value.padding.top.into_raw())),
        bottom: c::LengthPercentage(length(value.padding.bottom.into_raw())),
    };
    result.border = cephal::Rect {
        left: c::LengthPercentage(length(value.border.left.into_raw())),
        right: c::LengthPercentage(length(value.border.right.into_raw())),
        top: c::LengthPercentage(length(value.border.top.into_raw())),
        bottom: c::LengthPercentage(length(value.border.bottom.into_raw())),
    };
    result.grid_row = cephal::Line { start: placement(&value.grid_row.start), end: placement(&value.grid_row.end) };
    result.grid_column =
        cephal::Line { start: placement(&value.grid_column.start), end: placement(&value.grid_column.end) };
    if value.display == t::Display::Grid {
        let grid = result.grid_mut();
        grid.template_columns = value.grid_template_columns.iter().map(component).collect();
        grid.template_rows = value.grid_template_rows.iter().map(component).collect();
        grid.auto_columns = value.grid_auto_columns.iter().copied().map(track).collect();
        grid.auto_rows = value.grid_auto_rows.iter().copied().map(track).collect();
        grid.auto_flow = grid_auto_flow(value.grid_auto_flow);
    }
    result
}

pub fn available_space(value: taffy::Size<taffy::AvailableSpace>) -> Size<AvailableSpace> {
    fn axis(value: taffy::AvailableSpace) -> AvailableSpace {
        match value {
            taffy::AvailableSpace::Definite(value) => AvailableSpace::Definite(value),
            taffy::AvailableSpace::MinContent => AvailableSpace::MinContent,
            taffy::AvailableSpace::MaxContent => AvailableSpace::MaxContent,
        }
    }
    Size { width: axis(value.width), height: axis(value.height) }
}

/// Copy an existing fixture outside the timed layout operation.
pub fn copy_tree<C>(source: &taffy::TaffyTree<C>, root: taffy::NodeId) -> (Tree, NodeId) {
    copy_tree_with_context(source, root, &mut |_| None)
}

pub fn copy_tree_with_context<C, D>(
    source: &taffy::TaffyTree<C>,
    root: taffy::NodeId,
    context: &mut impl FnMut(taffy::NodeId) -> Option<D>,
) -> (Tree<D>, NodeId) {
    fn copy<C, D>(
        source: &taffy::TaffyTree<C>,
        node: taffy::NodeId,
        target: &mut Tree<D>,
        context: &mut impl FnMut(taffy::NodeId) -> Option<D>,
    ) -> NodeId {
        let children: Vec<_> =
            source.children(node).unwrap().into_iter().map(|child| copy(source, child, target, context)).collect();
        let result = target.new_with_children(style(source.style(node).unwrap()), &children);
        target.set_context(result, context(node));
        result
    }
    let mut tree = Tree::new();
    let root = copy(source, root, &mut tree, context);
    (tree, root)
}

pub struct CephalTreeBuilder<R: Rng, G: GenStyle<taffy::Style>> {
    rng: R,
    style_generator: G,
    tree: Tree,
    root: NodeId,
}

impl<R: Rng, G: GenStyle<taffy::Style>> BuildTree<R, G> for CephalTreeBuilder<R, G> {
    const NAME: &'static str = "cephal";
    type Tree = Tree;
    type Node = NodeId;

    fn with_rng(mut rng: R, mut style_generator: G) -> Self {
        let mut tree = Tree::new();
        let root = tree.new_leaf(style(&style_generator.create_root_style(&mut rng)));
        Self { rng, style_generator, tree, root }
    }

    fn compute_layout_inner(&mut self, width: Option<f32>, height: Option<f32>) {
        self.tree
            .compute_layout(self.root, available_space(taffy::Size { width: width.into(), height: height.into() }));
    }

    fn mark_all_dirty(&mut self) {
        self.tree.clear_layout();
    }

    fn random_usize(&mut self, range: impl SampleRange<usize>) -> usize {
        self.rng.random_range(range)
    }

    fn create_leaf_node(&mut self) -> NodeId {
        self.tree.new_leaf(style(&self.style_generator.create_leaf_style(&mut self.rng)))
    }

    fn create_container_node(&mut self, children: &[NodeId]) -> NodeId {
        self.tree.new_with_children(style(&self.style_generator.create_container_style(&mut self.rng)), children)
    }

    fn create_node_with_style(&mut self, value: taffy::Style, children: &[NodeId]) -> NodeId {
        self.tree.new_with_children(style(&value), children)
    }

    fn set_root_children(&mut self, children: &[NodeId]) {
        self.tree.set_children(self.root, children);
    }

    fn total_node_count(&mut self) -> usize {
        self.tree.node_count()
    }

    fn into_tree_and_root(self) -> (Tree, NodeId) {
        (self.tree, self.root)
    }
}

impl<G: GenStyle<taffy::Style>> BuildTreeExt<G> for CephalTreeBuilder<ChaCha8Rng, G> {}

#[cfg(test)]
mod tests {
    use super::{available_space, c, copy_tree, placement, style, t, CephalTreeBuilder};
    use crate::{BuildTree, BuildTreeExt};
    use taffy::prelude::*;

    #[test]
    fn empty_builder_and_cold_layout() {
        let mut builder = CephalTreeBuilder::new(super::super::FixedStyleGenerator(taffy::Style::DEFAULT));
        assert_eq!(builder.total_node_count(), 1);
        builder.compute_layout(None, None);
        builder.mark_all_dirty();
        builder.compute_layout(None, None);
        let (tree, root) = builder.into_tree_and_root();
        assert_eq!(tree.layout(root).size, cephal::Size::ZERO);
    }

    #[test]
    fn copied_layout_and_incremental_edit_match_cold_layout() {
        for display in [taffy::Display::Flex, taffy::Display::Block, taffy::Display::Grid] {
            let mut source = taffy::TaffyTree::<()>::new();
            let leaf = source
                .new_leaf(taffy::Style {
                    size: Size { width: length(10.0), height: length(20.0) },
                    ..Default::default()
                })
                .unwrap();
            let root = source
                .new_with_children(
                    taffy::Style {
                        display,
                        padding: Rect::length(2.0),
                        grid_template_columns: vec![fr(1.0)],
                        ..Default::default()
                    },
                    &[leaf],
                )
                .unwrap();
            let space = Size { width: AvailableSpace::Definite(100.0), height: AvailableSpace::MaxContent };
            source.compute_layout(root, space).unwrap();
            let (mut tree, root) = copy_tree(&source, root);
            tree.compute_layout(root, available_space(space));
            assert_eq!(tree.layout(root).size.width, source.layout(source.parent(leaf).unwrap()).unwrap().size.width);
            assert_eq!(tree.layout(root).size.height, source.layout(source.parent(leaf).unwrap()).unwrap().size.height);
            let leaf = tree.children(root).next().unwrap();
            tree.patch_style(leaf, |style| style.size.height = c::Dimension::length(40.0));
            tree.compute_layout(root, available_space(space));
            let incremental = *tree.layout(root);
            tree.clear_layout();
            tree.compute_layout(root, available_space(space));
            assert_eq!(incremental, *tree.layout(root));
        }
    }

    #[test]
    fn converts_percentages_alignment_and_absolute_insets() {
        let result = style(&taffy::Style {
            position: taffy::Position::Absolute,
            size: Size { width: percent(0.5), height: auto() },
            inset: Rect { left: length(3.0), ..Rect::auto() },
            align_items: taffy::AlignItems::SAFE_CENTER,
            ..Default::default()
        });
        assert_eq!(result.display, c::Display::Flex);
        assert_eq!(result.position, c::Position::Absolute);
        assert_eq!(result.size.width, c::Dimension::percent(0.5));
        assert_eq!(result.inset.left, c::LengthPercentageAuto::length(3.0));
        assert_eq!(result.align_items, Some(c::AlignItems::SAFE_CENTER));
    }

    #[test]
    #[should_panic(expected = "named grid lines are not used")]
    fn rejects_unsupported_named_grid_lines() {
        let _ = placement(&t::GridPlacement::NamedLine("line".into(), 1));
    }
}
