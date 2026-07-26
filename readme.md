# finite_field_lib

An educational finite field arithmetic library written in Rust. This crate serves as a foundational component for understanding the algebraic primitives required in zero-knowledge proof (ZKP) systems.

## Mathematical Scope
This library implements prime field arithmetic with strict memory and algebraic boundary enforcement.
* **Canonical State Enforcement:** Utilizes `num-bigint` to guarantee that all elements are reduced modulo the prime upon instantiation, preventing canonical representation vulnerabilities.
* **The Multiverse Defense:** Overloads standard operations (`Add`, `Sub`, `Mul`, `Div`) with strict boundary checks. The system will inherently panic if operations are attempted across different prime moduli, preventing field pollution.
* **Memory Optimization:** Leverages in-place mutation and temporary lifetime extension to minimize heap allocations and avoid garbage collector thrashing.

## Algorithmic Choices
* Addition and Subtraction execute in O(N) software long division and utilize CPU branch prediction to skip redundant boundary checks.
* Division is implemented using Fermat's Little Theorem. It utilizes the `num-bigint` Windowed Exponentiation engine to calculate the modular inverse in O(log P) time complexity.

## Security Posture
**WARNING: This is a variable-time implementation.**
This crate is strictly designed for educational and architectural understanding of finite fields. Because it relies on dynamic `BigUint` heap allocations and short-circuiting logic, it is highly vulnerable to Timing Side-Channel Attacks. It is not cryptographically safe. Do not use this in a production zero-knowledge prover or any cryptographic protocol. Production systems must utilize constant-time, stack-allocated fixed arrays (e.g., `[u64; 4]`).

## Usage and Testing
To verify the algebraic invariants and run the boundary test suite:

```bash
cargo test
```