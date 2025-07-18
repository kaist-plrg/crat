use ::libc;
extern "C" {
    fn setlocale_null_r(
        category: libc::c_int,
        buf: *mut libc::c_char,
        bufsize: size_t,
    ) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub unsafe extern "C" fn hard_locale(mut category: libc::c_int) -> bool {
    let mut locale: [libc::c_char; 257] = [0; 257];
    if setlocale_null_r(
        category,
        locale.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 257]>() as libc::c_ulong,
    ) != 0
    {
        return 0 as libc::c_int != 0;
    }
    return !(strcmp(locale.as_mut_ptr(), b"C\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
        || strcmp(locale.as_mut_ptr(), b"POSIX\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int);
}
