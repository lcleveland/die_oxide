use crate::number_generator::traits;
use rand::Rng;

struct PseudoRandom {
    upper_limit: i32,
}

impl traits::NumberGenerator for PseudoRandom {
    fn generate(&self) -> i32 {
        rand::thread_rng().gen_range(1..=self.upper_limit)
    }
}
