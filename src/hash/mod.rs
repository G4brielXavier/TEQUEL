use std::str::from_utf8;

use zeroize::{Zeroize, ZeroizeOnDrop};

#[cfg(feature = "serialization")]
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
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
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


    // from &str 
    /// <br>
    /// 
    /// ```
    /// 
    /// use tequel_rs::hash::TequelHash;
    /// 
    /// let mut tequelHash: TequelHash = TequelHash::new();
    /// 
    /// let hash: String = tequelHash.dif_hash_string("my_secret"); // -> s2ohs192...
    /// let hash1: String = tequelHash.dif_hash_string("my_secret"); // -> 29js19ss...
    /// ```
    /// Generates a different HASH even `&str` being same.
    pub fn dif_hash_string(&mut self, input: &str) -> String {

        let combined = format!("{}{}", self.salt, input);

        let byteinput: &[u8] = combined.as_bytes();

        for (idx, byte) in byteinput.iter().enumerate() {

            let b = *byte as u32;

            let pos = idx % 12;

            self.states[pos] = self.states[pos].wrapping_add(b ^ 0x9E3779B1);
            // self.states[5] = self.states[5] ^ b.rotate_left(4);
            // self.states[11] = self.states[11].wrapping_sub(b.rotate_left(8));

            for _ in 0..4 {
                for i in 0..12 {
                    let next = (i + 1) % 12;
                    let jump = (i + 5) % 12;

                    let dyna_shift = (self.states[i].count_ones() + i as u32) % 32;

                    self.states[i] = self.states[i]
                        .wrapping_add(self.states[jump])
                        .rotate_left(dyna_shift);

                    self.states[next] = (self.states[next] ^ self.states[i])
                        .wrapping_mul(0x85EBCA6B);
                }
            }

            
        }
        
        self.states[0] = self.states[0] ^ self.states[11];

        for r in 0..64 {
            for i in 0..12 {

                let prev = if i == 0 { 11 } else { i - 1 };
                let next = (i + 1) % 12;

                self.states[i] = self.states[i]
                    .wrapping_add(self.states[prev])
                    .rotate_left((r % 31) + 1);


                self.states[next] ^= self.states[i].wrapping_mul(0xAD35744D);

            }
        }
        
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

    /// <br>
    /// 
    /// ```rust
    /// 
    /// use tequel_rs::hash::TequelHash;
    /// 
    /// let mut tequelHash: TequelHash = TequelHash::new();
    /// 
    /// let hash: String = tequelHash.dt_hash_string("my_secret"); // -> 9as12sk21...
    /// let hash1: String = tequelHash.dt_hash_string("my_secret"); // -> 9as12sk21...
    /// ```
    /// Generates a unique HASH from the same `&str`.
    pub fn dt_hash_string(&mut self, input: &str) -> String {

        self.states = Self::new().states;

        let combined = format!("{}{}", self.salt, input);

        let byteinput: &[u8] = combined.as_bytes();

        for (idx, byte) in byteinput.iter().enumerate() {

            let b = *byte as u32;

            let pos = idx % 12;

            self.states[pos] = self.states[pos].wrapping_add(b ^ 0x9E3779B1);
            // self.states[5] = self.states[5] ^ b.rotate_left(4);
            // self.states[11] = self.states[11].wrapping_sub(b.rotate_left(8));

            for _ in 0..4 {
                for i in 0..12 {
                    let next = (i + 1) % 12;
                    let jump = (i + 5) % 12;

                    let dyna_shift = (self.states[i].count_ones() + i as u32) % 32;

                    self.states[i] = self.states[i]
                        .wrapping_add(self.states[jump])
                        .rotate_left(dyna_shift);

                    self.states[next] = (self.states[next] ^ self.states[i])
                        .wrapping_mul(0x85EBCA6B);
                }
            }

            
        }
        
        self.states[0] = self.states[0] ^ self.states[11];

        for r in 0..64 {
            for i in 0..12 {

                let prev = if i == 0 { 11 } else { i - 1 };
                let next = (i + 1) % 12;

                self.states[i] = self.states[i]
                    .wrapping_add(self.states[prev])
                    .rotate_left((r % 31) + 1);


                self.states[next] ^= self.states[i].wrapping_mul(0xAD35744D);

            }
        }
        
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




    // from &[u8]

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
    /// let hash: String = tequelHash.dif_hash_bytes(&mybytes); // -> 9as12sk21...
    /// let hash1: String = tequelHash.dif_hash_bytes(&mybytes); // -> 29js19ss...
    /// ```
    /// Generates a different HASH even `&[u8]` being same
    pub fn dif_hash_bytes(&mut self, input: &[u8]) -> String {

        let combined = format!("{}{}", self.salt, from_utf8(input).unwrap());

        let byteinput: &[u8] = combined.as_bytes();

        for (idx, byte) in byteinput.iter().enumerate() {

            let b = *byte as u32;

            let pos = idx % 12;

            self.states[pos] = self.states[pos].wrapping_add(b ^ 0x9E3779B1);
            // self.states[5] = self.states[5] ^ b.rotate_left(4);
            // self.states[11] = self.states[11].wrapping_sub(b.rotate_left(8));

            for _ in 0..4 {
                for i in 0..12 {
                    let next = (i + 1) % 12;
                    let jump = (i + 5) % 12;

                    let dyna_shift = (self.states[i].count_ones() + i as u32) % 32;

                    self.states[i] = self.states[i]
                        .wrapping_add(self.states[jump])
                        .rotate_left(dyna_shift);

                    self.states[next] = (self.states[next] ^ self.states[i])
                        .wrapping_mul(0x85EBCA6B);
                }
            }

            
        }
        
        self.states[0] = self.states[0] ^ self.states[11];

        for r in 0..64 {
            for i in 0..12 {

                let prev = if i == 0 { 11 } else { i - 1 };
                let next = (i + 1) % 12;

                self.states[i] = self.states[i]
                    .wrapping_add(self.states[prev])
                    .rotate_left((r % 31) + 1);


                self.states[next] ^= self.states[i].wrapping_mul(0xAD35744D);

            }
        }
        
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
    /// let hash: String = tequelHash.dt_hash_bytes(&mybytes); // -> 9as12sk21...
    /// let hash1: String = tequelHash.dt_hash_bytes(&mybytes); // -> 9as12sk21...
    /// ```
    /// Generates a unique HASH for the same `&[u8]`.
    pub fn dt_hash_bytes(&mut self, input: &[u8]) -> String {

        self.states = Self::new().states;

        let combined = format!("{}{}", self.salt, from_utf8(input).unwrap());

        let byteinput: &[u8] = combined.as_bytes();

        for (idx, byte) in byteinput.iter().enumerate() {

            let b = *byte as u32;

            let pos = idx % 12;

            self.states[pos] = self.states[pos].wrapping_add(b ^ 0x9E3779B1);
            // self.states[5] = self.states[5] ^ b.rotate_left(4);
            // self.states[11] = self.states[11].wrapping_sub(b.rotate_left(8));

            for _ in 0..4 {
                for i in 0..12 {
                    let next = (i + 1) % 12;
                    let jump = (i + 5) % 12;

                    let dyna_shift = (self.states[i].count_ones() + i as u32) % 32;

                    self.states[i] = self.states[i]
                        .wrapping_add(self.states[jump])
                        .rotate_left(dyna_shift);

                    self.states[next] = (self.states[next] ^ self.states[i])
                        .wrapping_mul(0x85EBCA6B);
                }
            }

            
        }
        
        self.states[0] = self.states[0] ^ self.states[11];

        for r in 0..64 {
            for i in 0..12 {

                let prev = if i == 0 { 11 } else { i - 1 };
                let next = (i + 1) % 12;

                self.states[i] = self.states[i]
                    .wrapping_add(self.states[prev])
                    .rotate_left((r % 31) + 1);


                self.states[next] ^= self.states[i].wrapping_mul(0xAD35744D);

            }
        }
        
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
    /// let hash: String = tequelHash.dt_hash_bytes(&mybytes); // -> 9as12sk21...
    /// 
    /// if tequelHash.is_valid_hash_from_bytes(&hash, &mybytes) {
    ///     println!("VALID!")
    /// } else {
    ///     println!("NO VALID!")
    /// }
    /// 
    /// ```
    /// Generates a unique HASH for the same `&[u8]`.
    pub fn is_valid_hash_from_bytes(&mut self, hash: &String, value: &[u8]) -> bool {
        
        let mut prop_tequel = TequelHash::new()
            .with_salt(&self.salt)
            .with_iteration(self.iterations);

        if *hash == prop_tequel.dt_hash_bytes(&value) {
            true
        } else {
            false
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
    /// let my_data: &str = "secret";
    /// 
    /// let hash: String = tequelHash.dt_hash_string(my_data); // -> 9as12sk21...
    /// 
    /// if tequelHash.is_valid_hash_from_string(&hash, &my_data) {
    ///     println!("VALID!")
    /// } else {
    ///     println!("NO VALID!")
    /// }
    /// 
    /// ```
    /// Generates a unique HASH for the same `&[u8]`.
    pub fn is_valid_hash_from_string(&mut self, hash: &String, value: &str) -> bool {
        
        let mut prop_tequel = TequelHash::new()
            .with_salt(&self.salt)
            .with_iteration(self.iterations);

        if *hash == prop_tequel.dt_hash_string(&value) {
            true
        } else {
            false
        }
        
    }




    pub fn derive_key(&mut self, password: &str) -> [u8; 32] {
        let mut derived = format!("{}{}{}", self.salt, password, self.salt);

        for i in 0..self.iterations {
            let hash_hex = self.dt_hash_bytes(derived.as_bytes());
            derived = format!("{}{}{}", i, hash_hex, self.salt);
        }

        let final_hash = self.dt_hash_bytes(derived.as_bytes());
        let bytes = hex::decode(&final_hash).expect("Error in key closing");

        let mut key = [0u8; 32];
        key.copy_from_slice(&bytes[0..32]);
        key
    }


}