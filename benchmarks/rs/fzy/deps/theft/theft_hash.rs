use ::libc;
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint64_t = libc::c_ulong;
pub type uint8_t = __uint8_t;
pub type uint64_t = __uint64_t;
pub type theft_hash = uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct theft_hasher {
    pub accum: theft_hash,
}
static mut fnv64_prime: uint64_t = 1099511628211 as libc::c_long as uint64_t;
static mut fnv64_offset_basis: uint64_t = 14695981039346656037 as libc::c_ulong;
pub unsafe extern "C" fn theft_hash_init(mut h: *mut theft_hasher) {
    (*h).accum = fnv64_offset_basis;
}
pub unsafe extern "C" fn theft_hash_sink(
    mut h: *mut theft_hasher,
    mut data: *mut uint8_t,
    mut bytes: size_t,
) {
    if h.is_null() || data.is_null() {
        return;
    }
    let mut a: uint64_t = (*h).accum;
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < bytes {
        a = (a ^ *data.offset(i as isize) as libc::c_ulong).wrapping_mul(fnv64_prime);
        i = i.wrapping_add(1);
        i;
    }
    (*h).accum = a;
}
pub unsafe extern "C" fn theft_hash_done(mut h: *mut theft_hasher) -> theft_hash {
    let mut res: theft_hash = (*h).accum;
    theft_hash_init(h);
    return res;
}
pub unsafe extern "C" fn theft_hash_onepass(
    mut data: *mut uint8_t,
    mut bytes: size_t,
) -> theft_hash {
    let mut h: theft_hasher = theft_hasher { accum: 0 };
    theft_hash_init(&mut h);
    theft_hash_sink(&mut h, data, bytes);
    return theft_hash_done(&mut h);
}
