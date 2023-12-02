// changes must not break day7

use std::hash::{Hash, Hasher};
use std::sync::atomic::AtomicUsize;

#[derive(Debug, Clone, Copy, Eq)]
pub struct NodeId {
    idx: usize,
    arena: usize,
}

impl Hash for NodeId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.idx.hash(state);
        self.arena.hash(state);
    }
}

impl PartialEq for NodeId {
    fn eq(&self, other: &Self) -> bool {
        self.idx == other.idx && self.arena == other.arena
    }
}

pub struct Iter<'a, T> {
    arena_id: usize,
    items: &'a Vec<T>,
    idx: usize,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = (NodeId, &'a T);
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(item) = self.items.get(self.idx) {
            let item = Some((
                NodeId {
                    idx: self.idx,
                    arena: self.arena_id,
                },
                item,
            ));
            self.idx += 1;
            item
        } else {
            None
        }
    }
}

pub struct Arena<T> {
    items: Vec<T>,
    id: usize,
}

impl<T> Default for Arena<T> {
    fn default() -> Self {
        Self {
            items: vec![],
            id: Arena::<T>::new_arena_id(),
        }
    }
}

impl<T> Arena<T> {
    fn new_arena_id() -> usize {
        static CURRENT_ID: AtomicUsize = AtomicUsize::new(0);
        CURRENT_ID.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
    }

    pub fn new() -> Self {
        Default::default()
    }

    pub fn add_node(&mut self, val: T) -> NodeId {
        self.items.push(val);
        NodeId {
            idx: self.items.len() - 1,
            arena: self.id,
        }
    }

    pub fn get_node(&self, id: NodeId) -> Option<&T> {
        if id.arena != self.id {
            None
        } else {
            self.items.get(id.idx)
        }
    }

    pub fn get_node_mut(&mut self, id: NodeId) -> Option<&mut T> {
        if id.arena != self.id {
            None
        } else {
            self.items.get_mut(id.idx)
        }
    }

    pub fn iter(&self) -> impl Iterator + '_ {
        Iter {
            arena_id: self.id,
            items: &self.items,
            idx: 0,
        }
    }

    pub fn next_id(&self) -> NodeId {
        NodeId {
            idx: self.items.len(),
            arena: self.id,
        }
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<T> IntoIterator for Arena<T> {
    type IntoIter = std::vec::IntoIter<T>;
    type Item = T;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}

impl<T> std::ops::Index<NodeId> for Arena<T> {
    type Output = T;

    fn index(&self, index: NodeId) -> &Self::Output {
        assert_eq!(self.id, index.arena);
        &self.items[index.idx]
    }
}

impl<T> std::ops::IndexMut<NodeId> for Arena<T> {
    fn index_mut(&mut self, index: NodeId) -> &mut T {
        assert_eq!(self.id, index.arena);
        &mut self.items[index.idx]
    }
}
