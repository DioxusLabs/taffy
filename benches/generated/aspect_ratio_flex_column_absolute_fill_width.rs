pub fn compute() {
    #[allow(unused_imports)]
    use taffy::prelude::*;
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy
        .new_leaf(taffy::style::Style {
            position: taffy::style::Position::Absolute,
            size: taffy::geometry::Size { width: taffy::style::Dimension::Percent(0.2f32), height: auto() },
            aspect_ratio: Some(3f32),
            inset: taffy::geometry::Rect {
                left: taffy::style::LengthPercentageAuto::Percent(0.05f32),
                right: auto(),
                top: taffy::style::LengthPercentageAuto::Percent(0.05f32),
                bottom: auto(),
            },
            ..Default::default()
        })
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Flex,
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(1280f32),
                    height: taffy::style::Dimension::Points(720f32),
                },
                ..Default::default()
            },
            &[node0],
        )
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT).unwrap();
}
