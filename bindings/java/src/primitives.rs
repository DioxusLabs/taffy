use crate::conversions::f_get_value;
use jni::objects::{JObject, JValueOwned};
use jni::JNIEnv;
use taffy::NodeId;

/// Convert Java float to f32
pub fn f32_from_primitive<'local>(_env: &mut JNIEnv<'local>, value: JValueOwned<'local>, def: fn() -> f32) -> f32 {
    value.f().unwrap_or(def())
}

/// Convert Java Float object to f32
#[allow(dead_code)]
pub fn f32_from_object<'local>(env: &mut JNIEnv<'local>, value: JValueOwned<'local>, def: fn() -> f32) -> f32 {
    let object = value.l().unwrap();

    if object.is_null() {
        return def();
    }

    env.call_method(object, "floatValue", "()F", &[]).unwrap().f().unwrap() as f32
}

/// Convert Java Float object to Option<f32>
pub fn opt_f32_from_object<'local>(
    env: &mut JNIEnv<'local>,
    value: JValueOwned<'local>,
    def: fn() -> Option<f32>,
) -> Option<f32> {
    let object = value.l().unwrap();

    if object.is_null() {
        return def();
    }

    Some(env.call_method(object, "floatValue", "()F", &[]).unwrap().f().unwrap())
}

/// Convert Java byte to i8
pub fn i8_from_primitive<'local>(_env: &mut JNIEnv<'local>, value: JValueOwned<'local>, def: fn() -> i8) -> i8 {
    value.b().unwrap_or(def()) as i8
}

/// Convert Java Byte object to i8
#[allow(dead_code)]
pub fn i8_from_object<'local>(env: &mut JNIEnv<'local>, value: JValueOwned<'local>, def: fn() -> i8) -> i8 {
    let object = value.l().unwrap();

    if object.is_null() {
        return def();
    }

    env.call_method(object, "byteValue", "()B", &[]).unwrap().b().unwrap() as i8
}

/// Convert Java short to i16
pub fn i16_from_primitive<'local>(_env: &mut JNIEnv<'local>, value: JValueOwned<'local>, def: fn() -> i16) -> i16 {
    value.s().unwrap_or(def()) as i16
}

/// Convert Java Short object to i16
#[allow(dead_code)]
pub fn i16_from_object<'local>(env: &mut JNIEnv<'local>, value: JValueOwned<'local>, def: fn() -> i16) -> i16 {
    let object = value.l().unwrap();

    if object.is_null() {
        return def();
    }

    env.call_method(object, "shortValue", "()S", &[]).unwrap().s().unwrap() as i16
}

/// Convert Java int to i32
pub fn i32_from_primitive<'local>(_env: &mut JNIEnv<'local>, value: JValueOwned<'local>, def: fn() -> i32) -> i32 {
    value.i().unwrap_or(def()) as i32
}

/// Convert Java Integer object to i32
#[allow(dead_code)]
pub fn i32_from_object<'local>(env: &mut JNIEnv<'local>, value: JValueOwned<'local>, def: fn() -> i32) -> i32 {
    let object = value.l().unwrap();

    if object.is_null() {
        return def();
    }

    env.call_method(object, "intValue", "()I", &[]).unwrap().i().unwrap() as i32
}

/// Convert Java long to i64
#[allow(dead_code)]
pub fn i64_from_primitive<'local>(_env: &mut JNIEnv<'local>, value: JValueOwned<'local>, def: fn() -> i64) -> i64 {
    value.j().unwrap_or(def()) as i64
}

/// Convert Java Long object to i64
#[allow(dead_code)]
pub fn i64_from_object<'local>(env: &mut JNIEnv<'local>, value: JValueOwned<'local>, def: fn() -> i64) -> i64 {
    let object = value.l().unwrap();

    if object.is_null() {
        return def();
    }

    env.call_method(object, "longValue", "()J", &[]).unwrap().j().unwrap() as i64
}

/// Convert Java Long object to Option<i64>
pub fn opt_i64_from_object<'local>(env: &mut JNIEnv<'local>, value: JValueOwned<'local>) -> Option<i64> {
    let object = value.l().unwrap();

    if object.is_null() {
        return None;
    }

    Some(env.call_method(object, "longValue", "()J", &[]).unwrap().j().unwrap() as i64)
}

/// Convert Java boolean to bool
pub fn bool_from_primitive<'local>(_env: &mut JNIEnv<'local>, value: JValueOwned<'local>, def: fn() -> bool) -> bool {
    value.z().unwrap_or(def())
}

/// Convert Java Boolean object to bool
#[allow(dead_code)]
pub fn bool_from_object<'local>(env: &mut JNIEnv<'local>, value: JValueOwned<'local>, def: fn() -> bool) -> bool {
    let object = value.l().unwrap();

    if object.is_null() {
        return def();
    }

    env.call_method(object, "booleanValue", "()Z", &[]).unwrap().z().unwrap()
}

/// Convert Java Boolean object to Option<bool>
#[allow(dead_code)]
pub fn opt_bool_from_object<'local>(
    env: &mut JNIEnv<'local>,
    value: JValueOwned<'local>,
    def: fn() -> Option<bool>,
) -> Option<bool> {
    let object = value.l().unwrap();

    if object.is_null() {
        return def();
    }

    Some(env.call_method(object, "booleanValue", "()Z", &[]).unwrap().z().unwrap())
}

/// Convert Java long to NodeId
#[allow(dead_code)]
pub fn node_id_from_primitive<'local>(env: &mut JNIEnv<'local>, value: JValueOwned<'local>) -> NodeId {
    NodeId::from(i64_from_primitive(env, value, || 0) as u64)
}

/// Convert Java Long to NodeId
#[allow(dead_code)]
pub fn node_id_from_object<'local>(env: &mut JNIEnv<'local>, value: JValueOwned<'local>) -> NodeId {
    NodeId::from(i64_from_object(env, value, || 0) as u64)
}

/// Convert Java Long to Option<NodeId>
pub fn opt_node_id_from_object<'local>(env: &mut JNIEnv<'local>, value: JValueOwned<'local>) -> Option<NodeId> {
    let o = opt_i64_from_object(env, value);
    if o.is_none() {
        None
    } else {
        Some(NodeId::from(o? as u64))
    }
}

/// Convert Java float to f32, using a field name
pub fn f_f32_from_primitive<'local>(
    env: &mut JNIEnv<'local>,
    base: &JObject<'local>,
    field: &str,
    def: fn() -> f32,
) -> f32 {
    let value = f_get_value(env, base, field, "F");

    f32_from_primitive(env, value, def)
}

/// Convert Java float to f32, using a field name
pub fn f_opt_f32_from_object<'local>(
    env: &mut JNIEnv<'local>,
    base: &JObject<'local>,
    field: &str,
    def: fn() -> Option<f32>,
) -> Option<f32> {
    let value = f_get_value(env, base, field, "Ljava/lang/Float;");

    opt_f32_from_object(env, value, def)
}

/// Convert Java byte to i8, using a field name
pub fn f_i8_from_primitive<'local>(
    env: &mut JNIEnv<'local>,
    base: &JObject<'local>,
    field: &str,
    def: fn() -> i8,
) -> i8 {
    let value = f_get_value(env, base, field, "B");

    i8_from_primitive(env, value, def)
}

/// Convert Java short to i16, using a field name
pub fn f_i16_from_primitive<'local>(
    env: &mut JNIEnv<'local>,
    base: &JObject<'local>,
    field: &str,
    def: fn() -> i16,
) -> i16 {
    let value = f_get_value(env, base, field, "B");

    i16_from_primitive(env, value, def)
}

/// Convert Java int to i32, using a field name
pub fn f_i32_from_primitive<'local>(
    env: &mut JNIEnv<'local>,
    base: &JObject<'local>,
    field: &str,
    def: fn() -> i32,
) -> i32 {
    let value = f_get_value(env, base, field, "I");

    i32_from_primitive(env, value, def)
}

/// Convert Java long to i64, using a field name
#[allow(dead_code)]
pub fn f_i64_from_primitive<'local>(
    env: &mut JNIEnv<'local>,
    base: &JObject<'local>,
    field: &str,
    def: fn() -> i64,
) -> i64 {
    let value = f_get_value(env, base, field, "J");

    i64_from_primitive(env, value, def)
}

/// Convert Java boolean to bool, using a field name
pub fn f_bool_from_primitive<'local>(
    env: &mut JNIEnv<'local>,
    base: &JObject<'local>,
    field: &str,
    def: fn() -> bool,
) -> bool {
    let value = f_get_value(env, base, field, "Z");

    bool_from_primitive(env, value, def)
}
