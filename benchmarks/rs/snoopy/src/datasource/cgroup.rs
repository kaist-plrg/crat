use ::libc;
extern "C" {
    fn getpid() -> __pid_t;
    fn snoopy_util_file_getSmallTextFileContent(
        filePath: *const libc::c_char,
        contentPtr: *mut *mut libc::c_char,
    ) -> libc::c_int;
    fn snoopy_util_string_containsOnlyDigits(str: *const libc::c_char) -> libc::c_int;
    fn snoopy_util_string_findLineStartingWith(
        content: *const libc::c_char,
        searchString: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn snoopy_util_string_nullTerminateLine(lineInContent: *const libc::c_char);
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strtok_r(
        __s: *mut libc::c_char,
        __delim: *const libc::c_char,
        __save_ptr: *mut *mut libc::c_char,
    ) -> *mut libc::c_char;
}
pub type size_t = libc::c_ulong;
pub type __pid_t = libc::c_int;
pub unsafe extern "C" fn snoopy_datasource_cgroup(
    result: *mut libc::c_char,
    arg: *const libc::c_char,
) -> libc::c_int {
    let mut myPid: libc::c_int = 0;
    let mut procPidCgroupFilePath: [libc::c_char; 32] = [0; 32];
    let mut procPidCgroupContent: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cgroupEntry: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut retMsgLen: libc::c_int = 0;
    if 0 as libc::c_int == strcmp(arg, b"\0" as *const u8 as *const libc::c_char) {
        snprintf(
            result,
            2048 as libc::c_int as libc::c_ulong,
            b"Missing cgroup selection argument\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    myPid = getpid();
    snprintf(
        procPidCgroupFilePath.as_mut_ptr(),
        32 as libc::c_int as libc::c_ulong,
        b"/proc/%d/cgroup\0" as *const u8 as *const libc::c_char,
        myPid,
    );
    if snoopy_util_file_getSmallTextFileContent(
        procPidCgroupFilePath.as_mut_ptr(),
        &mut procPidCgroupContent,
    ) < 0 as libc::c_int
    {
        snprintf(
            result,
            2048 as libc::c_int as libc::c_ulong,
            b"Unable to read file %s, reason: %s\0" as *const u8 as *const libc::c_char,
            procPidCgroupFilePath.as_mut_ptr(),
            procPidCgroupContent,
        );
        free(procPidCgroupContent as *mut libc::c_void);
        return -(1 as libc::c_int);
    }
    if 1 as libc::c_int == snoopy_util_string_containsOnlyDigits(arg) {
        let mut searchStringLen: size_t = 0;
        let mut searchString: *mut libc::c_char = 0 as *mut libc::c_char;
        searchStringLen = (strlen(arg)).wrapping_add(2 as libc::c_int as libc::c_ulong);
        searchString = malloc(searchStringLen) as *mut libc::c_char;
        snprintf(
            searchString,
            searchStringLen,
            b"%s:\0" as *const u8 as *const libc::c_char,
            arg,
        );
        cgroupEntry = snoopy_util_string_findLineStartingWith(
            procPidCgroupContent,
            searchString,
        );
        free(searchString as *mut libc::c_void);
        if !cgroupEntry.is_null() {
            snoopy_util_string_nullTerminateLine(cgroupEntry);
        }
    } else {
        let mut nextEntry: *mut libc::c_char = 0 as *mut libc::c_char;
        cgroupEntry = strtok_r(
            procPidCgroupContent,
            b"\n\0" as *const u8 as *const libc::c_char,
            &mut nextEntry,
        );
        while !cgroupEntry.is_null() {
            if 1 as libc::c_int == doesCgroupEntryContainController(cgroupEntry, arg) {
                break;
            }
            cgroupEntry = strtok_r(
                0 as *mut libc::c_char,
                b"\n\0" as *const u8 as *const libc::c_char,
                &mut nextEntry,
            );
        }
    }
    if cgroupEntry.is_null() {
        free(procPidCgroupContent as *mut libc::c_void);
        return snprintf(
            result,
            2048 as libc::c_int as libc::c_ulong,
            b"%s\0" as *const u8 as *const libc::c_char,
            b"(none)\0" as *const u8 as *const libc::c_char,
        );
    }
    retMsgLen = snprintf(
        result,
        2048 as libc::c_int as libc::c_ulong,
        b"%s\0" as *const u8 as *const libc::c_char,
        cgroupEntry,
    );
    free(procPidCgroupContent as *mut libc::c_void);
    return retMsgLen;
}
unsafe extern "C" fn doesCgroupEntryContainController(
    cgroupEntry: *const libc::c_char,
    controllerName: *const libc::c_char,
) -> libc::c_int {
    let mut firstColon: *const libc::c_char = 0 as *const libc::c_char;
    let mut controllerList: *const libc::c_char = 0 as *const libc::c_char;
    let mut secondColon: *mut libc::c_char = 0 as *mut libc::c_char;
    firstColon = strchr(cgroupEntry, ':' as i32);
    if firstColon.is_null() {
        return -(1 as libc::c_int);
    }
    controllerList = firstColon.offset(1 as libc::c_int as isize);
    secondColon = strchr(controllerList, ':' as i32);
    if secondColon.is_null() {
        return -(1 as libc::c_int);
    }
    if controllerList == secondColon as *const libc::c_char {
        return 0 as libc::c_int;
    }
    *secondColon = '\0' as i32 as libc::c_char;
    if 0 as libc::c_int == strcmp(controllerList, controllerName) {
        *secondColon = ':' as i32 as libc::c_char;
        return 1 as libc::c_int;
    }
    if (strchr(controllerList, ',' as i32)).is_null() {
        *secondColon = ':' as i32 as libc::c_char;
        return 0 as libc::c_int;
    }
    let mut tokenPtr: *const libc::c_char = controllerList;
    while !tokenPtr.is_null() {
        let mut commaPtr: *mut libc::c_char = strchr(tokenPtr, ',' as i32);
        if !commaPtr.is_null() {
            *commaPtr = '\0' as i32 as libc::c_char;
        }
        if 0 as libc::c_int == strcmp(tokenPtr, controllerName) {
            if !commaPtr.is_null() {
                *commaPtr = ',' as i32 as libc::c_char;
            }
            *secondColon = ':' as i32 as libc::c_char;
            return 1 as libc::c_int;
        }
        if !commaPtr.is_null() {
            *commaPtr = ',' as i32 as libc::c_char;
            tokenPtr = commaPtr.offset(1 as libc::c_int as isize);
        } else {
            tokenPtr = 0 as *const libc::c_char;
            break;
        }
    }
    *secondColon = ':' as i32 as libc::c_char;
    return 0 as libc::c_int;
}
