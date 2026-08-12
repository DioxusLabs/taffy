//! Computes CSS Table layout (CSS 2.1 §17 / [css-tables-3](https://www.w3.org/TR/css-tables-3/))
//!
//! Implements the automatic and fixed table layout algorithms for well-formed table trees
//! (table → optional row group → row → cell). Anonymous box fixup
//! (<https://www.w3.org/TR/css-tables-3/#fixup>) is the responsibility of the code
//! constructing the tree. Border collapsing is likewise expected to be resolved by the
//! embedder: with `border-collapse: collapse`, resolve the winning border for each edge,
//! write half of it into each cell's border style, and set `border_spacing` to zero.
#[cfg(feature = "content_size")]
use crate::compute::common::content_size::compute_content_size_contribution;
use crate::geometry::{AbsoluteAxis, Line, Point, Rect, Size};
use crate::style::{
    AvailableSpace, BlockContainerStyle, CompactLength, CoreStyle, Direction, TableContainerStyle, TableItemStyle,
    TableLayout, TableRole,
};
use crate::tree::traits::{LayoutPartialTreeExt, LayoutTableContainer};
use crate::tree::{Layout, LayoutInput, LayoutOutput, NodeId, RequestedAxis, RunMode, SizingMode};
use crate::util::sys::Vec;
use crate::util::{MaybeMath, MaybeResolve, ResolveOrZero};
use crate::BoxSizing;

/// A cell placed into the table grid
struct Cell {
    /// The node id of the cell
    node_id: NodeId,
    /// Index of the first row this cell occupies
    row: usize,
    /// Index of the first column this cell occupies
    col: usize,
    /// Number of columns spanned
    colspan: usize,
    /// Number of rows spanned
    rowspan: usize,
    /// The cell's index among its row's children
    order: u32,
}

/// One cell's inline constraints, as measured for column sizing
struct CellMeasure {
    /// The cell's min-content inline size (border-box)
    min: f32,
    /// The cell's max-content inline size (border-box)
    max: f32,
    /// The column-width percentage specified on the cell, if any
    percent: Option<f32>,
    /// Whether the cell has a fixed (length) specified width
    is_constrained: bool,
}

/// One cell's resolved geometry, carried from row sizing through to positioning
#[derive(Clone, Copy, Default)]
struct CellSizing {
    /// The width of every column the cell spans, plus the spacing between them
    width: f32,
    /// The height the cell asks for at that width
    height: f32,
    /// The cell's own baseline, when it takes part in row baseline alignment
    baseline: Option<f32>,
    /// How far baseline alignment pushes the cell's content down
    shift: f32,
}

/// Accumulated sizing constraints for one column of the table grid
#[derive(Clone, Copy, Default)]
struct Column {
    /// Largest min-content contribution of any cell in this column
    min: f32,
    /// Largest max-content contribution of any cell in this column
    max: f32,
    /// Largest percentage specified on any cell in this column
    percent: Option<f32>,
    /// Whether any cell in this column has a fixed (length) specified width
    is_constrained: bool,
    /// The resolved used width
    used: f32,
}

/// The table's cells, placed into rows and columns
#[derive(Default)]
struct Grid {
    /// The rows of every row group, in document order
    rows: Vec<Row>,
    /// Every cell, in the order its row placed it
    cells: Vec<Cell>,
    /// The table's row groups, in document order
    groups: Vec<Group>,
    /// The number of columns the placed cells add up to
    col_count: usize,
}

/// A row group of the table
struct Group {
    /// The node id of the row group
    node_id: NodeId,
    /// The group's index among the table's children
    order: u32,
}

/// A row of the table grid
struct Row {
    /// The node id of the row
    node_id: NodeId,
    /// Index of the row group this row belongs to, if any
    group: Option<usize>,
    /// The row's index among its parent's children
    order: u32,
    /// The resolved used height
    used_height: f32,
    /// The largest percentage height specified on the row or one of its cells
    percent: Option<f32>,
    /// Whether the row or one of its cells specifies a height
    is_constrained: bool,
    /// Whether a rowspanning cell starts in this row
    has_rowspan_start: bool,
    /// The row's baseline, once its cells have settled it
    baseline: Option<f32>,
    /// How far above the row's bottom edge to synthesise a baseline for a row with no cell
    /// that has one of its own
    fallback_descent: Option<f32>,
}

/// Computes the layout of [`LayoutTableContainer`] according to the CSS table layout algorithm
pub fn compute_table_layout(
    tree: &mut impl LayoutTableContainer,
    node_id: NodeId,
    inputs: LayoutInput,
) -> LayoutOutput {
    let LayoutInput { known_dimensions, parent_size, available_space, run_mode, axis, .. } = inputs;

    let style = tree.get_table_container_style(node_id);
    let raw_padding = style.padding();
    let raw_border = style.border();
    let raw_margin = style.margin();
    let raw_size = style.size();
    let raw_min_size = style.min_size();
    let raw_max_size = style.max_size();
    let box_sizing = style.box_sizing();
    let aspect_ratio = style.aspect_ratio();
    let border_spacing = style.border_spacing();
    let table_layout_mode = style.table_layout();
    drop(style);

    let parent_width = parent_size.width;
    let padding = raw_padding.resolve_or_zero(parent_width, |v, b| tree.calc(v, b));
    let border = raw_border.resolve_or_zero(parent_width, |v, b| tree.calc(v, b));
    let margin = raw_margin.resolve_or_zero(parent_width, |v, b| tree.calc(v, b));
    let padding_border = padding + border;
    let pb_size = padding_border.sum_axes();

    let box_sizing_adjustment = if box_sizing == BoxSizing::ContentBox { pb_size } else { Size::ZERO };
    let min_size = raw_min_size
        .maybe_resolve(parent_size, |v, b| tree.calc(v, b))
        .maybe_apply_aspect_ratio(aspect_ratio)
        .maybe_add(box_sizing_adjustment);
    let max_size = raw_max_size
        .maybe_resolve(parent_size, |v, b| tree.calc(v, b))
        .maybe_apply_aspect_ratio(aspect_ratio)
        .maybe_add(box_sizing_adjustment);
    let specified_size = raw_size
        .maybe_resolve(parent_size, |v, b| tree.calc(v, b))
        .maybe_apply_aspect_ratio(aspect_ratio)
        .maybe_add(box_sizing_adjustment)
        .maybe_clamp(min_size, max_size);

    let styled_known = known_dimensions.or(specified_size);

    let h_spacing = border_spacing.width.resolve_or_zero(parent_width, |v, b| tree.calc(v, b));
    let v_spacing = border_spacing.height.resolve_or_zero(parent_width, |v, b| tree.calc(v, b));

    // Phase 1: place cells into the grid
    let table_children: Vec<NodeId> = (0..tree.child_count(node_id)).map(|i| tree.get_child_id(node_id, i)).collect();
    let Grid { mut rows, cells, groups, col_count } = build_grid(tree, &table_children);

    if col_count == 0 || rows.is_empty() {
        let size = Size {
            width: styled_known.width.unwrap_or(pb_size.width).maybe_clamp(min_size.width, max_size.width),
            height: styled_known.height.unwrap_or(pb_size.height).maybe_clamp(min_size.height, max_size.height),
        };
        if run_mode == RunMode::PerformLayout {
            for (order, child_id) in table_children.into_iter().enumerate() {
                tree.set_unrounded_layout(child_id, &Layout::with_order(order as u32));
            }
        }
        return LayoutOutput::from_outer_size(size);
    }

    // Phase 2: collect intrinsic size contributions per column
    //
    // Fixed layout reads the column widths off the first row alone and never looks at content
    let is_fixed = table_layout_mode == TableLayout::Fixed && styled_known.width.is_some();
    let mut columns = measure_columns(tree, &cells, col_count, h_spacing, is_fixed);

    // Phase 3: resolve the table's used width and the columns' used widths
    let total_h_spacing = h_spacing * (col_count as f32 + 1.0);
    let (grid_min_sum, grid_max_sum) = grid_min_max(&columns, is_fixed);
    let grid_min = grid_min_sum + total_h_spacing;
    let grid_max = grid_max_sum + total_h_spacing;

    let table_width = match styled_known.width {
        Some(w) => w.max(grid_min + pb_size.width),
        None => {
            let shrink_to_fit = match available_space.width {
                AvailableSpace::Definite(w) => (w - margin.horizontal_axis_sum() - pb_size.width)
                    .maybe_clamp(max_size.width.map(|m| m - pb_size.width), None)
                    .clamp(grid_min, grid_max.max(grid_min)),
                AvailableSpace::MaxContent => grid_max,
                AvailableSpace::MinContent => grid_min,
            };
            (shrink_to_fit + pb_size.width).maybe_clamp(min_size.width, max_size.width).max(grid_min + pb_size.width)
        }
    };
    let assignable = table_width - pb_size.width - total_h_spacing;

    if is_fixed {
        resolve_fixed_layout_columns(&mut columns, assignable);
    } else {
        resolve_auto_layout_columns(&mut columns, assignable);
    }

    if run_mode == RunMode::ComputeSize && axis == RequestedAxis::Horizontal {
        return LayoutOutput::from_outer_size(Size { width: table_width, height: styled_known.height.unwrap_or(0.0) });
    }

    // Phase 4: resolve row heights by sizing each cell at its final width
    let num_rows = rows.len();
    let mut sizing: Vec<CellSizing> = cells
        .iter()
        .map(|cell| {
            let range = cell.col..(cell.col + cell.colspan).min(col_count);
            let width =
                columns[range.clone()].iter().map(|c| c.used).sum::<f32>() + h_spacing * (range.len() as f32 - 1.0);
            CellSizing { width, ..CellSizing::default() }
        })
        .collect();

    // Row baseline alignment per CSS 2.1 §17.5.4. An empty cell has no baseline of its own, so
    // the row falls back to sitting its baseline on the shallowest such cell's content edge
    for (cell, sizing) in cells.iter().zip(sizing.iter_mut()) {
        let output = tree.compute_child_layout(
            cell.node_id,
            LayoutInput {
                known_dimensions: Size { width: Some(sizing.width), height: None },
                parent_size: Size { width: Some(table_width - pb_size.width), height: None },
                available_space: Size {
                    width: AvailableSpace::Definite(sizing.width),
                    height: AvailableSpace::MaxContent,
                },
                sizing_mode: SizingMode::InherentSize,
                axis: RequestedAxis::Both,
                run_mode: RunMode::ComputeSize,
                vertical_margins_are_collapsible: Line::FALSE,
                content_offset_y: 0.0,
            },
        );
        sizing.height = output.size.height;

        let cell_style = tree.get_table_child_style(cell.node_id);
        let participates = tree.get_block_container_style(cell.node_id).align_content().is_none();
        let height_dim = cell_style.size().height;
        drop(cell_style);

        let row = &mut rows[cell.row];
        if cell.rowspan > 1 {
            row.has_rowspan_start = true;
        } else {
            if height_dim.tag() == CompactLength::PERCENT_TAG {
                row.percent = Some(row.percent.map_or(height_dim.value(), |p: f32| p.max(height_dim.value())));
                row.is_constrained = true;
            } else if height_dim.tag() == CompactLength::LENGTH_TAG {
                row.is_constrained = true;
            }
            row.used_height = row.used_height.max(output.size.height);
        }

        if participates {
            let cell_style = tree.get_table_child_style(cell.node_id);
            let pb_bottom = cell_style.padding().resolve_or_zero(parent_width, |v, b| tree.calc(v, b)).bottom
                + cell_style.border().resolve_or_zero(parent_width, |v, b| tree.calc(v, b)).bottom;
            drop(cell_style);

            let row = &mut rows[cell.row];
            if tree.child_count(cell.node_id) == 0 {
                row.fallback_descent = Some(row.fallback_descent.map_or(pb_bottom, |d: f32| d.min(pb_bottom)));
            } else {
                let baseline = output.first_baselines.y.unwrap_or(output.size.height - pb_bottom);
                sizing.baseline = Some(baseline);
                row.baseline = Some(row.baseline.map_or(baseline, |b: f32| b.max(baseline)));
            }
        }
    }

    // A rowspanning cell lends its ascent to the row it starts in but not its descent, which
    // the rows it spans absorb instead
    for (cell, sizing) in cells.iter().zip(sizing.iter_mut()) {
        if let (Some(baseline), Some(row_baseline)) = (sizing.baseline, rows[cell.row].baseline) {
            sizing.shift = row_baseline - baseline;
            if cell.rowspan == 1 {
                let row = &mut rows[cell.row];
                row.used_height = row.used_height.max(sizing.shift + sizing.height);
            }
        }
    }

    for row in rows.iter_mut() {
        let height_dim = tree.get_table_child_style(row.node_id).size().height;
        if height_dim.tag() == CompactLength::PERCENT_TAG {
            row.percent = Some(row.percent.map_or(height_dim.value(), |p: f32| p.max(height_dim.value())));
            row.is_constrained = true;
        } else if height_dim.tag() == CompactLength::LENGTH_TAG {
            row.used_height = row.used_height.max(height_dim.value());
            row.is_constrained = true;
        }
    }

    let total_v_spacing = v_spacing * (num_rows as f32 + 1.0);
    // Row percentages resolve against the space a definite-height table leaves for its rows
    let percentage_basis = styled_known.height.map(|h| (h - pb_size.height - total_v_spacing).max(0.0));

    for (cell, sizing) in cells.iter().zip(sizing.iter()).filter(|(cell, _)| cell.rowspan > 1) {
        let range = cell.row..(cell.row + cell.rowspan).min(num_rows);
        let target = sizing.shift + sizing.height - v_spacing * (range.len() as f32 - 1.0);
        distribute_excess_height(&mut rows[range], target, true, percentage_basis);
    }

    for (group_index, group) in groups.iter().enumerate() {
        let group_height = tree
            .get_table_child_style(group.node_id)
            .size()
            .height
            .maybe_resolve(percentage_basis, |v, b| tree.calc(v, b));

        if let Some(height) = group_height {
            let range = group_row_range(&rows, group_index);
            let target = height - v_spacing * (range.len() as f32 - 1.0);
            distribute_excess_height(&mut rows[range], target, false, percentage_basis);
        }
    }

    let content_height = rows.iter().map(|r| r.used_height).sum::<f32>() + total_v_spacing;
    let table_height = styled_known
        .height
        .unwrap_or((content_height + pb_size.height).maybe_clamp(min_size.height, max_size.height))
        .max(content_height + pb_size.height)
        .maybe_max(min_size.height);

    distribute_excess_height(&mut rows, table_height - pb_size.height - total_v_spacing, false, percentage_basis);

    for row in rows.iter_mut().filter(|r| r.baseline.is_none()) {
        row.baseline = row.fallback_descent.map(|descent| (row.used_height - descent).max(0.0));
    }

    let final_size = Size { width: table_width, height: table_height };
    // The table's baseline is its first row's baseline (css-tables-3 §4.2)
    let table_baseline =
        rows.first().and_then(|row| row.baseline).map(|baseline| padding_border.top + v_spacing + baseline);
    let baselines = Point { x: None, y: table_baseline };

    if run_mode == RunMode::ComputeSize {
        return LayoutOutput::from_sizes_and_baselines(final_size, Size::zero(), baselines);
    }

    // Phase 5: position rows, row groups, and cells (in RTL the first column is rightmost)
    let rtl = tree.get_table_container_style(node_id).direction() == Direction::Rtl;
    let mut col_x: Vec<f32> = Vec::with_capacity(col_count);
    let mut x = padding_border.left + h_spacing;
    for col in columns.iter() {
        col_x.push(x);
        x += col.used + h_spacing;
    }
    if rtl {
        for (i, col) in columns.iter().enumerate() {
            col_x[i] = table_width - pb_size.width + padding_border.left - (col_x[i] - padding_border.left) - col.used;
        }
    }
    let mut row_y: Vec<f32> = Vec::with_capacity(num_rows);
    let mut y = padding_border.top + v_spacing;
    for row in rows.iter() {
        row_y.push(y);
        y += row.used_height + v_spacing;
    }

    // Row and row-group boxes exclude the outer border-spacing, which belongs to the table
    let grid_origin_x = padding_border.left + h_spacing;
    let grid_width = assignable + h_spacing * (col_count as f32 - 1.0);

    let mut group_origin: Vec<Point<f32>> = Vec::with_capacity(groups.len());
    for (group_index, group) in groups.iter().enumerate() {
        let range = group_row_range(&rows, group_index);
        let (start_y, end_y) = match range.end.checked_sub(1) {
            Some(last) if !range.is_empty() => (row_y[range.start], row_y[last] + rows[last].used_height),
            _ => (padding_border.top + v_spacing, padding_border.top + v_spacing),
        };
        let origin = Point { x: grid_origin_x, y: start_y };
        group_origin.push(origin);

        set_item_layout(
            tree,
            group.node_id,
            group.order,
            origin,
            Size { width: grid_width, height: end_y - start_y },
            parent_width,
        );
    }

    for (row_index, row) in rows.iter().enumerate() {
        let origin = match row.group {
            Some(g) => Point { x: 0.0, y: row_y[row_index] - group_origin[g].y },
            None => Point { x: grid_origin_x, y: row_y[row_index] },
        };
        set_item_layout(
            tree,
            row.node_id,
            row.order,
            origin,
            Size { width: grid_width, height: row.used_height },
            parent_width,
        );
    }

    let mut content_size = Size::ZERO;
    for (cell, sizing) in cells.iter().zip(sizing.iter()) {
        let cell_width = sizing.width;
        let range = cell.row..(cell.row + cell.rowspan).min(num_rows);
        let span_len = range.len();
        let cell_height: f32 =
            rows[range].iter().map(|r| r.used_height).sum::<f32>() + v_spacing * (span_len as f32 - 1.0);

        let output = tree.compute_child_layout(
            cell.node_id,
            LayoutInput {
                known_dimensions: Size { width: Some(cell_width), height: Some(cell_height) },
                parent_size: Size { width: Some(grid_width), height: Some(table_height - pb_size.height) },
                available_space: Size {
                    width: AvailableSpace::Definite(cell_width),
                    height: AvailableSpace::Definite(cell_height),
                },
                sizing_mode: SizingMode::InherentSize,
                axis: RequestedAxis::Both,
                run_mode: RunMode::PerformLayout,
                vertical_margins_are_collapsible: Line::FALSE,
                content_offset_y: sizing.shift,
            },
        );

        let cell_style = tree.get_table_child_style(cell.node_id);
        let cell_padding = cell_style.padding().resolve_or_zero(parent_width, |v, b| tree.calc(v, b));
        let cell_border = cell_style.border().resolve_or_zero(parent_width, |v, b| tree.calc(v, b));
        #[cfg(feature = "content_size")]
        let cell_overflow = cell_style.overflow();
        drop(cell_style);

        // A spanning cell starts at its leftmost spanned column, which in RTL is the last one
        let col_range = cell.col..(cell.col + cell.colspan).min(col_count);
        let cell_x = col_x[col_range].iter().copied().fold(f32::INFINITY, f32::min);
        let location = Point { x: cell_x - grid_origin_x, y: 0.0 };

        tree.set_unrounded_layout(
            cell.node_id,
            &Layout {
                order: cell.order,
                location,
                size: Size { width: cell_width, height: cell_height },
                #[cfg(feature = "content_size")]
                content_size: output.content_size,
                scrollbar_size: Size::ZERO,
                padding: cell_padding,
                border: cell_border,
                margin: Rect::ZERO,
            },
        );

        #[cfg(feature = "content_size")]
        {
            content_size = content_size.f32_max(compute_content_size_contribution(
                Point { x: cell_x - border.left, y: row_y[cell.row] - border.top },
                Size { width: cell_width, height: cell_height },
                output.content_size,
                cell_overflow,
            ));
        }
    }

    LayoutOutput::from_sizes_and_baselines(final_size, content_size, baselines)
}

/// The rows a row group holds, which `build_grid` places contiguously
fn group_row_range(rows: &[Row], group: usize) -> core::ops::Range<usize> {
    let start = rows.iter().position(|r| r.group == Some(group)).unwrap_or(0);
    let len = rows[start..].iter().take_while(|r| r.group == Some(group)).count();

    start..start + len
}

/// Place every cell of the table into the grid, and collect its row groups. Rows outside a row
/// group belong to an anonymous one, and a rowspan never leaves the group it starts in
/// (css-tables-3 §2.2)
fn build_grid(tree: &mut impl LayoutTableContainer, table_children: &[NodeId]) -> Grid {
    let mut grid = Grid::default();
    // How many further rows the cell placed in each column still covers
    let mut occupancy: Vec<usize> = Vec::new();
    let mut index = 0;

    while index < table_children.len() {
        occupancy.clear();
        let group_id = table_children[index];

        if tree.get_table_child_style(group_id).table_role() == TableRole::RowGroup {
            let group = Some(grid.groups.len());
            grid.groups.push(Group { node_id: group_id, order: index as u32 });

            let row_count = tree.child_count(group_id);
            for row in 0..row_count {
                let row_id = tree.get_child_id(group_id, row);
                place_row(tree, row_id, group, row as u32, row_count - row, &mut occupancy, &mut grid);
            }
            index += 1;
        } else {
            // Rows outside a row group belong to one anonymous group, which the run of
            // consecutive non-group children marks out
            let mut end = index + 1;
            while end < table_children.len()
                && tree.get_table_child_style(table_children[end]).table_role() != TableRole::RowGroup
            {
                end += 1;
            }
            for (offset, &row_id) in table_children[index..end].iter().enumerate() {
                let row = index + offset;
                place_row(tree, row_id, None, row as u32, end - row, &mut occupancy, &mut grid);
            }
            index = end;
        }
    }

    grid
}

/// Place one row's cells into the grid, skipping columns still covered by
/// rowspanning cells from earlier rows. `rows_left` counts this row and the ones after it in
/// the same row group, which is as far as a rowspan can reach.
fn place_row(
    tree: &mut impl LayoutTableContainer,
    row_id: NodeId,
    group: Option<usize>,
    order: u32,
    rows_left: usize,
    occupancy: &mut Vec<usize>,
    grid: &mut Grid,
) {
    let row_index = grid.rows.len();
    grid.rows.push(Row {
        node_id: row_id,
        group,
        order,
        used_height: 0.0,
        percent: None,
        is_constrained: false,
        has_rowspan_start: false,
        baseline: None,
        fallback_descent: None,
    });

    let cell_count = tree.child_count(row_id);
    let mut col = 0;
    for cell in 0..cell_count {
        let cell_id = tree.get_child_id(row_id, cell);
        let cell_style = tree.get_table_child_style(cell_id);
        let colspan = (cell_style.colspan() as usize).max(1);
        let rowspan = (cell_style.rowspan() as usize).clamp(1, rows_left);
        drop(cell_style);

        while occupancy.get(col).is_some_and(|&o| o > 0) {
            col += 1;
        }
        if occupancy.len() < col + colspan {
            occupancy.resize(col + colspan, 0);
        }
        // Covers this row plus rowspan - 1 more; the end-of-row decrement consumes this row.
        // A colspan may overlap a slot an earlier rowspan still holds, which HTML calls a table
        // model error and lets stand, so the longer span keeps the slot.
        for slot in occupancy[col..col + colspan].iter_mut() {
            *slot = (*slot).max(rowspan);
        }

        grid.cells.push(Cell { node_id: cell_id, row: row_index, col, colspan, rowspan, order: cell as u32 });
        col += colspan;
    }

    grid.col_count = grid.col_count.max(occupancy.len().max(col));
    for slot in occupancy.iter_mut() {
        *slot = slot.saturating_sub(1);
    }
}

/// Grow `rows` until their heights sum to `target`. css-tables-3 leaves this undefined
/// (<https://github.com/w3c/csswg-drafts/issues/4418>), so this follows Blink's priority order:
/// percentage rows first, then the rows a rowspan starts in, then unconstrained rows, then empty
/// rows, and finally every non-empty row in proportion to its height.
fn distribute_excess_height(
    rows: &mut [Row],
    target: f32,
    is_rowspan_distribution: bool,
    percentage_basis: Option<f32>,
) {
    let deficit = |row: &Row| match (row.percent, percentage_basis) {
        (Some(percent), Some(basis)) => (percent * basis - row.used_height).max(0.0),
        _ => 0.0,
    };

    let mut total: f32 = rows.iter().map(|r| r.used_height).sum();
    let mut extra = target - total;
    if rows.is_empty() || extra <= 0.0 {
        return;
    }

    let percent_deficit: f32 = rows.iter().map(deficit).sum();
    if percent_deficit > 0.0 {
        let distributable = percent_deficit.min(extra);
        for row in rows.iter_mut() {
            let delta = distributable * deficit(row) / percent_deficit;
            row.used_height += delta;
            total += delta;
            extra -= delta;
        }
        if extra <= 0.0 {
            return;
        }
    }

    // A rowspan's excess goes to the rows where the spans it crosses begin, not to its own row
    if is_rowspan_distribution {
        let originating = rows[1..].iter().filter(|r| r.has_rowspan_start).count();
        if originating > 0 {
            for row in rows[1..].iter_mut().filter(|r| r.has_rowspan_start) {
                row.used_height += extra / originating as f32;
            }
            return;
        }
    }

    let unconstrained: f32 =
        rows.iter().filter(|r| !r.is_constrained && r.used_height > 0.0).map(|r| r.used_height).sum();
    if unconstrained > 0.0 {
        for row in rows.iter_mut().filter(|r| !r.is_constrained && r.used_height > 0.0) {
            row.used_height += extra * row.used_height / unconstrained;
        }
        return;
    }

    let empty_count = rows.iter().filter(|r| r.used_height == 0.0).count();
    let constrained_non_empty = rows.iter().filter(|r| r.used_height > 0.0 && r.is_constrained).count();

    if is_rowspan_distribution {
        // Nothing to grow proportionally, so the last row absorbs the lot
        if empty_count == rows.len() {
            rows[rows.len() - 1].used_height += extra;
            return;
        }
    } else if empty_count > 0 && empty_count + constrained_non_empty == rows.len() {
        let unconstrained_empty = rows.iter().filter(|r| r.used_height == 0.0 && !r.is_constrained).count();
        let grow_unconstrained_only = unconstrained_empty > 0;
        let count = if grow_unconstrained_only { unconstrained_empty } else { empty_count };

        for row in rows.iter_mut().filter(|r| r.used_height == 0.0) {
            if !grow_unconstrained_only || !row.is_constrained {
                row.used_height = extra / count as f32;
            }
        }
        return;
    }

    if total > 0.0 {
        for row in rows.iter_mut().filter(|r| r.used_height > 0.0) {
            row.used_height += extra * row.used_height / total;
        }
    }
}

/// Distribute `extra` over `field` across `columns`, proportionally to what each column already
/// has, or equally when they all have nothing
fn distribute(columns: &mut [Column], extra: f32, field: impl Fn(&mut Column) -> &mut f32) {
    let total: f32 = columns.iter_mut().map(|col| *field(col)).sum();
    let count = columns.len() as f32;
    for col in columns.iter_mut() {
        let share = if total > 0.0 { extra * *field(col) / total } else { extra / count };
        *field(col) += share;
    }
}

/// Measure every cell and fold its inline constraints into the column it occupies, then settle
/// the columns' percentages against each other
fn measure_columns(
    tree: &mut impl LayoutTableContainer,
    cells: &[Cell],
    col_count: usize,
    h_spacing: f32,
    is_fixed: bool,
) -> Vec<Column> {
    let mut columns: Vec<Column> = core::iter::repeat_with(Column::default).take(col_count).collect();
    let mut spans: Vec<(usize, usize, CellMeasure)> = Vec::new();

    // Fixed layout takes its column widths from the first row alone
    for cell in cells.iter().filter(|c| !is_fixed || c.row == 0) {
        let cell_style = tree.get_table_child_style(cell.node_id);
        let width_dim = cell_style.size().width;
        let width_tag = width_dim.tag();
        drop(cell_style);

        let mut measure = CellMeasure {
            min: 0.0,
            max: 0.0,
            percent: (width_tag == CompactLength::PERCENT_TAG).then(|| width_dim.value()),
            is_constrained: width_tag == CompactLength::LENGTH_TAG,
        };

        if !is_fixed {
            // Specified widths count toward max-content only; columns may shrink below them
            measure.min = tree.measure_child_size(
                cell.node_id,
                Size::NONE,
                Size::NONE,
                Size { width: AvailableSpace::MinContent, height: AvailableSpace::MinContent },
                SizingMode::ContentSize,
                AbsoluteAxis::Horizontal,
                Line::FALSE,
            );
        }
        // A fixed table never looks at content, so only a specified width is worth measuring
        if !is_fixed || measure.is_constrained || cell.colspan > 1 {
            measure.max = tree.measure_child_size(
                cell.node_id,
                Size::NONE,
                Size::NONE,
                Size { width: AvailableSpace::MaxContent, height: AvailableSpace::MaxContent },
                SizingMode::InherentSize,
                AbsoluteAxis::Horizontal,
                Line::FALSE,
            );
        }

        if cell.colspan > 1 {
            spans.push((cell.col, cell.colspan, measure));
        } else if !(is_fixed && columns[cell.col].is_constrained) {
            // A fixed table's column keeps the first width it is given
            encompass(&mut columns[cell.col], &measure);
        }
    }

    for (col, colspan, measure) in spans.iter() {
        let range = *col..(col + colspan).min(col_count);
        let spanned_spacing = h_spacing * (range.len() as f32 - 1.0);

        // Whatever a spanning cell's percentage asks for beyond the percentages already on the
        // columns is shared out over the remaining ones, weighted by max-content width
        if let Some(percent) = measure.percent {
            let spanned = &columns[range.clone()];
            let surplus = percent - spanned.iter().filter_map(|c| c.percent).sum::<f32>();
            let auto_count = spanned.iter().filter(|c| c.percent.is_none()).count();
            let auto_max: f32 = spanned.iter().filter(|c| c.percent.is_none()).map(|c| c.max).sum();

            if surplus > 0.0 && auto_count > 0 {
                for col in columns[range.clone()].iter_mut().filter(|c| c.percent.is_none()) {
                    let share = if auto_max > 0.0 { surplus * col.max / auto_max } else { surplus / auto_count as f32 };
                    col.percent = Some(share);
                }
            }
        }

        let current_min: f32 = columns[range.clone()].iter().map(|c| c.min).sum::<f32>() + spanned_spacing;
        if measure.min > current_min {
            distribute(&mut columns[range.clone()], measure.min - current_min, |c| &mut c.min);
        }
        let current_max: f32 = columns[range.clone()].iter().map(|c| c.max).sum::<f32>() + spanned_spacing;
        if measure.max > current_max {
            distribute(&mut columns[range.clone()], measure.max - current_max, |c| &mut c.max);
        }
    }

    for col in columns.iter_mut() {
        col.max = col.max.max(col.min);
    }

    // Percentages cannot claim more than the whole table between them. An auto table caps each
    // column by what the columns before it left of 100%; a fixed one scales them all down
    if is_fixed {
        let total: f32 = columns.iter().filter_map(|c| c.percent).sum();
        if total > 1.0 {
            for col in columns.iter_mut() {
                col.percent = col.percent.map(|percent| percent / total);
            }
        }
    } else {
        let mut budget = 1.0f32;
        for col in columns.iter_mut().filter(|c| c.percent.is_some()) {
            let capped = col.percent.unwrap_or(0.0).min(budget).max(0.0);
            budget -= capped;
            col.percent = Some(capped);
        }
    }

    columns
}

/// The width a percentage column asks for, which never drops below the column's minimum
fn percent_width(column: &Column, target: f32) -> f32 {
    column.min.max(column.percent.unwrap_or(0.0) * target)
}

/// Merge one cell's inline constraints into its column. A column pinned by a specified width only
/// grows to an unconstrained cell's *minimum*, so a wrappable cell wraps instead of widening it
fn encompass(column: &mut Column, cell: &CellMeasure) {
    column.min = column.min.max(cell.min);
    column.max = if column.is_constrained == cell.is_constrained {
        column.max.max(cell.max)
    } else if column.is_constrained {
        column.max.max(cell.min)
    } else {
        column.min.max(cell.max)
    };
    column.is_constrained |= cell.is_constrained;

    if let Some(percent) = cell.percent {
        column.percent = Some(column.percent.map_or(percent, |p: f32| p.max(percent)));
    }
}

/// Sum the columns into the table's min-content and max-content grid widths
/// (<https://www.w3.org/TR/css-tables-3/#computing-the-table-width>). Percentages have nothing to
/// resolve against yet, so they act as constraints on the total instead: a column at `p` percent
/// whose content needs `w` forces a total of at least `w / p`, and columns without a percentage
/// have to fit in the `1 - Σp` the percentage columns leave behind.
fn grid_min_max(columns: &[Column], is_fixed: bool) -> (f32, f32) {
    /// The width Blink hands a table whose columns claim the whole 100%
    const UNBOUNDED_WIDTH: f32 = 1_000_000.0;

    let mut min = 0.0;
    let mut max = 0.0;
    let mut percent_estimate: f32 = 0.0;
    let mut non_percent_max = 0.0;
    let mut percent_sum = 0.0;

    for column in columns.iter() {
        // A fixed table cannot shrink a specified column below its width
        let is_fixed_column = column.is_constrained && column.percent.is_none();
        min += if is_fixed && is_fixed_column { column.max } else { column.min };
        max += column.max;

        match column.percent {
            Some(percent) if percent > 0.0 => {
                if column.max > 0.0 {
                    percent_estimate = percent_estimate.max(column.max / percent);
                }
            }
            _ => non_percent_max += column.max,
        }
        percent_sum += column.percent.unwrap_or(0.0);
    }

    percent_sum = percent_sum.min(1.0);
    if percent_sum > 0.0 {
        let from_percentages = if non_percent_max == 0.0 {
            0.0
        } else if percent_sum >= 1.0 {
            UNBOUNDED_WIDTH
        } else {
            non_percent_max / (1.0 - percent_sum)
        };
        max = max.max(from_percentages).max(percent_estimate);
    }

    (min, max.max(min))
}

/// Resolve used column widths for `table-layout: auto` over `assignable`, the table's content
/// width minus all border-spacing. Implements the spec's width distribution algorithm
/// (<https://www.w3.org/TR/css-tables-3/#width-distribution-algorithm>): size every column four
/// ways, find the first sizing that reaches the target, and share the shortfall among the columns
/// that sizing was about to grow.
fn resolve_auto_layout_columns(columns: &mut [Column], assignable: f32) {
    const MIN: usize = 0;
    const PERCENTAGE: usize = 1;
    const SPECIFIED: usize = 2;
    const MAX: usize = 3;

    if columns.is_empty() {
        return;
    }

    let mut guess = [0.0f32; 4];
    // What each guess adds over the one before it, summed over the columns it grows
    let mut growth = [0.0f32; 4];
    let (mut percent_count, mut fixed_count, mut auto_count) = (0usize, 0usize, 0usize);
    let (mut percent_sum, mut fixed_max_sum, mut auto_max_sum) = (0.0f32, 0.0f32, 0.0f32);

    for column in columns.iter() {
        guess[MIN] += column.min;
        if let Some(percent) = column.percent {
            let width = percent_width(column, assignable);
            percent_count += 1;
            percent_sum += percent;
            guess[PERCENTAGE] += width;
            guess[SPECIFIED] += width;
            guess[MAX] += width;
            growth[PERCENTAGE] += width - column.min;
        } else if column.is_constrained {
            fixed_count += 1;
            fixed_max_sum += column.max;
            guess[PERCENTAGE] += column.min;
            guess[SPECIFIED] += column.max;
            guess[MAX] += column.max;
            growth[SPECIFIED] += column.max - column.min;
        } else {
            auto_count += 1;
            auto_max_sum += column.max;
            guess[PERCENTAGE] += column.min;
            guess[SPECIFIED] += column.min;
            guess[MAX] += column.max;
            growth[MAX] += column.max - column.min;
        }
    }

    // Columns never go below their minimums, however narrow the table is
    let target = assignable.max(guess[MIN]);
    let mut shortfall = 0.0;
    let mut last_grown = None;

    match (MIN..=MAX).find(|&i| guess[i] >= target) {
        Some(MIN) => {
            for column in columns.iter_mut() {
                column.used = column.min;
            }
        }
        // Percentage columns grow towards their percentage of the table
        Some(PERCENTAGE) => {
            let distributable = target - guess[MIN];
            shortfall = distributable;
            for (index, column) in columns.iter_mut().enumerate() {
                column.used = column.min;
                if column.percent.is_some() {
                    let wanted = percent_width(column, target) - column.min;
                    let delta = if growth[PERCENTAGE] > 0.0 {
                        distributable * wanted / growth[PERCENTAGE]
                    } else {
                        distributable / percent_count as f32
                    };
                    column.used += delta;
                    shortfall -= delta;
                    last_grown = Some(index);
                }
            }
        }
        // Columns with a specified width grow towards it, auto columns stay at their minimum
        Some(SPECIFIED) => {
            let distributable = target - guess[PERCENTAGE];
            shortfall = distributable;
            for (index, column) in columns.iter_mut().enumerate() {
                if column.percent.is_some() {
                    column.used = percent_width(column, target);
                } else if column.is_constrained {
                    let delta = if growth[SPECIFIED] > 0.0 {
                        distributable * (column.max - column.min) / growth[SPECIFIED]
                    } else {
                        distributable / fixed_count as f32
                    };
                    column.used = column.min + delta;
                    shortfall -= delta;
                    last_grown = Some(index);
                } else {
                    column.used = column.min;
                }
            }
        }
        // Auto columns grow towards their max-content width
        Some(_) => {
            let distributable = target - guess[SPECIFIED];
            // An exact match usually means an auto-width table, where handing out max-content
            // widths directly avoids rounding a column into wrapping its content
            let is_exact_match = target == guess[MAX];
            shortfall = if is_exact_match { 0.0 } else { distributable };

            for (index, column) in columns.iter_mut().enumerate() {
                if column.percent.is_some() {
                    column.used = percent_width(column, target);
                } else if column.is_constrained || is_exact_match {
                    column.used = column.max;
                } else {
                    let delta = if growth[MAX] > 0.0 {
                        distributable * (column.max - column.min) / growth[MAX]
                    } else {
                        distributable / auto_count as f32
                    };
                    column.used = column.min + delta;
                    shortfall -= delta;
                    last_grown = Some(index);
                }
            }
        }
        // Wider than every column's max-content width, so one category soaks up the rest
        None => {
            let distributable = target - guess[MAX];
            shortfall = distributable;

            for (index, column) in columns.iter_mut().enumerate() {
                let percent = column.percent;
                let (weight, weight_sum, count) = if auto_count > 0 {
                    (column.max, auto_max_sum, auto_count)
                } else if fixed_count > 0 {
                    (column.max, fixed_max_sum, fixed_count)
                } else {
                    (percent.unwrap_or(0.0), percent_sum, percent_count)
                };
                let grows = match (percent, column.is_constrained) {
                    (Some(_), _) => auto_count == 0 && fixed_count == 0,
                    (None, true) => auto_count == 0,
                    (None, false) => true,
                };

                column.used = if percent.is_some() { percent_width(column, target) } else { column.max };
                if grows {
                    let delta = if weight_sum > 0.0 {
                        distributable * weight / weight_sum
                    } else {
                        distributable / count as f32
                    };
                    column.used += delta;
                    shortfall -= delta;
                    last_grown = Some(index);
                }
            }
        }
    }

    // Rounding leaves the columns a hair short of the table, which the last one takes up
    if let Some(index) = last_grown {
        columns[index].used += shortfall;
    }
}

/// Resolve used column widths for `table-layout: fixed`: columns with a specified width take it,
/// percentage columns share out what is left, and auto columns split whatever remains
fn resolve_fixed_layout_columns(columns: &mut [Column], assignable: f32) {
    // Every browser treats a zero-width column as auto
    let is_fixed_column = |column: &Column| column.is_constrained && column.percent.is_none() && column.max > 0.0;
    let is_zero_width = |column: &Column| column.is_constrained && column.percent.is_none() && column.max == 0.0;

    let fixed_count = columns.iter().filter(|c| is_fixed_column(c)).count();
    let percent_count = columns.iter().filter(|c| c.percent.is_some()).count();
    let zero_width_count = columns.iter().filter(|c| is_zero_width(c)).count();
    let auto_count = columns.len() - fixed_count - percent_count - zero_width_count;

    let fixed_sum: f32 = columns.iter().filter(|c| is_fixed_column(c)).map(|c| c.max).sum();
    let percent_sum: f32 = columns.iter().filter(|c| c.percent.is_some()).map(|c| percent_width(c, assignable)).sum();

    let mut assigned = 0.0;
    let mut last = None;

    if fixed_count > 0 {
        // Columns with a specified width only stretch when nothing else can take the space
        let target = (assignable - percent_sum).max(0.0);
        let rescale = (fixed_sum < target && auto_count == 0) || fixed_sum > assignable;
        let scale = if rescale && fixed_sum > 0.0 { target / fixed_sum } else { 1.0 };

        for (index, column) in columns.iter_mut().enumerate().filter(|(_, c)| is_fixed_column(c)) {
            column.used = scale * column.max;
            assigned += column.used;
            last = Some(index);
        }
    }

    if percent_count > 0 && assigned < assignable {
        let target = assignable - assigned;
        let rescale = (percent_sum < target && auto_count == 0) || percent_sum > target;
        let scale = if rescale && percent_sum > 0.0 { target / percent_sum } else { 1.0 };

        for (index, column) in columns.iter_mut().enumerate().filter(|(_, c)| c.percent.is_some()) {
            column.used = if percent_sum > 0.0 {
                scale * percent_width(column, assignable)
            } else {
                target / percent_count as f32
            };
            assigned += column.used;
            last = Some(index);
        }
    }

    // Zero-width columns only grow when there is nothing else left to grow
    let grow_zero_width = zero_width_count == columns.len();
    let share_count = if grow_zero_width { zero_width_count } else { auto_count };
    let remaining = assignable - assigned;

    for (index, column) in columns.iter_mut().enumerate() {
        if column.percent.is_some() || is_fixed_column(column) || (is_zero_width(column) && !grow_zero_width) {
            continue;
        }
        column.used = remaining / share_count as f32;
        assigned += column.used;
        last = Some(index);
    }

    if let Some(index) = last {
        columns[index].used += assignable - assigned;
    }
}

/// Write a positioned layout for a non-cell table item (row or row group)
fn set_item_layout(
    tree: &mut impl LayoutTableContainer,
    node_id: NodeId,
    order: u32,
    location: Point<f32>,
    size: Size<f32>,
    parent_width: Option<f32>,
) {
    let style = tree.get_table_child_style(node_id);
    let padding = style.padding().resolve_or_zero(parent_width, |v, b| tree.calc(v, b));
    let border = style.border().resolve_or_zero(parent_width, |v, b| tree.calc(v, b));
    drop(style);

    tree.set_unrounded_layout(
        node_id,
        &Layout {
            order,
            location,
            size,
            #[cfg(feature = "content_size")]
            content_size: size,
            scrollbar_size: Size::ZERO,
            padding,
            border,
            margin: Rect::ZERO,
        },
    );
}
