#[cfg(test)]
mod caching {
    use taffy::prelude::*;
    use taffy_test_helpers::{new_test_tree, test_measure_function, TestNodeContext};

    const NODE_CONTEXT: TestNodeContext = TestNodeContext::fixed(50.0, 50.0);

    #[test]
    fn measure_count_flexbox() {
        let mut taffy = new_test_tree();

        let leaf = taffy.new_leaf_with_context(Style::default(), NODE_CONTEXT).unwrap();

        let mut node = taffy.new_with_children(Style::DEFAULT, &[leaf]).unwrap();
        for _ in 0..100 {
            node = taffy.new_with_children(Style::DEFAULT, &[node]).unwrap();
        }

        taffy.compute_layout_with_measure(node, Size::MAX_CONTENT, test_measure_function).unwrap();

        assert_eq!(taffy.get_node_context_mut(leaf).unwrap().count, 7);
    }

    #[test]
    #[cfg(feature = "grid")]
    fn measure_count_grid() {
        let mut taffy = new_test_tree();

        let style = || Style { display: Display::Grid, ..Default::default() };
        let leaf = taffy.new_leaf_with_context(style(), NODE_CONTEXT).unwrap();

        let mut node = taffy.new_with_children(Style::DEFAULT, &[leaf]).unwrap();
        for _ in 0..100 {
            node = taffy.new_with_children(Style::DEFAULT, &[node]).unwrap();
        }

        taffy.compute_layout_with_measure(node, Size::MAX_CONTENT, test_measure_function).unwrap();
        assert_eq!(taffy.get_node_context_mut(leaf).unwrap().count, 7);
    }

    /// A node's size measured for one axis must not be returned from the cache when the other
    /// axis is queried, as the size in the non-requested axis is not guaranteed to be valid.
    #[test]
    #[cfg(all(feature = "grid", feature = "block_layout"))]
    fn grid_block_item_height_not_polluted_by_width_measure() {
        let mut taffy = new_test_tree();

        let item_style = |row: i16| Style {
            display: Display::Block,
            grid_row: taffy::geometry::Line { start: line(row), end: line(row + 1) },
            grid_column: taffy::geometry::Line { start: line(1), end: line(2) },
            ..Default::default()
        };

        // Each grid item is a block container wrapping a measured leaf. The block layout
        // algorithm short-circuits width-only measures without computing a height, so a
        // subsequent height query must not reuse that cached result.
        let mut make_item = |row: i16| {
            let leaf = taffy.new_leaf_with_context(Style::default(), TestNodeContext::fixed(100.0, 19.0)).unwrap();
            taffy.new_with_children(item_style(row), &[leaf]).unwrap()
        };
        let a = make_item(1);
        let b = make_item(2);
        let c = make_item(3);

        let container = taffy
            .new_with_children(
                Style {
                    display: Display::Grid,
                    // The `auto` column is required to trigger intrinsic (width) sizing of the
                    // items before the row-sizing (height) pass.
                    grid_template_columns: vec![percent(0.6), auto()],
                    size: Size { width: length(1000.0), height: auto() },
                    ..Default::default()
                },
                &[a, b, c],
            )
            .unwrap();

        taffy
            .compute_layout_with_measure(
                container,
                Size { width: AvailableSpace::Definite(1000.0), height: AvailableSpace::Definite(1000.0) },
                test_measure_function,
            )
            .unwrap();

        for (index, node) in [a, b, c].into_iter().enumerate() {
            let layout = taffy.layout(node).unwrap();
            assert_eq!(layout.size.height, 19.0);
            assert_eq!(layout.location.y, 19.0 * index as f32);
        }
    }
}
