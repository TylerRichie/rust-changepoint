use algo::best_candidate::BestCandidate;

use errors::*;

pub trait ChangePointDetector<T: Ord> {
    fn find_candidate(&self, observations: &[T]) -> Result<BestCandidate<T>>;
}
