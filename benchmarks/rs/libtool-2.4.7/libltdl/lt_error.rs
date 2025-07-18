use ::libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn lt__realloc(mem: *mut libc::c_void, n: size_t) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
pub type C2RustUnnamed = libc::c_uint;
pub const LT_ERROR_MAX: C2RustUnnamed = 20;
pub const LT_ERROR_CONFLICTING_FLAGS: C2RustUnnamed = 19;
pub const LT_ERROR_INVALID_POSITION: C2RustUnnamed = 18;
pub const LT_ERROR_INVALID_MUTEX_ARGS: C2RustUnnamed = 17;
pub const LT_ERROR_CLOSE_RESIDENT_MODULE: C2RustUnnamed = 16;
pub const LT_ERROR_SHUTDOWN: C2RustUnnamed = 15;
pub const LT_ERROR_INVALID_ERRORCODE: C2RustUnnamed = 14;
pub const LT_ERROR_BUFFER_OVERFLOW: C2RustUnnamed = 13;
pub const LT_ERROR_INVALID_HANDLE: C2RustUnnamed = 12;
pub const LT_ERROR_NO_MEMORY: C2RustUnnamed = 11;
pub const LT_ERROR_SYMBOL_NOT_FOUND: C2RustUnnamed = 10;
pub const LT_ERROR_CANNOT_CLOSE: C2RustUnnamed = 9;
pub const LT_ERROR_CANNOT_OPEN: C2RustUnnamed = 8;
pub const LT_ERROR_NO_SYMBOLS: C2RustUnnamed = 7;
pub const LT_ERROR_DEPLIB_NOT_FOUND: C2RustUnnamed = 6;
pub const LT_ERROR_FILE_NOT_FOUND: C2RustUnnamed = 5;
pub const LT_ERROR_REMOVE_LOADER: C2RustUnnamed = 4;
pub const LT_ERROR_INIT_LOADER: C2RustUnnamed = 3;
pub const LT_ERROR_INVALID_LOADER: C2RustUnnamed = 2;
pub const LT_ERROR_DLOPEN_NOT_SUPPORTED: C2RustUnnamed = 1;
pub const LT_ERROR_UNKNOWN: C2RustUnnamed = 0;
static mut last_error: *const libc::c_char = 0 as *const libc::c_char;
static mut error_strings: [[libc::c_char; 42]; 20] = unsafe {
    [
        *::std::mem::transmute::<
            &[u8; 42],
            &[libc::c_char; 42],
        >(b"unknown error\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 42],
            &[libc::c_char; 42],
        >(b"dlopen support not available\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 42],
            &[libc::c_char; 42],
        >(b"invalid loader\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 42],
            &[libc::c_char; 42],
        >(b"loader initialization failed\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 42],
            &[libc::c_char; 42],
        >(b"loader removal failed\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 42],
            &[libc::c_char; 42],
        >(b"file not found\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 42],
            &[libc::c_char; 42],
        >(b"dependency library not found\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 42],
            &[libc::c_char; 42],
        >(b"no symbols defined\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 42],
            &[libc::c_char; 42],
        >(b"can't open the module\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 42],
            &[libc::c_char; 42],
        >(b"can't close the module\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 42],
            &[libc::c_char; 42],
        >(b"symbol not found\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 42],
            &[libc::c_char; 42],
        >(b"not enough memory\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 42],
            &[libc::c_char; 42],
        >(b"invalid module handle\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 42],
            &[libc::c_char; 42],
        >(b"internal buffer overflow\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 42],
            &[libc::c_char; 42],
        >(b"invalid errorcode\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 42],
            &[libc::c_char; 42],
        >(b"library already shutdown\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 42],
            &[libc::c_char; 42],
        >(b"can't close resident module\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 42],
            &[libc::c_char; 42],
        >(b"internal error (code withdrawn)\0\0\0\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 42],
            &[libc::c_char; 42],
        >(b"invalid search path insert position\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<
            &[u8; 42],
            &[libc::c_char; 42],
        >(b"symbol visibility can be global or local\0\0"),
    ]
};
static mut user_error_strings: *mut *const libc::c_char = 0 as *const *const libc::c_char
    as *mut *const libc::c_char;
static mut errorcount: libc::c_int = LT_ERROR_MAX as libc::c_int;
pub unsafe extern "C" fn lt_dladderror(
    mut diagnostic: *const libc::c_char,
) -> libc::c_int {
    let mut errindex: libc::c_int = 0 as libc::c_int;
    let mut result: libc::c_int = -(1 as libc::c_int);
    let mut temp: *mut *const libc::c_char = 0 as *mut *const libc::c_char;
    if !diagnostic.is_null() {} else {
        __assert_fail(
            b"diagnostic\0" as *const u8 as *const libc::c_char,
            b"libltdl/lt_error.c\0" as *const u8 as *const libc::c_char,
            53 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 32],
                &[libc::c_char; 32],
            >(b"int lt_dladderror(const char *)\0"))
                .as_ptr(),
        );
    }
    'c_3460: {
        if !diagnostic.is_null() {} else {
            __assert_fail(
                b"diagnostic\0" as *const u8 as *const libc::c_char,
                b"libltdl/lt_error.c\0" as *const u8 as *const libc::c_char,
                53 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 32],
                    &[libc::c_char; 32],
                >(b"int lt_dladderror(const char *)\0"))
                    .as_ptr(),
            );
        }
    };
    errindex = errorcount - LT_ERROR_MAX as libc::c_int;
    temp = lt__realloc(
        user_error_strings as *mut libc::c_void,
        ((1 as libc::c_int + errindex) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong),
    ) as *mut *const libc::c_char;
    if !temp.is_null() {
        user_error_strings = temp;
        let ref mut fresh0 = *user_error_strings.offset(errindex as isize);
        *fresh0 = diagnostic;
        let fresh1 = errorcount;
        errorcount = errorcount + 1;
        result = fresh1;
    }
    return result;
}
pub unsafe extern "C" fn lt_dlseterror(mut errindex: libc::c_int) -> libc::c_int {
    let mut errors: libc::c_int = 0 as libc::c_int;
    if errindex >= errorcount || errindex < 0 as libc::c_int {
        lt__set_last_error(lt__error_string(LT_ERROR_INVALID_ERRORCODE as libc::c_int));
        errors += 1;
        errors;
    } else if errindex < LT_ERROR_MAX as libc::c_int {
        lt__set_last_error((error_strings[errindex as usize]).as_ptr());
    } else {
        lt__set_last_error(
            *user_error_strings.offset((errindex - LT_ERROR_MAX as libc::c_int) as isize),
        );
    }
    return errors;
}
pub unsafe extern "C" fn lt__error_string(
    mut errorcode: libc::c_int,
) -> *const libc::c_char {
    if errorcode >= 0 as libc::c_int {} else {
        __assert_fail(
            b"errorcode >= 0\0" as *const u8 as *const libc::c_char,
            b"libltdl/lt_error.c\0" as *const u8 as *const libc::c_char,
            95 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 34],
                &[libc::c_char; 34],
            >(b"const char *lt__error_string(int)\0"))
                .as_ptr(),
        );
    }
    'c_3639: {
        if errorcode >= 0 as libc::c_int {} else {
            __assert_fail(
                b"errorcode >= 0\0" as *const u8 as *const libc::c_char,
                b"libltdl/lt_error.c\0" as *const u8 as *const libc::c_char,
                95 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 34],
                    &[libc::c_char; 34],
                >(b"const char *lt__error_string(int)\0"))
                    .as_ptr(),
            );
        }
    };
    if errorcode < LT_ERROR_MAX as libc::c_int {} else {
        __assert_fail(
            b"errorcode < LT_ERROR_MAX\0" as *const u8 as *const libc::c_char,
            b"libltdl/lt_error.c\0" as *const u8 as *const libc::c_char,
            96 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 34],
                &[libc::c_char; 34],
            >(b"const char *lt__error_string(int)\0"))
                .as_ptr(),
        );
    }
    'c_3601: {
        if errorcode < LT_ERROR_MAX as libc::c_int {} else {
            __assert_fail(
                b"errorcode < LT_ERROR_MAX\0" as *const u8 as *const libc::c_char,
                b"libltdl/lt_error.c\0" as *const u8 as *const libc::c_char,
                96 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 34],
                    &[libc::c_char; 34],
                >(b"const char *lt__error_string(int)\0"))
                    .as_ptr(),
            );
        }
    };
    return (error_strings[errorcode as usize]).as_ptr();
}
pub unsafe extern "C" fn lt__get_last_error() -> *const libc::c_char {
    return last_error;
}
pub unsafe extern "C" fn lt__set_last_error(
    mut errormsg: *const libc::c_char,
) -> *const libc::c_char {
    last_error = errormsg;
    return last_error;
}
