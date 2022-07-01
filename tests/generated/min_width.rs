#[test]
fn min_width() {
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy.new_with_children(
        taffy::style::FlexboxLayout {
            flex_grow: 1f32,
            min_size: taffy::geometry::Size { width: taffy::style::Dimension::Points(60f32), ..Default::default() },
            ..Default::default()
        },
        &[],
    );
    let node1 = taffy.new_with_children(taffy::style::FlexboxLayout { flex_grow: 1f32, ..Default::default() }, &[]);
    let node = taffy.new_with_children(
        taffy::style::FlexboxLayout {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Points(100f32),
                height: taffy::style::Dimension::Points(100f32),
                ..Default::default()
            },
            ..Default::default()
        },
        &[node0, node1],
    );
    taffy.compute_layout(node, taffy::geometry::Size::undefined()).unwrap();
    assert_eq!(taffy.layout(node).unwrap().size.width, 100f32);
    assert_eq!(taffy.layout(node).unwrap().size.height, 100f32);
    assert_eq!(taffy.layout(node).unwrap().location.x, 0f32);
    assert_eq!(taffy.layout(node).unwrap().location.y, 0f32);
    assert_eq!(taffy.layout(node0).unwrap().size.width, 60f32);
    assert_eq!(taffy.layout(node0).unwrap().size.height, 100f32);
    assert_eq!(taffy.layout(node0).unwrap().location.x, 0f32);
    assert_eq!(taffy.layout(node0).unwrap().location.y, 0f32);
    assert_eq!(taffy.layout(node1).unwrap().size.width, 40f32);
    assert_eq!(taffy.layout(node1).unwrap().size.height, 100f32);
    assert_eq!(taffy.layout(node1).unwrap().location.x, 60f32);
    assert_eq!(taffy.layout(node1).unwrap().location.y, 0f32);
}
