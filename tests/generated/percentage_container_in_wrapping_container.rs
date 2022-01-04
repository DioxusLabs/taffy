#[test]
fn percentage_container_in_wrapping_container() {
    let mut stretch = stretch2::Stretch::new();
    let node000 = stretch
        .new_node(
            stretch2::style::Style {
                size: stretch2::geometry::Size {
                    width: stretch2::style::Dimension::Points(50f32),
                    height: stretch2::style::Dimension::Points(50f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node001 = stretch
        .new_node(
            stretch2::style::Style {
                size: stretch2::geometry::Size {
                    width: stretch2::style::Dimension::Points(50f32),
                    height: stretch2::style::Dimension::Points(50f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node00 = stretch
        .new_node(
            stretch2::style::Style {
                justify_content: stretch2::style::JustifyContent::Center,
                size: stretch2::geometry::Size {
                    width: stretch2::style::Dimension::Percent(1f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node000, node001],
        )
        .unwrap();
    let node0 = stretch
        .new_node(
            stretch2::style::Style { flex_direction: stretch2::style::FlexDirection::Column, ..Default::default() },
            &[node00],
        )
        .unwrap();
    let node = stretch
        .new_node(
            stretch2::style::Style {
                flex_direction: stretch2::style::FlexDirection::Column,
                align_items: stretch2::style::AlignItems::Center,
                justify_content: stretch2::style::JustifyContent::Center,
                size: stretch2::geometry::Size {
                    width: stretch2::style::Dimension::Points(200f32),
                    height: stretch2::style::Dimension::Points(200f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node0],
        )
        .unwrap();
    stretch.compute_layout(node, stretch2::geometry::Size::undefined()).unwrap();
    assert_eq!(stretch.layout(node).unwrap().size.width, 200f32);
    assert_eq!(stretch.layout(node).unwrap().size.height, 200f32);
    assert_eq!(stretch.layout(node).unwrap().location.x, 0f32);
    assert_eq!(stretch.layout(node).unwrap().location.y, 0f32);
    assert_eq!(stretch.layout(node0).unwrap().size.width, 100f32);
    assert_eq!(stretch.layout(node0).unwrap().size.height, 50f32);
    assert_eq!(stretch.layout(node0).unwrap().location.x, 50f32);
    assert_eq!(stretch.layout(node0).unwrap().location.y, 75f32);
    assert_eq!(stretch.layout(node00).unwrap().size.width, 100f32);
    assert_eq!(stretch.layout(node00).unwrap().size.height, 50f32);
    assert_eq!(stretch.layout(node00).unwrap().location.x, 0f32);
    assert_eq!(stretch.layout(node00).unwrap().location.y, 0f32);
    assert_eq!(stretch.layout(node000).unwrap().size.width, 50f32);
    assert_eq!(stretch.layout(node000).unwrap().size.height, 50f32);
    assert_eq!(stretch.layout(node000).unwrap().location.x, 0f32);
    assert_eq!(stretch.layout(node000).unwrap().location.y, 0f32);
    assert_eq!(stretch.layout(node001).unwrap().size.width, 50f32);
    assert_eq!(stretch.layout(node001).unwrap().size.height, 50f32);
    assert_eq!(stretch.layout(node001).unwrap().location.x, 50f32);
    assert_eq!(stretch.layout(node001).unwrap().location.y, 0f32);
}
