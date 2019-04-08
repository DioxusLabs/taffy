#![cfg(target_os = "android")]
#![allow(non_snake_case)]

use jni::objects::JObject;
use jni::sys::{jboolean, jfloatArray, jint, jlong, jlongArray};
use jni::JNIEnv;

#[no_mangle]
pub unsafe extern "C" fn Java_app_visly_stretch_Node_nConstruct(
    env: JNIEnv,
    instance: JObject,
    style: jfloatArray,
    children: jlongArray,
) -> jlong {
    return 0;
}

#[no_mangle]
pub unsafe extern "C" fn Java_app_visly_stretch_Node_nConstructLeaf(
    env: JNIEnv,
    instance: JObject,
    style: jfloatArray,
    measure: JObject,
) -> jlong {
    return 0;
}

#[no_mangle]
pub unsafe extern "C" fn Java_app_visly_stretch_Node_nSetChildren(
    env: JNIEnv,
    instance: JObject,
    ptr: jlong,
    children: jlongArray,
) {
}

#[no_mangle]
pub unsafe extern "C" fn Java_app_visly_stretch_Node_nAddChild(
    env: JNIEnv,
    instance: JObject,
    ptr: jlong,
    child: jlong,
) {
}

#[no_mangle]
pub unsafe extern "C" fn Java_app_visly_stretch_Node_nReplaceChildAtIndex(
    env: JNIEnv,
    instance: JObject,
    ptr: jlong,
    index: jint,
    child: jlong,
) -> jlong {
    return 0;
}

#[no_mangle]
pub unsafe extern "C" fn Java_app_visly_stretch_Node_nRemoveChild(
    env: JNIEnv,
    instance: JObject,
    ptr: jlong,
    child: jlong,
) -> jlong {
    return 0;
}

#[no_mangle]
pub unsafe extern "C" fn Java_app_visly_stretch_Node_nRemoveChildAtIndex(
    env: JNIEnv,
    instance: JObject,
    ptr: jlong,
    index: jint,
) -> jlong {
    return 0;
}

#[no_mangle]
pub unsafe extern "C" fn Java_app_visly_stretch_Node_nSetStyle(
    env: JNIEnv,
    instance: JObject,
    ptr: jlong,
    args: jfloatArray,
) -> jboolean {
    return 0;
}

#[no_mangle]
pub unsafe extern "C" fn Java_app_visly_stretch_Node_nIsDirty(env: JNIEnv, instance: JObject, ptr: jlong) -> jboolean {
    return 0;
}

#[no_mangle]
pub unsafe extern "C" fn Java_app_visly_stretch_Node_nMarkDirty(env: JNIEnv, instance: JObject, ptr: jlong) {}

#[no_mangle]
pub unsafe extern "C" fn Java_app_visly_stretch_Node_nComputeLayput(
    env: JNIEnv,
    instance: JObject,
    ptr: jlong,
    args: jfloatArray,
) -> jfloatArray {
    return env.new_float_array(0).unwrap();
}
