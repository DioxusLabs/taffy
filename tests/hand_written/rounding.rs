use taffy::prelude::*;
use taffy_test_helpers::new_test_tree;

#[test]
fn rounding_doesnt_leave_gaps() {
    // First create an instance of TaffyTree
    let mut taffy = new_test_tree();

    let w_square = Size { width: length(100.3), height: length(100.3) };
    let child_a = taffy.new_leaf(Style { size: w_square, ..Default::default() }).unwrap();
    let child_b = taffy.new_leaf(Style { size: w_square, ..Default::default() }).unwrap();

    let root_node = taffy
        .new_with_children(
            Style {
                size: Size { width: length(963.3333), height: length(1000.) },
                justify_content: JustifyContent::CENTER,
                ..Default::default()
            },
            &[child_a, child_b],
        )
        .unwrap();

    taffy.compute_layout(root_node, Size::MAX_CONTENT).unwrap();
    taffy.print_tree(root_node);

    let layout_a = taffy.layout(child_a).unwrap();
    let layout_b = taffy.layout(child_b).unwrap();
    assert_eq!(layout_a.location.x + layout_a.size.width, layout_b.location.x);
}

/// Height of a `rule_height` box under a `top_height` one, in a column, after rounding.
fn rounded_rule_height(top_height: f32, rule_height: f32) -> f32 {
    let mut taffy = new_test_tree();
    let leaf_style =
        |height: f32| Style { size: Size { width: length(100.0), height: length(height) }, ..Default::default() };
    let top = taffy.new_leaf(leaf_style(top_height)).unwrap();
    let rule = taffy.new_leaf(leaf_style(rule_height)).unwrap();
    let root = taffy
        .new_with_children(Style { flex_direction: FlexDirection::Column, ..Default::default() }, &[top, rule])
        .unwrap();
    taffy.compute_layout(root, Size::MAX_CONTENT).unwrap();
    taffy.layout(rule).unwrap().size.height
}

#[test]
fn rounding_keeps_sub_pixel_boxes() {
    // Wherever it sits, a box with a real size keeps a pixel (as Chromium's pixel snapping does)
    // rather than rounding away to nothing.
    for top in [0.0, 0.25, 0.5, 0.6, 0.9] {
        assert_eq!(rounded_rule_height(top, 0.5), 1.0, "0.5 at {top}");
        assert_eq!(rounded_rule_height(top, 0.25), 1.0, "0.25 at {top}");
        assert_eq!(rounded_rule_height(top, 0.8), 1.0, "0.8 at {top}");
    }
    // Empty boxes and float noise still round to zero.
    assert_eq!(rounded_rule_height(0.6, 0.0), 0.0);
    assert_eq!(rounded_rule_height(0.6, 0.01), 0.0);
}
