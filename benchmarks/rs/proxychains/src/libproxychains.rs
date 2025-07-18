use ::libc;
extern "C" {
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
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn dlsym(
        __handle: *mut libc::c_void,
        __name: *const libc::c_char,
    ) -> *mut libc::c_void;
    fn dlerror() -> *mut libc::c_char;
    fn __errno_location() -> *mut libc::c_int;
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
    fn getsockopt(
        __fd: libc::c_int,
        __level: libc::c_int,
        __optname: libc::c_int,
        __optval: *mut libc::c_void,
        __optlen: *mut socklen_t,
    ) -> libc::c_int;
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
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn abort() -> !;
    fn exit(_: libc::c_int) -> !;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn inet_addr(__cp: *const libc::c_char) -> in_addr_t;
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
    static mut internal_ips_lock: pthread_mutex_t;
    fn proxychains_write_log(_: *mut libc::c_char, _: ...);
    fn proxy_gethostbyname(
        _: *const libc::c_char,
        _: *mut gethostbyname_data,
    ) -> *mut hostent;
    fn proxy_getaddrinfo(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: *const addrinfo,
        _: *mut *mut addrinfo,
    ) -> libc::c_int;
    fn proxy_freeaddrinfo(_: *mut addrinfo);
    fn connect_proxy_chain(
        _: libc::c_int,
        _: ip_type,
        _: libc::c_ushort,
        _: *mut proxy_data,
        _: libc::c_uint,
        _: chain_type,
        _: libc::c_uint,
    ) -> libc::c_int;
    fn pthread_once(
        __once_control: *mut pthread_once_t,
        __init_routine: Option::<unsafe extern "C" fn() -> ()>,
    ) -> libc::c_int;
    fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> libc::c_int;
    fn get_config_path(
        default_path: *mut libc::c_char,
        pbuf: *mut libc::c_char,
        bufsize: size_t,
    ) -> *mut libc::c_char;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
pub type size_t = libc::c_ulong;
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
pub union pthread_mutexattr_t {
    pub __size: [libc::c_char; 4],
    pub __align: libc::c_int,
}
pub type pthread_once_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
    pub __align: libc::c_long,
}
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
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
    pub __in6_u: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
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
pub type __CONST_SOCKADDR_ARG = *const sockaddr;
pub const SUCCESS: C2RustUnnamed_0 = 0;
pub type chain_type = libc::c_uint;
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
pub const SOCKS5_TYPE: proxy_type = 3;
pub const SOCKS4_TYPE: proxy_type = 2;
pub const RAW_TYPE: proxy_type = 1;
pub const HTTP_TYPE: proxy_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub union ip_type {
    pub octet: [libc::c_uchar; 4],
    pub as_int: uint32_t,
}
pub type connect_t = Option::<
    unsafe extern "C" fn(libc::c_int, *const sockaddr, socklen_t) -> libc::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct localaddr_arg {
    pub in_addr: in_addr,
    pub netmask: in_addr,
    pub port: libc::c_ushort,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dnat_arg {
    pub orig_dst: in_addr,
    pub new_dst: in_addr,
    pub orig_port: libc::c_ushort,
    pub new_port: libc::c_ushort,
}
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
pub type FILE = _IO_FILE;
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
    pub addr_name: [libc::c_char; 8192],
}
pub type C2RustUnnamed_0 = libc::c_uint;
pub const BLOCKED: C2RustUnnamed_0 = 5;
pub const CHAIN_EMPTY: C2RustUnnamed_0 = 4;
pub const CHAIN_DOWN: C2RustUnnamed_0 = 3;
pub const SOCKET_ERROR: C2RustUnnamed_0 = 2;
pub const MEMORY_FAIL: C2RustUnnamed_0 = 1;
#[inline]
unsafe extern "C" fn __bswap_16(mut __bsx: __uint16_t) -> __uint16_t {
    return (__bsx as libc::c_int >> 8 as libc::c_int & 0xff as libc::c_int
        | (__bsx as libc::c_int & 0xff as libc::c_int) << 8 as libc::c_int)
        as __uint16_t;
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
pub static mut true_connect: connect_t = None;
pub static mut true_gethostbyname: gethostbyname_t = None;
pub static mut true_getaddrinfo: getaddrinfo_t = None;
pub static mut true_freeaddrinfo: freeaddrinfo_t = None;
pub static mut true_getnameinfo: getnameinfo_t = None;
pub static mut true_gethostbyaddr: gethostbyaddr_t = None;
pub static mut tcp_read_time_out: libc::c_int = 0;
pub static mut tcp_connect_time_out: libc::c_int = 0;
pub static mut proxychains_got_chain_data: libc::c_int = 0 as libc::c_int;
pub static mut proxychains_quiet_mode: libc::c_int = 0 as libc::c_int;
pub static mut proxychains_resolver: libc::c_int = 0 as libc::c_int;
pub static mut proxychains_proxy_count: libc::c_uint = 0 as libc::c_int as libc::c_uint;
pub static mut proxychains_max_chain: libc::c_uint = 1 as libc::c_int as libc::c_uint;
pub static mut remote_dns_subnet: libc::c_uint = 224 as libc::c_int as libc::c_uint;
pub static mut localnet_addr: [localaddr_arg; 64] = [localaddr_arg {
    in_addr: in_addr { s_addr: 0 },
    netmask: in_addr { s_addr: 0 },
    port: 0,
}; 64];
pub static mut proxychains_ct: chain_type = DYNAMIC_TYPE;
pub static mut proxychains_pd: [proxy_data; 512] = [proxy_data {
    ip: ip_type { octet: [0; 4] },
    port: 0,
    pt: HTTP_TYPE,
    ps: PLAY_STATE,
    user: [0; 256],
    pass: [0; 256],
}; 512];
pub static mut num_localnet_addr: size_t = 0 as libc::c_int as size_t;
pub static mut dnats: [dnat_arg; 64] = [dnat_arg {
    orig_dst: in_addr { s_addr: 0 },
    new_dst: in_addr { s_addr: 0 },
    orig_port: 0,
    new_port: 0,
}; 64];
pub static mut num_dnats: size_t = 0 as libc::c_int as size_t;
pub static mut init_once: pthread_once_t = 0 as libc::c_int;
static mut init_l: libc::c_int = 0 as libc::c_int;
unsafe extern "C" fn load_sym(
    mut symname: *mut libc::c_char,
    mut proxyfunc: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut funcptr: *mut libc::c_void = dlsym(
        -(1 as libc::c_long) as *mut libc::c_void,
        symname,
    );
    if funcptr.is_null() {
        fprintf(
            stderr,
            b"Cannot load symbol '%s' %s\n\0" as *const u8 as *const libc::c_char,
            symname,
            dlerror(),
        );
        exit(1 as libc::c_int);
    }
    if funcptr == proxyfunc {
        abort();
    }
    return funcptr;
}
unsafe extern "C" fn do_init() {
    pthread_mutex_init(&mut internal_ips_lock, 0 as *const pthread_mutexattr_t);
    simple_socks5_env(
        proxychains_pd.as_mut_ptr(),
        &mut proxychains_proxy_count,
        &mut proxychains_ct,
    );
    get_chain_data(
        proxychains_pd.as_mut_ptr(),
        &mut proxychains_proxy_count,
        &mut proxychains_ct,
    );
    proxychains_write_log(
        b"[proxychains] DLL init\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
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
                        __CONST_SOCKADDR_ARG,
                        socklen_t,
                    ) -> libc::c_int,
                >,
                *mut libc::c_void,
            >(
                Some(
                    connect
                        as unsafe extern "C" fn(
                            libc::c_int,
                            __CONST_SOCKADDR_ARG,
                            socklen_t,
                        ) -> libc::c_int,
                ),
            ),
        ),
    );
    true_gethostbyname = ::std::mem::transmute::<
        *mut libc::c_void,
        gethostbyname_t,
    >(
        load_sym(
            b"gethostbyname\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*const libc::c_char) -> *mut hostent>,
                *mut libc::c_void,
            >(
                Some(
                    gethostbyname
                        as unsafe extern "C" fn(*const libc::c_char) -> *mut hostent,
                ),
            ),
        ),
    );
    true_getaddrinfo = ::std::mem::transmute::<
        *mut libc::c_void,
        getaddrinfo_t,
    >(
        load_sym(
            b"getaddrinfo\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
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
        ),
    );
    true_freeaddrinfo = ::std::mem::transmute::<
        *mut libc::c_void,
        freeaddrinfo_t,
    >(
        load_sym(
            b"freeaddrinfo\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut addrinfo) -> ()>,
                *mut libc::c_void,
            >(Some(freeaddrinfo as unsafe extern "C" fn(*mut addrinfo) -> ())),
        ),
    );
    true_gethostbyaddr = ::std::mem::transmute::<
        *mut libc::c_void,
        gethostbyaddr_t,
    >(
        load_sym(
            b"gethostbyaddr\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
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
        ),
    );
    true_getnameinfo = ::std::mem::transmute::<
        *mut libc::c_void,
        getnameinfo_t,
    >(
        load_sym(
            b"getnameinfo\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
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
        ),
    );
    init_l = 1 as libc::c_int;
}
unsafe extern "C" fn init_lib_wrapper(mut caller: *const libc::c_char) {
    init_l == 0;
    pthread_once(&mut init_once, Some(do_init as unsafe extern "C" fn() -> ()));
}
pub unsafe extern "C" fn open_config_file() -> *mut FILE {
    let mut home_conf: [libc::c_char; 4096] = [0; 4096];
    let mut prefix_conf: [libc::c_char; 4096] = [0; 4096];
    let mut file: *mut FILE = 0 as *mut FILE;
    snprintf(
        home_conf.as_mut_ptr(),
        256 as libc::c_int as libc::c_ulong,
        b"%s/.proxychains/proxychains.conf\0" as *const u8 as *const libc::c_char,
        getenv(b"HOME\0" as *const u8 as *const libc::c_char),
    );
    snprintf(
        prefix_conf.as_mut_ptr(),
        256 as libc::c_int as libc::c_ulong,
        b"%s/etc/proxychains.conf\0" as *const u8 as *const libc::c_char,
        b"/usr/local\0" as *const u8 as *const libc::c_char,
    );
    file = fopen(
        b"./proxychains.conf\0" as *const u8 as *const libc::c_char,
        b"r\0" as *const u8 as *const libc::c_char,
    );
    if file.is_null() {
        file = fopen(home_conf.as_mut_ptr(), b"r\0" as *const u8 as *const libc::c_char);
        if file.is_null() {
            file = fopen(
                prefix_conf.as_mut_ptr(),
                b"r\0" as *const u8 as *const libc::c_char,
            );
            if file.is_null() {
                file = fopen(
                    b"/etc/proxychains.conf\0" as *const u8 as *const libc::c_char,
                    b"r\0" as *const u8 as *const libc::c_char,
                );
                if file.is_null() {
                    perror(
                        b"Can't locate proxychains.conf\0" as *const u8
                            as *const libc::c_char,
                    );
                    exit(1 as libc::c_int);
                }
            }
        }
    }
    return file;
}
unsafe extern "C" fn load_default_settings(mut ct: *mut chain_type) {
    let mut env: *mut libc::c_char = 0 as *mut libc::c_char;
    tcp_read_time_out = 4 as libc::c_int * 1000 as libc::c_int;
    tcp_connect_time_out = 10 as libc::c_int * 1000 as libc::c_int;
    *ct = DYNAMIC_TYPE;
    env = getenv(b"PROXYCHAINS_QUIET_MODE\0" as *const u8 as *const libc::c_char);
    if !env.is_null() && *env as libc::c_int == '1' as i32 {
        proxychains_quiet_mode = 1 as libc::c_int;
    }
}
unsafe extern "C" fn get_chain_data(
    mut pd: *mut proxy_data,
    mut proxy_count: *mut libc::c_uint,
    mut ct: *mut chain_type,
) {
    let mut count: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut port_n: libc::c_int = 0 as libc::c_int;
    let mut list: libc::c_int = 0 as libc::c_int;
    let mut buff: [libc::c_char; 1024] = [0; 1024];
    let mut type_0: [libc::c_char; 1024] = [0; 1024];
    let mut host: [libc::c_char; 1024] = [0; 1024];
    let mut user: [libc::c_char; 1024] = [0; 1024];
    let mut env: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut local_in_addr_port: [libc::c_char; 32] = [0; 32];
    let mut local_in_addr: [libc::c_char; 32] = [0; 32];
    let mut local_in_port: [libc::c_char; 32] = [0; 32];
    let mut local_netmask: [libc::c_char; 32] = [0; 32];
    let mut dnat_orig_addr_port: [libc::c_char; 32] = [0; 32];
    let mut dnat_new_addr_port: [libc::c_char; 32] = [0; 32];
    let mut dnat_orig_addr: [libc::c_char; 32] = [0; 32];
    let mut dnat_orig_port: [libc::c_char; 32] = [0; 32];
    let mut dnat_new_addr: [libc::c_char; 32] = [0; 32];
    let mut dnat_new_port: [libc::c_char; 32] = [0; 32];
    let mut file: *mut FILE = 0 as *mut FILE;
    if proxychains_got_chain_data != 0 {
        return;
    }
    load_default_settings(ct);
    env = get_config_path(
        getenv(b"PROXYCHAINS_CONF_FILE\0" as *const u8 as *const libc::c_char),
        buff.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
    );
    file = fopen(env, b"r\0" as *const u8 as *const libc::c_char);
    while !(fgets(
        buff.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
        file,
    ))
        .is_null()
    {
        if !(buff[0 as libc::c_int as usize] as libc::c_int != '\n' as i32
            && buff[strspn(buff.as_mut_ptr(), b" \0" as *const u8 as *const libc::c_char)
                as usize] as libc::c_int != '#' as i32)
        {
            continue;
        }
        if list != 0 {
            if count >= 512 as libc::c_int as libc::c_uint {
                break;
            }
            memset(
                &mut *pd.offset(count as isize) as *mut proxy_data as *mut libc::c_void,
                0 as libc::c_int,
                ::std::mem::size_of::<proxy_data>() as libc::c_ulong,
            );
            (*pd.offset(count as isize)).ps = PLAY_STATE;
            port_n = 0 as libc::c_int;
            sscanf(
                buff.as_mut_ptr(),
                b"%s %s %d %s %s\0" as *const u8 as *const libc::c_char,
                type_0.as_mut_ptr(),
                host.as_mut_ptr(),
                &mut port_n as *mut libc::c_int,
                ((*pd.offset(count as isize)).user).as_mut_ptr(),
                ((*pd.offset(count as isize)).pass).as_mut_ptr(),
            );
            (*pd.offset(count as isize)).ip.as_int = inet_addr(host.as_mut_ptr());
            (*pd.offset(count as isize)).port = __bswap_16(port_n as libc::c_ushort);
            if strcmp(type_0.as_mut_ptr(), b"http\0" as *const u8 as *const libc::c_char)
                == 0
            {
                (*pd.offset(count as isize)).pt = HTTP_TYPE;
            } else if strcmp(
                type_0.as_mut_ptr(),
                b"raw\0" as *const u8 as *const libc::c_char,
            ) == 0
            {
                (*pd.offset(count as isize)).pt = RAW_TYPE;
            } else if strcmp(
                type_0.as_mut_ptr(),
                b"socks4\0" as *const u8 as *const libc::c_char,
            ) == 0
            {
                (*pd.offset(count as isize)).pt = SOCKS4_TYPE;
            } else {
                if !(strcmp(
                    type_0.as_mut_ptr(),
                    b"socks5\0" as *const u8 as *const libc::c_char,
                ) == 0)
                {
                    continue;
                }
                (*pd.offset(count as isize)).pt = SOCKS5_TYPE;
            }
            if (*pd.offset(count as isize)).ip.as_int != 0 && port_n != 0
                && (*pd.offset(count as isize)).ip.as_int
                    != -(1 as libc::c_int) as uint32_t
            {
                count = count.wrapping_add(1);
                count;
            }
        } else if !(strstr(
            buff.as_mut_ptr(),
            b"[ProxyList]\0" as *const u8 as *const libc::c_char,
        ))
            .is_null()
        {
            list = 1 as libc::c_int;
        } else if !(strstr(
            buff.as_mut_ptr(),
            b"random_chain\0" as *const u8 as *const libc::c_char,
        ))
            .is_null()
        {
            *ct = RANDOM_TYPE;
        } else if !(strstr(
            buff.as_mut_ptr(),
            b"strict_chain\0" as *const u8 as *const libc::c_char,
        ))
            .is_null()
        {
            *ct = STRICT_TYPE;
        } else if !(strstr(
            buff.as_mut_ptr(),
            b"dynamic_chain\0" as *const u8 as *const libc::c_char,
        ))
            .is_null()
        {
            *ct = DYNAMIC_TYPE;
        } else if !(strstr(
            buff.as_mut_ptr(),
            b"tcp_read_time_out\0" as *const u8 as *const libc::c_char,
        ))
            .is_null()
        {
            sscanf(
                buff.as_mut_ptr(),
                b"%s %d\0" as *const u8 as *const libc::c_char,
                user.as_mut_ptr(),
                &mut tcp_read_time_out as *mut libc::c_int,
            );
        } else if !(strstr(
            buff.as_mut_ptr(),
            b"tcp_connect_time_out\0" as *const u8 as *const libc::c_char,
        ))
            .is_null()
        {
            sscanf(
                buff.as_mut_ptr(),
                b"%s %d\0" as *const u8 as *const libc::c_char,
                user.as_mut_ptr(),
                &mut tcp_connect_time_out as *mut libc::c_int,
            );
        } else if !(strstr(
            buff.as_mut_ptr(),
            b"remote_dns_subnet\0" as *const u8 as *const libc::c_char,
        ))
            .is_null()
        {
            sscanf(
                buff.as_mut_ptr(),
                b"%s %d\0" as *const u8 as *const libc::c_char,
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
        } else if !(strstr(
            buff.as_mut_ptr(),
            b"localnet\0" as *const u8 as *const libc::c_char,
        ))
            .is_null()
        {
            if sscanf(
                buff.as_mut_ptr(),
                b"%s %21[^/]/%15s\0" as *const u8 as *const libc::c_char,
                user.as_mut_ptr(),
                local_in_addr_port.as_mut_ptr(),
                local_netmask.as_mut_ptr(),
            ) < 3 as libc::c_int
            {
                fprintf(
                    stderr,
                    b"localnet format error\0" as *const u8 as *const libc::c_char,
                );
                exit(1 as libc::c_int);
            }
            memset(
                local_in_port.as_mut_ptr() as *mut libc::c_void,
                0 as libc::c_int,
                (::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong)
                    .wrapping_div(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
            );
            sscanf(
                local_in_addr_port.as_mut_ptr(),
                b"%15[^:]:%5s\0" as *const u8 as *const libc::c_char,
                local_in_addr.as_mut_ptr(),
                local_in_port.as_mut_ptr(),
            ) < 2 as libc::c_int;
            if num_localnet_addr < 64 as libc::c_int as libc::c_ulong {
                let mut error: libc::c_int = 0;
                error = inet_pton(
                    2 as libc::c_int,
                    local_in_addr.as_mut_ptr(),
                    &mut (*localnet_addr.as_mut_ptr().offset(num_localnet_addr as isize))
                        .in_addr as *mut in_addr as *mut libc::c_void,
                );
                if error <= 0 as libc::c_int {
                    fprintf(
                        stderr,
                        b"localnet address error\n\0" as *const u8 as *const libc::c_char,
                    );
                    exit(1 as libc::c_int);
                }
                error = inet_pton(
                    2 as libc::c_int,
                    local_netmask.as_mut_ptr(),
                    &mut (*localnet_addr.as_mut_ptr().offset(num_localnet_addr as isize))
                        .netmask as *mut in_addr as *mut libc::c_void,
                );
                if error <= 0 as libc::c_int {
                    fprintf(
                        stderr,
                        b"localnet netmask error\n\0" as *const u8 as *const libc::c_char,
                    );
                    exit(1 as libc::c_int);
                }
                if local_in_port[0 as libc::c_int as usize] != 0 {
                    localnet_addr[num_localnet_addr as usize]
                        .port = atoi(local_in_port.as_mut_ptr()) as libc::c_ushort;
                } else {
                    localnet_addr[num_localnet_addr as usize]
                        .port = 0 as libc::c_int as libc::c_ushort;
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
        } else if !(strstr(
            buff.as_mut_ptr(),
            b"chain_len\0" as *const u8 as *const libc::c_char,
        ))
            .is_null()
        {
            let mut pc: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut len: libc::c_int = 0;
            pc = strchr(buff.as_mut_ptr(), '=' as i32);
            pc = pc.offset(1);
            len = atoi(pc);
            proxychains_max_chain = (if len != 0 { len } else { 1 as libc::c_int })
                as libc::c_uint;
        } else if !(strstr(
            buff.as_mut_ptr(),
            b"quiet_mode\0" as *const u8 as *const libc::c_char,
        ))
            .is_null()
        {
            proxychains_quiet_mode = 1 as libc::c_int;
        } else if !(strstr(
            buff.as_mut_ptr(),
            b"proxy_dns\0" as *const u8 as *const libc::c_char,
        ))
            .is_null()
        {
            proxychains_resolver = 1 as libc::c_int;
        } else if !(strstr(
            buff.as_mut_ptr(),
            b"dnat\0" as *const u8 as *const libc::c_char,
        ))
            .is_null()
        {
            if sscanf(
                buff.as_mut_ptr(),
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
                        .orig_port = atoi(dnat_orig_port.as_mut_ptr()) as libc::c_ushort;
                } else {
                    dnats[num_dnats as usize]
                        .orig_port = 0 as libc::c_int as libc::c_ushort;
                }
                if dnat_new_port[0 as libc::c_int as usize] != 0 {
                    dnats[num_dnats as usize]
                        .new_port = atoi(dnat_new_port.as_mut_ptr()) as libc::c_ushort;
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
    *proxy_count = count;
    proxychains_got_chain_data = 1 as libc::c_int;
}
unsafe extern "C" fn simple_socks5_env(
    mut pd: *mut proxy_data,
    mut proxy_count: *mut libc::c_uint,
    mut ct: *mut chain_type,
) {
    let mut port_string: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut host_string: *mut libc::c_char = 0 as *mut libc::c_char;
    if proxychains_got_chain_data != 0 {
        return;
    }
    load_default_settings(ct);
    port_string = getenv(
        b"PROXYCHAINS_SOCKS5_PORT\0" as *const u8 as *const libc::c_char,
    );
    if port_string.is_null() {
        return;
    }
    host_string = getenv(
        b"PROXYCHAINS_SOCKS5_HOST\0" as *const u8 as *const libc::c_char,
    );
    if host_string.is_null() {
        host_string = b"127.0.0.1\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char;
    }
    memset(
        pd as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<proxy_data>() as libc::c_ulong,
    );
    (*pd.offset(0 as libc::c_int as isize)).ps = PLAY_STATE;
    (*pd.offset(0 as libc::c_int as isize)).ip.as_int = inet_addr(host_string);
    (*pd.offset(0 as libc::c_int as isize))
        .port = __bswap_16(
        strtol(port_string, 0 as *mut *mut libc::c_char, 0 as libc::c_int)
            as libc::c_ushort,
    );
    (*pd.offset(0 as libc::c_int as isize)).pt = SOCKS5_TYPE;
    proxychains_max_chain = 1 as libc::c_int as libc::c_uint;
    if !(getenv(b"PROXYCHAINS_DNS\0" as *const u8 as *const libc::c_char)).is_null() {
        proxychains_resolver = 1 as libc::c_int;
    }
    *proxy_count = 1 as libc::c_int as libc::c_uint;
    proxychains_got_chain_data = 1 as libc::c_int;
}
pub unsafe extern "C" fn connect(
    mut sock: libc::c_int,
    mut addr: *const sockaddr,
    mut len: socklen_t,
) -> libc::c_int {
    let mut socktype: libc::c_int = 0 as libc::c_int;
    let mut flags: libc::c_int = 0 as libc::c_int;
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut optlen: socklen_t = 0 as libc::c_int as socklen_t;
    let mut dest_ip: ip_type = ip_type { octet: [0; 4] };
    let mut p_addr_in: *mut in_addr = 0 as *mut in_addr;
    let mut new_addr: sockaddr_in = sockaddr_in {
        sin_family: 0,
        sin_port: 0,
        sin_addr: in_addr { s_addr: 0 },
        sin_zero: [0; 8],
    };
    let mut dnat: *mut dnat_arg = 0 as *mut dnat_arg;
    let mut port: libc::c_ushort = 0;
    let mut i: size_t = 0;
    let mut remote_dns_connect: libc::c_int = 0 as libc::c_int;
    init_lib_wrapper(
        (*::std::mem::transmute::<&[u8; 8], &[libc::c_char; 8]>(b"connect\0")).as_ptr(),
    );
    optlen = ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t;
    getsockopt(
        sock,
        1 as libc::c_int,
        3 as libc::c_int,
        &mut socktype as *mut libc::c_int as *mut libc::c_void,
        &mut optlen,
    );
    if !((*(addr as *mut sockaddr_in)).sin_family as libc::c_int == 2 as libc::c_int
        && socktype == SOCK_STREAM as libc::c_int)
    {
        return true_connect.unwrap()(sock, addr, len);
    }
    p_addr_in = &mut (*(addr as *mut sockaddr_in)).sin_addr;
    port = __bswap_16((*(addr as *mut sockaddr_in)).sin_port);
    remote_dns_connect = (__bswap_32((*p_addr_in).s_addr) >> 24 as libc::c_int
        == remote_dns_subnet) as libc::c_int;
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
    if !dnat.is_null() {
        if (*dnat).new_port != 0 {
            new_addr.sin_port = __bswap_16((*dnat).new_port);
        } else {
            new_addr.sin_port = __bswap_16(port);
        }
        new_addr.sin_addr = (*dnat).new_dst;
        addr = &mut new_addr as *mut sockaddr_in as *mut sockaddr;
        p_addr_in = &mut (*(addr as *mut sockaddr_in)).sin_addr;
        port = __bswap_16((*(addr as *mut sockaddr_in)).sin_port);
    }
    i = 0 as libc::c_int as size_t;
    while i < num_localnet_addr && remote_dns_connect == 0 {
        if localnet_addr[i as usize].in_addr.s_addr
            & localnet_addr[i as usize].netmask.s_addr
            == (*p_addr_in).s_addr & localnet_addr[i as usize].netmask.s_addr
        {
            if localnet_addr[i as usize].port == 0
                || localnet_addr[i as usize].port as libc::c_int == port as libc::c_int
            {
                return true_connect.unwrap()(sock, addr, len);
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    flags = fcntl(sock, 3 as libc::c_int, 0 as libc::c_int);
    if flags & 0o4000 as libc::c_int != 0 {
        fcntl(sock, 4 as libc::c_int, (0o4000 as libc::c_int == 0) as libc::c_int);
    }
    dest_ip.as_int = (*(addr as *mut sockaddr_in)).sin_addr.s_addr;
    ret = connect_proxy_chain(
        sock,
        dest_ip,
        (*(addr as *mut sockaddr_in)).sin_port,
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
    addr_name: [0; 8192],
};
pub unsafe extern "C" fn gethostbyname(mut name: *const libc::c_char) -> *mut hostent {
    init_lib_wrapper(
        (*::std::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"gethostbyname\0"))
            .as_ptr(),
    );
    if proxychains_resolver != 0 {
        return proxy_gethostbyname(name, &mut ghbndata)
    } else {
        return true_gethostbyname.unwrap()(name)
    };
}
pub unsafe extern "C" fn getaddrinfo(
    mut node: *const libc::c_char,
    mut service: *const libc::c_char,
    mut hints: *const addrinfo,
    mut res: *mut *mut addrinfo,
) -> libc::c_int {
    let mut ret: libc::c_int = 0 as libc::c_int;
    init_lib_wrapper(
        (*::std::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"getaddrinfo\0"))
            .as_ptr(),
    );
    if proxychains_resolver != 0 {
        ret = proxy_getaddrinfo(node, service, hints, res);
    } else {
        ret = true_getaddrinfo.unwrap()(node, service, hints, res);
    }
    return ret;
}
pub unsafe extern "C" fn freeaddrinfo(mut res: *mut addrinfo) {
    init_lib_wrapper(
        (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"freeaddrinfo\0"))
            .as_ptr(),
    );
    if proxychains_resolver == 0 {
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
    let mut ip_buf: [libc::c_char; 16] = [0; 16];
    let mut ret: libc::c_int = 0 as libc::c_int;
    init_lib_wrapper(
        (*::std::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"getnameinfo\0"))
            .as_ptr(),
    );
    if proxychains_resolver == 0 {
        ret = true_getnameinfo.unwrap()(sa, salen, host, hostlen, serv, servlen, flags);
    } else {
        if hostlen != 0 {
            inet_ntop(
                2 as libc::c_int,
                &mut (*(sa as *mut sockaddr_in)).sin_addr as *mut in_addr
                    as *mut libc::c_uchar as *const libc::c_void,
                ip_buf.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong as socklen_t,
            );
            strncpy(host, ip_buf.as_mut_ptr(), hostlen as libc::c_ulong);
        }
        if servlen != 0 {
            snprintf(
                serv,
                servlen as libc::c_ulong,
                b"%d\0" as *const u8 as *const libc::c_char,
                __bswap_16((*(sa as *mut sockaddr_in)).sin_port) as libc::c_int,
            );
        }
    }
    return ret;
}
pub unsafe extern "C" fn gethostbyaddr(
    mut addr: *const libc::c_void,
    mut len: socklen_t,
    mut type_0: libc::c_int,
) -> *mut hostent {
    static mut buf: [libc::c_char; 16] = [0; 16];
    static mut ipv4: [libc::c_char; 4] = [0; 4];
    static mut list: [*mut libc::c_char; 2] = [0 as *const libc::c_char
        as *mut libc::c_char; 2];
    static mut he: hostent = hostent {
        h_name: 0 as *const libc::c_char as *mut libc::c_char,
        h_aliases: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        h_addrtype: 0,
        h_length: 0,
        h_addr_list: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
    };
    init_lib_wrapper(
        (*::std::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"gethostbyaddr\0"))
            .as_ptr(),
    );
    if proxychains_resolver == 0 {
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
        he.h_aliases = 0 as *mut *mut libc::c_char;
        he.h_length = 4 as libc::c_int;
        inet_ntop(
            2 as libc::c_int,
            addr,
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong as socklen_t,
        );
        return &mut he;
    };
}
