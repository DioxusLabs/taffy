use std::f32;
use std::os::raw::*;
use stretch::node::*;
use stretch::style::*;
use stretch::result::{Layout};
use stretch::geometry::*;
use stretch::number::*;

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
            _ => panic!()
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
pub extern "C" fn stretch_style_create(
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

    aspect_ratio: f32
) -> *mut c_void {
    Box::into_raw(Box::new(Style {
        display: match display {
            0 => Display::Flex,
            1 => Display::None,
            _ => panic!()
        },

        position_type: match position_type {
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

        flex_direction: match flex_direction {
            0 => FlexDirection::Row,
            1 => FlexDirection::Column,
            2 => FlexDirection::RowReverse,
            3 => FlexDirection::ColumnReverse,
            _ => panic!()
        },

        flex_wrap: match flex_wrap {
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

        align_items: match align_items {
            0 => AlignItems::FlexStart,
            1 => AlignItems::FlexEnd,
            2 => AlignItems::Center,
            3 => AlignItems::Baseline,
            4 => AlignItems::Stretch,
            _ => panic!()
        },

        align_self: match align_self {
            0 => AlignSelf::Auto,
            1 => AlignSelf::FlexStart,
            2 => AlignSelf::FlexEnd,
            3 => AlignSelf::Center,
            4 => AlignSelf::Baseline,
            5 => AlignSelf::Stretch,
            _ => panic!()
        },

        align_content: match align_content {
            0 => AlignContent::FlexStart,
            1 => AlignContent::FlexEnd,
            2 => AlignContent::Center,
            3 => AlignContent::Stretch,
            4 => AlignContent::SpaceBetween,
            5 => AlignContent::SpaceAround,
            _ => panic!()
        },

        justify_content: match justify_content {
            0 => JustifyContent::FlexStart,
            1 => JustifyContent::FlexEnd,
            2 => JustifyContent::Center,
            3 => JustifyContent::SpaceBetween,
            4 => JustifyContent::SpaceAround,
            5 => JustifyContent::SpaceEvenly,
            _ => panic!()
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

        aspect_ratio: if f32::is_nan(aspect_ratio) {
            Number::Undefined
        } else {
            Number::Defined(aspect_ratio)
        },
    })) as *mut c_void
}

#[no_mangle]
pub extern "C" fn stretch_style_free(style: *mut c_void) {
    let _style = unsafe { Box::from_raw(style as *mut Style) };
}

#[no_mangle]
pub extern "C" fn stretch_node_create(style: *mut c_void) -> *mut c_void {
    let style = unsafe { Box::from_raw(style as *mut Style) };
    Box::into_raw(Box::new(Node::new(*Box::leak(style), vec![]))) as *mut c_void
}

#[no_mangle]
pub extern "C" fn stretch_node_free(node: *mut c_void) {
    let _node = unsafe { Box::from_raw(node as *mut Node) };
}

#[no_mangle]
pub extern "C" fn stretch_node_set_measure(node: *mut c_void, swift_ptr: *mut c_void, measure: fn(*const c_void, f32, f32) -> StretchSize) {
    let node = unsafe { Box::from_raw(node as *mut Node) };
    Box::leak(node).set_measure(Some(Box::new(move |constraint| {
        let size = measure(
            swift_ptr,
            constraint.width.or_else(f32::NAN),
            constraint.height.or_else(f32::NAN),
        );
        Ok(Size { width: size.width, height: size.height })
    })));
}

#[no_mangle]
pub extern "C" fn stretch_node_set_style(node: *mut c_void, style: *mut c_void) {
    let node = unsafe { Box::from_raw(node as *mut Node) };
    let style = unsafe { Box::from_raw(style as *mut Style) };
    Box::leak(node).set_style(*Box::leak(style));
}

#[no_mangle]
pub extern "C" fn stretch_node_dirty(node: *mut c_void) -> bool {
    let node = unsafe { Box::from_raw(node as *mut Node) };
    Box::leak(node).dirty()
}

#[no_mangle]
pub extern "C" fn stretch_node_mark_dirty(node: *mut c_void) {
    let node = unsafe { Box::from_raw(node as *mut Node) };
    Box::leak(node).mark_dirty();
}

#[no_mangle]
pub extern "C" fn stretch_node_add_child(node: *mut c_void, child: *mut c_void) {
    let node = unsafe { Box::from_raw(node as *mut Node) };
    let child = unsafe { Box::from_raw(child as *mut Node) };
    Box::leak(node).add_child(Box::leak(child));
}

#[no_mangle]
pub extern "C" fn stretch_node_replace_child_at_index(node: *mut c_void, index: usize, child: *mut c_void) {
    let node = unsafe { Box::from_raw(node as *mut Node) };
    let child = unsafe { Box::from_raw(child as *mut Node) };
    Box::leak(node).replace_child_at_index(index, Box::leak(child));

}

#[no_mangle]
pub extern "C" fn stretch_node_remove_child(node: *mut c_void, child: *mut c_void) {
    let node = unsafe { Box::from_raw(node as *mut Node) };
    let child = unsafe { Box::from_raw(child as *mut Node) };
    Box::leak(node).remove_child(Box::leak(child));
}

#[no_mangle]
pub extern "C" fn stretch_node_remove_child_at_index(node: *mut c_void, index: usize) {
    let node = unsafe { Box::from_raw(node as *mut Node) };
    Box::leak(node).remove_child_at_index(index);
}

#[no_mangle]
pub extern "C" fn stretch_node_compute_layout(node: *mut c_void, width: f32, height: f32, create_layout: fn(*const f32) -> *mut c_void) -> *mut c_void {
    let node = unsafe { Box::from_raw(node as *mut Node) };
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

    create_layout(output.as_ptr())
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
