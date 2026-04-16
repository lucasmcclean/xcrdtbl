use std::hash::Hash;

pub trait Actor: Clone + Copy + Eq + Hash + Ord {}
impl<T: Clone + Copy + Eq + Hash + Ord> Actor for T {}
