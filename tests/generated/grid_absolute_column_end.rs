#[test]
fn grid_absolute_column_end() {
    #[allow(unused_imports)]
    use taffy::prelude::*;
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy
        .new_leaf(taffy::style::Style {
            position_type: taffy::style::PositionType::Absolute,
            grid_column: taffy::geometry::Line {
                start: taffy::style::GridPlacement::Auto,
                end: taffy::style::GridPlacement::Line(1i16),
            },
            position: taffy::geometry::Rect {
                left: taffy::style::LengthPercentageAuto::Points(4f32),
                right: taffy::style::LengthPercentageAuto::Points(3f32),
                top: taffy::style::LengthPercentageAuto::Points(1f32),
                bottom: taffy::style::LengthPercentageAuto::Points(2f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Grid,
                grid_template_rows: vec![points(40f32), points(40f32), points(40f32)],
                grid_template_columns: vec![points(40f32), points(40f32), points(40f32)],
                padding: taffy::geometry::Rect {
                    left: taffy::style::LengthPercentage::Points(40f32),
                    right: taffy::style::LengthPercentage::Points(20f32),
                    top: taffy::style::LengthPercentage::Points(10f32),
                    bottom: taffy::style::LengthPercentage::Points(30f32),
                },
                ..Default::default()
            },
            &[node0],
        )
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT).unwrap();
    println!("\nComputed tree:");
    taffy::debug::print_tree(&taffy, node);
    println!();
    assert_eq!(taffy.layout(node).unwrap().size.width, 180f32);
    assert_eq!(taffy.layout(node).unwrap().size.height, 160f32);
    assert_eq!(taffy.layout(node).unwrap().location.x, 0f32);
    assert_eq!(taffy.layout(node).unwrap().location.y, 0f32);
    assert_eq!(taffy.layout(node0).unwrap().size.width, 33f32);
    assert_eq!(taffy.layout(node0).unwrap().size.height, 157f32);
    assert_eq!(taffy.layout(node0).unwrap().location.x, 4f32);
    assert_eq!(taffy.layout(node0).unwrap().location.y, 1f32);
}
