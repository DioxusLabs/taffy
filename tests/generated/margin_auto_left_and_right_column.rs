#[test]
fn margin_auto_left_and_right_column() {
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy.new_with_children(
        taffy::style::FlexboxLayout {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Points(50f32),
                height: taffy::style::Dimension::Points(50f32),
                ..Default::default()
            },
            margin: taffy::geometry::Rect {
                start: taffy::style::Dimension::Auto,
                end: taffy::style::Dimension::Auto,
                ..Default::default()
            },
            ..Default::default()
        },
        &[],
    );
    let node1 = taffy.new_with_children(
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
    let node = taffy.new_with_children(
        taffy::style::FlexboxLayout {
            align_items: taffy::style::AlignItems::Center,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Points(200f32),
                height: taffy::style::Dimension::Points(200f32),
                ..Default::default()
            },
            ..Default::default()
        },
        &[node0, node1],
    );
    taffy.compute_layout(node, taffy::geometry::Size::undefined()).unwrap();
    assert_eq!(taffy.layout(node).unwrap().size.width, 200f32);
    assert_eq!(taffy.layout(node).unwrap().size.height, 200f32);
    assert_eq!(taffy.layout(node).unwrap().location.x, 0f32);
    assert_eq!(taffy.layout(node).unwrap().location.y, 0f32);
    assert_eq!(taffy.layout(node0).unwrap().size.width, 50f32);
    assert_eq!(taffy.layout(node0).unwrap().size.height, 50f32);
    assert_eq!(taffy.layout(node0).unwrap().location.x, 50f32);
    assert_eq!(taffy.layout(node0).unwrap().location.y, 75f32);
    assert_eq!(taffy.layout(node1).unwrap().size.width, 50f32);
    assert_eq!(taffy.layout(node1).unwrap().size.height, 50f32);
    assert_eq!(taffy.layout(node1).unwrap().location.x, 150f32);
    assert_eq!(taffy.layout(node1).unwrap().location.y, 75f32);
}
