use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type event_base;
    pub type evbuffer;
    pub type dict;
    pub type sockadr;
    pub type pool;
    pub type acl;
    fn redisAsyncDisconnect(ac: *mut redisAsyncContext);
    fn http_parser_init(parser: *mut http_parser, type_0: http_parser_type);
    fn http_parser_execute(
        parser: *mut http_parser,
        settings: *const http_parser_settings,
        data: *const libc::c_char,
        len: size_t,
    ) -> size_t;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn worker_process_client(c: *mut http_client);
    fn cmd_free(c: *mut cmd);
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
    fn memchr(
        _: *const libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strdup(__s: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strncasecmp(
        __s1: *const libc::c_char,
        __s2: *const libc::c_char,
        __n: size_t,
    ) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __ssize_t = libc::c_long;
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
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
    pub __align: libc::c_long,
}
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type sa_family_t = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [libc::c_char; 14],
}
pub type in_addr_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct event {
    pub ev_evcallback: event_callback,
    pub ev_timeout_pos: C2RustUnnamed_4,
    pub ev_fd: libc::c_int,
    pub ev_base: *mut event_base,
    pub ev_: C2RustUnnamed,
    pub ev_events: libc::c_short,
    pub ev_res: libc::c_short,
    pub ev_timeout: timeval,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub ev_io: C2RustUnnamed_2,
    pub ev_signal: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub ev_signal_next: C2RustUnnamed_1,
    pub ev_ncalls: libc::c_short,
    pub ev_pncalls: *mut libc::c_short,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub le_next: *mut event,
    pub le_prev: *mut *mut event,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub ev_io_next: C2RustUnnamed_3,
    pub ev_timeout: timeval,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub le_next: *mut event,
    pub le_prev: *mut *mut event,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
    pub ev_next_with_common_timeout: C2RustUnnamed_5,
    pub min_heap_idx: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct event_callback {
    pub evcb_active_next: C2RustUnnamed_7,
    pub evcb_flags: libc::c_short,
    pub evcb_pri: uint8_t,
    pub evcb_closure: uint8_t,
    pub evcb_cb_union: C2RustUnnamed_6,
    pub evcb_arg: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_6 {
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
pub struct C2RustUnnamed_7 {
    pub tqe_next: *mut event_callback,
    pub tqe_prev: *mut *mut event_callback,
}
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
pub type http_data_cb = Option::<
    unsafe extern "C" fn(*mut http_parser, *const libc::c_char, size_t) -> libc::c_int,
>;
pub type http_parser_type = libc::c_uint;
pub const HTTP_BOTH: http_parser_type = 2;
pub const HTTP_RESPONSE: http_parser_type = 1;
pub const HTTP_REQUEST: http_parser_type = 0;
pub type flags = libc::c_uint;
pub const F_SKIPBODY: flags = 32;
pub const F_UPGRADE: flags = 16;
pub const F_TRAILING: flags = 8;
pub const F_CONNECTION_CLOSE: flags = 4;
pub const F_CONNECTION_KEEP_ALIVE: flags = 2;
pub const F_CHUNKED: flags = 1;
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
    pub ev: C2RustUnnamed_9,
    pub onDisconnect: Option::<redisDisconnectCallback>,
    pub onConnect: Option::<redisConnectCallback>,
    pub replies: redisCallbackList,
    pub saddr: *mut sockaddr,
    pub addrlen: size_t,
    pub sub: C2RustUnnamed_8,
    pub push_cb: Option::<redisAsyncPushFn>,
}
pub type redisAsyncPushFn = unsafe extern "C" fn(
    *mut redisAsyncContext,
    *mut libc::c_void,
) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
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
pub struct C2RustUnnamed_9 {
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
    pub tcp: C2RustUnnamed_11,
    pub unix_sock: C2RustUnnamed_10,
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
pub struct C2RustUnnamed_10 {
    pub path: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_11 {
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmd {
    pub fd: libc::c_int,
    pub count: libc::c_int,
    pub argv: *mut *mut libc::c_char,
    pub argv_len: *mut size_t,
    pub mime: *mut libc::c_char,
    pub mime_free: libc::c_int,
    pub filename: *mut libc::c_char,
    pub if_none_match: *mut libc::c_char,
    pub jsonp: *mut libc::c_char,
    pub separator: *mut libc::c_char,
    pub keep_alive: libc::c_int,
    pub started_responding: libc::c_int,
    pub is_websocket: libc::c_int,
    pub http_version: libc::c_int,
    pub database: libc::c_int,
    pub http_client: *mut http_client,
    pub pub_sub_client: *mut http_client,
    pub ac: *mut redisAsyncContext,
    pub w: *mut worker,
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
    pub log: C2RustUnnamed_12,
    pub auth_log_mutex: pthread_mutex_t,
    pub auth_logged: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_12 {
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
    pub log_fsync: C2RustUnnamed_14,
    pub hiredis_opts: C2RustUnnamed_13,
    pub default_root: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_13 {
    pub keep_alive_sec: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_14 {
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
pub struct http_header {
    pub key: *mut libc::c_char,
    pub key_sz: size_t,
    pub val: *mut libc::c_char,
    pub val_sz: size_t,
    pub copy: header_copy,
}
pub type header_copy = libc::c_uint;
pub const HEADER_CHECK_DUPE: header_copy = 4;
pub const HEADER_COPY_VALUE: header_copy = 2;
pub const HEADER_COPY_KEY: header_copy = 1;
pub const HEADER_COPY_NONE: header_copy = 0;
pub type last_cb_t = libc::c_uint;
pub const LAST_CB_VAL: last_cb_t = 2;
pub const LAST_CB_KEY: last_cb_t = 1;
pub const LAST_CB_NONE: last_cb_t = 0;
pub type C2RustUnnamed_15 = libc::c_int;
pub const CLIENT_OOM: C2RustUnnamed_15 = -2;
pub const CLIENT_DISCONNECTED: C2RustUnnamed_15 = -1;
unsafe extern "C" fn http_client_on_url(
    mut p: *mut http_parser,
    mut at: *const libc::c_char,
    mut sz: size_t,
) -> libc::c_int {
    let mut c: *mut http_client = (*p).data as *mut http_client;
    (*c)
        .path = realloc(
        (*c).path as *mut libc::c_void,
        ((*c).path_sz).wrapping_add(sz).wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    if ((*c).path).is_null() {
        (*c).failed_alloc = 1 as libc::c_int as libc::c_char;
        return -(1 as libc::c_int);
    }
    memcpy(
        ((*c).path).offset((*c).path_sz as isize) as *mut libc::c_void,
        at as *const libc::c_void,
        sz,
    );
    (*c).path_sz = ((*c).path_sz as libc::c_ulong).wrapping_add(sz) as size_t as size_t;
    *((*c).path).offset((*c).path_sz as isize) = 0 as libc::c_int as libc::c_char;
    return 0 as libc::c_int;
}
unsafe extern "C" fn http_client_on_body(
    mut p: *mut http_parser,
    mut at: *const libc::c_char,
    mut sz: size_t,
) -> libc::c_int {
    let mut c: *mut http_client = (*p).data as *mut http_client;
    return http_client_add_to_body(c, at, sz);
}
pub unsafe extern "C" fn http_client_add_to_body(
    mut c: *mut http_client,
    mut at: *const libc::c_char,
    mut sz: size_t,
) -> libc::c_int {
    (*c)
        .body = realloc(
        (*c).body as *mut libc::c_void,
        ((*c).body_sz).wrapping_add(sz).wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    if ((*c).body).is_null() {
        (*c).failed_alloc = 1 as libc::c_int as libc::c_char;
        return -(1 as libc::c_int);
    }
    memcpy(
        ((*c).body).offset((*c).body_sz as isize) as *mut libc::c_void,
        at as *const libc::c_void,
        sz,
    );
    (*c).body_sz = ((*c).body_sz as libc::c_ulong).wrapping_add(sz) as size_t as size_t;
    *((*c).body).offset((*c).body_sz as isize) = 0 as libc::c_int as libc::c_char;
    return 0 as libc::c_int;
}
unsafe extern "C" fn http_client_on_header_name(
    mut p: *mut http_parser,
    mut at: *const libc::c_char,
    mut sz: size_t,
) -> libc::c_int {
    let mut c: *mut http_client = (*p).data as *mut http_client;
    let mut n: size_t = (*c).header_count as size_t;
    if (*c).last_cb as libc::c_uint != LAST_CB_KEY as libc::c_int as libc::c_uint {
        (*c).header_count += 1;
        n = (*c).header_count as size_t;
        (*c)
            .headers = realloc(
            (*c).headers as *mut libc::c_void,
            n.wrapping_mul(::std::mem::size_of::<http_header>() as libc::c_ulong),
        ) as *mut http_header;
        if ((*c).headers).is_null() {
            (*c).failed_alloc = 1 as libc::c_int as libc::c_char;
            return -(1 as libc::c_int);
        }
        memset(
            &mut *((*c).headers)
                .offset(n.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
                as *mut http_header as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<http_header>() as libc::c_ulong,
        );
    }
    let ref mut fresh0 = (*((*c).headers)
        .offset(n.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
        .key;
    *fresh0 = realloc(
        (*((*c).headers)
            .offset(n.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
            .key as *mut libc::c_void,
        ((*((*c).headers)
            .offset(n.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
            .key_sz)
            .wrapping_add(sz)
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    if (*fresh0).is_null() {
        (*c).failed_alloc = 1 as libc::c_int as libc::c_char;
        return -(1 as libc::c_int);
    }
    memcpy(
        ((*((*c).headers)
            .offset(n.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
            .key)
            .offset(
                (*((*c).headers)
                    .offset(n.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                    .key_sz as isize,
            ) as *mut libc::c_void,
        at as *const libc::c_void,
        sz,
    );
    let ref mut fresh1 = (*((*c).headers)
        .offset(n.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
        .key_sz;
    *fresh1 = (*fresh1 as libc::c_ulong).wrapping_add(sz) as size_t as size_t;
    *((*((*c).headers)
        .offset(n.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
        .key)
        .offset(
            (*((*c).headers)
                .offset(n.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .key_sz as isize,
        ) = 0 as libc::c_int as libc::c_char;
    (*c).last_cb = LAST_CB_KEY;
    return 0 as libc::c_int;
}
unsafe extern "C" fn wrap_filename(
    mut val: *const libc::c_char,
    mut val_len: size_t,
) -> *mut libc::c_char {
    let mut format: [libc::c_char; 23] = *::std::mem::transmute::<
        &[u8; 23],
        &mut [libc::c_char; 23],
    >(b"attachment; filename=\"\0");
    let mut sz: size_t = (::std::mem::size_of::<[libc::c_char; 23]>() as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        .wrapping_add(val_len)
        .wrapping_add(1 as libc::c_int as libc::c_ulong);
    let mut p: *mut libc::c_char = calloc(
        sz.wrapping_add(1 as libc::c_int as libc::c_ulong),
        1 as libc::c_int as libc::c_ulong,
    ) as *mut libc::c_char;
    memcpy(
        p as *mut libc::c_void,
        format.as_mut_ptr() as *const libc::c_void,
        (::std::mem::size_of::<[libc::c_char; 23]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    );
    memcpy(
        p
            .offset(
                ::std::mem::size_of::<[libc::c_char; 23]>() as libc::c_ulong as isize,
            )
            .offset(-(1 as libc::c_int as isize)) as *mut libc::c_void,
        val as *const libc::c_void,
        val_len,
    );
    *p
        .offset(
            sz.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
        ) = '"' as i32 as libc::c_char;
    return p;
}
unsafe extern "C" fn http_client_on_query_string(
    mut parser: *mut http_parser,
    mut at: *const libc::c_char,
    mut sz: size_t,
) -> libc::c_int {
    let mut c: *mut http_client = (*parser).data as *mut http_client;
    let mut p: *const libc::c_char = at;
    while p < at.offset(sz as isize) {
        let mut key: *const libc::c_char = p;
        let mut val: *const libc::c_char = 0 as *const libc::c_char;
        let mut key_len: libc::c_int = 0;
        let mut val_len: libc::c_int = 0;
        let mut eq: *mut libc::c_char = memchr(
            key as *const libc::c_void,
            '=' as i32,
            sz.wrapping_sub(p.offset_from(at) as libc::c_long as libc::c_ulong),
        ) as *mut libc::c_char;
        if eq.is_null() || eq > at.offset(sz as isize) as *mut libc::c_char {
            break;
        } else {
            let mut amp: *mut libc::c_char = 0 as *mut libc::c_char;
            val = eq.offset(1 as libc::c_int as isize);
            key_len = eq.offset_from(key) as libc::c_long as libc::c_int;
            p = eq.offset(1 as libc::c_int as isize);
            amp = memchr(
                p as *const libc::c_void,
                '&' as i32,
                sz.wrapping_sub(p.offset_from(at) as libc::c_long as libc::c_ulong),
            ) as *mut libc::c_char;
            if amp.is_null() || amp > at.offset(sz as isize) as *mut libc::c_char {
                val_len = at.offset(sz as isize).offset_from(p) as libc::c_long
                    as libc::c_int;
            } else {
                val_len = amp.offset_from(val) as libc::c_long as libc::c_int;
                p = amp.offset(1 as libc::c_int as isize);
            }
            if key_len == 4 as libc::c_int
                && strncmp(
                    key,
                    b"type\0" as *const u8 as *const libc::c_char,
                    4 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
            {
                (*c)
                    .type_0 = calloc(
                    (1 as libc::c_int + val_len) as libc::c_ulong,
                    1 as libc::c_int as libc::c_ulong,
                ) as *mut libc::c_char;
                memcpy(
                    (*c).type_0 as *mut libc::c_void,
                    val as *const libc::c_void,
                    val_len as libc::c_ulong,
                );
            } else if key_len == 5 as libc::c_int
                && strncmp(
                    key,
                    b"jsonp\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                || key_len == 8 as libc::c_int
                    && strncmp(
                        key,
                        b"callback\0" as *const u8 as *const libc::c_char,
                        8 as libc::c_int as libc::c_ulong,
                    ) == 0 as libc::c_int
            {
                (*c)
                    .jsonp = calloc(
                    (1 as libc::c_int + val_len) as libc::c_ulong,
                    1 as libc::c_int as libc::c_ulong,
                ) as *mut libc::c_char;
                memcpy(
                    (*c).jsonp as *mut libc::c_void,
                    val as *const libc::c_void,
                    val_len as libc::c_ulong,
                );
            } else if key_len == 3 as libc::c_int
                && strncmp(
                    key,
                    b"sep\0" as *const u8 as *const libc::c_char,
                    3 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
            {
                (*c)
                    .separator = calloc(
                    (1 as libc::c_int + val_len) as libc::c_ulong,
                    1 as libc::c_int as libc::c_ulong,
                ) as *mut libc::c_char;
                memcpy(
                    (*c).separator as *mut libc::c_void,
                    val as *const libc::c_void,
                    val_len as libc::c_ulong,
                );
            } else if key_len == 8 as libc::c_int
                && strncmp(
                    key,
                    b"filename\0" as *const u8 as *const libc::c_char,
                    8 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
            {
                (*c).filename = wrap_filename(val, val_len as size_t);
            }
            if amp.is_null() {
                break;
            }
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn http_client_on_header_value(
    mut p: *mut http_parser,
    mut at: *const libc::c_char,
    mut sz: size_t,
) -> libc::c_int {
    let mut c: *mut http_client = (*p).data as *mut http_client;
    let mut n: size_t = (*c).header_count as size_t;
    let ref mut fresh2 = (*((*c).headers)
        .offset(n.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
        .val;
    *fresh2 = realloc(
        (*((*c).headers)
            .offset(n.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
            .val as *mut libc::c_void,
        ((*((*c).headers)
            .offset(n.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
            .val_sz)
            .wrapping_add(sz)
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    if (*fresh2).is_null() {
        (*c).failed_alloc = 1 as libc::c_int as libc::c_char;
        return -(1 as libc::c_int);
    }
    memcpy(
        ((*((*c).headers)
            .offset(n.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
            .val)
            .offset(
                (*((*c).headers)
                    .offset(n.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                    .val_sz as isize,
            ) as *mut libc::c_void,
        at as *const libc::c_void,
        sz,
    );
    let ref mut fresh3 = (*((*c).headers)
        .offset(n.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
        .val_sz;
    *fresh3 = (*fresh3 as libc::c_ulong).wrapping_add(sz) as size_t as size_t;
    *((*((*c).headers)
        .offset(n.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
        .val)
        .offset(
            (*((*c).headers)
                .offset(n.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .val_sz as isize,
        ) = 0 as libc::c_int as libc::c_char;
    (*c).last_cb = LAST_CB_VAL;
    if strncmp(
        b"Expect\0" as *const u8 as *const libc::c_char,
        (*((*c).headers)
            .offset(n.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
            .key,
        (*((*c).headers)
            .offset(n.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
            .key_sz,
    ) == 0 as libc::c_int
    {
        if sz == 12 as libc::c_int as libc::c_ulong
            && strncasecmp(at, b"100-continue\0" as *const u8 as *const libc::c_char, sz)
                == 0 as libc::c_int
        {
            let mut http100: [libc::c_char; 26] = *::std::mem::transmute::<
                &[u8; 26],
                &mut [libc::c_char; 26],
            >(b"HTTP/1.1 100 Continue\r\n\r\n\0");
            let mut ret: libc::c_int = write(
                (*c).fd,
                http100.as_mut_ptr() as *const libc::c_void,
                (::std::mem::size_of::<[libc::c_char; 26]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            ) as libc::c_int;
        }
    } else if strncasecmp(
        b"Connection\0" as *const u8 as *const libc::c_char,
        (*((*c).headers)
            .offset(n.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
            .key,
        (*((*c).headers)
            .offset(n.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
            .key_sz,
    ) == 0 as libc::c_int
    {
        if sz == 10 as libc::c_int as libc::c_ulong
            && strncasecmp(at, b"Keep-Alive\0" as *const u8 as *const libc::c_char, sz)
                == 0 as libc::c_int
        {
            (*c).keep_alive = 1 as libc::c_int as libc::c_char;
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn http_client_on_message_complete(
    mut p: *mut http_parser,
) -> libc::c_int {
    let mut c: *mut http_client = (*p).data as *mut http_client;
    if ((*c).parser).flags() as libc::c_int & F_CONNECTION_CLOSE as libc::c_int != 0 {
        (*c).keep_alive = 0 as libc::c_int as libc::c_char;
        (*c).fully_read = 1 as libc::c_int as libc::c_char;
    } else if (*c).parser.http_major as libc::c_int == 1 as libc::c_int
        && (*c).parser.http_minor as libc::c_int == 1 as libc::c_int
    {
        (*c).keep_alive = 1 as libc::c_int as libc::c_char;
    }
    (*c).http_version = (*c).parser.http_minor as libc::c_char;
    if (*p).upgrade as libc::c_int != 0 && (*(*(*(*c).w).s).cfg).websockets != 0 {
        (*c).is_websocket = 1 as libc::c_int as libc::c_char;
        return 0 as libc::c_int;
    }
    if (*c).path_sz == 1 as libc::c_int as libc::c_ulong
        && *(*c).path as libc::c_int == '/' as i32
        && !((*(*(*(*c).w).s).cfg).default_root).is_null()
    {
        free((*c).path as *mut libc::c_void);
        (*c).path = strdup((*(*(*(*c).w).s).cfg).default_root);
        (*c).path_sz = strlen((*c).path);
    }
    worker_process_client(c);
    http_client_reset(c);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn http_client_new(
    mut w: *mut worker,
    mut fd: libc::c_int,
    mut addr: in_addr_t,
) -> *mut http_client {
    let mut c: *mut http_client = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<http_client>() as libc::c_ulong,
    ) as *mut http_client;
    (*c).fd = fd;
    (*c).w = w;
    (*c).addr = addr;
    (*c).s = (*w).s;
    http_parser_init(&mut (*c).parser, HTTP_REQUEST);
    (*c).parser.data = c as *mut libc::c_void;
    (*c)
        .settings
        .on_url = Some(
        http_client_on_url
            as unsafe extern "C" fn(
                *mut http_parser,
                *const libc::c_char,
                size_t,
            ) -> libc::c_int,
    );
    (*c)
        .settings
        .on_query_string = Some(
        http_client_on_query_string
            as unsafe extern "C" fn(
                *mut http_parser,
                *const libc::c_char,
                size_t,
            ) -> libc::c_int,
    );
    (*c)
        .settings
        .on_body = Some(
        http_client_on_body
            as unsafe extern "C" fn(
                *mut http_parser,
                *const libc::c_char,
                size_t,
            ) -> libc::c_int,
    );
    (*c)
        .settings
        .on_message_complete = Some(
        http_client_on_message_complete
            as unsafe extern "C" fn(*mut http_parser) -> libc::c_int,
    );
    (*c)
        .settings
        .on_header_field = Some(
        http_client_on_header_name
            as unsafe extern "C" fn(
                *mut http_parser,
                *const libc::c_char,
                size_t,
            ) -> libc::c_int,
    );
    (*c)
        .settings
        .on_header_value = Some(
        http_client_on_header_value
            as unsafe extern "C" fn(
                *mut http_parser,
                *const libc::c_char,
                size_t,
            ) -> libc::c_int,
    );
    (*c).last_cb = LAST_CB_NONE;
    return c;
}
pub unsafe extern "C" fn http_client_reset(mut c: *mut http_client) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*c).header_count {
        free((*((*c).headers).offset(i as isize)).key as *mut libc::c_void);
        free((*((*c).headers).offset(i as isize)).val as *mut libc::c_void);
        i += 1;
        i;
    }
    free((*c).headers as *mut libc::c_void);
    (*c).headers = 0 as *mut http_header;
    (*c).header_count = 0 as libc::c_int;
    free((*c).body as *mut libc::c_void);
    (*c).body = 0 as *mut libc::c_char;
    (*c).body_sz = 0 as libc::c_int as size_t;
    free((*c).path as *mut libc::c_void);
    (*c).path = 0 as *mut libc::c_char;
    (*c).path_sz = 0 as libc::c_int as size_t;
    free((*c).type_0 as *mut libc::c_void);
    (*c).type_0 = 0 as *mut libc::c_char;
    free((*c).jsonp as *mut libc::c_void);
    (*c).jsonp = 0 as *mut libc::c_char;
    free((*c).filename as *mut libc::c_void);
    (*c).filename = 0 as *mut libc::c_char;
    (*c).request_sz = 0 as libc::c_int as size_t;
    (*c).last_cb = LAST_CB_NONE;
    if (*c).keep_alive as libc::c_int == 0 as libc::c_int {
        (*c).broken = 1 as libc::c_int as libc::c_char;
    }
}
pub unsafe extern "C" fn http_client_free(mut c: *mut http_client) {
    http_client_reset(c);
    free((*c).buffer as *mut libc::c_void);
    free(c as *mut libc::c_void);
}
pub unsafe extern "C" fn http_client_read(mut c: *mut http_client) -> libc::c_int {
    let mut buffer: [libc::c_char; 4096] = [0; 4096];
    let mut ret: libc::c_int = 0;
    ret = read(
        (*c).fd,
        buffer.as_mut_ptr() as *mut libc::c_void,
        ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong,
    ) as libc::c_int;
    if ret <= 0 as libc::c_int {
        if !((*c).reused_cmd).is_null() && !((*(*c).reused_cmd).ac).is_null() {
            let mut cmd: *mut cmd = (*c).reused_cmd;
            redisAsyncDisconnect((*(*c).reused_cmd).ac);
            (*cmd).ac = 0 as *mut redisAsyncContext;
            (*c).reused_cmd = 0 as *mut cmd;
            cmd_free(cmd);
        }
        if !((*c).last_cmd).is_null() {
            (*(*c).last_cmd).fd = -(1 as libc::c_int);
            (*(*c).last_cmd).http_client = 0 as *mut http_client;
            (*c).last_cmd = 0 as *mut cmd;
        }
        close((*c).fd);
        http_client_free(c);
        return CLIENT_DISCONNECTED as libc::c_int;
    }
    (*c)
        .buffer = realloc(
        (*c).buffer as *mut libc::c_void,
        ((*c).sz).wrapping_add(ret as libc::c_ulong),
    ) as *mut libc::c_char;
    if ((*c).buffer).is_null() {
        return CLIENT_OOM as libc::c_int;
    }
    memcpy(
        ((*c).buffer).offset((*c).sz as isize) as *mut libc::c_void,
        buffer.as_mut_ptr() as *const libc::c_void,
        ret as libc::c_ulong,
    );
    (*c)
        .sz = ((*c).sz as libc::c_ulong).wrapping_add(ret as libc::c_ulong) as size_t
        as size_t;
    (*c)
        .request_sz = ((*c).request_sz as libc::c_ulong)
        .wrapping_add(ret as libc::c_ulong) as size_t as size_t;
    return ret;
}
pub unsafe extern "C" fn http_client_remove_data(
    mut c: *mut http_client,
    mut sz: size_t,
) -> libc::c_int {
    let mut buffer: *mut libc::c_char = 0 as *mut libc::c_char;
    if (*c).sz < sz {
        return -(1 as libc::c_int);
    }
    buffer = malloc(((*c).sz).wrapping_sub(sz)) as *mut libc::c_char;
    if buffer.is_null() {
        (*c).failed_alloc = 1 as libc::c_int as libc::c_char;
        return -(1 as libc::c_int);
    }
    memcpy(
        buffer as *mut libc::c_void,
        ((*c).buffer).offset(sz as isize) as *const libc::c_void,
        ((*c).sz).wrapping_sub(sz),
    );
    free((*c).buffer as *mut libc::c_void);
    (*c).buffer = buffer;
    (*c).sz = ((*c).sz as libc::c_ulong).wrapping_sub(sz) as size_t as size_t;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn http_client_execute(mut c: *mut http_client) -> libc::c_int {
    let mut nparsed: libc::c_int = http_parser_execute(
        &mut (*c).parser,
        &mut (*c).settings,
        (*c).buffer,
        (*c).sz,
    ) as libc::c_int;
    if (*c).is_websocket == 0 {
        free((*c).buffer as *mut libc::c_void);
        (*c).buffer = 0 as *mut libc::c_char;
        (*c).sz = 0 as libc::c_int as size_t;
    }
    return nparsed;
}
pub unsafe extern "C" fn client_get_header(
    mut c: *mut http_client,
    mut key: *const libc::c_char,
) -> *const libc::c_char {
    let mut i: libc::c_int = 0;
    let mut sz: size_t = strlen(key);
    i = 0 as libc::c_int;
    while i < (*c).header_count {
        if sz == (*((*c).headers).offset(i as isize)).key_sz
            && strncasecmp(key, (*((*c).headers).offset(i as isize)).key, sz)
                == 0 as libc::c_int
        {
            return (*((*c).headers).offset(i as isize)).val;
        }
        i += 1;
        i;
    }
    return 0 as *const libc::c_char;
}
