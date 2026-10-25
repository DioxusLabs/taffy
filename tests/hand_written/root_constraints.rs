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
                    width: AvailableSpace::Definite(taffy_test_helpers::px(100.0)),
                    height: AvailableSpace::Definite(taffy_test_helpers::px(200.0)),
                },
            )
            .unwrap();
        let layout = taffy.layout(node).unwrap();

        assert_eq!(layout.size.width.to_f32_px(), 100.0);
        assert_eq!(layout.size.height.to_f32_px(), 200.0);
    }

    #[test]
    fn root_with_no_size() {
        let mut taffy = new_test_tree();
        let node = taffy.new_leaf(taffy::style::Style::default()).unwrap();

        taffy
            .compute_layout(
                node,
                taffy::geometry::Size {
                    width: AvailableSpace::Definite(taffy_test_helpers::px(100.0)),
                    height: AvailableSpace::Definite(taffy_test_helpers::px(100.0)),
                },
            )
            .unwrap();
        let layout = taffy.layout(node).unwrap();

        assert_eq!(layout.size.width.to_f32_px(), 0.0);
        assert_eq!(layout.size.height.to_f32_px(), 0.0);
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
                    width: AvailableSpace::Definite(taffy_test_helpers::px(100.0)),
                    height: AvailableSpace::Definite(taffy_test_helpers::px(100.0)),
                },
            )
            .unwrap();
        let layout = taffy.layout(node).unwrap();

        assert_eq!(layout.size.width.to_f32_px(), 200.0);
        assert_eq!(layout.size.height.to_f32_px(), 200.0);
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

        assert_eq!(layout.size.width.to_f32_px(), 40.0);
        assert_eq!(layout.size.height.to_f32_px(), 40.0);
    }
}
