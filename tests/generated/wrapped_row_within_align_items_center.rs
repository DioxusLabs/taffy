#[test]
fn wrapped_row_within_align_items_center() {
    let mut stretch = stretch::Stretch::new();
    let node00 = stretch.new_node(
        stretch::style::Style {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(150f32),
                height: stretch::style::Dimension::Points(80f32),
                ..Default::default()
            },
            ..Default::default()
        },
        vec![],
    );
    let node01 = stretch.new_node(
        stretch::style::Style {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(80f32),
                height: stretch::style::Dimension::Points(80f32),
                ..Default::default()
            },
            ..Default::default()
        },
        vec![],
    );
    let node0 = stretch.new_node(
        stretch::style::Style { flex_wrap: stretch::style::FlexWrap::Wrap, ..Default::default() },
        vec![node00, node01],
    );
    let node = stretch.new_node(
        stretch::style::Style {
            flex_direction: stretch::style::FlexDirection::Column,
            align_items: stretch::style::AlignItems::Center,
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
    assert_eq!(stretch.layout(node0).size.width, 200f32);
    assert_eq!(stretch.layout(node0).size.height, 160f32);
    assert_eq!(stretch.layout(node0).location.x, 0f32);
    assert_eq!(stretch.layout(node0).location.y, 0f32);
    assert_eq!(stretch.layout(node00).size.width, 150f32);
    assert_eq!(stretch.layout(node00).size.height, 80f32);
    assert_eq!(stretch.layout(node00).location.x, 0f32);
    assert_eq!(stretch.layout(node00).location.y, 0f32);
    assert_eq!(stretch.layout(node01).size.width, 80f32);
    assert_eq!(stretch.layout(node01).size.height, 80f32);
    assert_eq!(stretch.layout(node01).location.x, 0f32);
    assert_eq!(stretch.layout(node01).location.y, 80f32);
}
