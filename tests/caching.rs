#[cfg(test)]
mod caching {
    use taffy::node::MeasureFunc;
    use taffy::prelude::*;

    #[test]
    fn measure_count_flexbox() {
        use std::sync::atomic::{AtomicU32, Ordering};

        let mut taffy = Taffy::new();
        static NUM_MEASURES: AtomicU32 = AtomicU32::new(0);

        let leaf = taffy
            .new_leaf_with_measure(
                Style { ..Default::default() },
                MeasureFunc::Raw(|known_dimensions, _available_space| {
                    NUM_MEASURES.fetch_add(1, Ordering::SeqCst);
                    Size {
                        width: known_dimensions.width.unwrap_or(50.0),
                        height: known_dimensions.height.unwrap_or(50.0),
                    }
                }),
            )
            .unwrap();

        let mut node = taffy.new_with_children(Style::DEFAULT, &[leaf]).unwrap();
        for _ in 0..100 {
            node = taffy.new_with_children(Style::DEFAULT, &[node]).unwrap();
        }

        taffy.compute_layout(node, Size::MAX_CONTENT).unwrap();

        assert_eq!(NUM_MEASURES.load(Ordering::SeqCst), 8);
    }

    #[test]
    #[cfg(feature = "grid")]
    fn measure_count_grid() {
        use std::sync::atomic::{AtomicU32, Ordering};

        let style = || Style { display: Display::Grid, ..Default::default() };

        let mut taffy = Taffy::new();
        static NUM_MEASURES: AtomicU32 = AtomicU32::new(0);

        let leaf = taffy
            .new_leaf_with_measure(
                style(),
                MeasureFunc::Raw(|known_dimensions, _available_space| {
                    NUM_MEASURES.fetch_add(1, Ordering::SeqCst);
                    Size {
                        width: known_dimensions.width.unwrap_or(50.0),
                        height: known_dimensions.height.unwrap_or(50.0),
                    }
                }),
            )
            .unwrap();

        let mut node = taffy.new_with_children(Style::DEFAULT, &[leaf]).unwrap();
        for _ in 0..100 {
            node = taffy.new_with_children(Style::DEFAULT, &[node]).unwrap();
        }

        taffy.compute_layout(node, Size::MAX_CONTENT).unwrap();
        assert_eq!(NUM_MEASURES.load(Ordering::SeqCst), 8);
    }
}
