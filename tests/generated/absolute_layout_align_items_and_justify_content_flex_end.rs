#[test]
fn absolute_layout_align_items_and_justify_content_flex_end() {
    let mut sprawl = sprawl::Sprawl::new();
    let node0 = sprawl
        .new_node(
            sprawl::style::FlexboxLayout {
                position_type: sprawl::style::PositionType::Absolute,
                size: sprawl::geometry::Size {
                    width: sprawl::style::Dimension::Points(60f32),
                    height: sprawl::style::Dimension::Points(40f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node = sprawl
        .new_node(
            sprawl::style::FlexboxLayout {
                align_items: sprawl::style::AlignItems::FlexEnd,
                justify_content: sprawl::style::JustifyContent::FlexEnd,
                size: sprawl::geometry::Size {
                    width: sprawl::style::Dimension::Points(110f32),
                    height: sprawl::style::Dimension::Points(100f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node0],
        )
        .unwrap();
    sprawl.compute_layout(node, sprawl::geometry::Size::undefined()).unwrap();
    assert_eq!(sprawl.layout(node).unwrap().size.width, 110f32);
    assert_eq!(sprawl.layout(node).unwrap().size.height, 100f32);
    assert_eq!(sprawl.layout(node).unwrap().location.x, 0f32);
    assert_eq!(sprawl.layout(node).unwrap().location.y, 0f32);
    assert_eq!(sprawl.layout(node0).unwrap().size.width, 60f32);
    assert_eq!(sprawl.layout(node0).unwrap().size.height, 40f32);
    assert_eq!(sprawl.layout(node0).unwrap().location.x, 50f32);
    assert_eq!(sprawl.layout(node0).unwrap().location.y, 60f32);
}
