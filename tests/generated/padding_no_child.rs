#[test]
fn padding_no_child() {
    let mut taffy = taffy::Taffy::new();
    let node = taffy
        .new_with_children(
            taffy::style::FlexboxLayout {
                padding: taffy::geometry::Rect {
                    start: taffy::style::Dimension::Points(10f32),
                    end: taffy::style::Dimension::Points(10f32),
                    top: taffy::style::Dimension::Points(10f32),
                    bottom: taffy::style::Dimension::Points(10f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::NONE).unwrap();
    assert_eq!(taffy.layout(node).unwrap().size.width, 20f32);
    assert_eq!(taffy.layout(node).unwrap().size.height, 20f32);
    assert_eq!(taffy.layout(node).unwrap().location.x, 0f32);
    assert_eq!(taffy.layout(node).unwrap().location.y, 0f32);
}
