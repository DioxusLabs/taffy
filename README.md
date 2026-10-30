<!-- markdownlint-disable-next-line MD041 -->
<p>
<picture>
  <img src="assets/logo.svg" alt="Taffy" height="70">
</picture>
</p>

[![GitHub CI](https://github.com/DioxusLabs/taffy/actions/workflows/ci.yml/badge.svg)](https://github.com/DioxusLabs/taffy/actions/workflows/ci.yml)
[![crates.io](https://img.shields.io/crates/v/taffy.svg)](https://crates.io/crates/taffy)
[![docs.rs](https://img.shields.io/docsrs/taffy)](https://docs.rs/taffy)
![Crates.io MSRV](https://img.shields.io/crates/msrv/taffy)

Taffy is a flexible, high-performance, cross-platform UI layout library written in [Rust](https://www.rust-lang.org).

It currently implements the CSS **Block**, **Flexbox** and **CSS Grid** layout algorithms. Support for other paradigms is planned. For more information on this and other future development plans see the [roadmap issue](https://github.com/DioxusLabs/taffy/issues/345).

This crate is a collaborative, cross-team project, and is designed to be used as a dependency for other UI and GUI libraries.
Right now, it powers:

- [Servo](https://github.com/servo/servo): an alternative web browser
- [Blitz](https://github.com/DioxusLabs/blitz): a radically modular web engine
- [Bevy](https://bevyengine.org/): an ergonomic, ECS-first Rust game engine
- [Takumi](https://github.com/kane50613/takumi): Renders your React components to images
- [iocraft](https://github.com/ccbrown/iocraft): crafting beautiful interfaces for the terminal
- [Slint](https://github.com/slint-ui/slint): a declarative GUI toolkit for building native user interfaces
- The [Lapce](https://lapce.dev/) text editor via the [Floem](https://github.com/lapce/floem) UI framework
- The [Zed](https://zed.dev/) text editor via the [GPUI](https://github.com/zed-industries/zed/tree/main/crates/gpui) UI framework

## Usage

```rust
use taffy::prelude::*;

// First create an instance of TaffyTree
let mut tree : TaffyTree<()> = TaffyTree::new();

// Create a tree of nodes using `TaffyTree.new_leaf` and `TaffyTree.new_with_children`.
// These functions both return a node id which can be used to refer to that node
// The Style struct is used to specify styling information
let header_node = tree
    .new_leaf(
        Style {
            size: Size { width: length(800.0), height: length(100.0) },
            ..Default::default()
        },
    ).unwrap();

let body_node = tree
    .new_leaf(
        Style {
            size: Size { width: length(800.0), height: auto() },
            flex_grow: 1.0,
            ..Default::default()
        },
    ).unwrap();

let root_node = tree
    .new_with_children(
        Style {
            flex_direction: FlexDirection::Column,
            size: Size { width: length(800.0), height: length(600.0) },
            ..Default::default()
        },
        &[header_node, body_node],
    )
    .unwrap();

// Call compute_layout on the root of your tree to run the layout algorithm
tree.compute_layout(root_node, Size::MAX_CONTENT).unwrap();

// Inspect the computed layout using `TaffyTree.layout`
assert_eq!(tree.layout(root_node).unwrap().size.width, 800.0);
assert_eq!(tree.layout(root_node).unwrap().size.height, 600.0);
assert_eq!(tree.layout(header_node).unwrap().size.width, 800.0);
assert_eq!(tree.layout(header_node).unwrap().size.height, 100.0);
assert_eq!(tree.layout(body_node).unwrap().size.width, 800.0);
assert_eq!(tree.layout(body_node).unwrap().size.height, 500.0); // This value was not set explicitly, but was computed by Taffy

```

## Bindings to other languages

- Python via [stretchable](https://github.com/mortencombat/stretchable)
- [WIP C bindings](https://github.com/DioxusLabs/taffy/pull/404)
- [WIP WASM bindings](https://github.com/DioxusLabs/taffy/pull/394)

## Learning Resources

Taffy implements the Flexbox and CSS Grid specifications faithfully, so documentation designed for the web should translate cleanly to Taffy's implementation. For reference documentation on individual style properties we recommend the MDN documentation (for example [this page](https://developer.mozilla.org/en-US/docs/Web/CSS/width) on the `width` property). Such pages can usually be found by searching for "MDN property-name" using a search engine.

If you are interested in guide-level documentation on CSS layout, then we recommend the following resources:

### Flexbox

- [Flexbox Froggy](https://flexboxfroggy.com/). This is an interactive tutorial/game that allows you to learn the essential parts of Flexbox in a fun engaging way.
- [A Complete Guide To Flexbox](https://css-tricks.com/snippets/css/a-guide-to-flexbox/) by CSS Tricks. This is detailed guide with illustrations and comprehensive written explanation of the different Flexbox properties and how they work.

### CSS Grid

- [CSS Grid Garden](https://cssgridgarden.com/). This is an interactive tutorial/game that allows you to learn the essential parts of CSS Grid in a fun engaging way.
- [A Complete Guide To CSS Grid](https://css-tricks.com/snippets/css/complete-guide-grid/) by CSS Tricks. This is detailed guide with illustrations and comprehensive written explanation of the different CSS Grid properties and how they work.

## Benchmarks (vs. [Yoga](https://github.com/facebook/yoga))

- Run on a 2021 MacBook Pro with M1 Pro processor using [criterion](https://github.com/bheisler/criterion.rs)
- The benchmarks measure layout computation only. They do not measure tree creation.
- Yoga benchmarks were run via the [yoga](https://github.com/bschwind/yoga-rs) crate (Rust bindings), version 0.5.0
- Most popular websites seem to have between 3,000 and 10,000 nodes (although they also require text layout, which neither yoga nor taffy implement).

Note that the table below contains multiple different units (seconds vs. milliseconds vs. microseconds)

| Benchmark                     | Node Count | Depth | Yoga ([v3.2.1])        | Taffy ([e747f6e]) |
| ---                           | ---        | ---   | ---                    | ---               |
| yoga 'huge nested'            | 1,000      | 3     | 328.24 µs              | 250.23 µs         |
| yoga 'huge nested'            | 10,000     | 4     | 3.2892 ms              | 2.6947 ms         |
| yoga 'huge nested'            | 100,000    | 5     | 41.686 ms              | 32.871 ms         |
| big trees (wide)              | 1,000      | 2     | 468.66 µs              | 471.14 µs         |
| big trees (wide)              | 10,000     | 2     | 4.8667 ms              | 4.8032 ms         |
| big trees (wide)              | 100,000    | 2     | 63.304 ms              | 58.111 ms         |
| absolutely positioned leaves  | 1,000      | 2     | 408.51 µs              | 259.91 µs         |
| absolutely positioned leaves  | 10,000     | 2     | 4.3898 ms              | 2.7890 ms         |
| absolutely positioned leaves  | 100,000    | 2     | 56.490 ms              | 41.270 ms         |
| big trees (deep, random size) | 4,000      | 12    | 1.7937 ms              | 1.8160 ms         |
| big trees (deep, random size) | 10,000     | 14    | 4.4436 ms              | 4.5843 ms         |
| big trees (deep, random size) | 100,000    | 17    | 63.555 ms              | 69.952 ms         |
| big trees (deep, auto size)   | 4,000      | 12    | 32.872 ms              | 4.7357 ms         |
| big trees (deep, auto size)   | 10,000     | 14    | 103.19 ms              | 11.894 ms         |
| big trees (deep, auto size)   | 100,000    | 17    | 2.3502 s               | 164.24 ms         |
| super deep                    | 150        | 50    | 61.132 ms              | 179.02 µs         |
| super deep                    | 300        | 100   | did not finish (>9min) | 366.89 µs         |

[v3.2.1]: https://github.com/facebook/yoga/commit/042f5013152eb81c1552dec945b88f7b95ca350f
[e747f6e]: https://github.com/DioxusLabs/taffy/commit/e747f6e5b94e7b0586d807aeb681b48f9a005f04

## Contributions

[Contributions welcome](https://github.com/DioxusLabs/taffy/blob/main/CONTRIBUTING.md):
if you'd like to use, improve or build `taffy`, feel free to join the conversation, open an [issue](https://github.com/DioxusLabs/taffy/issues) or submit a [PR](https://github.com/DioxusLabs/taffy/pulls).
If you have questions about how to use `taffy`, open a [discussion](https://github.com/DioxusLabs/taffy/discussions) so we can answer your questions in a way that others can find.
