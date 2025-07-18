use ::libc;
pub type uint32_t = __uint32_t;
pub type __uint32_t = libc::c_uint;
pub type size_t = libc::c_ulong;
pub type uint64_t = __uint64_t;
pub type __uint64_t = libc::c_ulong;
static mut FNV_64_INIT: uint64_t = 0xcbf29ce484222325 as libc::c_ulong;
static mut FNV_64_PRIME: uint64_t = 0x100000001b3 as libc::c_ulong;
static mut FNV_32_INIT: uint32_t = 2166136261 as libc::c_ulong as uint32_t;
static mut FNV_32_PRIME: uint32_t = 16777619 as libc::c_int as uint32_t;
pub unsafe extern "C" fn hash_fnv1_64(
    mut key: *const libc::c_char,
    mut key_length: size_t,
) -> uint32_t {
    let mut hash: uint64_t = FNV_64_INIT;
    let mut x: size_t = 0;
    x = 0 as libc::c_int as size_t;
    while x < key_length {
        hash = (hash as libc::c_ulong).wrapping_mul(FNV_64_PRIME) as uint64_t
            as uint64_t;
        hash ^= *key.offset(x as isize) as uint64_t;
        x = x.wrapping_add(1);
        x;
    }
    return hash as uint32_t;
}
pub unsafe extern "C" fn hash_fnv1a_64(
    mut key: *const libc::c_char,
    mut key_length: size_t,
) -> uint32_t {
    let mut hash: uint32_t = FNV_64_INIT as uint32_t;
    let mut x: size_t = 0;
    x = 0 as libc::c_int as size_t;
    while x < key_length {
        let mut val: uint32_t = *key.offset(x as isize) as uint32_t;
        hash ^= val;
        hash = (hash as libc::c_uint).wrapping_mul(FNV_64_PRIME as uint32_t) as uint32_t
            as uint32_t;
        x = x.wrapping_add(1);
        x;
    }
    return hash;
}
pub unsafe extern "C" fn hash_fnv1_32(
    mut key: *const libc::c_char,
    mut key_length: size_t,
) -> uint32_t {
    let mut hash: uint32_t = FNV_32_INIT;
    let mut x: size_t = 0;
    x = 0 as libc::c_int as size_t;
    while x < key_length {
        let mut val: uint32_t = *key.offset(x as isize) as uint32_t;
        hash = (hash as libc::c_uint).wrapping_mul(FNV_32_PRIME) as uint32_t as uint32_t;
        hash ^= val;
        x = x.wrapping_add(1);
        x;
    }
    return hash;
}
pub unsafe extern "C" fn hash_fnv1a_32(
    mut key: *const libc::c_char,
    mut key_length: size_t,
) -> uint32_t {
    let mut hash: uint32_t = FNV_32_INIT;
    let mut x: size_t = 0;
    x = 0 as libc::c_int as size_t;
    while x < key_length {
        let mut val: uint32_t = *key.offset(x as isize) as uint32_t;
        hash ^= val;
        hash = (hash as libc::c_uint).wrapping_mul(FNV_32_PRIME) as uint32_t as uint32_t;
        x = x.wrapping_add(1);
        x;
    }
    return hash;
}
