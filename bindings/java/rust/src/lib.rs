//! JNI bindings to the Taffy layout engine.
//!
//! These are the Rust counterpart of the `dev.dioxus.taffy` Java library.
//! All handles crossing the JNI boundary are raw pointers encoded as `jlong`.
//! Errors are reported to Java by throwing `dev/dioxus/taffy/TaffyException`.

#![allow(non_snake_case)]
#![allow(clippy::missing_safety_doc)]
#![allow(dead_code)]

use jni::objects::{JClass, JFloatArray, JLongArray};
use jni::sys::{jfloat, jint, jlong};
use jni::JNIEnv;

use taffy::geometry::{Point, Rect, Size};
use taffy::prelude::*;
use taffy::style::{BoxSizing, Overflow};

/// The concrete `Style` type used by these bindings. We want the default
/// `CheapCloneStr` so that the bindings don't expose a generic parameter.
type JStyle = taffy::style::Style;

// --- Tag constants (mirror the Java side in NativeBridge) ------------------

const DIM_LENGTH: jint = 0;
const DIM_PERCENT: jint = 1;
const DIM_AUTO: jint = 2;

const AVAIL_DEFINITE: jint = 0;
const AVAIL_MIN_CONTENT: jint = 1;
const AVAIL_MAX_CONTENT: jint = 2;

// --- Helpers ---------------------------------------------------------------

fn tree_from_handle<'a>(handle: jlong) -> &'a mut TaffyTree<()> {
    unsafe { &mut *(handle as *mut TaffyTree<()>) }
}

fn style_from_handle<'a>(handle: jlong) -> &'a mut JStyle {
    unsafe { &mut *(handle as *mut JStyle) }
}

fn throw(env: &mut JNIEnv, msg: &str) {
    let _ = env.throw_new("dev/dioxus/taffy/TaffyException", msg);
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
pub extern "system" fn Java_dev_dioxus_taffy_NativeBridge_treeNew(
    _env: JNIEnv,
    _class: JClass,
) -> jlong {
    Box::into_raw(Box::new(TaffyTree::<()>::new())) as jlong
}

#[no_mangle]
pub unsafe extern "system" fn Java_dev_dioxus_taffy_NativeBridge_treeFree(
    _env: JNIEnv,
    _class: JClass,
    handle: jlong,
) {
    if handle != 0 {
        drop(Box::from_raw(handle as *mut TaffyTree<()>));
    }
}

#[no_mangle]
pub extern "system" fn Java_dev_dioxus_taffy_NativeBridge_treeEnableRounding(
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
pub extern "system" fn Java_dev_dioxus_taffy_NativeBridge_treeNewLeaf(
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
pub extern "system" fn Java_dev_dioxus_taffy_NativeBridge_treeNewWithChildren<'local>(
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
pub extern "system" fn Java_dev_dioxus_taffy_NativeBridge_treeSetStyle(
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
pub extern "system" fn Java_dev_dioxus_taffy_NativeBridge_treeAddChild(
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
pub extern "system" fn Java_dev_dioxus_taffy_NativeBridge_treeRemoveChild(
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
pub extern "system" fn Java_dev_dioxus_taffy_NativeBridge_treeChildCount(
    _env: JNIEnv,
    _class: JClass,
    tree_handle: jlong,
    node: jlong,
) -> jint {
    let tree = tree_from_handle(tree_handle);
    tree.child_count(NodeId::from(node as u64)) as jint
}

#[no_mangle]
pub extern "system" fn Java_dev_dioxus_taffy_NativeBridge_treeMarkDirty(
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
pub extern "system" fn Java_dev_dioxus_taffy_NativeBridge_treeRemove(
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
pub extern "system" fn Java_dev_dioxus_taffy_NativeBridge_treeComputeLayout(
    mut env: JNIEnv,
    _class: JClass,
    tree_handle: jlong,
    node: jlong,
    width_tag: jint,
    width_value: jfloat,
    height_tag: jint,
    height_value: jfloat,
) {
    let tree = tree_from_handle(tree_handle);
    let space = Size { width: avail(width_tag, width_value), height: avail(height_tag, height_value) };
    if let Err(e) = tree.compute_layout(NodeId::from(node as u64), space) {
        throw(&mut env, &e.to_string());
    }
}

/// Writes the computed layout into `out`, which must have length >= 19.
/// Indices: 0 x, 1 y, 2 w, 3 h, 4 cw, 5 ch, 6-9 padding l/r/t/b,
/// 10-13 border l/r/t/b, 14-17 margin l/r/t/b (currently always zero), 18 order.
#[no_mangle]
pub extern "system" fn Java_dev_dioxus_taffy_NativeBridge_treeLayout<'local>(
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
pub extern "system" fn Java_dev_dioxus_taffy_NativeBridge_styleNew(
    _env: JNIEnv,
    _class: JClass,
) -> jlong {
    Box::into_raw(Box::new(JStyle::default())) as jlong
}

#[no_mangle]
pub unsafe extern "system" fn Java_dev_dioxus_taffy_NativeBridge_styleFree(
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
pub extern "system" fn Java_dev_dioxus_taffy_NativeBridge_styleSetDisplay(
    _env: JNIEnv,
    _class: JClass,
    handle: jlong,
    tag: jint,
) {
    style_from_handle(handle).display = display_from_tag(tag);
}

#[no_mangle]
pub extern "system" fn Java_dev_dioxus_taffy_NativeBridge_styleSetPosition(
    _env: JNIEnv,
    _class: JClass,
    handle: jlong,
    tag: jint,
) {
    style_from_handle(handle).position = position_from_tag(tag);
}

#[no_mangle]
pub extern "system" fn Java_dev_dioxus_taffy_NativeBridge_styleSetBoxSizing(
    _env: JNIEnv,
    _class: JClass,
    handle: jlong,
    tag: jint,
) {
    style_from_handle(handle).box_sizing = box_sizing_from_tag(tag);
}

#[no_mangle]
pub extern "system" fn Java_dev_dioxus_taffy_NativeBridge_styleSetOverflow(
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
pub extern "system" fn Java_dev_dioxus_taffy_NativeBridge_styleSetScrollbarWidth(
    _env: JNIEnv,
    _class: JClass,
    handle: jlong,
    value: jfloat,
) {
    style_from_handle(handle).scrollbar_width = value;
}

#[no_mangle]
pub extern "system" fn Java_dev_dioxus_taffy_NativeBridge_styleSetSize(
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
pub extern "system" fn Java_dev_dioxus_taffy_NativeBridge_styleSetMinSize(
    _env: JNIEnv,
    _class: JClass,
    handle: jlong,
    w_tag: jint,
    w_val: jfloat,
    h_tag: jint,
    h_val: jfloat,
) {
    style_from_handle(handle).min_size =
        Size { width: dim(w_tag, w_val), height: dim(h_tag, h_val) };
}

#[no_mangle]
pub extern "system" fn Java_dev_dioxus_taffy_NativeBridge_styleSetMaxSize(
    _env: JNIEnv,
    _class: JClass,
    handle: jlong,
    w_tag: jint,
    w_val: jfloat,
    h_tag: jint,
    h_val: jfloat,
) {
    style_from_handle(handle).max_size =
        Size { width: dim(w_tag, w_val), height: dim(h_tag, h_val) };
}

#[no_mangle]
pub extern "system" fn Java_dev_dioxus_taffy_NativeBridge_styleSetMargin(
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
    style_from_handle(handle).margin = Rect {
        left: lpa(l_tag, l_val),
        right: lpa(r_tag, r_val),
        top: lpa(t_tag, t_val),
        bottom: lpa(b_tag, b_val),
    };
}

#[no_mangle]
pub extern "system" fn Java_dev_dioxus_taffy_NativeBridge_styleSetInset(
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
    style_from_handle(handle).inset = Rect {
        left: lpa(l_tag, l_val),
        right: lpa(r_tag, r_val),
        top: lpa(t_tag, t_val),
        bottom: lpa(b_tag, b_val),
    };
}

#[no_mangle]
pub extern "system" fn Java_dev_dioxus_taffy_NativeBridge_styleSetPadding(
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
    style_from_handle(handle).padding = Rect {
        left: lp(l_tag, l_val),
        right: lp(r_tag, r_val),
        top: lp(t_tag, t_val),
        bottom: lp(b_tag, b_val),
    };
}

#[no_mangle]
pub extern "system" fn Java_dev_dioxus_taffy_NativeBridge_styleSetBorder(
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
    style_from_handle(handle).border = Rect {
        left: lp(l_tag, l_val),
        right: lp(r_tag, r_val),
        top: lp(t_tag, t_val),
        bottom: lp(b_tag, b_val),
    };
}

#[no_mangle]
pub extern "system" fn Java_dev_dioxus_taffy_NativeBridge_styleSetGap(
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
pub extern "system" fn Java_dev_dioxus_taffy_NativeBridge_styleSetFlexDirection(
    _env: JNIEnv,
    _class: JClass,
    handle: jlong,
    tag: jint,
) {
    style_from_handle(handle).flex_direction = flex_direction_from_tag(tag);
}

#[no_mangle]
pub extern "system" fn Java_dev_dioxus_taffy_NativeBridge_styleSetFlexWrap(
    _env: JNIEnv,
    _class: JClass,
    handle: jlong,
    tag: jint,
) {
    style_from_handle(handle).flex_wrap = flex_wrap_from_tag(tag);
}

#[no_mangle]
pub extern "system" fn Java_dev_dioxus_taffy_NativeBridge_styleSetFlexGrow(
    _env: JNIEnv,
    _class: JClass,
    handle: jlong,
    value: jfloat,
) {
    style_from_handle(handle).flex_grow = value;
}

#[no_mangle]
pub extern "system" fn Java_dev_dioxus_taffy_NativeBridge_styleSetFlexShrink(
    _env: JNIEnv,
    _class: JClass,
    handle: jlong,
    value: jfloat,
) {
    style_from_handle(handle).flex_shrink = value;
}

#[no_mangle]
pub extern "system" fn Java_dev_dioxus_taffy_NativeBridge_styleSetFlexBasis(
    _env: JNIEnv,
    _class: JClass,
    handle: jlong,
    tag: jint,
    value: jfloat,
) {
    style_from_handle(handle).flex_basis = dim(tag, value);
}

#[no_mangle]
pub extern "system" fn Java_dev_dioxus_taffy_NativeBridge_styleSetJustifyContent(
    _env: JNIEnv,
    _class: JClass,
    handle: jlong,
    tag: jint,
) {
    style_from_handle(handle).justify_content = align_content_from_tag(tag);
}

#[no_mangle]
pub extern "system" fn Java_dev_dioxus_taffy_NativeBridge_styleSetAlignContent(
    _env: JNIEnv,
    _class: JClass,
    handle: jlong,
    tag: jint,
) {
    style_from_handle(handle).align_content = align_content_from_tag(tag);
}

#[no_mangle]
pub extern "system" fn Java_dev_dioxus_taffy_NativeBridge_styleSetAlignItems(
    _env: JNIEnv,
    _class: JClass,
    handle: jlong,
    tag: jint,
) {
    style_from_handle(handle).align_items = align_items_from_tag(tag);
}

#[no_mangle]
pub extern "system" fn Java_dev_dioxus_taffy_NativeBridge_styleSetAlignSelf(
    _env: JNIEnv,
    _class: JClass,
    handle: jlong,
    tag: jint,
) {
    style_from_handle(handle).align_self = align_items_from_tag(tag);
}

#[no_mangle]
pub extern "system" fn Java_dev_dioxus_taffy_NativeBridge_styleSetJustifyItems(
    _env: JNIEnv,
    _class: JClass,
    handle: jlong,
    tag: jint,
) {
    style_from_handle(handle).justify_items = align_items_from_tag(tag);
}

#[no_mangle]
pub extern "system" fn Java_dev_dioxus_taffy_NativeBridge_styleSetJustifySelf(
    _env: JNIEnv,
    _class: JClass,
    handle: jlong,
    tag: jint,
) {
    style_from_handle(handle).justify_self = align_items_from_tag(tag);
}

/// NaN means "unset" (`None`).
#[no_mangle]
pub extern "system" fn Java_dev_dioxus_taffy_NativeBridge_styleSetAspectRatio(
    _env: JNIEnv,
    _class: JClass,
    handle: jlong,
    value: jfloat,
) {
    style_from_handle(handle).aspect_ratio = if value.is_nan() { None } else { Some(value) };
}

// --- Grid ------------------------------------------------------------------
//
// Tags (mirror Java side):
//   Track kind:  0=length, 1=percent, 2=fr, 3=auto, 4=min-content, 5=max-content
//   Placement:   0=auto,   1=line(value as i32),  2=span(value as u32)
//   Auto flow:   0=Row,    1=Column,              2=RowDense,  3=ColumnDense

fn track_from_tag(tag: jint, value: jfloat) -> TrackSizingFunction {
    match tag {
        0 => TrackSizingFunction::from_length(value),
        1 => TrackSizingFunction::from_percent(value),
        2 => TrackSizingFunction::from_fr(value),
        4 => TrackSizingFunction::MIN_CONTENT,
        5 => TrackSizingFunction::MAX_CONTENT,
        _ => TrackSizingFunction::AUTO,
    }
}

// `taffy::Style` defaults its `S` generic to Taffy's crate-private
// `DefaultCheapStr` — which is `String` under the `std` feature we
// build with. Keeping these aliases matches exactly.
type JGridTemplateComponent = GridTemplateComponent<String>;
type JGridPlacement = GridPlacement<String>;

fn template_component_from_tag(tag: jint, value: jfloat) -> JGridTemplateComponent {
    JGridTemplateComponent::Single(track_from_tag(tag, value))
}

fn placement_from_tag(tag: jint, value: jfloat) -> JGridPlacement {
    match tag {
        1 => JGridPlacement::from_line_index(value as i16),
        2 => JGridPlacement::from_span(value.max(1.0) as u16),
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

/// Shared writer for track-list setters. Reads two parallel int/float arrays
/// from the JNI side and writes the resulting `Vec<JGridTemplateComponent>`.
unsafe fn read_template_tracks(
    env: &mut JNIEnv,
    tags: jni::objects::JIntArray,
    values: jni::objects::JFloatArray,
) -> Option<Vec<JGridTemplateComponent>> {
    let n = env.get_array_length(&tags).ok()? as usize;
    let m = env.get_array_length(&values).ok()? as usize;
    if n != m {
        throw(env, "grid track tag/value arrays have different lengths");
        return None;
    }
    let mut tag_buf: Vec<jint> = vec![0; n];
    let mut val_buf: Vec<jfloat> = vec![0.0; n];
    env.get_int_array_region(&tags, 0, &mut tag_buf).ok()?;
    env.get_float_array_region(&values, 0, &mut val_buf).ok()?;
    Some(
        tag_buf
            .into_iter()
            .zip(val_buf.into_iter())
            .map(|(t, v)| template_component_from_tag(t, v))
            .collect(),
    )
}

unsafe fn read_tracks(
    env: &mut JNIEnv,
    tags: jni::objects::JIntArray,
    values: jni::objects::JFloatArray,
) -> Option<Vec<TrackSizingFunction>> {
    let n = env.get_array_length(&tags).ok()? as usize;
    let m = env.get_array_length(&values).ok()? as usize;
    if n != m {
        throw(env, "grid track tag/value arrays have different lengths");
        return None;
    }
    let mut tag_buf: Vec<jint> = vec![0; n];
    let mut val_buf: Vec<jfloat> = vec![0.0; n];
    env.get_int_array_region(&tags, 0, &mut tag_buf).ok()?;
    env.get_float_array_region(&values, 0, &mut val_buf).ok()?;
    Some(
        tag_buf
            .into_iter()
            .zip(val_buf.into_iter())
            .map(|(t, v)| track_from_tag(t, v))
            .collect(),
    )
}

#[no_mangle]
pub unsafe extern "system" fn Java_dev_dioxus_taffy_NativeBridge_styleSetGridTemplateColumns<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    handle: jlong,
    tags: jni::objects::JIntArray<'local>,
    values: jni::objects::JFloatArray<'local>,
) {
    if let Some(v) = read_template_tracks(&mut env, tags, values) {
        style_from_handle(handle).grid_template_columns = v.into();
    }
}

#[no_mangle]
pub unsafe extern "system" fn Java_dev_dioxus_taffy_NativeBridge_styleSetGridTemplateRows<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    handle: jlong,
    tags: jni::objects::JIntArray<'local>,
    values: jni::objects::JFloatArray<'local>,
) {
    if let Some(v) = read_template_tracks(&mut env, tags, values) {
        style_from_handle(handle).grid_template_rows = v.into();
    }
}

#[no_mangle]
pub unsafe extern "system" fn Java_dev_dioxus_taffy_NativeBridge_styleSetGridAutoColumns<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    handle: jlong,
    tags: jni::objects::JIntArray<'local>,
    values: jni::objects::JFloatArray<'local>,
) {
    if let Some(v) = read_tracks(&mut env, tags, values) {
        style_from_handle(handle).grid_auto_columns = v.into();
    }
}

#[no_mangle]
pub unsafe extern "system" fn Java_dev_dioxus_taffy_NativeBridge_styleSetGridAutoRows<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    handle: jlong,
    tags: jni::objects::JIntArray<'local>,
    values: jni::objects::JFloatArray<'local>,
) {
    if let Some(v) = read_tracks(&mut env, tags, values) {
        style_from_handle(handle).grid_auto_rows = v.into();
    }
}

#[no_mangle]
pub extern "system" fn Java_dev_dioxus_taffy_NativeBridge_styleSetGridColumn(
    _env: JNIEnv,
    _class: JClass,
    handle: jlong,
    start_tag: jint,
    start_val: jfloat,
    end_tag: jint,
    end_val: jfloat,
) {
    style_from_handle(handle).grid_column = Line {
        start: placement_from_tag(start_tag, start_val),
        end: placement_from_tag(end_tag, end_val),
    };
}

#[no_mangle]
pub extern "system" fn Java_dev_dioxus_taffy_NativeBridge_styleSetGridRow(
    _env: JNIEnv,
    _class: JClass,
    handle: jlong,
    start_tag: jint,
    start_val: jfloat,
    end_tag: jint,
    end_val: jfloat,
) {
    style_from_handle(handle).grid_row = Line {
        start: placement_from_tag(start_tag, start_val),
        end: placement_from_tag(end_tag, end_val),
    };
}

#[no_mangle]
pub extern "system" fn Java_dev_dioxus_taffy_NativeBridge_styleSetGridAutoFlow(
    _env: JNIEnv,
    _class: JClass,
    handle: jlong,
    tag: jint,
) {
    style_from_handle(handle).grid_auto_flow = grid_auto_flow_from_tag(tag);
}

