#[test]
fn border_no_child() {
    let layout = stretch::compute(
        &stretch::style::Node {
            border: stretch::geometry::Rect {
                start: stretch::style::Dimension::Points(10.0000),
                end: stretch::style::Dimension::Points(10.0000),
                top: stretch::style::Dimension::Points(10.0000),
                bottom: stretch::style::Dimension::Points(10.0000),
                ..Default::default()
            },
            ..Default::default()
        },
        stretch::geometry::Size::undefined(),
    )
    .unwrap();

    assert_eq!(layout.size.width, 20.0000);
    assert_eq!(layout.size.height, 20.0000);
    assert_eq!(layout.location.x, 0.0000);
    assert_eq!(layout.location.y, 0.0000);
}
