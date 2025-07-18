use ::libc;
extern "C" {
    fn rpl_mbrtowc(
        pwc: *mut wchar_t,
        s: *const libc::c_char,
        n: size_t,
        ps: *mut mbstate_t,
    ) -> size_t;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
pub type wchar_t = libc::c_int;
pub type wint_t = libc::c_uint;
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
pub unsafe extern "C" fn rpl_btowc(mut c: libc::c_int) -> wint_t {
    if c != -(1 as libc::c_int) {
        let mut buf: [libc::c_char; 1] = [0; 1];
        let mut wc: wchar_t = 0;
        buf[0 as libc::c_int as usize] = c as libc::c_char;
        let mut state: mbstate_t = mbstate_t {
            __count: 0,
            __value: C2RustUnnamed { __wch: 0 },
        };
        memset(
            &mut state as *mut mbstate_t as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<mbstate_t>() as libc::c_ulong,
        );
        let mut ret: size_t = rpl_mbrtowc(
            &mut wc,
            buf.as_mut_ptr(),
            1 as libc::c_int as size_t,
            &mut state,
        );
        if !(ret == -(1 as libc::c_int) as size_t
            || ret == -(2 as libc::c_int) as size_t)
        {
            return wc as wint_t;
        }
    }
    return 0xffffffff as libc::c_uint;
}
