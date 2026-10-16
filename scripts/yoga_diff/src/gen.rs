//! Random RN-like trees (a port of the original `gen.py`, same shape of distribution), with a
//! deterministic RNG so a seed always produces the same case.

use crate::case::{Case, LayoutInputs, TextSpec, TreeNode, V};
use serde_json::{json, Value};

/// PCG32 (fixed increment), so generated cases are stable across Rust and `rand` versions.
pub struct Rng(u64);

impl Rng {
    const MUL: u64 = 6364136223846793005;
    const INC: u64 = 1442695040888963407;

    pub fn new(seed: u64) -> Rng {
        let mut r = Rng(0);
        r.0 = seed.wrapping_add(Self::INC);
        r.next_u32();
        r
    }

    pub fn next_u32(&mut self) -> u32 {
        let old = self.0;
        self.0 = old.wrapping_mul(Self::MUL).wrapping_add(Self::INC);
        let xorshifted = (((old >> 18) ^ old) >> 27) as u32;
        xorshifted.rotate_right((old >> 59) as u32)
    }

    /// Uniform in [0, 1).
    pub fn f(&mut self) -> f32 {
        (self.next_u32() >> 8) as f32 / (1u32 << 24) as f32
    }

    /// Uniform integer in [a, b].
    pub fn int(&mut self, a: i64, b: i64) -> i64 {
        a + (self.next_u32() as i64 % (b - a + 1))
    }

    pub fn pick<'a, T>(&mut self, xs: &'a [T]) -> &'a T {
        &xs[self.int(0, xs.len() as i64 - 1) as usize]
    }

    fn px(&mut self, a: i64, b: i64) -> Value {
        V::px(self.int(a, b) as f32)
    }

    fn pct(&mut self, a: i64, b: i64) -> Value {
        V::pct(self.int(a, b) as f32)
    }
}

#[derive(Clone, Copy, Default)]
pub struct Profile {
    pub rtl: bool,
    /// Bias towards the areas where Yoga is known to disagree with CSS.
    pub quirks: bool,
    pub no_flex: bool,
}

fn node(depth: u32, rng: &mut Rng, p: Profile) -> TreeNode {
    let mut s = serde_json::Map::new();
    let mut set = |k: &str, v: Value| {
        s.insert(k.to_string(), v);
    };
    let r = |rng: &mut Rng| rng.f();
    if r(rng) < 0.4 {
        set("flex-direction", json!(rng.pick(&["row", "row", "column", "row-reverse", "column-reverse"])));
    }
    if r(rng) < 0.3 {
        set(
            "justify-content",
            json!(rng.pick(&["center", "space-between", "flex-end", "space-around", "space-evenly"])),
        );
    }
    if r(rng) < 0.3 {
        set("align-items", json!(rng.pick(&["center", "flex-start", "flex-end", "stretch"])));
    }
    if r(rng) < 0.15 {
        set("align-self", json!(rng.pick(&["center", "flex-start", "flex-end", "stretch"])));
    }
    if r(rng) < 0.1 {
        set("flex-wrap", json!("wrap"));
    }
    if r(rng) < 0.1 {
        set("align-content", json!(rng.pick(&["center", "flex-start", "flex-end", "stretch", "space-between"])));
    }
    if r(rng) < 0.2 && !p.no_flex {
        set("flex", json!(*rng.pick(&[1.0, 1.0, 2.0])));
    }
    if r(rng) < 0.1 {
        set("flex-grow", json!(*rng.pick(&[0.0, 1.0, 2.0])));
    }
    if r(rng) < 0.15 {
        set("flex-shrink", json!(*rng.pick(&[0.0, 1.0, 1.0])));
    }
    if r(rng) < 0.08 && !p.no_flex {
        let v = if rng.f() < 0.5 { rng.px(0, 150) } else { rng.pct(10, 60) };
        set("flex-basis", v);
    }
    if r(rng) < 0.35 {
        let v = if rng.f() < 2.0 / 3.0 { rng.px(10, 200) } else { rng.pct(10, 100) };
        set("width", v);
    }
    if r(rng) < 0.35 {
        let v = if rng.f() < 2.0 / 3.0 { rng.px(10, 150) } else { rng.pct(10, 100) };
        set("height", v);
    }
    if r(rng) < 0.08 {
        let v = rng.px(10, 120);
        set("min-width", v);
    }
    if r(rng) < 0.08 {
        let v = rng.px(10, 120);
        set("min-height", v);
    }
    if r(rng) < 0.08 {
        let v = if rng.f() < 0.5 { rng.px(20, 200) } else { rng.pct(20, 90) };
        set("max-width", v);
    }
    if r(rng) < 0.06 {
        let v = rng.px(20, 200);
        set("max-height", v);
    }
    for e in ["top", "bottom", "start", "end", "horizontal", "vertical", "left", "right", "all"] {
        if r(rng) < 0.06 {
            let v = if rng.f() < 2.0 / 3.0 { rng.px(0, 20) } else { json!("auto") };
            set(&format!("margin-{e}"), v);
        }
        if r(rng) < 0.06 {
            let v = rng.px(0, 20);
            set(&format!("padding-{e}"), v);
        }
    }
    if r(rng) < 0.1 {
        let v = rng.px(1, 3);
        set("border-all", v);
    }
    if r(rng) < 0.15 {
        set("overflow", json!(rng.pick(&["hidden", "scroll"])));
    }
    if r(rng) < 0.05 {
        let v = rng.px(2, 12);
        set("gap", v);
    }
    if depth > 0 && r(rng) < 0.12 {
        set("position-type", json!("absolute"));
        for e in ["top", "left", "right", "bottom"] {
            if r(rng) < 0.4 {
                let v = if rng.f() < 0.5 { rng.px(0, 30) } else { rng.pct(0, 50) };
                set(&format!("position-{e}"), v);
            }
        }
    }
    if depth > 0 && r(rng) < 0.02 {
        set("display", json!("none"));
    }
    if p.quirks {
        // Padding/border larger than the box, static position, content-box, aspect ratio, percent gaps,
        // explicit zero/auto minimums and baseline alignment.
        if r(rng) < 0.05 {
            let v = rng.px(40, 120);
            set("padding-all", v);
            let w = rng.px(10, 60);
            set("width", w);
        }
        if depth > 0 && r(rng) < 0.06 {
            set("position-type", json!("static"));
        }
        if r(rng) < 0.06 {
            set("box-sizing", json!("content-box"));
        }
        if r(rng) < 0.05 {
            set("aspect-ratio", json!(*rng.pick(&[0.5, 1.0, 2.0, 1.5])));
        }
        if r(rng) < 0.04 {
            let v = rng.pct(2, 10);
            set("gap", v);
        }
        if r(rng) < 0.05 {
            set("min-width", V::px(0.0));
        }
        if r(rng) < 0.08 {
            set("align-items", json!("baseline"));
        }
        if r(rng) < 0.04 {
            set("flex-basis", json!("auto"));
        }
    }
    let mut n = TreeNode { style: s, ..Default::default() };
    let kids = if depth >= 4 { 0 } else { *rng.pick(&[0, 1, 2, 2, 3, 4, 5]) };
    if kids == 0 {
        if rng.f() < 0.6 {
            let count = rng.int(1, 12);
            let ww = *rng.pick(&[20.0, 30.0, 45.0]);
            let lh = *rng.pick(&[16.0, 20.0]);
            let baseline = p.quirks && rng.f() < 0.5;
            n.text = Some(TextSpec { n: count, ww, lh, baseline });
        }
    } else {
        n.children = (0..kids).map(|_| node(depth + 1, rng, p)).collect();
    }
    n
}

pub fn case(seed: u64, p: Profile) -> Case {
    let mut rng = Rng::new(seed);
    let mut tree = node(0, &mut rng, p);
    tree.style.remove("width");
    tree.style.remove("height");
    let available_width = Some(*rng.pick(&[360.0, 430.0, 1024.0]));
    let available_height = Some(*rng.pick(&[640.0, 800.0]));
    Case {
        layout_inputs: LayoutInputs {
            available_width,
            available_height,
            owner_direction: if p.rtl { "rtl".into() } else { "ltr".into() },
        },
        tree,
        ..Default::default()
    }
}
