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
                    grid_template_columns: vec![fit_content(percent(0.5))],
                    grid_template_rows: vec![length(40.0)],
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
}
