
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

use zeroize::{Zeroize, ZeroizeOnDrop};

#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

/// TequelHash is a struct that controls Hashing, it has `Constants`, `Salt` and `Custom Iterations`. <br><br>
/// Your functions are:
/// - `dif_hash_string`
/// - `dt_hash_string`
/// - `dif_hash_bytes`
/// - `dt_hash_bytes`
/// - `is_valid_hash_from_string`
/// - `is_valid_hash_from_bytes`
#[derive(Debug, Zeroize, ZeroizeOnDrop, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TequelHash {
    pub states: [u32; 12],
    pub salt: String,
    pub iterations: u32
}

impl TequelHash {

    pub fn new() -> Self { 
        Self {
            states: [
                0x1A2B3C4D, 0x5E6F7A8B, 0x9C0D1E2F, 0x31415926,
                0x27182818, 0xDEADBEEF, 0xCAFEBABE, 0x80808080,
                0xABCDEF01, 0x456789AB, 0xFEDCBA98, 0x01234567
            ],
            salt: "".to_string(),
            iterations: 30
        } 
    }



    pub fn with_salt(mut self, salt: &str) -> Self {
        self.salt = salt.to_string();
        self
    }

    pub fn with_iteration(mut self, value: u32) -> Self{
        self.iterations = value;
        self
    }

    /// <br>
    /// 
    /// ```rust
    /// 
    /// use tequel_rs::hash::TequelHash;
    /// 
    /// let mut tequelHash: TequelHash = TequelHash::new();
    /// 
    /// let mybytes: &[u8] = b"secret";
    /// 
    /// let hash: String = tequelHash.tqlhash(&mybytes); // -> 9as12sk21...
    /// let hash1: String = tequelHash.tqlhash(&mybytes); // -> 9as12sk21...
    /// ```
    /// Generates a unique HASH with input (`&[u8]`), this uses SIMD/AVX2 to generate.
    pub fn tqlhash(&mut self, input: &[u8]) -> String {

        self.states = [0u32; 12];
        let mut states_simd = unsafe { [_mm256_setzero_si256(); 12] };

        let mut chunks = input.chunks_exact(32);

        for chunk in chunks.by_ref() {
            unsafe {
                let data_vec = _mm256_loadu_si256(chunk.as_ptr() as *const __m256i);

                states_simd[0] = _mm256_add_epi32(states_simd[0], data_vec);
                states_simd[0] = _mm256_or_si256(_mm256_slli_epi32(states_simd[0], 7), _mm256_srli_epi32(states_simd[0], 25));
                states_simd[1] = _mm256_xor_si256(states_simd[1], states_simd[0]);

                // S1 -> R13: Quebrando a simetria
                states_simd[1] = _mm256_add_epi32(states_simd[1], data_vec);
                states_simd[1] = _mm256_or_si256(_mm256_slli_epi32(states_simd[1], 13), _mm256_srli_epi32(states_simd[1], 19));
                states_simd[2] = _mm256_xor_si256(states_simd[2], states_simd[1]);

                // S2 -> R19: Expansão de bits
                states_simd[2] = _mm256_add_epi32(states_simd[2], data_vec);
                states_simd[2] = _mm256_or_si256(_mm256_slli_epi32(states_simd[2], 19), _mm256_srli_epi32(states_simd[2], 13));
                states_simd[3] = _mm256_xor_si256(states_simd[3], states_simd[2]);

                // S3 -> R23: Rotação pesada
                states_simd[3] = _mm256_add_epi32(states_simd[3], data_vec);
                states_simd[3] = _mm256_or_si256(_mm256_slli_epi32(states_simd[3], 23), _mm256_srli_epi32(states_simd[3], 9));
                states_simd[4] = _mm256_xor_si256(states_simd[4], states_simd[3]);

                // S4 -> R29: Quase um giro completo
                states_simd[4] = _mm256_add_epi32(states_simd[4], data_vec);
                states_simd[4] = _mm256_or_si256(_mm256_slli_epi32(states_simd[4], 29), _mm256_srli_epi32(states_simd[4], 3));
                states_simd[5] = _mm256_xor_si256(states_simd[5], states_simd[4]);

                // S5 -> R5: O toque sutil
                states_simd[5] = _mm256_add_epi32(states_simd[5], data_vec);
                states_simd[5] = _mm256_or_si256(_mm256_slli_epi32(states_simd[5], 5), _mm256_srli_epi32(states_simd[5], 27));
                states_simd[6] = _mm256_xor_si256(states_simd[6], states_simd[5]);

                // S6 -> R11: Difusão primária
                states_simd[6] = _mm256_add_epi32(states_simd[6], data_vec);
                states_simd[6] = _mm256_or_si256(_mm256_slli_epi32(states_simd[6], 11), _mm256_srli_epi32(states_simd[6], 21));
                states_simd[7] = _mm256_xor_si256(states_simd[7], states_simd[6]);

                // S7 -> R17: Centro da entropia
                states_simd[7] = _mm256_add_epi32(states_simd[7], data_vec);
                states_simd[7] = _mm256_or_si256(_mm256_slli_epi32(states_simd[7], 17), _mm256_srli_epi32(states_simd[7], 15));
                states_simd[8] = _mm256_xor_si256(states_simd[8], states_simd[7]);

                // S8 -> R25: Inversão do S0
                states_simd[8] = _mm256_add_epi32(states_simd[8], data_vec);
                states_simd[8] = _mm256_or_si256(_mm256_slli_epi32(states_simd[8], 25), _mm256_srli_epi32(states_simd[8], 7));
                states_simd[9] = _mm256_xor_si256(states_simd[9], states_simd[8]);

                // S9 -> R3: Curto e rápido
                states_simd[9] = _mm256_add_epi32(states_simd[9], data_vec);
                states_simd[9] = _mm256_or_si256(_mm256_slli_epi32(states_simd[9], 3), _mm256_srli_epi32(states_simd[9], 29));
                states_simd[10] = _mm256_xor_si256(states_simd[10], states_simd[9]);

                // S10 -> R31: O limite do bit
                states_simd[10] = _mm256_add_epi32(states_simd[10], data_vec);
                states_simd[10] = _mm256_or_si256(_mm256_slli_epi32(states_simd[10], 31), _mm256_srli_epi32(states_simd[10], 28));
                states_simd[11] = _mm256_xor_si256(states_simd[11], states_simd[10]);

                // S11 -> R2: Finalizador da corrente
                states_simd[11] = _mm256_add_epi32(states_simd[11], data_vec);
                states_simd[11] = _mm256_or_si256(_mm256_slli_epi32(states_simd[11], 2), _mm256_srli_epi32(states_simd[11], 30));
                states_simd[0] = _mm256_xor_si256(states_simd[0], states_simd[11]);

            }
        }

        let remainder = chunks.remainder();
        for (idx, &byte) in remainder.iter().enumerate() {
            let pos = idx % 12;
            self.states[pos] = self.states[pos].wrapping_add((byte as u32) ^ 0x9E3779B1);
        }

        for i in 0..12 {
            unsafe {
                self.states[i] = self.states[i].wrapping_add(self.horiz_add_avx2(states_simd[i]));
            }
        }

        self.apply_final_mixer_64();

        self.states.iter().map(|s| {
            let mut h = *s;
            h ^= h >> 16;
            h = h.wrapping_mul(0x85ebca6b);
            h ^= h >> 13;
            h = h.wrapping_mul(0xc2b2ae35);
            h ^= h >> 16;
            format!("{:08x}", h)
        }).collect::<String>()

    }



    #[inline]
    unsafe fn horiz_add_avx2(&self, v: __m256i) -> u32 {
        let mut arr = [0u32; 8];
        unsafe { _mm256_storeu_si256(arr.as_mut_ptr() as *mut __m256i, v); }
        arr.iter().fold(0, |acc, &x| acc.wrapping_add(x))
    }

    fn apply_final_mixer_64(&mut self) {
        for r in 0..64 {
            for i in 0..12 {
                let prev = if i == 0 { 11 } else { i - 1 };
                let next = (i + 1) % 12;

                self.states[i] = self.states[i]
                    .wrapping_add(self.states[prev])
                    .rotate_left(((r % 31) as u32) + 1);
                self.states[next] ^= self.states[i].wrapping_mul(0xAD35744D);
            }
        }
    }


    /// <br>
    /// 
    /// ```rust
    /// 
    /// use tequel_rs::hash::TequelHash;
    /// 
    /// let mut tequelHash: TequelHash = TequelHash::new();
    /// 
    /// let mybytes: &[u8] = b"secret";
    /// 
    /// let hash: String = tequelHash.tqlhash(&mybytes); // -> 9as12sk21...
    /// 
    /// if tequelHash.isv_tqlhash(&hash, &mybytes) {
    ///     println!("VALID!")
    /// } else {
    ///     println!("NO VALID!")
    /// }
    /// 
    /// ```
    /// Generates a unique HASH for the same `&[u8]`.
    pub fn isv_tqlhash(&mut self, hash: &String, value: &[u8]) -> bool {
        
        let mut prop_tequel = TequelHash::new()
            .with_salt(&self.salt)
            .with_iteration(self.iterations);

        if *hash == prop_tequel.tqlhash(&value) {
            true
        } else {
            false
        }

    }




    pub fn derive_key(&mut self, password: &str, iterations: u32) -> [u8; 32] {

        self.iterations = if iterations > 0 { iterations } else { 30 };

        let mut derived = format!("{}{}{}", self.salt, password, self.salt);

        for i in 0..self.iterations {
            let hash_hex = self.tqlhash(derived.as_bytes());
            derived = format!("{}{}{}", i, hash_hex, self.salt);
        }

        let final_hash = self.tqlhash(derived.as_bytes());
        let bytes = hex::decode(&final_hash).expect("Error in key closing");

        let mut key = [0u8; 32];
        key.copy_from_slice(&bytes[0..32]);
        key
    }


}