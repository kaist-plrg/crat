use ::libc;
extern "C" {
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn vsnprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type va_list = __builtin_va_list;
pub unsafe extern "C" fn aprintf(
    mut s: *mut *mut libc::c_char,
    mut fmt: *const libc::c_char,
    mut args: ...
) -> *mut libc::c_char {
    let mut c: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut n: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut ap: ::std::ffi::VaListImpl;
    ap = args.clone();
    n = vsnprintf(
        0 as *mut libc::c_char,
        0 as libc::c_int as libc::c_ulong,
        fmt,
        ap.as_va_list(),
    ) + 1 as libc::c_int;
    len = (if !(*s).is_null() { strlen(*s) } else { 0 as libc::c_int as libc::c_ulong })
        as libc::c_int;
    *s = realloc(
        *s as *mut libc::c_void,
        ((len + n) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
    ) as *mut libc::c_char;
    if !(*s).is_null() {
        c = (*s).offset(len as isize);
        ap = args.clone();
        vsnprintf(c, n as libc::c_ulong, fmt, ap.as_va_list());
    }
    return c;
}
