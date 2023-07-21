#[test]
fn rounding_fractial_input_7() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, tree::Layout};
    let mut taffy = taffy::Taffy::new();
    let node00 = taffy.new_leaf(taffy::style::Style {
        size: taffy::geometry::Size {
            width: taffy::style::Dimension::Length(1f32),
            height: taffy::style::Dimension::Length(10f32),
        },
        ..Default::default()
    });
    let node01 = taffy.new_leaf(taffy::style::Style {
        size: taffy::geometry::Size {
            width: taffy::style::Dimension::Length(1f32),
            height: taffy::style::Dimension::Length(10f32),
        },
        ..Default::default()
    });
    let node0 = taffy.new_with_children(
        taffy::style::Style {
            flex_wrap: taffy::style::FlexWrap::Wrap,
            size: taffy::geometry::Size { width: taffy::style::Dimension::Percent(0.25f32), height: auto() },
            ..Default::default()
        },
        &[node00, node01],
    );
    let node10 = taffy.new_leaf(taffy::style::Style {
        size: taffy::geometry::Size {
            width: taffy::style::Dimension::Length(1f32),
            height: taffy::style::Dimension::Length(10f32),
        },
        ..Default::default()
    });
    let node11 = taffy.new_leaf(taffy::style::Style {
        size: taffy::geometry::Size {
            width: taffy::style::Dimension::Length(1f32),
            height: taffy::style::Dimension::Length(10f32),
        },
        ..Default::default()
    });
    let node1 = taffy.new_with_children(
        taffy::style::Style {
            flex_wrap: taffy::style::FlexWrap::Wrap,
            size: taffy::geometry::Size { width: taffy::style::Dimension::Percent(0.25f32), height: auto() },
            ..Default::default()
        },
        &[node10, node11],
    );
    let node20 = taffy.new_leaf(taffy::style::Style {
        size: taffy::geometry::Size {
            width: taffy::style::Dimension::Length(1f32),
            height: taffy::style::Dimension::Length(10f32),
        },
        ..Default::default()
    });
    let node21 = taffy.new_leaf(taffy::style::Style {
        size: taffy::geometry::Size {
            width: taffy::style::Dimension::Length(1f32),
            height: taffy::style::Dimension::Length(10f32),
        },
        ..Default::default()
    });
    let node2 = taffy.new_with_children(
        taffy::style::Style {
            flex_wrap: taffy::style::FlexWrap::Wrap,
            size: taffy::geometry::Size { width: taffy::style::Dimension::Percent(0.25f32), height: auto() },
            ..Default::default()
        },
        &[node20, node21],
    );
    let node30 = taffy.new_leaf(taffy::style::Style {
        size: taffy::geometry::Size {
            width: taffy::style::Dimension::Length(1f32),
            height: taffy::style::Dimension::Length(10f32),
        },
        ..Default::default()
    });
    let node31 = taffy.new_leaf(taffy::style::Style {
        size: taffy::geometry::Size {
            width: taffy::style::Dimension::Length(1f32),
            height: taffy::style::Dimension::Length(10f32),
        },
        ..Default::default()
    });
    let node3 = taffy.new_with_children(
        taffy::style::Style {
            flex_wrap: taffy::style::FlexWrap::Wrap,
            size: taffy::geometry::Size { width: taffy::style::Dimension::Percent(0.25f32), height: auto() },
            ..Default::default()
        },
        &[node30, node31],
    );
    let node = taffy.new_with_children(
        taffy::style::Style {
            size: taffy::geometry::Size { width: taffy::style::Dimension::Length(7f32), height: auto() },
            ..Default::default()
        },
        &[node0, node1, node2, node3],
    );
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT);
    println!("\nComputed tree:");
    taffy::util::print_tree(&taffy, node);
    println!();
    let Layout { size, location, .. } = taffy.layout(node);
    assert_eq!(size.width, 7f32, "width of node {:?}. Expected {}. Actual {}", node, 7f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node, 20f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node0);
    assert_eq!(size.width, 2f32, "width of node {:?}. Expected {}. Actual {}", node0, 2f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node0, 20f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node0, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node0, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node00);
    assert_eq!(size.width, 1f32, "width of node {:?}. Expected {}. Actual {}", node00, 1f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node00, 10f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node00, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node00, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node01);
    assert_eq!(size.width, 1f32, "width of node {:?}. Expected {}. Actual {}", node01, 1f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node01, 10f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node01, 0f32, location.x);
    assert_eq!(location.y, 10f32, "y of node {:?}. Expected {}. Actual {}", node01, 10f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node1);
    assert_eq!(size.width, 2f32, "width of node {:?}. Expected {}. Actual {}", node1, 2f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node1, 20f32, size.height);
    assert_eq!(location.x, 2f32, "x of node {:?}. Expected {}. Actual {}", node1, 2f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node1, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node10);
    assert_eq!(size.width, 1f32, "width of node {:?}. Expected {}. Actual {}", node10, 1f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node10, 10f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node10, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node10, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node11);
    assert_eq!(size.width, 1f32, "width of node {:?}. Expected {}. Actual {}", node11, 1f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node11, 10f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node11, 0f32, location.x);
    assert_eq!(location.y, 10f32, "y of node {:?}. Expected {}. Actual {}", node11, 10f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node2);
    assert_eq!(size.width, 1f32, "width of node {:?}. Expected {}. Actual {}", node2, 1f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node2, 20f32, size.height);
    assert_eq!(location.x, 4f32, "x of node {:?}. Expected {}. Actual {}", node2, 4f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node2, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node20);
    assert_eq!(size.width, 1f32, "width of node {:?}. Expected {}. Actual {}", node20, 1f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node20, 10f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node20, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node20, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node21);
    assert_eq!(size.width, 1f32, "width of node {:?}. Expected {}. Actual {}", node21, 1f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node21, 10f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node21, 0f32, location.x);
    assert_eq!(location.y, 10f32, "y of node {:?}. Expected {}. Actual {}", node21, 10f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node3);
    assert_eq!(size.width, 2f32, "width of node {:?}. Expected {}. Actual {}", node3, 2f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node3, 20f32, size.height);
    assert_eq!(location.x, 5f32, "x of node {:?}. Expected {}. Actual {}", node3, 5f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node3, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node30);
    assert_eq!(size.width, 1f32, "width of node {:?}. Expected {}. Actual {}", node30, 1f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node30, 10f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node30, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node30, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node31);
    assert_eq!(size.width, 1f32, "width of node {:?}. Expected {}. Actual {}", node31, 1f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node31, 10f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node31, 0f32, location.x);
    assert_eq!(location.y, 10f32, "y of node {:?}. Expected {}. Actual {}", node31, 10f32, location.y);
}
