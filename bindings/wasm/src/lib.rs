#![allow(non_snake_case)]

mod utils;

use core::str::FromStr;
use std::cell::RefCell;
use std::rc::Rc;

use js_sys::Function;
use js_sys::Reflect;
use taffy::style::*;
use taffy::tree::LayoutTree;
use wasm_bindgen::prelude::*;

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
    fn new(allocator: &Allocator, node: taffy::node::Node) -> Layout {
        let taffy = allocator.taffy.borrow();
        let layout = taffy.layout(node).unwrap();
        let children = taffy.children(node).unwrap();

        Layout {
            width: layout.size.width,
            height: layout.size.height,
            x: layout.location.x,
            y: layout.location.y,
            childCount: children.len(),
            children: children.into_iter().map(|child| Layout::new(allocator, child)).collect(),
        }
    }

    #[wasm_bindgen]
    pub fn child(&self, at: usize) -> Layout {
        self.children[at].clone()
    }
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct Allocator {
    taffy: Rc<RefCell<taffy::Taffy>>,
}

#[wasm_bindgen]
impl Allocator {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self { taffy: Rc::new(RefCell::new(taffy::Taffy::new())) }
    }
}

#[wasm_bindgen]
pub struct Node {
    allocator: Allocator,
    node: taffy::node::Node,
    style: JsValue,

    #[wasm_bindgen(readonly)]
    pub childCount: usize,
}

#[wasm_bindgen]
impl Node {
    #[wasm_bindgen(constructor)]
    pub fn new(allocator: &Allocator, style: &JsValue) -> Self {
        Self {
            allocator: allocator.clone(),
            node: allocator.taffy.borrow_mut().new_leaf(parse_style(&style)).unwrap(),
            style: style.clone(),
            childCount: 0,
        }
    }

    #[wasm_bindgen(js_name = setMeasure)]
    pub fn set_measure(&mut self, measure: &JsValue) {
        let _measure = Function::from(measure.clone());

        self.allocator
            .taffy
            .borrow_mut()
            .set_measure(
                self.node,
                // Some(taffy::node::MeasureFunc::Boxed(Box::new(
                //     move |constraints| {
                //         use taffy::number::OrElse;

                //         let widthConstraint =
                //             if let taffy::number::Number::Defined(val) = constraints.width {
                //                 val.into()
                //             } else {
                //                 JsValue::UNDEFINED
                //             };

                //         let heightConstaint =
                //             if let taffy::number::Number::Defined(val) = constraints.height {
                //                 val.into()
                //             } else {
                //                 JsValue::UNDEFINED
                //             };

                //         if let Ok(result) =
                //             measure.call2(&JsValue::UNDEFINED, &widthConstraint, &heightConstaint)
                //         {
                //             let width = get_f32(&result, "width");
                //             let height = get_f32(&result, "height");

                //             if width.is_some() && height.is_some() {
                //                 return taffy::geometry::Size {
                //                     width: width.unwrap(),
                //                     height: height.unwrap(),
                //                 };
                //             }
                //         }

                //         constraints.map(|v| v.or_else(0.0))
                //     },
                // ))),
                None,
            )
            .unwrap();
    }

    #[wasm_bindgen(js_name = addChild)]
    pub fn add_child(&mut self, child: &Node) {
        self.allocator.taffy.borrow_mut().add_child(self.node, child.node).unwrap();
        self.childCount += 1;
    }

    #[wasm_bindgen(js_name = removeChild)]
    pub fn remove_child(&mut self, child: &Node) {
        self.allocator.taffy.borrow_mut().remove_child(self.node, child.node).unwrap();
        self.childCount -= 1;
    }

    #[wasm_bindgen(js_name = replaceChildAtIndex)]
    pub fn replace_child_at_index(&mut self, index: usize, child: &Node) {
        self.allocator.taffy.borrow_mut().replace_child_at_index(self.node, index, child.node).unwrap();
    }

    #[wasm_bindgen(js_name = removeChildAtIndex)]
    pub fn remove_child_at_index(&mut self, index: usize) {
        self.allocator.taffy.borrow_mut().remove_child_at_index(self.node, index).unwrap();
        self.childCount -= 1;
    }

    #[wasm_bindgen(js_name = getStyle)]
    pub fn get_style(&self) -> JsValue {
        self.style.clone()
    }

    #[wasm_bindgen(js_name = setStyle)]
    pub fn set_style(&mut self, style: &JsValue) {
        self.allocator.taffy.borrow_mut().set_style(self.node, parse_style(style)).unwrap();
        self.style = style.clone();
    }

    #[wasm_bindgen(js_name = markDirty)]
    pub fn mark_dirty(&mut self) {
        self.allocator.taffy.borrow_mut().mark_dirty(self.node).unwrap()
    }

    #[wasm_bindgen(js_name = isDirty)]
    pub fn is_dirty(&self) -> bool {
        self.allocator.taffy.borrow().dirty(self.node).unwrap()
    }

    #[wasm_bindgen(js_name = isChildless)]
    pub fn is_childless(&mut self) -> bool {
        self.allocator.taffy.borrow_mut().is_childless(self.node)
    }

    #[wasm_bindgen(js_name = computeLayout)]
    pub fn compute_layout(&mut self, size: &JsValue) -> Layout {
        self.allocator
            .taffy
            .borrow_mut()
            .compute_layout(
                self.node,
                taffy::geometry::Size {
                    width: try_parse_available_space(size, "width").unwrap_or(AvailableSpace::MaxContent),
                    height: try_parse_available_space(size, "height").unwrap_or(AvailableSpace::MaxContent),
                },
            )
            .unwrap();
        Layout::new(&self.allocator, self.node)
    }
}

fn parse_style(style: &JsValue) -> taffy::style::Style {
    taffy::style::Style {
        display: try_parse_from_i32(style, "display").unwrap_or_default(),

        // Position styles
        position: try_parse_from_i32(style, "position").unwrap_or_default(),
        inset: taffy::geometry::Rect {
            left: try_parse_length_percentage_auto(style, "insetLeft").unwrap_or(LengthPercentageAuto::Auto),
            right: try_parse_length_percentage_auto(style, "insetRight").unwrap_or(LengthPercentageAuto::Auto),
            top: try_parse_length_percentage_auto(style, "insetTop").unwrap_or(LengthPercentageAuto::Auto),
            bottom: try_parse_length_percentage_auto(style, "insetBottom").unwrap_or(LengthPercentageAuto::Auto),
        },

        // Size styles
        size: taffy::geometry::Size {
            width: try_parse_dimension(style, "width").unwrap_or(Dimension::Auto),
            height: try_parse_dimension(style, "height").unwrap_or(Dimension::Auto),
        },
        min_size: taffy::geometry::Size {
            width: try_parse_dimension(style, "minWidth").unwrap_or(Dimension::Auto),
            height: try_parse_dimension(style, "minHeight").unwrap_or(Dimension::Auto),
        },
        max_size: taffy::geometry::Size {
            width: try_parse_dimension(style, "maxWidth").unwrap_or(Dimension::Auto),
            height: try_parse_dimension(style, "maxHeight").unwrap_or(Dimension::Auto),
        },
        aspect_ratio: get_f32(style, "aspectRatio"),

        // Alignment styles
        align_items: try_parse_from_i32(style, "alignItems"),
        align_self: try_parse_from_i32(style, "alignSelf"),
        align_content: try_parse_from_i32(style, "alignContent"),
        justify_content: try_parse_from_i32(style, "justifyContent"),
        justify_self: try_parse_from_i32(style, "justifySelf"),
        justify_items: try_parse_from_i32(style, "justifyItems"),

        // Spacing styles
        margin: taffy::geometry::Rect {
            left: try_parse_length_percentage_auto(style, "marginLeft").unwrap_or(LengthPercentageAuto::Points(0.0)),
            right: try_parse_length_percentage_auto(style, "marginRight").unwrap_or(LengthPercentageAuto::Points(0.0)),
            top: try_parse_length_percentage_auto(style, "marginTop").unwrap_or(LengthPercentageAuto::Points(0.0)),
            bottom: try_parse_length_percentage_auto(style, "marginBottom")
                .unwrap_or(LengthPercentageAuto::Points(0.0)),
        },
        padding: taffy::geometry::Rect {
            left: try_parse_length_percentage(style, "paddingLeft").unwrap_or(LengthPercentage::Points(0.0)),
            right: try_parse_length_percentage(style, "paddingRight").unwrap_or(LengthPercentage::Points(0.0)),
            top: try_parse_length_percentage(style, "paddingTop").unwrap_or(LengthPercentage::Points(0.0)),
            bottom: try_parse_length_percentage(style, "paddingBottom").unwrap_or(LengthPercentage::Points(0.0)),
        },
        border: taffy::geometry::Rect {
            left: try_parse_length_percentage(style, "borderLeft").unwrap_or(LengthPercentage::Points(0.0)),
            right: try_parse_length_percentage(style, "borderRight").unwrap_or(LengthPercentage::Points(0.0)),
            top: try_parse_length_percentage(style, "borderTop").unwrap_or(LengthPercentage::Points(0.0)),
            bottom: try_parse_length_percentage(style, "borderBottom").unwrap_or(LengthPercentage::Points(0.0)),
        },
        gap: taffy::geometry::Size {
            width: try_parse_length_percentage(style, "gapWidth").unwrap_or(LengthPercentage::Points(0.0)),
            height: try_parse_length_percentage(style, "gapHeight").unwrap_or(LengthPercentage::Points(0.0)),
        },

        // Flexbox styles
        flex_direction: try_parse_from_i32(style, "flexDirection").unwrap_or_default(),
        flex_wrap: try_parse_from_i32(style, "flexWrap").unwrap_or_default(),
        flex_grow: get_f32(style, "flexGrow").unwrap_or(0.0),
        flex_shrink: get_f32(style, "flexShrink").unwrap_or(1.0),
        flex_basis: try_parse_dimension(style, "flexBasis").unwrap_or(Dimension::Auto),

        // CSS Grid styles
        // TODO parse the remaining CSS Grid styles
        grid_auto_flow: try_parse_from_i32(style, "gridAutoFlow").unwrap_or_default(),
        grid_template_rows: Default::default(),
        grid_template_columns: Default::default(),
        grid_auto_rows: Default::default(),
        grid_auto_columns: Default::default(),
        grid_row: Default::default(),
        grid_column: Default::default(),
    }
}

#[allow(dead_code)]
fn has_key(obj: &JsValue, key: &str) -> bool {
    Reflect::has(obj, &key.into()).unwrap_or(false)
}

fn get_key(obj: &JsValue, key: &str) -> Option<JsValue> {
    Reflect::get(obj, &key.into()).ok()
}

fn get_i32(obj: &JsValue, key: &str) -> Option<i32> {
    get_key(obj, key).and_then(|val| val.as_f64().map(|v| v as i32))
}

fn get_f32(obj: &JsValue, key: &str) -> Option<f32> {
    get_key(obj, key).and_then(|val| val.as_f64().map(|v| v as f32))
}

fn try_parse_from_i32<T: TryFrom<i32>>(style: &JsValue, property_key: &'static str) -> Option<T> {
    get_i32(style, property_key).and_then(|i| T::try_from(i).ok())
}


fn try_parse_dimension(obj: &JsValue, key: &str) -> Option<Dimension> {
    if let Some(val) = get_key(obj, key) {
        if let Some(number) = val.as_f64() {
            return Some(Dimension::Points(number as f32));
        }
        if let Some(string) = val.as_string() {
            return string.parse().ok()
        }
    };
    None
}

// We first parse into a Dimension then use the TryFrom impl to attempt a conversion
fn try_parse_length_percentage_auto(obj: &JsValue, key: &str) -> Option<LengthPercentageAuto> {
    try_parse_dimension(obj, key).and_then(|dim| dim.try_into().ok())
}

// We first parse into a Dimension then use the TryFrom impl to attempt a conversion
fn try_parse_length_percentage(obj: &JsValue, key: &str) -> Option<LengthPercentage> {
    try_parse_dimension(obj, key).and_then(|dim| dim.try_into().ok())
}

fn try_parse_available_space(obj: &JsValue, key: &str) -> Option<AvailableSpace> {
    if let Some(val) = get_key(obj, key) {
        if let Some(number) = val.as_f64() {
            return Some(AvailableSpace::Definite(number as f32));
        }
        if let Some(string) = val.as_string() {
            return string.parse().ok()
        }
    }
    None
}

// Generic try_parse_dimension impl
// Could in theory be used to replace the above 4 functions, but it doesn't quite work and it's
// a bit confusing
// fn try_parse_dimension<U, T: FromStr + From<f32> + Into<U>>(obj: &JsValue, key: &str) -> Option<U> {
//     if let Some(val) = get_key(obj, key) {
//         if let Some(number) = val.as_f64() {
//             return Some(T::from(number as f32).into());
//         }
//         if let Some(string) = val.as_string() {
//             return string.parse::<T>().map(|val| val.into()).ok()
//         }
//     };
//     None
// }
