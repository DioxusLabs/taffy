#[test]
fn percentage_container_in_wrapping_container() {
    let mut taffy = taffy::Taffy::new();
    let node000 = taffy.new_with_children(
        taffy::style::FlexboxLayout {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Points(50f32),
                height: taffy::style::Dimension::Points(50f32),
                ..Default::default()
            },
            ..Default::default()
        },
        &[],
    );
    let node001 = taffy.new_with_children(
        taffy::style::FlexboxLayout {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Points(50f32),
                height: taffy::style::Dimension::Points(50f32),
                ..Default::default()
            },
            ..Default::default()
        },
        &[],
    );
    let node00 = taffy.new_with_children(
        taffy::style::FlexboxLayout {
            justify_content: taffy::style::JustifyContent::Center,
            size: taffy::geometry::Size { width: taffy::style::Dimension::Percent(1f32), ..Default::default() },
            ..Default::default()
        },
        &[node000, node001],
    );
    let node0 = taffy.new_with_children(
        taffy::style::FlexboxLayout { flex_direction: taffy::style::FlexDirection::Column, ..Default::default() },
        &[node00],
    );
    let node = taffy.new_with_children(
        taffy::style::FlexboxLayout {
            flex_direction: taffy::style::FlexDirection::Column,
            align_items: taffy::style::AlignItems::Center,
            justify_content: taffy::style::JustifyContent::Center,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Points(200f32),
                height: taffy::style::Dimension::Points(200f32),
                ..Default::default()
            },
            ..Default::default()
        },
        &[node0],
    );
    taffy.compute_layout(node, taffy::geometry::Size::undefined()).unwrap();
    assert_eq!(taffy.layout(node).unwrap().size.width, 200f32);
    assert_eq!(taffy.layout(node).unwrap().size.height, 200f32);
    assert_eq!(taffy.layout(node).unwrap().location.x, 0f32);
    assert_eq!(taffy.layout(node).unwrap().location.y, 0f32);
    assert_eq!(taffy.layout(node0).unwrap().size.width, 100f32);
    assert_eq!(taffy.layout(node0).unwrap().size.height, 50f32);
    assert_eq!(taffy.layout(node0).unwrap().location.x, 50f32);
    assert_eq!(taffy.layout(node0).unwrap().location.y, 75f32);
    assert_eq!(taffy.layout(node00).unwrap().size.width, 100f32);
    assert_eq!(taffy.layout(node00).unwrap().size.height, 50f32);
    assert_eq!(taffy.layout(node00).unwrap().location.x, 0f32);
    assert_eq!(taffy.layout(node00).unwrap().location.y, 0f32);
    assert_eq!(taffy.layout(node000).unwrap().size.width, 50f32);
    assert_eq!(taffy.layout(node000).unwrap().size.height, 50f32);
    assert_eq!(taffy.layout(node000).unwrap().location.x, 0f32);
    assert_eq!(taffy.layout(node000).unwrap().location.y, 0f32);
    assert_eq!(taffy.layout(node001).unwrap().size.width, 50f32);
    assert_eq!(taffy.layout(node001).unwrap().size.height, 50f32);
    assert_eq!(taffy.layout(node001).unwrap().location.x, 50f32);
    assert_eq!(taffy.layout(node001).unwrap().location.y, 0f32);
}
