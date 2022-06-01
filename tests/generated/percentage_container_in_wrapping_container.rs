#[test]
fn percentage_container_in_wrapping_container() {
    let mut sprawl = sprawl::Sprawl::new();
    let node000 = sprawl
        .new_node(
            sprawl::style::Style {
                size: sprawl::geometry::Size {
                    width: sprawl::style::Dimension::Points(50f32),
                    height: sprawl::style::Dimension::Points(50f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node001 = sprawl
        .new_node(
            sprawl::style::Style {
                size: sprawl::geometry::Size {
                    width: sprawl::style::Dimension::Points(50f32),
                    height: sprawl::style::Dimension::Points(50f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node00 = sprawl
        .new_node(
            sprawl::style::Style {
                justify_content: sprawl::style::JustifyContent::Center,
                size: sprawl::geometry::Size { width: sprawl::style::Dimension::Percent(1f32), ..Default::default() },
                ..Default::default()
            },
            &[node000, node001],
        )
        .unwrap();
    let node0 = sprawl
        .new_node(
            sprawl::style::Style { flex_direction: sprawl::style::FlexDirection::Column, ..Default::default() },
            &[node00],
        )
        .unwrap();
    let node = sprawl
        .new_node(
            sprawl::style::Style {
                flex_direction: sprawl::style::FlexDirection::Column,
                align_items: sprawl::style::AlignItems::Center,
                justify_content: sprawl::style::JustifyContent::Center,
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
    assert_eq!(sprawl.layout(node0).unwrap().size.width, 100f32);
    assert_eq!(sprawl.layout(node0).unwrap().size.height, 50f32);
    assert_eq!(sprawl.layout(node0).unwrap().location.x, 50f32);
    assert_eq!(sprawl.layout(node0).unwrap().location.y, 75f32);
    assert_eq!(sprawl.layout(node00).unwrap().size.width, 100f32);
    assert_eq!(sprawl.layout(node00).unwrap().size.height, 50f32);
    assert_eq!(sprawl.layout(node00).unwrap().location.x, 0f32);
    assert_eq!(sprawl.layout(node00).unwrap().location.y, 0f32);
    assert_eq!(sprawl.layout(node000).unwrap().size.width, 50f32);
    assert_eq!(sprawl.layout(node000).unwrap().size.height, 50f32);
    assert_eq!(sprawl.layout(node000).unwrap().location.x, 0f32);
    assert_eq!(sprawl.layout(node000).unwrap().location.y, 0f32);
    assert_eq!(sprawl.layout(node001).unwrap().size.width, 50f32);
    assert_eq!(sprawl.layout(node001).unwrap().size.height, 50f32);
    assert_eq!(sprawl.layout(node001).unwrap().location.x, 50f32);
    assert_eq!(sprawl.layout(node001).unwrap().location.y, 0f32);
}
