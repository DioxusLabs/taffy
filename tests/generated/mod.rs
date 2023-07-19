#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum WritingMode {
    Horizontal,
    Vertical,
}
#[allow(dead_code)]
fn measure_standard_text(
    known_dimensions: taffy::geometry::Size<Option<f32>>,
    available_space: taffy::geometry::Size<taffy::style::AvailableSpace>,
    text_content: &str,
    writing_mode: WritingMode,
    _aspect_ratio: Option<f32>,
) -> taffy::geometry::Size<f32> {
    use taffy::geometry::AbsoluteAxis;
    use taffy::prelude::*;
    const ZWS: char = '\u{200B}';
    const H_WIDTH: f32 = 10.0;
    const H_HEIGHT: f32 = 10.0;
    if let Size { width: Some(width), height: Some(height) } = known_dimensions {
        return Size { width, height };
    }
    let inline_axis = match writing_mode {
        WritingMode::Horizontal => AbsoluteAxis::Horizontal,
        WritingMode::Vertical => AbsoluteAxis::Vertical,
    };
    let block_axis = inline_axis.other_axis();
    let lines: Vec<&str> = text_content.split(ZWS).collect();
    if lines.is_empty() {
        return Size::ZERO;
    }
    let min_line_length: usize = lines.iter().map(|line| line.len()).max().unwrap_or(0);
    let max_line_length: usize = lines.iter().map(|line| line.len()).sum();
    let inline_size =
        known_dimensions.get_abs(inline_axis).unwrap_or_else(|| match available_space.get_abs(inline_axis) {
            AvailableSpace::MinContent => min_line_length as f32 * H_WIDTH,
            AvailableSpace::MaxContent => max_line_length as f32 * H_WIDTH,
            AvailableSpace::Definite(inline_size) => {
                inline_size.min(max_line_length as f32 * H_WIDTH).max(min_line_length as f32 * H_WIDTH)
            }
        });
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
    match writing_mode {
        WritingMode::Horizontal => Size { width: inline_size, height: block_size },
        WritingMode::Vertical => Size { width: block_size, height: inline_size },
    }
}
mod block;
mod blockflex;
mod blockgrid;
mod flex;
mod grid;
mod gridflex;
mod leaf;
