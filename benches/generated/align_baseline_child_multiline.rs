pub fn compute() {
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy.new_with_children(
        taffy::style::FlexboxLayout {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Points(50f32),
                height: taffy::style::Dimension::Points(60f32),
                ..Default::default()
            },
            ..Default::default()
        },
        &[],
    );
    let node10 = taffy.new_with_children(
        taffy::style::FlexboxLayout {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Points(25f32),
                height: taffy::style::Dimension::Points(20f32),
                ..Default::default()
            },
            ..Default::default()
        },
        &[],
    );
    let node11 = taffy.new_with_children(
        taffy::style::FlexboxLayout {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Points(25f32),
                height: taffy::style::Dimension::Points(10f32),
                ..Default::default()
            },
            ..Default::default()
        },
        &[],
    );
    let node12 = taffy.new_with_children(
        taffy::style::FlexboxLayout {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Points(25f32),
                height: taffy::style::Dimension::Points(20f32),
                ..Default::default()
            },
            ..Default::default()
        },
        &[],
    );
    let node13 = taffy.new_with_children(
        taffy::style::FlexboxLayout {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Points(25f32),
                height: taffy::style::Dimension::Points(10f32),
                ..Default::default()
            },
            ..Default::default()
        },
        &[],
    );
    let node1 = taffy.new_with_children(
        taffy::style::FlexboxLayout {
            flex_wrap: taffy::style::FlexWrap::Wrap,
            size: taffy::geometry::Size { width: taffy::style::Dimension::Points(50f32), ..Default::default() },
            ..Default::default()
        },
        &[node10, node11, node12, node13],
    );
    let node = taffy.new_with_children(
        taffy::style::FlexboxLayout {
            align_items: taffy::style::AlignItems::Baseline,
            size: taffy::geometry::Size { width: taffy::style::Dimension::Points(100f32), ..Default::default() },
            ..Default::default()
        },
        &[node0, node1],
    );
    taffy.compute_layout(node, taffy::geometry::Size::undefined()).unwrap();
}
