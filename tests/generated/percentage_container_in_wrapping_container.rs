#[test]
fn percentage_container_in_wrapping_container() {
    let mut stretch = stretch::Stretch::new();
    let node000 = stretch.new_node(
        stretch::style::Style {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(50f32),
                height: stretch::style::Dimension::Points(50f32),
                ..Default::default()
            },
            ..Default::default()
        },
        vec![],
    );
    let node001 = stretch.new_node(
        stretch::style::Style {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(50f32),
                height: stretch::style::Dimension::Points(50f32),
                ..Default::default()
            },
            ..Default::default()
        },
        vec![],
    );
    let node00 = stretch.new_node(
        stretch::style::Style {
            justify_content: stretch::style::JustifyContent::Center,
            size: stretch::geometry::Size { width: stretch::style::Dimension::Percent(1f32), ..Default::default() },
            ..Default::default()
        },
        vec![node000, node001],
    );
    let node0 = stretch.new_node(
        stretch::style::Style { flex_direction: stretch::style::FlexDirection::Column, ..Default::default() },
        vec![node00],
    );
    let node = stretch.new_node(
        stretch::style::Style {
            flex_direction: stretch::style::FlexDirection::Column,
            align_items: stretch::style::AlignItems::Center,
            justify_content: stretch::style::JustifyContent::Center,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(200f32),
                height: stretch::style::Dimension::Points(200f32),
                ..Default::default()
            },
            ..Default::default()
        },
        vec![node0],
    );
    stretch.compute_layout(node, stretch::geometry::Size::undefined()).unwrap();
    assert_eq!(stretch.layout(node).size.width, 200f32);
    assert_eq!(stretch.layout(node).size.height, 200f32);
    assert_eq!(stretch.layout(node).location.x, 0f32);
    assert_eq!(stretch.layout(node).location.y, 0f32);
    assert_eq!(stretch.layout(node0).size.width, 100f32);
    assert_eq!(stretch.layout(node0).size.height, 50f32);
    assert_eq!(stretch.layout(node0).location.x, 50f32);
    assert_eq!(stretch.layout(node0).location.y, 75f32);
    assert_eq!(stretch.layout(node00).size.width, 100f32);
    assert_eq!(stretch.layout(node00).size.height, 50f32);
    assert_eq!(stretch.layout(node00).location.x, 0f32);
    assert_eq!(stretch.layout(node00).location.y, 0f32);
    assert_eq!(stretch.layout(node000).size.width, 50f32);
    assert_eq!(stretch.layout(node000).size.height, 50f32);
    assert_eq!(stretch.layout(node000).location.x, 0f32);
    assert_eq!(stretch.layout(node000).location.y, 0f32);
    assert_eq!(stretch.layout(node001).size.width, 50f32);
    assert_eq!(stretch.layout(node001).size.height, 50f32);
    assert_eq!(stretch.layout(node001).location.x, 50f32);
    assert_eq!(stretch.layout(node001).location.y, 0f32);
}
