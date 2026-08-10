# Scoping: `depends_on_block_size` as a standalone feature

A per-node, bottom-up-propagated answer to: *"can the block-axis constraint imposed on this subtree
change its inline (width) size?"*. Useful on its own — an embedder (e.g. Servo) can use it to skip
re-measuring a box's inline size when only the block-axis constraint changed, and to decide whether
an intrinsic-inline-size result may be reused across constraints. It is also the enabling primitive
for the inline-intrinsic-size cache (see `docs/inline-intrinsic-size-cache.md`), but it does not
depend on it and can land first.

## Semantics

`depends_on_block_size == false` is a promise:

> For a fixed `parent_size.width`, this node's outer width under a `MinContent`/`MaxContent` inline
> constraint is the same regardless of `known_dimensions.height`, `available_space.height` and
> `parent_size.height`.

`true` means "unknown / may depend". The flag is conservative in one direction only: a false
negative (reporting `false` when there is a dependence) is a correctness bug; a false positive is
only a missed optimisation. Everything not explicitly proven independent reports `true`.

It is a property of a node *and its subtree*, not of its style alone: a block container with a
`height: 100%` + `aspect-ratio` descendant depends on the block size even though its own style says
nothing about it.

## API shape

```rust
pub struct LayoutOutput {
    // ...existing fields
    /// Whether the block-axis constraint can affect this node's inline size. See docs.
    pub depends_on_block_size: bool,
}
```

- `LayoutOutput::HIDDEN` / `DEFAULT` → `false`; every other constructor
  (`from_outer_size`, `from_sizes`, `from_sizes_and_baselines`) → `true`, so third-party
  `LayoutPartialTree` implementations stay correct without changes.
- Read-back for embedders: `CacheTree`-independent accessor on the tree, e.g.
  `TaffyTree::depends_on_block_size(node) -> Option<bool>` (`None` before the node has been laid
  out once). Stored next to the cache entry so it survives across layout passes and is dropped by
  `Cache::clear`.
- Feature gating: the flag itself is ~1 byte in a struct that is already padded, and the
  propagation is an `|=` per child; not worth a cargo feature. The public accessor could live
  behind `taffy_tree` like the rest of the high-level API.

## Where it is computed

Bottom-up, mirroring the existing `content_size` propagation.

| Site | Rule |
| --- | --- |
| `compute_leaf_layout` | `style.aspect_ratio().is_some()` \|\| node has a measure function |
| `compute_flexbox_layout` | own `aspect_ratio`, `flex_direction.is_column() && flex_wrap != NoWrap`, OR of children |
| `compute_grid_layout` | own `aspect_ratio`, OR of children (row track sizes never feed back into column sizes, so grid itself adds nothing else) |
| `compute_block_layout` | own `aspect_ratio`, OR of children (including floats) |
| `compute_cached_layout` | records the flag alongside the cache entry; cached returns replay it |
| `compute_hidden_layout` | `false` |

Child-layout call sites that need the `|=`: 7 in `block.rs`, 9 in `flexbox.rs`, 4 across `grid/`.
Absolutely-positioned children can be skipped (they do not contribute to their container's
intrinsic width).

### Measure functions

A leaf with a measure function must report `true` unless the embedder says otherwise, because
`MeasureFunction` receives `available_space.height` and may use it. Two options, both cheap:

1. document the contract and let the embedder opt out via node context — no API change, but
   opt-out is per-tree rather than per-node;
2. widen the measure-function return to a small struct with a `depends_on_block_size` field
   (breaking, but the honest fix; text measurers would return `false` and immediately benefit).

Recommend (1) for the first cut with (2) noted in RELEASES.md as a candidate for the next breaking
release.

## Bootstrapping and invalidation

The flag is only known after a node has been laid out once, so the first query for a node is always
conservative; the value is recorded during that pass. Invalidation needs nothing new: mutating a
node clears its cache and every ancestor's, which is exactly the set of nodes whose flag can change.

One subtlety: during a `ComputeSize` pass an algorithm may short-circuit (e.g. `block.rs` returns
early when the width is fully determined by styles) without visiting its children. Such a pass must
report `true` rather than "no children depend on the block size", or record nothing at all — the
flag must only be *lowered* by a pass that actually visited the whole subtree.

## Testing

- generated tests are unaffected (no behaviour change) — the flag is observational
- hand-written tests asserting the flag on: a plain block/text tree (`false`), a node with
  `aspect-ratio` (`true`), an ancestor of such a node (`true`), a `flex-direction: column` +
  `flex-wrap: wrap` container (`true`), and a node whose aspect-ratio child is removed (`false`
  after re-layout, exercising invalidation)
- a `debug_assert`-only verification mode that re-measures the width under a second block-axis
  constraint whenever the flag is `false` and asserts equality would catch false negatives across
  the entire generated test suite. Cheap to add and worth having in CI's debug build.

## Effort

Roughly one session: the propagation is mechanical, the flag plumbing touches ~20 call sites, and
the risk is concentrated in the short-circuit paths described above. It is independent of the
`SizingMode::InherentSize` hoist and of the inline-intrinsic cache, and can be reviewed on its own.
