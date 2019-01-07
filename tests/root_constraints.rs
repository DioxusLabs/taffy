#[cfg(test)]
mod root_constraints {
    use stretch::number::*;

    #[test]
    fn root_with_percentage_size() {
        let layout = stretch::compute(
            &stretch::style::Node {
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Percent(1.0),
                    height: stretch::style::Dimension::Percent(1.0),
                },
                ..Default::default()
            },
            stretch::geometry::Size { width: Number::Defined(100.0), height: Number::Defined(100.0) },
        );

        assert_eq!(layout.size.width, 100.0);
        assert_eq!(layout.size.height, 100.0);
    }

    #[test]
    fn root_with_no_size() {
        let layout = stretch::compute(
            &stretch::style::Node { ..Default::default() },
            stretch::geometry::Size { width: Number::Defined(100.0), height: Number::Defined(100.0) },
        );

        assert_eq!(layout.size.width, 0.0);
        assert_eq!(layout.size.height, 0.0);
    }

    #[test]
    fn root_with_larger_size() {
        let layout = stretch::compute(
            &stretch::style::Node {
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(200.0),
                    height: stretch::style::Dimension::Points(200.0),
                },
                ..Default::default()
            },
            stretch::geometry::Size { width: Number::Defined(100.0), height: Number::Defined(100.0) },
        );

        assert_eq!(layout.size.width, 200.0);
        assert_eq!(layout.size.height, 200.0);
    }
}
