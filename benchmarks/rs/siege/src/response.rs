use ::libc;
extern "C" {
    pub type HASH_T;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
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
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strncasecmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn new_hash() -> HASH;
    fn hash_add(this: HASH, key: *mut libc::c_char, value: *mut libc::c_void);
    fn hash_get(this: HASH, key: *mut libc::c_char) -> *mut libc::c_void;
    fn hash_destroy(this: HASH) -> HASH;
    fn strmatch(str1: *mut libc::c_char, str2: *mut libc::c_char) -> BOOLEAN;
    fn stristr(
        haystack: *const libc::c_char,
        needle: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn xstrdup(str: *const libc::c_char) -> *mut libc::c_char;
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    fn xcalloc(_: size_t, _: size_t) -> *mut libc::c_void;
    fn xfree(ptr: *mut libc::c_void);
    fn trim(str: *mut libc::c_char) -> *mut libc::c_char;
}
pub type size_t = libc::c_ulong;
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
pub type BOOLEAN = libc::c_uint;
pub const boolean_true: BOOLEAN = 1;
pub const boolean_false: BOOLEAN = 0;
pub type TYPE = libc::c_uint;
pub const NTLM: TYPE = 2;
pub const DIGEST: TYPE = 1;
pub const BASIC: TYPE = 0;
pub type HASH = *mut HASH_T;
pub type HTTP_CONN = libc::c_uint;
pub const METER: HTTP_CONN = 4;
pub const KEEPALIVE: HTTP_CONN = 2;
pub const CLOSE: HTTP_CONN = 1;
pub type HTTP_TE = libc::c_uint;
pub const TRAILER: HTTP_TE = 4;
pub const CHUNKED: HTTP_TE = 2;
pub const NONE: HTTP_TE = 1;
pub type HTTP_CE = libc::c_uint;
pub const BZIP2: HTTP_CE = 8;
pub const GZIP: HTTP_CE = 4;
pub const DEFLATE: HTTP_CE = 2;
pub const COMPRESS: HTTP_CE = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RESPONSE_T {
    pub headers: HASH,
    pub auth: C2RustUnnamed_0,
    pub cached: BOOLEAN,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub www: libc::c_int,
    pub proxy: libc::c_int,
    pub realm: C2RustUnnamed_3,
    pub challenge: C2RustUnnamed_2,
    pub type_0: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub www: TYPE,
    pub proxy: TYPE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub www: *mut libc::c_char,
    pub proxy: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub www: *mut libc::c_char,
    pub proxy: *mut libc::c_char,
}
pub type RESPONSE = *mut RESPONSE_T;
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
pub static mut RESPONSESIZE: size_t = ::std::mem::size_of::<RESPONSE_T>()
    as libc::c_ulong;
pub unsafe extern "C" fn new_response() -> RESPONSE {
    let mut this: RESPONSE = 0 as *mut RESPONSE_T;
    this = xcalloc(RESPONSESIZE, 1 as libc::c_int as size_t) as RESPONSE;
    (*this).headers = new_hash();
    (*this).auth.realm.www = 0 as *mut libc::c_char;
    (*this).auth.challenge.www = 0 as *mut libc::c_char;
    (*this).auth.realm.proxy = 0 as *mut libc::c_char;
    (*this).auth.challenge.www = 0 as *mut libc::c_char;
    return this;
}
pub unsafe extern "C" fn response_destroy(mut this: RESPONSE) -> RESPONSE {
    if !this.is_null() {
        (*this).headers = hash_destroy((*this).headers);
        xfree((*this).auth.realm.www as *mut libc::c_void);
        xfree((*this).auth.challenge.www as *mut libc::c_void);
        xfree((*this).auth.realm.proxy as *mut libc::c_void);
        xfree((*this).auth.challenge.proxy as *mut libc::c_void);
        xfree(this as *mut libc::c_void);
        this = 0 as RESPONSE;
    }
    return this;
}
pub unsafe extern "C" fn response_set_code(
    mut this: RESPONSE,
    mut line: *mut libc::c_char,
) -> BOOLEAN {
    let mut tmp: *mut libc::c_char = line;
    let mut arr: [libc::c_char; 32] = [0; 32];
    if strncasecmp(
        line,
        b"http\0" as *const u8 as *const libc::c_char,
        4 as libc::c_int as libc::c_ulong,
    ) == 0 as libc::c_int
    {
        let mut num: libc::c_int = atoi(tmp.offset(9 as libc::c_int as isize));
        if num > 1 as libc::c_int {
            memset(
                arr.as_mut_ptr() as *mut libc::c_void,
                '\0' as i32,
                ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
            );
            strncpy(arr.as_mut_ptr(), line, 8 as libc::c_int as libc::c_ulong);
            hash_add(
                (*this).headers,
                b"protocol\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                arr.as_mut_ptr() as *mut libc::c_void,
            );
            hash_add(
                (*this).headers,
                b"response-code\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                line.offset(9 as libc::c_int as isize) as *mut libc::c_void,
            );
            return boolean_true;
        }
    }
    return boolean_false;
}
pub unsafe extern "C" fn response_get_code(mut this: RESPONSE) -> libc::c_int {
    if this.is_null() || ((*this).headers).is_null()
        || (hash_get(
            (*this).headers,
            b"response-code\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) as *mut libc::c_char)
            .is_null()
    {
        return 418 as libc::c_int;
    }
    return atoi(
        hash_get(
            (*this).headers,
            b"response-code\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) as *mut libc::c_char,
    );
}
pub unsafe extern "C" fn response_get_protocol(mut this: RESPONSE) -> *mut libc::c_char {
    return (if (hash_get(
        (*this).headers,
        b"protocol\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_char)
        .is_null()
    {
        b"HTTP/1.1\0" as *const u8 as *const libc::c_char
    } else {
        hash_get(
            (*this).headers,
            b"protocol\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) as *mut libc::c_char as *const libc::c_char
    }) as *mut libc::c_char;
}
pub unsafe extern "C" fn response_success(mut this: RESPONSE) -> libc::c_int {
    if (hash_get(
        (*this).headers,
        b"response-code\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_char)
        .is_null()
    {
        return 0 as libc::c_int;
    }
    let mut code: libc::c_int = atoi(
        hash_get(
            (*this).headers,
            b"response-code\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) as *mut libc::c_char,
    );
    return if code < 400 as libc::c_int || code == 401 as libc::c_int
        || code == 407 as libc::c_int
    {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    };
}
pub unsafe extern "C" fn response_failure(mut this: RESPONSE) -> libc::c_int {
    if (hash_get(
        (*this).headers,
        b"response-code\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_char)
        .is_null()
    {
        return 1 as libc::c_int;
    }
    let mut code: libc::c_int = atoi(
        hash_get(
            (*this).headers,
            b"response-code\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) as *mut libc::c_char,
    );
    return if code >= 400 as libc::c_int && code != 401 as libc::c_int
        && code != 407 as libc::c_int
    {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    };
}
pub unsafe extern "C" fn response_set_from_cache(
    mut this: RESPONSE,
    mut cached: BOOLEAN,
) {
    (*this).cached = cached;
}
pub unsafe extern "C" fn response_get_from_cache(mut this: RESPONSE) -> BOOLEAN {
    return (*this).cached;
}
pub unsafe extern "C" fn response_set_content_type(
    mut this: RESPONSE,
    mut line: *mut libc::c_char,
) -> BOOLEAN {
    let mut type_0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut set: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut aid: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut res: BOOLEAN = boolean_false;
    if !(strstr(line, b";\0" as *const u8 as *const libc::c_char)).is_null() {
        ptr = line
            .offset(
                (strlen(b"content-type\0" as *const u8 as *const libc::c_char))
                    .wrapping_add(2 as libc::c_int as libc::c_ulong) as isize,
            );
        type_0 = strtok_r(ptr, b";\0" as *const u8 as *const libc::c_char, &mut aid);
        if !type_0.is_null() {
            hash_add(
                (*this).headers,
                b"content-type\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                type_0 as *mut libc::c_void,
            );
            res = boolean_true;
        }
        set = stristr(aid, b"charset=\0" as *const u8 as *const libc::c_char);
        if !set.is_null() && strlen(set) > 8 as libc::c_int as libc::c_ulong {
            hash_add(
                (*this).headers,
                b"charset\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                set.offset(8 as libc::c_int as isize) as *mut libc::c_void,
            );
        }
    } else {
        hash_add(
            (*this).headers,
            b"content-type\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            line
                .offset(
                    (strlen(b"content-type\0" as *const u8 as *const libc::c_char))
                        .wrapping_add(2 as libc::c_int as libc::c_ulong) as isize,
                ) as *mut libc::c_void,
        );
        res = boolean_true;
    }
    return res;
}
pub unsafe extern "C" fn response_get_content_type(
    mut this: RESPONSE,
) -> *mut libc::c_char {
    return (if (hash_get(
        (*this).headers,
        b"content-type\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_char)
        .is_null()
    {
        b"unknown\0" as *const u8 as *const libc::c_char
    } else {
        hash_get(
            (*this).headers,
            b"content-type\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) as *mut libc::c_char as *const libc::c_char
    }) as *mut libc::c_char;
}
pub unsafe extern "C" fn response_get_charset(mut this: RESPONSE) -> *mut libc::c_char {
    if (hash_get(
        (*this).headers,
        b"charset\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_char)
        .is_null()
    {
        hash_add(
            (*this).headers,
            b"charset\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"iso-8859-1\0" as *const u8 as *const libc::c_char as *mut libc::c_void,
        );
    }
    return hash_get(
        (*this).headers,
        b"charset\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_char;
}
pub unsafe extern "C" fn response_set_content_length(
    mut this: RESPONSE,
    mut line: *mut libc::c_char,
) -> BOOLEAN {
    let mut tmp: *mut libc::c_char = line;
    if strncasecmp(
        line,
        b"content-length\0" as *const u8 as *const libc::c_char,
        strlen(b"content-length\0" as *const u8 as *const libc::c_char),
    ) == 0 as libc::c_int
    {
        let mut num: libc::c_int = atoi(
            tmp
                .offset(
                    (strlen(b"content-length\0" as *const u8 as *const libc::c_char))
                        .wrapping_add(2 as libc::c_int as libc::c_ulong) as isize,
                ),
        );
        if num > 1 as libc::c_int {
            hash_add(
                (*this).headers,
                b"content-length\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                line
                    .offset(
                        (strlen(b"content-length\0" as *const u8 as *const libc::c_char))
                            .wrapping_add(2 as libc::c_int as libc::c_ulong) as isize,
                    ) as *mut libc::c_void,
            );
            return boolean_true;
        }
    }
    return boolean_false;
}
pub unsafe extern "C" fn response_get_content_length(mut this: RESPONSE) -> libc::c_int {
    return __int_value(
        this,
        b"content-length\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
}
pub unsafe extern "C" fn response_set_content_encoding(
    mut this: RESPONSE,
    mut line: *mut libc::c_char,
) -> BOOLEAN {
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: [libc::c_char; 128] = [0; 128];
    if strncasecmp(
        line,
        b"content-encoding\0" as *const u8 as *const libc::c_char,
        strlen(b"content-encoding\0" as *const u8 as *const libc::c_char),
    ) == 0 as libc::c_int
    {
        memset(
            tmp.as_mut_ptr() as *mut libc::c_void,
            '\0' as i32,
            ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
        );
        ptr = line
            .offset(
                (strlen(b"content-encoding\0" as *const u8 as *const libc::c_char))
                    .wrapping_add(2 as libc::c_int as libc::c_ulong) as isize,
            );
        if strmatch(
            ptr,
            b"gzip\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) as u64 != 0
        {
            snprintf(
                tmp.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
                b"%d\0" as *const u8 as *const libc::c_char,
                GZIP as libc::c_int,
            );
            hash_add(
                (*this).headers,
                b"content-encoding\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                tmp.as_mut_ptr() as *mut libc::c_void,
            );
            return boolean_true;
        }
        if strmatch(
            ptr,
            b"deflate\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) as u64 != 0
        {
            snprintf(
                tmp.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
                b"%d\0" as *const u8 as *const libc::c_char,
                DEFLATE as libc::c_int,
            );
            hash_add(
                (*this).headers,
                b"content-encoding\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                tmp.as_mut_ptr() as *mut libc::c_void,
            );
            return boolean_true;
        }
    }
    return boolean_false;
}
pub unsafe extern "C" fn response_get_content_encoding(mut this: RESPONSE) -> HTTP_CE {
    return __int_value(
        this,
        b"content-encoding\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    ) as HTTP_CE;
}
pub unsafe extern "C" fn response_set_transfer_encoding(
    mut this: RESPONSE,
    mut line: *mut libc::c_char,
) -> BOOLEAN {
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: [libc::c_char; 128] = [0; 128];
    if strncasecmp(
        line,
        b"transfer-encoding\0" as *const u8 as *const libc::c_char,
        strlen(b"transfer-encoding\0" as *const u8 as *const libc::c_char),
    ) == 0 as libc::c_int
    {
        memset(
            tmp.as_mut_ptr() as *mut libc::c_void,
            '\0' as i32,
            ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
        );
        ptr = line
            .offset(
                (strlen(b"transfer-encoding\0" as *const u8 as *const libc::c_char))
                    .wrapping_add(2 as libc::c_int as libc::c_ulong) as isize,
            );
        ptr = trim(ptr);
        if strmatch(
            ptr,
            b"chunked\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) as u64 != 0
        {
            snprintf(
                tmp.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
                b"%d\0" as *const u8 as *const libc::c_char,
                CHUNKED as libc::c_int,
            );
        } else if strmatch(
            ptr,
            b"trailer\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) as u64 != 0
        {
            snprintf(
                tmp.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
                b"%d\0" as *const u8 as *const libc::c_char,
                TRAILER as libc::c_int,
            );
        } else {
            snprintf(
                tmp.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
                b"%d\0" as *const u8 as *const libc::c_char,
                NONE as libc::c_int,
            );
        }
        hash_add(
            (*this).headers,
            b"transfer-encoding\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            tmp.as_mut_ptr() as *mut libc::c_void,
        );
        return boolean_true;
    }
    return boolean_false;
}
pub unsafe extern "C" fn response_get_transfer_encoding(mut this: RESPONSE) -> HTTP_TE {
    return __int_value(
        this,
        b"transfer-encoding\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        NONE as libc::c_int,
    ) as HTTP_TE;
}
pub unsafe extern "C" fn response_set_location(
    mut this: RESPONSE,
    mut line: *mut libc::c_char,
) -> BOOLEAN {
    let mut len: libc::c_int = 0 as libc::c_int;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    if strncasecmp(
        line,
        b"location\0" as *const u8 as *const libc::c_char,
        strlen(b"location\0" as *const u8 as *const libc::c_char),
    ) == 0 as libc::c_int
    {
        len = strlen(line) as libc::c_int;
        tmp = xmalloc(len as size_t) as *mut libc::c_char;
        memset(tmp as *mut libc::c_void, '\0' as i32, len as libc::c_ulong);
        memmove(
            tmp as *mut libc::c_void,
            line.offset(10 as libc::c_int as isize) as *const libc::c_void,
            (len - 9 as libc::c_int) as libc::c_ulong,
        );
        *tmp.offset((len - 10 as libc::c_int) as isize) = '\0' as i32 as libc::c_char;
        hash_add(
            (*this).headers,
            b"location\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            tmp as *mut libc::c_void,
        );
        hash_add(
            (*this).headers,
            b"redirect\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"true\0" as *const u8 as *const libc::c_char as *mut libc::c_void,
        );
        xfree(tmp as *mut libc::c_void);
    }
    if strncasecmp(
        line,
        b"content-location\0" as *const u8 as *const libc::c_char,
        strlen(b"content-location\0" as *const u8 as *const libc::c_char),
    ) == 0 as libc::c_int
    {
        len = strlen(line) as libc::c_int;
        tmp = xmalloc(len as size_t) as *mut libc::c_char;
        memset(tmp as *mut libc::c_void, '\0' as i32, len as libc::c_ulong);
        memmove(
            tmp as *mut libc::c_void,
            line.offset(18 as libc::c_int as isize) as *const libc::c_void,
            (len - 17 as libc::c_int) as libc::c_ulong,
        );
        *tmp.offset((len - 18 as libc::c_int) as isize) = '\0' as i32 as libc::c_char;
        hash_add(
            (*this).headers,
            b"location\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            tmp as *mut libc::c_void,
        );
        hash_add(
            (*this).headers,
            b"redirect\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"true\0" as *const u8 as *const libc::c_char as *mut libc::c_void,
        );
        xfree(tmp as *mut libc::c_void);
    }
    return __boolean_value(
        this,
        b"redirect\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        boolean_false,
    );
}
pub unsafe extern "C" fn response_get_location(mut this: RESPONSE) -> *mut libc::c_char {
    return hash_get(
        (*this).headers,
        b"location\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_char;
}
pub unsafe extern "C" fn response_get_redirect(mut this: RESPONSE) -> BOOLEAN {
    return __boolean_value(
        this,
        b"redirect\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        boolean_false,
    );
}
pub unsafe extern "C" fn response_set_connection(
    mut this: RESPONSE,
    mut line: *mut libc::c_char,
) -> BOOLEAN {
    let mut tmp: [libc::c_char; 128] = [0; 128];
    if strncasecmp(
        line,
        b"connection\0" as *const u8 as *const libc::c_char,
        strlen(b"connection\0" as *const u8 as *const libc::c_char),
    ) == 0 as libc::c_int
    {
        memset(
            tmp.as_mut_ptr() as *mut libc::c_void,
            '\0' as i32,
            128 as libc::c_int as libc::c_ulong,
        );
        if strncasecmp(
            line.offset(12 as libc::c_int as isize),
            b"keep-alive\0" as *const u8 as *const libc::c_char,
            10 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            snprintf(
                tmp.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
                b"%d\0" as *const u8 as *const libc::c_char,
                KEEPALIVE as libc::c_int,
            );
        } else {
            snprintf(
                tmp.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
                b"%d\0" as *const u8 as *const libc::c_char,
                CLOSE as libc::c_int,
            );
        }
        hash_add(
            (*this).headers,
            b"connection\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            tmp.as_mut_ptr() as *mut libc::c_void,
        );
        return boolean_true;
    }
    return boolean_false;
}
pub unsafe extern "C" fn response_get_connection(mut this: RESPONSE) -> HTTP_CONN {
    return __int_value(
        this,
        b"connection\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        CLOSE as libc::c_int,
    ) as HTTP_CONN;
}
pub unsafe extern "C" fn response_set_keepalive(
    mut this: RESPONSE,
    mut line: *mut libc::c_char,
) -> BOOLEAN {
    let mut _tmp: [libc::c_char; 1024] = [0; 1024];
    let mut tmp: *mut libc::c_char = _tmp.as_mut_ptr();
    let mut option: *mut libc::c_char = b"\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    let mut value: *mut libc::c_char = b"\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    let mut newline: *mut libc::c_char = line;
    let mut res: BOOLEAN = boolean_false;
    loop {
        tmp = __parse_pair(&mut newline);
        if tmp.is_null() {
            break;
        }
        option = tmp;
        while *tmp as libc::c_int != 0
            && *(*__ctype_b_loc())
                .offset(*tmp as libc::c_int as libc::c_uchar as libc::c_int as isize)
                as libc::c_int & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                == 0
            && !('=' as i32 == *tmp as libc::c_int || ':' as i32 == *tmp as libc::c_int)
        {
            tmp = tmp.offset(1);
            tmp;
        }
        let fresh0 = tmp;
        tmp = tmp.offset(1);
        *fresh0 = 0 as libc::c_int as libc::c_char;
        while *(*__ctype_b_loc())
            .offset(*tmp as libc::c_int as libc::c_uchar as libc::c_int as isize)
            as libc::c_int & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
            != 0
            || ('=' as i32 == *tmp as libc::c_int || ':' as i32 == *tmp as libc::c_int)
        {
            tmp = tmp.offset(1);
            tmp;
        }
        value = tmp;
        while *tmp != 0 {
            tmp = tmp.offset(1);
            tmp;
        }
        if strncasecmp(
            option,
            b"timeout\0" as *const u8 as *const libc::c_char,
            7 as libc::c_int as libc::c_ulong,
        ) == 0
        {
            if value.is_null() {
                hash_add(
                    (*this).headers,
                    b"keepalive-timeout\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    b"15\0" as *const u8 as *const libc::c_char as *mut libc::c_void,
                );
            } else {
                let mut num: libc::c_int = atoi(value);
                if num > 0 as libc::c_int {
                    hash_add(
                        (*this).headers,
                        b"keepalive-timeout\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        value as *mut libc::c_void,
                    );
                }
            }
            res = boolean_true;
        }
        if strncasecmp(
            option,
            b"max\0" as *const u8 as *const libc::c_char,
            3 as libc::c_int as libc::c_ulong,
        ) == 0
        {
            if value.is_null() {
                hash_add(
                    (*this).headers,
                    b"keepalive-max\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    b"15\0" as *const u8 as *const libc::c_char as *mut libc::c_void,
                );
            } else {
                let mut num_0: libc::c_int = atoi(value);
                if num_0 > 0 as libc::c_int {
                    hash_add(
                        (*this).headers,
                        b"keepalive-max\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        value as *mut libc::c_void,
                    );
                }
            }
            res = boolean_true;
        }
    }
    return res;
}
pub unsafe extern "C" fn response_get_keepalive_timeout(
    mut this: RESPONSE,
) -> libc::c_int {
    return __int_value(
        this,
        b"keepalive-timeout\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        15 as libc::c_int,
    );
}
pub unsafe extern "C" fn response_get_keepalive_max(mut this: RESPONSE) -> libc::c_int {
    return __int_value(
        this,
        b"keepalive-max\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        5 as libc::c_int,
    );
}
pub unsafe extern "C" fn response_set_last_modified(
    mut this: RESPONSE,
    mut line: *mut libc::c_char,
) -> BOOLEAN {
    let mut len: libc::c_int = 0 as libc::c_int;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    if strncasecmp(
        line,
        b"last-modified\0" as *const u8 as *const libc::c_char,
        strlen(b"last-modified\0" as *const u8 as *const libc::c_char),
    ) == 0 as libc::c_int
    {
        len = strlen(line) as libc::c_int;
        tmp = xmalloc(len as size_t) as *mut libc::c_char;
        memset(tmp as *mut libc::c_void, '\0' as i32, len as libc::c_ulong);
        memcpy(
            tmp as *mut libc::c_void,
            line.offset(15 as libc::c_int as isize) as *const libc::c_void,
            (len - 14 as libc::c_int) as libc::c_ulong,
        );
        *tmp.offset((len - 15 as libc::c_int) as isize) = '\0' as i32 as libc::c_char;
        hash_add(
            (*this).headers,
            b"last-modified\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            tmp as *mut libc::c_void,
        );
        xfree(tmp as *mut libc::c_void);
        return boolean_true;
    }
    return boolean_false;
}
pub unsafe extern "C" fn response_get_last_modified(
    mut this: RESPONSE,
) -> *mut libc::c_char {
    return hash_get(
        (*this).headers,
        b"last-modified\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_char;
}
pub unsafe extern "C" fn response_set_etag(
    mut this: RESPONSE,
    mut line: *mut libc::c_char,
) -> BOOLEAN {
    let mut len: libc::c_int = 0 as libc::c_int;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    if strncasecmp(
        line,
        b"etag\0" as *const u8 as *const libc::c_char,
        strlen(b"etag\0" as *const u8 as *const libc::c_char),
    ) == 0 as libc::c_int
    {
        len = strlen(line) as libc::c_int;
        tmp = xmalloc(len as size_t) as *mut libc::c_char;
        memset(tmp as *mut libc::c_void, '\0' as i32, len as libc::c_ulong);
        memcpy(
            tmp as *mut libc::c_void,
            line.offset(6 as libc::c_int as isize) as *const libc::c_void,
            (len - 5 as libc::c_int) as libc::c_ulong,
        );
        *tmp.offset((len - 6 as libc::c_int) as isize) = '\0' as i32 as libc::c_char;
        hash_add(
            (*this).headers,
            b"etag\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            __dequote(tmp) as *mut libc::c_void,
        );
        xfree(tmp as *mut libc::c_void);
        return boolean_true;
    }
    return boolean_false;
}
pub unsafe extern "C" fn response_get_etag(mut this: RESPONSE) -> *mut libc::c_char {
    return hash_get(
        (*this).headers,
        b"etag\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_char;
}
pub unsafe extern "C" fn response_set_www_authenticate(
    mut this: RESPONSE,
    mut line: *mut libc::c_char,
) -> BOOLEAN {
    let mut _tmp: [libc::c_char; 1024] = [0; 1024];
    let mut tmp: *mut libc::c_char = _tmp.as_mut_ptr();
    let mut option: *mut libc::c_char = b"\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    let mut value: *mut libc::c_char = b"\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    let mut newline: *mut libc::c_char = line;
    if strncasecmp(
        line,
        b"www-authenticate\0" as *const u8 as *const libc::c_char,
        strlen(b"www-authenticate\0" as *const u8 as *const libc::c_char),
    ) == 0 as libc::c_int
    {
        if strncasecmp(
            line.offset(18 as libc::c_int as isize),
            b"digest\0" as *const u8 as *const libc::c_char,
            6 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            newline = newline.offset(24 as libc::c_int as isize);
            (*this).auth.type_0.www = DIGEST;
            (*this)
                .auth
                .challenge
                .www = xstrdup(line.offset(18 as libc::c_int as isize));
        } else if strncasecmp(
            line.offset(18 as libc::c_int as isize),
            b"ntlm\0" as *const u8 as *const libc::c_char,
            4 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            newline = newline.offset(22 as libc::c_int as isize);
            (*this).auth.type_0.www = NTLM;
            (*this)
                .auth
                .challenge
                .www = xstrdup(line.offset(18 as libc::c_int as isize));
        } else if (*this).auth.type_0.www as libc::c_uint
            != DIGEST as libc::c_int as libc::c_uint
            && (*this).auth.type_0.www as libc::c_uint
                != NTLM as libc::c_int as libc::c_uint
        {
            newline = newline.offset(23 as libc::c_int as isize);
            (*this).auth.type_0.www = BASIC;
        }
        loop {
            tmp = __parse_pair(&mut newline);
            if tmp.is_null() {
                break;
            }
            option = tmp;
            while *tmp as libc::c_int != 0
                && *(*__ctype_b_loc())
                    .offset(*tmp as libc::c_int as libc::c_uchar as libc::c_int as isize)
                    as libc::c_int
                    & _ISspace as libc::c_int as libc::c_ushort as libc::c_int == 0
                && !('=' as i32 == *tmp as libc::c_int
                    || ':' as i32 == *tmp as libc::c_int)
            {
                tmp = tmp.offset(1);
                tmp;
            }
            let fresh1 = tmp;
            tmp = tmp.offset(1);
            *fresh1 = '\0' as i32 as libc::c_char;
            while *(*__ctype_b_loc())
                .offset(*tmp as libc::c_int as libc::c_uchar as libc::c_int as isize)
                as libc::c_int & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                != 0
                || ('=' as i32 == *tmp as libc::c_int
                    || ':' as i32 == *tmp as libc::c_int)
            {
                tmp = tmp.offset(1);
                tmp;
            }
            value = tmp;
            while *tmp != 0 {
                tmp = tmp.offset(1);
                tmp;
            }
            if strncasecmp(
                option,
                b"realm\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int as libc::c_ulong,
            ) == 0
            {
                if !value.is_null() {
                    (*this).auth.realm.www = xstrdup(__dequote(value));
                } else {
                    (*this)
                        .auth
                        .realm
                        .www = xstrdup(b"\0" as *const u8 as *const libc::c_char);
                }
            }
        }
    }
    return boolean_true;
}
pub unsafe extern "C" fn response_get_www_auth_type(mut this: RESPONSE) -> TYPE {
    return (*this).auth.type_0.www;
}
pub unsafe extern "C" fn response_get_www_auth_challenge(
    mut this: RESPONSE,
) -> *mut libc::c_char {
    return (*this).auth.challenge.www;
}
pub unsafe extern "C" fn response_get_www_auth_realm(
    mut this: RESPONSE,
) -> *mut libc::c_char {
    return (*this).auth.realm.www;
}
pub unsafe extern "C" fn response_set_proxy_authenticate(
    mut this: RESPONSE,
    mut line: *mut libc::c_char,
) -> BOOLEAN {
    let mut _tmp: [libc::c_char; 1024] = [0; 1024];
    let mut tmp: *mut libc::c_char = _tmp.as_mut_ptr();
    let mut option: *mut libc::c_char = b"\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    let mut value: *mut libc::c_char = b"\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    let mut newline: *mut libc::c_char = line;
    if strncasecmp(
        line,
        b"proxy-authenticate\0" as *const u8 as *const libc::c_char,
        strlen(b"proxy-authenticate\0" as *const u8 as *const libc::c_char),
    ) == 0 as libc::c_int
    {
        if strncasecmp(
            line.offset(20 as libc::c_int as isize),
            b"digest\0" as *const u8 as *const libc::c_char,
            6 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            newline = newline.offset(26 as libc::c_int as isize);
            (*this).auth.type_0.proxy = DIGEST;
            (*this)
                .auth
                .challenge
                .proxy = xstrdup(line.offset(20 as libc::c_int as isize));
        } else {
            newline = newline.offset(25 as libc::c_int as isize);
            (*this).auth.type_0.proxy = BASIC;
        }
        loop {
            tmp = __parse_pair(&mut newline);
            if tmp.is_null() {
                break;
            }
            option = tmp;
            while *tmp as libc::c_int != 0
                && *(*__ctype_b_loc())
                    .offset(*tmp as libc::c_int as libc::c_uchar as libc::c_int as isize)
                    as libc::c_int
                    & _ISspace as libc::c_int as libc::c_ushort as libc::c_int == 0
                && !('=' as i32 == *tmp as libc::c_int
                    || ':' as i32 == *tmp as libc::c_int)
            {
                tmp = tmp.offset(1);
                tmp;
            }
            let fresh2 = tmp;
            tmp = tmp.offset(1);
            *fresh2 = '\0' as i32 as libc::c_char;
            while *(*__ctype_b_loc())
                .offset(*tmp as libc::c_int as libc::c_uchar as libc::c_int as isize)
                as libc::c_int & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                != 0
                || ('=' as i32 == *tmp as libc::c_int
                    || ':' as i32 == *tmp as libc::c_int)
            {
                tmp = tmp.offset(1);
                tmp;
            }
            value = tmp;
            while *tmp != 0 {
                tmp = tmp.offset(1);
                tmp;
            }
            if strncasecmp(
                option,
                b"realm\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int as libc::c_ulong,
            ) == 0
            {
                if !value.is_null() {
                    (*this).auth.realm.proxy = xstrdup(__dequote(value));
                } else {
                    (*this)
                        .auth
                        .realm
                        .proxy = xstrdup(b"\0" as *const u8 as *const libc::c_char);
                }
            }
        }
    }
    return boolean_true;
}
pub unsafe extern "C" fn response_get_proxy_auth_type(mut this: RESPONSE) -> TYPE {
    return (*this).auth.type_0.proxy;
}
pub unsafe extern "C" fn response_get_proxy_auth_challenge(
    mut this: RESPONSE,
) -> *mut libc::c_char {
    return (*this).auth.challenge.proxy;
}
pub unsafe extern "C" fn response_get_proxy_auth_realm(
    mut this: RESPONSE,
) -> *mut libc::c_char {
    return (*this).auth.realm.proxy;
}
unsafe extern "C" fn __int_value(
    mut this: RESPONSE,
    mut key: *mut libc::c_char,
    mut def: libc::c_int,
) -> libc::c_int {
    let mut num: libc::c_int = -(1 as libc::c_int);
    if !(hash_get((*this).headers, key) as *mut libc::c_char).is_null() {
        num = atoi(hash_get((*this).headers, key) as *mut libc::c_char);
    }
    return if num > 0 as libc::c_int { num } else { def };
}
unsafe extern "C" fn __boolean_value(
    mut this: RESPONSE,
    mut key: *mut libc::c_char,
    mut def: BOOLEAN,
) -> BOOLEAN {
    let mut res: BOOLEAN = def;
    let mut b: *mut libc::c_char = hash_get((*this).headers, key) as *mut libc::c_char;
    if b.is_null() {
        res = def;
    } else if strmatch(
        b,
        b"true\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as u64 != 0
    {
        res = boolean_true;
    } else if strmatch(
        b,
        b"false\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as u64 != 0
    {
        res = boolean_false;
    } else {
        res = def;
    }
    return res;
}
unsafe extern "C" fn __parse_pair(mut str: *mut *mut libc::c_char) -> *mut libc::c_char {
    let mut okay: libc::c_int = 0 as libc::c_int;
    let mut p: *mut libc::c_char = *str;
    let mut pair: *mut libc::c_char = 0 as *mut libc::c_char;
    if str.is_null() || (*str).is_null() {
        return 0 as *mut libc::c_char;
    }
    while *p as libc::c_int != 0 && *p as libc::c_int != ' ' as i32 {
        p = p.offset(1);
        p;
    }
    let fresh3 = p;
    p = p.offset(1);
    *fresh3 = 0 as libc::c_int as libc::c_char;
    if *p == 0 {
        *str = p;
        return 0 as *mut libc::c_char;
    }
    pair = p;
    while *p as libc::c_int != 0 && *p as libc::c_int != ';' as i32
        && *p as libc::c_int != ',' as i32
    {
        if *p == 0 {
            *str = p;
            return 0 as *mut libc::c_char;
        }
        if *p as libc::c_int == '=' as i32 {
            okay = 1 as libc::c_int;
        }
        p = p.offset(1);
        p;
    }
    let fresh4 = p;
    p = p.offset(1);
    *fresh4 = 0 as libc::c_int as libc::c_char;
    *str = p;
    return if okay != 0 { pair } else { 0 as *mut libc::c_char };
}
unsafe extern "C" fn __rquote(mut str: *mut libc::c_char) -> *mut libc::c_char {
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: libc::c_int = 0;
    len = strlen(str) as libc::c_int;
    ptr = str.offset(len as isize).offset(-(1 as libc::c_int as isize));
    while ptr >= str
        && (*ptr as libc::c_int == '"' as i32 || *ptr as libc::c_int == '\'' as i32)
    {
        ptr = ptr.offset(-1);
        ptr;
    }
    *ptr.offset(1 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
    return str;
}
unsafe extern "C" fn __lquote(mut str: *mut libc::c_char) -> *mut libc::c_char {
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: libc::c_int = 0;
    ptr = str;
    while *ptr as libc::c_int != 0
        && (*ptr as libc::c_int == '"' as i32 || *ptr as libc::c_int == '\'' as i32)
    {
        ptr = ptr.offset(1);
        ptr;
    }
    len = strlen(ptr) as libc::c_int;
    memmove(
        str as *mut libc::c_void,
        ptr as *const libc::c_void,
        (len + 1 as libc::c_int) as libc::c_ulong,
    );
    return str;
}
unsafe extern "C" fn __dequote(mut str: *mut libc::c_char) -> *mut libc::c_char {
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    ptr = __rquote(str);
    str = __lquote(ptr);
    return str;
}
