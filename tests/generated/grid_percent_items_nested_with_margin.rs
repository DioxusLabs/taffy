#[test]
fn grid_percent_items_nested_with_margin() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, tree::Layout};
    let mut taffy = taffy::Taffy::new();
    let node00 = taffy.new_leaf(taffy::style::Style {
        size: taffy::geometry::Size { width: taffy::style::Dimension::Percent(0.45f32), height: auto() },
        margin: taffy::geometry::Rect {
            left: taffy::style::LengthPercentageAuto::Percent(0.05f32),
            right: taffy::style::LengthPercentageAuto::Percent(0.05f32),
            top: taffy::style::LengthPercentageAuto::Percent(0.05f32),
            bottom: taffy::style::LengthPercentageAuto::Percent(0.05f32),
        },
        ..Default::default()
    });
    let node0 = taffy.new_with_children(
        taffy::style::Style {
            display: taffy::style::Display::Grid,
            size: taffy::geometry::Size { width: taffy::style::Dimension::Percent(0.5f32), height: auto() },
            ..Default::default()
        },
        &[node00],
    );
    let node = taffy.new_with_children(
        taffy::style::Style {
            display: taffy::style::Display::Grid,
            size: taffy::geometry::Size { width: taffy::style::Dimension::Length(200f32), height: auto() },
            ..Default::default()
        },
        &[node0],
    );
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT);
    println!("\nComputed tree:");
    taffy::util::print_tree(&taffy, node);
    println!();
    let Layout { size, location, .. } = taffy.layout(node);
    assert_eq!(size.width, 200f32, "width of node {:?}. Expected {}. Actual {}", node, 200f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node, 10f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node0);
    assert_eq!(size.width, 100f32, "width of node {:?}. Expected {}. Actual {}", node0, 100f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node0, 10f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node0, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node0, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node00);
    assert_eq!(size.width, 45f32, "width of node {:?}. Expected {}. Actual {}", node00, 45f32, size.width);
    assert_eq!(size.height, 0f32, "height of node {:?}. Expected {}. Actual {}", node00, 0f32, size.height);
    assert_eq!(location.x, 5f32, "x of node {:?}. Expected {}. Actual {}", node00, 5f32, location.x);
    assert_eq!(location.y, 5f32, "y of node {:?}. Expected {}. Actual {}", node00, 5f32, location.y);
}
