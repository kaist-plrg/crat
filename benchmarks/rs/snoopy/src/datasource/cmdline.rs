use ::libc;
extern "C" {
    fn snoopy_inputdatastorage_get() -> *mut snoopy_inputdatastorage_t;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct snoopy_inputdatastorage_t {
    pub initialized: libc::c_int,
    pub filename: *const libc::c_char,
    pub argv: *const *mut libc::c_char,
    pub envp: *const *mut libc::c_char,
}
pub unsafe extern "C" fn snoopy_datasource_cmdline(
    result: *mut libc::c_char,
    arg: *const libc::c_char,
) -> libc::c_int {
    let mut cmdLine: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cmdLineArgCount: libc::c_int = 0;
    let mut cmdLineSizeSum: libc::c_int = 0;
    let mut cmdLineSizeRet: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut snoopy_inputdatastorage: *const snoopy_inputdatastorage_t = 0
        as *const snoopy_inputdatastorage_t;
    snoopy_inputdatastorage = snoopy_inputdatastorage_get();
    if ((*snoopy_inputdatastorage).argv).is_null() {
        n = snprintf(
            result,
            2048 as libc::c_int as libc::c_ulong,
            b"%s\0" as *const u8 as *const libc::c_char,
            (*snoopy_inputdatastorage).filename,
        );
        return n;
    }
    cmdLineArgCount = 0 as libc::c_int;
    while !(*((*snoopy_inputdatastorage).argv).offset(cmdLineArgCount as isize))
        .is_null()
    {
        cmdLineArgCount += 1;
        cmdLineArgCount;
    }
    cmdLineSizeSum = 1 as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < cmdLineArgCount {
        cmdLineSizeSum = (cmdLineSizeSum as libc::c_ulong)
            .wrapping_add(
                (strlen(*((*snoopy_inputdatastorage).argv).offset(i as isize)))
                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
            ) as libc::c_int as libc::c_int;
        i += 1;
        i;
    }
    if cmdLineSizeSum > 2048 as libc::c_int {
        cmdLineSizeRet = 2048 as libc::c_int;
    } else {
        cmdLineSizeRet = cmdLineSizeSum;
    }
    cmdLine = malloc(cmdLineSizeRet as libc::c_ulong) as *mut libc::c_char;
    *cmdLine.offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
    n = 0 as libc::c_int;
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < cmdLineArgCount {
        n
            += snprintf(
                cmdLine.offset(n as isize),
                (cmdLineSizeRet - n) as libc::c_ulong,
                b"%s\0" as *const u8 as *const libc::c_char,
                *((*snoopy_inputdatastorage).argv).offset(i_0 as isize),
            );
        if n < cmdLineSizeRet {
            *cmdLine.offset(n as isize) = ' ' as i32 as libc::c_char;
            n += 1;
            n;
        }
        if n >= cmdLineSizeRet {
            n = cmdLineSizeRet;
            break;
        } else {
            i_0 += 1;
            i_0;
        }
    }
    if n > 0 as libc::c_int {
        n -= 1;
        n;
    }
    *cmdLine.offset(n as isize) = '\0' as i32 as libc::c_char;
    snprintf(
        result,
        2048 as libc::c_int as libc::c_ulong,
        b"%s\0" as *const u8 as *const libc::c_char,
        cmdLine,
    );
    free(cmdLine as *mut libc::c_void);
    return cmdLineSizeRet;
}
