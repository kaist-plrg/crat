use ::libc;
use ::f128;
extern "C" {
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn strncasecmp(
        __s1: *const libc::c_char,
        __s2: *const libc::c_char,
        __n: size_t,
    ) -> libc::c_int;
    fn aprintf(
        _: *mut *mut libc::c_char,
        _: *const libc::c_char,
        _: ...
    ) -> *mut libc::c_char;
}
pub type size_t = libc::c_ulong;
pub type __uint64_t = libc::c_ulong;
pub type uint64_t = __uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct units {
    pub scale: libc::c_int,
    pub base: *mut libc::c_char,
    pub units: [*mut libc::c_char; 6],
}
pub static mut time_units_us: units = {
    let mut init = units {
        scale: 1000 as libc::c_int,
        base: b"us\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        units: [
            b"ms\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
        ],
    };
    init
};
pub static mut time_units_s: units = {
    let mut init = units {
        scale: 60 as libc::c_int,
        base: b"s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        units: [
            b"m\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"h\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
        ],
    };
    init
};
pub static mut binary_units: units = {
    let mut init = units {
        scale: 1024 as libc::c_int,
        base: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        units: [
            b"K\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"M\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"G\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"T\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"P\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
        ],
    };
    init
};
pub static mut metric_units: units = {
    let mut init = units {
        scale: 1000 as libc::c_int,
        base: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        units: [
            b"k\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"M\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"G\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"T\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"P\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
        ],
    };
    init
};
unsafe extern "C" fn format_units(
    mut n: f128::f128,
    mut m: *mut units,
    mut p: libc::c_int,
) -> *mut libc::c_char {
    let mut amt: f128::f128 = n;
    let mut scale: f128::f128 = f128::f128::ZERO;
    let mut unit: *mut libc::c_char = (*m).base;
    let mut msg: *mut libc::c_char = 0 as *mut libc::c_char;
    scale = f128::f128::new((*m).scale as libc::c_double * 0.85f64);
    let mut i: libc::c_int = 0 as libc::c_int;
    while !((*m).units[(i + 1 as libc::c_int) as usize]).is_null() && amt >= scale {
        amt /= f128::f128::new((*m).scale);
        unit = (*m).units[i as usize];
        i += 1;
        i;
    }
    aprintf(
        &mut msg as *mut *mut libc::c_char,
        b"%.*Lf%s\0" as *const u8 as *const libc::c_char,
        p,
        amt,
        unit,
    );
    return msg;
}
unsafe extern "C" fn scan_units(
    mut s: *mut libc::c_char,
    mut n: *mut uint64_t,
    mut m: *mut units,
) -> libc::c_int {
    let mut base: uint64_t = 0;
    let mut scale: uint64_t = 1 as libc::c_int as uint64_t;
    let mut unit: [libc::c_char; 3] = [
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
    ];
    let mut i: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    c = sscanf(
        s,
        b"%lu%2s\0" as *const u8 as *const libc::c_char,
        &mut base as *mut uint64_t,
        unit.as_mut_ptr(),
    );
    if c < 1 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if c == 2 as libc::c_int
        && strncasecmp(unit.as_mut_ptr(), (*m).base, 3 as libc::c_int as size_t) != 0
    {
        i = 0 as libc::c_int;
        while !((*m).units[i as usize]).is_null() {
            scale = (scale as libc::c_ulong).wrapping_mul((*m).scale as libc::c_ulong)
                as uint64_t as uint64_t;
            if strncasecmp(
                unit.as_mut_ptr(),
                (*m).units[i as usize],
                3 as libc::c_int as size_t,
            ) == 0
            {
                break;
            }
            i += 1;
            i;
        }
        if ((*m).units[i as usize]).is_null() {
            return -(1 as libc::c_int);
        }
    }
    *n = base.wrapping_mul(scale);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn format_binary(mut n: f128::f128) -> *mut libc::c_char {
    return format_units(n, &mut binary_units, 2 as libc::c_int);
}
pub unsafe extern "C" fn format_metric(mut n: f128::f128) -> *mut libc::c_char {
    return format_units(n, &mut metric_units, 2 as libc::c_int);
}
pub unsafe extern "C" fn format_time_us(mut n: f128::f128) -> *mut libc::c_char {
    let mut units: *mut units = &mut time_units_us;
    if n >= f128::f128::new(1000000.0f64) {
        n /= f128::f128::new(1000000.0f64);
        units = &mut time_units_s;
    }
    return format_units(n, units, 2 as libc::c_int);
}
pub unsafe extern "C" fn format_time_s(mut n: f128::f128) -> *mut libc::c_char {
    return format_units(n, &mut time_units_s, 0 as libc::c_int);
}
pub unsafe extern "C" fn scan_metric(
    mut s: *mut libc::c_char,
    mut n: *mut uint64_t,
) -> libc::c_int {
    return scan_units(s, n, &mut metric_units);
}
pub unsafe extern "C" fn scan_time(
    mut s: *mut libc::c_char,
    mut n: *mut uint64_t,
) -> libc::c_int {
    return scan_units(s, n, &mut time_units_s);
}
