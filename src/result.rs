use core::fmt::Debug;

pub type Result<T> = core::result::Result<T, Box<Debug>>;
