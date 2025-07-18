use ::libc;
extern "C" {
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
}
pub type size_t = libc::c_ulong;
pub const _ISdigit: C2RustUnnamed = 2048;
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
pub unsafe extern "C" fn snoopy_util_string_append(
    mut destString: *mut libc::c_char,
    mut destStringBufSize: size_t,
    mut appendThis: *const libc::c_char,
) -> libc::c_int {
    let mut destStringSize: size_t = 0 as libc::c_int as size_t;
    let mut destStringSizeRemaining: size_t = 0 as libc::c_int as size_t;
    let mut appendThisSize: size_t = 0 as libc::c_int as size_t;
    destStringSize = strlen(destString);
    appendThisSize = strlen(appendThis);
    destStringSizeRemaining = destStringBufSize.wrapping_sub(destStringSize);
    if destStringSizeRemaining < appendThisSize {
        return -(1 as libc::c_int);
    }
    strcat(&mut *destString.offset(destStringSize as isize), appendThis);
    return appendThisSize as libc::c_int;
}
pub unsafe extern "C" fn snoopy_util_string_containsOnlyDigits(
    mut str: *const libc::c_char,
) -> libc::c_int {
    while *str != 0 {
        let fresh0 = str;
        str = str.offset(1);
        if *(*__ctype_b_loc()).offset(*fresh0 as libc::c_int as isize) as libc::c_int
            & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
            == 0 as libc::c_int
        {
            return 0 as libc::c_int;
        }
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn snoopy_util_string_copyLineFromContent(
    lineStartPtr: *const libc::c_char,
) -> *mut libc::c_char {
    let mut lineLen: libc::c_int = 0 as libc::c_int;
    let mut copiedLine: *mut libc::c_char = 0 as *mut libc::c_char;
    lineLen = snoopy_util_string_getLineLength(lineStartPtr);
    copiedLine = malloc((lineLen + 1 as libc::c_int) as libc::c_ulong)
        as *mut libc::c_char;
    strncpy(copiedLine, lineStartPtr, lineLen as libc::c_ulong);
    *copiedLine.offset(lineLen as isize) = '\0' as i32 as libc::c_char;
    return copiedLine;
}
pub unsafe extern "C" fn snoopy_util_string_countChars(
    mut stringToSearch: *const libc::c_char,
    mut characterToCount: libc::c_char,
) -> libc::c_int {
    let mut strPtr: *const libc::c_char = stringToSearch;
    let mut charCount: libc::c_int = 0 as libc::c_int;
    while '\0' as i32 != *strPtr as libc::c_int {
        if *strPtr as libc::c_int == characterToCount as libc::c_int {
            charCount += 1;
            charCount;
        }
        strPtr = strPtr.offset(1);
        strPtr;
    }
    return charCount;
}
pub unsafe extern "C" fn snoopy_util_string_findLineStartingWith(
    content: *const libc::c_char,
    searchString: *const libc::c_char,
) -> *mut libc::c_char {
    let mut contentPos: *const libc::c_char = 0 as *const libc::c_char;
    let mut foundStringPos: *mut libc::c_char = 0 as *mut libc::c_char;
    contentPos = content;
    loop {
        foundStringPos = strstr(contentPos, searchString);
        if foundStringPos.is_null() {
            break;
        }
        if foundStringPos == content as *mut libc::c_char
            || foundStringPos > content as *mut libc::c_char
                && *foundStringPos.offset(-(1 as libc::c_int) as isize) as libc::c_int
                    == '\n' as i32
        {
            return foundStringPos;
        }
        contentPos = foundStringPos.offset(strlen(searchString) as isize);
    }
    return 0 as *mut libc::c_char;
}
pub unsafe extern "C" fn snoopy_util_string_getLineLength(
    lineStartPtr: *const libc::c_char,
) -> libc::c_int {
    let mut lineLen: libc::c_long = 0 as libc::c_int as libc::c_long;
    let mut newlineCharPos: *const libc::c_char = 0 as *const libc::c_char;
    newlineCharPos = strchr(lineStartPtr, '\n' as i32);
    if newlineCharPos.is_null() {
        lineLen = strlen(lineStartPtr) as libc::c_long;
    } else {
        lineLen = newlineCharPos.offset_from(lineStartPtr) as libc::c_long;
    }
    return lineLen as libc::c_int;
}
pub unsafe extern "C" fn snoopy_util_string_nullTerminateLine(
    lineInContent: *const libc::c_char,
) {
    let mut newlineCharPos: *mut libc::c_char = 0 as *mut libc::c_char;
    newlineCharPos = strchr(lineInContent, '\n' as i32);
    if !newlineCharPos.is_null() {
        *newlineCharPos = '\0' as i32 as libc::c_char;
    }
}
pub unsafe extern "C" fn snoopy_util_string_toUpper(mut s: *mut libc::c_char) {
    while *s != 0 {
        if *s as libc::c_int >= 'a' as i32 && *s as libc::c_int <= 'z' as i32 {
            *s = (*s as libc::c_int - ('a' as i32 - 'A' as i32)) as libc::c_char;
        }
        s = s.offset(1);
        s;
    }
}
