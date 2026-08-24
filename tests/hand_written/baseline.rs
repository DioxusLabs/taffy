#[cfg(test)]
mod baseline {
    use taffy::prelude::*;
    use taffy::{Baselines, LayoutInput, LayoutOutput, Overflow, Point};

    /// A node context that pairs an intrinsic size with a first-baseline offset
    /// (measured from the node's top edge in the block axis).
    #[derive(Debug, Clone, Copy)]
    struct BaselineContext {
        size: Size<f32>,
        baseline_y: Option<f32>,
    }

    fn baseline_measure_function(
        _inputs: LayoutInput,
        _node_id: NodeId,
        context: Option<&mut BaselineContext>,
        _style: &Style,
    ) -> LayoutOutput {
        let Some(context) = context else { return LayoutOutput::DEFAULT };
        LayoutOutput::from_sizes_and_baselines(
            context.size,
            Rect::ZERO,
            Baselines::from_first(context.baseline_y.into()),
        )
    }

    /// Two flex items with different intrinsic baselines are aligned along their baselines
    /// when the container uses `align-items: baseline`. The item with the smaller distance
    /// from its top to its baseline should be shifted down so that both baselines coincide.
    #[test]
    fn flex_baseline_alignment_uses_measure_function_baseline() {
        let mut taffy: TaffyTree<BaselineContext> = TaffyTree::new();

        // Child A: 50x50 box with baseline 40px from the top
        let child_a = taffy
            .new_leaf_with_context(
                Style::default(),
                BaselineContext { size: Size { width: 50.0, height: 50.0 }, baseline_y: Some(40.0) },
            )
            .unwrap();

        // Child B: 30x30 box with baseline 20px from the top
        let child_b = taffy
            .new_leaf_with_context(
                Style::default(),
                BaselineContext { size: Size { width: 30.0, height: 30.0 }, baseline_y: Some(20.0) },
            )
            .unwrap();

        let root = taffy
            .new_with_children(
                Style {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    align_items: Some(AlignItems::BASELINE),
                    size: Size { width: length(200.0), height: length(100.0) },
                    ..Default::default()
                },
                &[child_a, child_b],
            )
            .unwrap();

        taffy.compute_layout_with_measure(root, Size::MAX_CONTENT, baseline_measure_function).unwrap();

        let layout_a = taffy.layout(child_a).unwrap();
        let layout_b = taffy.layout(child_b).unwrap();

        // Child A sets the max baseline (40 px from its top), so it sits at the top of the line.
        assert_eq!(layout_a.location.y, 0.0);
        // Child B is shifted down by (40 - 20) = 20 px so its baseline aligns with child A's.
        assert_eq!(layout_b.location.y, 20.0);

        // Sanity-check: both children's baselines now sit at y = 40 in the container.
        assert_eq!(layout_a.location.y + 40.0, layout_b.location.y + 20.0);
    }

    /// A block container child that is a scroll container and has no baseline of its own
    /// synthesizes a baseline from its border-box bottom edge, which propagates to the
    /// block container's first baseline.
    /// See <https://github.com/w3c/csswg-drafts/issues/7660>
    #[test]
    fn block_scroll_container_child_without_baseline_synthesizes_from_border_box() {
        let mut taffy: TaffyTree<BaselineContext> = TaffyTree::new();

        // An empty `overflow: hidden` block, 30x30
        let scroll_child = taffy
            .new_leaf_with_context(
                Style {
                    display: Display::Block,
                    overflow: Point { x: Overflow::Hidden, y: Overflow::Hidden },
                    size: Size { width: length(30.0), height: length(30.0) },
                    ..Default::default()
                },
                BaselineContext { size: Size { width: 30.0, height: 30.0 }, baseline_y: None },
            )
            .unwrap();

        // A block container wrapping the scroll container, with bottom padding so that the
        // synthesized baseline (30) differs from the wrapper's border-box bottom edge (50).
        let item_a = taffy
            .new_with_children(
                Style {
                    display: Display::Block,
                    size: Size { width: length(40.0), height: auto() },
                    padding: Rect { left: zero(), right: zero(), top: zero(), bottom: length(20.0) },
                    ..Default::default()
                },
                &[scroll_child],
            )
            .unwrap();

        // Reference item: 40x80 box with baseline 10px from the top
        let item_b = taffy
            .new_leaf_with_context(
                Style::default(),
                BaselineContext { size: Size { width: 40.0, height: 80.0 }, baseline_y: Some(10.0) },
            )
            .unwrap();

        let root = taffy
            .new_with_children(
                Style {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    align_items: Some(AlignItems::BASELINE),
                    size: Size { width: length(200.0), height: length(120.0) },
                    ..Default::default()
                },
                &[item_a, item_b],
            )
            .unwrap();

        taffy.compute_layout_with_measure(root, Size::MAX_CONTENT, baseline_measure_function).unwrap();

        // Item A's baseline is synthesized at the scroll container's bottom border edge (30),
        // not at item A's own bottom edge (50). Item A sets the max baseline, so it sits at
        // the top and item B is shifted down by (30 - 10) = 20px.
        assert_eq!(taffy.layout(item_a).unwrap().location.y, 0.0);
        assert_eq!(taffy.layout(item_b).unwrap().location.y, 20.0);
    }

    /// A block container child that is NOT a scroll container and has no baseline
    /// contributes no baseline to its block container parent.
    #[test]
    fn block_non_scroll_container_child_without_baseline_contributes_nothing() {
        let mut taffy: TaffyTree<BaselineContext> = TaffyTree::new();

        // An empty `overflow: visible` block, 30x30
        let plain_child = taffy
            .new_leaf_with_context(
                Style {
                    display: Display::Block,
                    size: Size { width: length(30.0), height: length(30.0) },
                    ..Default::default()
                },
                BaselineContext { size: Size { width: 30.0, height: 30.0 }, baseline_y: None },
            )
            .unwrap();

        let item_a = taffy
            .new_with_children(
                Style {
                    display: Display::Block,
                    size: Size { width: length(40.0), height: auto() },
                    padding: Rect { left: zero(), right: zero(), top: zero(), bottom: length(20.0) },
                    ..Default::default()
                },
                &[plain_child],
            )
            .unwrap();

        // Reference item: 40x80 box with baseline 10px from the top
        let item_b = taffy
            .new_leaf_with_context(
                Style::default(),
                BaselineContext { size: Size { width: 40.0, height: 80.0 }, baseline_y: Some(10.0) },
            )
            .unwrap();

        let root = taffy
            .new_with_children(
                Style {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    align_items: Some(AlignItems::BASELINE),
                    size: Size { width: length(200.0), height: length(120.0) },
                    ..Default::default()
                },
                &[item_a, item_b],
            )
            .unwrap();

        taffy.compute_layout_with_measure(root, Size::MAX_CONTENT, baseline_measure_function).unwrap();

        // Item A has no baseline, so the flexbox algorithm synthesizes one at its border-box
        // bottom edge (50). Item B is shifted down by (50 - 10) = 40px.
        assert_eq!(taffy.layout(item_a).unwrap().location.y, 0.0);
        assert_eq!(taffy.layout(item_b).unwrap().location.y, 40.0);
    }

    /// A block container child that is a scroll container contributes its content baseline
    /// clamped to its border box, so baselines of clipped content cannot leak below the
    /// child's border-box bottom edge.
    /// See <https://github.com/w3c/csswg-drafts/issues/7660>
    #[test]
    fn block_scroll_container_child_baseline_is_clamped_to_border_box() {
        let mut taffy: TaffyTree<BaselineContext> = TaffyTree::new();

        // Content with a baseline 48px from the top, inside a 30px tall scroll container
        let content = taffy
            .new_leaf_with_context(
                Style { display: Display::Block, ..Default::default() },
                BaselineContext { size: Size { width: 30.0, height: 60.0 }, baseline_y: Some(48.0) },
            )
            .unwrap();

        let scroll_child = taffy
            .new_with_children(
                Style {
                    display: Display::Block,
                    overflow: Point { x: Overflow::Hidden, y: Overflow::Hidden },
                    size: Size { width: length(30.0), height: length(30.0) },
                    ..Default::default()
                },
                &[content],
            )
            .unwrap();

        let item_a = taffy
            .new_with_children(
                Style {
                    display: Display::Block,
                    size: Size { width: length(40.0), height: auto() },
                    padding: Rect { left: zero(), right: zero(), top: zero(), bottom: length(20.0) },
                    ..Default::default()
                },
                &[scroll_child],
            )
            .unwrap();

        // Reference item: 40x80 box with baseline 10px from the top
        let item_b = taffy
            .new_leaf_with_context(
                Style::default(),
                BaselineContext { size: Size { width: 40.0, height: 80.0 }, baseline_y: Some(10.0) },
            )
            .unwrap();

        let root = taffy
            .new_with_children(
                Style {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    align_items: Some(AlignItems::BASELINE),
                    size: Size { width: length(200.0), height: length(120.0) },
                    ..Default::default()
                },
                &[item_a, item_b],
            )
            .unwrap();

        taffy.compute_layout_with_measure(root, Size::MAX_CONTENT, baseline_measure_function).unwrap();

        // The content baseline (48) is clamped to the scroll container's border-box height (30).
        // Item A's baseline is 30, so item B is shifted down by (30 - 10) = 20px.
        assert_eq!(taffy.layout(item_a).unwrap().location.y, 0.0);
        assert_eq!(taffy.layout(item_b).unwrap().location.y, 20.0);
    }

    /// Sanity-check: without `align-items: baseline`, items align at the cross-start edge
    /// regardless of the baseline reported by the measure function.
    #[test]
    fn flex_baseline_is_ignored_when_alignment_is_not_baseline() {
        let mut taffy: TaffyTree<BaselineContext> = TaffyTree::new();

        let child_a = taffy
            .new_leaf_with_context(
                Style::default(),
                BaselineContext { size: Size { width: 50.0, height: 50.0 }, baseline_y: Some(40.0) },
            )
            .unwrap();
        let child_b = taffy
            .new_leaf_with_context(
                Style::default(),
                BaselineContext { size: Size { width: 30.0, height: 30.0 }, baseline_y: Some(20.0) },
            )
            .unwrap();

        let root = taffy
            .new_with_children(
                Style {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    align_items: Some(AlignItems::FLEX_START),
                    size: Size { width: length(200.0), height: length(100.0) },
                    ..Default::default()
                },
                &[child_a, child_b],
            )
            .unwrap();

        taffy.compute_layout_with_measure(root, Size::MAX_CONTENT, baseline_measure_function).unwrap();

        assert_eq!(taffy.layout(child_a).unwrap().location.y, 0.0);
        assert_eq!(taffy.layout(child_b).unwrap().location.y, 0.0);
    }
}
