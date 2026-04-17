use std::collections::HashMap;

use crate::crdt::Actor;
use crate::crdt::traits::Join;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GCounter<A: Actor> {
    total: u64,
    counts: HashMap<A, u64>,
}

impl<A: Actor> Default for GCounter<A> {
    fn default() -> Self {
        Self {
            total: 0,
            counts: HashMap::new(),
        }
    }
}

impl<A: Actor> GCounter<A> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn inc_by(&mut self, actor: A, delta: u64) {
        let entry = self.counts.entry(actor).or_insert(0);
        *entry = entry.saturating_add(delta);
        self.total = self.total.saturating_add(delta);
    }

    pub fn inc(&mut self, actor: A) {
        self.inc_by(actor, 1)
    }

    pub fn value(&self) -> u64 {
        self.total
    }
}

impl<A: Actor> Join for GCounter<A> {
    fn merge(&mut self, other: &Self) {
        for (&actor, &value) in &other.counts {
            let entry = self.counts.entry(actor).or_insert(0);
            if value > *entry {
                self.total = self.total.saturating_add(value - *entry);
                *entry = value;
            }
        }
    }
}
