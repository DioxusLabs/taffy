//! Helper functions which it make it easier to create instances of types in the `style` and `geometry` modules.
use crate::{
    geometry::{Line, Point, Rect, Size},
    style::LengthPercentage,
};

#[cfg(feature = "grid")]
use crate::{
    geometry::MinMax,
    style::{
        GridTrackRepetition, MaxTrackSizingFunction, MinTrackSizingFunction, NonRepeatedTrackSizingFunction,
        TrackSizingFunction,
    },
    util::sys::Vec,
};
#[cfg(feature = "grid")]
use core::fmt::Debug;

/// Returns an auto-repeated track definition
#[cfg(feature = "grid")]
pub fn repeat<Input>(repetition_kind: Input, track_list: Vec<NonRepeatedTrackSizingFunction>) -> TrackSizingFunction
where
    Input: TryInto<GridTrackRepetition>,
    <Input as TryInto<GridTrackRepetition>>::Error: Debug,
{
    TrackSizingFunction::Repeat(repetition_kind.try_into().unwrap(), track_list)
}

#[cfg(feature = "grid")]
/// Returns a grid template containing `count` evenly sized tracks
pub fn evenly_sized_tracks(count: u16) -> Vec<TrackSizingFunction> {
    use crate::util::sys::new_vec_with_capacity;
    let mut repeated_tracks = new_vec_with_capacity(1);
    repeated_tracks.push(flex(1.0f32));
    let mut tracks = new_vec_with_capacity(1);
    tracks.push(repeat(count, repeated_tracks));
    tracks
}

/// Specifies a grid line to place a grid item between in CSS Grid Line coordinates:
///  - Positive indices count upwards from the start (top or left) of the explicit grid
///  - Negative indices count downwards from the end (bottom or right) of the explicit grid
///  - ZERO IS INVALID index, and will be treated as a GridPlacement::Auto.
pub fn line<T: TaffyGridLine>(index: i16) -> T {
    T::from_line_index(index)
}
/// Trait to abstract over grid line values
pub trait TaffyGridLine {
    /// Converts an i16 into Self
    fn from_line_index(index: i16) -> Self;
}

/// Returns a GridPlacement::Span
pub fn span<T: TaffyGridSpan>(span: u16) -> T {
    T::from_span(span)
}
/// Trait to abstract over grid span values
pub trait TaffyGridSpan {
    /// Converts an u16 into Self
    fn from_span(span: u16) -> Self;
}

/// Returns a MinMax with min value of min and max value of max
#[cfg(feature = "grid")]
pub fn minmax<Output>(min: MinTrackSizingFunction, max: MaxTrackSizingFunction) -> Output
where
    Output: From<MinMax<MinTrackSizingFunction, MaxTrackSizingFunction>>,
{
    MinMax { min, max }.into()
}

/// Shorthand for minmax(0, Nfr). Probably what you want if you want exactly evenly sized tracks.
#[cfg(feature = "grid")]
pub fn flex<Input, Output>(flex_fraction: Input) -> Output
where
    Input: Into<f32> + Copy,
    Output: From<MinMax<MinTrackSizingFunction, MaxTrackSizingFunction>>,
{
    MinMax { min: zero(), max: fr(flex_fraction.into()) }.into()
}

/// Returns the zero value for that type
pub const fn zero<T: TaffyZero>() -> T {
    T::ZERO
}

/// Trait to abstract over zero values
pub trait TaffyZero {
    /// The zero value for type implementing TaffyZero
    const ZERO: Self;
}
impl TaffyZero for f32 {
    const ZERO: f32 = 0.0;
}
impl<T: TaffyZero> TaffyZero for Option<T> {
    const ZERO: Option<T> = Some(T::ZERO);
}
impl<T: TaffyZero> TaffyZero for Point<T> {
    const ZERO: Point<T> = Point { x: T::ZERO, y: T::ZERO };
}
impl<T: TaffyZero> Point<T> {
    /// Returns a Point where both the x and y values are the zero value of the contained type
    /// (e.g. 0.0, Some(0.0), or Dimension::Length(0.0))
    pub const fn zero() -> Self {
        zero::<Self>()
    }
}
impl<T: TaffyZero> TaffyZero for Line<T> {
    const ZERO: Line<T> = Line { start: T::ZERO, end: T::ZERO };
}
impl<T: TaffyZero> Line<T> {
    /// Returns a Line where both the start and end values are the zero value of the contained type
    /// (e.g. 0.0, Some(0.0), or Dimension::Length(0.0))
    pub const fn zero() -> Self {
        zero::<Self>()
    }
}
impl<T: TaffyZero> TaffyZero for Size<T> {
    const ZERO: Size<T> = Size { width: T::ZERO, height: T::ZERO };
}
impl<T: TaffyZero> Size<T> {
    /// Returns a Size where both the width and height values are the zero value of the contained type
    /// (e.g. 0.0, Some(0.0), or Dimension::Length(0.0))
    pub const fn zero() -> Self {
        zero::<Self>()
    }
}
impl<T: TaffyZero> TaffyZero for Rect<T> {
    const ZERO: Rect<T> = Rect { left: T::ZERO, right: T::ZERO, top: T::ZERO, bottom: T::ZERO };
}
impl<T: TaffyZero> Rect<T> {
    /// Returns a Rect where the left, right, top, and bottom values are all the zero value of the contained type
    /// (e.g. 0.0, Some(0.0), or Dimension::Length(0.0))
    pub const fn zero() -> Self {
        zero::<Self>()
    }
}

/// Returns the auto value for that type
pub const fn auto<T: TaffyAuto>() -> T {
    T::AUTO
}

/// Trait to abstract over auto values
pub trait TaffyAuto {
    /// The auto value for type implementing TaffyAuto
    const AUTO: Self;
}
impl<T: TaffyAuto> TaffyAuto for Option<T> {
    const AUTO: Option<T> = Some(T::AUTO);
}
impl<T: TaffyAuto> TaffyAuto for Point<T> {
    const AUTO: Point<T> = Point { x: T::AUTO, y: T::AUTO };
}
impl<T: TaffyAuto> Point<T> {
    /// Returns a Point where both the x and y values are the auto value of the contained type
    /// (e.g. Dimension::Auto or LengthPercentageAuto::Auto)
    pub const fn auto() -> Self {
        auto::<Self>()
    }
}
impl<T: TaffyAuto> TaffyAuto for Line<T> {
    const AUTO: Line<T> = Line { start: T::AUTO, end: T::AUTO };
}
impl<T: TaffyAuto> Line<T> {
    /// Returns a Line where both the start and end values are the auto value of the contained type
    /// (e.g. Dimension::Auto or LengthPercentageAuto::Auto)
    pub const fn auto() -> Self {
        auto::<Self>()
    }
}
impl<T: TaffyAuto> TaffyAuto for Size<T> {
    const AUTO: Size<T> = Size { width: T::AUTO, height: T::AUTO };
}
impl<T: TaffyAuto> Size<T> {
    /// Returns a Size where both the width and height values are the auto value of the contained type
    /// (e.g. Dimension::Auto or LengthPercentageAuto::Auto)
    pub const fn auto() -> Self {
        auto::<Self>()
    }
}
impl<T: TaffyAuto> TaffyAuto for Rect<T> {
    const AUTO: Rect<T> = Rect { left: T::AUTO, right: T::AUTO, top: T::AUTO, bottom: T::AUTO };
}
impl<T: TaffyAuto> Rect<T> {
    /// Returns a Rect where the left, right, top, and bottom values are all the auto value of the contained type
    /// (e.g. Dimension::Auto or LengthPercentageAuto::Auto)
    pub const fn auto() -> Self {
        auto::<Self>()
    }
}

/// Returns the auto value for that type
pub const fn min_content<T: TaffyMinContent>() -> T {
    T::MIN_CONTENT
}

/// Trait to abstract over min_content values
pub trait TaffyMinContent {
    /// The min_content value for type implementing TaffyZero
    const MIN_CONTENT: Self;
}
impl<T: TaffyMinContent> TaffyMinContent for Option<T> {
    const MIN_CONTENT: Option<T> = Some(T::MIN_CONTENT);
}
impl<T: TaffyMinContent> TaffyMinContent for Point<T> {
    const MIN_CONTENT: Point<T> = Point { x: T::MIN_CONTENT, y: T::MIN_CONTENT };
}
impl<T: TaffyMinContent> Point<T> {
    /// Returns a Point where both the x and y values are the min_content value of the contained type
    /// (e.g. Dimension::Auto or LengthPercentageAuto::Auto)
    pub const fn min_content() -> Self {
        min_content::<Self>()
    }
}
impl<T: TaffyMinContent> TaffyMinContent for Line<T> {
    const MIN_CONTENT: Line<T> = Line { start: T::MIN_CONTENT, end: T::MIN_CONTENT };
}
impl<T: TaffyMinContent> Line<T> {
    /// Returns a Line where both the start and end values are the min_content value of the contained type
    /// (e.g. Dimension::Auto or LengthPercentageAuto::Auto)
    pub const fn min_content() -> Self {
        min_content::<Self>()
    }
}
impl<T: TaffyMinContent> TaffyMinContent for Size<T> {
    const MIN_CONTENT: Size<T> = Size { width: T::MIN_CONTENT, height: T::MIN_CONTENT };
}
impl<T: TaffyMinContent> Size<T> {
    /// Returns a Size where both the width and height values are the min_content value of the contained type
    /// (e.g. Dimension::Auto or LengthPercentageAuto::Auto)
    pub const fn min_content() -> Self {
        min_content::<Self>()
    }
}
impl<T: TaffyMinContent> TaffyMinContent for Rect<T> {
    const MIN_CONTENT: Rect<T> =
        Rect { left: T::MIN_CONTENT, right: T::MIN_CONTENT, top: T::MIN_CONTENT, bottom: T::MIN_CONTENT };
}
impl<T: TaffyMinContent> Rect<T> {
    /// Returns a Rect where the left, right, top, and bottom values are all the min_content value of the contained type
    /// (e.g. Dimension::Auto or LengthPercentageAuto::Auto)
    pub const fn min_content() -> Self {
        min_content::<Self>()
    }
}

/// Returns the auto value for that type
pub const fn max_content<T: TaffyMaxContent>() -> T {
    T::MAX_CONTENT
}

/// Trait to abstract over max_content values
pub trait TaffyMaxContent {
    /// The max_content value for type implementing TaffyZero
    const MAX_CONTENT: Self;
}
impl<T: TaffyMaxContent> TaffyMaxContent for Option<T> {
    const MAX_CONTENT: Option<T> = Some(T::MAX_CONTENT);
}
impl<T: TaffyMaxContent> TaffyMaxContent for Point<T> {
    const MAX_CONTENT: Point<T> = Point { x: T::MAX_CONTENT, y: T::MAX_CONTENT };
}
impl<T: TaffyMaxContent> Point<T> {
    /// Returns a Point where both the x and y values are the max_content value of the contained type
    /// (e.g. Dimension::Auto or LengthPercentageAuto::Auto)
    pub const fn max_content() -> Self {
        max_content::<Self>()
    }
}
impl<T: TaffyMaxContent> TaffyMaxContent for Line<T> {
    const MAX_CONTENT: Line<T> = Line { start: T::MAX_CONTENT, end: T::MAX_CONTENT };
}
impl<T: TaffyMaxContent> Line<T> {
    /// Returns a Line where both the start and end values are the max_content value of the contained type
    /// (e.g. Dimension::Auto or LengthPercentageAuto::Auto)
    pub const fn max_content() -> Self {
        max_content::<Self>()
    }
}
impl<T: TaffyMaxContent> TaffyMaxContent for Size<T> {
    const MAX_CONTENT: Size<T> = Size { width: T::MAX_CONTENT, height: T::MAX_CONTENT };
}
impl<T: TaffyMaxContent> Size<T> {
    /// Returns a Size where both the width and height values are the max_content value of the contained type
    /// (e.g. Dimension::Auto or LengthPercentageAuto::Auto)
    pub const fn max_content() -> Self {
        max_content::<Self>()
    }
}
impl<T: TaffyMaxContent> TaffyMaxContent for Rect<T> {
    const MAX_CONTENT: Rect<T> =
        Rect { left: T::MAX_CONTENT, right: T::MAX_CONTENT, top: T::MAX_CONTENT, bottom: T::MAX_CONTENT };
}
impl<T: TaffyMaxContent> Rect<T> {
    /// Returns a Rect where the left, right, top, and bottom values are all the max_content value of the contained type
    /// (e.g. Dimension::Auto or LengthPercentageAuto::Auto)
    pub const fn max_content() -> Self {
        max_content::<Self>()
    }
}

/// Returns a value of the inferred type which represent a `fit-content(…)` value
/// with the given argument.
pub fn fit_content<T: TaffyFitContent>(argument: LengthPercentage) -> T {
    T::fit_content(argument)
}

/// Trait to create `fit-content(…)` values from plain numbers
pub trait TaffyFitContent {
    /// Converts a LengthPercentage into Self
    fn fit_content(argument: LengthPercentage) -> Self;
}
impl<T: TaffyFitContent> TaffyFitContent for Point<T> {
    fn fit_content(argument: LengthPercentage) -> Self {
        Point { x: T::fit_content(argument), y: T::fit_content(argument) }
    }
}
impl<T: TaffyFitContent> Point<T> {
    /// Returns a Point with x and y set to the same `fit-content(…)` value
    /// with the given argument.
    pub fn fit_content(argument: LengthPercentage) -> Self {
        fit_content(argument)
    }
}
impl<T: TaffyFitContent> TaffyFitContent for Line<T> {
    fn fit_content(argument: LengthPercentage) -> Self {
        Line { start: T::fit_content(argument), end: T::fit_content(argument) }
    }
}
impl<T: TaffyFitContent> Line<T> {
    /// Returns a Line with start and end set to the same `fit-content(…)` value
    /// with the given argument.
    pub fn fit_content(argument: LengthPercentage) -> Self {
        fit_content(argument)
    }
}
impl<T: TaffyFitContent> TaffyFitContent for Size<T> {
    fn fit_content(argument: LengthPercentage) -> Self {
        Size { width: T::fit_content(argument), height: T::fit_content(argument) }
    }
}
impl<T: TaffyFitContent> Size<T> {
    /// Returns a Size where with width and height set to the same `fit-content(…)` value
    /// with the given argument.
    pub fn fit_content(argument: LengthPercentage) -> Self {
        fit_content(argument)
    }
}
impl<T: TaffyFitContent> TaffyFitContent for Rect<T> {
    fn fit_content(argument: LengthPercentage) -> Self {
        Rect {
            left: T::fit_content(argument),
            right: T::fit_content(argument),
            top: T::fit_content(argument),
            bottom: T::fit_content(argument),
        }
    }
}
impl<T: TaffyFitContent> Rect<T> {
    /// Returns a Rect where the left, right, top and bottom values are all constant fit_content value of the contained type
    /// (e.g. 2.1, Some(2.1), or Dimension::Length(2.1))
    pub fn fit_content(argument: LengthPercentage) -> Self {
        fit_content(argument)
    }
}

/// Returns a value of the inferred type which represent an absolute length
pub fn length<Input: Into<f32> + Copy, T: FromLength>(value: Input) -> T {
    T::from_length(value)
}

/// Trait to create absolute length values from plain numbers
pub trait FromLength {
    /// Converts into an `Into<f32>` into Self
    fn from_length<Input: Into<f32> + Copy>(value: Input) -> Self;
}
impl FromLength for f32 {
    fn from_length<Input: Into<f32> + Copy>(value: Input) -> Self {
        value.into()
    }
}
impl FromLength for Option<f32> {
    fn from_length<Input: Into<f32> + Copy>(value: Input) -> Self {
        Some(value.into())
    }
}
impl<T: FromLength> FromLength for Point<T> {
    fn from_length<Input: Into<f32> + Copy>(value: Input) -> Self {
        Point { x: T::from_length(value.into()), y: T::from_length(value.into()) }
    }
}
impl<T: FromLength> Point<T> {
    /// Returns a Point where x and y values are the same given absolute length
    pub fn length<Input: Into<f32> + Copy>(value: Input) -> Self {
        length::<Input, Self>(value)
    }
}
impl<T: FromLength> FromLength for Line<T> {
    fn from_length<Input: Into<f32> + Copy>(value: Input) -> Self {
        Line { start: T::from_length(value.into()), end: T::from_length(value.into()) }
    }
}
impl<T: FromLength> Line<T> {
    /// Returns a Line where both the start and end values are the same given absolute length
    pub fn length<Input: Into<f32> + Copy>(value: Input) -> Self {
        length::<Input, Self>(value)
    }
}
impl<T: FromLength> FromLength for Size<T> {
    fn from_length<Input: Into<f32> + Copy>(value: Input) -> Self {
        Size { width: T::from_length(value.into()), height: T::from_length(value.into()) }
    }
}
impl<T: FromLength> Size<T> {
    /// Returns a Size where both the width and height values the same given absolute length
    pub fn length<Input: Into<f32> + Copy>(value: Input) -> Self {
        length::<Input, Self>(value)
    }
}
impl<T: FromLength> FromLength for Rect<T> {
    fn from_length<Input: Into<f32> + Copy>(value: Input) -> Self {
        Rect {
            left: T::from_length(value.into()),
            right: T::from_length(value.into()),
            top: T::from_length(value.into()),
            bottom: T::from_length(value.into()),
        }
    }
}
impl<T: FromLength> Rect<T> {
    /// Returns a Rect where the left, right, top and bottom values are all the same given absolute length
    pub fn length<Input: Into<f32> + Copy>(value: Input) -> Self {
        length::<Input, Self>(value)
    }
}

/// Returns a value of the inferred type which represent a percentage
pub fn percent<Input: Into<f32> + Copy, T: FromPercent>(percent: Input) -> T {
    T::from_percent(percent)
}

/// Trait to create constant percent values from plain numbers
pub trait FromPercent {
    /// Converts into an `Into<f32>` into Self
    fn from_percent<Input: Into<f32> + Copy>(percent: Input) -> Self;
}
impl FromPercent for f32 {
    fn from_percent<Input: Into<f32> + Copy>(percent: Input) -> Self {
        percent.into()
    }
}
impl FromPercent for Option<f32> {
    fn from_percent<Input: Into<f32> + Copy>(percent: Input) -> Self {
        Some(percent.into())
    }
}
impl<T: FromPercent> FromPercent for Point<T> {
    fn from_percent<Input: Into<f32> + Copy>(percent: Input) -> Self {
        Point { x: T::from_percent(percent.into()), y: T::from_percent(percent.into()) }
    }
}
impl<T: FromPercent> Point<T> {
    /// Returns a Point where both the x and y values are the constant percent value of the contained type
    /// (e.g. 2.1, Some(2.1), or Dimension::Length(2.1))
    pub fn percent<Input: Into<f32> + Copy>(percent_value: Input) -> Self {
        percent::<Input, Self>(percent_value)
    }
}
impl<T: FromPercent> FromPercent for Line<T> {
    fn from_percent<Input: Into<f32> + Copy>(percent: Input) -> Self {
        Line { start: T::from_percent(percent.into()), end: T::from_percent(percent.into()) }
    }
}
impl<T: FromPercent> Line<T> {
    /// Returns a Line where both the start and end values are the constant percent value of the contained type
    /// (e.g. 2.1, Some(2.1), or Dimension::Length(2.1))
    pub fn percent<Input: Into<f32> + Copy>(percent_value: Input) -> Self {
        percent::<Input, Self>(percent_value)
    }
}
impl<T: FromPercent> FromPercent for Size<T> {
    fn from_percent<Input: Into<f32> + Copy>(percent: Input) -> Self {
        Size { width: T::from_percent(percent.into()), height: T::from_percent(percent.into()) }
    }
}
impl<T: FromPercent> Size<T> {
    /// Returns a Size where both the width and height values are the constant percent value of the contained type
    /// (e.g. 2.1, Some(2.1), or Dimension::Length(2.1))
    pub fn percent<Input: Into<f32> + Copy>(percent_value: Input) -> Self {
        percent::<Input, Self>(percent_value)
    }
}
impl<T: FromPercent> FromPercent for Rect<T> {
    fn from_percent<Input: Into<f32> + Copy>(percent: Input) -> Self {
        Rect {
            left: T::from_percent(percent.into()),
            right: T::from_percent(percent.into()),
            top: T::from_percent(percent.into()),
            bottom: T::from_percent(percent.into()),
        }
    }
}
impl<T: FromPercent> Rect<T> {
    /// Returns a Rect where the left, right, top and bottom values are all constant percent value of the contained type
    /// (e.g. 2.1, Some(2.1), or Dimension::Length(2.1))
    pub fn percent<Input: Into<f32> + Copy>(percent_value: Input) -> Self {
        percent::<Input, Self>(percent_value)
    }
}

/// Create a `Fraction` track sizing function (`fr` in CSS)
#[cfg(feature = "grid")]
pub fn fr<Input: Into<f32> + Copy, T: FromFr>(flex: Input) -> T {
    T::from_fr(flex)
}

/// Trait to create constant percent values from plain numbers
pub trait FromFr {
    /// Converts into an `Into<f32>` into Self
    fn from_fr<Input: Into<f32> + Copy>(flex: Input) -> Self;
}

#[cfg(feature = "grid")]
#[cfg(test)]
mod repeat_fn_tests {
    use super::repeat;
    use crate::style::{GridTrackRepetition, NonRepeatedTrackSizingFunction, TrackSizingFunction};

    const TEST_VEC: Vec<NonRepeatedTrackSizingFunction> = Vec::new();

    #[test]
    fn test_repeat_u16() {
        assert_eq!(repeat(123, TEST_VEC), TrackSizingFunction::Repeat(GridTrackRepetition::Count(123), TEST_VEC));
    }

    #[test]
    fn test_repeat_auto_fit_str() {
        assert_eq!(repeat("auto-fit", TEST_VEC), TrackSizingFunction::Repeat(GridTrackRepetition::AutoFit, TEST_VEC));
    }

    #[test]
    fn test_repeat_auto_fill_str() {
        assert_eq!(repeat("auto-fill", TEST_VEC), TrackSizingFunction::Repeat(GridTrackRepetition::AutoFill, TEST_VEC));
    }
}
