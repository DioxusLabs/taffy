use taffy::prelude::*;

#[test]
fn rounding_doesnt_leave_gaps() {
    // First create an instance of Taffy
    let mut taffy = Taffy::new();

    let w_square = Size { width: points(100.3), height: points(100.3) };
    let child_a = taffy.new_leaf(Style { size: w_square, ..Default::default() }).unwrap();
    let child_b = taffy.new_leaf(Style { size: w_square, ..Default::default() }).unwrap();

    let root_node = taffy
        .new_with_children(
            Style {
                size: Size { width: points(963.3333), height: points(1000.) },
                justify_content: Some(JustifyContent::Center),
                ..Default::default()
            },
            &[child_a, child_b],
        )
        .unwrap();

    taffy.compute_layout(root_node, Size::MAX_CONTENT).unwrap();

    let layout_a = taffy.layout(child_a).unwrap();
    let layout_b = taffy.layout(child_b).unwrap();
    taffy::debug::print_tree(&taffy, root_node);
    assert_eq!(layout_a.location.x + layout_a.size.width, layout_b.location.x);
}
