#[test]
fn percentage_position_bottom_right() {
    let mut sprawl = sprawl::Sprawl::new();
    let node0 = sprawl
        .new_node(
            sprawl::style::Style {
                size: sprawl::geometry::Size {
                    width: sprawl::style::Dimension::Percent(0.55f32),
                    height: sprawl::style::Dimension::Percent(0.15f32),
                    ..Default::default()
                },
                position: sprawl::geometry::Rect {
                    end: sprawl::style::Dimension::Percent(0.2f32),
                    bottom: sprawl::style::Dimension::Percent(0.1f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node = sprawl
        .new_node(
            sprawl::style::Style {
                size: sprawl::geometry::Size {
                    width: sprawl::style::Dimension::Points(500f32),
                    height: sprawl::style::Dimension::Points(500f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node0],
        )
        .unwrap();
    sprawl.compute_layout(node, sprawl::geometry::Size::undefined()).unwrap();
    assert_eq!(sprawl.layout(node).unwrap().size.width, 500f32);
    assert_eq!(sprawl.layout(node).unwrap().size.height, 500f32);
    assert_eq!(sprawl.layout(node).unwrap().location.x, 0f32);
    assert_eq!(sprawl.layout(node).unwrap().location.y, 0f32);
    assert_eq!(sprawl.layout(node0).unwrap().size.width, 275f32);
    assert_eq!(sprawl.layout(node0).unwrap().size.height, 75f32);
    assert_eq!(sprawl.layout(node0).unwrap().location.x, -100f32);
    assert_eq!(sprawl.layout(node0).unwrap().location.y, -50f32);
}
