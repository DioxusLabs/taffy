#[test]
fn min_max_percent_no_width_height() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, tree::Layout};
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy.new_leaf(taffy::style::Style {
        min_size: taffy::geometry::Size {
            width: taffy::style::Dimension::Percent(0.1f32),
            height: taffy::style::Dimension::Percent(0.1f32),
        },
        max_size: taffy::geometry::Size {
            width: taffy::style::Dimension::Percent(0.1f32),
            height: taffy::style::Dimension::Percent(0.1f32),
        },
        ..Default::default()
    });
    let node = taffy.new_with_children(
        taffy::style::Style {
            flex_direction: taffy::style::FlexDirection::Column,
            align_items: Some(taffy::style::AlignItems::FlexStart),
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Length(100f32),
                height: taffy::style::Dimension::Length(100f32),
            },
            ..Default::default()
        },
        &[node0],
    );
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT);
    println!("\nComputed tree:");
    taffy::util::print_tree(&taffy, node);
    println!();
    let Layout { size, location, .. } = taffy.layout(node);
    assert_eq!(size.width, 100f32, "width of node {:?}. Expected {}. Actual {}", node, 100f32, size.width);
    assert_eq!(size.height, 100f32, "height of node {:?}. Expected {}. Actual {}", node, 100f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node0);
    assert_eq!(size.width, 10f32, "width of node {:?}. Expected {}. Actual {}", node0, 10f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node0, 10f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node0, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node0, 0f32, location.y);
}
