#[test]
fn block_margin_y_sibling_collapse_negative_percentage() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, tree::Layout};
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy.new_leaf(taffy::style::Style {
        size: taffy::geometry::Size { width: auto(), height: taffy::style::Dimension::Length(10f32) },
        margin: taffy::geometry::Rect {
            left: zero(),
            right: zero(),
            top: taffy::style::LengthPercentageAuto::Percent(-0.1f32),
            bottom: taffy::style::LengthPercentageAuto::Percent(-0.1f32),
        },
        ..Default::default()
    });
    let node1 = taffy.new_leaf(taffy::style::Style {
        size: taffy::geometry::Size { width: auto(), height: taffy::style::Dimension::Length(10f32) },
        margin: taffy::geometry::Rect {
            left: zero(),
            right: zero(),
            top: taffy::style::LengthPercentageAuto::Percent(-0.1f32),
            bottom: taffy::style::LengthPercentageAuto::Percent(-0.1f32),
        },
        ..Default::default()
    });
    let node2 = taffy.new_leaf(taffy::style::Style {
        size: taffy::geometry::Size { width: auto(), height: taffy::style::Dimension::Length(10f32) },
        margin: taffy::geometry::Rect {
            left: zero(),
            right: zero(),
            top: taffy::style::LengthPercentageAuto::Percent(-0.05f32),
            bottom: taffy::style::LengthPercentageAuto::Percent(-0.05f32),
        },
        ..Default::default()
    });
    let node3 = taffy.new_leaf(taffy::style::Style {
        size: taffy::geometry::Size { width: auto(), height: taffy::style::Dimension::Length(10f32) },
        margin: taffy::geometry::Rect {
            left: zero(),
            right: zero(),
            top: taffy::style::LengthPercentageAuto::Percent(-0.1f32),
            bottom: taffy::style::LengthPercentageAuto::Percent(-0.1f32),
        },
        ..Default::default()
    });
    let node = taffy.new_with_children(
        taffy::style::Style {
            display: taffy::style::Display::Block,
            size: taffy::geometry::Size { width: taffy::style::Dimension::Length(50f32), height: auto() },
            ..Default::default()
        },
        &[node0, node1, node2, node3],
    );
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT);
    println!("\nComputed tree:");
    taffy::util::print_tree(&taffy, node);
    println!();
    let Layout { size, location, .. } = taffy.layout(node);
    assert_eq!(size.width, 50f32, "width of node {:?}. Expected {}. Actual {}", node, 50f32, size.width);
    assert_eq!(size.height, 15f32, "height of node {:?}. Expected {}. Actual {}", node, 15f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node0);
    assert_eq!(size.width, 50f32, "width of node {:?}. Expected {}. Actual {}", node0, 50f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node0, 10f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node0, 0f32, location.x);
    assert_eq!(location.y, -5f32, "y of node {:?}. Expected {}. Actual {}", node0, -5f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node1);
    assert_eq!(size.width, 50f32, "width of node {:?}. Expected {}. Actual {}", node1, 50f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node1, 10f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node1, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node1, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node2);
    assert_eq!(size.width, 50f32, "width of node {:?}. Expected {}. Actual {}", node2, 50f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node2, 10f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node2, 0f32, location.x);
    assert_eq!(location.y, 5f32, "y of node {:?}. Expected {}. Actual {}", node2, 5f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node3);
    assert_eq!(size.width, 50f32, "width of node {:?}. Expected {}. Actual {}", node3, 50f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node3, 10f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node3, 0f32, location.x);
    assert_eq!(location.y, 10f32, "y of node {:?}. Expected {}. Actual {}", node3, 10f32, location.y);
}
