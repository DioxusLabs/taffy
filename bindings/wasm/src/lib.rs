use wasm_bindgen::prelude::*;
use taffy::prelude::*;
use taffy::{TaffyTree, NodeId, Overflow, Point};
use taffy::GridTemplateComponent;

// ─── Core Layout Tree ────────────────────────────────────────────────────────

#[wasm_bindgen]
pub struct TaffyLayout {
    tree: TaffyTree<()>,
}

#[wasm_bindgen]
impl TaffyLayout {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            tree: TaffyTree::new(),
        }
    }

    /// Create a new leaf node with the given style. Returns the node ID.
    #[wasm_bindgen(js_name = "newLeaf")]
    pub fn new_leaf(&mut self, style: &JsValue) -> Result<usize, JsErr> {
        let s = parse_style(style)?;
        let node = self.tree.new_leaf(s).map_err(to_js)?;
        Ok(node.into())
    }

    /// Create a new node with children. Returns the node ID.
    #[wasm_bindgen(js_name = "newWithChildren")]
    pub fn new_with_children(&mut self, style: &JsValue, children: &[usize]) -> Result<usize, JsErr> {
        let s = parse_style(style)?;
        let child_ids: Vec<NodeId> = children.iter().map(|&id| NodeId::from(id)).collect();
        let node = self.tree.new_with_children(s, &child_ids).map_err(to_js)?;
        Ok(node.into())
    }

    /// Add a child to a parent node.
    #[wasm_bindgen(js_name = "addChild")]
    pub fn add_child(&mut self, parent: usize, child: usize) -> Result<(), JsErr> {
        self.tree.add_child(NodeId::from(parent), NodeId::from(child)).map_err(to_js)
    }

    /// Set the style of an existing node.
    #[wasm_bindgen(js_name = "setStyle")]
    pub fn set_style(&mut self, node: usize, style: &JsValue) -> Result<(), JsErr> {
        let s = parse_style(style)?;
        self.tree.set_style(NodeId::from(node), s).map_err(to_js)
    }

    /// Compute layout for the tree starting from the given root node.
    #[wasm_bindgen(js_name = "computeLayout")]
    pub fn compute_layout(&mut self, root: usize, available_width: f32, available_height: f32) -> Result<(), JsErr> {
        self.tree.compute_layout(
            NodeId::from(root),
            Size {
                width: AvailableSpace::Definite(available_width),
                height: AvailableSpace::Definite(available_height),
            },
        ).map_err(to_js)
    }

    /// Get the computed layout for a node. Returns {x, y, width, height}.
    #[wasm_bindgen(js_name = "getLayout")]
    pub fn get_layout(&self, node: usize) -> Result<JsValue, JsErr> {
        let layout = self.tree.layout(NodeId::from(node)).map_err(to_js)?;
        let obj = js_sys::Object::new();
        js_sys::Reflect::set(&obj, &"x".into(), &layout.location.x.into()).unwrap();
        js_sys::Reflect::set(&obj, &"y".into(), &layout.location.y.into()).unwrap();
        js_sys::Reflect::set(&obj, &"width".into(), &layout.size.width.into()).unwrap();
        js_sys::Reflect::set(&obj, &"height".into(), &layout.size.height.into()).unwrap();
        js_sys::Reflect::set(&obj, &"contentWidth".into(), &layout.content_size.width.into()).unwrap();
        js_sys::Reflect::set(&obj, &"contentHeight".into(), &layout.content_size.height.into()).unwrap();
        Ok(obj.into())
    }

    /// Get child count for a node.
    #[wasm_bindgen(js_name = "childCount")]
    pub fn child_count(&self, node: usize) -> usize {
        self.tree.child_count(NodeId::from(node))
    }

    /// Get children of a node as array of IDs.
    #[wasm_bindgen(js_name = "getChildren")]
    pub fn get_children(&self, node: usize) -> Result<Vec<usize>, JsErr> {
        let children = self.tree.children(NodeId::from(node)).map_err(to_js)?;
        Ok(children.iter().map(|id| (*id).into()).collect())
    }

    /// Remove a node from the tree.
    #[wasm_bindgen]
    pub fn remove(&mut self, node: usize) -> Result<usize, JsErr> {
        let removed = self.tree.remove(NodeId::from(node)).map_err(to_js)?;
        Ok(removed.into())
    }
}

// ─── Style Parsing ───────────────────────────────────────────────────────────

type JsErr = JsValue;

fn to_js<E: std::fmt::Display>(e: E) -> JsValue {
    JsValue::from_str(&e.to_string())
}

fn parse_style(js: &JsValue) -> Result<Style, JsValue> {
    let mut style = Style::DEFAULT;

    if js.is_undefined() || js.is_null() {
        return Ok(style);
    }

    // Display
    if let Some(v) = get_str(js, "display") {
        style.display = match v.as_str() {
            "flex" => Display::Flex,
            "block" => Display::Block,
            "grid" => Display::Grid,
            "none" => Display::None,
            _ => Display::Flex,
        };
    }

    // Position
    if let Some(v) = get_str(js, "position") {
        style.position = match v.as_str() {
            "absolute" => Position::Absolute,
            _ => Position::Relative,
        };
    }

    // Overflow
    if let Some(v) = get_str(js, "overflow") {
        let ov = match v.as_str() {
            "hidden" => Overflow::Hidden,
            "scroll" => Overflow::Scroll,
            "clip" => Overflow::Clip,
            _ => Overflow::Visible,
        };
        style.overflow = Point { x: ov, y: ov };
    }

    // Flex properties
    if let Some(v) = get_str(js, "flexDirection") {
        style.flex_direction = match v.as_str() {
            "row" => FlexDirection::Row,
            "row-reverse" => FlexDirection::RowReverse,
            "column-reverse" => FlexDirection::ColumnReverse,
            _ => FlexDirection::Column,
        };
    }
    if let Some(v) = get_str(js, "flexWrap") {
        style.flex_wrap = match v.as_str() {
            "wrap" => FlexWrap::Wrap,
            "wrap-reverse" => FlexWrap::WrapReverse,
            _ => FlexWrap::NoWrap,
        };
    }
    if let Some(v) = get_f32(js, "flexGrow") { style.flex_grow = v; }
    if let Some(v) = get_f32(js, "flexShrink") { style.flex_shrink = v; }
    if let Some(v) = get_dimension(js, "flexBasis") { style.flex_basis = v; }

    // Alignment
    if let Some(v) = get_str(js, "alignItems") { style.align_items = Some(parse_align_items(&v)); }
    if let Some(v) = get_str(js, "alignSelf") { style.align_self = Some(parse_align_items(&v)); }
    if let Some(v) = get_str(js, "alignContent") { style.align_content = Some(parse_align_content(&v)); }
    if let Some(v) = get_str(js, "justifyContent") { style.justify_content = Some(parse_align_content(&v)); }
    if let Some(v) = get_str(js, "justifyItems") { style.justify_items = Some(parse_align_items(&v)); }
    if let Some(v) = get_str(js, "justifySelf") { style.justify_self = Some(parse_align_items(&v)); }

    // Sizing
    if let Some(v) = get_dimension(js, "width") { style.size.width = v; }
    if let Some(v) = get_dimension(js, "height") { style.size.height = v; }
    if let Some(v) = get_dimension(js, "minWidth") { style.min_size.width = v; }
    if let Some(v) = get_dimension(js, "minHeight") { style.min_size.height = v; }
    if let Some(v) = get_dimension(js, "maxWidth") { style.max_size.width = v; }
    if let Some(v) = get_dimension(js, "maxHeight") { style.max_size.height = v; }

    // Aspect ratio
    if let Some(v) = get_f32(js, "aspectRatio") { style.aspect_ratio = Some(v); }

    // Margin (shorthand)
    if let Some(v) = get_f32(js, "margin") {
        let lpa = LengthPercentageAuto::length(v);
        style.margin = Rect { top: lpa, right: lpa, bottom: lpa, left: lpa };
    }
    if let Some(v) = get_length_percentage_auto(js, "marginTop") { style.margin.top = v; }
    if let Some(v) = get_length_percentage_auto(js, "marginRight") { style.margin.right = v; }
    if let Some(v) = get_length_percentage_auto(js, "marginBottom") { style.margin.bottom = v; }
    if let Some(v) = get_length_percentage_auto(js, "marginLeft") { style.margin.left = v; }

    // Padding (shorthand)
    if let Some(v) = get_f32(js, "padding") {
        let lp = LengthPercentage::length(v);
        style.padding = Rect { top: lp, right: lp, bottom: lp, left: lp };
    }
    if let Some(v) = get_length_percentage(js, "paddingTop") { style.padding.top = v; }
    if let Some(v) = get_length_percentage(js, "paddingRight") { style.padding.right = v; }
    if let Some(v) = get_length_percentage(js, "paddingBottom") { style.padding.bottom = v; }
    if let Some(v) = get_length_percentage(js, "paddingLeft") { style.padding.left = v; }

    // Border (shorthand)
    if let Some(v) = get_f32(js, "border") {
        let lp = LengthPercentage::length(v);
        style.border = Rect { top: lp, right: lp, bottom: lp, left: lp };
    }
    if let Some(v) = get_length_percentage(js, "borderTop") { style.border.top = v; }
    if let Some(v) = get_length_percentage(js, "borderRight") { style.border.right = v; }
    if let Some(v) = get_length_percentage(js, "borderBottom") { style.border.bottom = v; }
    if let Some(v) = get_length_percentage(js, "borderLeft") { style.border.left = v; }

    // Insets
    if let Some(v) = get_length_percentage_auto(js, "top") { style.inset.top = v; }
    if let Some(v) = get_length_percentage_auto(js, "right") { style.inset.right = v; }
    if let Some(v) = get_length_percentage_auto(js, "bottom") { style.inset.bottom = v; }
    if let Some(v) = get_length_percentage_auto(js, "left") { style.inset.left = v; }

    // Gap
    if let Some(v) = get_f32(js, "gap") {
        let lp = LengthPercentage::length(v);
        style.gap = Size { width: lp, height: lp };
    }
    if let Some(v) = get_length_percentage(js, "rowGap") { style.gap.height = v; }
    if let Some(v) = get_length_percentage(js, "columnGap") { style.gap.width = v; }

    // Grid template
    if let Some(cols) = get_track_list(js, "gridTemplateColumns") {
        style.grid_template_columns = cols.into_iter().map(GridTemplateComponent::Single).collect();
    }
    if let Some(rows) = get_track_list(js, "gridTemplateRows") {
        style.grid_template_rows = rows.into_iter().map(GridTemplateComponent::Single).collect();
    }

    // Grid placement
    if let Some(v) = get_grid_line(js, "gridColumnStart") { style.grid_column = Line { start: v, end: style.grid_column.end }; }
    if let Some(v) = get_grid_line(js, "gridColumnEnd") { style.grid_column = Line { start: style.grid_column.start, end: v }; }
    if let Some(v) = get_grid_line(js, "gridRowStart") { style.grid_row = Line { start: v, end: style.grid_row.end }; }
    if let Some(v) = get_grid_line(js, "gridRowEnd") { style.grid_row = Line { start: style.grid_row.start, end: v }; }

    Ok(style)
}

// ─── JS Value Helpers ────────────────────────────────────────────────────────

fn get_str(obj: &JsValue, key: &str) -> Option<String> {
    js_sys::Reflect::get(obj, &key.into()).ok()?.as_string()
}

fn get_f32(obj: &JsValue, key: &str) -> Option<f32> {
    js_sys::Reflect::get(obj, &key.into()).ok()?.as_f64().map(|v| v as f32)
}

fn get_dimension(obj: &JsValue, key: &str) -> Option<Dimension> {
    let val = js_sys::Reflect::get(obj, &key.into()).ok()?;
    if val.is_undefined() || val.is_null() { return None; }
    if let Some(n) = val.as_f64() { return Some(Dimension::length(n as f32)); }
    if let Some(s) = val.as_string() {
        if s == "auto" { return Some(Dimension::auto()); }
        if let Some(pct) = s.strip_suffix('%') {
            if let Ok(n) = pct.trim().parse::<f32>() {
                return Some(Dimension::percent(n / 100.0));
            }
        }
        if let Some(px) = s.strip_suffix("px") {
            if let Ok(n) = px.trim().parse::<f32>() {
                return Some(Dimension::length(n));
            }
        }
        if let Ok(n) = s.parse::<f32>() {
            return Some(Dimension::length(n));
        }
    }
    None
}

fn get_length_percentage(obj: &JsValue, key: &str) -> Option<LengthPercentage> {
    let val = js_sys::Reflect::get(obj, &key.into()).ok()?;
    if val.is_undefined() || val.is_null() { return None; }
    if let Some(n) = val.as_f64() { return Some(LengthPercentage::length(n as f32)); }
    if let Some(s) = val.as_string() {
        if let Some(pct) = s.strip_suffix('%') {
            if let Ok(n) = pct.trim().parse::<f32>() {
                return Some(LengthPercentage::percent(n / 100.0));
            }
        }
        if let Ok(n) = s.parse::<f32>() {
            return Some(LengthPercentage::length(n));
        }
    }
    None
}

fn get_length_percentage_auto(obj: &JsValue, key: &str) -> Option<LengthPercentageAuto> {
    let val = js_sys::Reflect::get(obj, &key.into()).ok()?;
    if val.is_undefined() || val.is_null() { return None; }
    if let Some(n) = val.as_f64() { return Some(LengthPercentageAuto::length(n as f32)); }
    if let Some(s) = val.as_string() {
        if s == "auto" { return Some(LengthPercentageAuto::auto()); }
        if let Some(pct) = s.strip_suffix('%') {
            if let Ok(n) = pct.trim().parse::<f32>() {
                return Some(LengthPercentageAuto::percent(n / 100.0));
            }
        }
        if let Ok(n) = s.parse::<f32>() {
            return Some(LengthPercentageAuto::length(n));
        }
    }
    None
}

fn parse_align_items(s: &str) -> AlignItems {
    match s {
        "flex-start" | "start" => AlignItems::FlexStart,
        "flex-end" | "end" => AlignItems::FlexEnd,
        "center" => AlignItems::Center,
        "baseline" => AlignItems::Baseline,
        "stretch" => AlignItems::Stretch,
        _ => AlignItems::Stretch,
    }
}

fn parse_align_content(s: &str) -> AlignContent {
    match s {
        "flex-start" | "start" => AlignContent::FlexStart,
        "flex-end" | "end" => AlignContent::FlexEnd,
        "center" => AlignContent::Center,
        "stretch" => AlignContent::Stretch,
        "space-between" => AlignContent::SpaceBetween,
        "space-around" => AlignContent::SpaceAround,
        "space-evenly" => AlignContent::SpaceEvenly,
        _ => AlignContent::Stretch,
    }
}

fn get_track_list(obj: &JsValue, key: &str) -> Option<Vec<TrackSizingFunction>> {
    let val = js_sys::Reflect::get(obj, &key.into()).ok()?;
    if val.is_undefined() || val.is_null() { return None; }

    let arr = js_sys::Array::from(&val);
    let mut tracks = Vec::new();

    for i in 0..arr.length() {
        let item = arr.get(i);
        if let Some(n) = item.as_f64() {
            tracks.push(length(n as f32));
        } else if let Some(s) = item.as_string() {
            if s == "auto" {
                tracks.push(auto());
            } else if let Some(fr_val) = s.strip_suffix("fr") {
                if let Ok(n) = fr_val.trim().parse::<f32>() {
                    tracks.push(fr(n));
                }
            } else if let Some(pct) = s.strip_suffix('%') {
                if let Ok(n) = pct.trim().parse::<f32>() {
                    tracks.push(percent(n / 100.0));
                }
            } else if let Ok(n) = s.parse::<f32>() {
                tracks.push(length(n));
            }
        }
    }

    Some(tracks)
}

fn get_grid_line(obj: &JsValue, key: &str) -> Option<GridPlacement> {
    let val = js_sys::Reflect::get(obj, &key.into()).ok()?;
    if val.is_undefined() || val.is_null() { return None; }
    if let Some(n) = val.as_f64() {
        return Some(GridPlacement::from_line_index(n as i16));
    }
    if let Some(s) = val.as_string() {
        if s == "auto" { return Some(GridPlacement::Auto); }
        if let Some(span_val) = s.strip_prefix("span ") {
            if let Ok(n) = span_val.trim().parse::<u16>() {
                return Some(GridPlacement::from_span(n));
            }
        }
        if let Ok(n) = s.parse::<i16>() {
            return Some(GridPlacement::from_line_index(n));
        }
    }
    None
}
