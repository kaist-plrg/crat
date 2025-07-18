use ::libc;
use ::c2rust_asm_casts;
use c2rust_asm_casts::AsmCastTrait;
use std::arch::asm;
extern "C" {
    pub type AUTH_T;
    pub type DIGEST_CRED;
    pub type DIGEST_CHLG;
    pub type ARRAY_T;
    pub type COOKIES_T;
    pub type sockaddr_x25;
    pub type sockaddr_un;
    pub type sockaddr_ns;
    pub type sockaddr_iso;
    pub type sockaddr_ipx;
    pub type sockaddr_inarp;
    pub type sockaddr_eon;
    pub type sockaddr_dl;
    pub type sockaddr_ax25;
    pub type sockaddr_at;
    pub type x509_st;
    pub type ssl_st;
    pub type ssl_ctx_st;
    pub type ssl_method_st;
    pub type PAGE_T;
    pub type CACHE_T;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn puts(__s: *const libc::c_char) -> libc::c_int;
    fn select(
        __nfds: libc::c_int,
        __readfds: *mut fd_set,
        __writefds: *mut fd_set,
        __exceptfds: *mut fd_set,
        __timeout: *mut timeval,
    ) -> libc::c_int;
    fn pthread_self() -> pthread_t;
    fn pthread_setcanceltype(
        __type: libc::c_int,
        __oldtype: *mut libc::c_int,
    ) -> libc::c_int;
    fn pthread_testcancel();
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn __errno_location() -> *mut libc::c_int;
    fn auth_get_proxy_required(this: AUTH) -> BOOLEAN;
    fn auth_get_proxy_host(this: AUTH) -> *mut libc::c_char;
    fn auth_get_proxy_port(this: AUTH) -> libc::c_int;
    static mut my: CONFIG;
    fn poll(__fds: *mut pollfd, __nfds: nfds_t, __timeout: libc::c_int) -> libc::c_int;
    fn gai_strerror(__ecode: libc::c_int) -> *const libc::c_char;
    fn getaddrinfo(
        __name: *const libc::c_char,
        __service: *const libc::c_char,
        __req: *const addrinfo,
        __pai: *mut *mut addrinfo,
    ) -> libc::c_int;
    fn shutdown(__fd: libc::c_int, __how: libc::c_int) -> libc::c_int;
    fn setsockopt(
        __fd: libc::c_int,
        __level: libc::c_int,
        __optname: libc::c_int,
        __optval: *const libc::c_void,
        __optlen: socklen_t,
    ) -> libc::c_int;
    fn connect(
        __fd: libc::c_int,
        __addr: __CONST_SOCKADDR_ARG,
        __len: socklen_t,
    ) -> libc::c_int;
    fn socket(
        __domain: libc::c_int,
        __type: libc::c_int,
        __protocol: libc::c_int,
    ) -> libc::c_int;
    fn SSL_get_shutdown(ssl: *const SSL) -> libc::c_int;
    fn SSL_shutdown(s: *mut SSL) -> libc::c_int;
    fn SSL_get_error(s: *const SSL, ret_code: libc::c_int) -> libc::c_int;
    fn SSL_write(
        ssl: *mut SSL,
        buf: *const libc::c_void,
        num: libc::c_int,
    ) -> libc::c_int;
    fn SSL_read(ssl: *mut SSL, buf: *mut libc::c_void, num: libc::c_int) -> libc::c_int;
    fn SSL_free(ssl: *mut SSL);
    fn SSL_CTX_free(_: *mut SSL_CTX);
    fn startswith(pre: *const libc::c_char, str: *const libc::c_char) -> BOOLEAN;
    fn NOTIFY(L: LEVEL, fmt: *const libc::c_char, _: ...);
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
pub type ssize_t = __ssize_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type __fd_mask = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fd_set {
    pub fds_bits: [__fd_mask; 16],
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_cond_s {
    pub c2rust_unnamed: C2RustUnnamed_1,
    pub c2rust_unnamed_0: C2RustUnnamed,
    pub __g_refs: [libc::c_uint; 2],
    pub __g_size: [libc::c_uint; 2],
    pub __g1_orig_size: libc::c_uint,
    pub __wrefs: libc::c_uint,
    pub __g_signals: [libc::c_uint; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub __g1_start: libc::c_ulonglong,
    pub __g1_start32: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub __low: libc::c_uint,
    pub __high: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub __wseq: libc::c_ulonglong,
    pub __wseq32: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub __low: libc::c_uint,
    pub __high: libc::c_uint,
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
pub union pthread_cond_t {
    pub __data: __pthread_cond_s,
    pub __size: [libc::c_char; 48],
    pub __align: libc::c_longlong,
}
pub type C2RustUnnamed_3 = libc::c_uint;
pub const PTHREAD_CANCEL_ASYNCHRONOUS: C2RustUnnamed_3 = 1;
pub const PTHREAD_CANCEL_DEFERRED: C2RustUnnamed_3 = 0;
pub type socklen_t = __socklen_t;
pub type BOOLEAN = libc::c_uint;
pub const boolean_true: BOOLEAN = 1;
pub const boolean_false: BOOLEAN = 0;
pub type METHOD = libc::c_uint;
pub const PATCH: METHOD = 9;
pub const CONNECT: METHOD = 8;
pub const OPTIONS: METHOD = 7;
pub const TRACE: METHOD = 6;
pub const DELETE: METHOD = 5;
pub const PUT: METHOD = 4;
pub const POST: METHOD = 3;
pub const GET: METHOD = 2;
pub const HEAD: METHOD = 1;
pub const NOMETHOD: METHOD = 0;
pub type SCHEME = libc::c_uint;
pub const PROXY: SCHEME = 4;
pub const FTP: SCHEME = 3;
pub const HTTPS: SCHEME = 2;
pub const HTTP: SCHEME = 1;
pub const UNSUPPORTED: SCHEME = 0;
pub type AUTH = *mut AUTH_T;
pub type DCRED = DIGEST_CRED;
pub type DCHLG = DIGEST_CHLG;
pub type TYPE = libc::c_uint;
pub const NTLM: TYPE = 2;
pub const DIGEST: TYPE = 1;
pub const BASIC: TYPE = 0;
pub type ARRAY = *mut ARRAY_T;
pub type COOKIES = *mut COOKIES_T;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LINES {
    pub index: libc::c_int,
    pub line: *mut *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CONFIG {
    pub logging: BOOLEAN,
    pub shlog: BOOLEAN,
    pub limit: libc::c_int,
    pub url: *mut libc::c_char,
    pub logfile: [libc::c_char; 128],
    pub verbose: BOOLEAN,
    pub quiet: BOOLEAN,
    pub parser: BOOLEAN,
    pub csv: BOOLEAN,
    pub fullurl: BOOLEAN,
    pub display: BOOLEAN,
    pub config: BOOLEAN,
    pub color: BOOLEAN,
    pub cusers: libc::c_int,
    pub delay: libc::c_float,
    pub timeout: libc::c_int,
    pub bench: BOOLEAN,
    pub internet: BOOLEAN,
    pub timestamp: BOOLEAN,
    pub time: libc::c_int,
    pub secs: libc::c_int,
    pub reps: libc::c_int,
    pub file: [libc::c_char; 128],
    pub length: libc::c_int,
    pub nomap: *mut LINES,
    pub debug: BOOLEAN,
    pub chunked: BOOLEAN,
    pub unique: BOOLEAN,
    pub get: BOOLEAN,
    pub print: BOOLEAN,
    pub mark: BOOLEAN,
    pub markstr: *mut libc::c_char,
    pub protocol: libc::c_int,
    pub cookies: COOKIES,
    pub uagent: [libc::c_char; 256],
    pub encoding: [libc::c_char; 256],
    pub conttype: [libc::c_char; 256],
    pub bids: libc::c_int,
    pub auth: AUTH,
    pub keepalive: BOOLEAN,
    pub signaled: libc::c_int,
    pub extra: [libc::c_char; 2048],
    pub login: BOOLEAN,
    pub loginurl: *mut libc::c_char,
    pub lurl: ARRAY,
    pub failures: libc::c_int,
    pub failed: libc::c_int,
    pub escape: BOOLEAN,
    pub expire: BOOLEAN,
    pub follow: BOOLEAN,
    pub zero_ok: BOOLEAN,
    pub spinner: BOOLEAN,
    pub cache: BOOLEAN,
    pub rc: [libc::c_char; 256],
    pub ssl_timeout: libc::c_int,
    pub ssl_cert: *mut libc::c_char,
    pub ssl_key: *mut libc::c_char,
    pub ssl_ciphers: *mut libc::c_char,
    pub method: METHOD,
    pub json_output: BOOLEAN,
    pub cond: pthread_cond_t,
    pub lock: pthread_mutex_t,
}
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type __socket_type = libc::c_uint;
pub const SOCK_NONBLOCK: __socket_type = 2048;
pub const SOCK_CLOEXEC: __socket_type = 524288;
pub const SOCK_PACKET: __socket_type = 10;
pub const SOCK_DCCP: __socket_type = 6;
pub const SOCK_SEQPACKET: __socket_type = 5;
pub const SOCK_RDM: __socket_type = 4;
pub const SOCK_RAW: __socket_type = 3;
pub const SOCK_DGRAM: __socket_type = 2;
pub const SOCK_STREAM: __socket_type = 1;
pub type sa_family_t = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [libc::c_char; 14],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in6 {
    pub sin6_family: sa_family_t,
    pub sin6_port: in_port_t,
    pub sin6_flowinfo: uint32_t,
    pub sin6_addr: in6_addr,
    pub sin6_scope_id: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in6_addr {
    pub __in6_u: C2RustUnnamed_4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
    pub __u6_addr8: [uint8_t; 16],
    pub __u6_addr16: [uint16_t; 8],
    pub __u6_addr32: [uint32_t; 4],
}
pub type in_port_t = uint16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in {
    pub sin_family: sa_family_t,
    pub sin_port: in_port_t,
    pub sin_addr: in_addr,
    pub sin_zero: [libc::c_uchar; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in_addr {
    pub s_addr: in_addr_t,
}
pub type in_addr_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union __CONST_SOCKADDR_ARG {
    pub __sockaddr__: *const sockaddr,
    pub __sockaddr_at__: *const sockaddr_at,
    pub __sockaddr_ax25__: *const sockaddr_ax25,
    pub __sockaddr_dl__: *const sockaddr_dl,
    pub __sockaddr_eon__: *const sockaddr_eon,
    pub __sockaddr_in__: *const sockaddr_in,
    pub __sockaddr_in6__: *const sockaddr_in6,
    pub __sockaddr_inarp__: *const sockaddr_inarp,
    pub __sockaddr_ipx__: *const sockaddr_ipx,
    pub __sockaddr_iso__: *const sockaddr_iso,
    pub __sockaddr_ns__: *const sockaddr_ns,
    pub __sockaddr_un__: *const sockaddr_un,
    pub __sockaddr_x25__: *const sockaddr_x25,
}
pub type C2RustUnnamed_5 = libc::c_uint;
pub const IPPROTO_MAX: C2RustUnnamed_5 = 256;
pub const IPPROTO_RAW: C2RustUnnamed_5 = 255;
pub const IPPROTO_MPLS: C2RustUnnamed_5 = 137;
pub const IPPROTO_UDPLITE: C2RustUnnamed_5 = 136;
pub const IPPROTO_SCTP: C2RustUnnamed_5 = 132;
pub const IPPROTO_COMP: C2RustUnnamed_5 = 108;
pub const IPPROTO_PIM: C2RustUnnamed_5 = 103;
pub const IPPROTO_ENCAP: C2RustUnnamed_5 = 98;
pub const IPPROTO_BEETPH: C2RustUnnamed_5 = 94;
pub const IPPROTO_MTP: C2RustUnnamed_5 = 92;
pub const IPPROTO_AH: C2RustUnnamed_5 = 51;
pub const IPPROTO_ESP: C2RustUnnamed_5 = 50;
pub const IPPROTO_GRE: C2RustUnnamed_5 = 47;
pub const IPPROTO_RSVP: C2RustUnnamed_5 = 46;
pub const IPPROTO_IPV6: C2RustUnnamed_5 = 41;
pub const IPPROTO_DCCP: C2RustUnnamed_5 = 33;
pub const IPPROTO_TP: C2RustUnnamed_5 = 29;
pub const IPPROTO_IDP: C2RustUnnamed_5 = 22;
pub const IPPROTO_UDP: C2RustUnnamed_5 = 17;
pub const IPPROTO_PUP: C2RustUnnamed_5 = 12;
pub const IPPROTO_EGP: C2RustUnnamed_5 = 8;
pub const IPPROTO_TCP: C2RustUnnamed_5 = 6;
pub const IPPROTO_IPIP: C2RustUnnamed_5 = 4;
pub const IPPROTO_IGMP: C2RustUnnamed_5 = 2;
pub const IPPROTO_ICMP: C2RustUnnamed_5 = 1;
pub const IPPROTO_IP: C2RustUnnamed_5 = 0;
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
pub type nfds_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pollfd {
    pub fd: libc::c_int,
    pub events: libc::c_short,
    pub revents: libc::c_short,
}
pub type X509 = x509_st;
pub type SSL = ssl_st;
pub type SSL_CTX = ssl_ctx_st;
pub type SSL_METHOD = ssl_method_st;
pub type PAGE = *mut PAGE_T;
pub type CACHE = *mut CACHE_T;
pub type S_STATUS = libc::c_uint;
pub const S_DONE: S_STATUS = 8;
pub const S_WRITING: S_STATUS = 4;
pub const S_READING: S_STATUS = 2;
pub const S_CONNECTING: S_STATUS = 1;
pub type SDSET = libc::c_uint;
pub const RDWR: SDSET = 3;
pub const WRITE: SDSET = 2;
pub const READ: SDSET = 1;
pub const UNDEF: SDSET = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CONN {
    pub sock: libc::c_int,
    pub status: S_STATUS,
    pub encrypt: BOOLEAN,
    pub scheme: SCHEME,
    pub page: PAGE,
    pub cache: CACHE,
    pub content: C2RustUnnamed_10,
    pub connection: C2RustUnnamed_9,
    pub auth: C2RustUnnamed_7,
    pub ssl: *mut SSL,
    pub ctx: *mut SSL_CTX,
    pub method: *const SSL_METHOD,
    pub cert: *mut X509,
    pub inbuffer: size_t,
    pub pos_ini: libc::c_int,
    pub buffer: [libc::c_char; 4096],
    pub chkbuf: [libc::c_char; 1024],
    pub pfd: [pollfd; 1],
    pub ws: *mut fd_set,
    pub rs: *mut fd_set,
    pub state: SDSET,
    pub ftp: C2RustUnnamed_6,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub code: libc::c_int,
    pub host: [libc::c_char; 64],
    pub port: libc::c_int,
    pub size: size_t,
    pub pasv: BOOLEAN,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub wchlg: *mut DCHLG,
    pub wcred: *mut DCRED,
    pub www: libc::c_int,
    pub pchlg: *mut DCHLG,
    pub pcred: *mut DCRED,
    pub proxy: libc::c_int,
    pub type_0: C2RustUnnamed_8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub www: TYPE,
    pub proxy: TYPE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub max: libc::c_int,
    pub timeout: libc::c_int,
    pub reuse: libc::c_int,
    pub status: libc::c_int,
    pub keepalive: libc::c_int,
    pub tested: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub transfer: libc::c_int,
    pub length: size_t,
}
pub type LEVEL = libc::c_uint;
pub const FATAL: LEVEL = 3;
pub const ERROR: LEVEL = 2;
pub const WARNING: LEVEL = 1;
pub const DEBUG: LEVEL = 0;
pub unsafe extern "C" fn new_socket(
    mut C: *mut CONN,
    mut hostparam: *const libc::c_char,
    mut portparam: libc::c_int,
) -> libc::c_int {
    let mut conn: libc::c_int = 0;
    let mut res: libc::c_int = 0;
    let mut addrlen: libc::c_int = 0;
    let mut s_addr: *mut sockaddr = 0 as *mut sockaddr;
    let mut hn: [libc::c_char; 512] = [0; 512];
    let mut port: libc::c_int = 0;
    let mut domain: libc::c_int = 0;
    let mut port_str: [libc::c_char; 10] = [0; 10];
    let mut hints: addrinfo = addrinfo {
        ai_flags: 0,
        ai_family: 0,
        ai_socktype: 0,
        ai_protocol: 0,
        ai_addrlen: 0,
        ai_addr: 0 as *mut sockaddr,
        ai_canonname: 0 as *mut libc::c_char,
        ai_next: 0 as *mut addrinfo,
    };
    let mut addr_res: *mut addrinfo = 0 as *mut addrinfo;
    let mut r: *mut addrinfo = 0 as *mut addrinfo;
    if hostparam.is_null() {
        NOTIFY(
            ERROR,
            b"Unable to resolve host %s:%d\0" as *const u8 as *const libc::c_char,
            b"sock.c\0" as *const u8 as *const libc::c_char,
            118 as libc::c_int,
        );
        return -(1 as libc::c_int);
    }
    (*C)
        .encrypt = (if (*C).scheme as libc::c_uint
        == HTTPS as libc::c_int as libc::c_uint
    {
        boolean_true as libc::c_int
    } else {
        boolean_false as libc::c_int
    }) as BOOLEAN;
    (*C).state = UNDEF;
    (*C).ftp.pasv = boolean_true;
    (*C).ftp.size = 0 as libc::c_int as size_t;
    memset(
        hn.as_mut_ptr() as *mut libc::c_void,
        '\0' as i32,
        ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong,
    );
    if auth_get_proxy_required(my.auth) as u64 != 0 {
        snprintf(
            hn.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong,
            b"%s\0" as *const u8 as *const libc::c_char,
            auth_get_proxy_host(my.auth),
        );
        port = auth_get_proxy_port(my.auth);
    } else {
        snprintf(
            hn.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong,
            b"%s\0" as *const u8 as *const libc::c_char,
            hostparam,
        );
        port = portparam;
    }
    __hostname_strip(hn.as_mut_ptr(), 512 as libc::c_int);
    if port < 1 as libc::c_int || port > 65535 as libc::c_int {
        NOTIFY(
            ERROR,
            b"invalid port number %d in %s:%d\0" as *const u8 as *const libc::c_char,
            port,
            b"sock.c\0" as *const u8 as *const libc::c_char,
            142 as libc::c_int,
        );
        return -(1 as libc::c_int);
    }
    snprintf(
        port_str.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong,
        b"%d\0" as *const u8 as *const libc::c_char,
        port,
    );
    memset(
        &mut hints as *mut addrinfo as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<addrinfo>() as libc::c_ulong,
    );
    hints.ai_family = 0 as libc::c_int;
    hints.ai_socktype = SOCK_STREAM as libc::c_int;
    hints.ai_protocol = IPPROTO_TCP as libc::c_int;
    res = getaddrinfo(hn.as_mut_ptr(), port_str.as_mut_ptr(), &mut hints, &mut addr_res);
    if res != 0 as libc::c_int {
        NOTIFY(
            ERROR,
            b"Address resolution failed at %s:%d with the following error:\0"
                as *const u8 as *const libc::c_char,
            b"sock.c\0" as *const u8 as *const libc::c_char,
            158 as libc::c_int,
        );
        NOTIFY(ERROR, b"%s\0" as *const u8 as *const libc::c_char, gai_strerror(res));
        return -(1 as libc::c_int);
    }
    s_addr = (*addr_res).ai_addr;
    addrlen = (*addr_res).ai_addrlen as libc::c_int;
    domain = (*addr_res).ai_family;
    if __socket_create(C, domain) < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    conn = connect(
        (*C).sock,
        __CONST_SOCKADDR_ARG {
            __sockaddr__: s_addr,
        },
        addrlen as socklen_t,
    );
    pthread_testcancel();
    if conn < 0 as libc::c_int && *__errno_location() != 115 as libc::c_int {
        addr_res = (*addr_res).ai_next;
        r = addr_res;
        while !r.is_null() {
            socket_close(C);
            if __socket_create(C, domain) < 0 as libc::c_int {
                return -(1 as libc::c_int);
            }
            conn = connect(
                (*C).sock,
                __CONST_SOCKADDR_ARG {
                    __sockaddr__: s_addr,
                },
                addrlen as socklen_t,
            );
            pthread_testcancel();
            if conn == 0 as libc::c_int {
                break;
            }
            r = (*r).ai_next;
        }
    }
    if conn < 0 as libc::c_int && *__errno_location() != 115 as libc::c_int {
        match *__errno_location() {
            13 => {
                NOTIFY(
                    ERROR,
                    b"socket: %d EACCES\0" as *const u8 as *const libc::c_char,
                    pthread_self(),
                );
            }
            99 => {
                NOTIFY(
                    ERROR,
                    b"socket: %d address is unavailable.\0" as *const u8
                        as *const libc::c_char,
                    pthread_self(),
                );
            }
            110 => {
                NOTIFY(
                    ERROR,
                    b"socket: %d connection timed out.\0" as *const u8
                        as *const libc::c_char,
                    pthread_self(),
                );
            }
            111 => {
                NOTIFY(
                    ERROR,
                    b"socket: %d connection refused.\0" as *const u8
                        as *const libc::c_char,
                    pthread_self(),
                );
            }
            101 => {
                NOTIFY(
                    ERROR,
                    b"socket: %d network is unreachable.\0" as *const u8
                        as *const libc::c_char,
                    pthread_self(),
                );
            }
            106 => {
                NOTIFY(
                    ERROR,
                    b"socket: %d already connected.\0" as *const u8
                        as *const libc::c_char,
                    pthread_self(),
                );
            }
            _ => {
                NOTIFY(
                    ERROR,
                    b"socket: %d unknown network error.\0" as *const u8
                        as *const libc::c_char,
                    pthread_self(),
                );
            }
        }
        socket_close(C);
        return -(1 as libc::c_int);
    } else if __socket_check(C, READ) as libc::c_uint
        == boolean_false as libc::c_int as libc::c_uint
    {
        pthread_testcancel();
        NOTIFY(
            WARNING,
            b"socket: read check timed out(%d) %s:%d\0" as *const u8
                as *const libc::c_char,
            my.timeout,
            b"sock.c\0" as *const u8 as *const libc::c_char,
            273 as libc::c_int,
        );
        socket_close(C);
        return -(1 as libc::c_int);
    } else {
        res = connect(
            (*C).sock,
            __CONST_SOCKADDR_ARG {
                __sockaddr__: s_addr,
            },
            addrlen as socklen_t,
        );
        if res < 0 as libc::c_int && *__errno_location() != 106 as libc::c_int {
            NOTIFY(
                ERROR,
                b"socket: unable to connect %s:%d\0" as *const u8 as *const libc::c_char,
                b"sock.c\0" as *const u8 as *const libc::c_char,
                282 as libc::c_int,
            );
            socket_close(C);
            return -(1 as libc::c_int);
        }
        (*C).status = S_READING;
    }
    if __socket_block((*C).sock, boolean_true) < 0 as libc::c_int {
        NOTIFY(
            ERROR,
            b"socket: unable to set socket to non-blocking %s:%d\0" as *const u8
                as *const libc::c_char,
            b"sock.c\0" as *const u8 as *const libc::c_char,
            291 as libc::c_int,
        );
        return -(1 as libc::c_int);
    }
    (*C).connection.status = 1 as libc::c_int;
    return (*C).sock;
}
unsafe extern "C" fn __socket_check(mut C: *mut CONN, mut mode: SDSET) -> BOOLEAN {
    if (*C).sock >= 1024 as libc::c_int {
        return __socket_poll(C, mode)
    } else {
        return __socket_select(C, mode)
    };
}
unsafe extern "C" fn __socket_poll(mut C: *mut CONN, mut mode: SDSET) -> BOOLEAN {
    let mut res: libc::c_int = 0;
    let mut timo: libc::c_int = if my.timeout != 0 {
        my.timeout * 1000 as libc::c_int
    } else {
        15000 as libc::c_int
    };
    __socket_block((*C).sock, boolean_false);
    (*C).pfd[0 as libc::c_int as usize].fd = (*C).sock + 1 as libc::c_int;
    (*C)
        .pfd[0 as libc::c_int as usize]
        .events = ((*C).pfd[0 as libc::c_int as usize].events as libc::c_int
        | 0x1 as libc::c_int) as libc::c_short;
    loop {
        res = poll(((*C).pfd).as_mut_ptr(), 1 as libc::c_int as nfds_t, timo);
        pthread_testcancel();
        if res < 0 as libc::c_int {
            puts(b"LESS THAN ZERO!\0" as *const u8 as *const libc::c_char);
        }
        if !(res < 0 as libc::c_int) {
            break;
        }
    }
    if res == 0 as libc::c_int {
        *__errno_location() = 110 as libc::c_int;
    }
    if res <= 0 as libc::c_int {
        (*C).state = UNDEF;
        NOTIFY(
            WARNING,
            b"socket: polled(%d) and discovered it's not ready %s:%d\0" as *const u8
                as *const libc::c_char,
            if my.timeout != 0 { my.timeout } else { 15 as libc::c_int },
            b"sock.c\0" as *const u8 as *const libc::c_char,
            343 as libc::c_int,
        );
        return boolean_false;
    } else {
        (*C).state = mode;
        return boolean_true;
    };
}
unsafe extern "C" fn __socket_select(mut C: *mut CONN, mut mode: SDSET) -> BOOLEAN {
    let mut timeout: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut res: libc::c_int = 0;
    let mut rs: fd_set = fd_set { fds_bits: [0; 16] };
    let mut ws: fd_set = fd_set { fds_bits: [0; 16] };
    memset(
        &mut timeout as *mut timeval as *mut libc::c_void,
        '\0' as i32,
        ::std::mem::size_of::<timeval>() as libc::c_ulong,
    );
    timeout
        .tv_sec = (if my.timeout > 0 as libc::c_int {
        my.timeout
    } else {
        30 as libc::c_int
    }) as __time_t;
    timeout.tv_usec = 0 as libc::c_int as __suseconds_t;
    if (*C).sock < 0 as libc::c_int || (*C).sock >= 1024 as libc::c_int {
        return boolean_false;
    }
    loop {
        let mut __d0: libc::c_int = 0;
        let mut __d1: libc::c_int = 0;
        let fresh0 = &mut __d0;
        let fresh1;
        let fresh2 = (::std::mem::size_of::<fd_set>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<__fd_mask>() as libc::c_ulong);
        let fresh3 = &mut __d1;
        let fresh4;
        let fresh5 = &mut *(rs.fds_bits).as_mut_ptr().offset(0 as libc::c_int as isize)
            as *mut __fd_mask;
        asm!(
            "cld; rep; stosq", inlateout("cx") c2rust_asm_casts::AsmCast::cast_in(fresh0,
            fresh2) => fresh1, inlateout("di") c2rust_asm_casts::AsmCast::cast_in(fresh3,
            fresh5) => fresh4, inlateout("ax") 0 as libc::c_int => _,
            options(preserves_flags, att_syntax)
        );
        c2rust_asm_casts::AsmCast::cast_out(fresh0, fresh2, fresh1);
        c2rust_asm_casts::AsmCast::cast_out(fresh3, fresh5, fresh4);
        let mut __d0_0: libc::c_int = 0;
        let mut __d1_0: libc::c_int = 0;
        let fresh6 = &mut __d0_0;
        let fresh7;
        let fresh8 = (::std::mem::size_of::<fd_set>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<__fd_mask>() as libc::c_ulong);
        let fresh9 = &mut __d1_0;
        let fresh10;
        let fresh11 = &mut *(ws.fds_bits).as_mut_ptr().offset(0 as libc::c_int as isize)
            as *mut __fd_mask;
        asm!(
            "cld; rep; stosq", inlateout("cx") c2rust_asm_casts::AsmCast::cast_in(fresh6,
            fresh8) => fresh7, inlateout("di") c2rust_asm_casts::AsmCast::cast_in(fresh9,
            fresh11) => fresh10, inlateout("ax") 0 as libc::c_int => _,
            options(preserves_flags, att_syntax)
        );
        c2rust_asm_casts::AsmCast::cast_out(fresh6, fresh8, fresh7);
        c2rust_asm_casts::AsmCast::cast_out(fresh9, fresh11, fresh10);
        rs
            .fds_bits[((*C).sock
            / (8 as libc::c_int
                * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
            as usize]
            |= ((1 as libc::c_ulong)
                << (*C).sock
                    % (8 as libc::c_int
                        * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                            as libc::c_int)) as __fd_mask;
        ws
            .fds_bits[((*C).sock
            / (8 as libc::c_int
                * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
            as usize]
            |= ((1 as libc::c_ulong)
                << (*C).sock
                    % (8 as libc::c_int
                        * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                            as libc::c_int)) as __fd_mask;
        res = select(
            (*C).sock + 1 as libc::c_int,
            &mut rs,
            &mut ws,
            0 as *mut fd_set,
            &mut timeout,
        );
        pthread_testcancel();
        if !(res < 0 as libc::c_int && *__errno_location() == 4 as libc::c_int) {
            break;
        }
    }
    if res == 0 as libc::c_int {
        *__errno_location() = 110 as libc::c_int;
    }
    if res <= 0 as libc::c_int {
        (*C).state = UNDEF;
        NOTIFY(
            WARNING,
            b"socket: select and discovered it's not ready %s:%d\0" as *const u8
                as *const libc::c_char,
            b"sock.c\0" as *const u8 as *const libc::c_char,
            384 as libc::c_int,
        );
        return boolean_false;
    } else {
        (*C).state = mode;
        return boolean_true;
    };
}
unsafe extern "C" fn __socket_create(
    mut C: *mut CONN,
    mut domain: libc::c_int,
) -> libc::c_int {
    (*C).sock = socket(domain, SOCK_STREAM as libc::c_int, 0 as libc::c_int);
    if (*C).sock < 0 as libc::c_int {
        match *__errno_location() {
            93 => {
                NOTIFY(
                    ERROR,
                    b"unsupported protocol %s:%d\0" as *const u8 as *const libc::c_char,
                    b"sock.c\0" as *const u8 as *const libc::c_char,
                    401 as libc::c_int,
                );
            }
            24 => {
                NOTIFY(
                    ERROR,
                    b"descriptor table full %s:%d\0" as *const u8 as *const libc::c_char,
                    b"sock.c\0" as *const u8 as *const libc::c_char,
                    402 as libc::c_int,
                );
            }
            23 => {
                NOTIFY(
                    ERROR,
                    b"file table full %s:%d\0" as *const u8 as *const libc::c_char,
                    b"sock.c\0" as *const u8 as *const libc::c_char,
                    403 as libc::c_int,
                );
            }
            13 => {
                NOTIFY(
                    ERROR,
                    b"permission denied %s:%d\0" as *const u8 as *const libc::c_char,
                    b"sock.c\0" as *const u8 as *const libc::c_char,
                    404 as libc::c_int,
                );
            }
            105 => {
                NOTIFY(
                    ERROR,
                    b"insufficient buffer %s:%d\0" as *const u8 as *const libc::c_char,
                    b"sock.c\0" as *const u8 as *const libc::c_char,
                    405 as libc::c_int,
                );
            }
            _ => {
                NOTIFY(
                    ERROR,
                    b"unknown socket error %s:%d\0" as *const u8 as *const libc::c_char,
                    b"sock.c\0" as *const u8 as *const libc::c_char,
                    406 as libc::c_int,
                );
            }
        }
        socket_close(C);
        return -(1 as libc::c_int);
    }
    if fcntl((*C).sock, 2 as libc::c_int, 0o4000 as libc::c_int) < 0 as libc::c_int {
        NOTIFY(
            ERROR,
            b"unable to set close control %s:%d\0" as *const u8 as *const libc::c_char,
            b"sock.c\0" as *const u8 as *const libc::c_char,
            410 as libc::c_int,
        );
    }
    if (*C).connection.keepalive != 0 {
        let mut opt: libc::c_int = 1 as libc::c_int;
        if setsockopt(
            (*C).sock,
            1 as libc::c_int,
            9 as libc::c_int,
            &mut opt as *mut libc::c_int as *mut libc::c_char as *const libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t,
        ) < 0 as libc::c_int
        {
            match *__errno_location() {
                9 => {
                    NOTIFY(
                        ERROR,
                        b"invalid descriptor %s:%d\0" as *const u8
                            as *const libc::c_char,
                        b"sock.c\0" as *const u8 as *const libc::c_char,
                        417 as libc::c_int,
                    );
                }
                88 => {
                    NOTIFY(
                        ERROR,
                        b"not a socket %s:%d\0" as *const u8 as *const libc::c_char,
                        b"sock.c\0" as *const u8 as *const libc::c_char,
                        418 as libc::c_int,
                    );
                }
                92 => {
                    NOTIFY(
                        ERROR,
                        b"not a protocol option %s:%d\0" as *const u8
                            as *const libc::c_char,
                        b"sock.c\0" as *const u8 as *const libc::c_char,
                        419 as libc::c_int,
                    );
                }
                14 => {
                    NOTIFY(
                        ERROR,
                        b"setsockopt unknown %s:%d\0" as *const u8
                            as *const libc::c_char,
                        b"sock.c\0" as *const u8 as *const libc::c_char,
                        420 as libc::c_int,
                    );
                }
                _ => {
                    NOTIFY(
                        ERROR,
                        b"unknown sockopt error %s:%d\0" as *const u8
                            as *const libc::c_char,
                        b"sock.c\0" as *const u8 as *const libc::c_char,
                        421 as libc::c_int,
                    );
                }
            }
            socket_close(C);
            return -(1 as libc::c_int);
        }
    }
    if __socket_block((*C).sock, boolean_false) < 0 as libc::c_int {
        NOTIFY(
            ERROR,
            b"socket: unable to set socket to non-blocking %s:%d\0" as *const u8
                as *const libc::c_char,
            b"sock.c\0" as *const u8 as *const libc::c_char,
            427 as libc::c_int,
        );
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn __hostname_strip(mut hn: *mut libc::c_char, mut len: libc::c_int) {
    let mut i: libc::c_int = 0;
    if startswith(b"[\0" as *const u8 as *const libc::c_char, hn) as u64 != 0 {
        memmove(
            hn as *mut libc::c_void,
            hn.offset(1 as libc::c_int as isize) as *const libc::c_void,
            (len - 1 as libc::c_int) as libc::c_ulong,
        );
        i = 0 as libc::c_int;
        while *hn.offset(i as isize) as libc::c_int != 0
            && *hn.offset(i as isize) as libc::c_int != ']' as i32
        {
            i += 1;
            i;
        }
        if *hn.offset(i as isize) as libc::c_int == ']' as i32 {
            memmove(
                hn.offset(i as isize) as *mut libc::c_void,
                hn.offset(i as isize).offset(1 as libc::c_int as isize)
                    as *const libc::c_void,
                (len - i - 1 as libc::c_int) as libc::c_ulong,
            );
        }
    }
}
unsafe extern "C" fn __socket_block(
    mut sock: libc::c_int,
    mut block: BOOLEAN,
) -> libc::c_int {
    let mut flags: libc::c_int = 0;
    let mut retval: libc::c_int = 0;
    if sock == -(1 as libc::c_int) {
        return sock;
    }
    flags = fcntl(sock, 3 as libc::c_int, 0 as libc::c_int);
    if flags < 0 as libc::c_int {
        match *__errno_location() {
            13 => {
                NOTIFY(
                    ERROR,
                    b"EACCES %s:%d\0" as *const u8 as *const libc::c_char,
                    b"sock.c\0" as *const u8 as *const libc::c_char,
                    478 as libc::c_int,
                );
            }
            9 => {
                NOTIFY(
                    ERROR,
                    b"bad file descriptor %s:%d\0" as *const u8 as *const libc::c_char,
                    b"sock.c\0" as *const u8 as *const libc::c_char,
                    479 as libc::c_int,
                );
            }
            11 => {
                NOTIFY(
                    ERROR,
                    b"address is unavailable %s:%d\0" as *const u8
                        as *const libc::c_char,
                    b"sock.c\0" as *const u8 as *const libc::c_char,
                    480 as libc::c_int,
                );
            }
            _ => {
                NOTIFY(
                    ERROR,
                    b"unknown network error %s:%d\0" as *const u8 as *const libc::c_char,
                    b"sock.c\0" as *const u8 as *const libc::c_char,
                    481 as libc::c_int,
                );
            }
        }
        return -(1 as libc::c_int);
    }
    if block as u64 != 0 {
        flags &= !(0o4000 as libc::c_int);
    } else {
        flags |= 0o4000 as libc::c_int;
        flags |= 0o4000 as libc::c_int;
    }
    retval = fcntl(sock, 4 as libc::c_int, flags);
    if retval < 0 as libc::c_int {
        NOTIFY(
            ERROR,
            b"unable to set fcntl flags %s:%d\0" as *const u8 as *const libc::c_char,
            b"sock.c\0" as *const u8 as *const libc::c_char,
            496 as libc::c_int,
        );
        return -(1 as libc::c_int);
    }
    return retval;
}
unsafe extern "C" fn __socket_write(
    mut sock: libc::c_int,
    mut vbuf: *const libc::c_void,
    mut len: size_t,
) -> ssize_t {
    let mut n: size_t = 0;
    let mut w: ssize_t = 0;
    let mut buf: *const libc::c_char = 0 as *const libc::c_char;
    buf = vbuf as *const libc::c_char;
    n = len;
    while n > 0 as libc::c_int as libc::c_ulong {
        w = write(sock, buf as *const libc::c_void, n);
        if w <= 0 as libc::c_int as libc::c_long {
            if *__errno_location() == 4 as libc::c_int {
                w = 0 as libc::c_int as ssize_t;
            } else {
                return -(1 as libc::c_int) as ssize_t
            }
        }
        n = (n as libc::c_ulong).wrapping_sub(w as libc::c_ulong) as size_t as size_t;
        buf = buf.offset(w as isize);
    }
    return len as ssize_t;
}
unsafe extern "C" fn __ssl_socket_write(
    mut C: *mut CONN,
    mut vbuf: *const libc::c_void,
    mut len: size_t,
) -> ssize_t {
    let mut n: size_t = 0;
    let mut w: ssize_t = 0;
    let mut buf: *const libc::c_char = 0 as *const libc::c_char;
    let mut err: libc::c_int = 0;
    buf = vbuf as *const libc::c_char;
    n = len;
    while n > 0 as libc::c_int as libc::c_ulong {
        w = SSL_write((*C).ssl, buf as *const libc::c_void, n as libc::c_int) as ssize_t;
        if w <= 0 as libc::c_int as libc::c_long {
            if w < 0 as libc::c_int as libc::c_long {
                err = SSL_get_error((*C).ssl, w as libc::c_int);
                match err {
                    2 | 3 => {
                        NOTIFY(
                            DEBUG,
                            b"SSL_write non-critical error %d\0" as *const u8
                                as *const libc::c_char,
                            err,
                        );
                        return 0 as libc::c_int as ssize_t;
                    }
                    5 => {
                        NOTIFY(
                            ERROR,
                            b"SSL_write() failed (syscall)\0" as *const u8
                                as *const libc::c_char,
                        );
                        return -(1 as libc::c_int) as ssize_t;
                    }
                    1 => return -(1 as libc::c_int) as ssize_t,
                    _ => {}
                }
            }
            NOTIFY(ERROR, b"SSL_write() failed.\0" as *const u8 as *const libc::c_char);
            return -(1 as libc::c_int) as ssize_t;
        }
        n = (n as libc::c_ulong).wrapping_sub(w as libc::c_ulong) as size_t as size_t;
        buf = buf.offset(w as isize);
    }
    return len as ssize_t;
}
pub unsafe extern "C" fn socket_read(
    mut C: *mut CONN,
    mut vbuf: *mut libc::c_void,
    mut len: size_t,
) -> ssize_t {
    let mut type_0: libc::c_int = 0;
    let mut n: size_t = 0;
    let mut r: ssize_t = 0;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ret_eof: libc::c_int = 0 as libc::c_int;
    pthread_setcanceltype(PTHREAD_CANCEL_DEFERRED as libc::c_int, &mut type_0);
    buf = vbuf as *mut libc::c_char;
    n = len;
    if (*C).encrypt as libc::c_uint == boolean_true as libc::c_int as libc::c_uint {
        while n > 0 as libc::c_int as libc::c_ulong {
            if __socket_check(C, READ) as libc::c_uint
                == boolean_false as libc::c_int as libc::c_uint
            {
                NOTIFY(
                    WARNING,
                    b"socket: read check timed out(%d) %s:%d\0" as *const u8
                        as *const libc::c_char,
                    if my.timeout != 0 { my.timeout } else { 15 as libc::c_int },
                    b"sock.c\0" as *const u8 as *const libc::c_char,
                    595 as libc::c_int,
                );
                return -(1 as libc::c_int) as ssize_t;
            }
            r = SSL_read((*C).ssl, buf as *mut libc::c_void, n as libc::c_int)
                as ssize_t;
            if r < 0 as libc::c_int as libc::c_long {
                if *__errno_location() == 4 as libc::c_int
                    || SSL_get_error((*C).ssl, r as libc::c_int) == 2 as libc::c_int
                {
                    r = 0 as libc::c_int as ssize_t;
                } else {
                    return -(1 as libc::c_int) as ssize_t
                }
            } else if r == 0 as libc::c_int as libc::c_long {
                break;
            }
            n = (n as libc::c_ulong).wrapping_sub(r as libc::c_ulong) as size_t
                as size_t;
            buf = buf.offset(r as isize);
        }
    } else {
        while n > 0 as libc::c_int as libc::c_ulong {
            if (*C).inbuffer < len {
                if __socket_check(C, READ) as libc::c_uint
                    == boolean_false as libc::c_int as libc::c_uint
                {
                    NOTIFY(
                        WARNING,
                        b"socket: read check timed out(%d) %s:%d\0" as *const u8
                            as *const libc::c_char,
                        if my.timeout != 0 { my.timeout } else { 15 as libc::c_int },
                        b"sock.c\0" as *const u8 as *const libc::c_char,
                        614 as libc::c_int,
                    );
                    return -(1 as libc::c_int) as ssize_t;
                }
            }
            if (*C).inbuffer < n {
                let mut lidos: libc::c_int = 0;
                memmove(
                    ((*C).buffer).as_mut_ptr() as *mut libc::c_void,
                    &mut *((*C).buffer).as_mut_ptr().offset((*C).pos_ini as isize)
                        as *mut libc::c_char as *const libc::c_void,
                    (*C).inbuffer,
                );
                (*C).pos_ini = 0 as libc::c_int;
                if __socket_check(C, READ) as libc::c_uint
                    == boolean_false as libc::c_int as libc::c_uint
                {
                    NOTIFY(
                        WARNING,
                        b"socket: read check timed out(%d) %s:%d\0" as *const u8
                            as *const libc::c_char,
                        if my.timeout != 0 { my.timeout } else { 15 as libc::c_int },
                        b"sock.c\0" as *const u8 as *const libc::c_char,
                        623 as libc::c_int,
                    );
                    return -(1 as libc::c_int) as ssize_t;
                }
                lidos = read(
                    (*C).sock,
                    &mut *((*C).buffer).as_mut_ptr().offset((*C).inbuffer as isize)
                        as *mut libc::c_char as *mut libc::c_void,
                    (::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong)
                        .wrapping_sub((*C).inbuffer),
                ) as libc::c_int;
                if lidos == 0 as libc::c_int {
                    ret_eof = 1 as libc::c_int;
                }
                if lidos < 0 as libc::c_int {
                    if *__errno_location() == 4 as libc::c_int
                        || *__errno_location() == 11 as libc::c_int
                    {
                        lidos = 0 as libc::c_int;
                    }
                    if *__errno_location() == 32 as libc::c_int {
                        return 0 as libc::c_int as ssize_t
                    } else {
                        NOTIFY(
                            ERROR,
                            b"socket: read error %s %s:%d\0" as *const u8
                                as *const libc::c_char,
                            strerror(*__errno_location()),
                            b"sock.c\0" as *const u8 as *const libc::c_char,
                            635 as libc::c_int,
                        );
                        return 0 as libc::c_int as ssize_t;
                    }
                }
                (*C)
                    .inbuffer = ((*C).inbuffer as libc::c_ulong)
                    .wrapping_add(lidos as libc::c_ulong) as size_t as size_t;
            }
            if (*C).inbuffer >= n {
                r = n as ssize_t;
            } else {
                r = (*C).inbuffer as ssize_t;
            }
            if r == 0 as libc::c_int as libc::c_long {
                break;
            }
            memmove(
                buf as *mut libc::c_void,
                &mut *((*C).buffer).as_mut_ptr().offset((*C).pos_ini as isize)
                    as *mut libc::c_char as *const libc::c_void,
                r as libc::c_ulong,
            );
            (*C).pos_ini = ((*C).pos_ini as libc::c_long + r) as libc::c_int;
            (*C)
                .inbuffer = ((*C).inbuffer as libc::c_ulong)
                .wrapping_sub(r as libc::c_ulong) as size_t as size_t;
            n = (n as libc::c_ulong).wrapping_sub(r as libc::c_ulong) as size_t
                as size_t;
            buf = buf.offset(r as isize);
            if ret_eof != 0 {
                break;
            }
        }
    }
    pthread_setcanceltype(type_0, 0 as *mut libc::c_int);
    pthread_testcancel();
    return len.wrapping_sub(n) as ssize_t;
}
pub unsafe extern "C" fn socket_readline(
    mut C: *mut CONN,
    mut ptr: *mut libc::c_char,
    mut maxlen: size_t,
) -> ssize_t {
    let mut type_0: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut res: libc::c_int = 0;
    let mut c: libc::c_char = 0;
    len = maxlen as libc::c_int;
    pthread_setcanceltype(PTHREAD_CANCEL_DEFERRED as libc::c_int, &mut type_0);
    n = 1 as libc::c_int;
    while n < len {
        res = socket_read(
            C,
            &mut c as *mut libc::c_char as *mut libc::c_void,
            1 as libc::c_int as size_t,
        ) as libc::c_int;
        if res == 1 as libc::c_int {
            let fresh12 = ptr;
            ptr = ptr.offset(1);
            *fresh12 = c;
            if c as libc::c_int == '\n' as i32 {
                break;
            }
            n += 1;
            n;
        } else if res == 0 as libc::c_int {
            if !(n == 1 as libc::c_int) {
                break;
            }
            return 0 as libc::c_int as ssize_t;
        } else {
            return -(1 as libc::c_int) as ssize_t
        }
    }
    *ptr = 0 as libc::c_int as libc::c_char;
    pthread_setcanceltype(type_0, 0 as *mut libc::c_int);
    pthread_testcancel();
    return n as ssize_t;
}
pub unsafe extern "C" fn socket_write(
    mut C: *mut CONN,
    mut buf: *const libc::c_void,
    mut len: size_t,
) -> libc::c_int {
    let mut type_0: libc::c_int = 0;
    let mut bytes: size_t = 0;
    pthread_setcanceltype(PTHREAD_CANCEL_DEFERRED as libc::c_int, &mut type_0);
    if (*C).encrypt as libc::c_uint == boolean_true as libc::c_int as libc::c_uint {
        loop {
            bytes = __ssl_socket_write(C, buf, len) as size_t;
            if bytes != len {
                if bytes == 0 as libc::c_int as libc::c_ulong {} else {
                    return -(1 as libc::c_int)
                }
            }
            if !(bytes == 0 as libc::c_int as libc::c_ulong) {
                break;
            }
        }
    } else {
        bytes = __socket_write((*C).sock, buf, len) as size_t;
        if bytes != len {
            NOTIFY(
                ERROR,
                b"unable to write to socket %s:%d\0" as *const u8 as *const libc::c_char,
                b"sock.c\0" as *const u8 as *const libc::c_char,
                733 as libc::c_int,
            );
            return -(1 as libc::c_int);
        }
    }
    pthread_setcanceltype(type_0, 0 as *mut libc::c_int);
    pthread_testcancel();
    return bytes as libc::c_int;
}
pub unsafe extern "C" fn socket_close(mut C: *mut CONN) {
    let mut type_0: libc::c_int = 0;
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut tries: libc::c_int = 0 as libc::c_int;
    if C.is_null() {
        return;
    }
    pthread_setcanceltype(PTHREAD_CANCEL_DEFERRED as libc::c_int, &mut type_0);
    if (*C).encrypt as libc::c_uint == boolean_true as libc::c_int as libc::c_uint {
        if (*C).connection.reuse == 0 || (*C).connection.max == 1 as libc::c_int {
            if !((*C).ssl).is_null() {
                loop {
                    ret = SSL_get_shutdown((*C).ssl);
                    if ret < 0 as libc::c_int {
                        NOTIFY(
                            WARNING,
                            b"socket: SSL Socket closed by server: %s:%d\0" as *const u8
                                as *const libc::c_char,
                            b"sock.c\0" as *const u8 as *const libc::c_char,
                            769 as libc::c_int,
                        );
                        break;
                    } else {
                        ret = SSL_shutdown((*C).ssl);
                        if ret == 1 as libc::c_int {
                            break;
                        }
                        tries += 1;
                        tries;
                        if !(tries < 5 as libc::c_int) {
                            break;
                        }
                    }
                }
            }
            SSL_free((*C).ssl);
            (*C).ssl = 0 as *mut SSL;
            SSL_CTX_free((*C).ctx);
            (*C).ctx = 0 as *mut SSL_CTX;
            close((*C).sock);
            (*C).sock = -(1 as libc::c_int);
            (*C).connection.status = 0 as libc::c_int;
            (*C).connection.max = 0 as libc::c_int;
            (*C).connection.tested = 0 as libc::c_int;
        }
    } else if (*C).connection.reuse == 0 as libc::c_int
        || (*C).connection.max == 1 as libc::c_int
    {
        if (*C).sock != -(1 as libc::c_int) {
            if __socket_block((*C).sock, boolean_false) < 0 as libc::c_int {
                NOTIFY(
                    ERROR,
                    b"unable to set to non-blocking %s:%d\0" as *const u8
                        as *const libc::c_char,
                    b"sock.c\0" as *const u8 as *const libc::c_char,
                    795 as libc::c_int,
                );
            }
            if (*C).connection.status > 1 as libc::c_int
                && {
                    ret = shutdown((*C).sock, 2 as libc::c_int);
                    ret < 0 as libc::c_int
                }
            {
                NOTIFY(
                    ERROR,
                    b"unable to shutdown the socket %s:%d\0" as *const u8
                        as *const libc::c_char,
                    b"sock.c\0" as *const u8 as *const libc::c_char,
                    797 as libc::c_int,
                );
            }
            ret = close((*C).sock);
            if ret < 0 as libc::c_int {
                NOTIFY(
                    ERROR,
                    b"unable to close the socket %s:%d\0" as *const u8
                        as *const libc::c_char,
                    b"sock.c\0" as *const u8 as *const libc::c_char,
                    799 as libc::c_int,
                );
            }
        }
        (*C).sock = -(1 as libc::c_int);
        (*C).connection.status = 0 as libc::c_int;
        (*C).connection.max = 0 as libc::c_int;
        (*C).connection.tested = 0 as libc::c_int;
    }
    C = 0 as *mut CONN;
    pthread_setcanceltype(type_0, 0 as *mut libc::c_int);
    pthread_testcancel();
}
