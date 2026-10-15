//! Port of the style resolution in `yoga/style/Style.h` and `yoga/node/Node.cpp` (`computeLeftEdge` and
//! friends, `resolveFlexGrow`, `resolveFlexShrink`, `processFlexBasis`) and of Yoga's style defaults.
//!
//! The Yoga mode reads Taffy's [`Style`]. Where Yoga's style is richer (nine edges, `flex` shorthand, gap
//! gutters, web defaults) the host resolves it to a `Style` with these helpers before layout.

use crate::geometry::{Rect, Size};
use crate::style::{
    AlignContent, AlignItems, Dimension, Direction, FlexDirection, JustifyContent, LengthPercentage, Position, Style,
};

use super::config::YogaConfig;

/// The nine edges a Yoga margin, padding, border or position can be set on.
///
/// `resolve` applies Yoga's precedence: `start`/`end` (for the resolved direction), then `left`/`right`,
/// then `horizontal`/`vertical`, then `all`.
#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub struct YogaEdges<T> {
    /// Physical left edge.
    pub left: Option<T>,
    /// Physical top edge.
    pub top: Option<T>,
    /// Physical right edge.
    pub right: Option<T>,
    /// Physical bottom edge.
    pub bottom: Option<T>,
    /// Inline start: left in LTR, right in RTL.
    pub start: Option<T>,
    /// Inline end: right in LTR, left in RTL.
    pub end: Option<T>,
    /// Both left and right.
    pub horizontal: Option<T>,
    /// Both top and bottom.
    pub vertical: Option<T>,
    /// Every edge.
    pub all: Option<T>,
}

impl<T: Copy> YogaEdges<T> {
    /// Resolves the four physical edges for `direction` (Yoga's `computeLeftEdge` etc.).
    pub fn resolve(&self, direction: Direction) -> Rect<Option<T>> {
        let (left_logical, right_logical) = match direction {
            Direction::Ltr => (self.start, self.end),
            Direction::Rtl => (self.end, self.start),
        };
        Rect {
            left: left_logical.or(self.left).or(self.horizontal).or(self.all),
            right: right_logical.or(self.right).or(self.horizontal).or(self.all),
            top: self.top.or(self.vertical).or(self.all),
            bottom: self.bottom.or(self.vertical).or(self.all),
        }
    }
}

/// Yoga's `gap` gutters: `row`/`column` take precedence over `all`.
#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub struct YogaGutters<T> {
    /// Gap between columns (horizontal gap).
    pub column: Option<T>,
    /// Gap between rows (vertical gap).
    pub row: Option<T>,
    /// Both, unless overridden.
    pub all: Option<T>,
}

impl<T: Copy> YogaGutters<T> {
    /// `width` is the column gap and `height` the row gap, matching [`Style::gap`].
    pub fn resolve(&self) -> Size<Option<T>> {
        Size { width: self.column.or(self.all), height: self.row.or(self.all) }
    }
}

/// Yoga's `flex`, `flex-grow`, `flex-shrink` and `flex-basis` properties, each unset (`None`) by default.
#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub struct YogaFlex {
    /// The `flex` shorthand: positive values grow with a zero basis, negative values shrink.
    pub flex: Option<f32>,
    /// Explicit `flex-grow`, overriding `flex`.
    pub flex_grow: Option<f32>,
    /// Explicit `flex-shrink`, overriding `flex`.
    pub flex_shrink: Option<f32>,
    /// Explicit `flex-basis`, overriding `flex`; `None` is `auto`.
    pub flex_basis: Option<Dimension>,
}

impl YogaFlex {
    /// Resolves to `(flex_grow, flex_shrink, flex_basis)` as Yoga's `resolveFlexGrow`, `resolveFlexShrink`
    /// and `processFlexBasis` do. The root's grow and shrink are forced to 0 by the algorithm, not here.
    pub fn resolve(&self, config: &YogaConfig) -> (f32, f32, Dimension) {
        let positive_flex = self.flex.filter(|f| *f > 0.0);
        let grow = self.flex_grow.or(positive_flex).unwrap_or(DEFAULT_FLEX_GROW);
        let shrink = match self.flex_shrink {
            Some(shrink) => shrink,
            None => match self.flex {
                Some(flex) if !config.use_web_defaults && flex < 0.0 => -flex,
                _ if config.use_web_defaults => WEB_DEFAULT_FLEX_SHRINK,
                _ => DEFAULT_FLEX_SHRINK,
            },
        };
        let basis = match self.flex_basis {
            Some(basis) if !basis.is_auto() => basis,
            _ if positive_flex.is_some() && !config.use_web_defaults => Dimension::length(0.0),
            _ => Dimension::auto(),
        };
        (grow, shrink, basis)
    }
}

/// `Style::DefaultFlexGrow`.
pub const DEFAULT_FLEX_GROW: f32 = 0.0;
/// `Style::DefaultFlexShrink`.
pub const DEFAULT_FLEX_SHRINK: f32 = 0.0;
/// `Style::WebDefaultFlexShrink`.
pub const WEB_DEFAULT_FLEX_SHRINK: f32 = 1.0;

/// A [`Style`] with Yoga's default values (`YGNodeNew()`), or the CSS ones if `config.use_web_defaults`.
///
/// Differences from `Style::default()`: `flex-direction: column`, `align-content: flex-start`,
/// `align-items: stretch`, `justify-content: flex-start`, `position: relative`, `flex-shrink: 0`, no
/// scrollbar width. The `display` is Taffy's default and must be set by the host.
pub fn default_style(config: &YogaConfig) -> Style {
    let (flex_direction, align_content) = if config.use_web_defaults {
        (FlexDirection::Row, AlignContent::STRETCH)
    } else {
        (FlexDirection::Column, AlignContent::FLEX_START)
    };
    let (flex_grow, flex_shrink, flex_basis) = YogaFlex::default().resolve(config);
    Style {
        flex_direction,
        align_content,
        align_items: AlignItems::STRETCH,
        justify_content: JustifyContent::FLEX_START,
        position: Position::Relative,
        flex_grow,
        flex_shrink,
        flex_basis,
        scrollbar_width: 0.0,
        gap: Size { width: LengthPercentage::length(0.0), height: LengthPercentage::length(0.0) },
        ..Style::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::style::LengthPercentageAuto;
    use crate::style_helpers::{auto, length, percent};

    #[test]
    fn edge_precedence() {
        let edges = YogaEdges {
            all: Some(1.0),
            horizontal: Some(2.0),
            vertical: Some(3.0),
            left: Some(4.0),
            start: Some(5.0),
            end: Some(6.0),
            ..YogaEdges::default()
        };
        let ltr = edges.resolve(Direction::Ltr);
        assert_eq!(ltr, Rect { left: Some(5.0), right: Some(6.0), top: Some(3.0), bottom: Some(3.0) });
        let rtl = edges.resolve(Direction::Rtl);
        assert_eq!(rtl, Rect { left: Some(6.0), right: Some(5.0), top: Some(3.0), bottom: Some(3.0) });

        let edges = YogaEdges { all: Some(1.0), left: Some(4.0), top: Some(7.0), ..YogaEdges::default() };
        assert_eq!(
            edges.resolve(Direction::Rtl),
            Rect { left: Some(4.0), right: Some(1.0), top: Some(7.0), bottom: Some(1.0) }
        );
        assert_eq!(
            YogaEdges::<f32>::default().resolve(Direction::Ltr),
            Rect { left: None, right: None, top: None, bottom: None }
        );
    }

    #[test]
    fn gutter_precedence() {
        let gutters = YogaGutters { all: Some(1.0), row: Some(2.0), column: None };
        assert_eq!(gutters.resolve(), Size { width: Some(1.0), height: Some(2.0) });
        assert_eq!(YogaGutters::<f32>::default().resolve(), Size { width: None, height: None });
    }

    #[test]
    fn flex_shorthand() {
        let yoga = YogaConfig::default();
        let web = YogaConfig { use_web_defaults: true, ..yoga };

        assert_eq!(YogaFlex::default().resolve(&yoga), (0.0, 0.0, auto()));
        assert_eq!(YogaFlex::default().resolve(&web), (0.0, 1.0, auto()));

        let grow = YogaFlex { flex: Some(2.0), ..Default::default() };
        assert_eq!(grow.resolve(&yoga), (2.0, 0.0, length(0.0)));
        assert_eq!(grow.resolve(&web), (2.0, 1.0, auto()));

        let shrink = YogaFlex { flex: Some(-3.0), ..Default::default() };
        assert_eq!(shrink.resolve(&yoga), (0.0, 3.0, auto()));
        assert_eq!(shrink.resolve(&web), (0.0, 1.0, auto()));

        let explicit =
            YogaFlex { flex: Some(2.0), flex_grow: Some(0.5), flex_shrink: Some(4.0), flex_basis: Some(percent(0.5)) };
        assert_eq!(explicit.resolve(&yoga), (0.5, 4.0, percent(0.5)));
        let auto_basis = YogaFlex { flex: Some(1.0), flex_basis: Some(auto()), ..Default::default() };
        assert_eq!(auto_basis.resolve(&yoga), (1.0, 0.0, length(0.0)));
    }

    #[test]
    fn defaults() {
        let style = default_style(&YogaConfig::default());
        assert_eq!(style.flex_direction, FlexDirection::Column);
        assert_eq!(style.align_content, AlignContent::FLEX_START);
        assert_eq!(style.align_items, AlignItems::STRETCH);
        assert_eq!(style.justify_content, JustifyContent::FLEX_START);
        assert_eq!(style.position, Position::Relative);
        assert_eq!(style.flex_shrink, 0.0);
        assert_eq!(style.flex_basis, Dimension::auto());
        assert_eq!(style.min_size.width, LengthPercentageAuto::auto());
        assert_eq!(style.scrollbar_width, 0.0);

        let web = default_style(&YogaConfig { use_web_defaults: true, ..YogaConfig::default() });
        assert_eq!(web.flex_direction, FlexDirection::Row);
        assert_eq!(web.align_content, AlignContent::STRETCH);
        assert_eq!(web.flex_shrink, 1.0);
        assert_eq!(web.position, Position::Relative);
    }
}
