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
}
