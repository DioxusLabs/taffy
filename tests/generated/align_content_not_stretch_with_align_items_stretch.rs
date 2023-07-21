#[test]
fn align_content_not_stretch_with_align_items_stretch() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, tree::Layout};
    let mut taffy = taffy::Taffy::new();
    let node00 = taffy.new_leaf(taffy::style::Style {
        size: taffy::geometry::Size {
            width: taffy::style::Dimension::Length(272f32),
            height: taffy::style::Dimension::Length(44f32),
        },
        ..Default::default()
    });
    let node0 = taffy.new_with_children(
        taffy::style::Style { flex_direction: taffy::style::FlexDirection::Column, ..Default::default() },
        &[node00],
    );
    let node10 = taffy.new_leaf(taffy::style::Style {
        size: taffy::geometry::Size {
            width: taffy::style::Dimension::Length(56f32),
            height: taffy::style::Dimension::Length(44f32),
        },
        ..Default::default()
    });
    let node1 = taffy.new_with_children(
        taffy::style::Style { flex_direction: taffy::style::FlexDirection::Column, ..Default::default() },
        &[node10],
    );
    let node = taffy.new_with_children(
        taffy::style::Style {
            flex_wrap: taffy::style::FlexWrap::Wrap,
            align_content: Some(taffy::style::AlignContent::FlexStart),
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Length(328f32),
                height: taffy::style::Dimension::Length(52f32),
            },
            ..Default::default()
        },
        &[node0, node1],
    );
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT);
    println!("\nComputed tree:");
    taffy::util::print_tree(&taffy, node);
    println!();
    let Layout { size, location, .. } = taffy.layout(node);
    assert_eq!(size.width, 328f32, "width of node {:?}. Expected {}. Actual {}", node, 328f32, size.width);
    assert_eq!(size.height, 52f32, "height of node {:?}. Expected {}. Actual {}", node, 52f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node0);
    assert_eq!(size.width, 272f32, "width of node {:?}. Expected {}. Actual {}", node0, 272f32, size.width);
    assert_eq!(size.height, 44f32, "height of node {:?}. Expected {}. Actual {}", node0, 44f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node0, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node0, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node00);
    assert_eq!(size.width, 272f32, "width of node {:?}. Expected {}. Actual {}", node00, 272f32, size.width);
    assert_eq!(size.height, 44f32, "height of node {:?}. Expected {}. Actual {}", node00, 44f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node00, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node00, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node1);
    assert_eq!(size.width, 56f32, "width of node {:?}. Expected {}. Actual {}", node1, 56f32, size.width);
    assert_eq!(size.height, 44f32, "height of node {:?}. Expected {}. Actual {}", node1, 44f32, size.height);
    assert_eq!(location.x, 272f32, "x of node {:?}. Expected {}. Actual {}", node1, 272f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node1, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node10);
    assert_eq!(size.width, 56f32, "width of node {:?}. Expected {}. Actual {}", node10, 56f32, size.width);
    assert_eq!(size.height, 44f32, "height of node {:?}. Expected {}. Actual {}", node10, 44f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node10, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node10, 0f32, location.y);
}
