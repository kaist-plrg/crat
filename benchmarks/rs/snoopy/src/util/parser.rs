use ::libc;
extern "C" {
    fn snoopy_util_string_countChars(
        stringToSearch: *const libc::c_char,
        characterToCount: libc::c_char,
    ) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
}
pub unsafe extern "C" fn snoopy_util_parser_csvToArgList(
    mut argListRaw: *mut libc::c_char,
    mut argListParsed: *mut *mut *mut libc::c_char,
) -> libc::c_int {
    let mut commaCount: libc::c_int = 0;
    let mut argCount: libc::c_int = 0;
    let mut argListParsedPtr: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut argListRaw_charCount: libc::c_int = 0;
    let mut argListRaw_pos: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut nextCommaPtr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    argListRaw_charCount = strlen(argListRaw) as libc::c_int;
    commaCount = snoopy_util_string_countChars(argListRaw, ',' as i32 as libc::c_char);
    argCount = commaCount + 1 as libc::c_int;
    argListParsedPtr = malloc(
        (::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
            .wrapping_mul((argCount + 1 as libc::c_int) as libc::c_ulong),
    ) as *mut *mut libc::c_char;
    if 0 as libc::c_int == argListRaw_charCount {
        argCount = 0 as libc::c_int;
        i = 0 as libc::c_int;
    } else {
        let ref mut fresh0 = *argListParsedPtr.offset(0 as libc::c_int as isize);
        *fresh0 = argListRaw;
        i = 1 as libc::c_int;
    }
    if commaCount > 0 as libc::c_int {
        argListRaw_pos = argListRaw;
        loop {
            nextCommaPtr = strchr(argListRaw_pos, ',' as i32);
            if nextCommaPtr.is_null() {
                break;
            }
            *nextCommaPtr = '\0' as i32 as libc::c_char;
            argListRaw_pos = nextCommaPtr.offset(1 as libc::c_int as isize);
            let ref mut fresh1 = *argListParsedPtr.offset(i as isize);
            *fresh1 = argListRaw_pos;
            i += 1;
            i;
        }
    }
    let ref mut fresh2 = *argListParsedPtr.offset(i as isize);
    *fresh2 = argListRaw
        .offset(argListRaw_charCount as isize)
        .offset(1 as libc::c_int as isize);
    *argListParsed = argListParsedPtr;
    return argCount;
}
