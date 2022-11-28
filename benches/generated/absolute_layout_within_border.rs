pub fn compute() {
    #[allow(unused_imports)]
    use taffy::prelude::*;
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy
        .new_with_children(
            taffy::style::Style {
                position_type: taffy::style::PositionType::Absolute,
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(50f32),
                    height: taffy::style::Dimension::Points(50f32),
                    ..Default::default()
                },
                position: taffy::geometry::Rect {
                    left: taffy::style::Dimension::Points(0f32),
                    top: taffy::style::Dimension::Points(0f32),
                    ..Rect::auto()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node1 = taffy
        .new_with_children(
            taffy::style::Style {
                position_type: taffy::style::PositionType::Absolute,
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(50f32),
                    height: taffy::style::Dimension::Points(50f32),
                    ..Default::default()
                },
                position: taffy::geometry::Rect {
                    right: taffy::style::Dimension::Points(0f32),
                    bottom: taffy::style::Dimension::Points(0f32),
                    ..Rect::auto()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node2 = taffy
        .new_with_children(
            taffy::style::Style {
                position_type: taffy::style::PositionType::Absolute,
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(50f32),
                    height: taffy::style::Dimension::Points(50f32),
                    ..Default::default()
                },
                margin: taffy::geometry::Rect {
                    left: taffy::style::Dimension::Points(10f32),
                    right: taffy::style::Dimension::Points(10f32),
                    top: taffy::style::Dimension::Points(10f32),
                    bottom: taffy::style::Dimension::Points(10f32),
                    ..Rect::zero()
                },
                position: taffy::geometry::Rect {
                    left: taffy::style::Dimension::Points(0f32),
                    top: taffy::style::Dimension::Points(0f32),
                    ..Rect::auto()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node3 = taffy
        .new_with_children(
            taffy::style::Style {
                position_type: taffy::style::PositionType::Absolute,
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(50f32),
                    height: taffy::style::Dimension::Points(50f32),
                    ..Default::default()
                },
                margin: taffy::geometry::Rect {
                    left: taffy::style::Dimension::Points(10f32),
                    right: taffy::style::Dimension::Points(10f32),
                    top: taffy::style::Dimension::Points(10f32),
                    bottom: taffy::style::Dimension::Points(10f32),
                    ..Rect::zero()
                },
                position: taffy::geometry::Rect {
                    right: taffy::style::Dimension::Points(0f32),
                    bottom: taffy::style::Dimension::Points(0f32),
                    ..Rect::auto()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(100f32),
                    height: taffy::style::Dimension::Points(100f32),
                    ..Default::default()
                },
                padding: taffy::geometry::Rect {
                    left: taffy::style::Dimension::Points(10f32),
                    right: taffy::style::Dimension::Points(10f32),
                    top: taffy::style::Dimension::Points(10f32),
                    bottom: taffy::style::Dimension::Points(10f32),
                    ..Rect::zero()
                },
                border: taffy::geometry::Rect {
                    left: taffy::style::Dimension::Points(10f32),
                    right: taffy::style::Dimension::Points(10f32),
                    top: taffy::style::Dimension::Points(10f32),
                    bottom: taffy::style::Dimension::Points(10f32),
                    ..Rect::zero()
                },
                ..Default::default()
            },
            &[node0, node1, node2, node3],
        )
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT).unwrap();
}
