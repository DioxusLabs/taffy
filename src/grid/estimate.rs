/// This module is not required for spec compliance, but is used as a performance optimisation
/// to reduce the number of allocations required when creating a grid.
use crate::geometry::{Line, Size};
use crate::node::Node;
use crate::style::{GridPlacement, Style};
use crate::tree::LayoutTree;
use std::cmp::{max, min};

/// Estimate the number of rows and columns in the grid
/// This is used as a performance optimisation to pre-size vectors and reduce allocations
///
/// Note that this function internally mixes use of grid track numbers and grid line numbers
pub(super) fn compute_grid_size_estimate(tree: &impl LayoutTree, node: Node) -> Size<(u16, u16, u16)> {
    // Initialise estimates with explicit track lengths (flooring at 1)
    let style = tree.style(node);
    let explicit_col_estimate = max(style.grid_template_columns.len(), 1) as u16;
    let explicit_row_estimate = max(style.grid_template_rows.len(), 1) as u16;

    // Iterate over children, producing an estimate of the min and max grid *lines*
    // along with the span of each itme
    let (mut col_min, mut col_max, mut col_max_span, mut row_min, mut row_max, mut row_max_span) = (0, 0, 0, 0, 0, 0);
    tree.children(node).into_iter().copied().map(|child_node| tree.style(child_node)).for_each(
        |child_style: &Style| {
            // Note: that the children reference the lines in between (and around) the tracks not tracks themselves,
            // and thus we must subtract 1 to get an accurate estimate of the number of tracks
            let (child_col_min, child_col_max, child_col_span) = child_min_line_max_line_span(child_style.grid_column);
            let (child_row_min, child_row_max, child_row_span) = child_min_line_max_line_span(child_style.grid_row);
            col_min = min(col_min, child_col_min);
            col_max = max(col_max, child_col_max);
            col_max_span = max(col_max_span, child_col_span);
            row_min = min(row_min, child_row_min);
            row_max = max(row_max, child_row_max);
            row_max_span = max(row_max_span, child_row_span);
        },
    );

    // The units of these variables are *track* counts
    let negative_implicit_inline_track_estimate =
        min((explicit_col_estimate + 1) as i16 + min(col_min, 0) - 1, 0).abs() as u16;
    let explicit_inline_track_estimate = explicit_col_estimate;
    let mut positive_implicit_inline_track_estimate =
        max(explicit_col_estimate, max(col_max - 1, 1) as u16) - explicit_col_estimate;
    let negative_implicit_block_track_estimate =
        min((explicit_row_estimate + 1) as i16 + min(row_min, 0) - 1, 0).abs() as u16;
    let explicit_block_track_estimate = explicit_row_estimate;
    let mut positive_implicit_block_track_estimate =
        max(explicit_row_estimate, max(row_max - 1, 1) as u16) - explicit_row_estimate;

    // In each axis, adjust positive track estimate if any items have a span that does not fit within
    // the total number of tracks in the estimate
    if negative_implicit_inline_track_estimate
        + explicit_inline_track_estimate
        + positive_implicit_inline_track_estimate
        < row_max_span
    {
        positive_implicit_inline_track_estimate =
            row_max_span - explicit_inline_track_estimate - negative_implicit_inline_track_estimate;
    }
    if negative_implicit_block_track_estimate + explicit_block_track_estimate + positive_implicit_block_track_estimate
        < row_max_span
    {
        positive_implicit_block_track_estimate =
            row_max_span - explicit_block_track_estimate - negative_implicit_block_track_estimate;
    }

    Size {
        width: (
            negative_implicit_inline_track_estimate,
            explicit_inline_track_estimate,
            positive_implicit_inline_track_estimate,
        ),
        height: (
            negative_implicit_block_track_estimate,
            explicit_block_track_estimate,
            positive_implicit_block_track_estimate,
        ),
    }
}

/// Helper function for `compute_grid_size_estimate`
/// Produces a conservative estimate of the greatest and smallest grid lines used by a single grid item
///
/// Values are returned in CSS grid line index coordinates where positive values are relative to the first
/// explicit grid line and negative values are relative to the last explicit grid line
#[inline]
fn child_min_line_max_line_span(line: Line<GridPlacement>) -> (i16, i16, u16) {
    use GridPlacement::*;

    // 8.3.1. Grid Placement Conflict Handling
    // A. If the placement for a grid item contains two lines, and the start line is further end-ward than the end line, swap the two lines.
    // B. If the start line is equal to the end line, remove the end line.
    // C. If the placement contains two spans, remove the one contributed by the end grid-placement property.
    // D. If the placement contains only a span for a named line, replace it with a span of 1.

    let min = match (line.start, line.end) {
        // Both tracks specified
        (Track(track1), Track(track2)) => {
            // See rules A and B above
            if track1 == track2 {
                track1
            } else {
                min(track1, track2)
            }
        }

        // Start track specified
        (Track(track), Auto) => track,
        (Track(track), Span(_)) => track,

        // End track specified
        (Auto, Track(track)) => track,
        (Span(span), Track(track)) => track - (span as i16),

        // Only spans or autos
        // We ignore spans here by returning 1 which never effect the estimate as these are accounted for separately
        (Auto | Span(_), Auto | Span(_)) => 1,
    };

    let max = match (line.start, line.end) {
        // Both tracks specified
        (Track(track1), Track(track2)) => {
            // See rules A and B above
            if track1 == track2 {
                track1 + 1
            } else {
                max(track1, track2)
            }
        }

        // Start track specified
        (Track(track), Auto) => track + 1,
        (Track(track), Span(span)) => track + (span as i16),

        // End track specified
        (Auto, Track(track)) => track,
        (Span(_), Track(track)) => track,

        // Only spans or autos
        // We ignore spans here by returning 1 which never effect the estimate as these are accounted for separately
        (Auto | Span(_), Auto | Span(_)) => 1,
    };

    let span = line.span();

    (min, max, span)
}

#[allow(clippy::bool_assert_comparison)]
#[cfg(test)]
mod tests {
    mod test_child_min_max_line {
        use super::super::child_min_line_max_line_span;
        use crate::geometry::Line;
        use crate::style::GridPlacement::*;

        #[test]
        fn child_min_max_line_auto() {
            let (min_col, max_col, span) = child_min_line_max_line_span(Line { start: Track(5), end: Span(6) });
            assert_eq!(min_col, 5);
            assert_eq!(max_col, 11);
            assert_eq!(span, 6);
        }

        #[test]
        fn child_min_max_line_negative_track() {
            let (min_col, max_col, span) = child_min_line_max_line_span(Line { start: Track(-5), end: Span(3) });
            assert_eq!(min_col, -5);
            assert_eq!(max_col, -2);
            assert_eq!(span, 3);
        }
    }
}
