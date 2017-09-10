use algo::best_candidate::BestCandidate;
use algo::changepoint::ChangePointDetector;
use rand::Rng;
use rayon;

use errors::*;

#[derive(Clone, Debug)]
pub struct PermutationTestResult {
    pub p_value: f64,
    pub changepoint_index: usize,
}

fn run_algorithm_on_permutation<'a, T, B>(
    detector: &B,
    true_statistic: &T,
    permutation: &[T],
) -> Result<f64>
where
    T: Ord + Clone,
    B: ChangePointDetector<T>,
{
    let BestCandidate { statistic, .. } = detector.find_candidate(permutation)?;
    if &statistic <= true_statistic {
        Ok(0.0)
    } else {
        Ok(1.0)
    }
}

struct PermutationIteration<T: Ord + Clone> {
    permutation: Vec<T>,
    greater_than_truth: Option<Result<f64>>,
}

fn do_permutation_iteration<T, B>(
    algorithm: &B,
    true_statistic: &T,
    permutation_iterations: &mut [PermutationIteration<T>],
) -> ()
where
    T: Ord + Clone + Send + Sync,
    B: ChangePointDetector<T> + Send + Sync,
{
    if permutation_iterations.len() <= 1 {
        for permutation_iteration in permutation_iterations {
            permutation_iteration.greater_than_truth = Some(run_algorithm_on_permutation(
                algorithm,
                true_statistic,
                &permutation_iteration.permutation,
            ));
        }
    } else {
        let slice_point: usize = permutation_iterations.len() / 2;
        let (left, right) = permutation_iterations.split_at_mut(slice_point);
        rayon::join(
            || do_permutation_iteration(algorithm, true_statistic, left),
            || do_permutation_iteration(algorithm, true_statistic, right),
        );
    }
}

pub fn permutation_test<'a, T, B, R>(
    algorithm: &B,
    mut rng: R,
    num_permutations: usize,
    observations: &'a [T],
) -> Result<PermutationTestResult>
where
    T: Ord + Clone + Send + Sync,
    B: ChangePointDetector<T> + Send + Sync,
    R: Rng,
{
    let BestCandidate {
        statistic: true_statistic,
        location: true_location,
    } = algorithm.find_candidate(observations)?;
    let mut permutations: Vec<PermutationIteration<T>> = Vec::new();
    for _ in 0..num_permutations {
        let mut inner_vec = observations.to_vec();
        rng.shuffle(&mut inner_vec);
        let permutation_iteration = PermutationIteration {
            permutation: inner_vec,
            greater_than_truth: None,
        };
        permutations.push(permutation_iteration);
    }
    do_permutation_iteration(algorithm, &true_statistic, &mut permutations);
    let num_failures: Result<f64> = permutations.into_iter().fold(Ok(0.0), |num_failures,
     permutation| {
        Ok(
            num_failures? +
                match permutation.greater_than_truth {
                    Some(result) => result?,
                    None => return Err(ErrorKind::PermutationNeverRan.into()),
                },
        )
    });
    let p_value = num_failures? / ((num_permutations + 1) as f64);
    Ok(PermutationTestResult {
        p_value: p_value,
        changepoint_index: true_location,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use mersenne_twister::MersenneTwister;
    use rand::SeedableRng;
    use rand::distributions::{IndependentSample, Normal};
    use algo::non_nan::NonNaN;
    use algo::edm_x::edm_x::EDMX;

    const NUM_PERMUTATIONS: usize = 10;
    // The paper recommends 199, but that takes way too long unless you compile with the
    // `--release` flag enabled. If you want to see how fast this algorithm is, enable `--release`
    // and set this constant to 199.

    #[test]
    fn edm_x_permutation_test_detects_if_change_occurred() {
        let mut rng: MersenneTwister = SeedableRng::from_seed(0x1234);
        let before_change_dist = Normal::new(10.0, 5.0);
        let after_change_dist = Normal::new(20.0, 5.0);
        let num_before_observations = 500;
        let num_after_observations = 200;
        let delta = 30;
        let num_permutations = NUM_PERMUTATIONS;
        let mut inputs: Vec<NonNaN<f64>> = Vec::new();
        for i in 0..(num_before_observations + num_after_observations) {
            let dist = if i < num_before_observations {
                before_change_dist
            } else {
                after_change_dist
            };
            inputs.push(NonNaN::new(dist.ind_sample(&mut rng)).unwrap());
        }
        let algorithm = EDMX::new(delta);
        let full_test = permutation_test(&algorithm, rng, num_permutations, &inputs).unwrap();
        assert!(full_test.p_value <= 0.1);
    }

    #[test]
    fn edm_x_permutation_test_detects_no_change_occurred() {
        let mut rng: MersenneTwister = SeedableRng::from_seed(0x1234);
        let dist = Normal::new(10.0, 5.0);
        let num_observations = 700;
        let delta = 30;
        let num_permutations = NUM_PERMUTATIONS;
        let mut inputs: Vec<NonNaN<f64>> = Vec::new();
        for _ in 0..num_observations {
            inputs.push(NonNaN::new(dist.ind_sample(&mut rng)).unwrap());
        }
        let algorithm = EDMX::new(delta);
        let full_test = permutation_test(&algorithm, rng, num_permutations, &inputs).unwrap();
        assert!(full_test.p_value > 0.1);
    }
}
