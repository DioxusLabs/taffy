//! Helper functions which it make it easier to create instances of types in the `style` and `geometry` modules.
use crate::{
    geometry::{MinMax, Point, Rect, Size},
    style::LengthPercentage,
};

#[cfg(feature = "grid")]
use crate::style::{GridTrackRepetition, NonRepeatedTrackSizingFunction, TrackSizingFunction};

/// Returns an auto-repeated track definition
#[cfg(feature = "grid")]
pub fn repeat(
    repetition_kind: GridTrackRepetition,
    track_list: Vec<NonRepeatedTrackSizingFunction>,
) -> TrackSizingFunction {
    TrackSizingFunction::AutoRepeat(repetition_kind, track_list)
}

/// Returns a GridPlacement::Line
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
    /// Converts an iu6 into Self
    fn from_span(span: u16) -> Self;
}

/// Returns a MinMax with min value of min and max value of max
pub fn minmax<Min, Max, Output: From<MinMax<Min, Max>>>(min: Min, max: Max) -> Output {
    MinMax { min, max }.into()
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
    /// (e.g. 0.0, Some(0.0), or Dimension::Points(0.0))
    pub const fn zero() -> Self {
        zero::<Self>()
    }
}
impl<T: TaffyZero> TaffyZero for Size<T> {
    const ZERO: Size<T> = Size { width: T::ZERO, height: T::ZERO };
}
impl<T: TaffyZero> Size<T> {
    /// Returns a Size where both the width and height values are the zero value of the contained type
    /// (e.g. 0.0, Some(0.0), or Dimension::Points(0.0))
    pub const fn zero() -> Self {
        zero::<Self>()
    }
}
impl<T: TaffyZero> TaffyZero for Rect<T> {
    const ZERO: Rect<T> = Rect { left: T::ZERO, right: T::ZERO, top: T::ZERO, bottom: T::ZERO };
}
impl<T: TaffyZero> Rect<T> {
    /// Returns a Size where the left, right, top, and bottom values are all the zero value of the contained type
    /// (e.g. 0.0, Some(0.0), or Dimension::Points(0.0))
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
    /// The auto value for type implementing TaffyZero
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
    /// Returns a Size where the left, right, top, and bottom values are all the auto value of the contained type
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
    /// Returns a Size where the left, right, top, and bottom values are all the min_content value of the contained type
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
    /// Returns a Size where the left, right, top, and bottom values are all the max_content value of the contained type
    /// (e.g. Dimension::Auto or LengthPercentageAuto::Auto)
    pub const fn max_content() -> Self {
        max_content::<Self>()
    }
}

/// Returns a value of the inferred type which represent a constant of points
pub fn fit_content<T: TaffyFitContent>(argument: LengthPercentage) -> T {
    T::fit_content(argument)
}

/// Trait to create constant points values from plain numbers
pub trait TaffyFitContent {
    /// Converts into a LengthPercentage into Self
    fn fit_content(argument: LengthPercentage) -> Self;
}
impl<T: TaffyFitContent> TaffyFitContent for Point<T> {
    fn fit_content(argument: LengthPercentage) -> Self {
        Point { x: T::fit_content(argument), y: T::fit_content(argument) }
    }
}
impl<T: TaffyFitContent> Point<T> {
    /// Returns a Point where both the x and y values are the constant points value of the contained type
    /// (e.g. 2.1, Some(2.1), or Dimension::Points(2.1))
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
    /// Returns a Size where both the width and height values are the constant fit_content value of the contained type
    /// (e.g. 2.1, Some(2.1), or Dimension::Points(2.1))
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
    /// (e.g. 2.1, Some(2.1), or Dimension::Points(2.1))
    pub fn fit_content(argument: LengthPercentage) -> Self {
        fit_content(argument)
    }
}

/// Returns a value of the inferred type which represent a constant of points
pub fn points<Input: Into<f32> + Copy, T: FromPoints>(points: Input) -> T {
    T::from_points(points)
}

/// Trait to create constant points values from plain numbers
pub trait FromPoints {
    /// Converts into an Into<f32> into Self
    fn from_points<Input: Into<f32> + Copy>(points: Input) -> Self;
}
impl FromPoints for f32 {
    fn from_points<Input: Into<f32> + Copy>(points: Input) -> Self {
        points.into()
    }
}
impl FromPoints for Option<f32> {
    fn from_points<Input: Into<f32> + Copy>(points: Input) -> Self {
        Some(points.into())
    }
}
impl<T: FromPoints> FromPoints for Point<T> {
    fn from_points<Input: Into<f32> + Copy>(points: Input) -> Self {
        Point { x: T::from_points(points.into()), y: T::from_points(points.into()) }
    }
}
impl<T: FromPoints> Point<T> {
    /// Returns a Point where both the x and y values are the constant points value of the contained type
    /// (e.g. 2.1, Some(2.1), or Dimension::Points(2.1))
    pub fn points<Input: Into<f32> + Copy>(points_value: Input) -> Self {
        points::<Input, Self>(points_value)
    }
}
impl<T: FromPoints> FromPoints for Size<T> {
    fn from_points<Input: Into<f32> + Copy>(points: Input) -> Self {
        Size { width: T::from_points(points.into()), height: T::from_points(points.into()) }
    }
}
impl<T: FromPoints> Size<T> {
    /// Returns a Size where both the width and height values are the constant points value of the contained type
    /// (e.g. 2.1, Some(2.1), or Dimension::Points(2.1))
    pub fn points<Input: Into<f32> + Copy>(points_value: Input) -> Self {
        points::<Input, Self>(points_value)
    }
}
impl<T: FromPoints> FromPoints for Rect<T> {
    fn from_points<Input: Into<f32> + Copy>(points: Input) -> Self {
        Rect {
            left: T::from_points(points.into()),
            right: T::from_points(points.into()),
            top: T::from_points(points.into()),
            bottom: T::from_points(points.into()),
        }
    }
}
impl<T: FromPoints> Rect<T> {
    /// Returns a Rect where the left, right, top and bottom values are all constant points value of the contained type
    /// (e.g. 2.1, Some(2.1), or Dimension::Points(2.1))
    pub fn points<Input: Into<f32> + Copy>(points_value: Input) -> Self {
        points::<Input, Self>(points_value)
    }
}

/// Returns a value of the inferred type which represent a constant of points
pub fn percent<Input: Into<f32> + Copy, T: FromPercent>(percent: Input) -> T {
    T::from_percent(percent)
}

/// Trait to create constant percent values from plain numbers
pub trait FromPercent {
    /// Converts into an Into<f32> into Self
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
    /// (e.g. 2.1, Some(2.1), or Dimension::Points(2.1))
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
    /// (e.g. 2.1, Some(2.1), or Dimension::Points(2.1))
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
    /// (e.g. 2.1, Some(2.1), or Dimension::Points(2.1))
    pub fn percent<Input: Into<f32> + Copy>(percent_value: Input) -> Self {
        percent::<Input, Self>(percent_value)
    }
}

/// Returns a value of the inferred type which represents a flex fraction
pub fn flex<Input: Into<f32> + Copy, T: FromFlex>(flex: Input) -> T {
    T::from_flex(flex)
}

/// Trait to create constant percent values from plain numbers
pub trait FromFlex {
    /// Converts into an Into<f32> into Self
    fn from_flex<Input: Into<f32> + Copy>(flex: Input) -> Self;
}
