# taffy

[![GitHub CI](https://github.com/DioxusLabs/taffy/actions/workflows/ci.yml/badge.svg)](https://github.com/DioxusLabs/taffy/actions/workflows/ci.yml)
[![crates.io](https://img.shields.io/crates/v/taffy.svg)](https://crates.io/crates/taffy)
[![docs.rs](https://img.shields.io/docsrs/taffy)](https://docs.rs/taffy)

`taffy` is a flexible, high-performance, cross-platform UI layout library written in [Rust](https://www.rust-lang.org).

Currently Taffy implements only CSS based layout algorithms:

- The stable `0.2.x` releases of Taffy implements the [flexbox](https://css-tricks.com/snippets/css/a-guide-to-flexbox/) layout algorithm.
- Support for [CSS Grid](https://css-tricks.com/snippets/css/complete-guide-grid/) is in preview. If you wish to try this out then you should use the `0.3.x` alpha releases ~and enable the `experimental_grid` cargo feature~ (from `0.3.0-alpha2` the CSS Grid feature is enabled by default). For information, see the [release notes](https://github.com/DioxusLabs/taffy/blob/main/RELEASES.md) and the [tracking issue](https://github.com/DioxusLabs/taffy/issues/204). Experimentation with the CSS Grid implementation is encouraged, and feedback and bug reports are welcomed.

Support for other paradigms [is planned](https://github.com/DioxusLabs/taffy/issues/28).

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
// The Style struct is used to specify styling information
let header_node = taffy
    .new_leaf(
        Style {
            size: Size { width: points(800.0), height: points(100.0) },
            ..Default::default()
        },
    ).unwrap();

let body_node = taffy
    .new_leaf(
        Style {
            size: Size { width: points(800.0), height: auto() },
            flex_grow: 1.0,
            ..Default::default()
        },
    ).unwrap();

let root_node = taffy
    .new_with_children(
        Style {
            flex_direction: FlexDirection::Column,
            size: Size { width: points(800.0), height: points(600.0) },
            ..Default::default()
        },
        &[header_node, body_node],
    )
    .unwrap();

// Call compute_layout on the root of your tree to run the layout algorithm
taffy.compute_layout(root_node, Size::MAX_CONTENT).unwrap();

// Inspect the computed layout using taffy.layout
assert_eq!(taffy.layout(root_node).unwrap().size.width, 800.0);
assert_eq!(taffy.layout(root_node).unwrap().size.height, 600.0);
assert_eq!(taffy.layout(header_node).unwrap().size.width, 800.0);
assert_eq!(taffy.layout(header_node).unwrap().size.height, 100.0);
assert_eq!(taffy.layout(body_node).unwrap().size.width, 800.0);
assert_eq!(taffy.layout(body_node).unwrap().size.height, 500.0); // This value was not set explicitly, but was computed by Taffy

```

## Benchmarks (vs. [Yoga](https://github.com/facebook/yoga))

- Run on a 2021 MacBook Pro with M1 Pro processor using [criterion](https://github.com/bheisler/criterion.rs)
- The benchmarks measure layout computation only. They do not measure tree creation.
- Yoga benchmarks were run via the [yoga](https://github.com/bschwind/yoga-rs) crate (Rust bindings)
- Most popular websites seem to have between 3,000 and 10,000 nodes (although they also require text layout, which neither yoga nor taffy implement).

Note that the table below contains multiple different units (milliseconds vs. microseconds

| Benchmark          | Node Count | Depth | Yoga ([ba27f9d]) | Taffy ([9059647]) |
| ---                | ---        | ---   | ---              | ---               |
| yoga 'huge nested' | 1,000      | 3     | 411.42 µs        | 275.47 µs         |
| yoga 'huge nested' | 10,000     | 4     | 3.9882 ms        | 3.9409 ms         |
| yoga 'huge nested' | 100,000    | 5     | 46.117 ms        | 32.458 ms         |
| big trees (wide)   | 1,000      | 1     | 750.75 µs        | 571.35 µs         |
| big trees (wide)   | 10,000     | 1     | 7.1639 ms        | 7.4838 ms         |
| big trees (wide)   | 100,000    | 1     | 132.17 ms        | 245.16 ms         |
| big trees (deep)   | 4,000      | 12    | 2.3140 ms        | 2.0300 ms         |
| big trees (deep)   | 10,000     | 14    | 6.0009 ms        | 5.1872 ms         |
| big trees (deep)   | 100,000    | 17    | 76.954 ms        | 74.946 ms         |
| super deep         | 1,000      | 1,000 | 563.15 µs        | 548.97 µs         |

[ba27f9d]: https://github.com/facebook/yoga/commit/ba27f9d1ecfa7518019845b84b035d3d4a2a6658
[9059647]: https://github.com/DioxusLabs/taffy/commit/ba27f9d1ecfa7518019845b84b035d3d4a2a6658

## Contributions

[Contributions welcome](https://github.com/DioxusLabs/taffy/blob/main/CONTRIBUTING.md):
if you'd like to use, improve or build `taffy`, feel free to join the conversation, open an [issue](https://github.com/DioxusLabs/taffy/issues) or submit a [PR](https://github.com/DioxusLabs/taffy/pulls).
If you have questions about how to use `taffy`, open a [discussion](https://github.com/DioxusLabs/taffy/discussions) so we can answer your questions in a way that others can find.
