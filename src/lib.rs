#[macro_use] extern crate error_chain;
extern crate mersenne_twister;
extern crate rand;
extern crate num;

pub mod errors;
mod algo;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
