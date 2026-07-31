//! Sizing of replaced (`item_is_replaced: true`) children of block containers.
//!
//! Block layout stretch-sizes in-flow children to the container width, but block-level
//! replaced elements are exempt: with `width: auto` they use their intrinsic size
//! (<https://www.w3.org/TR/CSS22/visudet.html#block-replaced-width>).
#[cfg(test)]
mod block_replaced {
    use taffy::prelude::*;
    use taffy::TaffyTree;
    use taffy_test_helpers::{new_test_tree, test_measure_function, TestNodeContext};

    // A 142x20 badge-like image (height = width * 20/142)
    const BADGE: TestNodeContext = TestNodeContext::aspect_ratio(142.0, 20.0 / 142.0);

    fn block_container(width: f32) -> Style {
        Style { display: Display::Block, size: Size { width: length(width), height: auto() }, ..Default::default() }
    }

    fn layout(taffy: &mut TaffyTree<TestNodeContext>, root: NodeId) {
        taffy.compute_layout_with_measure(root, Size::MAX_CONTENT, test_measure_function).unwrap();
    }

    #[test]
    fn auto_width_uses_intrinsic_size() {
        let mut taffy = new_test_tree();
        let child = taffy
            .new_leaf_with_context(
                Style { display: Display::Block, item_is_replaced: true, ..Default::default() },
                BADGE,
            )
            .unwrap();
        let root = taffy.new_with_children(block_container(600.0), &[child]).unwrap();
        layout(&mut taffy, root);

        let child_layout = taffy.layout(child).unwrap();
        assert_eq!(child_layout.size.width, 142.0);
        assert_eq!(child_layout.size.height, 20.0);
    }

    #[test]
    fn non_replaced_child_is_stretch_sized() {
        let mut taffy = new_test_tree();
        let child =
            taffy.new_leaf_with_context(Style { display: Display::Block, ..Default::default() }, BADGE).unwrap();
        let root = taffy.new_with_children(block_container(600.0), &[child]).unwrap();
        layout(&mut taffy, root);

        assert_eq!(taffy.layout(child).unwrap().size.width, 600.0);
    }

    #[test]
    fn explicit_width_is_used() {
        let mut taffy = new_test_tree();
        let child = taffy
            .new_leaf_with_context(
                Style {
                    display: Display::Block,
                    item_is_replaced: true,
                    size: Size { width: length(300.0), height: length(50.0) },
                    ..Default::default()
                },
                BADGE,
            )
            .unwrap();
        let root = taffy.new_with_children(block_container(600.0), &[child]).unwrap();
        layout(&mut taffy, root);

        let child_layout = taffy.layout(child).unwrap();
        assert_eq!(child_layout.size.width, 300.0);
        assert_eq!(child_layout.size.height, 50.0);
    }

    #[test]
    fn max_width_clamps_intrinsic_size() {
        let mut taffy = new_test_tree();
        let child = taffy
            .new_leaf_with_context(
                Style {
                    display: Display::Block,
                    item_is_replaced: true,
                    max_size: Size { width: percent(1.0), height: auto() },
                    ..Default::default()
                },
                BADGE,
            )
            .unwrap();
        let root = taffy.new_with_children(block_container(100.0), &[child]).unwrap();
        layout(&mut taffy, root);

        let child_layout = taffy.layout(child).unwrap();
        assert_eq!(child_layout.size.width, 100.0);
    }

    #[test]
    fn auto_margins_center_replaced_child() {
        let mut taffy = new_test_tree();
        let child = taffy
            .new_leaf_with_context(
                Style {
                    display: Display::Block,
                    item_is_replaced: true,
                    margin: Rect { left: auto(), right: auto(), top: zero(), bottom: zero() },
                    ..Default::default()
                },
                BADGE,
            )
            .unwrap();
        let root = taffy.new_with_children(block_container(600.0), &[child]).unwrap();
        layout(&mut taffy, root);

        let child_layout = taffy.layout(child).unwrap();
        assert_eq!(child_layout.size.width, 142.0);
        assert_eq!(child_layout.location.x, (600.0 - 142.0) / 2.0);
    }
}
