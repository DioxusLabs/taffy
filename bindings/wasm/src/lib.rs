#![allow(non_snake_case)]

mod utils;

use std::cell::RefCell;
use std::rc::Rc;

use js_sys::Array;
use js_sys::Function;
use js_sys::Reflect;
use taffy::prelude::*;
use taffy::tree::LayoutTree;
use wasm_bindgen::prelude::*;

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
    fn has_value(&self) -> bool {
        use StyleUnit::*;
        matches!(self, Px | Percent | FitContentPx | FitContentPercent | Fr)
    }

    fn try_into_dimension(self, val: f32) -> Result<Dimension, ()> {
        match self {
            StyleUnit::Px => Ok(Dimension::Points(val)),
            StyleUnit::Percent => Ok(Dimension::Percent(val)),
            StyleUnit::Auto => Ok(Dimension::Auto),
            _ => Err(()),
        }
    }

    fn try_into_length_percentage_auto(self, val: f32) -> Result<LengthPercentageAuto, ()> {
        match self {
            StyleUnit::Px => Ok(LengthPercentageAuto::Points(val)),
            StyleUnit::Percent => Ok(LengthPercentageAuto::Percent(val)),
            StyleUnit::Auto => Ok(LengthPercentageAuto::Auto),
            _ => Err(()),
        }
    }

    fn try_into_length_percentage(self, val: f32) -> Result<LengthPercentage, ()> {
        match self {
            StyleUnit::Px => Ok(LengthPercentage::Points(val)),
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

    #[wasm_bindgen(readonly)]
    pub childCount: usize,
}

#[wasm_bindgen]
impl Node {
    #[wasm_bindgen(constructor)]
    pub fn new(allocator: &Allocator) -> Self {
        Self {
            allocator: allocator.clone(),
            node: allocator.taffy.borrow_mut().new_leaf(Style::DEFAULT).unwrap(),
            childCount: 0,
        }
    }

    #[wasm_bindgen(js_name = setMeasure)]
    pub fn set_measure(&mut self, measure: &JsValue) {
        // let js_measure_func = Arc::new(Mutex::new(Function::from(measure.clone())));

        struct FuncWrap(Function);
        impl FuncWrap {
            fn apply(&self, this: &JsValue, args: &Array) -> Result<JsValue, JsValue> {
                self.0.apply(this, args)
            }
        }
        // SAFETY: Wasm is single-threaded so there can't be multiple threads
        unsafe impl Send for FuncWrap {}
        unsafe impl Sync for FuncWrap {}

        let js_measure_func = FuncWrap(Function::from(measure.clone()));

        let measure_func = move |known_dimensions: Size<Option<f32>>, available_space: Size<AvailableSpace>| {
            fn convert_available_space(val: AvailableSpace) -> JsValue {
                match val {
                    AvailableSpace::Definite(val) => val.into(),
                    AvailableSpace::MaxContent => JsValue::from_str("max-content"),
                    AvailableSpace::MinContent => JsValue::from_str("min-content"),
                }
            }

            let known_width = known_dimensions.width.map(|val| val.into()).unwrap_or(JsValue::UNDEFINED);
            let known_height = known_dimensions.height.map(|val| val.into()).unwrap_or(JsValue::UNDEFINED);

            let available_width = convert_available_space(available_space.width);
            let available_height = convert_available_space(available_space.height);

            let args = Array::new_with_length(4);
            args.set(0, known_width);
            args.set(1, known_height);
            args.set(2, available_width);
            args.set(3, available_height);

            if let Ok(result) = js_measure_func.apply(&JsValue::UNDEFINED, &args) {
                let width = get_f32(&result, "width");
                let height = get_f32(&result, "height");

                if width.is_some() && height.is_some() {
                    return Size { width: width.unwrap(), height: height.unwrap() };
                }
            }

            known_dimensions.unwrap_or(Size::ZERO)
        };

        self.allocator
            .taffy
            .borrow_mut()
            .set_measure(self.node, Some(taffy::node::MeasureFunc::Boxed(Box::new(measure_func))))
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

macro_rules! get_style {
    ($self:expr, $style_ident:ident, $block:expr) => {{
        let taffy = $self.allocator.taffy.borrow();
        let $style_ident = taffy.style($self.node)?;
        Ok($block)
    }};
}

macro_rules! with_style_mut {
    ($self:expr, $style_ident:ident, $block:expr) => {{
        let mut taffy = $self.allocator.taffy.borrow_mut();
        let $style_ident = taffy.style_mut($self.node)?;
        $block;
        Ok(())
    }};
}

// Style getter/setter methods
#[wasm_bindgen]
#[clippy::allow(non_snake_case)]
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
    pub fn setAspectRatio(&mut self, value: f32) -> Result<(), JsError> {
        with_style_mut!(self, style, style.aspect_ratio = option_from_f32(value))
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
    // TODO: Allow None values to be set
    pub fn setAlignContent(&mut self, value: AlignContent) -> Result<(), JsError> {
        with_style_mut!(self, style, style.align_content = Some(value))
    }
    pub fn setJustifyContent(&mut self, value: JustifyContent) -> Result<(), JsError> {
        with_style_mut!(self, style, style.justify_content = Some(value))
    }
    pub fn setAlignItems(&mut self, value: AlignItems) -> Result<(), JsError> {
        with_style_mut!(self, style, style.align_items = Some(value))
    }
    pub fn setJustifyItems(&mut self, value: JustifyItems) -> Result<(), JsError> {
        with_style_mut!(self, style, style.justify_items = Some(value))
    }
    pub fn setAlignSelf(&mut self, value: AlignSelf) -> Result<(), JsError> {
        with_style_mut!(self, style, style.align_self = Some(value))
    }
    pub fn setJustifySelf(&mut self, value: JustifySelf) -> Result<(), JsError> {
        with_style_mut!(self, style, style.justify_self = Some(value))
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

    // #[wasm_bindgen(js_name = setHeightStr)]
    // pub fn set_height_str(&mut self, height: &str) -> Result<(), JsError> {
    //     with_style_mut!(self, style, style.size.height = height.parse().unwrap())
    // }
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

fn option_from_f32(value: f32) -> Option<f32> {
    if value.is_nan() {
        None
    } else {
        Some(value)
    }
}

fn try_parse_dimension(obj: &JsValue, key: &str) -> Option<Dimension> {
    if let Some(val) = get_key(obj, key) {
        if let Some(number) = val.as_f64() {
            return Some(Dimension::Points(number as f32));
        }
        if let Some(string) = val.as_string() {
            return string.parse().ok();
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
            return string.parse().ok();
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