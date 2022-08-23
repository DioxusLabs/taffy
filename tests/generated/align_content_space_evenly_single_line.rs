#[test]
fn align_content_space_evenly_single_line() {
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy
        .new_with_children(
            taffy::style::FlexboxLayout {
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(10f32),
                    height: taffy::style::Dimension::Points(10f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node1 = taffy
        .new_with_children(
            taffy::style::FlexboxLayout {
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(10f32),
                    height: taffy::style::Dimension::Points(10f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node2 = taffy
        .new_with_children(
            taffy::style::FlexboxLayout {
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(10f32),
                    height: taffy::style::Dimension::Points(10f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node3 = taffy
        .new_with_children(
            taffy::style::FlexboxLayout {
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(10f32),
                    height: taffy::style::Dimension::Points(10f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node4 = taffy
        .new_with_children(
            taffy::style::FlexboxLayout {
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(10f32),
                    height: taffy::style::Dimension::Points(10f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node5 = taffy
        .new_with_children(
            taffy::style::FlexboxLayout {
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(10f32),
                    height: taffy::style::Dimension::Points(10f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::FlexboxLayout {
                align_content: taffy::style::AlignContent::SpaceEvenly,
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(100f32),
                    height: taffy::style::Dimension::Points(100f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node0, node1, node2, node3, node4, node5],
        )
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::NONE).unwrap();
    assert_eq!(taffy.layout(node).unwrap().size.width, 100f32);
    assert_eq!(taffy.layout(node).unwrap().size.height, 100f32);
    assert_eq!(taffy.layout(node).unwrap().location.x, 0f32);
    assert_eq!(taffy.layout(node).unwrap().location.y, 0f32);
    assert_eq!(taffy.layout(node0).unwrap().size.width, 10f32);
    assert_eq!(taffy.layout(node0).unwrap().size.height, 10f32);
    assert_eq!(taffy.layout(node0).unwrap().location.x, 0f32);
    assert_eq!(taffy.layout(node0).unwrap().location.y, 0f32);
    assert_eq!(taffy.layout(node1).unwrap().size.width, 10f32);
    assert_eq!(taffy.layout(node1).unwrap().size.height, 10f32);
    assert_eq!(taffy.layout(node1).unwrap().location.x, 10f32);
    assert_eq!(taffy.layout(node1).unwrap().location.y, 0f32);
    assert_eq!(taffy.layout(node2).unwrap().size.width, 10f32);
    assert_eq!(taffy.layout(node2).unwrap().size.height, 10f32);
    assert_eq!(taffy.layout(node2).unwrap().location.x, 20f32);
    assert_eq!(taffy.layout(node2).unwrap().location.y, 0f32);
    assert_eq!(taffy.layout(node3).unwrap().size.width, 10f32);
    assert_eq!(taffy.layout(node3).unwrap().size.height, 10f32);
    assert_eq!(taffy.layout(node3).unwrap().location.x, 30f32);
    assert_eq!(taffy.layout(node3).unwrap().location.y, 0f32);
    assert_eq!(taffy.layout(node4).unwrap().size.width, 10f32);
    assert_eq!(taffy.layout(node4).unwrap().size.height, 10f32);
    assert_eq!(taffy.layout(node4).unwrap().location.x, 40f32);
    assert_eq!(taffy.layout(node4).unwrap().location.y, 0f32);
    assert_eq!(taffy.layout(node5).unwrap().size.width, 10f32);
    assert_eq!(taffy.layout(node5).unwrap().size.height, 10f32);
    assert_eq!(taffy.layout(node5).unwrap().location.x, 50f32);
    assert_eq!(taffy.layout(node5).unwrap().location.y, 0f32);
}
