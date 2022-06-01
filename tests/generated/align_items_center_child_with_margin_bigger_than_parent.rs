#[test]
fn align_items_center_child_with_margin_bigger_than_parent() {
    let mut stretch = sprawl::Sprawl::new();
    let node00 = stretch
        .new_node(
            sprawl::style::Style {
                size: sprawl::geometry::Size {
                    width: sprawl::style::Dimension::Points(50f32),
                    height: sprawl::style::Dimension::Points(50f32),
                    ..Default::default()
                },
                margin: sprawl::geometry::Rect {
                    start: sprawl::style::Dimension::Points(10f32),
                    end: sprawl::style::Dimension::Points(10f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node0 = stretch
        .new_node(
            sprawl::style::Style { align_items: sprawl::style::AlignItems::Center, ..Default::default() },
            &[node00],
        )
        .unwrap();
    let node = stretch
        .new_node(
            sprawl::style::Style {
                align_items: sprawl::style::AlignItems::Center,
                justify_content: sprawl::style::JustifyContent::Center,
                size: sprawl::geometry::Size {
                    width: sprawl::style::Dimension::Points(50f32),
                    height: sprawl::style::Dimension::Points(50f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node0],
        )
        .unwrap();
    stretch.compute_layout(node, sprawl::geometry::Size::undefined()).unwrap();
    assert_eq!(stretch.layout(node).unwrap().size.width, 50f32);
    assert_eq!(stretch.layout(node).unwrap().size.height, 50f32);
    assert_eq!(stretch.layout(node).unwrap().location.x, 0f32);
    assert_eq!(stretch.layout(node).unwrap().location.y, 0f32);
    assert_eq!(stretch.layout(node0).unwrap().size.width, 70f32);
    assert_eq!(stretch.layout(node0).unwrap().size.height, 50f32);
    assert_eq!(stretch.layout(node0).unwrap().location.x, -10f32);
    assert_eq!(stretch.layout(node0).unwrap().location.y, 0f32);
    assert_eq!(stretch.layout(node00).unwrap().size.width, 50f32);
    assert_eq!(stretch.layout(node00).unwrap().size.height, 50f32);
    assert_eq!(stretch.layout(node00).unwrap().location.x, 10f32);
    assert_eq!(stretch.layout(node00).unwrap().location.y, 0f32);
}
