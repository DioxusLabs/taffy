//! Builds the case as a `TaffyTree` and lays it out, logging measure calls in Yoga's terms.
//!
//! Until `Display::YogaFlexbox` exists this uses Taffy's CSS flexbox with Yoga's defaults, so differences
//! against Yoga are expected; the Yoga mode will be added as a second `TaffyMode`.

use crate::case::{style_f32, style_str, style_value, Case, LayoutInputs, Step, StyleMap, TextSpec, TreeNode, V};
use crate::config::Config;
use crate::output::{BaselineCall, CallLog, MeasureCall, NodeLayout, Run};
use crate::text::{self, MeasureInput, Mode};
use taffy::geometry::Point;
use taffy::prelude::*;
use taffy::style::{AlignContent, AlignItems, BoxSizing, Overflow, Position};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TaffyMode {
    /// Taffy's CSS flexbox algorithm with Yoga's defaults and RN's root handling emulated.
    Flex,
}

struct Ctx {
    path: String,
    text: TextSpec,
}

fn lpa(v: V) -> LengthPercentageAuto {
    match v {
        V::Px(x) => LengthPercentageAuto::length(x),
        V::Pct(x) => LengthPercentageAuto::percent(x / 100.0),
        _ => LengthPercentageAuto::auto(),
    }
}

fn lp(v: V) -> LengthPercentage {
    match v {
        V::Px(x) => LengthPercentage::length(x),
        V::Pct(x) => LengthPercentage::percent(x / 100.0),
        _ => LengthPercentage::length(0.0),
    }
}

fn dim(v: V) -> Dimension {
    match v {
        V::Px(x) => Dimension::length(x),
        V::Pct(x) => Dimension::percent(x / 100.0),
        _ => Dimension::auto(),
    }
}

/// Yoga's edge precedence: start/end > left/right > horizontal/vertical > all.
fn edges(style: &StyleMap, prefix: &str, rtl: bool) -> Rect<V> {
    let get = |e: &str| style_value(style, &format!("{prefix}-{e}"));
    let pick = |candidates: &[&str]| candidates.iter().map(|e| get(e)).find(|v| *v != V::Undef).unwrap_or(V::Undef);
    let (left_logical, right_logical) = if rtl { ("end", "start") } else { ("start", "end") };
    Rect {
        left: pick(&[left_logical, "left", "horizontal", "all"]),
        right: pick(&[right_logical, "right", "horizontal", "all"]),
        top: pick(&["top", "vertical", "all"]),
        bottom: pick(&["bottom", "vertical", "all"]),
    }
}

fn align_items(s: &str) -> Option<AlignItems> {
    Some(match s {
        "flex-start" => AlignItems::FLEX_START,
        "flex-end" => AlignItems::FLEX_END,
        "center" => AlignItems::CENTER,
        "stretch" => AlignItems::STRETCH,
        "baseline" => AlignItems::BASELINE,
        "start" => AlignItems::START,
        "end" => AlignItems::END,
        _ => return None,
    })
}

fn align_content(s: &str) -> Option<AlignContent> {
    Some(match s {
        "flex-start" => AlignContent::FLEX_START,
        "flex-end" => AlignContent::FLEX_END,
        "center" => AlignContent::CENTER,
        "stretch" => AlignContent::STRETCH,
        "space-between" => AlignContent::SPACE_BETWEEN,
        "space-around" => AlignContent::SPACE_AROUND,
        "space-evenly" => AlignContent::SPACE_EVENLY,
        "start" => AlignContent::START,
        "end" => AlignContent::END,
        _ => return None,
    })
}

/// Yoga style keys to a Taffy `Style`, with Yoga's defaults for everything unset. `no_auto_min` is Yoga's
/// `MinSizeUndefinedInsteadOfAuto` errata: an unset minimum is 0 rather than the CSS automatic minimum.
/// Non-style node properties that plain `Style` hosts express through style fields:
/// `always-forms-containing-block` on a static node becomes `position: relative` without insets
/// (identical for layout, and claims the containing block through `CoreStyle::is_containing_block`).
pub fn apply_node_props(style: &mut Style, node: &StyleMap) {
    if node.get("always-forms-containing-block").and_then(|v| v.as_bool()) == Some(true)
        && style.position == Position::Static
    {
        style.position = Position::Relative;
        style.inset = Rect::auto();
    }
}

pub fn to_taffy_style(s: &StyleMap, is_root: bool, rtl: bool, no_auto_min: bool) -> Style {
    let gs = |k: &str| style_str(s, k);
    let gf = |k: &str| style_f32(s, k);
    let gv = |k: &str| style_value(s, k);
    let mut st = Style {
        display: match gs("display") {
            Some("none") => Display::None,
            _ => Display::YogaFlexbox,
        },
        flex_direction: match gs("flex-direction") {
            Some("row") => FlexDirection::Row,
            Some("row-reverse") => FlexDirection::RowReverse,
            Some("column-reverse") => FlexDirection::ColumnReverse,
            _ => FlexDirection::Column,
        },
        flex_wrap: match gs("flex-wrap") {
            Some("wrap") => FlexWrap::Wrap,
            Some("wrap-reverse") => FlexWrap::WrapReverse,
            _ => FlexWrap::NoWrap,
        },
        justify_content: gs("justify-content").and_then(align_content).unwrap_or(AlignContent::FLEX_START),
        align_items: gs("align-items").and_then(align_items).unwrap_or(AlignItems::STRETCH),
        align_self: gs("align-self").and_then(align_items),
        align_content: gs("align-content").and_then(align_content).unwrap_or(AlignContent::FLEX_START),
        position: match gs("position-type") {
            Some("absolute") => Position::Absolute,
            Some("static") => Position::Static,
            _ => Position::Relative,
        },
        box_sizing: match gs("box-sizing") {
            Some("content-box") => BoxSizing::ContentBox,
            _ => BoxSizing::BorderBox,
        },
        aspect_ratio: gf("aspect-ratio"),
        scrollbar_width: 0.0,
        ..Default::default()
    };
    let overflow = match gs("overflow") {
        Some("hidden") => Overflow::Hidden,
        Some("scroll") => Overflow::Scroll,
        _ => Overflow::Visible,
    };
    st.overflow = Point { x: overflow, y: overflow };
    // Yoga's `flex` shorthand: positive = grow with basis 0, negative = shrink.
    let flex = gf("flex");
    st.flex_grow = gf("flex-grow").unwrap_or(match flex {
        Some(f) if f > 0.0 => f,
        _ => 0.0,
    });
    st.flex_shrink = gf("flex-shrink").unwrap_or(match flex {
        Some(f) if f < 0.0 => -f,
        _ => 0.0,
    });
    st.flex_basis = match gv("flex-basis") {
        V::Px(x) => Dimension::length(x),
        V::Pct(x) => Dimension::percent(x / 100.0),
        _ => match flex {
            Some(f) if f > 0.0 => Dimension::length(0.0),
            _ => Dimension::auto(),
        },
    };
    if is_root {
        st.flex_grow = 0.0;
        st.flex_shrink = 0.0;
    }
    st.size = Size { width: dim(gv("width")), height: dim(gv("height")) };
    // An unset minimum stays `auto`: the Yoga mode treats it as undefined under the
    // `MinSizeUndefinedInsteadOfAuto` errata, which differs from an explicit 0 when a box is smaller
    // than its own padding (`calculateAvailableInnerDimension` clamps to 0 only for undefined).
    let _ = no_auto_min;
    let min = |v: V| lpa(v);
    st.min_size = Size { width: min(gv("min-width")), height: min(gv("min-height")) };
    st.max_size = Size { width: lpa(gv("max-width")), height: lpa(gv("max-height")) };
    let zero_if_unset = |v: V| if v == V::Undef { LengthPercentageAuto::length(0.0) } else { lpa(v) };
    // Yoga's `direction: inherit` default: every node gets the owner direction
    st.direction = if rtl { taffy::style::Direction::Rtl } else { taffy::style::Direction::Ltr };
    st.margin = edges(s, "margin", rtl).map(zero_if_unset);
    st.padding = edges(s, "padding", rtl).map(lp);
    st.border = edges(s, "border", rtl).map(lp);
    st.inset = edges(s, "position", rtl).map(lpa);
    let gap = gv("gap");
    let pick = |v: V| if v == V::Undef { gap } else { v };
    st.gap = Size { width: lp(pick(gv("column-gap"))), height: lp(pick(gv("row-gap"))) };
    st
}

pub struct TaffySide {
    tree: TaffyTree<Ctx>,
    /// Pre-order node ids; `None` for `display: contents` nodes, which are flattened into their parent.
    nodes: Vec<Option<NodeId>>,
    paths: Vec<Vec<usize>>,
    root: NodeId,
    rtl: bool,
    no_auto_min: bool,
    #[allow(dead_code)]
    mode: TaffyMode,
}

impl TaffySide {
    pub fn new(case: &Case, config: Config, mode: TaffyMode) -> TaffySide {
        let mut tree: TaffyTree<Ctx> = TaffyTree::new();
        {
            use taffy::yoga_flexbox::{YogaConfig, YogaErrata, YogaExperimentalFeature, YogaExperimentalFeatures};
            let mut features = YogaExperimentalFeatures::default();
            features.set(YogaExperimentalFeature::FixFlexBasisFitContent, config.fix_flex_basis_fit_content);
            features.set(YogaExperimentalFeature::WebFlexBasis, config.web_flex_basis);
            tree.set_yoga_config(YogaConfig {
                use_web_defaults: config.use_web_defaults,
                experimental_features: features,
                errata: YogaErrata::from_bits(config.errata.0 as u32),
                point_scale_factor: config.point_scale_factor,
            });
        }
        if config.point_scale_factor == 0.0 {
            tree.disable_rounding();
        } else {
            tree.enable_rounding();
        }
        let rtl = case.layout_inputs.owner_direction == "rtl";
        let no_auto_min =
            config.errata & yoga::Errata::MinSizeUndefinedInsteadOfAuto == yoga::Errata::MinSizeUndefinedInsteadOfAuto;
        let mut side =
            TaffySide { tree, nodes: Vec::new(), paths: Vec::new(), root: NodeId::new(0), rtl, no_auto_min, mode };
        let root = side.build(&case.tree, Vec::new());
        side.root = root.expect("root cannot be display: contents");
        side
    }

    fn build(&mut self, j: &TreeNode, path: Vec<usize>) -> Option<NodeId> {
        let index = self.nodes.len();
        self.nodes.push(None);
        self.paths.push(path.clone());
        let mut children = Vec::new();
        for (i, child) in j.children.iter().enumerate() {
            let mut child_path = path.clone();
            child_path.push(i);
            self.build_into(child, child_path, &mut children);
        }
        if style_str(&j.style, "display") == Some("contents") {
            // Flattened by the host: its children were pushed into the parent's list by `build_into`.
            return None;
        }
        let mut style = to_taffy_style(&j.style, path.is_empty(), self.rtl, self.no_auto_min);
        apply_node_props(&mut style, &j.node);
        let id = match j.text {
            Some(text) => {
                let id = self
                    .tree
                    .new_leaf_with_context(style, Ctx { path: crate::yoga_side::path_string(&path), text })
                    .unwrap();
                for c in children {
                    self.tree.add_child(id, c).unwrap();
                }
                id
            }
            None => self.tree.new_with_children(style, &children).unwrap(),
        };
        self.nodes[index] = Some(id);
        Some(id)
    }

    fn build_into(&mut self, j: &TreeNode, path: Vec<usize>, out: &mut Vec<NodeId>) {
        if style_str(&j.style, "display") == Some("contents") {
            let index = self.nodes.len();
            self.nodes.push(None);
            self.paths.push(path.clone());
            for (i, child) in j.children.iter().enumerate() {
                let mut child_path = path.clone();
                child_path.push(i);
                self.build_into(child, child_path, out);
            }
            let _ = index;
        } else if let Some(id) = self.build(j, path) {
            out.push(id);
        }
    }

    fn index_of(&self, path: &[usize]) -> usize {
        self.paths.iter().position(|p| p == path).expect("no node at path")
    }

    pub fn apply_step(&mut self, step: &Step, tree: &TreeNode) {
        match step {
            Step::SetStyle { path, .. } => {
                if let Some(id) = self.nodes[self.index_of(path)] {
                    let node = tree.get(path);
                    let mut style = to_taffy_style(&node.style, path.is_empty(), self.rtl, self.no_auto_min);
                    apply_node_props(&mut style, &node.node);
                    self.tree.set_style(id, style).unwrap();
                }
            }
            Step::MarkDirty { path } => {
                if let Some(id) = self.nodes[self.index_of(path)] {
                    self.tree.mark_dirty(id).unwrap();
                }
            }
            Step::LayoutInputs(inputs) => self.rtl = inputs.owner_direction == "rtl",
        }
    }

    pub fn calculate_layout(&mut self, tree: &TreeNode, inputs: &LayoutInputs) -> Run {
        let aw = inputs.available_width;
        let ah = inputs.available_height;
        // Yoga root sizing: definite style size > max size (fit-content) > stretch to owner size (minus margins).
        let root_style = &tree.style;
        let mut style = to_taffy_style(root_style, true, self.rtl, self.no_auto_min);
        apply_node_props(&mut style, &tree.node);
        let mut available = Size { width: AvailableSpace::MaxContent, height: AvailableSpace::MaxContent };
        let margins = edges(root_style, "margin", self.rtl);
        let resolve = |v: V, basis: f32| match v {
            V::Px(x) => x,
            V::Pct(x) => x / 100.0 * basis,
            _ => 0.0,
        };
        if let Some(w) = aw {
            available.width = AvailableSpace::Definite(w);
            if style.size.width.is_auto() && style.max_size.width.is_auto() {
                style.size.width = Dimension::length(w - resolve(margins.left, w) - resolve(margins.right, w));
            }
        }
        if let Some(h) = ah {
            available.height = AvailableSpace::Definite(h);
            if style.size.height.is_auto() && style.max_size.height.is_auto() {
                let basis = aw.unwrap_or(0.0);
                style.size.height = Dimension::length(h - resolve(margins.top, basis) - resolve(margins.bottom, basis));
            }
        }
        if *self.tree.style(self.root).unwrap() != style {
            self.tree.set_style(self.root, style).unwrap();
        }

        let mut log = CallLog::default();
        self.tree
            .compute_layout_with_measure(self.root, available, |layout_input, _id, ctx, style| {
                // Yoga leaves ignore their own style sizes: the parent has already resolved them into
                // the sizing modes, so the measure callback must not go through
                // `taffy::compute_leaf_layout` (which would re-apply `style.size`). The callback
                // receives border-box known sizes and returns a border-box size.
                use taffy::ResolveOrZero;
                let pw = layout_input.parent_size.width;
                let pb = style.padding.resolve_or_zero(pw, |_, _| 0.0) + style.border.resolve_or_zero(pw, |_, _| 0.0);
                let pb_size = Size { width: pb.left + pb.right, height: pb.top + pb.bottom };
                match ctx {
                    Some(ctx) => {
                        // Yoga's measure callback receives content-box sizes clamped to zero.
                        let known = Size {
                            width: layout_input.known_dimensions.width.map(|k| (k - pb_size.width).max(0.0)),
                            height: layout_input.known_dimensions.height.map(|k| (k - pb_size.height).max(0.0)),
                        };
                        // `measureNodeWithMeasureFunc`: `innerWidth = max(0, availableWidth - paddingAndBorder)`
                        // for both the exact and the at-most modes.
                        let axis = |known: Option<f32>, avail: AvailableSpace, pb: f32| match (known, avail) {
                            (Some(k), _) => (k, Mode::Exactly),
                            (None, AvailableSpace::Definite(a)) => ((a - pb).max(0.0), Mode::AtMost),
                            (None, AvailableSpace::MinContent) => (0.0, Mode::AtMost),
                            (None, AvailableSpace::MaxContent) => (f32::NAN, Mode::Undefined),
                        };
                        let avail = layout_input.available_space;
                        let (w, wm) = axis(known.width, avail.width, pb_size.width);
                        let (h, hm) = axis(known.height, avail.height, pb_size.height);
                        if wm == Mode::Exactly && hm == Mode::Exactly {
                            // Yoga never measures with both axes exact; `TaffyTree` asks the measure
                            // function for the baseline of a final size this way (Yoga's `baselineFunc`).
                            let size = Size { width: w + pb_size.width, height: h + pb_size.height };
                            let mut output = taffy::LayoutOutput::from_outer_size(size);
                            if ctx.text.baseline {
                                let out = text::baseline(&ctx.text, size.width, size.height);
                                log.baseline.push(BaselineCall {
                                    path: ctx.path.clone(),
                                    w: size.width,
                                    h: size.height,
                                    out,
                                });
                                output.baselines.first = Some(out);
                            }
                            return output;
                        }
                        let (width, height) = text::measure(&ctx.text, MeasureInput { w, wm, h, hm });
                        log.measure.push(MeasureCall {
                            path: ctx.path.clone(),
                            w: (wm != Mode::Undefined).then_some(w),
                            wm,
                            h: (hm != Mode::Undefined).then_some(h),
                            hm,
                            out: [width, height],
                        });
                        taffy::LayoutOutput::from_outer_size(Size {
                            width: width + pb_size.width,
                            height: height + pb_size.height,
                        })
                    }
                    None => taffy::LayoutOutput::from_outer_size(pb_size),
                }
            })
            .unwrap();

        // Positions are compared in root coordinates: Taffy stores hoisted (absolute) boxes relative
        // to their containing block, Yoga relative to their parent.
        let mut containing_block: std::collections::HashMap<NodeId, usize> = std::collections::HashMap::new();
        for (index, id) in self.nodes.iter().enumerate() {
            if let Some(id) = id {
                for hoisted in self.tree.hoisted_children(*id).unwrap() {
                    containing_block.insert(*hoisted, index);
                }
            }
        }
        let parents: Vec<Option<usize>> = tree.preorder().into_iter().map(|(_, p)| p).collect();
        let mut origin = vec![[0.0f32; 2]; self.nodes.len()];
        let mut layout = Vec::with_capacity(self.nodes.len());
        for index in 0..self.nodes.len() {
            let mut ancestor = parents[index];
            while let Some(a) = ancestor {
                if self.nodes[a].is_some() {
                    break;
                }
                ancestor = parents[a];
            }
            match self.nodes[index] {
                Some(id) => {
                    let l = self.tree.layout(id).unwrap();
                    let base = match containing_block.get(&id).copied().or(ancestor) {
                        Some(base) => origin[base],
                        None => [0.0, 0.0],
                    };
                    origin[index] = [base[0] + l.location.x, base[1] + l.location.y];
                    layout.push(NodeLayout([origin[index][0], origin[index][1], l.size.width, l.size.height]));
                }
                None => {
                    origin[index] = ancestor.map(|a| origin[a]).unwrap_or([0.0, 0.0]);
                    layout.push(NodeLayout([0.0; 4]));
                }
            }
        }
        Run { layout, measure: log.measure, baseline: log.baseline }
    }
}
