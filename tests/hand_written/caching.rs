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

        assert_eq!(taffy.get_node_context_mut(leaf).unwrap().count, 6);
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
        assert_eq!(taffy.get_node_context_mut(leaf).unwrap().count, 6);
    }

    /// A grid container that is only asked for its width (e.g. because it is an item of another
    /// grid whose columns are being sized) must not size its rows, as that measures the height of
    /// all of its items and, recursively, of everything below them.
    #[test]
    #[cfg(feature = "grid")]
    fn measure_count_nested_grid() {
        let mut taffy = new_test_tree();

        let style = || Style { display: Display::Grid, ..Default::default() };
        // The leaf's min-content width (20), max-content width (80) and final width (50) all differ, so
        // measuring its height at each of the first two widths can never be served from the cache.
        let text = TestNodeContext::ahem_text(
            "HH\u{200B}HH\u{200B}HH\u{200B}HH".to_string(),
            taffy_test_helpers::WritingMode::Horizontal,
        );
        let leaf = taffy.new_leaf_with_context(Style::default(), text).unwrap();
        let inner = taffy.new_with_children(style(), &[leaf]).unwrap();
        let middle = taffy.new_with_children(style(), &[inner]).unwrap();
        let outer = taffy
            .new_with_children(Style { size: Size { width: length(50.0), height: auto() }, ..style() }, &[middle])
            .unwrap();

        taffy.compute_layout_with_measure(outer, Size::MAX_CONTENT, test_measure_function).unwrap();
        assert_eq!(taffy.layout(leaf).unwrap().size, Size { width: 50.0, height: 20.0 });
        assert_eq!(taffy.get_node_context_mut(leaf).unwrap().count, 6);
    }

    /// A grid item whose size in the axis being measured is already known (here: a definite width) does not
    /// need to be measured to determine its contribution to the size of the tracks in that axis.
    /// (Without that short-circuit the leaf is measured 5 times)
    #[test]
    #[cfg(feature = "grid")]
    fn measure_count_grid_item_with_known_width() {
        let mut taffy = new_test_tree();

        let text = TestNodeContext::ahem_text(
            "HH\u{200B}HH\u{200B}HH\u{200B}HH".to_string(),
            taffy_test_helpers::WritingMode::Horizontal,
        );
        let leaf = taffy
            .new_leaf_with_context(
                Style { size: Size { width: length(40.0), height: auto() }, ..Default::default() },
                text,
            )
            .unwrap();
        let grid = taffy.new_with_children(Style { display: Display::Grid, ..Default::default() }, &[leaf]).unwrap();

        taffy.compute_layout_with_measure(grid, Size::MAX_CONTENT, test_measure_function).unwrap();
        assert_eq!(taffy.layout(grid).unwrap().size, Size { width: 40.0, height: 20.0 });
        assert_eq!(taffy.layout(leaf).unwrap().size, Size { width: 40.0, height: 20.0 });
        assert_eq!(taffy.get_node_context_mut(leaf).unwrap().count, 3);
    }

    /// A grid item in a track that is not intrinsically sized does not contribute to the size of that track, so
    /// it does not need to be measured when sizing the tracks (even if another track in the axis is intrinsic).
    /// (Without that short-circuit the leaf is measured 5 times)
    #[test]
    #[cfg(feature = "grid")]
    fn measure_count_grid_item_in_fixed_track() {
        let mut taffy = new_test_tree();

        let text = TestNodeContext::ahem_text(
            "HH\u{200B}HH\u{200B}HH\u{200B}HH".to_string(),
            taffy_test_helpers::WritingMode::Horizontal,
        );
        let leaf = taffy
            .new_leaf_with_context(
                Style { justify_self: Some(JustifySelf::START), grid_column: line(1), ..Default::default() },
                text,
            )
            .unwrap();
        let sibling = taffy
            .new_leaf(Style {
                size: Size { width: length(50.0), height: length(50.0) },
                grid_column: line(2),
                ..Default::default()
            })
            .unwrap();
        let grid = taffy
            .new_with_children(
                Style {
                    display: Display::Grid,
                    grid_template_columns: vec![length(100.0), auto()],
                    ..Default::default()
                },
                &[leaf, sibling],
            )
            .unwrap();

        taffy.compute_layout_with_measure(grid, Size::MAX_CONTENT, test_measure_function).unwrap();
        assert_eq!(taffy.layout(grid).unwrap().size, Size { width: 150.0, height: 50.0 });
        assert_eq!(taffy.layout(leaf).unwrap().size, Size { width: 80.0, height: 50.0 });
        assert_eq!(taffy.get_node_context_mut(leaf).unwrap().count, 3);
    }

    /// A flex container that is only asked for its main size (e.g. because it is an item of a grid
    /// whose tracks are being sized) must not measure the cross size of its items.
    /// (Without that short-circuit the leaf is measured 13 times in the row case and 12 times in the column case)
    #[test]
    #[cfg(all(feature = "grid", feature = "flexbox"))]
    fn measure_count_flex_in_grid() {
        for (flex_direction, expected_count) in [(FlexDirection::Row, 9), (FlexDirection::Column, 10)] {
            let mut taffy = new_test_tree();

            let text = TestNodeContext::ahem_text(
                "HH\u{200B}HH\u{200B}HH\u{200B}HH".to_string(),
                taffy_test_helpers::WritingMode::Horizontal,
            );
            let leaf = taffy.new_leaf_with_context(Style::default(), text).unwrap();
            let flex = taffy
                .new_with_children(Style { display: Display::Flex, flex_direction, ..Default::default() }, &[leaf])
                .unwrap();
            let grid = taffy
                .new_with_children(
                    Style {
                        display: Display::Grid,
                        size: Size { width: length(50.0), height: auto() },
                        ..Default::default()
                    },
                    &[flex],
                )
                .unwrap();

            taffy.compute_layout_with_measure(grid, Size::MAX_CONTENT, test_measure_function).unwrap();
            let count = taffy.get_node_context_mut(leaf).unwrap().count;
            assert_eq!(taffy.layout(leaf).unwrap().size, Size { width: 50.0, height: 20.0 });
            assert_eq!(count, expected_count);
        }
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
    /// An item whose spanned tracks cannot receive its min-/max-content contribution in any
    /// intrinsic track sizing step (definite min, flexible max) must not be measured for it.
    #[test]
    #[cfg(feature = "grid")]
    fn grid_item_spanning_only_unaffected_tracks_is_not_measured() {
        let mut taffy = new_test_tree();

        let text = TestNodeContext::ahem_text("HH HH HH HH".to_string(), taffy_test_helpers::WritingMode::Horizontal);
        let leaf_style =
            Style { grid_column: taffy::geometry::Line { start: line(2), end: line(3) }, ..Default::default() };
        let leaf = taffy.new_leaf_with_context(leaf_style, text).unwrap();
        let grid = taffy
            .new_with_children(
                Style {
                    display: Display::Grid,
                    size: Size { width: length(400.0), height: length(50.0) },
                    grid_template_columns: vec![length(100.0), minmax(length(0.0), fr(1.0))],
                    grid_template_rows: vec![length(50.0)],
                    ..Default::default()
                },
                &[leaf],
            )
            .unwrap();

        taffy.compute_layout_with_measure(grid, Size::MAX_CONTENT, test_measure_function).unwrap();

        // Only the final layout pass measures the leaf; track sizing does not.
        assert_eq!(taffy.layout(leaf).unwrap().size.width, 300.0);
        assert_eq!(taffy.get_node_context_mut(leaf).unwrap().count, 1);
    }
}
