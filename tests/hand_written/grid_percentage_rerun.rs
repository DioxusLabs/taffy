//! Percentage track sizing functions resolve against the grid container's size. When that size is
//! indefinite they are treated as auto while computing the container's size and then resolved against
//! that size. This must happen regardless of whether the available space is definite (e.g. a
//! shrink-to-fit grid container).
#[cfg(feature = "grid")]
mod grid_percentage_rerun {
    use taffy::prelude::*;
    use taffy_test_helpers::{new_test_tree, test_measure_function, TestNodeContext, WritingMode};

    /// Returns (container width, item width)
    fn layout_fit_content_percent_column(available_width: AvailableSpace) -> (f32, f32) {
        let mut tree = new_test_tree();
        let item = tree
            .new_leaf_with_context(
                Style::default(),
                TestNodeContext::ahem_text("HH\u{200b}HH\u{200b}HH".to_string(), WritingMode::Horizontal),
            )
            .unwrap();
        let root = tree
            .new_with_children(
                Style {
                    display: Display::Grid,
                    grid_template_columns: vec![fit_content(percent(0.5))].into(),
                    grid_template_rows: vec![length(40.0)].into(),
                    ..Default::default()
                },
                &[item],
            )
            .unwrap();
        tree.compute_layout_with_measure(
            root,
            Size { width: available_width, height: AvailableSpace::MaxContent },
            test_measure_function,
        )
        .unwrap();
        (tree.layout(root).unwrap().size.width, tree.layout(item).unwrap().size.width)
    }

    #[test]
    fn fit_content_percentage_indefinite_container_max_content_available_space() {
        assert_eq!(layout_fit_content_percent_column(AvailableSpace::MaxContent), (60.0, 30.0));
    }

    #[test]
    fn fit_content_percentage_indefinite_container_definite_available_space() {
        assert_eq!(layout_fit_content_percent_column(AvailableSpace::Definite(200.0)), (60.0, 30.0));
    }

    /// Returns (container height, row 1 height, row 2 height, row 3 height) for a grid with
    /// `grid-template-rows: auto 20% auto` whose items are 20px, 60px and 20px tall respectively.
    /// Mirrors WPT css/css-grid/layout-algorithm/grid-percent-rows-filled-shrinkwrap-001.html
    fn layout_percent_row_shrinkwrap(available_height: AvailableSpace) -> (f32, f32, f32, f32) {
        let mut tree: TaffyTree<()> = TaffyTree::new();
        let item = |tree: &mut TaffyTree<()>, height: f32| {
            tree.new_leaf(Style { size: Size { width: length(20.0), height: length(height) }, ..Default::default() })
                .unwrap()
        };
        let a = item(&mut tree, 20.0);
        let b = item(&mut tree, 60.0);
        let c = item(&mut tree, 20.0);
        let root = tree
            .new_with_children(
                Style {
                    display: Display::Grid,
                    grid_template_columns: vec![auto()].into(),
                    grid_template_rows: vec![auto(), percent(0.2), auto()].into(),
                    ..Default::default()
                },
                &[a, b, c],
            )
            .unwrap();
        tree.compute_layout(root, Size { width: AvailableSpace::Definite(200.0), height: available_height }).unwrap();
        let total = tree.layout(root).unwrap().size.height;
        let y = |node| tree.layout(node).unwrap().location.y;
        (total, y(b), y(c) - y(b), total - y(c))
    }

    /// Percentage rows must be re-resolved against the container's (first pass) height even when column sizing
    /// is not re-run. The auto rows then stretch to fill the remaining space.
    #[test]
    fn percentage_rows_indefinite_container_max_content_available_space() {
        assert_eq!(layout_percent_row_shrinkwrap(AvailableSpace::MaxContent), (100.0, 40.0, 20.0, 40.0));
    }

    #[test]
    fn percentage_rows_indefinite_container_definite_available_space() {
        assert_eq!(layout_percent_row_shrinkwrap(AvailableSpace::Definite(500.0)), (100.0, 40.0, 20.0, 40.0));
    }
}
