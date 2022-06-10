#[test]
fn percentage_flex_basis_cross_min_height() {
    let mut sprawl = sprawl::Sprawl::new();
    let node0 = sprawl
        .new_node(
            sprawl::style::FlexboxLayout {
                flex_grow: 1f32,
                min_size: sprawl::geometry::Size {
                    height: sprawl::style::Dimension::Percent(0.6f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node1 = sprawl
        .new_node(
            sprawl::style::FlexboxLayout {
                flex_grow: 2f32,
                min_size: sprawl::geometry::Size {
                    height: sprawl::style::Dimension::Percent(0.1f32),
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
                flex_direction: sprawl::style::FlexDirection::Column,
                size: sprawl::geometry::Size {
                    width: sprawl::style::Dimension::Points(200f32),
                    height: sprawl::style::Dimension::Points(400f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node0, node1],
        )
        .unwrap();
    sprawl.compute_layout(node, sprawl::geometry::Size::undefined()).unwrap();
    assert_eq!(sprawl.layout(node).unwrap().size.width, 200f32);
    assert_eq!(sprawl.layout(node).unwrap().size.height, 400f32);
    assert_eq!(sprawl.layout(node).unwrap().location.x, 0f32);
    assert_eq!(sprawl.layout(node).unwrap().location.y, 0f32);
    assert_eq!(sprawl.layout(node0).unwrap().size.width, 200f32);
    assert_eq!(sprawl.layout(node0).unwrap().size.height, 280f32);
    assert_eq!(sprawl.layout(node0).unwrap().location.x, 0f32);
    assert_eq!(sprawl.layout(node0).unwrap().location.y, 0f32);
    assert_eq!(sprawl.layout(node1).unwrap().size.width, 200f32);
    assert_eq!(sprawl.layout(node1).unwrap().size.height, 120f32);
    assert_eq!(sprawl.layout(node1).unwrap().location.x, 0f32);
    assert_eq!(sprawl.layout(node1).unwrap().location.y, 280f32);
}
