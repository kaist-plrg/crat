use ::libc;
extern "C" {
    fn free(__ptr: *mut libc::c_void);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn u8_check(s: *const uint8_t, n: size_t) -> *const uint8_t;
    fn log_fatal(
        loggerName: *const libc::c_char,
        logMessage: *const libc::c_char,
        _: ...
    ) -> !;
    fn log_warn(
        loggerName: *const libc::c_char,
        logMessage: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn xcalloc(count: size_t, size: size_t) -> *mut libc::c_void;
    fn xmalloc(size: size_t) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct field_def {
    pub name: *const libc::c_char,
    pub type_0: *const libc::c_char,
    pub desc: *const libc::c_char,
}
pub type fielddef_t = field_def;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fielddef_set {
    pub fielddefs: [fielddef_t; 128],
    pub len: libc::c_int,
}
pub type fielddefset_t = fielddef_set;
#[derive(Copy, Clone)]
#[repr(C)]
pub union field_val {
    pub ptr: *mut libc::c_void,
    pub num: uint64_t,
}
pub type field_val_t = field_val;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct field {
    pub name: *const libc::c_char,
    pub type_0: libc::c_int,
    pub free_: libc::c_int,
    pub len: size_t,
    pub value: field_val_t,
}
pub type field_t = field;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fieldset {
    pub len: libc::c_int,
    pub fields: [field_t; 128],
    pub fds: *mut fielddefset_t,
    pub inner_type: libc::c_int,
    pub type_0: libc::c_int,
    pub free_: libc::c_int,
}
pub type fieldset_t = fieldset;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct translation {
    pub len: libc::c_int,
    pub translation: [libc::c_int; 128],
}
pub type translation_t = translation;
pub unsafe extern "C" fn gen_fielddef_set(
    mut fds: *mut fielddefset_t,
    mut fs: *mut fielddef_t,
    mut len: libc::c_int,
) {
    if (*fds).len + len > 128 as libc::c_int {
        log_fatal(
            b"fieldset\0" as *const u8 as *const libc::c_char,
            b"out of room in field def set\0" as *const u8 as *const libc::c_char,
        );
    }
    let mut open: *mut fielddef_t = &mut *((*fds).fielddefs)
        .as_mut_ptr()
        .offset((*fds).len as isize) as *mut fielddef_t;
    memcpy(
        open as *mut libc::c_void,
        fs as *const libc::c_void,
        (len as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<fielddef_t>() as libc::c_ulong),
    );
    (*fds).len += len;
}
pub unsafe extern "C" fn fs_new_fieldset(
    mut fds: *mut fielddefset_t,
) -> *mut fieldset_t {
    let mut f: *mut fieldset_t = xcalloc(
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<fieldset_t>() as libc::c_ulong,
    ) as *mut fieldset_t;
    (*f).len = 0 as libc::c_int;
    (*f).type_0 = 5 as libc::c_int;
    (*f).fds = fds;
    return f;
}
pub unsafe extern "C" fn fs_new_repeated_field(
    mut type_0: libc::c_int,
    mut free_: libc::c_int,
) -> *mut fieldset_t {
    let mut f: *mut fieldset_t = xcalloc(
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<fieldset_t>() as libc::c_ulong,
    ) as *mut fieldset_t;
    (*f).len = 0 as libc::c_int;
    (*f).type_0 = 6 as libc::c_int;
    (*f).inner_type = type_0;
    (*f).free_ = free_;
    return f;
}
pub unsafe extern "C" fn fs_new_repeated_uint64() -> *mut fieldset_t {
    return fs_new_repeated_field(2 as libc::c_int, 0 as libc::c_int);
}
pub unsafe extern "C" fn fs_new_repeated_bool() -> *mut fieldset_t {
    return fs_new_repeated_field(7 as libc::c_int, 0 as libc::c_int);
}
pub unsafe extern "C" fn fs_new_repeated_string(
    mut free_: libc::c_int,
) -> *mut fieldset_t {
    return fs_new_repeated_field(1 as libc::c_int, free_);
}
pub unsafe extern "C" fn fs_new_repeated_binary(
    mut free_: libc::c_int,
) -> *mut fieldset_t {
    return fs_new_repeated_field(3 as libc::c_int, free_);
}
pub unsafe extern "C" fn fs_new_repeated_fieldset() -> *mut fieldset_t {
    return fs_new_repeated_field(5 as libc::c_int, 0 as libc::c_int);
}
#[inline]
unsafe extern "C" fn fs_add_word(
    mut fs: *mut fieldset_t,
    mut name: *const libc::c_char,
    mut type_0: libc::c_int,
    mut free_: libc::c_int,
    mut len: size_t,
    mut value: field_val_t,
) {
    if (*fs).len + 1 as libc::c_int >= 128 as libc::c_int {
        log_fatal(
            b"fieldset\0" as *const u8 as *const libc::c_char,
            b"out of room in fieldset\0" as *const u8 as *const libc::c_char,
        );
    }
    if (*fs).type_0 == 6 as libc::c_int && (*fs).inner_type != type_0 {
        log_fatal(
            b"fieldset\0" as *const u8 as *const libc::c_char,
            b"object added to repeated field does not match type of repeated field.\0"
                as *const u8 as *const libc::c_char,
        );
    }
    let mut f: *mut field_t = &mut *((*fs).fields)
        .as_mut_ptr()
        .offset((*fs).len as isize) as *mut field_t;
    if !((*fs).fds).is_null()
        && strcmp((*(*fs).fds).fielddefs[(*fs).len as usize].name, name) != 0
    {
        log_fatal(
            b"fieldset\0" as *const u8 as *const libc::c_char,
            b"added field (%s) is not next expected field (%s).\0" as *const u8
                as *const libc::c_char,
            name,
            (*(*fs).fds).fielddefs[(*fs).len as usize].name,
        );
    }
    (*fs).len += 1;
    (*fs).len;
    (*f).type_0 = type_0;
    (*f).name = name;
    (*f).len = len;
    (*f).value = value;
    (*f).free_ = free_;
}
unsafe extern "C" fn fs_modify_word(
    mut fs: *mut fieldset_t,
    mut name: *const libc::c_char,
    mut type_0: libc::c_int,
    mut free_: libc::c_int,
    mut len: size_t,
    mut value: field_val_t,
) {
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < (*fs).len {
        if strcmp((*fs).fields[i as usize].name, name) == 0 {
            if (*fs).fields[i as usize].free_ != 0 {
                free((*fs).fields[i as usize].value.ptr);
                (*fs).fields[i as usize].value.ptr = 0 as *mut libc::c_void;
            }
            (*fs).fields[i as usize].type_0 = type_0;
            (*fs).fields[i as usize].free_ = free_;
            (*fs).fields[i as usize].len = len;
            (*fs).fields[i as usize].value = value;
            return;
        }
        i += 1;
        i;
    }
    log_fatal(
        b"fs\0" as *const u8 as *const libc::c_char,
        b"attempting to modify non-existent field\0" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn sanitize_utf8(mut buf: *const libc::c_char) -> *mut libc::c_char {
    let mut ptr: *const libc::c_char = buf;
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while (i as libc::c_ulong) < strlen(buf) && ptr < buf.offset(strlen(buf) as isize) {
        ptr = u8_check(ptr as *mut uint8_t, strlen(ptr)) as *mut libc::c_char;
        if ptr.is_null() {
            break;
        }
        if ptr >= buf {} else {
            __assert_fail(
                b"ptr >= buf\0" as *const u8 as *const libc::c_char,
                b"fieldset.c\0" as *const u8 as *const libc::c_char,
                140 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 34],
                    &[libc::c_char; 34],
                >(b"char *sanitize_utf8(const char *)\0"))
                    .as_ptr(),
            );
        }
        'c_2436: {
            if ptr >= buf {} else {
                __assert_fail(
                    b"ptr >= buf\0" as *const u8 as *const libc::c_char,
                    b"fieldset.c\0" as *const u8 as *const libc::c_char,
                    140 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 34],
                        &[libc::c_char; 34],
                    >(b"char *sanitize_utf8(const char *)\0"))
                        .as_ptr(),
                );
            }
        };
        if ptr < buf.offset(strlen(buf) as isize) {} else {
            __assert_fail(
                b"ptr < buf + strlen(buf)\0" as *const u8 as *const libc::c_char,
                b"fieldset.c\0" as *const u8 as *const libc::c_char,
                141 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 34],
                    &[libc::c_char; 34],
                >(b"char *sanitize_utf8(const char *)\0"))
                    .as_ptr(),
            );
        }
        'c_2386: {
            if ptr < buf.offset(strlen(buf) as isize) {} else {
                __assert_fail(
                    b"ptr < buf + strlen(buf)\0" as *const u8 as *const libc::c_char,
                    b"fieldset.c\0" as *const u8 as *const libc::c_char,
                    141 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 34],
                        &[libc::c_char; 34],
                    >(b"char *sanitize_utf8(const char *)\0"))
                        .as_ptr(),
                );
            }
        };
        ptr = ptr.offset(1);
        ptr;
        i = i.wrapping_add(1);
        i;
    }
    let mut safe_buf: *mut libc::c_char = xmalloc(
        (strlen(buf))
            .wrapping_add(
                i.wrapping_mul(2 as libc::c_int as libc::c_uint) as libc::c_ulong,
            )
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    let mut safe_ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    memcpy(safe_buf as *mut libc::c_void, buf as *const libc::c_void, strlen(buf));
    let mut j: uint32_t = 0 as libc::c_int as uint32_t;
    while j < i {
        safe_ptr = u8_check(safe_buf as *mut uint8_t, strlen(safe_buf))
            as *mut libc::c_char;
        if safe_ptr.is_null() {
            log_warn(
                b"fieldset\0" as *const u8 as *const libc::c_char,
                b"UTF8 Sanitization issue. %u errors, fell through iter %u. Orig: %s new: %s\0"
                    as *const u8 as *const libc::c_char,
                i,
                j,
                buf,
                safe_buf,
            );
            i = j;
            break;
        } else {
            if safe_ptr >= safe_buf {} else {
                __assert_fail(
                    b"safe_ptr >= safe_buf\0" as *const u8 as *const libc::c_char,
                    b"fieldset.c\0" as *const u8 as *const libc::c_char,
                    171 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 34],
                        &[libc::c_char; 34],
                    >(b"char *sanitize_utf8(const char *)\0"))
                        .as_ptr(),
                );
            }
            'c_2258: {
                if safe_ptr >= safe_buf {} else {
                    __assert_fail(
                        b"safe_ptr >= safe_buf\0" as *const u8 as *const libc::c_char,
                        b"fieldset.c\0" as *const u8 as *const libc::c_char,
                        171 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 34],
                            &[libc::c_char; 34],
                        >(b"char *sanitize_utf8(const char *)\0"))
                            .as_ptr(),
                    );
                }
            };
            if safe_ptr < safe_buf.offset(strlen(safe_buf) as isize) {} else {
                __assert_fail(
                    b"safe_ptr < safe_buf + strlen(safe_buf)\0" as *const u8
                        as *const libc::c_char,
                    b"fieldset.c\0" as *const u8 as *const libc::c_char,
                    172 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 34],
                        &[libc::c_char; 34],
                    >(b"char *sanitize_utf8(const char *)\0"))
                        .as_ptr(),
                );
            }
            'c_2205: {
                if safe_ptr < safe_buf.offset(strlen(safe_buf) as isize) {} else {
                    __assert_fail(
                        b"safe_ptr < safe_buf + strlen(safe_buf)\0" as *const u8
                            as *const libc::c_char,
                        b"fieldset.c\0" as *const u8 as *const libc::c_char,
                        172 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 34],
                            &[libc::c_char; 34],
                        >(b"char *sanitize_utf8(const char *)\0"))
                            .as_ptr(),
                    );
                }
            };
            if strlen(safe_ptr) > 1 as libc::c_int as libc::c_ulong {
                memcpy(
                    safe_ptr.offset(3 as libc::c_int as isize) as *mut libc::c_void,
                    safe_ptr.offset(1 as libc::c_int as isize) as *const libc::c_void,
                    strlen(safe_ptr.offset(1 as libc::c_int as isize)),
                );
            }
            *safe_ptr
                .offset(0 as libc::c_int as isize) = 0xef as libc::c_int as libc::c_char;
            *safe_ptr
                .offset(1 as libc::c_int as isize) = 0xbf as libc::c_int as libc::c_char;
            *safe_ptr
                .offset(2 as libc::c_int as isize) = 0xbd as libc::c_int as libc::c_char;
            j = j.wrapping_add(1);
            j;
        }
    }
    if (u8_check(safe_buf as *mut uint8_t, strlen(safe_buf))).is_null() {} else {
        __assert_fail(
            b"u8_check((uint8_t *)safe_buf, strlen(safe_buf)) == NULL\0" as *const u8
                as *const libc::c_char,
            b"fieldset.c\0" as *const u8 as *const libc::c_char,
            187 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 34],
                &[libc::c_char; 34],
            >(b"char *sanitize_utf8(const char *)\0"))
                .as_ptr(),
        );
    }
    'c_2062: {
        if (u8_check(safe_buf as *mut uint8_t, strlen(safe_buf))).is_null() {} else {
            __assert_fail(
                b"u8_check((uint8_t *)safe_buf, strlen(safe_buf)) == NULL\0" as *const u8
                    as *const libc::c_char,
                b"fieldset.c\0" as *const u8 as *const libc::c_char,
                187 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 34],
                    &[libc::c_char; 34],
                >(b"char *sanitize_utf8(const char *)\0"))
                    .as_ptr(),
            );
        }
    };
    if *safe_buf
        .offset(
            (strlen(buf))
                .wrapping_add(
                    i.wrapping_mul(2 as libc::c_int as libc::c_uint) as libc::c_ulong,
                ) as isize,
        ) as libc::c_int == '\0' as i32
    {} else {
        __assert_fail(
            b"safe_buf[strlen(buf) + i * 2] == '\\0'\0" as *const u8
                as *const libc::c_char,
            b"fieldset.c\0" as *const u8 as *const libc::c_char,
            189 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 34],
                &[libc::c_char; 34],
            >(b"char *sanitize_utf8(const char *)\0"))
                .as_ptr(),
        );
    }
    'c_1995: {
        if *safe_buf
            .offset(
                (strlen(buf))
                    .wrapping_add(
                        i.wrapping_mul(2 as libc::c_int as libc::c_uint) as libc::c_ulong,
                    ) as isize,
            ) as libc::c_int == '\0' as i32
        {} else {
            __assert_fail(
                b"safe_buf[strlen(buf) + i * 2] == '\\0'\0" as *const u8
                    as *const libc::c_char,
                b"fieldset.c\0" as *const u8 as *const libc::c_char,
                189 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 34],
                    &[libc::c_char; 34],
                >(b"char *sanitize_utf8(const char *)\0"))
                    .as_ptr(),
            );
        }
    };
    if strlen(safe_buf)
        == (strlen(buf))
            .wrapping_add(
                i.wrapping_mul(2 as libc::c_int as libc::c_uint) as libc::c_ulong,
            )
    {} else {
        __assert_fail(
            b"strlen(safe_buf) == (strlen(buf) + i * 2)\0" as *const u8
                as *const libc::c_char,
            b"fieldset.c\0" as *const u8 as *const libc::c_char,
            191 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 34],
                &[libc::c_char; 34],
            >(b"char *sanitize_utf8(const char *)\0"))
                .as_ptr(),
        );
    }
    'c_1916: {
        if strlen(safe_buf)
            == (strlen(buf))
                .wrapping_add(
                    i.wrapping_mul(2 as libc::c_int as libc::c_uint) as libc::c_ulong,
                )
        {} else {
            __assert_fail(
                b"strlen(safe_buf) == (strlen(buf) + i * 2)\0" as *const u8
                    as *const libc::c_char,
                b"fieldset.c\0" as *const u8 as *const libc::c_char,
                191 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 34],
                    &[libc::c_char; 34],
                >(b"char *sanitize_utf8(const char *)\0"))
                    .as_ptr(),
            );
        }
    };
    return safe_buf;
}
pub unsafe extern "C" fn fs_add_null(
    mut fs: *mut fieldset_t,
    mut name: *const libc::c_char,
) {
    let mut val: field_val_t = field_val {
        ptr: 0 as *mut libc::c_void,
    };
    fs_add_word(
        fs,
        name,
        4 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int as size_t,
        val,
    );
}
pub unsafe extern "C" fn fs_add_string(
    mut fs: *mut fieldset_t,
    mut name: *const libc::c_char,
    mut value: *mut libc::c_char,
    mut free_: libc::c_int,
) {
    let mut val: field_val_t = field_val {
        ptr: value as *mut libc::c_void,
    };
    fs_add_word(fs, name, 1 as libc::c_int, free_, strlen(value), val);
}
pub unsafe extern "C" fn fs_add_unsafe_string(
    mut fs: *mut fieldset_t,
    mut name: *const libc::c_char,
    mut value: *mut libc::c_char,
    mut free_: libc::c_int,
) {
    if (u8_check(value as *mut uint8_t, strlen(value))).is_null() {
        let mut val: field_val_t = field_val {
            ptr: value as *mut libc::c_void,
        };
        fs_add_word(fs, name, 1 as libc::c_int, free_, strlen(value), val);
    } else {
        let mut safe_value: *mut libc::c_char = sanitize_utf8(value);
        if free_ != 0 {
            free(value as *mut libc::c_void);
        }
        let mut val_0: field_val_t = field_val {
            ptr: safe_value as *mut libc::c_void,
        };
        fs_add_word(
            fs,
            name,
            1 as libc::c_int,
            1 as libc::c_int,
            strlen(safe_value),
            val_0,
        );
    };
}
pub unsafe extern "C" fn fs_chkadd_string(
    mut fs: *mut fieldset_t,
    mut name: *const libc::c_char,
    mut value: *mut libc::c_char,
    mut free_: libc::c_int,
) {
    if !value.is_null() {
        fs_add_string(fs, name, value, free_);
    } else {
        fs_add_null(fs, name);
    };
}
pub unsafe extern "C" fn fs_chkadd_unsafe_string(
    mut fs: *mut fieldset_t,
    mut name: *const libc::c_char,
    mut value: *mut libc::c_char,
    mut free_: libc::c_int,
) {
    if !value.is_null() {
        fs_add_unsafe_string(fs, name, value, free_);
    } else {
        fs_add_null(fs, name);
    };
}
pub unsafe extern "C" fn fs_add_constchar(
    mut fs: *mut fieldset_t,
    mut name: *const libc::c_char,
    mut value: *const libc::c_char,
) {
    let mut val: field_val_t = field_val {
        ptr: value as *mut libc::c_char as *mut libc::c_void,
    };
    fs_add_word(fs, name, 1 as libc::c_int, 0 as libc::c_int, strlen(value), val);
}
pub unsafe extern "C" fn fs_add_uint64(
    mut fs: *mut fieldset_t,
    mut name: *const libc::c_char,
    mut value: uint64_t,
) {
    let mut val: field_val_t = field_val { num: value };
    fs_add_word(
        fs,
        name,
        2 as libc::c_int,
        0 as libc::c_int,
        ::std::mem::size_of::<uint64_t>() as libc::c_ulong,
        val,
    );
}
pub unsafe extern "C" fn fs_add_bool(
    mut fs: *mut fieldset_t,
    mut name: *const libc::c_char,
    mut value: libc::c_int,
) {
    let mut val: field_val_t = field_val {
        num: value as uint64_t,
    };
    fs_add_word(
        fs,
        name,
        7 as libc::c_int,
        0 as libc::c_int,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
        val,
    );
}
pub unsafe extern "C" fn fs_add_binary(
    mut fs: *mut fieldset_t,
    mut name: *const libc::c_char,
    mut len: size_t,
    mut value: *mut libc::c_void,
    mut free_: libc::c_int,
) {
    let mut val: field_val_t = field_val { ptr: value };
    fs_add_word(fs, name, 3 as libc::c_int, free_, len, val);
}
pub unsafe extern "C" fn fs_add_fieldset(
    mut fs: *mut fieldset_t,
    mut name: *const libc::c_char,
    mut child: *mut fieldset_t,
) {
    let mut val: field_val_t = field_val {
        ptr: child as *mut libc::c_void,
    };
    fs_add_word(
        fs,
        name,
        5 as libc::c_int,
        1 as libc::c_int,
        ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
        val,
    );
}
pub unsafe extern "C" fn fs_add_repeated(
    mut fs: *mut fieldset_t,
    mut name: *const libc::c_char,
    mut child: *mut fieldset_t,
) {
    let mut val: field_val_t = field_val {
        ptr: child as *mut libc::c_void,
    };
    fs_add_word(
        fs,
        name,
        6 as libc::c_int,
        1 as libc::c_int,
        ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
        val,
    );
}
pub unsafe extern "C" fn fs_modify_null(
    mut fs: *mut fieldset_t,
    mut name: *const libc::c_char,
) {
    let mut val: field_val_t = field_val {
        ptr: 0 as *mut libc::c_void,
    };
    fs_modify_word(
        fs,
        name,
        4 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int as size_t,
        val,
    );
}
pub unsafe extern "C" fn fs_modify_string(
    mut fs: *mut fieldset_t,
    mut name: *const libc::c_char,
    mut value: *mut libc::c_char,
    mut free_: libc::c_int,
) {
    let mut val: field_val_t = field_val {
        ptr: value as *mut libc::c_void,
    };
    fs_modify_word(fs, name, 1 as libc::c_int, free_, strlen(value), val);
}
pub unsafe extern "C" fn fs_modify_constchar(
    mut fs: *mut fieldset_t,
    mut name: *const libc::c_char,
    mut value: *const libc::c_char,
) {
    let mut val: field_val_t = field_val {
        ptr: value as *mut libc::c_char as *mut libc::c_void,
    };
    fs_modify_word(fs, name, 1 as libc::c_int, 0 as libc::c_int, strlen(value), val);
}
pub unsafe extern "C" fn fs_modify_uint64(
    mut fs: *mut fieldset_t,
    mut name: *const libc::c_char,
    mut value: uint64_t,
) {
    let mut val: field_val_t = field_val { num: value };
    fs_modify_word(
        fs,
        name,
        2 as libc::c_int,
        0 as libc::c_int,
        ::std::mem::size_of::<uint64_t>() as libc::c_ulong,
        val,
    );
}
pub unsafe extern "C" fn fs_modify_bool(
    mut fs: *mut fieldset_t,
    mut name: *const libc::c_char,
    mut value: libc::c_int,
) {
    let mut val: field_val_t = field_val {
        num: value as uint64_t,
    };
    fs_modify_word(
        fs,
        name,
        7 as libc::c_int,
        0 as libc::c_int,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
        val,
    );
}
pub unsafe extern "C" fn fs_modify_binary(
    mut fs: *mut fieldset_t,
    mut name: *const libc::c_char,
    mut len: size_t,
    mut value: *mut libc::c_void,
    mut free_: libc::c_int,
) {
    let mut val: field_val_t = field_val { ptr: value };
    fs_modify_word(fs, name, 3 as libc::c_int, free_, len, val);
}
pub unsafe extern "C" fn fs_get_uint64_by_index(
    mut fs: *mut fieldset_t,
    mut index: libc::c_int,
) -> uint64_t {
    return (*fs).fields[index as usize].value.num;
}
pub unsafe extern "C" fn fs_get_string_by_index(
    mut fs: *mut fieldset_t,
    mut index: libc::c_int,
) -> *mut libc::c_char {
    return (*fs).fields[index as usize].value.ptr as *mut libc::c_char;
}
pub unsafe extern "C" fn fds_get_index_by_name(
    mut fds: *mut fielddefset_t,
    mut name: *const libc::c_char,
) -> libc::c_int {
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < (*fds).len {
        if strcmp((*fds).fielddefs[i as usize].name, name) == 0 {
            return i;
        }
        i += 1;
        i;
    }
    return -(1 as libc::c_int);
}
pub unsafe extern "C" fn field_free(mut f: *mut field_t) {
    if (*f).type_0 == 5 as libc::c_int || (*f).type_0 == 6 as libc::c_int {
        fs_free((*f).value.ptr as *mut fieldset_t);
    } else if (*f).free_ != 0 {
        free((*f).value.ptr);
    }
}
pub unsafe extern "C" fn fs_free(mut fs: *mut fieldset_t) {
    if fs.is_null() {
        return;
    }
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < (*fs).len {
        let mut f: *mut field_t = &mut *((*fs).fields).as_mut_ptr().offset(i as isize)
            as *mut field_t;
        field_free(f);
        i += 1;
        i;
    }
    free(fs as *mut libc::c_void);
}
pub unsafe extern "C" fn fs_generate_fieldset_translation(
    mut t: *mut translation_t,
    mut avail: *mut fielddefset_t,
    mut req: *mut *const libc::c_char,
    mut reqlen: libc::c_int,
) {
    memset(
        t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<translation_t>() as libc::c_ulong,
    );
    if t.is_null() {
        log_fatal(
            b"fieldset\0" as *const u8 as *const libc::c_char,
            b"unable to allocate memory for translation\0" as *const u8
                as *const libc::c_char,
        );
    }
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < reqlen {
        let mut l: libc::c_int = fds_get_index_by_name(avail, *req.offset(i as isize));
        if l < 0 as libc::c_int {
            log_fatal(
                b"fieldset\0" as *const u8 as *const libc::c_char,
                b"specified field (%s) not available in selected probe module.\0"
                    as *const u8 as *const libc::c_char,
                *req.offset(i as isize),
            );
        }
        let fresh0 = (*t).len;
        (*t).len = (*t).len + 1;
        (*t).translation[fresh0 as usize] = l;
        i += 1;
        i;
    }
}
pub unsafe extern "C" fn fs_generate_full_fieldset_translation(
    mut t: *mut translation_t,
    mut avail: *mut fielddefset_t,
) {
    memset(
        t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<translation_t>() as libc::c_ulong,
    );
    if t.is_null() {
        log_fatal(
            b"fieldset\0" as *const u8 as *const libc::c_char,
            b"unable to allocate memory for translation\0" as *const u8
                as *const libc::c_char,
        );
    }
    (*t).len = (*avail).len;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < (*avail).len {
        (*t).translation[i as usize] = i;
        i += 1;
        i;
    }
}
pub unsafe extern "C" fn translate_fieldset(
    mut fs: *mut fieldset_t,
    mut t: *mut translation_t,
) -> *mut fieldset_t {
    let mut retv: *mut fieldset_t = fs_new_fieldset(0 as *mut fielddefset_t);
    if retv.is_null() {
        log_fatal(
            b"fieldset\0" as *const u8 as *const libc::c_char,
            b"unable to allocate space for translated field set\0" as *const u8
                as *const libc::c_char,
        );
    }
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < (*t).len {
        let mut o: libc::c_int = (*t).translation[i as usize];
        memcpy(
            &mut *((*retv).fields).as_mut_ptr().offset(i as isize) as *mut field_t
                as *mut libc::c_void,
            &mut *((*fs).fields).as_mut_ptr().offset(o as isize) as *mut field_t
                as *const libc::c_void,
            ::std::mem::size_of::<field_t>() as libc::c_ulong,
        );
        i += 1;
        i;
    }
    (*retv).len = (*t).len;
    return retv;
}
