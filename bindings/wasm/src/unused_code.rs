
// Style getter/setter methods
#[wasm_bindgen]
#[clippy::allow(non_snake_case)]
impl Node {
    // #[wasm_bindgen(js_name = setHeightStr)]
    // pub fn set_height_str(&mut self, height: &str) -> Result<(), JsError> {
    //     with_style_mut!(self, style, style.size.height = height.parse().unwrap())
    // }
}

impl StyleUnit {
    fn has_value(&self) -> bool {
        use StyleUnit::*;
        matches!(self, Px | Percent | FitContentPx | FitContentPercent | Fr)
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

        // Scroll styles
        overflow: taffy::geometry::Point {
            x: try_parse_from_i32(style, "overflowX").unwrap_or_default(),
            y: try_parse_from_i32(style, "overflowY").unwrap_or_default(),
        },
        scrollbar_width: get_f32(style, "scrollbarWidth").unwrap_or(0.0),

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
            left: try_parse_length_percentage_auto(style, "marginLeft").unwrap_or(LengthPercentageAuto::Length(0.0)),
            right: try_parse_length_percentage_auto(style, "marginRight").unwrap_or(LengthPercentageAuto::Length(0.0)),
            top: try_parse_length_percentage_auto(style, "marginTop").unwrap_or(LengthPercentageAuto::Length(0.0)),
            bottom: try_parse_length_percentage_auto(style, "marginBottom")
                .unwrap_or(LengthPercentageAuto::Length(0.0)),
        },
        padding: taffy::geometry::Rect {
            left: try_parse_length_percentage(style, "paddingLeft").unwrap_or(LengthPercentage::Length(0.0)),
            right: try_parse_length_percentage(style, "paddingRight").unwrap_or(LengthPercentage::Length(0.0)),
            top: try_parse_length_percentage(style, "paddingTop").unwrap_or(LengthPercentage::Length(0.0)),
            bottom: try_parse_length_percentage(style, "paddingBottom").unwrap_or(LengthPercentage::Length(0.0)),
        },
        border: taffy::geometry::Rect {
            left: try_parse_length_percentage(style, "borderLeft").unwrap_or(LengthPercentage::Length(0.0)),
            right: try_parse_length_percentage(style, "borderRight").unwrap_or(LengthPercentage::Length(0.0)),
            top: try_parse_length_percentage(style, "borderTop").unwrap_or(LengthPercentage::Length(0.0)),
            bottom: try_parse_length_percentage(style, "borderBottom").unwrap_or(LengthPercentage::Length(0.0)),
        },
        gap: taffy::geometry::Size {
            width: try_parse_length_percentage(style, "gapWidth").unwrap_or(LengthPercentage::Length(0.0)),
            height: try_parse_length_percentage(style, "gapHeight").unwrap_or(LengthPercentage::Length(0.0)),
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

fn get_i32(obj: &JsValue, key: &str) -> Option<i32> {
    get_key(obj, key).and_then(|val| val.as_f64().map(|v| v as i32))
}

fn try_parse_from_i32<T: TryFrom<i32>>(style: &JsValue, property_key: &'static str) -> Option<T> {
    get_i32(style, property_key).and_then(|i| T::try_from(i).ok())
}

fn try_parse_dimension(obj: &JsValue, key: &str) -> Option<Dimension> {
    if let Some(val) = get_key(obj, key) {
        if let Some(number) = val.as_f64() {
            return Some(Dimension::Length(number as f32));
        }
        if let Some(string) = val.as_string() {
            return string.parse().ok();
        }
    };
    None
}

/// Convert an f32 to an Option<f32> by mapping NaN values to None
fn option_from_f32(value: f32) -> Option<f32> {
    if value.is_nan() {
        None
    } else {
        Some(value)
    }
}

// We first parse into a Dimension then use the TryFrom impl to attempt a conversion
fn try_parse_length_percentage_auto(obj: &JsValue, key: &str) -> Option<LengthPercentageAuto> {
    try_parse_dimension(obj, key).and_then(|dim| dim.try_into().ok())
}

// We first parse into a Dimension then use the TryFrom impl to attempt a conversion
fn try_parse_length_percentage(obj: &JsValue, key: &str) -> Option<LengthPercentage> {
    try_parse_dimension(obj, key).and_then(|dim| dim.try_into().ok())
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