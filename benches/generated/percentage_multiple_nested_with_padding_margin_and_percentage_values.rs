pub fn compute() {
    let mut stretch = stretch2::Stretch::new();
    let node000 = stretch
        .new_node(
            stretch2::style::Style {
                size: stretch2::geometry::Size {
                    width: stretch2::style::Dimension::Percent(0.45f32),
                    ..Default::default()
                },
                margin: stretch2::geometry::Rect {
                    start: stretch2::style::Dimension::Percent(0.05f32),
                    end: stretch2::style::Dimension::Percent(0.05f32),
                    top: stretch2::style::Dimension::Percent(0.05f32),
                    bottom: stretch2::style::Dimension::Percent(0.05f32),
                    ..Default::default()
                },
                padding: stretch2::geometry::Rect {
                    start: stretch2::style::Dimension::Points(3f32),
                    end: stretch2::style::Dimension::Points(3f32),
                    top: stretch2::style::Dimension::Points(3f32),
                    bottom: stretch2::style::Dimension::Points(3f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node00 = stretch
        .new_node(
            stretch2::style::Style {
                flex_direction: stretch2::style::FlexDirection::Column,
                size: stretch2::geometry::Size {
                    width: stretch2::style::Dimension::Percent(0.5f32),
                    ..Default::default()
                },
                margin: stretch2::geometry::Rect {
                    start: stretch2::style::Dimension::Points(5f32),
                    end: stretch2::style::Dimension::Points(5f32),
                    top: stretch2::style::Dimension::Points(5f32),
                    bottom: stretch2::style::Dimension::Points(5f32),
                    ..Default::default()
                },
                padding: stretch2::geometry::Rect {
                    start: stretch2::style::Dimension::Percent(0.03f32),
                    end: stretch2::style::Dimension::Percent(0.03f32),
                    top: stretch2::style::Dimension::Percent(0.03f32),
                    bottom: stretch2::style::Dimension::Percent(0.03f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node000],
        )
        .unwrap();
    let node0 = stretch
        .new_node(
            stretch2::style::Style {
                flex_direction: stretch2::style::FlexDirection::Column,
                flex_grow: 1f32,
                flex_basis: stretch2::style::Dimension::Percent(0.1f32),
                min_size: stretch2::geometry::Size {
                    width: stretch2::style::Dimension::Percent(0.6f32),
                    ..Default::default()
                },
                margin: stretch2::geometry::Rect {
                    start: stretch2::style::Dimension::Points(5f32),
                    end: stretch2::style::Dimension::Points(5f32),
                    top: stretch2::style::Dimension::Points(5f32),
                    bottom: stretch2::style::Dimension::Points(5f32),
                    ..Default::default()
                },
                padding: stretch2::geometry::Rect {
                    start: stretch2::style::Dimension::Points(3f32),
                    end: stretch2::style::Dimension::Points(3f32),
                    top: stretch2::style::Dimension::Points(3f32),
                    bottom: stretch2::style::Dimension::Points(3f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node00],
        )
        .unwrap();
    let node1 = stretch
        .new_node(
            stretch2::style::Style {
                flex_grow: 4f32,
                flex_basis: stretch2::style::Dimension::Percent(0.15f32),
                min_size: stretch2::geometry::Size {
                    width: stretch2::style::Dimension::Percent(0.2f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node = stretch
        .new_node(
            stretch2::style::Style {
                flex_direction: stretch2::style::FlexDirection::Column,
                size: stretch2::geometry::Size {
                    width: stretch2::style::Dimension::Points(200f32),
                    height: stretch2::style::Dimension::Points(200f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node0, node1],
        )
        .unwrap();
    stretch.compute_layout(node, stretch2::geometry::Size::undefined()).unwrap();
}
