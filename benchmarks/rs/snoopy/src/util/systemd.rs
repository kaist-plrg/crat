use ::libc;
extern "C" {
    fn snoopy_util_pwd_convertUidToUsername(uid: uid_t) -> *mut libc::c_char;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strdup(__s: *const libc::c_char) -> *mut libc::c_char;
    fn strndup(__string: *const libc::c_char, __n: size_t) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
}
pub type uid_t = __uid_t;
pub type __uid_t = libc::c_uint;
pub type size_t = libc::c_ulong;
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
pub unsafe extern "C" fn snoopy_util_systemd_convertCgroupEntryToUnitName(
    cgroupEntry: *const libc::c_char,
) -> *mut libc::c_char {
    let mut matchPtr: *const libc::c_char = 0 as *const libc::c_char;
    let mut dotPtr: *const libc::c_char = 0 as *const libc::c_char;
    matchPtr = cgroupEntry_movePastInitialChaff(cgroupEntry);
    if matchPtr.is_null() {
        return 0 as *mut libc::c_char;
    }
    if *matchPtr as libc::c_int == '\0' as i32 {
        return strdup(b"-\0" as *const u8 as *const libc::c_char)
    } else if 0 as libc::c_int
        == strncmp(
            matchPtr,
            b"init.scope\0" as *const u8 as *const libc::c_char,
            strlen(b"init.scope\0" as *const u8 as *const libc::c_char),
        )
    {
        return strdup(b"init\0" as *const u8 as *const libc::c_char)
    } else if 0 as libc::c_int
        == strncmp(
            matchPtr,
            b"system.slice/\0" as *const u8 as *const libc::c_char,
            strlen(b"system.slice/\0" as *const u8 as *const libc::c_char),
        )
    {
        matchPtr = strchr(matchPtr, '/' as i32);
        matchPtr = matchPtr.offset(1);
        matchPtr;
        dotPtr = strchr(matchPtr, '.' as i32);
        if !dotPtr.is_null()
            && 0 as libc::c_int
                == strcmp(dotPtr, b".service\0" as *const u8 as *const libc::c_char)
        {
            return strndup(
                matchPtr,
                dotPtr.offset_from(matchPtr) as libc::c_long as size_t,
            )
        } else {
            return strdup(matchPtr)
        }
    } else if 0 as libc::c_int
        == strncmp(
            matchPtr,
            b"user.slice/\0" as *const u8 as *const libc::c_char,
            strlen(b"user.slice/\0" as *const u8 as *const libc::c_char),
        )
    {
        matchPtr = strchr(matchPtr, '/' as i32);
        matchPtr = matchPtr.offset(1);
        matchPtr;
        return snoopy_util_systemd_convertUserSliceInfoToUsername(matchPtr);
    }
    return 0 as *mut libc::c_char;
}
pub unsafe extern "C" fn snoopy_util_systemd_convertUserSliceInfoToUsername(
    userSliceInfo: *const libc::c_char,
) -> *mut libc::c_char {
    let mut matchPtr: *const libc::c_char = userSliceInfo;
    let mut dotPtr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut uid: uid_t = 0;
    if 0 as libc::c_int
        != strncmp(
            matchPtr,
            b"user-\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int as libc::c_ulong,
        )
    {
        return 0 as *mut libc::c_char;
    }
    matchPtr = matchPtr.offset(5 as libc::c_int as isize);
    dotPtr = strchr(matchPtr, '.' as i32);
    if dotPtr.is_null() {
        return 0 as *mut libc::c_char;
    }
    *dotPtr = '\0' as i32 as libc::c_char;
    uid = atoi(matchPtr) as uid_t;
    return snoopy_util_pwd_convertUidToUsername(uid);
}
unsafe extern "C" fn cgroupEntry_movePastInitialChaff(
    cgroupEntry: *const libc::c_char,
) -> *const libc::c_char {
    let mut matchPtr: *const libc::c_char = cgroupEntry;
    matchPtr = strchr(cgroupEntry, ':' as i32);
    if matchPtr.is_null() {
        return 0 as *const libc::c_char;
    }
    matchPtr = matchPtr.offset(1);
    matchPtr;
    matchPtr = strchr(matchPtr, ':' as i32);
    if matchPtr.is_null() {
        return 0 as *const libc::c_char;
    }
    matchPtr = matchPtr.offset(1);
    matchPtr;
    if *matchPtr as libc::c_int != '/' as i32 {
        return 0 as *const libc::c_char;
    }
    matchPtr = matchPtr.offset(1);
    matchPtr;
    return matchPtr;
}
