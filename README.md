# Regev Encryption (Toy u64 Implementation)

This project provides a minimal one-file implementation of the Regev LWE-based public-key encryption scheme using toy parameters and `u64` arithmetic. It's designed for educational use only.

## Prerequisites

* [Rust toolchain](https://rustup.rs/) (recommended: stable)
* `cargo` package manager (comes with Rust)

## Running the Program

The default binary runs encryption and decryption for values in `{0, 1, 2, 3}` and prints the results.

```sh
cargo run
```

## Running Tests

You can add tests to the source file using `#[test]` functions. Then run:

```sh
cargo test
```

## Running Benchmarks

For simple timing benchmarks already included just run:

```sh
cargo bench
```

## TODO (Student Extensions)

Refer to the top of the source file `main.rs` for a detailed list of 9 open-ended TODOs, each suitable for deeper exploration or a project deliverable, such as:

* Replace toy parameters with secure ones
* Use ring arithmetic
* Use proper modular arithmetic libraries
* Add discrete Gaussian noise
* Implement prover/verifier communication
* Benchmark timings rigorously
* Generalise message support
* Compare with dual-Regev
* Abstract data structures
* Write unit and property-based tests

