//! Computes the [flexbox](https://css-tricks.com/snippets/css/a-guide-to-flexbox/) layout algorithm on [`Taffy`](crate::Taffy) according to the [spec](https://www.w3.org/TR/css-flexbox-1/)
use crate::compute::LayoutAlgorithm;
use crate::geometry::{Point, Rect, Size};
use crate::style::{AvailableSpace, Display, LengthPercentageAuto, Overflow, Position};
use crate::tree::{Layout, RunMode, SizeAndBaselines, SizingMode};
use crate::tree::{LayoutTree, NodeId};
use crate::util::sys::f32_max;
use crate::util::sys::Vec;
use crate::util::MaybeMath;
use crate::util::{MaybeResolve, ResolveOrZero};

#[cfg(feature = "debug")]
use crate::util::debug::NODE_LOGGER;

/// The public interface to Taffy's Flexbox algorithm implementation
pub struct BlockAlgorithm;
impl LayoutAlgorithm for BlockAlgorithm {
    const NAME: &'static str = "FLEXBOX";

    fn perform_layout(
        tree: &mut impl LayoutTree,
        node: NodeId,
        known_dimensions: Size<Option<f32>>,
        parent_size: Size<Option<f32>>,
        available_space: Size<AvailableSpace>,
        _sizing_mode: SizingMode,
    ) -> SizeAndBaselines {
        compute(tree, node, known_dimensions, parent_size, available_space, RunMode::PeformLayout)
    }

    fn measure_size(
        tree: &mut impl LayoutTree,
        node: NodeId,
        known_dimensions: Size<Option<f32>>,
        parent_size: Size<Option<f32>>,
        available_space: Size<AvailableSpace>,
        _sizing_mode: SizingMode,
    ) -> Size<f32> {
        compute(tree, node, known_dimensions, parent_size, available_space, RunMode::ComputeSize).size
    }
}

/// The intermediate results of a flexbox calculation for a single item
struct BlockItem {
    /// The identifier for the associated node
    node_id: NodeId,

    /// The order of the item
    order: u32,

    /// The base size of this item
    size: Size<Option<f32>>,
    /// The minimum allowable size of this item
    min_size: Size<Option<f32>>,
    /// The maximum allowable size of this item
    max_size: Size<Option<f32>>,

    /// The display style of the item
    display: Display,
    /// The position style of the item
    position: Position,
    /// The overflow style of the item
    overflow: Point<Overflow>,

    /// The final offset of this item
    inset: Rect<LengthPercentageAuto>,
    /// The margin of this item
    margin: Rect<f32>,
    /// Whether each margin is an auto margin or not
    margin_is_auto: Rect<bool>,
    /// The padding of this item
    padding: Rect<f32>,
    /// The border of this item
    border: Rect<f32>,

    /// The computed border box size of this item
    computed_size: Size<f32>,
    /// The computed "static position" of this item. The static position is the position
    /// taking into account padding, border, margins, and scrollbar_gutters but not inset
    static_position: Point<f32>,

    /// The position of the bottom edge of this item
    baseline: f32,
}

/// Computes the layout of [`LayoutTree`] according to the block layout algorithm
pub fn compute(
    tree: &mut impl LayoutTree,
    node_id: NodeId,
    known_dimensions: Size<Option<f32>>,
    parent_size: Size<Option<f32>>,
    available_space: Size<AvailableSpace>,
    run_mode: RunMode,
) -> SizeAndBaselines {
    let style = tree.style(node_id);

    // Pull these out earlier to avoid borrowing issues
    let aspect_ratio = style.aspect_ratio;
    let margin = style.margin.resolve_or_zero(parent_size.width);
    let min_size = style.min_size.maybe_resolve(parent_size).maybe_apply_aspect_ratio(aspect_ratio);
    let max_size = style.max_size.maybe_resolve(parent_size).maybe_apply_aspect_ratio(aspect_ratio);
    let clamped_style_size =
        style.size.maybe_resolve(parent_size).maybe_apply_aspect_ratio(aspect_ratio).maybe_clamp(min_size, max_size);

    // If both min and max in a given axis are set and max <= min then this determines the size in that axis
    let min_max_definite_size = min_size.zip_map(max_size, |min, max| match (min, max) {
        (Some(min), Some(max)) if max <= min => Some(min),
        _ => None,
    });

    // Block nodes automatically stretch fit their width to fit available space if available space is definite
    let available_space_based_size =
        Size { width: available_space.width.into_option().maybe_sub(margin.horizontal_axis_sum()), height: None };

    let styled_based_known_dimensions =
        known_dimensions.or(min_max_definite_size).or(clamped_style_size).or(available_space_based_size);

    // Short-circuit layout if the container's size is fully determined by the container's size and the run mode
    // is ComputeSize (and thus the container's size is all that we're interested in)
    if run_mode == RunMode::ComputeSize {
        if let Size { width: Some(width), height: Some(height) } = styled_based_known_dimensions {
            return Size { width, height }.into();
        }
    }

    #[cfg(feature = "debug")]
    NODE_LOGGER.log("BLOCK");
    compute_inner(tree, node_id, styled_based_known_dimensions, parent_size, available_space, run_mode)
}

/// Computes the layout of [`LayoutTree`] according to the block layout algorithm
fn compute_inner(
    tree: &mut impl LayoutTree,
    node_id: NodeId,
    known_dimensions: Size<Option<f32>>,
    parent_size: Size<Option<f32>>,
    available_space: Size<AvailableSpace>,
    run_mode: RunMode,
) -> SizeAndBaselines {
    let style = tree.style(node_id);
    let raw_padding = style.padding;
    let raw_border = style.border;
    let padding = style.padding.resolve_or_zero(parent_size.width);
    let border = style.border.resolve_or_zero(parent_size.width);

    // Scrollbar gutters are reserved when the `overflow` property is set to `Overflow::Scroll`.
    // However, the axis are switched (transposed) because a node that scrolls vertically needs
    // *horizontal* space to be reserved for a scrollbar
    let scrollbar_gutter = {
        let offsets = style.overflow.transpose().map(|overflow| match overflow {
            Overflow::Scroll => style.scrollbar_width,
            _ => 0.0,
        });
        // TODO: make side configurable based on the `direction` property
        Rect { top: 0.0, left: 0.0, right: offsets.x, bottom: offsets.y }
    };
    let content_box_inset = padding + border + scrollbar_gutter;

    let container_content_box_size = known_dimensions.maybe_sub(content_box_inset.sum_axes());
    let mut items = generate_item_list(tree, node_id, container_content_box_size);

    // Compute container width
    let container_outer_width = known_dimensions.width.unwrap_or_else(|| {
        let available_width = available_space.width.maybe_sub(content_box_inset.horizontal_axis_sum());
        determine_content_based_container_width(tree, &items, available_width) + content_box_inset.horizontal_axis_sum()
    });

    let resolved_padding = raw_padding.resolve_or_zero(Some(container_outer_width));
    let resolved_border = raw_border.resolve_or_zero(Some(container_outer_width));
    let resolved_content_box_inset = resolved_padding + resolved_border + scrollbar_gutter;

    // Perform item layout and return content height
    let intrinsic_outer_height = perform_final_layout_on_in_flow_children(
        tree,
        &mut items,
        container_outer_width,
        content_box_inset,
        resolved_content_box_inset,
    );
    let container_outer_height = known_dimensions.height.unwrap_or(intrinsic_outer_height);
    let final_outer_size =
        known_dimensions.unwrap_or(Size { width: container_outer_width, height: container_outer_height });

    if run_mode == RunMode::ComputeSize {
        return final_outer_size.into();
    }

    // Layout absolutely positioned children
    let absolute_position_inset = resolved_border + scrollbar_gutter;
    let absolute_position_area = final_outer_size - absolute_position_inset.sum_axes();
    let absolute_position_offset = Point { x: absolute_position_inset.left, y: absolute_position_inset.top };
    perform_absolute_layout_on_absolute_children(tree, &mut items, absolute_position_area, absolute_position_offset);

    final_outer_size.into()
}

/// Create a `Vec` of `BlockItem` structs where each item in the `Vec` represents a child of the current node
#[inline]
fn generate_item_list(tree: &impl LayoutTree, node: NodeId, node_inner_size: Size<Option<f32>>) -> Vec<BlockItem> {
    tree.children(node)
        .map(|child_node_id| (child_node_id, tree.style(child_node_id)))
        .filter(|(_, style)| style.display != Display::None)
        .enumerate()
        .map(|(order, (child_node_id, child_style))| {
            let aspect_ratio = child_style.aspect_ratio;
            BlockItem {
                node_id: child_node_id,
                order: order as u32,
                display: child_style.display,
                position: child_style.position,
                size: child_style.size.maybe_resolve(node_inner_size).maybe_apply_aspect_ratio(aspect_ratio),
                min_size: child_style.min_size.maybe_resolve(node_inner_size).maybe_apply_aspect_ratio(aspect_ratio),
                max_size: child_style.max_size.maybe_resolve(node_inner_size).maybe_apply_aspect_ratio(aspect_ratio),

                inset: child_style.inset,
                margin: child_style.margin.resolve_or_zero(node_inner_size.width),
                margin_is_auto: child_style.margin.map(|m| m == LengthPercentageAuto::Auto),
                padding: child_style.padding.resolve_or_zero(node_inner_size.width),
                border: child_style.border.resolve_or_zero(node_inner_size.width),
                overflow: child_style.overflow,

                // Fields to be computed later (for now we initialise with dummy values)
                computed_size: Size::zero(),
                static_position: Point::zero(),
                baseline: 0.0,
            }
        })
        .collect()
}

/// Compute the content-based width in the case that the width of the container is not known
#[inline]
fn determine_content_based_container_width(
    tree: &mut impl LayoutTree,
    items: &[BlockItem],
    available_width: AvailableSpace,
) -> f32 {
    let available_space = Size { width: available_width, height: AvailableSpace::MinContent };

    let mut max_child_width = 0.0;
    for item in items.iter().filter(|item| item.position != Position::Absolute) {
        let known_dimensions = item.size.maybe_clamp(item.min_size, item.max_size);
        let width = known_dimensions.width.unwrap_or_else(|| {
            let size_and_baselines = tree.perform_child_layout(
                item.node_id,
                known_dimensions,
                Size::NONE,
                available_space,
                SizingMode::InherentSize,
            );

            size_and_baselines.size.width
        });

        max_child_width = f32_max(max_child_width, width);
    }

    max_child_width
}

/// Compute each child's final size and position
#[inline]
fn perform_final_layout_on_in_flow_children(
    tree: &mut impl LayoutTree,
    items: &mut [BlockItem],
    container_outer_width: f32,
    content_box_inset: Rect<f32>,
    resolved_content_box_inset: Rect<f32>,
) -> f32 {
    // Resolve container_inner_width for sizing child nodes using intial content_box_inset
    let container_inner_width = container_outer_width - content_box_inset.horizontal_axis_sum();
    let parent_size = Size { width: Some(container_outer_width), height: None };
    let available_space =
        Size { width: AvailableSpace::Definite(container_inner_width), height: AvailableSpace::MinContent };

    let mut total_y_offset = resolved_content_box_inset.top;
    for item in items.iter_mut() {
        if item.position == Position::Absolute {
            item.static_position.y = total_y_offset;
        } else {
            let known_dimensions = item
                .size
                .maybe_clamp(item.min_size, item.max_size)
                .map_width(|width| Some(width.unwrap_or(container_inner_width)));

            let size_and_baselines = tree.perform_child_layout(
                item.node_id,
                known_dimensions,
                parent_size,
                available_space,
                SizingMode::InherentSize,
            );

            item.computed_size = size_and_baselines.size;
            item.static_position = Point { x: resolved_content_box_inset.left, y: total_y_offset };

            // Resolve item inset
            let inset =
                item.inset.zip_size(Size { width: container_inner_width, height: 0.0 }, |p, s| p.maybe_resolve(s));
            let inset_offset = Point {
                x: inset.left.or(inset.right.map(|x| -x)).unwrap_or(0.0),
                y: inset.top.or(inset.bottom.map(|x| -x)).unwrap_or(0.0),
            };

            *tree.layout_mut(item.node_id) = Layout {
                order: item.order,
                size: size_and_baselines.size,
                location: Point {
                    x: resolved_content_box_inset.left + inset_offset.x,
                    y: total_y_offset + inset_offset.y,
                },
            };

            total_y_offset += size_and_baselines.size.height;
        }
    }

    total_y_offset += resolved_content_box_inset.bottom;
    total_y_offset
}

/// Perform absolute layout on all absolutely positioned children.
#[inline]
fn perform_absolute_layout_on_absolute_children(
    tree: &mut impl LayoutTree,
    items: &mut [BlockItem],
    area_size: Size<f32>,
    area_offset: Point<f32>,
) {
    let area_width = area_size.width;
    let area_height = area_size.height;

    for item in items.iter().filter(|item| item.position == Position::Absolute) {
        let child_style = tree.style(item.node_id);

        // Skip items that are display:none or are not position:absolute
        if child_style.display == Display::None || child_style.position != Position::Absolute {
            continue;
        }

        let aspect_ratio = child_style.aspect_ratio;
        let margin = child_style.margin.map(|margin| margin.resolve_to_option(area_width));
        let padding = child_style.padding.resolve_or_zero(Some(area_width));
        let border = child_style.border.resolve_or_zero(Some(area_width));
        let padding_border_sum = (padding + border).sum_axes();

        // Resolve inset
        let left = child_style.inset.left.maybe_resolve(area_width);
        let right = child_style.inset.right.maybe_resolve(area_width);
        let top = child_style.inset.top.maybe_resolve(area_height);
        let bottom = child_style.inset.bottom.maybe_resolve(area_height);

        // Compute known dimensions from min/max/inherent size styles
        let style_size = child_style.size.maybe_resolve(area_size).maybe_apply_aspect_ratio(aspect_ratio);
        let min_size = child_style
            .min_size
            .maybe_resolve(area_size)
            .maybe_apply_aspect_ratio(aspect_ratio)
            .or(padding_border_sum.map(Some))
            .maybe_max(padding_border_sum);
        let max_size = child_style.max_size.maybe_resolve(area_size).maybe_apply_aspect_ratio(aspect_ratio);
        let mut known_dimensions = style_size.maybe_clamp(min_size, max_size);

        // Fill in width from left/right and reapply aspect ratio if:
        //   - Width is not already known
        //   - Item has both left and right inset properties set
        if let (None, Some(left), Some(right)) = (known_dimensions.width, left, right) {
            let new_width_raw = area_width.maybe_sub(margin.left).maybe_sub(margin.right) - left - right;
            known_dimensions.width = Some(f32_max(new_width_raw, 0.0));
            known_dimensions = known_dimensions.maybe_apply_aspect_ratio(aspect_ratio).maybe_clamp(min_size, max_size);
        }

        // Fill in height from top/bottom and reapply aspect ratio if:
        //   - Height is not already known
        //   - Item has both top and bottom inset properties set
        if let (None, Some(top), Some(bottom)) = (known_dimensions.height, top, bottom) {
            let new_height_raw = area_height.maybe_sub(margin.top).maybe_sub(margin.bottom) - top - bottom;
            known_dimensions.height = Some(f32_max(new_height_raw, 0.0));
            known_dimensions = known_dimensions.maybe_apply_aspect_ratio(aspect_ratio).maybe_clamp(min_size, max_size);
        }

        let measured_size_and_baselines = tree.perform_child_layout(
            item.node_id,
            known_dimensions,
            area_size.map(Some),
            Size {
                width: AvailableSpace::Definite(area_width.maybe_clamp(min_size.width, max_size.width)),
                height: AvailableSpace::Definite(area_height.maybe_clamp(min_size.height, max_size.height)),
            },
            SizingMode::ContentSize,
        );
        let measured_size = measured_size_and_baselines.size;
        let final_size = known_dimensions.unwrap_or(measured_size).maybe_clamp(min_size, max_size);

        let non_auto_margin = margin.map(|m| m.unwrap_or(0.0));

        let free_space = Size {
            width: area_size.width - final_size.width - non_auto_margin.horizontal_axis_sum(),
            height: area_size.height - final_size.height - non_auto_margin.vertical_axis_sum(),
        }
        .f32_max(Size::ZERO);

        // Expand auto margins to fill available space
        // https://www.w3.org/TR/CSS21/visudet.html#abs-non-replaced-width
        let resolved_margin = {
            let auto_margin_size = Size {
                // If all three of 'left', 'width', and 'right' are 'auto': First set any 'auto' values for 'margin-left' and 'margin-right' to 0.
                // Then, if the 'direction' property of the element establishing the static-position containing block is 'ltr' set 'left' to the
                // static position and apply rule number three below; otherwise, set 'right' to the static position and apply rule number one below.
                //
                // If none of the three is 'auto': If both 'margin-left' and 'margin-right' are 'auto', solve the equation under the extra constraint
                // that the two margins get equal values, unless this would make them negative, in which case when direction of the containing block is
                // 'ltr' ('rtl'), set 'margin-left' ('margin-right') to zero and solve for 'margin-right' ('margin-left'). If one of 'margin-left' or
                // 'margin-right' is 'auto', solve the equation for that value. If the values are over-constrained, ignore the value for 'left' (in case
                // the 'direction' property of the containing block is 'rtl') or 'right' (in case 'direction' is 'ltr') and solve for that value.
                width: {
                    let auto_margin_count = margin.left.is_none() as u8 + margin.right.is_none() as u8;
                    if auto_margin_count == 2 && style_size.width.is_none() {
                        0.0
                    } else if auto_margin_count > 0 {
                        free_space.width / auto_margin_count as f32
                    } else {
                        0.0
                    }
                },
                height: {
                    let auto_margin_count = margin.top.is_none() as u8 + margin.bottom.is_none() as u8;
                    if auto_margin_count == 2 && style_size.height.is_none() {
                        0.0
                    } else if auto_margin_count > 0 {
                        free_space.height / auto_margin_count as f32
                    } else {
                        0.0
                    }
                },
            };

            Rect {
                left: margin.left.unwrap_or(auto_margin_size.width),
                right: margin.right.unwrap_or(auto_margin_size.width),
                top: margin.top.unwrap_or(auto_margin_size.height),
                bottom: margin.bottom.unwrap_or(auto_margin_size.height),
            }
        };

        let item_offset = Point {
            x: left.or(right.map(|x| -x)).unwrap_or(0.0) + resolved_margin.left,
            y: top.or(bottom.map(|x| -x)).unwrap_or(item.static_position.y) + resolved_margin.top,
        };

        *tree.layout_mut(item.node_id) =
            Layout { order: item.order, size: final_size, location: area_offset + item_offset };
    }
}