use std::fmt::{Debug, Display};

use crate::crdt::Actor;

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct Dot<A: Actor> {
    pub actor: A,
    pub counter: u64,
}

impl<A: Actor> Dot<A> {
    pub fn new(actor: A, counter: u64) -> Self {
        Self { actor, counter }
    }
}

impl<A: Actor + Debug> Debug for Dot<A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Dot({:?}, {})", self.actor, self.counter)
    }
}

impl<A: Actor + Display> Display for Dot<A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}@{}", self.actor, self.counter)
    }
}
