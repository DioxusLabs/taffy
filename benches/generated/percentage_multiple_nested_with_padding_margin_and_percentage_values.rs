pub fn compute() {
    let mut stretch = sprawl::Sprawl::new();
    let node000 = stretch
        .new_node(
            sprawl::style::Style {
                size: sprawl::geometry::Size {
                    width: sprawl::style::Dimension::Percent(0.45f32),
                    ..Default::default()
                },
                margin: sprawl::geometry::Rect {
                    start: sprawl::style::Dimension::Percent(0.05f32),
                    end: sprawl::style::Dimension::Percent(0.05f32),
                    top: sprawl::style::Dimension::Percent(0.05f32),
                    bottom: sprawl::style::Dimension::Percent(0.05f32),
                    ..Default::default()
                },
                padding: sprawl::geometry::Rect {
                    start: sprawl::style::Dimension::Points(3f32),
                    end: sprawl::style::Dimension::Points(3f32),
                    top: sprawl::style::Dimension::Points(3f32),
                    bottom: sprawl::style::Dimension::Points(3f32),
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
                flex_direction: sprawl::style::FlexDirection::Column,
                size: sprawl::geometry::Size { width: sprawl::style::Dimension::Percent(0.5f32), ..Default::default() },
                margin: sprawl::geometry::Rect {
                    start: sprawl::style::Dimension::Points(5f32),
                    end: sprawl::style::Dimension::Points(5f32),
                    top: sprawl::style::Dimension::Points(5f32),
                    bottom: sprawl::style::Dimension::Points(5f32),
                    ..Default::default()
                },
                padding: sprawl::geometry::Rect {
                    start: sprawl::style::Dimension::Percent(0.03f32),
                    end: sprawl::style::Dimension::Percent(0.03f32),
                    top: sprawl::style::Dimension::Percent(0.03f32),
                    bottom: sprawl::style::Dimension::Percent(0.03f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node000],
        )
        .unwrap();
    let node0 = stretch
        .new_node(
            sprawl::style::Style {
                flex_direction: sprawl::style::FlexDirection::Column,
                flex_grow: 1f32,
                flex_basis: sprawl::style::Dimension::Percent(0.1f32),
                min_size: sprawl::geometry::Size {
                    width: sprawl::style::Dimension::Percent(0.6f32),
                    ..Default::default()
                },
                margin: sprawl::geometry::Rect {
                    start: sprawl::style::Dimension::Points(5f32),
                    end: sprawl::style::Dimension::Points(5f32),
                    top: sprawl::style::Dimension::Points(5f32),
                    bottom: sprawl::style::Dimension::Points(5f32),
                    ..Default::default()
                },
                padding: sprawl::geometry::Rect {
                    start: sprawl::style::Dimension::Points(3f32),
                    end: sprawl::style::Dimension::Points(3f32),
                    top: sprawl::style::Dimension::Points(3f32),
                    bottom: sprawl::style::Dimension::Points(3f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node00],
        )
        .unwrap();
    let node1 = stretch
        .new_node(
            sprawl::style::Style {
                flex_grow: 4f32,
                flex_basis: sprawl::style::Dimension::Percent(0.15f32),
                min_size: sprawl::geometry::Size {
                    width: sprawl::style::Dimension::Percent(0.2f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node = stretch
        .new_node(
            sprawl::style::Style {
                flex_direction: sprawl::style::FlexDirection::Column,
                size: sprawl::geometry::Size {
                    width: sprawl::style::Dimension::Points(200f32),
                    height: sprawl::style::Dimension::Points(200f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node0, node1],
        )
        .unwrap();
    stretch.compute_layout(node, sprawl::geometry::Size::undefined()).unwrap();
}
