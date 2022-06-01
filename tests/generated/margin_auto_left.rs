#[test]
fn margin_auto_left() {
    let mut stretch = sprawl::Sprawl::new();
    let node0 = stretch
        .new_node(
            sprawl::style::Style {
                size: sprawl::geometry::Size {
                    width: sprawl::style::Dimension::Points(50f32),
                    height: sprawl::style::Dimension::Points(50f32),
                    ..Default::default()
                },
                margin: sprawl::geometry::Rect { start: sprawl::style::Dimension::Auto, ..Default::default() },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node1 = stretch
        .new_node(
            sprawl::style::Style {
                size: sprawl::geometry::Size {
                    width: sprawl::style::Dimension::Points(50f32),
                    height: sprawl::style::Dimension::Points(50f32),
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
                align_items: sprawl::style::AlignItems::Center,
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
    assert_eq!(stretch.layout(node).unwrap().size.width, 200f32);
    assert_eq!(stretch.layout(node).unwrap().size.height, 200f32);
    assert_eq!(stretch.layout(node).unwrap().location.x, 0f32);
    assert_eq!(stretch.layout(node).unwrap().location.y, 0f32);
    assert_eq!(stretch.layout(node0).unwrap().size.width, 50f32);
    assert_eq!(stretch.layout(node0).unwrap().size.height, 50f32);
    assert_eq!(stretch.layout(node0).unwrap().location.x, 100f32);
    assert_eq!(stretch.layout(node0).unwrap().location.y, 75f32);
    assert_eq!(stretch.layout(node1).unwrap().size.width, 50f32);
    assert_eq!(stretch.layout(node1).unwrap().size.height, 50f32);
    assert_eq!(stretch.layout(node1).unwrap().location.x, 150f32);
    assert_eq!(stretch.layout(node1).unwrap().location.y, 75f32);
}
