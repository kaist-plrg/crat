use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type dict;
    fn poll(__fds: *mut pollfd, __nfds: nfds_t, __timeout: libc::c_int) -> libc::c_int;
    fn freeaddrinfo(__ai: *mut addrinfo);
    fn getaddrinfo(
        __name: *const libc::c_char,
        __service: *const libc::c_char,
        __req: *const addrinfo,
        __pai: *mut *mut addrinfo,
    ) -> libc::c_int;
    fn access(__name: *const libc::c_char, __type: libc::c_int) -> libc::c_int;
    fn usleep(__useconds: __useconds_t) -> libc::c_int;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
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
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
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
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strdup(__s: *const libc::c_char) -> *mut libc::c_char;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strcasecmp(__s1: *const libc::c_char, __s2: *const libc::c_char) -> libc::c_int;
    fn gettimeofday(__tv: *mut timeval, __tz: *mut libc::c_void) -> libc::c_int;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;
    fn __errno_location() -> *mut libc::c_int;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn redisReaderCreate() -> *mut redisReader;
    fn freeReplyObject(reply: *mut libc::c_void);
    fn redisFormatCommand(
        target: *mut *mut libc::c_char,
        format: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn redisFormatCommandArgv(
        target: *mut *mut libc::c_char,
        argc: libc::c_int,
        argv: *mut *const libc::c_char,
        argvlen: *const size_t,
    ) -> libc::c_longlong;
    fn redisFormatSdsCommandArgv(
        target: *mut sds,
        argc: libc::c_int,
        argv: *mut *const libc::c_char,
        argvlen: *const size_t,
    ) -> libc::c_longlong;
    fn redisConnectWithOptions(options: *const redisOptions) -> *mut redisContext;
    fn redisConnect(ip: *const libc::c_char, port: libc::c_int) -> *mut redisContext;
    fn redisConnectWithTimeout(
        ip: *const libc::c_char,
        port: libc::c_int,
        tv: timeval,
    ) -> *mut redisContext;
    fn redisConnectUnix(path: *const libc::c_char) -> *mut redisContext;
    fn redisConnectUnixWithTimeout(
        path: *const libc::c_char,
        tv: timeval,
    ) -> *mut redisContext;
    fn redisConnectFd(fd: redisFD) -> *mut redisContext;
    fn redisReconnect(c: *mut redisContext) -> libc::c_int;
    fn redisSetPushCallback(
        c: *mut redisContext,
        fn_0: Option::<redisPushFn>,
    ) -> Option::<redisPushFn>;
    fn redisSetTimeout(c: *mut redisContext, tv: timeval) -> libc::c_int;
    fn redisEnableKeepAlive(c: *mut redisContext) -> libc::c_int;
    fn redisSetTcpUserTimeout(
        c: *mut redisContext,
        timeout: libc::c_uint,
    ) -> libc::c_int;
    fn redisFree(c: *mut redisContext);
    fn redisFreeKeepFd(c: *mut redisContext) -> redisFD;
    fn redisGetReply(c: *mut redisContext, reply: *mut *mut libc::c_void) -> libc::c_int;
    fn redisAppendFormattedCommand(
        c: *mut redisContext,
        cmd: *const libc::c_char,
        len: size_t,
    ) -> libc::c_int;
    fn redisAppendCommand(
        c: *mut redisContext,
        format: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn sdsempty() -> sds;
    fn sdsfree(s: sds);
    fn hiredisSetAllocators(ha: *mut hiredisAllocFuncs) -> hiredisAllocFuncs;
    fn hiredisResetAllocators();
    static mut hiredisAllocFns: hiredisAllocFuncs;
    fn redisCommand(
        c: *mut redisContext,
        format: *const libc::c_char,
        _: ...
    ) -> *mut libc::c_void;
    fn redisCommandArgv(
        c: *mut redisContext,
        argc: libc::c_int,
        argv: *mut *const libc::c_char,
        argvlen: *const size_t,
    ) -> *mut libc::c_void;
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
    fn redisAsyncConnectWithOptions(
        options: *const redisOptions,
    ) -> *mut redisAsyncContext;
    fn redisAsyncSetConnectCallbackNC(
        ac: *mut redisAsyncContext,
        fn_0: Option::<redisConnectCallbackNC>,
    ) -> libc::c_int;
    fn redisAsyncSetDisconnectCallback(
        ac: *mut redisAsyncContext,
        fn_0: Option::<redisDisconnectCallback>,
    ) -> libc::c_int;
    fn redisAsyncDisconnect(ac: *mut redisAsyncContext);
    fn redisAsyncFree(ac: *mut redisAsyncContext);
    fn redisAsyncHandleRead(ac: *mut redisAsyncContext);
    fn redisAsyncHandleWrite(ac: *mut redisAsyncContext);
    fn redisAsyncHandleTimeout(ac: *mut redisAsyncContext);
    fn redisAsyncCommand(
        ac: *mut redisAsyncContext,
        fn_0: Option::<redisCallbackFn>,
        privdata: *mut libc::c_void,
        format: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __useconds_t = libc::c_uint;
pub type __suseconds_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;
pub type socklen_t = __socklen_t;
pub type sa_family_t = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [libc::c_char; 14],
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct addrinfo {
    pub ai_flags: libc::c_int,
    pub ai_family: libc::c_int,
    pub ai_socktype: libc::c_int,
    pub ai_protocol: libc::c_int,
    pub ai_addrlen: socklen_t,
    pub ai_addr: *mut sockaddr,
    pub ai_canonname: *mut libc::c_char,
    pub ai_next: *mut addrinfo,
}
pub type nfds_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pollfd {
    pub fd: libc::c_int,
    pub events: libc::c_short,
    pub revents: libc::c_short,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type __sighandler_t = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
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
pub struct redisPollEvents {
    pub context: *mut redisAsyncContext,
    pub fd: redisFD,
    pub reading: libc::c_char,
    pub writing: libc::c_char,
    pub in_tick: libc::c_char,
    pub deleted: libc::c_char,
    pub deadline: libc::c_double,
}
pub type connection_type = libc::c_uint;
pub const CONN_SSL: connection_type = 3;
pub const CONN_FD: connection_type = 2;
pub const CONN_UNIX: connection_type = 1;
pub const CONN_TCP: connection_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct config {
    pub type_0: connection_type,
    pub connect_timeout: timeval,
    pub tcp: C2RustUnnamed_7,
    pub unix_sock: C2RustUnnamed_6,
    pub ssl: C2RustUnnamed_5,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub host: *const libc::c_char,
    pub port: libc::c_int,
    pub ca_cert: *const libc::c_char,
    pub cert: *const libc::c_char,
    pub key: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub path: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub host: *const libc::c_char,
    pub port: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct privdata {
    pub dtor_counter: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pushCounters {
    pub nil: libc::c_int,
    pub str_0: libc::c_int,
}
pub type astest_no = libc::c_uint;
pub const ASTEST_ISSUE_931_PING: astest_no = 5;
pub const ASTEST_ISSUE_931: astest_no = 4;
pub const ASTEST_PINGPONG_TIMEOUT: astest_no = 3;
pub const ASTEST_PINGPONG: astest_no = 2;
pub const ASTEST_CONN_TIMEOUT: astest_no = 1;
pub const ASTEST_CONNECT: astest_no = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _astest {
    pub ac: *mut redisAsyncContext,
    pub testno: astest_no,
    pub counter: libc::c_int,
    pub connects: libc::c_int,
    pub connect_status: libc::c_int,
    pub disconnects: libc::c_int,
    pub pongs: libc::c_int,
    pub disconnect_status: libc::c_int,
    pub connected: libc::c_int,
    pub err: libc::c_int,
    pub errstr: [libc::c_char; 256],
}
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
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
unsafe extern "C" fn hi_free(mut ptr: *mut libc::c_void) {
    (hiredisAllocFns.freeFn).unwrap()(ptr);
}
unsafe extern "C" fn redisPollTimevalToDouble(mut tv: *mut timeval) -> libc::c_double {
    if tv.is_null() {
        return 0.0f64;
    }
    return (*tv).tv_sec as libc::c_double
        + (*tv).tv_usec as libc::c_double / 1000000.00f64;
}
unsafe extern "C" fn redisPollGetNow() -> libc::c_double {
    let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    gettimeofday(&mut tv, 0 as *mut libc::c_void);
    return redisPollTimevalToDouble(&mut tv);
}
unsafe extern "C" fn redisPollTick(
    mut ac: *mut redisAsyncContext,
    mut timeout: libc::c_double,
) -> libc::c_int {
    let mut reading: libc::c_int = 0;
    let mut writing: libc::c_int = 0;
    let mut pfd: pollfd = pollfd {
        fd: 0,
        events: 0,
        revents: 0,
    };
    let mut handled: libc::c_int = 0;
    let mut ns: libc::c_int = 0;
    let mut itimeout: libc::c_int = 0;
    let mut e: *mut redisPollEvents = (*ac).ev.data as *mut redisPollEvents;
    if e.is_null() {
        return 0 as libc::c_int;
    }
    reading = (*e).reading as libc::c_int;
    writing = (*e).writing as libc::c_int;
    if reading == 0 && writing == 0 {
        return 0 as libc::c_int;
    }
    pfd.fd = (*e).fd;
    pfd.events = 0 as libc::c_int as libc::c_short;
    if reading != 0 {
        pfd.events = 0x1 as libc::c_int as libc::c_short;
    }
    if writing != 0 {
        pfd.events = (pfd.events as libc::c_int | 0x4 as libc::c_int) as libc::c_short;
    }
    if timeout >= 0.0f64 {
        itimeout = (timeout * 1000.0f64) as libc::c_int;
    } else {
        itimeout = -(1 as libc::c_int);
    }
    ns = poll(&mut pfd, 1 as libc::c_int as nfds_t, itimeout);
    if ns < 0 as libc::c_int {
        if *__errno_location() != 4 as libc::c_int {
            return ns;
        }
        ns = 0 as libc::c_int;
    }
    handled = 0 as libc::c_int;
    (*e).in_tick = 1 as libc::c_int as libc::c_char;
    if ns != 0 {
        if reading != 0 && pfd.revents as libc::c_int & 0x1 as libc::c_int != 0 {
            redisAsyncHandleRead(ac);
            handled |= 1 as libc::c_int;
        }
        if writing != 0
            && pfd.revents as libc::c_int & (0x4 as libc::c_int | 0x8 as libc::c_int)
                != 0
        {
            if (*e).deleted == 0 {
                redisAsyncHandleWrite(ac);
                handled |= 2 as libc::c_int;
            }
        }
    }
    if (*e).deleted == 0 && (*e).deadline != 0.0f64 {
        let mut now: libc::c_double = redisPollGetNow();
        if now >= (*e).deadline {
            (*e).deadline = 0.0f64;
            redisAsyncHandleTimeout(ac);
            handled |= 4 as libc::c_int;
        }
    }
    if (*e).deleted != 0 {
        hi_free(e as *mut libc::c_void);
    } else {
        (*e).in_tick = 0 as libc::c_int as libc::c_char;
    }
    return handled;
}
unsafe extern "C" fn redisPollAddRead(mut data: *mut libc::c_void) {
    let mut e: *mut redisPollEvents = data as *mut redisPollEvents;
    (*e).reading = 1 as libc::c_int as libc::c_char;
}
unsafe extern "C" fn redisPollDelRead(mut data: *mut libc::c_void) {
    let mut e: *mut redisPollEvents = data as *mut redisPollEvents;
    (*e).reading = 0 as libc::c_int as libc::c_char;
}
unsafe extern "C" fn redisPollAddWrite(mut data: *mut libc::c_void) {
    let mut e: *mut redisPollEvents = data as *mut redisPollEvents;
    (*e).writing = 1 as libc::c_int as libc::c_char;
}
unsafe extern "C" fn redisPollDelWrite(mut data: *mut libc::c_void) {
    let mut e: *mut redisPollEvents = data as *mut redisPollEvents;
    (*e).writing = 0 as libc::c_int as libc::c_char;
}
unsafe extern "C" fn redisPollCleanup(mut data: *mut libc::c_void) {
    let mut e: *mut redisPollEvents = data as *mut redisPollEvents;
    if (*e).in_tick != 0 {
        (*e).deleted = 1 as libc::c_int as libc::c_char;
    } else {
        hi_free(e as *mut libc::c_void);
    };
}
unsafe extern "C" fn redisPollScheduleTimer(
    mut data: *mut libc::c_void,
    mut tv: timeval,
) {
    let mut e: *mut redisPollEvents = data as *mut redisPollEvents;
    let mut now: libc::c_double = redisPollGetNow();
    (*e).deadline = now + redisPollTimevalToDouble(&mut tv);
}
unsafe extern "C" fn redisPollAttach(mut ac: *mut redisAsyncContext) -> libc::c_int {
    let mut c: *mut redisContext = &mut (*ac).c;
    let mut e: *mut redisPollEvents = 0 as *mut redisPollEvents;
    if !((*ac).ev.data).is_null() {
        return -(1 as libc::c_int);
    }
    e = hi_malloc(::std::mem::size_of::<redisPollEvents>() as libc::c_ulong)
        as *mut redisPollEvents;
    if e.is_null() {
        return -(1 as libc::c_int);
    }
    memset(
        e as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<redisPollEvents>() as libc::c_ulong,
    );
    (*e).context = ac;
    (*e).fd = (*c).fd;
    (*e).writing = 0 as libc::c_int as libc::c_char;
    (*e).reading = (*e).writing;
    (*e).deleted = 0 as libc::c_int as libc::c_char;
    (*e).in_tick = (*e).deleted;
    (*e).deadline = 0.0f64;
    (*ac)
        .ev
        .addRead = Some(
        redisPollAddRead as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    (*ac)
        .ev
        .delRead = Some(
        redisPollDelRead as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    (*ac)
        .ev
        .addWrite = Some(
        redisPollAddWrite as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    (*ac)
        .ev
        .delWrite = Some(
        redisPollDelWrite as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    (*ac)
        .ev
        .scheduleTimer = Some(
        redisPollScheduleTimer as unsafe extern "C" fn(*mut libc::c_void, timeval) -> (),
    );
    (*ac)
        .ev
        .cleanup = Some(
        redisPollCleanup as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    (*ac).ev.data = e as *mut libc::c_void;
    return 0 as libc::c_int;
}
static mut insecure_calloc_calls: libc::c_int = 0;
static mut tests: libc::c_int = 0 as libc::c_int;
static mut fails: libc::c_int = 0 as libc::c_int;
static mut skips: libc::c_int = 0 as libc::c_int;
unsafe extern "C" fn millisleep(mut ms: libc::c_int) {
    usleep((ms * 1000 as libc::c_int) as __useconds_t);
}
unsafe extern "C" fn usec() -> libc::c_longlong {
    let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    gettimeofday(&mut tv, 0 as *mut libc::c_void);
    return tv.tv_sec as libc::c_longlong * 1000000 as libc::c_int as libc::c_longlong
        + tv.tv_usec as libc::c_longlong;
}
pub unsafe extern "C" fn get_redis_version(
    mut c: *mut redisContext,
    mut majorptr: *mut libc::c_int,
    mut minorptr: *mut libc::c_int,
) {
    let mut reply: *mut redisReply = 0 as *mut redisReply;
    let mut eptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut e: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut major: libc::c_int = 0;
    let mut minor: libc::c_int = 0;
    reply = redisCommand(c, b"INFO\0" as *const u8 as *const libc::c_char)
        as *mut redisReply;
    if !(reply.is_null() || (*c).err != 0 || (*reply).type_0 != 1 as libc::c_int) {
        s = strstr(
            (*reply).str_0,
            b"redis_version:\0" as *const u8 as *const libc::c_char,
        );
        if !s.is_null() {
            s = s
                .offset(
                    strlen(b"redis_version:\0" as *const u8 as *const libc::c_char)
                        as isize,
                );
            e = strstr(s, b"\r\n\0" as *const u8 as *const libc::c_char);
            if !(e.is_null()
                || (e.offset_from(s) as libc::c_long) < 5 as libc::c_int as libc::c_long)
            {
                major = strtol(s, &mut eptr, 10 as libc::c_int) as libc::c_int;
                if !(*eptr as libc::c_int != '.' as i32) {
                    minor = strtol(
                        eptr.offset(1 as libc::c_int as isize),
                        0 as *mut *mut libc::c_char,
                        10 as libc::c_int,
                    ) as libc::c_int;
                    if !majorptr.is_null() {
                        *majorptr = major;
                    }
                    if !minorptr.is_null() {
                        *minorptr = minor;
                    }
                    freeReplyObject(reply as *mut libc::c_void);
                    return;
                }
            }
        }
    }
    freeReplyObject(reply as *mut libc::c_void);
    fprintf(
        stderr,
        b"Error:  Cannot determine Redis version, aborting\n\0" as *const u8
            as *const libc::c_char,
    );
    exit(1 as libc::c_int);
}
unsafe extern "C" fn select_database(mut c: *mut redisContext) -> *mut redisContext {
    let mut reply: *mut redisReply = 0 as *mut redisReply;
    reply = redisCommand(c, b"SELECT 9\0" as *const u8 as *const libc::c_char)
        as *mut redisReply;
    if !reply.is_null() {} else {
        __assert_fail(
            b"reply != NULL\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            156 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 46],
                &[libc::c_char; 46],
            >(b"redisContext *select_database(redisContext *)\0"))
                .as_ptr(),
        );
    };
    freeReplyObject(reply as *mut libc::c_void);
    reply = redisCommand(c, b"DBSIZE\0" as *const u8 as *const libc::c_char)
        as *mut redisReply;
    if !reply.is_null() {} else {
        __assert_fail(
            b"reply != NULL\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            161 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 46],
                &[libc::c_char; 46],
            >(b"redisContext *select_database(redisContext *)\0"))
                .as_ptr(),
        );
    };
    if (*reply).type_0 == 3 as libc::c_int
        && (*reply).integer == 0 as libc::c_int as libc::c_longlong
    {
        freeReplyObject(reply as *mut libc::c_void);
    } else {
        printf(
            b"Database #9 is not empty, test can not continue\n\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    return c;
}
unsafe extern "C" fn send_hello(mut c: *mut redisContext, mut version: libc::c_int) {
    let mut reply: *mut redisReply = 0 as *mut redisReply;
    let mut expected: libc::c_int = 0;
    reply = redisCommand(c, b"HELLO %d\0" as *const u8 as *const libc::c_char, version)
        as *mut redisReply;
    expected = if version == 3 as libc::c_int {
        9 as libc::c_int
    } else {
        2 as libc::c_int
    };
    if !reply.is_null() && (*reply).type_0 == expected {} else {
        __assert_fail(
            b"reply != NULL && reply->type == expected\0" as *const u8
                as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            180 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 37],
                &[libc::c_char; 37],
            >(b"void send_hello(redisContext *, int)\0"))
                .as_ptr(),
        );
    };
    freeReplyObject(reply as *mut libc::c_void);
}
unsafe extern "C" fn send_client_tracking(
    mut c: *mut redisContext,
    mut str: *const libc::c_char,
) {
    let mut reply: *mut redisReply = 0 as *mut redisReply;
    reply = redisCommand(
        c,
        b"CLIENT TRACKING %s\0" as *const u8 as *const libc::c_char,
        str,
    ) as *mut redisReply;
    if !reply.is_null() && (*reply).type_0 == 5 as libc::c_int {} else {
        __assert_fail(
            b"reply != NULL && reply->type == REDIS_REPLY_STATUS\0" as *const u8
                as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            189 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 56],
                &[libc::c_char; 56],
            >(b"void send_client_tracking(redisContext *, const char *)\0"))
                .as_ptr(),
        );
    };
    freeReplyObject(reply as *mut libc::c_void);
}
unsafe extern "C" fn disconnect(
    mut c: *mut redisContext,
    mut keep_fd: libc::c_int,
) -> libc::c_int {
    let mut reply: *mut redisReply = 0 as *mut redisReply;
    reply = redisCommand(c, b"SELECT 9\0" as *const u8 as *const libc::c_char)
        as *mut redisReply;
    if !reply.is_null() {} else {
        __assert_fail(
            b"reply != NULL\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            198 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 36],
                &[libc::c_char; 36],
            >(b"int disconnect(redisContext *, int)\0"))
                .as_ptr(),
        );
    };
    freeReplyObject(reply as *mut libc::c_void);
    reply = redisCommand(c, b"FLUSHDB\0" as *const u8 as *const libc::c_char)
        as *mut redisReply;
    if !reply.is_null() {} else {
        __assert_fail(
            b"reply != NULL\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            201 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 36],
                &[libc::c_char; 36],
            >(b"int disconnect(redisContext *, int)\0"))
                .as_ptr(),
        );
    };
    freeReplyObject(reply as *mut libc::c_void);
    if keep_fd != 0 {
        return redisFreeKeepFd(c);
    }
    redisFree(c);
    return -(1 as libc::c_int);
}
unsafe extern "C" fn do_ssl_handshake(mut c: *mut redisContext) {}
unsafe extern "C" fn do_connect(mut config: config) -> *mut redisContext {
    let mut c: *mut redisContext = 0 as *mut redisContext;
    if config.type_0 as libc::c_uint == CONN_TCP as libc::c_int as libc::c_uint {
        c = redisConnect(config.tcp.host, config.tcp.port);
    } else if config.type_0 as libc::c_uint == CONN_SSL as libc::c_int as libc::c_uint {
        c = redisConnect(config.ssl.host, config.ssl.port);
    } else if config.type_0 as libc::c_uint == CONN_UNIX as libc::c_int as libc::c_uint {
        c = redisConnectUnix(config.unix_sock.path);
    } else if config.type_0 as libc::c_uint == CONN_FD as libc::c_int as libc::c_uint {
        let mut dummy_ctx: *mut redisContext = redisConnectUnix(config.unix_sock.path);
        if !dummy_ctx.is_null() {
            let mut fd: libc::c_int = disconnect(dummy_ctx, 1 as libc::c_int);
            printf(
                b"Connecting to inherited fd %d\n\0" as *const u8 as *const libc::c_char,
                fd,
            );
            c = redisConnectFd(fd);
        }
    } else {
        fprintf(
            stderr,
            b"PANIC: %s (In function \"%s\", file \"%s\", line %d)\n\0" as *const u8
                as *const libc::c_char,
            b"Unknown connection type!\0" as *const u8 as *const libc::c_char,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"do_connect\0"))
                .as_ptr(),
            b"test.c\0" as *const u8 as *const libc::c_char,
            242 as libc::c_int,
        );
        exit(1 as libc::c_int);
    }
    if c.is_null() {
        printf(
            b"Connection error: can't allocate redis context\n\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    } else if (*c).err != 0 {
        printf(
            b"Connection error: %s\n\0" as *const u8 as *const libc::c_char,
            ((*c).errstr).as_mut_ptr(),
        );
        redisFree(c);
        exit(1 as libc::c_int);
    }
    if config.type_0 as libc::c_uint == CONN_SSL as libc::c_int as libc::c_uint {
        do_ssl_handshake(c);
    }
    return select_database(c);
}
unsafe extern "C" fn do_reconnect(mut c: *mut redisContext, mut config: config) {
    redisReconnect(c);
    if config.type_0 as libc::c_uint == CONN_SSL as libc::c_int as libc::c_uint {
        do_ssl_handshake(c);
    }
}
unsafe extern "C" fn test_format_commands() {
    let mut cmd: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: libc::c_int = 0;
    tests += 1;
    printf(b"#%02d \0" as *const u8 as *const libc::c_char, tests);
    printf(
        b"Format command without interpolation: \0" as *const u8 as *const libc::c_char,
    );
    len = redisFormatCommand(
        &mut cmd as *mut *mut libc::c_char,
        b"SET foo bar\0" as *const u8 as *const libc::c_char,
    );
    if strncmp(
        cmd,
        b"*3\r\n$3\r\nSET\r\n$3\r\nfoo\r\n$3\r\nbar\r\n\0" as *const u8
            as *const libc::c_char,
        len as libc::c_ulong,
    ) == 0 as libc::c_int
        && len
            == 4 as libc::c_int + 4 as libc::c_int
                + (3 as libc::c_int + 2 as libc::c_int) + 4 as libc::c_int
                + (3 as libc::c_int + 2 as libc::c_int) + 4 as libc::c_int
                + (3 as libc::c_int + 2 as libc::c_int)
    {
        printf(b"\x1B[0;32mPASSED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(b"\x1B[0;31mFAILED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
        fails += 1;
        fails;
    }
    hi_free(cmd as *mut libc::c_void);
    tests += 1;
    printf(b"#%02d \0" as *const u8 as *const libc::c_char, tests);
    printf(
        b"Format command with %%s string interpolation: \0" as *const u8
            as *const libc::c_char,
    );
    len = redisFormatCommand(
        &mut cmd as *mut *mut libc::c_char,
        b"SET %s %s\0" as *const u8 as *const libc::c_char,
        b"foo\0" as *const u8 as *const libc::c_char,
        b"bar\0" as *const u8 as *const libc::c_char,
    );
    if strncmp(
        cmd,
        b"*3\r\n$3\r\nSET\r\n$3\r\nfoo\r\n$3\r\nbar\r\n\0" as *const u8
            as *const libc::c_char,
        len as libc::c_ulong,
    ) == 0 as libc::c_int
        && len
            == 4 as libc::c_int + 4 as libc::c_int
                + (3 as libc::c_int + 2 as libc::c_int) + 4 as libc::c_int
                + (3 as libc::c_int + 2 as libc::c_int) + 4 as libc::c_int
                + (3 as libc::c_int + 2 as libc::c_int)
    {
        printf(b"\x1B[0;32mPASSED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(b"\x1B[0;31mFAILED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
        fails += 1;
        fails;
    }
    hi_free(cmd as *mut libc::c_void);
    tests += 1;
    printf(b"#%02d \0" as *const u8 as *const libc::c_char, tests);
    printf(
        b"Format command with %%s and an empty string: \0" as *const u8
            as *const libc::c_char,
    );
    len = redisFormatCommand(
        &mut cmd as *mut *mut libc::c_char,
        b"SET %s %s\0" as *const u8 as *const libc::c_char,
        b"foo\0" as *const u8 as *const libc::c_char,
        b"\0" as *const u8 as *const libc::c_char,
    );
    if strncmp(
        cmd,
        b"*3\r\n$3\r\nSET\r\n$3\r\nfoo\r\n$0\r\n\r\n\0" as *const u8
            as *const libc::c_char,
        len as libc::c_ulong,
    ) == 0 as libc::c_int
        && len
            == 4 as libc::c_int + 4 as libc::c_int
                + (3 as libc::c_int + 2 as libc::c_int) + 4 as libc::c_int
                + (3 as libc::c_int + 2 as libc::c_int) + 4 as libc::c_int
                + (0 as libc::c_int + 2 as libc::c_int)
    {
        printf(b"\x1B[0;32mPASSED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(b"\x1B[0;31mFAILED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
        fails += 1;
        fails;
    }
    hi_free(cmd as *mut libc::c_void);
    tests += 1;
    printf(b"#%02d \0" as *const u8 as *const libc::c_char, tests);
    printf(
        b"Format command with an empty string in between proper interpolations: \0"
            as *const u8 as *const libc::c_char,
    );
    len = redisFormatCommand(
        &mut cmd as *mut *mut libc::c_char,
        b"SET %s %s\0" as *const u8 as *const libc::c_char,
        b"\0" as *const u8 as *const libc::c_char,
        b"foo\0" as *const u8 as *const libc::c_char,
    );
    if strncmp(
        cmd,
        b"*3\r\n$3\r\nSET\r\n$0\r\n\r\n$3\r\nfoo\r\n\0" as *const u8
            as *const libc::c_char,
        len as libc::c_ulong,
    ) == 0 as libc::c_int
        && len
            == 4 as libc::c_int + 4 as libc::c_int
                + (3 as libc::c_int + 2 as libc::c_int) + 4 as libc::c_int
                + (0 as libc::c_int + 2 as libc::c_int) + 4 as libc::c_int
                + (3 as libc::c_int + 2 as libc::c_int)
    {
        printf(b"\x1B[0;32mPASSED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(b"\x1B[0;31mFAILED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
        fails += 1;
        fails;
    }
    hi_free(cmd as *mut libc::c_void);
    tests += 1;
    printf(b"#%02d \0" as *const u8 as *const libc::c_char, tests);
    printf(
        b"Format command with %%b string interpolation: \0" as *const u8
            as *const libc::c_char,
    );
    len = redisFormatCommand(
        &mut cmd as *mut *mut libc::c_char,
        b"SET %b %b\0" as *const u8 as *const libc::c_char,
        b"foo\0" as *const u8 as *const libc::c_char,
        3 as libc::c_int as size_t,
        b"b\0r\0" as *const u8 as *const libc::c_char,
        3 as libc::c_int as size_t,
    );
    if strncmp(
        cmd,
        b"*3\r\n$3\r\nSET\r\n$3\r\nfoo\r\n$3\r\nb\0r\r\n\0" as *const u8
            as *const libc::c_char,
        len as libc::c_ulong,
    ) == 0 as libc::c_int
        && len
            == 4 as libc::c_int + 4 as libc::c_int
                + (3 as libc::c_int + 2 as libc::c_int) + 4 as libc::c_int
                + (3 as libc::c_int + 2 as libc::c_int) + 4 as libc::c_int
                + (3 as libc::c_int + 2 as libc::c_int)
    {
        printf(b"\x1B[0;32mPASSED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(b"\x1B[0;31mFAILED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
        fails += 1;
        fails;
    }
    hi_free(cmd as *mut libc::c_void);
    tests += 1;
    printf(b"#%02d \0" as *const u8 as *const libc::c_char, tests);
    printf(
        b"Format command with %%b and an empty string: \0" as *const u8
            as *const libc::c_char,
    );
    len = redisFormatCommand(
        &mut cmd as *mut *mut libc::c_char,
        b"SET %b %b\0" as *const u8 as *const libc::c_char,
        b"foo\0" as *const u8 as *const libc::c_char,
        3 as libc::c_int as size_t,
        b"\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int as size_t,
    );
    if strncmp(
        cmd,
        b"*3\r\n$3\r\nSET\r\n$3\r\nfoo\r\n$0\r\n\r\n\0" as *const u8
            as *const libc::c_char,
        len as libc::c_ulong,
    ) == 0 as libc::c_int
        && len
            == 4 as libc::c_int + 4 as libc::c_int
                + (3 as libc::c_int + 2 as libc::c_int) + 4 as libc::c_int
                + (3 as libc::c_int + 2 as libc::c_int) + 4 as libc::c_int
                + (0 as libc::c_int + 2 as libc::c_int)
    {
        printf(b"\x1B[0;32mPASSED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(b"\x1B[0;31mFAILED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
        fails += 1;
        fails;
    }
    hi_free(cmd as *mut libc::c_void);
    tests += 1;
    printf(b"#%02d \0" as *const u8 as *const libc::c_char, tests);
    printf(b"Format command with literal %%: \0" as *const u8 as *const libc::c_char);
    len = redisFormatCommand(
        &mut cmd as *mut *mut libc::c_char,
        b"SET %% %%\0" as *const u8 as *const libc::c_char,
    );
    if strncmp(
        cmd,
        b"*3\r\n$3\r\nSET\r\n$1\r\n%\r\n$1\r\n%\r\n\0" as *const u8
            as *const libc::c_char,
        len as libc::c_ulong,
    ) == 0 as libc::c_int
        && len
            == 4 as libc::c_int + 4 as libc::c_int
                + (3 as libc::c_int + 2 as libc::c_int) + 4 as libc::c_int
                + (1 as libc::c_int + 2 as libc::c_int) + 4 as libc::c_int
                + (1 as libc::c_int + 2 as libc::c_int)
    {
        printf(b"\x1B[0;32mPASSED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(b"\x1B[0;31mFAILED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
        fails += 1;
        fails;
    }
    hi_free(cmd as *mut libc::c_void);
    let mut value: libc::c_int = 123 as libc::c_int;
    tests += 1;
    printf(b"#%02d \0" as *const u8 as *const libc::c_char, tests);
    printf(
        b"Format command with printf-delegation (int): \0" as *const u8
            as *const libc::c_char,
    );
    len = redisFormatCommand(
        &mut cmd as *mut *mut libc::c_char,
        b"key:%08d str:%s\0" as *const u8 as *const libc::c_char,
        value,
        b"hello\0" as *const u8 as *const libc::c_char,
    );
    if strncmp(
        cmd,
        b"*2\r\n$12\r\nkey:00000123\r\n$9\r\nstr:hello\r\n\0" as *const u8
            as *const libc::c_char,
        len as libc::c_ulong,
    ) == 0 as libc::c_int
        && len
            == 4 as libc::c_int + 5 as libc::c_int
                + (12 as libc::c_int + 2 as libc::c_int) + 4 as libc::c_int
                + (9 as libc::c_int + 2 as libc::c_int)
    {
        printf(b"\x1B[0;32mPASSED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(b"\x1B[0;31mFAILED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
        fails += 1;
        fails;
    }
    hi_free(cmd as *mut libc::c_void);
    let mut value_0: libc::c_char = 123 as libc::c_int as libc::c_char;
    tests += 1;
    printf(b"#%02d \0" as *const u8 as *const libc::c_char, tests);
    printf(
        b"Format command with printf-delegation (char): \0" as *const u8
            as *const libc::c_char,
    );
    len = redisFormatCommand(
        &mut cmd as *mut *mut libc::c_char,
        b"key:%08hhd str:%s\0" as *const u8 as *const libc::c_char,
        value_0 as libc::c_int,
        b"hello\0" as *const u8 as *const libc::c_char,
    );
    if strncmp(
        cmd,
        b"*2\r\n$12\r\nkey:00000123\r\n$9\r\nstr:hello\r\n\0" as *const u8
            as *const libc::c_char,
        len as libc::c_ulong,
    ) == 0 as libc::c_int
        && len
            == 4 as libc::c_int + 5 as libc::c_int
                + (12 as libc::c_int + 2 as libc::c_int) + 4 as libc::c_int
                + (9 as libc::c_int + 2 as libc::c_int)
    {
        printf(b"\x1B[0;32mPASSED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(b"\x1B[0;31mFAILED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
        fails += 1;
        fails;
    }
    hi_free(cmd as *mut libc::c_void);
    let mut value_1: libc::c_short = 123 as libc::c_int as libc::c_short;
    tests += 1;
    printf(b"#%02d \0" as *const u8 as *const libc::c_char, tests);
    printf(
        b"Format command with printf-delegation (short): \0" as *const u8
            as *const libc::c_char,
    );
    len = redisFormatCommand(
        &mut cmd as *mut *mut libc::c_char,
        b"key:%08hd str:%s\0" as *const u8 as *const libc::c_char,
        value_1 as libc::c_int,
        b"hello\0" as *const u8 as *const libc::c_char,
    );
    if strncmp(
        cmd,
        b"*2\r\n$12\r\nkey:00000123\r\n$9\r\nstr:hello\r\n\0" as *const u8
            as *const libc::c_char,
        len as libc::c_ulong,
    ) == 0 as libc::c_int
        && len
            == 4 as libc::c_int + 5 as libc::c_int
                + (12 as libc::c_int + 2 as libc::c_int) + 4 as libc::c_int
                + (9 as libc::c_int + 2 as libc::c_int)
    {
        printf(b"\x1B[0;32mPASSED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(b"\x1B[0;31mFAILED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
        fails += 1;
        fails;
    }
    hi_free(cmd as *mut libc::c_void);
    let mut value_2: libc::c_long = 123 as libc::c_int as libc::c_long;
    tests += 1;
    printf(b"#%02d \0" as *const u8 as *const libc::c_char, tests);
    printf(
        b"Format command with printf-delegation (long): \0" as *const u8
            as *const libc::c_char,
    );
    len = redisFormatCommand(
        &mut cmd as *mut *mut libc::c_char,
        b"key:%08ld str:%s\0" as *const u8 as *const libc::c_char,
        value_2,
        b"hello\0" as *const u8 as *const libc::c_char,
    );
    if strncmp(
        cmd,
        b"*2\r\n$12\r\nkey:00000123\r\n$9\r\nstr:hello\r\n\0" as *const u8
            as *const libc::c_char,
        len as libc::c_ulong,
    ) == 0 as libc::c_int
        && len
            == 4 as libc::c_int + 5 as libc::c_int
                + (12 as libc::c_int + 2 as libc::c_int) + 4 as libc::c_int
                + (9 as libc::c_int + 2 as libc::c_int)
    {
        printf(b"\x1B[0;32mPASSED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(b"\x1B[0;31mFAILED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
        fails += 1;
        fails;
    }
    hi_free(cmd as *mut libc::c_void);
    let mut value_3: libc::c_longlong = 123 as libc::c_int as libc::c_longlong;
    tests += 1;
    printf(b"#%02d \0" as *const u8 as *const libc::c_char, tests);
    printf(
        b"Format command with printf-delegation (long long): \0" as *const u8
            as *const libc::c_char,
    );
    len = redisFormatCommand(
        &mut cmd as *mut *mut libc::c_char,
        b"key:%08lld str:%s\0" as *const u8 as *const libc::c_char,
        value_3,
        b"hello\0" as *const u8 as *const libc::c_char,
    );
    if strncmp(
        cmd,
        b"*2\r\n$12\r\nkey:00000123\r\n$9\r\nstr:hello\r\n\0" as *const u8
            as *const libc::c_char,
        len as libc::c_ulong,
    ) == 0 as libc::c_int
        && len
            == 4 as libc::c_int + 5 as libc::c_int
                + (12 as libc::c_int + 2 as libc::c_int) + 4 as libc::c_int
                + (9 as libc::c_int + 2 as libc::c_int)
    {
        printf(b"\x1B[0;32mPASSED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(b"\x1B[0;31mFAILED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
        fails += 1;
        fails;
    }
    hi_free(cmd as *mut libc::c_void);
    let mut value_4: libc::c_uint = 123 as libc::c_int as libc::c_uint;
    tests += 1;
    printf(b"#%02d \0" as *const u8 as *const libc::c_char, tests);
    printf(
        b"Format command with printf-delegation (unsigned int): \0" as *const u8
            as *const libc::c_char,
    );
    len = redisFormatCommand(
        &mut cmd as *mut *mut libc::c_char,
        b"key:%08u str:%s\0" as *const u8 as *const libc::c_char,
        value_4,
        b"hello\0" as *const u8 as *const libc::c_char,
    );
    if strncmp(
        cmd,
        b"*2\r\n$12\r\nkey:00000123\r\n$9\r\nstr:hello\r\n\0" as *const u8
            as *const libc::c_char,
        len as libc::c_ulong,
    ) == 0 as libc::c_int
        && len
            == 4 as libc::c_int + 5 as libc::c_int
                + (12 as libc::c_int + 2 as libc::c_int) + 4 as libc::c_int
                + (9 as libc::c_int + 2 as libc::c_int)
    {
        printf(b"\x1B[0;32mPASSED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(b"\x1B[0;31mFAILED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
        fails += 1;
        fails;
    }
    hi_free(cmd as *mut libc::c_void);
    let mut value_5: libc::c_uchar = 123 as libc::c_int as libc::c_uchar;
    tests += 1;
    printf(b"#%02d \0" as *const u8 as *const libc::c_char, tests);
    printf(
        b"Format command with printf-delegation (unsigned char): \0" as *const u8
            as *const libc::c_char,
    );
    len = redisFormatCommand(
        &mut cmd as *mut *mut libc::c_char,
        b"key:%08hhu str:%s\0" as *const u8 as *const libc::c_char,
        value_5 as libc::c_int,
        b"hello\0" as *const u8 as *const libc::c_char,
    );
    if strncmp(
        cmd,
        b"*2\r\n$12\r\nkey:00000123\r\n$9\r\nstr:hello\r\n\0" as *const u8
            as *const libc::c_char,
        len as libc::c_ulong,
    ) == 0 as libc::c_int
        && len
            == 4 as libc::c_int + 5 as libc::c_int
                + (12 as libc::c_int + 2 as libc::c_int) + 4 as libc::c_int
                + (9 as libc::c_int + 2 as libc::c_int)
    {
        printf(b"\x1B[0;32mPASSED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(b"\x1B[0;31mFAILED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
        fails += 1;
        fails;
    }
    hi_free(cmd as *mut libc::c_void);
    let mut value_6: libc::c_ushort = 123 as libc::c_int as libc::c_ushort;
    tests += 1;
    printf(b"#%02d \0" as *const u8 as *const libc::c_char, tests);
    printf(
        b"Format command with printf-delegation (unsigned short): \0" as *const u8
            as *const libc::c_char,
    );
    len = redisFormatCommand(
        &mut cmd as *mut *mut libc::c_char,
        b"key:%08hu str:%s\0" as *const u8 as *const libc::c_char,
        value_6 as libc::c_int,
        b"hello\0" as *const u8 as *const libc::c_char,
    );
    if strncmp(
        cmd,
        b"*2\r\n$12\r\nkey:00000123\r\n$9\r\nstr:hello\r\n\0" as *const u8
            as *const libc::c_char,
        len as libc::c_ulong,
    ) == 0 as libc::c_int
        && len
            == 4 as libc::c_int + 5 as libc::c_int
                + (12 as libc::c_int + 2 as libc::c_int) + 4 as libc::c_int
                + (9 as libc::c_int + 2 as libc::c_int)
    {
        printf(b"\x1B[0;32mPASSED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(b"\x1B[0;31mFAILED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
        fails += 1;
        fails;
    }
    hi_free(cmd as *mut libc::c_void);
    let mut value_7: libc::c_ulong = 123 as libc::c_int as libc::c_ulong;
    tests += 1;
    printf(b"#%02d \0" as *const u8 as *const libc::c_char, tests);
    printf(
        b"Format command with printf-delegation (unsigned long): \0" as *const u8
            as *const libc::c_char,
    );
    len = redisFormatCommand(
        &mut cmd as *mut *mut libc::c_char,
        b"key:%08lu str:%s\0" as *const u8 as *const libc::c_char,
        value_7,
        b"hello\0" as *const u8 as *const libc::c_char,
    );
    if strncmp(
        cmd,
        b"*2\r\n$12\r\nkey:00000123\r\n$9\r\nstr:hello\r\n\0" as *const u8
            as *const libc::c_char,
        len as libc::c_ulong,
    ) == 0 as libc::c_int
        && len
            == 4 as libc::c_int + 5 as libc::c_int
                + (12 as libc::c_int + 2 as libc::c_int) + 4 as libc::c_int
                + (9 as libc::c_int + 2 as libc::c_int)
    {
        printf(b"\x1B[0;32mPASSED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(b"\x1B[0;31mFAILED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
        fails += 1;
        fails;
    }
    hi_free(cmd as *mut libc::c_void);
    let mut value_8: libc::c_ulonglong = 123 as libc::c_int as libc::c_ulonglong;
    tests += 1;
    printf(b"#%02d \0" as *const u8 as *const libc::c_char, tests);
    printf(
        b"Format command with printf-delegation (unsigned long long): \0" as *const u8
            as *const libc::c_char,
    );
    len = redisFormatCommand(
        &mut cmd as *mut *mut libc::c_char,
        b"key:%08llu str:%s\0" as *const u8 as *const libc::c_char,
        value_8,
        b"hello\0" as *const u8 as *const libc::c_char,
    );
    if strncmp(
        cmd,
        b"*2\r\n$12\r\nkey:00000123\r\n$9\r\nstr:hello\r\n\0" as *const u8
            as *const libc::c_char,
        len as libc::c_ulong,
    ) == 0 as libc::c_int
        && len
            == 4 as libc::c_int + 5 as libc::c_int
                + (12 as libc::c_int + 2 as libc::c_int) + 4 as libc::c_int
                + (9 as libc::c_int + 2 as libc::c_int)
    {
        printf(b"\x1B[0;32mPASSED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(b"\x1B[0;31mFAILED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
        fails += 1;
        fails;
    }
    hi_free(cmd as *mut libc::c_void);
    let mut value_9: libc::c_float = 123.0f64 as libc::c_float;
    tests += 1;
    printf(b"#%02d \0" as *const u8 as *const libc::c_char, tests);
    printf(
        b"Format command with printf-delegation (float): \0" as *const u8
            as *const libc::c_char,
    );
    len = redisFormatCommand(
        &mut cmd as *mut *mut libc::c_char,
        b"key:%08.3f str:%s\0" as *const u8 as *const libc::c_char,
        value_9 as libc::c_double,
        b"hello\0" as *const u8 as *const libc::c_char,
    );
    if strncmp(
        cmd,
        b"*2\r\n$12\r\nkey:0123.000\r\n$9\r\nstr:hello\r\n\0" as *const u8
            as *const libc::c_char,
        len as libc::c_ulong,
    ) == 0 as libc::c_int
        && len
            == 4 as libc::c_int + 5 as libc::c_int
                + (12 as libc::c_int + 2 as libc::c_int) + 4 as libc::c_int
                + (9 as libc::c_int + 2 as libc::c_int)
    {
        printf(b"\x1B[0;32mPASSED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(b"\x1B[0;31mFAILED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
        fails += 1;
        fails;
    }
    hi_free(cmd as *mut libc::c_void);
    let mut value_10: libc::c_double = 123.0f64;
    tests += 1;
    printf(b"#%02d \0" as *const u8 as *const libc::c_char, tests);
    printf(
        b"Format command with printf-delegation (double): \0" as *const u8
            as *const libc::c_char,
    );
    len = redisFormatCommand(
        &mut cmd as *mut *mut libc::c_char,
        b"key:%08.3f str:%s\0" as *const u8 as *const libc::c_char,
        value_10,
        b"hello\0" as *const u8 as *const libc::c_char,
    );
    if strncmp(
        cmd,
        b"*2\r\n$12\r\nkey:0123.000\r\n$9\r\nstr:hello\r\n\0" as *const u8
            as *const libc::c_char,
        len as libc::c_ulong,
    ) == 0 as libc::c_int
        && len
            == 4 as libc::c_int + 5 as libc::c_int
                + (12 as libc::c_int + 2 as libc::c_int) + 4 as libc::c_int
                + (9 as libc::c_int + 2 as libc::c_int)
    {
        printf(b"\x1B[0;32mPASSED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(b"\x1B[0;31mFAILED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
        fails += 1;
        fails;
    }
    hi_free(cmd as *mut libc::c_void);
    tests += 1;
    printf(b"#%02d \0" as *const u8 as *const libc::c_char, tests);
    printf(
        b"Format command with unhandled printf format (specifier 'p' not supported): \0"
            as *const u8 as *const libc::c_char,
    );
    len = redisFormatCommand(
        &mut cmd as *mut *mut libc::c_char,
        b"key:%08p %b\0" as *const u8 as *const libc::c_char,
        1234 as libc::c_int as *mut libc::c_void,
        b"foo\0" as *const u8 as *const libc::c_char,
        3 as libc::c_int as size_t,
    );
    if len == -(1 as libc::c_int) {
        printf(b"\x1B[0;32mPASSED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(b"\x1B[0;31mFAILED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
        fails += 1;
        fails;
    }
    tests += 1;
    printf(b"#%02d \0" as *const u8 as *const libc::c_char, tests);
    printf(
        b"Format command with invalid printf format (specifier missing): \0" as *const u8
            as *const libc::c_char,
    );
    len = redisFormatCommand(
        &mut cmd as *mut *mut libc::c_char,
        b"%-\0" as *const u8 as *const libc::c_char,
    );
    if len == -(1 as libc::c_int) {
        printf(b"\x1B[0;32mPASSED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(b"\x1B[0;31mFAILED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
        fails += 1;
        fails;
    }
    let mut argv: [*const libc::c_char; 3] = [0 as *const libc::c_char; 3];
    argv[0 as libc::c_int as usize] = b"SET\0" as *const u8 as *const libc::c_char;
    argv[1 as libc::c_int as usize] = b"foo\0xxx\0" as *const u8 as *const libc::c_char;
    argv[2 as libc::c_int as usize] = b"bar\0" as *const u8 as *const libc::c_char;
    let mut lens: [size_t; 3] = [
        3 as libc::c_int as size_t,
        7 as libc::c_int as size_t,
        3 as libc::c_int as size_t,
    ];
    let mut argc: libc::c_int = 3 as libc::c_int;
    tests += 1;
    printf(b"#%02d \0" as *const u8 as *const libc::c_char, tests);
    printf(
        b"Format command by passing argc/argv without lengths: \0" as *const u8
            as *const libc::c_char,
    );
    len = redisFormatCommandArgv(&mut cmd, argc, argv.as_mut_ptr(), 0 as *const size_t)
        as libc::c_int;
    if strncmp(
        cmd,
        b"*3\r\n$3\r\nSET\r\n$3\r\nfoo\r\n$3\r\nbar\r\n\0" as *const u8
            as *const libc::c_char,
        len as libc::c_ulong,
    ) == 0 as libc::c_int
        && len
            == 4 as libc::c_int + 4 as libc::c_int
                + (3 as libc::c_int + 2 as libc::c_int) + 4 as libc::c_int
                + (3 as libc::c_int + 2 as libc::c_int) + 4 as libc::c_int
                + (3 as libc::c_int + 2 as libc::c_int)
    {
        printf(b"\x1B[0;32mPASSED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(b"\x1B[0;31mFAILED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
        fails += 1;
        fails;
    }
    hi_free(cmd as *mut libc::c_void);
    tests += 1;
    printf(b"#%02d \0" as *const u8 as *const libc::c_char, tests);
    printf(
        b"Format command by passing argc/argv with lengths: \0" as *const u8
            as *const libc::c_char,
    );
    len = redisFormatCommandArgv(&mut cmd, argc, argv.as_mut_ptr(), lens.as_mut_ptr())
        as libc::c_int;
    if strncmp(
        cmd,
        b"*3\r\n$3\r\nSET\r\n$7\r\nfoo\0xxx\r\n$3\r\nbar\r\n\0" as *const u8
            as *const libc::c_char,
        len as libc::c_ulong,
    ) == 0 as libc::c_int
        && len
            == 4 as libc::c_int + 4 as libc::c_int
                + (3 as libc::c_int + 2 as libc::c_int) + 4 as libc::c_int
                + (7 as libc::c_int + 2 as libc::c_int) + 4 as libc::c_int
                + (3 as libc::c_int + 2 as libc::c_int)
    {
        printf(b"\x1B[0;32mPASSED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(b"\x1B[0;31mFAILED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
        fails += 1;
        fails;
    }
    hi_free(cmd as *mut libc::c_void);
    let mut sds_cmd: sds = 0 as *mut libc::c_char;
    sds_cmd = 0 as sds;
    tests += 1;
    printf(b"#%02d \0" as *const u8 as *const libc::c_char, tests);
    printf(
        b"Format command into sds by passing argc/argv without lengths: \0" as *const u8
            as *const libc::c_char,
    );
    len = redisFormatSdsCommandArgv(
        &mut sds_cmd,
        argc,
        argv.as_mut_ptr(),
        0 as *const size_t,
    ) as libc::c_int;
    if strncmp(
        sds_cmd as *const libc::c_char,
        b"*3\r\n$3\r\nSET\r\n$3\r\nfoo\r\n$3\r\nbar\r\n\0" as *const u8
            as *const libc::c_char,
        len as libc::c_ulong,
    ) == 0 as libc::c_int
        && len
            == 4 as libc::c_int + 4 as libc::c_int
                + (3 as libc::c_int + 2 as libc::c_int) + 4 as libc::c_int
                + (3 as libc::c_int + 2 as libc::c_int) + 4 as libc::c_int
                + (3 as libc::c_int + 2 as libc::c_int)
    {
        printf(b"\x1B[0;32mPASSED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(b"\x1B[0;31mFAILED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
        fails += 1;
        fails;
    }
    sdsfree(sds_cmd);
    sds_cmd = 0 as sds;
    tests += 1;
    printf(b"#%02d \0" as *const u8 as *const libc::c_char, tests);
    printf(
        b"Format command into sds by passing argc/argv with lengths: \0" as *const u8
            as *const libc::c_char,
    );
    len = redisFormatSdsCommandArgv(
        &mut sds_cmd,
        argc,
        argv.as_mut_ptr(),
        lens.as_mut_ptr(),
    ) as libc::c_int;
    if strncmp(
        sds_cmd as *const libc::c_char,
        b"*3\r\n$3\r\nSET\r\n$7\r\nfoo\0xxx\r\n$3\r\nbar\r\n\0" as *const u8
            as *const libc::c_char,
        len as libc::c_ulong,
    ) == 0 as libc::c_int
        && len
            == 4 as libc::c_int + 4 as libc::c_int
                + (3 as libc::c_int + 2 as libc::c_int) + 4 as libc::c_int
                + (7 as libc::c_int + 2 as libc::c_int) + 4 as libc::c_int
                + (3 as libc::c_int + 2 as libc::c_int)
    {
        printf(b"\x1B[0;32mPASSED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(b"\x1B[0;31mFAILED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
        fails += 1;
        fails;
    }
    sdsfree(sds_cmd);
}
unsafe extern "C" fn test_append_formatted_commands(mut config: config) {
    let mut c: *mut redisContext = 0 as *mut redisContext;
    let mut reply: *mut redisReply = 0 as *mut redisReply;
    let mut cmd: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: libc::c_int = 0;
    c = do_connect(config);
    tests += 1;
    printf(b"#%02d \0" as *const u8 as *const libc::c_char, tests);
    printf(b"Append format command: \0" as *const u8 as *const libc::c_char);
    len = redisFormatCommand(
        &mut cmd as *mut *mut libc::c_char,
        b"SET foo bar\0" as *const u8 as *const libc::c_char,
    );
    if redisAppendFormattedCommand(c, cmd, len as size_t) == 0 as libc::c_int {
        printf(b"\x1B[0;32mPASSED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(b"\x1B[0;31mFAILED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
        fails += 1;
        fails;
    }
    if redisGetReply(
        c,
        &mut reply as *mut *mut redisReply as *mut libc::c_void as *mut *mut libc::c_void,
    ) == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"redisGetReply(c, (void*)&reply) == REDIS_OK\0" as *const u8
                as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            407 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 51],
                &[libc::c_char; 51],
            >(b"void test_append_formatted_commands(struct config)\0"))
                .as_ptr(),
        );
    };
    hi_free(cmd as *mut libc::c_void);
    freeReplyObject(reply as *mut libc::c_void);
    disconnect(c, 0 as libc::c_int);
}
unsafe extern "C" fn test_tcp_options(mut cfg: config) {
    let mut c: *mut redisContext = 0 as *mut redisContext;
    c = do_connect(cfg);
    tests += 1;
    printf(b"#%02d \0" as *const u8 as *const libc::c_char, tests);
    printf(b"We can enable TCP_KEEPALIVE: \0" as *const u8 as *const libc::c_char);
    if redisEnableKeepAlive(c) == 0 as libc::c_int {
        printf(b"\x1B[0;32mPASSED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(b"\x1B[0;31mFAILED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
        fails += 1;
        fails;
    }
    tests += 1;
    printf(b"#%02d \0" as *const u8 as *const libc::c_char, tests);
    printf(b"We can set TCP_USER_TIMEOUT: \0" as *const u8 as *const libc::c_char);
    if redisSetTcpUserTimeout(c, 100 as libc::c_int as libc::c_uint) == 0 as libc::c_int
    {
        printf(b"\x1B[0;32mPASSED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(b"\x1B[0;31mFAILED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
        fails += 1;
        fails;
    }
    redisFree(c);
}
unsafe extern "C" fn test_unix_keepalive(mut cfg: config) {
    let mut c: *mut redisContext = 0 as *mut redisContext;
    let mut r: *mut redisReply = 0 as *mut redisReply;
    c = do_connect(cfg);
    tests += 1;
    printf(b"#%02d \0" as *const u8 as *const libc::c_char, tests);
    printf(
        b"Setting TCP_KEEPALIVE on a unix socket returns an error: \0" as *const u8
            as *const libc::c_char,
    );
    if redisEnableKeepAlive(c) == -(1 as libc::c_int) && (*c).err == 0 as libc::c_int {
        printf(b"\x1B[0;32mPASSED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(b"\x1B[0;31mFAILED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
        fails += 1;
        fails;
    }
    tests += 1;
    printf(b"#%02d \0" as *const u8 as *const libc::c_char, tests);
    printf(
        b"Setting TCP_KEEPALIVE on a unix socket doesn't break the connection: \0"
            as *const u8 as *const libc::c_char,
    );
    r = redisCommand(c, b"PING\0" as *const u8 as *const libc::c_char)
        as *mut redisReply;
    if !r.is_null() && (*r).type_0 == 5 as libc::c_int
        && (*r).len == 4 as libc::c_int as libc::c_ulong
        && memcmp(
            (*r).str_0 as *const libc::c_void,
            b"PONG\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            4 as libc::c_int as libc::c_ulong,
        ) == 0
    {
        printf(b"\x1B[0;32mPASSED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(b"\x1B[0;31mFAILED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
        fails += 1;
        fails;
    }
    freeReplyObject(r as *mut libc::c_void);
    redisFree(c);
}
unsafe extern "C" fn test_reply_reader() {
    let mut reader: *mut redisReader = 0 as *mut redisReader;
    let mut reply: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut root: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut ret: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    tests += 1;
    printf(b"#%02d \0" as *const u8 as *const libc::c_char, tests);
    printf(b"Error handling in reply parser: \0" as *const u8 as *const libc::c_char);
    reader = redisReaderCreate();
    redisReaderFeed(
        reader,
        b"@foo\r\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        6 as libc::c_int as size_t,
    );
    ret = redisReaderGetReply(reader, 0 as *mut *mut libc::c_void);
    if ret == -(1 as libc::c_int)
        && strcasecmp(
            ((*reader).errstr).as_mut_ptr(),
            b"Protocol error, got \"@\" as reply type byte\0" as *const u8
                as *const libc::c_char,
        ) == 0 as libc::c_int
    {
        printf(b"\x1B[0;32mPASSED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(b"\x1B[0;31mFAILED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
        fails += 1;
        fails;
    }
    redisReaderFree(reader);
    tests += 1;
    printf(b"#%02d \0" as *const u8 as *const libc::c_char, tests);
    printf(b"Memory cleanup in reply parser: \0" as *const u8 as *const libc::c_char);
    reader = redisReaderCreate();
    redisReaderFeed(
        reader,
        b"*2\r\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        4 as libc::c_int as size_t,
    );
    redisReaderFeed(
        reader,
        b"$5\r\nhello\r\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        11 as libc::c_int as size_t,
    );
    redisReaderFeed(
        reader,
        b"@foo\r\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        6 as libc::c_int as size_t,
    );
    ret = redisReaderGetReply(reader, 0 as *mut *mut libc::c_void);
    if ret == -(1 as libc::c_int)
        && strcasecmp(
            ((*reader).errstr).as_mut_ptr(),
            b"Protocol error, got \"@\" as reply type byte\0" as *const u8
                as *const libc::c_char,
        ) == 0 as libc::c_int
    {
        printf(b"\x1B[0;32mPASSED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(b"\x1B[0;31mFAILED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
        fails += 1;
        fails;
    }
    redisReaderFree(reader);
    reader = redisReaderCreate();
    tests += 1;
    printf(b"#%02d \0" as *const u8 as *const libc::c_char, tests);
    printf(
        b"Can handle arbitrarily nested multi-bulks: \0" as *const u8
            as *const libc::c_char,
    );
    i = 0 as libc::c_int;
    while i < 128 as libc::c_int {
        redisReaderFeed(
            reader,
            b"*1\r\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            4 as libc::c_int as size_t,
        );
        i += 1;
        i;
    }
    redisReaderFeed(
        reader,
        b"$6\r\nLOLWUT\r\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        12 as libc::c_int as size_t,
    );
    ret = redisReaderGetReply(reader, &mut reply);
    root = reply;
    if ret == 0 as libc::c_int
        && (*(reply as *mut redisReply)).type_0 == 2 as libc::c_int
        && (*(reply as *mut redisReply)).elements == 1 as libc::c_int as libc::c_ulong
    {
        printf(b"\x1B[0;32mPASSED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(b"\x1B[0;31mFAILED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
        fails += 1;
        fails;
    }
    tests += 1;
    printf(b"#%02d \0" as *const u8 as *const libc::c_char, tests);
    printf(
        b"Can parse arbitrarily nested multi-bulks correctly: \0" as *const u8
            as *const libc::c_char,
    );
    loop {
        let fresh0 = i;
        i = i - 1;
        if !(fresh0 != 0) {
            break;
        }
        if !reply.is_null() && (*(reply as *mut redisReply)).type_0 == 2 as libc::c_int
        {} else {
            __assert_fail(
                b"reply != NULL && ((redisReply*)reply)->type == REDIS_REPLY_ARRAY\0"
                    as *const u8 as *const libc::c_char,
                b"test.c\0" as *const u8 as *const libc::c_char,
                492 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 29],
                    &[libc::c_char; 29],
                >(b"void test_reply_reader(void)\0"))
                    .as_ptr(),
            );
        };
        reply = *((*(reply as *mut redisReply)).element)
            .offset(0 as libc::c_int as isize) as *mut libc::c_void;
    }
    if (*(reply as *mut redisReply)).type_0 == 1 as libc::c_int
        && memcmp(
            (*(reply as *mut redisReply)).str_0 as *const libc::c_void,
            b"LOLWUT\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            6 as libc::c_int as libc::c_ulong,
        ) == 0
    {
        printf(b"\x1B[0;32mPASSED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(b"\x1B[0;31mFAILED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
        fails += 1;
        fails;
    }
    freeReplyObject(root);
    redisReaderFree(reader);
    tests += 1;
    printf(b"#%02d \0" as *const u8 as *const libc::c_char, tests);
    printf(b"Correctly parses LLONG_MAX: \0" as *const u8 as *const libc::c_char);
    reader = redisReaderCreate();
    redisReaderFeed(
        reader,
        b":9223372036854775807\r\n\0" as *const u8 as *const libc::c_char,
        22 as libc::c_int as size_t,
    );
    ret = redisReaderGetReply(reader, &mut reply);
    if ret == 0 as libc::c_int
        && (*(reply as *mut redisReply)).type_0 == 3 as libc::c_int
        && (*(reply as *mut redisReply)).integer
            == 9223372036854775807 as libc::c_longlong
    {
        printf(b"\x1B[0;32mPASSED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(b"\x1B[0;31mFAILED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
        fails += 1;
        fails;
    }
    freeReplyObject(reply);
    redisReaderFree(reader);
    tests += 1;
    printf(b"#%02d \0" as *const u8 as *const libc::c_char, tests);
    printf(b"Set error when > LLONG_MAX: \0" as *const u8 as *const libc::c_char);
    reader = redisReaderCreate();
    redisReaderFeed(
        reader,
        b":9223372036854775808\r\n\0" as *const u8 as *const libc::c_char,
        22 as libc::c_int as size_t,
    );
    ret = redisReaderGetReply(reader, &mut reply);
    if ret == -(1 as libc::c_int)
        && strcasecmp(
            ((*reader).errstr).as_mut_ptr(),
            b"Bad integer value\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
    {
        printf(b"\x1B[0;32mPASSED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(b"\x1B[0;31mFAILED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
        fails += 1;
        fails;
    }
    freeReplyObject(reply);
    redisReaderFree(reader);
    tests += 1;
    printf(b"#%02d \0" as *const u8 as *const libc::c_char, tests);
    printf(b"Correctly parses LLONG_MIN: \0" as *const u8 as *const libc::c_char);
    reader = redisReaderCreate();
    redisReaderFeed(
        reader,
        b":-9223372036854775808\r\n\0" as *const u8 as *const libc::c_char,
        23 as libc::c_int as size_t,
    );
    ret = redisReaderGetReply(reader, &mut reply);
    if ret == 0 as libc::c_int
        && (*(reply as *mut redisReply)).type_0 == 3 as libc::c_int
        && (*(reply as *mut redisReply)).integer
            == -(9223372036854775807 as libc::c_longlong) - 1 as libc::c_longlong
    {
        printf(b"\x1B[0;32mPASSED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(b"\x1B[0;31mFAILED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
        fails += 1;
        fails;
    }
    freeReplyObject(reply);
    redisReaderFree(reader);
    tests += 1;
    printf(b"#%02d \0" as *const u8 as *const libc::c_char, tests);
    printf(b"Set error when < LLONG_MIN: \0" as *const u8 as *const libc::c_char);
    reader = redisReaderCreate();
    redisReaderFeed(
        reader,
        b":-9223372036854775809\r\n\0" as *const u8 as *const libc::c_char,
        23 as libc::c_int as size_t,
    );
    ret = redisReaderGetReply(reader, &mut reply);
    if ret == -(1 as libc::c_int)
        && strcasecmp(
            ((*reader).errstr).as_mut_ptr(),
            b"Bad integer value\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
    {
        printf(b"\x1B[0;32mPASSED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(b"\x1B[0;31mFAILED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
        fails += 1;
        fails;
    }
    freeReplyObject(reply);
    redisReaderFree(reader);
    tests += 1;
    printf(b"#%02d \0" as *const u8 as *const libc::c_char, tests);
    printf(b"Set error when array < -1: \0" as *const u8 as *const libc::c_char);
    reader = redisReaderCreate();
    redisReaderFeed(
        reader,
        b"*-2\r\n+asdf\r\n\0" as *const u8 as *const libc::c_char,
        12 as libc::c_int as size_t,
    );
    ret = redisReaderGetReply(reader, &mut reply);
    if ret == -(1 as libc::c_int)
        && strcasecmp(
            ((*reader).errstr).as_mut_ptr(),
            b"Multi-bulk length out of range\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
    {
        printf(b"\x1B[0;32mPASSED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(b"\x1B[0;31mFAILED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
        fails += 1;
        fails;
    }
    freeReplyObject(reply);
    redisReaderFree(reader);
    tests += 1;
    printf(b"#%02d \0" as *const u8 as *const libc::c_char, tests);
    printf(b"Set error when bulk < -1: \0" as *const u8 as *const libc::c_char);
    reader = redisReaderCreate();
    redisReaderFeed(
        reader,
        b"$-2\r\nasdf\r\n\0" as *const u8 as *const libc::c_char,
        11 as libc::c_int as size_t,
    );
    ret = redisReaderGetReply(reader, &mut reply);
    if ret == -(1 as libc::c_int)
        && strcasecmp(
            ((*reader).errstr).as_mut_ptr(),
            b"Bulk string length out of range\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
    {
        printf(b"\x1B[0;32mPASSED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(b"\x1B[0;31mFAILED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
        fails += 1;
        fails;
    }
    freeReplyObject(reply);
    redisReaderFree(reader);
    tests += 1;
    printf(b"#%02d \0" as *const u8 as *const libc::c_char, tests);
    printf(
        b"Can configure maximum multi-bulk elements: \0" as *const u8
            as *const libc::c_char,
    );
    reader = redisReaderCreate();
    (*reader).maxelements = 1024 as libc::c_int as libc::c_longlong;
    redisReaderFeed(
        reader,
        b"*1025\r\n\0" as *const u8 as *const libc::c_char,
        7 as libc::c_int as size_t,
    );
    ret = redisReaderGetReply(reader, &mut reply);
    if ret == -(1 as libc::c_int)
        && strcasecmp(
            ((*reader).errstr).as_mut_ptr(),
            b"Multi-bulk length out of range\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
    {
        printf(b"\x1B[0;32mPASSED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(b"\x1B[0;31mFAILED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
        fails += 1;
        fails;
    }
    freeReplyObject(reply);
    redisReaderFree(reader);
    tests += 1;
    printf(b"#%02d \0" as *const u8 as *const libc::c_char, tests);
    printf(
        b"Multi-bulk never overflows regardless of maxelements: \0" as *const u8
            as *const libc::c_char,
    );
    let mut bad_mbulk_len: size_t = (18446744073709551615 as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
        .wrapping_add(3 as libc::c_int as libc::c_ulong);
    let mut bad_mbulk_reply: [libc::c_char; 100] = [0; 100];
    snprintf(
        bad_mbulk_reply.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 100]>() as libc::c_ulong,
        b"*%llu\r\n+asdf\r\n\0" as *const u8 as *const libc::c_char,
        bad_mbulk_len as libc::c_ulonglong,
    );
    reader = redisReaderCreate();
    (*reader).maxelements = 0 as libc::c_int as libc::c_longlong;
    redisReaderFeed(
        reader,
        bad_mbulk_reply.as_mut_ptr(),
        strlen(bad_mbulk_reply.as_mut_ptr()),
    );
    ret = redisReaderGetReply(reader, &mut reply);
    if ret == -(1 as libc::c_int)
        && strcasecmp(
            ((*reader).errstr).as_mut_ptr(),
            b"Out of memory\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
    {
        printf(b"\x1B[0;32mPASSED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(b"\x1B[0;31mFAILED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
        fails += 1;
        fails;
    }
    freeReplyObject(reply);
    redisReaderFree(reader);
    tests += 1;
    printf(b"#%02d \0" as *const u8 as *const libc::c_char, tests);
    printf(
        b"Works with NULL functions for reply: \0" as *const u8 as *const libc::c_char,
    );
    reader = redisReaderCreate();
    (*reader).fn_0 = 0 as *mut redisReplyObjectFunctions;
    redisReaderFeed(
        reader,
        b"+OK\r\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        5 as libc::c_int as size_t,
    );
    ret = redisReaderGetReply(reader, &mut reply);
    if ret == 0 as libc::c_int && reply == 5 as libc::c_int as *mut libc::c_void {
        printf(b"\x1B[0;32mPASSED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(b"\x1B[0;31mFAILED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
        fails += 1;
        fails;
    }
    redisReaderFree(reader);
    tests += 1;
    printf(b"#%02d \0" as *const u8 as *const libc::c_char, tests);
    printf(
        b"Works when a single newline (\\r\\n) covers two calls to feed: \0" as *const u8
            as *const libc::c_char,
    );
    reader = redisReaderCreate();
    (*reader).fn_0 = 0 as *mut redisReplyObjectFunctions;
    redisReaderFeed(
        reader,
        b"+OK\r\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        4 as libc::c_int as size_t,
    );
    ret = redisReaderGetReply(reader, &mut reply);
    if ret == 0 as libc::c_int && reply.is_null() {} else {
        __assert_fail(
            b"ret == REDIS_OK && reply == NULL\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            613 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 29],
                &[libc::c_char; 29],
            >(b"void test_reply_reader(void)\0"))
                .as_ptr(),
        );
    };
    redisReaderFeed(
        reader,
        b"\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        1 as libc::c_int as size_t,
    );
    ret = redisReaderGetReply(reader, &mut reply);
    if ret == 0 as libc::c_int && reply == 5 as libc::c_int as *mut libc::c_void {
        printf(b"\x1B[0;32mPASSED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(b"\x1B[0;31mFAILED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
        fails += 1;
        fails;
    }
    redisReaderFree(reader);
    tests += 1;
    printf(b"#%02d \0" as *const u8 as *const libc::c_char, tests);
    printf(
        b"Don't reset state after protocol error: \0" as *const u8 as *const libc::c_char,
    );
    reader = redisReaderCreate();
    (*reader).fn_0 = 0 as *mut redisReplyObjectFunctions;
    redisReaderFeed(
        reader,
        b"x\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        1 as libc::c_int as size_t,
    );
    ret = redisReaderGetReply(reader, &mut reply);
    if ret == -(1 as libc::c_int) {} else {
        __assert_fail(
            b"ret == REDIS_ERR\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            624 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 29],
                &[libc::c_char; 29],
            >(b"void test_reply_reader(void)\0"))
                .as_ptr(),
        );
    };
    ret = redisReaderGetReply(reader, &mut reply);
    if ret == -(1 as libc::c_int) && reply.is_null() {
        printf(b"\x1B[0;32mPASSED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(b"\x1B[0;31mFAILED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
        fails += 1;
        fails;
    }
    redisReaderFree(reader);
    tests += 1;
    printf(b"#%02d \0" as *const u8 as *const libc::c_char, tests);
    printf(
        b"Don't reset state after protocol error(not segfault): \0" as *const u8
            as *const libc::c_char,
    );
    reader = redisReaderCreate();
    redisReaderFeed(
        reader,
        b"*3\r\n$3\r\nSET\r\n$5\r\nhello\r\n$\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        25 as libc::c_int as size_t,
    );
    ret = redisReaderGetReply(reader, &mut reply);
    if ret == 0 as libc::c_int {} else {
        __assert_fail(
            b"ret == REDIS_OK\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            633 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 29],
                &[libc::c_char; 29],
            >(b"void test_reply_reader(void)\0"))
                .as_ptr(),
        );
    };
    redisReaderFeed(
        reader,
        b"3\r\nval\r\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        8 as libc::c_int as size_t,
    );
    ret = redisReaderGetReply(reader, &mut reply);
    if ret == 0 as libc::c_int
        && (*(reply as *mut redisReply)).type_0 == 2 as libc::c_int
        && (*(reply as *mut redisReply)).elements == 3 as libc::c_int as libc::c_ulong
    {
        printf(b"\x1B[0;32mPASSED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(b"\x1B[0;31mFAILED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
        fails += 1;
        fails;
    }
    freeReplyObject(reply);
    redisReaderFree(reader);
    tests += 1;
    printf(b"#%02d \0" as *const u8 as *const libc::c_char, tests);
    printf(
        b"Don't do empty allocation for empty multi bulk: \0" as *const u8
            as *const libc::c_char,
    );
    reader = redisReaderCreate();
    redisReaderFeed(
        reader,
        b"*0\r\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        4 as libc::c_int as size_t,
    );
    ret = redisReaderGetReply(reader, &mut reply);
    if ret == 0 as libc::c_int
        && (*(reply as *mut redisReply)).type_0 == 2 as libc::c_int
        && (*(reply as *mut redisReply)).elements == 0 as libc::c_int as libc::c_ulong
    {
        printf(b"\x1B[0;32mPASSED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(b"\x1B[0;31mFAILED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
        fails += 1;
        fails;
    }
    freeReplyObject(reply);
    redisReaderFree(reader);
    tests += 1;
    printf(b"#%02d \0" as *const u8 as *const libc::c_char, tests);
    printf(b"Can parse RESP3 verbatim strings: \0" as *const u8 as *const libc::c_char);
    reader = redisReaderCreate();
    redisReaderFeed(
        reader,
        b"=10\r\ntxt:LOLWUT\r\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        17 as libc::c_int as size_t,
    );
    ret = redisReaderGetReply(reader, &mut reply);
    if ret == 0 as libc::c_int
        && (*(reply as *mut redisReply)).type_0 == 14 as libc::c_int
        && memcmp(
            (*(reply as *mut redisReply)).str_0 as *const libc::c_void,
            b"LOLWUT\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            6 as libc::c_int as libc::c_ulong,
        ) == 0
    {
        printf(b"\x1B[0;32mPASSED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(b"\x1B[0;31mFAILED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
        fails += 1;
        fails;
    }
    freeReplyObject(reply);
    redisReaderFree(reader);
    tests += 1;
    printf(b"#%02d \0" as *const u8 as *const libc::c_char, tests);
    printf(b"Can parse RESP3 push messages: \0" as *const u8 as *const libc::c_char);
    reader = redisReaderCreate();
    redisReaderFeed(
        reader,
        b">2\r\n$6\r\nLOLWUT\r\n:42\r\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        21 as libc::c_int as size_t,
    );
    ret = redisReaderGetReply(reader, &mut reply);
    if ret == 0 as libc::c_int
        && (*(reply as *mut redisReply)).type_0 == 12 as libc::c_int
        && (*(reply as *mut redisReply)).elements == 2 as libc::c_int as libc::c_ulong
        && (**((*(reply as *mut redisReply)).element).offset(0 as libc::c_int as isize))
            .type_0 == 1 as libc::c_int
        && memcmp(
            (**((*(reply as *mut redisReply)).element).offset(0 as libc::c_int as isize))
                .str_0 as *const libc::c_void,
            b"LOLWUT\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            6 as libc::c_int as libc::c_ulong,
        ) == 0
        && (**((*(reply as *mut redisReply)).element).offset(1 as libc::c_int as isize))
            .type_0 == 3 as libc::c_int
        && (**((*(reply as *mut redisReply)).element).offset(1 as libc::c_int as isize))
            .integer == 42 as libc::c_int as libc::c_longlong
    {
        printf(b"\x1B[0;32mPASSED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(b"\x1B[0;31mFAILED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
        fails += 1;
        fails;
    }
    freeReplyObject(reply);
    redisReaderFree(reader);
    tests += 1;
    printf(b"#%02d \0" as *const u8 as *const libc::c_char, tests);
    printf(b"Can parse RESP3 doubles: \0" as *const u8 as *const libc::c_char);
    reader = redisReaderCreate();
    redisReaderFeed(
        reader,
        b",3.14159265358979323846\r\n\0" as *const u8 as *const libc::c_char,
        25 as libc::c_int as size_t,
    );
    ret = redisReaderGetReply(reader, &mut reply);
    if ret == 0 as libc::c_int
        && (*(reply as *mut redisReply)).type_0 == 7 as libc::c_int
        && fabs((*(reply as *mut redisReply)).dval - 3.14159265358979323846f64)
            < 0.00000001f64
        && (*(reply as *mut redisReply)).len == 22 as libc::c_int as libc::c_ulong
        && strcmp(
            (*(reply as *mut redisReply)).str_0,
            b"3.14159265358979323846\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
    {
        printf(b"\x1B[0;32mPASSED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(b"\x1B[0;31mFAILED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
        fails += 1;
        fails;
    }
    freeReplyObject(reply);
    redisReaderFree(reader);
    tests += 1;
    printf(b"#%02d \0" as *const u8 as *const libc::c_char, tests);
    printf(b"Set error on invalid RESP3 double: \0" as *const u8 as *const libc::c_char);
    reader = redisReaderCreate();
    redisReaderFeed(
        reader,
        b",3.14159\x00265358979323846\r\n\0" as *const u8 as *const libc::c_char,
        26 as libc::c_int as size_t,
    );
    ret = redisReaderGetReply(reader, &mut reply);
    if ret == -(1 as libc::c_int)
        && strcasecmp(
            ((*reader).errstr).as_mut_ptr(),
            b"Bad double value\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
    {
        printf(b"\x1B[0;32mPASSED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(b"\x1B[0;31mFAILED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
        fails += 1;
        fails;
    }
    freeReplyObject(reply);
    redisReaderFree(reader);
    tests += 1;
    printf(b"#%02d \0" as *const u8 as *const libc::c_char, tests);
    printf(
        b"Correctly parses RESP3 double INFINITY: \0" as *const u8 as *const libc::c_char,
    );
    reader = redisReaderCreate();
    redisReaderFeed(
        reader,
        b",inf\r\n\0" as *const u8 as *const libc::c_char,
        6 as libc::c_int as size_t,
    );
    ret = redisReaderGetReply(reader, &mut reply);
    if ret == 0 as libc::c_int
        && (*(reply as *mut redisReply)).type_0 == 7 as libc::c_int
        && if ((*(reply as *mut redisReply)).dval).is_infinite() {
            if ((*(reply as *mut redisReply)).dval).is_sign_positive() { 1 } else { -1 }
        } else {
            0
        } != 0 && (*(reply as *mut redisReply)).dval > 0 as libc::c_int as libc::c_double
    {
        printf(b"\x1B[0;32mPASSED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(b"\x1B[0;31mFAILED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
        fails += 1;
        fails;
    }
    freeReplyObject(reply);
    redisReaderFree(reader);
    tests += 1;
    printf(b"#%02d \0" as *const u8 as *const libc::c_char, tests);
    printf(b"Correctly parses RESP3 double NaN: \0" as *const u8 as *const libc::c_char);
    reader = redisReaderCreate();
    redisReaderFeed(
        reader,
        b",nan\r\n\0" as *const u8 as *const libc::c_char,
        6 as libc::c_int as size_t,
    );
    ret = redisReaderGetReply(reader, &mut reply);
    if ret == 0 as libc::c_int
        && (*(reply as *mut redisReply)).type_0 == 7 as libc::c_int
        && ((*(reply as *mut redisReply)).dval).is_nan() as i32 != 0
    {
        printf(b"\x1B[0;32mPASSED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(b"\x1B[0;31mFAILED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
        fails += 1;
        fails;
    }
    freeReplyObject(reply);
    redisReaderFree(reader);
    tests += 1;
    printf(b"#%02d \0" as *const u8 as *const libc::c_char, tests);
    printf(
        b"Correctly parses RESP3 double -Nan: \0" as *const u8 as *const libc::c_char,
    );
    reader = redisReaderCreate();
    redisReaderFeed(
        reader,
        b",-nan\r\n\0" as *const u8 as *const libc::c_char,
        7 as libc::c_int as size_t,
    );
    ret = redisReaderGetReply(reader, &mut reply);
    if ret == 0 as libc::c_int
        && (*(reply as *mut redisReply)).type_0 == 7 as libc::c_int
        && ((*(reply as *mut redisReply)).dval).is_nan() as i32 != 0
    {
        printf(b"\x1B[0;32mPASSED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(b"\x1B[0;31mFAILED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
        fails += 1;
        fails;
    }
    freeReplyObject(reply);
    redisReaderFree(reader);
    tests += 1;
    printf(b"#%02d \0" as *const u8 as *const libc::c_char, tests);
    printf(b"Can parse RESP3 nil: \0" as *const u8 as *const libc::c_char);
    reader = redisReaderCreate();
    redisReaderFeed(
        reader,
        b"_\r\n\0" as *const u8 as *const libc::c_char,
        3 as libc::c_int as size_t,
    );
    ret = redisReaderGetReply(reader, &mut reply);
    if ret == 0 as libc::c_int
        && (*(reply as *mut redisReply)).type_0 == 4 as libc::c_int
    {
        printf(b"\x1B[0;32mPASSED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(b"\x1B[0;31mFAILED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
        fails += 1;
        fails;
    }
    freeReplyObject(reply);
    redisReaderFree(reader);
    tests += 1;
    printf(b"#%02d \0" as *const u8 as *const libc::c_char, tests);
    printf(b"Set error on invalid RESP3 nil: \0" as *const u8 as *const libc::c_char);
    reader = redisReaderCreate();
    redisReaderFeed(
        reader,
        b"_nil\r\n\0" as *const u8 as *const libc::c_char,
        6 as libc::c_int as size_t,
    );
    ret = redisReaderGetReply(reader, &mut reply);
    if ret == -(1 as libc::c_int)
        && strcasecmp(
            ((*reader).errstr).as_mut_ptr(),
            b"Bad nil value\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
    {
        printf(b"\x1B[0;32mPASSED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(b"\x1B[0;31mFAILED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
        fails += 1;
        fails;
    }
    freeReplyObject(reply);
    redisReaderFree(reader);
    tests += 1;
    printf(b"#%02d \0" as *const u8 as *const libc::c_char, tests);
    printf(b"Can parse RESP3 bool (true): \0" as *const u8 as *const libc::c_char);
    reader = redisReaderCreate();
    redisReaderFeed(
        reader,
        b"#t\r\n\0" as *const u8 as *const libc::c_char,
        4 as libc::c_int as size_t,
    );
    ret = redisReaderGetReply(reader, &mut reply);
    if ret == 0 as libc::c_int
        && (*(reply as *mut redisReply)).type_0 == 8 as libc::c_int
        && (*(reply as *mut redisReply)).integer != 0
    {
        printf(b"\x1B[0;32mPASSED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(b"\x1B[0;31mFAILED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
        fails += 1;
        fails;
    }
    freeReplyObject(reply);
    redisReaderFree(reader);
    tests += 1;
    printf(b"#%02d \0" as *const u8 as *const libc::c_char, tests);
    printf(b"Can parse RESP3 bool (false): \0" as *const u8 as *const libc::c_char);
    reader = redisReaderCreate();
    redisReaderFeed(
        reader,
        b"#f\r\n\0" as *const u8 as *const libc::c_char,
        4 as libc::c_int as size_t,
    );
    ret = redisReaderGetReply(reader, &mut reply);
    if ret == 0 as libc::c_int
        && (*(reply as *mut redisReply)).type_0 == 8 as libc::c_int
        && (*(reply as *mut redisReply)).integer == 0
    {
        printf(b"\x1B[0;32mPASSED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(b"\x1B[0;31mFAILED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
        fails += 1;
        fails;
    }
    freeReplyObject(reply);
    redisReaderFree(reader);
    tests += 1;
    printf(b"#%02d \0" as *const u8 as *const libc::c_char, tests);
    printf(b"Set error on invalid RESP3 bool: \0" as *const u8 as *const libc::c_char);
    reader = redisReaderCreate();
    redisReaderFeed(
        reader,
        b"#foobar\r\n\0" as *const u8 as *const libc::c_char,
        9 as libc::c_int as size_t,
    );
    ret = redisReaderGetReply(reader, &mut reply);
    if ret == -(1 as libc::c_int)
        && strcasecmp(
            ((*reader).errstr).as_mut_ptr(),
            b"Bad bool value\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
    {
        printf(b"\x1B[0;32mPASSED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(b"\x1B[0;31mFAILED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
        fails += 1;
        fails;
    }
    freeReplyObject(reply);
    redisReaderFree(reader);
    tests += 1;
    printf(b"#%02d \0" as *const u8 as *const libc::c_char, tests);
    printf(b"Can parse RESP3 map: \0" as *const u8 as *const libc::c_char);
    reader = redisReaderCreate();
    redisReaderFeed(
        reader,
        b"%2\r\n+first\r\n:123\r\n$6\r\nsecond\r\n#t\r\n\0" as *const u8
            as *const libc::c_char,
        34 as libc::c_int as size_t,
    );
    ret = redisReaderGetReply(reader, &mut reply);
    if ret == 0 as libc::c_int
        && (*(reply as *mut redisReply)).type_0 == 9 as libc::c_int
        && (*(reply as *mut redisReply)).elements == 4 as libc::c_int as libc::c_ulong
        && (**((*(reply as *mut redisReply)).element).offset(0 as libc::c_int as isize))
            .type_0 == 5 as libc::c_int
        && (**((*(reply as *mut redisReply)).element).offset(0 as libc::c_int as isize))
            .len == 5 as libc::c_int as libc::c_ulong
        && strcmp(
            (**((*(reply as *mut redisReply)).element).offset(0 as libc::c_int as isize))
                .str_0,
            b"first\0" as *const u8 as *const libc::c_char,
        ) == 0
        && (**((*(reply as *mut redisReply)).element).offset(1 as libc::c_int as isize))
            .type_0 == 3 as libc::c_int
        && (**((*(reply as *mut redisReply)).element).offset(1 as libc::c_int as isize))
            .integer == 123 as libc::c_int as libc::c_longlong
        && (**((*(reply as *mut redisReply)).element).offset(2 as libc::c_int as isize))
            .type_0 == 1 as libc::c_int
        && (**((*(reply as *mut redisReply)).element).offset(2 as libc::c_int as isize))
            .len == 6 as libc::c_int as libc::c_ulong
        && strcmp(
            (**((*(reply as *mut redisReply)).element).offset(2 as libc::c_int as isize))
                .str_0,
            b"second\0" as *const u8 as *const libc::c_char,
        ) == 0
        && (**((*(reply as *mut redisReply)).element).offset(3 as libc::c_int as isize))
            .type_0 == 8 as libc::c_int
        && (**((*(reply as *mut redisReply)).element).offset(3 as libc::c_int as isize))
            .integer != 0
    {
        printf(b"\x1B[0;32mPASSED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(b"\x1B[0;31mFAILED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
        fails += 1;
        fails;
    }
    freeReplyObject(reply);
    redisReaderFree(reader);
    tests += 1;
    printf(b"#%02d \0" as *const u8 as *const libc::c_char, tests);
    printf(b"Can parse RESP3 attribute: \0" as *const u8 as *const libc::c_char);
    reader = redisReaderCreate();
    redisReaderFeed(
        reader,
        b"|2\r\n+foo\r\n:123\r\n+bar\r\n#t\r\n\0" as *const u8 as *const libc::c_char,
        26 as libc::c_int as size_t,
    );
    ret = redisReaderGetReply(reader, &mut reply);
    if ret == 0 as libc::c_int
        && (*(reply as *mut redisReply)).type_0 == 11 as libc::c_int
        && (*(reply as *mut redisReply)).elements == 4 as libc::c_int as libc::c_ulong
        && (**((*(reply as *mut redisReply)).element).offset(0 as libc::c_int as isize))
            .type_0 == 5 as libc::c_int
        && (**((*(reply as *mut redisReply)).element).offset(0 as libc::c_int as isize))
            .len == 3 as libc::c_int as libc::c_ulong
        && strcmp(
            (**((*(reply as *mut redisReply)).element).offset(0 as libc::c_int as isize))
                .str_0,
            b"foo\0" as *const u8 as *const libc::c_char,
        ) == 0
        && (**((*(reply as *mut redisReply)).element).offset(1 as libc::c_int as isize))
            .type_0 == 3 as libc::c_int
        && (**((*(reply as *mut redisReply)).element).offset(1 as libc::c_int as isize))
            .integer == 123 as libc::c_int as libc::c_longlong
        && (**((*(reply as *mut redisReply)).element).offset(2 as libc::c_int as isize))
            .type_0 == 5 as libc::c_int
        && (**((*(reply as *mut redisReply)).element).offset(2 as libc::c_int as isize))
            .len == 3 as libc::c_int as libc::c_ulong
        && strcmp(
            (**((*(reply as *mut redisReply)).element).offset(2 as libc::c_int as isize))
                .str_0,
            b"bar\0" as *const u8 as *const libc::c_char,
        ) == 0
        && (**((*(reply as *mut redisReply)).element).offset(3 as libc::c_int as isize))
            .type_0 == 8 as libc::c_int
        && (**((*(reply as *mut redisReply)).element).offset(3 as libc::c_int as isize))
            .integer != 0
    {
        printf(b"\x1B[0;32mPASSED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(b"\x1B[0;31mFAILED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
        fails += 1;
        fails;
    }
    freeReplyObject(reply);
    redisReaderFree(reader);
    tests += 1;
    printf(b"#%02d \0" as *const u8 as *const libc::c_char, tests);
    printf(b"Can parse RESP3 set: \0" as *const u8 as *const libc::c_char);
    reader = redisReaderCreate();
    redisReaderFeed(
        reader,
        b"~5\r\n+orange\r\n$5\r\napple\r\n#f\r\n:100\r\n:999\r\n\0" as *const u8
            as *const libc::c_char,
        40 as libc::c_int as size_t,
    );
    ret = redisReaderGetReply(reader, &mut reply);
    if ret == 0 as libc::c_int
        && (*(reply as *mut redisReply)).type_0 == 10 as libc::c_int
        && (*(reply as *mut redisReply)).elements == 5 as libc::c_int as libc::c_ulong
        && (**((*(reply as *mut redisReply)).element).offset(0 as libc::c_int as isize))
            .type_0 == 5 as libc::c_int
        && (**((*(reply as *mut redisReply)).element).offset(0 as libc::c_int as isize))
            .len == 6 as libc::c_int as libc::c_ulong
        && strcmp(
            (**((*(reply as *mut redisReply)).element).offset(0 as libc::c_int as isize))
                .str_0,
            b"orange\0" as *const u8 as *const libc::c_char,
        ) == 0
        && (**((*(reply as *mut redisReply)).element).offset(1 as libc::c_int as isize))
            .type_0 == 1 as libc::c_int
        && (**((*(reply as *mut redisReply)).element).offset(1 as libc::c_int as isize))
            .len == 5 as libc::c_int as libc::c_ulong
        && strcmp(
            (**((*(reply as *mut redisReply)).element).offset(1 as libc::c_int as isize))
                .str_0,
            b"apple\0" as *const u8 as *const libc::c_char,
        ) == 0
        && (**((*(reply as *mut redisReply)).element).offset(2 as libc::c_int as isize))
            .type_0 == 8 as libc::c_int
        && (**((*(reply as *mut redisReply)).element).offset(2 as libc::c_int as isize))
            .integer == 0
        && (**((*(reply as *mut redisReply)).element).offset(3 as libc::c_int as isize))
            .type_0 == 3 as libc::c_int
        && (**((*(reply as *mut redisReply)).element).offset(3 as libc::c_int as isize))
            .integer == 100 as libc::c_int as libc::c_longlong
        && (**((*(reply as *mut redisReply)).element).offset(4 as libc::c_int as isize))
            .type_0 == 3 as libc::c_int
        && (**((*(reply as *mut redisReply)).element).offset(4 as libc::c_int as isize))
            .integer == 999 as libc::c_int as libc::c_longlong
    {
        printf(b"\x1B[0;32mPASSED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(b"\x1B[0;31mFAILED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
        fails += 1;
        fails;
    }
    freeReplyObject(reply);
    redisReaderFree(reader);
    tests += 1;
    printf(b"#%02d \0" as *const u8 as *const libc::c_char, tests);
    printf(b"Can parse RESP3 bignum: \0" as *const u8 as *const libc::c_char);
    reader = redisReaderCreate();
    redisReaderFeed(
        reader,
        b"(3492890328409238509324850943850943825024385\r\n\0" as *const u8
            as *const libc::c_char,
        46 as libc::c_int as size_t,
    );
    ret = redisReaderGetReply(reader, &mut reply);
    if ret == 0 as libc::c_int
        && (*(reply as *mut redisReply)).type_0 == 13 as libc::c_int
        && (*(reply as *mut redisReply)).len == 43 as libc::c_int as libc::c_ulong
        && strcmp(
            (*(reply as *mut redisReply)).str_0,
            b"3492890328409238509324850943850943825024385\0" as *const u8
                as *const libc::c_char,
        ) == 0
    {
        printf(b"\x1B[0;32mPASSED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(b"\x1B[0;31mFAILED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
        fails += 1;
        fails;
    }
    freeReplyObject(reply);
    redisReaderFree(reader);
    tests += 1;
    printf(b"#%02d \0" as *const u8 as *const libc::c_char, tests);
    printf(
        b"Can parse RESP3 doubles in an array: \0" as *const u8 as *const libc::c_char,
    );
    reader = redisReaderCreate();
    redisReaderFeed(
        reader,
        b"*1\r\n,3.14159265358979323846\r\n\0" as *const u8 as *const libc::c_char,
        31 as libc::c_int as size_t,
    );
    ret = redisReaderGetReply(reader, &mut reply);
    if ret == 0 as libc::c_int
        && (*(reply as *mut redisReply)).type_0 == 2 as libc::c_int
        && (*(reply as *mut redisReply)).elements == 1 as libc::c_int as libc::c_ulong
        && (**((*(reply as *mut redisReply)).element).offset(0 as libc::c_int as isize))
            .type_0 == 7 as libc::c_int
        && fabs(
            (**((*(reply as *mut redisReply)).element).offset(0 as libc::c_int as isize))
                .dval - 3.14159265358979323846f64,
        ) < 0.00000001f64
        && (**((*(reply as *mut redisReply)).element).offset(0 as libc::c_int as isize))
            .len == 22 as libc::c_int as libc::c_ulong
        && strcmp(
            (**((*(reply as *mut redisReply)).element).offset(0 as libc::c_int as isize))
                .str_0,
            b"3.14159265358979323846\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
    {
        printf(b"\x1B[0;32mPASSED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(b"\x1B[0;31mFAILED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
        fails += 1;
        fails;
    }
    freeReplyObject(reply);
    redisReaderFree(reader);
}
unsafe extern "C" fn test_free_null() {
    let mut redisCtx: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut reply: *mut libc::c_void = 0 as *mut libc::c_void;
    tests += 1;
    printf(b"#%02d \0" as *const u8 as *const libc::c_char, tests);
    printf(
        b"Don't fail when redisFree is passed a NULL value: \0" as *const u8
            as *const libc::c_char,
    );
    redisFree(redisCtx as *mut redisContext);
    if redisCtx.is_null() {
        printf(b"\x1B[0;32mPASSED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(b"\x1B[0;31mFAILED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
        fails += 1;
        fails;
    }
    tests += 1;
    printf(b"#%02d \0" as *const u8 as *const libc::c_char, tests);
    printf(
        b"Don't fail when freeReplyObject is passed a NULL value: \0" as *const u8
            as *const libc::c_char,
    );
    freeReplyObject(reply);
    if reply.is_null() {
        printf(b"\x1B[0;32mPASSED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(b"\x1B[0;31mFAILED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
        fails += 1;
        fails;
    };
}
unsafe extern "C" fn hi_malloc_fail(mut size: size_t) -> *mut libc::c_void {
    return 0 as *mut libc::c_void;
}
unsafe extern "C" fn hi_calloc_fail(
    mut nmemb: size_t,
    mut size: size_t,
) -> *mut libc::c_void {
    return 0 as *mut libc::c_void;
}
unsafe extern "C" fn hi_calloc_insecure(
    mut nmemb: size_t,
    mut size: size_t,
) -> *mut libc::c_void {
    insecure_calloc_calls += 1;
    insecure_calloc_calls;
    return 0xdeadc0de as libc::c_uint as *mut libc::c_void;
}
unsafe extern "C" fn hi_realloc_fail(
    mut ptr: *mut libc::c_void,
    mut size: size_t,
) -> *mut libc::c_void {
    return 0 as *mut libc::c_void;
}
unsafe extern "C" fn test_allocator_injection() {
    let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut ha: hiredisAllocFuncs = {
        let mut init = hiredisAllocFuncs {
            mallocFn: Some(
                hi_malloc_fail as unsafe extern "C" fn(size_t) -> *mut libc::c_void,
            ),
            callocFn: Some(
                hi_calloc_fail
                    as unsafe extern "C" fn(size_t, size_t) -> *mut libc::c_void,
            ),
            reallocFn: Some(
                hi_realloc_fail
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        size_t,
                    ) -> *mut libc::c_void,
            ),
            strdupFn: Some(
                strdup as unsafe extern "C" fn(*const libc::c_char) -> *mut libc::c_char,
            ),
            freeFn: Some(free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
        };
        init
    };
    hiredisSetAllocators(&mut ha);
    tests += 1;
    printf(b"#%02d \0" as *const u8 as *const libc::c_char, tests);
    printf(
        b"redisContext uses injected allocators: \0" as *const u8 as *const libc::c_char,
    );
    let mut c: *mut redisContext = redisConnect(
        b"localhost\0" as *const u8 as *const libc::c_char,
        6379 as libc::c_int,
    );
    if c.is_null() {
        printf(b"\x1B[0;32mPASSED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(b"\x1B[0;31mFAILED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
        fails += 1;
        fails;
    }
    tests += 1;
    printf(b"#%02d \0" as *const u8 as *const libc::c_char, tests);
    printf(
        b"redisReader uses injected allocators: \0" as *const u8 as *const libc::c_char,
    );
    let mut reader: *mut redisReader = redisReaderCreate();
    if reader.is_null() {
        printf(b"\x1B[0;32mPASSED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(b"\x1B[0;31mFAILED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
        fails += 1;
        fails;
    }
    tests += 1;
    printf(b"#%02d \0" as *const u8 as *const libc::c_char, tests);
    printf(
        b"hiredis calloc wrapper protects against overflow: \0" as *const u8
            as *const libc::c_char,
    );
    ha
        .callocFn = Some(
        hi_calloc_insecure as unsafe extern "C" fn(size_t, size_t) -> *mut libc::c_void,
    );
    hiredisSetAllocators(&mut ha);
    ptr = hi_calloc(
        (18446744073709551615 as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
            .wrapping_add(3 as libc::c_int as libc::c_ulong),
        ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
    );
    if ptr.is_null() && insecure_calloc_calls == 0 as libc::c_int {
        printf(b"\x1B[0;32mPASSED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(b"\x1B[0;31mFAILED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
        fails += 1;
        fails;
    }
    hiredisResetAllocators();
}
unsafe extern "C" fn test_blocking_connection_errors() {
    let mut hints: addrinfo = {
        let mut init = addrinfo {
            ai_flags: 0,
            ai_family: 2 as libc::c_int,
            ai_socktype: 0,
            ai_protocol: 0,
            ai_addrlen: 0,
            ai_addr: 0 as *mut sockaddr,
            ai_canonname: 0 as *mut libc::c_char,
            ai_next: 0 as *mut addrinfo,
        };
        init
    };
    let mut ai_tmp: *mut addrinfo = 0 as *mut addrinfo;
    let mut c: *mut redisContext = 0 as *mut redisContext;
    let mut rv: libc::c_int = getaddrinfo(
        b"idontexist-noreally.com\0" as *const u8 as *const libc::c_char,
        b"6379\0" as *const u8 as *const libc::c_char,
        &mut hints,
        &mut ai_tmp,
    );
    if rv != 0 as libc::c_int {
        tests += 1;
        printf(b"#%02d \0" as *const u8 as *const libc::c_char, tests);
        printf(
            b"Returns error when host cannot be resolved: \0" as *const u8
                as *const libc::c_char,
        );
        c = redisConnect(
            b"idontexist-noreally.com\0" as *const u8 as *const libc::c_char,
            6379 as libc::c_int,
        );
        if (*c).err == 2 as libc::c_int
            && (strcmp(
                ((*c).errstr).as_mut_ptr(),
                b"Name or service not known\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
                || strcmp(
                    ((*c).errstr).as_mut_ptr(),
                    b"Can't resolve: idontexist-noreally.com\0" as *const u8
                        as *const libc::c_char,
                ) == 0 as libc::c_int
                || strcmp(
                    ((*c).errstr).as_mut_ptr(),
                    b"Name does not resolve\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                || strcmp(
                    ((*c).errstr).as_mut_ptr(),
                    b"nodename nor servname provided, or not known\0" as *const u8
                        as *const libc::c_char,
                ) == 0 as libc::c_int
                || strcmp(
                    ((*c).errstr).as_mut_ptr(),
                    b"node name or service name not known\0" as *const u8
                        as *const libc::c_char,
                ) == 0 as libc::c_int
                || strcmp(
                    ((*c).errstr).as_mut_ptr(),
                    b"No address associated with hostname\0" as *const u8
                        as *const libc::c_char,
                ) == 0 as libc::c_int
                || strcmp(
                    ((*c).errstr).as_mut_ptr(),
                    b"Temporary failure in name resolution\0" as *const u8
                        as *const libc::c_char,
                ) == 0 as libc::c_int
                || strcmp(
                    ((*c).errstr).as_mut_ptr(),
                    b"hostname nor servname provided, or not known\0" as *const u8
                        as *const libc::c_char,
                ) == 0 as libc::c_int
                || strcmp(
                    ((*c).errstr).as_mut_ptr(),
                    b"no address associated with name\0" as *const u8
                        as *const libc::c_char,
                ) == 0 as libc::c_int
                || strcmp(
                    ((*c).errstr).as_mut_ptr(),
                    b"No such host is known. \0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int)
        {
            printf(b"\x1B[0;32mPASSED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
        } else {
            printf(b"\x1B[0;31mFAILED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
            fails += 1;
            fails;
        }
        redisFree(c);
    } else {
        printf(
            b"Skipping NXDOMAIN test. Found evil ISP!\n\0" as *const u8
                as *const libc::c_char,
        );
        freeaddrinfo(ai_tmp);
    }
    let mut opt: redisOptions = {
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
    let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    tests += 1;
    printf(b"#%02d \0" as *const u8 as *const libc::c_char, tests);
    printf(
        b"Returns error when the port is not open: \0" as *const u8
            as *const libc::c_char,
    );
    c = redisConnect(
        b"localhost\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        1 as libc::c_int,
    );
    if (*c).err == 1 as libc::c_int
        && strcmp(
            ((*c).errstr).as_mut_ptr(),
            b"Connection refused\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
    {
        printf(b"\x1B[0;32mPASSED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(b"\x1B[0;31mFAILED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
        fails += 1;
        fails;
    }
    redisFree(c);
    tests += 1;
    printf(b"#%02d \0" as *const u8 as *const libc::c_char, tests);
    printf(
        b"We don't clobber connection exception with setsockopt error: \0" as *const u8
            as *const libc::c_char,
    );
    tv = {
        let mut init = timeval {
            tv_sec: 0 as libc::c_int as __time_t,
            tv_usec: 500000 as libc::c_int as __suseconds_t,
        };
        init
    };
    opt.connect_timeout = &mut tv;
    opt.command_timeout = opt.connect_timeout;
    opt.type_0 = REDIS_CONN_TCP as libc::c_int;
    opt.endpoint.tcp.ip = b"localhost\0" as *const u8 as *const libc::c_char;
    opt.endpoint.tcp.port = 10337 as libc::c_int;
    c = redisConnectWithOptions(&mut opt);
    if (*c).err == 1 as libc::c_int
        && strcmp(
            ((*c).errstr).as_mut_ptr(),
            b"Connection refused\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
    {
        printf(b"\x1B[0;32mPASSED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(b"\x1B[0;31mFAILED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
        fails += 1;
        fails;
    }
    redisFree(c);
    tests += 1;
    printf(b"#%02d \0" as *const u8 as *const libc::c_char, tests);
    printf(
        b"Returns error when the unix_sock socket path doesn't accept connections: \0"
            as *const u8 as *const libc::c_char,
    );
    c = redisConnectUnix(
        b"/tmp/idontexist.sock\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    if (*c).err == 1 as libc::c_int {
        printf(b"\x1B[0;32mPASSED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(b"\x1B[0;31mFAILED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
        fails += 1;
        fails;
    }
    redisFree(c);
}
pub unsafe extern "C" fn push_handler(
    mut privdata: *mut libc::c_void,
    mut r: *mut libc::c_void,
) {
    let mut pcounts: *mut pushCounters = privdata as *mut pushCounters;
    let mut reply: *mut redisReply = r as *mut redisReply;
    let mut payload: *mut redisReply = 0 as *mut redisReply;
    if !reply.is_null() && (*reply).type_0 == 12 as libc::c_int
        && (*reply).elements == 2 as libc::c_int as libc::c_ulong
    {} else {
        __assert_fail(
            b"reply && reply->type == REDIS_REPLY_PUSH && reply->elements == 2\0"
                as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            999 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 34],
                &[libc::c_char; 34],
            >(b"void push_handler(void *, void *)\0"))
                .as_ptr(),
        );
    };
    payload = *((*reply).element).offset(1 as libc::c_int as isize);
    if (*payload).type_0 == 2 as libc::c_int {
        payload = *((*payload).element).offset(0 as libc::c_int as isize);
    }
    if (*payload).type_0 == 1 as libc::c_int {
        (*pcounts).str_0 += 1;
        (*pcounts).str_0;
    } else if (*payload).type_0 == 4 as libc::c_int {
        (*pcounts).nil += 1;
        (*pcounts).nil;
    }
    freeReplyObject(reply as *mut libc::c_void);
}
pub unsafe extern "C" fn push_handler_async(
    mut ac: *mut redisAsyncContext,
    mut reply: *mut libc::c_void,
) {}
unsafe extern "C" fn test_resp3_push_handler(mut c: *mut redisContext) {
    let mut pc: pushCounters = {
        let mut init = pushCounters {
            nil: 0 as libc::c_int,
            str_0: 0,
        };
        init
    };
    let mut old: Option::<redisPushFn> = None;
    let mut reply: *mut redisReply = 0 as *mut redisReply;
    let mut privdata: *mut libc::c_void = 0 as *mut libc::c_void;
    send_hello(c, 3 as libc::c_int);
    send_client_tracking(c, b"ON\0" as *const u8 as *const libc::c_char);
    privdata = (*c).privdata;
    (*c).privdata = &mut pc as *mut pushCounters as *mut libc::c_void;
    reply = redisCommand(c, b"GET key:0\0" as *const u8 as *const libc::c_char)
        as *mut redisReply;
    if !reply.is_null() {} else {
        __assert_fail(
            b"reply != NULL\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            1034 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 45],
                &[libc::c_char; 45],
            >(b"void test_resp3_push_handler(redisContext *)\0"))
                .as_ptr(),
        );
    };
    freeReplyObject(reply as *mut libc::c_void);
    tests += 1;
    printf(b"#%02d \0" as *const u8 as *const libc::c_char, tests);
    printf(
        b"RESP3 PUSH messages are handled out of band by default: \0" as *const u8
            as *const libc::c_char,
    );
    reply = redisCommand(c, b"SET key:0 val:0\0" as *const u8 as *const libc::c_char)
        as *mut redisReply;
    if !reply.is_null() && (*reply).type_0 == 5 as libc::c_int {
        printf(b"\x1B[0;32mPASSED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(b"\x1B[0;31mFAILED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
        fails += 1;
        fails;
    }
    freeReplyObject(reply as *mut libc::c_void);
    reply = redisCommand(c, b"GET key:0\0" as *const u8 as *const libc::c_char)
        as *mut redisReply;
    if !reply.is_null() {} else {
        __assert_fail(
            b"(reply = redisCommand(c, \"GET key:0\")) != NULL\0" as *const u8
                as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            1042 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 45],
                &[libc::c_char; 45],
            >(b"void test_resp3_push_handler(redisContext *)\0"))
                .as_ptr(),
        );
    };
    freeReplyObject(reply as *mut libc::c_void);
    old = redisSetPushCallback(
        c,
        Some(
            push_handler
                as unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
        ),
    );
    tests += 1;
    printf(b"#%02d \0" as *const u8 as *const libc::c_char, tests);
    printf(
        b"We can set a custom RESP3 PUSH handler: \0" as *const u8 as *const libc::c_char,
    );
    reply = redisCommand(c, b"SET key:0 val:0\0" as *const u8 as *const libc::c_char)
        as *mut redisReply;
    if !reply.is_null() {} else {
        __assert_fail(
            b"reply != NULL\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            1050 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 45],
                &[libc::c_char; 45],
            >(b"void test_resp3_push_handler(redisContext *)\0"))
                .as_ptr(),
        );
    };
    freeReplyObject(reply as *mut libc::c_void);
    reply = redisCommand(c, b"PING\0" as *const u8 as *const libc::c_char)
        as *mut redisReply;
    if !reply.is_null() && (*reply).type_0 == 5 as libc::c_int
        && pc.str_0 == 1 as libc::c_int
    {
        printf(b"\x1B[0;32mPASSED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(b"\x1B[0;31mFAILED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
        fails += 1;
        fails;
    }
    freeReplyObject(reply as *mut libc::c_void);
    tests += 1;
    printf(b"#%02d \0" as *const u8 as *const libc::c_char, tests);
    printf(
        b"We properly handle a NIL invalidation payload: \0" as *const u8
            as *const libc::c_char,
    );
    reply = redisCommand(c, b"FLUSHDB\0" as *const u8 as *const libc::c_char)
        as *mut redisReply;
    if !reply.is_null() {} else {
        __assert_fail(
            b"reply != NULL\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            1058 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 45],
                &[libc::c_char; 45],
            >(b"void test_resp3_push_handler(redisContext *)\0"))
                .as_ptr(),
        );
    };
    freeReplyObject(reply as *mut libc::c_void);
    reply = redisCommand(c, b"PING\0" as *const u8 as *const libc::c_char)
        as *mut redisReply;
    if !reply.is_null() && (*reply).type_0 == 5 as libc::c_int
        && pc.nil == 1 as libc::c_int
    {
        printf(b"\x1B[0;32mPASSED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(b"\x1B[0;31mFAILED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
        fails += 1;
        fails;
    }
    freeReplyObject(reply as *mut libc::c_void);
    tests += 1;
    printf(b"#%02d \0" as *const u8 as *const libc::c_char, tests);
    printf(
        b"With no handler, PUSH replies come in-band: \0" as *const u8
            as *const libc::c_char,
    );
    redisSetPushCallback(c, None);
    reply = redisCommand(c, b"GET key:0\0" as *const u8 as *const libc::c_char)
        as *mut redisReply;
    if !reply.is_null() {} else {
        __assert_fail(
            b"(reply = redisCommand(c, \"GET key:0\")) != NULL\0" as *const u8
                as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            1068 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 45],
                &[libc::c_char; 45],
            >(b"void test_resp3_push_handler(redisContext *)\0"))
                .as_ptr(),
        );
    };
    freeReplyObject(reply as *mut libc::c_void);
    reply = redisCommand(c, b"SET key:0 invalid\0" as *const u8 as *const libc::c_char)
        as *mut redisReply;
    if !reply.is_null() {} else {
        __assert_fail(
            b"(reply = redisCommand(c, \"SET key:0 invalid\")) != NULL\0" as *const u8
                as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            1070 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 45],
                &[libc::c_char; 45],
            >(b"void test_resp3_push_handler(redisContext *)\0"))
                .as_ptr(),
        );
    };
    if (*reply).type_0 == 5 as libc::c_int {
        freeReplyObject(reply as *mut libc::c_void);
        reply = redisCommand(c, b"PING\0" as *const u8 as *const libc::c_char)
            as *mut redisReply;
    }
    if (*reply).type_0 == 12 as libc::c_int {
        printf(b"\x1B[0;32mPASSED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(b"\x1B[0;31mFAILED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
        fails += 1;
        fails;
    }
    freeReplyObject(reply as *mut libc::c_void);
    tests += 1;
    printf(b"#%02d \0" as *const u8 as *const libc::c_char, tests);
    printf(
        b"With no PUSH handler, no replies are lost: \0" as *const u8
            as *const libc::c_char,
    );
    if redisGetReply(c, &mut reply as *mut *mut redisReply as *mut *mut libc::c_void)
        == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"redisGetReply(c, (void**)&reply) == REDIS_OK\0" as *const u8
                as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            1081 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 45],
                &[libc::c_char; 45],
            >(b"void test_resp3_push_handler(redisContext *)\0"))
                .as_ptr(),
        );
    };
    if !reply.is_null() && (*reply).type_0 == 5 as libc::c_int {
        printf(b"\x1B[0;32mPASSED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(b"\x1B[0;31mFAILED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
        fails += 1;
        fails;
    }
    freeReplyObject(reply as *mut libc::c_void);
    if old.is_some() {} else {
        __assert_fail(
            b"old != NULL\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            1086 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 45],
                &[libc::c_char; 45],
            >(b"void test_resp3_push_handler(redisContext *)\0"))
                .as_ptr(),
        );
    };
    redisSetPushCallback(c, old);
    (*c).privdata = privdata;
    send_client_tracking(c, b"OFF\0" as *const u8 as *const libc::c_char);
    send_hello(c, 2 as libc::c_int);
}
pub unsafe extern "C" fn get_redis_tcp_options(mut config: config) -> redisOptions {
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
    options.endpoint.tcp.ip = config.tcp.host;
    options.endpoint.tcp.port = config.tcp.port;
    return options;
}
unsafe extern "C" fn test_resp3_push_options(mut config: config) {
    let mut ac: *mut redisAsyncContext = 0 as *mut redisAsyncContext;
    let mut c: *mut redisContext = 0 as *mut redisContext;
    let mut options: redisOptions = redisOptions {
        type_0: 0,
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
    tests += 1;
    printf(b"#%02d \0" as *const u8 as *const libc::c_char, tests);
    printf(
        b"We set a default RESP3 handler for redisContext: \0" as *const u8
            as *const libc::c_char,
    );
    options = get_redis_tcp_options(config);
    c = redisConnectWithOptions(&mut options);
    if !c.is_null() {} else {
        __assert_fail(
            b"(c = redisConnectWithOptions(&options)) != NULL\0" as *const u8
                as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            1108 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 44],
                &[libc::c_char; 44],
            >(b"void test_resp3_push_options(struct config)\0"))
                .as_ptr(),
        );
    };
    if ((*c).push_cb).is_some() {
        printf(b"\x1B[0;32mPASSED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(b"\x1B[0;31mFAILED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
        fails += 1;
        fails;
    }
    redisFree(c);
    tests += 1;
    printf(b"#%02d \0" as *const u8 as *const libc::c_char, tests);
    printf(
        b"We don't set a default RESP3 push handler for redisAsyncContext: \0"
            as *const u8 as *const libc::c_char,
    );
    options = get_redis_tcp_options(config);
    ac = redisAsyncConnectWithOptions(&mut options);
    if !ac.is_null() {} else {
        __assert_fail(
            b"(ac = redisAsyncConnectWithOptions(&options)) != NULL\0" as *const u8
                as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            1114 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 44],
                &[libc::c_char; 44],
            >(b"void test_resp3_push_options(struct config)\0"))
                .as_ptr(),
        );
    };
    if ((*ac).c.push_cb).is_none() {
        printf(b"\x1B[0;32mPASSED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(b"\x1B[0;31mFAILED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
        fails += 1;
        fails;
    }
    redisAsyncFree(ac);
    tests += 1;
    printf(b"#%02d \0" as *const u8 as *const libc::c_char, tests);
    printf(
        b"Our REDIS_OPT_NO_PUSH_AUTOFREE flag works: \0" as *const u8
            as *const libc::c_char,
    );
    options = get_redis_tcp_options(config);
    options.options |= 0x8 as libc::c_int;
    c = redisConnectWithOptions(&mut options);
    if !c.is_null() {} else {
        __assert_fail(
            b"(c = redisConnectWithOptions(&options)) != NULL\0" as *const u8
                as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            1121 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 44],
                &[libc::c_char; 44],
            >(b"void test_resp3_push_options(struct config)\0"))
                .as_ptr(),
        );
    };
    if ((*c).push_cb).is_none() {
        printf(b"\x1B[0;32mPASSED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(b"\x1B[0;31mFAILED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
        fails += 1;
        fails;
    }
    redisFree(c);
    tests += 1;
    printf(b"#%02d \0" as *const u8 as *const libc::c_char, tests);
    printf(
        b"We can use redisOptions to set a custom PUSH handler for redisContext: \0"
            as *const u8 as *const libc::c_char,
    );
    options = get_redis_tcp_options(config);
    options
        .push_cb = Some(
        push_handler as unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
    );
    c = redisConnectWithOptions(&mut options);
    if !c.is_null() {} else {
        __assert_fail(
            b"(c = redisConnectWithOptions(&options)) != NULL\0" as *const u8
                as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            1128 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 44],
                &[libc::c_char; 44],
            >(b"void test_resp3_push_options(struct config)\0"))
                .as_ptr(),
        );
    };
    if (*c).push_cb
        == Some(
            push_handler
                as unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
        )
    {
        printf(b"\x1B[0;32mPASSED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(b"\x1B[0;31mFAILED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
        fails += 1;
        fails;
    }
    redisFree(c);
    tests += 1;
    printf(b"#%02d \0" as *const u8 as *const libc::c_char, tests);
    printf(
        b"We can use redisOptions to set a custom PUSH handler for redisAsyncContext: \0"
            as *const u8 as *const libc::c_char,
    );
    options = get_redis_tcp_options(config);
    options
        .async_push_cb = Some(
        push_handler_async
            as unsafe extern "C" fn(*mut redisAsyncContext, *mut libc::c_void) -> (),
    );
    ac = redisAsyncConnectWithOptions(&mut options);
    if !ac.is_null() {} else {
        __assert_fail(
            b"(ac = redisAsyncConnectWithOptions(&options)) != NULL\0" as *const u8
                as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            1135 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 44],
                &[libc::c_char; 44],
            >(b"void test_resp3_push_options(struct config)\0"))
                .as_ptr(),
        );
    };
    if (*ac).push_cb
        == Some(
            push_handler_async
                as unsafe extern "C" fn(*mut redisAsyncContext, *mut libc::c_void) -> (),
        )
    {
        printf(b"\x1B[0;32mPASSED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(b"\x1B[0;31mFAILED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
        fails += 1;
        fails;
    }
    redisAsyncFree(ac);
}
pub unsafe extern "C" fn free_privdata(mut privdata: *mut libc::c_void) {
    let mut data: *mut privdata = privdata as *mut privdata;
    (*data).dtor_counter += 1;
    (*data).dtor_counter;
}
unsafe extern "C" fn test_privdata_hooks(mut config: config) {
    let mut data: privdata = {
        let mut init = privdata {
            dtor_counter: 0 as libc::c_int,
        };
        init
    };
    let mut options: redisOptions = redisOptions {
        type_0: 0,
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
    let mut c: *mut redisContext = 0 as *mut redisContext;
    tests += 1;
    printf(b"#%02d \0" as *const u8 as *const libc::c_char, tests);
    printf(
        b"We can use redisOptions to set privdata: \0" as *const u8
            as *const libc::c_char,
    );
    options = get_redis_tcp_options(config);
    options.privdata = &mut data as *mut privdata as *mut libc::c_void;
    options
        .free_privdata = Some(
        free_privdata as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    c = redisConnectWithOptions(&mut options);
    if !c.is_null() {} else {
        __assert_fail(
            b"(c = redisConnectWithOptions(&options)) != NULL\0" as *const u8
                as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            1153 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 40],
                &[libc::c_char; 40],
            >(b"void test_privdata_hooks(struct config)\0"))
                .as_ptr(),
        );
    };
    if (*c).privdata == &mut data as *mut privdata as *mut libc::c_void {
        printf(b"\x1B[0;32mPASSED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(b"\x1B[0;31mFAILED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
        fails += 1;
        fails;
    }
    tests += 1;
    printf(b"#%02d \0" as *const u8 as *const libc::c_char, tests);
    printf(
        b"Our privdata destructor fires when we free the context: \0" as *const u8
            as *const libc::c_char,
    );
    redisFree(c);
    if data.dtor_counter == 1 as libc::c_int {
        printf(b"\x1B[0;32mPASSED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(b"\x1B[0;31mFAILED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
        fails += 1;
        fails;
    };
}
unsafe extern "C" fn test_blocking_connection(mut config: config) {
    let mut c: *mut redisContext = 0 as *mut redisContext;
    let mut reply: *mut redisReply = 0 as *mut redisReply;
    let mut major: libc::c_int = 0;
    c = do_connect(config);
    tests += 1;
    printf(b"#%02d \0" as *const u8 as *const libc::c_char, tests);
    printf(b"Is able to deliver commands: \0" as *const u8 as *const libc::c_char);
    reply = redisCommand(c, b"PING\0" as *const u8 as *const libc::c_char)
        as *mut redisReply;
    if (*reply).type_0 == 5 as libc::c_int
        && strcasecmp((*reply).str_0, b"pong\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
    {
        printf(b"\x1B[0;32mPASSED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(b"\x1B[0;31mFAILED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
        fails += 1;
        fails;
    }
    freeReplyObject(reply as *mut libc::c_void);
    tests += 1;
    printf(b"#%02d \0" as *const u8 as *const libc::c_char, tests);
    printf(
        b"Is a able to send commands verbatim: \0" as *const u8 as *const libc::c_char,
    );
    reply = redisCommand(c, b"SET foo bar\0" as *const u8 as *const libc::c_char)
        as *mut redisReply;
    if (*reply).type_0 == 5 as libc::c_int
        && strcasecmp((*reply).str_0, b"ok\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
    {
        printf(b"\x1B[0;32mPASSED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(b"\x1B[0;31mFAILED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
        fails += 1;
        fails;
    }
    freeReplyObject(reply as *mut libc::c_void);
    tests += 1;
    printf(b"#%02d \0" as *const u8 as *const libc::c_char, tests);
    printf(b"%%s String interpolation works: \0" as *const u8 as *const libc::c_char);
    reply = redisCommand(
        c,
        b"SET %s %s\0" as *const u8 as *const libc::c_char,
        b"foo\0" as *const u8 as *const libc::c_char,
        b"hello world\0" as *const u8 as *const libc::c_char,
    ) as *mut redisReply;
    freeReplyObject(reply as *mut libc::c_void);
    reply = redisCommand(c, b"GET foo\0" as *const u8 as *const libc::c_char)
        as *mut redisReply;
    if (*reply).type_0 == 1 as libc::c_int
        && strcmp((*reply).str_0, b"hello world\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
    {
        printf(b"\x1B[0;32mPASSED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(b"\x1B[0;31mFAILED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
        fails += 1;
        fails;
    }
    freeReplyObject(reply as *mut libc::c_void);
    tests += 1;
    printf(b"#%02d \0" as *const u8 as *const libc::c_char, tests);
    printf(b"%%b String interpolation works: \0" as *const u8 as *const libc::c_char);
    reply = redisCommand(
        c,
        b"SET %b %b\0" as *const u8 as *const libc::c_char,
        b"foo\0" as *const u8 as *const libc::c_char,
        3 as libc::c_int as size_t,
        b"hello\0world\0" as *const u8 as *const libc::c_char,
        11 as libc::c_int as size_t,
    ) as *mut redisReply;
    freeReplyObject(reply as *mut libc::c_void);
    reply = redisCommand(c, b"GET foo\0" as *const u8 as *const libc::c_char)
        as *mut redisReply;
    if (*reply).type_0 == 1 as libc::c_int
        && memcmp(
            (*reply).str_0 as *const libc::c_void,
            b"hello\0world\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            11 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
    {
        printf(b"\x1B[0;32mPASSED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(b"\x1B[0;31mFAILED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
        fails += 1;
        fails;
    }
    tests += 1;
    printf(b"#%02d \0" as *const u8 as *const libc::c_char, tests);
    printf(b"Binary reply length is correct: \0" as *const u8 as *const libc::c_char);
    if (*reply).len == 11 as libc::c_int as libc::c_ulong {
        printf(b"\x1B[0;32mPASSED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(b"\x1B[0;31mFAILED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
        fails += 1;
        fails;
    }
    freeReplyObject(reply as *mut libc::c_void);
    tests += 1;
    printf(b"#%02d \0" as *const u8 as *const libc::c_char, tests);
    printf(b"Can parse nil replies: \0" as *const u8 as *const libc::c_char);
    reply = redisCommand(c, b"GET nokey\0" as *const u8 as *const libc::c_char)
        as *mut redisReply;
    if (*reply).type_0 == 4 as libc::c_int {
        printf(b"\x1B[0;32mPASSED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(b"\x1B[0;31mFAILED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
        fails += 1;
        fails;
    }
    freeReplyObject(reply as *mut libc::c_void);
    tests += 1;
    printf(b"#%02d \0" as *const u8 as *const libc::c_char, tests);
    printf(b"Can parse integer replies: \0" as *const u8 as *const libc::c_char);
    reply = redisCommand(c, b"INCR mycounter\0" as *const u8 as *const libc::c_char)
        as *mut redisReply;
    if (*reply).type_0 == 3 as libc::c_int
        && (*reply).integer == 1 as libc::c_int as libc::c_longlong
    {
        printf(b"\x1B[0;32mPASSED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(b"\x1B[0;31mFAILED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
        fails += 1;
        fails;
    }
    freeReplyObject(reply as *mut libc::c_void);
    tests += 1;
    printf(b"#%02d \0" as *const u8 as *const libc::c_char, tests);
    printf(b"Can parse multi bulk replies: \0" as *const u8 as *const libc::c_char);
    freeReplyObject(
        redisCommand(c, b"LPUSH mylist foo\0" as *const u8 as *const libc::c_char),
    );
    freeReplyObject(
        redisCommand(c, b"LPUSH mylist bar\0" as *const u8 as *const libc::c_char),
    );
    reply = redisCommand(c, b"LRANGE mylist 0 -1\0" as *const u8 as *const libc::c_char)
        as *mut redisReply;
    if (*reply).type_0 == 2 as libc::c_int
        && (*reply).elements == 2 as libc::c_int as libc::c_ulong
        && memcmp(
            (**((*reply).element).offset(0 as libc::c_int as isize)).str_0
                as *const libc::c_void,
            b"bar\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            3 as libc::c_int as libc::c_ulong,
        ) == 0
        && memcmp(
            (**((*reply).element).offset(1 as libc::c_int as isize)).str_0
                as *const libc::c_void,
            b"foo\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            3 as libc::c_int as libc::c_ulong,
        ) == 0
    {
        printf(b"\x1B[0;32mPASSED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(b"\x1B[0;31mFAILED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
        fails += 1;
        fails;
    }
    freeReplyObject(reply as *mut libc::c_void);
    tests += 1;
    printf(b"#%02d \0" as *const u8 as *const libc::c_char, tests);
    printf(
        b"Can handle nested multi bulk replies: \0" as *const u8 as *const libc::c_char,
    );
    freeReplyObject(redisCommand(c, b"MULTI\0" as *const u8 as *const libc::c_char));
    freeReplyObject(
        redisCommand(c, b"LRANGE mylist 0 -1\0" as *const u8 as *const libc::c_char),
    );
    freeReplyObject(redisCommand(c, b"PING\0" as *const u8 as *const libc::c_char));
    reply = redisCommand(c, b"EXEC\0" as *const u8 as *const libc::c_char)
        as *mut redisReply;
    if (*reply).type_0 == 2 as libc::c_int
        && (*reply).elements == 2 as libc::c_int as libc::c_ulong
        && (**((*reply).element).offset(0 as libc::c_int as isize)).type_0
            == 2 as libc::c_int
        && (**((*reply).element).offset(0 as libc::c_int as isize)).elements
            == 2 as libc::c_int as libc::c_ulong
        && memcmp(
            (**((**((*reply).element).offset(0 as libc::c_int as isize)).element)
                .offset(0 as libc::c_int as isize))
                .str_0 as *const libc::c_void,
            b"bar\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            3 as libc::c_int as libc::c_ulong,
        ) == 0
        && memcmp(
            (**((**((*reply).element).offset(0 as libc::c_int as isize)).element)
                .offset(1 as libc::c_int as isize))
                .str_0 as *const libc::c_void,
            b"foo\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            3 as libc::c_int as libc::c_ulong,
        ) == 0
        && (**((*reply).element).offset(1 as libc::c_int as isize)).type_0
            == 5 as libc::c_int
        && strcasecmp(
            (**((*reply).element).offset(1 as libc::c_int as isize)).str_0,
            b"pong\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
    {
        printf(b"\x1B[0;32mPASSED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(b"\x1B[0;31mFAILED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
        fails += 1;
        fails;
    }
    freeReplyObject(reply as *mut libc::c_void);
    tests += 1;
    printf(b"#%02d \0" as *const u8 as *const libc::c_char, tests);
    printf(b"Send command by passing argc/argv: \0" as *const u8 as *const libc::c_char);
    let mut argv: [*const libc::c_char; 3] = [
        b"SET\0" as *const u8 as *const libc::c_char,
        b"foo\0" as *const u8 as *const libc::c_char,
        b"bar\0" as *const u8 as *const libc::c_char,
    ];
    let mut argvlen: [size_t; 3] = [
        3 as libc::c_int as size_t,
        3 as libc::c_int as size_t,
        3 as libc::c_int as size_t,
    ];
    reply = redisCommandArgv(
        c,
        3 as libc::c_int,
        argv.as_mut_ptr(),
        argvlen.as_mut_ptr(),
    ) as *mut redisReply;
    if (*reply).type_0 == 5 as libc::c_int {
        printf(b"\x1B[0;32mPASSED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(b"\x1B[0;31mFAILED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
        fails += 1;
        fails;
    }
    freeReplyObject(reply as *mut libc::c_void);
    tests += 1;
    printf(b"#%02d \0" as *const u8 as *const libc::c_char, tests);
    printf(b"Can pass NULL to redisGetReply: \0" as *const u8 as *const libc::c_char);
    if redisAppendCommand(c, b"PING\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"redisAppendCommand(c, \"PING\") == REDIS_OK\0" as *const u8
                as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            1246 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 45],
                &[libc::c_char; 45],
            >(b"void test_blocking_connection(struct config)\0"))
                .as_ptr(),
        );
    };
    if redisGetReply(c, 0 as *mut *mut libc::c_void) == 0 as libc::c_int {
        printf(b"\x1B[0;32mPASSED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(b"\x1B[0;31mFAILED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
        fails += 1;
        fails;
    }
    get_redis_version(c, &mut major, 0 as *mut libc::c_int);
    if major >= 6 as libc::c_int {
        test_resp3_push_handler(c);
    }
    test_resp3_push_options(config);
    test_privdata_hooks(config);
    disconnect(c, 0 as libc::c_int);
}
unsafe extern "C" fn detect_debug_sleep(mut c: *mut redisContext) -> libc::c_int {
    let mut detected: libc::c_int = 0;
    let mut reply: *mut redisReply = redisCommand(
        c,
        b"DEBUG SLEEP 0\r\n\0" as *const u8 as *const libc::c_char,
    ) as *mut redisReply;
    if reply.is_null() || (*c).err != 0 {
        let mut cause: *const libc::c_char = if (*c).err != 0 {
            ((*c).errstr).as_mut_ptr() as *const libc::c_char
        } else {
            b"(none)\0" as *const u8 as *const libc::c_char
        };
        fprintf(
            stderr,
            b"Error testing for DEBUG SLEEP (Redis error: %s), exiting\n\0" as *const u8
                as *const libc::c_char,
            cause,
        );
        exit(-(1 as libc::c_int));
    }
    detected = ((*reply).type_0 == 5 as libc::c_int) as libc::c_int;
    freeReplyObject(reply as *mut libc::c_void);
    return detected;
}
unsafe extern "C" fn test_blocking_connection_timeouts(mut config: config) {
    let mut c: *mut redisContext = 0 as *mut redisContext;
    let mut reply: *mut redisReply = 0 as *mut redisReply;
    let mut s: ssize_t = 0;
    let mut sleep_cmd: *const libc::c_char = b"DEBUG SLEEP 3\r\n\0" as *const u8
        as *const libc::c_char;
    let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    c = do_connect(config);
    tests += 1;
    printf(b"#%02d \0" as *const u8 as *const libc::c_char, tests);
    printf(
        b"Successfully completes a command when the timeout is not exceeded: \0"
            as *const u8 as *const libc::c_char,
    );
    reply = redisCommand(c, b"SET foo fast\0" as *const u8 as *const libc::c_char)
        as *mut redisReply;
    freeReplyObject(reply as *mut libc::c_void);
    tv.tv_sec = 0 as libc::c_int as __time_t;
    tv.tv_usec = 10000 as libc::c_int as __suseconds_t;
    redisSetTimeout(c, tv);
    reply = redisCommand(c, b"GET foo\0" as *const u8 as *const libc::c_char)
        as *mut redisReply;
    if !reply.is_null() && (*reply).type_0 == 1 as libc::c_int
        && memcmp(
            (*reply).str_0 as *const libc::c_void,
            b"fast\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            4 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
    {
        printf(b"\x1B[0;32mPASSED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(b"\x1B[0;31mFAILED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
        fails += 1;
        fails;
    }
    freeReplyObject(reply as *mut libc::c_void);
    disconnect(c, 0 as libc::c_int);
    c = do_connect(config);
    tests += 1;
    printf(b"#%02d \0" as *const u8 as *const libc::c_char, tests);
    printf(
        b"Does not return a reply when the command times out: \0" as *const u8
            as *const libc::c_char,
    );
    if detect_debug_sleep(c) != 0 {
        redisAppendFormattedCommand(c, sleep_cmd, strlen(sleep_cmd));
        s = ((*(*c).funcs).write).unwrap()(c);
        if s == sdslen((*c).obuf) as ssize_t {} else {
            __assert_fail(
                b"s == (ssize_t)sdslen(c->obuf)\0" as *const u8 as *const libc::c_char,
                b"test.c\0" as *const u8 as *const libc::c_char,
                1301 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 54],
                    &[libc::c_char; 54],
                >(b"void test_blocking_connection_timeouts(struct config)\0"))
                    .as_ptr(),
            );
        };
        sdsfree((*c).obuf);
        (*c).obuf = sdsempty();
        tv.tv_sec = 0 as libc::c_int as __time_t;
        tv.tv_usec = 10000 as libc::c_int as __suseconds_t;
        redisSetTimeout(c, tv);
        reply = redisCommand(c, b"GET foo\0" as *const u8 as *const libc::c_char)
            as *mut redisReply;
        if s > 0 as libc::c_int as libc::c_long && reply.is_null()
            && (*c).err == 1 as libc::c_int
            && strcmp(
                ((*c).errstr).as_mut_ptr(),
                b"Resource temporarily unavailable\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
        {
            printf(b"\x1B[0;32mPASSED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
        } else {
            printf(b"\x1B[0;31mFAILED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
            fails += 1;
            fails;
        }
        freeReplyObject(reply as *mut libc::c_void);
        millisleep(3000 as libc::c_int);
    } else {
        printf(b"\x1B[01;33mSKIPPED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
        skips += 1;
        skips;
    }
    tests += 1;
    printf(b"#%02d \0" as *const u8 as *const libc::c_char, tests);
    printf(
        b"Reconnect properly reconnects after a timeout: \0" as *const u8
            as *const libc::c_char,
    );
    do_reconnect(c, config);
    reply = redisCommand(c, b"PING\0" as *const u8 as *const libc::c_char)
        as *mut redisReply;
    if !reply.is_null() && (*reply).type_0 == 5 as libc::c_int
        && strcmp((*reply).str_0, b"PONG\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
    {
        printf(b"\x1B[0;32mPASSED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(b"\x1B[0;31mFAILED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
        fails += 1;
        fails;
    }
    freeReplyObject(reply as *mut libc::c_void);
    tests += 1;
    printf(b"#%02d \0" as *const u8 as *const libc::c_char, tests);
    printf(
        b"Reconnect properly uses owned parameters: \0" as *const u8
            as *const libc::c_char,
    );
    config.tcp.host = b"foo\0" as *const u8 as *const libc::c_char;
    config.unix_sock.path = b"foo\0" as *const u8 as *const libc::c_char;
    do_reconnect(c, config);
    reply = redisCommand(c, b"PING\0" as *const u8 as *const libc::c_char)
        as *mut redisReply;
    if !reply.is_null() && (*reply).type_0 == 5 as libc::c_int
        && strcmp((*reply).str_0, b"PONG\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
    {
        printf(b"\x1B[0;32mPASSED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(b"\x1B[0;31mFAILED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
        fails += 1;
        fails;
    }
    freeReplyObject(reply as *mut libc::c_void);
    disconnect(c, 0 as libc::c_int);
}
unsafe extern "C" fn test_blocking_io_errors(mut config: config) {
    let mut c: *mut redisContext = 0 as *mut redisContext;
    let mut reply: *mut redisReply = 0 as *mut redisReply;
    let mut _reply: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut major: libc::c_int = 0;
    let mut minor: libc::c_int = 0;
    c = do_connect(config);
    get_redis_version(c, &mut major, &mut minor);
    tests += 1;
    printf(b"#%02d \0" as *const u8 as *const libc::c_char, tests);
    printf(
        b"Returns I/O error when the connection is lost: \0" as *const u8
            as *const libc::c_char,
    );
    reply = redisCommand(c, b"QUIT\0" as *const u8 as *const libc::c_char)
        as *mut redisReply;
    if major > 2 as libc::c_int || major == 2 as libc::c_int && minor > 0 as libc::c_int
    {
        if strcasecmp((*reply).str_0, b"OK\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int && redisGetReply(c, &mut _reply) == -(1 as libc::c_int)
        {
            printf(b"\x1B[0;32mPASSED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
        } else {
            printf(b"\x1B[0;31mFAILED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
            fails += 1;
            fails;
        }
        freeReplyObject(reply as *mut libc::c_void);
    } else if reply.is_null() {
        printf(b"\x1B[0;32mPASSED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(b"\x1B[0;31mFAILED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
        fails += 1;
        fails;
    }
    if (*c).err == 3 as libc::c_int
        && strcmp(
            ((*c).errstr).as_mut_ptr(),
            b"Server closed the connection\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"c->err == REDIS_ERR_EOF && strcmp(c->errstr,\"Server closed the connection\") == 0\0"
                as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            1370 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 44],
                &[libc::c_char; 44],
            >(b"void test_blocking_io_errors(struct config)\0"))
                .as_ptr(),
        );
    };
    redisFree(c);
    c = do_connect(config);
    tests += 1;
    printf(b"#%02d \0" as *const u8 as *const libc::c_char, tests);
    printf(
        b"Returns I/O error on socket timeout: \0" as *const u8 as *const libc::c_char,
    );
    let mut tv: timeval = {
        let mut init = timeval {
            tv_sec: 0 as libc::c_int as __time_t,
            tv_usec: 1000 as libc::c_int as __suseconds_t,
        };
        init
    };
    if redisSetTimeout(c, tv) == 0 as libc::c_int {} else {
        __assert_fail(
            b"redisSetTimeout(c,tv) == REDIS_OK\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            1377 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 44],
                &[libc::c_char; 44],
            >(b"void test_blocking_io_errors(struct config)\0"))
                .as_ptr(),
        );
    };
    let mut respcode: libc::c_int = redisGetReply(c, &mut _reply);
    if respcode == -(1 as libc::c_int) && (*c).err == 1 as libc::c_int
        && *__errno_location() == 11 as libc::c_int
    {
        printf(b"\x1B[0;32mPASSED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(b"\x1B[0;31mFAILED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
        fails += 1;
        fails;
    }
    redisFree(c);
}
unsafe extern "C" fn test_invalid_timeout_errors(mut config: config) {
    let mut c: *mut redisContext = 0 as *mut redisContext;
    tests += 1;
    printf(b"#%02d \0" as *const u8 as *const libc::c_char, tests);
    printf(
        b"Set error when an invalid timeout usec value is used during connect: \0"
            as *const u8 as *const libc::c_char,
    );
    config.connect_timeout.tv_sec = 0 as libc::c_int as __time_t;
    config.connect_timeout.tv_usec = 10000001 as libc::c_int as __suseconds_t;
    if config.type_0 as libc::c_uint == CONN_TCP as libc::c_int as libc::c_uint
        || config.type_0 as libc::c_uint == CONN_SSL as libc::c_int as libc::c_uint
    {
        c = redisConnectWithTimeout(
            config.tcp.host,
            config.tcp.port,
            config.connect_timeout,
        );
    } else if config.type_0 as libc::c_uint == CONN_UNIX as libc::c_int as libc::c_uint {
        c = redisConnectUnixWithTimeout(config.unix_sock.path, config.connect_timeout);
    } else {
        fprintf(
            stderr,
            b"PANIC: %s (In function \"%s\", file \"%s\", line %d)\n\0" as *const u8
                as *const libc::c_char,
            b"Unknown connection type!\0" as *const u8 as *const libc::c_char,
            (*::std::mem::transmute::<
                &[u8; 28],
                &[libc::c_char; 28],
            >(b"test_invalid_timeout_errors\0"))
                .as_ptr(),
            b"test.c\0" as *const u8 as *const libc::c_char,
            1400 as libc::c_int,
        );
        exit(1 as libc::c_int);
    }
    if !c.is_null() && (*c).err == 1 as libc::c_int
        && strcmp(
            ((*c).errstr).as_mut_ptr(),
            b"Invalid timeout specified\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
    {
        printf(b"\x1B[0;32mPASSED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(b"\x1B[0;31mFAILED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
        fails += 1;
        fails;
    }
    redisFree(c);
    tests += 1;
    printf(b"#%02d \0" as *const u8 as *const libc::c_char, tests);
    printf(
        b"Set error when an invalid timeout sec value is used during connect: \0"
            as *const u8 as *const libc::c_char,
    );
    config
        .connect_timeout
        .tv_sec = (9223372036854775807 as libc::c_long
        - 999 as libc::c_int as libc::c_long) / 1000 as libc::c_int as libc::c_long
        + 1 as libc::c_int as libc::c_long;
    config.connect_timeout.tv_usec = 0 as libc::c_int as __suseconds_t;
    if config.type_0 as libc::c_uint == CONN_TCP as libc::c_int as libc::c_uint
        || config.type_0 as libc::c_uint == CONN_SSL as libc::c_int as libc::c_uint
    {
        c = redisConnectWithTimeout(
            config.tcp.host,
            config.tcp.port,
            config.connect_timeout,
        );
    } else if config.type_0 as libc::c_uint == CONN_UNIX as libc::c_int as libc::c_uint {
        c = redisConnectUnixWithTimeout(config.unix_sock.path, config.connect_timeout);
    } else {
        fprintf(
            stderr,
            b"PANIC: %s (In function \"%s\", file \"%s\", line %d)\n\0" as *const u8
                as *const libc::c_char,
            b"Unknown connection type!\0" as *const u8 as *const libc::c_char,
            (*::std::mem::transmute::<
                &[u8; 28],
                &[libc::c_char; 28],
            >(b"test_invalid_timeout_errors\0"))
                .as_ptr(),
            b"test.c\0" as *const u8 as *const libc::c_char,
            1416 as libc::c_int,
        );
        exit(1 as libc::c_int);
    }
    if !c.is_null() && (*c).err == 1 as libc::c_int
        && strcmp(
            ((*c).errstr).as_mut_ptr(),
            b"Invalid timeout specified\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
    {
        printf(b"\x1B[0;32mPASSED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(b"\x1B[0;31mFAILED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
        fails += 1;
        fails;
    }
    redisFree(c);
}
pub unsafe extern "C" fn hi_malloc_safe(mut size: size_t) -> *mut libc::c_void {
    let mut ptr: *mut libc::c_void = hi_malloc(size);
    if ptr.is_null() {
        fprintf(
            stderr,
            b"Error:  Out of memory\n\0" as *const u8 as *const libc::c_char,
        );
        exit(-(1 as libc::c_int));
    }
    return ptr;
}
unsafe extern "C" fn test_throughput(mut config: config) {
    let mut c: *mut redisContext = do_connect(config);
    let mut replies: *mut *mut redisReply = 0 as *mut *mut redisReply;
    let mut i: libc::c_int = 0;
    let mut num: libc::c_int = 0;
    let mut t1: libc::c_longlong = 0;
    let mut t2: libc::c_longlong = 0;
    tests += 1;
    printf(b"#%02d \0" as *const u8 as *const libc::c_char, tests);
    printf(b"Throughput:\n\0" as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int;
    while i < 500 as libc::c_int {
        freeReplyObject(
            redisCommand(c, b"LPUSH mylist foo\0" as *const u8 as *const libc::c_char),
        );
        i += 1;
        i;
    }
    num = 1000 as libc::c_int;
    replies = hi_malloc_safe(
        (::std::mem::size_of::<*mut redisReply>() as libc::c_ulong)
            .wrapping_mul(num as libc::c_ulong),
    ) as *mut *mut redisReply;
    t1 = usec();
    i = 0 as libc::c_int;
    while i < num {
        let ref mut fresh1 = *replies.offset(i as isize);
        *fresh1 = redisCommand(c, b"PING\0" as *const u8 as *const libc::c_char)
            as *mut redisReply;
        if !(*replies.offset(i as isize)).is_null()
            && (**replies.offset(i as isize)).type_0 == 5 as libc::c_int
        {} else {
            __assert_fail(
                b"replies[i] != NULL && replies[i]->type == REDIS_REPLY_STATUS\0"
                    as *const u8 as *const libc::c_char,
                b"test.c\0" as *const u8 as *const libc::c_char,
                1450 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 36],
                    &[libc::c_char; 36],
                >(b"void test_throughput(struct config)\0"))
                    .as_ptr(),
            );
        };
        i += 1;
        i;
    }
    t2 = usec();
    i = 0 as libc::c_int;
    while i < num {
        freeReplyObject(*replies.offset(i as isize) as *mut libc::c_void);
        i += 1;
        i;
    }
    hi_free(replies as *mut libc::c_void);
    printf(
        b"\t(%dx PING: %.3fs)\n\0" as *const u8 as *const libc::c_char,
        num,
        (t2 - t1) as libc::c_double / 1000000.0f64,
    );
    replies = hi_malloc_safe(
        (::std::mem::size_of::<*mut redisReply>() as libc::c_ulong)
            .wrapping_mul(num as libc::c_ulong),
    ) as *mut *mut redisReply;
    t1 = usec();
    i = 0 as libc::c_int;
    while i < num {
        let ref mut fresh2 = *replies.offset(i as isize);
        *fresh2 = redisCommand(
            c,
            b"LRANGE mylist 0 499\0" as *const u8 as *const libc::c_char,
        ) as *mut redisReply;
        if !(*replies.offset(i as isize)).is_null()
            && (**replies.offset(i as isize)).type_0 == 2 as libc::c_int
        {} else {
            __assert_fail(
                b"replies[i] != NULL && replies[i]->type == REDIS_REPLY_ARRAY\0"
                    as *const u8 as *const libc::c_char,
                b"test.c\0" as *const u8 as *const libc::c_char,
                1461 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 36],
                    &[libc::c_char; 36],
                >(b"void test_throughput(struct config)\0"))
                    .as_ptr(),
            );
        };
        if !(*replies.offset(i as isize)).is_null()
            && (**replies.offset(i as isize)).elements
                == 500 as libc::c_int as libc::c_ulong
        {} else {
            __assert_fail(
                b"replies[i] != NULL && replies[i]->elements == 500\0" as *const u8
                    as *const libc::c_char,
                b"test.c\0" as *const u8 as *const libc::c_char,
                1462 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 36],
                    &[libc::c_char; 36],
                >(b"void test_throughput(struct config)\0"))
                    .as_ptr(),
            );
        };
        i += 1;
        i;
    }
    t2 = usec();
    i = 0 as libc::c_int;
    while i < num {
        freeReplyObject(*replies.offset(i as isize) as *mut libc::c_void);
        i += 1;
        i;
    }
    hi_free(replies as *mut libc::c_void);
    printf(
        b"\t(%dx LRANGE with 500 elements: %.3fs)\n\0" as *const u8
            as *const libc::c_char,
        num,
        (t2 - t1) as libc::c_double / 1000000.0f64,
    );
    replies = hi_malloc_safe(
        (::std::mem::size_of::<*mut redisReply>() as libc::c_ulong)
            .wrapping_mul(num as libc::c_ulong),
    ) as *mut *mut redisReply;
    t1 = usec();
    i = 0 as libc::c_int;
    while i < num {
        let ref mut fresh3 = *replies.offset(i as isize);
        *fresh3 = redisCommand(
            c,
            b"INCRBY incrkey %d\0" as *const u8 as *const libc::c_char,
            1000000 as libc::c_int,
        ) as *mut redisReply;
        if !(*replies.offset(i as isize)).is_null()
            && (**replies.offset(i as isize)).type_0 == 3 as libc::c_int
        {} else {
            __assert_fail(
                b"replies[i] != NULL && replies[i]->type == REDIS_REPLY_INTEGER\0"
                    as *const u8 as *const libc::c_char,
                b"test.c\0" as *const u8 as *const libc::c_char,
                1473 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 36],
                    &[libc::c_char; 36],
                >(b"void test_throughput(struct config)\0"))
                    .as_ptr(),
            );
        };
        i += 1;
        i;
    }
    t2 = usec();
    i = 0 as libc::c_int;
    while i < num {
        freeReplyObject(*replies.offset(i as isize) as *mut libc::c_void);
        i += 1;
        i;
    }
    hi_free(replies as *mut libc::c_void);
    printf(
        b"\t(%dx INCRBY: %.3fs)\n\0" as *const u8 as *const libc::c_char,
        num,
        (t2 - t1) as libc::c_double / 1000000.0f64,
    );
    num = 10000 as libc::c_int;
    replies = hi_malloc_safe(
        (::std::mem::size_of::<*mut redisReply>() as libc::c_ulong)
            .wrapping_mul(num as libc::c_ulong),
    ) as *mut *mut redisReply;
    i = 0 as libc::c_int;
    while i < num {
        redisAppendCommand(c, b"PING\0" as *const u8 as *const libc::c_char);
        i += 1;
        i;
    }
    t1 = usec();
    i = 0 as libc::c_int;
    while i < num {
        if redisGetReply(
            c,
            &mut *replies.offset(i as isize) as *mut *mut redisReply as *mut libc::c_void
                as *mut *mut libc::c_void,
        ) == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"redisGetReply(c, (void*)&replies[i]) == REDIS_OK\0" as *const u8
                    as *const libc::c_char,
                b"test.c\0" as *const u8 as *const libc::c_char,
                1486 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 36],
                    &[libc::c_char; 36],
                >(b"void test_throughput(struct config)\0"))
                    .as_ptr(),
            );
        };
        if !(*replies.offset(i as isize)).is_null()
            && (**replies.offset(i as isize)).type_0 == 5 as libc::c_int
        {} else {
            __assert_fail(
                b"replies[i] != NULL && replies[i]->type == REDIS_REPLY_STATUS\0"
                    as *const u8 as *const libc::c_char,
                b"test.c\0" as *const u8 as *const libc::c_char,
                1487 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 36],
                    &[libc::c_char; 36],
                >(b"void test_throughput(struct config)\0"))
                    .as_ptr(),
            );
        };
        i += 1;
        i;
    }
    t2 = usec();
    i = 0 as libc::c_int;
    while i < num {
        freeReplyObject(*replies.offset(i as isize) as *mut libc::c_void);
        i += 1;
        i;
    }
    hi_free(replies as *mut libc::c_void);
    printf(
        b"\t(%dx PING (pipelined): %.3fs)\n\0" as *const u8 as *const libc::c_char,
        num,
        (t2 - t1) as libc::c_double / 1000000.0f64,
    );
    replies = hi_malloc_safe(
        (::std::mem::size_of::<*mut redisReply>() as libc::c_ulong)
            .wrapping_mul(num as libc::c_ulong),
    ) as *mut *mut redisReply;
    i = 0 as libc::c_int;
    while i < num {
        redisAppendCommand(
            c,
            b"LRANGE mylist 0 499\0" as *const u8 as *const libc::c_char,
        );
        i += 1;
        i;
    }
    t1 = usec();
    i = 0 as libc::c_int;
    while i < num {
        if redisGetReply(
            c,
            &mut *replies.offset(i as isize) as *mut *mut redisReply as *mut libc::c_void
                as *mut *mut libc::c_void,
        ) == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"redisGetReply(c, (void*)&replies[i]) == REDIS_OK\0" as *const u8
                    as *const libc::c_char,
                b"test.c\0" as *const u8 as *const libc::c_char,
                1499 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 36],
                    &[libc::c_char; 36],
                >(b"void test_throughput(struct config)\0"))
                    .as_ptr(),
            );
        };
        if !(*replies.offset(i as isize)).is_null()
            && (**replies.offset(i as isize)).type_0 == 2 as libc::c_int
        {} else {
            __assert_fail(
                b"replies[i] != NULL && replies[i]->type == REDIS_REPLY_ARRAY\0"
                    as *const u8 as *const libc::c_char,
                b"test.c\0" as *const u8 as *const libc::c_char,
                1500 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 36],
                    &[libc::c_char; 36],
                >(b"void test_throughput(struct config)\0"))
                    .as_ptr(),
            );
        };
        if !(*replies.offset(i as isize)).is_null()
            && (**replies.offset(i as isize)).elements
                == 500 as libc::c_int as libc::c_ulong
        {} else {
            __assert_fail(
                b"replies[i] != NULL && replies[i]->elements == 500\0" as *const u8
                    as *const libc::c_char,
                b"test.c\0" as *const u8 as *const libc::c_char,
                1501 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 36],
                    &[libc::c_char; 36],
                >(b"void test_throughput(struct config)\0"))
                    .as_ptr(),
            );
        };
        i += 1;
        i;
    }
    t2 = usec();
    i = 0 as libc::c_int;
    while i < num {
        freeReplyObject(*replies.offset(i as isize) as *mut libc::c_void);
        i += 1;
        i;
    }
    hi_free(replies as *mut libc::c_void);
    printf(
        b"\t(%dx LRANGE with 500 elements (pipelined): %.3fs)\n\0" as *const u8
            as *const libc::c_char,
        num,
        (t2 - t1) as libc::c_double / 1000000.0f64,
    );
    replies = hi_malloc_safe(
        (::std::mem::size_of::<*mut redisReply>() as libc::c_ulong)
            .wrapping_mul(num as libc::c_ulong),
    ) as *mut *mut redisReply;
    i = 0 as libc::c_int;
    while i < num {
        redisAppendCommand(
            c,
            b"INCRBY incrkey %d\0" as *const u8 as *const libc::c_char,
            1000000 as libc::c_int,
        );
        i += 1;
        i;
    }
    t1 = usec();
    i = 0 as libc::c_int;
    while i < num {
        if redisGetReply(
            c,
            &mut *replies.offset(i as isize) as *mut *mut redisReply as *mut libc::c_void
                as *mut *mut libc::c_void,
        ) == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"redisGetReply(c, (void*)&replies[i]) == REDIS_OK\0" as *const u8
                    as *const libc::c_char,
                b"test.c\0" as *const u8 as *const libc::c_char,
                1513 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 36],
                    &[libc::c_char; 36],
                >(b"void test_throughput(struct config)\0"))
                    .as_ptr(),
            );
        };
        if !(*replies.offset(i as isize)).is_null()
            && (**replies.offset(i as isize)).type_0 == 3 as libc::c_int
        {} else {
            __assert_fail(
                b"replies[i] != NULL && replies[i]->type == REDIS_REPLY_INTEGER\0"
                    as *const u8 as *const libc::c_char,
                b"test.c\0" as *const u8 as *const libc::c_char,
                1514 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 36],
                    &[libc::c_char; 36],
                >(b"void test_throughput(struct config)\0"))
                    .as_ptr(),
            );
        };
        i += 1;
        i;
    }
    t2 = usec();
    i = 0 as libc::c_int;
    while i < num {
        freeReplyObject(*replies.offset(i as isize) as *mut libc::c_void);
        i += 1;
        i;
    }
    hi_free(replies as *mut libc::c_void);
    printf(
        b"\t(%dx INCRBY (pipelined): %.3fs)\n\0" as *const u8 as *const libc::c_char,
        num,
        (t2 - t1) as libc::c_double / 1000000.0f64,
    );
    disconnect(c, 0 as libc::c_int);
}
static mut astest: _astest = _astest {
    ac: 0 as *const redisAsyncContext as *mut redisAsyncContext,
    testno: ASTEST_CONNECT,
    counter: 0,
    connects: 0,
    connect_status: 0,
    disconnects: 0,
    pongs: 0,
    disconnect_status: 0,
    connected: 0,
    err: 0,
    errstr: [0; 256],
};
unsafe extern "C" fn asCleanup(mut data: *mut libc::c_void) {
    let mut t: *mut _astest = data as *mut _astest;
    (*t).ac = 0 as *mut redisAsyncContext;
}
unsafe extern "C" fn connectCallback(
    mut c: *mut redisAsyncContext,
    mut status: libc::c_int,
) {
    let mut t: *mut _astest = (*c).data as *mut _astest;
    if t == &mut astest as *mut _astest {} else {
        __assert_fail(
            b"t == &astest\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            2107 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 47],
                &[libc::c_char; 47],
            >(b"void connectCallback(redisAsyncContext *, int)\0"))
                .as_ptr(),
        );
    };
    if (*t).connects == 0 as libc::c_int {} else {
        __assert_fail(
            b"t->connects == 0\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            2108 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 47],
                &[libc::c_char; 47],
            >(b"void connectCallback(redisAsyncContext *, int)\0"))
                .as_ptr(),
        );
    };
    (*t).err = (*c).err;
    strcpy(((*t).errstr).as_mut_ptr(), (*c).errstr);
    (*t).connects += 1;
    (*t).connects;
    (*t).connect_status = status;
    (*t)
        .connected = if status == 0 as libc::c_int {
        1 as libc::c_int
    } else {
        -(1 as libc::c_int)
    };
    if (*t).testno as libc::c_uint == ASTEST_ISSUE_931 as libc::c_int as libc::c_uint {
        redisAsyncDisconnect(c);
    } else if (*t).testno as libc::c_uint
        == ASTEST_ISSUE_931_PING as libc::c_int as libc::c_uint
    {
        redisAsyncCommand(
            c,
            Some(
                commandCallback
                    as unsafe extern "C" fn(
                        *mut redisAsyncContext,
                        *mut libc::c_void,
                        *mut libc::c_void,
                    ) -> (),
            ),
            0 as *mut libc::c_void,
            b"PING\0" as *const u8 as *const libc::c_char,
        );
    }
}
unsafe extern "C" fn disconnectCallback(
    mut c: *const redisAsyncContext,
    mut status: libc::c_int,
) {
    if (*c).data == &mut astest as *mut _astest as *mut libc::c_void {} else {
        __assert_fail(
            b"c->data == (void*)&astest\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            2125 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 56],
                &[libc::c_char; 56],
            >(b"void disconnectCallback(const redisAsyncContext *, int)\0"))
                .as_ptr(),
        );
    };
    if astest.disconnects == 0 as libc::c_int {} else {
        __assert_fail(
            b"astest.disconnects == 0\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            2126 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 56],
                &[libc::c_char; 56],
            >(b"void disconnectCallback(const redisAsyncContext *, int)\0"))
                .as_ptr(),
        );
    };
    astest.err = (*c).err;
    strcpy((astest.errstr).as_mut_ptr(), (*c).errstr);
    astest.disconnects += 1;
    astest.disconnects;
    astest.disconnect_status = status;
    astest.connected = 0 as libc::c_int;
}
unsafe extern "C" fn commandCallback(
    mut ac: *mut redisAsyncContext,
    mut _reply: *mut libc::c_void,
    mut _privdata: *mut libc::c_void,
) {
    let mut reply: *mut redisReply = _reply as *mut redisReply;
    let mut t: *mut _astest = (*ac).data as *mut _astest;
    if t == &mut astest as *mut _astest {} else {
        __assert_fail(
            b"t == &astest\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            2138 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 65],
                &[libc::c_char; 65],
            >(b"void commandCallback(struct redisAsyncContext *, void *, void *)\0"))
                .as_ptr(),
        );
    };
    (*t).err = (*ac).err;
    strcpy(((*t).errstr).as_mut_ptr(), (*ac).errstr);
    (*t).counter += 1;
    (*t).counter;
    if (*t).testno as libc::c_uint == ASTEST_PINGPONG as libc::c_int as libc::c_uint
        || (*t).testno as libc::c_uint
            == ASTEST_ISSUE_931_PING as libc::c_int as libc::c_uint
    {
        if !reply.is_null() && (*reply).type_0 == 5 as libc::c_int
            && strcmp((*reply).str_0, b"PONG\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"reply != NULL && reply->type == REDIS_REPLY_STATUS && strcmp(reply->str, \"PONG\") == 0\0"
                    as *const u8 as *const libc::c_char,
                b"test.c\0" as *const u8 as *const libc::c_char,
                2145 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 65],
                    &[libc::c_char; 65],
                >(b"void commandCallback(struct redisAsyncContext *, void *, void *)\0"))
                    .as_ptr(),
            );
        };
        (*t).pongs += 1;
        (*t).pongs;
        redisAsyncFree(ac);
    }
    if (*t).testno as libc::c_uint
        == ASTEST_PINGPONG_TIMEOUT as libc::c_int as libc::c_uint
    {
        if !reply.is_null() && (*reply).type_0 == 5 as libc::c_int
            && strcmp((*reply).str_0, b"PONG\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"reply != NULL && reply->type == REDIS_REPLY_STATUS && strcmp(reply->str, \"PONG\") == 0\0"
                    as *const u8 as *const libc::c_char,
                b"test.c\0" as *const u8 as *const libc::c_char,
                2152 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 65],
                    &[libc::c_char; 65],
                >(b"void commandCallback(struct redisAsyncContext *, void *, void *)\0"))
                    .as_ptr(),
            );
        };
        (*t).pongs += 1;
        (*t).pongs;
        if (*t).counter == 1 as libc::c_int {
            let mut status: libc::c_int = redisAsyncCommand(
                ac,
                Some(
                    commandCallback
                        as unsafe extern "C" fn(
                            *mut redisAsyncContext,
                            *mut libc::c_void,
                            *mut libc::c_void,
                        ) -> (),
                ),
                0 as *mut libc::c_void,
                b"PING\0" as *const u8 as *const libc::c_char,
            );
            if status == 0 as libc::c_int {} else {
                __assert_fail(
                    b"status == REDIS_OK\0" as *const u8 as *const libc::c_char,
                    b"test.c\0" as *const u8 as *const libc::c_char,
                    2156 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 65],
                        &[libc::c_char; 65],
                    >(
                        b"void commandCallback(struct redisAsyncContext *, void *, void *)\0",
                    ))
                        .as_ptr(),
                );
            };
        } else {
            redisAsyncFree(ac);
        }
    }
}
unsafe extern "C" fn do_aconnect(
    mut config: config,
    mut testno: astest_no,
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
    memset(
        &mut astest as *mut _astest as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<_astest>() as libc::c_ulong,
    );
    astest.testno = testno;
    astest.disconnect_status = -(2 as libc::c_int);
    astest.connect_status = astest.disconnect_status;
    if config.type_0 as libc::c_uint == CONN_TCP as libc::c_int as libc::c_uint {
        options.type_0 = REDIS_CONN_TCP as libc::c_int;
        options.connect_timeout = &mut config.connect_timeout;
        options.type_0 = REDIS_CONN_TCP as libc::c_int;
        options.endpoint.tcp.ip = config.tcp.host;
        options.endpoint.tcp.port = config.tcp.port;
    } else if config.type_0 as libc::c_uint == CONN_SSL as libc::c_int as libc::c_uint {
        options.type_0 = REDIS_CONN_TCP as libc::c_int;
        options.connect_timeout = &mut config.connect_timeout;
        options.type_0 = REDIS_CONN_TCP as libc::c_int;
        options.endpoint.tcp.ip = config.ssl.host;
        options.endpoint.tcp.port = config.ssl.port;
    } else if config.type_0 as libc::c_uint == CONN_UNIX as libc::c_int as libc::c_uint {
        options.type_0 = REDIS_CONN_UNIX as libc::c_int;
        options.endpoint.unix_socket = config.unix_sock.path;
    } else if config.type_0 as libc::c_uint == CONN_FD as libc::c_int as libc::c_uint {
        options.type_0 = REDIS_CONN_USERFD as libc::c_int;
        let mut dummy_ctx: *mut redisContext = redisConnectUnix(config.unix_sock.path);
        if !dummy_ctx.is_null() {
            let mut fd: redisFD = disconnect(dummy_ctx, 1 as libc::c_int);
            printf(
                b"Connecting to inherited fd %d\n\0" as *const u8 as *const libc::c_char,
                fd,
            );
            options.endpoint.fd = fd;
        }
    }
    let mut c: *mut redisAsyncContext = redisAsyncConnectWithOptions(&mut options);
    if !c.is_null() {} else {
        __assert_fail(
            b"c\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            2193 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 57],
                &[libc::c_char; 57],
            >(b"redisAsyncContext *do_aconnect(struct config, astest_no)\0"))
                .as_ptr(),
        );
    };
    astest.ac = c;
    (*c).data = &mut astest as *mut _astest as *mut libc::c_void;
    (*c).dataCleanup = Some(asCleanup as unsafe extern "C" fn(*mut libc::c_void) -> ());
    redisPollAttach(c);
    redisAsyncSetConnectCallbackNC(
        c,
        Some(
            connectCallback
                as unsafe extern "C" fn(*mut redisAsyncContext, libc::c_int) -> (),
        ),
    );
    redisAsyncSetDisconnectCallback(
        c,
        Some(
            disconnectCallback
                as unsafe extern "C" fn(*const redisAsyncContext, libc::c_int) -> (),
        ),
    );
    return c;
}
unsafe extern "C" fn as_printerr() {
    printf(
        b"Async err %d : %s\n\0" as *const u8 as *const libc::c_char,
        astest.err,
        (astest.errstr).as_mut_ptr(),
    );
}
unsafe extern "C" fn test_async_polling(mut config: config) {
    let mut status: libc::c_int = 0;
    let mut c: *mut redisAsyncContext = 0 as *mut redisAsyncContext;
    let mut defaultconfig: config = config;
    tests += 1;
    printf(b"#%02d \0" as *const u8 as *const libc::c_char, tests);
    printf(b"Async connect: \0" as *const u8 as *const libc::c_char);
    c = do_aconnect(config, ASTEST_CONNECT);
    if !c.is_null() {} else {
        __assert_fail(
            b"c\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            2220 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 39],
                &[libc::c_char; 39],
            >(b"void test_async_polling(struct config)\0"))
                .as_ptr(),
        );
    };
    while astest.connected == 0 as libc::c_int {
        redisPollTick(c, 0.1f64);
    }
    if astest.connects == 1 as libc::c_int {} else {
        __assert_fail(
            b"astest.connects == 1\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            2223 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 39],
                &[libc::c_char; 39],
            >(b"void test_async_polling(struct config)\0"))
                .as_ptr(),
        );
    };
    if !(astest.connect_status == 0 as libc::c_int) {
        as_printerr();
    }
    if astest.connect_status == 0 as libc::c_int {} else {
        __assert_fail(
            b"astest.connect_status == 0\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            2224 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 39],
                &[libc::c_char; 39],
            >(b"void test_async_polling(struct config)\0"))
                .as_ptr(),
        );
    };
    if astest.disconnects == 0 as libc::c_int {} else {
        __assert_fail(
            b"astest.disconnects == 0\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            2225 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 39],
                &[libc::c_char; 39],
            >(b"void test_async_polling(struct config)\0"))
                .as_ptr(),
        );
    };
    if astest.connected == 1 as libc::c_int {
        printf(b"\x1B[0;32mPASSED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(b"\x1B[0;31mFAILED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
        fails += 1;
        fails;
    }
    tests += 1;
    printf(b"#%02d \0" as *const u8 as *const libc::c_char, tests);
    printf(b"Async free after connect: \0" as *const u8 as *const libc::c_char);
    if !(astest.ac).is_null() {} else {
        __assert_fail(
            b"astest.ac != NULL\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            2229 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 39],
                &[libc::c_char; 39],
            >(b"void test_async_polling(struct config)\0"))
                .as_ptr(),
        );
    };
    redisAsyncFree(c);
    if astest.disconnects == 1 as libc::c_int {} else {
        __assert_fail(
            b"astest.disconnects == 1\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            2231 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 39],
                &[libc::c_char; 39],
            >(b"void test_async_polling(struct config)\0"))
                .as_ptr(),
        );
    };
    if (astest.ac).is_null() {} else {
        __assert_fail(
            b"astest.ac == NULL\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            2232 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 39],
                &[libc::c_char; 39],
            >(b"void test_async_polling(struct config)\0"))
                .as_ptr(),
        );
    };
    if astest.disconnect_status == 0 as libc::c_int {
        printf(b"\x1B[0;32mPASSED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(b"\x1B[0;31mFAILED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
        fails += 1;
        fails;
    }
    if config.type_0 as libc::c_uint == CONN_TCP as libc::c_int as libc::c_uint
        || config.type_0 as libc::c_uint == CONN_SSL as libc::c_int as libc::c_uint
    {
        tests += 1;
        printf(b"#%02d \0" as *const u8 as *const libc::c_char, tests);
        printf(b"Async connect timeout: \0" as *const u8 as *const libc::c_char);
        config.tcp.host = b"192.168.254.254\0" as *const u8 as *const libc::c_char;
        config.connect_timeout.tv_usec = 100000 as libc::c_int as __suseconds_t;
        c = do_aconnect(config, ASTEST_CONN_TIMEOUT);
        if !c.is_null() {} else {
            __assert_fail(
                b"c\0" as *const u8 as *const libc::c_char,
                b"test.c\0" as *const u8 as *const libc::c_char,
                2241 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 39],
                    &[libc::c_char; 39],
                >(b"void test_async_polling(struct config)\0"))
                    .as_ptr(),
            );
        };
        if (*c).err == 0 as libc::c_int {} else {
            __assert_fail(
                b"c->err == 0\0" as *const u8 as *const libc::c_char,
                b"test.c\0" as *const u8 as *const libc::c_char,
                2242 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 39],
                    &[libc::c_char; 39],
                >(b"void test_async_polling(struct config)\0"))
                    .as_ptr(),
            );
        };
        while astest.connected == 0 as libc::c_int {
            redisPollTick(c, 0.1f64);
        }
        if astest.connected == -(1 as libc::c_int) {} else {
            __assert_fail(
                b"astest.connected == -1\0" as *const u8 as *const libc::c_char,
                b"test.c\0" as *const u8 as *const libc::c_char,
                2245 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 39],
                    &[libc::c_char; 39],
                >(b"void test_async_polling(struct config)\0"))
                    .as_ptr(),
            );
        };
        if (astest.ac).is_null() {} else {
            __assert_fail(
                b"astest.ac == NULL\0" as *const u8 as *const libc::c_char,
                b"test.c\0" as *const u8 as *const libc::c_char,
                2250 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 39],
                    &[libc::c_char; 39],
                >(b"void test_async_polling(struct config)\0"))
                    .as_ptr(),
            );
        };
        if astest.connect_status == -(1 as libc::c_int) {
            printf(b"\x1B[0;32mPASSED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
        } else {
            printf(b"\x1B[0;31mFAILED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
            fails += 1;
            fails;
        }
        config = defaultconfig;
    }
    tests += 1;
    printf(b"#%02d \0" as *const u8 as *const libc::c_char, tests);
    printf(b"Async PING/PONG: \0" as *const u8 as *const libc::c_char);
    c = do_aconnect(config, ASTEST_PINGPONG);
    while astest.connected == 0 as libc::c_int {
        redisPollTick(c, 0.1f64);
    }
    status = redisAsyncCommand(
        c,
        Some(
            commandCallback
                as unsafe extern "C" fn(
                    *mut redisAsyncContext,
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> (),
        ),
        0 as *mut libc::c_void,
        b"PING\0" as *const u8 as *const libc::c_char,
    );
    if status == 0 as libc::c_int {} else {
        __assert_fail(
            b"status == REDIS_OK\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            2261 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 39],
                &[libc::c_char; 39],
            >(b"void test_async_polling(struct config)\0"))
                .as_ptr(),
        );
    };
    while !(astest.ac).is_null() {
        redisPollTick(c, 0.1f64);
    }
    if astest.pongs == 1 as libc::c_int {
        printf(b"\x1B[0;32mPASSED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(b"\x1B[0;31mFAILED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
        fails += 1;
        fails;
    }
    if config.type_0 as libc::c_uint == CONN_TCP as libc::c_int as libc::c_uint
        || config.type_0 as libc::c_uint == CONN_SSL as libc::c_int as libc::c_uint
    {
        tests += 1;
        printf(b"#%02d \0" as *const u8 as *const libc::c_char, tests);
        printf(
            b"Async PING/PONG after connect timeout: \0" as *const u8
                as *const libc::c_char,
        );
        config.connect_timeout.tv_usec = 10000 as libc::c_int as __suseconds_t;
        c = do_aconnect(config, ASTEST_PINGPONG_TIMEOUT);
        while astest.connected == 0 as libc::c_int {
            redisPollTick(c, 0.1f64);
        }
        millisleep(10 as libc::c_int);
        status = redisAsyncCommand(
            c,
            Some(
                commandCallback
                    as unsafe extern "C" fn(
                        *mut redisAsyncContext,
                        *mut libc::c_void,
                        *mut libc::c_void,
                    ) -> (),
            ),
            0 as *mut libc::c_void,
            b"PING\0" as *const u8 as *const libc::c_char,
        );
        if status == 0 as libc::c_int {} else {
            __assert_fail(
                b"status == REDIS_OK\0" as *const u8 as *const libc::c_char,
                b"test.c\0" as *const u8 as *const libc::c_char,
                2278 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 39],
                    &[libc::c_char; 39],
                >(b"void test_async_polling(struct config)\0"))
                    .as_ptr(),
            );
        };
        while !(astest.ac).is_null() {
            redisPollTick(c, 0.1f64);
        }
        if astest.pongs == 2 as libc::c_int {
            printf(b"\x1B[0;32mPASSED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
        } else {
            printf(b"\x1B[0;31mFAILED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
            fails += 1;
            fails;
        }
        config = defaultconfig;
    }
    tests += 1;
    printf(b"#%02d \0" as *const u8 as *const libc::c_char, tests);
    printf(
        b"Disconnect from onConnected callback (Issue #931): \0" as *const u8
            as *const libc::c_char,
    );
    c = do_aconnect(config, ASTEST_ISSUE_931);
    while astest.disconnects == 0 as libc::c_int {
        redisPollTick(c, 0.1f64);
    }
    if astest.connected == 0 as libc::c_int {} else {
        __assert_fail(
            b"astest.connected == 0\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            2292 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 39],
                &[libc::c_char; 39],
            >(b"void test_async_polling(struct config)\0"))
                .as_ptr(),
        );
    };
    if astest.connects == 1 as libc::c_int {} else {
        __assert_fail(
            b"astest.connects == 1\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            2293 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 39],
                &[libc::c_char; 39],
            >(b"void test_async_polling(struct config)\0"))
                .as_ptr(),
        );
    };
    if astest.disconnects == 1 as libc::c_int {
        printf(b"\x1B[0;32mPASSED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(b"\x1B[0;31mFAILED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
        fails += 1;
        fails;
    }
    tests += 1;
    printf(b"#%02d \0" as *const u8 as *const libc::c_char, tests);
    printf(
        b"Ping/Pong from onConnected callback (Issue #931): \0" as *const u8
            as *const libc::c_char,
    );
    c = do_aconnect(config, ASTEST_ISSUE_931_PING);
    while !(astest.ac).is_null() {
        redisPollTick(c, 0.1f64);
    }
    if astest.connected == 0 as libc::c_int {} else {
        __assert_fail(
            b"astest.connected == 0\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            2304 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 39],
                &[libc::c_char; 39],
            >(b"void test_async_polling(struct config)\0"))
                .as_ptr(),
        );
    };
    if astest.connects == 1 as libc::c_int {} else {
        __assert_fail(
            b"astest.connects == 1\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            2305 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 39],
                &[libc::c_char; 39],
            >(b"void test_async_polling(struct config)\0"))
                .as_ptr(),
        );
    };
    if astest.disconnects == 1 as libc::c_int {} else {
        __assert_fail(
            b"astest.disconnects == 1\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            2306 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 39],
                &[libc::c_char; 39],
            >(b"void test_async_polling(struct config)\0"))
                .as_ptr(),
        );
    };
    if astest.pongs == 1 as libc::c_int {
        printf(b"\x1B[0;32mPASSED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(b"\x1B[0;31mFAILED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
        fails += 1;
        fails;
    };
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut cfg: config = {
        let mut init = config {
            type_0: CONN_TCP,
            connect_timeout: timeval { tv_sec: 0, tv_usec: 0 },
            tcp: {
                let mut init = C2RustUnnamed_7 {
                    host: b"127.0.0.1\0" as *const u8 as *const libc::c_char,
                    port: 6379 as libc::c_int,
                };
                init
            },
            unix_sock: {
                let mut init = C2RustUnnamed_6 {
                    path: b"/tmp/redis.sock\0" as *const u8 as *const libc::c_char,
                };
                init
            },
            ssl: C2RustUnnamed_5 {
                host: 0 as *const libc::c_char,
                port: 0,
                ca_cert: 0 as *const libc::c_char,
                cert: 0 as *const libc::c_char,
                key: 0 as *const libc::c_char,
            },
        };
        init
    };
    let mut throughput: libc::c_int = 1 as libc::c_int;
    let mut test_inherit_fd: libc::c_int = 1 as libc::c_int;
    let mut skips_as_fails: libc::c_int = 0 as libc::c_int;
    let mut test_unix_socket: libc::c_int = 0;
    argv = argv.offset(1);
    argv;
    argc -= 1;
    argc;
    while argc != 0 {
        if argc >= 2 as libc::c_int
            && strcmp(
                *argv.offset(0 as libc::c_int as isize),
                b"-h\0" as *const u8 as *const libc::c_char,
            ) == 0
        {
            argv = argv.offset(1);
            argv;
            argc -= 1;
            argc;
            cfg.tcp.host = *argv.offset(0 as libc::c_int as isize);
        } else if argc >= 2 as libc::c_int
            && strcmp(
                *argv.offset(0 as libc::c_int as isize),
                b"-p\0" as *const u8 as *const libc::c_char,
            ) == 0
        {
            argv = argv.offset(1);
            argv;
            argc -= 1;
            argc;
            cfg.tcp.port = atoi(*argv.offset(0 as libc::c_int as isize));
        } else if argc >= 2 as libc::c_int
            && strcmp(
                *argv.offset(0 as libc::c_int as isize),
                b"-s\0" as *const u8 as *const libc::c_char,
            ) == 0
        {
            argv = argv.offset(1);
            argv;
            argc -= 1;
            argc;
            cfg.unix_sock.path = *argv.offset(0 as libc::c_int as isize);
        } else if argc >= 1 as libc::c_int
            && strcmp(
                *argv.offset(0 as libc::c_int as isize),
                b"--skip-throughput\0" as *const u8 as *const libc::c_char,
            ) == 0
        {
            throughput = 0 as libc::c_int;
        } else if argc >= 1 as libc::c_int
            && strcmp(
                *argv.offset(0 as libc::c_int as isize),
                b"--skip-inherit-fd\0" as *const u8 as *const libc::c_char,
            ) == 0
        {
            test_inherit_fd = 0 as libc::c_int;
        } else if argc >= 1 as libc::c_int
            && strcmp(
                *argv.offset(0 as libc::c_int as isize),
                b"--skips-as-fails\0" as *const u8 as *const libc::c_char,
            ) == 0
        {
            skips_as_fails = 1 as libc::c_int;
        } else {
            fprintf(
                stderr,
                b"Invalid argument: %s\n\0" as *const u8 as *const libc::c_char,
                *argv.offset(0 as libc::c_int as isize),
            );
            exit(1 as libc::c_int);
        }
        argv = argv.offset(1);
        argv;
        argc -= 1;
        argc;
    }
    signal(
        13 as libc::c_int,
        ::std::mem::transmute::<
            libc::intptr_t,
            __sighandler_t,
        >(1 as libc::c_int as libc::intptr_t),
    );
    test_unix_socket = (access(cfg.unix_sock.path, 0 as libc::c_int) == 0 as libc::c_int)
        as libc::c_int;
    test_allocator_injection();
    test_format_commands();
    test_reply_reader();
    test_blocking_connection_errors();
    test_free_null();
    printf(
        b"\nTesting against TCP connection (%s:%d):\n\0" as *const u8
            as *const libc::c_char,
        cfg.tcp.host,
        cfg.tcp.port,
    );
    cfg.type_0 = CONN_TCP;
    test_blocking_connection(cfg);
    test_blocking_connection_timeouts(cfg);
    test_blocking_io_errors(cfg);
    test_invalid_timeout_errors(cfg);
    test_append_formatted_commands(cfg);
    test_tcp_options(cfg);
    if throughput != 0 {
        test_throughput(cfg);
    }
    printf(
        b"\nTesting against Unix socket connection (%s): \0" as *const u8
            as *const libc::c_char,
        cfg.unix_sock.path,
    );
    if test_unix_socket != 0 {
        printf(b"\n\0" as *const u8 as *const libc::c_char);
        cfg.type_0 = CONN_UNIX;
        test_blocking_connection(cfg);
        test_blocking_connection_timeouts(cfg);
        test_blocking_io_errors(cfg);
        test_invalid_timeout_errors(cfg);
        test_unix_keepalive(cfg);
        if throughput != 0 {
            test_throughput(cfg);
        }
    } else {
        printf(b"\x1B[01;33mSKIPPED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char);
        skips += 1;
        skips;
    }
    cfg.type_0 = CONN_TCP;
    printf(
        b"\nTesting asynchronous API using polling_adapter TCP (%s:%d):\n\0" as *const u8
            as *const libc::c_char,
        cfg.tcp.host,
        cfg.tcp.port,
    );
    test_async_polling(cfg);
    if test_unix_socket != 0 {
        cfg.type_0 = CONN_UNIX;
        printf(
            b"\nTesting asynchronous API using polling_adapter UNIX (%s):\n\0"
                as *const u8 as *const libc::c_char,
            cfg.unix_sock.path,
        );
        test_async_polling(cfg);
    }
    if test_inherit_fd != 0 {
        printf(
            b"\nTesting against inherited fd (%s): \0" as *const u8
                as *const libc::c_char,
            cfg.unix_sock.path,
        );
        if test_unix_socket != 0 {
            printf(b"\n\0" as *const u8 as *const libc::c_char);
            cfg.type_0 = CONN_FD;
            test_blocking_connection(cfg);
        } else {
            printf(
                b"\x1B[01;33mSKIPPED\x1B[0;0m\n\0" as *const u8 as *const libc::c_char,
            );
            skips += 1;
            skips;
        }
    }
    if fails != 0 || skips_as_fails != 0 && skips != 0 {
        printf(
            b"*** %d TESTS FAILED ***\n\0" as *const u8 as *const libc::c_char,
            fails,
        );
        if skips != 0 {
            printf(
                b"*** %d TESTS SKIPPED ***\n\0" as *const u8 as *const libc::c_char,
                skips,
            );
        }
        return 1 as libc::c_int;
    }
    printf(
        b"ALL TESTS PASSED (%d skipped)\n\0" as *const u8 as *const libc::c_char,
        skips,
    );
    return 0 as libc::c_int;
}
pub fn main() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args_os() {
        args.push(
            (::std::ffi::CString::new(
                ::std::os::unix::ffi::OsStrExt::as_bytes(arg.as_os_str()),
            ))
                .unwrap()
                .into_raw(),
        );
    }
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as libc::c_int,
                args.as_mut_ptr() as *mut *mut libc::c_char,
            ) as i32,
        )
    }
}
