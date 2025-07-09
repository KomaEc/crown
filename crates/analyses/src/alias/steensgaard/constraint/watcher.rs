use rustc_index::IndexVec;

use crate::alias::steensgaard::AbstractLocation;

#[derive(Clone, Copy)]
pub struct Watcher {
    next: u32,
    constraint: u32,
}

impl Watcher {
    #[inline]
    pub fn new_first(constraint: usize) -> Self {
        Watcher::new(0, constraint)
    }

    #[inline]
    pub fn new(next: usize, constraint: usize) -> Self {
        Watcher {
            next: next as u32,
            constraint: constraint as u32,
        }
    }

    #[inline]
    pub fn next(&self) -> usize {
        self.next as usize
    }

    #[inline]
    pub fn constraint(&self) -> usize {
        self.constraint as usize
    }
}

pub struct WatcherLists {
    nodes: Vec<Watcher>,
    /// start index of an abstract location
    head: IndexVec<AbstractLocation, usize>,
}

impl WatcherLists {
    pub fn new(n_locs: usize) -> Self {
        WatcherLists {
            nodes: vec![Watcher::new_first(0)],
            head: IndexVec::from_elem_n(0, n_locs),
        }
    }

    pub fn get_list(&self, loc: AbstractLocation) -> WatcherList<'_> {
        WatcherList {
            lists: self,
            this: loc,
        }
    }

    /// Add a new watch location for constraint
    #[inline]
    pub fn add_watch(&mut self, constraint_idx: usize, loc: AbstractLocation) {
        let next = std::mem::replace(&mut self.head[loc], self.nodes.len());
        let watch = Watcher::new(next, constraint_idx);
        self.nodes.push(watch);
    }
}

pub struct WatcherList<'me> {
    lists: &'me WatcherLists,
    this: AbstractLocation,
}

impl<'me> WatcherList<'me> {
    pub fn iter(&self) -> WatcherListIter<'_> {
        WatcherListIter {
            watcher_lists: self.lists,
            node_idx: self.lists.head[self.this],
        }
    }
}

pub struct WatcherListIter<'me> {
    watcher_lists: &'me WatcherLists,
    // loc: AbstractLocation,
    node_idx: usize,
}

impl<'me> Iterator for WatcherListIter<'me> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.node_idx > 0 {
            let cur_watcher = self.watcher_lists.nodes[self.node_idx];
            self.node_idx = cur_watcher.next();
            Some(cur_watcher.constraint())
        } else {
            None
        }
    }
}
