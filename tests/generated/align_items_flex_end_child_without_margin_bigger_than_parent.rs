#[test]
fn align_items_flex_end_child_without_margin_bigger_than_parent() {
    let mut sprawl = sprawl::Sprawl::new();
    let node00 = sprawl
        .new_node(
            sprawl::style::FlexboxLayout {
                size: sprawl::geometry::Size {
                    width: sprawl::style::Dimension::Points(70f32),
                    height: sprawl::style::Dimension::Points(70f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node0 = sprawl
        .new_node(
            sprawl::style::FlexboxLayout { align_items: sprawl::style::AlignItems::FlexEnd, ..Default::default() },
            &[node00],
        )
        .unwrap();
    let node = sprawl
        .new_node(
            sprawl::style::FlexboxLayout {
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
    sprawl.compute_layout(node, sprawl::geometry::Size::undefined()).unwrap();
    assert_eq!(sprawl.layout(node).unwrap().size.width, 50f32);
    assert_eq!(sprawl.layout(node).unwrap().size.height, 50f32);
    assert_eq!(sprawl.layout(node).unwrap().location.x, 0f32);
    assert_eq!(sprawl.layout(node).unwrap().location.y, 0f32);
    assert_eq!(sprawl.layout(node0).unwrap().size.width, 70f32);
    assert_eq!(sprawl.layout(node0).unwrap().size.height, 70f32);
    assert_eq!(sprawl.layout(node0).unwrap().location.x, -10f32);
    assert_eq!(sprawl.layout(node0).unwrap().location.y, -10f32);
    assert_eq!(sprawl.layout(node00).unwrap().size.width, 70f32);
    assert_eq!(sprawl.layout(node00).unwrap().size.height, 70f32);
    assert_eq!(sprawl.layout(node00).unwrap().location.x, 0f32);
    assert_eq!(sprawl.layout(node00).unwrap().location.y, 0f32);
}
