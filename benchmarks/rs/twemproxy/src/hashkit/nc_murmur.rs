use ::libc;
pub type uint32_t = __uint32_t;
pub type __uint32_t = libc::c_uint;
pub type size_t = libc::c_ulong;
pub unsafe extern "C" fn hash_murmur(
    mut key: *const libc::c_char,
    mut length: size_t,
) -> uint32_t {
    let m: libc::c_uint = 0x5bd1e995 as libc::c_int as libc::c_uint;
    let seed: uint32_t = (0xdeadbeef as libc::c_uint).wrapping_mul(length as uint32_t);
    let r: libc::c_int = 24 as libc::c_int;
    let mut h: uint32_t = seed ^ length as uint32_t;
    let mut data: *const libc::c_uchar = key as *const libc::c_uchar;
    while length >= 4 as libc::c_int as libc::c_ulong {
        let mut k: libc::c_uint = *(data as *mut libc::c_uint);
        k = k.wrapping_mul(m);
        k ^= k >> r;
        k = k.wrapping_mul(m);
        h = (h as libc::c_uint).wrapping_mul(m) as uint32_t as uint32_t;
        h ^= k;
        data = data.offset(4 as libc::c_int as isize);
        length = (length as libc::c_ulong)
            .wrapping_sub(4 as libc::c_int as libc::c_ulong) as size_t as size_t;
    }
    let mut current_block_12: u64;
    match length {
        3 => {
            h
                ^= (*data.offset(2 as libc::c_int as isize) as uint32_t)
                    << 16 as libc::c_int;
            current_block_12 = 220676342854842915;
        }
        2 => {
            current_block_12 = 220676342854842915;
        }
        1 => {
            current_block_12 = 13329554123530295150;
        }
        _ => {
            current_block_12 = 12800627514080957624;
        }
    }
    match current_block_12 {
        220676342854842915 => {
            h
                ^= (*data.offset(1 as libc::c_int as isize) as uint32_t)
                    << 8 as libc::c_int;
            current_block_12 = 13329554123530295150;
        }
        _ => {}
    }
    match current_block_12 {
        13329554123530295150 => {
            h ^= *data.offset(0 as libc::c_int as isize) as libc::c_uint;
            h = (h as libc::c_uint).wrapping_mul(m) as uint32_t as uint32_t;
        }
        _ => {}
    }
    h ^= h >> 13 as libc::c_int;
    h = (h as libc::c_uint).wrapping_mul(m) as uint32_t as uint32_t;
    h ^= h >> 15 as libc::c_int;
    return h;
}
