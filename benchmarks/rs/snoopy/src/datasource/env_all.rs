use ::libc;
extern "C" {
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    static mut environ: *mut *mut libc::c_char;
}
pub unsafe extern "C" fn snoopy_datasource_env_all(
    result: *mut libc::c_char,
    arg: *const libc::c_char,
) -> libc::c_int {
    let mut resultSize: libc::c_int = 0 as libc::c_int;
    let mut envItem: *mut libc::c_char = *environ;
    let mut i: libc::c_int = 0 as libc::c_int;
    while !envItem.is_null() {
        i += 1;
        i;
        let mut remResultSize: libc::c_int = 0 as libc::c_int;
        remResultSize = 2048 as libc::c_int - resultSize;
        if i > 1 as libc::c_int && remResultSize >= 5 as libc::c_int {
            *result.offset(resultSize as isize) = ',' as i32 as libc::c_char;
            *result
                .offset(
                    (resultSize + 1 as libc::c_int) as isize,
                ) = '\0' as i32 as libc::c_char;
            resultSize += 1;
            resultSize;
            remResultSize -= 1;
            remResultSize;
        }
        if ((strlen(envItem))
            .wrapping_add(3 as libc::c_int as libc::c_ulong)
            .wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int)
            < remResultSize
        {
            let mut strSizeCopied: libc::c_int = 0 as libc::c_int;
            strSizeCopied = snprintf(
                &mut *result.offset(resultSize as isize) as *mut libc::c_char,
                remResultSize as libc::c_ulong,
                b"%s\0" as *const u8 as *const libc::c_char,
                envItem,
            );
            resultSize += strSizeCopied;
            envItem = *environ.offset(i as isize);
        } else {
            let mut strSizeToCopy: libc::c_int = 0 as libc::c_int;
            strSizeToCopy = remResultSize - 3 as libc::c_int;
            snprintf(
                &mut *result.offset(resultSize as isize) as *mut libc::c_char,
                strSizeToCopy as libc::c_ulong,
                b"%s\0" as *const u8 as *const libc::c_char,
                envItem,
            );
            resultSize += strSizeToCopy - 1 as libc::c_int;
            strSizeToCopy = 4 as libc::c_int;
            snprintf(
                &mut *result.offset(resultSize as isize) as *mut libc::c_char,
                strSizeToCopy as libc::c_ulong,
                b"...\0" as *const u8 as *const libc::c_char,
            );
            resultSize += strSizeToCopy - 1 as libc::c_int;
            break;
        }
    }
    return resultSize;
}
