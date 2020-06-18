pub mod domains;
pub mod polynomials;
pub mod fft;
pub mod cs;
pub mod multicore;
pub mod utils;
pub mod transparent_engine;

pub extern crate pairing;
pub use pairing::*;

use crate::pairing::ff as ff;
pub use ff::*;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
