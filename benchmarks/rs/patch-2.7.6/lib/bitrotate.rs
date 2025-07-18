use ::libc;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type size_t = libc::c_ulong;
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn rotr8(mut x: uint8_t, mut n: libc::c_int) -> uint8_t {
    return ((x as libc::c_int >> n | (x as libc::c_int) << 8 as libc::c_int - n)
        & 255 as libc::c_int) as uint8_t;
}
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn rotl8(mut x: uint8_t, mut n: libc::c_int) -> uint8_t {
    return (((x as libc::c_int) << n | x as libc::c_int >> 8 as libc::c_int - n)
        & 255 as libc::c_int) as uint8_t;
}
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn rotr16(mut x: uint16_t, mut n: libc::c_int) -> uint16_t {
    return ((x as libc::c_int >> n | (x as libc::c_int) << 16 as libc::c_int - n)
        & 65535 as libc::c_int) as uint16_t;
}
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn rotl16(mut x: uint16_t, mut n: libc::c_int) -> uint16_t {
    return (((x as libc::c_int) << n | x as libc::c_int >> 16 as libc::c_int - n)
        & 65535 as libc::c_int) as uint16_t;
}
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn rotr_sz(mut x: size_t, mut n: libc::c_int) -> size_t {
    return (x >> n
        | x
            << (8 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<size_t>() as libc::c_ulong)
                .wrapping_sub(n as libc::c_ulong))
        & 18446744073709551615 as libc::c_ulong;
}
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn rotl64(mut x: uint64_t, mut n: libc::c_int) -> uint64_t {
    return (x << n | x >> 64 as libc::c_int - n) & 18446744073709551615 as libc::c_ulong;
}
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn rotr64(mut x: uint64_t, mut n: libc::c_int) -> uint64_t {
    return (x >> n | x << 64 as libc::c_int - n) & 18446744073709551615 as libc::c_ulong;
}
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn rotl32(mut x: uint32_t, mut n: libc::c_int) -> uint32_t {
    return (x << n | x >> 32 as libc::c_int - n) & 4294967295 as libc::c_uint;
}
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn rotr32(mut x: uint32_t, mut n: libc::c_int) -> uint32_t {
    return (x >> n | x << 32 as libc::c_int - n) & 4294967295 as libc::c_uint;
}
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn rotl_sz(mut x: size_t, mut n: libc::c_int) -> size_t {
    return (x << n
        | x
            >> (8 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<size_t>() as libc::c_ulong)
                .wrapping_sub(n as libc::c_ulong))
        & 18446744073709551615 as libc::c_ulong;
}
