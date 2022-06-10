#[test]
fn flex_grow_root_minimized() {
    let mut sprawl = sprawl::Sprawl::new();
    let node00 = sprawl
        .new_node(
            sprawl::style::FlexboxLayout {
                flex_grow: 1f32,
                flex_basis: sprawl::style::Dimension::Points(200f32),
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node01 = sprawl
        .new_node(
            sprawl::style::FlexboxLayout {
                size: sprawl::geometry::Size { height: sprawl::style::Dimension::Points(100f32), ..Default::default() },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node0 = sprawl
        .new_node(
            sprawl::style::FlexboxLayout {
                flex_direction: sprawl::style::FlexDirection::Column,
                flex_grow: 1f32,
                min_size: sprawl::geometry::Size {
                    height: sprawl::style::Dimension::Points(100f32),
                    ..Default::default()
                },
                max_size: sprawl::geometry::Size {
                    height: sprawl::style::Dimension::Points(500f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node00, node01],
        )
        .unwrap();
    let node = sprawl
        .new_node(
            sprawl::style::FlexboxLayout {
                flex_direction: sprawl::style::FlexDirection::Column,
                size: sprawl::geometry::Size { width: sprawl::style::Dimension::Points(100f32), ..Default::default() },
                min_size: sprawl::geometry::Size {
                    height: sprawl::style::Dimension::Points(100f32),
                    ..Default::default()
                },
                max_size: sprawl::geometry::Size {
                    height: sprawl::style::Dimension::Points(500f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node0],
        )
        .unwrap();
    sprawl.compute_layout(node, sprawl::geometry::Size::undefined()).unwrap();
    assert_eq!(sprawl.layout(node).unwrap().size.width, 100f32);
    assert_eq!(sprawl.layout(node).unwrap().size.height, 300f32);
    assert_eq!(sprawl.layout(node).unwrap().location.x, 0f32);
    assert_eq!(sprawl.layout(node).unwrap().location.y, 0f32);
    assert_eq!(sprawl.layout(node0).unwrap().size.width, 100f32);
    assert_eq!(sprawl.layout(node0).unwrap().size.height, 300f32);
    assert_eq!(sprawl.layout(node0).unwrap().location.x, 0f32);
    assert_eq!(sprawl.layout(node0).unwrap().location.y, 0f32);
    assert_eq!(sprawl.layout(node00).unwrap().size.width, 100f32);
    assert_eq!(sprawl.layout(node00).unwrap().size.height, 200f32);
    assert_eq!(sprawl.layout(node00).unwrap().location.x, 0f32);
    assert_eq!(sprawl.layout(node00).unwrap().location.y, 0f32);
    assert_eq!(sprawl.layout(node01).unwrap().size.width, 100f32);
    assert_eq!(sprawl.layout(node01).unwrap().size.height, 100f32);
    assert_eq!(sprawl.layout(node01).unwrap().location.x, 0f32);
    assert_eq!(sprawl.layout(node01).unwrap().location.y, 200f32);
}
