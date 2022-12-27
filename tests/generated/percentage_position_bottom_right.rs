#[test]
fn percentage_position_bottom_right() {
    use slotmap::Key;
    #[allow(unused_imports)]
    use taffy::{layout::Layout, prelude::*};
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy
        .new_leaf(taffy::style::Style {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Percent(0.55f32),
                height: taffy::style::Dimension::Percent(0.15f32),
            },
            inset: taffy::geometry::Rect {
                left: auto(),
                right: taffy::style::LengthPercentageAuto::Percent(0.2f32),
                top: auto(),
                bottom: taffy::style::LengthPercentageAuto::Percent(0.1f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(500f32),
                    height: taffy::style::Dimension::Points(500f32),
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
    assert_eq!(size.width, 500f32, "width of node {:?}. Expected {}. Actual {}", node.data(), 500f32, size.width);
    assert_eq!(size.height, 500f32, "height of node {:?}. Expected {}. Actual {}", node.data(), 500f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node.data(), 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node.data(), 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node0).unwrap();
    assert_eq!(size.width, 275f32, "width of node {:?}. Expected {}. Actual {}", node0.data(), 275f32, size.width);
    assert_eq!(size.height, 75f32, "height of node {:?}. Expected {}. Actual {}", node0.data(), 75f32, size.height);
    assert_eq!(location.x, -100f32, "x of node {:?}. Expected {}. Actual {}", node0.data(), -100f32, location.x);
    assert_eq!(location.y, -50f32, "y of node {:?}. Expected {}. Actual {}", node0.data(), -50f32, location.y);
}
