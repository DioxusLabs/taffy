use app_units::Au;
use taffy::{compute_leaf_layout, LayoutInput, LayoutOutput, NodeId, Size, Style, TaffyTree};

pub const fn px(px: f32) -> Au {
    Au((px.mul_add(60.0, 0.5f32.copysign(px))) as i32)
}

pub fn rect_px(left: f32, right: f32, top: f32, bottom: f32) -> taffy::Rect<Au> {
    taffy::Rect { left: px(left), right: px(right), top: px(top), bottom: px(bottom) }
}

/// Creates a `TaffyTree` that uses `TestNodeContext`. The purpose of this function is
/// to allow `TaffyTree` to be monomophised once in this crate rather than separately for
/// each test crate.
pub fn new_test_tree() -> TaffyTree<TestNodeContext> {
    TaffyTree::new()
}

/// A sharednode context for tests which means that tests compiled with separate crates
/// and using different styles of measure function. This saves on compile time when running tests.
#[derive(Debug, Clone)]
pub struct TestNodeContext {
    /// How many times the measure function has been called
    pub count: usize,
    /// The measurement data to compute the intrinsic size from
    pub measure_data: TestMeasureData,
}

impl TestNodeContext {
    /// Create a new `TestNodeContext` from `TestMeasureData`
    pub const fn new(measure_data: TestMeasureData) -> Self {
        Self { count: 0, measure_data }
    }

    /// Create a `TestNodeContext` for a zero-sized node
    pub const fn zero() -> Self {
        Self::new(TestMeasureData::Zero)
    }

    /// Create a `TestNodeContext` for a fixed-sized node
    pub const fn fixed(width: f32, height: f32) -> Self {
        Self::new(TestMeasureData::Fixed(Size { width, height }))
    }

    /// Create a `TestNodeContext` for a node with a width and aspect-ratio
    pub const fn aspect_ratio(width: f32, height_ratio: f32) -> Self {
        let data = AspectRatioMeasureData { width, height_ratio };
        Self::new(TestMeasureData::AspectRatio(data))
    }

    /// Create a `TestNodeContext` for a node with text using the Ahem font
    pub const fn ahem_text(text_content: String, writing_mode: WritingMode) -> Self {
        let data = AhemTextMeasureData { text_content, writing_mode };
        Self::new(TestMeasureData::AhemText(data))
    }
}

/// The measurement data for the node
#[derive(Debug, Clone)]
pub enum TestMeasureData {
    /// A zero-sized node
    Zero,
    /// A node with a fixed size
    Fixed(Size<f32>),
    /// A node with a fixed size
    AspectRatio(AspectRatioMeasureData),
    /// A node with text using the Ahem font
    AhemText(AhemTextMeasureData),
}

/// A measure function for tests that works with `TestNodeContext`
pub fn test_measure_function(
    inputs: LayoutInput,
    _node_id: NodeId,
    context: Option<&mut TestNodeContext>,
    style: &Style,
) -> LayoutOutput {
    compute_leaf_layout(
        inputs,
        style,
        |_, _| 0.0,
        |known_dimensions, available_space| {
            if let Size { width: Some(width), height: Some(height) } = known_dimensions {
                return Size { width, height };
            }

            let Some(context) = context else { return known_dimensions.map(|d| d.unwrap_or(Au(0))) };

            // Increment count
            context.count += 1;

            let compute_size = match &context.measure_data {
                TestMeasureData::Zero => Size::ZERO,
                TestMeasureData::Fixed(size) => size.map(px),
                TestMeasureData::AspectRatio(data) => data.measure(known_dimensions),
                TestMeasureData::AhemText(data) => data.measure(known_dimensions, available_space),
            };

            Size {
                width: known_dimensions.width.unwrap_or(compute_size.width),
                height: known_dimensions.height.unwrap_or(compute_size.height),
            }
        },
    )
}

/// Measure data for nodes that returns results based on an intrinsic aspect ratio
#[derive(Debug, Copy, Clone)]
pub struct AspectRatioMeasureData {
    width: f32,
    height_ratio: f32,
}
impl AspectRatioMeasureData {
    fn measure(&self, known_dimensions: Size<Option<Au>>) -> Size<Au> {
        let width = known_dimensions.width.unwrap_or_else(|| px(self.width));
        let height = known_dimensions.height.unwrap_or_else(|| px(width.to_f32_px() * self.height_ratio));
        Size { width, height }
    }
}

/// Whether text is horizontal or vertical
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum WritingMode {
    /// Horizontal text
    #[default]
    Horizontal,
    /// Vertical text
    Vertical,
}

impl core::str::FromStr for WritingMode {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains("vertical") {
            Ok(Self::Vertical)
        } else if s.contains("horizontal") {
            Ok(Self::Horizontal)
        } else {
            Err(())
        }
    }
}

/// Measure data for nodes contain text using the Ahem testing font
#[derive(Debug, Clone)]
pub struct AhemTextMeasureData {
    /// The text string
    pub text_content: String,
    /// The writing mode
    pub writing_mode: WritingMode,
}
impl AhemTextMeasureData {
    fn measure(
        &self,
        known_dimensions: taffy::Size<Option<Au>>,
        available_space: taffy::Size<taffy::AvailableSpace>,
    ) -> taffy::Size<Au> {
        use taffy::prelude::*;
        use taffy::AbsoluteAxis;

        const ZWS: char = '\u{200B}';
        const H_WIDTH: Au = Au(600);
        const H_HEIGHT: Au = Au(600);

        let inline_axis = match self.writing_mode {
            WritingMode::Horizontal => AbsoluteAxis::Horizontal,
            WritingMode::Vertical => AbsoluteAxis::Vertical,
        };
        let block_axis = inline_axis.other_axis();
        let lines: Vec<&str> = self.text_content.split(ZWS).collect();

        if lines.is_empty() {
            return Size::ZERO;
        }

        let min_line_length: usize = lines.iter().map(|line| line.len()).max().unwrap_or(0);
        let max_line_length: usize = lines.iter().map(|line| line.len()).sum();
        let inline_size = known_dimensions
            .get_abs(inline_axis)
            .unwrap_or_else(|| match available_space.get_abs(inline_axis) {
                AvailableSpace::MinContent => H_WIDTH * min_line_length as i32,
                AvailableSpace::MaxContent => H_WIDTH * max_line_length as i32,
                AvailableSpace::Definite(inline_size) => inline_size.min(H_WIDTH * max_line_length as i32),
            })
            .max(H_WIDTH * min_line_length as i32);
        let block_size = known_dimensions.get_abs(block_axis).unwrap_or_else(|| {
            let inline_line_length = (inline_size / H_WIDTH) as usize;
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
            H_HEIGHT * line_count
        });

        match self.writing_mode {
            WritingMode::Horizontal => Size { width: inline_size, height: block_size },
            WritingMode::Vertical => Size { width: block_size, height: inline_size },
        }
    }
}
