//! Baseline-aligned grid items contribute their baseline shim to the intrinsic size of their row,
//! including their minimum contribution (and its clamp to the grid area for fixed-max tracks).
//! Mirrors WPT `css/css-grid/layout-algorithm/grid-minimum-contribution-baseline-shim.html`.
#[cfg(feature = "grid")]
mod grid_baseline_shim {
    use taffy::prelude::*;
    use taffy::style::MaxTrackSizingFunction;
    use taffy::{Baselines, LayoutInput, LayoutOutput};

    /// A block-like leaf with vertical padding whose 25px-high content has a baseline
    /// `baseline_in_content` px below the content box top.
    #[derive(Debug, Clone, Copy)]
    struct Item {
        padding_top: f32,
        padding_bottom: f32,
        baseline_in_content: f32,
    }

    fn measure(inputs: LayoutInput, _node: NodeId, context: Option<&mut Item>, _style: &Style) -> LayoutOutput {
        let Some(item) = context else {
            return LayoutOutput::from_outer_size(inputs.known_dimensions.unwrap_or(Size::ZERO));
        };
        let intrinsic = Size { width: 25.0, height: item.padding_top + 25.0 + item.padding_bottom };
        let size = inputs.known_dimensions.unwrap_or(intrinsic);
        LayoutOutput::from_sizes_and_baselines(
            size,
            Rect::ZERO,
            Baselines::from_first(Some(item.padding_top + item.baseline_in_content)),
        )
    }

    /// The row size of a 0x0 grid with `grid-template-columns: 50px 50px`, `align-items: baseline`
    /// and `grid-template-rows: minmax(auto, <max>)`, read from an absolutely positioned child
    /// covering the row. Returns (row size, item1 y, item2 y).
    fn layout(max: MaxTrackSizingFunction) -> (f32, f32, f32) {
        let mut taffy: TaffyTree<Item> = TaffyTree::new();
        let item1 = taffy
            .new_leaf_with_context(
                Style { padding: Rect { top: length(25.0), ..Rect::zero() }, ..Default::default() },
                Item { padding_top: 25.0, padding_bottom: 0.0, baseline_in_content: 25.0 },
            )
            .unwrap();
        let item2 = taffy
            .new_leaf_with_context(
                Style { padding: Rect { bottom: length(25.0), ..Rect::zero() }, ..Default::default() },
                Item { padding_top: 0.0, padding_bottom: 25.0, baseline_in_content: 0.0 },
            )
            .unwrap();
        let area = taffy
            .new_leaf(Style {
                position: Position::Absolute,
                inset: Rect { left: length(0.0), right: length(0.0), top: length(0.0), bottom: length(0.0) },
                grid_column: Line { start: line(1), end: line(3) },
                grid_row: Line { start: line(1), end: line(2) },
                ..Default::default()
            })
            .unwrap();
        let grid = taffy
            .new_with_children(
                Style {
                    display: Display::Grid,
                    size: Size { width: length(0.0), height: length(0.0) },
                    grid_template_columns: vec![length(50.0), length(50.0)],
                    grid_template_rows: vec![minmax(MinTrackSizingFunction::auto(), max)],
                    align_items: AlignItems::BASELINE,
                    ..Default::default()
                },
                &[item1, item2, area],
            )
            .unwrap();
        taffy.compute_layout_with_measure(grid, Size::MAX_CONTENT, measure).unwrap();
        (
            taffy.layout(area).unwrap().size.height,
            taffy.layout(item1).unwrap().location.y,
            taffy.layout(item2).unwrap().location.y,
        )
    }

    #[test]
    fn minimum_contribution_includes_baseline_shim() {
        // item1's baseline is 50px below its top and item2's is at its top, so item2 is shimmed down by 50px.
        // With a fixed max track sizing function the content-based minimum size is clamped to the space left
        // in the grid area after the item's padding and shim (item1: 25px, item2: 25px + 50px shim = 75px).
        let maxes =
            [length(0.0), length(75.0), length(88.0), length(100.0), length(150.0), MaxTrackSizingFunction::auto()];
        let results = maxes.map(layout);
        assert_eq!(
            results,
            [
                (75.0, 0.0, 50.0),
                (75.0, 0.0, 50.0),
                (88.0, 0.0, 50.0),
                (100.0, 0.0, 50.0),
                (100.0, 0.0, 50.0),
                (100.0, 0.0, 50.0),
            ]
        );
    }
}
