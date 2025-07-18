use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type x509_store_ctx_st;
    pub type ossl_init_settings_st;
    pub type ssl_st;
    pub type ssl_ctx_st;
    pub type ssl_method_st;
    pub type lua_State;
    fn OPENSSL_init_crypto(
        opts: uint64_t,
        settings: *const OPENSSL_INIT_SETTINGS,
    ) -> libc::c_int;
    fn OPENSSL_init_ssl(
        opts: uint64_t,
        settings: *const OPENSSL_INIT_SETTINGS,
    ) -> libc::c_int;
    fn SSL_shutdown(s: *mut SSL) -> libc::c_int;
    fn TLS_client_method() -> *const SSL_METHOD;
    fn SSL_get_error(s: *const SSL, ret_code: libc::c_int) -> libc::c_int;
    fn SSL_CTX_ctrl(
        ctx: *mut SSL_CTX,
        cmd: libc::c_int,
        larg: libc::c_long,
        parg: *mut libc::c_void,
    ) -> libc::c_long;
    fn SSL_ctrl(
        ssl: *mut SSL,
        cmd: libc::c_int,
        larg: libc::c_long,
        parg: *mut libc::c_void,
    ) -> libc::c_long;
    fn SSL_write(
        ssl: *mut SSL,
        buf: *const libc::c_void,
        num: libc::c_int,
    ) -> libc::c_int;
    fn SSL_read(ssl: *mut SSL, buf: *mut libc::c_void, num: libc::c_int) -> libc::c_int;
    fn SSL_connect(ssl: *mut SSL) -> libc::c_int;
    fn SSL_CTX_new(meth: *const SSL_METHOD) -> *mut SSL_CTX;
    fn SSL_clear(s: *mut SSL) -> libc::c_int;
    fn SSL_pending(s: *const SSL) -> libc::c_int;
    fn SSL_set_fd(s: *mut SSL, fd: libc::c_int) -> libc::c_int;
    fn SSL_CTX_set_verify(ctx: *mut SSL_CTX, mode: libc::c_int, callback: SSL_verify_cb);
    fn SSL_CTX_set_verify_depth(ctx: *mut SSL_CTX, depth: libc::c_int);
}
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __time_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
pub type size_t = libc::c_ulong;
pub type time_t = __time_t;
pub type pthread_t = libc::c_ulong;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type X509_STORE_CTX = x509_store_ctx_st;
pub type OPENSSL_INIT_SETTINGS = ossl_init_settings_st;
pub type SSL = ssl_st;
pub type SSL_CTX = ssl_ctx_st;
pub type SSL_METHOD = ssl_method_st;
pub type SSL_verify_cb = Option::<
    unsafe extern "C" fn(libc::c_int, *mut X509_STORE_CTX) -> libc::c_int,
>;
pub type socklen_t = __socklen_t;
pub type sa_family_t = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [libc::c_char; 14],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct addrinfo {
    pub ai_flags: libc::c_int,
    pub ai_family: libc::c_int,
    pub ai_socktype: libc::c_int,
    pub ai_protocol: libc::c_int,
    pub ai_addrlen: socklen_t,
    pub ai_addr: *mut sockaddr,
    pub ai_canonname: *mut libc::c_char,
    pub ai_next: *mut addrinfo,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct errors {
    pub connect: uint32_t,
    pub read: uint32_t,
    pub write: uint32_t,
    pub status: uint32_t,
    pub timeout: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aeEventLoop {
    pub maxfd: libc::c_int,
    pub setsize: libc::c_int,
    pub timeEventNextId: libc::c_longlong,
    pub lastTime: time_t,
    pub events: *mut aeFileEvent,
    pub fired: *mut aeFiredEvent,
    pub timeEventHead: *mut aeTimeEvent,
    pub stop: libc::c_int,
    pub apidata: *mut libc::c_void,
    pub beforesleep: Option::<aeBeforeSleepProc>,
}
pub type aeBeforeSleepProc = unsafe extern "C" fn(*mut aeEventLoop) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aeTimeEvent {
    pub id: libc::c_longlong,
    pub when_sec: libc::c_long,
    pub when_ms: libc::c_long,
    pub timeProc: Option::<aeTimeProc>,
    pub finalizerProc: Option::<aeEventFinalizerProc>,
    pub clientData: *mut libc::c_void,
    pub next: *mut aeTimeEvent,
}
pub type aeEventFinalizerProc = unsafe extern "C" fn(
    *mut aeEventLoop,
    *mut libc::c_void,
) -> ();
pub type aeTimeProc = unsafe extern "C" fn(
    *mut aeEventLoop,
    libc::c_longlong,
    *mut libc::c_void,
) -> libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aeFiredEvent {
    pub fd: libc::c_int,
    pub mask: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aeFileEvent {
    pub mask: libc::c_int,
    pub rfileProc: Option::<aeFileProc>,
    pub wfileProc: Option::<aeFileProc>,
    pub clientData: *mut libc::c_void,
}
pub type aeFileProc = unsafe extern "C" fn(
    *mut aeEventLoop,
    libc::c_int,
    *mut libc::c_void,
    libc::c_int,
) -> ();
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct http_parser {
    #[bitfield(name = "type_0", ty = "libc::c_uint", bits = "0..=1")]
    #[bitfield(name = "flags", ty = "libc::c_uint", bits = "2..=9")]
    #[bitfield(name = "state", ty = "libc::c_uint", bits = "10..=16")]
    #[bitfield(name = "header_state", ty = "libc::c_uint", bits = "17..=23")]
    #[bitfield(name = "index", ty = "libc::c_uint", bits = "24..=30")]
    #[bitfield(name = "lenient_http_headers", ty = "libc::c_uint", bits = "31..=31")]
    pub type_0_flags_state_header_state_index_lenient_http_headers: [u8; 4],
    pub nread: uint32_t,
    pub content_length: uint64_t,
    pub http_major: libc::c_ushort,
    pub http_minor: libc::c_ushort,
    #[bitfield(name = "status_code", ty = "libc::c_uint", bits = "0..=15")]
    #[bitfield(name = "method", ty = "libc::c_uint", bits = "16..=23")]
    #[bitfield(name = "http_errno", ty = "libc::c_uint", bits = "24..=30")]
    #[bitfield(name = "upgrade", ty = "libc::c_uint", bits = "31..=31")]
    pub status_code_method_http_errno_upgrade: [u8; 4],
    pub data: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct thread {
    pub thread: pthread_t,
    pub loop_0: *mut aeEventLoop,
    pub addr: *mut addrinfo,
    pub connections: uint64_t,
    pub complete: uint64_t,
    pub requests: uint64_t,
    pub bytes: uint64_t,
    pub start: uint64_t,
    pub L: *mut lua_State,
    pub errors: errors,
    pub cs: *mut connection,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct connection {
    pub thread: *mut thread,
    pub parser: http_parser,
    pub state: C2RustUnnamed,
    pub fd: libc::c_int,
    pub ssl: *mut SSL,
    pub delayed: bool,
    pub start: uint64_t,
    pub request: *mut libc::c_char,
    pub length: size_t,
    pub written: size_t,
    pub pending: uint64_t,
    pub headers: buffer,
    pub body: buffer,
    pub buf: [libc::c_char; 8192],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct buffer {
    pub buffer: *mut libc::c_char,
    pub length: size_t,
    pub cursor: *mut libc::c_char,
}
pub type C2RustUnnamed = libc::c_uint;
pub const VALUE: C2RustUnnamed = 1;
pub const FIELD: C2RustUnnamed = 0;
pub type status = libc::c_uint;
pub const RETRY: status = 2;
pub const ERROR: status = 1;
pub const OK: status = 0;
pub unsafe extern "C" fn ssl_init() -> *mut SSL_CTX {
    let mut ctx: *mut SSL_CTX = 0 as *mut SSL_CTX;
    OPENSSL_init_ssl(
        (0x200000 as libc::c_long | 0x2 as libc::c_long) as uint64_t,
        0 as *const OPENSSL_INIT_SETTINGS,
    );
    OPENSSL_init_ssl(0 as libc::c_int as uint64_t, 0 as *const OPENSSL_INIT_SETTINGS);
    OPENSSL_init_crypto(
        (0x4 as libc::c_long | 0x8 as libc::c_long) as uint64_t,
        0 as *const OPENSSL_INIT_SETTINGS,
    );
    ctx = SSL_CTX_new(TLS_client_method());
    if !ctx.is_null() {
        SSL_CTX_set_verify(ctx, 0 as libc::c_int, None);
        SSL_CTX_set_verify_depth(ctx, 0 as libc::c_int);
        SSL_CTX_ctrl(
            ctx,
            33 as libc::c_int,
            0x4 as libc::c_uint as libc::c_long,
            0 as *mut libc::c_void,
        );
        SSL_CTX_ctrl(
            ctx,
            44 as libc::c_int,
            0x1 as libc::c_int as libc::c_long,
            0 as *mut libc::c_void,
        );
    }
    return ctx;
}
pub unsafe extern "C" fn ssl_connect(
    mut c: *mut connection,
    mut host: *mut libc::c_char,
) -> status {
    let mut r: libc::c_int = 0;
    SSL_set_fd((*c).ssl, (*c).fd);
    SSL_ctrl(
        (*c).ssl,
        55 as libc::c_int,
        0 as libc::c_int as libc::c_long,
        host as *mut libc::c_void,
    );
    r = SSL_connect((*c).ssl);
    if r != 1 as libc::c_int {
        match SSL_get_error((*c).ssl, r) {
            2 => return RETRY,
            3 => return RETRY,
            _ => return ERROR,
        }
    }
    return OK;
}
pub unsafe extern "C" fn ssl_close(mut c: *mut connection) -> status {
    SSL_shutdown((*c).ssl);
    SSL_clear((*c).ssl);
    return OK;
}
pub unsafe extern "C" fn ssl_read(mut c: *mut connection, mut n: *mut size_t) -> status {
    let mut r: libc::c_int = 0;
    r = SSL_read(
        (*c).ssl,
        ((*c).buf).as_mut_ptr() as *mut libc::c_void,
        ::std::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong as libc::c_int,
    );
    if r <= 0 as libc::c_int {
        match SSL_get_error((*c).ssl, r) {
            2 => return RETRY,
            3 => return RETRY,
            _ => return ERROR,
        }
    }
    *n = r as size_t;
    return OK;
}
pub unsafe extern "C" fn ssl_write(
    mut c: *mut connection,
    mut buf: *mut libc::c_char,
    mut len: size_t,
    mut n: *mut size_t,
) -> status {
    let mut r: libc::c_int = 0;
    r = SSL_write((*c).ssl, buf as *const libc::c_void, len as libc::c_int);
    if r <= 0 as libc::c_int {
        match SSL_get_error((*c).ssl, r) {
            2 => return RETRY,
            3 => return RETRY,
            _ => return ERROR,
        }
    }
    *n = r as size_t;
    return OK;
}
pub unsafe extern "C" fn ssl_readable(mut c: *mut connection) -> size_t {
    return SSL_pending((*c).ssl) as size_t;
}
