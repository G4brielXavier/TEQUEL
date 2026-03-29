use std::arch::x86_64::*;


#[inline(always)]
pub unsafe fn loadu(src: *const __m256i) -> __m256i {
    unsafe { _mm256_loadu_si256(src as *const __m256i) }
}


#[inline(always)]
pub unsafe fn storeu(dest: *mut __m256i, src: __m256i) {
    unsafe { _mm256_storeu_si256(dest, src) }
}


#[inline(always)]
pub unsafe fn add(a: __m256i, b: __m256i) -> __m256i {
    unsafe { _mm256_add_epi32(a, b) }
}


#[inline(always)]
pub unsafe fn sub(a: __m256i, b: __m256i) -> __m256i {
    unsafe { _mm256_sub_epi32(a, b) }
}




#[inline(always)]
pub unsafe fn xor(a: __m256i, b: __m256i) -> __m256i {
    unsafe { _mm256_xor_si256(a, b) }
}


#[inline(always)]
pub unsafe fn or(a: __m256i, b: __m256i) -> __m256i {
    unsafe { _mm256_or_si256(a, b) }
}


#[inline(always)]
pub unsafe fn setzero() -> __m256i {
    unsafe { _mm256_setzero_si256() }
}


#[inline(always)]
pub unsafe fn setone(v: i8) -> __m256i {
    unsafe { _mm256_set1_epi8(v) }
}



#[inline(always)]
pub unsafe fn rota_lf<const IMM8: i32>(c: __m256i) -> __m256i {
    unsafe { _mm256_slli_epi32(c, IMM8) }
}


#[inline(always)]
pub unsafe fn rota_rg<const IMM8: i32>(c: __m256i) -> __m256i {
    unsafe { _mm256_srli_epi32(c, IMM8) }
}