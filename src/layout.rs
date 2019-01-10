use crate::geometry::{Point, Size};

#[derive(Debug, Clone)]
pub struct Node {
    pub(crate) order: u32,
    pub size: Size<f32>,
    pub location: Point<f32>,
    pub children: Vec<Node>,
}
