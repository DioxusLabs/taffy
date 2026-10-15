//! Port of `yoga/algorithm/PixelGrid.cpp`: `roundValueToPixelGrid` and the layout rounding pass
//! `roundLayoutResultsToPixelGrid` (`round_yoga_layout`).

use super::numeric::inexact_equals_f64;
use crate::tree::{NodeId, RoundTree};

/// Rounds `value` to the pixel grid of `point_scale_factor` pixels per point, in `f64` as Yoga does.
///
/// `force_ceil`/`force_floor` are used for text nodes. Returns NaN for an undefined value or scale.
pub(crate) fn round_value_to_pixel_grid(
    value: f64,
    point_scale_factor: f64,
    force_ceil: bool,
    force_floor: bool,
) -> f32 {
    let mut scaled_value = value * point_scale_factor;
    // `fractional` is such that `floor(scaled_value) == scaled_value - fractional`, also for negatives.
    let mut fractional = scaled_value % 1.0;
    if fractional < 0.0 {
        fractional += 1.0;
    }
    if inexact_equals_f64(fractional, 0.0) {
        // Already rounded.
        scaled_value -= fractional;
    } else if inexact_equals_f64(fractional, 1.0) || force_ceil {
        scaled_value = scaled_value - fractional + 1.0;
    } else if force_floor {
        scaled_value -= fractional;
    } else {
        let round_up = !fractional.is_nan() && (fractional > 0.5 || inexact_equals_f64(fractional, 0.5));
        scaled_value = scaled_value - fractional + if round_up { 1.0 } else { 0.0 };
    }
    if scaled_value.is_nan() || point_scale_factor.is_nan() {
        f32::NAN
    } else {
        (scaled_value / point_scale_factor) as f32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rounds_to_scale() {
        assert_eq!(round_value_to_pixel_grid(10.4, 1.0, false, false), 10.0);
        assert_eq!(round_value_to_pixel_grid(10.5, 1.0, false, false), 11.0);
        assert_eq!(round_value_to_pixel_grid(10.49996, 1.0, false, false), 11.0);
        assert_eq!(round_value_to_pixel_grid(-2.2, 1.0, false, false), -2.0);
        assert_eq!(round_value_to_pixel_grid(-2.7, 1.0, false, false), -3.0);
        assert_eq!(round_value_to_pixel_grid(10.3, 2.0, false, false), 10.5);
        assert_eq!(round_value_to_pixel_grid(10.2, 2.0, false, false), 10.0);
        assert_eq!(round_value_to_pixel_grid(10.2, 1.0, true, false), 11.0);
        assert_eq!(round_value_to_pixel_grid(10.8, 1.0, false, true), 10.0);
        assert_eq!(round_value_to_pixel_grid(10.00005, 1.0, true, false), 10.0);
        assert!(round_value_to_pixel_grid(f64::NAN, 1.0, false, false).is_nan());
    }
}

/// Yoga's `roundLayoutResultsToPixelGrid`: round the unrounded layouts of `root`'s subtree to the pixel
/// grid of `point_scale_factor` pixels per point and store them as the final layouts.
///
/// Unlike [`round_layout`](crate::round_layout), a node's position is rounded on its own (relative to
/// the parent) while its size is the difference of its rounded absolute edges; padding, border and
/// scrollbar sizes are left unrounded, as in Yoga. A scale of 0 disables rounding, as in Yoga, and
/// stores the unrounded layouts. Nodes for which `is_text` returns true get Yoga's text rounding
/// (`NodeType::Text`, which `YGNodeSetMeasureFunc` sets on every measured leaf): the position is
/// floored and a fractional size is rounded up, a whole one down.
pub fn round_yoga_layout<Tree: RoundTree>(
    tree: &mut Tree,
    root: NodeId,
    point_scale_factor: f32,
    is_text: impl Fn(&Tree, NodeId) -> bool,
) {
    return round_inner(tree, root, 0.0, 0.0, point_scale_factor as f64, &is_text);

    /// One node, with the unrounded absolute position of its parent
    fn round_inner<Tree: RoundTree>(
        tree: &mut Tree,
        node: NodeId,
        absolute_left: f64,
        absolute_top: f64,
        scale: f64,
        is_text: &impl Fn(&Tree, NodeId) -> bool,
    ) {
        let unrounded = tree.get_unrounded_layout(node);
        let mut layout = unrounded;

        let node_left = unrounded.location.x as f64;
        let node_top = unrounded.location.y as f64;
        let node_width = unrounded.size.width as f64;
        let node_height = unrounded.size.height as f64;
        let absolute_node_left = absolute_left + node_left;
        let absolute_node_top = absolute_top + node_top;
        let absolute_node_right = absolute_node_left + node_width;
        let absolute_node_bottom = absolute_node_top + node_height;

        if scale != 0.0 {
            let text_rounding = is_text(tree, node);
            layout.location.x = round_value_to_pixel_grid(node_left, scale, false, text_rounding);
            layout.location.y = round_value_to_pixel_grid(node_top, scale, false, text_rounding);

            let scaled_width = node_width * scale;
            let has_fractional_width = !inexact_equals_f64(scaled_width.round(), scaled_width);
            let scaled_height = node_height * scale;
            let has_fractional_height = !inexact_equals_f64(scaled_height.round(), scaled_height);
            layout.size.width = round_value_to_pixel_grid(
                absolute_node_right,
                scale,
                text_rounding && has_fractional_width,
                text_rounding && !has_fractional_width,
            ) - round_value_to_pixel_grid(absolute_node_left, scale, false, text_rounding);
            layout.size.height = round_value_to_pixel_grid(
                absolute_node_bottom,
                scale,
                text_rounding && has_fractional_height,
                text_rounding && !has_fractional_height,
            ) - round_value_to_pixel_grid(absolute_node_top, scale, false, text_rounding);
        }
        tree.set_final_layout(node, &layout);

        let child_count = tree.child_count(node);
        for index in 0..child_count {
            let child = tree.get_child_id(node, index);
            if !tree.is_out_of_flow(child) {
                round_inner(tree, child, absolute_node_left, absolute_node_top, scale, is_text);
            }
        }
        let hoisted_count = tree.hoisted_child_count(node);
        for index in 0..hoisted_count {
            let child = tree.get_hoisted_child_id(node, index);
            round_inner(tree, child, absolute_node_left, absolute_node_top, scale, is_text);
        }
    }
}
