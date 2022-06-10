#[test]
fn flex_grow_in_at_most_container() {
    let mut sprawl = sprawl::Sprawl::new();
    let node00 = sprawl
        .new_node(
            sprawl::style::FlexboxLayout {
                flex_grow: 1f32,
                flex_basis: sprawl::style::Dimension::Points(0f32),
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node0 = sprawl.new_node(sprawl::style::FlexboxLayout { ..Default::default() }, &[node00]).unwrap();
    let node = sprawl
        .new_node(
            sprawl::style::FlexboxLayout {
                align_items: sprawl::style::AlignItems::FlexStart,
                size: sprawl::geometry::Size {
                    width: sprawl::style::Dimension::Points(100f32),
                    height: sprawl::style::Dimension::Points(100f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node0],
        )
        .unwrap();
    sprawl.compute_layout(node, sprawl::geometry::Size::undefined()).unwrap();
    assert_eq!(sprawl.layout(node).unwrap().size.width, 100f32);
    assert_eq!(sprawl.layout(node).unwrap().size.height, 100f32);
    assert_eq!(sprawl.layout(node).unwrap().location.x, 0f32);
    assert_eq!(sprawl.layout(node).unwrap().location.y, 0f32);
    assert_eq!(sprawl.layout(node0).unwrap().size.width, 0f32);
    assert_eq!(sprawl.layout(node0).unwrap().size.height, 0f32);
    assert_eq!(sprawl.layout(node0).unwrap().location.x, 0f32);
    assert_eq!(sprawl.layout(node0).unwrap().location.y, 0f32);
    assert_eq!(sprawl.layout(node00).unwrap().size.width, 0f32);
    assert_eq!(sprawl.layout(node00).unwrap().size.height, 0f32);
    assert_eq!(sprawl.layout(node00).unwrap().location.x, 0f32);
    assert_eq!(sprawl.layout(node00).unwrap().location.y, 0f32);
}
