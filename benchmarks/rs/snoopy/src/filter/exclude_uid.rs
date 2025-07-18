use ::libc;
extern "C" {
    fn getuid() -> __uid_t;
    fn snoopy_util_parser_csvToArgList(
        argListRaw: *mut libc::c_char,
        argListParsed: *mut *mut *mut libc::c_char,
    ) -> libc::c_int;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn free(__ptr: *mut libc::c_void);
    fn strdup(__s: *const libc::c_char) -> *mut libc::c_char;
}
pub type uid_t = __uid_t;
pub type __uid_t = libc::c_uint;
#[inline]
unsafe extern "C" fn atol(mut __nptr: *const libc::c_char) -> libc::c_long {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    );
}
pub unsafe extern "C" fn snoopy_filter_exclude_uid(
    arg: *const libc::c_char,
) -> libc::c_int {
    let mut current_block: u64;
    let mut curUid: uid_t = 0;
    let mut argDup: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut argParsed: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut argCount: libc::c_int = 0 as libc::c_int;
    let mut retVal: libc::c_int = -(1 as libc::c_int);
    curUid = getuid();
    argDup = strdup(arg);
    argCount = snoopy_util_parser_csvToArgList(argDup, &mut argParsed);
    let mut i: libc::c_int = 0 as libc::c_int;
    loop {
        if !(i < argCount) {
            current_block = 5720623009719927633;
            break;
        }
        let mut argCurUid: uid_t = 0;
        argCurUid = atol(*argParsed.offset(i as isize)) as uid_t;
        if argCurUid == curUid {
            retVal = 0 as libc::c_int;
            current_block = 5619145961076938547;
            break;
        } else {
            i += 1;
            i;
        }
    }
    match current_block {
        5720623009719927633 => {
            retVal = 1 as libc::c_int;
        }
        _ => {}
    }
    free(argDup as *mut libc::c_void);
    free(argParsed as *mut libc::c_void);
    return retVal;
}
