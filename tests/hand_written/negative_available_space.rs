#[cfg(test)]
mod negative_available_space {
    use taffy::prelude::*;
    use taffy::{AvailableSpace, LayoutInput, LayoutOutput, NodeId};
    use taffy_test_helpers::{new_test_tree, test_measure_function, TestNodeContext};

    /// Lays out a 20px wide container of the given display type with a single leaf whose
    /// horizontal margins (40px) exceed the container width. Returns the leaf's final width
    /// and every `LayoutInput` with a negative known dimension or available space that reached
    /// the leaf's measure function.
    fn layout_with_oversized_margins(display: Display, flex_direction: FlexDirection) -> (f32, Vec<String>) {
        let mut taffy = new_test_tree();
        let leaf = taffy
            .new_leaf_with_context(
                Style {
                    margin: Rect {
                        left: LengthPercentageAuto::length(30.0),
                        right: LengthPercentageAuto::length(10.0),
                        top: zero(),
                        bottom: zero(),
                    },
                    size: Size { width: auto(), height: auto() },
                    ..Default::default()
                },
                TestNodeContext::zero(),
            )
            .unwrap();
        let root = taffy
            .new_with_children(
                Style {
                    display,
                    flex_direction,
                    size: Size { width: length(20.0), height: auto() },
                    grid_template_columns: vec![length(20.0)],
                    ..Default::default()
                },
                &[leaf],
            )
            .unwrap();

        let mut negative_inputs = Vec::new();
        taffy
            .compute_layout_with_measure(
                root,
                Size::MAX_CONTENT,
                |inputs: LayoutInput, node_id: NodeId, context: Option<&mut TestNodeContext>, style: &Style| {
                    let known = inputs.known_dimensions;
                    let avail = inputs.available_space;
                    let is_negative = |value: Option<f32>| value.is_some_and(|v| v < 0.0);
                    let is_negative_avail =
                        |value: AvailableSpace| matches!(value, AvailableSpace::Definite(v) if v < 0.0);
                    if is_negative(known.width)
                        || is_negative(known.height)
                        || is_negative_avail(avail.width)
                        || is_negative_avail(avail.height)
                    {
                        negative_inputs.push(format!("known={known:?} available={avail:?}"));
                    }
                    let output: LayoutOutput = test_measure_function(inputs, node_id, context, style);
                    output
                },
            )
            .unwrap();

        (taffy.layout(leaf).unwrap().size.width, negative_inputs)
    }

    fn assert_floored_at_zero(display: Display, flex_direction: FlexDirection) {
        let (leaf_width, negative_inputs) = layout_with_oversized_margins(display, flex_direction);
        assert!(negative_inputs.is_empty(), "leaf received negative layout inputs: {negative_inputs:?}");
        assert_eq!(leaf_width, 0.0);
    }

    #[test]
    fn block_child_margins_exceed_container_width() {
        assert_floored_at_zero(Display::Block, FlexDirection::Row);
    }

    #[test]
    fn flex_column_child_margins_exceed_container_width() {
        assert_floored_at_zero(Display::Flex, FlexDirection::Column);
    }

    #[test]
    fn grid_child_margins_exceed_container_width() {
        assert_floored_at_zero(Display::Grid, FlexDirection::Row);
    }
}
