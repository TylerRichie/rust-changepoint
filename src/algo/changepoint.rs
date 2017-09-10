use algo::best_candidate::BestCandidate;

use errors::*;

pub trait ChangePointDetector {
    type Candidate: Ord;
    fn find_candidate(self) -> BestCandidate<Self::Candidate>;
}

pub trait ChangePointDetectorBuilder<'a, T: Ord> {
    type Detector: ChangePointDetector<Candidate=T> + 'a;
    fn detect_changepoint_on(&self, observations: &'a [T]) -> Result<Self::Detector>;
}
