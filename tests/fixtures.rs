// this declaration is necessary to "mount" the generated code where cargo can see it
// this allows us to both keep code generation scoped to a singe directory for fs events
// and to keep each test in a separate file
mod generated;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum WritingMode {
    Horizontal,
    Vertical,
}

#[allow(dead_code)]
struct TextMeasure {
    text_content: &'static str,
    writing_mode: WritingMode,
    _aspect_ratio: Option<f32>,
}

#[allow(dead_code)]
fn test_measure_function(
    known_dimensions: taffy::Size<Option<f32>>,
    available_space: taffy::Size<taffy::AvailableSpace>,
    _node_id: taffy::NodeId,
    node_context: Option<&mut TextMeasure>,
    _style: &taffy::Style,
) -> taffy::Size<f32> {
    use taffy::prelude::*;
    use taffy::AbsoluteAxis;

    const ZWS: char = '\u{200B}';
    const H_WIDTH: f32 = 10.0;
    const H_HEIGHT: f32 = 10.0;

    if let Size { width: Some(width), height: Some(height) } = known_dimensions {
        return Size { width, height };
    }

    let Some(node_context) = node_context else { return Size::ZERO };

    let inline_axis = match node_context.writing_mode {
        WritingMode::Horizontal => AbsoluteAxis::Horizontal,
        WritingMode::Vertical => AbsoluteAxis::Vertical,
    };
    let block_axis = inline_axis.other_axis();
    let lines: Vec<&str> = node_context.text_content.split(ZWS).collect();

    if lines.is_empty() {
        return Size::ZERO;
    }

    let min_line_length: usize = lines.iter().map(|line| line.len()).max().unwrap_or(0);
    let max_line_length: usize = lines.iter().map(|line| line.len()).sum();
    let inline_size = known_dimensions
        .get_abs(inline_axis)
        .unwrap_or_else(|| match available_space.get_abs(inline_axis) {
            AvailableSpace::MinContent => min_line_length as f32 * H_WIDTH,
            AvailableSpace::MaxContent => max_line_length as f32 * H_WIDTH,
            AvailableSpace::Definite(inline_size) => inline_size.min(max_line_length as f32 * H_WIDTH),
        })
        .max(min_line_length as f32 * H_WIDTH);
    let block_size = known_dimensions.get_abs(block_axis).unwrap_or_else(|| {
        let inline_line_length = (inline_size / H_WIDTH).floor() as usize;
        let mut line_count = 1;
        let mut current_line_length = 0;
        for line in &lines {
            if current_line_length + line.len() > inline_line_length {
                if current_line_length > 0 {
                    line_count += 1
                };
                current_line_length = line.len();
            } else {
                current_line_length += line.len();
            };
        }
        (line_count as f32) * H_HEIGHT
    });

    match node_context.writing_mode {
        WritingMode::Horizontal => Size { width: inline_size, height: block_size },
        WritingMode::Vertical => Size { width: block_size, height: inline_size },
    }
}
