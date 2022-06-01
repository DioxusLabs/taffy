#[test]
fn flex_grow_height_maximized() {
    let mut stretch = sprawl::Sprawl::new();
    let node00 = stretch
        .new_node(
            sprawl::style::Style {
                flex_grow: 1f32,
                flex_basis: sprawl::style::Dimension::Points(200f32),
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node01 = stretch
        .new_node(
            sprawl::style::Style {
                size: sprawl::geometry::Size { height: sprawl::style::Dimension::Points(100f32), ..Default::default() },
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
                min_size: sprawl::geometry::Size {
                    height: sprawl::style::Dimension::Points(100f32),
                    ..Default::default()
                },
                max_size: sprawl::geometry::Size {
                    height: sprawl::style::Dimension::Points(500f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node00, node01],
        )
        .unwrap();
    let node = stretch
        .new_node(
            sprawl::style::Style {
                flex_direction: sprawl::style::FlexDirection::Column,
                size: sprawl::geometry::Size {
                    width: sprawl::style::Dimension::Points(100f32),
                    height: sprawl::style::Dimension::Points(500f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node0],
        )
        .unwrap();
    stretch.compute_layout(node, sprawl::geometry::Size::undefined()).unwrap();
    assert_eq!(stretch.layout(node).unwrap().size.width, 100f32);
    assert_eq!(stretch.layout(node).unwrap().size.height, 500f32);
    assert_eq!(stretch.layout(node).unwrap().location.x, 0f32);
    assert_eq!(stretch.layout(node).unwrap().location.y, 0f32);
    assert_eq!(stretch.layout(node0).unwrap().size.width, 100f32);
    assert_eq!(stretch.layout(node0).unwrap().size.height, 500f32);
    assert_eq!(stretch.layout(node0).unwrap().location.x, 0f32);
    assert_eq!(stretch.layout(node0).unwrap().location.y, 0f32);
    assert_eq!(stretch.layout(node00).unwrap().size.width, 100f32);
    assert_eq!(stretch.layout(node00).unwrap().size.height, 400f32);
    assert_eq!(stretch.layout(node00).unwrap().location.x, 0f32);
    assert_eq!(stretch.layout(node00).unwrap().location.y, 0f32);
    assert_eq!(stretch.layout(node01).unwrap().size.width, 100f32);
    assert_eq!(stretch.layout(node01).unwrap().size.height, 100f32);
    assert_eq!(stretch.layout(node01).unwrap().location.x, 0f32);
    assert_eq!(stretch.layout(node01).unwrap().location.y, 400f32);
}
