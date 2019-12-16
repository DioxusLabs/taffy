#![cfg(target_os = "android")]
#![allow(non_snake_case)]

use jni::objects::{JClass, JObject, JValue};
use jni::sys::{jboolean, jfloat, jfloatArray, jint, jlong, jlongArray};
use jni::JNIEnv;
use std::f32;
use stretch::geometry::*;
use stretch::node::*;
use stretch::number::*;
use stretch::style::*;

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
pub unsafe extern "C" fn Java_app_visly_stretch_Stretch_nInit(_: JNIEnv, _: JClass) -> jlong {
    let stretch = Stretch::new();
    Box::into_raw(Box::new(stretch)) as jlong
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

    aspectRatio: jfloat,
) -> jlong {
    let style = Style {
        display: match display {
            0 => Display::Flex,
            1 => Display::None,
            _ => panic!(),
        },

        position_type: match positionType {
            0 => PositionType::Relative,
            1 => PositionType::Absolute,
            _ => panic!(),
        },

        direction: match direction {
            0 => Direction::Inherit,
            1 => Direction::LTR,
            2 => Direction::RTL,
            _ => panic!(),
        },

        flex_direction: match flexDirection {
            0 => FlexDirection::Row,
            1 => FlexDirection::Column,
            2 => FlexDirection::RowReverse,
            3 => FlexDirection::ColumnReverse,
            _ => panic!(),
        },

        flex_wrap: match flexWrap {
            0 => FlexWrap::NoWrap,
            1 => FlexWrap::Wrap,
            2 => FlexWrap::WrapReverse,
            _ => panic!(),
        },

        overflow: match overflow {
            0 => Overflow::Visible,
            1 => Overflow::Hidden,
            2 => Overflow::Scroll,
            _ => panic!(),
        },

        align_items: match alignItems {
            0 => AlignItems::FlexStart,
            1 => AlignItems::FlexEnd,
            2 => AlignItems::Center,
            3 => AlignItems::Baseline,
            4 => AlignItems::Stretch,
            _ => panic!(),
        },

        align_self: match alignSelf {
            0 => AlignSelf::Auto,
            1 => AlignSelf::FlexStart,
            2 => AlignSelf::FlexEnd,
            3 => AlignSelf::Center,
            4 => AlignSelf::Baseline,
            5 => AlignSelf::Stretch,
            _ => panic!(),
        },

        align_content: match alignContent {
            0 => AlignContent::FlexStart,
            1 => AlignContent::FlexEnd,
            2 => AlignContent::Center,
            3 => AlignContent::Stretch,
            4 => AlignContent::SpaceBetween,
            5 => AlignContent::SpaceAround,
            _ => panic!(),
        },

        justify_content: match justifyContent {
            0 => JustifyContent::FlexStart,
            1 => JustifyContent::FlexEnd,
            2 => JustifyContent::Center,
            3 => JustifyContent::SpaceBetween,
            4 => JustifyContent::SpaceAround,
            5 => JustifyContent::SpaceEvenly,
            _ => panic!(),
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

        size: Size { width: dimension(widthType, widthValue), height: dimension(heightType, heightValue) },

        min_size: Size {
            width: dimension(minWidthType, minWidthValue),
            height: dimension(minHeightType, minHeightValue),
        },

        max_size: Size {
            width: dimension(maxWidthType, maxWidthValue),
            height: dimension(maxHeightType, maxHeightValue),
        },

        aspect_ratio: if f32::is_nan(aspectRatio) { Number::Undefined } else { Number::Defined(aspectRatio) },
    };

    Box::into_raw(Box::new(style)) as jlong
}

#[no_mangle]
pub unsafe extern "C" fn Java_app_visly_stretch_Style_nFree(_: JNIEnv, _: JObject, style: jlong) {
    let _style = Box::from_raw(style as *mut Style);
}

#[no_mangle]
pub unsafe extern "C" fn Java_app_visly_stretch_Node_nConstruct(
    env: JNIEnv,
    _: JObject,
    stretch: jlong,
    style: jlong,
    children: jlongArray,
) -> jlong {
    let num_children = env.get_array_length(children).unwrap();
    let mut buff = vec![0; num_children as usize];
    env.get_long_array_region(children, 0, &mut buff).unwrap();

    let children = buff.into_iter().map(|ptr| *Box::leak(Box::from_raw(ptr as *mut Node))).collect::<Vec<_>>();

    let mut stretch = Box::from_raw(stretch as *mut Stretch);
    let style = Box::from_raw(style as *mut Style);
    let node = stretch.new_node(*style, &children).unwrap();

    Box::leak(stretch);
    Box::leak(style);

    Box::into_raw(Box::new(node)) as jlong
}

#[no_mangle]
pub unsafe extern "C" fn Java_app_visly_stretch_Node_nConstructLeaf(
    env: JNIEnv<'static>,
    _: JObject,
    stretch: jlong,
    style: jlong,
    measure: JObject,
) -> jlong {
    let measure = env.new_global_ref(measure).unwrap();
    let mut stretch = Box::from_raw(stretch as *mut Stretch);
    let style = Box::from_raw(style as *mut Style);

    let node = stretch
        .new_leaf(
            *style,
            stretch::node::MeasureFunc::Boxed(Box::new(move |constraint| {
                let result = env.call_method(
                    measure.as_obj(),
                    "measure",
                    "(FF)[F",
                    &[
                        JValue::from(constraint.width.or_else(f32::NAN)),
                        JValue::from(constraint.height.or_else(f32::NAN)),
                    ],
                );

                match result {
                    Ok(result) => {
                        let size = result.l().unwrap().into_inner() as jfloatArray;
                        let mut buff: [f32; 2] = [0.0, 0.0];
                        env.get_float_array_region(size, 0, &mut buff).unwrap();
                        Size { width: buff[0], height: buff[1] }
                    }
                    Err(_) => constraint.map(|v| v.or_else(0.0)),
                }
            })),
        )
        .unwrap();

    Box::leak(stretch);
    Box::leak(style);

    Box::into_raw(Box::new(node)) as jlong
}

#[no_mangle]
pub unsafe extern "C" fn Java_app_visly_stretch_Node_nFree(_: JNIEnv, _: JObject, stretch: jlong, node: jlong) {
    let mut stretch = Box::from_raw(stretch as *mut Stretch);
    let node = Box::from_raw(node as *mut Node);

    stretch.remove(*node);

    Box::leak(stretch);
}

#[no_mangle]
pub unsafe extern "C" fn Java_app_visly_stretch_Node_nSetMeasure(
    env: JNIEnv<'static>,
    _: JObject,
    stretch: jlong,
    node: jlong,
    measure: JObject,
) {
    let measure = env.new_global_ref(measure).unwrap();
    let mut stretch = Box::from_raw(stretch as *mut Stretch);
    let node = Box::from_raw(node as *mut Node);

    stretch
        .set_measure(
            *node,
            Some(stretch::node::MeasureFunc::Boxed(Box::new(move |constraint| {
                let result = env.call_method(
                    measure.as_obj(),
                    "measure",
                    "(FF)[F",
                    &[
                        JValue::from(constraint.width.or_else(f32::NAN)),
                        JValue::from(constraint.height.or_else(f32::NAN)),
                    ],
                );

                match result {
                    Ok(result) => {
                        let size = result.l().unwrap().into_inner() as jfloatArray;
                        let mut buff: [f32; 2] = [0.0, 0.0];
                        env.get_float_array_region(size, 0, &mut buff).unwrap();
                        Size { width: buff[0], height: buff[1] }
                    }
                    Err(_) => constraint.map(|v| v.or_else(0.0)),
                }
            }))),
        )
        .unwrap();

    Box::leak(node);
    Box::leak(stretch);
}

#[no_mangle]
pub unsafe extern "C" fn Java_app_visly_stretch_Node_nSetStyle(
    _: JNIEnv,
    _: JObject,
    stretch: jlong,
    node: jlong,
    style: jlong,
) {
    let style = Box::from_raw(style as *mut Style);
    let mut stretch = Box::from_raw(stretch as *mut Stretch);
    let node = Box::from_raw(node as *mut Node);

    stretch.set_style(*node, *style).unwrap();

    Box::leak(style);
    Box::leak(node);
    Box::leak(stretch);
}

#[no_mangle]
pub unsafe extern "C" fn Java_app_visly_stretch_Node_nIsDirty(
    _: JNIEnv,
    _: JObject,
    stretch: jlong,
    node: jlong,
) -> jboolean {
    let stretch = Box::from_raw(stretch as *mut Stretch);
    let node = Box::from_raw(node as *mut Node);

    let dirty = stretch.dirty(*node).unwrap() as jboolean;

    Box::leak(node);
    Box::leak(stretch);

    dirty
}

#[no_mangle]
pub unsafe extern "C" fn Java_app_visly_stretch_Node_nMarkDirty(_: JNIEnv, _: JObject, stretch: jlong, node: jlong) {
    let mut stretch = Box::from_raw(stretch as *mut Stretch);
    let node = Box::from_raw(node as *mut Node);
    stretch.mark_dirty(*node).unwrap();

    Box::leak(node);
    Box::leak(stretch);
}

#[no_mangle]
pub unsafe extern "C" fn Java_app_visly_stretch_Node_nSetChildren(
    env: JNIEnv,
    _: JObject,
    stretch: jlong,
    node: jlong,
    children: jlongArray,
) {
    let num_children = env.get_array_length(children).unwrap();
    let mut buff = vec![0; num_children as usize];
    env.get_long_array_region(children, 0, &mut buff).unwrap();

    let children = buff.into_iter().map(|ptr| *Box::leak(Box::from_raw(ptr as *mut Node))).collect::<Vec<_>>();

    let mut stretch = Box::from_raw(stretch as *mut Stretch);
    let node = Box::from_raw(node as *mut Node);
    stretch.set_children(*node, &children).unwrap();

    Box::leak(node);
    Box::leak(stretch);
}

#[no_mangle]
pub unsafe extern "C" fn Java_app_visly_stretch_Node_nAddChild(
    _: JNIEnv,
    _: JObject,
    stretch: jlong,
    node: jlong,
    child: jlong,
) {
    let mut stretch = Box::from_raw(stretch as *mut Stretch);
    let node = Box::from_raw(node as *mut Node);
    let child = Box::from_raw(child as *mut Node);

    stretch.add_child(*node, *child).unwrap();

    Box::leak(node);
    Box::leak(child);
    Box::leak(stretch);
}

#[no_mangle]
pub unsafe extern "C" fn Java_app_visly_stretch_Node_nReplaceChildAtIndex(
    _: JNIEnv,
    _: JObject,
    stretch: jlong,
    node: jlong,
    index: jint,
    child: jlong,
) {
    let mut stretch = Box::from_raw(stretch as *mut Stretch);
    let node = Box::from_raw(node as *mut Node);
    let child = Box::from_raw(child as *mut Node);

    stretch.replace_child_at_index(*node, index as usize, *child).unwrap();

    Box::leak(node);
    Box::leak(child);
    Box::leak(stretch);
}

#[no_mangle]
pub unsafe extern "C" fn Java_app_visly_stretch_Node_nRemoveChild(
    _: JNIEnv,
    _: JObject,
    stretch: jlong,
    node: jlong,
    child: jlong,
) {
    let mut stretch = Box::from_raw(stretch as *mut Stretch);
    let node = Box::from_raw(node as *mut Node);
    let child = Box::from_raw(child as *mut Node);

    stretch.remove_child(*node, *child).unwrap();

    Box::leak(node);
    Box::leak(child);
    Box::leak(stretch);
}

#[no_mangle]
pub unsafe extern "C" fn Java_app_visly_stretch_Node_nRemoveChildAtIndex(
    _: JNIEnv,
    _: JObject,
    stretch: jlong,
    node: jlong,
    index: jint,
) {
    let mut stretch = Box::from_raw(stretch as *mut Stretch);
    let node = Box::from_raw(node as *mut Node);

    stretch.remove_child_at_index(*node, index as usize).unwrap();

    Box::leak(node);
    Box::leak(stretch);
}

#[no_mangle]
pub unsafe extern "C" fn Java_app_visly_stretch_Node_nComputeLayout(
    env: JNIEnv,
    _: JObject,
    stretch: jlong,
    node: jlong,
    width: jfloat,
    height: jfloat,
) -> jfloatArray {
    let mut stretch = Box::from_raw(stretch as *mut Stretch);
    let node = Box::from_raw(node as *mut Node);

    stretch
        .compute_layout(
            *node,
            Size {
                width: if f32::is_nan(width) { Number::Undefined } else { Number::Defined(width) },
                height: if f32::is_nan(height) { Number::Undefined } else { Number::Defined(height) },
            },
        )
        .unwrap();

    let mut output = vec![];
    copy_output(&stretch, *node, &mut output);

    Box::leak(node);
    Box::leak(stretch);

    let result = env.new_float_array(output.len() as i32).unwrap();
    env.set_float_array_region(result, 0, &output).unwrap();
    result
}

fn copy_output(stretch: &Stretch, node: Node, output: &mut Vec<f32>) {
    let layout = stretch.layout(node).unwrap();
    let children = stretch.children(node).unwrap();

    output.push(layout.location.x);
    output.push(layout.location.y);
    output.push(layout.size.width);
    output.push(layout.size.height);
    output.push(children.len() as f32);

    for child in &children {
        copy_output(stretch, *child, output);
    }
}
