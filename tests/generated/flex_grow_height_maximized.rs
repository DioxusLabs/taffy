#[test]
fn flex_grow_height_maximized() {
    let mut taffy = taffy::Taffy::new();
    let node00 = taffy.new_with_children(
        taffy::style::FlexboxLayout {
            flex_grow: 1f32,
            flex_basis: taffy::style::Dimension::Points(200f32),
            ..Default::default()
        },
        &[],
    );
    let node01 = taffy.new_with_children(
        taffy::style::FlexboxLayout {
            size: taffy::geometry::Size { height: taffy::style::Dimension::Points(100f32), ..Default::default() },
            ..Default::default()
        },
        &[],
    );
    let node0 = taffy.new_with_children(
        taffy::style::FlexboxLayout {
            flex_direction: taffy::style::FlexDirection::Column,
            flex_grow: 1f32,
            min_size: taffy::geometry::Size { height: taffy::style::Dimension::Points(100f32), ..Default::default() },
            max_size: taffy::geometry::Size { height: taffy::style::Dimension::Points(500f32), ..Default::default() },
            ..Default::default()
        },
        &[node00, node01],
    );
    let node = taffy.new_with_children(
        taffy::style::FlexboxLayout {
            flex_direction: taffy::style::FlexDirection::Column,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Points(100f32),
                height: taffy::style::Dimension::Points(500f32),
                ..Default::default()
            },
            ..Default::default()
        },
        &[node0],
    );
    taffy.compute_layout(node, taffy::geometry::Size::undefined()).unwrap();
    assert_eq!(taffy.layout(node).unwrap().size.width, 100f32);
    assert_eq!(taffy.layout(node).unwrap().size.height, 500f32);
    assert_eq!(taffy.layout(node).unwrap().location.x, 0f32);
    assert_eq!(taffy.layout(node).unwrap().location.y, 0f32);
    assert_eq!(taffy.layout(node0).unwrap().size.width, 100f32);
    assert_eq!(taffy.layout(node0).unwrap().size.height, 500f32);
    assert_eq!(taffy.layout(node0).unwrap().location.x, 0f32);
    assert_eq!(taffy.layout(node0).unwrap().location.y, 0f32);
    assert_eq!(taffy.layout(node00).unwrap().size.width, 100f32);
    assert_eq!(taffy.layout(node00).unwrap().size.height, 400f32);
    assert_eq!(taffy.layout(node00).unwrap().location.x, 0f32);
    assert_eq!(taffy.layout(node00).unwrap().location.y, 0f32);
    assert_eq!(taffy.layout(node01).unwrap().size.width, 100f32);
    assert_eq!(taffy.layout(node01).unwrap().size.height, 100f32);
    assert_eq!(taffy.layout(node01).unwrap().location.x, 0f32);
    assert_eq!(taffy.layout(node01).unwrap().location.y, 400f32);
}
