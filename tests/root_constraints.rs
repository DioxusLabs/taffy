#[cfg(test)]
mod root_constraints {
    use taffy::number::*;

    #[test]
    fn root_with_percentage_size() {
        let mut taffy = taffy::node::Taffy::new();
        let node = taffy
            .new_node(
                taffy::style::Style {
                    size: taffy::geometry::Size {
                        width: taffy::style::Dimension::Percent(1.0),
                        height: taffy::style::Dimension::Percent(1.0),
                    },
                    ..Default::default()
                },
                &[],
            )
            .unwrap();

        taffy
            .compute_layout(
                node,
                taffy::geometry::Size { width: Number::Defined(100.0), height: Number::Defined(200.0) },
            )
            .unwrap();
        let layout = taffy.layout(node).unwrap();

        assert_eq!(layout.size.width, 100.0);
        assert_eq!(layout.size.height, 200.0);
    }

    #[test]
    fn root_with_no_size() {
        let mut taffy = taffy::node::Taffy::new();
        let node = taffy.new_node(taffy::style::Style { ..Default::default() }, &[]).unwrap();

        taffy
            .compute_layout(
                node,
                taffy::geometry::Size { width: Number::Defined(100.0), height: Number::Defined(100.0) },
            )
            .unwrap();
        let layout = taffy.layout(node).unwrap();

        assert_eq!(layout.size.width, 0.0);
        assert_eq!(layout.size.height, 0.0);
    }

    #[test]
    fn root_with_larger_size() {
        let mut taffy = taffy::node::Taffy::new();
        let node = taffy
            .new_node(
                taffy::style::Style {
                    size: taffy::geometry::Size {
                        width: taffy::style::Dimension::Points(200.0),
                        height: taffy::style::Dimension::Points(200.0),
                    },
                    ..Default::default()
                },
                &[],
            )
            .unwrap();

        taffy
            .compute_layout(
                node,
                taffy::geometry::Size { width: Number::Defined(100.0), height: Number::Defined(100.0) },
            )
            .unwrap();
        let layout = taffy.layout(node).unwrap();

        assert_eq!(layout.size.width, 200.0);
        assert_eq!(layout.size.height, 200.0);
    }
}
