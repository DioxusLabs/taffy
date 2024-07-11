# Release Notes

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
