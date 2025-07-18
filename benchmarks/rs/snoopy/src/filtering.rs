use ::libc;
extern "C" {
    fn snoopy_filterregistry_doesNameExist(
        filterName: *const libc::c_char,
    ) -> libc::c_int;
    fn snoopy_filterregistry_callByName(
        filterName: *const libc::c_char,
        filterArg: *const libc::c_char,
    ) -> libc::c_int;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strtok_r(
        __s: *mut libc::c_char,
        __delim: *const libc::c_char,
        __save_ptr: *mut *mut libc::c_char,
    ) -> *mut libc::c_char;
}
pub type size_t = libc::c_ulong;
pub unsafe extern "C" fn snoopy_filtering_check_chain(
    filterChain: *const libc::c_char,
) -> libc::c_int {
    let mut filterChainCopy: [libc::c_char; 4096] = [0; 4096];
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut rest: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut filterSpec: *const libc::c_char = 0 as *const libc::c_char;
    let mut fcPos_filterSpecArg: *const libc::c_char = 0 as *const libc::c_char;
    strncpy(
        filterChainCopy.as_mut_ptr(),
        filterChain,
        (4096 as libc::c_int - 1 as libc::c_int) as libc::c_ulong,
    );
    filterChainCopy[(4096 as libc::c_int - 1 as libc::c_int)
        as usize] = '\0' as i32 as libc::c_char;
    str = filterChainCopy.as_mut_ptr();
    filterSpec = b"\0" as *const u8 as *const libc::c_char;
    let mut j: libc::c_int = 0 as libc::c_int;
    while !filterSpec.is_null() {
        j += 1;
        j;
        let mut filterName: [libc::c_char; 1024] = [0; 1024];
        let mut filterNamePtr: *const libc::c_char = 0 as *const libc::c_char;
        let mut filterNameSize: size_t = 0;
        let mut filterArg: [libc::c_char; 1024] = [0; 1024];
        let mut filterArgPtr: *const libc::c_char = 0 as *const libc::c_char;
        if j > 1 as libc::c_int {
            str = 0 as *mut libc::c_char;
        }
        filterSpec = strtok_r(
            str,
            b";\0" as *const u8 as *const libc::c_char,
            &mut rest,
        );
        if filterSpec.is_null() {
            continue;
        }
        fcPos_filterSpecArg = strstr(
            filterSpec,
            b":\0" as *const u8 as *const libc::c_char,
        );
        if fcPos_filterSpecArg.is_null() {
            filterName[0 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
            filterNamePtr = filterSpec;
            filterArg[0 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
            filterArgPtr = filterArg.as_mut_ptr();
        } else {
            filterNameSize = fcPos_filterSpecArg.offset_from(filterSpec) as libc::c_long
                as size_t;
            filterName[0 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
            strncpy(filterName.as_mut_ptr(), filterSpec, filterNameSize);
            filterName[filterNameSize as usize] = '\0' as i32 as libc::c_char;
            filterNamePtr = filterName.as_mut_ptr();
            filterArgPtr = fcPos_filterSpecArg.offset(1 as libc::c_int as isize);
        }
        if 0 as libc::c_int == snoopy_filterregistry_doesNameExist(filterNamePtr) {
            continue;
        }
        if 0 as libc::c_int
            == snoopy_filterregistry_callByName(filterNamePtr, filterArgPtr)
        {
            return 0 as libc::c_int;
        }
    }
    return 1 as libc::c_int;
}
