#[test]
fn margin_fix_left_auto_right_child_bigger_than_parent() {
    let mut sprawl = sprawl::Sprawl::new();
    let node0 = sprawl
        .new_node(
            sprawl::style::FlexboxLayout {
                size: sprawl::geometry::Size {
                    width: sprawl::style::Dimension::Points(72f32),
                    height: sprawl::style::Dimension::Points(72f32),
                    ..Default::default()
                },
                margin: sprawl::geometry::Rect {
                    start: sprawl::style::Dimension::Points(10f32),
                    end: sprawl::style::Dimension::Auto,
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
                size: sprawl::geometry::Size {
                    width: sprawl::style::Dimension::Points(52f32),
                    height: sprawl::style::Dimension::Points(52f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node0],
        )
        .unwrap();
    sprawl.compute_layout(node, sprawl::geometry::Size::undefined()).unwrap();
    assert_eq!(sprawl.layout(node).unwrap().size.width, 52f32);
    assert_eq!(sprawl.layout(node).unwrap().size.height, 52f32);
    assert_eq!(sprawl.layout(node).unwrap().location.x, 0f32);
    assert_eq!(sprawl.layout(node).unwrap().location.y, 0f32);
    assert_eq!(sprawl.layout(node0).unwrap().size.width, 42f32);
    assert_eq!(sprawl.layout(node0).unwrap().size.height, 72f32);
    assert_eq!(sprawl.layout(node0).unwrap().location.x, 10f32);
    assert_eq!(sprawl.layout(node0).unwrap().location.y, 0f32);
}
