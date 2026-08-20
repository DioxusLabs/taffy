#[cfg(all(feature = "detailed_layout_info", feature = "grid"))]
mod detailed_grid_info {
    use taffy::prelude::*;
    use taffy::{DetailedGridInfo, DetailedLayoutInfo, Rect};

    fn detailed_grid_info(tree: &TaffyTree, node: NodeId) -> &DetailedGridInfo {
        match tree.detailed_layout_info(node) {
            DetailedLayoutInfo::Grid(info) => info,
            _ => panic!("expected detailed grid info"),
        }
    }

    #[test]
    fn item_grid_area() {
        let mut tree: TaffyTree<()> = TaffyTree::new();
        let child_a = tree.new_leaf(Style::default()).unwrap();
        let child_b = tree
            .new_leaf(Style {
                grid_row: line(2),
                grid_column: Line { start: line(1), end: line(3) },
                ..Default::default()
            })
            .unwrap();
        let container = tree
            .new_with_children(
                Style {
                    display: Display::Grid,
                    grid_template_columns: vec![length(40.0), length(60.0)],
                    grid_template_rows: vec![length(50.0), length(30.0)],
                    gap: length(10.0),
                    padding: Rect { left: length(5.0), right: length(5.0), top: length(5.0), bottom: length(5.0) },
                    ..Default::default()
                },
                &[child_a, child_b],
            )
            .unwrap();

        tree.compute_layout(container, Size::MAX_CONTENT).unwrap();
        let info = detailed_grid_info(&tree, container);

        // child_a is auto-placed into row 1 / column 1
        assert_eq!(info.item_grid_area(0), Some(Rect { left: 5.0, right: 45.0, top: 5.0, bottom: 55.0 }));
        // child_b spans both columns in row 2
        assert_eq!(info.item_grid_area(1), Some(Rect { left: 5.0, right: 115.0, top: 65.0, bottom: 95.0 }));
        // out of bounds index
        assert_eq!(info.item_grid_area(2), None);
    }
}
