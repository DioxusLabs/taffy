pub fn compute() {
    let mut stretch = sprawl::Sprawl::new();
    let node000 = stretch
        .new_node(
            sprawl::style::Style {
                size: sprawl::geometry::Size {
                    width: sprawl::style::Dimension::Points(199f32),
                    height: sprawl::style::Dimension::Points(100f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node00 = stretch
        .new_node(
            sprawl::style::Style {
                justify_content: sprawl::style::JustifyContent::Center,
                min_size: sprawl::geometry::Size {
                    width: sprawl::style::Dimension::Points(400f32),
                    ..Default::default()
                },
                padding: sprawl::geometry::Rect {
                    start: sprawl::style::Dimension::Points(100f32),
                    end: sprawl::style::Dimension::Points(100f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node000],
        )
        .unwrap();
    let node0 = stretch.new_node(sprawl::style::Style { ..Default::default() }, &[node00]).unwrap();
    let node = stretch
        .new_node(
            sprawl::style::Style {
                flex_direction: sprawl::style::FlexDirection::Column,
                size: sprawl::geometry::Size {
                    width: sprawl::style::Dimension::Points(1080f32),
                    height: sprawl::style::Dimension::Points(1584f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node0],
        )
        .unwrap();
    stretch.compute_layout(node, sprawl::geometry::Size::undefined()).unwrap();
}
