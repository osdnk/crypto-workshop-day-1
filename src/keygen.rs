use crate::arithmetic::*;
use crate::params::*;
use crate::sampling::*;

pub fn keygen() -> (VecN, (MatMxN, VecM)) {
    let s = sample_uniform_vec_q::<N>();
    let mut a = [[0u64; N]; M];
    let mut b = [0u64; M];

    for i in 0..M {
        let ai = sample_uniform_vec_q::<N>();
        a[i] = ai;

        let dot = dot_mod_q(&ai, &s);
        let e = sample_small_error();
        b[i] = (dot + e) % Q;
    }
    (s, (a, b))
}
