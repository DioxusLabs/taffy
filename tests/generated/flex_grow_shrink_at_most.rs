#[test]
fn flex_grow_shrink_at_most() {
    #[allow(unused_imports)]
    use taffy::prelude::*;
    let mut taffy = taffy::Taffy::new();
    let node00 =
        taffy.new_leaf(taffy::style::Style { flex_grow: 1f32, flex_shrink: 1f32, ..Default::default() }).unwrap();
    let node0 = taffy.new_with_children(taffy::style::Style { ..Default::default() }, &[node00]).unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(100f32),
                    height: taffy::style::Dimension::Points(100f32),
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
    assert_eq!(taffy.layout(node).unwrap().size.width, 100f32);
    assert_eq!(taffy.layout(node).unwrap().size.height, 100f32);
    assert_eq!(taffy.layout(node).unwrap().location.x, 0f32);
    assert_eq!(taffy.layout(node).unwrap().location.y, 0f32);
    assert_eq!(taffy.layout(node0).unwrap().size.width, 0f32);
    assert_eq!(taffy.layout(node0).unwrap().size.height, 100f32);
    assert_eq!(taffy.layout(node0).unwrap().location.x, 0f32);
    assert_eq!(taffy.layout(node0).unwrap().location.y, 0f32);
    assert_eq!(taffy.layout(node00).unwrap().size.width, 0f32);
    assert_eq!(taffy.layout(node00).unwrap().size.height, 100f32);
    assert_eq!(taffy.layout(node00).unwrap().location.x, 0f32);
    assert_eq!(taffy.layout(node00).unwrap().location.y, 0f32);
}
