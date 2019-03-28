#[test]
fn max_height_overrides_height_on_root() {
    let layout = stretch::compute(
        &stretch::style::Node {
            size: stretch::geometry::Size { height: stretch::style::Dimension::Points(200.0000), ..Default::default() },
            max_size: stretch::geometry::Size {
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            ..Default::default()
        },
        stretch::geometry::Size::undefined(),
    )
    .unwrap();

    assert_eq!(layout.size.width, 0.0000);
    assert_eq!(layout.size.height, 100.0000);
    assert_eq!(layout.location.x, 0.0000);
    assert_eq!(layout.location.y, 0.0000);
}
