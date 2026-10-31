//! Final data structures that represent the high-level UI layout
use crate::geometry::{AbsoluteAxis, Line, Point, Rect, Size};
use crate::style::{AlignItemsKeyword, AlignSelf, AlignmentSafety, AvailableSpace, CheapCloneStr, Position};
use crate::style_helpers::TaffyMaxContent;
use crate::sys::DefaultCheapStr;
use crate::tree::NodeId;
use crate::util::sys::{abs, copysign, f32_max, f32_min};

/// Scrollable overflow and the physical directions in which scrolling starts at the end edge.
///
/// The rectangle uses physical x/y coordinates relative to the client area's top-left edge.
/// To obtain border-box coordinates, add `border.left`/`border.top`, plus the horizontal
/// scrollbar gutter in RTL (where the vertical scrollbar is on the left). RTL does not mirror
/// these coordinates. For a scroll container, unreachable overflow is clipped: on normal axes
/// the start is zero, while on reversed axes the end is at most the client extent.
/// Non-scroll containers retain all overflow, for propagation and hit testing, and have no flags.
///
/// Rectangles contain `(0, 0)`: left/top are non-positive and right/bottom non-negative.
/// Algorithms accumulate ordinary rectangles from [`Rect::ZERO`] and pack them only at output.
/// The sign bits of the stored right/bottom encode reversal, including when an extent is zero.
/// Always use [`Self::rect`] before doing geometry with this value.
#[derive(Copy, Clone, Debug)]
pub struct ScrollableOverflow(Rect<f32>);

impl ScrollableOverflow {
    /// Empty overflow with normal scroll origins.
    pub const ZERO: Self = Self(Rect::ZERO);

    /// Pack an origin-containing rectangle and its physical scroll-origin directions.
    #[inline]
    pub fn new(rect: Rect<f32>, reversed: Point<bool>) -> Self {
        debug_assert!(rect.left <= 0.0 && rect.top <= 0.0 && rect.right >= 0.0 && rect.bottom >= 0.0);
        Self(Rect {
            right: copysign(rect.right, if reversed.x { -1.0 } else { 1.0 }),
            bottom: copysign(rect.bottom, if reversed.y { -1.0 } else { 1.0 }),
            ..rect
        })
    }

    /// The physical overflow rectangle, with the packed signs restored.
    #[inline(always)]
    pub fn rect(self) -> Rect<f32> {
        Rect { right: abs(self.0.right), bottom: abs(self.0.bottom), ..self.0 }
    }

    /// The physical left overflow edge.
    #[inline(always)]
    pub fn left(self) -> f32 {
        self.0.left
    }

    /// The physical right overflow edge, with its sign restored.
    #[inline(always)]
    pub fn right(self) -> f32 {
        abs(self.0.right)
    }

    /// The physical top overflow edge.
    #[inline(always)]
    pub fn top(self) -> f32 {
        self.0.top
    }

    /// The physical bottom overflow edge, with its sign restored.
    #[inline(always)]
    pub fn bottom(self) -> f32 {
        abs(self.0.bottom)
    }

    /// Whether the physical x/y scroll origins are at the right/bottom edges.
    #[inline(always)]
    pub fn is_reversed(self) -> Point<bool> {
        Point { x: self.0.right.is_sign_negative(), y: self.0.bottom.is_sign_negative() }
    }

    /// Swap the physical axes, preserving origin directions and signed zeroes.
    #[inline]
    pub fn transpose(self) -> Self {
        Self(self.0.transpose())
    }
}

impl From<(Rect<f32>, Point<bool>)> for ScrollableOverflow {
    #[inline]
    fn from((rect, reversed): (Rect<f32>, Point<bool>)) -> Self {
        Self::new(rect, reversed)
    }
}

impl Default for ScrollableOverflow {
    #[inline]
    fn default() -> Self {
        Self::ZERO
    }
}

impl PartialEq for ScrollableOverflow {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.0.left.to_bits() == other.0.left.to_bits()
            && self.0.right.to_bits() == other.0.right.to_bits()
            && self.0.top.to_bits() == other.0.top.to_bits()
            && self.0.bottom.to_bits() == other.0.bottom.to_bits()
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for ScrollableOverflow {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("ScrollableOverflow", 2)?;
        state.serialize_field("rect", &self.rect())?;
        state.serialize_field("reversed", &self.is_reversed())?;
        state.end()
    }
}

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

impl RequestedAxis {
    /// Swap the horizontal and vertical axes (`Both` is unchanged)
    #[inline(always)]
    pub fn transpose(self) -> RequestedAxis {
        match self {
            RequestedAxis::Horizontal => RequestedAxis::Vertical,
            RequestedAxis::Vertical => RequestedAxis::Horizontal,
            RequestedAxis::Both => RequestedAxis::Both,
        }
    }
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
    ///
    /// A definite available space is the space available to the node's border box: the parent
    /// subtracts the node's margins before passing it, and the node must not subtract them again.
    pub available_space: Size<AvailableSpace>,
    /// Yoga's `availableWidth`/`availableHeight` when a Yoga parent sizes this node with its
    /// `MaxContent` mode: the node sizes to its content, but its own children still see this size
    /// (Yoga's `availableInnerWidth`). `None` for every other constraint and for non-Yoga parents.
    #[cfg(feature = "yoga_flexbox")]
    pub yoga_available_size: Size<Option<f32>>,
    /// Specific to CSS Block layout. Used for correctly computing margin collapsing. You probably want to set this to `Line::FALSE`.
    pub vertical_margins_are_collapsible: Line<bool>,
}

impl LayoutInput {
    /// A LayoutInput that can be used to request hidden layout
    pub const HIDDEN: LayoutInput = LayoutInput {
        #[cfg(feature = "yoga_flexbox")]
        yoga_available_size: Size::NONE,
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

    /// Swap the horizontal and vertical axes of every per-axis input.
    ///
    /// Lets a tree lay out a node in a vertical writing mode: transpose the inputs, run the
    /// (horizontal-tb) algorithm, then transpose the [`LayoutOutput`] back.
    /// `vertical_margins_are_collapsible` is reset to `Line::FALSE` as margins cannot collapse
    /// across a change of block axis.
    pub fn transpose(self) -> LayoutInput {
        LayoutInput {
            #[cfg(feature = "yoga_flexbox")]
            yoga_available_size: Size::NONE,
            run_mode: self.run_mode,
            sizing_mode: self.sizing_mode,
            axis: self.axis.transpose(),
            known_dimensions: self.known_dimensions.transpose(),
            known_dimensions_are_definite: self.known_dimensions_are_definite.transpose(),
            parent_size: self.parent_size.transpose(),
            available_space: self.available_space.transpose(),
            vertical_margins_are_collapsible: Line::FALSE,
        }
    }
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
pub enum AxisStaticEdge {
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
pub struct AxisStaticAlign {
    /// The physical alignment keyword
    pub keyword: AxisStaticEdge,
    /// The `safe` overflow-position keyword from CSS Box Alignment: when `Safe`, the
    /// `fallback` keyword is used instead if the box's margin box overflows the area
    pub safety: AlignmentSafety,
    /// The physical alignment keyword to fall back to when `safety` is `Safe` and the box's
    /// margin box overflows the area
    pub fallback: AxisStaticEdge,
}

impl AxisStaticAlign {
    /// Create a `AxisStaticAlign` from a keyword with no safe fallback
    pub const fn from_keyword(keyword: AxisStaticEdge) -> Self {
        Self { keyword, safety: AlignmentSafety::Default, fallback: keyword }
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
pub struct AxisStaticPosition {
    /// The alignment container in this axis
    pub area: Line<f32>,
    /// The alignment to apply to the box's margin box within `area`
    pub align: AxisStaticAlign,
}

impl AxisStaticPosition {
    /// Create a `AxisStaticPosition` with a degenerate (zero-extent) area at `anchor` and no
    /// safe fallback
    pub const fn from_edge(anchor: f32, edge: AxisStaticEdge) -> Self {
        Self { area: Line { start: anchor, end: anchor }, align: AxisStaticAlign::from_keyword(edge) }
    }

    /// Create a `AxisStaticPosition` which aligns the box within `area` according to its
    /// self-alignment property in this axis (`justify-self` in the inline axis, `align-self` in
    /// the block axis). `normal` and `stretch` behave as `start`. This is the static position
    /// used by block and inline layout.
    ///
    /// `alignment` must already be resolved: `auto` (`None`) replaced by the container's
    /// `justify-items`/`align-items`, and `self-start`/`self-end` resolved against the
    /// container's direction (see [`AlignSelf::resolve_self_relative`]).
    /// `axis_is_rtl` is whether this axis is the inline axis of a right-to-left static-position
    /// containing block, in which case `start` and `end` are flipped.
    pub fn from_alignment(alignment: AlignSelf, area: Line<f32>, axis_is_rtl: bool) -> Self {
        debug_assert!(
            !matches!(alignment.keyword, AlignItemsKeyword::SelfStart | AlignItemsKeyword::SelfEnd),
            "self-start/self-end must be resolved before computing the static position"
        );
        /// The edge of the static-position rectangle that a (resolved) alignment keyword
        /// aligns the box to. Normal and Stretch (and anything else that is not an end or
        /// center alignment) fall back to start-alignment for static-position purposes.
        fn edge_for(keyword: AlignItemsKeyword, axis_is_rtl: bool) -> AxisStaticEdge {
            let start_position = !matches!(keyword, AlignItemsKeyword::End | AlignItemsKeyword::FlexEnd) ^ axis_is_rtl;
            match keyword {
                AlignItemsKeyword::Center => AxisStaticEdge::Center,
                _ if start_position => AxisStaticEdge::Start,
                _ => AxisStaticEdge::End,
            }
        }
        let fallback = if matches!(alignment.safety, AlignmentSafety::Safe) {
            AlignItemsKeyword::Start
        } else {
            alignment.keyword
        };
        Self {
            area,
            align: AxisStaticAlign {
                keyword: edge_for(alignment.keyword, axis_is_rtl),
                safety: alignment.safety,
                fallback: edge_for(fallback, axis_is_rtl),
            },
        }
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
    pub static_position: Point<AxisStaticPosition>,
    /// Whether the parent's main axis is a row (Yoga's absolute children of column containers fit
    /// the containing block width); set by Yoga parents only
    #[cfg(feature = "yoga_flexbox")]
    pub yoga_parent_is_row: bool,
}

/// A list of [`OofCandidate`]s.
///
/// This is a lazily-allocated heap vector (an empty list performs no allocation).
#[derive(Debug, Clone, PartialEq, Default)]
pub struct OofCandidates {
    /// Backing storage for the candidates
    inner: Option<crate::util::sys::Box<crate::util::sys::Vec<OofCandidate>>>,
}

impl OofCandidates {
    /// An empty list of candidates
    pub const NONE: Self = Self { inner: None };

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
        self.inner.as_ref().map(|boxed| boxed.as_slice()).unwrap_or(&[])
    }

    /// The candidates as a mutable slice
    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [OofCandidate] {
        self.inner.as_mut().map(|boxed| boxed.as_mut_slice()).unwrap_or(&mut [])
    }

    /// Append a candidate to the list
    #[inline]
    pub fn push(&mut self, candidate: OofCandidate) {
        self.inner.get_or_insert_with(Default::default).push(candidate);
    }

    /// Move all candidates from `other` into `self`, leaving `other` empty
    #[inline]
    pub fn append(&mut self, other: &mut OofCandidates) {
        if other.is_empty() {
            return;
        }
        match &mut self.inner {
            None => self.inner = other.inner.take(),
            Some(vec) => vec.append(other.inner.as_mut().unwrap()),
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
        self.inner = None;
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
    pub scrollable_overflow_rect: ScrollableOverflow,
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
        scrollable_overflow_rect: ScrollableOverflow::ZERO,
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
        #[cfg_attr(not(feature = "content_size"), allow(unused_variables))]
        scrollable_overflow_rect: ScrollableOverflow,
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
    pub fn from_sizes(size: Size<f32>, scrollable_overflow_rect: ScrollableOverflow) -> Self {
        Self::from_sizes_and_baselines(size, scrollable_overflow_rect, Baselines::NONE)
    }

    /// Swap the horizontal and vertical axes of the output (inverse of [`LayoutInput::transpose`]).
    ///
    /// Baselines and collapsible margins are block-axis quantities that have no meaning on the
    /// parent's inline axis, so they are dropped. Out-of-flow candidates are unaffected.
    pub fn transpose(self) -> Self {
        Self {
            size: self.size.transpose(),
            #[cfg(feature = "content_size")]
            scrollable_overflow_rect: self.scrollable_overflow_rect.transpose(),
            baselines: Baselines::NONE,
            top_margin: CollapsibleMarginSet::ZERO,
            bottom_margin: CollapsibleMarginSet::ZERO,
            margins_can_collapse_through: false,
            oof_candidates: self.oof_candidates,
            oof_positioning_area: self
                .oof_positioning_area
                .map(|area| OofPositioningArea { size: area.size.transpose(), offset: area.offset.transpose() }),
        }
    }

    /// Construct a `LayoutOutput` from just the container's size.
    pub fn from_outer_size(size: Size<f32>) -> Self {
        Self::from_sizes(size, ScrollableOverflow::ZERO)
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
    /// See [`ScrollableOverflow`] for physical coordinates and reversed-origin flags.
    /// A normal axis has range `0..scroll_width()` (or `scroll_height()`); a reversed axis
    /// has range `-scroll_width()..0` (or `-scroll_height()..0`). The initial offset is zero.
    pub scrollable_overflow_rect: ScrollableOverflow,
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
            scrollable_overflow_rect: ScrollableOverflow::ZERO,
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
            scrollable_overflow_rect: ScrollableOverflow::ZERO,
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
    /// Return the magnitude of the reachable horizontal scroll range.
    /// This is the reachable extent of the content less the width of the padding box, floored at zero.
    pub fn scroll_width(&self) -> f32 {
        if self.scrollable_overflow_rect.is_reversed().x {
            return -self.scrollable_overflow_rect.left();
        }
        f32_max(
            0.0,
            self.scrollable_overflow_rect.right() + f32_min(self.scrollbar_size.width, self.size.width)
                - self.size.width
                + self.border.left
                + self.border.right,
        )
    }

    /// Return the magnitude of the reachable vertical scroll range.
    /// This is the reachable extent of the content less the height of the padding box, floored at zero.
    pub fn scroll_height(&self) -> f32 {
        if self.scrollable_overflow_rect.is_reversed().y {
            return -self.scrollable_overflow_rect.top();
        }
        f32_max(
            0.0,
            self.scrollable_overflow_rect.bottom() + f32_min(self.scrollbar_size.height, self.size.height)
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
    /// Enum variant for [`YogaDetailedInfo`](crate::compute::yoga_flexbox::YogaDetailedInfo)
    #[cfg(feature = "yoga_flexbox")]
    YogaFlexbox(crate::compute::yoga_flexbox::YogaDetailedInfo),
    /// For node that hasn't had any detailed information yet
    None,
    /// Unused variant which exists only to consume the `S` type parameter when no variant
    /// using it is enabled
    #[cfg(not(feature = "grid"))]
    #[doc(hidden)]
    Phantom(core::convert::Infallible, core::marker::PhantomData<S>),
}

#[cfg(test)]
mod scrollable_overflow_tests {
    use super::ScrollableOverflow;
    use crate::geometry::{Point, Rect};

    #[test]
    fn side_accessors_match_restored_rectangle() {
        for rect in [
            Rect::ZERO,
            Rect { left: -0.0, top: -0.0, right: 0.0, bottom: 0.0 },
            Rect { left: -10.0, top: -20.0, right: 30.0, bottom: 40.0 },
        ] {
            for x in [false, true] {
                for y in [false, true] {
                    let overflow = ScrollableOverflow::new(rect, Point { x, y });
                    let restored = overflow.rect();
                    assert_eq!(overflow.left().to_bits(), restored.left.to_bits());
                    assert_eq!(overflow.right().to_bits(), restored.right.to_bits());
                    assert_eq!(overflow.top().to_bits(), restored.top.to_bits());
                    assert_eq!(overflow.bottom().to_bits(), restored.bottom.to_bits());
                }
            }
        }
    }

    #[test]
    fn packing_preserves_flags_rectangles_and_signed_zeroes() {
        for right in [0.0, 10.0] {
            for bottom in [0.0, 20.0] {
                for x in [false, true] {
                    for y in [false, true] {
                        let rect = Rect { left: -0.0, right, top: -0.0, bottom };
                        let reversed = Point { x, y };
                        let overflow = ScrollableOverflow::new(rect, reversed);
                        assert_eq!(overflow.left().to_bits(), (-0.0_f32).to_bits());
                        assert_eq!(overflow.top().to_bits(), (-0.0_f32).to_bits());
                        assert_eq!(overflow.right().to_bits(), right.to_bits());
                        assert_eq!(overflow.bottom().to_bits(), bottom.to_bits());
                        assert_eq!(overflow.is_reversed(), reversed);
                        assert_eq!(overflow.0.right.is_sign_negative(), x);
                        assert_eq!(overflow.0.bottom.is_sign_negative(), y);
                        assert_eq!(overflow.transpose().is_reversed(), Point { x: y, y: x });
                        assert_eq!(overflow.transpose().transpose(), overflow);
                    }
                }
            }
        }
        assert_eq!(ScrollableOverflow::default(), ScrollableOverflow::ZERO);
        assert_ne!(ScrollableOverflow::ZERO, ScrollableOverflow::new(Rect::ZERO, Point { x: true, y: false }));
        assert_ne!(
            ScrollableOverflow::ZERO,
            ScrollableOverflow::new(Rect { left: -0.0, ..Rect::ZERO }, Point { x: false, y: false })
        );
        assert_ne!(
            ScrollableOverflow::ZERO,
            ScrollableOverflow::new(Rect { top: -0.0, ..Rect::ZERO }, Point { x: false, y: false })
        );
    }

    #[test]
    #[cfg(debug_assertions)]
    #[should_panic]
    fn packing_rejects_invalid_rectangles() {
        ScrollableOverflow::new(Rect { left: 1.0, ..Rect::ZERO }, Point { x: false, y: false });
    }

    #[test]
    #[cfg(feature = "serde")]
    fn serialization_exposes_rect_and_flags_not_packed_signs() {
        let overflow = ScrollableOverflow::new(
            Rect { left: -10.0, right: 20.0, top: 0.0, bottom: 0.0 },
            Point { x: true, y: true },
        );
        assert_eq!(
            serde_json::to_value(overflow).unwrap(),
            serde_json::json!({
                "rect": { "left": -10.0, "right": 20.0, "top": 0.0, "bottom": 0.0 },
                "reversed": { "x": true, "y": true }
            })
        );
    }
}
