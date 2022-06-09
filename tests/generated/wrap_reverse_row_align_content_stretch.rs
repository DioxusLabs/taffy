#[test]
fn wrap_reverse_row_align_content_stretch() {
    let mut sprawl = sprawl::Sprawl::new();
    let node0 = sprawl
        .new_with_children(
            sprawl::style::Style {
                size: sprawl::geometry::Size {
                    width: sprawl::style::Dimension::Points(30f32),
                    height: sprawl::style::Dimension::Points(10f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node1 = sprawl
        .new_with_children(
            sprawl::style::Style {
                size: sprawl::geometry::Size {
                    width: sprawl::style::Dimension::Points(30f32),
                    height: sprawl::style::Dimension::Points(20f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node2 = sprawl
        .new_with_children(
            sprawl::style::Style {
                size: sprawl::geometry::Size {
                    width: sprawl::style::Dimension::Points(30f32),
                    height: sprawl::style::Dimension::Points(30f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node3 = sprawl
        .new_with_children(
            sprawl::style::Style {
                size: sprawl::geometry::Size {
                    width: sprawl::style::Dimension::Points(30f32),
                    height: sprawl::style::Dimension::Points(40f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node4 = sprawl
        .new_with_children(
            sprawl::style::Style {
                size: sprawl::geometry::Size {
                    width: sprawl::style::Dimension::Points(30f32),
                    height: sprawl::style::Dimension::Points(50f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node = sprawl
        .new_with_children(
            sprawl::style::Style {
                flex_wrap: sprawl::style::FlexWrap::WrapReverse,
                size: sprawl::geometry::Size { width: sprawl::style::Dimension::Points(100f32), ..Default::default() },
                ..Default::default()
            },
            &[node0, node1, node2, node3, node4],
        )
        .unwrap();
    sprawl.compute_layout(node, sprawl::geometry::Size::undefined()).unwrap();
    assert_eq!(sprawl.layout(node).unwrap().size.width, 100f32);
    assert_eq!(sprawl.layout(node).unwrap().size.height, 80f32);
    assert_eq!(sprawl.layout(node).unwrap().location.x, 0f32);
    assert_eq!(sprawl.layout(node).unwrap().location.y, 0f32);
    assert_eq!(sprawl.layout(node0).unwrap().size.width, 30f32);
    assert_eq!(sprawl.layout(node0).unwrap().size.height, 10f32);
    assert_eq!(sprawl.layout(node0).unwrap().location.x, 0f32);
    assert_eq!(sprawl.layout(node0).unwrap().location.y, 70f32);
    assert_eq!(sprawl.layout(node1).unwrap().size.width, 30f32);
    assert_eq!(sprawl.layout(node1).unwrap().size.height, 20f32);
    assert_eq!(sprawl.layout(node1).unwrap().location.x, 30f32);
    assert_eq!(sprawl.layout(node1).unwrap().location.y, 60f32);
    assert_eq!(sprawl.layout(node2).unwrap().size.width, 30f32);
    assert_eq!(sprawl.layout(node2).unwrap().size.height, 30f32);
    assert_eq!(sprawl.layout(node2).unwrap().location.x, 60f32);
    assert_eq!(sprawl.layout(node2).unwrap().location.y, 50f32);
    assert_eq!(sprawl.layout(node3).unwrap().size.width, 30f32);
    assert_eq!(sprawl.layout(node3).unwrap().size.height, 40f32);
    assert_eq!(sprawl.layout(node3).unwrap().location.x, 0f32);
    assert_eq!(sprawl.layout(node3).unwrap().location.y, 10f32);
    assert_eq!(sprawl.layout(node4).unwrap().size.width, 30f32);
    assert_eq!(sprawl.layout(node4).unwrap().size.height, 50f32);
    assert_eq!(sprawl.layout(node4).unwrap().location.x, 30f32);
    assert_eq!(sprawl.layout(node4).unwrap().location.y, 0f32);
}
