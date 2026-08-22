//! Final data structures that represent the high-level UI layout
use crate::geometry::{AbsoluteAxis, Line, Point, Rect, Size};
use crate::style::{AlignmentSafety, AvailableSpace, CheapCloneStr, Position};
use crate::style_helpers::TaffyMaxContent;
use crate::sys::DefaultCheapStr;
use crate::tree::NodeId;
use crate::util::sys::{f32_max, f32_min};

/// Whether we are performing a full layout, or we merely need to size the node
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum RunMode {
    /// A full layout for this node and all children should be computed
    PerformLayout,
    /// The layout algorithm should be executed such that an accurate container size for the node can be determined.
    /// Layout steps that aren't necessary for determining the container size of the current node can be skipped.
    ComputeSize,
    /// This node should have a null layout set as it has been hidden (i.e. using `Display::None`)
    PerformHiddenLayout,
}

/// Whether styles should be taken into account when computing size
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum SizingMode {
    /// Only content contributions should be taken into account
    ContentSize,
    /// Inherent size styles should be taken into account in addition to content contributions
    InherentSize,
}

/// A set of margins that are available for collapsing with for block layout's margin collapsing
#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CollapsibleMarginSet {
    /// The largest positive margin
    positive: f32,
    /// The smallest negative margin (with largest absolute value)
    negative: f32,
}

impl CollapsibleMarginSet {
    /// A default margin set with no collapsible margins
    pub const ZERO: Self = Self { positive: 0.0, negative: 0.0 };

    /// Create a set from a single margin
    pub fn from_margin(margin: f32) -> Self {
        if margin >= 0.0 {
            Self { positive: margin, negative: 0.0 }
        } else {
            Self { positive: 0.0, negative: margin }
        }
    }

    /// Collapse a single margin with this set
    pub fn collapse_with_margin(mut self, margin: f32) -> Self {
        if margin >= 0.0 {
            self.positive = f32_max(self.positive, margin);
        } else {
            self.negative = f32_min(self.negative, margin);
        }
        self
    }

    /// Collapse another margin set with this set
    pub fn collapse_with_set(mut self, other: CollapsibleMarginSet) -> Self {
        self.positive = f32_max(self.positive, other.positive);
        self.negative = f32_min(self.negative, other.negative);
        self
    }

    /// Resolve the resultant margin from this set once all collapsible margins
    /// have been collapsed into it
    pub fn resolve(&self) -> f32 {
        self.positive + self.negative
    }
}

/// An axis that layout algorithms can be requested to compute a size for
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum RequestedAxis {
    /// The horizontal axis
    Horizontal,
    /// The vertical axis
    Vertical,
    /// Both axes
    Both,
}

impl From<AbsoluteAxis> for RequestedAxis {
    fn from(value: AbsoluteAxis) -> Self {
        match value {
            AbsoluteAxis::Horizontal => RequestedAxis::Horizontal,
            AbsoluteAxis::Vertical => RequestedAxis::Vertical,
        }
    }
}
impl TryFrom<RequestedAxis> for AbsoluteAxis {
    type Error = ();
    fn try_from(value: RequestedAxis) -> Result<Self, Self::Error> {
        match value {
            RequestedAxis::Horizontal => Ok(AbsoluteAxis::Horizontal),
            RequestedAxis::Vertical => Ok(AbsoluteAxis::Vertical),
            RequestedAxis::Both => Err(()),
        }
    }
}

/// A struct containing the inputs constraints/hints for laying out a node, which are passed in by the parent
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct LayoutInput {
    /// Whether we only need to know the Node's size, or whether we need to perform a full layout
    pub run_mode: RunMode,
    /// Whether a Node's style sizes should be taken into account or ignored
    pub sizing_mode: SizingMode,
    /// Which axis we need the size of
    pub axis: RequestedAxis,

    /// Known dimensions represent dimensions (width/height) which should be taken as fixed when performing layout.
    /// For example, if known_dimensions.width is set to Some(WIDTH) then this means something like:
    ///
    ///    "What would the height of this node be, assuming the width is WIDTH"
    ///
    /// Layout functions will be called with both known_dimensions set for final layout. Where the meaning is:
    ///
    ///   "The exact size of this node is WIDTHxHEIGHT. Please lay out your children"
    ///
    pub known_dimensions: Size<Option<f32>>,
    /// Whether each known dimension should be treated as a *definite* size when laying out the node's
    /// own content (resolving percentage sizes of children, and collecting flex items into flex lines).
    ///
    /// This should be set to `false` for a dimension when a parent imposes a known dimension on a node
    /// that is derived from the node's own content, and is therefore indefinite per CSS. For example,
    /// the post-flexing main size of a flex item is indefinite if the flex container's main size is
    /// indefinite and the item's used flex basis is not definite
    /// (see <https://www.w3.org/TR/css-flexbox-1/#definite-sizes>).
    ///
    /// This flag is ignored (treated as `true`) for axes where the corresponding known dimension is `None`.
    pub known_dimensions_are_definite: Size<bool>,
    /// Parent size dimensions are intended to be used for percentage resolution.
    pub parent_size: Size<Option<f32>>,
    /// Available space represents an amount of space to layout into, and is used as a soft constraint
    /// for the purpose of wrapping.
    pub available_space: Size<AvailableSpace>,
    /// Specific to CSS Block layout. Used for correctly computing margin collapsing. You probably want to set this to `Line::FALSE`.
    pub vertical_margins_are_collapsible: Line<bool>,
}

impl LayoutInput {
    /// A LayoutInput that can be used to request hidden layout
    pub const HIDDEN: LayoutInput = LayoutInput {
        // The important property for hidden layout
        run_mode: RunMode::PerformHiddenLayout,
        // The rest will be ignored
        known_dimensions: Size::NONE,
        known_dimensions_are_definite: Size { width: true, height: true },
        parent_size: Size::NONE,
        available_space: Size::MAX_CONTENT,
        sizing_mode: SizingMode::InherentSize,
        axis: RequestedAxis::Both,
        vertical_margins_are_collapsible: Line::FALSE,
    };
}

/// The first and last baselines of a node in the horizontal axis (i.e. baselines for horizontal text,
/// measured as an offset from the top edge of the node's border box).
///
/// A baseline is the line on which text sits. See <https://www.w3.org/TR/css-writing-modes-3/#intro-baselines>
/// for details.
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Baselines {
    /// The first baseline of the node, if any
    pub first: Option<f32>,
    /// The last baseline of the node, if any
    pub last: Option<f32>,
}

impl Baselines {
    /// A `Baselines` with neither a first nor a last baseline
    pub const NONE: Self = Self { first: None, last: None };

    /// Create a `Baselines` from just a first baseline
    pub const fn from_first(first: Option<f32>) -> Self {
        Self { first, last: None }
    }
}

/// The physical alignment of an out-of-flow box's margin box within its static-position
/// area (per-axis).
///
/// The static position of an out-of-flow box may depend on alignment properties
/// (`justify-content`/`align-self` in flexbox, `justify-self`/`align-self` in grid) which cannot
/// be fully resolved until the box's final size is known. To defer that resolution, the static
/// position is recorded as an *area* plus the physical alignment keyword to apply to the box's
/// margin box within that area.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum StaticEdge {
    /// Align the start (left/top) edge of the box's margin box to the start of the area
    Start,
    /// Center the box within the area
    Center,
    /// Align the end (right/bottom) edge of the box's margin box to the end of the area
    End,
}

/// The alignment to apply to an out-of-flow box's margin box within its static-position area
/// in a single axis.
///
/// Keywords are physical: container-specific semantics (`justify-content`/`align-self`
/// resolution, flex-direction/wrap-reverse reversal, RTL flipping, `self-start`/`self-end`)
/// are resolved by the emitting layout algorithm.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct StaticAlign {
    /// The physical alignment keyword
    pub keyword: StaticEdge,
    /// The `safe` overflow-position keyword from CSS Box Alignment: when `Safe`, the
    /// `fallback` keyword is used instead if the box's margin box overflows the area
    pub safety: AlignmentSafety,
    /// The physical alignment keyword to fall back to when `safety` is `Safe` and the box's
    /// margin box overflows the area
    pub fallback: StaticEdge,
}

impl StaticAlign {
    /// Create a `StaticAlign` from a keyword with no safe fallback
    pub const fn from_keyword(keyword: StaticEdge) -> Self {
        Self { keyword, safety: AlignmentSafety::Unsafe, fallback: keyword }
    }
}

/// The static position of an out-of-flow box in a single axis: the alignment container
/// (start/end coordinates) plus the alignment to apply to the box's margin box within it.
///
/// Area coordinates are initially relative to the border box of the node which produced the
/// candidate and are translated by each node's location as the candidate bubbles up the tree, so
/// they are always relative to the border box of the node currently holding the candidate.
///
/// Emitters which have no alignment area in an axis (e.g. block layout, where the static
/// position is a point) emit a degenerate area (`start == end`).
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct StaticPosition {
    /// The alignment container in this axis
    pub area: Line<f32>,
    /// The alignment to apply to the box's margin box within `area`
    pub align: StaticAlign,
}

impl StaticPosition {
    /// Create a `StaticPosition` with a degenerate (zero-extent) area at `anchor` and no
    /// safe fallback
    pub const fn from_edge(anchor: f32, edge: StaticEdge) -> Self {
        Self { area: Line { start: anchor, end: anchor }, align: StaticAlign::from_keyword(edge) }
    }
}

/// A record of an out-of-flow (`position: absolute` or `position: fixed`) box which has not yet
/// been laid out because its containing block is an ancestor of its parent.
///
/// Candidates are produced by the layout algorithm of the box's parent (which computes the static
/// position) and bubble up through `LayoutOutput` until they reach a node which acts as the box's
/// containing block, which lays the box out.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct OofCandidate {
    /// The id of the out-of-flow node
    pub node: NodeId,
    /// The `order` value that the node's layout should be assigned (its child index within its parent)
    pub order: u32,
    /// The position style of the node (`Position::Absolute` or `Position::Fixed`)
    pub position: Position,
    /// The static position of the box in each axis
    pub static_position: Point<StaticPosition>,
}

/// A list of [`OofCandidate`]s.
///
/// When built with the `std` or `alloc` features this is a lazily-allocated heap vector
/// (an empty list performs no allocation). In heapless builds it is an inline fixed-capacity
/// vector which panics if more than 16 candidates accumulate in a single list.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct OofCandidates {
    /// Backing storage for the candidates
    #[cfg(any(feature = "std", feature = "alloc"))]
    inner: Option<crate::util::sys::Box<crate::util::sys::Vec<OofCandidate>>>,
    /// Backing storage for the candidates
    #[cfg(not(any(feature = "std", feature = "alloc")))]
    inner: arrayvec::ArrayVec<OofCandidate, 16>,
}

impl OofCandidates {
    /// An empty list of candidates
    pub const NONE: Self = Self {
        #[cfg(any(feature = "std", feature = "alloc"))]
        inner: None,
        #[cfg(not(any(feature = "std", feature = "alloc")))]
        inner: arrayvec::ArrayVec::new_const(),
    };

    /// Create a new empty list of candidates
    pub const fn new() -> Self {
        Self::NONE
    }

    /// Whether the list contains no candidates
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.as_slice().is_empty()
    }

    /// The candidates as a slice
    #[inline]
    pub fn as_slice(&self) -> &[OofCandidate] {
        #[cfg(any(feature = "std", feature = "alloc"))]
        return self.inner.as_ref().map(|boxed| boxed.as_slice()).unwrap_or(&[]);
        #[cfg(not(any(feature = "std", feature = "alloc")))]
        return self.inner.as_slice();
    }

    /// The candidates as a mutable slice
    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [OofCandidate] {
        #[cfg(any(feature = "std", feature = "alloc"))]
        return self.inner.as_mut().map(|boxed| boxed.as_mut_slice()).unwrap_or(&mut []);
        #[cfg(not(any(feature = "std", feature = "alloc")))]
        return self.inner.as_mut_slice();
    }

    /// Append a candidate to the list
    #[inline]
    pub fn push(&mut self, candidate: OofCandidate) {
        #[cfg(any(feature = "std", feature = "alloc"))]
        self.inner.get_or_insert_with(Default::default).push(candidate);
        #[cfg(not(any(feature = "std", feature = "alloc")))]
        self.inner.push(candidate);
    }

    /// Move all candidates from `other` into `self`, leaving `other` empty
    #[inline]
    pub fn append(&mut self, other: &mut OofCandidates) {
        if other.is_empty() {
            return;
        }
        #[cfg(any(feature = "std", feature = "alloc"))]
        match &mut self.inner {
            None => self.inner = other.inner.take(),
            Some(vec) => vec.append(other.inner.as_mut().unwrap()),
        }
        #[cfg(not(any(feature = "std", feature = "alloc")))]
        {
            for candidate in other.inner.drain(..) {
                self.inner.push(candidate);
            }
        }
    }

    /// Take the candidates out of this list, leaving it empty
    #[inline]
    pub fn take(&mut self) -> Self {
        core::mem::take(self)
    }

    /// Remove all candidates from the list
    #[inline]
    pub fn clear(&mut self) {
        #[cfg(any(feature = "std", feature = "alloc"))]
        {
            self.inner = None;
        }
        #[cfg(not(any(feature = "std", feature = "alloc")))]
        self.inner.clear();
    }

    /// Iterate over the candidates
    #[inline]
    pub fn iter(&self) -> core::slice::Iter<'_, OofCandidate> {
        self.as_slice().iter()
    }

    /// Translate the static-position areas of every candidate in the list by `offset`.
    ///
    /// This is used to convert areas from being relative to a child node's border box to being
    /// relative to the parent node's border box as candidates bubble up the tree.
    #[inline]
    pub fn translate(&mut self, offset: Point<f32>) {
        for candidate in self.as_mut_slice() {
            candidate.static_position.x.area.start += offset.x;
            candidate.static_position.x.area.end += offset.x;
            candidate.static_position.y.area.start += offset.y;
            candidate.static_position.y.area.end += offset.y;
        }
    }
}

impl<'a> IntoIterator for &'a OofCandidates {
    type Item = &'a OofCandidate;
    type IntoIter = core::slice::Iter<'a, OofCandidate>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

/// The area of a container against which the insets of its out-of-flow (absolute/fixed) boxes
/// are resolved: the container's border box minus its borders and scrollbar gutters.
///
/// Container layout algorithms record this on [`LayoutOutput`] (for `RunMode::PerformLayout` runs)
/// so that the out-of-flow positioning pass ([`compute_oof_layout`](crate::compute_oof_layout))
/// can run after the algorithm has finished. When the container is a grid, per-box grid areas are
/// resolved within this area by the positioning pass itself (from the container's detailed grid
/// info).
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct OofPositioningArea {
    /// The size of the area
    pub size: Size<f32>,
    /// The offset of the area from the container's border box origin
    pub offset: Point<f32>,
}

/// A struct containing the result of laying a single node, which is returned up to the parent node
///
/// A baseline is the line on which text sits. Your node likely has a baseline if it is a text node, or contains
/// children that may be text nodes. See <https://www.w3.org/TR/css-writing-modes-3/#intro-baselines> for details.
/// If your node does not have a baseline (or you are unsure how to compute it), then simply return `Baselines::NONE`
/// for the baselines field
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct LayoutOutput {
    /// The size of the node
    pub size: Size<f32>,
    #[cfg(feature = "content_size")]
    /// The scrollable overflow rectangle of the node's content
    /// (see [`Layout::scrollable_overflow_rect`] for the coordinate conventions)
    pub scrollable_overflow_rect: Rect<f32>,
    /// The first and last baselines of the node in the horizontal axis, if any
    pub baselines: Baselines,
    /// Top margin that can be collapsed with. This is used for CSS block layout and can be set to
    /// `CollapsibleMarginSet::ZERO` for other layout modes that don't support margin collapsing
    pub top_margin: CollapsibleMarginSet,
    /// Bottom margin that can be collapsed with. This is used for CSS block layout and can be set to
    /// `CollapsibleMarginSet::ZERO` for other layout modes that don't support margin collapsing
    pub bottom_margin: CollapsibleMarginSet,
    /// Whether margins can be collapsed through this node. This is used for CSS block layout and can
    /// be set to `false` for other layout modes that don't support margin collapsing
    pub margins_can_collapse_through: bool,
    /// Out-of-flow (absolute/fixed) descendants of this node for which this node is *not* the
    /// containing block. These bubble up the tree until they reach their containing block, which
    /// lays them out. Empty (allocation-free) when there are no such descendants (the common case).
    ///
    /// Candidates are only produced by `RunMode::PerformLayout` passes.
    ///
    /// Container layout algorithms place the *full* document-ordered candidate list here (direct
    /// out-of-flow children plus candidates bubbled from in-flow children). The out-of-flow
    /// positioning pass ([`compute_oof_layout`](crate::compute_oof_layout)) then lays out those
    /// for which the node is the containing block and replaces this list with the unclaimed
    /// remainder.
    #[cfg_attr(feature = "serde", serde(skip_serializing))]
    pub oof_candidates: OofCandidates,
    /// The node's inset-resolution area for out-of-flow boxes. Set by container layout
    /// algorithms for `RunMode::PerformLayout` runs; `None` for leaves and size-only runs
    /// (in which case the out-of-flow positioning pass is a no-op).
    pub oof_positioning_area: Option<OofPositioningArea>,
}

impl LayoutOutput {
    /// An all-zero `LayoutOutput` for hidden nodes
    pub const HIDDEN: Self = Self {
        size: Size::ZERO,
        #[cfg(feature = "content_size")]
        scrollable_overflow_rect: Rect::ZERO,
        baselines: Baselines::NONE,
        top_margin: CollapsibleMarginSet::ZERO,
        bottom_margin: CollapsibleMarginSet::ZERO,
        margins_can_collapse_through: false,
        oof_candidates: OofCandidates::NONE,
        oof_positioning_area: None,
    };

    /// A blank layout output
    pub const DEFAULT: Self = Self::HIDDEN;

    /// Constructor to create a `LayoutOutput` from just the size, scrollable overflow rectangle and baselines
    pub fn from_sizes_and_baselines(
        size: Size<f32>,
        #[cfg_attr(not(feature = "content_size"), allow(unused_variables))] scrollable_overflow_rect: Rect<f32>,
        baselines: Baselines,
    ) -> Self {
        Self {
            size,
            #[cfg(feature = "content_size")]
            scrollable_overflow_rect,
            baselines,
            top_margin: CollapsibleMarginSet::ZERO,
            bottom_margin: CollapsibleMarginSet::ZERO,
            margins_can_collapse_through: false,
            oof_candidates: OofCandidates::NONE,
            oof_positioning_area: None,
        }
    }

    /// Construct a `LayoutOutput` from just the container size and scrollable overflow rectangle
    pub fn from_sizes(size: Size<f32>, scrollable_overflow_rect: Rect<f32>) -> Self {
        Self::from_sizes_and_baselines(size, scrollable_overflow_rect, Baselines::NONE)
    }

    /// Construct a `LayoutOutput` from just the container's size.
    pub fn from_outer_size(size: Size<f32>) -> Self {
        Self::from_sizes(size, Rect::ZERO)
    }
}

/// The final result of a layout algorithm for a single node.
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Layout {
    /// The relative ordering of the node
    ///
    /// Nodes with a higher order should be rendered on top of those with a lower order.
    /// This is effectively a topological sort of each tree.
    pub order: u32,
    /// The top-left corner of the node
    pub location: Point<f32>,
    /// The width and height of the node
    pub size: Size<f32>,
    #[cfg(feature = "content_size")]
    /// The scrollable overflow rectangle of the node: the axis-aligned rectangle containing the
    /// content of the node (the border boxes of its descendants plus their non-clipped overflow),
    /// corresponding to the CSS "scrollable overflow rectangle"
    /// (<https://www.w3.org/TR/css-overflow-3/#scrollable>), except that transforms are not
    /// accounted for.
    ///
    /// Coordinates are measured from the node's *scroll origin*: the corner of the padding box at
    /// the block-start/inline-start edge (the top-left corner in LTR, the top-*right* corner in
    /// RTL), with `left`/`right` measuring along the inline axis in the direction of reachable
    /// scrolling. The rectangle always contains the origin, so `left`/`top` are `<= 0.0` (negative
    /// values represent overflow before the scroll origin, which is unreachable by scrolling) and
    /// `right`/`bottom` are `>= 0.0` (representing the reachable extent of the content, which is
    /// useful for computing a "scroll width/height" for scrollable nodes).
    pub scrollable_overflow_rect: Rect<f32>,
    /// The size of the scrollbars in each dimension. If there is no scrollbar then the size will be zero.
    pub scrollbar_size: Size<f32>,
    /// The size of the borders of the node
    pub border: Rect<f32>,
    /// The size of the padding of the node
    pub padding: Rect<f32>,
    /// The size of the margin of the node
    pub margin: Rect<f32>,
}

impl Default for Layout {
    fn default() -> Self {
        Self::new()
    }
}

impl Layout {
    /// Creates a new zero-[`Layout`].
    ///
    /// The Zero-layout has size and location set to ZERO.
    /// The `order` value of this layout is set to the minimum value of 0.
    /// This means it should be rendered below all other [`Layout`]s.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            order: 0,
            location: Point::ZERO,
            size: Size::zero(),
            #[cfg(feature = "content_size")]
            scrollable_overflow_rect: Rect::ZERO,
            scrollbar_size: Size::zero(),
            border: Rect::zero(),
            padding: Rect::zero(),
            margin: Rect::zero(),
        }
    }

    /// Creates a new zero-[`Layout`] with the supplied `order` value.
    ///
    /// Nodes with a higher order should be rendered on top of those with a lower order.
    /// The Zero-layout has size and location set to ZERO.
    #[must_use]
    pub const fn with_order(order: u32) -> Self {
        Self {
            order,
            size: Size::zero(),
            location: Point::ZERO,
            #[cfg(feature = "content_size")]
            scrollable_overflow_rect: Rect::ZERO,
            scrollbar_size: Size::zero(),
            border: Rect::zero(),
            padding: Rect::zero(),
            margin: Rect::zero(),
        }
    }

    /// Get the width of the node's content box
    #[inline]
    pub fn content_box_width(&self) -> f32 {
        self.size.width - self.padding.left - self.padding.right - self.border.left - self.border.right
    }

    /// Get the height of the node's content box
    #[inline]
    pub fn content_box_height(&self) -> f32 {
        self.size.height - self.padding.top - self.padding.bottom - self.border.top - self.border.bottom
    }

    /// Get the size of the node's content box
    #[inline]
    pub fn content_box_size(&self) -> Size<f32> {
        Size { width: self.content_box_width(), height: self.content_box_height() }
    }

    /// Get x offset of the node's content box relative to it's parent's border box
    pub fn content_box_x(&self) -> f32 {
        self.location.x + self.border.left + self.padding.left
    }

    /// Get x offset of the node's content box relative to it's parent's border box
    pub fn content_box_y(&self) -> f32 {
        self.location.y + self.border.top + self.padding.top
    }
}

#[cfg(feature = "content_size")]
impl Layout {
    /// Return the maximum horizontal scroll offset of the node.
    /// This is the reachable extent of the content less the width of the padding box, floored at zero.
    pub fn scroll_width(&self) -> f32 {
        f32_max(
            0.0,
            self.scrollable_overflow_rect.right + f32_min(self.scrollbar_size.width, self.size.width) - self.size.width
                + self.border.left
                + self.border.right,
        )
    }

    /// Return the maximum vertical scroll offset of the node.
    /// This is the reachable extent of the content less the height of the padding box, floored at zero.
    pub fn scroll_height(&self) -> f32 {
        f32_max(
            0.0,
            self.scrollable_overflow_rect.bottom + f32_min(self.scrollbar_size.height, self.size.height)
                - self.size.height
                + self.border.top
                + self.border.bottom,
        )
    }
}

/// The additional information from layout algorithm
#[derive(Debug, Clone, PartialEq)]
pub enum DetailedLayoutInfo<S: CheapCloneStr = DefaultCheapStr> {
    /// Enum variant for [`DetailedGridInfo`](crate::compute::grid::DetailedGridInfo)
    #[cfg(feature = "grid")]
    Grid(crate::util::sys::Box<crate::compute::grid::DetailedGridInfo<S>>),
    /// For node that hasn't had any detailed information yet
    None,
    /// Unused variant which exists only to consume the `S` type parameter when no variant
    /// carrying detailed layout info is enabled
    #[cfg(not(feature = "grid"))]
    #[doc(hidden)]
    Phantom(core::convert::Infallible, core::marker::PhantomData<S>),
}
