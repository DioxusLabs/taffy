//! JSON test case format (the Yoga capture format used by `gen.py`, plus `steps` for relayout scenarios).

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

pub type StyleMap = Map<String, Value>;

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Case {
    #[serde(rename = "layout-inputs")]
    pub layout_inputs: LayoutInputs,
    #[serde(default, skip_serializing_if = "ConfigSpec::is_default")]
    pub config: ConfigSpec,
    pub tree: TreeNode,
    /// Relayout scenario: each step mutates the tree and triggers another layout, which is compared too.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub steps: Vec<Step>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct LayoutInputs {
    #[serde(rename = "available-width")]
    pub available_width: Option<f32>,
    #[serde(rename = "available-height")]
    pub available_height: Option<f32>,
    #[serde(rename = "owner-direction", default = "ltr")]
    pub owner_direction: String,
}

fn ltr() -> String {
    "ltr".into()
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ConfigSpec {
    /// `all`, `none`, `classic`, `default`, or `|`-separated errata names (e.g. `stretch-flex-basis|min-size-undefined-instead-of-auto`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub errata: Option<String>,
    #[serde(rename = "experimental-features", default, skip_serializing_if = "Vec::is_empty")]
    pub experimental_features: Vec<String>,
    #[serde(rename = "point-scale-factor", default, skip_serializing_if = "Option::is_none")]
    pub point_scale_factor: Option<f32>,
    #[serde(rename = "use-web-defaults", default, skip_serializing_if = "std::ops::Not::not")]
    pub use_web_defaults: bool,
}

impl ConfigSpec {
    pub fn is_default(&self) -> bool {
        *self == ConfigSpec::default()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct TreeNode {
    #[serde(default, skip_serializing_if = "Map::is_empty")]
    pub style: StyleMap,
    /// Non-style node properties: `always-forms-containing-block`, `node-type`.
    #[serde(default, skip_serializing_if = "Map::is_empty")]
    pub node: StyleMap,
    #[serde(rename = "x-text", default, skip_serializing_if = "Option::is_none")]
    pub text: Option<TextSpec>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<TreeNode>,
}

impl TreeNode {
    pub fn get(&self, path: &[usize]) -> &TreeNode {
        path.iter().fold(self, |n, &i| &n.children[i])
    }

    pub fn get_mut(&mut self, path: &[usize]) -> &mut TreeNode {
        path.iter().fold(self, |n, &i| &mut n.children[i])
    }

    /// Pre-order list of (path, parent index).
    pub fn preorder(&self) -> Vec<(Vec<usize>, Option<usize>)> {
        fn walk(n: &TreeNode, path: Vec<usize>, parent: Option<usize>, out: &mut Vec<(Vec<usize>, Option<usize>)>) {
            let index = out.len();
            out.push((path.clone(), parent));
            for (i, c) in n.children.iter().enumerate() {
                let mut p = path.clone();
                p.push(i);
                walk(c, p, Some(index), out);
            }
        }
        let mut out = Vec::new();
        walk(self, Vec::new(), None, &mut out);
        out
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Default, PartialEq)]
pub struct TextSpec {
    pub n: i64,
    pub ww: f32,
    pub lh: f32,
    /// Also register a baseline function for this node.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub baseline: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum Step {
    /// Merge `style` into the style of the node at `path` (empty path = root).
    SetStyle {
        path: Vec<usize>,
        style: StyleMap,
    },
    MarkDirty {
        path: Vec<usize>,
    },
    LayoutInputs(LayoutInputs),
}

/// A parsed style value: `{"unit": "px"|"pct"|"auto", "value": f}`, `"auto"`, or absent.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum V {
    Undef,
    Px(f32),
    Pct(f32),
    Auto,
}

impl V {
    pub fn parse(v: &Value) -> V {
        if v.as_str() == Some("auto") {
            return V::Auto;
        }
        let value = || v["value"].as_f64().unwrap() as f32;
        match v.get("unit").and_then(|u| u.as_str()) {
            Some("px") | Some("point") => V::Px(value()),
            Some("pct") | Some("percent") => V::Pct(value()),
            Some("auto") => V::Auto,
            _ => V::Undef,
        }
    }

    pub fn px(x: f32) -> Value {
        serde_json::json!({ "unit": "px", "value": x })
    }

    pub fn pct(x: f32) -> Value {
        serde_json::json!({ "unit": "pct", "value": x })
    }
}

pub fn style_value(style: &StyleMap, key: &str) -> V {
    style.get(key).map(V::parse).unwrap_or(V::Undef)
}

pub fn style_str<'a>(style: &'a StyleMap, key: &str) -> Option<&'a str> {
    style.get(key).and_then(|v| v.as_str())
}

pub fn style_f32(style: &StyleMap, key: &str) -> Option<f32> {
    style.get(key).and_then(|v| v.as_f64()).map(|x| x as f32)
}

pub const EDGES: [&str; 9] = ["left", "top", "right", "bottom", "start", "end", "horizontal", "vertical", "all"];
