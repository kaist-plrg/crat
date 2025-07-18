use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type sockaddr_x25;
    pub type sockaddr_un;
    pub type sockaddr_ns;
    pub type sockaddr_iso;
    pub type sockaddr_ipx;
    pub type sockaddr_inarp;
    pub type sockaddr_eon;
    pub type sockaddr_dl;
    pub type sockaddr_ax25;
    pub type sockaddr_at;
    pub type event_base;
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type evbuffer;
    pub type dict;
    pub type sockadr;
    pub type pool;
    pub type acl;
    pub type cmd;
    pub type http_header;
    fn socket(
        __domain: libc::c_int,
        __type: libc::c_int,
        __protocol: libc::c_int,
    ) -> libc::c_int;
    fn bind(
        __fd: libc::c_int,
        __addr: __CONST_SOCKADDR_ARG,
        __len: socklen_t,
    ) -> libc::c_int;
    fn getsockname(
        __fd: libc::c_int,
        __addr: __SOCKADDR_ARG,
        __len: *mut socklen_t,
    ) -> libc::c_int;
    fn setsockopt(
        __fd: libc::c_int,
        __level: libc::c_int,
        __optname: libc::c_int,
        __optval: *const libc::c_void,
        __optlen: socklen_t,
    ) -> libc::c_int;
    fn listen(__fd: libc::c_int, __n: libc::c_int) -> libc::c_int;
    fn accept(
        __fd: libc::c_int,
        __addr: __SOCKADDR_ARG,
        __addr_len: *mut socklen_t,
    ) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn event_base_new() -> *mut event_base;
    fn event_reinit(base: *mut event_base) -> libc::c_int;
    fn event_base_dispatch(_: *mut event_base) -> libc::c_int;
    fn event_base_set(_: *mut event_base, _: *mut event) -> libc::c_int;
    fn event_add(ev: *mut event, timeout: *const timeval) -> libc::c_int;
    fn event_set(
        _: *mut event,
        _: libc::c_int,
        _: libc::c_short,
        _: Option::<
            unsafe extern "C" fn(libc::c_int, libc::c_short, *mut libc::c_void) -> (),
        >,
        _: *mut libc::c_void,
    );
    fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> libc::c_int;
    fn worker_new(s: *mut server) -> *mut worker;
    fn worker_start(w: *mut worker);
    fn worker_add_client(w: *mut worker, c: *mut http_client);
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn inet_addr(__cp: *const libc::c_char) -> in_addr_t;
    fn http_client_new(
        w: *mut worker,
        fd: libc::c_int,
        addr: in_addr_t,
    ) -> *mut http_client;
    fn conf_read(filename: *const libc::c_char) -> *mut conf;
    fn slog_init(s: *mut server);
    fn slog_fsync_init(s: *mut server);
    fn slog(s: *mut server, level: log_level, body: *const libc::c_char, sz: size_t);
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn dup2(__fd: libc::c_int, __fd2: libc::c_int) -> libc::c_int;
    fn getpid() -> __pid_t;
    fn setsid() -> __pid_t;
    fn fsync(__fd: libc::c_int) -> libc::c_int;
    fn fork() -> __pid_t;
    fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
    fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, _: ...) -> libc::c_int;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
pub type gid_t = __gid_t;
pub type uid_t = __uid_t;
pub type pid_t = __pid_t;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;
pub type int64_t = __int64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
pub type __pthread_list_t = __pthread_internal_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_mutex_s {
    pub __lock: libc::c_int,
    pub __count: libc::c_uint,
    pub __owner: libc::c_int,
    pub __nusers: libc::c_uint,
    pub __kind: libc::c_int,
    pub __spins: libc::c_short,
    pub __elision: libc::c_short,
    pub __list: __pthread_list_t,
}
pub type pthread_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutexattr_t {
    pub __size: [libc::c_char; 4],
    pub __align: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
    pub __align: libc::c_long,
}
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type socklen_t = __socklen_t;
pub type __socket_type = libc::c_uint;
pub const SOCK_NONBLOCK: __socket_type = 2048;
pub const SOCK_CLOEXEC: __socket_type = 524288;
pub const SOCK_PACKET: __socket_type = 10;
pub const SOCK_DCCP: __socket_type = 6;
pub const SOCK_SEQPACKET: __socket_type = 5;
pub const SOCK_RDM: __socket_type = 4;
pub const SOCK_RAW: __socket_type = 3;
pub const SOCK_DGRAM: __socket_type = 2;
pub const SOCK_STREAM: __socket_type = 1;
pub type sa_family_t = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [libc::c_char; 14],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __SOCKADDR_ARG {
    pub __sockaddr__: *mut sockaddr,
    pub __sockaddr_at__: *mut sockaddr_at,
    pub __sockaddr_ax25__: *mut sockaddr_ax25,
    pub __sockaddr_dl__: *mut sockaddr_dl,
    pub __sockaddr_eon__: *mut sockaddr_eon,
    pub __sockaddr_in__: *mut sockaddr_in,
    pub __sockaddr_in6__: *mut sockaddr_in6,
    pub __sockaddr_inarp__: *mut sockaddr_inarp,
    pub __sockaddr_ipx__: *mut sockaddr_ipx,
    pub __sockaddr_iso__: *mut sockaddr_iso,
    pub __sockaddr_ns__: *mut sockaddr_ns,
    pub __sockaddr_un__: *mut sockaddr_un,
    pub __sockaddr_x25__: *mut sockaddr_x25,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in6 {
    pub sin6_family: sa_family_t,
    pub sin6_port: in_port_t,
    pub sin6_flowinfo: uint32_t,
    pub sin6_addr: in6_addr,
    pub sin6_scope_id: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in6_addr {
    pub __in6_u: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub __u6_addr8: [uint8_t; 16],
    pub __u6_addr16: [uint16_t; 8],
    pub __u6_addr32: [uint32_t; 4],
}
pub type in_port_t = uint16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in {
    pub sin_family: sa_family_t,
    pub sin_port: in_port_t,
    pub sin_addr: in_addr,
    pub sin_zero: [libc::c_uchar; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in_addr {
    pub s_addr: in_addr_t,
}
pub type in_addr_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union __CONST_SOCKADDR_ARG {
    pub __sockaddr__: *const sockaddr,
    pub __sockaddr_at__: *const sockaddr_at,
    pub __sockaddr_ax25__: *const sockaddr_ax25,
    pub __sockaddr_dl__: *const sockaddr_dl,
    pub __sockaddr_eon__: *const sockaddr_eon,
    pub __sockaddr_in__: *const sockaddr_in,
    pub __sockaddr_in6__: *const sockaddr_in6,
    pub __sockaddr_inarp__: *const sockaddr_inarp,
    pub __sockaddr_ipx__: *const sockaddr_ipx,
    pub __sockaddr_iso__: *const sockaddr_iso,
    pub __sockaddr_ns__: *const sockaddr_ns,
    pub __sockaddr_un__: *const sockaddr_un,
    pub __sockaddr_x25__: *const sockaddr_x25,
}
pub type C2RustUnnamed_0 = libc::c_uint;
pub const IPPROTO_MAX: C2RustUnnamed_0 = 256;
pub const IPPROTO_RAW: C2RustUnnamed_0 = 255;
pub const IPPROTO_MPLS: C2RustUnnamed_0 = 137;
pub const IPPROTO_UDPLITE: C2RustUnnamed_0 = 136;
pub const IPPROTO_SCTP: C2RustUnnamed_0 = 132;
pub const IPPROTO_COMP: C2RustUnnamed_0 = 108;
pub const IPPROTO_PIM: C2RustUnnamed_0 = 103;
pub const IPPROTO_ENCAP: C2RustUnnamed_0 = 98;
pub const IPPROTO_BEETPH: C2RustUnnamed_0 = 94;
pub const IPPROTO_MTP: C2RustUnnamed_0 = 92;
pub const IPPROTO_AH: C2RustUnnamed_0 = 51;
pub const IPPROTO_ESP: C2RustUnnamed_0 = 50;
pub const IPPROTO_GRE: C2RustUnnamed_0 = 47;
pub const IPPROTO_RSVP: C2RustUnnamed_0 = 46;
pub const IPPROTO_IPV6: C2RustUnnamed_0 = 41;
pub const IPPROTO_DCCP: C2RustUnnamed_0 = 33;
pub const IPPROTO_TP: C2RustUnnamed_0 = 29;
pub const IPPROTO_IDP: C2RustUnnamed_0 = 22;
pub const IPPROTO_UDP: C2RustUnnamed_0 = 17;
pub const IPPROTO_PUP: C2RustUnnamed_0 = 12;
pub const IPPROTO_EGP: C2RustUnnamed_0 = 8;
pub const IPPROTO_TCP: C2RustUnnamed_0 = 6;
pub const IPPROTO_IPIP: C2RustUnnamed_0 = 4;
pub const IPPROTO_IGMP: C2RustUnnamed_0 = 2;
pub const IPPROTO_ICMP: C2RustUnnamed_0 = 1;
pub const IPPROTO_IP: C2RustUnnamed_0 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct event {
    pub ev_evcallback: event_callback,
    pub ev_timeout_pos: C2RustUnnamed_6,
    pub ev_fd: libc::c_int,
    pub ev_base: *mut event_base,
    pub ev_: C2RustUnnamed_1,
    pub ev_events: libc::c_short,
    pub ev_res: libc::c_short,
    pub ev_timeout: timeval,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub ev_io: C2RustUnnamed_4,
    pub ev_signal: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub ev_signal_next: C2RustUnnamed_3,
    pub ev_ncalls: libc::c_short,
    pub ev_pncalls: *mut libc::c_short,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub le_next: *mut event,
    pub le_prev: *mut *mut event,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub ev_io_next: C2RustUnnamed_5,
    pub ev_timeout: timeval,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub le_next: *mut event,
    pub le_prev: *mut *mut event,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_6 {
    pub ev_next_with_common_timeout: C2RustUnnamed_7,
    pub min_heap_idx: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct event_callback {
    pub evcb_active_next: C2RustUnnamed_9,
    pub evcb_flags: libc::c_short,
    pub evcb_pri: uint8_t,
    pub evcb_closure: uint8_t,
    pub evcb_cb_union: C2RustUnnamed_8,
    pub evcb_arg: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_8 {
    pub evcb_callback: Option::<
        unsafe extern "C" fn(libc::c_int, libc::c_short, *mut libc::c_void) -> (),
    >,
    pub evcb_selfcb: Option::<
        unsafe extern "C" fn(*mut event_callback, *mut libc::c_void) -> (),
    >,
    pub evcb_evfinalize: Option::<
        unsafe extern "C" fn(*mut event, *mut libc::c_void) -> (),
    >,
    pub evcb_cbfinalize: Option::<
        unsafe extern "C" fn(*mut event_callback, *mut libc::c_void) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub tqe_next: *mut event_callback,
    pub tqe_prev: *mut *mut event_callback,
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
pub struct redisAsyncContext {
    pub c: redisContext,
    pub err: libc::c_int,
    pub errstr: *mut libc::c_char,
    pub data: *mut libc::c_void,
    pub dataCleanup: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub ev: C2RustUnnamed_11,
    pub onDisconnect: Option::<redisDisconnectCallback>,
    pub onConnect: Option::<redisConnectCallback>,
    pub replies: redisCallbackList,
    pub saddr: *mut sockaddr,
    pub addrlen: size_t,
    pub sub: C2RustUnnamed_10,
    pub push_cb: Option::<redisAsyncPushFn>,
}
pub type redisAsyncPushFn = unsafe extern "C" fn(
    *mut redisAsyncContext,
    *mut libc::c_void,
) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub invalid: redisCallbackList,
    pub channels: *mut dict,
    pub patterns: *mut dict,
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
    pub privdata: *mut libc::c_void,
}
pub type redisCallbackFn = unsafe extern "C" fn(
    *mut redisAsyncContext,
    *mut libc::c_void,
    *mut libc::c_void,
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
pub struct C2RustUnnamed_11 {
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
    pub tcp: C2RustUnnamed_13,
    pub unix_sock: C2RustUnnamed_12,
    pub saddr: *mut sockadr,
    pub addrlen: size_t,
    pub privdata: *mut libc::c_void,
    pub free_privdata: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub privctx: *mut libc::c_void,
    pub push_cb: Option::<redisPushFn>,
}
pub type redisPushFn = unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_12 {
    pub path: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_13 {
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
pub struct worker {
    pub thread: pthread_t,
    pub base: *mut event_base,
    pub s: *mut server,
    pub link: [libc::c_int; 2],
    pub pool: *mut pool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct server {
    pub fd: libc::c_int,
    pub ev: event,
    pub base: *mut event_base,
    pub cfg: *mut conf,
    pub w: *mut *mut worker,
    pub next_worker: libc::c_int,
    pub log: C2RustUnnamed_14,
    pub auth_log_mutex: pthread_mutex_t,
    pub auth_logged: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_14 {
    pub self_0: pid_t,
    pub fd: libc::c_int,
    pub fsync_tv: timeval,
    pub fsync_ev: *mut event,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct conf {
    pub redis_host: *mut libc::c_char,
    pub redis_port: libc::c_int,
    pub redis_auth: *mut auth,
    pub http_host: *mut libc::c_char,
    pub http_port: libc::c_int,
    pub http_threads: libc::c_int,
    pub http_max_request_size: size_t,
    pub pool_size_per_thread: libc::c_int,
    pub daemonize: libc::c_int,
    pub pidfile: *mut libc::c_char,
    pub websockets: libc::c_int,
    pub database: libc::c_int,
    pub perms: *mut acl,
    pub user: uid_t,
    pub group: gid_t,
    pub logfile: *mut libc::c_char,
    pub verbosity: log_level,
    pub log_fsync: C2RustUnnamed_16,
    pub hiredis_opts: C2RustUnnamed_15,
    pub default_root: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_15 {
    pub keep_alive_sec: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_16 {
    pub mode: log_fsync_mode,
    pub period_millis: libc::c_int,
}
pub type log_fsync_mode = libc::c_uint;
pub const LOG_FSYNC_ALL: log_fsync_mode = 2;
pub const LOG_FSYNC_MILLIS: log_fsync_mode = 1;
pub const LOG_FSYNC_AUTO: log_fsync_mode = 0;
pub type log_level = libc::c_uint;
pub const WEBDIS_TRACE: log_level = 8;
pub const WEBDIS_DEBUG: log_level = 4;
pub const WEBDIS_INFO: log_level = 3;
pub const WEBDIS_NOTICE: log_level = 2;
pub const WEBDIS_WARNING: log_level = 1;
pub const WEBDIS_ERROR: log_level = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct auth {
    pub use_legacy_auth: libc::c_int,
    pub username: *mut libc::c_char,
    pub password: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct http_client {
    pub fd: libc::c_int,
    pub addr: in_addr_t,
    pub ev: event,
    pub w: *mut worker,
    pub s: *mut server,
    pub parser: http_parser,
    pub settings: http_parser_settings,
    pub buffer: *mut libc::c_char,
    pub sz: size_t,
    pub request_sz: size_t,
    pub last_cb: last_cb_t,
    pub keep_alive: libc::c_char,
    pub broken: libc::c_char,
    pub fully_read: libc::c_char,
    pub is_websocket: libc::c_char,
    pub http_version: libc::c_char,
    pub failed_alloc: libc::c_char,
    pub path: *mut libc::c_char,
    pub path_sz: size_t,
    pub headers: *mut http_header,
    pub header_count: libc::c_int,
    pub body: *mut libc::c_char,
    pub body_sz: size_t,
    pub type_0: *mut libc::c_char,
    pub jsonp: *mut libc::c_char,
    pub separator: *mut libc::c_char,
    pub filename: *mut libc::c_char,
    pub reused_cmd: *mut cmd,
    pub last_cmd: *mut cmd,
    pub ws: *mut ws_client,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ws_client {
    pub http_client: *mut http_client,
    pub scheduled_read: libc::c_int,
    pub scheduled_write: libc::c_int,
    pub rbuf: *mut evbuffer,
    pub wbuf: *mut evbuffer,
    pub ac: *mut redisAsyncContext,
    pub cmd: *mut cmd,
    pub close_after_events: libc::c_int,
    pub ran_subscribe: libc::c_int,
}
pub type last_cb_t = libc::c_uint;
pub const LAST_CB_VAL: last_cb_t = 2;
pub const LAST_CB_KEY: last_cb_t = 1;
pub const LAST_CB_NONE: last_cb_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct http_parser_settings {
    pub on_message_begin: http_cb,
    pub on_path: http_data_cb,
    pub on_query_string: http_data_cb,
    pub on_url: http_data_cb,
    pub on_fragment: http_data_cb,
    pub on_header_field: http_data_cb,
    pub on_header_value: http_data_cb,
    pub on_headers_complete: http_cb,
    pub on_body: http_data_cb,
    pub on_message_complete: http_cb,
}
pub type http_cb = Option::<unsafe extern "C" fn(*mut http_parser) -> libc::c_int>;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct http_parser {
    #[bitfield(name = "type_0", ty = "libc::c_uchar", bits = "0..=1")]
    #[bitfield(name = "flags", ty = "libc::c_uchar", bits = "2..=7")]
    pub type_0_flags: [u8; 1],
    pub state: libc::c_uchar,
    pub header_state: libc::c_uchar,
    pub index: libc::c_uchar,
    pub nread: uint32_t,
    pub content_length: int64_t,
    pub http_major: libc::c_ushort,
    pub http_minor: libc::c_ushort,
    pub status_code: libc::c_ushort,
    pub method: libc::c_uchar,
    pub upgrade: libc::c_char,
    pub data: *mut libc::c_void,
}
pub type http_data_cb = Option::<
    unsafe extern "C" fn(*mut http_parser, *const libc::c_char, size_t) -> libc::c_int,
>;
pub type __sighandler_t = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
#[inline]
unsafe extern "C" fn __bswap_16(mut __bsx: __uint16_t) -> __uint16_t {
    return (__bsx as libc::c_int >> 8 as libc::c_int & 0xff as libc::c_int
        | (__bsx as libc::c_int & 0xff as libc::c_int) << 8 as libc::c_int)
        as __uint16_t;
}
unsafe extern "C" fn socket_setup(
    mut s: *mut server,
    mut ip: *const libc::c_char,
    mut port: libc::c_int,
) -> libc::c_int {
    let mut reuse: libc::c_int = 1 as libc::c_int;
    let mut addr: sockaddr_in = sockaddr_in {
        sin_family: 0,
        sin_port: 0,
        sin_addr: in_addr { s_addr: 0 },
        sin_zero: [0; 8],
    };
    let mut len: socklen_t = ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong
        as socklen_t;
    let mut fd: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    memset(
        &mut addr as *mut sockaddr_in as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong,
    );
    addr.sin_family = 2 as libc::c_int as sa_family_t;
    addr.sin_port = __bswap_16(port as __uint16_t);
    addr.sin_addr.s_addr = inet_addr(ip);
    fd = socket(
        2 as libc::c_int,
        SOCK_STREAM as libc::c_int,
        IPPROTO_TCP as libc::c_int,
    );
    if -(1 as libc::c_int) == fd {
        slog(s, WEBDIS_ERROR, strerror(*__errno_location()), 0 as libc::c_int as size_t);
        return -(1 as libc::c_int);
    }
    if setsockopt(
        fd,
        1 as libc::c_int,
        2 as libc::c_int,
        &mut reuse as *mut libc::c_int as *const libc::c_void,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t,
    ) < 0 as libc::c_int
    {
        slog(s, WEBDIS_ERROR, strerror(*__errno_location()), 0 as libc::c_int as size_t);
        return -(1 as libc::c_int);
    }
    ret = fcntl(fd, 2 as libc::c_int, 0o4000 as libc::c_int);
    if 0 as libc::c_int != ret {
        slog(s, WEBDIS_ERROR, strerror(*__errno_location()), 0 as libc::c_int as size_t);
        return -(1 as libc::c_int);
    }
    ret = bind(
        fd,
        __CONST_SOCKADDR_ARG {
            __sockaddr__: &mut addr as *mut sockaddr_in as *mut sockaddr,
        },
        len,
    );
    if 0 as libc::c_int != ret {
        slog(s, WEBDIS_ERROR, strerror(*__errno_location()), 0 as libc::c_int as size_t);
        return -(1 as libc::c_int);
    }
    ret = listen(fd, 4096 as libc::c_int);
    if 0 as libc::c_int != ret {
        slog(s, WEBDIS_ERROR, strerror(*__errno_location()), 0 as libc::c_int as size_t);
        return -(1 as libc::c_int);
    }
    if getsockname(
        fd,
        __SOCKADDR_ARG {
            __sockaddr__: &mut addr as *mut sockaddr_in as *mut sockaddr,
        },
        &mut len,
    ) != -(1 as libc::c_int)
    {
        let mut comment: *const libc::c_char = b"Webdis listening on port %d\0"
            as *const u8 as *const libc::c_char;
        let mut port_num: libc::c_int = __bswap_16(addr.sin_port) as libc::c_int;
        let mut buffer: *mut libc::c_char = malloc(
            (strlen(comment))
                .wrapping_sub(2 as libc::c_int as libc::c_ulong)
                .wrapping_add(strlen(b"65535\0" as *const u8 as *const libc::c_char))
                .wrapping_add(1 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_char;
        sprintf(buffer, comment, port_num);
        slog(s, WEBDIS_INFO, buffer, 0 as libc::c_int as size_t);
        free(buffer as *mut libc::c_void);
    }
    return fd;
}
pub unsafe extern "C" fn server_new(mut cfg_file: *const libc::c_char) -> *mut server {
    let mut i: libc::c_int = 0;
    let mut s: *mut server = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<server>() as libc::c_ulong,
    ) as *mut server;
    (*s).log.fd = -(1 as libc::c_int);
    (*s).cfg = conf_read(cfg_file);
    slog_init(s);
    (*s)
        .w = calloc(
        (*(*s).cfg).http_threads as libc::c_ulong,
        ::std::mem::size_of::<*mut worker>() as libc::c_ulong,
    ) as *mut *mut worker;
    i = 0 as libc::c_int;
    while i < (*(*s).cfg).http_threads {
        let ref mut fresh0 = *((*s).w).offset(i as isize);
        *fresh0 = worker_new(s);
        i += 1;
        i;
    }
    pthread_mutex_init(&mut (*s).auth_log_mutex, 0 as *const pthread_mutexattr_t);
    return s;
}
unsafe extern "C" fn server_can_accept(
    mut fd: libc::c_int,
    mut event: libc::c_short,
    mut ptr: *mut libc::c_void,
) {
    let mut s: *mut server = ptr as *mut server;
    let mut w: *mut worker = 0 as *mut worker;
    let mut c: *mut http_client = 0 as *mut http_client;
    let mut client_fd: libc::c_int = 0;
    let mut addr: sockaddr_in = sockaddr_in {
        sin_family: 0,
        sin_port: 0,
        sin_addr: in_addr { s_addr: 0 },
        sin_zero: [0; 8],
    };
    let mut addr_sz: socklen_t = ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong
        as socklen_t;
    let mut on: libc::c_int = 1 as libc::c_int;
    w = *((*s).w).offset((*s).next_worker as isize);
    client_fd = accept(
        fd,
        __SOCKADDR_ARG {
            __sockaddr__: &mut addr as *mut sockaddr_in as *mut sockaddr,
        },
        &mut addr_sz,
    );
    let mut status: libc::c_int = ioctl(
        client_fd,
        0x5421 as libc::c_int as libc::c_ulong,
        &mut on as *mut libc::c_int,
    );
    if status == -(1 as libc::c_int) {
        let mut log_msg: [libc::c_char; 200] = [0; 200];
        let mut log_msg_sz: libc::c_int = snprintf(
            log_msg.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 200]>() as libc::c_ulong,
            b"ioctl failed (%d): %s\0" as *const u8 as *const libc::c_char,
            *__errno_location(),
            strerror(*__errno_location()),
        );
        slog(s, WEBDIS_ERROR, log_msg.as_mut_ptr(), log_msg_sz as size_t);
    }
    if client_fd > 0 as libc::c_int {
        c = http_client_new(w, client_fd, addr.sin_addr.s_addr);
        worker_add_client(w, c);
        (*s)
            .next_worker = ((*s).next_worker + 1 as libc::c_int)
            % (*(*s).cfg).http_threads;
    } else {
        slog(
            s,
            WEBDIS_NOTICE,
            b"Too many connections\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int as size_t,
        );
    };
}
unsafe extern "C" fn server_daemonize(
    mut s: *mut server,
    mut pidfile: *const libc::c_char,
) {
    let mut fd: libc::c_int = 0;
    if fork() != 0 as libc::c_int {
        exit(0 as libc::c_int);
    }
    setsid();
    fd = open(
        b"/dev/null\0" as *const u8 as *const libc::c_char,
        0o2 as libc::c_int,
        0 as libc::c_int,
    );
    if fd != -(1 as libc::c_int) {
        dup2(fd, 0 as libc::c_int);
        dup2(fd, 1 as libc::c_int);
        dup2(fd, 2 as libc::c_int);
        if fd > 2 as libc::c_int {
            close(fd);
        }
    }
    if !pidfile.is_null() {
        let mut pid_fd: libc::c_int = open(
            pidfile,
            0o1 as libc::c_int | 0o100 as libc::c_int | 0o1000 as libc::c_int,
            0o600 as libc::c_int,
        );
        if pid_fd > 0 as libc::c_int {
            let mut pid_buffer: [libc::c_char; 13] = [0; 13];
            let mut pid_sz: libc::c_int = snprintf(
                pid_buffer.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong,
                b"%d\n\0" as *const u8 as *const libc::c_char,
                getpid(),
            );
            let mut written: ssize_t = 0;
            let mut written_total: libc::c_int = 0 as libc::c_int;
            loop {
                written = write(
                    pid_fd,
                    pid_buffer.as_mut_ptr().offset(written_total as isize)
                        as *const libc::c_void,
                    (pid_sz - written_total) as size_t,
                );
                if !(written > 0 as libc::c_int as libc::c_long
                    && written_total < pid_sz)
                {
                    break;
                }
                written_total = (written_total as libc::c_long + written) as libc::c_int;
            }
            close(pid_fd);
        } else {
            let err_msg: [libc::c_char; 26] = *::std::mem::transmute::<
                &[u8; 26],
                &[libc::c_char; 26],
            >(b"Failed to create PID file\0");
            slog(
                s,
                WEBDIS_ERROR,
                err_msg.as_ptr(),
                (::std::mem::size_of::<[libc::c_char; 26]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            );
            if *__errno_location() != 0 {
                let mut errno_msg: *mut libc::c_char = strerror(*__errno_location());
                slog(s, WEBDIS_ERROR, errno_msg, strlen(errno_msg));
            }
        }
    }
}
static mut __server: *mut server = 0 as *const server as *mut server;
unsafe extern "C" fn server_handle_signal(mut id: libc::c_int) {
    let mut ret: libc::c_int = 0;
    match id {
        1 => {
            slog_init(__server);
        }
        15 | 2 => {
            slog(
                __server,
                WEBDIS_INFO,
                b"Webdis terminating\0" as *const u8 as *const libc::c_char,
                0 as libc::c_int as size_t,
            );
            ret = fsync((*__server).log.fd);
            exit(0 as libc::c_int);
        }
        _ => {}
    };
}
unsafe extern "C" fn server_install_signal_handlers(mut s: *mut server) {
    __server = s;
    signal(
        1 as libc::c_int,
        Some(server_handle_signal as unsafe extern "C" fn(libc::c_int) -> ()),
    );
    signal(
        15 as libc::c_int,
        Some(server_handle_signal as unsafe extern "C" fn(libc::c_int) -> ()),
    );
    signal(
        2 as libc::c_int,
        Some(server_handle_signal as unsafe extern "C" fn(libc::c_int) -> ()),
    );
}
pub unsafe extern "C" fn server_start(mut s: *mut server) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    (*s).base = event_base_new();
    if (*(*s).cfg).daemonize != 0 {
        server_daemonize(s, (*(*s).cfg).pidfile);
        if event_reinit((*s).base) != 0 as libc::c_int {
            fprintf(
                stderr,
                b"Error: event_reinit failed after fork\0" as *const u8
                    as *const libc::c_char,
            );
        }
    }
    signal(
        13 as libc::c_int,
        ::std::mem::transmute::<
            libc::intptr_t,
            __sighandler_t,
        >(1 as libc::c_int as libc::intptr_t),
    );
    server_install_signal_handlers(s);
    i = 0 as libc::c_int;
    while i < (*(*s).cfg).http_threads {
        worker_start(*((*s).w).offset(i as isize));
        i += 1;
        i;
    }
    (*s).fd = socket_setup(s, (*(*s).cfg).http_host, (*(*s).cfg).http_port);
    if (*s).fd < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    let mut keep_alive: libc::c_int = 1 as libc::c_int;
    setsockopt(
        (*s).fd,
        1 as libc::c_int,
        9 as libc::c_int,
        &mut keep_alive as *mut libc::c_int as *mut libc::c_void,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t,
    );
    event_set(
        &mut (*s).ev,
        (*s).fd,
        (0x2 as libc::c_int | 0x10 as libc::c_int) as libc::c_short,
        Some(
            server_can_accept
                as unsafe extern "C" fn(
                    libc::c_int,
                    libc::c_short,
                    *mut libc::c_void,
                ) -> (),
        ),
        s as *mut libc::c_void,
    );
    event_base_set((*s).base, &mut (*s).ev);
    ret = event_add(&mut (*s).ev, 0 as *const timeval);
    if ret < 0 as libc::c_int {
        slog(
            s,
            WEBDIS_ERROR,
            b"Error calling event_add on socket\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int as size_t,
        );
        return -(1 as libc::c_int);
    }
    slog_fsync_init(s);
    slog(
        s,
        WEBDIS_INFO,
        b"Webdis 0.1.23-dev up and running\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int as size_t,
    );
    event_base_dispatch((*s).base);
    return 0 as libc::c_int;
}
