//! Sizing of absolutely positioned replaced (`item_is_replaced: true`) boxes.
//!
//! An `auto` width/height of an absolutely positioned replaced element resolves to its
//! intrinsic size rather than being stretched between its insets
//! (<https://www.w3.org/TR/CSS22/visudet.html#abs-replaced-width>,
//! <https://www.w3.org/TR/CSS22/visudet.html#abs-replaced-height>).
#[cfg(test)]
mod absolute_replaced {
    use taffy::prelude::*;
    use taffy::{Point, TaffyTree};
    use taffy_test_helpers::{new_test_tree, test_measure_function, TestNodeContext};

    // A 100x50 image (height = width * 0.5)
    const IMAGE: TestNodeContext = TestNodeContext::aspect_ratio(100.0, 0.5);

    fn container() -> Style {
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

    fn layout_child(child_style: Style) -> Layout {
        let mut taffy: TaffyTree<TestNodeContext> = new_test_tree();
        let child = taffy.new_leaf_with_context(child_style, IMAGE).unwrap();
        let root = taffy.new_with_children(container(), &[child]).unwrap();
        taffy.compute_layout_with_measure(root, Size::MAX_CONTENT, test_measure_function).unwrap();
        *taffy.layout(child).unwrap()
    }

    #[test]
    fn auto_size_uses_intrinsic_size_despite_insets() {
        let layout = layout_child(Style {
            position: Position::Absolute,
            item_is_replaced: true,
            inset: all_insets(10.0),
            ..Default::default()
        });
        assert_eq!(layout.size, Size { width: 100.0, height: 50.0 });
        assert_eq!(layout.location, Point { x: 10.0, y: 10.0 });
    }

    #[test]
    fn auto_height_uses_aspect_ratio_despite_insets() {
        let layout = layout_child(Style {
            position: Position::Absolute,
            item_is_replaced: true,
            inset: all_insets(10.0),
            size: Size { width: length(200.0), height: auto() },
            ..Default::default()
        });
        assert_eq!(layout.size, Size { width: 200.0, height: 100.0 });
    }

    #[test]
    fn non_replaced_auto_size_is_stretched_between_insets() {
        let layout =
            layout_child(Style { position: Position::Absolute, inset: all_insets(10.0), ..Default::default() });
        assert_eq!(layout.size, Size { width: 380.0, height: 280.0 });
    }
}
