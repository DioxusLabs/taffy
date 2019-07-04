//! Identifier for a Node
//!
//!

/// Internal node id.
pub(crate) type NodeId = usize;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) struct Id {
    id: u32,
    generation: u32,
}

pub(crate) struct Allocator {
    new_id: u32,
    free_ids: Vec<Id>,
}

impl Allocator {
    pub fn new() -> Self {
        Allocator { new_id: 0, free_ids: Vec::new() }
    }

    pub fn allocate(&mut self) -> Id {
        // TODO: better balancing
        match self.free_ids.pop() {
            Some(id) => Id { id: id.id, generation: id.generation + 1 },
            None => {
                let id = self.new_id;
                self.new_id += 1;
                Id { id, generation: 0 }
            }
        }
    }

    pub fn free(&mut self, ids: &[Id]) {
        self.free_ids.extend(ids);
    }
}
