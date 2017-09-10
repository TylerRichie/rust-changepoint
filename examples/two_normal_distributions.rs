extern crate changepoint;
extern crate rand;
extern crate mersenne_twister;

use changepoint::{EDMX, NonNaN, permutation_test};
use rand::SeedableRng;
use rand::distributions::{Normal, IndependentSample};
use mersenne_twister::MersenneTwister;

const START_DISTRIBUTION_MEAN: f64 = 10.0;
const START_DISTRIBUTION_STD: f64 = 5.0;

const END_DISTRIBUTION_MEAN: f64 = 20.0;
const END_DISTRIBUTION_STD: f64 = 5.0;

const NUM_START_OBSERVATIONS: usize = 500;
const NUM_END_OBSERVATIONS: usize = 200;

const DELTA: usize = 30;
const NUM_PERMUTATIONS: usize = 199;

fn main() {
    println!("");
    println!("**Detect a Changepoint from observations drawn from two normal distributions**");
    println!("");
    let mut rng: MersenneTwister = SeedableRng::from_seed(0x1234);
    let before_change_dist = Normal::new(START_DISTRIBUTION_MEAN, START_DISTRIBUTION_STD);
    let after_change_dist = Normal::new(END_DISTRIBUTION_MEAN, END_DISTRIBUTION_STD);
    let num_before_observations = NUM_START_OBSERVATIONS;
    let num_after_observations = NUM_END_OBSERVATIONS;
    println!("Drawing {} samples from a normal distribution with mean {:.1} and standard deviation {:.1}",
             NUM_START_OBSERVATIONS,
             START_DISTRIBUTION_MEAN,
             START_DISTRIBUTION_STD,
    );
    println!("Drawing {} samples from a normal distribution with mean {:.1} and standard deviation {:.1}",
             NUM_END_OBSERVATIONS,
             END_DISTRIBUTION_MEAN,
             END_DISTRIBUTION_STD,
    );
    let mut inputs: Vec<NonNaN<f64>> = Vec::new();
    for i in 0..(num_before_observations + num_after_observations) {
        let dist = if i < num_before_observations {
            before_change_dist
        } else {
            after_change_dist
        };
        inputs.push(NonNaN::new(dist.ind_sample(&mut rng)).unwrap());
    }
    println!("Initialized EDM-X algorithm with delta as {}", DELTA);
    let algorithm = EDMX::new(DELTA);
    println!(
        "Performing a permutation test with {} iterations",
        NUM_PERMUTATIONS
    );
    let full_test = permutation_test(&algorithm, rng, NUM_PERMUTATIONS, &inputs).unwrap();
    println!("");
    println!("Candidate split location: {}", full_test.changepoint_index);
    println!("P-Value: {:.5}", full_test.p_value);
}
