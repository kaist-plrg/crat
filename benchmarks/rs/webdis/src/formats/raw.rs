use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type dict;
    pub type sockadr;
    pub type worker;
    pub type evbuffer;
    pub type server;
    pub type event_base;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
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
    fn redisReaderCreate() -> *mut redisReader;
    fn freeReplyObject(reply: *mut libc::c_void);
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn format_send_error(cmd: *mut cmd, code: libc::c_short, msg: *const libc::c_char);
    fn integer_length(i: libc::c_longlong) -> libc::c_int;
    fn format_send_reply(
        cmd: *mut cmd,
        p: *const libc::c_char,
        sz: size_t,
        content_type: *const libc::c_char,
    );
    fn cmd_free(c: *mut cmd);
    fn cmd_new(c: *mut http_client, count: libc::c_int) -> *mut cmd;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
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
pub unsafe extern "C" fn raw_reply(
    mut c: *mut redisAsyncContext,
    mut r: *mut libc::c_void,
    mut privdata: *mut libc::c_void,
) {
    let mut reply: *mut redisReply = r as *mut redisReply;
    let mut cmd: *mut cmd = privdata as *mut cmd;
    let mut raw_out: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sz: size_t = 0;
    if reply.is_null() {
        format_send_error(
            cmd,
            503 as libc::c_int as libc::c_short,
            b"Service Unavailable\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    raw_out = raw_wrap(r as *const redisReply, &mut sz);
    format_send_reply(
        cmd,
        raw_out,
        sz,
        b"binary/octet-stream\0" as *const u8 as *const libc::c_char,
    );
    free(raw_out as *mut libc::c_void);
}
pub unsafe extern "C" fn raw_ws_extract(
    mut c: *mut http_client,
    mut p: *const libc::c_char,
    mut sz: size_t,
) -> *mut cmd {
    let mut cmd: *mut cmd = 0 as *mut cmd;
    let mut reader: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut reply: *mut redisReply = 0 as *mut redisReply;
    let mut reply_ptr: *mut *mut libc::c_void = &mut reply as *mut *mut redisReply
        as *mut *mut libc::c_void;
    let mut i: libc::c_uint = 0;
    reader = redisReaderCreate() as *mut libc::c_void;
    redisReaderFeed(reader as *mut redisReader, p as *mut libc::c_char, sz);
    if !(redisReaderGetReply(reader as *mut redisReader, reply_ptr)
        == -(1 as libc::c_int))
    {
        if !((*reply).type_0 != 2 as libc::c_int) {
            cmd = cmd_new(c, (*reply).elements as libc::c_int);
            i = 0 as libc::c_int as libc::c_uint;
            while (i as libc::c_ulong) < (*reply).elements {
                let mut ri: *mut redisReply = *((*reply).element).offset(i as isize);
                match (*ri).type_0 {
                    1 => {
                        *((*cmd).argv_len).offset(i as isize) = (*ri).len;
                        let ref mut fresh0 = *((*cmd).argv).offset(i as isize);
                        *fresh0 = calloc(
                            (*((*cmd).argv_len).offset(i as isize))
                                .wrapping_add(1 as libc::c_int as libc::c_ulong),
                            1 as libc::c_int as libc::c_ulong,
                        ) as *mut libc::c_char;
                        memcpy(
                            *((*cmd).argv).offset(i as isize) as *mut libc::c_void,
                            (*ri).str_0 as *const libc::c_void,
                            (*ri).len,
                        );
                    }
                    3 => {
                        *((*cmd).argv_len)
                            .offset(
                                i as isize,
                            ) = integer_length((*ri).integer) as size_t;
                        let ref mut fresh1 = *((*cmd).argv).offset(i as isize);
                        *fresh1 = calloc(
                            (*((*cmd).argv_len).offset(i as isize))
                                .wrapping_add(1 as libc::c_int as libc::c_ulong),
                            1 as libc::c_int as libc::c_ulong,
                        ) as *mut libc::c_char;
                        sprintf(
                            *((*cmd).argv).offset(i as isize),
                            b"%lld\0" as *const u8 as *const libc::c_char,
                            (*ri).integer,
                        );
                    }
                    _ => {
                        cmd_free(cmd);
                        cmd = 0 as *mut cmd;
                        break;
                    }
                }
                i = i.wrapping_add(1);
                i;
            }
        }
    }
    if !reader.is_null() {
        redisReaderFree(reader as *mut redisReader);
    }
    if !reply.is_null() {
        freeReplyObject(reply as *mut libc::c_void);
    }
    return cmd;
}
unsafe extern "C" fn raw_array(
    mut r: *const redisReply,
    mut sz: *mut size_t,
) -> *mut libc::c_char {
    let mut i: libc::c_uint = 0;
    let mut ret: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    *sz = 0 as libc::c_int as size_t;
    *sz = (*sz as libc::c_ulong)
        .wrapping_add(
            (1 as libc::c_int + integer_length((*r).elements as libc::c_longlong)
                + 2 as libc::c_int) as libc::c_ulong,
        ) as size_t as size_t;
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong) < (*r).elements {
        let mut e: *mut redisReply = *((*r).element).offset(i as isize);
        match (*e).type_0 {
            1 => {
                *sz = (*sz as libc::c_ulong)
                    .wrapping_add(
                        ((1 as libc::c_int + integer_length((*e).len as libc::c_longlong)
                            + 2 as libc::c_int) as libc::c_ulong)
                            .wrapping_add((*e).len)
                            .wrapping_add(2 as libc::c_int as libc::c_ulong),
                    ) as size_t as size_t;
            }
            3 => {
                *sz = (*sz as libc::c_ulong)
                    .wrapping_add(
                        (1 as libc::c_int
                            + integer_length(
                                integer_length((*e).integer) as libc::c_longlong,
                            ) + 2 as libc::c_int + integer_length((*e).integer)
                            + 2 as libc::c_int) as libc::c_ulong,
                    ) as size_t as size_t;
            }
            _ => {}
        }
        i = i.wrapping_add(1);
        i;
    }
    ret = malloc((1 as libc::c_int as libc::c_ulong).wrapping_add(*sz))
        as *mut libc::c_char;
    p = ret;
    p = p
        .offset(
            sprintf(p, b"*%zd\r\n\0" as *const u8 as *const libc::c_char, (*r).elements)
                as isize,
        );
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong) < (*r).elements {
        let mut e_0: *mut redisReply = *((*r).element).offset(i as isize);
        match (*e_0).type_0 {
            1 => {
                p = p
                    .offset(
                        sprintf(
                            p,
                            b"$%zu\r\n\0" as *const u8 as *const libc::c_char,
                            (*e_0).len,
                        ) as isize,
                    );
                memcpy(
                    p as *mut libc::c_void,
                    (*e_0).str_0 as *const libc::c_void,
                    (*e_0).len,
                );
                p = p.offset((*e_0).len as isize);
                *p = '\r' as i32 as libc::c_char;
                p = p.offset(1);
                p;
                *p = '\n' as i32 as libc::c_char;
                p = p.offset(1);
                p;
            }
            3 => {
                p = p
                    .offset(
                        sprintf(
                            p,
                            b"$%d\r\n%lld\r\n\0" as *const u8 as *const libc::c_char,
                            integer_length((*e_0).integer),
                            (*e_0).integer,
                        ) as isize,
                    );
            }
            _ => {}
        }
        i = i.wrapping_add(1);
        i;
    }
    return ret;
}
unsafe extern "C" fn raw_wrap(
    mut r: *const redisReply,
    mut sz: *mut size_t,
) -> *mut libc::c_char {
    let mut ret: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    match (*r).type_0 {
        5 | 6 => {
            *sz = (3 as libc::c_int as libc::c_ulong).wrapping_add((*r).len);
            ret = malloc(*sz) as *mut libc::c_char;
            *ret
                .offset(
                    0 as libc::c_int as isize,
                ) = (if (*r).type_0 == 5 as libc::c_int {
                '+' as i32
            } else {
                '-' as i32
            }) as libc::c_char;
            memcpy(
                ret.offset(1 as libc::c_int as isize) as *mut libc::c_void,
                (*r).str_0 as *const libc::c_void,
                (*sz).wrapping_sub(3 as libc::c_int as libc::c_ulong),
            );
            memcpy(
                ret.offset(*sz as isize).offset(-(2 as libc::c_int as isize))
                    as *mut libc::c_void,
                b"\r\n\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                2 as libc::c_int as libc::c_ulong,
            );
            return ret;
        }
        1 => {
            *sz = ((1 as libc::c_int + integer_length((*r).len as libc::c_longlong)
                + 2 as libc::c_int) as libc::c_ulong)
                .wrapping_add((*r).len)
                .wrapping_add(2 as libc::c_int as libc::c_ulong);
            ret = malloc(*sz) as *mut libc::c_char;
            p = ret;
            p = p
                .offset(
                    sprintf(
                        p,
                        b"$%zu\r\n\0" as *const u8 as *const libc::c_char,
                        (*r).len,
                    ) as isize,
                );
            memcpy(
                p as *mut libc::c_void,
                (*r).str_0 as *const libc::c_void,
                (*sz)
                    .wrapping_sub(2 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(p.offset_from(ret) as libc::c_long as libc::c_ulong),
            );
            memcpy(
                ret.offset(*sz as isize).offset(-(2 as libc::c_int as isize))
                    as *mut libc::c_void,
                b"\r\n\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                2 as libc::c_int as libc::c_ulong,
            );
            return ret;
        }
        3 => {
            *sz = (3 as libc::c_int + integer_length((*r).integer)) as size_t;
            ret = malloc((4 as libc::c_int as libc::c_ulong).wrapping_add(*sz))
                as *mut libc::c_char;
            sprintf(
                ret,
                b":%lld\r\n\0" as *const u8 as *const libc::c_char,
                (*r).integer,
            );
            return ret;
        }
        2 => return raw_array(r, sz),
        _ => {
            *sz = 5 as libc::c_int as size_t;
            ret = malloc(*sz) as *mut libc::c_char;
            memcpy(
                ret as *mut libc::c_void,
                b"$-1\r\n\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                5 as libc::c_int as libc::c_ulong,
            );
            return ret;
        }
    };
}
pub unsafe extern "C" fn raw_ws_error(
    mut http_status: libc::c_int,
    mut msg: *const libc::c_char,
    mut msg_sz: size_t,
    mut out_sz: *mut size_t,
) -> *mut libc::c_char {
    let mut ret: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    *out_sz = (5 as libc::c_int as libc::c_ulong)
        .wrapping_add(msg_sz)
        .wrapping_add(2 as libc::c_int as libc::c_ulong);
    ret = malloc(*out_sz) as *mut libc::c_char;
    p = ret;
    memcpy(
        p as *mut libc::c_void,
        b"-ERR \0" as *const u8 as *const libc::c_char as *const libc::c_void,
        5 as libc::c_int as libc::c_ulong,
    );
    p = p.offset(5 as libc::c_int as isize);
    memcpy(p as *mut libc::c_void, msg as *const libc::c_void, msg_sz);
    p = p.offset(msg_sz as isize);
    memcpy(
        p as *mut libc::c_void,
        b"\r\n\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        2 as libc::c_int as libc::c_ulong,
    );
    return ret;
}
