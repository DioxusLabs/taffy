#[test]
fn bevy_issue_9530_reduced2() {
    use slotmap::Key;
    #[allow(unused_imports)]
    use taffy::{layout::Layout, prelude::*};
    let mut taffy = taffy::Taffy::new();
    let node00 = taffy
        .new_leaf_with_measure(
            taffy::style::Style { flex_grow: 1f32, ..Default::default() },
            taffy::node::MeasureFunc::Raw(|known_dimensions, available_space| {
                const TEXT: &str = "HH\u{200b}HH\u{200b}HH\u{200b}HH\u{200b}HH\u{200b}HH\u{200b}HH\u{200b}HH";
                super::measure_standard_text(
                    known_dimensions,
                    available_space,
                    TEXT,
                    super::WritingMode::Horizontal,
                    None,
                )
            }),
        )
        .unwrap();
    let node0 = taffy
        .new_with_children(
            taffy::style::Style {
                flex_direction: taffy::style::FlexDirection::Column,
                flex_grow: 1f32,
                size: taffy::geometry::Size { width: taffy::style::Dimension::Points(80f32), height: auto() },
                margin: taffy::geometry::Rect {
                    left: taffy::style::LengthPercentageAuto::Points(20f32),
                    right: taffy::style::LengthPercentageAuto::Points(20f32),
                    top: taffy::style::LengthPercentageAuto::Points(0f32),
                    bottom: taffy::style::LengthPercentageAuto::Points(0f32),
                },
                ..Default::default()
            },
            &[node00],
        )
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style { flex_direction: taffy::style::FlexDirection::Column, ..Default::default() },
            &[node0],
        )
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT).unwrap();
    println!("\nComputed tree:");
    taffy::debug::print_tree(&taffy, node);
    println!();
    let Layout { size, location, .. } = taffy.layout(node).unwrap();
    assert_eq!(size.width, 120f32, "width of node {:?}. Expected {}. Actual {}", node.data(), 120f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node.data(), 20f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node.data(), 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node.data(), 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node0).unwrap();
    assert_eq!(size.width, 80f32, "width of node {:?}. Expected {}. Actual {}", node0.data(), 80f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node0.data(), 20f32, size.height);
    assert_eq!(location.x, 20f32, "x of node {:?}. Expected {}. Actual {}", node0.data(), 20f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node0.data(), 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node00).unwrap();
    assert_eq!(size.width, 80f32, "width of node {:?}. Expected {}. Actual {}", node00.data(), 80f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node00.data(), 20f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node00.data(), 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node00.data(), 0f32, location.y);
}
