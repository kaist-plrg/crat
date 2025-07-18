use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type dict;
    pub type sockadr;
    pub type event_base;
    pub type evbuffer;
    fn SHA1Reset(_: *mut SHA1Context);
    fn SHA1Result(_: *mut SHA1Context) -> libc::c_int;
    fn SHA1Input(_: *mut SHA1Context, _: *const libc::c_uchar, _: libc::c_uint);
    fn base64_init_encodestate(state_in: *mut base64_encodestate);
    fn base64_encode_block(
        plaintext_in: *const libc::c_char,
        length_in: libc::c_int,
        code_out: *mut libc::c_char,
        state_in: *mut base64_encodestate,
    ) -> libc::c_int;
    fn base64_encode_blockend(
        code_out: *mut libc::c_char,
        state_in: *mut base64_encodestate,
    ) -> libc::c_int;
    fn acl_allow_command(
        cmd: *mut cmd,
        cfg: *mut conf,
        client: *mut http_client,
    ) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn event_base_once(
        _: *mut event_base,
        _: libc::c_int,
        _: libc::c_short,
        _: event_callback_fn,
        _: *mut libc::c_void,
        _: *const timeval,
    ) -> libc::c_int;
    fn evbuffer_new() -> *mut evbuffer;
    fn evbuffer_free(buf: *mut evbuffer);
    fn evbuffer_get_length(buf: *const evbuffer) -> size_t;
    fn evbuffer_add(
        buf: *mut evbuffer,
        data: *const libc::c_void,
        datlen: size_t,
    ) -> libc::c_int;
    fn evbuffer_copyout(
        buf: *mut evbuffer,
        data_out: *mut libc::c_void,
        datlen: size_t,
    ) -> ssize_t;
    fn evbuffer_drain(buf: *mut evbuffer, len: size_t) -> libc::c_int;
    fn evbuffer_write_atmost(
        buffer: *mut evbuffer,
        fd: libc::c_int,
        howmuch: ssize_t,
    ) -> libc::c_int;
    fn evbuffer_read(
        buffer: *mut evbuffer,
        fd: libc::c_int,
        howmuch: libc::c_int,
    ) -> libc::c_int;
    fn redisAsyncFree(ac: *mut redisAsyncContext);
    fn http_client_free(c: *mut http_client);
    fn client_get_header(
        c: *mut http_client,
        key: *const libc::c_char,
    ) -> *const libc::c_char;
    fn cmd_free_argv(c: *mut cmd);
    fn cmd_free(c: *mut cmd);
    fn cmd_is_subscribe_args(cmd: *mut cmd) -> libc::c_int;
    fn cmd_is_unsubscribe_args(cmd: *mut cmd) -> libc::c_int;
    fn cmd_send(cmd: *mut cmd, f_format: formatting_fun);
    fn cmd_setup(cmd: *mut cmd, client: *mut http_client);
    fn pool_free_context(ac: *mut redisAsyncContext);
    fn pool_connect(
        p: *mut pool,
        db_num: libc::c_int,
        attach: libc::c_int,
    ) -> *mut redisAsyncContext;
    fn slog_enabled(s: *mut server, level: log_level) -> libc::c_int;
    fn slog(s: *mut server, level: log_level, body: *const libc::c_char, sz: size_t);
    fn json_reply(
        c: *mut redisAsyncContext,
        r: *mut libc::c_void,
        privdata: *mut libc::c_void,
    );
    fn json_ws_extract(
        c: *mut http_client,
        p: *const libc::c_char,
        sz: size_t,
    ) -> *mut cmd;
    fn json_ws_error(
        http_status: libc::c_int,
        msg: *const libc::c_char,
        msg_sz: size_t,
        out_sz: *mut size_t,
    ) -> *mut libc::c_char;
    fn raw_reply(
        c: *mut redisAsyncContext,
        r: *mut libc::c_void,
        privdata: *mut libc::c_void,
    );
    fn raw_ws_extract(
        c: *mut http_client,
        p: *const libc::c_char,
        sz: size_t,
    ) -> *mut cmd;
    fn raw_ws_error(
        http_status: libc::c_int,
        msg: *const libc::c_char,
        msg_sz: size_t,
        out_sz: *mut size_t,
    ) -> *mut libc::c_char;
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
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn close(__fd: libc::c_int) -> libc::c_int;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SHA1Context {
    pub Message_Digest: [libc::c_uint; 5],
    pub Length_Low: libc::c_uint,
    pub Length_High: libc::c_uint,
    pub Message_Block: [libc::c_uchar; 64],
    pub Message_Block_Index: libc::c_int,
    pub Computed: libc::c_int,
    pub Corrupted: libc::c_int,
}
pub type base64_encodestep = libc::c_uint;
pub const step_C: base64_encodestep = 2;
pub const step_B: base64_encodestep = 1;
pub const step_A: base64_encodestep = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct base64_encodestate {
    pub step: base64_encodestep,
    pub result: libc::c_char,
    pub stepcount: libc::c_int,
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type size_t = libc::c_ulong;
pub type gid_t = __gid_t;
pub type uid_t = __uid_t;
pub type pid_t = __pid_t;
pub type ssize_t = __ssize_t;
pub type int64_t = __int64_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct acl {
    pub cidr: C2RustUnnamed_5,
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
pub struct C2RustUnnamed_5 {
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
    pub log: C2RustUnnamed_6,
    pub auth_log_mutex: pthread_mutex_t,
    pub auth_logged: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub self_0: pid_t,
    pub fd: libc::c_int,
    pub fsync_tv: timeval,
    pub fsync_ev: *mut event,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct event {
    pub ev_evcallback: event_callback,
    pub ev_timeout_pos: C2RustUnnamed_12,
    pub ev_fd: libc::c_int,
    pub ev_base: *mut event_base,
    pub ev_: C2RustUnnamed_7,
    pub ev_events: libc::c_short,
    pub ev_res: libc::c_short,
    pub ev_timeout: timeval,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_7 {
    pub ev_io: C2RustUnnamed_10,
    pub ev_signal: C2RustUnnamed_8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub ev_signal_next: C2RustUnnamed_9,
    pub ev_ncalls: libc::c_short,
    pub ev_pncalls: *mut libc::c_short,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub le_next: *mut event,
    pub le_prev: *mut *mut event,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub ev_io_next: C2RustUnnamed_11,
    pub ev_timeout: timeval,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_11 {
    pub le_next: *mut event,
    pub le_prev: *mut *mut event,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_12 {
    pub ev_next_with_common_timeout: C2RustUnnamed_13,
    pub min_heap_idx: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_13 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct event_callback {
    pub evcb_active_next: C2RustUnnamed_15,
    pub evcb_flags: libc::c_short,
    pub evcb_pri: uint8_t,
    pub evcb_closure: uint8_t,
    pub evcb_cb_union: C2RustUnnamed_14,
    pub evcb_arg: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_14 {
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
pub struct C2RustUnnamed_15 {
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
pub type http_data_cb = Option::<
    unsafe extern "C" fn(*mut http_parser, *const libc::c_char, size_t) -> libc::c_int,
>;
pub type event_callback_fn = Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_short, *mut libc::c_void) -> (),
>;
pub type ws_state = libc::c_uint;
pub const WS_MSG_COMPLETE: ws_state = 2;
pub const WS_READING: ws_state = 1;
pub const WS_ERROR: ws_state = 0;
pub type ws_frame_type = libc::c_int;
pub const WS_UNKNOWN_FRAME: ws_frame_type = -1;
pub const WS_PONG: ws_frame_type = 10;
pub const WS_PING: ws_frame_type = 9;
pub const WS_CONNECTION_CLOSE: ws_frame_type = 8;
pub const WS_BINARY_FRAME: ws_frame_type = 1;
pub const WS_TEXT_FRAME: ws_frame_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ws_msg {
    pub type_0: ws_frame_type,
    pub payload: *mut libc::c_char,
    pub payload_sz: size_t,
    pub total_sz: size_t,
}
pub type formatting_fun = Option::<
    unsafe extern "C" fn(
        *mut redisAsyncContext,
        *mut libc::c_void,
        *mut libc::c_void,
    ) -> (),
>;
pub type ws_error_fun = Option::<
    unsafe extern "C" fn(
        libc::c_int,
        *const libc::c_char,
        size_t,
        *mut size_t,
    ) -> *mut libc::c_char,
>;
#[inline]
unsafe extern "C" fn __bswap_16(mut __bsx: __uint16_t) -> __uint16_t {
    return (__bsx as libc::c_int >> 8 as libc::c_int & 0xff as libc::c_int
        | (__bsx as libc::c_int & 0xff as libc::c_int) << 8 as libc::c_int)
        as __uint16_t;
}
#[inline]
unsafe extern "C" fn __bswap_32(mut __bsx: __uint32_t) -> __uint32_t {
    return (__bsx & 0xff000000 as libc::c_uint) >> 24 as libc::c_int
        | (__bsx & 0xff0000 as libc::c_uint) >> 8 as libc::c_int
        | (__bsx & 0xff00 as libc::c_uint) << 8 as libc::c_int
        | (__bsx & 0xff as libc::c_uint) << 24 as libc::c_int;
}
unsafe extern "C" fn ws_compute_handshake(
    mut c: *mut http_client,
    mut out: *mut libc::c_char,
    mut out_sz: *mut size_t,
) -> libc::c_int {
    let mut buffer: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut sha1_output: [libc::c_uchar; 20] = [0; 20];
    let mut magic: [libc::c_char; 37] = *::std::mem::transmute::<
        &[u8; 37],
        &mut [libc::c_char; 37],
    >(b"258EAFA5-E914-47DA-95CA-C5AB0DC85B11\0");
    let mut ctx: SHA1Context = SHA1Context {
        Message_Digest: [0; 5],
        Length_Low: 0,
        Length_High: 0,
        Message_Block: [0; 64],
        Message_Block_Index: 0,
        Computed: 0,
        Corrupted: 0,
    };
    let mut b64_ctx: base64_encodestate = base64_encodestate {
        step: step_A,
        result: 0,
        stepcount: 0,
    };
    let mut pos: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut key: *const libc::c_char = client_get_header(
        c,
        b"Sec-WebSocket-Key\0" as *const u8 as *const libc::c_char,
    );
    let mut key_sz: size_t = if !key.is_null() {
        strlen(key)
    } else {
        0 as libc::c_int as libc::c_ulong
    };
    let mut buffer_sz: size_t = key_sz
        .wrapping_add(::std::mem::size_of::<[libc::c_char; 37]>() as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong);
    if key.is_null() || key_sz < 16 as libc::c_int as libc::c_ulong
        || key_sz > 32 as libc::c_int as libc::c_ulong
    {
        slog(
            (*c).s,
            WEBDIS_WARNING,
            b"Invalid Sec-WebSocket-Key\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int as size_t,
        );
        return -(1 as libc::c_int);
    }
    buffer = calloc(buffer_sz, 1 as libc::c_int as libc::c_ulong) as *mut libc::c_uchar;
    if buffer.is_null() {
        slog(
            (*c).s,
            WEBDIS_ERROR,
            b"Failed to allocate memory for WS header\0" as *const u8
                as *const libc::c_char,
            0 as libc::c_int as size_t,
        );
        return -(1 as libc::c_int);
    }
    memcpy(buffer as *mut libc::c_void, key as *const libc::c_void, key_sz);
    memcpy(
        buffer.offset(key_sz as isize) as *mut libc::c_void,
        magic.as_mut_ptr() as *const libc::c_void,
        (::std::mem::size_of::<[libc::c_char; 37]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    );
    SHA1Reset(&mut ctx);
    SHA1Input(&mut ctx, buffer, buffer_sz as libc::c_uint);
    SHA1Result(&mut ctx);
    i = 0 as libc::c_int;
    while i
        < (20 as libc::c_int as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            as libc::c_int
    {
        ctx.Message_Digest[i as usize] = __bswap_32(ctx.Message_Digest[i as usize]);
        i += 1;
        i;
    }
    memcpy(
        sha1_output.as_mut_ptr() as *mut libc::c_void,
        (ctx.Message_Digest).as_mut_ptr() as *const libc::c_void,
        20 as libc::c_int as libc::c_ulong,
    );
    base64_init_encodestate(&mut b64_ctx);
    pos = base64_encode_block(
        sha1_output.as_mut_ptr() as *const libc::c_char,
        20 as libc::c_int,
        out,
        &mut b64_ctx,
    );
    base64_encode_blockend(out.offset(pos as isize), &mut b64_ctx);
    *out_sz = strlen(out);
    if *out.offset((*out_sz).wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
        as libc::c_int == '\n' as i32
    {
        *out_sz = (*out_sz).wrapping_sub(1);
        *out_sz;
    }
    free(buffer as *mut libc::c_void);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn ws_client_new(
    mut http_client: *mut http_client,
) -> *mut ws_client {
    let mut db_num: libc::c_int = (*(*(*(*http_client).w).s).cfg).database;
    let mut ws: *mut ws_client = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<ws_client>() as libc::c_ulong,
    ) as *mut ws_client;
    let mut rbuf: *mut evbuffer = evbuffer_new();
    let mut wbuf: *mut evbuffer = evbuffer_new();
    let mut ac: *mut redisAsyncContext = pool_connect(
        (*(*http_client).w).pool,
        db_num,
        0 as libc::c_int,
    );
    if ws.is_null() || rbuf.is_null() || wbuf.is_null() {
        slog(
            (*http_client).s,
            WEBDIS_ERROR,
            b"Failed to allocate memory for WS client\0" as *const u8
                as *const libc::c_char,
            0 as libc::c_int as size_t,
        );
        if !ws.is_null() {
            free(ws as *mut libc::c_void);
        }
        if !rbuf.is_null() {
            evbuffer_free(rbuf);
        }
        if !wbuf.is_null() {
            evbuffer_free(wbuf);
        }
        if !ac.is_null() {
            redisAsyncFree(ac);
        }
        return 0 as *mut ws_client;
    }
    (*http_client).ws = ws;
    (*ws).http_client = http_client;
    (*ws).rbuf = rbuf;
    (*ws).wbuf = wbuf;
    (*ws).ac = ac;
    return ws;
}
unsafe extern "C" fn ws_client_free(mut ws: *mut ws_client) {
    (*ws).close_after_events = 1 as libc::c_int;
    pool_free_context((*ws).ac);
    let mut c: *mut http_client = (*ws).http_client;
    if !c.is_null() {
        close((*c).fd);
        (*c).ws = 0 as *mut ws_client;
    }
    evbuffer_free((*ws).rbuf);
    evbuffer_free((*ws).wbuf);
    if !((*ws).cmd).is_null() {
        (*(*ws).cmd).ac = 0 as *mut redisAsyncContext;
        cmd_free((*ws).cmd);
    }
    free(ws as *mut libc::c_void);
    if !c.is_null() {
        http_client_free(c);
    }
}
pub unsafe extern "C" fn ws_handshake_reply(mut ws: *mut ws_client) -> libc::c_int {
    let mut c: *mut http_client = (*ws).http_client;
    let mut sha1_handshake: [libc::c_char; 40] = [0; 40];
    let mut buffer: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut origin: *const libc::c_char = 0 as *const libc::c_char;
    let mut host: *const libc::c_char = 0 as *const libc::c_char;
    let mut origin_sz: size_t = 0 as libc::c_int as size_t;
    let mut host_sz: size_t = 0 as libc::c_int as size_t;
    let mut handshake_sz: size_t = 0 as libc::c_int as size_t;
    let mut sz: size_t = 0;
    let mut template_start: [libc::c_char; 74] = *::std::mem::transmute::<
        &[u8; 74],
        &mut [libc::c_char; 74],
    >(
        b"HTTP/1.1 101 Switching Protocols\r\nUpgrade: websocket\r\nConnection: Upgrade\0",
    );
    let mut template_accept: [libc::c_char; 25] = *::std::mem::transmute::<
        &[u8; 25],
        &mut [libc::c_char; 25],
    >(b"\r\nSec-WebSocket-Accept: \0");
    let mut template_sec_origin: [libc::c_char; 25] = *::std::mem::transmute::<
        &[u8; 25],
        &mut [libc::c_char; 25],
    >(b"\r\nSec-WebSocket-Origin: \0");
    let mut template_loc: [libc::c_char; 32] = *::std::mem::transmute::<
        &[u8; 32],
        &mut [libc::c_char; 32],
    >(b"\r\nSec-WebSocket-Location: ws://\0");
    let mut template_end: [libc::c_char; 5] = *::std::mem::transmute::<
        &[u8; 5],
        &mut [libc::c_char; 5],
    >(b"\r\n\r\n\0");
    origin = client_get_header(c, b"Origin\0" as *const u8 as *const libc::c_char);
    if !origin.is_null() {
        origin_sz = strlen(origin);
    } else {
        origin = client_get_header(
            c,
            b"Sec-WebSocket-Origin\0" as *const u8 as *const libc::c_char,
        );
        if !origin.is_null() {
            origin_sz = strlen(origin);
        }
    }
    host = client_get_header(c, b"Host\0" as *const u8 as *const libc::c_char);
    if !host.is_null() {
        host_sz = strlen(host);
    }
    if host.is_null() || host_sz == 0 || ((*c).path).is_null() || (*c).path_sz == 0 {
        slog(
            (*c).s,
            WEBDIS_WARNING,
            b"Missing headers for WS handshake\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int as size_t,
        );
        return -(1 as libc::c_int);
    }
    memset(
        sha1_handshake.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[libc::c_char; 40]>() as libc::c_ulong,
    );
    if ws_compute_handshake(
        c,
        &mut *sha1_handshake.as_mut_ptr().offset(0 as libc::c_int as isize),
        &mut handshake_sz,
    ) != 0 as libc::c_int
    {
        slog(
            (*c).s,
            WEBDIS_WARNING,
            b"Failed to compute handshake\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int as size_t,
        );
        return -(1 as libc::c_int);
    }
    sz = (::std::mem::size_of::<[libc::c_char; 74]>() as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<[libc::c_char; 25]>() as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        .wrapping_add(handshake_sz)
        .wrapping_add(
            (if !origin.is_null() && origin_sz != 0 {
                (::std::mem::size_of::<[libc::c_char; 25]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    .wrapping_add(origin_sz)
            } else {
                0 as libc::c_int as libc::c_ulong
            }),
        )
        .wrapping_add(::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        .wrapping_add(host_sz)
        .wrapping_add((*c).path_sz)
        .wrapping_add(::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong);
    buffer = malloc(sz) as *mut libc::c_char;
    p = buffer;
    if p.is_null() {
        slog(
            (*c).s,
            WEBDIS_ERROR,
            b"Failed to allocate buffer for WS handshake\0" as *const u8
                as *const libc::c_char,
            0 as libc::c_int as size_t,
        );
        return -(1 as libc::c_int);
    }
    memcpy(
        p as *mut libc::c_void,
        template_start.as_mut_ptr() as *const libc::c_void,
        (::std::mem::size_of::<[libc::c_char; 74]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    );
    p = p
        .offset(
            (::std::mem::size_of::<[libc::c_char; 74]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
        );
    memcpy(
        p as *mut libc::c_void,
        template_accept.as_mut_ptr() as *const libc::c_void,
        (::std::mem::size_of::<[libc::c_char; 25]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    );
    p = p
        .offset(
            (::std::mem::size_of::<[libc::c_char; 25]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
        );
    memcpy(
        p as *mut libc::c_void,
        &mut *sha1_handshake.as_mut_ptr().offset(0 as libc::c_int as isize)
            as *mut libc::c_char as *const libc::c_void,
        handshake_sz,
    );
    p = p.offset(handshake_sz as isize);
    if !origin.is_null() && origin_sz != 0 {
        memcpy(
            p as *mut libc::c_void,
            template_sec_origin.as_mut_ptr() as *const libc::c_void,
            (::std::mem::size_of::<[libc::c_char; 25]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
        p = p
            .offset(
                (::std::mem::size_of::<[libc::c_char; 25]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
            );
        memcpy(p as *mut libc::c_void, origin as *const libc::c_void, origin_sz);
        p = p.offset(origin_sz as isize);
    }
    memcpy(
        p as *mut libc::c_void,
        template_loc.as_mut_ptr() as *const libc::c_void,
        (::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    );
    p = p
        .offset(
            (::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
        );
    memcpy(p as *mut libc::c_void, host as *const libc::c_void, host_sz);
    p = p.offset(host_sz as isize);
    memcpy(p as *mut libc::c_void, (*c).path as *const libc::c_void, (*c).path_sz);
    p = p.offset((*c).path_sz as isize);
    memcpy(
        p as *mut libc::c_void,
        template_end.as_mut_ptr() as *const libc::c_void,
        (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    );
    p = p
        .offset(
            (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
        );
    let mut add_ret: libc::c_int = evbuffer_add(
        (*ws).wbuf,
        buffer as *const libc::c_void,
        sz,
    );
    free(buffer as *mut libc::c_void);
    if add_ret < 0 as libc::c_int {
        slog(
            (*c).s,
            WEBDIS_ERROR,
            b"Failed to add response for WS handshake\0" as *const u8
                as *const libc::c_char,
            0 as libc::c_int as size_t,
        );
        return -(1 as libc::c_int);
    }
    return ws_schedule_write(ws);
}
unsafe extern "C" fn ws_log_cmd(mut ws: *mut ws_client, mut cmd: *mut cmd) {
    let mut log_msg: [libc::c_char; 124] = [0; 124];
    let mut p: *mut libc::c_char = log_msg.as_mut_ptr();
    let mut eom: *mut libc::c_char = log_msg
        .as_mut_ptr()
        .offset(::std::mem::size_of::<[libc::c_char; 124]>() as libc::c_ulong as isize)
        .offset(-(1 as libc::c_int as isize));
    if slog_enabled((*(*ws).http_client).s, WEBDIS_DEBUG) == 0 {
        return;
    }
    memset(
        log_msg.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[libc::c_char; 124]>() as libc::c_ulong,
    );
    memcpy(
        p as *mut libc::c_void,
        b"WS: \0" as *const u8 as *const libc::c_char as *const libc::c_void,
        4 as libc::c_int as libc::c_ulong,
    );
    p = p.offset(4 as libc::c_int as isize);
    let mut i: libc::c_int = 0 as libc::c_int;
    while p < eom && i < (*cmd).count {
        let fresh0 = p;
        p = p.offset(1);
        *fresh0 = '/' as i32 as libc::c_char;
        let mut arg: *mut libc::c_char = *((*cmd).argv).offset(i as isize);
        let mut arg_sz: size_t = *((*cmd).argv_len).offset(i as isize);
        let mut copy_sz: size_t = if arg_sz
            < eom.offset_from(p) as libc::c_long as size_t
        {
            arg_sz
        } else {
            eom.offset_from(p) as libc::c_long as size_t
        };
        memcpy(p as *mut libc::c_void, arg as *const libc::c_void, copy_sz);
        p = p.offset(copy_sz as isize);
        i += 1;
        i;
    }
    slog(
        (*(*ws).http_client).s,
        WEBDIS_DEBUG,
        log_msg.as_mut_ptr(),
        p.offset_from(log_msg.as_mut_ptr()) as libc::c_long as size_t,
    );
}
unsafe extern "C" fn ws_log_unauthorized(mut ws: *mut ws_client) {
    if slog_enabled((*(*ws).http_client).s, WEBDIS_DEBUG) == 0 {
        return;
    }
    let msg: [libc::c_char; 8] = *::std::mem::transmute::<
        &[u8; 8],
        &[libc::c_char; 8],
    >(b"WS: 403\0");
    slog(
        (*(*ws).http_client).s,
        WEBDIS_DEBUG,
        msg.as_ptr(),
        (::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    );
}
unsafe extern "C" fn ws_execute(
    mut ws: *mut ws_client,
    mut msg: *mut ws_msg,
) -> libc::c_int {
    let mut c: *mut http_client = (*ws).http_client;
    let mut fun_extract: Option::<
        unsafe extern "C" fn(*mut http_client, *const libc::c_char, size_t) -> *mut cmd,
    > = None;
    let mut fun_reply: formatting_fun = None;
    let mut fun_error: ws_error_fun = None;
    if (*c).path_sz == 1 as libc::c_int as libc::c_ulong
        && strncmp(
            (*c).path,
            b"/\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
        || strncmp(
            (*c).path,
            b"/.json\0" as *const u8 as *const libc::c_char,
            6 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
    {
        fun_extract = Some(
            json_ws_extract
                as unsafe extern "C" fn(
                    *mut http_client,
                    *const libc::c_char,
                    size_t,
                ) -> *mut cmd,
        );
        fun_reply = Some(
            json_reply
                as unsafe extern "C" fn(
                    *mut redisAsyncContext,
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> (),
        );
        fun_error = Some(
            json_ws_error
                as unsafe extern "C" fn(
                    libc::c_int,
                    *const libc::c_char,
                    size_t,
                    *mut size_t,
                ) -> *mut libc::c_char,
        );
    } else if strncmp(
        (*c).path,
        b"/.raw\0" as *const u8 as *const libc::c_char,
        5 as libc::c_int as libc::c_ulong,
    ) == 0 as libc::c_int
    {
        fun_extract = Some(
            raw_ws_extract
                as unsafe extern "C" fn(
                    *mut http_client,
                    *const libc::c_char,
                    size_t,
                ) -> *mut cmd,
        );
        fun_reply = Some(
            raw_reply
                as unsafe extern "C" fn(
                    *mut redisAsyncContext,
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> (),
        );
        fun_error = Some(
            raw_ws_error
                as unsafe extern "C" fn(
                    libc::c_int,
                    *const libc::c_char,
                    size_t,
                    *mut size_t,
                ) -> *mut libc::c_char,
        );
    }
    if fun_extract.is_some() {
        let mut cmd: *mut cmd = fun_extract
            .unwrap()(c, (*msg).payload, (*msg).payload_sz);
        if !cmd.is_null() {
            (*cmd).is_websocket = 1 as libc::c_int;
            if !((*ws).cmd).is_null() {
                cmd_free_argv((*ws).cmd);
                (*(*ws).cmd).count = (*cmd).count;
                (*(*ws).cmd).argv = (*cmd).argv;
                (*(*ws).cmd).argv_len = (*cmd).argv_len;
                (*(*ws).cmd).pub_sub_client = c;
                (*cmd).argv = 0 as *mut *mut libc::c_char;
                (*cmd).argv_len = 0 as *mut size_t;
                (*cmd).count = 0 as libc::c_int;
                cmd_free(cmd);
                cmd = (*ws).cmd;
            } else {
                cmd_setup(cmd, c);
                (*cmd).ac = (*ws).ac;
                (*ws).cmd = cmd;
                (*cmd).pub_sub_client = c;
            }
            let mut is_subscribe: libc::c_int = cmd_is_subscribe_args(cmd);
            let mut is_unsubscribe: libc::c_int = cmd_is_unsubscribe_args(cmd);
            if acl_allow_command(cmd, (*(*c).s).cfg, c) == 0 {
                let forbidden: [libc::c_char; 10] = *::std::mem::transmute::<
                    &[u8; 10],
                    &[libc::c_char; 10],
                >(b"Forbidden\0");
                let mut error_sz: size_t = 0;
                let mut error: *mut libc::c_char = fun_error
                    .unwrap()(
                    403 as libc::c_int,
                    forbidden.as_ptr(),
                    (::std::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    &mut error_sz,
                );
                ws_frame_and_send_response(ws, WS_BINARY_FRAME, error, error_sz);
                free(error as *mut libc::c_void);
                ws_log_cmd(ws, cmd);
                ws_log_unauthorized(ws);
            } else if (*ws).ran_subscribe != 0 && is_subscribe == 0
                && is_unsubscribe == 0
            {
                let mut error_msg: [libc::c_char; 36] = *::std::mem::transmute::<
                    &[u8; 36],
                    &mut [libc::c_char; 36],
                >(b"Command not allowed after subscribe\0");
                ws_frame_and_send_response(
                    ws,
                    WS_BINARY_FRAME,
                    error_msg.as_mut_ptr(),
                    (::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                );
            } else {
                ws_log_cmd(ws, cmd);
                cmd_send(cmd, fun_reply);
                (*ws).ran_subscribe = is_subscribe;
            }
            return 0 as libc::c_int;
        }
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn ws_msg_new(mut frame_type: ws_frame_type) -> *mut ws_msg {
    let mut msg: *mut ws_msg = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<ws_msg>() as libc::c_ulong,
    ) as *mut ws_msg;
    if msg.is_null() {
        return 0 as *mut ws_msg;
    }
    (*msg).type_0 = frame_type;
    return msg;
}
unsafe extern "C" fn ws_msg_add(
    mut m: *mut ws_msg,
    mut p: *const libc::c_char,
    mut psz: size_t,
    mut mask: *const libc::c_uchar,
) -> libc::c_int {
    let mut i: size_t = 0;
    (*m)
        .payload = realloc(
        (*m).payload as *mut libc::c_void,
        ((*m).payload_sz).wrapping_add(psz),
    ) as *mut libc::c_char;
    if ((*m).payload).is_null() {
        return -(1 as libc::c_int);
    }
    memcpy(
        ((*m).payload).offset((*m).payload_sz as isize) as *mut libc::c_void,
        p as *const libc::c_void,
        psz,
    );
    i = 0 as libc::c_int as size_t;
    while i < psz && !mask.is_null() {
        *((*m).payload)
            .offset(
                ((*m).payload_sz).wrapping_add(i) as isize,
            ) = (*p.offset(i as isize) as libc::c_uchar as libc::c_int
            ^ *mask.offset(i.wrapping_rem(4 as libc::c_int as libc::c_ulong) as isize)
                as libc::c_int) as libc::c_char;
        i = i.wrapping_add(1);
        i;
    }
    (*m)
        .payload_sz = ((*m).payload_sz as libc::c_ulong).wrapping_add(psz) as size_t
        as size_t;
    return 0 as libc::c_int;
}
unsafe extern "C" fn ws_msg_free(mut m: *mut ws_msg) {
    free((*m).payload as *mut libc::c_void);
    free(m as *mut libc::c_void);
}
unsafe extern "C" fn ws_peek_data(
    mut ws: *mut ws_client,
    mut out_msg: *mut *mut ws_msg,
) -> ws_state {
    let mut has_mask: libc::c_int = 0;
    let mut len: uint64_t = 0;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut frame: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut mask: [libc::c_uchar; 4] = [0; 4];
    let mut fin_bit_set: libc::c_char = 0;
    let mut frame_type: ws_frame_type = WS_TEXT_FRAME;
    let mut sz: size_t = evbuffer_get_length((*ws).rbuf);
    if sz < 2 as libc::c_int as libc::c_ulong {
        return WS_READING;
    }
    frame = malloc(sz) as *mut libc::c_char;
    if frame.is_null() {
        return WS_ERROR;
    }
    let mut rem_ret: libc::c_int = evbuffer_copyout(
        (*ws).rbuf,
        frame as *mut libc::c_void,
        sz,
    ) as libc::c_int;
    if rem_ret < 0 as libc::c_int {
        free(frame as *mut libc::c_void);
        return WS_ERROR;
    }
    fin_bit_set = (if *frame.offset(0 as libc::c_int as isize) as libc::c_int
        & 0x80 as libc::c_int != 0
    {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    }) as libc::c_char;
    frame_type = (*frame.offset(0 as libc::c_int as isize) as libc::c_int
        & 0xf as libc::c_int) as ws_frame_type;
    has_mask = if *frame.offset(1 as libc::c_int as isize) as libc::c_int
        & 0x80 as libc::c_int != 0
    {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    };
    if has_mask == 0 {
        (*ws).close_after_events = 1 as libc::c_int;
        let close_code_reason: [libc::c_char; 85] = *::std::mem::transmute::<
            &[u8; 85],
            &[libc::c_char; 85],
        >(
            b"\x03\xEAReceived a frame without a mask from the client (violates RFC6455, 5.1. Overview).\0",
        );
        ws_frame_and_send_response(
            ws,
            WS_CONNECTION_CLOSE,
            close_code_reason.as_ptr(),
            (::std::mem::size_of::<[libc::c_char; 85]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
        free(frame as *mut libc::c_void);
        return WS_ERROR;
    }
    len = (*frame.offset(1 as libc::c_int as isize) as libc::c_int & 0x7f as libc::c_int)
        as uint64_t;
    let mut min_sz: size_t = 6 as libc::c_int as size_t;
    if len == 126 as libc::c_int as libc::c_ulong {
        min_sz = (min_sz as libc::c_ulong)
            .wrapping_add(::std::mem::size_of::<uint16_t>() as libc::c_ulong) as size_t
            as size_t;
    } else if len == 127 as libc::c_int as libc::c_ulong {
        min_sz = (min_sz as libc::c_ulong)
            .wrapping_add(::std::mem::size_of::<uint64_t>() as libc::c_ulong) as size_t
            as size_t;
    }
    if sz < min_sz {
        free(frame as *mut libc::c_void);
        return WS_READING;
    }
    if len <= 125 as libc::c_int as libc::c_ulong {
        p = frame.offset(6 as libc::c_int as isize);
        memcpy(
            &mut mask as *mut [libc::c_uchar; 4] as *mut libc::c_void,
            frame.offset(2 as libc::c_int as isize) as *const libc::c_void,
            ::std::mem::size_of::<[libc::c_uchar; 4]>() as libc::c_ulong,
        );
    } else if len == 126 as libc::c_int as libc::c_ulong {
        let mut sz16: uint16_t = 0;
        memcpy(
            &mut sz16 as *mut uint16_t as *mut libc::c_void,
            frame.offset(2 as libc::c_int as isize) as *const libc::c_void,
            ::std::mem::size_of::<uint16_t>() as libc::c_ulong,
        );
        len = __bswap_16(sz16) as uint64_t;
        p = frame
            .offset(6 as libc::c_int as isize)
            .offset(::std::mem::size_of::<uint16_t>() as libc::c_ulong as isize);
        memcpy(
            &mut mask as *mut [libc::c_uchar; 4] as *mut libc::c_void,
            frame.offset(4 as libc::c_int as isize) as *const libc::c_void,
            ::std::mem::size_of::<[libc::c_uchar; 4]>() as libc::c_ulong,
        );
    } else if len == 127 as libc::c_int as libc::c_ulong {
        let mut sz64: uint64_t = *(frame.offset(2 as libc::c_int as isize)
            as *mut uint64_t);
        len = (__bswap_32(
            (sz64 & 0xffffffff as libc::c_uint as libc::c_ulong) as __uint32_t,
        ) as uint64_t) << 32 as libc::c_int
            | __bswap_32((sz64 >> 32 as libc::c_int) as __uint32_t) as libc::c_ulong;
        p = frame
            .offset(6 as libc::c_int as isize)
            .offset(::std::mem::size_of::<uint64_t>() as libc::c_ulong as isize);
        memcpy(
            &mut mask as *mut [libc::c_uchar; 4] as *mut libc::c_void,
            frame.offset(10 as libc::c_int as isize) as *const libc::c_void,
            ::std::mem::size_of::<[libc::c_uchar; 4]>() as libc::c_ulong,
        );
    } else {
        free(frame as *mut libc::c_void);
        return WS_ERROR;
    }
    if len > sz.wrapping_sub(p.offset_from(frame) as libc::c_long as libc::c_ulong) {
        free(frame as *mut libc::c_void);
        return WS_READING;
    }
    let mut ev_copy: libc::c_int = 0 as libc::c_int;
    if !out_msg.is_null() {
        let mut msg: *mut ws_msg = ws_msg_new(frame_type);
        if msg.is_null() {
            free(frame as *mut libc::c_void);
            return WS_ERROR;
        }
        *out_msg = msg;
        let mut add_ret: libc::c_int = ws_msg_add(msg, p, len, mask.as_mut_ptr());
        if add_ret < 0 as libc::c_int {
            free(frame as *mut libc::c_void);
            return WS_ERROR;
        }
        let mut processed_sz: size_t = len
            .wrapping_add(p.offset_from(frame) as libc::c_long as libc::c_ulong);
        (*msg)
            .total_sz = ((*msg).total_sz as libc::c_ulong).wrapping_add(processed_sz)
            as size_t as size_t;
        ev_copy = evbuffer_drain((*ws).rbuf, processed_sz);
    }
    free(frame as *mut libc::c_void);
    if ev_copy < 0 as libc::c_int {
        return WS_ERROR
    } else if fin_bit_set != 0 {
        return WS_MSG_COMPLETE
    } else {
        return WS_READING
    };
}
pub unsafe extern "C" fn ws_process_read_data(
    mut ws: *mut ws_client,
    mut out_processed: *mut libc::c_uint,
) -> ws_state {
    let mut state: ws_state = WS_ERROR;
    if !out_processed.is_null() {
        *out_processed = 0 as libc::c_int as libc::c_uint;
    }
    state = ws_peek_data(ws, 0 as *mut *mut ws_msg);
    while state as libc::c_uint == WS_MSG_COMPLETE as libc::c_int as libc::c_uint {
        let mut ret: libc::c_int = 0 as libc::c_int;
        let mut msg: *mut ws_msg = 0 as *mut ws_msg;
        ws_peek_data(ws, &mut msg);
        if !msg.is_null()
            && ((*msg).type_0 as libc::c_int == WS_TEXT_FRAME as libc::c_int
                || (*msg).type_0 as libc::c_int == WS_BINARY_FRAME as libc::c_int)
        {
            ret = ws_execute(ws, msg);
            if !out_processed.is_null() {
                *out_processed = (*out_processed).wrapping_add(1);
                *out_processed;
            }
        } else if !msg.is_null()
            && (*msg).type_0 as libc::c_int == WS_PING as libc::c_int
        {
            ws_frame_and_send_response(ws, WS_PONG, (*msg).payload, (*msg).payload_sz);
        } else if !msg.is_null()
            && (*msg).type_0 as libc::c_int == WS_CONNECTION_CLOSE as libc::c_int
        {
            (*ws).close_after_events = 1 as libc::c_int;
            ws_frame_and_send_response(
                ws,
                WS_CONNECTION_CLOSE,
                (*msg).payload,
                (*msg).payload_sz,
            );
        } else if !msg.is_null() {
            let mut format: [libc::c_char; 40] = *::std::mem::transmute::<
                &[u8; 40],
                &mut [libc::c_char; 40],
            >(b"Received unexpected WS frame type: 0x%x\0");
            let mut error: [libc::c_char; 40] = [0; 40];
            let mut error_len: libc::c_int = snprintf(
                error.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 40]>() as libc::c_ulong,
                format.as_mut_ptr(),
                (*msg).type_0 as libc::c_int,
            );
            slog(
                (*(*ws).http_client).s,
                WEBDIS_WARNING,
                error.as_mut_ptr(),
                error_len as size_t,
            );
        }
        if !msg.is_null() {
            ws_msg_free(msg);
        }
        if ret != 0 as libc::c_int {
            slog(
                (*(*ws).http_client).s,
                WEBDIS_DEBUG,
                b"ws_process_read_data: ws_execute failed\0" as *const u8
                    as *const libc::c_char,
                0 as libc::c_int as size_t,
            );
            return WS_ERROR;
        }
        state = ws_peek_data(ws, 0 as *mut *mut ws_msg);
    }
    return state;
}
pub unsafe extern "C" fn ws_frame_and_send_response(
    mut ws: *mut ws_client,
    mut frame_type: ws_frame_type,
    mut p: *const libc::c_char,
    mut sz: size_t,
) -> libc::c_int {
    let mut frame: *mut libc::c_char = malloc(
        sz.wrapping_add(14 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    let mut frame_sz: size_t = 0 as libc::c_int as size_t;
    if frame.is_null() {
        return -(1 as libc::c_int);
    }
    *frame
        .offset(
            0 as libc::c_int as isize,
        ) = (0x80 as libc::c_int | frame_type as libc::c_int) as libc::c_char;
    if sz <= 125 as libc::c_int as libc::c_ulong {
        *frame.offset(1 as libc::c_int as isize) = sz as libc::c_char;
        memcpy(
            frame.offset(2 as libc::c_int as isize) as *mut libc::c_void,
            p as *const libc::c_void,
            sz,
        );
        frame_sz = sz.wrapping_add(2 as libc::c_int as libc::c_ulong);
    } else if sz <= 65536 as libc::c_int as libc::c_ulong {
        let mut sz16: uint16_t = __bswap_16(sz as __uint16_t);
        *frame.offset(1 as libc::c_int as isize) = 126 as libc::c_int as libc::c_char;
        memcpy(
            frame.offset(2 as libc::c_int as isize) as *mut libc::c_void,
            &mut sz16 as *mut uint16_t as *const libc::c_void,
            2 as libc::c_int as libc::c_ulong,
        );
        memcpy(
            frame.offset(4 as libc::c_int as isize) as *mut libc::c_void,
            p as *const libc::c_void,
            sz,
        );
        frame_sz = sz.wrapping_add(4 as libc::c_int as libc::c_ulong);
    } else {
        let mut sz_be: uint64_t = (__bswap_32(
            (sz & 0xffffffff as libc::c_uint as libc::c_ulong) as __uint32_t,
        ) as uint64_t) << 32 as libc::c_int
            | __bswap_32((sz >> 32 as libc::c_int) as __uint32_t) as libc::c_ulong;
        let mut sz64: [libc::c_char; 8] = [0; 8];
        memcpy(
            sz64.as_mut_ptr() as *mut libc::c_void,
            &mut sz_be as *mut uint64_t as *const libc::c_void,
            8 as libc::c_int as libc::c_ulong,
        );
        *frame.offset(1 as libc::c_int as isize) = 127 as libc::c_int as libc::c_char;
        memcpy(
            frame.offset(2 as libc::c_int as isize) as *mut libc::c_void,
            sz64.as_mut_ptr() as *const libc::c_void,
            8 as libc::c_int as libc::c_ulong,
        );
        memcpy(
            frame.offset(10 as libc::c_int as isize) as *mut libc::c_void,
            p as *const libc::c_void,
            sz,
        );
        frame_sz = sz.wrapping_add(10 as libc::c_int as libc::c_ulong);
    }
    let mut add_ret: libc::c_int = evbuffer_add(
        (*ws).wbuf,
        frame as *const libc::c_void,
        frame_sz,
    );
    free(frame as *mut libc::c_void);
    if add_ret < 0 as libc::c_int {
        slog(
            (*(*(*ws).http_client).w).s,
            WEBDIS_ERROR,
            b"Failed response allocation in ws_frame_and_send_response\0" as *const u8
                as *const libc::c_char,
            0 as libc::c_int as size_t,
        );
        return -(1 as libc::c_int);
    }
    return ws_schedule_write(ws);
}
pub unsafe extern "C" fn ws_close_if_able(mut ws: *mut ws_client) {
    (*ws).close_after_events = 1 as libc::c_int;
    if (*ws).scheduled_read != 0 || (*ws).scheduled_write != 0 {
        return;
    }
    ws_client_free(ws);
}
unsafe extern "C" fn ws_can_read(
    mut fd: libc::c_int,
    mut event: libc::c_short,
    mut p: *mut libc::c_void,
) {
    let mut ret: libc::c_int = 0;
    let mut ws: *mut ws_client = p as *mut ws_client;
    (*ws).scheduled_read = 0 as libc::c_int;
    ret = evbuffer_read((*ws).rbuf, fd, 4096 as libc::c_int);
    if ret <= 0 as libc::c_int || (*ws).close_after_events != 0 {
        ws_close_if_able(ws);
    } else {
        let mut state: ws_state = ws_process_read_data(ws, 0 as *mut libc::c_uint);
        if state as libc::c_uint == WS_READING as libc::c_int as libc::c_uint {
            ws_monitor_input(ws);
        } else if state as libc::c_uint == WS_ERROR as libc::c_int as libc::c_uint {
            ws_close_if_able(ws);
        }
    };
}
unsafe extern "C" fn ws_can_write(
    mut fd: libc::c_int,
    mut event: libc::c_short,
    mut p: *mut libc::c_void,
) {
    let mut ret: libc::c_int = 0;
    let mut ws: *mut ws_client = p as *mut ws_client;
    (*ws).scheduled_write = 0 as libc::c_int;
    ret = evbuffer_write_atmost((*ws).wbuf, fd, 4096 as libc::c_int as ssize_t);
    if ret <= 0 as libc::c_int {
        ws_close_if_able(ws);
    } else if evbuffer_get_length((*ws).wbuf) > 0 as libc::c_int as libc::c_ulong {
        ws_schedule_write(ws);
    } else if (*ws).close_after_events != 0 {
        ws_close_if_able(ws);
    } else {
        let mut processed: libc::c_uint = 0 as libc::c_int as libc::c_uint;
        ws_process_read_data(ws, &mut processed);
        ws_monitor_input(ws);
    };
}
unsafe extern "C" fn ws_schedule_write(mut ws: *mut ws_client) -> libc::c_int {
    let mut c: *mut http_client = (*ws).http_client;
    if (*ws).scheduled_write == 0 {
        (*ws).scheduled_write = 1 as libc::c_int;
        return event_base_once(
            (*(*c).w).base,
            (*c).fd,
            0x4 as libc::c_int as libc::c_short,
            Some(
                ws_can_write
                    as unsafe extern "C" fn(
                        libc::c_int,
                        libc::c_short,
                        *mut libc::c_void,
                    ) -> (),
            ),
            ws as *mut libc::c_void,
            0 as *const timeval,
        );
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn ws_monitor_input(mut ws: *mut ws_client) -> libc::c_int {
    let mut c: *mut http_client = (*ws).http_client;
    if (*ws).scheduled_read == 0 {
        (*ws).scheduled_read = 1 as libc::c_int;
        return event_base_once(
            (*(*c).w).base,
            (*c).fd,
            0x2 as libc::c_int as libc::c_short,
            Some(
                ws_can_read
                    as unsafe extern "C" fn(
                        libc::c_int,
                        libc::c_short,
                        *mut libc::c_void,
                    ) -> (),
            ),
            ws as *mut libc::c_void,
            0 as *const timeval,
        );
    }
    return 0 as libc::c_int;
}
