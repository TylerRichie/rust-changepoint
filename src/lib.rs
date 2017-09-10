#[macro_use] extern crate error_chain;
extern crate rayon;
extern crate mersenne_twister;
extern crate rand;
extern crate num;

pub mod errors;
mod algo;

pub use algo::edm_x::edm_x::EDMX;
pub use algo::changepoint::ChangePointDetector;
pub use algo::non_nan::{NonNaN, to_non_nans};
pub use algo::permutation_test::{permutation_test, PermutationTestResult};
