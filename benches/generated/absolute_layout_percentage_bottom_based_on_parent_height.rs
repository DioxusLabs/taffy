pub fn compute() {
    #[allow(unused_imports)]
    use taffy::prelude::*;
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy
        .new_leaf(taffy::style::Style {
            position_type: taffy::style::PositionType::Absolute,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Points(10f32),
                height: taffy::style::Dimension::Points(10f32),
            },
            position: taffy::geometry::Rect {
                left: auto(),
                right: auto(),
                top: taffy::style::LengthPercentageAuto::Percent(0.5f32),
                bottom: auto(),
            },
            ..Default::default()
        })
        .unwrap();
    let node1 = taffy
        .new_leaf(taffy::style::Style {
            position_type: taffy::style::PositionType::Absolute,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Points(10f32),
                height: taffy::style::Dimension::Points(10f32),
            },
            position: taffy::geometry::Rect {
                left: auto(),
                right: auto(),
                top: auto(),
                bottom: taffy::style::LengthPercentageAuto::Percent(0.5f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node2 = taffy
        .new_leaf(taffy::style::Style {
            position_type: taffy::style::PositionType::Absolute,
            size: taffy::geometry::Size { width: taffy::style::Dimension::Points(10f32), height: auto() },
            position: taffy::geometry::Rect {
                left: auto(),
                right: auto(),
                top: taffy::style::LengthPercentageAuto::Percent(0.1f32),
                bottom: taffy::style::LengthPercentageAuto::Percent(0.1f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(100f32),
                    height: taffy::style::Dimension::Points(200f32),
                },
                ..Default::default()
            },
            &[node0, node1, node2],
        )
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT).unwrap();
}
