use crate::conversions::f_get_value;
use jni::objects::{JObject, JValue, JValueOwned};
use jni::JNIEnv;
use std::convert::TryInto;

/// Unwraps a Java List into a Rust Vec
pub unsafe fn get_list<'local, T, F>(env: &mut JNIEnv<'local>, value: JValueOwned<'local>, f: F) -> Vec<T>
where
    F: Fn(&mut JNIEnv<'local>, JValueOwned<'local>) -> Option<T>,
{
    let jlist_object = value.l().unwrap();

    if jlist_object.is_null() {
        return Vec::with_capacity(0);
    }

    let list = env.get_list(&jlist_object).unwrap();

    let jlist_size = env.call_method(&list, "size", "()I", &[]).expect("Couldn't call size on List").i().unwrap();

    let mut vec: Vec<T> = Vec::new();
    for i in 0..jlist_size {
        let element = env.call_method(&list, "get", "(I)Ljava/lang/Object;", &[JValue::Int(i)]).unwrap();

        let value = f(env, element);
        if value.is_some() {
            vec.push(value.unwrap());
        }
    }

    vec
}

/// Unwraps a Java List into a Rust array, it basically uses `unwrap_jlist` and then tries to convert the Vec into an array
#[allow(dead_code)]
pub unsafe fn get_array<'local, T, F, const N: usize>(
    env: &mut JNIEnv<'local>,
    value: JValueOwned<'local>,
    f: F,
) -> [T; N]
where
    F: Fn(&mut JNIEnv<'local>, JValueOwned<'local>) -> Option<T>,
{
    let vec = get_list(env, value, f);
    vec.try_into().unwrap_or_else(|v: Vec<T>| panic!("Expected a Vec of length {} but it was {}", N, v.len()))
}

pub unsafe fn f_get_list<'local, T, F>(env: &mut JNIEnv<'local>, base: &JObject<'local>, field: &str, f: F) -> Vec<T>
where
    F: Fn(&mut JNIEnv<'local>, JValueOwned<'local>) -> Option<T>,
{
    let value = f_get_value(env, base, field, "Ljava/util/List;");
    get_list(env, value, f)
}
