use ::libc;
pub const GLOBPAT_SPECIAL: C2RustUnnamed = 1;
pub const GLOBPAT_NONE: C2RustUnnamed = 0;
pub const GLOBPAT_BRACKET: C2RustUnnamed = 4;
pub const GLOBPAT_BACKSLASH: C2RustUnnamed = 2;
pub type C2RustUnnamed = libc::c_uint;
#[inline]
unsafe extern "C" fn __glob_pattern_type(
    mut pattern: *const libc::c_char,
    mut quote: libc::c_int,
) -> libc::c_int {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut ret: libc::c_int = GLOBPAT_NONE as libc::c_int;
    p = pattern;
    while *p as libc::c_int != '\0' as i32 {
        match *p as libc::c_int {
            63 | 42 => return GLOBPAT_SPECIAL as libc::c_int,
            92 => {
                if quote != 0 {
                    if *p.offset(1 as libc::c_int as isize) as libc::c_int != '\0' as i32
                    {
                        p = p.offset(1);
                        p;
                    }
                    ret |= GLOBPAT_BACKSLASH as libc::c_int;
                }
            }
            91 => {
                ret |= GLOBPAT_BRACKET as libc::c_int;
            }
            93 => {
                if ret & 4 as libc::c_int != 0 {
                    return GLOBPAT_SPECIAL as libc::c_int;
                }
            }
            _ => {}
        }
        p = p.offset(1);
        p;
    }
    return ret;
}
pub unsafe extern "C" fn rpl_glob_pattern_p(
    mut pattern: *const libc::c_char,
    mut quote: libc::c_int,
) -> libc::c_int {
    return (__glob_pattern_type(pattern, quote) == GLOBPAT_SPECIAL as libc::c_int)
        as libc::c_int;
}
