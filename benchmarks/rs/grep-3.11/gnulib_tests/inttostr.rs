use ::libc;
pub unsafe extern "C" fn inttostr(
    mut i: libc::c_int,
    mut buf: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut p: *mut libc::c_char = buf
        .offset(
            (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                .wrapping_sub(
                    !((0 as libc::c_int) < -(1 as libc::c_int)) as libc::c_int
                        as libc::c_ulong,
                )
                .wrapping_mul(146 as libc::c_int as libc::c_ulong)
                .wrapping_add(484 as libc::c_int as libc::c_ulong)
                .wrapping_div(485 as libc::c_int as libc::c_ulong)
                .wrapping_add(
                    !((0 as libc::c_int) < -(1 as libc::c_int)) as libc::c_int
                        as libc::c_ulong,
                ) as isize,
        );
    *p = 0 as libc::c_int as libc::c_char;
    if i < 0 as libc::c_int {
        loop {
            p = p.offset(-1);
            *p = ('0' as i32 - i % 10 as libc::c_int) as libc::c_char;
            i /= 10 as libc::c_int;
            if !(i != 0 as libc::c_int) {
                break;
            }
        }
        p = p.offset(-1);
        *p = '-' as i32 as libc::c_char;
    } else {
        loop {
            p = p.offset(-1);
            *p = ('0' as i32 + i % 10 as libc::c_int) as libc::c_char;
            i /= 10 as libc::c_int;
            if !(i != 0 as libc::c_int) {
                break;
            }
        }
    }
    return p;
}
