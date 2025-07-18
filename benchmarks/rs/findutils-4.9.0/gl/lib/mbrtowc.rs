use ::libc;
extern "C" {
    fn mbrtowc(
        __pwc: *mut wchar_t,
        __s: *const libc::c_char,
        __n: size_t,
        __p: *mut mbstate_t,
    ) -> size_t;
    fn hard_locale(category: libc::c_int) -> bool;
}
pub type size_t = libc::c_ulong;
pub type wchar_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mbstate_t {
    pub __count: libc::c_int,
    pub __value: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub __wch: libc::c_uint,
    pub __wchb: [libc::c_char; 4],
}
pub type __mbstate_t = mbstate_t;
pub unsafe extern "C" fn rpl_mbrtowc(
    mut pwc: *mut wchar_t,
    mut s: *const libc::c_char,
    mut n: size_t,
    mut ps: *mut mbstate_t,
) -> size_t {
    let mut ret: size_t = 0;
    let mut wc: wchar_t = 0;
    if pwc.is_null() {
        pwc = &mut wc;
    }
    ret = mbrtowc(pwc, s, n, ps);
    if -(2 as libc::c_int) as size_t <= ret && n != 0 as libc::c_int as libc::c_ulong
        && !hard_locale(0 as libc::c_int)
    {
        let mut uc: libc::c_uchar = *s as libc::c_uchar;
        *pwc = uc as wchar_t;
        return 1 as libc::c_int as size_t;
    }
    return ret;
}
