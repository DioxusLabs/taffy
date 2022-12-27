#[test]
fn measure_child_constraint_padding_parent() {
    use slotmap::Key;
    #[allow(unused_imports)]
    use taffy::{layout::Layout, prelude::*};
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy . new_leaf_with_measure (taffy :: style :: Style { .. Default :: default () } , taffy :: node :: MeasureFunc :: Raw (| known_dimensions , available_space | { const TEXT : & str = "HHHHHHHHHH\u{200b}HHHHHHHHHH\u{200b}HHHHHHHHHH\u{200b}HHHHHHHHHH\u{200b}HHHHHHHHHH\u{200b}HHHHHHHHHH\u{200b}HHHHHHHHHH\u{200b}HHHHHHHHHH\u{200b}HHHHHHHHHH\u{200b}HHHHHHHHHH" ; super :: measure_standard_text (known_dimensions , available_space , TEXT , super :: WritingMode :: Horizontal) }) ,) . unwrap () ;
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(50f32),
                    height: taffy::style::Dimension::Auto,
                },
                padding: taffy::geometry::Rect {
                    left: taffy::style::LengthPercentage::Points(10f32),
                    right: taffy::style::LengthPercentage::Points(10f32),
                    top: taffy::style::LengthPercentage::Points(10f32),
                    bottom: taffy::style::LengthPercentage::Points(10f32),
                },
                ..Default::default()
            },
            &[node0],
        )
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT).unwrap();
    println!("\nComputed tree:");
    taffy::debug::print_tree(&taffy, node);
    println!();
    let Layout { size, location, .. } = taffy.layout(node).unwrap();
    assert_eq!(size.width, 50f32, "width of node {:?}. Expected {}. Actual {}", node.data(), 50f32, size.width);
    assert_eq!(size.height, 120f32, "height of node {:?}. Expected {}. Actual {}", node.data(), 120f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node.data(), 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node.data(), 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node0).unwrap();
    assert_eq!(size.width, 100f32, "width of node {:?}. Expected {}. Actual {}", node0.data(), 100f32, size.width);
    assert_eq!(size.height, 100f32, "height of node {:?}. Expected {}. Actual {}", node0.data(), 100f32, size.height);
    assert_eq!(location.x, 10f32, "x of node {:?}. Expected {}. Actual {}", node0.data(), 10f32, location.x);
    assert_eq!(location.y, 10f32, "y of node {:?}. Expected {}. Actual {}", node0.data(), 10f32, location.y);
}
