//! Layout output and call logs, compared bit-for-bit.

use crate::text::Mode;
use serde::{Deserialize, Serialize};

/// `[x, y, width, height]` of one node, in document (pre-order) order.
#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct NodeLayout(pub [f32; 4]);

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MeasureCall {
    pub path: String,
    pub w: Option<f32>,
    pub wm: Mode,
    pub h: Option<f32>,
    pub hm: Mode,
    pub out: [f32; 2],
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct BaselineCall {
    pub path: String,
    pub w: f32,
    pub h: f32,
    pub out: f32,
}

/// Everything observed during one layout pass.
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Run {
    pub layout: Vec<NodeLayout>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub measure: Vec<MeasureCall>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub baseline: Vec<BaselineCall>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct CallLog {
    pub measure: Vec<MeasureCall>,
    pub baseline: Vec<BaselineCall>,
}

/// Bit-for-bit when `tolerance` is 0 (NaN equals NaN), otherwise within `tolerance`.
pub fn same_f32(a: f32, b: f32, tolerance: f32) -> bool {
    if tolerance == 0.0 {
        a.to_bits() == b.to_bits() || (a.is_nan() && b.is_nan())
    } else {
        (a - b).abs() <= tolerance || a == b || (a.is_nan() && b.is_nan())
    }
}

pub fn same_layout(a: &NodeLayout, b: &NodeLayout, tolerance: f32) -> bool {
    a.0.iter().zip(b.0.iter()).all(|(x, y)| same_f32(*x, *y, tolerance))
}

/// Node indices (pre-order) whose layout differs.
pub fn differing_nodes(a: &Run, b: &Run, tolerance: f32) -> Vec<usize> {
    (0..a.layout.len().max(b.layout.len()))
        .filter(|&i| match (a.layout.get(i), b.layout.get(i)) {
            (Some(x), Some(y)) => !same_layout(x, y, tolerance),
            _ => true,
        })
        .collect()
}

pub fn same_calls(a: &Run, b: &Run) -> bool {
    a.measure == b.measure && a.baseline == b.baseline
}
