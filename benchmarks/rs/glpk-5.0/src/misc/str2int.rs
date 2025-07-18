use ::libc;
extern "C" {
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
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
pub unsafe extern "C" fn _glp_str2int(
    mut str: *const libc::c_char,
    mut val_: *mut libc::c_int,
) -> libc::c_int {
    let mut d: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut s: libc::c_int = 0;
    let mut val: libc::c_int = 0 as libc::c_int;
    if *str.offset(0 as libc::c_int as isize) as libc::c_int == '+' as i32 {
        s = 1 as libc::c_int;
        k = 1 as libc::c_int;
    } else if *str.offset(0 as libc::c_int as isize) as libc::c_int == '-' as i32 {
        s = -(1 as libc::c_int);
        k = 1 as libc::c_int;
    } else {
        s = 1 as libc::c_int;
        k = 0 as libc::c_int;
    }
    if *(*__ctype_b_loc())
        .offset(*str.offset(k as isize) as libc::c_uchar as libc::c_int as isize)
        as libc::c_int & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int == 0
    {
        return 2 as libc::c_int;
    }
    while *(*__ctype_b_loc())
        .offset(*str.offset(k as isize) as libc::c_uchar as libc::c_int as isize)
        as libc::c_int & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        let fresh0 = k;
        k = k + 1;
        d = *str.offset(fresh0 as isize) as libc::c_int - '0' as i32;
        if s > 0 as libc::c_int {
            if val > 2147483647 as libc::c_int / 10 as libc::c_int {
                return 1 as libc::c_int;
            }
            val *= 10 as libc::c_int;
            if val > 2147483647 as libc::c_int - d {
                return 1 as libc::c_int;
            }
            val += d;
        } else {
            if val
                < (-(2147483647 as libc::c_int) - 1 as libc::c_int) / 10 as libc::c_int
            {
                return 1 as libc::c_int;
            }
            val *= 10 as libc::c_int;
            if val < -(2147483647 as libc::c_int) - 1 as libc::c_int + d {
                return 1 as libc::c_int;
            }
            val -= d;
        }
    }
    if *str.offset(k as isize) as libc::c_int != '\0' as i32 {
        return 2 as libc::c_int;
    }
    *val_ = val;
    return 0 as libc::c_int;
}
