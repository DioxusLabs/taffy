pub fn compute() {
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
            align_content: taffy::style::AlignContent::FlexStart,
            size: taffy::geometry::Size { width: taffy::style::Dimension::Points(300f32), ..Default::default() },
            ..Default::default()
        },
        &[node0, node1, node2, node3, node4],
    );
    taffy.compute_layout(node, taffy::geometry::Size::undefined()).unwrap();
}
