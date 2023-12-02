// changes must not break day7

use crate::arena::{Arena, NodeId};
use std::{slice, vec::IntoIter};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TreeError {
    #[error("added a node without parent but root was already set")]
    RootAlreadyExists,
    #[error("specified a parent but root doesn't exist")]
    NoRoot,
}

pub struct IterChildren<'a, T> {
    tree: &'a Tree<T>,
    iter: slice::Iter<'a, NodeId>,
}

impl<'a, T> Iterator for IterChildren<'a, T> {
    type Item = (NodeId, &'a T);
    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(&id) => Some((id, self.tree.get(id))),
            None => None,
        }
    }
}

pub struct IterDescendants<'a, T> {
    tree: &'a Tree<T>,
    stack: Vec<IterChildren<'a, T>>,
    current: IterChildren<'a, T>,
}

impl<'a, T> Iterator for IterDescendants<'a, T> {
    type Item = (NodeId, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        match self.current.next() {
            Some(item) => {
                let node_children = self.tree.children_internal(item.0);
                self.stack.push(node_children);
                Some(item)
            }
            None => match self.stack.pop() {
                Some(iter) => {
                    self.current = iter;
                    self.next()
                }
                None => None,
            },
        }
    }
}

pub struct IterAncestors<'a, T> {
    tree: &'a Tree<T>,
    current: NodeId,
}

impl<'a, T> Iterator for IterAncestors<'a, T> {
    type Item = (NodeId, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(parent) = self.tree.parent(self.current) {
            let val = self.tree.get(parent);
            let item = (parent, val);
            self.current = parent;
            Some(item)
        } else {
            None
        }
    }
}

struct TreeNode<T> {
    parent: Option<NodeId>,
    children: Vec<NodeId>,
    value: T,
}

impl<T> TreeNode<T> {
    fn new(val: T) -> Self {
        Self {
            parent: None,
            children: vec![],
            value: val,
        }
    }
}

pub struct IterTree<T> {
    iter: IntoIter<TreeNode<T>>,
}

impl<T> Iterator for IterTree<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(node) => Some(node.value),
            None => None,
        }
    }
}

pub struct Tree<T> {
    arena: Arena<TreeNode<T>>,
    root: Option<NodeId>,
}

impl<T> Default for Tree<T> {
    fn default() -> Self {
        Self {
            arena: Default::default(),
            root: None,
        }
    }
}

impl<T> Tree<T> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add_node(&mut self, val: T, parent: Option<NodeId>) -> Result<NodeId, TreeError> {
        if self.root.is_some() && parent.is_none() {
            return Err(TreeError::RootAlreadyExists);
        }
        if parent.is_some() && self.root.is_none() {
            return Err(TreeError::NoRoot);
        }
        let next_id = self.arena.next_id();
        let mut new_node = TreeNode::new(val);
        if let (Some(_), Some(parent)) = (self.root, parent) {
            new_node.parent = Some(parent);
            let parent_node = self.arena.get_node_mut(parent).unwrap();
            parent_node.children.push(next_id);
        } else {
            self.root = Some(next_id)
        }
        Ok(self.arena.add_node(new_node))
    }

    pub fn get(&self, node_id: NodeId) -> &T {
        let node = self.arena.get_node(node_id).unwrap();
        &node.value
    }

    pub fn get_mut(&mut self, node_id: NodeId) -> &mut T {
        let node = self.arena.get_node_mut(node_id).unwrap();
        &mut node.value
    }

    pub fn root(&self) -> Option<NodeId> {
        self.root
    }

    pub fn children(&self, node_id: NodeId) -> impl Iterator<Item = (NodeId, &T)> + '_ {
        self.children_internal(node_id)
    }

    fn children_internal(&self, node_id: NodeId) -> IterChildren<T> {
        let node = self.arena.get_node(node_id).unwrap();
        let iter = node.children.iter();
        IterChildren { iter, tree: self }
    }

    pub fn children_ids(&self, node_id: NodeId) -> impl Iterator<Item = &NodeId> + '_ {
        self.arena.get_node(node_id).unwrap().children.iter()
    }

    pub fn descendants(&self, node_id: NodeId) -> impl Iterator<Item = (NodeId, &T)> {
        IterDescendants {
            tree: self,
            stack: vec![],
            current: self.children_internal(node_id),
        }
    }

    pub fn parent(&self, node_id: NodeId) -> Option<NodeId> {
        self.arena.get_node(node_id).and_then(|node| node.parent)
    }

    pub fn ancestors(&self, node_id: NodeId) -> impl Iterator<Item = (NodeId, &T)> {
        IterAncestors {
            tree: self,
            current: node_id,
        }
    }

    pub fn mutate_ancestors<F>(&mut self, node_id: NodeId, mut f: F)
    where
        F: FnMut(NodeId, &mut T),
    {
        let mut current = node_id;
        while let Some(parent) = self.parent(current) {
            let node = self.arena.get_node_mut(parent).unwrap();
            let val_ref = &mut node.value;
            f(parent, val_ref);
            current = parent;
        }
    }

    pub fn mutate_children<F>(&mut self, node_id: NodeId, mut f: F)
    where
        F: FnMut(NodeId, &mut T),
    {
        let children: Vec<NodeId> = self.children_ids(node_id).copied().collect();
        for id in children {
            let node = self.arena.get_node_mut(id).unwrap();
            f(id, &mut node.value)
        }
    }

    pub fn mutate_descendants<F>(&mut self, node_id: NodeId, mut f: F)
    where
        F: FnMut(NodeId, &mut T),
    {
        let mut stack: Vec<NodeId> = self.arena.get_node(node_id).unwrap().children.to_vec();
        while let Some(id) = stack.pop() {
            let node = self.arena.get_node_mut(id).unwrap();
            f(id, &mut node.value);
            stack.extend(node.children.iter())
        }
    }

    pub fn len(&self) -> usize {
        self.arena.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<T> IntoIterator for Tree<T> {
    type IntoIter = IterTree<T>;
    type Item = T;
    fn into_iter(self) -> Self::IntoIter {
        IterTree {
            iter: self.arena.into_iter(),
        }
    }
}
