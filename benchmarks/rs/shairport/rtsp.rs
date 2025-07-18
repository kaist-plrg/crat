use ::libc;
use ::c2rust_asm_casts;
use c2rust_asm_casts::AsmCastTrait;
use std::arch::asm;
extern "C" {
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
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
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strtok_r(
        __s: *mut libc::c_char,
        __delim: *const libc::c_char,
        __save_ptr: *mut *mut libc::c_char,
    ) -> *mut libc::c_char;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strsep(
        __stringp: *mut *mut libc::c_char,
        __delim: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn select(
        __nfds: libc::c_int,
        __readfds: *mut fd_set,
        __writefds: *mut fd_set,
        __exceptfds: *mut fd_set,
        __timeout: *mut timeval,
    ) -> libc::c_int;
    fn socket(
        __domain: libc::c_int,
        __type: libc::c_int,
        __protocol: libc::c_int,
    ) -> libc::c_int;
    fn bind(__fd: libc::c_int, __addr: *const sockaddr, __len: socklen_t) -> libc::c_int;
    fn getsockname(
        __fd: libc::c_int,
        __addr: *mut sockaddr,
        __len: *mut socklen_t,
    ) -> libc::c_int;
    fn setsockopt(
        __fd: libc::c_int,
        __level: libc::c_int,
        __optname: libc::c_int,
        __optval: *const libc::c_void,
        __optlen: socklen_t,
    ) -> libc::c_int;
    fn listen(__fd: libc::c_int, __n: libc::c_int) -> libc::c_int;
    fn accept(
        __fd: libc::c_int,
        __addr: *mut sockaddr,
        __addr_len: *mut socklen_t,
    ) -> libc::c_int;
    fn inet_ntop(
        __af: libc::c_int,
        __cp: *const libc::c_void,
        __buf: *mut libc::c_char,
        __len: socklen_t,
    ) -> *const libc::c_char;
    fn getaddrinfo(
        __name: *const libc::c_char,
        __service: *const libc::c_char,
        __req: *const addrinfo,
        __pai: *mut *mut addrinfo,
    ) -> libc::c_int;
    fn freeaddrinfo(__ai: *mut addrinfo);
    fn gai_strerror(__ecode: libc::c_int) -> *const libc::c_char;
    fn pthread_kill(__threadid: pthread_t, __signo: libc::c_int) -> libc::c_int;
    fn sigemptyset(__set: *mut sigset_t) -> libc::c_int;
    fn sigaddset(__set: *mut sigset_t, __signo: libc::c_int) -> libc::c_int;
    fn pthread_sigmask(
        __how: libc::c_int,
        __newmask: *const __sigset_t,
        __oldmask: *mut __sigset_t,
    ) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn perror(__s: *const libc::c_char);
    fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_double;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn __errno_location() -> *mut libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option::<
            unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
        >,
        __arg: *mut libc::c_void,
    ) -> libc::c_int;
    fn pthread_join(
        __th: pthread_t,
        __thread_return: *mut *mut libc::c_void,
    ) -> libc::c_int;
    fn pthread_self() -> pthread_t;
    fn pthread_mutex_trylock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn MD5_Init(c: *mut MD5_CTX) -> libc::c_int;
    fn MD5_Update(
        c: *mut MD5_CTX,
        data: *const libc::c_void,
        len: size_t,
    ) -> libc::c_int;
    fn MD5_Final(md: *mut libc::c_uchar, c: *mut MD5_CTX) -> libc::c_int;
    fn shairport_startup_complete();
    static mut config: shairport_cfg;
    fn rsa_apply(
        input: *mut uint8_t,
        inlen: libc::c_int,
        outlen: *mut libc::c_int,
        mode: libc::c_int,
    ) -> *mut uint8_t;
    fn base64_enc(input: *mut uint8_t, length: libc::c_int) -> *mut libc::c_char;
    fn base64_dec(input: *mut libc::c_char, outlen: *mut libc::c_int) -> *mut uint8_t;
    fn debug(level: libc::c_int, format: *mut libc::c_char, _: ...);
    fn warn(format: *mut libc::c_char, _: ...);
    fn die(format: *mut libc::c_char, _: ...);
    fn mdns_register();
    fn player_play(cfg: *mut stream_cfg) -> libc::c_int;
    fn player_stop();
    fn player_volume(f: libc::c_double);
    fn player_flush();
    fn metadata_write();
    fn metadata_cover_image(
        buf: *const libc::c_char,
        len: libc::c_int,
        ext: *const libc::c_char,
    );
    static mut player_meta: metadata;
    fn metadata_set(field: *mut *mut libc::c_char, value: *const libc::c_char);
    fn rtp_setup(
        remote: *mut sockaddr_storage,
        controlport: libc::c_int,
        timingport: libc::c_int,
    ) -> libc::c_int;
    fn rtp_shutdown();
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
pub type ssize_t = __ssize_t;
pub type int32_t = __int32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
pub type sigset_t = __sigset_t;
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
    pub __fds_bits: [__fd_mask; 16],
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
pub type pthread_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_attr_t {
    pub __size: [libc::c_char; 56],
    pub __align: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
    pub __align: libc::c_long,
}
pub type socklen_t = __socklen_t;
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
pub struct sockaddr_storage {
    pub ss_family: sa_family_t,
    pub __ss_padding: [libc::c_char; 118],
    pub __ss_align: libc::c_ulong,
}
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type in_addr_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in_addr {
    pub s_addr: in_addr_t,
}
pub type C2RustUnnamed = libc::c_uint;
pub const IPPROTO_MAX: C2RustUnnamed = 256;
pub const IPPROTO_RAW: C2RustUnnamed = 255;
pub const IPPROTO_MPLS: C2RustUnnamed = 137;
pub const IPPROTO_UDPLITE: C2RustUnnamed = 136;
pub const IPPROTO_SCTP: C2RustUnnamed = 132;
pub const IPPROTO_COMP: C2RustUnnamed = 108;
pub const IPPROTO_PIM: C2RustUnnamed = 103;
pub const IPPROTO_ENCAP: C2RustUnnamed = 98;
pub const IPPROTO_BEETPH: C2RustUnnamed = 94;
pub const IPPROTO_MTP: C2RustUnnamed = 92;
pub const IPPROTO_AH: C2RustUnnamed = 51;
pub const IPPROTO_ESP: C2RustUnnamed = 50;
pub const IPPROTO_GRE: C2RustUnnamed = 47;
pub const IPPROTO_RSVP: C2RustUnnamed = 46;
pub const IPPROTO_IPV6: C2RustUnnamed = 41;
pub const IPPROTO_DCCP: C2RustUnnamed = 33;
pub const IPPROTO_TP: C2RustUnnamed = 29;
pub const IPPROTO_IDP: C2RustUnnamed = 22;
pub const IPPROTO_UDP: C2RustUnnamed = 17;
pub const IPPROTO_PUP: C2RustUnnamed = 12;
pub const IPPROTO_EGP: C2RustUnnamed = 8;
pub const IPPROTO_TCP: C2RustUnnamed = 6;
pub const IPPROTO_IPIP: C2RustUnnamed = 4;
pub const IPPROTO_IGMP: C2RustUnnamed = 2;
pub const IPPROTO_ICMP: C2RustUnnamed = 1;
pub const IPPROTO_IP: C2RustUnnamed = 0;
pub type in_port_t = uint16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in6_addr {
    pub __in6_u: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub __u6_addr8: [uint8_t; 16],
    pub __u6_addr16: [uint16_t; 8],
    pub __u6_addr32: [uint32_t; 4],
}
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
pub struct sockaddr_in6 {
    pub sin6_family: sa_family_t,
    pub sin6_port: in_port_t,
    pub sin6_flowinfo: uint32_t,
    pub sin6_addr: in6_addr,
    pub sin6_scope_id: uint32_t,
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
pub type C2RustUnnamed_1 = libc::c_uint;
pub const PTHREAD_MUTEX_DEFAULT: C2RustUnnamed_1 = 0;
pub const PTHREAD_MUTEX_ERRORCHECK: C2RustUnnamed_1 = 2;
pub const PTHREAD_MUTEX_RECURSIVE: C2RustUnnamed_1 = 1;
pub const PTHREAD_MUTEX_NORMAL: C2RustUnnamed_1 = 0;
pub const PTHREAD_MUTEX_ADAPTIVE_NP: C2RustUnnamed_1 = 3;
pub const PTHREAD_MUTEX_ERRORCHECK_NP: C2RustUnnamed_1 = 2;
pub const PTHREAD_MUTEX_RECURSIVE_NP: C2RustUnnamed_1 = 1;
pub const PTHREAD_MUTEX_TIMED_NP: C2RustUnnamed_1 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MD5state_st {
    pub A: libc::c_uint,
    pub B: libc::c_uint,
    pub C: libc::c_uint,
    pub D: libc::c_uint,
    pub Nl: libc::c_uint,
    pub Nh: libc::c_uint,
    pub data: [libc::c_uint; 16],
    pub num: libc::c_uint,
}
pub type MD5_CTX = MD5state_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct audio_output {
    pub help: Option::<unsafe extern "C" fn() -> ()>,
    pub name: *mut libc::c_char,
    pub init: Option::<
        unsafe extern "C" fn(libc::c_int, *mut *mut libc::c_char) -> libc::c_int,
    >,
    pub deinit: Option::<unsafe extern "C" fn() -> ()>,
    pub start: Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
    pub play: Option::<unsafe extern "C" fn(*mut libc::c_short, libc::c_int) -> ()>,
    pub stop: Option::<unsafe extern "C" fn() -> ()>,
    pub volume: Option::<unsafe extern "C" fn(libc::c_double) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mdns_backend {
    pub name: *mut libc::c_char,
    pub mdns_register: Option::<
        unsafe extern "C" fn(*mut libc::c_char, libc::c_int) -> libc::c_int,
    >,
    pub mdns_unregister: Option::<unsafe extern "C" fn() -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct shairport_cfg {
    pub password: *mut libc::c_char,
    pub apname: *mut libc::c_char,
    pub hw_addr: [uint8_t; 6],
    pub port: libc::c_int,
    pub output_name: *mut libc::c_char,
    pub output: *mut audio_output,
    pub mdns_name: *mut libc::c_char,
    pub mdns: *mut mdns_backend,
    pub buffer_start_fill: libc::c_int,
    pub daemonise: libc::c_int,
    pub cmd_start: *mut libc::c_char,
    pub cmd_stop: *mut libc::c_char,
    pub cmd_blocking: libc::c_int,
    pub meta_dir: *mut libc::c_char,
    pub pidfile: *mut libc::c_char,
    pub logfile: *mut libc::c_char,
    pub errfile: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct metadata {
    pub artist: *mut libc::c_char,
    pub title: *mut libc::c_char,
    pub album: *mut libc::c_char,
    pub artwork: *mut libc::c_char,
    pub comment: *mut libc::c_char,
    pub genre: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stream_cfg {
    pub aesiv: [uint8_t; 16],
    pub aeskey: [uint8_t; 16],
    pub fmtp: [int32_t; 12],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rtsp_conn_info {
    pub fd: libc::c_int,
    pub stream: stream_cfg,
    pub remote: sockaddr_storage,
    pub running: libc::c_int,
    pub thread: pthread_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rtsp_message {
    pub nheaders: libc::c_int,
    pub name: [*mut libc::c_char; 16],
    pub value: [*mut libc::c_char; 16],
    pub contentlength: libc::c_int,
    pub content: *mut libc::c_char,
    pub method: [libc::c_char; 16],
    pub respcode: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct method_handler {
    pub method: *mut libc::c_char,
    pub handler: Option::<
        unsafe extern "C" fn(
            *mut rtsp_conn_info,
            *mut rtsp_message,
            *mut rtsp_message,
        ) -> (),
    >,
}
#[inline]
unsafe extern "C" fn __bswap_32(mut __bsx: __uint32_t) -> __uint32_t {
    return (__bsx & 0xff000000 as libc::c_uint) >> 24 as libc::c_int
        | (__bsx & 0xff0000 as libc::c_uint) >> 8 as libc::c_int
        | (__bsx & 0xff00 as libc::c_uint) << 8 as libc::c_int
        | (__bsx & 0xff as libc::c_uint) << 24 as libc::c_int;
}
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
#[inline]
unsafe extern "C" fn atof(mut __nptr: *const libc::c_char) -> libc::c_double {
    return strtod(__nptr, 0 as *mut libc::c_void as *mut *mut libc::c_char);
}
#[inline]
unsafe extern "C" fn pthread_equal(
    mut __thread1: pthread_t,
    mut __thread2: pthread_t,
) -> libc::c_int {
    return (__thread1 == __thread2) as libc::c_int;
}
static mut playing_mutex: pthread_mutex_t = pthread_mutex_t {
    __data: {
        let mut init = __pthread_mutex_s {
            __lock: 0 as libc::c_int,
            __count: 0 as libc::c_int as libc::c_uint,
            __owner: 0 as libc::c_int,
            __nusers: 0 as libc::c_int as libc::c_uint,
            __kind: PTHREAD_MUTEX_TIMED_NP as libc::c_int,
            __spins: 0 as libc::c_int as libc::c_short,
            __elision: 0 as libc::c_int as libc::c_short,
            __list: {
                let mut init = __pthread_internal_list {
                    __prev: 0 as *const __pthread_internal_list
                        as *mut __pthread_internal_list,
                    __next: 0 as *const __pthread_internal_list
                        as *mut __pthread_internal_list,
                };
                init
            },
        };
        init
    },
};
static mut please_shutdown: libc::c_int = 0 as libc::c_int;
static mut playing_thread: pthread_t = 0;
#[inline]
unsafe extern "C" fn rtsp_playing() -> libc::c_int {
    if pthread_mutex_trylock(&mut playing_mutex) != 0 {
        return pthread_equal(playing_thread, pthread_self())
    } else {
        pthread_mutex_unlock(&mut playing_mutex);
        return 0 as libc::c_int;
    };
}
unsafe extern "C" fn rtsp_take_player() {
    if rtsp_playing() != 0 {
        return;
    }
    if pthread_mutex_trylock(&mut playing_mutex) != 0 {
        debug(
            1 as libc::c_int,
            b"shutting down playing thread\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        please_shutdown = 1 as libc::c_int;
        pthread_kill(playing_thread, 10 as libc::c_int);
        pthread_mutex_lock(&mut playing_mutex);
    }
    playing_thread = pthread_self();
}
pub unsafe extern "C" fn rtsp_shutdown_stream() {
    rtsp_take_player();
    pthread_mutex_unlock(&mut playing_mutex);
}
static mut conns: *mut *mut rtsp_conn_info = 0 as *const *mut rtsp_conn_info
    as *mut *mut rtsp_conn_info;
static mut nconns: libc::c_int = 0 as libc::c_int;
unsafe extern "C" fn track_thread(mut conn: *mut rtsp_conn_info) {
    conns = realloc(
        conns as *mut libc::c_void,
        (::std::mem::size_of::<*mut rtsp_conn_info>() as libc::c_ulong)
            .wrapping_mul((nconns + 1 as libc::c_int) as libc::c_ulong),
    ) as *mut *mut rtsp_conn_info;
    let ref mut fresh0 = *conns.offset(nconns as isize);
    *fresh0 = conn;
    nconns += 1;
    nconns;
}
unsafe extern "C" fn cleanup_threads() {
    let mut retval: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut i: libc::c_int = 0;
    debug(
        2 as libc::c_int,
        b"culling threads.\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    i = 0 as libc::c_int;
    while i < nconns {
        if (**conns.offset(i as isize)).running == 0 as libc::c_int {
            pthread_join((**conns.offset(i as isize)).thread, &mut retval);
            free(*conns.offset(i as isize) as *mut libc::c_void);
            debug(
                2 as libc::c_int,
                b"one joined\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            nconns -= 1;
            nconns;
            if nconns != 0 {
                let ref mut fresh1 = *conns.offset(i as isize);
                *fresh1 = *conns.offset(nconns as isize);
            }
        } else {
            i += 1;
            i;
        }
    }
}
unsafe extern "C" fn nextline(
    mut in_0: *mut libc::c_char,
    mut inbuf: libc::c_int,
) -> *mut libc::c_char {
    let mut out: *mut libc::c_char = 0 as *mut libc::c_char;
    while inbuf != 0 {
        if *in_0 as libc::c_int == '\r' as i32 {
            let fresh2 = in_0;
            in_0 = in_0.offset(1);
            *fresh2 = 0 as libc::c_int as libc::c_char;
            out = in_0;
        }
        if *in_0 as libc::c_int == '\n' as i32 {
            let fresh3 = in_0;
            in_0 = in_0.offset(1);
            *fresh3 = 0 as libc::c_int as libc::c_char;
            out = in_0;
        }
        if !out.is_null() {
            break;
        }
        in_0 = in_0.offset(1);
        in_0;
        inbuf -= 1;
        inbuf;
    }
    return out;
}
unsafe extern "C" fn msg_init() -> *mut rtsp_message {
    let mut msg: *mut rtsp_message = malloc(
        ::std::mem::size_of::<rtsp_message>() as libc::c_ulong,
    ) as *mut rtsp_message;
    memset(
        msg as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<rtsp_message>() as libc::c_ulong,
    );
    return msg;
}
unsafe extern "C" fn msg_add_header(
    mut msg: *mut rtsp_message,
    mut name: *mut libc::c_char,
    mut value: *mut libc::c_char,
) -> libc::c_int {
    if (*msg).nheaders as libc::c_ulong
        >= (::std::mem::size_of::<[*mut libc::c_char; 16]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
    {
        warn(
            b"too many headers?!\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return 1 as libc::c_int;
    }
    (*msg).name[(*msg).nheaders as usize] = strdup(name);
    (*msg).value[(*msg).nheaders as usize] = strdup(value);
    (*msg).nheaders += 1;
    (*msg).nheaders;
    return 0 as libc::c_int;
}
unsafe extern "C" fn msg_get_header(
    mut msg: *mut rtsp_message,
    mut name: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*msg).nheaders {
        if strcasecmp((*msg).name[i as usize], name) == 0 {
            return (*msg).value[i as usize];
        }
        i += 1;
        i;
    }
    return 0 as *mut libc::c_char;
}
unsafe extern "C" fn msg_free(mut msg: *mut rtsp_message) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*msg).nheaders {
        free((*msg).name[i as usize] as *mut libc::c_void);
        free((*msg).value[i as usize] as *mut libc::c_void);
        i += 1;
        i;
    }
    if !((*msg).content).is_null() {
        free((*msg).content as *mut libc::c_void);
    }
    free(msg as *mut libc::c_void);
}
unsafe extern "C" fn msg_handle_line(
    mut pmsg: *mut *mut rtsp_message,
    mut line: *mut libc::c_char,
) -> libc::c_int {
    let mut msg: *mut rtsp_message = *pmsg;
    if msg.is_null() {
        msg = msg_init();
        *pmsg = msg;
        let mut sp: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
        debug(
            1 as libc::c_int,
            b"received request: %s\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            line,
        );
        p = strtok_r(line, b" \0" as *const u8 as *const libc::c_char, &mut sp);
        if !p.is_null() {
            strncpy(
                ((*msg).method).as_mut_ptr(),
                p,
                (::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            );
            p = strtok_r(
                0 as *mut libc::c_char,
                b" \0" as *const u8 as *const libc::c_char,
                &mut sp,
            );
            if !p.is_null() {
                p = strtok_r(
                    0 as *mut libc::c_char,
                    b" \0" as *const u8 as *const libc::c_char,
                    &mut sp,
                );
                if !p.is_null() {
                    if !(strcmp(p, b"RTSP/1.0\0" as *const u8 as *const libc::c_char)
                        != 0)
                    {
                        return -(1 as libc::c_int);
                    }
                }
            }
        }
    } else if strlen(line) != 0 {
        let mut p_0: *mut libc::c_char = 0 as *mut libc::c_char;
        p_0 = strstr(line, b": \0" as *const u8 as *const libc::c_char);
        if p_0.is_null() {
            warn(
                b"bad header: >>%s<<\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                line,
            );
        } else {
            *p_0 = 0 as libc::c_int as libc::c_char;
            p_0 = p_0.offset(2 as libc::c_int as isize);
            msg_add_header(msg, line, p_0);
            debug(
                2 as libc::c_int,
                b"    %s: %s\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                line,
                p_0,
            );
            return -(1 as libc::c_int);
        }
    } else {
        let mut cl: *mut libc::c_char = msg_get_header(
            msg,
            b"Content-Length\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        if !cl.is_null() { return atoi(cl) } else { return 0 as libc::c_int }
    }
    *pmsg = 0 as *mut rtsp_message;
    msg_free(msg);
    return 0 as libc::c_int;
}
unsafe extern "C" fn rtsp_read_request(mut fd: libc::c_int) -> *mut rtsp_message {
    let mut current_block: u64;
    let mut buflen: ssize_t = 512 as libc::c_int as ssize_t;
    let mut buf: *mut libc::c_char = malloc(
        (buflen + 1 as libc::c_int as libc::c_long) as libc::c_ulong,
    ) as *mut libc::c_char;
    let mut msg: *mut rtsp_message = 0 as *mut rtsp_message;
    let mut nread: ssize_t = 0;
    let mut inbuf: ssize_t = 0 as libc::c_int as ssize_t;
    let mut msg_size: libc::c_int = -(1 as libc::c_int);
    's_13: loop {
        if !(msg_size < 0 as libc::c_int) {
            current_block = 6057473163062296781;
            break;
        }
        if please_shutdown != 0 {
            debug(
                1 as libc::c_int,
                b"RTSP shutdown requested\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            current_block = 14236761776607291104;
            break;
        } else {
            nread = read(
                fd,
                buf.offset(inbuf as isize) as *mut libc::c_void,
                (buflen - inbuf) as size_t,
            );
            if nread == 0 {
                debug(
                    1 as libc::c_int,
                    b"RTSP connection closed\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
                current_block = 14236761776607291104;
                break;
            } else if nread < 0 as libc::c_int as libc::c_long {
                if *__errno_location() == 4 as libc::c_int {
                    continue;
                }
                perror(b"read failure\0" as *const u8 as *const libc::c_char);
                current_block = 14236761776607291104;
                break;
            } else {
                inbuf += nread;
                let mut next: *mut libc::c_char = 0 as *mut libc::c_char;
                while msg_size < 0 as libc::c_int
                    && {
                        next = nextline(buf, inbuf as libc::c_int);
                        !next.is_null()
                    }
                {
                    msg_size = msg_handle_line(&mut msg, buf);
                    if msg.is_null() {
                        warn(
                            b"no RTSP header received\0" as *const u8
                                as *const libc::c_char as *mut libc::c_char,
                        );
                        current_block = 14236761776607291104;
                        break 's_13;
                    } else {
                        inbuf -= next.offset_from(buf) as libc::c_long;
                        if inbuf != 0 {
                            memmove(
                                buf as *mut libc::c_void,
                                next as *const libc::c_void,
                                inbuf as libc::c_ulong,
                            );
                        }
                    }
                }
            }
        }
    }
    match current_block {
        6057473163062296781 => {
            if msg_size as libc::c_long > buflen {
                buf = realloc(buf as *mut libc::c_void, msg_size as libc::c_ulong)
                    as *mut libc::c_char;
                if buf.is_null() {
                    warn(
                        b"too much content\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    );
                    current_block = 14236761776607291104;
                } else {
                    buflen = msg_size as ssize_t;
                    current_block = 11298138898191919651;
                }
            } else {
                current_block = 11298138898191919651;
            }
            match current_block {
                14236761776607291104 => {}
                _ => {
                    loop {
                        if !(inbuf < msg_size as libc::c_long) {
                            current_block = 11932355480408055363;
                            break;
                        }
                        nread = read(
                            fd,
                            buf.offset(inbuf as isize) as *mut libc::c_void,
                            (msg_size as libc::c_long - inbuf) as size_t,
                        );
                        if nread == 0 {
                            current_block = 14236761776607291104;
                            break;
                        }
                        if nread == 4 as libc::c_int as libc::c_long {
                            continue;
                        }
                        if nread < 0 as libc::c_int as libc::c_long {
                            perror(
                                b"read failure\0" as *const u8 as *const libc::c_char,
                            );
                            current_block = 14236761776607291104;
                            break;
                        } else {
                            inbuf += nread;
                        }
                    }
                    match current_block {
                        14236761776607291104 => {}
                        _ => {
                            (*msg).contentlength = inbuf as libc::c_int;
                            (*msg).content = buf;
                            return msg;
                        }
                    }
                }
            }
        }
        _ => {}
    }
    free(buf as *mut libc::c_void);
    if !msg.is_null() {
        msg_free(msg);
    }
    return 0 as *mut rtsp_message;
}
unsafe extern "C" fn msg_write_response(
    mut fd: libc::c_int,
    mut resp: *mut rtsp_message,
) {
    let mut pkt: [libc::c_char; 1024] = [0; 1024];
    let mut pktfree: libc::c_int = ::std::mem::size_of::<[libc::c_char; 1024]>()
        as libc::c_ulong as libc::c_int;
    let mut p: *mut libc::c_char = pkt.as_mut_ptr();
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    n = snprintf(
        p,
        pktfree as libc::c_ulong,
        b"RTSP/1.0 %d %s\r\n\0" as *const u8 as *const libc::c_char,
        (*resp).respcode,
        if (*resp).respcode == 200 as libc::c_int {
            b"OK\0" as *const u8 as *const libc::c_char
        } else {
            b"Error\0" as *const u8 as *const libc::c_char
        },
    );
    debug(
        1 as libc::c_int,
        b"sending response: %s\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        pkt.as_mut_ptr(),
    );
    pktfree -= n;
    p = p.offset(n as isize);
    i = 0 as libc::c_int;
    while i < (*resp).nheaders {
        debug(
            2 as libc::c_int,
            b"    %s: %s\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            (*resp).name[i as usize],
            (*resp).value[i as usize],
        );
        n = snprintf(
            p,
            pktfree as libc::c_ulong,
            b"%s: %s\r\n\0" as *const u8 as *const libc::c_char,
            (*resp).name[i as usize],
            (*resp).value[i as usize],
        );
        pktfree -= n;
        p = p.offset(n as isize);
        if pktfree <= 0 as libc::c_int {
            die(
                b"Attempted to write overlong RTSP packet\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            );
        }
        i += 1;
        i;
    }
    if pktfree < 3 as libc::c_int {
        die(
            b"Attempted to write overlong RTSP packet\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        );
    }
    strcpy(p, b"\r\n\0" as *const u8 as *const libc::c_char);
    write(
        fd,
        pkt.as_mut_ptr() as *const libc::c_void,
        (p.offset_from(pkt.as_mut_ptr()) as libc::c_long
            + 2 as libc::c_int as libc::c_long) as size_t,
    );
}
unsafe extern "C" fn handle_options(
    mut conn: *mut rtsp_conn_info,
    mut req: *mut rtsp_message,
    mut resp: *mut rtsp_message,
) {
    (*resp).respcode = 200 as libc::c_int;
    msg_add_header(
        resp,
        b"Public\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"ANNOUNCE, SETUP, RECORD, PAUSE, FLUSH, TEARDOWN, OPTIONS, GET_PARAMETER, SET_PARAMETER\0"
            as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
unsafe extern "C" fn handle_teardown(
    mut conn: *mut rtsp_conn_info,
    mut req: *mut rtsp_message,
    mut resp: *mut rtsp_message,
) {
    if rtsp_playing() == 0 {
        return;
    }
    (*resp).respcode = 200 as libc::c_int;
    msg_add_header(
        resp,
        b"Connection\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"close\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    please_shutdown = 1 as libc::c_int;
}
unsafe extern "C" fn handle_flush(
    mut conn: *mut rtsp_conn_info,
    mut req: *mut rtsp_message,
    mut resp: *mut rtsp_message,
) {
    if rtsp_playing() == 0 {
        return;
    }
    player_flush();
    (*resp).respcode = 200 as libc::c_int;
}
unsafe extern "C" fn handle_setup(
    mut conn: *mut rtsp_conn_info,
    mut req: *mut rtsp_message,
    mut resp: *mut rtsp_message,
) {
    let mut cport: libc::c_int = 0;
    let mut tport: libc::c_int = 0;
    let mut hdr: *mut libc::c_char = msg_get_header(
        req,
        b"Transport\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if hdr.is_null() {
        return;
    }
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    p = strstr(hdr, b"control_port=\0" as *const u8 as *const libc::c_char);
    if p.is_null() {
        return;
    }
    p = (strchr(p, '=' as i32)).offset(1 as libc::c_int as isize);
    cport = atoi(p);
    p = strstr(hdr, b"timing_port=\0" as *const u8 as *const libc::c_char);
    if p.is_null() {
        return;
    }
    p = (strchr(p, '=' as i32)).offset(1 as libc::c_int as isize);
    tport = atoi(p);
    rtsp_take_player();
    let mut sport: libc::c_int = rtp_setup(&mut (*conn).remote, cport, tport);
    if sport == 0 {
        return;
    }
    player_play(&mut (*conn).stream);
    let mut resphdr: [libc::c_char; 100] = [0; 100];
    snprintf(
        resphdr.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 100]>() as libc::c_ulong,
        b"RTP/AVP/UDP;unicast;mode=record;server_port=%d;control_port=%d;timing_port=%d\0"
            as *const u8 as *const libc::c_char,
        sport,
        sport,
        sport,
    );
    msg_add_header(
        resp,
        b"Transport\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        resphdr.as_mut_ptr(),
    );
    msg_add_header(
        resp,
        b"Session\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    (*resp).respcode = 200 as libc::c_int;
}
unsafe extern "C" fn handle_ignore(
    mut conn: *mut rtsp_conn_info,
    mut req: *mut rtsp_message,
    mut resp: *mut rtsp_message,
) {
    (*resp).respcode = 200 as libc::c_int;
}
unsafe extern "C" fn handle_set_parameter_parameter(
    mut conn: *mut rtsp_conn_info,
    mut req: *mut rtsp_message,
    mut resp: *mut rtsp_message,
) {
    let mut cp: *mut libc::c_char = (*req).content;
    let mut cp_left: libc::c_int = (*req).contentlength;
    let mut next: *mut libc::c_char = 0 as *mut libc::c_char;
    while cp_left != 0 && !cp.is_null() {
        next = nextline(cp, cp_left);
        cp_left = (cp_left as libc::c_long - next.offset_from(cp) as libc::c_long)
            as libc::c_int;
        if strncmp(
            cp,
            b"volume: \0" as *const u8 as *const libc::c_char,
            8 as libc::c_int as libc::c_ulong,
        ) == 0
        {
            let mut volume: libc::c_float = atof(cp.offset(8 as libc::c_int as isize))
                as libc::c_float;
            debug(
                1 as libc::c_int,
                b"volume: %f\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                volume as libc::c_double,
            );
            player_volume(volume as libc::c_double);
        } else if strncmp(
            cp,
            b"progress: \0" as *const u8 as *const libc::c_char,
            10 as libc::c_int as libc::c_ulong,
        ) == 0
        {
            let mut progress: *mut libc::c_char = cp.offset(10 as libc::c_int as isize);
            debug(
                1 as libc::c_int,
                b"progress: %s\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                progress,
            );
        } else {
            debug(
                1 as libc::c_int,
                b"unrecognised parameter: >>%s<< (%d)\n\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                cp,
                strlen(cp),
            );
        }
        cp = next;
    }
}
unsafe extern "C" fn handle_set_parameter_metadata(
    mut conn: *mut rtsp_conn_info,
    mut req: *mut rtsp_message,
    mut resp: *mut rtsp_message,
) {
    let mut cp: *mut libc::c_char = (*req).content;
    let mut cl: libc::c_int = (*req).contentlength;
    let mut off: libc::c_uint = 8 as libc::c_int as libc::c_uint;
    while off < cl as libc::c_uint {
        let mut tag: [libc::c_char; 5] = [0; 5];
        strncpy(
            tag.as_mut_ptr(),
            cp.offset(off as isize),
            4 as libc::c_int as libc::c_ulong,
        );
        tag[4 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
        off = off.wrapping_add(4 as libc::c_int as libc::c_uint);
        let mut vl: uint32_t = __bswap_32(*(cp.offset(off as isize) as *mut uint32_t));
        off = (off as libc::c_ulong)
            .wrapping_add(::std::mem::size_of::<uint32_t>() as libc::c_ulong)
            as libc::c_uint as libc::c_uint;
        let mut val: *mut libc::c_char = malloc(
            vl.wrapping_add(1 as libc::c_int as libc::c_uint) as libc::c_ulong,
        ) as *mut libc::c_char;
        strncpy(val, cp.offset(off as isize), vl as libc::c_ulong);
        *val.offset(vl as isize) = '\0' as i32 as libc::c_char;
        off = off.wrapping_add(vl);
        debug(
            2 as libc::c_int,
            b"Tag: %s   Content: %s\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            tag.as_mut_ptr(),
            val,
        );
        if strncmp(
            tag.as_mut_ptr(),
            b"asal \0" as *const u8 as *const libc::c_char,
            4 as libc::c_int as libc::c_ulong,
        ) == 0
        {
            debug(
                1 as libc::c_int,
                b"META Album: %s\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                val,
            );
            metadata_set(&mut player_meta.album, val);
        } else if strncmp(
            tag.as_mut_ptr(),
            b"asar \0" as *const u8 as *const libc::c_char,
            4 as libc::c_int as libc::c_ulong,
        ) == 0
        {
            debug(
                1 as libc::c_int,
                b"META Artist: %s\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                val,
            );
            metadata_set(&mut player_meta.artist, val);
        } else if strncmp(
            tag.as_mut_ptr(),
            b"ascm \0" as *const u8 as *const libc::c_char,
            4 as libc::c_int as libc::c_ulong,
        ) == 0
        {
            debug(
                1 as libc::c_int,
                b"META Comment: %s\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                val,
            );
            metadata_set(&mut player_meta.comment, val);
        } else if strncmp(
            tag.as_mut_ptr(),
            b"asgn \0" as *const u8 as *const libc::c_char,
            4 as libc::c_int as libc::c_ulong,
        ) == 0
        {
            debug(
                1 as libc::c_int,
                b"META Genre: %s\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                val,
            );
            metadata_set(&mut player_meta.genre, val);
        } else if strncmp(
            tag.as_mut_ptr(),
            b"minm \0" as *const u8 as *const libc::c_char,
            4 as libc::c_int as libc::c_ulong,
        ) == 0
        {
            debug(
                1 as libc::c_int,
                b"META Title: %s\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                val,
            );
            metadata_set(&mut player_meta.title, val);
        }
        free(val as *mut libc::c_void);
    }
    metadata_write();
}
unsafe extern "C" fn handle_set_parameter_coverart(
    mut conn: *mut rtsp_conn_info,
    mut req: *mut rtsp_message,
    mut resp: *mut rtsp_message,
) {
    let mut cp: *mut libc::c_char = (*req).content;
    let mut cl: libc::c_int = (*req).contentlength;
    let mut ct: *mut libc::c_char = msg_get_header(
        req,
        b"Content-Type\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if strncmp(
        ct,
        b"image/jpeg\0" as *const u8 as *const libc::c_char,
        10 as libc::c_int as libc::c_ulong,
    ) == 0
    {
        metadata_cover_image(cp, cl, b"jpg\0" as *const u8 as *const libc::c_char);
    } else if strncmp(
        ct,
        b"image/png\0" as *const u8 as *const libc::c_char,
        9 as libc::c_int as libc::c_ulong,
    ) == 0
    {
        metadata_cover_image(cp, cl, b"png\0" as *const u8 as *const libc::c_char);
    } else {
        metadata_cover_image(
            0 as *const libc::c_char,
            0 as libc::c_int,
            0 as *const libc::c_char,
        );
    };
}
unsafe extern "C" fn handle_set_parameter(
    mut conn: *mut rtsp_conn_info,
    mut req: *mut rtsp_message,
    mut resp: *mut rtsp_message,
) {
    if (*req).contentlength == 0 {
        debug(
            1 as libc::c_int,
            b"received empty SET_PARAMETER request\n\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        );
    }
    let mut ct: *mut libc::c_char = msg_get_header(
        req,
        b"Content-Type\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if !ct.is_null() {
        debug(
            2 as libc::c_int,
            b"SET_PARAMETER Content-Type: %s\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            ct,
        );
        if strncmp(
            ct,
            b"application/x-dmap-tagged\0" as *const u8 as *const libc::c_char,
            25 as libc::c_int as libc::c_ulong,
        ) == 0
        {
            debug(
                1 as libc::c_int,
                b"received metadata tags in SET_PARAMETER request\n\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            );
            handle_set_parameter_metadata(conn, req, resp);
        } else if strncmp(
            ct,
            b"image/jpeg\0" as *const u8 as *const libc::c_char,
            10 as libc::c_int as libc::c_ulong,
        ) == 0
            || strncmp(
                ct,
                b"image/png\0" as *const u8 as *const libc::c_char,
                9 as libc::c_int as libc::c_ulong,
            ) == 0
            || strncmp(
                ct,
                b"image/none\0" as *const u8 as *const libc::c_char,
                10 as libc::c_int as libc::c_ulong,
            ) == 0
        {
            debug(
                1 as libc::c_int,
                b"received image in SET_PARAMETER request\n\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            );
            handle_set_parameter_coverart(conn, req, resp);
        } else if strncmp(
            ct,
            b"text/parameters\0" as *const u8 as *const libc::c_char,
            15 as libc::c_int as libc::c_ulong,
        ) == 0
        {
            debug(
                1 as libc::c_int,
                b"received parameters in SET_PARAMETER request\n\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            );
            handle_set_parameter_parameter(conn, req, resp);
        } else {
            debug(
                1 as libc::c_int,
                b"received unknown Content-Type %s in SET_PARAMETER request\n\0"
                    as *const u8 as *const libc::c_char as *mut libc::c_char,
                ct,
            );
        }
    } else {
        debug(
            1 as libc::c_int,
            b"missing Content-Type header in SET_PARAMETER request\n\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        );
    }
    (*resp).respcode = 200 as libc::c_int;
}
unsafe extern "C" fn handle_announce(
    mut conn: *mut rtsp_conn_info,
    mut req: *mut rtsp_message,
    mut resp: *mut rtsp_message,
) {
    let mut paesiv: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut prsaaeskey: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pfmtp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cp: *mut libc::c_char = (*req).content;
    let mut cp_left: libc::c_int = (*req).contentlength;
    let mut next: *mut libc::c_char = 0 as *mut libc::c_char;
    while cp_left != 0 && !cp.is_null() {
        next = nextline(cp, cp_left);
        cp_left = (cp_left as libc::c_long - next.offset_from(cp) as libc::c_long)
            as libc::c_int;
        if strncmp(
            cp,
            b"a=fmtp:\0" as *const u8 as *const libc::c_char,
            7 as libc::c_int as libc::c_ulong,
        ) == 0
        {
            pfmtp = cp.offset(7 as libc::c_int as isize);
        }
        if strncmp(
            cp,
            b"a=aesiv:\0" as *const u8 as *const libc::c_char,
            8 as libc::c_int as libc::c_ulong,
        ) == 0
        {
            paesiv = cp.offset(8 as libc::c_int as isize);
        }
        if strncmp(
            cp,
            b"a=rsaaeskey:\0" as *const u8 as *const libc::c_char,
            12 as libc::c_int as libc::c_ulong,
        ) == 0
        {
            prsaaeskey = cp.offset(12 as libc::c_int as isize);
        }
        cp = next;
    }
    if paesiv.is_null() || prsaaeskey.is_null() || pfmtp.is_null() {
        warn(
            b"required params missing from announce\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        );
        return;
    }
    let mut len: libc::c_int = 0;
    let mut keylen: libc::c_int = 0;
    let mut aesiv: *mut uint8_t = base64_dec(paesiv, &mut len);
    if len != 16 as libc::c_int {
        warn(
            b"client announced aeskey of %d bytes, wanted 16\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            len,
        );
        free(aesiv as *mut libc::c_void);
        return;
    }
    memcpy(
        ((*conn).stream.aesiv).as_mut_ptr() as *mut libc::c_void,
        aesiv as *const libc::c_void,
        16 as libc::c_int as libc::c_ulong,
    );
    free(aesiv as *mut libc::c_void);
    let mut rsaaeskey: *mut uint8_t = base64_dec(prsaaeskey, &mut len);
    let mut aeskey: *mut uint8_t = rsa_apply(
        rsaaeskey,
        len,
        &mut keylen,
        1 as libc::c_int,
    );
    free(rsaaeskey as *mut libc::c_void);
    if keylen != 16 as libc::c_int {
        warn(
            b"client announced rsaaeskey of %d bytes, wanted 16\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            keylen,
        );
        free(aeskey as *mut libc::c_void);
        return;
    }
    memcpy(
        ((*conn).stream.aeskey).as_mut_ptr() as *mut libc::c_void,
        aeskey as *const libc::c_void,
        16 as libc::c_int as libc::c_ulong,
    );
    free(aeskey as *mut libc::c_void);
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while (i as libc::c_ulong)
        < (::std::mem::size_of::<[int32_t; 12]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<int32_t>() as libc::c_ulong)
    {
        (*conn)
            .stream
            .fmtp[i
            as usize] = atoi(
            strsep(&mut pfmtp, b" \t\0" as *const u8 as *const libc::c_char),
        );
        i += 1;
        i;
    }
    (*resp).respcode = 200 as libc::c_int;
}
static mut method_handlers: [method_handler; 9] = unsafe {
    [
        {
            let mut init = method_handler {
                method: b"OPTIONS\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                handler: Some(
                    handle_options
                        as unsafe extern "C" fn(
                            *mut rtsp_conn_info,
                            *mut rtsp_message,
                            *mut rtsp_message,
                        ) -> (),
                ),
            };
            init
        },
        {
            let mut init = method_handler {
                method: b"ANNOUNCE\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                handler: Some(
                    handle_announce
                        as unsafe extern "C" fn(
                            *mut rtsp_conn_info,
                            *mut rtsp_message,
                            *mut rtsp_message,
                        ) -> (),
                ),
            };
            init
        },
        {
            let mut init = method_handler {
                method: b"FLUSH\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                handler: Some(
                    handle_flush
                        as unsafe extern "C" fn(
                            *mut rtsp_conn_info,
                            *mut rtsp_message,
                            *mut rtsp_message,
                        ) -> (),
                ),
            };
            init
        },
        {
            let mut init = method_handler {
                method: b"TEARDOWN\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                handler: Some(
                    handle_teardown
                        as unsafe extern "C" fn(
                            *mut rtsp_conn_info,
                            *mut rtsp_message,
                            *mut rtsp_message,
                        ) -> (),
                ),
            };
            init
        },
        {
            let mut init = method_handler {
                method: b"SETUP\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                handler: Some(
                    handle_setup
                        as unsafe extern "C" fn(
                            *mut rtsp_conn_info,
                            *mut rtsp_message,
                            *mut rtsp_message,
                        ) -> (),
                ),
            };
            init
        },
        {
            let mut init = method_handler {
                method: b"GET_PARAMETER\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                handler: Some(
                    handle_ignore
                        as unsafe extern "C" fn(
                            *mut rtsp_conn_info,
                            *mut rtsp_message,
                            *mut rtsp_message,
                        ) -> (),
                ),
            };
            init
        },
        {
            let mut init = method_handler {
                method: b"SET_PARAMETER\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                handler: Some(
                    handle_set_parameter
                        as unsafe extern "C" fn(
                            *mut rtsp_conn_info,
                            *mut rtsp_message,
                            *mut rtsp_message,
                        ) -> (),
                ),
            };
            init
        },
        {
            let mut init = method_handler {
                method: b"RECORD\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                handler: Some(
                    handle_ignore
                        as unsafe extern "C" fn(
                            *mut rtsp_conn_info,
                            *mut rtsp_message,
                            *mut rtsp_message,
                        ) -> (),
                ),
            };
            init
        },
        {
            let mut init = method_handler {
                method: 0 as *const libc::c_char as *mut libc::c_char,
                handler: None,
            };
            init
        },
    ]
};
unsafe extern "C" fn apple_challenge(
    mut fd: libc::c_int,
    mut req: *mut rtsp_message,
    mut resp: *mut rtsp_message,
) {
    let mut hdr: *mut libc::c_char = msg_get_header(
        req,
        b"Apple-Challenge\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if hdr.is_null() {
        return;
    }
    let mut fdsa: sockaddr_storage = sockaddr_storage {
        ss_family: 0,
        __ss_padding: [0; 118],
        __ss_align: 0,
    };
    let mut sa_len: socklen_t = ::std::mem::size_of::<sockaddr_storage>()
        as libc::c_ulong as socklen_t;
    getsockname(fd, &mut fdsa as *mut sockaddr_storage as *mut sockaddr, &mut sa_len);
    let mut chall_len: libc::c_int = 0;
    let mut chall: *mut uint8_t = base64_dec(hdr, &mut chall_len);
    let mut buf: [uint8_t; 48] = [0; 48];
    let mut bp: *mut uint8_t = buf.as_mut_ptr();
    let mut i: libc::c_int = 0;
    memset(
        buf.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[uint8_t; 48]>() as libc::c_ulong,
    );
    if chall_len > 16 as libc::c_int {
        warn(
            b"oversized Apple-Challenge!\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        free(chall as *mut libc::c_void);
        return;
    }
    memcpy(
        bp as *mut libc::c_void,
        chall as *const libc::c_void,
        chall_len as libc::c_ulong,
    );
    free(chall as *mut libc::c_void);
    bp = bp.offset(chall_len as isize);
    if fdsa.ss_family as libc::c_int == 10 as libc::c_int {
        let mut sa6: *mut sockaddr_in6 = &mut fdsa as *mut sockaddr_storage
            as *mut sockaddr_in6;
        memcpy(
            bp as *mut libc::c_void,
            ((*sa6).sin6_addr.__in6_u.__u6_addr8).as_mut_ptr() as *const libc::c_void,
            16 as libc::c_int as libc::c_ulong,
        );
        bp = bp.offset(16 as libc::c_int as isize);
    } else {
        let mut sa: *mut sockaddr_in = &mut fdsa as *mut sockaddr_storage
            as *mut sockaddr_in;
        memcpy(
            bp as *mut libc::c_void,
            &mut (*sa).sin_addr.s_addr as *mut in_addr_t as *const libc::c_void,
            4 as libc::c_int as libc::c_ulong,
        );
        bp = bp.offset(4 as libc::c_int as isize);
    }
    i = 0 as libc::c_int;
    while i < 6 as libc::c_int {
        let fresh4 = bp;
        bp = bp.offset(1);
        *fresh4 = config.hw_addr[i as usize];
        i += 1;
        i;
    }
    let mut buflen: libc::c_int = 0;
    let mut resplen: libc::c_int = 0;
    buflen = bp.offset_from(buf.as_mut_ptr()) as libc::c_long as libc::c_int;
    if buflen < 0x20 as libc::c_int {
        buflen = 0x20 as libc::c_int;
    }
    let mut challresp: *mut uint8_t = rsa_apply(
        buf.as_mut_ptr(),
        buflen,
        &mut resplen,
        0 as libc::c_int,
    );
    let mut encoded: *mut libc::c_char = base64_enc(challresp, resplen);
    let mut padding: *mut libc::c_char = strchr(encoded, '=' as i32);
    if !padding.is_null() {
        *padding = 0 as libc::c_int as libc::c_char;
    }
    msg_add_header(
        resp,
        b"Apple-Response\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        encoded,
    );
    free(challresp as *mut libc::c_void);
    free(encoded as *mut libc::c_void);
}
unsafe extern "C" fn make_nonce() -> *mut libc::c_char {
    let mut random: [uint8_t; 8] = [0; 8];
    let mut fd: libc::c_int = open(
        b"/dev/random\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
    );
    if fd < 0 as libc::c_int {
        die(
            b"could not open /dev/random!\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    read(
        fd,
        random.as_mut_ptr() as *mut libc::c_void,
        ::std::mem::size_of::<[uint8_t; 8]>() as libc::c_ulong,
    );
    close(fd);
    return base64_enc(random.as_mut_ptr(), 8 as libc::c_int);
}
unsafe extern "C" fn rtsp_auth(
    mut nonce: *mut *mut libc::c_char,
    mut req: *mut rtsp_message,
    mut resp: *mut rtsp_message,
) -> libc::c_int {
    let mut hdr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut realm: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut username: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut response: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut uri: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut quote: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut digest_urp: [uint8_t; 16] = [0; 16];
    let mut digest_mu: [uint8_t; 16] = [0; 16];
    let mut digest_total: [uint8_t; 16] = [0; 16];
    let mut ctx: MD5_CTX = MD5_CTX {
        A: 0,
        B: 0,
        C: 0,
        D: 0,
        Nl: 0,
        Nh: 0,
        data: [0; 16],
        num: 0,
    };
    let mut i: libc::c_int = 0;
    let mut buf: [libc::c_char; 33] = [0; 33];
    if (config.password).is_null() {
        return 0 as libc::c_int;
    }
    if (*nonce).is_null() {
        *nonce = make_nonce();
    } else {
        hdr = msg_get_header(
            req,
            b"Authorization\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        if !(hdr.is_null()
            || strncmp(
                hdr,
                b"Digest \0" as *const u8 as *const libc::c_char,
                7 as libc::c_int as libc::c_ulong,
            ) != 0)
        {
            realm = strstr(hdr, b"realm=\"\0" as *const u8 as *const libc::c_char);
            username = strstr(hdr, b"username=\"\0" as *const u8 as *const libc::c_char);
            response = strstr(hdr, b"response=\"\0" as *const u8 as *const libc::c_char);
            uri = strstr(hdr, b"uri=\"\0" as *const u8 as *const libc::c_char);
            if !(realm.is_null() || username.is_null() || response.is_null()
                || uri.is_null())
            {
                quote = 0 as *mut libc::c_char;
                realm = (strchr(realm, '"' as i32)).offset(1 as libc::c_int as isize);
                quote = strchr(realm, '"' as i32);
                if !quote.is_null() {
                    *quote = 0 as libc::c_int as libc::c_char;
                    username = (strchr(username, '"' as i32))
                        .offset(1 as libc::c_int as isize);
                    quote = strchr(username, '"' as i32);
                    if !quote.is_null() {
                        *quote = 0 as libc::c_int as libc::c_char;
                        response = (strchr(response, '"' as i32))
                            .offset(1 as libc::c_int as isize);
                        quote = strchr(response, '"' as i32);
                        if !quote.is_null() {
                            *quote = 0 as libc::c_int as libc::c_char;
                            uri = (strchr(uri, '"' as i32))
                                .offset(1 as libc::c_int as isize);
                            quote = strchr(uri, '"' as i32);
                            if !quote.is_null() {
                                *quote = 0 as libc::c_int as libc::c_char;
                                digest_urp = [0; 16];
                                digest_mu = [0; 16];
                                digest_total = [0; 16];
                                ctx = MD5_CTX {
                                    A: 0,
                                    B: 0,
                                    C: 0,
                                    D: 0,
                                    Nl: 0,
                                    Nh: 0,
                                    data: [0; 16],
                                    num: 0,
                                };
                                MD5_Init(&mut ctx);
                                MD5_Update(
                                    &mut ctx,
                                    username as *const libc::c_void,
                                    strlen(username),
                                );
                                MD5_Update(
                                    &mut ctx,
                                    b":\0" as *const u8 as *const libc::c_char
                                        as *const libc::c_void,
                                    1 as libc::c_int as size_t,
                                );
                                MD5_Update(
                                    &mut ctx,
                                    realm as *const libc::c_void,
                                    strlen(realm),
                                );
                                MD5_Update(
                                    &mut ctx,
                                    b":\0" as *const u8 as *const libc::c_char
                                        as *const libc::c_void,
                                    1 as libc::c_int as size_t,
                                );
                                MD5_Update(
                                    &mut ctx,
                                    config.password as *const libc::c_void,
                                    strlen(config.password),
                                );
                                MD5_Final(digest_urp.as_mut_ptr(), &mut ctx);
                                MD5_Init(&mut ctx);
                                MD5_Update(
                                    &mut ctx,
                                    ((*req).method).as_mut_ptr() as *const libc::c_void,
                                    strlen(((*req).method).as_mut_ptr()),
                                );
                                MD5_Update(
                                    &mut ctx,
                                    b":\0" as *const u8 as *const libc::c_char
                                        as *const libc::c_void,
                                    1 as libc::c_int as size_t,
                                );
                                MD5_Update(
                                    &mut ctx,
                                    uri as *const libc::c_void,
                                    strlen(uri),
                                );
                                MD5_Final(digest_mu.as_mut_ptr(), &mut ctx);
                                i = 0;
                                buf = [0; 33];
                                i = 0 as libc::c_int;
                                while i < 16 as libc::c_int {
                                    sprintf(
                                        buf.as_mut_ptr().offset((2 as libc::c_int * i) as isize),
                                        b"%02x\0" as *const u8 as *const libc::c_char,
                                        digest_urp[i as usize] as libc::c_int,
                                    );
                                    i += 1;
                                    i;
                                }
                                MD5_Init(&mut ctx);
                                MD5_Update(
                                    &mut ctx,
                                    buf.as_mut_ptr() as *const libc::c_void,
                                    32 as libc::c_int as size_t,
                                );
                                MD5_Update(
                                    &mut ctx,
                                    b":\0" as *const u8 as *const libc::c_char
                                        as *const libc::c_void,
                                    1 as libc::c_int as size_t,
                                );
                                MD5_Update(
                                    &mut ctx,
                                    *nonce as *const libc::c_void,
                                    strlen(*nonce),
                                );
                                MD5_Update(
                                    &mut ctx,
                                    b":\0" as *const u8 as *const libc::c_char
                                        as *const libc::c_void,
                                    1 as libc::c_int as size_t,
                                );
                                i = 0 as libc::c_int;
                                while i < 16 as libc::c_int {
                                    sprintf(
                                        buf.as_mut_ptr().offset((2 as libc::c_int * i) as isize),
                                        b"%02x\0" as *const u8 as *const libc::c_char,
                                        digest_mu[i as usize] as libc::c_int,
                                    );
                                    i += 1;
                                    i;
                                }
                                MD5_Update(
                                    &mut ctx,
                                    buf.as_mut_ptr() as *const libc::c_void,
                                    32 as libc::c_int as size_t,
                                );
                                MD5_Final(digest_total.as_mut_ptr(), &mut ctx);
                                i = 0 as libc::c_int;
                                while i < 16 as libc::c_int {
                                    sprintf(
                                        buf.as_mut_ptr().offset((2 as libc::c_int * i) as isize),
                                        b"%02x\0" as *const u8 as *const libc::c_char,
                                        digest_total[i as usize] as libc::c_int,
                                    );
                                    i += 1;
                                    i;
                                }
                                if strcmp(response, buf.as_mut_ptr()) == 0 {
                                    return 0 as libc::c_int;
                                }
                                warn(
                                    b"auth failed\0" as *const u8 as *const libc::c_char
                                        as *mut libc::c_char,
                                );
                            }
                        }
                    }
                }
            }
        }
    }
    (*resp).respcode = 401 as libc::c_int;
    let mut hdrlen: libc::c_int = (strlen(*nonce))
        .wrapping_add(40 as libc::c_int as libc::c_ulong) as libc::c_int;
    let mut authhdr: *mut libc::c_char = malloc(hdrlen as libc::c_ulong)
        as *mut libc::c_char;
    snprintf(
        authhdr,
        hdrlen as libc::c_ulong,
        b"Digest realm=\"taco\", nonce=\"%s\"\0" as *const u8 as *const libc::c_char,
        *nonce,
    );
    msg_add_header(
        resp,
        b"WWW-Authenticate\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        authhdr,
    );
    free(authhdr as *mut libc::c_void);
    return 1 as libc::c_int;
}
unsafe extern "C" fn rtsp_conversation_thread_func(
    mut pconn: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut set: sigset_t = sigset_t { __val: [0; 16] };
    sigemptyset(&mut set);
    sigaddset(&mut set, 10 as libc::c_int);
    pthread_sigmask(1 as libc::c_int, &mut set, 0 as *mut __sigset_t);
    let mut conn: *mut rtsp_conn_info = pconn as *mut rtsp_conn_info;
    let mut req: *mut rtsp_message = 0 as *mut rtsp_message;
    let mut resp: *mut rtsp_message = 0 as *mut rtsp_message;
    let mut hdr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut auth_nonce: *mut libc::c_char = 0 as *mut libc::c_char;
    loop {
        req = rtsp_read_request((*conn).fd);
        if req.is_null() {
            break;
        }
        let mut mh: *mut method_handler = 0 as *mut method_handler;
        resp = msg_init();
        (*resp).respcode = 400 as libc::c_int;
        apple_challenge((*conn).fd, req, resp);
        hdr = msg_get_header(
            req,
            b"CSeq\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        if !hdr.is_null() {
            msg_add_header(
                resp,
                b"CSeq\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                hdr,
            );
        }
        msg_add_header(
            resp,
            b"Audio-Jack-Status\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            b"connected; type=analog\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        if !(rtsp_auth(&mut auth_nonce, req, resp) != 0) {
            mh = 0 as *mut method_handler;
            mh = method_handlers.as_mut_ptr();
            while !((*mh).method).is_null() {
                if strcmp((*mh).method, ((*req).method).as_mut_ptr()) == 0 {
                    ((*mh).handler).unwrap()(conn, req, resp);
                    break;
                } else {
                    mh = mh.offset(1);
                    mh;
                }
            }
        }
        msg_write_response((*conn).fd, resp);
        msg_free(req);
        msg_free(resp);
    }
    debug(
        1 as libc::c_int,
        b"closing RTSP connection\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    if (*conn).fd > 0 as libc::c_int {
        close((*conn).fd);
    }
    if rtsp_playing() != 0 {
        rtp_shutdown();
        player_stop();
        please_shutdown = 0 as libc::c_int;
        pthread_mutex_unlock(&mut playing_mutex);
    }
    if !auth_nonce.is_null() {
        free(auth_nonce as *mut libc::c_void);
    }
    (*conn).running = 0 as libc::c_int;
    debug(
        2 as libc::c_int,
        b"terminating RTSP thread\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    return 0 as *mut libc::c_void;
}
unsafe extern "C" fn format_address(mut fsa: *mut sockaddr) -> *const libc::c_char {
    static mut string: [libc::c_char; 46] = [0; 46];
    let mut addr: *mut libc::c_void = 0 as *mut libc::c_void;
    if (*fsa).sa_family as libc::c_int == 10 as libc::c_int {
        let mut sa6: *mut sockaddr_in6 = fsa as *mut sockaddr_in6;
        addr = &mut (*sa6).sin6_addr as *mut in6_addr as *mut libc::c_void;
    } else {
        let mut sa: *mut sockaddr_in = fsa as *mut sockaddr_in;
        addr = &mut (*sa).sin_addr as *mut in_addr as *mut libc::c_void;
    }
    return inet_ntop(
        (*fsa).sa_family as libc::c_int,
        addr,
        string.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 46]>() as libc::c_ulong as socklen_t,
    );
}
pub unsafe extern "C" fn rtsp_listen_loop() {
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
    let mut info: *mut addrinfo = 0 as *mut addrinfo;
    let mut p: *mut addrinfo = 0 as *mut addrinfo;
    let mut portstr: [libc::c_char; 6] = [0; 6];
    let mut sockfd: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut nsock: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    memset(
        &mut hints as *mut addrinfo as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<addrinfo>() as libc::c_ulong,
    );
    hints.ai_family = 0 as libc::c_int;
    hints.ai_socktype = SOCK_STREAM as libc::c_int;
    hints.ai_flags = 0x1 as libc::c_int;
    snprintf(
        portstr.as_mut_ptr(),
        6 as libc::c_int as libc::c_ulong,
        b"%d\0" as *const u8 as *const libc::c_char,
        config.port,
    );
    ret = getaddrinfo(
        0 as *const libc::c_char,
        portstr.as_mut_ptr(),
        &mut hints,
        &mut info,
    );
    if ret != 0 {
        die(
            b"getaddrinfo failed: %s\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            gai_strerror(ret),
        );
    }
    p = info;
    while !p.is_null() {
        let mut fd: libc::c_int = socket(
            (*p).ai_family,
            (*p).ai_socktype,
            IPPROTO_TCP as libc::c_int,
        );
        let mut yes: libc::c_int = 1 as libc::c_int;
        ret = setsockopt(
            fd,
            1 as libc::c_int,
            2 as libc::c_int,
            &mut yes as *mut libc::c_int as *const libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t,
        );
        if (*p).ai_family == 10 as libc::c_int {
            ret
                |= setsockopt(
                    fd,
                    IPPROTO_IPV6 as libc::c_int,
                    26 as libc::c_int,
                    &mut yes as *mut libc::c_int as *const libc::c_void,
                    ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t,
                );
        }
        if ret == 0 {
            ret = bind(fd, (*p).ai_addr, (*p).ai_addrlen);
        }
        if ret != 0 {
            debug(
                1 as libc::c_int,
                b"Failed to bind to address %s\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                format_address((*p).ai_addr),
            );
        } else {
            debug(
                1 as libc::c_int,
                b"Bound to address %s\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                format_address((*p).ai_addr),
            );
            listen(fd, 5 as libc::c_int);
            nsock += 1;
            nsock;
            sockfd = realloc(
                sockfd as *mut libc::c_void,
                (nsock as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
            ) as *mut libc::c_int;
            *sockfd.offset((nsock - 1 as libc::c_int) as isize) = fd;
        }
        p = (*p).ai_next;
    }
    freeaddrinfo(info);
    if nsock == 0 {
        die(
            b"could not bind any listen sockets!\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    let mut maxfd: libc::c_int = -(1 as libc::c_int);
    let mut fds: fd_set = fd_set { __fds_bits: [0; 16] };
    let mut __d0: libc::c_int = 0;
    let mut __d1: libc::c_int = 0;
    let fresh5 = &mut __d0;
    let fresh6;
    let fresh7 = (::std::mem::size_of::<fd_set>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<__fd_mask>() as libc::c_ulong);
    let fresh8 = &mut __d1;
    let fresh9;
    let fresh10 = &mut *(fds.__fds_bits).as_mut_ptr().offset(0 as libc::c_int as isize)
        as *mut __fd_mask;
    asm!(
        "cld; rep; stosq", inlateout("cx") c2rust_asm_casts::AsmCast::cast_in(fresh5,
        fresh7) => fresh6, inlateout("di") c2rust_asm_casts::AsmCast::cast_in(fresh8,
        fresh10) => fresh9, inlateout("ax") 0 as libc::c_int => _,
        options(preserves_flags, att_syntax)
    );
    c2rust_asm_casts::AsmCast::cast_out(fresh5, fresh7, fresh6);
    c2rust_asm_casts::AsmCast::cast_out(fresh8, fresh10, fresh9);
    i = 0 as libc::c_int;
    while i < nsock {
        if *sockfd.offset(i as isize) > maxfd {
            maxfd = *sockfd.offset(i as isize);
        }
        i += 1;
        i;
    }
    mdns_register();
    printf(b"Listening for connections.\n\0" as *const u8 as *const libc::c_char);
    shairport_startup_complete();
    let mut acceptfd: libc::c_int = 0;
    let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    loop {
        tv.tv_sec = 300 as libc::c_int as __time_t;
        tv.tv_usec = 0 as libc::c_int as __suseconds_t;
        i = 0 as libc::c_int;
        while i < nsock {
            fds
                .__fds_bits[(*sockfd.offset(i as isize)
                / (8 as libc::c_int
                    * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                        as libc::c_int)) as usize]
                |= ((1 as libc::c_ulong)
                    << *sockfd.offset(i as isize)
                        % (8 as libc::c_int
                            * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                as libc::c_int)) as __fd_mask;
            i += 1;
            i;
        }
        ret = select(
            maxfd + 1 as libc::c_int,
            &mut fds,
            0 as *mut fd_set,
            0 as *mut fd_set,
            &mut tv,
        );
        if ret < 0 as libc::c_int {
            if !(*__errno_location() == 4 as libc::c_int) {
                break;
            }
        } else {
            cleanup_threads();
            acceptfd = -(1 as libc::c_int);
            i = 0 as libc::c_int;
            while i < nsock {
                if fds
                    .__fds_bits[(*sockfd.offset(i as isize)
                    / (8 as libc::c_int
                        * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                            as libc::c_int)) as usize]
                    & ((1 as libc::c_ulong)
                        << *sockfd.offset(i as isize)
                            % (8 as libc::c_int
                                * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                    as libc::c_int)) as __fd_mask
                    != 0 as libc::c_int as libc::c_long
                {
                    acceptfd = *sockfd.offset(i as isize);
                    break;
                } else {
                    i += 1;
                    i;
                }
            }
            if acceptfd < 0 as libc::c_int {
                continue;
            }
            let mut conn: *mut rtsp_conn_info = malloc(
                ::std::mem::size_of::<rtsp_conn_info>() as libc::c_ulong,
            ) as *mut rtsp_conn_info;
            memset(
                conn as *mut libc::c_void,
                0 as libc::c_int,
                ::std::mem::size_of::<rtsp_conn_info>() as libc::c_ulong,
            );
            let mut slen: socklen_t = ::std::mem::size_of::<sockaddr_storage>()
                as libc::c_ulong as socklen_t;
            debug(
                1 as libc::c_int,
                b"new RTSP connection\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            (*conn)
                .fd = accept(
                acceptfd,
                &mut (*conn).remote as *mut sockaddr_storage as *mut sockaddr,
                &mut slen,
            );
            if (*conn).fd < 0 as libc::c_int {
                perror(
                    b"failed to accept connection\0" as *const u8 as *const libc::c_char,
                );
                free(conn as *mut libc::c_void);
            } else {
                let mut rtsp_conversation_thread: pthread_t = 0;
                ret = pthread_create(
                    &mut rtsp_conversation_thread,
                    0 as *const pthread_attr_t,
                    Some(
                        rtsp_conversation_thread_func
                            as unsafe extern "C" fn(
                                *mut libc::c_void,
                            ) -> *mut libc::c_void,
                    ),
                    conn as *mut libc::c_void,
                );
                if ret != 0 {
                    die(
                        b"Failed to create RTSP receiver thread!\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                    );
                }
                (*conn).thread = rtsp_conversation_thread;
                (*conn).running = 1 as libc::c_int;
                track_thread(conn);
            }
        }
    }
    perror(b"select\0" as *const u8 as *const libc::c_char);
    die(
        b"fell out of the RTSP select loop\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
}
