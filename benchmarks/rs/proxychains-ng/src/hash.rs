use ::libc;
pub type __uint32_t = libc::c_uint;
pub type uint32_t = __uint32_t;
pub type uint_fast32_t = libc::c_ulong;
pub unsafe extern "C" fn dalias_hash(mut s0: *mut libc::c_char) -> uint32_t {
    let mut s: *mut libc::c_uchar = s0 as *mut libc::c_void as *mut libc::c_uchar;
    let mut h: uint_fast32_t = 0 as libc::c_int as uint_fast32_t;
    while *s != 0 {
        let fresh0 = s;
        s = s.offset(1);
        h = (16 as libc::c_int as libc::c_ulong)
            .wrapping_mul(h)
            .wrapping_add(*fresh0 as libc::c_ulong);
        h ^= h >> 24 as libc::c_int & 0xf0 as libc::c_int as libc::c_ulong;
    }
    return (h & 0xfffffff as libc::c_int as libc::c_ulong) as uint32_t;
}
