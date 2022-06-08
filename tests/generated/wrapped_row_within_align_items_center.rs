#[test]
fn wrapped_row_within_align_items_center() {
    let mut sprawl = sprawl::Sprawl::new();
    let node00 = sprawl
        .new_node(
            sprawl::style::Style {
                size: sprawl::geometry::Size {
                    width: sprawl::style::Dimension::Points(150f32),
                    height: sprawl::style::Dimension::Points(80f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node01 = sprawl
        .new_node(
            sprawl::style::Style {
                size: sprawl::geometry::Size {
                    width: sprawl::style::Dimension::Points(80f32),
                    height: sprawl::style::Dimension::Points(80f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node0 = sprawl
        .new_node(
            sprawl::style::Style { flex_wrap: sprawl::style::FlexWrap::Wrap, ..Default::default() },
            &[node00, node01],
        )
        .unwrap();
    let node = sprawl
        .new_node(
            sprawl::style::Style {
                flex_direction: sprawl::style::FlexDirection::Column,
                align_items: sprawl::style::AlignItems::Center,
                size: sprawl::geometry::Size {
                    width: sprawl::style::Dimension::Points(200f32),
                    height: sprawl::style::Dimension::Points(200f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node0],
        )
        .unwrap();
    sprawl.compute_layout(node, sprawl::geometry::Size::undefined()).unwrap();
    assert_eq!(sprawl.layout(node).unwrap().size.width, 200f32);
    assert_eq!(sprawl.layout(node).unwrap().size.height, 200f32);
    assert_eq!(sprawl.layout(node).unwrap().location.x, 0f32);
    assert_eq!(sprawl.layout(node).unwrap().location.y, 0f32);
    assert_eq!(sprawl.layout(node0).unwrap().size.width, 200f32);
    assert_eq!(sprawl.layout(node0).unwrap().size.height, 160f32);
    assert_eq!(sprawl.layout(node0).unwrap().location.x, 0f32);
    assert_eq!(sprawl.layout(node0).unwrap().location.y, 0f32);
    assert_eq!(sprawl.layout(node00).unwrap().size.width, 150f32);
    assert_eq!(sprawl.layout(node00).unwrap().size.height, 80f32);
    assert_eq!(sprawl.layout(node00).unwrap().location.x, 0f32);
    assert_eq!(sprawl.layout(node00).unwrap().location.y, 0f32);
    assert_eq!(sprawl.layout(node01).unwrap().size.width, 80f32);
    assert_eq!(sprawl.layout(node01).unwrap().size.height, 80f32);
    assert_eq!(sprawl.layout(node01).unwrap().location.x, 0f32);
    assert_eq!(sprawl.layout(node01).unwrap().location.y, 80f32);
}
