#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

use core::any::Any;

use crate::geometry::{Point, Size};
use crate::number::Number;
use crate::algo::ComputeResult;

pub type Result<T> = core::result::Result<T, Box<Any>>;

#[derive(Debug, Clone)]
pub struct Layout {
    pub(crate) order: u32,
    pub size: Size<f32>,
    pub location: Point<f32>,
    pub children: Vec<Layout>,
}

#[derive(Debug, Clone)]
pub struct Cache {
    pub node_size: Size<Number>,
    pub parent_size: Size<Number>,
    pub perform_layout: bool,

    pub result: ComputeResult,
}
