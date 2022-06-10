#[test]
fn percentage_size_based_on_parent_inner_size() {
    let mut sprawl = sprawl::Sprawl::new();
    let node0 = sprawl
        .new_node(
            sprawl::style::FlexboxLayout {
                size: sprawl::geometry::Size {
                    width: sprawl::style::Dimension::Percent(0.5f32),
                    height: sprawl::style::Dimension::Percent(0.5f32),
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
                padding: sprawl::geometry::Rect {
                    start: sprawl::style::Dimension::Points(20f32),
                    end: sprawl::style::Dimension::Points(20f32),
                    top: sprawl::style::Dimension::Points(20f32),
                    bottom: sprawl::style::Dimension::Points(20f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node0],
        )
        .unwrap();
    sprawl.compute_layout(node, sprawl::geometry::Size::undefined()).unwrap();
    assert_eq!(sprawl.layout(node).unwrap().size.width, 200f32);
    assert_eq!(sprawl.layout(node).unwrap().size.height, 400f32);
    assert_eq!(sprawl.layout(node).unwrap().location.x, 0f32);
    assert_eq!(sprawl.layout(node).unwrap().location.y, 0f32);
    assert_eq!(sprawl.layout(node0).unwrap().size.width, 80f32);
    assert_eq!(sprawl.layout(node0).unwrap().size.height, 180f32);
    assert_eq!(sprawl.layout(node0).unwrap().location.x, 20f32);
    assert_eq!(sprawl.layout(node0).unwrap().location.y, 20f32);
}
