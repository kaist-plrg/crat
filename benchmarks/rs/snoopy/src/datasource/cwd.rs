use ::libc;
extern "C" {
    fn getcwd(__buf: *mut libc::c_char, __size: size_t) -> *mut libc::c_char;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub unsafe extern "C" fn snoopy_datasource_cwd(
    result: *mut libc::c_char,
    arg: *const libc::c_char,
) -> libc::c_int {
    let mut cwdBuf: [libc::c_char; 4097] = [0; 4097];
    if !(getcwd(cwdBuf.as_mut_ptr(), (4096 as libc::c_int + 1 as libc::c_int) as size_t))
        .is_null()
    {
        return snprintf(
            result,
            2048 as libc::c_int as libc::c_ulong,
            b"%s\0" as *const u8 as *const libc::c_char,
            cwdBuf.as_mut_ptr(),
        );
    }
    return -(1 as libc::c_int);
}
