use std::collections::HashMap;
use std::hash::Hash;

use crate::crdt::traits::Join;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct GCounter<Id: Eq + Hash + Copy> {
    total: u64,
    counts: HashMap<Id, u64>,
}

impl<Id: Eq + Hash + Copy> Default for GCounter<Id> {
    fn default() -> Self {
        Self {
            counts: HashMap::new(),
            total: 0,
        }
    }
}

impl<Id: Eq + Hash + Copy> GCounter<Id> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn inc_by(&mut self, replica: Id, delta: u64) {
        let entry = self.counts.entry(replica).or_insert(0);
        *entry = entry.saturating_add(delta);
        self.total = self.total.saturating_add(delta);
    }

    pub fn inc(&mut self, replica: Id) {
        self.inc_by(replica, 1)
    }

    pub fn value(&self) -> u64 {
        self.total
    }
}

impl<Id: Eq + Hash + Copy> Join for GCounter<Id> {
    fn join(&self, other: &Self) -> Self {
        let mut out = self.clone();
        out.merge(other);
        out
    }

    fn merge(&mut self, other: &Self) {
        for (&rep, &value) in other.counts.iter() {
            self.counts
                .entry(rep)
                .and_modify(|v| {
                    if *v < value {
                        self.total = self.total.saturating_add(value - *v);
                        *v = value;
                    }
                })
                .or_insert_with(|| {
                    self.total = self.total.saturating_add(value);
                    value
                });
        }
    }
}
