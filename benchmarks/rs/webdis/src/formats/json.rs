use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type dict;
    pub type sockadr;
    pub type worker;
    pub type evbuffer;
    pub type server;
    pub type event_base;
    fn json_object() -> *mut json_t;
    fn json_array() -> *mut json_t;
    fn json_string(value: *const libc::c_char) -> *mut json_t;
    fn json_integer(value: json_int_t) -> *mut json_t;
    fn json_true() -> *mut json_t;
    fn json_false() -> *mut json_t;
    fn json_null() -> *mut json_t;
    fn json_delete(json: *mut json_t);
    fn json_object_set_new(
        object: *mut json_t,
        key: *const libc::c_char,
        value: *mut json_t,
    ) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn json_array_size(array: *const json_t) -> size_t;
    fn json_array_get(array: *const json_t, index: size_t) -> *mut json_t;
    fn json_array_append_new(array: *mut json_t, value: *mut json_t) -> libc::c_int;
    fn json_string_value(string: *const json_t) -> *const libc::c_char;
    fn json_integer_value(integer: *const json_t) -> json_int_t;
    fn json_loads(
        input: *const libc::c_char,
        flags: size_t,
        error: *mut json_error_t,
    ) -> *mut json_t;
    fn json_dumps(json: *const json_t, flags: size_t) -> *mut libc::c_char;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn format_send_reply(
        cmd: *mut cmd,
        p: *const libc::c_char,
        sz: size_t,
        content_type: *const libc::c_char,
    );
    fn format_send_error(cmd: *mut cmd, code: libc::c_short, msg: *const libc::c_char);
    fn cmd_new(c: *mut http_client, count: libc::c_int) -> *mut cmd;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strdup(__s: *const libc::c_char) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strcasecmp(__s1: *const libc::c_char, __s2: *const libc::c_char) -> libc::c_int;
    fn strncasecmp(
        __s1: *const libc::c_char,
        __s2: *const libc::c_char,
        __n: size_t,
    ) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type ssize_t = __ssize_t;
pub type json_type = libc::c_uint;
pub const JSON_NULL: json_type = 7;
pub const JSON_FALSE: json_type = 6;
pub const JSON_TRUE: json_type = 5;
pub const JSON_REAL: json_type = 4;
pub const JSON_INTEGER: json_type = 3;
pub const JSON_STRING: json_type = 2;
pub const JSON_ARRAY: json_type = 1;
pub const JSON_OBJECT: json_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct json_t {
    pub type_0: json_type,
    pub refcount: size_t,
}
pub type json_int_t = libc::c_longlong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct json_error_t {
    pub text: [libc::c_char; 160],
    pub line: libc::c_int,
    pub column: libc::c_int,
    pub source: [libc::c_char; 80],
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
pub type in_addr_t = uint32_t;
#[inline]
unsafe extern "C" fn json_decref(mut json: *mut json_t) {
    if !json.is_null() && (*json).refcount != -(1 as libc::c_int) as size_t
        && {
            (*json).refcount = ((*json).refcount).wrapping_sub(1);
            (*json).refcount == 0 as libc::c_int as libc::c_ulong
        }
    {
        json_delete(json);
    }
}
pub unsafe extern "C" fn json_reply(
    mut c: *mut redisAsyncContext,
    mut r: *mut libc::c_void,
    mut privdata: *mut libc::c_void,
) {
    let mut reply: *mut redisReply = r as *mut redisReply;
    let mut cmd: *mut cmd = privdata as *mut cmd;
    let mut j: *mut json_t = 0 as *mut json_t;
    let mut jstr: *mut libc::c_char = 0 as *mut libc::c_char;
    if cmd.is_null() {
        return;
    }
    if reply.is_null() {
        format_send_error(
            cmd,
            503 as libc::c_int as libc::c_short,
            b"Service Unavailable\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    j = json_wrap_redis_reply(cmd, r as *const redisReply);
    jstr = json_string_output(j, (*cmd).jsonp);
    format_send_reply(
        cmd,
        jstr,
        strlen(jstr),
        b"application/json\0" as *const u8 as *const libc::c_char,
    );
    json_decref(j);
    free(jstr as *mut libc::c_void);
}
unsafe extern "C" fn json_info_reply(mut s: *const libc::c_char) -> *mut json_t {
    let mut p: *const libc::c_char = s;
    let mut sz: size_t = strlen(s);
    let mut jroot: *mut json_t = json_object();
    while p < s.offset(sz as isize) {
        let mut key: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut val: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut nl: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut colon: *mut libc::c_char = 0 as *mut libc::c_char;
        colon = strchr(p, ':' as i32);
        if colon.is_null() {
            break;
        }
        key = calloc(
            (colon.offset_from(p) as libc::c_long + 1 as libc::c_int as libc::c_long)
                as libc::c_ulong,
            1 as libc::c_int as libc::c_ulong,
        ) as *mut libc::c_char;
        memcpy(
            key as *mut libc::c_void,
            p as *const libc::c_void,
            colon.offset_from(p) as libc::c_long as libc::c_ulong,
        );
        p = colon.offset(1 as libc::c_int as isize);
        nl = strchr(p, '\r' as i32);
        if nl.is_null() {
            free(key as *mut libc::c_void);
            break;
        } else {
            val = calloc(
                (nl.offset_from(p) as libc::c_long + 1 as libc::c_int as libc::c_long)
                    as libc::c_ulong,
                1 as libc::c_int as libc::c_ulong,
            ) as *mut libc::c_char;
            memcpy(
                val as *mut libc::c_void,
                p as *const libc::c_void,
                nl.offset_from(p) as libc::c_long as libc::c_ulong,
            );
            p = nl.offset(1 as libc::c_int as isize);
            if *p as libc::c_int == '\n' as i32 {
                p = p.offset(1);
                p;
            }
            json_object_set_new(jroot, key, json_string(val));
            free(key as *mut libc::c_void);
            free(val as *mut libc::c_void);
        }
    }
    return jroot;
}
unsafe extern "C" fn json_array_to_keyvalue_reply(
    mut r: *const redisReply,
) -> *mut json_t {
    let mut jroot: *mut json_t = 0 as *mut json_t;
    let mut jlist: *mut json_t = 0 as *mut json_t;
    let mut i: libc::c_uint = 0;
    if ((*r).elements).wrapping_rem(2 as libc::c_int as libc::c_ulong)
        != 0 as libc::c_int as libc::c_ulong
    {
        return 0 as *mut json_t;
    }
    jroot = json_object();
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong) < (*r).elements {
        let mut k: *mut redisReply = *((*r).element).offset(i as isize);
        let mut v: *mut redisReply = *((*r).element)
            .offset(i.wrapping_add(1 as libc::c_int as libc::c_uint) as isize);
        if (*k).type_0 != 1 as libc::c_int {
            json_decref(jroot);
            return 0 as *mut json_t;
        }
        match (*v).type_0 {
            4 => {
                json_object_set_new(jroot, (*k).str_0, json_null());
            }
            1 => {
                json_object_set_new(jroot, (*k).str_0, json_string((*v).str_0));
            }
            3 => {
                json_object_set_new(jroot, (*k).str_0, json_integer((*v).integer));
            }
            2 => {
                jlist = json_expand_array(v);
                if jlist.is_null() {
                    jlist = json_null();
                }
                json_object_set_new(jroot, (*k).str_0, jlist);
            }
            _ => {
                json_decref(jroot);
                return 0 as *mut json_t;
            }
        }
        i = i.wrapping_add(2 as libc::c_int as libc::c_uint);
    }
    return jroot;
}
unsafe extern "C" fn json_expand_array(mut r: *const redisReply) -> *mut json_t {
    let mut i: libc::c_uint = 0;
    let mut jlist: *mut json_t = 0 as *mut json_t;
    let mut sublist: *mut json_t = 0 as *mut json_t;
    let mut e: *const redisReply = 0 as *const redisReply;
    jlist = json_array();
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong) < (*r).elements {
        e = *((*r).element).offset(i as isize);
        match (*e).type_0 {
            5 | 1 => {
                json_array_append_new(jlist, json_string((*e).str_0));
            }
            3 => {
                json_array_append_new(jlist, json_integer((*e).integer));
            }
            2 => {
                sublist = json_expand_array(e);
                if sublist.is_null() {
                    sublist = json_null();
                }
                json_array_append_new(jlist, sublist);
            }
            4 | _ => {
                json_array_append_new(jlist, json_null());
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    return jlist;
}
unsafe extern "C" fn json_singlestream_list(mut r: *const redisReply) -> *mut json_t {
    let mut i: libc::c_uint = 0;
    let mut jlist: *mut json_t = 0 as *mut json_t;
    let mut jmsg: *mut json_t = 0 as *mut json_t;
    let mut jsubmsg: *mut json_t = 0 as *mut json_t;
    let mut id: *const redisReply = 0 as *const redisReply;
    let mut msg: *const redisReply = 0 as *const redisReply;
    let mut e: *const redisReply = 0 as *const redisReply;
    jlist = json_array();
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong) < (*r).elements {
        e = *((*r).element).offset(i as isize);
        if !((*e).type_0 != 2 as libc::c_int
            || (*e).elements < 2 as libc::c_int as libc::c_ulong)
        {
            id = *((*e).element).offset(0 as libc::c_int as isize);
            msg = *((*e).element).offset(1 as libc::c_int as isize);
            if !((*id).type_0 != 1 as libc::c_int
                || (*id).len < 1 as libc::c_int as libc::c_ulong)
            {
                if !((*msg).type_0 != 2 as libc::c_int
                    || (*msg).elements < 2 as libc::c_int as libc::c_ulong)
                {
                    jmsg = json_object();
                    json_object_set_new(
                        jmsg,
                        b"id\0" as *const u8 as *const libc::c_char,
                        json_string((*id).str_0),
                    );
                    jsubmsg = json_array_to_keyvalue_reply(msg);
                    if jsubmsg.is_null() {
                        jsubmsg = json_null();
                    }
                    json_object_set_new(
                        jmsg,
                        b"msg\0" as *const u8 as *const libc::c_char,
                        jsubmsg,
                    );
                    json_array_append_new(jlist, jmsg);
                }
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    return jlist;
}
unsafe extern "C" fn json_xreadstream_list(mut r: *const redisReply) -> *mut json_t {
    let mut i: libc::c_uint = 0;
    let mut jobj: *mut json_t = 0 as *mut json_t;
    let mut jlist: *mut json_t = 0 as *mut json_t;
    let mut sid: *const redisReply = 0 as *const redisReply;
    let mut msglist: *const redisReply = 0 as *const redisReply;
    let mut e: *const redisReply = 0 as *const redisReply;
    if (*r).elements != 0 {
        jobj = json_object();
    }
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong) < (*r).elements {
        e = *((*r).element).offset(i as isize);
        if !((*e).type_0 != 2 as libc::c_int
            || (*e).elements < 2 as libc::c_int as libc::c_ulong)
        {
            sid = *((*e).element).offset(0 as libc::c_int as isize);
            msglist = *((*e).element).offset(1 as libc::c_int as isize);
            if !((*sid).type_0 != 1 as libc::c_int
                || (*sid).len < 1 as libc::c_int as libc::c_ulong)
            {
                if !((*msglist).type_0 != 2 as libc::c_int) {
                    jlist = json_singlestream_list(msglist);
                    if jlist.is_null() {
                        jlist = json_null();
                    }
                    json_object_set_new(jobj, (*sid).str_0, jlist);
                }
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    return jobj;
}
unsafe extern "C" fn json_xpending_list(mut r: *const redisReply) -> *mut json_t {
    let mut i: libc::c_uint = 0;
    let mut jobj: *mut json_t = 0 as *mut json_t;
    let mut jlist: *mut json_t = 0 as *mut json_t;
    let mut jown: *mut json_t = 0 as *mut json_t;
    let mut own: *const redisReply = 0 as *const redisReply;
    let mut msgs: *const redisReply = 0 as *const redisReply;
    let mut e: *const redisReply = 0 as *const redisReply;
    if (*r).elements >= 4 as libc::c_int as libc::c_ulong
        && (**((*r).element).offset(0 as libc::c_int as isize)).type_0
            == 3 as libc::c_int
    {
        jobj = json_object();
        json_object_set_new(
            jobj,
            b"msgs\0" as *const u8 as *const libc::c_char,
            json_integer((**((*r).element).offset(0 as libc::c_int as isize)).integer),
        );
        if (**((*r).element).offset(1 as libc::c_int as isize)).type_0
            == 1 as libc::c_int
        {
            json_object_set_new(
                jobj,
                b"idmin\0" as *const u8 as *const libc::c_char,
                json_string((**((*r).element).offset(1 as libc::c_int as isize)).str_0),
            );
        }
        if (**((*r).element).offset(2 as libc::c_int as isize)).type_0
            == 1 as libc::c_int
        {
            json_object_set_new(
                jobj,
                b"idmax\0" as *const u8 as *const libc::c_char,
                json_string((**((*r).element).offset(2 as libc::c_int as isize)).str_0),
            );
        }
        if (**((*r).element).offset(3 as libc::c_int as isize)).type_0
            != 2 as libc::c_int
        {
            return jobj;
        }
        jown = json_object();
        i = 0 as libc::c_int as libc::c_uint;
        while (i as libc::c_ulong)
            < (**((*r).element).offset(3 as libc::c_int as isize)).elements
        {
            e = *((**((*r).element).offset(3 as libc::c_int as isize)).element)
                .offset(i as isize);
            if !((*e).type_0 != 2 as libc::c_int
                || (*e).elements < 2 as libc::c_int as libc::c_ulong)
            {
                own = *((*e).element).offset(0 as libc::c_int as isize);
                msgs = *((*e).element).offset(1 as libc::c_int as isize);
                if !((*own).type_0 != 1 as libc::c_int) {
                    match (*msgs).type_0 {
                        1 => {
                            json_object_set_new(
                                jown,
                                (*own).str_0,
                                json_string((*msgs).str_0),
                            );
                        }
                        3 => {
                            json_object_set_new(
                                jown,
                                (*own).str_0,
                                json_integer((*msgs).integer),
                            );
                        }
                        _ => {}
                    }
                }
            }
            i = i.wrapping_add(1);
            i;
        }
        json_object_set_new(
            jobj,
            b"msgsperconsumer\0" as *const u8 as *const libc::c_char,
            jown,
        );
        return jobj;
    }
    jlist = json_array();
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong) < (*r).elements {
        e = *((*r).element).offset(i as isize);
        if !((*e).type_0 != 2 as libc::c_int
            || (*e).elements < 4 as libc::c_int as libc::c_ulong)
        {
            jobj = json_object();
            if (**((*e).element).offset(0 as libc::c_int as isize)).type_0
                == 1 as libc::c_int
            {
                json_object_set_new(
                    jobj,
                    b"id\0" as *const u8 as *const libc::c_char,
                    json_string(
                        (**((*e).element).offset(0 as libc::c_int as isize)).str_0,
                    ),
                );
            }
            if (**((*e).element).offset(1 as libc::c_int as isize)).type_0
                == 1 as libc::c_int
            {
                json_object_set_new(
                    jobj,
                    b"owner\0" as *const u8 as *const libc::c_char,
                    json_string(
                        (**((*e).element).offset(1 as libc::c_int as isize)).str_0,
                    ),
                );
            }
            if (**((*e).element).offset(2 as libc::c_int as isize)).type_0
                == 3 as libc::c_int
            {
                json_object_set_new(
                    jobj,
                    b"elapsedtime\0" as *const u8 as *const libc::c_char,
                    json_integer(
                        (**((*e).element).offset(2 as libc::c_int as isize)).integer,
                    ),
                );
            }
            if (**((*e).element).offset(3 as libc::c_int as isize)).type_0
                == 3 as libc::c_int
            {
                json_object_set_new(
                    jobj,
                    b"deliveries\0" as *const u8 as *const libc::c_char,
                    json_integer(
                        (**((*e).element).offset(3 as libc::c_int as isize)).integer,
                    ),
                );
            }
            json_array_append_new(jlist, jobj);
        }
        i = i.wrapping_add(1);
        i;
    }
    return jlist;
}
unsafe extern "C" fn json_georadius_with_list(mut r: *const redisReply) -> *mut json_t {
    let mut i: libc::c_uint = 0;
    let mut j: libc::c_uint = 0;
    let mut jobj: *mut json_t = 0 as *mut json_t;
    let mut jlist: *mut json_t = 0 as *mut json_t;
    let mut jcoo: *mut json_t = 0 as *mut json_t;
    let mut e: *const redisReply = 0 as *const redisReply;
    jlist = json_array();
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong) < (*r).elements {
        e = *((*r).element).offset(i as isize);
        if !((*e).type_0 != 2 as libc::c_int
            || (*e).elements < 1 as libc::c_int as libc::c_ulong)
        {
            jobj = json_object();
            json_object_set_new(
                jobj,
                b"name\0" as *const u8 as *const libc::c_char,
                json_string((**((*e).element).offset(0 as libc::c_int as isize)).str_0),
            );
            j = 1 as libc::c_int as libc::c_uint;
            while (j as libc::c_ulong) < (*e).elements {
                match (**((*e).element).offset(j as isize)).type_0 {
                    3 => {
                        json_object_set_new(
                            jobj,
                            b"hash\0" as *const u8 as *const libc::c_char,
                            json_integer((**((*e).element).offset(j as isize)).integer),
                        );
                    }
                    1 => {
                        json_object_set_new(
                            jobj,
                            b"dist\0" as *const u8 as *const libc::c_char,
                            json_string((**((*e).element).offset(j as isize)).str_0),
                        );
                    }
                    2 => {
                        if !((**((*e).element).offset(j as isize)).type_0
                            != 2 as libc::c_int
                            || (**((*e).element).offset(j as isize)).elements
                                != 2 as libc::c_int as libc::c_ulong)
                        {
                            if !((**((**((*e).element).offset(j as isize)).element)
                                .offset(0 as libc::c_int as isize))
                                .type_0 != 1 as libc::c_int
                                || (**((**((*e).element).offset(j as isize)).element)
                                    .offset(1 as libc::c_int as isize))
                                    .type_0 != 1 as libc::c_int)
                            {
                                jcoo = json_array();
                                json_array_append_new(
                                    jcoo,
                                    json_string(
                                        (**((**((*e).element).offset(j as isize)).element)
                                            .offset(0 as libc::c_int as isize))
                                            .str_0,
                                    ),
                                );
                                json_array_append_new(
                                    jcoo,
                                    json_string(
                                        (**((**((*e).element).offset(j as isize)).element)
                                            .offset(1 as libc::c_int as isize))
                                            .str_0,
                                    ),
                                );
                                json_object_set_new(
                                    jobj,
                                    b"coords\0" as *const u8 as *const libc::c_char,
                                    jcoo,
                                );
                            }
                        }
                    }
                    _ => {}
                }
                j = j.wrapping_add(1);
                j;
            }
            json_array_append_new(jlist, jobj);
        }
        i = i.wrapping_add(1);
        i;
    }
    return jlist;
}
unsafe extern "C" fn json_wrap_redis_reply(
    mut cmd: *const cmd,
    mut r: *const redisReply,
) -> *mut json_t {
    let mut jlist: *mut json_t = 0 as *mut json_t;
    let mut jobj: *mut json_t = 0 as *mut json_t;
    let mut jroot: *mut json_t = json_object();
    let mut verb: *mut libc::c_char = 0 as *mut libc::c_char;
    if (*cmd).count != 0 {
        verb = calloc(
            (*((*cmd).argv_len).offset(0 as libc::c_int as isize))
                .wrapping_add(1 as libc::c_int as libc::c_ulong),
            1 as libc::c_int as libc::c_ulong,
        ) as *mut libc::c_char;
        memcpy(
            verb as *mut libc::c_void,
            *((*cmd).argv).offset(0 as libc::c_int as isize) as *const libc::c_void,
            *((*cmd).argv_len).offset(0 as libc::c_int as isize),
        );
    } else {
        verb = strdup(b"\0" as *const u8 as *const libc::c_char);
    }
    match (*r).type_0 {
        5 | 6 => {
            jlist = json_array();
            json_array_append_new(
                jlist,
                if (*r).type_0 == 6 as libc::c_int { json_false() } else { json_true() },
            );
            json_array_append_new(jlist, json_string((*r).str_0));
            json_object_set_new(jroot, verb, jlist);
        }
        1 => {
            if strcasecmp(verb, b"INFO\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                json_object_set_new(jroot, verb, json_info_reply((*r).str_0));
            } else {
                json_object_set_new(jroot, verb, json_string((*r).str_0));
            }
        }
        3 => {
            json_object_set_new(jroot, verb, json_integer((*r).integer));
        }
        2 => {
            if strcasecmp(verb, b"HGETALL\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                jobj = json_array_to_keyvalue_reply(r);
                if !jobj.is_null() {
                    json_object_set_new(jroot, verb, jobj);
                }
            } else if strcasecmp(verb, b"XRANGE\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
                || strcasecmp(verb, b"XREVRANGE\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                || strcasecmp(verb, b"XCLAIM\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                    && (*r).elements > 0 as libc::c_int as libc::c_ulong
                    && (**((*r).element).offset(0 as libc::c_int as isize)).type_0
                        == 2 as libc::c_int
            {
                jobj = json_singlestream_list(r);
                if !jobj.is_null() {
                    json_object_set_new(jroot, verb, jobj);
                }
            } else if strcasecmp(verb, b"XREAD\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
                || strcasecmp(verb, b"XREADGROUP\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
            {
                jobj = json_xreadstream_list(r);
                if !jobj.is_null() {
                    json_object_set_new(jroot, verb, jobj);
                }
            } else if strcasecmp(verb, b"XPENDING\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                jobj = json_xpending_list(r);
                if !jobj.is_null() {
                    json_object_set_new(jroot, verb, jobj);
                }
            } else if strncasecmp(
                verb,
                b"GEORADIUS\0" as *const u8 as *const libc::c_char,
                9 as libc::c_int as size_t,
            ) == 0 as libc::c_int && (*r).elements > 0 as libc::c_int as libc::c_ulong
                && (**((*r).element).offset(0 as libc::c_int as isize)).type_0
                    == 2 as libc::c_int
            {
                jobj = json_georadius_with_list(r);
                if !jobj.is_null() {
                    json_object_set_new(jroot, verb, jobj);
                }
            } else {
                jlist = json_expand_array(r);
                if jlist.is_null() {
                    jlist = json_null();
                }
                json_object_set_new(jroot, verb, jlist);
            }
        }
        4 | _ => {
            json_object_set_new(jroot, verb, json_null());
        }
    }
    free(verb as *mut libc::c_void);
    return jroot;
}
pub unsafe extern "C" fn json_string_output(
    mut j: *mut json_t,
    mut jsonp: *const libc::c_char,
) -> *mut libc::c_char {
    let mut json_reply_0: *mut libc::c_char = json_dumps(
        j,
        0x20 as libc::c_int as size_t,
    );
    if !jsonp.is_null() {
        let mut jsonp_len: size_t = strlen(jsonp);
        let mut json_len: size_t = strlen(json_reply_0);
        let mut ret_len: size_t = jsonp_len
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_add(json_len)
            .wrapping_add(3 as libc::c_int as libc::c_ulong);
        let mut ret: *mut libc::c_char = calloc(
            (1 as libc::c_int as libc::c_ulong).wrapping_add(ret_len),
            1 as libc::c_int as libc::c_ulong,
        ) as *mut libc::c_char;
        memcpy(ret as *mut libc::c_void, jsonp as *const libc::c_void, jsonp_len);
        *ret.offset(jsonp_len as isize) = '(' as i32 as libc::c_char;
        memcpy(
            ret.offset(jsonp_len as isize).offset(1 as libc::c_int as isize)
                as *mut libc::c_void,
            json_reply_0 as *const libc::c_void,
            json_len,
        );
        memcpy(
            ret
                .offset(jsonp_len as isize)
                .offset(1 as libc::c_int as isize)
                .offset(json_len as isize) as *mut libc::c_void,
            b");\n\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            3 as libc::c_int as libc::c_ulong,
        );
        free(json_reply_0 as *mut libc::c_void);
        return ret;
    }
    return json_reply_0;
}
pub unsafe extern "C" fn json_ws_extract(
    mut c: *mut http_client,
    mut p: *const libc::c_char,
    mut sz: size_t,
) -> *mut cmd {
    let mut cmd: *mut cmd = 0 as *mut cmd;
    let mut j: *mut json_t = 0 as *mut json_t;
    let mut jsonz: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_uint = 0;
    let mut cur: libc::c_uint = 0;
    let mut argc: libc::c_int = 0 as libc::c_int;
    let mut jerror: json_error_t = json_error_t {
        text: [0; 160],
        line: 0,
        column: 0,
        source: [0; 80],
    };
    jsonz = calloc(
        sz.wrapping_add(1 as libc::c_int as libc::c_ulong),
        1 as libc::c_int as libc::c_ulong,
    ) as *mut libc::c_char;
    memcpy(jsonz as *mut libc::c_void, p as *const libc::c_void, sz);
    j = json_loads(jsonz, sz, &mut jerror);
    free(jsonz as *mut libc::c_void);
    if j.is_null() {
        return 0 as *mut cmd;
    }
    if (*j).type_0 as libc::c_uint != JSON_ARRAY as libc::c_int as libc::c_uint {
        json_decref(j);
        return 0 as *mut cmd;
    }
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong) < json_array_size(j) {
        let mut jelem: *mut json_t = json_array_get(j, i as size_t);
        match (*jelem).type_0 as libc::c_uint {
            2 | 3 => {
                argc += 1;
                argc;
            }
            _ => {}
        }
        i = i.wrapping_add(1);
        i;
    }
    if argc == 0 {
        json_decref(j);
        return 0 as *mut cmd;
    }
    cmd = cmd_new(c, argc);
    i = 0 as libc::c_int as libc::c_uint;
    cur = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong) < json_array_size(j) {
        let mut jelem_0: *mut json_t = json_array_get(j, i as size_t);
        let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
        match (*jelem_0).type_0 as libc::c_uint {
            2 => {
                tmp = strdup(json_string_value(jelem_0));
                let ref mut fresh0 = *((*cmd).argv).offset(cur as isize);
                *fresh0 = tmp;
                *((*cmd).argv_len).offset(cur as isize) = strlen(tmp);
                cur = cur.wrapping_add(1);
                cur;
            }
            3 => {
                tmp = malloc(40 as libc::c_int as libc::c_ulong) as *mut libc::c_char;
                sprintf(
                    tmp,
                    b"%d\0" as *const u8 as *const libc::c_char,
                    json_integer_value(jelem_0) as libc::c_int,
                );
                let ref mut fresh1 = *((*cmd).argv).offset(cur as isize);
                *fresh1 = tmp;
                *((*cmd).argv_len).offset(cur as isize) = strlen(tmp);
                cur = cur.wrapping_add(1);
                cur;
            }
            _ => {}
        }
        i = i.wrapping_add(1);
        i;
    }
    json_decref(j);
    return cmd;
}
pub unsafe extern "C" fn json_ws_error(
    mut http_status: libc::c_int,
    mut msg: *const libc::c_char,
    mut msg_sz: size_t,
    mut out_sz: *mut size_t,
) -> *mut libc::c_char {
    let mut jroot: *mut json_t = json_object();
    let mut jstr: *mut libc::c_char = 0 as *mut libc::c_char;
    json_object_set_new(
        jroot,
        b"error\0" as *const u8 as *const libc::c_char,
        json_true(),
    );
    json_object_set_new(
        jroot,
        b"message\0" as *const u8 as *const libc::c_char,
        json_string(msg),
    );
    json_object_set_new(
        jroot,
        b"http_status\0" as *const u8 as *const libc::c_char,
        json_integer(http_status as json_int_t),
    );
    jstr = json_string_output(jroot, 0 as *const libc::c_char);
    json_decref(jroot);
    *out_sz = strlen(jstr);
    return jstr;
}
