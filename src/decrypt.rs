use crate::arithmetic::*;
use crate::params::*;

pub fn decrypt(sk: &VecN, ct: &(VecN, u64)) -> u8 {
    let (u, v) = ct;
    let dot = dot_mod_q(u, sk);
    let diff = (v + Q - dot) % Q;

    ((4 * diff + Q / 2) / Q).min(3) as u8
}