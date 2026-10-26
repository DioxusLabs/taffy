//! Sizing of compressible replaced boxes that are not replaced elements
//! (`item_is_compressible_replaced: true`, e.g. form controls such as `<input>`).
//!
//! Like replaced elements, block-level form controls with `width: auto` take their intrinsic
//! width rather than being stretched to the container (css-sizing-3 "compressible replaced
//! elements"). Unlike replaced elements, an absolutely positioned form control with both insets
//! set *is* stretched between them (CSS2 §10.3.7 applies, not §10.3.8).
#[cfg(test)]
mod compressible_replaced {
    use taffy::prelude::*;
    use taffy::style::CoreStyle;
    use taffy::{Point, TaffyTree};
    use taffy_test_helpers::{new_test_tree, test_measure_function, TestNodeContext};

    // A 150x20 text input (height = width * 20/150)
    const INPUT: TestNodeContext = TestNodeContext::aspect_ratio(150.0, 20.0 / 150.0);

    fn layout_child(container: Style, child_style: Style) -> Layout {
        let mut taffy: TaffyTree<TestNodeContext> = new_test_tree();
        let child = taffy.new_leaf_with_context(child_style, INPUT).unwrap();
        let root = taffy.new_with_children(container, &[child]).unwrap();
        taffy.compute_layout_with_measure(root, Size::MAX_CONTENT, test_measure_function).unwrap();
        *taffy.layout(child).unwrap()
    }

    fn block_container() -> Style {
        Style { display: Display::Block, size: Size { width: length(400.0), height: auto() }, ..Default::default() }
    }

    fn positioned_container() -> Style {
        Style {
            display: Display::Block,
            position: Position::Relative,
            size: Size { width: length(400.0), height: length(300.0) },
            ..Default::default()
        }
    }

    fn all_insets(value: f32) -> Rect<LengthPercentageAuto> {
        Rect { left: length(value), right: length(value), top: length(value), bottom: length(value) }
    }

    #[test]
    fn block_level_auto_width_uses_intrinsic_size() {
        let layout = layout_child(
            block_container(),
            Style { display: Display::Block, item_is_compressible_replaced: true, ..Default::default() },
        );
        assert_eq!(layout.size, Size { width: 150.0, height: 20.0 });
    }

    #[test]
    fn block_level_justify_self_stretch_stretches() {
        let layout = layout_child(
            block_container(),
            Style {
                display: Display::Block,
                item_is_compressible_replaced: true,
                justify_self: Some(AlignItems::STRETCH),
                ..Default::default()
            },
        );
        assert_eq!(layout.size.width, 400.0);
    }

    #[test]
    fn absolute_auto_size_is_stretched_between_insets() {
        let layout = layout_child(
            positioned_container(),
            Style {
                position: Position::Absolute,
                item_is_compressible_replaced: true,
                inset: all_insets(10.0),
                ..Default::default()
            },
        );
        assert_eq!(layout.size, Size { width: 380.0, height: 280.0 });
        assert_eq!(layout.location, Point { x: 10.0, y: 10.0 });
    }

    /// A `width: 100px; max-width: 100%` item in a zero-width grid's `1fr` column.
    fn grid_item_width_with_percent_max_width(replaced: bool, compressible_replaced: bool) -> f32 {
        let container = Style {
            display: Display::Grid,
            size: Size { width: length(0.0), height: auto() },
            grid_template_columns: vec![fr(1.0)],
            ..Default::default()
        };
        let child = Style {
            item_is_replaced: replaced,
            item_is_compressible_replaced: compressible_replaced,
            size: Size { width: length(100.0), height: length(20.0) },
            min_size: Size { width: length(0.0), height: auto() },
            max_size: Size { width: percent(1.0), height: auto() },
            ..Default::default()
        };
        layout_child(container, child).size.width
    }

    #[test]
    fn grid_percent_max_width_does_not_compress_minimum_contribution() {
        assert_eq!(grid_item_width_with_percent_max_width(false, true), 100.0);
    }

    #[test]
    fn grid_percent_max_width_compresses_replaced_minimum_contribution() {
        assert_eq!(grid_item_width_with_percent_max_width(true, false), 0.0);
    }

    #[test]
    fn replaced_implies_compressible_replaced() {
        let style: Style = Style { item_is_replaced: true, ..Default::default() };
        assert!(CoreStyle::is_compressible_replaced(&style));
        assert!(CoreStyle::is_replaced(&style));
        let style: Style = Style { item_is_compressible_replaced: true, ..Default::default() };
        assert!(CoreStyle::is_compressible_replaced(&style));
        assert!(!CoreStyle::is_replaced(&style));
    }
}
