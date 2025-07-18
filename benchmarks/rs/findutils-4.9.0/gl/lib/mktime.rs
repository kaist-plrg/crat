use ::libc;
extern "C" {
    fn __errno_location() -> *mut libc::c_int;
}
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
pub type mktime_offset_t = time_t;
pub type long_int = libc::c_long;
unsafe extern "C" fn shr(mut a: long_int, mut b: libc::c_int) -> long_int {
    let mut one: long_int = 1 as libc::c_int as long_int;
    return if -one >> 1 as libc::c_int == -(1 as libc::c_int) as libc::c_long {
        a >> b
    } else {
        (a + (a < 0 as libc::c_int as libc::c_long) as libc::c_int as libc::c_long)
            / (one << b)
            - (a < 0 as libc::c_int as libc::c_long) as libc::c_int as libc::c_long
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
    let mut a4: libc::c_int = (shr(year1, 2 as libc::c_int)
        + shr(1900 as libc::c_int as long_int, 2 as libc::c_int)
        - (year1 & 3 as libc::c_int as libc::c_long == 0) as libc::c_int as libc::c_long)
        as libc::c_int;
    let mut b4: libc::c_int = (shr(year0 as long_int, 2 as libc::c_int)
        + shr(1900 as libc::c_int as long_int, 2 as libc::c_int)
        - (year0 & 3 as libc::c_int == 0) as libc::c_int as libc::c_long) as libc::c_int;
    let mut a100: libc::c_int = (a4 + (a4 < 0 as libc::c_int) as libc::c_int)
        / 25 as libc::c_int - (a4 < 0 as libc::c_int) as libc::c_int;
    let mut b100: libc::c_int = (b4 + (b4 < 0 as libc::c_int) as libc::c_int)
        / 25 as libc::c_int - (b4 < 0 as libc::c_int) as libc::c_int;
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
unsafe extern "C" fn tm_diff(
    mut year: long_int,
    mut yday: long_int,
    mut hour: libc::c_int,
    mut min: libc::c_int,
    mut sec: libc::c_int,
    mut tp: *const tm,
) -> long_int {
    return ydhms_diff(
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
    let mut t1: long_int = if *t < mktime_min {
        mktime_min
    } else if *t <= mktime_max {
        *t
    } else {
        mktime_max
    };
    let mut r: *mut tm = convert_time(convert, t1, tp);
    if !r.is_null() {
        *t = t1;
        return r;
    }
    if *__errno_location() != 75 as libc::c_int {
        return 0 as *mut tm;
    }
    let mut bad: long_int = t1;
    let mut ok: long_int = 0 as libc::c_int as long_int;
    let mut oktm: tm = tm {
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
    oktm.tm_sec = -(1 as libc::c_int);
    loop {
        let mut mid: long_int = long_int_avg(ok, bad);
        if mid == ok || mid == bad {
            break;
        }
        if !(convert_time(convert, mid, tp)).is_null() {
            ok = mid;
            oktm = *tp;
        } else if *__errno_location() != 75 as libc::c_int {
            return 0 as *mut tm
        } else {
            bad = mid;
        }
    }
    if oktm.tm_sec < 0 as libc::c_int {
        return 0 as *mut tm;
    }
    *t = ok;
    *tp = oktm;
    return tp;
}
pub unsafe extern "C" fn mktime_internal(
    mut tp: *mut tm,
    mut convert: Option::<unsafe extern "C" fn(*const time_t, *mut tm) -> *mut tm>,
    mut offset: *mut mktime_offset_t,
) -> time_t {
    let mut current_block: u64;
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
    let mut dst2: libc::c_int = 0 as libc::c_int;
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
    let mut off: mktime_offset_t = *offset;
    let mut negative_offset_guess: libc::c_int = 0;
    let mut sec_requested: libc::c_int = sec;
    if sec < 0 as libc::c_int {
        sec = 0 as libc::c_int;
    }
    if (59 as libc::c_int) < sec {
        sec = 59 as libc::c_int;
    }
    let (fresh0, fresh1) = (0 as libc::c_int).overflowing_sub(off.try_into().unwrap());
    *&mut negative_offset_guess = fresh0;
    let mut t0: long_int = ydhms_diff(
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
    let mut t: long_int = t0;
    let mut t1: long_int = t0;
    let mut t2: long_int = t0;
    loop {
        if (ranged_convert(convert, &mut t, &mut tm)).is_null() {
            return -(1 as libc::c_int) as time_t;
        }
        let mut dt: long_int = tm_diff(year, yday, hour, min, sec, &mut tm);
        if dt == 0 as libc::c_int as libc::c_long {
            current_block = 10652014663920648156;
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
            current_block = 15349183548924829661;
            break;
        }
        remaining_probes -= 1;
        remaining_probes;
        if remaining_probes == 0 as libc::c_int {
            *__errno_location() = 75 as libc::c_int;
            return -(1 as libc::c_int) as time_t;
        }
        t1 = t2;
        t2 = t;
        t += dt;
        dst2 = (tm.tm_isdst != 0 as libc::c_int) as libc::c_int;
    }
    match current_block {
        10652014663920648156 => {
            if isdst_differ(isdst, tm.tm_isdst) {
                let mut stride: libc::c_int = 601200 as libc::c_int;
                let mut duration_max: libc::c_int = 536454000 as libc::c_int;
                let mut delta_bound: libc::c_int = duration_max / 2 as libc::c_int
                    + stride;
                let mut delta: libc::c_int = 0;
                let mut direction: libc::c_int = 0;
                delta = stride;
                's_133: loop {
                    if !(delta < delta_bound) {
                        current_block = 7427571413727699167;
                        break;
                    }
                    direction = -(1 as libc::c_int);
                    while direction <= 1 as libc::c_int {
                        let mut ot: long_int = 0;
                        let (fresh2, fresh3) = t.overflowing_add((delta * direction).into());
                        *(&mut ot as *mut long_int) = fresh2;
                        if !fresh3 {
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
                            if (ranged_convert(convert, &mut ot, &mut otm)).is_null() {
                                return -(1 as libc::c_int) as time_t;
                            }
                            if !isdst_differ(isdst, otm.tm_isdst) {
                                let mut gt: long_int = ot
                                    + tm_diff(year, yday, hour, min, sec, &mut otm);
                                if mktime_min <= gt && gt <= mktime_max {
                                    if !(convert_time(convert, gt, &mut tm)).is_null() {
                                        t = gt;
                                        current_block = 15349183548924829661;
                                        break 's_133;
                                    } else if *__errno_location() != 75 as libc::c_int {
                                        return -(1 as libc::c_int) as time_t
                                    }
                                }
                            }
                        }
                        direction += 2 as libc::c_int;
                    }
                    delta += stride;
                }
                match current_block {
                    15349183548924829661 => {}
                    _ => {
                        *__errno_location() = 75 as libc::c_int;
                        return -(1 as libc::c_int) as time_t;
                    }
                }
            }
        }
        _ => {}
    }
    let (fresh4, fresh5) = t.overflowing_sub(t0);
    *offset = fresh4;
    let (fresh6, fresh7) = (*offset).overflowing_sub(negative_offset_guess.into());
    *offset = fresh6;
    if 1 as libc::c_int != 0 && sec_requested != tm.tm_sec {
        let mut sec_adjustment: long_int = (sec == 0 as libc::c_int
            && tm.tm_sec == 60 as libc::c_int) as libc::c_int as long_int;
        sec_adjustment -= sec as libc::c_long;
        sec_adjustment += sec_requested as libc::c_long;
        let (fresh8, fresh9) = t.overflowing_add(sec_adjustment);
        *(&mut t as *mut long_int) = fresh8;
        if fresh9 as libc::c_int != 0 || !(mktime_min <= t && t <= mktime_max) {
            *__errno_location() = 75 as libc::c_int;
            return -(1 as libc::c_int) as time_t;
        }
        if (convert_time(convert, t, &mut tm)).is_null() {
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
