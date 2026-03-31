/*
 * Tequel-rs: High-Density 384-bit Cryptographic Hash Engine
 * Copyright (C) 2026 Gabriel Xavier (dotxav)
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published
 * by the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 */


use std::{time::SystemTime, vec};
use getrandom::getrandom;



/// TequelRng is a struct that controls RNG functions. <br><br>
/// As:
/// - `rand_by_nano`
/// - `rand_deep_string`
/// - `rand_weak_u32`
/// - `rand_lgc`
/// - `rand_in_range_by_deep`
pub struct TequelRng {}

impl TequelRng {

    pub fn new() -> Self { Self {  } }



    // RNG by Nano seconds

    /// Generates a set of numbers using **Nano Seconds** + **Constants**
    pub fn rand_by_nano(&self) -> String {

        let now = SystemTime::now();

        if let Ok(dur) = now.duration_since(SystemTime::UNIX_EPOCH) {
            let nanos = dur.subsec_nanos();
            let mut bytes_nanos = nanos.to_be_bytes();

            for (i, byte) in bytes_nanos.iter_mut().enumerate() {
                let magic = [0x6a, 0x09, 0xe6, 0x67, 0xbb, 0x67, 0xae, 0x85];
                *byte = byte.wrapping_add(magic[i % magic.len()]).rotate_left(5);
            }

            return bytes_nanos.iter().map(|b| format!("{:02x}", b)).collect::<String>()
        };
        
        "".to_string()
    }




    // RNG by SO/Hardware

    /// Generates a set of numbers and letters with custom length delivered. It returs a ``String``
    pub fn rand_deep_string(&self, len: usize) -> String {

        let mut buffer = vec![0u8; len];
        getrandom(&mut buffer).unwrap();

        for (i, byte) in buffer.iter_mut().enumerate() {
            let magic = [0x6a, 0x09, 0xe6, 0x67, 0xbb, 0x67, 0xae, 0x85];
            *byte = byte.wrapping_add(magic[i % magic.len()]).rotate_left(3);
        }

        buffer.iter().map(|b| format!("{:02x}", b)).collect()

    }

    /// Generates a set of random `u32` from hardware byte noises.
    pub fn rand_u32(&self) -> u32 {
        
        let mut buffer = [0u8; 4];

        getrandom(&mut buffer).unwrap();

        u32::from_ne_bytes(buffer)

    }



    // LGC (Linear Congruential Generator)

    /// Generates a random `u32` from a M.A (Modular Arithmetic) with `seed`. 
    pub fn rand_lgc(&self, mut seed: u32) -> u32 {

        let a: u32 = 1201029921;
        let c: u32 = 885949403;
        let m: u32 = 82721712;

        seed = (a.wrapping_mul(seed).wrapping_add(c)) % m;
        seed

    }



    // In range
    
    /// Generates a random `u32` between `min` and `max` using `rand_u32`.
    pub fn rand_range(&self, min: u32, max: u32) -> u32 {

        let range = max - min + 1;
        let limit = u32::MAX - (u32::MAX % range);

        loop {
            let res = self.rand_u32();
            if res < limit {
                return min + (res % range);
            }
        }

    }

}