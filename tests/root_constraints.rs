#[cfg(test)]
mod root_constraints {
    use taffy::style::Dimension;

    #[test]
    fn root_with_percentage_size() {
        let mut taffy = taffy::node::Taffy::new();
        let node = taffy
            .new_leaf(taffy::style::FlexboxLayout {
                size: taffy::geometry::Size::<Option<Dimension>>::from_percent(1.0, 1.0),
                ..Default::default()
            })
            .unwrap();

        taffy.compute_layout(node, taffy::geometry::Size { width: Some(100.0), height: Some(200.0) }).unwrap();
        let layout = taffy.layout(node).unwrap();

        assert_eq!(layout.size.width, 100.0);
        assert_eq!(layout.size.height, 200.0);
    }

    #[test]
    fn root_with_no_size() {
        let mut taffy = taffy::node::Taffy::new();
        let node = taffy.new_leaf(taffy::style::FlexboxLayout { ..Default::default() }).unwrap();

        taffy.compute_layout(node, taffy::geometry::Size { width: Some(100.0), height: Some(100.0) }).unwrap();
        let layout = taffy.layout(node).unwrap();

        assert_eq!(layout.size.width, 0.0);
        assert_eq!(layout.size.height, 0.0);
    }

    #[test]
    fn root_with_larger_size() {
        let mut taffy = taffy::node::Taffy::new();
        let node = taffy
            .new_leaf(taffy::style::FlexboxLayout {
                size: taffy::geometry::Size::<Option<Dimension>>::from_points(200.0, 200.0),
                ..Default::default()
            })
            .unwrap();

        taffy.compute_layout(node, taffy::geometry::Size { width: Some(100.0), height: Some(100.0) }).unwrap();
        let layout = taffy.layout(node).unwrap();

        assert_eq!(layout.size.width, 200.0);
        assert_eq!(layout.size.height, 200.0);
    }
}
