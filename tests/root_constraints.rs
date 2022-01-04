#[cfg(test)]
mod root_constraints {
    use stretch2::number::*;

    #[test]
    fn root_with_percentage_size() {
        let mut stretch = stretch2::node::Stretch::new();
        let node = stretch
            .new_node(
                stretch2::style::Style {
                    size: stretch2::geometry::Size {
                        width: stretch2::style::Dimension::Percent(1.0),
                        height: stretch2::style::Dimension::Percent(1.0),
                    },
                    ..Default::default()
                },
                &[],
            )
            .unwrap();

        stretch
            .compute_layout(
                node,
                stretch2::geometry::Size { width: Number::Defined(100.0), height: Number::Defined(200.0) },
            )
            .unwrap();
        let layout = stretch.layout(node).unwrap();

        assert_eq!(layout.size.width, 100.0);
        assert_eq!(layout.size.height, 200.0);
    }

    #[test]
    fn root_with_no_size() {
        let mut stretch = stretch2::node::Stretch::new();
        let node = stretch.new_node(stretch2::style::Style { ..Default::default() }, &[]).unwrap();

        stretch
            .compute_layout(
                node,
                stretch2::geometry::Size { width: Number::Defined(100.0), height: Number::Defined(100.0) },
            )
            .unwrap();
        let layout = stretch.layout(node).unwrap();

        assert_eq!(layout.size.width, 0.0);
        assert_eq!(layout.size.height, 0.0);
    }

    #[test]
    fn root_with_larger_size() {
        let mut stretch = stretch2::node::Stretch::new();
        let node = stretch
            .new_node(
                stretch2::style::Style {
                    size: stretch2::geometry::Size {
                        width: stretch2::style::Dimension::Points(200.0),
                        height: stretch2::style::Dimension::Points(200.0),
                    },
                    ..Default::default()
                },
                &[],
            )
            .unwrap();

        stretch
            .compute_layout(
                node,
                stretch2::geometry::Size { width: Number::Defined(100.0), height: Number::Defined(100.0) },
            )
            .unwrap();
        let layout = stretch.layout(node).unwrap();

        assert_eq!(layout.size.width, 200.0);
        assert_eq!(layout.size.height, 200.0);
    }
}
