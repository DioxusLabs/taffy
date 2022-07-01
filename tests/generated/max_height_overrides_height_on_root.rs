#[test]
fn max_height_overrides_height_on_root() {
    let mut taffy = taffy::Taffy::new();
    let node = taffy.new_with_children(
        taffy::style::FlexboxLayout {
            size: taffy::geometry::Size { height: taffy::style::Dimension::Points(200f32), ..Default::default() },
            max_size: taffy::geometry::Size { height: taffy::style::Dimension::Points(100f32), ..Default::default() },
            ..Default::default()
        },
        &[],
    );
    taffy.compute_layout(node, taffy::geometry::Size::undefined()).unwrap();
    assert_eq!(taffy.layout(node).unwrap().size.width, 0f32);
    assert_eq!(taffy.layout(node).unwrap().size.height, 100f32);
    assert_eq!(taffy.layout(node).unwrap().location.x, 0f32);
    assert_eq!(taffy.layout(node).unwrap().location.y, 0f32);
}
