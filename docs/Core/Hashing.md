# ⚡ Hashing (TQL-11 Algorithm)

Tequel-rs implements the **TQL-11** algorithm, a 384-bit hash function optimized for **AVX2 SIMD** architectures, delivering elite throughput for large-scale data integrity.

## Function Variants

### 1. Hexadecimal String (`tqlhash`)
Returns the 384-bit hash as a formatted hexadecimal `String` (96 characters). Best for logging, database storage, and UI display.

```rust
let hash: String = teq.tqlhash(b"input");
```

### 2. Raw Bytes (`tqlhash_raw`)
Returns the internal state as a fixed-size byte array `[u8; 48]`. This is the most efficient method, ideal for high-frequency internal processing and FFI boundaries.

```rust
let hash: [u8; 48] = teq.tqlhash_raw(b"input");
```

## Salt Management
Tequel supports custom salting to mitigate Rainbow Table attacks.

```rust
use tequel::{TequelHash, TequelRng};

let rng = TequelRng::new();

let teq = TequelHash::new()
    .with_salt(rng.rand_by_nano()); // High-entropy dynamic salt
```

> Hardware Note: Optimal performance is achieved on x86_64 CPUs with native AVX2 support.


## Performance Benchmarks (AVX2)
Tequel (TQL-11) is designed to saturate the memory bus on modern x86_64 CPUs, achieving ~22.13 GiB/s (in parallel with `rayon`) and ~7.7 GiB/s in single-core by utilizing 256-bit YMM registers and instruction 2x unrolling.
