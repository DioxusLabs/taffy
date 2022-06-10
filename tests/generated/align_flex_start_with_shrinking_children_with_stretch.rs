#[test]
fn align_flex_start_with_shrinking_children_with_stretch() {
    let mut taffy = taffy::Taffy::new();
    let node000 =
        taffy.new_node(taffy::style::Style { flex_grow: 1f32, flex_shrink: 1f32, ..Default::default() }, &[]).unwrap();
    let node00 = taffy
        .new_node(taffy::style::Style { flex_grow: 1f32, flex_shrink: 1f32, ..Default::default() }, &[node000])
        .unwrap();
    let node0 = taffy
        .new_node(
            taffy::style::Style { align_items: taffy::style::AlignItems::FlexStart, ..Default::default() },
            &[node00],
        )
        .unwrap();
    let node = taffy
        .new_node(
            taffy::style::Style {
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(500f32),
                    height: taffy::style::Dimension::Points(500f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node0],
        )
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::undefined()).unwrap();
    assert_eq!(taffy.layout(node).unwrap().size.width, 500f32);
    assert_eq!(taffy.layout(node).unwrap().size.height, 500f32);
    assert_eq!(taffy.layout(node).unwrap().location.x, 0f32);
    assert_eq!(taffy.layout(node).unwrap().location.y, 0f32);
    assert_eq!(taffy.layout(node0).unwrap().size.width, 0f32);
    assert_eq!(taffy.layout(node0).unwrap().size.height, 500f32);
    assert_eq!(taffy.layout(node0).unwrap().location.x, 0f32);
    assert_eq!(taffy.layout(node0).unwrap().location.y, 0f32);
    assert_eq!(taffy.layout(node00).unwrap().size.width, 0f32);
    assert_eq!(taffy.layout(node00).unwrap().size.height, 0f32);
    assert_eq!(taffy.layout(node00).unwrap().location.x, 0f32);
    assert_eq!(taffy.layout(node00).unwrap().location.y, 0f32);
    assert_eq!(taffy.layout(node000).unwrap().size.width, 0f32);
    assert_eq!(taffy.layout(node000).unwrap().size.height, 0f32);
    assert_eq!(taffy.layout(node000).unwrap().location.x, 0f32);
    assert_eq!(taffy.layout(node000).unwrap().location.y, 0f32);
}
