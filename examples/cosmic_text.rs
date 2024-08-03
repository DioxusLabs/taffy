mod common {
    pub mod image;
}
use common::image::{image_measure_function, ImageContext};
use cosmic_text::{Attrs, Buffer, FontSystem, Metrics, Shaping};
use taffy::prelude::*;

pub const LOREM_IPSUM : &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";

struct CosmicTextContext {
    buffer: cosmic_text::Buffer,
}

impl CosmicTextContext {
    fn new(metrics: Metrics, text: &str, attrs: Attrs, font_system: &mut FontSystem) -> Self {
        let mut buffer = Buffer::new_empty(metrics);
        buffer.set_size(font_system, None, None);
        buffer.set_text(font_system, text, attrs, Shaping::Advanced);
        Self { buffer }
    }

    fn measure(
        &mut self,
        known_dimensions: taffy::Size<Option<f32>>,
        available_space: taffy::Size<taffy::AvailableSpace>,
        font_system: &mut FontSystem,
    ) -> taffy::Size<f32> {
        // Set width constraint
        let width_constraint = known_dimensions.width.or(match available_space.width {
            AvailableSpace::MinContent => Some(0.0),
            AvailableSpace::MaxContent => None,
            AvailableSpace::Definite(width) => Some(width),
        });
        self.buffer.set_size(font_system, width_constraint, None);

        // Compute layout
        self.buffer.shape_until_scroll(font_system, false);

        // Determine measured size of text
        let (width, total_lines) = self
            .buffer
            .layout_runs()
            .fold((0.0, 0usize), |(width, total_lines), run| (run.line_w.max(width), total_lines + 1));
        let height = total_lines as f32 * self.buffer.metrics().line_height;

        taffy::Size { width, height }
    }
}

enum NodeContext {
    Text(CosmicTextContext),
    Image(ImageContext),
}

impl NodeContext {
    /// Constructor for a text node context
    fn text(metrics: Metrics, text: &str, attrs: Attrs, font_system: &mut FontSystem) -> Self {
        NodeContext::Text(CosmicTextContext::new(metrics, text, attrs, font_system))
    }

    /// Constructor for an image node context
    fn image(width: f32, height: f32) -> Self {
        NodeContext::Image(ImageContext { width, height })
    }
}

fn measure_function(
    known_dimensions: taffy::Size<Option<f32>>,
    available_space: taffy::Size<taffy::AvailableSpace>,
    node_context: Option<&mut NodeContext>,
    font_system: &mut FontSystem,
) -> Size<f32> {
    if let Size { width: Some(width), height: Some(height) } = known_dimensions {
        return Size { width, height };
    }

    match node_context {
        None => Size::ZERO,
        Some(NodeContext::Text(text_context)) => text_context.measure(known_dimensions, available_space, font_system),
        Some(NodeContext::Image(image_context)) => image_measure_function(known_dimensions, image_context),
    }
}

fn main() -> Result<(), taffy::TaffyError> {
    let metrics = Metrics { font_size: 14.0, line_height: 16.0 };
    let mut font_system = FontSystem::new();
    let mut taffy: TaffyTree<NodeContext> = TaffyTree::new();

    let text_node = taffy.new_leaf_with_context(
        Style::default(),
        NodeContext::text(metrics, LOREM_IPSUM, Attrs::new(), &mut font_system),
    )?;

    let image_node = taffy.new_leaf_with_context(Style::default(), NodeContext::image(400.0, 300.0))?;

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
        |known_dimensions, available_space, _node_id, node_context, _style| {
            measure_function(known_dimensions, available_space, node_context, &mut font_system)
        },
    )?;
    taffy.print_tree(root);

    Ok(())
}
