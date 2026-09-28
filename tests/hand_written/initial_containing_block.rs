//! Tests for the initial containing block: boxes with no nearer containing block are positioned
//! against a viewport-sized rectangle at the canvas origin, not against the root element's box.
#[cfg(test)]
mod initial_containing_block {
    use taffy::prelude::*;
    use taffy::Point;

    const VIEWPORT: Size<AvailableSpace> =
        Size { width: AvailableSpace::Definite(800.0), height: AvailableSpace::Definite(600.0) };

    fn root_style() -> Style {
        Style {
            display: Display::Block,
            margin: Rect { left: length(10.0), right: length(10.0), top: length(20.0), bottom: length(20.0) },
            border: Rect { left: length(5.0), right: length(5.0), top: length(5.0), bottom: length(5.0) },
            ..Default::default()
        }
    }

    fn oof_style(position: Position, inset: Rect<LengthPercentageAuto>, size: Size<Dimension>) -> Style {
        Style { position, inset, size, ..Default::default() }
    }

    #[test]
    fn root_is_placed_at_its_margin() {
        let mut tree: TaffyTree<()> = TaffyTree::new();
        let root = tree.new_leaf(root_style()).unwrap();
        tree.compute_layout(root, VIEWPORT).unwrap();

        let layout = tree.layout(root).unwrap();
        assert_eq!(layout.location, Point { x: 10.0, y: 20.0 });
        assert_eq!(layout.size.width, 780.0);
    }

    #[test]
    fn inset_zero_fills_viewport() {
        for position in [Position::Absolute, Position::Fixed] {
            let mut tree: TaffyTree<()> = TaffyTree::new();
            let oof = tree.new_leaf(oof_style(position, Rect::length(0.0), Size::auto())).unwrap();
            let root = tree.new_with_children(root_style(), &[oof]).unwrap();
            tree.compute_layout(root, VIEWPORT).unwrap();

            // Location is relative to the root's border box, which sits at (10, 20) in the viewport
            let layout = tree.layout(oof).unwrap();
            assert_eq!(layout.location, Point { x: -10.0, y: -20.0 }, "{position:?}");
            assert_eq!(layout.size, Size { width: 800.0, height: 600.0 }, "{position:?}");
        }
    }

    #[test]
    fn bottom_right_insets_resolve_against_viewport() {
        for position in [Position::Absolute, Position::Fixed] {
            let mut tree: TaffyTree<()> = TaffyTree::new();
            let oof = tree
                .new_leaf(oof_style(
                    position,
                    Rect { left: auto(), top: auto(), right: length(0.0), bottom: length(0.0) },
                    Size { width: length(50.0), height: length(50.0) },
                ))
                .unwrap();
            // The root has no in-flow content, so its border box is much shorter than the viewport
            let root = tree.new_with_children(root_style(), &[oof]).unwrap();
            tree.compute_layout(root, VIEWPORT).unwrap();

            let layout = tree.layout(oof).unwrap();
            assert_eq!(layout.location, Point { x: 800.0 - 50.0 - 10.0, y: 600.0 - 50.0 - 20.0 }, "{position:?}");
        }
    }

    #[test]
    fn static_position_is_unaffected() {
        let mut tree: TaffyTree<()> = TaffyTree::new();
        let oof = tree
            .new_leaf(oof_style(Position::Absolute, Rect::auto(), Size { width: length(50.0), height: length(50.0) }))
            .unwrap();
        let root = tree.new_with_children(root_style(), &[oof]).unwrap();
        tree.compute_layout(root, VIEWPORT).unwrap();

        assert_eq!(tree.layout(oof).unwrap().location, Point { x: 5.0, y: 5.0 });
    }

    fn positioned_root_style(position: Position, inset: Rect<LengthPercentageAuto>, size: Size<Dimension>) -> Style {
        Style { display: Display::Block, position, inset, size, border: Rect::length(10.0), ..Default::default() }
    }

    #[test]
    fn out_of_flow_root_is_positioned_by_left_top_insets() {
        for position in [Position::Absolute, Position::Fixed] {
            let mut tree: TaffyTree<()> = TaffyTree::new();
            let root = tree
                .new_leaf(positioned_root_style(
                    position,
                    Rect { left: length(100.0), top: percent(0.5), right: auto(), bottom: auto() },
                    Size { width: length(100.0), height: length(100.0) },
                ))
                .unwrap();
            tree.compute_layout(root, VIEWPORT).unwrap();

            let layout = tree.layout(root).unwrap();
            assert_eq!(layout.location, Point { x: 100.0, y: 300.0 }, "{position:?}");
            assert_eq!(layout.size, Size { width: 100.0, height: 100.0 }, "{position:?}");
        }
    }

    #[test]
    fn out_of_flow_root_is_positioned_by_right_bottom_insets() {
        let mut tree: TaffyTree<()> = TaffyTree::new();
        let root = tree
            .new_leaf(positioned_root_style(
                Position::Absolute,
                Rect { left: auto(), top: auto(), right: length(10.0), bottom: length(20.0) },
                Size { width: length(100.0), height: length(100.0) },
            ))
            .unwrap();
        tree.compute_layout(root, VIEWPORT).unwrap();

        let layout = tree.layout(root).unwrap();
        assert_eq!(layout.location, Point { x: 800.0 - 100.0 - 10.0, y: 600.0 - 100.0 - 20.0 });
    }

    #[test]
    fn out_of_flow_root_is_sized_by_opposing_insets() {
        let mut tree: TaffyTree<()> = TaffyTree::new();
        let root = tree.new_leaf(positioned_root_style(Position::Absolute, Rect::length(50.0), Size::auto())).unwrap();
        tree.compute_layout(root, VIEWPORT).unwrap();

        let layout = tree.layout(root).unwrap();
        assert_eq!(layout.location, Point { x: 50.0, y: 50.0 });
        assert_eq!(layout.size, Size { width: 700.0, height: 500.0 });
    }

    #[test]
    fn out_of_flow_root_with_auto_insets_is_shrink_to_fit_at_static_position() {
        let mut tree: TaffyTree<()> = TaffyTree::new();
        let child = tree
            .new_leaf(Style { size: Size { width: length(30.0), height: length(40.0) }, ..Default::default() })
            .unwrap();
        let root = tree
            .new_with_children(positioned_root_style(Position::Absolute, Rect::auto(), Size::auto()), &[child])
            .unwrap();
        tree.compute_layout(root, VIEWPORT).unwrap();

        let layout = tree.layout(root).unwrap();
        assert_eq!(layout.location, Point { x: 0.0, y: 0.0 });
        assert_eq!(layout.size, Size { width: 50.0, height: 60.0 });
    }

    #[test]
    fn relative_root_is_offset_by_insets() {
        let mut tree: TaffyTree<()> = TaffyTree::new();
        let oof = tree
            .new_leaf(oof_style(
                Position::Absolute,
                Rect { left: length(0.0), top: length(0.0), right: auto(), bottom: auto() },
                Size { width: length(50.0), height: length(50.0) },
            ))
            .unwrap();
        let root = tree
            .new_with_children(
                Style {
                    position: Position::Relative,
                    inset: Rect { left: length(100.0), top: auto(), right: auto(), bottom: length(25.0) },
                    ..root_style()
                },
                &[oof],
            )
            .unwrap();
        tree.compute_layout(root, VIEWPORT).unwrap();

        let layout = tree.layout(root).unwrap();
        assert_eq!(layout.location, Point { x: 10.0 + 100.0, y: 20.0 - 25.0 });
        assert_eq!(layout.size.width, 780.0);
        // The relatively positioned root is the containing block for its absolutely positioned child
        assert_eq!(tree.layout(oof).unwrap().location, Point { x: 5.0, y: 5.0 });
    }
}
