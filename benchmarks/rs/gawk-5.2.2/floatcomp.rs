use ::libc;
pub type __uintmax_t = libc::c_ulong;
pub type uintmax_t = __uintmax_t;
unsafe extern "C" fn count_trailing_zeros(mut n: uintmax_t) -> libc::c_int {
    return (n as libc::c_ulonglong).trailing_zeros() as i32;
}
pub unsafe extern "C" fn adjust_uint(mut n: uintmax_t) -> uintmax_t {
    let mut wordbits: libc::c_int = (8 as libc::c_int as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<uintmax_t>() as libc::c_ulong)
        as libc::c_int;
    if (if ::std::mem::size_of::<libc::c_double>() as libc::c_ulong
        == ::std::mem::size_of::<f128::f128>() as libc::c_ulong
    {
        64 as libc::c_int
    } else {
        if ::std::mem::size_of::<libc::c_double>() as libc::c_ulong
            == ::std::mem::size_of::<libc::c_double>() as libc::c_ulong
        {
            53 as libc::c_int
        } else {
            24 as libc::c_int
        }
    })
        * (if 2 as libc::c_int == 2 as libc::c_int {
            1 as libc::c_int
        } else {
            4 as libc::c_int
        }) < wordbits
    {
        let mut one: uintmax_t = 1 as libc::c_int as uintmax_t;
        let mut sentinel: uintmax_t = one
            << wordbits
                - (if ::std::mem::size_of::<libc::c_double>() as libc::c_ulong
                    == ::std::mem::size_of::<f128::f128>() as libc::c_ulong
                {
                    64 as libc::c_int
                } else {
                    if ::std::mem::size_of::<libc::c_double>() as libc::c_ulong
                        == ::std::mem::size_of::<libc::c_double>() as libc::c_ulong
                    {
                        53 as libc::c_int
                    } else {
                        24 as libc::c_int
                    }
                })
                    * (if 2 as libc::c_int == 2 as libc::c_int {
                        1 as libc::c_int
                    } else {
                        4 as libc::c_int
                    });
        let mut shift: libc::c_int = count_trailing_zeros(n | sentinel);
        let mut mask: uintmax_t = (one
            << (if ::std::mem::size_of::<libc::c_double>() as libc::c_ulong
                == ::std::mem::size_of::<f128::f128>() as libc::c_ulong
            {
                64 as libc::c_int
            } else {
                if ::std::mem::size_of::<libc::c_double>() as libc::c_ulong
                    == ::std::mem::size_of::<libc::c_double>() as libc::c_ulong
                {
                    53 as libc::c_int
                } else {
                    24 as libc::c_int
                }
            })
                * (if 2 as libc::c_int == 2 as libc::c_int {
                    1 as libc::c_int
                } else {
                    4 as libc::c_int
                }))
            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        n &= mask << shift;
    }
    return n;
}
