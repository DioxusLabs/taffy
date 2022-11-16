# taffy

[![GitHub CI](https://github.com/DioxusLabs/taffy/actions/workflows/ci.yml/badge.svg)](https://github.com/DioxusLabs/taffy/actions/workflows/ci.yml)
[![crates.io](https://img.shields.io/crates/v/taffy.svg)](https://crates.io/crates/taffy)
[![docs.rs](https://img.shields.io/docsrs/taffy)](https://docs.rs/taffy)

`taffy` is a flexible, high-performance, cross-platform UI layout library written in [Rust](https://www.rust-lang.org).

Currently, we only support a [flexbox](https://css-tricks.com/snippets/css/a-guide-to-flexbox/) layout algorithm, but support for other paradigms [is planned](https://github.com/DioxusLabs/taffy/issues/28).

This crate is a collaborative, cross-team project, and is designed to be used as a dependency for other UI and GUI libraries.
Right now, it powers:

- [Dioxus](https://dioxuslabs.com/): a React-like library for building fast, portable, and beautiful user interfaces with Rust
- [Bevy](https://bevyengine.org/): an ergonomic, ECS-first Rust game engine

## Usage

```rust
use taffy::prelude::*;

// First create an instance of Taffy
let mut taffy = Taffy::new();

// Create a tree of nodes using `taffy.new_leaf` and `taffy.new_with_children`.
// These functions both return a node id which can be used to refer to that node
// The FlexboxLayout struct is used to specify styling information
let header_node = taffy
    .new_leaf(
        FlexboxLayout {
            size: Size { width: Dimension::Points(800.0), height: Dimension::Points(100.0) },
            ..Default::default()
        },
    ).unwrap();

let body_node = taffy
    .new_leaf(
        FlexboxLayout {
            size: Size { width: Dimension::Points(800.0), height: Dimension::Undefined },
            flex_grow: 1.0,
            ..Default::default()
        },
    ).unwrap();

let root_node = taffy
    .new_with_children(
        FlexboxLayout {
            flex_direction: FlexDirection::Column,
            size: Size { width: Dimension::Points(800.0), height: Dimension::Points(600.0) },
            ..Default::default()
        },
        &[header_node, body_node],
    )
    .unwrap();

// Call compute_layout on the root of your tree to run the layout algorithm
taffy.compute_layout(root_node, Size::NONE).unwrap();

// Inspect the computed layout using taffy.layout
assert_eq!(taffy.layout(root_node).unwrap().size.width, 800.0);
assert_eq!(taffy.layout(root_node).unwrap().size.height, 600.0);
assert_eq!(taffy.layout(header_node).unwrap().size.width, 800.0);
assert_eq!(taffy.layout(header_node).unwrap().size.height, 100.0);
assert_eq!(taffy.layout(body_node).unwrap().size.width, 800.0);
assert_eq!(taffy.layout(body_node).unwrap().size.height, 500.0); // This value was not set explicitly, but was computed by Taffy

```

## Contributions

[Contributions welcome](https://github.com/DioxusLabs/taffy/blob/main/CONTRIBUTING.md):
if you'd like to use, improve or build `taffy`, feel free to join the conversation, open an [issue](https://github.com/DioxusLabs/taffy/issues) or submit a [PR](https://github.com/DioxusLabs/taffy/pulls).
If you have questions about how to use `taffy`, open a [discussion](https://github.com/DioxusLabs/taffy/discussions) so we can answer your questions in a way that others can find.
