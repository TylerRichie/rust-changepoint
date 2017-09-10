use algo::best_candidate::BestCandidate;
use algo::changepoint::{ChangePointDetector, ChangePointDetectorBuilder};
use rand::Rng;

use errors::*;

#[derive(Clone, Debug)]
pub struct PermutationTestResult {
    pub p_value: f64,
    pub changepoint_index: usize
}

fn permutation_test<'a, T, B, R>(algorithm: B, rng: R, num_permutations: usize, observations: &'a [T]) -> Result<PermutationTestResult>
    where
    T: Ord + Clone,
    B: ChangePointDetectorBuilder<'a, T>,
    R: Rng,
{
    let true_detector = algorithm.detect_changepoint_on(observations)?;
    let BestCandidate { statistic: true_statistic, location: true_location } = true_detector.find_candidate();
    let mut permutations: Vec<_> = Vec::new();
    for _ in 0..num_permutations {
        let mut inner_vec: Vec<T> = Vec::with_capacity(observations.len());
        inner_vec.clone_from_slice(observations);
        rng.shuffle(&mut inner_vec);
        let detector = algorithm.detect_changepoint_on(&inner_vec)?;
        permutations.push(detector);
    };
    let p_value = permutations.into_iter().map(|permutation| {
        let BestCandidate { statistic, .. } = permutation.find_candidate();
        if &statistic <= &true_statistic {
            1.0
        } else {
            0.0
        }
    }).sum() / ((num_permutations + 1) as f64);
    Ok(PermutationTestResult {
        p_value: p_value,
        changepoint_index: true_location
    })
}
