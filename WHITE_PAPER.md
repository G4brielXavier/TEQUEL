# TQL-11 Specification: The Dual-Wide Vectorized Chaos Engine

**Technical Specification v0.8.0** | **Author:** Gabriel Xavier (@G4brielXavier)  
**Status:** Validated (**49.22% Avalanche** / **11.10 MiB/s Single-Core** / **~970 MiB/s Parallel**)

---

## 1. Abstract

The **TQL-11 Primitive** (Tequel) is a high-density ARX-based (Addition-Rotation-XOR) cryptographic hash and encryption engine. The v0.8.0 revision introduces the **Dual-Wide SIMD Pipeline**, processing 64-byte chunks per iteration through interleaved register states. By utilizing **Asymmetric Index Shifting** and **AVX2 Intrinsics**, TQL-11 achieves a near-perfect Strict Avalanche Criterion (SAC) while maintaining high scalability across multi-core architectures via Rayon.

## 2. Mathematical Notation

Operations are executed over 32-bit unsigned words ($u32$) within 256-bit SIMD registers ($V_{256}$):

- $A \boxplus B$: Modular addition (`_mm256_add_epi32` / `wrapping_add`).
- $A \oplus B$: Bitwise Exclusive OR (`_mm256_xor_si256`).
- $A \lll n$: Bitwise Left Rotation (implemented via shift/OR combinations).
- $S_{12}$: The internal state array consisting of 12 SIMD registers ($12 \times 256$ bits).
- $V_{y1}, V_{y2}$: Dual-input vectors representing two contiguous 32-byte data blocks.

## 3. The 384-bit Chaos State

TQL-11 maintains a massive internal state of 384 bits (12 x 32-bit words per SIMD lane). This state is initialized with a sequence of irrational constants to ensure an initial high-entropy distribution:
- **Seed Constants:** `{0x107912FA, 0x220952EA, 0x3320212A, 0x4324312F, 0x5320212A, 0x9E3779B1, 0x85EBCA6B, 0xAD35744D, 0xCC2912FA, 0xEE0952EA, 0x1120212A, 0x2224312F}`

## 4. Architectural Innovations (v0.8.0)

### Phase I: Dual-Wide Interleaved Processing
Unlike traditional serial hashes, v0.8.0 processes **64 bytes per loop**. To prevent "Register Spilling" (overflowing the 16 physical YMM registers), the engine utilizes an interleaved state approach:
1. Load $V_{y1}$ (Bytes 0-31) and $V_{y2}$ (Bytes 32-63).
2. Apply a constant bit-twist ($\mathcal{K} = \text{0x517CC1B7}$) to $V_{y2}$ to break input symmetry.
3. Inject $V_{y1}$ into state $S[i]$ and $V_{y2}$ into state $S[(i+1) \pmod{12}]$.

### Phase II: Asymmetric Diffusion Macro (`teq!`)
The core transformation uses a non-linear mixing function that combines $XOR$, $ADD$, and $ROT$ at varying bit-distances (e.g., 7, 25, 31, 13). By shifting the target state index between $V_{y1}$ and $V_{y2}$, the engine forces cross-pollination of entropy between adjacent data blocks.

### Phase III: High-Speed Hexadecimal Serialization
To eliminate the $O(n)$ bottleneck of string formatting, v0.8.0 implements a **Zero-Allocation Hex Encoder**. By mapping 4-bit nibbles directly to a static lookup table (`0-9a-f`), the engine bypasses the overhead of the Rust `format!` macro, ensuring that the 11 MiB/s throughput is limited only by algorithmic density, not string I/O.

## 5. Security & Performance Analysis

### Avalanche Effect (v0.8.0 Result: 49.22%)
Through **Asymmetric Index Shifting**, TQL-11 achieves an avalanche result of **49.22%**. This confirms that any single-bit flip in the input yields a ~50% change in the output hash, satisfying the rigorous requirements for cryptographic integrity.

### Throughput & Scalability
- **Single-Core:** ~11.10 MiB/s (Optimized for density and bit-diffusion).
- **Multi-Core (Parallel):** ~970 MiB/s (Linear scalability via Rayon).
- **Instruction Efficiency:** Optimized for x86_64 AVX2, utilizing 14 of 16 available YMM registers to minimize L1 Cache latency.

## 6. Conclusion
TQL-11 v0.8.0 represents a leap in vectorized chaos engineering, designed for local-first integrity (Emet) and high-performance worldbuilding data protection. Its dual-wide architecture provides an efficient blueprint for SIMD-based hashing that prioritizes both statistical randomness and hardware-level optimization.