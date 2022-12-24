#[test]
fn wrapped_column_max_height() {
    use slotmap::Key;
    #[allow(unused_imports)]
    use taffy::{layout::Layout, prelude::*};
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy
        .new_leaf(taffy::style::Style {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Points(100f32),
                height: taffy::style::Dimension::Points(500f32),
            },
            max_size: taffy::geometry::Size { width: auto(), height: taffy::style::Dimension::Points(200f32) },
            ..Default::default()
        })
        .unwrap();
    let node1 = taffy
        .new_leaf(taffy::style::Style {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Points(200f32),
                height: taffy::style::Dimension::Points(200f32),
            },
            margin: taffy::geometry::Rect {
                left: taffy::style::LengthPercentageAuto::Points(20f32),
                right: taffy::style::LengthPercentageAuto::Points(20f32),
                top: taffy::style::LengthPercentageAuto::Points(20f32),
                bottom: taffy::style::LengthPercentageAuto::Points(20f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node2 = taffy
        .new_leaf(taffy::style::Style {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Points(100f32),
                height: taffy::style::Dimension::Points(100f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                flex_direction: taffy::style::FlexDirection::Column,
                flex_wrap: taffy::style::FlexWrap::Wrap,
                align_items: Some(taffy::style::AlignItems::Center),
                align_content: Some(taffy::style::AlignContent::Center),
                justify_content: Some(taffy::style::JustifyContent::Center),
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(700f32),
                    height: taffy::style::Dimension::Points(500f32),
                },
                ..Default::default()
            },
            &[node0, node1, node2],
        )
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT).unwrap();
    println!("\nComputed tree:");
    taffy::debug::print_tree(&taffy, node);
    println!();
    let Layout { size, location, .. } = taffy.layout(node).unwrap();
    assert_eq!(size.width, 700f32, "width of node {:?}. Expected {}. Actual {}", node.data(), 700f32, size.width);
    assert_eq!(size.height, 500f32, "height of node {:?}. Expected {}. Actual {}", node.data(), 500f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node.data(), 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node.data(), 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node0).unwrap();
    assert_eq!(size.width, 100f32, "width of node {:?}. Expected {}. Actual {}", node0.data(), 100f32, size.width);
    assert_eq!(size.height, 200f32, "height of node {:?}. Expected {}. Actual {}", node0.data(), 200f32, size.height);
    assert_eq!(location.x, 250f32, "x of node {:?}. Expected {}. Actual {}", node0.data(), 250f32, location.x);
    assert_eq!(location.y, 30f32, "y of node {:?}. Expected {}. Actual {}", node0.data(), 30f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node1).unwrap();
    assert_eq!(size.width, 200f32, "width of node {:?}. Expected {}. Actual {}", node1.data(), 200f32, size.width);
    assert_eq!(size.height, 200f32, "height of node {:?}. Expected {}. Actual {}", node1.data(), 200f32, size.height);
    assert_eq!(location.x, 200f32, "x of node {:?}. Expected {}. Actual {}", node1.data(), 200f32, location.x);
    assert_eq!(location.y, 250f32, "y of node {:?}. Expected {}. Actual {}", node1.data(), 250f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node2).unwrap();
    assert_eq!(size.width, 100f32, "width of node {:?}. Expected {}. Actual {}", node2.data(), 100f32, size.width);
    assert_eq!(size.height, 100f32, "height of node {:?}. Expected {}. Actual {}", node2.data(), 100f32, size.height);
    assert_eq!(location.x, 420f32, "x of node {:?}. Expected {}. Actual {}", node2.data(), 420f32, location.x);
    assert_eq!(location.y, 200f32, "y of node {:?}. Expected {}. Actual {}", node2.data(), 200f32, location.y);
}
