pub trait Join: Clone + Eq {
    // Must be associative, commutative, and idempotent.
    fn join(&self, other: &Self) -> Self;

    fn merge(&mut self, other: &Self) {
        *self = self.join(other);
    }
}
