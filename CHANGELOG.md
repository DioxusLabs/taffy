# Changelog

## Unreleased

### Added

- Support for `flex-wrap: balance` and the `flex-line-count` property from [CSS Flexbox Level 2](https://drafts.csswg.org/css-flexbox-2/#balance-values), gated behind a new on-by-default `flexbox_balance` cargo feature (which depends on `flexbox`):
  - `FlexWrap` gains `Balance` and `BalanceReverse` variants, and its CSS parser accepts the multi-keyword grammar `nowrap | [ wrap | wrap-reverse ] || balance`. When balancing, items are collected into flex lines such that the largest line is as small as possible (minimising the sum of squares of each line's free space), rather than greedily filling each line
  - `Style::flex_line_count` (and a corresponding `FlexboxContainerStyle::flex_line_count` trait method) specifies the minimum number of lines to balance items into (default `1`). When it is greater than 1 on any multi-line container (`wrap`, `wrap-reverse`, `balance` or `balance-reverse`), definite cross-axis available space for measuring items is divided between the requested number of lines (after subtracting cross-axis gaps), per the [CSSWG resolution](https://github.com/w3c/csswg-drafts/issues/13414)

- `Dimension` (used for `size`) now supports the sizing keywords `min-content`, `max-content`, `fit-content`, `fit-content(<length-percentage>)` and `stretch` via new constructors (`Dimension::min_content()`, `Dimension::max_content()`, `Dimension::fit_content()`, `Dimension::fit_content_px()`, `Dimension::fit_content_percent()`, `Dimension::stretch()`), new `CompactLength::FIT_CONTENT_KEYWORD_TAG`/`CompactLength::STRETCH_TAG` representations, and CSS parsing support. These keywords are resolved:
  - In block layout: for the widths of in-flow children and floats
  - In flexbox layout: for the main-axis size when determining an item's flex base size, for the cross-axis size when determining an item's hypothetical cross size, and for `flex-basis` (where `stretch` resolves to the stretch-fit main size and the other keywords determine the sizing constraint the item is measured under when computing its flex base size)
  - In grid layout: for the width/height of grid items, both during track sizing (content contributions) and during final item sizing/alignment. A keyword-sized axis is treated as non-`auto`, so the default `normal` self-alignment resolves to `start` rather than `stretch` in that axis

  In contexts where a keyword cannot be resolved it behaves as `auto`

- All built-in layout algorithms (flexbox, grid and block) now compute and output the *last baseline* of a container (`LayoutOutput::baselines.last`) in addition to its first baseline. A flex container's last baseline is generated from the last item of its cross-end-most line (preferring items participating in last-baseline alignment), a grid container's from the last row containing items, and a block container's from its last in-flow child with a baseline. Last baselines reported by children (e.g. by measure functions) are propagated up the tree, with scroll containers' baselines clamped to their border box

- Support for *last-baseline alignment* (CSS `align-items: last baseline` / `align-self: last baseline`) in flexbox and grid layout. `AlignItemsKeyword` gains a `LastBaseline` variant, with a corresponding `AlignItems::LAST_BASELINE` constant, `"LastBaseline"` serde representation and CSS parsing support for `last baseline` (and `first baseline` as an alias of `baseline`). Items with `align-self: last baseline` form a separate baseline group from first-baseline items: within a flex line or grid row their last baselines are aligned, with the group anchored towards the cross-end/block-end of the line or row. As with first-baseline alignment, flex items with `auto` cross-axis margins do not participate, missing baselines are synthesized from the item's border box, and scroll containers' baselines are clamped to their border box. In flex columns (where horizontal baselines cannot be aligned along the cross axis) last-baseline items fall back to being anchored to the cross-end edge of the line

- `Dimension` also supports the `content` keyword (`Dimension::content()`, CSS `content`), which indicates an automatic size based on the box's content. This keyword is only valid for `flex-basis`, where it sizes the item based on its content (ignoring its main size property) when computing its flex base size. In any other context (e.g. `width`/`height`) it behaves as `auto`

- Support for the layout-affecting parts of the `layout` and `paint` values of the CSS `contain` property via a new `Contain` bitflags style type, a new `Style::contain` field and a new (defaulted) `CoreStyle::contain` trait method:
  - `Contain::LAYOUT` (layout containment): the box establishes an independent formatting context (its margins do not collapse with those of its children, it contains its own floats, and it avoids external floats) and its baseline is suppressed (boxes requiring a baseline synthesize one from its border box)
  - `Contain::PAINT` (paint containment): the box establishes an independent formatting context like layout containment, but its baseline is not suppressed. Paint containment's other effects (clipping, containing absolutely-positioned descendants, stacking context) do not affect layout and are not implemented
  - Layout and paint containment also prevent the box's overflowing content from contributing to its ancestors' scrollable overflow regions: layout containment treats such overflow as ink overflow, and paint containment clips it. The contained box's own `scrollable_overflow_rect` still includes its overflowing content

  The CSS parser (`parse` feature) accepts `none | content | [ layout || style || paint ]` for the `contain` property: `content` maps to `LAYOUT | PAINT`, and the `style` keyword is accepted but ignored as it does not affect layout. The `strict`, `size` and `inline-size` values are not supported as size containment is not implemented

- `DetailedGridTracksInfo` (behind the `detailed_layout_info` feature) gains a `line_names: GridLineNames` field containing the names of each grid line in the axis, index-aligned with `positions` (line `i` bounds the start of track `i`, in logical order). Names include those from `grid-template-rows`/`grid-template-columns` (with `repeat()`s expanded using the resolved auto-repetition count) and the implicit `<name>-start`/`<name>-end` names generated by `grid-template-areas`. The new `GridLineNames` type stores names in a compact CSR representation (one flat `Vec` of names plus a `Vec` of offsets) and exposes them via `iter()` (yielding one `&[S]` name group per line) and `line(index)`. Both detailed-info structs are now generic over the custom identifier string type (defaulting to the default string type), and `LayoutGridContainer::set_detailed_grid_info` now takes `DetailedGridInfo<Self::CustomIdent>`

- `DetailedGridInfo` (behind the `detailed_layout_info` feature) gains `write_grid_template_rows()`/`write_grid_template_columns()` methods generic over any `core::fmt::Write` writer (plus allocating `grid_template_rows()`/`grid_template_columns()` conveniences, and per-axis `DetailedGridTracksInfo::write_track_list()`/`to_track_list_string()`) which serialize the used track sizes and line names in the [resolved value format](https://www.w3.org/TR/css-grid-1/#resolved-track-list) of the corresponding CSS properties, e.g. `[full-start main-start] 120px [main-end] 480px [full-end]`

- `DetailedGridInfo` (behind the `detailed_layout_info` feature) gains an `item_grid_area(item_index)` method returning the location and size (`(Point<f32>, Size<f32>)`) of the grid area occupied by an item, relative to the grid container's border box

### Changed

- `DetailedGridTracksInfo` (behind the `detailed_layout_info` feature) now exposes a single `positions: Vec<Line<f32>>` field containing the start and end position of each track relative to the grid container's border box, replacing the previous `gutters` and `sizes` fields. Unlike the previous fields, these positions account for content alignment (`align-content`/`justify-content`). Collapsed tracks are included as zero-width entries, so indices remain 1:1 with track numbers. Track sizes and gutters can be derived from the positions (`size = end - start`; gutter = distance between adjacent tracks)

- Grid: for `direction: rtl` grid containers, all internal grid data structures and the `detailed_layout_info` output (`DetailedGridInfo`) are now in *logical* order (line 1 = inline-start = the right-hand side in RTL), matching LTR. Previously RTL grids were internally mirrored into visual order, so `DetailedGridTracksInfo` column track positions and `DetailedGridItemsInfo` column line numbers were reported in visual (left-to-right) order. RTL is now applied purely when assigning physical geometry, and the rendered layout is unchanged

- Flexbox/Block: absolutely positioned children are no longer measured when both of their dimensions are already known (e.g. from explicit sizes or insets), matching the existing grid behaviour

### Fixed

- Block/float: the height of overflowing in-flow content of a nested block no longer contributes to the height of the block formatting context root as if it were floated content. Previously an auto-height BFC root containing a block whose in-flow content overflowed it (e.g. a fixed-height block with taller content) was incorrectly extended to contain that overflowing content

- Block: the baselines contributed by in-flow children that are scroll containers are now clamped to the child's border box, and a scroll-container child with no natural baseline synthesizes one at its border-box bottom edge, matching the existing flexbox/grid behaviour per the [CSSWG resolution](https://github.com/w3c/csswg-drafts/issues/7660). Previously baselines of clipped content could leak outside an `overflow: hidden` child, and an empty scroll-container child contributed no baseline

- Grid: items with an `auto` block-axis margin no longer participate in baseline alignment, per [CSS Align §9.5](https://www.w3.org/TR/css-align-3/#baseline-align-self). Such items are no longer baseline-shimmed (their auto margin aligns them instead), and they are no longer selected as the item the grid container's own first baseline is generated from, matching the existing flexbox behaviour

- Flexbox: a flex item's cross-axis size is now only treated as definite (for resolving percentage sizes of its descendants) when the item is stretched or its cross size style resolves to a definite size, per [CSS Flexbox §4.5](https://www.w3.org/TR/css-flexbox-1/#definite-sizes). Previously content-derived cross sizes of non-stretched items were incorrectly used as percentage resolution bases

- Block: a container's `min-height` alone no longer acts as the resolution basis for the percentage heights of its children when the container's own height is indefinite. Per [CSS 2 §10.5](https://www.w3.org/TR/CSS22/visudet.html#the-height-property), such percentages resolve as `auto`

- Block: percentage vertical insets (`top`/`bottom`) of relatively positioned children now resolve against the containing block's height when it is definite. Previously they always resolved against a zero basis, so e.g. `top: 50%` had no effect

- Flexbox: skip the automatic min-content measurement when an item's minimum main size is already resolved from its style or overflow. Previously this fallback was evaluated eagerly and could cause nested flex layouts with explicit minimum sizes to perform unnecessary recursive measurements

- Block/Flexbox/Grid: content overflowing a box past its scroll origin (the inline-start/block-start edge, e.g. an absolutely positioned child with a negative position) no longer inflates the box's `content_size`. Per the [CSS Overflow spec](https://www.w3.org/TR/css-overflow-3/#scrollable), the scrollable overflow region only extends from the scroll origin towards the inline-end/block-end directions, so such content is unreachable and does not contribute to scrollable overflow. Additionally, block layout now correctly measures absolutely positioned children from the inline-end edge in RTL when computing their `content_size` contribution, matching the existing flexbox and grid behaviour

- Flexbox: free space absorbed by main-axis `auto` margins is no longer also distributed by `justify-content`. Previously an item with e.g. `margin: 0 auto` inside a `justify-content: center` container was pushed past the centre by an extra half of the free space, as the same free space was counted twice (once by the auto margins and once by the alignment step). Per [CSS Flexbox §9.5](https://www.w3.org/TR/css-flexbox-1/#algo-main-align), auto margins consume all of the positive free space, leaving none for `justify-content`

- Block/Flexbox/Grid: a container's own end-side padding (right padding for LTR, left padding for RTL, and bottom padding) is now only included in its `content_size` when the container is a scroll container (i.e. has `overflow` other than `visible`/`clip` in either axis). Per the [CSS Overflow spec](https://www.w3.org/TR/css-overflow-3/#scrollable), boxes that are not scroll containers do not extend their scrollable overflow region by their own padding, so overflowing content within an ordinary padded box no longer propagates spuriously enlarged content sizes to ancestor scroll containers

- Leaf: `compute_leaf_layout` now follows the same convention as block/flexbox/grid for its `content_size`: the node's own end-side padding (right padding for LTR, left padding for RTL, and bottom padding) is only included when the node is a scroll container. Previously both the start and end padding were included unconditionally, over-reporting the content size of ordinary padded leaves whose measured content overflows

- Grid: the first baselines of grid items which are scroll containers are now clamped to the item's border box (matching the existing flexbox behaviour, per the [CSSWG resolution](https://github.com/w3c/csswg-drafts/issues/7660))

- Flexbox: items with `align-self: baseline` and an `auto` cross-axis margin no longer participate in baseline alignment, per [CSS Flexbox §8.3](https://www.w3.org/TR/css-flexbox-1/#baseline-participation). Previously such items were still counted when deciding whether a line performs baseline alignment, had their baselines measured, and could affect the container's own first baseline

### Changed

- The `content_size: Size<f32>` field of `Layout` and `LayoutOutput` (behind the `content_size` cargo feature) has been replaced with `scrollable_overflow_rect: Rect<f32>`, representing the CSS [scrollable overflow rectangle](https://www.w3.org/TR/css-overflow-3/#scrollable) (excluding the effect of transforms, which Taffy has no knowledge of). Coordinates are measured from the node's scroll origin (the inline-start/block-start corner of its padding box — the top-*right* corner in RTL — with `left`/`right` measuring along the inline axis in the direction of reachable scrolling), so the rectangle always contains the origin: negative `left`/`top` values capture start-side overflow that is unreachable by scrolling, while `right`/`bottom` give the reachable extent of the content (equivalent to the old `content_size`). Boxes wholly in the unreachable scrollable overflow region of a scroll container are excluded from its scrollable overflow rectangle entirely, while boxes only partially in the unreachable region contribute their whole border box. `Layout::scroll_width()`/`Layout::scroll_height()` are unchanged

  Migration: replace reads of `layout.content_size.width`/`layout.content_size.height` with `layout.scrollable_overflow_rect.right`/`layout.scrollable_overflow_rect.bottom`, and `LayoutOutput::from_sizes(size, content_size)` with `LayoutOutput::from_sizes(size, Rect { left: 0.0, right: content_size.width, top: 0.0, bottom: content_size.height })`

- `LayoutOutput`'s `first_baselines: Point<Option<f32>>` field has been replaced with `baselines: Baselines`, where the new `Baselines` struct has `first: Option<f32>` and `last: Option<f32>` fields (both horizontal baselines, measured from the top edge of the node). The never-read vertical (x) baseline slot has been dropped in favour of a last-baseline slot. Last baselines are not yet computed by any of the built-in layout algorithms (this is purely a representational change), but custom tree implementations and measure functions can now report them

  Migration: replace `first_baselines: Point { x: None, y: baseline }` with `baselines: Baselines::from_first(baseline)`, and reads of `first_baselines.y` with `baselines.first`

- `Style::min_size` and `Style::max_size` (and the corresponding `CoreStyle::min_size`/`CoreStyle::max_size` trait methods) are now `Size<LengthPercentageAuto>` rather than `Size<Dimension>`, as the min/max sizing properties do not support the new sizing keywords that `Dimension` now supports

- `TaffyTree::compute_layout_with_measure`'s measure function now takes the full `LayoutInput` (plus `NodeId`, `Option<&mut NodeContext>` and `&Style`) and returns a `LayoutOutput` directly instead of a `Size<f32>`, allowing measure functions to set baselines (and other `LayoutOutput` fields) on leaf nodes. `compute_leaf_layout` is no longer called implicitly (#953)

  Migration: to retain the previous behaviour, wrap your existing measure logic in an explicit call to `compute_leaf_layout` within the new-style measure function:

  ```rust
  // Before
  tree.compute_layout_with_measure(node, available_space, |known_dimensions, available_space, node_id, node_context, style| {
      my_measure_logic(known_dimensions, available_space, node_id, node_context, style)
  })?;

  // After
  tree.compute_layout_with_measure(node, available_space, |inputs, node_id, node_context, style| {
      taffy::compute_leaf_layout(inputs, style, |_, _| 0.0, |known_dimensions, available_space| {
          my_measure_logic(known_dimensions, available_space, node_id, node_context, style)
      })
  })?;
  ```

### Fixed

- `TaffyTree::remove` now marks the removed node's former parent as dirty, like `remove_child`, `remove_child_at_index` and `remove_children_range` already did. Previously the parent and its ancestors kept their stale cached layout, so recomputing the layout of an ancestor did not account for the removed node (#998)

- Grid: fixed a subtract-with-overflow panic (in debug builds) when resolving named lines for a template containing a repetition with fewer line name sets than tracks. Any template combining line names with a repetition created by the `repeat()` style helper (or parsed from CSS such as `[a] repeat(2, 10px) [c] 10px`) could trigger this. In release builds the same bug silently mis-numbered the lines following the repetition. The length of `GridTemplateRepetition::line_names` is now part of the API contract: it must either be empty (all lines unnamed) or contain exactly `tracks.len() + 1` line name sets, and any other length panics (in all builds) during layout

- Grid: the track sizing algorithm no longer loops forever when styles contain non-finite values (e.g. `NaN` flex factors or non-finite lengths). The "find the size of an fr" and "distribute space up to limits" loops are now bounded by the track count

- Grid: fixed an infinite loop in auto-placement when an item's placement resolved to a span larger than the implicit grid size estimate (e.g. a zero span — an invalid value which is now normalized to 1 — combined with an unresolvable named span). The auto-placement search now bails out (and clamps the placement into the limited grid) when a span cannot fit even at the start of the grid

- Flexbox: the definiteness of known dimensions is now tracked through nested layouts via a new `known_dimensions_are_definite` field on `LayoutInput`. A flex item's post-flexing main size is only treated as definite (for resolving percentage sizes of its children and for collecting its children into flex lines) when the container's main size is definite or the item's used flex basis is definite. Previously a wrapping flex container nested in a container with an indefinite main size could incorrectly wrap its items into multiple lines based on its own content-derived size (#999)

- Flexbox: percentage heights of block descendants no longer resolve against a flex item's post-flexing main size when that size is indefinite (#950)

- Flexbox: a wrapping container with an indefinite main size now wraps its items against its max main size (e.g. `max-width` for a row container) when the available space exceeds it. Previously items were collected into flex lines using the raw available space, so a fit-content sized container (such as a float) with a `max-width` smaller than the available space never wrapped

## 0.13.0

The MSRV for this release is 1.71.

### Added

- Support for the `self-start` and `self-end` alignment keywords (`AlignItems::SELF_START`/`SELF_END` and safe variants). These resolve against the `direction` of the item itself rather than that of its container, so they only differ from `start`/`end` when the item's direction differs from its container's. Supported for `align-self`/`align-items` and `justify-self`/`justify-items` on both in-flow and absolutely positioned Flexbox and Grid items (#1074)

- Support for `display: flow-root`. The new `Display::FlowRoot` variant lays out children using the block layout algorithm but always establishes a new block formatting context (its margins do not collapse with those of its children, it contains its own floats, and it avoids external floats)

### Changed

- Numeric style helpers (`length`, `percent`, `fr`, `flex`) now accept `Input: Into<f64>` instead of `Input: Into<f32>`. This allows bare float literals such as `length(800.0)` to be used without triggering the `float_literal_f32_fallback` future-compatibility lint, while widening the set of accepted numeric input types (#974)
- Grid: `grid_template_areas` is now `Option<GridTemplateAreas<S>>`, where the new `GridTemplateAreas` struct includes `row_count`/`column_count` fields. This allows templates containing unnamed (`.`) cells beyond the extents of the named areas (e.g. `grid-template-areas: "a ."`) to be represented.
- Block/float: `BlockContext::place_floated_box` takes an additional `adjoins_unresolved_strut: bool` parameter indicating whether the float is being placed while the position of the current margin-collapse strut is still unresolved

### Fixed

A large number of miscelaneous bug fixes are included in this release:

- Flexbox: clamp the flex base size, automatic minimum size and hypothetical main/cross sizes with min/max sizes transferred through the aspect ratio, instead of baking them into the item's used min/max sizes (#989)
- Flexbox: resolve `justify-content: start`/`end` and `align-self: start`/`end`/`self-start`/`self-end` as writing-mode relative (rather than flex-relative) in the static position of absolutely positioned children; use the flex-relative start for `justify-content: space-between`/`normal`, and a fallback of `start` for `align-self: baseline` (#1072)
- Flexbox: only let `auto` margins on absolutely positioned children absorb free space when the box is inset-constrained in that axis; otherwise they resolve to zero per CSS2 §10.3.7/§10.6.4 (#1072)
- Grid: jump the auto-placement search past entire colliding occupied intervals rather than advancing one track at a time (#1038)
- Grid: resolve the grid lines used by absolutely positioned items to the edges of the adjacent tracks, so that free space distributed by `align-content`/`justify-content` is not included in the abspos grid area (#1071)
- Grid: exclude absolutely positioned children from the implicit grid size estimate, as they do not take part in grid placement and must not create implicit tracks. Previously their out-of-range lines resolved to phantom implicit tracks instead of being treated as `auto` (#1075)
- Grid: align an empty grid (one whose tracks have all collapsed) within its container, rather than always placing it at the start (#1078)
- Grid: only distribute space "beyond limits" to tracks whose *max* track sizing function is `max-content` (or `fit-content()`) (#1033)
- Grid: don't clamp an explicitly specified preferred or minimum size by the spanned tracks' fixed max track sizing functions when computing an item's minimum contribution (#1022)
- Grid: ignore the tracks' growth limits when distributing an item's intrinsic size contribution "beyond limits" (#1022)
- Grid: convert the container's min/max size to a content-box size before using it in the track sizing algorithm (#1023)
- Grid: compute the flex factor sum over only the spanned tracks eligible to receive space when distributing intrinsic size contributions to flexible tracks (#1019)
- Grid: don't grow tracks past their growth limits when distributing free space to multiple tracks with asymmetric limits (#1001)
- Block/Grid: measure `Layout::content_size` from the container's padding-box origin and include the container's own end-side padding, matching Flexbox and browser `scrollWidth`/`scrollHeight` semantics. Grid items in tracks that overflow the container now also contribute their position within the container, not just their own size (#1051)
- Block: don't stretch-size replaced elements; an auto width now resolves to the intrinsic size (#1002)
- Block: don't let boxes that establish an independent formatting context overlap floats: they narrow to fit beside the float, or move down below it if they don't fit (#991)
- Block: detect floats placed by the subtree of a preceding in-flow sibling, not just floats among direct siblings, when placing a box that establishes an independent formatting context (#1049)
- Block: apply a negative margin as usual on a BFC root's float-free side, instead of clamping it to the containing block edge (#1061)
- Block: compute clearance from the hypothetical position of the cleared element (including its collapsed top margin), supporting negative clearance (#1042)
- Block: prevent the cleared element's top margin from collapsing with preceding margins and with the parent's top margin (#1043)
- Block: force clearance for floats placed while the position of the enclosing margin-collapse strut is unresolved, and position such floats including the pending collapsible margins (#1046)
- Block: collapse the top and bottom margins of a self-collapsing element with clearance with each other and apply them inside the parent, rather than collapsing them with the parent's bottom margin (#1044)
- Block/float: treat `clear` as a no-op when no float has been placed on the relevant side(s) (#1041)
- Block: allow elements containing only floated children to be collapsed through (#1040)
- Block/float: record zero-width floats in the float context, so their edge acts as an obstacle for boxes establishing an independent formatting context (#1062)
- Block/float: a float establishes a new block formatting context, so its margins no longer collapse with the margins of its children (#1065)
- Block/float: honour CSS2 §9.5.1 rules 3 & 7 when placing floats: a float unconstrained by other floats may overflow its containing block, but one placed beside another float may not (#1064)
- Block/float: apply CSS2 §9.5.1 rule 5 (float ceiling) and `clear` past zero-sized floats, which occupy no float segment and so were previously ignored when positioning later floats and cleared elements (#1056)
- Block/float: sum float contributions when computing a float container's intrinsic width under definite available space (clamped between the widest single float and the available width), instead of dropping them entirely (#1055)
- Block: a block container with a non-`normal` `align-content` establishes an independent formatting context (its margins do not collapse with its children's, it cannot be collapsed through, and it contains its own floats)
- Block/float: `align-content` shifts a block container's floated children along with its in-flow content

## 0.12.2

### Fixed

- Block: return margin-collapsing outputs from vertical axis ComputeSize calls (#976)

## 0.12.1

This release container a couple of critical fixes for layout/caching bugs in the 0.12.0 release.

### Fixed

- Block: don't commit deferred in-flow layouts to the tree when only computing size (#971)
- Block: pass through the requested `run_mode` when performing final layout on in-flow children, instead of always using `MeasureSize` (#972)

## 0.12.0

The MSRV for this release is 1.71.

### Block: support for `align-content` (#959)

Block containers now implement `align_content` along the block axis for their in-flow children.

### More correct caching logic

- The cache key now includes the axis, parent size, and available space, and ignores available space in an axis when a known dimension is set there. This is a performance hit (~10% in common cases, ~60% in pathalogically ones) but is necessary for correctness. It does also enable early-return optimizations (in cases where only the horizontal size is needed, which can allow that performance to be recouped in some cases (#911)

### Fixed

- Flexbox: fall back to safe `align-self` of `start` on absolute-position overflow (#958)
- Block: derive definite height from `aspect-ratio` at final layout. A block container with `aspect-ratio` and an automatic height now becomes definite when its width is filled/stretched, so children's percentage heights resolve correctly and the ratio is preserved (#965)

## 0.11.0

The MSRV for this release is 1.71.

### Implemented safe alignment keywords (#952)

Taffy now implements [safe alignment](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/align-items#safe) (in addition to unsafe alignment).

The alignment style types are now structs consisting of an `AlignmentKeyword` and an `AlignmentSafety` modifier. For most users this will mean changing from using enum variants like `AlignContent::Start` to associated constants like `AlignContent::START`.

This change applies to the `AlignContent`, `JustifyContent`, `AlignItems`, `JustifyItems`, `AlignSelf`, and `JustifySelf` types.

### Fixed

- Grid: resolve item percentages against grid area rather than grid container (#960)

## 0.10.1

### Fixed

- CSS Grid auto-repeat and minimum-size handling (#946)

## 0.10.0

The MSRV for this release is 1.71.

### Support for `direction`

The `direction` property is now supported, allowing for RTL layout of boxes in Block, Flexbox, and CSS Grid layout modes.

### Support for floats

The `float` and `clear` properties are now supported. Support consists of a general-purpose `FloatContext` in the `compute` module, and integration of float layout into Block layout. Block layout now also has a `BlockContext` that allows a `FloatContext` to be shared across an entire Block formatting context.

Float support is feature flagged by the `float_layout` feature.

### Support for parsing styles from CSS string (#929)

All of Taffy's style types (except the top-level `Style` struct) now have `FromStr` implementations that parses the type from the CSS representation of that value (e.g. `30px` or `50%` for `LengthPercentage`. A future version of Taffy will likely add support for parsing `Style` from `;`-seperated CSS.

CSS parsing is feature flagged by the `parse` feature.

Additionally the `parse_faster` feature enables optimizations for faster parsing at the cost of pulling in proc-macro dependencies such as `syn`.

### Changed

- Make DetailedGridTracksInfo accessible from a public module (#899)
- Add `TaffyTree::write_tree` method to debug print the tree into an arbitrary writer (#925)
- The cache `set` and `set` APIs now take `&LayoutInput` rather than individual values (#933)

### Fixed

- Flexbox: apply gap even when there are auto margins (#938)

## 0.9.3

### Added

- Added write_tree method to utils.

## 0.9.2

### Fixed

- Fix wrong size propogation for absolute elements (#878)
- Fix bounds check in CellOccupancyMatrix::last_of_type (#890)
- Use doc_cfg instead of doc_auto_cfg (#868)

### Changed

- Upgraded grid dependency from 0.18 to 1.0 (#864)

## 0.9.1

### Fixed

- Flexbox: don't apply cross-axis stretch alignment to children with auto margins (#861)

## 0.9.0

The MSRV for this release is 1.65.

### Support for named grid lines and grid areas

Taffy now supports named grid lines and areas.

As these rely on arbitrary user-provided strings, Taffy's `Style` struct is now generic
over a string-like type (via the `CheapCloneStr` trait). Additionally as the `grid` feature is optional,
it has a `PhantomData` field of that type to make type inference work.

### Changed

- `PrintTree` and `RoundTree`: use `Layout` instead of `&Layout` (#849).
- Renamed `TrackSizingFunction` to `GridTemplateComponent`
- Renamed `NonRepeatedTrackSizingFunction` to `TrackSizingFunction`
- The `Repeat` variant of `GridTemplateComponent` now contains a new `GridTemplateRepetition` struct, which allows
  line names to be specifed in addition to tracks.
- The way that grid styles are exposed in the low-level API is now a lot more generic with many associated types.

### Added

- `GridTemplateArea` struct and `Style::grid_template_areas` field
- `Style::grid_template_column_names` and `Style::grid_template_row_names` fields. If non-empty, these
  should have length of exactly one greater than the corresponding `grid_template_column`/`grid_template_rows` style.

## 0.8.3

### Fixed

- Fix `serde` feature on 32bit targets (#845)

## 0.8.2

### Fixed

- Fix: Calculate correct new grid size when expanding cell_occupancy_matrix in the negative direction (#843)

## 0.8.1

### Added

- Impl `GridItemStyle` and `BlockContainerStyle` for `Style` (#832).

## 0.8.0

### Highlights

**The big feature in this release is support for `calc()` values in the low-level API.**

To use this API:

- Implement the `resolve_calc_value` method when implementing the `LayoutPartialTree` trait.
- Pass a type-erased pointer (`*const ()`) to constructors like `LengthPercentage::calc(...)`

Taffy treats the pointer as an opaque value (excepting that it uses the low 3 bits as a tag) which it will
pass to `LayoutPartialTree::resolve_calc_value` along with a percentage resolution basis when it needs to
resolve the value.

### Changed

- The representation of many "size" types is now a tagged pointer than an enum. This is to enable `calc()`.
  The effected types are `LengthPercentage`, `LengthPercentageAuto`, `Dimension`, `MinTrackSizingFunction`, and
`MaxTrackSizingFunction` types.

### Added

- Special-case "compressible replaced elements" in grid sizing algorithm (#807)
  This allows for more correct sizing of "replaced" elements such as images that are children
  of flexbox or grid containers.

### Fixed

- Grid: Fix infinite loop due to float precision in grid layout maximise tracks step (#792)
- Grid: Fix removed wrong addition, causing items to be misplaced. (#817)
- Grid: Fix grid placement for items with fixed primary axis (#818)
- Leaf layout: don't set available space to max-size (#819)

## 0.7.7

### Fixed

- Add `#[inline]` annotation to some methods on `TaffyTree` (#802)
- Add `TaffyTree::remove_children_range` method (#802)

## 0.7.6

### Fixed

- Fix infinite loop due to float precision in grid layout maximise tracks step (#792)

## 0.7.5

### Fixed

- Grid: only stretch auto tracks if content-alignment is stretch (#783)

## 0.7.4

### Fixed

- Fix detailed grid info for empty grid (#782)

## 0.7.3

### Fixed

- Make `TaffyTree::detailed_layout_info` take `&self` rather than `&mut self` (#779)

## 0.7.2

### Added

- The ability to access computed track sizes and item positions of a CSS Grid layout (#772).
  This information can be accessed using the `LayoutGridContainer::set_detailed_grid_info` method
  in the low-level API or the `TaffyTree::detailed_layout_info` method in the high-level API.

## 0.7.1

### Fixed

- Improve interaction of abspos children of block containers with margin collapsing (#760)

### Added

- Add `TaffyTree::unrounded_layout` getter (#765)

### Removed

- The `num-traits` dependency was removed (#761) (#762)

## 0.7.0

### Changed

- BREAKING: The `cache_mut` method on the `LayoutPartialTree` trait has been replaced with a separate `CacheTree` trait. This allows
  Taffy to be more easily used without caching or with a custom cache implementation.
- BREAKING: the `TaffyTree::set_children` method now removes the children from their previous parent (if they have one).

### Added

- Helper methods to retrieve content-box sizes were added to `Layout`

## 0.6.3

### Fixes

- Block: ignore margin collapsing when computing static position of abspos items (#747)

## 0.6.2

### Fixes

- Fix: clamp indefinite available space by min- and max- size as appropriate (#742)

## 0.6.1

### Fixes

- Fix calculation of `auto-fill`/`auto-fit` repetition count when container has a definite percentage size (#722)
- Fix min-size style not affecting intrinsic sizes (#723)
- Fix documentation of dirty and mark_dirty functions (#724)
- Fix intrinsic size of scroll containers that have a small explicit flex-basis (#728)

## 0.6.0

### Highlights

- The `Style` struct has been "traitified". This supports Taffy's integration in Servo and generally makes Taffy more flexible. The
  `Style` struct still exists and implements the new traits so existing uses of Taffy will continue to work as before.
- The `box-sizing` style is supported
- Computed margins are output in `Layout`

### Fixes

- Fix `print_tree()` when rounding is disabled (#680)
- Absolute Insets should be resolved against the container size minus border (#666)
- Fix flooring hypothetical_main_size by computed min size (#689)
- Fix flex line cross-size determination (#690)
- Fix panics in the grid algorithm (#691)
- Fix resolving flexible lengths (WPT css/flexbox-multiline-min-max test) (#692)
- Fix wrapping when a max main size style is present (#694)
- Fix case where Taffy allowed margins to collapse through an element when it shouldn't have (#695)

### Added

- Legacy text align (for laying out `<center>` and `<div align="..">`) is supported
- Add `is_table` for block items (#701)
- Impl `Debug` and `Clone` for `Cache` (#688)
- Implement `Debug` and `PartialEq` for tree types (#697)

## 0.5.2

- Fix block stretch sizing (don't always apply stretch sizing to block containers) (#674)
- Fix computation of intrinsic main size when it depends on a child's known cross size (#673)
- Fix panic when GridLine 0 is specified (#671)
- Docs: Document feature flags and scrape examples (#672)
- Docs: Update cosmic-text example to cosmic-text 0.12 (#670)

## 0.5.1

- Fix: Clamp block item stretch widths by their min and max width (#664)
- Fix: Auto margin computation in block layout (#663)

## 0.5.0

The changes in 0.5 are relatively small but the new measure function parameter is a breaking change so it requires a minor version bump.

- Added: A `style: &Style` parameter has been added to measure functions.
- Added: The `MaybeMath`, `MaybeResolve`, and `ResolveOrZero` traits have been made public.
- Fix: use SizingMode::Inherent when sizing absolute children of flexbox nodes.

## 0.4.4

### Fixes

- Content alignment (`align-content`/`justify-content`) behaviour was updated to match the latest spec (and Chrome 123+) (#635)
- Ensure that root Flexbox nodes are floored by their padding-border (#651, #655)
- Use grid area size not available space when applying aspect ratio to grid containers (#656)

## 0.4.3

### Fixes

- Fix compilation error in `evenly_sized_tracks` style helper in recent versions of rustc caused by a change/regression in type
  inference (#643). Note that 3rd-party code that call style helpers that take an `Into<f32>` parameter may still be affected by this issue,
  but they should be able to fix on their side by clarifying the type passed in

## 0.4.2

- Fixed: single-line flex-container should clamp the line's cross-size (#638)
- Reduced binary footprint of Taffy from around 300kb to around 150kb (#636)

## 0.4.1

- Fixed: CSS Grid track sizing not respecting growth limits in some circumstances (#624)

## 0.4.0

### Highlights

- Support for CSS Block layout (`display: block`)
- Support for the `overflow` property (+ `scrollbar_width` for `overflow: scroll`)
- Improved measure function API
- Completely refactored low-level API
- Simplified module hierarchy (+ most types/functions are now exported from the crate root)
- Expanded set of examples which better document integration with other layout systems (e.g. text layout)
- Computed values for `padding` and `border` are now output into the `Layout` struct

### Block layout

Support for [CSS Block layout](https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_Flow_Layout/Block_and_Inline_Layout_in_Normal_Flow#elements_participating_in_a_block_formatting_context) has been added. This can be used via the new `Display::Block` variant of the `Display` enum. Note that  full flow layout: inline, inline-block and float layout have *not* been implemented. The use case supported is block container nodes which contain block-level children.

### Overflow property

Support has been added for a new `overflow` style property with `Visible`, `Clip`, `Hidden`, and `Scroll` values (`Auto` is not currently implemented). Additionally a `scrollbar_width` property has been added to control the size of scrollbars for nodes with `Overflow::Scroll` set.

- Overflow is settable indpendently in each axis.
- `Visible` and `Clip` will produce layouts equivalent to the Taffy 0.3. `Clip` will affect the new `content_size` output by restricting it to the available space.
- `Hidden` and `Scroll` affect layout by changing the automatic minimum size of Flexbox and Grid children
- `Scroll` additionally reserves `scrollbar_width` pixels for a scrollbar in the opposite axis to which scrolling is enabled. `Scroll` with `scrollbar_width` set to zero is equivalent to `Hidden`.

### Measure function changes

The "measure function" API for integrating Taffy with other measurement systems (such as text layout) has been changed to be more flexible
and to interact better with borrow checking (you can now borrow external data in your measure function!).

- There are no longer per-node measure functions.
- There is now a single "global" measure function, and a per-node "context" of a user-defined type
- The `Taffy` tree is now a generic `TaffyTree<T>` where `T` is the "context" type.
- The measure function is now called for all leaf nodes (nodes without children). If you wish to maintain compatibility with the previous
  behaviour then your measure function should return `Size::ZERO` for leaf nodes whose context is `None`.

If you are not using measure functions, then the only change you will need to make is from:

```rust
let mut tree = Taffy::new();
```

to

```rust
let mut tree : TaffyTree<()> = TaffyTree::new();
```

And generally update any uses of `Taffy` in your codebase to `TaffyTree<()>`.

If you are using measure functions then you will need to make some bigger (but straightforward) changes. The following Taffy 0.3 code:

```rust
let mut tree = Taffy::new();
let leaf = tree.new_leaf_with_measure(
  Style::DEFAULT,
  |known_dimensions: Size<Option<f32>>, available_space: Size<AvailableSpace>| Size { width: 100.0, height: 200.0 }
);
tree.compute_layout(leaf, Size::MAX_CONTENT);
```

Should become something like the following with Taffy 0.4:

```rust
let mut tree : TaffyTree<Size> = TaffyTree::new();
let leaf = tree.new_leaf_with_context(Style::DEFAULT, Size { width: 100.0, height: 200.0 });
tree.compute_layout_with_measure(
  leaf,
  Size::MAX_CONTENT,
  |known_dimensions: Size<Option<f32>>, available_space: Size<AvailableSpace>, node_id: NodeId, node_context: Option<Size>| {
    node_context.unwrap_or(Size::ZERO)
  }
);
```

Note that:

- You can choose any type instead of `Size` in the above example. This includes your own custom type (which can be an enum or a trait object).
- If you don't need a context then you can use `()` for the context type
- As the single "global" measure function passed to `compute_layout_with_measure` only needs to exist for the duration of a single layout run,
  it can (mutably) borrow data from it's environment

### Low-level API (`LayoutTree` trait) refactor

The low-level API has been completely reworked:

- The `LayoutTree` trait has been split into 5 smaller traits which live in the `taffy::tree:traits` module (along with their associated documentation)
- The following methods have been removed from split `LayoutTree` traits entirely: `parent`, `is_childless`, `measure_node`, `needs_measure`, and `mark_dirty`.
- `taffy::node::Node` has been replaced with `taffy::NodeId`. This should make it much easier to implement the low-level traits as the underlying type backing the node id now a `u64` rather than a `slotmap::DefaultKey`.
- Support for running each layout algorithm individually on a single node via the following top-level functions:
  - `compute_flexbox_layout`
  - `compute_grid_layout`
  - `compute_block_layout`
  - `compute_leaf_layout`
  - `compute_root_layout`
  - `compute_hidden_layout`

It is believed that nobody was previously using the low-level API so we are not providing a migration guide. However, along with the refactor we have greatly
improved both the documentation and have added examples using the new API, both of which are linked to from the [main documentation page](https://docs.rs/taffy).

### Module hierarchy changes

The specific changes are detailed below. However for most users the most significant change will be that almost all types are now re-exported from the root module. This means that module specific imports like `use taffy::layout::Layout` can now in almost all cases be replaced with the simpler `use taffy::Layout`.

Specific changes:

- The `math` module has been made private
- The `axis` module has been merged into the `geometry` module
- The debug module is no longer public. The `print_tree` function is now accessible under `util`.
- All types from the `node`, `data`, `layout`, `error` and `cache` modules have been moved to the  the `tree` module.
- The `layout_flexbox()` function has been removed from the prelude. Use `taffy::compute_flexbox_layout` instead.

### Many APIs have been renamed to replace `points` or `Points` with `length` or `Length`

This new name better describes one-dimensional measure of space in some unspecified unit
which is often unrelated to the PostScript point or the CSS `pt` unit.

This also removes a misleading similarity with the 2D `Point`,
whose components can have any unit and are not even necessarily absolute lengths.

Example usage change:

```diff
 use taffy::prelude::*;

 // …

 let header_node = taffy
     .new_leaf(
         Style {
-            size: Size { width: points(800.0), height: points(100.0) },
+            size: Size { width: length(800.0), height: length(100.0) },
             ..Default::default()
         },
     ).unwrap();
```

### Other Changes

- The `Taffy` type was renamed to `TaffyTree` and made generic of a context parameter
- The Flexbox algorithm has now been moved behind the `flexbox` feature. The `flexbox` feature is enabled by default.
- The `justify_self` property has been moved behind the `grid` feature.
- Fixed misspelling: `RunMode::PeformLayout` renamed into `RunMode::PerformLayout` (added missing `r`).
- `serde` dependency has been made compatible with `no_std` environments
- `slotmap` dependency has been made compatible with `no_std` environments
- Added `insert_child_at_index()` method to the `TaffyTree`. This can be used to insert a child node at any position instead of just the end.
- Added `total_node_count()` method to the `TaffyTree` which returns the total number of nodes in the tree.
- Added `get_disjoint_node_context_mut()` method to the `TaffyTree`. This can be used to safely get multiple mutable borrows at the same time.

## 0.3.19

### Fixes

- Fix compilation error in `evenly_sized_tracks` style helper in recent versions of rustc caused by a change/regression in type
  inference (#643). Note that 3rd-party code that call style helpers that take an `Into<f32>` parameter may still be affected by this issue,
  but they should be able to fix on their side by clarifying the type passed in

## 0.3.18

### Fixes

- Fix computation of Flexbox automatic minimum size when grid or flexbox child has an explicit width/height style set (#576)

## 0.3.17

### Added

- Added `total_node_count` method to the `Taffy` struct. Returns the total number of nodes in the tree.

## 0.3.16

### Fixes

- Improve performance of flexbox columns

## 0.3.15

### Fixes

- Fix justify-content and align-content when free space is negative (content overflows container) (#549) (#551)

## 0.3.14

### Fixes

- Flex: Fix issue where constraints were not being propagated, causing nodes with inherent aspect-ratio (typically images) to not apply that aspect-ratio (#545) (Fixes bevyengine/bevy#9841)

## 0.3.13

### Fixes

- Fix rounding accumulation bug (#521) (Fixes #501 and bevyengine/bevy#8911)
- Flexbox: pass correct cross-axis available space when computing an item's intrinsic main size (#522)(Fixes bevyengine/bevy#9350)
- Flexbox: Subtract child margin not parent margin when computing stretch-alignment known size
- Grid: Make CSS Grid algorithm correctly apply max width/height and available space when it is the root node (#491)
- Grid: Fix CSS Grid "auto track" / placement bugs #481
  - Fix divide by zero when using grid_auto_rows/grid_auto_columns with zero negative implicit tracks
  - Fix over counting of tracks (leading to incorrect container heights) when auto-placing in grids that contain negative implicit tracks.
  - Fix axis conflation in auto-placement code when grid_auto_flow is column
  - Fix assignment of auto track sizes when initializing negative implicit tracks
- Leaf: Apply margins to leaf nodes when computing available space for measure functions
- Leaf: Reserve space for padding/borders in nodes with measure functions (#497)
  
  **NOTE: This has the potential to break layouts relying on the old behaviour.** However, such layouts would be relying on a style having no effect, so it is judged that such layouts are unlikely to exist in the wild. If this turns out not to be true then this fix will be reverted on the 0.3.x branch.

### Dependencies

- Upgrade `grid` to `0.10`. This eliminates the transitive dependency on `no-std-compat`.

## 0.3.12

### Fixes

- Fix caching issue when toggling `display:none` on and off

## 0.3.11

### Fixes

- Fix exponential blowup when laying out trees containing nodes with min and max sizes.

## 0.3.10

### Fixes

- Fix sizing of children when the available_space < min_size (#407)

## 0.3.9

### Fixes

- Fix caching bug where a cached result would sometimes be incorrectly used when the amount of available space increased (bevyengine/bevy#8111) and (bevyengine/bevy#8124)

## 0.3.8

### Fixes

- Fix incorrect min-content size for `flex-wrap: wrap` nodes (bevyengine/bevy#8082)

## 0.3.7

### Fixes

- Fix: Make `padding` and `border` floor node sizes (#372)
- Fix: Prevent percentages contributing to min-content sizes (#388) (also fixes bevyengine/bevy#8017)

## 0.3.6

### Fixes

- Fix: Ignore `align_content` when `flex_wrap` is set to `nowrap` (#383)

## 0.3.5

### Fixes

- Fix `display: none` when it is set on a flexbox child (#380)
- Fix `display: none` when it is set on a grid child (#381)

## 0.3.4

### Fixes

- Fix `display: none` when it is set for the only node in the hierarchy (#377)

## 0.3.3

### Added

- Added `enable_rounding` and `disable_rounding` methods to the `Taffy` struct which enable consumers of Taffy to obtain unrounded `f32` values for the computed layouts if they want them. Rounding remains enabled by default.

### Fixes

- Fixed rounding algorithm such that it never leaves gaps between adjacent nodes (#369)
- Fixed compiling with the `grid` feature disabled (#370)
- Fixed compiling with the `std` feature disabled

## 0.3.2

### Fixes

- Allow partial nested values to be deserialized into a `Style` using the `serde` feature.

## 0.3.1

### Fixes

- The `serde` feature now works when the `grid` feature is enabled

## 0.3.0

### Highlights

- [CSS Grid algorithm support](#new-feature-css-grid)
- [Style helper functions](#new-feature-style-helpers)

See below for details of breaking changes.

### New Feature: CSS Grid

We very excited to report that we now have support for CSS Grid layout. This is in addition to the existing Flexbox layout support, and the two modes interoperate. You can set a node to use Grid layout by setting the `display` property to `Display::Grid`.

#### Learning Resources

Taffy implements the CSS Grid specification faithfully, so documentation designed for the web should translate cleanly to Taffy's implementation. If you are interested in learning how to use CSS Grid, we would recommend the following resources:

- [CSS Grid Garden](https://cssgridgarden.com/). This is an interactive tutorial/game that allows you to learn the essential parts of CSS Grid in a fun engaging way.
- [A Complete Guide To CSS Grid](https://css-tricks.com/snippets/css/complete-guide-grid/) by CSS Tricks. This is detailed guide with illustrations and comprehensive written explanation of the different Grid properties and how they work.

#### Supported Features & Properties

In addition to the usual sizing/spacing properties (size, min_size, padding, margin, etc), the following Grid style properties are supported on Grid Containers:

| Property                  | Explanation                                                                                    |
| ---                       | ---                                                                                            |
| [`grid-template-columns`] | The track sizing functions of the grid's explicit columns                                      |
| [`grid-template-rows`]    | The track sizing functions of the grid's explicit rows                                         |
| [`grid-auto-rows`]        | Track sizing functions for the grid's implicitly generated rows                                |
| [`grid-auto-columns`]     | Track sizing functions for the grid's implicitly generated columns                             |
| [`grid-auto-flow`]        | Whether auto-placed items are placed row-wise or column-wise. And sparsely or densely.         |
| [`gap`]                   | The size of the vertical and horizontal gaps between grid rows/columns                         |
| [`align-content`]         | Align grid tracks within the container in the inline (horizontal) axis                         |
| [`justify-content`]       | Align grid tracks within the container in the block (vertical) axis                            |
| [`align-items`]           | Align the child items within their grid areas in the inline (horizontal) axis                  |
| [`justify-items`]         | Align the child items within their grid areas in the block (vertical) axis                     |

And the following Grid style properties are supported on Grid Items (children):

| Property                  | Explanation                                                                                    |
| ---                       | ---                                                                                            |
| [`grid-row`]              | The (row) grid line the item starts at (or a span)                                             |
| [`grid-column`]           | The (column) grid line the item end at (or a span)                                             |
| [`align-self`]            | Align the item within it's grid area in the inline (horizontal) axis. Overrides `align-items`. |
| [`justify-self`]          | Align the item within it's grid area in the block (vertical) axis. Overrides `justify-items`.  |

[`grid-template-columns`]: https://developer.mozilla.org/en-US/docs/Web/CSS/grid-template-columns
[`grid-template-rows`]: https://developer.mozilla.org/en-US/docs/Web/CSS/grid-template-rows
[`grid-auto-rows`]: https://developer.mozilla.org/en-US/docs/Web/CSS/grid-auto-rows
[`grid-auto-columns`]: https://developer.mozilla.org/en-US/docs/Web/CSS/grid-auto-columns
[`grid-auto-flow`]: https://developer.mozilla.org/en-US/docs/Web/CSS/grid-auto-flow
[`gap`]: https://developer.mozilla.org/en-US/docs/Web/CSS/gap
[`align-content`]: https://developer.mozilla.org/en-US/docs/Web/CSS/align_content
[`justify-content`]: https://developer.mozilla.org/en-US/docs/Web/CSS/justify_content
[`align-items`]: https://developer.mozilla.org/en-US/docs/Web/CSS/align-items
[`justify-items`]: https://developer.mozilla.org/en-US/docs/Web/CSS/justify-items
[`grid-row`]: https://developer.mozilla.org/en-US/docs/Web/CSS/grid-row
[`grid-column`]: https://developer.mozilla.org/en-US/docs/Web/CSS/grid-column
[`align-self`]: https://developer.mozilla.org/en-US/docs/Web/CSS/align-self
[`justify-self`]: https://developer.mozilla.org/en-US/docs/Web/CSS/justify-self

The following properties and features are not currently supported:

- Subgrids
- Masonry grid layout
- Named grid lines
- Named areas: `grid-template-areas` and `grid-area`
- `grid-template` or `grid` shorthand

#### Example

See [examples/grid_holy_grail.rs](https://github.com/DioxusLabs/taffy/blob/main/examples/grid_holy_grail.rs) for an example using Taffy to implement the so-called [Holy Grail Layout](https://en.wikipedia.org/wiki/Holy_grail_(web_design)). If you want to run this example, the don't forget the enable the CSS Grid cargo feature:

```bash
cargo run --example grid_holy_grail --features grid
```

### New Feature: Style Helpers

Ten new helper functions have added to the taffy prelude. These helper functions have short, intuitive names, and have generic return types which allow them to magically return the correct type depending on context. They make defining styles much easier, and means you won't typically need to use types like `Dimension` or `TrackSizingFunction` directly.

For example, instead of:

```rust
let size : Size<Dimension> = Size { width: Dimension::Points(100.0), height: Dimension::Percent(50.0) };
```

you can now write

```rust
let size : Size<Dimension> = Size { width: points(100.0), height: percent(50.0) };
```

And that same helper function will work other types like `LengthPercentage` and `MinTrackSizingFunction` that also have a `Points` variant. There are also generic impl's for `Size<T>`, `Rect<T>` and `Line<T>` which means if your node is the same size in all dimensions you can even write

```rust
let size : Size<Dimension> = points(100.0);
```

Available style helpers:

<table>
  <thead><tr><th>Type(s)</th><th colspan="2">Helpers that work with that type</th></tr></thead>
  <tbody>
    <tr>
      <td rowspan="3"><code>LengthPercentage</code></td>
      <td><code>zero()</code></td>
      <td>Generates a <code>Points</code> variant with the value <code>0.0</code></td>
    </tr>
    <tr>
      <td><code>points(val:&nbsp;f32)</code></td>
      <td>Generates a <code>Points</code> variant with the specified value</td>
    </tr>
    <tr>
      <td><code>percent(val:&nbsp;f32)</code></td>
      <td>Generates a <code>Percent</code> variant with the specified value.<br />Note that the scale of 0-1 not 0-100.</td>
    </tr>
    <tr>
      <td rowspan="2"><code>LengthPercentageAuto</code><br /><code>Dimension</code></td>
      <td colspan="2"><i>All helpers from <code>LengthPercentage</code> and...</i></td>
    </tr>
    <tr>
      <td><code>auto()</code></td>
      <td>Generates an <code>Auto</code> variant</td>
    </tr>
    <tr>
      <td rowspan="3"><code>MinTrackSizingFunction</code></td>
      <td colspan="2"><i>All helpers from <code>LengthPercentageAuto</code>/<code>Dimension</code> and...</i></td>
    </tr>
    <tr>
      <td><code>min_content()</code></td>
      <td>Generates an <code>MinContent</code> variant</td>
    </tr>
      <tr>
      <td><code>max_content()</code></td>
      <td>Generates an <code>MinContent</code> variant</td>
    </tr>
    <tr>
      <td rowspan="3"><code>MaxTrackSizingFunction</code></td>
      <td colspan="2"><i>All helpers from <code>MinTrackSizingFunction</code> and...</i></td>
    </tr>
    <tr>
      <td><code>fit_content(limit:&nbsp;LengthPercentage)</code></td>
      <td>Generates a <code>FitContent</code> variant with the specified limit.<br />Nest the <code>points</code> or <code>percent</code> helper inside this function to specified the limit.</td>
    </tr>
      <tr>
      <td><code>fr(fraction:&nbsp;f32)</code></td>
      <td>Generates a <code>Fraction</code> (<code>fr</code>) variant with the specified flex fraction </td>
    </tr>
    <tr>
      <td rowspan="3"><code>NonRepeatingTrackSizingFunction</code></td>
      <td colspan="2"><i>All helpers from <code>MaxTrackSizingFunction</code> and...</i></td>
    </tr>
    <tr>
      <td><code>minmax(min: MinTrackSizingFunction, max: MaxTrackSizingFunction)</code></td>
      <td>Equivalent to CSS <code>minmax()</code> function.</td>
    </tr>
    <tr>
      <td><code>flex(fraction:&nbsp;f32)</code></td>
      <td>Equivalent to CSS <code>minmax(0px, 1fr)</code>. This is likely what you want if you want evenly sized rows/columns.</td>
    </tr>
    <tr>
      <td rowspan="2"><code>TrackSizingFunction</code></td>
      <td colspan="2"><i>All helpers from <code>NonRepeatingTrackSizingFunction</code> and...</i></td>
    </tr>
    <tr>
      <td><code>repeat(rep: GridTrackRepetition, tracks: Vec&lt;TrackSizingFunction&gt;)</code></td>
      <td>Equivalent to css <code>repeat()</code> function.</td>
    </tr>
    <tr>
      <td><code>Vec&lt;TrackSizingFunction&gt;</code></td>
      <td><code>evenly_sized_tracks(count:&nbsp;u16)</code></td>
      <td>Equivalent to CSS <code>repeat(count, minmax(0px, 1fr)</code></td>
    </tr>
    <tr>
      <td rowspan="3"><code>AvailableSpace</code></td>
      <td><code>auto()</code></td>
      <td>Generates an <code>Auto</code> variant</td>
    </tr>
    <tr>
      <td><code>min_content()</code></td>
      <td>Generates an <code>MinContent</code> variant</td>
    </tr>
      <tr>
      <td><code>max_content()</code></td>
      <td>Generates an <code>MinContent</code> variant</td>
    </tr>
    <tr>
      <td><code>Size&lt;T&gt;</code></td>
      <td colspan="2">Any helper that works for <code>T</code> will also work for <code>Size&lt;T&gt;</code> and will set both <code>width</code> and <code>height</code> to that value</td>
    </tr>
    <tr>
      <td><code>Rect&lt;T&gt;</code></td>
      <td colspan="2">Any helper that works for <code>T</code> will also work for <code>Rect&lt;T&gt;</code> and will set <code>top</code>, <code>left</code>, <code>bottom</code>, and <code>right</code> to that value</td>
    </tr>
  </tbody>
</table>

### Breaking API changes

#### Changes to alignment style types

- `AlignContent` and `JustifyContent` has been merged.
  - `JustifyContent` is now an alias of `AlignContent` and contains the `Stretch` variant.
  - This variant will be *ignored* (falling back to `Start`) when applied Flexbox containers. It is valid value for Grid containers.
- `AlignItems` and `AlignSelf` have been merged.
  - The `Auto` variant of `AlignSelf` has been removed. You should now use `Option::None` if you wish to specify `AlignSelf::Auto`.
  - `AlignSelf` is now an alias of `AlignItems`.
  - `JustifyItems` and `JustifySelf` aliases have been added. These properties have no affect on Flexbox containers, but apply to Grid containers.
- `Default` impls have been removed from all alignment types. This is because the correct default varies by property, and the types are now shared between multiple properties. The `Style` struct still has a default for each alignment property, so this is considered unlikely to affect you in practice.

#### Strict style types

- New types `LengthPercentage` and `LengthPercentageAuto` have been added.
  - `LengthPercentage` is like `Dimension` but only contains the `Points` and `Percent` variants, which allows us to increase type safety for properties that don't support the `Auto` value.
  - `LengthPercentageAuto` is currently identical to `Dimension` but will allow us to expand dimension in future to support values like `MinContent`, `MaxContent` and `FitContent`.
- Some style properties have been updated to use either `LengthPercentage` or `LengthPercentageAuto` instead of `Dimension`. You will need to update your code, but it is recommended that you use the new style helpers (see above) rather than using the new types directly (although you certainly can use them directly if you want to).

#### Position properties renamed

- The `position` property is now renamed to `inset` and is now in line with [CSS inset specs](https://developer.mozilla.org/en-US/docs/Web/CSS/inset)
- The `position_type` property is now renamed to `position` and is now in line with [CSS position specs](https://developer.mozilla.org/en-US/docs/Web/CSS/position). The `PositionType` enum has been similarly renamed to `Position`.

#### Changes to `LayoutTree`

- Added generic associated type to `LayoutTree` for a `ChildIter`, an iterator on the children of a given node.
- Changed the `children` method of `LayoutTree` to return the `ChildIter` generic associated type to allow for custom tree storage implementations which do not store the children of a node contiguously.
- Added `child_count`  method to `LayoutTree` for querying the number of children of a node. Required because the `children` method now returns an iterator instead of an array.
- Added `is_childless` method to `LayoutTree` for querying whether a node has no children.

#### `AvailableSpace` has been moved

The `AvailableSpace` enum has been moved from the `layout` module to the `style` module. If you are importing it via the prelude then you will unaffected by the change.

### Fixes

- Flexbox nodes sized under a min-content constraint now size correctly (#291)
- Aspect ratio is now applied correctly in many circumstances
- Absolutely positioned items now apply margins correctly
- Min/max size are now applied correctly
- Inset applied incorrectly to relatively positioned flexbox children when both `top` and `bottom` or `left` and `right` were specified (#348)
- Fix case where column-gap style could be used in place of row-gap style (when using a percentage gap with an indefinite container size)

### Removed

- Removed `top_from_points`, `bot_from_points`, `top_from_percent`, and `bot_from_percent` methods removed from `Rect<Dimension>`. These functions were incredibly specific for an unusual use case, so we would be surprised if anyone was using them. Please use the new style helpers instead.
- Removed `min_main_size`, `max_main_size`, `min_cross_size`, `max_cross_size`, and `cross_size` methods from `Style`. Use the more general `cross` and `main` methods directly on the `size`, `min_size`, and `max_size` properties instead.
- Removed `main_margin_start`, `main_margin_end`, `cross_margin_start`, `cross_margin_end` from `Style`. Use the more general `main_start`, `main_end`, `cross_start`, and `cross_end` on the `margin` property instead.

## 0.2.2

### Fixes

- Border or padding on the horizontal axis could, in some cases, increase the height of nodes.

## 0.2.1

### Fixes

- In case of conflicts, `min_size` now overrides `max_size` which overrides `size` (#261). This is the behaviour specified in the CSS specification, and was also the behaviour in Taffy `v0.1.0`, but a regression was introduced in Taffy `v0.2.0`.
- `taffy::compute_layout` has been made public allowing Taffy to be used with custom storage (#263)

## 0.2.0

### New features

#### Flexbox "gap" and `AlignContent::SpaceEvenly`

The [gap](https://developer.mozilla.org/en-US/docs/Web/CSS/gap) property is now supported on flex containers. This can make it much easier to create even spacing or "gutters" between nodes.

Additionally we have a `SpaceEvenly` variant to the `AlignContent` enum to support evenly spaced justification in the cross axis (equivalent to  `align-content: space-evenly` in CSS)

#### Debug module and cargo feature

Two debugging features have been added:

- `taffy::debug::print_tree(&Taffy, root)` - This will print a debug representation of the computed layout of an entire node tree (starting at `root`), which can be useful for debugging layouts.
- A cargo feature `debug`. This enabled debug logging of the layout computation process itself (this is probably mainly useful for those working taffy itself).

### Performance improvements

A number of performance improvements have landed since taffy 0.1:

- Firstly, our custom `taffy::forest` storage implementation was ripped out and replaced with a much simpler implementation using the `slotmap` crate. This led to performance increases of up to 90%.
- Secondly, the caching implementation was improved by upping the number of cache slots from 2 to 4 and tweaking how computed results are allocated to cache slots to better match the actual usage patterns of the flexbox layout algorithm. This had a particularly dramatic effect on deep hierarchies (which often involve recomputing the same results repeatedly), fixing the exponential blowup that was previously exhibited on these trees and improving performance by over 1000x in some cases!

#### Benchmarks vs. Taffy 0.1

| Benchmark                                 | Taffy 0.1 | Taffy 0.2 | % change (0.1 -> 0.2) |
| ---                                       | ---       | ---       | ---                   |
| wide/1_000 nodes (2-level hierarchy)      | 699.18 µs | 445.01 µs | -36.279%              |
| wide/10_000 nodes (2-level hierarchy)     | 8.8244 ms | 7.1313 ms | -16.352%              |
| wide/100_000 nodes (2-level hierarchy)    | 204.48 ms | 242.93 ms | +18.803%              |
| deep/4000 nodes (12-level hierarchy))     | 5.2320 s  | 2.7363 ms | -99.947%              |
| deep/10_000 nodes (14-level hierarchy)    | 75.207 s  | 6.9415 ms | -99.991%              |
| deep/100_000 nodes (17-level hierarchy)   | -         | 102.72 ms | -                     |
| deep/1_000_000 nodes (20-level hierarchy) | -         | 799.35 ms | -                     |

(note that the table above contains multiple different units (milliseconds vs. microseconds vs. nanoseconds))

As you can see, we have actually regressed slightly in the "wide" benchmarks (where all nodes are siblings of a single parent node). Although it should be noted our results in these benchmarks are still very fast, especially on the 10,000 node benchmark which we consider to be the most realistic size where the result is measured in microseconds.

However, in the "deep" benchmarks we see dramatic improvements. The previous version of Taffy suffered from exponential blowup in the case of deeply nested hierarchies. This has resulted in somewhat silly improvements like the 10,000 node (14-level) hierarchy where Taffy 0.2 is a full 1 million times faster than Taffy 0.1. We've also included results with larger numbers of nodes (although you're unlikely to need that many) to demonstrate that this scalability continues up to even deeper levels of nesting.

#### Benchmarks vs. [Yoga](https://github.com/facebook/yoga)

Yoga benchmarks run via it's node.js bindings (the `yoga-layout-prebuilt` npm package), they were run a few times manually and it was verified that variance in the numbers of each run was minimal. It should be noted that this is using an old version of Yoga.

| Benchmark | Yoga | Taffy 0.2 |
| --- | --- | --- |
| yoga/10 nodes (1-level hierarchy) | 45.1670 µs | 33.297 ns |
| yoga/100 nodes (2-level hierarchy) | 134.1250 µs | 336.53 ns |
| yoga/1_000 nodes (3-level hierarchy) | 1.2221 ms | 3.8928 µs |
| yoga/10_000 nodes (4-level hierarchy) | 13.8672 ms | 36.162 µs |
| yoga/100_000 nodes (5-level hierarchy) | 141.5307 ms | 1.6404 ms |

(note that the table above contains multiple different units (milliseconds vs. microseconds vs. nanoseconds))

While we're trying not to get too excited (there could easily be an issue with our benchmarking methodology which make this an unfair comparison), we are pleased to see that we seem to be anywhere between 100x and 1000x times faster depending on the node count!

### Breaking API changes

#### Node creation changes

- `taffy::Node` is now unique only to the Taffy instance from which it was created.
- Renamed `Taffy.new_node(..)` -> `Taffy.new_with_children(..)`
- Renamed `Taffy.new_leaf()` -> `Taffy.new_leaf_with_measure()`
- Added `taffy::node::Taffy.new_leaf()` which allows the creation of new leaf-nodes without having to supply a measure function

#### Error handling/representation improvements

- Renamed `taffy::Error` -> `taffy::error::TaffyError`
- Replaced `taffy::error::InvalidChild` with a new `InvalidChild` variant of `taffy::error::TaffyError`
- Replaced `taffy::error::InvalidNode` with a new `InvalidNode` variant of `taffy::error::TaffyError`
- The following method new return `Err(TaffyError::ChildIndexOutOfBounds)` instead of panicking:
  - `taffy::Taffy::remove_child_at_index`
  - `taffy::Taffy::replace_child_at_index`
  - `taffy::Taffy::child_at_index`
- `Taffy::remove` now returns a `Result<usize, Error>`, to indicate if the operation was successful (and if it was, which ID was invalidated).

#### Some uses of `Option<f32>` replaced with a new `AvailableSpace` enum

A new enum `Taffy::layout::AvailableSpace` has been added.

The definition looks like this:

```rust
/// The amount of space available to a node in a given axis
pub enum AvailableSpace {
    /// The amount of space available is the specified number of pixels
    Definite(f32),
    /// The amount of space available is indefinite and the node should be laid out under a min-content constraint
    MinContent,
    /// The amount of space available is indefinite and the node should be laid out under a max-content constraint
    MaxContent,
}
```

This enum is now used instead of `Option<f32>` when calling `Taffy.compute_layout` (if you previously passing `Size::NONE` to `compute_layout`, then you will need to change this to `Size::MAX_CONTENT`).

And a different instance of it is passed as a new second parameter to `MeasureFunc`. `MeasureFunc`s may choose to use this parameter in their computation or ignore it as they see fit. The canonical example of when it makes sense to use it is when laying out text. If `MinContent` has been passed in the axis in which the text is flowing (i.e. the horizontal axis for left-to-right text), then you should line-break at every possible opportunity (e.g. all word boundaries), whereas if `MaxContent` has been passed then you shouldn't line break at all..

#### Builder methods are now `const` where possible

- Several convenience constants have been defined: notably `Style::DEFAULT`
- `Size<f32>.zero()` is now `Size::<f32>::ZERO`
- `Point<f32>.zero()` is now  `Point::<f32>::ZERO`
- `Size::undefined()` is now `Size::NONE`

#### Removals

- Removed `taffy::forest::Forest`. `taffy::node::Taffy` now handles it's own storage using a slotmap (which comes with a performance boost up to 90%).
- Removed `taffy::number::Number`. Use `Option<f32>` is used instead
  - the associated public `MinMax` and `OrElse` traits have also been removed; these should never have been public
- Removed unused dependencies `hashbrown`, `hash32`, and `typenum`. `slotmap` is now the only required dependency (`num_traits` and `arrayvec` are also required if you wish to use taffy in a `no_std` environment).

### Fixes

- Miscellaneous correctness fixes which align our implementation with Chrome:

  - Nodes can only ever have one parent
  - Fixed rounding of fractional values to follow latest Chrome - values are now rounded the same regardless of their position
  - Fixed computing free space when using both `flex-grow` and a minimum size
  - Padding is now only subtracted when determining the available space if the node size is unspecified, following [section 9.2.2 of the flexbox spec](https://www.w3.org/TR/css-flexbox-1/#line-sizing)
  - `MeasureFunc` (and hence `NodeData` and hence `Forest` and hence the public `Taffy` type) are now `Send` and `Sync`, enabling their use in async and parallel applications
- Taffy can now be vendored using `cargo-vendor` (README.md is now included in package).

## 0.1.0

### 0.1.0 Changed

- the `order` field of `Layout` is now public, and describes the relative z-ordering of nodes
- renamed crate from `stretch2` to `taffy`
- updated to the latest version of all dependencies to reduce upstream pain caused by duplicate dependencies
- renamed `stretch::node::Stretch` -> `taffy::node::Taffy`

### 0.1.0 Fixed

- fixed feature strategy for `alloc` and `std`: these can now be compiled together, with `std`'s types taking priority

### 0.1.0 Removed

- removed Javascript / Kotlin / Swift bindings
  - the maintainer team lacks expertise to keep these working
  - more serious refactors are planned, and this will be challenging to keep working through that process
  - if you are interested in helping us maintain bindings to other languages, [get in touch](https://github.com/DioxusLabs/taffy/discussions)!
- the `serde_camel_case` and `serde_kebab_case` features have been removed: they were poorly motivated and were not correctly additive (if both were enabled compilation would fail)
- removed the `Direction` and `Overflow` structs, and the corresponding `direction` and `overflow` fields from `Style`
  - these had no effect in the current code base and were actively misleading

## stretch2 0.4.3

This is the final release of `stretch`: migrate to the crate named `taffy` for future fixes and features!

These notes describe the differences between this release and `stretch` 0.3.2, the abandoned crate from which this library was forked.

### Changed

- updated [assorted dependencies](https://github.com/vislyhq/stretch/commit/a6491117379cea52dedc9584d892594a143e8cb0)

### Fixed

- fixed an exponential performance blow-up with deep nesting
- fixed percent height values, which were using parent width
- recomputing layout no longer moves children of non-zero-positioned parent
- fixed broken Swift bindings
