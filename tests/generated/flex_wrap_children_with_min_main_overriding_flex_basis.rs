#[test]
fn flex_wrap_children_with_min_main_overriding_flex_basis() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, tree::Layout};
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy.new_leaf(taffy::style::Style {
        flex_basis: taffy::style::Dimension::Length(50f32),
        size: taffy::geometry::Size { width: auto(), height: taffy::style::Dimension::Length(50f32) },
        min_size: taffy::geometry::Size { width: taffy::style::Dimension::Length(55f32), height: auto() },
        ..Default::default()
    });
    let node1 = taffy.new_leaf(taffy::style::Style {
        flex_basis: taffy::style::Dimension::Length(50f32),
        size: taffy::geometry::Size { width: auto(), height: taffy::style::Dimension::Length(50f32) },
        min_size: taffy::geometry::Size { width: taffy::style::Dimension::Length(55f32), height: auto() },
        ..Default::default()
    });
    let node = taffy.new_with_children(
        taffy::style::Style {
            flex_wrap: taffy::style::FlexWrap::Wrap,
            size: taffy::geometry::Size { width: taffy::style::Dimension::Length(100f32), height: auto() },
            ..Default::default()
        },
        &[node0, node1],
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
    assert_eq!(size.width, 55f32, "width of node {:?}. Expected {}. Actual {}", node0, 55f32, size.width);
    assert_eq!(size.height, 50f32, "height of node {:?}. Expected {}. Actual {}", node0, 50f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node0, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node0, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node1);
    assert_eq!(size.width, 55f32, "width of node {:?}. Expected {}. Actual {}", node1, 55f32, size.width);
    assert_eq!(size.height, 50f32, "height of node {:?}. Expected {}. Actual {}", node1, 50f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node1, 0f32, location.x);
    assert_eq!(location.y, 50f32, "y of node {:?}. Expected {}. Actual {}", node1, 50f32, location.y);
}
