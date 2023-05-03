//! Pseudo-random number generator.

use crate::number_generator::traits;
use rand::Rng;
use serde::{Deserialize, Serialize};

///  A number generator that returns a pseudo-random number using the rand crate.
///  This number is between 1 and the upper_limit.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub struct PseudoRandom {
    /// The highest number that is allowed to be generated.
    pub upper_limit: i32,
}

impl PseudoRandom {
    /// Creates a new instance of the PseudoRandom struct.
    pub fn new(upper_limit: i32) -> Self {
        Self { upper_limit }
    }
}

impl Default for PseudoRandom {
    fn default() -> Self {
        Self { upper_limit: 20 }
    }
}

impl traits::NumberGenerator for PseudoRandom {
    fn generate(&self) -> i32 {
        rand::thread_rng().gen_range(1..=self.upper_limit)
    }
}
