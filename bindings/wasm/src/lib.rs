#![deny(unsafe_code)]
#![forbid(unsafe_code)]
// #![warn(missing_docs)]
// #![warn(clippy::missing_docs_in_private_items)]
#![allow(non_snake_case)] // JS uses camelCase by default
#![allow(clippy::new_without_default)] // Default is useless for WASM

#[allow(dead_code)]
pub fn set_panic_hook() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

use std::cell::RefCell;
use std::rc::Rc;

use js_sys::Array;
use js_sys::Function;
use js_sys::Reflect;
use taffy::prelude::*;
use taffy::TraversePartialTree;
use wasm_bindgen::prelude::*;

/// Get the value of a property named "key" from the JsValue "obj"
fn get_key(obj: &JsValue, key: &str) -> Option<JsValue> {
    Reflect::get(obj, &key.into()).ok()
}

/// Get a property named "key" from the JsValue "obj" assuming that it is a number and casting it to f32
fn get_f32(obj: &JsValue, key: &str) -> Option<f32> {
    get_key(obj, key).and_then(|val| val.as_f64().map(|v| v as f32))
}

/// Convert a JS number or string to an AvailableSpace.
///   - Numbers will be converted to AvailableSpace::Definite
///   - Strings are expected to be "min-context", "max-content" or a numeric value followed by "px"
fn try_parse_available_space(obj: &JsValue, key: &str) -> Option<AvailableSpace> {
    if let Some(val) = get_key(obj, key) {
        if let Some(number) = val.as_f64() {
            return Some(AvailableSpace::Definite(number as f32));
        }
        if let Some(string) = val.as_string() {
            return string.parse().ok();
        }
    }
    None
}

#[wasm_bindgen]
#[repr(u8)]
pub enum StyleUnit {
    Px,
    Percent,
    Auto,
    MinContent,
    MaxContent,
    FitContentPx,
    FitContentPercent,
    Fr,
}

impl StyleUnit {
    fn try_into_dimension(self, val: f32) -> Result<Dimension, ()> {
        match self {
            StyleUnit::Px => Ok(Dimension::Length(val)),
            StyleUnit::Percent => Ok(Dimension::Percent(val)),
            StyleUnit::Auto => Ok(Dimension::Auto),
            _ => Err(()),
        }
    }

    fn try_into_length_percentage_auto(self, val: f32) -> Result<LengthPercentageAuto, ()> {
        match self {
            StyleUnit::Px => Ok(LengthPercentageAuto::Length(val)),
            StyleUnit::Percent => Ok(LengthPercentageAuto::Percent(val)),
            StyleUnit::Auto => Ok(LengthPercentageAuto::Auto),
            _ => Err(()),
        }
    }

    fn try_into_length_percentage(self, val: f32) -> Result<LengthPercentage, ()> {
        match self {
            StyleUnit::Px => Ok(LengthPercentage::Length(val)),
            StyleUnit::Percent => Ok(LengthPercentage::Percent(val)),
            _ => Err(()),
        }
    }
}

impl TryFrom<u8> for StyleUnit {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(StyleUnit::Px),
            1 => Ok(StyleUnit::Percent),
            2 => Ok(StyleUnit::Auto),
            3 => Ok(StyleUnit::MinContent),
            4 => Ok(StyleUnit::MaxContent),
            5 => Ok(StyleUnit::FitContentPx),
            6 => Ok(StyleUnit::FitContentPercent),
            7 => Ok(StyleUnit::Fr),
            _ => Err(()),
        }
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Layout {
    #[wasm_bindgen(readonly)]
    pub width: f32,

    #[wasm_bindgen(readonly)]
    pub height: f32,

    #[wasm_bindgen(readonly)]
    pub x: f32,

    #[wasm_bindgen(readonly)]
    pub y: f32,

    #[wasm_bindgen(readonly)]
    pub childCount: usize,

    children: Vec<Layout>,
}

#[wasm_bindgen]
impl Layout {
    fn new(tree: &TaffyTree, node: taffy::NodeId) -> Layout {
        let taffy = tree.taffy.borrow();
        let layout = taffy.layout(node).unwrap();
        let children = taffy.children(node).unwrap();

        Layout {
            width: layout.size.width,
            height: layout.size.height,
            x: layout.location.x,
            y: layout.location.y,
            childCount: children.len(),
            children: children.into_iter().map(|child| Layout::new(tree, child)).collect(),
        }
    }

    #[wasm_bindgen]
    pub fn child(&self, at: usize) -> Layout {
        self.children[at].clone()
    }
}

struct WasmNodeContext {
    measure_func: Function,
}

impl WasmNodeContext {
    fn from_js_measure(measure_func: Function) -> Self {
        Self { measure_func }
    }
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct TaffyTree {
    taffy: Rc<RefCell<taffy::TaffyTree<WasmNodeContext>>>,
}

#[wasm_bindgen]
impl TaffyTree {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self { taffy: Rc::new(RefCell::new(taffy::TaffyTree::new())) }
    }
}

#[wasm_bindgen]
pub struct Node {
    tree: TaffyTree,
    node: taffy::NodeId,

    #[wasm_bindgen(readonly)]
    pub childCount: usize,
}

fn wasm_measure_function(
    known_dimensions: Size<Option<f32>>,
    available_space: Size<AvailableSpace>,
    _node_id: NodeId,
    context: Option<&mut WasmNodeContext>,
) -> Size<f32> {
    fn convert_available_space(val: AvailableSpace) -> JsValue {
        match val {
            AvailableSpace::Definite(val) => val.into(),
            AvailableSpace::MaxContent => JsValue::from_str("max-content"),
            AvailableSpace::MinContent => JsValue::from_str("min-content"),
        }
    }

    let Some(context) = context else { return Size::ZERO };

    let known_width = known_dimensions.width.map(|val| val.into()).unwrap_or(JsValue::UNDEFINED);
    let known_height = known_dimensions.height.map(|val| val.into()).unwrap_or(JsValue::UNDEFINED);

    let available_width = convert_available_space(available_space.width);
    let available_height = convert_available_space(available_space.height);

    let args = Array::new_with_length(4);
    args.set(0, known_width);
    args.set(1, known_height);
    args.set(2, available_width);
    args.set(3, available_height);

    if let Ok(result) = context.measure_func.apply(&JsValue::UNDEFINED, &args) {
        let width = get_f32(&result, "width");
        let height = get_f32(&result, "height");

        if let (Some(width), Some(height)) = (width, height) {
            return Size { width, height };
        }
    }

    known_dimensions.unwrap_or(Size::ZERO)
}

#[wasm_bindgen]
impl Node {
    #[wasm_bindgen(constructor)]
    pub fn new(tree: &TaffyTree) -> Self {
        Self { tree: tree.clone(), node: tree.taffy.borrow_mut().new_leaf(Style::DEFAULT).unwrap(), childCount: 0 }
    }

    #[wasm_bindgen(js_name = setMeasure)]
    pub fn set_measure(&mut self, measure: &JsValue) {
        let js_measure_func = Function::from(measure.clone());
        self.tree
            .taffy
            .borrow_mut()
            .set_node_context(self.node, Some(WasmNodeContext::from_js_measure(js_measure_func)))
            .unwrap();
    }

    #[wasm_bindgen(js_name = addChild)]
    pub fn add_child(&mut self, child: &Node) {
        self.tree.taffy.borrow_mut().add_child(self.node, child.node).unwrap();
        self.childCount += 1;
    }

    #[wasm_bindgen(js_name = removeChild)]
    pub fn remove_child(&mut self, child: &Node) {
        self.tree.taffy.borrow_mut().remove_child(self.node, child.node).unwrap();
        self.childCount -= 1;
    }

    #[wasm_bindgen(js_name = replaceChildAtIndex)]
    pub fn replace_child_at_index(&mut self, index: usize, child: &Node) {
        self.tree.taffy.borrow_mut().replace_child_at_index(self.node, index, child.node).unwrap();
    }

    #[wasm_bindgen(js_name = removeChildAtIndex)]
    pub fn remove_child_at_index(&mut self, index: usize) {
        self.tree.taffy.borrow_mut().remove_child_at_index(self.node, index).unwrap();
        self.childCount -= 1;
    }

    #[wasm_bindgen(js_name = markDirty)]
    pub fn mark_dirty(&mut self) {
        self.tree.taffy.borrow_mut().mark_dirty(self.node).unwrap()
    }

    #[wasm_bindgen(js_name = isDirty)]
    pub fn is_dirty(&self) -> bool {
        self.tree.taffy.borrow().dirty(self.node).unwrap()
    }

    #[wasm_bindgen(js_name = childCount)]
    pub fn child_count(&mut self) -> usize {
        self.tree.taffy.borrow_mut().child_count(self.node)
    }

    #[wasm_bindgen(js_name = computeLayout)]
    pub fn compute_layout(&mut self, size: &JsValue) -> Layout {
        self.tree
            .taffy
            .borrow_mut()
            .compute_layout_with_measure(
                self.node,
                taffy::geometry::Size {
                    width: try_parse_available_space(size, "width").unwrap_or(AvailableSpace::MaxContent),
                    height: try_parse_available_space(size, "height").unwrap_or(AvailableSpace::MaxContent),
                },
                wasm_measure_function,
            )
            .unwrap();
        Layout::new(&self.tree, self.node)
    }
}

macro_rules! get_style {
    ($self:expr, $style_ident:ident, $block:expr) => {{
        let taffy = $self.tree.taffy.borrow();
        let $style_ident = taffy.style($self.node)?;
        Ok($block)
    }};
}

macro_rules! with_style_mut {
    ($self:expr, $style_ident:ident, $block:expr) => {{
        let mut taffy = $self.tree.taffy.borrow_mut();
        let $style_ident = taffy.style_mut($self.node)?;
        $block;
        Ok(())
    }};
}

// Style getter/setter methods
#[wasm_bindgen]
impl Node {
    // Display / Position
    pub fn getDisplay(&mut self) -> Result<Display, JsError> {
        get_style!(self, style, style.display)
    }
    pub fn setDisplay(&mut self, value: Display) -> Result<(), JsError> {
        with_style_mut!(self, style, style.display = value)
    }
    pub fn getPosition(&mut self) -> Result<Position, JsError> {
        get_style!(self, style, style.position)
    }
    pub fn setPosition(&mut self, value: Position) -> Result<(), JsError> {
        with_style_mut!(self, style, style.position = value)
    }

    // Overflow
    pub fn getOverflowX(&mut self) -> Result<Overflow, JsError> {
        get_style!(self, style, style.overflow.x)
    }
    pub fn setOverflowX(&mut self, value: Overflow) -> Result<(), JsError> {
        with_style_mut!(self, style, style.overflow.x = value)
    }
    pub fn getOverflowY(&mut self) -> Result<Overflow, JsError> {
        get_style!(self, style, style.overflow.y)
    }
    pub fn setOverflowY(&mut self, value: Overflow) -> Result<(), JsError> {
        with_style_mut!(self, style, style.overflow.y = value)
    }
    pub fn setOverflow(&mut self, value: Overflow) -> Result<(), JsError> {
        with_style_mut!(self, style, {
            style.overflow.x = value;
            style.overflow.y = value;
        })
    }

    // inset
    pub fn setInsetTop(&mut self, value: f32, unit: StyleUnit) -> Result<(), JsError> {
        with_style_mut!(self, style, style.inset.top = unit.try_into_length_percentage_auto(value).unwrap())
    }
    pub fn setInsetBottom(&mut self, value: f32, unit: StyleUnit) -> Result<(), JsError> {
        with_style_mut!(self, style, style.inset.bottom = unit.try_into_length_percentage_auto(value).unwrap())
    }
    pub fn setInsetLeft(&mut self, value: f32, unit: StyleUnit) -> Result<(), JsError> {
        with_style_mut!(self, style, style.inset.left = unit.try_into_length_percentage_auto(value).unwrap())
    }
    pub fn setInsetRight(&mut self, value: f32, unit: StyleUnit) -> Result<(), JsError> {
        with_style_mut!(self, style, style.inset.right = unit.try_into_length_percentage_auto(value).unwrap())
    }
    pub fn setInsetHorizontal(&mut self, value: f32, unit: StyleUnit) -> Result<(), JsError> {
        with_style_mut!(self, style, {
            let value = unit.try_into_length_percentage_auto(value).unwrap();
            style.inset.left = value;
            style.inset.right = value;
        })
    }
    pub fn setInsetVertical(&mut self, value: f32, unit: StyleUnit) -> Result<(), JsError> {
        with_style_mut!(self, style, {
            let value = unit.try_into_length_percentage_auto(value).unwrap();
            style.inset.left = value;
            style.inset.right = value;
        })
    }
    pub fn setInsetAll(&mut self, value: f32, unit: StyleUnit) -> Result<(), JsError> {
        with_style_mut!(self, style, {
            let value = unit.try_into_length_percentage_auto(value).unwrap();
            style.inset.top = value;
            style.inset.bottom = value;
            style.inset.left = value;
            style.inset.right = value;
        })
    }

    // Sizes
    pub fn setWidth(&mut self, value: f32, unit: StyleUnit) -> Result<(), JsError> {
        with_style_mut!(self, style, style.size.width = unit.try_into_dimension(value).unwrap())
    }
    pub fn setHeight(&mut self, value: f32, unit: StyleUnit) -> Result<(), JsError> {
        with_style_mut!(self, style, style.size.height = unit.try_into_dimension(value).unwrap())
    }
    pub fn setMinWidth(&mut self, value: f32, unit: StyleUnit) -> Result<(), JsError> {
        with_style_mut!(self, style, style.min_size.width = unit.try_into_dimension(value).unwrap())
    }
    pub fn setMinHeight(&mut self, value: f32, unit: StyleUnit) -> Result<(), JsError> {
        with_style_mut!(self, style, style.min_size.height = unit.try_into_dimension(value).unwrap())
    }
    pub fn setMaxWidth(&mut self, value: f32, unit: StyleUnit) -> Result<(), JsError> {
        with_style_mut!(self, style, style.max_size.width = unit.try_into_dimension(value).unwrap())
    }
    pub fn setMaxHeight(&mut self, value: f32, unit: StyleUnit) -> Result<(), JsError> {
        with_style_mut!(self, style, style.max_size.height = unit.try_into_dimension(value).unwrap())
    }
    pub fn setAspectRatio(&mut self, value: Option<f32>) -> Result<(), JsError> {
        with_style_mut!(self, style, style.aspect_ratio = value)
    }

    // Padding
    pub fn setPaddingTop(&mut self, value: f32, unit: StyleUnit) -> Result<(), JsError> {
        with_style_mut!(self, style, style.padding.top = unit.try_into_length_percentage(value).unwrap())
    }
    pub fn setPaddingBottom(&mut self, value: f32, unit: StyleUnit) -> Result<(), JsError> {
        with_style_mut!(self, style, style.padding.bottom = unit.try_into_length_percentage(value).unwrap())
    }
    pub fn setPaddingLeft(&mut self, value: f32, unit: StyleUnit) -> Result<(), JsError> {
        with_style_mut!(self, style, style.padding.left = unit.try_into_length_percentage(value).unwrap())
    }
    pub fn setPaddingRight(&mut self, value: f32, unit: StyleUnit) -> Result<(), JsError> {
        with_style_mut!(self, style, style.padding.right = unit.try_into_length_percentage(value).unwrap())
    }
    pub fn setPaddingHorizontal(&mut self, value: f32, unit: StyleUnit) -> Result<(), JsError> {
        with_style_mut!(self, style, {
            let value = unit.try_into_length_percentage(value).unwrap();
            style.padding.left = value;
            style.padding.right = value;
        })
    }
    pub fn setPaddingVertical(&mut self, value: f32, unit: StyleUnit) -> Result<(), JsError> {
        with_style_mut!(self, style, {
            let value = unit.try_into_length_percentage(value).unwrap();
            style.padding.left = value;
            style.padding.right = value;
        })
    }
    pub fn setPaddingAll(&mut self, value: f32, unit: StyleUnit) -> Result<(), JsError> {
        with_style_mut!(self, style, {
            let value = unit.try_into_length_percentage(value).unwrap();
            style.padding.top = value;
            style.padding.bottom = value;
            style.padding.left = value;
            style.padding.right = value;
        })
    }

    // Margin
    pub fn setMarginTop(&mut self, value: f32, unit: StyleUnit) -> Result<(), JsError> {
        with_style_mut!(self, style, style.margin.top = unit.try_into_length_percentage_auto(value).unwrap())
    }
    pub fn setMarginBottom(&mut self, value: f32, unit: StyleUnit) -> Result<(), JsError> {
        with_style_mut!(self, style, style.margin.bottom = unit.try_into_length_percentage_auto(value).unwrap())
    }
    pub fn setMarginLeft(&mut self, value: f32, unit: StyleUnit) -> Result<(), JsError> {
        with_style_mut!(self, style, style.margin.left = unit.try_into_length_percentage_auto(value).unwrap())
    }
    pub fn setMarginRight(&mut self, value: f32, unit: StyleUnit) -> Result<(), JsError> {
        with_style_mut!(self, style, style.margin.right = unit.try_into_length_percentage_auto(value).unwrap())
    }
    pub fn setMarginHorizontal(&mut self, value: f32, unit: StyleUnit) -> Result<(), JsError> {
        with_style_mut!(self, style, {
            let value = unit.try_into_length_percentage_auto(value).unwrap();
            style.margin.left = value;
            style.margin.right = value;
        })
    }
    pub fn setMarginVertical(&mut self, value: f32, unit: StyleUnit) -> Result<(), JsError> {
        with_style_mut!(self, style, {
            let value = unit.try_into_length_percentage_auto(value).unwrap();
            style.margin.left = value;
            style.margin.right = value;
        })
    }
    pub fn setMarginAll(&mut self, value: f32, unit: StyleUnit) -> Result<(), JsError> {
        with_style_mut!(self, style, {
            let value = unit.try_into_length_percentage_auto(value).unwrap();
            style.margin.top = value;
            style.margin.bottom = value;
            style.margin.left = value;
            style.margin.right = value;
        })
    }

    // Border
    pub fn setBorderWidthTop(&mut self, value: f32, unit: StyleUnit) -> Result<(), JsError> {
        with_style_mut!(self, style, style.border.top = unit.try_into_length_percentage(value).unwrap())
    }
    pub fn setBorderWidthBottom(&mut self, value: f32, unit: StyleUnit) -> Result<(), JsError> {
        with_style_mut!(self, style, style.border.bottom = unit.try_into_length_percentage(value).unwrap())
    }
    pub fn setBorderWidthLeft(&mut self, value: f32, unit: StyleUnit) -> Result<(), JsError> {
        with_style_mut!(self, style, style.border.left = unit.try_into_length_percentage(value).unwrap())
    }
    pub fn setBorderWidthRight(&mut self, value: f32, unit: StyleUnit) -> Result<(), JsError> {
        with_style_mut!(self, style, style.border.right = unit.try_into_length_percentage(value).unwrap())
    }
    pub fn setBorderWidthHorizontal(&mut self, value: f32, unit: StyleUnit) -> Result<(), JsError> {
        with_style_mut!(self, style, {
            let value = unit.try_into_length_percentage(value).unwrap();
            style.border.left = value;
            style.border.right = value;
        })
    }
    pub fn setBorderWidthVertical(&mut self, value: f32, unit: StyleUnit) -> Result<(), JsError> {
        with_style_mut!(self, style, {
            let value = unit.try_into_length_percentage(value).unwrap();
            style.border.left = value;
            style.border.right = value;
        })
    }
    pub fn setBorderWidthAll(&mut self, value: f32, unit: StyleUnit) -> Result<(), JsError> {
        with_style_mut!(self, style, {
            let value = unit.try_into_length_percentage(value).unwrap();
            style.border.top = value;
            style.border.bottom = value;
            style.border.left = value;
            style.border.right = value;
        })
    }

    // Gap
    pub fn setRowGap(&mut self, value: f32, unit: StyleUnit) -> Result<(), JsError> {
        with_style_mut!(self, style, style.gap.width = unit.try_into_length_percentage(value).unwrap())
    }
    pub fn setColumnGap(&mut self, value: f32, unit: StyleUnit) -> Result<(), JsError> {
        with_style_mut!(self, style, style.gap.height = unit.try_into_length_percentage(value).unwrap())
    }
    pub fn setGap(&mut self, value: f32, unit: StyleUnit) -> Result<(), JsError> {
        with_style_mut!(self, style, {
            let value = unit.try_into_length_percentage(value).unwrap();
            style.gap.width = value;
            style.gap.height = value;
        })
    }

    // Alignment
    pub fn setAlignContent(&mut self, value: Option<AlignContent>) -> Result<(), JsError> {
        with_style_mut!(self, style, style.align_content = value)
    }
    pub fn setJustifyContent(&mut self, value: Option<JustifyContent>) -> Result<(), JsError> {
        with_style_mut!(self, style, style.justify_content = value)
    }
    pub fn setAlignItems(&mut self, value: Option<AlignItems>) -> Result<(), JsError> {
        with_style_mut!(self, style, style.align_items = value)
    }
    pub fn setJustifyItems(&mut self, value: Option<JustifyItems>) -> Result<(), JsError> {
        with_style_mut!(self, style, style.justify_items = value)
    }
    pub fn setAlignSelf(&mut self, value: Option<AlignSelf>) -> Result<(), JsError> {
        with_style_mut!(self, style, style.align_self = value)
    }
    pub fn setJustifySelf(&mut self, value: Option<JustifySelf>) -> Result<(), JsError> {
        with_style_mut!(self, style, style.justify_self = value)
    }

    // Flex
    pub fn setFlexDirection(&mut self, value: FlexDirection) -> Result<(), JsError> {
        with_style_mut!(self, style, style.flex_direction = value)
    }
    pub fn setFlexWrap(&mut self, value: FlexWrap) -> Result<(), JsError> {
        with_style_mut!(self, style, style.flex_wrap = value)
    }
    pub fn setFlexGrow(&mut self, value: f32) -> Result<(), JsError> {
        with_style_mut!(self, style, style.flex_grow = value)
    }
    pub fn setFlexShrink(&mut self, value: f32) -> Result<(), JsError> {
        with_style_mut!(self, style, style.flex_shrink = value)
    }
    pub fn setFlexBasis(&mut self, value: f32, unit: StyleUnit) -> Result<(), JsError> {
        with_style_mut!(self, style, style.flex_basis = unit.try_into_dimension(value).unwrap())
    }

    // Grid
    // pub fn setGridAutoFlow(&mut self, value: GridAutoFlow) -> Result<(), JsError> {
    //     with_style_mut!(self, style, style.grid_auto_flow = value)
    // }
}
