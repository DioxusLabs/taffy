#[test]
fn display_none_with_child() {
    let mut sprawl = sprawl::Sprawl::new();
    let node0 = sprawl
        .new_node(
            sprawl::style::Style {
                flex_grow: 1f32,
                flex_shrink: 1f32,
                flex_basis: sprawl::style::Dimension::Percent(0f32),
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node10 = sprawl
        .new_node(
            sprawl::style::Style {
                flex_grow: 1f32,
                flex_shrink: 1f32,
                flex_basis: sprawl::style::Dimension::Percent(0f32),
                size: sprawl::geometry::Size { width: sprawl::style::Dimension::Points(20f32), ..Default::default() },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node1 = sprawl
        .new_node(
            sprawl::style::Style {
                display: sprawl::style::Display::None,
                flex_direction: sprawl::style::FlexDirection::Column,
                flex_grow: 1f32,
                flex_shrink: 1f32,
                flex_basis: sprawl::style::Dimension::Percent(0f32),
                ..Default::default()
            },
            &[node10],
        )
        .unwrap();
    let node2 = sprawl
        .new_node(
            sprawl::style::Style {
                flex_grow: 1f32,
                flex_shrink: 1f32,
                flex_basis: sprawl::style::Dimension::Percent(0f32),
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node = sprawl
        .new_node(
            sprawl::style::Style {
                size: sprawl::geometry::Size {
                    width: sprawl::style::Dimension::Points(100f32),
                    height: sprawl::style::Dimension::Points(100f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node0, node1, node2],
        )
        .unwrap();
    sprawl.compute_layout(node, sprawl::geometry::Size::undefined()).unwrap();
    assert_eq!(sprawl.layout(node).unwrap().size.width, 100f32);
    assert_eq!(sprawl.layout(node).unwrap().size.height, 100f32);
    assert_eq!(sprawl.layout(node).unwrap().location.x, 0f32);
    assert_eq!(sprawl.layout(node).unwrap().location.y, 0f32);
    assert_eq!(sprawl.layout(node0).unwrap().size.width, 50f32);
    assert_eq!(sprawl.layout(node0).unwrap().size.height, 100f32);
    assert_eq!(sprawl.layout(node0).unwrap().location.x, 0f32);
    assert_eq!(sprawl.layout(node0).unwrap().location.y, 0f32);
    assert_eq!(sprawl.layout(node1).unwrap().size.width, 0f32);
    assert_eq!(sprawl.layout(node1).unwrap().size.height, 0f32);
    assert_eq!(sprawl.layout(node1).unwrap().location.x, 0f32);
    assert_eq!(sprawl.layout(node1).unwrap().location.y, 0f32);
    assert_eq!(sprawl.layout(node10).unwrap().size.width, 0f32);
    assert_eq!(sprawl.layout(node10).unwrap().size.height, 0f32);
    assert_eq!(sprawl.layout(node10).unwrap().location.x, 0f32);
    assert_eq!(sprawl.layout(node10).unwrap().location.y, 0f32);
    assert_eq!(sprawl.layout(node2).unwrap().size.width, 50f32);
    assert_eq!(sprawl.layout(node2).unwrap().size.height, 100f32);
    assert_eq!(sprawl.layout(node2).unwrap().location.x, 50f32);
    assert_eq!(sprawl.layout(node2).unwrap().location.y, 0f32);
}
