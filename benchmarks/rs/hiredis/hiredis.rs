use ::libc;
extern "C" {
    pub type dict;
    pub type sockaddr;
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
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __xpg_strerror_r(
        __errnum: libc::c_int,
        __buf: *mut libc::c_char,
        __buflen: size_t,
    ) -> libc::c_int;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn __errno_location() -> *mut libc::c_int;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sdscat(s: sds, t: *const libc::c_char) -> sds;
    fn redisReaderCreateWithFunctions(
        fn_0: *mut redisReplyObjectFunctions,
    ) -> *mut redisReader;
    fn redisReaderFree(r: *mut redisReader);
    fn redisReaderFeed(
        r: *mut redisReader,
        buf: *const libc::c_char,
        len: size_t,
    ) -> libc::c_int;
    fn redisReaderGetReply(
        r: *mut redisReader,
        reply: *mut *mut libc::c_void,
    ) -> libc::c_int;
    fn sdsempty() -> sds;
    fn sdsfree(s: sds);
    fn sdscatlen(s: sds, t: *const libc::c_void, len: size_t) -> sds;
    static mut hiredisAllocFns: hiredisAllocFuncs;
    fn sdscatvprintf(s: sds, fmt: *const libc::c_char, ap: ::std::ffi::VaList) -> sds;
    fn sdscatfmt(s: sds, fmt: *const libc::c_char, _: ...) -> sds;
    fn sdsrange(s: sds, start: ssize_t, end: ssize_t) -> libc::c_int;
    fn sdsMakeRoomFor(s: sds, addlen: size_t) -> sds;
    fn redisNetClose(c: *mut redisContext);
    fn redisNetRead(
        c: *mut redisContext,
        buf: *mut libc::c_char,
        bufcap: size_t,
    ) -> ssize_t;
    fn redisNetWrite(c: *mut redisContext) -> ssize_t;
    fn redisContextSetTimeout(c: *mut redisContext, tv: timeval) -> libc::c_int;
    fn redisContextConnectBindTcp(
        c: *mut redisContext,
        addr: *const libc::c_char,
        port: libc::c_int,
        timeout: *const timeval,
        source_addr: *const libc::c_char,
    ) -> libc::c_int;
    fn redisContextConnectUnix(
        c: *mut redisContext,
        path: *const libc::c_char,
        timeout: *const timeval,
    ) -> libc::c_int;
    fn redisKeepAlive(c: *mut redisContext, interval: libc::c_int) -> libc::c_int;
    fn redisContextSetTcpUserTimeout(
        c: *mut redisContext,
        timeout: libc::c_uint,
    ) -> libc::c_int;
    fn redisAsyncRead(ac: *mut redisAsyncContext);
    fn redisAsyncWrite(ac: *mut redisAsyncContext);
    fn redisContextUpdateConnectTimeout(
        c: *mut redisContext,
        timeout: *const timeval,
    ) -> libc::c_int;
    fn redisContextUpdateCommandTimeout(
        c: *mut redisContext,
        timeout: *const timeval,
    ) -> libc::c_int;
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
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
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
pub type va_list = __builtin_va_list;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type sds = *mut libc::c_char;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct redisAsyncContext {
    pub c: redisContext,
    pub err: libc::c_int,
    pub errstr: *mut libc::c_char,
    pub data: *mut libc::c_void,
    pub dataCleanup: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub ev: C2RustUnnamed_1,
    pub onDisconnect: Option::<redisDisconnectCallback>,
    pub onConnect: Option::<redisConnectCallback>,
    pub onConnectNC: Option::<redisConnectCallbackNC>,
    pub replies: redisCallbackList,
    pub saddr: *mut sockaddr,
    pub addrlen: size_t,
    pub sub: C2RustUnnamed_0,
    pub push_cb: Option::<redisAsyncPushFn>,
}
pub type redisAsyncPushFn = unsafe extern "C" fn(
    *mut redisAsyncContext,
    *mut libc::c_void,
) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub replies: redisCallbackList,
    pub channels: *mut dict,
    pub patterns: *mut dict,
    pub pending_unsubs: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct redisCallbackList {
    pub head: *mut redisCallback,
    pub tail: *mut redisCallback,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct redisCallback {
    pub next: *mut redisCallback,
    pub fn_0: Option::<redisCallbackFn>,
    pub pending_subs: libc::c_int,
    pub unsubscribe_sent: libc::c_int,
    pub privdata: *mut libc::c_void,
}
pub type redisCallbackFn = unsafe extern "C" fn(
    *mut redisAsyncContext,
    *mut libc::c_void,
    *mut libc::c_void,
) -> ();
pub type redisConnectCallbackNC = unsafe extern "C" fn(
    *mut redisAsyncContext,
    libc::c_int,
) -> ();
pub type redisConnectCallback = unsafe extern "C" fn(
    *const redisAsyncContext,
    libc::c_int,
) -> ();
pub type redisDisconnectCallback = unsafe extern "C" fn(
    *const redisAsyncContext,
    libc::c_int,
) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub data: *mut libc::c_void,
    pub addRead: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub delRead: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub addWrite: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub delWrite: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub cleanup: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub scheduleTimer: Option::<unsafe extern "C" fn(*mut libc::c_void, timeval) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct redisContext {
    pub funcs: *const redisContextFuncs,
    pub err: libc::c_int,
    pub errstr: [libc::c_char; 128],
    pub fd: redisFD,
    pub flags: libc::c_int,
    pub obuf: *mut libc::c_char,
    pub reader: *mut redisReader,
    pub connection_type: redisConnectionType,
    pub connect_timeout: *mut timeval,
    pub command_timeout: *mut timeval,
    pub tcp: C2RustUnnamed_3,
    pub unix_sock: C2RustUnnamed_2,
    pub saddr: *mut sockaddr,
    pub addrlen: size_t,
    pub privdata: *mut libc::c_void,
    pub free_privdata: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub privctx: *mut libc::c_void,
    pub push_cb: Option::<redisPushFn>,
}
pub type redisPushFn = unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub path: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub host: *mut libc::c_char,
    pub source_addr: *mut libc::c_char,
    pub port: libc::c_int,
}
pub type redisConnectionType = libc::c_uint;
pub const REDIS_CONN_USERFD: redisConnectionType = 2;
pub const REDIS_CONN_UNIX: redisConnectionType = 1;
pub const REDIS_CONN_TCP: redisConnectionType = 0;
pub type redisFD = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct redisContextFuncs {
    pub close: Option::<unsafe extern "C" fn(*mut redisContext) -> ()>,
    pub free_privctx: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub async_read: Option::<unsafe extern "C" fn(*mut redisAsyncContext) -> ()>,
    pub async_write: Option::<unsafe extern "C" fn(*mut redisAsyncContext) -> ()>,
    pub read: Option::<
        unsafe extern "C" fn(*mut redisContext, *mut libc::c_char, size_t) -> ssize_t,
    >,
    pub write: Option::<unsafe extern "C" fn(*mut redisContext) -> ssize_t>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct redisReply {
    pub type_0: libc::c_int,
    pub integer: libc::c_longlong,
    pub dval: libc::c_double,
    pub len: size_t,
    pub str_0: *mut libc::c_char,
    pub vtype: [libc::c_char; 4],
    pub elements: size_t,
    pub element: *mut *mut redisReply,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct redisOptions {
    pub type_0: libc::c_int,
    pub options: libc::c_int,
    pub connect_timeout: *const timeval,
    pub command_timeout: *const timeval,
    pub endpoint: C2RustUnnamed_4,
    pub privdata: *mut libc::c_void,
    pub free_privdata: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub push_cb: Option::<redisPushFn>,
    pub async_push_cb: Option::<redisAsyncPushFn>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
    pub tcp: C2RustUnnamed_5,
    pub unix_socket: *const libc::c_char,
    pub fd: redisFD,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub source_addr: *const libc::c_char,
    pub ip: *const libc::c_char,
    pub port: libc::c_int,
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
unsafe extern "C" fn hi_malloc(mut size: size_t) -> *mut libc::c_void {
    return (hiredisAllocFns.mallocFn).unwrap()(size);
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
unsafe extern "C" fn hi_free(mut ptr: *mut libc::c_void) {
    (hiredisAllocFns.freeFn).unwrap()(ptr);
}
static mut redisContextDefaultFuncs: redisContextFuncs = unsafe {
    {
        let mut init = redisContextFuncs {
            close: Some(redisNetClose as unsafe extern "C" fn(*mut redisContext) -> ()),
            free_privctx: None,
            async_read: Some(
                redisAsyncRead as unsafe extern "C" fn(*mut redisAsyncContext) -> (),
            ),
            async_write: Some(
                redisAsyncWrite as unsafe extern "C" fn(*mut redisAsyncContext) -> (),
            ),
            read: Some(
                redisNetRead
                    as unsafe extern "C" fn(
                        *mut redisContext,
                        *mut libc::c_char,
                        size_t,
                    ) -> ssize_t,
            ),
            write: Some(
                redisNetWrite as unsafe extern "C" fn(*mut redisContext) -> ssize_t,
            ),
        };
        init
    }
};
static mut defaultFunctions: redisReplyObjectFunctions = unsafe {
    {
        let mut init = redisReplyObjectFunctions {
            createString: Some(
                createStringObject
                    as unsafe extern "C" fn(
                        *const redisReadTask,
                        *mut libc::c_char,
                        size_t,
                    ) -> *mut libc::c_void,
            ),
            createArray: Some(
                createArrayObject
                    as unsafe extern "C" fn(
                        *const redisReadTask,
                        size_t,
                    ) -> *mut libc::c_void,
            ),
            createInteger: Some(
                createIntegerObject
                    as unsafe extern "C" fn(
                        *const redisReadTask,
                        libc::c_longlong,
                    ) -> *mut libc::c_void,
            ),
            createDouble: Some(
                createDoubleObject
                    as unsafe extern "C" fn(
                        *const redisReadTask,
                        libc::c_double,
                        *mut libc::c_char,
                        size_t,
                    ) -> *mut libc::c_void,
            ),
            createNil: Some(
                createNilObject
                    as unsafe extern "C" fn(*const redisReadTask) -> *mut libc::c_void,
            ),
            createBool: Some(
                createBoolObject
                    as unsafe extern "C" fn(
                        *const redisReadTask,
                        libc::c_int,
                    ) -> *mut libc::c_void,
            ),
            freeObject: Some(
                freeReplyObject as unsafe extern "C" fn(*mut libc::c_void) -> (),
            ),
        };
        init
    }
};
unsafe extern "C" fn createReplyObject(mut type_0: libc::c_int) -> *mut redisReply {
    let mut r: *mut redisReply = hi_calloc(
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<redisReply>() as libc::c_ulong,
    ) as *mut redisReply;
    if r.is_null() {
        return 0 as *mut redisReply;
    }
    (*r).type_0 = type_0;
    return r;
}
pub unsafe extern "C" fn freeReplyObject(mut reply: *mut libc::c_void) {
    let mut r: *mut redisReply = reply as *mut redisReply;
    let mut j: size_t = 0;
    if r.is_null() {
        return;
    }
    match (*r).type_0 {
        2 | 9 | 11 | 10 | 12 => {
            if !((*r).element).is_null() {
                j = 0 as libc::c_int as size_t;
                while j < (*r).elements {
                    freeReplyObject(
                        *((*r).element).offset(j as isize) as *mut libc::c_void,
                    );
                    j = j.wrapping_add(1);
                    j;
                }
                hi_free((*r).element as *mut libc::c_void);
            }
        }
        6 | 5 | 1 | 7 | 14 | 13 => {
            hi_free((*r).str_0 as *mut libc::c_void);
        }
        3 | 4 | 8 | _ => {}
    }
    hi_free(r as *mut libc::c_void);
}
unsafe extern "C" fn createStringObject(
    mut task: *const redisReadTask,
    mut str: *mut libc::c_char,
    mut len: size_t,
) -> *mut libc::c_void {
    let mut current_block: u64;
    let mut r: *mut redisReply = 0 as *mut redisReply;
    let mut parent: *mut redisReply = 0 as *mut redisReply;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    r = createReplyObject((*task).type_0);
    if r.is_null() {
        return 0 as *mut libc::c_void;
    }
    if (*task).type_0 == 6 as libc::c_int || (*task).type_0 == 5 as libc::c_int
        || (*task).type_0 == 1 as libc::c_int || (*task).type_0 == 14 as libc::c_int
        || (*task).type_0 == 13 as libc::c_int
    {} else {
        __assert_fail(
            b"task->type == REDIS_REPLY_ERROR || task->type == REDIS_REPLY_STATUS || task->type == REDIS_REPLY_STRING || task->type == REDIS_REPLY_VERB || task->type == REDIS_REPLY_BIGNUM\0"
                as *const u8 as *const libc::c_char,
            b"hiredis.c\0" as *const u8 as *const libc::c_char,
            138 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 64],
                &[libc::c_char; 64],
            >(b"void *createStringObject(const redisReadTask *, char *, size_t)\0"))
                .as_ptr(),
        );
    };
    if (*task).type_0 == 14 as libc::c_int {
        buf = hi_malloc(
            len
                .wrapping_sub(4 as libc::c_int as libc::c_ulong)
                .wrapping_add(1 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_char;
        if buf.is_null() {
            current_block = 12954409626474800040;
        } else {
            memcpy(
                ((*r).vtype).as_mut_ptr() as *mut libc::c_void,
                str as *const libc::c_void,
                3 as libc::c_int as libc::c_ulong,
            );
            (*r).vtype[3 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
            memcpy(
                buf as *mut libc::c_void,
                str.offset(4 as libc::c_int as isize) as *const libc::c_void,
                len.wrapping_sub(4 as libc::c_int as libc::c_ulong),
            );
            *buf
                .offset(
                    len.wrapping_sub(4 as libc::c_int as libc::c_ulong) as isize,
                ) = '\0' as i32 as libc::c_char;
            (*r).len = len.wrapping_sub(4 as libc::c_int as libc::c_ulong);
            current_block = 12800627514080957624;
        }
    } else {
        buf = hi_malloc(len.wrapping_add(1 as libc::c_int as libc::c_ulong))
            as *mut libc::c_char;
        if buf.is_null() {
            current_block = 12954409626474800040;
        } else {
            memcpy(buf as *mut libc::c_void, str as *const libc::c_void, len);
            *buf.offset(len as isize) = '\0' as i32 as libc::c_char;
            (*r).len = len;
            current_block = 12800627514080957624;
        }
    }
    match current_block {
        12954409626474800040 => {
            freeReplyObject(r as *mut libc::c_void);
            return 0 as *mut libc::c_void;
        }
        _ => {
            (*r).str_0 = buf;
            if !((*task).parent).is_null() {
                parent = (*(*task).parent).obj as *mut redisReply;
                if (*parent).type_0 == 2 as libc::c_int
                    || (*parent).type_0 == 9 as libc::c_int
                    || (*parent).type_0 == 11 as libc::c_int
                    || (*parent).type_0 == 10 as libc::c_int
                    || (*parent).type_0 == 12 as libc::c_int
                {} else {
                    __assert_fail(
                        b"parent->type == REDIS_REPLY_ARRAY || parent->type == REDIS_REPLY_MAP || parent->type == REDIS_REPLY_ATTR || parent->type == REDIS_REPLY_SET || parent->type == REDIS_REPLY_PUSH\0"
                            as *const u8 as *const libc::c_char,
                        b"hiredis.c\0" as *const u8 as *const libc::c_char,
                        166 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 64],
                            &[libc::c_char; 64],
                        >(
                            b"void *createStringObject(const redisReadTask *, char *, size_t)\0",
                        ))
                            .as_ptr(),
                    );
                };
                let ref mut fresh0 = *((*parent).element).offset((*task).idx as isize);
                *fresh0 = r;
            }
            return r as *mut libc::c_void;
        }
    };
}
unsafe extern "C" fn createArrayObject(
    mut task: *const redisReadTask,
    mut elements: size_t,
) -> *mut libc::c_void {
    let mut r: *mut redisReply = 0 as *mut redisReply;
    let mut parent: *mut redisReply = 0 as *mut redisReply;
    r = createReplyObject((*task).type_0);
    if r.is_null() {
        return 0 as *mut libc::c_void;
    }
    if elements > 0 as libc::c_int as libc::c_ulong {
        (*r)
            .element = hi_calloc(
            elements,
            ::std::mem::size_of::<*mut redisReply>() as libc::c_ulong,
        ) as *mut *mut redisReply;
        if ((*r).element).is_null() {
            freeReplyObject(r as *mut libc::c_void);
            return 0 as *mut libc::c_void;
        }
    }
    (*r).elements = elements;
    if !((*task).parent).is_null() {
        parent = (*(*task).parent).obj as *mut redisReply;
        if (*parent).type_0 == 2 as libc::c_int || (*parent).type_0 == 9 as libc::c_int
            || (*parent).type_0 == 11 as libc::c_int
            || (*parent).type_0 == 10 as libc::c_int
            || (*parent).type_0 == 12 as libc::c_int
        {} else {
            __assert_fail(
                b"parent->type == REDIS_REPLY_ARRAY || parent->type == REDIS_REPLY_MAP || parent->type == REDIS_REPLY_ATTR || parent->type == REDIS_REPLY_SET || parent->type == REDIS_REPLY_PUSH\0"
                    as *const u8 as *const libc::c_char,
                b"hiredis.c\0" as *const u8 as *const libc::c_char,
                199 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 55],
                    &[libc::c_char; 55],
                >(b"void *createArrayObject(const redisReadTask *, size_t)\0"))
                    .as_ptr(),
            );
        };
        let ref mut fresh1 = *((*parent).element).offset((*task).idx as isize);
        *fresh1 = r;
    }
    return r as *mut libc::c_void;
}
unsafe extern "C" fn createIntegerObject(
    mut task: *const redisReadTask,
    mut value: libc::c_longlong,
) -> *mut libc::c_void {
    let mut r: *mut redisReply = 0 as *mut redisReply;
    let mut parent: *mut redisReply = 0 as *mut redisReply;
    r = createReplyObject(3 as libc::c_int);
    if r.is_null() {
        return 0 as *mut libc::c_void;
    }
    (*r).integer = value;
    if !((*task).parent).is_null() {
        parent = (*(*task).parent).obj as *mut redisReply;
        if (*parent).type_0 == 2 as libc::c_int || (*parent).type_0 == 9 as libc::c_int
            || (*parent).type_0 == 11 as libc::c_int
            || (*parent).type_0 == 10 as libc::c_int
            || (*parent).type_0 == 12 as libc::c_int
        {} else {
            __assert_fail(
                b"parent->type == REDIS_REPLY_ARRAY || parent->type == REDIS_REPLY_MAP || parent->type == REDIS_REPLY_ATTR || parent->type == REDIS_REPLY_SET || parent->type == REDIS_REPLY_PUSH\0"
                    as *const u8 as *const libc::c_char,
                b"hiredis.c\0" as *const u8 as *const libc::c_char,
                220 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 60],
                    &[libc::c_char; 60],
                >(b"void *createIntegerObject(const redisReadTask *, long long)\0"))
                    .as_ptr(),
            );
        };
        let ref mut fresh2 = *((*parent).element).offset((*task).idx as isize);
        *fresh2 = r;
    }
    return r as *mut libc::c_void;
}
unsafe extern "C" fn createDoubleObject(
    mut task: *const redisReadTask,
    mut value: libc::c_double,
    mut str: *mut libc::c_char,
    mut len: size_t,
) -> *mut libc::c_void {
    let mut r: *mut redisReply = 0 as *mut redisReply;
    let mut parent: *mut redisReply = 0 as *mut redisReply;
    if len == 18446744073709551615 as libc::c_ulong {
        return 0 as *mut libc::c_void;
    }
    r = createReplyObject(7 as libc::c_int);
    if r.is_null() {
        return 0 as *mut libc::c_void;
    }
    (*r).dval = value;
    (*r)
        .str_0 = hi_malloc(len.wrapping_add(1 as libc::c_int as libc::c_ulong))
        as *mut libc::c_char;
    if ((*r).str_0).is_null() {
        freeReplyObject(r as *mut libc::c_void);
        return 0 as *mut libc::c_void;
    }
    memcpy((*r).str_0 as *mut libc::c_void, str as *const libc::c_void, len);
    *((*r).str_0).offset(len as isize) = '\0' as i32 as libc::c_char;
    (*r).len = len;
    if !((*task).parent).is_null() {
        parent = (*(*task).parent).obj as *mut redisReply;
        if (*parent).type_0 == 2 as libc::c_int || (*parent).type_0 == 9 as libc::c_int
            || (*parent).type_0 == 11 as libc::c_int
            || (*parent).type_0 == 10 as libc::c_int
            || (*parent).type_0 == 12 as libc::c_int
        {} else {
            __assert_fail(
                b"parent->type == REDIS_REPLY_ARRAY || parent->type == REDIS_REPLY_MAP || parent->type == REDIS_REPLY_ATTR || parent->type == REDIS_REPLY_SET || parent->type == REDIS_REPLY_PUSH\0"
                    as *const u8 as *const libc::c_char,
                b"hiredis.c\0" as *const u8 as *const libc::c_char,
                258 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 72],
                    &[libc::c_char; 72],
                >(
                    b"void *createDoubleObject(const redisReadTask *, double, char *, size_t)\0",
                ))
                    .as_ptr(),
            );
        };
        let ref mut fresh3 = *((*parent).element).offset((*task).idx as isize);
        *fresh3 = r;
    }
    return r as *mut libc::c_void;
}
unsafe extern "C" fn createNilObject(
    mut task: *const redisReadTask,
) -> *mut libc::c_void {
    let mut r: *mut redisReply = 0 as *mut redisReply;
    let mut parent: *mut redisReply = 0 as *mut redisReply;
    r = createReplyObject(4 as libc::c_int);
    if r.is_null() {
        return 0 as *mut libc::c_void;
    }
    if !((*task).parent).is_null() {
        parent = (*(*task).parent).obj as *mut redisReply;
        if (*parent).type_0 == 2 as libc::c_int || (*parent).type_0 == 9 as libc::c_int
            || (*parent).type_0 == 11 as libc::c_int
            || (*parent).type_0 == 10 as libc::c_int
            || (*parent).type_0 == 12 as libc::c_int
        {} else {
            __assert_fail(
                b"parent->type == REDIS_REPLY_ARRAY || parent->type == REDIS_REPLY_MAP || parent->type == REDIS_REPLY_ATTR || parent->type == REDIS_REPLY_SET || parent->type == REDIS_REPLY_PUSH\0"
                    as *const u8 as *const libc::c_char,
                b"hiredis.c\0" as *const u8 as *const libc::c_char,
                277 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 45],
                    &[libc::c_char; 45],
                >(b"void *createNilObject(const redisReadTask *)\0"))
                    .as_ptr(),
            );
        };
        let ref mut fresh4 = *((*parent).element).offset((*task).idx as isize);
        *fresh4 = r;
    }
    return r as *mut libc::c_void;
}
unsafe extern "C" fn createBoolObject(
    mut task: *const redisReadTask,
    mut bval: libc::c_int,
) -> *mut libc::c_void {
    let mut r: *mut redisReply = 0 as *mut redisReply;
    let mut parent: *mut redisReply = 0 as *mut redisReply;
    r = createReplyObject(8 as libc::c_int);
    if r.is_null() {
        return 0 as *mut libc::c_void;
    }
    (*r).integer = (bval != 0 as libc::c_int) as libc::c_int as libc::c_longlong;
    if !((*task).parent).is_null() {
        parent = (*(*task).parent).obj as *mut redisReply;
        if (*parent).type_0 == 2 as libc::c_int || (*parent).type_0 == 9 as libc::c_int
            || (*parent).type_0 == 11 as libc::c_int
            || (*parent).type_0 == 10 as libc::c_int
            || (*parent).type_0 == 12 as libc::c_int
        {} else {
            __assert_fail(
                b"parent->type == REDIS_REPLY_ARRAY || parent->type == REDIS_REPLY_MAP || parent->type == REDIS_REPLY_ATTR || parent->type == REDIS_REPLY_SET || parent->type == REDIS_REPLY_PUSH\0"
                    as *const u8 as *const libc::c_char,
                b"hiredis.c\0" as *const u8 as *const libc::c_char,
                298 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 51],
                    &[libc::c_char; 51],
                >(b"void *createBoolObject(const redisReadTask *, int)\0"))
                    .as_ptr(),
            );
        };
        let ref mut fresh5 = *((*parent).element).offset((*task).idx as isize);
        *fresh5 = r;
    }
    return r as *mut libc::c_void;
}
unsafe extern "C" fn countDigits(mut v: uint64_t) -> uint32_t {
    let mut result: uint32_t = 1 as libc::c_int as uint32_t;
    loop {
        if v < 10 as libc::c_int as libc::c_ulong {
            return result;
        }
        if v < 100 as libc::c_int as libc::c_ulong {
            return result.wrapping_add(1 as libc::c_int as libc::c_uint);
        }
        if v < 1000 as libc::c_int as libc::c_ulong {
            return result.wrapping_add(2 as libc::c_int as libc::c_uint);
        }
        if v < 10000 as libc::c_int as libc::c_ulong {
            return result.wrapping_add(3 as libc::c_int as libc::c_uint);
        }
        v = (v as libc::c_ulong).wrapping_div(10000 as libc::c_uint as libc::c_ulong)
            as uint64_t as uint64_t;
        result = (result as libc::c_uint).wrapping_add(4 as libc::c_int as libc::c_uint)
            as uint32_t as uint32_t;
    };
}
unsafe extern "C" fn bulklen(mut len: size_t) -> size_t {
    return ((1 as libc::c_int as libc::c_uint)
        .wrapping_add(countDigits(len))
        .wrapping_add(2 as libc::c_int as libc::c_uint) as libc::c_ulong)
        .wrapping_add(len)
        .wrapping_add(2 as libc::c_int as libc::c_ulong);
}
pub unsafe extern "C" fn redisvFormatCommand(
    mut target: *mut *mut libc::c_char,
    mut format: *const libc::c_char,
    mut ap: ::std::ffi::VaList,
) -> libc::c_int {
    let mut current_block: u64;
    let mut c: *const libc::c_char = format;
    let mut cmd: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pos: libc::c_int = 0;
    let mut curarg: sds = 0 as *mut libc::c_char;
    let mut newarg: sds = 0 as *mut libc::c_char;
    let mut touched: libc::c_int = 0 as libc::c_int;
    let mut curargv: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut newargv: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut argc: libc::c_int = 0 as libc::c_int;
    let mut totlen: libc::c_int = 0 as libc::c_int;
    let mut error_type: libc::c_int = 0 as libc::c_int;
    let mut j: libc::c_int = 0;
    if target.is_null() {
        return -(1 as libc::c_int);
    }
    curarg = sdsempty();
    if curarg.is_null() {
        return -(1 as libc::c_int);
    }
    loop {
        if !(*c as libc::c_int != '\0' as i32) {
            current_block = 1352918242886884122;
            break;
        }
        if *c as libc::c_int != '%' as i32
            || *c.offset(1 as libc::c_int as isize) as libc::c_int == '\0' as i32
        {
            if *c as libc::c_int == ' ' as i32 {
                if touched != 0 {
                    newargv = hi_realloc(
                        curargv as *mut libc::c_void,
                        (::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
                            .wrapping_mul((argc + 1 as libc::c_int) as libc::c_ulong),
                    ) as *mut *mut libc::c_char;
                    if newargv.is_null() {
                        current_block = 11001950527543677274;
                        break;
                    }
                    curargv = newargv;
                    let fresh6 = argc;
                    argc = argc + 1;
                    let ref mut fresh7 = *curargv.offset(fresh6 as isize);
                    *fresh7 = curarg;
                    totlen = (totlen as libc::c_ulong)
                        .wrapping_add(bulklen(sdslen(curarg))) as libc::c_int
                        as libc::c_int;
                    curarg = sdsempty();
                    if curarg.is_null() {
                        current_block = 11001950527543677274;
                        break;
                    }
                    touched = 0 as libc::c_int;
                }
            } else {
                newarg = sdscatlen(
                    curarg,
                    c as *const libc::c_void,
                    1 as libc::c_int as size_t,
                );
                if newarg.is_null() {
                    current_block = 11001950527543677274;
                    break;
                }
                curarg = newarg;
                touched = 1 as libc::c_int;
            }
        } else {
            let mut arg: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut size: size_t = 0;
            newarg = curarg;
            match *c.offset(1 as libc::c_int as isize) as libc::c_int {
                115 => {
                    arg = ap.arg::<*mut libc::c_char>();
                    size = strlen(arg);
                    if size > 0 as libc::c_int as libc::c_ulong {
                        newarg = sdscatlen(curarg, arg as *const libc::c_void, size);
                    }
                }
                98 => {
                    arg = ap.arg::<*mut libc::c_char>();
                    size = ap.arg::<size_t>();
                    if size > 0 as libc::c_int as libc::c_ulong {
                        newarg = sdscatlen(curarg, arg as *const libc::c_void, size);
                    }
                }
                37 => {
                    newarg = sdscat(curarg, b"%\0" as *const u8 as *const libc::c_char);
                }
                _ => {
                    static mut intfmts: [libc::c_char; 7] = unsafe {
                        *::std::mem::transmute::<
                            &[u8; 7],
                            &[libc::c_char; 7],
                        >(b"diouxX\0")
                    };
                    static mut flags: [libc::c_char; 6] = unsafe {
                        *::std::mem::transmute::<
                            &[u8; 6],
                            &[libc::c_char; 6],
                        >(b"#0-+ \0")
                    };
                    let mut _format: [libc::c_char; 16] = [0; 16];
                    let mut _p: *const libc::c_char = c
                        .offset(1 as libc::c_int as isize);
                    let mut _l: size_t = 0 as libc::c_int as size_t;
                    let mut _cpy: ::std::ffi::VaListImpl;
                    while *_p as libc::c_int != '\0' as i32
                        && !(strchr(flags.as_ptr(), *_p as libc::c_int)).is_null()
                    {
                        _p = _p.offset(1);
                        _p;
                    }
                    while *_p as libc::c_int != '\0' as i32
                        && *(*__ctype_b_loc()).offset(*_p as libc::c_int as isize)
                            as libc::c_int
                            & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
                            != 0
                    {
                        _p = _p.offset(1);
                        _p;
                    }
                    if *_p as libc::c_int == '.' as i32 {
                        _p = _p.offset(1);
                        _p;
                        while *_p as libc::c_int != '\0' as i32
                            && *(*__ctype_b_loc()).offset(*_p as libc::c_int as isize)
                                as libc::c_int
                                & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
                                != 0
                        {
                            _p = _p.offset(1);
                            _p;
                        }
                    }
                    _cpy = ap.clone();
                    if *_p as libc::c_int == '\0' as i32 {
                        current_block = 9216762913005083347;
                        break;
                    }
                    if !(strchr(intfmts.as_ptr(), *_p as libc::c_int)).is_null() {
                        ap.arg::<libc::c_int>();
                    } else if !(strchr(
                        b"eEfFgGaA\0" as *const u8 as *const libc::c_char,
                        *_p as libc::c_int,
                    ))
                        .is_null()
                    {
                        ap.arg::<libc::c_double>();
                    } else if *_p.offset(0 as libc::c_int as isize) as libc::c_int
                        == 'h' as i32
                        && *_p.offset(1 as libc::c_int as isize) as libc::c_int
                            == 'h' as i32
                    {
                        _p = _p.offset(2 as libc::c_int as isize);
                        if !(*_p as libc::c_int != '\0' as i32
                            && !(strchr(intfmts.as_ptr(), *_p as libc::c_int)).is_null())
                        {
                            current_block = 9216762913005083347;
                            break;
                        }
                        ap.arg::<libc::c_int>();
                    } else if *_p.offset(0 as libc::c_int as isize) as libc::c_int
                        == 'h' as i32
                    {
                        _p = _p.offset(1 as libc::c_int as isize);
                        if !(*_p as libc::c_int != '\0' as i32
                            && !(strchr(intfmts.as_ptr(), *_p as libc::c_int)).is_null())
                        {
                            current_block = 9216762913005083347;
                            break;
                        }
                        ap.arg::<libc::c_int>();
                    } else if *_p.offset(0 as libc::c_int as isize) as libc::c_int
                        == 'l' as i32
                        && *_p.offset(1 as libc::c_int as isize) as libc::c_int
                            == 'l' as i32
                    {
                        _p = _p.offset(2 as libc::c_int as isize);
                        if !(*_p as libc::c_int != '\0' as i32
                            && !(strchr(intfmts.as_ptr(), *_p as libc::c_int)).is_null())
                        {
                            current_block = 9216762913005083347;
                            break;
                        }
                        ap.arg::<libc::c_longlong>();
                    } else {
                        if !(*_p.offset(0 as libc::c_int as isize) as libc::c_int
                            == 'l' as i32)
                        {
                            current_block = 9216762913005083347;
                            break;
                        }
                        _p = _p.offset(1 as libc::c_int as isize);
                        if !(*_p as libc::c_int != '\0' as i32
                            && !(strchr(intfmts.as_ptr(), *_p as libc::c_int)).is_null())
                        {
                            current_block = 9216762913005083347;
                            break;
                        }
                        ap.arg::<libc::c_long>();
                    }
                    _l = _p.offset(1 as libc::c_int as isize).offset_from(c)
                        as libc::c_long as size_t;
                    if _l
                        < (::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong)
                            .wrapping_sub(2 as libc::c_int as libc::c_ulong)
                    {
                        memcpy(
                            _format.as_mut_ptr() as *mut libc::c_void,
                            c as *const libc::c_void,
                            _l,
                        );
                        _format[_l as usize] = '\0' as i32 as libc::c_char;
                        newarg = sdscatvprintf(
                            curarg,
                            _format.as_mut_ptr(),
                            _cpy.as_va_list(),
                        );
                        c = _p.offset(-(1 as libc::c_int as isize));
                    }
                }
            }
            if newarg.is_null() {
                current_block = 11001950527543677274;
                break;
            }
            curarg = newarg;
            touched = 1 as libc::c_int;
            c = c.offset(1);
            c;
            if *c as libc::c_int == '\0' as i32 {
                current_block = 1352918242886884122;
                break;
            }
        }
        c = c.offset(1);
        c;
    }
    match current_block {
        1352918242886884122 => {
            if touched != 0 {
                newargv = hi_realloc(
                    curargv as *mut libc::c_void,
                    (::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
                        .wrapping_mul((argc + 1 as libc::c_int) as libc::c_ulong),
                ) as *mut *mut libc::c_char;
                if newargv.is_null() {
                    current_block = 11001950527543677274;
                } else {
                    curargv = newargv;
                    let fresh8 = argc;
                    argc = argc + 1;
                    let ref mut fresh9 = *curargv.offset(fresh8 as isize);
                    *fresh9 = curarg;
                    totlen = (totlen as libc::c_ulong)
                        .wrapping_add(bulklen(sdslen(curarg))) as libc::c_int
                        as libc::c_int;
                    current_block = 15855550149339537395;
                }
            } else {
                sdsfree(curarg);
                current_block = 15855550149339537395;
            }
            match current_block {
                11001950527543677274 => {}
                _ => {
                    curarg = 0 as sds;
                    totlen = (totlen as libc::c_uint)
                        .wrapping_add(
                            (1 as libc::c_int as libc::c_uint)
                                .wrapping_add(countDigits(argc as uint64_t))
                                .wrapping_add(2 as libc::c_int as libc::c_uint),
                        ) as libc::c_int as libc::c_int;
                    cmd = hi_malloc((totlen + 1 as libc::c_int) as size_t)
                        as *mut libc::c_char;
                    if cmd.is_null() {
                        current_block = 11001950527543677274;
                    } else {
                        pos = sprintf(
                            cmd,
                            b"*%d\r\n\0" as *const u8 as *const libc::c_char,
                            argc,
                        );
                        j = 0 as libc::c_int;
                        while j < argc {
                            pos
                                += sprintf(
                                    cmd.offset(pos as isize),
                                    b"$%zu\r\n\0" as *const u8 as *const libc::c_char,
                                    sdslen(*curargv.offset(j as isize)),
                                );
                            memcpy(
                                cmd.offset(pos as isize) as *mut libc::c_void,
                                *curargv.offset(j as isize) as *const libc::c_void,
                                sdslen(*curargv.offset(j as isize)),
                            );
                            pos = (pos as libc::c_ulong)
                                .wrapping_add(sdslen(*curargv.offset(j as isize)))
                                as libc::c_int as libc::c_int;
                            sdsfree(*curargv.offset(j as isize));
                            let fresh10 = pos;
                            pos = pos + 1;
                            *cmd.offset(fresh10 as isize) = '\r' as i32 as libc::c_char;
                            let fresh11 = pos;
                            pos = pos + 1;
                            *cmd.offset(fresh11 as isize) = '\n' as i32 as libc::c_char;
                            j += 1;
                            j;
                        }
                        if pos == totlen {} else {
                            __assert_fail(
                                b"pos == totlen\0" as *const u8 as *const libc::c_char,
                                b"hiredis.c\0" as *const u8 as *const libc::c_char,
                                532 as libc::c_int as libc::c_uint,
                                (*::std::mem::transmute::<
                                    &[u8; 71],
                                    &[libc::c_char; 71],
                                >(
                                    b"int redisvFormatCommand(char **, const char *, struct __va_list_tag *)\0",
                                ))
                                    .as_ptr(),
                            );
                        };
                        *cmd.offset(pos as isize) = '\0' as i32 as libc::c_char;
                        hi_free(curargv as *mut libc::c_void);
                        *target = cmd;
                        return totlen;
                    }
                }
            }
        }
        9216762913005083347 => {
            error_type = -(2 as libc::c_int);
            current_block = 4055649729159803826;
        }
        _ => {}
    }
    match current_block {
        11001950527543677274 => {
            error_type = -(1 as libc::c_int);
        }
        _ => {}
    }
    if !curargv.is_null() {
        loop {
            let fresh12 = argc;
            argc = argc - 1;
            if !(fresh12 != 0) {
                break;
            }
            sdsfree(*curargv.offset(argc as isize));
        }
        hi_free(curargv as *mut libc::c_void);
    }
    sdsfree(curarg);
    hi_free(cmd as *mut libc::c_void);
    return error_type;
}
pub unsafe extern "C" fn redisFormatCommand(
    mut target: *mut *mut libc::c_char,
    mut format: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut ap: ::std::ffi::VaListImpl;
    let mut len: libc::c_int = 0;
    ap = args.clone();
    len = redisvFormatCommand(target, format, ap.as_va_list());
    if len < 0 as libc::c_int {
        len = -(1 as libc::c_int);
    }
    return len;
}
pub unsafe extern "C" fn redisFormatSdsCommandArgv(
    mut target: *mut sds,
    mut argc: libc::c_int,
    mut argv: *mut *const libc::c_char,
    mut argvlen: *const size_t,
) -> libc::c_longlong {
    let mut cmd: sds = 0 as *mut libc::c_char;
    let mut aux: sds = 0 as *mut libc::c_char;
    let mut totlen: libc::c_ulonglong = 0;
    let mut len: libc::c_ulonglong = 0;
    let mut j: libc::c_int = 0;
    if target.is_null() {
        return -(1 as libc::c_int) as libc::c_longlong;
    }
    totlen = (1 as libc::c_int as libc::c_uint)
        .wrapping_add(countDigits(argc as uint64_t))
        .wrapping_add(2 as libc::c_int as libc::c_uint) as libc::c_ulonglong;
    j = 0 as libc::c_int;
    while j < argc {
        len = (if !argvlen.is_null() {
            *argvlen.offset(j as isize)
        } else {
            strlen(*argv.offset(j as isize))
        }) as libc::c_ulonglong;
        totlen = totlen.wrapping_add(bulklen(len as size_t) as libc::c_ulonglong);
        j += 1;
        j;
    }
    cmd = sdsempty();
    if cmd.is_null() {
        return -(1 as libc::c_int) as libc::c_longlong;
    }
    aux = sdsMakeRoomFor(cmd, totlen as size_t);
    if aux.is_null() {
        sdsfree(cmd);
        return -(1 as libc::c_int) as libc::c_longlong;
    }
    cmd = aux;
    cmd = sdscatfmt(cmd, b"*%i\r\n\0" as *const u8 as *const libc::c_char, argc);
    j = 0 as libc::c_int;
    while j < argc {
        len = (if !argvlen.is_null() {
            *argvlen.offset(j as isize)
        } else {
            strlen(*argv.offset(j as isize))
        }) as libc::c_ulonglong;
        cmd = sdscatfmt(cmd, b"$%U\r\n\0" as *const u8 as *const libc::c_char, len);
        cmd = sdscatlen(
            cmd,
            *argv.offset(j as isize) as *const libc::c_void,
            len as size_t,
        );
        cmd = sdscatlen(
            cmd,
            b"\r\n\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            (::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
        j += 1;
        j;
    }
    if sdslen(cmd) as libc::c_ulonglong == totlen {} else {
        __assert_fail(
            b"sdslen(cmd)==totlen\0" as *const u8 as *const libc::c_char,
            b"hiredis.c\0" as *const u8 as *const libc::c_char,
            634 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 79],
                &[libc::c_char; 79],
            >(
                b"long long redisFormatSdsCommandArgv(sds *, int, const char **, const size_t *)\0",
            ))
                .as_ptr(),
        );
    };
    *target = cmd;
    return totlen as libc::c_longlong;
}
pub unsafe extern "C" fn redisFreeSdsCommand(mut cmd: sds) {
    sdsfree(cmd);
}
pub unsafe extern "C" fn redisFormatCommandArgv(
    mut target: *mut *mut libc::c_char,
    mut argc: libc::c_int,
    mut argv: *mut *const libc::c_char,
    mut argvlen: *const size_t,
) -> libc::c_longlong {
    let mut cmd: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pos: size_t = 0;
    let mut len: size_t = 0;
    let mut totlen: size_t = 0;
    let mut j: libc::c_int = 0;
    if target.is_null() {
        return -(1 as libc::c_int) as libc::c_longlong;
    }
    totlen = (1 as libc::c_int as libc::c_uint)
        .wrapping_add(countDigits(argc as uint64_t))
        .wrapping_add(2 as libc::c_int as libc::c_uint) as size_t;
    j = 0 as libc::c_int;
    while j < argc {
        len = if !argvlen.is_null() {
            *argvlen.offset(j as isize)
        } else {
            strlen(*argv.offset(j as isize))
        };
        totlen = (totlen as libc::c_ulong).wrapping_add(bulklen(len)) as size_t
            as size_t;
        j += 1;
        j;
    }
    cmd = hi_malloc(totlen.wrapping_add(1 as libc::c_int as libc::c_ulong))
        as *mut libc::c_char;
    if cmd.is_null() {
        return -(1 as libc::c_int) as libc::c_longlong;
    }
    pos = sprintf(cmd, b"*%d\r\n\0" as *const u8 as *const libc::c_char, argc) as size_t;
    j = 0 as libc::c_int;
    while j < argc {
        len = if !argvlen.is_null() {
            *argvlen.offset(j as isize)
        } else {
            strlen(*argv.offset(j as isize))
        };
        pos = (pos as libc::c_ulong)
            .wrapping_add(
                sprintf(
                    cmd.offset(pos as isize),
                    b"$%zu\r\n\0" as *const u8 as *const libc::c_char,
                    len,
                ) as libc::c_ulong,
            ) as size_t as size_t;
        memcpy(
            cmd.offset(pos as isize) as *mut libc::c_void,
            *argv.offset(j as isize) as *const libc::c_void,
            len,
        );
        pos = (pos as libc::c_ulong).wrapping_add(len) as size_t as size_t;
        let fresh13 = pos;
        pos = pos.wrapping_add(1);
        *cmd.offset(fresh13 as isize) = '\r' as i32 as libc::c_char;
        let fresh14 = pos;
        pos = pos.wrapping_add(1);
        *cmd.offset(fresh14 as isize) = '\n' as i32 as libc::c_char;
        j += 1;
        j;
    }
    if pos == totlen {} else {
        __assert_fail(
            b"pos == totlen\0" as *const u8 as *const libc::c_char,
            b"hiredis.c\0" as *const u8 as *const libc::c_char,
            680 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 78],
                &[libc::c_char; 78],
            >(
                b"long long redisFormatCommandArgv(char **, int, const char **, const size_t *)\0",
            ))
                .as_ptr(),
        );
    };
    *cmd.offset(pos as isize) = '\0' as i32 as libc::c_char;
    *target = cmd;
    return totlen as libc::c_longlong;
}
pub unsafe extern "C" fn redisFreeCommand(mut cmd: *mut libc::c_char) {
    hi_free(cmd as *mut libc::c_void);
}
pub unsafe extern "C" fn __redisSetError(
    mut c: *mut redisContext,
    mut type_0: libc::c_int,
    mut str: *const libc::c_char,
) {
    let mut len: size_t = 0;
    (*c).err = type_0;
    if !str.is_null() {
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
            ((*c).errstr).as_mut_ptr() as *mut libc::c_void,
            str as *const libc::c_void,
            len,
        );
        (*c).errstr[len as usize] = '\0' as i32 as libc::c_char;
    } else {
        if type_0 == 1 as libc::c_int {} else {
            __assert_fail(
                b"type == REDIS_ERR_IO\0" as *const u8 as *const libc::c_char,
                b"hiredis.c\0" as *const u8 as *const libc::c_char,
                702 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 56],
                    &[libc::c_char; 56],
                >(b"void __redisSetError(redisContext *, int, const char *)\0"))
                    .as_ptr(),
            );
        };
        __xpg_strerror_r(
            *__errno_location(),
            ((*c).errstr).as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
        );
    };
}
pub unsafe extern "C" fn redisReaderCreate() -> *mut redisReader {
    return redisReaderCreateWithFunctions(&mut defaultFunctions);
}
unsafe extern "C" fn redisPushAutoFree(
    mut privdata: *mut libc::c_void,
    mut reply: *mut libc::c_void,
) {
    freeReplyObject(reply);
}
unsafe extern "C" fn redisContextInit() -> *mut redisContext {
    let mut c: *mut redisContext = 0 as *mut redisContext;
    c = hi_calloc(
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<redisContext>() as libc::c_ulong,
    ) as *mut redisContext;
    if c.is_null() {
        return 0 as *mut redisContext;
    }
    (*c).funcs = &mut redisContextDefaultFuncs;
    (*c).obuf = sdsempty();
    (*c).reader = redisReaderCreate();
    (*c).fd = -(1 as libc::c_int);
    if ((*c).obuf).is_null() || ((*c).reader).is_null() {
        redisFree(c);
        return 0 as *mut redisContext;
    }
    return c;
}
pub unsafe extern "C" fn redisFree(mut c: *mut redisContext) {
    if c.is_null() {
        return;
    }
    if !((*c).funcs).is_null() && ((*(*c).funcs).close).is_some() {
        ((*(*c).funcs).close).unwrap()(c);
    }
    sdsfree((*c).obuf);
    redisReaderFree((*c).reader);
    hi_free((*c).tcp.host as *mut libc::c_void);
    hi_free((*c).tcp.source_addr as *mut libc::c_void);
    hi_free((*c).unix_sock.path as *mut libc::c_void);
    hi_free((*c).connect_timeout as *mut libc::c_void);
    hi_free((*c).command_timeout as *mut libc::c_void);
    hi_free((*c).saddr as *mut libc::c_void);
    if !((*c).privdata).is_null() && ((*c).free_privdata).is_some() {
        ((*c).free_privdata).unwrap()((*c).privdata);
    }
    if !((*c).funcs).is_null() && ((*(*c).funcs).free_privctx).is_some() {
        ((*(*c).funcs).free_privctx).unwrap()((*c).privctx);
    }
    memset(
        c as *mut libc::c_void,
        0xff as libc::c_int,
        ::std::mem::size_of::<redisContext>() as libc::c_ulong,
    );
    hi_free(c as *mut libc::c_void);
}
pub unsafe extern "C" fn redisFreeKeepFd(mut c: *mut redisContext) -> redisFD {
    let mut fd: redisFD = (*c).fd;
    (*c).fd = -(1 as libc::c_int);
    redisFree(c);
    return fd;
}
pub unsafe extern "C" fn redisReconnect(mut c: *mut redisContext) -> libc::c_int {
    (*c).err = 0 as libc::c_int;
    memset(
        ((*c).errstr).as_mut_ptr() as *mut libc::c_void,
        '\0' as i32,
        strlen(((*c).errstr).as_mut_ptr()),
    );
    if !((*c).privctx).is_null() && ((*(*c).funcs).free_privctx).is_some() {
        ((*(*c).funcs).free_privctx).unwrap()((*c).privctx);
        (*c).privctx = 0 as *mut libc::c_void;
    }
    if !((*c).funcs).is_null() && ((*(*c).funcs).close).is_some() {
        ((*(*c).funcs).close).unwrap()(c);
    }
    sdsfree((*c).obuf);
    redisReaderFree((*c).reader);
    (*c).obuf = sdsempty();
    (*c).reader = redisReaderCreate();
    if ((*c).obuf).is_null() || ((*c).reader).is_null() {
        __redisSetError(
            c,
            5 as libc::c_int,
            b"Out of memory\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    let mut ret: libc::c_int = -(1 as libc::c_int);
    if (*c).connection_type as libc::c_uint
        == REDIS_CONN_TCP as libc::c_int as libc::c_uint
    {
        ret = redisContextConnectBindTcp(
            c,
            (*c).tcp.host,
            (*c).tcp.port,
            (*c).connect_timeout,
            (*c).tcp.source_addr,
        );
    } else if (*c).connection_type as libc::c_uint
        == REDIS_CONN_UNIX as libc::c_int as libc::c_uint
    {
        ret = redisContextConnectUnix(c, (*c).unix_sock.path, (*c).connect_timeout);
    } else {
        __redisSetError(
            c,
            2 as libc::c_int,
            b"Not enough information to reconnect\0" as *const u8 as *const libc::c_char,
        );
        ret = -(1 as libc::c_int);
    }
    if !((*c).command_timeout).is_null() && (*c).flags & 0x1 as libc::c_int != 0
        && (*c).fd != -(1 as libc::c_int)
    {
        redisContextSetTimeout(c, *(*c).command_timeout);
    }
    return ret;
}
pub unsafe extern "C" fn redisConnectWithOptions(
    mut options: *const redisOptions,
) -> *mut redisContext {
    let mut c: *mut redisContext = redisContextInit();
    if c.is_null() {
        return 0 as *mut redisContext;
    }
    if (*options).options & 0x1 as libc::c_int == 0 {
        (*c).flags |= 0x1 as libc::c_int;
    }
    if (*options).options & 0x2 as libc::c_int != 0 {
        (*c).flags |= 0x80 as libc::c_int;
    }
    if (*options).options & 0x4 as libc::c_int != 0 {
        (*c).flags |= 0x200 as libc::c_int;
    }
    if (*options).options & 0x10 as libc::c_int != 0 {
        (*c).flags |= 0x400 as libc::c_int;
    }
    if (*options).options & 0x20 as libc::c_int != 0 {
        (*c).flags |= 0x800 as libc::c_int;
    }
    if (*options).options & 0x40 as libc::c_int != 0 {
        (*c).flags |= 0x1000 as libc::c_int;
    }
    if ((*options).push_cb).is_some() {
        redisSetPushCallback(c, (*options).push_cb);
    } else if (*options).options & 0x8 as libc::c_int == 0 {
        redisSetPushCallback(
            c,
            Some(
                redisPushAutoFree
                    as unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
            ),
        );
    }
    (*c).privdata = (*options).privdata;
    (*c).free_privdata = (*options).free_privdata;
    if redisContextUpdateConnectTimeout(c, (*options).connect_timeout)
        != 0 as libc::c_int
        || redisContextUpdateCommandTimeout(c, (*options).command_timeout)
            != 0 as libc::c_int
    {
        __redisSetError(
            c,
            5 as libc::c_int,
            b"Out of memory\0" as *const u8 as *const libc::c_char,
        );
        return c;
    }
    if (*options).type_0 == REDIS_CONN_TCP as libc::c_int {
        redisContextConnectBindTcp(
            c,
            (*options).endpoint.tcp.ip,
            (*options).endpoint.tcp.port,
            (*options).connect_timeout,
            (*options).endpoint.tcp.source_addr,
        );
    } else if (*options).type_0 == REDIS_CONN_UNIX as libc::c_int {
        redisContextConnectUnix(
            c,
            (*options).endpoint.unix_socket,
            (*options).connect_timeout,
        );
    } else if (*options).type_0 == REDIS_CONN_USERFD as libc::c_int {
        (*c).fd = (*options).endpoint.fd;
        (*c).flags |= 0x2 as libc::c_int;
    } else {
        redisFree(c);
        return 0 as *mut redisContext;
    }
    if (*c).err == 0 as libc::c_int && (*c).fd != -(1 as libc::c_int)
        && !((*options).command_timeout).is_null()
        && (*c).flags & 0x1 as libc::c_int != 0
    {
        redisContextSetTimeout(c, *(*options).command_timeout);
    }
    return c;
}
pub unsafe extern "C" fn redisConnect(
    mut ip: *const libc::c_char,
    mut port: libc::c_int,
) -> *mut redisContext {
    let mut options: redisOptions = {
        let mut init = redisOptions {
            type_0: 0 as libc::c_int,
            options: 0,
            connect_timeout: 0 as *const timeval,
            command_timeout: 0 as *const timeval,
            endpoint: C2RustUnnamed_4 {
                tcp: C2RustUnnamed_5 {
                    source_addr: 0 as *const libc::c_char,
                    ip: 0 as *const libc::c_char,
                    port: 0,
                },
            },
            privdata: 0 as *mut libc::c_void,
            free_privdata: None,
            push_cb: None,
            async_push_cb: None,
        };
        init
    };
    options.type_0 = REDIS_CONN_TCP as libc::c_int;
    options.endpoint.tcp.ip = ip;
    options.endpoint.tcp.port = port;
    return redisConnectWithOptions(&mut options);
}
pub unsafe extern "C" fn redisConnectWithTimeout(
    mut ip: *const libc::c_char,
    mut port: libc::c_int,
    tv: timeval,
) -> *mut redisContext {
    let mut options: redisOptions = {
        let mut init = redisOptions {
            type_0: 0 as libc::c_int,
            options: 0,
            connect_timeout: 0 as *const timeval,
            command_timeout: 0 as *const timeval,
            endpoint: C2RustUnnamed_4 {
                tcp: C2RustUnnamed_5 {
                    source_addr: 0 as *const libc::c_char,
                    ip: 0 as *const libc::c_char,
                    port: 0,
                },
            },
            privdata: 0 as *mut libc::c_void,
            free_privdata: None,
            push_cb: None,
            async_push_cb: None,
        };
        init
    };
    options.type_0 = REDIS_CONN_TCP as libc::c_int;
    options.endpoint.tcp.ip = ip;
    options.endpoint.tcp.port = port;
    options.connect_timeout = &tv;
    return redisConnectWithOptions(&mut options);
}
pub unsafe extern "C" fn redisConnectNonBlock(
    mut ip: *const libc::c_char,
    mut port: libc::c_int,
) -> *mut redisContext {
    let mut options: redisOptions = {
        let mut init = redisOptions {
            type_0: 0 as libc::c_int,
            options: 0,
            connect_timeout: 0 as *const timeval,
            command_timeout: 0 as *const timeval,
            endpoint: C2RustUnnamed_4 {
                tcp: C2RustUnnamed_5 {
                    source_addr: 0 as *const libc::c_char,
                    ip: 0 as *const libc::c_char,
                    port: 0,
                },
            },
            privdata: 0 as *mut libc::c_void,
            free_privdata: None,
            push_cb: None,
            async_push_cb: None,
        };
        init
    };
    options.type_0 = REDIS_CONN_TCP as libc::c_int;
    options.endpoint.tcp.ip = ip;
    options.endpoint.tcp.port = port;
    options.options |= 0x1 as libc::c_int;
    return redisConnectWithOptions(&mut options);
}
pub unsafe extern "C" fn redisConnectBindNonBlock(
    mut ip: *const libc::c_char,
    mut port: libc::c_int,
    mut source_addr: *const libc::c_char,
) -> *mut redisContext {
    let mut options: redisOptions = {
        let mut init = redisOptions {
            type_0: 0 as libc::c_int,
            options: 0,
            connect_timeout: 0 as *const timeval,
            command_timeout: 0 as *const timeval,
            endpoint: C2RustUnnamed_4 {
                tcp: C2RustUnnamed_5 {
                    source_addr: 0 as *const libc::c_char,
                    ip: 0 as *const libc::c_char,
                    port: 0,
                },
            },
            privdata: 0 as *mut libc::c_void,
            free_privdata: None,
            push_cb: None,
            async_push_cb: None,
        };
        init
    };
    options.type_0 = REDIS_CONN_TCP as libc::c_int;
    options.endpoint.tcp.ip = ip;
    options.endpoint.tcp.port = port;
    options.endpoint.tcp.source_addr = source_addr;
    options.options |= 0x1 as libc::c_int;
    return redisConnectWithOptions(&mut options);
}
pub unsafe extern "C" fn redisConnectBindNonBlockWithReuse(
    mut ip: *const libc::c_char,
    mut port: libc::c_int,
    mut source_addr: *const libc::c_char,
) -> *mut redisContext {
    let mut options: redisOptions = {
        let mut init = redisOptions {
            type_0: 0 as libc::c_int,
            options: 0,
            connect_timeout: 0 as *const timeval,
            command_timeout: 0 as *const timeval,
            endpoint: C2RustUnnamed_4 {
                tcp: C2RustUnnamed_5 {
                    source_addr: 0 as *const libc::c_char,
                    ip: 0 as *const libc::c_char,
                    port: 0,
                },
            },
            privdata: 0 as *mut libc::c_void,
            free_privdata: None,
            push_cb: None,
            async_push_cb: None,
        };
        init
    };
    options.type_0 = REDIS_CONN_TCP as libc::c_int;
    options.endpoint.tcp.ip = ip;
    options.endpoint.tcp.port = port;
    options.endpoint.tcp.source_addr = source_addr;
    options.options |= 0x1 as libc::c_int | 0x2 as libc::c_int;
    return redisConnectWithOptions(&mut options);
}
pub unsafe extern "C" fn redisConnectUnix(
    mut path: *const libc::c_char,
) -> *mut redisContext {
    let mut options: redisOptions = {
        let mut init = redisOptions {
            type_0: 0 as libc::c_int,
            options: 0,
            connect_timeout: 0 as *const timeval,
            command_timeout: 0 as *const timeval,
            endpoint: C2RustUnnamed_4 {
                tcp: C2RustUnnamed_5 {
                    source_addr: 0 as *const libc::c_char,
                    ip: 0 as *const libc::c_char,
                    port: 0,
                },
            },
            privdata: 0 as *mut libc::c_void,
            free_privdata: None,
            push_cb: None,
            async_push_cb: None,
        };
        init
    };
    options.type_0 = REDIS_CONN_UNIX as libc::c_int;
    options.endpoint.unix_socket = path;
    return redisConnectWithOptions(&mut options);
}
pub unsafe extern "C" fn redisConnectUnixWithTimeout(
    mut path: *const libc::c_char,
    tv: timeval,
) -> *mut redisContext {
    let mut options: redisOptions = {
        let mut init = redisOptions {
            type_0: 0 as libc::c_int,
            options: 0,
            connect_timeout: 0 as *const timeval,
            command_timeout: 0 as *const timeval,
            endpoint: C2RustUnnamed_4 {
                tcp: C2RustUnnamed_5 {
                    source_addr: 0 as *const libc::c_char,
                    ip: 0 as *const libc::c_char,
                    port: 0,
                },
            },
            privdata: 0 as *mut libc::c_void,
            free_privdata: None,
            push_cb: None,
            async_push_cb: None,
        };
        init
    };
    options.type_0 = REDIS_CONN_UNIX as libc::c_int;
    options.endpoint.unix_socket = path;
    options.connect_timeout = &tv;
    return redisConnectWithOptions(&mut options);
}
pub unsafe extern "C" fn redisConnectUnixNonBlock(
    mut path: *const libc::c_char,
) -> *mut redisContext {
    let mut options: redisOptions = {
        let mut init = redisOptions {
            type_0: 0 as libc::c_int,
            options: 0,
            connect_timeout: 0 as *const timeval,
            command_timeout: 0 as *const timeval,
            endpoint: C2RustUnnamed_4 {
                tcp: C2RustUnnamed_5 {
                    source_addr: 0 as *const libc::c_char,
                    ip: 0 as *const libc::c_char,
                    port: 0,
                },
            },
            privdata: 0 as *mut libc::c_void,
            free_privdata: None,
            push_cb: None,
            async_push_cb: None,
        };
        init
    };
    options.type_0 = REDIS_CONN_UNIX as libc::c_int;
    options.endpoint.unix_socket = path;
    options.options |= 0x1 as libc::c_int;
    return redisConnectWithOptions(&mut options);
}
pub unsafe extern "C" fn redisConnectFd(mut fd: redisFD) -> *mut redisContext {
    let mut options: redisOptions = {
        let mut init = redisOptions {
            type_0: 0 as libc::c_int,
            options: 0,
            connect_timeout: 0 as *const timeval,
            command_timeout: 0 as *const timeval,
            endpoint: C2RustUnnamed_4 {
                tcp: C2RustUnnamed_5 {
                    source_addr: 0 as *const libc::c_char,
                    ip: 0 as *const libc::c_char,
                    port: 0,
                },
            },
            privdata: 0 as *mut libc::c_void,
            free_privdata: None,
            push_cb: None,
            async_push_cb: None,
        };
        init
    };
    options.type_0 = REDIS_CONN_USERFD as libc::c_int;
    options.endpoint.fd = fd;
    return redisConnectWithOptions(&mut options);
}
pub unsafe extern "C" fn redisSetTimeout(
    mut c: *mut redisContext,
    tv: timeval,
) -> libc::c_int {
    if (*c).flags & 0x1 as libc::c_int != 0 {
        return redisContextSetTimeout(c, tv);
    }
    return -(1 as libc::c_int);
}
pub unsafe extern "C" fn redisEnableKeepAliveWithInterval(
    mut c: *mut redisContext,
    mut interval: libc::c_int,
) -> libc::c_int {
    return redisKeepAlive(c, interval);
}
pub unsafe extern "C" fn redisEnableKeepAlive(mut c: *mut redisContext) -> libc::c_int {
    return redisKeepAlive(c, 15 as libc::c_int);
}
pub unsafe extern "C" fn redisSetTcpUserTimeout(
    mut c: *mut redisContext,
    mut timeout: libc::c_uint,
) -> libc::c_int {
    return redisContextSetTcpUserTimeout(c, timeout);
}
pub unsafe extern "C" fn redisSetPushCallback(
    mut c: *mut redisContext,
    mut fn_0: Option::<redisPushFn>,
) -> Option::<redisPushFn> {
    let mut old: Option::<redisPushFn> = (*c).push_cb;
    (*c).push_cb = fn_0;
    return old;
}
pub unsafe extern "C" fn redisBufferRead(mut c: *mut redisContext) -> libc::c_int {
    let mut buf: [libc::c_char; 16384] = [0; 16384];
    let mut nread: libc::c_int = 0;
    if (*c).err != 0 {
        return -(1 as libc::c_int);
    }
    nread = ((*(*c).funcs).read)
        .unwrap()(
        c,
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 16384]>() as libc::c_ulong,
    ) as libc::c_int;
    if nread < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if nread > 0 as libc::c_int
        && redisReaderFeed((*c).reader, buf.as_mut_ptr(), nread as size_t)
            != 0 as libc::c_int
    {
        __redisSetError(c, (*(*c).reader).err, ((*(*c).reader).errstr).as_mut_ptr());
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn redisBufferWrite(
    mut c: *mut redisContext,
    mut done: *mut libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    if (*c).err != 0 {
        return -(1 as libc::c_int);
    }
    if sdslen((*c).obuf) > 0 as libc::c_int as libc::c_ulong {
        let mut nwritten: ssize_t = ((*(*c).funcs).write).unwrap()(c);
        if nwritten < 0 as libc::c_int as libc::c_long {
            return -(1 as libc::c_int)
        } else if nwritten > 0 as libc::c_int as libc::c_long {
            if nwritten == sdslen((*c).obuf) as ssize_t {
                sdsfree((*c).obuf);
                (*c).obuf = sdsempty();
                if ((*c).obuf).is_null() {
                    current_block = 9337634068658818778;
                } else {
                    current_block = 12209867499936983673;
                }
            } else if sdsrange((*c).obuf, nwritten, -(1 as libc::c_int) as ssize_t)
                < 0 as libc::c_int
            {
                current_block = 9337634068658818778;
            } else {
                current_block = 12209867499936983673;
            }
            match current_block {
                12209867499936983673 => {}
                _ => {
                    __redisSetError(
                        c,
                        5 as libc::c_int,
                        b"Out of memory\0" as *const u8 as *const libc::c_char,
                    );
                    return -(1 as libc::c_int);
                }
            }
        }
    }
    if !done.is_null() {
        *done = (sdslen((*c).obuf) == 0 as libc::c_int as libc::c_ulong) as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn redisHandledPushReply(
    mut c: *mut redisContext,
    mut reply: *mut libc::c_void,
) -> libc::c_int {
    if !reply.is_null() && ((*c).push_cb).is_some()
        && (*(reply as *mut redisReply)).type_0 == 12 as libc::c_int
    {
        ((*c).push_cb).unwrap()((*c).privdata, reply);
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn redisGetReplyFromReader(
    mut c: *mut redisContext,
    mut reply: *mut *mut libc::c_void,
) -> libc::c_int {
    if redisReaderGetReply((*c).reader, reply) == -(1 as libc::c_int) {
        __redisSetError(c, (*(*c).reader).err, ((*(*c).reader).errstr).as_mut_ptr());
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn redisNextInBandReplyFromReader(
    mut c: *mut redisContext,
    mut reply: *mut *mut libc::c_void,
) -> libc::c_int {
    loop {
        if redisGetReplyFromReader(c, reply) == -(1 as libc::c_int) {
            return -(1 as libc::c_int);
        }
        if !(redisHandledPushReply(c, *reply) != 0) {
            break;
        }
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn redisGetReply(
    mut c: *mut redisContext,
    mut reply: *mut *mut libc::c_void,
) -> libc::c_int {
    let mut wdone: libc::c_int = 0 as libc::c_int;
    let mut aux: *mut libc::c_void = 0 as *mut libc::c_void;
    if redisNextInBandReplyFromReader(c, &mut aux) == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    if aux.is_null() && (*c).flags & 0x1 as libc::c_int != 0 {
        loop {
            if redisBufferWrite(c, &mut wdone) == -(1 as libc::c_int) {
                return -(1 as libc::c_int);
            }
            if !(wdone == 0) {
                break;
            }
        }
        loop {
            if redisBufferRead(c) == -(1 as libc::c_int) {
                return -(1 as libc::c_int);
            }
            if redisNextInBandReplyFromReader(c, &mut aux) == -(1 as libc::c_int) {
                return -(1 as libc::c_int);
            }
            if !aux.is_null() {
                break;
            }
        }
    }
    if !reply.is_null() {
        *reply = aux;
    } else {
        freeReplyObject(aux);
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn __redisAppendCommand(
    mut c: *mut redisContext,
    mut cmd: *const libc::c_char,
    mut len: size_t,
) -> libc::c_int {
    let mut newbuf: sds = 0 as *mut libc::c_char;
    newbuf = sdscatlen((*c).obuf, cmd as *const libc::c_void, len);
    if newbuf.is_null() {
        __redisSetError(
            c,
            5 as libc::c_int,
            b"Out of memory\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    (*c).obuf = newbuf;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn redisAppendFormattedCommand(
    mut c: *mut redisContext,
    mut cmd: *const libc::c_char,
    mut len: size_t,
) -> libc::c_int {
    if __redisAppendCommand(c, cmd, len) != 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn redisvAppendCommand(
    mut c: *mut redisContext,
    mut format: *const libc::c_char,
    mut ap: ::std::ffi::VaList,
) -> libc::c_int {
    let mut cmd: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: libc::c_int = 0;
    len = redisvFormatCommand(&mut cmd, format, ap.as_va_list());
    if len == -(1 as libc::c_int) {
        __redisSetError(
            c,
            5 as libc::c_int,
            b"Out of memory\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    } else if len == -(2 as libc::c_int) {
        __redisSetError(
            c,
            2 as libc::c_int,
            b"Invalid format string\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if __redisAppendCommand(c, cmd, len as size_t) != 0 as libc::c_int {
        hi_free(cmd as *mut libc::c_void);
        return -(1 as libc::c_int);
    }
    hi_free(cmd as *mut libc::c_void);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn redisAppendCommand(
    mut c: *mut redisContext,
    mut format: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut ap: ::std::ffi::VaListImpl;
    let mut ret: libc::c_int = 0;
    ap = args.clone();
    ret = redisvAppendCommand(c, format, ap.as_va_list());
    return ret;
}
pub unsafe extern "C" fn redisAppendCommandArgv(
    mut c: *mut redisContext,
    mut argc: libc::c_int,
    mut argv: *mut *const libc::c_char,
    mut argvlen: *const size_t,
) -> libc::c_int {
    let mut cmd: sds = 0 as *mut libc::c_char;
    let mut len: libc::c_longlong = 0;
    len = redisFormatSdsCommandArgv(&mut cmd, argc, argv, argvlen);
    if len == -(1 as libc::c_int) as libc::c_longlong {
        __redisSetError(
            c,
            5 as libc::c_int,
            b"Out of memory\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if __redisAppendCommand(c, cmd as *const libc::c_char, len as size_t)
        != 0 as libc::c_int
    {
        sdsfree(cmd);
        return -(1 as libc::c_int);
    }
    sdsfree(cmd);
    return 0 as libc::c_int;
}
unsafe extern "C" fn __redisBlockForReply(
    mut c: *mut redisContext,
) -> *mut libc::c_void {
    let mut reply: *mut libc::c_void = 0 as *mut libc::c_void;
    if (*c).flags & 0x1 as libc::c_int != 0 {
        if redisGetReply(c, &mut reply) != 0 as libc::c_int {
            return 0 as *mut libc::c_void;
        }
        return reply;
    }
    return 0 as *mut libc::c_void;
}
pub unsafe extern "C" fn redisvCommand(
    mut c: *mut redisContext,
    mut format: *const libc::c_char,
    mut ap: ::std::ffi::VaList,
) -> *mut libc::c_void {
    if redisvAppendCommand(c, format, ap.as_va_list()) != 0 as libc::c_int {
        return 0 as *mut libc::c_void;
    }
    return __redisBlockForReply(c);
}
pub unsafe extern "C" fn redisCommand(
    mut c: *mut redisContext,
    mut format: *const libc::c_char,
    mut args: ...
) -> *mut libc::c_void {
    let mut ap: ::std::ffi::VaListImpl;
    ap = args.clone();
    let mut reply: *mut libc::c_void = redisvCommand(c, format, ap.as_va_list());
    return reply;
}
pub unsafe extern "C" fn redisCommandArgv(
    mut c: *mut redisContext,
    mut argc: libc::c_int,
    mut argv: *mut *const libc::c_char,
    mut argvlen: *const size_t,
) -> *mut libc::c_void {
    if redisAppendCommandArgv(c, argc, argv, argvlen) != 0 as libc::c_int {
        return 0 as *mut libc::c_void;
    }
    return __redisBlockForReply(c);
}
