use crate::arithmetic::*;
use crate::params::*;
use crate::sampling::sample_small_error;

pub fn encrypt(pk: &(MatMxN, VecM), value: u8) -> (VecN, u64) {
    assert!(value < 4);
    let (a, b) = pk;

    let mut r = [0u64; M];
    for i in 0..M {
        r[i] = sample_small_error();
    }

    let u = vec_mat_mul_mod_q(&r, a);
    let mut v = vec_dot_mod_q(&r, b);
    v = (v + (Q / 4) * (value as u64)) % Q;

    (u, v)
}
