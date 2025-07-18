use ::libc;
extern "C" {
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __ctype_tolower_loc() -> *mut *const __int32_t;
}
pub type __int32_t = libc::c_int;
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
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct partime {
    pub tm: tm,
    pub ymodulus: libc::c_int,
    pub yweek: libc::c_int,
    pub zone: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct name_val {
    pub name: [libc::c_char; 4],
    pub val: libc::c_int,
}
#[inline]
unsafe extern "C" fn tolower(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_tolower_loc()).offset(__c as isize)
    } else {
        __c
    };
}
static mut month_names: [name_val; 13] = unsafe {
    [
        {
            let mut init = name_val {
                name: *::std::mem::transmute::<
                    &[u8; 4],
                    &mut [libc::c_char; 4],
                >(b"jan\0"),
                val: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = name_val {
                name: *::std::mem::transmute::<
                    &[u8; 4],
                    &mut [libc::c_char; 4],
                >(b"feb\0"),
                val: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = name_val {
                name: *::std::mem::transmute::<
                    &[u8; 4],
                    &mut [libc::c_char; 4],
                >(b"mar\0"),
                val: 2 as libc::c_int,
            };
            init
        },
        {
            let mut init = name_val {
                name: *::std::mem::transmute::<
                    &[u8; 4],
                    &mut [libc::c_char; 4],
                >(b"apr\0"),
                val: 3 as libc::c_int,
            };
            init
        },
        {
            let mut init = name_val {
                name: *::std::mem::transmute::<
                    &[u8; 4],
                    &mut [libc::c_char; 4],
                >(b"may\0"),
                val: 4 as libc::c_int,
            };
            init
        },
        {
            let mut init = name_val {
                name: *::std::mem::transmute::<
                    &[u8; 4],
                    &mut [libc::c_char; 4],
                >(b"jun\0"),
                val: 5 as libc::c_int,
            };
            init
        },
        {
            let mut init = name_val {
                name: *::std::mem::transmute::<
                    &[u8; 4],
                    &mut [libc::c_char; 4],
                >(b"jul\0"),
                val: 6 as libc::c_int,
            };
            init
        },
        {
            let mut init = name_val {
                name: *::std::mem::transmute::<
                    &[u8; 4],
                    &mut [libc::c_char; 4],
                >(b"aug\0"),
                val: 7 as libc::c_int,
            };
            init
        },
        {
            let mut init = name_val {
                name: *::std::mem::transmute::<
                    &[u8; 4],
                    &mut [libc::c_char; 4],
                >(b"sep\0"),
                val: 8 as libc::c_int,
            };
            init
        },
        {
            let mut init = name_val {
                name: *::std::mem::transmute::<
                    &[u8; 4],
                    &mut [libc::c_char; 4],
                >(b"oct\0"),
                val: 9 as libc::c_int,
            };
            init
        },
        {
            let mut init = name_val {
                name: *::std::mem::transmute::<
                    &[u8; 4],
                    &mut [libc::c_char; 4],
                >(b"nov\0"),
                val: 10 as libc::c_int,
            };
            init
        },
        {
            let mut init = name_val {
                name: *::std::mem::transmute::<
                    &[u8; 4],
                    &mut [libc::c_char; 4],
                >(b"dec\0"),
                val: 11 as libc::c_int,
            };
            init
        },
        {
            let mut init = name_val {
                name: *::std::mem::transmute::<
                    &[u8; 4],
                    &mut [libc::c_char; 4],
                >(b"\0\0\0\0"),
                val: -(1 as libc::c_int),
            };
            init
        },
    ]
};
static mut weekday_names: [name_val; 8] = unsafe {
    [
        {
            let mut init = name_val {
                name: *::std::mem::transmute::<
                    &[u8; 4],
                    &mut [libc::c_char; 4],
                >(b"sun\0"),
                val: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = name_val {
                name: *::std::mem::transmute::<
                    &[u8; 4],
                    &mut [libc::c_char; 4],
                >(b"mon\0"),
                val: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = name_val {
                name: *::std::mem::transmute::<
                    &[u8; 4],
                    &mut [libc::c_char; 4],
                >(b"tue\0"),
                val: 2 as libc::c_int,
            };
            init
        },
        {
            let mut init = name_val {
                name: *::std::mem::transmute::<
                    &[u8; 4],
                    &mut [libc::c_char; 4],
                >(b"wed\0"),
                val: 3 as libc::c_int,
            };
            init
        },
        {
            let mut init = name_val {
                name: *::std::mem::transmute::<
                    &[u8; 4],
                    &mut [libc::c_char; 4],
                >(b"thu\0"),
                val: 4 as libc::c_int,
            };
            init
        },
        {
            let mut init = name_val {
                name: *::std::mem::transmute::<
                    &[u8; 4],
                    &mut [libc::c_char; 4],
                >(b"fri\0"),
                val: 5 as libc::c_int,
            };
            init
        },
        {
            let mut init = name_val {
                name: *::std::mem::transmute::<
                    &[u8; 4],
                    &mut [libc::c_char; 4],
                >(b"sat\0"),
                val: 6 as libc::c_int,
            };
            init
        },
        {
            let mut init = name_val {
                name: *::std::mem::transmute::<
                    &[u8; 4],
                    &mut [libc::c_char; 4],
                >(b"\0\0\0\0"),
                val: -(1 as libc::c_int),
            };
            init
        },
    ]
};
static mut zone_names: [name_val; 36] = [name_val { name: [0; 4], val: 0 }; 36];
unsafe extern "C" fn lookup(
    mut s: *const libc::c_char,
    mut table: *const name_val,
) -> libc::c_int {
    let mut j: libc::c_int = 0;
    let mut buf: [libc::c_char; 4] = [0 as libc::c_int as libc::c_char, 0, 0, 0];
    j = 0 as libc::c_int;
    while j < 4 as libc::c_int {
        let fresh0 = s;
        s = s.offset(1);
        let mut c: libc::c_uchar = *fresh0 as libc::c_uchar;
        buf[j
            as usize] = (if *(*__ctype_b_loc()).offset(c as libc::c_int as isize)
            as libc::c_int & _ISupper as libc::c_int as libc::c_ushort as libc::c_int
            != 0
        {
            {
                let mut __res: libc::c_int = 0;
                if ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong
                    > 1 as libc::c_int as libc::c_ulong
                {
                    if 0 != 0 {
                        let mut __c: libc::c_int = c as libc::c_int;
                        __res = if __c < -(128 as libc::c_int)
                            || __c > 255 as libc::c_int
                        {
                            __c
                        } else {
                            *(*__ctype_tolower_loc()).offset(__c as isize)
                        };
                    } else {
                        __res = tolower(c as libc::c_int);
                    }
                } else {
                    __res = *(*__ctype_tolower_loc()).offset(c as libc::c_int as isize);
                }
                __res
            }
        } else {
            c as libc::c_int
        }) as libc::c_char;
        if *(*__ctype_b_loc()).offset(c as libc::c_int as isize) as libc::c_int
            & _ISalpha as libc::c_int as libc::c_ushort as libc::c_int == 0
        {
            break;
        }
        j += 1;
        j;
    }
    's_30: while (*table.offset(0 as libc::c_int as isize))
        .name[0 as libc::c_int as usize] != 0
    {
        j = 0 as libc::c_int;
        while buf[j as usize] as libc::c_int
            == (*table.offset(0 as libc::c_int as isize)).name[j as usize] as libc::c_int
        {
            j += 1;
            if j == 4 as libc::c_int
                || (*table.offset(0 as libc::c_int as isize)).name[j as usize] == 0
            {
                break 's_30;
            }
        }
        table = table.offset(1);
        table;
    }
    return (*table.offset(0 as libc::c_int as isize)).val;
}
unsafe extern "C" fn undefine(mut t: *mut partime) {
    (*t).yweek = -(1 as libc::c_int);
    (*t).ymodulus = (*t).yweek;
    (*t).tm.tm_yday = (*t).ymodulus;
    (*t).tm.tm_wday = (*t).tm.tm_yday;
    (*t).tm.tm_year = (*t).tm.tm_wday;
    (*t).tm.tm_mon = (*t).tm.tm_year;
    (*t).tm.tm_mday = (*t).tm.tm_mon;
    (*t).tm.tm_hour = (*t).tm.tm_mday;
    (*t).tm.tm_min = (*t).tm.tm_hour;
    (*t).tm.tm_sec = (*t).tm.tm_min;
    (*t)
        .zone = -(24 as libc::c_int) as libc::c_long * 60 as libc::c_int as libc::c_long
        * 60 as libc::c_int as libc::c_long;
}
static mut patterns: [*const libc::c_char; 53] = [
    b"E_n_y\0" as *const u8 as *const libc::c_char,
    b"x\0" as *const u8 as *const libc::c_char,
    b"E_n\0" as *const u8 as *const libc::c_char,
    b"n_E\0" as *const u8 as *const libc::c_char,
    b"n\0" as *const u8 as *const libc::c_char,
    b"t:m:s_A\0" as *const u8 as *const libc::c_char,
    b"t:m_A\0" as *const u8 as *const libc::c_char,
    b"t_A\0" as *const u8 as *const libc::c_char,
    b"y/N/D$\0" as *const u8 as *const libc::c_char,
    b"y-N-D$\0" as *const u8 as *const libc::c_char,
    b"4ND$\0" as *const u8 as *const libc::c_char,
    b"Y-N$\0" as *const u8 as *const libc::c_char,
    b"RND$\0" as *const u8 as *const libc::c_char,
    b"-R=N$\0" as *const u8 as *const libc::c_char,
    b"-R$\0" as *const u8 as *const libc::c_char,
    b"--N=D$\0" as *const u8 as *const libc::c_char,
    b"N=DT\0" as *const u8 as *const libc::c_char,
    b"--N$\0" as *const u8 as *const libc::c_char,
    b"---D$\0" as *const u8 as *const libc::c_char,
    b"DT\0" as *const u8 as *const libc::c_char,
    b"Y-d$\0" as *const u8 as *const libc::c_char,
    b"4d$\0" as *const u8 as *const libc::c_char,
    b"R=d$\0" as *const u8 as *const libc::c_char,
    b"-d$\0" as *const u8 as *const libc::c_char,
    b"dT\0" as *const u8 as *const libc::c_char,
    b"y-W-X\0" as *const u8 as *const libc::c_char,
    b"yWX\0" as *const u8 as *const libc::c_char,
    b"y=W\0" as *const u8 as *const libc::c_char,
    b"-r-W-X\0" as *const u8 as *const libc::c_char,
    b"r-W-XT\0" as *const u8 as *const libc::c_char,
    b"-rWX\0" as *const u8 as *const libc::c_char,
    b"rWXT\0" as *const u8 as *const libc::c_char,
    b"-W=X\0" as *const u8 as *const libc::c_char,
    b"W=XT\0" as *const u8 as *const libc::c_char,
    b"-W\0" as *const u8 as *const libc::c_char,
    b"-w-X\0" as *const u8 as *const libc::c_char,
    b"w-XT\0" as *const u8 as *const libc::c_char,
    b"---X$\0" as *const u8 as *const libc::c_char,
    b"XT\0" as *const u8 as *const libc::c_char,
    b"4$\0" as *const u8 as *const libc::c_char,
    b"T\0" as *const u8 as *const libc::c_char,
    b"h:m:s$\0" as *const u8 as *const libc::c_char,
    b"hms$\0" as *const u8 as *const libc::c_char,
    b"h:m$\0" as *const u8 as *const libc::c_char,
    b"hm$\0" as *const u8 as *const libc::c_char,
    b"h$\0" as *const u8 as *const libc::c_char,
    b"-m:s$\0" as *const u8 as *const libc::c_char,
    b"-ms$\0" as *const u8 as *const libc::c_char,
    b"-m$\0" as *const u8 as *const libc::c_char,
    b"--s$\0" as *const u8 as *const libc::c_char,
    b"Y\0" as *const u8 as *const libc::c_char,
    b"Z\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
unsafe extern "C" fn parse_fixed(
    mut s: *const libc::c_char,
    mut digits: libc::c_int,
    mut res: *mut libc::c_int,
) -> *const libc::c_char {
    let mut n: libc::c_int = 0 as libc::c_int;
    let mut lim: *const libc::c_char = s.offset(digits as isize);
    while s < lim {
        let fresh1 = s;
        s = s.offset(1);
        let mut d: libc::c_uint = (*fresh1 as libc::c_int - '0' as i32) as libc::c_uint;
        if (9 as libc::c_int as libc::c_uint) < d {
            return 0 as *const libc::c_char;
        }
        n = ((10 as libc::c_int * n) as libc::c_uint).wrapping_add(d) as libc::c_int;
    }
    *res = n;
    return s;
}
unsafe extern "C" fn parse_ranged(
    mut s: *const libc::c_char,
    mut digits: libc::c_int,
    mut lo: libc::c_int,
    mut hi: libc::c_int,
    mut res: *mut libc::c_int,
) -> *const libc::c_char {
    s = parse_fixed(s, digits, res);
    return if !s.is_null() && lo <= *res && *res <= hi {
        s
    } else {
        0 as *const libc::c_char
    };
}
unsafe extern "C" fn parse_decimal(
    mut s: *const libc::c_char,
    mut digits: libc::c_int,
    mut lo: libc::c_int,
    mut hi: libc::c_int,
    mut resolution: libc::c_int,
    mut res: *mut libc::c_int,
    mut fres: *mut libc::c_int,
) -> *const libc::c_char {
    s = parse_fixed(s, digits, res);
    *fres = 0 as libc::c_int;
    if !s.is_null() && lo <= *res && *res <= hi {
        let mut f: libc::c_int = 0 as libc::c_int;
        if (*s.offset(0 as libc::c_int as isize) as libc::c_int == ',' as i32
            || *s.offset(0 as libc::c_int as isize) as libc::c_int == '.' as i32)
            && *(*__ctype_b_loc())
                .offset(
                    *s.offset(1 as libc::c_int as isize) as libc::c_uchar as libc::c_int
                        as isize,
                ) as libc::c_int
                & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
        {
            s = s.offset(1);
            let mut s1: *const libc::c_char = s;
            let mut num10: libc::c_int = 0 as libc::c_int;
            let mut denom10: libc::c_int = 10 as libc::c_int;
            let mut product: libc::c_int = 0;
            loop {
                s = s.offset(1);
                if !(*(*__ctype_b_loc())
                    .offset(*s as libc::c_uchar as libc::c_int as isize) as libc::c_int
                    & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0)
                {
                    break;
                }
                denom10 *= 10 as libc::c_int;
            }
            s = parse_fixed(
                s1,
                s.offset_from(s1) as libc::c_long as libc::c_int,
                &mut num10,
            );
            product = num10 * resolution;
            f = (product + (denom10 >> 1 as libc::c_int)) / denom10;
            f -= f & (product % denom10 == denom10 >> 1 as libc::c_int) as libc::c_int;
            if f < 0 as libc::c_int || product / resolution != num10 {
                return 0 as *const libc::c_char;
            }
        }
        *fres = f;
        return s;
    }
    return 0 as *const libc::c_char;
}
pub unsafe extern "C" fn parzone(
    mut s: *const libc::c_char,
    mut zone: *mut libc::c_long,
) -> *const libc::c_char {
    let mut sign: libc::c_char = 0;
    let mut hh: libc::c_int = 0;
    let mut mm: libc::c_int = 0;
    let mut ss: libc::c_int = 0;
    let mut minutesEastOfUTC: libc::c_int = 0;
    let mut offset: libc::c_long = 0;
    let mut z: libc::c_long = 0;
    let mut current_block_18: u64;
    match *s as libc::c_int {
        45 | 43 => {
            z = 0 as libc::c_int as libc::c_long;
        }
        _ => {
            minutesEastOfUTC = lookup(s, zone_names.as_ptr());
            if minutesEastOfUTC == -(1 as libc::c_int) {
                return 0 as *const libc::c_char;
            }
            while *(*__ctype_b_loc()).offset(*s as libc::c_uchar as libc::c_int as isize)
                as libc::c_int & _ISalpha as libc::c_int as libc::c_ushort as libc::c_int
                != 0
            {
                s = s.offset(1);
                s;
            }
            if minutesEastOfUTC == 1 as libc::c_int {
                *zone = -(24 as libc::c_int) as libc::c_long
                    * 60 as libc::c_int as libc::c_long
                    * 60 as libc::c_int as libc::c_long
                    - 1 as libc::c_int as libc::c_long;
                return s;
            }
            z = minutesEastOfUTC as libc::c_long * 60 as libc::c_long;
            if (*s.offset(-(1 as libc::c_int) as isize) as libc::c_int == 'T' as i32
                || *s.offset(-(1 as libc::c_int) as isize) as libc::c_int == 't' as i32)
                && (*s.offset(-(2 as libc::c_int) as isize) as libc::c_int == 'S' as i32
                    || *s.offset(-(2 as libc::c_int) as isize) as libc::c_int
                        == 's' as i32)
                && (*s.offset(-(3 as libc::c_int) as isize) as libc::c_int == 'D' as i32
                    || *s.offset(-(3 as libc::c_int) as isize) as libc::c_int
                        == 't' as i32)
            {
                current_block_18 = 12706761212494288355;
            } else {
                while *(*__ctype_b_loc())
                    .offset(*s as libc::c_uchar as libc::c_int as isize) as libc::c_int
                    & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
                {
                    s = s.offset(1);
                    s;
                }
                if (*s.offset(0 as libc::c_int as isize) as libc::c_int == 'D' as i32
                    || *s.offset(0 as libc::c_int as isize) as libc::c_int == 'd' as i32)
                    && (*s.offset(1 as libc::c_int as isize) as libc::c_int == 'S' as i32
                        || *s.offset(1 as libc::c_int as isize) as libc::c_int
                            == 's' as i32)
                    && (*s.offset(2 as libc::c_int as isize) as libc::c_int == 'T' as i32
                        || *s.offset(2 as libc::c_int as isize) as libc::c_int
                            == 't' as i32)
                {
                    s = s.offset(3 as libc::c_int as isize);
                    current_block_18 = 12706761212494288355;
                } else {
                    match *s as libc::c_int {
                        45 | 43 => {}
                        _ => {
                            *zone = z;
                            return s;
                        }
                    }
                    current_block_18 = 4495394744059808450;
                }
            }
            match current_block_18 {
                4495394744059808450 => {}
                _ => {
                    *zone = z + (60 as libc::c_int * 60 as libc::c_int) as libc::c_long;
                    return s;
                }
            }
        }
    }
    let fresh2 = s;
    s = s.offset(1);
    sign = *fresh2;
    s = parse_ranged(s, 2 as libc::c_int, 0 as libc::c_int, 23 as libc::c_int, &mut hh);
    if s.is_null() {
        return 0 as *const libc::c_char;
    }
    ss = 0 as libc::c_int;
    mm = ss;
    if *s as libc::c_int == ':' as i32 {
        s = s.offset(1);
        s;
    }
    if *(*__ctype_b_loc()).offset(*s as libc::c_uchar as libc::c_int as isize)
        as libc::c_int & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        s = parse_ranged(
            s,
            2 as libc::c_int,
            0 as libc::c_int,
            59 as libc::c_int,
            &mut mm,
        );
        if s.is_null() {
            return 0 as *const libc::c_char;
        }
        if *s as libc::c_int == ':' as i32
            && *s.offset(-(3 as libc::c_int) as isize) as libc::c_int == ':' as i32
            && *(*__ctype_b_loc())
                .offset(
                    *s.offset(1 as libc::c_int as isize) as libc::c_uchar as libc::c_int
                        as isize,
                ) as libc::c_int
                & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
        {
            s = parse_ranged(
                s.offset(1 as libc::c_int as isize),
                2 as libc::c_int,
                0 as libc::c_int,
                59 as libc::c_int,
                &mut ss,
            );
            if s.is_null() {
                return 0 as *const libc::c_char;
            }
        }
    }
    if *(*__ctype_b_loc()).offset(*s as libc::c_uchar as libc::c_int as isize)
        as libc::c_int & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        return 0 as *const libc::c_char;
    }
    offset = (hh * 60 as libc::c_int + mm) as libc::c_long * 60 as libc::c_long
        + ss as libc::c_long;
    *zone = z + (if sign as libc::c_int == '-' as i32 { -offset } else { offset });
    return s;
}
unsafe extern "C" fn parse_pattern_letter(
    mut s: *const libc::c_char,
    mut c: libc::c_int,
    mut t: *mut partime,
) -> *const libc::c_char {
    let mut current_block_61: u64;
    match c {
        36 => {
            if *(*__ctype_b_loc()).offset(*s as libc::c_uchar as libc::c_int as isize)
                as libc::c_int & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
                != 0
            {
                return 0 as *const libc::c_char;
            }
            current_block_61 = 919954187481050311;
        }
        45 | 47 | 58 => {
            let fresh3 = s;
            s = s.offset(1);
            if *fresh3 as libc::c_int != c {
                return 0 as *const libc::c_char;
            }
            current_block_61 = 919954187481050311;
        }
        52 => {
            s = parse_fixed(s, 4 as libc::c_int, &mut (*t).tm.tm_year);
            current_block_61 = 919954187481050311;
        }
        61 => {
            s = s.offset((*s as libc::c_int == '-' as i32) as libc::c_int as isize);
            current_block_61 = 919954187481050311;
        }
        65 => {
            let fresh4 = s;
            s = s.offset(1);
            match *fresh4 as libc::c_int {
                65 | 97 => {
                    if (*t).tm.tm_hour == 12 as libc::c_int {
                        (*t).tm.tm_hour = 0 as libc::c_int;
                    }
                }
                80 | 112 => {
                    if (*t).tm.tm_hour != 12 as libc::c_int {
                        (*t).tm.tm_hour += 12 as libc::c_int;
                    }
                }
                _ => return 0 as *const libc::c_char,
            }
            match *s as libc::c_int {
                77 | 109 => {
                    s = s.offset(1);
                    s;
                }
                _ => {}
            }
            if *(*__ctype_b_loc()).offset(*s as libc::c_int as isize) as libc::c_int
                & _ISalnum as libc::c_int as libc::c_ushort as libc::c_int != 0
            {
                return 0 as *const libc::c_char;
            }
            current_block_61 = 919954187481050311;
        }
        68 => {
            s = parse_ranged(
                s,
                2 as libc::c_int,
                1 as libc::c_int,
                31 as libc::c_int,
                &mut (*t).tm.tm_mday,
            );
            current_block_61 = 919954187481050311;
        }
        100 => {
            s = parse_ranged(
                s,
                3 as libc::c_int,
                1 as libc::c_int,
                366 as libc::c_int,
                &mut (*t).tm.tm_yday,
            );
            (*t).tm.tm_yday -= 1;
            (*t).tm.tm_yday;
            current_block_61 = 919954187481050311;
        }
        69 => {
            s = parse_ranged(
                s,
                (*(*__ctype_b_loc())
                    .offset(
                        *s.offset(0 as libc::c_int as isize) as libc::c_uchar
                            as libc::c_int as isize,
                    ) as libc::c_int
                    & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
                    && *(*__ctype_b_loc())
                        .offset(
                            *s.offset(1 as libc::c_int as isize) as libc::c_uchar
                                as libc::c_int as isize,
                        ) as libc::c_int
                        & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0)
                    as libc::c_int + 1 as libc::c_int,
                1 as libc::c_int,
                31 as libc::c_int,
                &mut (*t).tm.tm_mday,
            );
            current_block_61 = 919954187481050311;
        }
        104 => {
            let mut frac: libc::c_int = 0;
            s = parse_decimal(
                s,
                2 as libc::c_int,
                0 as libc::c_int,
                23 as libc::c_int,
                60 as libc::c_int * 60 as libc::c_int,
                &mut (*t).tm.tm_hour,
                &mut frac,
            );
            (*t).tm.tm_min = frac / 60 as libc::c_int;
            (*t).tm.tm_sec = frac % 60 as libc::c_int;
            current_block_61 = 919954187481050311;
        }
        109 => {
            s = parse_decimal(
                s,
                2 as libc::c_int,
                0 as libc::c_int,
                59 as libc::c_int,
                60 as libc::c_int,
                &mut (*t).tm.tm_min,
                &mut (*t).tm.tm_sec,
            );
            current_block_61 = 919954187481050311;
        }
        110 => {
            (*t).tm.tm_mon = lookup(s, month_names.as_ptr());
            if !(0 as libc::c_int <= (*t).tm.tm_mon) {
                return 0 as *const libc::c_char;
            }
            while *(*__ctype_b_loc()).offset(*s as libc::c_uchar as libc::c_int as isize)
                as libc::c_int & _ISalpha as libc::c_int as libc::c_ushort as libc::c_int
                != 0
            {
                s = s.offset(1);
                s;
            }
            current_block_61 = 919954187481050311;
        }
        78 => {
            s = parse_ranged(
                s,
                2 as libc::c_int,
                1 as libc::c_int,
                12 as libc::c_int,
                &mut (*t).tm.tm_mon,
            );
            (*t).tm.tm_mon -= 1;
            (*t).tm.tm_mon;
            current_block_61 = 919954187481050311;
        }
        114 => {
            s = parse_fixed(s, 1 as libc::c_int, &mut (*t).tm.tm_year);
            (*t).ymodulus = 10 as libc::c_int;
            current_block_61 = 919954187481050311;
        }
        82 => {
            current_block_61 = 9229290191643294329;
        }
        115 => {
            let mut frac_0: libc::c_int = 0;
            s = parse_decimal(
                s,
                2 as libc::c_int,
                0 as libc::c_int,
                60 as libc::c_int,
                1 as libc::c_int,
                &mut (*t).tm.tm_sec,
                &mut frac_0,
            );
            (*t).tm.tm_sec += frac_0;
            current_block_61 = 919954187481050311;
        }
        84 => {
            let fresh5 = s;
            s = s.offset(1);
            match *fresh5 as libc::c_int {
                84 | 116 => {}
                _ => return 0 as *const libc::c_char,
            }
            current_block_61 = 919954187481050311;
        }
        116 => {
            s = parse_ranged(
                s,
                (*(*__ctype_b_loc())
                    .offset(
                        *s.offset(0 as libc::c_int as isize) as libc::c_uchar
                            as libc::c_int as isize,
                    ) as libc::c_int
                    & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
                    && *(*__ctype_b_loc())
                        .offset(
                            *s.offset(1 as libc::c_int as isize) as libc::c_uchar
                                as libc::c_int as isize,
                        ) as libc::c_int
                        & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0)
                    as libc::c_int + 1 as libc::c_int,
                1 as libc::c_int,
                12 as libc::c_int,
                &mut (*t).tm.tm_hour,
            );
            current_block_61 = 919954187481050311;
        }
        119 => {
            let fresh6 = s;
            s = s.offset(1);
            match *fresh6 as libc::c_int {
                87 | 119 => {}
                _ => return 0 as *const libc::c_char,
            }
            current_block_61 = 919954187481050311;
        }
        87 => {
            let fresh7 = s;
            s = s.offset(1);
            match *fresh7 as libc::c_int {
                87 | 119 => {}
                _ => return 0 as *const libc::c_char,
            }
            s = parse_ranged(
                s,
                2 as libc::c_int,
                0 as libc::c_int,
                53 as libc::c_int,
                &mut (*t).yweek,
            );
            current_block_61 = 919954187481050311;
        }
        88 => {
            s = parse_ranged(
                s,
                1 as libc::c_int,
                1 as libc::c_int,
                7 as libc::c_int,
                &mut (*t).tm.tm_wday,
            );
            (*t).tm.tm_wday %= 7 as libc::c_int;
            current_block_61 = 919954187481050311;
        }
        120 => {
            (*t).tm.tm_wday = lookup(s, weekday_names.as_ptr());
            if !(0 as libc::c_int <= (*t).tm.tm_wday) {
                return 0 as *const libc::c_char;
            }
            while *(*__ctype_b_loc()).offset(*s as libc::c_uchar as libc::c_int as isize)
                as libc::c_int & _ISalpha as libc::c_int as libc::c_ushort as libc::c_int
                != 0
            {
                s = s.offset(1);
                s;
            }
            current_block_61 = 919954187481050311;
        }
        121 => {
            if *(*__ctype_b_loc())
                .offset(
                    *s.offset(0 as libc::c_int as isize) as libc::c_uchar as libc::c_int
                        as isize,
                ) as libc::c_int
                & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
                && *(*__ctype_b_loc())
                    .offset(
                        *s.offset(1 as libc::c_int as isize) as libc::c_uchar
                            as libc::c_int as isize,
                    ) as libc::c_int
                    & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
                && *(*__ctype_b_loc())
                    .offset(
                        *s.offset(2 as libc::c_int as isize) as libc::c_uchar
                            as libc::c_int as isize,
                    ) as libc::c_int
                    & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int == 0
            {
                current_block_61 = 9229290191643294329;
            } else {
                current_block_61 = 10728555529599907815;
            }
        }
        89 => {
            current_block_61 = 10728555529599907815;
        }
        90 => {
            s = parzone(s, &mut (*t).zone);
            current_block_61 = 919954187481050311;
        }
        95 => {
            while *(*__ctype_b_loc()).offset(*s as libc::c_int as isize) as libc::c_int
                & _ISalnum as libc::c_int as libc::c_ushort as libc::c_int == 0
                && *s as libc::c_int != 0
            {
                s = s.offset(1);
                s;
            }
            current_block_61 = 919954187481050311;
        }
        _ => return 0 as *const libc::c_char,
    }
    match current_block_61 {
        9229290191643294329 => {
            s = parse_fixed(s, 2 as libc::c_int, &mut (*t).tm.tm_year);
            (*t).ymodulus = 100 as libc::c_int;
        }
        10728555529599907815 => {
            let mut len: libc::c_int = 0 as libc::c_int;
            while *(*__ctype_b_loc())
                .offset(*s.offset(len as isize) as libc::c_uchar as libc::c_int as isize)
                as libc::c_int & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
                != 0
            {
                len += 1;
                len;
            }
            if len < 4 as libc::c_int {
                return 0 as *const libc::c_char;
            }
            s = parse_fixed(s, len, &mut (*t).tm.tm_year);
        }
        _ => {}
    }
    return s;
}
unsafe extern "C" fn parse_prefix(
    mut str: *const libc::c_char,
    mut t: *mut partime,
    mut pi: *mut libc::c_int,
) -> *const libc::c_char {
    let mut i: libc::c_int = *pi;
    let mut pat: *const libc::c_char = 0 as *const libc::c_char;
    let mut c: libc::c_uchar = 0;
    if i < 0 as libc::c_int {
        return 0 as *const libc::c_char;
    }
    loop {
        c = *str as libc::c_uchar;
        if !(*(*__ctype_b_loc()).offset(c as libc::c_int as isize) as libc::c_int
            & _ISalnum as libc::c_int as libc::c_ushort as libc::c_int == 0
            && c as libc::c_int != '-' as i32 && c as libc::c_int != '+' as i32)
        {
            break;
        }
        if c == 0 {
            undefine(t);
            *pi = -(1 as libc::c_int);
            return str;
        }
        str = str.offset(1);
        str;
    }
    loop {
        let fresh8 = i;
        i = i + 1;
        pat = patterns[fresh8 as usize];
        if pat.is_null() {
            break;
        }
        let mut s: *const libc::c_char = str;
        undefine(t);
        loop {
            let fresh9 = pat;
            pat = pat.offset(1);
            c = *fresh9 as libc::c_uchar;
            if c == 0 {
                *pi = i;
                return s;
            }
            s = parse_pattern_letter(s, c as libc::c_int, t);
            if s.is_null() {
                break;
            }
        }
    }
    return 0 as *const libc::c_char;
}
unsafe extern "C" fn merge_partime(
    mut t: *mut partime,
    mut u: *const partime,
) -> libc::c_int {
    if (*t).tm.tm_sec != (*u).tm.tm_sec && 0 as libc::c_int <= (*t).tm.tm_sec
        && 0 as libc::c_int <= (*u).tm.tm_sec
        || (*t).tm.tm_min != (*u).tm.tm_min && 0 as libc::c_int <= (*t).tm.tm_min
            && 0 as libc::c_int <= (*u).tm.tm_min
        || (*t).tm.tm_hour != (*u).tm.tm_hour && 0 as libc::c_int <= (*t).tm.tm_hour
            && 0 as libc::c_int <= (*u).tm.tm_hour
        || (*t).tm.tm_mday != (*u).tm.tm_mday && 0 as libc::c_int <= (*t).tm.tm_mday
            && 0 as libc::c_int <= (*u).tm.tm_mday
        || (*t).tm.tm_mon != (*u).tm.tm_mon && 0 as libc::c_int <= (*t).tm.tm_mon
            && 0 as libc::c_int <= (*u).tm.tm_mon
        || (*t).tm.tm_year != (*u).tm.tm_year && 0 as libc::c_int <= (*t).tm.tm_year
            && 0 as libc::c_int <= (*u).tm.tm_year
        || (*t).tm.tm_wday != (*u).tm.tm_wday && 0 as libc::c_int <= (*t).tm.tm_wday
            && 0 as libc::c_int <= (*u).tm.tm_wday
        || (*t).tm.tm_yday != (*u).tm.tm_yday && 0 as libc::c_int <= (*t).tm.tm_yday
            && 0 as libc::c_int <= (*u).tm.tm_yday
        || (*t).ymodulus != (*u).ymodulus && 0 as libc::c_int <= (*t).ymodulus
            && 0 as libc::c_int <= (*u).ymodulus
        || (*t).yweek != (*u).yweek && 0 as libc::c_int <= (*t).yweek
            && 0 as libc::c_int <= (*u).yweek
        || (*t).zone != (*u).zone
            && (*t).zone
                != -(24 as libc::c_int) as libc::c_long
                    * 60 as libc::c_int as libc::c_long
                    * 60 as libc::c_int as libc::c_long
            && (*u).zone
                != -(24 as libc::c_int) as libc::c_long
                    * 60 as libc::c_int as libc::c_long
                    * 60 as libc::c_int as libc::c_long
    {
        return -(1 as libc::c_int);
    }
    if 0 as libc::c_int <= (*u).tm.tm_sec {
        (*t).tm.tm_sec = (*u).tm.tm_sec;
    }
    if 0 as libc::c_int <= (*u).tm.tm_min {
        (*t).tm.tm_min = (*u).tm.tm_min;
    }
    if 0 as libc::c_int <= (*u).tm.tm_hour {
        (*t).tm.tm_hour = (*u).tm.tm_hour;
    }
    if 0 as libc::c_int <= (*u).tm.tm_mday {
        (*t).tm.tm_mday = (*u).tm.tm_mday;
    }
    if 0 as libc::c_int <= (*u).tm.tm_mon {
        (*t).tm.tm_mon = (*u).tm.tm_mon;
    }
    if 0 as libc::c_int <= (*u).tm.tm_year {
        (*t).tm.tm_year = (*u).tm.tm_year;
    }
    if 0 as libc::c_int <= (*u).tm.tm_wday {
        (*t).tm.tm_wday = (*u).tm.tm_wday;
    }
    if 0 as libc::c_int <= (*u).tm.tm_yday {
        (*t).tm.tm_yday = (*u).tm.tm_yday;
    }
    if 0 as libc::c_int <= (*u).ymodulus {
        (*t).ymodulus = (*u).ymodulus;
    }
    if 0 as libc::c_int <= (*u).yweek {
        (*t).yweek = (*u).yweek;
    }
    if (*u).zone
        != -(24 as libc::c_int) as libc::c_long * 60 as libc::c_int as libc::c_long
            * 60 as libc::c_int as libc::c_long
    {
        (*t).zone = (*u).zone;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn partime(
    mut s: *const libc::c_char,
    mut t: *mut partime,
) -> *const libc::c_char {
    let mut p: partime = partime {
        tm: tm {
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
        },
        ymodulus: 0,
        yweek: 0,
        zone: 0,
    };
    undefine(t);
    while *s != 0 {
        let mut i: libc::c_int = 0 as libc::c_int;
        let mut s1: *const libc::c_char = 0 as *const libc::c_char;
        loop {
            s1 = parse_prefix(s, &mut p, &mut i);
            if s1.is_null() {
                return s;
            }
            if !(0 as libc::c_int > merge_partime(t, &mut p)) {
                break;
            }
        }
        s = s1;
    }
    return s;
}
unsafe extern "C" fn run_static_initializers() {
    zone_names = [
        {
            let mut init = name_val {
                name: *::std::mem::transmute::<
                    &[u8; 4],
                    &mut [libc::c_char; 4],
                >(b"hst\0"),
                val: if -(1000 as libc::c_int) < 0 as libc::c_int {
                    -(--(1000 as libc::c_int) / 100 as libc::c_int * 60 as libc::c_int
                        + --(1000 as libc::c_int) % 100 as libc::c_int)
                } else {
                    -(1000 as libc::c_int) / 100 as libc::c_int * 60 as libc::c_int
                        + -(1000 as libc::c_int) % 100 as libc::c_int
                },
            };
            init
        },
        {
            let mut init = name_val {
                name: *::std::mem::transmute::<
                    &[u8; 4],
                    &mut [libc::c_char; 4],
                >(b"hast"),
                val: if -(1000 as libc::c_int) < 0 as libc::c_int {
                    -(--(1000 as libc::c_int) / 100 as libc::c_int * 60 as libc::c_int
                        + --(1000 as libc::c_int) % 100 as libc::c_int)
                } else {
                    -(1000 as libc::c_int) / 100 as libc::c_int * 60 as libc::c_int
                        + -(1000 as libc::c_int) % 100 as libc::c_int
                },
            };
            init
        },
        {
            let mut init = name_val {
                name: *::std::mem::transmute::<
                    &[u8; 4],
                    &mut [libc::c_char; 4],
                >(b"hadt"),
                val: if (-(1000 as libc::c_int) + 100 as libc::c_int) < 0 as libc::c_int
                {
                    -(-(-(1000 as libc::c_int) + 100 as libc::c_int) / 100 as libc::c_int
                        * 60 as libc::c_int
                        + -(-(1000 as libc::c_int) + 100 as libc::c_int)
                            % 100 as libc::c_int)
                } else {
                    (-(1000 as libc::c_int) + 100 as libc::c_int) / 100 as libc::c_int
                        * 60 as libc::c_int
                        + (-(1000 as libc::c_int) + 100 as libc::c_int)
                            % 100 as libc::c_int
                },
            };
            init
        },
        {
            let mut init = name_val {
                name: *::std::mem::transmute::<
                    &[u8; 4],
                    &mut [libc::c_char; 4],
                >(b"akst"),
                val: if -(900 as libc::c_int) < 0 as libc::c_int {
                    -(--(900 as libc::c_int) / 100 as libc::c_int * 60 as libc::c_int
                        + --(900 as libc::c_int) % 100 as libc::c_int)
                } else {
                    -(900 as libc::c_int) / 100 as libc::c_int * 60 as libc::c_int
                        + -(900 as libc::c_int) % 100 as libc::c_int
                },
            };
            init
        },
        {
            let mut init = name_val {
                name: *::std::mem::transmute::<
                    &[u8; 4],
                    &mut [libc::c_char; 4],
                >(b"akdt"),
                val: if (-(900 as libc::c_int) + 100 as libc::c_int) < 0 as libc::c_int {
                    -(-(-(900 as libc::c_int) + 100 as libc::c_int) / 100 as libc::c_int
                        * 60 as libc::c_int
                        + -(-(900 as libc::c_int) + 100 as libc::c_int)
                            % 100 as libc::c_int)
                } else {
                    (-(900 as libc::c_int) + 100 as libc::c_int) / 100 as libc::c_int
                        * 60 as libc::c_int
                        + (-(900 as libc::c_int) + 100 as libc::c_int)
                            % 100 as libc::c_int
                },
            };
            init
        },
        {
            let mut init = name_val {
                name: *::std::mem::transmute::<
                    &[u8; 4],
                    &mut [libc::c_char; 4],
                >(b"pst\0"),
                val: if -(800 as libc::c_int) < 0 as libc::c_int {
                    -(--(800 as libc::c_int) / 100 as libc::c_int * 60 as libc::c_int
                        + --(800 as libc::c_int) % 100 as libc::c_int)
                } else {
                    -(800 as libc::c_int) / 100 as libc::c_int * 60 as libc::c_int
                        + -(800 as libc::c_int) % 100 as libc::c_int
                },
            };
            init
        },
        {
            let mut init = name_val {
                name: *::std::mem::transmute::<
                    &[u8; 4],
                    &mut [libc::c_char; 4],
                >(b"pdt\0"),
                val: if (-(800 as libc::c_int) + 100 as libc::c_int) < 0 as libc::c_int {
                    -(-(-(800 as libc::c_int) + 100 as libc::c_int) / 100 as libc::c_int
                        * 60 as libc::c_int
                        + -(-(800 as libc::c_int) + 100 as libc::c_int)
                            % 100 as libc::c_int)
                } else {
                    (-(800 as libc::c_int) + 100 as libc::c_int) / 100 as libc::c_int
                        * 60 as libc::c_int
                        + (-(800 as libc::c_int) + 100 as libc::c_int)
                            % 100 as libc::c_int
                },
            };
            init
        },
        {
            let mut init = name_val {
                name: *::std::mem::transmute::<
                    &[u8; 4],
                    &mut [libc::c_char; 4],
                >(b"mst\0"),
                val: if -(700 as libc::c_int) < 0 as libc::c_int {
                    -(--(700 as libc::c_int) / 100 as libc::c_int * 60 as libc::c_int
                        + --(700 as libc::c_int) % 100 as libc::c_int)
                } else {
                    -(700 as libc::c_int) / 100 as libc::c_int * 60 as libc::c_int
                        + -(700 as libc::c_int) % 100 as libc::c_int
                },
            };
            init
        },
        {
            let mut init = name_val {
                name: *::std::mem::transmute::<
                    &[u8; 4],
                    &mut [libc::c_char; 4],
                >(b"mdt\0"),
                val: if (-(700 as libc::c_int) + 100 as libc::c_int) < 0 as libc::c_int {
                    -(-(-(700 as libc::c_int) + 100 as libc::c_int) / 100 as libc::c_int
                        * 60 as libc::c_int
                        + -(-(700 as libc::c_int) + 100 as libc::c_int)
                            % 100 as libc::c_int)
                } else {
                    (-(700 as libc::c_int) + 100 as libc::c_int) / 100 as libc::c_int
                        * 60 as libc::c_int
                        + (-(700 as libc::c_int) + 100 as libc::c_int)
                            % 100 as libc::c_int
                },
            };
            init
        },
        {
            let mut init = name_val {
                name: *::std::mem::transmute::<
                    &[u8; 4],
                    &mut [libc::c_char; 4],
                >(b"cst\0"),
                val: if -(600 as libc::c_int) < 0 as libc::c_int {
                    -(--(600 as libc::c_int) / 100 as libc::c_int * 60 as libc::c_int
                        + --(600 as libc::c_int) % 100 as libc::c_int)
                } else {
                    -(600 as libc::c_int) / 100 as libc::c_int * 60 as libc::c_int
                        + -(600 as libc::c_int) % 100 as libc::c_int
                },
            };
            init
        },
        {
            let mut init = name_val {
                name: *::std::mem::transmute::<
                    &[u8; 4],
                    &mut [libc::c_char; 4],
                >(b"cdt\0"),
                val: if (-(600 as libc::c_int) + 100 as libc::c_int) < 0 as libc::c_int {
                    -(-(-(600 as libc::c_int) + 100 as libc::c_int) / 100 as libc::c_int
                        * 60 as libc::c_int
                        + -(-(600 as libc::c_int) + 100 as libc::c_int)
                            % 100 as libc::c_int)
                } else {
                    (-(600 as libc::c_int) + 100 as libc::c_int) / 100 as libc::c_int
                        * 60 as libc::c_int
                        + (-(600 as libc::c_int) + 100 as libc::c_int)
                            % 100 as libc::c_int
                },
            };
            init
        },
        {
            let mut init = name_val {
                name: *::std::mem::transmute::<
                    &[u8; 4],
                    &mut [libc::c_char; 4],
                >(b"est\0"),
                val: if -(500 as libc::c_int) < 0 as libc::c_int {
                    -(--(500 as libc::c_int) / 100 as libc::c_int * 60 as libc::c_int
                        + --(500 as libc::c_int) % 100 as libc::c_int)
                } else {
                    -(500 as libc::c_int) / 100 as libc::c_int * 60 as libc::c_int
                        + -(500 as libc::c_int) % 100 as libc::c_int
                },
            };
            init
        },
        {
            let mut init = name_val {
                name: *::std::mem::transmute::<
                    &[u8; 4],
                    &mut [libc::c_char; 4],
                >(b"edt\0"),
                val: if (-(500 as libc::c_int) + 100 as libc::c_int) < 0 as libc::c_int {
                    -(-(-(500 as libc::c_int) + 100 as libc::c_int) / 100 as libc::c_int
                        * 60 as libc::c_int
                        + -(-(500 as libc::c_int) + 100 as libc::c_int)
                            % 100 as libc::c_int)
                } else {
                    (-(500 as libc::c_int) + 100 as libc::c_int) / 100 as libc::c_int
                        * 60 as libc::c_int
                        + (-(500 as libc::c_int) + 100 as libc::c_int)
                            % 100 as libc::c_int
                },
            };
            init
        },
        {
            let mut init = name_val {
                name: *::std::mem::transmute::<
                    &[u8; 4],
                    &mut [libc::c_char; 4],
                >(b"ast\0"),
                val: if -(400 as libc::c_int) < 0 as libc::c_int {
                    -(--(400 as libc::c_int) / 100 as libc::c_int * 60 as libc::c_int
                        + --(400 as libc::c_int) % 100 as libc::c_int)
                } else {
                    -(400 as libc::c_int) / 100 as libc::c_int * 60 as libc::c_int
                        + -(400 as libc::c_int) % 100 as libc::c_int
                },
            };
            init
        },
        {
            let mut init = name_val {
                name: *::std::mem::transmute::<
                    &[u8; 4],
                    &mut [libc::c_char; 4],
                >(b"adt\0"),
                val: if (-(400 as libc::c_int) + 100 as libc::c_int) < 0 as libc::c_int {
                    -(-(-(400 as libc::c_int) + 100 as libc::c_int) / 100 as libc::c_int
                        * 60 as libc::c_int
                        + -(-(400 as libc::c_int) + 100 as libc::c_int)
                            % 100 as libc::c_int)
                } else {
                    (-(400 as libc::c_int) + 100 as libc::c_int) / 100 as libc::c_int
                        * 60 as libc::c_int
                        + (-(400 as libc::c_int) + 100 as libc::c_int)
                            % 100 as libc::c_int
                },
            };
            init
        },
        {
            let mut init = name_val {
                name: *::std::mem::transmute::<
                    &[u8; 4],
                    &mut [libc::c_char; 4],
                >(b"nst\0"),
                val: if -(330 as libc::c_int) < 0 as libc::c_int {
                    -(--(330 as libc::c_int) / 100 as libc::c_int * 60 as libc::c_int
                        + --(330 as libc::c_int) % 100 as libc::c_int)
                } else {
                    -(330 as libc::c_int) / 100 as libc::c_int * 60 as libc::c_int
                        + -(330 as libc::c_int) % 100 as libc::c_int
                },
            };
            init
        },
        {
            let mut init = name_val {
                name: *::std::mem::transmute::<
                    &[u8; 4],
                    &mut [libc::c_char; 4],
                >(b"ndt\0"),
                val: if (-(330 as libc::c_int) + 100 as libc::c_int) < 0 as libc::c_int {
                    -(-(-(330 as libc::c_int) + 100 as libc::c_int) / 100 as libc::c_int
                        * 60 as libc::c_int
                        + -(-(330 as libc::c_int) + 100 as libc::c_int)
                            % 100 as libc::c_int)
                } else {
                    (-(330 as libc::c_int) + 100 as libc::c_int) / 100 as libc::c_int
                        * 60 as libc::c_int
                        + (-(330 as libc::c_int) + 100 as libc::c_int)
                            % 100 as libc::c_int
                },
            };
            init
        },
        {
            let mut init = name_val {
                name: *::std::mem::transmute::<
                    &[u8; 4],
                    &mut [libc::c_char; 4],
                >(b"utc\0"),
                val: if (0 as libc::c_int) < 0 as libc::c_int {
                    -(-(0 as libc::c_int) / 100 as libc::c_int * 60 as libc::c_int
                        + -(0 as libc::c_int) % 100 as libc::c_int)
                } else {
                    0 as libc::c_int / 100 as libc::c_int * 60 as libc::c_int
                        + 0 as libc::c_int % 100 as libc::c_int
                },
            };
            init
        },
        {
            let mut init = name_val {
                name: *::std::mem::transmute::<
                    &[u8; 4],
                    &mut [libc::c_char; 4],
                >(b"cut\0"),
                val: if (0 as libc::c_int) < 0 as libc::c_int {
                    -(-(0 as libc::c_int) / 100 as libc::c_int * 60 as libc::c_int
                        + -(0 as libc::c_int) % 100 as libc::c_int)
                } else {
                    0 as libc::c_int / 100 as libc::c_int * 60 as libc::c_int
                        + 0 as libc::c_int % 100 as libc::c_int
                },
            };
            init
        },
        {
            let mut init = name_val {
                name: *::std::mem::transmute::<
                    &[u8; 4],
                    &mut [libc::c_char; 4],
                >(b"ut\0\0"),
                val: if (0 as libc::c_int) < 0 as libc::c_int {
                    -(-(0 as libc::c_int) / 100 as libc::c_int * 60 as libc::c_int
                        + -(0 as libc::c_int) % 100 as libc::c_int)
                } else {
                    0 as libc::c_int / 100 as libc::c_int * 60 as libc::c_int
                        + 0 as libc::c_int % 100 as libc::c_int
                },
            };
            init
        },
        {
            let mut init = name_val {
                name: *::std::mem::transmute::<
                    &[u8; 4],
                    &mut [libc::c_char; 4],
                >(b"z\0\0\0"),
                val: if (0 as libc::c_int) < 0 as libc::c_int {
                    -(-(0 as libc::c_int) / 100 as libc::c_int * 60 as libc::c_int
                        + -(0 as libc::c_int) % 100 as libc::c_int)
                } else {
                    0 as libc::c_int / 100 as libc::c_int * 60 as libc::c_int
                        + 0 as libc::c_int % 100 as libc::c_int
                },
            };
            init
        },
        {
            let mut init = name_val {
                name: *::std::mem::transmute::<
                    &[u8; 4],
                    &mut [libc::c_char; 4],
                >(b"gmt\0"),
                val: if (0 as libc::c_int) < 0 as libc::c_int {
                    -(-(0 as libc::c_int) / 100 as libc::c_int * 60 as libc::c_int
                        + -(0 as libc::c_int) % 100 as libc::c_int)
                } else {
                    0 as libc::c_int / 100 as libc::c_int * 60 as libc::c_int
                        + 0 as libc::c_int % 100 as libc::c_int
                },
            };
            init
        },
        {
            let mut init = name_val {
                name: *::std::mem::transmute::<
                    &[u8; 4],
                    &mut [libc::c_char; 4],
                >(b"bst\0"),
                val: if (0 as libc::c_int + 100 as libc::c_int) < 0 as libc::c_int {
                    -(-(0 as libc::c_int + 100 as libc::c_int) / 100 as libc::c_int
                        * 60 as libc::c_int
                        + -(0 as libc::c_int + 100 as libc::c_int) % 100 as libc::c_int)
                } else {
                    (0 as libc::c_int + 100 as libc::c_int) / 100 as libc::c_int
                        * 60 as libc::c_int
                        + (0 as libc::c_int + 100 as libc::c_int) % 100 as libc::c_int
                },
            };
            init
        },
        {
            let mut init = name_val {
                name: *::std::mem::transmute::<
                    &[u8; 4],
                    &mut [libc::c_char; 4],
                >(b"wet\0"),
                val: if (0 as libc::c_int) < 0 as libc::c_int {
                    -(-(0 as libc::c_int) / 100 as libc::c_int * 60 as libc::c_int
                        + -(0 as libc::c_int) % 100 as libc::c_int)
                } else {
                    0 as libc::c_int / 100 as libc::c_int * 60 as libc::c_int
                        + 0 as libc::c_int % 100 as libc::c_int
                },
            };
            init
        },
        {
            let mut init = name_val {
                name: *::std::mem::transmute::<
                    &[u8; 4],
                    &mut [libc::c_char; 4],
                >(b"met\0"),
                val: if (100 as libc::c_int) < 0 as libc::c_int {
                    -(-(100 as libc::c_int) / 100 as libc::c_int * 60 as libc::c_int
                        + -(100 as libc::c_int) % 100 as libc::c_int)
                } else {
                    100 as libc::c_int / 100 as libc::c_int * 60 as libc::c_int
                        + 100 as libc::c_int % 100 as libc::c_int
                },
            };
            init
        },
        {
            let mut init = name_val {
                name: *::std::mem::transmute::<
                    &[u8; 4],
                    &mut [libc::c_char; 4],
                >(b"cet\0"),
                val: if (100 as libc::c_int) < 0 as libc::c_int {
                    -(-(100 as libc::c_int) / 100 as libc::c_int * 60 as libc::c_int
                        + -(100 as libc::c_int) % 100 as libc::c_int)
                } else {
                    100 as libc::c_int / 100 as libc::c_int * 60 as libc::c_int
                        + 100 as libc::c_int % 100 as libc::c_int
                },
            };
            init
        },
        {
            let mut init = name_val {
                name: *::std::mem::transmute::<
                    &[u8; 4],
                    &mut [libc::c_char; 4],
                >(b"eet\0"),
                val: if (200 as libc::c_int) < 0 as libc::c_int {
                    -(-(200 as libc::c_int) / 100 as libc::c_int * 60 as libc::c_int
                        + -(200 as libc::c_int) % 100 as libc::c_int)
                } else {
                    200 as libc::c_int / 100 as libc::c_int * 60 as libc::c_int
                        + 200 as libc::c_int % 100 as libc::c_int
                },
            };
            init
        },
        {
            let mut init = name_val {
                name: *::std::mem::transmute::<
                    &[u8; 4],
                    &mut [libc::c_char; 4],
                >(b"ist\0"),
                val: if (530 as libc::c_int) < 0 as libc::c_int {
                    -(-(530 as libc::c_int) / 100 as libc::c_int * 60 as libc::c_int
                        + -(530 as libc::c_int) % 100 as libc::c_int)
                } else {
                    530 as libc::c_int / 100 as libc::c_int * 60 as libc::c_int
                        + 530 as libc::c_int % 100 as libc::c_int
                },
            };
            init
        },
        {
            let mut init = name_val {
                name: *::std::mem::transmute::<
                    &[u8; 4],
                    &mut [libc::c_char; 4],
                >(b"jst\0"),
                val: if (900 as libc::c_int) < 0 as libc::c_int {
                    -(-(900 as libc::c_int) / 100 as libc::c_int * 60 as libc::c_int
                        + -(900 as libc::c_int) % 100 as libc::c_int)
                } else {
                    900 as libc::c_int / 100 as libc::c_int * 60 as libc::c_int
                        + 900 as libc::c_int % 100 as libc::c_int
                },
            };
            init
        },
        {
            let mut init = name_val {
                name: *::std::mem::transmute::<
                    &[u8; 4],
                    &mut [libc::c_char; 4],
                >(b"jdt\0"),
                val: if (900 as libc::c_int + 100 as libc::c_int) < 0 as libc::c_int {
                    -(-(900 as libc::c_int + 100 as libc::c_int) / 100 as libc::c_int
                        * 60 as libc::c_int
                        + -(900 as libc::c_int + 100 as libc::c_int)
                            % 100 as libc::c_int)
                } else {
                    (900 as libc::c_int + 100 as libc::c_int) / 100 as libc::c_int
                        * 60 as libc::c_int
                        + (900 as libc::c_int + 100 as libc::c_int) % 100 as libc::c_int
                },
            };
            init
        },
        {
            let mut init = name_val {
                name: *::std::mem::transmute::<
                    &[u8; 4],
                    &mut [libc::c_char; 4],
                >(b"kst\0"),
                val: if (900 as libc::c_int) < 0 as libc::c_int {
                    -(-(900 as libc::c_int) / 100 as libc::c_int * 60 as libc::c_int
                        + -(900 as libc::c_int) % 100 as libc::c_int)
                } else {
                    900 as libc::c_int / 100 as libc::c_int * 60 as libc::c_int
                        + 900 as libc::c_int % 100 as libc::c_int
                },
            };
            init
        },
        {
            let mut init = name_val {
                name: *::std::mem::transmute::<
                    &[u8; 4],
                    &mut [libc::c_char; 4],
                >(b"kdt\0"),
                val: if (900 as libc::c_int + 100 as libc::c_int) < 0 as libc::c_int {
                    -(-(900 as libc::c_int + 100 as libc::c_int) / 100 as libc::c_int
                        * 60 as libc::c_int
                        + -(900 as libc::c_int + 100 as libc::c_int)
                            % 100 as libc::c_int)
                } else {
                    (900 as libc::c_int + 100 as libc::c_int) / 100 as libc::c_int
                        * 60 as libc::c_int
                        + (900 as libc::c_int + 100 as libc::c_int) % 100 as libc::c_int
                },
            };
            init
        },
        {
            let mut init = name_val {
                name: *::std::mem::transmute::<
                    &[u8; 4],
                    &mut [libc::c_char; 4],
                >(b"nzst"),
                val: if (1200 as libc::c_int) < 0 as libc::c_int {
                    -(-(1200 as libc::c_int) / 100 as libc::c_int * 60 as libc::c_int
                        + -(1200 as libc::c_int) % 100 as libc::c_int)
                } else {
                    1200 as libc::c_int / 100 as libc::c_int * 60 as libc::c_int
                        + 1200 as libc::c_int % 100 as libc::c_int
                },
            };
            init
        },
        {
            let mut init = name_val {
                name: *::std::mem::transmute::<
                    &[u8; 4],
                    &mut [libc::c_char; 4],
                >(b"nzdt"),
                val: if (1200 as libc::c_int + 100 as libc::c_int) < 0 as libc::c_int {
                    -(-(1200 as libc::c_int + 100 as libc::c_int) / 100 as libc::c_int
                        * 60 as libc::c_int
                        + -(1200 as libc::c_int + 100 as libc::c_int)
                            % 100 as libc::c_int)
                } else {
                    (1200 as libc::c_int + 100 as libc::c_int) / 100 as libc::c_int
                        * 60 as libc::c_int
                        + (1200 as libc::c_int + 100 as libc::c_int) % 100 as libc::c_int
                },
            };
            init
        },
        {
            let mut init = name_val {
                name: *::std::mem::transmute::<
                    &[u8; 4],
                    &mut [libc::c_char; 4],
                >(b"lt\0\0"),
                val: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = name_val {
                name: *::std::mem::transmute::<
                    &[u8; 4],
                    &mut [libc::c_char; 4],
                >(b"\0\0\0\0"),
                val: -(1 as libc::c_int),
            };
            init
        },
    ];
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
