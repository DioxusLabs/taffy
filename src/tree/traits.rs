//! The abstractions that make up the core of Taffy's low-level API
//!
//! ## Examples
//!
//! The following examples demonstrate end-to-end implementation of Taffy's traits and usage of the low-level compute APIs:
//!
//!   - [custom_tree_vec](https://github.com/DioxusLabs/taffy/blob/main/examples/custom_tree_vec.rs) which implements a custom Taffy tree using a `Vec` as an arena with NodeId's being index's into the Vec.
//!   - [custom_tree_owned_partial](https://github.com/DioxusLabs/taffy/blob/main/examples/custom_tree_owned_partial.rs) which implements a custom Taffy tree using directly owned children with NodeId's being index's into vec on parent node.
//!   - [custom_tree_owned_unsafe](https://github.com/DioxusLabs/taffy/blob/main/examples/custom_tree_owned_unsafe.rs) which implements a custom Taffy tree using directly owned children with NodeId's being pointers.
//!
//! ## Overview
//!
//! ### Trait dependency tree
//!
//! The tree below illustrates which traits depend on which other traits.
//!
//! ```text
//! TraversePartialTree     - Access a node's children
//! ├──  LayoutPartialTree  - Run layout algorithms on a node and it's direct children
//! └──  TraverseTree       - Recursively access a node's descendants
//!     ├──  RoundTree      - Round a float-valued`  layout to integer pixels
//!     └──  PrintTree      - Print a debug representation of a node tree
//! ```
//!
//! ### A table of traits
//!
//! | Trait                 | Requires                | Enables                                                                                                                                                                                                                                                                                                                                                                                                                   |
//! | ---                   | ---                     | ---                                                                                                                                                                                                                                                                                                                                                                                                                       |
//! | [`LayoutPartialTree`] | [`TraversePartialTree`] | [`compute_flexbox_layout`](crate::compute_flexbox_layout)<br />[`compute_grid_layout`](crate::compute_grid_layout)<br />[`compute_block_layout`](crate::compute_block_layout)<br />[`compute_root_layout`](crate::compute_root_layout)<br />[`compute_leaf_layout`](crate::compute_leaf_layout)<br />[`compute_hidden_layout`](crate::compute_hidden_layout)<br />[`compute_cached_layout`](crate::compute_cached_layout) |
//! | [`RoundTree`]         | [`TraverseTree`]        | [`round_layout`](crate::round_layout)                                                                                                                                                                                                                                                                                                                                                                                     |
//! | [`PrintTree`]         | [`TraverseTree`]        | [`print_tree`](crate::print_tree)                                                                                                                                                                                                                                                                                                                                                                                         |
//!
//! ## All of the traits on one page
//!
//! ### TraversePartialTree and TraverseTree
//! These traits are Taffy's abstraction for downward tree traversal:
//!  - [`TraversePartialTree`] allows access to a single container node, and it's immediate children. This is the only "traverse" trait that is required
//!    for use of Taffy's core layout algorithms (flexbox, grid, etc).
//!  - [`TraverseTree`] is a marker trait which uses the same API signature as `TraversePartialTree`, but extends it with a guarantee that the child/children methods can be used to recurse
//!    infinitely down the tree. It is required by the `RoundTree` and
//!    the `PrintTree` traits.
//! ```rust
//! # use taffy::*;
//! pub trait TraversePartialTree {
//!     /// Type representing an iterator of the children of a node
//!     type ChildIter<'a>: Iterator<Item = NodeId>
//!     where
//!         Self: 'a;
//!
//!     /// Get the list of children IDs for the given node
//!     fn child_ids(&self, parent_node_id: NodeId) -> Self::ChildIter<'_>;
//!
//!     /// Get the number of children for the given node
//!     fn child_count(&self, parent_node_id: NodeId) -> usize;
//!
//!     /// Get a specific child of a node, where the index represents the nth child
//!     fn get_child_id(&self, parent_node_id: NodeId, child_index: usize) -> NodeId;
//! }
//!
//! pub trait TraverseTree: TraversePartialTree {}
//! ```
//!
//! You must implement [`TraversePartialTree`] to access any of Taffy's low-level API. If your tree implementation allows you to implement [`TraverseTree`] with
//! the correct semantics (full recursive traversal is available) then you should.
//!
//! ### LayoutPartialTree
//!
//! **Requires:** `TraversePartialTree`<br />
//! **Enables:** Flexbox, Grid, Block and Leaf layout algorithms from the [`crate::compute`] module
//!
//! Any type that implements [`LayoutPartialTree`] can be laid out using [Taffy's algorithms](crate::compute)
//!
//! Note that this trait extends [`TraversePartialTree`] (not [`TraverseTree`]). Taffy's algorithm implementations have been designed such that they can be used for a laying out a single
//! node that only has access to it's immediate children.
//!
//! ```rust
//! # use taffy::*;
//! pub trait LayoutPartialTree: TraversePartialTree {
//!     /// Get a reference to the [`Style`] for this node.
//!     fn get_style(&self, node_id: NodeId) -> &Style;
//!
//!     /// Set the node's unrounded layout
//!     fn set_unrounded_layout(&mut self, node_id: NodeId, layout: &Layout);
//!
//!     /// Get a mutable reference to the [`Cache`] for this node.
//!     fn get_cache_mut(&mut self, node_id: NodeId) -> &mut Cache;
//!
//!     /// Compute the specified node's size or full layout given the specified constraints
//!     fn compute_child_layout(&mut self, node_id: NodeId, inputs: LayoutInput) -> LayoutOutput;
//! }
//! ```
//!
//! ### RoundTree
//!
//! **Requires:** `TraverseTree`
//!
//! Trait used by the `round_layout` method which takes a tree of unrounded float-valued layouts and performs
//! rounding to snap the values to the pixel grid.
//!
//! As indicated by it's dependence on `TraverseTree`, it required full recursive access to the tree.
//!
//! ```rust
//! # use taffy::*;
//! pub trait RoundTree: TraverseTree {
//!     /// Get the node's unrounded layout
//!     fn get_unrounded_layout(&self, node_id: NodeId) -> Layout;
//!     /// Get a reference to the node's final layout
//!     fn set_final_layout(&mut self, node_id: NodeId, layout: &Layout);
//! }
//! ```
//!
//! ### PrintTree
//!
//! **Requires:** `TraverseTree`
//!
//! ```rust
//! /// Trait used by the `print_tree` method which prints a debug representation
//! ///
//! /// As indicated by it's dependence on `TraverseTree`, it required full recursive access to the tree.
//! # use taffy::*;
//! pub trait PrintTree: TraverseTree {
//!     /// Get a debug label for the node (typically the type of node: flexbox, grid, text, image, etc)
//!     fn get_debug_label(&self, node_id: NodeId) -> &'static str;
//!     /// Get a reference to the node's final layout
//!     fn get_final_layout(&self, node_id: NodeId) -> Layout;
//! }
//! ```
//!
use super::{DetailedLayoutInfo, Layout, LayoutInput, LayoutOutput, NodeId, RequestedAxis, RunMode};
use crate::debug::debug_log;
use crate::geometry::{AbsoluteAxis, Line, Size};
use crate::style::{AvailableSpace, CoreStyle, Dimension, OofItemStyle};
#[cfg(feature = "flexbox")]
use crate::style::{FlexboxContainerStyle, FlexboxItemStyle};
#[cfg(feature = "grid")]
use crate::style::{GridContainerStyle, GridItemStyle};
use crate::CheapCloneStr;
#[cfg(feature = "block_layout")]
use crate::{BlockContainerStyle, BlockContext, BlockItemStyle};

#[cfg(feature = "grid")]
use crate::compute::grid::DetailedGridInfo;

/// Taffy's abstraction for downward tree traversal.
///
/// However, this trait does *not* require access to any node's other than a single container node's immediate children unless you also intend to implement `TraverseTree`.
pub trait TraversePartialTree {
    /// Type representing an iterator of the children of a node
    type ChildIter<'a>: Iterator<Item = NodeId>
    where
        Self: 'a;

    /// Get the list of children IDs for the given node
    fn child_ids(&self, parent_node_id: NodeId) -> Self::ChildIter<'_>;

    /// Get the number of children for the given node
    fn child_count(&self, parent_node_id: NodeId) -> usize;

    /// Get a specific child of a node, where the index represents the nth child
    fn get_child_id(&self, parent_node_id: NodeId, child_index: usize) -> NodeId;
}

/// A marker trait which extends `TraversePartialTree`
///
/// Implementing this trait implies the additional guarantee that the child/children methods can be used to recurse
/// infinitely down the tree. Is required by the `RoundTree` and the `PrintTree` traits.
pub trait TraverseTree: TraversePartialTree {}

/// Any type that implements [`LayoutPartialTree`] can be laid out using [Taffy's algorithms](crate::compute)
///
/// Note that this trait extends [`TraversePartialTree`] (not [`TraverseTree`]). Taffy's algorithm implementations have been designed such that they can be used for a laying out a single
/// node that only has access to it's immediate children.
pub trait LayoutPartialTree: TraversePartialTree {
    /// The style type representing the core container styles that all containers should have
    /// Used when laying out the root node of a tree
    type CoreContainerStyle<'a>: CoreStyle<CustomIdent = Self::CustomIdent>
    where
        Self: 'a;

    /// String type for representing "custom identifiers" (for example, named grid lines or areas)
    /// If you are unsure what to use here then consider `Arc<str>`.
    type CustomIdent: CheapCloneStr;

    /// Get core style
    fn get_core_container_style(&self, node_id: NodeId) -> Self::CoreContainerStyle<'_>;

    /// Resolve calc value
    #[inline(always)]
    fn resolve_calc_value(&self, val: *const (), basis: f32) -> f32 {
        let _ = val;
        let _ = basis;
        0.0
    }

    /// Set the node's unrounded layout
    fn set_unrounded_layout(&mut self, node_id: NodeId, layout: &Layout);

    /// Compute the specified node's size or full layout given the specified constraints
    fn compute_child_layout(&mut self, node_id: NodeId, inputs: LayoutInput) -> LayoutOutput;
}

/// Extends [`LayoutPartialTree`] with the operations needed by the out-of-flow positioning pass
/// ([`compute_oof_layout`](crate::compute_oof_layout)), which lays out the absolute/fixed boxes
/// for which a node acts as the containing block.
///
/// This trait is required by [`compute_oof_layout`](crate::compute_oof_layout) and
/// [`compute_root_layout`](crate::compute_root_layout).
pub trait LayoutContainingBlock: LayoutPartialTree {
    /// The style type representing the styles of an out-of-flow (absolute/fixed) box being
    /// positioned by its containing block
    type OofItemStyle<'a>: OofItemStyle<CustomIdent = Self::CustomIdent>
    where
        Self: 'a;

    /// Get the style of an out-of-flow box being positioned by its containing block
    fn get_oof_item_style(&self, node_id: NodeId) -> Self::OofItemStyle<'_>;

    /// Clear the list of out-of-flow (absolute/fixed) boxes whose containing block is `node_id`.
    ///
    /// This is called (exactly once) for each node laid out with `RunMode::PerformLayout`, before
    /// the boxes it lays out are recorded with [`add_hoisted_children`](Self::add_hoisted_children),
    /// so that lists recorded by previous layout runs do not persist.
    fn clear_hoisted_children(&mut self, node_id: NodeId);

    /// Append to the list of out-of-flow boxes whose containing block is `node_id`.
    ///
    /// This is called by each containing block for the boxes it lays out, and by
    /// [`compute_root_layout`](crate::compute_root_layout) to record boxes (e.g. `position: fixed`
    /// boxes) which are positioned by the final root positioning pass, which runs after the root
    /// node's own layout algorithm has already recorded its list. The recorded lists are consumed
    /// by [`round_layout`](crate::round_layout) (out-of-flow boxes are rounded via their
    /// containing block rather than via their parent), and are also useful for consumers
    /// implementing paint/hit-testing traversals.
    fn add_hoisted_children(&mut self, node_id: NodeId, hoisted: &[NodeId]);

    /// Read back the detailed layout information most recently recorded for `node_id`
    /// (as stored by [`LayoutGridContainer::set_detailed_grid_info`]).
    ///
    /// This is used by the out-of-flow positioning pass to resolve the grid area of absolutely
    /// positioned boxes whose containing block is a grid container. Implementing this method is
    /// optional: the default implementation returns [`DetailedLayoutInfo::None`], in which case
    /// such boxes are positioned relative to the grid container's padding box instead of their
    /// grid area.
    #[inline(always)]
    fn get_detailed_layout_info(&self, node_id: NodeId) -> &DetailedLayoutInfo<Self::CustomIdent> {
        let _ = node_id;
        &DetailedLayoutInfo::None
    }
}

/// Trait used by the `compute_cached_layout` method which allows cached layout results to be stored and retrieved.
///
/// The `Cache` struct implements a per-node cache that is compatible with this trait.
pub trait CacheTree {
    /// Try to retrieve a cached result from the cache
    fn cache_get(&mut self, node_id: NodeId, input: &LayoutInput) -> Option<LayoutOutput>;

    /// Store a computed size in the cache
    fn cache_store(&mut self, node_id: NodeId, input: &LayoutInput, layout_output: LayoutOutput);

    /// Clear all cache entries for the node
    fn cache_clear(&mut self, node_id: NodeId);
}

/// Trait used by the `round_layout` method which takes a tree of unrounded float-valued layouts and performs
/// rounding to snap the values to the pixel grid.
///
/// As indicated by it's dependence on `TraverseTree`, it required full recursive access to the tree.
pub trait RoundTree: TraverseTree {
    /// Get the node's unrounded layout
    fn get_unrounded_layout(&self, node_id: NodeId) -> Layout;
    /// Get a reference to the node's final layout
    fn set_final_layout(&mut self, node_id: NodeId, layout: &Layout);
    /// Whether the node is an out-of-flow (absolute/fixed) box. Out-of-flow boxes are hoisted to
    /// their containing block, so [`round_layout`](crate::round_layout) skips them when visiting a
    /// node's children and instead visits them via their containing block's hoisted child list.
    ///
    /// This should return `true` for box-generating nodes whose position style is `absolute` or
    /// `fixed`, and `false` otherwise (including for `display: none` nodes).
    fn is_out_of_flow(&self, node_id: NodeId) -> bool;
    /// The number of out-of-flow boxes whose containing block is `node_id`
    /// (as recorded by [`LayoutContainingBlock::add_hoisted_children`])
    fn hoisted_child_count(&self, node_id: NodeId) -> usize;
    /// Get the nth out-of-flow box whose containing block is `node_id`
    /// (as recorded by [`LayoutContainingBlock::add_hoisted_children`])
    fn get_hoisted_child_id(&self, node_id: NodeId, index: usize) -> NodeId;
}

/// Trait used by the `print_tree` method which prints a debug representation
///
/// As indicated by it's dependence on `TraverseTree`, it required full recursive access to the tree.
pub trait PrintTree: TraverseTree {
    /// Get a debug label for the node (typically the type of node: flexbox, grid, text, image, etc)
    fn get_debug_label(&self, node_id: NodeId) -> &'static str;
    /// Get a reference to the node's final layout
    fn get_final_layout(&self, node_id: NodeId) -> Layout;
}

#[cfg(feature = "flexbox")]
/// Extends [`LayoutPartialTree`] with getters for the styles required for Flexbox layout
pub trait LayoutFlexboxContainer: LayoutPartialTree {
    /// The style type representing the Flexbox container's styles
    type FlexboxContainerStyle<'a>: FlexboxContainerStyle
    where
        Self: 'a;
    /// The style type representing each Flexbox item's styles
    type FlexboxItemStyle<'a>: FlexboxItemStyle
    where
        Self: 'a;

    /// Get the container's styles
    fn get_flexbox_container_style(&self, node_id: NodeId) -> Self::FlexboxContainerStyle<'_>;

    /// Get the child's styles
    fn get_flexbox_child_style(&self, child_node_id: NodeId) -> Self::FlexboxItemStyle<'_>;
}

#[cfg(feature = "grid")]
/// Extends [`LayoutPartialTree`] with getters for the styles required for CSS Grid layout
pub trait LayoutGridContainer: LayoutPartialTree {
    /// The style type representing the CSS Grid container's styles
    type GridContainerStyle<'a>: GridContainerStyle<CustomIdent = Self::CustomIdent>
    where
        Self: 'a;

    /// The style type representing each CSS Grid item's styles
    type GridItemStyle<'a>: GridItemStyle<CustomIdent = Self::CustomIdent>
    where
        Self: 'a;

    /// Get the container's styles
    fn get_grid_container_style(&self, node_id: NodeId) -> Self::GridContainerStyle<'_>;

    /// Get the child's styles
    fn get_grid_child_style(&self, child_node_id: NodeId) -> Self::GridItemStyle<'_>;

    /// Set the node's detailed grid information
    ///
    /// Implementing this method is optional. Doing so allows you to access details about the the grid such as
    /// the computed size of each grid track and the computed placement of each grid item.
    fn set_detailed_grid_info(&mut self, _node_id: NodeId, _detailed_grid_info: DetailedGridInfo<Self::CustomIdent>) {
        debug_log!("LayoutGridContainer::set_detailed_grid_info called");
    }
}

#[cfg(feature = "block_layout")]
/// Extends [`LayoutPartialTree`] with getters for the styles required for CSS Block layout
pub trait LayoutBlockContainer: LayoutPartialTree {
    /// The style type representing the CSS Block container's styles
    type BlockContainerStyle<'a>: BlockContainerStyle
    where
        Self: 'a;
    /// The style type representing each CSS Block item's styles
    type BlockItemStyle<'a>: BlockItemStyle
    where
        Self: 'a;

    /// Get the container's styles
    fn get_block_container_style(&self, node_id: NodeId) -> Self::BlockContainerStyle<'_>;

    /// Get the child's styles
    fn get_block_child_style(&self, child_node_id: NodeId) -> Self::BlockItemStyle<'_>;

    /// Compute the specified node's size or full layout given the specified constraints
    #[cfg(feature = "block_layout")]
    fn compute_block_child_layout(
        &mut self,
        node_id: NodeId,
        inputs: LayoutInput,
        block_ctx: Option<&mut BlockContext<'_>>,
    ) -> LayoutOutput {
        let _ = block_ctx;
        self.compute_child_layout(node_id, inputs)
    }
}

// --- PRIVATE TRAITS

/// The parts of a child's own sizing styles that the parent applies on its behalf.
///
/// Layout algorithms never apply a node's own preferred size: the node's parent resolves it and
/// passes it down as a known dimension. This struct carries the remaining constraints, which the
/// parent applies to the inputs that it passes to the child and to the size that the child reports.
#[derive(Debug, Clone, Copy)]
pub(crate) struct ChildStyleConstraints {
    /// The child's resolved `min_size` style
    pub(crate) min_size: Size<Option<f32>>,
    /// The child's resolved `max_size` style
    pub(crate) max_size: Size<Option<f32>>,
    /// The child's resolved `max_size` style without any size transferred through the child's
    /// aspect ratio. This is the limit applied to the size that the child reports: content is
    /// allowed to make a box larger than the size that its aspect ratio would produce.
    pub(crate) untransferred_max_size: Size<Option<f32>>,
    /// The child's `aspect_ratio` style
    pub(crate) aspect_ratio: Option<f32>,
    /// The sum of the child's padding and border in each axis
    pub(crate) padding_border_size: Size<f32>,
    /// The sum of the child's margins in each axis
    pub(crate) margin_size: Size<f32>,
    /// The child's `min_size.width` style if it is an intrinsic sizing keyword
    /// (`min-content`, `max-content`, `fit-content` or `fit-content(...)`), else `auto`
    pub(crate) keyword_min_width: Dimension,
    /// The axes in which `min_size` was resolved from a `stretch` keyword. Such a minimum is equal to the space
    /// that is available to the child, so it is not used to adjust that space.
    pub(crate) min_size_is_stretch: Size<bool>,
    /// The child's `max_size.width` style if it is an intrinsic sizing keyword
    /// (`min-content`, `max-content`, `fit-content` or `fit-content(...)`), else `auto`
    pub(crate) keyword_max_width: Dimension,
}

/// Resolve the sizing styles of a node from its core style. For use when the node is not being laid out
/// as an item of a specific kind of container (which have their own item style accessors).
pub(crate) fn resolve_core_style_constraints<Tree: LayoutPartialTree>(
    tree: &Tree,
    node_id: NodeId,
    inputs: &mut LayoutInput,
) -> (ChildStyleConstraints, AutoAxes) {
    ChildStyleConstraints::resolve(&tree.get_core_container_style(node_id), inputs, |val, basis| tree.calc(val, basis))
}

/// Whether either of a `min_size` and a `max_size` style contain a sizing keyword in either axis
#[inline(always)]
pub(crate) fn has_min_max_sizing_keyword(min_size: Size<Dimension>, max_size: Size<Dimension>) -> bool {
    min_size.width.is_sizing_keyword()
        || min_size.height.is_sizing_keyword()
        || max_size.width.is_sizing_keyword()
        || max_size.height.is_sizing_keyword()
}

/// The size that a `stretch` min or max size of a child resolves to: the space that the child's parent
/// makes available to the child.
///
/// The available space is only used in an axis in which the parent's size is definite. Otherwise it may be
/// an available space that was passed down from a more distant ancestor, and `stretch` is treated as cyclic.
#[inline]
pub(crate) fn min_max_stretch_size(
    parent_size: Size<Option<f32>>,
    available_space: Size<AvailableSpace>,
) -> Size<Option<f32>> {
    use crate::util::sys::f32_max;
    Size {
        width: parent_size.width.and(available_space.width.into_option()).map(|space| f32_max(space, 0.0)),
        height: parent_size.height.and(available_space.height.into_option()).map(|space| f32_max(space, 0.0)),
    }
}

/// Resolve the sizing keywords in a `min_size` or `max_size` style that do not depend on the size of the
/// box's content, given the result of resolving the style's lengths and percentages:
///
/// - `stretch` resolves to `stretch_size`, or behaves as the property's initial value if that is indefinite
/// - The content-based keywords behave as the property's initial value in the block axis
///   <https://drafts.csswg.org/css-sizing-3/#sizing-values>
///
/// Returns the resolved border-box size, and the width style if it is a content-based keyword (else `auto`).
/// `box_sizing_adjustment` is the adjustment that the caller will add to the returned size.
#[cold]
pub(crate) fn resolve_extrinsic_min_max_keywords(
    style: Size<Dimension>,
    mut resolved: Size<Option<f32>>,
    stretch_size: Size<Option<f32>>,
    box_sizing_adjustment: Size<f32>,
) -> (Size<Option<f32>>, Dimension) {
    use crate::util::sys::f32_max;
    let mut keyword_width = Dimension::auto();
    if style.width.is_stretch() {
        resolved.width = stretch_size.width.map(|size| f32_max(size - box_sizing_adjustment.width, 0.0));
    } else if style.width.is_sizing_keyword() {
        keyword_width = style.width;
    }
    if style.height.is_stretch() {
        resolved.height = stretch_size.height.map(|size| f32_max(size - box_sizing_adjustment.height, 0.0));
    }
    (resolved, keyword_width)
}

/// Resolve a container's own `min_size` or `max_size` style for use by the container's layout algorithm.
///
/// In addition to lengths and percentages (which resolve against `parent_size`) this resolves `stretch` against the
/// space that the container's parent made available to the container. The content-based keywords are cyclic while
/// a container is sizing itself, so they behave as the property's initial value. The container's parent is
/// responsible for applying them to the size that the container reports.
///
/// `box_sizing_adjustment` is the adjustment that the caller will add to the returned size.
#[inline(always)]
pub(crate) fn resolve_container_min_max_size(
    style: Size<Dimension>,
    parent_size: Size<Option<f32>>,
    available_space: Size<AvailableSpace>,
    box_sizing_adjustment: Size<f32>,
    calc: impl Fn(*const (), f32) -> f32,
) -> Size<Option<f32>> {
    use crate::util::MaybeResolve;
    let resolved = style.maybe_resolve(parent_size, calc);
    if style.width.is_stretch() || style.height.is_stretch() {
        let stretch_size = min_max_stretch_size(parent_size, available_space);
        return resolve_extrinsic_min_max_keywords(style, resolved, stretch_size, box_sizing_adjustment).0;
    }
    resolved
}

/// The available space that measuring a box under yields the size that an intrinsic keyword min or
/// max width resolves to.
///
/// Returns `None` if the keyword does not constrain the box: a `fit-content` bound is the box's
/// fit-content size under the space that is available to it, which is never exceeded in either direction
/// by the width of a box whose width is determined by its content.
#[inline]
fn intrinsic_keyword_available_space(style: Dimension, basis: Option<f32>) -> Option<AvailableSpace> {
    use crate::CompactLength;
    match style.tag() {
        CompactLength::MIN_CONTENT_TAG => Some(AvailableSpace::MinContent),
        CompactLength::MAX_CONTENT_TAG => Some(AvailableSpace::MaxContent),
        CompactLength::FIT_CONTENT_PX_TAG => Some(AvailableSpace::Definite(style.value())),
        CompactLength::FIT_CONTENT_PERCENT_TAG => basis.map(|basis| AvailableSpace::Definite(basis * style.value())),
        _ => None,
    }
}

/// The smaller of two available spaces, where `MinContent` < `Definite(_)` < `MaxContent`
#[inline]
fn min_available_space(a: AvailableSpace, b: AvailableSpace) -> AvailableSpace {
    match (a, b) {
        (AvailableSpace::MinContent, _) | (_, AvailableSpace::MinContent) => AvailableSpace::MinContent,
        (AvailableSpace::MaxContent, other) | (other, AvailableSpace::MaxContent) => other,
        (AvailableSpace::Definite(a), AvailableSpace::Definite(b)) => {
            AvailableSpace::Definite(crate::util::sys::f32_min(a, b))
        }
    }
}

/// The larger of two available spaces, where `MinContent` < `Definite(_)` < `MaxContent`
#[inline]
fn max_available_space(a: AvailableSpace, b: AvailableSpace) -> AvailableSpace {
    match (a, b) {
        (AvailableSpace::MaxContent, _) | (_, AvailableSpace::MaxContent) => AvailableSpace::MaxContent,
        (AvailableSpace::MinContent, other) | (other, AvailableSpace::MinContent) => other,
        (AvailableSpace::Definite(a), AvailableSpace::Definite(b)) => {
            AvailableSpace::Definite(crate::util::sys::f32_max(a, b))
        }
    }
}

/// The axes in which a child's size is determined by its content rather than by a known dimension
#[derive(Debug, Clone, Copy)]
pub(crate) struct AutoAxes(Size<bool>);

impl ChildStyleConstraints {
    /// Resolve a child's own `size`, `min_size`, `max_size` and `aspect_ratio` styles into the
    /// `known_dimensions` and `available_space` of `inputs`, and return the constraints that must be
    /// applied to the size that the child reports.
    ///
    /// `style` must be the style of the child *as seen by its parent* (for example the style returned
    /// by `get_block_child_style` if the parent is a block container).
    pub(crate) fn resolve(
        style: &impl crate::style::CoreStyle,
        inputs: &mut LayoutInput,
        calc: impl Fn(*const (), f32) -> f32,
    ) -> (Self, AutoAxes) {
        use crate::style::BoxSizing;
        use crate::util::{MaybeMath, MaybeResolve, ResolveOrZero};

        let parent_size = inputs.parent_size;
        let aspect_ratio = style.aspect_ratio();
        let margin = style.margin().resolve_or_zero(parent_size.width, &calc);
        let padding = style.padding().resolve_or_zero(parent_size.width, &calc);
        let border = style.border().resolve_or_zero(parent_size.width, &calc);
        let padding_border_size = (padding + border).sum_axes();
        let box_sizing_adjustment =
            if style.box_sizing() == BoxSizing::ContentBox { padding_border_size } else { Size::ZERO };
        let min_size_style = style.min_size();
        let max_size_style = style.max_size();
        let mut untransferred_min_size = min_size_style.maybe_resolve(parent_size, &calc);
        let mut untransferred_max_size = max_size_style.maybe_resolve(parent_size, &calc);
        let mut keyword_min_width = Dimension::auto();
        let mut keyword_max_width = Dimension::auto();
        if has_min_max_sizing_keyword(min_size_style, max_size_style) {
            let stretch_size = min_max_stretch_size(parent_size, inputs.available_space);
            (untransferred_min_size, keyword_min_width) = resolve_extrinsic_min_max_keywords(
                min_size_style,
                untransferred_min_size,
                stretch_size,
                box_sizing_adjustment,
            );
            (untransferred_max_size, keyword_max_width) = resolve_extrinsic_min_max_keywords(
                max_size_style,
                untransferred_max_size,
                stretch_size,
                box_sizing_adjustment,
            );
        }
        // A minimum size that is transferred from the other axis through the aspect ratio does not
        // take precedence over a maximum size that is set in the axis that it is transferred to
        // <https://drafts.csswg.org/css-sizing-4/#aspect-ratio-size-transfers>
        let min_size = untransferred_min_size
            .or(untransferred_min_size.maybe_apply_aspect_ratio(aspect_ratio).maybe_min(untransferred_max_size))
            .maybe_add(box_sizing_adjustment);
        let untransferred_max_size = untransferred_max_size.maybe_add(box_sizing_adjustment);
        let max_size = untransferred_max_size
            .maybe_sub(box_sizing_adjustment)
            .maybe_apply_aspect_ratio(aspect_ratio)
            .maybe_add(box_sizing_adjustment);
        let clamped_style_size = style
            .size()
            .maybe_resolve(parent_size, &calc)
            .maybe_apply_aspect_ratio(aspect_ratio)
            .maybe_add(box_sizing_adjustment)
            .maybe_clamp(min_size, max_size);

        // The child's preferred size takes effect in any axis for which the caller has not already determined a size
        inputs.known_dimensions = inputs.known_dimensions.or(clamped_style_size.maybe_max(padding_border_size));

        let constraints = ChildStyleConstraints {
            min_size,
            max_size,
            untransferred_max_size,
            aspect_ratio,
            padding_border_size,
            margin_size: margin.sum_axes(),
            keyword_min_width,
            min_size_is_stretch: min_size_style.map(|style| style.is_stretch()),
            keyword_max_width,
        };
        let auto_axes = constraints.apply_to_inputs(inputs);
        (constraints, auto_axes)
    }

    /// Apply the constraints to the inputs that are about to be passed to the child. Returns the
    /// axes in which the child's size is not known, which must be passed to [`Self::apply_to_output`].
    #[inline]
    pub(crate) fn apply_to_inputs(&self, inputs: &mut LayoutInput) -> AutoAxes {
        use crate::util::MaybeMath;

        // If both min and max in a given axis are set and max <= min then this determines the size in that axis
        let min_max_definite_size = self.min_size.zip_map(self.max_size, |min, max| match (min, max) {
            (Some(min), Some(max)) if max <= min => Some(min),
            _ => None,
        });
        inputs.known_dimensions = inputs.known_dimensions.or(min_max_definite_size.maybe_max(self.padding_border_size));

        // The space available to the child's content is limited by the child's own min and max sizes
        inputs.available_space = Size {
            width: inputs.available_space.width.map_definite_value(|space| {
                (space - self.margin_size.width)
                    .maybe_clamp(self.min_size.width.filter(|_| !self.min_size_is_stretch.width), self.max_size.width)
                    + self.margin_size.width
            }),
            height: inputs.available_space.height.map_definite_value(|space| {
                (space - self.margin_size.height).maybe_clamp(
                    self.min_size.height.filter(|_| !self.min_size_is_stretch.height),
                    self.max_size.height,
                ) + self.margin_size.height
            }),
        };

        if self.has_keywords() {
            self.apply_keywords_to_inputs(inputs);
        }

        AutoAxes(inputs.known_dimensions.map(|dim| dim.is_none()))
    }

    /// Whether the child has a min or max width which depends on the size of its content
    #[inline(always)]
    pub(crate) fn has_keywords(&self) -> bool {
        !(self.keyword_min_width.is_auto() && self.keyword_max_width.is_auto())
    }

    /// Apply the child's intrinsic keyword min and max widths to the space that is made available to
    /// the child, for use when the child's width is determined by its content.
    ///
    /// The width of a box whose width is determined by its content is its fit-content size under
    /// the available space, and a fit-content size is monotonic in the available space. So clamping
    /// that width by a keyword bound (which is itself a fit-content size under some other
    /// available space) is the same as measuring the child once under the clamped available space.
    /// This means that no additional measurement of the child is required.
    #[cold]
    fn apply_keywords_to_inputs(&self, inputs: &mut LayoutInput) {
        if inputs.known_dimensions.width.is_some() {
            return;
        }
        let basis = inputs.parent_size.width;
        let mut available_width = inputs.available_space.width;
        // The child's other min and max widths are applied to the size that the child reports, which
        // gives them precedence over the bounds that are applied here. So that they have the precedence
        // that CSS gives them (the minimum is applied after the maximum) they are also applied here.
        if let Some(max) = self.max_size.width {
            available_width = min_available_space(available_width, AvailableSpace::Definite(max));
        }
        if let Some(max) = intrinsic_keyword_available_space(self.keyword_max_width, basis) {
            available_width = min_available_space(available_width, max);
        }
        if let Some(min) = self.min_size.width {
            available_width = max_available_space(available_width, AvailableSpace::Definite(min));
        }
        if let Some(min) = intrinsic_keyword_available_space(self.keyword_min_width, basis) {
            available_width = max_available_space(available_width, min);
        }
        inputs.available_space.width = available_width;
    }

    /// Apply the child's `min_size`, `max_size` and `aspect_ratio` styles to the size reported by the child
    #[inline]
    pub(crate) fn apply_to_output(&self, auto_axes: AutoAxes, output: &mut LayoutOutput) {
        use crate::util::MaybeMath;

        let AutoAxes(axis_is_auto) = auto_axes;
        // The size that the child reports is only modified in an axis that the child has a min or max size in
        // (the child itself is responsible for making sure that its size is large enough for its padding and border)
        let clamp = |size: f32, min: Option<f32>, max: Option<f32>, padding_border: f32| {
            if min.is_none() && max.is_none() {
                size
            } else {
                crate::util::sys::f32_max(size.maybe_clamp(min, max), padding_border)
            }
        };
        let mut size = output.size;
        if axis_is_auto.width {
            // A maximum width is applied to the child's available space if it has a keyword minimum width
            // (see `apply_keywords_to_inputs`)
            let max_width = if self.keyword_min_width.is_auto() { self.untransferred_max_size.width } else { None };
            size.width = clamp(size.width, self.min_size.width, max_width, self.padding_border_size.width);
        }
        if axis_is_auto.height {
            size.height = clamp(
                size.height,
                self.min_size.height,
                self.untransferred_max_size.height,
                self.padding_border_size.height,
            );
            if let Some(ratio) = self.aspect_ratio {
                size.height = crate::util::sys::f32_max(size.height, size.width / ratio);
            }
        }
        // A box with a non-zero height cannot be collapsed through
        if size.height != 0.0 {
            output.margins_can_collapse_through = false;
        }
        output.size = size;
    }
}

/// A private trait which allows us to add extra convenience methods to types which implement
/// LayoutTree without making those methods public.
pub(crate) trait LayoutPartialTreeExt: LayoutPartialTree {
    /// Compute the size of the node in a single axis given the specified constraints.
    /// The child's own sizing styles are not applied: the caller is responsible for applying them.
    #[inline(always)]
    fn measure_child_size(
        &mut self,
        node_id: NodeId,
        known_dimensions: Size<Option<f32>>,
        parent_size: Size<Option<f32>>,
        available_space: Size<AvailableSpace>,
        axis: AbsoluteAxis,
        vertical_margins_are_collapsible: Line<bool>,
    ) -> f32 {
        self.compute_child_layout(
            node_id,
            LayoutInput {
                known_dimensions,
                known_dimensions_are_definite: Size { width: true, height: true },
                parent_size,
                available_space,
                axis: axis.into(),
                run_mode: RunMode::ComputeSize,
                vertical_margins_are_collapsible,
            },
        )
        .size
        .get_abs(axis)
    }

    /// Compute the size of the node in both axes given the specified constraints.
    /// The child's own sizing styles are not applied: the caller is responsible for applying them.
    #[inline(always)]
    fn measure_child_size_both(
        &mut self,
        node_id: NodeId,
        known_dimensions: Size<Option<f32>>,
        parent_size: Size<Option<f32>>,
        available_space: Size<AvailableSpace>,
        vertical_margins_are_collapsible: Line<bool>,
    ) -> Size<f32> {
        self.compute_child_layout(
            node_id,
            LayoutInput {
                known_dimensions,
                known_dimensions_are_definite: Size { width: true, height: true },
                parent_size,
                available_space,
                axis: RequestedAxis::Both,
                run_mode: RunMode::ComputeSize,
                vertical_margins_are_collapsible,
            },
        )
        .size
    }

    /// Perform a full layout on the node given the specified constraints.
    /// The child's own sizing styles are not applied: the caller is responsible for applying them.
    #[inline(always)]
    fn perform_child_layout(
        &mut self,
        node_id: NodeId,
        known_dimensions: Size<Option<f32>>,
        parent_size: Size<Option<f32>>,
        available_space: Size<AvailableSpace>,
        vertical_margins_are_collapsible: Line<bool>,
    ) -> LayoutOutput {
        self.compute_child_layout(
            node_id,
            LayoutInput {
                known_dimensions,
                known_dimensions_are_definite: Size { width: true, height: true },
                parent_size,
                available_space,
                axis: RequestedAxis::Both,
                run_mode: RunMode::PerformLayout,
                vertical_margins_are_collapsible,
            },
        )
    }

    /// Compute the size of the node in a single axis given the specified constraints.
    /// The child's own sizing styles are resolved (by `resolve_styles`) and applied on its behalf.
    #[inline(always)]
    #[allow(clippy::too_many_arguments)]
    fn measure_child_size_with_styles(
        &mut self,
        node_id: NodeId,
        known_dimensions: Size<Option<f32>>,
        parent_size: Size<Option<f32>>,
        available_space: Size<AvailableSpace>,
        axis: AbsoluteAxis,
        vertical_margins_are_collapsible: Line<bool>,
        resolve_styles: impl FnOnce(&Self, NodeId, &mut LayoutInput) -> (ChildStyleConstraints, AutoAxes),
    ) -> f32 {
        self.compute_child_layout_with_styles(
            node_id,
            LayoutInput {
                known_dimensions,
                known_dimensions_are_definite: Size { width: true, height: true },
                parent_size,
                available_space,
                axis: axis.into(),
                run_mode: RunMode::ComputeSize,
                vertical_margins_are_collapsible,
            },
            resolve_styles,
        )
        .size
        .get_abs(axis)
    }

    /// Compute the size of the node in both axes given the specified constraints.
    /// The child's own sizing styles are resolved (by `resolve_styles`) and applied on its behalf.
    #[inline(always)]
    #[allow(clippy::too_many_arguments)]
    fn measure_child_size_both_with_styles(
        &mut self,
        node_id: NodeId,
        known_dimensions: Size<Option<f32>>,
        parent_size: Size<Option<f32>>,
        available_space: Size<AvailableSpace>,
        vertical_margins_are_collapsible: Line<bool>,
        resolve_styles: impl FnOnce(&Self, NodeId, &mut LayoutInput) -> (ChildStyleConstraints, AutoAxes),
    ) -> Size<f32> {
        self.compute_child_layout_with_styles(
            node_id,
            LayoutInput {
                known_dimensions,
                known_dimensions_are_definite: Size { width: true, height: true },
                parent_size,
                available_space,
                axis: RequestedAxis::Both,
                run_mode: RunMode::ComputeSize,
                vertical_margins_are_collapsible,
            },
            resolve_styles,
        )
        .size
    }

    /// Perform a full layout on the node given the specified constraints.
    /// The child's own sizing styles are resolved (by `resolve_styles`) and applied on its behalf.
    #[inline(always)]
    #[allow(clippy::too_many_arguments)]
    fn perform_child_layout_with_styles(
        &mut self,
        node_id: NodeId,
        known_dimensions: Size<Option<f32>>,
        parent_size: Size<Option<f32>>,
        available_space: Size<AvailableSpace>,
        vertical_margins_are_collapsible: Line<bool>,
        resolve_styles: impl FnOnce(&Self, NodeId, &mut LayoutInput) -> (ChildStyleConstraints, AutoAxes),
    ) -> LayoutOutput {
        self.compute_child_layout_with_styles(
            node_id,
            LayoutInput {
                known_dimensions,
                known_dimensions_are_definite: Size { width: true, height: true },
                parent_size,
                available_space,
                axis: RequestedAxis::Both,
                run_mode: RunMode::PerformLayout,
                vertical_margins_are_collapsible,
            },
            resolve_styles,
        )
    }

    /// Compute the layout of a child, resolving and applying the child's own sizing styles on its behalf.
    /// `resolve_styles` resolves those styles from the style of the child as seen by its parent.
    #[inline(always)]
    fn compute_child_layout_with_styles(
        &mut self,
        node_id: NodeId,
        mut inputs: LayoutInput,
        resolve_styles: impl FnOnce(&Self, NodeId, &mut LayoutInput) -> (ChildStyleConstraints, AutoAxes),
    ) -> LayoutOutput {
        // The child's own sizing styles only affect axes in which its size is not already known
        if inputs.known_dimensions.both_axis_defined() {
            return self.compute_child_layout(node_id, inputs);
        }
        let caller_known_dimensions = inputs.known_dimensions;
        let (constraints, mut auto_axes) = resolve_styles(self, node_id, &mut inputs);
        if constraints.has_keywords() {
            // The caller is responsible for applying the child's min and max sizes to the dimensions that it
            // determined. The ones that were resolved from the child's preferred size style are clamped here.
            if let (None, Some(width)) = (caller_known_dimensions.width, inputs.known_dimensions.width) {
                inputs.known_dimensions.width = Some(self.clamp_width_by_min_max_sizing_keywords(
                    node_id,
                    width,
                    constraints.keyword_min_width,
                    constraints.keyword_max_width,
                    constraints.min_size.width,
                    constraints.padding_border_size.width,
                    min_max_stretch_size(inputs.parent_size, inputs.available_space).width,
                    match inputs.available_space.width {
                        AvailableSpace::MinContent => AvailableSpace::MinContent,
                        _ => AvailableSpace::MaxContent,
                    },
                    inputs.known_dimensions.height,
                    inputs.parent_size,
                    inputs.available_space.height,
                    inputs.vertical_margins_are_collapsible,
                ));
            }
            auto_axes = AutoAxes(inputs.known_dimensions.map(|dim| dim.is_none()));
        }
        let mut output = self.compute_child_layout(node_id, inputs);
        constraints.apply_to_output(auto_axes, &mut output);
        output
    }

    /// Clamp a width of a child, which is not determined by the size of the child's content, by its
    /// `min_size.width` and `max_size.width` styles if those are content-based sizing keywords (`min-content`,
    /// `max-content`, `fit-content`, or `fit-content(...)`). This measures the child's content.
    ///
    /// - `width` must already be clamped by the child's other min and max widths
    /// - `min_width` is the child's resolved minimum width, which takes precedence over a keyword maximum
    /// - `stretch_width` is the width that the child would have if it were stretched to fill the space
    ///   available to it. `fit-content` is resolved against it.
    /// - `fit_content_max_fallback` is the constraint that a `fit-content` maximum is measured under if
    ///   `stretch_width` is indefinite: `MinContent` if the child's min-content contribution is being computed,
    ///   else `MaxContent`
    /// - `known_height`, `parent_size`, `available_height` and `vertical_margins_are_collapsible` are the inputs
    ///   that the child is otherwise being laid out with
    #[cold]
    #[allow(clippy::too_many_arguments)]
    fn clamp_width_by_min_max_sizing_keywords(
        &mut self,
        node_id: NodeId,
        width: f32,
        min_width_style: Dimension,
        max_width_style: Dimension,
        min_width: Option<f32>,
        padding_border_width: f32,
        stretch_width: Option<f32>,
        fit_content_max_fallback: AvailableSpace,
        known_height: Option<f32>,
        parent_size: Size<Option<f32>>,
        available_height: AvailableSpace,
        vertical_margins_are_collapsible: Line<bool>,
    ) -> f32 {
        use crate::util::MaybeMath;

        let mut resolve = |style: Dimension, fallback: AvailableSpace| {
            self.resolve_min_max_width_keyword(
                node_id,
                style,
                fallback,
                stretch_width,
                known_height,
                parent_size,
                available_height,
                vertical_margins_are_collapsible,
            )
        };
        // With an indefinite available width a `fit-content` maximum is the intrinsic size that is being
        // computed (else the max-content size) and a `fit-content` minimum is the min-content size
        let max = resolve(max_width_style, fit_content_max_fallback);
        let min = resolve(min_width_style, AvailableSpace::MinContent);
        width.maybe_min(max).maybe_max(min).maybe_max(min_width).max(padding_border_width)
    }

    /// Resolve a child's `min_size.width` or `max_size.width` style if it is a sizing keyword (`min-content`,
    /// `max-content`, `fit-content`, `fit-content(...)`, or `stretch`), measuring the child's content
    /// if required. Returns `None` if the style is not a sizing keyword, or if it cannot be resolved.
    ///
    /// - `fallback` is the constraint that a `fit-content` keyword is measured under if the size
    ///   that it depends on is indefinite
    /// - `stretch_width` is the width that the child would have if it were stretched to fill the space
    ///   available to it
    /// - `known_height`, `parent_size`, `available_height` and `vertical_margins_are_collapsible` are the inputs
    ///   that the child is otherwise being laid out with
    #[cold]
    #[allow(clippy::too_many_arguments)]
    fn resolve_min_max_width_keyword(
        &mut self,
        node_id: NodeId,
        style: Dimension,
        fallback: AvailableSpace,
        stretch_width: Option<f32>,
        known_height: Option<f32>,
        parent_size: Size<Option<f32>>,
        available_height: AvailableSpace,
        vertical_margins_are_collapsible: Line<bool>,
    ) -> Option<f32> {
        use crate::compute::common::sizing_keyword::{resolve_sizing_keyword, SizingKeywordResolution};

        if !style.is_sizing_keyword() {
            return None;
        }
        let available_width = match resolve_sizing_keyword(style, stretch_width, parent_size.width) {
            Some(SizingKeywordResolution::Exact(size)) => return Some(size),
            Some(SizingKeywordResolution::Measure(space)) => space,
            None if style.is_stretch() => return None,
            None => fallback,
        };
        Some(self.measure_child_size(
            node_id,
            Size { width: None, height: known_height },
            parent_size,
            Size { width: available_width, height: available_height },
            AbsoluteAxis::Horizontal,
            vertical_margins_are_collapsible,
        ))
    }

    /// Compute the layout of a child whose own sizing styles have already been resolved by the caller:
    /// the child's preferred size must already be included in the known dimensions of `inputs`.
    /// `constraints` may be `None` if both of the child's dimensions are known.
    #[inline(always)]
    fn compute_child_layout_with_constraints(
        &mut self,
        node_id: NodeId,
        mut inputs: LayoutInput,
        constraints: Option<&ChildStyleConstraints>,
    ) -> LayoutOutput {
        // The child's own sizing styles only affect axes in which its size is not already known
        let Some(constraints) = constraints else {
            return self.compute_child_layout(node_id, inputs);
        };
        let auto_axes = constraints.apply_to_inputs(&mut inputs);
        let mut output = self.compute_child_layout(node_id, inputs);
        constraints.apply_to_output(auto_axes, &mut output);
        output
    }

    /// Alias to `resolve_calc_value` with a shorter function name
    #[inline(always)]
    #[cfg(feature = "calc")]
    fn calc(&self, val: *const (), basis: f32) -> f32 {
        self.resolve_calc_value(val, basis)
    }

    /// Alias to `resolve_calc_value` with a shorter function name
    #[inline(always)]
    #[cfg(not(feature = "calc"))]
    fn calc(&self, _val: *const (), _basis: f32) -> f32 {
        0.0
    }
}

impl<T: LayoutPartialTree> LayoutPartialTreeExt for T {}
