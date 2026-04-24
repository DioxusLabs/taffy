//! JNI bindings to the Taffy layout engine.
//!
//! These are the Rust counterpart of the `com.dioxuslabs.taffy` Java library.
//! All handles crossing the JNI boundary are raw pointers encoded as `jlong`.
//! Errors are reported to Java by throwing `com/dioxuslabs/taffy/TaffyException`.

#![allow(non_snake_case)]
#![allow(clippy::missing_safety_doc)]
#![allow(dead_code)]

use jni::objects::{GlobalRef, JClass, JFloatArray, JLongArray, JObject, JValue};
use jni::sys::{jfloat, jint, jlong};
use jni::JNIEnv;

use taffy::geometry::{MinMax, Point, Rect, Size};
use taffy::prelude::*;
use taffy::style::{BoxSizing, GridTemplateRepetition, Overflow, RepetitionCount};

/// The concrete `Style` type used by these bindings. We want the default
/// `CheapCloneStr` so that the bindings don't expose a generic parameter.
type JStyle = taffy::style::Style;

/// Per-node context: an optional reference to a Java `MeasureFunction`.
/// `None` for nodes without a measure callback (most of them).
type NodeCtx = Option<GlobalRef>;
type JTree = TaffyTree<NodeCtx>;

// --- Tag constants (mirror the Java side in NativeBridge) ------------------

const DIM_LENGTH: jint = 0;
const DIM_PERCENT: jint = 1;
const DIM_AUTO: jint = 2;

const AVAIL_DEFINITE: jint = 0;
const AVAIL_MIN_CONTENT: jint = 1;
const AVAIL_MAX_CONTENT: jint = 2;

// --- Helpers ---------------------------------------------------------------

fn tree_from_handle<'a>(handle: jlong) -> &'a mut JTree {
    unsafe { &mut *(handle as *mut JTree) }
}

fn style_from_handle<'a>(handle: jlong) -> &'a mut JStyle {
    unsafe { &mut *(handle as *mut JStyle) }
}

fn throw(env: &mut JNIEnv, msg: &str) {
    let _ = env.throw_new("com/dioxuslabs/taffy/TaffyException", msg);
}

fn dim(tag: jint, value: jfloat) -> Dimension {
    match tag {
        DIM_LENGTH => Dimension::length(value),
        DIM_PERCENT => Dimension::percent(value),
        _ => Dimension::auto(),
    }
}

fn lp(tag: jint, value: jfloat) -> LengthPercentage {
    match tag {
        DIM_PERCENT => LengthPercentage::percent(value),
        // Default to length for anything that is not explicitly percent
        _ => LengthPercentage::length(value),
    }
}

fn lpa(tag: jint, value: jfloat) -> LengthPercentageAuto {
    match tag {
        DIM_LENGTH => LengthPercentageAuto::length(value),
        DIM_PERCENT => LengthPercentageAuto::percent(value),
        _ => LengthPercentageAuto::auto(),
    }
}

fn avail(tag: jint, value: jfloat) -> AvailableSpace {
    match tag {
        AVAIL_DEFINITE => AvailableSpace::Definite(value),
        AVAIL_MIN_CONTENT => AvailableSpace::MinContent,
        _ => AvailableSpace::MaxContent,
    }
}

/// Map a non-negative `tag` to the corresponding `Display` variant.
/// Any out-of-range tag falls back to `Display::Flex`.
fn display_from_tag(tag: jint) -> Display {
    // 0=Block, 1=Flex, 2=Grid, 3=None (must mirror Java side)
    match tag {
        0 => Display::Block,
        2 => Display::Grid,
        3 => Display::None,
        _ => Display::Flex,
    }
}

fn position_from_tag(tag: jint) -> Position {
    match tag {
        1 => Position::Absolute,
        _ => Position::Relative,
    }
}

fn box_sizing_from_tag(tag: jint) -> BoxSizing {
    match tag {
        1 => BoxSizing::ContentBox,
        _ => BoxSizing::BorderBox,
    }
}

fn overflow_from_tag(tag: jint) -> Overflow {
    match tag {
        1 => Overflow::Clip,
        2 => Overflow::Hidden,
        3 => Overflow::Scroll,
        _ => Overflow::Visible,
    }
}

fn flex_direction_from_tag(tag: jint) -> FlexDirection {
    match tag {
        1 => FlexDirection::Column,
        2 => FlexDirection::RowReverse,
        3 => FlexDirection::ColumnReverse,
        _ => FlexDirection::Row,
    }
}

fn flex_wrap_from_tag(tag: jint) -> FlexWrap {
    match tag {
        1 => FlexWrap::Wrap,
        2 => FlexWrap::WrapReverse,
        _ => FlexWrap::NoWrap,
    }
}

/// 0 = None (unset). 1..=9 map to AlignContent/JustifyContent variants.
fn align_content_from_tag(tag: jint) -> Option<AlignContent> {
    match tag {
        1 => Some(AlignContent::Start),
        2 => Some(AlignContent::End),
        3 => Some(AlignContent::FlexStart),
        4 => Some(AlignContent::FlexEnd),
        5 => Some(AlignContent::Center),
        6 => Some(AlignContent::Stretch),
        7 => Some(AlignContent::SpaceBetween),
        8 => Some(AlignContent::SpaceEvenly),
        9 => Some(AlignContent::SpaceAround),
        _ => None,
    }
}

/// 0 = None (unset). 1..=7 map to AlignItems variants.
fn align_items_from_tag(tag: jint) -> Option<AlignItems> {
    match tag {
        1 => Some(AlignItems::Start),
        2 => Some(AlignItems::End),
        3 => Some(AlignItems::FlexStart),
        4 => Some(AlignItems::FlexEnd),
        5 => Some(AlignItems::Center),
        6 => Some(AlignItems::Baseline),
        7 => Some(AlignItems::Stretch),
        _ => None,
    }
}

// --- Tree lifecycle --------------------------------------------------------

#[no_mangle]
pub extern "system" fn Java_com_dioxuslabs_taffy_NativeBridge_treeNew(_env: JNIEnv, _class: JClass) -> jlong {
    Box::into_raw(Box::new(JTree::new())) as jlong
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_dioxuslabs_taffy_NativeBridge_treeFree(
    _env: JNIEnv,
    _class: JClass,
    handle: jlong,
) {
    if handle != 0 {
        drop(Box::from_raw(handle as *mut JTree));
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dioxuslabs_taffy_NativeBridge_treeEnableRounding(
    _env: JNIEnv,
    _class: JClass,
    handle: jlong,
    enable: jint,
) {
    let tree = tree_from_handle(handle);
    if enable != 0 {
        tree.enable_rounding();
    } else {
        tree.disable_rounding();
    }
}

// --- Node creation ---------------------------------------------------------

#[no_mangle]
pub extern "system" fn Java_com_dioxuslabs_taffy_NativeBridge_treeNewLeaf(
    mut env: JNIEnv,
    _class: JClass,
    tree_handle: jlong,
    style_handle: jlong,
) -> jlong {
    let tree = tree_from_handle(tree_handle);
    let style = style_from_handle(style_handle).clone();
    match tree.new_leaf(style) {
        Ok(id) => u64::from(id) as jlong,
        Err(e) => {
            throw(&mut env, &e.to_string());
            0
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dioxuslabs_taffy_NativeBridge_treeNewLeafWithMeasure<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    tree_handle: jlong,
    style_handle: jlong,
    measure: JObject<'local>,
) -> jlong {
    let tree = tree_from_handle(tree_handle);
    let style = style_from_handle(style_handle).clone();
    let ctx = match env.new_global_ref(&measure) {
        Ok(g) => Some(g),
        Err(e) => {
            throw(&mut env, &format!("failed to pin measure callback: {e}"));
            return 0;
        }
    };
    match tree.new_leaf_with_context(style, ctx) {
        Ok(id) => u64::from(id) as jlong,
        Err(e) => {
            throw(&mut env, &e.to_string());
            0
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dioxuslabs_taffy_NativeBridge_treeSetMeasure<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    tree_handle: jlong,
    node: jlong,
    measure: JObject<'local>,
) {
    let tree = tree_from_handle(tree_handle);
    let ctx: NodeCtx = if measure.is_null() {
        None
    } else {
        match env.new_global_ref(&measure) {
            Ok(g) => Some(g),
            Err(e) => {
                throw(&mut env, &format!("failed to pin measure callback: {e}"));
                return;
            }
        }
    };
    if let Err(e) = tree.set_node_context(NodeId::from(node as u64), Some(ctx)) {
        throw(&mut env, &e.to_string());
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dioxuslabs_taffy_NativeBridge_treeNewWithChildren<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    tree_handle: jlong,
    style_handle: jlong,
    children: JLongArray<'local>,
) -> jlong {
    let tree = tree_from_handle(tree_handle);
    let style = style_from_handle(style_handle).clone();

    let len = match env.get_array_length(&children) {
        Ok(l) => l as usize,
        Err(_) => {
            throw(&mut env, "failed to read children array length");
            return 0;
        }
    };
    let mut buf: Vec<jlong> = vec![0; len];
    if env.get_long_array_region(&children, 0, &mut buf).is_err() {
        throw(&mut env, "failed to read children array");
        return 0;
    }
    let child_ids: Vec<NodeId> = buf.into_iter().map(|v| NodeId::from(v as u64)).collect();

    match tree.new_with_children(style, &child_ids) {
        Ok(id) => u64::from(id) as jlong,
        Err(e) => {
            throw(&mut env, &e.to_string());
            0
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dioxuslabs_taffy_NativeBridge_treeSetStyle(
    mut env: JNIEnv,
    _class: JClass,
    tree_handle: jlong,
    node_id: jlong,
    style_handle: jlong,
) {
    let tree = tree_from_handle(tree_handle);
    let style = style_from_handle(style_handle).clone();
    if let Err(e) = tree.set_style(NodeId::from(node_id as u64), style) {
        throw(&mut env, &e.to_string());
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dioxuslabs_taffy_NativeBridge_treeAddChild(
    mut env: JNIEnv,
    _class: JClass,
    tree_handle: jlong,
    parent: jlong,
    child: jlong,
) {
    let tree = tree_from_handle(tree_handle);
    if let Err(e) = tree.add_child(NodeId::from(parent as u64), NodeId::from(child as u64)) {
        throw(&mut env, &e.to_string());
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dioxuslabs_taffy_NativeBridge_treeRemoveChild(
    mut env: JNIEnv,
    _class: JClass,
    tree_handle: jlong,
    parent: jlong,
    child: jlong,
) {
    let tree = tree_from_handle(tree_handle);
    if let Err(e) = tree.remove_child(NodeId::from(parent as u64), NodeId::from(child as u64)) {
        throw(&mut env, &e.to_string());
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dioxuslabs_taffy_NativeBridge_treeChildCount(
    _env: JNIEnv,
    _class: JClass,
    tree_handle: jlong,
    node: jlong,
) -> jint {
    let tree = tree_from_handle(tree_handle);
    tree.child_count(NodeId::from(node as u64)) as jint
}

#[no_mangle]
pub extern "system" fn Java_com_dioxuslabs_taffy_NativeBridge_treeMarkDirty(
    mut env: JNIEnv,
    _class: JClass,
    tree_handle: jlong,
    node: jlong,
) {
    let tree = tree_from_handle(tree_handle);
    if let Err(e) = tree.mark_dirty(NodeId::from(node as u64)) {
        throw(&mut env, &e.to_string());
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dioxuslabs_taffy_NativeBridge_treeRemove(
    mut env: JNIEnv,
    _class: JClass,
    tree_handle: jlong,
    node: jlong,
) {
    let tree = tree_from_handle(tree_handle);
    if let Err(e) = tree.remove(NodeId::from(node as u64)) {
        throw(&mut env, &e.to_string());
    }
}

// --- Layout computation ----------------------------------------------------

#[no_mangle]
pub extern "system" fn Java_com_dioxuslabs_taffy_NativeBridge_treeComputeLayout<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    tree_handle: jlong,
    node: jlong,
    width_tag: jint,
    width_value: jfloat,
    height_tag: jint,
    height_value: jfloat,
) {
    let tree = tree_from_handle(tree_handle);
    let space = Size { width: avail(width_tag, width_value), height: avail(height_tag, height_value) };

    // We always take the `_with_measure` path. For nodes whose context is
    // `None` (or without any context at all) the closure short-circuits to
    // `Size::ZERO`, which matches taffy's default behavior for leaves without
    // a measure function — so we keep a single entry point.
    let result = tree.compute_layout_with_measure(
        NodeId::from(node as u64),
        space,
        |known, avail_space, _node_id, node_context, _style| {
            // Taffy hint: if both dimensions are already known, skip the
            // callback — the measure function would usually just return them.
            if let (Some(w), Some(h)) = (known.width, known.height) {
                return Size { width: w, height: h };
            }
            let callback = match node_context {
                Some(Some(g)) => g,
                _ => return Size::ZERO,
            };
            invoke_measure(&mut env, callback, known, avail_space).unwrap_or(Size::ZERO)
        },
    );
    if let Err(e) = result {
        throw(&mut env, &e.to_string());
    }
}

/// Invoke the Java `MeasureFunction` behind `callback`. On any JNI error
/// (including a Java exception thrown by the callback) we clear the pending
/// exception and return `None` so the caller can fall back to `Size::ZERO`.
fn invoke_measure(
    env: &mut JNIEnv,
    callback: &GlobalRef,
    known: Size<Option<f32>>,
    avail: Size<AvailableSpace>,
) -> Option<Size<f32>> {
    let (aw_tag, aw_val) = encode_avail(avail.width);
    let (ah_tag, ah_val) = encode_avail(avail.height);
    let known_w = known.width.unwrap_or(f32::NAN);
    let known_h = known.height.unwrap_or(f32::NAN);

    let result = env.call_method(
        callback,
        "measure",
        "(FFIFIF)[F",
        &[
            JValue::Float(known_w),
            JValue::Float(known_h),
            JValue::Int(aw_tag),
            JValue::Float(aw_val),
            JValue::Int(ah_tag),
            JValue::Float(ah_val),
        ],
    );

    // Java exceptions become returned `Err(JavaException)` from `call_method`.
    // Clear and fall back to zero so layout keeps going.
    if env.exception_check().unwrap_or(false) {
        let _ = env.exception_clear();
    }

    let arr_obj = result.ok()?.l().ok()?;
    let arr = JFloatArray::from(arr_obj);
    let mut buf = [0f32; 2];
    env.get_float_array_region(&arr, 0, &mut buf).ok()?;
    Some(Size { width: buf[0], height: buf[1] })
}

fn encode_avail(a: AvailableSpace) -> (jint, jfloat) {
    match a {
        AvailableSpace::Definite(v) => (AVAIL_DEFINITE, v),
        AvailableSpace::MinContent => (AVAIL_MIN_CONTENT, 0.0),
        AvailableSpace::MaxContent => (AVAIL_MAX_CONTENT, 0.0),
    }
}

/// Writes the computed layout into `out`, which must have length >= 19.
/// Indices: 0 x, 1 y, 2 w, 3 h, 4 cw, 5 ch, 6-9 padding l/r/t/b,
/// 10-13 border l/r/t/b, 14-17 margin l/r/t/b (currently always zero), 18 order.
#[no_mangle]
pub extern "system" fn Java_com_dioxuslabs_taffy_NativeBridge_treeLayout<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    tree_handle: jlong,
    node: jlong,
    out: JFloatArray<'local>,
) {
    let tree = tree_from_handle(tree_handle);
    let layout = match tree.layout(NodeId::from(node as u64)) {
        Ok(l) => *l,
        Err(e) => {
            throw(&mut env, &e.to_string());
            return;
        }
    };
    let buf: [jfloat; 19] = [
        layout.location.x,
        layout.location.y,
        layout.size.width,
        layout.size.height,
        layout.content_size.width,
        layout.content_size.height,
        layout.padding.left,
        layout.padding.right,
        layout.padding.top,
        layout.padding.bottom,
        layout.border.left,
        layout.border.right,
        layout.border.top,
        layout.border.bottom,
        layout.margin.left,
        layout.margin.right,
        layout.margin.top,
        layout.margin.bottom,
        layout.order as jfloat,
    ];
    if env.set_float_array_region(&out, 0, &buf).is_err() {
        throw(&mut env, "failed to write layout output array");
    }
}

// --- Style lifecycle -------------------------------------------------------

#[no_mangle]
pub extern "system" fn Java_com_dioxuslabs_taffy_NativeBridge_styleNew(_env: JNIEnv, _class: JClass) -> jlong {
    Box::into_raw(Box::new(JStyle::default())) as jlong
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_dioxuslabs_taffy_NativeBridge_styleFree(
    _env: JNIEnv,
    _class: JClass,
    handle: jlong,
) {
    if handle != 0 {
        drop(Box::from_raw(handle as *mut JStyle));
    }
}

// --- Style setters ---------------------------------------------------------

#[no_mangle]
pub extern "system" fn Java_com_dioxuslabs_taffy_NativeBridge_styleSetDisplay(
    _env: JNIEnv,
    _class: JClass,
    handle: jlong,
    tag: jint,
) {
    style_from_handle(handle).display = display_from_tag(tag);
}

#[no_mangle]
pub extern "system" fn Java_com_dioxuslabs_taffy_NativeBridge_styleSetPosition(
    _env: JNIEnv,
    _class: JClass,
    handle: jlong,
    tag: jint,
) {
    style_from_handle(handle).position = position_from_tag(tag);
}

#[no_mangle]
pub extern "system" fn Java_com_dioxuslabs_taffy_NativeBridge_styleSetBoxSizing(
    _env: JNIEnv,
    _class: JClass,
    handle: jlong,
    tag: jint,
) {
    style_from_handle(handle).box_sizing = box_sizing_from_tag(tag);
}

#[no_mangle]
pub extern "system" fn Java_com_dioxuslabs_taffy_NativeBridge_styleSetOverflow(
    _env: JNIEnv,
    _class: JClass,
    handle: jlong,
    x_tag: jint,
    y_tag: jint,
) {
    let s = style_from_handle(handle);
    s.overflow = Point { x: overflow_from_tag(x_tag), y: overflow_from_tag(y_tag) };
}

#[no_mangle]
pub extern "system" fn Java_com_dioxuslabs_taffy_NativeBridge_styleSetScrollbarWidth(
    _env: JNIEnv,
    _class: JClass,
    handle: jlong,
    value: jfloat,
) {
    style_from_handle(handle).scrollbar_width = value;
}

#[no_mangle]
pub extern "system" fn Java_com_dioxuslabs_taffy_NativeBridge_styleSetSize(
    _env: JNIEnv,
    _class: JClass,
    handle: jlong,
    w_tag: jint,
    w_val: jfloat,
    h_tag: jint,
    h_val: jfloat,
) {
    style_from_handle(handle).size = Size { width: dim(w_tag, w_val), height: dim(h_tag, h_val) };
}

#[no_mangle]
pub extern "system" fn Java_com_dioxuslabs_taffy_NativeBridge_styleSetMinSize(
    _env: JNIEnv,
    _class: JClass,
    handle: jlong,
    w_tag: jint,
    w_val: jfloat,
    h_tag: jint,
    h_val: jfloat,
) {
    style_from_handle(handle).min_size = Size { width: dim(w_tag, w_val), height: dim(h_tag, h_val) };
}

#[no_mangle]
pub extern "system" fn Java_com_dioxuslabs_taffy_NativeBridge_styleSetMaxSize(
    _env: JNIEnv,
    _class: JClass,
    handle: jlong,
    w_tag: jint,
    w_val: jfloat,
    h_tag: jint,
    h_val: jfloat,
) {
    style_from_handle(handle).max_size = Size { width: dim(w_tag, w_val), height: dim(h_tag, h_val) };
}

#[no_mangle]
pub extern "system" fn Java_com_dioxuslabs_taffy_NativeBridge_styleSetMargin(
    _env: JNIEnv,
    _class: JClass,
    handle: jlong,
    l_tag: jint,
    l_val: jfloat,
    r_tag: jint,
    r_val: jfloat,
    t_tag: jint,
    t_val: jfloat,
    b_tag: jint,
    b_val: jfloat,
) {
    style_from_handle(handle).margin =
        Rect { left: lpa(l_tag, l_val), right: lpa(r_tag, r_val), top: lpa(t_tag, t_val), bottom: lpa(b_tag, b_val) };
}

#[no_mangle]
pub extern "system" fn Java_com_dioxuslabs_taffy_NativeBridge_styleSetInset(
    _env: JNIEnv,
    _class: JClass,
    handle: jlong,
    l_tag: jint,
    l_val: jfloat,
    r_tag: jint,
    r_val: jfloat,
    t_tag: jint,
    t_val: jfloat,
    b_tag: jint,
    b_val: jfloat,
) {
    style_from_handle(handle).inset =
        Rect { left: lpa(l_tag, l_val), right: lpa(r_tag, r_val), top: lpa(t_tag, t_val), bottom: lpa(b_tag, b_val) };
}

#[no_mangle]
pub extern "system" fn Java_com_dioxuslabs_taffy_NativeBridge_styleSetPadding(
    _env: JNIEnv,
    _class: JClass,
    handle: jlong,
    l_tag: jint,
    l_val: jfloat,
    r_tag: jint,
    r_val: jfloat,
    t_tag: jint,
    t_val: jfloat,
    b_tag: jint,
    b_val: jfloat,
) {
    style_from_handle(handle).padding =
        Rect { left: lp(l_tag, l_val), right: lp(r_tag, r_val), top: lp(t_tag, t_val), bottom: lp(b_tag, b_val) };
}

#[no_mangle]
pub extern "system" fn Java_com_dioxuslabs_taffy_NativeBridge_styleSetBorder(
    _env: JNIEnv,
    _class: JClass,
    handle: jlong,
    l_tag: jint,
    l_val: jfloat,
    r_tag: jint,
    r_val: jfloat,
    t_tag: jint,
    t_val: jfloat,
    b_tag: jint,
    b_val: jfloat,
) {
    style_from_handle(handle).border =
        Rect { left: lp(l_tag, l_val), right: lp(r_tag, r_val), top: lp(t_tag, t_val), bottom: lp(b_tag, b_val) };
}

#[no_mangle]
pub extern "system" fn Java_com_dioxuslabs_taffy_NativeBridge_styleSetGap(
    _env: JNIEnv,
    _class: JClass,
    handle: jlong,
    w_tag: jint,
    w_val: jfloat,
    h_tag: jint,
    h_val: jfloat,
) {
    style_from_handle(handle).gap = Size { width: lp(w_tag, w_val), height: lp(h_tag, h_val) };
}

#[no_mangle]
pub extern "system" fn Java_com_dioxuslabs_taffy_NativeBridge_styleSetFlexDirection(
    _env: JNIEnv,
    _class: JClass,
    handle: jlong,
    tag: jint,
) {
    style_from_handle(handle).flex_direction = flex_direction_from_tag(tag);
}

#[no_mangle]
pub extern "system" fn Java_com_dioxuslabs_taffy_NativeBridge_styleSetFlexWrap(
    _env: JNIEnv,
    _class: JClass,
    handle: jlong,
    tag: jint,
) {
    style_from_handle(handle).flex_wrap = flex_wrap_from_tag(tag);
}

#[no_mangle]
pub extern "system" fn Java_com_dioxuslabs_taffy_NativeBridge_styleSetFlexGrow(
    _env: JNIEnv,
    _class: JClass,
    handle: jlong,
    value: jfloat,
) {
    style_from_handle(handle).flex_grow = value;
}

#[no_mangle]
pub extern "system" fn Java_com_dioxuslabs_taffy_NativeBridge_styleSetFlexShrink(
    _env: JNIEnv,
    _class: JClass,
    handle: jlong,
    value: jfloat,
) {
    style_from_handle(handle).flex_shrink = value;
}

#[no_mangle]
pub extern "system" fn Java_com_dioxuslabs_taffy_NativeBridge_styleSetFlexBasis(
    _env: JNIEnv,
    _class: JClass,
    handle: jlong,
    tag: jint,
    value: jfloat,
) {
    style_from_handle(handle).flex_basis = dim(tag, value);
}

#[no_mangle]
pub extern "system" fn Java_com_dioxuslabs_taffy_NativeBridge_styleSetJustifyContent(
    _env: JNIEnv,
    _class: JClass,
    handle: jlong,
    tag: jint,
) {
    style_from_handle(handle).justify_content = align_content_from_tag(tag);
}

#[no_mangle]
pub extern "system" fn Java_com_dioxuslabs_taffy_NativeBridge_styleSetAlignContent(
    _env: JNIEnv,
    _class: JClass,
    handle: jlong,
    tag: jint,
) {
    style_from_handle(handle).align_content = align_content_from_tag(tag);
}

#[no_mangle]
pub extern "system" fn Java_com_dioxuslabs_taffy_NativeBridge_styleSetAlignItems(
    _env: JNIEnv,
    _class: JClass,
    handle: jlong,
    tag: jint,
) {
    style_from_handle(handle).align_items = align_items_from_tag(tag);
}

#[no_mangle]
pub extern "system" fn Java_com_dioxuslabs_taffy_NativeBridge_styleSetAlignSelf(
    _env: JNIEnv,
    _class: JClass,
    handle: jlong,
    tag: jint,
) {
    style_from_handle(handle).align_self = align_items_from_tag(tag);
}

#[no_mangle]
pub extern "system" fn Java_com_dioxuslabs_taffy_NativeBridge_styleSetJustifyItems(
    _env: JNIEnv,
    _class: JClass,
    handle: jlong,
    tag: jint,
) {
    style_from_handle(handle).justify_items = align_items_from_tag(tag);
}

#[no_mangle]
pub extern "system" fn Java_com_dioxuslabs_taffy_NativeBridge_styleSetJustifySelf(
    _env: JNIEnv,
    _class: JClass,
    handle: jlong,
    tag: jint,
) {
    style_from_handle(handle).justify_self = align_items_from_tag(tag);
}

/// NaN means "unset" (`None`).
#[no_mangle]
pub extern "system" fn Java_com_dioxuslabs_taffy_NativeBridge_styleSetAspectRatio(
    _env: JNIEnv,
    _class: JClass,
    handle: jlong,
    value: jfloat,
) {
    style_from_handle(handle).aspect_ratio = if value.is_nan() { None } else { Some(value) };
}

// --- Style snapshot (reads) -------------------------------------------------
//
// `treeGetStyle` fills one int[] and one float[] with every scalar style
// property, in a fixed layout that mirrors the Java side (see
// TaffyTree.getStyle). Grid tracks (variable-length) are not included
// and must be read separately if ever exposed.
//
// Int buffer (39 slots):
//   0: display, 1: position, 2: box_sizing, 3: overflow.x, 4: overflow.y,
//   5: flex_direction, 6: flex_wrap,
//   7: justify_content (0 = unset),  8: align_content  (0 = unset),
//   9: align_items     (0 = unset), 10: align_self     (0 = unset),
//  11: justify_items   (0 = unset), 12: justify_self   (0 = unset),
//  13: grid_auto_flow,
//  14..=20: Dimension tags — size.w, size.h, min.w, min.h, max.w, max.h, flex_basis
//  21..=28: LPA tags      — margin.l/r/t/b, inset.l/r/t/b
//  29..=38: LP tags       — padding.l/r/t/b, border.l/r/t/b, gap.w, gap.h
//
// Float buffer (29 slots):
//   0: scrollbar_width, 1: flex_grow, 2: flex_shrink,
//   3: aspect_ratio (NaN = unset),
//   4..=10: Dimension values  (same order as Int slots 14..=20)
//  11..=18: LPA values        (same order as Int slots 21..=28)
//  19..=28: LP values         (same order as Int slots 29..=38)

const SNAPSHOT_INTS: usize = 39;
const SNAPSHOT_FLOATS: usize = 29;

fn display_to_tag(d: Display) -> jint {
    match d {
        Display::Block => 0,
        Display::Flex => 1,
        Display::Grid => 2,
        Display::None => 3,
    }
}
fn position_to_tag(p: Position) -> jint {
    match p {
        Position::Relative => 0,
        Position::Absolute => 1,
    }
}
fn box_sizing_to_tag(b: BoxSizing) -> jint {
    match b {
        BoxSizing::BorderBox => 0,
        BoxSizing::ContentBox => 1,
    }
}
fn overflow_to_tag(o: Overflow) -> jint {
    match o {
        Overflow::Visible => 0,
        Overflow::Clip => 1,
        Overflow::Hidden => 2,
        Overflow::Scroll => 3,
    }
}
fn flex_direction_to_tag(f: FlexDirection) -> jint {
    match f {
        FlexDirection::Row => 0,
        FlexDirection::Column => 1,
        FlexDirection::RowReverse => 2,
        FlexDirection::ColumnReverse => 3,
    }
}
fn flex_wrap_to_tag(f: FlexWrap) -> jint {
    match f {
        FlexWrap::NoWrap => 0,
        FlexWrap::Wrap => 1,
        FlexWrap::WrapReverse => 2,
    }
}
fn align_content_opt_to_tag(o: Option<AlignContent>) -> jint {
    match o {
        None => 0,
        Some(AlignContent::Start) => 1,
        Some(AlignContent::End) => 2,
        Some(AlignContent::FlexStart) => 3,
        Some(AlignContent::FlexEnd) => 4,
        Some(AlignContent::Center) => 5,
        Some(AlignContent::Stretch) => 6,
        Some(AlignContent::SpaceBetween) => 7,
        Some(AlignContent::SpaceEvenly) => 8,
        Some(AlignContent::SpaceAround) => 9,
    }
}
fn align_items_opt_to_tag(o: Option<AlignItems>) -> jint {
    match o {
        None => 0,
        Some(AlignItems::Start) => 1,
        Some(AlignItems::End) => 2,
        Some(AlignItems::FlexStart) => 3,
        Some(AlignItems::FlexEnd) => 4,
        Some(AlignItems::Center) => 5,
        Some(AlignItems::Baseline) => 6,
        Some(AlignItems::Stretch) => 7,
    }
}
fn grid_auto_flow_to_tag(g: GridAutoFlow) -> jint {
    match g {
        GridAutoFlow::Row => 0,
        GridAutoFlow::Column => 1,
        GridAutoFlow::RowDense => 2,
        GridAutoFlow::ColumnDense => 3,
    }
}

/// Convert a taffy `Dimension` to the Java (tag, value) pair.
fn dim_to_java(d: Dimension) -> (jint, jfloat) {
    match d.into_raw().tag() {
        CompactLength::LENGTH_TAG => (DIM_LENGTH, d.value()),
        CompactLength::PERCENT_TAG => (DIM_PERCENT, d.value()),
        _ => (DIM_AUTO, 0.0),
    }
}
fn lpa_to_java(l: LengthPercentageAuto) -> (jint, jfloat) {
    let raw = l.into_raw();
    match raw.tag() {
        CompactLength::LENGTH_TAG => (DIM_LENGTH, raw.value()),
        CompactLength::PERCENT_TAG => (DIM_PERCENT, raw.value()),
        _ => (DIM_AUTO, 0.0),
    }
}
fn lp_to_java(l: LengthPercentage) -> (jint, jfloat) {
    let raw = l.into_raw();
    match raw.tag() {
        CompactLength::PERCENT_TAG => (DIM_PERCENT, raw.value()),
        // Everything else (length, calc, …) falls back to a length.
        _ => (DIM_LENGTH, raw.value()),
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dioxuslabs_taffy_NativeBridge_treeGetStyle<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    tree_handle: jlong,
    node: jlong,
    int_out: jni::objects::JIntArray<'local>,
    float_out: jni::objects::JFloatArray<'local>,
) {
    let tree = tree_from_handle(tree_handle);
    let style = match tree.style(NodeId::from(node as u64)) {
        Ok(s) => s.clone(),
        Err(e) => {
            throw(&mut env, &e.to_string());
            return;
        }
    };

    let mut ints = [0 as jint; SNAPSHOT_INTS];
    let mut floats = [0.0 as jfloat; SNAPSHOT_FLOATS];

    // Scalar enums
    ints[0] = display_to_tag(style.display);
    ints[1] = position_to_tag(style.position);
    ints[2] = box_sizing_to_tag(style.box_sizing);
    ints[3] = overflow_to_tag(style.overflow.x);
    ints[4] = overflow_to_tag(style.overflow.y);
    ints[5] = flex_direction_to_tag(style.flex_direction);
    ints[6] = flex_wrap_to_tag(style.flex_wrap);
    ints[7] = align_content_opt_to_tag(style.justify_content);
    ints[8] = align_content_opt_to_tag(style.align_content);
    ints[9] = align_items_opt_to_tag(style.align_items);
    ints[10] = align_items_opt_to_tag(style.align_self);
    ints[11] = align_items_opt_to_tag(style.justify_items);
    ints[12] = align_items_opt_to_tag(style.justify_self);
    ints[13] = grid_auto_flow_to_tag(style.grid_auto_flow);

    // Scalar floats
    floats[0] = style.scrollbar_width;
    floats[1] = style.flex_grow;
    floats[2] = style.flex_shrink;
    floats[3] = style.aspect_ratio.unwrap_or(f32::NAN);

    // Dimensions: size, min_size, max_size, flex_basis → int 14..=20 / float 4..=10
    let dims: [Dimension; 7] = [
        style.size.width,
        style.size.height,
        style.min_size.width,
        style.min_size.height,
        style.max_size.width,
        style.max_size.height,
        style.flex_basis,
    ];
    for (i, d) in dims.iter().enumerate() {
        let (tag, val) = dim_to_java(*d);
        ints[14 + i] = tag;
        floats[4 + i] = val;
    }

    // LPA rects: margin, inset → int 21..=28 / float 11..=18
    let lpas: [LengthPercentageAuto; 8] = [
        style.margin.left,
        style.margin.right,
        style.margin.top,
        style.margin.bottom,
        style.inset.left,
        style.inset.right,
        style.inset.top,
        style.inset.bottom,
    ];
    for (i, l) in lpas.iter().enumerate() {
        let (tag, val) = lpa_to_java(*l);
        ints[21 + i] = tag;
        floats[11 + i] = val;
    }

    // LP rects + gap: padding, border, gap → int 29..=38 / float 19..=28
    let lps: [LengthPercentage; 10] = [
        style.padding.left,
        style.padding.right,
        style.padding.top,
        style.padding.bottom,
        style.border.left,
        style.border.right,
        style.border.top,
        style.border.bottom,
        style.gap.width,
        style.gap.height,
    ];
    for (i, l) in lps.iter().enumerate() {
        let (tag, val) = lp_to_java(*l);
        ints[29 + i] = tag;
        floats[19 + i] = val;
    }

    if env.set_int_array_region(&int_out, 0, &ints).is_err()
        || env.set_float_array_region(&float_out, 0, &floats).is_err()
    {
        throw(&mut env, "failed to write style snapshot arrays");
    }
}

// --- Grid ------------------------------------------------------------------
//
// Tags (mirror Java side):
//   Track kind:  0=length, 1=percent, 2=fr, 3=auto, 4=min-content, 5=max-content
//   Placement:   0=auto,   1=line(value as i32),  2=span(value as u32)
//   Auto flow:   0=Row,    1=Column,              2=RowDense,  3=ColumnDense

/// Tags invalid for the min side of a track (fr / fit-content) fold to Auto,
/// matching CSS behavior.
fn min_track_from_tag(tag: jint, value: jfloat) -> MinTrackSizingFunction {
    match tag {
        0 => MinTrackSizingFunction::length(value),
        1 => MinTrackSizingFunction::percent(value),
        4 => MinTrackSizingFunction::min_content(),
        5 => MinTrackSizingFunction::max_content(),
        _ => MinTrackSizingFunction::auto(),
    }
}

fn max_track_from_tag(tag: jint, value: jfloat) -> MaxTrackSizingFunction {
    match tag {
        0 => MaxTrackSizingFunction::length(value),
        1 => MaxTrackSizingFunction::percent(value),
        2 => MaxTrackSizingFunction::fr(value),
        4 => MaxTrackSizingFunction::min_content(),
        5 => MaxTrackSizingFunction::max_content(),
        6 => MaxTrackSizingFunction::fit_content_px(value),
        7 => MaxTrackSizingFunction::fit_content_percent(value),
        _ => MaxTrackSizingFunction::auto(),
    }
}

fn track_from_tags(min_tag: jint, min_val: jfloat, max_tag: jint, max_val: jfloat) -> TrackSizingFunction {
    MinMax { min: min_track_from_tag(min_tag, min_val), max: max_track_from_tag(max_tag, max_val) }
}

// `taffy::Style` defaults its `S` generic to Taffy's crate-private
// `DefaultCheapStr` — which is `String` under the `std` feature we
// build with. Keeping these aliases matches exactly.
type JGridTemplateComponent = GridTemplateComponent<String>;
type JGridPlacement = GridPlacement<String>;

fn template_component_from_tags(
    min_tag: jint,
    min_val: jfloat,
    max_tag: jint,
    max_val: jfloat,
) -> JGridTemplateComponent {
    JGridTemplateComponent::Single(track_from_tags(min_tag, min_val, max_tag, max_val))
}

fn placement_from_tag(tag: jint, value: jint, name: Option<String>) -> JGridPlacement {
    match tag {
        1 => JGridPlacement::from_line_index(value as i16),
        2 => JGridPlacement::from_span(value.max(1) as u16),
        3 => JGridPlacement::NamedLine(name.unwrap_or_default(), value as i16),
        4 => JGridPlacement::NamedSpan(name.unwrap_or_default(), value.max(1) as u16),
        _ => JGridPlacement::Auto,
    }
}

fn grid_auto_flow_from_tag(tag: jint) -> GridAutoFlow {
    match tag {
        1 => GridAutoFlow::Column,
        2 => GridAutoFlow::RowDense,
        3 => GridAutoFlow::ColumnDense,
        _ => GridAutoFlow::Row,
    }
}

/// Read four parallel arrays (minTags, minVals, maxTags, maxVals) and return
/// an iterator yielding `(minTag, minVal, maxTag, maxVal)` tuples. Throws
/// a Java exception and returns None if array lengths disagree.
unsafe fn read_track_quads(
    env: &mut JNIEnv,
    min_tags: jni::objects::JIntArray,
    min_vals: jni::objects::JFloatArray,
    max_tags: jni::objects::JIntArray,
    max_vals: jni::objects::JFloatArray,
) -> Option<Vec<(jint, jfloat, jint, jfloat)>> {
    let n = env.get_array_length(&min_tags).ok()? as usize;
    let others = [
        env.get_array_length(&min_vals).ok()? as usize,
        env.get_array_length(&max_tags).ok()? as usize,
        env.get_array_length(&max_vals).ok()? as usize,
    ];
    if others.iter().any(|&l| l != n) {
        throw(env, "grid track array lengths disagree");
        return None;
    }
    let mut min_tag_buf: Vec<jint> = vec![0; n];
    let mut min_val_buf: Vec<jfloat> = vec![0.0; n];
    let mut max_tag_buf: Vec<jint> = vec![0; n];
    let mut max_val_buf: Vec<jfloat> = vec![0.0; n];
    env.get_int_array_region(&min_tags, 0, &mut min_tag_buf).ok()?;
    env.get_float_array_region(&min_vals, 0, &mut min_val_buf).ok()?;
    env.get_int_array_region(&max_tags, 0, &mut max_tag_buf).ok()?;
    env.get_float_array_region(&max_vals, 0, &mut max_val_buf).ok()?;
    Some(
        min_tag_buf
            .into_iter()
            .zip(min_val_buf)
            .zip(max_tag_buf.into_iter().zip(max_val_buf))
            .map(|((minT, minV), (maxT, maxV))| (minT, minV, maxT, maxV))
            .collect(),
    )
}

/// Header layout: 4 ints per component — [kind, repKind, repCount, trackCount].
/// Reads headers + 4 parallel track arrays and assembles GridTemplateComponents.
unsafe fn read_template_components(
    env: &mut JNIEnv,
    headers: jni::objects::JIntArray,
    min_tags: jni::objects::JIntArray,
    min_vals: jni::objects::JFloatArray,
    max_tags: jni::objects::JIntArray,
    max_vals: jni::objects::JFloatArray,
) -> Option<Vec<JGridTemplateComponent>> {
    let tracks = read_track_quads(env, min_tags, min_vals, max_tags, max_vals)?;

    let hlen = env.get_array_length(&headers).ok()? as usize;
    if hlen % 4 != 0 {
        throw(env, "grid template headers length must be a multiple of 4");
        return None;
    }
    let comp_count = hlen / 4;
    let mut hdr = vec![0 as jint; hlen];
    env.get_int_array_region(&headers, 0, &mut hdr).ok()?;

    let mut out = Vec::with_capacity(comp_count);
    let mut ti = 0usize;
    for i in 0..comp_count {
        let kind = hdr[4 * i];
        let rep_kind = hdr[4 * i + 1];
        let rep_count = hdr[4 * i + 2];
        let track_count = hdr[4 * i + 3] as usize;

        if ti + track_count > tracks.len() {
            throw(env, "grid template header track-count exceeds the supplied tracks");
            return None;
        }
        let slice = &tracks[ti..ti + track_count];
        ti += track_count;

        match kind {
            // Repeat
            1 => {
                let count = match rep_kind {
                    0 => RepetitionCount::AutoFill,
                    1 => RepetitionCount::AutoFit,
                    _ => RepetitionCount::Count(rep_count.max(1) as u16),
                };
                let repeated: Vec<TrackSizingFunction> =
                    slice.iter().map(|&(a, b, c, d)| track_from_tags(a, b, c, d)).collect();
                out.push(JGridTemplateComponent::Repeat(GridTemplateRepetition {
                    count,
                    tracks: repeated,
                    line_names: Vec::new(),
                }));
            }
            // Single (default for any kind != 1)
            _ => {
                if track_count != 1 {
                    throw(env, "single grid template component must have exactly one track");
                    return None;
                }
                let (a, b, c, d) = slice[0];
                out.push(template_component_from_tags(a, b, c, d));
            }
        }
    }
    Some(out)
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_dioxuslabs_taffy_NativeBridge_styleSetGridTemplateColumns<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    handle: jlong,
    headers: jni::objects::JIntArray<'local>,
    min_tags: jni::objects::JIntArray<'local>,
    min_vals: jni::objects::JFloatArray<'local>,
    max_tags: jni::objects::JIntArray<'local>,
    max_vals: jni::objects::JFloatArray<'local>,
) {
    if let Some(comps) = read_template_components(&mut env, headers, min_tags, min_vals, max_tags, max_vals) {
        style_from_handle(handle).grid_template_columns = comps.into_iter().collect();
    }
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_dioxuslabs_taffy_NativeBridge_styleSetGridTemplateRows<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    handle: jlong,
    headers: jni::objects::JIntArray<'local>,
    min_tags: jni::objects::JIntArray<'local>,
    min_vals: jni::objects::JFloatArray<'local>,
    max_tags: jni::objects::JIntArray<'local>,
    max_vals: jni::objects::JFloatArray<'local>,
) {
    if let Some(comps) = read_template_components(&mut env, headers, min_tags, min_vals, max_tags, max_vals) {
        style_from_handle(handle).grid_template_rows = comps.into_iter().collect();
    }
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_dioxuslabs_taffy_NativeBridge_styleSetGridAutoColumns<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    handle: jlong,
    min_tags: jni::objects::JIntArray<'local>,
    min_vals: jni::objects::JFloatArray<'local>,
    max_tags: jni::objects::JIntArray<'local>,
    max_vals: jni::objects::JFloatArray<'local>,
) {
    if let Some(quads) = read_track_quads(&mut env, min_tags, min_vals, max_tags, max_vals) {
        style_from_handle(handle).grid_auto_columns =
            quads.into_iter().map(|(a, b, c, d)| track_from_tags(a, b, c, d)).collect();
    }
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_dioxuslabs_taffy_NativeBridge_styleSetGridAutoRows<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    handle: jlong,
    min_tags: jni::objects::JIntArray<'local>,
    min_vals: jni::objects::JFloatArray<'local>,
    max_tags: jni::objects::JIntArray<'local>,
    max_vals: jni::objects::JFloatArray<'local>,
) {
    if let Some(quads) = read_track_quads(&mut env, min_tags, min_vals, max_tags, max_vals) {
        style_from_handle(handle).grid_auto_rows =
            quads.into_iter().map(|(a, b, c, d)| track_from_tags(a, b, c, d)).collect();
    }
}

fn read_jstring(env: &mut JNIEnv, obj: &JObject) -> Option<String> {
    if obj.is_null() {
        return None;
    }
    let jstr = jni::objects::JString::from(unsafe { JObject::from_raw(obj.as_raw()) });
    env.get_string(&jstr).ok().map(|s| s.into())
}

#[no_mangle]
pub extern "system" fn Java_com_dioxuslabs_taffy_NativeBridge_styleSetGridColumn<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    handle: jlong,
    start_tag: jint,
    start_val: jint,
    start_name: JObject<'local>,
    end_tag: jint,
    end_val: jint,
    end_name: JObject<'local>,
) {
    let start_name = read_jstring(&mut env, &start_name);
    let end_name = read_jstring(&mut env, &end_name);
    style_from_handle(handle).grid_column = Line {
        start: placement_from_tag(start_tag, start_val, start_name),
        end: placement_from_tag(end_tag, end_val, end_name),
    };
}

#[no_mangle]
pub extern "system" fn Java_com_dioxuslabs_taffy_NativeBridge_styleSetGridRow<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    handle: jlong,
    start_tag: jint,
    start_val: jint,
    start_name: JObject<'local>,
    end_tag: jint,
    end_val: jint,
    end_name: JObject<'local>,
) {
    let start_name = read_jstring(&mut env, &start_name);
    let end_name = read_jstring(&mut env, &end_name);
    style_from_handle(handle).grid_row = Line {
        start: placement_from_tag(start_tag, start_val, start_name),
        end: placement_from_tag(end_tag, end_val, end_name),
    };
}

#[no_mangle]
pub extern "system" fn Java_com_dioxuslabs_taffy_NativeBridge_styleSetGridAutoFlow(
    _env: JNIEnv,
    _class: JClass,
    handle: jlong,
    tag: jint,
) {
    style_from_handle(handle).grid_auto_flow = grid_auto_flow_from_tag(tag);
}

// --- Grid template names + areas (D.3) ------------------------------------

/// Read a `String[]` from JNI as `Vec<String>`. None on any JNI error (caller
/// should have thrown already or can fall back to empty).
unsafe fn read_string_array(env: &mut JNIEnv, arr: &jni::objects::JObjectArray) -> Option<Vec<String>> {
    let n = env.get_array_length(arr).ok()? as usize;
    let mut out = Vec::with_capacity(n);
    for i in 0..n {
        let obj = env.get_object_array_element(arr, i as jni::sys::jsize).ok()?;
        if obj.is_null() {
            out.push(String::new());
            continue;
        }
        let jstr = jni::objects::JString::from(obj);
        let s: String = env.get_string(&jstr).ok()?.into();
        out.push(s);
    }
    Some(out)
}

/// Read line-name buffers (counts[], flat String[]) and produce the nested
/// `Vec<Vec<String>>` taffy expects.
unsafe fn read_line_names(
    env: &mut JNIEnv,
    counts: jni::objects::JIntArray,
    flat_names: jni::objects::JObjectArray,
) -> Option<Vec<Vec<String>>> {
    let n = env.get_array_length(&counts).ok()? as usize;
    let mut count_buf = vec![0 as jint; n];
    env.get_int_array_region(&counts, 0, &mut count_buf).ok()?;

    let all_names = read_string_array(env, &flat_names)?;
    let total: i64 = count_buf.iter().map(|&c| c as i64).sum();
    if total != all_names.len() as i64 {
        throw(env, "grid line-name counts don't match flat-names length");
        return None;
    }

    let mut out: Vec<Vec<String>> = Vec::with_capacity(n);
    let mut cursor = 0usize;
    for &c in &count_buf {
        let c = c as usize;
        out.push(all_names[cursor..cursor + c].to_vec());
        cursor += c;
    }
    Some(out)
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_dioxuslabs_taffy_NativeBridge_styleSetGridTemplateColumnNames<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    handle: jlong,
    counts: jni::objects::JIntArray<'local>,
    flat_names: jni::objects::JObjectArray<'local>,
) {
    if let Some(names) = read_line_names(&mut env, counts, flat_names) {
        style_from_handle(handle).grid_template_column_names = names.into_iter().collect();
    }
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_dioxuslabs_taffy_NativeBridge_styleSetGridTemplateRowNames<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    handle: jlong,
    counts: jni::objects::JIntArray<'local>,
    flat_names: jni::objects::JObjectArray<'local>,
) {
    if let Some(names) = read_line_names(&mut env, counts, flat_names) {
        style_from_handle(handle).grid_template_row_names = names.into_iter().collect();
    }
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_dioxuslabs_taffy_NativeBridge_styleSetGridTemplateAreas<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    handle: jlong,
    names: jni::objects::JObjectArray<'local>,
    row_starts: jni::objects::JIntArray<'local>,
    row_ends: jni::objects::JIntArray<'local>,
    col_starts: jni::objects::JIntArray<'local>,
    col_ends: jni::objects::JIntArray<'local>,
) {
    let Some(name_vec) = read_string_array(&mut env, &names) else { return };
    let n = name_vec.len();

    let lengths = [
        env.get_array_length(&row_starts).unwrap_or(0) as usize,
        env.get_array_length(&row_ends).unwrap_or(0) as usize,
        env.get_array_length(&col_starts).unwrap_or(0) as usize,
        env.get_array_length(&col_ends).unwrap_or(0) as usize,
    ];
    if lengths.iter().any(|&l| l != n) {
        throw(&mut env, "grid template area array lengths disagree");
        return;
    }

    let mut rs = vec![0 as jint; n];
    let mut re = vec![0 as jint; n];
    let mut cs = vec![0 as jint; n];
    let mut ce = vec![0 as jint; n];
    if env.get_int_array_region(&row_starts, 0, &mut rs).is_err()
        || env.get_int_array_region(&row_ends, 0, &mut re).is_err()
        || env.get_int_array_region(&col_starts, 0, &mut cs).is_err()
        || env.get_int_array_region(&col_ends, 0, &mut ce).is_err()
    {
        throw(&mut env, "failed to read grid template area arrays");
        return;
    }

    let areas: Vec<taffy::style::GridTemplateArea<String>> = name_vec
        .into_iter()
        .enumerate()
        .map(|(i, name)| taffy::style::GridTemplateArea {
            name,
            row_start: rs[i].max(1) as u16,
            row_end: re[i].max(1) as u16,
            column_start: cs[i].max(1) as u16,
            column_end: ce[i].max(1) as u16,
        })
        .collect();

    style_from_handle(handle).grid_template_areas = areas.into_iter().collect();
}
