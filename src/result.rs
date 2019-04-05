#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

use core::any::Any;

use crate::algo::ComputeResult;
use crate::geometry::{Point, Size};
use crate::number::Number;

pub type Result<T> = core::result::Result<T, Box<Any>>;

#[derive(Debug, Clone)]
pub struct Layout {
    pub(crate) order: u32,
    pub size: Size<f32>,
    pub location: Point<f32>,
    pub children: Vec<Layout>,
}

#[derive(Debug, Clone)]
pub(crate) struct Cache {
    pub(crate) node_size: Size<Number>,
    pub(crate) parent_size: Size<Number>,
    pub(crate) perform_layout: bool,

    pub(crate) result: ComputeResult,
}
