use rand::Rng;
use crate::params::{ERROR_BOUND, Q};

pub fn sample_small_error() -> u64 {
    let mut rng = rand::rng();
    rng.random_range(0..=ERROR_BOUND)
}

pub fn sample_uniform_q() -> u64 {
    let mut rng = rand::rng();
    rng.random_range(0..Q)
}

pub fn sample_uniform_vec_q<const LEN: usize>() -> [u64; LEN] {
    let mut rng = rand::rng();
    let mut vec = [0u64; LEN];
    for i in 0..LEN {
        vec[i] = rng.random_range(0..Q);
    }
    vec
}
