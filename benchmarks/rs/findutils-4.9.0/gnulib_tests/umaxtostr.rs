use ::libc;
pub type __uintmax_t = libc::c_ulong;
pub type uintmax_t = __uintmax_t;
pub unsafe extern "C" fn umaxtostr(
    mut i: uintmax_t,
    mut buf: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut p: *mut libc::c_char = buf
        .offset(
            (::std::mem::size_of::<uintmax_t>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                .wrapping_sub(
                    !((0 as libc::c_int as uintmax_t) < -(1 as libc::c_int) as uintmax_t)
                        as libc::c_int as libc::c_ulong,
                )
                .wrapping_mul(146 as libc::c_int as libc::c_ulong)
                .wrapping_add(484 as libc::c_int as libc::c_ulong)
                .wrapping_div(485 as libc::c_int as libc::c_ulong)
                .wrapping_add(
                    !((0 as libc::c_int as uintmax_t) < -(1 as libc::c_int) as uintmax_t)
                        as libc::c_int as libc::c_ulong,
                ) as isize,
        );
    *p = 0 as libc::c_int as libc::c_char;
    if i < 0 as libc::c_int as libc::c_ulong {
        loop {
            p = p.offset(-1);
            *p = ('0' as i32 as libc::c_ulong)
                .wrapping_sub(i.wrapping_rem(10 as libc::c_int as libc::c_ulong))
                as libc::c_char;
            i = (i as libc::c_ulong).wrapping_div(10 as libc::c_int as libc::c_ulong)
                as uintmax_t as uintmax_t;
            if !(i != 0 as libc::c_int as libc::c_ulong) {
                break;
            }
        }
        p = p.offset(-1);
        *p = '-' as i32 as libc::c_char;
    } else {
        loop {
            p = p.offset(-1);
            *p = ('0' as i32 as libc::c_ulong)
                .wrapping_add(i.wrapping_rem(10 as libc::c_int as libc::c_ulong))
                as libc::c_char;
            i = (i as libc::c_ulong).wrapping_div(10 as libc::c_int as libc::c_ulong)
                as uintmax_t as uintmax_t;
            if !(i != 0 as libc::c_int as libc::c_ulong) {
                break;
            }
        }
    }
    return p;
}
