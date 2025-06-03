use crate::params::*;
pub fn dot_mod_q(v1: &[u64], v2: &[u64]) -> u64 {
    v1.iter().zip(v2.iter()).map(|(&x, &y)| (x * y) % Q).sum::<u64>() % Q
}

pub fn mat_vec_mul_mod_q(mat: &MatMxN, vec: &VecN) -> VecM {
    let mut result = [0u64; M];
    for i in 0..M {
        result[i] = dot_mod_q(&mat[i], vec);
    }
    result
}

pub fn vec_mat_mul_mod_q(vec: &[u64; M], mat: &MatMxN) -> VecN {
    let mut result = [0u64; N];
    for j in 0..N {
        for i in 0..M {
            result[j] = (result[j] + vec[i] * mat[i][j]) % Q;
        }
    }
    result
}

pub fn vec_dot_mod_q(vec1: &[u64; M], vec2: &[u64; M]) -> u64 {
    vec1.iter().zip(vec2.iter()).map(|(&x, &y)| (x * y) % Q).sum::<u64>() % Q
}
