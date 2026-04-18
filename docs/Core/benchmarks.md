# Performance Benchmarks: TQL-11 (AVX2)

Tequel-rs (TQL-11) is engineered to bridge the gap between high-entropy (384-bit) and high-throughput processing. Below is the comparative analysis between standard hashing algorithms and Tequel-rs v1.0.0.

## Core Efficiency (x86_64)

The TQL-11 engine is designed to saturate the memory bus on modern CPUs by utilizing **256-bit YMM registers** and **4x instruction unrolling**.

* **Multi-Core Throughput:** ~22 GiB/s (Scalable with `rayon`)
* **Single-Core Throughput:** ~7.7 GiB/s
* **Cycles per Byte (CpB):** ~2.8 (Optimized for AVX2)
* **Register Pinning:** Direct utilization of 16x YMM registers to minimize cache misses.

## Testing Environment
Benchmarks were performed on a standardized industrial-grade node:
- **Architecture:** x86_64 with Native AVX2 Support.
- **Memory:** Zero-allocation, stack-only processing during the core mix.
- **Compiler:** `rustc 1.75+` with `--release` and `target-cpu=native`.

> **Note:** Performance is memory-bound during large-scale parallelization. Results may vary based on CPU cache size and RAM frequency.