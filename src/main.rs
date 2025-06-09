use regev::*;

// ===== TODOs for deployment-ready Regev system =====
// These are improvements we can make to turn this toy implementation
// into a secure, usable cryptographic system based on LWE (Regev's encryption).
// Each task is self-contained and can be worked on independently.
//
// 1. Replace toy parameters (N, Q, M, etc.) with secure values returned by a lattice estimator.
//   -> observe that probably you get "stack overflow" as we allocate everything on stack. 
//    Use heap instead (Vec<u64> instead of [u64; SIZE]) or set "ulimit -s unlimited" (does not scale for long).
// 2. Switch to ring arithmetic (e.g., use polynomial rings Z_q[X]/(X^n+1)) to enable faster operations (using HEXL, lattirust or hand-crafted NTT or Karatsuba).
// 3. Replace u64 arithmetic with % Q with better modular arithmetic from a cryptographic library (e.g., ff or arkworks).
// 4. Use discrete Gaussian sampling for error terms (instead of uniform noise).
// 5. Separate prover and verifier into two programs using serialization (e.g., serde + bincode or protobuf).
// 6. Generalise plaintext: currently we encrypt values in {0,1,2,3}; define an abstraction that supports larger messages (e.g., full u8) and pack bits efficiently.
// 7. Implement and compare with dual-Regev encryption to understand trade-offs in key size and performance.
// 8. Introduce type-safe abstractions for ciphertexts, keys, and parameters using structs or traits.
// 9. Add comprehensive test suite, including unit tests and property-based tests.
fn main() {
    let (s, pk) = keygen::keygen();

    // for value in 0..4 {
    //     let ct = encrypt::encrypt(&pk, value);
    //     let recovered = decrypt::decrypt(&s, &ct);
    //     println!("Original: {} | Decrypted: {}", value, recovered);
    // }
}

#[cfg(test)]
mod tests {
    use crate::{decrypt, encrypt, keygen};

    #[test]
    fn encrypted_and_decrypted_values_are_the_same() {
        let (s, pk) = keygen::keygen();
        for value in 0..4 {
            let ct = encrypt::encrypt(&pk, value);
            let recovered = decrypt::decrypt(&s, &ct);
            assert_eq!(value, recovered);
        }
    }
}

