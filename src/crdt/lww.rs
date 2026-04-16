use crate::crdt::{Actor, traits::Join};
use chrono::{DateTime, Utc};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Lww<T: Clone + Eq, A: Actor> {
    value: T,
    timestamp: DateTime<Utc>,
    actor: A,
}

impl<T: Clone + Eq, A: Actor> Lww<T, A> {
    pub fn new(value: T, timestamp: DateTime<Utc>, actor: A) -> Self {
        Self {
            value,
            timestamp,
            actor,
        }
    }

    pub fn value(&self) -> &T {
        &self.value
    }

    pub fn timestamp(&self) -> DateTime<Utc> {
        self.timestamp
    }
}

impl<T: Clone + Eq, A: Actor> Join for Lww<T, A> {
    fn merge(&mut self, other: &Self) {
        let should_replace = match other.timestamp.cmp(&self.timestamp) {
            std::cmp::Ordering::Greater => true,
            std::cmp::Ordering::Less => false,
            std::cmp::Ordering::Equal => other.actor > self.actor,
        };

        if should_replace {
            *self = other.clone();
        }
    }
}
