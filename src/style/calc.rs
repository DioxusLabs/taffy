//! `calc()` values for [`TaffyTree`](crate::TaffyTree)

#[cfg(feature = "taffy_tree")]
use crate::style::CalcHandle;
#[cfg(feature = "taffy_tree")]
use crate::util::sys::Vec;

/// A `calc()` value: `percent * basis + length`.
///
/// Every `calc()` of lengths and percentages using `+`, `-`, and `*` or `/` by a number reduces to this.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LinearCalc {
    /// The percentage, as a fraction (`0.5` is `50%`)
    pub percent: f32,
    /// The length
    pub length: f32,
}

impl LinearCalc {
    /// A length
    pub const fn length(length: f32) -> Self {
        Self { percent: 0.0, length }
    }

    /// A percentage, as a fraction (`0.5` is `50%`)
    pub const fn percent(percent: f32) -> Self {
        Self { percent, length: 0.0 }
    }

    /// The value with the percentage resolved against `basis`
    pub fn resolve(self, basis: f32) -> f32 {
        self.percent * basis + self.length
    }
}

impl core::ops::Add for LinearCalc {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self { percent: self.percent + rhs.percent, length: self.length + rhs.length }
    }
}

impl core::ops::Sub for LinearCalc {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self { percent: self.percent - rhs.percent, length: self.length - rhs.length }
    }
}

impl core::ops::Mul<f32> for LinearCalc {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self {
        Self { percent: self.percent * rhs, length: self.length * rhs }
    }
}

impl core::ops::Div<f32> for LinearCalc {
    type Output = Self;
    fn div(self, rhs: f32) -> Self {
        Self { percent: self.percent / rhs, length: self.length / rhs }
    }
}

#[cfg(feature = "taffy_tree")]
/// The generation bits a handle keeps (it has 61: 32 for the slot index and 28 for the generation)
const GENERATION_MASK: u32 = (1 << 28) - 1;

#[cfg(feature = "taffy_tree")]
/// One slot of a [`CalcStore`]
#[derive(Clone, Debug)]
struct CalcSlot {
    /// Bumped when the slot is emptied, so old handles stop matching
    generation: u32,
    /// The value, if the slot holds one
    calc: Option<LinearCalc>,
}

#[cfg(feature = "taffy_tree")]
/// The `calc()` values of a [`TaffyTree`](crate::TaffyTree)
#[derive(Clone, Debug, Default)]
pub(crate) struct CalcStore {
    /// The slots
    slots: Vec<CalcSlot>,
    /// Empty slots, to be reused
    free: Vec<u32>,
}

#[cfg(feature = "taffy_tree")]
impl CalcStore {
    /// Stores a value and returns its handle
    pub(crate) fn insert(&mut self, calc: LinearCalc) -> CalcHandle {
        let index = match self.free.pop() {
            Some(index) => index,
            None => {
                self.slots.push(CalcSlot { generation: 1, calc: None });
                (self.slots.len() - 1) as u32
            }
        };
        let slot = &mut self.slots[index as usize];
        slot.calc = Some(calc);
        // Past the 3 tag bits; the generation is never 0, so neither is the handle
        let key = (u64::from(slot.generation) << 32) | u64::from(index);
        CalcHandle::from_bits(key << 3)
    }

    /// Removes a value and returns it
    pub(crate) fn remove(&mut self, handle: CalcHandle) -> Option<LinearCalc> {
        let index = self.index_of(handle)?;
        let slot = &mut self.slots[index];
        let calc = slot.calc.take()?;
        slot.generation = (slot.generation % GENERATION_MASK) + 1;
        self.free.push(index as u32);
        Some(calc)
    }

    /// Resolves a handle against `basis` (0 if the store doesn't hold it)
    pub(crate) fn resolve(&self, handle: CalcHandle, basis: f32) -> f32 {
        self.index_of(handle).and_then(|index| self.slots[index].calc).map_or(0.0, |calc| calc.resolve(basis))
    }

    /// Empties the store
    pub(crate) fn clear(&mut self) {
        self.slots.clear();
        self.free.clear();
    }

    /// The slot a handle refers to, if it still holds the handle's value
    fn index_of(&self, handle: CalcHandle) -> Option<usize> {
        let key = handle.to_bits() >> 3;
        let index = (key & u64::from(u32::MAX)) as usize;
        let generation = (key >> 32) as u32;
        self.slots.get(index).filter(|slot| slot.generation == generation && slot.calc.is_some()).map(|_| index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arithmetic() {
        let (pc, px) = (LinearCalc::percent, LinearCalc::length);
        // calc((10% + 5px) * 3 / 2 - 1px)
        assert_eq!(((pc(0.1) + px(5.0)) * 3.0 / 2.0 - px(1.0)).resolve(400.0), 66.5);
    }

    #[cfg(feature = "taffy_tree")]
    #[test]
    fn removed_handles_stop_resolving() {
        let mut store = CalcStore::default();
        let a = store.insert(LinearCalc::percent(0.5));
        assert_eq!(store.resolve(a, 100.0), 50.0);
        assert_eq!(store.remove(a), Some(LinearCalc::percent(0.5)));
        assert_eq!(store.remove(a), None);
        assert_eq!(store.resolve(a, 100.0), 0.0);
        let b = store.insert(LinearCalc::length(7.0));
        assert_ne!(a, b);
        assert_eq!(store.resolve(a, 100.0), 0.0);
        assert_eq!(store.resolve(b, 100.0), 7.0);
        assert_eq!(b.to_bits() & 0b111, 0);
        store.clear();
        assert_eq!(store.resolve(b, 100.0), 0.0);
    }
}
