use ::libc;
extern "C" {
    pub type sockaddr;
    static mut hiredisAllocFns: hiredisAllocFuncs;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strcasecmp(__s1: *const libc::c_char, __s2: *const libc::c_char) -> libc::c_int;
    fn strncasecmp(
        __s1: *const libc::c_char,
        __s2: *const libc::c_char,
        __n: size_t,
    ) -> libc::c_int;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn __ctype_tolower_loc() -> *mut *const __int32_t;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn sdsnewlen(init: *const libc::c_void, initlen: size_t) -> sds;
    fn sdsfree(s: sds);
    fn redisvFormatCommand(
        target: *mut *mut libc::c_char,
        format: *const libc::c_char,
        ap: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn redisFormatSdsCommandArgv(
        target: *mut sds,
        argc: libc::c_int,
        argv: *mut *const libc::c_char,
        argvlen: *const size_t,
    ) -> libc::c_longlong;
    fn redisConnectWithOptions(options: *const redisOptions) -> *mut redisContext;
    fn redisGetReply(c: *mut redisContext, reply: *mut *mut libc::c_void) -> libc::c_int;
    fn redisBufferWrite(c: *mut redisContext, done: *mut libc::c_int) -> libc::c_int;
    fn redisBufferRead(c: *mut redisContext) -> libc::c_int;
    fn redisFree(c: *mut redisContext);
    fn redisCheckSocketError(c: *mut redisContext) -> libc::c_int;
    fn redisCheckConnectDone(
        c: *mut redisContext,
        completed: *mut libc::c_int,
    ) -> libc::c_int;
    fn redisSetTcpNoDelay(c: *mut redisContext) -> libc::c_int;
    fn __redisAppendCommand(
        c: *mut redisContext,
        cmd: *const libc::c_char,
        len: size_t,
    ) -> libc::c_int;
    fn __redisSetError(
        c: *mut redisContext,
        type_0: libc::c_int,
        str: *const libc::c_char,
    );
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
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
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
pub type ssize_t = __ssize_t;
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
pub struct redisAsyncContext {
    pub c: redisContext,
    pub err: libc::c_int,
    pub errstr: *mut libc::c_char,
    pub data: *mut libc::c_void,
    pub dataCleanup: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub ev: C2RustUnnamed_0,
    pub onDisconnect: Option::<redisDisconnectCallback>,
    pub onConnect: Option::<redisConnectCallback>,
    pub onConnectNC: Option::<redisConnectCallbackNC>,
    pub replies: redisCallbackList,
    pub saddr: *mut sockaddr,
    pub addrlen: size_t,
    pub sub: C2RustUnnamed,
    pub push_cb: Option::<redisAsyncPushFn>,
}
pub type redisAsyncPushFn = unsafe extern "C" fn(
    *mut redisAsyncContext,
    *mut libc::c_void,
) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub replies: redisCallbackList,
    pub channels: *mut dict,
    pub patterns: *mut dict,
    pub pending_unsubs: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dict {
    pub table: *mut *mut dictEntry,
    pub type_0: *mut dictType,
    pub size: libc::c_ulong,
    pub sizemask: libc::c_ulong,
    pub used: libc::c_ulong,
    pub privdata: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dictType {
    pub hashFunction: Option::<
        unsafe extern "C" fn(*const libc::c_void) -> libc::c_uint,
    >,
    pub keyDup: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *const libc::c_void) -> *mut libc::c_void,
    >,
    pub valDup: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *const libc::c_void) -> *mut libc::c_void,
    >,
    pub keyCompare: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *const libc::c_void,
            *const libc::c_void,
        ) -> libc::c_int,
    >,
    pub keyDestructor: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
    >,
    pub valDestructor: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dictEntry {
    pub key: *mut libc::c_void,
    pub val: *mut libc::c_void,
    pub next: *mut dictEntry,
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
pub struct C2RustUnnamed_0 {
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
    pub tcp: C2RustUnnamed_2,
    pub unix_sock: C2RustUnnamed_1,
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
pub struct C2RustUnnamed_1 {
    pub path: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
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
    pub endpoint: C2RustUnnamed_3,
    pub privdata: *mut libc::c_void,
    pub free_privdata: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub push_cb: Option::<redisPushFn>,
    pub async_push_cb: Option::<redisAsyncPushFn>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub tcp: C2RustUnnamed_4,
    pub unix_socket: *const libc::c_char,
    pub fd: redisFD,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub source_addr: *const libc::c_char,
    pub ip: *const libc::c_char,
    pub port: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dictIterator {
    pub ht: *mut dict,
    pub index: libc::c_int,
    pub entry: *mut dictEntry,
    pub nextEntry: *mut dictEntry,
}
#[inline]
unsafe extern "C" fn hi_free(mut ptr: *mut libc::c_void) {
    (hiredisAllocFns.freeFn).unwrap()(ptr);
}
#[inline]
unsafe extern "C" fn hi_realloc(
    mut ptr: *mut libc::c_void,
    mut size: size_t,
) -> *mut libc::c_void {
    return (hiredisAllocFns.reallocFn).unwrap()(ptr, size);
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
unsafe extern "C" fn tolower(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_tolower_loc()).offset(__c as isize)
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
unsafe extern "C" fn dictGenHashFunction(
    mut buf: *const libc::c_uchar,
    mut len: libc::c_int,
) -> libc::c_uint {
    let mut hash: libc::c_uint = 5381 as libc::c_int as libc::c_uint;
    loop {
        let fresh0 = len;
        len = len - 1;
        if !(fresh0 != 0) {
            break;
        }
        let fresh1 = buf;
        buf = buf.offset(1);
        hash = (hash << 5 as libc::c_int)
            .wrapping_add(hash)
            .wrapping_add(*fresh1 as libc::c_uint);
    }
    return hash;
}
unsafe extern "C" fn _dictReset(mut ht: *mut dict) {
    (*ht).table = 0 as *mut *mut dictEntry;
    (*ht).size = 0 as libc::c_int as libc::c_ulong;
    (*ht).sizemask = 0 as libc::c_int as libc::c_ulong;
    (*ht).used = 0 as libc::c_int as libc::c_ulong;
}
unsafe extern "C" fn dictCreate(
    mut type_0: *mut dictType,
    mut privDataPtr: *mut libc::c_void,
) -> *mut dict {
    let mut ht: *mut dict = hi_malloc(::std::mem::size_of::<dict>() as libc::c_ulong)
        as *mut dict;
    if ht.is_null() {
        return 0 as *mut dict;
    }
    _dictInit(ht, type_0, privDataPtr);
    return ht;
}
unsafe extern "C" fn _dictInit(
    mut ht: *mut dict,
    mut type_0: *mut dictType,
    mut privDataPtr: *mut libc::c_void,
) -> libc::c_int {
    _dictReset(ht);
    (*ht).type_0 = type_0;
    (*ht).privdata = privDataPtr;
    return 0 as libc::c_int;
}
unsafe extern "C" fn dictExpand(
    mut ht: *mut dict,
    mut size: libc::c_ulong,
) -> libc::c_int {
    let mut n: dict = dict {
        table: 0 as *mut *mut dictEntry,
        type_0: 0 as *mut dictType,
        size: 0,
        sizemask: 0,
        used: 0,
        privdata: 0 as *mut libc::c_void,
    };
    let mut realsize: libc::c_ulong = _dictNextPower(size);
    let mut i: libc::c_ulong = 0;
    if (*ht).used > size {
        return 1 as libc::c_int;
    }
    _dictInit(&mut n, (*ht).type_0, (*ht).privdata);
    n.size = realsize;
    n.sizemask = realsize.wrapping_sub(1 as libc::c_int as libc::c_ulong);
    n
        .table = hi_calloc(
        realsize,
        ::std::mem::size_of::<*mut dictEntry>() as libc::c_ulong,
    ) as *mut *mut dictEntry;
    if (n.table).is_null() {
        return 1 as libc::c_int;
    }
    n.used = (*ht).used;
    i = 0 as libc::c_int as libc::c_ulong;
    while i < (*ht).size && (*ht).used > 0 as libc::c_int as libc::c_ulong {
        let mut he: *mut dictEntry = 0 as *mut dictEntry;
        let mut nextHe: *mut dictEntry = 0 as *mut dictEntry;
        if !(*((*ht).table).offset(i as isize)).is_null() {
            he = *((*ht).table).offset(i as isize);
            while !he.is_null() {
                let mut h: libc::c_uint = 0;
                nextHe = (*he).next;
                h = (((*(*ht).type_0).hashFunction).unwrap()((*he).key) as libc::c_ulong
                    & n.sizemask) as libc::c_uint;
                (*he).next = *(n.table).offset(h as isize);
                let ref mut fresh2 = *(n.table).offset(h as isize);
                *fresh2 = he;
                (*ht).used = ((*ht).used).wrapping_sub(1);
                (*ht).used;
                he = nextHe;
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    if (*ht).used == 0 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"ht->used == 0\0" as *const u8 as *const libc::c_char,
            b"./dict.c\0" as *const u8 as *const libc::c_char,
            132 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 38],
                &[libc::c_char; 38],
            >(b"int dictExpand(dict *, unsigned long)\0"))
                .as_ptr(),
        );
    };
    hi_free((*ht).table as *mut libc::c_void);
    *ht = n;
    return 0 as libc::c_int;
}
unsafe extern "C" fn dictAdd(
    mut ht: *mut dict,
    mut key: *mut libc::c_void,
    mut val: *mut libc::c_void,
) -> libc::c_int {
    let mut index: libc::c_int = 0;
    let mut entry: *mut dictEntry = 0 as *mut dictEntry;
    index = _dictKeyIndex(ht, key);
    if index == -(1 as libc::c_int) {
        return 1 as libc::c_int;
    }
    entry = hi_malloc(::std::mem::size_of::<dictEntry>() as libc::c_ulong)
        as *mut dictEntry;
    if entry.is_null() {
        return 1 as libc::c_int;
    }
    (*entry).next = *((*ht).table).offset(index as isize);
    let ref mut fresh3 = *((*ht).table).offset(index as isize);
    *fresh3 = entry;
    if ((*(*ht).type_0).keyDup).is_some() {
        (*entry).key = ((*(*ht).type_0).keyDup).unwrap()((*ht).privdata, key);
    } else {
        (*entry).key = key;
    }
    if ((*(*ht).type_0).valDup).is_some() {
        (*entry).val = ((*(*ht).type_0).valDup).unwrap()((*ht).privdata, val);
    } else {
        (*entry).val = val;
    }
    (*ht).used = ((*ht).used).wrapping_add(1);
    (*ht).used;
    return 0 as libc::c_int;
}
unsafe extern "C" fn dictReplace(
    mut ht: *mut dict,
    mut key: *mut libc::c_void,
    mut val: *mut libc::c_void,
) -> libc::c_int {
    let mut entry: *mut dictEntry = 0 as *mut dictEntry;
    let mut auxentry: dictEntry = dictEntry {
        key: 0 as *mut libc::c_void,
        val: 0 as *mut libc::c_void,
        next: 0 as *mut dictEntry,
    };
    if dictAdd(ht, key, val) == 0 as libc::c_int {
        return 1 as libc::c_int;
    }
    entry = dictFind(ht, key);
    if entry.is_null() {
        return 0 as libc::c_int;
    }
    auxentry = *entry;
    if ((*(*ht).type_0).valDup).is_some() {
        (*entry).val = ((*(*ht).type_0).valDup).unwrap()((*ht).privdata, val);
    } else {
        (*entry).val = val;
    }
    if ((*(*ht).type_0).valDestructor).is_some() {
        ((*(*ht).type_0).valDestructor).unwrap()((*ht).privdata, auxentry.val);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn dictDelete(
    mut ht: *mut dict,
    mut key: *const libc::c_void,
) -> libc::c_int {
    let mut h: libc::c_uint = 0;
    let mut de: *mut dictEntry = 0 as *mut dictEntry;
    let mut prevde: *mut dictEntry = 0 as *mut dictEntry;
    if (*ht).size == 0 as libc::c_int as libc::c_ulong {
        return 1 as libc::c_int;
    }
    h = (((*(*ht).type_0).hashFunction).unwrap()(key) as libc::c_ulong & (*ht).sizemask)
        as libc::c_uint;
    de = *((*ht).table).offset(h as isize);
    prevde = 0 as *mut dictEntry;
    while !de.is_null() {
        if if ((*(*ht).type_0).keyCompare).is_some() {
            ((*(*ht).type_0).keyCompare).unwrap()((*ht).privdata, key, (*de).key)
        } else {
            (key == (*de).key as *const libc::c_void) as libc::c_int
        } != 0
        {
            if !prevde.is_null() {
                (*prevde).next = (*de).next;
            } else {
                let ref mut fresh4 = *((*ht).table).offset(h as isize);
                *fresh4 = (*de).next;
            }
            if ((*(*ht).type_0).keyDestructor).is_some() {
                ((*(*ht).type_0).keyDestructor).unwrap()((*ht).privdata, (*de).key);
            }
            if ((*(*ht).type_0).valDestructor).is_some() {
                ((*(*ht).type_0).valDestructor).unwrap()((*ht).privdata, (*de).val);
            }
            hi_free(de as *mut libc::c_void);
            (*ht).used = ((*ht).used).wrapping_sub(1);
            (*ht).used;
            return 0 as libc::c_int;
        }
        prevde = de;
        de = (*de).next;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn _dictClear(mut ht: *mut dict) -> libc::c_int {
    let mut i: libc::c_ulong = 0;
    i = 0 as libc::c_int as libc::c_ulong;
    while i < (*ht).size && (*ht).used > 0 as libc::c_int as libc::c_ulong {
        let mut he: *mut dictEntry = 0 as *mut dictEntry;
        let mut nextHe: *mut dictEntry = 0 as *mut dictEntry;
        he = *((*ht).table).offset(i as isize);
        if !he.is_null() {
            while !he.is_null() {
                nextHe = (*he).next;
                if ((*(*ht).type_0).keyDestructor).is_some() {
                    ((*(*ht).type_0).keyDestructor).unwrap()((*ht).privdata, (*he).key);
                }
                if ((*(*ht).type_0).valDestructor).is_some() {
                    ((*(*ht).type_0).valDestructor).unwrap()((*ht).privdata, (*he).val);
                }
                hi_free(he as *mut libc::c_void);
                (*ht).used = ((*ht).used).wrapping_sub(1);
                (*ht).used;
                he = nextHe;
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    hi_free((*ht).table as *mut libc::c_void);
    _dictReset(ht);
    return 0 as libc::c_int;
}
unsafe extern "C" fn dictFind(
    mut ht: *mut dict,
    mut key: *const libc::c_void,
) -> *mut dictEntry {
    let mut he: *mut dictEntry = 0 as *mut dictEntry;
    let mut h: libc::c_uint = 0;
    if (*ht).size == 0 as libc::c_int as libc::c_ulong {
        return 0 as *mut dictEntry;
    }
    h = (((*(*ht).type_0).hashFunction).unwrap()(key) as libc::c_ulong & (*ht).sizemask)
        as libc::c_uint;
    he = *((*ht).table).offset(h as isize);
    while !he.is_null() {
        if if ((*(*ht).type_0).keyCompare).is_some() {
            ((*(*ht).type_0).keyCompare).unwrap()((*ht).privdata, key, (*he).key)
        } else {
            (key == (*he).key as *const libc::c_void) as libc::c_int
        } != 0
        {
            return he;
        }
        he = (*he).next;
    }
    return 0 as *mut dictEntry;
}
unsafe extern "C" fn dictRelease(mut ht: *mut dict) {
    _dictClear(ht);
    hi_free(ht as *mut libc::c_void);
}
unsafe extern "C" fn _dictNextPower(mut size: libc::c_ulong) -> libc::c_ulong {
    let mut i: libc::c_ulong = 4 as libc::c_int as libc::c_ulong;
    if size >= 9223372036854775807 as libc::c_long as libc::c_ulong {
        return 9223372036854775807 as libc::c_long as libc::c_ulong;
    }
    loop {
        if i >= size {
            return i;
        }
        i = i.wrapping_mul(2 as libc::c_int as libc::c_ulong);
    };
}
unsafe extern "C" fn dictInitIterator(mut iter: *mut dictIterator, mut ht: *mut dict) {
    (*iter).ht = ht;
    (*iter).index = -(1 as libc::c_int);
    (*iter).entry = 0 as *mut dictEntry;
    (*iter).nextEntry = 0 as *mut dictEntry;
}
unsafe extern "C" fn dictNext(mut iter: *mut dictIterator) -> *mut dictEntry {
    loop {
        if ((*iter).entry).is_null() {
            (*iter).index += 1;
            (*iter).index;
            if (*iter).index >= (*(*iter).ht).size as libc::c_int {
                break;
            }
            (*iter).entry = *((*(*iter).ht).table).offset((*iter).index as isize);
        } else {
            (*iter).entry = (*iter).nextEntry;
        }
        if !((*iter).entry).is_null() {
            (*iter).nextEntry = (*(*iter).entry).next;
            return (*iter).entry;
        }
    }
    return 0 as *mut dictEntry;
}
unsafe extern "C" fn _dictKeyIndex(
    mut ht: *mut dict,
    mut key: *const libc::c_void,
) -> libc::c_int {
    let mut h: libc::c_uint = 0;
    let mut he: *mut dictEntry = 0 as *mut dictEntry;
    if _dictExpandIfNeeded(ht) == 1 as libc::c_int {
        return -(1 as libc::c_int);
    }
    h = (((*(*ht).type_0).hashFunction).unwrap()(key) as libc::c_ulong & (*ht).sizemask)
        as libc::c_uint;
    he = *((*ht).table).offset(h as isize);
    while !he.is_null() {
        if if ((*(*ht).type_0).keyCompare).is_some() {
            ((*(*ht).type_0).keyCompare).unwrap()((*ht).privdata, key, (*he).key)
        } else {
            (key == (*he).key as *const libc::c_void) as libc::c_int
        } != 0
        {
            return -(1 as libc::c_int);
        }
        he = (*he).next;
    }
    return h as libc::c_int;
}
unsafe extern "C" fn _dictExpandIfNeeded(mut ht: *mut dict) -> libc::c_int {
    if (*ht).size == 0 as libc::c_int as libc::c_ulong {
        return dictExpand(ht, 4 as libc::c_int as libc::c_ulong);
    }
    if (*ht).used == (*ht).size {
        return dictExpand(
            ht,
            ((*ht).size).wrapping_mul(2 as libc::c_int as libc::c_ulong),
        );
    }
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn refreshTimeout(mut ctx: *mut redisAsyncContext) {
    if (*ctx).c.flags & 0x2 as libc::c_int != 0 {
        if ((*ctx).ev.scheduleTimer).is_some()
            && (!((*ctx).c.command_timeout).is_null()
                && ((*(*ctx).c.command_timeout).tv_sec != 0
                    || (*(*ctx).c.command_timeout).tv_usec != 0))
        {
            ((*ctx).ev.scheduleTimer)
                .unwrap()((*ctx).ev.data, *(*ctx).c.command_timeout);
        }
    } else if ((*ctx).ev.scheduleTimer).is_some()
        && (!((*ctx).c.connect_timeout).is_null()
            && ((*(*ctx).c.connect_timeout).tv_sec != 0
                || (*(*ctx).c.connect_timeout).tv_usec != 0))
    {
        ((*ctx).ev.scheduleTimer).unwrap()((*ctx).ev.data, *(*ctx).c.connect_timeout);
    }
}
unsafe extern "C" fn callbackHash(mut key: *const libc::c_void) -> libc::c_uint {
    return dictGenHashFunction(
        key as *const libc::c_uchar,
        sdslen(key as sds) as libc::c_int,
    );
}
unsafe extern "C" fn callbackValDup(
    mut privdata: *mut libc::c_void,
    mut src: *const libc::c_void,
) -> *mut libc::c_void {
    let mut dup: *mut redisCallback = 0 as *mut redisCallback;
    dup = hi_malloc(::std::mem::size_of::<redisCallback>() as libc::c_ulong)
        as *mut redisCallback;
    if dup.is_null() {
        return 0 as *mut libc::c_void;
    }
    memcpy(
        dup as *mut libc::c_void,
        src,
        ::std::mem::size_of::<redisCallback>() as libc::c_ulong,
    );
    return dup as *mut libc::c_void;
}
unsafe extern "C" fn callbackKeyCompare(
    mut privdata: *mut libc::c_void,
    mut key1: *const libc::c_void,
    mut key2: *const libc::c_void,
) -> libc::c_int {
    let mut l1: libc::c_int = 0;
    let mut l2: libc::c_int = 0;
    l1 = sdslen(key1 as sds) as libc::c_int;
    l2 = sdslen(key2 as sds) as libc::c_int;
    if l1 != l2 {
        return 0 as libc::c_int;
    }
    return (memcmp(key1, key2, l1 as libc::c_ulong) == 0 as libc::c_int) as libc::c_int;
}
unsafe extern "C" fn callbackKeyDestructor(
    mut privdata: *mut libc::c_void,
    mut key: *mut libc::c_void,
) {
    sdsfree(key as sds);
}
unsafe extern "C" fn callbackValDestructor(
    mut privdata: *mut libc::c_void,
    mut val: *mut libc::c_void,
) {
    hi_free(val);
}
static mut callbackDict: dictType = unsafe {
    {
        let mut init = dictType {
            hashFunction: Some(
                callbackHash as unsafe extern "C" fn(*const libc::c_void) -> libc::c_uint,
            ),
            keyDup: None,
            valDup: Some(
                callbackValDup
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const libc::c_void,
                    ) -> *mut libc::c_void,
            ),
            keyCompare: Some(
                callbackKeyCompare
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const libc::c_void,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
            keyDestructor: Some(
                callbackKeyDestructor
                    as unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
            ),
            valDestructor: Some(
                callbackValDestructor
                    as unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
            ),
        };
        init
    }
};
unsafe extern "C" fn redisAsyncInitialize(
    mut c: *mut redisContext,
) -> *mut redisAsyncContext {
    let mut ac: *mut redisAsyncContext = 0 as *mut redisAsyncContext;
    let mut channels: *mut dict = 0 as *mut dict;
    let mut patterns: *mut dict = 0 as *mut dict;
    channels = dictCreate(&mut callbackDict, 0 as *mut libc::c_void);
    if !channels.is_null() {
        patterns = dictCreate(&mut callbackDict, 0 as *mut libc::c_void);
        if !patterns.is_null() {
            ac = hi_realloc(
                c as *mut libc::c_void,
                ::std::mem::size_of::<redisAsyncContext>() as libc::c_ulong,
            ) as *mut redisAsyncContext;
            if !ac.is_null() {
                c = &mut (*ac).c;
                (*c).flags &= !(0x2 as libc::c_int);
                (*ac).err = 0 as libc::c_int;
                (*ac).errstr = 0 as *mut libc::c_char;
                (*ac).data = 0 as *mut libc::c_void;
                (*ac).dataCleanup = None;
                (*ac).ev.data = 0 as *mut libc::c_void;
                (*ac).ev.addRead = None;
                (*ac).ev.delRead = None;
                (*ac).ev.addWrite = None;
                (*ac).ev.delWrite = None;
                (*ac).ev.cleanup = None;
                (*ac).ev.scheduleTimer = None;
                (*ac).onConnect = None;
                (*ac).onConnectNC = None;
                (*ac).onDisconnect = None;
                (*ac).replies.head = 0 as *mut redisCallback;
                (*ac).replies.tail = 0 as *mut redisCallback;
                (*ac).sub.replies.head = 0 as *mut redisCallback;
                (*ac).sub.replies.tail = 0 as *mut redisCallback;
                (*ac).sub.channels = channels;
                (*ac).sub.patterns = patterns;
                (*ac).sub.pending_unsubs = 0 as libc::c_int;
                return ac;
            }
        }
    }
    if !channels.is_null() {
        dictRelease(channels);
    }
    if !patterns.is_null() {
        dictRelease(patterns);
    }
    return 0 as *mut redisAsyncContext;
}
unsafe extern "C" fn __redisAsyncCopyError(mut ac: *mut redisAsyncContext) {
    if ac.is_null() {
        return;
    }
    let mut c: *mut redisContext = &mut (*ac).c;
    (*ac).err = (*c).err;
    (*ac).errstr = ((*c).errstr).as_mut_ptr();
}
pub unsafe extern "C" fn redisAsyncConnectWithOptions(
    mut options: *const redisOptions,
) -> *mut redisAsyncContext {
    let mut myOptions: redisOptions = *options;
    let mut c: *mut redisContext = 0 as *mut redisContext;
    let mut ac: *mut redisAsyncContext = 0 as *mut redisAsyncContext;
    myOptions.push_cb = None;
    myOptions.options |= 0x8 as libc::c_int;
    myOptions.options |= 0x1 as libc::c_int;
    c = redisConnectWithOptions(&mut myOptions);
    if c.is_null() {
        return 0 as *mut redisAsyncContext;
    }
    ac = redisAsyncInitialize(c);
    if ac.is_null() {
        redisFree(c);
        return 0 as *mut redisAsyncContext;
    }
    redisAsyncSetPushCallback(ac, myOptions.async_push_cb);
    __redisAsyncCopyError(ac);
    return ac;
}
pub unsafe extern "C" fn redisAsyncConnect(
    mut ip: *const libc::c_char,
    mut port: libc::c_int,
) -> *mut redisAsyncContext {
    let mut options: redisOptions = {
        let mut init = redisOptions {
            type_0: 0 as libc::c_int,
            options: 0,
            connect_timeout: 0 as *const timeval,
            command_timeout: 0 as *const timeval,
            endpoint: C2RustUnnamed_3 {
                tcp: C2RustUnnamed_4 {
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
    return redisAsyncConnectWithOptions(&mut options);
}
pub unsafe extern "C" fn redisAsyncConnectBind(
    mut ip: *const libc::c_char,
    mut port: libc::c_int,
    mut source_addr: *const libc::c_char,
) -> *mut redisAsyncContext {
    let mut options: redisOptions = {
        let mut init = redisOptions {
            type_0: 0 as libc::c_int,
            options: 0,
            connect_timeout: 0 as *const timeval,
            command_timeout: 0 as *const timeval,
            endpoint: C2RustUnnamed_3 {
                tcp: C2RustUnnamed_4 {
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
    return redisAsyncConnectWithOptions(&mut options);
}
pub unsafe extern "C" fn redisAsyncConnectBindWithReuse(
    mut ip: *const libc::c_char,
    mut port: libc::c_int,
    mut source_addr: *const libc::c_char,
) -> *mut redisAsyncContext {
    let mut options: redisOptions = {
        let mut init = redisOptions {
            type_0: 0 as libc::c_int,
            options: 0,
            connect_timeout: 0 as *const timeval,
            command_timeout: 0 as *const timeval,
            endpoint: C2RustUnnamed_3 {
                tcp: C2RustUnnamed_4 {
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
    options.options |= 0x2 as libc::c_int;
    options.endpoint.tcp.source_addr = source_addr;
    return redisAsyncConnectWithOptions(&mut options);
}
pub unsafe extern "C" fn redisAsyncConnectUnix(
    mut path: *const libc::c_char,
) -> *mut redisAsyncContext {
    let mut options: redisOptions = {
        let mut init = redisOptions {
            type_0: 0 as libc::c_int,
            options: 0,
            connect_timeout: 0 as *const timeval,
            command_timeout: 0 as *const timeval,
            endpoint: C2RustUnnamed_3 {
                tcp: C2RustUnnamed_4 {
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
    return redisAsyncConnectWithOptions(&mut options);
}
unsafe extern "C" fn redisAsyncSetConnectCallbackImpl(
    mut ac: *mut redisAsyncContext,
    mut fn_0: Option::<redisConnectCallback>,
    mut fn_nc: Option::<redisConnectCallbackNC>,
) -> libc::c_int {
    if ((*ac).onConnect).is_some() || ((*ac).onConnectNC).is_some() {
        return -(1 as libc::c_int);
    }
    if fn_0.is_some() {
        (*ac).onConnect = fn_0;
    } else if fn_nc.is_some() {
        (*ac).onConnectNC = fn_nc;
    }
    refreshTimeout(ac);
    if ((*ac).ev.addWrite).is_some() {
        ((*ac).ev.addWrite).unwrap()((*ac).ev.data);
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn redisAsyncSetConnectCallback(
    mut ac: *mut redisAsyncContext,
    mut fn_0: Option::<redisConnectCallback>,
) -> libc::c_int {
    return redisAsyncSetConnectCallbackImpl(ac, fn_0, None);
}
pub unsafe extern "C" fn redisAsyncSetConnectCallbackNC(
    mut ac: *mut redisAsyncContext,
    mut fn_0: Option::<redisConnectCallbackNC>,
) -> libc::c_int {
    return redisAsyncSetConnectCallbackImpl(ac, None, fn_0);
}
pub unsafe extern "C" fn redisAsyncSetDisconnectCallback(
    mut ac: *mut redisAsyncContext,
    mut fn_0: Option::<redisDisconnectCallback>,
) -> libc::c_int {
    if ((*ac).onDisconnect).is_none() {
        (*ac).onDisconnect = fn_0;
        return 0 as libc::c_int;
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn __redisPushCallback(
    mut list: *mut redisCallbackList,
    mut source: *mut redisCallback,
) -> libc::c_int {
    let mut cb: *mut redisCallback = 0 as *mut redisCallback;
    cb = hi_malloc(::std::mem::size_of::<redisCallback>() as libc::c_ulong)
        as *mut redisCallback;
    if cb.is_null() {
        return 5 as libc::c_int;
    }
    if !source.is_null() {
        memcpy(
            cb as *mut libc::c_void,
            source as *const libc::c_void,
            ::std::mem::size_of::<redisCallback>() as libc::c_ulong,
        );
        (*cb).next = 0 as *mut redisCallback;
    }
    if ((*list).head).is_null() {
        (*list).head = cb;
    }
    if !((*list).tail).is_null() {
        (*(*list).tail).next = cb;
    }
    (*list).tail = cb;
    return 0 as libc::c_int;
}
unsafe extern "C" fn __redisShiftCallback(
    mut list: *mut redisCallbackList,
    mut target: *mut redisCallback,
) -> libc::c_int {
    let mut cb: *mut redisCallback = (*list).head;
    if !cb.is_null() {
        (*list).head = (*cb).next;
        if cb == (*list).tail {
            (*list).tail = 0 as *mut redisCallback;
        }
        if !target.is_null() {
            memcpy(
                target as *mut libc::c_void,
                cb as *const libc::c_void,
                ::std::mem::size_of::<redisCallback>() as libc::c_ulong,
            );
        }
        hi_free(cb as *mut libc::c_void);
        return 0 as libc::c_int;
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn __redisRunCallback(
    mut ac: *mut redisAsyncContext,
    mut cb: *mut redisCallback,
    mut reply: *mut redisReply,
) {
    let mut c: *mut redisContext = &mut (*ac).c;
    if ((*cb).fn_0).is_some() {
        (*c).flags |= 0x10 as libc::c_int;
        ((*cb).fn_0).unwrap()(ac, reply as *mut libc::c_void, (*cb).privdata);
        (*c).flags &= !(0x10 as libc::c_int);
    }
}
unsafe extern "C" fn __redisRunPushCallback(
    mut ac: *mut redisAsyncContext,
    mut reply: *mut redisReply,
) {
    if ((*ac).push_cb).is_some() {
        (*ac).c.flags |= 0x10 as libc::c_int;
        ((*ac).push_cb).unwrap()(ac, reply as *mut libc::c_void);
        (*ac).c.flags &= !(0x10 as libc::c_int);
    }
}
unsafe extern "C" fn __redisRunConnectCallback(
    mut ac: *mut redisAsyncContext,
    mut status: libc::c_int,
) {
    if ((*ac).onConnect).is_none() && ((*ac).onConnectNC).is_none() {
        return;
    }
    if (*ac).c.flags & 0x10 as libc::c_int == 0 {
        (*ac).c.flags |= 0x10 as libc::c_int;
        if ((*ac).onConnect).is_some() {
            ((*ac).onConnect).unwrap()(ac, status);
        } else {
            ((*ac).onConnectNC).unwrap()(ac, status);
        }
        (*ac).c.flags &= !(0x10 as libc::c_int);
    } else if ((*ac).onConnect).is_some() {
        ((*ac).onConnect).unwrap()(ac, status);
    } else {
        ((*ac).onConnectNC).unwrap()(ac, status);
    };
}
unsafe extern "C" fn __redisRunDisconnectCallback(
    mut ac: *mut redisAsyncContext,
    mut status: libc::c_int,
) {
    if ((*ac).onDisconnect).is_some() {
        if (*ac).c.flags & 0x10 as libc::c_int == 0 {
            (*ac).c.flags |= 0x10 as libc::c_int;
            ((*ac).onDisconnect).unwrap()(ac, status);
            (*ac).c.flags &= !(0x10 as libc::c_int);
        } else {
            ((*ac).onDisconnect).unwrap()(ac, status);
        }
    }
}
unsafe extern "C" fn __redisAsyncFree(mut ac: *mut redisAsyncContext) {
    let mut c: *mut redisContext = &mut (*ac).c;
    let mut cb: redisCallback = redisCallback {
        next: 0 as *mut redisCallback,
        fn_0: None,
        pending_subs: 0,
        unsubscribe_sent: 0,
        privdata: 0 as *mut libc::c_void,
    };
    let mut it: dictIterator = dictIterator {
        ht: 0 as *mut dict,
        index: 0,
        entry: 0 as *mut dictEntry,
        nextEntry: 0 as *mut dictEntry,
    };
    let mut de: *mut dictEntry = 0 as *mut dictEntry;
    while __redisShiftCallback(&mut (*ac).replies, &mut cb) == 0 as libc::c_int {
        __redisRunCallback(ac, &mut cb, 0 as *mut redisReply);
    }
    while __redisShiftCallback(&mut (*ac).sub.replies, &mut cb) == 0 as libc::c_int {
        __redisRunCallback(ac, &mut cb, 0 as *mut redisReply);
    }
    if !((*ac).sub.channels).is_null() {
        dictInitIterator(&mut it, (*ac).sub.channels);
        loop {
            de = dictNext(&mut it);
            if de.is_null() {
                break;
            }
            __redisRunCallback(
                ac,
                (*de).val as *mut redisCallback,
                0 as *mut redisReply,
            );
        }
        dictRelease((*ac).sub.channels);
    }
    if !((*ac).sub.patterns).is_null() {
        dictInitIterator(&mut it, (*ac).sub.patterns);
        loop {
            de = dictNext(&mut it);
            if de.is_null() {
                break;
            }
            __redisRunCallback(
                ac,
                (*de).val as *mut redisCallback,
                0 as *mut redisReply,
            );
        }
        dictRelease((*ac).sub.patterns);
    }
    if ((*ac).ev.cleanup).is_some() {
        ((*ac).ev.cleanup).unwrap()((*ac).ev.data);
    }
    (*ac).ev.cleanup = None;
    if (*c).flags & 0x2 as libc::c_int != 0 {
        let mut status: libc::c_int = if (*ac).err == 0 as libc::c_int {
            0 as libc::c_int
        } else {
            -(1 as libc::c_int)
        };
        if (*c).flags & 0x8 as libc::c_int != 0 {
            status = 0 as libc::c_int;
        }
        __redisRunDisconnectCallback(ac, status);
    }
    if ((*ac).dataCleanup).is_some() {
        ((*ac).dataCleanup).unwrap()((*ac).data);
    }
    redisFree(c);
}
pub unsafe extern "C" fn redisAsyncFree(mut ac: *mut redisAsyncContext) {
    if ac.is_null() {
        return;
    }
    let mut c: *mut redisContext = &mut (*ac).c;
    (*c).flags |= 0x8 as libc::c_int;
    if (*c).flags & 0x10 as libc::c_int == 0 {
        __redisAsyncFree(ac);
    }
}
pub unsafe extern "C" fn __redisAsyncDisconnect(mut ac: *mut redisAsyncContext) {
    let mut c: *mut redisContext = &mut (*ac).c;
    __redisAsyncCopyError(ac);
    if (*ac).err == 0 as libc::c_int {
        let mut ret: libc::c_int = __redisShiftCallback(
            &mut (*ac).replies,
            0 as *mut redisCallback,
        );
        if ret == -(1 as libc::c_int) {} else {
            __assert_fail(
                b"ret == REDIS_ERR\0" as *const u8 as *const libc::c_char,
                b"async.c\0" as *const u8 as *const libc::c_char,
                436 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 49],
                    &[libc::c_char; 49],
                >(b"void __redisAsyncDisconnect(redisAsyncContext *)\0"))
                    .as_ptr(),
            );
        };
    } else {
        (*c).flags |= 0x4 as libc::c_int;
    }
    if ((*ac).ev.cleanup).is_some() {
        ((*ac).ev.cleanup).unwrap()((*ac).ev.data);
    }
    (*ac).ev.cleanup = None;
    if (*c).flags & 0x200 as libc::c_int == 0 {
        __redisAsyncFree(ac);
    }
}
pub unsafe extern "C" fn redisAsyncDisconnect(mut ac: *mut redisAsyncContext) {
    let mut c: *mut redisContext = &mut (*ac).c;
    (*c).flags |= 0x4 as libc::c_int;
    (*c).flags &= !(0x200 as libc::c_int);
    if (*c).flags & 0x10 as libc::c_int == 0 && ((*ac).replies.head).is_null() {
        __redisAsyncDisconnect(ac);
    }
}
unsafe extern "C" fn __redisGetSubscribeCallback(
    mut ac: *mut redisAsyncContext,
    mut reply: *mut redisReply,
    mut dstcb: *mut redisCallback,
) -> libc::c_int {
    let mut c: *mut redisContext = &mut (*ac).c;
    let mut callbacks: *mut dict = 0 as *mut dict;
    let mut cb: *mut redisCallback = 0 as *mut redisCallback;
    let mut de: *mut dictEntry = 0 as *mut dictEntry;
    let mut pvariant: libc::c_int = 0;
    let mut stype: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sname: sds = 0 as sds;
    if (*reply).type_0 == 2 as libc::c_int && (*c).flags & 0x100 as libc::c_int == 0
        && (*reply).elements >= 3 as libc::c_int as libc::c_ulong
        || (*reply).type_0 == 12 as libc::c_int
    {
        if (**((*reply).element).offset(0 as libc::c_int as isize)).type_0
            == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"reply->element[0]->type == REDIS_REPLY_STRING\0" as *const u8
                    as *const libc::c_char,
                b"async.c\0" as *const u8 as *const libc::c_char,
                484 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 84],
                    &[libc::c_char; 84],
                >(
                    b"int __redisGetSubscribeCallback(redisAsyncContext *, redisReply *, redisCallback *)\0",
                ))
                    .as_ptr(),
            );
        };
        stype = (**((*reply).element).offset(0 as libc::c_int as isize)).str_0;
        pvariant = if ({
            let mut __res: libc::c_int = 0;
            if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                > 1 as libc::c_int as libc::c_ulong
            {
                if 0 != 0 {
                    let mut __c: libc::c_int = *stype.offset(0 as libc::c_int as isize)
                        as libc::c_int;
                    __res = (if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                        __c
                    } else {
                        *(*__ctype_tolower_loc()).offset(__c as isize)
                    });
                } else {
                    __res = tolower(
                        *stype.offset(0 as libc::c_int as isize) as libc::c_int,
                    );
                }
            } else {
                __res = *(*__ctype_tolower_loc())
                    .offset(
                        *stype.offset(0 as libc::c_int as isize) as libc::c_int as isize,
                    );
            }
            __res
        }) == 'p' as i32
        {
            1 as libc::c_int
        } else {
            0 as libc::c_int
        };
        if pvariant != 0 {
            callbacks = (*ac).sub.patterns;
        } else {
            callbacks = (*ac).sub.channels;
        }
        if (**((*reply).element).offset(1 as libc::c_int as isize)).type_0
            == 1 as libc::c_int
        {
            sname = sdsnewlen(
                (**((*reply).element).offset(1 as libc::c_int as isize)).str_0
                    as *const libc::c_void,
                (**((*reply).element).offset(1 as libc::c_int as isize)).len,
            );
            if sname.is_null() {
                __redisSetError(
                    &mut (*ac).c,
                    5 as libc::c_int,
                    b"Out of memory\0" as *const u8 as *const libc::c_char,
                );
                __redisAsyncCopyError(ac);
                return -(1 as libc::c_int);
            } else {
                de = dictFind(callbacks, sname as *const libc::c_void);
                if !de.is_null() {
                    cb = (*de).val as *mut redisCallback;
                    memcpy(
                        dstcb as *mut libc::c_void,
                        cb as *const libc::c_void,
                        ::std::mem::size_of::<redisCallback>() as libc::c_ulong,
                    );
                }
            }
        }
        if strcasecmp(
            stype.offset(pvariant as isize),
            b"subscribe\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            if !cb.is_null() {} else {
                __assert_fail(
                    b"cb != NULL\0" as *const u8 as *const libc::c_char,
                    b"async.c\0" as *const u8 as *const libc::c_char,
                    506 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 84],
                        &[libc::c_char; 84],
                    >(
                        b"int __redisGetSubscribeCallback(redisAsyncContext *, redisReply *, redisCallback *)\0",
                    ))
                        .as_ptr(),
                );
            };
            (*cb).pending_subs -= 1 as libc::c_int;
        } else if strcasecmp(
            stype.offset(pvariant as isize),
            b"unsubscribe\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            if cb.is_null() {
                (*ac).sub.pending_unsubs -= 1 as libc::c_int;
            } else if (*cb).pending_subs == 0 as libc::c_int {
                dictDelete(callbacks, sname as *const libc::c_void);
            }
            if (**((*reply).element).offset(2 as libc::c_int as isize)).type_0
                == 3 as libc::c_int
            {} else {
                __assert_fail(
                    b"reply->element[2]->type == REDIS_REPLY_INTEGER\0" as *const u8
                        as *const libc::c_char,
                    b"async.c\0" as *const u8 as *const libc::c_char,
                    517 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 84],
                        &[libc::c_char; 84],
                    >(
                        b"int __redisGetSubscribeCallback(redisAsyncContext *, redisReply *, redisCallback *)\0",
                    ))
                        .as_ptr(),
                );
            };
            if (**((*reply).element).offset(2 as libc::c_int as isize)).integer
                == 0 as libc::c_int as libc::c_longlong
                && (*(*ac).sub.channels).used == 0 as libc::c_int as libc::c_ulong
                && (*(*ac).sub.patterns).used == 0 as libc::c_int as libc::c_ulong
                && (*ac).sub.pending_unsubs == 0 as libc::c_int
            {
                (*c).flags &= !(0x20 as libc::c_int);
                let mut cb_0: redisCallback = redisCallback {
                    next: 0 as *mut redisCallback,
                    fn_0: None,
                    pending_subs: 0,
                    unsubscribe_sent: 0,
                    privdata: 0 as *mut libc::c_void,
                };
                while __redisShiftCallback(&mut (*ac).sub.replies, &mut cb_0)
                    == 0 as libc::c_int
                {
                    __redisPushCallback(&mut (*ac).replies, &mut cb_0);
                }
            }
        }
        sdsfree(sname);
    } else {
        __redisShiftCallback(&mut (*ac).sub.replies, dstcb);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn redisIsSubscribeReply(mut reply: *mut redisReply) -> libc::c_int {
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: size_t = 0;
    let mut off: size_t = 0;
    if (*reply).elements < 1 as libc::c_int as libc::c_ulong
        || (**((*reply).element).offset(0 as libc::c_int as isize)).type_0
            != 1 as libc::c_int
        || (**((*reply).element).offset(0 as libc::c_int as isize)).len
            < (::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
    {
        return 0 as libc::c_int;
    }
    off = (({
        let mut __res: libc::c_int = 0;
        if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
            > 1 as libc::c_int as libc::c_ulong
        {
            if 0 != 0 {
                let mut __c: libc::c_int = *((**((*reply).element)
                    .offset(0 as libc::c_int as isize))
                    .str_0)
                    .offset(0 as libc::c_int as isize) as libc::c_int;
                __res = (if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                    __c
                } else {
                    *(*__ctype_tolower_loc()).offset(__c as isize)
                });
            } else {
                __res = tolower(
                    *((**((*reply).element).offset(0 as libc::c_int as isize)).str_0)
                        .offset(0 as libc::c_int as isize) as libc::c_int,
                );
            }
        } else {
            __res = *(*__ctype_tolower_loc())
                .offset(
                    *((**((*reply).element).offset(0 as libc::c_int as isize)).str_0)
                        .offset(0 as libc::c_int as isize) as libc::c_int as isize,
                );
        }
        __res
    }) == 'p' as i32) as libc::c_int as size_t;
    str = ((**((*reply).element).offset(0 as libc::c_int as isize)).str_0)
        .offset(off as isize);
    len = ((**((*reply).element).offset(0 as libc::c_int as isize)).len)
        .wrapping_sub(off);
    return (strncasecmp(str, b"subscribe\0" as *const u8 as *const libc::c_char, len)
        == 0
        || strncasecmp(str, b"message\0" as *const u8 as *const libc::c_char, len) == 0
        || strncasecmp(str, b"unsubscribe\0" as *const u8 as *const libc::c_char, len)
            == 0) as libc::c_int;
}
pub unsafe extern "C" fn redisProcessCallbacks(mut ac: *mut redisAsyncContext) {
    let mut c: *mut redisContext = &mut (*ac).c;
    let mut reply: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut status: libc::c_int = 0;
    loop {
        status = redisGetReply(c, &mut reply);
        if !(status == 0 as libc::c_int) {
            break;
        }
        if reply.is_null() {
            if (*c).flags & 0x4 as libc::c_int != 0
                && sdslen((*c).obuf) == 0 as libc::c_int as libc::c_ulong
                && ((*ac).replies.head).is_null()
            {
                __redisAsyncDisconnect(ac);
                return;
            }
            break;
        } else {
            if (*(reply as *mut redisReply)).type_0 == 12 as libc::c_int {
                (*c).flags |= 0x100 as libc::c_int;
            }
            if (*(reply as *mut redisReply)).type_0 == 12 as libc::c_int
                && redisIsSubscribeReply(reply as *mut redisReply) == 0
            {
                __redisRunPushCallback(ac, reply as *mut redisReply);
                ((*(*(*c).reader).fn_0).freeObject).unwrap()(reply);
            } else {
                let mut cb: redisCallback = {
                    let mut init = redisCallback {
                        next: 0 as *mut redisCallback,
                        fn_0: None,
                        pending_subs: 0 as libc::c_int,
                        unsubscribe_sent: 0 as libc::c_int,
                        privdata: 0 as *mut libc::c_void,
                    };
                    init
                };
                if __redisShiftCallback(&mut (*ac).replies, &mut cb) != 0 as libc::c_int
                {
                    if (*(reply as *mut redisReply)).type_0 == 6 as libc::c_int {
                        (*c).err = 2 as libc::c_int;
                        snprintf(
                            ((*c).errstr).as_mut_ptr(),
                            ::std::mem::size_of::<[libc::c_char; 128]>()
                                as libc::c_ulong,
                            b"%s\0" as *const u8 as *const libc::c_char,
                            (*(reply as *mut redisReply)).str_0,
                        );
                        ((*(*(*c).reader).fn_0).freeObject).unwrap()(reply);
                        __redisAsyncDisconnect(ac);
                        return;
                    }
                    if (*c).flags & 0x20 as libc::c_int != 0 {} else {
                        __assert_fail(
                            b"c->flags & REDIS_SUBSCRIBED\0" as *const u8
                                as *const libc::c_char,
                            b"async.c\0" as *const u8 as *const libc::c_char,
                            629 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 48],
                                &[libc::c_char; 48],
                            >(b"void redisProcessCallbacks(redisAsyncContext *)\0"))
                                .as_ptr(),
                        );
                    };
                    if (*c).flags & 0x20 as libc::c_int != 0 {
                        __redisGetSubscribeCallback(
                            ac,
                            reply as *mut redisReply,
                            &mut cb,
                        );
                    }
                }
                if (cb.fn_0).is_some() {
                    __redisRunCallback(ac, &mut cb, reply as *mut redisReply);
                    if (*c).flags & 0x400 as libc::c_int == 0 {
                        ((*(*(*c).reader).fn_0).freeObject).unwrap()(reply);
                    }
                    if (*c).flags & 0x8 as libc::c_int != 0 {
                        __redisAsyncFree(ac);
                        return;
                    }
                } else {
                    ((*(*(*c).reader).fn_0).freeObject).unwrap()(reply);
                }
                if (*c).flags & 0x40 as libc::c_int != 0 {
                    __redisPushCallback(&mut (*ac).replies, &mut cb);
                }
            }
        }
    }
    if status != 0 as libc::c_int {
        __redisAsyncDisconnect(ac);
    }
}
unsafe extern "C" fn __redisAsyncHandleConnectFailure(mut ac: *mut redisAsyncContext) {
    __redisRunConnectCallback(ac, -(1 as libc::c_int));
    __redisAsyncDisconnect(ac);
}
unsafe extern "C" fn __redisAsyncHandleConnect(
    mut ac: *mut redisAsyncContext,
) -> libc::c_int {
    let mut completed: libc::c_int = 0 as libc::c_int;
    let mut c: *mut redisContext = &mut (*ac).c;
    if redisCheckConnectDone(c, &mut completed) == -(1 as libc::c_int) {
        if redisCheckSocketError(c) == -(1 as libc::c_int) {
            __redisAsyncCopyError(ac);
        }
        __redisAsyncHandleConnectFailure(ac);
        return -(1 as libc::c_int);
    } else if completed == 1 as libc::c_int {
        if (*c).connection_type as libc::c_uint
            == REDIS_CONN_TCP as libc::c_int as libc::c_uint
            && redisSetTcpNoDelay(c) == -(1 as libc::c_int)
        {
            __redisAsyncHandleConnectFailure(ac);
            return -(1 as libc::c_int);
        }
        (*c).flags |= 0x2 as libc::c_int;
        __redisRunConnectCallback(ac, 0 as libc::c_int);
        if (*ac).c.flags & 0x4 as libc::c_int != 0 {
            redisAsyncDisconnect(ac);
            return -(1 as libc::c_int);
        } else if (*ac).c.flags & 0x8 as libc::c_int != 0 {
            redisAsyncFree(ac);
            return -(1 as libc::c_int);
        }
        return 0 as libc::c_int;
    } else {
        return 0 as libc::c_int
    };
}
pub unsafe extern "C" fn redisAsyncRead(mut ac: *mut redisAsyncContext) {
    let mut c: *mut redisContext = &mut (*ac).c;
    if redisBufferRead(c) == -(1 as libc::c_int) {
        __redisAsyncDisconnect(ac);
    } else {
        refreshTimeout(ac);
        if ((*ac).ev.addRead).is_some() {
            ((*ac).ev.addRead).unwrap()((*ac).ev.data);
        }
        redisProcessCallbacks(ac);
    };
}
pub unsafe extern "C" fn redisAsyncHandleRead(mut ac: *mut redisAsyncContext) {
    let mut c: *mut redisContext = &mut (*ac).c;
    if (*c).flags & 0x10 as libc::c_int == 0 {} else {
        __assert_fail(
            b"!(c->flags & REDIS_IN_CALLBACK)\0" as *const u8 as *const libc::c_char,
            b"async.c\0" as *const u8 as *const libc::c_char,
            727 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 47],
                &[libc::c_char; 47],
            >(b"void redisAsyncHandleRead(redisAsyncContext *)\0"))
                .as_ptr(),
        );
    };
    if (*c).flags & 0x2 as libc::c_int == 0 {
        if __redisAsyncHandleConnect(ac) != 0 as libc::c_int {
            return;
        }
        if (*c).flags & 0x2 as libc::c_int == 0 {
            return;
        }
    }
    ((*(*c).funcs).async_read).unwrap()(ac);
}
pub unsafe extern "C" fn redisAsyncWrite(mut ac: *mut redisAsyncContext) {
    let mut c: *mut redisContext = &mut (*ac).c;
    let mut done: libc::c_int = 0 as libc::c_int;
    if redisBufferWrite(c, &mut done) == -(1 as libc::c_int) {
        __redisAsyncDisconnect(ac);
    } else {
        if done == 0 {
            refreshTimeout(ac);
            if ((*ac).ev.addWrite).is_some() {
                ((*ac).ev.addWrite).unwrap()((*ac).ev.data);
            }
        } else if ((*ac).ev.delWrite).is_some() {
            ((*ac).ev.delWrite).unwrap()((*ac).ev.data);
        }
        refreshTimeout(ac);
        if ((*ac).ev.addRead).is_some() {
            ((*ac).ev.addRead).unwrap()((*ac).ev.data);
        }
    };
}
pub unsafe extern "C" fn redisAsyncHandleWrite(mut ac: *mut redisAsyncContext) {
    let mut c: *mut redisContext = &mut (*ac).c;
    if (*c).flags & 0x10 as libc::c_int == 0 {} else {
        __assert_fail(
            b"!(c->flags & REDIS_IN_CALLBACK)\0" as *const u8 as *const libc::c_char,
            b"async.c\0" as *const u8 as *const libc::c_char,
            762 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 48],
                &[libc::c_char; 48],
            >(b"void redisAsyncHandleWrite(redisAsyncContext *)\0"))
                .as_ptr(),
        );
    };
    if (*c).flags & 0x2 as libc::c_int == 0 {
        if __redisAsyncHandleConnect(ac) != 0 as libc::c_int {
            return;
        }
        if (*c).flags & 0x2 as libc::c_int == 0 {
            return;
        }
    }
    ((*(*c).funcs).async_write).unwrap()(ac);
}
pub unsafe extern "C" fn redisAsyncHandleTimeout(mut ac: *mut redisAsyncContext) {
    let mut c: *mut redisContext = &mut (*ac).c;
    let mut cb: redisCallback = redisCallback {
        next: 0 as *mut redisCallback,
        fn_0: None,
        pending_subs: 0,
        unsubscribe_sent: 0,
        privdata: 0 as *mut libc::c_void,
    };
    if (*c).flags & 0x10 as libc::c_int == 0 {} else {
        __assert_fail(
            b"!(c->flags & REDIS_IN_CALLBACK)\0" as *const u8 as *const libc::c_char,
            b"async.c\0" as *const u8 as *const libc::c_char,
            780 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 50],
                &[libc::c_char; 50],
            >(b"void redisAsyncHandleTimeout(redisAsyncContext *)\0"))
                .as_ptr(),
        );
    };
    if (*c).flags & 0x2 as libc::c_int != 0 {
        if ((*ac).replies.head).is_null() && ((*ac).sub.replies.head).is_null() {
            return;
        }
        if ((*ac).c.command_timeout).is_null()
            || (*(*ac).c.command_timeout).tv_sec == 0
                && (*(*ac).c.command_timeout).tv_usec == 0
        {
            return;
        }
    }
    if (*c).err == 0 {
        __redisSetError(
            c,
            6 as libc::c_int,
            b"Timeout\0" as *const u8 as *const libc::c_char,
        );
        __redisAsyncCopyError(ac);
    }
    if (*c).flags & 0x2 as libc::c_int == 0 {
        __redisRunConnectCallback(ac, -(1 as libc::c_int));
    }
    while __redisShiftCallback(&mut (*ac).replies, &mut cb) == 0 as libc::c_int {
        __redisRunCallback(ac, &mut cb, 0 as *mut redisReply);
    }
    __redisAsyncDisconnect(ac);
}
unsafe extern "C" fn nextArgument(
    mut start: *const libc::c_char,
    mut str: *mut *const libc::c_char,
    mut len: *mut size_t,
) -> *const libc::c_char {
    let mut p: *const libc::c_char = start;
    if *p.offset(0 as libc::c_int as isize) as libc::c_int != '$' as i32 {
        p = strchr(p, '$' as i32);
        if p.is_null() {
            return 0 as *const libc::c_char;
        }
    }
    *len = strtol(
        p.offset(1 as libc::c_int as isize),
        0 as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int as size_t;
    p = strchr(p, '\r' as i32);
    if !p.is_null() {} else {
        __assert_fail(
            b"p\0" as *const u8 as *const libc::c_char,
            b"async.c\0" as *const u8 as *const libc::c_char,
            826 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 64],
                &[libc::c_char; 64],
            >(b"const char *nextArgument(const char *, const char **, size_t *)\0"))
                .as_ptr(),
        );
    };
    *str = p.offset(2 as libc::c_int as isize);
    return p
        .offset(2 as libc::c_int as isize)
        .offset(*len as isize)
        .offset(2 as libc::c_int as isize);
}
unsafe extern "C" fn __redisAsyncCommand(
    mut ac: *mut redisAsyncContext,
    mut fn_0: Option::<redisCallbackFn>,
    mut privdata: *mut libc::c_void,
    mut cmd: *const libc::c_char,
    mut len: size_t,
) -> libc::c_int {
    let mut current_block: u64;
    let mut c: *mut redisContext = &mut (*ac).c;
    let mut cb: redisCallback = redisCallback {
        next: 0 as *mut redisCallback,
        fn_0: None,
        pending_subs: 0,
        unsubscribe_sent: 0,
        privdata: 0 as *mut libc::c_void,
    };
    let mut cbdict: *mut dict = 0 as *mut dict;
    let mut it: dictIterator = dictIterator {
        ht: 0 as *mut dict,
        index: 0,
        entry: 0 as *mut dictEntry,
        nextEntry: 0 as *mut dictEntry,
    };
    let mut de: *mut dictEntry = 0 as *mut dictEntry;
    let mut existcb: *mut redisCallback = 0 as *mut redisCallback;
    let mut pvariant: libc::c_int = 0;
    let mut hasnext: libc::c_int = 0;
    let mut cstr: *const libc::c_char = 0 as *const libc::c_char;
    let mut astr: *const libc::c_char = 0 as *const libc::c_char;
    let mut clen: size_t = 0;
    let mut alen: size_t = 0;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut sname: sds = 0 as *mut libc::c_char;
    let mut ret: libc::c_int = 0;
    if (*c).flags & (0x4 as libc::c_int | 0x8 as libc::c_int) != 0 {
        return -(1 as libc::c_int);
    }
    cb.fn_0 = fn_0;
    cb.privdata = privdata;
    cb.pending_subs = 1 as libc::c_int;
    cb.unsubscribe_sent = 0 as libc::c_int;
    p = nextArgument(cmd, &mut cstr, &mut clen);
    if !p.is_null() {} else {
        __assert_fail(
            b"p != NULL\0" as *const u8 as *const libc::c_char,
            b"async.c\0" as *const u8 as *const libc::c_char,
            859 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 94],
                &[libc::c_char; 94],
            >(
                b"int __redisAsyncCommand(redisAsyncContext *, redisCallbackFn *, void *, const char *, size_t)\0",
            ))
                .as_ptr(),
        );
    };
    hasnext = (*p.offset(0 as libc::c_int as isize) as libc::c_int == '$' as i32)
        as libc::c_int;
    pvariant = if ({
        let mut __res: libc::c_int = 0;
        if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
            > 1 as libc::c_int as libc::c_ulong
        {
            if 0 != 0 {
                let mut __c: libc::c_int = *cstr.offset(0 as libc::c_int as isize)
                    as libc::c_int;
                __res = (if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                    __c
                } else {
                    *(*__ctype_tolower_loc()).offset(__c as isize)
                });
            } else {
                __res = tolower(*cstr.offset(0 as libc::c_int as isize) as libc::c_int);
            }
        } else {
            __res = *(*__ctype_tolower_loc())
                .offset(*cstr.offset(0 as libc::c_int as isize) as libc::c_int as isize);
        }
        __res
    }) == 'p' as i32
    {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    };
    cstr = cstr.offset(pvariant as isize);
    clen = (clen as libc::c_ulong).wrapping_sub(pvariant as libc::c_ulong) as size_t
        as size_t;
    if hasnext != 0
        && strncasecmp(
            cstr,
            b"subscribe\r\n\0" as *const u8 as *const libc::c_char,
            11 as libc::c_int as size_t,
        ) == 0 as libc::c_int
    {
        (*c).flags |= 0x20 as libc::c_int;
        loop {
            p = nextArgument(p, &mut astr, &mut alen);
            if p.is_null() {
                current_block = 6545907279487748450;
                break;
            }
            sname = sdsnewlen(astr as *const libc::c_void, alen);
            if sname.is_null() {
                current_block = 18345826251299785256;
                break;
            }
            if pvariant != 0 {
                cbdict = (*ac).sub.patterns;
            } else {
                cbdict = (*ac).sub.channels;
            }
            de = dictFind(cbdict, sname as *const libc::c_void);
            if !de.is_null() {
                existcb = (*de).val as *mut redisCallback;
                cb.pending_subs = (*existcb).pending_subs + 1 as libc::c_int;
            }
            ret = dictReplace(
                cbdict,
                sname as *mut libc::c_void,
                &mut cb as *mut redisCallback as *mut libc::c_void,
            );
            if ret == 0 as libc::c_int {
                sdsfree(sname);
            }
        }
    } else if strncasecmp(
        cstr,
        b"unsubscribe\r\n\0" as *const u8 as *const libc::c_char,
        13 as libc::c_int as size_t,
    ) == 0 as libc::c_int
    {
        if (*c).flags & 0x20 as libc::c_int == 0 {
            return -(1 as libc::c_int);
        }
        if pvariant != 0 {
            cbdict = (*ac).sub.patterns;
        } else {
            cbdict = (*ac).sub.channels;
        }
        if hasnext != 0 {
            loop {
                p = nextArgument(p, &mut astr, &mut alen);
                if p.is_null() {
                    current_block = 6545907279487748450;
                    break;
                }
                sname = sdsnewlen(astr as *const libc::c_void, alen);
                if sname.is_null() {
                    current_block = 18345826251299785256;
                    break;
                }
                de = dictFind(cbdict, sname as *const libc::c_void);
                if !de.is_null() {
                    existcb = (*de).val as *mut redisCallback;
                    if (*existcb).unsubscribe_sent == 0 as libc::c_int {
                        (*existcb).unsubscribe_sent = 1 as libc::c_int;
                    } else {
                        (*ac).sub.pending_unsubs += 1 as libc::c_int;
                    }
                } else {
                    (*ac).sub.pending_unsubs += 1 as libc::c_int;
                }
                sdsfree(sname);
            }
        } else {
            let mut no_subs: libc::c_int = 1 as libc::c_int;
            dictInitIterator(&mut it, cbdict);
            loop {
                de = dictNext(&mut it);
                if de.is_null() {
                    break;
                }
                existcb = (*de).val as *mut redisCallback;
                if (*existcb).unsubscribe_sent == 0 as libc::c_int {
                    (*existcb).unsubscribe_sent = 1 as libc::c_int;
                    no_subs = 0 as libc::c_int;
                }
            }
            if no_subs == 1 as libc::c_int {
                (*ac).sub.pending_unsubs += 1 as libc::c_int;
            }
            current_block = 6545907279487748450;
        }
    } else if strncasecmp(
        cstr,
        b"monitor\r\n\0" as *const u8 as *const libc::c_char,
        9 as libc::c_int as size_t,
    ) == 0 as libc::c_int
    {
        (*c).flags |= 0x40 as libc::c_int;
        if __redisPushCallback(&mut (*ac).replies, &mut cb) != 0 as libc::c_int {
            current_block = 18345826251299785256;
        } else {
            current_block = 6545907279487748450;
        }
    } else if (*c).flags & 0x20 as libc::c_int != 0 {
        if __redisPushCallback(&mut (*ac).sub.replies, &mut cb) != 0 as libc::c_int {
            current_block = 18345826251299785256;
        } else {
            current_block = 6545907279487748450;
        }
    } else if __redisPushCallback(&mut (*ac).replies, &mut cb) != 0 as libc::c_int {
        current_block = 18345826251299785256;
    } else {
        current_block = 6545907279487748450;
    }
    match current_block {
        18345826251299785256 => {
            __redisSetError(
                &mut (*ac).c,
                5 as libc::c_int,
                b"Out of memory\0" as *const u8 as *const libc::c_char,
            );
            __redisAsyncCopyError(ac);
            return -(1 as libc::c_int);
        }
        _ => {
            __redisAppendCommand(c, cmd, len);
            refreshTimeout(ac);
            if ((*ac).ev.addWrite).is_some() {
                ((*ac).ev.addWrite).unwrap()((*ac).ev.data);
            }
            return 0 as libc::c_int;
        }
    };
}
pub unsafe extern "C" fn redisvAsyncCommand(
    mut ac: *mut redisAsyncContext,
    mut fn_0: Option::<redisCallbackFn>,
    mut privdata: *mut libc::c_void,
    mut format: *const libc::c_char,
    mut ap: ::std::ffi::VaList,
) -> libc::c_int {
    let mut cmd: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: libc::c_int = 0;
    let mut status: libc::c_int = 0;
    len = redisvFormatCommand(&mut cmd, format, ap.as_va_list());
    if len < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    status = __redisAsyncCommand(ac, fn_0, privdata, cmd, len as size_t);
    hi_free(cmd as *mut libc::c_void);
    return status;
}
pub unsafe extern "C" fn redisAsyncCommand(
    mut ac: *mut redisAsyncContext,
    mut fn_0: Option::<redisCallbackFn>,
    mut privdata: *mut libc::c_void,
    mut format: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut ap: ::std::ffi::VaListImpl;
    let mut status: libc::c_int = 0;
    ap = args.clone();
    status = redisvAsyncCommand(ac, fn_0, privdata, format, ap.as_va_list());
    return status;
}
pub unsafe extern "C" fn redisAsyncCommandArgv(
    mut ac: *mut redisAsyncContext,
    mut fn_0: Option::<redisCallbackFn>,
    mut privdata: *mut libc::c_void,
    mut argc: libc::c_int,
    mut argv: *mut *const libc::c_char,
    mut argvlen: *const size_t,
) -> libc::c_int {
    let mut cmd: sds = 0 as *mut libc::c_char;
    let mut len: libc::c_longlong = 0;
    let mut status: libc::c_int = 0;
    len = redisFormatSdsCommandArgv(&mut cmd, argc, argv, argvlen);
    if len < 0 as libc::c_int as libc::c_longlong {
        return -(1 as libc::c_int);
    }
    status = __redisAsyncCommand(
        ac,
        fn_0,
        privdata,
        cmd as *const libc::c_char,
        len as size_t,
    );
    sdsfree(cmd);
    return status;
}
pub unsafe extern "C" fn redisAsyncFormattedCommand(
    mut ac: *mut redisAsyncContext,
    mut fn_0: Option::<redisCallbackFn>,
    mut privdata: *mut libc::c_void,
    mut cmd: *const libc::c_char,
    mut len: size_t,
) -> libc::c_int {
    let mut status: libc::c_int = __redisAsyncCommand(ac, fn_0, privdata, cmd, len);
    return status;
}
pub unsafe extern "C" fn redisAsyncSetPushCallback(
    mut ac: *mut redisAsyncContext,
    mut fn_0: Option::<redisAsyncPushFn>,
) -> Option::<redisAsyncPushFn> {
    let mut old: Option::<redisAsyncPushFn> = (*ac).push_cb;
    (*ac).push_cb = fn_0;
    return old;
}
pub unsafe extern "C" fn redisAsyncSetTimeout(
    mut ac: *mut redisAsyncContext,
    mut tv: timeval,
) -> libc::c_int {
    if ((*ac).c.command_timeout).is_null() {
        (*ac)
            .c
            .command_timeout = hi_calloc(
            1 as libc::c_int as size_t,
            ::std::mem::size_of::<timeval>() as libc::c_ulong,
        ) as *mut timeval;
        if ((*ac).c.command_timeout).is_null() {
            __redisSetError(
                &mut (*ac).c,
                5 as libc::c_int,
                b"Out of memory\0" as *const u8 as *const libc::c_char,
            );
            __redisAsyncCopyError(ac);
            return -(1 as libc::c_int);
        }
    }
    if tv.tv_sec != (*(*ac).c.command_timeout).tv_sec
        || tv.tv_usec != (*(*ac).c.command_timeout).tv_usec
    {
        *(*ac).c.command_timeout = tv;
    }
    return 0 as libc::c_int;
}
