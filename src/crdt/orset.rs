use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

use crate::crdt::{Actor, Dot, traits::Join};

type Seen<A> = HashSet<Dot<A>>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ORSet<E: Eq + Hash, A: Actor> {
    adds: HashMap<E, HashSet<Dot<A>>>,
    removes: HashMap<E, HashSet<Dot<A>>>,
}

impl<E: Eq + Hash, A: Actor> ORSet<E, A> {
    pub fn add(&mut self, element: E, actor: A, counter: u64) {
        let dot = Dot::new(actor, counter);
        self.adds.entry(element).or_default().insert(dot);
    }

    pub fn observe(&self, element: &E) -> Seen<A> {
        self.adds.get(element).cloned().unwrap_or_default()
    }

    pub fn remove(&mut self, element: E, seen: &Seen<A>) {
        self.removes.entry(element).or_default().extend(seen);
    }

    pub fn has(&self, element: &E) -> bool {
        let adds = match self.adds.get(element) {
            Some(a) => a,
            None => return false,
        };

        let removes = self.removes.get(element);

        adds.iter().any(|d| removes.is_none_or(|r| !r.contains(d)))
    }
}

impl<E: Eq + Hash + Clone, A: Actor> Join for ORSet<E, A> {
    fn merge(&mut self, other: &Self) {
        for (element, dots) in &other.adds {
            self.adds
                .entry(element.clone())
                .or_default()
                .extend(dots.iter().cloned());
        }
        for (element, seen) in &other.removes {
            self.removes
                .entry(element.clone())
                .or_default()
                .extend(seen.iter().cloned());
        }
    }
}
