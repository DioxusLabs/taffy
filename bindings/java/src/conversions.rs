use jni::JNIEnv;
use jni::objects::{JObject, JValue};
use taffy::{AlignContent, AlignItems, BoxSizing, Dimension, Display, FlexDirection, FlexWrap, GridAutoFlow, GridPlacement, GridTrackRepetition, LengthPercentage, LengthPercentageAuto, Line, MaxTrackSizingFunction, MinTrackSizingFunction, NonRepeatedTrackSizingFunction, Overflow, Point, Position, Rect, Size, Style, TextAlign, TrackSizingFunction};
use taffy::style_helpers::{TaffyGridLine, TaffyGridSpan};

pub(crate) fn get_style<'local>(env: &mut JNIEnv<'local>, style: &JObject<'local>) -> Style {
    let display = get_enum_equiv(env, style, "display", "Lcom/dioxuslabs/taffy/style/TaffyDisplay;", get_display);
    let item_is_table = get_field_bool(env, style, "itemIsTable", false);
    let box_sizing = get_enum_equiv(env, style, "boxSizing", "Lcom/dioxuslabs/taffy/style/TaffyBoxSizing;", get_box_sizing);

    let overflow = get_field_point(env, style, "overflow", get_overflow, || Overflow::default());
    let scrollbar_width = get_field_float(env, style, "scrollbarWidth", 0.0);

    let position = get_enum_equiv(env, style, "position", "Lcom/dioxuslabs/taffy/style/TaffyPosition;", get_position);
    let inset = get_field_rect(env, style, "inset", get_length_percentage_auto, || LengthPercentageAuto::Auto);

    let size = get_field_size(env, style, "size", get_dimension, || Dimension::Auto);
    let min_size = get_field_size(env, style, "minSize", get_dimension, || Dimension::Auto);
    let max_size = get_field_size(env, style, "maxSize", get_dimension, || Dimension::Auto);
    let aspect_ratio = get_float(env, style, "aspectRatio", || None);

    let margin = get_field_rect(env, style, "margin", get_length_percentage_auto, || LengthPercentageAuto::Length(0.0));
    let padding = get_field_rect(env, style, "padding", get_length_percentage, || LengthPercentage::Length(0.0));
    let border = get_field_rect(env, style, "border", get_length_percentage, || LengthPercentage::Length(0.0));

    let align_items = get_align_items(env, style, "alignItems");
    let align_self = get_align_items(env, style, "alignSelf");
    let justify_items = get_align_items(env, style, "justifyItems");
    let justify_self = get_align_items(env, style, "justifySelf");
    let align_content = get_align_content(env, style, "alignContent");
    let justify_content = get_align_content(env, style, "justifyContent");
    let gap = get_field_size(env, style, "gap", get_length_percentage, || LengthPercentage::Length(0.0));

    let text_align = get_enum_equiv(env, style, "textAlign", "Lcom/dioxuslabs/taffy/style/TaffyTextAlign;", get_text_align);
    let flex_direction = get_enum_equiv(env, style, "flexDirection", "Lcom/dioxuslabs/taffy/style/TaffyFlexDirection;", get_flex_direction);
    let flex_wrap = get_enum_equiv(env, style, "flexWrap", "Lcom/dioxuslabs/taffy/style/TaffyFlexWrap;", get_flex_wrap);

    let flex_basis = resolve_field(env, style, "flexBasis", "Lcom/dioxuslabs/taffy/geom/measure/TaffyDimension;", get_dimension, || Dimension::Auto);
    let flex_grow = get_field_float(env, style, "flexGrow", 0.0);
    let flex_shrink = get_field_float(env, style, "flexShrink", 1.0);

    let grid_template_rows = get_field_list(env, style, "gridTemplateRows", get_track_sizing_function);
    let grid_template_columns = get_field_list(env, style, "gridTemplateColumns", get_track_sizing_function);
    let grid_auto_rows = get_field_list(env, style, "gridAutoRows", get_non_repeated_track_sizing_function);
    let grid_auto_columns = get_field_list(env, style, "gridAutoColumns", get_non_repeated_track_sizing_function);
    let grid_auto_flow = get_enum_equiv(env, style, "gridAutoFlow", "Lcom/dioxuslabs/taffy/style/TaffyGridAutoFlow;", get_grid_auto_flow);

    let grid_row = get_field_line(env, style, "gridRow", get_grid_placement, || GridPlacement::Auto);
    let grid_column = get_field_line(env, style, "gridColumn", get_grid_placement, || GridPlacement::Auto);

    println!("Inset: {:?}", inset);

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
        grid_column
    }
}

fn get_float<'local>(env: &mut JNIEnv<'local>, object: &JObject<'local>, field: &str, def: fn() -> Option<f32>) -> Option<f32> {
    let float_obj = get_field_as(env, object, field, "Ljava/lang/Float;");

    if float_obj.is_null() {
        return def();
    }

    Some(
        env
            .call_method(float_obj, "floatValue", "()F", &[])
            .expect("Couldn't call floatValue on java/lang/Float")
            .f()
            .unwrap() as f32
    )
}

fn get_field_point<'local, T, F>(env: &mut JNIEnv<'local>, object: &JObject<'local>, field: &str, f: F, def: fn() -> T) -> Point<T>
where
    F: Fn(&mut JNIEnv<'local>, &JObject<'local>) -> T,
{
    let point_field = &get_field_as(env, object, field, "Lcom/dioxuslabs/taffy/geom/TaffyPoint;");
    if point_field.is_null() {
        return Point {
            x: def(),
            y: def(),
        };
    }

    let x_field = &get_field_as(env, point_field, "x", "Ljava/lang/Object;");
    let y_field = &get_field_as(env, point_field, "y", "Ljava/lang/Object;");

    Point {
        x: f(env, x_field, def),
        y: f(env, y_field, def),
    }
}

fn get_field_line<'local, T, F>(env: &mut JNIEnv<'local>, object: &JObject<'local>, field: &str, f: F, def: fn() -> T) -> Line<T>
where
    F: Fn(&mut JNIEnv<'local>, &JObject<'local>) -> T,
{
    let line_field = &get_field_as(env, object, field, "Lcom/dioxuslabs/taffy/geom/TaffyLine;");
    if line_field.is_null() {
        return Line {
            start: def(),
            end: def(),
        };
    }

    let s_field = &get_field_as(env, line_field, "start", "Ljava/lang/Object;");
    let e_field = &get_field_as(env, line_field, "end", "Ljava/lang/Object;");

    Line {
        start: f(env, s_field, def),
        end: f(env, e_field, def),
    }
}

fn get_field_list<'local, T, F>(env: &mut JNIEnv<'local>, object: &JObject<'local>, field: &str, f: F) -> Vec<T>
where
    F: Fn(&mut JNIEnv<'local>, &JObject<'local>) -> T,
{
    let list_field = &get_field_as(env, object, field, "Ljava/util/List;");

    if list_field.is_null() {
        return Vec::new()
    }

    let list_size = env
        .call_method(list_field, "size", "()I", &[])
        .expect("Couldn't call size on List")
        .i()
        .unwrap();

    let mut objects = Vec::new();
    for i in 0..list_size {
        let object = env
            .call_method(list_field, "get", "(I)Ljava/lang/Object;", &[JValue::from(i)])
            .expect("Couldn't call get on List")
            .l()
            .unwrap();

        objects.push(f(env, &object));
    }

    objects
}

fn get_field_size<'local, T, F>(env: &mut JNIEnv<'local>, object: &JObject<'local>, field: &str, f: F, def: fn() -> T) -> Size<T>
where
    F: Fn(&mut JNIEnv<'local>, &JObject<'local>) -> T,
{
    let point_field = &get_field_as(env, object, field, "Lcom/dioxuslabs/taffy/geom/TaffySize;");
    if point_field.is_null() {
        return Size {
            width: def(),
            height: def(),
        };
    }

    let w_field = &get_field_as(env, point_field, "width", "Ljava/lang/Object;");
    let h_field = &get_field_as(env, point_field, "height", "Ljava/lang/Object;");

    Size {
        width: f(env, w_field, def),
        height: f(env, h_field, def),
    }
}

fn get_field_rect<'local, T, F>(env: &mut JNIEnv<'local>, object: &JObject<'local>, field: &str, f: F, def: fn() -> T) -> Rect<T>
where
    F: Fn(&mut JNIEnv<'local>, &JObject<'local>, fn() -> T) -> T,
{
    let point_field = &get_field_as(env, object, field, "Lcom/dioxuslabs/taffy/geom/TaffyRect;");
    if point_field.is_null() {
        return Rect {
            left: def(),
            right: def(),
            top: def(),
            bottom: def(),
        };
    }

    let l_field = &get_field_as(env, point_field, "left", "Ljava/lang/Object;");
    let r_field = &get_field_as(env, point_field, "right", "Ljava/lang/Object;");
    let t_field = &get_field_as(env, point_field, "top", "Ljava/lang/Object;");
    let b_field = &get_field_as(env, point_field, "bottom", "Ljava/lang/Object;");

    Rect {
        left: f(env, l_field, def),
        right: f(env, r_field, def),
        top: f(env, t_field, def),
        bottom: f(env, b_field, def),
    }
}

fn get_field_as<'local>(env: &mut JNIEnv<'local>, object: &JObject<'local>, field: &str, jtype: &str) -> JObject<'local> {
    env
        .get_field(object, field, jtype)
        .expect("Couldn't get field")
        .l()
        .unwrap()
}

fn resolve_field<'local, T, F>(env: &mut JNIEnv<'local>, object: &JObject<'local>, field: &str, jtype: &str, f: F, def: fn() -> T) -> T
where
    F: Fn(&mut JNIEnv<'local>, &JObject<'local>, fn() -> T) -> T,
{
    let field = env
        .get_field(object, field, jtype)
        .expect("Couldn't get field")
        .l()
        .unwrap();

    f(env, &field, def)
}

fn get_field_bool(env: &mut JNIEnv, object: &JObject, field: &str, def: bool) -> bool {
    env
        .get_field(object, field, "Z")
        .expect("Couldn't get field")
        .z()
        .unwrap_or(def)
}

fn get_field_float(env: &mut JNIEnv, object: &JObject, field: &str, def: f32) -> f32 {
    env
        .get_field(object, field, "F")
        .expect("Couldn't get field")
        .f()
        .unwrap_or(def)
}

fn get_enum_value(env: &mut JNIEnv, object: &JObject) -> i32 {
    env
        .get_field(object, "internal", "I")
        .expect("Couldn't get field")
        .i()
        .unwrap()
}

fn get_enum_equiv<'local, T, F>(env: &mut JNIEnv<'local>, style: &JObject<'local>, field: &str, jtype: &str,
                                f: F) -> T where
    F: Fn(&mut JNIEnv<'local>, &JObject<'local>) -> T,
{
    let field = &get_field_as(env, style, field, jtype);
    f(env, field)
}

fn get_display<'local>(env: &mut JNIEnv<'local>, display: &JObject<'local>) -> Display {
    if display.is_null() {
        return Display::default();
    }

    let internal = get_enum_value(env, display);

    match internal {
        0 => Display::Block,
        1 => Display::Flex,
        2 => Display::Grid,
        _ => Display::None,
    }
}

fn get_box_sizing<'local>(env: &mut JNIEnv<'local>, box_sizing: &JObject<'local>) -> BoxSizing {
    if box_sizing.is_null() {
        return BoxSizing::default();
    }

    let internal = get_enum_value(env, box_sizing);

    match internal {
        0 => BoxSizing::BorderBox,
        _ => BoxSizing::ContentBox
    }
}

fn get_length_percentage_auto<'local>(env: &mut JNIEnv<'local>,
                                      length_percentage_auto: &JObject<'local>,
                                      def: fn() -> LengthPercentageAuto) -> LengthPercentageAuto {
    if length_percentage_auto.is_null() {
        return def();
    }

    let internal = env
        .get_field(length_percentage_auto, "type", "B")
        .expect("Couldn't get field")
        .b()
        .unwrap() as i8;

    match internal {
        0 => LengthPercentageAuto::Length(
            env
                .get_field(length_percentage_auto, "value", "F")
                .expect("Couldn't get field")
                .f()
                .unwrap() as f32
        ),
        1 => LengthPercentageAuto::Percent(
            env
                .get_field(length_percentage_auto, "value", "F")
                .expect("Couldn't get field")
                .f()
                .unwrap() as f32
        ),
        _ => LengthPercentageAuto::Auto
    }
}

fn get_non_repeated_track_sizing_function<'local>(env: &mut JNIEnv<'local>,
                                                  non_repeated_track_sizing_function: &JObject<'local>,
                                                  def: fn() -> NonRepeatedTrackSizingFunction) -> NonRepeatedTrackSizingFunction {
    if non_repeated_track_sizing_function.is_null() {
        return def();
    }

    let n_field = &get_field_as(env, non_repeated_track_sizing_function, "min", "Lcom/dioxuslabs/taffy/geom/grid/TaffyMinTrackSizingFunction;");
    let x_field = &get_field_as(env, non_repeated_track_sizing_function, "max", "Lcom/dioxuslabs/taffy/geom/grid/TaffyMaxTrackSizingFunction;");

    let min = get_min_track_sizing_function(env, n_field, || MinTrackSizingFunction::Auto);
    let max = get_max_track_sizing_function(env, x_field, || MaxTrackSizingFunction::Auto);

    NonRepeatedTrackSizingFunction {
        min,
        max,
    }
}

fn get_track_sizing_function<'local>(env: &mut JNIEnv<'local>,
                                     track_sizing_function: &JObject<'local>,
                                     def: fn() -> TrackSizingFunction) -> TrackSizingFunction {
    if track_sizing_function.is_null() {
        return def();
    }

    let internal = env
        .get_field(track_sizing_function, "type", "B")
        .expect("Couldn't get field")
        .b()
        .unwrap() as i8;

    match internal {
        0 => TrackSizingFunction::Single({
            let func_field = &get_field_as(env, track_sizing_function, "func", "Lcom/dioxuslabs/taffy/geom/grid/TaffyNonRepeatedTrackSizingFunction;");

            get_non_repeated_track_sizing_function(
                env,
                func_field,
                || NonRepeatedTrackSizingFunction {
                    min: MinTrackSizingFunction::Auto,
                    max: MaxTrackSizingFunction::Auto,
                },
            )
        }),
        _ => TrackSizingFunction::Repeat(
            {
                let reps_field = &get_field_as(env, track_sizing_function, "repetitions", "Lcom/dioxuslabs/taffy/geom/grid/TaffyGridTrackRepetition;");
                get_grid_track_repetition(env, reps_field, || GridTrackRepetition::AutoFill)
            },
            {
                get_field_list(env, track_sizing_function, "functions", |env, func| {
                    let func_field = &get_field_as(env, func, "func", "Lcom/dioxuslabs/taffy/geom/grid/TaffyNonRepeatedTrackSizingFunction;");

                    get_non_repeated_track_sizing_function(
                        env,
                        func_field,
                        || NonRepeatedTrackSizingFunction {
                            min: MinTrackSizingFunction::Auto,
                            max: MaxTrackSizingFunction::Auto,
                        },
                    )
                })
            },
        )
    }
}

fn get_min_track_sizing_function<'local>(env: &mut JNIEnv<'local>,
                                         min_track_sizing_function: &JObject<'local>,
                                         def: fn() -> MinTrackSizingFunction) -> MinTrackSizingFunction {
    if min_track_sizing_function.is_null() {
        return def();
    }

    let internal = env
        .get_field(min_track_sizing_function, "type", "B")
        .expect("Couldn't get field")
        .b()
        .unwrap() as i8;

    match internal {
        0 => MinTrackSizingFunction::Fixed(
            resolve_field(
                env,
                min_track_sizing_function,
                "value",
                "Lcom/dioxuslabs/taffy/geom/measure/TaffyLengthPercentage;",
                get_length_percentage,
                || LengthPercentage::Length(0.0),
            )
        ),
        1 => MinTrackSizingFunction::MinContent,
        2 => MinTrackSizingFunction::MaxContent,
        _ => MinTrackSizingFunction::Auto
    }
}

fn get_max_track_sizing_function<'local>(env: &mut JNIEnv<'local>,
                                         min_track_sizing_function: &JObject<'local>,
                                         def: fn() -> MaxTrackSizingFunction) -> MaxTrackSizingFunction {
    if min_track_sizing_function.is_null() {
        return def();
    }

    let internal = env
        .get_field(min_track_sizing_function, "type", "B")
        .expect("Couldn't get field")
        .b()
        .unwrap() as i8;

    match internal {
        0 => MaxTrackSizingFunction::Fixed(
            resolve_field(
                env,
                min_track_sizing_function,
                "value",
                "Ljava/lang/Object;",
                get_length_percentage,
                || LengthPercentage::Length(0.0),
            )
        ),
        1 => MaxTrackSizingFunction::MinContent,
        2 => MaxTrackSizingFunction::MaxContent,
        3 => MaxTrackSizingFunction::FitContent(
            resolve_field(
                env,
                min_track_sizing_function,
                "value",
                "Ljava/lang/Object;",
                get_length_percentage,
                || LengthPercentage::Length(0.0),
            )
        ),
        4 => MaxTrackSizingFunction::Auto,
        _ => MaxTrackSizingFunction::Fraction(
            env
                .get_field(min_track_sizing_function, "value", "Ljava/lang/Object;")
                .expect("Couldn't get field")
                .f()
                .unwrap() as f32
        ),
    }
}

fn get_dimension<'local>(env: &mut JNIEnv<'local>,
                         dimension: &JObject<'local>,
                         def: fn() -> Dimension) -> Dimension {
    if dimension.is_null() {
        return def();
    }

    let internal = env
        .get_field(dimension, "type", "B")
        .expect("Couldn't get field")
        .b()
        .unwrap() as i8;

    match internal {
        0 => Dimension::Length(
            env
                .get_field(dimension, "value", "F")
                .expect("Couldn't get field")
                .f()
                .unwrap() as f32
        ),
        1 => Dimension::Percent(
            env
                .get_field(dimension, "value", "F")
                .expect("Couldn't get field")
                .f()
                .unwrap() as f32
        ),
        _ => Dimension::Auto
    }
}

fn get_grid_track_repetition<'local>(env: &mut JNIEnv<'local>,
                                     grid_track_repetition: &JObject<'local>,
                                     def: fn() -> GridTrackRepetition) -> GridTrackRepetition {
    if grid_track_repetition.is_null() {
        return def();
    }

    let internal = env
        .get_field(grid_track_repetition, "type", "B")
        .expect("Couldn't get field")
        .b()
        .unwrap() as i8;

    match internal {
        0 => GridTrackRepetition::AutoFill,
        1 => GridTrackRepetition::AutoFit,
        _ => GridTrackRepetition::Count(
            env
                .get_field(grid_track_repetition, "value", "S")
                .expect("Couldn't get field")
                .s()
                .unwrap() as u16
        )
    }
}

fn get_length_percentage<'local>(env: &mut JNIEnv<'local>,
                                 length_percentage: &JObject<'local>,
                                 def: fn() -> LengthPercentage) -> LengthPercentage {
    if length_percentage.is_null() {
        return def();
    }

    let internal = env
        .get_field(length_percentage, "type", "B")
        .expect("Couldn't get field")
        .b()
        .unwrap() as i8;

    match internal {
        0 => LengthPercentage::Length(
            env
                .get_field(length_percentage, "value", "F")
                .expect("Couldn't get field")
                .f()
                .unwrap() as f32
        ),
        _ => LengthPercentage::Percent(
            env
                .get_field(length_percentage, "value", "F")
                .expect("Couldn't get field")
                .f()
                .unwrap() as f32
        ),
    }
}

fn get_grid_placement<'local>(env: &mut JNIEnv<'local>,
                                 grid_placement: &JObject<'local>,
                                 def: fn() -> GridPlacement) -> GridPlacement {
    if grid_placement.is_null() {
        return def();
    }

    let internal = env
        .get_field(grid_placement, "type", "B")
        .expect("Couldn't get field")
        .b()
        .unwrap() as i8;

    match internal {
        0 => GridPlacement::Auto,
        1 => GridPlacement::from_line_index(
            env
                .get_field(grid_placement, "value", "S")
                .expect("Couldn't get field")
                .s()
                .unwrap() as i16
        ),
        _ => GridPlacement::from_span(
            env
                .get_field(grid_placement, "value", "F")
                .expect("Couldn't get field")
                .f()
                .unwrap() as u16
        ),
    }
}

fn get_overflow<'local>(env: &mut JNIEnv<'local>, overflow: &JObject<'local>) -> Overflow {
    if overflow.is_null() {
        return Overflow::default();
    }

    let internal = get_enum_value(env, overflow);

    match internal {
        0 => Overflow::Visible,
        1 => Overflow::Clip,
        2 => Overflow::Hidden,
        _ => Overflow::Scroll
    }
}

fn get_position<'local>(env: &mut JNIEnv<'local>, position: &JObject<'local>) -> Position {
    if position.is_null() {
        return Position::default();
    }

    let internal = get_enum_value(env, position);

    match internal {
        0 => Position::Relative,
        _ => Position::Absolute
    }
}

fn get_text_align<'local>(env: &mut JNIEnv<'local>, text_align: &JObject<'local>) -> TextAlign {
    if text_align.is_null() {
        return TextAlign::default();
    }

    let internal = get_enum_value(env, text_align);

    match internal {
        0 => TextAlign::Auto,
        1 => TextAlign::LegacyLeft,
        2 => TextAlign::LegacyRight,
        _ => TextAlign::LegacyCenter
    }
}

fn get_flex_direction<'local>(env: &mut JNIEnv<'local>, flex_direction: &JObject<'local>) -> FlexDirection {
    if flex_direction.is_null() {
        return FlexDirection::default();
    }

    let internal = get_enum_value(env, flex_direction);

    match internal {
        0 => FlexDirection::Row,
        1 => FlexDirection::Column,
        2 => FlexDirection::RowReverse,
        _ => FlexDirection::ColumnReverse
    }
}

fn get_flex_wrap<'local>(env: &mut JNIEnv<'local>, flex_wrap: &JObject<'local>) -> FlexWrap {
    if flex_wrap.is_null() {
        return FlexWrap::default();
    }

    let internal = get_enum_value(env, flex_wrap);

    match internal {
        0 => FlexWrap::NoWrap,
        1 => FlexWrap::Wrap,
        _ => FlexWrap::WrapReverse
    }
}

fn get_grid_auto_flow<'local>(env: &mut JNIEnv<'local>, grid_auto_flow: &JObject<'local>) -> GridAutoFlow {
    if grid_auto_flow.is_null() {
        return GridAutoFlow::default();
    }

    let internal = get_enum_value(env, grid_auto_flow);

    match internal {
        0 => GridAutoFlow::Row,
        1 => GridAutoFlow::Column,
        2 => GridAutoFlow::RowDense,
        _ => GridAutoFlow::ColumnDense
    }
}

fn get_align_items<'local>(env: &mut JNIEnv<'local>, style: &JObject<'local>, field: &str) -> Option<AlignItems> {
    let align_items = &get_field_as(env, style, field, "Lcom/dioxuslabs/taffy/style/TaffyAlignItems;");

    if align_items.is_null() {
        return None;
    }

    let internal = get_enum_value(env, align_items);

    match internal {
        0 => Some(AlignItems::Start),
        1 => Some(AlignItems::End),
        2 => Some(AlignItems::FlexStart),
        3 => Some(AlignItems::FlexEnd),
        4 => Some(AlignItems::Center),
        5 => Some(AlignItems::Baseline),
        _ => Some(AlignItems::Stretch)
    }
}

fn get_align_content<'local>(env: &mut JNIEnv<'local>, style: &JObject<'local>, field: &str) -> Option<AlignContent> {
    let align_content = &get_field_as(env, style, field, "Lcom/dioxuslabs/taffy/style/TaffyAlignContent;");

    if align_content.is_null() {
        return None;
    }

    let internal = get_enum_value(env, align_content);

    match internal {
        0 => Some(AlignContent::Start),
        1 => Some(AlignContent::End),
        2 => Some(AlignContent::FlexStart),
        3 => Some(AlignContent::FlexEnd),
        4 => Some(AlignContent::Center),
        5 => Some(AlignContent::Stretch),
        6 => Some(AlignContent::SpaceBetween),
        7 => Some(AlignContent::SpaceEvenly),
        _ => Some(AlignContent::SpaceAround)
    }
}
