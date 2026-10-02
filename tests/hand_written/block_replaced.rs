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

    fn ratio_box_style(content_box_ratio: bool) -> Style {
        Style {
            display: Display::Block,
            box_sizing: BoxSizing::BorderBox,
            size: Size { width: length(200.0), height: auto() },
            padding: Rect { left: length(10.0), right: length(10.0), top: length(10.0), bottom: length(10.0) },
            aspect_ratio: Some(2.0),
            aspect_ratio_content_box: content_box_ratio,
            ..Default::default()
        }
    }

    // Chrome: `aspect-ratio: 2 / 1` relates the authored border box.
    #[test]
    fn aspect_ratio_content_box_false_keeps_plain_ratio_border_box() {
        let mut tree: TaffyTree<()> = TaffyTree::new();
        let node = tree.new_leaf(ratio_box_style(false)).unwrap();
        tree.compute_layout(node, Size::MAX_CONTENT).unwrap();
        assert_eq!(tree.layout(node).unwrap().size, Size { width: 200.0, height: 100.0 });
    }

    // Chrome: `aspect-ratio: auto 2 / 1` relates the content box without changing
    // the border-box interpretation of the authored width, including for containers.
    #[test]
    fn aspect_ratio_content_box_true_keeps_size_styles_border_box() {
        for display in [Display::Block, Display::Flex, Display::Grid] {
            let mut tree: TaffyTree<()> = TaffyTree::new();
            let child = tree.new_leaf(Style::default()).unwrap();
            let root = tree.new_with_children(Style { display, ..ratio_box_style(true) }, &[child]).unwrap();
            tree.compute_layout(root, Size::MAX_CONTENT).unwrap();
            assert_eq!(tree.layout(root).unwrap().size, Size { width: 200.0, height: 110.0 }, "{display:?}");
        }
    }

    // The host selects an image's natural ratio, while Taffy applies it to the
    // content box. Parent item snapshots must retain that independent ratio box.
    #[test]
    fn aspect_ratio_content_box_natural_replaced_item() {
        for display in [Display::Block, Display::Flex, Display::Grid] {
            let mut tree = new_test_tree();
            let child = tree
                .new_leaf_with_context(
                    Style { item_is_replaced: true, ..ratio_box_style(true) },
                    TestNodeContext::aspect_ratio(100.0, 0.5),
                )
                .unwrap();
            let root = tree
                .new_with_children(
                    Style {
                        display,
                        size: Size { width: length(400.0), height: auto() },
                        align_items: Some(AlignItems::START),
                        ..Default::default()
                    },
                    &[child],
                )
                .unwrap();
            layout(&mut tree, root);
            assert_eq!(tree.layout(child).unwrap().size, Size { width: 200.0, height: 110.0 }, "{display:?}");
        }
    }
    #[test]
    fn invalid_flex_ratios_behave_as_auto() {
        for direction in [FlexDirection::Row, FlexDirection::Column] {
            for content_box in [false, true] {
                for invalid in [0.0, -1.0, f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
                    let mut results = Vec::new();
                    for aspect_ratio in [None, Some(invalid)] {
                        let mut tree: TaffyTree<()> = TaffyTree::new();
                        let child = tree
                            .new_leaf(Style {
                                size: Size { width: length(20.0), height: auto() },
                                aspect_ratio,
                                aspect_ratio_content_box: content_box,
                                ..Default::default()
                            })
                            .unwrap();
                        let root = tree
                            .new_with_children(
                                Style {
                                    display: Display::Flex,
                                    flex_direction: direction,
                                    size: Size { width: length(100.0), height: length(100.0) },
                                    ..Default::default()
                                },
                                &[child],
                            )
                            .unwrap();
                        tree.compute_layout(root, Size::MAX_CONTENT).unwrap();
                        results.push(tree.layout(child).unwrap().size);
                    }
                    assert_eq!(results[0], results[1], "ratio {invalid}, {direction:?}");
                }
            }
        }
    }
}
