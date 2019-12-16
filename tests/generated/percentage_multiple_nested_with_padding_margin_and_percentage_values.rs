#[test]
fn percentage_multiple_nested_with_padding_margin_and_percentage_values() {
    let mut stretch = stretch::Stretch::new();
    let node000 = stretch
        .new_node(
            stretch::style::Style {
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Percent(0.45f32),
                    ..Default::default()
                },
                margin: stretch::geometry::Rect {
                    start: stretch::style::Dimension::Percent(0.05f32),
                    end: stretch::style::Dimension::Percent(0.05f32),
                    top: stretch::style::Dimension::Percent(0.05f32),
                    bottom: stretch::style::Dimension::Percent(0.05f32),
                    ..Default::default()
                },
                padding: stretch::geometry::Rect {
                    start: stretch::style::Dimension::Points(3f32),
                    end: stretch::style::Dimension::Points(3f32),
                    top: stretch::style::Dimension::Points(3f32),
                    bottom: stretch::style::Dimension::Points(3f32),
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
                flex_direction: stretch::style::FlexDirection::Column,
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Percent(0.5f32),
                    ..Default::default()
                },
                margin: stretch::geometry::Rect {
                    start: stretch::style::Dimension::Points(5f32),
                    end: stretch::style::Dimension::Points(5f32),
                    top: stretch::style::Dimension::Points(5f32),
                    bottom: stretch::style::Dimension::Points(5f32),
                    ..Default::default()
                },
                padding: stretch::geometry::Rect {
                    start: stretch::style::Dimension::Percent(0.03f32),
                    end: stretch::style::Dimension::Percent(0.03f32),
                    top: stretch::style::Dimension::Percent(0.03f32),
                    bottom: stretch::style::Dimension::Percent(0.03f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node000],
        )
        .unwrap();
    let node0 = stretch
        .new_node(
            stretch::style::Style {
                flex_direction: stretch::style::FlexDirection::Column,
                flex_grow: 1f32,
                flex_basis: stretch::style::Dimension::Percent(0.1f32),
                min_size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Percent(0.6f32),
                    ..Default::default()
                },
                margin: stretch::geometry::Rect {
                    start: stretch::style::Dimension::Points(5f32),
                    end: stretch::style::Dimension::Points(5f32),
                    top: stretch::style::Dimension::Points(5f32),
                    bottom: stretch::style::Dimension::Points(5f32),
                    ..Default::default()
                },
                padding: stretch::geometry::Rect {
                    start: stretch::style::Dimension::Points(3f32),
                    end: stretch::style::Dimension::Points(3f32),
                    top: stretch::style::Dimension::Points(3f32),
                    bottom: stretch::style::Dimension::Points(3f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node00],
        )
        .unwrap();
    let node1 = stretch
        .new_node(
            stretch::style::Style {
                flex_grow: 4f32,
                flex_basis: stretch::style::Dimension::Percent(0.15f32),
                min_size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Percent(0.2f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node = stretch
        .new_node(
            stretch::style::Style {
                flex_direction: stretch::style::FlexDirection::Column,
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(200f32),
                    height: stretch::style::Dimension::Points(200f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node0, node1],
        )
        .unwrap();
    stretch.compute_layout(node, stretch::geometry::Size::undefined()).unwrap();
    assert_eq!(stretch.layout(node).unwrap().size.width, 200f32);
    assert_eq!(stretch.layout(node).unwrap().size.height, 200f32);
    assert_eq!(stretch.layout(node).unwrap().location.x, 0f32);
    assert_eq!(stretch.layout(node).unwrap().location.y, 0f32);
    assert_eq!(stretch.layout(node0).unwrap().size.width, 190f32);
    assert_eq!(stretch.layout(node0).unwrap().size.height, 48f32);
    assert_eq!(stretch.layout(node0).unwrap().location.x, 5f32);
    assert_eq!(stretch.layout(node0).unwrap().location.y, 5f32);
    assert_eq!(stretch.layout(node00).unwrap().size.width, 92f32);
    assert_eq!(stretch.layout(node00).unwrap().size.height, 25f32);
    assert_eq!(stretch.layout(node00).unwrap().location.x, 8f32);
    assert_eq!(stretch.layout(node00).unwrap().location.y, 8f32);
    assert_eq!(stretch.layout(node000).unwrap().size.width, 36f32);
    assert_eq!(stretch.layout(node000).unwrap().size.height, 6f32);
    assert_eq!(stretch.layout(node000).unwrap().location.x, 10f32);
    assert_eq!(stretch.layout(node000).unwrap().location.y, 10f32);
    assert_eq!(stretch.layout(node1).unwrap().size.width, 200f32);
    assert_eq!(stretch.layout(node1).unwrap().size.height, 142f32);
    assert_eq!(stretch.layout(node1).unwrap().location.x, 0f32);
    assert_eq!(stretch.layout(node1).unwrap().location.y, 58f32);
}
