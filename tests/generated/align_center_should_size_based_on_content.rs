#[test]
fn align_center_should_size_based_on_content() {
    let mut sprawl = sprawl::Sprawl::new();
    let node000 = sprawl
        .new_node(
            sprawl::style::Style {
                size: sprawl::geometry::Size {
                    width: sprawl::style::Dimension::Points(20f32),
                    height: sprawl::style::Dimension::Points(20f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node00 = sprawl
        .new_node(sprawl::style::Style { flex_grow: 1f32, flex_shrink: 1f32, ..Default::default() }, &[node000])
        .unwrap();
    let node0 = sprawl
        .new_node(
            sprawl::style::Style {
                justify_content: sprawl::style::JustifyContent::Center,
                flex_grow: 0f32,
                flex_shrink: 1f32,
                ..Default::default()
            },
            &[node00],
        )
        .unwrap();
    let node = sprawl
        .new_node(
            sprawl::style::Style {
                align_items: sprawl::style::AlignItems::Center,
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
    assert_eq!(sprawl.layout(node0).unwrap().size.width, 20f32);
    assert_eq!(sprawl.layout(node0).unwrap().size.height, 20f32);
    assert_eq!(sprawl.layout(node0).unwrap().location.x, 0f32);
    assert_eq!(sprawl.layout(node0).unwrap().location.y, 40f32);
    assert_eq!(sprawl.layout(node00).unwrap().size.width, 20f32);
    assert_eq!(sprawl.layout(node00).unwrap().size.height, 20f32);
    assert_eq!(sprawl.layout(node00).unwrap().location.x, 0f32);
    assert_eq!(sprawl.layout(node00).unwrap().location.y, 0f32);
    assert_eq!(sprawl.layout(node000).unwrap().size.width, 20f32);
    assert_eq!(sprawl.layout(node000).unwrap().size.height, 20f32);
    assert_eq!(sprawl.layout(node000).unwrap().location.x, 0f32);
    assert_eq!(sprawl.layout(node000).unwrap().location.y, 0f32);
}
