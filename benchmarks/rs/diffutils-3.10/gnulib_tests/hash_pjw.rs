use ::libc;
pub type size_t = libc::c_ulong;
pub unsafe extern "C" fn hash_pjw(
    mut x: *const libc::c_void,
    mut tablesize: size_t,
) -> size_t {
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut h: size_t = 0 as libc::c_int as size_t;
    s = x as *const libc::c_char;
    while *s != 0 {
        h = (*s as libc::c_ulong)
            .wrapping_add(
                h << 9 as libc::c_int
                    | h
                        >> (::std::mem::size_of::<size_t>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(9 as libc::c_int as libc::c_ulong),
            );
        s = s.offset(1);
        s;
    }
    return h.wrapping_rem(tablesize);
}
