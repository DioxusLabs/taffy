#[test]
fn wrapped_column_max_height() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, tree::Layout};
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy.new_leaf(taffy::style::Style {
        size: taffy::geometry::Size {
            width: taffy::style::Dimension::Length(100f32),
            height: taffy::style::Dimension::Length(500f32),
        },
        max_size: taffy::geometry::Size { width: auto(), height: taffy::style::Dimension::Length(200f32) },
        ..Default::default()
    });
    let node1 = taffy.new_leaf(taffy::style::Style {
        size: taffy::geometry::Size {
            width: taffy::style::Dimension::Length(200f32),
            height: taffy::style::Dimension::Length(200f32),
        },
        margin: taffy::geometry::Rect {
            left: taffy::style::LengthPercentageAuto::Length(20f32),
            right: taffy::style::LengthPercentageAuto::Length(20f32),
            top: taffy::style::LengthPercentageAuto::Length(20f32),
            bottom: taffy::style::LengthPercentageAuto::Length(20f32),
        },
        ..Default::default()
    });
    let node2 = taffy.new_leaf(taffy::style::Style {
        size: taffy::geometry::Size {
            width: taffy::style::Dimension::Length(100f32),
            height: taffy::style::Dimension::Length(100f32),
        },
        ..Default::default()
    });
    let node = taffy.new_with_children(
        taffy::style::Style {
            flex_direction: taffy::style::FlexDirection::Column,
            flex_wrap: taffy::style::FlexWrap::Wrap,
            align_items: Some(taffy::style::AlignItems::Center),
            align_content: Some(taffy::style::AlignContent::Center),
            justify_content: Some(taffy::style::JustifyContent::Center),
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Length(700f32),
                height: taffy::style::Dimension::Length(500f32),
            },
            ..Default::default()
        },
        &[node0, node1, node2],
    );
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT);
    println!("\nComputed tree:");
    taffy::util::print_tree(&taffy, node);
    println!();
    let Layout { size, location, .. } = taffy.layout(node);
    assert_eq!(size.width, 700f32, "width of node {:?}. Expected {}. Actual {}", node, 700f32, size.width);
    assert_eq!(size.height, 500f32, "height of node {:?}. Expected {}. Actual {}", node, 500f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node0);
    assert_eq!(size.width, 100f32, "width of node {:?}. Expected {}. Actual {}", node0, 100f32, size.width);
    assert_eq!(size.height, 200f32, "height of node {:?}. Expected {}. Actual {}", node0, 200f32, size.height);
    assert_eq!(location.x, 250f32, "x of node {:?}. Expected {}. Actual {}", node0, 250f32, location.x);
    assert_eq!(location.y, 30f32, "y of node {:?}. Expected {}. Actual {}", node0, 30f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node1);
    assert_eq!(size.width, 200f32, "width of node {:?}. Expected {}. Actual {}", node1, 200f32, size.width);
    assert_eq!(size.height, 200f32, "height of node {:?}. Expected {}. Actual {}", node1, 200f32, size.height);
    assert_eq!(location.x, 200f32, "x of node {:?}. Expected {}. Actual {}", node1, 200f32, location.x);
    assert_eq!(location.y, 250f32, "y of node {:?}. Expected {}. Actual {}", node1, 250f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node2);
    assert_eq!(size.width, 100f32, "width of node {:?}. Expected {}. Actual {}", node2, 100f32, size.width);
    assert_eq!(size.height, 100f32, "height of node {:?}. Expected {}. Actual {}", node2, 100f32, size.height);
    assert_eq!(location.x, 420f32, "x of node {:?}. Expected {}. Actual {}", node2, 420f32, location.x);
    assert_eq!(location.y, 200f32, "y of node {:?}. Expected {}. Actual {}", node2, 200f32, location.y);
}
