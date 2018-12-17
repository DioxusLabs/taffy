use crate::geometry::{Point, Size};

#[derive(Debug)]
pub struct Node {
    pub size: Size<f32>,
    pub location: Point<f32>,
    pub children: Vec<Node>,
}
