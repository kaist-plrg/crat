use ::libc;
pub type __time_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type C2RustUnnamed = libc::c_uint;
pub const TIMESPEC_HZ: C2RustUnnamed = 1000000000;
#[inline]
unsafe extern "C" fn make_timespec(mut s: time_t, mut ns: libc::c_long) -> timespec {
    return {
        let mut init = timespec { tv_sec: s, tv_nsec: ns };
        init
    };
}
pub unsafe extern "C" fn dtotimespec(mut sec: libc::c_double) -> timespec {
    if !((!(if (0 as libc::c_int as time_t) < -(1 as libc::c_int) as time_t {
        -(1 as libc::c_int) as time_t
    } else {
        (((1 as libc::c_int as time_t)
            << (::std::mem::size_of::<time_t>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
            - 1 as libc::c_int as libc::c_long) * 2 as libc::c_int as libc::c_long
            + 1 as libc::c_int as libc::c_long
    }) as libc::c_double) < sec)
    {
        return make_timespec(
            !if (0 as libc::c_int as time_t) < -(1 as libc::c_int) as time_t {
                -(1 as libc::c_int) as time_t
            } else {
                (((1 as libc::c_int as time_t)
                    << (::std::mem::size_of::<time_t>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                    - 1 as libc::c_int as libc::c_long)
                    * 2 as libc::c_int as libc::c_long + 1 as libc::c_int as libc::c_long
            },
            0 as libc::c_int as libc::c_long,
        )
    } else if !(sec
        < 1.0f64
            + (if (0 as libc::c_int as time_t) < -(1 as libc::c_int) as time_t {
                -(1 as libc::c_int) as time_t
            } else {
                (((1 as libc::c_int as time_t)
                    << (::std::mem::size_of::<time_t>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                    - 1 as libc::c_int as libc::c_long)
                    * 2 as libc::c_int as libc::c_long + 1 as libc::c_int as libc::c_long
            }) as libc::c_double)
    {
        return make_timespec(
            if (0 as libc::c_int as time_t) < -(1 as libc::c_int) as time_t {
                -(1 as libc::c_int) as time_t
            } else {
                (((1 as libc::c_int as time_t)
                    << (::std::mem::size_of::<time_t>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                    - 1 as libc::c_int as libc::c_long)
                    * 2 as libc::c_int as libc::c_long + 1 as libc::c_int as libc::c_long
            },
            (TIMESPEC_HZ as libc::c_int - 1 as libc::c_int) as libc::c_long,
        )
    } else {
        let mut s: time_t = sec as time_t;
        let mut frac: libc::c_double = TIMESPEC_HZ as libc::c_int as libc::c_double
            * (sec - s as libc::c_double);
        let mut ns: libc::c_long = frac as libc::c_long;
        ns += ((ns as libc::c_double) < frac) as libc::c_int as libc::c_long;
        s += ns / TIMESPEC_HZ as libc::c_int as libc::c_long;
        ns %= TIMESPEC_HZ as libc::c_int as libc::c_long;
        if ns < 0 as libc::c_int as libc::c_long {
            s -= 1;
            s;
            ns += TIMESPEC_HZ as libc::c_int as libc::c_long;
        }
        return make_timespec(s, ns);
    };
}
