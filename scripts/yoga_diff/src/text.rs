//! The synthetic text model shared by the Yoga and Taffy sides: `n` words of width `ww`
//! laid out on lines of height `lh`, wrapping at the available width.

use crate::case::TextSpec;
use serde::{Deserialize, Serialize};

/// Yoga's `YGMeasureMode`, which is also what the Taffy side's `(known, available)` pair is mapped to.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Mode {
    Undefined,
    Exactly,
    AtMost,
}

#[derive(Clone, Copy, Debug)]
pub struct MeasureInput {
    pub w: f32,
    pub wm: Mode,
    pub h: f32,
    pub hm: Mode,
}

pub fn measure(t: &TextSpec, i: MeasureInput) -> (f32, f32) {
    let avail = if i.wm == Mode::Undefined { f32::INFINITY } else { i.w };
    let per_line = if avail.is_infinite() { i64::MAX } else { ((avail / t.ww + 1e-4).floor() as i64).max(1) };
    let lines = if t.n == 0 { 0 } else { (t.n - 1) / per_line + 1 };
    // CSS-like fit-content: min(max-content, max(min-content, available))
    let mut width = if t.n == 0 { 0.0 } else { (t.n as f32 * t.ww).min(t.ww.max(avail)) };
    if i.wm == Mode::Exactly {
        width = i.w;
    }
    let mut height = lines as f32 * t.lh;
    if i.hm == Mode::Exactly {
        height = i.h;
    }
    // Exact constraints can be negative (padding larger than the box); text can't be.
    (width.max(0.0), height.max(0.0))
}

/// First-line ascent.
pub fn baseline(t: &TextSpec, _w: f32, _h: f32) -> f32 {
    t.lh * 0.8
}
