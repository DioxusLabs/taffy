#[test]
fn min_width_overrides_width_on_root() {
    let layout = stretch::compute(
        &stretch::style::Node {
            size: stretch::geometry::Size { width: stretch::style::Dimension::Points(50.0000), ..Default::default() },
            min_size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            ..Default::default()
        },
        stretch::geometry::Size::undefined(),
    )
    .unwrap();

    assert_eq!(layout.size.width, 100.0000);
    assert_eq!(layout.size.height, 0.0000);
    assert_eq!(layout.location.x, 0.0000);
    assert_eq!(layout.location.y, 0.0000);
}
