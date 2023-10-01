#[test]
fn block_inset_fixed() {
    #[allow(unused_imports)]
    use taffy::{
        prelude::*,
        tree::{Layout, MeasureFunc},
        Taffy,
    };
    let mut taffy: Taffy<crate::TextMeasure> = Taffy::new();
    let node0 = taffy
        .new_leaf(taffy::style::Style {
            size: taffy::geometry::Size { width: auto(), height: taffy::style::Dimension::Length(10f32) },
            inset: taffy::geometry::Rect {
                left: taffy::style::LengthPercentageAuto::Length(2f32),
                right: auto(),
                top: taffy::style::LengthPercentageAuto::Length(4f32),
                bottom: auto(),
            },
            ..Default::default()
        })
        .unwrap();
    let node1 = taffy
        .new_leaf(taffy::style::Style {
            size: taffy::geometry::Size { width: auto(), height: taffy::style::Dimension::Length(10f32) },
            inset: taffy::geometry::Rect {
                left: auto(),
                right: taffy::style::LengthPercentageAuto::Length(6f32),
                top: auto(),
                bottom: taffy::style::LengthPercentageAuto::Length(8f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node2 = taffy
        .new_leaf(taffy::style::Style {
            size: taffy::geometry::Size { width: auto(), height: taffy::style::Dimension::Length(10f32) },
            inset: taffy::geometry::Rect {
                left: taffy::style::LengthPercentageAuto::Length(2f32),
                right: taffy::style::LengthPercentageAuto::Length(6f32),
                top: taffy::style::LengthPercentageAuto::Length(4f32),
                bottom: taffy::style::LengthPercentageAuto::Length(8f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Block,
                size: taffy::geometry::Size { width: taffy::style::Dimension::Length(50f32), height: auto() },
                ..Default::default()
            },
            &[node0, node1, node2],
        )
        .unwrap();
    taffy.compute_layout_with_measure(node, taffy::geometry::Size::MAX_CONTENT, crate::test_measure_function).unwrap();
    println!("\nComputed tree:");
    taffy::util::print_tree(&taffy, node);
    println!();
    let Layout { size, location, .. } = taffy.layout(node).unwrap();
    assert_eq!(size.width, 50f32, "width of node {:?}. Expected {}. Actual {}", node, 50f32, size.width);
    assert_eq!(size.height, 30f32, "height of node {:?}. Expected {}. Actual {}", node, 30f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node0).unwrap();
    assert_eq!(size.width, 50f32, "width of node {:?}. Expected {}. Actual {}", node0, 50f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node0, 10f32, size.height);
    assert_eq!(location.x, 2f32, "x of node {:?}. Expected {}. Actual {}", node0, 2f32, location.x);
    assert_eq!(location.y, 4f32, "y of node {:?}. Expected {}. Actual {}", node0, 4f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node1).unwrap();
    assert_eq!(size.width, 50f32, "width of node {:?}. Expected {}. Actual {}", node1, 50f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node1, 10f32, size.height);
    assert_eq!(location.x, -6f32, "x of node {:?}. Expected {}. Actual {}", node1, -6f32, location.x);
    assert_eq!(location.y, 2f32, "y of node {:?}. Expected {}. Actual {}", node1, 2f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node2).unwrap();
    assert_eq!(size.width, 50f32, "width of node {:?}. Expected {}. Actual {}", node2, 50f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node2, 10f32, size.height);
    assert_eq!(location.x, 2f32, "x of node {:?}. Expected {}. Actual {}", node2, 2f32, location.x);
    assert_eq!(location.y, 24f32, "y of node {:?}. Expected {}. Actual {}", node2, 24f32, location.y);
}
