#[cfg(test)]
mod depends_on_block_size {
    use taffy::prelude::*;
    use taffy::{compute_leaf_layout, LayoutInput, LayoutOutput, Style, TaffyTree};

    /// Lay out `node` as a root under max-content constraints and return `depends_on_block_size`
    fn depends_on_block_size(taffy: &mut TaffyTree<()>, node: NodeId) -> bool {
        taffy.compute_layout(node, Size::MAX_CONTENT).unwrap().depends_on_block_size
    }

    fn container(display: Display, children: &[NodeId], taffy: &mut TaffyTree<()>) -> NodeId {
        taffy.new_with_children(Style { display, ..Default::default() }, children).unwrap()
    }

    fn aspect_ratio_leaf(taffy: &mut TaffyTree<()>) -> NodeId {
        taffy.new_leaf(Style { aspect_ratio: Some(2.0), ..Default::default() }).unwrap()
    }

    #[test]
    fn plain_leaf_does_not_depend_on_block_size() {
        let mut taffy: TaffyTree<()> = TaffyTree::new();
        let leaf = taffy.new_leaf(Style::DEFAULT).unwrap();
        assert!(!depends_on_block_size(&mut taffy, leaf));
    }

    #[test]
    fn leaf_with_aspect_ratio_depends_on_block_size() {
        let mut taffy: TaffyTree<()> = TaffyTree::new();
        let leaf = aspect_ratio_leaf(&mut taffy);
        assert!(depends_on_block_size(&mut taffy, leaf));
    }

    #[test]
    #[cfg(feature = "flexbox")]
    fn flexbox_container_reports_descendant_aspect_ratio() {
        let mut taffy: TaffyTree<()> = TaffyTree::new();

        let plain = taffy.new_leaf(Style::DEFAULT).unwrap();
        let plain_parent = container(Display::Flex, &[plain], &mut taffy);
        assert!(!depends_on_block_size(&mut taffy, plain_parent));

        let ratio = aspect_ratio_leaf(&mut taffy);
        let inner = container(Display::Flex, &[ratio], &mut taffy);
        let outer = container(Display::Flex, &[inner], &mut taffy);
        assert!(depends_on_block_size(&mut taffy, outer));
    }

    #[test]
    #[cfg(feature = "flexbox")]
    fn wrapping_column_flexbox_depends_on_block_size() {
        let mut taffy: TaffyTree<()> = TaffyTree::new();
        let leaf = taffy
            .new_leaf(Style { size: Size { width: length(10.0), height: length(10.0) }, ..Default::default() })
            .unwrap();
        let container = taffy
            .new_with_children(
                Style {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    flex_wrap: FlexWrap::Wrap,
                    ..Default::default()
                },
                &[leaf],
            )
            .unwrap();

        assert!(depends_on_block_size(&mut taffy, container));
    }

    #[test]
    #[cfg(feature = "block_layout")]
    fn block_container_reports_descendant_aspect_ratio() {
        let mut taffy: TaffyTree<()> = TaffyTree::new();

        let plain = taffy.new_leaf(Style::DEFAULT).unwrap();
        let plain_parent = container(Display::Block, &[plain], &mut taffy);
        assert!(!depends_on_block_size(&mut taffy, plain_parent));

        let ratio = aspect_ratio_leaf(&mut taffy);
        let inner = container(Display::Block, &[ratio], &mut taffy);
        let outer = container(Display::Block, &[inner], &mut taffy);
        assert!(depends_on_block_size(&mut taffy, outer));
    }

    #[test]
    #[cfg(feature = "grid")]
    fn grid_container_reports_descendant_aspect_ratio() {
        let mut taffy: TaffyTree<()> = TaffyTree::new();

        let plain = taffy.new_leaf(Style::DEFAULT).unwrap();
        let plain_parent = container(Display::Grid, &[plain], &mut taffy);
        assert!(!depends_on_block_size(&mut taffy, plain_parent));

        let ratio = aspect_ratio_leaf(&mut taffy);
        let parent = container(Display::Grid, &[ratio], &mut taffy);
        assert!(depends_on_block_size(&mut taffy, parent));
    }

    #[test]
    #[cfg(feature = "flexbox")]
    fn flag_survives_the_cache_and_is_invalidated_with_it() {
        let mut taffy: TaffyTree<()> = TaffyTree::new();
        let leaf = taffy.new_leaf(Style::DEFAULT).unwrap();
        let parent = container(Display::Flex, &[leaf], &mut taffy);

        assert!(!depends_on_block_size(&mut taffy, parent));
        // Second run is served from the cache
        assert!(!depends_on_block_size(&mut taffy, parent));

        taffy.set_style(leaf, Style { aspect_ratio: Some(2.0), ..Default::default() }).unwrap();
        assert!(depends_on_block_size(&mut taffy, parent));
    }

    #[test]
    fn measure_function_can_declare_independence() {
        let measure = |declares_independence: bool| {
            move |inputs: LayoutInput, _node, _context: Option<&mut ()>, style: &Style| -> LayoutOutput {
                compute_leaf_layout(inputs, style, |_, _| 0.0, |_, _| Size { width: 40.0, height: 20.0 })
                    .with_depends_on_block_size(!declares_independence)
            }
        };

        let mut taffy: TaffyTree<()> = TaffyTree::new();
        let leaf = taffy.new_leaf(Style::DEFAULT).unwrap();

        // Measure functions are opaque, so they are conservative unless they say otherwise
        let output = taffy.compute_layout_with_measure(leaf, Size::MAX_CONTENT, measure(false)).unwrap();
        assert!(output.depends_on_block_size);
        assert_eq!(output.size.width, 40.0);

        taffy.mark_dirty(leaf).unwrap();
        let output = taffy.compute_layout_with_measure(leaf, Size::MAX_CONTENT, measure(true)).unwrap();
        assert!(!output.depends_on_block_size);
    }
}
