#[cfg(test)]
mod baseline {
    use taffy::prelude::*;
    use taffy::{LayoutInput, LayoutOutput, Point};

    /// A node context that pairs an intrinsic size with a first-baseline offset
    /// (measured from the node's top edge in the block axis).
    #[derive(Debug, Clone, Copy)]
    struct BaselineContext {
        size: Size<f32>,
        baseline_y: f32,
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
            Size::ZERO,
            Point { x: None, y: Some(context.baseline_y) },
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
                BaselineContext { size: Size { width: 50.0, height: 50.0 }, baseline_y: 40.0 },
            )
            .unwrap();

        // Child B: 30x30 box with baseline 20px from the top
        let child_b = taffy
            .new_leaf_with_context(
                Style::default(),
                BaselineContext { size: Size { width: 30.0, height: 30.0 }, baseline_y: 20.0 },
            )
            .unwrap();

        let root = taffy
            .new_with_children(
                Style {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    align_items: Some(AlignItems::Baseline),
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

    /// Sanity-check: without `align-items: baseline`, items align at the cross-start edge
    /// regardless of the baseline reported by the measure function.
    #[test]
    fn flex_baseline_is_ignored_when_alignment_is_not_baseline() {
        let mut taffy: TaffyTree<BaselineContext> = TaffyTree::new();

        let child_a = taffy
            .new_leaf_with_context(
                Style::default(),
                BaselineContext { size: Size { width: 50.0, height: 50.0 }, baseline_y: 40.0 },
            )
            .unwrap();
        let child_b = taffy
            .new_leaf_with_context(
                Style::default(),
                BaselineContext { size: Size { width: 30.0, height: 30.0 }, baseline_y: 20.0 },
            )
            .unwrap();

        let root = taffy
            .new_with_children(
                Style {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    align_items: Some(AlignItems::FlexStart),
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
