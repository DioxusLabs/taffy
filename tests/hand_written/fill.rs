//! The `fl` unit: a share of the free space of the container, for which the content of the box (or of the
//! items in the grid track) is never measured.
mod fill {
    use taffy::prelude::*;
    use taffy_test_helpers::{new_test_tree, test_measure_function, TestNodeContext};

    type Tree = TaffyTree<TestNodeContext>;

    /// A leaf whose content is 150px by 30px
    fn leaf(tree: &mut Tree, style: Style) -> NodeId {
        tree.new_leaf_with_context(style, TestNodeContext::fixed(150.0, 30.0)).unwrap()
    }

    fn layout(tree: &mut Tree, root: NodeId, available_space: Size<AvailableSpace>) {
        tree.compute_layout_with_measure(root, available_space, test_measure_function).unwrap();
    }

    fn width(tree: &Tree, node: NodeId) -> f32 {
        tree.layout(node).unwrap().size.width
    }

    /// The number of times that the content of a leaf has been measured. The final layout of a leaf counts as one.
    fn measure_count(tree: &Tree, node: NodeId) -> usize {
        tree.get_node_context(node).unwrap().count
    }

    #[cfg(feature = "grid")]
    mod grid {
        use super::*;

        fn grid(
            tree: &mut Tree,
            columns: Vec<GridTemplateComponent<String>>,
            style: Style,
            items: &[NodeId],
        ) -> NodeId {
            let style = Style {
                display: Display::Grid,
                grid_template_columns: columns,
                grid_template_rows: vec![length(40.0)],
                ..style
            };
            tree.new_with_children(style, items).unwrap()
        }

        #[test]
        fn tracks_share_free_space_without_measuring_items() {
            let mut tree = new_test_tree();
            let items: Vec<_> = (0..3).map(|_| leaf(&mut tree, Style::default())).collect();
            let style = Style { size: Size { width: length(400.0), height: auto() }, ..Default::default() };
            let root = grid(&mut tree, vec![length(100.0), fill(1.0), fill(2.0)], style, &items);
            layout(&mut tree, root, Size::MAX_CONTENT);

            // The 150px content of the item in the `1fl` track does not floor the track's size
            assert_eq!(items.iter().map(|item| width(&tree, *item)).collect::<Vec<_>>(), [100.0, 100.0, 200.0]);
            assert_eq!(items.iter().map(|item| measure_count(&tree, *item)).collect::<Vec<_>>(), [1, 1, 1]);
        }

        #[test]
        fn fr_tracks_are_floored_by_item_content() {
            let mut tree = new_test_tree();
            let items: Vec<_> = (0..3).map(|_| leaf(&mut tree, Style::default())).collect();
            let style = Style { size: Size { width: length(400.0), height: auto() }, ..Default::default() };
            let root = grid(&mut tree, vec![length(100.0), fr(1.0), fr(2.0)], style, &items);
            layout(&mut tree, root, Size::MAX_CONTENT);

            assert_eq!(items.iter().map(|item| width(&tree, *item)).collect::<Vec<_>>(), [100.0, 150.0, 150.0]);
        }

        #[test]
        fn tracks_mixed_with_fr_tracks_share_one_flex_fraction() {
            let mut tree = new_test_tree();
            let items: Vec<_> = (0..2).map(|_| leaf(&mut tree, Style::default())).collect();
            let style = Style { size: Size { width: length(400.0), height: auto() }, ..Default::default() };
            let root = grid(&mut tree, vec![fill(1.0), minmax(length(0.0), fr(3.0))], style, &items);
            layout(&mut tree, root, Size::MAX_CONTENT);

            assert_eq!(items.iter().map(|item| width(&tree, *item)).collect::<Vec<_>>(), [100.0, 300.0]);
        }

        #[test]
        fn tracks_are_zero_sized_in_an_indefinite_container() {
            let mut tree = new_test_tree();
            let items: Vec<_> = (0..2).map(|_| leaf(&mut tree, Style::default())).collect();
            let root = grid(&mut tree, vec![fill(1.0), auto()], Style::default(), &items);
            layout(&mut tree, root, Size::MAX_CONTENT);

            assert_eq!(width(&tree, root), 150.0);
            assert_eq!(items.iter().map(|item| width(&tree, *item)).collect::<Vec<_>>(), [0.0, 150.0]);
            assert_eq!(measure_count(&tree, items[0]), 1);
        }

        #[test]
        fn fr_tracks_fit_item_content_in_an_indefinite_container() {
            let mut tree = new_test_tree();
            let items: Vec<_> = (0..2).map(|_| leaf(&mut tree, Style::default())).collect();
            let root = grid(&mut tree, vec![minmax(length(0.0), fr(1.0)), auto()], Style::default(), &items);
            layout(&mut tree, root, Size::MAX_CONTENT);

            assert_eq!(items.iter().map(|item| width(&tree, *item)).collect::<Vec<_>>(), [150.0, 150.0]);
            assert!(measure_count(&tree, items[0]) > 1);
        }

        #[test]
        fn tracks_fill_the_minimum_size_of_an_indefinite_container() {
            let mut tree = new_test_tree();
            let items: Vec<_> = (0..2).map(|_| leaf(&mut tree, Style::default())).collect();
            let style = Style { min_size: Size { width: length(400.0), height: auto() }, ..Default::default() };
            let root = grid(&mut tree, vec![fill(1.0), auto()], style, &items);
            layout(&mut tree, root, Size::MAX_CONTENT);

            assert_eq!(items.iter().map(|item| width(&tree, *item)).collect::<Vec<_>>(), [250.0, 150.0]);
            assert_eq!(measure_count(&tree, items[0]), 1);
        }

        #[test]
        fn item_spanning_a_fill_track_has_no_content_based_minimum() {
            let mut tree = new_test_tree();
            let style = Style { grid_column: Line { start: line(1), end: span(2) }, ..Default::default() };
            let item = leaf(&mut tree, style);
            let style = Style { size: Size { width: length(100.0), height: auto() }, ..Default::default() };
            let root = grid(&mut tree, vec![minmax(auto(), length(20.0)), fill(1.0)], style, &[item]);
            layout(&mut tree, root, Size::MAX_CONTENT);

            assert_eq!(width(&tree, item), 100.0);
        }
    }

    #[cfg(feature = "flexbox")]
    mod flexbox {
        use super::*;

        fn row(tree: &mut Tree, style: Style, items: &[NodeId]) -> NodeId {
            tree.new_with_children(Style { display: Display::Flex, ..style }, items).unwrap()
        }

        fn fill_item(factor: f32) -> Style {
            Style { size: Size { width: fill(factor), height: auto() }, ..Default::default() }
        }

        fn width_400() -> Style {
            Style { size: Size { width: length(400.0), height: length(40.0) }, ..Default::default() }
        }

        fn widths(tree: &Tree, items: &[NodeId]) -> Vec<f32> {
            items.iter().map(|item| width(tree, *item)).collect()
        }

        #[test]
        fn items_share_free_space_without_being_measured() {
            let mut tree = new_test_tree();
            let fixed = Style { size: Size { width: length(100.0), height: auto() }, ..Default::default() };
            let items = [leaf(&mut tree, fixed), leaf(&mut tree, fill_item(1.0)), leaf(&mut tree, fill_item(2.0))];
            let root = row(&mut tree, width_400(), &items);
            layout(&mut tree, root, Size::MAX_CONTENT);

            // The 150px content of the `1fl` item is neither its flex basis nor its minimum size
            assert_eq!(widths(&tree, &items), [100.0, 100.0, 200.0]);
            // The `fl` items are measured for their cross size and for their final layout, but not (as the
            // fixed-size item is, for its automatic minimum size) for their main size
            assert_eq!(items.map(|item| measure_count(&tree, item)), [3, 2, 2]);
        }

        #[test]
        fn items_ignore_flex_properties() {
            let mut tree = new_test_tree();
            let style = Style { flex_grow: 5.0, flex_shrink: 5.0, flex_basis: length(300.0), ..fill_item(1.0) };
            let items = [leaf(&mut tree, style), leaf(&mut tree, fill_item(1.0))];
            let root = row(&mut tree, width_400(), &items);
            layout(&mut tree, root, Size::MAX_CONTENT);

            assert_eq!(widths(&tree, &items), [200.0, 200.0]);
        }

        #[test]
        fn items_leave_no_free_space_for_flex_grow() {
            let mut tree = new_test_tree();
            let grow = Style { flex_grow: 1.0, ..Default::default() };
            let items = [leaf(&mut tree, grow), leaf(&mut tree, fill_item(1.0))];
            let root = row(&mut tree, width_400(), &items);
            layout(&mut tree, root, Size::MAX_CONTENT);

            // The `flex-grow` item stays at its 150px content size
            assert_eq!(widths(&tree, &items), [150.0, 250.0]);
        }

        #[test]
        fn items_at_their_max_size_leave_free_space_for_flex_grow() {
            let mut tree = new_test_tree();
            let grow = Style { flex_grow: 1.0, ..Default::default() };
            let capped = Style { max_size: Size { width: length(50.0), height: auto() }, ..fill_item(1.0) };
            let items = [leaf(&mut tree, grow), leaf(&mut tree, capped)];
            let root = row(&mut tree, width_400(), &items);
            layout(&mut tree, root, Size::MAX_CONTENT);

            assert_eq!(widths(&tree, &items), [350.0, 50.0]);
        }

        #[test]
        fn min_and_max_sizes_redistribute_space_between_items() {
            let mut tree = new_test_tree();
            let floored = Style { min_size: Size { width: length(250.0), height: auto() }, ..fill_item(1.0) };
            let capped = Style { max_size: Size { width: length(30.0), height: auto() }, ..fill_item(1.0) };
            let items = [leaf(&mut tree, floored), leaf(&mut tree, capped), leaf(&mut tree, fill_item(1.0))];
            let root = row(&mut tree, width_400(), &items);
            layout(&mut tree, root, Size::MAX_CONTENT);

            assert_eq!(widths(&tree, &items), [250.0, 30.0, 120.0]);
        }

        #[test]
        fn factors_that_sum_to_less_than_one_leave_free_space() {
            let mut tree = new_test_tree();
            let items = [leaf(&mut tree, fill_item(0.25)), leaf(&mut tree, fill_item(0.25))];
            let root = row(&mut tree, width_400(), &items);
            layout(&mut tree, root, Size::MAX_CONTENT);

            assert_eq!(widths(&tree, &items), [100.0, 100.0]);
        }

        #[test]
        fn gaps_margins_and_padding_are_subtracted_from_the_free_space() {
            let mut tree = new_test_tree();
            let margin = Style { margin: Rect { left: length(20.0), ..Rect::zero() }, ..fill_item(1.0) };
            let padding = Style { padding: Rect { left: length(170.0), ..Rect::zero() }, ..fill_item(1.0) };
            let items = [leaf(&mut tree, margin), leaf(&mut tree, padding), leaf(&mut tree, fill_item(1.0))];
            let style = Style { gap: Size { width: length(10.0), height: zero() }, ..width_400() };
            let root = row(&mut tree, style, &items);
            layout(&mut tree, root, Size::MAX_CONTENT);

            // 400 - 2 * 10 (gaps) - 20 (margin) = 360. The padded item is floored at its 170px of padding,
            // and the other two share the remaining 190px.
            assert_eq!(widths(&tree, &items), [95.0, 170.0, 95.0]);
        }

        #[test]
        fn items_do_not_shrink_other_items_and_sit_at_their_minimum_on_overflow() {
            let mut tree = new_test_tree();
            let fixed = Style { size: Size { width: length(500.0), height: auto() }, ..Default::default() };
            let floored = Style { min_size: Size { width: length(20.0), height: auto() }, ..fill_item(1.0) };
            let items = [leaf(&mut tree, fixed), leaf(&mut tree, floored), leaf(&mut tree, fill_item(1.0))];
            let root = row(&mut tree, width_400(), &items);
            layout(&mut tree, root, Size::MAX_CONTENT);

            // The fixed item shrinks (`flex-shrink: 1`) by the 120px of overflow
            assert_eq!(widths(&tree, &items), [380.0, 20.0, 0.0]);
        }

        #[test]
        fn items_contribute_their_minimum_size_to_an_indefinite_container() {
            let mut tree = new_test_tree();
            let floored = Style { min_size: Size { width: length(20.0), height: auto() }, ..fill_item(1.0) };
            let items = [leaf(&mut tree, Style::default()), leaf(&mut tree, floored), leaf(&mut tree, fill_item(1.0))];
            let root = row(&mut tree, Style::default(), &items);
            layout(&mut tree, root, Size::MAX_CONTENT);

            assert_eq!(width(&tree, root), 170.0);
            assert_eq!(widths(&tree, &items), [150.0, 20.0, 0.0]);
            assert_eq!(measure_count(&tree, items[1]), 2);
            assert_eq!(measure_count(&tree, items[2]), 2);
        }

        #[test]
        fn items_collect_onto_lines_at_their_minimum_size_and_fill_each_line() {
            let mut tree = new_test_tree();
            let fixed = Style { size: Size { width: length(300.0), height: auto() }, ..Default::default() };
            let floored = Style { min_size: Size { width: length(120.0), height: auto() }, ..fill_item(1.0) };
            let items = [leaf(&mut tree, fixed), leaf(&mut tree, floored), leaf(&mut tree, fill_item(1.0))];
            let style = Style {
                flex_wrap: FlexWrap::Wrap,
                size: Size { width: length(400.0), height: auto() },
                ..Default::default()
            };
            let root = row(&mut tree, style, &items);
            layout(&mut tree, root, Size::MAX_CONTENT);

            // The first line holds only the fixed item. The two `fl` items share the second line.
            assert_eq!(widths(&tree, &items), [300.0, 200.0, 200.0]);
        }

        #[test]
        fn column_items_fill_the_height_of_the_container() {
            let mut tree = new_test_tree();
            let fill_height = Style { size: Size { width: auto(), height: fill(1.0) }, ..Default::default() };
            let items = [leaf(&mut tree, Style::default()), leaf(&mut tree, fill_height)];
            let style = Style {
                flex_direction: FlexDirection::Column,
                size: Size { width: length(400.0), height: length(100.0) },
                ..Default::default()
            };
            let root = row(&mut tree, style, &items);
            layout(&mut tree, root, Size::MAX_CONTENT);

            assert_eq!(items.map(|item| tree.layout(item).unwrap().size.height), [30.0, 70.0]);
        }
    }
}
