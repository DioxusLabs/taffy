#[test]
fn gap_column_gap_child_margins() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, tree::Layout};
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy.new_leaf(taffy::style::Style {
        flex_grow: 1f32,
        flex_shrink: 1f32,
        flex_basis: taffy::style::Dimension::Percent(0f32),
        margin: taffy::geometry::Rect {
            left: taffy::style::LengthPercentageAuto::Length(2f32),
            right: taffy::style::LengthPercentageAuto::Length(2f32),
            top: zero(),
            bottom: zero(),
        },
        ..Default::default()
    });
    let node1 = taffy.new_leaf(taffy::style::Style {
        flex_grow: 1f32,
        flex_shrink: 1f32,
        flex_basis: taffy::style::Dimension::Percent(0f32),
        margin: taffy::geometry::Rect {
            left: taffy::style::LengthPercentageAuto::Length(10f32),
            right: taffy::style::LengthPercentageAuto::Length(10f32),
            top: zero(),
            bottom: zero(),
        },
        ..Default::default()
    });
    let node2 = taffy.new_leaf(taffy::style::Style {
        flex_grow: 1f32,
        flex_shrink: 1f32,
        flex_basis: taffy::style::Dimension::Percent(0f32),
        margin: taffy::geometry::Rect {
            left: taffy::style::LengthPercentageAuto::Length(15f32),
            right: taffy::style::LengthPercentageAuto::Length(15f32),
            top: zero(),
            bottom: zero(),
        },
        ..Default::default()
    });
    let node = taffy.new_with_children(
        taffy::style::Style {
            gap: taffy::geometry::Size { width: taffy::style::LengthPercentage::Length(10f32), height: zero() },
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Length(80f32),
                height: taffy::style::Dimension::Length(100f32),
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
    assert_eq!(size.width, 80f32, "width of node {:?}. Expected {}. Actual {}", node, 80f32, size.width);
    assert_eq!(size.height, 100f32, "height of node {:?}. Expected {}. Actual {}", node, 100f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node0);
    assert_eq!(size.width, 2f32, "width of node {:?}. Expected {}. Actual {}", node0, 2f32, size.width);
    assert_eq!(size.height, 100f32, "height of node {:?}. Expected {}. Actual {}", node0, 100f32, size.height);
    assert_eq!(location.x, 2f32, "x of node {:?}. Expected {}. Actual {}", node0, 2f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node0, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node1);
    assert_eq!(size.width, 2f32, "width of node {:?}. Expected {}. Actual {}", node1, 2f32, size.width);
    assert_eq!(size.height, 100f32, "height of node {:?}. Expected {}. Actual {}", node1, 100f32, size.height);
    assert_eq!(location.x, 26f32, "x of node {:?}. Expected {}. Actual {}", node1, 26f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node1, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node2);
    assert_eq!(size.width, 2f32, "width of node {:?}. Expected {}. Actual {}", node2, 2f32, size.width);
    assert_eq!(size.height, 100f32, "height of node {:?}. Expected {}. Actual {}", node2, 100f32, size.height);
    assert_eq!(location.x, 63f32, "x of node {:?}. Expected {}. Actual {}", node2, 63f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node2, 0f32, location.y);
}
