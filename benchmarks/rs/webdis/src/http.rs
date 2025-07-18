use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type event_base;
    pub type evbuffer;
    pub type cmd;
    pub type dict;
    pub type sockadr;
    pub type pool;
    pub type conf;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
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
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn http_client_reset(c: *mut http_client);
    fn slog(s: *mut server, level: log_level, body: *const libc::c_char, sz: size_t);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strncasecmp(
        __s1: *const libc::c_char,
        __s2: *const libc::c_char,
        __n: size_t,
    ) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type pid_t = __pid_t;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
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
pub struct worker {
    pub thread: pthread_t,
    pub base: *mut event_base,
    pub s: *mut server,
    pub link: [libc::c_int; 2],
    pub pool: *mut pool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct http_response {
    pub ev: event,
    pub code: libc::c_short,
    pub msg: *const libc::c_char,
    pub headers: *mut http_header,
    pub header_count: libc::c_int,
    pub headers_array_size: libc::c_int,
    pub body: *const libc::c_char,
    pub body_len: size_t,
    pub out: *mut libc::c_char,
    pub out_sz: size_t,
    pub chunked: libc::c_int,
    pub http_version: libc::c_int,
    pub keep_alive: libc::c_int,
    pub sent: libc::c_int,
    pub w: *mut worker,
}
pub type log_level = libc::c_uint;
pub const WEBDIS_TRACE: log_level = 8;
pub const WEBDIS_DEBUG: log_level = 4;
pub const WEBDIS_INFO: log_level = 3;
pub const WEBDIS_NOTICE: log_level = 2;
pub const WEBDIS_WARNING: log_level = 1;
pub const WEBDIS_ERROR: log_level = 0;
unsafe extern "C" fn http_response_allocate_headers(mut r: *mut http_response) {
    (*r).headers_array_size = 9 as libc::c_int;
    (*r)
        .headers = calloc(
        (*r).headers_array_size as libc::c_ulong,
        ::std::mem::size_of::<http_header>() as libc::c_ulong,
    ) as *mut http_header;
}
pub unsafe extern "C" fn http_response_init(
    mut w: *mut worker,
    mut code: libc::c_int,
    mut msg: *const libc::c_char,
) -> *mut http_response {
    let mut r: *mut http_response = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<http_response>() as libc::c_ulong,
    ) as *mut http_response;
    if r.is_null() {
        if !w.is_null() && !((*w).s).is_null() {
            slog(
                (*w).s,
                WEBDIS_ERROR,
                b"Failed to allocate http_response\0" as *const u8
                    as *const libc::c_char,
                0 as libc::c_int as size_t,
            );
        }
        return 0 as *mut http_response;
    }
    (*r).code = code as libc::c_short;
    (*r).msg = msg;
    (*r).w = w;
    (*r).keep_alive = 0 as libc::c_int;
    http_response_allocate_headers(r);
    if ((*r).headers).is_null() {
        if !w.is_null() && !((*w).s).is_null() {
            slog(
                (*w).s,
                WEBDIS_ERROR,
                b"Failed to allocate http_response headers\0" as *const u8
                    as *const libc::c_char,
                0 as libc::c_int as size_t,
            );
        }
        free(r as *mut libc::c_void);
        return 0 as *mut http_response;
    }
    http_response_set_header(
        r,
        b"Server\0" as *const u8 as *const libc::c_char,
        b"Webdis\0" as *const u8 as *const libc::c_char,
        HEADER_COPY_NONE,
    );
    http_response_set_header(
        r,
        b"Allow\0" as *const u8 as *const libc::c_char,
        b"GET,POST,PUT,OPTIONS\0" as *const u8 as *const libc::c_char,
        HEADER_COPY_NONE,
    );
    http_response_set_header(
        r,
        b"Access-Control-Allow-Methods\0" as *const u8 as *const libc::c_char,
        b"GET,POST,PUT,OPTIONS\0" as *const u8 as *const libc::c_char,
        HEADER_COPY_NONE,
    );
    http_response_set_header(
        r,
        b"Access-Control-Allow-Origin\0" as *const u8 as *const libc::c_char,
        b"*\0" as *const u8 as *const libc::c_char,
        HEADER_COPY_NONE,
    );
    http_response_set_header(
        r,
        b"Access-Control-Allow-Headers\0" as *const u8 as *const libc::c_char,
        b"X-Requested-With, Content-Type, Authorization\0" as *const u8
            as *const libc::c_char,
        HEADER_COPY_NONE,
    );
    return r;
}
pub unsafe extern "C" fn http_response_init_with_buffer(
    mut w: *mut worker,
    mut data: *mut libc::c_char,
    mut data_sz: size_t,
    mut keep_alive: libc::c_int,
) -> *mut http_response {
    let mut r: *mut http_response = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<http_response>() as libc::c_ulong,
    ) as *mut http_response;
    if r.is_null() {
        if !w.is_null() && !((*w).s).is_null() {
            slog(
                (*w).s,
                WEBDIS_ERROR,
                b"Failed to allocate http_response with buffer\0" as *const u8
                    as *const libc::c_char,
                0 as libc::c_int as size_t,
            );
        }
        return 0 as *mut http_response;
    }
    (*r).w = w;
    http_response_allocate_headers(r);
    if ((*r).headers).is_null() {
        if !w.is_null() && !((*w).s).is_null() {
            slog(
                (*w).s,
                WEBDIS_ERROR,
                b"Failed to allocate http_response headers\0" as *const u8
                    as *const libc::c_char,
                0 as libc::c_int as size_t,
            );
        }
        free(r as *mut libc::c_void);
        return 0 as *mut http_response;
    }
    (*r).out = data;
    (*r).out_sz = data_sz;
    (*r).sent = 0 as libc::c_int;
    (*r).keep_alive = keep_alive;
    return r;
}
pub unsafe extern "C" fn http_response_set_header(
    mut r: *mut http_response,
    mut k: *const libc::c_char,
    mut v: *const libc::c_char,
    mut copy: header_copy,
) {
    let mut i: libc::c_int = 0;
    let mut pos: libc::c_int = (*r).header_count;
    let mut replaced: libc::c_int = 0 as libc::c_int;
    let mut key_sz: size_t = strlen(k);
    let mut val_sz: size_t = strlen(v);
    if copy as libc::c_uint & HEADER_CHECK_DUPE as libc::c_int as libc::c_uint != 0 {
        i = 0 as libc::c_int;
        while i < (*r).header_count {
            if strncmp((*((*r).headers).offset(i as isize)).key, k, key_sz)
                == 0 as libc::c_int
            {
                pos = i;
                if (*((*r).headers).offset(i as isize)).copy as libc::c_uint
                    & HEADER_COPY_KEY as libc::c_int as libc::c_uint != 0
                {
                    free((*((*r).headers).offset(i as isize)).key as *mut libc::c_void);
                }
                if (*((*r).headers).offset(i as isize)).copy as libc::c_uint
                    & HEADER_COPY_VALUE as libc::c_int as libc::c_uint != 0
                {
                    free((*((*r).headers).offset(i as isize)).val as *mut libc::c_void);
                }
                replaced = 1 as libc::c_int;
                break;
            } else {
                i += 1;
                i;
            }
        }
    }
    if pos == (*r).headers_array_size {
        (*r)
            .headers = realloc(
            (*r).headers as *mut libc::c_void,
            (::std::mem::size_of::<http_header>() as libc::c_ulong)
                .wrapping_mul(
                    ((*r).headers_array_size + 1 as libc::c_int) as libc::c_ulong,
                ),
        ) as *mut http_header;
        (*r).headers_array_size += 1;
        (*r).headers_array_size;
    }
    if replaced == 0 {
        (*r).header_count += 1;
        (*r).header_count;
    }
    if copy as libc::c_uint & HEADER_COPY_KEY as libc::c_int as libc::c_uint != 0 {
        let ref mut fresh0 = (*((*r).headers).offset(pos as isize)).key;
        *fresh0 = calloc(
            key_sz.wrapping_add(1 as libc::c_int as libc::c_ulong),
            1 as libc::c_int as libc::c_ulong,
        ) as *mut libc::c_char;
        memcpy(
            (*((*r).headers).offset(pos as isize)).key as *mut libc::c_void,
            k as *const libc::c_void,
            key_sz,
        );
    } else {
        let ref mut fresh1 = (*((*r).headers).offset(pos as isize)).key;
        *fresh1 = k as *mut libc::c_char;
    }
    (*((*r).headers).offset(pos as isize)).key_sz = key_sz;
    if copy as libc::c_uint & HEADER_COPY_VALUE as libc::c_int as libc::c_uint != 0 {
        let ref mut fresh2 = (*((*r).headers).offset(pos as isize)).val;
        *fresh2 = calloc(
            val_sz.wrapping_add(1 as libc::c_int as libc::c_ulong),
            1 as libc::c_int as libc::c_ulong,
        ) as *mut libc::c_char;
        memcpy(
            (*((*r).headers).offset(pos as isize)).val as *mut libc::c_void,
            v as *const libc::c_void,
            val_sz,
        );
    } else {
        let ref mut fresh3 = (*((*r).headers).offset(pos as isize)).val;
        *fresh3 = v as *mut libc::c_char;
    }
    (*((*r).headers).offset(pos as isize)).val_sz = val_sz;
    (*((*r).headers).offset(pos as isize)).copy = copy;
    if (*r).chunked == 0
        && strcmp(k, b"Transfer-Encoding\0" as *const u8 as *const libc::c_char) == 0
        && strcmp(v, b"chunked\0" as *const u8 as *const libc::c_char) == 0
    {
        (*r).chunked = 1 as libc::c_int;
    }
}
pub unsafe extern "C" fn http_response_set_body(
    mut r: *mut http_response,
    mut body: *const libc::c_char,
    mut body_len: size_t,
) {
    (*r).body = body;
    (*r).body_len = body_len;
}
unsafe extern "C" fn http_response_cleanup(
    mut r: *mut http_response,
    mut fd: libc::c_int,
    mut success: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    free((*r).out as *mut libc::c_void);
    if ((*r).keep_alive == 0 || success == 0) && fd > 0 as libc::c_int {
        close(fd);
    }
    i = 0 as libc::c_int;
    while i < (*r).header_count {
        if (*((*r).headers).offset(i as isize)).copy as libc::c_uint
            & HEADER_COPY_KEY as libc::c_int as libc::c_uint != 0
        {
            free((*((*r).headers).offset(i as isize)).key as *mut libc::c_void);
        }
        if (*((*r).headers).offset(i as isize)).copy as libc::c_uint
            & HEADER_COPY_VALUE as libc::c_int as libc::c_uint != 0
        {
            free((*((*r).headers).offset(i as isize)).val as *mut libc::c_void);
        }
        i += 1;
        i;
    }
    free((*r).headers as *mut libc::c_void);
    free(r as *mut libc::c_void);
}
unsafe extern "C" fn http_can_write(
    mut fd: libc::c_int,
    mut event: libc::c_short,
    mut p: *mut libc::c_void,
) {
    let mut ret: libc::c_int = 0;
    let mut r: *mut http_response = p as *mut http_response;
    ret = write(
        fd,
        ((*r).out).offset((*r).sent as isize) as *const libc::c_void,
        ((*r).out_sz).wrapping_sub((*r).sent as libc::c_ulong),
    ) as libc::c_int;
    if ret > 0 as libc::c_int {
        (*r).sent += ret;
    }
    if ret <= 0 as libc::c_int
        || ((*r).out_sz).wrapping_sub((*r).sent as libc::c_ulong)
            == 0 as libc::c_int as libc::c_ulong
    {
        http_response_cleanup(
            r,
            fd,
            if (*r).out_sz as libc::c_int == (*r).sent {
                1 as libc::c_int
            } else {
                0 as libc::c_int
            },
        );
    } else {
        http_schedule_write(fd, r);
    };
}
pub unsafe extern "C" fn http_schedule_write(
    mut fd: libc::c_int,
    mut r: *mut http_response,
) {
    if !((*r).w).is_null() {
        event_set(
            &mut (*r).ev,
            fd,
            0x4 as libc::c_int as libc::c_short,
            Some(
                http_can_write
                    as unsafe extern "C" fn(
                        libc::c_int,
                        libc::c_short,
                        *mut libc::c_void,
                    ) -> (),
            ),
            r as *mut libc::c_void,
        );
        event_base_set((*(*r).w).base, &mut (*r).ev);
        let mut ret: libc::c_int = event_add(&mut (*r).ev, 0 as *const timeval);
        if ret != 0 as libc::c_int {
            slog(
                (*(*r).w).s,
                WEBDIS_ERROR,
                b"Could not schedule HTTP write\0" as *const u8 as *const libc::c_char,
                0 as libc::c_int as size_t,
            );
            http_response_cleanup(r, fd, 0 as libc::c_int);
        }
    } else {
        http_can_write(fd, 0 as libc::c_int as libc::c_short, r as *mut libc::c_void);
    };
}
unsafe extern "C" fn format_chunk(
    mut p: *const libc::c_char,
    mut sz: size_t,
    mut out_sz: *mut size_t,
) -> *mut libc::c_char {
    let mut out: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: [libc::c_char; 64] = [0; 64];
    let mut chunk_size: libc::c_int = 0;
    chunk_size = sprintf(
        tmp.as_mut_ptr(),
        b"%x\r\n\0" as *const u8 as *const libc::c_char,
        sz as libc::c_int,
    );
    *out_sz = (chunk_size as libc::c_ulong)
        .wrapping_add(sz)
        .wrapping_add(2 as libc::c_int as libc::c_ulong);
    out = malloc(*out_sz) as *mut libc::c_char;
    memcpy(
        out as *mut libc::c_void,
        tmp.as_mut_ptr() as *const libc::c_void,
        chunk_size as libc::c_ulong,
    );
    memcpy(
        out.offset(chunk_size as isize) as *mut libc::c_void,
        p as *const libc::c_void,
        sz,
    );
    memcpy(
        out.offset(chunk_size as isize).offset(sz as isize) as *mut libc::c_void,
        b"\r\n\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        2 as libc::c_int as libc::c_ulong,
    );
    return out;
}
pub unsafe extern "C" fn http_response_write(
    mut r: *mut http_response,
    mut fd: libc::c_int,
) {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    if fd < 0 as libc::c_int {
        http_response_cleanup(r, fd, 0 as libc::c_int);
        return;
    }
    (*r)
        .out_sz = (::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        .wrapping_add(strlen((*r).msg))
        .wrapping_add(2 as libc::c_int as libc::c_ulong);
    (*r)
        .out = calloc(
        ((*r).out_sz).wrapping_add(1 as libc::c_int as libc::c_ulong),
        1 as libc::c_int as libc::c_ulong,
    ) as *mut libc::c_char;
    ret = sprintf(
        (*r).out,
        b"HTTP/1.%d %d %s\r\n\0" as *const u8 as *const libc::c_char,
        if (*r).http_version != 0 { 1 as libc::c_int } else { 0 as libc::c_int },
        (*r).code as libc::c_int,
        (*r).msg,
    );
    p = (*r).out;
    if (*r).chunked == 0 {
        if (*r).code as libc::c_int == 200 as libc::c_int && !((*r).body).is_null() {
            let mut content_length: [libc::c_char; 22] = [0; 22];
            sprintf(
                content_length.as_mut_ptr(),
                b"%zd\0" as *const u8 as *const libc::c_char,
                (*r).body_len,
            );
            http_response_set_header(
                r,
                b"Content-Length\0" as *const u8 as *const libc::c_char,
                content_length.as_mut_ptr(),
                (HEADER_COPY_VALUE as libc::c_int | HEADER_CHECK_DUPE as libc::c_int)
                    as header_copy,
            );
        } else {
            http_response_set_header(
                r,
                b"Content-Length\0" as *const u8 as *const libc::c_char,
                b"0\0" as *const u8 as *const libc::c_char,
                (HEADER_COPY_NONE as libc::c_int | HEADER_CHECK_DUPE as libc::c_int)
                    as header_copy,
            );
        }
    }
    i = 0 as libc::c_int;
    while i < (*r).header_count {
        let mut header_sz: size_t = ((*((*r).headers).offset(i as isize)).key_sz)
            .wrapping_add(2 as libc::c_int as libc::c_ulong)
            .wrapping_add((*((*r).headers).offset(i as isize)).val_sz)
            .wrapping_add(2 as libc::c_int as libc::c_ulong);
        (*r)
            .out = realloc(
            (*r).out as *mut libc::c_void,
            ((*r).out_sz).wrapping_add(header_sz),
        ) as *mut libc::c_char;
        p = ((*r).out).offset((*r).out_sz as isize);
        memcpy(
            p as *mut libc::c_void,
            (*((*r).headers).offset(i as isize)).key as *const libc::c_void,
            (*((*r).headers).offset(i as isize)).key_sz,
        );
        p = p.offset((*((*r).headers).offset(i as isize)).key_sz as isize);
        let fresh4 = p;
        p = p.offset(1);
        *fresh4 = ':' as i32 as libc::c_char;
        let fresh5 = p;
        p = p.offset(1);
        *fresh5 = ' ' as i32 as libc::c_char;
        memcpy(
            p as *mut libc::c_void,
            (*((*r).headers).offset(i as isize)).val as *const libc::c_void,
            (*((*r).headers).offset(i as isize)).val_sz,
        );
        p = p.offset((*((*r).headers).offset(i as isize)).val_sz as isize);
        let fresh6 = p;
        p = p.offset(1);
        *fresh6 = '\r' as i32 as libc::c_char;
        let fresh7 = p;
        p = p.offset(1);
        *fresh7 = '\n' as i32 as libc::c_char;
        (*r)
            .out_sz = ((*r).out_sz as libc::c_ulong).wrapping_add(header_sz) as size_t
            as size_t;
        if strncasecmp(
            b"Connection\0" as *const u8 as *const libc::c_char,
            (*((*r).headers).offset(i as isize)).key,
            (*((*r).headers).offset(i as isize)).key_sz,
        ) == 0 as libc::c_int
            && strncasecmp(
                b"Keep-Alive\0" as *const u8 as *const libc::c_char,
                (*((*r).headers).offset(i as isize)).val,
                (*((*r).headers).offset(i as isize)).val_sz,
            ) == 0 as libc::c_int
        {
            (*r).keep_alive = 1 as libc::c_int;
        }
        i += 1;
        i;
    }
    (*r)
        .out = realloc(
        (*r).out as *mut libc::c_void,
        ((*r).out_sz).wrapping_add(2 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    memcpy(
        ((*r).out).offset((*r).out_sz as isize) as *mut libc::c_void,
        b"\r\n\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        2 as libc::c_int as libc::c_ulong,
    );
    (*r)
        .out_sz = ((*r).out_sz as libc::c_ulong)
        .wrapping_add(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
    if !((*r).body).is_null() && (*r).body_len != 0 {
        let mut tmp: *mut libc::c_char = (*r).body as *mut libc::c_char;
        let mut tmp_len: size_t = (*r).body_len;
        if (*r).chunked != 0 {
            tmp = format_chunk((*r).body, (*r).body_len, &mut tmp_len);
        }
        (*r)
            .out = realloc(
            (*r).out as *mut libc::c_void,
            ((*r).out_sz).wrapping_add(tmp_len),
        ) as *mut libc::c_char;
        memcpy(
            ((*r).out).offset((*r).out_sz as isize) as *mut libc::c_void,
            tmp as *const libc::c_void,
            tmp_len,
        );
        (*r)
            .out_sz = ((*r).out_sz as libc::c_ulong).wrapping_add(tmp_len) as size_t
            as size_t;
        if (*r).chunked != 0 {
            free(tmp as *mut libc::c_void);
        }
    }
    (*r).sent = 0 as libc::c_int;
    http_schedule_write(fd, r);
}
unsafe extern "C" fn http_response_set_connection_header(
    mut c: *mut http_client,
    mut r: *mut http_response,
) {
    http_response_set_keep_alive(r, (*c).keep_alive as libc::c_int);
}
pub unsafe extern "C" fn http_crossdomain(mut c: *mut http_client) {
    let mut resp: *mut http_response = http_response_init(
        0 as *mut worker,
        200 as libc::c_int,
        b"OK\0" as *const u8 as *const libc::c_char,
    );
    let mut out: [libc::c_char; 200] = *::std::mem::transmute::<
        &[u8; 200],
        &mut [libc::c_char; 200],
    >(
        b"<?xml version=\"1.0\"?>\n<!DOCTYPE cross-domain-policy SYSTEM \"http://www.macromedia.com/xml/dtds/cross-domain-policy.dtd\">\n<cross-domain-policy>\n<allow-access-from domain=\"*\" />\n</cross-domain-policy>\n\0",
    );
    (*resp).http_version = (*c).http_version as libc::c_int;
    http_response_set_connection_header(c, resp);
    http_response_set_header(
        resp,
        b"Content-Type\0" as *const u8 as *const libc::c_char,
        b"application/xml\0" as *const u8 as *const libc::c_char,
        HEADER_COPY_NONE,
    );
    http_response_set_body(
        resp,
        out.as_mut_ptr(),
        (::std::mem::size_of::<[libc::c_char; 200]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    );
    http_response_write(resp, (*c).fd);
    http_client_reset(c);
}
pub unsafe extern "C" fn http_send_error(
    mut c: *mut http_client,
    mut code: libc::c_short,
    mut msg: *const libc::c_char,
) {
    let mut resp: *mut http_response = http_response_init(
        0 as *mut worker,
        code as libc::c_int,
        msg,
    );
    (*resp).http_version = (*c).http_version as libc::c_int;
    http_response_set_connection_header(c, resp);
    http_response_set_body(resp, 0 as *const libc::c_char, 0 as libc::c_int as size_t);
    http_response_write(resp, (*c).fd);
    http_client_reset(c);
}
pub unsafe extern "C" fn http_response_set_keep_alive(
    mut r: *mut http_response,
    mut enabled: libc::c_int,
) {
    (*r).keep_alive = enabled;
    if enabled != 0 {
        http_response_set_header(
            r,
            b"Connection\0" as *const u8 as *const libc::c_char,
            b"Keep-Alive\0" as *const u8 as *const libc::c_char,
            HEADER_COPY_NONE,
        );
    } else {
        http_response_set_header(
            r,
            b"Connection\0" as *const u8 as *const libc::c_char,
            b"Close\0" as *const u8 as *const libc::c_char,
            HEADER_COPY_NONE,
        );
    };
}
pub unsafe extern "C" fn http_send_options(mut c: *mut http_client) {
    let mut resp: *mut http_response = http_response_init(
        0 as *mut worker,
        200 as libc::c_int,
        b"OK\0" as *const u8 as *const libc::c_char,
    );
    (*resp).http_version = (*c).http_version as libc::c_int;
    http_response_set_connection_header(c, resp);
    http_response_set_header(
        resp,
        b"Content-Type\0" as *const u8 as *const libc::c_char,
        b"text/html\0" as *const u8 as *const libc::c_char,
        HEADER_COPY_NONE,
    );
    http_response_set_header(
        resp,
        b"Content-Length\0" as *const u8 as *const libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char,
        HEADER_COPY_NONE,
    );
    http_response_write(resp, (*c).fd);
    http_client_reset(c);
}
pub unsafe extern "C" fn http_response_write_chunk(
    mut fd: libc::c_int,
    mut w: *mut worker,
    mut p: *const libc::c_char,
    mut sz: size_t,
) {
    let mut r: *mut http_response = http_response_init(
        w,
        0 as libc::c_int,
        0 as *const libc::c_char,
    );
    (*r).keep_alive = 1 as libc::c_int;
    (*r).out = format_chunk(p, sz, &mut (*r).out_sz);
    http_schedule_write(fd, r);
}
