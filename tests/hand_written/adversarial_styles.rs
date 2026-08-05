//! Tests for style inputs which are valid to construct, but which are degenerate or hostile
//! (non-finite numbers, extreme values, or internally inconsistent grid templates).
//!
//! These tests are primarily about layout *terminating* without panicking.
#[cfg(test)]
mod adversarial_styles {
    use taffy::prelude::*;
    use taffy::style::{
        Direction, GridTemplateComponent, MaxTrackSizingFunction, MinTrackSizingFunction, TrackSizingFunction,
    };
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

    /// A zero span (an invalid value which gets normalized to 1) combined with an unresolvable
    /// named span used to size the implicit grid estimate to zero tracks, while auto-placement
    /// resolved the same placement to a span of 1. The auto-placement search then looped forever
    /// looking for a position that could never fit.
    #[test]
    fn zero_span_and_named_span_terminates() {
        for direction in [Direction::Ltr, Direction::Rtl] {
            let mut tree = new_test_tree();
            let child = tree
                .new_leaf(Style {
                    grid_column: Line {
                        start: GridPlacement::NamedSpan("does-not-exist".into(), 1),
                        end: GridPlacement::from_span(0),
                    },
                    ..Default::default()
                })
                .unwrap();
            let root = tree
                .new_with_children(Style { display: Display::Grid, direction, ..Default::default() }, &[child])
                .unwrap();

            tree.compute_layout(root, definite(100.0)).unwrap();
        }
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
