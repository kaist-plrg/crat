use ::libc;
extern "C" {
    fn rpl_mbrtowc(
        pwc: *mut wchar_t,
        s: *const libc::c_char,
        n: size_t,
        ps: *mut mbstate_t,
    ) -> size_t;
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
static mut internal_state: mbstate_t = mbstate_t {
    __count: 0,
    __value: C2RustUnnamed { __wch: 0 },
};
pub unsafe extern "C" fn rpl_mbrlen(
    mut s: *const libc::c_char,
    mut n: size_t,
    mut ps: *mut mbstate_t,
) -> size_t {
    if ps.is_null() {
        ps = &mut internal_state;
    }
    return rpl_mbrtowc(0 as *mut wchar_t, s, n, ps);
}
