mod conversions;

use std::panic;
// This is the interface to the JVM that we'll call the majority of our
// methods on.
use jni::JNIEnv;

// These objects are what you should use as arguments to your native
// function. They carry extra lifetime information to prevent them escaping
// this context and getting used after being GC'd.
use jni::objects::{JClass, JObject};

// This is just a pointer. We'll be returning it from our function. We
// can't return one of the objects with lifetime information because the
// lifetime checker won't let us.
use jni::sys::{jint, jlong, jobject};

use taffy::{NodeId, TaffyTree, TraversePartialTree};

use conversions::get_style;

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
        res.map_or(-1, |v| v.0 as jlong) as jlong
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dioxuslabs_taffy_TaffyTree_childCount<'local>(_env: JNIEnv<'local>,
                                                                              _class: JClass<'local>,
                                                                              pointer: jlong,
                                                                              node_id: jlong)
                                                                              -> jint {
    let result = panic::catch_unwind(|| {
        let raw_ptr: *mut TaffyTree<()> = pointer as *mut TaffyTree<()>;
        if raw_ptr.is_null() {
            return -2;
        }
        let tree: &mut TaffyTree<()> = unsafe { &mut *raw_ptr };
        let res = tree.child_count(NodeId::from(node_id as u64));
        res as jint
    });

    result.unwrap_or_else(|_| -9)
}
