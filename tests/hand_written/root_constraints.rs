#[cfg(test)]
mod root_constraints {
    use taffy::prelude::{FromLength, FromPercent};
    use taffy::style_helpers::{length, TaffyMaxContent};
    use taffy::{AvailableSpace, Rect, Size, Style, TaffyTree};
    use taffy_test_helpers::new_test_tree;

    #[test]
    fn root_with_percentage_size() {
        let mut taffy = new_test_tree();
        let node = taffy
            .new_leaf(taffy::style::Style {
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::from_percent(1.0),
                    height: taffy::style::Dimension::from_percent(1.0),
                },
                ..Default::default()
            })
            .unwrap();

        taffy
            .compute_layout(
                node,
                taffy::geometry::Size {
                    width: AvailableSpace::Definite(100.0),
                    height: AvailableSpace::Definite(200.0),
                },
            )
            .unwrap();
        let layout = taffy.layout(node).unwrap();

        assert_eq!(layout.size.width, 100.0);
        assert_eq!(layout.size.height, 200.0);
    }

    #[test]
    fn root_with_no_size() {
        let mut taffy = new_test_tree();
        let node = taffy.new_leaf(taffy::style::Style::default()).unwrap();

        taffy
            .compute_layout(
                node,
                taffy::geometry::Size {
                    width: AvailableSpace::Definite(100.0),
                    height: AvailableSpace::Definite(100.0),
                },
            )
            .unwrap();
        let layout = taffy.layout(node).unwrap();

        assert_eq!(layout.size.width, 0.0);
        assert_eq!(layout.size.height, 0.0);
    }

    #[test]
    fn root_with_larger_size() {
        let mut taffy = new_test_tree();
        let node = taffy
            .new_leaf(taffy::style::Style {
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::from_length(200.0),
                    height: taffy::style::Dimension::from_length(200.0),
                },
                ..Default::default()
            })
            .unwrap();

        taffy
            .compute_layout(
                node,
                taffy::geometry::Size {
                    width: AvailableSpace::Definite(100.0),
                    height: AvailableSpace::Definite(100.0),
                },
            )
            .unwrap();
        let layout = taffy.layout(node).unwrap();

        assert_eq!(layout.size.width, 200.0);
        assert_eq!(layout.size.height, 200.0);
    }

    #[test]
    fn root_padding_and_border_larger_than_definite_size() {
        let mut tree: TaffyTree<()> = TaffyTree::with_capacity(16);

        let child = tree.new_leaf(Style::default()).unwrap();

        let root = tree
            .new_with_children(
                Style {
                    size: Size { width: length(10.0), height: length(10.0) },
                    padding: Rect { left: length(10.0), right: length(10.0), top: length(10.0), bottom: length(10.0) },

                    border: Rect { left: length(10.0), right: length(10.0), top: length(10.0), bottom: length(10.0) },
                    ..Default::default()
                },
                &[child],
            )
            .unwrap();

        tree.compute_layout(root, Size::MAX_CONTENT).unwrap();

        let layout = tree.layout(root).unwrap();

        assert_eq!(layout.size.width, 40.0);
        assert_eq!(layout.size.height, 40.0);
    }

    /// The available space passed to a node is the space available to its border box, so the node's
    /// `min_size` and `max_size` limit the space that its content is measured under directly
    /// (without its margins being taken into account again).
    #[test]
    fn min_max_size_limits_border_box_available_space() {
        use taffy::style::{Display, LengthPercentageAuto};
        for (min_width, max_width, expected) in [(None, Some(50.0), 50.0), (Some(80.0), None, 80.0)] {
            let mut taffy: TaffyTree<()> = TaffyTree::new();
            let node = taffy
                .new_leaf_with_context(
                    Style {
                        display: Display::Flex,
                        margin: Rect::length(20.0).map(|m: taffy::LengthPercentage| LengthPercentageAuto::from(m)),
                        min_size: Size {
                            width: min_width.map_or(LengthPercentageAuto::auto(), LengthPercentageAuto::from_length),
                            height: LengthPercentageAuto::auto(),
                        },
                        max_size: Size {
                            width: max_width.map_or(LengthPercentageAuto::auto(), LengthPercentageAuto::from_length),
                            height: LengthPercentageAuto::auto(),
                        },
                        ..Default::default()
                    },
                    (),
                )
                .unwrap();
            let mut seen = None;
            taffy
                .compute_layout_with_measure(
                    node,
                    Size { width: AvailableSpace::Definite(100.0), height: AvailableSpace::MaxContent },
                    |inputs, _node, _ctx, style| {
                        taffy::compute_leaf_layout(
                            inputs,
                            style,
                            |_, _| 0.0,
                            |_known_dimensions, available_space| {
                                seen = Some(available_space.width);
                                Size { width: 200.0, height: 10.0 }
                            },
                        )
                    },
                )
                .unwrap();
            assert_eq!(seen, Some(AvailableSpace::Definite(expected)));
        }
    }
}
