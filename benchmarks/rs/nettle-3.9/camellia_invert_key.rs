use ::libc;
pub type __uint64_t = libc::c_ulong;
pub type uint64_t = __uint64_t;
pub unsafe extern "C" fn _nettle_camellia_invert_key(
    mut nkeys: libc::c_uint,
    mut dst: *mut uint64_t,
    mut src: *const uint64_t,
) {
    let mut i: libc::c_uint = 0;
    if dst == src as *mut uint64_t {
        i = 0 as libc::c_int as libc::c_uint;
        while i < nkeys.wrapping_sub(1 as libc::c_int as libc::c_uint).wrapping_sub(i) {
            let mut t_swap: uint64_t = *dst.offset(i as isize);
            *dst
                .offset(
                    i as isize,
                ) = *dst
                .offset(
                    nkeys.wrapping_sub(1 as libc::c_int as libc::c_uint).wrapping_sub(i)
                        as isize,
                );
            *dst
                .offset(
                    nkeys.wrapping_sub(1 as libc::c_int as libc::c_uint).wrapping_sub(i)
                        as isize,
                ) = t_swap;
            i = i.wrapping_add(1);
            i;
        }
    } else {
        i = 0 as libc::c_int as libc::c_uint;
        while i < nkeys {
            *dst
                .offset(
                    i as isize,
                ) = *src
                .offset(
                    nkeys.wrapping_sub(1 as libc::c_int as libc::c_uint).wrapping_sub(i)
                        as isize,
                );
            i = i.wrapping_add(1);
            i;
        }
    };
}
