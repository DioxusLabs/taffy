#[test]
fn display_none() {
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy.new_node(taffy::style::Style { flex_grow: 1f32, ..Default::default() }, &[]).unwrap();
    let node1 = taffy
        .new_node(
            taffy::style::Style { display: taffy::style::Display::None, flex_grow: 1f32, ..Default::default() },
            &[],
        )
        .unwrap();
    let node = taffy
        .new_node(
            taffy::style::Style {
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(100f32),
                    height: taffy::style::Dimension::Points(100f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node0, node1],
        )
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::undefined()).unwrap();
    assert_eq!(taffy.layout(node).unwrap().size.width, 100f32);
    assert_eq!(taffy.layout(node).unwrap().size.height, 100f32);
    assert_eq!(taffy.layout(node).unwrap().location.x, 0f32);
    assert_eq!(taffy.layout(node).unwrap().location.y, 0f32);
    assert_eq!(taffy.layout(node0).unwrap().size.width, 100f32);
    assert_eq!(taffy.layout(node0).unwrap().size.height, 100f32);
    assert_eq!(taffy.layout(node0).unwrap().location.x, 0f32);
    assert_eq!(taffy.layout(node0).unwrap().location.y, 0f32);
    assert_eq!(taffy.layout(node1).unwrap().size.width, 0f32);
    assert_eq!(taffy.layout(node1).unwrap().size.height, 0f32);
    assert_eq!(taffy.layout(node1).unwrap().location.x, 0f32);
    assert_eq!(taffy.layout(node1).unwrap().location.y, 0f32);
}
