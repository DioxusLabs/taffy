#[test]
fn rounding_fractial_input_1() {
    let mut sprawl = sprawl::Sprawl::new();
    let node0 = sprawl
        .new_node(
            sprawl::style::Style {
                flex_grow: 1f32,
                flex_basis: sprawl::style::Dimension::Points(50f32),
                size: sprawl::geometry::Size { height: sprawl::style::Dimension::Points(20f32), ..Default::default() },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node1 = sprawl
        .new_node(
            sprawl::style::Style {
                flex_grow: 1f32,
                size: sprawl::geometry::Size { height: sprawl::style::Dimension::Points(10f32), ..Default::default() },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node2 = sprawl
        .new_node(
            sprawl::style::Style {
                flex_grow: 1f32,
                size: sprawl::geometry::Size { height: sprawl::style::Dimension::Points(10f32), ..Default::default() },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node = sprawl
        .new_node(
            sprawl::style::Style {
                flex_direction: sprawl::style::FlexDirection::Column,
                size: sprawl::geometry::Size {
                    width: sprawl::style::Dimension::Points(100f32),
                    height: sprawl::style::Dimension::Points(113.4f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node0, node1, node2],
        )
        .unwrap();
    sprawl.compute_layout(node, sprawl::geometry::Size::undefined()).unwrap();
    assert_eq!(sprawl.layout(node).unwrap().size.width, 100f32);
    assert_eq!(sprawl.layout(node).unwrap().size.height, 113f32);
    assert_eq!(sprawl.layout(node).unwrap().location.x, 0f32);
    assert_eq!(sprawl.layout(node).unwrap().location.y, 0f32);
    assert_eq!(sprawl.layout(node0).unwrap().size.width, 100f32);
    assert_eq!(sprawl.layout(node0).unwrap().size.height, 64f32);
    assert_eq!(sprawl.layout(node0).unwrap().location.x, 0f32);
    assert_eq!(sprawl.layout(node0).unwrap().location.y, 0f32);
    assert_eq!(sprawl.layout(node1).unwrap().size.width, 100f32);
    assert_eq!(sprawl.layout(node1).unwrap().size.height, 25f32);
    assert_eq!(sprawl.layout(node1).unwrap().location.x, 0f32);
    assert_eq!(sprawl.layout(node1).unwrap().location.y, 64f32);
    assert_eq!(sprawl.layout(node2).unwrap().size.width, 100f32);
    assert_eq!(sprawl.layout(node2).unwrap().size.height, 24f32);
    assert_eq!(sprawl.layout(node2).unwrap().location.x, 0f32);
    assert_eq!(sprawl.layout(node2).unwrap().location.y, 89f32);
}
