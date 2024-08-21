mod collections;
mod conversions;
mod enums;
mod geom;
mod java;
mod measure;
mod primitives;

use crate::collections::get_list;
use crate::conversions::get_style;
use crate::geom::get_size;
use crate::java::to_java_layout;
use crate::measure::get_available_space;
use crate::primitives::{node_id_from_primitive, opt_node_id_from_object};
use jni::objects::{JClass, JObject, JValueOwned};
use jni::sys::{jint, jlong, jobject};
use jni::JNIEnv;
use std::panic;
use taffy::{NodeId, TaffyTree, TraversePartialTree};

#[no_mangle]
pub extern "system" fn Java_com_dioxuslabs_taffy_TaffyTree_nvNewTaffyTree<'local>(
    _env: JNIEnv<'local>,
    _class: JClass<'local>,
) -> jlong {
    let tree: Box<TaffyTree<()>> = Box::new(TaffyTree::new());
    Box::into_raw(tree) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dioxuslabs_taffy_TaffyTree_nvNewTaffyTreeWithCapacity<'local>(
    _env: JNIEnv<'local>,
    _class: JClass<'local>,
    capacity: jint,
) -> jlong {
    let tree: Box<TaffyTree<()>> = Box::new(TaffyTree::with_capacity(capacity as usize));
    Box::into_raw(tree) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dioxuslabs_taffy_TaffyTree_nvEnableRounding<'local>(
    _env: JNIEnv<'local>,
    _class: JClass<'local>,
    pointer: jlong,
) {
    unsafe {
        let raw_ptr: *mut TaffyTree<()> = pointer as *mut TaffyTree<()>;
        if raw_ptr.is_null() {
            return;
        }
        let tree: &mut TaffyTree<()> = &mut *raw_ptr;
        tree.enable_rounding()
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dioxuslabs_taffy_TaffyTree_nvDisableRounding<'local>(
    _env: JNIEnv<'local>,
    _class: JClass<'local>,
    pointer: jlong,
) {
    unsafe {
        let raw_ptr: *mut TaffyTree<()> = pointer as *mut TaffyTree<()>;
        if raw_ptr.is_null() {
            return;
        }
        let tree: &mut TaffyTree<()> = &mut *raw_ptr;
        tree.disable_rounding()
    }
}

#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "system" fn Java_com_dioxuslabs_taffy_TaffyTree_nvNewLeaf<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    pointer: jlong,
    style: jobject,
) -> jlong {
    unsafe {
        let style = get_style(&mut env, &JObject::from_raw(style));

        let raw_ptr: *mut TaffyTree<()> = pointer as *mut TaffyTree<()>;
        let tree: &mut TaffyTree<()> = &mut *raw_ptr;
        let res = tree.new_leaf(style);
        res.map_or(-1, |v| <NodeId as Into<u64>>::into(v) as jlong) as jlong
    }
}

#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "system" fn Java_com_dioxuslabs_taffy_TaffyTree_nvNewWithChildren<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    pointer: jlong,
    style: jobject,
    children: jobject,
) -> jlong {
    unsafe {
        let style = get_style(&mut env, &JObject::from_raw(style));
        let list = JValueOwned::from(JObject::from_raw(children));
        let children = &get_list(&mut env, list, opt_node_id_from_object);

        let raw_ptr: *mut TaffyTree<()> = pointer as *mut TaffyTree<()>;
        let tree: &mut TaffyTree<()> = &mut *raw_ptr;
        let res = tree.new_with_children(style, children);
        res.map_or(-1, |v| <NodeId as Into<u64>>::into(v) as jlong) as jlong
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dioxuslabs_taffy_TaffyTree_nvChildCount<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    pointer: jlong,
    node: jlong,
) -> jint {
    let node = node_id_from_primitive(&mut env, JValueOwned::from(node));
    let raw_ptr: *mut TaffyTree<()> = pointer as *mut TaffyTree<()>;
    if raw_ptr.is_null() {
        return -2;
    }
    let tree: &mut TaffyTree<()> = unsafe { &mut *raw_ptr };
    tree.child_count(node) as jint
}

#[no_mangle]
pub extern "system" fn Java_com_dioxuslabs_taffy_TaffyTree_nvClear<'local>(
    _env: JNIEnv<'local>,
    _class: JClass<'local>,
    pointer: jlong,
) {
    unsafe {
        let raw_ptr: *mut TaffyTree<()> = pointer as *mut TaffyTree<()>;
        if raw_ptr.is_null() {
            return;
        }
        let tree: &mut TaffyTree<()> = &mut *raw_ptr;
        tree.clear()
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dioxuslabs_taffy_TaffyTree_nvRemove<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    pointer: jlong,
    node: jlong,
) -> jlong {
    unsafe {
        let raw_ptr: *mut TaffyTree<()> = pointer as *mut TaffyTree<()>;
        let tree: &mut TaffyTree<()> = &mut *raw_ptr;

        let node = node_id_from_primitive(&mut env, JValueOwned::from(node));
        let res = tree.remove(node);

        res.map_or(-1, |v| <NodeId as Into<u64>>::into(v) as jlong) as jlong
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dioxuslabs_taffy_TaffyTree_nvAddChild<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    pointer: jlong,
    parent: jlong,
    child: jlong,
) {
    unsafe {
        let raw_ptr: *mut TaffyTree<()> = pointer as *mut TaffyTree<()>;
        let tree: &mut TaffyTree<()> = &mut *raw_ptr;

        let parent_node = node_id_from_primitive(&mut env, JValueOwned::from(parent));
        let child_node = node_id_from_primitive(&mut env, JValueOwned::from(child));
        tree.add_child(parent_node, child_node).expect("Failed to add child");
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dioxuslabs_taffy_TaffyTree_nvInsertChildAtIndex<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    pointer: jlong,
    parent: jlong,
    child_index: jint,
    child: jlong,
) {
    unsafe {
        let raw_ptr: *mut TaffyTree<()> = pointer as *mut TaffyTree<()>;
        let tree: &mut TaffyTree<()> = &mut *raw_ptr;

        let parent_node = node_id_from_primitive(&mut env, JValueOwned::from(parent));
        let child_node = node_id_from_primitive(&mut env, JValueOwned::from(child));
        tree.insert_child_at_index(parent_node, child_index as usize, child_node).expect("Failed to insert child");
    }
}

#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "system" fn Java_com_dioxuslabs_taffy_TaffyTree_nvSetChildren<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    pointer: jlong,
    parent: jlong,
    children: jobject,
) {
    unsafe {
        let raw_ptr: *mut TaffyTree<()> = pointer as *mut TaffyTree<()>;
        let tree: &mut TaffyTree<()> = &mut *raw_ptr;

        let parent_node = node_id_from_primitive(&mut env, JValueOwned::from(parent));

        let list = JValueOwned::from(JObject::from_raw(children));
        let children = &get_list(&mut env, list, opt_node_id_from_object);

        tree.set_children(parent_node, children).expect("Failed to set children");
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dioxuslabs_taffy_TaffyTree_nvRemoveChild<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    pointer: jlong,
    parent: jlong,
    child: jlong,
) -> jlong {
    unsafe {
        let raw_ptr: *mut TaffyTree<()> = pointer as *mut TaffyTree<()>;
        let tree: &mut TaffyTree<()> = &mut *raw_ptr;

        let parent_node = node_id_from_primitive(&mut env, JValueOwned::from(parent));
        let child_node = node_id_from_primitive(&mut env, JValueOwned::from(child));

        let res = tree.remove_child(parent_node, child_node);
        res.map_or(-1, |v| <NodeId as Into<u64>>::into(v) as jlong) as jlong
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dioxuslabs_taffy_TaffyTree_nvRemoveChildAtIndex<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    pointer: jlong,
    parent: jlong,
    child_index: jint,
) -> jlong {
    unsafe {
        let raw_ptr: *mut TaffyTree<()> = pointer as *mut TaffyTree<()>;
        let tree: &mut TaffyTree<()> = &mut *raw_ptr;

        let parent_node = node_id_from_primitive(&mut env, JValueOwned::from(parent));

        let res = tree.remove_child_at_index(parent_node, child_index as usize);
        res.map_or(-1, |v| <NodeId as Into<u64>>::into(v) as jlong) as jlong
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dioxuslabs_taffy_TaffyTree_nvReplaceChildAtIndex<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    pointer: jlong,
    parent: jlong,
    child_index: jint,
    new_child: jlong,
) -> jlong {
    unsafe {
        let raw_ptr: *mut TaffyTree<()> = pointer as *mut TaffyTree<()>;
        let tree: &mut TaffyTree<()> = &mut *raw_ptr;

        let parent_node = node_id_from_primitive(&mut env, JValueOwned::from(parent));
        let new_child_node = node_id_from_primitive(&mut env, JValueOwned::from(new_child));

        let res = tree.replace_child_at_index(parent_node, child_index as usize, new_child_node);
        res.map_or(-1, |v| <NodeId as Into<u64>>::into(v) as jlong) as jlong
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dioxuslabs_taffy_TaffyTree_nvChildAtIndex<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    pointer: jlong,
    parent: jlong,
    child_index: jint,
) -> jlong {
    unsafe {
        let raw_ptr: *mut TaffyTree<()> = pointer as *mut TaffyTree<()>;
        let tree: &mut TaffyTree<()> = &mut *raw_ptr;

        let parent_node = node_id_from_primitive(&mut env, JValueOwned::from(parent));

        let res = tree.child_at_index(parent_node, child_index as usize);
        res.map_or(-1, |v| <NodeId as Into<u64>>::into(v) as jlong) as jlong
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dioxuslabs_taffy_TaffyTree_nvTotalNodeCount<'local>(
    _env: JNIEnv<'local>,
    _class: JClass<'local>,
    pointer: jlong,
) -> jint {
    unsafe {
        let raw_ptr: *mut TaffyTree<()> = pointer as *mut TaffyTree<()>;
        let tree: &mut TaffyTree<()> = &mut *raw_ptr;

        tree.total_node_count() as jint
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dioxuslabs_taffy_TaffyTree_nvParent<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    pointer: jlong,
    child_id: jlong,
) -> jlong {
    unsafe {
        let raw_ptr: *mut TaffyTree<()> = pointer as *mut TaffyTree<()>;
        let tree: &mut TaffyTree<()> = &mut *raw_ptr;

        let child_node = node_id_from_primitive(&mut env, JValueOwned::from(child_id));

        let res = tree.parent(child_node);
        res.map_or(-1, |v| <NodeId as Into<u64>>::into(v) as jlong) as jlong
    }
}

#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "system" fn Java_com_dioxuslabs_taffy_TaffyTree_nvSetStyle<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    pointer: jlong,
    node: jlong,
    style: jobject,
) {
    unsafe {
        let raw_ptr: *mut TaffyTree<()> = pointer as *mut TaffyTree<()>;
        let tree: &mut TaffyTree<()> = &mut *raw_ptr;

        let node = node_id_from_primitive(&mut env, JValueOwned::from(node));

        let style = get_style(&mut env, &JObject::from_raw(style));

        tree.set_style(node, style).expect("Failed to set children");
    }
}

#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "system" fn Java_com_dioxuslabs_taffy_TaffyTree_computeLayout<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    pointer: jlong,
    node: jlong,
    available_size: jobject,
) {
    unsafe {
        let node = node_id_from_primitive(&mut env, JValueOwned::from(node));
        let as_object = JValueOwned::from(JObject::from_raw(available_size));
        let available_size =
            get_size(&mut env, as_object, get_available_space, || panic!("AvailableSize cannot be null"));

        let raw_ptr: *mut TaffyTree<()> = pointer as *mut TaffyTree<()>;
        if raw_ptr.is_null() {
            return;
        }
        let tree: &mut TaffyTree<()> = &mut *raw_ptr;
        tree.compute_layout(node, available_size).expect("Failed to compute layout");
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dioxuslabs_taffy_TaffyTree_nvPrintTree<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    pointer: jlong,
    node: jlong,
) {
    unsafe {
        let raw_ptr: *mut TaffyTree<()> = pointer as *mut TaffyTree<()>;
        if raw_ptr.is_null() {
            return;
        }
        let tree: &mut TaffyTree<()> = &mut *raw_ptr;
        let node = node_id_from_primitive(&mut env, JValueOwned::from(node));
        tree.print_tree(node)
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dioxuslabs_taffy_TaffyTree_layout<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    pointer: jlong,
    node: jlong,
) -> jobject {
    let node = node_id_from_primitive(&mut env, JValueOwned::from(node));
    let raw_ptr: *mut TaffyTree<()> = pointer as *mut TaffyTree<()>;
    if raw_ptr.is_null() {
        panic!("Pointer cannot be null");
    }
    let tree: &mut TaffyTree<()> = unsafe { &mut *raw_ptr };
    let res = tree.layout(node);
    to_java_layout(&mut env, res.unwrap())
}
