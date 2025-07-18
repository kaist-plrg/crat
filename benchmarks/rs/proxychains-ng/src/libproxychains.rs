use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
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
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fgets(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
    fn perror(__s: *const libc::c_char);
    fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
    fn srand(__seed: libc::c_uint);
    fn abort() -> !;
    fn exit(_: libc::c_int) -> !;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __errno_location() -> *mut libc::c_int;
    fn ntohl(__netlong: uint32_t) -> uint32_t;
    fn ntohs(__netshort: uint16_t) -> uint16_t;
    fn htonl(__hostlong: uint32_t) -> uint32_t;
    fn htons(__hostshort: uint16_t) -> uint16_t;
    fn getsockopt(
        __fd: libc::c_int,
        __level: libc::c_int,
        __optname: libc::c_int,
        __optval: *mut libc::c_void,
        __optlen: *mut socklen_t,
    ) -> libc::c_int;
    fn inet_pton(
        __af: libc::c_int,
        __cp: *const libc::c_char,
        __buf: *mut libc::c_void,
    ) -> libc::c_int;
    fn inet_ntop(
        __af: libc::c_int,
        __cp: *const libc::c_void,
        __buf: *mut libc::c_char,
        __len: socklen_t,
    ) -> *const libc::c_char;
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
    fn dlsym(
        __handle: *mut libc::c_void,
        __name: *const libc::c_char,
    ) -> *mut libc::c_void;
    fn dlerror() -> *mut libc::c_char;
    fn pthread_once(
        __once_control: *mut pthread_once_t,
        __init_routine: Option::<unsafe extern "C" fn() -> ()>,
    ) -> libc::c_int;
    fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> libc::c_int;
    fn connect_proxy_chain(
        sock: libc::c_int,
        target_ip: ip_type,
        target_port: libc::c_ushort,
        pd: *mut proxy_data,
        proxy_count: libc::c_uint,
        ct: chain_type,
        max_chain: libc::c_uint,
    ) -> libc::c_int;
    fn proxychains_write_log(str: *mut libc::c_char, _: ...);
    fn proxy_getaddrinfo(
        node: *const libc::c_char,
        service: *const libc::c_char,
        hints: *const addrinfo,
        res: *mut *mut addrinfo,
    ) -> libc::c_int;
    fn proxy_gethostbyname_old(name: *const libc::c_char) -> *mut hostent;
    fn proxy_gethostbyname(
        name: *const libc::c_char,
        data: *mut gethostbyname_data,
    ) -> *mut hostent;
    fn core_initialize();
    fn proxy_freeaddrinfo(res: *mut addrinfo);
    fn get_config_path(
        default_path: *mut libc::c_char,
        pbuf: *mut libc::c_char,
        bufsize: size_t,
    ) -> *mut libc::c_char;
    fn pc_stringfromipv4(
        ip_buf_4_bytes: *mut libc::c_uchar,
        outbuf_16_bytes: *mut libc::c_char,
    );
    fn rdns_init(flavor: dns_lookup_flavor);
    fn rdns_set_daemon(addr: *mut sockaddr_in);
    fn rdns_resolver_string(flavor: dns_lookup_flavor) -> *const libc::c_char;
    static mut req_pipefd: [libc::c_int; 2];
    static mut resp_pipefd: [libc::c_int; 2];
    fn at_get_ip_for_host(host: *mut libc::c_char, len: size_t) -> ip_type4;
    fn proxychains_get_version() -> *const libc::c_char;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __clockid_t = libc::c_int;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type ssize_t = __ssize_t;
pub type socklen_t = __socklen_t;
pub type close_t = Option::<unsafe extern "C" fn(libc::c_int) -> libc::c_int>;
pub const DNSLF_RDNS_THREAD: dns_lookup_flavor = 2;
pub type dns_lookup_flavor = libc::c_uint;
pub const DNSLF_RDNS_DAEMON: dns_lookup_flavor = 3;
pub const DNSLF_RDNS_START: dns_lookup_flavor = 2;
pub const DNSLF_FORKEXEC: dns_lookup_flavor = 1;
pub const DNSLF_LIBC: dns_lookup_flavor = 0;
pub type clockid_t = __clockid_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type pthread_once_t = libc::c_int;
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
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
pub type C2RustUnnamed_0 = libc::c_uint;
pub const MSG_CMSG_CLOEXEC: C2RustUnnamed_0 = 1073741824;
pub const MSG_FASTOPEN: C2RustUnnamed_0 = 536870912;
pub const MSG_ZEROCOPY: C2RustUnnamed_0 = 67108864;
pub const MSG_BATCH: C2RustUnnamed_0 = 262144;
pub const MSG_WAITFORONE: C2RustUnnamed_0 = 65536;
pub const MSG_MORE: C2RustUnnamed_0 = 32768;
pub const MSG_NOSIGNAL: C2RustUnnamed_0 = 16384;
pub const MSG_ERRQUEUE: C2RustUnnamed_0 = 8192;
pub const MSG_RST: C2RustUnnamed_0 = 4096;
pub const MSG_CONFIRM: C2RustUnnamed_0 = 2048;
pub const MSG_SYN: C2RustUnnamed_0 = 1024;
pub const MSG_FIN: C2RustUnnamed_0 = 512;
pub const MSG_WAITALL: C2RustUnnamed_0 = 256;
pub const MSG_EOR: C2RustUnnamed_0 = 128;
pub const MSG_DONTWAIT: C2RustUnnamed_0 = 64;
pub const MSG_TRUNC: C2RustUnnamed_0 = 32;
pub const MSG_PROXY: C2RustUnnamed_0 = 16;
pub const MSG_CTRUNC: C2RustUnnamed_0 = 8;
pub const MSG_TRYHARD: C2RustUnnamed_0 = 4;
pub const MSG_DONTROUTE: C2RustUnnamed_0 = 4;
pub const MSG_PEEK: C2RustUnnamed_0 = 2;
pub const MSG_OOB: C2RustUnnamed_0 = 1;
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
    pub __in6_u: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
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
pub const SUCCESS: C2RustUnnamed_6 = 0;
pub type chain_type = libc::c_uint;
pub const ROUND_ROBIN_TYPE: chain_type = 3;
pub const RANDOM_TYPE: chain_type = 2;
pub const STRICT_TYPE: chain_type = 1;
pub const DYNAMIC_TYPE: chain_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct proxy_data {
    pub ip: ip_type,
    pub port: libc::c_ushort,
    pub pt: proxy_type,
    pub ps: proxy_state,
    pub user: [libc::c_char; 256],
    pub pass: [libc::c_char; 256],
}
pub type proxy_state = libc::c_uint;
pub const BUSY_STATE: proxy_state = 3;
pub const BLOCKED_STATE: proxy_state = 2;
pub const DOWN_STATE: proxy_state = 1;
pub const PLAY_STATE: proxy_state = 0;
pub type proxy_type = libc::c_uint;
pub const RAW_TYPE: proxy_type = 3;
pub const SOCKS5_TYPE: proxy_type = 2;
pub const SOCKS4_TYPE: proxy_type = 1;
pub const HTTP_TYPE: proxy_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ip_type {
    pub addr: C2RustUnnamed_2,
    pub is_v6: libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
    pub v4: ip_type4,
    pub v6: [libc::c_uchar; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union ip_type4 {
    pub octet: [libc::c_uchar; 4],
    pub as_int: uint32_t,
}
pub type connect_t = Option::<
    unsafe extern "C" fn(libc::c_int, *const sockaddr, socklen_t) -> libc::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub in_addr: in_addr,
    pub in_mask: in_addr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
    pub c2rust_unnamed: C2RustUnnamed_3,
    pub c2rust_unnamed_0: C2RustUnnamed_5,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub in6_addr: in6_addr,
    pub in6_prefix: libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct localaddr_arg {
    pub family: sa_family_t,
    pub port: libc::c_ushort,
    pub c2rust_unnamed: C2RustUnnamed_4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dnat_arg {
    pub orig_dst: in_addr,
    pub new_dst: in_addr,
    pub orig_port: libc::c_ushort,
    pub new_port: libc::c_ushort,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct close_range_args_t {
    pub first: libc::c_uint,
    pub last: libc::c_uint,
    pub flags: libc::c_uint,
}
pub type close_range_t = Option::<
    unsafe extern "C" fn(libc::c_uint, libc::c_uint, libc::c_int) -> libc::c_int,
>;
pub type getnameinfo_t = Option::<
    unsafe extern "C" fn(
        *const sockaddr,
        socklen_t,
        *mut libc::c_char,
        socklen_t,
        *mut libc::c_char,
        socklen_t,
        libc::c_int,
    ) -> libc::c_int,
>;
pub type gethostbyaddr_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, socklen_t, libc::c_int) -> *mut hostent,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hostent {
    pub h_name: *mut libc::c_char,
    pub h_aliases: *mut *mut libc::c_char,
    pub h_addrtype: libc::c_int,
    pub h_length: libc::c_int,
    pub h_addr_list: *mut *mut libc::c_char,
}
pub type freeaddrinfo_t = Option::<unsafe extern "C" fn(*mut addrinfo) -> libc::c_int>;
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
pub type getaddrinfo_t = Option::<
    unsafe extern "C" fn(
        *const libc::c_char,
        *const libc::c_char,
        *const addrinfo,
        *mut *mut addrinfo,
    ) -> libc::c_int,
>;
pub type gethostbyname_t = Option::<
    unsafe extern "C" fn(*const libc::c_char) -> *mut hostent,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gethostbyname_data {
    pub hostent_space: hostent,
    pub resolved_addr: in_addr_t,
    pub resolved_addr_p: [*mut libc::c_char; 2],
    pub addr_name: [libc::c_char; 256],
}
pub type sendto_t = Option::<
    unsafe extern "C" fn(
        libc::c_int,
        *const libc::c_void,
        size_t,
        libc::c_int,
        *const sockaddr,
        socklen_t,
    ) -> ssize_t,
>;
pub const RS_PT_HTTP: rs_proxyType = 3;
pub const RS_PT_SOCKS5: rs_proxyType = 2;
pub const RS_PT_SOCKS4: rs_proxyType = 1;
pub type rs_proxyType = libc::c_uint;
pub const RS_PT_NONE: rs_proxyType = 0;
pub type C2RustUnnamed_6 = libc::c_uint;
pub const BLOCKED: C2RustUnnamed_6 = 5;
pub const CHAIN_EMPTY: C2RustUnnamed_6 = 4;
pub const CHAIN_DOWN: C2RustUnnamed_6 = 3;
pub const SOCKET_ERROR: C2RustUnnamed_6 = 2;
pub const MEMORY_FAIL: C2RustUnnamed_6 = 1;
pub static mut true_close: close_t = None;
pub static mut true_close_range: close_range_t = None;
pub static mut true_connect: connect_t = None;
pub static mut true_gethostbyname: gethostbyname_t = None;
pub static mut true_getaddrinfo: getaddrinfo_t = None;
pub static mut true_freeaddrinfo: freeaddrinfo_t = None;
pub static mut true_getnameinfo: getnameinfo_t = None;
pub static mut true_gethostbyaddr: gethostbyaddr_t = None;
pub static mut true_sendto: sendto_t = None;
pub static mut tcp_read_time_out: libc::c_int = 0;
pub static mut tcp_connect_time_out: libc::c_int = 0;
pub static mut proxychains_ct: chain_type = DYNAMIC_TYPE;
pub static mut proxychains_pd: [proxy_data; 512] = [proxy_data {
    ip: ip_type {
        addr: C2RustUnnamed_2 {
            v4: ip_type4 { octet: [0; 4] },
        },
        is_v6: 0,
    },
    port: 0,
    pt: HTTP_TYPE,
    ps: PLAY_STATE,
    user: [0; 256],
    pass: [0; 256],
}; 512];
pub static mut proxychains_proxy_count: libc::c_uint = 0 as libc::c_int as libc::c_uint;
pub static mut proxychains_proxy_offset: libc::c_uint = 0 as libc::c_int as libc::c_uint;
pub static mut proxychains_got_chain_data: libc::c_int = 0 as libc::c_int;
pub static mut proxychains_max_chain: libc::c_uint = 1 as libc::c_int as libc::c_uint;
pub static mut proxychains_quiet_mode: libc::c_int = 0 as libc::c_int;
pub static mut proxychains_resolver: dns_lookup_flavor = DNSLF_LIBC;
pub static mut localnet_addr: [localaddr_arg; 64] = [localaddr_arg {
    family: 0,
    port: 0,
    c2rust_unnamed: C2RustUnnamed_4 {
        c2rust_unnamed: C2RustUnnamed_3 {
            in_addr: in_addr { s_addr: 0 },
            in_mask: in_addr { s_addr: 0 },
        },
    },
}; 64];
pub static mut num_localnet_addr: size_t = 0 as libc::c_int as size_t;
pub static mut dnats: [dnat_arg; 64] = [dnat_arg {
    orig_dst: in_addr { s_addr: 0 },
    new_dst: in_addr { s_addr: 0 },
    orig_port: 0,
    new_port: 0,
}; 64];
pub static mut num_dnats: size_t = 0 as libc::c_int as size_t;
pub static mut remote_dns_subnet: libc::c_uint = 224 as libc::c_int as libc::c_uint;
pub static mut init_once: pthread_once_t = 0 as libc::c_int;
static mut init_l: libc::c_int = 0 as libc::c_int;
unsafe extern "C" fn load_sym(
    mut symname: *mut libc::c_char,
    mut proxyfunc: *mut libc::c_void,
    mut is_mandatory: libc::c_int,
) -> *mut libc::c_void {
    let mut funcptr: *mut libc::c_void = dlsym(
        -(1 as libc::c_long) as *mut libc::c_void,
        symname,
    );
    if is_mandatory != 0 && funcptr.is_null() {
        fprintf(
            stderr,
            b"Cannot load symbol '%s' %s\n\0" as *const u8 as *const libc::c_char,
            symname,
            dlerror(),
        );
        exit(1 as libc::c_int);
    } else if funcptr.is_null() {
        return funcptr
    }
    if funcptr == proxyfunc {
        abort();
    }
    return funcptr;
}
static mut close_fds: [libc::c_int; 16] = [0; 16];
static mut close_fds_cnt: libc::c_int = 0 as libc::c_int;
static mut close_range_buffer: [close_range_args_t; 16] = [close_range_args_t {
    first: 0,
    last: 0,
    flags: 0,
}; 16];
static mut close_range_buffer_cnt: libc::c_int = 0 as libc::c_int;
unsafe extern "C" fn get_rand_seed() -> libc::c_uint {
    let mut now: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    clock_gettime(0 as libc::c_int, &mut now);
    return (now.tv_sec ^ now.tv_nsec) as libc::c_uint;
}
unsafe extern "C" fn do_init() {
    srand(get_rand_seed());
    core_initialize();
    get_chain_data(
        proxychains_pd.as_mut_ptr(),
        &mut proxychains_proxy_count,
        &mut proxychains_ct,
    );
    proxychains_write_log(
        b"[proxychains] DLL init: proxychains-ng %s\n\0" as *const u8
            as *const libc::c_char as *mut libc::c_char,
        proxychains_get_version(),
    );
    setup_hooks();
    while close_fds_cnt != 0 {
        close_fds_cnt -= 1;
        true_close.unwrap()(close_fds[close_fds_cnt as usize]);
    }
    while close_range_buffer_cnt != 0 {
        close_range_buffer_cnt -= 1;
        let mut i: libc::c_int = close_range_buffer_cnt;
        true_close_range
            .unwrap()(
            close_range_buffer[i as usize].first,
            close_range_buffer[i as usize].last,
            close_range_buffer[i as usize].flags as libc::c_int,
        );
    }
    init_l = 1 as libc::c_int;
    rdns_init(proxychains_resolver);
}
unsafe extern "C" fn init_lib_wrapper(mut caller: *const libc::c_char) {
    init_l == 0;
    pthread_once(&mut init_once, Some(do_init as unsafe extern "C" fn() -> ()));
}
unsafe extern "C" fn proxy_from_string(
    mut proxystring: *const libc::c_char,
    mut type_buf: *mut libc::c_char,
    mut host_buf: *mut libc::c_char,
    mut port_n: *mut libc::c_int,
    mut user_buf: *mut libc::c_char,
    mut pass_buf: *mut libc::c_char,
) -> libc::c_int {
    let mut at: *const libc::c_char = 0 as *const libc::c_char;
    let mut h: *const libc::c_char = 0 as *const libc::c_char;
    let mut current_block: u64;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut proxytype: rs_proxyType = RS_PT_NONE;
    let mut next_token: size_t = 6 as libc::c_int as size_t;
    let mut ul: size_t = 0 as libc::c_int as size_t;
    let mut pl: size_t = 0 as libc::c_int as size_t;
    let mut hl: size_t = 0;
    if !(*proxystring.offset(0 as libc::c_int as isize) == 0
        || *proxystring.offset(1 as libc::c_int as isize) == 0
        || *proxystring.offset(2 as libc::c_int as isize) == 0
        || *proxystring.offset(3 as libc::c_int as isize) == 0
        || *proxystring.offset(4 as libc::c_int as isize) == 0
        || *proxystring.offset(5 as libc::c_int as isize) == 0)
    {
        if *proxystring as libc::c_int == 's' as i32 {
            match *proxystring.offset(5 as libc::c_int as isize) as libc::c_int {
                53 => {
                    current_block = 16908692114134692612;
                    match current_block {
                        13475044908890067863 => {
                            proxytype = RS_PT_SOCKS4;
                        }
                        _ => {
                            proxytype = RS_PT_SOCKS5;
                        }
                    }
                    current_block = 17965632435239708295;
                }
                52 => {
                    current_block = 13475044908890067863;
                    match current_block {
                        13475044908890067863 => {
                            proxytype = RS_PT_SOCKS4;
                        }
                        _ => {
                            proxytype = RS_PT_SOCKS5;
                        }
                    }
                    current_block = 17965632435239708295;
                }
                _ => {
                    current_block = 1764929708269692711;
                }
            }
        } else if *proxystring as libc::c_int == 'h' as i32 {
            proxytype = RS_PT_HTTP;
            next_token = 4 as libc::c_int as size_t;
            current_block = 17965632435239708295;
        } else {
            current_block = 1764929708269692711;
        }
        match current_block {
            1764929708269692711 => {}
            _ => {
                let fresh0 = next_token;
                next_token = next_token.wrapping_add(1);
                if !(*proxystring.offset(fresh0 as isize) as libc::c_int != ':' as i32
                    || {
                        let fresh1 = next_token;
                        next_token = next_token.wrapping_add(1);
                        *proxystring.offset(fresh1 as isize) as libc::c_int != '/' as i32
                    }
                    || {
                        let fresh2 = next_token;
                        next_token = next_token.wrapping_add(1);
                        *proxystring.offset(fresh2 as isize) as libc::c_int != '/' as i32
                    })
                {
                    at = strrchr(proxystring.offset(next_token as isize), '@' as i32);
                    if !at.is_null() {
                        if proxytype as libc::c_uint
                            == RS_PT_SOCKS4 as libc::c_int as libc::c_uint
                        {
                            return 0 as libc::c_int;
                        }
                        p = strchr(proxystring.offset(next_token as isize), ':' as i32);
                        if p.is_null() || p >= at {
                            current_block = 1764929708269692711;
                        } else {
                            let mut u: *const libc::c_char = proxystring
                                .offset(next_token as isize);
                            ul = p.offset_from(u) as libc::c_long as size_t;
                            p = p.offset(1);
                            p;
                            pl = at.offset_from(p) as libc::c_long as size_t;
                            if proxytype as libc::c_uint
                                == RS_PT_SOCKS5 as libc::c_int as libc::c_uint
                                && (ul > 255 as libc::c_int as libc::c_ulong
                                    || pl > 255 as libc::c_int as libc::c_ulong)
                            {
                                return 0 as libc::c_int;
                            }
                            memcpy(
                                user_buf as *mut libc::c_void,
                                u as *const libc::c_void,
                                ul,
                            );
                            *user_buf
                                .offset(ul as isize) = 0 as libc::c_int as libc::c_char;
                            memcpy(
                                pass_buf as *mut libc::c_void,
                                p as *const libc::c_void,
                                pl,
                            );
                            *pass_buf
                                .offset(pl as isize) = 0 as libc::c_int as libc::c_char;
                            next_token = (next_token as libc::c_ulong)
                                .wrapping_add(
                                    (2 as libc::c_int as libc::c_ulong)
                                        .wrapping_add(ul)
                                        .wrapping_add(pl),
                                ) as size_t as size_t;
                            current_block = 18386322304582297246;
                        }
                    } else {
                        *user_buf
                            .offset(
                                0 as libc::c_int as isize,
                            ) = 0 as libc::c_int as libc::c_char;
                        *pass_buf
                            .offset(
                                0 as libc::c_int as isize,
                            ) = 0 as libc::c_int as libc::c_char;
                        current_block = 18386322304582297246;
                    }
                    match current_block {
                        1764929708269692711 => {}
                        _ => {
                            h = proxystring.offset(next_token as isize);
                            p = strchr(h, ':' as i32);
                            if !p.is_null() {
                                hl = p.offset_from(h) as libc::c_long as size_t;
                                if hl > 255 as libc::c_int as libc::c_ulong {
                                    return 0 as libc::c_int;
                                }
                                memcpy(
                                    host_buf as *mut libc::c_void,
                                    h as *const libc::c_void,
                                    hl,
                                );
                                *host_buf
                                    .offset(hl as isize) = 0 as libc::c_int as libc::c_char;
                                *port_n = atoi(p.offset(1 as libc::c_int as isize));
                                match proxytype as libc::c_uint {
                                    1 => {
                                        strcpy(
                                            type_buf,
                                            b"socks4\0" as *const u8 as *const libc::c_char,
                                        );
                                    }
                                    2 => {
                                        strcpy(
                                            type_buf,
                                            b"socks5\0" as *const u8 as *const libc::c_char,
                                        );
                                    }
                                    3 => {
                                        strcpy(
                                            type_buf,
                                            b"http\0" as *const u8 as *const libc::c_char,
                                        );
                                    }
                                    _ => return 0 as libc::c_int,
                                }
                                return 1 as libc::c_int;
                            }
                        }
                    }
                }
            }
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn bool_str(mut bool_val: libc::c_int) -> *const libc::c_char {
    if bool_val != 0 {
        return b"true\0" as *const u8 as *const libc::c_char;
    }
    return b"false\0" as *const u8 as *const libc::c_char;
}
unsafe extern "C" fn get_chain_data(
    mut pd: *mut proxy_data,
    mut proxy_count: *mut libc::c_uint,
    mut ct: *mut chain_type,
) {
    let mut count: libc::c_int = 0 as libc::c_int;
    let mut port_n: libc::c_int = 0 as libc::c_int;
    let mut list: libc::c_int = 0 as libc::c_int;
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut type_0: [libc::c_char; 1024] = [0; 1024];
    let mut host: [libc::c_char; 1024] = [0; 1024];
    let mut user: [libc::c_char; 1024] = [0; 1024];
    let mut buff: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut env: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut local_addr_port: [libc::c_char; 64] = [0; 64];
    let mut local_addr: [libc::c_char; 64] = [0; 64];
    let mut local_netmask: [libc::c_char; 32] = [0; 32];
    let mut dnat_orig_addr_port: [libc::c_char; 32] = [0; 32];
    let mut dnat_new_addr_port: [libc::c_char; 32] = [0; 32];
    let mut dnat_orig_addr: [libc::c_char; 32] = [0; 32];
    let mut dnat_orig_port: [libc::c_char; 32] = [0; 32];
    let mut dnat_new_addr: [libc::c_char; 32] = [0; 32];
    let mut dnat_new_port: [libc::c_char; 32] = [0; 32];
    let mut rdnsd_addr: [libc::c_char; 32] = [0; 32];
    let mut rdnsd_port: [libc::c_char; 8] = [0; 8];
    let mut file: *mut FILE = 0 as *mut FILE;
    if proxychains_got_chain_data != 0 {
        return;
    }
    tcp_read_time_out = 4 as libc::c_int * 1000 as libc::c_int;
    tcp_connect_time_out = 10 as libc::c_int * 1000 as libc::c_int;
    *ct = DYNAMIC_TYPE;
    env = get_config_path(
        getenv(b"PROXYCHAINS_CONF_FILE\0" as *const u8 as *const libc::c_char),
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
    );
    file = fopen(env, b"r\0" as *const u8 as *const libc::c_char);
    if file.is_null() {
        perror(b"couldnt read configuration file\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    env = getenv(b"PROXYCHAINS_QUIET_MODE\0" as *const u8 as *const libc::c_char);
    if !env.is_null() && *env as libc::c_int == '1' as i32 {
        proxychains_quiet_mode = 1 as libc::c_int;
    }
    let mut current_block_218: u64;
    while !(fgets(
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
        file,
    ))
        .is_null()
    {
        buff = buf.as_mut_ptr();
        while *(*__ctype_b_loc()).offset(*buff as libc::c_int as isize) as libc::c_int
            & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
        {
            buff = buff.offset(1);
            buff;
        }
        p = strrchr(buff, '\n' as i32);
        if !p.is_null() {
            *p = 0 as libc::c_int as libc::c_char;
        }
        p = buff.offset(strlen(buff) as isize).offset(-(1 as libc::c_int as isize));
        while p >= buff
            && *(*__ctype_b_loc()).offset(*p as libc::c_int as isize) as libc::c_int
                & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
        {
            let fresh3 = p;
            p = p.offset(-1);
            *fresh3 = 0 as libc::c_int as libc::c_char;
        }
        if *buff == 0 || *buff as libc::c_int == '#' as i32 {
            continue;
        }
        if list != 0 {
            if count >= 512 as libc::c_int {
                break;
            }
            memset(
                &mut *pd.offset(count as isize) as *mut proxy_data as *mut libc::c_void,
                0 as libc::c_int,
                ::std::mem::size_of::<proxy_data>() as libc::c_ulong,
            );
            (*pd.offset(count as isize)).ps = PLAY_STATE;
            port_n = 0 as libc::c_int;
            let mut ret: libc::c_int = sscanf(
                buff,
                b"%s %s %d %s %s\0" as *const u8 as *const libc::c_char,
                type_0.as_mut_ptr(),
                host.as_mut_ptr(),
                &mut port_n as *mut libc::c_int,
                ((*pd.offset(count as isize)).user).as_mut_ptr(),
                ((*pd.offset(count as isize)).pass).as_mut_ptr(),
            );
            if ret < 3 as libc::c_int || ret == -(1 as libc::c_int) {
                if proxy_from_string(
                    buff,
                    type_0.as_mut_ptr(),
                    host.as_mut_ptr(),
                    &mut port_n,
                    ((*pd.offset(count as isize)).user).as_mut_ptr(),
                    ((*pd.offset(count as isize)).pass).as_mut_ptr(),
                ) == 0
                {
                    current_block_218 = 15018078534797999522;
                } else {
                    current_block_218 = 3275366147856559585;
                }
            } else {
                current_block_218 = 3275366147856559585;
            }
            match current_block_218 {
                3275366147856559585 => {
                    memset(
                        &mut (*pd.offset(count as isize)).ip as *mut ip_type
                            as *mut libc::c_void,
                        0 as libc::c_int,
                        ::std::mem::size_of::<ip_type>() as libc::c_ulong,
                    );
                    (*pd.offset(count as isize))
                        .ip
                        .is_v6 = !(strchr(host.as_mut_ptr(), ':' as i32)).is_null()
                        as libc::c_int as libc::c_char;
                    (*pd.offset(count as isize)).port = htons(port_n as libc::c_ushort);
                    let mut host_ip: *mut ip_type = &mut (*pd.offset(count as isize)).ip;
                    if 1 as libc::c_int
                        != inet_pton(
                            (if (*host_ip).is_v6 as libc::c_int != 0 {
                                10 as libc::c_int
                            } else {
                                2 as libc::c_int
                            }),
                            host.as_mut_ptr(),
                            ((*host_ip).addr.v6).as_mut_ptr() as *mut libc::c_void,
                        )
                    {
                        's_216: {
                            if *ct as libc::c_uint
                                == STRICT_TYPE as libc::c_int as libc::c_uint
                                && proxychains_resolver as libc::c_uint
                                    >= DNSLF_RDNS_START as libc::c_int as libc::c_uint
                                && count > 0 as libc::c_int
                            {
                                rdns_init(proxychains_resolver);
                                let mut internal_ip: ip_type4 = at_get_ip_for_host(
                                    host.as_mut_ptr(),
                                    strlen(host.as_mut_ptr()),
                                );
                                (*pd.offset(count as isize))
                                    .ip
                                    .is_v6 = 0 as libc::c_int as libc::c_char;
                                (*host_ip).addr.v4 = internal_ip;
                                if !(internal_ip.as_int
                                    == ip_type4 {
                                        as_int: -(1 as libc::c_int) as uint32_t,
                                    }
                                        .as_int)
                                {
                                    break 's_216;
                                }
                            }
                            fprintf(
                                stderr,
                                b"proxy %s has invalid value or is not numeric\n\0"
                                    as *const u8 as *const libc::c_char,
                                host.as_mut_ptr(),
                            );
                            fprintf(
                                stderr,
                                b"non-numeric ips are only allowed under the following circumstances:\n\0"
                                    as *const u8 as *const libc::c_char,
                            );
                            fprintf(
                                stderr,
                                b"chaintype == strict (%s), proxy is not first in list (%s), proxy_dns active (%s)\n\n\0"
                                    as *const u8 as *const libc::c_char,
                                bool_str(
                                    (*ct as libc::c_uint
                                        == STRICT_TYPE as libc::c_int as libc::c_uint)
                                        as libc::c_int,
                                ),
                                bool_str((count > 0 as libc::c_int) as libc::c_int),
                                rdns_resolver_string(proxychains_resolver),
                            );
                            exit(1 as libc::c_int);
                        }
                    }
                    if strcmp(
                        type_0.as_mut_ptr(),
                        b"http\0" as *const u8 as *const libc::c_char,
                    ) == 0
                    {
                        (*pd.offset(count as isize)).pt = HTTP_TYPE;
                        current_block_218 = 7018308795614528254;
                    } else if strcmp(
                        type_0.as_mut_ptr(),
                        b"raw\0" as *const u8 as *const libc::c_char,
                    ) == 0
                    {
                        (*pd.offset(count as isize)).pt = RAW_TYPE;
                        current_block_218 = 7018308795614528254;
                    } else if strcmp(
                        type_0.as_mut_ptr(),
                        b"socks4\0" as *const u8 as *const libc::c_char,
                    ) == 0
                    {
                        (*pd.offset(count as isize)).pt = SOCKS4_TYPE;
                        current_block_218 = 7018308795614528254;
                    } else if strcmp(
                        type_0.as_mut_ptr(),
                        b"socks5\0" as *const u8 as *const libc::c_char,
                    ) == 0
                    {
                        (*pd.offset(count as isize)).pt = SOCKS5_TYPE;
                        current_block_218 = 7018308795614528254;
                    } else {
                        current_block_218 = 15018078534797999522;
                    }
                    match current_block_218 {
                        15018078534797999522 => {}
                        _ => {
                            if port_n != 0 {
                                count += 1;
                                count;
                            }
                            continue;
                        }
                    }
                }
                _ => {}
            }
            fprintf(
                stderr,
                b"error: invalid item in proxylist section: %s\0" as *const u8
                    as *const libc::c_char,
                buff,
            );
            exit(1 as libc::c_int);
        } else if strcmp(buff, b"[ProxyList]\0" as *const u8 as *const libc::c_char) == 0
        {
            list = 1 as libc::c_int;
        } else if strcmp(buff, b"random_chain\0" as *const u8 as *const libc::c_char)
            == 0
        {
            *ct = RANDOM_TYPE;
        } else if strcmp(buff, b"strict_chain\0" as *const u8 as *const libc::c_char)
            == 0
        {
            *ct = STRICT_TYPE;
        } else if strcmp(buff, b"dynamic_chain\0" as *const u8 as *const libc::c_char)
            == 0
        {
            *ct = DYNAMIC_TYPE;
        } else if strcmp(
            buff,
            b"round_robin_chain\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            *ct = ROUND_ROBIN_TYPE;
        } else if strncmp(
            buff,
            b"tcp_read_time_out\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 18]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ) == 0
        {
            sscanf(
                buff,
                b"%s %d\0" as *const u8 as *const libc::c_char,
                user.as_mut_ptr(),
                &mut tcp_read_time_out as *mut libc::c_int,
            );
        } else if strncmp(
            buff,
            b"tcp_connect_time_out\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 21]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ) == 0
        {
            sscanf(
                buff,
                b"%s %d\0" as *const u8 as *const libc::c_char,
                user.as_mut_ptr(),
                &mut tcp_connect_time_out as *mut libc::c_int,
            );
        } else if strncmp(
            buff,
            b"remote_dns_subnet\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 18]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ) == 0
        {
            sscanf(
                buff,
                b"%s %u\0" as *const u8 as *const libc::c_char,
                user.as_mut_ptr(),
                &mut remote_dns_subnet as *mut libc::c_uint,
            );
            if remote_dns_subnet >= 256 as libc::c_int as libc::c_uint {
                fprintf(
                    stderr,
                    b"remote_dns_subnet: invalid value. requires a number between 0 and 255.\n\0"
                        as *const u8 as *const libc::c_char,
                );
                exit(1 as libc::c_int);
            }
        } else if strncmp(
            buff,
            b"localnet\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ) == 0
        {
            let mut colon: libc::c_char = 0;
            let mut extra: libc::c_char = 0;
            let mut right_bracket: [libc::c_char; 2] = [0; 2];
            let mut local_port: libc::c_ushort = 0 as libc::c_int as libc::c_ushort;
            let mut local_prefix: libc::c_ushort = 0;
            let mut local_family: libc::c_int = 0;
            let mut n: libc::c_int = 0;
            let mut valid: libc::c_int = 0;
            if sscanf(
                buff,
                b"%s %53[^/]/%15s%c\0" as *const u8 as *const libc::c_char,
                user.as_mut_ptr(),
                local_addr_port.as_mut_ptr(),
                local_netmask.as_mut_ptr(),
                &mut extra as *mut libc::c_char,
            ) != 3 as libc::c_int
            {
                fprintf(
                    stderr,
                    b"localnet format error\0" as *const u8 as *const libc::c_char,
                );
                exit(1 as libc::c_int);
            }
            p = strchr(local_addr_port.as_mut_ptr(), ':' as i32);
            if p.is_null() || p == strrchr(local_addr_port.as_mut_ptr(), ':' as i32) {
                local_family = 2 as libc::c_int;
                n = sscanf(
                    local_addr_port.as_mut_ptr(),
                    b"%15[^:]%c%5hu%c\0" as *const u8 as *const libc::c_char,
                    local_addr.as_mut_ptr(),
                    &mut colon as *mut libc::c_char,
                    &mut local_port as *mut libc::c_ushort,
                    &mut extra as *mut libc::c_char,
                );
                valid = (n == 1 as libc::c_int
                    || n == 3 as libc::c_int && colon as libc::c_int == ':' as i32)
                    as libc::c_int;
            } else if local_addr_port[0 as libc::c_int as usize] as libc::c_int
                == '[' as i32
            {
                local_family = 10 as libc::c_int;
                n = sscanf(
                    local_addr_port.as_mut_ptr(),
                    b"[%45[^][]%1[]]%c%5hu%c\0" as *const u8 as *const libc::c_char,
                    local_addr.as_mut_ptr(),
                    right_bracket.as_mut_ptr(),
                    &mut colon as *mut libc::c_char,
                    &mut local_port as *mut libc::c_ushort,
                    &mut extra as *mut libc::c_char,
                );
                valid = (n == 2 as libc::c_int
                    || n == 4 as libc::c_int && colon as libc::c_int == ':' as i32)
                    as libc::c_int;
            } else {
                local_family = 10 as libc::c_int;
                valid = (sscanf(
                    local_addr_port.as_mut_ptr(),
                    b"%45[^][]%c\0" as *const u8 as *const libc::c_char,
                    local_addr.as_mut_ptr(),
                    &mut extra as *mut libc::c_char,
                ) == 1 as libc::c_int) as libc::c_int;
            }
            if valid == 0 {
                fprintf(
                    stderr,
                    b"localnet address or port error\n\0" as *const u8
                        as *const libc::c_char,
                );
                exit(1 as libc::c_int);
            }
            local_port != 0;
            if num_localnet_addr < 64 as libc::c_int as libc::c_ulong {
                localnet_addr[num_localnet_addr as usize]
                    .family = local_family as sa_family_t;
                localnet_addr[num_localnet_addr as usize].port = local_port;
                valid = 0 as libc::c_int;
                if local_family == 2 as libc::c_int {
                    valid = (inet_pton(
                        local_family,
                        local_addr.as_mut_ptr(),
                        &mut (*localnet_addr
                            .as_mut_ptr()
                            .offset(num_localnet_addr as isize))
                            .c2rust_unnamed
                            .c2rust_unnamed
                            .in_addr as *mut in_addr as *mut libc::c_void,
                    ) > 0 as libc::c_int) as libc::c_int;
                } else if local_family == 10 as libc::c_int {
                    valid = (inet_pton(
                        local_family,
                        local_addr.as_mut_ptr(),
                        &mut (*localnet_addr
                            .as_mut_ptr()
                            .offset(num_localnet_addr as isize))
                            .c2rust_unnamed
                            .c2rust_unnamed_0
                            .in6_addr as *mut in6_addr as *mut libc::c_void,
                    ) > 0 as libc::c_int) as libc::c_int;
                }
                if valid == 0 {
                    fprintf(
                        stderr,
                        b"localnet address error\n\0" as *const u8 as *const libc::c_char,
                    );
                    exit(1 as libc::c_int);
                }
                if local_family == 2 as libc::c_int
                    && !(strchr(local_netmask.as_mut_ptr(), '.' as i32)).is_null()
                {
                    valid = (inet_pton(
                        local_family,
                        local_netmask.as_mut_ptr(),
                        &mut (*localnet_addr
                            .as_mut_ptr()
                            .offset(num_localnet_addr as isize))
                            .c2rust_unnamed
                            .c2rust_unnamed
                            .in_mask as *mut in_addr as *mut libc::c_void,
                    ) > 0 as libc::c_int) as libc::c_int;
                } else {
                    valid = (sscanf(
                        local_netmask.as_mut_ptr(),
                        b"%hu%c\0" as *const u8 as *const libc::c_char,
                        &mut local_prefix as *mut libc::c_ushort,
                        &mut extra as *mut libc::c_char,
                    ) == 1 as libc::c_int) as libc::c_int;
                    if valid != 0 {
                        if local_family == 2 as libc::c_int
                            && local_prefix as libc::c_int <= 32 as libc::c_int
                        {
                            localnet_addr[num_localnet_addr as usize]
                                .c2rust_unnamed
                                .c2rust_unnamed
                                .in_mask
                                .s_addr = htonl(
                                (0xffffffff as libc::c_uint)
                                    << (32 as libc::c_uint)
                                        .wrapping_sub(local_prefix as libc::c_uint),
                            );
                        } else if local_family == 10 as libc::c_int
                            && local_prefix as libc::c_int <= 128 as libc::c_int
                        {
                            localnet_addr[num_localnet_addr as usize]
                                .c2rust_unnamed
                                .c2rust_unnamed_0
                                .in6_prefix = local_prefix as libc::c_uchar;
                        } else {
                            valid = 0 as libc::c_int;
                        }
                    }
                }
                if valid == 0 {
                    fprintf(
                        stderr,
                        b"localnet netmask error\n\0" as *const u8 as *const libc::c_char,
                    );
                    exit(1 as libc::c_int);
                }
                num_localnet_addr = num_localnet_addr.wrapping_add(1);
                num_localnet_addr;
            } else {
                fprintf(
                    stderr,
                    b"# of localnet exceed %d.\n\0" as *const u8 as *const libc::c_char,
                    64 as libc::c_int,
                );
            }
        } else if strncmp(
            buff,
            b"chain_len\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ) == 0
        {
            let mut pc: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut len: libc::c_int = 0;
            pc = strchr(buff, '=' as i32);
            if pc.is_null() {
                fprintf(
                    stderr,
                    b"error: missing equals sign '=' in chain_len directive.\n\0"
                        as *const u8 as *const libc::c_char,
                );
                exit(1 as libc::c_int);
            }
            pc = pc.offset(1);
            len = atoi(pc);
            proxychains_max_chain = (if len != 0 { len } else { 1 as libc::c_int })
                as libc::c_uint;
        } else if strcmp(buff, b"quiet_mode\0" as *const u8 as *const libc::c_char) == 0
        {
            proxychains_quiet_mode = 1 as libc::c_int;
        } else if strcmp(buff, b"proxy_dns_old\0" as *const u8 as *const libc::c_char)
            == 0
        {
            proxychains_resolver = DNSLF_FORKEXEC;
        } else if strcmp(buff, b"proxy_dns\0" as *const u8 as *const libc::c_char) == 0 {
            proxychains_resolver = DNSLF_RDNS_THREAD;
        } else if strncmp(
            buff,
            b"proxy_dns_daemon\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 17]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ) == 0
        {
            let mut rdns_server_buffer: sockaddr_in = sockaddr_in {
                sin_family: 0,
                sin_port: 0,
                sin_addr: in_addr { s_addr: 0 },
                sin_zero: [0; 8],
            };
            if sscanf(
                buff,
                b"%s %15[^:]:%5s\0" as *const u8 as *const libc::c_char,
                user.as_mut_ptr(),
                rdnsd_addr.as_mut_ptr(),
                rdnsd_port.as_mut_ptr(),
            ) < 3 as libc::c_int
            {
                fprintf(
                    stderr,
                    b"proxy_dns_daemon format error\n\0" as *const u8
                        as *const libc::c_char,
                );
                exit(1 as libc::c_int);
            }
            rdns_server_buffer.sin_family = 2 as libc::c_int as sa_family_t;
            let mut error: libc::c_int = inet_pton(
                2 as libc::c_int,
                rdnsd_addr.as_mut_ptr(),
                &mut rdns_server_buffer.sin_addr as *mut in_addr as *mut libc::c_void,
            );
            if error <= 0 as libc::c_int {
                fprintf(
                    stderr,
                    b"bogus proxy_dns_daemon address\n\0" as *const u8
                        as *const libc::c_char,
                );
                exit(1 as libc::c_int);
            }
            rdns_server_buffer
                .sin_port = htons(atoi(rdnsd_port.as_mut_ptr()) as uint16_t);
            proxychains_resolver = DNSLF_RDNS_DAEMON;
            rdns_set_daemon(&mut rdns_server_buffer);
        } else if strncmp(
            buff,
            b"dnat\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ) == 0
        {
            if sscanf(
                buff,
                b"%s %21[^ ] %21s\n\0" as *const u8 as *const libc::c_char,
                user.as_mut_ptr(),
                dnat_orig_addr_port.as_mut_ptr(),
                dnat_new_addr_port.as_mut_ptr(),
            ) < 3 as libc::c_int
            {
                fprintf(
                    stderr,
                    b"dnat format error\0" as *const u8 as *const libc::c_char,
                );
                exit(1 as libc::c_int);
            }
            memset(
                dnat_orig_port.as_mut_ptr() as *mut libc::c_void,
                0 as libc::c_int,
                (::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong)
                    .wrapping_div(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
            );
            memset(
                dnat_new_port.as_mut_ptr() as *mut libc::c_void,
                0 as libc::c_int,
                (::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong)
                    .wrapping_div(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
            );
            sscanf(
                dnat_orig_addr_port.as_mut_ptr(),
                b"%15[^:]:%5s\0" as *const u8 as *const libc::c_char,
                dnat_orig_addr.as_mut_ptr(),
                dnat_orig_port.as_mut_ptr(),
            );
            sscanf(
                dnat_new_addr_port.as_mut_ptr(),
                b"%15[^:]:%5s\0" as *const u8 as *const libc::c_char,
                dnat_new_addr.as_mut_ptr(),
                dnat_new_port.as_mut_ptr(),
            );
            if num_dnats < 64 as libc::c_int as libc::c_ulong {
                let mut error_0: libc::c_int = 0;
                error_0 = inet_pton(
                    2 as libc::c_int,
                    dnat_orig_addr.as_mut_ptr(),
                    &mut (*dnats.as_mut_ptr().offset(num_dnats as isize)).orig_dst
                        as *mut in_addr as *mut libc::c_void,
                );
                if error_0 <= 0 as libc::c_int {
                    fprintf(
                        stderr,
                        b"dnat original destination address error\n\0" as *const u8
                            as *const libc::c_char,
                    );
                    exit(1 as libc::c_int);
                }
                error_0 = inet_pton(
                    2 as libc::c_int,
                    dnat_new_addr.as_mut_ptr(),
                    &mut (*dnats.as_mut_ptr().offset(num_dnats as isize)).new_dst
                        as *mut in_addr as *mut libc::c_void,
                );
                if error_0 <= 0 as libc::c_int {
                    fprintf(
                        stderr,
                        b"dnat effective destination address error\n\0" as *const u8
                            as *const libc::c_char,
                    );
                    exit(1 as libc::c_int);
                }
                if dnat_orig_port[0 as libc::c_int as usize] != 0 {
                    dnats[num_dnats as usize]
                        .orig_port = atoi(dnat_orig_port.as_mut_ptr()) as libc::c_short
                        as libc::c_ushort;
                } else {
                    dnats[num_dnats as usize]
                        .orig_port = 0 as libc::c_int as libc::c_ushort;
                }
                if dnat_new_port[0 as libc::c_int as usize] != 0 {
                    dnats[num_dnats as usize]
                        .new_port = atoi(dnat_new_port.as_mut_ptr()) as libc::c_short
                        as libc::c_ushort;
                } else {
                    dnats[num_dnats as usize]
                        .new_port = 0 as libc::c_int as libc::c_ushort;
                }
                num_dnats = num_dnats.wrapping_add(1);
                num_dnats;
            } else {
                fprintf(
                    stderr,
                    b"# of dnat exceed %d.\n\0" as *const u8 as *const libc::c_char,
                    64 as libc::c_int,
                );
            }
        }
    }
    fclose(file);
    if count == 0 {
        fprintf(
            stderr,
            b"error: no valid proxy found in config\n\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    *proxy_count = count as libc::c_uint;
    proxychains_got_chain_data = 1 as libc::c_int;
}
pub unsafe extern "C" fn close(mut fd: libc::c_int) -> libc::c_int {
    if init_l == 0 {
        if !(close_fds_cnt as libc::c_ulong
            >= (::std::mem::size_of::<[libc::c_int; 16]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<libc::c_int>() as libc::c_ulong))
        {
            let fresh4 = close_fds_cnt;
            close_fds_cnt = close_fds_cnt + 1;
            close_fds[fresh4 as usize] = fd;
            *__errno_location() = 0 as libc::c_int;
            return 0 as libc::c_int;
        }
    } else {
        if proxychains_resolver as libc::c_uint
            != DNSLF_RDNS_THREAD as libc::c_int as libc::c_uint
        {
            return true_close.unwrap()(fd);
        }
        if fd != req_pipefd[0 as libc::c_int as usize]
            && fd != req_pipefd[1 as libc::c_int as usize]
            && fd != resp_pipefd[0 as libc::c_int as usize]
            && fd != resp_pipefd[1 as libc::c_int as usize]
        {
            return true_close.unwrap()(fd);
        }
    }
    *__errno_location() = 9 as libc::c_int;
    return -(1 as libc::c_int);
}
unsafe extern "C" fn is_v4inv6(mut a: *const in6_addr) -> libc::c_int {
    return (memcmp(
        ((*a).__in6_u.__u6_addr8).as_ptr() as *const libc::c_void,
        b"\0\0\0\0\0\0\0\0\0\0\xFF\xFF\0" as *const u8 as *const libc::c_char
            as *const libc::c_void,
        12 as libc::c_int as libc::c_ulong,
    ) == 0) as libc::c_int;
}
unsafe extern "C" fn intsort(mut a: *mut libc::c_int, mut n: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut s: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < n {
        j = i + 1 as libc::c_int;
        while j < n {
            if *a.offset(j as isize) < *a.offset(i as isize) {
                s = *a.offset(i as isize);
                *a.offset(i as isize) = *a.offset(j as isize);
                *a.offset(j as isize) = s;
            }
            j += 1;
            j;
        }
        i += 1;
        i;
    }
}
pub unsafe extern "C" fn close_range(
    mut first: libc::c_uint,
    mut last: libc::c_uint,
    mut flags: libc::c_int,
) -> libc::c_int {
    if true_close_range.is_none() {
        fprintf(
            stderr,
            b"Calling close_range, but this platform does not provide this system call. \0"
                as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if init_l == 0 {
        if close_range_buffer_cnt as libc::c_ulong
            >= (::std::mem::size_of::<[close_range_args_t; 16]>() as libc::c_ulong)
                .wrapping_div(
                    ::std::mem::size_of::<close_range_args_t>() as libc::c_ulong,
                )
        {
            *__errno_location() = 12 as libc::c_int;
            return -(1 as libc::c_int);
        }
        let fresh5 = close_range_buffer_cnt;
        close_range_buffer_cnt = close_range_buffer_cnt + 1;
        let mut i: libc::c_int = fresh5;
        close_range_buffer[i as usize].first = first;
        close_range_buffer[i as usize].last = last;
        close_range_buffer[i as usize].flags = flags as libc::c_uint;
        let ref mut fresh6 = *__errno_location();
        *fresh6 = 0 as libc::c_int;
        return *fresh6;
    }
    if proxychains_resolver as libc::c_uint
        != DNSLF_RDNS_THREAD as libc::c_int as libc::c_uint
    {
        return true_close_range.unwrap()(first, last, flags);
    }
    let mut res: libc::c_int = 0 as libc::c_int;
    let mut uerrno: libc::c_int = 0 as libc::c_int;
    let mut i_0: libc::c_int = 0;
    let mut protected_fds: [libc::c_int; 4] = [
        req_pipefd[0 as libc::c_int as usize],
        req_pipefd[1 as libc::c_int as usize],
        resp_pipefd[0 as libc::c_int as usize],
        resp_pipefd[1 as libc::c_int as usize],
    ];
    intsort(protected_fds.as_mut_ptr(), 4 as libc::c_int);
    let mut next_fd_to_close: libc::c_int = first as libc::c_int;
    i_0 = 0 as libc::c_int;
    while i_0 < 4 as libc::c_int {
        if !((protected_fds[i_0 as usize] as libc::c_uint) < first
            || protected_fds[i_0 as usize] as libc::c_uint > last)
        {
            let mut prev: libc::c_int = (if i_0 == 0 as libc::c_int
                || (protected_fds[(i_0 - 1 as libc::c_int) as usize] as libc::c_uint)
                    < first
            {
                first
            } else {
                (protected_fds[(i_0 - 1 as libc::c_int) as usize] + 1 as libc::c_int)
                    as libc::c_uint
            }) as libc::c_int;
            if prev != protected_fds[i_0 as usize] {
                if -(1 as libc::c_int)
                    == true_close_range
                        .unwrap()(
                        prev as libc::c_uint,
                        (protected_fds[i_0 as usize] - 1 as libc::c_int) as libc::c_uint,
                        flags,
                    )
                {
                    res = -(1 as libc::c_int);
                    uerrno = *__errno_location();
                }
            }
            next_fd_to_close = protected_fds[i_0 as usize] + 1 as libc::c_int;
        }
        i_0 += 1;
        i_0;
    }
    if next_fd_to_close as libc::c_uint <= last {
        if -(1 as libc::c_int)
            == true_close_range.unwrap()(next_fd_to_close as libc::c_uint, last, flags)
        {
            res = -(1 as libc::c_int);
            uerrno = *__errno_location();
        }
    }
    *__errno_location() = uerrno;
    return res;
}
pub unsafe extern "C" fn connect(
    mut sock: libc::c_int,
    mut addr: *const sockaddr,
    mut len: libc::c_uint,
) -> libc::c_int {
    init_lib_wrapper(
        (*::std::mem::transmute::<&[u8; 8], &[libc::c_char; 8]>(b"connect\0")).as_ptr(),
    );
    let mut socktype: libc::c_int = 0 as libc::c_int;
    let mut flags: libc::c_int = 0 as libc::c_int;
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut optlen: socklen_t = 0 as libc::c_int as socklen_t;
    let mut dest_ip: ip_type = ip_type {
        addr: C2RustUnnamed_2 {
            v4: ip_type4 { octet: [0; 4] },
        },
        is_v6: 0,
    };
    let mut p_addr_in: *mut in_addr = 0 as *mut in_addr;
    let mut p_addr_in6: *mut in6_addr = 0 as *mut in6_addr;
    let mut dnat: *mut dnat_arg = 0 as *mut dnat_arg;
    let mut port: libc::c_ushort = 0;
    let mut i: size_t = 0;
    let mut remote_dns_connect: libc::c_int = 0 as libc::c_int;
    optlen = ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t;
    let mut fam: sa_family_t = (*(addr as *mut sockaddr_in)).sin_family;
    getsockopt(
        sock,
        1 as libc::c_int,
        3 as libc::c_int,
        &mut socktype as *mut libc::c_int as *mut libc::c_void,
        &mut optlen,
    );
    if !((fam as libc::c_int == 2 as libc::c_int
        || fam as libc::c_int == 10 as libc::c_int)
        && socktype == SOCK_STREAM as libc::c_int)
    {
        return true_connect.unwrap()(sock, addr, len);
    }
    dest_ip
        .is_v6 = (fam as libc::c_int == 10 as libc::c_int) as libc::c_int
        as libc::c_char;
    let mut v6: libc::c_int = dest_ip.is_v6 as libc::c_int;
    p_addr_in = &mut (*(addr as *mut sockaddr_in)).sin_addr;
    p_addr_in6 = &mut (*(addr as *mut sockaddr_in6)).sin6_addr;
    port = (if v6 == 0 {
        ntohs((*(addr as *mut sockaddr_in)).sin_port) as libc::c_int
    } else {
        ntohs((*(addr as *mut sockaddr_in6)).sin6_port) as libc::c_int
    }) as libc::c_ushort;
    let mut v4inv6: in_addr = in_addr { s_addr: 0 };
    if v6 != 0 && is_v4inv6(p_addr_in6) != 0 {
        memcpy(
            &mut v4inv6.s_addr as *mut in_addr_t as *mut libc::c_void,
            &mut *((*p_addr_in6).__in6_u.__u6_addr8)
                .as_mut_ptr()
                .offset(12 as libc::c_int as isize) as *mut uint8_t
                as *const libc::c_void,
            4 as libc::c_int as libc::c_ulong,
        );
        dest_ip.is_v6 = 0 as libc::c_int as libc::c_char;
        v6 = dest_ip.is_v6 as libc::c_int;
        p_addr_in = &mut v4inv6;
    }
    if v6 == 0
        && memcmp(
            p_addr_in as *const libc::c_void,
            b"\0\0\0\0\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            4 as libc::c_int as libc::c_ulong,
        ) == 0
    {
        *__errno_location() = 111 as libc::c_int;
        return -(1 as libc::c_int);
    }
    remote_dns_connect = (v6 == 0
        && ntohl((*p_addr_in).s_addr) >> 24 as libc::c_int == remote_dns_subnet)
        as libc::c_int;
    if v6 == 0 {
        i = 0 as libc::c_int as size_t;
        while i < num_dnats && remote_dns_connect == 0 && dnat.is_null() {
            if dnats[i as usize].orig_dst.s_addr == (*p_addr_in).s_addr {
                if dnats[i as usize].orig_port as libc::c_int != 0
                    && dnats[i as usize].orig_port as libc::c_int == port as libc::c_int
                {
                    dnat = &mut *dnats.as_mut_ptr().offset(i as isize) as *mut dnat_arg;
                }
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    if v6 == 0 {
        i = 0 as libc::c_int as size_t;
        while i < num_dnats && remote_dns_connect == 0 && dnat.is_null() {
            if dnats[i as usize].orig_dst.s_addr == (*p_addr_in).s_addr {
                if dnats[i as usize].orig_port == 0 {
                    dnat = &mut *dnats.as_mut_ptr().offset(i as isize) as *mut dnat_arg;
                }
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    if !dnat.is_null() {
        p_addr_in = &mut (*dnat).new_dst;
        if (*dnat).new_port != 0 {
            port = (*dnat).new_port;
        }
    }
    let mut current_block_47: u64;
    i = 0 as libc::c_int as size_t;
    while i < num_localnet_addr && remote_dns_connect == 0 {
        if !(localnet_addr[i as usize].port as libc::c_int != 0
            && localnet_addr[i as usize].port as libc::c_int != port as libc::c_int)
        {
            if !(localnet_addr[i as usize].family as libc::c_int
                != (if v6 != 0 { 10 as libc::c_int } else { 2 as libc::c_int }))
            {
                if v6 != 0 {
                    let mut prefix_bytes: size_t = (localnet_addr[i as usize]
                        .c2rust_unnamed
                        .c2rust_unnamed_0
                        .in6_prefix as libc::c_int / 8 as libc::c_int) as size_t;
                    let mut prefix_bits: size_t = (localnet_addr[i as usize]
                        .c2rust_unnamed
                        .c2rust_unnamed_0
                        .in6_prefix as libc::c_int % 8 as libc::c_int) as size_t;
                    if prefix_bytes != 0
                        && memcmp(
                            ((*p_addr_in6).__in6_u.__u6_addr8).as_mut_ptr()
                                as *const libc::c_void,
                            (localnet_addr[i as usize]
                                .c2rust_unnamed
                                .c2rust_unnamed_0
                                .in6_addr
                                .__in6_u
                                .__u6_addr8)
                                .as_mut_ptr() as *const libc::c_void,
                            prefix_bytes,
                        ) != 0 as libc::c_int
                    {
                        current_block_47 = 17784502470059252271;
                    } else if prefix_bits != 0
                        && ((*p_addr_in6).__in6_u.__u6_addr8[prefix_bytes as usize]
                            as libc::c_int
                            ^ localnet_addr[i as usize]
                                .c2rust_unnamed
                                .c2rust_unnamed_0
                                .in6_addr
                                .__in6_u
                                .__u6_addr8[prefix_bytes as usize] as libc::c_int)
                            >> (8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(prefix_bits) != 0
                    {
                        current_block_47 = 17784502470059252271;
                    } else {
                        current_block_47 = 2516253395664191498;
                    }
                } else if ((*p_addr_in).s_addr
                    ^ localnet_addr[i as usize]
                        .c2rust_unnamed
                        .c2rust_unnamed
                        .in_addr
                        .s_addr)
                    & localnet_addr[i as usize]
                        .c2rust_unnamed
                        .c2rust_unnamed
                        .in_mask
                        .s_addr != 0
                {
                    current_block_47 = 17784502470059252271;
                } else {
                    current_block_47 = 2516253395664191498;
                }
                match current_block_47 {
                    17784502470059252271 => {}
                    _ => return true_connect.unwrap()(sock, addr, len),
                }
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    flags = fcntl(sock, 3 as libc::c_int, 0 as libc::c_int);
    if flags & 0o4000 as libc::c_int != 0 {
        fcntl(sock, 4 as libc::c_int, (0o4000 as libc::c_int == 0) as libc::c_int);
    }
    memcpy(
        (dest_ip.addr.v6).as_mut_ptr() as *mut libc::c_void,
        if v6 != 0 {
            p_addr_in6 as *mut libc::c_void
        } else {
            p_addr_in as *mut libc::c_void
        },
        (if v6 != 0 { 16 as libc::c_int } else { 4 as libc::c_int }) as libc::c_ulong,
    );
    ret = connect_proxy_chain(
        sock,
        dest_ip,
        htons(port),
        proxychains_pd.as_mut_ptr(),
        proxychains_proxy_count,
        proxychains_ct,
        proxychains_max_chain,
    );
    fcntl(sock, 4 as libc::c_int, flags);
    if ret != SUCCESS as libc::c_int {
        *__errno_location() = 111 as libc::c_int;
    }
    return ret;
}
static mut ghbndata: gethostbyname_data = gethostbyname_data {
    hostent_space: hostent {
        h_name: 0 as *const libc::c_char as *mut libc::c_char,
        h_aliases: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        h_addrtype: 0,
        h_length: 0,
        h_addr_list: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
    },
    resolved_addr: 0,
    resolved_addr_p: [0 as *const libc::c_char as *mut libc::c_char; 2],
    addr_name: [0; 256],
};
pub unsafe extern "C" fn gethostbyname(mut name: *const libc::c_char) -> *mut hostent {
    init_lib_wrapper(
        (*::std::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"gethostbyname\0"))
            .as_ptr(),
    );
    if proxychains_resolver as libc::c_uint
        == DNSLF_FORKEXEC as libc::c_int as libc::c_uint
    {
        return proxy_gethostbyname_old(name)
    } else if proxychains_resolver as libc::c_uint
        == DNSLF_LIBC as libc::c_int as libc::c_uint
    {
        return true_gethostbyname.unwrap()(name)
    } else {
        return proxy_gethostbyname(name, &mut ghbndata)
    };
}
pub unsafe extern "C" fn getaddrinfo(
    mut node: *const libc::c_char,
    mut service: *const libc::c_char,
    mut hints: *const addrinfo,
    mut res: *mut *mut addrinfo,
) -> libc::c_int {
    init_lib_wrapper(
        (*::std::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"getaddrinfo\0"))
            .as_ptr(),
    );
    if proxychains_resolver as libc::c_uint != DNSLF_LIBC as libc::c_int as libc::c_uint
    {
        return proxy_getaddrinfo(node, service, hints, res)
    } else {
        return true_getaddrinfo.unwrap()(node, service, hints, res)
    };
}
pub unsafe extern "C" fn freeaddrinfo(mut res: *mut addrinfo) {
    init_lib_wrapper(
        (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"freeaddrinfo\0"))
            .as_ptr(),
    );
    if proxychains_resolver as libc::c_uint == DNSLF_LIBC as libc::c_int as libc::c_uint
    {
        true_freeaddrinfo.unwrap()(res);
    } else {
        proxy_freeaddrinfo(res);
    };
}
pub unsafe extern "C" fn getnameinfo(
    mut sa: *const sockaddr,
    mut salen: socklen_t,
    mut host: *mut libc::c_char,
    mut hostlen: socklen_t,
    mut serv: *mut libc::c_char,
    mut servlen: socklen_t,
    mut flags: libc::c_int,
) -> libc::c_int {
    init_lib_wrapper(
        (*::std::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"getnameinfo\0"))
            .as_ptr(),
    );
    if proxychains_resolver as libc::c_uint == DNSLF_LIBC as libc::c_int as libc::c_uint
    {
        return true_getnameinfo.unwrap()(sa, salen, host, hostlen, serv, servlen, flags)
    } else {
        if salen == 0
            || !((*(sa as *mut sockaddr_in)).sin_family as libc::c_int
                == 2 as libc::c_int
                || (*(sa as *mut sockaddr_in)).sin_family as libc::c_int
                    == 10 as libc::c_int)
        {
            return -(6 as libc::c_int);
        }
        let mut v6: libc::c_int = ((*(sa as *mut sockaddr_in)).sin_family as libc::c_int
            == 10 as libc::c_int) as libc::c_int;
        if (salen as libc::c_ulong)
            < (if v6 != 0 {
                ::std::mem::size_of::<sockaddr_in6>() as libc::c_ulong
            } else {
                ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong
            })
        {
            return -(6 as libc::c_int);
        }
        if hostlen != 0 {
            let mut v4inv6buf: [libc::c_uchar; 4] = [0; 4];
            let mut ip: *const libc::c_void = if v6 != 0 {
                &mut (*(sa as *mut sockaddr_in6)).sin6_addr as *mut in6_addr
                    as *mut libc::c_void
            } else {
                &mut (*(sa as *mut sockaddr_in)).sin_addr as *mut in_addr
                    as *mut libc::c_void
            };
            let mut scopeid: libc::c_uint = 0 as libc::c_int as libc::c_uint;
            if v6 != 0 {
                if is_v4inv6(&mut (*(sa as *mut sockaddr_in6)).sin6_addr) != 0 {
                    memcpy(
                        v4inv6buf.as_mut_ptr() as *mut libc::c_void,
                        &mut *((*(sa as *mut sockaddr_in6)).sin6_addr.__in6_u.__u6_addr8)
                            .as_mut_ptr()
                            .offset(12 as libc::c_int as isize) as *mut uint8_t
                            as *const libc::c_void,
                        4 as libc::c_int as libc::c_ulong,
                    );
                    ip = v4inv6buf.as_mut_ptr() as *const libc::c_void;
                    v6 = 0 as libc::c_int;
                } else {
                    scopeid = (*(sa as *mut sockaddr_in6)).sin6_scope_id;
                }
            }
            if (inet_ntop(
                if v6 != 0 { 10 as libc::c_int } else { 2 as libc::c_int },
                ip,
                host,
                hostlen,
            ))
                .is_null()
            {
                return -(12 as libc::c_int);
            }
            if scopeid != 0 {
                let mut l: size_t = strlen(host);
                if snprintf(
                    host.offset(l as isize),
                    (hostlen as libc::c_ulong).wrapping_sub(l),
                    b"%%%u\0" as *const u8 as *const libc::c_char,
                    scopeid,
                ) as libc::c_ulong >= (hostlen as libc::c_ulong).wrapping_sub(l)
                {
                    return -(12 as libc::c_int);
                }
            }
        }
        if servlen != 0 {
            if snprintf(
                serv,
                servlen as libc::c_ulong,
                b"%d\0" as *const u8 as *const libc::c_char,
                ntohs((*(sa as *mut sockaddr_in)).sin_port) as libc::c_int,
            ) as libc::c_uint >= servlen
            {
                return -(12 as libc::c_int);
            }
        }
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn gethostbyaddr(
    mut addr: *const libc::c_void,
    mut len: socklen_t,
    mut type_0: libc::c_int,
) -> *mut hostent {
    init_lib_wrapper(
        (*::std::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"gethostbyaddr\0"))
            .as_ptr(),
    );
    static mut buf: [libc::c_char; 16] = [0; 16];
    static mut ipv4: [libc::c_char; 4] = [0; 4];
    static mut list: [*mut libc::c_char; 2] = [0 as *const libc::c_char
        as *mut libc::c_char; 2];
    static mut aliases: [*mut libc::c_char; 1] = [0 as *const libc::c_char
        as *mut libc::c_char; 1];
    static mut he: hostent = hostent {
        h_name: 0 as *const libc::c_char as *mut libc::c_char,
        h_aliases: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        h_addrtype: 0,
        h_length: 0,
        h_addr_list: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
    };
    if proxychains_resolver as libc::c_uint == DNSLF_LIBC as libc::c_int as libc::c_uint
    {
        return true_gethostbyaddr.unwrap()(addr, len, type_0)
    } else {
        if len != 4 as libc::c_int as libc::c_uint {
            return 0 as *mut hostent;
        }
        he.h_name = buf.as_mut_ptr();
        memcpy(
            ipv4.as_mut_ptr() as *mut libc::c_void,
            addr,
            4 as libc::c_int as libc::c_ulong,
        );
        list[0 as libc::c_int as usize] = ipv4.as_mut_ptr();
        list[1 as libc::c_int as usize] = 0 as *mut libc::c_char;
        he.h_addr_list = list.as_mut_ptr();
        he.h_addrtype = 2 as libc::c_int;
        aliases[0 as libc::c_int as usize] = 0 as *mut libc::c_char;
        he.h_aliases = aliases.as_mut_ptr();
        he.h_length = 4 as libc::c_int;
        pc_stringfromipv4(addr as *mut libc::c_uchar, buf.as_mut_ptr());
        return &mut he;
    };
}
pub unsafe extern "C" fn sendto(
    mut sockfd: libc::c_int,
    mut buf: *const libc::c_void,
    mut len: size_t,
    mut flags: libc::c_int,
    mut dest_addr: __CONST_SOCKADDR_ARG,
    mut addrlen: socklen_t,
) -> ssize_t {
    init_lib_wrapper(
        (*::std::mem::transmute::<&[u8; 7], &[libc::c_char; 7]>(b"sendto\0")).as_ptr(),
    );
    if flags & MSG_FASTOPEN as libc::c_int != 0 {
        if connect(sockfd, dest_addr.__sockaddr__, addrlen) == 0
            && *__errno_location() != 115 as libc::c_int
        {
            return -(1 as libc::c_int) as ssize_t;
        }
        dest_addr.__sockaddr__ = 0 as *const sockaddr;
        addrlen = 0 as libc::c_int as socklen_t;
        flags &= !(MSG_FASTOPEN as libc::c_int);
    }
    return true_sendto
        .unwrap()(sockfd, buf, len, flags, dest_addr.__sockaddr__, addrlen);
}
unsafe extern "C" fn setup_hooks() {
    if true_connect.is_none() {
        true_connect = ::std::mem::transmute::<
            *mut libc::c_void,
            connect_t,
        >(
            load_sym(
                b"connect\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ::std::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            libc::c_int,
                            *const sockaddr,
                            libc::c_uint,
                        ) -> libc::c_int,
                    >,
                    *mut libc::c_void,
                >(
                    Some(
                        connect
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const sockaddr,
                                libc::c_uint,
                            ) -> libc::c_int,
                    ),
                ),
                1 as libc::c_int,
            ),
        );
    }
    if true_sendto.is_none() {
        true_sendto = ::std::mem::transmute::<
            *mut libc::c_void,
            sendto_t,
        >(
            load_sym(
                b"sendto\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ::std::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            libc::c_int,
                            *const libc::c_void,
                            size_t,
                            libc::c_int,
                            __CONST_SOCKADDR_ARG,
                            socklen_t,
                        ) -> ssize_t,
                    >,
                    *mut libc::c_void,
                >(
                    Some(
                        sendto
                            as unsafe extern "C" fn(
                                libc::c_int,
                                *const libc::c_void,
                                size_t,
                                libc::c_int,
                                __CONST_SOCKADDR_ARG,
                                socklen_t,
                            ) -> ssize_t,
                    ),
                ),
                1 as libc::c_int,
            ),
        );
    }
    if true_gethostbyname.is_none() {
        true_gethostbyname = ::std::mem::transmute::<
            *mut libc::c_void,
            gethostbyname_t,
        >(
            load_sym(
                b"gethostbyname\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(*const libc::c_char) -> *mut hostent>,
                    *mut libc::c_void,
                >(
                    Some(
                        gethostbyname
                            as unsafe extern "C" fn(*const libc::c_char) -> *mut hostent,
                    ),
                ),
                1 as libc::c_int,
            ),
        );
    }
    if true_getaddrinfo.is_none() {
        true_getaddrinfo = ::std::mem::transmute::<
            *mut libc::c_void,
            getaddrinfo_t,
        >(
            load_sym(
                b"getaddrinfo\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ::std::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                            *const addrinfo,
                            *mut *mut addrinfo,
                        ) -> libc::c_int,
                    >,
                    *mut libc::c_void,
                >(
                    Some(
                        getaddrinfo
                            as unsafe extern "C" fn(
                                *const libc::c_char,
                                *const libc::c_char,
                                *const addrinfo,
                                *mut *mut addrinfo,
                            ) -> libc::c_int,
                    ),
                ),
                1 as libc::c_int,
            ),
        );
    }
    if true_freeaddrinfo.is_none() {
        true_freeaddrinfo = ::std::mem::transmute::<
            *mut libc::c_void,
            freeaddrinfo_t,
        >(
            load_sym(
                b"freeaddrinfo\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(*mut addrinfo) -> ()>,
                    *mut libc::c_void,
                >(Some(freeaddrinfo as unsafe extern "C" fn(*mut addrinfo) -> ())),
                1 as libc::c_int,
            ),
        );
    }
    if true_gethostbyaddr.is_none() {
        true_gethostbyaddr = ::std::mem::transmute::<
            *mut libc::c_void,
            gethostbyaddr_t,
        >(
            load_sym(
                b"gethostbyaddr\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ::std::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *const libc::c_void,
                            __socklen_t,
                            libc::c_int,
                        ) -> *mut hostent,
                    >,
                    *mut libc::c_void,
                >(
                    Some(
                        gethostbyaddr
                            as unsafe extern "C" fn(
                                *const libc::c_void,
                                __socklen_t,
                                libc::c_int,
                            ) -> *mut hostent,
                    ),
                ),
                1 as libc::c_int,
            ),
        );
    }
    if true_getnameinfo.is_none() {
        true_getnameinfo = ::std::mem::transmute::<
            *mut libc::c_void,
            getnameinfo_t,
        >(
            load_sym(
                b"getnameinfo\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ::std::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *const sockaddr,
                            socklen_t,
                            *mut libc::c_char,
                            socklen_t,
                            *mut libc::c_char,
                            socklen_t,
                            libc::c_int,
                        ) -> libc::c_int,
                    >,
                    *mut libc::c_void,
                >(
                    Some(
                        getnameinfo
                            as unsafe extern "C" fn(
                                *const sockaddr,
                                socklen_t,
                                *mut libc::c_char,
                                socklen_t,
                                *mut libc::c_char,
                                socklen_t,
                                libc::c_int,
                            ) -> libc::c_int,
                    ),
                ),
                1 as libc::c_int,
            ),
        );
    }
    if true_close.is_none() {
        true_close = ::std::mem::transmute::<
            *mut libc::c_void,
            close_t,
        >(
            load_sym(
                b"close\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(libc::c_int) -> libc::c_int>,
                    *mut libc::c_void,
                >(Some(close as unsafe extern "C" fn(libc::c_int) -> libc::c_int)),
                1 as libc::c_int,
            ),
        );
    }
    if true_close_range.is_none() {
        true_close_range = ::std::mem::transmute::<
            *mut libc::c_void,
            close_range_t,
        >(
            load_sym(
                b"close_range\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ::std::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            libc::c_uint,
                            libc::c_uint,
                            libc::c_int,
                        ) -> libc::c_int,
                    >,
                    *mut libc::c_void,
                >(
                    Some(
                        close_range
                            as unsafe extern "C" fn(
                                libc::c_uint,
                                libc::c_uint,
                                libc::c_int,
                            ) -> libc::c_int,
                    ),
                ),
                0 as libc::c_int,
            ),
        );
    }
}
