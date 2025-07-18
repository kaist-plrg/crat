use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type dict;
    pub type sockadr;
    pub type event_base;
    pub type evbuffer;
    fn redisAsyncCommandArgv(
        ac: *mut redisAsyncContext,
        fn_0: Option::<redisCallbackFn>,
        privdata: *mut libc::c_void,
        argc: libc::c_int,
        argv: *mut *const libc::c_char,
        argvlen: *const size_t,
    ) -> libc::c_int;
    fn free(__ptr: *mut libc::c_void);
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn acl_allow_command(
        cmd: *mut cmd,
        cfg: *mut conf,
        client: *mut http_client,
    ) -> libc::c_int;
    fn pool_free_context(ac: *mut redisAsyncContext);
    fn pool_connect(
        p: *mut pool,
        db_num: libc::c_int,
        attach: libc::c_int,
    ) -> *mut redisAsyncContext;
    fn pool_get_context(p: *mut pool) -> *const redisAsyncContext;
    fn json_reply(
        c: *mut redisAsyncContext,
        r: *mut libc::c_void,
        privdata: *mut libc::c_void,
    );
    fn raw_reply(
        c: *mut redisAsyncContext,
        r: *mut libc::c_void,
        privdata: *mut libc::c_void,
    );
    fn custom_type_reply(
        c: *mut redisAsyncContext,
        r: *mut libc::c_void,
        privdata: *mut libc::c_void,
    );
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
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strcasecmp(__s1: *const libc::c_char, __s2: *const libc::c_char) -> libc::c_int;
    fn strncasecmp(
        __s1: *const libc::c_char,
        __s2: *const libc::c_char,
        __n: size_t,
    ) -> libc::c_int;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type ssize_t = __ssize_t;
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
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type gid_t = __gid_t;
pub type uid_t = __uid_t;
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
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
    pub __align: libc::c_long,
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
pub type in_addr_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct event {
    pub ev_evcallback: event_callback,
    pub ev_timeout_pos: C2RustUnnamed_8,
    pub ev_fd: libc::c_int,
    pub ev_base: *mut event_base,
    pub ev_: C2RustUnnamed_3,
    pub ev_events: libc::c_short,
    pub ev_res: libc::c_short,
    pub ev_timeout: timeval,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub ev_io: C2RustUnnamed_6,
    pub ev_signal: C2RustUnnamed_4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub ev_signal_next: C2RustUnnamed_5,
    pub ev_ncalls: libc::c_short,
    pub ev_pncalls: *mut libc::c_short,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub le_next: *mut event,
    pub le_prev: *mut *mut event,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub ev_io_next: C2RustUnnamed_7,
    pub ev_timeout: timeval,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub le_next: *mut event,
    pub le_prev: *mut *mut event,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_8 {
    pub ev_next_with_common_timeout: C2RustUnnamed_9,
    pub min_heap_idx: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct event_callback {
    pub evcb_active_next: C2RustUnnamed_11,
    pub evcb_flags: libc::c_short,
    pub evcb_pri: uint8_t,
    pub evcb_closure: uint8_t,
    pub evcb_cb_union: C2RustUnnamed_10,
    pub evcb_arg: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_10 {
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
pub struct C2RustUnnamed_11 {
    pub tqe_next: *mut event_callback,
    pub tqe_prev: *mut *mut event_callback,
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
    pub log_fsync: C2RustUnnamed_13,
    pub hiredis_opts: C2RustUnnamed_12,
    pub default_root: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_12 {
    pub keep_alive_sec: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_13 {
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
pub struct acl {
    pub cidr: C2RustUnnamed_14,
    pub http_basic_auth: *mut libc::c_char,
    pub enabled: acl_commands,
    pub disabled: acl_commands,
    pub next: *mut acl,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct acl_commands {
    pub count: libc::c_uint,
    pub commands: *mut *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_14 {
    pub enabled: libc::c_int,
    pub subnet: in_addr_t,
    pub mask: in_addr_t,
}
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
    pub log: C2RustUnnamed_15,
    pub auth_log_mutex: pthread_mutex_t,
    pub auth_logged: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_15 {
    pub self_0: pid_t,
    pub fd: libc::c_int,
    pub fsync_tv: timeval,
    pub fsync_ev: *mut event,
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
pub type http_data_cb = Option::<
    unsafe extern "C" fn(*mut http_parser, *const libc::c_char, size_t) -> libc::c_int,
>;
pub type formatting_fun = Option::<
    unsafe extern "C" fn(
        *mut redisAsyncContext,
        *mut libc::c_void,
        *mut libc::c_void,
    ) -> (),
>;
pub type cmd_response_t = libc::c_uint;
pub const CMD_REDIS_UNAVAIL: cmd_response_t = 3;
pub const CMD_ACL_FAIL: cmd_response_t = 2;
pub const CMD_PARAM_ERROR: cmd_response_t = 1;
pub const CMD_SENT: cmd_response_t = 0;
pub const _ISxdigit: C2RustUnnamed_16 = 4096;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct reply_format {
    pub s: *const libc::c_char,
    pub sz: size_t,
    pub f: formatting_fun,
    pub ct: *const libc::c_char,
}
pub type C2RustUnnamed_16 = libc::c_uint;
pub const _ISalnum: C2RustUnnamed_16 = 8;
pub const _ISpunct: C2RustUnnamed_16 = 4;
pub const _IScntrl: C2RustUnnamed_16 = 2;
pub const _ISblank: C2RustUnnamed_16 = 1;
pub const _ISgraph: C2RustUnnamed_16 = 32768;
pub const _ISprint: C2RustUnnamed_16 = 16384;
pub const _ISspace: C2RustUnnamed_16 = 8192;
pub const _ISdigit: C2RustUnnamed_16 = 2048;
pub const _ISalpha: C2RustUnnamed_16 = 1024;
pub const _ISlower: C2RustUnnamed_16 = 512;
pub const _ISupper: C2RustUnnamed_16 = 256;
pub unsafe extern "C" fn cmd_new(
    mut client: *mut http_client,
    mut count: libc::c_int,
) -> *mut cmd {
    let mut c: *mut cmd = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<cmd>() as libc::c_ulong,
    ) as *mut cmd;
    (*c).count = count;
    (*c).http_client = client;
    if !client.is_null() {
        (*client).last_cmd = c;
    }
    (*c)
        .argv = calloc(
        count as libc::c_ulong,
        ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
    ) as *mut *mut libc::c_char;
    (*c)
        .argv_len = calloc(
        count as libc::c_ulong,
        ::std::mem::size_of::<size_t>() as libc::c_ulong,
    ) as *mut size_t;
    return c;
}
pub unsafe extern "C" fn cmd_free_argv(mut c: *mut cmd) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*c).count {
        free(*((*c).argv).offset(i as isize) as *mut libc::c_void);
        i += 1;
        i;
    }
    free((*c).argv as *mut libc::c_void);
    free((*c).argv_len as *mut libc::c_void);
}
pub unsafe extern "C" fn cmd_free(mut c: *mut cmd) {
    if c.is_null() {
        return;
    }
    free((*c).jsonp as *mut libc::c_void);
    free((*c).separator as *mut libc::c_void);
    free((*c).if_none_match as *mut libc::c_void);
    if (*c).mime_free != 0 {
        free((*c).mime as *mut libc::c_void);
    }
    if !((*c).http_client).is_null() && (*(*c).http_client).last_cmd == c {
        (*(*c).http_client).last_cmd = 0 as *mut cmd;
    }
    if !((*c).ac).is_null()
        && ((*c).database != (*(*(*(*c).w).s).cfg).database || cmd_is_subscribe(c) != 0)
    {
        pool_free_context((*c).ac);
    }
    cmd_free_argv(c);
    free(c as *mut libc::c_void);
}
unsafe extern "C" fn decode_uri(
    mut uri: *const libc::c_char,
    mut length: size_t,
    mut out_len: *mut size_t,
    mut always_decode_plus: libc::c_int,
) -> *mut libc::c_char {
    let mut c: libc::c_char = 0;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut in_query: libc::c_int = always_decode_plus;
    let mut ret: *mut libc::c_char = malloc(length) as *mut libc::c_char;
    j = 0 as libc::c_int as size_t;
    i = j;
    while i < length {
        c = *uri.offset(i as isize);
        if c as libc::c_int == '?' as i32 {
            in_query = 1 as libc::c_int;
        } else if c as libc::c_int == '+' as i32 && in_query != 0 {
            c = ' ' as i32 as libc::c_char;
        } else if c as libc::c_int == '%' as i32
            && *(*__ctype_b_loc())
                .offset(
                    *uri
                        .offset(
                            i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        ) as libc::c_uchar as libc::c_int as isize,
                ) as libc::c_int
                & _ISxdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
            && *(*__ctype_b_loc())
                .offset(
                    *uri
                        .offset(
                            i.wrapping_add(2 as libc::c_int as libc::c_ulong) as isize,
                        ) as libc::c_uchar as libc::c_int as isize,
                ) as libc::c_int
                & _ISxdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
        {
            let mut tmp: [libc::c_char; 3] = [
                *uri.offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize),
                *uri.offset(i.wrapping_add(2 as libc::c_int as libc::c_ulong) as isize),
                '\0' as i32 as libc::c_char,
            ];
            c = strtol(tmp.as_mut_ptr(), 0 as *mut *mut libc::c_char, 16 as libc::c_int)
                as libc::c_char;
            i = (i as libc::c_ulong).wrapping_add(2 as libc::c_int as libc::c_ulong)
                as size_t as size_t;
        }
        let fresh0 = j;
        j = j.wrapping_add(1);
        *ret.offset(fresh0 as isize) = c;
        i = i.wrapping_add(1);
        i;
    }
    *out_len = j;
    return ret;
}
pub unsafe extern "C" fn cmd_setup(mut cmd: *mut cmd, mut client: *mut http_client) {
    let mut i: libc::c_int = 0;
    (*cmd).keep_alive = (*client).keep_alive as libc::c_int;
    (*cmd).w = (*client).w;
    i = 0 as libc::c_int;
    while i < (*client).header_count {
        if strcasecmp(
            (*((*client).headers).offset(i as isize)).key,
            b"If-None-Match\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            (*cmd)
                .if_none_match = calloc(
                (1 as libc::c_int as libc::c_ulong)
                    .wrapping_add((*((*client).headers).offset(i as isize)).val_sz),
                1 as libc::c_int as libc::c_ulong,
            ) as *mut libc::c_char;
            memcpy(
                (*cmd).if_none_match as *mut libc::c_void,
                (*((*client).headers).offset(i as isize)).val as *const libc::c_void,
                (*((*client).headers).offset(i as isize)).val_sz,
            );
        } else if strcasecmp(
            (*((*client).headers).offset(i as isize)).key,
            b"Connection\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
            && strcasecmp(
                (*((*client).headers).offset(i as isize)).val,
                b"Keep-Alive\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
        {
            (*cmd).keep_alive = 1 as libc::c_int;
        }
        i += 1;
        i;
    }
    if !((*client).type_0).is_null() {
        (*cmd).mime = (*client).type_0;
        (*cmd).mime_free = 1 as libc::c_int;
        (*client).type_0 = 0 as *mut libc::c_char;
    }
    if !((*client).jsonp).is_null() {
        (*cmd).jsonp = (*client).jsonp;
        (*client).jsonp = 0 as *mut libc::c_char;
    }
    if !((*client).separator).is_null() {
        (*cmd).separator = (*client).separator;
        (*client).separator = 0 as *mut libc::c_char;
    }
    if !((*client).filename).is_null() {
        (*cmd).filename = (*client).filename;
        (*client).filename = 0 as *mut libc::c_char;
    }
    (*cmd).fd = (*client).fd;
    (*cmd).http_version = (*client).http_version as libc::c_int;
}
pub unsafe extern "C" fn cmd_run(
    mut w: *mut worker,
    mut client: *mut http_client,
    mut uri: *const libc::c_char,
    mut uri_len: size_t,
    mut body: *const libc::c_char,
    mut body_len: size_t,
) -> cmd_response_t {
    let mut qmark: *mut libc::c_char = memchr(
        uri as *const libc::c_void,
        '?' as i32,
        uri_len,
    ) as *mut libc::c_char;
    let mut slash: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut cmd_name: *const libc::c_char = uri;
    let mut cmd_len: libc::c_int = 0;
    let mut param_count: libc::c_int = 0 as libc::c_int;
    let mut cur_param: libc::c_int = 1 as libc::c_int;
    let mut cmd: *mut cmd = 0 as *mut cmd;
    let mut f_format: formatting_fun = None;
    if !qmark.is_null() {
        uri_len = qmark.offset_from(uri) as libc::c_long as size_t;
    }
    p = uri;
    while !p.is_null() && p < uri.offset(uri_len as isize) {
        p = memchr(
            p.offset(1 as libc::c_int as isize) as *const libc::c_void,
            '/' as i32,
            uri_len
                .wrapping_sub(
                    p.offset(1 as libc::c_int as isize).offset_from(uri) as libc::c_long
                        as libc::c_ulong,
                ),
        ) as *const libc::c_char;
        param_count += 1;
        param_count;
    }
    if !body.is_null() && body_len != 0 {
        param_count += 1;
        param_count;
    }
    if param_count == 0 as libc::c_int {
        return CMD_PARAM_ERROR;
    }
    cmd = cmd_new(client, param_count);
    (*cmd).fd = (*client).fd;
    (*cmd).database = (*(*(*w).s).cfg).database;
    uri_len = cmd_select_format(client, cmd, uri, uri_len, &mut f_format) as size_t;
    cmd_setup(cmd, client);
    slash = memchr(uri as *const libc::c_void, '/' as i32, uri_len) as *mut libc::c_char;
    if !slash.is_null() {
        let mut has_db: libc::c_int = 1 as libc::c_int;
        let mut db_num: libc::c_int = 0 as libc::c_int;
        p = uri;
        while p < slash as *const libc::c_char {
            if (*p as libc::c_int) < '0' as i32 || *p as libc::c_int > '9' as i32 {
                has_db = 0 as libc::c_int;
                break;
            } else {
                db_num = db_num * 10 as libc::c_int + (*p as libc::c_int - '0' as i32);
                p = p.offset(1);
                p;
            }
        }
        if has_db != 0 {
            let mut next: *mut libc::c_char = 0 as *mut libc::c_char;
            (*cmd).database = db_num;
            (*cmd).count -= 1;
            (*cmd).count;
            cmd_name = slash.offset(1 as libc::c_int as isize);
            next = memchr(
                cmd_name as *const libc::c_void,
                '/' as i32,
                uri_len
                    .wrapping_sub(
                        slash.offset_from(uri) as libc::c_long as libc::c_ulong,
                    ),
            ) as *mut libc::c_char;
            if !next.is_null() {
                cmd_len = next.offset_from(cmd_name) as libc::c_long as libc::c_int;
            } else {
                cmd_len = uri_len
                    .wrapping_sub(
                        (slash.offset_from(uri) as libc::c_long
                            + 1 as libc::c_int as libc::c_long) as libc::c_ulong,
                    ) as libc::c_int;
            }
        } else {
            cmd_len = slash.offset_from(uri) as libc::c_long as libc::c_int;
        }
    } else {
        cmd_len = uri_len as libc::c_int;
    }
    let ref mut fresh1 = *((*cmd).argv).offset(0 as libc::c_int as isize);
    *fresh1 = malloc(cmd_len as libc::c_ulong) as *mut libc::c_char;
    memcpy(
        *((*cmd).argv).offset(0 as libc::c_int as isize) as *mut libc::c_void,
        cmd_name as *const libc::c_void,
        cmd_len as libc::c_ulong,
    );
    *((*cmd).argv_len).offset(0 as libc::c_int as isize) = cmd_len as size_t;
    if acl_allow_command(cmd, (*(*w).s).cfg, client) == 0 {
        cmd_free(cmd);
        return CMD_ACL_FAIL;
    }
    if cmd_is_subscribe(cmd) != 0 {
        (*cmd).ac = pool_connect((*w).pool, (*cmd).database, 0 as libc::c_int);
        (*client).reused_cmd = cmd;
        (*cmd).pub_sub_client = client;
    } else if (*cmd).database != (*(*(*w).s).cfg).database {
        (*cmd).ac = pool_connect((*w).pool, (*cmd).database, 0 as libc::c_int);
    } else {
        (*cmd).ac = pool_get_context((*w).pool) as *mut redisAsyncContext;
    }
    if slash.is_null() {
        if ((*cmd).ac).is_null() {
            cmd_free(cmd);
            return CMD_REDIS_UNAVAIL;
        }
        redisAsyncCommandArgv(
            (*cmd).ac,
            f_format,
            cmd as *mut libc::c_void,
            1 as libc::c_int,
            (*cmd).argv as *mut *const libc::c_char,
            (*cmd).argv_len,
        );
        return CMD_SENT;
    }
    p = cmd_name.offset(cmd_len as isize).offset(1 as libc::c_int as isize);
    while p < uri.offset(uri_len as isize) {
        let mut arg: *const libc::c_char = p;
        let mut arg_len: libc::c_int = 0;
        let mut next_0: *mut libc::c_char = memchr(
            arg as *const libc::c_void,
            '/' as i32,
            uri_len.wrapping_sub(arg.offset_from(uri) as libc::c_long as libc::c_ulong),
        ) as *mut libc::c_char;
        if next_0.is_null() || next_0 > uri.offset(uri_len as isize) as *mut libc::c_char
        {
            p = uri.offset(uri_len as isize);
            arg_len = p.offset_from(arg) as libc::c_long as libc::c_int;
        } else {
            arg_len = next_0.offset_from(arg) as libc::c_long as libc::c_int;
            p = next_0.offset(1 as libc::c_int as isize);
        }
        let ref mut fresh2 = *((*cmd).argv).offset(cur_param as isize);
        *fresh2 = decode_uri(
            arg,
            arg_len as size_t,
            &mut *((*cmd).argv_len).offset(cur_param as isize),
            1 as libc::c_int,
        );
        cur_param += 1;
        cur_param;
    }
    if !body.is_null() && body_len != 0 {
        let ref mut fresh3 = *((*cmd).argv).offset(cur_param as isize);
        *fresh3 = malloc(body_len) as *mut libc::c_char;
        memcpy(
            *((*cmd).argv).offset(cur_param as isize) as *mut libc::c_void,
            body as *const libc::c_void,
            body_len,
        );
        *((*cmd).argv_len).offset(cur_param as isize) = body_len;
    }
    if !((*cmd).ac).is_null() {
        cmd_send(cmd, f_format);
        return CMD_SENT;
    }
    cmd_free(cmd);
    (*client).reused_cmd = 0 as *mut cmd;
    return CMD_REDIS_UNAVAIL;
}
pub unsafe extern "C" fn cmd_send(mut cmd: *mut cmd, mut f_format: formatting_fun) {
    redisAsyncCommandArgv(
        (*cmd).ac,
        f_format,
        cmd as *mut libc::c_void,
        (*cmd).count,
        (*cmd).argv as *mut *const libc::c_char,
        (*cmd).argv_len,
    );
}
pub unsafe extern "C" fn cmd_select_format(
    mut client: *mut http_client,
    mut cmd: *mut cmd,
    mut uri: *const libc::c_char,
    mut uri_len: size_t,
    mut f_format: *mut formatting_fun,
) -> libc::c_int {
    let mut ext: *const libc::c_char = 0 as *const libc::c_char;
    let mut ext_len: libc::c_int = -(1 as libc::c_int);
    let mut i: libc::c_uint = 0;
    let mut found: libc::c_int = 0 as libc::c_int;
    let mut funs: [reply_format; 12] = [
        {
            let mut init = reply_format {
                s: b"json\0" as *const u8 as *const libc::c_char,
                sz: 4 as libc::c_int as size_t,
                f: Some(
                    json_reply
                        as unsafe extern "C" fn(
                            *mut redisAsyncContext,
                            *mut libc::c_void,
                            *mut libc::c_void,
                        ) -> (),
                ),
                ct: b"application/json\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = reply_format {
                s: b"raw\0" as *const u8 as *const libc::c_char,
                sz: 3 as libc::c_int as size_t,
                f: Some(
                    raw_reply
                        as unsafe extern "C" fn(
                            *mut redisAsyncContext,
                            *mut libc::c_void,
                            *mut libc::c_void,
                        ) -> (),
                ),
                ct: b"binary/octet-stream\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = reply_format {
                s: b"bin\0" as *const u8 as *const libc::c_char,
                sz: 3 as libc::c_int as size_t,
                f: Some(
                    custom_type_reply
                        as unsafe extern "C" fn(
                            *mut redisAsyncContext,
                            *mut libc::c_void,
                            *mut libc::c_void,
                        ) -> (),
                ),
                ct: b"binary/octet-stream\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = reply_format {
                s: b"txt\0" as *const u8 as *const libc::c_char,
                sz: 3 as libc::c_int as size_t,
                f: Some(
                    custom_type_reply
                        as unsafe extern "C" fn(
                            *mut redisAsyncContext,
                            *mut libc::c_void,
                            *mut libc::c_void,
                        ) -> (),
                ),
                ct: b"text/plain\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = reply_format {
                s: b"html\0" as *const u8 as *const libc::c_char,
                sz: 4 as libc::c_int as size_t,
                f: Some(
                    custom_type_reply
                        as unsafe extern "C" fn(
                            *mut redisAsyncContext,
                            *mut libc::c_void,
                            *mut libc::c_void,
                        ) -> (),
                ),
                ct: b"text/html\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = reply_format {
                s: b"xhtml\0" as *const u8 as *const libc::c_char,
                sz: 5 as libc::c_int as size_t,
                f: Some(
                    custom_type_reply
                        as unsafe extern "C" fn(
                            *mut redisAsyncContext,
                            *mut libc::c_void,
                            *mut libc::c_void,
                        ) -> (),
                ),
                ct: b"application/xhtml+xml\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = reply_format {
                s: b"xml\0" as *const u8 as *const libc::c_char,
                sz: 3 as libc::c_int as size_t,
                f: Some(
                    custom_type_reply
                        as unsafe extern "C" fn(
                            *mut redisAsyncContext,
                            *mut libc::c_void,
                            *mut libc::c_void,
                        ) -> (),
                ),
                ct: b"text/xml\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = reply_format {
                s: b"png\0" as *const u8 as *const libc::c_char,
                sz: 3 as libc::c_int as size_t,
                f: Some(
                    custom_type_reply
                        as unsafe extern "C" fn(
                            *mut redisAsyncContext,
                            *mut libc::c_void,
                            *mut libc::c_void,
                        ) -> (),
                ),
                ct: b"image/png\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = reply_format {
                s: b"jpg\0" as *const u8 as *const libc::c_char,
                sz: 3 as libc::c_int as size_t,
                f: Some(
                    custom_type_reply
                        as unsafe extern "C" fn(
                            *mut redisAsyncContext,
                            *mut libc::c_void,
                            *mut libc::c_void,
                        ) -> (),
                ),
                ct: b"image/jpeg\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = reply_format {
                s: b"jpeg\0" as *const u8 as *const libc::c_char,
                sz: 4 as libc::c_int as size_t,
                f: Some(
                    custom_type_reply
                        as unsafe extern "C" fn(
                            *mut redisAsyncContext,
                            *mut libc::c_void,
                            *mut libc::c_void,
                        ) -> (),
                ),
                ct: b"image/jpeg\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = reply_format {
                s: b"js\0" as *const u8 as *const libc::c_char,
                sz: 2 as libc::c_int as size_t,
                f: Some(
                    json_reply
                        as unsafe extern "C" fn(
                            *mut redisAsyncContext,
                            *mut libc::c_void,
                            *mut libc::c_void,
                        ) -> (),
                ),
                ct: b"application/javascript\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = reply_format {
                s: b"css\0" as *const u8 as *const libc::c_char,
                sz: 3 as libc::c_int as size_t,
                f: Some(
                    custom_type_reply
                        as unsafe extern "C" fn(
                            *mut redisAsyncContext,
                            *mut libc::c_void,
                            *mut libc::c_void,
                        ) -> (),
                ),
                ct: b"text/css\0" as *const u8 as *const libc::c_char,
            };
            init
        },
    ];
    *f_format = Some(
        json_reply
            as unsafe extern "C" fn(
                *mut redisAsyncContext,
                *mut libc::c_void,
                *mut libc::c_void,
            ) -> (),
    );
    ext = uri.offset(uri_len as isize).offset(-(1 as libc::c_int as isize));
    while ext != uri && *ext as libc::c_int != '/' as i32 {
        if *ext as libc::c_int == '.' as i32 {
            ext = ext.offset(1);
            ext;
            ext_len = uri.offset(uri_len as isize).offset_from(ext) as libc::c_long
                as libc::c_int;
            break;
        } else {
            ext = ext.offset(-1);
            ext;
        }
    }
    if ext_len == 0 {
        return uri_len as libc::c_int;
    }
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong)
        < (::std::mem::size_of::<[reply_format; 12]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<reply_format>() as libc::c_ulong)
    {
        if ext_len == funs[i as usize].sz as libc::c_int
            && strncmp(ext, funs[i as usize].s, ext_len as libc::c_ulong)
                == 0 as libc::c_int
        {
            if (*cmd).mime_free != 0 {
                free((*cmd).mime as *mut libc::c_void);
            }
            (*cmd).mime = funs[i as usize].ct as *mut libc::c_char;
            (*cmd).mime_free = 0 as libc::c_int;
            *f_format = funs[i as usize].f;
            found = 1 as libc::c_int;
        }
        i = i.wrapping_add(1);
        i;
    }
    if !((*client).type_0).is_null() {
        *f_format = Some(
            custom_type_reply
                as unsafe extern "C" fn(
                    *mut redisAsyncContext,
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> (),
        );
    }
    if found != 0 {
        return uri_len
            .wrapping_sub(ext_len as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int
    } else {
        return uri_len as libc::c_int
    };
}
pub unsafe extern "C" fn cmd_is_subscribe(mut cmd: *mut cmd) -> libc::c_int {
    if !((*cmd).pub_sub_client).is_null() || cmd_is_subscribe_args(cmd) != 0 {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn cmd_is_subscribe_args(mut cmd: *mut cmd) -> libc::c_int {
    if (*cmd).count >= 2 as libc::c_int
        && (*((*cmd).argv_len).offset(0 as libc::c_int as isize)
            == 9 as libc::c_int as libc::c_ulong
            && strncasecmp(
                *((*cmd).argv).offset(0 as libc::c_int as isize),
                b"subscribe\0" as *const u8 as *const libc::c_char,
                9 as libc::c_int as size_t,
            ) == 0 as libc::c_int
            || *((*cmd).argv_len).offset(0 as libc::c_int as isize)
                == 10 as libc::c_int as libc::c_ulong
                && strncasecmp(
                    *((*cmd).argv).offset(0 as libc::c_int as isize),
                    b"psubscribe\0" as *const u8 as *const libc::c_char,
                    10 as libc::c_int as size_t,
                ) == 0 as libc::c_int)
    {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn cmd_is_unsubscribe_args(mut cmd: *mut cmd) -> libc::c_int {
    if (*cmd).count >= 2 as libc::c_int
        && (*((*cmd).argv_len).offset(0 as libc::c_int as isize)
            == 11 as libc::c_int as libc::c_ulong
            && strncasecmp(
                *((*cmd).argv).offset(0 as libc::c_int as isize),
                b"unsubscribe\0" as *const u8 as *const libc::c_char,
                11 as libc::c_int as size_t,
            ) == 0 as libc::c_int
            || *((*cmd).argv_len).offset(0 as libc::c_int as isize)
                == 12 as libc::c_int as libc::c_ulong
                && strncasecmp(
                    *((*cmd).argv).offset(0 as libc::c_int as isize),
                    b"punsubscribe\0" as *const u8 as *const libc::c_char,
                    12 as libc::c_int as size_t,
                ) == 0 as libc::c_int)
    {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
