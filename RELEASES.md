# Release Notes

## 0.2.0

### 0.2.0 Added

- Added `taffy::error::InvalidChild` Error type
- `taffy::node::Taffy.new_leaf()` which allows the creation of new leaf-nodes without having to supply a measure function

### 0.2.0 Changed

- `Size<f32>.zero()` is now `Size::<f32>::ZERO`
- `Point<f32>.zero()` is now  `Point::<f32>::ZERO`
- renamed `taffy::node::Taffy.new_leaf()` -> `taffy::node::Taffy.new_leaf_with_measure()`
- removed the public `Number` type; a more idiomatic `Option<f32>` is used instead
  - the associated public `MinMax` and `OrElse` traits have also been removed; these should never have been public
- `Sprawl::remove` now returns a `Result<usize, Error>`, to indicate if the operation was sucessful, and if it was, which ID was invalidated.
- renamed `taffy::forest::Forest.new-node(..)` `taffy::forest::Forest.new_with_children(..)`
- renamed `taffy::node::Taffy.new-node(..)` -> `taffy::node::Taffy.new_with_children(..)`
- renamed `taffy::style::Style` -> `taffy::style::FlexboxLayout` to more precicely indicate its purpose
- renamed `taffy::Error` -> `taffy::error::InvalidNode`
- `taffy::Taffy::remove_child_at_index`, `taffy::Taffy::replace_child_at_index`, and `taffy::Taffy::child_at_index` now return `taffy::InvalidChild::ChildIndexOutOfBounds` instead of panicing

### 0.2.0 Fixed

- fixed rounding of fractional values to follow latest Chrome - values are now rounded the same regardless of their position
- fixed computing free space when using both `flex-grow` and a minimum size
- padding is now only subtracted when determining the available space if the node size is unspecified, following [section 9.2.2 of the flexbox spec](https://www.w3.org/TR/css-flexbox-1/#line-sizing)
- `MeasureFunc` (and hence `NodeData` and hence `Forest` and hence the public `Taffy` type) are now `Send` and `Sync`, enabling their use in async and parallel applications

### 0.2.0 Removed

- various internal types are no longer public
  - if you needed one of these, please file an issue!

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
