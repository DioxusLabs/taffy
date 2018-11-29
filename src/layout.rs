#[derive(Debug)]
pub struct Edges {
  pub left: f32,
  pub right: f32,
  pub top: f32,
  pub bottom: f32,
}

#[derive(Debug)]
pub struct Node {
  pub width: f32,
  pub height: f32,
  pub x: f32,
  pub y: f32,
  pub children: Vec<Node>,
}