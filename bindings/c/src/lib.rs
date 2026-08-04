mod error;
mod style;
mod style_enums;
mod tree;
mod value;

pub struct TaffyStyle;
pub type TaffyStyleMutRef = *mut TaffyStyle;
pub type TaffyStyleConstRef = *const TaffyStyle;

impl TaffyFFIDefault for TaffyStyleMutRef {
    fn default() -> Self {
        core::ptr::null_mut()
    }
}
impl TaffyFFIDefault for TaffyStyleConstRef {
    fn default() -> Self {
        core::ptr::null()
    }
}

pub use error::*;
pub use style::*;
pub use style_enums::*;
pub use tree::*;
pub use value::*;
