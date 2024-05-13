#[cfg(test)]
mod root_constraints {
    use taffy::style_helpers::{length, TaffyMaxContent};
    use taffy::{AvailableSpace, Rect, Size, Style, TaffyTree};

    #[test]
    fn root_with_percentage_size() {
        let mut taffy: TaffyTree<()> = TaffyTree::new();
        let node = taffy
            .new_leaf(taffy::style::Style {
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Percent(1.0),
                    height: taffy::style::Dimension::Percent(1.0),
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
        let mut taffy: TaffyTree<()> = TaffyTree::new();
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
        let mut taffy: TaffyTree<()> = TaffyTree::new();
        let node = taffy
            .new_leaf(taffy::style::Style {
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Length(200.0),
                    height: taffy::style::Dimension::Length(200.0),
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
}
