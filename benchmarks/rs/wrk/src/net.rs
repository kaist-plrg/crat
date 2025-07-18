use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type ssl_st;
    pub type lua_State;
    fn __errno_location() -> *mut libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, _: ...) -> libc::c_int;
}
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __time_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;
pub type socklen_t = __socklen_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type time_t = __time_t;
pub type pthread_t = libc::c_ulong;
pub type SSL = ssl_st;
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
pub unsafe extern "C" fn sock_connect(
    mut c: *mut connection,
    mut host: *mut libc::c_char,
) -> status {
    return OK;
}
pub unsafe extern "C" fn sock_close(mut c: *mut connection) -> status {
    return OK;
}
pub unsafe extern "C" fn sock_read(
    mut c: *mut connection,
    mut n: *mut size_t,
) -> status {
    let mut r: ssize_t = read(
        (*c).fd,
        ((*c).buf).as_mut_ptr() as *mut libc::c_void,
        ::std::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong,
    );
    *n = r as size_t;
    return (if r >= 0 as libc::c_int as libc::c_long {
        OK as libc::c_int
    } else {
        ERROR as libc::c_int
    }) as status;
}
pub unsafe extern "C" fn sock_write(
    mut c: *mut connection,
    mut buf: *mut libc::c_char,
    mut len: size_t,
    mut n: *mut size_t,
) -> status {
    let mut r: ssize_t = 0;
    r = write((*c).fd, buf as *const libc::c_void, len);
    if r == -(1 as libc::c_int) as libc::c_long {
        match *__errno_location() {
            11 => return RETRY,
            _ => return ERROR,
        }
    }
    *n = r as size_t;
    return OK;
}
pub unsafe extern "C" fn sock_readable(mut c: *mut connection) -> size_t {
    let mut n: libc::c_int = 0;
    let mut rc: libc::c_int = 0;
    rc = ioctl(
        (*c).fd,
        0x541b as libc::c_int as libc::c_ulong,
        &mut n as *mut libc::c_int,
    );
    return (if rc == -(1 as libc::c_int) { 0 as libc::c_int } else { n }) as size_t;
}
