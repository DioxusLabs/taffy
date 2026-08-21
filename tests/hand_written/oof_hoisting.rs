//! Tests for out-of-flow hoisting: absolute/fixed boxes are laid out by their containing
//! block (nearest positioned ancestor, or the root) rather than by their DOM parent, and
//! their `Layout.location` is relative to that containing block.
#[cfg(test)]
mod oof_hoisting {
    use taffy::prelude::*;
    use taffy::Point;

    fn leaf_style(width: f32, height: f32) -> Style {
        Style { size: Size { width: length(width), height: length(height) }, ..Default::default() }
    }

    /// An absolute box whose parent is static is positioned relative to the nearest
    /// positioned ancestor, with a containing-block-relative location.
    #[test]
    fn absolute_skips_static_parent() {
        for display in [Display::Block, Display::Flex, Display::Grid] {
            let mut tree: TaffyTree<()> = TaffyTree::new();
            let abs = tree
                .new_leaf(Style {
                    position: Position::Absolute,
                    inset: Rect { left: length(10.0), top: length(20.0), right: auto(), bottom: auto() },
                    size: Size { width: length(30.0), height: length(30.0) },
                    ..Default::default()
                })
                .unwrap();
            // Static in-flow parent, offset from the containing block's origin
            let static_parent = tree
                .new_with_children(
                    Style {
                        display,
                        margin: Rect { left: length(40.0), top: length(40.0), right: auto(), bottom: auto() },
                        size: Size { width: length(50.0), height: length(50.0) },
                        ..Default::default()
                    },
                    &[abs],
                )
                .unwrap();
            let cb = tree
                .new_with_children(
                    Style {
                        display,
                        position: Position::Relative,
                        size: Size { width: length(200.0), height: length(200.0) },
                        ..Default::default()
                    },
                    &[static_parent],
                )
                .unwrap();

            tree.compute_layout(cb, Size::MAX_CONTENT).unwrap();

            // Location is relative to the containing block, not the static parent
            let layout = tree.layout(abs).unwrap();
            assert_eq!(layout.location, Point { x: 10.0, y: 20.0 }, "{display:?}");
            assert_eq!(layout.size, Size { width: 30.0, height: 30.0 }, "{display:?}");
        }
    }

    /// An absolute box with auto insets is placed at its static position: where it would
    /// have been placed in the normal flow of its DOM parent, expressed relative to its
    /// containing block.
    #[test]
    fn static_position_is_containing_block_relative() {
        for display in [Display::Block, Display::Flex, Display::Grid] {
            let mut tree: TaffyTree<()> = TaffyTree::new();
            let abs = tree
                .new_leaf(Style {
                    position: Position::Absolute,
                    size: Size { width: length(30.0), height: length(30.0) },
                    ..Default::default()
                })
                .unwrap();
            let sibling = tree.new_leaf(leaf_style(50.0, 50.0)).unwrap();
            let static_parent = tree
                .new_with_children(
                    Style {
                        display,
                        margin: Rect { left: length(40.0), top: length(40.0), right: auto(), bottom: auto() },
                        size: Size { width: length(100.0), height: length(100.0) },
                        ..Default::default()
                    },
                    &[sibling, abs],
                )
                .unwrap();
            let cb = tree
                .new_with_children(
                    Style {
                        display,
                        position: Position::Relative,
                        size: Size { width: length(200.0), height: length(200.0) },
                        ..Default::default()
                    },
                    &[static_parent],
                )
                .unwrap();

            tree.compute_layout(cb, Size::MAX_CONTENT).unwrap();

            // Static position is offset by the static parent's location within the CB
            let layout = tree.layout(abs).unwrap();
            let expected = match display {
                // Block: below the in-flow sibling
                Display::Block => Point { x: 40.0, y: 90.0 },
                // Flex/grid: placed as if it were the sole item, at the content-box start
                Display::Flex => Point { x: 40.0, y: 40.0 },
                Display::Grid => Point { x: 40.0, y: 40.0 },
                _ => unreachable!(),
            };
            assert_eq!(layout.location, expected, "{display:?}");
        }
    }

    /// A fixed box bubbles past positioned (non-transformed) ancestors all the way to the root.
    #[test]
    fn fixed_hoists_to_root() {
        let mut tree: TaffyTree<()> = TaffyTree::new();
        let fixed = tree
            .new_leaf(Style {
                position: Position::Fixed,
                inset: Rect { left: length(5.0), top: length(5.0), right: auto(), bottom: auto() },
                size: Size { width: length(10.0), height: length(10.0) },
                ..Default::default()
            })
            .unwrap();
        let positioned_parent = tree
            .new_with_children(
                Style {
                    position: Position::Relative,
                    margin: Rect { left: length(50.0), top: length(50.0), right: auto(), bottom: auto() },
                    size: Size { width: length(50.0), height: length(50.0) },
                    ..Default::default()
                },
                &[fixed],
            )
            .unwrap();
        let root = tree
            .new_with_children(
                Style { size: Size { width: length(200.0), height: length(200.0) }, ..Default::default() },
                &[positioned_parent],
            )
            .unwrap();

        tree.compute_layout(root, Size::MAX_CONTENT).unwrap();

        // Location is relative to the root, ignoring the positioned (but not fixed-CB) parent
        assert_eq!(tree.layout(fixed).unwrap().location, Point { x: 5.0, y: 5.0 });
    }

    /// An absolute descendant of an absolute box resolves against the absolute box
    /// (which is positioned and therefore a containing block).
    #[test]
    fn absolute_within_absolute() {
        let mut tree: TaffyTree<()> = TaffyTree::new();
        let inner = tree
            .new_leaf(Style {
                position: Position::Absolute,
                inset: Rect { right: length(0.0), bottom: length(0.0), left: auto(), top: auto() },
                size: Size { width: length(10.0), height: length(10.0) },
                ..Default::default()
            })
            .unwrap();
        let outer = tree
            .new_with_children(
                Style {
                    position: Position::Absolute,
                    inset: Rect { left: length(20.0), top: length(20.0), right: auto(), bottom: auto() },
                    size: Size { width: length(50.0), height: length(50.0) },
                    ..Default::default()
                },
                &[inner],
            )
            .unwrap();
        let root = tree
            .new_with_children(
                Style { size: Size { width: length(200.0), height: length(200.0) }, ..Default::default() },
                &[outer],
            )
            .unwrap();

        tree.compute_layout(root, Size::MAX_CONTENT).unwrap();

        assert_eq!(tree.layout(outer).unwrap().location, Point { x: 20.0, y: 20.0 });
        // Relative to `outer`, its containing block
        assert_eq!(tree.layout(inner).unwrap().location, Point { x: 40.0, y: 40.0 });
    }

    /// A fixed descendant surfaced while laying out an absolute box (at the abspos box's
    /// containing block) is re-swept and continues bubbling to the root.
    #[test]
    fn fixed_within_absolute_hoists_to_root() {
        let mut tree: TaffyTree<()> = TaffyTree::new();
        let fixed = tree
            .new_leaf(Style {
                position: Position::Fixed,
                inset: Rect { left: length(1.0), top: length(2.0), right: auto(), bottom: auto() },
                size: Size { width: length(10.0), height: length(10.0) },
                ..Default::default()
            })
            .unwrap();
        let abs = tree
            .new_with_children(
                Style {
                    position: Position::Absolute,
                    inset: Rect { left: length(20.0), top: length(20.0), right: auto(), bottom: auto() },
                    size: Size { width: length(50.0), height: length(50.0) },
                    ..Default::default()
                },
                &[fixed],
            )
            .unwrap();
        let cb = tree
            .new_with_children(
                Style {
                    position: Position::Relative,
                    margin: Rect { left: length(30.0), top: length(30.0), right: auto(), bottom: auto() },
                    size: Size { width: length(100.0), height: length(100.0) },
                    ..Default::default()
                },
                &[abs],
            )
            .unwrap();
        let root = tree
            .new_with_children(
                Style { size: Size { width: length(200.0), height: length(200.0) }, ..Default::default() },
                &[cb],
            )
            .unwrap();

        tree.compute_layout(root, Size::MAX_CONTENT).unwrap();

        assert_eq!(tree.layout(abs).unwrap().location, Point { x: 20.0, y: 20.0 });
        // Root-relative, unaffected by `cb` or `abs` offsets
        assert_eq!(tree.layout(fixed).unwrap().location, Point { x: 1.0, y: 2.0 });
    }

    /// Hoisting still works when the intermediate (static) ancestors hit the layout cache:
    /// the cached `LayoutOutput` re-propagates the candidates without descending into
    /// the skipped subtree.
    #[test]
    fn hoisting_survives_cache_hits() {
        let mut tree: TaffyTree<()> = TaffyTree::new();
        let abs = tree
            .new_leaf(Style {
                position: Position::Absolute,
                inset: Rect { left: length(10.0), top: length(20.0), right: auto(), bottom: auto() },
                size: Size { width: length(30.0), height: length(30.0) },
                ..Default::default()
            })
            .unwrap();
        let mut static_ancestor = tree.new_with_children(leaf_style(50.0, 50.0), &[abs]).unwrap();
        for _ in 0..3 {
            static_ancestor = tree.new_with_children(leaf_style(50.0, 50.0), &[static_ancestor]).unwrap();
        }
        let cb = tree
            .new_with_children(
                Style {
                    position: Position::Relative,
                    size: Size { width: length(200.0), height: length(200.0) },
                    ..Default::default()
                },
                &[static_ancestor],
            )
            .unwrap();

        tree.compute_layout(cb, Size::MAX_CONTENT).unwrap();
        assert_eq!(tree.layout(abs).unwrap().location, Point { x: 10.0, y: 20.0 });

        // Recompute without dirtying anything: everything hits the cache
        tree.compute_layout(cb, Size::MAX_CONTENT).unwrap();
        assert_eq!(tree.layout(abs).unwrap().location, Point { x: 10.0, y: 20.0 });

        // Relayout with the same constraints: intermediate nodes hit the cache
        tree.mark_dirty(cb).unwrap();
        tree.compute_layout(cb, Size::MAX_CONTENT).unwrap();
        assert_eq!(tree.layout(abs).unwrap().location, Point { x: 10.0, y: 20.0 });

        // Changing the abspos box's styles relays it out through its containing block
        let mut new_style = tree.style(abs).unwrap().clone();
        new_style.inset.left = length(15.0);
        tree.set_style(abs, new_style).unwrap();
        tree.compute_layout(cb, Size::MAX_CONTENT).unwrap();
        assert_eq!(tree.layout(abs).unwrap().location, Point { x: 15.0, y: 20.0 });
    }

    /// Rounding recurses into hoisted boxes via their containing block, so a hoisted box's
    /// rounded location is computed against the containing block's cumulative offset.
    #[test]
    fn rounding_uses_containing_block_offsets() {
        let mut tree: TaffyTree<()> = TaffyTree::new();
        tree.enable_rounding();
        let abs = tree
            .new_leaf(Style {
                position: Position::Absolute,
                inset: Rect { left: length(10.6), top: length(10.6), right: auto(), bottom: auto() },
                size: Size { width: length(10.0), height: length(10.0) },
                ..Default::default()
            })
            .unwrap();
        let static_parent = tree.new_with_children(leaf_style(50.0, 50.0), &[abs]).unwrap();
        let cb = tree
            .new_with_children(
                Style {
                    position: Position::Relative,
                    margin: Rect { left: length(20.3), top: length(20.3), right: auto(), bottom: auto() },
                    size: Size { width: length(100.0), height: length(100.0) },
                    ..Default::default()
                },
                &[static_parent],
            )
            .unwrap();
        let root = tree
            .new_with_children(
                Style { size: Size { width: length(200.0), height: length(200.0) }, ..Default::default() },
                &[cb],
            )
            .unwrap();

        tree.compute_layout(root, Size::MAX_CONTENT).unwrap();

        // cb rounds to x=20; abs at cumulative 20.3+10.6=30.9 rounds to 31, i.e. 11 relative to cb
        assert_eq!(tree.layout(cb).unwrap().location, Point { x: 20.0, y: 20.0 });
        assert_eq!(tree.layout(abs).unwrap().location, Point { x: 11.0, y: 11.0 });
    }
}
