//! Tests for style inputs which are valid to construct, but which are degenerate or hostile
//! (non-finite numbers, extreme values, or internally inconsistent grid templates).
//!
//! These tests are primarily about layout *terminating* without panicking.
#[cfg(test)]
mod adversarial_styles {
    use taffy::prelude::*;
    use taffy::style::{GridTemplateComponent, MaxTrackSizingFunction, MinTrackSizingFunction, TrackSizingFunction};
    use taffy_test_helpers::new_test_tree;

    fn definite(size: f32) -> Size<AvailableSpace> {
        Size { width: AvailableSpace::Definite(size), height: AvailableSpace::Definite(size) }
    }

    /// A `NaN` flex factor makes every comparison in the "find the size of an fr" algorithm false,
    /// which used to prevent that algorithm from ever finding a valid hypothetical fr size.
    #[test]
    fn non_finite_flex_factor_terminates() {
        for flex_factor in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
            let mut tree = new_test_tree();
            let child = tree
                .new_leaf(Style {
                    size: Size { width: Dimension::from_length(10.0), height: Dimension::from_length(10.0) },
                    ..Default::default()
                })
                .unwrap();
            let root = tree
                .new_with_children(
                    Style {
                        display: Display::Grid,
                        grid_template_columns: vec![GridTemplateComponent::Single(TrackSizingFunction {
                            min: MinTrackSizingFunction::AUTO,
                            max: MaxTrackSizingFunction::from_fr(flex_factor),
                        })],
                        ..Default::default()
                    },
                    &[child],
                )
                .unwrap();

            tree.compute_layout(root, definite(100.0)).unwrap();
        }
    }

    /// A non-finite size on a grid container makes the space to fill non-finite, which used to
    /// prevent the "find the size of an fr" algorithm from terminating.
    #[test]
    fn non_finite_grid_container_size_terminates() {
        for width in [f32::NAN, f32::INFINITY] {
            let mut tree = new_test_tree();
            let child = tree.new_leaf(Style::default()).unwrap();
            let root = tree
                .new_with_children(
                    Style {
                        display: Display::Grid,
                        size: Size { width: Dimension::from_length(width), height: Dimension::from_length(100.0) },
                        grid_template_columns: vec![repeat(2, vec![fr(1.0)])],
                        ..Default::default()
                    },
                    &[child],
                )
                .unwrap();

            tree.compute_layout(root, definite(100.0)).unwrap();
        }
    }

    /// `repeat()` definitions are allowed to contain fewer line name sets than they have tracks
    /// (the `repeat` style helper creates repetitions with no line names at all). Line numbering
    /// must still account for the tracks that each repetition generates.
    #[test]
    fn repetition_without_line_names_resolves_lines() {
        let mut tree = new_test_tree();
        let child = tree
            .new_leaf(Style {
                size: Size { width: Dimension::from_length(5.0), height: Dimension::from_length(5.0) },
                grid_column: Line { start: GridPlacement::NamedLine("c".into(), 1), end: GridPlacement::Auto },
                ..Default::default()
            })
            .unwrap();
        let root = tree
            .new_with_children(
                Style {
                    display: Display::Grid,
                    size: Size { width: Dimension::from_length(100.0), height: Dimension::from_length(100.0) },
                    grid_template_columns: vec![repeat(2, vec![length(10.0)]), length(10.0)],
                    grid_template_column_names: vec![vec![], vec![], vec!["c".into()]],
                    ..Default::default()
                },
                &[child],
            )
            .unwrap();

        tree.compute_layout(root, definite(100.0)).unwrap();

        // "c" is the 4th line of the grid: the line after the three 10px columns
        assert_eq!(tree.layout(child).unwrap().location.x, 30.0);
    }
}
