#[test]
fn percentage_absolute_position() {
    let mut sprawl = sprawl::Sprawl::new();
    let node0 = sprawl
        .new_with_children(
            sprawl::style::Style {
                position_type: sprawl::style::PositionType::Absolute,
                size: sprawl::geometry::Size {
                    width: sprawl::style::Dimension::Points(10f32),
                    height: sprawl::style::Dimension::Points(10f32),
                    ..Default::default()
                },
                position: sprawl::geometry::Rect {
                    start: sprawl::style::Dimension::Percent(0.3f32),
                    top: sprawl::style::Dimension::Percent(0.1f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node = sprawl
        .new_with_children(
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
    sprawl.compute_layout(node, sprawl::geometry::Size::undefined()).unwrap();
    assert_eq!(sprawl.layout(node).unwrap().size.width, 200f32);
    assert_eq!(sprawl.layout(node).unwrap().size.height, 100f32);
    assert_eq!(sprawl.layout(node).unwrap().location.x, 0f32);
    assert_eq!(sprawl.layout(node).unwrap().location.y, 0f32);
    assert_eq!(sprawl.layout(node0).unwrap().size.width, 10f32);
    assert_eq!(sprawl.layout(node0).unwrap().size.height, 10f32);
    assert_eq!(sprawl.layout(node0).unwrap().location.x, 60f32);
    assert_eq!(sprawl.layout(node0).unwrap().location.y, 10f32);
}
