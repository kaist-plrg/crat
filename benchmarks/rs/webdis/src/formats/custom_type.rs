use ::libc;
extern "C" {
    pub type dict;
    pub type sockadr;
    pub type worker;
    pub type http_client;
    pub type event_base;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn free(__ptr: *mut libc::c_void);
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn cmd_is_subscribe(cmd: *mut cmd) -> libc::c_int;
    fn cmd_free(c: *mut cmd);
    fn format_send_reply(
        cmd: *mut cmd,
        p: *const libc::c_char,
        sz: size_t,
        content_type: *const libc::c_char,
    );
    fn format_send_error(cmd: *mut cmd, code: libc::c_short, msg: *const libc::c_char);
    fn http_response_init(
        w: *mut worker,
        code: libc::c_int,
        msg: *const libc::c_char,
    ) -> *mut http_response;
    fn http_response_set_header(
        r: *mut http_response,
        k: *const libc::c_char,
        v: *const libc::c_char,
        copy: header_copy,
    );
    fn http_response_write(r: *mut http_response, fd: libc::c_int);
    fn http_response_set_keep_alive(r: *mut http_response, enabled: libc::c_int);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
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
pub type uint8_t = __uint8_t;
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
pub unsafe extern "C" fn custom_type_reply(
    mut c: *mut redisAsyncContext,
    mut r: *mut libc::c_void,
    mut privdata: *mut libc::c_void,
) {
    let mut reply: *mut redisReply = r as *mut redisReply;
    let mut cmd: *mut cmd = privdata as *mut cmd;
    let mut int_buffer: [libc::c_char; 50] = [0; 50];
    let mut status_buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut int_len: libc::c_int = 0;
    let mut resp: *mut http_response = 0 as *mut http_response;
    let mut sz: size_t = 0;
    let mut array_out: *mut libc::c_char = 0 as *mut libc::c_char;
    if reply.is_null() {
        format_send_error(
            cmd,
            503 as libc::c_int as libc::c_short,
            b"Service Unavailable\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if !((*cmd).mime).is_null() {
        match (*reply).type_0 {
            4 => {
                format_send_error(
                    cmd,
                    404 as libc::c_int as libc::c_short,
                    b"Not found\0" as *const u8 as *const libc::c_char,
                );
                return;
            }
            1 => {
                format_send_reply(cmd, (*reply).str_0, (*reply).len, (*cmd).mime);
                return;
            }
            5 | 6 => {
                status_buf = calloc(
                    (1 as libc::c_int as libc::c_ulong).wrapping_add((*reply).len),
                    1 as libc::c_int as libc::c_ulong,
                ) as *mut libc::c_char;
                *status_buf
                    .offset(
                        0 as libc::c_int as isize,
                    ) = (if (*reply).type_0 == 5 as libc::c_int {
                    '+' as i32
                } else {
                    '-' as i32
                }) as libc::c_char;
                memcpy(
                    status_buf.offset(1 as libc::c_int as isize) as *mut libc::c_void,
                    (*reply).str_0 as *const libc::c_void,
                    (*reply).len,
                );
                format_send_reply(
                    cmd,
                    status_buf,
                    (1 as libc::c_int as libc::c_ulong).wrapping_add((*reply).len),
                    (*cmd).mime,
                );
                free(status_buf as *mut libc::c_void);
                return;
            }
            3 => {
                int_len = sprintf(
                    int_buffer.as_mut_ptr(),
                    b"%lld\0" as *const u8 as *const libc::c_char,
                    (*reply).integer,
                );
                format_send_reply(
                    cmd,
                    int_buffer.as_mut_ptr(),
                    int_len as size_t,
                    (*cmd).mime,
                );
                return;
            }
            2 => {
                array_out = custom_array(cmd, r as *const redisReply, &mut sz);
                format_send_reply(cmd, array_out, sz, (*cmd).mime);
                free(array_out as *mut libc::c_void);
                return;
            }
            _ => {}
        }
    }
    resp = http_response_init(
        (*cmd).w,
        400 as libc::c_int,
        b"Bad Request\0" as *const u8 as *const libc::c_char,
    );
    http_response_set_header(
        resp,
        b"Content-Length\0" as *const u8 as *const libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char,
        HEADER_COPY_NONE,
    );
    http_response_set_keep_alive(resp, (*cmd).keep_alive);
    http_response_write(resp, (*cmd).fd);
    if cmd_is_subscribe(cmd) == 0 {
        cmd_free(cmd);
    }
}
unsafe extern "C" fn custom_array(
    mut cmd: *mut cmd,
    mut r: *const redisReply,
    mut sz: *mut size_t,
) -> *mut libc::c_char {
    let mut i: libc::c_uint = 0;
    let mut ret: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sep_len: size_t = 0 as libc::c_int as size_t;
    if !((*cmd).separator).is_null() {
        sep_len = strlen((*cmd).separator);
    }
    *sz = 0 as libc::c_int as size_t;
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong) < (*r).elements {
        let mut e: *mut redisReply = *((*r).element).offset(i as isize);
        match (*e).type_0 {
            1 => {
                if sep_len != 0 && i != 0 as libc::c_int as libc::c_uint {
                    *sz = (*sz as libc::c_ulong).wrapping_add(sep_len) as size_t
                        as size_t;
                }
                *sz = (*sz as libc::c_ulong).wrapping_add((*e).len) as size_t as size_t;
            }
            _ => {}
        }
        i = i.wrapping_add(1);
        i;
    }
    ret = malloc(*sz) as *mut libc::c_char;
    p = ret;
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong) < (*r).elements {
        let mut e_0: *mut redisReply = *((*r).element).offset(i as isize);
        match (*e_0).type_0 {
            1 => {
                if sep_len != 0 && i != 0 as libc::c_int as libc::c_uint {
                    memcpy(
                        p as *mut libc::c_void,
                        (*cmd).separator as *const libc::c_void,
                        sep_len,
                    );
                    p = p.offset(sep_len as isize);
                }
                memcpy(
                    p as *mut libc::c_void,
                    (*e_0).str_0 as *const libc::c_void,
                    (*e_0).len,
                );
                p = p.offset((*e_0).len as isize);
            }
            _ => {}
        }
        i = i.wrapping_add(1);
        i;
    }
    return ret;
}
