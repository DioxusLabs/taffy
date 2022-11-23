pub fn compute() {
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy
        .new_with_children(
            taffy::style::FlexboxLayout {
                size: taffy::geometry::Size { width: taffy::style::Dimension::Points(60f32), ..Default::default() },
                margin: taffy::geometry::Rect {
                    top: taffy::style::Dimension::Points(2f32),
                    bottom: taffy::style::Dimension::Points(2f32),
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
                size: taffy::geometry::Size { width: taffy::style::Dimension::Points(60f32), ..Default::default() },
                margin: taffy::geometry::Rect {
                    top: taffy::style::Dimension::Points(10f32),
                    bottom: taffy::style::Dimension::Points(10f32),
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
                size: taffy::geometry::Size { width: taffy::style::Dimension::Points(60f32), ..Default::default() },
                margin: taffy::geometry::Rect {
                    top: taffy::style::Dimension::Points(15f32),
                    bottom: taffy::style::Dimension::Points(15f32),
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
                flex_wrap: taffy::style::FlexWrap::Wrap,
                gap: taffy::geometry::Size { height: taffy::style::Dimension::Points(10f32), ..Default::default() },
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(100f32),
                    height: taffy::style::Dimension::Points(200f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node0, node1, node2],
        )
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT).unwrap();
}
