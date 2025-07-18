use ::libc;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub unsafe extern "C" fn _nettle_umac_l3_init(
    mut size: libc::c_uint,
    mut k: *mut uint64_t,
) {
    let mut i: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while i < size {
        let mut w: uint64_t = *k.offset(i as isize);
        w = (w & 0xff as libc::c_int as libc::c_ulong) << 56 as libc::c_int
            | (w & 0xff00 as libc::c_int as libc::c_ulong) << 40 as libc::c_int
            | (w & 0xff0000 as libc::c_int as libc::c_ulong) << 24 as libc::c_int
            | (w & 0xff000000 as libc::c_uint as libc::c_ulong) << 8 as libc::c_int
            | w >> 8 as libc::c_int & 0xff000000 as libc::c_uint as libc::c_ulong
            | w >> 24 as libc::c_int & 0xff0000 as libc::c_int as libc::c_ulong
            | w >> 40 as libc::c_int & 0xff00 as libc::c_int as libc::c_ulong
            | w >> 56 as libc::c_int;
        *k
            .offset(
                i as isize,
            ) = (w as libc::c_ulonglong).wrapping_rem(0xffffffffb as libc::c_ulonglong)
            as uint64_t;
        i = i.wrapping_add(1);
        i;
    }
}
unsafe extern "C" fn umac_l3_word(mut k: *const uint64_t, mut w: uint64_t) -> uint64_t {
    let mut i: libc::c_uint = 0;
    let mut y: uint64_t = 0;
    y = 0 as libc::c_int as uint64_t;
    i = y as libc::c_uint;
    while i < 4 as libc::c_int as libc::c_uint {
        y = (y as libc::c_ulong)
            .wrapping_add(
                (w & 0xffff as libc::c_int as libc::c_ulong)
                    .wrapping_mul(
                        *k
                            .offset(
                                (3 as libc::c_int as libc::c_uint).wrapping_sub(i) as isize,
                            ),
                    ),
            ) as uint64_t as uint64_t;
        i = i.wrapping_add(1);
        i;
        w >>= 16 as libc::c_int;
    }
    return y;
}
pub unsafe extern "C" fn _nettle_umac_l3(
    mut key: *const uint64_t,
    mut m: *const uint64_t,
) -> uint32_t {
    let mut y: uint32_t = ((umac_l3_word(key, *m.offset(0 as libc::c_int as isize)))
        .wrapping_add(
            umac_l3_word(
                key.offset(4 as libc::c_int as isize),
                *m.offset(1 as libc::c_int as isize),
            ),
        ) as libc::c_ulonglong)
        .wrapping_rem(0xffffffffb as libc::c_ulonglong) as uint32_t;
    y = ((y << 8 as libc::c_int | y >> (-(8 as libc::c_int) & 31 as libc::c_int))
        as libc::c_ulong & 0xff00ff as libc::c_ulong
        | (y << 24 as libc::c_int | y >> (-(24 as libc::c_int) & 31 as libc::c_int))
            as libc::c_ulong & 0xff00ff00 as libc::c_ulong) as uint32_t;
    return y;
}
