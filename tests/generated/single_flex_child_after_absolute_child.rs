#[test]
fn single_flex_child_after_absolute_child() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, tree::Layout};
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy.new_leaf(taffy::style::Style {
        position: taffy::style::Position::Absolute,
        size: taffy::geometry::Size {
            width: taffy::style::Dimension::Percent(1f32),
            height: taffy::style::Dimension::Percent(1f32),
        },
        ..Default::default()
    });
    let node1 = taffy.new_leaf(taffy::style::Style { flex_grow: 1f32, flex_shrink: 1f32, ..Default::default() });
    let node2 = taffy.new_leaf(taffy::style::Style {
        flex_shrink: 0f32,
        flex_basis: taffy::style::Dimension::Length(174f32),
        ..Default::default()
    });
    let node = taffy.new_with_children(
        taffy::style::Style {
            flex_direction: taffy::style::FlexDirection::Column,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Length(428f32),
                height: taffy::style::Dimension::Length(845f32),
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
    assert_eq!(size.width, 428f32, "width of node {:?}. Expected {}. Actual {}", node, 428f32, size.width);
    assert_eq!(size.height, 845f32, "height of node {:?}. Expected {}. Actual {}", node, 845f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node0);
    assert_eq!(size.width, 428f32, "width of node {:?}. Expected {}. Actual {}", node0, 428f32, size.width);
    assert_eq!(size.height, 845f32, "height of node {:?}. Expected {}. Actual {}", node0, 845f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node0, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node0, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node1);
    assert_eq!(size.width, 428f32, "width of node {:?}. Expected {}. Actual {}", node1, 428f32, size.width);
    assert_eq!(size.height, 671f32, "height of node {:?}. Expected {}. Actual {}", node1, 671f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node1, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node1, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node2);
    assert_eq!(size.width, 428f32, "width of node {:?}. Expected {}. Actual {}", node2, 428f32, size.width);
    assert_eq!(size.height, 174f32, "height of node {:?}. Expected {}. Actual {}", node2, 174f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node2, 0f32, location.x);
    assert_eq!(location.y, 671f32, "y of node {:?}. Expected {}. Actual {}", node2, 671f32, location.y);
}
