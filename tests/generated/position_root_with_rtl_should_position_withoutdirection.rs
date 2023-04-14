#[test]
fn position_root_with_rtl_should_position_withoutdirection() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, tree::Layout};
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy
        .new_leaf(taffy::style::Style {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Points(52f32),
                height: taffy::style::Dimension::Points(52f32),
            },
            inset: taffy::geometry::Rect {
                left: taffy::style::LengthPercentageAuto::Points(72f32),
                right: auto(),
                top: auto(),
                bottom: auto(),
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
    assert_eq!(size.width, 52f32, "width of node {:?}. Expected {}. Actual {}", node, 52f32, size.width);
    assert_eq!(size.height, 52f32, "height of node {:?}. Expected {}. Actual {}", node, 52f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node0).unwrap();
    assert_eq!(size.width, 52f32, "width of node {:?}. Expected {}. Actual {}", node0, 52f32, size.width);
    assert_eq!(size.height, 52f32, "height of node {:?}. Expected {}. Actual {}", node0, 52f32, size.height);
    assert_eq!(location.x, 72f32, "x of node {:?}. Expected {}. Actual {}", node0, 72f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node0, 0f32, location.y);
}
