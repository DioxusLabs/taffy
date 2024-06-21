use std::ops::Neg;
use std::sync::Arc;
use crate::LengthPercentage;

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Calc(Arc<CalcNode>);
impl Calc {
    pub fn resolve(&self, percentage_length: f32) -> f32 {
        self.0.resolve(percentage_length)
    }
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum CalcNode {
    Leaf(LengthPercentage),

    Sum(Box<CalcNode>, Box<CalcNode>),
    Difference(Box<CalcNode>, Box<CalcNode>),
    Product(Box<CalcNode>, Box<CalcNode>),
    Quotient(Box<CalcNode>, Box<CalcNode>),

    Negate(Box<CalcNode>),
    Min(Vec<CalcNode>),
    Max(Vec<CalcNode>),

    Clamp { min: Box<CalcNode>, center: Box<CalcNode>, max: Box<CalcNode> },
}
impl CalcNode {
    fn resolve(&self, percentage_length: f32) -> f32 {
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
            CalcNode::Clamp { min, center, max } => {
                let min = min.resolve(percentage_length);
                let center = center.resolve(percentage_length);
                let max = max.resolve(percentage_length);

                let max = center.max(max);
                let min = min.min(max);
                min
            }
        }
    }

    fn into_calc(self) -> Calc {
        Calc(Arc::new(self))
    }
}
