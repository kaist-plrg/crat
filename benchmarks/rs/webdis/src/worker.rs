use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type dict;
    pub type sockadr;
    pub type acl;
    pub type event_base;
    pub type evbuffer;
    fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option::<
            unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
        >,
        __arg: *mut libc::c_void,
    ) -> libc::c_int;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn event_base_dispatch(_: *mut event_base) -> libc::c_int;
    fn event_add(ev: *mut event, timeout: *const timeval) -> libc::c_int;
    fn event_base_set(_: *mut event_base, _: *mut event) -> libc::c_int;
    fn http_client_free(c: *mut http_client);
    fn free(__ptr: *mut libc::c_void);
    fn ws_process_read_data(
        ws: *mut ws_client,
        out_processed: *mut libc::c_uint,
    ) -> ws_state;
    fn ws_close_if_able(ws: *mut ws_client);
    fn ws_handshake_reply(ws: *mut ws_client) -> libc::c_int;
    fn ws_client_new(http_client: *mut http_client) -> *mut ws_client;
    fn http_client_execute(c: *mut http_client) -> libc::c_int;
    fn http_client_read(c: *mut http_client) -> libc::c_int;
    fn event_set(
        _: *mut event,
        _: libc::c_int,
        _: libc::c_short,
        _: Option::<
            unsafe extern "C" fn(libc::c_int, libc::c_short, *mut libc::c_void) -> (),
        >,
        _: *mut libc::c_void,
    );
    fn event_base_new() -> *mut event_base;
    fn http_crossdomain(c: *mut http_client);
    fn http_send_error(
        c: *mut http_client,
        code: libc::c_short,
        msg: *const libc::c_char,
    );
    fn http_send_options(c: *mut http_client);
    fn cmd_run(
        w: *mut worker,
        client: *mut http_client,
        uri: *const libc::c_char,
        uri_len: size_t,
        body: *const libc::c_char,
        body_len: size_t,
    ) -> cmd_response_t;
    fn pool_new(w: *mut worker, count: libc::c_int) -> *mut pool;
    fn pool_connect(
        p: *mut pool,
        db_num: libc::c_int,
        attach: libc::c_int,
    ) -> *mut redisAsyncContext;
    fn slog(s: *mut server, level: log_level, body: *const libc::c_char, sz: size_t);
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn pipe(__pipedes: *mut libc::c_int) -> libc::c_int;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
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
pub type size_t = libc::c_ulong;
pub type pid_t = __pid_t;
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
pub union pthread_attr_t {
    pub __size: [libc::c_char; 56],
    pub __align: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
    pub __align: libc::c_long,
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
pub struct pool {
    pub w: *mut worker,
    pub cfg: *mut conf,
    pub ac: *mut *const redisAsyncContext,
    pub count: libc::c_int,
    pub cur: libc::c_int,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [libc::c_char; 14],
}
pub type sa_family_t = libc::c_ushort;
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
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
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
pub struct redisReadTask {
    pub type_0: libc::c_int,
    pub elements: libc::c_longlong,
    pub idx: libc::c_int,
    pub obj: *mut libc::c_void,
    pub parent: *mut redisReadTask,
    pub privdata: *mut libc::c_void,
}
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
pub type ssize_t = __ssize_t;
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
    pub log_fsync: C2RustUnnamed_4,
    pub hiredis_opts: C2RustUnnamed_3,
    pub default_root: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub keep_alive_sec: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
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
pub type gid_t = __gid_t;
pub type uid_t = __uid_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct auth {
    pub use_legacy_auth: libc::c_int,
    pub username: *mut libc::c_char,
    pub password: *mut libc::c_char,
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
    pub log: C2RustUnnamed_5,
    pub auth_log_mutex: pthread_mutex_t,
    pub auth_logged: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub self_0: pid_t,
    pub fd: libc::c_int,
    pub fsync_tv: timeval,
    pub fsync_ev: *mut event,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct event {
    pub ev_evcallback: event_callback,
    pub ev_timeout_pos: C2RustUnnamed_11,
    pub ev_fd: libc::c_int,
    pub ev_base: *mut event_base,
    pub ev_: C2RustUnnamed_6,
    pub ev_events: libc::c_short,
    pub ev_res: libc::c_short,
    pub ev_timeout: timeval,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_6 {
    pub ev_io: C2RustUnnamed_9,
    pub ev_signal: C2RustUnnamed_7,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub ev_signal_next: C2RustUnnamed_8,
    pub ev_ncalls: libc::c_short,
    pub ev_pncalls: *mut libc::c_short,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub le_next: *mut event,
    pub le_prev: *mut *mut event,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub ev_io_next: C2RustUnnamed_10,
    pub ev_timeout: timeval,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub le_next: *mut event,
    pub le_prev: *mut *mut event,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_11 {
    pub ev_next_with_common_timeout: C2RustUnnamed_12,
    pub min_heap_idx: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_12 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct event_callback {
    pub evcb_active_next: C2RustUnnamed_14,
    pub evcb_flags: libc::c_short,
    pub evcb_pri: uint8_t,
    pub evcb_closure: uint8_t,
    pub evcb_cb_union: C2RustUnnamed_13,
    pub evcb_arg: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_13 {
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
pub type uint8_t = __uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_14 {
    pub tqe_next: *mut event_callback,
    pub tqe_prev: *mut *mut event_callback,
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
pub type int64_t = __int64_t;
pub type uint32_t = __uint32_t;
pub type http_data_cb = Option::<
    unsafe extern "C" fn(*mut http_parser, *const libc::c_char, size_t) -> libc::c_int,
>;
pub type in_addr_t = uint32_t;
pub const WS_ERROR: ws_state = 0;
pub type ws_state = libc::c_uint;
pub const WS_MSG_COMPLETE: ws_state = 2;
pub const WS_READING: ws_state = 1;
pub const F_CONNECTION_CLOSE: flags = 4;
pub const CLIENT_OOM: client_error_t = -2;
pub type client_error_t = libc::c_int;
pub const CLIENT_DISCONNECTED: client_error_t = -1;
pub const CMD_REDIS_UNAVAIL: cmd_response_t = 3;
pub const CMD_PARAM_ERROR: cmd_response_t = 1;
pub const CMD_ACL_FAIL: cmd_response_t = 2;
pub type cmd_response_t = libc::c_uint;
pub const CMD_SENT: cmd_response_t = 0;
pub const HTTP_OPTIONS: http_method = 6;
pub const HTTP_PUT: http_method = 4;
pub const HTTP_POST: http_method = 3;
pub const HTTP_GET: http_method = 1;
pub type http_method = libc::c_uint;
pub const HTTP_UNSUBSCRIBE: http_method = 22;
pub const HTTP_SUBSCRIBE: http_method = 21;
pub const HTTP_NOTIFY: http_method = 20;
pub const HTTP_MSEARCH: http_method = 19;
pub const HTTP_MERGE: http_method = 18;
pub const HTTP_CHECKOUT: http_method = 17;
pub const HTTP_MKACTIVITY: http_method = 16;
pub const HTTP_REPORT: http_method = 15;
pub const HTTP_UNLOCK: http_method = 14;
pub const HTTP_PROPPATCH: http_method = 13;
pub const HTTP_PROPFIND: http_method = 12;
pub const HTTP_MOVE: http_method = 11;
pub const HTTP_MKCOL: http_method = 10;
pub const HTTP_LOCK: http_method = 9;
pub const HTTP_COPY: http_method = 8;
pub const HTTP_TRACE: http_method = 7;
pub const HTTP_CONNECT: http_method = 5;
pub const HTTP_HEAD: http_method = 2;
pub const HTTP_DELETE: http_method = 0;
pub type flags = libc::c_uint;
pub const F_SKIPBODY: flags = 32;
pub const F_UPGRADE: flags = 16;
pub const F_TRAILING: flags = 8;
pub const F_CONNECTION_KEEP_ALIVE: flags = 2;
pub const F_CHUNKED: flags = 1;
pub unsafe extern "C" fn worker_new(mut s: *mut server) -> *mut worker {
    let mut ret: libc::c_int = 0;
    let mut w: *mut worker = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<worker>() as libc::c_ulong,
    ) as *mut worker;
    (*w).s = s;
    ret = pipe(((*w).link).as_mut_ptr());
    (*w).pool = pool_new(w, (*(*s).cfg).pool_size_per_thread);
    return w;
}
pub unsafe extern "C" fn worker_can_read(
    mut fd: libc::c_int,
    mut event: libc::c_short,
    mut p: *mut libc::c_void,
) {
    let mut c: *mut http_client = p as *mut http_client;
    let mut ret: libc::c_int = 0;
    let mut nparsed: libc::c_int = 0;
    ret = http_client_read(c);
    if ret <= 0 as libc::c_int {
        if ret as client_error_t as libc::c_int == CLIENT_DISCONNECTED as libc::c_int {
            return
        } else if (*c).failed_alloc as libc::c_int != 0
            || ret as client_error_t as libc::c_int == CLIENT_OOM as libc::c_int
        {
            slog(
                (*(*c).w).s,
                WEBDIS_DEBUG,
                b"503\0" as *const u8 as *const libc::c_char,
                3 as libc::c_int as size_t,
            );
            http_send_error(
                c,
                503 as libc::c_int as libc::c_short,
                b"Service Unavailable\0" as *const u8 as *const libc::c_char,
            );
            return;
        }
    }
    if (*c).is_websocket == 0 {
        nparsed = http_client_execute(c);
        if (*c).failed_alloc != 0 {
            slog(
                (*(*c).w).s,
                WEBDIS_DEBUG,
                b"503\0" as *const u8 as *const libc::c_char,
                3 as libc::c_int as size_t,
            );
            http_send_error(
                c,
                503 as libc::c_int as libc::c_short,
                b"Service Unavailable\0" as *const u8 as *const libc::c_char,
            );
        } else if ((*c).parser).flags() as libc::c_int
            & F_CONNECTION_CLOSE as libc::c_int != 0
            && (*c).fully_read as libc::c_int != 0
        {
            (*c).broken = 1 as libc::c_int as libc::c_char;
        } else if (*c).is_websocket != 0 {
            (*c).ws = ws_client_new(c);
            if ((*c).ws).is_null() {
                (*c).broken = 1 as libc::c_int as libc::c_char;
            } else {
                free((*c).buffer as *mut libc::c_void);
                (*c).buffer = 0 as *mut libc::c_char;
                (*c).sz = 0 as libc::c_int as size_t;
                let mut reply_ret: libc::c_int = ws_handshake_reply((*c).ws);
                if reply_ret < 0 as libc::c_int {
                    (*(*c).ws).http_client = 0 as *mut http_client;
                    ws_close_if_able((*c).ws);
                    (*c).broken = 1 as libc::c_int as libc::c_char;
                } else {
                    let mut processed: libc::c_uint = 0 as libc::c_int as libc::c_uint;
                    let mut process_ret: libc::c_int = ws_process_read_data(
                        (*c).ws,
                        &mut processed,
                    ) as libc::c_int;
                    if process_ret == WS_ERROR as libc::c_int {
                        (*c).broken = 1 as libc::c_int as libc::c_char;
                    }
                }
            }
            free((*c).buffer as *mut libc::c_void);
            (*c).buffer = 0 as *mut libc::c_char;
            (*c).sz = 0 as libc::c_int as size_t;
        } else if nparsed != ret {
            slog(
                (*(*c).w).s,
                WEBDIS_DEBUG,
                b"400\0" as *const u8 as *const libc::c_char,
                3 as libc::c_int as size_t,
            );
            http_send_error(
                c,
                400 as libc::c_int as libc::c_short,
                b"Bad Request\0" as *const u8 as *const libc::c_char,
            );
        } else if (*c).request_sz > (*(*(*c).s).cfg).http_max_request_size {
            slog(
                (*(*c).w).s,
                WEBDIS_DEBUG,
                b"413\0" as *const u8 as *const libc::c_char,
                3 as libc::c_int as size_t,
            );
            http_send_error(
                c,
                413 as libc::c_int as libc::c_short,
                b"Request Entity Too Large\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    if (*c).broken != 0 {
        if (*c).is_websocket != 0 {
            close((*c).fd);
        }
        http_client_free(c);
    } else if (*c).is_websocket == 0 {
        worker_monitor_input(c);
    }
}
pub unsafe extern "C" fn worker_monitor_input(mut c: *mut http_client) {
    event_set(
        &mut (*c).ev,
        (*c).fd,
        0x2 as libc::c_int as libc::c_short,
        Some(
            worker_can_read
                as unsafe extern "C" fn(
                    libc::c_int,
                    libc::c_short,
                    *mut libc::c_void,
                ) -> (),
        ),
        c as *mut libc::c_void,
    );
    event_base_set((*(*c).w).base, &mut (*c).ev);
    event_add(&mut (*c).ev, 0 as *const timeval);
}
unsafe extern "C" fn worker_on_new_client(
    mut pipefd: libc::c_int,
    mut event: libc::c_short,
    mut ptr: *mut libc::c_void,
) {
    let mut c: *mut http_client = 0 as *mut http_client;
    let mut addr: libc::c_ulong = 0;
    let mut ret: libc::c_int = read(
        pipefd,
        &mut addr as *mut libc::c_ulong as *mut libc::c_void,
        ::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong,
    ) as libc::c_int;
    if ret as libc::c_ulong == ::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong {
        c = addr as *mut http_client;
        worker_monitor_input(c);
    }
}
unsafe extern "C" fn worker_pool_connect(mut w: *mut worker) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*(*w).pool).count {
        pool_connect((*w).pool, (*(*(*w).s).cfg).database, 1 as libc::c_int);
        i += 1;
        i;
    }
}
unsafe extern "C" fn worker_main(mut p: *mut libc::c_void) -> *mut libc::c_void {
    let mut w: *mut worker = p as *mut worker;
    let mut ev: event = event {
        ev_evcallback: event_callback {
            evcb_active_next: C2RustUnnamed_14 {
                tqe_next: 0 as *mut event_callback,
                tqe_prev: 0 as *mut *mut event_callback,
            },
            evcb_flags: 0,
            evcb_pri: 0,
            evcb_closure: 0,
            evcb_cb_union: C2RustUnnamed_13 {
                evcb_callback: None,
            },
            evcb_arg: 0 as *mut libc::c_void,
        },
        ev_timeout_pos: C2RustUnnamed_11 {
            ev_next_with_common_timeout: C2RustUnnamed_12 {
                tqe_next: 0 as *mut event,
                tqe_prev: 0 as *mut *mut event,
            },
        },
        ev_fd: 0,
        ev_base: 0 as *mut event_base,
        ev_: C2RustUnnamed_6 {
            ev_io: C2RustUnnamed_9 {
                ev_io_next: C2RustUnnamed_10 {
                    le_next: 0 as *mut event,
                    le_prev: 0 as *mut *mut event,
                },
                ev_timeout: timeval { tv_sec: 0, tv_usec: 0 },
            },
        },
        ev_events: 0,
        ev_res: 0,
        ev_timeout: timeval { tv_sec: 0, tv_usec: 0 },
    };
    (*w).base = event_base_new();
    event_set(
        &mut ev,
        (*w).link[0 as libc::c_int as usize],
        (0x2 as libc::c_int | 0x10 as libc::c_int) as libc::c_short,
        Some(
            worker_on_new_client
                as unsafe extern "C" fn(
                    libc::c_int,
                    libc::c_short,
                    *mut libc::c_void,
                ) -> (),
        ),
        w as *mut libc::c_void,
    );
    event_base_set((*w).base, &mut ev);
    event_add(&mut ev, 0 as *const timeval);
    worker_pool_connect(w);
    event_base_dispatch((*w).base);
    return 0 as *mut libc::c_void;
}
pub unsafe extern "C" fn worker_start(mut w: *mut worker) {
    pthread_create(
        &mut (*w).thread,
        0 as *const pthread_attr_t,
        Some(
            worker_main as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
        ),
        w as *mut libc::c_void,
    );
}
pub unsafe extern "C" fn worker_add_client(mut w: *mut worker, mut c: *mut http_client) {
    let mut addr: libc::c_ulong = c as libc::c_ulong;
    let mut ret: libc::c_int = write(
        (*w).link[1 as libc::c_int as usize],
        &mut addr as *mut libc::c_ulong as *const libc::c_void,
        ::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong,
    ) as libc::c_int;
}
pub unsafe extern "C" fn worker_process_client(mut c: *mut http_client) {
    let mut w: *mut worker = (*c).w;
    let mut ret: cmd_response_t = CMD_PARAM_ERROR;
    match (*c).parser.method as libc::c_int {
        1 => {
            if (*c).path_sz == 16 as libc::c_int as libc::c_ulong
                && memcmp(
                    (*c).path as *const libc::c_void,
                    b"/crossdomain.xml\0" as *const u8 as *const libc::c_char
                        as *const libc::c_void,
                    16 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
            {
                http_crossdomain(c);
                return;
            }
            slog((*w).s, WEBDIS_DEBUG, (*c).path, (*c).path_sz);
            ret = cmd_run(
                (*c).w,
                c,
                ((*c).path).offset(1 as libc::c_int as isize),
                ((*c).path_sz).wrapping_sub(1 as libc::c_int as libc::c_ulong),
                0 as *const libc::c_char,
                0 as libc::c_int as size_t,
            );
        }
        3 => {
            slog((*w).s, WEBDIS_DEBUG, (*c).path, (*c).path_sz);
            ret = cmd_run(
                (*c).w,
                c,
                (*c).body,
                (*c).body_sz,
                0 as *const libc::c_char,
                0 as libc::c_int as size_t,
            );
        }
        4 => {
            slog((*w).s, WEBDIS_DEBUG, (*c).path, (*c).path_sz);
            ret = cmd_run(
                (*c).w,
                c,
                ((*c).path).offset(1 as libc::c_int as isize),
                ((*c).path_sz).wrapping_sub(1 as libc::c_int as libc::c_ulong),
                (*c).body,
                (*c).body_sz,
            );
        }
        6 => {
            http_send_options(c);
            return;
        }
        _ => {
            slog(
                (*w).s,
                WEBDIS_DEBUG,
                b"405\0" as *const u8 as *const libc::c_char,
                3 as libc::c_int as size_t,
            );
            http_send_error(
                c,
                405 as libc::c_int as libc::c_short,
                b"Method Not Allowed\0" as *const u8 as *const libc::c_char,
            );
            return;
        }
    }
    match ret as libc::c_uint {
        2 | 1 => {
            slog(
                (*w).s,
                WEBDIS_DEBUG,
                b"403\0" as *const u8 as *const libc::c_char,
                3 as libc::c_int as size_t,
            );
            http_send_error(
                c,
                403 as libc::c_int as libc::c_short,
                b"Forbidden\0" as *const u8 as *const libc::c_char,
            );
        }
        3 => {
            slog(
                (*w).s,
                WEBDIS_DEBUG,
                b"503\0" as *const u8 as *const libc::c_char,
                3 as libc::c_int as size_t,
            );
            http_send_error(
                c,
                503 as libc::c_int as libc::c_short,
                b"Service Unavailable\0" as *const u8 as *const libc::c_char,
            );
        }
        _ => {}
    };
}
