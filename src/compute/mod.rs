pub(crate) mod flexbox;
pub(crate) mod leaf;

use core::sync::atomic::AtomicU16;
use std::env::consts;
use slotmap::Key;

use crate::error::TaffyError;
use crate::geometry::{Point, Size};
use crate::layout::{AvailableSpace, Cache, Layout, RunMode, SizingMode};
use crate::math::MaybeMath;
use crate::node::Node;
use crate::resolve::MaybeResolve;
use crate::style::Display;
use crate::sys::round;
use crate::tree::LayoutTree;
use crate::debug::NODE_LOGGER;

/// Updates the stored layout of the provided `node` and its children
pub fn compute_layout(
    tree: &mut impl LayoutTree,
    root: Node,
    available_space: Size<AvailableSpace>,
) -> Result<(), TaffyError> {
    // Recursively compute node layout
    let size = compute_node_layout(tree, root, Size::NONE, available_space, RunMode::PeformLayout, SizingMode::InherentSize, 0);

    let layout = Layout { order: 0, size, location: Point::ZERO };
    *tree.layout_mut(root) = layout;

    // Recursively round the layout's of this node and all children
    round_layout(tree, root, 0.0, 0.0);

    Ok(())
}

/// Updates the stored layout of the provided `node` and its children
fn compute_node_layout(
    tree: &mut impl LayoutTree,
    node: Node,
    known_dimensions: Size<Option<f32>>,
    available_space: Size<AvailableSpace>,
    run_mode: RunMode,
    sizing_mode: SizingMode,
    cache_slot: usize,
) -> Size<f32> {
    // clear the dirtiness of the node now that we've computed it
    tree.mark_dirty(node, false);

    NODE_LOGGER.push_node(node);
    println!("");
    

    // First we check if we have a cached result for the given input
    if let Some(cached_size) = compute_from_cache(tree, node, known_dimensions, available_space) {
        NODE_LOGGER.debug_llog("CACHE", cached_size);
        NODE_LOGGER.pop_node();
        return cached_size;
    }

    NODE_LOGGER.log("COMPUTE");
    NODE_LOGGER.debug_llog("run_mode", run_mode);
    NODE_LOGGER.debug_llog("sizing_mode", sizing_mode);
    NODE_LOGGER.debug_llog("known_dimensions", known_dimensions);
    NODE_LOGGER.debug_llog("available_space", available_space);

    // Attempt to shortcut size computation based on
    //  - KnownSize sizing constraints
    //  - The node's preferred sizes (width/heights) styles and AvailableSpace sizing constraints
    // (percentages resolve to pixel values if there is a definite AvailableSpace sizing constraint)
    // let style = tree.style(node);
    // // let known_node_size = available_space.
    // //     .zip_map(style.size, |constraint, style_size| style_size.maybe_resolve(constraint.as_option()));
    // let known_node_size = style
    //     .size
    //     .maybe_resolve(available_space.as_options())
    //     .zip_map(known_dimensions, |style_size, known_dimensions| known_dimensions.or(style_size));
    // if run_mode == RunMode::ComputeSize && known_node_size.width.is_some() && known_node_size.height.is_some() {
    //     let node_min_size = style.min_size.maybe_resolve(available_space.as_options());
    //     let node_max_size = style.max_size.maybe_resolve(available_space.as_options());
    //     return Size {
    //         width: known_node_size.width.maybe_max(node_min_size.width).maybe_min(node_max_size.width).unwrap_or(0.0),
    //         height: known_node_size
    //             .height
    //             .maybe_max(node_min_size.height)
    //             .maybe_min(node_max_size.height)
    //             .unwrap_or(0.0),
    //     };
    // }

    // If this is a leaf node we can skip a lot of this function in some cases
    let computed_size = if tree.children(node).is_empty() {
        NODE_LOGGER.log("Algo: leaf");
        self::leaf::compute(tree, node, known_dimensions, available_space, run_mode, sizing_mode)
    } else {
        // println!("match {:?}", tree.style(node).display);
        match tree.style(node).display {
            Display::Flex => {
              NODE_LOGGER.log("Algo: flexbox");
              self::flexbox::compute(tree, node, known_dimensions, available_space, run_mode, cache_slot)
            },
            Display::None => {
              NODE_LOGGER.log("Algo: none");
              Size { width: 0.0, height: 0.0 }
            },
        }
    };

    // Cache result
    *tree.cache_mut(node, cache_slot) =
        Some(Cache { known_dimensions, available_space, run_mode, cached_size: computed_size });

    NODE_LOGGER.debug_llog("RESULT", computed_size);
    NODE_LOGGER.pop_node();

    computed_size
}

/// Try to get the computation result from the cache.
#[inline]
fn compute_from_cache(
    tree: &mut impl LayoutTree,
    node: Node,
    known_dimensions: Size<Option<f32>>,
    available_space: Size<AvailableSpace>,
) -> Option<Size<f32>> {
    for idx in 0..4 {
        let entry = tree.cache_mut(node, idx);
        // NODE_LOGGER.debug_llog("cache_entry", &entry);
        if let Some(entry) = entry {
            if known_dimensions.width == entry.known_dimensions.width
                && known_dimensions.height == entry.known_dimensions.height
            // if (known_dimensions.width == entry.known_dimensions.width || known_dimensions.width == Some(entry.cached_size.width))
                // && (known_dimensions.height == entry.known_dimensions.height || known_dimensions.height == Some(entry.cached_size.height))
                && entry.available_space.width.is_roughly_equal(available_space.width)
                && entry.available_space.height.is_roughly_equal(available_space.height)
                // && (known_dimensions.width.is_some() || entry.available_space.width.is_roughly_equal(available_space.width))
                // && (known_dimensions.height.is_some() || entry.available_space.height.is_roughly_equal(available_space.height))
                // && (entry.available_space.width.is_roughly_equal(available_space.width) || (available_space.width.is_definite() && available_space.width.unwrap() >= entry.cached_size.width))
                // && (entry.available_space.height.is_roughly_equal(available_space.height) || (available_space.height.is_definite() && available_space.height.unwrap() >= entry.cached_size.height))
            {
                return Some(entry.cached_size);
            }
        }
    }

    None
}

/// Rounds the calculated [`NodeData`] according to the spec
fn round_layout(tree: &mut impl LayoutTree, root: Node, abs_x: f32, abs_y: f32) {
    let layout = tree.layout_mut(root);
    let abs_x = abs_x + layout.location.x;
    let abs_y = abs_y + layout.location.y;

    layout.location.x = round(layout.location.x);
    layout.location.y = round(layout.location.y);

    layout.size.width = round(layout.size.width);
    layout.size.height = round(layout.size.height);

    // Satisfy the borrow checker here by re-indexing to shorten the lifetime to the loop scope
    for x in 0..tree.children(root).len() {
        let child = tree.child(root, x);
        round_layout(tree, child, abs_x, abs_y);
    }
}
