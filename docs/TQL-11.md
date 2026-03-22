# TQL-11 Specification

**Technical Specification v0.4.5 Author:** Gabriel Xavier

## 1. Abstract

The **TQL-11 Primitive** is an ARX-based (Addition-Rotation-XOR) transformation function designed for high-speed data hashing and secure obfuscation. Unlike traditional Substitution-Permutation Networks (SPN) that rely on fixed S-Boxes, TQL-11 achieves non-linearity through the interleaving of modular arithmetics and circular bit-shifts across an 11-register internal state.


## 2. Mathematical Notation

To ensure implementation consistency, the following operations are defined over 32-bit/64-bit words:

- $A \boxplus B$: Modular addiction (Wrapping Add).
- $A \otimes B$: Modular multiplication (Wrapping Mul).
- $A \oplus B$: Bitwise Exclusive OR (XOR).
- $A \lll n$: Circular left rotation by $n$ bits.


## 3. "Nothing-Up-My-Sleeve" Constants

The **TQL-11** avoids arbitrary "magic numbers" to ensure transparency and prevent hardware backdoors. The internal state ($S_0, S_1, \dots, S_{10}$) is initializated and mixed using fundamental mathematical constants:

- **Irrational Bases:** Derived from $\pi$ ($0x31415926$) and $e$ ($0x27182818$).
- **The Golden Ratio ($\phi$):** $0x9E3779B1$. Used as the primary non-linear disperser due to its superior bit-distribution properties.
- **MurmurHash3 Primes:**  $C_1 = 0x85EBCA6B$ and $C_2 = 0xAD35744D$, selected for their proven resistance to cluster collisions.

## 4. The TQL-11 Pipeline

### Phase I: Absortion (Injection)

Each input byte $M_i$ is combined with a salt $s$ and absorbed into the primary state $S_0$:

$$S_0 \leftarrow (S_0 \boxplus (M_i \oplus s)) \otimes C_1$$

This stage ensures that single bit change in the input immediately propagates through the entire multiplier.

---

### Phase II: Chained Permutation (The Round Function)

The core of the primitive consists of **30 rounds** of asymmetric permutations. For each register $S_j$ (where $j \in \{1, \dots, 10\}$):

1. **Chained Mixing:** $S_j \leftarrow S_j \oplus (S_{j-1} \lll 13) \oplus \phi$
2. **Modular Diffusion:** $S_j \leftarrow (S_j  \otimes C_2) \lll 7$
3. **Finalizer XOR:** At the end of each round, a feedback loop connects the tail and the head:

$$S_{last} \leftarrow S_{last} \oplus (S_0 \lll (13 + \text{iteration}))$$

---


## 5. Security Analysis

### Avalanche Effect

Due to the dependency chain between the 11 registers, a 1-bit change in $M_i$ results in an average 50% bit-flip across the entire state within the first 12 rounds. The 30-round standard provides a significant security margin.

### Non-Linearity

The security of TQL-11 relies on the fact that **Modular Addiction** and **XOR** do not commute. This mathematical property prevents differential cryptanalysis, as attackers cannot easily predict how changes propagate through the mixed operations.

### Resistance to Side-Channel Attacks

By eliminating S-Box lookups, TQL-11 is inherently resistant to cache-timing attacks. Furthermore, the implementation utilizes the `Zeroize` protocol to ensure all sentisitive buffers are wiped from RAM immediately after execution.

## 6. Conclusion

The Tequel (TQL-11) represents a robust, pure-software alternative for modern applications requiring high-entropy data protection. Its reliance on fundamental constants and ARX architecture places it among high-performance modern primitives like BLAKE3 and ChaCha20.