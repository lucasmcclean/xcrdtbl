use std::fmt::{Debug, Display};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Dot<A> {
    pub actor: A,
    pub counter: u64,
}

impl<A> Dot<A> {
    pub fn new(actor: A, counter: u64) -> Self {
        Self { actor, counter }
    }
}

impl<A: Debug> Debug for Dot<A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Dot({:?}, {})", self.actor, self.counter)
    }
}

impl<A: Display> Display for Dot<A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}@{}", self.actor, self.counter)
    }
}
