use ::libc;
extern "C" {
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_double;
}
pub const _ISdigit: C2RustUnnamed = 2048;
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
pub unsafe extern "C" fn _glp_str2num(
    mut str: *const libc::c_char,
    mut val_: *mut libc::c_double,
) -> libc::c_int {
    let mut k: libc::c_int = 0;
    let mut val: libc::c_double = 0.;
    k = if *str.offset(0 as libc::c_int as isize) as libc::c_int == '+' as i32
        || *str.offset(0 as libc::c_int as isize) as libc::c_int == '-' as i32
    {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    };
    if *str.offset(k as isize) as libc::c_int == '.' as i32 {
        k += 1;
        k;
        if *(*__ctype_b_loc())
            .offset(*str.offset(k as isize) as libc::c_uchar as libc::c_int as isize)
            as libc::c_int & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
            == 0
        {
            return 2 as libc::c_int;
        }
        k += 1;
        k;
    } else {
        if *(*__ctype_b_loc())
            .offset(*str.offset(k as isize) as libc::c_uchar as libc::c_int as isize)
            as libc::c_int & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
            == 0
        {
            return 2 as libc::c_int;
        }
        while *(*__ctype_b_loc())
            .offset(*str.offset(k as isize) as libc::c_uchar as libc::c_int as isize)
            as libc::c_int & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
            != 0
        {
            k += 1;
            k;
        }
        if *str.offset(k as isize) as libc::c_int == '.' as i32 {
            k += 1;
            k;
        }
    }
    while *(*__ctype_b_loc())
        .offset(*str.offset(k as isize) as libc::c_uchar as libc::c_int as isize)
        as libc::c_int & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        k += 1;
        k;
    }
    if *str.offset(k as isize) as libc::c_int == 'E' as i32
        || *str.offset(k as isize) as libc::c_int == 'e' as i32
    {
        k += 1;
        k;
        if *str.offset(k as isize) as libc::c_int == '+' as i32
            || *str.offset(k as isize) as libc::c_int == '-' as i32
        {
            k += 1;
            k;
        }
        if *(*__ctype_b_loc())
            .offset(*str.offset(k as isize) as libc::c_uchar as libc::c_int as isize)
            as libc::c_int & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
            == 0
        {
            return 2 as libc::c_int;
        }
    }
    while *(*__ctype_b_loc())
        .offset(*str.offset(k as isize) as libc::c_uchar as libc::c_int as isize)
        as libc::c_int & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        k += 1;
        k;
    }
    if *str.offset(k as isize) as libc::c_int != '\0' as i32 {
        return 2 as libc::c_int;
    }
    let mut endptr: *mut libc::c_char = 0 as *mut libc::c_char;
    val = strtod(str, &mut endptr);
    if *endptr as libc::c_int != '\0' as i32 {
        return 2 as libc::c_int;
    }
    if !(-1.7976931348623157e+308f64 <= val && val <= 1.7976931348623157e+308f64) {
        return 1 as libc::c_int;
    }
    if -2.2250738585072014e-308f64 < val && val < 2.2250738585072014e-308f64 {
        val = 0.0f64;
    }
    *val_ = val;
    return 0 as libc::c_int;
}
