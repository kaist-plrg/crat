use ::libc;
extern "C" {
    fn wcwidth(__c: wchar_t) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn iswcntrl(__wc: wint_t) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type wchar_t = libc::c_int;
pub type wint_t = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mbchar {
    pub ptr: *const libc::c_char,
    pub bytes: size_t,
    pub wc_valid: bool,
    pub wc: wchar_t,
    pub buf: [libc::c_char; 24],
}
pub type mbchar_t = mbchar;
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mb_width_aux(mut wc: wint_t) -> libc::c_int {
    let mut w: libc::c_int = wcwidth(wc as wchar_t);
    return if w >= 0 as libc::c_int {
        w
    } else if iswcntrl(wc) != 0 {
        0 as libc::c_int
    } else {
        1 as libc::c_int
    };
}
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mb_copy(
    mut new_mbc: *mut mbchar_t,
    mut old_mbc: *const mbchar_t,
) {
    if (*old_mbc).ptr
        == &*((*old_mbc).buf).as_ptr().offset(0 as libc::c_int as isize)
            as *const libc::c_char
    {
        memcpy(
            &mut *((*new_mbc).buf).as_mut_ptr().offset(0 as libc::c_int as isize)
                as *mut libc::c_char as *mut libc::c_void,
            &*((*old_mbc).buf).as_ptr().offset(0 as libc::c_int as isize)
                as *const libc::c_char as *const libc::c_void,
            (*old_mbc).bytes,
        );
        (*new_mbc)
            .ptr = &mut *((*new_mbc).buf).as_mut_ptr().offset(0 as libc::c_int as isize)
            as *mut libc::c_char;
    } else {
        (*new_mbc).ptr = (*old_mbc).ptr;
    }
    (*new_mbc).bytes = (*old_mbc).bytes;
    (*new_mbc).wc_valid = (*old_mbc).wc_valid;
    if (*new_mbc).wc_valid {
        (*new_mbc).wc = (*old_mbc).wc;
    }
}
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn is_basic(mut c: libc::c_char) -> bool {
    return *is_basic_table
        .as_ptr()
        .offset((c as libc::c_uchar as libc::c_int >> 5 as libc::c_int) as isize)
        >> (c as libc::c_uchar as libc::c_int & 31 as libc::c_int)
        & 1 as libc::c_int as libc::c_uint != 0;
}
pub static mut is_basic_table: [libc::c_uint; 8] = [
    0x1a00 as libc::c_int as libc::c_uint,
    0xffffffef as libc::c_uint,
    0xfffffffe as libc::c_uint,
    0x7ffffffe as libc::c_int as libc::c_uint,
    0,
    0,
    0,
    0,
];
