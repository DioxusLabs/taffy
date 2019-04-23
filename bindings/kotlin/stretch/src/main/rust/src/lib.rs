#![cfg(target_os = "android")]
#![allow(non_snake_case)]

use std::f32;
use jni::objects::{JObject, JValue};
use jni::sys::{jboolean, jint, jfloat, jlong, jlongArray, jfloatArray};
use jni::JNIEnv;
use stretch::node::*;
use stretch::style::*;
use stretch::result::{Layout};
use stretch::geometry::*;
use stretch::number::*;

fn dimension(t: jint, v: jfloat) -> Dimension {
    match t {
        0 => Dimension::Points(v),
        1 => Dimension::Percent(v),
        2 => Dimension::Undefined,
        3 => Dimension::Auto,
        _ => panic!(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn Java_app_visly_stretch_Style_nConstruct(
    _: JNIEnv,
    _: JObject,

    display: jint,
    positionType: jint,
    direction: jint,
    flexDirection: jint,
    flexWrap: jint,
    overflow: jint,
    alignItems: jint,
    alignSelf: jint,
    alignContent: jint,
    justifyContent: jint,

    positionStartType: jint,
    positionStartValue: jfloat,
    positionEndType: jint,
    positionEndValue: jfloat,
    positionTopType: jint,
    positionTopValue: jfloat,
    positionBottomType: jint,
    positionBottomValue: jfloat,

    marginStartType: jint,
    marginStartValue: jfloat,
    marginEndType: jint,
    marginEndValue: jfloat,
    marginTopType: jint,
    marginTopValue: jfloat,
    marginBottomType: jint,
    marginBottomValue: jfloat,

    paddingStartType: jint,
    paddingStartValue: jfloat,
    paddingEndType: jint,
    paddingEndValue: jfloat,
    paddingTopType: jint,
    paddingTopValue: jfloat,
    paddingBottomType: jint,
    paddingBottomValue: jfloat,

    borderStartType: jint,
    borderStartValue: jfloat,
    borderEndType: jint,
    borderEndValue: jfloat,
    borderTopType: jint,
    borderTopValue: jfloat,
    borderBottomType: jint,
    borderBottomValue: jfloat,

    flexGrow: jfloat,
    flexShrink: jfloat,

    flexBasisType: jint,
    flexBasisValue: jfloat,

    widthType: jint,
    widthValue: jfloat,
    heightType: jint,
    heightValue: jfloat,

    minWidthType: jint,
    minWidthValue: jfloat,
    minHeightType: jint,
    minHeightValue: jfloat,

    maxWidthType: jint,
    maxWidthValue: jfloat,
    maxHeightType: jint,
    maxHeightValue: jfloat,

    aspectRatio: jfloat
) -> jlong {
    let style = Style {
        display: match display {
            0 => Display::Flex,
            1 => Display::None,
            _ => panic!()
        },

        position_type: match positionType {
            0 => PositionType::Relative,
            1 => PositionType::Absolute,
            _ => panic!()
        },

        direction: match direction {
            0 => Direction::Inherit,
            1 => Direction::LTR,
            2 => Direction::RTL,
            _ => panic!()
        },

        flex_direction: match flexDirection {
            0 => FlexDirection::Row,
            1 => FlexDirection::Column,
            2 => FlexDirection::RowReverse,
            3 => FlexDirection::ColumnReverse,
            _ => panic!()
        },

        flex_wrap: match flexWrap {
            0 => FlexWrap::NoWrap,
            1 => FlexWrap::Wrap,
            2 => FlexWrap::WrapReverse,
            _ => panic!()
        },

        overflow: match overflow {
            0 => Overflow::Visible,
            1 => Overflow::Hidden,
            2 => Overflow::Scroll,
            _ => panic!()
        },

        align_items: match alignItems {
            0 => AlignItems::FlexStart,
            1 => AlignItems::FlexEnd,
            2 => AlignItems::Center,
            3 => AlignItems::Baseline,
            4 => AlignItems::Stretch,
            _ => panic!()
        },

        align_self: match alignSelf {
            0 => AlignSelf::Auto,
            1 => AlignSelf::FlexStart,
            2 => AlignSelf::FlexEnd,
            3 => AlignSelf::Center,
            4 => AlignSelf::Baseline,
            5 => AlignSelf::Stretch,
            _ => panic!()
        },

        align_content: match alignContent {
            0 => AlignContent::FlexStart,
            1 => AlignContent::FlexEnd,
            2 => AlignContent::Center,
            3 => AlignContent::Stretch,
            4 => AlignContent::SpaceBetween,
            5 => AlignContent::SpaceAround,
            _ => panic!()
        },

        justify_content: match justifyContent {
            0 => JustifyContent::FlexStart,
            1 => JustifyContent::FlexEnd,
            2 => JustifyContent::Center,
            3 => JustifyContent::SpaceBetween,
            4 => JustifyContent::SpaceAround,
            5 => JustifyContent::SpaceEvenly,
            _ => panic!()
        },

        position: Rect {
            start: dimension(positionStartType, positionStartValue),
            end: dimension(positionEndType, positionEndValue),
            top: dimension(positionTopType, positionTopValue),
            bottom: dimension(positionBottomType, positionBottomValue),
        },

        margin: Rect {
            start: dimension(marginStartType, marginStartValue),
            end: dimension(marginEndType, marginEndValue),
            top: dimension(marginTopType, marginTopValue),
            bottom: dimension(marginBottomType, marginBottomValue),
        },

        padding: Rect {
            start: dimension(paddingStartType, paddingStartValue),
            end: dimension(paddingEndType, paddingEndValue),
            top: dimension(paddingTopType, paddingTopValue),
            bottom: dimension(paddingBottomType, paddingBottomValue),
        },

        border: Rect {
            start: dimension(borderStartType, borderStartValue),
            end: dimension(borderEndType, borderEndValue),
            top: dimension(borderTopType, borderTopValue),
            bottom: dimension(borderBottomType, borderBottomValue),
        },

        flex_grow: flexGrow,
        flex_shrink: flexShrink,

        flex_basis: dimension(flexBasisType, flexBasisValue),

        size: Size {
            width: dimension(widthType, widthValue),
            height: dimension(heightType, heightValue),
        },

        min_size: Size {
            width: dimension(minWidthType, minWidthValue),
            height: dimension(minHeightType, minHeightValue),
        },

        max_size: Size {
            width: dimension(maxWidthType, maxWidthValue),
            height: dimension(maxHeightType, maxHeightValue),
        },

        aspect_ratio: if f32::is_nan(aspectRatio) {
            Number::Undefined
        } else {
            Number::Defined(aspectRatio)
        },
    };

    Box::into_raw(Box::new(style)) as jlong
}

#[no_mangle]
pub unsafe extern "C" fn Java_app_visly_stretch_Style_nFree(
    _: JNIEnv,
    _: JObject,
    style: jlong,
) {
    let _style: Box<Style> = Box::from_raw(style as *mut Style);
}

#[no_mangle]
pub unsafe extern "C" fn Java_app_visly_stretch_Node_nConstruct(
    env: JNIEnv,
    _: JObject,
    style: jlong,
    children: jlongArray,
) -> jlong {
    let num_children = env.get_array_length(children).unwrap();
    let mut buff = vec![0; num_children as usize];
    env.get_long_array_region(children, 0, &mut buff).unwrap();

    let children = buff.into_iter().map(|ptr| {
        &*Box::leak(Box::from_raw(ptr as *mut Node))
    }).collect();

    let style: Box<Style> = Box::from_raw(style as *mut Style);
    let node = Node::new(*Box::leak(style), children);

    Box::into_raw(Box::new(node)) as jlong
}

#[no_mangle]
pub unsafe extern "C" fn Java_app_visly_stretch_Node_nConstructLeaf(
    env: JNIEnv<'static>,
    _: JObject,
    style: jlong,
    measure: JObject,
) -> jlong {
    let measure = env.new_global_ref(measure).unwrap();
    let style: Box<Style> = Box::from_raw(style as *mut Style);
    let node = Node::new_leaf(*Box::leak(style), Box::new(move |constraint| {
        let result = env.call_method(
            measure.as_obj(),
            "measure",
            "(FF)[F",
            &[JValue::from(constraint.width.or_else(f32::NAN)), JValue::from(constraint.height.or_else(f32::NAN))],
        );

        match result {
            Ok(result) => {
                let size = result.l().unwrap().into_inner() as jfloatArray;
                let mut buff: [f32; 2] = [0.0, 0.0];
                env.get_float_array_region(size, 0, &mut buff).unwrap();
                Ok(Size { width: buff[0], height: buff[1] })
            }
            Err(err) => Err(Box::new(err)),
        }
    }));
    Box::into_raw(Box::new(node)) as jlong
}

#[no_mangle]
pub unsafe extern "C" fn Java_app_visly_stretch_Node_nFree(
    _: JNIEnv,
    _: JObject,
    node: jlong,
) {
    let _node: Box<Node> = Box::from_raw(node as *mut Node);
}

#[no_mangle]
pub unsafe extern "C" fn Java_app_visly_stretch_Node_nSetMeasure(
    env: JNIEnv<'static>,
    _: JObject,
    node: jlong,
    measure: JObject,
) {
    let measure = env.new_global_ref(measure).unwrap();
    let node: Box<Node> = Box::from_raw(node as *mut Node);

    Box::leak(node).set_measure(Some(Box::new(move |constraint| {
         let result = env.call_method(
             measure.as_obj(),
             "measure",
             "(FF)[F",
             &[JValue::from(constraint.width.or_else(f32::NAN)), JValue::from(constraint.height.or_else(f32::NAN))],
         );

         match result {
             Ok(result) => {
                 let size = result.l().unwrap().into_inner() as jfloatArray;
                 let mut buff: [f32; 2] = [0.0, 0.0];
                 env.get_float_array_region(size, 0, &mut buff).unwrap();
                 Ok(Size { width: buff[0], height: buff[1] })
             }
             Err(err) => Err(Box::new(err)),
         }
     })));
}

#[no_mangle]
pub unsafe extern "C" fn Java_app_visly_stretch_Node_nSetStyle(
    _: JNIEnv,
    _: JObject,
    node: jlong,
    style: jlong,
) {
    let style: Box<Style> = Box::from_raw(style as *mut Style);
    let mut node: Box<Node> = Box::from_raw(node as *mut Node);
    node.set_style(*style);

    Box::leak(style);
    Box::leak(node);
}

#[no_mangle]
pub unsafe extern "C" fn Java_app_visly_stretch_Node_nIsDirty(
    _: JNIEnv,
    _: JObject,
    node: jlong,
) -> jboolean {
    let node: Box<Node> = Box::from_raw(node as *mut Node);
    Box::leak(node).dirty() as jboolean
}

#[no_mangle]
pub unsafe extern "C" fn Java_app_visly_stretch_Node_nMarkDirty(
    _: JNIEnv,
    _: JObject,
    node: jlong,
) {
    let node: Box<Node> = Box::from_raw(node as *mut Node);
    Box::leak(node).mark_dirty();
}

#[no_mangle]
pub unsafe extern "C" fn Java_app_visly_stretch_Node_nSetChildren(
    env: JNIEnv,
    _: JObject,
    node: jlong,
    children: jlongArray,
) {
    let num_children = env.get_array_length(children).unwrap();
    let mut buff = vec![0; num_children as usize];
    env.get_long_array_region(children, 0, &mut buff).unwrap();

    let children = buff.into_iter().map(|ptr| {
        &*Box::leak(Box::from_raw(ptr as *mut Node))
    }).collect();

    let node: Box<Node> = Box::from_raw(node as *mut Node);
    Box::leak(node).set_children(children);
}

#[no_mangle]
pub unsafe extern "C" fn Java_app_visly_stretch_Node_nAddChild(
    _: JNIEnv,
    _: JObject,
    node: jlong,
    child: jlong,
) {
    let mut node: Box<Node> = Box::from_raw(node as *mut Node);
    let child: Box<Node> = Box::from_raw(child as *mut Node);
    node.add_child(&*child);

    Box::leak(node);
    Box::leak(child);
}

#[no_mangle]
pub unsafe extern "C" fn Java_app_visly_stretch_Node_nReplaceChildAtIndex(
    _: JNIEnv,
    _: JObject,
    node: jlong,
    index: jint,
    child: jlong,
) {
    let mut node: Box<Node> = Box::from_raw(node as *mut Node);
    let child: Box<Node> = Box::from_raw(child as *mut Node);
    node.replace_child_at_index(index as usize, &*child);

    Box::leak(node);
    Box::leak(child);
}

#[no_mangle]
pub unsafe extern "C" fn Java_app_visly_stretch_Node_nRemoveChild(
    _: JNIEnv,
    _: JObject,
    node: jlong,
    child: jlong,
) {
    let mut node: Box<Node> = Box::from_raw(node as *mut Node);
    let child: Box<Node> = Box::from_raw(child as *mut Node);
    node.remove_child(&*child);

    Box::leak(node);
    Box::leak(child);
}

#[no_mangle]
pub unsafe extern "C" fn Java_app_visly_stretch_Node_nRemoveChildAtIndex(
    _: JNIEnv,
    _: JObject,
    node: jlong,
    index: jint,
) {
    let node: Box<Node> = Box::from_raw(node as *mut Node);
    Box::leak(node).remove_child_at_index(index as usize);
}

#[no_mangle]
pub unsafe extern "C" fn Java_app_visly_stretch_Node_nComputeLayout(
    env: JNIEnv,
    _: JObject,
    node: jlong,
    width: jfloat,
    height: jfloat,
) -> jfloatArray {
    let node: Box<Node> = Box::from_raw(node as *mut Node);
    let layout = Box::leak(node).compute_layout(Size {
        width: if f32::is_nan(width) {
           Number::Undefined
       } else {
           Number::Defined(width)
       },
        height: if f32::is_nan(height) {
            Number::Undefined
        } else {
            Number::Defined(height)
        },
    }).unwrap();

    let mut output = vec![];
    copy_output(&layout, &mut output);

    let result = env.new_float_array(output.len() as i32).unwrap();
    env.set_float_array_region(result, 0, &output).unwrap();
    result
}

fn copy_output(layout: &Layout, output: &mut Vec<f32>) {
    output.push(layout.location.x);
    output.push(layout.location.y);
    output.push(layout.size.width);
    output.push(layout.size.height);
    output.push(layout.children.len() as f32);

    for child in &layout.children {
        copy_output(child, output);
    }
}
