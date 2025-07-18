use ::libc;
extern "C" {
    fn ttyname_r(
        __fd: libc::c_int,
        __buf: *mut libc::c_char,
        __buflen: size_t,
    ) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub unsafe extern "C" fn snoopy_filter_only_tty(
    arg: *const libc::c_char,
) -> libc::c_int {
    let mut ttyPath: [libc::c_char; 4097] = [0; 4097];
    let mut ttyPathLen: size_t = 4096 as libc::c_int as size_t;
    let mut retVal: libc::c_int = ttyname_r(
        0 as libc::c_int,
        ttyPath.as_mut_ptr(),
        ttyPathLen,
    );
    if 0 as libc::c_int == retVal {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
