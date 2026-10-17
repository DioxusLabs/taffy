//! `Display::Stack`: a single row or column whose items are never flexed
#[cfg(feature = "flexbox")]
mod stack {
    use taffy::prelude::*;
    use taffy_test_helpers::{new_test_tree, test_measure_function, TestNodeContext};

    type Tree = TaffyTree<TestNodeContext>;

    /// A leaf whose content is 150px by 30px
    fn leaf(tree: &mut Tree, style: Style) -> NodeId {
        tree.new_leaf_with_context(style, TestNodeContext::fixed(150.0, 30.0)).unwrap()
    }

    fn width(width: Dimension) -> Style {
        Style { size: Size { width, height: auto() }, ..Default::default() }
    }

    fn container(display: Display) -> Style {
        Style { display, size: Size { width: length(400.0), height: length(40.0) }, ..Default::default() }
    }

    /// Lay out a container with the given style and items. Returns the width of the container and the x
    /// position, width and measure count (the final layout of a leaf counts as one) of each item.
    fn layout(container: Style, items: &[Style]) -> (f32, Vec<(f32, f32, usize)>) {
        let mut tree = new_test_tree();
        let items: Vec<_> = items.iter().map(|style| leaf(&mut tree, style.clone())).collect();
        let root = tree.new_with_children(container, &items).unwrap();
        tree.compute_layout_with_measure(root, Size::MAX_CONTENT, test_measure_function).unwrap();
        let item_layouts = items
            .iter()
            .map(|item| {
                let layout = tree.layout(*item).unwrap();
                (layout.location.x, layout.size.width, tree.get_node_context(*item).unwrap().count)
            })
            .collect();
        (tree.layout(root).unwrap().size.width, item_layouts)
    }

    #[test]
    fn items_are_sized_by_length_content_or_fill() {
        let items = [width(length(100.0)), width(auto()), width(fill(1.0)), width(fill(2.0))];
        let (_, items) = layout(container(Display::Stack), &items);
        // 400 - 100 - 150 leaves 150 for the `fl` items. Each item is measured once, for its final layout:
        // the content-sized item's main size is taken from that layout.
        assert_eq!(items, [(0.0, 100.0, 1), (100.0, 150.0, 1), (250.0, 50.0, 1), (300.0, 100.0, 1)]);
    }

    #[test]
    fn items_overflow_rather_than_shrink() {
        let items = [width(length(300.0)), width(auto()), width(fill(1.0))];
        let (_, items) = layout(container(Display::Stack), &items);
        assert_eq!(items, [(0.0, 300.0, 1), (300.0, 150.0, 1), (450.0, 0.0, 1)]);
    }

    #[test]
    fn flex_properties_are_not_used() {
        let flexed = |size| Style { flex_grow: 1.0, flex_shrink: 1.0, flex_basis: length(20.0), ..width(size) };
        let style = Style { flex_wrap: FlexWrap::Wrap, ..container(Display::Stack) };
        let (_, items) = layout(style, &[flexed(length(300.0)), flexed(auto())]);
        assert_eq!(items, [(0.0, 300.0, 1), (300.0, 150.0, 1)]);
    }

    #[test]
    fn fill_items_contribute_their_minimum_size_to_a_content_sized_container() {
        let floored = Style { min_size: Size { width: length(20.0), height: auto() }, ..width(fill(1.0)) };
        let style = Style { size: Size::auto(), ..container(Display::Stack) };
        let (container_width, items) = layout(style, &[width(auto()), floored, width(fill(1.0))]);
        assert_eq!(container_width, 170.0);
        assert_eq!(items, [(0.0, 150.0, 1), (150.0, 20.0, 1), (170.0, 0.0, 1)]);
    }

    #[test]
    fn gap_and_justify_content_apply() {
        let style = Style {
            gap: Size { width: length(10.0), height: zero() },
            justify_content: JustifyContent::END,
            ..container(Display::Stack)
        };
        let (_, items) = layout(style, &[width(length(100.0)), width(length(100.0))]);
        assert_eq!(items.iter().map(|item| item.0).collect::<Vec<_>>(), [190.0, 300.0]);
    }

    /// A stack container lays out like a flex container whose items have `flex-shrink: 0` and a zero minimum
    /// main size (measure counts are not compared: a stack measures its items less often).
    #[test]
    fn layout_matches_flexbox_without_shrinking_or_automatic_minimum_sizes() {
        let sizes = [length(100.0), auto(), fill(1.0), length(250.0), fill(3.0)];
        for item_count in 1..=sizes.len() {
            let stack_items: Vec<_> = sizes[..item_count].iter().map(|size| width(*size)).collect();
            let flex_items: Vec<_> = stack_items
                .iter()
                .map(|style| Style {
                    flex_shrink: 0.0,
                    min_size: Size { width: length(0.0), height: auto() },
                    ..style.clone()
                })
                .collect();
            let (stack_width, stack) = layout(container(Display::Stack), &stack_items);
            let (flex_width, flex) = layout(container(Display::Flex), &flex_items);
            let positions = |items: &[(f32, f32, usize)]| items.iter().map(|item| (item.0, item.1)).collect::<Vec<_>>();
            assert_eq!(stack_width, flex_width);
            assert_eq!(positions(&stack), positions(&flex));
        }
    }
}
