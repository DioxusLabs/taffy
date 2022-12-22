# Release Notes

## 0.3.0-alpha1 (Unreleased)

This is the first in a series of planned alpha releases to allow users of Taffy to try out the new CSS Grid layout mode in advance of a stable release. We hope that by marking this is alpha release we are clearly communicating that this a pre-release and that the implementation is not yet of production quality. But we never-the-less encourage you to try it out. Feedback is welcome, and bug reports for the Grid implementation are being accepted as of this release.

**Note: CSS Grid support must be enabled using the `experimental_grid` feature. For the time being this feature is not enabled by default.**

### New Features

#### CSS Grid (Experimental)

We very excited to report that we have an initial version of the CSS Grid layout available. This is in addition to the existing Flexbox layout support, and the two modes interoperate (although this interaction has not been extensively tested). You can set a node to use Grid layout by setting the `display` property to `Display::Grid`.

Taffy implements the CSS Grid specification faithfully, so documentation designed for the web should translate cleanly to Taffy's implementation. If you are interested in learning how to use CSS Grid, we would recommend the following resources:

- [CSS Grid Garden](https://cssgridgarden.com/). This is an interactive tutorial/game that allows you to learn the essential parts of CSS Grid in a fun engaging way.
- [A Complete Guide To CSS Grid](https://css-tricks.com/snippets/css/complete-guide-grid/) by CSS Tricks. This is detailed guide with illustrations and comphrehensive written explanation of the different Grid propertie and how they work.

##### Supported Features & Properties

In addition to the usual sizing/spacing proerties (size, min_size, padding, margin, etc), the following Grid style properties are supported on Grid Containers:

| Property                  | Explanation                                                                                    |
| ---                       | ---                                                                                            |
| [`grid-template-columns`] | The track sizing functions of the grid's explicit columns                                      |
| [`grid-template-rows`]    | The track sizing functions of the grid's explicit rows                                         |
| [`grid-template-areas`]   | Defines named grid areas                                                                       |
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

[`grid-template-columns`]: https:://developer.mozilla.org/en-US/docs/Web/CSS/grid-template-columns
[`grid-template-rows`]: https:://developer.mozilla.org/en-US/docs/Web/CSS/grid-template-rows
[`grid-template-areas`]: https:://developer.mozilla.org/en-US/docs/Web/CSS/grid-template-areas
[`grid-auto-rows`]: https:://developer.mozilla.org/en-US/docs/Web/CSS/grid-auto-rows
[`grid-auto-columns`]: https:://developer.mozilla.org/en-US/docs/Web/CSS/grid-auto-columns
[`grid-auto-flow`]: https:://developer.mozilla.org/en-US/docs/Web/CSS/grid-auto-flow
[`gap`]: https:://developer.mozilla.org/en-US/docs/Web/CSS/gap
[`align-content`]: https:://developer.mozilla.org/en-US/docs/Web/CSS/align_content
[`justify-content`]: https:://developer.mozilla.org/en-US/docs/Web/CSS/justify_content
[`align-items`]: https:://developer.mozilla.org/en-US/docs/Web/CSS/align-items
[`justify-items`]: https:://developer.mozilla.org/en-US/docs/Web/CSS/justify-items
[`grid-row`]: https:://developer.mozilla.org/en-US/docs/Web/CSS/grid-row
[`grid-column`]: https:://developer.mozilla.org/en-US/docs/Web/CSS/grid-column
[`align-self`]: https:://developer.mozilla.org/en-US/docs/Web/CSS/align-self
[`justify-self`]: https:://developer.mozilla.org/en-US/docs/Web/CSS/justify-self

The following properties and features are not yet supported, but are planned for the near future:

- Baseline alignment
- `fit-content()` with a percentage argument.
- `repeat()` with integer repetition (but users of Taffy can just expand these definition manually)

The following properties and features are not supported, and there are no immediate plans to implement them:

- Subgrids
- Named grid lines
- Named areas: `grid-template-areas` and `grid-area`
- `grid-template` or `grid` shorthand

##### Example

This example is based on the so-called [Holy Grail Layout](https://en.wikipedia.org/wiki/Holy_grail_(web_design)). The HTML we will be recreating with Taffy is as follows:

```html
<div style="display: grid;grid-template-columns: 250px 1fr 250px;grid-template-rows: 150px 1fr 150px">
  <div id="header" style="grid-row: 1;grid-column: span 3"></div>
  <div id="left_sidebar" style="grid-row: 2;grid-column: 1"></div>
  <div id="content_area" style="grid-row: 2;grid-column: 2"></div>
  <div id="right_sidebar" style="grid-row: 2;grid-column: 3"></div>
  <div id="footer" style="grid-row: 3;grid-column: span 3"></div>
</div>

```

The equivalent taffy code is:

```rust
/// Small helper function to make the rest of the examples less verbose
fn default<T: Default>() -> T {
    Default::default()
}

fn main() -> Result<(), taffy::error::TaffyError> {
    // The taffy prelude contains most of Taffy's user-facing types and lots of helper functions to help make defining style less verbose
    use taffy::prelude::*;

    // Create an instance of Taffy
    let mut taffy = Taffy::new();

    // Setup the grid
    let root_style = Style {
        display: Display::Grid,
        size: Size { width: points(800.0), height: points(600.0) },
        grid_template_columns: vec![points(250.0), flex(1.0), points(250.0)],
        grid_template_rows: vec![points(150.0), flex(1.0), points(150.0)],
        ..default()
    };

    // Define the child nodes
    let header = taffy.new_leaf(Style { grid_row: line(1), grid_column: span(3), ..default() })?;
    let left_sidebar = taffy.new_leaf(Style { grid_row: line(2), grid_column: line(1), ..default() })?;
    let content_area = taffy.new_leaf(Style { grid_row: line(2), grid_column: line(2), ..default() })?;
    let right_sidebar = taffy.new_leaf(Style { grid_row: line(2), grid_column: line(3), ..default() })?;
    let footer = taffy.new_leaf(Style { grid_row: line(3), grid_column: span(3), ..default() })?;

    // Create the container with the children
    let root = taffy.new_with_children(root_style, &[header, left_sidebar, content_area, right_sidebar, footer])?;

    // Compute layout and print result
    taffy.compute_layout(root, Size { width: points(800.0), height: points(600.0) })?;
    taffy::debug::print_tree(&taffy, root);

    Ok(())
}
```

#### Style Helpers

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

The following functions work for `Dimension`, `LengthPercentageAuto`, `LengthPercentage`, `AvailableSpace` and for Grid track sizing functions

- `points(f32)` - Generates a `Points` variant with the specified value
- `zero()` - Generates a `Points` variant with the value `0.0`.

The following functions work for `Dimension`, `LengthPercentageAuto`, `LengthPercentage` and for Grid track sizing functions

- `percent(f32)` - Generates a `Percent` value
- `auto()` - Generates an `Auto` variant

The following functions work for `AvailableSpace` and grid track sizing functions:

- `min_content()` - Generates an `MinContent` variant
- `max_content()` - Generates an `MaxContent` variant

The following functions currently work only for grid track sizing functions:

- `flex(f32)` - Genrates a `Flex` variant with the specified flex fraction
- `fit_content(LengthPercentage)` - Generates a `FitContent` variant with the specified limit. Nest `points` or `percent` inside this function to specified the limit.
- `minmax(MinTrackSizingFunction, MaxTrackSizingFunction)` - Generates a track sizing function with different min and max sizing functions. Nest `points`, `percent`, `auto`, `min_content`, `max_content`, or `flex` to specify the min and max functions.
- `repeat(GridTrackRepetition, Vec<TrackSizingFunction>)` - Genereate an auto-repeating track definition.

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

#### Changes to `LayoutTree`

- Added generic associated type to `LayoutTree` for a `ChildIter`, an iterator on the children of a given node.
- Changed the `children` method of `LayoutTree` to return the `ChildIter` generic associated type to allow for custom tree storage implementations which do not store the children of a node contiguously.
- Added `child_count`  method to `LayoutTree` for querying the number of children of a node. Required because the `children` method now returns an iterator instead of an array.
- Added `is_childless` method to `LayoutTree` for querying whether a node has no children.

### Fixes

- Fix case where column-gap style could be used in place of row-gap style (when using a percentage gap with an indefinite container size)

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

| Benchmark | Taffy 0.1 | Taffy 0.2 | % change (0.1 -> 0.2) |
| --- | --- | --- | --- |
| wide/1_000 nodes (2-level hierarchy) | 3.5458 µs | 4.3571 µs | +23.333% |
| wide/10_000 nodes (2-level hierarchy) | 36.418 µs | 42.967 µs | +17.357% |
| wide/100_000 nodes (2-level hierarchy) | 1.8275 ms | 3.9096 ms | +112.26% |
| deep/4000 nodes (12-level hierarchy)) | 5.1845 s | 15.318 µs | -100.000% |
| deep/10_000 nodes (14-level hierarchy) | 75.978 s | 40.315 µs | -100.000% |
| deep/100_000 nodes (17-level hierarchy) | - | 2.7644 ms| - |
| deep/1_000_000 nodes (20-level hierarchy) | - | 1.2130 s| - |

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
