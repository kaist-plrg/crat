use ::libc;
extern "C" {
    fn vsnprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __ctype_tolower_loc() -> *mut *const __int32_t;
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    static mut hiredisAllocFns: hiredisAllocFuncs;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type size_t = libc::c_ulong;
pub type va_list = __builtin_va_list;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
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
pub type sds = *mut libc::c_char;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sdshdr5 {
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
pub struct sdshdr32 {
    pub len: uint32_t,
    pub alloc: uint32_t,
    pub flags: libc::c_uchar,
    pub buf: [libc::c_char; 0],
}
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
#[inline]
unsafe extern "C" fn tolower(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_tolower_loc()).offset(__c as isize)
    } else {
        __c
    };
}
#[inline]
unsafe extern "C" fn toupper(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_toupper_loc()).offset(__c as isize)
    } else {
        __c
    };
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
#[inline]
unsafe extern "C" fn sdssetlen(mut s: sds, mut newlen: size_t) {
    let mut flags: libc::c_uchar = *s.offset(-(1 as libc::c_int) as isize)
        as libc::c_uchar;
    match flags as libc::c_int & 7 as libc::c_int {
        0 => {
            let mut fp: *mut libc::c_uchar = (s as *mut libc::c_uchar)
                .offset(-(1 as libc::c_int as isize));
            *fp = (0 as libc::c_int as libc::c_ulong | newlen << 3 as libc::c_int)
                as libc::c_uchar;
        }
        1 => {
            (*(s.offset(-(::std::mem::size_of::<sdshdr8>() as libc::c_ulong as isize))
                as *mut sdshdr8))
                .len = newlen as uint8_t;
        }
        2 => {
            (*(s.offset(-(::std::mem::size_of::<sdshdr16>() as libc::c_ulong as isize))
                as *mut sdshdr16))
                .len = newlen as uint16_t;
        }
        3 => {
            (*(s.offset(-(::std::mem::size_of::<sdshdr32>() as libc::c_ulong as isize))
                as *mut sdshdr32))
                .len = newlen as uint32_t;
        }
        4 => {
            (*(s.offset(-(::std::mem::size_of::<sdshdr64>() as libc::c_ulong as isize))
                as *mut sdshdr64))
                .len = newlen;
        }
        _ => {}
    };
}
#[inline]
unsafe extern "C" fn sdsinclen(mut s: sds, mut inc: size_t) {
    let mut flags: libc::c_uchar = *s.offset(-(1 as libc::c_int) as isize)
        as libc::c_uchar;
    match flags as libc::c_int & 7 as libc::c_int {
        0 => {
            let mut fp: *mut libc::c_uchar = (s as *mut libc::c_uchar)
                .offset(-(1 as libc::c_int as isize));
            let mut newlen: libc::c_uchar = ((flags as libc::c_int >> 3 as libc::c_int)
                + inc as libc::c_uchar as libc::c_int) as libc::c_uchar;
            *fp = (0 as libc::c_int | (newlen as libc::c_int) << 3 as libc::c_int)
                as libc::c_uchar;
        }
        1 => {
            let ref mut fresh0 = (*(s
                .offset(-(::std::mem::size_of::<sdshdr8>() as libc::c_ulong as isize))
                as *mut sdshdr8))
                .len;
            *fresh0 = (*fresh0 as libc::c_int + inc as uint8_t as libc::c_int)
                as uint8_t;
        }
        2 => {
            let ref mut fresh1 = (*(s
                .offset(-(::std::mem::size_of::<sdshdr16>() as libc::c_ulong as isize))
                as *mut sdshdr16))
                .len;
            *fresh1 = (*fresh1 as libc::c_int + inc as uint16_t as libc::c_int)
                as uint16_t;
        }
        3 => {
            let ref mut fresh2 = (*(s
                .offset(-(::std::mem::size_of::<sdshdr32>() as libc::c_ulong as isize))
                as *mut sdshdr32))
                .len;
            *fresh2 = (*fresh2 as libc::c_uint).wrapping_add(inc as uint32_t) as uint32_t
                as uint32_t;
        }
        4 => {
            let ref mut fresh3 = (*(s
                .offset(-(::std::mem::size_of::<sdshdr64>() as libc::c_ulong as isize))
                as *mut sdshdr64))
                .len;
            *fresh3 = (*fresh3 as libc::c_ulong).wrapping_add(inc) as uint64_t
                as uint64_t;
        }
        _ => {}
    };
}
#[inline]
unsafe extern "C" fn sdsalloc(s: sds) -> size_t {
    let mut flags: libc::c_uchar = *s.offset(-(1 as libc::c_int) as isize)
        as libc::c_uchar;
    match flags as libc::c_int & 7 as libc::c_int {
        0 => return (flags as libc::c_int >> 3 as libc::c_int) as size_t,
        1 => {
            return (*(s
                .offset(-(::std::mem::size_of::<sdshdr8>() as libc::c_ulong as isize))
                as *mut sdshdr8))
                .alloc as size_t;
        }
        2 => {
            return (*(s
                .offset(-(::std::mem::size_of::<sdshdr16>() as libc::c_ulong as isize))
                as *mut sdshdr16))
                .alloc as size_t;
        }
        3 => {
            return (*(s
                .offset(-(::std::mem::size_of::<sdshdr32>() as libc::c_ulong as isize))
                as *mut sdshdr32))
                .alloc as size_t;
        }
        4 => {
            return (*(s
                .offset(-(::std::mem::size_of::<sdshdr64>() as libc::c_ulong as isize))
                as *mut sdshdr64))
                .alloc;
        }
        _ => {}
    }
    return 0 as libc::c_int as size_t;
}
#[inline]
unsafe extern "C" fn sdssetalloc(mut s: sds, mut newlen: size_t) {
    let mut flags: libc::c_uchar = *s.offset(-(1 as libc::c_int) as isize)
        as libc::c_uchar;
    match flags as libc::c_int & 7 as libc::c_int {
        1 => {
            (*(s.offset(-(::std::mem::size_of::<sdshdr8>() as libc::c_ulong as isize))
                as *mut sdshdr8))
                .alloc = newlen as uint8_t;
        }
        2 => {
            (*(s.offset(-(::std::mem::size_of::<sdshdr16>() as libc::c_ulong as isize))
                as *mut sdshdr16))
                .alloc = newlen as uint16_t;
        }
        3 => {
            (*(s.offset(-(::std::mem::size_of::<sdshdr32>() as libc::c_ulong as isize))
                as *mut sdshdr32))
                .alloc = newlen as uint32_t;
        }
        4 => {
            (*(s.offset(-(::std::mem::size_of::<sdshdr64>() as libc::c_ulong as isize))
                as *mut sdshdr64))
                .alloc = newlen;
        }
        0 | _ => {}
    };
}
#[inline]
unsafe extern "C" fn hi_free(mut ptr: *mut libc::c_void) {
    (hiredisAllocFns.freeFn).unwrap()(ptr);
}
#[inline]
unsafe extern "C" fn hi_malloc(mut size: size_t) -> *mut libc::c_void {
    return (hiredisAllocFns.mallocFn).unwrap()(size);
}
#[inline]
unsafe extern "C" fn hi_realloc(
    mut ptr: *mut libc::c_void,
    mut size: size_t,
) -> *mut libc::c_void {
    return (hiredisAllocFns.reallocFn).unwrap()(ptr, size);
}
#[inline]
unsafe extern "C" fn sdsHdrSize(mut type_0: libc::c_char) -> libc::c_int {
    match type_0 as libc::c_int & 7 as libc::c_int {
        0 => return ::std::mem::size_of::<sdshdr5>() as libc::c_ulong as libc::c_int,
        1 => return ::std::mem::size_of::<sdshdr8>() as libc::c_ulong as libc::c_int,
        2 => return ::std::mem::size_of::<sdshdr16>() as libc::c_ulong as libc::c_int,
        3 => return ::std::mem::size_of::<sdshdr32>() as libc::c_ulong as libc::c_int,
        4 => return ::std::mem::size_of::<sdshdr64>() as libc::c_ulong as libc::c_int,
        _ => {}
    }
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn sdsReqType(mut string_size: size_t) -> libc::c_char {
    if string_size < 32 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int as libc::c_char;
    }
    if string_size < 0xff as libc::c_int as libc::c_ulong {
        return 1 as libc::c_int as libc::c_char;
    }
    if string_size < 0xffff as libc::c_int as libc::c_ulong {
        return 2 as libc::c_int as libc::c_char;
    }
    if string_size < 0xffffffff as libc::c_uint as libc::c_ulong {
        return 3 as libc::c_int as libc::c_char;
    }
    return 4 as libc::c_int as libc::c_char;
}
pub unsafe extern "C" fn sdsnewlen(
    mut init: *const libc::c_void,
    mut initlen: size_t,
) -> sds {
    let mut sh: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut s: sds = 0 as *mut libc::c_char;
    let mut type_0: libc::c_char = sdsReqType(initlen);
    if type_0 as libc::c_int == 0 as libc::c_int
        && initlen == 0 as libc::c_int as libc::c_ulong
    {
        type_0 = 1 as libc::c_int as libc::c_char;
    }
    let mut hdrlen: libc::c_int = sdsHdrSize(type_0);
    let mut fp: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    if (hdrlen as libc::c_ulong)
        .wrapping_add(initlen)
        .wrapping_add(1 as libc::c_int as libc::c_ulong) <= initlen
    {
        return 0 as sds;
    }
    sh = hi_malloc(
        (hdrlen as libc::c_ulong)
            .wrapping_add(initlen)
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
    );
    if sh.is_null() {
        return 0 as sds;
    }
    if init.is_null() {
        memset(
            sh,
            0 as libc::c_int,
            (hdrlen as libc::c_ulong)
                .wrapping_add(initlen)
                .wrapping_add(1 as libc::c_int as libc::c_ulong),
        );
    }
    s = (sh as *mut libc::c_char).offset(hdrlen as isize);
    fp = (s as *mut libc::c_uchar).offset(-(1 as libc::c_int as isize));
    match type_0 as libc::c_int {
        0 => {
            *fp = (type_0 as libc::c_ulong | initlen << 3 as libc::c_int)
                as libc::c_uchar;
        }
        1 => {
            let mut sh_0: *mut sdshdr8 = s
                .offset(-(::std::mem::size_of::<sdshdr8>() as libc::c_ulong as isize))
                as *mut sdshdr8;
            (*sh_0).len = initlen as uint8_t;
            (*sh_0).alloc = initlen as uint8_t;
            *fp = type_0 as libc::c_uchar;
        }
        2 => {
            let mut sh_1: *mut sdshdr16 = s
                .offset(-(::std::mem::size_of::<sdshdr16>() as libc::c_ulong as isize))
                as *mut sdshdr16;
            (*sh_1).len = initlen as uint16_t;
            (*sh_1).alloc = initlen as uint16_t;
            *fp = type_0 as libc::c_uchar;
        }
        3 => {
            let mut sh_2: *mut sdshdr32 = s
                .offset(-(::std::mem::size_of::<sdshdr32>() as libc::c_ulong as isize))
                as *mut sdshdr32;
            (*sh_2).len = initlen as uint32_t;
            (*sh_2).alloc = initlen as uint32_t;
            *fp = type_0 as libc::c_uchar;
        }
        4 => {
            let mut sh_3: *mut sdshdr64 = s
                .offset(-(::std::mem::size_of::<sdshdr64>() as libc::c_ulong as isize))
                as *mut sdshdr64;
            (*sh_3).len = initlen;
            (*sh_3).alloc = initlen;
            *fp = type_0 as libc::c_uchar;
        }
        _ => {}
    }
    if initlen != 0 && !init.is_null() {
        memcpy(s as *mut libc::c_void, init, initlen);
    }
    *s.offset(initlen as isize) = '\0' as i32 as libc::c_char;
    return s;
}
pub unsafe extern "C" fn sdsempty() -> sds {
    return sdsnewlen(
        b"\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        0 as libc::c_int as size_t,
    );
}
pub unsafe extern "C" fn sdsnew(mut init: *const libc::c_char) -> sds {
    let mut initlen: size_t = if init.is_null() {
        0 as libc::c_int as libc::c_ulong
    } else {
        strlen(init)
    };
    return sdsnewlen(init as *const libc::c_void, initlen);
}
pub unsafe extern "C" fn sdsdup(s: sds) -> sds {
    return sdsnewlen(s as *const libc::c_void, sdslen(s));
}
pub unsafe extern "C" fn sdsfree(mut s: sds) {
    if s.is_null() {
        return;
    }
    hi_free(
        s.offset(-(sdsHdrSize(*s.offset(-(1 as libc::c_int) as isize)) as isize))
            as *mut libc::c_void,
    );
}
pub unsafe extern "C" fn sdsupdatelen(mut s: sds) {
    let mut reallen: size_t = strlen(s as *const libc::c_char);
    sdssetlen(s, reallen);
}
pub unsafe extern "C" fn sdsclear(mut s: sds) {
    sdssetlen(s, 0 as libc::c_int as size_t);
    *s.offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
}
pub unsafe extern "C" fn sdsMakeRoomFor(mut s: sds, mut addlen: size_t) -> sds {
    let mut sh: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut newsh: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut avail: size_t = sdsavail(s);
    let mut len: size_t = 0;
    let mut newlen: size_t = 0;
    let mut reqlen: size_t = 0;
    let mut type_0: libc::c_char = 0;
    let mut oldtype: libc::c_char = (*s.offset(-(1 as libc::c_int) as isize)
        as libc::c_int & 7 as libc::c_int) as libc::c_char;
    let mut hdrlen: libc::c_int = 0;
    if avail >= addlen {
        return s;
    }
    len = sdslen(s);
    sh = s.offset(-(sdsHdrSize(oldtype) as isize)) as *mut libc::c_void;
    newlen = len.wrapping_add(addlen);
    reqlen = newlen;
    if newlen <= len {
        return 0 as sds;
    }
    if newlen < (1024 as libc::c_int * 1024 as libc::c_int) as libc::c_ulong {
        newlen = (newlen as libc::c_ulong)
            .wrapping_mul(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
    } else {
        newlen = (newlen as libc::c_ulong)
            .wrapping_add((1024 as libc::c_int * 1024 as libc::c_int) as libc::c_ulong)
            as size_t as size_t;
    }
    type_0 = sdsReqType(newlen);
    if type_0 as libc::c_int == 0 as libc::c_int {
        type_0 = 1 as libc::c_int as libc::c_char;
    }
    hdrlen = sdsHdrSize(type_0);
    if (hdrlen as libc::c_ulong)
        .wrapping_add(newlen)
        .wrapping_add(1 as libc::c_int as libc::c_ulong) <= reqlen
    {
        return 0 as sds;
    }
    if oldtype as libc::c_int == type_0 as libc::c_int {
        newsh = hi_realloc(
            sh,
            (hdrlen as libc::c_ulong)
                .wrapping_add(newlen)
                .wrapping_add(1 as libc::c_int as libc::c_ulong),
        );
        if newsh.is_null() {
            return 0 as sds;
        }
        s = (newsh as *mut libc::c_char).offset(hdrlen as isize);
    } else {
        newsh = hi_malloc(
            (hdrlen as libc::c_ulong)
                .wrapping_add(newlen)
                .wrapping_add(1 as libc::c_int as libc::c_ulong),
        );
        if newsh.is_null() {
            return 0 as sds;
        }
        memcpy(
            (newsh as *mut libc::c_char).offset(hdrlen as isize) as *mut libc::c_void,
            s as *const libc::c_void,
            len.wrapping_add(1 as libc::c_int as libc::c_ulong),
        );
        hi_free(sh);
        s = (newsh as *mut libc::c_char).offset(hdrlen as isize);
        *s.offset(-(1 as libc::c_int) as isize) = type_0;
        sdssetlen(s, len);
    }
    sdssetalloc(s, newlen);
    return s;
}
pub unsafe extern "C" fn sdsRemoveFreeSpace(mut s: sds) -> sds {
    let mut sh: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut newsh: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut type_0: libc::c_char = 0;
    let mut oldtype: libc::c_char = (*s.offset(-(1 as libc::c_int) as isize)
        as libc::c_int & 7 as libc::c_int) as libc::c_char;
    let mut hdrlen: libc::c_int = 0;
    let mut len: size_t = sdslen(s);
    sh = s.offset(-(sdsHdrSize(oldtype) as isize)) as *mut libc::c_void;
    type_0 = sdsReqType(len);
    hdrlen = sdsHdrSize(type_0);
    if oldtype as libc::c_int == type_0 as libc::c_int {
        newsh = hi_realloc(
            sh,
            (hdrlen as libc::c_ulong)
                .wrapping_add(len)
                .wrapping_add(1 as libc::c_int as libc::c_ulong),
        );
        if newsh.is_null() {
            return 0 as sds;
        }
        s = (newsh as *mut libc::c_char).offset(hdrlen as isize);
    } else {
        newsh = hi_malloc(
            (hdrlen as libc::c_ulong)
                .wrapping_add(len)
                .wrapping_add(1 as libc::c_int as libc::c_ulong),
        );
        if newsh.is_null() {
            return 0 as sds;
        }
        memcpy(
            (newsh as *mut libc::c_char).offset(hdrlen as isize) as *mut libc::c_void,
            s as *const libc::c_void,
            len.wrapping_add(1 as libc::c_int as libc::c_ulong),
        );
        hi_free(sh);
        s = (newsh as *mut libc::c_char).offset(hdrlen as isize);
        *s.offset(-(1 as libc::c_int) as isize) = type_0;
        sdssetlen(s, len);
    }
    sdssetalloc(s, len);
    return s;
}
pub unsafe extern "C" fn sdsAllocSize(mut s: sds) -> size_t {
    let mut alloc: size_t = sdsalloc(s);
    return (sdsHdrSize(*s.offset(-(1 as libc::c_int) as isize)) as libc::c_ulong)
        .wrapping_add(alloc)
        .wrapping_add(1 as libc::c_int as libc::c_ulong);
}
pub unsafe extern "C" fn sdsAllocPtr(mut s: sds) -> *mut libc::c_void {
    return s.offset(-(sdsHdrSize(*s.offset(-(1 as libc::c_int) as isize)) as isize))
        as *mut libc::c_void;
}
pub unsafe extern "C" fn sdsIncrLen(mut s: sds, mut incr: libc::c_int) {
    let mut flags: libc::c_uchar = *s.offset(-(1 as libc::c_int) as isize)
        as libc::c_uchar;
    let mut len: size_t = 0;
    match flags as libc::c_int & 7 as libc::c_int {
        0 => {
            let mut fp: *mut libc::c_uchar = (s as *mut libc::c_uchar)
                .offset(-(1 as libc::c_int as isize));
            let mut oldlen: libc::c_uchar = (flags as libc::c_int >> 3 as libc::c_int)
                as libc::c_uchar;
            if incr > 0 as libc::c_int
                && oldlen as libc::c_int + incr < 32 as libc::c_int
                || incr < 0 as libc::c_int
                    && oldlen as libc::c_uint >= -incr as libc::c_uint
            {} else {
                __assert_fail(
                    b"(incr > 0 && oldlen+incr < 32) || (incr < 0 && oldlen >= (unsigned int)(-incr))\0"
                        as *const u8 as *const libc::c_char,
                    b"sds.c\0" as *const u8 as *const libc::c_char,
                    324 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 26],
                        &[libc::c_char; 26],
                    >(b"void sdsIncrLen(sds, int)\0"))
                        .as_ptr(),
                );
            };
            *fp = (0 as libc::c_int | oldlen as libc::c_int + incr << 3 as libc::c_int)
                as libc::c_uchar;
            len = (oldlen as libc::c_int + incr) as size_t;
        }
        1 => {
            let mut sh: *mut sdshdr8 = s
                .offset(-(::std::mem::size_of::<sdshdr8>() as libc::c_ulong as isize))
                as *mut sdshdr8;
            if incr >= 0 as libc::c_int
                && (*sh).alloc as libc::c_int - (*sh).len as libc::c_int >= incr
                || incr < 0 as libc::c_int
                    && (*sh).len as libc::c_uint >= -incr as libc::c_uint
            {} else {
                __assert_fail(
                    b"(incr >= 0 && sh->alloc-sh->len >= incr) || (incr < 0 && sh->len >= (unsigned int)(-incr))\0"
                        as *const u8 as *const libc::c_char,
                    b"sds.c\0" as *const u8 as *const libc::c_char,
                    331 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 26],
                        &[libc::c_char; 26],
                    >(b"void sdsIncrLen(sds, int)\0"))
                        .as_ptr(),
                );
            };
            (*sh).len = ((*sh).len as libc::c_int + incr) as uint8_t;
            len = (*sh).len as size_t;
        }
        2 => {
            let mut sh_0: *mut sdshdr16 = s
                .offset(-(::std::mem::size_of::<sdshdr16>() as libc::c_ulong as isize))
                as *mut sdshdr16;
            if incr >= 0 as libc::c_int
                && (*sh_0).alloc as libc::c_int - (*sh_0).len as libc::c_int >= incr
                || incr < 0 as libc::c_int
                    && (*sh_0).len as libc::c_uint >= -incr as libc::c_uint
            {} else {
                __assert_fail(
                    b"(incr >= 0 && sh->alloc-sh->len >= incr) || (incr < 0 && sh->len >= (unsigned int)(-incr))\0"
                        as *const u8 as *const libc::c_char,
                    b"sds.c\0" as *const u8 as *const libc::c_char,
                    337 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 26],
                        &[libc::c_char; 26],
                    >(b"void sdsIncrLen(sds, int)\0"))
                        .as_ptr(),
                );
            };
            (*sh_0).len = ((*sh_0).len as libc::c_int + incr) as uint16_t;
            len = (*sh_0).len as size_t;
        }
        3 => {
            let mut sh_1: *mut sdshdr32 = s
                .offset(-(::std::mem::size_of::<sdshdr32>() as libc::c_ulong as isize))
                as *mut sdshdr32;
            if incr >= 0 as libc::c_int
                && ((*sh_1).alloc).wrapping_sub((*sh_1).len) >= incr as libc::c_uint
                || incr < 0 as libc::c_int && (*sh_1).len >= -incr as libc::c_uint
            {} else {
                __assert_fail(
                    b"(incr >= 0 && sh->alloc-sh->len >= (unsigned int)incr) || (incr < 0 && sh->len >= (unsigned int)(-incr))\0"
                        as *const u8 as *const libc::c_char,
                    b"sds.c\0" as *const u8 as *const libc::c_char,
                    343 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 26],
                        &[libc::c_char; 26],
                    >(b"void sdsIncrLen(sds, int)\0"))
                        .as_ptr(),
                );
            };
            (*sh_1)
                .len = ((*sh_1).len as libc::c_uint).wrapping_add(incr as libc::c_uint)
                as uint32_t as uint32_t;
            len = (*sh_1).len as size_t;
        }
        4 => {
            let mut sh_2: *mut sdshdr64 = s
                .offset(-(::std::mem::size_of::<sdshdr64>() as libc::c_ulong as isize))
                as *mut sdshdr64;
            if incr >= 0 as libc::c_int
                && ((*sh_2).alloc).wrapping_sub((*sh_2).len) >= incr as uint64_t
                || incr < 0 as libc::c_int && (*sh_2).len >= -incr as uint64_t
            {} else {
                __assert_fail(
                    b"(incr >= 0 && sh->alloc-sh->len >= (uint64_t)incr) || (incr < 0 && sh->len >= (uint64_t)(-incr))\0"
                        as *const u8 as *const libc::c_char,
                    b"sds.c\0" as *const u8 as *const libc::c_char,
                    349 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 26],
                        &[libc::c_char; 26],
                    >(b"void sdsIncrLen(sds, int)\0"))
                        .as_ptr(),
                );
            };
            (*sh_2)
                .len = ((*sh_2).len as libc::c_ulong).wrapping_add(incr as libc::c_ulong)
                as uint64_t as uint64_t;
            len = (*sh_2).len;
        }
        _ => {
            len = 0 as libc::c_int as size_t;
        }
    }
    *s.offset(len as isize) = '\0' as i32 as libc::c_char;
}
pub unsafe extern "C" fn sdsgrowzero(mut s: sds, mut len: size_t) -> sds {
    let mut curlen: size_t = sdslen(s);
    if len <= curlen {
        return s;
    }
    s = sdsMakeRoomFor(s, len.wrapping_sub(curlen));
    if s.is_null() {
        return 0 as sds;
    }
    memset(
        s.offset(curlen as isize) as *mut libc::c_void,
        0 as libc::c_int,
        len.wrapping_sub(curlen).wrapping_add(1 as libc::c_int as libc::c_ulong),
    );
    sdssetlen(s, len);
    return s;
}
pub unsafe extern "C" fn sdscatlen(
    mut s: sds,
    mut t: *const libc::c_void,
    mut len: size_t,
) -> sds {
    let mut curlen: size_t = sdslen(s);
    s = sdsMakeRoomFor(s, len);
    if s.is_null() {
        return 0 as sds;
    }
    memcpy(s.offset(curlen as isize) as *mut libc::c_void, t, len);
    sdssetlen(s, curlen.wrapping_add(len));
    *s.offset(curlen.wrapping_add(len) as isize) = '\0' as i32 as libc::c_char;
    return s;
}
pub unsafe extern "C" fn sdscat(mut s: sds, mut t: *const libc::c_char) -> sds {
    return sdscatlen(s, t as *const libc::c_void, strlen(t));
}
pub unsafe extern "C" fn sdscatsds(mut s: sds, t: sds) -> sds {
    return sdscatlen(s, t as *const libc::c_void, sdslen(t));
}
pub unsafe extern "C" fn sdscpylen(
    mut s: sds,
    mut t: *const libc::c_char,
    mut len: size_t,
) -> sds {
    if sdsalloc(s) < len {
        s = sdsMakeRoomFor(s, len.wrapping_sub(sdslen(s)));
        if s.is_null() {
            return 0 as sds;
        }
    }
    memcpy(s as *mut libc::c_void, t as *const libc::c_void, len);
    *s.offset(len as isize) = '\0' as i32 as libc::c_char;
    sdssetlen(s, len);
    return s;
}
pub unsafe extern "C" fn sdscpy(mut s: sds, mut t: *const libc::c_char) -> sds {
    return sdscpylen(s, t, strlen(t));
}
pub unsafe extern "C" fn sdsll2str(
    mut s: *mut libc::c_char,
    mut value: libc::c_longlong,
) -> libc::c_int {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut aux: libc::c_char = 0;
    let mut v: libc::c_ulonglong = 0;
    let mut l: size_t = 0;
    v = (if value < 0 as libc::c_int as libc::c_longlong { -value } else { value })
        as libc::c_ulonglong;
    p = s;
    loop {
        let fresh4 = p;
        p = p.offset(1);
        *fresh4 = ('0' as i32 as libc::c_ulonglong)
            .wrapping_add(v.wrapping_rem(10 as libc::c_int as libc::c_ulonglong))
            as libc::c_char;
        v = v.wrapping_div(10 as libc::c_int as libc::c_ulonglong);
        if !(v != 0) {
            break;
        }
    }
    if value < 0 as libc::c_int as libc::c_longlong {
        let fresh5 = p;
        p = p.offset(1);
        *fresh5 = '-' as i32 as libc::c_char;
    }
    l = p.offset_from(s) as libc::c_long as size_t;
    *p = '\0' as i32 as libc::c_char;
    p = p.offset(-1);
    p;
    while s < p {
        aux = *s;
        *s = *p;
        *p = aux;
        s = s.offset(1);
        s;
        p = p.offset(-1);
        p;
    }
    return l as libc::c_int;
}
pub unsafe extern "C" fn sdsull2str(
    mut s: *mut libc::c_char,
    mut v: libc::c_ulonglong,
) -> libc::c_int {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut aux: libc::c_char = 0;
    let mut l: size_t = 0;
    p = s;
    loop {
        let fresh6 = p;
        p = p.offset(1);
        *fresh6 = ('0' as i32 as libc::c_ulonglong)
            .wrapping_add(v.wrapping_rem(10 as libc::c_int as libc::c_ulonglong))
            as libc::c_char;
        v = v.wrapping_div(10 as libc::c_int as libc::c_ulonglong);
        if !(v != 0) {
            break;
        }
    }
    l = p.offset_from(s) as libc::c_long as size_t;
    *p = '\0' as i32 as libc::c_char;
    p = p.offset(-1);
    p;
    while s < p {
        aux = *s;
        *s = *p;
        *p = aux;
        s = s.offset(1);
        s;
        p = p.offset(-1);
        p;
    }
    return l as libc::c_int;
}
pub unsafe extern "C" fn sdsfromlonglong(mut value: libc::c_longlong) -> sds {
    let mut buf: [libc::c_char; 21] = [0; 21];
    let mut len: libc::c_int = sdsll2str(buf.as_mut_ptr(), value);
    return sdsnewlen(buf.as_mut_ptr() as *const libc::c_void, len as size_t);
}
pub unsafe extern "C" fn sdscatvprintf(
    mut s: sds,
    mut fmt: *const libc::c_char,
    mut ap: ::std::ffi::VaList,
) -> sds {
    let mut cpy: ::std::ffi::VaListImpl;
    let mut staticbuf: [libc::c_char; 1024] = [0; 1024];
    let mut buf: *mut libc::c_char = staticbuf.as_mut_ptr();
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut buflen: size_t = (strlen(fmt))
        .wrapping_mul(2 as libc::c_int as libc::c_ulong);
    if buflen > ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong {
        buf = hi_malloc(buflen) as *mut libc::c_char;
        if buf.is_null() {
            return 0 as sds;
        }
    } else {
        buflen = ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong;
    }
    loop {
        *buf
            .offset(
                buflen.wrapping_sub(2 as libc::c_int as libc::c_ulong) as isize,
            ) = '\0' as i32 as libc::c_char;
        cpy = ap.clone();
        vsnprintf(buf, buflen, fmt, cpy.as_va_list());
        if !(*buf.offset(buflen.wrapping_sub(2 as libc::c_int as libc::c_ulong) as isize)
            as libc::c_int != '\0' as i32)
        {
            break;
        }
        if buf != staticbuf.as_mut_ptr() {
            hi_free(buf as *mut libc::c_void);
        }
        buflen = (buflen as libc::c_ulong)
            .wrapping_mul(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
        buf = hi_malloc(buflen) as *mut libc::c_char;
        if buf.is_null() {
            return 0 as sds;
        }
    }
    t = sdscat(s, buf);
    if buf != staticbuf.as_mut_ptr() {
        hi_free(buf as *mut libc::c_void);
    }
    return t;
}
pub unsafe extern "C" fn sdscatprintf(
    mut s: sds,
    mut fmt: *const libc::c_char,
    mut args: ...
) -> sds {
    let mut ap: ::std::ffi::VaListImpl;
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    ap = args.clone();
    t = sdscatvprintf(s, fmt, ap.as_va_list());
    return t;
}
pub unsafe extern "C" fn sdscatfmt(
    mut s: sds,
    mut fmt: *const libc::c_char,
    mut args: ...
) -> sds {
    let mut current_block: u64;
    let mut f: *const libc::c_char = fmt;
    let mut i: libc::c_long = 0;
    let mut ap: ::std::ffi::VaListImpl;
    ap = args.clone();
    i = sdslen(s) as libc::c_long;
    loop {
        if !(*f != 0) {
            current_block = 15004371738079956865;
            break;
        }
        let mut next: libc::c_char = 0;
        let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut l: size_t = 0;
        let mut num: libc::c_longlong = 0;
        let mut unum: libc::c_ulonglong = 0;
        if sdsavail(s) == 0 as libc::c_int as libc::c_ulong {
            s = sdsMakeRoomFor(s, 1 as libc::c_int as size_t);
            if s.is_null() {
                current_block = 6174974146017752131;
                break;
            }
        }
        match *f as libc::c_int {
            37 => {
                next = *f.offset(1 as libc::c_int as isize);
                f = f.offset(1);
                f;
                match next as libc::c_int {
                    115 | 83 => {
                        str = ap.arg::<*mut libc::c_char>();
                        l = if next as libc::c_int == 's' as i32 {
                            strlen(str)
                        } else {
                            sdslen(str)
                        };
                        if sdsavail(s) < l {
                            s = sdsMakeRoomFor(s, l);
                            if s.is_null() {
                                current_block = 6174974146017752131;
                                break;
                            }
                        }
                        memcpy(
                            s.offset(i as isize) as *mut libc::c_void,
                            str as *const libc::c_void,
                            l,
                        );
                        sdsinclen(s, l);
                        i = (i as libc::c_ulong).wrapping_add(l) as libc::c_long
                            as libc::c_long;
                    }
                    105 | 73 => {
                        if next as libc::c_int == 'i' as i32 {
                            num = ap.arg::<libc::c_int>() as libc::c_longlong;
                        } else {
                            num = ap.arg::<libc::c_longlong>();
                        }
                        let mut buf: [libc::c_char; 21] = [0; 21];
                        l = sdsll2str(buf.as_mut_ptr(), num) as size_t;
                        if sdsavail(s) < l {
                            s = sdsMakeRoomFor(s, l);
                            if s.is_null() {
                                current_block = 6174974146017752131;
                                break;
                            }
                        }
                        memcpy(
                            s.offset(i as isize) as *mut libc::c_void,
                            buf.as_mut_ptr() as *const libc::c_void,
                            l,
                        );
                        sdsinclen(s, l);
                        i = (i as libc::c_ulong).wrapping_add(l) as libc::c_long
                            as libc::c_long;
                    }
                    117 | 85 => {
                        if next as libc::c_int == 'u' as i32 {
                            unum = ap.arg::<libc::c_uint>() as libc::c_ulonglong;
                        } else {
                            unum = ap.arg::<libc::c_ulonglong>();
                        }
                        let mut buf_0: [libc::c_char; 21] = [0; 21];
                        l = sdsull2str(buf_0.as_mut_ptr(), unum) as size_t;
                        if sdsavail(s) < l {
                            s = sdsMakeRoomFor(s, l);
                            if s.is_null() {
                                current_block = 6174974146017752131;
                                break;
                            }
                        }
                        memcpy(
                            s.offset(i as isize) as *mut libc::c_void,
                            buf_0.as_mut_ptr() as *const libc::c_void,
                            l,
                        );
                        sdsinclen(s, l);
                        i = (i as libc::c_ulong).wrapping_add(l) as libc::c_long
                            as libc::c_long;
                    }
                    _ => {
                        let fresh7 = i;
                        i = i + 1;
                        *s.offset(fresh7 as isize) = next;
                        sdsinclen(s, 1 as libc::c_int as size_t);
                    }
                }
            }
            _ => {
                let fresh8 = i;
                i = i + 1;
                *s.offset(fresh8 as isize) = *f;
                sdsinclen(s, 1 as libc::c_int as size_t);
            }
        }
        f = f.offset(1);
        f;
    }
    match current_block {
        6174974146017752131 => return 0 as sds,
        _ => {
            *s.offset(i as isize) = '\0' as i32 as libc::c_char;
            return s;
        }
    };
}
pub unsafe extern "C" fn sdstrim(mut s: sds, mut cset: *const libc::c_char) -> sds {
    let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ep: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: size_t = 0;
    sp = s;
    end = s.offset(sdslen(s) as isize).offset(-(1 as libc::c_int as isize));
    ep = end;
    while sp <= end && !(strchr(cset, *sp as libc::c_int)).is_null() {
        sp = sp.offset(1);
        sp;
    }
    while ep > sp && !(strchr(cset, *ep as libc::c_int)).is_null() {
        ep = ep.offset(-1);
        ep;
    }
    len = (if sp > ep {
        0 as libc::c_int as libc::c_long
    } else {
        ep.offset_from(sp) as libc::c_long + 1 as libc::c_int as libc::c_long
    }) as size_t;
    if s != sp {
        memmove(s as *mut libc::c_void, sp as *const libc::c_void, len);
    }
    *s.offset(len as isize) = '\0' as i32 as libc::c_char;
    sdssetlen(s, len);
    return s;
}
pub unsafe extern "C" fn sdsrange(
    mut s: sds,
    mut start: ssize_t,
    mut end: ssize_t,
) -> libc::c_int {
    let mut newlen: size_t = 0;
    let mut len: size_t = sdslen(s);
    if len > 9223372036854775807 as libc::c_long as libc::c_ulong {
        return -(1 as libc::c_int);
    }
    if len == 0 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int;
    }
    if start < 0 as libc::c_int as libc::c_long {
        start = len.wrapping_add(start as libc::c_ulong) as ssize_t;
        if start < 0 as libc::c_int as libc::c_long {
            start = 0 as libc::c_int as ssize_t;
        }
    }
    if end < 0 as libc::c_int as libc::c_long {
        end = len.wrapping_add(end as libc::c_ulong) as ssize_t;
        if end < 0 as libc::c_int as libc::c_long {
            end = 0 as libc::c_int as ssize_t;
        }
    }
    newlen = (if start > end {
        0 as libc::c_int as libc::c_long
    } else {
        end - start + 1 as libc::c_int as libc::c_long
    }) as size_t;
    if newlen != 0 as libc::c_int as libc::c_ulong {
        if start >= len as ssize_t {
            newlen = 0 as libc::c_int as size_t;
        } else if end >= len as ssize_t {
            end = len.wrapping_sub(1 as libc::c_int as libc::c_ulong) as ssize_t;
            newlen = (if start > end {
                0 as libc::c_int as libc::c_long
            } else {
                end - start + 1 as libc::c_int as libc::c_long
            }) as size_t;
        }
    } else {
        start = 0 as libc::c_int as ssize_t;
    }
    if start != 0 && newlen != 0 {
        memmove(
            s as *mut libc::c_void,
            s.offset(start as isize) as *const libc::c_void,
            newlen,
        );
    }
    *s.offset(newlen as isize) = 0 as libc::c_int as libc::c_char;
    sdssetlen(s, newlen);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn sdstolower(mut s: sds) {
    let mut len: size_t = sdslen(s);
    let mut j: size_t = 0;
    j = 0 as libc::c_int as size_t;
    while j < len {
        *s
            .offset(
                j as isize,
            ) = ({
            let mut __res: libc::c_int = 0;
            if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                > 1 as libc::c_int as libc::c_ulong
            {
                if 0 != 0 {
                    let mut __c: libc::c_int = *s.offset(j as isize) as libc::c_int;
                    __res = if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                        __c
                    } else {
                        *(*__ctype_tolower_loc()).offset(__c as isize)
                    };
                } else {
                    __res = tolower(*s.offset(j as isize) as libc::c_int);
                }
            } else {
                __res = *(*__ctype_tolower_loc())
                    .offset(*s.offset(j as isize) as libc::c_int as isize);
            }
            __res
        }) as libc::c_char;
        j = j.wrapping_add(1);
        j;
    }
}
pub unsafe extern "C" fn sdstoupper(mut s: sds) {
    let mut len: size_t = sdslen(s);
    let mut j: size_t = 0;
    j = 0 as libc::c_int as size_t;
    while j < len {
        *s
            .offset(
                j as isize,
            ) = ({
            let mut __res: libc::c_int = 0;
            if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                > 1 as libc::c_int as libc::c_ulong
            {
                if 0 != 0 {
                    let mut __c: libc::c_int = *s.offset(j as isize) as libc::c_int;
                    __res = if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                        __c
                    } else {
                        *(*__ctype_toupper_loc()).offset(__c as isize)
                    };
                } else {
                    __res = toupper(*s.offset(j as isize) as libc::c_int);
                }
            } else {
                __res = *(*__ctype_toupper_loc())
                    .offset(*s.offset(j as isize) as libc::c_int as isize);
            }
            __res
        }) as libc::c_char;
        j = j.wrapping_add(1);
        j;
    }
}
pub unsafe extern "C" fn sdscmp(s1: sds, s2: sds) -> libc::c_int {
    let mut l1: size_t = 0;
    let mut l2: size_t = 0;
    let mut minlen: size_t = 0;
    let mut cmp: libc::c_int = 0;
    l1 = sdslen(s1);
    l2 = sdslen(s2);
    minlen = if l1 < l2 { l1 } else { l2 };
    cmp = memcmp(s1 as *const libc::c_void, s2 as *const libc::c_void, minlen);
    if cmp == 0 as libc::c_int {
        return l1.wrapping_sub(l2) as libc::c_int;
    }
    return cmp;
}
pub unsafe extern "C" fn sdssplitlen(
    mut s: *const libc::c_char,
    mut len: libc::c_int,
    mut sep: *const libc::c_char,
    mut seplen: libc::c_int,
    mut count: *mut libc::c_int,
) -> *mut sds {
    let mut current_block: u64;
    let mut elements: libc::c_int = 0 as libc::c_int;
    let mut slots: libc::c_int = 5 as libc::c_int;
    let mut start: libc::c_int = 0 as libc::c_int;
    let mut j: libc::c_int = 0;
    let mut tokens: *mut sds = 0 as *mut sds;
    if seplen < 1 as libc::c_int || len < 0 as libc::c_int {
        return 0 as *mut sds;
    }
    tokens = hi_malloc(
        (::std::mem::size_of::<sds>() as libc::c_ulong)
            .wrapping_mul(slots as libc::c_ulong),
    ) as *mut sds;
    if tokens.is_null() {
        return 0 as *mut sds;
    }
    if len == 0 as libc::c_int {
        *count = 0 as libc::c_int;
        return tokens;
    }
    j = 0 as libc::c_int;
    loop {
        if !(j < len - (seplen - 1 as libc::c_int)) {
            current_block = 15904375183555213903;
            break;
        }
        if slots < elements + 2 as libc::c_int {
            let mut newtokens: *mut sds = 0 as *mut sds;
            slots *= 2 as libc::c_int;
            newtokens = hi_realloc(
                tokens as *mut libc::c_void,
                (::std::mem::size_of::<sds>() as libc::c_ulong)
                    .wrapping_mul(slots as libc::c_ulong),
            ) as *mut sds;
            if newtokens.is_null() {
                current_block = 9540916681445002721;
                break;
            }
            tokens = newtokens;
        }
        if seplen == 1 as libc::c_int
            && *s.offset(j as isize) as libc::c_int
                == *sep.offset(0 as libc::c_int as isize) as libc::c_int
            || memcmp(
                s.offset(j as isize) as *const libc::c_void,
                sep as *const libc::c_void,
                seplen as libc::c_ulong,
            ) == 0 as libc::c_int
        {
            let ref mut fresh9 = *tokens.offset(elements as isize);
            *fresh9 = sdsnewlen(
                s.offset(start as isize) as *const libc::c_void,
                (j - start) as size_t,
            );
            if (*tokens.offset(elements as isize)).is_null() {
                current_block = 9540916681445002721;
                break;
            }
            elements += 1;
            elements;
            start = j + seplen;
            j = j + seplen - 1 as libc::c_int;
        }
        j += 1;
        j;
    }
    match current_block {
        15904375183555213903 => {
            let ref mut fresh10 = *tokens.offset(elements as isize);
            *fresh10 = sdsnewlen(
                s.offset(start as isize) as *const libc::c_void,
                (len - start) as size_t,
            );
            if !(*tokens.offset(elements as isize)).is_null() {
                elements += 1;
                elements;
                *count = elements;
                return tokens;
            }
        }
        _ => {}
    }
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < elements {
        sdsfree(*tokens.offset(i as isize));
        i += 1;
        i;
    }
    hi_free(tokens as *mut libc::c_void);
    *count = 0 as libc::c_int;
    return 0 as *mut sds;
}
pub unsafe extern "C" fn sdsfreesplitres(mut tokens: *mut sds, mut count: libc::c_int) {
    if tokens.is_null() {
        return;
    }
    loop {
        let fresh11 = count;
        count = count - 1;
        if !(fresh11 != 0) {
            break;
        }
        sdsfree(*tokens.offset(count as isize));
    }
    hi_free(tokens as *mut libc::c_void);
}
pub unsafe extern "C" fn sdscatrepr(
    mut s: sds,
    mut p: *const libc::c_char,
    mut len: size_t,
) -> sds {
    s = sdscatlen(
        s,
        b"\"\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        1 as libc::c_int as size_t,
    );
    loop {
        let fresh12 = len;
        len = len.wrapping_sub(1);
        if !(fresh12 != 0) {
            break;
        }
        match *p as libc::c_int {
            92 | 34 => {
                s = sdscatprintf(
                    s,
                    b"\\%c\0" as *const u8 as *const libc::c_char,
                    *p as libc::c_int,
                );
            }
            10 => {
                s = sdscatlen(
                    s,
                    b"\\n\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                    2 as libc::c_int as size_t,
                );
            }
            13 => {
                s = sdscatlen(
                    s,
                    b"\\r\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                    2 as libc::c_int as size_t,
                );
            }
            9 => {
                s = sdscatlen(
                    s,
                    b"\\t\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                    2 as libc::c_int as size_t,
                );
            }
            7 => {
                s = sdscatlen(
                    s,
                    b"\\a\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                    2 as libc::c_int as size_t,
                );
            }
            8 => {
                s = sdscatlen(
                    s,
                    b"\\b\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                    2 as libc::c_int as size_t,
                );
            }
            _ => {
                if *(*__ctype_b_loc()).offset(*p as libc::c_int as isize) as libc::c_int
                    & _ISprint as libc::c_int as libc::c_ushort as libc::c_int != 0
                {
                    s = sdscatprintf(
                        s,
                        b"%c\0" as *const u8 as *const libc::c_char,
                        *p as libc::c_int,
                    );
                } else {
                    s = sdscatprintf(
                        s,
                        b"\\x%02x\0" as *const u8 as *const libc::c_char,
                        *p as libc::c_uchar as libc::c_int,
                    );
                }
            }
        }
        p = p.offset(1);
        p;
    }
    return sdscatlen(
        s,
        b"\"\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        1 as libc::c_int as size_t,
    );
}
pub unsafe extern "C" fn hex_digit_to_int(mut c: libc::c_char) -> libc::c_int {
    match c as libc::c_int {
        48 => return 0 as libc::c_int,
        49 => return 1 as libc::c_int,
        50 => return 2 as libc::c_int,
        51 => return 3 as libc::c_int,
        52 => return 4 as libc::c_int,
        53 => return 5 as libc::c_int,
        54 => return 6 as libc::c_int,
        55 => return 7 as libc::c_int,
        56 => return 8 as libc::c_int,
        57 => return 9 as libc::c_int,
        97 | 65 => return 10 as libc::c_int,
        98 | 66 => return 11 as libc::c_int,
        99 | 67 => return 12 as libc::c_int,
        100 | 68 => return 13 as libc::c_int,
        101 | 69 => return 14 as libc::c_int,
        102 | 70 => return 15 as libc::c_int,
        _ => return 0 as libc::c_int,
    };
}
pub unsafe extern "C" fn sdssplitargs(
    mut line: *const libc::c_char,
    mut argc: *mut libc::c_int,
) -> *mut sds {
    let mut p: *const libc::c_char = line;
    let mut current: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut vector: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    *argc = 0 as libc::c_int;
    's_13: loop {
        while *p as libc::c_int != 0
            && *(*__ctype_b_loc()).offset(*p as libc::c_int as isize) as libc::c_int
                & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
        {
            p = p.offset(1);
            p;
        }
        if *p != 0 {
            let mut inq: libc::c_int = 0 as libc::c_int;
            let mut insq: libc::c_int = 0 as libc::c_int;
            let mut done: libc::c_int = 0 as libc::c_int;
            if current.is_null() {
                current = sdsempty();
            }
            while done == 0 {
                if inq != 0 {
                    if *p as libc::c_int == '\\' as i32
                        && *p.offset(1 as libc::c_int as isize) as libc::c_int
                            == 'x' as i32
                        && *(*__ctype_b_loc())
                            .offset(
                                *p.offset(2 as libc::c_int as isize) as libc::c_int as isize,
                            ) as libc::c_int
                            & _ISxdigit as libc::c_int as libc::c_ushort as libc::c_int
                            != 0
                        && *(*__ctype_b_loc())
                            .offset(
                                *p.offset(3 as libc::c_int as isize) as libc::c_int as isize,
                            ) as libc::c_int
                            & _ISxdigit as libc::c_int as libc::c_ushort as libc::c_int
                            != 0
                    {
                        let mut byte: libc::c_uchar = 0;
                        byte = (hex_digit_to_int(*p.offset(2 as libc::c_int as isize))
                            * 16 as libc::c_int
                            + hex_digit_to_int(*p.offset(3 as libc::c_int as isize)))
                            as libc::c_uchar;
                        current = sdscatlen(
                            current,
                            &mut byte as *mut libc::c_uchar as *mut libc::c_char
                                as *const libc::c_void,
                            1 as libc::c_int as size_t,
                        );
                        p = p.offset(3 as libc::c_int as isize);
                    } else if *p as libc::c_int == '\\' as i32
                        && *p.offset(1 as libc::c_int as isize) as libc::c_int != 0
                    {
                        let mut c: libc::c_char = 0;
                        p = p.offset(1);
                        p;
                        match *p as libc::c_int {
                            110 => {
                                c = '\n' as i32 as libc::c_char;
                            }
                            114 => {
                                c = '\r' as i32 as libc::c_char;
                            }
                            116 => {
                                c = '\t' as i32 as libc::c_char;
                            }
                            98 => {
                                c = '\u{8}' as i32 as libc::c_char;
                            }
                            97 => {
                                c = '\u{7}' as i32 as libc::c_char;
                            }
                            _ => {
                                c = *p;
                            }
                        }
                        current = sdscatlen(
                            current,
                            &mut c as *mut libc::c_char as *const libc::c_void,
                            1 as libc::c_int as size_t,
                        );
                    } else if *p as libc::c_int == '"' as i32 {
                        if *p.offset(1 as libc::c_int as isize) as libc::c_int != 0
                            && *(*__ctype_b_loc())
                                .offset(
                                    *p.offset(1 as libc::c_int as isize) as libc::c_int as isize,
                                ) as libc::c_int
                                & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                                == 0
                        {
                            break 's_13;
                        }
                        done = 1 as libc::c_int;
                    } else if *p == 0 {
                        break 's_13;
                    } else {
                        current = sdscatlen(
                            current,
                            p as *const libc::c_void,
                            1 as libc::c_int as size_t,
                        );
                    }
                } else if insq != 0 {
                    if *p as libc::c_int == '\\' as i32
                        && *p.offset(1 as libc::c_int as isize) as libc::c_int
                            == '\'' as i32
                    {
                        p = p.offset(1);
                        p;
                        current = sdscatlen(
                            current,
                            b"'\0" as *const u8 as *const libc::c_char
                                as *const libc::c_void,
                            1 as libc::c_int as size_t,
                        );
                    } else if *p as libc::c_int == '\'' as i32 {
                        if *p.offset(1 as libc::c_int as isize) as libc::c_int != 0
                            && *(*__ctype_b_loc())
                                .offset(
                                    *p.offset(1 as libc::c_int as isize) as libc::c_int as isize,
                                ) as libc::c_int
                                & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                                == 0
                        {
                            break 's_13;
                        }
                        done = 1 as libc::c_int;
                    } else {
                        if *p == 0 {
                            break 's_13;
                        }
                        current = sdscatlen(
                            current,
                            p as *const libc::c_void,
                            1 as libc::c_int as size_t,
                        );
                    }
                } else {
                    match *p as libc::c_int {
                        32 | 10 | 13 | 9 | 0 => {
                            done = 1 as libc::c_int;
                        }
                        34 => {
                            inq = 1 as libc::c_int;
                        }
                        39 => {
                            insq = 1 as libc::c_int;
                        }
                        _ => {
                            current = sdscatlen(
                                current,
                                p as *const libc::c_void,
                                1 as libc::c_int as size_t,
                            );
                        }
                    }
                }
                if *p != 0 {
                    p = p.offset(1);
                    p;
                }
            }
            let mut new_vector: *mut *mut libc::c_char = hi_realloc(
                vector as *mut libc::c_void,
                ((*argc + 1 as libc::c_int) as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                    ),
            ) as *mut *mut libc::c_char;
            if new_vector.is_null() {
                hi_free(vector as *mut libc::c_void);
                return 0 as *mut sds;
            }
            vector = new_vector;
            let ref mut fresh13 = *vector.offset(*argc as isize);
            *fresh13 = current;
            *argc += 1;
            *argc;
            current = 0 as *mut libc::c_char;
        } else {
            if vector.is_null() {
                vector = hi_malloc(
                    ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
                ) as *mut *mut libc::c_char;
            }
            return vector;
        }
    }
    loop {
        let fresh14 = *argc;
        *argc = *argc - 1;
        if !(fresh14 != 0) {
            break;
        }
        sdsfree(*vector.offset(*argc as isize));
    }
    hi_free(vector as *mut libc::c_void);
    if !current.is_null() {
        sdsfree(current);
    }
    *argc = 0 as libc::c_int;
    return 0 as *mut sds;
}
pub unsafe extern "C" fn sdsmapchars(
    mut s: sds,
    mut from: *const libc::c_char,
    mut to: *const libc::c_char,
    mut setlen: size_t,
) -> sds {
    let mut j: size_t = 0;
    let mut i: size_t = 0;
    let mut l: size_t = sdslen(s);
    j = 0 as libc::c_int as size_t;
    while j < l {
        i = 0 as libc::c_int as size_t;
        while i < setlen {
            if *s.offset(j as isize) as libc::c_int
                == *from.offset(i as isize) as libc::c_int
            {
                *s.offset(j as isize) = *to.offset(i as isize);
                break;
            } else {
                i = i.wrapping_add(1);
                i;
            }
        }
        j = j.wrapping_add(1);
        j;
    }
    return s;
}
pub unsafe extern "C" fn sdsjoin(
    mut argv: *mut *mut libc::c_char,
    mut argc: libc::c_int,
    mut sep: *mut libc::c_char,
) -> sds {
    let mut join: sds = sdsempty();
    let mut j: libc::c_int = 0;
    j = 0 as libc::c_int;
    while j < argc {
        join = sdscat(join, *argv.offset(j as isize));
        if j != argc - 1 as libc::c_int {
            join = sdscat(join, sep);
        }
        j += 1;
        j;
    }
    return join;
}
pub unsafe extern "C" fn sdsjoinsds(
    mut argv: *mut sds,
    mut argc: libc::c_int,
    mut sep: *const libc::c_char,
    mut seplen: size_t,
) -> sds {
    let mut join: sds = sdsempty();
    let mut j: libc::c_int = 0;
    j = 0 as libc::c_int;
    while j < argc {
        join = sdscatsds(join, *argv.offset(j as isize));
        if j != argc - 1 as libc::c_int {
            join = sdscatlen(join, sep as *const libc::c_void, seplen);
        }
        j += 1;
        j;
    }
    return join;
}
pub unsafe extern "C" fn sds_malloc(mut size: size_t) -> *mut libc::c_void {
    return hi_malloc(size);
}
pub unsafe extern "C" fn sds_realloc(
    mut ptr: *mut libc::c_void,
    mut size: size_t,
) -> *mut libc::c_void {
    return hi_realloc(ptr, size);
}
pub unsafe extern "C" fn sds_free(mut ptr: *mut libc::c_void) {
    hi_free(ptr);
}
