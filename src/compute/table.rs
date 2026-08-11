//! Computes CSS Table layout (CSS 2.1 §17 / [css-tables-3](https://www.w3.org/TR/css-tables-3/))
//!
//! Implements the automatic and fixed table layout algorithms for well-formed table trees
//! (table → optional row group → row → cell). Anonymous box fixup
//! (<https://www.w3.org/TR/css-tables-3/#fixup>) is the responsibility of the code
//! constructing the tree. Border collapsing is likewise expected to be resolved by the
//! embedder: with `border-collapse: collapse`, resolve the winning border for each edge,
//! write half of it into each cell's border style, and set `border_spacing` to zero.
use crate::geometry::{Line, Point, Rect, Size};
use crate::style::{
    AvailableSpace, CompactLength, CoreStyle, Direction, TableContainerStyle, TableItemStyle, TableLayout, TableRole,
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
    /// The cell's min-content inline size (border-box)
    min_content_width: f32,
    /// The cell's max-content inline size (border-box)
    max_content_width: f32,
    /// The column-width percentage specified on the cell, if any
    percent: Option<f32>,
    /// Whether the cell has a fixed (length) specified width
    is_constrained: bool,
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

/// A row of the table grid
struct Row {
    /// The node id of the row
    node_id: NodeId,
    /// Index of the row group this row belongs to, if any
    group: Option<usize>,
    /// The resolved used height
    used_height: f32,
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
    let mut rows: Vec<Row> = Vec::new();
    let mut groups: Vec<NodeId> = Vec::new();
    let mut cells: Vec<Cell> = Vec::new();
    // occupancy[col] = number of further rows the cell above still covers
    let mut occupancy: Vec<usize> = Vec::new();
    let mut col_count: usize = 0;

    let table_children: Vec<NodeId> = (0..tree.child_count(node_id)).map(|i| tree.get_child_id(node_id, i)).collect();

    for &child_id in table_children.iter() {
        let role = tree.get_table_child_style(child_id).table_role();
        match role {
            TableRole::RowGroup => {
                let group_index = groups.len();
                groups.push(child_id);
                let group_rows: Vec<NodeId> =
                    (0..tree.child_count(child_id)).map(|i| tree.get_child_id(child_id, i)).collect();
                for row_id in group_rows {
                    place_row(tree, row_id, Some(group_index), &mut rows, &mut cells, &mut occupancy, &mut col_count);
                }
            }
            _ => {
                place_row(tree, child_id, None, &mut rows, &mut cells, &mut occupancy, &mut col_count);
            }
        }
    }

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
    let mut columns: Vec<Column> = core::iter::repeat_with(Column::default).take(col_count).collect();

    for cell in cells.iter_mut() {
        let cell_style = tree.get_table_child_style(cell.node_id);
        let width_dim = cell_style.size().width;
        let width_tag = width_dim.tag();
        drop(cell_style);

        if width_tag == CompactLength::PERCENT_TAG {
            cell.percent = Some(width_dim.value());
        } else if width_tag == CompactLength::LENGTH_TAG {
            cell.is_constrained = true;
        }

        // Specified widths count toward max-content only; columns may shrink below them
        cell.min_content_width = tree.measure_child_size(
            cell.node_id,
            Size::NONE,
            Size::NONE,
            Size { width: AvailableSpace::MinContent, height: AvailableSpace::MinContent },
            SizingMode::ContentSize,
            crate::geometry::AbsoluteAxis::Horizontal,
            Line::FALSE,
        );
        cell.max_content_width = tree.measure_child_size(
            cell.node_id,
            Size::NONE,
            Size::NONE,
            Size { width: AvailableSpace::MaxContent, height: AvailableSpace::MaxContent },
            SizingMode::InherentSize,
            crate::geometry::AbsoluteAxis::Horizontal,
            Line::FALSE,
        );

        if cell.colspan == 1 {
            let col = &mut columns[cell.col];
            col.min = col.min.max(cell.min_content_width);
            col.max = col.max.max(cell.max_content_width);
            if let Some(pct) = cell.percent {
                col.percent = Some(col.percent.map_or(pct, |p: f32| p.max(pct)));
            }
            col.is_constrained |= cell.is_constrained;
        }
    }

    for cell in cells.iter().filter(|c| c.colspan > 1) {
        let range = cell.col..(cell.col + cell.colspan).min(col_count);
        let spanned_spacing = h_spacing * (range.len() as f32 - 1.0);

        let current_min: f32 = columns[range.clone()].iter().map(|c| c.min).sum::<f32>() + spanned_spacing;
        if cell.min_content_width > current_min {
            distribute(&mut columns[range.clone()], cell.min_content_width - current_min, |c| c.min, |c, v| c.min += v);
        }
        let current_max: f32 = columns[range.clone()].iter().map(|c| c.max).sum::<f32>() + spanned_spacing;
        if cell.max_content_width > current_max {
            distribute(&mut columns[range.clone()], cell.max_content_width - current_max, |c| c.max, |c, v| c.max += v);
        }
    }

    for col in columns.iter_mut() {
        col.max = col.max.max(col.min);
    }

    // Phase 3: resolve the table's used width and the columns' used widths
    let total_h_spacing = h_spacing * (col_count as f32 + 1.0);
    let grid_min: f32 = columns.iter().map(|c| c.min).sum::<f32>() + total_h_spacing;
    let grid_max: f32 = columns.iter().map(|c| c.max).sum::<f32>() + total_h_spacing;

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

    match table_layout_mode {
        TableLayout::Fixed if styled_known.width.is_some() => {
            resolve_fixed_layout_columns(&mut columns, &cells, assignable, h_spacing)
        }
        _ => resolve_auto_layout_columns(&mut columns, assignable),
    }

    if run_mode == RunMode::ComputeSize && axis == RequestedAxis::Horizontal {
        return LayoutOutput::from_outer_size(Size { width: table_width, height: styled_known.height.unwrap_or(0.0) });
    }

    // Phase 4: resolve row heights by sizing each cell at its final width
    let num_rows = rows.len();
    let cell_widths: Vec<f32> = cells
        .iter()
        .map(|cell| {
            let range = cell.col..(cell.col + cell.colspan).min(col_count);
            columns[range.clone()].iter().map(|c| c.used).sum::<f32>() + h_spacing * (range.len() as f32 - 1.0)
        })
        .collect();

    // Row baseline alignment per CSS 2.1 §17.5.4; a cell without a baseline synthesizes
    // one from the bottom of its content edge
    let mut cell_heights: Vec<f32> = core::iter::repeat(0.0).take(cells.len()).collect();
    let mut cell_baselines: Vec<Option<f32>> = core::iter::repeat(None).take(cells.len()).collect();
    let mut cell_shifts: Vec<f32> = core::iter::repeat(0.0).take(cells.len()).collect();
    let mut row_baselines: Vec<Option<f32>> = core::iter::repeat(None).take(num_rows).collect();

    for (index, (cell, &cell_width)) in cells.iter().zip(cell_widths.iter()).enumerate() {
        let output = tree.compute_child_layout(
            cell.node_id,
            LayoutInput {
                known_dimensions: Size { width: Some(cell_width), height: None },
                parent_size: Size { width: Some(table_width - pb_size.width), height: None },
                available_space: Size {
                    width: AvailableSpace::Definite(cell_width),
                    height: AvailableSpace::MaxContent,
                },
                sizing_mode: SizingMode::InherentSize,
                axis: RequestedAxis::Both,
                run_mode: RunMode::ComputeSize,
                vertical_margins_are_collapsible: Line::FALSE,
                content_offset_y: 0.0,
            },
        );
        cell_heights[index] = output.size.height;
        if cell.rowspan > 1 {
            continue;
        }

        let cell_style = tree.get_table_child_style(cell.node_id);
        let participates = cell_style.align_content().is_none();
        let pb_bottom = cell_style.padding().resolve_or_zero(parent_width, |v, b| tree.calc(v, b)).bottom
            + cell_style.border().resolve_or_zero(parent_width, |v, b| tree.calc(v, b)).bottom;
        drop(cell_style);

        if participates {
            let baseline = output.first_baselines.y.unwrap_or(output.size.height - pb_bottom);
            cell_baselines[index] = Some(baseline);
            row_baselines[cell.row] = Some(row_baselines[cell.row].map_or(baseline, |b: f32| b.max(baseline)));
        } else {
            rows[cell.row].used_height = rows[cell.row].used_height.max(output.size.height);
        }
    }

    for (index, cell) in cells.iter().enumerate() {
        if let (Some(baseline), Some(row_baseline)) = (cell_baselines[index], row_baselines[cell.row]) {
            let shift = row_baseline - baseline;
            cell_shifts[index] = shift;
            rows[cell.row].used_height = rows[cell.row].used_height.max(shift + cell_heights[index]);
        }
    }

    for row in rows.iter_mut() {
        let row_height =
            tree.get_table_child_style(row.node_id).size().height.maybe_resolve(None, |v, b| tree.calc(v, b));
        if let Some(h) = row_height {
            row.used_height = row.used_height.max(h);
        }
    }

    for (index, cell) in cells.iter().enumerate() {
        if cell.rowspan == 1 {
            continue;
        }
        let range = cell.row..(cell.row + cell.rowspan).min(num_rows);
        let span_len = range.len();
        let spanned: f32 =
            rows[range.clone()].iter().map(|r| r.used_height).sum::<f32>() + v_spacing * (span_len as f32 - 1.0);
        if cell_heights[index] > spanned {
            let extra = (cell_heights[index] - spanned) / span_len as f32;
            for row in rows[range].iter_mut() {
                row.used_height += extra;
            }
        }
    }

    let total_v_spacing = v_spacing * (num_rows as f32 + 1.0);
    let rows_height: f32 = rows.iter().map(|r| r.used_height).sum();
    let content_height = rows_height + total_v_spacing;
    let table_height = styled_known
        .height
        .unwrap_or((content_height + pb_size.height).maybe_clamp(min_size.height, max_size.height))
        .max(content_height + pb_size.height)
        .maybe_max(min_size.height);

    let extra_height = table_height - pb_size.height - content_height;
    if extra_height > 0.0 && num_rows > 0 {
        if rows_height > 0.0 {
            for row in rows.iter_mut() {
                row.used_height += extra_height * row.used_height / rows_height;
            }
        } else {
            for row in rows.iter_mut() {
                row.used_height += extra_height / num_rows as f32;
            }
        }
    }

    let final_size = Size { width: table_width, height: table_height };
    // The table's baseline is its first row's baseline (css-tables-3 §4.2)
    let table_baseline =
        row_baselines.first().copied().flatten().map(|baseline| padding_border.top + v_spacing + baseline);
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
    for (group_index, &group_id) in groups.iter().enumerate() {
        let group_rows: Vec<usize> =
            rows.iter().enumerate().filter(|(_, r)| r.group == Some(group_index)).map(|(i, _)| i).collect();
        let (start_y, end_y) = match (group_rows.first(), group_rows.last()) {
            (Some(&first), Some(&last)) => (row_y[first], row_y[last] + rows[last].used_height),
            _ => (padding_border.top + v_spacing, padding_border.top + v_spacing),
        };
        let origin = Point { x: grid_origin_x, y: start_y };
        group_origin.push(origin);

        let order = table_children.iter().position(|&id| id == group_id).unwrap_or(0) as u32;
        set_item_layout(
            tree,
            group_id,
            order,
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
        let order = match row.group {
            Some(g) => rows.iter().take(row_index).filter(|r| r.group == Some(g)).count() as u32,
            None => table_children.iter().position(|&id| id == row.node_id).unwrap_or(0) as u32,
        };
        set_item_layout(
            tree,
            row.node_id,
            order,
            origin,
            Size { width: grid_width, height: row.used_height },
            parent_width,
        );
    }

    let mut content_size = Size::ZERO;
    for (index, (cell, &cell_width)) in cells.iter().zip(cell_widths.iter()).enumerate() {
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
                content_offset_y: cell_shifts[index],
            },
        );

        let cell_style = tree.get_table_child_style(cell.node_id);
        let cell_padding = cell_style.padding().resolve_or_zero(parent_width, |v, b| tree.calc(v, b));
        let cell_border = cell_style.border().resolve_or_zero(parent_width, |v, b| tree.calc(v, b));
        drop(cell_style);

        // A spanning cell starts at its leftmost spanned column, which in RTL is the last one
        let col_range = cell.col..(cell.col + cell.colspan).min(col_count);
        let cell_x = col_x[col_range].iter().copied().fold(f32::INFINITY, f32::min);
        let location = Point { x: cell_x - grid_origin_x, y: 0.0 };
        let order = cells[..index].iter().filter(|c| c.row == cell.row).count() as u32;

        tree.set_unrounded_layout(
            cell.node_id,
            &Layout {
                order,
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

        content_size.width = content_size.width.max(col_x[cell.col] + cell_width);
        content_size.height = content_size.height.max(row_y[cell.row] + cell_height);
    }

    #[cfg(not(feature = "content_size"))]
    let _ = content_size;
    LayoutOutput::from_sizes_and_baselines(
        final_size,
        Size { width: content_size.width + padding.right, height: content_size.height + padding.bottom },
        baselines,
    )
}

/// Place one row's cells into the grid, skipping columns still covered by
/// rowspanning cells from earlier rows
fn place_row(
    tree: &mut impl LayoutTableContainer,
    row_id: NodeId,
    group: Option<usize>,
    rows: &mut Vec<Row>,
    cells: &mut Vec<Cell>,
    occupancy: &mut Vec<usize>,
    col_count: &mut usize,
) {
    let row_index = rows.len();
    rows.push(Row { node_id: row_id, group, used_height: 0.0 });

    let row_cells: Vec<NodeId> = (0..tree.child_count(row_id)).map(|i| tree.get_child_id(row_id, i)).collect();
    let mut col = 0;
    for cell_id in row_cells {
        let cell_style = tree.get_table_child_style(cell_id);
        let colspan = (cell_style.colspan() as usize).max(1);
        let rowspan = (cell_style.rowspan() as usize).max(1);
        drop(cell_style);

        while occupancy.get(col).is_some_and(|&o| o > 0) {
            col += 1;
        }
        if occupancy.len() < col + colspan {
            occupancy.resize(col + colspan, 0);
        }
        // Covers this row plus rowspan - 1 more; the end-of-row decrement consumes this row
        for slot in occupancy[col..col + colspan].iter_mut() {
            *slot = rowspan;
        }

        cells.push(Cell {
            node_id: cell_id,
            row: row_index,
            col,
            colspan,
            rowspan,
            min_content_width: 0.0,
            max_content_width: 0.0,
            percent: None,
            is_constrained: false,
        });
        col += colspan;
    }

    *col_count = (*col_count).max(occupancy.len().max(col));
    for slot in occupancy.iter_mut() {
        *slot = slot.saturating_sub(1);
    }
}

/// Distribute `extra` over `columns` proportionally to `weight`, equally when all weights are zero
fn distribute(columns: &mut [Column], extra: f32, weight: impl Fn(&Column) -> f32, add: impl Fn(&mut Column, f32)) {
    let total: f32 = columns.iter().map(&weight).sum();
    let count = columns.len() as f32;
    for col in columns.iter_mut() {
        let share = if total > 0.0 { extra * weight(col) / total } else { extra / count };
        add(col, share);
    }
}

/// Resolve used column widths for `table-layout: auto` given the assignable grid width
/// (the table's content width minus all border-spacing)
fn resolve_auto_layout_columns(columns: &mut [Column], assignable: f32) {
    // Each percentage is capped by what remains of 100% after earlier percentage columns
    let mut remaining = assignable;
    let mut percent_total = 0.0;
    let mut percent_budget = 1.0f32;
    for col in columns.iter_mut() {
        if let Some(pct) = col.percent {
            let effective = pct.min(percent_budget).max(0.0);
            percent_budget -= effective;
            col.used = (effective * assignable).max(col.min);
            percent_total += col.used;
        }
    }
    remaining -= percent_total;

    let others: Vec<usize> = (0..columns.len()).filter(|&i| columns[i].percent.is_none()).collect();

    // With only percentage columns, the percentages are scaled to fill the table exactly
    if others.is_empty() {
        if percent_total > 0.0 {
            let scale = assignable / percent_total;
            for col in columns.iter_mut() {
                col.used = (col.used * scale).max(col.min);
            }
        }
        return;
    }

    let min_sum: f32 = others.iter().map(|&i| columns[i].min).sum();
    let growth_sum: f32 = others.iter().map(|&i| columns[i].max - columns[i].min).sum();

    if remaining <= min_sum {
        for &i in others.iter() {
            columns[i].used = columns[i].min;
        }
        return;
    }

    if remaining <= min_sum + growth_sum {
        let t = if growth_sum > 0.0 { (remaining - min_sum) / growth_sum } else { 0.0 };
        for &i in others.iter() {
            columns[i].used = columns[i].min + (columns[i].max - columns[i].min) * t;
        }
        return;
    }

    // Surplus beyond max-content goes to unconstrained columns when any exist
    for &i in others.iter() {
        columns[i].used = columns[i].max;
    }
    let surplus = remaining - (min_sum + growth_sum);
    let unconstrained: Vec<usize> = others.iter().copied().filter(|&i| !columns[i].is_constrained).collect();
    let targets = if !unconstrained.is_empty() { unconstrained } else { others };
    if targets.is_empty() {
        return;
    }
    let weight_sum: f32 = targets.iter().map(|&i| columns[i].max).sum();
    for &i in targets.iter() {
        columns[i].used +=
            if weight_sum > 0.0 { surplus * columns[i].max / weight_sum } else { surplus / targets.len() as f32 };
    }
}

/// Resolve used column widths for `table-layout: fixed`: only the first row's cells
/// (and the table's width) determine column widths
fn resolve_fixed_layout_columns(columns: &mut [Column], cells: &[Cell], assignable: f32, h_spacing: f32) {
    let mut assigned: Vec<bool> = core::iter::repeat(false).take(columns.len()).collect();
    let mut remaining = assignable;
    for cell in cells.iter().filter(|c| c.row == 0) {
        let range = cell.col..(cell.col + cell.colspan).min(columns.len());
        let span_spacing = h_spacing * (range.len() as f32 - 1.0);
        let width = if let Some(pct) = cell.percent {
            Some(pct * assignable)
        } else if cell.is_constrained {
            Some(cell.max_content_width)
        } else {
            None
        };
        if let Some(w) = width {
            let per_col = (w - span_spacing).max(0.0) / range.len() as f32;
            for i in range {
                if !assigned[i] {
                    columns[i].used = per_col;
                    assigned[i] = true;
                    remaining -= per_col;
                }
            }
        }
    }

    let unassigned: Vec<usize> = (0..columns.len()).filter(|&i| !assigned[i]).collect();
    if !unassigned.is_empty() {
        let per_col = (remaining / unassigned.len() as f32).max(0.0);
        for i in unassigned {
            columns[i].used = per_col;
        }
    } else if remaining > 0.0 {
        // Table wider than the first row's specified widths: grow all columns proportionally
        distribute(columns, remaining, |c| c.used, |c, v| c.used += v);
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
