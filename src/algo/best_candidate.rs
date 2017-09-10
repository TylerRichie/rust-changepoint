use std::cmp::Ordering;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BestCandidate<T: Ord> {
    pub statistic: T,
    pub location: usize,
}

impl<T: Ord> PartialOrd for BestCandidate<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(match self.statistic.cmp(&other.statistic) {
            Ordering::Equal => self.location.cmp(&other.location).reverse(),
            ordering => ordering,
        })
    }
}

impl<T: Ord> Ord for BestCandidate<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).expect(
            "BestCandidate is totally ordered, so partial_ord always returns Some",
        )
    }
}
