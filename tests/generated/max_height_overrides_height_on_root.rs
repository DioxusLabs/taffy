#[test]
fn max_height_overrides_height_on_root() {
    let layout = stretch::node::Node::new(
        stretch::style::Style {
            size: stretch::geometry::Size { height: stretch::style::Dimension::Points(200f32), ..Default::default() },
            max_size: stretch::geometry::Size {
                height: stretch::style::Dimension::Points(100f32),
                ..Default::default()
            },
            ..Default::default()
        },
        vec![],
    )
    .compute_layout(stretch::geometry::Size::undefined())
    .unwrap();
    assert_eq!(layout.size.width, 0f32);
    assert_eq!(layout.size.height, 100f32);
    assert_eq!(layout.location.x, 0f32);
    assert_eq!(layout.location.y, 0f32);
}
