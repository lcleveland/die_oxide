use crate::number_generator::traits;
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub struct PseudoRandom {
    pub upper_limit: i32,
}

impl PseudoRandom {
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
