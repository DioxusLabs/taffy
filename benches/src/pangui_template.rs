//! Benchmark template file parser for Taffy benchmarks
//!
//! Parses the benchmark template file format and produces a LayoutNode tree.

#![allow(dead_code)]

/// Size mode for dimension specification
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SizeMode {
    Pixels,
    Fit,
    Expand,
    Percentage,
    Ratio,
}

/// Size value with mode
#[derive(Debug, Clone, Copy)]
pub struct BenchmarkSize {
    pub value: f32,
    pub mode: SizeMode,
}

impl BenchmarkSize {
    pub fn pixels(v: f32) -> Self {
        Self { value: v, mode: SizeMode::Pixels }
    }

    pub fn fit() -> Self {
        Self { value: 1.0, mode: SizeMode::Fit }
    }

    pub fn expand(weight: f32) -> Self {
        Self { value: weight, mode: SizeMode::Expand }
    }

    pub fn percentage(v: f32) -> Self {
        Self { value: v, mode: SizeMode::Percentage }
    }

    pub fn ratio(v: f32) -> Self {
        Self { value: v, mode: SizeMode::Ratio }
    }
}

/// Wrap mode
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WrapMode {
    NoWrap,
    Auto,
}

/// Flow direction
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FlowDirection {
    Vertical,
    Horizontal,
}

/// Layout node structure
#[derive(Debug, Clone)]
pub struct LayoutNode {
    pub width: Option<BenchmarkSize>,
    pub height: Option<BenchmarkSize>,
    pub min_width: Option<BenchmarkSize>,
    pub min_height: Option<BenchmarkSize>,
    pub max_width: Option<BenchmarkSize>,
    pub max_height: Option<BenchmarkSize>,
    pub gap: Option<f32>,
    pub padding_left: Option<f32>,
    pub padding_right: Option<f32>,
    pub padding_top: Option<f32>,
    pub padding_bottom: Option<f32>,
    pub margin_left: Option<f32>,
    pub margin_right: Option<f32>,
    pub margin_top: Option<f32>,
    pub margin_bottom: Option<f32>,
    pub wrap: Option<WrapMode>,
    pub flow_direction: Option<FlowDirection>,
    pub children: Vec<LayoutNode>,
}

impl Default for LayoutNode {
    fn default() -> Self {
        Self {
            width: None,
            height: None,
            min_width: None,
            min_height: None,
            max_width: None,
            max_height: None,
            gap: None,
            padding_left: None,
            padding_right: None,
            padding_top: None,
            padding_bottom: None,
            margin_left: None,
            margin_right: None,
            margin_top: None,
            margin_bottom: None,
            wrap: None,
            flow_direction: None,
            children: Vec::new(),
        }
    }
}

/// Test case parsed from a template file
pub struct TestCase {
    pub name: String,
    pub root_node: Option<LayoutNode>,
}

/// Parser for benchmark template files
pub struct BenchmarkParser<'a> {
    text: &'a str,
    chars: Vec<char>,
    index: usize,
    line: usize,
}

impl<'a> BenchmarkParser<'a> {
    fn new(text: &'a str) -> Self {
        Self { text, chars: text.chars().collect(), index: 0, line: 1 }
    }

    fn is_at_end(&self) -> bool {
        self.index >= self.chars.len()
    }

    fn peek(&self) -> Option<char> {
        self.chars.get(self.index).copied()
    }

    fn peek_char(&self, c: char) -> bool {
        self.peek() == Some(c)
    }

    fn advance(&mut self) {
        if !self.is_at_end() {
            if self.chars[self.index] == '\n' {
                self.line += 1;
            }
            self.index += 1;
        }
    }

    fn skip_whitespace_and_comments(&mut self) {
        while !self.is_at_end() {
            let c = self.peek().unwrap();
            if c.is_whitespace() {
                self.advance();
            } else if c == '/' && self.chars.get(self.index + 1) == Some(&'/') {
                // Skip line comment
                while !self.is_at_end() && self.peek() != Some('\n') {
                    self.advance();
                }
            } else {
                break;
            }
        }
    }

    fn try_eat(&mut self, s: &str) -> bool {
        self.skip_whitespace_and_comments();
        let s_chars: Vec<char> = s.chars().collect();
        if self.index + s_chars.len() > self.chars.len() {
            return false;
        }
        for (i, &c) in s_chars.iter().enumerate() {
            if self.chars[self.index + i] != c {
                return false;
            }
        }
        // Make sure it's not a prefix of a longer identifier
        if let Some(&next) = self.chars.get(self.index + s_chars.len()) {
            if next.is_alphanumeric() || next == '_' {
                return false;
            }
        }
        self.index += s_chars.len();
        true
    }

    fn expect(&mut self, c: char) -> Result<(), String> {
        self.skip_whitespace_and_comments();
        if self.peek() == Some(c) {
            self.advance();
            Ok(())
        } else {
            Err(format!("Line {}: Expected '{}', found {:?}", self.line, c, self.peek()))
        }
    }

    fn try_parse_float(&mut self) -> Option<f32> {
        self.skip_whitespace_and_comments();

        let start = self.index;
        let mut has_digits = false;
        let mut has_dot = false;

        // Handle negative sign
        if self.peek() == Some('-') {
            self.advance();
        }

        while !self.is_at_end() {
            let c = self.peek().unwrap();
            if c.is_ascii_digit() {
                has_digits = true;
                self.advance();
            } else if c == '.' && !has_dot {
                has_dot = true;
                self.advance();
            } else {
                break;
            }
        }

        if !has_digits {
            self.index = start;
            return None;
        }

        let num_str: String = self.chars[start..self.index].iter().collect();
        num_str.parse().ok()
    }

    fn try_parse_size(&mut self) -> Option<BenchmarkSize> {
        self.skip_whitespace_and_comments();

        // Pixels(value)
        if self.try_eat("Pixels") {
            if self.expect('(').is_ok() {
                if let Some(value) = self.try_parse_float() {
                    if self.expect(')').is_ok() {
                        return Some(BenchmarkSize::pixels(value));
                    }
                }
            }
            return None;
        }

        // Fit or Fit(weight)
        if self.try_eat("Fit") {
            self.skip_whitespace_and_comments();
            if self.peek_char('(') {
                self.advance();
                if let Some(weight) = self.try_parse_float() {
                    if self.expect(')').is_ok() {
                        return Some(BenchmarkSize { value: weight, mode: SizeMode::Fit });
                    }
                }
                return None;
            }
            return Some(BenchmarkSize::fit());
        }

        // Expand or Expand(weight)
        if self.try_eat("Expand") {
            self.skip_whitespace_and_comments();
            if self.peek_char('(') {
                self.advance();
                if let Some(weight) = self.try_parse_float() {
                    if self.expect(')').is_ok() {
                        return Some(BenchmarkSize::expand(weight));
                    }
                }
                return None;
            }
            return Some(BenchmarkSize::expand(1.0));
        }

        // Percentage(value)
        if self.try_eat("Percentage") {
            if self.expect('(').is_ok() {
                if let Some(value) = self.try_parse_float() {
                    if self.expect(')').is_ok() {
                        return Some(BenchmarkSize::percentage(value));
                    }
                }
            }
            return None;
        }

        // Ratio(value)
        if self.try_eat("Ratio") {
            if self.expect('(').is_ok() {
                if let Some(value) = self.try_parse_float() {
                    if self.expect(')').is_ok() {
                        return Some(BenchmarkSize::ratio(value));
                    }
                }
            }
            return None;
        }

        // Bare numeric value = Pixels
        if let Some(value) = self.try_parse_float() {
            return Some(BenchmarkSize::pixels(value));
        }

        None
    }

    fn try_parse_property(&mut self, node: &mut LayoutNode) -> bool {
        self.skip_whitespace_and_comments();

        // Width(...)
        if self.try_eat("Width") {
            if self.expect('(').is_ok() {
                if let Some(size) = self.try_parse_size() {
                    if self.expect(')').is_ok() {
                        node.width = Some(size);
                        return true;
                    }
                }
            }
            return false;
        }

        // Height(...)
        if self.try_eat("Height") {
            if self.expect('(').is_ok() {
                if let Some(size) = self.try_parse_size() {
                    if self.expect(')').is_ok() {
                        node.height = Some(size);
                        return true;
                    }
                }
            }
            return false;
        }

        // MinWidth(...)
        if self.try_eat("MinWidth") {
            if self.expect('(').is_ok() {
                if let Some(size) = self.try_parse_size() {
                    if self.expect(')').is_ok() {
                        node.min_width = Some(size);
                        return true;
                    }
                }
            }
            return false;
        }

        // MinHeight(...)
        if self.try_eat("MinHeight") {
            if self.expect('(').is_ok() {
                if let Some(size) = self.try_parse_size() {
                    if self.expect(')').is_ok() {
                        node.min_height = Some(size);
                        return true;
                    }
                }
            }
            return false;
        }

        // MaxWidth(...)
        if self.try_eat("MaxWidth") {
            if self.expect('(').is_ok() {
                if let Some(size) = self.try_parse_size() {
                    if self.expect(')').is_ok() {
                        node.max_width = Some(size);
                        return true;
                    }
                }
            }
            return false;
        }

        // MaxHeight(...)
        if self.try_eat("MaxHeight") {
            if self.expect('(').is_ok() {
                if let Some(size) = self.try_parse_size() {
                    if self.expect(')').is_ok() {
                        node.max_height = Some(size);
                        return true;
                    }
                }
            }
            return false;
        }

        // Gap(...)
        if self.try_eat("Gap") {
            if self.expect('(').is_ok() {
                if let Some(value) = self.try_parse_float() {
                    if self.expect(')').is_ok() {
                        node.gap = Some(value);
                        return true;
                    }
                }
            }
            return false;
        }

        // PaddingX(...)
        if self.try_eat("PaddingX") {
            if self.expect('(').is_ok() {
                if let Some(value) = self.try_parse_float() {
                    if self.expect(')').is_ok() {
                        node.padding_left = Some(value);
                        node.padding_right = Some(value);
                        return true;
                    }
                }
            }
            return false;
        }

        // PaddingY(...)
        if self.try_eat("PaddingY") {
            if self.expect('(').is_ok() {
                if let Some(value) = self.try_parse_float() {
                    if self.expect(')').is_ok() {
                        node.padding_top = Some(value);
                        node.padding_bottom = Some(value);
                        return true;
                    }
                }
            }
            return false;
        }

        // Padding(...) - must come after PaddingX/PaddingY
        if self.try_eat("Padding") {
            if self.expect('(').is_ok() {
                if let Some(value) = self.try_parse_float() {
                    if self.expect(')').is_ok() {
                        node.padding_left = Some(value);
                        node.padding_right = Some(value);
                        node.padding_top = Some(value);
                        node.padding_bottom = Some(value);
                        return true;
                    }
                }
            }
            return false;
        }

        // MarginX(...)
        if self.try_eat("MarginX") {
            if self.expect('(').is_ok() {
                if let Some(value) = self.try_parse_float() {
                    if self.expect(')').is_ok() {
                        node.margin_left = Some(value);
                        node.margin_right = Some(value);
                        return true;
                    }
                }
            }
            return false;
        }

        // MarginY(...)
        if self.try_eat("MarginY") {
            if self.expect('(').is_ok() {
                if let Some(value) = self.try_parse_float() {
                    if self.expect(')').is_ok() {
                        node.margin_top = Some(value);
                        node.margin_bottom = Some(value);
                        return true;
                    }
                }
            }
            return false;
        }

        // Margin(...) - must come after MarginX/MarginY
        if self.try_eat("Margin") {
            if self.expect('(').is_ok() {
                if let Some(value) = self.try_parse_float() {
                    if self.expect(')').is_ok() {
                        node.margin_left = Some(value);
                        node.margin_right = Some(value);
                        node.margin_top = Some(value);
                        node.margin_bottom = Some(value);
                        return true;
                    }
                }
            }
            return false;
        }

        // Wrap(...)
        if self.try_eat("Wrap") {
            if self.expect('(').is_ok() {
                self.skip_whitespace_and_comments();
                if self.try_eat("Auto") {
                    node.wrap = Some(WrapMode::Auto);
                } else if self.try_eat("NoWrap") {
                    node.wrap = Some(WrapMode::NoWrap);
                } else {
                    return false;
                }
                return self.expect(')').is_ok();
            }
            return false;
        }

        // Horizontal()
        if self.try_eat("Horizontal") {
            if self.expect('(').is_ok() && self.expect(')').is_ok() {
                node.flow_direction = Some(FlowDirection::Horizontal);
                return true;
            }
            return false;
        }

        // Vertical()
        if self.try_eat("Vertical") {
            if self.expect('(').is_ok() && self.expect(')').is_ok() {
                node.flow_direction = Some(FlowDirection::Vertical);
                return true;
            }
            return false;
        }

        false
    }

    /// Returns an error for the current position in the template
    fn error<T>(&self, expected: &str) -> Result<T, String> {
        Err(format!("Line {}: Expected {}, found {:?}", self.line, expected, self.peek()))
    }

    /// Parse a node: `.`-separated properties followed by a `{ ... }` block containing its children
    fn parse_layout_node(&mut self) -> Result<LayoutNode, String> {
        let mut node = LayoutNode::default();

        self.skip_whitespace_and_comments();
        while !self.peek_char('{') {
            if !self.try_parse_property(&mut node) {
                return self.error("a property or '{'");
            }
            self.skip_whitespace_and_comments();
            if self.peek_char('.') {
                self.advance();
            } else if !self.peek_char('{') {
                return self.error("'.' or '{'");
            }
        }

        self.advance(); // consume '{'
        self.parse_children(&mut node.children)?;
        self.expect('}')?;

        Ok(node)
    }

    /// Parse nodes and `Repeat(N) { ... }` groups up to the `}` that closes the block containing them
    fn parse_children(&mut self, children: &mut Vec<LayoutNode>) -> Result<(), String> {
        loop {
            self.skip_whitespace_and_comments();
            if self.is_at_end() || self.peek_char('}') {
                return Ok(());
            }

            if self.try_eat("Repeat") {
                self.expect('(')?;
                let Some(repeat_count) = self.try_parse_float() else {
                    return self.error("a repeat count");
                };
                self.expect(')')?;
                self.expect('{')?;
                let mut template_nodes = Vec::new();
                self.parse_children(&mut template_nodes)?;
                self.expect('}')?;

                // Add N copies of the group
                for _ in 0..(repeat_count as usize) {
                    children.extend(template_nodes.iter().cloned());
                }
            } else {
                children.push(self.parse_layout_node()?);
            }
        }
    }

    fn expect_section(&mut self, name: &str) -> Result<(), String> {
        self.skip_whitespace_and_comments();
        if !self.peek_char('#') {
            return Err(format!("Expected '# {}' section", name));
        }
        self.advance(); // consume '#'
        self.skip_whitespace_and_comments();

        let name_chars: Vec<char> = name.chars().collect();
        for &c in &name_chars {
            if self.peek() != Some(c) {
                return Err(format!("Expected '# {}' section", name));
            }
            self.advance();
        }
        Ok(())
    }

    fn parse_test_case(&mut self, file_name: &str) -> Result<TestCase, String> {
        let name = file_name.strip_suffix(".txt").unwrap_or(file_name).to_string();

        self.skip_whitespace_and_comments();

        // Parse # Input section
        self.expect_section("Input")?;

        self.skip_whitespace_and_comments();

        // Parse the root node
        let root_node = self.parse_layout_node()?;

        // Only further sections (which we do not use) may follow the root node
        self.skip_whitespace_and_comments();
        if !self.is_at_end() && !self.peek_char('#') {
            return self.error("'#' or the end of the file");
        }

        Ok(TestCase { name, root_node: Some(root_node) })
    }
}

/// Parse a test case from a template file
pub fn parse_test_case(file_name: &str, text: &str) -> Result<TestCase, String> {
    let mut parser = BenchmarkParser::new(text);
    parser.parse_test_case(file_name)
}
