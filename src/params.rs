pub const N: usize = 600; // Dimension of the secret
pub const Q: u64 = 32741;  // Modulus
pub const M: usize = 1000; // Number of A matrix rows (more than N)
pub const ERROR_BOUND: u64 = 3; // Small error

pub type VecN = [u64; N];
pub type VecM = [u64; M];
pub type MatMxN = [[u64; N]; M];