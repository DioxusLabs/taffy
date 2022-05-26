#[cfg(test)]
mod root_constraints {
    use sprawl::number::*;

    #[test]
    fn root_with_percentage_size() {
        let mut stretch = sprawl::node::Stretch::new();
        let node = stretch
            .new_node(
                sprawl::style::Style {
                    size: sprawl::geometry::Size {
                        width: sprawl::style::Dimension::Percent(1.0),
                        height: sprawl::style::Dimension::Percent(1.0),
                    },
                    ..Default::default()
                },
                &[],
            )
            .unwrap();

        stretch
            .compute_layout(
                node,
                sprawl::geometry::Size { width: Number::Defined(100.0), height: Number::Defined(200.0) },
            )
            .unwrap();
        let layout = stretch.layout(node).unwrap();

        assert_eq!(layout.size.width, 100.0);
        assert_eq!(layout.size.height, 200.0);
    }

    #[test]
    fn root_with_no_size() {
        let mut stretch = sprawl::node::Stretch::new();
        let node = stretch.new_node(sprawl::style::Style { ..Default::default() }, &[]).unwrap();

        stretch
            .compute_layout(
                node,
                sprawl::geometry::Size { width: Number::Defined(100.0), height: Number::Defined(100.0) },
            )
            .unwrap();
        let layout = stretch.layout(node).unwrap();

        assert_eq!(layout.size.width, 0.0);
        assert_eq!(layout.size.height, 0.0);
    }

    #[test]
    fn root_with_larger_size() {
        let mut stretch = sprawl::node::Stretch::new();
        let node = stretch
            .new_node(
                sprawl::style::Style {
                    size: sprawl::geometry::Size {
                        width: sprawl::style::Dimension::Points(200.0),
                        height: sprawl::style::Dimension::Points(200.0),
                    },
                    ..Default::default()
                },
                &[],
            )
            .unwrap();

        stretch
            .compute_layout(
                node,
                sprawl::geometry::Size { width: Number::Defined(100.0), height: Number::Defined(100.0) },
            )
            .unwrap();
        let layout = stretch.layout(node).unwrap();

        assert_eq!(layout.size.width, 200.0);
        assert_eq!(layout.size.height, 200.0);
    }
}
