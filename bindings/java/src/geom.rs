use jni::JNIEnv;
use jni::objects::{JObject, JValueOwned};
use taffy::{Line, MaxTrackSizingFunction, MinTrackSizingFunction, NonRepeatedTrackSizingFunction, Point, Rect, Size};
use crate::conversions::{f_get_value};
use crate::measure::{get_max_track_sizing_function, get_min_track_sizing_function};

/// Get a Point from its Java counterpart, using a JValueOwned
pub fn get_point<'local, T, F>(env: &mut JNIEnv<'local>, value: JValueOwned<'local>, f: F, def: fn() -> Point<T>) -> Point<T>
where
    F: Fn(&mut JNIEnv<'local>, JValueOwned<'local>) -> T,
{
    let point_object = &value.l().unwrap();
    if point_object.is_null() {
        return def();
    }

    let x_field = f_get_value(env, point_object, "x", "Ljava/lang/Object;");
    let y_field = f_get_value(env, point_object, "y", "Ljava/lang/Object;");

    Point {
        x: f(env, x_field),
        y: f(env, y_field),
    }
}

/// Get a Rect from its Java counterpart, using a JValueOwned
pub fn get_rect<'local, T, F>(env: &mut JNIEnv<'local>, value: JValueOwned<'local>, f: F, def: fn() -> Rect<T>) -> Rect<T>
where
    F: Fn(&mut JNIEnv<'local>, JValueOwned<'local>) -> T,
{
    let rect_object = &value.l().unwrap();
    if rect_object.is_null() {
        return def();
    }

    let l_field = f_get_value(env, rect_object, "left", "Ljava/lang/Object;");
    let r_field = f_get_value(env, rect_object, "right", "Ljava/lang/Object;");
    let t_field = f_get_value(env, rect_object, "top", "Ljava/lang/Object;");
    let b_field = f_get_value(env, rect_object, "bottom", "Ljava/lang/Object;");


    Rect {
        left: f(env, l_field),
        right: f(env, r_field),
        top: f(env, t_field),
        bottom: f(env, b_field),
    }
}

/// Get a Size from its Java counterpart, using a JValueOwned
pub fn get_size<'local, T, F>(env: &mut JNIEnv<'local>, value: JValueOwned<'local>, f: F, def: fn() -> Size<T>) -> Size<T>
where
    F: Fn(&mut JNIEnv<'local>, JValueOwned<'local>) -> T,
{
    let size_object = &value.l().unwrap();
    if size_object.is_null() {
        return def();
    }

    let w_field = f_get_value(env, size_object, "width", "Ljava/lang/Object;");
    let h_field = f_get_value(env, size_object, "height", "Ljava/lang/Object;");

    Size {
        width: f(env, w_field),
        height: f(env, h_field),
    }
}

/// Get a Line from its Java counterpart, using a JValueOwned
pub fn get_line<'local, T, F>(env: &mut JNIEnv<'local>, value: JValueOwned<'local>, f: F, def: fn() -> Line<T>) -> Line<T>
where
    F: Fn(&mut JNIEnv<'local>, JValueOwned<'local>) -> T,
{
    let line_object = &value.l().unwrap();
    if line_object.is_null() {
        return def();
    }

    let s_field = f_get_value(env, line_object, "start", "Ljava/lang/Object;");
    let e_field = f_get_value(env, line_object, "end", "Ljava/lang/Object;");

    Line {
        start: f(env, s_field),
        end: f(env, e_field),
    }
}

/// Get a NonRepeatedTrackSizingFunction from its Java counterpart, using a JValueOwned
pub fn get_non_repeated_track_sizing_function<'local>(
    env: &mut JNIEnv<'local>,
    value: JValueOwned<'local>
) -> NonRepeatedTrackSizingFunction {
    let nrtsf_object = &value.l().unwrap();
    if nrtsf_object.is_null() {
        panic!("NonRepeatedTrackSizingFunction cannot be null");
    }

    let n_val = f_get_value(env, nrtsf_object, "min", "Lcom/dioxuslabs/taffy/geom/grid/TaffyMinTrackSizingFunction;");
    let x_val = f_get_value(env, nrtsf_object, "max", "Lcom/dioxuslabs/taffy/geom/grid/TaffyMaxTrackSizingFunction;");

    let min = get_min_track_sizing_function(env, n_val, || MinTrackSizingFunction::Auto);
    let max = get_max_track_sizing_function(env, x_val, || MaxTrackSizingFunction::Auto);

    NonRepeatedTrackSizingFunction {
        min,
        max,
    }
}

/// Get an Option<NonRepeatedTrackSizingFunction> from its Java counterpart, using a JValueOwned
pub fn get_opt_non_repeated_track_sizing_function<'local>(
    env: &mut JNIEnv<'local>,
    value: JValueOwned<'local>
) -> Option<NonRepeatedTrackSizingFunction> {
    let nrtsf_object = &value.l().unwrap();
    if nrtsf_object.is_null() {
        return None
    }

    let n_val = f_get_value(env, nrtsf_object, "min", "Lcom/dioxuslabs/taffy/geom/grid/TaffyMinTrackSizingFunction;");
    let x_val = f_get_value(env, nrtsf_object, "max", "Lcom/dioxuslabs/taffy/geom/grid/TaffyMaxTrackSizingFunction;");

    let min = get_min_track_sizing_function(env, n_val, || MinTrackSizingFunction::Auto);
    let max = get_max_track_sizing_function(env, x_val, || MaxTrackSizingFunction::Auto);

    Some(NonRepeatedTrackSizingFunction {
        min,
        max,
    })
}

/// Get a Point from its Java counterpart, using a field name
pub fn f_get_point<'local, T, F>(env: &mut JNIEnv<'local>, base: &JObject<'local>, field: &str, f: F, def: fn() -> Point<T>) -> Point<T>
where
    F: Fn(&mut JNIEnv<'local>, JValueOwned<'local>) -> T,
{
    let point_field = f_get_value(env, base, field, "Lcom/dioxuslabs/taffy/geom/TaffyPoint;");

    get_point(env, point_field, f, def)
}

/// Get a Rect from its Java counterpart, using a field name
pub fn f_get_rect<'local, T, F>(env: &mut JNIEnv<'local>, base: &JObject<'local>, field: &str, f: F, def: fn() -> Rect<T>) -> Rect<T>
where
    F: Fn(&mut JNIEnv<'local>, JValueOwned<'local>) -> T,
{
    let point_field = f_get_value(env, base, field, "Lcom/dioxuslabs/taffy/geom/TaffyRect;");

    get_rect(env, point_field, f, def)
}

/// Get a Size from its Java counterpart, using a field name
pub fn f_get_size<'local, T, F>(env: &mut JNIEnv<'local>, base: &JObject<'local>, field: &str, f: F, def: fn() -> Size<T>) -> Size<T>
where
    F: Fn(&mut JNIEnv<'local>, JValueOwned<'local>) -> T,
{
    let point_field = f_get_value(env, base, field, "Lcom/dioxuslabs/taffy/geom/TaffySize;");

    get_size(env, point_field, f, def)
}

/// Get a Line from its Java counterpart, using a field name
pub fn f_get_line<'local, T, F>(env: &mut JNIEnv<'local>, base: &JObject<'local>, field: &str, f: F, def: fn() -> Line<T>) -> Line<T>
where
    F: Fn(&mut JNIEnv<'local>, JValueOwned<'local>) -> T,
{
    let point_field = f_get_value(env, base, field, "Lcom/dioxuslabs/taffy/geom/TaffyLine;");

    get_line(env, point_field, f, def)
}

/// Get a NonRepeatedTrackSizingFunction from its Java counterpart, using a field name
pub fn f_get_non_repeated_track_sizing_function<'local>(env: &mut JNIEnv<'local>, base: &JObject<'local>, field: &str) -> NonRepeatedTrackSizingFunction {
    let nrtsft_field = f_get_value(env, base, field, "Lcom/dioxuslabs/taffy/geom/grid/TaffyNonRepeatedTrackSizingFunction;");

    get_non_repeated_track_sizing_function(env, nrtsft_field)
}