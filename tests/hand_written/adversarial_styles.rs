//! Tests for style inputs which are valid to construct, but which are degenerate or hostile
//! (non-finite numbers, extreme values, or internally inconsistent grid templates).
//!
//! These tests are primarily about layout *terminating* without panicking.
#[cfg(test)]
mod adversarial_styles {
    use taffy::prelude::*;
    use taffy_test_helpers::new_test_tree;

    fn definite(size: f32) -> Size<AvailableSpace> {
        Size { width: AvailableSpace::Definite(size), height: AvailableSpace::Definite(size) }
    }

    /// When auto-placement collides with an occupied area, the search cursor must jump past the
    /// whole occupied interval. Jumping only within the queried area used to make the search
    /// advance one track at a time through large occupied areas, so a small item searching a grid
    /// occupied by a maximum-size item scanned (10000 tracks)^2 candidate positions.
    #[test]
    fn small_item_in_max_size_occupied_grid_terminates() {
        let mut tree = new_test_tree();
        let big = tree
            .new_leaf(Style {
                grid_row: Line { start: GridPlacement::from_span(10000), end: GridPlacement::Auto },
                grid_column: Line { start: GridPlacement::from_span(10000), end: GridPlacement::Auto },
                ..Default::default()
            })
            .unwrap();
        let small = tree
            .new_leaf(Style {
                grid_row: Line { start: GridPlacement::from_span(10000), end: GridPlacement::Auto },
                grid_column: Line { start: GridPlacement::from_span(1), end: GridPlacement::Auto },
                ..Default::default()
            })
            .unwrap();
        let root = tree
            .new_with_children(
                Style {
                    display: Display::Grid,
                    size: Size { width: Dimension::from_length(100.0), height: Dimension::from_length(100.0) },
                    ..Default::default()
                },
                &[big, small],
            )
            .unwrap();

        tree.compute_layout(root, definite(100.0)).unwrap();
    }

    /// Items may span thousands of tracks and be placed thousands of tracks outside of the explicit
    /// grid. Auto-placing such items must not take an excessive amount of time.
    #[test]
    fn extreme_placements_and_spans_terminate() {
        let mut tree = new_test_tree();
        let children: Vec<_> = [
            Line { start: GridPlacement::Auto, end: GridPlacement::from_span(2000) },
            Line { start: GridPlacement::from_line_index(-2000), end: GridPlacement::from_line_index(2000) },
        ]
        .into_iter()
        .map(|placement| {
            tree.new_leaf(Style { grid_row: placement.clone(), grid_column: placement, ..Default::default() }).unwrap()
        })
        .collect();
        let root = tree
            .new_with_children(
                Style {
                    display: Display::Grid,
                    size: Size { width: Dimension::from_length(100.0), height: Dimension::from_length(100.0) },
                    ..Default::default()
                },
                &children,
            )
            .unwrap();

        tree.compute_layout(root, definite(100.0)).unwrap();
    }
}
