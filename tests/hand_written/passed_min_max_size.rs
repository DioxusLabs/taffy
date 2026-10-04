#[cfg(test)]
mod passed_min_max_size {
    use taffy::prelude::*;
    use taffy::{BoxSizing, LayoutInput, NodeId, Position};
    use taffy_test_helpers::{new_test_tree, test_measure_function, TestNodeContext};

    /// A style with a min and a max size in both axes. With `box-sizing: content-box` and 5px of padding on
    /// every side, the border-box sizes that it resolves to are `EXPECTED_MIN_SIZE` and `EXPECTED_MAX_SIZE`.
    fn bounded_style(position: Position) -> Style {
        Style {
            position,
            box_sizing: BoxSizing::ContentBox,
            padding: Rect::length(5.0),
            min_size: Size { width: length(10.0), height: length(20.0) },
            max_size: Size { width: length(150.0), height: length(300.0) },
            ..Default::default()
        }
    }
    const EXPECTED_MIN_SIZE: Size<Option<f32>> = Size { width: Some(20.0), height: Some(30.0) };
    const EXPECTED_MAX_SIZE: Size<Option<f32>> = Size { width: Some(160.0), height: Some(310.0) };

    /// The `min_size` and `max_size` of a `LayoutInput`
    type MinMaxSize = (Size<Option<f32>>, Size<Option<f32>>);

    /// Lays out `root` and returns the min and max sizes of every `LayoutInput` that reached `node`
    fn passed_min_max_sizes(taffy: &mut TaffyTree<TestNodeContext>, root: NodeId, node: NodeId) -> Vec<MinMaxSize> {
        let mut passed = Vec::new();
        taffy
            .compute_layout_with_measure(
                root,
                Size { width: AvailableSpace::Definite(400.0), height: AvailableSpace::Definite(400.0) },
                |inputs: LayoutInput, node_id: NodeId, context: Option<&mut TestNodeContext>, style: &Style| {
                    if node_id == node {
                        passed.push((inputs.min_size, inputs.max_size));
                    }
                    test_measure_function(inputs, node_id, context, style)
                },
            )
            .unwrap();
        passed
    }

    fn assert_child_is_passed_its_min_max_size(container_style: Style, position: Position) {
        let mut taffy = new_test_tree();
        let child = taffy.new_leaf_with_context(bounded_style(position), TestNodeContext::zero()).unwrap();
        let root = taffy
            .new_with_children(
                Style { size: Size { width: length(200.0), height: length(100.0) }, ..container_style },
                &[child],
            )
            .unwrap();

        let passed = passed_min_max_sizes(&mut taffy, root, child);
        assert!(!passed.is_empty(), "the child was never laid out");
        for (min_size, max_size) in passed {
            assert_eq!(min_size, EXPECTED_MIN_SIZE);
            assert_eq!(max_size, EXPECTED_MAX_SIZE);
        }
    }

    fn assert_children_are_passed_their_min_max_size(container_style: Style) {
        assert_child_is_passed_its_min_max_size(container_style.clone(), Position::Relative);
        assert_child_is_passed_its_min_max_size(container_style, Position::Absolute);
    }

    #[test]
    fn block_children() {
        assert_children_are_passed_their_min_max_size(Style { display: Display::Block, ..Default::default() });
    }

    #[test]
    fn flex_row_children() {
        assert_children_are_passed_their_min_max_size(Style {
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            ..Default::default()
        });
        // With baseline alignment and wrapping, which measure the children through other code paths
        assert_children_are_passed_their_min_max_size(Style {
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            flex_wrap: FlexWrap::Wrap,
            align_items: Some(AlignItems::BASELINE),
            ..Default::default()
        });
    }

    #[test]
    fn flex_column_children() {
        assert_children_are_passed_their_min_max_size(Style {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            ..Default::default()
        });
    }

    #[test]
    fn grid_children() {
        assert_children_are_passed_their_min_max_size(Style { display: Display::Grid, ..Default::default() });
        // With definite tracks, in which the child's size is fully determined by stretch alignment
        assert_children_are_passed_their_min_max_size(Style {
            display: Display::Grid,
            grid_template_columns: vec![length(100.0)],
            grid_template_rows: vec![length(50.0)],
            ..Default::default()
        });
        assert_children_are_passed_their_min_max_size(Style {
            display: Display::Grid,
            align_items: Some(AlignItems::BASELINE),
            ..Default::default()
        });
    }

    #[test]
    fn root() {
        let mut taffy = new_test_tree();
        let root = taffy.new_leaf_with_context(bounded_style(Position::Relative), TestNodeContext::zero()).unwrap();
        let passed = passed_min_max_sizes(&mut taffy, root, root);
        assert!(!passed.is_empty(), "the root was never laid out");
        for (min_size, max_size) in passed {
            assert_eq!(min_size, EXPECTED_MIN_SIZE);
            assert_eq!(max_size, EXPECTED_MAX_SIZE);
        }
    }
}
