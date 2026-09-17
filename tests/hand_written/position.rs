#[cfg(test)]
mod position_variants {
    use taffy::prelude::*;
    use taffy::{NodeId, Point, TaffyTree};

    fn leaf_with_inset(position: Position, display: Display) -> (TaffyTree<()>, NodeId) {
        let mut taffy: TaffyTree<()> = TaffyTree::new();
        let child = taffy
            .new_leaf(Style {
                position,
                size: Size { width: length(10.0), height: length(10.0) },
                inset: Rect { left: length(5.0), right: auto(), top: length(7.0), bottom: auto() },
                ..Default::default()
            })
            .unwrap();
        let root = taffy
            .new_with_children(
                Style { display, size: Size { width: length(100.0), height: length(100.0) }, ..Default::default() },
                &[child],
            )
            .unwrap();
        taffy.compute_layout(root, Size::MAX_CONTENT).unwrap();
        (taffy, child)
    }

    #[test]
    fn static_and_sticky_ignore_inset() {
        for display in [Display::Block, Display::Flex, Display::Grid] {
            for position in [Position::Static, Position::Sticky] {
                let (taffy, child) = leaf_with_inset(position, display);
                let layout = taffy.layout(child).unwrap();
                assert_eq!(layout.location, Point { x: 0.0, y: 0.0 }, "{position:?} in {display:?}");
            }
            let (taffy, child) = leaf_with_inset(Position::Relative, display);
            assert_eq!(taffy.layout(child).unwrap().location, Point { x: 5.0, y: 7.0 }, "Relative in {display:?}");
        }
    }

    #[test]
    fn sticky_is_positioned_but_in_flow() {
        assert!(Position::Sticky.is_positioned());
        assert!(!Position::Sticky.is_out_of_flow());
        assert!(!Position::Static.is_positioned());
    }
}
