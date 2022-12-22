#[test]
fn max_height_overrides_height_on_root() {
    #[allow(unused_imports)]
    use taffy::prelude::*;
    let mut taffy = taffy::Taffy::new();
    let node = taffy
        .new_leaf(taffy::style::Style {
            size: taffy::geometry::Size { width: auto(), height: taffy::style::Dimension::Points(200f32) },
            max_size: taffy::geometry::Size { width: auto(), height: taffy::style::Dimension::Points(100f32) },
            ..Default::default()
        })
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT).unwrap();
    println!("\nComputed tree:");
    taffy::debug::print_tree(&taffy, node);
    println!();
    assert_eq!(taffy.layout(node).unwrap().size.width, 0f32);
    assert_eq!(taffy.layout(node).unwrap().size.height, 100f32);
    assert_eq!(taffy.layout(node).unwrap().location.x, 0f32);
    assert_eq!(taffy.layout(node).unwrap().location.y, 0f32);
}
