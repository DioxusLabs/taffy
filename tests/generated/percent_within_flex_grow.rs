#[test]
fn percent_within_flex_grow() {
    let mut sprawl = sprawl::Sprawl::new();
    let node0 = sprawl
        .new_node(
            sprawl::style::FlexboxLayout {
                size: sprawl::geometry::Size { width: sprawl::style::Dimension::Points(100f32), ..Default::default() },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node10 = sprawl
        .new_node(
            sprawl::style::FlexboxLayout {
                size: sprawl::geometry::Size { width: sprawl::style::Dimension::Percent(1f32), ..Default::default() },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node1 = sprawl
        .new_node(
            sprawl::style::FlexboxLayout {
                flex_direction: sprawl::style::FlexDirection::Column,
                flex_grow: 1f32,
                ..Default::default()
            },
            &[node10],
        )
        .unwrap();
    let node2 = sprawl
        .new_node(
            sprawl::style::FlexboxLayout {
                size: sprawl::geometry::Size { width: sprawl::style::Dimension::Points(100f32), ..Default::default() },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node = sprawl
        .new_node(
            sprawl::style::FlexboxLayout {
                size: sprawl::geometry::Size {
                    width: sprawl::style::Dimension::Points(350f32),
                    height: sprawl::style::Dimension::Points(100f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node0, node1, node2],
        )
        .unwrap();
    sprawl.compute_layout(node, sprawl::geometry::Size::undefined()).unwrap();
    assert_eq!(sprawl.layout(node).unwrap().size.width, 350f32);
    assert_eq!(sprawl.layout(node).unwrap().size.height, 100f32);
    assert_eq!(sprawl.layout(node).unwrap().location.x, 0f32);
    assert_eq!(sprawl.layout(node).unwrap().location.y, 0f32);
    assert_eq!(sprawl.layout(node0).unwrap().size.width, 100f32);
    assert_eq!(sprawl.layout(node0).unwrap().size.height, 100f32);
    assert_eq!(sprawl.layout(node0).unwrap().location.x, 0f32);
    assert_eq!(sprawl.layout(node0).unwrap().location.y, 0f32);
    assert_eq!(sprawl.layout(node1).unwrap().size.width, 150f32);
    assert_eq!(sprawl.layout(node1).unwrap().size.height, 100f32);
    assert_eq!(sprawl.layout(node1).unwrap().location.x, 100f32);
    assert_eq!(sprawl.layout(node1).unwrap().location.y, 0f32);
    assert_eq!(sprawl.layout(node10).unwrap().size.width, 150f32);
    assert_eq!(sprawl.layout(node10).unwrap().size.height, 0f32);
    assert_eq!(sprawl.layout(node10).unwrap().location.x, 0f32);
    assert_eq!(sprawl.layout(node10).unwrap().location.y, 0f32);
    assert_eq!(sprawl.layout(node2).unwrap().size.width, 100f32);
    assert_eq!(sprawl.layout(node2).unwrap().size.height, 100f32);
    assert_eq!(sprawl.layout(node2).unwrap().location.x, 250f32);
    assert_eq!(sprawl.layout(node2).unwrap().location.y, 0f32);
}
