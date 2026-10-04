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
use crate::style::{AvailableSpace, CoreStyle, OofItemStyle};
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
}

/// A node's `min_size` and `max_size` styles, resolved into border-box sizes in the form in which they
/// are passed down to the node in [`LayoutInput::min_size`] and [`LayoutInput::max_size`].
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct MinMaxSize {
    /// The resolved `min_size` style
    pub(crate) min: Size<Option<f32>>,
    /// The resolved `max_size` style
    pub(crate) max: Size<Option<f32>>,
}

impl MinMaxSize {
    /// No minimum or maximum size in either axis
    pub(crate) const NONE: Self = Self { min: Size::NONE, max: Size::NONE };

    /// Whether a node has a `min_size` or `max_size` style that is not `auto` in either axis
    #[inline(always)]
    pub(crate) fn is_set(style: &impl crate::style::CoreStyle) -> bool {
        let min_size = style.min_size();
        let max_size = style.max_size();
        !(min_size.width.is_auto()
            && min_size.height.is_auto()
            && max_size.width.is_auto()
            && max_size.height.is_auto())
    }

    /// Resolve the `min_size` and `max_size` styles of a node whose percentages resolve against `parent_size`.
    /// Each size is transferred through the node's aspect ratio into an axis in which it is not set.
    #[inline]
    pub(crate) fn resolve(
        style: &impl crate::style::CoreStyle,
        parent_size: Size<Option<f32>>,
        calc: impl Fn(*const (), f32) -> f32,
    ) -> Self {
        use crate::style::BoxSizing;
        use crate::util::{MaybeMath, MaybeResolve, ResolveOrZero};

        // The overwhelmingly common case: the node has neither a minimum nor a maximum size
        if !Self::is_set(style) {
            return Self::NONE;
        }
        let min_size = style.min_size();
        let max_size = style.max_size();

        let aspect_ratio = style.aspect_ratio();
        let box_sizing_adjustment = if style.box_sizing() == BoxSizing::ContentBox {
            let padding = style.padding().resolve_or_zero(parent_size.width, &calc);
            let border = style.border().resolve_or_zero(parent_size.width, &calc);
            (padding + border).sum_axes()
        } else {
            Size::ZERO
        };
        Self {
            min: min_size
                .maybe_resolve(parent_size, &calc)
                .maybe_apply_aspect_ratio(aspect_ratio)
                .maybe_add(box_sizing_adjustment),
            max: max_size
                .maybe_resolve(parent_size, &calc)
                .maybe_apply_aspect_ratio(aspect_ratio)
                .maybe_add(box_sizing_adjustment),
        }
    }

    /// Set the `min_size` and `max_size` of `inputs`
    #[inline(always)]
    pub(crate) fn apply_to_inputs(self, inputs: &mut LayoutInput) {
        inputs.min_size = self.min;
        inputs.max_size = self.max;
    }
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

/// As [`resolve_core_style_constraints`], but only resolves the constraints that are needed
/// (see [`ChildStyleConstraints::resolve_unknown_axes`]).
pub(crate) fn resolve_core_style_constraints_for_unknown_axes<Tree: LayoutPartialTree>(
    tree: &Tree,
    node_id: NodeId,
    inputs: &mut LayoutInput,
) -> Option<(ChildStyleConstraints, AutoAxes)> {
    ChildStyleConstraints::resolve_unknown_axes(&tree.get_core_container_style(node_id), inputs, |val, basis| {
        tree.calc(val, basis)
    })
}

/// The axes in which a child's size is determined by its content rather than by a known dimension
#[derive(Debug, Clone, Copy)]
pub(crate) struct AutoAxes(Size<bool>);

impl ChildStyleConstraints {
    /// As [`Self::resolve`], except that if both of the child's dimensions are already known then only
    /// the `min_size` and `max_size` of `inputs` are resolved and `None` is returned: the child's other
    /// sizing styles only affect axes in which its size is not known.
    #[inline(always)]
    pub(crate) fn resolve_unknown_axes(
        style: &impl crate::style::CoreStyle,
        inputs: &mut LayoutInput,
        calc: impl Fn(*const (), f32) -> f32,
    ) -> Option<(Self, AutoAxes)> {
        if inputs.known_dimensions.both_axis_defined() {
            MinMaxSize::resolve(style, inputs.parent_size, calc).apply_to_inputs(inputs);
            return None;
        }
        Some(Self::resolve(style, inputs, calc))
    }

    /// Resolve a child's own `size`, `min_size`, `max_size` and `aspect_ratio` styles into the
    /// `known_dimensions`, `min_size`, `max_size` and `available_space` of `inputs`, and return the
    /// constraints that must be applied to the size that the child reports.
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
        let untransferred_min_size = style.min_size().maybe_resolve(parent_size, &calc);
        let untransferred_max_size = style.max_size().maybe_resolve(parent_size, &calc);

        // The min and max sizes that are passed down to the child for use by its own layout algorithm.
        // These are equal to `MinMaxSize::resolve`.
        inputs.min_size =
            untransferred_min_size.maybe_apply_aspect_ratio(aspect_ratio).maybe_add(box_sizing_adjustment);
        inputs.max_size =
            untransferred_max_size.maybe_apply_aspect_ratio(aspect_ratio).maybe_add(box_sizing_adjustment);

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
                (space - self.margin_size.width).maybe_clamp(self.min_size.width, self.max_size.width)
                    + self.margin_size.width
            }),
            height: inputs.available_space.height.map_definite_value(|space| {
                (space - self.margin_size.height).maybe_clamp(self.min_size.height, self.max_size.height)
                    + self.margin_size.height
            }),
        };

        AutoAxes(inputs.known_dimensions.map(|dim| dim.is_none()))
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
            size.width = clamp(
                size.width,
                self.min_size.width,
                self.untransferred_max_size.width,
                self.padding_border_size.width,
            );
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
    #[allow(clippy::too_many_arguments)]
    fn measure_child_size(
        &mut self,
        node_id: NodeId,
        known_dimensions: Size<Option<f32>>,
        min_max_size: MinMaxSize,
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
                min_size: min_max_size.min,
                max_size: min_max_size.max,
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
        min_max_size: MinMaxSize,
        parent_size: Size<Option<f32>>,
        available_space: Size<AvailableSpace>,
        vertical_margins_are_collapsible: Line<bool>,
    ) -> Size<f32> {
        self.compute_child_layout(
            node_id,
            LayoutInput {
                known_dimensions,
                known_dimensions_are_definite: Size { width: true, height: true },
                min_size: min_max_size.min,
                max_size: min_max_size.max,
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
        min_max_size: MinMaxSize,
        parent_size: Size<Option<f32>>,
        available_space: Size<AvailableSpace>,
        vertical_margins_are_collapsible: Line<bool>,
    ) -> LayoutOutput {
        self.compute_child_layout(
            node_id,
            LayoutInput {
                known_dimensions,
                known_dimensions_are_definite: Size { width: true, height: true },
                min_size: min_max_size.min,
                max_size: min_max_size.max,
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
        resolve_styles: impl FnOnce(&Self, NodeId, &mut LayoutInput) -> Option<(ChildStyleConstraints, AutoAxes)>,
    ) -> f32 {
        self.compute_child_layout_with_styles(
            node_id,
            LayoutInput {
                known_dimensions,
                known_dimensions_are_definite: Size { width: true, height: true },
                // Filled in by `resolve_styles`
                min_size: Size::NONE,
                max_size: Size::NONE,
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
        resolve_styles: impl FnOnce(&Self, NodeId, &mut LayoutInput) -> Option<(ChildStyleConstraints, AutoAxes)>,
    ) -> Size<f32> {
        self.compute_child_layout_with_styles(
            node_id,
            LayoutInput {
                known_dimensions,
                known_dimensions_are_definite: Size { width: true, height: true },
                // Filled in by `resolve_styles`
                min_size: Size::NONE,
                max_size: Size::NONE,
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
        resolve_styles: impl FnOnce(&Self, NodeId, &mut LayoutInput) -> Option<(ChildStyleConstraints, AutoAxes)>,
    ) -> LayoutOutput {
        self.compute_child_layout_with_styles(
            node_id,
            LayoutInput {
                known_dimensions,
                known_dimensions_are_definite: Size { width: true, height: true },
                // Filled in by `resolve_styles`
                min_size: Size::NONE,
                max_size: Size::NONE,
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
    /// `resolve_styles` resolves those styles from the style of the child as seen by its parent. It always
    /// resolves the `min_size` and `max_size` of the inputs, and returns `None` if both of the child's
    /// dimensions are already known (in which case there is nothing to apply to the child's output).
    #[inline(always)]
    fn compute_child_layout_with_styles(
        &mut self,
        node_id: NodeId,
        mut inputs: LayoutInput,
        resolve_styles: impl FnOnce(&Self, NodeId, &mut LayoutInput) -> Option<(ChildStyleConstraints, AutoAxes)>,
    ) -> LayoutOutput {
        // The child's own sizing styles (other than the min and max sizes that are passed down to the
        // child) only affect axes in which its size is not already known
        let Some((constraints, auto_axes)) = resolve_styles(self, node_id, &mut inputs) else {
            return self.compute_child_layout(node_id, inputs);
        };
        let mut output = self.compute_child_layout(node_id, inputs);
        constraints.apply_to_output(auto_axes, &mut output);
        output
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
