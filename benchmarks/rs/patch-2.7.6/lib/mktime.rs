use ::libc;
pub type __time_t = libc::c_long;
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: libc::c_int,
    pub tm_min: libc::c_int,
    pub tm_hour: libc::c_int,
    pub tm_mday: libc::c_int,
    pub tm_mon: libc::c_int,
    pub tm_year: libc::c_int,
    pub tm_wday: libc::c_int,
    pub tm_yday: libc::c_int,
    pub tm_isdst: libc::c_int,
    pub tm_gmtoff: libc::c_long,
    pub tm_zone: *const libc::c_char,
}
pub type long_int = libc::c_long;
pub type mktime_offset_t = time_t;
unsafe extern "C" fn shr(mut a: long_int, mut b: libc::c_int) -> long_int {
    let mut one: long_int = 1 as libc::c_int as long_int;
    return if -one >> 1 as libc::c_int == -(1 as libc::c_int) as libc::c_long {
        a >> b
    } else {
        a / (one << b)
            - (a % (one << b) < 0 as libc::c_int as libc::c_long) as libc::c_int
                as libc::c_long
    };
}
static mut mktime_min: long_int = 0;
static mut mktime_max: long_int = 0;
unsafe extern "C" fn leapyear(mut year: long_int) -> bool {
    return year & 3 as libc::c_int as libc::c_long == 0 as libc::c_int as libc::c_long
        && (year % 100 as libc::c_int as libc::c_long != 0 as libc::c_int as libc::c_long
            || year / 100 as libc::c_int as libc::c_long
                & 3 as libc::c_int as libc::c_long
                == (-(1900 as libc::c_int / 100 as libc::c_int) & 3 as libc::c_int)
                    as libc::c_long);
}
static mut __mon_yday: [[libc::c_ushort; 13]; 2] = [
    [
        0 as libc::c_int as libc::c_ushort,
        31 as libc::c_int as libc::c_ushort,
        59 as libc::c_int as libc::c_ushort,
        90 as libc::c_int as libc::c_ushort,
        120 as libc::c_int as libc::c_ushort,
        151 as libc::c_int as libc::c_ushort,
        181 as libc::c_int as libc::c_ushort,
        212 as libc::c_int as libc::c_ushort,
        243 as libc::c_int as libc::c_ushort,
        273 as libc::c_int as libc::c_ushort,
        304 as libc::c_int as libc::c_ushort,
        334 as libc::c_int as libc::c_ushort,
        365 as libc::c_int as libc::c_ushort,
    ],
    [
        0 as libc::c_int as libc::c_ushort,
        31 as libc::c_int as libc::c_ushort,
        60 as libc::c_int as libc::c_ushort,
        91 as libc::c_int as libc::c_ushort,
        121 as libc::c_int as libc::c_ushort,
        152 as libc::c_int as libc::c_ushort,
        182 as libc::c_int as libc::c_ushort,
        213 as libc::c_int as libc::c_ushort,
        244 as libc::c_int as libc::c_ushort,
        274 as libc::c_int as libc::c_ushort,
        305 as libc::c_int as libc::c_ushort,
        335 as libc::c_int as libc::c_ushort,
        366 as libc::c_int as libc::c_ushort,
    ],
];
unsafe extern "C" fn isdst_differ(mut a: libc::c_int, mut b: libc::c_int) -> bool {
    return (a == 0) as libc::c_int != (b == 0) as libc::c_int && 0 as libc::c_int <= a
        && 0 as libc::c_int <= b;
}
unsafe extern "C" fn ydhms_diff(
    mut year1: long_int,
    mut yday1: long_int,
    mut hour1: libc::c_int,
    mut min1: libc::c_int,
    mut sec1: libc::c_int,
    mut year0: libc::c_int,
    mut yday0: libc::c_int,
    mut hour0: libc::c_int,
    mut min0: libc::c_int,
    mut sec0: libc::c_int,
) -> long_int {
    extern "C" {
        #[link_name = "_gl_verify_function5"]
        fn _gl_verify_function5_0() -> *mut [libc::c_int; 1];
    }
    let mut a4: libc::c_int = (shr(year1, 2 as libc::c_int)
        + shr(1900 as libc::c_int as long_int, 2 as libc::c_int)
        - (year1 & 3 as libc::c_int as libc::c_long == 0) as libc::c_int as libc::c_long)
        as libc::c_int;
    let mut b4: libc::c_int = (shr(year0 as long_int, 2 as libc::c_int)
        + shr(1900 as libc::c_int as long_int, 2 as libc::c_int)
        - (year0 & 3 as libc::c_int == 0) as libc::c_int as libc::c_long) as libc::c_int;
    let mut a100: libc::c_int = a4 / 25 as libc::c_int
        - ((a4 % 25 as libc::c_int) < 0 as libc::c_int) as libc::c_int;
    let mut b100: libc::c_int = b4 / 25 as libc::c_int
        - ((b4 % 25 as libc::c_int) < 0 as libc::c_int) as libc::c_int;
    let mut a400: libc::c_int = shr(a100 as long_int, 2 as libc::c_int) as libc::c_int;
    let mut b400: libc::c_int = shr(b100 as long_int, 2 as libc::c_int) as libc::c_int;
    let mut intervening_leap_days: libc::c_int = a4 - b4 - (a100 - b100) + (a400 - b400);
    let mut years: long_int = year1 - year0 as libc::c_long;
    let mut days: long_int = 365 as libc::c_int as libc::c_long * years + yday1
        - yday0 as libc::c_long + intervening_leap_days as libc::c_long;
    let mut hours: long_int = 24 as libc::c_int as libc::c_long * days
        + hour1 as libc::c_long - hour0 as libc::c_long;
    let mut minutes: long_int = 60 as libc::c_int as libc::c_long * hours
        + min1 as libc::c_long - min0 as libc::c_long;
    let mut seconds: long_int = 60 as libc::c_int as libc::c_long * minutes
        + sec1 as libc::c_long - sec0 as libc::c_long;
    return seconds;
}
unsafe extern "C" fn long_int_avg(mut a: long_int, mut b: long_int) -> long_int {
    return shr(a, 1 as libc::c_int) + shr(b, 1 as libc::c_int)
        + ((a | b) & 1 as libc::c_int as libc::c_long);
}
unsafe extern "C" fn guess_time_tm(
    mut year: long_int,
    mut yday: long_int,
    mut hour: libc::c_int,
    mut min: libc::c_int,
    mut sec: libc::c_int,
    mut t: long_int,
    mut tp: *const tm,
) -> long_int {
    if !tp.is_null() {
        let mut result: long_int = 0;
        let mut d: long_int = ydhms_diff(
            year,
            yday,
            hour,
            min,
            sec,
            (*tp).tm_year,
            (*tp).tm_yday,
            (*tp).tm_hour,
            (*tp).tm_min,
            (*tp).tm_sec,
        );
        if if ::std::mem::size_of::<long_int>() as libc::c_ulong
            == ::std::mem::size_of::<libc::c_schar>() as libc::c_ulong
        {
            if (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                < ::std::mem::size_of::<libc::c_schar>() as libc::c_ulong
            {
                if (if (if (0 as libc::c_int
                    * (0 as libc::c_int * d as libc::c_schar as libc::c_int
                        + t as libc::c_schar as libc::c_int) - 1 as libc::c_int)
                    < 0 as libc::c_int
                {
                    !((((0 as libc::c_int
                        * (0 as libc::c_int * d as libc::c_schar as libc::c_int
                            + t as libc::c_schar as libc::c_int) + 1 as libc::c_int)
                        << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                        - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                } else {
                    0 as libc::c_int
                        * (0 as libc::c_int * d as libc::c_schar as libc::c_int
                            + t as libc::c_schar as libc::c_int) + 0 as libc::c_int
                }) < 0 as libc::c_int
                {
                    if (d as libc::c_schar as libc::c_int) < 0 as libc::c_int {
                        ((t as libc::c_schar as libc::c_int)
                            < (if (0 as libc::c_int
                                * (0 as libc::c_int * d as libc::c_schar as libc::c_int
                                    + t as libc::c_schar as libc::c_int) - 1 as libc::c_int)
                                < 0 as libc::c_int
                            {
                                !((((0 as libc::c_int
                                    * (0 as libc::c_int * d as libc::c_schar as libc::c_int
                                        + t as libc::c_schar as libc::c_int) + 1 as libc::c_int)
                                    << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                    - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                            } else {
                                0 as libc::c_int
                                    * (0 as libc::c_int * d as libc::c_schar as libc::c_int
                                        + t as libc::c_schar as libc::c_int) + 0 as libc::c_int
                            }) - d as libc::c_schar as libc::c_int) as libc::c_int
                    } else {
                        (((if (0 as libc::c_int
                            * (0 as libc::c_int * d as libc::c_schar as libc::c_int
                                + t as libc::c_schar as libc::c_int) - 1 as libc::c_int)
                            < 0 as libc::c_int
                        {
                            (((0 as libc::c_int
                                * (0 as libc::c_int * d as libc::c_schar as libc::c_int
                                    + t as libc::c_schar as libc::c_int) + 1 as libc::c_int)
                                << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                        } else {
                            0 as libc::c_int
                                * (0 as libc::c_int * d as libc::c_schar as libc::c_int
                                    + t as libc::c_schar as libc::c_int) - 1 as libc::c_int
                        }) - d as libc::c_schar as libc::c_int)
                            < t as libc::c_schar as libc::c_int) as libc::c_int
                    }
                } else {
                    if (t as libc::c_schar as libc::c_int) < 0 as libc::c_int {
                        (d as libc::c_schar as libc::c_int
                            <= t as libc::c_schar as libc::c_int
                                + d as libc::c_schar as libc::c_int) as libc::c_int
                    } else {
                        if (d as libc::c_schar as libc::c_int) < 0 as libc::c_int {
                            (t as libc::c_schar as libc::c_int
                                <= t as libc::c_schar as libc::c_int
                                    + d as libc::c_schar as libc::c_int) as libc::c_int
                        } else {
                            ((t as libc::c_schar as libc::c_int
                                + d as libc::c_schar as libc::c_int)
                                < d as libc::c_schar as libc::c_int) as libc::c_int
                        }
                    }
                }) != 0
                    || (0 as libc::c_int
                        * (t as libc::c_schar as libc::c_int
                            + d as libc::c_schar as libc::c_int) - 1 as libc::c_int)
                        < 0 as libc::c_int
                        && (t as libc::c_schar as libc::c_int
                            + d as libc::c_schar as libc::c_int)
                            < -(127 as libc::c_int) - 1 as libc::c_int
                    || (127 as libc::c_int)
                        < t as libc::c_schar as libc::c_int
                            + d as libc::c_schar as libc::c_int
                {
                    result = (t as libc::c_schar as libc::c_uint)
                        .wrapping_add(d as libc::c_schar as libc::c_uint)
                        as libc::c_schar as long_int;
                    1 as libc::c_int
                } else {
                    result = (t as libc::c_schar as libc::c_uint)
                        .wrapping_add(d as libc::c_schar as libc::c_uint)
                        as libc::c_schar as long_int;
                    0 as libc::c_int
                }
            } else if (if (if (0 as libc::c_int as libc::c_long
                * (0 as libc::c_int as libc::c_long * d + t)
                - 1 as libc::c_int as libc::c_long) < 0 as libc::c_int as libc::c_long
            {
                !((((0 as libc::c_int as libc::c_long
                    * (0 as libc::c_int as libc::c_long * d + t)
                    + 1 as libc::c_int as libc::c_long)
                    << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                    - 1 as libc::c_int as libc::c_long)
                    * 2 as libc::c_int as libc::c_long
                    + 1 as libc::c_int as libc::c_long)
            } else {
                0 as libc::c_int as libc::c_long
                    * (0 as libc::c_int as libc::c_long * d + t)
                    + 0 as libc::c_int as libc::c_long
            }) < 0 as libc::c_int as libc::c_long
            {
                if d < 0 as libc::c_int as libc::c_long {
                    (t
                        < (if (0 as libc::c_int as libc::c_long
                            * (0 as libc::c_int as libc::c_long * d + t)
                            - 1 as libc::c_int as libc::c_long)
                            < 0 as libc::c_int as libc::c_long
                        {
                            !((((0 as libc::c_int as libc::c_long
                                * (0 as libc::c_int as libc::c_long * d + t)
                                + 1 as libc::c_int as libc::c_long)
                                << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                - 1 as libc::c_int as libc::c_long)
                                * 2 as libc::c_int as libc::c_long
                                + 1 as libc::c_int as libc::c_long)
                        } else {
                            0 as libc::c_int as libc::c_long
                                * (0 as libc::c_int as libc::c_long * d + t)
                                + 0 as libc::c_int as libc::c_long
                        }) - d) as libc::c_int
                } else {
                    ((if (0 as libc::c_int as libc::c_long
                        * (0 as libc::c_int as libc::c_long * d + t)
                        - 1 as libc::c_int as libc::c_long)
                        < 0 as libc::c_int as libc::c_long
                    {
                        (((0 as libc::c_int as libc::c_long
                            * (0 as libc::c_int as libc::c_long * d + t)
                            + 1 as libc::c_int as libc::c_long)
                            << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                            - 1 as libc::c_int as libc::c_long)
                            * 2 as libc::c_int as libc::c_long
                            + 1 as libc::c_int as libc::c_long
                    } else {
                        0 as libc::c_int as libc::c_long
                            * (0 as libc::c_int as libc::c_long * d + t)
                            - 1 as libc::c_int as libc::c_long
                    }) - d < t) as libc::c_int
                }
            } else {
                if t < 0 as libc::c_int as libc::c_long {
                    (d <= t + d) as libc::c_int
                } else {
                    if d < 0 as libc::c_int as libc::c_long {
                        (t <= t + d) as libc::c_int
                    } else {
                        (t + d < d) as libc::c_int
                    }
                }
            }) != 0
                || (0 as libc::c_int as libc::c_long * (t + d)
                    - 1 as libc::c_int as libc::c_long)
                    < 0 as libc::c_int as libc::c_long
                    && t + d < (-(127 as libc::c_int) - 1 as libc::c_int) as libc::c_long
                || (127 as libc::c_int as libc::c_long) < t + d
            {
                result = (t as libc::c_uint).wrapping_add(d as libc::c_uint)
                    as libc::c_schar as long_int;
                1 as libc::c_int
            } else {
                result = (t as libc::c_uint).wrapping_add(d as libc::c_uint)
                    as libc::c_schar as long_int;
                0 as libc::c_int
            }
        } else if ::std::mem::size_of::<long_int>() as libc::c_ulong
            == ::std::mem::size_of::<libc::c_short>() as libc::c_ulong
        {
            if (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                < ::std::mem::size_of::<libc::c_short>() as libc::c_ulong
            {
                if (if (if (0 as libc::c_int
                    * (0 as libc::c_int * d as libc::c_short as libc::c_int
                        + t as libc::c_short as libc::c_int) - 1 as libc::c_int)
                    < 0 as libc::c_int
                {
                    !((((0 as libc::c_int
                        * (0 as libc::c_int * d as libc::c_short as libc::c_int
                            + t as libc::c_short as libc::c_int) + 1 as libc::c_int)
                        << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                        - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                } else {
                    0 as libc::c_int
                        * (0 as libc::c_int * d as libc::c_short as libc::c_int
                            + t as libc::c_short as libc::c_int) + 0 as libc::c_int
                }) < 0 as libc::c_int
                {
                    if (d as libc::c_short as libc::c_int) < 0 as libc::c_int {
                        ((t as libc::c_short as libc::c_int)
                            < (if (0 as libc::c_int
                                * (0 as libc::c_int * d as libc::c_short as libc::c_int
                                    + t as libc::c_short as libc::c_int) - 1 as libc::c_int)
                                < 0 as libc::c_int
                            {
                                !((((0 as libc::c_int
                                    * (0 as libc::c_int * d as libc::c_short as libc::c_int
                                        + t as libc::c_short as libc::c_int) + 1 as libc::c_int)
                                    << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                    - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                            } else {
                                0 as libc::c_int
                                    * (0 as libc::c_int * d as libc::c_short as libc::c_int
                                        + t as libc::c_short as libc::c_int) + 0 as libc::c_int
                            }) - d as libc::c_short as libc::c_int) as libc::c_int
                    } else {
                        (((if (0 as libc::c_int
                            * (0 as libc::c_int * d as libc::c_short as libc::c_int
                                + t as libc::c_short as libc::c_int) - 1 as libc::c_int)
                            < 0 as libc::c_int
                        {
                            (((0 as libc::c_int
                                * (0 as libc::c_int * d as libc::c_short as libc::c_int
                                    + t as libc::c_short as libc::c_int) + 1 as libc::c_int)
                                << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                        } else {
                            0 as libc::c_int
                                * (0 as libc::c_int * d as libc::c_short as libc::c_int
                                    + t as libc::c_short as libc::c_int) - 1 as libc::c_int
                        }) - d as libc::c_short as libc::c_int)
                            < t as libc::c_short as libc::c_int) as libc::c_int
                    }
                } else {
                    if (t as libc::c_short as libc::c_int) < 0 as libc::c_int {
                        (d as libc::c_short as libc::c_int
                            <= t as libc::c_short as libc::c_int
                                + d as libc::c_short as libc::c_int) as libc::c_int
                    } else {
                        if (d as libc::c_short as libc::c_int) < 0 as libc::c_int {
                            (t as libc::c_short as libc::c_int
                                <= t as libc::c_short as libc::c_int
                                    + d as libc::c_short as libc::c_int) as libc::c_int
                        } else {
                            ((t as libc::c_short as libc::c_int
                                + d as libc::c_short as libc::c_int)
                                < d as libc::c_short as libc::c_int) as libc::c_int
                        }
                    }
                }) != 0
                    || (0 as libc::c_int
                        * (t as libc::c_short as libc::c_int
                            + d as libc::c_short as libc::c_int) - 1 as libc::c_int)
                        < 0 as libc::c_int
                        && (t as libc::c_short as libc::c_int
                            + d as libc::c_short as libc::c_int)
                            < -(32767 as libc::c_int) - 1 as libc::c_int
                    || (32767 as libc::c_int)
                        < t as libc::c_short as libc::c_int
                            + d as libc::c_short as libc::c_int
                {
                    result = (t as libc::c_short as libc::c_uint)
                        .wrapping_add(d as libc::c_short as libc::c_uint)
                        as libc::c_short as long_int;
                    1 as libc::c_int
                } else {
                    result = (t as libc::c_short as libc::c_uint)
                        .wrapping_add(d as libc::c_short as libc::c_uint)
                        as libc::c_short as long_int;
                    0 as libc::c_int
                }
            } else if (if (if (0 as libc::c_int as libc::c_long
                * (0 as libc::c_int as libc::c_long * d + t)
                - 1 as libc::c_int as libc::c_long) < 0 as libc::c_int as libc::c_long
            {
                !((((0 as libc::c_int as libc::c_long
                    * (0 as libc::c_int as libc::c_long * d + t)
                    + 1 as libc::c_int as libc::c_long)
                    << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                    - 1 as libc::c_int as libc::c_long)
                    * 2 as libc::c_int as libc::c_long
                    + 1 as libc::c_int as libc::c_long)
            } else {
                0 as libc::c_int as libc::c_long
                    * (0 as libc::c_int as libc::c_long * d + t)
                    + 0 as libc::c_int as libc::c_long
            }) < 0 as libc::c_int as libc::c_long
            {
                if d < 0 as libc::c_int as libc::c_long {
                    (t
                        < (if (0 as libc::c_int as libc::c_long
                            * (0 as libc::c_int as libc::c_long * d + t)
                            - 1 as libc::c_int as libc::c_long)
                            < 0 as libc::c_int as libc::c_long
                        {
                            !((((0 as libc::c_int as libc::c_long
                                * (0 as libc::c_int as libc::c_long * d + t)
                                + 1 as libc::c_int as libc::c_long)
                                << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                - 1 as libc::c_int as libc::c_long)
                                * 2 as libc::c_int as libc::c_long
                                + 1 as libc::c_int as libc::c_long)
                        } else {
                            0 as libc::c_int as libc::c_long
                                * (0 as libc::c_int as libc::c_long * d + t)
                                + 0 as libc::c_int as libc::c_long
                        }) - d) as libc::c_int
                } else {
                    ((if (0 as libc::c_int as libc::c_long
                        * (0 as libc::c_int as libc::c_long * d + t)
                        - 1 as libc::c_int as libc::c_long)
                        < 0 as libc::c_int as libc::c_long
                    {
                        (((0 as libc::c_int as libc::c_long
                            * (0 as libc::c_int as libc::c_long * d + t)
                            + 1 as libc::c_int as libc::c_long)
                            << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                            - 1 as libc::c_int as libc::c_long)
                            * 2 as libc::c_int as libc::c_long
                            + 1 as libc::c_int as libc::c_long
                    } else {
                        0 as libc::c_int as libc::c_long
                            * (0 as libc::c_int as libc::c_long * d + t)
                            - 1 as libc::c_int as libc::c_long
                    }) - d < t) as libc::c_int
                }
            } else {
                if t < 0 as libc::c_int as libc::c_long {
                    (d <= t + d) as libc::c_int
                } else {
                    if d < 0 as libc::c_int as libc::c_long {
                        (t <= t + d) as libc::c_int
                    } else {
                        (t + d < d) as libc::c_int
                    }
                }
            }) != 0
                || (0 as libc::c_int as libc::c_long * (t + d)
                    - 1 as libc::c_int as libc::c_long)
                    < 0 as libc::c_int as libc::c_long
                    && t + d
                        < (-(32767 as libc::c_int) - 1 as libc::c_int) as libc::c_long
                || (32767 as libc::c_int as libc::c_long) < t + d
            {
                result = (t as libc::c_uint).wrapping_add(d as libc::c_uint)
                    as libc::c_short as long_int;
                1 as libc::c_int
            } else {
                result = (t as libc::c_uint).wrapping_add(d as libc::c_uint)
                    as libc::c_short as long_int;
                0 as libc::c_int
            }
        } else if ::std::mem::size_of::<long_int>() as libc::c_ulong
            == ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
        {
            if (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                < ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
            {
                if (if (if (0 as libc::c_int
                    * (0 as libc::c_int * d as libc::c_int + t as libc::c_int)
                    - 1 as libc::c_int) < 0 as libc::c_int
                {
                    !((((0 as libc::c_int
                        * (0 as libc::c_int * d as libc::c_int + t as libc::c_int)
                        + 1 as libc::c_int)
                        << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                        - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                } else {
                    0 as libc::c_int
                        * (0 as libc::c_int * d as libc::c_int + t as libc::c_int)
                        + 0 as libc::c_int
                }) < 0 as libc::c_int
                {
                    if (d as libc::c_int) < 0 as libc::c_int {
                        ((t as libc::c_int)
                            < (if (0 as libc::c_int
                                * (0 as libc::c_int * d as libc::c_int + t as libc::c_int)
                                - 1 as libc::c_int) < 0 as libc::c_int
                            {
                                !((((0 as libc::c_int
                                    * (0 as libc::c_int * d as libc::c_int + t as libc::c_int)
                                    + 1 as libc::c_int)
                                    << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                    - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                            } else {
                                0 as libc::c_int
                                    * (0 as libc::c_int * d as libc::c_int + t as libc::c_int)
                                    + 0 as libc::c_int
                            }) - d as libc::c_int) as libc::c_int
                    } else {
                        (((if (0 as libc::c_int
                            * (0 as libc::c_int * d as libc::c_int + t as libc::c_int)
                            - 1 as libc::c_int) < 0 as libc::c_int
                        {
                            (((0 as libc::c_int
                                * (0 as libc::c_int * d as libc::c_int + t as libc::c_int)
                                + 1 as libc::c_int)
                                << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                        } else {
                            0 as libc::c_int
                                * (0 as libc::c_int * d as libc::c_int + t as libc::c_int)
                                - 1 as libc::c_int
                        }) - d as libc::c_int) < t as libc::c_int) as libc::c_int
                    }
                } else {
                    if (t as libc::c_int) < 0 as libc::c_int {
                        (d as libc::c_int <= t as libc::c_int + d as libc::c_int)
                            as libc::c_int
                    } else {
                        if (d as libc::c_int) < 0 as libc::c_int {
                            (t as libc::c_int <= t as libc::c_int + d as libc::c_int)
                                as libc::c_int
                        } else {
                            ((t as libc::c_int + d as libc::c_int) < d as libc::c_int)
                                as libc::c_int
                        }
                    }
                }) != 0
                    || (0 as libc::c_int * (t as libc::c_int + d as libc::c_int)
                        - 1 as libc::c_int) < 0 as libc::c_int
                        && (t as libc::c_int + d as libc::c_int)
                            < -(2147483647 as libc::c_int) - 1 as libc::c_int
                    || (2147483647 as libc::c_int) < t as libc::c_int + d as libc::c_int
                {
                    result = (t as libc::c_int as libc::c_uint)
                        .wrapping_add(d as libc::c_int as libc::c_uint) as libc::c_int
                        as long_int;
                    1 as libc::c_int
                } else {
                    result = (t as libc::c_int as libc::c_uint)
                        .wrapping_add(d as libc::c_int as libc::c_uint) as libc::c_int
                        as long_int;
                    0 as libc::c_int
                }
            } else if (if (if (0 as libc::c_int as libc::c_long
                * (0 as libc::c_int as libc::c_long * d + t)
                - 1 as libc::c_int as libc::c_long) < 0 as libc::c_int as libc::c_long
            {
                !((((0 as libc::c_int as libc::c_long
                    * (0 as libc::c_int as libc::c_long * d + t)
                    + 1 as libc::c_int as libc::c_long)
                    << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                    - 1 as libc::c_int as libc::c_long)
                    * 2 as libc::c_int as libc::c_long
                    + 1 as libc::c_int as libc::c_long)
            } else {
                0 as libc::c_int as libc::c_long
                    * (0 as libc::c_int as libc::c_long * d + t)
                    + 0 as libc::c_int as libc::c_long
            }) < 0 as libc::c_int as libc::c_long
            {
                if d < 0 as libc::c_int as libc::c_long {
                    (t
                        < (if (0 as libc::c_int as libc::c_long
                            * (0 as libc::c_int as libc::c_long * d + t)
                            - 1 as libc::c_int as libc::c_long)
                            < 0 as libc::c_int as libc::c_long
                        {
                            !((((0 as libc::c_int as libc::c_long
                                * (0 as libc::c_int as libc::c_long * d + t)
                                + 1 as libc::c_int as libc::c_long)
                                << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                - 1 as libc::c_int as libc::c_long)
                                * 2 as libc::c_int as libc::c_long
                                + 1 as libc::c_int as libc::c_long)
                        } else {
                            0 as libc::c_int as libc::c_long
                                * (0 as libc::c_int as libc::c_long * d + t)
                                + 0 as libc::c_int as libc::c_long
                        }) - d) as libc::c_int
                } else {
                    ((if (0 as libc::c_int as libc::c_long
                        * (0 as libc::c_int as libc::c_long * d + t)
                        - 1 as libc::c_int as libc::c_long)
                        < 0 as libc::c_int as libc::c_long
                    {
                        (((0 as libc::c_int as libc::c_long
                            * (0 as libc::c_int as libc::c_long * d + t)
                            + 1 as libc::c_int as libc::c_long)
                            << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                            - 1 as libc::c_int as libc::c_long)
                            * 2 as libc::c_int as libc::c_long
                            + 1 as libc::c_int as libc::c_long
                    } else {
                        0 as libc::c_int as libc::c_long
                            * (0 as libc::c_int as libc::c_long * d + t)
                            - 1 as libc::c_int as libc::c_long
                    }) - d < t) as libc::c_int
                }
            } else {
                if t < 0 as libc::c_int as libc::c_long {
                    (d <= t + d) as libc::c_int
                } else {
                    if d < 0 as libc::c_int as libc::c_long {
                        (t <= t + d) as libc::c_int
                    } else {
                        (t + d < d) as libc::c_int
                    }
                }
            }) != 0
                || (0 as libc::c_int as libc::c_long * (t + d)
                    - 1 as libc::c_int as libc::c_long)
                    < 0 as libc::c_int as libc::c_long
                    && t + d
                        < (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                            as libc::c_long
                || (2147483647 as libc::c_int as libc::c_long) < t + d
            {
                result = (t as libc::c_uint).wrapping_add(d as libc::c_uint)
                    as libc::c_int as long_int;
                1 as libc::c_int
            } else {
                result = (t as libc::c_uint).wrapping_add(d as libc::c_uint)
                    as libc::c_int as long_int;
                0 as libc::c_int
            }
        } else if ::std::mem::size_of::<long_int>() as libc::c_ulong
            == ::std::mem::size_of::<libc::c_long>() as libc::c_ulong
        {
            if (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                < ::std::mem::size_of::<libc::c_long>() as libc::c_ulong
            {
                if (if (if (0 as libc::c_int as libc::c_long
                    * (0 as libc::c_int as libc::c_long * d + t)
                    - 1 as libc::c_int as libc::c_long)
                    < 0 as libc::c_int as libc::c_long
                {
                    !((((0 as libc::c_int as libc::c_long
                        * (0 as libc::c_int as libc::c_long * d + t)
                        + 1 as libc::c_int as libc::c_long)
                        << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                        - 1 as libc::c_int as libc::c_long)
                        * 2 as libc::c_int as libc::c_long
                        + 1 as libc::c_int as libc::c_long)
                } else {
                    0 as libc::c_int as libc::c_long
                        * (0 as libc::c_int as libc::c_long * d + t)
                        + 0 as libc::c_int as libc::c_long
                }) < 0 as libc::c_int as libc::c_long
                {
                    if d < 0 as libc::c_int as libc::c_long {
                        (t
                            < (if (0 as libc::c_int as libc::c_long
                                * (0 as libc::c_int as libc::c_long * d + t)
                                - 1 as libc::c_int as libc::c_long)
                                < 0 as libc::c_int as libc::c_long
                            {
                                !((((0 as libc::c_int as libc::c_long
                                    * (0 as libc::c_int as libc::c_long * d + t)
                                    + 1 as libc::c_int as libc::c_long)
                                    << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                    - 1 as libc::c_int as libc::c_long)
                                    * 2 as libc::c_int as libc::c_long
                                    + 1 as libc::c_int as libc::c_long)
                            } else {
                                0 as libc::c_int as libc::c_long
                                    * (0 as libc::c_int as libc::c_long * d + t)
                                    + 0 as libc::c_int as libc::c_long
                            }) - d) as libc::c_int
                    } else {
                        ((if (0 as libc::c_int as libc::c_long
                            * (0 as libc::c_int as libc::c_long * d + t)
                            - 1 as libc::c_int as libc::c_long)
                            < 0 as libc::c_int as libc::c_long
                        {
                            (((0 as libc::c_int as libc::c_long
                                * (0 as libc::c_int as libc::c_long * d + t)
                                + 1 as libc::c_int as libc::c_long)
                                << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                - 1 as libc::c_int as libc::c_long)
                                * 2 as libc::c_int as libc::c_long
                                + 1 as libc::c_int as libc::c_long
                        } else {
                            0 as libc::c_int as libc::c_long
                                * (0 as libc::c_int as libc::c_long * d + t)
                                - 1 as libc::c_int as libc::c_long
                        }) - d < t) as libc::c_int
                    }
                } else {
                    if t < 0 as libc::c_int as libc::c_long {
                        (d <= t + d) as libc::c_int
                    } else {
                        if d < 0 as libc::c_int as libc::c_long {
                            (t <= t + d) as libc::c_int
                        } else {
                            (t + d < d) as libc::c_int
                        }
                    }
                }) != 0
                    || (0 as libc::c_int as libc::c_long * (t + d)
                        - 1 as libc::c_int as libc::c_long)
                        < 0 as libc::c_int as libc::c_long
                        && t + d
                            < -(9223372036854775807 as libc::c_long) - 1 as libc::c_long
                    || (9223372036854775807 as libc::c_long) < t + d
                {
                    result = (t as libc::c_ulong).wrapping_add(d as libc::c_ulong)
                        as libc::c_long;
                    1 as libc::c_int
                } else {
                    result = (t as libc::c_ulong).wrapping_add(d as libc::c_ulong)
                        as libc::c_long;
                    0 as libc::c_int
                }
            } else if (if (if (0 as libc::c_int as libc::c_long
                * (0 as libc::c_int as libc::c_long * d + t)
                - 1 as libc::c_int as libc::c_long) < 0 as libc::c_int as libc::c_long
            {
                !((((0 as libc::c_int as libc::c_long
                    * (0 as libc::c_int as libc::c_long * d + t)
                    + 1 as libc::c_int as libc::c_long)
                    << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                    - 1 as libc::c_int as libc::c_long)
                    * 2 as libc::c_int as libc::c_long
                    + 1 as libc::c_int as libc::c_long)
            } else {
                0 as libc::c_int as libc::c_long
                    * (0 as libc::c_int as libc::c_long * d + t)
                    + 0 as libc::c_int as libc::c_long
            }) < 0 as libc::c_int as libc::c_long
            {
                if d < 0 as libc::c_int as libc::c_long {
                    (t
                        < (if (0 as libc::c_int as libc::c_long
                            * (0 as libc::c_int as libc::c_long * d + t)
                            - 1 as libc::c_int as libc::c_long)
                            < 0 as libc::c_int as libc::c_long
                        {
                            !((((0 as libc::c_int as libc::c_long
                                * (0 as libc::c_int as libc::c_long * d + t)
                                + 1 as libc::c_int as libc::c_long)
                                << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                - 1 as libc::c_int as libc::c_long)
                                * 2 as libc::c_int as libc::c_long
                                + 1 as libc::c_int as libc::c_long)
                        } else {
                            0 as libc::c_int as libc::c_long
                                * (0 as libc::c_int as libc::c_long * d + t)
                                + 0 as libc::c_int as libc::c_long
                        }) - d) as libc::c_int
                } else {
                    ((if (0 as libc::c_int as libc::c_long
                        * (0 as libc::c_int as libc::c_long * d + t)
                        - 1 as libc::c_int as libc::c_long)
                        < 0 as libc::c_int as libc::c_long
                    {
                        (((0 as libc::c_int as libc::c_long
                            * (0 as libc::c_int as libc::c_long * d + t)
                            + 1 as libc::c_int as libc::c_long)
                            << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                            - 1 as libc::c_int as libc::c_long)
                            * 2 as libc::c_int as libc::c_long
                            + 1 as libc::c_int as libc::c_long
                    } else {
                        0 as libc::c_int as libc::c_long
                            * (0 as libc::c_int as libc::c_long * d + t)
                            - 1 as libc::c_int as libc::c_long
                    }) - d < t) as libc::c_int
                }
            } else {
                if t < 0 as libc::c_int as libc::c_long {
                    (d <= t + d) as libc::c_int
                } else {
                    if d < 0 as libc::c_int as libc::c_long {
                        (t <= t + d) as libc::c_int
                    } else {
                        (t + d < d) as libc::c_int
                    }
                }
            }) != 0
                || (0 as libc::c_int as libc::c_long * (t + d)
                    - 1 as libc::c_int as libc::c_long)
                    < 0 as libc::c_int as libc::c_long
                    && t + d < -(9223372036854775807 as libc::c_long) - 1 as libc::c_long
                || (9223372036854775807 as libc::c_long) < t + d
            {
                result = (t as libc::c_ulong).wrapping_add(d as libc::c_ulong)
                    as libc::c_long;
                1 as libc::c_int
            } else {
                result = (t as libc::c_ulong).wrapping_add(d as libc::c_ulong)
                    as libc::c_long;
                0 as libc::c_int
            }
        } else if (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
            < ::std::mem::size_of::<libc::c_longlong>() as libc::c_ulong
        {
            if (if (if (0 as libc::c_int as libc::c_longlong
                * (0 as libc::c_int as libc::c_longlong * d as libc::c_longlong
                    + t as libc::c_longlong) - 1 as libc::c_int as libc::c_longlong)
                < 0 as libc::c_int as libc::c_longlong
            {
                !((((0 as libc::c_int as libc::c_longlong
                    * (0 as libc::c_int as libc::c_longlong * d as libc::c_longlong
                        + t as libc::c_longlong) + 1 as libc::c_int as libc::c_longlong)
                    << (::std::mem::size_of::<libc::c_longlong>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                    - 1 as libc::c_int as libc::c_longlong)
                    * 2 as libc::c_int as libc::c_longlong
                    + 1 as libc::c_int as libc::c_longlong)
            } else {
                0 as libc::c_int as libc::c_longlong
                    * (0 as libc::c_int as libc::c_longlong * d as libc::c_longlong
                        + t as libc::c_longlong) + 0 as libc::c_int as libc::c_longlong
            }) < 0 as libc::c_int as libc::c_longlong
            {
                if (d as libc::c_longlong) < 0 as libc::c_int as libc::c_longlong {
                    ((t as libc::c_longlong)
                        < (if (0 as libc::c_int as libc::c_longlong
                            * (0 as libc::c_int as libc::c_longlong
                                * d as libc::c_longlong + t as libc::c_longlong)
                            - 1 as libc::c_int as libc::c_longlong)
                            < 0 as libc::c_int as libc::c_longlong
                        {
                            !((((0 as libc::c_int as libc::c_longlong
                                * (0 as libc::c_int as libc::c_longlong
                                    * d as libc::c_longlong + t as libc::c_longlong)
                                + 1 as libc::c_int as libc::c_longlong)
                                << (::std::mem::size_of::<libc::c_longlong>()
                                    as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                - 1 as libc::c_int as libc::c_longlong)
                                * 2 as libc::c_int as libc::c_longlong
                                + 1 as libc::c_int as libc::c_longlong)
                        } else {
                            0 as libc::c_int as libc::c_longlong
                                * (0 as libc::c_int as libc::c_longlong
                                    * d as libc::c_longlong + t as libc::c_longlong)
                                + 0 as libc::c_int as libc::c_longlong
                        }) - d as libc::c_longlong) as libc::c_int
                } else {
                    (((if (0 as libc::c_int as libc::c_longlong
                        * (0 as libc::c_int as libc::c_longlong * d as libc::c_longlong
                            + t as libc::c_longlong)
                        - 1 as libc::c_int as libc::c_longlong)
                        < 0 as libc::c_int as libc::c_longlong
                    {
                        (((0 as libc::c_int as libc::c_longlong
                            * (0 as libc::c_int as libc::c_longlong
                                * d as libc::c_longlong + t as libc::c_longlong)
                            + 1 as libc::c_int as libc::c_longlong)
                            << (::std::mem::size_of::<libc::c_longlong>()
                                as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                            - 1 as libc::c_int as libc::c_longlong)
                            * 2 as libc::c_int as libc::c_longlong
                            + 1 as libc::c_int as libc::c_longlong
                    } else {
                        0 as libc::c_int as libc::c_longlong
                            * (0 as libc::c_int as libc::c_longlong
                                * d as libc::c_longlong + t as libc::c_longlong)
                            - 1 as libc::c_int as libc::c_longlong
                    }) - d as libc::c_longlong) < t as libc::c_longlong) as libc::c_int
                }
            } else {
                if (t as libc::c_longlong) < 0 as libc::c_int as libc::c_longlong {
                    (d as libc::c_longlong
                        <= t as libc::c_longlong + d as libc::c_longlong) as libc::c_int
                } else {
                    if (d as libc::c_longlong) < 0 as libc::c_int as libc::c_longlong {
                        (t as libc::c_longlong
                            <= t as libc::c_longlong + d as libc::c_longlong)
                            as libc::c_int
                    } else {
                        ((t as libc::c_longlong + d as libc::c_longlong)
                            < d as libc::c_longlong) as libc::c_int
                    }
                }
            }) != 0
                || (0 as libc::c_int as libc::c_longlong
                    * (t as libc::c_longlong + d as libc::c_longlong)
                    - 1 as libc::c_int as libc::c_longlong)
                    < 0 as libc::c_int as libc::c_longlong
                    && (t as libc::c_longlong + d as libc::c_longlong)
                        < -(9223372036854775807 as libc::c_longlong)
                            - 1 as libc::c_longlong
                || (9223372036854775807 as libc::c_longlong)
                    < t as libc::c_longlong + d as libc::c_longlong
            {
                result = (t as libc::c_longlong as libc::c_ulonglong)
                    .wrapping_add(d as libc::c_longlong as libc::c_ulonglong)
                    as libc::c_longlong as long_int;
                1 as libc::c_int
            } else {
                result = (t as libc::c_longlong as libc::c_ulonglong)
                    .wrapping_add(d as libc::c_longlong as libc::c_ulonglong)
                    as libc::c_longlong as long_int;
                0 as libc::c_int
            }
        } else if (if (if (0 as libc::c_int as libc::c_long
            * (0 as libc::c_int as libc::c_long * d + t)
            - 1 as libc::c_int as libc::c_long) < 0 as libc::c_int as libc::c_long
        {
            !((((0 as libc::c_int as libc::c_long
                * (0 as libc::c_int as libc::c_long * d + t)
                + 1 as libc::c_int as libc::c_long)
                << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                - 1 as libc::c_int as libc::c_long) * 2 as libc::c_int as libc::c_long
                + 1 as libc::c_int as libc::c_long)
        } else {
            0 as libc::c_int as libc::c_long * (0 as libc::c_int as libc::c_long * d + t)
                + 0 as libc::c_int as libc::c_long
        }) < 0 as libc::c_int as libc::c_long
        {
            if d < 0 as libc::c_int as libc::c_long {
                (t
                    < (if (0 as libc::c_int as libc::c_long
                        * (0 as libc::c_int as libc::c_long * d + t)
                        - 1 as libc::c_int as libc::c_long)
                        < 0 as libc::c_int as libc::c_long
                    {
                        !((((0 as libc::c_int as libc::c_long
                            * (0 as libc::c_int as libc::c_long * d + t)
                            + 1 as libc::c_int as libc::c_long)
                            << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                            - 1 as libc::c_int as libc::c_long)
                            * 2 as libc::c_int as libc::c_long
                            + 1 as libc::c_int as libc::c_long)
                    } else {
                        0 as libc::c_int as libc::c_long
                            * (0 as libc::c_int as libc::c_long * d + t)
                            + 0 as libc::c_int as libc::c_long
                    }) - d) as libc::c_int
            } else {
                ((if (0 as libc::c_int as libc::c_long
                    * (0 as libc::c_int as libc::c_long * d + t)
                    - 1 as libc::c_int as libc::c_long)
                    < 0 as libc::c_int as libc::c_long
                {
                    (((0 as libc::c_int as libc::c_long
                        * (0 as libc::c_int as libc::c_long * d + t)
                        + 1 as libc::c_int as libc::c_long)
                        << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                        - 1 as libc::c_int as libc::c_long)
                        * 2 as libc::c_int as libc::c_long
                        + 1 as libc::c_int as libc::c_long
                } else {
                    0 as libc::c_int as libc::c_long
                        * (0 as libc::c_int as libc::c_long * d + t)
                        - 1 as libc::c_int as libc::c_long
                }) - d < t) as libc::c_int
            }
        } else {
            if t < 0 as libc::c_int as libc::c_long {
                (d <= t + d) as libc::c_int
            } else {
                if d < 0 as libc::c_int as libc::c_long {
                    (t <= t + d) as libc::c_int
                } else {
                    (t + d < d) as libc::c_int
                }
            }
        }) != 0
            || (0 as libc::c_int as libc::c_long * (t + d)
                - 1 as libc::c_int as libc::c_long) < 0 as libc::c_int as libc::c_long
                && ((t + d) as libc::c_longlong)
                    < -(9223372036854775807 as libc::c_longlong) - 1 as libc::c_longlong
            || (9223372036854775807 as libc::c_longlong) < (t + d) as libc::c_longlong
        {
            result = (t as libc::c_ulonglong).wrapping_add(d as libc::c_ulonglong)
                as libc::c_longlong as long_int;
            1 as libc::c_int
        } else {
            result = (t as libc::c_ulonglong).wrapping_add(d as libc::c_ulonglong)
                as libc::c_longlong as long_int;
            0 as libc::c_int
        } == 0
        {
            return result;
        }
    }
    return if t < long_int_avg(mktime_min, mktime_max) {
        if t <= mktime_min + 1 as libc::c_int as libc::c_long {
            t + 1 as libc::c_int as libc::c_long
        } else {
            mktime_min
        }
    } else if mktime_max - 1 as libc::c_int as libc::c_long <= t {
        t - 1 as libc::c_int as libc::c_long
    } else {
        mktime_max
    };
}
unsafe extern "C" fn convert_time(
    mut convert: Option::<unsafe extern "C" fn(*const time_t, *mut tm) -> *mut tm>,
    mut t: long_int,
    mut tm: *mut tm,
) -> *mut tm {
    let mut x: time_t = t;
    return convert.unwrap()(&mut x, tm);
}
unsafe extern "C" fn ranged_convert(
    mut convert: Option::<unsafe extern "C" fn(*const time_t, *mut tm) -> *mut tm>,
    mut t: *mut long_int,
    mut tp: *mut tm,
) -> *mut tm {
    let mut r: *mut tm = 0 as *mut tm;
    if *t < mktime_min {
        *t = mktime_min;
    } else if mktime_max < *t {
        *t = mktime_max;
    }
    r = convert_time(convert, *t, tp);
    if r.is_null() && *t != 0 {
        let mut bad: long_int = *t;
        let mut ok: long_int = 0 as libc::c_int as long_int;
        loop {
            let mut mid: long_int = long_int_avg(ok, bad);
            if mid != ok && mid != bad {
                break;
            }
            r = convert_time(convert, mid, tp);
            if !r.is_null() {
                ok = mid;
            } else {
                bad = mid;
            }
        }
        if r.is_null() && ok != 0 {
            r = convert_time(convert, ok, tp);
        }
    }
    return r;
}
pub unsafe extern "C" fn mktime_internal(
    mut tp: *mut tm,
    mut convert: Option::<unsafe extern "C" fn(*const time_t, *mut tm) -> *mut tm>,
    mut offset: *mut mktime_offset_t,
) -> time_t {
    let mut current_block: u64;
    let mut t: long_int = 0;
    let mut gt: long_int = 0;
    let mut t0: long_int = 0;
    let mut t1: long_int = 0;
    let mut t2: long_int = 0;
    let mut dt: long_int = 0;
    let mut tm: tm = tm {
        tm_sec: 0,
        tm_min: 0,
        tm_hour: 0,
        tm_mday: 0,
        tm_mon: 0,
        tm_year: 0,
        tm_wday: 0,
        tm_yday: 0,
        tm_isdst: 0,
        tm_gmtoff: 0,
        tm_zone: 0 as *const libc::c_char,
    };
    let mut remaining_probes: libc::c_int = 6 as libc::c_int;
    let mut sec: libc::c_int = (*tp).tm_sec;
    let mut min: libc::c_int = (*tp).tm_min;
    let mut hour: libc::c_int = (*tp).tm_hour;
    let mut mday: libc::c_int = (*tp).tm_mday;
    let mut mon: libc::c_int = (*tp).tm_mon;
    let mut year_requested: libc::c_int = (*tp).tm_year;
    let mut isdst: libc::c_int = (*tp).tm_isdst;
    let mut dst2: libc::c_int = 0;
    let mut mon_remainder: libc::c_int = mon % 12 as libc::c_int;
    let mut negative_mon_remainder: libc::c_int = (mon_remainder < 0 as libc::c_int)
        as libc::c_int;
    let mut mon_years: libc::c_int = mon / 12 as libc::c_int - negative_mon_remainder;
    let mut lyear_requested: long_int = year_requested as long_int;
    let mut year: long_int = lyear_requested + mon_years as libc::c_long;
    let mut mon_yday: libc::c_int = __mon_yday[leapyear(year)
        as usize][(mon_remainder + 12 as libc::c_int * negative_mon_remainder) as usize]
        as libc::c_int - 1 as libc::c_int;
    let mut lmday: long_int = mday as long_int;
    let mut yday: long_int = mon_yday as libc::c_long + lmday;
    let mut negative_offset_guess: libc::c_int = 0;
    let mut sec_requested: libc::c_int = sec;
    if sec < 0 as libc::c_int {
        sec = 0 as libc::c_int;
    }
    if (59 as libc::c_int) < sec {
        sec = 59 as libc::c_int;
    }
    if ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
        == ::std::mem::size_of::<libc::c_schar>() as libc::c_ulong
    {
        if (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
            < ::std::mem::size_of::<libc::c_schar>() as libc::c_ulong
        {
            if (if (if (0 as libc::c_int
                * (0 as libc::c_int * *offset as libc::c_schar as libc::c_int
                    + 0 as libc::c_int as libc::c_schar as libc::c_int)
                - 1 as libc::c_int) < 0 as libc::c_int
            {
                !((((0 as libc::c_int
                    * (0 as libc::c_int * *offset as libc::c_schar as libc::c_int
                        + 0 as libc::c_int as libc::c_schar as libc::c_int)
                    + 1 as libc::c_int)
                    << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                    - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
            } else {
                0 as libc::c_int
                    * (0 as libc::c_int * *offset as libc::c_schar as libc::c_int
                        + 0 as libc::c_int as libc::c_schar as libc::c_int)
                    + 0 as libc::c_int
            }) < 0 as libc::c_int
            {
                if (*offset as libc::c_schar as libc::c_int) < 0 as libc::c_int {
                    (((if (0 as libc::c_int
                        * (0 as libc::c_int * *offset as libc::c_schar as libc::c_int
                            + 0 as libc::c_int as libc::c_schar as libc::c_int)
                        - 1 as libc::c_int) < 0 as libc::c_int
                    {
                        (((0 as libc::c_int
                            * (0 as libc::c_int * *offset as libc::c_schar as libc::c_int
                                + 0 as libc::c_int as libc::c_schar as libc::c_int)
                            + 1 as libc::c_int)
                            << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                            - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                    } else {
                        0 as libc::c_int
                            * (0 as libc::c_int * *offset as libc::c_schar as libc::c_int
                                + 0 as libc::c_int as libc::c_schar as libc::c_int)
                            - 1 as libc::c_int
                    }) + *offset as libc::c_schar as libc::c_int)
                        < 0 as libc::c_int as libc::c_schar as libc::c_int)
                        as libc::c_int
                } else {
                    ((0 as libc::c_int as libc::c_schar as libc::c_int)
                        < (if (0 as libc::c_int
                            * (0 as libc::c_int * *offset as libc::c_schar as libc::c_int
                                + 0 as libc::c_int as libc::c_schar as libc::c_int)
                            - 1 as libc::c_int) < 0 as libc::c_int
                        {
                            !((((0 as libc::c_int
                                * (0 as libc::c_int
                                    * *offset as libc::c_schar as libc::c_int
                                    + 0 as libc::c_int as libc::c_schar as libc::c_int)
                                + 1 as libc::c_int)
                                << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                        } else {
                            0 as libc::c_int
                                * (0 as libc::c_int
                                    * *offset as libc::c_schar as libc::c_int
                                    + 0 as libc::c_int as libc::c_schar as libc::c_int)
                                + 0 as libc::c_int
                        }) + *offset as libc::c_schar as libc::c_int) as libc::c_int
                }
            } else {
                if (0 as libc::c_int as libc::c_schar as libc::c_int) < 0 as libc::c_int
                {
                    1 as libc::c_int
                } else {
                    if (*offset as libc::c_schar as libc::c_int) < 0 as libc::c_int {
                        (0 as libc::c_int as libc::c_schar as libc::c_int
                            - *offset as libc::c_schar as libc::c_int
                            <= 0 as libc::c_int as libc::c_schar as libc::c_int)
                            as libc::c_int
                    } else {
                        ((0 as libc::c_int as libc::c_schar as libc::c_int)
                            < *offset as libc::c_schar as libc::c_int) as libc::c_int
                    }
                }
            }) != 0
                || (0 as libc::c_int
                    * (0 as libc::c_int as libc::c_schar as libc::c_int
                        - *offset as libc::c_schar as libc::c_int) - 1 as libc::c_int)
                    < 0 as libc::c_int
                    && (0 as libc::c_int as libc::c_schar as libc::c_int
                        - *offset as libc::c_schar as libc::c_int)
                        < -(127 as libc::c_int) - 1 as libc::c_int
                || (127 as libc::c_int)
                    < 0 as libc::c_int as libc::c_schar as libc::c_int
                        - *offset as libc::c_schar as libc::c_int
            {
                negative_offset_guess = (0 as libc::c_int as libc::c_schar
                    as libc::c_uint)
                    .wrapping_sub(*offset as libc::c_schar as libc::c_uint)
                    as libc::c_schar as libc::c_int;
            } else {
                negative_offset_guess = (0 as libc::c_int as libc::c_schar
                    as libc::c_uint)
                    .wrapping_sub(*offset as libc::c_schar as libc::c_uint)
                    as libc::c_schar as libc::c_int;
            };
        } else {
            if (if (if (0 as libc::c_int as libc::c_long
                * (0 as libc::c_int as libc::c_long * *offset
                    + 0 as libc::c_int as libc::c_long)
                - 1 as libc::c_int as libc::c_long) < 0 as libc::c_int as libc::c_long
            {
                !((((0 as libc::c_int as libc::c_long
                    * (0 as libc::c_int as libc::c_long * *offset
                        + 0 as libc::c_int as libc::c_long)
                    + 1 as libc::c_int as libc::c_long)
                    << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                    - 1 as libc::c_int as libc::c_long)
                    * 2 as libc::c_int as libc::c_long
                    + 1 as libc::c_int as libc::c_long)
            } else {
                0 as libc::c_int as libc::c_long
                    * (0 as libc::c_int as libc::c_long * *offset
                        + 0 as libc::c_int as libc::c_long)
                    + 0 as libc::c_int as libc::c_long
            }) < 0 as libc::c_int as libc::c_long
            {
                if *offset < 0 as libc::c_int as libc::c_long {
                    ((if (0 as libc::c_int as libc::c_long
                        * (0 as libc::c_int as libc::c_long * *offset
                            + 0 as libc::c_int as libc::c_long)
                        - 1 as libc::c_int as libc::c_long)
                        < 0 as libc::c_int as libc::c_long
                    {
                        (((0 as libc::c_int as libc::c_long
                            * (0 as libc::c_int as libc::c_long * *offset
                                + 0 as libc::c_int as libc::c_long)
                            + 1 as libc::c_int as libc::c_long)
                            << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                            - 1 as libc::c_int as libc::c_long)
                            * 2 as libc::c_int as libc::c_long
                            + 1 as libc::c_int as libc::c_long
                    } else {
                        0 as libc::c_int as libc::c_long
                            * (0 as libc::c_int as libc::c_long * *offset
                                + 0 as libc::c_int as libc::c_long)
                            - 1 as libc::c_int as libc::c_long
                    }) + *offset < 0 as libc::c_int as libc::c_long) as libc::c_int
                } else {
                    ((0 as libc::c_int as libc::c_long)
                        < (if (0 as libc::c_int as libc::c_long
                            * (0 as libc::c_int as libc::c_long * *offset
                                + 0 as libc::c_int as libc::c_long)
                            - 1 as libc::c_int as libc::c_long)
                            < 0 as libc::c_int as libc::c_long
                        {
                            !((((0 as libc::c_int as libc::c_long
                                * (0 as libc::c_int as libc::c_long * *offset
                                    + 0 as libc::c_int as libc::c_long)
                                + 1 as libc::c_int as libc::c_long)
                                << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                - 1 as libc::c_int as libc::c_long)
                                * 2 as libc::c_int as libc::c_long
                                + 1 as libc::c_int as libc::c_long)
                        } else {
                            0 as libc::c_int as libc::c_long
                                * (0 as libc::c_int as libc::c_long * *offset
                                    + 0 as libc::c_int as libc::c_long)
                                + 0 as libc::c_int as libc::c_long
                        }) + *offset) as libc::c_int
                }
            } else {
                if (0 as libc::c_int) < 0 as libc::c_int {
                    1 as libc::c_int
                } else {
                    if *offset < 0 as libc::c_int as libc::c_long {
                        (0 as libc::c_int as libc::c_long - *offset
                            <= 0 as libc::c_int as libc::c_long) as libc::c_int
                    } else {
                        ((0 as libc::c_int as libc::c_long) < *offset) as libc::c_int
                    }
                }
            }) != 0
                || (0 as libc::c_int as libc::c_long
                    * (0 as libc::c_int as libc::c_long - *offset)
                    - 1 as libc::c_int as libc::c_long)
                    < 0 as libc::c_int as libc::c_long
                    && 0 as libc::c_int as libc::c_long - *offset
                        < (-(127 as libc::c_int) - 1 as libc::c_int) as libc::c_long
                || (127 as libc::c_int as libc::c_long)
                    < 0 as libc::c_int as libc::c_long - *offset
            {
                negative_offset_guess = (0 as libc::c_int as libc::c_uint)
                    .wrapping_sub(*offset as libc::c_uint) as libc::c_schar
                    as libc::c_int;
            } else {
                negative_offset_guess = (0 as libc::c_int as libc::c_uint)
                    .wrapping_sub(*offset as libc::c_uint) as libc::c_schar
                    as libc::c_int;
            };
        };
    } else {
        if ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
            == ::std::mem::size_of::<libc::c_short>() as libc::c_ulong
        {
            if (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                < ::std::mem::size_of::<libc::c_short>() as libc::c_ulong
            {
                if (if (if (0 as libc::c_int
                    * (0 as libc::c_int * *offset as libc::c_short as libc::c_int
                        + 0 as libc::c_int as libc::c_short as libc::c_int)
                    - 1 as libc::c_int) < 0 as libc::c_int
                {
                    !((((0 as libc::c_int
                        * (0 as libc::c_int * *offset as libc::c_short as libc::c_int
                            + 0 as libc::c_int as libc::c_short as libc::c_int)
                        + 1 as libc::c_int)
                        << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                        - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                } else {
                    0 as libc::c_int
                        * (0 as libc::c_int * *offset as libc::c_short as libc::c_int
                            + 0 as libc::c_int as libc::c_short as libc::c_int)
                        + 0 as libc::c_int
                }) < 0 as libc::c_int
                {
                    if (*offset as libc::c_short as libc::c_int) < 0 as libc::c_int {
                        (((if (0 as libc::c_int
                            * (0 as libc::c_int * *offset as libc::c_short as libc::c_int
                                + 0 as libc::c_int as libc::c_short as libc::c_int)
                            - 1 as libc::c_int) < 0 as libc::c_int
                        {
                            (((0 as libc::c_int
                                * (0 as libc::c_int
                                    * *offset as libc::c_short as libc::c_int
                                    + 0 as libc::c_int as libc::c_short as libc::c_int)
                                + 1 as libc::c_int)
                                << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                        } else {
                            0 as libc::c_int
                                * (0 as libc::c_int
                                    * *offset as libc::c_short as libc::c_int
                                    + 0 as libc::c_int as libc::c_short as libc::c_int)
                                - 1 as libc::c_int
                        }) + *offset as libc::c_short as libc::c_int)
                            < 0 as libc::c_int as libc::c_short as libc::c_int)
                            as libc::c_int
                    } else {
                        ((0 as libc::c_int as libc::c_short as libc::c_int)
                            < (if (0 as libc::c_int
                                * (0 as libc::c_int
                                    * *offset as libc::c_short as libc::c_int
                                    + 0 as libc::c_int as libc::c_short as libc::c_int)
                                - 1 as libc::c_int) < 0 as libc::c_int
                            {
                                !((((0 as libc::c_int
                                    * (0 as libc::c_int
                                        * *offset as libc::c_short as libc::c_int
                                        + 0 as libc::c_int as libc::c_short as libc::c_int)
                                    + 1 as libc::c_int)
                                    << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                    - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                            } else {
                                0 as libc::c_int
                                    * (0 as libc::c_int
                                        * *offset as libc::c_short as libc::c_int
                                        + 0 as libc::c_int as libc::c_short as libc::c_int)
                                    + 0 as libc::c_int
                            }) + *offset as libc::c_short as libc::c_int) as libc::c_int
                    }
                } else {
                    if (0 as libc::c_int as libc::c_short as libc::c_int)
                        < 0 as libc::c_int
                    {
                        1 as libc::c_int
                    } else {
                        if (*offset as libc::c_short as libc::c_int) < 0 as libc::c_int
                        {
                            (0 as libc::c_int as libc::c_short as libc::c_int
                                - *offset as libc::c_short as libc::c_int
                                <= 0 as libc::c_int as libc::c_short as libc::c_int)
                                as libc::c_int
                        } else {
                            ((0 as libc::c_int as libc::c_short as libc::c_int)
                                < *offset as libc::c_short as libc::c_int) as libc::c_int
                        }
                    }
                }) != 0
                    || (0 as libc::c_int
                        * (0 as libc::c_int as libc::c_short as libc::c_int
                            - *offset as libc::c_short as libc::c_int)
                        - 1 as libc::c_int) < 0 as libc::c_int
                        && (0 as libc::c_int as libc::c_short as libc::c_int
                            - *offset as libc::c_short as libc::c_int)
                            < -(32767 as libc::c_int) - 1 as libc::c_int
                    || (32767 as libc::c_int)
                        < 0 as libc::c_int as libc::c_short as libc::c_int
                            - *offset as libc::c_short as libc::c_int
                {
                    negative_offset_guess = (0 as libc::c_int as libc::c_short
                        as libc::c_uint)
                        .wrapping_sub(*offset as libc::c_short as libc::c_uint)
                        as libc::c_short as libc::c_int;
                } else {
                    negative_offset_guess = (0 as libc::c_int as libc::c_short
                        as libc::c_uint)
                        .wrapping_sub(*offset as libc::c_short as libc::c_uint)
                        as libc::c_short as libc::c_int;
                };
            } else {
                if (if (if (0 as libc::c_int as libc::c_long
                    * (0 as libc::c_int as libc::c_long * *offset
                        + 0 as libc::c_int as libc::c_long)
                    - 1 as libc::c_int as libc::c_long)
                    < 0 as libc::c_int as libc::c_long
                {
                    !((((0 as libc::c_int as libc::c_long
                        * (0 as libc::c_int as libc::c_long * *offset
                            + 0 as libc::c_int as libc::c_long)
                        + 1 as libc::c_int as libc::c_long)
                        << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                        - 1 as libc::c_int as libc::c_long)
                        * 2 as libc::c_int as libc::c_long
                        + 1 as libc::c_int as libc::c_long)
                } else {
                    0 as libc::c_int as libc::c_long
                        * (0 as libc::c_int as libc::c_long * *offset
                            + 0 as libc::c_int as libc::c_long)
                        + 0 as libc::c_int as libc::c_long
                }) < 0 as libc::c_int as libc::c_long
                {
                    if *offset < 0 as libc::c_int as libc::c_long {
                        ((if (0 as libc::c_int as libc::c_long
                            * (0 as libc::c_int as libc::c_long * *offset
                                + 0 as libc::c_int as libc::c_long)
                            - 1 as libc::c_int as libc::c_long)
                            < 0 as libc::c_int as libc::c_long
                        {
                            (((0 as libc::c_int as libc::c_long
                                * (0 as libc::c_int as libc::c_long * *offset
                                    + 0 as libc::c_int as libc::c_long)
                                + 1 as libc::c_int as libc::c_long)
                                << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                - 1 as libc::c_int as libc::c_long)
                                * 2 as libc::c_int as libc::c_long
                                + 1 as libc::c_int as libc::c_long
                        } else {
                            0 as libc::c_int as libc::c_long
                                * (0 as libc::c_int as libc::c_long * *offset
                                    + 0 as libc::c_int as libc::c_long)
                                - 1 as libc::c_int as libc::c_long
                        }) + *offset < 0 as libc::c_int as libc::c_long) as libc::c_int
                    } else {
                        ((0 as libc::c_int as libc::c_long)
                            < (if (0 as libc::c_int as libc::c_long
                                * (0 as libc::c_int as libc::c_long * *offset
                                    + 0 as libc::c_int as libc::c_long)
                                - 1 as libc::c_int as libc::c_long)
                                < 0 as libc::c_int as libc::c_long
                            {
                                !((((0 as libc::c_int as libc::c_long
                                    * (0 as libc::c_int as libc::c_long * *offset
                                        + 0 as libc::c_int as libc::c_long)
                                    + 1 as libc::c_int as libc::c_long)
                                    << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                    - 1 as libc::c_int as libc::c_long)
                                    * 2 as libc::c_int as libc::c_long
                                    + 1 as libc::c_int as libc::c_long)
                            } else {
                                0 as libc::c_int as libc::c_long
                                    * (0 as libc::c_int as libc::c_long * *offset
                                        + 0 as libc::c_int as libc::c_long)
                                    + 0 as libc::c_int as libc::c_long
                            }) + *offset) as libc::c_int
                    }
                } else {
                    if (0 as libc::c_int) < 0 as libc::c_int {
                        1 as libc::c_int
                    } else {
                        if *offset < 0 as libc::c_int as libc::c_long {
                            (0 as libc::c_int as libc::c_long - *offset
                                <= 0 as libc::c_int as libc::c_long) as libc::c_int
                        } else {
                            ((0 as libc::c_int as libc::c_long) < *offset) as libc::c_int
                        }
                    }
                }) != 0
                    || (0 as libc::c_int as libc::c_long
                        * (0 as libc::c_int as libc::c_long - *offset)
                        - 1 as libc::c_int as libc::c_long)
                        < 0 as libc::c_int as libc::c_long
                        && 0 as libc::c_int as libc::c_long - *offset
                            < (-(32767 as libc::c_int) - 1 as libc::c_int)
                                as libc::c_long
                    || (32767 as libc::c_int as libc::c_long)
                        < 0 as libc::c_int as libc::c_long - *offset
                {
                    negative_offset_guess = (0 as libc::c_int as libc::c_uint)
                        .wrapping_sub(*offset as libc::c_uint) as libc::c_short
                        as libc::c_int;
                } else {
                    negative_offset_guess = (0 as libc::c_int as libc::c_uint)
                        .wrapping_sub(*offset as libc::c_uint) as libc::c_short
                        as libc::c_int;
                };
            };
        } else {
            if ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                == ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
            {
                if (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                    < ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                {
                    if (if (if (0 as libc::c_int
                        * (0 as libc::c_int * *offset as libc::c_int + 0 as libc::c_int)
                        - 1 as libc::c_int) < 0 as libc::c_int
                    {
                        !((((0 as libc::c_int
                            * (0 as libc::c_int * *offset as libc::c_int
                                + 0 as libc::c_int) + 1 as libc::c_int)
                            << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                            - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                    } else {
                        0 as libc::c_int
                            * (0 as libc::c_int * *offset as libc::c_int
                                + 0 as libc::c_int) + 0 as libc::c_int
                    }) < 0 as libc::c_int
                    {
                        if (*offset as libc::c_int) < 0 as libc::c_int {
                            (((if (0 as libc::c_int
                                * (0 as libc::c_int * *offset as libc::c_int
                                    + 0 as libc::c_int) - 1 as libc::c_int) < 0 as libc::c_int
                            {
                                (((0 as libc::c_int
                                    * (0 as libc::c_int * *offset as libc::c_int
                                        + 0 as libc::c_int) + 1 as libc::c_int)
                                    << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                    - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                            } else {
                                0 as libc::c_int
                                    * (0 as libc::c_int * *offset as libc::c_int
                                        + 0 as libc::c_int) - 1 as libc::c_int
                            }) + *offset as libc::c_int) < 0 as libc::c_int)
                                as libc::c_int
                        } else {
                            ((0 as libc::c_int)
                                < (if (0 as libc::c_int
                                    * (0 as libc::c_int * *offset as libc::c_int
                                        + 0 as libc::c_int) - 1 as libc::c_int) < 0 as libc::c_int
                                {
                                    !((((0 as libc::c_int
                                        * (0 as libc::c_int * *offset as libc::c_int
                                            + 0 as libc::c_int) + 1 as libc::c_int)
                                        << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                        - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                                } else {
                                    0 as libc::c_int
                                        * (0 as libc::c_int * *offset as libc::c_int
                                            + 0 as libc::c_int) + 0 as libc::c_int
                                }) + *offset as libc::c_int) as libc::c_int
                        }
                    } else {
                        if (0 as libc::c_int) < 0 as libc::c_int {
                            1 as libc::c_int
                        } else {
                            if (*offset as libc::c_int) < 0 as libc::c_int {
                                (0 as libc::c_int - *offset as libc::c_int
                                    <= 0 as libc::c_int) as libc::c_int
                            } else {
                                ((0 as libc::c_int) < *offset as libc::c_int) as libc::c_int
                            }
                        }
                    }) != 0
                        || (0 as libc::c_int
                            * (0 as libc::c_int - *offset as libc::c_int)
                            - 1 as libc::c_int) < 0 as libc::c_int
                            && (0 as libc::c_int - *offset as libc::c_int)
                                < -(2147483647 as libc::c_int) - 1 as libc::c_int
                        || (2147483647 as libc::c_int)
                            < 0 as libc::c_int - *offset as libc::c_int
                    {
                        negative_offset_guess = (0 as libc::c_int as libc::c_uint)
                            .wrapping_sub(*offset as libc::c_int as libc::c_uint)
                            as libc::c_int;
                    } else {
                        negative_offset_guess = (0 as libc::c_int as libc::c_uint)
                            .wrapping_sub(*offset as libc::c_int as libc::c_uint)
                            as libc::c_int;
                    };
                } else {
                    if (if (if (0 as libc::c_int as libc::c_long
                        * (0 as libc::c_int as libc::c_long * *offset
                            + 0 as libc::c_int as libc::c_long)
                        - 1 as libc::c_int as libc::c_long)
                        < 0 as libc::c_int as libc::c_long
                    {
                        !((((0 as libc::c_int as libc::c_long
                            * (0 as libc::c_int as libc::c_long * *offset
                                + 0 as libc::c_int as libc::c_long)
                            + 1 as libc::c_int as libc::c_long)
                            << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                            - 1 as libc::c_int as libc::c_long)
                            * 2 as libc::c_int as libc::c_long
                            + 1 as libc::c_int as libc::c_long)
                    } else {
                        0 as libc::c_int as libc::c_long
                            * (0 as libc::c_int as libc::c_long * *offset
                                + 0 as libc::c_int as libc::c_long)
                            + 0 as libc::c_int as libc::c_long
                    }) < 0 as libc::c_int as libc::c_long
                    {
                        if *offset < 0 as libc::c_int as libc::c_long {
                            ((if (0 as libc::c_int as libc::c_long
                                * (0 as libc::c_int as libc::c_long * *offset
                                    + 0 as libc::c_int as libc::c_long)
                                - 1 as libc::c_int as libc::c_long)
                                < 0 as libc::c_int as libc::c_long
                            {
                                (((0 as libc::c_int as libc::c_long
                                    * (0 as libc::c_int as libc::c_long * *offset
                                        + 0 as libc::c_int as libc::c_long)
                                    + 1 as libc::c_int as libc::c_long)
                                    << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                    - 1 as libc::c_int as libc::c_long)
                                    * 2 as libc::c_int as libc::c_long
                                    + 1 as libc::c_int as libc::c_long
                            } else {
                                0 as libc::c_int as libc::c_long
                                    * (0 as libc::c_int as libc::c_long * *offset
                                        + 0 as libc::c_int as libc::c_long)
                                    - 1 as libc::c_int as libc::c_long
                            }) + *offset < 0 as libc::c_int as libc::c_long)
                                as libc::c_int
                        } else {
                            ((0 as libc::c_int as libc::c_long)
                                < (if (0 as libc::c_int as libc::c_long
                                    * (0 as libc::c_int as libc::c_long * *offset
                                        + 0 as libc::c_int as libc::c_long)
                                    - 1 as libc::c_int as libc::c_long)
                                    < 0 as libc::c_int as libc::c_long
                                {
                                    !((((0 as libc::c_int as libc::c_long
                                        * (0 as libc::c_int as libc::c_long * *offset
                                            + 0 as libc::c_int as libc::c_long)
                                        + 1 as libc::c_int as libc::c_long)
                                        << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                        - 1 as libc::c_int as libc::c_long)
                                        * 2 as libc::c_int as libc::c_long
                                        + 1 as libc::c_int as libc::c_long)
                                } else {
                                    0 as libc::c_int as libc::c_long
                                        * (0 as libc::c_int as libc::c_long * *offset
                                            + 0 as libc::c_int as libc::c_long)
                                        + 0 as libc::c_int as libc::c_long
                                }) + *offset) as libc::c_int
                        }
                    } else {
                        if (0 as libc::c_int) < 0 as libc::c_int {
                            1 as libc::c_int
                        } else {
                            if *offset < 0 as libc::c_int as libc::c_long {
                                (0 as libc::c_int as libc::c_long - *offset
                                    <= 0 as libc::c_int as libc::c_long) as libc::c_int
                            } else {
                                ((0 as libc::c_int as libc::c_long) < *offset)
                                    as libc::c_int
                            }
                        }
                    }) != 0
                        || (0 as libc::c_int as libc::c_long
                            * (0 as libc::c_int as libc::c_long - *offset)
                            - 1 as libc::c_int as libc::c_long)
                            < 0 as libc::c_int as libc::c_long
                            && 0 as libc::c_int as libc::c_long - *offset
                                < (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                    as libc::c_long
                        || (2147483647 as libc::c_int as libc::c_long)
                            < 0 as libc::c_int as libc::c_long - *offset
                    {
                        negative_offset_guess = (0 as libc::c_int as libc::c_uint)
                            .wrapping_sub(*offset as libc::c_uint) as libc::c_int;
                    } else {
                        negative_offset_guess = (0 as libc::c_int as libc::c_uint)
                            .wrapping_sub(*offset as libc::c_uint) as libc::c_int;
                    };
                };
            } else {
                if ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                    == ::std::mem::size_of::<libc::c_long>() as libc::c_ulong
                {
                    if (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                        < ::std::mem::size_of::<libc::c_long>() as libc::c_ulong
                    {
                        if (if (if (0 as libc::c_int as libc::c_long
                            * (0 as libc::c_int as libc::c_long * *offset
                                + 0 as libc::c_int as libc::c_long)
                            - 1 as libc::c_int as libc::c_long)
                            < 0 as libc::c_int as libc::c_long
                        {
                            !((((0 as libc::c_int as libc::c_long
                                * (0 as libc::c_int as libc::c_long * *offset
                                    + 0 as libc::c_int as libc::c_long)
                                + 1 as libc::c_int as libc::c_long)
                                << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                - 1 as libc::c_int as libc::c_long)
                                * 2 as libc::c_int as libc::c_long
                                + 1 as libc::c_int as libc::c_long)
                        } else {
                            0 as libc::c_int as libc::c_long
                                * (0 as libc::c_int as libc::c_long * *offset
                                    + 0 as libc::c_int as libc::c_long)
                                + 0 as libc::c_int as libc::c_long
                        }) < 0 as libc::c_int as libc::c_long
                        {
                            if *offset < 0 as libc::c_int as libc::c_long {
                                ((if (0 as libc::c_int as libc::c_long
                                    * (0 as libc::c_int as libc::c_long * *offset
                                        + 0 as libc::c_int as libc::c_long)
                                    - 1 as libc::c_int as libc::c_long)
                                    < 0 as libc::c_int as libc::c_long
                                {
                                    (((0 as libc::c_int as libc::c_long
                                        * (0 as libc::c_int as libc::c_long * *offset
                                            + 0 as libc::c_int as libc::c_long)
                                        + 1 as libc::c_int as libc::c_long)
                                        << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                        - 1 as libc::c_int as libc::c_long)
                                        * 2 as libc::c_int as libc::c_long
                                        + 1 as libc::c_int as libc::c_long
                                } else {
                                    0 as libc::c_int as libc::c_long
                                        * (0 as libc::c_int as libc::c_long * *offset
                                            + 0 as libc::c_int as libc::c_long)
                                        - 1 as libc::c_int as libc::c_long
                                }) + *offset < 0 as libc::c_int as libc::c_long)
                                    as libc::c_int
                            } else {
                                ((0 as libc::c_int as libc::c_long)
                                    < (if (0 as libc::c_int as libc::c_long
                                        * (0 as libc::c_int as libc::c_long * *offset
                                            + 0 as libc::c_int as libc::c_long)
                                        - 1 as libc::c_int as libc::c_long)
                                        < 0 as libc::c_int as libc::c_long
                                    {
                                        !((((0 as libc::c_int as libc::c_long
                                            * (0 as libc::c_int as libc::c_long * *offset
                                                + 0 as libc::c_int as libc::c_long)
                                            + 1 as libc::c_int as libc::c_long)
                                            << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                            - 1 as libc::c_int as libc::c_long)
                                            * 2 as libc::c_int as libc::c_long
                                            + 1 as libc::c_int as libc::c_long)
                                    } else {
                                        0 as libc::c_int as libc::c_long
                                            * (0 as libc::c_int as libc::c_long * *offset
                                                + 0 as libc::c_int as libc::c_long)
                                            + 0 as libc::c_int as libc::c_long
                                    }) + *offset) as libc::c_int
                            }
                        } else {
                            if (0 as libc::c_int as libc::c_long)
                                < 0 as libc::c_int as libc::c_long
                            {
                                1 as libc::c_int
                            } else {
                                if *offset < 0 as libc::c_int as libc::c_long {
                                    (0 as libc::c_int as libc::c_long - *offset
                                        <= 0 as libc::c_int as libc::c_long) as libc::c_int
                                } else {
                                    ((0 as libc::c_int as libc::c_long) < *offset)
                                        as libc::c_int
                                }
                            }
                        }) != 0
                            || (0 as libc::c_int as libc::c_long
                                * (0 as libc::c_int as libc::c_long - *offset)
                                - 1 as libc::c_int as libc::c_long)
                                < 0 as libc::c_int as libc::c_long
                                && 0 as libc::c_int as libc::c_long - *offset
                                    < -(9223372036854775807 as libc::c_long) - 1 as libc::c_long
                            || (9223372036854775807 as libc::c_long)
                                < 0 as libc::c_int as libc::c_long - *offset
                        {
                            negative_offset_guess = (0 as libc::c_int as libc::c_long
                                as libc::c_ulong)
                                .wrapping_sub(*offset as libc::c_ulong) as libc::c_long
                                as libc::c_int;
                        } else {
                            negative_offset_guess = (0 as libc::c_int as libc::c_long
                                as libc::c_ulong)
                                .wrapping_sub(*offset as libc::c_ulong) as libc::c_long
                                as libc::c_int;
                        };
                    } else {
                        if (if (if (0 as libc::c_int as libc::c_long
                            * (0 as libc::c_int as libc::c_long * *offset
                                + 0 as libc::c_int as libc::c_long)
                            - 1 as libc::c_int as libc::c_long)
                            < 0 as libc::c_int as libc::c_long
                        {
                            !((((0 as libc::c_int as libc::c_long
                                * (0 as libc::c_int as libc::c_long * *offset
                                    + 0 as libc::c_int as libc::c_long)
                                + 1 as libc::c_int as libc::c_long)
                                << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                - 1 as libc::c_int as libc::c_long)
                                * 2 as libc::c_int as libc::c_long
                                + 1 as libc::c_int as libc::c_long)
                        } else {
                            0 as libc::c_int as libc::c_long
                                * (0 as libc::c_int as libc::c_long * *offset
                                    + 0 as libc::c_int as libc::c_long)
                                + 0 as libc::c_int as libc::c_long
                        }) < 0 as libc::c_int as libc::c_long
                        {
                            if *offset < 0 as libc::c_int as libc::c_long {
                                ((if (0 as libc::c_int as libc::c_long
                                    * (0 as libc::c_int as libc::c_long * *offset
                                        + 0 as libc::c_int as libc::c_long)
                                    - 1 as libc::c_int as libc::c_long)
                                    < 0 as libc::c_int as libc::c_long
                                {
                                    (((0 as libc::c_int as libc::c_long
                                        * (0 as libc::c_int as libc::c_long * *offset
                                            + 0 as libc::c_int as libc::c_long)
                                        + 1 as libc::c_int as libc::c_long)
                                        << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                        - 1 as libc::c_int as libc::c_long)
                                        * 2 as libc::c_int as libc::c_long
                                        + 1 as libc::c_int as libc::c_long
                                } else {
                                    0 as libc::c_int as libc::c_long
                                        * (0 as libc::c_int as libc::c_long * *offset
                                            + 0 as libc::c_int as libc::c_long)
                                        - 1 as libc::c_int as libc::c_long
                                }) + *offset < 0 as libc::c_int as libc::c_long)
                                    as libc::c_int
                            } else {
                                ((0 as libc::c_int as libc::c_long)
                                    < (if (0 as libc::c_int as libc::c_long
                                        * (0 as libc::c_int as libc::c_long * *offset
                                            + 0 as libc::c_int as libc::c_long)
                                        - 1 as libc::c_int as libc::c_long)
                                        < 0 as libc::c_int as libc::c_long
                                    {
                                        !((((0 as libc::c_int as libc::c_long
                                            * (0 as libc::c_int as libc::c_long * *offset
                                                + 0 as libc::c_int as libc::c_long)
                                            + 1 as libc::c_int as libc::c_long)
                                            << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                            - 1 as libc::c_int as libc::c_long)
                                            * 2 as libc::c_int as libc::c_long
                                            + 1 as libc::c_int as libc::c_long)
                                    } else {
                                        0 as libc::c_int as libc::c_long
                                            * (0 as libc::c_int as libc::c_long * *offset
                                                + 0 as libc::c_int as libc::c_long)
                                            + 0 as libc::c_int as libc::c_long
                                    }) + *offset) as libc::c_int
                            }
                        } else {
                            if (0 as libc::c_int) < 0 as libc::c_int {
                                1 as libc::c_int
                            } else {
                                if *offset < 0 as libc::c_int as libc::c_long {
                                    (0 as libc::c_int as libc::c_long - *offset
                                        <= 0 as libc::c_int as libc::c_long) as libc::c_int
                                } else {
                                    ((0 as libc::c_int as libc::c_long) < *offset)
                                        as libc::c_int
                                }
                            }
                        }) != 0
                            || (0 as libc::c_int as libc::c_long
                                * (0 as libc::c_int as libc::c_long - *offset)
                                - 1 as libc::c_int as libc::c_long)
                                < 0 as libc::c_int as libc::c_long
                                && 0 as libc::c_int as libc::c_long - *offset
                                    < -(9223372036854775807 as libc::c_long) - 1 as libc::c_long
                            || (9223372036854775807 as libc::c_long)
                                < 0 as libc::c_int as libc::c_long - *offset
                        {
                            negative_offset_guess = (0 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(*offset as libc::c_ulong) as libc::c_long
                                as libc::c_int;
                        } else {
                            negative_offset_guess = (0 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(*offset as libc::c_ulong) as libc::c_long
                                as libc::c_int;
                        };
                    };
                } else {
                    if (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                        < ::std::mem::size_of::<libc::c_longlong>() as libc::c_ulong
                    {
                        if (if (if (0 as libc::c_int as libc::c_longlong
                            * (0 as libc::c_int as libc::c_longlong
                                * *offset as libc::c_longlong
                                + 0 as libc::c_int as libc::c_longlong)
                            - 1 as libc::c_int as libc::c_longlong)
                            < 0 as libc::c_int as libc::c_longlong
                        {
                            !((((0 as libc::c_int as libc::c_longlong
                                * (0 as libc::c_int as libc::c_longlong
                                    * *offset as libc::c_longlong
                                    + 0 as libc::c_int as libc::c_longlong)
                                + 1 as libc::c_int as libc::c_longlong)
                                << (::std::mem::size_of::<libc::c_longlong>()
                                    as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                - 1 as libc::c_int as libc::c_longlong)
                                * 2 as libc::c_int as libc::c_longlong
                                + 1 as libc::c_int as libc::c_longlong)
                        } else {
                            0 as libc::c_int as libc::c_longlong
                                * (0 as libc::c_int as libc::c_longlong
                                    * *offset as libc::c_longlong
                                    + 0 as libc::c_int as libc::c_longlong)
                                + 0 as libc::c_int as libc::c_longlong
                        }) < 0 as libc::c_int as libc::c_longlong
                        {
                            if (*offset as libc::c_longlong)
                                < 0 as libc::c_int as libc::c_longlong
                            {
                                (((if (0 as libc::c_int as libc::c_longlong
                                    * (0 as libc::c_int as libc::c_longlong
                                        * *offset as libc::c_longlong
                                        + 0 as libc::c_int as libc::c_longlong)
                                    - 1 as libc::c_int as libc::c_longlong)
                                    < 0 as libc::c_int as libc::c_longlong
                                {
                                    (((0 as libc::c_int as libc::c_longlong
                                        * (0 as libc::c_int as libc::c_longlong
                                            * *offset as libc::c_longlong
                                            + 0 as libc::c_int as libc::c_longlong)
                                        + 1 as libc::c_int as libc::c_longlong)
                                        << (::std::mem::size_of::<libc::c_longlong>()
                                            as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                        - 1 as libc::c_int as libc::c_longlong)
                                        * 2 as libc::c_int as libc::c_longlong
                                        + 1 as libc::c_int as libc::c_longlong
                                } else {
                                    0 as libc::c_int as libc::c_longlong
                                        * (0 as libc::c_int as libc::c_longlong
                                            * *offset as libc::c_longlong
                                            + 0 as libc::c_int as libc::c_longlong)
                                        - 1 as libc::c_int as libc::c_longlong
                                }) + *offset as libc::c_longlong)
                                    < 0 as libc::c_int as libc::c_longlong) as libc::c_int
                            } else {
                                ((0 as libc::c_int as libc::c_longlong)
                                    < (if (0 as libc::c_int as libc::c_longlong
                                        * (0 as libc::c_int as libc::c_longlong
                                            * *offset as libc::c_longlong
                                            + 0 as libc::c_int as libc::c_longlong)
                                        - 1 as libc::c_int as libc::c_longlong)
                                        < 0 as libc::c_int as libc::c_longlong
                                    {
                                        !((((0 as libc::c_int as libc::c_longlong
                                            * (0 as libc::c_int as libc::c_longlong
                                                * *offset as libc::c_longlong
                                                + 0 as libc::c_int as libc::c_longlong)
                                            + 1 as libc::c_int as libc::c_longlong)
                                            << (::std::mem::size_of::<libc::c_longlong>()
                                                as libc::c_ulong)
                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                            - 1 as libc::c_int as libc::c_longlong)
                                            * 2 as libc::c_int as libc::c_longlong
                                            + 1 as libc::c_int as libc::c_longlong)
                                    } else {
                                        0 as libc::c_int as libc::c_longlong
                                            * (0 as libc::c_int as libc::c_longlong
                                                * *offset as libc::c_longlong
                                                + 0 as libc::c_int as libc::c_longlong)
                                            + 0 as libc::c_int as libc::c_longlong
                                    }) + *offset as libc::c_longlong) as libc::c_int
                            }
                        } else {
                            if (0 as libc::c_int as libc::c_longlong)
                                < 0 as libc::c_int as libc::c_longlong
                            {
                                1 as libc::c_int
                            } else {
                                if (*offset as libc::c_longlong)
                                    < 0 as libc::c_int as libc::c_longlong
                                {
                                    (0 as libc::c_int as libc::c_longlong
                                        - *offset as libc::c_longlong
                                        <= 0 as libc::c_int as libc::c_longlong) as libc::c_int
                                } else {
                                    ((0 as libc::c_int as libc::c_longlong)
                                        < *offset as libc::c_longlong) as libc::c_int
                                }
                            }
                        }) != 0
                            || (0 as libc::c_int as libc::c_longlong
                                * (0 as libc::c_int as libc::c_longlong
                                    - *offset as libc::c_longlong)
                                - 1 as libc::c_int as libc::c_longlong)
                                < 0 as libc::c_int as libc::c_longlong
                                && (0 as libc::c_int as libc::c_longlong
                                    - *offset as libc::c_longlong)
                                    < -(9223372036854775807 as libc::c_longlong)
                                        - 1 as libc::c_longlong
                            || (9223372036854775807 as libc::c_longlong)
                                < 0 as libc::c_int as libc::c_longlong
                                    - *offset as libc::c_longlong
                        {
                            negative_offset_guess = (0 as libc::c_int as libc::c_longlong
                                as libc::c_ulonglong)
                                .wrapping_sub(
                                    *offset as libc::c_longlong as libc::c_ulonglong,
                                ) as libc::c_longlong as libc::c_int;
                        } else {
                            negative_offset_guess = (0 as libc::c_int as libc::c_longlong
                                as libc::c_ulonglong)
                                .wrapping_sub(
                                    *offset as libc::c_longlong as libc::c_ulonglong,
                                ) as libc::c_longlong as libc::c_int;
                        };
                    } else {
                        if (if (if (0 as libc::c_int as libc::c_long
                            * (0 as libc::c_int as libc::c_long * *offset
                                + 0 as libc::c_int as libc::c_long)
                            - 1 as libc::c_int as libc::c_long)
                            < 0 as libc::c_int as libc::c_long
                        {
                            !((((0 as libc::c_int as libc::c_long
                                * (0 as libc::c_int as libc::c_long * *offset
                                    + 0 as libc::c_int as libc::c_long)
                                + 1 as libc::c_int as libc::c_long)
                                << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                - 1 as libc::c_int as libc::c_long)
                                * 2 as libc::c_int as libc::c_long
                                + 1 as libc::c_int as libc::c_long)
                        } else {
                            0 as libc::c_int as libc::c_long
                                * (0 as libc::c_int as libc::c_long * *offset
                                    + 0 as libc::c_int as libc::c_long)
                                + 0 as libc::c_int as libc::c_long
                        }) < 0 as libc::c_int as libc::c_long
                        {
                            if *offset < 0 as libc::c_int as libc::c_long {
                                ((if (0 as libc::c_int as libc::c_long
                                    * (0 as libc::c_int as libc::c_long * *offset
                                        + 0 as libc::c_int as libc::c_long)
                                    - 1 as libc::c_int as libc::c_long)
                                    < 0 as libc::c_int as libc::c_long
                                {
                                    (((0 as libc::c_int as libc::c_long
                                        * (0 as libc::c_int as libc::c_long * *offset
                                            + 0 as libc::c_int as libc::c_long)
                                        + 1 as libc::c_int as libc::c_long)
                                        << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                        - 1 as libc::c_int as libc::c_long)
                                        * 2 as libc::c_int as libc::c_long
                                        + 1 as libc::c_int as libc::c_long
                                } else {
                                    0 as libc::c_int as libc::c_long
                                        * (0 as libc::c_int as libc::c_long * *offset
                                            + 0 as libc::c_int as libc::c_long)
                                        - 1 as libc::c_int as libc::c_long
                                }) + *offset < 0 as libc::c_int as libc::c_long)
                                    as libc::c_int
                            } else {
                                ((0 as libc::c_int as libc::c_long)
                                    < (if (0 as libc::c_int as libc::c_long
                                        * (0 as libc::c_int as libc::c_long * *offset
                                            + 0 as libc::c_int as libc::c_long)
                                        - 1 as libc::c_int as libc::c_long)
                                        < 0 as libc::c_int as libc::c_long
                                    {
                                        !((((0 as libc::c_int as libc::c_long
                                            * (0 as libc::c_int as libc::c_long * *offset
                                                + 0 as libc::c_int as libc::c_long)
                                            + 1 as libc::c_int as libc::c_long)
                                            << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                            - 1 as libc::c_int as libc::c_long)
                                            * 2 as libc::c_int as libc::c_long
                                            + 1 as libc::c_int as libc::c_long)
                                    } else {
                                        0 as libc::c_int as libc::c_long
                                            * (0 as libc::c_int as libc::c_long * *offset
                                                + 0 as libc::c_int as libc::c_long)
                                            + 0 as libc::c_int as libc::c_long
                                    }) + *offset) as libc::c_int
                            }
                        } else {
                            if (0 as libc::c_int) < 0 as libc::c_int {
                                1 as libc::c_int
                            } else {
                                if *offset < 0 as libc::c_int as libc::c_long {
                                    (0 as libc::c_int as libc::c_long - *offset
                                        <= 0 as libc::c_int as libc::c_long) as libc::c_int
                                } else {
                                    ((0 as libc::c_int as libc::c_long) < *offset)
                                        as libc::c_int
                                }
                            }
                        }) != 0
                            || (0 as libc::c_int as libc::c_long
                                * (0 as libc::c_int as libc::c_long - *offset)
                                - 1 as libc::c_int as libc::c_long)
                                < 0 as libc::c_int as libc::c_long
                                && ((0 as libc::c_int as libc::c_long - *offset)
                                    as libc::c_longlong)
                                    < -(9223372036854775807 as libc::c_longlong)
                                        - 1 as libc::c_longlong
                            || (9223372036854775807 as libc::c_longlong)
                                < (0 as libc::c_int as libc::c_long - *offset)
                                    as libc::c_longlong
                        {
                            negative_offset_guess = (0 as libc::c_int
                                as libc::c_ulonglong)
                                .wrapping_sub(*offset as libc::c_ulonglong)
                                as libc::c_longlong as libc::c_int;
                        } else {
                            negative_offset_guess = (0 as libc::c_int
                                as libc::c_ulonglong)
                                .wrapping_sub(*offset as libc::c_ulonglong)
                                as libc::c_longlong as libc::c_int;
                        };
                    };
                };
            };
        };
    };
    t0 = ydhms_diff(
        year,
        yday,
        hour,
        min,
        sec,
        1970 as libc::c_int - 1900 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        negative_offset_guess,
    );
    t2 = t0;
    t1 = t2;
    t = t1;
    dst2 = 0 as libc::c_int;
    loop {
        gt = guess_time_tm(
            year,
            yday,
            hour,
            min,
            sec,
            t,
            ranged_convert(convert, &mut t, &mut tm),
        );
        if !(t != gt) {
            current_block = 2370887241019905314;
            break;
        }
        if t == t1 && t != t2
            && (tm.tm_isdst < 0 as libc::c_int
                || (if isdst < 0 as libc::c_int {
                    (dst2 <= (tm.tm_isdst != 0 as libc::c_int) as libc::c_int)
                        as libc::c_int
                } else {
                    ((isdst != 0 as libc::c_int) as libc::c_int
                        != (tm.tm_isdst != 0 as libc::c_int) as libc::c_int)
                        as libc::c_int
                }) != 0)
        {
            current_block = 14799345100478017308;
            break;
        }
        remaining_probes -= 1;
        if remaining_probes == 0 as libc::c_int {
            return -(1 as libc::c_int) as time_t;
        }
        t1 = t2;
        t2 = t;
        t = gt;
        dst2 = (tm.tm_isdst != 0 as libc::c_int) as libc::c_int;
    }
    match current_block {
        2370887241019905314 => {
            if isdst_differ(isdst, tm.tm_isdst) {
                let mut stride: libc::c_int = 601200 as libc::c_int;
                let mut duration_max: libc::c_int = 536454000 as libc::c_int;
                let mut delta_bound: libc::c_int = duration_max / 2 as libc::c_int
                    + stride;
                let mut delta: libc::c_int = 0;
                let mut direction: libc::c_int = 0;
                delta = stride;
                's_109: while delta < delta_bound {
                    direction = -(1 as libc::c_int);
                    while direction <= 1 as libc::c_int {
                        let mut ot: long_int = 0;
                        if if ::std::mem::size_of::<long_int>() as libc::c_ulong
                            == ::std::mem::size_of::<libc::c_schar>() as libc::c_ulong
                        {
                            if (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                < ::std::mem::size_of::<libc::c_schar>() as libc::c_ulong
                            {
                                if (if (if (0 as libc::c_int
                                    * (0 as libc::c_int
                                        * (delta * direction) as libc::c_schar as libc::c_int
                                        + t as libc::c_schar as libc::c_int) - 1 as libc::c_int)
                                    < 0 as libc::c_int
                                {
                                    !((((0 as libc::c_int
                                        * (0 as libc::c_int
                                            * (delta * direction) as libc::c_schar as libc::c_int
                                            + t as libc::c_schar as libc::c_int) + 1 as libc::c_int)
                                        << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                        - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                                } else {
                                    0 as libc::c_int
                                        * (0 as libc::c_int
                                            * (delta * direction) as libc::c_schar as libc::c_int
                                            + t as libc::c_schar as libc::c_int) + 0 as libc::c_int
                                }) < 0 as libc::c_int
                                {
                                    if ((delta * direction) as libc::c_schar as libc::c_int)
                                        < 0 as libc::c_int
                                    {
                                        ((t as libc::c_schar as libc::c_int)
                                            < (if (0 as libc::c_int
                                                * (0 as libc::c_int
                                                    * (delta * direction) as libc::c_schar as libc::c_int
                                                    + t as libc::c_schar as libc::c_int) - 1 as libc::c_int)
                                                < 0 as libc::c_int
                                            {
                                                !((((0 as libc::c_int
                                                    * (0 as libc::c_int
                                                        * (delta * direction) as libc::c_schar as libc::c_int
                                                        + t as libc::c_schar as libc::c_int) + 1 as libc::c_int)
                                                    << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                    - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                                            } else {
                                                0 as libc::c_int
                                                    * (0 as libc::c_int
                                                        * (delta * direction) as libc::c_schar as libc::c_int
                                                        + t as libc::c_schar as libc::c_int) + 0 as libc::c_int
                                            }) - (delta * direction) as libc::c_schar as libc::c_int)
                                            as libc::c_int
                                    } else {
                                        (((if (0 as libc::c_int
                                            * (0 as libc::c_int
                                                * (delta * direction) as libc::c_schar as libc::c_int
                                                + t as libc::c_schar as libc::c_int) - 1 as libc::c_int)
                                            < 0 as libc::c_int
                                        {
                                            (((0 as libc::c_int
                                                * (0 as libc::c_int
                                                    * (delta * direction) as libc::c_schar as libc::c_int
                                                    + t as libc::c_schar as libc::c_int) + 1 as libc::c_int)
                                                << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                                        } else {
                                            0 as libc::c_int
                                                * (0 as libc::c_int
                                                    * (delta * direction) as libc::c_schar as libc::c_int
                                                    + t as libc::c_schar as libc::c_int) - 1 as libc::c_int
                                        }) - (delta * direction) as libc::c_schar as libc::c_int)
                                            < t as libc::c_schar as libc::c_int) as libc::c_int
                                    }
                                } else {
                                    if (t as libc::c_schar as libc::c_int) < 0 as libc::c_int {
                                        ((delta * direction) as libc::c_schar as libc::c_int
                                            <= t as libc::c_schar as libc::c_int
                                                + (delta * direction) as libc::c_schar as libc::c_int)
                                            as libc::c_int
                                    } else {
                                        if ((delta * direction) as libc::c_schar as libc::c_int)
                                            < 0 as libc::c_int
                                        {
                                            (t as libc::c_schar as libc::c_int
                                                <= t as libc::c_schar as libc::c_int
                                                    + (delta * direction) as libc::c_schar as libc::c_int)
                                                as libc::c_int
                                        } else {
                                            ((t as libc::c_schar as libc::c_int
                                                + (delta * direction) as libc::c_schar as libc::c_int)
                                                < (delta * direction) as libc::c_schar as libc::c_int)
                                                as libc::c_int
                                        }
                                    }
                                }) != 0
                                    || (0 as libc::c_int
                                        * (t as libc::c_schar as libc::c_int
                                            + (delta * direction) as libc::c_schar as libc::c_int)
                                        - 1 as libc::c_int) < 0 as libc::c_int
                                        && (t as libc::c_schar as libc::c_int
                                            + (delta * direction) as libc::c_schar as libc::c_int)
                                            < -(127 as libc::c_int) - 1 as libc::c_int
                                    || (127 as libc::c_int)
                                        < t as libc::c_schar as libc::c_int
                                            + (delta * direction) as libc::c_schar as libc::c_int
                                {
                                    ot = (t as libc::c_schar as libc::c_uint)
                                        .wrapping_add(
                                            (delta * direction) as libc::c_schar as libc::c_uint,
                                        ) as libc::c_schar as long_int;
                                    1 as libc::c_int
                                } else {
                                    ot = (t as libc::c_schar as libc::c_uint)
                                        .wrapping_add(
                                            (delta * direction) as libc::c_schar as libc::c_uint,
                                        ) as libc::c_schar as long_int;
                                    0 as libc::c_int
                                }
                            } else if (if (if (0 as libc::c_int as libc::c_long
                                * ((0 as libc::c_int * (delta * direction)) as libc::c_long
                                    + t) - 1 as libc::c_int as libc::c_long)
                                < 0 as libc::c_int as libc::c_long
                            {
                                !((((0 as libc::c_int as libc::c_long
                                    * ((0 as libc::c_int * (delta * direction)) as libc::c_long
                                        + t) + 1 as libc::c_int as libc::c_long)
                                    << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                    - 1 as libc::c_int as libc::c_long)
                                    * 2 as libc::c_int as libc::c_long
                                    + 1 as libc::c_int as libc::c_long)
                            } else {
                                0 as libc::c_int as libc::c_long
                                    * ((0 as libc::c_int * (delta * direction)) as libc::c_long
                                        + t) + 0 as libc::c_int as libc::c_long
                            }) < 0 as libc::c_int as libc::c_long
                            {
                                if delta * direction < 0 as libc::c_int {
                                    (t
                                        < (if (0 as libc::c_int as libc::c_long
                                            * ((0 as libc::c_int * (delta * direction)) as libc::c_long
                                                + t) - 1 as libc::c_int as libc::c_long)
                                            < 0 as libc::c_int as libc::c_long
                                        {
                                            !((((0 as libc::c_int as libc::c_long
                                                * ((0 as libc::c_int * (delta * direction)) as libc::c_long
                                                    + t) + 1 as libc::c_int as libc::c_long)
                                                << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                - 1 as libc::c_int as libc::c_long)
                                                * 2 as libc::c_int as libc::c_long
                                                + 1 as libc::c_int as libc::c_long)
                                        } else {
                                            0 as libc::c_int as libc::c_long
                                                * ((0 as libc::c_int * (delta * direction)) as libc::c_long
                                                    + t) + 0 as libc::c_int as libc::c_long
                                        }) - (delta * direction) as libc::c_long) as libc::c_int
                                } else {
                                    (((if (0 as libc::c_int as libc::c_long
                                        * ((0 as libc::c_int * (delta * direction)) as libc::c_long
                                            + t) - 1 as libc::c_int as libc::c_long)
                                        < 0 as libc::c_int as libc::c_long
                                    {
                                        (((0 as libc::c_int as libc::c_long
                                            * ((0 as libc::c_int * (delta * direction)) as libc::c_long
                                                + t) + 1 as libc::c_int as libc::c_long)
                                            << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                            - 1 as libc::c_int as libc::c_long)
                                            * 2 as libc::c_int as libc::c_long
                                            + 1 as libc::c_int as libc::c_long
                                    } else {
                                        0 as libc::c_int as libc::c_long
                                            * ((0 as libc::c_int * (delta * direction)) as libc::c_long
                                                + t) - 1 as libc::c_int as libc::c_long
                                    }) - (delta * direction) as libc::c_long) < t)
                                        as libc::c_int
                                }
                            } else {
                                if t < 0 as libc::c_int as libc::c_long {
                                    ((delta * direction) as libc::c_long
                                        <= t + (delta * direction) as libc::c_long) as libc::c_int
                                } else {
                                    if delta * direction < 0 as libc::c_int {
                                        (t <= t + (delta * direction) as libc::c_long)
                                            as libc::c_int
                                    } else {
                                        ((t + (delta * direction) as libc::c_long)
                                            < (delta * direction) as libc::c_long) as libc::c_int
                                    }
                                }
                            }) != 0
                                || (0 as libc::c_int as libc::c_long
                                    * (t + (delta * direction) as libc::c_long)
                                    - 1 as libc::c_int as libc::c_long)
                                    < 0 as libc::c_int as libc::c_long
                                    && (t + (delta * direction) as libc::c_long)
                                        < (-(127 as libc::c_int) - 1 as libc::c_int) as libc::c_long
                                || (127 as libc::c_int as libc::c_long)
                                    < t + (delta * direction) as libc::c_long
                            {
                                ot = (t as libc::c_uint)
                                    .wrapping_add((delta * direction) as libc::c_uint)
                                    as libc::c_schar as long_int;
                                1 as libc::c_int
                            } else {
                                ot = (t as libc::c_uint)
                                    .wrapping_add((delta * direction) as libc::c_uint)
                                    as libc::c_schar as long_int;
                                0 as libc::c_int
                            }
                        } else if ::std::mem::size_of::<long_int>() as libc::c_ulong
                            == ::std::mem::size_of::<libc::c_short>() as libc::c_ulong
                        {
                            if (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                < ::std::mem::size_of::<libc::c_short>() as libc::c_ulong
                            {
                                if (if (if (0 as libc::c_int
                                    * (0 as libc::c_int
                                        * (delta * direction) as libc::c_short as libc::c_int
                                        + t as libc::c_short as libc::c_int) - 1 as libc::c_int)
                                    < 0 as libc::c_int
                                {
                                    !((((0 as libc::c_int
                                        * (0 as libc::c_int
                                            * (delta * direction) as libc::c_short as libc::c_int
                                            + t as libc::c_short as libc::c_int) + 1 as libc::c_int)
                                        << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                        - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                                } else {
                                    0 as libc::c_int
                                        * (0 as libc::c_int
                                            * (delta * direction) as libc::c_short as libc::c_int
                                            + t as libc::c_short as libc::c_int) + 0 as libc::c_int
                                }) < 0 as libc::c_int
                                {
                                    if ((delta * direction) as libc::c_short as libc::c_int)
                                        < 0 as libc::c_int
                                    {
                                        ((t as libc::c_short as libc::c_int)
                                            < (if (0 as libc::c_int
                                                * (0 as libc::c_int
                                                    * (delta * direction) as libc::c_short as libc::c_int
                                                    + t as libc::c_short as libc::c_int) - 1 as libc::c_int)
                                                < 0 as libc::c_int
                                            {
                                                !((((0 as libc::c_int
                                                    * (0 as libc::c_int
                                                        * (delta * direction) as libc::c_short as libc::c_int
                                                        + t as libc::c_short as libc::c_int) + 1 as libc::c_int)
                                                    << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                    - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                                            } else {
                                                0 as libc::c_int
                                                    * (0 as libc::c_int
                                                        * (delta * direction) as libc::c_short as libc::c_int
                                                        + t as libc::c_short as libc::c_int) + 0 as libc::c_int
                                            }) - (delta * direction) as libc::c_short as libc::c_int)
                                            as libc::c_int
                                    } else {
                                        (((if (0 as libc::c_int
                                            * (0 as libc::c_int
                                                * (delta * direction) as libc::c_short as libc::c_int
                                                + t as libc::c_short as libc::c_int) - 1 as libc::c_int)
                                            < 0 as libc::c_int
                                        {
                                            (((0 as libc::c_int
                                                * (0 as libc::c_int
                                                    * (delta * direction) as libc::c_short as libc::c_int
                                                    + t as libc::c_short as libc::c_int) + 1 as libc::c_int)
                                                << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                                        } else {
                                            0 as libc::c_int
                                                * (0 as libc::c_int
                                                    * (delta * direction) as libc::c_short as libc::c_int
                                                    + t as libc::c_short as libc::c_int) - 1 as libc::c_int
                                        }) - (delta * direction) as libc::c_short as libc::c_int)
                                            < t as libc::c_short as libc::c_int) as libc::c_int
                                    }
                                } else {
                                    if (t as libc::c_short as libc::c_int) < 0 as libc::c_int {
                                        ((delta * direction) as libc::c_short as libc::c_int
                                            <= t as libc::c_short as libc::c_int
                                                + (delta * direction) as libc::c_short as libc::c_int)
                                            as libc::c_int
                                    } else {
                                        if ((delta * direction) as libc::c_short as libc::c_int)
                                            < 0 as libc::c_int
                                        {
                                            (t as libc::c_short as libc::c_int
                                                <= t as libc::c_short as libc::c_int
                                                    + (delta * direction) as libc::c_short as libc::c_int)
                                                as libc::c_int
                                        } else {
                                            ((t as libc::c_short as libc::c_int
                                                + (delta * direction) as libc::c_short as libc::c_int)
                                                < (delta * direction) as libc::c_short as libc::c_int)
                                                as libc::c_int
                                        }
                                    }
                                }) != 0
                                    || (0 as libc::c_int
                                        * (t as libc::c_short as libc::c_int
                                            + (delta * direction) as libc::c_short as libc::c_int)
                                        - 1 as libc::c_int) < 0 as libc::c_int
                                        && (t as libc::c_short as libc::c_int
                                            + (delta * direction) as libc::c_short as libc::c_int)
                                            < -(32767 as libc::c_int) - 1 as libc::c_int
                                    || (32767 as libc::c_int)
                                        < t as libc::c_short as libc::c_int
                                            + (delta * direction) as libc::c_short as libc::c_int
                                {
                                    ot = (t as libc::c_short as libc::c_uint)
                                        .wrapping_add(
                                            (delta * direction) as libc::c_short as libc::c_uint,
                                        ) as libc::c_short as long_int;
                                    1 as libc::c_int
                                } else {
                                    ot = (t as libc::c_short as libc::c_uint)
                                        .wrapping_add(
                                            (delta * direction) as libc::c_short as libc::c_uint,
                                        ) as libc::c_short as long_int;
                                    0 as libc::c_int
                                }
                            } else if (if (if (0 as libc::c_int as libc::c_long
                                * ((0 as libc::c_int * (delta * direction)) as libc::c_long
                                    + t) - 1 as libc::c_int as libc::c_long)
                                < 0 as libc::c_int as libc::c_long
                            {
                                !((((0 as libc::c_int as libc::c_long
                                    * ((0 as libc::c_int * (delta * direction)) as libc::c_long
                                        + t) + 1 as libc::c_int as libc::c_long)
                                    << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                    - 1 as libc::c_int as libc::c_long)
                                    * 2 as libc::c_int as libc::c_long
                                    + 1 as libc::c_int as libc::c_long)
                            } else {
                                0 as libc::c_int as libc::c_long
                                    * ((0 as libc::c_int * (delta * direction)) as libc::c_long
                                        + t) + 0 as libc::c_int as libc::c_long
                            }) < 0 as libc::c_int as libc::c_long
                            {
                                if delta * direction < 0 as libc::c_int {
                                    (t
                                        < (if (0 as libc::c_int as libc::c_long
                                            * ((0 as libc::c_int * (delta * direction)) as libc::c_long
                                                + t) - 1 as libc::c_int as libc::c_long)
                                            < 0 as libc::c_int as libc::c_long
                                        {
                                            !((((0 as libc::c_int as libc::c_long
                                                * ((0 as libc::c_int * (delta * direction)) as libc::c_long
                                                    + t) + 1 as libc::c_int as libc::c_long)
                                                << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                - 1 as libc::c_int as libc::c_long)
                                                * 2 as libc::c_int as libc::c_long
                                                + 1 as libc::c_int as libc::c_long)
                                        } else {
                                            0 as libc::c_int as libc::c_long
                                                * ((0 as libc::c_int * (delta * direction)) as libc::c_long
                                                    + t) + 0 as libc::c_int as libc::c_long
                                        }) - (delta * direction) as libc::c_long) as libc::c_int
                                } else {
                                    (((if (0 as libc::c_int as libc::c_long
                                        * ((0 as libc::c_int * (delta * direction)) as libc::c_long
                                            + t) - 1 as libc::c_int as libc::c_long)
                                        < 0 as libc::c_int as libc::c_long
                                    {
                                        (((0 as libc::c_int as libc::c_long
                                            * ((0 as libc::c_int * (delta * direction)) as libc::c_long
                                                + t) + 1 as libc::c_int as libc::c_long)
                                            << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                            - 1 as libc::c_int as libc::c_long)
                                            * 2 as libc::c_int as libc::c_long
                                            + 1 as libc::c_int as libc::c_long
                                    } else {
                                        0 as libc::c_int as libc::c_long
                                            * ((0 as libc::c_int * (delta * direction)) as libc::c_long
                                                + t) - 1 as libc::c_int as libc::c_long
                                    }) - (delta * direction) as libc::c_long) < t)
                                        as libc::c_int
                                }
                            } else {
                                if t < 0 as libc::c_int as libc::c_long {
                                    ((delta * direction) as libc::c_long
                                        <= t + (delta * direction) as libc::c_long) as libc::c_int
                                } else {
                                    if delta * direction < 0 as libc::c_int {
                                        (t <= t + (delta * direction) as libc::c_long)
                                            as libc::c_int
                                    } else {
                                        ((t + (delta * direction) as libc::c_long)
                                            < (delta * direction) as libc::c_long) as libc::c_int
                                    }
                                }
                            }) != 0
                                || (0 as libc::c_int as libc::c_long
                                    * (t + (delta * direction) as libc::c_long)
                                    - 1 as libc::c_int as libc::c_long)
                                    < 0 as libc::c_int as libc::c_long
                                    && (t + (delta * direction) as libc::c_long)
                                        < (-(32767 as libc::c_int) - 1 as libc::c_int)
                                            as libc::c_long
                                || (32767 as libc::c_int as libc::c_long)
                                    < t + (delta * direction) as libc::c_long
                            {
                                ot = (t as libc::c_uint)
                                    .wrapping_add((delta * direction) as libc::c_uint)
                                    as libc::c_short as long_int;
                                1 as libc::c_int
                            } else {
                                ot = (t as libc::c_uint)
                                    .wrapping_add((delta * direction) as libc::c_uint)
                                    as libc::c_short as long_int;
                                0 as libc::c_int
                            }
                        } else if ::std::mem::size_of::<long_int>() as libc::c_ulong
                            == ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                        {
                            if (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                < ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                            {
                                if (if (if (0 as libc::c_int
                                    * (0 as libc::c_int * (delta * direction)
                                        + t as libc::c_int) - 1 as libc::c_int) < 0 as libc::c_int
                                {
                                    !((((0 as libc::c_int
                                        * (0 as libc::c_int * (delta * direction)
                                            + t as libc::c_int) + 1 as libc::c_int)
                                        << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                        - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                                } else {
                                    0 as libc::c_int
                                        * (0 as libc::c_int * (delta * direction)
                                            + t as libc::c_int) + 0 as libc::c_int
                                }) < 0 as libc::c_int
                                {
                                    if delta * direction < 0 as libc::c_int {
                                        ((t as libc::c_int)
                                            < (if (0 as libc::c_int
                                                * (0 as libc::c_int * (delta * direction)
                                                    + t as libc::c_int) - 1 as libc::c_int) < 0 as libc::c_int
                                            {
                                                !((((0 as libc::c_int
                                                    * (0 as libc::c_int * (delta * direction)
                                                        + t as libc::c_int) + 1 as libc::c_int)
                                                    << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                    - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                                            } else {
                                                0 as libc::c_int
                                                    * (0 as libc::c_int * (delta * direction)
                                                        + t as libc::c_int) + 0 as libc::c_int
                                            }) - delta * direction) as libc::c_int
                                    } else {
                                        ((if (0 as libc::c_int
                                            * (0 as libc::c_int * (delta * direction)
                                                + t as libc::c_int) - 1 as libc::c_int) < 0 as libc::c_int
                                        {
                                            (((0 as libc::c_int
                                                * (0 as libc::c_int * (delta * direction)
                                                    + t as libc::c_int) + 1 as libc::c_int)
                                                << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                                        } else {
                                            0 as libc::c_int
                                                * (0 as libc::c_int * (delta * direction)
                                                    + t as libc::c_int) - 1 as libc::c_int
                                        }) - delta * direction < t as libc::c_int) as libc::c_int
                                    }
                                } else {
                                    if (t as libc::c_int) < 0 as libc::c_int {
                                        (delta * direction <= t as libc::c_int + delta * direction)
                                            as libc::c_int
                                    } else {
                                        if delta * direction < 0 as libc::c_int {
                                            (t as libc::c_int <= t as libc::c_int + delta * direction)
                                                as libc::c_int
                                        } else {
                                            (t as libc::c_int + delta * direction < delta * direction)
                                                as libc::c_int
                                        }
                                    }
                                }) != 0
                                    || (0 as libc::c_int
                                        * (t as libc::c_int + delta * direction) - 1 as libc::c_int)
                                        < 0 as libc::c_int
                                        && t as libc::c_int + delta * direction
                                            < -(2147483647 as libc::c_int) - 1 as libc::c_int
                                    || (2147483647 as libc::c_int)
                                        < t as libc::c_int + delta * direction
                                {
                                    ot = (t as libc::c_int as libc::c_uint)
                                        .wrapping_add((delta * direction) as libc::c_uint)
                                        as libc::c_int as long_int;
                                    1 as libc::c_int
                                } else {
                                    ot = (t as libc::c_int as libc::c_uint)
                                        .wrapping_add((delta * direction) as libc::c_uint)
                                        as libc::c_int as long_int;
                                    0 as libc::c_int
                                }
                            } else if (if (if (0 as libc::c_int as libc::c_long
                                * ((0 as libc::c_int * (delta * direction)) as libc::c_long
                                    + t) - 1 as libc::c_int as libc::c_long)
                                < 0 as libc::c_int as libc::c_long
                            {
                                !((((0 as libc::c_int as libc::c_long
                                    * ((0 as libc::c_int * (delta * direction)) as libc::c_long
                                        + t) + 1 as libc::c_int as libc::c_long)
                                    << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                    - 1 as libc::c_int as libc::c_long)
                                    * 2 as libc::c_int as libc::c_long
                                    + 1 as libc::c_int as libc::c_long)
                            } else {
                                0 as libc::c_int as libc::c_long
                                    * ((0 as libc::c_int * (delta * direction)) as libc::c_long
                                        + t) + 0 as libc::c_int as libc::c_long
                            }) < 0 as libc::c_int as libc::c_long
                            {
                                if delta * direction < 0 as libc::c_int {
                                    (t
                                        < (if (0 as libc::c_int as libc::c_long
                                            * ((0 as libc::c_int * (delta * direction)) as libc::c_long
                                                + t) - 1 as libc::c_int as libc::c_long)
                                            < 0 as libc::c_int as libc::c_long
                                        {
                                            !((((0 as libc::c_int as libc::c_long
                                                * ((0 as libc::c_int * (delta * direction)) as libc::c_long
                                                    + t) + 1 as libc::c_int as libc::c_long)
                                                << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                - 1 as libc::c_int as libc::c_long)
                                                * 2 as libc::c_int as libc::c_long
                                                + 1 as libc::c_int as libc::c_long)
                                        } else {
                                            0 as libc::c_int as libc::c_long
                                                * ((0 as libc::c_int * (delta * direction)) as libc::c_long
                                                    + t) + 0 as libc::c_int as libc::c_long
                                        }) - (delta * direction) as libc::c_long) as libc::c_int
                                } else {
                                    (((if (0 as libc::c_int as libc::c_long
                                        * ((0 as libc::c_int * (delta * direction)) as libc::c_long
                                            + t) - 1 as libc::c_int as libc::c_long)
                                        < 0 as libc::c_int as libc::c_long
                                    {
                                        (((0 as libc::c_int as libc::c_long
                                            * ((0 as libc::c_int * (delta * direction)) as libc::c_long
                                                + t) + 1 as libc::c_int as libc::c_long)
                                            << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                            - 1 as libc::c_int as libc::c_long)
                                            * 2 as libc::c_int as libc::c_long
                                            + 1 as libc::c_int as libc::c_long
                                    } else {
                                        0 as libc::c_int as libc::c_long
                                            * ((0 as libc::c_int * (delta * direction)) as libc::c_long
                                                + t) - 1 as libc::c_int as libc::c_long
                                    }) - (delta * direction) as libc::c_long) < t)
                                        as libc::c_int
                                }
                            } else {
                                if t < 0 as libc::c_int as libc::c_long {
                                    ((delta * direction) as libc::c_long
                                        <= t + (delta * direction) as libc::c_long) as libc::c_int
                                } else {
                                    if delta * direction < 0 as libc::c_int {
                                        (t <= t + (delta * direction) as libc::c_long)
                                            as libc::c_int
                                    } else {
                                        ((t + (delta * direction) as libc::c_long)
                                            < (delta * direction) as libc::c_long) as libc::c_int
                                    }
                                }
                            }) != 0
                                || (0 as libc::c_int as libc::c_long
                                    * (t + (delta * direction) as libc::c_long)
                                    - 1 as libc::c_int as libc::c_long)
                                    < 0 as libc::c_int as libc::c_long
                                    && (t + (delta * direction) as libc::c_long)
                                        < (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                            as libc::c_long
                                || (2147483647 as libc::c_int as libc::c_long)
                                    < t + (delta * direction) as libc::c_long
                            {
                                ot = (t as libc::c_uint)
                                    .wrapping_add((delta * direction) as libc::c_uint)
                                    as libc::c_int as long_int;
                                1 as libc::c_int
                            } else {
                                ot = (t as libc::c_uint)
                                    .wrapping_add((delta * direction) as libc::c_uint)
                                    as libc::c_int as long_int;
                                0 as libc::c_int
                            }
                        } else if ::std::mem::size_of::<long_int>() as libc::c_ulong
                            == ::std::mem::size_of::<libc::c_long>() as libc::c_ulong
                        {
                            if (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                < ::std::mem::size_of::<libc::c_long>() as libc::c_ulong
                            {
                                if (if (if (0 as libc::c_int as libc::c_long
                                    * (0 as libc::c_int as libc::c_long
                                        * (delta * direction) as libc::c_long + t)
                                    - 1 as libc::c_int as libc::c_long)
                                    < 0 as libc::c_int as libc::c_long
                                {
                                    !((((0 as libc::c_int as libc::c_long
                                        * (0 as libc::c_int as libc::c_long
                                            * (delta * direction) as libc::c_long + t)
                                        + 1 as libc::c_int as libc::c_long)
                                        << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                        - 1 as libc::c_int as libc::c_long)
                                        * 2 as libc::c_int as libc::c_long
                                        + 1 as libc::c_int as libc::c_long)
                                } else {
                                    0 as libc::c_int as libc::c_long
                                        * (0 as libc::c_int as libc::c_long
                                            * (delta * direction) as libc::c_long + t)
                                        + 0 as libc::c_int as libc::c_long
                                }) < 0 as libc::c_int as libc::c_long
                                {
                                    if ((delta * direction) as libc::c_long)
                                        < 0 as libc::c_int as libc::c_long
                                    {
                                        (t
                                            < (if (0 as libc::c_int as libc::c_long
                                                * (0 as libc::c_int as libc::c_long
                                                    * (delta * direction) as libc::c_long + t)
                                                - 1 as libc::c_int as libc::c_long)
                                                < 0 as libc::c_int as libc::c_long
                                            {
                                                !((((0 as libc::c_int as libc::c_long
                                                    * (0 as libc::c_int as libc::c_long
                                                        * (delta * direction) as libc::c_long + t)
                                                    + 1 as libc::c_int as libc::c_long)
                                                    << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                    - 1 as libc::c_int as libc::c_long)
                                                    * 2 as libc::c_int as libc::c_long
                                                    + 1 as libc::c_int as libc::c_long)
                                            } else {
                                                0 as libc::c_int as libc::c_long
                                                    * (0 as libc::c_int as libc::c_long
                                                        * (delta * direction) as libc::c_long + t)
                                                    + 0 as libc::c_int as libc::c_long
                                            }) - (delta * direction) as libc::c_long) as libc::c_int
                                    } else {
                                        (((if (0 as libc::c_int as libc::c_long
                                            * (0 as libc::c_int as libc::c_long
                                                * (delta * direction) as libc::c_long + t)
                                            - 1 as libc::c_int as libc::c_long)
                                            < 0 as libc::c_int as libc::c_long
                                        {
                                            (((0 as libc::c_int as libc::c_long
                                                * (0 as libc::c_int as libc::c_long
                                                    * (delta * direction) as libc::c_long + t)
                                                + 1 as libc::c_int as libc::c_long)
                                                << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                - 1 as libc::c_int as libc::c_long)
                                                * 2 as libc::c_int as libc::c_long
                                                + 1 as libc::c_int as libc::c_long
                                        } else {
                                            0 as libc::c_int as libc::c_long
                                                * (0 as libc::c_int as libc::c_long
                                                    * (delta * direction) as libc::c_long + t)
                                                - 1 as libc::c_int as libc::c_long
                                        }) - (delta * direction) as libc::c_long) < t)
                                            as libc::c_int
                                    }
                                } else {
                                    if t < 0 as libc::c_int as libc::c_long {
                                        ((delta * direction) as libc::c_long
                                            <= t + (delta * direction) as libc::c_long) as libc::c_int
                                    } else {
                                        if ((delta * direction) as libc::c_long)
                                            < 0 as libc::c_int as libc::c_long
                                        {
                                            (t <= t + (delta * direction) as libc::c_long)
                                                as libc::c_int
                                        } else {
                                            ((t + (delta * direction) as libc::c_long)
                                                < (delta * direction) as libc::c_long) as libc::c_int
                                        }
                                    }
                                }) != 0
                                    || (0 as libc::c_int as libc::c_long
                                        * (t + (delta * direction) as libc::c_long)
                                        - 1 as libc::c_int as libc::c_long)
                                        < 0 as libc::c_int as libc::c_long
                                        && (t + (delta * direction) as libc::c_long)
                                            < -(9223372036854775807 as libc::c_long) - 1 as libc::c_long
                                    || (9223372036854775807 as libc::c_long)
                                        < t + (delta * direction) as libc::c_long
                                {
                                    ot = (t as libc::c_ulong)
                                        .wrapping_add(
                                            (delta * direction) as libc::c_long as libc::c_ulong,
                                        ) as libc::c_long;
                                    1 as libc::c_int
                                } else {
                                    ot = (t as libc::c_ulong)
                                        .wrapping_add(
                                            (delta * direction) as libc::c_long as libc::c_ulong,
                                        ) as libc::c_long;
                                    0 as libc::c_int
                                }
                            } else if (if (if (0 as libc::c_int as libc::c_long
                                * ((0 as libc::c_int * (delta * direction)) as libc::c_long
                                    + t) - 1 as libc::c_int as libc::c_long)
                                < 0 as libc::c_int as libc::c_long
                            {
                                !((((0 as libc::c_int as libc::c_long
                                    * ((0 as libc::c_int * (delta * direction)) as libc::c_long
                                        + t) + 1 as libc::c_int as libc::c_long)
                                    << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                    - 1 as libc::c_int as libc::c_long)
                                    * 2 as libc::c_int as libc::c_long
                                    + 1 as libc::c_int as libc::c_long)
                            } else {
                                0 as libc::c_int as libc::c_long
                                    * ((0 as libc::c_int * (delta * direction)) as libc::c_long
                                        + t) + 0 as libc::c_int as libc::c_long
                            }) < 0 as libc::c_int as libc::c_long
                            {
                                if delta * direction < 0 as libc::c_int {
                                    (t
                                        < (if (0 as libc::c_int as libc::c_long
                                            * ((0 as libc::c_int * (delta * direction)) as libc::c_long
                                                + t) - 1 as libc::c_int as libc::c_long)
                                            < 0 as libc::c_int as libc::c_long
                                        {
                                            !((((0 as libc::c_int as libc::c_long
                                                * ((0 as libc::c_int * (delta * direction)) as libc::c_long
                                                    + t) + 1 as libc::c_int as libc::c_long)
                                                << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                - 1 as libc::c_int as libc::c_long)
                                                * 2 as libc::c_int as libc::c_long
                                                + 1 as libc::c_int as libc::c_long)
                                        } else {
                                            0 as libc::c_int as libc::c_long
                                                * ((0 as libc::c_int * (delta * direction)) as libc::c_long
                                                    + t) + 0 as libc::c_int as libc::c_long
                                        }) - (delta * direction) as libc::c_long) as libc::c_int
                                } else {
                                    (((if (0 as libc::c_int as libc::c_long
                                        * ((0 as libc::c_int * (delta * direction)) as libc::c_long
                                            + t) - 1 as libc::c_int as libc::c_long)
                                        < 0 as libc::c_int as libc::c_long
                                    {
                                        (((0 as libc::c_int as libc::c_long
                                            * ((0 as libc::c_int * (delta * direction)) as libc::c_long
                                                + t) + 1 as libc::c_int as libc::c_long)
                                            << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                            - 1 as libc::c_int as libc::c_long)
                                            * 2 as libc::c_int as libc::c_long
                                            + 1 as libc::c_int as libc::c_long
                                    } else {
                                        0 as libc::c_int as libc::c_long
                                            * ((0 as libc::c_int * (delta * direction)) as libc::c_long
                                                + t) - 1 as libc::c_int as libc::c_long
                                    }) - (delta * direction) as libc::c_long) < t)
                                        as libc::c_int
                                }
                            } else {
                                if t < 0 as libc::c_int as libc::c_long {
                                    ((delta * direction) as libc::c_long
                                        <= t + (delta * direction) as libc::c_long) as libc::c_int
                                } else {
                                    if delta * direction < 0 as libc::c_int {
                                        (t <= t + (delta * direction) as libc::c_long)
                                            as libc::c_int
                                    } else {
                                        ((t + (delta * direction) as libc::c_long)
                                            < (delta * direction) as libc::c_long) as libc::c_int
                                    }
                                }
                            }) != 0
                                || (0 as libc::c_int as libc::c_long
                                    * (t + (delta * direction) as libc::c_long)
                                    - 1 as libc::c_int as libc::c_long)
                                    < 0 as libc::c_int as libc::c_long
                                    && (t + (delta * direction) as libc::c_long)
                                        < -(9223372036854775807 as libc::c_long) - 1 as libc::c_long
                                || (9223372036854775807 as libc::c_long)
                                    < t + (delta * direction) as libc::c_long
                            {
                                ot = (t as libc::c_ulong)
                                    .wrapping_add((delta * direction) as libc::c_ulong)
                                    as libc::c_long;
                                1 as libc::c_int
                            } else {
                                ot = (t as libc::c_ulong)
                                    .wrapping_add((delta * direction) as libc::c_ulong)
                                    as libc::c_long;
                                0 as libc::c_int
                            }
                        } else if (::std::mem::size_of::<libc::c_long>()
                            as libc::c_ulong)
                            < ::std::mem::size_of::<libc::c_longlong>() as libc::c_ulong
                        {
                            if (if (if (0 as libc::c_int as libc::c_longlong
                                * (0 as libc::c_int as libc::c_longlong
                                    * (delta * direction) as libc::c_longlong
                                    + t as libc::c_longlong)
                                - 1 as libc::c_int as libc::c_longlong)
                                < 0 as libc::c_int as libc::c_longlong
                            {
                                !((((0 as libc::c_int as libc::c_longlong
                                    * (0 as libc::c_int as libc::c_longlong
                                        * (delta * direction) as libc::c_longlong
                                        + t as libc::c_longlong)
                                    + 1 as libc::c_int as libc::c_longlong)
                                    << (::std::mem::size_of::<libc::c_longlong>()
                                        as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                    - 1 as libc::c_int as libc::c_longlong)
                                    * 2 as libc::c_int as libc::c_longlong
                                    + 1 as libc::c_int as libc::c_longlong)
                            } else {
                                0 as libc::c_int as libc::c_longlong
                                    * (0 as libc::c_int as libc::c_longlong
                                        * (delta * direction) as libc::c_longlong
                                        + t as libc::c_longlong)
                                    + 0 as libc::c_int as libc::c_longlong
                            }) < 0 as libc::c_int as libc::c_longlong
                            {
                                if ((delta * direction) as libc::c_longlong)
                                    < 0 as libc::c_int as libc::c_longlong
                                {
                                    ((t as libc::c_longlong)
                                        < (if (0 as libc::c_int as libc::c_longlong
                                            * (0 as libc::c_int as libc::c_longlong
                                                * (delta * direction) as libc::c_longlong
                                                + t as libc::c_longlong)
                                            - 1 as libc::c_int as libc::c_longlong)
                                            < 0 as libc::c_int as libc::c_longlong
                                        {
                                            !((((0 as libc::c_int as libc::c_longlong
                                                * (0 as libc::c_int as libc::c_longlong
                                                    * (delta * direction) as libc::c_longlong
                                                    + t as libc::c_longlong)
                                                + 1 as libc::c_int as libc::c_longlong)
                                                << (::std::mem::size_of::<libc::c_longlong>()
                                                    as libc::c_ulong)
                                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                - 1 as libc::c_int as libc::c_longlong)
                                                * 2 as libc::c_int as libc::c_longlong
                                                + 1 as libc::c_int as libc::c_longlong)
                                        } else {
                                            0 as libc::c_int as libc::c_longlong
                                                * (0 as libc::c_int as libc::c_longlong
                                                    * (delta * direction) as libc::c_longlong
                                                    + t as libc::c_longlong)
                                                + 0 as libc::c_int as libc::c_longlong
                                        }) - (delta * direction) as libc::c_longlong) as libc::c_int
                                } else {
                                    (((if (0 as libc::c_int as libc::c_longlong
                                        * (0 as libc::c_int as libc::c_longlong
                                            * (delta * direction) as libc::c_longlong
                                            + t as libc::c_longlong)
                                        - 1 as libc::c_int as libc::c_longlong)
                                        < 0 as libc::c_int as libc::c_longlong
                                    {
                                        (((0 as libc::c_int as libc::c_longlong
                                            * (0 as libc::c_int as libc::c_longlong
                                                * (delta * direction) as libc::c_longlong
                                                + t as libc::c_longlong)
                                            + 1 as libc::c_int as libc::c_longlong)
                                            << (::std::mem::size_of::<libc::c_longlong>()
                                                as libc::c_ulong)
                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                            - 1 as libc::c_int as libc::c_longlong)
                                            * 2 as libc::c_int as libc::c_longlong
                                            + 1 as libc::c_int as libc::c_longlong
                                    } else {
                                        0 as libc::c_int as libc::c_longlong
                                            * (0 as libc::c_int as libc::c_longlong
                                                * (delta * direction) as libc::c_longlong
                                                + t as libc::c_longlong)
                                            - 1 as libc::c_int as libc::c_longlong
                                    }) - (delta * direction) as libc::c_longlong)
                                        < t as libc::c_longlong) as libc::c_int
                                }
                            } else {
                                if (t as libc::c_longlong)
                                    < 0 as libc::c_int as libc::c_longlong
                                {
                                    ((delta * direction) as libc::c_longlong
                                        <= t as libc::c_longlong
                                            + (delta * direction) as libc::c_longlong) as libc::c_int
                                } else {
                                    if ((delta * direction) as libc::c_longlong)
                                        < 0 as libc::c_int as libc::c_longlong
                                    {
                                        (t as libc::c_longlong
                                            <= t as libc::c_longlong
                                                + (delta * direction) as libc::c_longlong) as libc::c_int
                                    } else {
                                        ((t as libc::c_longlong
                                            + (delta * direction) as libc::c_longlong)
                                            < (delta * direction) as libc::c_longlong) as libc::c_int
                                    }
                                }
                            }) != 0
                                || (0 as libc::c_int as libc::c_longlong
                                    * (t as libc::c_longlong
                                        + (delta * direction) as libc::c_longlong)
                                    - 1 as libc::c_int as libc::c_longlong)
                                    < 0 as libc::c_int as libc::c_longlong
                                    && (t as libc::c_longlong
                                        + (delta * direction) as libc::c_longlong)
                                        < -(9223372036854775807 as libc::c_longlong)
                                            - 1 as libc::c_longlong
                                || (9223372036854775807 as libc::c_longlong)
                                    < t as libc::c_longlong
                                        + (delta * direction) as libc::c_longlong
                            {
                                ot = (t as libc::c_longlong as libc::c_ulonglong)
                                    .wrapping_add(
                                        (delta * direction) as libc::c_longlong as libc::c_ulonglong,
                                    ) as libc::c_longlong as long_int;
                                1 as libc::c_int
                            } else {
                                ot = (t as libc::c_longlong as libc::c_ulonglong)
                                    .wrapping_add(
                                        (delta * direction) as libc::c_longlong as libc::c_ulonglong,
                                    ) as libc::c_longlong as long_int;
                                0 as libc::c_int
                            }
                        } else if (if (if (0 as libc::c_int as libc::c_long
                            * ((0 as libc::c_int * (delta * direction)) as libc::c_long
                                + t) - 1 as libc::c_int as libc::c_long)
                            < 0 as libc::c_int as libc::c_long
                        {
                            !((((0 as libc::c_int as libc::c_long
                                * ((0 as libc::c_int * (delta * direction)) as libc::c_long
                                    + t) + 1 as libc::c_int as libc::c_long)
                                << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                - 1 as libc::c_int as libc::c_long)
                                * 2 as libc::c_int as libc::c_long
                                + 1 as libc::c_int as libc::c_long)
                        } else {
                            0 as libc::c_int as libc::c_long
                                * ((0 as libc::c_int * (delta * direction)) as libc::c_long
                                    + t) + 0 as libc::c_int as libc::c_long
                        }) < 0 as libc::c_int as libc::c_long
                        {
                            if delta * direction < 0 as libc::c_int {
                                (t
                                    < (if (0 as libc::c_int as libc::c_long
                                        * ((0 as libc::c_int * (delta * direction)) as libc::c_long
                                            + t) - 1 as libc::c_int as libc::c_long)
                                        < 0 as libc::c_int as libc::c_long
                                    {
                                        !((((0 as libc::c_int as libc::c_long
                                            * ((0 as libc::c_int * (delta * direction)) as libc::c_long
                                                + t) + 1 as libc::c_int as libc::c_long)
                                            << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                            - 1 as libc::c_int as libc::c_long)
                                            * 2 as libc::c_int as libc::c_long
                                            + 1 as libc::c_int as libc::c_long)
                                    } else {
                                        0 as libc::c_int as libc::c_long
                                            * ((0 as libc::c_int * (delta * direction)) as libc::c_long
                                                + t) + 0 as libc::c_int as libc::c_long
                                    }) - (delta * direction) as libc::c_long) as libc::c_int
                            } else {
                                (((if (0 as libc::c_int as libc::c_long
                                    * ((0 as libc::c_int * (delta * direction)) as libc::c_long
                                        + t) - 1 as libc::c_int as libc::c_long)
                                    < 0 as libc::c_int as libc::c_long
                                {
                                    (((0 as libc::c_int as libc::c_long
                                        * ((0 as libc::c_int * (delta * direction)) as libc::c_long
                                            + t) + 1 as libc::c_int as libc::c_long)
                                        << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                        - 1 as libc::c_int as libc::c_long)
                                        * 2 as libc::c_int as libc::c_long
                                        + 1 as libc::c_int as libc::c_long
                                } else {
                                    0 as libc::c_int as libc::c_long
                                        * ((0 as libc::c_int * (delta * direction)) as libc::c_long
                                            + t) - 1 as libc::c_int as libc::c_long
                                }) - (delta * direction) as libc::c_long) < t)
                                    as libc::c_int
                            }
                        } else {
                            if t < 0 as libc::c_int as libc::c_long {
                                ((delta * direction) as libc::c_long
                                    <= t + (delta * direction) as libc::c_long) as libc::c_int
                            } else {
                                if delta * direction < 0 as libc::c_int {
                                    (t <= t + (delta * direction) as libc::c_long)
                                        as libc::c_int
                                } else {
                                    ((t + (delta * direction) as libc::c_long)
                                        < (delta * direction) as libc::c_long) as libc::c_int
                                }
                            }
                        }) != 0
                            || (0 as libc::c_int as libc::c_long
                                * (t + (delta * direction) as libc::c_long)
                                - 1 as libc::c_int as libc::c_long)
                                < 0 as libc::c_int as libc::c_long
                                && ((t + (delta * direction) as libc::c_long)
                                    as libc::c_longlong)
                                    < -(9223372036854775807 as libc::c_longlong)
                                        - 1 as libc::c_longlong
                            || (9223372036854775807 as libc::c_longlong)
                                < (t + (delta * direction) as libc::c_long)
                                    as libc::c_longlong
                        {
                            ot = (t as libc::c_ulonglong)
                                .wrapping_add((delta * direction) as libc::c_ulonglong)
                                as libc::c_longlong as long_int;
                            1 as libc::c_int
                        } else {
                            ot = (t as libc::c_ulonglong)
                                .wrapping_add((delta * direction) as libc::c_ulonglong)
                                as libc::c_longlong as long_int;
                            0 as libc::c_int
                        } == 0
                        {
                            let mut otm: tm = tm {
                                tm_sec: 0,
                                tm_min: 0,
                                tm_hour: 0,
                                tm_mday: 0,
                                tm_mon: 0,
                                tm_year: 0,
                                tm_wday: 0,
                                tm_yday: 0,
                                tm_isdst: 0,
                                tm_gmtoff: 0,
                                tm_zone: 0 as *const libc::c_char,
                            };
                            ranged_convert(convert, &mut ot, &mut otm);
                            if !isdst_differ(isdst, otm.tm_isdst) {
                                t = guess_time_tm(year, yday, hour, min, sec, ot, &mut otm);
                                ranged_convert(convert, &mut t, &mut tm);
                                break 's_109;
                            }
                        }
                        direction += 2 as libc::c_int;
                    }
                    delta += stride;
                }
            }
        }
        _ => {}
    }
    if ::std::mem::size_of::<long_int>() as libc::c_ulong
        == ::std::mem::size_of::<libc::c_schar>() as libc::c_ulong
    {
        if (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
            < ::std::mem::size_of::<libc::c_schar>() as libc::c_ulong
        {
            if (if (if (0 as libc::c_int
                * (0 as libc::c_int * t0 as libc::c_schar as libc::c_int
                    + t as libc::c_schar as libc::c_int) - 1 as libc::c_int)
                < 0 as libc::c_int
            {
                !((((0 as libc::c_int
                    * (0 as libc::c_int * t0 as libc::c_schar as libc::c_int
                        + t as libc::c_schar as libc::c_int) + 1 as libc::c_int)
                    << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                    - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
            } else {
                0 as libc::c_int
                    * (0 as libc::c_int * t0 as libc::c_schar as libc::c_int
                        + t as libc::c_schar as libc::c_int) + 0 as libc::c_int
            }) < 0 as libc::c_int
            {
                if (t0 as libc::c_schar as libc::c_int) < 0 as libc::c_int {
                    (((if (0 as libc::c_int
                        * (0 as libc::c_int * t0 as libc::c_schar as libc::c_int
                            + t as libc::c_schar as libc::c_int) - 1 as libc::c_int)
                        < 0 as libc::c_int
                    {
                        (((0 as libc::c_int
                            * (0 as libc::c_int * t0 as libc::c_schar as libc::c_int
                                + t as libc::c_schar as libc::c_int) + 1 as libc::c_int)
                            << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                            - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                    } else {
                        0 as libc::c_int
                            * (0 as libc::c_int * t0 as libc::c_schar as libc::c_int
                                + t as libc::c_schar as libc::c_int) - 1 as libc::c_int
                    }) + t0 as libc::c_schar as libc::c_int)
                        < t as libc::c_schar as libc::c_int) as libc::c_int
                } else {
                    ((t as libc::c_schar as libc::c_int)
                        < (if (0 as libc::c_int
                            * (0 as libc::c_int * t0 as libc::c_schar as libc::c_int
                                + t as libc::c_schar as libc::c_int) - 1 as libc::c_int)
                            < 0 as libc::c_int
                        {
                            !((((0 as libc::c_int
                                * (0 as libc::c_int * t0 as libc::c_schar as libc::c_int
                                    + t as libc::c_schar as libc::c_int) + 1 as libc::c_int)
                                << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                        } else {
                            0 as libc::c_int
                                * (0 as libc::c_int * t0 as libc::c_schar as libc::c_int
                                    + t as libc::c_schar as libc::c_int) + 0 as libc::c_int
                        }) + t0 as libc::c_schar as libc::c_int) as libc::c_int
                }
            } else {
                if (t as libc::c_schar as libc::c_int) < 0 as libc::c_int {
                    1 as libc::c_int
                } else {
                    if (t0 as libc::c_schar as libc::c_int) < 0 as libc::c_int {
                        (t as libc::c_schar as libc::c_int
                            - t0 as libc::c_schar as libc::c_int
                            <= t as libc::c_schar as libc::c_int) as libc::c_int
                    } else {
                        ((t as libc::c_schar as libc::c_int)
                            < t0 as libc::c_schar as libc::c_int) as libc::c_int
                    }
                }
            }) != 0
                || (0 as libc::c_int
                    * (t as libc::c_schar as libc::c_int
                        - t0 as libc::c_schar as libc::c_int) - 1 as libc::c_int)
                    < 0 as libc::c_int
                    && (t as libc::c_schar as libc::c_int
                        - t0 as libc::c_schar as libc::c_int)
                        < -(127 as libc::c_int) - 1 as libc::c_int
                || (127 as libc::c_int)
                    < t as libc::c_schar as libc::c_int
                        - t0 as libc::c_schar as libc::c_int
            {
                dt = (t as libc::c_schar as libc::c_uint)
                    .wrapping_sub(t0 as libc::c_schar as libc::c_uint) as libc::c_schar
                    as long_int;
            } else {
                dt = (t as libc::c_schar as libc::c_uint)
                    .wrapping_sub(t0 as libc::c_schar as libc::c_uint) as libc::c_schar
                    as long_int;
            };
        } else {
            if (if (if (0 as libc::c_int as libc::c_long
                * (0 as libc::c_int as libc::c_long * t0 + t)
                - 1 as libc::c_int as libc::c_long) < 0 as libc::c_int as libc::c_long
            {
                !((((0 as libc::c_int as libc::c_long
                    * (0 as libc::c_int as libc::c_long * t0 + t)
                    + 1 as libc::c_int as libc::c_long)
                    << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                    - 1 as libc::c_int as libc::c_long)
                    * 2 as libc::c_int as libc::c_long
                    + 1 as libc::c_int as libc::c_long)
            } else {
                0 as libc::c_int as libc::c_long
                    * (0 as libc::c_int as libc::c_long * t0 + t)
                    + 0 as libc::c_int as libc::c_long
            }) < 0 as libc::c_int as libc::c_long
            {
                if t0 < 0 as libc::c_int as libc::c_long {
                    ((if (0 as libc::c_int as libc::c_long
                        * (0 as libc::c_int as libc::c_long * t0 + t)
                        - 1 as libc::c_int as libc::c_long)
                        < 0 as libc::c_int as libc::c_long
                    {
                        (((0 as libc::c_int as libc::c_long
                            * (0 as libc::c_int as libc::c_long * t0 + t)
                            + 1 as libc::c_int as libc::c_long)
                            << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                            - 1 as libc::c_int as libc::c_long)
                            * 2 as libc::c_int as libc::c_long
                            + 1 as libc::c_int as libc::c_long
                    } else {
                        0 as libc::c_int as libc::c_long
                            * (0 as libc::c_int as libc::c_long * t0 + t)
                            - 1 as libc::c_int as libc::c_long
                    }) + t0 < t) as libc::c_int
                } else {
                    (t
                        < (if (0 as libc::c_int as libc::c_long
                            * (0 as libc::c_int as libc::c_long * t0 + t)
                            - 1 as libc::c_int as libc::c_long)
                            < 0 as libc::c_int as libc::c_long
                        {
                            !((((0 as libc::c_int as libc::c_long
                                * (0 as libc::c_int as libc::c_long * t0 + t)
                                + 1 as libc::c_int as libc::c_long)
                                << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                - 1 as libc::c_int as libc::c_long)
                                * 2 as libc::c_int as libc::c_long
                                + 1 as libc::c_int as libc::c_long)
                        } else {
                            0 as libc::c_int as libc::c_long
                                * (0 as libc::c_int as libc::c_long * t0 + t)
                                + 0 as libc::c_int as libc::c_long
                        }) + t0) as libc::c_int
                }
            } else {
                if t < 0 as libc::c_int as libc::c_long {
                    1 as libc::c_int
                } else {
                    if t0 < 0 as libc::c_int as libc::c_long {
                        (t - t0 <= t) as libc::c_int
                    } else {
                        (t < t0) as libc::c_int
                    }
                }
            }) != 0
                || (0 as libc::c_int as libc::c_long * (t - t0)
                    - 1 as libc::c_int as libc::c_long)
                    < 0 as libc::c_int as libc::c_long
                    && t - t0
                        < (-(127 as libc::c_int) - 1 as libc::c_int) as libc::c_long
                || (127 as libc::c_int as libc::c_long) < t - t0
            {
                dt = (t as libc::c_uint).wrapping_sub(t0 as libc::c_uint)
                    as libc::c_schar as long_int;
            } else {
                dt = (t as libc::c_uint).wrapping_sub(t0 as libc::c_uint)
                    as libc::c_schar as long_int;
            };
        };
    } else {
        if ::std::mem::size_of::<long_int>() as libc::c_ulong
            == ::std::mem::size_of::<libc::c_short>() as libc::c_ulong
        {
            if (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                < ::std::mem::size_of::<libc::c_short>() as libc::c_ulong
            {
                if (if (if (0 as libc::c_int
                    * (0 as libc::c_int * t0 as libc::c_short as libc::c_int
                        + t as libc::c_short as libc::c_int) - 1 as libc::c_int)
                    < 0 as libc::c_int
                {
                    !((((0 as libc::c_int
                        * (0 as libc::c_int * t0 as libc::c_short as libc::c_int
                            + t as libc::c_short as libc::c_int) + 1 as libc::c_int)
                        << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                        - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                } else {
                    0 as libc::c_int
                        * (0 as libc::c_int * t0 as libc::c_short as libc::c_int
                            + t as libc::c_short as libc::c_int) + 0 as libc::c_int
                }) < 0 as libc::c_int
                {
                    if (t0 as libc::c_short as libc::c_int) < 0 as libc::c_int {
                        (((if (0 as libc::c_int
                            * (0 as libc::c_int * t0 as libc::c_short as libc::c_int
                                + t as libc::c_short as libc::c_int) - 1 as libc::c_int)
                            < 0 as libc::c_int
                        {
                            (((0 as libc::c_int
                                * (0 as libc::c_int * t0 as libc::c_short as libc::c_int
                                    + t as libc::c_short as libc::c_int) + 1 as libc::c_int)
                                << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                        } else {
                            0 as libc::c_int
                                * (0 as libc::c_int * t0 as libc::c_short as libc::c_int
                                    + t as libc::c_short as libc::c_int) - 1 as libc::c_int
                        }) + t0 as libc::c_short as libc::c_int)
                            < t as libc::c_short as libc::c_int) as libc::c_int
                    } else {
                        ((t as libc::c_short as libc::c_int)
                            < (if (0 as libc::c_int
                                * (0 as libc::c_int * t0 as libc::c_short as libc::c_int
                                    + t as libc::c_short as libc::c_int) - 1 as libc::c_int)
                                < 0 as libc::c_int
                            {
                                !((((0 as libc::c_int
                                    * (0 as libc::c_int * t0 as libc::c_short as libc::c_int
                                        + t as libc::c_short as libc::c_int) + 1 as libc::c_int)
                                    << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                    - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                            } else {
                                0 as libc::c_int
                                    * (0 as libc::c_int * t0 as libc::c_short as libc::c_int
                                        + t as libc::c_short as libc::c_int) + 0 as libc::c_int
                            }) + t0 as libc::c_short as libc::c_int) as libc::c_int
                    }
                } else {
                    if (t as libc::c_short as libc::c_int) < 0 as libc::c_int {
                        1 as libc::c_int
                    } else {
                        if (t0 as libc::c_short as libc::c_int) < 0 as libc::c_int {
                            (t as libc::c_short as libc::c_int
                                - t0 as libc::c_short as libc::c_int
                                <= t as libc::c_short as libc::c_int) as libc::c_int
                        } else {
                            ((t as libc::c_short as libc::c_int)
                                < t0 as libc::c_short as libc::c_int) as libc::c_int
                        }
                    }
                }) != 0
                    || (0 as libc::c_int
                        * (t as libc::c_short as libc::c_int
                            - t0 as libc::c_short as libc::c_int) - 1 as libc::c_int)
                        < 0 as libc::c_int
                        && (t as libc::c_short as libc::c_int
                            - t0 as libc::c_short as libc::c_int)
                            < -(32767 as libc::c_int) - 1 as libc::c_int
                    || (32767 as libc::c_int)
                        < t as libc::c_short as libc::c_int
                            - t0 as libc::c_short as libc::c_int
                {
                    dt = (t as libc::c_short as libc::c_uint)
                        .wrapping_sub(t0 as libc::c_short as libc::c_uint)
                        as libc::c_short as long_int;
                } else {
                    dt = (t as libc::c_short as libc::c_uint)
                        .wrapping_sub(t0 as libc::c_short as libc::c_uint)
                        as libc::c_short as long_int;
                };
            } else {
                if (if (if (0 as libc::c_int as libc::c_long
                    * (0 as libc::c_int as libc::c_long * t0 + t)
                    - 1 as libc::c_int as libc::c_long)
                    < 0 as libc::c_int as libc::c_long
                {
                    !((((0 as libc::c_int as libc::c_long
                        * (0 as libc::c_int as libc::c_long * t0 + t)
                        + 1 as libc::c_int as libc::c_long)
                        << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                        - 1 as libc::c_int as libc::c_long)
                        * 2 as libc::c_int as libc::c_long
                        + 1 as libc::c_int as libc::c_long)
                } else {
                    0 as libc::c_int as libc::c_long
                        * (0 as libc::c_int as libc::c_long * t0 + t)
                        + 0 as libc::c_int as libc::c_long
                }) < 0 as libc::c_int as libc::c_long
                {
                    if t0 < 0 as libc::c_int as libc::c_long {
                        ((if (0 as libc::c_int as libc::c_long
                            * (0 as libc::c_int as libc::c_long * t0 + t)
                            - 1 as libc::c_int as libc::c_long)
                            < 0 as libc::c_int as libc::c_long
                        {
                            (((0 as libc::c_int as libc::c_long
                                * (0 as libc::c_int as libc::c_long * t0 + t)
                                + 1 as libc::c_int as libc::c_long)
                                << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                - 1 as libc::c_int as libc::c_long)
                                * 2 as libc::c_int as libc::c_long
                                + 1 as libc::c_int as libc::c_long
                        } else {
                            0 as libc::c_int as libc::c_long
                                * (0 as libc::c_int as libc::c_long * t0 + t)
                                - 1 as libc::c_int as libc::c_long
                        }) + t0 < t) as libc::c_int
                    } else {
                        (t
                            < (if (0 as libc::c_int as libc::c_long
                                * (0 as libc::c_int as libc::c_long * t0 + t)
                                - 1 as libc::c_int as libc::c_long)
                                < 0 as libc::c_int as libc::c_long
                            {
                                !((((0 as libc::c_int as libc::c_long
                                    * (0 as libc::c_int as libc::c_long * t0 + t)
                                    + 1 as libc::c_int as libc::c_long)
                                    << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                    - 1 as libc::c_int as libc::c_long)
                                    * 2 as libc::c_int as libc::c_long
                                    + 1 as libc::c_int as libc::c_long)
                            } else {
                                0 as libc::c_int as libc::c_long
                                    * (0 as libc::c_int as libc::c_long * t0 + t)
                                    + 0 as libc::c_int as libc::c_long
                            }) + t0) as libc::c_int
                    }
                } else {
                    if t < 0 as libc::c_int as libc::c_long {
                        1 as libc::c_int
                    } else {
                        if t0 < 0 as libc::c_int as libc::c_long {
                            (t - t0 <= t) as libc::c_int
                        } else {
                            (t < t0) as libc::c_int
                        }
                    }
                }) != 0
                    || (0 as libc::c_int as libc::c_long * (t - t0)
                        - 1 as libc::c_int as libc::c_long)
                        < 0 as libc::c_int as libc::c_long
                        && t - t0
                            < (-(32767 as libc::c_int) - 1 as libc::c_int)
                                as libc::c_long
                    || (32767 as libc::c_int as libc::c_long) < t - t0
                {
                    dt = (t as libc::c_uint).wrapping_sub(t0 as libc::c_uint)
                        as libc::c_short as long_int;
                } else {
                    dt = (t as libc::c_uint).wrapping_sub(t0 as libc::c_uint)
                        as libc::c_short as long_int;
                };
            };
        } else {
            if ::std::mem::size_of::<long_int>() as libc::c_ulong
                == ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
            {
                if (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                    < ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                {
                    if (if (if (0 as libc::c_int
                        * (0 as libc::c_int * t0 as libc::c_int + t as libc::c_int)
                        - 1 as libc::c_int) < 0 as libc::c_int
                    {
                        !((((0 as libc::c_int
                            * (0 as libc::c_int * t0 as libc::c_int + t as libc::c_int)
                            + 1 as libc::c_int)
                            << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                            - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                    } else {
                        0 as libc::c_int
                            * (0 as libc::c_int * t0 as libc::c_int + t as libc::c_int)
                            + 0 as libc::c_int
                    }) < 0 as libc::c_int
                    {
                        if (t0 as libc::c_int) < 0 as libc::c_int {
                            (((if (0 as libc::c_int
                                * (0 as libc::c_int * t0 as libc::c_int + t as libc::c_int)
                                - 1 as libc::c_int) < 0 as libc::c_int
                            {
                                (((0 as libc::c_int
                                    * (0 as libc::c_int * t0 as libc::c_int + t as libc::c_int)
                                    + 1 as libc::c_int)
                                    << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                    - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                            } else {
                                0 as libc::c_int
                                    * (0 as libc::c_int * t0 as libc::c_int + t as libc::c_int)
                                    - 1 as libc::c_int
                            }) + t0 as libc::c_int) < t as libc::c_int) as libc::c_int
                        } else {
                            ((t as libc::c_int)
                                < (if (0 as libc::c_int
                                    * (0 as libc::c_int * t0 as libc::c_int + t as libc::c_int)
                                    - 1 as libc::c_int) < 0 as libc::c_int
                                {
                                    !((((0 as libc::c_int
                                        * (0 as libc::c_int * t0 as libc::c_int + t as libc::c_int)
                                        + 1 as libc::c_int)
                                        << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                        - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                                } else {
                                    0 as libc::c_int
                                        * (0 as libc::c_int * t0 as libc::c_int + t as libc::c_int)
                                        + 0 as libc::c_int
                                }) + t0 as libc::c_int) as libc::c_int
                        }
                    } else {
                        if (t as libc::c_int) < 0 as libc::c_int {
                            1 as libc::c_int
                        } else {
                            if (t0 as libc::c_int) < 0 as libc::c_int {
                                (t as libc::c_int - t0 as libc::c_int <= t as libc::c_int)
                                    as libc::c_int
                            } else {
                                ((t as libc::c_int) < t0 as libc::c_int) as libc::c_int
                            }
                        }
                    }) != 0
                        || (0 as libc::c_int * (t as libc::c_int - t0 as libc::c_int)
                            - 1 as libc::c_int) < 0 as libc::c_int
                            && (t as libc::c_int - t0 as libc::c_int)
                                < -(2147483647 as libc::c_int) - 1 as libc::c_int
                        || (2147483647 as libc::c_int)
                            < t as libc::c_int - t0 as libc::c_int
                    {
                        dt = (t as libc::c_int as libc::c_uint)
                            .wrapping_sub(t0 as libc::c_int as libc::c_uint)
                            as libc::c_int as long_int;
                    } else {
                        dt = (t as libc::c_int as libc::c_uint)
                            .wrapping_sub(t0 as libc::c_int as libc::c_uint)
                            as libc::c_int as long_int;
                    };
                } else {
                    if (if (if (0 as libc::c_int as libc::c_long
                        * (0 as libc::c_int as libc::c_long * t0 + t)
                        - 1 as libc::c_int as libc::c_long)
                        < 0 as libc::c_int as libc::c_long
                    {
                        !((((0 as libc::c_int as libc::c_long
                            * (0 as libc::c_int as libc::c_long * t0 + t)
                            + 1 as libc::c_int as libc::c_long)
                            << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                            - 1 as libc::c_int as libc::c_long)
                            * 2 as libc::c_int as libc::c_long
                            + 1 as libc::c_int as libc::c_long)
                    } else {
                        0 as libc::c_int as libc::c_long
                            * (0 as libc::c_int as libc::c_long * t0 + t)
                            + 0 as libc::c_int as libc::c_long
                    }) < 0 as libc::c_int as libc::c_long
                    {
                        if t0 < 0 as libc::c_int as libc::c_long {
                            ((if (0 as libc::c_int as libc::c_long
                                * (0 as libc::c_int as libc::c_long * t0 + t)
                                - 1 as libc::c_int as libc::c_long)
                                < 0 as libc::c_int as libc::c_long
                            {
                                (((0 as libc::c_int as libc::c_long
                                    * (0 as libc::c_int as libc::c_long * t0 + t)
                                    + 1 as libc::c_int as libc::c_long)
                                    << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                    - 1 as libc::c_int as libc::c_long)
                                    * 2 as libc::c_int as libc::c_long
                                    + 1 as libc::c_int as libc::c_long
                            } else {
                                0 as libc::c_int as libc::c_long
                                    * (0 as libc::c_int as libc::c_long * t0 + t)
                                    - 1 as libc::c_int as libc::c_long
                            }) + t0 < t) as libc::c_int
                        } else {
                            (t
                                < (if (0 as libc::c_int as libc::c_long
                                    * (0 as libc::c_int as libc::c_long * t0 + t)
                                    - 1 as libc::c_int as libc::c_long)
                                    < 0 as libc::c_int as libc::c_long
                                {
                                    !((((0 as libc::c_int as libc::c_long
                                        * (0 as libc::c_int as libc::c_long * t0 + t)
                                        + 1 as libc::c_int as libc::c_long)
                                        << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                        - 1 as libc::c_int as libc::c_long)
                                        * 2 as libc::c_int as libc::c_long
                                        + 1 as libc::c_int as libc::c_long)
                                } else {
                                    0 as libc::c_int as libc::c_long
                                        * (0 as libc::c_int as libc::c_long * t0 + t)
                                        + 0 as libc::c_int as libc::c_long
                                }) + t0) as libc::c_int
                        }
                    } else {
                        if t < 0 as libc::c_int as libc::c_long {
                            1 as libc::c_int
                        } else {
                            if t0 < 0 as libc::c_int as libc::c_long {
                                (t - t0 <= t) as libc::c_int
                            } else {
                                (t < t0) as libc::c_int
                            }
                        }
                    }) != 0
                        || (0 as libc::c_int as libc::c_long * (t - t0)
                            - 1 as libc::c_int as libc::c_long)
                            < 0 as libc::c_int as libc::c_long
                            && t - t0
                                < (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                    as libc::c_long
                        || (2147483647 as libc::c_int as libc::c_long) < t - t0
                    {
                        dt = (t as libc::c_uint).wrapping_sub(t0 as libc::c_uint)
                            as libc::c_int as long_int;
                    } else {
                        dt = (t as libc::c_uint).wrapping_sub(t0 as libc::c_uint)
                            as libc::c_int as long_int;
                    };
                };
            } else {
                if ::std::mem::size_of::<long_int>() as libc::c_ulong
                    == ::std::mem::size_of::<libc::c_long>() as libc::c_ulong
                {
                    if (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                        < ::std::mem::size_of::<libc::c_long>() as libc::c_ulong
                    {
                        if (if (if (0 as libc::c_int as libc::c_long
                            * (0 as libc::c_int as libc::c_long * t0 + t)
                            - 1 as libc::c_int as libc::c_long)
                            < 0 as libc::c_int as libc::c_long
                        {
                            !((((0 as libc::c_int as libc::c_long
                                * (0 as libc::c_int as libc::c_long * t0 + t)
                                + 1 as libc::c_int as libc::c_long)
                                << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                - 1 as libc::c_int as libc::c_long)
                                * 2 as libc::c_int as libc::c_long
                                + 1 as libc::c_int as libc::c_long)
                        } else {
                            0 as libc::c_int as libc::c_long
                                * (0 as libc::c_int as libc::c_long * t0 + t)
                                + 0 as libc::c_int as libc::c_long
                        }) < 0 as libc::c_int as libc::c_long
                        {
                            if t0 < 0 as libc::c_int as libc::c_long {
                                ((if (0 as libc::c_int as libc::c_long
                                    * (0 as libc::c_int as libc::c_long * t0 + t)
                                    - 1 as libc::c_int as libc::c_long)
                                    < 0 as libc::c_int as libc::c_long
                                {
                                    (((0 as libc::c_int as libc::c_long
                                        * (0 as libc::c_int as libc::c_long * t0 + t)
                                        + 1 as libc::c_int as libc::c_long)
                                        << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                        - 1 as libc::c_int as libc::c_long)
                                        * 2 as libc::c_int as libc::c_long
                                        + 1 as libc::c_int as libc::c_long
                                } else {
                                    0 as libc::c_int as libc::c_long
                                        * (0 as libc::c_int as libc::c_long * t0 + t)
                                        - 1 as libc::c_int as libc::c_long
                                }) + t0 < t) as libc::c_int
                            } else {
                                (t
                                    < (if (0 as libc::c_int as libc::c_long
                                        * (0 as libc::c_int as libc::c_long * t0 + t)
                                        - 1 as libc::c_int as libc::c_long)
                                        < 0 as libc::c_int as libc::c_long
                                    {
                                        !((((0 as libc::c_int as libc::c_long
                                            * (0 as libc::c_int as libc::c_long * t0 + t)
                                            + 1 as libc::c_int as libc::c_long)
                                            << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                            - 1 as libc::c_int as libc::c_long)
                                            * 2 as libc::c_int as libc::c_long
                                            + 1 as libc::c_int as libc::c_long)
                                    } else {
                                        0 as libc::c_int as libc::c_long
                                            * (0 as libc::c_int as libc::c_long * t0 + t)
                                            + 0 as libc::c_int as libc::c_long
                                    }) + t0) as libc::c_int
                            }
                        } else {
                            if t < 0 as libc::c_int as libc::c_long {
                                1 as libc::c_int
                            } else {
                                if t0 < 0 as libc::c_int as libc::c_long {
                                    (t - t0 <= t) as libc::c_int
                                } else {
                                    (t < t0) as libc::c_int
                                }
                            }
                        }) != 0
                            || (0 as libc::c_int as libc::c_long * (t - t0)
                                - 1 as libc::c_int as libc::c_long)
                                < 0 as libc::c_int as libc::c_long
                                && t - t0
                                    < -(9223372036854775807 as libc::c_long) - 1 as libc::c_long
                            || (9223372036854775807 as libc::c_long) < t - t0
                        {
                            dt = (t as libc::c_ulong).wrapping_sub(t0 as libc::c_ulong)
                                as libc::c_long;
                        } else {
                            dt = (t as libc::c_ulong).wrapping_sub(t0 as libc::c_ulong)
                                as libc::c_long;
                        };
                    } else {
                        if (if (if (0 as libc::c_int as libc::c_long
                            * (0 as libc::c_int as libc::c_long * t0 + t)
                            - 1 as libc::c_int as libc::c_long)
                            < 0 as libc::c_int as libc::c_long
                        {
                            !((((0 as libc::c_int as libc::c_long
                                * (0 as libc::c_int as libc::c_long * t0 + t)
                                + 1 as libc::c_int as libc::c_long)
                                << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                - 1 as libc::c_int as libc::c_long)
                                * 2 as libc::c_int as libc::c_long
                                + 1 as libc::c_int as libc::c_long)
                        } else {
                            0 as libc::c_int as libc::c_long
                                * (0 as libc::c_int as libc::c_long * t0 + t)
                                + 0 as libc::c_int as libc::c_long
                        }) < 0 as libc::c_int as libc::c_long
                        {
                            if t0 < 0 as libc::c_int as libc::c_long {
                                ((if (0 as libc::c_int as libc::c_long
                                    * (0 as libc::c_int as libc::c_long * t0 + t)
                                    - 1 as libc::c_int as libc::c_long)
                                    < 0 as libc::c_int as libc::c_long
                                {
                                    (((0 as libc::c_int as libc::c_long
                                        * (0 as libc::c_int as libc::c_long * t0 + t)
                                        + 1 as libc::c_int as libc::c_long)
                                        << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                        - 1 as libc::c_int as libc::c_long)
                                        * 2 as libc::c_int as libc::c_long
                                        + 1 as libc::c_int as libc::c_long
                                } else {
                                    0 as libc::c_int as libc::c_long
                                        * (0 as libc::c_int as libc::c_long * t0 + t)
                                        - 1 as libc::c_int as libc::c_long
                                }) + t0 < t) as libc::c_int
                            } else {
                                (t
                                    < (if (0 as libc::c_int as libc::c_long
                                        * (0 as libc::c_int as libc::c_long * t0 + t)
                                        - 1 as libc::c_int as libc::c_long)
                                        < 0 as libc::c_int as libc::c_long
                                    {
                                        !((((0 as libc::c_int as libc::c_long
                                            * (0 as libc::c_int as libc::c_long * t0 + t)
                                            + 1 as libc::c_int as libc::c_long)
                                            << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                            - 1 as libc::c_int as libc::c_long)
                                            * 2 as libc::c_int as libc::c_long
                                            + 1 as libc::c_int as libc::c_long)
                                    } else {
                                        0 as libc::c_int as libc::c_long
                                            * (0 as libc::c_int as libc::c_long * t0 + t)
                                            + 0 as libc::c_int as libc::c_long
                                    }) + t0) as libc::c_int
                            }
                        } else {
                            if t < 0 as libc::c_int as libc::c_long {
                                1 as libc::c_int
                            } else {
                                if t0 < 0 as libc::c_int as libc::c_long {
                                    (t - t0 <= t) as libc::c_int
                                } else {
                                    (t < t0) as libc::c_int
                                }
                            }
                        }) != 0
                            || (0 as libc::c_int as libc::c_long * (t - t0)
                                - 1 as libc::c_int as libc::c_long)
                                < 0 as libc::c_int as libc::c_long
                                && t - t0
                                    < -(9223372036854775807 as libc::c_long) - 1 as libc::c_long
                            || (9223372036854775807 as libc::c_long) < t - t0
                        {
                            dt = (t as libc::c_ulong).wrapping_sub(t0 as libc::c_ulong)
                                as libc::c_long;
                        } else {
                            dt = (t as libc::c_ulong).wrapping_sub(t0 as libc::c_ulong)
                                as libc::c_long;
                        };
                    };
                } else {
                    if (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                        < ::std::mem::size_of::<libc::c_longlong>() as libc::c_ulong
                    {
                        if (if (if (0 as libc::c_int as libc::c_longlong
                            * (0 as libc::c_int as libc::c_longlong
                                * t0 as libc::c_longlong + t as libc::c_longlong)
                            - 1 as libc::c_int as libc::c_longlong)
                            < 0 as libc::c_int as libc::c_longlong
                        {
                            !((((0 as libc::c_int as libc::c_longlong
                                * (0 as libc::c_int as libc::c_longlong
                                    * t0 as libc::c_longlong + t as libc::c_longlong)
                                + 1 as libc::c_int as libc::c_longlong)
                                << (::std::mem::size_of::<libc::c_longlong>()
                                    as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                - 1 as libc::c_int as libc::c_longlong)
                                * 2 as libc::c_int as libc::c_longlong
                                + 1 as libc::c_int as libc::c_longlong)
                        } else {
                            0 as libc::c_int as libc::c_longlong
                                * (0 as libc::c_int as libc::c_longlong
                                    * t0 as libc::c_longlong + t as libc::c_longlong)
                                + 0 as libc::c_int as libc::c_longlong
                        }) < 0 as libc::c_int as libc::c_longlong
                        {
                            if (t0 as libc::c_longlong)
                                < 0 as libc::c_int as libc::c_longlong
                            {
                                (((if (0 as libc::c_int as libc::c_longlong
                                    * (0 as libc::c_int as libc::c_longlong
                                        * t0 as libc::c_longlong + t as libc::c_longlong)
                                    - 1 as libc::c_int as libc::c_longlong)
                                    < 0 as libc::c_int as libc::c_longlong
                                {
                                    (((0 as libc::c_int as libc::c_longlong
                                        * (0 as libc::c_int as libc::c_longlong
                                            * t0 as libc::c_longlong + t as libc::c_longlong)
                                        + 1 as libc::c_int as libc::c_longlong)
                                        << (::std::mem::size_of::<libc::c_longlong>()
                                            as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                        - 1 as libc::c_int as libc::c_longlong)
                                        * 2 as libc::c_int as libc::c_longlong
                                        + 1 as libc::c_int as libc::c_longlong
                                } else {
                                    0 as libc::c_int as libc::c_longlong
                                        * (0 as libc::c_int as libc::c_longlong
                                            * t0 as libc::c_longlong + t as libc::c_longlong)
                                        - 1 as libc::c_int as libc::c_longlong
                                }) + t0 as libc::c_longlong) < t as libc::c_longlong)
                                    as libc::c_int
                            } else {
                                ((t as libc::c_longlong)
                                    < (if (0 as libc::c_int as libc::c_longlong
                                        * (0 as libc::c_int as libc::c_longlong
                                            * t0 as libc::c_longlong + t as libc::c_longlong)
                                        - 1 as libc::c_int as libc::c_longlong)
                                        < 0 as libc::c_int as libc::c_longlong
                                    {
                                        !((((0 as libc::c_int as libc::c_longlong
                                            * (0 as libc::c_int as libc::c_longlong
                                                * t0 as libc::c_longlong + t as libc::c_longlong)
                                            + 1 as libc::c_int as libc::c_longlong)
                                            << (::std::mem::size_of::<libc::c_longlong>()
                                                as libc::c_ulong)
                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                            - 1 as libc::c_int as libc::c_longlong)
                                            * 2 as libc::c_int as libc::c_longlong
                                            + 1 as libc::c_int as libc::c_longlong)
                                    } else {
                                        0 as libc::c_int as libc::c_longlong
                                            * (0 as libc::c_int as libc::c_longlong
                                                * t0 as libc::c_longlong + t as libc::c_longlong)
                                            + 0 as libc::c_int as libc::c_longlong
                                    }) + t0 as libc::c_longlong) as libc::c_int
                            }
                        } else {
                            if (t as libc::c_longlong)
                                < 0 as libc::c_int as libc::c_longlong
                            {
                                1 as libc::c_int
                            } else {
                                if (t0 as libc::c_longlong)
                                    < 0 as libc::c_int as libc::c_longlong
                                {
                                    (t as libc::c_longlong - t0 as libc::c_longlong
                                        <= t as libc::c_longlong) as libc::c_int
                                } else {
                                    ((t as libc::c_longlong) < t0 as libc::c_longlong)
                                        as libc::c_int
                                }
                            }
                        }) != 0
                            || (0 as libc::c_int as libc::c_longlong
                                * (t as libc::c_longlong - t0 as libc::c_longlong)
                                - 1 as libc::c_int as libc::c_longlong)
                                < 0 as libc::c_int as libc::c_longlong
                                && (t as libc::c_longlong - t0 as libc::c_longlong)
                                    < -(9223372036854775807 as libc::c_longlong)
                                        - 1 as libc::c_longlong
                            || (9223372036854775807 as libc::c_longlong)
                                < t as libc::c_longlong - t0 as libc::c_longlong
                        {
                            dt = (t as libc::c_longlong as libc::c_ulonglong)
                                .wrapping_sub(t0 as libc::c_longlong as libc::c_ulonglong)
                                as libc::c_longlong as long_int;
                        } else {
                            dt = (t as libc::c_longlong as libc::c_ulonglong)
                                .wrapping_sub(t0 as libc::c_longlong as libc::c_ulonglong)
                                as libc::c_longlong as long_int;
                        };
                    } else {
                        if (if (if (0 as libc::c_int as libc::c_long
                            * (0 as libc::c_int as libc::c_long * t0 + t)
                            - 1 as libc::c_int as libc::c_long)
                            < 0 as libc::c_int as libc::c_long
                        {
                            !((((0 as libc::c_int as libc::c_long
                                * (0 as libc::c_int as libc::c_long * t0 + t)
                                + 1 as libc::c_int as libc::c_long)
                                << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                - 1 as libc::c_int as libc::c_long)
                                * 2 as libc::c_int as libc::c_long
                                + 1 as libc::c_int as libc::c_long)
                        } else {
                            0 as libc::c_int as libc::c_long
                                * (0 as libc::c_int as libc::c_long * t0 + t)
                                + 0 as libc::c_int as libc::c_long
                        }) < 0 as libc::c_int as libc::c_long
                        {
                            if t0 < 0 as libc::c_int as libc::c_long {
                                ((if (0 as libc::c_int as libc::c_long
                                    * (0 as libc::c_int as libc::c_long * t0 + t)
                                    - 1 as libc::c_int as libc::c_long)
                                    < 0 as libc::c_int as libc::c_long
                                {
                                    (((0 as libc::c_int as libc::c_long
                                        * (0 as libc::c_int as libc::c_long * t0 + t)
                                        + 1 as libc::c_int as libc::c_long)
                                        << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                        - 1 as libc::c_int as libc::c_long)
                                        * 2 as libc::c_int as libc::c_long
                                        + 1 as libc::c_int as libc::c_long
                                } else {
                                    0 as libc::c_int as libc::c_long
                                        * (0 as libc::c_int as libc::c_long * t0 + t)
                                        - 1 as libc::c_int as libc::c_long
                                }) + t0 < t) as libc::c_int
                            } else {
                                (t
                                    < (if (0 as libc::c_int as libc::c_long
                                        * (0 as libc::c_int as libc::c_long * t0 + t)
                                        - 1 as libc::c_int as libc::c_long)
                                        < 0 as libc::c_int as libc::c_long
                                    {
                                        !((((0 as libc::c_int as libc::c_long
                                            * (0 as libc::c_int as libc::c_long * t0 + t)
                                            + 1 as libc::c_int as libc::c_long)
                                            << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                            - 1 as libc::c_int as libc::c_long)
                                            * 2 as libc::c_int as libc::c_long
                                            + 1 as libc::c_int as libc::c_long)
                                    } else {
                                        0 as libc::c_int as libc::c_long
                                            * (0 as libc::c_int as libc::c_long * t0 + t)
                                            + 0 as libc::c_int as libc::c_long
                                    }) + t0) as libc::c_int
                            }
                        } else {
                            if t < 0 as libc::c_int as libc::c_long {
                                1 as libc::c_int
                            } else {
                                if t0 < 0 as libc::c_int as libc::c_long {
                                    (t - t0 <= t) as libc::c_int
                                } else {
                                    (t < t0) as libc::c_int
                                }
                            }
                        }) != 0
                            || (0 as libc::c_int as libc::c_long * (t - t0)
                                - 1 as libc::c_int as libc::c_long)
                                < 0 as libc::c_int as libc::c_long
                                && ((t - t0) as libc::c_longlong)
                                    < -(9223372036854775807 as libc::c_longlong)
                                        - 1 as libc::c_longlong
                            || (9223372036854775807 as libc::c_longlong)
                                < (t - t0) as libc::c_longlong
                        {
                            dt = (t as libc::c_ulonglong)
                                .wrapping_sub(t0 as libc::c_ulonglong) as libc::c_longlong
                                as long_int;
                        } else {
                            dt = (t as libc::c_ulonglong)
                                .wrapping_sub(t0 as libc::c_ulonglong) as libc::c_longlong
                                as long_int;
                        };
                    };
                };
            };
        };
    };
    if ::std::mem::size_of::<mktime_offset_t>() as libc::c_ulong
        == ::std::mem::size_of::<libc::c_schar>() as libc::c_ulong
    {
        if (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
            < ::std::mem::size_of::<libc::c_schar>() as libc::c_ulong
        {
            if (if (if (0 as libc::c_int
                * (0 as libc::c_int
                    * negative_offset_guess as libc::c_schar as libc::c_int
                    + dt as libc::c_schar as libc::c_int) - 1 as libc::c_int)
                < 0 as libc::c_int
            {
                !((((0 as libc::c_int
                    * (0 as libc::c_int
                        * negative_offset_guess as libc::c_schar as libc::c_int
                        + dt as libc::c_schar as libc::c_int) + 1 as libc::c_int)
                    << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                    - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
            } else {
                0 as libc::c_int
                    * (0 as libc::c_int
                        * negative_offset_guess as libc::c_schar as libc::c_int
                        + dt as libc::c_schar as libc::c_int) + 0 as libc::c_int
            }) < 0 as libc::c_int
            {
                if (negative_offset_guess as libc::c_schar as libc::c_int)
                    < 0 as libc::c_int
                {
                    (((if (0 as libc::c_int
                        * (0 as libc::c_int
                            * negative_offset_guess as libc::c_schar as libc::c_int
                            + dt as libc::c_schar as libc::c_int) - 1 as libc::c_int)
                        < 0 as libc::c_int
                    {
                        (((0 as libc::c_int
                            * (0 as libc::c_int
                                * negative_offset_guess as libc::c_schar as libc::c_int
                                + dt as libc::c_schar as libc::c_int) + 1 as libc::c_int)
                            << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                            - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                    } else {
                        0 as libc::c_int
                            * (0 as libc::c_int
                                * negative_offset_guess as libc::c_schar as libc::c_int
                                + dt as libc::c_schar as libc::c_int) - 1 as libc::c_int
                    }) + negative_offset_guess as libc::c_schar as libc::c_int)
                        < dt as libc::c_schar as libc::c_int) as libc::c_int
                } else {
                    ((dt as libc::c_schar as libc::c_int)
                        < (if (0 as libc::c_int
                            * (0 as libc::c_int
                                * negative_offset_guess as libc::c_schar as libc::c_int
                                + dt as libc::c_schar as libc::c_int) - 1 as libc::c_int)
                            < 0 as libc::c_int
                        {
                            !((((0 as libc::c_int
                                * (0 as libc::c_int
                                    * negative_offset_guess as libc::c_schar as libc::c_int
                                    + dt as libc::c_schar as libc::c_int) + 1 as libc::c_int)
                                << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                        } else {
                            0 as libc::c_int
                                * (0 as libc::c_int
                                    * negative_offset_guess as libc::c_schar as libc::c_int
                                    + dt as libc::c_schar as libc::c_int) + 0 as libc::c_int
                        }) + negative_offset_guess as libc::c_schar as libc::c_int)
                        as libc::c_int
                }
            } else {
                if (dt as libc::c_schar as libc::c_int) < 0 as libc::c_int {
                    1 as libc::c_int
                } else {
                    if (negative_offset_guess as libc::c_schar as libc::c_int)
                        < 0 as libc::c_int
                    {
                        (dt as libc::c_schar as libc::c_int
                            - negative_offset_guess as libc::c_schar as libc::c_int
                            <= dt as libc::c_schar as libc::c_int) as libc::c_int
                    } else {
                        ((dt as libc::c_schar as libc::c_int)
                            < negative_offset_guess as libc::c_schar as libc::c_int)
                            as libc::c_int
                    }
                }
            }) != 0
                || (0 as libc::c_int
                    * (dt as libc::c_schar as libc::c_int
                        - negative_offset_guess as libc::c_schar as libc::c_int)
                    - 1 as libc::c_int) < 0 as libc::c_int
                    && (dt as libc::c_schar as libc::c_int
                        - negative_offset_guess as libc::c_schar as libc::c_int)
                        < -(127 as libc::c_int) - 1 as libc::c_int
                || (127 as libc::c_int)
                    < dt as libc::c_schar as libc::c_int
                        - negative_offset_guess as libc::c_schar as libc::c_int
            {
                *offset = (dt as libc::c_schar as libc::c_uint)
                    .wrapping_sub(negative_offset_guess as libc::c_schar as libc::c_uint)
                    as libc::c_schar as mktime_offset_t;
            } else {
                *offset = (dt as libc::c_schar as libc::c_uint)
                    .wrapping_sub(negative_offset_guess as libc::c_schar as libc::c_uint)
                    as libc::c_schar as mktime_offset_t;
            };
        } else {
            if (if (if (0 as libc::c_int as libc::c_long
                * ((0 as libc::c_int * negative_offset_guess) as libc::c_long + dt)
                - 1 as libc::c_int as libc::c_long) < 0 as libc::c_int as libc::c_long
            {
                !((((0 as libc::c_int as libc::c_long
                    * ((0 as libc::c_int * negative_offset_guess) as libc::c_long + dt)
                    + 1 as libc::c_int as libc::c_long)
                    << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                    - 1 as libc::c_int as libc::c_long)
                    * 2 as libc::c_int as libc::c_long
                    + 1 as libc::c_int as libc::c_long)
            } else {
                0 as libc::c_int as libc::c_long
                    * ((0 as libc::c_int * negative_offset_guess) as libc::c_long + dt)
                    + 0 as libc::c_int as libc::c_long
            }) < 0 as libc::c_int as libc::c_long
            {
                if negative_offset_guess < 0 as libc::c_int {
                    (((if (0 as libc::c_int as libc::c_long
                        * ((0 as libc::c_int * negative_offset_guess) as libc::c_long
                            + dt) - 1 as libc::c_int as libc::c_long)
                        < 0 as libc::c_int as libc::c_long
                    {
                        (((0 as libc::c_int as libc::c_long
                            * ((0 as libc::c_int * negative_offset_guess) as libc::c_long
                                + dt) + 1 as libc::c_int as libc::c_long)
                            << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                            - 1 as libc::c_int as libc::c_long)
                            * 2 as libc::c_int as libc::c_long
                            + 1 as libc::c_int as libc::c_long
                    } else {
                        0 as libc::c_int as libc::c_long
                            * ((0 as libc::c_int * negative_offset_guess) as libc::c_long
                                + dt) - 1 as libc::c_int as libc::c_long
                    }) + negative_offset_guess as libc::c_long) < dt) as libc::c_int
                } else {
                    (dt
                        < (if (0 as libc::c_int as libc::c_long
                            * ((0 as libc::c_int * negative_offset_guess) as libc::c_long
                                + dt) - 1 as libc::c_int as libc::c_long)
                            < 0 as libc::c_int as libc::c_long
                        {
                            !((((0 as libc::c_int as libc::c_long
                                * ((0 as libc::c_int * negative_offset_guess)
                                    as libc::c_long + dt) + 1 as libc::c_int as libc::c_long)
                                << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                - 1 as libc::c_int as libc::c_long)
                                * 2 as libc::c_int as libc::c_long
                                + 1 as libc::c_int as libc::c_long)
                        } else {
                            0 as libc::c_int as libc::c_long
                                * ((0 as libc::c_int * negative_offset_guess)
                                    as libc::c_long + dt) + 0 as libc::c_int as libc::c_long
                        }) + negative_offset_guess as libc::c_long) as libc::c_int
                }
            } else {
                if dt < 0 as libc::c_int as libc::c_long {
                    1 as libc::c_int
                } else {
                    if negative_offset_guess < 0 as libc::c_int {
                        (dt - negative_offset_guess as libc::c_long <= dt) as libc::c_int
                    } else {
                        (dt < negative_offset_guess as libc::c_long) as libc::c_int
                    }
                }
            }) != 0
                || (0 as libc::c_int as libc::c_long
                    * (dt - negative_offset_guess as libc::c_long)
                    - 1 as libc::c_int as libc::c_long)
                    < 0 as libc::c_int as libc::c_long
                    && (dt - negative_offset_guess as libc::c_long)
                        < (-(127 as libc::c_int) - 1 as libc::c_int) as libc::c_long
                || (127 as libc::c_int as libc::c_long)
                    < dt - negative_offset_guess as libc::c_long
            {
                *offset = (dt as libc::c_uint)
                    .wrapping_sub(negative_offset_guess as libc::c_uint) as libc::c_schar
                    as mktime_offset_t;
            } else {
                *offset = (dt as libc::c_uint)
                    .wrapping_sub(negative_offset_guess as libc::c_uint) as libc::c_schar
                    as mktime_offset_t;
            };
        };
    } else {
        if ::std::mem::size_of::<mktime_offset_t>() as libc::c_ulong
            == ::std::mem::size_of::<libc::c_short>() as libc::c_ulong
        {
            if (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                < ::std::mem::size_of::<libc::c_short>() as libc::c_ulong
            {
                if (if (if (0 as libc::c_int
                    * (0 as libc::c_int
                        * negative_offset_guess as libc::c_short as libc::c_int
                        + dt as libc::c_short as libc::c_int) - 1 as libc::c_int)
                    < 0 as libc::c_int
                {
                    !((((0 as libc::c_int
                        * (0 as libc::c_int
                            * negative_offset_guess as libc::c_short as libc::c_int
                            + dt as libc::c_short as libc::c_int) + 1 as libc::c_int)
                        << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                        - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                } else {
                    0 as libc::c_int
                        * (0 as libc::c_int
                            * negative_offset_guess as libc::c_short as libc::c_int
                            + dt as libc::c_short as libc::c_int) + 0 as libc::c_int
                }) < 0 as libc::c_int
                {
                    if (negative_offset_guess as libc::c_short as libc::c_int)
                        < 0 as libc::c_int
                    {
                        (((if (0 as libc::c_int
                            * (0 as libc::c_int
                                * negative_offset_guess as libc::c_short as libc::c_int
                                + dt as libc::c_short as libc::c_int) - 1 as libc::c_int)
                            < 0 as libc::c_int
                        {
                            (((0 as libc::c_int
                                * (0 as libc::c_int
                                    * negative_offset_guess as libc::c_short as libc::c_int
                                    + dt as libc::c_short as libc::c_int) + 1 as libc::c_int)
                                << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                        } else {
                            0 as libc::c_int
                                * (0 as libc::c_int
                                    * negative_offset_guess as libc::c_short as libc::c_int
                                    + dt as libc::c_short as libc::c_int) - 1 as libc::c_int
                        }) + negative_offset_guess as libc::c_short as libc::c_int)
                            < dt as libc::c_short as libc::c_int) as libc::c_int
                    } else {
                        ((dt as libc::c_short as libc::c_int)
                            < (if (0 as libc::c_int
                                * (0 as libc::c_int
                                    * negative_offset_guess as libc::c_short as libc::c_int
                                    + dt as libc::c_short as libc::c_int) - 1 as libc::c_int)
                                < 0 as libc::c_int
                            {
                                !((((0 as libc::c_int
                                    * (0 as libc::c_int
                                        * negative_offset_guess as libc::c_short as libc::c_int
                                        + dt as libc::c_short as libc::c_int) + 1 as libc::c_int)
                                    << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                    - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                            } else {
                                0 as libc::c_int
                                    * (0 as libc::c_int
                                        * negative_offset_guess as libc::c_short as libc::c_int
                                        + dt as libc::c_short as libc::c_int) + 0 as libc::c_int
                            }) + negative_offset_guess as libc::c_short as libc::c_int)
                            as libc::c_int
                    }
                } else {
                    if (dt as libc::c_short as libc::c_int) < 0 as libc::c_int {
                        1 as libc::c_int
                    } else {
                        if (negative_offset_guess as libc::c_short as libc::c_int)
                            < 0 as libc::c_int
                        {
                            (dt as libc::c_short as libc::c_int
                                - negative_offset_guess as libc::c_short as libc::c_int
                                <= dt as libc::c_short as libc::c_int) as libc::c_int
                        } else {
                            ((dt as libc::c_short as libc::c_int)
                                < negative_offset_guess as libc::c_short as libc::c_int)
                                as libc::c_int
                        }
                    }
                }) != 0
                    || (0 as libc::c_int
                        * (dt as libc::c_short as libc::c_int
                            - negative_offset_guess as libc::c_short as libc::c_int)
                        - 1 as libc::c_int) < 0 as libc::c_int
                        && (dt as libc::c_short as libc::c_int
                            - negative_offset_guess as libc::c_short as libc::c_int)
                            < -(32767 as libc::c_int) - 1 as libc::c_int
                    || (32767 as libc::c_int)
                        < dt as libc::c_short as libc::c_int
                            - negative_offset_guess as libc::c_short as libc::c_int
                {
                    *offset = (dt as libc::c_short as libc::c_uint)
                        .wrapping_sub(
                            negative_offset_guess as libc::c_short as libc::c_uint,
                        ) as libc::c_short as mktime_offset_t;
                } else {
                    *offset = (dt as libc::c_short as libc::c_uint)
                        .wrapping_sub(
                            negative_offset_guess as libc::c_short as libc::c_uint,
                        ) as libc::c_short as mktime_offset_t;
                };
            } else {
                if (if (if (0 as libc::c_int as libc::c_long
                    * ((0 as libc::c_int * negative_offset_guess) as libc::c_long + dt)
                    - 1 as libc::c_int as libc::c_long)
                    < 0 as libc::c_int as libc::c_long
                {
                    !((((0 as libc::c_int as libc::c_long
                        * ((0 as libc::c_int * negative_offset_guess) as libc::c_long
                            + dt) + 1 as libc::c_int as libc::c_long)
                        << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                        - 1 as libc::c_int as libc::c_long)
                        * 2 as libc::c_int as libc::c_long
                        + 1 as libc::c_int as libc::c_long)
                } else {
                    0 as libc::c_int as libc::c_long
                        * ((0 as libc::c_int * negative_offset_guess) as libc::c_long
                            + dt) + 0 as libc::c_int as libc::c_long
                }) < 0 as libc::c_int as libc::c_long
                {
                    if negative_offset_guess < 0 as libc::c_int {
                        (((if (0 as libc::c_int as libc::c_long
                            * ((0 as libc::c_int * negative_offset_guess) as libc::c_long
                                + dt) - 1 as libc::c_int as libc::c_long)
                            < 0 as libc::c_int as libc::c_long
                        {
                            (((0 as libc::c_int as libc::c_long
                                * ((0 as libc::c_int * negative_offset_guess)
                                    as libc::c_long + dt) + 1 as libc::c_int as libc::c_long)
                                << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                - 1 as libc::c_int as libc::c_long)
                                * 2 as libc::c_int as libc::c_long
                                + 1 as libc::c_int as libc::c_long
                        } else {
                            0 as libc::c_int as libc::c_long
                                * ((0 as libc::c_int * negative_offset_guess)
                                    as libc::c_long + dt) - 1 as libc::c_int as libc::c_long
                        }) + negative_offset_guess as libc::c_long) < dt) as libc::c_int
                    } else {
                        (dt
                            < (if (0 as libc::c_int as libc::c_long
                                * ((0 as libc::c_int * negative_offset_guess)
                                    as libc::c_long + dt) - 1 as libc::c_int as libc::c_long)
                                < 0 as libc::c_int as libc::c_long
                            {
                                !((((0 as libc::c_int as libc::c_long
                                    * ((0 as libc::c_int * negative_offset_guess)
                                        as libc::c_long + dt) + 1 as libc::c_int as libc::c_long)
                                    << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                    - 1 as libc::c_int as libc::c_long)
                                    * 2 as libc::c_int as libc::c_long
                                    + 1 as libc::c_int as libc::c_long)
                            } else {
                                0 as libc::c_int as libc::c_long
                                    * ((0 as libc::c_int * negative_offset_guess)
                                        as libc::c_long + dt) + 0 as libc::c_int as libc::c_long
                            }) + negative_offset_guess as libc::c_long) as libc::c_int
                    }
                } else {
                    if dt < 0 as libc::c_int as libc::c_long {
                        1 as libc::c_int
                    } else {
                        if negative_offset_guess < 0 as libc::c_int {
                            (dt - negative_offset_guess as libc::c_long <= dt)
                                as libc::c_int
                        } else {
                            (dt < negative_offset_guess as libc::c_long) as libc::c_int
                        }
                    }
                }) != 0
                    || (0 as libc::c_int as libc::c_long
                        * (dt - negative_offset_guess as libc::c_long)
                        - 1 as libc::c_int as libc::c_long)
                        < 0 as libc::c_int as libc::c_long
                        && (dt - negative_offset_guess as libc::c_long)
                            < (-(32767 as libc::c_int) - 1 as libc::c_int)
                                as libc::c_long
                    || (32767 as libc::c_int as libc::c_long)
                        < dt - negative_offset_guess as libc::c_long
                {
                    *offset = (dt as libc::c_uint)
                        .wrapping_sub(negative_offset_guess as libc::c_uint)
                        as libc::c_short as mktime_offset_t;
                } else {
                    *offset = (dt as libc::c_uint)
                        .wrapping_sub(negative_offset_guess as libc::c_uint)
                        as libc::c_short as mktime_offset_t;
                };
            };
        } else {
            if ::std::mem::size_of::<mktime_offset_t>() as libc::c_ulong
                == ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
            {
                if (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                    < ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                {
                    if (if (if (0 as libc::c_int
                        * (0 as libc::c_int * negative_offset_guess + dt as libc::c_int)
                        - 1 as libc::c_int) < 0 as libc::c_int
                    {
                        !((((0 as libc::c_int
                            * (0 as libc::c_int * negative_offset_guess
                                + dt as libc::c_int) + 1 as libc::c_int)
                            << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                            - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                    } else {
                        0 as libc::c_int
                            * (0 as libc::c_int * negative_offset_guess
                                + dt as libc::c_int) + 0 as libc::c_int
                    }) < 0 as libc::c_int
                    {
                        if negative_offset_guess < 0 as libc::c_int {
                            ((if (0 as libc::c_int
                                * (0 as libc::c_int * negative_offset_guess
                                    + dt as libc::c_int) - 1 as libc::c_int) < 0 as libc::c_int
                            {
                                (((0 as libc::c_int
                                    * (0 as libc::c_int * negative_offset_guess
                                        + dt as libc::c_int) + 1 as libc::c_int)
                                    << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                    - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                            } else {
                                0 as libc::c_int
                                    * (0 as libc::c_int * negative_offset_guess
                                        + dt as libc::c_int) - 1 as libc::c_int
                            }) + negative_offset_guess < dt as libc::c_int)
                                as libc::c_int
                        } else {
                            ((dt as libc::c_int)
                                < (if (0 as libc::c_int
                                    * (0 as libc::c_int * negative_offset_guess
                                        + dt as libc::c_int) - 1 as libc::c_int) < 0 as libc::c_int
                                {
                                    !((((0 as libc::c_int
                                        * (0 as libc::c_int * negative_offset_guess
                                            + dt as libc::c_int) + 1 as libc::c_int)
                                        << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                        - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                                } else {
                                    0 as libc::c_int
                                        * (0 as libc::c_int * negative_offset_guess
                                            + dt as libc::c_int) + 0 as libc::c_int
                                }) + negative_offset_guess) as libc::c_int
                        }
                    } else {
                        if (dt as libc::c_int) < 0 as libc::c_int {
                            1 as libc::c_int
                        } else {
                            if negative_offset_guess < 0 as libc::c_int {
                                (dt as libc::c_int - negative_offset_guess
                                    <= dt as libc::c_int) as libc::c_int
                            } else {
                                ((dt as libc::c_int) < negative_offset_guess) as libc::c_int
                            }
                        }
                    }) != 0
                        || (0 as libc::c_int
                            * (dt as libc::c_int - negative_offset_guess)
                            - 1 as libc::c_int) < 0 as libc::c_int
                            && dt as libc::c_int - negative_offset_guess
                                < -(2147483647 as libc::c_int) - 1 as libc::c_int
                        || (2147483647 as libc::c_int)
                            < dt as libc::c_int - negative_offset_guess
                    {
                        *offset = (dt as libc::c_int as libc::c_uint)
                            .wrapping_sub(negative_offset_guess as libc::c_uint)
                            as libc::c_int as mktime_offset_t;
                    } else {
                        *offset = (dt as libc::c_int as libc::c_uint)
                            .wrapping_sub(negative_offset_guess as libc::c_uint)
                            as libc::c_int as mktime_offset_t;
                    };
                } else {
                    if (if (if (0 as libc::c_int as libc::c_long
                        * ((0 as libc::c_int * negative_offset_guess) as libc::c_long
                            + dt) - 1 as libc::c_int as libc::c_long)
                        < 0 as libc::c_int as libc::c_long
                    {
                        !((((0 as libc::c_int as libc::c_long
                            * ((0 as libc::c_int * negative_offset_guess) as libc::c_long
                                + dt) + 1 as libc::c_int as libc::c_long)
                            << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                            - 1 as libc::c_int as libc::c_long)
                            * 2 as libc::c_int as libc::c_long
                            + 1 as libc::c_int as libc::c_long)
                    } else {
                        0 as libc::c_int as libc::c_long
                            * ((0 as libc::c_int * negative_offset_guess) as libc::c_long
                                + dt) + 0 as libc::c_int as libc::c_long
                    }) < 0 as libc::c_int as libc::c_long
                    {
                        if negative_offset_guess < 0 as libc::c_int {
                            (((if (0 as libc::c_int as libc::c_long
                                * ((0 as libc::c_int * negative_offset_guess)
                                    as libc::c_long + dt) - 1 as libc::c_int as libc::c_long)
                                < 0 as libc::c_int as libc::c_long
                            {
                                (((0 as libc::c_int as libc::c_long
                                    * ((0 as libc::c_int * negative_offset_guess)
                                        as libc::c_long + dt) + 1 as libc::c_int as libc::c_long)
                                    << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                    - 1 as libc::c_int as libc::c_long)
                                    * 2 as libc::c_int as libc::c_long
                                    + 1 as libc::c_int as libc::c_long
                            } else {
                                0 as libc::c_int as libc::c_long
                                    * ((0 as libc::c_int * negative_offset_guess)
                                        as libc::c_long + dt) - 1 as libc::c_int as libc::c_long
                            }) + negative_offset_guess as libc::c_long) < dt)
                                as libc::c_int
                        } else {
                            (dt
                                < (if (0 as libc::c_int as libc::c_long
                                    * ((0 as libc::c_int * negative_offset_guess)
                                        as libc::c_long + dt) - 1 as libc::c_int as libc::c_long)
                                    < 0 as libc::c_int as libc::c_long
                                {
                                    !((((0 as libc::c_int as libc::c_long
                                        * ((0 as libc::c_int * negative_offset_guess)
                                            as libc::c_long + dt) + 1 as libc::c_int as libc::c_long)
                                        << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                        - 1 as libc::c_int as libc::c_long)
                                        * 2 as libc::c_int as libc::c_long
                                        + 1 as libc::c_int as libc::c_long)
                                } else {
                                    0 as libc::c_int as libc::c_long
                                        * ((0 as libc::c_int * negative_offset_guess)
                                            as libc::c_long + dt) + 0 as libc::c_int as libc::c_long
                                }) + negative_offset_guess as libc::c_long) as libc::c_int
                        }
                    } else {
                        if dt < 0 as libc::c_int as libc::c_long {
                            1 as libc::c_int
                        } else {
                            if negative_offset_guess < 0 as libc::c_int {
                                (dt - negative_offset_guess as libc::c_long <= dt)
                                    as libc::c_int
                            } else {
                                (dt < negative_offset_guess as libc::c_long) as libc::c_int
                            }
                        }
                    }) != 0
                        || (0 as libc::c_int as libc::c_long
                            * (dt - negative_offset_guess as libc::c_long)
                            - 1 as libc::c_int as libc::c_long)
                            < 0 as libc::c_int as libc::c_long
                            && (dt - negative_offset_guess as libc::c_long)
                                < (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                    as libc::c_long
                        || (2147483647 as libc::c_int as libc::c_long)
                            < dt - negative_offset_guess as libc::c_long
                    {
                        *offset = (dt as libc::c_uint)
                            .wrapping_sub(negative_offset_guess as libc::c_uint)
                            as libc::c_int as mktime_offset_t;
                    } else {
                        *offset = (dt as libc::c_uint)
                            .wrapping_sub(negative_offset_guess as libc::c_uint)
                            as libc::c_int as mktime_offset_t;
                    };
                };
            } else {
                if ::std::mem::size_of::<mktime_offset_t>() as libc::c_ulong
                    == ::std::mem::size_of::<libc::c_long>() as libc::c_ulong
                {
                    if (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                        < ::std::mem::size_of::<libc::c_long>() as libc::c_ulong
                    {
                        if (if (if (0 as libc::c_int as libc::c_long
                            * (0 as libc::c_int as libc::c_long
                                * negative_offset_guess as libc::c_long + dt)
                            - 1 as libc::c_int as libc::c_long)
                            < 0 as libc::c_int as libc::c_long
                        {
                            !((((0 as libc::c_int as libc::c_long
                                * (0 as libc::c_int as libc::c_long
                                    * negative_offset_guess as libc::c_long + dt)
                                + 1 as libc::c_int as libc::c_long)
                                << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                - 1 as libc::c_int as libc::c_long)
                                * 2 as libc::c_int as libc::c_long
                                + 1 as libc::c_int as libc::c_long)
                        } else {
                            0 as libc::c_int as libc::c_long
                                * (0 as libc::c_int as libc::c_long
                                    * negative_offset_guess as libc::c_long + dt)
                                + 0 as libc::c_int as libc::c_long
                        }) < 0 as libc::c_int as libc::c_long
                        {
                            if (negative_offset_guess as libc::c_long)
                                < 0 as libc::c_int as libc::c_long
                            {
                                (((if (0 as libc::c_int as libc::c_long
                                    * (0 as libc::c_int as libc::c_long
                                        * negative_offset_guess as libc::c_long + dt)
                                    - 1 as libc::c_int as libc::c_long)
                                    < 0 as libc::c_int as libc::c_long
                                {
                                    (((0 as libc::c_int as libc::c_long
                                        * (0 as libc::c_int as libc::c_long
                                            * negative_offset_guess as libc::c_long + dt)
                                        + 1 as libc::c_int as libc::c_long)
                                        << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                        - 1 as libc::c_int as libc::c_long)
                                        * 2 as libc::c_int as libc::c_long
                                        + 1 as libc::c_int as libc::c_long
                                } else {
                                    0 as libc::c_int as libc::c_long
                                        * (0 as libc::c_int as libc::c_long
                                            * negative_offset_guess as libc::c_long + dt)
                                        - 1 as libc::c_int as libc::c_long
                                }) + negative_offset_guess as libc::c_long) < dt)
                                    as libc::c_int
                            } else {
                                (dt
                                    < (if (0 as libc::c_int as libc::c_long
                                        * (0 as libc::c_int as libc::c_long
                                            * negative_offset_guess as libc::c_long + dt)
                                        - 1 as libc::c_int as libc::c_long)
                                        < 0 as libc::c_int as libc::c_long
                                    {
                                        !((((0 as libc::c_int as libc::c_long
                                            * (0 as libc::c_int as libc::c_long
                                                * negative_offset_guess as libc::c_long + dt)
                                            + 1 as libc::c_int as libc::c_long)
                                            << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                            - 1 as libc::c_int as libc::c_long)
                                            * 2 as libc::c_int as libc::c_long
                                            + 1 as libc::c_int as libc::c_long)
                                    } else {
                                        0 as libc::c_int as libc::c_long
                                            * (0 as libc::c_int as libc::c_long
                                                * negative_offset_guess as libc::c_long + dt)
                                            + 0 as libc::c_int as libc::c_long
                                    }) + negative_offset_guess as libc::c_long) as libc::c_int
                            }
                        } else {
                            if dt < 0 as libc::c_int as libc::c_long {
                                1 as libc::c_int
                            } else {
                                if (negative_offset_guess as libc::c_long)
                                    < 0 as libc::c_int as libc::c_long
                                {
                                    (dt - negative_offset_guess as libc::c_long <= dt)
                                        as libc::c_int
                                } else {
                                    (dt < negative_offset_guess as libc::c_long) as libc::c_int
                                }
                            }
                        }) != 0
                            || (0 as libc::c_int as libc::c_long
                                * (dt - negative_offset_guess as libc::c_long)
                                - 1 as libc::c_int as libc::c_long)
                                < 0 as libc::c_int as libc::c_long
                                && (dt - negative_offset_guess as libc::c_long)
                                    < -(9223372036854775807 as libc::c_long) - 1 as libc::c_long
                            || (9223372036854775807 as libc::c_long)
                                < dt - negative_offset_guess as libc::c_long
                        {
                            *offset = (dt as libc::c_ulong)
                                .wrapping_sub(
                                    negative_offset_guess as libc::c_long as libc::c_ulong,
                                ) as libc::c_long;
                        } else {
                            *offset = (dt as libc::c_ulong)
                                .wrapping_sub(
                                    negative_offset_guess as libc::c_long as libc::c_ulong,
                                ) as libc::c_long;
                        };
                    } else {
                        if (if (if (0 as libc::c_int as libc::c_long
                            * ((0 as libc::c_int * negative_offset_guess) as libc::c_long
                                + dt) - 1 as libc::c_int as libc::c_long)
                            < 0 as libc::c_int as libc::c_long
                        {
                            !((((0 as libc::c_int as libc::c_long
                                * ((0 as libc::c_int * negative_offset_guess)
                                    as libc::c_long + dt) + 1 as libc::c_int as libc::c_long)
                                << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                - 1 as libc::c_int as libc::c_long)
                                * 2 as libc::c_int as libc::c_long
                                + 1 as libc::c_int as libc::c_long)
                        } else {
                            0 as libc::c_int as libc::c_long
                                * ((0 as libc::c_int * negative_offset_guess)
                                    as libc::c_long + dt) + 0 as libc::c_int as libc::c_long
                        }) < 0 as libc::c_int as libc::c_long
                        {
                            if negative_offset_guess < 0 as libc::c_int {
                                (((if (0 as libc::c_int as libc::c_long
                                    * ((0 as libc::c_int * negative_offset_guess)
                                        as libc::c_long + dt) - 1 as libc::c_int as libc::c_long)
                                    < 0 as libc::c_int as libc::c_long
                                {
                                    (((0 as libc::c_int as libc::c_long
                                        * ((0 as libc::c_int * negative_offset_guess)
                                            as libc::c_long + dt) + 1 as libc::c_int as libc::c_long)
                                        << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                        - 1 as libc::c_int as libc::c_long)
                                        * 2 as libc::c_int as libc::c_long
                                        + 1 as libc::c_int as libc::c_long
                                } else {
                                    0 as libc::c_int as libc::c_long
                                        * ((0 as libc::c_int * negative_offset_guess)
                                            as libc::c_long + dt) - 1 as libc::c_int as libc::c_long
                                }) + negative_offset_guess as libc::c_long) < dt)
                                    as libc::c_int
                            } else {
                                (dt
                                    < (if (0 as libc::c_int as libc::c_long
                                        * ((0 as libc::c_int * negative_offset_guess)
                                            as libc::c_long + dt) - 1 as libc::c_int as libc::c_long)
                                        < 0 as libc::c_int as libc::c_long
                                    {
                                        !((((0 as libc::c_int as libc::c_long
                                            * ((0 as libc::c_int * negative_offset_guess)
                                                as libc::c_long + dt) + 1 as libc::c_int as libc::c_long)
                                            << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                            - 1 as libc::c_int as libc::c_long)
                                            * 2 as libc::c_int as libc::c_long
                                            + 1 as libc::c_int as libc::c_long)
                                    } else {
                                        0 as libc::c_int as libc::c_long
                                            * ((0 as libc::c_int * negative_offset_guess)
                                                as libc::c_long + dt) + 0 as libc::c_int as libc::c_long
                                    }) + negative_offset_guess as libc::c_long) as libc::c_int
                            }
                        } else {
                            if dt < 0 as libc::c_int as libc::c_long {
                                1 as libc::c_int
                            } else {
                                if negative_offset_guess < 0 as libc::c_int {
                                    (dt - negative_offset_guess as libc::c_long <= dt)
                                        as libc::c_int
                                } else {
                                    (dt < negative_offset_guess as libc::c_long) as libc::c_int
                                }
                            }
                        }) != 0
                            || (0 as libc::c_int as libc::c_long
                                * (dt - negative_offset_guess as libc::c_long)
                                - 1 as libc::c_int as libc::c_long)
                                < 0 as libc::c_int as libc::c_long
                                && (dt - negative_offset_guess as libc::c_long)
                                    < -(9223372036854775807 as libc::c_long) - 1 as libc::c_long
                            || (9223372036854775807 as libc::c_long)
                                < dt - negative_offset_guess as libc::c_long
                        {
                            *offset = (dt as libc::c_ulong)
                                .wrapping_sub(negative_offset_guess as libc::c_ulong)
                                as libc::c_long;
                        } else {
                            *offset = (dt as libc::c_ulong)
                                .wrapping_sub(negative_offset_guess as libc::c_ulong)
                                as libc::c_long;
                        };
                    };
                } else {
                    if (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                        < ::std::mem::size_of::<libc::c_longlong>() as libc::c_ulong
                    {
                        if (if (if (0 as libc::c_int as libc::c_longlong
                            * (0 as libc::c_int as libc::c_longlong
                                * negative_offset_guess as libc::c_longlong
                                + dt as libc::c_longlong)
                            - 1 as libc::c_int as libc::c_longlong)
                            < 0 as libc::c_int as libc::c_longlong
                        {
                            !((((0 as libc::c_int as libc::c_longlong
                                * (0 as libc::c_int as libc::c_longlong
                                    * negative_offset_guess as libc::c_longlong
                                    + dt as libc::c_longlong)
                                + 1 as libc::c_int as libc::c_longlong)
                                << (::std::mem::size_of::<libc::c_longlong>()
                                    as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                - 1 as libc::c_int as libc::c_longlong)
                                * 2 as libc::c_int as libc::c_longlong
                                + 1 as libc::c_int as libc::c_longlong)
                        } else {
                            0 as libc::c_int as libc::c_longlong
                                * (0 as libc::c_int as libc::c_longlong
                                    * negative_offset_guess as libc::c_longlong
                                    + dt as libc::c_longlong)
                                + 0 as libc::c_int as libc::c_longlong
                        }) < 0 as libc::c_int as libc::c_longlong
                        {
                            if (negative_offset_guess as libc::c_longlong)
                                < 0 as libc::c_int as libc::c_longlong
                            {
                                (((if (0 as libc::c_int as libc::c_longlong
                                    * (0 as libc::c_int as libc::c_longlong
                                        * negative_offset_guess as libc::c_longlong
                                        + dt as libc::c_longlong)
                                    - 1 as libc::c_int as libc::c_longlong)
                                    < 0 as libc::c_int as libc::c_longlong
                                {
                                    (((0 as libc::c_int as libc::c_longlong
                                        * (0 as libc::c_int as libc::c_longlong
                                            * negative_offset_guess as libc::c_longlong
                                            + dt as libc::c_longlong)
                                        + 1 as libc::c_int as libc::c_longlong)
                                        << (::std::mem::size_of::<libc::c_longlong>()
                                            as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                        - 1 as libc::c_int as libc::c_longlong)
                                        * 2 as libc::c_int as libc::c_longlong
                                        + 1 as libc::c_int as libc::c_longlong
                                } else {
                                    0 as libc::c_int as libc::c_longlong
                                        * (0 as libc::c_int as libc::c_longlong
                                            * negative_offset_guess as libc::c_longlong
                                            + dt as libc::c_longlong)
                                        - 1 as libc::c_int as libc::c_longlong
                                }) + negative_offset_guess as libc::c_longlong)
                                    < dt as libc::c_longlong) as libc::c_int
                            } else {
                                ((dt as libc::c_longlong)
                                    < (if (0 as libc::c_int as libc::c_longlong
                                        * (0 as libc::c_int as libc::c_longlong
                                            * negative_offset_guess as libc::c_longlong
                                            + dt as libc::c_longlong)
                                        - 1 as libc::c_int as libc::c_longlong)
                                        < 0 as libc::c_int as libc::c_longlong
                                    {
                                        !((((0 as libc::c_int as libc::c_longlong
                                            * (0 as libc::c_int as libc::c_longlong
                                                * negative_offset_guess as libc::c_longlong
                                                + dt as libc::c_longlong)
                                            + 1 as libc::c_int as libc::c_longlong)
                                            << (::std::mem::size_of::<libc::c_longlong>()
                                                as libc::c_ulong)
                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                            - 1 as libc::c_int as libc::c_longlong)
                                            * 2 as libc::c_int as libc::c_longlong
                                            + 1 as libc::c_int as libc::c_longlong)
                                    } else {
                                        0 as libc::c_int as libc::c_longlong
                                            * (0 as libc::c_int as libc::c_longlong
                                                * negative_offset_guess as libc::c_longlong
                                                + dt as libc::c_longlong)
                                            + 0 as libc::c_int as libc::c_longlong
                                    }) + negative_offset_guess as libc::c_longlong)
                                    as libc::c_int
                            }
                        } else {
                            if (dt as libc::c_longlong)
                                < 0 as libc::c_int as libc::c_longlong
                            {
                                1 as libc::c_int
                            } else {
                                if (negative_offset_guess as libc::c_longlong)
                                    < 0 as libc::c_int as libc::c_longlong
                                {
                                    (dt as libc::c_longlong
                                        - negative_offset_guess as libc::c_longlong
                                        <= dt as libc::c_longlong) as libc::c_int
                                } else {
                                    ((dt as libc::c_longlong)
                                        < negative_offset_guess as libc::c_longlong) as libc::c_int
                                }
                            }
                        }) != 0
                            || (0 as libc::c_int as libc::c_longlong
                                * (dt as libc::c_longlong
                                    - negative_offset_guess as libc::c_longlong)
                                - 1 as libc::c_int as libc::c_longlong)
                                < 0 as libc::c_int as libc::c_longlong
                                && (dt as libc::c_longlong
                                    - negative_offset_guess as libc::c_longlong)
                                    < -(9223372036854775807 as libc::c_longlong)
                                        - 1 as libc::c_longlong
                            || (9223372036854775807 as libc::c_longlong)
                                < dt as libc::c_longlong
                                    - negative_offset_guess as libc::c_longlong
                        {
                            *offset = (dt as libc::c_longlong as libc::c_ulonglong)
                                .wrapping_sub(
                                    negative_offset_guess as libc::c_longlong
                                        as libc::c_ulonglong,
                                ) as libc::c_longlong as mktime_offset_t;
                        } else {
                            *offset = (dt as libc::c_longlong as libc::c_ulonglong)
                                .wrapping_sub(
                                    negative_offset_guess as libc::c_longlong
                                        as libc::c_ulonglong,
                                ) as libc::c_longlong as mktime_offset_t;
                        };
                    } else {
                        if (if (if (0 as libc::c_int as libc::c_long
                            * ((0 as libc::c_int * negative_offset_guess) as libc::c_long
                                + dt) - 1 as libc::c_int as libc::c_long)
                            < 0 as libc::c_int as libc::c_long
                        {
                            !((((0 as libc::c_int as libc::c_long
                                * ((0 as libc::c_int * negative_offset_guess)
                                    as libc::c_long + dt) + 1 as libc::c_int as libc::c_long)
                                << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                - 1 as libc::c_int as libc::c_long)
                                * 2 as libc::c_int as libc::c_long
                                + 1 as libc::c_int as libc::c_long)
                        } else {
                            0 as libc::c_int as libc::c_long
                                * ((0 as libc::c_int * negative_offset_guess)
                                    as libc::c_long + dt) + 0 as libc::c_int as libc::c_long
                        }) < 0 as libc::c_int as libc::c_long
                        {
                            if negative_offset_guess < 0 as libc::c_int {
                                (((if (0 as libc::c_int as libc::c_long
                                    * ((0 as libc::c_int * negative_offset_guess)
                                        as libc::c_long + dt) - 1 as libc::c_int as libc::c_long)
                                    < 0 as libc::c_int as libc::c_long
                                {
                                    (((0 as libc::c_int as libc::c_long
                                        * ((0 as libc::c_int * negative_offset_guess)
                                            as libc::c_long + dt) + 1 as libc::c_int as libc::c_long)
                                        << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                        - 1 as libc::c_int as libc::c_long)
                                        * 2 as libc::c_int as libc::c_long
                                        + 1 as libc::c_int as libc::c_long
                                } else {
                                    0 as libc::c_int as libc::c_long
                                        * ((0 as libc::c_int * negative_offset_guess)
                                            as libc::c_long + dt) - 1 as libc::c_int as libc::c_long
                                }) + negative_offset_guess as libc::c_long) < dt)
                                    as libc::c_int
                            } else {
                                (dt
                                    < (if (0 as libc::c_int as libc::c_long
                                        * ((0 as libc::c_int * negative_offset_guess)
                                            as libc::c_long + dt) - 1 as libc::c_int as libc::c_long)
                                        < 0 as libc::c_int as libc::c_long
                                    {
                                        !((((0 as libc::c_int as libc::c_long
                                            * ((0 as libc::c_int * negative_offset_guess)
                                                as libc::c_long + dt) + 1 as libc::c_int as libc::c_long)
                                            << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                            - 1 as libc::c_int as libc::c_long)
                                            * 2 as libc::c_int as libc::c_long
                                            + 1 as libc::c_int as libc::c_long)
                                    } else {
                                        0 as libc::c_int as libc::c_long
                                            * ((0 as libc::c_int * negative_offset_guess)
                                                as libc::c_long + dt) + 0 as libc::c_int as libc::c_long
                                    }) + negative_offset_guess as libc::c_long) as libc::c_int
                            }
                        } else {
                            if dt < 0 as libc::c_int as libc::c_long {
                                1 as libc::c_int
                            } else {
                                if negative_offset_guess < 0 as libc::c_int {
                                    (dt - negative_offset_guess as libc::c_long <= dt)
                                        as libc::c_int
                                } else {
                                    (dt < negative_offset_guess as libc::c_long) as libc::c_int
                                }
                            }
                        }) != 0
                            || (0 as libc::c_int as libc::c_long
                                * (dt - negative_offset_guess as libc::c_long)
                                - 1 as libc::c_int as libc::c_long)
                                < 0 as libc::c_int as libc::c_long
                                && ((dt - negative_offset_guess as libc::c_long)
                                    as libc::c_longlong)
                                    < -(9223372036854775807 as libc::c_longlong)
                                        - 1 as libc::c_longlong
                            || (9223372036854775807 as libc::c_longlong)
                                < (dt - negative_offset_guess as libc::c_long)
                                    as libc::c_longlong
                        {
                            *offset = (dt as libc::c_ulonglong)
                                .wrapping_sub(negative_offset_guess as libc::c_ulonglong)
                                as libc::c_longlong as mktime_offset_t;
                        } else {
                            *offset = (dt as libc::c_ulonglong)
                                .wrapping_sub(negative_offset_guess as libc::c_ulonglong)
                                as libc::c_longlong as mktime_offset_t;
                        };
                    };
                };
            };
        };
    };
    if 1 as libc::c_int != 0 && sec_requested != tm.tm_sec {
        let mut sec_adjustment: long_int = (sec == 0 as libc::c_int
            && tm.tm_sec == 60 as libc::c_int) as libc::c_int as long_int;
        sec_adjustment -= sec as libc::c_long;
        sec_adjustment += sec_requested as libc::c_long;
        if (if ::std::mem::size_of::<long_int>() as libc::c_ulong
            == ::std::mem::size_of::<libc::c_schar>() as libc::c_ulong
        {
            if (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                < ::std::mem::size_of::<libc::c_schar>() as libc::c_ulong
            {
                if (if (if (0 as libc::c_int
                    * (0 as libc::c_int * sec_adjustment as libc::c_schar as libc::c_int
                        + t as libc::c_schar as libc::c_int) - 1 as libc::c_int)
                    < 0 as libc::c_int
                {
                    !((((0 as libc::c_int
                        * (0 as libc::c_int
                            * sec_adjustment as libc::c_schar as libc::c_int
                            + t as libc::c_schar as libc::c_int) + 1 as libc::c_int)
                        << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                        - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                } else {
                    0 as libc::c_int
                        * (0 as libc::c_int
                            * sec_adjustment as libc::c_schar as libc::c_int
                            + t as libc::c_schar as libc::c_int) + 0 as libc::c_int
                }) < 0 as libc::c_int
                {
                    if (sec_adjustment as libc::c_schar as libc::c_int)
                        < 0 as libc::c_int
                    {
                        ((t as libc::c_schar as libc::c_int)
                            < (if (0 as libc::c_int
                                * (0 as libc::c_int
                                    * sec_adjustment as libc::c_schar as libc::c_int
                                    + t as libc::c_schar as libc::c_int) - 1 as libc::c_int)
                                < 0 as libc::c_int
                            {
                                !((((0 as libc::c_int
                                    * (0 as libc::c_int
                                        * sec_adjustment as libc::c_schar as libc::c_int
                                        + t as libc::c_schar as libc::c_int) + 1 as libc::c_int)
                                    << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                    - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                            } else {
                                0 as libc::c_int
                                    * (0 as libc::c_int
                                        * sec_adjustment as libc::c_schar as libc::c_int
                                        + t as libc::c_schar as libc::c_int) + 0 as libc::c_int
                            }) - sec_adjustment as libc::c_schar as libc::c_int)
                            as libc::c_int
                    } else {
                        (((if (0 as libc::c_int
                            * (0 as libc::c_int
                                * sec_adjustment as libc::c_schar as libc::c_int
                                + t as libc::c_schar as libc::c_int) - 1 as libc::c_int)
                            < 0 as libc::c_int
                        {
                            (((0 as libc::c_int
                                * (0 as libc::c_int
                                    * sec_adjustment as libc::c_schar as libc::c_int
                                    + t as libc::c_schar as libc::c_int) + 1 as libc::c_int)
                                << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                        } else {
                            0 as libc::c_int
                                * (0 as libc::c_int
                                    * sec_adjustment as libc::c_schar as libc::c_int
                                    + t as libc::c_schar as libc::c_int) - 1 as libc::c_int
                        }) - sec_adjustment as libc::c_schar as libc::c_int)
                            < t as libc::c_schar as libc::c_int) as libc::c_int
                    }
                } else {
                    if (t as libc::c_schar as libc::c_int) < 0 as libc::c_int {
                        (sec_adjustment as libc::c_schar as libc::c_int
                            <= t as libc::c_schar as libc::c_int
                                + sec_adjustment as libc::c_schar as libc::c_int)
                            as libc::c_int
                    } else {
                        if (sec_adjustment as libc::c_schar as libc::c_int)
                            < 0 as libc::c_int
                        {
                            (t as libc::c_schar as libc::c_int
                                <= t as libc::c_schar as libc::c_int
                                    + sec_adjustment as libc::c_schar as libc::c_int)
                                as libc::c_int
                        } else {
                            ((t as libc::c_schar as libc::c_int
                                + sec_adjustment as libc::c_schar as libc::c_int)
                                < sec_adjustment as libc::c_schar as libc::c_int)
                                as libc::c_int
                        }
                    }
                }) != 0
                    || (0 as libc::c_int
                        * (t as libc::c_schar as libc::c_int
                            + sec_adjustment as libc::c_schar as libc::c_int)
                        - 1 as libc::c_int) < 0 as libc::c_int
                        && (t as libc::c_schar as libc::c_int
                            + sec_adjustment as libc::c_schar as libc::c_int)
                            < -(127 as libc::c_int) - 1 as libc::c_int
                    || (127 as libc::c_int)
                        < t as libc::c_schar as libc::c_int
                            + sec_adjustment as libc::c_schar as libc::c_int
                {
                    t = (t as libc::c_schar as libc::c_uint)
                        .wrapping_add(sec_adjustment as libc::c_schar as libc::c_uint)
                        as libc::c_schar as long_int;
                    1 as libc::c_int
                } else {
                    t = (t as libc::c_schar as libc::c_uint)
                        .wrapping_add(sec_adjustment as libc::c_schar as libc::c_uint)
                        as libc::c_schar as long_int;
                    0 as libc::c_int
                }
            } else {
                if (if (if (0 as libc::c_int as libc::c_long
                    * (0 as libc::c_int as libc::c_long * sec_adjustment + t)
                    - 1 as libc::c_int as libc::c_long)
                    < 0 as libc::c_int as libc::c_long
                {
                    !((((0 as libc::c_int as libc::c_long
                        * (0 as libc::c_int as libc::c_long * sec_adjustment + t)
                        + 1 as libc::c_int as libc::c_long)
                        << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                        - 1 as libc::c_int as libc::c_long)
                        * 2 as libc::c_int as libc::c_long
                        + 1 as libc::c_int as libc::c_long)
                } else {
                    0 as libc::c_int as libc::c_long
                        * (0 as libc::c_int as libc::c_long * sec_adjustment + t)
                        + 0 as libc::c_int as libc::c_long
                }) < 0 as libc::c_int as libc::c_long
                {
                    if sec_adjustment < 0 as libc::c_int as libc::c_long {
                        (t
                            < (if (0 as libc::c_int as libc::c_long
                                * (0 as libc::c_int as libc::c_long * sec_adjustment + t)
                                - 1 as libc::c_int as libc::c_long)
                                < 0 as libc::c_int as libc::c_long
                            {
                                !((((0 as libc::c_int as libc::c_long
                                    * (0 as libc::c_int as libc::c_long * sec_adjustment + t)
                                    + 1 as libc::c_int as libc::c_long)
                                    << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                    - 1 as libc::c_int as libc::c_long)
                                    * 2 as libc::c_int as libc::c_long
                                    + 1 as libc::c_int as libc::c_long)
                            } else {
                                0 as libc::c_int as libc::c_long
                                    * (0 as libc::c_int as libc::c_long * sec_adjustment + t)
                                    + 0 as libc::c_int as libc::c_long
                            }) - sec_adjustment) as libc::c_int
                    } else {
                        ((if (0 as libc::c_int as libc::c_long
                            * (0 as libc::c_int as libc::c_long * sec_adjustment + t)
                            - 1 as libc::c_int as libc::c_long)
                            < 0 as libc::c_int as libc::c_long
                        {
                            (((0 as libc::c_int as libc::c_long
                                * (0 as libc::c_int as libc::c_long * sec_adjustment + t)
                                + 1 as libc::c_int as libc::c_long)
                                << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                - 1 as libc::c_int as libc::c_long)
                                * 2 as libc::c_int as libc::c_long
                                + 1 as libc::c_int as libc::c_long
                        } else {
                            0 as libc::c_int as libc::c_long
                                * (0 as libc::c_int as libc::c_long * sec_adjustment + t)
                                - 1 as libc::c_int as libc::c_long
                        }) - sec_adjustment < t) as libc::c_int
                    }
                } else {
                    if t < 0 as libc::c_int as libc::c_long {
                        (sec_adjustment <= t + sec_adjustment) as libc::c_int
                    } else {
                        if sec_adjustment < 0 as libc::c_int as libc::c_long {
                            (t <= t + sec_adjustment) as libc::c_int
                        } else {
                            (t + sec_adjustment < sec_adjustment) as libc::c_int
                        }
                    }
                }) != 0
                    || (0 as libc::c_int as libc::c_long * (t + sec_adjustment)
                        - 1 as libc::c_int as libc::c_long)
                        < 0 as libc::c_int as libc::c_long
                        && t + sec_adjustment
                            < (-(127 as libc::c_int) - 1 as libc::c_int) as libc::c_long
                    || (127 as libc::c_int as libc::c_long) < t + sec_adjustment
                {
                    t = (t as libc::c_uint).wrapping_add(sec_adjustment as libc::c_uint)
                        as libc::c_schar as long_int;
                    1 as libc::c_int
                } else {
                    t = (t as libc::c_uint).wrapping_add(sec_adjustment as libc::c_uint)
                        as libc::c_schar as long_int;
                    0 as libc::c_int
                }
            }
        } else {
            if ::std::mem::size_of::<long_int>() as libc::c_ulong
                == ::std::mem::size_of::<libc::c_short>() as libc::c_ulong
            {
                if (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                    < ::std::mem::size_of::<libc::c_short>() as libc::c_ulong
                {
                    if (if (if (0 as libc::c_int
                        * (0 as libc::c_int
                            * sec_adjustment as libc::c_short as libc::c_int
                            + t as libc::c_short as libc::c_int) - 1 as libc::c_int)
                        < 0 as libc::c_int
                    {
                        !((((0 as libc::c_int
                            * (0 as libc::c_int
                                * sec_adjustment as libc::c_short as libc::c_int
                                + t as libc::c_short as libc::c_int) + 1 as libc::c_int)
                            << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                            - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                    } else {
                        0 as libc::c_int
                            * (0 as libc::c_int
                                * sec_adjustment as libc::c_short as libc::c_int
                                + t as libc::c_short as libc::c_int) + 0 as libc::c_int
                    }) < 0 as libc::c_int
                    {
                        if (sec_adjustment as libc::c_short as libc::c_int)
                            < 0 as libc::c_int
                        {
                            ((t as libc::c_short as libc::c_int)
                                < (if (0 as libc::c_int
                                    * (0 as libc::c_int
                                        * sec_adjustment as libc::c_short as libc::c_int
                                        + t as libc::c_short as libc::c_int) - 1 as libc::c_int)
                                    < 0 as libc::c_int
                                {
                                    !((((0 as libc::c_int
                                        * (0 as libc::c_int
                                            * sec_adjustment as libc::c_short as libc::c_int
                                            + t as libc::c_short as libc::c_int) + 1 as libc::c_int)
                                        << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                        - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                                } else {
                                    0 as libc::c_int
                                        * (0 as libc::c_int
                                            * sec_adjustment as libc::c_short as libc::c_int
                                            + t as libc::c_short as libc::c_int) + 0 as libc::c_int
                                }) - sec_adjustment as libc::c_short as libc::c_int)
                                as libc::c_int
                        } else {
                            (((if (0 as libc::c_int
                                * (0 as libc::c_int
                                    * sec_adjustment as libc::c_short as libc::c_int
                                    + t as libc::c_short as libc::c_int) - 1 as libc::c_int)
                                < 0 as libc::c_int
                            {
                                (((0 as libc::c_int
                                    * (0 as libc::c_int
                                        * sec_adjustment as libc::c_short as libc::c_int
                                        + t as libc::c_short as libc::c_int) + 1 as libc::c_int)
                                    << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                    - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                            } else {
                                0 as libc::c_int
                                    * (0 as libc::c_int
                                        * sec_adjustment as libc::c_short as libc::c_int
                                        + t as libc::c_short as libc::c_int) - 1 as libc::c_int
                            }) - sec_adjustment as libc::c_short as libc::c_int)
                                < t as libc::c_short as libc::c_int) as libc::c_int
                        }
                    } else {
                        if (t as libc::c_short as libc::c_int) < 0 as libc::c_int {
                            (sec_adjustment as libc::c_short as libc::c_int
                                <= t as libc::c_short as libc::c_int
                                    + sec_adjustment as libc::c_short as libc::c_int)
                                as libc::c_int
                        } else {
                            if (sec_adjustment as libc::c_short as libc::c_int)
                                < 0 as libc::c_int
                            {
                                (t as libc::c_short as libc::c_int
                                    <= t as libc::c_short as libc::c_int
                                        + sec_adjustment as libc::c_short as libc::c_int)
                                    as libc::c_int
                            } else {
                                ((t as libc::c_short as libc::c_int
                                    + sec_adjustment as libc::c_short as libc::c_int)
                                    < sec_adjustment as libc::c_short as libc::c_int)
                                    as libc::c_int
                            }
                        }
                    }) != 0
                        || (0 as libc::c_int
                            * (t as libc::c_short as libc::c_int
                                + sec_adjustment as libc::c_short as libc::c_int)
                            - 1 as libc::c_int) < 0 as libc::c_int
                            && (t as libc::c_short as libc::c_int
                                + sec_adjustment as libc::c_short as libc::c_int)
                                < -(32767 as libc::c_int) - 1 as libc::c_int
                        || (32767 as libc::c_int)
                            < t as libc::c_short as libc::c_int
                                + sec_adjustment as libc::c_short as libc::c_int
                    {
                        t = (t as libc::c_short as libc::c_uint)
                            .wrapping_add(
                                sec_adjustment as libc::c_short as libc::c_uint,
                            ) as libc::c_short as long_int;
                        1 as libc::c_int
                    } else {
                        t = (t as libc::c_short as libc::c_uint)
                            .wrapping_add(
                                sec_adjustment as libc::c_short as libc::c_uint,
                            ) as libc::c_short as long_int;
                        0 as libc::c_int
                    }
                } else {
                    if (if (if (0 as libc::c_int as libc::c_long
                        * (0 as libc::c_int as libc::c_long * sec_adjustment + t)
                        - 1 as libc::c_int as libc::c_long)
                        < 0 as libc::c_int as libc::c_long
                    {
                        !((((0 as libc::c_int as libc::c_long
                            * (0 as libc::c_int as libc::c_long * sec_adjustment + t)
                            + 1 as libc::c_int as libc::c_long)
                            << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                            - 1 as libc::c_int as libc::c_long)
                            * 2 as libc::c_int as libc::c_long
                            + 1 as libc::c_int as libc::c_long)
                    } else {
                        0 as libc::c_int as libc::c_long
                            * (0 as libc::c_int as libc::c_long * sec_adjustment + t)
                            + 0 as libc::c_int as libc::c_long
                    }) < 0 as libc::c_int as libc::c_long
                    {
                        if sec_adjustment < 0 as libc::c_int as libc::c_long {
                            (t
                                < (if (0 as libc::c_int as libc::c_long
                                    * (0 as libc::c_int as libc::c_long * sec_adjustment + t)
                                    - 1 as libc::c_int as libc::c_long)
                                    < 0 as libc::c_int as libc::c_long
                                {
                                    !((((0 as libc::c_int as libc::c_long
                                        * (0 as libc::c_int as libc::c_long * sec_adjustment + t)
                                        + 1 as libc::c_int as libc::c_long)
                                        << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                        - 1 as libc::c_int as libc::c_long)
                                        * 2 as libc::c_int as libc::c_long
                                        + 1 as libc::c_int as libc::c_long)
                                } else {
                                    0 as libc::c_int as libc::c_long
                                        * (0 as libc::c_int as libc::c_long * sec_adjustment + t)
                                        + 0 as libc::c_int as libc::c_long
                                }) - sec_adjustment) as libc::c_int
                        } else {
                            ((if (0 as libc::c_int as libc::c_long
                                * (0 as libc::c_int as libc::c_long * sec_adjustment + t)
                                - 1 as libc::c_int as libc::c_long)
                                < 0 as libc::c_int as libc::c_long
                            {
                                (((0 as libc::c_int as libc::c_long
                                    * (0 as libc::c_int as libc::c_long * sec_adjustment + t)
                                    + 1 as libc::c_int as libc::c_long)
                                    << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                    - 1 as libc::c_int as libc::c_long)
                                    * 2 as libc::c_int as libc::c_long
                                    + 1 as libc::c_int as libc::c_long
                            } else {
                                0 as libc::c_int as libc::c_long
                                    * (0 as libc::c_int as libc::c_long * sec_adjustment + t)
                                    - 1 as libc::c_int as libc::c_long
                            }) - sec_adjustment < t) as libc::c_int
                        }
                    } else {
                        if t < 0 as libc::c_int as libc::c_long {
                            (sec_adjustment <= t + sec_adjustment) as libc::c_int
                        } else {
                            if sec_adjustment < 0 as libc::c_int as libc::c_long {
                                (t <= t + sec_adjustment) as libc::c_int
                            } else {
                                (t + sec_adjustment < sec_adjustment) as libc::c_int
                            }
                        }
                    }) != 0
                        || (0 as libc::c_int as libc::c_long * (t + sec_adjustment)
                            - 1 as libc::c_int as libc::c_long)
                            < 0 as libc::c_int as libc::c_long
                            && t + sec_adjustment
                                < (-(32767 as libc::c_int) - 1 as libc::c_int)
                                    as libc::c_long
                        || (32767 as libc::c_int as libc::c_long) < t + sec_adjustment
                    {
                        t = (t as libc::c_uint)
                            .wrapping_add(sec_adjustment as libc::c_uint)
                            as libc::c_short as long_int;
                        1 as libc::c_int
                    } else {
                        t = (t as libc::c_uint)
                            .wrapping_add(sec_adjustment as libc::c_uint)
                            as libc::c_short as long_int;
                        0 as libc::c_int
                    }
                }
            } else {
                if ::std::mem::size_of::<long_int>() as libc::c_ulong
                    == ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                {
                    if (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                        < ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                    {
                        if (if (if (0 as libc::c_int
                            * (0 as libc::c_int * sec_adjustment as libc::c_int
                                + t as libc::c_int) - 1 as libc::c_int) < 0 as libc::c_int
                        {
                            !((((0 as libc::c_int
                                * (0 as libc::c_int * sec_adjustment as libc::c_int
                                    + t as libc::c_int) + 1 as libc::c_int)
                                << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                        } else {
                            0 as libc::c_int
                                * (0 as libc::c_int * sec_adjustment as libc::c_int
                                    + t as libc::c_int) + 0 as libc::c_int
                        }) < 0 as libc::c_int
                        {
                            if (sec_adjustment as libc::c_int) < 0 as libc::c_int {
                                ((t as libc::c_int)
                                    < (if (0 as libc::c_int
                                        * (0 as libc::c_int * sec_adjustment as libc::c_int
                                            + t as libc::c_int) - 1 as libc::c_int) < 0 as libc::c_int
                                    {
                                        !((((0 as libc::c_int
                                            * (0 as libc::c_int * sec_adjustment as libc::c_int
                                                + t as libc::c_int) + 1 as libc::c_int)
                                            << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                            - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                                    } else {
                                        0 as libc::c_int
                                            * (0 as libc::c_int * sec_adjustment as libc::c_int
                                                + t as libc::c_int) + 0 as libc::c_int
                                    }) - sec_adjustment as libc::c_int) as libc::c_int
                            } else {
                                (((if (0 as libc::c_int
                                    * (0 as libc::c_int * sec_adjustment as libc::c_int
                                        + t as libc::c_int) - 1 as libc::c_int) < 0 as libc::c_int
                                {
                                    (((0 as libc::c_int
                                        * (0 as libc::c_int * sec_adjustment as libc::c_int
                                            + t as libc::c_int) + 1 as libc::c_int)
                                        << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                        - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                                } else {
                                    0 as libc::c_int
                                        * (0 as libc::c_int * sec_adjustment as libc::c_int
                                            + t as libc::c_int) - 1 as libc::c_int
                                }) - sec_adjustment as libc::c_int) < t as libc::c_int)
                                    as libc::c_int
                            }
                        } else {
                            if (t as libc::c_int) < 0 as libc::c_int {
                                (sec_adjustment as libc::c_int
                                    <= t as libc::c_int + sec_adjustment as libc::c_int)
                                    as libc::c_int
                            } else {
                                if (sec_adjustment as libc::c_int) < 0 as libc::c_int {
                                    (t as libc::c_int
                                        <= t as libc::c_int + sec_adjustment as libc::c_int)
                                        as libc::c_int
                                } else {
                                    ((t as libc::c_int + sec_adjustment as libc::c_int)
                                        < sec_adjustment as libc::c_int) as libc::c_int
                                }
                            }
                        }) != 0
                            || (0 as libc::c_int
                                * (t as libc::c_int + sec_adjustment as libc::c_int)
                                - 1 as libc::c_int) < 0 as libc::c_int
                                && (t as libc::c_int + sec_adjustment as libc::c_int)
                                    < -(2147483647 as libc::c_int) - 1 as libc::c_int
                            || (2147483647 as libc::c_int)
                                < t as libc::c_int + sec_adjustment as libc::c_int
                        {
                            t = (t as libc::c_int as libc::c_uint)
                                .wrapping_add(sec_adjustment as libc::c_int as libc::c_uint)
                                as libc::c_int as long_int;
                            1 as libc::c_int
                        } else {
                            t = (t as libc::c_int as libc::c_uint)
                                .wrapping_add(sec_adjustment as libc::c_int as libc::c_uint)
                                as libc::c_int as long_int;
                            0 as libc::c_int
                        }
                    } else {
                        if (if (if (0 as libc::c_int as libc::c_long
                            * (0 as libc::c_int as libc::c_long * sec_adjustment + t)
                            - 1 as libc::c_int as libc::c_long)
                            < 0 as libc::c_int as libc::c_long
                        {
                            !((((0 as libc::c_int as libc::c_long
                                * (0 as libc::c_int as libc::c_long * sec_adjustment + t)
                                + 1 as libc::c_int as libc::c_long)
                                << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                - 1 as libc::c_int as libc::c_long)
                                * 2 as libc::c_int as libc::c_long
                                + 1 as libc::c_int as libc::c_long)
                        } else {
                            0 as libc::c_int as libc::c_long
                                * (0 as libc::c_int as libc::c_long * sec_adjustment + t)
                                + 0 as libc::c_int as libc::c_long
                        }) < 0 as libc::c_int as libc::c_long
                        {
                            if sec_adjustment < 0 as libc::c_int as libc::c_long {
                                (t
                                    < (if (0 as libc::c_int as libc::c_long
                                        * (0 as libc::c_int as libc::c_long * sec_adjustment + t)
                                        - 1 as libc::c_int as libc::c_long)
                                        < 0 as libc::c_int as libc::c_long
                                    {
                                        !((((0 as libc::c_int as libc::c_long
                                            * (0 as libc::c_int as libc::c_long * sec_adjustment + t)
                                            + 1 as libc::c_int as libc::c_long)
                                            << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                            - 1 as libc::c_int as libc::c_long)
                                            * 2 as libc::c_int as libc::c_long
                                            + 1 as libc::c_int as libc::c_long)
                                    } else {
                                        0 as libc::c_int as libc::c_long
                                            * (0 as libc::c_int as libc::c_long * sec_adjustment + t)
                                            + 0 as libc::c_int as libc::c_long
                                    }) - sec_adjustment) as libc::c_int
                            } else {
                                ((if (0 as libc::c_int as libc::c_long
                                    * (0 as libc::c_int as libc::c_long * sec_adjustment + t)
                                    - 1 as libc::c_int as libc::c_long)
                                    < 0 as libc::c_int as libc::c_long
                                {
                                    (((0 as libc::c_int as libc::c_long
                                        * (0 as libc::c_int as libc::c_long * sec_adjustment + t)
                                        + 1 as libc::c_int as libc::c_long)
                                        << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                        - 1 as libc::c_int as libc::c_long)
                                        * 2 as libc::c_int as libc::c_long
                                        + 1 as libc::c_int as libc::c_long
                                } else {
                                    0 as libc::c_int as libc::c_long
                                        * (0 as libc::c_int as libc::c_long * sec_adjustment + t)
                                        - 1 as libc::c_int as libc::c_long
                                }) - sec_adjustment < t) as libc::c_int
                            }
                        } else {
                            if t < 0 as libc::c_int as libc::c_long {
                                (sec_adjustment <= t + sec_adjustment) as libc::c_int
                            } else {
                                if sec_adjustment < 0 as libc::c_int as libc::c_long {
                                    (t <= t + sec_adjustment) as libc::c_int
                                } else {
                                    (t + sec_adjustment < sec_adjustment) as libc::c_int
                                }
                            }
                        }) != 0
                            || (0 as libc::c_int as libc::c_long * (t + sec_adjustment)
                                - 1 as libc::c_int as libc::c_long)
                                < 0 as libc::c_int as libc::c_long
                                && t + sec_adjustment
                                    < (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                        as libc::c_long
                            || (2147483647 as libc::c_int as libc::c_long)
                                < t + sec_adjustment
                        {
                            t = (t as libc::c_uint)
                                .wrapping_add(sec_adjustment as libc::c_uint) as libc::c_int
                                as long_int;
                            1 as libc::c_int
                        } else {
                            t = (t as libc::c_uint)
                                .wrapping_add(sec_adjustment as libc::c_uint) as libc::c_int
                                as long_int;
                            0 as libc::c_int
                        }
                    }
                } else {
                    if ::std::mem::size_of::<long_int>() as libc::c_ulong
                        == ::std::mem::size_of::<libc::c_long>() as libc::c_ulong
                    {
                        if (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                            < ::std::mem::size_of::<libc::c_long>() as libc::c_ulong
                        {
                            if (if (if (0 as libc::c_int as libc::c_long
                                * (0 as libc::c_int as libc::c_long * sec_adjustment + t)
                                - 1 as libc::c_int as libc::c_long)
                                < 0 as libc::c_int as libc::c_long
                            {
                                !((((0 as libc::c_int as libc::c_long
                                    * (0 as libc::c_int as libc::c_long * sec_adjustment + t)
                                    + 1 as libc::c_int as libc::c_long)
                                    << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                    - 1 as libc::c_int as libc::c_long)
                                    * 2 as libc::c_int as libc::c_long
                                    + 1 as libc::c_int as libc::c_long)
                            } else {
                                0 as libc::c_int as libc::c_long
                                    * (0 as libc::c_int as libc::c_long * sec_adjustment + t)
                                    + 0 as libc::c_int as libc::c_long
                            }) < 0 as libc::c_int as libc::c_long
                            {
                                if sec_adjustment < 0 as libc::c_int as libc::c_long {
                                    (t
                                        < (if (0 as libc::c_int as libc::c_long
                                            * (0 as libc::c_int as libc::c_long * sec_adjustment + t)
                                            - 1 as libc::c_int as libc::c_long)
                                            < 0 as libc::c_int as libc::c_long
                                        {
                                            !((((0 as libc::c_int as libc::c_long
                                                * (0 as libc::c_int as libc::c_long * sec_adjustment + t)
                                                + 1 as libc::c_int as libc::c_long)
                                                << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                - 1 as libc::c_int as libc::c_long)
                                                * 2 as libc::c_int as libc::c_long
                                                + 1 as libc::c_int as libc::c_long)
                                        } else {
                                            0 as libc::c_int as libc::c_long
                                                * (0 as libc::c_int as libc::c_long * sec_adjustment + t)
                                                + 0 as libc::c_int as libc::c_long
                                        }) - sec_adjustment) as libc::c_int
                                } else {
                                    ((if (0 as libc::c_int as libc::c_long
                                        * (0 as libc::c_int as libc::c_long * sec_adjustment + t)
                                        - 1 as libc::c_int as libc::c_long)
                                        < 0 as libc::c_int as libc::c_long
                                    {
                                        (((0 as libc::c_int as libc::c_long
                                            * (0 as libc::c_int as libc::c_long * sec_adjustment + t)
                                            + 1 as libc::c_int as libc::c_long)
                                            << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                            - 1 as libc::c_int as libc::c_long)
                                            * 2 as libc::c_int as libc::c_long
                                            + 1 as libc::c_int as libc::c_long
                                    } else {
                                        0 as libc::c_int as libc::c_long
                                            * (0 as libc::c_int as libc::c_long * sec_adjustment + t)
                                            - 1 as libc::c_int as libc::c_long
                                    }) - sec_adjustment < t) as libc::c_int
                                }
                            } else {
                                if t < 0 as libc::c_int as libc::c_long {
                                    (sec_adjustment <= t + sec_adjustment) as libc::c_int
                                } else {
                                    if sec_adjustment < 0 as libc::c_int as libc::c_long {
                                        (t <= t + sec_adjustment) as libc::c_int
                                    } else {
                                        (t + sec_adjustment < sec_adjustment) as libc::c_int
                                    }
                                }
                            }) != 0
                                || (0 as libc::c_int as libc::c_long * (t + sec_adjustment)
                                    - 1 as libc::c_int as libc::c_long)
                                    < 0 as libc::c_int as libc::c_long
                                    && t + sec_adjustment
                                        < -(9223372036854775807 as libc::c_long) - 1 as libc::c_long
                                || (9223372036854775807 as libc::c_long)
                                    < t + sec_adjustment
                            {
                                t = (t as libc::c_ulong)
                                    .wrapping_add(sec_adjustment as libc::c_ulong)
                                    as libc::c_long;
                                1 as libc::c_int
                            } else {
                                t = (t as libc::c_ulong)
                                    .wrapping_add(sec_adjustment as libc::c_ulong)
                                    as libc::c_long;
                                0 as libc::c_int
                            }
                        } else {
                            if (if (if (0 as libc::c_int as libc::c_long
                                * (0 as libc::c_int as libc::c_long * sec_adjustment + t)
                                - 1 as libc::c_int as libc::c_long)
                                < 0 as libc::c_int as libc::c_long
                            {
                                !((((0 as libc::c_int as libc::c_long
                                    * (0 as libc::c_int as libc::c_long * sec_adjustment + t)
                                    + 1 as libc::c_int as libc::c_long)
                                    << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                    - 1 as libc::c_int as libc::c_long)
                                    * 2 as libc::c_int as libc::c_long
                                    + 1 as libc::c_int as libc::c_long)
                            } else {
                                0 as libc::c_int as libc::c_long
                                    * (0 as libc::c_int as libc::c_long * sec_adjustment + t)
                                    + 0 as libc::c_int as libc::c_long
                            }) < 0 as libc::c_int as libc::c_long
                            {
                                if sec_adjustment < 0 as libc::c_int as libc::c_long {
                                    (t
                                        < (if (0 as libc::c_int as libc::c_long
                                            * (0 as libc::c_int as libc::c_long * sec_adjustment + t)
                                            - 1 as libc::c_int as libc::c_long)
                                            < 0 as libc::c_int as libc::c_long
                                        {
                                            !((((0 as libc::c_int as libc::c_long
                                                * (0 as libc::c_int as libc::c_long * sec_adjustment + t)
                                                + 1 as libc::c_int as libc::c_long)
                                                << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                - 1 as libc::c_int as libc::c_long)
                                                * 2 as libc::c_int as libc::c_long
                                                + 1 as libc::c_int as libc::c_long)
                                        } else {
                                            0 as libc::c_int as libc::c_long
                                                * (0 as libc::c_int as libc::c_long * sec_adjustment + t)
                                                + 0 as libc::c_int as libc::c_long
                                        }) - sec_adjustment) as libc::c_int
                                } else {
                                    ((if (0 as libc::c_int as libc::c_long
                                        * (0 as libc::c_int as libc::c_long * sec_adjustment + t)
                                        - 1 as libc::c_int as libc::c_long)
                                        < 0 as libc::c_int as libc::c_long
                                    {
                                        (((0 as libc::c_int as libc::c_long
                                            * (0 as libc::c_int as libc::c_long * sec_adjustment + t)
                                            + 1 as libc::c_int as libc::c_long)
                                            << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                            - 1 as libc::c_int as libc::c_long)
                                            * 2 as libc::c_int as libc::c_long
                                            + 1 as libc::c_int as libc::c_long
                                    } else {
                                        0 as libc::c_int as libc::c_long
                                            * (0 as libc::c_int as libc::c_long * sec_adjustment + t)
                                            - 1 as libc::c_int as libc::c_long
                                    }) - sec_adjustment < t) as libc::c_int
                                }
                            } else {
                                if t < 0 as libc::c_int as libc::c_long {
                                    (sec_adjustment <= t + sec_adjustment) as libc::c_int
                                } else {
                                    if sec_adjustment < 0 as libc::c_int as libc::c_long {
                                        (t <= t + sec_adjustment) as libc::c_int
                                    } else {
                                        (t + sec_adjustment < sec_adjustment) as libc::c_int
                                    }
                                }
                            }) != 0
                                || (0 as libc::c_int as libc::c_long * (t + sec_adjustment)
                                    - 1 as libc::c_int as libc::c_long)
                                    < 0 as libc::c_int as libc::c_long
                                    && t + sec_adjustment
                                        < -(9223372036854775807 as libc::c_long) - 1 as libc::c_long
                                || (9223372036854775807 as libc::c_long)
                                    < t + sec_adjustment
                            {
                                t = (t as libc::c_ulong)
                                    .wrapping_add(sec_adjustment as libc::c_ulong)
                                    as libc::c_long;
                                1 as libc::c_int
                            } else {
                                t = (t as libc::c_ulong)
                                    .wrapping_add(sec_adjustment as libc::c_ulong)
                                    as libc::c_long;
                                0 as libc::c_int
                            }
                        }
                    } else {
                        if (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                            < ::std::mem::size_of::<libc::c_longlong>() as libc::c_ulong
                        {
                            if (if (if (0 as libc::c_int as libc::c_longlong
                                * (0 as libc::c_int as libc::c_longlong
                                    * sec_adjustment as libc::c_longlong
                                    + t as libc::c_longlong)
                                - 1 as libc::c_int as libc::c_longlong)
                                < 0 as libc::c_int as libc::c_longlong
                            {
                                !((((0 as libc::c_int as libc::c_longlong
                                    * (0 as libc::c_int as libc::c_longlong
                                        * sec_adjustment as libc::c_longlong
                                        + t as libc::c_longlong)
                                    + 1 as libc::c_int as libc::c_longlong)
                                    << (::std::mem::size_of::<libc::c_longlong>()
                                        as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                    - 1 as libc::c_int as libc::c_longlong)
                                    * 2 as libc::c_int as libc::c_longlong
                                    + 1 as libc::c_int as libc::c_longlong)
                            } else {
                                0 as libc::c_int as libc::c_longlong
                                    * (0 as libc::c_int as libc::c_longlong
                                        * sec_adjustment as libc::c_longlong
                                        + t as libc::c_longlong)
                                    + 0 as libc::c_int as libc::c_longlong
                            }) < 0 as libc::c_int as libc::c_longlong
                            {
                                if (sec_adjustment as libc::c_longlong)
                                    < 0 as libc::c_int as libc::c_longlong
                                {
                                    ((t as libc::c_longlong)
                                        < (if (0 as libc::c_int as libc::c_longlong
                                            * (0 as libc::c_int as libc::c_longlong
                                                * sec_adjustment as libc::c_longlong
                                                + t as libc::c_longlong)
                                            - 1 as libc::c_int as libc::c_longlong)
                                            < 0 as libc::c_int as libc::c_longlong
                                        {
                                            !((((0 as libc::c_int as libc::c_longlong
                                                * (0 as libc::c_int as libc::c_longlong
                                                    * sec_adjustment as libc::c_longlong
                                                    + t as libc::c_longlong)
                                                + 1 as libc::c_int as libc::c_longlong)
                                                << (::std::mem::size_of::<libc::c_longlong>()
                                                    as libc::c_ulong)
                                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                - 1 as libc::c_int as libc::c_longlong)
                                                * 2 as libc::c_int as libc::c_longlong
                                                + 1 as libc::c_int as libc::c_longlong)
                                        } else {
                                            0 as libc::c_int as libc::c_longlong
                                                * (0 as libc::c_int as libc::c_longlong
                                                    * sec_adjustment as libc::c_longlong
                                                    + t as libc::c_longlong)
                                                + 0 as libc::c_int as libc::c_longlong
                                        }) - sec_adjustment as libc::c_longlong) as libc::c_int
                                } else {
                                    (((if (0 as libc::c_int as libc::c_longlong
                                        * (0 as libc::c_int as libc::c_longlong
                                            * sec_adjustment as libc::c_longlong
                                            + t as libc::c_longlong)
                                        - 1 as libc::c_int as libc::c_longlong)
                                        < 0 as libc::c_int as libc::c_longlong
                                    {
                                        (((0 as libc::c_int as libc::c_longlong
                                            * (0 as libc::c_int as libc::c_longlong
                                                * sec_adjustment as libc::c_longlong
                                                + t as libc::c_longlong)
                                            + 1 as libc::c_int as libc::c_longlong)
                                            << (::std::mem::size_of::<libc::c_longlong>()
                                                as libc::c_ulong)
                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                            - 1 as libc::c_int as libc::c_longlong)
                                            * 2 as libc::c_int as libc::c_longlong
                                            + 1 as libc::c_int as libc::c_longlong
                                    } else {
                                        0 as libc::c_int as libc::c_longlong
                                            * (0 as libc::c_int as libc::c_longlong
                                                * sec_adjustment as libc::c_longlong
                                                + t as libc::c_longlong)
                                            - 1 as libc::c_int as libc::c_longlong
                                    }) - sec_adjustment as libc::c_longlong)
                                        < t as libc::c_longlong) as libc::c_int
                                }
                            } else {
                                if (t as libc::c_longlong)
                                    < 0 as libc::c_int as libc::c_longlong
                                {
                                    (sec_adjustment as libc::c_longlong
                                        <= t as libc::c_longlong
                                            + sec_adjustment as libc::c_longlong) as libc::c_int
                                } else {
                                    if (sec_adjustment as libc::c_longlong)
                                        < 0 as libc::c_int as libc::c_longlong
                                    {
                                        (t as libc::c_longlong
                                            <= t as libc::c_longlong
                                                + sec_adjustment as libc::c_longlong) as libc::c_int
                                    } else {
                                        ((t as libc::c_longlong
                                            + sec_adjustment as libc::c_longlong)
                                            < sec_adjustment as libc::c_longlong) as libc::c_int
                                    }
                                }
                            }) != 0
                                || (0 as libc::c_int as libc::c_longlong
                                    * (t as libc::c_longlong
                                        + sec_adjustment as libc::c_longlong)
                                    - 1 as libc::c_int as libc::c_longlong)
                                    < 0 as libc::c_int as libc::c_longlong
                                    && (t as libc::c_longlong
                                        + sec_adjustment as libc::c_longlong)
                                        < -(9223372036854775807 as libc::c_longlong)
                                            - 1 as libc::c_longlong
                                || (9223372036854775807 as libc::c_longlong)
                                    < t as libc::c_longlong + sec_adjustment as libc::c_longlong
                            {
                                t = (t as libc::c_longlong as libc::c_ulonglong)
                                    .wrapping_add(
                                        sec_adjustment as libc::c_longlong as libc::c_ulonglong,
                                    ) as libc::c_longlong as long_int;
                                1 as libc::c_int
                            } else {
                                t = (t as libc::c_longlong as libc::c_ulonglong)
                                    .wrapping_add(
                                        sec_adjustment as libc::c_longlong as libc::c_ulonglong,
                                    ) as libc::c_longlong as long_int;
                                0 as libc::c_int
                            }
                        } else {
                            if (if (if (0 as libc::c_int as libc::c_long
                                * (0 as libc::c_int as libc::c_long * sec_adjustment + t)
                                - 1 as libc::c_int as libc::c_long)
                                < 0 as libc::c_int as libc::c_long
                            {
                                !((((0 as libc::c_int as libc::c_long
                                    * (0 as libc::c_int as libc::c_long * sec_adjustment + t)
                                    + 1 as libc::c_int as libc::c_long)
                                    << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                    - 1 as libc::c_int as libc::c_long)
                                    * 2 as libc::c_int as libc::c_long
                                    + 1 as libc::c_int as libc::c_long)
                            } else {
                                0 as libc::c_int as libc::c_long
                                    * (0 as libc::c_int as libc::c_long * sec_adjustment + t)
                                    + 0 as libc::c_int as libc::c_long
                            }) < 0 as libc::c_int as libc::c_long
                            {
                                if sec_adjustment < 0 as libc::c_int as libc::c_long {
                                    (t
                                        < (if (0 as libc::c_int as libc::c_long
                                            * (0 as libc::c_int as libc::c_long * sec_adjustment + t)
                                            - 1 as libc::c_int as libc::c_long)
                                            < 0 as libc::c_int as libc::c_long
                                        {
                                            !((((0 as libc::c_int as libc::c_long
                                                * (0 as libc::c_int as libc::c_long * sec_adjustment + t)
                                                + 1 as libc::c_int as libc::c_long)
                                                << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                - 1 as libc::c_int as libc::c_long)
                                                * 2 as libc::c_int as libc::c_long
                                                + 1 as libc::c_int as libc::c_long)
                                        } else {
                                            0 as libc::c_int as libc::c_long
                                                * (0 as libc::c_int as libc::c_long * sec_adjustment + t)
                                                + 0 as libc::c_int as libc::c_long
                                        }) - sec_adjustment) as libc::c_int
                                } else {
                                    ((if (0 as libc::c_int as libc::c_long
                                        * (0 as libc::c_int as libc::c_long * sec_adjustment + t)
                                        - 1 as libc::c_int as libc::c_long)
                                        < 0 as libc::c_int as libc::c_long
                                    {
                                        (((0 as libc::c_int as libc::c_long
                                            * (0 as libc::c_int as libc::c_long * sec_adjustment + t)
                                            + 1 as libc::c_int as libc::c_long)
                                            << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                            - 1 as libc::c_int as libc::c_long)
                                            * 2 as libc::c_int as libc::c_long
                                            + 1 as libc::c_int as libc::c_long
                                    } else {
                                        0 as libc::c_int as libc::c_long
                                            * (0 as libc::c_int as libc::c_long * sec_adjustment + t)
                                            - 1 as libc::c_int as libc::c_long
                                    }) - sec_adjustment < t) as libc::c_int
                                }
                            } else {
                                if t < 0 as libc::c_int as libc::c_long {
                                    (sec_adjustment <= t + sec_adjustment) as libc::c_int
                                } else {
                                    if sec_adjustment < 0 as libc::c_int as libc::c_long {
                                        (t <= t + sec_adjustment) as libc::c_int
                                    } else {
                                        (t + sec_adjustment < sec_adjustment) as libc::c_int
                                    }
                                }
                            }) != 0
                                || (0 as libc::c_int as libc::c_long * (t + sec_adjustment)
                                    - 1 as libc::c_int as libc::c_long)
                                    < 0 as libc::c_int as libc::c_long
                                    && ((t + sec_adjustment) as libc::c_longlong)
                                        < -(9223372036854775807 as libc::c_longlong)
                                            - 1 as libc::c_longlong
                                || (9223372036854775807 as libc::c_longlong)
                                    < (t + sec_adjustment) as libc::c_longlong
                            {
                                t = (t as libc::c_ulonglong)
                                    .wrapping_add(sec_adjustment as libc::c_ulonglong)
                                    as libc::c_longlong as long_int;
                                1 as libc::c_int
                            } else {
                                t = (t as libc::c_ulonglong)
                                    .wrapping_add(sec_adjustment as libc::c_ulonglong)
                                    as libc::c_longlong as long_int;
                                0 as libc::c_int
                            }
                        }
                    }
                }
            }
        }) != 0 || !(mktime_min <= t && t <= mktime_max)
            || (convert_time(convert, t, &mut tm)).is_null()
        {
            return -(1 as libc::c_int) as time_t;
        }
    }
    *tp = tm;
    return t;
}
unsafe extern "C" fn run_static_initializers() {
    mktime_min = if !((0 as libc::c_int as time_t) < -(1 as libc::c_int) as time_t)
        && !(if (0 as libc::c_int as time_t) < -(1 as libc::c_int) as time_t {
            -(1 as libc::c_int) as time_t
        } else {
            (((1 as libc::c_int as time_t)
                << (::std::mem::size_of::<time_t>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                - 1 as libc::c_int as libc::c_long) * 2 as libc::c_int as libc::c_long
                + 1 as libc::c_int as libc::c_long
        })
            < !(if (0 as libc::c_int as long_int) < -(1 as libc::c_int) as long_int {
                -(1 as libc::c_int) as long_int
            } else {
                (((1 as libc::c_int as long_int)
                    << (::std::mem::size_of::<long_int>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                    - 1 as libc::c_int as libc::c_long)
                    * 2 as libc::c_int as libc::c_long + 1 as libc::c_int as libc::c_long
            })
    {
        !if (0 as libc::c_int as long_int) < -(1 as libc::c_int) as long_int {
            -(1 as libc::c_int) as long_int
        } else {
            (((1 as libc::c_int as long_int)
                << (::std::mem::size_of::<long_int>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                - 1 as libc::c_int as libc::c_long) * 2 as libc::c_int as libc::c_long
                + 1 as libc::c_int as libc::c_long
        }
    } else {
        !if (0 as libc::c_int as time_t) < -(1 as libc::c_int) as time_t {
            -(1 as libc::c_int) as time_t
        } else {
            (((1 as libc::c_int as time_t)
                << (::std::mem::size_of::<time_t>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                - 1 as libc::c_int as libc::c_long) * 2 as libc::c_int as libc::c_long
                + 1 as libc::c_int as libc::c_long
        }
    };
    mktime_max = if (if (0 as libc::c_int as long_int) < -(1 as libc::c_int) as long_int
    {
        -(1 as libc::c_int) as long_int
    } else {
        (((1 as libc::c_int as long_int)
            << (::std::mem::size_of::<long_int>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
            - 1 as libc::c_int as libc::c_long) * 2 as libc::c_int as libc::c_long
            + 1 as libc::c_int as libc::c_long
    })
        < (if (0 as libc::c_int as time_t) < -(1 as libc::c_int) as time_t {
            -(1 as libc::c_int) as time_t
        } else {
            (((1 as libc::c_int as time_t)
                << (::std::mem::size_of::<time_t>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                - 1 as libc::c_int as libc::c_long) * 2 as libc::c_int as libc::c_long
                + 1 as libc::c_int as libc::c_long
        })
    {
        if (0 as libc::c_int as long_int) < -(1 as libc::c_int) as long_int {
            -(1 as libc::c_int) as long_int
        } else {
            (((1 as libc::c_int as long_int)
                << (::std::mem::size_of::<long_int>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                - 1 as libc::c_int as libc::c_long) * 2 as libc::c_int as libc::c_long
                + 1 as libc::c_int as libc::c_long
        }
    } else if (0 as libc::c_int as time_t) < -(1 as libc::c_int) as time_t {
        -(1 as libc::c_int) as time_t
    } else {
        (((1 as libc::c_int as time_t)
            << (::std::mem::size_of::<time_t>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
            - 1 as libc::c_int as libc::c_long) * 2 as libc::c_int as libc::c_long
            + 1 as libc::c_int as libc::c_long
    };
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
