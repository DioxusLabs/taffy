use taffy::prelude::*;
use taffy::{Direction, Point, TableLayout, Visibility};
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

/// Builds a table of `rows` rows of cells, each cell `cell_size` wide/tall, preceded by the
/// given column-group/column children
fn table_with_columns(
    tree: &mut TaffyTree<taffy_test_helpers::TestNodeContext>,
    table_style: Style,
    column_children: &[NodeId],
    rows: &[&[Style]],
) -> NodeId {
    let mut children: Vec<NodeId> = column_children.to_vec();
    for row_cells in rows {
        let cells: Vec<NodeId> = row_cells
            .iter()
            .map(|cell_style| {
                let content = tree
                    .new_leaf(Style { size: Size { width: length(20.0), height: length(10.0) }, ..Default::default() })
                    .unwrap();
                tree.new_with_children(Style { display: Display::TableCell, ..cell_style.clone() }, &[content]).unwrap()
            })
            .collect();
        children
            .push(tree.new_with_children(Style { display: Display::TableRow, ..Default::default() }, &cells).unwrap());
    }
    tree.new_with_children(Style { display: Display::Table, ..table_style }, &children).unwrap()
}

/// CSS 2.1 §17.5.2.2: a column box's `width` is a lower bound on its column in auto layout
#[test]
fn column_width_raises_auto_column() {
    let mut tree = new_test_tree();
    let col = tree
        .new_leaf(Style {
            display: Display::TableColumn,
            size: Size { width: length(50.0), height: auto() },
            ..Default::default()
        })
        .unwrap();
    let cell = Style::default();
    let table = table_with_columns(&mut tree, Style::default(), &[col], &[&[cell.clone(), cell]]);

    tree.compute_layout(table, Size::MAX_CONTENT).unwrap();

    assert_eq!(tree.layout(table).unwrap().size, Size { width: 70.0, height: 10.0 });
    let col_layout = tree.layout(col).unwrap();
    assert_eq!(col_layout.location, Point { x: 0.0, y: 0.0 });
    assert_eq!(col_layout.size, Size { width: 50.0, height: 10.0 });
}

/// A column box's `width` takes precedence over the first row's cells in a fixed table
#[test]
fn column_width_wins_in_fixed_layout() {
    let mut tree = new_test_tree();
    let col = tree
        .new_leaf(Style {
            display: Display::TableColumn,
            size: Size { width: length(30.0), height: auto() },
            ..Default::default()
        })
        .unwrap();
    let wide_cell = Style { size: Size { width: length(80.0), height: auto() }, ..Default::default() };
    let table = table_with_columns(
        &mut tree,
        Style {
            table_layout: TableLayout::Fixed,
            size: Size { width: length(100.0), height: auto() },
            ..Default::default()
        },
        &[col],
        &[&[wide_cell, Style::default()]],
    );
    let row = tree.child_at_index(table, 1).unwrap();
    let first_cell = tree.child_at_index(row, 0).unwrap();
    let second_cell = tree.child_at_index(row, 1).unwrap();

    tree.compute_layout(table, Size::MAX_CONTENT).unwrap();

    assert_eq!(tree.layout(first_cell).unwrap().size.width, 30.0);
    assert_eq!(tree.layout(second_cell).unwrap().size.width, 70.0);
}

/// A column spanning several columns (HTML `<col span>`) constrains each of them, and its box
/// covers all of them; a column group's box covers its columns
#[test]
fn column_span_and_group_geometry() {
    let mut tree = new_test_tree();
    let spanning_col = tree
        .new_leaf(Style {
            display: Display::TableColumn,
            colspan: 2,
            size: Size { width: length(40.0), height: auto() },
            ..Default::default()
        })
        .unwrap();
    let group = tree
        .new_with_children(Style { display: Display::TableColumnGroup, ..Default::default() }, &[spanning_col])
        .unwrap();
    let plain_col = tree.new_leaf(Style { display: Display::TableColumn, ..Default::default() }).unwrap();
    let cell = Style::default();
    let table = table_with_columns(
        &mut tree,
        Style { border_spacing: Size { width: length(5.0), height: length(5.0) }, ..Default::default() },
        &[group, plain_col],
        &[&[cell.clone(), cell.clone(), cell.clone()], &[cell.clone(), cell.clone(), cell]],
    );

    tree.compute_layout(table, Size::MAX_CONTENT).unwrap();

    // 5 + 40 + 5 + 40 + 5 + 20 + 5 wide; 5 + 10 + 5 + 10 + 5 tall
    assert_eq!(tree.layout(table).unwrap().size, Size { width: 120.0, height: 35.0 });
    let group_layout = tree.layout(group).unwrap();
    assert_eq!(group_layout.location, Point { x: 5.0, y: 5.0 });
    assert_eq!(group_layout.size, Size { width: 85.0, height: 25.0 });
    let spanning_layout = tree.layout(spanning_col).unwrap();
    assert_eq!(spanning_layout.location, Point { x: 0.0, y: 0.0 });
    assert_eq!(spanning_layout.size, Size { width: 85.0, height: 25.0 });
    let plain_layout = tree.layout(plain_col).unwrap();
    assert_eq!(plain_layout.location, Point { x: 95.0, y: 5.0 });
    assert_eq!(plain_layout.size, Size { width: 20.0, height: 25.0 });
}

/// Column boxes add grid columns even when no row has cells for them (css-tables-3 §2.3), so a
/// table with columns and no rows is as wide as its columns and as tall as its padding and border
#[test]
fn columns_without_rows_size_the_table() {
    let mut tree = new_test_tree();
    let col = tree
        .new_leaf(Style {
            display: Display::TableColumn,
            size: Size { width: length(60.0), height: auto() },
            ..Default::default()
        })
        .unwrap();
    let table = tree
        .new_with_children(
            Style {
                display: Display::Table,
                border_spacing: Size::zero(),
                border: Rect { left: length(2.0), right: length(2.0), top: length(3.0), bottom: length(3.0) },
                ..Default::default()
            },
            &[col],
        )
        .unwrap();

    tree.compute_layout(table, Size::MAX_CONTENT).unwrap();

    assert_eq!(tree.layout(table).unwrap().size, Size { width: 64.0, height: 6.0 });
    assert_eq!(tree.layout(col).unwrap().size, Size { width: 60.0, height: 0.0 });
}

/// CSS 2.1 §17.5.5: a collapsed column takes no room (nor does its spacing) and the table
/// narrows by what it took; the other columns keep their widths
#[test]
fn collapsed_column_narrows_the_table() {
    let mut tree = new_test_tree();
    let visible_col = tree.new_leaf(Style { display: Display::TableColumn, ..Default::default() }).unwrap();
    let collapsed_col = tree
        .new_leaf(Style { display: Display::TableColumn, visibility: Visibility::Collapse, ..Default::default() })
        .unwrap();
    let cell = Style::default();
    let table = table_with_columns(
        &mut tree,
        Style { border_spacing: Size { width: length(5.0), height: length(5.0) }, ..Default::default() },
        &[visible_col, collapsed_col],
        &[&[cell.clone(), cell.clone(), cell]],
    );
    let row = tree.child_at_index(table, 2).unwrap();
    let third_cell = tree.child_at_index(row, 2).unwrap();

    tree.compute_layout(table, Size::MAX_CONTENT).unwrap();

    // 5 + 20 + 5 + 20 + 5 wide: the middle column and one spacing are gone
    assert_eq!(tree.layout(table).unwrap().size, Size { width: 55.0, height: 20.0 });
    assert_eq!(tree.layout(collapsed_col).unwrap().size.width, 0.0);
    assert_eq!(tree.layout(third_cell).unwrap().location.x, 25.0);
}

/// A collapsed row (or row group) takes no room and the table shortens by what it took
#[test]
fn collapsed_row_shortens_the_table() {
    let mut tree = new_test_tree();
    let cell = Style::default();
    let table = table_with_columns(
        &mut tree,
        Style { border_spacing: Size { width: length(5.0), height: length(5.0) }, ..Default::default() },
        &[],
        &[&[cell.clone()], &[cell.clone()], &[cell]],
    );
    let collapsed_row = tree.child_at_index(table, 1).unwrap();
    tree.set_style(
        collapsed_row,
        Style { display: Display::TableRow, visibility: Visibility::Collapse, ..Default::default() },
    )
    .unwrap();
    let last_row = tree.child_at_index(table, 2).unwrap();

    tree.compute_layout(table, Size::MAX_CONTENT).unwrap();

    // 5 + 10 + 5 + 10 + 5 tall: the middle row and one spacing are gone
    assert_eq!(tree.layout(table).unwrap().size, Size { width: 30.0, height: 35.0 });
    assert_eq!(tree.layout(collapsed_row).unwrap().size.height, 0.0);
    assert_eq!(tree.layout(last_row).unwrap().location.y, 20.0);
}

/// A table with rows but no cells is as tall as its rows (and spacing) say; one with no rows at
/// all is just its padding and border
#[test]
fn table_without_cells_has_row_heights() {
    let mut tree = new_test_tree();
    let mut rows = Vec::new();
    for height in [20.0, 30.0] {
        let style = Style {
            display: Display::TableRow,
            size: Size { width: auto(), height: length(height) },
            ..Default::default()
        };
        rows.push(tree.new_leaf(style).unwrap());
    }
    let table = tree
        .new_with_children(
            Style {
                display: Display::Table,
                border: Rect::length(5.0),
                border_spacing: Size { width: length(2.0), height: length(2.0) },
                ..Default::default()
            },
            &rows,
        )
        .unwrap();
    tree.compute_layout(table, Size::MAX_CONTENT).unwrap();
    // 5 + 2 + 20 + 2 + 30 + 2 + 5
    assert_eq!(tree.layout(table).unwrap().size, Size { width: 10.0, height: 66.0 });
    assert_eq!(tree.layout(rows[1]).unwrap().location.y, 29.0);

    let empty = tree
        .new_with_children(
            Style {
                display: Display::Table,
                border: Rect::length(5.0),
                size: Size { width: length(0.0), height: length(0.0) },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    tree.compute_layout(empty, Size::MAX_CONTENT).unwrap();
    assert_eq!(tree.layout(empty).unwrap().size, Size { width: 10.0, height: 10.0 });
}

/// In a `direction: rtl` table the first column is the rightmost one, and a spanning cell
/// starts at the leftmost column it covers
#[test]
fn rtl_table_mirrors_its_columns() {
    let mut tree = new_test_tree();
    let narrow = Style { size: Size { width: length(20.0), height: length(10.0) }, ..Default::default() };
    let wide = Style { size: Size { width: length(50.0), height: length(10.0) }, ..Default::default() };
    let spanning = Style { colspan: 2, size: Size { width: auto(), height: length(10.0) }, ..Default::default() };
    let table = table_with_columns(
        &mut tree,
        Style {
            direction: Direction::Rtl,
            border_spacing: Size { width: length(5.0), height: length(0.0) },
            ..Default::default()
        },
        &[],
        &[&[narrow, wide], &[spanning]],
    );
    let first_row = tree.child_at_index(table, 0).unwrap();
    let first_cell = tree.child_at_index(first_row, 0).unwrap();
    let second_cell = tree.child_at_index(first_row, 1).unwrap();
    let second_row = tree.child_at_index(table, 1).unwrap();
    let spanning_cell = tree.child_at_index(second_row, 0).unwrap();

    tree.compute_layout(table, Size::MAX_CONTENT).unwrap();

    // 5 + 20 + 5 + 50 + 5 wide; the rows start after the outer spacing
    assert_eq!(tree.layout(table).unwrap().size.width, 85.0);
    assert_eq!(tree.layout(first_row).unwrap().location.x, 5.0);
    // The first (narrow) column is on the right, the second on the left
    assert_eq!(tree.layout(first_cell).unwrap().location.x, 55.0);
    assert_eq!(tree.layout(first_cell).unwrap().size.width, 20.0);
    assert_eq!(tree.layout(second_cell).unwrap().location.x, 0.0);
    assert_eq!(tree.layout(second_cell).unwrap().size.width, 50.0);
    assert_eq!(tree.layout(spanning_cell).unwrap().location.x, 0.0);
    assert_eq!(tree.layout(spanning_cell).unwrap().size.width, 75.0);
}
