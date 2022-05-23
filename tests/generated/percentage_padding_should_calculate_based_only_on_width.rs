#[test]
fn percentage_padding_should_calculate_based_only_on_width() {
    let mut stretch = sprawl::Stretch::new();
    let node00 = stretch
        .new_node(
            sprawl::style::Style {
                size: sprawl::geometry::Size {
                    width: sprawl::style::Dimension::Points(10f32),
                    height: sprawl::style::Dimension::Points(10f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node0 = stretch
        .new_node(
            sprawl::style::Style {
                flex_direction: sprawl::style::FlexDirection::Column,
                flex_grow: 1f32,
                padding: sprawl::geometry::Rect {
                    start: sprawl::style::Dimension::Percent(0.1f32),
                    end: sprawl::style::Dimension::Percent(0.1f32),
                    top: sprawl::style::Dimension::Percent(0.1f32),
                    bottom: sprawl::style::Dimension::Percent(0.1f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node00],
        )
        .unwrap();
    let node = stretch
        .new_node(
            sprawl::style::Style {
                flex_direction: sprawl::style::FlexDirection::Column,
                size: sprawl::geometry::Size {
                    width: sprawl::style::Dimension::Points(200f32),
                    height: sprawl::style::Dimension::Points(100f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node0],
        )
        .unwrap();
    stretch.compute_layout(node, sprawl::geometry::Size::undefined()).unwrap();
    assert_eq!(stretch.layout(node).unwrap().size.width, 200f32);
    assert_eq!(stretch.layout(node).unwrap().size.height, 100f32);
    assert_eq!(stretch.layout(node).unwrap().location.x, 0f32);
    assert_eq!(stretch.layout(node).unwrap().location.y, 0f32);
    assert_eq!(stretch.layout(node0).unwrap().size.width, 200f32);
    assert_eq!(stretch.layout(node0).unwrap().size.height, 100f32);
    assert_eq!(stretch.layout(node0).unwrap().location.x, 0f32);
    assert_eq!(stretch.layout(node0).unwrap().location.y, 0f32);
    assert_eq!(stretch.layout(node00).unwrap().size.width, 10f32);
    assert_eq!(stretch.layout(node00).unwrap().size.height, 10f32);
    assert_eq!(stretch.layout(node00).unwrap().location.x, 20f32);
    assert_eq!(stretch.layout(node00).unwrap().location.y, 20f32);
}
