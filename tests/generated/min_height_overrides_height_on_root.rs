#[test]
fn min_height_overrides_height_on_root() {
    let mut sprawl = sprawl::Sprawl::new();
    let node = sprawl
        .new_node(
            sprawl::style::FlexboxLayout {
                size: sprawl::geometry::Size { height: sprawl::style::Dimension::Points(50f32), ..Default::default() },
                min_size: sprawl::geometry::Size {
                    height: sprawl::style::Dimension::Points(100f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    sprawl.compute_layout(node, sprawl::geometry::Size::undefined()).unwrap();
    assert_eq!(sprawl.layout(node).unwrap().size.width, 0f32);
    assert_eq!(sprawl.layout(node).unwrap().size.height, 100f32);
    assert_eq!(sprawl.layout(node).unwrap().location.x, 0f32);
    assert_eq!(sprawl.layout(node).unwrap().location.y, 0f32);
}
