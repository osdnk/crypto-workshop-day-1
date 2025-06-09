# 🛠️ Roadmap and Refactoring Tasks for Lattice Crypto Prototype

This document outlines key improvements and refactors to transition the current prototype into a secure, modular, and efficient cryptographic implementation.

---

## ✅ 1. Replace Toy Parameters with Secure Lattice Estimates
**Description:**  
Upgrade all hardcoded toy parameters (e.g., `N`, `Q`, `M`) with values obtained from a trusted lattice security estimator.

**Goal:**  
Ensure cryptographic security appropriate for the desired level (e.g., 128-bit).

---

## ✅ 2. Migrate from Stack to Heap for Large Data Structures
**Description:**  
Avoid stack overflow issues by replacing `[u64; SIZE]` arrays with `Vec<u64>` or setting `ulimit -s unlimited`.

**Note:**  
Stack size adjustments are not scalable; prefer heap allocations for large buffers.

---

## ✅ 3. Integrate Ring Arithmetic for Efficient Polynomial Operations
**Description:**  
Use modular polynomial arithmetic over rings like ℤ_q[X]/(Xⁿ+1) to enable fast crypto operations.

**Tools:**  
Consider HEXL, `lattirust`, hand-crafted NTT, or Karatsuba multiplication.

---

## ✅ 4. Use Cryptographically-Secure Modular Arithmetic
**Description:**  
Replace standard `% Q` operations with optimized modular arithmetic from crypto libraries.

**Options:**
- [`ff`](https://docs.rs/ff)
- [`arkworks`](https://github.com/arkworks-rs)

---

## ✅ 5. Enable Discrete Gaussian Sampling for Noise Generation
**Description:**  
Replace uniform noise with discrete Gaussian sampling to conform to LWE security requirements.

---

## ✅ 6. Refactor into Separate Prover and Verifier Binaries
**Description:**  
Split the binary into two roles: prover and verifier. Use serialization for communication.

**Suggested Tools:**
- `serde`
- `bincode`
- `protobuf`

---

## ✅ 7. Generalise Plaintext Support and Bit Packing
**Description:**  
Extend plaintext domain from `{0,1,2,3}` to broader sets (e.g., full `u8`) and enable efficient bit-packing.

---

## ✅ 8. Evaluate Dual-Regev Encryption Scheme
**Description:**  
Implement a dual-Regev variant and benchmark against standard Regev to explore trade-offs in key size and performance.

---

## ✅ 9. Add Type-Safe Abstractions for Crypto Primitives
**Description:**  
Use Rust `structs` and `traits` to define clear and type-safe APIs for ciphertexts, keys, and parameters.

---

## ✅ 10. Add Unit and Property-Based Testing for Core Components
**Description:**  
Implement a comprehensive test suite.

**Types of Tests:**
- Unit tests for core logic
- Property-based tests using `proptest` or similar

---