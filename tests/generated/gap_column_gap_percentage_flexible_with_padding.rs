#[test]
fn gap_column_gap_percentage_flexible_with_padding() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, tree::Layout};
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy.new_leaf(taffy::style::Style {
        flex_grow: 1f32,
        flex_shrink: 1f32,
        flex_basis: taffy::style::Dimension::Percent(0f32),
        ..Default::default()
    });
    let node1 = taffy.new_leaf(taffy::style::Style {
        flex_grow: 1f32,
        flex_shrink: 1f32,
        flex_basis: taffy::style::Dimension::Percent(0f32),
        ..Default::default()
    });
    let node2 = taffy.new_leaf(taffy::style::Style {
        flex_grow: 1f32,
        flex_shrink: 1f32,
        flex_basis: taffy::style::Dimension::Percent(0f32),
        ..Default::default()
    });
    let node = taffy.new_with_children(
        taffy::style::Style {
            gap: taffy::geometry::Size {
                width: taffy::style::LengthPercentage::Percent(0.1f32),
                height: taffy::style::LengthPercentage::Length(20f32),
            },
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Length(100f32),
                height: taffy::style::Dimension::Length(100f32),
            },
            padding: taffy::geometry::Rect {
                left: taffy::style::LengthPercentage::Length(10f32),
                right: taffy::style::LengthPercentage::Length(10f32),
                top: taffy::style::LengthPercentage::Length(10f32),
                bottom: taffy::style::LengthPercentage::Length(10f32),
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
    assert_eq!(size.height, 100f32, "height of node {:?}. Expected {}. Actual {}", node, 100f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node0);
    assert_eq!(size.width, 21f32, "width of node {:?}. Expected {}. Actual {}", node0, 21f32, size.width);
    assert_eq!(size.height, 80f32, "height of node {:?}. Expected {}. Actual {}", node0, 80f32, size.height);
    assert_eq!(location.x, 10f32, "x of node {:?}. Expected {}. Actual {}", node0, 10f32, location.x);
    assert_eq!(location.y, 10f32, "y of node {:?}. Expected {}. Actual {}", node0, 10f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node1);
    assert_eq!(size.width, 22f32, "width of node {:?}. Expected {}. Actual {}", node1, 22f32, size.width);
    assert_eq!(size.height, 80f32, "height of node {:?}. Expected {}. Actual {}", node1, 80f32, size.height);
    assert_eq!(location.x, 39f32, "x of node {:?}. Expected {}. Actual {}", node1, 39f32, location.x);
    assert_eq!(location.y, 10f32, "y of node {:?}. Expected {}. Actual {}", node1, 10f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node2);
    assert_eq!(size.width, 21f32, "width of node {:?}. Expected {}. Actual {}", node2, 21f32, size.width);
    assert_eq!(size.height, 80f32, "height of node {:?}. Expected {}. Actual {}", node2, 80f32, size.height);
    assert_eq!(location.x, 69f32, "x of node {:?}. Expected {}. Actual {}", node2, 69f32, location.x);
    assert_eq!(location.y, 10f32, "y of node {:?}. Expected {}. Actual {}", node2, 10f32, location.y);
}
