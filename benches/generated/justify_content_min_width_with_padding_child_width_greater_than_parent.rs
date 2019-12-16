pub fn compute() {
    let mut stretch = stretch::Stretch::new();
    let node000 = stretch
        .new_node(
            stretch::style::Style {
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(300f32),
                    height: stretch::style::Dimension::Points(100f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node00 = stretch
        .new_node(
            stretch::style::Style {
                justify_content: stretch::style::JustifyContent::Center,
                min_size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(400f32),
                    ..Default::default()
                },
                padding: stretch::geometry::Rect {
                    start: stretch::style::Dimension::Points(100f32),
                    end: stretch::style::Dimension::Points(100f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node000],
        )
        .unwrap();
    let node0 = stretch.new_node(stretch::style::Style { ..Default::default() }, &[node00]).unwrap();
    let node = stretch
        .new_node(
            stretch::style::Style {
                flex_direction: stretch::style::FlexDirection::Column,
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(1000f32),
                    height: stretch::style::Dimension::Points(1584f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node0],
        )
        .unwrap();
    stretch.compute_layout(node, stretch::geometry::Size::undefined()).unwrap();
}
