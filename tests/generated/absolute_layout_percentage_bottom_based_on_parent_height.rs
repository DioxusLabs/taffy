#[test]
fn absolute_layout_percentage_bottom_based_on_parent_height() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, tree::Layout};
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy.new_leaf(taffy::style::Style {
        position: taffy::style::Position::Absolute,
        size: taffy::geometry::Size {
            width: taffy::style::Dimension::Length(10f32),
            height: taffy::style::Dimension::Length(10f32),
        },
        inset: taffy::geometry::Rect {
            left: auto(),
            right: auto(),
            top: taffy::style::LengthPercentageAuto::Percent(0.5f32),
            bottom: auto(),
        },
        ..Default::default()
    });
    let node1 = taffy.new_leaf(taffy::style::Style {
        position: taffy::style::Position::Absolute,
        size: taffy::geometry::Size {
            width: taffy::style::Dimension::Length(10f32),
            height: taffy::style::Dimension::Length(10f32),
        },
        inset: taffy::geometry::Rect {
            left: auto(),
            right: auto(),
            top: auto(),
            bottom: taffy::style::LengthPercentageAuto::Percent(0.5f32),
        },
        ..Default::default()
    });
    let node2 = taffy.new_leaf(taffy::style::Style {
        position: taffy::style::Position::Absolute,
        size: taffy::geometry::Size { width: taffy::style::Dimension::Length(10f32), height: auto() },
        inset: taffy::geometry::Rect {
            left: auto(),
            right: auto(),
            top: taffy::style::LengthPercentageAuto::Percent(0.1f32),
            bottom: taffy::style::LengthPercentageAuto::Percent(0.1f32),
        },
        ..Default::default()
    });
    let node = taffy.new_with_children(
        taffy::style::Style {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Length(100f32),
                height: taffy::style::Dimension::Length(200f32),
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
    assert_eq!(size.width, 100f32, "width of node {:?}. Expected {}. Actual {}", node, 100f32, size.width);
    assert_eq!(size.height, 200f32, "height of node {:?}. Expected {}. Actual {}", node, 200f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node0);
    assert_eq!(size.width, 10f32, "width of node {:?}. Expected {}. Actual {}", node0, 10f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node0, 10f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node0, 0f32, location.x);
    assert_eq!(location.y, 100f32, "y of node {:?}. Expected {}. Actual {}", node0, 100f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node1);
    assert_eq!(size.width, 10f32, "width of node {:?}. Expected {}. Actual {}", node1, 10f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node1, 10f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node1, 0f32, location.x);
    assert_eq!(location.y, 90f32, "y of node {:?}. Expected {}. Actual {}", node1, 90f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node2);
    assert_eq!(size.width, 10f32, "width of node {:?}. Expected {}. Actual {}", node2, 10f32, size.width);
    assert_eq!(size.height, 160f32, "height of node {:?}. Expected {}. Actual {}", node2, 160f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node2, 0f32, location.x);
    assert_eq!(location.y, 20f32, "y of node {:?}. Expected {}. Actual {}", node2, 20f32, location.y);
}
