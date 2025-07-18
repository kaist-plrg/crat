use ::libc;
extern "C" {
    fn __errno_location() -> *mut libc::c_int;
    fn mem_cd_iconv(
        src: *const libc::c_char,
        srclen: size_t,
        cd: iconv_t,
        resultp: *mut *mut libc::c_char,
        lengthp: *mut size_t,
    ) -> libc::c_int;
    fn str_cd_iconv(src: *const libc::c_char, cd: iconv_t) -> *mut libc::c_char;
    fn str_iconv(
        src: *const libc::c_char,
        from_codeset: *const libc::c_char,
        to_codeset: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn xalloc_die();
}
pub type size_t = libc::c_ulong;
pub type iconv_t = *mut libc::c_void;
pub unsafe extern "C" fn xmem_cd_iconv(
    mut src: *const libc::c_char,
    mut srclen: size_t,
    mut cd: iconv_t,
    mut resultp: *mut *mut libc::c_char,
    mut lengthp: *mut size_t,
) -> libc::c_int {
    let mut retval: libc::c_int = mem_cd_iconv(src, srclen, cd, resultp, lengthp);
    if retval < 0 as libc::c_int && *__errno_location() == 12 as libc::c_int {
        xalloc_die();
    }
    return retval;
}
pub unsafe extern "C" fn xstr_cd_iconv(
    mut src: *const libc::c_char,
    mut cd: iconv_t,
) -> *mut libc::c_char {
    let mut result: *mut libc::c_char = str_cd_iconv(src, cd);
    if result.is_null() && *__errno_location() == 12 as libc::c_int {
        xalloc_die();
    }
    return result;
}
pub unsafe extern "C" fn xstr_iconv(
    mut src: *const libc::c_char,
    mut from_codeset: *const libc::c_char,
    mut to_codeset: *const libc::c_char,
) -> *mut libc::c_char {
    let mut result: *mut libc::c_char = str_iconv(src, from_codeset, to_codeset);
    if result.is_null() && *__errno_location() == 12 as libc::c_int {
        xalloc_die();
    }
    return result;
}
