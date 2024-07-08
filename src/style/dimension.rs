//! Style types for representing lengths / sizes

use core::fmt::Debug;
#[cfg(feature = "calc")]
use core::{
    mem::{transmute, transmute_copy},
    ops::Neg,
};

use num_traits::Signed;
#[cfg(feature = "calc")]
use num_traits::Zero;
#[cfg(feature = "calc")]
use sptr::{from_exposed_addr, Strict};

use crate::geometry::{Rect, Size};
use crate::style_helpers::{FromLength, FromPercent, TaffyAuto, TaffyMaxContent, TaffyMinContent, TaffyZero};
#[cfg(feature = "calc")]
use crate::sys::{Arc, Box};
use crate::util::sys::abs;

#[cfg(feature = "calc")]
pub(super) union CalcVariant {
    #[cfg(target_pointer_width = "32")]
    ptr: (u32, *const CalcNode),
    #[cfg(target_pointer_width = "64")]
    ptr: u64,

    tag: u8,
}
#[cfg(feature = "calc")]
impl CalcVariant {
    const PTR_SHIFT: u8 = 5;
    const CALC_VARIANT_DISCRIMINANT: u8 = 0;

    #[inline]
    fn new(calc_node: CalcNode) -> Self {
        let arc_ptr = Arc::into_raw(Arc::new(calc_node));

        #[cfg(target_pointer_width = "32")]
        return Self { ptr: (0, arc_ptr) }.set_tag();
        #[cfg(target_pointer_width = "64")]
        return Self { ptr: (arc_ptr.expose_addr() as u64) << Self::PTR_SHIFT }.set_tag();
    }
    #[inline]
    const fn set_tag(mut self) -> Self {
        self.tag = Self::CALC_VARIANT_DISCRIMINANT;
        self
    }
    #[inline]
    unsafe fn get_ptr(&self) -> *const CalcNode {
        #[cfg(target_pointer_width = "32")]
        return self.ptr.1;
        #[cfg(target_pointer_width = "64")]
        return from_exposed_addr((self.ptr >> Self::PTR_SHIFT) as usize);
    }
    #[inline]
    fn get_calc(&self) -> Option<&CalcNode> {
        unsafe {
            if self.tag == Self::CALC_VARIANT_DISCRIMINANT {
                Some(&*self.get_ptr())
            } else {
                None
            }
        }
    }
    #[inline]
    unsafe fn increment_strong_count(&self) {
        Arc::increment_strong_count(self.get_ptr());
    }
    #[inline]
    unsafe fn decrement_strong_count(&self) {
        Arc::decrement_strong_count(self.get_ptr());
    }
}

macro_rules! impl_calc {
    ($ty:ident) => {
        impl $ty {
            #[inline]
            pub fn calc(calc_node: CalcNode) -> Self {
                unsafe { Self::from_calc_variant(CalcVariant::new(calc_node)) }
            }
            #[inline]
            pub fn get_calc(&self) -> Option<&CalcNode> {
                unsafe { self.to_calc_variant() }.get_calc()
            }
            #[inline]
            pub unsafe fn get_calc_unchecked(&self) -> &CalcNode {
                unsafe { &*self.to_calc_variant().get_ptr() }
            }
            #[inline]
            const fn from_calc_variant(cv: CalcVariant) -> Self {
                unsafe { transmute::<CalcVariant, Self>(cv) }
            }
            #[inline]
            const unsafe fn into_calc_variant(self) -> CalcVariant {
                transmute::<Self, CalcVariant>(self)
            }
            #[inline]
            const unsafe fn to_calc_variant(&self) -> &CalcVariant {
                transmute::<&Self, &CalcVariant>(self)
            }
        }

        impl Clone for $ty {
            fn clone(&self) -> Self {
                if let $ty::Calc = self {
                    unsafe { self.to_calc_variant().increment_strong_count() };
                }
                unsafe { transmute_copy(self) }
            }
        }
        impl Drop for $ty {
            fn drop(&mut self) {
                if let $ty::Calc = self {
                    unsafe { self.to_calc_variant().decrement_strong_count() };
                }
            }
        }
    };
}

/// A unit of linear measurement
///
/// This is commonly combined with [`Rect`], [`Point`](crate::geometry::Point) and [`Size<T>`].
#[derive(PartialEq, Debug)]
#[cfg_attr(not(feature = "calc"), derive(Clone))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(u8, C, align(8))]
pub enum LengthPercentage {
    #[cfg(feature = "calc")]
    /// Represents an CSS calc() <https://www.w3.org/TR/css-values-4/#funcdef-calc>
    /// # Safety
    /// Only construct this variant with the associated [LengthPercentage::calc()] function.
    /// Constructing this from the enum Variant can cause unexpected behaviour like program crashes.
    Calc = CalcVariant::CALC_VARIANT_DISCRIMINANT,
    /// An absolute length in some abstract units. Users of Taffy may define what they correspond
    /// to in their application (pixels, logical pixels, mm, etc) as they see fit.
    Length(f32),
    /// The dimension is stored in percentage relative to the parent item.
    Percent(f32),
}
impl_calc!(LengthPercentage);
impl LengthPercentage {
    pub fn resolve(&self, percentage_length: f32) -> f32 {
        match self {
            LengthPercentage::Length(length) => *length,
            LengthPercentage::Percent(fraction) => fraction * percentage_length,
            #[cfg(feature = "calc")]
            LengthPercentage::Calc => unsafe { self.get_calc_unchecked().resolve(percentage_length) },
        }
    }
}
impl TaffyZero for LengthPercentage {
    const ZERO: Self = Self::Length(0.0);
}
impl FromLength for LengthPercentage {
    fn from_length<Input: Into<f32> + Copy>(value: Input) -> Self {
        Self::Length(value.into())
    }
}
impl FromPercent for LengthPercentage {
    fn from_percent<Input: Into<f32> + Copy>(percent: Input) -> Self {
        Self::Percent(percent.into())
    }
}

/// A unit of linear measurement
///
/// This is commonly combined with [`Rect`], [`Point`](crate::geometry::Point) and [`Size<T>`].
#[derive(PartialEq, Debug)]
#[cfg_attr(not(feature = "calc"), derive(Clone))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(u8, C, align(8))]
pub enum LengthPercentageAuto {
    #[cfg(feature = "calc")]
    /// Represents an CSS calc() <https://www.w3.org/TR/css-values-4/#funcdef-calc>
    /// # Safety
    /// Only construct this variant with the associated [LengthPercentageAuto::calc()] function.
    /// Constructing this from the enum Variant can cause unexpected behaviour like program crashes.
    Calc = CalcVariant::CALC_VARIANT_DISCRIMINANT,
    /// An absolute length in some abstract units. Users of Taffy may define what they correspond
    /// to in their application (pixels, logical pixels, mm, etc) as they see fit.
    Length(f32),
    /// The dimension is stored in percentage relative to the parent item.
    Percent(f32),
    /// The dimension should be automatically computed
    Auto,
}
impl_calc!(LengthPercentageAuto);

impl TaffyZero for LengthPercentageAuto {
    const ZERO: Self = Self::Length(0.0);
}
impl TaffyAuto for LengthPercentageAuto {
    const AUTO: Self = Self::Auto;
}
impl FromLength for LengthPercentageAuto {
    fn from_length<Input: Into<f32> + Copy>(value: Input) -> Self {
        Self::Length(value.into())
    }
}
impl FromPercent for LengthPercentageAuto {
    fn from_percent<Input: Into<f32> + Copy>(percent: Input) -> Self {
        Self::Percent(percent.into())
    }
}

impl From<LengthPercentage> for LengthPercentageAuto {
    fn from(input: LengthPercentage) -> Self {
        match input {
            LengthPercentage::Length(value) => Self::Length(value),
            LengthPercentage::Percent(value) => Self::Percent(value),
            #[cfg(feature = "calc")]
            LengthPercentage::Calc => Self::from_calc_variant(unsafe { input.into_calc_variant() }),
        }
    }
}

impl LengthPercentageAuto {
    /// Returns:
    ///   - Some(length) for Length variants
    ///   - Some(resolved) using the provided context for Percent or Calc variants
    ///   - None for Auto variants
    #[inline(always)]
    pub fn resolve_to_option(self, context: f32) -> Option<f32> {
        match self {
            LengthPercentageAuto::Length(length) => Some(length),
            LengthPercentageAuto::Percent(percent) => Some(context * percent),
            #[cfg(feature = "calc")]
            LengthPercentageAuto::Calc => self.get_calc().map(|calc| calc.resolve(context)),
            LengthPercentageAuto::Auto => None,
        }
    }
    /// Returns true if value is LengthPercentageAuto::Auto
    #[inline(always)]
    pub fn is_auto(&self) -> bool {
        self == &Self::Auto
    }
}

/// A unit of linear measurement
///
/// This is commonly combined with [`Rect`], [`Point`](crate::geometry::Point) and [`Size<T>`].
#[derive(PartialEq, Debug)]
#[cfg_attr(not(feature = "calc"), derive(Clone))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(u8, C, align(8))]
pub enum Dimension {
    #[cfg(feature = "calc")]
    /// Represents an CSS calc() <https://www.w3.org/TR/css-values-4/#funcdef-calc>
    /// # Safety
    /// Only construct this variant with the associated [Dimension::calc()] function.
    /// Constructing this from the enum Variant can cause unexpected behaviour like program crashes.
    Calc = CalcVariant::CALC_VARIANT_DISCRIMINANT,
    /// An absolute length in some abstract units. Users of Taffy may define what they correspond
    /// to in their application (pixels, logical pixels, mm, etc) as they see fit.
    Length(f32),
    /// The dimension is stored in percentage relative to the parent item.
    Percent(f32),
    /// The dimension should be automatically computed
    Auto,
}
impl_calc!(Dimension);

impl TaffyZero for Dimension {
    const ZERO: Self = Self::Length(0.0);
}
impl TaffyAuto for Dimension {
    const AUTO: Self = Self::Auto;
}
impl FromLength for Dimension {
    fn from_length<Input: Into<f32> + Copy>(value: Input) -> Self {
        Self::Length(value.into())
    }
}
impl FromPercent for Dimension {
    fn from_percent<Input: Into<f32> + Copy>(percent: Input) -> Self {
        Self::Percent(percent.into())
    }
}

impl From<LengthPercentage> for Dimension {
    fn from(input: LengthPercentage) -> Self {
        match input {
            LengthPercentage::Length(value) => Self::Length(value),
            LengthPercentage::Percent(value) => Self::Percent(value),
            #[cfg(feature = "calc")]
            LengthPercentage::Calc => Self::from_calc_variant(unsafe { input.into_calc_variant() }),
        }
    }
}

impl From<LengthPercentageAuto> for Dimension {
    fn from(input: LengthPercentageAuto) -> Self {
        match input {
            LengthPercentageAuto::Length(value) => Self::Length(value),
            LengthPercentageAuto::Percent(value) => Self::Percent(value),
            #[cfg(feature = "calc")]
            LengthPercentageAuto::Calc => Self::from_calc_variant(unsafe { input.into_calc_variant() }),
            LengthPercentageAuto::Auto => Self::Auto,
        }
    }
}

impl Dimension {
    /// Get Length value if value is Length variant
    #[cfg(feature = "grid")]
    pub fn into_option(self) -> Option<f32> {
        match self {
            Dimension::Length(value) => Some(value),
            _ => None,
        }
    }
    #[cfg(feature = "grid")]
    pub fn to_option(&self) -> Option<f32> {
        match self {
            Dimension::Length(value) => Some(*value),
            _ => None,
        }
    }
    /// Returns true if value is Dimension::Auto
    #[inline(always)]
    pub fn is_auto(self) -> bool {
        self == Self::Auto
    }
}

impl Rect<Dimension> {
    /// Create a new Rect with [`Dimension::Length`]
    #[must_use]
    pub const fn from_length(start: f32, end: f32, top: f32, bottom: f32) -> Self {
        Rect {
            left: Dimension::Length(start),
            right: Dimension::Length(end),
            top: Dimension::Length(top),
            bottom: Dimension::Length(bottom),
        }
    }

    /// Create a new Rect with [`Dimension::Percent`]
    #[must_use]
    pub const fn from_percent(start: f32, end: f32, top: f32, bottom: f32) -> Self {
        Rect {
            left: Dimension::Percent(start),
            right: Dimension::Percent(end),
            top: Dimension::Percent(top),
            bottom: Dimension::Percent(bottom),
        }
    }
}

/// A tree of nodes that can be found in an CSS calc() function.
#[cfg(feature = "calc")]
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum CalcNode {
    Leaf(LengthPercentage),

    /// Add the two values.
    Sum(Box<CalcNode>, Box<CalcNode>),
    /// Subtract the second value from the first.
    Difference(Box<CalcNode>, Box<CalcNode>),
    /// Multiplies the two values.
    Product(Box<CalcNode>, Box<CalcNode>),
    /// Divides the first value by the second.
    Quotient(Box<CalcNode>, Box<CalcNode>),

    /// Negates the value.
    Negate(Box<CalcNode>),

    /// Return the smallest value.
    /// <https://www.w3.org/TR/css-values-4/#funcdef-min>
    Min(Vec<CalcNode>),
    /// Returns the largest value.
    /// <https://www.w3.org/TR/css-values-4/#funcdef-max>
    Max(Vec<CalcNode>),
    /// Clamps value between min and max
    /// <https://www.w3.org/TR/css-values-4/#funcdef-clamp>
    Clamp {
        min: Box<CalcNode>,
        value: Box<CalcNode>,
        max: Box<CalcNode>,
    },
    /// Rounds value as defined by [RoundingStrategy].
    /// <https://www.w3.org/TR/css-values-4/#funcdef-round>
    Round {
        strategy: RoundingStrategy,
        value: Box<CalcNode>,
        interval: Box<CalcNode>,
    },
    /// Returns the remainder left over when the `dividend` is divided by the `divisor`.
    /// <https://www.w3.org/TR/css-values-4/#funcdef-rem>
    Rem {
        dividend: Box<CalcNode>,
        divisor: Box<CalcNode>,
    },
    /// Returns the modulus left over when the `dividend` is divided by the `divisor`.
    /// <https://www.w3.org/TR/css-values-4/#funcdef-mod>
    Mod {
        dividend: Box<CalcNode>,
        divisor: Box<CalcNode>,
    },
    /// Returns the square root of the sum of squares of its parameters.
    /// <https://www.w3.org/TR/css-values-4/#funcdef-hypot>
    Hypot(Vec<CalcNode>),
    /// Return `+1` for any positive value and `-1` for any negative value.
    /// Except for `-0` and `+0` in which themselves are return.
    /// <https://www.w3.org/TR/css-values-4/#funcdef-sign>
    Sign(Box<CalcNode>),
    /// Return the absolute value.
    /// <https://www.w3.org/TR/css-values-4/#funcdef-abs>
    Abs(Box<CalcNode>),
}
#[cfg(feature = "calc")]
impl CalcNode {
    /// Resolves the size of this [CalcNode]
    pub fn resolve(&self, percentage_length: f32) -> f32 {
        match self {
            CalcNode::Leaf(leaf) => leaf.resolve(percentage_length),
            CalcNode::Sum(lhs, rhs) => lhs.resolve(percentage_length) + rhs.resolve(percentage_length),
            CalcNode::Difference(lhs, rhs) => lhs.resolve(percentage_length) - rhs.resolve(percentage_length),
            CalcNode::Product(lhs, rhs) => lhs.resolve(percentage_length) * rhs.resolve(percentage_length),
            CalcNode::Quotient(lhs, rhs) => lhs.resolve(percentage_length) / rhs.resolve(percentage_length),
            CalcNode::Negate(node) => node.resolve(percentage_length).neg(),
            CalcNode::Min(nodes) => {
                nodes.iter().map(|node| node.resolve(percentage_length)).reduce(f32::min).unwrap_or_default()
            }
            CalcNode::Max(nodes) => {
                nodes.iter().map(|node| node.resolve(percentage_length)).reduce(f32::max).unwrap_or_default()
            }
            CalcNode::Clamp { min, value, max } => {
                let min_bound = min.resolve(percentage_length);
                let value = value.resolve(percentage_length);
                let max_bound = max.resolve(percentage_length);

                if min_bound.is_nan() || value.is_nan() || max_bound.is_nan() {
                    return f32::NAN;
                }

                value.max(max_bound).min(min_bound)
            }
            CalcNode::Round { strategy, value, interval } => {
                // https://developer.mozilla.org/en-US/docs/Web/CSS/round#return_value
                // https://drafts.csswg.org/css-values/#funcdef-round

                let value = value.resolve(percentage_length);
                let interval = interval.resolve(percentage_length);

                if interval.is_zero() {
                    return f32::NAN;
                }
                if value.is_infinite() {
                    if interval.is_infinite() {
                        return value;
                    }
                    return f32::NAN;
                }

                if interval.is_infinite() {
                    return match strategy {
                        RoundingStrategy::Up => {
                            if value.is_positive() && !value.is_zero() {
                                f32::INFINITY
                            } else if value.is_positive() {
                                0.0
                            } else {
                                -0.0
                            }
                        }
                        RoundingStrategy::Down => {
                            if value.is_negative() && !value.is_zero() {
                                f32::NEG_INFINITY
                            } else if value.is_negative() {
                                -0.0
                            } else {
                                0.0
                            }
                        }
                        RoundingStrategy::Nearest | RoundingStrategy::ToZero => {
                            if value.is_positive() {
                                0.0
                            } else {
                                -0.0
                            }
                        }
                    };
                }

                let div = value / interval;
                let lower_bound = div.floor() * interval;
                let upper_bound = div.ceil() * interval;

                match strategy {
                    RoundingStrategy::Up => upper_bound,
                    RoundingStrategy::Down => lower_bound,
                    RoundingStrategy::Nearest => {
                        if value - lower_bound < upper_bound - value {
                            lower_bound
                        } else {
                            upper_bound
                        }
                    }
                    RoundingStrategy::ToZero => {
                        if lower_bound.abs() < upper_bound.abs() {
                            lower_bound
                        } else {
                            upper_bound
                        }
                    }
                }
            }
            CalcNode::Rem { dividend, divisor } => {
                let dividend = dividend.resolve(percentage_length);
                let divisor = divisor.resolve(percentage_length);

                let rem = dividend - divisor * (dividend / divisor).trunc();
                if rem == 0.0 && dividend.is_negative() {
                    -0.0
                } else {
                    rem
                }
            }
            CalcNode::Mod { dividend, divisor } => {
                let dividend = dividend.resolve(percentage_length);
                let divisor = divisor.resolve(percentage_length);

                if divisor.is_infinite() && dividend.is_negative() != divisor.is_negative() {
                    return f32::NAN;
                }
                let modulo = dividend - divisor * (dividend / divisor).floor();
                if modulo == 0.0 && dividend.is_negative() {
                    -0.0
                } else {
                    modulo
                }
            }
            CalcNode::Hypot(nodes) => {
                nodes.iter().map(|node| node.resolve(percentage_length)).map(|len| len.powi(2)).sum::<f32>().sqrt()
            }
            CalcNode::Sign(value) => value.resolve(percentage_length).signum(),
            CalcNode::Abs(value) => value.resolve(percentage_length).abs(),
        }
    }
}
#[cfg(feature = "calc")]
impl From<LengthPercentage> for CalcNode {
    fn from(value: LengthPercentage) -> Self {
        CalcNode::Leaf(value)
    }
}

/// Rounding strategy used by an [CSS round()](https://www.w3.org/TR/css-values-4/#funcdef-round)
/// <https://www.w3.org/TR/css-values-4/#typedef-rounding-strategy>
#[cfg(feature = "calc")]
#[derive(Debug, Copy, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum RoundingStrategy {
    /// Round `value` up to the nearest integer multiple of `interval`
    /// (if the value is negative, it will become "more positive").
    Up,
    /// Round `value` down to the nearest integer multiple of `interval`
    /// (if the value is negative, it will become "more negative").
    Down,
    /// Round `value` to the nearest integer multiple of `interval`,
    /// which may be either above or below the value.
    /// If the `value` is halfway between the rounding targets above and below (neither is "nearest"),
    /// it will be rounded up.
    #[default]
    Nearest,
    /// Round `value` to the nearest integer multiple of `interval` closer to/towards zero
    /// (a positive number will decrease, while a negative value will become "less negative").
    ToZero,
}

/// The amount of space available to a node in a given axis
/// <https://www.w3.org/TR/css-sizing-3/#available>
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum AvailableSpace {
    /// The amount of space available is the specified number of pixels
    Definite(f32),
    /// The amount of space available is indefinite and the node should be laid out under a min-content constraint
    MinContent,
    /// The amount of space available is indefinite and the node should be laid out under a max-content constraint
    MaxContent,
}
impl TaffyZero for AvailableSpace {
    const ZERO: Self = Self::Definite(0.0);
}
impl TaffyMaxContent for AvailableSpace {
    const MAX_CONTENT: Self = Self::MaxContent;
}
impl TaffyMinContent for AvailableSpace {
    const MIN_CONTENT: Self = Self::MinContent;
}
impl FromLength for AvailableSpace {
    fn from_length<Input: Into<f32> + Copy>(value: Input) -> Self {
        Self::Definite(value.into())
    }
}

impl AvailableSpace {
    /// Returns true for definite values, else false
    pub fn is_definite(self) -> bool {
        matches!(self, AvailableSpace::Definite(_))
    }

    /// Convert to Option
    /// Definite values become Some(value). Constraints become None.
    pub fn into_option(self) -> Option<f32> {
        match self {
            AvailableSpace::Definite(value) => Some(value),
            _ => None,
        }
    }

    /// Return the definite value or a default value
    pub fn unwrap_or(self, default: f32) -> f32 {
        self.into_option().unwrap_or(default)
    }

    /// Return the definite value. Panic is the value is not definite.
    #[track_caller]
    pub fn unwrap(self) -> f32 {
        self.into_option().unwrap()
    }

    /// Return self if definite or a default value
    pub fn or(self, default: AvailableSpace) -> AvailableSpace {
        match self {
            AvailableSpace::Definite(_) => self,
            _ => default,
        }
    }

    /// Return self if definite or a the result of the default value callback
    pub fn or_else(self, default_cb: impl FnOnce() -> AvailableSpace) -> AvailableSpace {
        match self {
            AvailableSpace::Definite(_) => self,
            _ => default_cb(),
        }
    }

    /// Return the definite value or the result of the default value callback
    pub fn unwrap_or_else(self, default_cb: impl FnOnce() -> f32) -> f32 {
        self.into_option().unwrap_or_else(default_cb)
    }

    /// If passed value is Some then return AvailableSpace::Definite containing that value, else return self
    pub fn maybe_set(self, value: Option<f32>) -> AvailableSpace {
        match value {
            Some(value) => AvailableSpace::Definite(value),
            None => self,
        }
    }

    /// If passed value is Some then return AvailableSpace::Definite containing that value, else return self
    pub fn map_definite_value(self, map_function: impl FnOnce(f32) -> f32) -> AvailableSpace {
        match self {
            AvailableSpace::Definite(value) => AvailableSpace::Definite(map_function(value)),
            _ => self,
        }
    }

    /// Compute free_space given the passed used_space
    pub fn compute_free_space(&self, used_space: f32) -> f32 {
        match self {
            AvailableSpace::MaxContent => f32::INFINITY,
            AvailableSpace::MinContent => 0.0,
            AvailableSpace::Definite(available_space) => available_space - used_space,
        }
    }

    /// Compare equality with another AvailableSpace, treating definite values
    /// that are within f32::EPSILON of each other as equal
    pub fn is_roughly_equal(self, other: AvailableSpace) -> bool {
        use AvailableSpace::*;
        match (self, other) {
            (Definite(a), Definite(b)) => abs(a - b) < f32::EPSILON,
            (MinContent, MinContent) => true,
            (MaxContent, MaxContent) => true,
            _ => false,
        }
    }
}

impl From<f32> for AvailableSpace {
    fn from(value: f32) -> Self {
        Self::Definite(value)
    }
}

impl From<Option<f32>> for AvailableSpace {
    fn from(option: Option<f32>) -> Self {
        match option {
            Some(value) => Self::Definite(value),
            None => Self::MaxContent,
        }
    }
}

impl Size<AvailableSpace> {
    /// Convert `Size<AvailableSpace>` into `Size<Option<f32>>`
    pub fn into_options(self) -> Size<Option<f32>> {
        Size { width: self.width.into_option(), height: self.height.into_option() }
    }

    /// If passed value is Some then return AvailableSpace::Definite containing that value, else return self
    pub fn maybe_set(self, value: Size<Option<f32>>) -> Size<AvailableSpace> {
        Size { width: self.width.maybe_set(value.width), height: self.height.maybe_set(value.height) }
    }
}
