use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type worker;
    pub type dict;
    pub type sockadr;
    pub type evbuffer;
    pub type server;
    pub type event_base;
    fn client_get_header(
        c: *mut http_client,
        key: *const libc::c_char,
    ) -> *const libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncasecmp(
        __s1: *const libc::c_char,
        __s2: *const libc::c_char,
        __n: size_t,
    ) -> libc::c_int;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type size_t = libc::c_ulong;
pub type gid_t = __gid_t;
pub type uid_t = __uid_t;
pub type ssize_t = __ssize_t;
pub type int64_t = __int64_t;
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
#[inline]
unsafe extern "C" fn __bswap_32(mut __bsx: __uint32_t) -> __uint32_t {
    return (__bsx & 0xff000000 as libc::c_uint) >> 24 as libc::c_int
        | (__bsx & 0xff0000 as libc::c_uint) >> 8 as libc::c_int
        | (__bsx & 0xff00 as libc::c_uint) << 8 as libc::c_int
        | (__bsx & 0xff as libc::c_uint) << 24 as libc::c_int;
}
pub unsafe extern "C" fn acl_match_client(
    mut a: *mut acl,
    mut client: *mut http_client,
    mut ip: *mut in_addr_t,
) -> libc::c_int {
    let mut auth: *const libc::c_char = 0 as *const libc::c_char;
    auth = client_get_header(
        client,
        b"Authorization\0" as *const u8 as *const libc::c_char,
    );
    if !((*a).http_basic_auth).is_null() {
        if !auth.is_null()
            && strncasecmp(
                auth,
                b"Basic \0" as *const u8 as *const libc::c_char,
                6 as libc::c_int as size_t,
            ) == 0 as libc::c_int
        {
            if strcmp(auth.offset(6 as libc::c_int as isize), (*a).http_basic_auth)
                != 0 as libc::c_int
            {
                return 0 as libc::c_int;
            }
        } else {
            return 0 as libc::c_int
        }
    }
    if (*a).cidr.enabled == 0 as libc::c_int {
        return 1 as libc::c_int;
    }
    if *ip & (*a).cidr.mask == (*a).cidr.subnet & (*a).cidr.mask {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn acl_allow_command(
    mut cmd: *mut cmd,
    mut cfg: *mut conf,
    mut client: *mut http_client,
) -> libc::c_int {
    let mut always_off: [*mut libc::c_char; 5] = [
        b"MULTI\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"EXEC\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"WATCH\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"DISCARD\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"SELECT\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ];
    let mut i: libc::c_uint = 0;
    let mut authorized: libc::c_int = 1 as libc::c_int;
    let mut a: *mut acl = 0 as *mut acl;
    let mut client_addr: in_addr_t = 0;
    let mut cmd_name: *const libc::c_char = 0 as *const libc::c_char;
    let mut cmd_len: size_t = 0;
    if (*cmd).count == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    cmd_name = *((*cmd).argv).offset(0 as libc::c_int as isize);
    cmd_len = *((*cmd).argv_len).offset(0 as libc::c_int as isize);
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong)
        < (::std::mem::size_of::<[*mut libc::c_char; 5]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
    {
        if strncasecmp(always_off[i as usize], cmd_name, cmd_len) == 0 as libc::c_int {
            return 0 as libc::c_int;
        }
        i = i.wrapping_add(1);
        i;
    }
    client_addr = __bswap_32((*client).addr);
    a = (*cfg).perms;
    while !a.is_null() {
        if !(acl_match_client(a, client, &mut client_addr) == 0) {
            i = 0 as libc::c_int as libc::c_uint;
            while i < (*a).enabled.count {
                if strncasecmp(
                    *((*a).enabled.commands).offset(i as isize),
                    cmd_name,
                    cmd_len,
                ) == 0 as libc::c_int
                {
                    authorized = 1 as libc::c_int;
                }
                if strncasecmp(
                    *((*a).enabled.commands).offset(i as isize),
                    b"*\0" as *const u8 as *const libc::c_char,
                    1 as libc::c_int as size_t,
                ) == 0 as libc::c_int
                {
                    authorized = 1 as libc::c_int;
                }
                i = i.wrapping_add(1);
                i;
            }
            i = 0 as libc::c_int as libc::c_uint;
            while i < (*a).disabled.count {
                if strncasecmp(
                    *((*a).disabled.commands).offset(i as isize),
                    cmd_name,
                    cmd_len,
                ) == 0 as libc::c_int
                {
                    authorized = 0 as libc::c_int;
                }
                if strncasecmp(
                    *((*a).disabled.commands).offset(i as isize),
                    b"*\0" as *const u8 as *const libc::c_char,
                    1 as libc::c_int as size_t,
                ) == 0 as libc::c_int
                {
                    authorized = 0 as libc::c_int;
                }
                i = i.wrapping_add(1);
                i;
            }
        }
        a = (*a).next;
    }
    return authorized;
}
