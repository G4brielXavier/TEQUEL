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

pub mod error;
pub mod hash;
pub mod encrypt;
pub mod rng;
pub mod avx2_inline;


// --- FFI Boundary ---

#[unsafe(no_mangle)]
pub extern "C" fn tequel_hash_raw(data: *const u8, len: usize, out: *mut u8) {
    let input = unsafe { std::slice::from_raw_parts(data, len) };

    let mut teq = hash::TequelHash::new();
    let hash_result = teq.tqlhash_raw(input);

    unsafe {
        std::ptr::copy_nonoverlapping(hash_result.as_ptr(), out, 48);
    }
} 
