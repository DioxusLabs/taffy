use std::f32;
use std::os::raw::*;
use stretch::geometry::*;
use stretch::node::*;
use stretch::number::*;
use stretch::style::*;

#[repr(C)]
pub struct StretchSize {
    width: f32,
    height: f32,
}

#[repr(C)]
pub struct StretchStyleDimension {
    dimen_type: i32,
    dimen_value: f32,
}

impl Into<Dimension> for StretchStyleDimension {
    fn into(self) -> Dimension {
        match self.dimen_type {
            0 => Dimension::Points(self.dimen_value),
            1 => Dimension::Percent(self.dimen_value),
            2 => Dimension::Auto,
            3 => Dimension::Undefined,
            _ => panic!(),
        }
    }
}

#[repr(C)]
pub struct StretchStyleRect {
    start: StretchStyleDimension,
    end: StretchStyleDimension,
    top: StretchStyleDimension,
    bottom: StretchStyleDimension,
}

#[repr(C)]
pub struct StretchStyleSize {
    width: StretchStyleDimension,
    height: StretchStyleDimension,
}

#[no_mangle]
pub unsafe extern "C" fn stretch_style_create(
    display: i32,
    position_type: i32,
    direction: i32,
    flex_direction: i32,
    flex_wrap: i32,
    overflow: i32,
    align_items: i32,
    align_self: i32,
    align_content: i32,
    justify_content: i32,

    position: StretchStyleRect,
    margin: StretchStyleRect,
    padding: StretchStyleRect,
    border: StretchStyleRect,

    flex_grow: f32,
    flex_shrink: f32,

    flex_basis: StretchStyleDimension,

    size: StretchStyleSize,
    min_size: StretchStyleSize,
    max_size: StretchStyleSize,

    aspect_ratio: f32,
) -> *mut c_void {
    Box::into_raw(Box::new(Style {
        display: match display {
            0 => Display::Flex,
            1 => Display::None,
            _ => panic!(),
        },

        position_type: match position_type {
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

        flex_direction: match flex_direction {
            0 => FlexDirection::Row,
            1 => FlexDirection::Column,
            2 => FlexDirection::RowReverse,
            3 => FlexDirection::ColumnReverse,
            _ => panic!(),
        },

        flex_wrap: match flex_wrap {
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

        align_items: match align_items {
            0 => AlignItems::FlexStart,
            1 => AlignItems::FlexEnd,
            2 => AlignItems::Center,
            3 => AlignItems::Baseline,
            4 => AlignItems::Stretch,
            _ => panic!(),
        },

        align_self: match align_self {
            0 => AlignSelf::Auto,
            1 => AlignSelf::FlexStart,
            2 => AlignSelf::FlexEnd,
            3 => AlignSelf::Center,
            4 => AlignSelf::Baseline,
            5 => AlignSelf::Stretch,
            _ => panic!(),
        },

        align_content: match align_content {
            0 => AlignContent::FlexStart,
            1 => AlignContent::FlexEnd,
            2 => AlignContent::Center,
            3 => AlignContent::Stretch,
            4 => AlignContent::SpaceBetween,
            5 => AlignContent::SpaceAround,
            _ => panic!(),
        },

        justify_content: match justify_content {
            0 => JustifyContent::FlexStart,
            1 => JustifyContent::FlexEnd,
            2 => JustifyContent::Center,
            3 => JustifyContent::SpaceBetween,
            4 => JustifyContent::SpaceAround,
            5 => JustifyContent::SpaceEvenly,
            _ => panic!(),
        },

        position: Rect {
            start: position.start.into(),
            end: position.end.into(),
            top: position.top.into(),
            bottom: position.bottom.into(),
        },

        margin: Rect {
            start: margin.start.into(),
            end: margin.end.into(),
            top: margin.top.into(),
            bottom: margin.bottom.into(),
        },

        padding: Rect {
            start: padding.start.into(),
            end: padding.end.into(),
            top: padding.top.into(),
            bottom: padding.bottom.into(),
        },

        border: Rect {
            start: border.start.into(),
            end: border.end.into(),
            top: border.top.into(),
            bottom: border.bottom.into(),
        },

        flex_grow,
        flex_shrink,

        flex_basis: flex_basis.into(),

        size: Size { width: size.width.into(), height: size.height.into() },
        min_size: Size { width: min_size.width.into(), height: min_size.height.into() },
        max_size: Size { width: max_size.width.into(), height: max_size.height.into() },

        aspect_ratio: if f32::is_nan(aspect_ratio) { Number::Undefined } else { Number::Defined(aspect_ratio) },
    })) as *mut c_void
}

#[no_mangle]
pub unsafe extern "C" fn stretch_style_free(style: *mut c_void) {
    let _style = Box::from_raw(style as *mut Style);
}

#[no_mangle]
pub unsafe extern "C" fn stretch_init() -> *mut c_void {
    let stretch = stretch::node::Stretch::new();
    Box::into_raw(Box::new(stretch)) as *mut c_void
}

#[no_mangle]
pub unsafe extern "C" fn stretch_free(stretch: *mut c_void) {
    let _stretch = Box::from_raw(stretch as *mut Stretch);
}

#[no_mangle]
pub unsafe extern "C" fn stretch_node_create(stretch: *mut c_void, style: *mut c_void) -> *mut c_void {
    let mut stretch = Box::from_raw(stretch as *mut Stretch);
    let style = Box::from_raw(style as *mut Style);
    let node = stretch.new_node(*style, &[]).unwrap();

    Box::leak(style);
    Box::leak(stretch);

    Box::into_raw(Box::new(node)) as *mut c_void
}

#[no_mangle]
pub unsafe extern "C" fn stretch_node_free(stretch: *mut c_void, node: *mut c_void) {
    let mut stretch = Box::from_raw(stretch as *mut Stretch);
    let node = Box::from_raw(node as *mut Node);

    stretch.remove(*node);

    Box::leak(stretch);
}

#[no_mangle]
pub unsafe extern "C" fn stretch_node_set_measure(
    stretch: *mut c_void,
    node: *mut c_void,
    swift_ptr: *mut c_void,
    measure: fn(*const c_void, f32, f32) -> StretchSize,
) {
    let mut stretch = Box::from_raw(stretch as *mut Stretch);
    let node = Box::from_raw(node as *mut Node);

    stretch
        .set_measure(
            *node,
            Some(stretch::node::MeasureFunc::Boxed(Box::new(move |constraint| {
                let size = measure(swift_ptr, constraint.width.or_else(f32::NAN), constraint.height.or_else(f32::NAN));
                size
            }))),
        )
        .unwrap();

    Box::leak(stretch);
    Box::leak(node);
}

#[no_mangle]
pub unsafe extern "C" fn stretch_node_set_style(stretch: *mut c_void, node: *mut c_void, style: *mut c_void) {
    let mut stretch = Box::from_raw(stretch as *mut Stretch);
    let node = Box::from_raw(node as *mut Node);
    let style = Box::from_raw(style as *mut Style);

    stretch.set_style(*node, *style).unwrap();

    Box::leak(stretch);
    Box::leak(node);
    Box::leak(style);
}

#[no_mangle]
pub unsafe extern "C" fn stretch_node_dirty(stretch: *mut c_void, node: *mut c_void) -> bool {
    let stretch = Box::from_raw(stretch as *mut Stretch);
    let node = Box::from_raw(node as *mut Node);
    let dirty = stretch.dirty(*node).unwrap();

    Box::leak(stretch);
    Box::leak(node);

    dirty
}

#[no_mangle]
pub unsafe extern "C" fn stretch_node_mark_dirty(stretch: *mut c_void, node: *mut c_void) {
    let mut stretch = Box::from_raw(stretch as *mut Stretch);
    let node = Box::from_raw(node as *mut Node);

    stretch.mark_dirty(*node).unwrap();

    Box::leak(stretch);
    Box::leak(node);
}

#[no_mangle]
pub unsafe extern "C" fn stretch_node_add_child(stretch: *mut c_void, node: *mut c_void, child: *mut c_void) {
    let mut stretch = Box::from_raw(stretch as *mut Stretch);
    let node = Box::from_raw(node as *mut Node);
    let child = Box::from_raw(child as *mut Node);

    stretch.add_child(*node, *child).unwrap();

    Box::leak(stretch);
    Box::leak(node);
    Box::leak(child);
}

#[no_mangle]
pub unsafe extern "C" fn stretch_node_replace_child_at_index(
    stretch: *mut c_void,
    node: *mut c_void,
    index: usize,
    child: *mut c_void,
) {
    let mut stretch = Box::from_raw(stretch as *mut Stretch);
    let node = Box::from_raw(node as *mut Node);
    let child = Box::from_raw(child as *mut Node);

    stretch.replace_child_at_index(*node, index, *child).unwrap();

    Box::leak(stretch);
    Box::leak(node);
    Box::leak(child);
}

#[no_mangle]
pub unsafe extern "C" fn stretch_node_remove_child(stretch: *mut c_void, node: *mut c_void, child: *mut c_void) {
    let mut stretch = Box::from_raw(stretch as *mut Stretch);
    let node = Box::from_raw(node as *mut Node);
    let child = Box::from_raw(child as *mut Node);

    stretch.remove_child(*node, *child).unwrap();

    Box::leak(stretch);
    Box::leak(node);
    Box::leak(child);
}

#[no_mangle]
pub unsafe extern "C" fn stretch_node_remove_child_at_index(stretch: *mut c_void, node: *mut c_void, index: usize) {
    let mut stretch = Box::from_raw(stretch as *mut Stretch);
    let node = Box::from_raw(node as *mut Node);

    stretch.remove_child_at_index(*node, index).unwrap();

    Box::leak(stretch);
    Box::leak(node);
}

#[no_mangle]
pub unsafe extern "C" fn stretch_node_compute_layout(
    stretch: *mut c_void,
    node: *mut c_void,
    width: f32,
    height: f32,
    create_layout: fn(*const f32) -> *mut c_void,
) -> *mut c_void {
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

    Box::leak(stretch);
    Box::leak(node);

    create_layout(output.as_ptr())
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
