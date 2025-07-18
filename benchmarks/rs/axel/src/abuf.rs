use ::libc;
extern "C" {
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn vsnprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlcat(_: *mut libc::c_char, _: *const libc::c_char, _: size_t) -> size_t;
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
pub type size_t = libc::c_ulong;
pub type va_list = __builtin_va_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct abuf_t {
    pub p: *mut libc::c_char,
    pub len: size_t,
}
pub unsafe extern "C" fn abuf_setup(
    mut abuf: *mut abuf_t,
    mut len: size_t,
) -> libc::c_int {
    let mut p: *mut libc::c_char = realloc((*abuf).p as *mut libc::c_void, len)
        as *mut libc::c_char;
    if p.is_null() && len != 0 {
        return -(12 as libc::c_int);
    }
    (*abuf).p = p;
    (*abuf).len = len;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn abuf_printf(
    mut abuf: *mut abuf_t,
    mut fmt: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut ap: ::std::ffi::VaListImpl;
    ap = args.clone();
    loop {
        let mut len: size_t = vsnprintf((*abuf).p, (*abuf).len, fmt, ap.as_va_list())
            as size_t;
        if len < (*abuf).len {
            break;
        }
        let mut r: libc::c_int = abuf_setup(
            abuf,
            len.wrapping_add(1 as libc::c_int as libc::c_ulong),
        );
        if r < 0 as libc::c_int {
            return r;
        }
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn abuf_strcat(
    mut abuf: *mut abuf_t,
    mut src: *const libc::c_char,
) -> libc::c_int {
    let mut nread: size_t = strlcat((*abuf).p, src, (*abuf).len);
    if nread > (*abuf).len {
        let mut done: size_t = ((*abuf).len)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        let mut ret: libc::c_int = abuf_setup(abuf, nread);
        if ret < 0 as libc::c_int {
            return ret;
        }
        memcpy(
            ((*abuf).p).offset(done as isize) as *mut libc::c_void,
            src.offset(done as isize) as *const libc::c_void,
            nread.wrapping_sub(done),
        );
    }
    return 0 as libc::c_int;
}
