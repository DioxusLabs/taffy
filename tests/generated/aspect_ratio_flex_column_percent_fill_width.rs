#[test]
fn aspect_ratio_flex_column_percent_fill_width() {
    use slotmap::Key;
    #[allow(unused_imports)]
    use taffy::{layout::Layout, prelude::*};
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
    println!("\nComputed tree:");
    taffy::debug::print_tree(&taffy, node);
    println!();
    let Layout { size, location, .. } = taffy.layout(node).unwrap();
    assert_eq!(size.width, 1280f32, "width of node {:?}. Expected {}. Actual {}", node.data(), 1280f32, size.width);
    assert_eq!(size.height, 720f32, "height of node {:?}. Expected {}. Actual {}", node.data(), 720f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node.data(), 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node.data(), 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node0).unwrap();
    assert_eq!(size.width, 256f32, "width of node {:?}. Expected {}. Actual {}", node0.data(), 256f32, size.width);
    assert_eq!(size.height, 85f32, "height of node {:?}. Expected {}. Actual {}", node0.data(), 85f32, size.height);
    assert_eq!(location.x, 64f32, "x of node {:?}. Expected {}. Actual {}", node0.data(), 64f32, location.x);
    assert_eq!(location.y, 36f32, "y of node {:?}. Expected {}. Actual {}", node0.data(), 36f32, location.y);
}
