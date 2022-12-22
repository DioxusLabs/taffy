#[test]
fn size_defined_by_child_with_padding() {
    #[allow(unused_imports)]
    use taffy::prelude::*;
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy
        .new_leaf(taffy::style::Style {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Points(10f32),
                height: taffy::style::Dimension::Points(10f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
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
    assert_eq!(taffy.layout(node).unwrap().size.width, 30f32);
    assert_eq!(taffy.layout(node).unwrap().size.height, 30f32);
    assert_eq!(taffy.layout(node).unwrap().location.x, 0f32);
    assert_eq!(taffy.layout(node).unwrap().location.y, 0f32);
    assert_eq!(taffy.layout(node0).unwrap().size.width, 10f32);
    assert_eq!(taffy.layout(node0).unwrap().size.height, 10f32);
    assert_eq!(taffy.layout(node0).unwrap().location.x, 10f32);
    assert_eq!(taffy.layout(node0).unwrap().location.y, 10f32);
}
