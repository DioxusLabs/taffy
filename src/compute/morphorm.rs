use crate::compute::LayoutAlgorithm;
use crate::geometry::{Line, Point, Rect, Size};
use crate::style::{
    AlignContent, AlignItems, AlignSelf, AvailableSpace, Dimension, Display, FlexWrap, JustifyContent,
    LengthPercentageAuto, Position,
};
use crate::style::{FlexDirection, Style};
use crate::tree::LayoutTree;
use crate::tree::NodeId;
use crate::util::{MaybeMath, MaybeResolve};
use crate::tree::{Layout, RunMode, SizeBaselinesAndMargins, SizingMode};

/// The public interface to Taffy's Flexbox algorithm implementation
pub struct MorphormAlgorithm;
impl LayoutAlgorithm for MorphormAlgorithm {
    const NAME: &'static str = "MORPHORM";

    fn perform_layout(
        tree: &mut impl LayoutTree,
        node: NodeId,
        known_dimensions: Size<Option<f32>>,
        parent_size: Size<Option<f32>>,
        available_space: Size<AvailableSpace>,
        _sizing_mode: SizingMode,
        _vertical_margins_are_collapsible: Line<bool>,
    ) -> SizeBaselinesAndMargins {
        compute(tree, node, known_dimensions, parent_size, available_space, RunMode::PerformLayout)
    }

    fn measure_size(
        tree: &mut impl LayoutTree,
        node: NodeId,
        known_dimensions: Size<Option<f32>>,
        parent_size: Size<Option<f32>>,
        available_space: Size<AvailableSpace>,
        _sizing_mode: SizingMode,
        _vertical_margins_are_collapsible: Line<bool>,
    ) -> Size<f32> {
        compute(tree, node, known_dimensions, parent_size, available_space, RunMode::ComputeSize).size
    }
}

#[derive(Debug)]
pub struct ChildNode {
    // A reference to the node.
    node: NodeId,
    // Whether the child is in flow or not
    is_in_flow: bool,
    is_first: bool,
    is_last: bool,
    // The index of the node.
    index: usize,

    style: Style,

    size: Size<f32>,

    spacing: Rect<f32>,

    // // The stretch factor of the node.
    // stretch_factor: Size<f32>,
    // // The minimum constraint of the node.
    // min: Size<Option<f32>>,
    // // The maximum constraint of the node.
    // max: Size<Option<f32>>,

    // // Sum of the flex factors on the main axis of the node.
    // main_flex_sum: f32,
    // // The available free space on the main axis of the node.
    // main_non_flex: f32,
    // // A remainder used during stretch computation.
    // main_remainder: f32,
    // // Sum of the cross_before, cross, and cross_after flex factors of the node.
    // cross_flex_sum: f32,

    // cross_non_flex: f32,
    // cross: f32,
    // cross_remainder: f32,

    // // Computed main-before space of the node.
    // main_before: f32,
    // // Computed main-after space of the node.
    // main_after: f32,
    // // Computed cross-before space of the node.
    // cross_before: f32,
    // // Computed cross-after space of the node.
    // cross_after: f32,
}

/// Computes the layout of [`LayoutTree`] according to the flexbox algorithm
pub fn compute(
    tree: &mut impl LayoutTree,
    node: NodeId,
    known_dimensions: Size<Option<f32>>,
    parent_size: Size<Option<f32>>,
    available_space: Size<AvailableSpace>,
    run_mode: RunMode,
) -> SizeBaselinesAndMargins {
    // TODO: Min/Max constraints for space and size

    // The layout type of the node. Determines the main and cross axes of the children.
    let container_style = tree.style(node).clone();
    let dir = container_style.flex_direction;
    let is_row = dir.is_row();

    let min_size = container_style.min_size.maybe_resolve(parent_size);
    let max_size = container_style.max_size.maybe_resolve(parent_size);
    let clamped_style_size = container_style.size.maybe_resolve(parent_size).maybe_clamp(min_size, max_size);

    // If both min and max in a given axis are set and max <= min then this determines the size in that axis
    let min_max_definite_size = min_size.zip_map(max_size, |min, max| match (min, max) {
        (Some(min), Some(max)) if max <= min => Some(min),
        _ => None,
    });
    let styled_based_known_dimensions = known_dimensions.or(min_max_definite_size).or(clamped_style_size);

    // Short-circuit layout if the container's size is fully determined by the container's size and the run mode
    // is ComputeSize (and thus the container's size is all that we're interested in)
    if run_mode == RunMode::ComputeSize {
        if let Size { width: Some(width), height: Some(height) } = styled_based_known_dimensions {
            return Size { width, height }.into();
        }
    }

    let in_flow_children: Vec<ChildNode> = tree
        .children(node)
        .map(|child| (child, tree.style(*child)))
        .filter(|(_, style)| style.position != Position::Absolute)
        .filter(|(_, style)| style.display != Display::None)
        .enumerate()
        .map(|(index, (child, child_style))| {
            ChildNode {
                node,
                is_first: false, // Set later
                is_last: false, // Set later
                is_in_flow: child_style.position != Position::Absolute,
                index,
                style: child_style.clone(),
                size: Size::zero(), // Set later
                spacing: Rect::zero(), // Set later
            }
        })
        .collect();

    // Set is_first and is_last properties on the first and last children
    let in_flow_child_count = in_flow_children.len();
    in_flow_children[0].is_first = true;
    in_flow_children[in_flow_children.len() - 1].is_last = true;


    // Compute size and spacing for each child
    let space_around = container_style.spacing_around.resolve_or_zero(parent_size);
    let space_between = container_style.spacing_between.resolve_or_zero(parent_size);
    for child in in_flow_children.iter_mut() {
      let child_min_size = child.style.min_size.maybe_resolve(parent_size);
      let max_size = child.style.max_size.maybe_resolve(parent_size);
      let clamped_style_size = child.style.size.maybe_resolve(parent_size).maybe_clamp(min_size, max_size);

      // If both min and max in a given axis are set and max <= min then this determines the size in that axis
      let min_max_definite_size = min_size.zip_map(max_size, |min, max| match (min, max) {
          (Some(min), Some(max)) if max <= min => Some(min),
          _ => None,
      });
      let child_style_based_known_dimensions = min_max_definite_size.or(clamped_style_size);

      // Compute content size if size is not already known and the item has an "auto" dimension
      let has_auto_dimension =
          child.style.size.width == Dimension::Auto || child.style.size.height == Dimension::Auto;
      let content_size: Size<Option<f32>> =
          if !child_style_based_known_dimensions.both_axis_defined() && has_auto_dimension {
              GenericAlgorithm::measure_size(
                  tree,
                  node,
                  known_dimensions,
                  parent_size,
                  available_space,
                  SizingMode::InherentSize,
              )
              .map(Some)
          } else {
              child_style_based_known_dimensions
          };

      child.size = child_style_based_known_dimensions.or(content_size).unwrap_or(Size::zero()).maybe_clamp(min_size, max_size);

      let default_spacing = {
        let mut space = space_around;
        if !child.is_first {
          space.set_main_start(dir, space_between.main(dir));
        }
        if !child.is_last {
          space.set_main_end(dir, 0.0);
        }
      };

      let resolved_margin = child.style.margin.map(|m| m.resolve_to_option(0.0));
      let outer_spacing = resolved_margin.unwrap_or(space_around);

    }

    // Compute space used by the content size of each no in each axis
    let main_axis_used_space: f32 =
        in_flow_children.iter().map(|child| child.size.main(dir) + child.spacing.main_axis_sum(dir)).sum();
    let cross_axis_used_space: f32 = in_flow_children
        .iter()
        .map(|child| child.size.cross(dir) + child.spacing.cross_axis_sum(dir))
        .max_by(|a, b| a.total_cmp(b))
        .unwrap_or(0.0);
    let used_space = {
        let mut space = Size::zero();
        space.set_main(dir, main_axis_used_space);
        space.set_cross(dir, cross_axis_used_space);
        space
    };
    let container_size = styled_based_known_dimensions.unwrap_or(used_space);

    // Return early if we're only computing container size
    if run_mode == RunMode::ComputeSize {
        return container_size.into();
    }

    let free_space = container_size - used_space;
    
    // TODO perform stretch sizing
    // TODO position nodes


    // Sum of all non-flexible space and size on the main-axis of the node.
    let mut main_non_flex = 0.0f32;

    // Sum of all space and size flex factors on the main-axis of the node.
    let mut main_flex_sum = 0.0;

    // Sum of all child nodes on the main-axis.
    let mut main_sum = 0.0f32;

    // Maximum of all child nodes on the cross-axis.
    let mut cross_max = 0.0f32;

    // List of child nodes for the current node.
    let mut children = SmallVec::<[ChildNode<N>; 3]>::new();

    // List of stretch nodes for the current node.
    // A stretch node is any flexible space/size. e.g. main_before, main, and main_after are separate stretch nodes
    let mut stretch_nodes = SmallVec::<[StretchNode<N>; 3]>::new();

    // Parent overrides for child auto space.
    let node_child_main_before = node.child_main_before(store, layout_type);
    let node_child_main_after = node.child_main_after(store, layout_type);
    let node_child_cross_before = node.child_cross_before(store, layout_type);
    let node_child_cross_after = node.child_cross_after(store, layout_type);
    let node_child_main_between = node.main_between(store, layout_type);

    // Determine index of first and last parent-directed child nodes.
    let mut iter = node
        .children(tree)
        .enumerate()
        .filter(|(_, child)| child.position_type(store).unwrap_or_default() == PositionType::ParentDirected);

    let first = iter.next().map(|(index, _)| index);
    let last = iter.last().map_or(first, |(index, _)| Some(index));

    let num_children = node.children(tree).count();

    // Compute non-flexible children.
    for (index, child) in node.children(tree).enumerate() {
        let child_position_type = child.position_type(store).unwrap_or_default();

        let mut child_main_before = child.main_before(store, layout_type);
        let child_main = child.main(store, layout_type);
        let mut child_main_after = child.main_after(store, layout_type);

        let mut child_cross_before = child.cross_before(store, layout_type);
        let child_cross = child.cross(store, layout_type);
        let mut child_cross_after = child.cross_after(store, layout_type);

        // Apply parent overrides to auto child space.
        if child_main_before == Units::Auto {
            if first == Some(index) || child_position_type == PositionType::SelfDirected {
                child_main_before = node_child_main_before;
            } else {
                child_main_before = node_child_main_between;
            }
        }

        if child_main_after == Units::Auto && (last == Some(index) || child_position_type == PositionType::SelfDirected)
        {
            child_main_after = node_child_main_after;
        }

        if child_cross_before == Units::Auto {
            child_cross_before = node_child_cross_before;
        }

        if child_cross_after == Units::Auto {
            child_cross_after = node_child_cross_after;
        }

        // Sum of flex factors on the main and cross axes of the child node.
        let mut child_main_flex_sum = 0.0;
        let mut child_cross_flex_sum = 0.0;

        let mut computed_child_main_before = 0.0;
        let mut computed_child_main = 0.0;
        let mut computed_child_main_after = 0.0;

        let mut computed_child_cross_before = 0.0;
        let mut computed_child_cross = 0.0;
        let mut computed_child_cross_after = 0.0;

        match child_cross_before {
            Pixels(val) => {
                computed_child_cross_before = val;
            }

            Percentage(val) => {
                computed_child_cross_before = (parent_cross * (val / 100.0)).round();
            }

            Stretch(factor) => {
                child_cross_flex_sum += factor;
            }

            _ => {}
        }

        let child_min_cross_before = child.min_cross_before(store, layout_type).to_px(parent_cross, DEFAULT_MIN);
        let child_max_cross_before = child.max_cross_before(store, layout_type).to_px(parent_cross, DEFAULT_MAX);

        computed_child_cross_before = computed_child_cross_before.clamp(child_min_cross_before, child_max_cross_before);

        match child_cross_after {
            Pixels(val) => {
                computed_child_cross_after = val;
            }

            Percentage(val) => {
                computed_child_cross_after = (parent_cross * (val / 100.0)).round();
            }

            Stretch(factor) => {
                child_cross_flex_sum += factor;
            }

            _ => {}
        }

        let child_min_cross_after = child.min_cross_after(store, layout_type).to_px(parent_cross, DEFAULT_MIN);
        let child_max_cross_after = child.max_cross_after(store, layout_type).to_px(parent_cross, DEFAULT_MAX);

        computed_child_cross_after = computed_child_cross_after.clamp(child_min_cross_after, child_max_cross_after);

        match child_cross {
            Pixels(val) => {
                computed_child_cross = val;
            }

            Percentage(val) => {
                computed_child_cross = (parent_cross * (val / 100.0)).round();
            }

            Stretch(factor) => {
                child_cross_flex_sum += factor;
            }

            _ => {}
        }

        let child_min_cross = child.min_cross(store, layout_type).to_px(parent_cross, DEFAULT_MIN);
        let child_max_cross = child.max_cross(store, layout_type).to_px(parent_cross, DEFAULT_MAX);

        computed_child_cross = computed_child_cross.clamp(child_min_cross, child_max_cross);

        let child_min_main_before = child.min_main_before(store, layout_type).to_px(parent_main, DEFAULT_MIN);
        let child_max_main_before = child.max_main_before(store, layout_type).to_px(parent_main, DEFAULT_MAX);

        match child_main_before {
            Pixels(val) => {
                computed_child_main_before = val;
            }

            Percentage(val) => {
                computed_child_main_before = (parent_main * (val / 100.0)).round();
            }

            Stretch(factor) => {
                child_main_flex_sum += factor;

                stretch_nodes.push(StretchNode {
                    node: child,
                    index,
                    factor,
                    min: child_min_main_before,
                    max: child_max_main_before,
                    axis: Axis::MainBefore,
                });
            }

            _ => {}
        }

        computed_child_main_before = computed_child_main_before.clamp(child_min_main_before, child_max_main_before);

        let child_min_main_after = child.min_main_after(store, layout_type).to_px(parent_main, DEFAULT_MIN);
        let child_max_main_after = child.max_main_after(store, layout_type).to_px(parent_main, DEFAULT_MAX);

        match child_main_after {
            Pixels(val) => {
                computed_child_main_after = val;
            }

            Percentage(val) => {
                computed_child_main_after = (parent_main * (val / 100.0)).round();
            }

            Stretch(factor) => {
                child_main_flex_sum += factor;

                stretch_nodes.push(StretchNode {
                    node: child,
                    index,
                    factor,
                    min: child_min_main_after,
                    max: child_max_main_after,
                    axis: Axis::MainAfter,
                });
            }

            _ => {}
        }

        computed_child_main_after = computed_child_main_after.clamp(child_min_main_after, child_max_main_after);

        // Total computed size on the cross-axis of the child.
        let mut child_cross_non_flex = computed_child_cross_before + computed_child_cross + computed_child_cross_after;

        match child_main {
            Stretch(factor) => {
                child_main_flex_sum += factor;

                stretch_nodes.push(StretchNode {
                    node: child,
                    index,
                    factor,
                    min: DEFAULT_MIN,
                    max: DEFAULT_MAX,
                    axis: Axis::Main,
                });
            }

            _ => {
                let child_size = layout(child, layout_type, parent_main, computed_child_cross, cache, tree, store);

                computed_child_main = child_size.main;
                computed_child_cross = child_size.cross;

                if child_cross == Units::Auto {
                    child_cross_non_flex += computed_child_cross;
                }
            }
        }

        // Total computed size on the main-axis of the child.
        let child_main_non_flex = computed_child_main_before + computed_child_main + computed_child_main_after;

        if child_position_type == PositionType::ParentDirected {
            main_non_flex += child_main_non_flex;
            main_flex_sum += child_main_flex_sum;

            main_sum += child_main_non_flex;
        } else {
            main_sum = main_sum.max(child_main_non_flex);
        }

        cross_max = cross_max.max(child_cross_non_flex);

        children.push(ChildNode {
            node: child,
            main_flex_sum: child_main_flex_sum,
            main_non_flex: child_main_non_flex,
            main_remainder: 0.0,
            cross_flex_sum: child_cross_flex_sum,
            cross_non_flex: child_cross_non_flex,
            cross: computed_child_cross,
            cross_remainder: 0.0,
            main_before: computed_child_main_before,
            main_after: computed_child_main_after,
            cross_before: computed_child_cross_before,
            cross_after: computed_child_cross_after,
        });
    }

    // Calculate free space on the main-axis for the node.
    let free_main_space = (parent_main.max(main_sum) - main_non_flex).max(0.0);
    let mut remainder: f32 = 0.0;
    let main_px_per_flex = free_main_space / main_flex_sum;

    // Compute flexible space and size on the main axis.
    for item in stretch_nodes.iter() {
        let child_position_type = item.node.position_type(store).unwrap_or_default();

        let actual_main = if child_position_type == PositionType::SelfDirected {
            let child_main_free_space = (parent_main.max(main_sum) - children[item.index].main_non_flex).max(0.0);
            let px_per_flex = child_main_free_space / children[item.index].main_flex_sum;
            let desired_main = item.factor * px_per_flex + children[item.index].main_remainder;
            let actual_main = desired_main.round();
            children[item.index].main_remainder = desired_main - actual_main;
            actual_main
        } else {
            let desired_main = item.factor * main_px_per_flex + remainder;
            let actual_main = desired_main.round();
            remainder = desired_main - actual_main;
            actual_main
        };

        let computed_child_cross = children[item.index].cross;

        match item.axis {
            Axis::MainBefore => {
                children[item.index].main_before = actual_main;
            }

            Axis::MainAfter => {
                children[item.index].main_after = actual_main;
            }

            Axis::Main => {
                let child_size = layout(item.node, layout_type, actual_main, computed_child_cross, cache, tree, store);

                children[item.index].cross_non_flex += child_size.cross;
                cross_max = cross_max.max(children[item.index].cross_non_flex);
            }
        }
    }

    // Compute flexible space and size on the cross-axis.
    // This needs to be done after computing the main-axis because layout computation for stretch children may cause
    // the cross space to change due to content size.
    for child in children.iter_mut() {
        let child_cross_free_space = parent_cross.max(cross_max) - child.cross_non_flex;
        let cross_px_per_flex = child_cross_free_space / child.cross_flex_sum;

        let child_cross_before = child.node.cross_before(store, layout_type);
        let child_cross = child.node.cross(store, layout_type);
        let child_cross_after = child.node.cross_after(store, layout_type);

        if let Stretch(factor) = child_cross_before {
            let desired_cross = factor * cross_px_per_flex + child.cross_remainder;
            let actual_cross = desired_cross.round();
            child.cross_remainder = desired_cross - actual_cross;
            child.cross_before = actual_cross;
        }

        if let Stretch(factor) = child_cross {
            let desired_cross = factor * cross_px_per_flex + child.cross_remainder;
            let actual_cross = desired_cross.round();
            child.cross_remainder = desired_cross - actual_cross;

            // if cross == Units::Auto && num_children != 0 {
            let size = layout(child.node, layout_type, parent_main, actual_cross, cache, tree, store);
            //     println!("size: {:?}", size);
            // } else {
            // At this stage stretch nodes on the cross-axis can only be the determined size so we can set it directly
            // in the cache without needing to call layout again.
            // match layout_type {
            //     LayoutType::Row => {
            //         cache.set_height(child.node.key(), actual_cross);
            //     }

            //     LayoutType::Column => {
            //         cache.set_width(child.node.key(), actual_cross);
            //     }
            // }
            // }
        }

        if let Stretch(factor) = child_cross_after {
            let desired_cross = factor * cross_px_per_flex + child.cross_remainder;
            let actual_cross = desired_cross.round();
            child.cross_remainder = desired_cross - actual_cross;
            child.cross_after = actual_cross;
        }
    }

    // Position children.
    let mut main_pos = 0.0;
    for child in children.iter() {
        let child_position_type = child.node.position_type(store).unwrap_or_default();

        match child_position_type {
            PositionType::SelfDirected => match layout_type {
                LayoutType::Row => {
                    cache.set_posx(child.node.key(), child.main_before);
                    cache.set_posy(child.node.key(), child.cross_before);
                }

                LayoutType::Column => {
                    cache.set_posy(child.node.key(), child.main_before);
                    cache.set_posx(child.node.key(), child.cross_before);
                }
            },

            PositionType::ParentDirected => {
                main_pos += child.main_before;

                match layout_type {
                    LayoutType::Row => {
                        cache.set_posx(child.node.key(), main_pos);
                        cache.set_posy(child.node.key(), child.cross_before);
                        let child_width = cache.width(child.node.key());
                        main_pos += child_width;
                    }

                    LayoutType::Column => {
                        cache.set_posy(child.node.key(), main_pos);
                        cache.set_posx(child.node.key(), child.cross_before);
                        let child_height = cache.height(child.node.key());
                        main_pos += child_height;
                    }
                }

                main_pos += child.main_after;
            }
        };
    }

    // This part is required for auto size when the node has children but conflicts with the content size when the node doesn't have children
    // TODO: Make it so a node can only have content size if it has no children?
    // TODO: Potentially split and move this to before stretch calculations.
    if num_children != 0 {
        if parent_layout_type == layout_type {
            if let Auto = main {
                computed_main = main_sum;
            }

            if let Auto = cross {
                computed_cross = cross_max;
            }
        } else {
            if let Auto = main {
                computed_main = cross_max;
            }

            if let Auto = cross {
                computed_cross = main_sum;
            }
        }
    }

    // Set the computed size of the node in the cache.
    match parent_layout_type {
        LayoutType::Row => {
            cache.set_width(node.key(), computed_main);
            cache.set_height(node.key(), computed_cross);
        }

        LayoutType::Column => {
            cache.set_height(node.key(), computed_main);
            cache.set_width(node.key(), computed_cross);
        }
    }

    // Return the computed size, propagating it back up the tree.
    Size { main: computed_main, cross: computed_cross }.into()
}
