#[test]
fn padding_no_child() {
    let layout = stretch::compute(
        &stretch::style::Node {
            padding: stretch::geometry::Rect {
                start: stretch::style::Dimension::Points(10f32),
                end: stretch::style::Dimension::Points(10f32),
                top: stretch::style::Dimension::Points(10f32),
                bottom: stretch::style::Dimension::Points(10f32),
                ..Default::default()
            },
            ..Default::default()
        },
        stretch::geometry::Size::undefined(),
    )
    .unwrap();
    assert_eq!(layout.size.width, 20f32);
    assert_eq!(layout.size.height, 20f32);
    assert_eq!(layout.location.x, 0f32);
    assert_eq!(layout.location.y, 0f32);
}
