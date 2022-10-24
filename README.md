# taffy

[![GitHub CI](https://github.com/DioxusLabs/taffy/actions/workflows/ci.yml/badge.svg)](https://github.com/DioxusLabs/taffy/actions/workflows/ci.yml)
[![crates.io](https://img.shields.io/crates/v/taffy.svg)](https://crates.io/crates/taffy)

`taffy` is a flexible, high-performance, cross-platform UI layout library written in [Rust](https://www.rust-lang.org).

Currently, we only support a [flexbox](https://css-tricks.com/snippets/css/a-guide-to-flexbox/) layout algorithm, but support for other paradigms [is planned](https://github.com/DioxusLabs/taffy/issues/28).

This crate is a collaborative, cross-team project, and is designed to be used as a dependency for other UI and GUI libraries.
Right now, it powers:

- [Dioxus](https://dioxuslabs.com/): a React-like library for building fast, portable, and beautiful user interfaces with Rust
- [Bevy](https://bevyengine.org/): an ergonomic, ECS-first Rust game engine

## Usage

### Through the LayoutTree trait

In order to be more flexible over the underlying storage, Taffy provides the `LayoutTree` trait. This trait assumes that your implementation will be storing Taffy-specific data, like style and layout information. For the `Flexbox` algorithm, Taffy expects that your implementation of this trait will provide Flexbox-specific data, like its flex rules and specified dimensions.

```rust, ignore
struct MyTree {
    specified_layouts: Vec<FlexboxLayout>,
    final_layouts: Vec<Layout>
}

impl LayoutTree for MyTree {
    // ...
    fn measure_node(&self, node: Node) -> Option<Size<f32>> {
        // custom node measuring technology goes here
    }
}
```

### TaffyECS directly

Taffy also provides the `TaffyECS` struct which stores all the necessary layout data required to perform layout calculations. Internally, Taffy uses a set of SlotMaps with generational indicies. In this configuration, you would store `TaffyECS` alongside your UI tree and keep Taffy's NodeID attached to your own NodeID.

```rust, ignore
let my_tree = Tree::new();
let mut taffy = Taffy::new();

let mut el = my_tree.createElement("div");
let taffy_node = taffy.create_node();

el.taffy_node.set(taffy_node);

taffy.compute_layout(el, Size(100, 100));
```

## Contributions

[Contributions welcome](https://github.com/DioxusLabs/taffy/blob/main/CONTRIBUTING.md):
if you'd like to use, improve or build `taffy`, feel free to join the conversation, open an [issue](https://github.com/DioxusLabs/taffy/issues) or submit a [PR](https://github.com/DioxusLabs/taffy/pulls).
If you have questions about how to use `taffy`, open a [discussion](https://github.com/DioxusLabs/taffy/discussions) so we can answer your questions in a way that others can find.
