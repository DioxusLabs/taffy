//! Computes the position of floats in a block formatting context

pub struct FloatedBox {
    y_start: f32,
    y_end: f32,
    width: f32,
}

pub struct Segment {
    y_start: f32,
    y_end: f32,
    inset_left: f32,
    inset_right: f32,
}

pub struct FloatContext {
    left_floats: Vec<FloatedBox>,
    right_floats: Vec<FloatedBox>,
    segments: Vec<Segment>,
}
