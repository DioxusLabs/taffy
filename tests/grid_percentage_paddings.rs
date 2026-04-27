use taffy::prelude::*;
use taffy::Direction;
use taffy_test_helpers::{new_test_tree, test_measure_function, TestNodeContext};

const TEXT_10X10: TestNodeContext = TestNodeContext::fixed(10.0, 10.0);

#[test]
fn grid_item_percentage_padding_top_uses_grid_area_width_in_rtl() {
    let mut taffy = new_test_tree();

    let padded_item = taffy
        .new_leaf_with_context(
            Style {
                direction: Direction::Rtl,
                padding: Rect { left: zero(), right: zero(), top: percent(0.5), bottom: zero() },
                ..Default::default()
            },
            TEXT_10X10,
        )
        .unwrap();

    let second_row_item = taffy
        .new_leaf(Style {
            direction: Direction::Rtl,
            size: Size { width: percent(1.0), height: length(10.0) },
            ..Default::default()
        })
        .unwrap();

    let grid = taffy
        .new_with_children(
            Style {
                display: Display::Grid,
                direction: Direction::Rtl,
                size: Size { width: length(500.0), height: auto() },
                justify_items: Some(AlignItems::Start),
                grid_template_columns: vec![length(100.0)],
                ..Default::default()
            },
            &[padded_item, second_row_item],
        )
        .unwrap();

    taffy.compute_layout_with_measure(grid, Size::MAX_CONTENT, test_measure_function).unwrap();

    let grid_layout = taffy.layout(grid).unwrap();
    assert_eq!(grid_layout.size.width, 500.0);
    assert_eq!(grid_layout.size.height, 70.0);

    let padded_item_layout = taffy.layout(padded_item).unwrap();
    assert_eq!(padded_item_layout.location.x, 490.0);
    assert_eq!(padded_item_layout.location.y, 0.0);
    assert_eq!(padded_item_layout.size.width, 10.0);
    assert_eq!(padded_item_layout.size.height, 60.0);

    let second_row_item_layout = taffy.layout(second_row_item).unwrap();
    assert_eq!(second_row_item_layout.location.x, 400.0);
    assert_eq!(second_row_item_layout.location.y, 60.0);
    assert_eq!(second_row_item_layout.size.width, 100.0);
    assert_eq!(second_row_item_layout.size.height, 10.0);
}
