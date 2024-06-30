//! Style types for representing lengths / sizes

use core::fmt::{Debug, Formatter};
use core::ops::Neg;
use std::ptr::NonNull;

use num_traits::{Signed, Zero};

use crate::geometry::{Rect, Size};
use crate::style_helpers::{FromLength, FromPercent, TaffyAuto, TaffyMaxContent, TaffyMinContent, TaffyZero};
use crate::sys::{Box, Vec};
use crate::util::sys::abs;

macro_rules! impl_debug {
    ($ty:ident, $name:literal) => {
        impl Debug for $ty {
            fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
                unsafe {
                    f.debug_tuple($name)
                        .field(match self.is_calc() {
                            false => &self.inner,
                            true => self.ptr.ptr.as_ref(),
                        })
                        .finish()
                }
            }
        }
    };
}

macro_rules! impl_measurement {
    ($ty:ident, $inner_ty:ident) => {
        impl $ty {
            #[inline(always)]
            pub const fn get_inner(&self) -> $inner_ty {
                unsafe { self.inner }
            }
            #[inline]
            pub const fn is_percent(&self) -> bool {
                matches!(self.get_inner(), $inner_ty::Percent(_))
            }
            #[inline]
            pub const fn is_length(&self) -> bool {
                matches!(self.get_inner(), $inner_ty::Length(_))
            }
            
            #[inline]
            pub const fn length(length: f32) -> Self {
                Self { inner: $inner_ty::Length(length) }
            }
            #[inline]
            pub const fn percent(percent: f32) -> Self {
                Self { inner: $inner_ty::Percent(percent) }
            }
            #[cfg(feature = "calc")]
            #[inline]
            pub fn calc(calc: CalcNode) -> Self {
                Self::static_calc(Box::leak(Box::new(calc)))
            }
            #[cfg(feature = "calc")]
            #[inline]
            pub const fn static_calc(calc: &'static CalcNode) -> Self {
                let ptr = std::ptr::from_ref(calc) as *mut CalcNode;
                unsafe { Self { ptr: CalcPtr::new(NonNull::new_unchecked(ptr)) }.set_calc_tag() } 
            }
            #[cfg(feature = "calc")]
            #[inline]
            const fn calc_from_calc_ptr(calc_ptr: CalcPtr) -> Self {
                unsafe { Self { ptr: calc_ptr } }
            }
            #[cfg(feature = "calc")]
            #[inline]
            const unsafe fn set_calc_tag(mut self) -> Self {
                #[cfg(target_pointer_width = "64")]
                { self.bits = self.bits << 8 };
                
                self.tag.0 = 0;
                self
            }
            #[cfg(feature = "calc")]
            #[inline]
            pub const fn is_calc(&self) -> bool {
                matches!(self.get_inner(), $inner_ty::Calc)
            }
            #[cfg(feature = "calc")]
            #[inline]
            pub fn get_calc(&self) -> Option<&CalcNode> {
                if self.is_calc() {
                    Some( unsafe { &*self.get_calc_ptr() } )
                } else {
                    None
                }
            }
            #[cfg(feature = "calc")]
            #[inline]
            pub const unsafe fn get_calc_ptr(&self) -> *mut CalcNode {
                #[cfg(target_pointer_width = "64")]
                return (self.bits >> 8) as *mut CalcNode;
                #[cfg(target_pointer_width = "32")]
                return self.ptr.ptr.as_ptr();
            }
            #[cfg(feature = "calc")]
            #[inline]
            const unsafe fn get_calc_ptr_data(self) -> CalcPtr {
                self.ptr
            }
        }
        impl PartialEq for $ty {
            #[inline]
            fn eq(&self, other: &Self) -> bool {
                #[cfg(not(feature = "calc"))]
                {
                    self.get_inner() == other.get_inner()
                }
                #[cfg(feature = "calc")]
                if self.get_inner() == other.get_inner() && !self.is_calc() {
                    true
                } else {
                    self.get_calc() == other.get_calc()
                }
            }
        }
    };
    ($ty:ident, $inner_ty:ident, auto) => {
        impl_measurement!($ty, $inner_ty);
        impl $ty {
            /// Returns true if value is LengthPercentageAuto::Auto
            #[inline]
            pub const fn is_auto(&self) -> bool {
                matches!(self.get_inner(), $inner_ty::Auto)
            }
            #[inline]
            pub const fn auto() -> Self {
                Self { inner: $inner_ty::Auto }
            }
        }
    };
}
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
#[repr(align(8), C)]
struct Tag(u8);

#[cfg(feature = "calc")]
#[derive(Copy, Clone)]
#[repr(transparent)]
struct CalcPtr {
    #[cfg(target_pointer_width = "32")]
    _space: u32,
    ptr: NonNull<CalcNode> 
}
#[cfg(feature = "calc")]
impl CalcPtr {
    #[inline]
    const fn new(ptr: NonNull<CalcNode>) -> Self {
        #[cfg(target_pointer_width = "64")]
        return Self { ptr };
        #[cfg(target_pointer_width = "32")]
        return Self { ptr, _space: 0 };
    }
}


/// A unit of linear measurement
///
/// This is commonly combined with [`Rect`], [`Point`](crate::geometry::Point) and [`Size<T>`].
#[derive(Copy, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(u8, C, align(8))]
pub enum LengthPercentageInner {
    #[cfg(feature = "calc")]
    /// todo
    Calc = 0,
    /// An absolute length in some abstract units. Users of Taffy may define what they correspond
    /// to in their application (pixels, logical pixels, mm, etc) as they see fit.
    Length(f32),
    /// The dimension is stored in percentage relative to the parent item.
    Percent(f32),
}

#[derive(Copy, Clone)]
pub union LengthPercentage {
    inner: LengthPercentageInner,
    #[cfg(feature = "calc")]
    ptr: CalcPtr,

    tag: Tag,
    bits: u64,
}
impl_debug!(LengthPercentage, "LengthPercentage");
impl_measurement!(LengthPercentage, LengthPercentageInner);
impl LengthPercentage {
    pub fn resolve(&self, percentage_length: f32) -> f32 {
        match self.get_inner() {
            LengthPercentageInner::Length(length) => length,
            LengthPercentageInner::Percent(fraction) => fraction * percentage_length,
            #[cfg(feature = "calc")]
            LengthPercentageInner::Calc => unsafe { self.get_calc_ptr().read().resolve(percentage_length) },
        }
    }
}
impl TaffyZero for LengthPercentage {
    const ZERO: Self = Self::length(0.0);
}
impl FromLength for LengthPercentage {
    fn from_length<Input: Into<f32> + Copy>(value: Input) -> Self {
        Self::length(value.into())
    }
}
impl FromPercent for LengthPercentage {
    fn from_percent<Input: Into<f32> + Copy>(percent: Input) -> Self {
        Self::percent(percent.into())
    }
}

/// A unit of linear measurement
///
/// This is commonly combined with [`Rect`], [`Point`](crate::geometry::Point) and [`Size<T>`].
#[derive(Copy, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(u8, C, align(8))]
pub enum LengthPercentageAutoInner {
    #[cfg(feature = "calc")]
    /// todo
    Calc = 0,
    /// An absolute length in some abstract units. Users of Taffy may define what they correspond
    /// to in their application (pixels, logical pixels, mm, etc) as they see fit.
    Length(f32),
    /// The dimension is stored in percentage relative to the parent item.
    Percent(f32),
    /// The dimension should be automatically computed
    Auto,
}

#[derive(Copy, Clone)]
pub union LengthPercentageAuto {
    inner: LengthPercentageAutoInner,
    #[cfg(feature = "calc")]
    ptr: CalcPtr,

    tag: Tag,
    bits: u64,
}
impl_debug!(LengthPercentageAuto, "LengthPercentageAuto");
impl_measurement!(LengthPercentageAuto, LengthPercentageAutoInner, auto);

impl TaffyZero for LengthPercentageAuto {
    const ZERO: Self = Self::length(0.0);
}
impl TaffyAuto for LengthPercentageAuto {
    const AUTO: Self = Self::auto();
}
impl FromLength for LengthPercentageAuto {
    fn from_length<Input: Into<f32> + Copy>(value: Input) -> Self {
        Self::length(value.into())
    }
}
impl FromPercent for LengthPercentageAuto {
    fn from_percent<Input: Into<f32> + Copy>(percent: Input) -> Self {
        Self::percent(percent.into())
    }
}

impl From<LengthPercentage> for LengthPercentageAuto {
    fn from(input: LengthPercentage) -> Self {
        match input.get_inner() {
            LengthPercentageInner::Length(value) => Self::length(value),
            LengthPercentageInner::Percent(value) => Self::percent(value),
            #[cfg(feature = "calc")]
            LengthPercentageInner::Calc => Self::calc_from_calc_ptr(unsafe { input.get_calc_ptr_data() }),
        }
    }
}

impl LengthPercentageAuto {
    /// Returns:
    ///   - Some(length) for Length variants
    ///   - Some(resolved) using the provided context for Percent variants
    ///   - Some(calculation) todo
    ///   - None for Auto variants
    #[inline(always)]
    pub fn resolve_to_option(self, context: f32) -> Option<f32> {
        match self.get_inner() {
            LengthPercentageAutoInner::Length(length) => Some(length),
            LengthPercentageAutoInner::Percent(percent) => Some(context * percent),
            LengthPercentageAutoInner::Calc => self.get_calc().map(|calc| calc.resolve(context)),
            LengthPercentageAutoInner::Auto => None,
        }
    }
}

/// A unit of linear measurement
///
/// This is commonly combined with [`Rect`], [`Point`](crate::geometry::Point) and [`Size<T>`].
//#[derive(Clone, PartialEq, Debug)]
//#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
//pub enum Dimension {
//    /// An absolute length in some abstract units. Users of Taffy may define what they correspond
//    /// to in their application (pixels, logical pixels, mm, etc) as they see fit.
//    Length(f32),
//    /// The dimension is stored in percentage relative to the parent item.
//    Percent(f32),
//    /// todo
//    Calc(Calc),
//    /// The dimension should be automatically computed
//    Auto,
//}

#[derive(Copy, Clone)]
pub union Dimension {
    inner: DimensionInner,
    #[cfg(feature = "calc")]
    ptr: CalcPtr,

    tag: Tag,
    bits: u64,
}
impl_debug!(Dimension, "Dimension");
impl_measurement!(Dimension, DimensionInner, auto);
impl Dimension {
    const fn layout() {
        const {
            assert!(core::mem::size_of::<Dimension>() == 8);
            assert!(core::mem::size_of::<DimensionInner>() == 8);
            #[cfg(feature = "calc")]
            assert!(core::mem::size_of::<CalcPtr>() == 8);
        }
    }
}
#[derive(Copy, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(u8, C, align(8))]
pub enum DimensionInner {
    #[cfg(feature = "calc")]
    /// todo
    Calc = 0,
    /// An absolute length in some abstract units. Users of Taffy may define what they correspond
    /// to in their application (pixels, logical pixels, mm, etc) as they see fit.
    Length(f32),
    /// The dimension is stored in percentage relative to the parent item.
    Percent(f32),
    /// The dimension should be automatically computed
    Auto,
}
impl TaffyZero for Dimension {
    const ZERO: Self = Self::length(0.0);
}
impl TaffyAuto for Dimension {
    const AUTO: Self = Self::auto();
}
impl FromLength for Dimension {
    fn from_length<Input: Into<f32> + Copy>(value: Input) -> Self {
        Self::length(value.into())
    }
}
impl FromPercent for Dimension {
    fn from_percent<Input: Into<f32> + Copy>(percent: Input) -> Self {
        Self::percent(percent.into())
    }
}

impl From<LengthPercentage> for Dimension {
    fn from(input: LengthPercentage) -> Self {
        match input.get_inner() {
            LengthPercentageInner::Length(value) => Self::length(value),
            LengthPercentageInner::Percent(value) => Self::percent(value),
            #[cfg(feature = "calc")]
            LengthPercentageInner::Calc => Self::calc_from_calc_ptr(unsafe { input.get_calc_ptr_data() }),
        }
    }
}

impl From<LengthPercentageAuto> for Dimension {
    fn from(input: LengthPercentageAuto) -> Self {
        match input.get_inner() {
            LengthPercentageAutoInner::Length(value) => Self::length(value),
            LengthPercentageAutoInner::Percent(value) => Self::percent(value),
            #[cfg(feature = "calc")]
            LengthPercentageAutoInner::Calc => Self::calc_from_calc_ptr(unsafe { input.get_calc_ptr_data() }),
            LengthPercentageAutoInner::Auto => Self::auto(),
        }
    }
}

impl Dimension {
    /// Get Length value if value is Length variant
    #[cfg(feature = "grid")]
    pub fn into_option(self) -> Option<f32> {
        match self.get_inner() {
            DimensionInner::Length(value) => Some(value),
            _ => None,
        }
    }
}

impl Rect<Dimension> {
    /// Create a new Rect with [`Dimension::Length`]
    #[must_use]
    pub const fn from_length(start: f32, end: f32, top: f32, bottom: f32) -> Self {
        Rect {
            left: Dimension::length(start),
            right: Dimension::length(end),
            top: Dimension::length(top),
            bottom: Dimension::length(bottom),
        }
    }

    /// Create a new Rect with [`Dimension::Percent`]
    #[must_use]
    pub const fn from_percent(start: f32, end: f32, top: f32, bottom: f32) -> Self {
        Rect {
            left: Dimension::percent(start),
            right: Dimension::percent(end),
            top: Dimension::percent(top),
            bottom: Dimension::percent(bottom),
        }
    }
}

#[cfg(feature = "calc")]
#[derive(Debug, PartialEq)]
//#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum CalcNode {
    Leaf(LengthPercentage),

    Sum(Box<CalcNode>, Box<CalcNode>),
    Difference(Box<CalcNode>, Box<CalcNode>),
    Product(Box<CalcNode>, Box<CalcNode>),
    Quotient(Box<CalcNode>, Box<CalcNode>),

    Negate(Box<CalcNode>),
    Min(Vec<CalcNode>),
    Max(Vec<CalcNode>),
    Clamp { min: Box<CalcNode>, value: Box<CalcNode>, max: Box<CalcNode> },
    Round { strategy: RoundingStrategy, value: Box<CalcNode>, interval: Box<CalcNode> },
    Rem { dividend: Box<CalcNode>, divisor: Box<CalcNode> },
    Mod { dividend: Box<CalcNode>, divisor: Box<CalcNode> },
    Hypot(Vec<CalcNode>),
    Sign(Box<CalcNode>),
    Abs(Box<CalcNode>),
}
#[cfg(feature = "calc")]
impl CalcNode {
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
                let min = min.resolve(percentage_length);
                let value = value.resolve(percentage_length);
                let max = max.resolve(percentage_length);

                if min.is_nan() || value.is_nan() || max.is_nan() {
                    return f32::NAN;
                }

                let max = value.max(max);
                let min = min.min(max);
                min
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
#[cfg(feature = "calc")]
#[derive(Debug, Copy, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum RoundingStrategy {
    Up,
    Down,
    #[default]
    Nearest,
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
