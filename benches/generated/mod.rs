use criterion::{criterion_group, criterion_main, Criterion};
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum WritingMode {
    Horizontal,
    Vertical,
}
#[allow(dead_code)]
fn measure_standard_text(
    known_dimensions: taffy::geometry::Size<Option<f32>>,
    available_space: taffy::geometry::Size<taffy::style::AvailableSpace>,
    text_content: &str,
    writing_mode: WritingMode,
    _aspect_ratio: Option<f32>,
) -> taffy::geometry::Size<f32> {
    use taffy::axis::AbsoluteAxis;
    use taffy::prelude::*;
    const ZWS: char = '\u{200B}';
    const H_WIDTH: f32 = 10.0;
    const H_HEIGHT: f32 = 10.0;
    if let Size { width: Some(width), height: Some(height) } = known_dimensions {
        return Size { width, height };
    }
    let inline_axis = match writing_mode {
        WritingMode::Horizontal => AbsoluteAxis::Horizontal,
        WritingMode::Vertical => AbsoluteAxis::Vertical,
    };
    let block_axis = inline_axis.other_axis();
    let lines: Vec<&str> = text_content.split(ZWS).collect();
    if lines.is_empty() {
        return Size::ZERO;
    }
    let min_line_length: usize = lines.iter().map(|line| line.len()).max().unwrap_or(0);
    let max_line_length: usize = lines.iter().map(|line| line.len()).sum();
    let inline_size =
        known_dimensions.get_abs(inline_axis).unwrap_or_else(|| match available_space.get_abs(inline_axis) {
            AvailableSpace::MinContent => min_line_length as f32 * H_WIDTH,
            AvailableSpace::MaxContent => max_line_length as f32 * H_WIDTH,
            AvailableSpace::Definite(inline_size) => {
                inline_size.min(max_line_length as f32 * H_WIDTH).max(min_line_length as f32 * H_WIDTH)
            }
        });
    let block_size = known_dimensions.get_abs(block_axis).unwrap_or_else(|| {
        let inline_line_length = (inline_size / H_WIDTH).floor() as usize;
        let mut line_count = 1;
        let mut current_line_length = 0;
        for line in &lines {
            if current_line_length + line.len() > inline_line_length {
                if current_line_length > 0 {
                    line_count += 1
                };
                current_line_length = line.len();
            } else {
                current_line_length += line.len();
            };
        }
        (line_count as f32) * H_HEIGHT
    });
    match writing_mode {
        WritingMode::Horizontal => Size { width: inline_size, height: block_size },
        WritingMode::Vertical => Size { width: block_size, height: inline_size },
    }
}
mod absolute_layout_align_items_and_justify_content_center;
mod absolute_layout_align_items_and_justify_content_center_and_bottom_position;
mod absolute_layout_align_items_and_justify_content_center_and_left_position;
mod absolute_layout_align_items_and_justify_content_center_and_right_position;
mod absolute_layout_align_items_and_justify_content_center_and_top_position;
mod absolute_layout_align_items_and_justify_content_flex_end;
mod absolute_layout_align_items_center;
mod absolute_layout_align_items_center_on_child_only;
mod absolute_layout_child_order;
mod absolute_layout_in_wrap_reverse_column_container;
mod absolute_layout_in_wrap_reverse_column_container_flex_end;
mod absolute_layout_in_wrap_reverse_row_container;
mod absolute_layout_in_wrap_reverse_row_container_flex_end;
mod absolute_layout_justify_content_center;
mod absolute_layout_no_size;
mod absolute_layout_percentage_bottom_based_on_parent_height;
mod absolute_layout_start_top_end_bottom;
mod absolute_layout_width_height_end_bottom;
mod absolute_layout_width_height_start_top;
mod absolute_layout_width_height_start_top_end_bottom;
mod absolute_layout_within_border;
mod absolute_margin_bottom_left;
mod align_baseline;
mod align_baseline_child_multiline;
mod align_baseline_nested_child;
mod align_center_should_size_based_on_content;
mod align_content_space_around_single_line;
mod align_content_space_around_wrapped;
mod align_content_space_between_single_line;
mod align_content_space_between_wrapped;
mod align_content_space_evenly_single_line;
mod align_content_space_evenly_wrapped;
mod align_flex_start_with_shrinking_children;
mod align_flex_start_with_shrinking_children_with_stretch;
mod align_flex_start_with_stretching_children;
mod align_items_center;
mod align_items_center_child_with_margin_bigger_than_parent;
mod align_items_center_child_without_margin_bigger_than_parent;
mod align_items_center_with_child_margin;
mod align_items_center_with_child_top;
mod align_items_flex_end;
mod align_items_flex_end_child_with_margin_bigger_than_parent;
mod align_items_flex_end_child_without_margin_bigger_than_parent;
mod align_items_flex_start;
mod align_items_min_max;
mod align_items_stretch;
mod align_self_baseline;
mod align_self_center;
mod align_self_flex_end;
mod align_self_flex_end_override_flex_start;
mod align_self_flex_start;
mod align_stretch_should_size_based_on_parent;
mod aspect_ratio_flex_absolute_aspect_ratio_overrides_height_of_full_inset;
mod aspect_ratio_flex_absolute_fill_height;
mod aspect_ratio_flex_absolute_fill_height_from_inset;
mod aspect_ratio_flex_absolute_fill_width;
mod aspect_ratio_flex_absolute_fill_width_from_inset;
mod aspect_ratio_flex_absolute_height_overrides_inset;
mod aspect_ratio_flex_absolute_width_overrides_inset;
mod aspect_ratio_flex_column_fill_height;
mod aspect_ratio_flex_column_fill_max_height;
mod aspect_ratio_flex_column_fill_max_width;
mod aspect_ratio_flex_column_fill_min_height;
mod aspect_ratio_flex_column_fill_min_width;
mod aspect_ratio_flex_column_fill_width;
mod aspect_ratio_flex_column_fill_width_flex;
mod aspect_ratio_flex_column_stretch_fill_height;
mod aspect_ratio_flex_column_stretch_fill_max_height;
mod aspect_ratio_flex_column_stretch_fill_max_width;
mod aspect_ratio_flex_column_stretch_fill_width;
mod aspect_ratio_flex_row_fill_height;
mod aspect_ratio_flex_row_fill_max_height;
mod aspect_ratio_flex_row_fill_max_width;
mod aspect_ratio_flex_row_fill_min_height;
mod aspect_ratio_flex_row_fill_min_width;
mod aspect_ratio_flex_row_fill_width;
mod aspect_ratio_flex_row_fill_width_flex;
mod aspect_ratio_flex_row_stretch_fill_height;
mod aspect_ratio_flex_row_stretch_fill_max_height;
mod aspect_ratio_flex_row_stretch_fill_max_width;
mod aspect_ratio_flex_row_stretch_fill_width;
mod border_center_child;
mod border_flex_child;
mod border_no_child;
mod border_stretch_child;
mod child_min_max_width_flexing;
mod container_with_unsized_child;
mod display_none;
mod display_none_absolute_child;
mod display_none_fixed_size;
mod display_none_with_child;
mod display_none_with_margin;
mod display_none_with_position;
mod display_none_with_position_absolute;
mod flex_basis_and_main_dimen_set_when_flexing;
mod flex_basis_flex_grow_column;
mod flex_basis_flex_grow_row;
mod flex_basis_flex_shrink_column;
mod flex_basis_flex_shrink_row;
mod flex_basis_larger_than_content_column;
mod flex_basis_larger_than_content_row;
mod flex_basis_overrides_main_size;
mod flex_basis_slightly_smaller_then_content_with_flex_grow_large_size;
mod flex_basis_smaller_than_content_column;
mod flex_basis_smaller_than_content_row;
mod flex_basis_smaller_than_main_dimen_column;
mod flex_basis_smaller_than_main_dimen_row;
mod flex_basis_smaller_then_content_with_flex_grow_large_size;
mod flex_basis_smaller_then_content_with_flex_grow_small_size;
mod flex_basis_smaller_then_content_with_flex_grow_unconstraint_size;
mod flex_basis_smaller_then_content_with_flex_grow_very_large_size;
mod flex_basis_unconstraint_column;
mod flex_basis_unconstraint_row;
mod flex_direction_column;
mod flex_direction_column_no_height;
mod flex_direction_column_reverse;
mod flex_direction_row;
mod flex_direction_row_no_width;
mod flex_direction_row_reverse;
mod flex_grow_child;
mod flex_grow_flex_basis_percent_min_max;
mod flex_grow_height_maximized;
mod flex_grow_in_at_most_container;
mod flex_grow_less_than_factor_one;
mod flex_grow_root_minimized;
mod flex_grow_shrink_at_most;
mod flex_grow_to_min;
mod flex_grow_within_constrained_max_column;
mod flex_grow_within_constrained_max_row;
mod flex_grow_within_constrained_max_width;
mod flex_grow_within_constrained_min_column;
mod flex_grow_within_constrained_min_max_column;
mod flex_grow_within_constrained_min_row;
mod flex_grow_within_max_width;
mod flex_root_ignored;
mod flex_shrink_by_outer_margin_with_max_size;
mod flex_shrink_flex_grow_child_flex_shrink_other_child;
mod flex_shrink_flex_grow_row;
mod flex_shrink_to_zero;
mod flex_wrap_align_stretch_fits_one_row;
mod flex_wrap_children_with_min_main_overriding_flex_basis;
mod flex_wrap_wrap_to_child_height;
mod gap_column_gap_child_margins;
mod gap_column_gap_determines_parent_width;
mod gap_column_gap_flexible;
mod gap_column_gap_flexible_undefined_parent;
mod gap_column_gap_inflexible;
mod gap_column_gap_inflexible_undefined_parent;
mod gap_column_gap_justify_center;
mod gap_column_gap_justify_flex_end;
mod gap_column_gap_justify_flex_start;
mod gap_column_gap_justify_space_around;
mod gap_column_gap_justify_space_between;
mod gap_column_gap_justify_space_evenly;
mod gap_column_gap_mixed_flexible;
mod gap_column_gap_percentage_cyclic_partially_shrinkable;
mod gap_column_gap_percentage_cyclic_shrinkable;
mod gap_column_gap_percentage_cyclic_unshrinkable;
mod gap_column_gap_percentage_flexible;
mod gap_column_gap_percentage_flexible_with_padding;
mod gap_column_gap_percentage_inflexible;
mod gap_column_gap_wrap_align_center;
mod gap_column_gap_wrap_align_flex_end;
mod gap_column_gap_wrap_align_flex_start;
mod gap_column_gap_wrap_align_space_around;
mod gap_column_gap_wrap_align_space_between;
mod gap_column_gap_wrap_align_stretch;
mod gap_column_row_gap_wrapping;
mod gap_percentage_row_gap_wrapping;
mod gap_row_gap_align_items_end;
mod gap_row_gap_align_items_stretch;
mod gap_row_gap_column_child_margins;
mod gap_row_gap_determines_parent_height;
mod gap_row_gap_row_wrap_child_margins;
#[cfg(feature = "grid")]
mod grid_absolute_align_self_sized_all;
#[cfg(feature = "grid")]
mod grid_absolute_column_end;
#[cfg(feature = "grid")]
mod grid_absolute_column_start;
#[cfg(feature = "grid")]
mod grid_absolute_container_bottom_left;
#[cfg(feature = "grid")]
mod grid_absolute_container_bottom_left_margin;
#[cfg(feature = "grid")]
mod grid_absolute_container_left_overrides_right;
#[cfg(feature = "grid")]
mod grid_absolute_container_left_right;
#[cfg(feature = "grid")]
mod grid_absolute_container_left_right_margin;
#[cfg(feature = "grid")]
mod grid_absolute_container_negative_position;
#[cfg(feature = "grid")]
mod grid_absolute_container_negative_position_margin;
#[cfg(feature = "grid")]
mod grid_absolute_container_top_bottom;
#[cfg(feature = "grid")]
mod grid_absolute_container_top_bottom_margin;
#[cfg(feature = "grid")]
mod grid_absolute_container_top_right;
#[cfg(feature = "grid")]
mod grid_absolute_container_top_right_margin;
#[cfg(feature = "grid")]
mod grid_absolute_justify_self_sized_all;
#[cfg(feature = "grid")]
mod grid_absolute_layout_within_border;
#[cfg(feature = "grid")]
mod grid_absolute_row_end;
#[cfg(feature = "grid")]
mod grid_absolute_row_start;
#[cfg(feature = "grid")]
mod grid_absolute_top_overrides_bottom;
#[cfg(feature = "grid")]
mod grid_absolute_with_padding;
#[cfg(feature = "grid")]
mod grid_absolute_with_padding_and_margin;
#[cfg(feature = "grid")]
mod grid_align_content_center;
#[cfg(feature = "grid")]
mod grid_align_content_end;
#[cfg(feature = "grid")]
mod grid_align_content_end_with_padding_border;
#[cfg(feature = "grid")]
mod grid_align_content_space_around;
#[cfg(feature = "grid")]
mod grid_align_content_space_around_with_padding_border;
#[cfg(feature = "grid")]
mod grid_align_content_space_between;
#[cfg(feature = "grid")]
mod grid_align_content_space_between_with_padding_border;
#[cfg(feature = "grid")]
mod grid_align_content_space_evenly;
#[cfg(feature = "grid")]
mod grid_align_content_space_evenly_with_padding_border;
#[cfg(feature = "grid")]
mod grid_align_content_start;
#[cfg(feature = "grid")]
mod grid_align_content_start_with_padding_border;
#[cfg(feature = "grid")]
mod grid_align_items_sized_center;
#[cfg(feature = "grid")]
mod grid_align_items_sized_end;
#[cfg(feature = "grid")]
mod grid_align_items_sized_start;
#[cfg(feature = "grid")]
mod grid_align_items_sized_stretch;
#[cfg(feature = "grid")]
mod grid_align_self_sized_all;
#[cfg(feature = "grid")]
mod grid_aspect_ratio_absolute_aspect_ratio_overrides_height_of_full_inset;
#[cfg(feature = "grid")]
mod grid_aspect_ratio_absolute_fill_height_from_inset;
#[cfg(feature = "grid")]
mod grid_aspect_ratio_absolute_fill_width_from_inset;
#[cfg(feature = "grid")]
mod grid_aspect_ratio_absolute_height_overrides_inset;
#[cfg(feature = "grid")]
mod grid_aspect_ratio_absolute_width_overrides_inset;
#[cfg(feature = "grid")]
mod grid_aspect_ratio_child_fill_content_height;
#[cfg(feature = "grid")]
mod grid_aspect_ratio_child_fill_content_width;
#[cfg(feature = "grid")]
mod grid_aspect_ratio_fill_child_height;
#[cfg(feature = "grid")]
mod grid_aspect_ratio_fill_child_max_height;
#[cfg(feature = "grid")]
mod grid_aspect_ratio_fill_child_max_width;
#[cfg(feature = "grid")]
mod grid_aspect_ratio_fill_child_min_height;
#[cfg(feature = "grid")]
mod grid_aspect_ratio_fill_child_min_width;
#[cfg(feature = "grid")]
mod grid_aspect_ratio_fill_child_width;
#[cfg(feature = "grid")]
mod grid_aspect_ratio_overriden_by_explicit_sizes;
#[cfg(feature = "grid")]
mod grid_aspect_ratio_overriden_by_explicit_sizes_flex;
#[cfg(feature = "grid")]
mod grid_auto_columns_fixed_width;
#[cfg(feature = "grid")]
mod grid_auto_fill_fixed_size;
#[cfg(feature = "grid")]
mod grid_auto_fill_with_empty_auto_track;
#[cfg(feature = "grid")]
mod grid_auto_fit_with_empty_auto_track;
#[cfg(feature = "grid")]
mod grid_auto_single_item;
#[cfg(feature = "grid")]
mod grid_auto_single_item_fixed_width;
#[cfg(feature = "grid")]
mod grid_auto_single_item_fixed_width_with_definite_width;
#[cfg(feature = "grid")]
mod grid_basic;
#[cfg(feature = "grid")]
mod grid_basic_implicit_tracks;
#[cfg(feature = "grid")]
mod grid_basic_with_overflow;
#[cfg(feature = "grid")]
mod grid_basic_with_padding;
#[cfg(feature = "grid")]
mod grid_fit_content_points_argument;
#[cfg(feature = "grid")]
mod grid_fit_content_points_max_content;
#[cfg(feature = "grid")]
mod grid_fit_content_points_min_content;
#[cfg(feature = "grid")]
mod grid_fr_auto_no_sized_items;
#[cfg(feature = "grid")]
mod grid_fr_auto_single_item;
#[cfg(feature = "grid")]
mod grid_fr_fixed_size_no_content;
#[cfg(feature = "grid")]
mod grid_fr_fixed_size_single_item;
#[cfg(feature = "grid")]
mod grid_gap;
#[cfg(feature = "grid")]
mod grid_hidden;
#[cfg(feature = "grid")]
mod grid_justify_content_center;
#[cfg(feature = "grid")]
mod grid_justify_content_center_with_padding_border;
#[cfg(feature = "grid")]
mod grid_justify_content_end;
#[cfg(feature = "grid")]
mod grid_justify_content_end_with_padding_border;
#[cfg(feature = "grid")]
mod grid_justify_content_space_around;
#[cfg(feature = "grid")]
mod grid_justify_content_space_around_with_padding_border;
#[cfg(feature = "grid")]
mod grid_justify_content_space_between;
#[cfg(feature = "grid")]
mod grid_justify_content_space_between_with_padding_border;
#[cfg(feature = "grid")]
mod grid_justify_content_space_evenly;
#[cfg(feature = "grid")]
mod grid_justify_content_space_evenly_with_padding_border;
#[cfg(feature = "grid")]
mod grid_justify_content_start;
#[cfg(feature = "grid")]
mod grid_justify_content_start_with_padding_border;
#[cfg(feature = "grid")]
mod grid_justify_items_sized_center;
#[cfg(feature = "grid")]
mod grid_justify_items_sized_end;
#[cfg(feature = "grid")]
mod grid_justify_items_sized_start;
#[cfg(feature = "grid")]
mod grid_justify_items_sized_stretch;
#[cfg(feature = "grid")]
mod grid_justify_self_sized_all;
#[cfg(feature = "grid")]
mod grid_margins_auto_margins;
#[cfg(feature = "grid")]
mod grid_margins_auto_margins_override_stretch;
#[cfg(feature = "grid")]
mod grid_margins_fixed_center;
#[cfg(feature = "grid")]
mod grid_margins_fixed_end;
#[cfg(feature = "grid")]
mod grid_margins_fixed_start;
#[cfg(feature = "grid")]
mod grid_margins_fixed_stretch;
#[cfg(feature = "grid")]
mod grid_margins_percent_center;
#[cfg(feature = "grid")]
mod grid_margins_percent_end;
#[cfg(feature = "grid")]
mod grid_margins_percent_start;
#[cfg(feature = "grid")]
mod grid_margins_percent_stretch;
#[cfg(feature = "grid")]
mod grid_max_content_maximum_single_item;
#[cfg(feature = "grid")]
mod grid_max_content_single_item;
#[cfg(feature = "grid")]
mod grid_max_content_single_item_margin_auto;
#[cfg(feature = "grid")]
mod grid_max_content_single_item_margin_fixed;
#[cfg(feature = "grid")]
mod grid_max_content_single_item_margin_percent;
#[cfg(feature = "grid")]
mod grid_min_content_flex_column;
#[cfg(feature = "grid")]
mod grid_min_content_flex_row;
#[cfg(feature = "grid")]
mod grid_min_content_flex_single_item;
#[cfg(feature = "grid")]
mod grid_min_content_flex_single_item_margin_auto;
#[cfg(feature = "grid")]
mod grid_min_content_flex_single_item_margin_fixed;
#[cfg(feature = "grid")]
mod grid_min_content_flex_single_item_margin_percent;
#[cfg(feature = "grid")]
mod grid_min_content_maximum_single_item;
#[cfg(feature = "grid")]
mod grid_min_content_single_item;
#[cfg(feature = "grid")]
mod grid_min_max_column_auto;
#[cfg(feature = "grid")]
mod grid_min_max_column_fixed_width_above_range;
#[cfg(feature = "grid")]
mod grid_min_max_column_fixed_width_below_range;
#[cfg(feature = "grid")]
mod grid_min_max_column_fixed_width_within_range;
#[cfg(feature = "grid")]
mod grid_out_of_order_items;
#[cfg(feature = "grid")]
mod grid_percent_items_nested_moderate;
#[cfg(feature = "grid")]
mod grid_percent_items_nested_with_padding_margin;
#[cfg(feature = "grid")]
mod grid_percent_items_width_and_margin;
#[cfg(feature = "grid")]
mod grid_percent_items_width_and_padding;
#[cfg(feature = "grid")]
mod grid_percent_tracks_definite_overflow;
#[cfg(feature = "grid")]
mod grid_percent_tracks_definite_underflow;
#[cfg(feature = "grid")]
mod grid_percent_tracks_indefinite_only;
#[cfg(feature = "grid")]
mod grid_percent_tracks_indefinite_with_content_overflow;
#[cfg(feature = "grid")]
mod grid_percent_tracks_indefinite_with_content_underflow;
#[cfg(feature = "grid")]
mod grid_placement_auto_negative;
#[cfg(feature = "grid")]
mod grid_placement_definite_in_secondary_axis_with_fully_definite_negative;
#[cfg(feature = "grid")]
mod grid_relayout_vertical_text;
#[cfg(feature = "grid")]
mod grid_size_child_fixed_tracks;
mod justify_content_column_center;
mod justify_content_column_flex_end;
mod justify_content_column_flex_start;
mod justify_content_column_min_height_and_margin_bottom;
mod justify_content_column_min_height_and_margin_top;
mod justify_content_column_space_around;
mod justify_content_column_space_between;
mod justify_content_column_space_evenly;
mod justify_content_min_max;
mod justify_content_min_width_with_padding_child_width_greater_than_parent;
mod justify_content_min_width_with_padding_child_width_lower_than_parent;
mod justify_content_overflow_min_max;
mod justify_content_row_center;
mod justify_content_row_flex_end;
mod justify_content_row_flex_start;
mod justify_content_row_max_width_and_margin;
mod justify_content_row_min_width_and_margin;
mod justify_content_row_space_around;
mod justify_content_row_space_between;
mod justify_content_row_space_evenly;
mod margin_and_flex_column;
mod margin_and_flex_row;
mod margin_and_stretch_column;
mod margin_and_stretch_row;
mod margin_auto_bottom;
mod margin_auto_bottom_and_top;
mod margin_auto_bottom_and_top_justify_center;
mod margin_auto_left;
mod margin_auto_left_and_right;
mod margin_auto_left_and_right_column;
mod margin_auto_left_and_right_column_and_center;
mod margin_auto_left_and_right_stretch;
mod margin_auto_left_child_bigger_than_parent;
mod margin_auto_left_fix_right_child_bigger_than_parent;
mod margin_auto_left_right_child_bigger_than_parent;
mod margin_auto_left_stretching_child;
mod margin_auto_mutiple_children_column;
mod margin_auto_mutiple_children_row;
mod margin_auto_right;
mod margin_auto_top;
mod margin_auto_top_and_bottom_stretch;
mod margin_auto_top_stretching_child;
mod margin_bottom;
mod margin_fix_left_auto_right_child_bigger_than_parent;
mod margin_left;
mod margin_right;
mod margin_should_not_be_part_of_max_height;
mod margin_should_not_be_part_of_max_width;
mod margin_top;
mod margin_with_sibling_column;
mod margin_with_sibling_row;
mod max_height;
mod max_height_overrides_height;
mod max_height_overrides_height_on_root;
mod max_width;
mod max_width_overrides_width;
mod max_width_overrides_width_on_root;
mod measure_child;
mod measure_child_absolute;
mod measure_child_constraint;
mod measure_child_constraint_padding_parent;
mod measure_child_with_flex_grow;
mod measure_child_with_flex_shrink;
mod measure_flex_basis_overrides_measure;
mod measure_height_overrides_measure;
mod measure_remeasure_child_after_growing;
mod measure_remeasure_child_after_shrinking;
mod measure_remeasure_child_after_stretching;
mod measure_root;
mod measure_stretch_overrides_measure;
mod measure_width_overrides_measure;
mod min_height;
mod min_height_overrides_height;
mod min_height_overrides_height_on_root;
mod min_height_overrides_max_height;
mod min_max_percent_no_width_height;
mod min_width;
mod min_width_overrides_max_width;
mod min_width_overrides_width;
mod min_width_overrides_width_on_root;
mod nested_overflowing_child;
mod nested_overflowing_child_in_constraint_parent;
mod overflow_cross_axis;
mod overflow_main_axis;
mod padding_align_end_child;
mod padding_center_child;
mod padding_flex_child;
mod padding_no_child;
mod padding_stretch_child;
mod parent_wrap_child_size_overflowing_parent;
mod percent_absolute_position;
mod percent_within_flex_grow;
mod percentage_absolute_position;
mod percentage_container_in_wrapping_container;
mod percentage_flex_basis;
mod percentage_flex_basis_cross;
mod percentage_flex_basis_cross_max_height;
mod percentage_flex_basis_cross_max_width;
mod percentage_flex_basis_cross_min_height;
mod percentage_flex_basis_cross_min_width;
mod percentage_flex_basis_main_max_height;
mod percentage_flex_basis_main_max_width;
mod percentage_flex_basis_main_min_width;
mod percentage_margin_should_calculate_based_only_on_width;
mod percentage_moderate_complexity;
mod percentage_multiple_nested_with_padding_margin_and_percentage_values;
mod percentage_padding_should_calculate_based_only_on_width;
mod percentage_position_bottom_right;
mod percentage_position_left_top;
mod percentage_size_based_on_parent_inner_size;
mod percentage_size_of_flex_basis;
mod percentage_width_height;
mod percentage_width_height_undefined_parent_size;
mod relative_position_should_not_nudge_siblings;
mod rounding_flex_basis_flex_grow_row_prime_number_width;
mod rounding_flex_basis_flex_grow_row_width_of_100;
mod rounding_flex_basis_flex_shrink_row;
mod rounding_flex_basis_overrides_main_size;
mod rounding_fractial_input_1;
mod rounding_fractial_input_2;
mod rounding_fractial_input_3;
mod rounding_fractial_input_4;
mod rounding_total_fractial;
mod rounding_total_fractial_nested;
mod size_defined_by_child;
mod size_defined_by_child_with_border;
mod size_defined_by_child_with_padding;
mod size_defined_by_grand_child;
mod width_smaller_then_content_with_flex_grow_large_size;
mod width_smaller_then_content_with_flex_grow_small_size;
mod width_smaller_then_content_with_flex_grow_unconstraint_size;
mod width_smaller_then_content_with_flex_grow_very_large_size;
mod wrap_column;
mod wrap_nodes_with_content_sizing_margin_cross;
mod wrap_nodes_with_content_sizing_overflowing_margin;
mod wrap_reverse_column;
mod wrap_reverse_column_fixed_size;
mod wrap_reverse_row;
mod wrap_reverse_row_align_content_center;
mod wrap_reverse_row_align_content_flex_start;
mod wrap_reverse_row_align_content_space_around;
mod wrap_reverse_row_align_content_stretch;
mod wrap_reverse_row_single_line_different_size;
mod wrap_row;
mod wrap_row_align_items_center;
mod wrap_row_align_items_flex_end;
mod wrapped_column_max_height;
mod wrapped_column_max_height_flex;
mod wrapped_row_within_align_items_center;
mod wrapped_row_within_align_items_flex_end;
mod wrapped_row_within_align_items_flex_start;
fn benchmark(c: &mut Criterion) {
    c.bench_function("generated benchmarks", |b| {
        b.iter(|| {
            absolute_layout_align_items_and_justify_content_center::compute();
            absolute_layout_align_items_and_justify_content_center_and_bottom_position::compute();
            absolute_layout_align_items_and_justify_content_center_and_left_position::compute();
            absolute_layout_align_items_and_justify_content_center_and_right_position::compute();
            absolute_layout_align_items_and_justify_content_center_and_top_position::compute();
            absolute_layout_align_items_and_justify_content_flex_end::compute();
            absolute_layout_align_items_center::compute();
            absolute_layout_align_items_center_on_child_only::compute();
            absolute_layout_child_order::compute();
            absolute_layout_in_wrap_reverse_column_container::compute();
            absolute_layout_in_wrap_reverse_column_container_flex_end::compute();
            absolute_layout_in_wrap_reverse_row_container::compute();
            absolute_layout_in_wrap_reverse_row_container_flex_end::compute();
            absolute_layout_justify_content_center::compute();
            absolute_layout_no_size::compute();
            absolute_layout_percentage_bottom_based_on_parent_height::compute();
            absolute_layout_start_top_end_bottom::compute();
            absolute_layout_width_height_end_bottom::compute();
            absolute_layout_width_height_start_top::compute();
            absolute_layout_width_height_start_top_end_bottom::compute();
            absolute_layout_within_border::compute();
            absolute_margin_bottom_left::compute();
            align_baseline::compute();
            align_baseline_child_multiline::compute();
            align_baseline_nested_child::compute();
            align_center_should_size_based_on_content::compute();
            align_content_space_around_single_line::compute();
            align_content_space_around_wrapped::compute();
            align_content_space_between_single_line::compute();
            align_content_space_between_wrapped::compute();
            align_content_space_evenly_single_line::compute();
            align_content_space_evenly_wrapped::compute();
            align_flex_start_with_shrinking_children::compute();
            align_flex_start_with_shrinking_children_with_stretch::compute();
            align_flex_start_with_stretching_children::compute();
            align_items_center::compute();
            align_items_center_child_with_margin_bigger_than_parent::compute();
            align_items_center_child_without_margin_bigger_than_parent::compute();
            align_items_center_with_child_margin::compute();
            align_items_center_with_child_top::compute();
            align_items_flex_end::compute();
            align_items_flex_end_child_with_margin_bigger_than_parent::compute();
            align_items_flex_end_child_without_margin_bigger_than_parent::compute();
            align_items_flex_start::compute();
            align_items_min_max::compute();
            align_items_stretch::compute();
            align_self_baseline::compute();
            align_self_center::compute();
            align_self_flex_end::compute();
            align_self_flex_end_override_flex_start::compute();
            align_self_flex_start::compute();
            align_stretch_should_size_based_on_parent::compute();
            aspect_ratio_flex_absolute_aspect_ratio_overrides_height_of_full_inset::compute();
            aspect_ratio_flex_absolute_fill_height::compute();
            aspect_ratio_flex_absolute_fill_height_from_inset::compute();
            aspect_ratio_flex_absolute_fill_width::compute();
            aspect_ratio_flex_absolute_fill_width_from_inset::compute();
            aspect_ratio_flex_absolute_height_overrides_inset::compute();
            aspect_ratio_flex_absolute_width_overrides_inset::compute();
            aspect_ratio_flex_column_fill_height::compute();
            aspect_ratio_flex_column_fill_max_height::compute();
            aspect_ratio_flex_column_fill_max_width::compute();
            aspect_ratio_flex_column_fill_min_height::compute();
            aspect_ratio_flex_column_fill_min_width::compute();
            aspect_ratio_flex_column_fill_width::compute();
            aspect_ratio_flex_column_fill_width_flex::compute();
            aspect_ratio_flex_column_stretch_fill_height::compute();
            aspect_ratio_flex_column_stretch_fill_max_height::compute();
            aspect_ratio_flex_column_stretch_fill_max_width::compute();
            aspect_ratio_flex_column_stretch_fill_width::compute();
            aspect_ratio_flex_row_fill_height::compute();
            aspect_ratio_flex_row_fill_max_height::compute();
            aspect_ratio_flex_row_fill_max_width::compute();
            aspect_ratio_flex_row_fill_min_height::compute();
            aspect_ratio_flex_row_fill_min_width::compute();
            aspect_ratio_flex_row_fill_width::compute();
            aspect_ratio_flex_row_fill_width_flex::compute();
            aspect_ratio_flex_row_stretch_fill_height::compute();
            aspect_ratio_flex_row_stretch_fill_max_height::compute();
            aspect_ratio_flex_row_stretch_fill_max_width::compute();
            aspect_ratio_flex_row_stretch_fill_width::compute();
            border_center_child::compute();
            border_flex_child::compute();
            border_no_child::compute();
            border_stretch_child::compute();
            child_min_max_width_flexing::compute();
            container_with_unsized_child::compute();
            display_none::compute();
            display_none_absolute_child::compute();
            display_none_fixed_size::compute();
            display_none_with_child::compute();
            display_none_with_margin::compute();
            display_none_with_position::compute();
            display_none_with_position_absolute::compute();
            flex_basis_and_main_dimen_set_when_flexing::compute();
            flex_basis_flex_grow_column::compute();
            flex_basis_flex_grow_row::compute();
            flex_basis_flex_shrink_column::compute();
            flex_basis_flex_shrink_row::compute();
            flex_basis_larger_than_content_column::compute();
            flex_basis_larger_than_content_row::compute();
            flex_basis_overrides_main_size::compute();
            flex_basis_slightly_smaller_then_content_with_flex_grow_large_size::compute();
            flex_basis_smaller_than_content_column::compute();
            flex_basis_smaller_than_content_row::compute();
            flex_basis_smaller_than_main_dimen_column::compute();
            flex_basis_smaller_than_main_dimen_row::compute();
            flex_basis_smaller_then_content_with_flex_grow_large_size::compute();
            flex_basis_smaller_then_content_with_flex_grow_small_size::compute();
            flex_basis_smaller_then_content_with_flex_grow_unconstraint_size::compute();
            flex_basis_smaller_then_content_with_flex_grow_very_large_size::compute();
            flex_basis_unconstraint_column::compute();
            flex_basis_unconstraint_row::compute();
            flex_direction_column::compute();
            flex_direction_column_no_height::compute();
            flex_direction_column_reverse::compute();
            flex_direction_row::compute();
            flex_direction_row_no_width::compute();
            flex_direction_row_reverse::compute();
            flex_grow_child::compute();
            flex_grow_flex_basis_percent_min_max::compute();
            flex_grow_height_maximized::compute();
            flex_grow_in_at_most_container::compute();
            flex_grow_less_than_factor_one::compute();
            flex_grow_root_minimized::compute();
            flex_grow_shrink_at_most::compute();
            flex_grow_to_min::compute();
            flex_grow_within_constrained_max_column::compute();
            flex_grow_within_constrained_max_row::compute();
            flex_grow_within_constrained_max_width::compute();
            flex_grow_within_constrained_min_column::compute();
            flex_grow_within_constrained_min_max_column::compute();
            flex_grow_within_constrained_min_row::compute();
            flex_grow_within_max_width::compute();
            flex_root_ignored::compute();
            flex_shrink_by_outer_margin_with_max_size::compute();
            flex_shrink_flex_grow_child_flex_shrink_other_child::compute();
            flex_shrink_flex_grow_row::compute();
            flex_shrink_to_zero::compute();
            flex_wrap_align_stretch_fits_one_row::compute();
            flex_wrap_children_with_min_main_overriding_flex_basis::compute();
            flex_wrap_wrap_to_child_height::compute();
            gap_column_gap_child_margins::compute();
            gap_column_gap_determines_parent_width::compute();
            gap_column_gap_flexible::compute();
            gap_column_gap_flexible_undefined_parent::compute();
            gap_column_gap_inflexible::compute();
            gap_column_gap_inflexible_undefined_parent::compute();
            gap_column_gap_justify_center::compute();
            gap_column_gap_justify_flex_end::compute();
            gap_column_gap_justify_flex_start::compute();
            gap_column_gap_justify_space_around::compute();
            gap_column_gap_justify_space_between::compute();
            gap_column_gap_justify_space_evenly::compute();
            gap_column_gap_mixed_flexible::compute();
            gap_column_gap_percentage_cyclic_partially_shrinkable::compute();
            gap_column_gap_percentage_cyclic_shrinkable::compute();
            gap_column_gap_percentage_cyclic_unshrinkable::compute();
            gap_column_gap_percentage_flexible::compute();
            gap_column_gap_percentage_flexible_with_padding::compute();
            gap_column_gap_percentage_inflexible::compute();
            gap_column_gap_wrap_align_center::compute();
            gap_column_gap_wrap_align_flex_end::compute();
            gap_column_gap_wrap_align_flex_start::compute();
            gap_column_gap_wrap_align_space_around::compute();
            gap_column_gap_wrap_align_space_between::compute();
            gap_column_gap_wrap_align_stretch::compute();
            gap_column_row_gap_wrapping::compute();
            gap_percentage_row_gap_wrapping::compute();
            gap_row_gap_align_items_end::compute();
            gap_row_gap_align_items_stretch::compute();
            gap_row_gap_column_child_margins::compute();
            gap_row_gap_determines_parent_height::compute();
            gap_row_gap_row_wrap_child_margins::compute();
            #[cfg(feature = "grid")]
            grid_absolute_align_self_sized_all::compute();
            #[cfg(feature = "grid")]
            grid_absolute_column_end::compute();
            #[cfg(feature = "grid")]
            grid_absolute_column_start::compute();
            #[cfg(feature = "grid")]
            grid_absolute_container_bottom_left::compute();
            #[cfg(feature = "grid")]
            grid_absolute_container_bottom_left_margin::compute();
            #[cfg(feature = "grid")]
            grid_absolute_container_left_overrides_right::compute();
            #[cfg(feature = "grid")]
            grid_absolute_container_left_right::compute();
            #[cfg(feature = "grid")]
            grid_absolute_container_left_right_margin::compute();
            #[cfg(feature = "grid")]
            grid_absolute_container_negative_position::compute();
            #[cfg(feature = "grid")]
            grid_absolute_container_negative_position_margin::compute();
            #[cfg(feature = "grid")]
            grid_absolute_container_top_bottom::compute();
            #[cfg(feature = "grid")]
            grid_absolute_container_top_bottom_margin::compute();
            #[cfg(feature = "grid")]
            grid_absolute_container_top_right::compute();
            #[cfg(feature = "grid")]
            grid_absolute_container_top_right_margin::compute();
            #[cfg(feature = "grid")]
            grid_absolute_justify_self_sized_all::compute();
            #[cfg(feature = "grid")]
            grid_absolute_layout_within_border::compute();
            #[cfg(feature = "grid")]
            grid_absolute_row_end::compute();
            #[cfg(feature = "grid")]
            grid_absolute_row_start::compute();
            #[cfg(feature = "grid")]
            grid_absolute_top_overrides_bottom::compute();
            #[cfg(feature = "grid")]
            grid_absolute_with_padding::compute();
            #[cfg(feature = "grid")]
            grid_absolute_with_padding_and_margin::compute();
            #[cfg(feature = "grid")]
            grid_align_content_center::compute();
            #[cfg(feature = "grid")]
            grid_align_content_end::compute();
            #[cfg(feature = "grid")]
            grid_align_content_end_with_padding_border::compute();
            #[cfg(feature = "grid")]
            grid_align_content_space_around::compute();
            #[cfg(feature = "grid")]
            grid_align_content_space_around_with_padding_border::compute();
            #[cfg(feature = "grid")]
            grid_align_content_space_between::compute();
            #[cfg(feature = "grid")]
            grid_align_content_space_between_with_padding_border::compute();
            #[cfg(feature = "grid")]
            grid_align_content_space_evenly::compute();
            #[cfg(feature = "grid")]
            grid_align_content_space_evenly_with_padding_border::compute();
            #[cfg(feature = "grid")]
            grid_align_content_start::compute();
            #[cfg(feature = "grid")]
            grid_align_content_start_with_padding_border::compute();
            #[cfg(feature = "grid")]
            grid_align_items_sized_center::compute();
            #[cfg(feature = "grid")]
            grid_align_items_sized_end::compute();
            #[cfg(feature = "grid")]
            grid_align_items_sized_start::compute();
            #[cfg(feature = "grid")]
            grid_align_items_sized_stretch::compute();
            #[cfg(feature = "grid")]
            grid_align_self_sized_all::compute();
            #[cfg(feature = "grid")]
            grid_aspect_ratio_absolute_aspect_ratio_overrides_height_of_full_inset::compute();
            #[cfg(feature = "grid")]
            grid_aspect_ratio_absolute_fill_height_from_inset::compute();
            #[cfg(feature = "grid")]
            grid_aspect_ratio_absolute_fill_width_from_inset::compute();
            #[cfg(feature = "grid")]
            grid_aspect_ratio_absolute_height_overrides_inset::compute();
            #[cfg(feature = "grid")]
            grid_aspect_ratio_absolute_width_overrides_inset::compute();
            #[cfg(feature = "grid")]
            grid_aspect_ratio_child_fill_content_height::compute();
            #[cfg(feature = "grid")]
            grid_aspect_ratio_child_fill_content_width::compute();
            #[cfg(feature = "grid")]
            grid_aspect_ratio_fill_child_height::compute();
            #[cfg(feature = "grid")]
            grid_aspect_ratio_fill_child_max_height::compute();
            #[cfg(feature = "grid")]
            grid_aspect_ratio_fill_child_max_width::compute();
            #[cfg(feature = "grid")]
            grid_aspect_ratio_fill_child_min_height::compute();
            #[cfg(feature = "grid")]
            grid_aspect_ratio_fill_child_min_width::compute();
            #[cfg(feature = "grid")]
            grid_aspect_ratio_fill_child_width::compute();
            #[cfg(feature = "grid")]
            grid_aspect_ratio_overriden_by_explicit_sizes::compute();
            #[cfg(feature = "grid")]
            grid_aspect_ratio_overriden_by_explicit_sizes_flex::compute();
            #[cfg(feature = "grid")]
            grid_auto_columns_fixed_width::compute();
            #[cfg(feature = "grid")]
            grid_auto_fill_fixed_size::compute();
            #[cfg(feature = "grid")]
            grid_auto_fill_with_empty_auto_track::compute();
            #[cfg(feature = "grid")]
            grid_auto_fit_with_empty_auto_track::compute();
            #[cfg(feature = "grid")]
            grid_auto_single_item::compute();
            #[cfg(feature = "grid")]
            grid_auto_single_item_fixed_width::compute();
            #[cfg(feature = "grid")]
            grid_auto_single_item_fixed_width_with_definite_width::compute();
            #[cfg(feature = "grid")]
            grid_basic::compute();
            #[cfg(feature = "grid")]
            grid_basic_implicit_tracks::compute();
            #[cfg(feature = "grid")]
            grid_basic_with_overflow::compute();
            #[cfg(feature = "grid")]
            grid_basic_with_padding::compute();
            #[cfg(feature = "grid")]
            grid_fit_content_points_argument::compute();
            #[cfg(feature = "grid")]
            grid_fit_content_points_max_content::compute();
            #[cfg(feature = "grid")]
            grid_fit_content_points_min_content::compute();
            #[cfg(feature = "grid")]
            grid_fr_auto_no_sized_items::compute();
            #[cfg(feature = "grid")]
            grid_fr_auto_single_item::compute();
            #[cfg(feature = "grid")]
            grid_fr_fixed_size_no_content::compute();
            #[cfg(feature = "grid")]
            grid_fr_fixed_size_single_item::compute();
            #[cfg(feature = "grid")]
            grid_gap::compute();
            #[cfg(feature = "grid")]
            grid_hidden::compute();
            #[cfg(feature = "grid")]
            grid_justify_content_center::compute();
            #[cfg(feature = "grid")]
            grid_justify_content_center_with_padding_border::compute();
            #[cfg(feature = "grid")]
            grid_justify_content_end::compute();
            #[cfg(feature = "grid")]
            grid_justify_content_end_with_padding_border::compute();
            #[cfg(feature = "grid")]
            grid_justify_content_space_around::compute();
            #[cfg(feature = "grid")]
            grid_justify_content_space_around_with_padding_border::compute();
            #[cfg(feature = "grid")]
            grid_justify_content_space_between::compute();
            #[cfg(feature = "grid")]
            grid_justify_content_space_between_with_padding_border::compute();
            #[cfg(feature = "grid")]
            grid_justify_content_space_evenly::compute();
            #[cfg(feature = "grid")]
            grid_justify_content_space_evenly_with_padding_border::compute();
            #[cfg(feature = "grid")]
            grid_justify_content_start::compute();
            #[cfg(feature = "grid")]
            grid_justify_content_start_with_padding_border::compute();
            #[cfg(feature = "grid")]
            grid_justify_items_sized_center::compute();
            #[cfg(feature = "grid")]
            grid_justify_items_sized_end::compute();
            #[cfg(feature = "grid")]
            grid_justify_items_sized_start::compute();
            #[cfg(feature = "grid")]
            grid_justify_items_sized_stretch::compute();
            #[cfg(feature = "grid")]
            grid_justify_self_sized_all::compute();
            #[cfg(feature = "grid")]
            grid_margins_auto_margins::compute();
            #[cfg(feature = "grid")]
            grid_margins_auto_margins_override_stretch::compute();
            #[cfg(feature = "grid")]
            grid_margins_fixed_center::compute();
            #[cfg(feature = "grid")]
            grid_margins_fixed_end::compute();
            #[cfg(feature = "grid")]
            grid_margins_fixed_start::compute();
            #[cfg(feature = "grid")]
            grid_margins_fixed_stretch::compute();
            #[cfg(feature = "grid")]
            grid_margins_percent_center::compute();
            #[cfg(feature = "grid")]
            grid_margins_percent_end::compute();
            #[cfg(feature = "grid")]
            grid_margins_percent_start::compute();
            #[cfg(feature = "grid")]
            grid_margins_percent_stretch::compute();
            #[cfg(feature = "grid")]
            grid_max_content_maximum_single_item::compute();
            #[cfg(feature = "grid")]
            grid_max_content_single_item::compute();
            #[cfg(feature = "grid")]
            grid_max_content_single_item_margin_auto::compute();
            #[cfg(feature = "grid")]
            grid_max_content_single_item_margin_fixed::compute();
            #[cfg(feature = "grid")]
            grid_max_content_single_item_margin_percent::compute();
            #[cfg(feature = "grid")]
            grid_min_content_flex_column::compute();
            #[cfg(feature = "grid")]
            grid_min_content_flex_row::compute();
            #[cfg(feature = "grid")]
            grid_min_content_flex_single_item::compute();
            #[cfg(feature = "grid")]
            grid_min_content_flex_single_item_margin_auto::compute();
            #[cfg(feature = "grid")]
            grid_min_content_flex_single_item_margin_fixed::compute();
            #[cfg(feature = "grid")]
            grid_min_content_flex_single_item_margin_percent::compute();
            #[cfg(feature = "grid")]
            grid_min_content_maximum_single_item::compute();
            #[cfg(feature = "grid")]
            grid_min_content_single_item::compute();
            #[cfg(feature = "grid")]
            grid_min_max_column_auto::compute();
            #[cfg(feature = "grid")]
            grid_min_max_column_fixed_width_above_range::compute();
            #[cfg(feature = "grid")]
            grid_min_max_column_fixed_width_below_range::compute();
            #[cfg(feature = "grid")]
            grid_min_max_column_fixed_width_within_range::compute();
            #[cfg(feature = "grid")]
            grid_out_of_order_items::compute();
            #[cfg(feature = "grid")]
            grid_percent_items_nested_moderate::compute();
            #[cfg(feature = "grid")]
            grid_percent_items_nested_with_padding_margin::compute();
            #[cfg(feature = "grid")]
            grid_percent_items_width_and_margin::compute();
            #[cfg(feature = "grid")]
            grid_percent_items_width_and_padding::compute();
            #[cfg(feature = "grid")]
            grid_percent_tracks_definite_overflow::compute();
            #[cfg(feature = "grid")]
            grid_percent_tracks_definite_underflow::compute();
            #[cfg(feature = "grid")]
            grid_percent_tracks_indefinite_only::compute();
            #[cfg(feature = "grid")]
            grid_percent_tracks_indefinite_with_content_overflow::compute();
            #[cfg(feature = "grid")]
            grid_percent_tracks_indefinite_with_content_underflow::compute();
            #[cfg(feature = "grid")]
            grid_placement_auto_negative::compute();
            #[cfg(feature = "grid")]
            grid_placement_definite_in_secondary_axis_with_fully_definite_negative::compute();
            #[cfg(feature = "grid")]
            grid_relayout_vertical_text::compute();
            #[cfg(feature = "grid")]
            grid_size_child_fixed_tracks::compute();
            justify_content_column_center::compute();
            justify_content_column_flex_end::compute();
            justify_content_column_flex_start::compute();
            justify_content_column_min_height_and_margin_bottom::compute();
            justify_content_column_min_height_and_margin_top::compute();
            justify_content_column_space_around::compute();
            justify_content_column_space_between::compute();
            justify_content_column_space_evenly::compute();
            justify_content_min_max::compute();
            justify_content_min_width_with_padding_child_width_greater_than_parent::compute();
            justify_content_min_width_with_padding_child_width_lower_than_parent::compute();
            justify_content_overflow_min_max::compute();
            justify_content_row_center::compute();
            justify_content_row_flex_end::compute();
            justify_content_row_flex_start::compute();
            justify_content_row_max_width_and_margin::compute();
            justify_content_row_min_width_and_margin::compute();
            justify_content_row_space_around::compute();
            justify_content_row_space_between::compute();
            justify_content_row_space_evenly::compute();
            margin_and_flex_column::compute();
            margin_and_flex_row::compute();
            margin_and_stretch_column::compute();
            margin_and_stretch_row::compute();
            margin_auto_bottom::compute();
            margin_auto_bottom_and_top::compute();
            margin_auto_bottom_and_top_justify_center::compute();
            margin_auto_left::compute();
            margin_auto_left_and_right::compute();
            margin_auto_left_and_right_column::compute();
            margin_auto_left_and_right_column_and_center::compute();
            margin_auto_left_and_right_stretch::compute();
            margin_auto_left_child_bigger_than_parent::compute();
            margin_auto_left_fix_right_child_bigger_than_parent::compute();
            margin_auto_left_right_child_bigger_than_parent::compute();
            margin_auto_left_stretching_child::compute();
            margin_auto_mutiple_children_column::compute();
            margin_auto_mutiple_children_row::compute();
            margin_auto_right::compute();
            margin_auto_top::compute();
            margin_auto_top_and_bottom_stretch::compute();
            margin_auto_top_stretching_child::compute();
            margin_bottom::compute();
            margin_fix_left_auto_right_child_bigger_than_parent::compute();
            margin_left::compute();
            margin_right::compute();
            margin_should_not_be_part_of_max_height::compute();
            margin_should_not_be_part_of_max_width::compute();
            margin_top::compute();
            margin_with_sibling_column::compute();
            margin_with_sibling_row::compute();
            max_height::compute();
            max_height_overrides_height::compute();
            max_height_overrides_height_on_root::compute();
            max_width::compute();
            max_width_overrides_width::compute();
            max_width_overrides_width_on_root::compute();
            measure_child::compute();
            measure_child_absolute::compute();
            measure_child_constraint::compute();
            measure_child_constraint_padding_parent::compute();
            measure_child_with_flex_grow::compute();
            measure_child_with_flex_shrink::compute();
            measure_flex_basis_overrides_measure::compute();
            measure_height_overrides_measure::compute();
            measure_remeasure_child_after_growing::compute();
            measure_remeasure_child_after_shrinking::compute();
            measure_remeasure_child_after_stretching::compute();
            measure_root::compute();
            measure_stretch_overrides_measure::compute();
            measure_width_overrides_measure::compute();
            min_height::compute();
            min_height_overrides_height::compute();
            min_height_overrides_height_on_root::compute();
            min_height_overrides_max_height::compute();
            min_max_percent_no_width_height::compute();
            min_width::compute();
            min_width_overrides_max_width::compute();
            min_width_overrides_width::compute();
            min_width_overrides_width_on_root::compute();
            nested_overflowing_child::compute();
            nested_overflowing_child_in_constraint_parent::compute();
            overflow_cross_axis::compute();
            overflow_main_axis::compute();
            padding_align_end_child::compute();
            padding_center_child::compute();
            padding_flex_child::compute();
            padding_no_child::compute();
            padding_stretch_child::compute();
            parent_wrap_child_size_overflowing_parent::compute();
            percent_absolute_position::compute();
            percent_within_flex_grow::compute();
            percentage_absolute_position::compute();
            percentage_container_in_wrapping_container::compute();
            percentage_flex_basis::compute();
            percentage_flex_basis_cross::compute();
            percentage_flex_basis_cross_max_height::compute();
            percentage_flex_basis_cross_max_width::compute();
            percentage_flex_basis_cross_min_height::compute();
            percentage_flex_basis_cross_min_width::compute();
            percentage_flex_basis_main_max_height::compute();
            percentage_flex_basis_main_max_width::compute();
            percentage_flex_basis_main_min_width::compute();
            percentage_margin_should_calculate_based_only_on_width::compute();
            percentage_moderate_complexity::compute();
            percentage_multiple_nested_with_padding_margin_and_percentage_values::compute();
            percentage_padding_should_calculate_based_only_on_width::compute();
            percentage_position_bottom_right::compute();
            percentage_position_left_top::compute();
            percentage_size_based_on_parent_inner_size::compute();
            percentage_size_of_flex_basis::compute();
            percentage_width_height::compute();
            percentage_width_height_undefined_parent_size::compute();
            relative_position_should_not_nudge_siblings::compute();
            rounding_flex_basis_flex_grow_row_prime_number_width::compute();
            rounding_flex_basis_flex_grow_row_width_of_100::compute();
            rounding_flex_basis_flex_shrink_row::compute();
            rounding_flex_basis_overrides_main_size::compute();
            rounding_fractial_input_1::compute();
            rounding_fractial_input_2::compute();
            rounding_fractial_input_3::compute();
            rounding_fractial_input_4::compute();
            rounding_total_fractial::compute();
            rounding_total_fractial_nested::compute();
            size_defined_by_child::compute();
            size_defined_by_child_with_border::compute();
            size_defined_by_child_with_padding::compute();
            size_defined_by_grand_child::compute();
            width_smaller_then_content_with_flex_grow_large_size::compute();
            width_smaller_then_content_with_flex_grow_small_size::compute();
            width_smaller_then_content_with_flex_grow_unconstraint_size::compute();
            width_smaller_then_content_with_flex_grow_very_large_size::compute();
            wrap_column::compute();
            wrap_nodes_with_content_sizing_margin_cross::compute();
            wrap_nodes_with_content_sizing_overflowing_margin::compute();
            wrap_reverse_column::compute();
            wrap_reverse_column_fixed_size::compute();
            wrap_reverse_row::compute();
            wrap_reverse_row_align_content_center::compute();
            wrap_reverse_row_align_content_flex_start::compute();
            wrap_reverse_row_align_content_space_around::compute();
            wrap_reverse_row_align_content_stretch::compute();
            wrap_reverse_row_single_line_different_size::compute();
            wrap_row::compute();
            wrap_row_align_items_center::compute();
            wrap_row_align_items_flex_end::compute();
            wrapped_column_max_height::compute();
            wrapped_column_max_height_flex::compute();
            wrapped_row_within_align_items_center::compute();
            wrapped_row_within_align_items_flex_end::compute();
            wrapped_row_within_align_items_flex_start::compute();
        })
    });
}
criterion_group!(benches, benchmark);
criterion_main!(benches);
