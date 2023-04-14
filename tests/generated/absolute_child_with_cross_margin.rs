#[test]
fn absolute_child_with_cross_margin() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, tree::Layout};
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy
        .new_leaf(taffy::style::Style {
            align_content: Some(taffy::style::AlignContent::Stretch),
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Points(28f32),
                height: taffy::style::Dimension::Points(27f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node1 = taffy
        .new_leaf_with_measure(
            taffy::style::Style {
                position: taffy::style::Position::Absolute,
                align_content: Some(taffy::style::AlignContent::Stretch),
                flex_grow: 0f32,
                flex_shrink: 1f32,
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Percent(1f32),
                    height: taffy::style::Dimension::Points(15f32),
                },
                margin: taffy::geometry::Rect {
                    left: zero(),
                    right: zero(),
                    top: taffy::style::LengthPercentageAuto::Points(4f32),
                    bottom: zero(),
                },
                ..Default::default()
            },
            taffy::tree::MeasureFunc::Raw(|known_dimensions, available_space| {
                const TEXT: &str = "\n  ";
                super::measure_standard_text(
                    known_dimensions,
                    available_space,
                    TEXT,
                    super::WritingMode::Horizontal,
                    None,
                )
            }),
        )
        .unwrap();
    let node2 = taffy
        .new_leaf(taffy::style::Style {
            align_content: Some(taffy::style::AlignContent::Stretch),
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Points(25f32),
                height: taffy::style::Dimension::Points(27f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                justify_content: Some(taffy::style::JustifyContent::SpaceBetween),
                min_size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(311f32),
                    height: taffy::style::Dimension::Points(0f32),
                },
                max_size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(311f32),
                    height: taffy::style::Dimension::Points(36893500000000000000f32),
                },
                ..Default::default()
            },
            &[node0, node1, node2],
        )
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT).unwrap();
    println!("\nComputed tree:");
    taffy::debug::print_tree(&taffy, node);
    println!();
    let Layout { size, location, .. } = taffy.layout(node).unwrap();
    assert_eq!(size.width, 311f32, "width of node {:?}. Expected {}. Actual {}", node, 311f32, size.width);
    assert_eq!(size.height, 27f32, "height of node {:?}. Expected {}. Actual {}", node, 27f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node0).unwrap();
    assert_eq!(size.width, 28f32, "width of node {:?}. Expected {}. Actual {}", node0, 28f32, size.width);
    assert_eq!(size.height, 27f32, "height of node {:?}. Expected {}. Actual {}", node0, 27f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node0, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node0, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node1).unwrap();
    assert_eq!(size.width, 311f32, "width of node {:?}. Expected {}. Actual {}", node1, 311f32, size.width);
    assert_eq!(size.height, 15f32, "height of node {:?}. Expected {}. Actual {}", node1, 15f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node1, 0f32, location.x);
    assert_eq!(location.y, 4f32, "y of node {:?}. Expected {}. Actual {}", node1, 4f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node2).unwrap();
    assert_eq!(size.width, 25f32, "width of node {:?}. Expected {}. Actual {}", node2, 25f32, size.width);
    assert_eq!(size.height, 27f32, "height of node {:?}. Expected {}. Actual {}", node2, 27f32, size.height);
    assert_eq!(location.x, 286f32, "x of node {:?}. Expected {}. Actual {}", node2, 286f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node2, 0f32, location.y);
}
