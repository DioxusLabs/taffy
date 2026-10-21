use taffy::geometry::Size;

pub struct ImageContext {
    pub width: f32,
    pub height: f32,
}

pub fn image_measure_function(
    known_dimensions: taffy::geometry::Size<Option<taffy::Au>>,
    image_context: &ImageContext,
) -> taffy::geometry::Size<taffy::Au> {
    let px = |value: f32| taffy::Au(((value * 60.0) + 0.5f32.copysign(value)) as i32);
    match (known_dimensions.width, known_dimensions.height) {
        (Some(width), Some(height)) => Size { width, height },
        (Some(width), None) => {
            Size { width, height: px((width.to_f32_px() / image_context.width) * image_context.height) }
        }
        (None, Some(height)) => {
            Size { width: px((height.to_f32_px() / image_context.height) * image_context.width), height }
        }
        (None, None) => Size { width: px(image_context.width), height: px(image_context.height) },
    }
}
