pub trait Join: Clone + Eq {
    // Must be associative, commutative, and idempotent.
    fn merge(&mut self, other: &Self);

    fn join(&self, other: &Self) -> Self {
        let mut out = self.clone();
        out.merge(other);
        out
    }
}
