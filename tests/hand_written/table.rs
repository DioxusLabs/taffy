use taffy::prelude::*;
use taffy_test_helpers::new_test_tree;

/// css-tables-3 §6.1 and CSS 2.2 §17.6.1: a row and a row group have no padding, and their
/// border is ignored in the separate borders model
#[test]
fn row_and_row_group_have_no_padding_or_border() {
    let mut tree = new_test_tree();
    let styled = Style {
        padding: Rect { left: length(7.0), right: length(7.0), top: length(7.0), bottom: length(7.0) },
        border: Rect { left: length(5.0), right: length(5.0), top: length(5.0), bottom: length(5.0) },
        ..Default::default()
    };

    let cell_content = tree
        .new_leaf(Style { size: Size { width: length(40.0), height: length(20.0) }, ..Default::default() })
        .unwrap();
    let cell =
        tree.new_with_children(Style { display: Display::TableCell, ..Default::default() }, &[cell_content]).unwrap();
    let row = tree.new_with_children(Style { display: Display::TableRow, ..styled.clone() }, &[cell]).unwrap();
    let group = tree.new_with_children(Style { display: Display::TableRowGroup, ..styled }, &[row]).unwrap();
    let table = tree
        .new_with_children(
            Style { display: Display::Table, border_spacing: Size::zero(), ..Default::default() },
            &[group],
        )
        .unwrap();

    tree.compute_layout(table, Size::MAX_CONTENT).unwrap();

    for node in [row, group] {
        let layout = tree.layout(node).unwrap();
        assert_eq!(layout.padding, Rect::ZERO);
        assert_eq!(layout.border, Rect::ZERO);
    }

    let table_layout = tree.layout(table).unwrap();
    assert_eq!(table_layout.size, Size { width: 40.0, height: 20.0 });
}
