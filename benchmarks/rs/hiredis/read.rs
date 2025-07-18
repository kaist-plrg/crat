use ::libc;
extern "C" {
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memchr(
        _: *const libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_double;
    fn strcasecmp(__s1: *const libc::c_char, __s2: *const libc::c_char) -> libc::c_int;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    static mut hiredisAllocFns: hiredisAllocFuncs;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn sdsempty() -> sds;
    fn sdsfree(s: sds);
    fn sdscatlen(s: sds, t: *const libc::c_void, len: size_t) -> sds;
    fn sdsrange(s: sds, start: ssize_t, end: ssize_t) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __ssize_t = libc::c_long;
pub type ssize_t = __ssize_t;
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
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type uintptr_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hiredisAllocFuncs {
    pub mallocFn: Option::<unsafe extern "C" fn(size_t) -> *mut libc::c_void>,
    pub callocFn: Option::<unsafe extern "C" fn(size_t, size_t) -> *mut libc::c_void>,
    pub reallocFn: Option::<
        unsafe extern "C" fn(*mut libc::c_void, size_t) -> *mut libc::c_void,
    >,
    pub strdupFn: Option::<
        unsafe extern "C" fn(*const libc::c_char) -> *mut libc::c_char,
    >,
    pub freeFn: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct redisReadTask {
    pub type_0: libc::c_int,
    pub elements: libc::c_longlong,
    pub idx: libc::c_int,
    pub obj: *mut libc::c_void,
    pub parent: *mut redisReadTask,
    pub privdata: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct redisReplyObjectFunctions {
    pub createString: Option::<
        unsafe extern "C" fn(
            *const redisReadTask,
            *mut libc::c_char,
            size_t,
        ) -> *mut libc::c_void,
    >,
    pub createArray: Option::<
        unsafe extern "C" fn(*const redisReadTask, size_t) -> *mut libc::c_void,
    >,
    pub createInteger: Option::<
        unsafe extern "C" fn(*const redisReadTask, libc::c_longlong) -> *mut libc::c_void,
    >,
    pub createDouble: Option::<
        unsafe extern "C" fn(
            *const redisReadTask,
            libc::c_double,
            *mut libc::c_char,
            size_t,
        ) -> *mut libc::c_void,
    >,
    pub createNil: Option::<
        unsafe extern "C" fn(*const redisReadTask) -> *mut libc::c_void,
    >,
    pub createBool: Option::<
        unsafe extern "C" fn(*const redisReadTask, libc::c_int) -> *mut libc::c_void,
    >,
    pub freeObject: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct redisReader {
    pub err: libc::c_int,
    pub errstr: [libc::c_char; 128],
    pub buf: *mut libc::c_char,
    pub pos: size_t,
    pub len: size_t,
    pub maxbuf: size_t,
    pub maxelements: libc::c_longlong,
    pub task: *mut *mut redisReadTask,
    pub tasks: libc::c_int,
    pub ridx: libc::c_int,
    pub reply: *mut libc::c_void,
    pub fn_0: *mut redisReplyObjectFunctions,
    pub privdata: *mut libc::c_void,
}
pub type sds = *mut libc::c_char;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sdshdr64 {
    pub len: uint64_t,
    pub alloc: uint64_t,
    pub flags: libc::c_uchar,
    pub buf: [libc::c_char; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sdshdr32 {
    pub len: uint32_t,
    pub alloc: uint32_t,
    pub flags: libc::c_uchar,
    pub buf: [libc::c_char; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sdshdr16 {
    pub len: uint16_t,
    pub alloc: uint16_t,
    pub flags: libc::c_uchar,
    pub buf: [libc::c_char; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sdshdr8 {
    pub len: uint8_t,
    pub alloc: uint8_t,
    pub flags: libc::c_uchar,
    pub buf: [libc::c_char; 0],
}
#[inline]
unsafe extern "C" fn hi_free(mut ptr: *mut libc::c_void) {
    (hiredisAllocFns.freeFn).unwrap()(ptr);
}
#[inline]
unsafe extern "C" fn hi_calloc(
    mut nmemb: size_t,
    mut size: size_t,
) -> *mut libc::c_void {
    if (18446744073709551615 as libc::c_ulong).wrapping_div(size) < nmemb {
        return 0 as *mut libc::c_void;
    }
    return (hiredisAllocFns.callocFn).unwrap()(nmemb, size);
}
#[inline]
unsafe extern "C" fn hi_realloc(
    mut ptr: *mut libc::c_void,
    mut size: size_t,
) -> *mut libc::c_void {
    return (hiredisAllocFns.reallocFn).unwrap()(ptr, size);
}
#[inline]
unsafe extern "C" fn sdslen(s: sds) -> size_t {
    let mut flags: libc::c_uchar = *s.offset(-(1 as libc::c_int) as isize)
        as libc::c_uchar;
    match flags as libc::c_int & 7 as libc::c_int {
        0 => return (flags as libc::c_int >> 3 as libc::c_int) as size_t,
        1 => {
            return (*(s
                .offset(-(::std::mem::size_of::<sdshdr8>() as libc::c_ulong as isize))
                as *mut sdshdr8))
                .len as size_t;
        }
        2 => {
            return (*(s
                .offset(-(::std::mem::size_of::<sdshdr16>() as libc::c_ulong as isize))
                as *mut sdshdr16))
                .len as size_t;
        }
        3 => {
            return (*(s
                .offset(-(::std::mem::size_of::<sdshdr32>() as libc::c_ulong as isize))
                as *mut sdshdr32))
                .len as size_t;
        }
        4 => {
            return (*(s
                .offset(-(::std::mem::size_of::<sdshdr64>() as libc::c_ulong as isize))
                as *mut sdshdr64))
                .len;
        }
        _ => {}
    }
    return 0 as libc::c_int as size_t;
}
#[inline]
unsafe extern "C" fn sdsavail(s: sds) -> size_t {
    let mut flags: libc::c_uchar = *s.offset(-(1 as libc::c_int) as isize)
        as libc::c_uchar;
    match flags as libc::c_int & 7 as libc::c_int {
        0 => return 0 as libc::c_int as size_t,
        1 => {
            let mut sh: *mut sdshdr8 = s
                .offset(-(::std::mem::size_of::<sdshdr8>() as libc::c_ulong as isize))
                as *mut sdshdr8;
            return ((*sh).alloc as libc::c_int - (*sh).len as libc::c_int) as size_t;
        }
        2 => {
            let mut sh_0: *mut sdshdr16 = s
                .offset(-(::std::mem::size_of::<sdshdr16>() as libc::c_ulong as isize))
                as *mut sdshdr16;
            return ((*sh_0).alloc as libc::c_int - (*sh_0).len as libc::c_int) as size_t;
        }
        3 => {
            let mut sh_1: *mut sdshdr32 = s
                .offset(-(::std::mem::size_of::<sdshdr32>() as libc::c_ulong as isize))
                as *mut sdshdr32;
            return ((*sh_1).alloc).wrapping_sub((*sh_1).len) as size_t;
        }
        4 => {
            let mut sh_2: *mut sdshdr64 = s
                .offset(-(::std::mem::size_of::<sdshdr64>() as libc::c_ulong as isize))
                as *mut sdshdr64;
            return ((*sh_2).alloc).wrapping_sub((*sh_2).len);
        }
        _ => {}
    }
    return 0 as libc::c_int as size_t;
}
unsafe extern "C" fn __redisReaderSetError(
    mut r: *mut redisReader,
    mut type_0: libc::c_int,
    mut str: *const libc::c_char,
) {
    let mut len: size_t = 0;
    if !((*r).reply).is_null() && !((*r).fn_0).is_null()
        && ((*(*r).fn_0).freeObject).is_some()
    {
        ((*(*r).fn_0).freeObject).unwrap()((*r).reply);
        (*r).reply = 0 as *mut libc::c_void;
    }
    sdsfree((*r).buf);
    (*r).buf = 0 as *mut libc::c_char;
    (*r).len = 0 as libc::c_int as size_t;
    (*r).pos = (*r).len;
    (*r).ridx = -(1 as libc::c_int);
    (*r).err = type_0;
    len = strlen(str);
    len = if len
        < (::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
    {
        len
    } else {
        (::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
    };
    memcpy(
        ((*r).errstr).as_mut_ptr() as *mut libc::c_void,
        str as *const libc::c_void,
        len,
    );
    (*r).errstr[len as usize] = '\0' as i32 as libc::c_char;
}
unsafe extern "C" fn chrtos(
    mut buf: *mut libc::c_char,
    mut size: size_t,
    mut byte: libc::c_char,
) -> size_t {
    let mut len: size_t = 0 as libc::c_int as size_t;
    match byte as libc::c_int {
        92 | 34 => {
            len = snprintf(
                buf,
                size,
                b"\"\\%c\"\0" as *const u8 as *const libc::c_char,
                byte as libc::c_int,
            ) as size_t;
        }
        10 => {
            len = snprintf(buf, size, b"\"\\n\"\0" as *const u8 as *const libc::c_char)
                as size_t;
        }
        13 => {
            len = snprintf(buf, size, b"\"\\r\"\0" as *const u8 as *const libc::c_char)
                as size_t;
        }
        9 => {
            len = snprintf(buf, size, b"\"\\t\"\0" as *const u8 as *const libc::c_char)
                as size_t;
        }
        7 => {
            len = snprintf(buf, size, b"\"\\a\"\0" as *const u8 as *const libc::c_char)
                as size_t;
        }
        8 => {
            len = snprintf(buf, size, b"\"\\b\"\0" as *const u8 as *const libc::c_char)
                as size_t;
        }
        _ => {
            if *(*__ctype_b_loc()).offset(byte as libc::c_int as isize) as libc::c_int
                & _ISprint as libc::c_int as libc::c_ushort as libc::c_int != 0
            {
                len = snprintf(
                    buf,
                    size,
                    b"\"%c\"\0" as *const u8 as *const libc::c_char,
                    byte as libc::c_int,
                ) as size_t;
            } else {
                len = snprintf(
                    buf,
                    size,
                    b"\"\\x%02x\"\0" as *const u8 as *const libc::c_char,
                    byte as libc::c_uchar as libc::c_int,
                ) as size_t;
            }
        }
    }
    return len;
}
unsafe extern "C" fn __redisReaderSetErrorProtocolByte(
    mut r: *mut redisReader,
    mut byte: libc::c_char,
) {
    let mut cbuf: [libc::c_char; 8] = [0; 8];
    let mut sbuf: [libc::c_char; 128] = [0; 128];
    chrtos(
        cbuf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong,
        byte,
    );
    snprintf(
        sbuf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
        b"Protocol error, got %s as reply type byte\0" as *const u8
            as *const libc::c_char,
        cbuf.as_mut_ptr(),
    );
    __redisReaderSetError(r, 4 as libc::c_int, sbuf.as_mut_ptr());
}
unsafe extern "C" fn __redisReaderSetErrorOOM(mut r: *mut redisReader) {
    __redisReaderSetError(
        r,
        5 as libc::c_int,
        b"Out of memory\0" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn readBytes(
    mut r: *mut redisReader,
    mut bytes: libc::c_uint,
) -> *mut libc::c_char {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    if ((*r).len).wrapping_sub((*r).pos) >= bytes as libc::c_ulong {
        p = ((*r).buf).offset((*r).pos as isize);
        (*r)
            .pos = ((*r).pos as libc::c_ulong).wrapping_add(bytes as libc::c_ulong)
            as size_t as size_t;
        return p;
    }
    return 0 as *mut libc::c_char;
}
unsafe extern "C" fn seekNewline(
    mut s: *mut libc::c_char,
    mut len: size_t,
) -> *mut libc::c_char {
    let mut ret: *mut libc::c_char = 0 as *mut libc::c_char;
    if len < 2 as libc::c_int as libc::c_ulong {
        return 0 as *mut libc::c_char;
    }
    len = len.wrapping_sub(1);
    len;
    loop {
        ret = memchr(s as *const libc::c_void, '\r' as i32, len) as *mut libc::c_char;
        if ret.is_null() {
            break;
        }
        if *ret.offset(1 as libc::c_int as isize) as libc::c_int == '\n' as i32 {
            break;
        }
        ret = ret.offset(1);
        ret;
        len = (len as libc::c_ulong)
            .wrapping_sub(ret.offset_from(s) as libc::c_long as libc::c_ulong) as size_t
            as size_t;
        s = ret;
    }
    return ret;
}
unsafe extern "C" fn string2ll(
    mut s: *const libc::c_char,
    mut slen: size_t,
    mut value: *mut libc::c_longlong,
) -> libc::c_int {
    let mut p: *const libc::c_char = s;
    let mut plen: size_t = 0 as libc::c_int as size_t;
    let mut negative: libc::c_int = 0 as libc::c_int;
    let mut v: libc::c_ulonglong = 0;
    if plen == slen {
        return -(1 as libc::c_int);
    }
    if slen == 1 as libc::c_int as libc::c_ulong
        && *p.offset(0 as libc::c_int as isize) as libc::c_int == '0' as i32
    {
        if !value.is_null() {
            *value = 0 as libc::c_int as libc::c_longlong;
        }
        return 0 as libc::c_int;
    }
    if *p.offset(0 as libc::c_int as isize) as libc::c_int == '-' as i32 {
        negative = 1 as libc::c_int;
        p = p.offset(1);
        p;
        plen = plen.wrapping_add(1);
        plen;
        if plen == slen {
            return -(1 as libc::c_int);
        }
    }
    if *p.offset(0 as libc::c_int as isize) as libc::c_int >= '1' as i32
        && *p.offset(0 as libc::c_int as isize) as libc::c_int <= '9' as i32
    {
        v = (*p.offset(0 as libc::c_int as isize) as libc::c_int - '0' as i32)
            as libc::c_ulonglong;
        p = p.offset(1);
        p;
        plen = plen.wrapping_add(1);
        plen;
    } else if *p.offset(0 as libc::c_int as isize) as libc::c_int == '0' as i32
        && slen == 1 as libc::c_int as libc::c_ulong
    {
        *value = 0 as libc::c_int as libc::c_longlong;
        return 0 as libc::c_int;
    } else {
        return -(1 as libc::c_int)
    }
    while plen < slen
        && *p.offset(0 as libc::c_int as isize) as libc::c_int >= '0' as i32
        && *p.offset(0 as libc::c_int as isize) as libc::c_int <= '9' as i32
    {
        if v
            > (9223372036854775807 as libc::c_longlong as libc::c_ulonglong)
                .wrapping_mul(2 as libc::c_ulonglong)
                .wrapping_add(1 as libc::c_ulonglong)
                .wrapping_div(10 as libc::c_int as libc::c_ulonglong)
        {
            return -(1 as libc::c_int);
        }
        v = v.wrapping_mul(10 as libc::c_int as libc::c_ulonglong);
        if v
            > (9223372036854775807 as libc::c_longlong as libc::c_ulonglong)
                .wrapping_mul(2 as libc::c_ulonglong)
                .wrapping_add(1 as libc::c_ulonglong)
                .wrapping_sub(
                    (*p.offset(0 as libc::c_int as isize) as libc::c_int - '0' as i32)
                        as libc::c_ulonglong,
                )
        {
            return -(1 as libc::c_int);
        }
        v = v
            .wrapping_add(
                (*p.offset(0 as libc::c_int as isize) as libc::c_int - '0' as i32)
                    as libc::c_ulonglong,
            );
        p = p.offset(1);
        p;
        plen = plen.wrapping_add(1);
        plen;
    }
    if plen < slen {
        return -(1 as libc::c_int);
    }
    if negative != 0 {
        if v
            > (-(-(9223372036854775807 as libc::c_longlong) - 1 as libc::c_longlong
                + 1 as libc::c_int as libc::c_longlong) as libc::c_ulonglong)
                .wrapping_add(1 as libc::c_int as libc::c_ulonglong)
        {
            return -(1 as libc::c_int);
        }
        if !value.is_null() {
            *value = v.wrapping_neg() as libc::c_longlong;
        }
    } else {
        if v > 9223372036854775807 as libc::c_longlong as libc::c_ulonglong {
            return -(1 as libc::c_int);
        }
        if !value.is_null() {
            *value = v as libc::c_longlong;
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn readLine(
    mut r: *mut redisReader,
    mut _len: *mut libc::c_int,
) -> *mut libc::c_char {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: libc::c_int = 0;
    p = ((*r).buf).offset((*r).pos as isize);
    s = seekNewline(p, ((*r).len).wrapping_sub((*r).pos));
    if !s.is_null() {
        len = s.offset_from(((*r).buf).offset((*r).pos as isize)) as libc::c_long
            as libc::c_int;
        (*r)
            .pos = ((*r).pos as libc::c_ulong)
            .wrapping_add((len + 2 as libc::c_int) as libc::c_ulong) as size_t as size_t;
        if !_len.is_null() {
            *_len = len;
        }
        return p;
    }
    return 0 as *mut libc::c_char;
}
unsafe extern "C" fn moveToNextTask(mut r: *mut redisReader) {
    let mut cur: *mut redisReadTask = 0 as *mut redisReadTask;
    let mut prv: *mut redisReadTask = 0 as *mut redisReadTask;
    while (*r).ridx >= 0 as libc::c_int {
        if (*r).ridx == 0 as libc::c_int {
            (*r).ridx -= 1;
            (*r).ridx;
            return;
        }
        cur = *((*r).task).offset((*r).ridx as isize);
        prv = *((*r).task).offset(((*r).ridx - 1 as libc::c_int) as isize);
        if (*prv).type_0 == 2 as libc::c_int || (*prv).type_0 == 9 as libc::c_int
            || (*prv).type_0 == 11 as libc::c_int || (*prv).type_0 == 10 as libc::c_int
            || (*prv).type_0 == 12 as libc::c_int
        {} else {
            __assert_fail(
                b"prv->type == REDIS_REPLY_ARRAY || prv->type == REDIS_REPLY_MAP || prv->type == REDIS_REPLY_ATTR || prv->type == REDIS_REPLY_SET || prv->type == REDIS_REPLY_PUSH\0"
                    as *const u8 as *const libc::c_char,
                b"read.c\0" as *const u8 as *const libc::c_char,
                255 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 35],
                    &[libc::c_char; 35],
                >(b"void moveToNextTask(redisReader *)\0"))
                    .as_ptr(),
            );
        };
        if (*cur).idx as libc::c_longlong
            == (*prv).elements - 1 as libc::c_int as libc::c_longlong
        {
            (*r).ridx -= 1;
            (*r).ridx;
        } else {
            if ((*cur).idx as libc::c_longlong) < (*prv).elements {} else {
                __assert_fail(
                    b"cur->idx < prv->elements\0" as *const u8 as *const libc::c_char,
                    b"read.c\0" as *const u8 as *const libc::c_char,
                    260 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 35],
                        &[libc::c_char; 35],
                    >(b"void moveToNextTask(redisReader *)\0"))
                        .as_ptr(),
                );
            };
            (*cur).type_0 = -(1 as libc::c_int);
            (*cur).elements = -(1 as libc::c_int) as libc::c_longlong;
            (*cur).idx += 1;
            (*cur).idx;
            return;
        }
    }
}
unsafe extern "C" fn processLineItem(mut r: *mut redisReader) -> libc::c_int {
    let mut cur: *mut redisReadTask = *((*r).task).offset((*r).ridx as isize);
    let mut obj: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: libc::c_int = 0;
    p = readLine(r, &mut len);
    if !p.is_null() {
        if (*cur).type_0 == 3 as libc::c_int {
            let mut v: libc::c_longlong = 0;
            if string2ll(p, len as size_t, &mut v) == -(1 as libc::c_int) {
                __redisReaderSetError(
                    r,
                    4 as libc::c_int,
                    b"Bad integer value\0" as *const u8 as *const libc::c_char,
                );
                return -(1 as libc::c_int);
            }
            if !((*r).fn_0).is_null() && ((*(*r).fn_0).createInteger).is_some() {
                obj = ((*(*r).fn_0).createInteger).unwrap()(cur, v);
            } else {
                obj = 3 as libc::c_int as *mut libc::c_void;
            }
        } else if (*cur).type_0 == 7 as libc::c_int {
            let mut buf: [libc::c_char; 326] = [0; 326];
            let mut eptr: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut d: libc::c_double = 0.;
            if len as size_t
                >= ::std::mem::size_of::<[libc::c_char; 326]>() as libc::c_ulong
            {
                __redisReaderSetError(
                    r,
                    4 as libc::c_int,
                    b"Double value is too large\0" as *const u8 as *const libc::c_char,
                );
                return -(1 as libc::c_int);
            }
            memcpy(
                buf.as_mut_ptr() as *mut libc::c_void,
                p as *const libc::c_void,
                len as libc::c_ulong,
            );
            buf[len as usize] = '\0' as i32 as libc::c_char;
            if len == 3 as libc::c_int
                && strcasecmp(
                    buf.as_mut_ptr(),
                    b"inf\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
            {
                d = ::std::f32::INFINITY as libc::c_double;
            } else if len == 4 as libc::c_int
                && strcasecmp(
                    buf.as_mut_ptr(),
                    b"-inf\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
            {
                d = -::std::f32::INFINITY as libc::c_double;
            } else if len == 3 as libc::c_int
                && strcasecmp(
                    buf.as_mut_ptr(),
                    b"nan\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                || len == 4 as libc::c_int
                    && strcasecmp(
                        buf.as_mut_ptr(),
                        b"-nan\0" as *const u8 as *const libc::c_char,
                    ) == 0 as libc::c_int
            {
                d = ::std::f32::NAN as libc::c_double;
            } else {
                d = strtod(buf.as_mut_ptr(), &mut eptr);
                if buf[0 as libc::c_int as usize] as libc::c_int == '\0' as i32
                    || eptr
                        != &mut *buf.as_mut_ptr().offset(len as isize)
                            as *mut libc::c_char || d.is_finite() as i32 == 0
                {
                    __redisReaderSetError(
                        r,
                        4 as libc::c_int,
                        b"Bad double value\0" as *const u8 as *const libc::c_char,
                    );
                    return -(1 as libc::c_int);
                }
            }
            if !((*r).fn_0).is_null() && ((*(*r).fn_0).createDouble).is_some() {
                obj = ((*(*r).fn_0).createDouble)
                    .unwrap()(cur, d, buf.as_mut_ptr(), len as size_t);
            } else {
                obj = 7 as libc::c_int as *mut libc::c_void;
            }
        } else if (*cur).type_0 == 4 as libc::c_int {
            if len != 0 as libc::c_int {
                __redisReaderSetError(
                    r,
                    4 as libc::c_int,
                    b"Bad nil value\0" as *const u8 as *const libc::c_char,
                );
                return -(1 as libc::c_int);
            }
            if !((*r).fn_0).is_null() && ((*(*r).fn_0).createNil).is_some() {
                obj = ((*(*r).fn_0).createNil).unwrap()(cur);
            } else {
                obj = 4 as libc::c_int as *mut libc::c_void;
            }
        } else if (*cur).type_0 == 8 as libc::c_int {
            let mut bval: libc::c_int = 0;
            if len != 1 as libc::c_int
                || (strchr(
                    b"tTfF\0" as *const u8 as *const libc::c_char,
                    *p.offset(0 as libc::c_int as isize) as libc::c_int,
                ))
                    .is_null()
            {
                __redisReaderSetError(
                    r,
                    4 as libc::c_int,
                    b"Bad bool value\0" as *const u8 as *const libc::c_char,
                );
                return -(1 as libc::c_int);
            }
            bval = (*p.offset(0 as libc::c_int as isize) as libc::c_int == 't' as i32
                || *p.offset(0 as libc::c_int as isize) as libc::c_int == 'T' as i32)
                as libc::c_int;
            if !((*r).fn_0).is_null() && ((*(*r).fn_0).createBool).is_some() {
                obj = ((*(*r).fn_0).createBool).unwrap()(cur, bval);
            } else {
                obj = 8 as libc::c_int as *mut libc::c_void;
            }
        } else if (*cur).type_0 == 13 as libc::c_int {
            let mut i: libc::c_int = 0 as libc::c_int;
            while i < len {
                if !(i == 0 as libc::c_int
                    && *p.offset(0 as libc::c_int as isize) as libc::c_int == '-' as i32)
                {
                    if (*p.offset(i as isize) as libc::c_int) < '0' as i32
                        || *p.offset(i as isize) as libc::c_int > '9' as i32
                    {
                        __redisReaderSetError(
                            r,
                            4 as libc::c_int,
                            b"Bad bignum value\0" as *const u8 as *const libc::c_char,
                        );
                        return -(1 as libc::c_int);
                    }
                }
                i += 1;
                i;
            }
            if !((*r).fn_0).is_null() && ((*(*r).fn_0).createString).is_some() {
                obj = ((*(*r).fn_0).createString).unwrap()(cur, p, len as size_t);
            } else {
                obj = 13 as libc::c_int as *mut libc::c_void;
            }
        } else {
            let mut i_0: libc::c_int = 0 as libc::c_int;
            while i_0 < len {
                if *p.offset(i_0 as isize) as libc::c_int == '\r' as i32
                    || *p.offset(i_0 as isize) as libc::c_int == '\n' as i32
                {
                    __redisReaderSetError(
                        r,
                        4 as libc::c_int,
                        b"Bad simple string value\0" as *const u8 as *const libc::c_char,
                    );
                    return -(1 as libc::c_int);
                }
                i_0 += 1;
                i_0;
            }
            if !((*r).fn_0).is_null() && ((*(*r).fn_0).createString).is_some() {
                obj = ((*(*r).fn_0).createString).unwrap()(cur, p, len as size_t);
            } else {
                obj = (*cur).type_0 as uintptr_t as *mut libc::c_void;
            }
        }
        if obj.is_null() {
            __redisReaderSetErrorOOM(r);
            return -(1 as libc::c_int);
        }
        if (*r).ridx == 0 as libc::c_int {
            (*r).reply = obj;
        }
        moveToNextTask(r);
        return 0 as libc::c_int;
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn processBulkItem(mut r: *mut redisReader) -> libc::c_int {
    let mut cur: *mut redisReadTask = *((*r).task).offset((*r).ridx as isize);
    let mut obj: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: libc::c_longlong = 0;
    let mut bytelen: libc::c_ulong = 0;
    let mut success: libc::c_int = 0 as libc::c_int;
    p = ((*r).buf).offset((*r).pos as isize);
    s = seekNewline(p, ((*r).len).wrapping_sub((*r).pos));
    if !s.is_null() {
        p = ((*r).buf).offset((*r).pos as isize);
        bytelen = (s.offset_from(((*r).buf).offset((*r).pos as isize)) as libc::c_long
            + 2 as libc::c_int as libc::c_long) as libc::c_ulong;
        if string2ll(
            p,
            bytelen.wrapping_sub(2 as libc::c_int as libc::c_ulong),
            &mut len,
        ) == -(1 as libc::c_int)
        {
            __redisReaderSetError(
                r,
                4 as libc::c_int,
                b"Bad bulk string length\0" as *const u8 as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
        if len < -(1 as libc::c_int) as libc::c_longlong
            || 9223372036854775807 as libc::c_longlong as libc::c_ulonglong
                > 18446744073709551615 as libc::c_ulong as libc::c_ulonglong
                && len > 18446744073709551615 as libc::c_ulong as libc::c_longlong
        {
            __redisReaderSetError(
                r,
                4 as libc::c_int,
                b"Bulk string length out of range\0" as *const u8 as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
        if len == -(1 as libc::c_int) as libc::c_longlong {
            if !((*r).fn_0).is_null() && ((*(*r).fn_0).createNil).is_some() {
                obj = ((*(*r).fn_0).createNil).unwrap()(cur);
            } else {
                obj = 4 as libc::c_int as *mut libc::c_void;
            }
            success = 1 as libc::c_int;
        } else {
            bytelen = (bytelen as libc::c_ulonglong)
                .wrapping_add(
                    (len + 2 as libc::c_int as libc::c_longlong) as libc::c_ulonglong,
                ) as libc::c_ulong as libc::c_ulong;
            if ((*r).pos).wrapping_add(bytelen) <= (*r).len {
                if (*cur).type_0 == 14 as libc::c_int
                    && len < 4 as libc::c_int as libc::c_longlong
                    || (*cur).type_0 == 14 as libc::c_int
                        && *s.offset(5 as libc::c_int as isize) as libc::c_int
                            != ':' as i32
                {
                    __redisReaderSetError(
                        r,
                        4 as libc::c_int,
                        b"Verbatim string 4 bytes of content type are missing or incorrectly encoded.\0"
                            as *const u8 as *const libc::c_char,
                    );
                    return -(1 as libc::c_int);
                }
                if !((*r).fn_0).is_null() && ((*(*r).fn_0).createString).is_some() {
                    obj = ((*(*r).fn_0).createString)
                        .unwrap()(
                        cur,
                        s.offset(2 as libc::c_int as isize),
                        len as size_t,
                    );
                } else {
                    obj = (*cur).type_0 as uintptr_t as *mut libc::c_void;
                }
                success = 1 as libc::c_int;
            }
        }
        if success != 0 {
            if obj.is_null() {
                __redisReaderSetErrorOOM(r);
                return -(1 as libc::c_int);
            }
            (*r)
                .pos = ((*r).pos as libc::c_ulong).wrapping_add(bytelen) as size_t
                as size_t;
            if (*r).ridx == 0 as libc::c_int {
                (*r).reply = obj;
            }
            moveToNextTask(r);
            return 0 as libc::c_int;
        }
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn redisReaderGrow(mut r: *mut redisReader) -> libc::c_int {
    let mut current_block: u64;
    let mut aux: *mut *mut redisReadTask = 0 as *mut *mut redisReadTask;
    let mut newlen: libc::c_int = 0;
    newlen = (*r).tasks + 9 as libc::c_int;
    aux = hi_realloc(
        (*r).task as *mut libc::c_void,
        (::std::mem::size_of::<*mut redisReadTask>() as libc::c_ulong)
            .wrapping_mul(newlen as libc::c_ulong),
    ) as *mut *mut redisReadTask;
    if !aux.is_null() {
        (*r).task = aux;
        loop {
            if !((*r).tasks < newlen) {
                current_block = 10879442775620481940;
                break;
            }
            let ref mut fresh0 = *((*r).task).offset((*r).tasks as isize);
            *fresh0 = hi_calloc(
                1 as libc::c_int as size_t,
                ::std::mem::size_of::<redisReadTask>() as libc::c_ulong,
            ) as *mut redisReadTask;
            if (*((*r).task).offset((*r).tasks as isize)).is_null() {
                current_block = 17080877559662114965;
                break;
            }
            (*r).tasks += 1;
            (*r).tasks;
        }
        match current_block {
            17080877559662114965 => {}
            _ => return 0 as libc::c_int,
        }
    }
    __redisReaderSetErrorOOM(r);
    return -(1 as libc::c_int);
}
unsafe extern "C" fn processAggregateItem(mut r: *mut redisReader) -> libc::c_int {
    let mut cur: *mut redisReadTask = *((*r).task).offset((*r).ridx as isize);
    let mut obj: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut elements: libc::c_longlong = 0;
    let mut root: libc::c_int = 0 as libc::c_int;
    let mut len: libc::c_int = 0;
    if (*r).ridx == (*r).tasks - 1 as libc::c_int {
        if redisReaderGrow(r) == -(1 as libc::c_int) {
            return -(1 as libc::c_int);
        }
    }
    p = readLine(r, &mut len);
    if !p.is_null() {
        if string2ll(p, len as size_t, &mut elements) == -(1 as libc::c_int) {
            __redisReaderSetError(
                r,
                4 as libc::c_int,
                b"Bad multi-bulk length\0" as *const u8 as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
        root = ((*r).ridx == 0 as libc::c_int) as libc::c_int;
        if elements < -(1 as libc::c_int) as libc::c_longlong
            || 9223372036854775807 as libc::c_longlong as libc::c_ulonglong
                > 18446744073709551615 as libc::c_ulong as libc::c_ulonglong
                && elements as libc::c_ulonglong
                    > 18446744073709551615 as libc::c_ulong as libc::c_ulonglong
            || (*r).maxelements > 0 as libc::c_int as libc::c_longlong
                && elements > (*r).maxelements
        {
            __redisReaderSetError(
                r,
                4 as libc::c_int,
                b"Multi-bulk length out of range\0" as *const u8 as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
        if elements == -(1 as libc::c_int) as libc::c_longlong {
            if !((*r).fn_0).is_null() && ((*(*r).fn_0).createNil).is_some() {
                obj = ((*(*r).fn_0).createNil).unwrap()(cur);
            } else {
                obj = 4 as libc::c_int as *mut libc::c_void;
            }
            if obj.is_null() {
                __redisReaderSetErrorOOM(r);
                return -(1 as libc::c_int);
            }
            moveToNextTask(r);
        } else {
            if (*cur).type_0 == 9 as libc::c_int || (*cur).type_0 == 11 as libc::c_int {
                elements *= 2 as libc::c_int as libc::c_longlong;
            }
            if !((*r).fn_0).is_null() && ((*(*r).fn_0).createArray).is_some() {
                obj = ((*(*r).fn_0).createArray).unwrap()(cur, elements as size_t);
            } else {
                obj = (*cur).type_0 as uintptr_t as *mut libc::c_void;
            }
            if obj.is_null() {
                __redisReaderSetErrorOOM(r);
                return -(1 as libc::c_int);
            }
            if elements > 0 as libc::c_int as libc::c_longlong {
                (*cur).elements = elements;
                (*cur).obj = obj;
                (*r).ridx += 1;
                (*r).ridx;
                (**((*r).task).offset((*r).ridx as isize)).type_0 = -(1 as libc::c_int);
                (**((*r).task).offset((*r).ridx as isize))
                    .elements = -(1 as libc::c_int) as libc::c_longlong;
                (**((*r).task).offset((*r).ridx as isize)).idx = 0 as libc::c_int;
                let ref mut fresh1 = (**((*r).task).offset((*r).ridx as isize)).obj;
                *fresh1 = 0 as *mut libc::c_void;
                let ref mut fresh2 = (**((*r).task).offset((*r).ridx as isize)).parent;
                *fresh2 = cur;
                let ref mut fresh3 = (**((*r).task).offset((*r).ridx as isize)).privdata;
                *fresh3 = (*r).privdata;
            } else {
                moveToNextTask(r);
            }
        }
        if root != 0 {
            (*r).reply = obj;
        }
        return 0 as libc::c_int;
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn processItem(mut r: *mut redisReader) -> libc::c_int {
    let mut cur: *mut redisReadTask = *((*r).task).offset((*r).ridx as isize);
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    if (*cur).type_0 < 0 as libc::c_int {
        p = readBytes(r, 1 as libc::c_int as libc::c_uint);
        if !p.is_null() {
            match *p.offset(0 as libc::c_int as isize) as libc::c_int {
                45 => {
                    (*cur).type_0 = 6 as libc::c_int;
                }
                43 => {
                    (*cur).type_0 = 5 as libc::c_int;
                }
                58 => {
                    (*cur).type_0 = 3 as libc::c_int;
                }
                44 => {
                    (*cur).type_0 = 7 as libc::c_int;
                }
                95 => {
                    (*cur).type_0 = 4 as libc::c_int;
                }
                36 => {
                    (*cur).type_0 = 1 as libc::c_int;
                }
                42 => {
                    (*cur).type_0 = 2 as libc::c_int;
                }
                37 => {
                    (*cur).type_0 = 9 as libc::c_int;
                }
                124 => {
                    (*cur).type_0 = 11 as libc::c_int;
                }
                126 => {
                    (*cur).type_0 = 10 as libc::c_int;
                }
                35 => {
                    (*cur).type_0 = 8 as libc::c_int;
                }
                61 => {
                    (*cur).type_0 = 14 as libc::c_int;
                }
                62 => {
                    (*cur).type_0 = 12 as libc::c_int;
                }
                40 => {
                    (*cur).type_0 = 13 as libc::c_int;
                }
                _ => {
                    __redisReaderSetErrorProtocolByte(r, *p);
                    return -(1 as libc::c_int);
                }
            }
        } else {
            return -(1 as libc::c_int)
        }
    }
    match (*cur).type_0 {
        6 | 5 | 3 | 7 | 4 | 8 | 13 => return processLineItem(r),
        1 | 14 => return processBulkItem(r),
        2 | 9 | 11 | 10 | 12 => return processAggregateItem(r),
        _ => {
            if !(0 as *mut libc::c_void).is_null() {} else {
                __assert_fail(
                    b"NULL\0" as *const u8 as *const libc::c_char,
                    b"read.c\0" as *const u8 as *const libc::c_char,
                    654 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 31],
                        &[libc::c_char; 31],
                    >(b"int processItem(redisReader *)\0"))
                        .as_ptr(),
                );
            };
            return -(1 as libc::c_int);
        }
    };
}
pub unsafe extern "C" fn redisReaderCreateWithFunctions(
    mut fn_0: *mut redisReplyObjectFunctions,
) -> *mut redisReader {
    let mut current_block: u64;
    let mut r: *mut redisReader = 0 as *mut redisReader;
    r = hi_calloc(
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<redisReader>() as libc::c_ulong,
    ) as *mut redisReader;
    if r.is_null() {
        return 0 as *mut redisReader;
    }
    (*r).buf = sdsempty();
    if !((*r).buf).is_null() {
        (*r)
            .task = hi_calloc(
            9 as libc::c_int as size_t,
            ::std::mem::size_of::<*mut redisReadTask>() as libc::c_ulong,
        ) as *mut *mut redisReadTask;
        if !((*r).task).is_null() {
            loop {
                if !((*r).tasks < 9 as libc::c_int) {
                    current_block = 5720623009719927633;
                    break;
                }
                let ref mut fresh4 = *((*r).task).offset((*r).tasks as isize);
                *fresh4 = hi_calloc(
                    1 as libc::c_int as size_t,
                    ::std::mem::size_of::<redisReadTask>() as libc::c_ulong,
                ) as *mut redisReadTask;
                if (*((*r).task).offset((*r).tasks as isize)).is_null() {
                    current_block = 8607045692920977316;
                    break;
                }
                (*r).tasks += 1;
                (*r).tasks;
            }
            match current_block {
                8607045692920977316 => {}
                _ => {
                    (*r).fn_0 = fn_0;
                    (*r).maxbuf = (1024 as libc::c_int * 16 as libc::c_int) as size_t;
                    (*r)
                        .maxelements = ((1 as libc::c_longlong) << 32 as libc::c_int)
                        - 1 as libc::c_int as libc::c_longlong;
                    (*r).ridx = -(1 as libc::c_int);
                    return r;
                }
            }
        }
    }
    redisReaderFree(r);
    return 0 as *mut redisReader;
}
pub unsafe extern "C" fn redisReaderFree(mut r: *mut redisReader) {
    if r.is_null() {
        return;
    }
    if !((*r).reply).is_null() && !((*r).fn_0).is_null()
        && ((*(*r).fn_0).freeObject).is_some()
    {
        ((*(*r).fn_0).freeObject).unwrap()((*r).reply);
    }
    if !((*r).task).is_null() {
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < (*r).tasks {
            hi_free(*((*r).task).offset(i as isize) as *mut libc::c_void);
            i += 1;
            i;
        }
        hi_free((*r).task as *mut libc::c_void);
    }
    sdsfree((*r).buf);
    hi_free(r as *mut libc::c_void);
}
pub unsafe extern "C" fn redisReaderFeed(
    mut r: *mut redisReader,
    mut buf: *const libc::c_char,
    mut len: size_t,
) -> libc::c_int {
    let mut current_block: u64;
    let mut newbuf: sds = 0 as *mut libc::c_char;
    if (*r).err != 0 {
        return -(1 as libc::c_int);
    }
    if !buf.is_null() && len >= 1 as libc::c_int as libc::c_ulong {
        if (*r).len == 0 as libc::c_int as libc::c_ulong
            && (*r).maxbuf != 0 as libc::c_int as libc::c_ulong
            && sdsavail((*r).buf) > (*r).maxbuf
        {
            sdsfree((*r).buf);
            (*r).buf = sdsempty();
            if ((*r).buf).is_null() {
                current_block = 6934972728035309289;
            } else {
                (*r).pos = 0 as libc::c_int as size_t;
                current_block = 6483416627284290920;
            }
        } else {
            current_block = 6483416627284290920;
        }
        match current_block {
            6483416627284290920 => {
                newbuf = sdscatlen((*r).buf, buf as *const libc::c_void, len);
                if newbuf.is_null() {
                    current_block = 6934972728035309289;
                } else {
                    (*r).buf = newbuf;
                    (*r).len = sdslen((*r).buf);
                    current_block = 11812396948646013369;
                }
            }
            _ => {}
        }
        match current_block {
            11812396948646013369 => {}
            _ => {
                __redisReaderSetErrorOOM(r);
                return -(1 as libc::c_int);
            }
        }
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn redisReaderGetReply(
    mut r: *mut redisReader,
    mut reply: *mut *mut libc::c_void,
) -> libc::c_int {
    if !reply.is_null() {
        *reply = 0 as *mut libc::c_void;
    }
    if (*r).err != 0 {
        return -(1 as libc::c_int);
    }
    if (*r).len == 0 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int;
    }
    if (*r).ridx == -(1 as libc::c_int) {
        (**((*r).task).offset(0 as libc::c_int as isize)).type_0 = -(1 as libc::c_int);
        (**((*r).task).offset(0 as libc::c_int as isize))
            .elements = -(1 as libc::c_int) as libc::c_longlong;
        (**((*r).task).offset(0 as libc::c_int as isize)).idx = -(1 as libc::c_int);
        let ref mut fresh5 = (**((*r).task).offset(0 as libc::c_int as isize)).obj;
        *fresh5 = 0 as *mut libc::c_void;
        let ref mut fresh6 = (**((*r).task).offset(0 as libc::c_int as isize)).parent;
        *fresh6 = 0 as *mut redisReadTask;
        let ref mut fresh7 = (**((*r).task).offset(0 as libc::c_int as isize)).privdata;
        *fresh7 = (*r).privdata;
        (*r).ridx = 0 as libc::c_int;
    }
    while (*r).ridx >= 0 as libc::c_int {
        if processItem(r) != 0 as libc::c_int {
            break;
        }
    }
    if (*r).err != 0 {
        return -(1 as libc::c_int);
    }
    if (*r).pos >= 1024 as libc::c_int as libc::c_ulong {
        if sdsrange((*r).buf, (*r).pos as ssize_t, -(1 as libc::c_int) as ssize_t)
            < 0 as libc::c_int
        {
            return -(1 as libc::c_int);
        }
        (*r).pos = 0 as libc::c_int as size_t;
        (*r).len = sdslen((*r).buf);
    }
    if (*r).ridx == -(1 as libc::c_int) {
        if !reply.is_null() {
            *reply = (*r).reply;
        } else if !((*r).reply).is_null() && !((*r).fn_0).is_null()
            && ((*(*r).fn_0).freeObject).is_some()
        {
            ((*(*r).fn_0).freeObject).unwrap()((*r).reply);
        }
        (*r).reply = 0 as *mut libc::c_void;
    }
    return 0 as libc::c_int;
}
