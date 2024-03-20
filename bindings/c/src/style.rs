//! Public API for C FFI

use super::{
    debug_assert_non_null, TaffyAlignContent, TaffyAlignItems, TaffyDimension, TaffyDisplay, TaffyEdge,
    TaffyFlexDirection, TaffyFlexWrap, TaffyGridAutoFlow, TaffyGridPlacement, TaffyOverflow, TaffyPosition,
    TaffyReturnCode, TaffyStyleConstRef, TaffyStyleMutRef, TaffyUnit,
};
use taffy::prelude as core;

/// Assert that the passed raw style pointer is non-null
/// Then give the passed expression access to the value of the inner [`core::Style`] struct pointed to by the raw style pointer
/// Return whatever the expression evaluates to wrapped in a [`TaffyDimensionResult`] if the expression does not interally return.
macro_rules! get_style {
    ($raw_style_ptr:expr, $style_ident:ident, $block:expr) => {{
        debug_assert_non_null!($raw_style_ptr);
        let $style_ident = unsafe { &*($raw_style_ptr as *const core::Style) };

        let return_value = $block;

        return_value.into()
    }};
}

/// Assert that the passed raw style pointer is non-null
/// Then give the passed expression mutable access to the value of the inner [`core::Style`] struct pointed to by the raw style pointer
/// Return [`TaffyReturnCode::Ok`] if the expression does not internally return.
macro_rules! with_style_mut {
    ($raw_style_ptr:expr, $style_ident:ident, $block:expr) => {{
        debug_assert_non_null!($raw_style_ptr);
        let $style_ident = unsafe { &mut *($raw_style_ptr as *mut core::Style) };

        $block;

        TaffyReturnCode::Ok
    }};
}

/// Attempt to convert a [`TaffyDimension`] into a type that implements `TryFrom<TaffyDimension>`
/// In the case of a conversion error, return a [`TaffyReturnCode`].
macro_rules! try_from_value {
    ($value:expr) => {
        match $value.try_into() {
            Ok(val) => val,
            Err(err) => return err,
        }
    };
}

/// Attempt to convert a [`TaffyUnit`] and a `f32` into a type that implements `TryFrom<TaffyDimension>`
/// In the case of a conversion error, return a [`TaffyReturnCode`].
macro_rules! try_from_raw {
    ($unit:expr, $value:expr) => {
        try_from_value!(TaffyDimension::from_raw($unit, $value))
    };
}

// Simple enum properties

macro_rules! enum_prop_getter {
    ($func_name:ident; $enum:ident; $($props:ident).+) => {
        #[no_mangle]
        #[allow(clippy::missing_safety_doc)]
        pub unsafe extern "C" fn $func_name(raw_style: TaffyStyleConstRef) -> $enum {
            get_style!(raw_style, style, style.$($props).*)
        }
    };
}

macro_rules! option_enum_prop_getter {
    ($func_name:ident; $($props:ident).+) => {
        #[no_mangle]
        #[allow(clippy::missing_safety_doc)]
        pub unsafe extern "C" fn $func_name(raw_style: TaffyStyleConstRef) -> i32 {
            get_style!(raw_style, style, style.$($props).*.map(|v| v as i32).unwrap_or(0))
        }
    };
}

// Generate a function to set a style value such as margin.top or size.width
macro_rules! enum_prop_setter {
    ($func_name:ident; $enum:ident; $($props:ident).+) => {
        #[no_mangle]
        #[allow(clippy::missing_safety_doc)]
        pub unsafe extern "C" fn $func_name(raw_style: TaffyStyleMutRef, value: $enum) -> TaffyReturnCode {
            with_style_mut!(raw_style, style, style.$($props).* = value.into())
        }
    };
}

// Generate a function to get a style value such as margin.top or size.width
macro_rules! style_value_prop_getter {
    ($func_name:ident; $($props:ident).+) => {
        #[no_mangle]
        #[allow(clippy::missing_safety_doc)]
        pub unsafe extern "C" fn $func_name(raw_style: TaffyStyleConstRef) -> TaffyDimension {
            get_style!(raw_style, style, style.$($props).*)
        }
    };
}

// Generate a function to set a style value such as margin.top or size.width
macro_rules! style_value_prop_setter {
    ($func_name:ident; $($props:ident).+) => {
        #[no_mangle]
        #[allow(clippy::missing_safety_doc)]
        pub unsafe extern "C" fn $func_name(raw_style: TaffyStyleMutRef, value: f32, unit: TaffyUnit) -> TaffyReturnCode {
            with_style_mut!(raw_style, style, style.$($props).* = try_from_raw!(unit, value))
        }
    };
}

// Generate a function to get a style value such as margin.top or size.width
macro_rules! float_prop_getter {
    ($func_name:ident; $($props:ident).+) => {
        #[no_mangle]
        #[allow(clippy::missing_safety_doc)]
        pub unsafe extern "C" fn $func_name(raw_style: TaffyStyleConstRef) -> f32 {
            get_style!(raw_style, style, style.$($props).*)
        }
    };
}

// Generate a function to set a style value such as margin.top or size.width
macro_rules! float_prop_setter {
    ($func_name:ident; $($props:ident).+) => {
        #[no_mangle]
        #[allow(clippy::missing_safety_doc)]
        pub unsafe extern "C" fn $func_name(raw_style: TaffyStyleMutRef, value: f32) -> TaffyReturnCode {
            with_style_mut!(raw_style, style, style.$($props).* = value)
        }
    };
}

enum_prop_getter!(TaffyStyle_GetDisplay; TaffyDisplay; display);
enum_prop_setter!(TaffyStyle_SetDisplay; TaffyDisplay; display);

// Position
enum_prop_getter!(TaffyStyle_GetPosition; TaffyPosition; position);
enum_prop_setter!(TaffyStyle_SetPosition; TaffyPosition; position);

// Overflow
enum_prop_getter!(TaffyStyle_GetOverflowX; TaffyOverflow; overflow.x);
enum_prop_setter!(TaffyStyle_SetOverflowX; TaffyOverflow; overflow.x);
enum_prop_getter!(TaffyStyle_GetOverflowY; TaffyOverflow; overflow.y);
enum_prop_setter!(TaffyStyle_SetOverflowY; TaffyOverflow; overflow.y);

// Alignment
option_enum_prop_getter!(TaffyStyle_GetAlignContent; align_content);
option_enum_prop_getter!(TaffyStyle_GetAlignItems; align_items);
option_enum_prop_getter!(TaffyStyle_GetAlignSelf; align_self);
option_enum_prop_getter!(TaffyStyle_GetJustifyContent; justify_content);
option_enum_prop_getter!(TaffyStyle_GetJustifyItems; justify_items);
option_enum_prop_getter!(TaffyStyle_GetJustifySelf; justify_self);
enum_prop_setter!(TaffyStyle_SetAlignContent; TaffyAlignContent; align_content);
enum_prop_setter!(TaffyStyle_SetAlignItems; TaffyAlignItems; align_items);
enum_prop_setter!(TaffyStyle_SetAlignSelf; TaffyAlignItems; align_self);
enum_prop_setter!(TaffyStyle_SetJustifyContent; TaffyAlignContent; justify_content);
enum_prop_setter!(TaffyStyle_SetJustifyItems; TaffyAlignItems; justify_items);
enum_prop_setter!(TaffyStyle_SetJustifySelf; TaffyAlignItems; justify_self);

// FlexDirection & FlexWrap
enum_prop_getter!(TaffyStyle_GetFlexDirection; TaffyFlexDirection; flex_direction);
enum_prop_setter!(TaffyStyle_SetFlexDirection; TaffyFlexDirection; flex_direction);
enum_prop_getter!(TaffyStyle_GetFlexWrap; TaffyFlexWrap; flex_wrap);
enum_prop_setter!(TaffyStyle_SetFlexWrap; TaffyFlexWrap; flex_wrap);

// GridAutoFlow
enum_prop_getter!(TaffyStyle_GetGridAutoFlow; TaffyGridAutoFlow; grid_auto_flow);
enum_prop_setter!(TaffyStyle_SetGridAutoFlow; TaffyGridAutoFlow; grid_auto_flow);

/* API variant with single parameter that combines "value" and "unit" into a `TaffyDimension` struct */

// Size
style_value_prop_getter!(TaffyStyle_GetWidth; size.width);
style_value_prop_setter!(TaffyStyle_SetWidth; size.width);
style_value_prop_getter!(TaffyStyle_GetHeight; size.height);
style_value_prop_setter!(TaffyStyle_SetHeight; size.height);

// MinSize
style_value_prop_getter!(TaffyStyle_GetMinWidth; min_size.width);
style_value_prop_setter!(TaffyStyle_SetMinWidth; min_size.width);
style_value_prop_getter!(TaffyStyle_GetMinHeight; min_size.height);
style_value_prop_setter!(TaffyStyle_SetMinHeight; min_size.height);

// MaxSize
style_value_prop_getter!(TaffyStyle_GetMaxWidth; max_size.width);
style_value_prop_setter!(TaffyStyle_SetMaxWidth; max_size.width);
style_value_prop_getter!(TaffyStyle_GetMaxHeight; max_size.height);
style_value_prop_setter!(TaffyStyle_SetMaxHeight; max_size.height);

// Inset
style_value_prop_getter!(TaffyStyle_GetInsetTop; inset.top);
style_value_prop_setter!(TaffyStyle_SetInsetTop; inset.top);
style_value_prop_getter!(TaffyStyle_GetInsetBottom; inset.bottom);
style_value_prop_setter!(TaffyStyle_SetInsetBottom; inset.bottom);
style_value_prop_getter!(TaffyStyle_GetInsetLeft; inset.left);
style_value_prop_getter!(TaffyStyle_GetInsetRight; inset.right);
style_value_prop_setter!(TaffyStyle_SetInsetLeft; inset.left);
style_value_prop_setter!(TaffyStyle_SetInsetRight; inset.right);

// Margin
style_value_prop_getter!(TaffyStyle_GetMarginTop; margin.top);
style_value_prop_setter!(TaffyStyle_SetMarginTop; margin.top);
style_value_prop_getter!(TaffyStyle_GetMarginBottom; margin.bottom);
style_value_prop_setter!(TaffyStyle_SetMarginBottom; margin.bottom);
style_value_prop_getter!(TaffyStyle_GetMarginLeft; margin.left);
style_value_prop_getter!(TaffyStyle_GetMarginRight; margin.right);
style_value_prop_setter!(TaffyStyle_SetMarginLeft; margin.left);
style_value_prop_setter!(TaffyStyle_SetMarginRight; margin.right);

// Padding
style_value_prop_getter!(TaffyStyle_GetPaddingTop; padding.top);
style_value_prop_setter!(TaffyStyle_SetPaddingTop; padding.top);
style_value_prop_getter!(TaffyStyle_GetPaddingBottom; padding.bottom);
style_value_prop_setter!(TaffyStyle_SetPaddingBottom; padding.bottom);
style_value_prop_getter!(TaffyStyle_GetPaddingLeft; padding.left);
style_value_prop_getter!(TaffyStyle_GetPaddingRight; padding.right);
style_value_prop_setter!(TaffyStyle_SetPaddingLeft; padding.left);
style_value_prop_setter!(TaffyStyle_SetPaddingRight; padding.right);

// Border
style_value_prop_getter!(TaffyStyle_GetBorderTop; border.top);
style_value_prop_setter!(TaffyStyle_SetBorderTop; border.top);
style_value_prop_getter!(TaffyStyle_GetBorderBottom; border.bottom);
style_value_prop_setter!(TaffyStyle_SetBorderBottom; border.bottom);
style_value_prop_getter!(TaffyStyle_GetBorderLeft; border.left);
style_value_prop_getter!(TaffyStyle_GetBorderRight; border.right);
style_value_prop_setter!(TaffyStyle_SetBorderLeft; border.left);
style_value_prop_setter!(TaffyStyle_SetBorderRight; border.right);

// Gap
style_value_prop_getter!(TaffyStyle_GetColumnGap; gap.width);
style_value_prop_setter!(TaffyStyle_SetColumnGap; gap.width);
style_value_prop_getter!(TaffyStyle_GetRowGap; gap.height);
style_value_prop_setter!(TaffyStyle_SetRowGap; gap.height);

// Aspect ratio
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn TaffyStyle_GetAspectRatio(raw_style: TaffyStyleConstRef) -> f32 {
    get_style!(raw_style, style, style.aspect_ratio.unwrap_or(f32::NAN))
}
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn TaffyStyle_SetAspectRatio(raw_style: TaffyStyleMutRef, value: f32) -> TaffyReturnCode {
    with_style_mut!(raw_style, style, {
        if value.is_finite() && value > 0.0 {
            style.aspect_ratio = Some(value)
        } else {
            style.aspect_ratio = None;
        }
    })
}

// Scrollbar width
float_prop_getter!(TaffyStyle_GetScrollbarWidth; scrollbar_width);
float_prop_setter!(TaffyStyle_SetScrollbarWidth; scrollbar_width);

// Flex
style_value_prop_getter!(TaffyStyle_GetFlexBasis; flex_basis);
style_value_prop_setter!(TaffyStyle_SetFlexBasis; flex_basis);
float_prop_getter!(TaffyStyle_GetFlexGrow; flex_grow);
float_prop_setter!(TaffyStyle_SetFlexGrow; flex_grow);
float_prop_getter!(TaffyStyle_GetFlexShrink; flex_shrink);
float_prop_setter!(TaffyStyle_SetFlexShrink; flex_shrink);

/// Function to set all the value of margin
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn TaffyStyle_SetMargin(
    raw_style: TaffyStyleMutRef,
    edge: TaffyEdge,
    value: TaffyDimension,
) -> TaffyReturnCode {
    let value = try_from_value!(value);
    with_style_mut!(raw_style, style, {
        match edge {
            TaffyEdge::Top => style.margin.top = value,
            TaffyEdge::Bottom => style.margin.bottom = value,
            TaffyEdge::Left => style.margin.left = value,
            TaffyEdge::Right => style.margin.right = value,
            TaffyEdge::Vertical => {
                style.margin.top = value;
                style.margin.bottom = value;
            }
            TaffyEdge::Horizontal => {
                style.margin.left = value;
                style.margin.right = value;
            }
            TaffyEdge::All => {
                style.margin.top = value;
                style.margin.bottom = value;
                style.margin.left = value;
                style.margin.right = value;
            }
        };
    })
}

/* Grid APIs */

/// Get grid item's column placement
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn TaffyStyle_GetGridColumn(raw_style: TaffyStyleMutRef) -> TaffyGridPlacement {
    get_style!(raw_style, style, style.grid_column)
}

/// Set grid item's column placement
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn TaffyStyle_SetGridColumn(
    raw_style: TaffyStyleMutRef,
    placement: TaffyGridPlacement,
) -> TaffyReturnCode {
    with_style_mut!(raw_style, style, style.grid_column = placement.into())
}

/// Get grid item's row placement
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn TaffyStyle_GetGridRow(raw_style: TaffyStyleMutRef) -> TaffyGridPlacement {
    get_style!(raw_style, style, style.grid_row)
}

/// Set grid item's row placement
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn TaffyStyle_SetGridRow(
    raw_style: TaffyStyleMutRef,
    placement: TaffyGridPlacement,
) -> TaffyReturnCode {
    with_style_mut!(raw_style, style, style.grid_row = placement.into())
}
