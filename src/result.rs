use core::any::Any;

pub type Result<T> = core::result::Result<T, Box<Any>>;
