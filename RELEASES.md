# Release Notes

## Unreleased

### Changes

- The Flexbox algorithm has now been moved behind the `flexbox` feature. The `flexbox` feature is enabled by default.

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
- [A Complete Guide To CSS Grid](https://css-tricks.com/snippets/css/complete-guide-grid/) by CSS Tricks. This is detailed guide with illustrations and comphrehensive written explanation of the different Grid propertie and how they work.

#### Supported Features & Properties

In addition to the usual sizing/spacing proerties (size, min_size, padding, margin, etc), the following Grid style properties are supported on Grid Containers:

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
- Secondly, the caching implementation was improved by upping the number of cache slots from 2 to 4 and tweaking how computed results are allocated to chache slots to better match the actual usage patterns of the flexbox layout algorithm. This had a particularly dramatic effect on deep hierachies (which often involve recomputing the same results repeatedly), fixing the exponential blowup that was previously exhibited on these trees and improving performance by over 1000x in some cases!

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

However, in the "deep" benchmarks we see dramatic improvements. The previous version of Taffy suffered from exponential blowup in the case of deeply nested hierachies. This has resulted in somewhat silly improvements like the 10,000 node (14-level) hierachy where Taffy 0.2 is a full 1 million times faster than Taffy 0.1. We've also included results with larger numbers of nodes (although you're unlikely to need that many) to demonstrate that this scalability continues up to even deeper levels of nesting.

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
- `Taffy::remove` now returns a `Result<usize, Error>`, to indicate if the operation was sucessful (and if it was, which ID was invalidated).

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
- renamed `stretch::node::Strech` -> `taffy::node::Taffy`

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
