#[test]
fn wrap_reverse_row_align_content_center() {
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy.new_with_children(
        taffy::style::FlexboxLayout {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Points(30f32),
                height: taffy::style::Dimension::Points(10f32),
                ..Default::default()
            },
            ..Default::default()
        },
        &[],
    );
    let node1 = taffy.new_with_children(
        taffy::style::FlexboxLayout {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Points(30f32),
                height: taffy::style::Dimension::Points(20f32),
                ..Default::default()
            },
            ..Default::default()
        },
        &[],
    );
    let node2 = taffy.new_with_children(
        taffy::style::FlexboxLayout {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Points(30f32),
                height: taffy::style::Dimension::Points(30f32),
                ..Default::default()
            },
            ..Default::default()
        },
        &[],
    );
    let node3 = taffy.new_with_children(
        taffy::style::FlexboxLayout {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Points(30f32),
                height: taffy::style::Dimension::Points(40f32),
                ..Default::default()
            },
            ..Default::default()
        },
        &[],
    );
    let node4 = taffy.new_with_children(
        taffy::style::FlexboxLayout {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Points(30f32),
                height: taffy::style::Dimension::Points(50f32),
                ..Default::default()
            },
            ..Default::default()
        },
        &[],
    );
    let node = taffy.new_with_children(
        taffy::style::FlexboxLayout {
            flex_wrap: taffy::style::FlexWrap::WrapReverse,
            align_content: taffy::style::AlignContent::Center,
            size: taffy::geometry::Size { width: taffy::style::Dimension::Points(100f32), ..Default::default() },
            ..Default::default()
        },
        &[node0, node1, node2, node3, node4],
    );
    taffy.compute_layout(node, taffy::geometry::Size::undefined()).unwrap();
    assert_eq!(taffy.layout(node).unwrap().size.width, 100f32);
    assert_eq!(taffy.layout(node).unwrap().size.height, 80f32);
    assert_eq!(taffy.layout(node).unwrap().location.x, 0f32);
    assert_eq!(taffy.layout(node).unwrap().location.y, 0f32);
    assert_eq!(taffy.layout(node0).unwrap().size.width, 30f32);
    assert_eq!(taffy.layout(node0).unwrap().size.height, 10f32);
    assert_eq!(taffy.layout(node0).unwrap().location.x, 0f32);
    assert_eq!(taffy.layout(node0).unwrap().location.y, 70f32);
    assert_eq!(taffy.layout(node1).unwrap().size.width, 30f32);
    assert_eq!(taffy.layout(node1).unwrap().size.height, 20f32);
    assert_eq!(taffy.layout(node1).unwrap().location.x, 30f32);
    assert_eq!(taffy.layout(node1).unwrap().location.y, 60f32);
    assert_eq!(taffy.layout(node2).unwrap().size.width, 30f32);
    assert_eq!(taffy.layout(node2).unwrap().size.height, 30f32);
    assert_eq!(taffy.layout(node2).unwrap().location.x, 60f32);
    assert_eq!(taffy.layout(node2).unwrap().location.y, 50f32);
    assert_eq!(taffy.layout(node3).unwrap().size.width, 30f32);
    assert_eq!(taffy.layout(node3).unwrap().size.height, 40f32);
    assert_eq!(taffy.layout(node3).unwrap().location.x, 0f32);
    assert_eq!(taffy.layout(node3).unwrap().location.y, 10f32);
    assert_eq!(taffy.layout(node4).unwrap().size.width, 30f32);
    assert_eq!(taffy.layout(node4).unwrap().size.height, 50f32);
    assert_eq!(taffy.layout(node4).unwrap().location.x, 30f32);
    assert_eq!(taffy.layout(node4).unwrap().location.y, 0f32);
}
