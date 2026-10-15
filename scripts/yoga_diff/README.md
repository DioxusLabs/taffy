# yoga_diff

Differential harness for the `yoga_flexbox` port: lays out the same JSON trees with C++ Yoga (via the
[DioxusLabs/yoga-rs](https://github.com/DioxusLabs/yoga-rs) crate, compiled in-process) and with Taffy, and
compares layouts and measure/baseline call logs bit-for-bit (`f32::to_bits`, not a tolerance).

Building the `yoga` crate needs a C++ toolchain and `libclang` (`apt install libclang-dev clang`), so this
crate is excluded from the workspace.

```sh
cd scripts/yoga_diff
cargo run --release -- gen cases/random 2000            # random RN-like trees (seeded, stable)
cargo run --release -- gen cases/quirks 2000 --quirks --rtl --seed-offset 100000
cargo run --release -- run cases/random --matrix         # Yoga vs Taffy on every configuration
cargo run --release -- run fixtures --errata none --scale 0 --show 20
cargo run --release -- record fixtures --out recorded    # write Yoga's layouts + call logs
cargo run --release -- check recorded                    # replay the recorded expectations against Taffy
```

Case format (the Yoga capture format used by `gen.py`, plus `steps`):

```json
{
  "layout-inputs": { "available-width": 360, "available-height": 640, "owner-direction": "ltr" },
  "config": { "errata": "all", "experimental-features": ["fix-flex-basis-fit-content"], "point-scale-factor": 2 },
  "tree": {
    "style": { "flex-direction": "row", "margin-start": { "unit": "px", "value": 4 } },
    "node": { "always-forms-containing-block": true },
    "x-text": { "n": 5, "ww": 30, "lh": 20, "baseline": true },
    "children": []
  },
  "steps": [
    { "set-style": { "path": [0], "style": { "width": { "unit": "pct", "value": 50 } } } },
    { "mark-dirty": { "path": [0] } },
    { "layout-inputs": { "available-width": 240, "available-height": 640, "owner-direction": "rtl" } }
  ]
}
```

Style keys are Yoga's (`position-type`, `margin-<edge>` with Yoga's nine edges, `flex`, `gap`, ...). `x-text`
nodes get a measure function for `n` words of width `ww` on lines of height `lh`. Each step mutates the tree
and triggers another layout, so cache and dirty-marking behaviour is compared too.

Errata names: `all`, `none`, `classic`, `default`, or `|`-joined `stretch-flex-basis`,
`absolute-position-without-insets-excludes-padding`, `absolute-percent-against-inner-size`,
`min-size-undefined-instead-of-auto`, `flex-first-pass-uses-running-totals`. `--matrix` runs RN's three
configurations first (`all`, `min-size-undefined-instead-of-auto`, `none`, each with and without
`fix-flex-basis-fit-content`), then `classic`, Yoga's default, each errata bit alone, `web-flex-basis`,
and point scale factors 1, 2, 3 and 2.625.

Until `Display::YogaFlexbox` lands the Taffy side runs Taffy's CSS flexbox with Yoga's defaults, so `run`
reports the Yoga/CSS differences rather than port bugs; `record` output is what the port is checked against.
