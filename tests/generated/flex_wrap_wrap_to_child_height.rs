#[test]
fn flex_wrap_wrap_to_child_height() {
    let mut stretch = sprawl::Sprawl::new();
    let node000 = stretch
        .new_node(
            sprawl::style::Style {
                size: sprawl::geometry::Size {
                    width: sprawl::style::Dimension::Points(100f32),
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
                flex_direction: sprawl::style::FlexDirection::Column,
                size: sprawl::geometry::Size { width: sprawl::style::Dimension::Points(100f32), ..Default::default() },
                ..Default::default()
            },
            &[node000],
        )
        .unwrap();
    let node0 = stretch
        .new_node(
            sprawl::style::Style {
                flex_wrap: sprawl::style::FlexWrap::Wrap,
                align_items: sprawl::style::AlignItems::FlexStart,
                ..Default::default()
            },
            &[node00],
        )
        .unwrap();
    let node1 = stretch
        .new_node(
            sprawl::style::Style {
                size: sprawl::geometry::Size {
                    width: sprawl::style::Dimension::Points(100f32),
                    height: sprawl::style::Dimension::Points(100f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node = stretch
        .new_node(
            sprawl::style::Style { flex_direction: sprawl::style::FlexDirection::Column, ..Default::default() },
            &[node0, node1],
        )
        .unwrap();
    stretch.compute_layout(node, sprawl::geometry::Size::undefined()).unwrap();
    assert_eq!(stretch.layout(node).unwrap().size.width, 100f32);
    assert_eq!(stretch.layout(node).unwrap().size.height, 200f32);
    assert_eq!(stretch.layout(node).unwrap().location.x, 0f32);
    assert_eq!(stretch.layout(node).unwrap().location.y, 0f32);
    assert_eq!(stretch.layout(node0).unwrap().size.width, 100f32);
    assert_eq!(stretch.layout(node0).unwrap().size.height, 100f32);
    assert_eq!(stretch.layout(node0).unwrap().location.x, 0f32);
    assert_eq!(stretch.layout(node0).unwrap().location.y, 0f32);
    assert_eq!(stretch.layout(node00).unwrap().size.width, 100f32);
    assert_eq!(stretch.layout(node00).unwrap().size.height, 100f32);
    assert_eq!(stretch.layout(node00).unwrap().location.x, 0f32);
    assert_eq!(stretch.layout(node00).unwrap().location.y, 0f32);
    assert_eq!(stretch.layout(node000).unwrap().size.width, 100f32);
    assert_eq!(stretch.layout(node000).unwrap().size.height, 100f32);
    assert_eq!(stretch.layout(node000).unwrap().location.x, 0f32);
    assert_eq!(stretch.layout(node000).unwrap().location.y, 0f32);
    assert_eq!(stretch.layout(node1).unwrap().size.width, 100f32);
    assert_eq!(stretch.layout(node1).unwrap().size.height, 100f32);
    assert_eq!(stretch.layout(node1).unwrap().location.x, 0f32);
    assert_eq!(stretch.layout(node1).unwrap().location.y, 100f32);
}
