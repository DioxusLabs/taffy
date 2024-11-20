use crate::collections::f_get_list;
use crate::enums::{
    f_get_align_content, f_get_align_items, f_get_box_sizing, f_get_display, f_get_flex_direction, f_get_flex_wrap,
    f_get_grid_auto_flow, f_get_position, f_get_text_align, get_overflow,
};
use crate::geom::{f_get_line, f_get_point, f_get_rect, f_get_size, get_opt_non_repeated_track_sizing_function};
use crate::measure::{
    f_get_dimension_or, get_dimension, get_grid_placement, get_length_percentage, get_length_percentage_auto,
    get_opt_track_sizing_function,
};
use crate::primitives::{f_bool_from_primitive, f_f32_from_primitive, f_opt_f32_from_object};
use jni::objects::{JObject, JValueOwned};
use jni::JNIEnv;
use taffy::{Dimension, GridPlacement, Line, Overflow, Point, Rect, Size, Style};

pub unsafe fn get_style<'local>(env: &mut JNIEnv<'local>, style: &JObject<'local>) -> Style {
    let display = f_get_display(env, style, "display");
    let item_is_table = f_bool_from_primitive(env, style, "itemIsTable", || false);
    let box_sizing = f_get_box_sizing(env, style, "boxSizing");

    let overflow =
        f_get_point(env, style, "overflow", get_overflow, || Point { x: Overflow::Visible, y: Overflow::Visible });
    let scrollbar_width = f_f32_from_primitive(env, style, "scrollbarWidth", || 0.0);

    let position = f_get_position(env, style, "position");
    let inset = f_get_rect(env, style, "inset", get_length_percentage_auto, Rect::auto);

    let size = f_get_size(env, style, "size", get_dimension, Size::auto);
    let min_size = f_get_size(env, style, "minSize", get_dimension, Size::auto);
    let max_size = f_get_size(env, style, "maxSize", get_dimension, Size::auto);
    let aspect_ratio = f_opt_f32_from_object(env, style, "aspectRatio", || None);

    let margin = f_get_rect(env, style, "margin", get_length_percentage_auto, Rect::zero);
    let padding = f_get_rect(env, style, "padding", get_length_percentage, Rect::zero);
    let border = f_get_rect(env, style, "border", get_length_percentage, Rect::zero);

    let align_items = f_get_align_items(env, style, "alignItems");
    let align_self = f_get_align_items(env, style, "alignSelf");
    let justify_items = f_get_align_items(env, style, "justifyItems");
    let justify_self = f_get_align_items(env, style, "justifySelf");
    let align_content = f_get_align_content(env, style, "alignContent");
    let justify_content = f_get_align_content(env, style, "justifyContent");
    let gap = f_get_size(env, style, "gap", get_length_percentage, Size::zero);

    let text_align = f_get_text_align(env, style, "textAlign");
    let flex_direction = f_get_flex_direction(env, style, "flexDirection");
    let flex_wrap = f_get_flex_wrap(env, style, "flexWrap");

    let flex_basis = f_get_dimension_or(env, style, "flexBasis", || Dimension::Auto);
    let flex_grow = f_f32_from_primitive(env, style, "flexGrow", || 0.0);
    let flex_shrink = f_f32_from_primitive(env, style, "flexShrink", || 1.0);

    let grid_template_rows = f_get_list(env, style, "gridTemplateRows", get_opt_track_sizing_function);
    let grid_template_columns = f_get_list(env, style, "gridTemplateColumns", get_opt_track_sizing_function);
    let grid_auto_rows = f_get_list(env, style, "gridAutoRows", get_opt_non_repeated_track_sizing_function);
    let grid_auto_columns = f_get_list(env, style, "gridAutoColumns", get_opt_non_repeated_track_sizing_function);
    let grid_auto_flow = f_get_grid_auto_flow(env, style, "gridAutoFlow");

    let grid_row = f_get_line(env, style, "gridRow", get_grid_placement, || Line {
        start: GridPlacement::Auto,
        end: GridPlacement::Auto,
    });

    let grid_column = f_get_line(env, style, "gridColumn", get_grid_placement, || Line {
        start: GridPlacement::Auto,
        end: GridPlacement::Auto,
    });

    Style {
        display,
        item_is_table,
        box_sizing,

        overflow,
        scrollbar_width,

        position,
        inset,

        size,
        min_size,
        max_size,
        aspect_ratio,

        margin,
        padding,
        border,

        align_items,
        align_self,
        justify_items,
        justify_self,
        align_content,
        justify_content,
        gap,

        text_align,
        flex_direction,
        flex_wrap,

        flex_basis,
        flex_grow,
        flex_shrink,

        grid_template_rows,
        grid_template_columns,
        grid_auto_rows,
        grid_auto_columns,
        grid_auto_flow,

        grid_row,
        grid_column,
    }
}

pub fn f_get_value<'local>(
    env: &mut JNIEnv<'local>,
    object: &JObject<'local>,
    field: &str,
    jtype: &str,
) -> JValueOwned<'local> {
    env.get_field(object, field, jtype).unwrap_or_else(|_| panic!("Couldn't get field {}, {}", field, jtype))
}
