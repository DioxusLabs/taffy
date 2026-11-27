//! `calc()` values stored in a `TaffyTree`. Expected layouts are Chrome's.
#[cfg(feature = "calc")]
mod taffy_tree_calc {
    use taffy::prelude::*;
    use taffy::{BoxSizing, LinearCalc, compute_leaf_layout};

    /// `calc(percent% + px)`
    fn lin(percent: f32, px: f32) -> LinearCalc {
        LinearCalc::percent(percent / 100.0) + LinearCalc::length(px)
    }

    fn sized(width: Dimension, height: Dimension) -> Style {
        Style { size: Size { width, height }, ..Default::default() }
    }

    fn rect<T>(tree: &TaffyTree<T>, node: NodeId) -> [f32; 4] {
        let l = tree.layout(node).unwrap();
        [l.location.x, l.location.y, l.size.width, l.size.height]
    }

    #[test]
    fn size_margin_padding_and_gap() {
        let mut t: TaffyTree = TaffyTree::new();
        let width = t.new_calc(lin(50.0, -12.0)).into();
        let margin = t.new_calc(lin(10.0, -10.0)).into();
        let padding = t.new_calc(lin(5.0, 2.0));
        let gap = t.new_calc(lin(5.0, -5.0));
        let a =
            t.new_leaf(Style { margin: Rect { left: margin, ..Rect::zero() }, ..sized(width, length(10.0)) }).unwrap();
        let b = t.new_leaf(sized(length(20.0), length(10.0))).unwrap();
        let row = t
            .new_with_children(
                Style {
                    padding: Rect { left: padding, ..Rect::zero() },
                    gap: Size { width: gap, height: zero() },
                    align_items: AlignItems::FLEX_START,
                    ..sized(percent(1.0), length(100.0))
                },
                &[a, b],
            )
            .unwrap();
        let root = t.new_with_children(sized(length(400.0), length(100.0)), &[row]).unwrap();
        t.compute_layout(root, Size::MAX_CONTENT).unwrap();
        assert_eq!(rect(&t, a), [50.0, 0.0, 177.0, 10.0]);
        assert_eq!(rect(&t, b), [241.0, 0.0, 20.0, 10.0]);
    }

    #[test]
    fn a_length_alone_is_definite_against_an_indefinite_basis() {
        let mut t: TaffyTree = TaffyTree::new();
        assert_eq!(t.new_calc(lin(0.0, 15.0)), LengthPercentage::length(15.0));
        assert_eq!(t.new_calc(lin(50.0, 0.0)), LengthPercentage::percent(0.5));
        let half = t.new_calc(lin(50.0, 10.0)).into();
        let fixed = t.new_calc(lin(0.0, 15.0)).into();
        let inner = t.new_leaf(sized(auto(), length(30.0))).unwrap();
        let a = t.new_with_children(sized(length(20.0), half), &[inner]).unwrap();
        let b = t.new_leaf(sized(length(20.0), fixed)).unwrap();
        let root_style = Style { flex_direction: FlexDirection::Column, ..sized(length(400.0), auto()) };
        let root = t.new_with_children(root_style, &[a, b]).unwrap();
        t.compute_layout(root, Size::MAX_CONTENT).unwrap();
        assert_eq!(rect(&t, a), [0.0, 0.0, 20.0, 30.0]);
        assert_eq!(rect(&t, b), [0.0, 30.0, 20.0, 15.0]);
    }

    #[cfg(feature = "grid")]
    #[test]
    fn grid_tracks() {
        let mut t: TaffyTree = TaffyTree::new();
        let first = t.new_calc(lin(25.0, 10.0)).into();
        let min = t.new_calc(lin(10.0, 5.0)).into();
        let kids = [t.new_leaf(Style::default()).unwrap(), t.new_leaf(Style::default()).unwrap()];
        let root = t
            .new_with_children(
                Style {
                    display: Display::Grid,
                    grid_template_columns: vec![GridTemplateComponent::Single(first), minmax(min, fr(1.0))],
                    ..sized(length(400.0), length(100.0))
                },
                &kids,
            )
            .unwrap();
        t.compute_layout(root, Size::MAX_CONTENT).unwrap();
        assert_eq!(rect(&t, kids[0]), [0.0, 0.0, 110.0, 100.0]);
        assert_eq!(rect(&t, kids[1]), [110.0, 0.0, 290.0, 100.0]);
    }

    #[test]
    fn measured_leaf_padding() {
        let mut t: TaffyTree<Size<f32>> = TaffyTree::new();
        let left = t.new_calc(lin(10.0, 2.0));
        let top = t.new_calc(lin(5.0, -4.0));
        let style = Style {
            box_sizing: BoxSizing::ContentBox,
            padding: Rect { left, top, ..Rect::zero() },
            ..sized(length(50.0), length(20.0))
        };
        let leaf = t.new_leaf_with_context(style, Size { width: 50.0, height: 20.0 }).unwrap();
        let root_style = Style { align_items: AlignItems::FLEX_START, ..sized(length(400.0), length(300.0)) };
        let root = t.new_with_children(root_style, &[leaf]).unwrap();
        t.compute_layout_with_measure_and_calc(root, Size::MAX_CONTENT, |inputs, _, size, style, calc| {
            let measure = |_, _| size.copied().unwrap_or(Size::ZERO);
            compute_leaf_layout(inputs, style, |handle, basis| calc.resolve(handle, basis), measure)
        })
        .unwrap();
        assert_eq!(rect(&t, leaf)[2..], [92.0, 36.0]);
    }

    #[test]
    fn removed_values_resolve_to_zero() {
        let mut t: TaffyTree = TaffyTree::new();
        let width: Dimension = t.new_calc(lin(50.0, -10.0)).into();
        let a = t.new_leaf(sized(width, length(10.0))).unwrap();
        let root = t.new_with_children(sized(length(200.0), length(100.0)), &[a]).unwrap();
        t.compute_layout(root, Size::MAX_CONTENT).unwrap();
        assert_eq!(rect(&t, a)[2], 90.0);
        assert_eq!(t.remove_calc(width.into_raw()), Some(lin(50.0, -10.0)));
        assert_eq!(t.remove_calc(width.into_raw()), None);
        t.mark_dirty(a).unwrap();
        t.compute_layout(root, Size::MAX_CONTENT).unwrap();
        assert_eq!(rect(&t, a)[2], 0.0);
    }
}
