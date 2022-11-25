# Release Notes

## 0.2.1 (unreleased)

### Fixes
- `taffy::compute_layout` has been made public allowing Taffy to be used with custom storage (#263)

- In case of conflicts, `min_size` now overrides `max_size` which overrides `size` (#261)

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
