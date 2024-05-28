#[cfg(test)]
mod caching {
    use taffy::prelude::*;

    struct CountMeasure {
        count: usize,
    }
    impl CountMeasure {
        fn new() -> CountMeasure {
            CountMeasure { count: 0 }
        }
    }

    fn count_measure_function(
        known_dimensions: Size<Option<f32>>,
        _available_space: Size<AvailableSpace>,
        _node_id: NodeId,
        mut node_context: Option<&mut CountMeasure>,
        _style: &Style,
    ) -> Size<f32> {
        node_context.as_mut().unwrap().count += 1;
        Size { width: known_dimensions.width.unwrap_or(50.0), height: known_dimensions.height.unwrap_or(50.0) }
    }

    #[test]
    fn measure_count_flexbox() {
        let mut taffy: TaffyTree<CountMeasure> = TaffyTree::new();

        let leaf = taffy.new_leaf_with_context(Style::default(), CountMeasure::new()).unwrap();

        let mut node = taffy.new_with_children(Style::DEFAULT, &[leaf]).unwrap();
        for _ in 0..100 {
            node = taffy.new_with_children(Style::DEFAULT, &[node]).unwrap();
        }

        taffy.compute_layout_with_measure(node, Size::MAX_CONTENT, count_measure_function).unwrap();

        assert_eq!(taffy.get_node_context_mut(leaf).unwrap().count, 4);
    }

    #[test]
    #[cfg(feature = "grid")]
    fn measure_count_grid() {
        let mut taffy: TaffyTree<CountMeasure> = TaffyTree::new();

        let style = || Style { display: Display::Grid, ..Default::default() };
        let leaf = taffy.new_leaf_with_context(style(), CountMeasure::new()).unwrap();

        let mut node = taffy.new_with_children(Style::DEFAULT, &[leaf]).unwrap();
        for _ in 0..100 {
            node = taffy.new_with_children(Style::DEFAULT, &[node]).unwrap();
        }

        taffy.compute_layout_with_measure(node, Size::MAX_CONTENT, count_measure_function).unwrap();
        assert_eq!(taffy.get_node_context_mut(leaf).unwrap().count, 4);
    }
}
