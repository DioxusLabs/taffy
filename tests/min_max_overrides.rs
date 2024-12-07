#[cfg(test)]
mod min_max_overrides {
    use taffy::prelude::*;
    use taffy_test_helpers::new_test_tree;

    #[test]
    fn min_overrides_max() {
        let mut taffy = new_test_tree();

        let child = taffy
            .new_leaf(Style {
                size: Size { width: Dimension::Length(50.0), height: Dimension::Length(50.0) },
                min_size: Size { width: Dimension::Length(100.0), height: Dimension::Length(100.0) },
                max_size: Size { width: Dimension::Length(10.0), height: Dimension::Length(10.0) },
                ..Default::default()
            })
            .unwrap();

        taffy
            .compute_layout(
                child,
                Size { width: AvailableSpace::Definite(100.0), height: AvailableSpace::Definite(100.0) },
            )
            .unwrap();

        assert_eq!(taffy.layout(child).unwrap().size, Size { width: 100.0, height: 100.0 });
    }

    #[test]
    fn max_overrides_size() {
        let mut taffy = new_test_tree();

        let child = taffy
            .new_leaf(Style {
                size: Size { width: Dimension::Length(50.0), height: Dimension::Length(50.0) },
                max_size: Size { width: Dimension::Length(10.0), height: Dimension::Length(10.0) },
                ..Default::default()
            })
            .unwrap();

        taffy
            .compute_layout(
                child,
                Size { width: AvailableSpace::Definite(100.0), height: AvailableSpace::Definite(100.0) },
            )
            .unwrap();

        assert_eq!(taffy.layout(child).unwrap().size, Size { width: 10.0, height: 10.0 });
    }

    #[test]
    fn min_overrides_size() {
        let mut taffy = new_test_tree();

        let child = taffy
            .new_leaf(Style {
                size: Size { width: Dimension::Length(50.0), height: Dimension::Length(50.0) },
                min_size: Size { width: Dimension::Length(100.0), height: Dimension::Length(100.0) },
                ..Default::default()
            })
            .unwrap();

        taffy
            .compute_layout(
                child,
                Size { width: AvailableSpace::Definite(100.0), height: AvailableSpace::Definite(100.0) },
            )
            .unwrap();

        assert_eq!(taffy.layout(child).unwrap().size, Size { width: 100.0, height: 100.0 });
    }
}
