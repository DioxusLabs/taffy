#[test]
fn justify_content_row_max_width_and_margin() {
    let mut sprawl = sprawl::Sprawl::new();
    let node0 = sprawl
        .new_node(
            sprawl::style::FlexboxLayout {
                size: sprawl::geometry::Size {
                    width: sprawl::style::Dimension::Points(20f32),
                    height: sprawl::style::Dimension::Points(20f32),
                    ..Default::default()
                },
                margin: sprawl::geometry::Rect {
                    start: sprawl::style::Dimension::Points(100f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node = sprawl
        .new_node(
            sprawl::style::FlexboxLayout {
                justify_content: sprawl::style::JustifyContent::Center,
                size: sprawl::geometry::Size { width: sprawl::style::Dimension::Points(100f32), ..Default::default() },
                max_size: sprawl::geometry::Size {
                    width: sprawl::style::Dimension::Points(80f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node0],
        )
        .unwrap();
    sprawl.compute_layout(node, sprawl::geometry::Size::undefined()).unwrap();
    assert_eq!(sprawl.layout(node).unwrap().size.width, 80f32);
    assert_eq!(sprawl.layout(node).unwrap().size.height, 20f32);
    assert_eq!(sprawl.layout(node).unwrap().location.x, 0f32);
    assert_eq!(sprawl.layout(node).unwrap().location.y, 0f32);
    assert_eq!(sprawl.layout(node0).unwrap().size.width, 0f32);
    assert_eq!(sprawl.layout(node0).unwrap().size.height, 20f32);
    assert_eq!(sprawl.layout(node0).unwrap().location.x, 90f32);
    assert_eq!(sprawl.layout(node0).unwrap().location.y, 0f32);
}
