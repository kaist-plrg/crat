use ::libc;
extern "C" {
    fn ttyname_r(
        __fd: libc::c_int,
        __buf: *mut libc::c_char,
        __buflen: size_t,
    ) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub unsafe extern "C" fn snoopy_datasource_tty(
    result: *mut libc::c_char,
    arg: *const libc::c_char,
) -> libc::c_int {
    let mut ttyPath: [libc::c_char; 4097] = [0; 4097];
    let mut ttyPathLen: size_t = 4096 as libc::c_int as size_t;
    let mut retVal: libc::c_int = 0;
    retVal = ttyname_r(0 as libc::c_int, ttyPath.as_mut_ptr(), ttyPathLen);
    if 0 as libc::c_int != retVal {
        if 9 as libc::c_int == retVal {
            return snprintf(
                result,
                2048 as libc::c_int as libc::c_ulong,
                b"ERROR(ttyname_r->EBADF)\0" as *const u8 as *const libc::c_char,
            );
        }
        if 34 as libc::c_int == retVal {
            return snprintf(
                result,
                2048 as libc::c_int as libc::c_ulong,
                b"ERROR(ttyname_r->ERANGE)\0" as *const u8 as *const libc::c_char,
            );
        }
        if 25 as libc::c_int == retVal {
            return snprintf(
                result,
                2048 as libc::c_int as libc::c_ulong,
                b"(none)\0" as *const u8 as *const libc::c_char,
            );
        }
        return snprintf(
            result,
            2048 as libc::c_int as libc::c_ulong,
            b"(unknown)\0" as *const u8 as *const libc::c_char,
        );
    }
    return snprintf(
        result,
        2048 as libc::c_int as libc::c_ulong,
        b"%s\0" as *const u8 as *const libc::c_char,
        ttyPath.as_mut_ptr(),
    );
}
