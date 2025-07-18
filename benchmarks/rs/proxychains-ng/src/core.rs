use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn vsnprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn perror(__s: *const libc::c_char);
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn pipe2(__pipedes: *mut libc::c_int, __flags: libc::c_int) -> libc::c_int;
    fn usleep(__useconds: __useconds_t) -> libc::c_int;
    fn dup2(__fd: libc::c_int, __fd2: libc::c_int) -> libc::c_int;
    fn execlp(
        __file: *const libc::c_char,
        __arg: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn fork() -> __pid_t;
    fn gethostname(__name: *mut libc::c_char, __len: size_t) -> libc::c_int;
    fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
    fn rand() -> libc::c_int;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
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
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __errno_location() -> *mut libc::c_int;
    fn socket(
        __domain: libc::c_int,
        __type: libc::c_int,
        __protocol: libc::c_int,
    ) -> libc::c_int;
    fn send(
        __fd: libc::c_int,
        __buf: *const libc::c_void,
        __n: size_t,
        __flags: libc::c_int,
    ) -> ssize_t;
    fn getsockopt(
        __fd: libc::c_int,
        __level: libc::c_int,
        __optname: libc::c_int,
        __optval: *mut libc::c_void,
        __optlen: *mut socklen_t,
    ) -> libc::c_int;
    fn ntohs(__netshort: uint16_t) -> uint16_t;
    fn htons(__hostshort: uint16_t) -> uint16_t;
    fn gethostent() -> *mut hostent;
    fn getservbyname(
        __name: *const libc::c_char,
        __proto: *const libc::c_char,
    ) -> *mut servent;
    fn getservbyname_r(
        __name: *const libc::c_char,
        __proto: *const libc::c_char,
        __result_buf: *mut servent,
        __buf: *mut libc::c_char,
        __buflen: size_t,
        __result: *mut *mut servent,
    ) -> libc::c_int;
    fn inet_addr(__cp: *const libc::c_char) -> in_addr_t;
    fn inet_ntoa(__in: in_addr) -> *mut libc::c_char;
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
    fn inet_aton(__cp: *const libc::c_char, __inp: *mut in_addr) -> libc::c_int;
    fn poll(__fds: *mut pollfd, __nfds: nfds_t, __timeout: libc::c_int) -> libc::c_int;
    fn waitpid(
        __pid: __pid_t,
        __stat_loc: *mut libc::c_int,
        __options: libc::c_int,
    ) -> __pid_t;
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
    fn gettimeofday(__tv: *mut timeval, __tz: *mut libc::c_void) -> libc::c_int;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    static mut true_connect: connect_t;
    fn pc_isnumericipv4(ipstring: *const libc::c_char) -> libc::c_int;
    fn rdns_get_host_for_ip(ip: ip_type4, readbuf: *mut libc::c_char) -> size_t;
    fn rdns_get_ip_for_host(host: *mut libc::c_char, len: size_t) -> ip_type4;
    static mut proxychains_resolver: dns_lookup_flavor;
    fn pthread_mutex_destroy(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> libc::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    static mut tcp_read_time_out: libc::c_int;
    static mut tcp_connect_time_out: libc::c_int;
    static mut proxychains_quiet_mode: libc::c_int;
    static mut proxychains_proxy_offset: libc::c_uint;
    static mut remote_dns_subnet: libc::c_uint;
    fn hostsreader_get_numeric_ip_for_name(name: *const libc::c_char) -> ip_type4;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type size_t = libc::c_ulong;
pub type va_list = __builtin_va_list;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __useconds_t = libc::c_uint;
pub type __suseconds_t = libc::c_long;
pub type __ssize_t = libc::c_long;
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
pub type pid_t = __pid_t;
pub type socklen_t = __socklen_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
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
pub union pthread_mutexattr_t {
    pub __size: [libc::c_char; 4],
    pub __align: libc::c_int,
}
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
pub type C2RustUnnamed_0 = libc::c_uint;
pub const IPPROTO_MAX: C2RustUnnamed_0 = 256;
pub const IPPROTO_RAW: C2RustUnnamed_0 = 255;
pub const IPPROTO_MPLS: C2RustUnnamed_0 = 137;
pub const IPPROTO_UDPLITE: C2RustUnnamed_0 = 136;
pub const IPPROTO_SCTP: C2RustUnnamed_0 = 132;
pub const IPPROTO_COMP: C2RustUnnamed_0 = 108;
pub const IPPROTO_PIM: C2RustUnnamed_0 = 103;
pub const IPPROTO_ENCAP: C2RustUnnamed_0 = 98;
pub const IPPROTO_BEETPH: C2RustUnnamed_0 = 94;
pub const IPPROTO_MTP: C2RustUnnamed_0 = 92;
pub const IPPROTO_AH: C2RustUnnamed_0 = 51;
pub const IPPROTO_ESP: C2RustUnnamed_0 = 50;
pub const IPPROTO_GRE: C2RustUnnamed_0 = 47;
pub const IPPROTO_RSVP: C2RustUnnamed_0 = 46;
pub const IPPROTO_IPV6: C2RustUnnamed_0 = 41;
pub const IPPROTO_DCCP: C2RustUnnamed_0 = 33;
pub const IPPROTO_TP: C2RustUnnamed_0 = 29;
pub const IPPROTO_IDP: C2RustUnnamed_0 = 22;
pub const IPPROTO_UDP: C2RustUnnamed_0 = 17;
pub const IPPROTO_PUP: C2RustUnnamed_0 = 12;
pub const IPPROTO_EGP: C2RustUnnamed_0 = 8;
pub const IPPROTO_TCP: C2RustUnnamed_0 = 6;
pub const IPPROTO_IPIP: C2RustUnnamed_0 = 4;
pub const IPPROTO_IGMP: C2RustUnnamed_0 = 2;
pub const IPPROTO_ICMP: C2RustUnnamed_0 = 1;
pub const IPPROTO_IP: C2RustUnnamed_0 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hostent {
    pub h_name: *mut libc::c_char,
    pub h_aliases: *mut *mut libc::c_char,
    pub h_addrtype: libc::c_int,
    pub h_length: libc::c_int,
    pub h_addr_list: *mut *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct servent {
    pub s_name: *mut libc::c_char,
    pub s_aliases: *mut *mut libc::c_char,
    pub s_port: libc::c_int,
    pub s_proto: *mut libc::c_char,
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
pub type nfds_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pollfd {
    pub fd: libc::c_int,
    pub events: libc::c_short,
    pub revents: libc::c_short,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union ip_type4 {
    pub octet: [libc::c_uchar; 4],
    pub as_int: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ip_type {
    pub addr: C2RustUnnamed_1,
    pub is_v6: libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub v4: ip_type4,
    pub v6: [libc::c_uchar; 16],
}
pub type C2RustUnnamed_2 = libc::c_uint;
pub const BLOCKED: C2RustUnnamed_2 = 5;
pub const CHAIN_EMPTY: C2RustUnnamed_2 = 4;
pub const CHAIN_DOWN: C2RustUnnamed_2 = 3;
pub const SOCKET_ERROR: C2RustUnnamed_2 = 2;
pub const MEMORY_FAIL: C2RustUnnamed_2 = 1;
pub const SUCCESS: C2RustUnnamed_2 = 0;
pub type proxy_type = libc::c_uint;
pub const RAW_TYPE: proxy_type = 3;
pub const SOCKS5_TYPE: proxy_type = 2;
pub const SOCKS4_TYPE: proxy_type = 1;
pub const HTTP_TYPE: proxy_type = 0;
pub type chain_type = libc::c_uint;
pub const ROUND_ROBIN_TYPE: chain_type = 3;
pub const RANDOM_TYPE: chain_type = 2;
pub const STRICT_TYPE: chain_type = 1;
pub const DYNAMIC_TYPE: chain_type = 0;
pub type proxy_state = libc::c_uint;
pub const BUSY_STATE: proxy_state = 3;
pub const BLOCKED_STATE: proxy_state = 2;
pub const DOWN_STATE: proxy_state = 1;
pub const PLAY_STATE: proxy_state = 0;
pub type select_type = libc::c_uint;
pub const FIFOLY: select_type = 1;
pub const RANDOMLY: select_type = 0;
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
pub const DNSLF_RDNS_START: dns_lookup_flavor = 2;
pub type dns_lookup_flavor = libc::c_uint;
pub const DNSLF_RDNS_DAEMON: dns_lookup_flavor = 3;
pub const DNSLF_RDNS_THREAD: dns_lookup_flavor = 2;
pub const DNSLF_FORKEXEC: dns_lookup_flavor = 1;
pub const DNSLF_LIBC: dns_lookup_flavor = 0;
pub type connect_t = Option::<
    unsafe extern "C" fn(libc::c_int, *const sockaddr, socklen_t) -> libc::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gethostbyname_data {
    pub hostent_space: hostent,
    pub resolved_addr: in_addr_t,
    pub resolved_addr_p: [*mut libc::c_char; 2],
    pub addr_name: [libc::c_char; 256],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct addrinfo_data {
    pub addrinfo_space: addrinfo,
    pub sockaddr_space: sockaddr_storage,
    pub addr_name: [libc::c_char; 256],
}
unsafe extern "C" fn poll_retry(
    mut fds: *mut pollfd,
    mut nfsd: nfds_t,
    mut timeout: libc::c_int,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut time_remain: libc::c_int = timeout;
    let mut time_elapsed: libc::c_int = 0 as libc::c_int;
    let mut start_time: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    gettimeofday(&mut start_time, 0 as *mut libc::c_void);
    loop {
        ret = poll(fds, nfsd, time_remain);
        gettimeofday(&mut tv, 0 as *mut libc::c_void);
        time_elapsed = ((tv.tv_sec - start_time.tv_sec)
            * 1000 as libc::c_int as libc::c_long
            + (tv.tv_usec - start_time.tv_usec) / 1000 as libc::c_int as libc::c_long)
            as libc::c_int;
        time_remain = timeout - time_elapsed;
        if !(ret == -(1 as libc::c_int) && *__errno_location() == 4 as libc::c_int
            && time_remain > 0 as libc::c_int)
        {
            break;
        }
    }
    return ret;
}
unsafe extern "C" fn encode_base_64(
    mut src: *mut libc::c_char,
    mut dest: *mut libc::c_char,
    mut max_len: libc::c_int,
) {
    static mut base64: [libc::c_char; 65] = unsafe {
        *::std::mem::transmute::<
            &[u8; 65],
            &[libc::c_char; 65],
        >(b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/\0")
    };
    let mut n: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    l = strlen(src) as libc::c_int;
    max_len = (max_len - 1 as libc::c_int) / 4 as libc::c_int;
    i = 0 as libc::c_int;
    while i < max_len {
        match l {
            0 => {}
            1 => {
                n = (*src.offset(0 as libc::c_int as isize) as libc::c_int)
                    << 16 as libc::c_int;
                let fresh0 = dest;
                dest = dest.offset(1);
                *fresh0 = base64[(n >> 18 as libc::c_int & 0o77 as libc::c_int)
                    as usize];
                let fresh1 = dest;
                dest = dest.offset(1);
                *fresh1 = base64[(n >> 12 as libc::c_int & 0o77 as libc::c_int)
                    as usize];
                let fresh2 = dest;
                dest = dest.offset(1);
                *fresh2 = '=' as i32 as libc::c_char;
                let fresh3 = dest;
                dest = dest.offset(1);
                *fresh3 = '=' as i32 as libc::c_char;
            }
            2 => {
                n = (*src.offset(0 as libc::c_int as isize) as libc::c_int)
                    << 16 as libc::c_int
                    | (*src.offset(1 as libc::c_int as isize) as libc::c_int)
                        << 8 as libc::c_int;
                let fresh4 = dest;
                dest = dest.offset(1);
                *fresh4 = base64[(n >> 18 as libc::c_int & 0o77 as libc::c_int)
                    as usize];
                let fresh5 = dest;
                dest = dest.offset(1);
                *fresh5 = base64[(n >> 12 as libc::c_int & 0o77 as libc::c_int)
                    as usize];
                let fresh6 = dest;
                dest = dest.offset(1);
                *fresh6 = base64[(n >> 6 as libc::c_int & 0o77 as libc::c_int) as usize];
                let fresh7 = dest;
                dest = dest.offset(1);
                *fresh7 = '=' as i32 as libc::c_char;
            }
            _ => {
                n = (*src.offset(0 as libc::c_int as isize) as libc::c_int)
                    << 16 as libc::c_int
                    | (*src.offset(1 as libc::c_int as isize) as libc::c_int)
                        << 8 as libc::c_int
                    | *src.offset(2 as libc::c_int as isize) as libc::c_int;
                let fresh8 = dest;
                dest = dest.offset(1);
                *fresh8 = base64[(n >> 18 as libc::c_int & 0o77 as libc::c_int)
                    as usize];
                let fresh9 = dest;
                dest = dest.offset(1);
                *fresh9 = base64[(n >> 12 as libc::c_int & 0o77 as libc::c_int)
                    as usize];
                let fresh10 = dest;
                dest = dest.offset(1);
                *fresh10 = base64[(n >> 6 as libc::c_int & 0o77 as libc::c_int)
                    as usize];
                let fresh11 = dest;
                dest = dest.offset(1);
                *fresh11 = base64[(n & 0o77 as libc::c_int) as usize];
            }
        }
        if l < 3 as libc::c_int {
            break;
        }
        i += 1;
        i;
        src = src.offset(3 as libc::c_int as isize);
        l -= 3 as libc::c_int;
    }
    let fresh12 = dest;
    dest = dest.offset(1);
    *fresh12 = 0 as libc::c_int as libc::c_char;
}
pub unsafe extern "C" fn proxychains_write_log(
    mut str: *mut libc::c_char,
    mut args: ...
) {
    let mut buff: [libc::c_char; 4096] = [0; 4096];
    let mut arglist: ::std::ffi::VaListImpl;
    if proxychains_quiet_mode == 0 {
        arglist = args.clone();
        vsnprintf(
            buff.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong,
            str,
            arglist.as_va_list(),
        );
        fprintf(stderr, b"%s\0" as *const u8 as *const libc::c_char, buff.as_mut_ptr());
        fflush(stderr);
    }
}
unsafe extern "C" fn write_n_bytes(
    mut fd: libc::c_int,
    mut buff: *mut libc::c_char,
    mut size: size_t,
) -> libc::c_int {
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut wrote: size_t = 0 as libc::c_int as size_t;
    loop {
        i = write(
            fd,
            &mut *buff.offset(wrote as isize) as *mut libc::c_char
                as *const libc::c_void,
            size.wrapping_sub(wrote),
        ) as libc::c_int;
        if i <= 0 as libc::c_int {
            return i;
        }
        wrote = (wrote as libc::c_ulong).wrapping_add(i as libc::c_ulong) as size_t
            as size_t;
        if wrote == size {
            return wrote as libc::c_int;
        }
    };
}
unsafe extern "C" fn read_n_bytes(
    mut fd: libc::c_int,
    mut buff: *mut libc::c_char,
    mut size: size_t,
) -> libc::c_int {
    let mut ready: libc::c_int = 0;
    let mut i: size_t = 0;
    let mut pfd: [pollfd; 1] = [pollfd {
        fd: 0,
        events: 0,
        revents: 0,
    }; 1];
    pfd[0 as libc::c_int as usize].fd = fd;
    pfd[0 as libc::c_int as usize].events = 0x1 as libc::c_int as libc::c_short;
    i = 0 as libc::c_int as size_t;
    while i < size {
        pfd[0 as libc::c_int as usize].revents = 0 as libc::c_int as libc::c_short;
        ready = poll_retry(
            pfd.as_mut_ptr(),
            1 as libc::c_int as nfds_t,
            tcp_read_time_out,
        );
        if ready != 1 as libc::c_int
            || pfd[0 as libc::c_int as usize].revents as libc::c_int & 0x1 as libc::c_int
                == 0
            || 1 as libc::c_int as libc::c_long
                != read(
                    fd,
                    &mut *buff.offset(i as isize) as *mut libc::c_char
                        as *mut libc::c_void,
                    1 as libc::c_int as size_t,
                )
        {
            return -(1 as libc::c_int);
        }
        i = i.wrapping_add(1);
        i;
    }
    return size as libc::c_int;
}
unsafe extern "C" fn timed_connect(
    mut sock: libc::c_int,
    mut addr: *const sockaddr,
    mut len: socklen_t,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut value: libc::c_int = 0;
    let mut value_len: socklen_t = 0;
    let mut pfd: [pollfd; 1] = [pollfd {
        fd: 0,
        events: 0,
        revents: 0,
    }; 1];
    pfd[0 as libc::c_int as usize].fd = sock;
    pfd[0 as libc::c_int as usize].events = 0x4 as libc::c_int as libc::c_short;
    fcntl(sock, 4 as libc::c_int, 0o4000 as libc::c_int);
    ret = true_connect.unwrap()(sock, addr, len);
    if ret == -(1 as libc::c_int) && *__errno_location() == 115 as libc::c_int {
        ret = poll_retry(
            pfd.as_mut_ptr(),
            1 as libc::c_int as nfds_t,
            tcp_connect_time_out,
        );
        if ret == 1 as libc::c_int {
            value_len = ::std::mem::size_of::<socklen_t>() as libc::c_ulong as socklen_t;
            getsockopt(
                sock,
                1 as libc::c_int,
                4 as libc::c_int,
                &mut value as *mut libc::c_int as *mut libc::c_void,
                &mut value_len,
            );
            if value == 0 {
                ret = 0 as libc::c_int;
            } else {
                ret = -(1 as libc::c_int);
            }
        } else {
            ret = -(1 as libc::c_int);
        }
    } else if ret != 0 as libc::c_int {
        ret = -(1 as libc::c_int);
    }
    fcntl(sock, 4 as libc::c_int, (0o4000 as libc::c_int == 0) as libc::c_int);
    return ret;
}
unsafe extern "C" fn tunnel_to(
    mut sock: libc::c_int,
    mut ip: ip_type,
    mut port: libc::c_ushort,
    mut pt: proxy_type,
    mut user: *mut libc::c_char,
    mut pass: *mut libc::c_char,
) -> libc::c_int {
    let mut ulen: size_t = 0;
    let mut passlen: size_t = 0;
    let mut len: libc::c_int = 0;
    let mut buff: [libc::c_uchar; 1024] = [0; 1024];
    let mut ip_buf: [libc::c_char; 46] = [0; 46];
    let mut v6: libc::c_int = 0;
    let mut current_block: u64;
    let mut dns_name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut hostnamebuf: [libc::c_char; 256] = [0; 256];
    let mut dns_len: size_t = 0 as libc::c_int as size_t;
    if ip.is_v6 == 0
        && proxychains_resolver as libc::c_uint
            >= DNSLF_RDNS_START as libc::c_int as libc::c_uint
        && ip.addr.v4.octet[0 as libc::c_int as usize] as libc::c_uint
            == remote_dns_subnet
    {
        dns_len = rdns_get_host_for_ip(ip.addr.v4, hostnamebuf.as_mut_ptr());
        if dns_len == 0 {
            current_block = 6813626912928578684;
        } else {
            dns_name = hostnamebuf.as_mut_ptr();
            current_block = 11650488183268122163;
        }
    } else {
        current_block = 11650488183268122163;
    }
    match current_block {
        11650488183268122163 => {
            ulen = strlen(user);
            passlen = strlen(pass);
            if ulen > 0xff as libc::c_int as libc::c_ulong
                || passlen > 0xff as libc::c_int as libc::c_ulong
                || dns_len > 0xff as libc::c_int as libc::c_ulong
            {
                proxychains_write_log(
                    b"[proxychains] error: maximum size of 255 for user/pass or domain name!\n\0"
                        as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
            } else {
                len = 0;
                buff = [0; 1024];
                ip_buf = [0; 46];
                v6 = ip.is_v6 as libc::c_int;
                match pt as libc::c_uint {
                    3 => {
                        current_block = 12304796239473295489;
                        match current_block {
                            12304796239473295489 => return SUCCESS as libc::c_int,
                            14072441030219150333 => {
                                if v6 != 0 {
                                    proxychains_write_log(
                                        b"[proxychains] error: SOCKS4 doesn't support ipv6 addresses\n\0"
                                            as *const u8 as *const libc::c_char as *mut libc::c_char,
                                    );
                                } else {
                                    buff[0 as libc::c_int
                                        as usize] = 4 as libc::c_int as libc::c_uchar;
                                    buff[1 as libc::c_int
                                        as usize] = 1 as libc::c_int as libc::c_uchar;
                                    memcpy(
                                        &mut *buff.as_mut_ptr().offset(2 as libc::c_int as isize)
                                            as *mut libc::c_uchar as *mut libc::c_void,
                                        &mut port as *mut libc::c_ushort as *const libc::c_void,
                                        2 as libc::c_int as libc::c_ulong,
                                    );
                                    if dns_len != 0 {
                                        ip
                                            .addr
                                            .v4
                                            .octet[0 as libc::c_int
                                            as usize] = 0 as libc::c_int as libc::c_uchar;
                                        ip
                                            .addr
                                            .v4
                                            .octet[1 as libc::c_int
                                            as usize] = 0 as libc::c_int as libc::c_uchar;
                                        ip
                                            .addr
                                            .v4
                                            .octet[2 as libc::c_int
                                            as usize] = 0 as libc::c_int as libc::c_uchar;
                                        ip
                                            .addr
                                            .v4
                                            .octet[3 as libc::c_int
                                            as usize] = 1 as libc::c_int as libc::c_uchar;
                                    }
                                    memcpy(
                                        &mut *buff.as_mut_ptr().offset(4 as libc::c_int as isize)
                                            as *mut libc::c_uchar as *mut libc::c_void,
                                        &mut ip.addr.v4 as *mut ip_type4 as *const libc::c_void,
                                        4 as libc::c_int as libc::c_ulong,
                                    );
                                    len = ulen.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                        as libc::c_int;
                                    if len > 1 as libc::c_int {
                                        memcpy(
                                            &mut *buff.as_mut_ptr().offset(8 as libc::c_int as isize)
                                                as *mut libc::c_uchar as *mut libc::c_void,
                                            user as *const libc::c_void,
                                            len as libc::c_ulong,
                                        );
                                    } else {
                                        buff[8 as libc::c_int
                                            as usize] = 0 as libc::c_int as libc::c_uchar;
                                    }
                                    if dns_len != 0 {
                                        memcpy(
                                            &mut *buff
                                                .as_mut_ptr()
                                                .offset((8 as libc::c_int + len) as isize)
                                                as *mut libc::c_uchar as *mut libc::c_void,
                                            dns_name as *const libc::c_void,
                                            dns_len.wrapping_add(1 as libc::c_int as libc::c_ulong),
                                        );
                                        len = (len as libc::c_ulong)
                                            .wrapping_add(
                                                dns_len.wrapping_add(1 as libc::c_int as libc::c_ulong),
                                            ) as libc::c_int as libc::c_int;
                                    }
                                    if !(len + 8 as libc::c_int
                                        != write_n_bytes(
                                            sock,
                                            buff.as_mut_ptr() as *mut libc::c_char,
                                            (8 as libc::c_int + len) as size_t,
                                        ))
                                    {
                                        if !(8 as libc::c_int
                                            != read_n_bytes(
                                                sock,
                                                buff.as_mut_ptr() as *mut libc::c_char,
                                                8 as libc::c_int as size_t,
                                            ))
                                        {
                                            if buff[0 as libc::c_int as usize] as libc::c_int
                                                != 0 as libc::c_int
                                                || buff[1 as libc::c_int as usize] as libc::c_int
                                                    != 90 as libc::c_int
                                            {
                                                return BLOCKED as libc::c_int;
                                            }
                                            return SUCCESS as libc::c_int;
                                        }
                                    }
                                }
                            }
                            8457315219000651999 => {
                                if dns_len == 0 {
                                    if (inet_ntop(
                                        if v6 != 0 { 10 as libc::c_int } else { 2 as libc::c_int },
                                        (ip.addr.v6).as_mut_ptr() as *const libc::c_void,
                                        ip_buf.as_mut_ptr(),
                                        ::std::mem::size_of::<[libc::c_char; 46]>() as libc::c_ulong
                                            as socklen_t,
                                    ))
                                        .is_null()
                                    {
                                        proxychains_write_log(
                                            b"[proxychains] error: ip address conversion failed\n\0"
                                                as *const u8 as *const libc::c_char as *mut libc::c_char,
                                        );
                                        current_block = 6813626912928578684;
                                    } else {
                                        dns_name = ip_buf.as_mut_ptr();
                                        current_block = 5783071609795492627;
                                    }
                                } else {
                                    current_block = 5783071609795492627;
                                }
                                match current_block {
                                    6813626912928578684 => {}
                                    _ => {
                                        let mut src: [libc::c_char; 512] = [0; 512];
                                        let mut dst: [libc::c_char; 2048] = [0; 2048];
                                        if ulen != 0 {
                                            snprintf(
                                                src.as_mut_ptr(),
                                                ::std::mem::size_of::<[libc::c_char; 512]>()
                                                    as libc::c_ulong,
                                                b"%s:%s\0" as *const u8 as *const libc::c_char,
                                                user,
                                                pass,
                                            );
                                            encode_base_64(
                                                src.as_mut_ptr(),
                                                dst.as_mut_ptr(),
                                                ::std::mem::size_of::<[libc::c_char; 2048]>()
                                                    as libc::c_ulong as libc::c_int,
                                            );
                                        } else {
                                            dst[0 as libc::c_int
                                                as usize] = 0 as libc::c_int as libc::c_char;
                                        }
                                        let mut hs_port: uint16_t = ntohs(port);
                                        len = snprintf(
                                            buff.as_mut_ptr() as *mut libc::c_char,
                                            ::std::mem::size_of::<[libc::c_uchar; 1024]>()
                                                as libc::c_ulong,
                                            b"CONNECT %s:%d HTTP/1.0\r\nHost: %s:%d\r\n%s%s%s\r\n\0"
                                                as *const u8 as *const libc::c_char,
                                            dns_name,
                                            hs_port as libc::c_int,
                                            dns_name,
                                            hs_port as libc::c_int,
                                            if ulen != 0 {
                                                b"Proxy-Authorization: Basic \0" as *const u8
                                                    as *const libc::c_char
                                            } else {
                                                dst.as_mut_ptr() as *const libc::c_char
                                            },
                                            dst.as_mut_ptr(),
                                            if ulen != 0 {
                                                b"\r\n\0" as *const u8 as *const libc::c_char
                                            } else {
                                                dst.as_mut_ptr() as *const libc::c_char
                                            },
                                        );
                                        if !(len < 0 as libc::c_int
                                            || len as libc::c_long
                                                != send(
                                                    sock,
                                                    buff.as_mut_ptr() as *const libc::c_void,
                                                    len as size_t,
                                                    0 as libc::c_int,
                                                ))
                                        {
                                            len = 0 as libc::c_int;
                                            loop {
                                                if !(len < 1024 as libc::c_int) {
                                                    current_block = 8704759739624374314;
                                                    break;
                                                }
                                                if !(1 as libc::c_int
                                                    == read_n_bytes(
                                                        sock,
                                                        buff.as_mut_ptr().offset(len as isize) as *mut libc::c_char,
                                                        1 as libc::c_int as size_t,
                                                    ))
                                                {
                                                    current_block = 6813626912928578684;
                                                    break;
                                                }
                                                len += 1;
                                                len;
                                                if len > 4 as libc::c_int
                                                    && buff[(len - 1 as libc::c_int) as usize] as libc::c_int
                                                        == '\n' as i32
                                                    && buff[(len - 2 as libc::c_int) as usize] as libc::c_int
                                                        == '\r' as i32
                                                    && buff[(len - 3 as libc::c_int) as usize] as libc::c_int
                                                        == '\n' as i32
                                                    && buff[(len - 4 as libc::c_int) as usize] as libc::c_int
                                                        == '\r' as i32
                                                {
                                                    current_block = 8704759739624374314;
                                                    break;
                                                }
                                            }
                                            match current_block {
                                                6813626912928578684 => {}
                                                _ => {
                                                    if len == 1024 as libc::c_int
                                                        || !(buff[9 as libc::c_int as usize] as libc::c_int
                                                            == '2' as i32
                                                            && buff[10 as libc::c_int as usize] as libc::c_int
                                                                == '0' as i32
                                                            && buff[11 as libc::c_int as usize] as libc::c_int
                                                                == '0' as i32)
                                                    {
                                                        return BLOCKED as libc::c_int;
                                                    }
                                                    return SUCCESS as libc::c_int;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            _ => {
                                let mut n_methods: libc::c_int = if ulen != 0 {
                                    2 as libc::c_int
                                } else {
                                    1 as libc::c_int
                                };
                                buff[0 as libc::c_int
                                    as usize] = 5 as libc::c_int as libc::c_uchar;
                                buff[1 as libc::c_int
                                    as usize] = n_methods as libc::c_uchar;
                                buff[2 as libc::c_int
                                    as usize] = 0 as libc::c_int as libc::c_uchar;
                                if ulen != 0 {
                                    buff[3 as libc::c_int
                                        as usize] = 2 as libc::c_int as libc::c_uchar;
                                }
                                if !(2 as libc::c_int + n_methods
                                    != write_n_bytes(
                                        sock,
                                        buff.as_mut_ptr() as *mut libc::c_char,
                                        (2 as libc::c_int + n_methods) as size_t,
                                    ))
                                {
                                    if !(2 as libc::c_int
                                        != read_n_bytes(
                                            sock,
                                            buff.as_mut_ptr() as *mut libc::c_char,
                                            2 as libc::c_int as size_t,
                                        ))
                                    {
                                        if buff[0 as libc::c_int as usize] as libc::c_int
                                            != 5 as libc::c_int
                                            || buff[1 as libc::c_int as usize] as libc::c_int
                                                != 0 as libc::c_int
                                                && buff[1 as libc::c_int as usize] as libc::c_int
                                                    != 2 as libc::c_int
                                        {
                                            if buff[0 as libc::c_int as usize] as libc::c_int
                                                == 5 as libc::c_int
                                                && buff[1 as libc::c_int as usize] as libc::c_int
                                                    == 0xff as libc::c_int
                                            {
                                                return BLOCKED as libc::c_int;
                                            }
                                        } else {
                                            if buff[1 as libc::c_int as usize] as libc::c_int
                                                == 2 as libc::c_int
                                            {
                                                let mut in_0: [libc::c_char; 2] = [0; 2];
                                                let mut out: [libc::c_char; 515] = [0; 515];
                                                let mut cur: *mut libc::c_char = out.as_mut_ptr();
                                                let mut c: size_t = 0;
                                                let fresh13 = cur;
                                                cur = cur.offset(1);
                                                *fresh13 = 1 as libc::c_int as libc::c_char;
                                                c = ulen & 0xff as libc::c_int as libc::c_ulong;
                                                let fresh14 = cur;
                                                cur = cur.offset(1);
                                                *fresh14 = c as libc::c_char;
                                                memcpy(
                                                    cur as *mut libc::c_void,
                                                    user as *const libc::c_void,
                                                    c,
                                                );
                                                cur = cur.offset(c as isize);
                                                c = passlen & 0xff as libc::c_int as libc::c_ulong;
                                                let fresh15 = cur;
                                                cur = cur.offset(1);
                                                *fresh15 = c as libc::c_char;
                                                memcpy(
                                                    cur as *mut libc::c_void,
                                                    pass as *const libc::c_void,
                                                    c,
                                                );
                                                cur = cur.offset(c as isize);
                                                if cur.offset_from(out.as_mut_ptr()) as libc::c_long
                                                    != write_n_bytes(
                                                        sock,
                                                        out.as_mut_ptr(),
                                                        cur.offset_from(out.as_mut_ptr()) as libc::c_long as size_t,
                                                    ) as libc::c_long
                                                {
                                                    current_block = 6813626912928578684;
                                                } else if 2 as libc::c_int
                                                    != read_n_bytes(
                                                        sock,
                                                        in_0.as_mut_ptr(),
                                                        2 as libc::c_int as size_t,
                                                    )
                                                {
                                                    current_block = 6813626912928578684;
                                                } else if !(in_0[0 as libc::c_int as usize] as libc::c_int
                                                    == 5 as libc::c_int
                                                    || in_0[0 as libc::c_int as usize] as libc::c_int
                                                        == 1 as libc::c_int)
                                                {
                                                    current_block = 6813626912928578684;
                                                } else {
                                                    if in_0[1 as libc::c_int as usize] as libc::c_int
                                                        != 0 as libc::c_int
                                                    {
                                                        return BLOCKED as libc::c_int;
                                                    }
                                                    current_block = 8464383504555462953;
                                                }
                                            } else {
                                                current_block = 8464383504555462953;
                                            }
                                            match current_block {
                                                6813626912928578684 => {}
                                                _ => {
                                                    let mut buff_iter: libc::c_int = 0 as libc::c_int;
                                                    let fresh16 = buff_iter;
                                                    buff_iter = buff_iter + 1;
                                                    buff[fresh16 as usize] = 5 as libc::c_int as libc::c_uchar;
                                                    let fresh17 = buff_iter;
                                                    buff_iter = buff_iter + 1;
                                                    buff[fresh17 as usize] = 1 as libc::c_int as libc::c_uchar;
                                                    let fresh18 = buff_iter;
                                                    buff_iter = buff_iter + 1;
                                                    buff[fresh18 as usize] = 0 as libc::c_int as libc::c_uchar;
                                                    if dns_len == 0 {
                                                        let fresh19 = buff_iter;
                                                        buff_iter = buff_iter + 1;
                                                        buff[fresh19
                                                            as usize] = (if v6 != 0 {
                                                            4 as libc::c_int
                                                        } else {
                                                            1 as libc::c_int
                                                        }) as libc::c_uchar;
                                                        memcpy(
                                                            buff.as_mut_ptr().offset(buff_iter as isize)
                                                                as *mut libc::c_void,
                                                            (ip.addr.v6).as_mut_ptr() as *const libc::c_void,
                                                            (if v6 != 0 { 16 as libc::c_int } else { 4 as libc::c_int })
                                                                as libc::c_ulong,
                                                        );
                                                        buff_iter
                                                            += if v6 != 0 {
                                                                16 as libc::c_int
                                                            } else {
                                                                4 as libc::c_int
                                                            };
                                                    } else {
                                                        let fresh20 = buff_iter;
                                                        buff_iter = buff_iter + 1;
                                                        buff[fresh20 as usize] = 3 as libc::c_int as libc::c_uchar;
                                                        let fresh21 = buff_iter;
                                                        buff_iter = buff_iter + 1;
                                                        buff[fresh21
                                                            as usize] = (dns_len & 0xff as libc::c_int as libc::c_ulong)
                                                            as libc::c_uchar;
                                                        memcpy(
                                                            buff.as_mut_ptr().offset(buff_iter as isize)
                                                                as *mut libc::c_void,
                                                            dns_name as *const libc::c_void,
                                                            dns_len,
                                                        );
                                                        buff_iter = (buff_iter as libc::c_ulong)
                                                            .wrapping_add(dns_len) as libc::c_int as libc::c_int;
                                                    }
                                                    memcpy(
                                                        buff.as_mut_ptr().offset(buff_iter as isize)
                                                            as *mut libc::c_void,
                                                        &mut port as *mut libc::c_ushort as *const libc::c_void,
                                                        2 as libc::c_int as libc::c_ulong,
                                                    );
                                                    buff_iter += 2 as libc::c_int;
                                                    if !(buff_iter
                                                        != write_n_bytes(
                                                            sock,
                                                            buff.as_mut_ptr() as *mut libc::c_char,
                                                            buff_iter as size_t,
                                                        ))
                                                    {
                                                        if !(4 as libc::c_int
                                                            != read_n_bytes(
                                                                sock,
                                                                buff.as_mut_ptr() as *mut libc::c_char,
                                                                4 as libc::c_int as size_t,
                                                            ))
                                                        {
                                                            if !(buff[0 as libc::c_int as usize] as libc::c_int
                                                                != 5 as libc::c_int
                                                                || buff[1 as libc::c_int as usize] as libc::c_int
                                                                    != 0 as libc::c_int)
                                                            {
                                                                match buff[3 as libc::c_int as usize] as libc::c_int {
                                                                    1 => {
                                                                        current_block = 8739749648707081367;
                                                                        match current_block {
                                                                            8331409615400467618 => {
                                                                                len = 16 as libc::c_int;
                                                                                current_block = 4899250571165509867;
                                                                            }
                                                                            8739749648707081367 => {
                                                                                len = 4 as libc::c_int;
                                                                                current_block = 4899250571165509867;
                                                                            }
                                                                            _ => {
                                                                                len = 0 as libc::c_int;
                                                                                if 1 as libc::c_int
                                                                                    != read_n_bytes(
                                                                                        sock,
                                                                                        &mut len as *mut libc::c_int as *mut libc::c_char,
                                                                                        1 as libc::c_int as size_t,
                                                                                    )
                                                                                {
                                                                                    current_block = 6813626912928578684;
                                                                                } else {
                                                                                    current_block = 4899250571165509867;
                                                                                }
                                                                            }
                                                                        }
                                                                        match current_block {
                                                                            6813626912928578684 => {}
                                                                            _ => {
                                                                                if !(len + 2 as libc::c_int
                                                                                    != read_n_bytes(
                                                                                        sock,
                                                                                        buff.as_mut_ptr() as *mut libc::c_char,
                                                                                        (len + 2 as libc::c_int) as size_t,
                                                                                    ))
                                                                                {
                                                                                    return SUCCESS as libc::c_int;
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                    4 => {
                                                                        current_block = 8331409615400467618;
                                                                        match current_block {
                                                                            8331409615400467618 => {
                                                                                len = 16 as libc::c_int;
                                                                                current_block = 4899250571165509867;
                                                                            }
                                                                            8739749648707081367 => {
                                                                                len = 4 as libc::c_int;
                                                                                current_block = 4899250571165509867;
                                                                            }
                                                                            _ => {
                                                                                len = 0 as libc::c_int;
                                                                                if 1 as libc::c_int
                                                                                    != read_n_bytes(
                                                                                        sock,
                                                                                        &mut len as *mut libc::c_int as *mut libc::c_char,
                                                                                        1 as libc::c_int as size_t,
                                                                                    )
                                                                                {
                                                                                    current_block = 6813626912928578684;
                                                                                } else {
                                                                                    current_block = 4899250571165509867;
                                                                                }
                                                                            }
                                                                        }
                                                                        match current_block {
                                                                            6813626912928578684 => {}
                                                                            _ => {
                                                                                if !(len + 2 as libc::c_int
                                                                                    != read_n_bytes(
                                                                                        sock,
                                                                                        buff.as_mut_ptr() as *mut libc::c_char,
                                                                                        (len + 2 as libc::c_int) as size_t,
                                                                                    ))
                                                                                {
                                                                                    return SUCCESS as libc::c_int;
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                    3 => {
                                                                        current_block = 13835226679685971936;
                                                                        match current_block {
                                                                            8331409615400467618 => {
                                                                                len = 16 as libc::c_int;
                                                                                current_block = 4899250571165509867;
                                                                            }
                                                                            8739749648707081367 => {
                                                                                len = 4 as libc::c_int;
                                                                                current_block = 4899250571165509867;
                                                                            }
                                                                            _ => {
                                                                                len = 0 as libc::c_int;
                                                                                if 1 as libc::c_int
                                                                                    != read_n_bytes(
                                                                                        sock,
                                                                                        &mut len as *mut libc::c_int as *mut libc::c_char,
                                                                                        1 as libc::c_int as size_t,
                                                                                    )
                                                                                {
                                                                                    current_block = 6813626912928578684;
                                                                                } else {
                                                                                    current_block = 4899250571165509867;
                                                                                }
                                                                            }
                                                                        }
                                                                        match current_block {
                                                                            6813626912928578684 => {}
                                                                            _ => {
                                                                                if !(len + 2 as libc::c_int
                                                                                    != read_n_bytes(
                                                                                        sock,
                                                                                        buff.as_mut_ptr() as *mut libc::c_char,
                                                                                        (len + 2 as libc::c_int) as size_t,
                                                                                    ))
                                                                                {
                                                                                    return SUCCESS as libc::c_int;
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                    _ => {}
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    0 => {
                        current_block = 8457315219000651999;
                        match current_block {
                            12304796239473295489 => return SUCCESS as libc::c_int,
                            14072441030219150333 => {
                                if v6 != 0 {
                                    proxychains_write_log(
                                        b"[proxychains] error: SOCKS4 doesn't support ipv6 addresses\n\0"
                                            as *const u8 as *const libc::c_char as *mut libc::c_char,
                                    );
                                } else {
                                    buff[0 as libc::c_int
                                        as usize] = 4 as libc::c_int as libc::c_uchar;
                                    buff[1 as libc::c_int
                                        as usize] = 1 as libc::c_int as libc::c_uchar;
                                    memcpy(
                                        &mut *buff.as_mut_ptr().offset(2 as libc::c_int as isize)
                                            as *mut libc::c_uchar as *mut libc::c_void,
                                        &mut port as *mut libc::c_ushort as *const libc::c_void,
                                        2 as libc::c_int as libc::c_ulong,
                                    );
                                    if dns_len != 0 {
                                        ip
                                            .addr
                                            .v4
                                            .octet[0 as libc::c_int
                                            as usize] = 0 as libc::c_int as libc::c_uchar;
                                        ip
                                            .addr
                                            .v4
                                            .octet[1 as libc::c_int
                                            as usize] = 0 as libc::c_int as libc::c_uchar;
                                        ip
                                            .addr
                                            .v4
                                            .octet[2 as libc::c_int
                                            as usize] = 0 as libc::c_int as libc::c_uchar;
                                        ip
                                            .addr
                                            .v4
                                            .octet[3 as libc::c_int
                                            as usize] = 1 as libc::c_int as libc::c_uchar;
                                    }
                                    memcpy(
                                        &mut *buff.as_mut_ptr().offset(4 as libc::c_int as isize)
                                            as *mut libc::c_uchar as *mut libc::c_void,
                                        &mut ip.addr.v4 as *mut ip_type4 as *const libc::c_void,
                                        4 as libc::c_int as libc::c_ulong,
                                    );
                                    len = ulen.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                        as libc::c_int;
                                    if len > 1 as libc::c_int {
                                        memcpy(
                                            &mut *buff.as_mut_ptr().offset(8 as libc::c_int as isize)
                                                as *mut libc::c_uchar as *mut libc::c_void,
                                            user as *const libc::c_void,
                                            len as libc::c_ulong,
                                        );
                                    } else {
                                        buff[8 as libc::c_int
                                            as usize] = 0 as libc::c_int as libc::c_uchar;
                                    }
                                    if dns_len != 0 {
                                        memcpy(
                                            &mut *buff
                                                .as_mut_ptr()
                                                .offset((8 as libc::c_int + len) as isize)
                                                as *mut libc::c_uchar as *mut libc::c_void,
                                            dns_name as *const libc::c_void,
                                            dns_len.wrapping_add(1 as libc::c_int as libc::c_ulong),
                                        );
                                        len = (len as libc::c_ulong)
                                            .wrapping_add(
                                                dns_len.wrapping_add(1 as libc::c_int as libc::c_ulong),
                                            ) as libc::c_int as libc::c_int;
                                    }
                                    if !(len + 8 as libc::c_int
                                        != write_n_bytes(
                                            sock,
                                            buff.as_mut_ptr() as *mut libc::c_char,
                                            (8 as libc::c_int + len) as size_t,
                                        ))
                                    {
                                        if !(8 as libc::c_int
                                            != read_n_bytes(
                                                sock,
                                                buff.as_mut_ptr() as *mut libc::c_char,
                                                8 as libc::c_int as size_t,
                                            ))
                                        {
                                            if buff[0 as libc::c_int as usize] as libc::c_int
                                                != 0 as libc::c_int
                                                || buff[1 as libc::c_int as usize] as libc::c_int
                                                    != 90 as libc::c_int
                                            {
                                                return BLOCKED as libc::c_int;
                                            }
                                            return SUCCESS as libc::c_int;
                                        }
                                    }
                                }
                            }
                            8457315219000651999 => {
                                if dns_len == 0 {
                                    if (inet_ntop(
                                        if v6 != 0 { 10 as libc::c_int } else { 2 as libc::c_int },
                                        (ip.addr.v6).as_mut_ptr() as *const libc::c_void,
                                        ip_buf.as_mut_ptr(),
                                        ::std::mem::size_of::<[libc::c_char; 46]>() as libc::c_ulong
                                            as socklen_t,
                                    ))
                                        .is_null()
                                    {
                                        proxychains_write_log(
                                            b"[proxychains] error: ip address conversion failed\n\0"
                                                as *const u8 as *const libc::c_char as *mut libc::c_char,
                                        );
                                        current_block = 6813626912928578684;
                                    } else {
                                        dns_name = ip_buf.as_mut_ptr();
                                        current_block = 5783071609795492627;
                                    }
                                } else {
                                    current_block = 5783071609795492627;
                                }
                                match current_block {
                                    6813626912928578684 => {}
                                    _ => {
                                        let mut src: [libc::c_char; 512] = [0; 512];
                                        let mut dst: [libc::c_char; 2048] = [0; 2048];
                                        if ulen != 0 {
                                            snprintf(
                                                src.as_mut_ptr(),
                                                ::std::mem::size_of::<[libc::c_char; 512]>()
                                                    as libc::c_ulong,
                                                b"%s:%s\0" as *const u8 as *const libc::c_char,
                                                user,
                                                pass,
                                            );
                                            encode_base_64(
                                                src.as_mut_ptr(),
                                                dst.as_mut_ptr(),
                                                ::std::mem::size_of::<[libc::c_char; 2048]>()
                                                    as libc::c_ulong as libc::c_int,
                                            );
                                        } else {
                                            dst[0 as libc::c_int
                                                as usize] = 0 as libc::c_int as libc::c_char;
                                        }
                                        let mut hs_port: uint16_t = ntohs(port);
                                        len = snprintf(
                                            buff.as_mut_ptr() as *mut libc::c_char,
                                            ::std::mem::size_of::<[libc::c_uchar; 1024]>()
                                                as libc::c_ulong,
                                            b"CONNECT %s:%d HTTP/1.0\r\nHost: %s:%d\r\n%s%s%s\r\n\0"
                                                as *const u8 as *const libc::c_char,
                                            dns_name,
                                            hs_port as libc::c_int,
                                            dns_name,
                                            hs_port as libc::c_int,
                                            if ulen != 0 {
                                                b"Proxy-Authorization: Basic \0" as *const u8
                                                    as *const libc::c_char
                                            } else {
                                                dst.as_mut_ptr() as *const libc::c_char
                                            },
                                            dst.as_mut_ptr(),
                                            if ulen != 0 {
                                                b"\r\n\0" as *const u8 as *const libc::c_char
                                            } else {
                                                dst.as_mut_ptr() as *const libc::c_char
                                            },
                                        );
                                        if !(len < 0 as libc::c_int
                                            || len as libc::c_long
                                                != send(
                                                    sock,
                                                    buff.as_mut_ptr() as *const libc::c_void,
                                                    len as size_t,
                                                    0 as libc::c_int,
                                                ))
                                        {
                                            len = 0 as libc::c_int;
                                            loop {
                                                if !(len < 1024 as libc::c_int) {
                                                    current_block = 8704759739624374314;
                                                    break;
                                                }
                                                if !(1 as libc::c_int
                                                    == read_n_bytes(
                                                        sock,
                                                        buff.as_mut_ptr().offset(len as isize) as *mut libc::c_char,
                                                        1 as libc::c_int as size_t,
                                                    ))
                                                {
                                                    current_block = 6813626912928578684;
                                                    break;
                                                }
                                                len += 1;
                                                len;
                                                if len > 4 as libc::c_int
                                                    && buff[(len - 1 as libc::c_int) as usize] as libc::c_int
                                                        == '\n' as i32
                                                    && buff[(len - 2 as libc::c_int) as usize] as libc::c_int
                                                        == '\r' as i32
                                                    && buff[(len - 3 as libc::c_int) as usize] as libc::c_int
                                                        == '\n' as i32
                                                    && buff[(len - 4 as libc::c_int) as usize] as libc::c_int
                                                        == '\r' as i32
                                                {
                                                    current_block = 8704759739624374314;
                                                    break;
                                                }
                                            }
                                            match current_block {
                                                6813626912928578684 => {}
                                                _ => {
                                                    if len == 1024 as libc::c_int
                                                        || !(buff[9 as libc::c_int as usize] as libc::c_int
                                                            == '2' as i32
                                                            && buff[10 as libc::c_int as usize] as libc::c_int
                                                                == '0' as i32
                                                            && buff[11 as libc::c_int as usize] as libc::c_int
                                                                == '0' as i32)
                                                    {
                                                        return BLOCKED as libc::c_int;
                                                    }
                                                    return SUCCESS as libc::c_int;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            _ => {
                                let mut n_methods: libc::c_int = if ulen != 0 {
                                    2 as libc::c_int
                                } else {
                                    1 as libc::c_int
                                };
                                buff[0 as libc::c_int
                                    as usize] = 5 as libc::c_int as libc::c_uchar;
                                buff[1 as libc::c_int
                                    as usize] = n_methods as libc::c_uchar;
                                buff[2 as libc::c_int
                                    as usize] = 0 as libc::c_int as libc::c_uchar;
                                if ulen != 0 {
                                    buff[3 as libc::c_int
                                        as usize] = 2 as libc::c_int as libc::c_uchar;
                                }
                                if !(2 as libc::c_int + n_methods
                                    != write_n_bytes(
                                        sock,
                                        buff.as_mut_ptr() as *mut libc::c_char,
                                        (2 as libc::c_int + n_methods) as size_t,
                                    ))
                                {
                                    if !(2 as libc::c_int
                                        != read_n_bytes(
                                            sock,
                                            buff.as_mut_ptr() as *mut libc::c_char,
                                            2 as libc::c_int as size_t,
                                        ))
                                    {
                                        if buff[0 as libc::c_int as usize] as libc::c_int
                                            != 5 as libc::c_int
                                            || buff[1 as libc::c_int as usize] as libc::c_int
                                                != 0 as libc::c_int
                                                && buff[1 as libc::c_int as usize] as libc::c_int
                                                    != 2 as libc::c_int
                                        {
                                            if buff[0 as libc::c_int as usize] as libc::c_int
                                                == 5 as libc::c_int
                                                && buff[1 as libc::c_int as usize] as libc::c_int
                                                    == 0xff as libc::c_int
                                            {
                                                return BLOCKED as libc::c_int;
                                            }
                                        } else {
                                            if buff[1 as libc::c_int as usize] as libc::c_int
                                                == 2 as libc::c_int
                                            {
                                                let mut in_0: [libc::c_char; 2] = [0; 2];
                                                let mut out: [libc::c_char; 515] = [0; 515];
                                                let mut cur: *mut libc::c_char = out.as_mut_ptr();
                                                let mut c: size_t = 0;
                                                let fresh13 = cur;
                                                cur = cur.offset(1);
                                                *fresh13 = 1 as libc::c_int as libc::c_char;
                                                c = ulen & 0xff as libc::c_int as libc::c_ulong;
                                                let fresh14 = cur;
                                                cur = cur.offset(1);
                                                *fresh14 = c as libc::c_char;
                                                memcpy(
                                                    cur as *mut libc::c_void,
                                                    user as *const libc::c_void,
                                                    c,
                                                );
                                                cur = cur.offset(c as isize);
                                                c = passlen & 0xff as libc::c_int as libc::c_ulong;
                                                let fresh15 = cur;
                                                cur = cur.offset(1);
                                                *fresh15 = c as libc::c_char;
                                                memcpy(
                                                    cur as *mut libc::c_void,
                                                    pass as *const libc::c_void,
                                                    c,
                                                );
                                                cur = cur.offset(c as isize);
                                                if cur.offset_from(out.as_mut_ptr()) as libc::c_long
                                                    != write_n_bytes(
                                                        sock,
                                                        out.as_mut_ptr(),
                                                        cur.offset_from(out.as_mut_ptr()) as libc::c_long as size_t,
                                                    ) as libc::c_long
                                                {
                                                    current_block = 6813626912928578684;
                                                } else if 2 as libc::c_int
                                                    != read_n_bytes(
                                                        sock,
                                                        in_0.as_mut_ptr(),
                                                        2 as libc::c_int as size_t,
                                                    )
                                                {
                                                    current_block = 6813626912928578684;
                                                } else if !(in_0[0 as libc::c_int as usize] as libc::c_int
                                                    == 5 as libc::c_int
                                                    || in_0[0 as libc::c_int as usize] as libc::c_int
                                                        == 1 as libc::c_int)
                                                {
                                                    current_block = 6813626912928578684;
                                                } else {
                                                    if in_0[1 as libc::c_int as usize] as libc::c_int
                                                        != 0 as libc::c_int
                                                    {
                                                        return BLOCKED as libc::c_int;
                                                    }
                                                    current_block = 8464383504555462953;
                                                }
                                            } else {
                                                current_block = 8464383504555462953;
                                            }
                                            match current_block {
                                                6813626912928578684 => {}
                                                _ => {
                                                    let mut buff_iter: libc::c_int = 0 as libc::c_int;
                                                    let fresh16 = buff_iter;
                                                    buff_iter = buff_iter + 1;
                                                    buff[fresh16 as usize] = 5 as libc::c_int as libc::c_uchar;
                                                    let fresh17 = buff_iter;
                                                    buff_iter = buff_iter + 1;
                                                    buff[fresh17 as usize] = 1 as libc::c_int as libc::c_uchar;
                                                    let fresh18 = buff_iter;
                                                    buff_iter = buff_iter + 1;
                                                    buff[fresh18 as usize] = 0 as libc::c_int as libc::c_uchar;
                                                    if dns_len == 0 {
                                                        let fresh19 = buff_iter;
                                                        buff_iter = buff_iter + 1;
                                                        buff[fresh19
                                                            as usize] = (if v6 != 0 {
                                                            4 as libc::c_int
                                                        } else {
                                                            1 as libc::c_int
                                                        }) as libc::c_uchar;
                                                        memcpy(
                                                            buff.as_mut_ptr().offset(buff_iter as isize)
                                                                as *mut libc::c_void,
                                                            (ip.addr.v6).as_mut_ptr() as *const libc::c_void,
                                                            (if v6 != 0 { 16 as libc::c_int } else { 4 as libc::c_int })
                                                                as libc::c_ulong,
                                                        );
                                                        buff_iter
                                                            += if v6 != 0 {
                                                                16 as libc::c_int
                                                            } else {
                                                                4 as libc::c_int
                                                            };
                                                    } else {
                                                        let fresh20 = buff_iter;
                                                        buff_iter = buff_iter + 1;
                                                        buff[fresh20 as usize] = 3 as libc::c_int as libc::c_uchar;
                                                        let fresh21 = buff_iter;
                                                        buff_iter = buff_iter + 1;
                                                        buff[fresh21
                                                            as usize] = (dns_len & 0xff as libc::c_int as libc::c_ulong)
                                                            as libc::c_uchar;
                                                        memcpy(
                                                            buff.as_mut_ptr().offset(buff_iter as isize)
                                                                as *mut libc::c_void,
                                                            dns_name as *const libc::c_void,
                                                            dns_len,
                                                        );
                                                        buff_iter = (buff_iter as libc::c_ulong)
                                                            .wrapping_add(dns_len) as libc::c_int as libc::c_int;
                                                    }
                                                    memcpy(
                                                        buff.as_mut_ptr().offset(buff_iter as isize)
                                                            as *mut libc::c_void,
                                                        &mut port as *mut libc::c_ushort as *const libc::c_void,
                                                        2 as libc::c_int as libc::c_ulong,
                                                    );
                                                    buff_iter += 2 as libc::c_int;
                                                    if !(buff_iter
                                                        != write_n_bytes(
                                                            sock,
                                                            buff.as_mut_ptr() as *mut libc::c_char,
                                                            buff_iter as size_t,
                                                        ))
                                                    {
                                                        if !(4 as libc::c_int
                                                            != read_n_bytes(
                                                                sock,
                                                                buff.as_mut_ptr() as *mut libc::c_char,
                                                                4 as libc::c_int as size_t,
                                                            ))
                                                        {
                                                            if !(buff[0 as libc::c_int as usize] as libc::c_int
                                                                != 5 as libc::c_int
                                                                || buff[1 as libc::c_int as usize] as libc::c_int
                                                                    != 0 as libc::c_int)
                                                            {
                                                                match buff[3 as libc::c_int as usize] as libc::c_int {
                                                                    1 => {
                                                                        current_block = 8739749648707081367;
                                                                        match current_block {
                                                                            8331409615400467618 => {
                                                                                len = 16 as libc::c_int;
                                                                                current_block = 4899250571165509867;
                                                                            }
                                                                            8739749648707081367 => {
                                                                                len = 4 as libc::c_int;
                                                                                current_block = 4899250571165509867;
                                                                            }
                                                                            _ => {
                                                                                len = 0 as libc::c_int;
                                                                                if 1 as libc::c_int
                                                                                    != read_n_bytes(
                                                                                        sock,
                                                                                        &mut len as *mut libc::c_int as *mut libc::c_char,
                                                                                        1 as libc::c_int as size_t,
                                                                                    )
                                                                                {
                                                                                    current_block = 6813626912928578684;
                                                                                } else {
                                                                                    current_block = 4899250571165509867;
                                                                                }
                                                                            }
                                                                        }
                                                                        match current_block {
                                                                            6813626912928578684 => {}
                                                                            _ => {
                                                                                if !(len + 2 as libc::c_int
                                                                                    != read_n_bytes(
                                                                                        sock,
                                                                                        buff.as_mut_ptr() as *mut libc::c_char,
                                                                                        (len + 2 as libc::c_int) as size_t,
                                                                                    ))
                                                                                {
                                                                                    return SUCCESS as libc::c_int;
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                    4 => {
                                                                        current_block = 8331409615400467618;
                                                                        match current_block {
                                                                            8331409615400467618 => {
                                                                                len = 16 as libc::c_int;
                                                                                current_block = 4899250571165509867;
                                                                            }
                                                                            8739749648707081367 => {
                                                                                len = 4 as libc::c_int;
                                                                                current_block = 4899250571165509867;
                                                                            }
                                                                            _ => {
                                                                                len = 0 as libc::c_int;
                                                                                if 1 as libc::c_int
                                                                                    != read_n_bytes(
                                                                                        sock,
                                                                                        &mut len as *mut libc::c_int as *mut libc::c_char,
                                                                                        1 as libc::c_int as size_t,
                                                                                    )
                                                                                {
                                                                                    current_block = 6813626912928578684;
                                                                                } else {
                                                                                    current_block = 4899250571165509867;
                                                                                }
                                                                            }
                                                                        }
                                                                        match current_block {
                                                                            6813626912928578684 => {}
                                                                            _ => {
                                                                                if !(len + 2 as libc::c_int
                                                                                    != read_n_bytes(
                                                                                        sock,
                                                                                        buff.as_mut_ptr() as *mut libc::c_char,
                                                                                        (len + 2 as libc::c_int) as size_t,
                                                                                    ))
                                                                                {
                                                                                    return SUCCESS as libc::c_int;
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                    3 => {
                                                                        current_block = 13835226679685971936;
                                                                        match current_block {
                                                                            8331409615400467618 => {
                                                                                len = 16 as libc::c_int;
                                                                                current_block = 4899250571165509867;
                                                                            }
                                                                            8739749648707081367 => {
                                                                                len = 4 as libc::c_int;
                                                                                current_block = 4899250571165509867;
                                                                            }
                                                                            _ => {
                                                                                len = 0 as libc::c_int;
                                                                                if 1 as libc::c_int
                                                                                    != read_n_bytes(
                                                                                        sock,
                                                                                        &mut len as *mut libc::c_int as *mut libc::c_char,
                                                                                        1 as libc::c_int as size_t,
                                                                                    )
                                                                                {
                                                                                    current_block = 6813626912928578684;
                                                                                } else {
                                                                                    current_block = 4899250571165509867;
                                                                                }
                                                                            }
                                                                        }
                                                                        match current_block {
                                                                            6813626912928578684 => {}
                                                                            _ => {
                                                                                if !(len + 2 as libc::c_int
                                                                                    != read_n_bytes(
                                                                                        sock,
                                                                                        buff.as_mut_ptr() as *mut libc::c_char,
                                                                                        (len + 2 as libc::c_int) as size_t,
                                                                                    ))
                                                                                {
                                                                                    return SUCCESS as libc::c_int;
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                    _ => {}
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    1 => {
                        current_block = 14072441030219150333;
                        match current_block {
                            12304796239473295489 => return SUCCESS as libc::c_int,
                            14072441030219150333 => {
                                if v6 != 0 {
                                    proxychains_write_log(
                                        b"[proxychains] error: SOCKS4 doesn't support ipv6 addresses\n\0"
                                            as *const u8 as *const libc::c_char as *mut libc::c_char,
                                    );
                                } else {
                                    buff[0 as libc::c_int
                                        as usize] = 4 as libc::c_int as libc::c_uchar;
                                    buff[1 as libc::c_int
                                        as usize] = 1 as libc::c_int as libc::c_uchar;
                                    memcpy(
                                        &mut *buff.as_mut_ptr().offset(2 as libc::c_int as isize)
                                            as *mut libc::c_uchar as *mut libc::c_void,
                                        &mut port as *mut libc::c_ushort as *const libc::c_void,
                                        2 as libc::c_int as libc::c_ulong,
                                    );
                                    if dns_len != 0 {
                                        ip
                                            .addr
                                            .v4
                                            .octet[0 as libc::c_int
                                            as usize] = 0 as libc::c_int as libc::c_uchar;
                                        ip
                                            .addr
                                            .v4
                                            .octet[1 as libc::c_int
                                            as usize] = 0 as libc::c_int as libc::c_uchar;
                                        ip
                                            .addr
                                            .v4
                                            .octet[2 as libc::c_int
                                            as usize] = 0 as libc::c_int as libc::c_uchar;
                                        ip
                                            .addr
                                            .v4
                                            .octet[3 as libc::c_int
                                            as usize] = 1 as libc::c_int as libc::c_uchar;
                                    }
                                    memcpy(
                                        &mut *buff.as_mut_ptr().offset(4 as libc::c_int as isize)
                                            as *mut libc::c_uchar as *mut libc::c_void,
                                        &mut ip.addr.v4 as *mut ip_type4 as *const libc::c_void,
                                        4 as libc::c_int as libc::c_ulong,
                                    );
                                    len = ulen.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                        as libc::c_int;
                                    if len > 1 as libc::c_int {
                                        memcpy(
                                            &mut *buff.as_mut_ptr().offset(8 as libc::c_int as isize)
                                                as *mut libc::c_uchar as *mut libc::c_void,
                                            user as *const libc::c_void,
                                            len as libc::c_ulong,
                                        );
                                    } else {
                                        buff[8 as libc::c_int
                                            as usize] = 0 as libc::c_int as libc::c_uchar;
                                    }
                                    if dns_len != 0 {
                                        memcpy(
                                            &mut *buff
                                                .as_mut_ptr()
                                                .offset((8 as libc::c_int + len) as isize)
                                                as *mut libc::c_uchar as *mut libc::c_void,
                                            dns_name as *const libc::c_void,
                                            dns_len.wrapping_add(1 as libc::c_int as libc::c_ulong),
                                        );
                                        len = (len as libc::c_ulong)
                                            .wrapping_add(
                                                dns_len.wrapping_add(1 as libc::c_int as libc::c_ulong),
                                            ) as libc::c_int as libc::c_int;
                                    }
                                    if !(len + 8 as libc::c_int
                                        != write_n_bytes(
                                            sock,
                                            buff.as_mut_ptr() as *mut libc::c_char,
                                            (8 as libc::c_int + len) as size_t,
                                        ))
                                    {
                                        if !(8 as libc::c_int
                                            != read_n_bytes(
                                                sock,
                                                buff.as_mut_ptr() as *mut libc::c_char,
                                                8 as libc::c_int as size_t,
                                            ))
                                        {
                                            if buff[0 as libc::c_int as usize] as libc::c_int
                                                != 0 as libc::c_int
                                                || buff[1 as libc::c_int as usize] as libc::c_int
                                                    != 90 as libc::c_int
                                            {
                                                return BLOCKED as libc::c_int;
                                            }
                                            return SUCCESS as libc::c_int;
                                        }
                                    }
                                }
                            }
                            8457315219000651999 => {
                                if dns_len == 0 {
                                    if (inet_ntop(
                                        if v6 != 0 { 10 as libc::c_int } else { 2 as libc::c_int },
                                        (ip.addr.v6).as_mut_ptr() as *const libc::c_void,
                                        ip_buf.as_mut_ptr(),
                                        ::std::mem::size_of::<[libc::c_char; 46]>() as libc::c_ulong
                                            as socklen_t,
                                    ))
                                        .is_null()
                                    {
                                        proxychains_write_log(
                                            b"[proxychains] error: ip address conversion failed\n\0"
                                                as *const u8 as *const libc::c_char as *mut libc::c_char,
                                        );
                                        current_block = 6813626912928578684;
                                    } else {
                                        dns_name = ip_buf.as_mut_ptr();
                                        current_block = 5783071609795492627;
                                    }
                                } else {
                                    current_block = 5783071609795492627;
                                }
                                match current_block {
                                    6813626912928578684 => {}
                                    _ => {
                                        let mut src: [libc::c_char; 512] = [0; 512];
                                        let mut dst: [libc::c_char; 2048] = [0; 2048];
                                        if ulen != 0 {
                                            snprintf(
                                                src.as_mut_ptr(),
                                                ::std::mem::size_of::<[libc::c_char; 512]>()
                                                    as libc::c_ulong,
                                                b"%s:%s\0" as *const u8 as *const libc::c_char,
                                                user,
                                                pass,
                                            );
                                            encode_base_64(
                                                src.as_mut_ptr(),
                                                dst.as_mut_ptr(),
                                                ::std::mem::size_of::<[libc::c_char; 2048]>()
                                                    as libc::c_ulong as libc::c_int,
                                            );
                                        } else {
                                            dst[0 as libc::c_int
                                                as usize] = 0 as libc::c_int as libc::c_char;
                                        }
                                        let mut hs_port: uint16_t = ntohs(port);
                                        len = snprintf(
                                            buff.as_mut_ptr() as *mut libc::c_char,
                                            ::std::mem::size_of::<[libc::c_uchar; 1024]>()
                                                as libc::c_ulong,
                                            b"CONNECT %s:%d HTTP/1.0\r\nHost: %s:%d\r\n%s%s%s\r\n\0"
                                                as *const u8 as *const libc::c_char,
                                            dns_name,
                                            hs_port as libc::c_int,
                                            dns_name,
                                            hs_port as libc::c_int,
                                            if ulen != 0 {
                                                b"Proxy-Authorization: Basic \0" as *const u8
                                                    as *const libc::c_char
                                            } else {
                                                dst.as_mut_ptr() as *const libc::c_char
                                            },
                                            dst.as_mut_ptr(),
                                            if ulen != 0 {
                                                b"\r\n\0" as *const u8 as *const libc::c_char
                                            } else {
                                                dst.as_mut_ptr() as *const libc::c_char
                                            },
                                        );
                                        if !(len < 0 as libc::c_int
                                            || len as libc::c_long
                                                != send(
                                                    sock,
                                                    buff.as_mut_ptr() as *const libc::c_void,
                                                    len as size_t,
                                                    0 as libc::c_int,
                                                ))
                                        {
                                            len = 0 as libc::c_int;
                                            loop {
                                                if !(len < 1024 as libc::c_int) {
                                                    current_block = 8704759739624374314;
                                                    break;
                                                }
                                                if !(1 as libc::c_int
                                                    == read_n_bytes(
                                                        sock,
                                                        buff.as_mut_ptr().offset(len as isize) as *mut libc::c_char,
                                                        1 as libc::c_int as size_t,
                                                    ))
                                                {
                                                    current_block = 6813626912928578684;
                                                    break;
                                                }
                                                len += 1;
                                                len;
                                                if len > 4 as libc::c_int
                                                    && buff[(len - 1 as libc::c_int) as usize] as libc::c_int
                                                        == '\n' as i32
                                                    && buff[(len - 2 as libc::c_int) as usize] as libc::c_int
                                                        == '\r' as i32
                                                    && buff[(len - 3 as libc::c_int) as usize] as libc::c_int
                                                        == '\n' as i32
                                                    && buff[(len - 4 as libc::c_int) as usize] as libc::c_int
                                                        == '\r' as i32
                                                {
                                                    current_block = 8704759739624374314;
                                                    break;
                                                }
                                            }
                                            match current_block {
                                                6813626912928578684 => {}
                                                _ => {
                                                    if len == 1024 as libc::c_int
                                                        || !(buff[9 as libc::c_int as usize] as libc::c_int
                                                            == '2' as i32
                                                            && buff[10 as libc::c_int as usize] as libc::c_int
                                                                == '0' as i32
                                                            && buff[11 as libc::c_int as usize] as libc::c_int
                                                                == '0' as i32)
                                                    {
                                                        return BLOCKED as libc::c_int;
                                                    }
                                                    return SUCCESS as libc::c_int;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            _ => {
                                let mut n_methods: libc::c_int = if ulen != 0 {
                                    2 as libc::c_int
                                } else {
                                    1 as libc::c_int
                                };
                                buff[0 as libc::c_int
                                    as usize] = 5 as libc::c_int as libc::c_uchar;
                                buff[1 as libc::c_int
                                    as usize] = n_methods as libc::c_uchar;
                                buff[2 as libc::c_int
                                    as usize] = 0 as libc::c_int as libc::c_uchar;
                                if ulen != 0 {
                                    buff[3 as libc::c_int
                                        as usize] = 2 as libc::c_int as libc::c_uchar;
                                }
                                if !(2 as libc::c_int + n_methods
                                    != write_n_bytes(
                                        sock,
                                        buff.as_mut_ptr() as *mut libc::c_char,
                                        (2 as libc::c_int + n_methods) as size_t,
                                    ))
                                {
                                    if !(2 as libc::c_int
                                        != read_n_bytes(
                                            sock,
                                            buff.as_mut_ptr() as *mut libc::c_char,
                                            2 as libc::c_int as size_t,
                                        ))
                                    {
                                        if buff[0 as libc::c_int as usize] as libc::c_int
                                            != 5 as libc::c_int
                                            || buff[1 as libc::c_int as usize] as libc::c_int
                                                != 0 as libc::c_int
                                                && buff[1 as libc::c_int as usize] as libc::c_int
                                                    != 2 as libc::c_int
                                        {
                                            if buff[0 as libc::c_int as usize] as libc::c_int
                                                == 5 as libc::c_int
                                                && buff[1 as libc::c_int as usize] as libc::c_int
                                                    == 0xff as libc::c_int
                                            {
                                                return BLOCKED as libc::c_int;
                                            }
                                        } else {
                                            if buff[1 as libc::c_int as usize] as libc::c_int
                                                == 2 as libc::c_int
                                            {
                                                let mut in_0: [libc::c_char; 2] = [0; 2];
                                                let mut out: [libc::c_char; 515] = [0; 515];
                                                let mut cur: *mut libc::c_char = out.as_mut_ptr();
                                                let mut c: size_t = 0;
                                                let fresh13 = cur;
                                                cur = cur.offset(1);
                                                *fresh13 = 1 as libc::c_int as libc::c_char;
                                                c = ulen & 0xff as libc::c_int as libc::c_ulong;
                                                let fresh14 = cur;
                                                cur = cur.offset(1);
                                                *fresh14 = c as libc::c_char;
                                                memcpy(
                                                    cur as *mut libc::c_void,
                                                    user as *const libc::c_void,
                                                    c,
                                                );
                                                cur = cur.offset(c as isize);
                                                c = passlen & 0xff as libc::c_int as libc::c_ulong;
                                                let fresh15 = cur;
                                                cur = cur.offset(1);
                                                *fresh15 = c as libc::c_char;
                                                memcpy(
                                                    cur as *mut libc::c_void,
                                                    pass as *const libc::c_void,
                                                    c,
                                                );
                                                cur = cur.offset(c as isize);
                                                if cur.offset_from(out.as_mut_ptr()) as libc::c_long
                                                    != write_n_bytes(
                                                        sock,
                                                        out.as_mut_ptr(),
                                                        cur.offset_from(out.as_mut_ptr()) as libc::c_long as size_t,
                                                    ) as libc::c_long
                                                {
                                                    current_block = 6813626912928578684;
                                                } else if 2 as libc::c_int
                                                    != read_n_bytes(
                                                        sock,
                                                        in_0.as_mut_ptr(),
                                                        2 as libc::c_int as size_t,
                                                    )
                                                {
                                                    current_block = 6813626912928578684;
                                                } else if !(in_0[0 as libc::c_int as usize] as libc::c_int
                                                    == 5 as libc::c_int
                                                    || in_0[0 as libc::c_int as usize] as libc::c_int
                                                        == 1 as libc::c_int)
                                                {
                                                    current_block = 6813626912928578684;
                                                } else {
                                                    if in_0[1 as libc::c_int as usize] as libc::c_int
                                                        != 0 as libc::c_int
                                                    {
                                                        return BLOCKED as libc::c_int;
                                                    }
                                                    current_block = 8464383504555462953;
                                                }
                                            } else {
                                                current_block = 8464383504555462953;
                                            }
                                            match current_block {
                                                6813626912928578684 => {}
                                                _ => {
                                                    let mut buff_iter: libc::c_int = 0 as libc::c_int;
                                                    let fresh16 = buff_iter;
                                                    buff_iter = buff_iter + 1;
                                                    buff[fresh16 as usize] = 5 as libc::c_int as libc::c_uchar;
                                                    let fresh17 = buff_iter;
                                                    buff_iter = buff_iter + 1;
                                                    buff[fresh17 as usize] = 1 as libc::c_int as libc::c_uchar;
                                                    let fresh18 = buff_iter;
                                                    buff_iter = buff_iter + 1;
                                                    buff[fresh18 as usize] = 0 as libc::c_int as libc::c_uchar;
                                                    if dns_len == 0 {
                                                        let fresh19 = buff_iter;
                                                        buff_iter = buff_iter + 1;
                                                        buff[fresh19
                                                            as usize] = (if v6 != 0 {
                                                            4 as libc::c_int
                                                        } else {
                                                            1 as libc::c_int
                                                        }) as libc::c_uchar;
                                                        memcpy(
                                                            buff.as_mut_ptr().offset(buff_iter as isize)
                                                                as *mut libc::c_void,
                                                            (ip.addr.v6).as_mut_ptr() as *const libc::c_void,
                                                            (if v6 != 0 { 16 as libc::c_int } else { 4 as libc::c_int })
                                                                as libc::c_ulong,
                                                        );
                                                        buff_iter
                                                            += if v6 != 0 {
                                                                16 as libc::c_int
                                                            } else {
                                                                4 as libc::c_int
                                                            };
                                                    } else {
                                                        let fresh20 = buff_iter;
                                                        buff_iter = buff_iter + 1;
                                                        buff[fresh20 as usize] = 3 as libc::c_int as libc::c_uchar;
                                                        let fresh21 = buff_iter;
                                                        buff_iter = buff_iter + 1;
                                                        buff[fresh21
                                                            as usize] = (dns_len & 0xff as libc::c_int as libc::c_ulong)
                                                            as libc::c_uchar;
                                                        memcpy(
                                                            buff.as_mut_ptr().offset(buff_iter as isize)
                                                                as *mut libc::c_void,
                                                            dns_name as *const libc::c_void,
                                                            dns_len,
                                                        );
                                                        buff_iter = (buff_iter as libc::c_ulong)
                                                            .wrapping_add(dns_len) as libc::c_int as libc::c_int;
                                                    }
                                                    memcpy(
                                                        buff.as_mut_ptr().offset(buff_iter as isize)
                                                            as *mut libc::c_void,
                                                        &mut port as *mut libc::c_ushort as *const libc::c_void,
                                                        2 as libc::c_int as libc::c_ulong,
                                                    );
                                                    buff_iter += 2 as libc::c_int;
                                                    if !(buff_iter
                                                        != write_n_bytes(
                                                            sock,
                                                            buff.as_mut_ptr() as *mut libc::c_char,
                                                            buff_iter as size_t,
                                                        ))
                                                    {
                                                        if !(4 as libc::c_int
                                                            != read_n_bytes(
                                                                sock,
                                                                buff.as_mut_ptr() as *mut libc::c_char,
                                                                4 as libc::c_int as size_t,
                                                            ))
                                                        {
                                                            if !(buff[0 as libc::c_int as usize] as libc::c_int
                                                                != 5 as libc::c_int
                                                                || buff[1 as libc::c_int as usize] as libc::c_int
                                                                    != 0 as libc::c_int)
                                                            {
                                                                match buff[3 as libc::c_int as usize] as libc::c_int {
                                                                    1 => {
                                                                        current_block = 8739749648707081367;
                                                                        match current_block {
                                                                            8331409615400467618 => {
                                                                                len = 16 as libc::c_int;
                                                                                current_block = 4899250571165509867;
                                                                            }
                                                                            8739749648707081367 => {
                                                                                len = 4 as libc::c_int;
                                                                                current_block = 4899250571165509867;
                                                                            }
                                                                            _ => {
                                                                                len = 0 as libc::c_int;
                                                                                if 1 as libc::c_int
                                                                                    != read_n_bytes(
                                                                                        sock,
                                                                                        &mut len as *mut libc::c_int as *mut libc::c_char,
                                                                                        1 as libc::c_int as size_t,
                                                                                    )
                                                                                {
                                                                                    current_block = 6813626912928578684;
                                                                                } else {
                                                                                    current_block = 4899250571165509867;
                                                                                }
                                                                            }
                                                                        }
                                                                        match current_block {
                                                                            6813626912928578684 => {}
                                                                            _ => {
                                                                                if !(len + 2 as libc::c_int
                                                                                    != read_n_bytes(
                                                                                        sock,
                                                                                        buff.as_mut_ptr() as *mut libc::c_char,
                                                                                        (len + 2 as libc::c_int) as size_t,
                                                                                    ))
                                                                                {
                                                                                    return SUCCESS as libc::c_int;
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                    4 => {
                                                                        current_block = 8331409615400467618;
                                                                        match current_block {
                                                                            8331409615400467618 => {
                                                                                len = 16 as libc::c_int;
                                                                                current_block = 4899250571165509867;
                                                                            }
                                                                            8739749648707081367 => {
                                                                                len = 4 as libc::c_int;
                                                                                current_block = 4899250571165509867;
                                                                            }
                                                                            _ => {
                                                                                len = 0 as libc::c_int;
                                                                                if 1 as libc::c_int
                                                                                    != read_n_bytes(
                                                                                        sock,
                                                                                        &mut len as *mut libc::c_int as *mut libc::c_char,
                                                                                        1 as libc::c_int as size_t,
                                                                                    )
                                                                                {
                                                                                    current_block = 6813626912928578684;
                                                                                } else {
                                                                                    current_block = 4899250571165509867;
                                                                                }
                                                                            }
                                                                        }
                                                                        match current_block {
                                                                            6813626912928578684 => {}
                                                                            _ => {
                                                                                if !(len + 2 as libc::c_int
                                                                                    != read_n_bytes(
                                                                                        sock,
                                                                                        buff.as_mut_ptr() as *mut libc::c_char,
                                                                                        (len + 2 as libc::c_int) as size_t,
                                                                                    ))
                                                                                {
                                                                                    return SUCCESS as libc::c_int;
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                    3 => {
                                                                        current_block = 13835226679685971936;
                                                                        match current_block {
                                                                            8331409615400467618 => {
                                                                                len = 16 as libc::c_int;
                                                                                current_block = 4899250571165509867;
                                                                            }
                                                                            8739749648707081367 => {
                                                                                len = 4 as libc::c_int;
                                                                                current_block = 4899250571165509867;
                                                                            }
                                                                            _ => {
                                                                                len = 0 as libc::c_int;
                                                                                if 1 as libc::c_int
                                                                                    != read_n_bytes(
                                                                                        sock,
                                                                                        &mut len as *mut libc::c_int as *mut libc::c_char,
                                                                                        1 as libc::c_int as size_t,
                                                                                    )
                                                                                {
                                                                                    current_block = 6813626912928578684;
                                                                                } else {
                                                                                    current_block = 4899250571165509867;
                                                                                }
                                                                            }
                                                                        }
                                                                        match current_block {
                                                                            6813626912928578684 => {}
                                                                            _ => {
                                                                                if !(len + 2 as libc::c_int
                                                                                    != read_n_bytes(
                                                                                        sock,
                                                                                        buff.as_mut_ptr() as *mut libc::c_char,
                                                                                        (len + 2 as libc::c_int) as size_t,
                                                                                    ))
                                                                                {
                                                                                    return SUCCESS as libc::c_int;
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                    _ => {}
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    2 => {
                        current_block = 17728966195399430138;
                        match current_block {
                            12304796239473295489 => return SUCCESS as libc::c_int,
                            14072441030219150333 => {
                                if v6 != 0 {
                                    proxychains_write_log(
                                        b"[proxychains] error: SOCKS4 doesn't support ipv6 addresses\n\0"
                                            as *const u8 as *const libc::c_char as *mut libc::c_char,
                                    );
                                } else {
                                    buff[0 as libc::c_int
                                        as usize] = 4 as libc::c_int as libc::c_uchar;
                                    buff[1 as libc::c_int
                                        as usize] = 1 as libc::c_int as libc::c_uchar;
                                    memcpy(
                                        &mut *buff.as_mut_ptr().offset(2 as libc::c_int as isize)
                                            as *mut libc::c_uchar as *mut libc::c_void,
                                        &mut port as *mut libc::c_ushort as *const libc::c_void,
                                        2 as libc::c_int as libc::c_ulong,
                                    );
                                    if dns_len != 0 {
                                        ip
                                            .addr
                                            .v4
                                            .octet[0 as libc::c_int
                                            as usize] = 0 as libc::c_int as libc::c_uchar;
                                        ip
                                            .addr
                                            .v4
                                            .octet[1 as libc::c_int
                                            as usize] = 0 as libc::c_int as libc::c_uchar;
                                        ip
                                            .addr
                                            .v4
                                            .octet[2 as libc::c_int
                                            as usize] = 0 as libc::c_int as libc::c_uchar;
                                        ip
                                            .addr
                                            .v4
                                            .octet[3 as libc::c_int
                                            as usize] = 1 as libc::c_int as libc::c_uchar;
                                    }
                                    memcpy(
                                        &mut *buff.as_mut_ptr().offset(4 as libc::c_int as isize)
                                            as *mut libc::c_uchar as *mut libc::c_void,
                                        &mut ip.addr.v4 as *mut ip_type4 as *const libc::c_void,
                                        4 as libc::c_int as libc::c_ulong,
                                    );
                                    len = ulen.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                        as libc::c_int;
                                    if len > 1 as libc::c_int {
                                        memcpy(
                                            &mut *buff.as_mut_ptr().offset(8 as libc::c_int as isize)
                                                as *mut libc::c_uchar as *mut libc::c_void,
                                            user as *const libc::c_void,
                                            len as libc::c_ulong,
                                        );
                                    } else {
                                        buff[8 as libc::c_int
                                            as usize] = 0 as libc::c_int as libc::c_uchar;
                                    }
                                    if dns_len != 0 {
                                        memcpy(
                                            &mut *buff
                                                .as_mut_ptr()
                                                .offset((8 as libc::c_int + len) as isize)
                                                as *mut libc::c_uchar as *mut libc::c_void,
                                            dns_name as *const libc::c_void,
                                            dns_len.wrapping_add(1 as libc::c_int as libc::c_ulong),
                                        );
                                        len = (len as libc::c_ulong)
                                            .wrapping_add(
                                                dns_len.wrapping_add(1 as libc::c_int as libc::c_ulong),
                                            ) as libc::c_int as libc::c_int;
                                    }
                                    if !(len + 8 as libc::c_int
                                        != write_n_bytes(
                                            sock,
                                            buff.as_mut_ptr() as *mut libc::c_char,
                                            (8 as libc::c_int + len) as size_t,
                                        ))
                                    {
                                        if !(8 as libc::c_int
                                            != read_n_bytes(
                                                sock,
                                                buff.as_mut_ptr() as *mut libc::c_char,
                                                8 as libc::c_int as size_t,
                                            ))
                                        {
                                            if buff[0 as libc::c_int as usize] as libc::c_int
                                                != 0 as libc::c_int
                                                || buff[1 as libc::c_int as usize] as libc::c_int
                                                    != 90 as libc::c_int
                                            {
                                                return BLOCKED as libc::c_int;
                                            }
                                            return SUCCESS as libc::c_int;
                                        }
                                    }
                                }
                            }
                            8457315219000651999 => {
                                if dns_len == 0 {
                                    if (inet_ntop(
                                        if v6 != 0 { 10 as libc::c_int } else { 2 as libc::c_int },
                                        (ip.addr.v6).as_mut_ptr() as *const libc::c_void,
                                        ip_buf.as_mut_ptr(),
                                        ::std::mem::size_of::<[libc::c_char; 46]>() as libc::c_ulong
                                            as socklen_t,
                                    ))
                                        .is_null()
                                    {
                                        proxychains_write_log(
                                            b"[proxychains] error: ip address conversion failed\n\0"
                                                as *const u8 as *const libc::c_char as *mut libc::c_char,
                                        );
                                        current_block = 6813626912928578684;
                                    } else {
                                        dns_name = ip_buf.as_mut_ptr();
                                        current_block = 5783071609795492627;
                                    }
                                } else {
                                    current_block = 5783071609795492627;
                                }
                                match current_block {
                                    6813626912928578684 => {}
                                    _ => {
                                        let mut src: [libc::c_char; 512] = [0; 512];
                                        let mut dst: [libc::c_char; 2048] = [0; 2048];
                                        if ulen != 0 {
                                            snprintf(
                                                src.as_mut_ptr(),
                                                ::std::mem::size_of::<[libc::c_char; 512]>()
                                                    as libc::c_ulong,
                                                b"%s:%s\0" as *const u8 as *const libc::c_char,
                                                user,
                                                pass,
                                            );
                                            encode_base_64(
                                                src.as_mut_ptr(),
                                                dst.as_mut_ptr(),
                                                ::std::mem::size_of::<[libc::c_char; 2048]>()
                                                    as libc::c_ulong as libc::c_int,
                                            );
                                        } else {
                                            dst[0 as libc::c_int
                                                as usize] = 0 as libc::c_int as libc::c_char;
                                        }
                                        let mut hs_port: uint16_t = ntohs(port);
                                        len = snprintf(
                                            buff.as_mut_ptr() as *mut libc::c_char,
                                            ::std::mem::size_of::<[libc::c_uchar; 1024]>()
                                                as libc::c_ulong,
                                            b"CONNECT %s:%d HTTP/1.0\r\nHost: %s:%d\r\n%s%s%s\r\n\0"
                                                as *const u8 as *const libc::c_char,
                                            dns_name,
                                            hs_port as libc::c_int,
                                            dns_name,
                                            hs_port as libc::c_int,
                                            if ulen != 0 {
                                                b"Proxy-Authorization: Basic \0" as *const u8
                                                    as *const libc::c_char
                                            } else {
                                                dst.as_mut_ptr() as *const libc::c_char
                                            },
                                            dst.as_mut_ptr(),
                                            if ulen != 0 {
                                                b"\r\n\0" as *const u8 as *const libc::c_char
                                            } else {
                                                dst.as_mut_ptr() as *const libc::c_char
                                            },
                                        );
                                        if !(len < 0 as libc::c_int
                                            || len as libc::c_long
                                                != send(
                                                    sock,
                                                    buff.as_mut_ptr() as *const libc::c_void,
                                                    len as size_t,
                                                    0 as libc::c_int,
                                                ))
                                        {
                                            len = 0 as libc::c_int;
                                            loop {
                                                if !(len < 1024 as libc::c_int) {
                                                    current_block = 8704759739624374314;
                                                    break;
                                                }
                                                if !(1 as libc::c_int
                                                    == read_n_bytes(
                                                        sock,
                                                        buff.as_mut_ptr().offset(len as isize) as *mut libc::c_char,
                                                        1 as libc::c_int as size_t,
                                                    ))
                                                {
                                                    current_block = 6813626912928578684;
                                                    break;
                                                }
                                                len += 1;
                                                len;
                                                if len > 4 as libc::c_int
                                                    && buff[(len - 1 as libc::c_int) as usize] as libc::c_int
                                                        == '\n' as i32
                                                    && buff[(len - 2 as libc::c_int) as usize] as libc::c_int
                                                        == '\r' as i32
                                                    && buff[(len - 3 as libc::c_int) as usize] as libc::c_int
                                                        == '\n' as i32
                                                    && buff[(len - 4 as libc::c_int) as usize] as libc::c_int
                                                        == '\r' as i32
                                                {
                                                    current_block = 8704759739624374314;
                                                    break;
                                                }
                                            }
                                            match current_block {
                                                6813626912928578684 => {}
                                                _ => {
                                                    if len == 1024 as libc::c_int
                                                        || !(buff[9 as libc::c_int as usize] as libc::c_int
                                                            == '2' as i32
                                                            && buff[10 as libc::c_int as usize] as libc::c_int
                                                                == '0' as i32
                                                            && buff[11 as libc::c_int as usize] as libc::c_int
                                                                == '0' as i32)
                                                    {
                                                        return BLOCKED as libc::c_int;
                                                    }
                                                    return SUCCESS as libc::c_int;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            _ => {
                                let mut n_methods: libc::c_int = if ulen != 0 {
                                    2 as libc::c_int
                                } else {
                                    1 as libc::c_int
                                };
                                buff[0 as libc::c_int
                                    as usize] = 5 as libc::c_int as libc::c_uchar;
                                buff[1 as libc::c_int
                                    as usize] = n_methods as libc::c_uchar;
                                buff[2 as libc::c_int
                                    as usize] = 0 as libc::c_int as libc::c_uchar;
                                if ulen != 0 {
                                    buff[3 as libc::c_int
                                        as usize] = 2 as libc::c_int as libc::c_uchar;
                                }
                                if !(2 as libc::c_int + n_methods
                                    != write_n_bytes(
                                        sock,
                                        buff.as_mut_ptr() as *mut libc::c_char,
                                        (2 as libc::c_int + n_methods) as size_t,
                                    ))
                                {
                                    if !(2 as libc::c_int
                                        != read_n_bytes(
                                            sock,
                                            buff.as_mut_ptr() as *mut libc::c_char,
                                            2 as libc::c_int as size_t,
                                        ))
                                    {
                                        if buff[0 as libc::c_int as usize] as libc::c_int
                                            != 5 as libc::c_int
                                            || buff[1 as libc::c_int as usize] as libc::c_int
                                                != 0 as libc::c_int
                                                && buff[1 as libc::c_int as usize] as libc::c_int
                                                    != 2 as libc::c_int
                                        {
                                            if buff[0 as libc::c_int as usize] as libc::c_int
                                                == 5 as libc::c_int
                                                && buff[1 as libc::c_int as usize] as libc::c_int
                                                    == 0xff as libc::c_int
                                            {
                                                return BLOCKED as libc::c_int;
                                            }
                                        } else {
                                            if buff[1 as libc::c_int as usize] as libc::c_int
                                                == 2 as libc::c_int
                                            {
                                                let mut in_0: [libc::c_char; 2] = [0; 2];
                                                let mut out: [libc::c_char; 515] = [0; 515];
                                                let mut cur: *mut libc::c_char = out.as_mut_ptr();
                                                let mut c: size_t = 0;
                                                let fresh13 = cur;
                                                cur = cur.offset(1);
                                                *fresh13 = 1 as libc::c_int as libc::c_char;
                                                c = ulen & 0xff as libc::c_int as libc::c_ulong;
                                                let fresh14 = cur;
                                                cur = cur.offset(1);
                                                *fresh14 = c as libc::c_char;
                                                memcpy(
                                                    cur as *mut libc::c_void,
                                                    user as *const libc::c_void,
                                                    c,
                                                );
                                                cur = cur.offset(c as isize);
                                                c = passlen & 0xff as libc::c_int as libc::c_ulong;
                                                let fresh15 = cur;
                                                cur = cur.offset(1);
                                                *fresh15 = c as libc::c_char;
                                                memcpy(
                                                    cur as *mut libc::c_void,
                                                    pass as *const libc::c_void,
                                                    c,
                                                );
                                                cur = cur.offset(c as isize);
                                                if cur.offset_from(out.as_mut_ptr()) as libc::c_long
                                                    != write_n_bytes(
                                                        sock,
                                                        out.as_mut_ptr(),
                                                        cur.offset_from(out.as_mut_ptr()) as libc::c_long as size_t,
                                                    ) as libc::c_long
                                                {
                                                    current_block = 6813626912928578684;
                                                } else if 2 as libc::c_int
                                                    != read_n_bytes(
                                                        sock,
                                                        in_0.as_mut_ptr(),
                                                        2 as libc::c_int as size_t,
                                                    )
                                                {
                                                    current_block = 6813626912928578684;
                                                } else if !(in_0[0 as libc::c_int as usize] as libc::c_int
                                                    == 5 as libc::c_int
                                                    || in_0[0 as libc::c_int as usize] as libc::c_int
                                                        == 1 as libc::c_int)
                                                {
                                                    current_block = 6813626912928578684;
                                                } else {
                                                    if in_0[1 as libc::c_int as usize] as libc::c_int
                                                        != 0 as libc::c_int
                                                    {
                                                        return BLOCKED as libc::c_int;
                                                    }
                                                    current_block = 8464383504555462953;
                                                }
                                            } else {
                                                current_block = 8464383504555462953;
                                            }
                                            match current_block {
                                                6813626912928578684 => {}
                                                _ => {
                                                    let mut buff_iter: libc::c_int = 0 as libc::c_int;
                                                    let fresh16 = buff_iter;
                                                    buff_iter = buff_iter + 1;
                                                    buff[fresh16 as usize] = 5 as libc::c_int as libc::c_uchar;
                                                    let fresh17 = buff_iter;
                                                    buff_iter = buff_iter + 1;
                                                    buff[fresh17 as usize] = 1 as libc::c_int as libc::c_uchar;
                                                    let fresh18 = buff_iter;
                                                    buff_iter = buff_iter + 1;
                                                    buff[fresh18 as usize] = 0 as libc::c_int as libc::c_uchar;
                                                    if dns_len == 0 {
                                                        let fresh19 = buff_iter;
                                                        buff_iter = buff_iter + 1;
                                                        buff[fresh19
                                                            as usize] = (if v6 != 0 {
                                                            4 as libc::c_int
                                                        } else {
                                                            1 as libc::c_int
                                                        }) as libc::c_uchar;
                                                        memcpy(
                                                            buff.as_mut_ptr().offset(buff_iter as isize)
                                                                as *mut libc::c_void,
                                                            (ip.addr.v6).as_mut_ptr() as *const libc::c_void,
                                                            (if v6 != 0 { 16 as libc::c_int } else { 4 as libc::c_int })
                                                                as libc::c_ulong,
                                                        );
                                                        buff_iter
                                                            += if v6 != 0 {
                                                                16 as libc::c_int
                                                            } else {
                                                                4 as libc::c_int
                                                            };
                                                    } else {
                                                        let fresh20 = buff_iter;
                                                        buff_iter = buff_iter + 1;
                                                        buff[fresh20 as usize] = 3 as libc::c_int as libc::c_uchar;
                                                        let fresh21 = buff_iter;
                                                        buff_iter = buff_iter + 1;
                                                        buff[fresh21
                                                            as usize] = (dns_len & 0xff as libc::c_int as libc::c_ulong)
                                                            as libc::c_uchar;
                                                        memcpy(
                                                            buff.as_mut_ptr().offset(buff_iter as isize)
                                                                as *mut libc::c_void,
                                                            dns_name as *const libc::c_void,
                                                            dns_len,
                                                        );
                                                        buff_iter = (buff_iter as libc::c_ulong)
                                                            .wrapping_add(dns_len) as libc::c_int as libc::c_int;
                                                    }
                                                    memcpy(
                                                        buff.as_mut_ptr().offset(buff_iter as isize)
                                                            as *mut libc::c_void,
                                                        &mut port as *mut libc::c_ushort as *const libc::c_void,
                                                        2 as libc::c_int as libc::c_ulong,
                                                    );
                                                    buff_iter += 2 as libc::c_int;
                                                    if !(buff_iter
                                                        != write_n_bytes(
                                                            sock,
                                                            buff.as_mut_ptr() as *mut libc::c_char,
                                                            buff_iter as size_t,
                                                        ))
                                                    {
                                                        if !(4 as libc::c_int
                                                            != read_n_bytes(
                                                                sock,
                                                                buff.as_mut_ptr() as *mut libc::c_char,
                                                                4 as libc::c_int as size_t,
                                                            ))
                                                        {
                                                            if !(buff[0 as libc::c_int as usize] as libc::c_int
                                                                != 5 as libc::c_int
                                                                || buff[1 as libc::c_int as usize] as libc::c_int
                                                                    != 0 as libc::c_int)
                                                            {
                                                                match buff[3 as libc::c_int as usize] as libc::c_int {
                                                                    1 => {
                                                                        current_block = 8739749648707081367;
                                                                        match current_block {
                                                                            8331409615400467618 => {
                                                                                len = 16 as libc::c_int;
                                                                                current_block = 4899250571165509867;
                                                                            }
                                                                            8739749648707081367 => {
                                                                                len = 4 as libc::c_int;
                                                                                current_block = 4899250571165509867;
                                                                            }
                                                                            _ => {
                                                                                len = 0 as libc::c_int;
                                                                                if 1 as libc::c_int
                                                                                    != read_n_bytes(
                                                                                        sock,
                                                                                        &mut len as *mut libc::c_int as *mut libc::c_char,
                                                                                        1 as libc::c_int as size_t,
                                                                                    )
                                                                                {
                                                                                    current_block = 6813626912928578684;
                                                                                } else {
                                                                                    current_block = 4899250571165509867;
                                                                                }
                                                                            }
                                                                        }
                                                                        match current_block {
                                                                            6813626912928578684 => {}
                                                                            _ => {
                                                                                if !(len + 2 as libc::c_int
                                                                                    != read_n_bytes(
                                                                                        sock,
                                                                                        buff.as_mut_ptr() as *mut libc::c_char,
                                                                                        (len + 2 as libc::c_int) as size_t,
                                                                                    ))
                                                                                {
                                                                                    return SUCCESS as libc::c_int;
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                    4 => {
                                                                        current_block = 8331409615400467618;
                                                                        match current_block {
                                                                            8331409615400467618 => {
                                                                                len = 16 as libc::c_int;
                                                                                current_block = 4899250571165509867;
                                                                            }
                                                                            8739749648707081367 => {
                                                                                len = 4 as libc::c_int;
                                                                                current_block = 4899250571165509867;
                                                                            }
                                                                            _ => {
                                                                                len = 0 as libc::c_int;
                                                                                if 1 as libc::c_int
                                                                                    != read_n_bytes(
                                                                                        sock,
                                                                                        &mut len as *mut libc::c_int as *mut libc::c_char,
                                                                                        1 as libc::c_int as size_t,
                                                                                    )
                                                                                {
                                                                                    current_block = 6813626912928578684;
                                                                                } else {
                                                                                    current_block = 4899250571165509867;
                                                                                }
                                                                            }
                                                                        }
                                                                        match current_block {
                                                                            6813626912928578684 => {}
                                                                            _ => {
                                                                                if !(len + 2 as libc::c_int
                                                                                    != read_n_bytes(
                                                                                        sock,
                                                                                        buff.as_mut_ptr() as *mut libc::c_char,
                                                                                        (len + 2 as libc::c_int) as size_t,
                                                                                    ))
                                                                                {
                                                                                    return SUCCESS as libc::c_int;
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                    3 => {
                                                                        current_block = 13835226679685971936;
                                                                        match current_block {
                                                                            8331409615400467618 => {
                                                                                len = 16 as libc::c_int;
                                                                                current_block = 4899250571165509867;
                                                                            }
                                                                            8739749648707081367 => {
                                                                                len = 4 as libc::c_int;
                                                                                current_block = 4899250571165509867;
                                                                            }
                                                                            _ => {
                                                                                len = 0 as libc::c_int;
                                                                                if 1 as libc::c_int
                                                                                    != read_n_bytes(
                                                                                        sock,
                                                                                        &mut len as *mut libc::c_int as *mut libc::c_char,
                                                                                        1 as libc::c_int as size_t,
                                                                                    )
                                                                                {
                                                                                    current_block = 6813626912928578684;
                                                                                } else {
                                                                                    current_block = 4899250571165509867;
                                                                                }
                                                                            }
                                                                        }
                                                                        match current_block {
                                                                            6813626912928578684 => {}
                                                                            _ => {
                                                                                if !(len + 2 as libc::c_int
                                                                                    != read_n_bytes(
                                                                                        sock,
                                                                                        buff.as_mut_ptr() as *mut libc::c_char,
                                                                                        (len + 2 as libc::c_int) as size_t,
                                                                                    ))
                                                                                {
                                                                                    return SUCCESS as libc::c_int;
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                    _ => {}
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
        _ => {}
    }
    return SOCKET_ERROR as libc::c_int;
}
unsafe extern "C" fn start_chain(
    mut fd: *mut libc::c_int,
    mut pd: *mut proxy_data,
    mut begin_mark: *mut libc::c_char,
) -> libc::c_int {
    let mut ip_buf: [libc::c_char; 46] = [0; 46];
    let mut addr: sockaddr_in = sockaddr_in {
        sin_family: 0,
        sin_port: 0,
        sin_addr: in_addr { s_addr: 0 },
        sin_zero: [0; 8],
    };
    let mut addr6: sockaddr_in6 = sockaddr_in6 {
        sin6_family: 0,
        sin6_port: 0,
        sin6_flowinfo: 0,
        sin6_addr: in6_addr {
            __in6_u: C2RustUnnamed {
                __u6_addr8: [0; 16],
            },
        },
        sin6_scope_id: 0,
    };
    let mut v6: libc::c_int = (*pd).ip.is_v6 as libc::c_int;
    *fd = socket(
        if v6 != 0 { 10 as libc::c_int } else { 2 as libc::c_int },
        SOCK_STREAM as libc::c_int,
        0 as libc::c_int,
    );
    if !(*fd == -(1 as libc::c_int)) {
        ip_buf = [0; 46];
        if !(inet_ntop(
            if v6 != 0 { 10 as libc::c_int } else { 2 as libc::c_int },
            ((*pd).ip.addr.v6).as_mut_ptr() as *const libc::c_void,
            ip_buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 46]>() as libc::c_ulong as socklen_t,
        ))
            .is_null()
        {
            proxychains_write_log(
                b"[proxychains] %s  ...  %s:%d \0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                begin_mark,
                ip_buf.as_mut_ptr(),
                htons((*pd).port) as libc::c_int,
            );
            (*pd).ps = PLAY_STATE;
            addr = {
                let mut init = sockaddr_in {
                    sin_family: 2 as libc::c_int as sa_family_t,
                    sin_port: (*pd).port,
                    sin_addr: {
                        let mut init = in_addr {
                            s_addr: (*pd).ip.addr.v4.as_int,
                        };
                        init
                    },
                    sin_zero: [0; 8],
                };
                init
            };
            addr6 = {
                let mut init = sockaddr_in6 {
                    sin6_family: 10 as libc::c_int as sa_family_t,
                    sin6_port: (*pd).port,
                    sin6_flowinfo: 0,
                    sin6_addr: in6_addr {
                        __in6_u: C2RustUnnamed {
                            __u6_addr8: [0; 16],
                        },
                    },
                    sin6_scope_id: 0,
                };
                init
            };
            if v6 != 0 {
                memcpy(
                    &mut addr6.sin6_addr.__in6_u.__u6_addr8 as *mut [uint8_t; 16]
                        as *mut libc::c_void,
                    ((*pd).ip.addr.v6).as_mut_ptr() as *const libc::c_void,
                    16 as libc::c_int as libc::c_ulong,
                );
            }
            if timed_connect(
                *fd,
                (if v6 != 0 {
                    &mut addr6 as *mut sockaddr_in6 as *mut libc::c_void
                } else {
                    &mut addr as *mut sockaddr_in as *mut libc::c_void
                }) as *mut sockaddr,
                (if v6 != 0 {
                    ::std::mem::size_of::<sockaddr_in6>() as libc::c_ulong
                } else {
                    ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong
                }) as socklen_t,
            ) != 0
            {
                (*pd).ps = DOWN_STATE;
                proxychains_write_log(
                    b" ...  timeout\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
            } else {
                (*pd).ps = BUSY_STATE;
                return SUCCESS as libc::c_int;
            }
        }
    }
    if *fd != -(1 as libc::c_int) {
        close(*fd);
    }
    return SOCKET_ERROR as libc::c_int;
}
unsafe extern "C" fn select_proxy(
    mut how: select_type,
    mut pd: *mut proxy_data,
    mut proxy_count: libc::c_uint,
    mut offset: *mut libc::c_uint,
) -> *mut proxy_data {
    let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut k: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    if *offset >= proxy_count {
        return 0 as *mut proxy_data;
    }
    match how as libc::c_uint {
        0 => {
            loop {
                k = k.wrapping_add(1);
                k;
                i = (rand() as libc::c_uint).wrapping_rem(proxy_count);
                if !((*pd.offset(i as isize)).ps as libc::c_uint
                    != PLAY_STATE as libc::c_int as libc::c_uint
                    && k < proxy_count.wrapping_mul(100 as libc::c_int as libc::c_uint))
                {
                    break;
                }
            }
        }
        1 => {
            i = *offset;
            while i < proxy_count {
                if (*pd.offset(i as isize)).ps as libc::c_uint
                    == PLAY_STATE as libc::c_int as libc::c_uint
                {
                    *offset = i;
                    break;
                } else {
                    i = i.wrapping_add(1);
                    i;
                }
            }
        }
        _ => {}
    }
    if i >= proxy_count {
        i = 0 as libc::c_int as libc::c_uint;
    }
    return if (*pd.offset(i as isize)).ps as libc::c_uint
        == PLAY_STATE as libc::c_int as libc::c_uint
    {
        &mut *pd.offset(i as isize) as *mut proxy_data
    } else {
        0 as *mut proxy_data
    };
}
unsafe extern "C" fn release_all(
    mut pd: *mut proxy_data,
    mut proxy_count: libc::c_uint,
) {
    let mut i: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while i < proxy_count {
        (*pd.offset(i as isize)).ps = PLAY_STATE;
        i = i.wrapping_add(1);
        i;
    }
}
unsafe extern "C" fn release_busy(
    mut pd: *mut proxy_data,
    mut proxy_count: libc::c_uint,
) {
    let mut i: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while i < proxy_count {
        if (*pd.offset(i as isize)).ps as libc::c_uint
            == BUSY_STATE as libc::c_int as libc::c_uint
        {
            (*pd.offset(i as isize)).ps = PLAY_STATE;
        }
        i = i.wrapping_add(1);
        i;
    }
}
unsafe extern "C" fn calc_alive(
    mut pd: *mut proxy_data,
    mut proxy_count: libc::c_uint,
) -> libc::c_uint {
    let mut i: libc::c_uint = 0;
    let mut alive_count: libc::c_int = 0 as libc::c_int;
    release_busy(pd, proxy_count);
    i = 0 as libc::c_int as libc::c_uint;
    while i < proxy_count {
        if (*pd.offset(i as isize)).ps as libc::c_uint
            == PLAY_STATE as libc::c_int as libc::c_uint
        {
            alive_count += 1;
            alive_count;
        }
        i = i.wrapping_add(1);
        i;
    }
    return alive_count as libc::c_uint;
}
unsafe extern "C" fn chain_step(
    mut ns: libc::c_int,
    mut pfrom: *mut proxy_data,
    mut pto: *mut proxy_data,
) -> libc::c_int {
    let mut retcode: libc::c_int = -(1 as libc::c_int);
    let mut hostname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut hostname_buf: [libc::c_char; 256] = [0; 256];
    let mut ip_buf: [libc::c_char; 46] = [0; 46];
    let mut v6: libc::c_int = (*pto).ip.is_v6 as libc::c_int;
    let mut current_block_11: u64;
    if v6 == 0
        && proxychains_resolver as libc::c_uint
            >= DNSLF_RDNS_START as libc::c_int as libc::c_uint
        && (*pto).ip.addr.v4.octet[0 as libc::c_int as usize] as libc::c_uint
            == remote_dns_subnet
    {
        if rdns_get_host_for_ip((*pto).ip.addr.v4, hostname_buf.as_mut_ptr()) == 0 {
            current_block_11 = 4104256195691340693;
        } else {
            hostname = hostname_buf.as_mut_ptr();
            current_block_11 = 12349973810996921269;
        }
    } else {
        current_block_11 = 4104256195691340693;
    }
    match current_block_11 {
        4104256195691340693 => {
            if (inet_ntop(
                if v6 != 0 { 10 as libc::c_int } else { 2 as libc::c_int },
                ((*pto).ip.addr.v6).as_mut_ptr() as *const libc::c_void,
                ip_buf.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 46]>() as libc::c_ulong as socklen_t,
            ))
                .is_null()
            {
                (*pto).ps = DOWN_STATE;
                proxychains_write_log(
                    b"<--ip conversion error!\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
                close(ns);
                return SOCKET_ERROR as libc::c_int;
            }
            hostname = ip_buf.as_mut_ptr();
        }
        _ => {}
    }
    proxychains_write_log(
        b" ...  %s:%d \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        hostname,
        htons((*pto).port) as libc::c_int,
    );
    retcode = tunnel_to(
        ns,
        (*pto).ip,
        (*pto).port,
        (*pfrom).pt,
        ((*pfrom).user).as_mut_ptr(),
        ((*pfrom).pass).as_mut_ptr(),
    );
    match retcode {
        0 => {
            (*pto).ps = BUSY_STATE;
        }
        5 => {
            (*pto).ps = BLOCKED_STATE;
            proxychains_write_log(
                b"<--denied\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            close(ns);
        }
        2 => {
            (*pto).ps = DOWN_STATE;
            proxychains_write_log(
                b"<--socket error or timeout!\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            close(ns);
        }
        _ => {}
    }
    return retcode;
}
pub unsafe extern "C" fn connect_proxy_chain(
    mut sock: libc::c_int,
    mut target_ip: ip_type,
    mut target_port: libc::c_ushort,
    mut pd: *mut proxy_data,
    mut proxy_count: libc::c_uint,
    mut ct: chain_type,
    mut max_chain: libc::c_uint,
) -> libc::c_int {
    let mut current_block: u64;
    let mut p4: proxy_data = proxy_data {
        ip: ip_type {
            addr: C2RustUnnamed_1 {
                v4: ip_type4 { octet: [0; 4] },
            },
            is_v6: 0,
        },
        port: 0,
        pt: HTTP_TYPE,
        ps: PLAY_STATE,
        user: [0; 256],
        pass: [0; 256],
    };
    let mut p1: *mut proxy_data = 0 as *mut proxy_data;
    let mut p2: *mut proxy_data = 0 as *mut proxy_data;
    let mut p3: *mut proxy_data = 0 as *mut proxy_data;
    let mut ns: libc::c_int = -(1 as libc::c_int);
    let mut rc: libc::c_int = -(1 as libc::c_int);
    let mut offset: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut alive_count: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut curr_len: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut looped: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut rr_loop_max: libc::c_uint = 14 as libc::c_int as libc::c_uint;
    p3 = &mut p4;
    '_again: loop {
        rc = -(1 as libc::c_int);
        match ct as libc::c_uint {
            0 => {
                alive_count = calc_alive(pd, proxy_count);
                offset = 0 as libc::c_int as libc::c_uint;
                loop {
                    p1 = select_proxy(FIFOLY, pd, proxy_count, &mut offset);
                    if p1.is_null() {
                        current_block = 16045472035088126313;
                        break '_again;
                    }
                    if !(SUCCESS as libc::c_int
                        != start_chain(
                            &mut ns,
                            p1,
                            b"Dynamic chain\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                        ) && offset < proxy_count)
                    {
                        break;
                    }
                }
                loop {
                    p2 = select_proxy(FIFOLY, pd, proxy_count, &mut offset);
                    if p2.is_null() {
                        break;
                    }
                    if SUCCESS as libc::c_int != chain_step(ns, p1, p2) {
                        continue '_again;
                    }
                    p1 = p2;
                }
                (*p3).ip = target_ip;
                (*p3).port = target_port;
                if SUCCESS as libc::c_int != chain_step(ns, p1, p3) {
                    current_block = 14147571675441435126;
                    break;
                } else {
                    current_block = 3634396408142324656;
                    break;
                }
            }
            3 => {
                alive_count = calc_alive(pd, proxy_count);
                offset = proxychains_proxy_offset;
                if alive_count < max_chain {
                    current_block = 16045472035088126313;
                    break;
                }
                while rc != SUCCESS as libc::c_int {
                    p1 = select_proxy(FIFOLY, pd, proxy_count, &mut offset);
                    if p1.is_null() {
                        offset = 0 as libc::c_int as libc::c_uint;
                        looped = looped.wrapping_add(1);
                        looped;
                        if looped > rr_loop_max {
                            proxychains_proxy_offset = 0 as libc::c_int as libc::c_uint;
                            current_block = 16045472035088126313;
                            break '_again;
                        } else {
                            release_all(pd, proxy_count);
                            usleep(
                                (10000 as libc::c_int as libc::c_uint).wrapping_mul(looped),
                            );
                        }
                    } else {
                        rc = start_chain(
                            &mut ns,
                            p1,
                            b"Round Robin chain\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                        );
                    }
                }
                curr_len = 1 as libc::c_int as libc::c_uint;
                while curr_len < max_chain {
                    p2 = select_proxy(FIFOLY, pd, proxy_count, &mut offset);
                    if p2.is_null() {
                        offset = 0 as libc::c_int as libc::c_uint;
                    } else {
                        if SUCCESS as libc::c_int != chain_step(ns, p1, p2) {
                            continue '_again;
                        }
                        p1 = p2;
                        curr_len = curr_len.wrapping_add(1);
                        curr_len;
                    }
                }
                (*p3).ip = target_ip;
                (*p3).port = target_port;
                proxychains_proxy_offset = offset
                    .wrapping_add(1 as libc::c_int as libc::c_uint);
                if SUCCESS as libc::c_int != chain_step(ns, p1, p3) {
                    current_block = 14147571675441435126;
                    break;
                } else {
                    current_block = 3634396408142324656;
                    break;
                }
            }
            1 => {
                alive_count = calc_alive(pd, proxy_count);
                offset = 0 as libc::c_int as libc::c_uint;
                p1 = select_proxy(FIFOLY, pd, proxy_count, &mut offset);
                if p1.is_null() {
                    current_block = 2220405792722996547;
                    break;
                } else {
                    current_block = 5372832139739605200;
                    break;
                }
            }
            2 => {
                alive_count = calc_alive(pd, proxy_count);
                if alive_count < max_chain {
                    current_block = 16045472035088126313;
                    break;
                }
                offset = 0 as libc::c_int as libc::c_uint;
                curr_len = offset;
                loop {
                    p1 = select_proxy(RANDOMLY, pd, proxy_count, &mut offset);
                    if p1.is_null() {
                        current_block = 16045472035088126313;
                        break '_again;
                    }
                    if !(SUCCESS as libc::c_int
                        != start_chain(
                            &mut ns,
                            p1,
                            b"Random chain\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                        ) && offset < max_chain)
                    {
                        break;
                    }
                }
                loop {
                    curr_len = curr_len.wrapping_add(1);
                    if !(curr_len < max_chain) {
                        break;
                    }
                    p2 = select_proxy(RANDOMLY, pd, proxy_count, &mut offset);
                    if p2.is_null() {
                        current_block = 16045472035088126313;
                        break '_again;
                    }
                    if SUCCESS as libc::c_int != chain_step(ns, p1, p2) {
                        continue '_again;
                    }
                    p1 = p2;
                }
                (*p3).ip = target_ip;
                (*p3).port = target_port;
                if SUCCESS as libc::c_int != chain_step(ns, p1, p3) {
                    current_block = 14147571675441435126;
                    break;
                } else {
                    current_block = 3634396408142324656;
                    break;
                }
            }
            _ => {
                current_block = 3634396408142324656;
                break;
            }
        }
    }
    match current_block {
        5372832139739605200 => {
            if SUCCESS as libc::c_int
                != start_chain(
                    &mut ns,
                    p1,
                    b"Strict chain\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                )
            {
                current_block = 2220405792722996547;
            } else {
                loop {
                    if !(offset < proxy_count) {
                        current_block = 11739054925370445424;
                        break;
                    }
                    p2 = select_proxy(FIFOLY, pd, proxy_count, &mut offset);
                    if p2.is_null() {
                        current_block = 11739054925370445424;
                        break;
                    }
                    if SUCCESS as libc::c_int != chain_step(ns, p1, p2) {
                        current_block = 2220405792722996547;
                        break;
                    }
                    p1 = p2;
                }
                match current_block {
                    2220405792722996547 => {}
                    _ => {
                        (*p3).ip = target_ip;
                        (*p3).port = target_port;
                        if SUCCESS as libc::c_int != chain_step(ns, p1, p3) {
                            current_block = 14147571675441435126;
                        } else {
                            current_block = 3634396408142324656;
                        }
                    }
                }
            }
        }
        16045472035088126313 => {
            proxychains_write_log(
                b"\n!!!need more proxies!!!\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            current_block = 2220405792722996547;
        }
        _ => {}
    }
    match current_block {
        2220405792722996547 => {
            release_all(pd, proxy_count);
            if ns != -(1 as libc::c_int) {
                close(ns);
            }
            *__errno_location() = 110 as libc::c_int;
            return -(1 as libc::c_int);
        }
        14147571675441435126 => {
            if ns != -(1 as libc::c_int) {
                close(ns);
            }
            *__errno_location() = 111 as libc::c_int;
            return -(1 as libc::c_int);
        }
        _ => {
            proxychains_write_log(
                b" ...  OK\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            dup2(ns, sock);
            close(ns);
            return 0 as libc::c_int;
        }
    };
}
static mut servbyname_lock: pthread_mutex_t = pthread_mutex_t {
    __data: __pthread_mutex_s {
        __lock: 0,
        __count: 0,
        __owner: 0,
        __nusers: 0,
        __kind: 0,
        __spins: 0,
        __elision: 0,
        __list: __pthread_list_t {
            __prev: 0 as *const __pthread_internal_list as *mut __pthread_internal_list,
            __next: 0 as *const __pthread_internal_list as *mut __pthread_internal_list,
        },
    },
};
pub unsafe extern "C" fn core_initialize() {
    pthread_mutex_init(&mut servbyname_lock, 0 as *const pthread_mutexattr_t);
}
pub unsafe extern "C" fn core_unload() {
    pthread_mutex_destroy(&mut servbyname_lock);
}
unsafe extern "C" fn gethostbyname_data_setstring(
    mut data: *mut gethostbyname_data,
    mut name: *mut libc::c_char,
) {
    snprintf(
        ((*data).addr_name).as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
        b"%s\0" as *const u8 as *const libc::c_char,
        name,
    );
    (*data).hostent_space.h_name = ((*data).addr_name).as_mut_ptr();
}
pub unsafe extern "C" fn proxy_gethostbyname_old(
    mut name: *const libc::c_char,
) -> *mut hostent {
    let mut current_block: u64;
    static mut hostent_space: hostent = hostent {
        h_name: 0 as *const libc::c_char as *mut libc::c_char,
        h_aliases: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        h_addrtype: 0,
        h_length: 0,
        h_addr_list: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
    };
    static mut resolved_addr: in_addr_t = 0;
    static mut resolved_addr_p: *mut libc::c_char = 0 as *const libc::c_char
        as *mut libc::c_char;
    static mut addr_name: [libc::c_char; 256] = [0; 256];
    let mut pipe_fd: [libc::c_int; 2] = [0; 2];
    let mut buff: [libc::c_char; 256] = [0; 256];
    let mut addr: in_addr_t = 0;
    let mut pid: pid_t = 0;
    let mut status: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut l: size_t = 0;
    let mut hp: *mut hostent = 0 as *mut hostent;
    hostent_space.h_addr_list = &mut resolved_addr_p;
    *hostent_space
        .h_addr_list = &mut resolved_addr as *mut in_addr_t as *mut libc::c_char;
    resolved_addr = 0 as libc::c_int as in_addr_t;
    if pc_isnumericipv4(name) != 0 {
        strcpy(buff.as_mut_ptr(), name);
        current_block = 12060365713884072758;
    } else {
        gethostname(
            buff.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
        );
        if strcmp(buff.as_mut_ptr(), name) == 0 {
            current_block = 12060365713884072758;
        } else {
            memset(
                buff.as_mut_ptr() as *mut libc::c_void,
                0 as libc::c_int,
                ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
            );
            loop {
                hp = gethostent();
                if hp.is_null() {
                    break;
                }
                if strcmp((*hp).h_name, name) == 0 {
                    return hp;
                }
            }
            ret = pipe2(pipe_fd.as_mut_ptr(), 0o2000000 as libc::c_int);
            if ret != 0 {
                current_block = 14979695132257679753;
            } else {
                pid = fork();
                match pid {
                    0 => {
                        proxychains_write_log(
                            b"|DNS-request| %s \n\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                            name,
                        );
                        close(pipe_fd[0 as libc::c_int as usize]);
                        dup2(pipe_fd[1 as libc::c_int as usize], 1 as libc::c_int);
                        close(pipe_fd[1 as libc::c_int as usize]);
                        execlp(
                            b"proxyresolv\0" as *const u8 as *const libc::c_char,
                            b"proxyresolv\0" as *const u8 as *const libc::c_char,
                            name,
                            0 as *mut libc::c_void,
                        );
                        perror(
                            b"can't exec proxyresolv\0" as *const u8
                                as *const libc::c_char,
                        );
                        exit(2 as libc::c_int);
                    }
                    -1 => {
                        close(pipe_fd[0 as libc::c_int as usize]);
                        close(pipe_fd[1 as libc::c_int as usize]);
                        perror(b"can't fork\0" as *const u8 as *const libc::c_char);
                        current_block = 14979695132257679753;
                    }
                    _ => {
                        close(pipe_fd[1 as libc::c_int as usize]);
                        waitpid(pid, &mut status, 0 as libc::c_int);
                        buff[0 as libc::c_int
                            as usize] = 0 as libc::c_int as libc::c_char;
                        read(
                            pipe_fd[0 as libc::c_int as usize],
                            &mut buff as *mut [libc::c_char; 256] as *mut libc::c_void,
                            ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
                        );
                        close(pipe_fd[0 as libc::c_int as usize]);
                        current_block = 12060365713884072758;
                    }
                }
            }
        }
    }
    match current_block {
        12060365713884072758 => {
            l = strlen(buff.as_mut_ptr());
            if l != 0
                && buff[l.wrapping_sub(1 as libc::c_int as libc::c_ulong) as usize]
                    as libc::c_int == '\n' as i32
            {
                buff[l.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    as usize] = 0 as libc::c_int as libc::c_char;
            }
            addr = inet_addr(buff.as_mut_ptr());
            if addr == -(1 as libc::c_int) as in_addr_t {
                proxychains_write_log(
                    b"|DNS-response|: %s does not exist\n\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    name,
                );
                perror(b"err_dns\0" as *const u8 as *const libc::c_char);
            } else {
                memcpy(
                    *hostent_space.h_addr_list as *mut libc::c_void,
                    &mut addr as *mut in_addr_t as *const libc::c_void,
                    ::std::mem::size_of::<in_addr>() as libc::c_ulong,
                );
                hostent_space.h_name = addr_name.as_mut_ptr();
                snprintf(
                    addr_name.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
                    b"%s\0" as *const u8 as *const libc::c_char,
                    buff.as_mut_ptr(),
                );
                hostent_space
                    .h_length = ::std::mem::size_of::<in_addr_t>() as libc::c_ulong
                    as libc::c_int;
                hostent_space.h_addrtype = 2 as libc::c_int;
                proxychains_write_log(
                    b"|DNS-response| %s is %s\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    name,
                    inet_ntoa(*(&mut addr as *mut in_addr_t as *mut in_addr)),
                );
                return &mut hostent_space;
            }
        }
        _ => {}
    }
    return 0 as *mut hostent;
}
pub unsafe extern "C" fn proxy_gethostbyname(
    mut name: *const libc::c_char,
    mut data: *mut gethostbyname_data,
) -> *mut hostent {
    let mut hdb_res: ip_type4 = ip_type4 { octet: [0; 4] };
    let mut buff: [libc::c_char; 256] = [0; 256];
    (*data)
        .resolved_addr_p[0 as libc::c_int
        as usize] = &mut (*data).resolved_addr as *mut in_addr_t as *mut libc::c_char;
    (*data).resolved_addr_p[1 as libc::c_int as usize] = 0 as *mut libc::c_char;
    (*data).hostent_space.h_addr_list = ((*data).resolved_addr_p).as_mut_ptr();
    (*data)
        .hostent_space
        .h_aliases = &mut *((*data).resolved_addr_p)
        .as_mut_ptr()
        .offset(1 as libc::c_int as isize) as *mut *mut libc::c_char;
    (*data).resolved_addr = 0 as libc::c_int as in_addr_t;
    (*data).hostent_space.h_addrtype = 2 as libc::c_int;
    (*data)
        .hostent_space
        .h_length = ::std::mem::size_of::<in_addr_t>() as libc::c_ulong as libc::c_int;
    if pc_isnumericipv4(name) != 0 {
        (*data).resolved_addr = inet_addr(name);
    } else {
        gethostname(
            buff.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
        );
        if strcmp(buff.as_mut_ptr(), name) == 0 {
            (*data).resolved_addr = inet_addr(buff.as_mut_ptr());
            if (*data).resolved_addr == -(1 as libc::c_int) as in_addr_t {
                (*data)
                    .resolved_addr = ip_type4 {
                    octet: [
                        127 as libc::c_int as libc::c_uchar,
                        0 as libc::c_int as libc::c_uchar,
                        0 as libc::c_int as libc::c_uchar,
                        1 as libc::c_int as libc::c_uchar,
                    ],
                }
                    .as_int;
            }
        } else {
            hdb_res = hostsreader_get_numeric_ip_for_name(name);
            if (hdb_res.as_int
                != ip_type4 {
                    as_int: -(1 as libc::c_int) as uint32_t,
                }
                    .as_int)
            {
                (*data).resolved_addr = hdb_res.as_int;
            } else {
                (*data)
                    .resolved_addr = (rdns_get_ip_for_host(
                    name as *mut libc::c_char,
                    strlen(name),
                ))
                    .as_int;
                if ((*data).resolved_addr
                    == ip_type4 {
                        as_int: -(1 as libc::c_int) as uint32_t,
                    }
                        .as_int)
                {
                    return 0 as *mut hostent;
                }
            }
        }
    }
    gethostbyname_data_setstring(data, name as *mut libc::c_char);
    return &mut (*data).hostent_space;
}
pub unsafe extern "C" fn proxy_freeaddrinfo(mut res: *mut addrinfo) {
    free(res as *mut libc::c_void);
}
unsafe extern "C" fn mygetservbyname_r(
    mut name: *const libc::c_char,
    mut proto: *const libc::c_char,
    mut result_buf: *mut servent,
    mut buf: *mut libc::c_char,
    mut buflen: size_t,
    mut result: *mut *mut servent,
) -> libc::c_int {
    return getservbyname_r(name, proto, result_buf, buf, buflen, result);
}
unsafe extern "C" fn looks_like_numeric_ipv6(
    mut node: *const libc::c_char,
) -> libc::c_int {
    if (strchr(node, ':' as i32)).is_null() {
        return 0 as libc::c_int;
    }
    let mut p: *const libc::c_char = node;
    loop {
        let fresh22 = p;
        p = p.offset(1);
        match *fresh22 as libc::c_int {
            0 => return 1 as libc::c_int,
            58 | 46 | 48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 | 65 | 66 | 67 | 68
            | 69 | 70 | 97 | 98 | 99 | 100 | 101 | 102 => {}
            _ => return 0 as libc::c_int,
        }
    };
}
unsafe extern "C" fn my_inet_aton(
    mut node: *const libc::c_char,
    mut space: *mut addrinfo_data,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    (*(&mut (*space).sockaddr_space as *mut sockaddr_storage as *mut sockaddr_in))
        .sin_family = 2 as libc::c_int as sa_family_t;
    ret = inet_aton(
        node,
        &mut (*(&mut (*space).sockaddr_space as *mut sockaddr_storage
            as *mut sockaddr_in))
            .sin_addr,
    );
    if ret != 0 || looks_like_numeric_ipv6(node) == 0 {
        return ret;
    }
    ret = inet_pton(
        10 as libc::c_int,
        node,
        &mut (*(&mut (*space).sockaddr_space as *mut sockaddr_storage
            as *mut sockaddr_in6))
            .sin6_addr as *mut in6_addr as *mut libc::c_void,
    );
    if ret != 0 {
        (*(&mut (*space).sockaddr_space as *mut sockaddr_storage as *mut sockaddr_in6))
            .sin6_family = 10 as libc::c_int as sa_family_t;
    }
    return ret;
}
pub unsafe extern "C" fn proxy_getaddrinfo(
    mut node: *const libc::c_char,
    mut service: *const libc::c_char,
    mut hints: *const addrinfo,
    mut res: *mut *mut addrinfo,
) -> libc::c_int {
    let mut current_block: u64;
    let mut ghdata: gethostbyname_data = gethostbyname_data {
        hostent_space: hostent {
            h_name: 0 as *const libc::c_char as *mut libc::c_char,
            h_aliases: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
            h_addrtype: 0,
            h_length: 0,
            h_addr_list: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        },
        resolved_addr: 0,
        resolved_addr_p: [0 as *mut libc::c_char; 2],
        addr_name: [0; 256],
    };
    let mut space: *mut addrinfo_data = 0 as *mut addrinfo_data;
    let mut se: *mut servent = 0 as *mut servent;
    let mut hp: *mut hostent = 0 as *mut hostent;
    let mut se_buf: servent = servent {
        s_name: 0 as *mut libc::c_char,
        s_aliases: 0 as *mut *mut libc::c_char,
        s_port: 0,
        s_proto: 0 as *mut libc::c_char,
    };
    let mut p: *mut addrinfo = 0 as *mut addrinfo;
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut port: libc::c_int = 0;
    let mut af: libc::c_int = 2 as libc::c_int;
    space = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<addrinfo_data>() as libc::c_ulong,
    ) as *mut addrinfo_data;
    if !space.is_null() {
        if !node.is_null() && my_inet_aton(node, space) == 0 {
            if !hints.is_null() && (*hints).ai_flags & 0x4 as libc::c_int != 0 {
                free(space as *mut libc::c_void);
                return -(2 as libc::c_int);
            }
            if proxychains_resolver as libc::c_uint
                == DNSLF_FORKEXEC as libc::c_int as libc::c_uint
            {
                hp = proxy_gethostbyname_old(node);
            } else {
                hp = proxy_gethostbyname(node, &mut ghdata);
            }
            if !hp.is_null() {
                memcpy(
                    &mut (*(&mut (*space).sockaddr_space as *mut sockaddr_storage
                        as *mut sockaddr_in))
                        .sin_addr as *mut in_addr as *mut libc::c_void,
                    *(*hp).h_addr_list as *const libc::c_void,
                    ::std::mem::size_of::<in_addr_t>() as libc::c_ulong,
                );
                current_block = 11584701595673473500;
            } else {
                free(space as *mut libc::c_void);
                current_block = 10508991800853968959;
            }
        } else {
            if !node.is_null() {
                af = (*(&mut (*space).sockaddr_space as *mut sockaddr_storage
                    as *mut sockaddr_in))
                    .sin_family as libc::c_int;
            } else if node.is_null() && (*hints).ai_flags & 0x1 as libc::c_int == 0 {
                let ref mut fresh23 = (*(&mut (*space).sockaddr_space
                    as *mut sockaddr_storage as *mut sockaddr_in))
                    .sin_family;
                *fresh23 = 2 as libc::c_int as sa_family_t;
                af = *fresh23 as libc::c_int;
                memcpy(
                    &mut (*(&mut (*space).sockaddr_space as *mut sockaddr_storage
                        as *mut sockaddr_in))
                        .sin_addr as *mut in_addr as *mut libc::c_void,
                    [
                        127 as libc::c_int as libc::c_char,
                        0 as libc::c_int as libc::c_char,
                        0 as libc::c_int as libc::c_char,
                        1 as libc::c_int as libc::c_char,
                    ]
                        .as_mut_ptr() as *const libc::c_void,
                    4 as libc::c_int as libc::c_ulong,
                );
            }
            current_block = 11584701595673473500;
        }
        match current_block {
            10508991800853968959 => {}
            _ => {
                if !service.is_null() {
                    mygetservbyname_r(
                        service,
                        0 as *const libc::c_char,
                        &mut se_buf,
                        buf.as_mut_ptr(),
                        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
                        &mut se,
                    );
                }
                port = if !se.is_null() {
                    (*se).s_port
                } else {
                    htons(
                        atoi(
                            if !service.is_null() {
                                service
                            } else {
                                b"0\0" as *const u8 as *const libc::c_char
                            },
                        ) as uint16_t,
                    ) as libc::c_int
                };
                if af == 2 as libc::c_int {
                    (*(&mut (*space).sockaddr_space as *mut sockaddr_storage
                        as *mut sockaddr_in))
                        .sin_port = port as in_port_t;
                } else {
                    (*(&mut (*space).sockaddr_space as *mut sockaddr_storage
                        as *mut sockaddr_in6))
                        .sin6_port = port as in_port_t;
                }
                p = &mut (*space).addrinfo_space;
                *res = p;
                if p as size_t == space as size_t {} else {
                    __assert_fail(
                        b"(size_t)p == (size_t) space\0" as *const u8
                            as *const libc::c_char,
                        b"src/core.c\0" as *const u8 as *const libc::c_char,
                        1007 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 95],
                            &[libc::c_char; 95],
                        >(
                            b"int proxy_getaddrinfo(const char *, const char *, const struct addrinfo *, struct addrinfo **)\0",
                        ))
                            .as_ptr(),
                    );
                };
                (*p)
                    .ai_addr = &mut (*space).sockaddr_space as *mut sockaddr_storage
                    as *mut libc::c_void as *mut sockaddr;
                if !node.is_null() {
                    snprintf(
                        ((*space).addr_name).as_mut_ptr(),
                        ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
                        b"%s\0" as *const u8 as *const libc::c_char,
                        node,
                    );
                }
                (*p).ai_canonname = ((*space).addr_name).as_mut_ptr();
                (*p).ai_next = 0 as *mut addrinfo;
                (*space).sockaddr_space.ss_family = af as sa_family_t;
                (*p).ai_family = (*space).sockaddr_space.ss_family as libc::c_int;
                (*p)
                    .ai_addrlen = (if af == 2 as libc::c_int {
                    ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong
                } else {
                    ::std::mem::size_of::<sockaddr_in6>() as libc::c_ulong
                }) as socklen_t;
                if !hints.is_null() {
                    (*p).ai_socktype = (*hints).ai_socktype;
                    (*p).ai_flags = (*hints).ai_flags;
                    (*p).ai_protocol = (*hints).ai_protocol;
                    if (*p).ai_socktype == 0
                        && (*p).ai_protocol == IPPROTO_TCP as libc::c_int
                    {
                        (*p).ai_socktype = SOCK_STREAM as libc::c_int;
                    }
                } else {
                    (*p).ai_flags = 0x8 as libc::c_int | 0x20 as libc::c_int;
                }
                return 0 as libc::c_int;
            }
        }
    }
    return 1 as libc::c_int;
}
