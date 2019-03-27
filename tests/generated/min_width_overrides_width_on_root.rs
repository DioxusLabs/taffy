#[test]
fn min_width_overrides_width_on_root() {
    let layout = stretch::compute(
        &stretch::style::Node {
            size: stretch::geometry::Size { width: stretch::style::Dimension::Points(50f32), ..Default::default() },
            min_size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100f32),
                ..Default::default()
            },
            ..Default::default()
        },
        stretch::geometry::Size::undefined(),
    )
    .unwrap();
    assert_eq!(layout.size.width, 100f32);
    assert_eq!(layout.size.height, 0f32);
    assert_eq!(layout.location.x, 0f32);
    assert_eq!(layout.location.y, 0f32);
}
