use taffy::prelude::*;

const LOREM_IPSUM : &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";

struct FontMetrics {
    char_width: f32,
    char_height: f32,
}
enum WritingMode {
    Horizontal,
    Vertical,
}
struct TextContext {
    text_content: String,
    writing_mode: WritingMode,
}
struct ImageContext {
    width: f32,
    height: f32,
}
enum NodeContext {
    Text(TextContext),
    Image(ImageContext),
}

fn main() -> Result<(), taffy::TaffyError> {
    let mut taffy: Taffy<NodeContext> = Taffy::new();

    let font_metrics = FontMetrics { char_width: 10.0, char_height: 10.0 };

    let text_node = taffy.new_leaf_with_context(
        Style::default(),
        NodeContext::Text(TextContext { text_content: LOREM_IPSUM.into(), writing_mode: WritingMode::Horizontal }),
    )?;

    let image_node = taffy
        .new_leaf_with_context(Style::default(), NodeContext::Image(ImageContext { width: 400.0, height: 300.0 }))?;

    let root = taffy.new_with_children(
        Style {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            size: Size { width: length(200.0), height: auto() },
            ..Default::default()
        },
        &[text_node, image_node],
    )?;

    // Compute layout and print result
    taffy.compute_layout_with_measure(
        root,
        Size::MAX_CONTENT,
        // Note: this closure is a FnMut closure and can be used to borrow external context for the duration of layout
        // For example, you may wish to borrow a global font registry and pass it into your text measuring function
        |known_dimensions, available_space, _node_id, node_context| {
            measure_function(known_dimensions, available_space, node_context, &font_metrics)
        },
    )?;
    taffy.print_tree(root);

    Ok(())
}

fn measure_function(
    known_dimensions: taffy::geometry::Size<Option<f32>>,
    available_space: taffy::geometry::Size<taffy::style::AvailableSpace>,
    node_context: Option<&mut NodeContext>,
    font_metrics: &FontMetrics,
) -> Size<f32> {
    if let Size { width: Some(width), height: Some(height) } = known_dimensions {
        return Size { width, height };
    }

    match node_context {
        None => Size::ZERO,
        Some(NodeContext::Text(text_context)) => {
            text_measure_function(known_dimensions, available_space, &*text_context, font_metrics)
        }
        Some(NodeContext::Image(image_context)) => image_measure_function(known_dimensions, image_context),
    }
}

fn image_measure_function(
    known_dimensions: taffy::geometry::Size<Option<f32>>,
    image_context: &ImageContext,
) -> taffy::geometry::Size<f32> {
    match (known_dimensions.width, known_dimensions.height) {
        (Some(width), Some(height)) => Size { width, height },
        (Some(width), None) => Size { width, height: (width / image_context.width) * image_context.height },
        (None, Some(height)) => Size { width: (height / image_context.height) * image_context.width, height },
        (None, None) => Size { width: image_context.width, height: image_context.height },
    }
}

fn text_measure_function(
    known_dimensions: taffy::geometry::Size<Option<f32>>,
    available_space: taffy::geometry::Size<taffy::style::AvailableSpace>,
    text_context: &TextContext,
    font_metrics: &FontMetrics,
) -> taffy::geometry::Size<f32> {
    use taffy::geometry::AbsoluteAxis;
    use taffy::prelude::*;

    let inline_axis = match text_context.writing_mode {
        WritingMode::Horizontal => AbsoluteAxis::Horizontal,
        WritingMode::Vertical => AbsoluteAxis::Vertical,
    };
    let block_axis = inline_axis.other_axis();
    let words: Vec<&str> = text_context.text_content.split_whitespace().collect();

    if words.is_empty() {
        return Size::ZERO;
    }

    let min_line_length: usize = words.iter().map(|line| line.len()).max().unwrap_or(0);
    let max_line_length: usize = words.iter().map(|line| line.len()).sum();
    let inline_size =
        known_dimensions.get_abs(inline_axis).unwrap_or_else(|| match available_space.get_abs(inline_axis) {
            AvailableSpace::MinContent => min_line_length as f32 * font_metrics.char_width,
            AvailableSpace::MaxContent => max_line_length as f32 * font_metrics.char_width,
            AvailableSpace::Definite(inline_size) => inline_size
                .min(max_line_length as f32 * font_metrics.char_width)
                .max(min_line_length as f32 * font_metrics.char_width),
        });
    let block_size = known_dimensions.get_abs(block_axis).unwrap_or_else(|| {
        let inline_line_length = (inline_size / font_metrics.char_width).floor() as usize;
        let mut line_count = 1;
        let mut current_line_length = 0;
        for word in &words {
            if current_line_length + word.len() > inline_line_length {
                if current_line_length > 0 {
                    line_count += 1
                };
                current_line_length = word.len();
            } else {
                current_line_length += word.len();
            };
        }
        (line_count as f32) * font_metrics.char_height
    });

    match text_context.writing_mode {
        WritingMode::Horizontal => Size { width: inline_size, height: block_size },
        WritingMode::Vertical => Size { width: block_size, height: inline_size },
    }
}
