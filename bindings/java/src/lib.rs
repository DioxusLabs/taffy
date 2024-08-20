mod conversions;
mod collections;
mod primitives;
mod geom;
mod enums;
mod measure;
mod java;

use std::panic;
use jni::JNIEnv;
use jni::objects::{JClass, JObject, JValueOwned};
use jni::sys::{jint, jlong, jobject};
use taffy::{NodeId, TaffyTree, TraversePartialTree};
use crate::collections::{get_list};
use crate::conversions::get_style;
use crate::geom::get_size;
use crate::java::to_java_layout;
use crate::measure::get_available_space;
use crate::primitives::{node_id_from_primitive, opt_node_id_from_object};

#[no_mangle]
pub extern "system" fn Java_com_dioxuslabs_taffy_TaffyTree_newTaffyTree<'local>(_env: JNIEnv<'local>,
                                                                                _class: JClass<'local>)
                                                                                -> jlong {
    let tree: Box<TaffyTree<()>> = Box::new(TaffyTree::new());
    Box::into_raw(tree) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dioxuslabs_taffy_TaffyTree_newLeaf<'local>(mut env: JNIEnv<'local>,
                                                                           _class: JClass<'local>,
                                                                           pointer: jlong,
                                                                           style: jobject)
                                                                           -> jlong {
    unsafe {
        let style = get_style(&mut env, &JObject::from_raw(style));

        let raw_ptr: *mut TaffyTree<()> = pointer as *mut TaffyTree<()>;
        if raw_ptr.is_null() {
            return -2;
        }
        let tree: &mut TaffyTree<()> = &mut *raw_ptr;
        let res = tree.new_leaf(style);
        res.map_or(-1, |v| <NodeId as Into<u64>>::into(v) as jlong) as jlong
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dioxuslabs_taffy_TaffyTree_newWithChildren<'local>(mut env: JNIEnv<'local>,
                                                                                   _class: JClass<'local>,
                                                                                   pointer: jlong,
                                                                                   style: jobject,
                                                                                   children: jobject)
                                                                                   -> jlong {
    unsafe {
        let style = get_style(&mut env, &JObject::from_raw(style));
        let list = JValueOwned::from(JObject::from_raw(children));
        let children = &get_list(&mut env, list, opt_node_id_from_object);

        let raw_ptr: *mut TaffyTree<()> = pointer as *mut TaffyTree<()>;
        if raw_ptr.is_null() {
            return -2;
        }
        let tree: &mut TaffyTree<()> = &mut *raw_ptr;
        let res = tree.new_with_children(style, children);
        res.map_or(-1, |v| <NodeId as Into<u64>>::into(v) as jlong) as jlong
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dioxuslabs_taffy_TaffyTree_computeLayout<'local>(mut env: JNIEnv<'local>,
                                                                                 _class: JClass<'local>,
                                                                                 pointer: jlong,
                                                                                 node: jlong,
                                                                                 available_size: jobject) {
    unsafe {
        let node = node_id_from_primitive(&mut env, JValueOwned::from(node));
        let as_object = JValueOwned::from(JObject::from_raw(available_size));
        let available_size = get_size(&mut env, as_object, get_available_space,
                                      || panic!("AvailableSize cannot be null"));

        let raw_ptr: *mut TaffyTree<()> = pointer as *mut TaffyTree<()>;
        if raw_ptr.is_null() {
            return;
        }
        let tree: &mut TaffyTree<()> = &mut *raw_ptr;
        tree.compute_layout(node, available_size)
            .expect("Failed to compute layout");
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dioxuslabs_taffy_TaffyTree_childCount<'local>(mut env: JNIEnv<'local>,
                                                                              _class: JClass<'local>,
                                                                              pointer: jlong,
                                                                              node: jlong)
                                                                              -> jint {
    let node = node_id_from_primitive(&mut env, JValueOwned::from(node));
    let raw_ptr: *mut TaffyTree<()> = pointer as *mut TaffyTree<()>;
    if raw_ptr.is_null() {
        return -2;
    }
    let tree: &mut TaffyTree<()> = unsafe { &mut *raw_ptr };
    let res = tree.child_count(node);
    res as jint
}
#[no_mangle]
pub extern "system" fn Java_com_dioxuslabs_taffy_TaffyTree_layout<'local>(mut env: JNIEnv<'local>,
                                                                          _class: JClass<'local>,
                                                                          pointer: jlong,
                                                                          node: jlong)
                                                                          -> jobject {
    let node = node_id_from_primitive(&mut env, JValueOwned::from(node));
    let raw_ptr: *mut TaffyTree<()> = pointer as *mut TaffyTree<()>;
    if raw_ptr.is_null() {
        panic!("Pointer cannot be null");
    }
    let tree: &mut TaffyTree<()> = unsafe { &mut *raw_ptr };
    let res = tree.layout(node);
    to_java_layout(&mut env, res.unwrap())
}
