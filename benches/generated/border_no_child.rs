pub fn compute() {
    let mut taffy = taffy::Taffy::new();
    let node = taffy
        .new_with_children(
            taffy::style::FlexboxLayout {
                border: taffy::geometry::Rect {
                    start: Some(taffy::style::Dimension::Points(10f32)),
                    end: Some(taffy::style::Dimension::Points(10f32)),
                    top: Some(taffy::style::Dimension::Points(10f32)),
                    bottom: Some(taffy::style::Dimension::Points(10f32)),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::undefined()).unwrap();
}
