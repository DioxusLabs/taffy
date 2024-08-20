use crate::collections::f_get_list;
use crate::conversions::f_get_value;
use crate::geom::{f_get_non_repeated_track_sizing_function, get_opt_non_repeated_track_sizing_function};
use crate::primitives::{f_f32_from_primitive, f_i16_from_primitive, f_i8_from_primitive};
use jni::objects::{JObject, JValueOwned};
use jni::JNIEnv;
use taffy::style_helpers::{TaffyGridLine, TaffyGridSpan};
use taffy::{
    AvailableSpace, Dimension, GridPlacement, GridTrackRepetition, LengthPercentage, LengthPercentageAuto,
    MaxTrackSizingFunction, MinTrackSizingFunction, TrackSizingFunction,
};

pub fn get_length_percentage<'local>(env: &mut JNIEnv<'local>, value: JValueOwned<'local>) -> LengthPercentage {
    let obj = &value.l().unwrap();

    if obj.is_null() {
        panic!("LengthPercentage cannot be null");
    }

    let internal = f_i8_from_primitive(env, obj, "type", || 0);

    match internal {
        0 => LengthPercentage::Length(f_f32_from_primitive(env, obj, "value", || 0.0)),
        _ => LengthPercentage::Percent(f_f32_from_primitive(env, obj, "value", || 0.0)),
    }
}

pub fn get_length_percentage_auto<'local>(
    env: &mut JNIEnv<'local>,
    value: JValueOwned<'local>,
) -> LengthPercentageAuto {
    let obj = &value.l().unwrap();

    if obj.is_null() {
        panic!("LengthPercentageAuto cannot be null");
    }

    let internal = f_i8_from_primitive(env, obj, "type", || 0);

    match internal {
        0 => LengthPercentageAuto::Length(f_f32_from_primitive(env, obj, "value", || 0.0)),
        1 => LengthPercentageAuto::Percent(f_f32_from_primitive(env, obj, "value", || 0.0)),
        _ => LengthPercentageAuto::Auto,
    }
}

pub fn get_dimension<'local>(env: &mut JNIEnv<'local>, value: JValueOwned<'local>) -> Dimension {
    let obj = &value.l().unwrap();

    if obj.is_null() {
        panic!("Dimension cannot be null");
    }

    let internal = f_i8_from_primitive(env, obj, "type", || 0);

    match internal {
        0 => Dimension::Length(f_f32_from_primitive(env, obj, "value", || 0.0)),
        1 => Dimension::Percent(f_f32_from_primitive(env, obj, "value", || 0.0)),
        _ => Dimension::Auto,
    }
}

pub fn get_available_space<'local>(env: &mut JNIEnv<'local>, value: JValueOwned<'local>) -> AvailableSpace {
    let obj = &value.l().unwrap();

    if obj.is_null() {
        panic!("AvailableSpace cannot be null");
    }

    let internal = f_i8_from_primitive(env, obj, "type", || 0);

    match internal {
        0 => AvailableSpace::Definite(f_f32_from_primitive(env, obj, "value", || 0.0)),
        1 => AvailableSpace::MinContent,
        _ => AvailableSpace::MaxContent,
    }
}

pub fn get_dimension_or<'local>(
    env: &mut JNIEnv<'local>,
    value: JValueOwned<'local>,
    f: fn() -> Dimension,
) -> Dimension {
    let obj = &value.l().unwrap();

    if obj.is_null() {
        return f();
    }

    let internal = f_i8_from_primitive(env, obj, "type", || 0);

    match internal {
        0 => Dimension::Length(f_f32_from_primitive(env, obj, "value", || 0.0)),
        1 => Dimension::Percent(f_f32_from_primitive(env, obj, "value", || 0.0)),
        _ => Dimension::Auto,
    }
}

pub fn get_grid_track_repetition<'local>(env: &mut JNIEnv<'local>, value: JValueOwned<'local>) -> GridTrackRepetition {
    let obj = &value.l().unwrap();

    if obj.is_null() {
        panic!("GridTrackRepetition cannot be null");
    }

    let internal = f_i8_from_primitive(env, obj, "type", || 0);

    match internal {
        0 => GridTrackRepetition::AutoFill,
        1 => GridTrackRepetition::AutoFit,
        _ => GridTrackRepetition::Count(f_i16_from_primitive(env, obj, "value", || 0) as u16),
    }
}

pub fn get_grid_placement<'local>(env: &mut JNIEnv<'local>, value: JValueOwned<'local>) -> GridPlacement {
    let obj = &value.l().unwrap();

    if obj.is_null() {
        panic!("GridPlacement cannot be null");
    }

    let internal = f_i8_from_primitive(env, obj, "type", || 0);

    match internal {
        0 => GridPlacement::Auto,
        1 => GridPlacement::from_line_index(f_i16_from_primitive(env, obj, "value", || 0)),
        _ => GridPlacement::from_span(f_i16_from_primitive(env, obj, "value", || 0) as u16),
    }
}

pub fn get_min_track_sizing_function<'local>(
    env: &mut JNIEnv<'local>,
    value: JValueOwned<'local>,
    def: fn() -> MinTrackSizingFunction,
) -> MinTrackSizingFunction {
    let obj = &value.l().unwrap();

    if obj.is_null() {
        return def();
    }

    let internal = f_i8_from_primitive(env, obj, "type", || 0);

    match internal {
        0 => MinTrackSizingFunction::Fixed(f_get_length_percentage(env, obj, "value")),
        1 => MinTrackSizingFunction::MinContent,
        2 => MinTrackSizingFunction::MaxContent,
        _ => MinTrackSizingFunction::Auto,
    }
}

pub fn get_max_track_sizing_function<'local>(
    env: &mut JNIEnv<'local>,
    value: JValueOwned<'local>,
    def: fn() -> MaxTrackSizingFunction,
) -> MaxTrackSizingFunction {
    let obj = &value.l().unwrap();

    if obj.is_null() {
        return def();
    }

    let internal = f_i8_from_primitive(env, obj, "type", || 0);

    match internal {
        0 => MaxTrackSizingFunction::Fixed(f_get_length_percentage(env, obj, "value")),
        1 => MaxTrackSizingFunction::MinContent,
        2 => MaxTrackSizingFunction::MaxContent,
        3 => MaxTrackSizingFunction::FitContent(f_get_length_percentage(env, obj, "value")),
        4 => MaxTrackSizingFunction::Auto,
        _ => MaxTrackSizingFunction::Fraction(f_f32_from_primitive(env, obj, "value", || 0.0)),
    }
}

pub fn get_opt_track_sizing_function<'local>(
    env: &mut JNIEnv<'local>,
    value: JValueOwned<'local>,
) -> Option<TrackSizingFunction> {
    let obj = &value.l().unwrap();

    if obj.is_null() {
        return None;
    }

    let internal = f_i8_from_primitive(env, obj, "type", || 0);

    Some(match internal {
        0 => TrackSizingFunction::Single(f_get_non_repeated_track_sizing_function(env, obj, "func")),
        _ => TrackSizingFunction::Repeat(f_get_grid_track_repetition(env, obj, "repetitions"), unsafe {
            f_get_list(env, obj, "functions", get_opt_non_repeated_track_sizing_function)
        }),
    })
}

pub fn f_get_length_percentage<'local>(
    env: &mut JNIEnv<'local>,
    base: &JObject<'local>,
    field: &str,
) -> LengthPercentage {
    let value = f_get_value(env, base, field, "Lcom/dioxuslabs/taffy/geom/measure/TaffyLengthPercentage;");

    get_length_percentage(env, value)
}

#[allow(dead_code)]
pub fn f_get_dimension<'local>(env: &mut JNIEnv<'local>, base: &JObject<'local>, field: &str) -> Dimension {
    let value = f_get_value(env, base, field, "Lcom/dioxuslabs/taffy/geom/measure/TaffyDimension;");

    get_dimension(env, value)
}

pub fn f_get_dimension_or<'local>(
    env: &mut JNIEnv<'local>,
    base: &JObject<'local>,
    field: &str,
    f: fn() -> Dimension,
) -> Dimension {
    let value = f_get_value(env, base, field, "Lcom/dioxuslabs/taffy/geom/measure/TaffyDimension;");

    get_dimension_or(env, value, f)
}

pub fn f_get_grid_track_repetition<'local>(
    env: &mut JNIEnv<'local>,
    base: &JObject<'local>,
    field: &str,
) -> GridTrackRepetition {
    let value = f_get_value(env, base, field, "Lcom/dioxuslabs/taffy/geom/grid/TaffyGridTrackRepetition;");

    get_grid_track_repetition(env, value)
}
