#[cfg(test)]
mod order {
    use taffy::prelude::*;
    use taffy_test_helpers::new_test_tree;

    #[cfg(feature = "flexbox")]
    mod flex {
        use super::*;

        #[test]
        fn flex_order_reorders_items() {
            let mut taffy = new_test_tree();

            // Three children with order: 2, 0, 1
            // Visual order should be: child_b (0), child_c (1), child_a (2)
            let child_a = taffy
                .new_leaf(Style {
                    size: Size { width: Dimension::from_length(50.0), height: Dimension::from_length(50.0) },
                    order: 2,
                    ..Default::default()
                })
                .unwrap();

            let child_b = taffy
                .new_leaf(Style {
                    size: Size { width: Dimension::from_length(50.0), height: Dimension::from_length(50.0) },
                    order: 0,
                    ..Default::default()
                })
                .unwrap();

            let child_c = taffy
                .new_leaf(Style {
                    size: Size { width: Dimension::from_length(50.0), height: Dimension::from_length(50.0) },
                    order: 1,
                    ..Default::default()
                })
                .unwrap();

            let container = taffy
                .new_with_children(
                    Style {
                        display: Display::Flex,
                        flex_direction: FlexDirection::Row,
                        size: Size { width: Dimension::from_length(300.0), height: Dimension::from_length(100.0) },
                        ..Default::default()
                    },
                    &[child_a, child_b, child_c],
                )
                .unwrap();

            taffy
                .compute_layout(
                    container,
                    Size { width: AvailableSpace::Definite(300.0), height: AvailableSpace::Definite(100.0) },
                )
                .unwrap();

            // child_b (order: 0) should be first (x=0)
            // child_c (order: 1) should be second (x=50)
            // child_a (order: 2) should be third (x=100)
            let layout_a = taffy.layout(child_a).unwrap();
            let layout_b = taffy.layout(child_b).unwrap();
            let layout_c = taffy.layout(child_c).unwrap();

            assert_eq!(layout_b.location.x, 0.0, "child_b (order: 0) should be at x=0");
            assert_eq!(layout_c.location.x, 50.0, "child_c (order: 1) should be at x=50");
            assert_eq!(layout_a.location.x, 100.0, "child_a (order: 2) should be at x=100");

            // Layout.order reflects visual position
            assert_eq!(layout_b.order, 0, "child_b should have visual order 0");
            assert_eq!(layout_c.order, 1, "child_c should have visual order 1");
            assert_eq!(layout_a.order, 2, "child_a should have visual order 2");
        }

        #[test]
        fn flex_order_preserves_source_order_for_equal_values() {
            let mut taffy = new_test_tree();

            // All children have default order (0) — should maintain source order
            let child_a = taffy
                .new_leaf(Style {
                    size: Size { width: Dimension::from_length(50.0), height: Dimension::from_length(50.0) },
                    ..Default::default()
                })
                .unwrap();

            let child_b = taffy
                .new_leaf(Style {
                    size: Size { width: Dimension::from_length(50.0), height: Dimension::from_length(50.0) },
                    ..Default::default()
                })
                .unwrap();

            let child_c = taffy
                .new_leaf(Style {
                    size: Size { width: Dimension::from_length(50.0), height: Dimension::from_length(50.0) },
                    ..Default::default()
                })
                .unwrap();

            let container = taffy
                .new_with_children(
                    Style {
                        display: Display::Flex,
                        flex_direction: FlexDirection::Row,
                        size: Size { width: Dimension::from_length(300.0), height: Dimension::from_length(100.0) },
                        ..Default::default()
                    },
                    &[child_a, child_b, child_c],
                )
                .unwrap();

            taffy
                .compute_layout(
                    container,
                    Size { width: AvailableSpace::Definite(300.0), height: AvailableSpace::Definite(100.0) },
                )
                .unwrap();

            // Source order preserved: a=0, b=50, c=100
            assert_eq!(taffy.layout(child_a).unwrap().location.x, 0.0);
            assert_eq!(taffy.layout(child_b).unwrap().location.x, 50.0);
            assert_eq!(taffy.layout(child_c).unwrap().location.x, 100.0);
        }

        #[test]
        fn flex_order_mixed_positive_negative() {
            let mut taffy = new_test_tree();

            // Items with order: 1, -2, 0, -1, 3
            // Expected visual order: -2, -1, 0, 1, 3
            let children: Vec<_> = [1, -2, 0, -1, 3]
                .iter()
                .map(|&ord| {
                    taffy
                        .new_leaf(Style {
                            size: Size { width: Dimension::from_length(20.0), height: Dimension::from_length(20.0) },
                            order: ord,
                            ..Default::default()
                        })
                        .unwrap()
                })
                .collect();

            let container = taffy
                .new_with_children(
                    Style {
                        display: Display::Flex,
                        flex_direction: FlexDirection::Row,
                        size: Size { width: Dimension::from_length(200.0), height: Dimension::from_length(50.0) },
                        ..Default::default()
                    },
                    &children,
                )
                .unwrap();

            taffy
                .compute_layout(
                    container,
                    Size { width: AvailableSpace::Definite(200.0), height: AvailableSpace::Definite(50.0) },
                )
                .unwrap();

            // Visual order: children[1](-2), children[3](-1), children[2](0), children[0](1), children[4](3)
            assert_eq!(taffy.layout(children[1]).unwrap().location.x, 0.0, "order -2 at x=0");
            assert_eq!(taffy.layout(children[3]).unwrap().location.x, 20.0, "order -1 at x=20");
            assert_eq!(taffy.layout(children[2]).unwrap().location.x, 40.0, "order 0 at x=40");
            assert_eq!(taffy.layout(children[0]).unwrap().location.x, 60.0, "order 1 at x=60");
            assert_eq!(taffy.layout(children[4]).unwrap().location.x, 80.0, "order 3 at x=80");
        }

        #[test]
        fn flex_order_negative_values() {
            let mut taffy = new_test_tree();

            let child_a = taffy
                .new_leaf(Style {
                    size: Size { width: Dimension::from_length(50.0), height: Dimension::from_length(50.0) },
                    order: 0,
                    ..Default::default()
                })
                .unwrap();

            let child_b = taffy
                .new_leaf(Style {
                    size: Size { width: Dimension::from_length(50.0), height: Dimension::from_length(50.0) },
                    order: -1,
                    ..Default::default()
                })
                .unwrap();

            let container = taffy
                .new_with_children(
                    Style {
                        display: Display::Flex,
                        flex_direction: FlexDirection::Row,
                        size: Size { width: Dimension::from_length(300.0), height: Dimension::from_length(100.0) },
                        ..Default::default()
                    },
                    &[child_a, child_b],
                )
                .unwrap();

            taffy
                .compute_layout(
                    container,
                    Size { width: AvailableSpace::Definite(300.0), height: AvailableSpace::Definite(100.0) },
                )
                .unwrap();

            // child_b (order: -1) comes before child_a (order: 0)
            assert_eq!(taffy.layout(child_b).unwrap().location.x, 0.0, "child_b (order: -1) should be first");
            assert_eq!(taffy.layout(child_a).unwrap().location.x, 50.0, "child_a (order: 0) should be second");
        }
    }

    #[cfg(feature = "grid")]
    mod grid {
        use super::*;

        #[test]
        fn grid_order_reorders_auto_placed_items() {
            let mut taffy = new_test_tree();

            // Three auto-placed children with order: 2, 0, 1
            // Grid auto-placement should place them in order-modified document order
            let child_a = taffy
                .new_leaf(Style {
                    size: Size { width: Dimension::from_length(50.0), height: Dimension::from_length(50.0) },
                    order: 2,
                    ..Default::default()
                })
                .unwrap();

            let child_b = taffy
                .new_leaf(Style {
                    size: Size { width: Dimension::from_length(50.0), height: Dimension::from_length(50.0) },
                    order: 0,
                    ..Default::default()
                })
                .unwrap();

            let child_c = taffy
                .new_leaf(Style {
                    size: Size { width: Dimension::from_length(50.0), height: Dimension::from_length(50.0) },
                    order: 1,
                    ..Default::default()
                })
                .unwrap();

            let container = taffy
                .new_with_children(
                    Style {
                        display: Display::Grid,
                        grid_template_columns: vec![
                            GridTemplateComponent::from_length(50.0),
                            GridTemplateComponent::from_length(50.0),
                            GridTemplateComponent::from_length(50.0),
                        ],
                        size: Size { width: Dimension::from_length(150.0), height: Dimension::AUTO },
                        ..Default::default()
                    },
                    &[child_a, child_b, child_c],
                )
                .unwrap();

            taffy
                .compute_layout(
                    container,
                    Size { width: AvailableSpace::Definite(150.0), height: AvailableSpace::Definite(200.0) },
                )
                .unwrap();

            // Auto-placement in order-modified document order:
            // child_b (order: 0) -> column 1 (x=0)
            // child_c (order: 1) -> column 2 (x=50)
            // child_a (order: 2) -> column 3 (x=100)
            let layout_a = taffy.layout(child_a).unwrap();
            let layout_b = taffy.layout(child_b).unwrap();
            let layout_c = taffy.layout(child_c).unwrap();

            assert_eq!(layout_b.location.x, 0.0, "child_b (order: 0) should be in column 1");
            assert_eq!(layout_c.location.x, 50.0, "child_c (order: 1) should be in column 2");
            assert_eq!(layout_a.location.x, 100.0, "child_a (order: 2) should be in column 3");

            // Layout.order reflects CSS order-modified visual position
            assert_eq!(layout_b.order, 0, "child_b should have visual order 0");
            assert_eq!(layout_c.order, 1, "child_c should have visual order 1");
            assert_eq!(layout_a.order, 2, "child_a should have visual order 2");
        }

        #[test]
        fn grid_order_preserves_source_order_for_equal_values() {
            let mut taffy = new_test_tree();

            // Three auto-placed children with same order (0) — source order must be preserved
            let child_a = taffy
                .new_leaf(Style {
                    size: Size { width: Dimension::from_length(50.0), height: Dimension::from_length(50.0) },
                    ..Default::default()
                })
                .unwrap();

            let child_b = taffy
                .new_leaf(Style {
                    size: Size { width: Dimension::from_length(50.0), height: Dimension::from_length(50.0) },
                    ..Default::default()
                })
                .unwrap();

            let child_c = taffy
                .new_leaf(Style {
                    size: Size { width: Dimension::from_length(50.0), height: Dimension::from_length(50.0) },
                    ..Default::default()
                })
                .unwrap();

            let container = taffy
                .new_with_children(
                    Style {
                        display: Display::Grid,
                        grid_template_columns: vec![
                            GridTemplateComponent::from_length(50.0),
                            GridTemplateComponent::from_length(50.0),
                            GridTemplateComponent::from_length(50.0),
                        ],
                        size: Size { width: Dimension::from_length(150.0), height: Dimension::AUTO },
                        ..Default::default()
                    },
                    &[child_a, child_b, child_c],
                )
                .unwrap();

            taffy
                .compute_layout(
                    container,
                    Size { width: AvailableSpace::Definite(150.0), height: AvailableSpace::Definite(200.0) },
                )
                .unwrap();

            // Source order preserved: a=col1, b=col2, c=col3
            assert_eq!(taffy.layout(child_a).unwrap().location.x, 0.0);
            assert_eq!(taffy.layout(child_b).unwrap().location.x, 50.0);
            assert_eq!(taffy.layout(child_c).unwrap().location.x, 100.0);
        }
    }
}
