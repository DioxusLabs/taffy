#[test]
fn grid_percent_nested_moderate() {
    use slotmap::Key;
    #[allow(unused_imports)]
    use taffy::{layout::Layout, prelude::*};
    let mut taffy = taffy::Taffy::new();
    let node00 = taffy
        .new_leaf(taffy::style::Style {
            size: taffy::geometry::Size { width: taffy::style::Dimension::Percent(0.45f32), height: auto() },
            margin: taffy::geometry::Rect {
                left: taffy::style::LengthPercentageAuto::Percent(0.05f32),
                right: taffy::style::LengthPercentageAuto::Percent(0.05f32),
                top: taffy::style::LengthPercentageAuto::Percent(0.05f32),
                bottom: taffy::style::LengthPercentageAuto::Percent(0.05f32),
            },
            padding: taffy::geometry::Rect {
                left: taffy::style::LengthPercentage::Points(3f32),
                right: taffy::style::LengthPercentage::Points(3f32),
                top: taffy::style::LengthPercentage::Points(3f32),
                bottom: taffy::style::LengthPercentage::Points(3f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node0 = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Grid,
                size: taffy::geometry::Size { width: taffy::style::Dimension::Percent(0.5f32), height: auto() },
                margin: taffy::geometry::Rect {
                    left: taffy::style::LengthPercentageAuto::Points(5f32),
                    right: taffy::style::LengthPercentageAuto::Points(5f32),
                    top: taffy::style::LengthPercentageAuto::Points(5f32),
                    bottom: taffy::style::LengthPercentageAuto::Points(5f32),
                },
                padding: taffy::geometry::Rect {
                    left: taffy::style::LengthPercentage::Percent(0.03f32),
                    right: taffy::style::LengthPercentage::Percent(0.03f32),
                    top: taffy::style::LengthPercentage::Percent(0.03f32),
                    bottom: taffy::style::LengthPercentage::Percent(0.03f32),
                },
                ..Default::default()
            },
            &[node00],
        )
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Grid,
                size: taffy::geometry::Size { width: taffy::style::Dimension::Points(200f32), height: auto() },
                padding: taffy::geometry::Rect {
                    left: taffy::style::LengthPercentage::Points(3f32),
                    right: taffy::style::LengthPercentage::Points(3f32),
                    top: taffy::style::LengthPercentage::Points(3f32),
                    bottom: taffy::style::LengthPercentage::Points(3f32),
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
    assert_eq!(size.width, 200f32, "width of node {:?}. Expected {}. Actual {}", node.data(), 200f32, size.width);
    assert_eq!(size.height, 42f32, "height of node {:?}. Expected {}. Actual {}", node.data(), 42f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node.data(), 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node.data(), 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node0).unwrap();
    assert_eq!(size.width, 97f32, "width of node {:?}. Expected {}. Actual {}", node0.data(), 97f32, size.width);
    assert_eq!(size.height, 26f32, "height of node {:?}. Expected {}. Actual {}", node0.data(), 26f32, size.height);
    assert_eq!(location.x, 8f32, "x of node {:?}. Expected {}. Actual {}", node0.data(), 8f32, location.x);
    assert_eq!(location.y, 8f32, "y of node {:?}. Expected {}. Actual {}", node0.data(), 8f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node00).unwrap();
    assert_eq!(size.width, 38f32, "width of node {:?}. Expected {}. Actual {}", node00.data(), 38f32, size.width);
    assert_eq!(size.height, 6f32, "height of node {:?}. Expected {}. Actual {}", node00.data(), 6f32, size.height);
    assert_eq!(location.x, 10f32, "x of node {:?}. Expected {}. Actual {}", node00.data(), 10f32, location.x);
    assert_eq!(location.y, 10f32, "y of node {:?}. Expected {}. Actual {}", node00.data(), 10f32, location.y);
}
