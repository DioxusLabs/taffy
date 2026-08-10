# Design: a dedicated cache for inline-axis (width) intrinsic sizes

Status: investigation / design proposal. A prototype of "Stage 1" is implemented in `src/tree/cache.rs`.

## 1. The current cache

`src/tree/cache.rs` stores, per node:

- one `final_layout_entry` (`RunMode::PerformLayout`), keyed on the full `CacheKey`
- nine `measure_entries` (`RunMode::ComputeSize`), each holding a `Size<f32>`

The `CacheKey` packs:

- `kd_available_space: u64` — for each axis, the `known_dimension` bits if `Some`, otherwise the
  `AvailableSpace` bits (`Definite(v)` is stored as `-v` so it can never collide with a known
  dimension, `MinContent` = `-inf`, `MaxContent` = `+inf`)
- `parent_size: u64` — both parent axes, with two spare sign bits used to encode `RequestedAxis`

Two things are worth noting about `Cache::get` in `ComputeSize` mode:

1. It does a **linear scan** over all nine slots and matches on `kd_available_space` plus only the
   **x-axis half** of `parent_size` (`x_axis_parent_size()`). So the parent's height is already
   deliberately ignored, and the `RequestedAxis` bits are masked out too: an entry stored by a
   `Both` request can satisfy a `Horizontal` request and vice versa.
2. `compute_cache_slot` only decides *where to write*, not what matches. Slots exist purely to stop
   later measurements clobbering earlier ones.

### Which slots serve width-intrinsic queries today

A "width intrinsic size" query is `run_mode = ComputeSize`, `axis = Horizontal`,
`known_dimensions.width = None`, `available_space.width ∈ {MinContent, MaxContent}`. Given the slot
rules, such a query lands in:

| known_dimensions.height | available_space.width | available_space.height | slot |
| --- | --- | --- | --- |
| `Some` | Min/MaxContent | any | 3 or 4 |
| `None` | MaxContent | MaxContent/Definite | 5 |
| `None` | MaxContent | MinContent | 6 |
| `None` | MinContent | MaxContent/Definite | 7 |
| `None` | MinContent | MinContent | 8 |

So a single node that is asked for both its min-content and max-content width under two different
block-axis constraints consumes **four** of the nine slots, and those same slots are the ones the
height-measuring passes want (`axis = Vertical` requests with no known dimensions also land in
5–8). Grid track sizing in particular alternates between "min-content width", "max-content width",
"min-content height" and "max-content height" queries for the same item, so slots are evicted and
re-filled repeatedly.

The main callers of width-intrinsic queries are:

- `compute/block.rs::determine_content_based_container_width` — shrink-to-fit width; passes
  `available_space = { width: <inherited>, height: MinContent }`
- `compute/grid/types/grid_item.rs::{min,max}_content_contribution` — column track sizing; passes
  `available_space.height = Min/MaxContent` depending on the pass, plus a per-track-sizing-run
  memo (`min_content_contribution_cache`) that shadows the node cache
- `compute/flexbox.rs::determine_flex_base_size` — for `flex-direction: row`, `axis = Horizontal`
  with `available_space = { width: Min/MaxContent, height: <cross constraint> }`
- `compute/float.rs::FloatIntrinsicWidthCalculator`

## 2. Why a dedicated cache is possible

Per [css-sizing-3 §5](https://www.w3.org/TR/css-sizing-3/#intrinsic-sizes), in horizontal writing
modes a box's min-content and max-content **inline** sizes are *constraint-independent*: they are a
property of the box and its contents, not of the space it is being sized into. Concretely they do
not depend on:

- `available_space` (in either axis)
- `known_dimensions.width` (by definition — the query only exists when the width is unknown)
- `parent_size.height`

They *can* depend on `parent_size.width`, because percentage padding/border/margin (in **both**
axes — CSS resolves vertical percentage padding against the containing block's inline size too)
resolve against it. `compute_leaf_layout` and all three container algorithms do exactly this
(`style.padding().resolve_or_zero(parent_size.width, ..)`). Percentage `width`/`min-width` styles
also resolve against `parent_size.width`, but those disappear from the algorithms once
`SizingMode::InherentSize` is hoisted into the parent-side helper.

So the natural key for an inline intrinsic size is `(min-vs-max, parent_size.width)` — two scalars
per node — rather than the full 128-bit `CacheKey`.

## 3. Proposed design

### Storage

```rust
struct InlineIntrinsicEntry { parent_width: u32 /* f32 bits, INF == None */, width: f32 }

pub struct Cache {
    final_layout_entry: Option<CacheEntry<LayoutOutput>>,
    measure_entries: [Option<CacheEntry<Size<f32>>>; 9],
    inline_intrinsic_entries: [Option<InlineIntrinsicEntry>; 2], // [min-content, max-content]
    is_empty: bool,
}
```

8 bytes per entry, 16 bytes per node, and it is cleared by the existing `Cache::clear` path, so
invalidation is already correct (style/child mutations call `cache_clear` on the node and all its
ancestors via `mark_dirty`).

### Where the lookup lives

**In `Cache::get`/`Cache::store`**, not in the parent-side helper. Reasons:

- every caller benefits without touching four algorithms
- the cache is where `parent_size.width`-keying already happens
- the planned parent-side helper (`LayoutPartialTreeExt`) sits *above* `compute_cached_layout`, so
  a lookup there would have to duplicate the cache-clear/invalidation plumbing and would not be
  reachable from `compute_child_layout` callers that bypass the helper

The predicate is cheap and purely a function of `LayoutInput`:

```rust
run_mode == ComputeSize
  && axis == Horizontal
  && known_dimensions.width.is_none()
  && known_dimensions.height.is_none()   // see caveats
  && matches!(available_space.width, MinContent | MaxContent)
```

Hits then compare only `parent_size.width`, and hit/miss is O(1) rather than a scan of nine slots.
Because a width-only request already returns `LayoutOutput::from_outer_size({ width, height: 0.0 })`
from all three algorithms, storing just the width loses nothing.

### Interaction with the planned `SizingMode::InherentSize` removal

The two changes compose well and the ordering does not matter much, but doing
`InherentSize`-hoisting first makes the new cache strictly more valuable:

- with inherent sizes applied above the cache, algorithms always run content-size semantics, so the
  only remaining `parent_size` dependency is percentage box properties. If we additionally
  normalised measurements to **content-box** values (or resolved percentage padding against the
  node's own resolved width, as `block.rs` now does for the final layout pass), `parent_size.width`
  could be dropped from the key entirely and the entries would become unconditional.
- for future `width: min-content | max-content | fit-content`, the parent-side helper resolves the
  keyword by issuing exactly one of these queries; each resolves to a scalar lookup that is shared
  with every other consumer (flex-basis, grid contributions, block shrink-to-fit) instead of
  competing for slots 5–8. `fit-content` needs both scalars plus the available space, which is a
  pure `clamp(min, available, max)` above the cache — no extra layout pass.

## 4. Correctness caveats

These are the cases where taffy's measured width is **not** constraint-independent. The prototype
handles them by bypassing the dedicated cache (falling back to the existing slots).

1. **`aspect-ratio` with a definite block size.** `width = height × ratio`, so the result depends on
   `known_dimensions.height`. Crucially this is *not* only a property of the node itself: a block
   container with a known height can have a `height: 100%` child with an `aspect-ratio`, so the
   transfer happens arbitrarily deep in the subtree. There is no cheap style-only test for "no
   descendant transfers block size to inline size", so the prototype simply excludes any request
   with `known_dimensions.height.is_some()`. This is the only exclusion needed in practice, and it
   costs little: the callers listed above almost always pass `known_dimensions.height = None` when
   asking for a width (the exception is grid, which passes the grid-area size).
   Note that the *pure* transferred-size case (aspect-ratio + definite **style** height, no content
   measurement) is unaffected — after the `InherentSize` refactor it is resolved from styles in the
   parent-side helper and never reaches the cache.
2. **Column-direction flexbox with `flex-wrap: wrap`.** taffy's flexbox treats a `MinContent`
   main-axis constraint as "take every wrapping opportunity" (`flexbox.rs`, line ~921). For a
   column container the main axis is the block axis, so `available_space.height` changes the number
   of columns and therefore the container's width. This is the one place where collapsing over
   `available_space.height` is theoretically unsound. It does not show up in the test suite (all
   5541 generated tests plus the hand-written suite pass with the prototype), because a wrap-column
   container is only ever asked for its width under an indefinite block constraint in the current
   algorithms — but it is a real hazard and should be revisited if/when the block-axis constraint
   plumbing changes. A conservative alternative is to also key the entries on a single bit
   ("block-axis constraint is `MinContent`"), which costs one extra entry pair and removes the
   hazard entirely.
3. **Custom measure functions that read `available_space.height`.** The `MeasureFunction` contract
   allows it. Text measurement (the motivating case) does not, but a "shrink text to fit height"
   measurer would. Worth documenting on `MeasureFunction` as "the inline-axis result must not
   depend on the block-axis constraint".
4. **Percentage-sized children.** Safe: percentage *inline* sizes against an indefinite width behave
   as auto, and percentage *padding/border/margin* resolve against `parent_size.width`, which is
   part of the key. Percentage block sizes only affect the block axis unless combined with
   `aspect-ratio`, which is case 1.
5. **Floats (`float_layout` feature).** `FloatIntrinsicWidthCalculator` computes the container's
   intrinsic width from float and non-float child widths under a given inline constraint; it is
   itself inline-axis only and needs no special handling. Float *placement* depends on the block
   axis, but that only affects the container's height.

## 4a. Tracking block-axis dependence instead of bypassing

Caveats 1–3 are all the same question: "does anything in this subtree let the block-axis constraint
affect the inline size?". That is trackable, and doing so would let the dedicated cache serve
requests with a known height and collapse over `available_space.height` unconditionally.

The propagation fits the existing plumbing: add a `block_axis_affects_inline_size: bool` to
`LayoutOutput`, computed bottom-up.

- `compute_leaf_layout`: `style.aspect_ratio().is_some()` — plus `true` for nodes with a measure
  function, unless `MeasureFunction` grows a way to declare that its inline result ignores the
  block-axis constraint (a `const` on the measure trait, or a new `Size<AvailableSpace>` contract
  note; conservatively `true` today).
- containers: own style (`aspect_ratio`, and for flexbox `flex_direction.is_column() && is_wrap`)
  OR'd with the flag from each child's `LayoutOutput`.
- `compute_cached_layout` stores the flag next to the cache entry so cached returns carry it, and
  `Cache::clear` drops it. Invalidation is already correct: mutating a node clears its cache and
  every ancestor's, which is exactly the set of nodes whose flag could change.

Cost is one bool in `LayoutOutput` (which is already padded) and one in `Cache`, plus an `|=` per
child in each algorithm's child loop. No extra traversal.

The one wrinkle is bootstrapping: the flag is only known after the subtree has been laid out at
least once, so the first inline-intrinsic query for a node must still take the conservative path
(measure, then record the flag). That is fine — it is exactly the query that populates the entry.
The second and subsequent queries, which are the ones the cache exists for, can use it.

This is worth doing as Stage 1b rather than Stage 1: it subsumes the `known_dimensions.height`
bypass, removes the wrap-column hazard entirely, and the same flag is directly reusable for the
future `width: min-content | max-content` keyword resolution (a node whose flag is false can have
its keyword width resolved once and reused for every constraint).

Prototype = Stage 1 as described above (bypass when `known_dimensions.height.is_some()`).

**Correctness:** `cargo test --workspace` is fully green (5541 generated + 61 hand-written tests).

**Cache statistics** (instrumented build, synthetic mixed flex/grid/block trees, layout from
cold cache):

| tree | ComputeSize gets | hit rate | of which inline-intrinsic | inline hit rate | slot writes |
| --- | --- | --- | --- | --- | --- |
| depth 3 × width 4 (85 nodes) | 1060 | 57.4% | 136 | 70.6% | 412 (was 452) |
| depth 4 × width 3 (121 nodes) | 2010 | 58.4% | 354 | 78.0% | 759 (was 837) |
| depth 5 × width 3 (364 nodes) | 6450 | 59.5% | 1233 | 80.5% | 2370 (was 2610) |

~19% of all `ComputeSize` cache queries are pure inline-intrinsic queries, they hit ~80% of the
time, and they no longer consume (or evict) any of the nine general slots — about 9–10% fewer
writes into the slot array.

**A more aggressive variant** (also using the dedicated cache when `known_dimensions.height` is
set — i.e. accepting caveat 1) passes every generated test and reduces the leaf measure count in
`tests/hand_written/caching.rs` from 7 to 5, but it is unsound in the presence of aspect-ratio
transfers and is not recommended.

**Benchmarks:** inconclusive on the machine used. `benches/` on a shared VM had run-to-run noise up
to ±80% (a baseline-vs-baseline comparison reported "performance regressed by 78%"). Interleaved
repeated runs of `grid/deep` showed the prototype consistently 2–4% faster
(`grid/deep/3x3/6561`: 24.5 ms → 23.8 ms; `grid/deep/2x2/16384`: 94.3 ms → 92.9 ms), which is the
right order of magnitude for removing a nine-slot linear scan from a fifth of all cache queries.
Benchmarks should be re-run on quiet hardware before drawing conclusions.

## 6. Recommendation

1. Land the `SizingMode::InherentSize` hoist first.
2. Land Stage 1 (dedicated two-entry inline-intrinsic cache in `Cache`, gated on
   `known_dimensions.height.is_none()`), plus a documentation note on `MeasureFunction` about
   caveat 3. This is ~40 lines and needs no algorithm changes.
3. Stage 2, once measurements are content-box/percentage-independent: drop `parent_size.width` from
   the key so the two entries become unconditional per-node values. At that point the same storage
   can back `width: min-content | max-content | fit-content` keyword resolution in the parent-side
   helper at zero extra cost.
4. Do **not** extend the dedicated cache to the block axis: block-axis intrinsic sizes genuinely
   depend on the imposed inline size and must stay keyed on it.
