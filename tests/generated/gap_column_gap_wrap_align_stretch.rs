#[test]
fn gap_column_gap_wrap_align_stretch() {
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy
        .new_with_children(
            taffy::style::FlexboxLayout {
                flex_grow: 1f32,
                min_size: taffy::geometry::Size { width: taffy::style::Dimension::Points(60f32), ..Default::default() },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node1 = taffy
        .new_with_children(
            taffy::style::FlexboxLayout {
                flex_grow: 1f32,
                min_size: taffy::geometry::Size { width: taffy::style::Dimension::Points(60f32), ..Default::default() },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node2 = taffy
        .new_with_children(
            taffy::style::FlexboxLayout {
                flex_grow: 1f32,
                min_size: taffy::geometry::Size { width: taffy::style::Dimension::Points(60f32), ..Default::default() },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node3 = taffy
        .new_with_children(
            taffy::style::FlexboxLayout {
                flex_grow: 1f32,
                min_size: taffy::geometry::Size { width: taffy::style::Dimension::Points(60f32), ..Default::default() },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node4 = taffy
        .new_with_children(
            taffy::style::FlexboxLayout {
                flex_grow: 1f32,
                min_size: taffy::geometry::Size { width: taffy::style::Dimension::Points(60f32), ..Default::default() },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::FlexboxLayout {
                flex_wrap: taffy::style::FlexWrap::Wrap,
                gap: taffy::geometry::Size { width: taffy::style::Dimension::Points(5f32), ..Default::default() },
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(300f32),
                    height: taffy::style::Dimension::Points(300f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node0, node1, node2, node3, node4],
        )
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT).unwrap();
    assert_eq!(taffy.layout(node).unwrap().size.width, 300f32);
    assert_eq!(taffy.layout(node).unwrap().size.height, 300f32);
    assert_eq!(taffy.layout(node).unwrap().location.x, 0f32);
    assert_eq!(taffy.layout(node).unwrap().location.y, 0f32);
    assert_eq!(taffy.layout(node0).unwrap().size.width, 71f32);
    assert_eq!(taffy.layout(node0).unwrap().size.height, 150f32);
    assert_eq!(taffy.layout(node0).unwrap().location.x, 0f32);
    assert_eq!(taffy.layout(node0).unwrap().location.y, 0f32);
    assert_eq!(taffy.layout(node1).unwrap().size.width, 71f32);
    assert_eq!(taffy.layout(node1).unwrap().size.height, 150f32);
    assert_eq!(taffy.layout(node1).unwrap().location.x, 76f32);
    assert_eq!(taffy.layout(node1).unwrap().location.y, 0f32);
    assert_eq!(taffy.layout(node2).unwrap().size.width, 71f32);
    assert_eq!(taffy.layout(node2).unwrap().size.height, 150f32);
    assert_eq!(taffy.layout(node2).unwrap().location.x, 153f32);
    assert_eq!(taffy.layout(node2).unwrap().location.y, 0f32);
    assert_eq!(taffy.layout(node3).unwrap().size.width, 71f32);
    assert_eq!(taffy.layout(node3).unwrap().size.height, 150f32);
    assert_eq!(taffy.layout(node3).unwrap().location.x, 229f32);
    assert_eq!(taffy.layout(node3).unwrap().location.y, 0f32);
    assert_eq!(taffy.layout(node4).unwrap().size.width, 300f32);
    assert_eq!(taffy.layout(node4).unwrap().size.height, 150f32);
    assert_eq!(taffy.layout(node4).unwrap().location.x, 0f32);
    assert_eq!(taffy.layout(node4).unwrap().location.y, 150f32);
}
