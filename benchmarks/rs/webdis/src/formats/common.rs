use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type worker;
    pub type dict;
    pub type sockadr;
    pub type evbuffer;
    pub type server;
    pub type event_base;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn llabs(_: libc::c_longlong) -> libc::c_longlong;
    fn cmd_free(c: *mut cmd);
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn cmd_is_subscribe(cmd: *mut cmd) -> libc::c_int;
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
    fn http_response_set_body(
        r: *mut http_response,
        body: *const libc::c_char,
        body_len: size_t,
    );
    fn http_response_write(r: *mut http_response, fd: libc::c_int);
    fn http_response_write_chunk(
        fd: libc::c_int,
        w: *mut worker,
        p: *const libc::c_char,
        sz: size_t,
    );
    fn http_response_set_keep_alive(r: *mut http_response, enabled: libc::c_int);
    fn ws_frame_and_send_response(
        ws: *mut ws_client,
        type_0: ws_frame_type,
        p: *const libc::c_char,
        sz: size_t,
    ) -> libc::c_int;
    fn md5_init(pms: *mut md5_state_t);
    fn md5_append(pms: *mut md5_state_t, data: *const md5_byte_t, nbytes: libc::c_int);
    fn md5_finish(pms: *mut md5_state_t, digest: *mut md5_byte_t);
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
}
pub type size_t = libc::c_ulong;
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
pub type __suseconds_t = libc::c_long;
pub type __time_t = libc::c_long;
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
pub type __ssize_t = libc::c_long;
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
pub type int64_t = __int64_t;
pub type __int64_t = libc::c_long;
pub type uint32_t = __uint32_t;
pub type __uint32_t = libc::c_uint;
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
pub type uint8_t = __uint8_t;
pub type __uint8_t = libc::c_uchar;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_11 {
    pub tqe_next: *mut event_callback,
    pub tqe_prev: *mut *mut event_callback,
}
pub type in_addr_t = uint32_t;
pub type ws_frame_type = libc::c_int;
pub const WS_UNKNOWN_FRAME: ws_frame_type = -1;
pub const WS_PONG: ws_frame_type = 10;
pub const WS_PING: ws_frame_type = 9;
pub const WS_CONNECTION_CLOSE: ws_frame_type = 8;
pub const WS_BINARY_FRAME: ws_frame_type = 1;
pub const WS_TEXT_FRAME: ws_frame_type = 0;
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
pub type md5_byte_t = libc::c_uchar;
pub type md5_state_t = md5_state_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct md5_state_s {
    pub count: [md5_word_t; 2],
    pub abcd: [md5_word_t; 4],
    pub buf: [md5_byte_t; 64],
}
pub type md5_word_t = libc::c_uint;
pub unsafe extern "C" fn etag_new(
    mut p: *const libc::c_char,
    mut sz: size_t,
) -> *mut libc::c_char {
    let mut buf: [md5_byte_t; 16] = [0; 16];
    let mut etag: *mut libc::c_char = calloc(
        (34 as libc::c_int + 1 as libc::c_int) as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
    ) as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    if etag.is_null() {
        return 0 as *mut libc::c_char;
    }
    let mut pms: md5_state_t = md5_state_t {
        count: [0; 2],
        abcd: [0; 4],
        buf: [0; 64],
    };
    md5_init(&mut pms);
    md5_append(&mut pms, p as *const md5_byte_t, sz as libc::c_int);
    md5_finish(&mut pms, buf.as_mut_ptr());
    i = 0 as libc::c_int;
    while i < 16 as libc::c_int {
        sprintf(
            etag
                .offset(1 as libc::c_int as isize)
                .offset((2 as libc::c_int * i) as isize),
            b"%.2x\0" as *const u8 as *const libc::c_char,
            buf[i as usize] as libc::c_int,
        );
        i += 1;
        i;
    }
    *etag.offset(0 as libc::c_int as isize) = '"' as i32 as libc::c_char;
    *etag.offset(33 as libc::c_int as isize) = '"' as i32 as libc::c_char;
    return etag;
}
pub unsafe extern "C" fn format_send_error(
    mut cmd: *mut cmd,
    mut code: libc::c_short,
    mut msg: *const libc::c_char,
) {
    let mut resp: *mut http_response = 0 as *mut http_response;
    if (*cmd).is_websocket == 0 && ((*cmd).pub_sub_client).is_null() {
        resp = http_response_init((*cmd).w, code as libc::c_int, msg);
        (*resp).http_version = (*cmd).http_version;
        http_response_set_keep_alive(resp, (*cmd).keep_alive);
        http_response_write(resp, (*cmd).fd);
    } else if (*cmd).is_websocket != 0
        && (*(*(*cmd).http_client).ws).close_after_events == 0
    {
        ws_frame_and_send_response(
            (*(*cmd).http_client).ws,
            WS_BINARY_FRAME,
            msg,
            strlen(msg),
        );
    }
    if (*cmd).is_websocket == 0 {
        if !((*cmd).pub_sub_client).is_null() {
            (*(*cmd).pub_sub_client).reused_cmd = 0 as *mut cmd;
        } else {
            cmd_free(cmd);
        }
    }
}
pub unsafe extern "C" fn format_send_reply(
    mut cmd: *mut cmd,
    mut p: *const libc::c_char,
    mut sz: size_t,
    mut content_type: *const libc::c_char,
) {
    let mut free_cmd: libc::c_int = 1 as libc::c_int;
    let mut ct: *const libc::c_char = if !((*cmd).mime).is_null() {
        (*cmd).mime as *const libc::c_char
    } else {
        content_type
    };
    let mut resp: *mut http_response = 0 as *mut http_response;
    if (*cmd).is_websocket != 0 {
        ws_frame_and_send_response((*(*cmd).http_client).ws, WS_BINARY_FRAME, p, sz);
        if cmd_is_subscribe(cmd) == 0 {
            cmd_free(cmd);
        }
        return;
    }
    if cmd_is_subscribe(cmd) != 0 {
        free_cmd = 0 as libc::c_int;
        if (*cmd).started_responding == 0 as libc::c_int {
            (*cmd).started_responding = 1 as libc::c_int;
            resp = http_response_init(
                (*cmd).w,
                200 as libc::c_int,
                b"OK\0" as *const u8 as *const libc::c_char,
            );
            (*resp).http_version = (*cmd).http_version;
            if !((*cmd).filename).is_null() {
                http_response_set_header(
                    resp,
                    b"Content-Disposition\0" as *const u8 as *const libc::c_char,
                    (*cmd).filename,
                    HEADER_COPY_VALUE,
                );
            }
            http_response_set_header(
                resp,
                b"Content-Type\0" as *const u8 as *const libc::c_char,
                ct,
                HEADER_COPY_VALUE,
            );
            http_response_set_keep_alive(resp, 1 as libc::c_int);
            http_response_set_header(
                resp,
                b"Transfer-Encoding\0" as *const u8 as *const libc::c_char,
                b"chunked\0" as *const u8 as *const libc::c_char,
                HEADER_COPY_NONE,
            );
            http_response_set_body(resp, p, sz);
            http_response_write(resp, (*cmd).fd);
        } else {
            http_response_write_chunk((*cmd).fd, (*cmd).w, p, sz);
        }
    } else {
        let mut etag: *mut libc::c_char = etag_new(p, sz);
        if !etag.is_null() {
            if !((*cmd).if_none_match).is_null()
                && strcmp((*cmd).if_none_match, etag) == 0 as libc::c_int
            {
                resp = http_response_init(
                    (*cmd).w,
                    304 as libc::c_int,
                    b"Not Modified\0" as *const u8 as *const libc::c_char,
                );
            } else {
                resp = http_response_init(
                    (*cmd).w,
                    200 as libc::c_int,
                    b"OK\0" as *const u8 as *const libc::c_char,
                );
                if !((*cmd).filename).is_null() {
                    http_response_set_header(
                        resp,
                        b"Content-Disposition\0" as *const u8 as *const libc::c_char,
                        (*cmd).filename,
                        HEADER_COPY_VALUE,
                    );
                }
                http_response_set_header(
                    resp,
                    b"Content-Type\0" as *const u8 as *const libc::c_char,
                    ct,
                    HEADER_COPY_VALUE,
                );
                http_response_set_header(
                    resp,
                    b"ETag\0" as *const u8 as *const libc::c_char,
                    etag,
                    HEADER_COPY_VALUE,
                );
                http_response_set_body(resp, p, sz);
            }
            (*resp).http_version = (*cmd).http_version;
            http_response_set_keep_alive(resp, (*cmd).keep_alive);
            http_response_write(resp, (*cmd).fd);
            free(etag as *mut libc::c_void);
        } else {
            format_send_error(
                cmd,
                503 as libc::c_int as libc::c_short,
                b"Service Unavailable\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    if free_cmd != 0 {
        cmd_free(cmd);
    }
}
pub unsafe extern "C" fn integer_length(mut i: libc::c_longlong) -> libc::c_int {
    let mut sz: libc::c_int = 0 as libc::c_int;
    let mut ci: libc::c_int = llabs(i) as libc::c_int;
    while ci > 0 as libc::c_int {
        ci = ci / 10 as libc::c_int;
        sz += 1 as libc::c_int;
    }
    if i == 0 as libc::c_int as libc::c_longlong {
        sz = 1 as libc::c_int;
    } else if i < 0 as libc::c_int as libc::c_longlong {
        sz += 1;
        sz;
    }
    return sz;
}
