#[test]
fn absolute_padding_border_overrides_max_size() {
    use slotmap::Key;
    #[allow(unused_imports)]
    use taffy::{layout::Layout, prelude::*};
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy
        .new_leaf(taffy::style::Style {
            position: taffy::style::Position::Absolute,
            max_size: taffy::geometry::Size {
                width: taffy::style::Dimension::Points(12f32),
                height: taffy::style::Dimension::Points(12f32),
            },
            padding: taffy::geometry::Rect {
                left: taffy::style::LengthPercentage::Points(8f32),
                right: taffy::style::LengthPercentage::Points(4f32),
                top: taffy::style::LengthPercentage::Points(2f32),
                bottom: taffy::style::LengthPercentage::Points(6f32),
            },
            border: taffy::geometry::Rect {
                left: taffy::style::LengthPercentage::Points(7f32),
                right: taffy::style::LengthPercentage::Points(3f32),
                top: taffy::style::LengthPercentage::Points(1f32),
                bottom: taffy::style::LengthPercentage::Points(5f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node = taffy.new_with_children(taffy::style::Style { ..Default::default() }, &[node0]).unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT).unwrap();
    println!("\nComputed tree:");
    taffy::debug::print_tree(&taffy, node);
    println!();
    let Layout { size, location, .. } = taffy.layout(node).unwrap();
    assert_eq!(size.width, 0f32, "width of node {:?}. Expected {}. Actual {}", node.data(), 0f32, size.width);
    assert_eq!(size.height, 0f32, "height of node {:?}. Expected {}. Actual {}", node.data(), 0f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node.data(), 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node.data(), 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node0).unwrap();
    assert_eq!(size.width, 22f32, "width of node {:?}. Expected {}. Actual {}", node0.data(), 22f32, size.width);
    assert_eq!(size.height, 14f32, "height of node {:?}. Expected {}. Actual {}", node0.data(), 14f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node0.data(), 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node0.data(), 0f32, location.y);
}
