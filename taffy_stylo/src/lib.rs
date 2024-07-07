//! Conversion functions from Stylo types to Taffy types

pub(crate) mod arc;
pub(crate) mod borrowed;
pub(crate) mod t2s;

// Module of type aliases so we can refer to stylo types with nicer names
mod stylo {
    pub(crate) use style::computed_values::flex_direction::T as FlexDirection;
    pub(crate) use style::computed_values::flex_wrap::T as FlexWrap;
    pub(crate) use style::properties::generated::longhands::box_sizing::computed_value::T as BoxSizing;
    pub(crate) use style::properties::longhands::aspect_ratio::computed_value::T as AspectRatio;
    pub(crate) use style::properties::longhands::position::computed_value::T as Position;
    pub(crate) use style::values::specified::align::AlignFlags;
    pub(crate) use style::values::specified::align::ContentDistribution;
    // pub(crate) use style::properties::style_structs::{Margin, Padding};
    pub(crate) use style::values::computed::LengthPercentage;
    pub(crate) use style::values::generics::flex::GenericFlexBasis;
    pub(crate) use style::values::generics::length::GenericLengthPercentageOrAuto;
    pub(crate) use style::values::generics::length::GenericLengthPercentageOrNormal;
    pub(crate) use style::values::generics::length::GenericMaxSize;
    pub(crate) use style::values::generics::length::GenericSize;
    pub(crate) use style::values::generics::position::PreferredRatio;
    pub(crate) use style::values::generics::NonNegative;
    pub(crate) use style::values::specified::box_::Display;
    pub(crate) use style::values::specified::box_::DisplayInside;
    pub(crate) use style::values::specified::box_::DisplayOutside;
    pub(crate) use style::values::specified::box_::Overflow;
    pub(crate) type LengthPercentageAuto = GenericLengthPercentageOrAuto<LengthPercentage>;
    pub(crate) type Size = GenericSize<NonNegative<LengthPercentage>>;
    pub(crate) type MaxSize = GenericMaxSize<NonNegative<LengthPercentage>>;
    pub(crate) type FlexBasis = GenericFlexBasis<Size>;
    pub(crate) type Gap = GenericLengthPercentageOrNormal<NonNegative<LengthPercentage>>;
    pub(crate) use style::properties::generated::ComputedValues;
}

pub use arc::TaffyStyloStyle;
pub use borrowed::TaffyStyloStyleRef;
