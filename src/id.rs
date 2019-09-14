//! Identifier for a Node
//!
//!
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;
use core::sync::atomic;

/// Internal node id.
pub(crate) type NodeId = usize;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) struct Id {
    id: u32,
    generation: u32,
}

pub(crate) struct Allocator {
    new_id: atomic::AtomicU32,
    free_ids: spin::RwLock<IdCache>,
}

impl Allocator {
    pub fn new() -> Self {
        Allocator { new_id: atomic::AtomicU32::new(0), free_ids: spin::RwLock::new(IdCache::default()) }
    }

    pub fn allocate(&self) -> Id {
        self.free_ids
            .read()
            .pop_atomic()
            .map(|Id { id, generation }| Id { id, generation: generation + 1 })
            .unwrap_or_else(|| Id {
                id: atomic_increment(&self.new_id).expect("No entity left to allocate"),
                generation: 0,
            })
    }

    pub fn free(&self, ids: &[Id]) {
        self.free_ids.write().extend(ids.iter().copied());
    }
}

// Code below this line is based on code from the `specs` library, in the `src/world/entity.rs`
// file, which has the following copyright notice (MIT license):

/*
Copyright (c) 2017 The Specs Project Developers

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
DEALINGS IN THE SOFTWARE.
*/

#[derive(Default, Debug)]
struct IdCache {
    cache: Vec<Id>,
    len: atomic::AtomicUsize,
}

impl IdCache {
    fn pop_atomic(&self) -> Option<Id> {
        atomic_decrement(&self.len).map(|x| self.cache[x - 1])
    }

    fn maintain(&mut self) {
        self.cache.truncate(*(self.len.get_mut()));
    }
}

impl Extend<Id> for IdCache {
    fn extend<T: IntoIterator<Item = Id>>(&mut self, iter: T) {
        self.maintain();
        self.cache.extend(iter);
        *self.len.get_mut() = self.cache.len();
    }
}

/// Increments `i` atomically without wrapping on overflow.
/// Resembles a `fetch_add(1, Ordering::Relaxed)` with
/// checked overflow, returning `None` instead.
fn atomic_increment(i: &atomic::AtomicU32) -> Option<u32> {
    let mut prev = i.load(atomic::Ordering::Relaxed);
    while prev != core::u32::MAX {
        match i.compare_exchange_weak(prev, prev + 1, atomic::Ordering::Relaxed, atomic::Ordering::Relaxed) {
            Ok(x) => return Some(x),
            Err(next_prev) => prev = next_prev,
        }
    }
    None
}

/// Decrements `i` atomically without wrapping on overflow.
/// Resembles a `fetch_sub(1, Ordering::Relaxed)` with
/// checked underflow, returning `None` instead.
fn atomic_decrement(i: &atomic::AtomicUsize) -> Option<usize> {
    let mut prev = i.load(atomic::Ordering::Relaxed);
    while prev != 0 {
        match i.compare_exchange_weak(prev, prev - 1, atomic::Ordering::Relaxed, atomic::Ordering::Relaxed) {
            Ok(x) => return Some(x),
            Err(next_prev) => prev = next_prev,
        }
    }
    None
}
