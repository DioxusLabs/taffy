#[test]
fn bevy_issue_8017() {
    #[allow(unused_imports)]
    use taffy::{
        prelude::*,
        tree::{Layout, MeasureFunc},
        Taffy,
    };
    let mut taffy: Taffy<MeasureFunc<()>> = Taffy::new();
    let node00 = taffy
        .new_leaf(taffy::style::Style {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Percent(1f32),
                height: taffy::style::Dimension::Percent(1f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node01 = taffy
        .new_leaf(taffy::style::Style {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Percent(1f32),
                height: taffy::style::Dimension::Percent(1f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node0 = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Flex,
                gap: taffy::geometry::Size {
                    width: taffy::style::LengthPercentage::Length(8f32),
                    height: taffy::style::LengthPercentage::Length(8f32),
                },
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Percent(1f32),
                    height: taffy::style::Dimension::Percent(0.5f32),
                },
                ..Default::default()
            },
            &[node00, node01],
        )
        .unwrap();
    let node10 = taffy
        .new_leaf(taffy::style::Style {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Percent(1f32),
                height: taffy::style::Dimension::Percent(1f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node11 = taffy
        .new_leaf(taffy::style::Style {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Percent(1f32),
                height: taffy::style::Dimension::Percent(1f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node1 = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Flex,
                gap: taffy::geometry::Size {
                    width: taffy::style::LengthPercentage::Length(8f32),
                    height: taffy::style::LengthPercentage::Length(8f32),
                },
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Percent(1f32),
                    height: taffy::style::Dimension::Percent(0.5f32),
                },
                ..Default::default()
            },
            &[node10, node11],
        )
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Flex,
                flex_direction: taffy::style::FlexDirection::Column,
                gap: taffy::geometry::Size {
                    width: taffy::style::LengthPercentage::Length(8f32),
                    height: taffy::style::LengthPercentage::Length(8f32),
                },
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Length(400f32),
                    height: taffy::style::Dimension::Length(400f32),
                },
                padding: taffy::geometry::Rect {
                    left: taffy::style::LengthPercentage::Length(8f32),
                    right: taffy::style::LengthPercentage::Length(8f32),
                    top: taffy::style::LengthPercentage::Length(8f32),
                    bottom: taffy::style::LengthPercentage::Length(8f32),
                },
                ..Default::default()
            },
            &[node0, node1],
        )
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT).unwrap();
    println!("\nComputed tree:");
    taffy::util::print_tree(&taffy, node);
    println!();
    let Layout { size, location, .. } = taffy.layout(node).unwrap();
    assert_eq!(size.width, 400f32, "width of node {:?}. Expected {}. Actual {}", node, 400f32, size.width);
    assert_eq!(size.height, 400f32, "height of node {:?}. Expected {}. Actual {}", node, 400f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node0).unwrap();
    assert_eq!(size.width, 384f32, "width of node {:?}. Expected {}. Actual {}", node0, 384f32, size.width);
    assert_eq!(size.height, 188f32, "height of node {:?}. Expected {}. Actual {}", node0, 188f32, size.height);
    assert_eq!(location.x, 8f32, "x of node {:?}. Expected {}. Actual {}", node0, 8f32, location.x);
    assert_eq!(location.y, 8f32, "y of node {:?}. Expected {}. Actual {}", node0, 8f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node00).unwrap();
    assert_eq!(size.width, 188f32, "width of node {:?}. Expected {}. Actual {}", node00, 188f32, size.width);
    assert_eq!(size.height, 188f32, "height of node {:?}. Expected {}. Actual {}", node00, 188f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node00, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node00, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node01).unwrap();
    assert_eq!(size.width, 188f32, "width of node {:?}. Expected {}. Actual {}", node01, 188f32, size.width);
    assert_eq!(size.height, 188f32, "height of node {:?}. Expected {}. Actual {}", node01, 188f32, size.height);
    assert_eq!(location.x, 196f32, "x of node {:?}. Expected {}. Actual {}", node01, 196f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node01, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node1).unwrap();
    assert_eq!(size.width, 384f32, "width of node {:?}. Expected {}. Actual {}", node1, 384f32, size.width);
    assert_eq!(size.height, 188f32, "height of node {:?}. Expected {}. Actual {}", node1, 188f32, size.height);
    assert_eq!(location.x, 8f32, "x of node {:?}. Expected {}. Actual {}", node1, 8f32, location.x);
    assert_eq!(location.y, 204f32, "y of node {:?}. Expected {}. Actual {}", node1, 204f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node10).unwrap();
    assert_eq!(size.width, 188f32, "width of node {:?}. Expected {}. Actual {}", node10, 188f32, size.width);
    assert_eq!(size.height, 188f32, "height of node {:?}. Expected {}. Actual {}", node10, 188f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node10, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node10, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node11).unwrap();
    assert_eq!(size.width, 188f32, "width of node {:?}. Expected {}. Actual {}", node11, 188f32, size.width);
    assert_eq!(size.height, 188f32, "height of node {:?}. Expected {}. Actual {}", node11, 188f32, size.height);
    assert_eq!(location.x, 196f32, "x of node {:?}. Expected {}. Actual {}", node11, 196f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node11, 0f32, location.y);
}
