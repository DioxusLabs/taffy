use jni::JNIEnv;
use jni::objects::{JObject, JValueOwned};
use taffy::{AlignContent, AlignItems, BoxSizing, Display, FlexDirection, FlexWrap, GridAutoFlow, Overflow, Position, TextAlign};
use crate::conversions::f_get_value;
use crate::primitives::f_i32_from_primitive;

pub fn get_overflow<'local>(env: &mut JNIEnv<'local>, value: JValueOwned<'local>) -> Overflow {
    let obj = &value.l().unwrap();
    if obj.is_null() {
        return Overflow::default();
    }

    let internal = get_enum_value(env, obj);

    match internal {
        0 => Overflow::Visible,
        1 => Overflow::Clip,
        2 => Overflow::Hidden,
        _ => Overflow::Scroll
    }
}

#[allow(dead_code)]
pub fn f_get_overflow<'local>(env: &mut JNIEnv<'local>, base: &JObject<'local>, field: &str) -> Overflow {
    let obj = f_get_value(env, base, field, "Lcom/dioxuslabs/taffy/style/TaffyOverflow;");

    get_overflow(env, obj)
}

pub fn get_position<'local>(env: &mut JNIEnv<'local>, value: JValueOwned<'local>) -> Position {
    let obj = &value.l().unwrap();
    if obj.is_null() {
        return Position::default();
    }

    let internal = get_enum_value(env, obj);

    match internal {
        0 => Position::Relative,
        _ => Position::Absolute
    }
}

pub fn f_get_position<'local>(env: &mut JNIEnv<'local>, base: &JObject<'local>, field: &str) -> Position {
    let obj = f_get_value(env, base, field, "Lcom/dioxuslabs/taffy/style/TaffyPosition;");

    get_position(env, obj)
}

pub fn get_text_align<'local>(env: &mut JNIEnv<'local>, value: JValueOwned<'local>, def: fn() -> TextAlign) -> TextAlign {
    let obj = &value.l().unwrap();
    if obj.is_null() {
        return def();
    }

    let internal = get_enum_value(env, obj);

    match internal {
        0 => TextAlign::Auto,
        1 => TextAlign::LegacyLeft,
        2 => TextAlign::LegacyRight,
        _ => TextAlign::LegacyCenter
    }
}

pub fn f_get_text_align<'local>(env: &mut JNIEnv<'local>, base: &JObject<'local>, field: &str) -> TextAlign {
    let obj = f_get_value(env, base, field, "Lcom/dioxuslabs/taffy/style/TaffyTextAlign;");

    get_text_align(env, obj, TextAlign::default)
}

pub fn get_flex_direction<'local>(env: &mut JNIEnv<'local>, value: JValueOwned<'local>, def: fn() -> FlexDirection) -> FlexDirection {
    let obj = &value.l().unwrap();
    if obj.is_null() {
        return def();
    }

    let internal = get_enum_value(env, obj);

    match internal {
        0 => FlexDirection::Row,
        1 => FlexDirection::Column,
        2 => FlexDirection::RowReverse,
        _ => FlexDirection::ColumnReverse
    }
}

pub fn f_get_flex_direction<'local>(env: &mut JNIEnv<'local>, base: &JObject<'local>, field: &str) -> FlexDirection {
    let obj = f_get_value(env, base, field, "Lcom/dioxuslabs/taffy/style/TaffyFlexDirection;");

    get_flex_direction(env, obj, FlexDirection::default)
}

pub fn get_flex_wrap<'local>(env: &mut JNIEnv<'local>, value: JValueOwned<'local>, def: fn() -> FlexWrap) -> FlexWrap {
    let obj = &value.l().unwrap();
    if obj.is_null() {
        return def();
    }

    let internal = get_enum_value(env, obj);

    match internal {
        0 => FlexWrap::NoWrap,
        1 => FlexWrap::Wrap,
        _ => FlexWrap::WrapReverse
    }
}

pub fn f_get_flex_wrap<'local>(env: &mut JNIEnv<'local>, base: &JObject<'local>, field: &str) -> FlexWrap {
    let obj = f_get_value(env, base, field, "Lcom/dioxuslabs/taffy/style/TaffyFlexWrap;");

    get_flex_wrap(env, obj, FlexWrap::default)
}

pub fn get_grid_auto_flow<'local>(env: &mut JNIEnv<'local>, value: JValueOwned<'local>, def: fn() -> GridAutoFlow) -> GridAutoFlow {
    let obj = &value.l().unwrap();
    if obj.is_null() {
        return def();
    }

    let internal = get_enum_value(env, obj);

    match internal {
        0 => GridAutoFlow::Row,
        1 => GridAutoFlow::Column,
        2 => GridAutoFlow::RowDense,
        _ => GridAutoFlow::ColumnDense
    }
}

pub fn f_get_grid_auto_flow<'local>(env: &mut JNIEnv<'local>, base: &JObject<'local>, field: &str) -> GridAutoFlow {
    let obj = f_get_value(env, base, field, "Lcom/dioxuslabs/taffy/style/TaffyGridAutoFlow;");

    get_grid_auto_flow(env, obj, GridAutoFlow::default)
}

pub fn get_align_items<'local>(env: &mut JNIEnv<'local>, value: JValueOwned<'local>) -> Option<AlignItems> {
    let obj = &value.l().unwrap();
    if obj.is_null() {
        return None;
    }

    let internal = get_enum_value(env, obj);

    match internal {
        0 => Some(AlignItems::Start),
        1 => Some(AlignItems::End),
        2 => Some(AlignItems::FlexStart),
        3 => Some(AlignItems::FlexEnd),
        4 => Some(AlignItems::Center),
        5 => Some(AlignItems::Baseline),
        _ => Some(AlignItems::Stretch)
    }
}

pub fn f_get_align_items<'local>(env: &mut JNIEnv<'local>, base: &JObject<'local>, field: &str) -> Option<AlignItems> {
    let obj = f_get_value(env, base, field, "Lcom/dioxuslabs/taffy/style/TaffyAlignItems;");

    get_align_items(env, obj)
}

pub fn get_align_content<'local>(env: &mut JNIEnv<'local>, value: JValueOwned<'local>) -> Option<AlignContent> {
    let obj = &value.l().unwrap();
    if obj.is_null() {
        return None;
    }

    let internal = get_enum_value(env, obj);

    match internal {
        0 => Some(AlignContent::Start),
        1 => Some(AlignContent::End),
        2 => Some(AlignContent::FlexStart),
        3 => Some(AlignContent::FlexEnd),
        4 => Some(AlignContent::Center),
        5 => Some(AlignContent::Stretch),
        6 => Some(AlignContent::SpaceBetween),
        7 => Some(AlignContent::SpaceEvenly),
        _ => Some(AlignContent::SpaceAround)
    }
}

pub fn f_get_align_content<'local>(env: &mut JNIEnv<'local>, base: &JObject<'local>, field: &str) -> Option<AlignContent> {
    let obj = f_get_value(env, base, field, "Lcom/dioxuslabs/taffy/style/TaffyAlignContent;");

    get_align_content(env, obj)
}

pub fn get_display<'local>(env: &mut JNIEnv<'local>, value: JValueOwned<'local>) -> Display {
    let obj = &value.l().unwrap();
    if obj.is_null() {
        return Display::default();
    }

    let internal = get_enum_value(env, obj);

    match internal {
        0 => Display::Block,
        1 => Display::Flex,
        2 => Display::Grid,
        _ => Display::None,
    }
}

pub fn f_get_display<'local>(env: &mut JNIEnv<'local>, base: &JObject<'local>, field: &str) -> Display {
    let obj = f_get_value(env, base, field, "Lcom/dioxuslabs/taffy/style/TaffyDisplay;");

    get_display(env, obj)
}

pub fn get_box_sizing<'local>(env: &mut JNIEnv<'local>, value: JValueOwned<'local>) -> BoxSizing {
    let obj = &value.l().unwrap();
    if obj.is_null() {
        return BoxSizing::default();
    }

    let internal = get_enum_value(env, obj);

    match internal {
        0 => BoxSizing::BorderBox,
        _ => BoxSizing::ContentBox
    }
}

pub fn f_get_box_sizing<'local>(env: &mut JNIEnv<'local>, base: &JObject<'local>, field: &str) -> BoxSizing {
    let obj = f_get_value(env, base, field, "Lcom/dioxuslabs/taffy/style/TaffyBoxSizing;");

    get_box_sizing(env, obj)
}

fn get_enum_value<'local>(env: &mut JNIEnv<'local>, object: &JObject<'local>) -> i32 {
    f_i32_from_primitive(env, object, "internal", || 0)
}
