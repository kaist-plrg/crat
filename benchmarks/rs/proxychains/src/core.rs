use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
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
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn dup2(__fd: libc::c_int, __fd2: libc::c_int) -> libc::c_int;
    fn gethostname(__name: *mut libc::c_char, __len: size_t) -> libc::c_int;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn rand() -> libc::c_int;
    fn srand(__seed: libc::c_uint);
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
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
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
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
    fn gethostent() -> *mut hostent;
    fn getservbyname_r(
        __name: *const libc::c_char,
        __proto: *const libc::c_char,
        __result_buf: *mut servent,
        __buf: *mut libc::c_char,
        __buflen: size_t,
        __result: *mut *mut servent,
    ) -> libc::c_int;
    fn inet_addr(__cp: *const libc::c_char) -> in_addr_t;
    fn inet_ntop(
        __af: libc::c_int,
        __cp: *const libc::c_void,
        __buf: *mut libc::c_char,
        __len: socklen_t,
    ) -> *const libc::c_char;
    fn inet_aton(__cp: *const libc::c_char, __inp: *mut in_addr) -> libc::c_int;
    fn poll(__fds: *mut pollfd, __nfds: nfds_t, __timeout: libc::c_int) -> libc::c_int;
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
    fn time(__timer: *mut time_t) -> time_t;
    fn gettimeofday(__tv: *mut timeval, __tz: *mut libc::c_void) -> libc::c_int;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    static mut true_connect: connect_t;
    static mut tcp_read_time_out: libc::c_int;
    static mut tcp_connect_time_out: libc::c_int;
    static mut proxychains_quiet_mode: libc::c_int;
    static mut remote_dns_subnet: libc::c_uint;
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
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
pub type size_t = libc::c_ulong;
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
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
    pub __align: libc::c_long,
}
pub type va_list = __builtin_va_list;
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
pub type socklen_t = __socklen_t;
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
pub type uint_fast32_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union ip_type {
    pub octet: [libc::c_uchar; 4],
    pub as_int: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct string_hash_tuple {
    pub hash: uint32_t,
    pub string: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct internal_ip_lookup_table {
    pub counter: uint32_t,
    pub capa: uint32_t,
    pub list: *mut *mut string_hash_tuple,
}
pub type C2RustUnnamed = libc::c_uint;
pub const BLOCKED: C2RustUnnamed = 5;
pub const CHAIN_EMPTY: C2RustUnnamed = 4;
pub const CHAIN_DOWN: C2RustUnnamed = 3;
pub const SOCKET_ERROR: C2RustUnnamed = 2;
pub const MEMORY_FAIL: C2RustUnnamed = 1;
pub const SUCCESS: C2RustUnnamed = 0;
pub type proxy_type = libc::c_uint;
pub const SOCKS5_TYPE: proxy_type = 3;
pub const SOCKS4_TYPE: proxy_type = 2;
pub const RAW_TYPE: proxy_type = 1;
pub const HTTP_TYPE: proxy_type = 0;
pub type chain_type = libc::c_uint;
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
pub type connect_t = Option::<
    unsafe extern "C" fn(libc::c_int, *const sockaddr, socklen_t) -> libc::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gethostbyname_data {
    pub hostent_space: hostent,
    pub resolved_addr: in_addr_t,
    pub resolved_addr_p: [*mut libc::c_char; 2],
    pub addr_name: [libc::c_char; 8192],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct addrinfo_data {
    pub addrinfo_space: addrinfo,
    pub sockaddr_space: sockaddr,
    pub addr_name: [libc::c_char; 256],
}
#[inline]
unsafe extern "C" fn __bswap_16(mut __bsx: __uint16_t) -> __uint16_t {
    return (__bsx as libc::c_int >> 8 as libc::c_int & 0xff as libc::c_int
        | (__bsx as libc::c_int & 0xff as libc::c_int) << 8 as libc::c_int)
        as __uint16_t;
}
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
pub static mut internal_ips_lock: pthread_mutex_t = pthread_mutex_t {
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
pub static mut internal_ips: internal_ip_lookup_table = {
    let mut init = internal_ip_lookup_table {
        counter: 0 as libc::c_int as uint32_t,
        capa: 0 as libc::c_int as uint32_t,
        list: 0 as *const *mut string_hash_tuple as *mut *mut string_hash_tuple,
    };
    init
};
pub unsafe extern "C" fn dalias_hash(mut s0: *mut libc::c_char) -> uint32_t {
    let mut s: *mut libc::c_uchar = s0 as *mut libc::c_void as *mut libc::c_uchar;
    let mut h: uint_fast32_t = 0 as libc::c_int as uint_fast32_t;
    while *s != 0 {
        let fresh0 = s;
        s = s.offset(1);
        h = (16 as libc::c_int as libc::c_ulong)
            .wrapping_mul(h)
            .wrapping_add(*fresh0 as libc::c_ulong);
        h ^= h >> 24 as libc::c_int & 0xf0 as libc::c_int as libc::c_ulong;
    }
    return (h & 0xfffffff as libc::c_int as libc::c_ulong) as uint32_t;
}
pub unsafe extern "C" fn index_from_internal_ip(mut internalip: ip_type) -> uint32_t {
    let mut tmp: ip_type = internalip;
    let mut ret: uint32_t = 0;
    ret = (tmp.octet[3 as libc::c_int as usize] as libc::c_int
        + ((tmp.octet[2 as libc::c_int as usize] as libc::c_int) << 8 as libc::c_int)
        + ((tmp.octet[1 as libc::c_int as usize] as libc::c_int) << 16 as libc::c_int))
        as uint32_t;
    ret = (ret as libc::c_uint).wrapping_sub(1 as libc::c_int as libc::c_uint)
        as uint32_t as uint32_t;
    return ret;
}
pub unsafe extern "C" fn string_from_internal_ip(
    mut internalip: ip_type,
) -> *mut libc::c_char {
    let mut res: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut index: uint32_t = index_from_internal_ip(internalip);
    pthread_mutex_lock(&mut internal_ips_lock);
    if index < internal_ips.counter {
        res = (**(internal_ips.list).offset(index as isize)).string;
    }
    pthread_mutex_unlock(&mut internal_ips_lock);
    return res;
}
pub unsafe extern "C" fn make_internal_ip(mut index: uint32_t) -> in_addr_t {
    let mut ret: ip_type = ip_type { octet: [0; 4] };
    index = index.wrapping_add(1);
    index;
    if index > 0xffffff as libc::c_int as libc::c_uint {
        return -(1 as libc::c_int) as in_addr_t;
    }
    ret
        .octet[0 as libc::c_int
        as usize] = (remote_dns_subnet & 0xff as libc::c_int as libc::c_uint)
        as libc::c_uchar;
    ret
        .octet[1 as libc::c_int
        as usize] = ((index & 0xff0000 as libc::c_int as libc::c_uint)
        >> 16 as libc::c_int) as libc::c_uchar;
    ret
        .octet[2 as libc::c_int
        as usize] = ((index & 0xff00 as libc::c_int as libc::c_uint) >> 8 as libc::c_int)
        as libc::c_uchar;
    ret
        .octet[3 as libc::c_int
        as usize] = (index & 0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
    return ret.as_int;
}
static mut base64: [libc::c_char; 65] = unsafe {
    *::std::mem::transmute::<
        &[u8; 65],
        &[libc::c_char; 65],
    >(b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/\0")
};
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
        time_elapsed = (tv.tv_sec - start_time.tv_sec) as libc::c_int
            * 1000 as libc::c_int
            + (tv.tv_usec - start_time.tv_usec) as libc::c_int / 1000 as libc::c_int;
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
    let mut n: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut l: size_t = 0;
    l = strlen(src);
    max_len = (max_len - 1 as libc::c_int) / 4 as libc::c_int;
    i = 0 as libc::c_int;
    while i < max_len {
        match l {
            0 => {}
            1 => {
                n = (*src.offset(0 as libc::c_int as isize) as libc::c_int)
                    << 16 as libc::c_int;
                let fresh1 = dest;
                dest = dest.offset(1);
                *fresh1 = base64[(n >> 18 as libc::c_int & 0o77 as libc::c_int)
                    as usize];
                let fresh2 = dest;
                dest = dest.offset(1);
                *fresh2 = base64[(n >> 12 as libc::c_int & 0o77 as libc::c_int)
                    as usize];
                let fresh3 = dest;
                dest = dest.offset(1);
                *fresh3 = '=' as i32 as libc::c_char;
                let fresh4 = dest;
                dest = dest.offset(1);
                *fresh4 = '=' as i32 as libc::c_char;
            }
            2 => {
                n = (*src.offset(0 as libc::c_int as isize) as libc::c_int)
                    << 16 as libc::c_int
                    | (*src.offset(1 as libc::c_int as isize) as libc::c_int)
                        << 8 as libc::c_int;
                let fresh5 = dest;
                dest = dest.offset(1);
                *fresh5 = base64[(n >> 18 as libc::c_int & 0o77 as libc::c_int)
                    as usize];
                let fresh6 = dest;
                dest = dest.offset(1);
                *fresh6 = base64[(n >> 12 as libc::c_int & 0o77 as libc::c_int)
                    as usize];
                let fresh7 = dest;
                dest = dest.offset(1);
                *fresh7 = base64[(n >> 6 as libc::c_int & 0o77 as libc::c_int) as usize];
                let fresh8 = dest;
                dest = dest.offset(1);
                *fresh8 = '=' as i32 as libc::c_char;
            }
            _ => {
                n = (*src.offset(0 as libc::c_int as isize) as libc::c_int)
                    << 16 as libc::c_int
                    | (*src.offset(1 as libc::c_int as isize) as libc::c_int)
                        << 8 as libc::c_int
                    | *src.offset(2 as libc::c_int as isize) as libc::c_int;
                let fresh9 = dest;
                dest = dest.offset(1);
                *fresh9 = base64[(n >> 18 as libc::c_int & 0o77 as libc::c_int)
                    as usize];
                let fresh10 = dest;
                dest = dest.offset(1);
                *fresh10 = base64[(n >> 12 as libc::c_int & 0o77 as libc::c_int)
                    as usize];
                let fresh11 = dest;
                dest = dest.offset(1);
                *fresh11 = base64[(n >> 6 as libc::c_int & 0o77 as libc::c_int)
                    as usize];
                let fresh12 = dest;
                dest = dest.offset(1);
                *fresh12 = base64[(n & 0o77 as libc::c_int) as usize];
            }
        }
        if l < 3 as libc::c_int as libc::c_ulong {
            break;
        }
        i += 1;
        i;
        src = src.offset(3 as libc::c_int as isize);
        l = (l as libc::c_ulong).wrapping_sub(3 as libc::c_int as libc::c_ulong)
            as size_t as size_t;
    }
    let fresh13 = dest;
    dest = dest.offset(1);
    *fresh13 = 0 as libc::c_int as libc::c_char;
}
pub unsafe extern "C" fn proxychains_write_log(
    mut str: *mut libc::c_char,
    mut args: ...
) {
    let mut buff: [libc::c_char; 20480] = [0; 20480];
    let mut arglist: ::std::ffi::VaListImpl;
    if proxychains_quiet_mode == 0 {
        arglist = args.clone();
        vsnprintf(
            buff.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 20480]>() as libc::c_ulong,
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
) -> size_t {
    let mut i: size_t = 0 as libc::c_int as size_t;
    let mut wrote: size_t = 0 as libc::c_int as size_t;
    loop {
        i = write(
            fd,
            &mut *buff.offset(wrote as isize) as *mut libc::c_char
                as *const libc::c_void,
            size.wrapping_sub(wrote),
        ) as size_t;
        if i <= 0 as libc::c_int as libc::c_ulong {
            return i;
        }
        wrote = (wrote as libc::c_ulong).wrapping_add(i) as size_t as size_t;
        if wrote == size {
            return wrote;
        }
    };
}
unsafe extern "C" fn read_n_bytes(
    mut fd: libc::c_int,
    mut buff: *mut libc::c_char,
    mut size: size_t,
) -> size_t {
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
            return 0 as libc::c_int as size_t;
        }
        i = i.wrapping_add(1);
        i;
    }
    return size;
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
    let mut len: size_t = 0;
    let mut buff: [libc::c_uchar; 8192] = [0; 8192];
    let mut ip_buf: [libc::c_char; 16] = [0; 16];
    let mut current_block: u64;
    let mut dns_name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut dns_len: size_t = 0 as libc::c_int as size_t;
    if ip.octet[0 as libc::c_int as usize] as libc::c_uint == remote_dns_subnet {
        dns_name = string_from_internal_ip(ip);
        if dns_name.is_null() {
            current_block = 12289599622484323274;
        } else {
            dns_len = strlen(dns_name);
            if dns_len == 0 {
                current_block = 12289599622484323274;
            } else {
                current_block = 3276175668257526147;
            }
        }
    } else {
        current_block = 3276175668257526147;
    }
    match current_block {
        3276175668257526147 => {
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
                buff = [0; 8192];
                ip_buf = [0; 16];
                match pt as libc::c_uint {
                    1 => {
                        current_block = 4650557577079045330;
                        match current_block {
                            4650557577079045330 => return SUCCESS as libc::c_int,
                            6417057564578538666 => {
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
                                        .octet[0 as libc::c_int
                                        as usize] = 0 as libc::c_int as libc::c_uchar;
                                    ip
                                        .octet[1 as libc::c_int
                                        as usize] = 0 as libc::c_int as libc::c_uchar;
                                    ip
                                        .octet[2 as libc::c_int
                                        as usize] = 0 as libc::c_int as libc::c_uchar;
                                    ip
                                        .octet[3 as libc::c_int
                                        as usize] = 1 as libc::c_int as libc::c_uchar;
                                }
                                memcpy(
                                    &mut *buff.as_mut_ptr().offset(4 as libc::c_int as isize)
                                        as *mut libc::c_uchar as *mut libc::c_void,
                                    &mut ip as *mut ip_type as *const libc::c_void,
                                    4 as libc::c_int as libc::c_ulong,
                                );
                                len = ulen.wrapping_add(1 as libc::c_int as libc::c_ulong);
                                if len > 1 as libc::c_int as libc::c_ulong {
                                    memcpy(
                                        &mut *buff.as_mut_ptr().offset(8 as libc::c_int as isize)
                                            as *mut libc::c_uchar as *mut libc::c_void,
                                        user as *const libc::c_void,
                                        len,
                                    );
                                } else {
                                    buff[8 as libc::c_int
                                        as usize] = 0 as libc::c_int as libc::c_uchar;
                                }
                                if dns_len != 0 {
                                    memcpy(
                                        &mut *buff
                                            .as_mut_ptr()
                                            .offset(
                                                (8 as libc::c_int as libc::c_ulong).wrapping_add(len)
                                                    as isize,
                                            ) as *mut libc::c_uchar as *mut libc::c_void,
                                        dns_name as *const libc::c_void,
                                        dns_len.wrapping_add(1 as libc::c_int as libc::c_ulong),
                                    );
                                    len = (len as libc::c_ulong)
                                        .wrapping_add(
                                            dns_len.wrapping_add(1 as libc::c_int as libc::c_ulong),
                                        ) as size_t as size_t;
                                }
                                if !(len.wrapping_add(8 as libc::c_int as libc::c_ulong)
                                    != write_n_bytes(
                                        sock,
                                        buff.as_mut_ptr() as *mut libc::c_char,
                                        (8 as libc::c_int as libc::c_ulong).wrapping_add(len),
                                    ))
                                {
                                    if !(8 as libc::c_int as libc::c_ulong
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
                            13056961889198038528 => {
                                if dns_len == 0 {
                                    inet_ntop(
                                        2 as libc::c_int,
                                        &mut *(ip.octet)
                                            .as_mut_ptr()
                                            .offset(0 as libc::c_int as isize) as *mut libc::c_uchar
                                            as *const libc::c_void,
                                        ip_buf.as_mut_ptr(),
                                        ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong
                                            as socklen_t,
                                    );
                                    dns_name = ip_buf.as_mut_ptr();
                                }
                                snprintf(
                                    buff.as_mut_ptr() as *mut libc::c_char,
                                    ::std::mem::size_of::<[libc::c_uchar; 8192]>()
                                        as libc::c_ulong,
                                    b"CONNECT %s:%d HTTP/1.0\r\n\0" as *const u8
                                        as *const libc::c_char,
                                    dns_name,
                                    __bswap_16(port) as libc::c_int,
                                );
                                if *user.offset(0 as libc::c_int as isize) != 0 {
                                    let mut src: [libc::c_char; 512] = [0; 512];
                                    let mut dst: [libc::c_char; 2048] = [0; 2048];
                                    memcpy(
                                        src.as_mut_ptr() as *mut libc::c_void,
                                        user as *const libc::c_void,
                                        ulen,
                                    );
                                    memcpy(
                                        src.as_mut_ptr().offset(ulen as isize) as *mut libc::c_void,
                                        b":\0" as *const u8 as *const libc::c_char
                                            as *const libc::c_void,
                                        1 as libc::c_int as libc::c_ulong,
                                    );
                                    memcpy(
                                        src
                                            .as_mut_ptr()
                                            .offset(ulen as isize)
                                            .offset(1 as libc::c_int as isize) as *mut libc::c_void,
                                        pass as *const libc::c_void,
                                        passlen,
                                    );
                                    src[ulen
                                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                        .wrapping_add(passlen)
                                        as usize] = 0 as libc::c_int as libc::c_char;
                                    encode_base_64(
                                        src.as_mut_ptr(),
                                        dst.as_mut_ptr(),
                                        ::std::mem::size_of::<[libc::c_char; 2048]>()
                                            as libc::c_ulong as libc::c_int,
                                    );
                                    strcat(
                                        buff.as_mut_ptr() as *mut libc::c_char,
                                        b"Proxy-Authorization: Basic \0" as *const u8
                                            as *const libc::c_char,
                                    );
                                    strcat(
                                        buff.as_mut_ptr() as *mut libc::c_char,
                                        dst.as_mut_ptr(),
                                    );
                                    strcat(
                                        buff.as_mut_ptr() as *mut libc::c_char,
                                        b"\r\n\r\n\0" as *const u8 as *const libc::c_char,
                                    );
                                } else {
                                    strcat(
                                        buff.as_mut_ptr() as *mut libc::c_char,
                                        b"\r\n\0" as *const u8 as *const libc::c_char,
                                    );
                                }
                                len = strlen(buff.as_mut_ptr() as *mut libc::c_char);
                                if !(len
                                    != send(
                                        sock,
                                        buff.as_mut_ptr() as *const libc::c_void,
                                        len,
                                        0 as libc::c_int,
                                    ) as size_t)
                                {
                                    len = 0 as libc::c_int as size_t;
                                    loop {
                                        if !(len
                                            < (8 as libc::c_int * 1024 as libc::c_int) as libc::c_ulong)
                                        {
                                            current_block = 18153031941552419006;
                                            break;
                                        }
                                        if !(1 as libc::c_int as libc::c_ulong
                                            == read_n_bytes(
                                                sock,
                                                buff.as_mut_ptr().offset(len as isize) as *mut libc::c_char,
                                                1 as libc::c_int as size_t,
                                            ))
                                        {
                                            current_block = 12289599622484323274;
                                            break;
                                        }
                                        len = len.wrapping_add(1);
                                        len;
                                        if len > 4 as libc::c_int as libc::c_ulong
                                            && buff[len.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                as usize] as libc::c_int == '\n' as i32
                                            && buff[len.wrapping_sub(2 as libc::c_int as libc::c_ulong)
                                                as usize] as libc::c_int == '\r' as i32
                                            && buff[len.wrapping_sub(3 as libc::c_int as libc::c_ulong)
                                                as usize] as libc::c_int == '\n' as i32
                                            && buff[len.wrapping_sub(4 as libc::c_int as libc::c_ulong)
                                                as usize] as libc::c_int == '\r' as i32
                                        {
                                            current_block = 18153031941552419006;
                                            break;
                                        }
                                    }
                                    match current_block {
                                        12289599622484323274 => {}
                                        _ => {
                                            if len
                                                == (8 as libc::c_int * 1024 as libc::c_int) as libc::c_ulong
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
                            _ => {
                                if !user.is_null() {
                                    buff[0 as libc::c_int
                                        as usize] = 5 as libc::c_int as libc::c_uchar;
                                    buff[1 as libc::c_int
                                        as usize] = 2 as libc::c_int as libc::c_uchar;
                                    buff[2 as libc::c_int
                                        as usize] = 0 as libc::c_int as libc::c_uchar;
                                    buff[3 as libc::c_int
                                        as usize] = 2 as libc::c_int as libc::c_uchar;
                                    if 4 as libc::c_int as libc::c_ulong
                                        != write_n_bytes(
                                            sock,
                                            buff.as_mut_ptr() as *mut libc::c_char,
                                            4 as libc::c_int as size_t,
                                        )
                                    {
                                        current_block = 12289599622484323274;
                                    } else {
                                        current_block = 3580086814630675314;
                                    }
                                } else {
                                    buff[0 as libc::c_int
                                        as usize] = 5 as libc::c_int as libc::c_uchar;
                                    buff[1 as libc::c_int
                                        as usize] = 1 as libc::c_int as libc::c_uchar;
                                    buff[2 as libc::c_int
                                        as usize] = 0 as libc::c_int as libc::c_uchar;
                                    if 3 as libc::c_int as libc::c_ulong
                                        != write_n_bytes(
                                            sock,
                                            buff.as_mut_ptr() as *mut libc::c_char,
                                            3 as libc::c_int as size_t,
                                        )
                                    {
                                        current_block = 12289599622484323274;
                                    } else {
                                        current_block = 3580086814630675314;
                                    }
                                }
                                match current_block {
                                    12289599622484323274 => {}
                                    _ => {
                                        if !(2 as libc::c_int as libc::c_ulong
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
                                                    let mut c: libc::c_int = 0;
                                                    let fresh14 = cur;
                                                    cur = cur.offset(1);
                                                    *fresh14 = 1 as libc::c_int as libc::c_char;
                                                    c = (ulen & 0xff as libc::c_int as libc::c_ulong)
                                                        as libc::c_int;
                                                    let fresh15 = cur;
                                                    cur = cur.offset(1);
                                                    *fresh15 = c as libc::c_char;
                                                    memcpy(
                                                        cur as *mut libc::c_void,
                                                        user as *const libc::c_void,
                                                        c as size_t,
                                                    );
                                                    cur = cur.offset(c as isize);
                                                    c = (passlen & 0xff as libc::c_int as libc::c_ulong)
                                                        as libc::c_int;
                                                    let fresh16 = cur;
                                                    cur = cur.offset(1);
                                                    *fresh16 = c as libc::c_char;
                                                    memcpy(
                                                        cur as *mut libc::c_void,
                                                        pass as *const libc::c_void,
                                                        c as size_t,
                                                    );
                                                    cur = cur.offset(c as isize);
                                                    if cur.offset_from(out.as_mut_ptr()) as libc::c_long
                                                        as size_t
                                                        != write_n_bytes(
                                                            sock,
                                                            out.as_mut_ptr(),
                                                            cur.offset_from(out.as_mut_ptr()) as libc::c_long as size_t,
                                                        )
                                                    {
                                                        current_block = 12289599622484323274;
                                                    } else if 2 as libc::c_int as libc::c_ulong
                                                        != read_n_bytes(
                                                            sock,
                                                            in_0.as_mut_ptr(),
                                                            2 as libc::c_int as size_t,
                                                        )
                                                    {
                                                        current_block = 12289599622484323274;
                                                    } else if in_0[0 as libc::c_int as usize] as libc::c_int
                                                        != 1 as libc::c_int
                                                        || in_0[1 as libc::c_int as usize] as libc::c_int
                                                            != 0 as libc::c_int
                                                    {
                                                        if in_0[0 as libc::c_int as usize] as libc::c_int
                                                            != 1 as libc::c_int
                                                        {
                                                            current_block = 12289599622484323274;
                                                        } else {
                                                            return BLOCKED as libc::c_int
                                                        }
                                                    } else {
                                                        current_block = 2472048668343472511;
                                                    }
                                                } else {
                                                    current_block = 2472048668343472511;
                                                }
                                                match current_block {
                                                    12289599622484323274 => {}
                                                    _ => {
                                                        let mut buff_iter: size_t = 0 as libc::c_int as size_t;
                                                        let fresh17 = buff_iter;
                                                        buff_iter = buff_iter.wrapping_add(1);
                                                        buff[fresh17 as usize] = 5 as libc::c_int as libc::c_uchar;
                                                        let fresh18 = buff_iter;
                                                        buff_iter = buff_iter.wrapping_add(1);
                                                        buff[fresh18 as usize] = 1 as libc::c_int as libc::c_uchar;
                                                        let fresh19 = buff_iter;
                                                        buff_iter = buff_iter.wrapping_add(1);
                                                        buff[fresh19 as usize] = 0 as libc::c_int as libc::c_uchar;
                                                        if dns_len == 0 {
                                                            let fresh20 = buff_iter;
                                                            buff_iter = buff_iter.wrapping_add(1);
                                                            buff[fresh20 as usize] = 1 as libc::c_int as libc::c_uchar;
                                                            memcpy(
                                                                buff.as_mut_ptr().offset(buff_iter as isize)
                                                                    as *mut libc::c_void,
                                                                &mut ip as *mut ip_type as *const libc::c_void,
                                                                4 as libc::c_int as libc::c_ulong,
                                                            );
                                                            buff_iter = (buff_iter as libc::c_ulong)
                                                                .wrapping_add(4 as libc::c_int as libc::c_ulong) as size_t
                                                                as size_t;
                                                        } else {
                                                            let fresh21 = buff_iter;
                                                            buff_iter = buff_iter.wrapping_add(1);
                                                            buff[fresh21 as usize] = 3 as libc::c_int as libc::c_uchar;
                                                            let fresh22 = buff_iter;
                                                            buff_iter = buff_iter.wrapping_add(1);
                                                            buff[fresh22
                                                                as usize] = (dns_len & 0xff as libc::c_int as libc::c_ulong)
                                                                as libc::c_uchar;
                                                            memcpy(
                                                                buff.as_mut_ptr().offset(buff_iter as isize)
                                                                    as *mut libc::c_void,
                                                                dns_name as *const libc::c_void,
                                                                dns_len,
                                                            );
                                                            buff_iter = (buff_iter as libc::c_ulong)
                                                                .wrapping_add(dns_len) as size_t as size_t;
                                                        }
                                                        memcpy(
                                                            buff.as_mut_ptr().offset(buff_iter as isize)
                                                                as *mut libc::c_void,
                                                            &mut port as *mut libc::c_ushort as *const libc::c_void,
                                                            2 as libc::c_int as libc::c_ulong,
                                                        );
                                                        buff_iter = (buff_iter as libc::c_ulong)
                                                            .wrapping_add(2 as libc::c_int as libc::c_ulong) as size_t
                                                            as size_t;
                                                        if !(buff_iter
                                                            != write_n_bytes(
                                                                sock,
                                                                buff.as_mut_ptr() as *mut libc::c_char,
                                                                buff_iter,
                                                            ))
                                                        {
                                                            if !(4 as libc::c_int as libc::c_ulong
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
                                                                            current_block = 2824739507554377001;
                                                                            match current_block {
                                                                                13060446176355317402 => {
                                                                                    len = 16 as libc::c_int as size_t;
                                                                                    current_block = 13003737910779602957;
                                                                                }
                                                                                2824739507554377001 => {
                                                                                    len = 4 as libc::c_int as size_t;
                                                                                    current_block = 13003737910779602957;
                                                                                }
                                                                                _ => {
                                                                                    len = 0 as libc::c_int as size_t;
                                                                                    if 1 as libc::c_int as libc::c_ulong
                                                                                        != read_n_bytes(
                                                                                            sock,
                                                                                            &mut len as *mut size_t as *mut libc::c_char,
                                                                                            1 as libc::c_int as size_t,
                                                                                        )
                                                                                    {
                                                                                        current_block = 12289599622484323274;
                                                                                    } else {
                                                                                        current_block = 13003737910779602957;
                                                                                    }
                                                                                }
                                                                            }
                                                                            match current_block {
                                                                                12289599622484323274 => {}
                                                                                _ => {
                                                                                    if !(len.wrapping_add(2 as libc::c_int as libc::c_ulong)
                                                                                        != read_n_bytes(
                                                                                            sock,
                                                                                            buff.as_mut_ptr() as *mut libc::c_char,
                                                                                            len.wrapping_add(2 as libc::c_int as libc::c_ulong),
                                                                                        ))
                                                                                    {
                                                                                        return SUCCESS as libc::c_int;
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                        4 => {
                                                                            current_block = 13060446176355317402;
                                                                            match current_block {
                                                                                13060446176355317402 => {
                                                                                    len = 16 as libc::c_int as size_t;
                                                                                    current_block = 13003737910779602957;
                                                                                }
                                                                                2824739507554377001 => {
                                                                                    len = 4 as libc::c_int as size_t;
                                                                                    current_block = 13003737910779602957;
                                                                                }
                                                                                _ => {
                                                                                    len = 0 as libc::c_int as size_t;
                                                                                    if 1 as libc::c_int as libc::c_ulong
                                                                                        != read_n_bytes(
                                                                                            sock,
                                                                                            &mut len as *mut size_t as *mut libc::c_char,
                                                                                            1 as libc::c_int as size_t,
                                                                                        )
                                                                                    {
                                                                                        current_block = 12289599622484323274;
                                                                                    } else {
                                                                                        current_block = 13003737910779602957;
                                                                                    }
                                                                                }
                                                                            }
                                                                            match current_block {
                                                                                12289599622484323274 => {}
                                                                                _ => {
                                                                                    if !(len.wrapping_add(2 as libc::c_int as libc::c_ulong)
                                                                                        != read_n_bytes(
                                                                                            sock,
                                                                                            buff.as_mut_ptr() as *mut libc::c_char,
                                                                                            len.wrapping_add(2 as libc::c_int as libc::c_ulong),
                                                                                        ))
                                                                                    {
                                                                                        return SUCCESS as libc::c_int;
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                        3 => {
                                                                            current_block = 5664113730932431836;
                                                                            match current_block {
                                                                                13060446176355317402 => {
                                                                                    len = 16 as libc::c_int as size_t;
                                                                                    current_block = 13003737910779602957;
                                                                                }
                                                                                2824739507554377001 => {
                                                                                    len = 4 as libc::c_int as size_t;
                                                                                    current_block = 13003737910779602957;
                                                                                }
                                                                                _ => {
                                                                                    len = 0 as libc::c_int as size_t;
                                                                                    if 1 as libc::c_int as libc::c_ulong
                                                                                        != read_n_bytes(
                                                                                            sock,
                                                                                            &mut len as *mut size_t as *mut libc::c_char,
                                                                                            1 as libc::c_int as size_t,
                                                                                        )
                                                                                    {
                                                                                        current_block = 12289599622484323274;
                                                                                    } else {
                                                                                        current_block = 13003737910779602957;
                                                                                    }
                                                                                }
                                                                            }
                                                                            match current_block {
                                                                                12289599622484323274 => {}
                                                                                _ => {
                                                                                    if !(len.wrapping_add(2 as libc::c_int as libc::c_ulong)
                                                                                        != read_n_bytes(
                                                                                            sock,
                                                                                            buff.as_mut_ptr() as *mut libc::c_char,
                                                                                            len.wrapping_add(2 as libc::c_int as libc::c_ulong),
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
                    }
                    0 => {
                        current_block = 13056961889198038528;
                        match current_block {
                            4650557577079045330 => return SUCCESS as libc::c_int,
                            6417057564578538666 => {
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
                                        .octet[0 as libc::c_int
                                        as usize] = 0 as libc::c_int as libc::c_uchar;
                                    ip
                                        .octet[1 as libc::c_int
                                        as usize] = 0 as libc::c_int as libc::c_uchar;
                                    ip
                                        .octet[2 as libc::c_int
                                        as usize] = 0 as libc::c_int as libc::c_uchar;
                                    ip
                                        .octet[3 as libc::c_int
                                        as usize] = 1 as libc::c_int as libc::c_uchar;
                                }
                                memcpy(
                                    &mut *buff.as_mut_ptr().offset(4 as libc::c_int as isize)
                                        as *mut libc::c_uchar as *mut libc::c_void,
                                    &mut ip as *mut ip_type as *const libc::c_void,
                                    4 as libc::c_int as libc::c_ulong,
                                );
                                len = ulen.wrapping_add(1 as libc::c_int as libc::c_ulong);
                                if len > 1 as libc::c_int as libc::c_ulong {
                                    memcpy(
                                        &mut *buff.as_mut_ptr().offset(8 as libc::c_int as isize)
                                            as *mut libc::c_uchar as *mut libc::c_void,
                                        user as *const libc::c_void,
                                        len,
                                    );
                                } else {
                                    buff[8 as libc::c_int
                                        as usize] = 0 as libc::c_int as libc::c_uchar;
                                }
                                if dns_len != 0 {
                                    memcpy(
                                        &mut *buff
                                            .as_mut_ptr()
                                            .offset(
                                                (8 as libc::c_int as libc::c_ulong).wrapping_add(len)
                                                    as isize,
                                            ) as *mut libc::c_uchar as *mut libc::c_void,
                                        dns_name as *const libc::c_void,
                                        dns_len.wrapping_add(1 as libc::c_int as libc::c_ulong),
                                    );
                                    len = (len as libc::c_ulong)
                                        .wrapping_add(
                                            dns_len.wrapping_add(1 as libc::c_int as libc::c_ulong),
                                        ) as size_t as size_t;
                                }
                                if !(len.wrapping_add(8 as libc::c_int as libc::c_ulong)
                                    != write_n_bytes(
                                        sock,
                                        buff.as_mut_ptr() as *mut libc::c_char,
                                        (8 as libc::c_int as libc::c_ulong).wrapping_add(len),
                                    ))
                                {
                                    if !(8 as libc::c_int as libc::c_ulong
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
                            13056961889198038528 => {
                                if dns_len == 0 {
                                    inet_ntop(
                                        2 as libc::c_int,
                                        &mut *(ip.octet)
                                            .as_mut_ptr()
                                            .offset(0 as libc::c_int as isize) as *mut libc::c_uchar
                                            as *const libc::c_void,
                                        ip_buf.as_mut_ptr(),
                                        ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong
                                            as socklen_t,
                                    );
                                    dns_name = ip_buf.as_mut_ptr();
                                }
                                snprintf(
                                    buff.as_mut_ptr() as *mut libc::c_char,
                                    ::std::mem::size_of::<[libc::c_uchar; 8192]>()
                                        as libc::c_ulong,
                                    b"CONNECT %s:%d HTTP/1.0\r\n\0" as *const u8
                                        as *const libc::c_char,
                                    dns_name,
                                    __bswap_16(port) as libc::c_int,
                                );
                                if *user.offset(0 as libc::c_int as isize) != 0 {
                                    let mut src: [libc::c_char; 512] = [0; 512];
                                    let mut dst: [libc::c_char; 2048] = [0; 2048];
                                    memcpy(
                                        src.as_mut_ptr() as *mut libc::c_void,
                                        user as *const libc::c_void,
                                        ulen,
                                    );
                                    memcpy(
                                        src.as_mut_ptr().offset(ulen as isize) as *mut libc::c_void,
                                        b":\0" as *const u8 as *const libc::c_char
                                            as *const libc::c_void,
                                        1 as libc::c_int as libc::c_ulong,
                                    );
                                    memcpy(
                                        src
                                            .as_mut_ptr()
                                            .offset(ulen as isize)
                                            .offset(1 as libc::c_int as isize) as *mut libc::c_void,
                                        pass as *const libc::c_void,
                                        passlen,
                                    );
                                    src[ulen
                                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                        .wrapping_add(passlen)
                                        as usize] = 0 as libc::c_int as libc::c_char;
                                    encode_base_64(
                                        src.as_mut_ptr(),
                                        dst.as_mut_ptr(),
                                        ::std::mem::size_of::<[libc::c_char; 2048]>()
                                            as libc::c_ulong as libc::c_int,
                                    );
                                    strcat(
                                        buff.as_mut_ptr() as *mut libc::c_char,
                                        b"Proxy-Authorization: Basic \0" as *const u8
                                            as *const libc::c_char,
                                    );
                                    strcat(
                                        buff.as_mut_ptr() as *mut libc::c_char,
                                        dst.as_mut_ptr(),
                                    );
                                    strcat(
                                        buff.as_mut_ptr() as *mut libc::c_char,
                                        b"\r\n\r\n\0" as *const u8 as *const libc::c_char,
                                    );
                                } else {
                                    strcat(
                                        buff.as_mut_ptr() as *mut libc::c_char,
                                        b"\r\n\0" as *const u8 as *const libc::c_char,
                                    );
                                }
                                len = strlen(buff.as_mut_ptr() as *mut libc::c_char);
                                if !(len
                                    != send(
                                        sock,
                                        buff.as_mut_ptr() as *const libc::c_void,
                                        len,
                                        0 as libc::c_int,
                                    ) as size_t)
                                {
                                    len = 0 as libc::c_int as size_t;
                                    loop {
                                        if !(len
                                            < (8 as libc::c_int * 1024 as libc::c_int) as libc::c_ulong)
                                        {
                                            current_block = 18153031941552419006;
                                            break;
                                        }
                                        if !(1 as libc::c_int as libc::c_ulong
                                            == read_n_bytes(
                                                sock,
                                                buff.as_mut_ptr().offset(len as isize) as *mut libc::c_char,
                                                1 as libc::c_int as size_t,
                                            ))
                                        {
                                            current_block = 12289599622484323274;
                                            break;
                                        }
                                        len = len.wrapping_add(1);
                                        len;
                                        if len > 4 as libc::c_int as libc::c_ulong
                                            && buff[len.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                as usize] as libc::c_int == '\n' as i32
                                            && buff[len.wrapping_sub(2 as libc::c_int as libc::c_ulong)
                                                as usize] as libc::c_int == '\r' as i32
                                            && buff[len.wrapping_sub(3 as libc::c_int as libc::c_ulong)
                                                as usize] as libc::c_int == '\n' as i32
                                            && buff[len.wrapping_sub(4 as libc::c_int as libc::c_ulong)
                                                as usize] as libc::c_int == '\r' as i32
                                        {
                                            current_block = 18153031941552419006;
                                            break;
                                        }
                                    }
                                    match current_block {
                                        12289599622484323274 => {}
                                        _ => {
                                            if len
                                                == (8 as libc::c_int * 1024 as libc::c_int) as libc::c_ulong
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
                            _ => {
                                if !user.is_null() {
                                    buff[0 as libc::c_int
                                        as usize] = 5 as libc::c_int as libc::c_uchar;
                                    buff[1 as libc::c_int
                                        as usize] = 2 as libc::c_int as libc::c_uchar;
                                    buff[2 as libc::c_int
                                        as usize] = 0 as libc::c_int as libc::c_uchar;
                                    buff[3 as libc::c_int
                                        as usize] = 2 as libc::c_int as libc::c_uchar;
                                    if 4 as libc::c_int as libc::c_ulong
                                        != write_n_bytes(
                                            sock,
                                            buff.as_mut_ptr() as *mut libc::c_char,
                                            4 as libc::c_int as size_t,
                                        )
                                    {
                                        current_block = 12289599622484323274;
                                    } else {
                                        current_block = 3580086814630675314;
                                    }
                                } else {
                                    buff[0 as libc::c_int
                                        as usize] = 5 as libc::c_int as libc::c_uchar;
                                    buff[1 as libc::c_int
                                        as usize] = 1 as libc::c_int as libc::c_uchar;
                                    buff[2 as libc::c_int
                                        as usize] = 0 as libc::c_int as libc::c_uchar;
                                    if 3 as libc::c_int as libc::c_ulong
                                        != write_n_bytes(
                                            sock,
                                            buff.as_mut_ptr() as *mut libc::c_char,
                                            3 as libc::c_int as size_t,
                                        )
                                    {
                                        current_block = 12289599622484323274;
                                    } else {
                                        current_block = 3580086814630675314;
                                    }
                                }
                                match current_block {
                                    12289599622484323274 => {}
                                    _ => {
                                        if !(2 as libc::c_int as libc::c_ulong
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
                                                    let mut c: libc::c_int = 0;
                                                    let fresh14 = cur;
                                                    cur = cur.offset(1);
                                                    *fresh14 = 1 as libc::c_int as libc::c_char;
                                                    c = (ulen & 0xff as libc::c_int as libc::c_ulong)
                                                        as libc::c_int;
                                                    let fresh15 = cur;
                                                    cur = cur.offset(1);
                                                    *fresh15 = c as libc::c_char;
                                                    memcpy(
                                                        cur as *mut libc::c_void,
                                                        user as *const libc::c_void,
                                                        c as size_t,
                                                    );
                                                    cur = cur.offset(c as isize);
                                                    c = (passlen & 0xff as libc::c_int as libc::c_ulong)
                                                        as libc::c_int;
                                                    let fresh16 = cur;
                                                    cur = cur.offset(1);
                                                    *fresh16 = c as libc::c_char;
                                                    memcpy(
                                                        cur as *mut libc::c_void,
                                                        pass as *const libc::c_void,
                                                        c as size_t,
                                                    );
                                                    cur = cur.offset(c as isize);
                                                    if cur.offset_from(out.as_mut_ptr()) as libc::c_long
                                                        as size_t
                                                        != write_n_bytes(
                                                            sock,
                                                            out.as_mut_ptr(),
                                                            cur.offset_from(out.as_mut_ptr()) as libc::c_long as size_t,
                                                        )
                                                    {
                                                        current_block = 12289599622484323274;
                                                    } else if 2 as libc::c_int as libc::c_ulong
                                                        != read_n_bytes(
                                                            sock,
                                                            in_0.as_mut_ptr(),
                                                            2 as libc::c_int as size_t,
                                                        )
                                                    {
                                                        current_block = 12289599622484323274;
                                                    } else if in_0[0 as libc::c_int as usize] as libc::c_int
                                                        != 1 as libc::c_int
                                                        || in_0[1 as libc::c_int as usize] as libc::c_int
                                                            != 0 as libc::c_int
                                                    {
                                                        if in_0[0 as libc::c_int as usize] as libc::c_int
                                                            != 1 as libc::c_int
                                                        {
                                                            current_block = 12289599622484323274;
                                                        } else {
                                                            return BLOCKED as libc::c_int
                                                        }
                                                    } else {
                                                        current_block = 2472048668343472511;
                                                    }
                                                } else {
                                                    current_block = 2472048668343472511;
                                                }
                                                match current_block {
                                                    12289599622484323274 => {}
                                                    _ => {
                                                        let mut buff_iter: size_t = 0 as libc::c_int as size_t;
                                                        let fresh17 = buff_iter;
                                                        buff_iter = buff_iter.wrapping_add(1);
                                                        buff[fresh17 as usize] = 5 as libc::c_int as libc::c_uchar;
                                                        let fresh18 = buff_iter;
                                                        buff_iter = buff_iter.wrapping_add(1);
                                                        buff[fresh18 as usize] = 1 as libc::c_int as libc::c_uchar;
                                                        let fresh19 = buff_iter;
                                                        buff_iter = buff_iter.wrapping_add(1);
                                                        buff[fresh19 as usize] = 0 as libc::c_int as libc::c_uchar;
                                                        if dns_len == 0 {
                                                            let fresh20 = buff_iter;
                                                            buff_iter = buff_iter.wrapping_add(1);
                                                            buff[fresh20 as usize] = 1 as libc::c_int as libc::c_uchar;
                                                            memcpy(
                                                                buff.as_mut_ptr().offset(buff_iter as isize)
                                                                    as *mut libc::c_void,
                                                                &mut ip as *mut ip_type as *const libc::c_void,
                                                                4 as libc::c_int as libc::c_ulong,
                                                            );
                                                            buff_iter = (buff_iter as libc::c_ulong)
                                                                .wrapping_add(4 as libc::c_int as libc::c_ulong) as size_t
                                                                as size_t;
                                                        } else {
                                                            let fresh21 = buff_iter;
                                                            buff_iter = buff_iter.wrapping_add(1);
                                                            buff[fresh21 as usize] = 3 as libc::c_int as libc::c_uchar;
                                                            let fresh22 = buff_iter;
                                                            buff_iter = buff_iter.wrapping_add(1);
                                                            buff[fresh22
                                                                as usize] = (dns_len & 0xff as libc::c_int as libc::c_ulong)
                                                                as libc::c_uchar;
                                                            memcpy(
                                                                buff.as_mut_ptr().offset(buff_iter as isize)
                                                                    as *mut libc::c_void,
                                                                dns_name as *const libc::c_void,
                                                                dns_len,
                                                            );
                                                            buff_iter = (buff_iter as libc::c_ulong)
                                                                .wrapping_add(dns_len) as size_t as size_t;
                                                        }
                                                        memcpy(
                                                            buff.as_mut_ptr().offset(buff_iter as isize)
                                                                as *mut libc::c_void,
                                                            &mut port as *mut libc::c_ushort as *const libc::c_void,
                                                            2 as libc::c_int as libc::c_ulong,
                                                        );
                                                        buff_iter = (buff_iter as libc::c_ulong)
                                                            .wrapping_add(2 as libc::c_int as libc::c_ulong) as size_t
                                                            as size_t;
                                                        if !(buff_iter
                                                            != write_n_bytes(
                                                                sock,
                                                                buff.as_mut_ptr() as *mut libc::c_char,
                                                                buff_iter,
                                                            ))
                                                        {
                                                            if !(4 as libc::c_int as libc::c_ulong
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
                                                                            current_block = 2824739507554377001;
                                                                            match current_block {
                                                                                13060446176355317402 => {
                                                                                    len = 16 as libc::c_int as size_t;
                                                                                    current_block = 13003737910779602957;
                                                                                }
                                                                                2824739507554377001 => {
                                                                                    len = 4 as libc::c_int as size_t;
                                                                                    current_block = 13003737910779602957;
                                                                                }
                                                                                _ => {
                                                                                    len = 0 as libc::c_int as size_t;
                                                                                    if 1 as libc::c_int as libc::c_ulong
                                                                                        != read_n_bytes(
                                                                                            sock,
                                                                                            &mut len as *mut size_t as *mut libc::c_char,
                                                                                            1 as libc::c_int as size_t,
                                                                                        )
                                                                                    {
                                                                                        current_block = 12289599622484323274;
                                                                                    } else {
                                                                                        current_block = 13003737910779602957;
                                                                                    }
                                                                                }
                                                                            }
                                                                            match current_block {
                                                                                12289599622484323274 => {}
                                                                                _ => {
                                                                                    if !(len.wrapping_add(2 as libc::c_int as libc::c_ulong)
                                                                                        != read_n_bytes(
                                                                                            sock,
                                                                                            buff.as_mut_ptr() as *mut libc::c_char,
                                                                                            len.wrapping_add(2 as libc::c_int as libc::c_ulong),
                                                                                        ))
                                                                                    {
                                                                                        return SUCCESS as libc::c_int;
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                        4 => {
                                                                            current_block = 13060446176355317402;
                                                                            match current_block {
                                                                                13060446176355317402 => {
                                                                                    len = 16 as libc::c_int as size_t;
                                                                                    current_block = 13003737910779602957;
                                                                                }
                                                                                2824739507554377001 => {
                                                                                    len = 4 as libc::c_int as size_t;
                                                                                    current_block = 13003737910779602957;
                                                                                }
                                                                                _ => {
                                                                                    len = 0 as libc::c_int as size_t;
                                                                                    if 1 as libc::c_int as libc::c_ulong
                                                                                        != read_n_bytes(
                                                                                            sock,
                                                                                            &mut len as *mut size_t as *mut libc::c_char,
                                                                                            1 as libc::c_int as size_t,
                                                                                        )
                                                                                    {
                                                                                        current_block = 12289599622484323274;
                                                                                    } else {
                                                                                        current_block = 13003737910779602957;
                                                                                    }
                                                                                }
                                                                            }
                                                                            match current_block {
                                                                                12289599622484323274 => {}
                                                                                _ => {
                                                                                    if !(len.wrapping_add(2 as libc::c_int as libc::c_ulong)
                                                                                        != read_n_bytes(
                                                                                            sock,
                                                                                            buff.as_mut_ptr() as *mut libc::c_char,
                                                                                            len.wrapping_add(2 as libc::c_int as libc::c_ulong),
                                                                                        ))
                                                                                    {
                                                                                        return SUCCESS as libc::c_int;
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                        3 => {
                                                                            current_block = 5664113730932431836;
                                                                            match current_block {
                                                                                13060446176355317402 => {
                                                                                    len = 16 as libc::c_int as size_t;
                                                                                    current_block = 13003737910779602957;
                                                                                }
                                                                                2824739507554377001 => {
                                                                                    len = 4 as libc::c_int as size_t;
                                                                                    current_block = 13003737910779602957;
                                                                                }
                                                                                _ => {
                                                                                    len = 0 as libc::c_int as size_t;
                                                                                    if 1 as libc::c_int as libc::c_ulong
                                                                                        != read_n_bytes(
                                                                                            sock,
                                                                                            &mut len as *mut size_t as *mut libc::c_char,
                                                                                            1 as libc::c_int as size_t,
                                                                                        )
                                                                                    {
                                                                                        current_block = 12289599622484323274;
                                                                                    } else {
                                                                                        current_block = 13003737910779602957;
                                                                                    }
                                                                                }
                                                                            }
                                                                            match current_block {
                                                                                12289599622484323274 => {}
                                                                                _ => {
                                                                                    if !(len.wrapping_add(2 as libc::c_int as libc::c_ulong)
                                                                                        != read_n_bytes(
                                                                                            sock,
                                                                                            buff.as_mut_ptr() as *mut libc::c_char,
                                                                                            len.wrapping_add(2 as libc::c_int as libc::c_ulong),
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
                    }
                    2 => {
                        current_block = 6417057564578538666;
                        match current_block {
                            4650557577079045330 => return SUCCESS as libc::c_int,
                            6417057564578538666 => {
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
                                        .octet[0 as libc::c_int
                                        as usize] = 0 as libc::c_int as libc::c_uchar;
                                    ip
                                        .octet[1 as libc::c_int
                                        as usize] = 0 as libc::c_int as libc::c_uchar;
                                    ip
                                        .octet[2 as libc::c_int
                                        as usize] = 0 as libc::c_int as libc::c_uchar;
                                    ip
                                        .octet[3 as libc::c_int
                                        as usize] = 1 as libc::c_int as libc::c_uchar;
                                }
                                memcpy(
                                    &mut *buff.as_mut_ptr().offset(4 as libc::c_int as isize)
                                        as *mut libc::c_uchar as *mut libc::c_void,
                                    &mut ip as *mut ip_type as *const libc::c_void,
                                    4 as libc::c_int as libc::c_ulong,
                                );
                                len = ulen.wrapping_add(1 as libc::c_int as libc::c_ulong);
                                if len > 1 as libc::c_int as libc::c_ulong {
                                    memcpy(
                                        &mut *buff.as_mut_ptr().offset(8 as libc::c_int as isize)
                                            as *mut libc::c_uchar as *mut libc::c_void,
                                        user as *const libc::c_void,
                                        len,
                                    );
                                } else {
                                    buff[8 as libc::c_int
                                        as usize] = 0 as libc::c_int as libc::c_uchar;
                                }
                                if dns_len != 0 {
                                    memcpy(
                                        &mut *buff
                                            .as_mut_ptr()
                                            .offset(
                                                (8 as libc::c_int as libc::c_ulong).wrapping_add(len)
                                                    as isize,
                                            ) as *mut libc::c_uchar as *mut libc::c_void,
                                        dns_name as *const libc::c_void,
                                        dns_len.wrapping_add(1 as libc::c_int as libc::c_ulong),
                                    );
                                    len = (len as libc::c_ulong)
                                        .wrapping_add(
                                            dns_len.wrapping_add(1 as libc::c_int as libc::c_ulong),
                                        ) as size_t as size_t;
                                }
                                if !(len.wrapping_add(8 as libc::c_int as libc::c_ulong)
                                    != write_n_bytes(
                                        sock,
                                        buff.as_mut_ptr() as *mut libc::c_char,
                                        (8 as libc::c_int as libc::c_ulong).wrapping_add(len),
                                    ))
                                {
                                    if !(8 as libc::c_int as libc::c_ulong
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
                            13056961889198038528 => {
                                if dns_len == 0 {
                                    inet_ntop(
                                        2 as libc::c_int,
                                        &mut *(ip.octet)
                                            .as_mut_ptr()
                                            .offset(0 as libc::c_int as isize) as *mut libc::c_uchar
                                            as *const libc::c_void,
                                        ip_buf.as_mut_ptr(),
                                        ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong
                                            as socklen_t,
                                    );
                                    dns_name = ip_buf.as_mut_ptr();
                                }
                                snprintf(
                                    buff.as_mut_ptr() as *mut libc::c_char,
                                    ::std::mem::size_of::<[libc::c_uchar; 8192]>()
                                        as libc::c_ulong,
                                    b"CONNECT %s:%d HTTP/1.0\r\n\0" as *const u8
                                        as *const libc::c_char,
                                    dns_name,
                                    __bswap_16(port) as libc::c_int,
                                );
                                if *user.offset(0 as libc::c_int as isize) != 0 {
                                    let mut src: [libc::c_char; 512] = [0; 512];
                                    let mut dst: [libc::c_char; 2048] = [0; 2048];
                                    memcpy(
                                        src.as_mut_ptr() as *mut libc::c_void,
                                        user as *const libc::c_void,
                                        ulen,
                                    );
                                    memcpy(
                                        src.as_mut_ptr().offset(ulen as isize) as *mut libc::c_void,
                                        b":\0" as *const u8 as *const libc::c_char
                                            as *const libc::c_void,
                                        1 as libc::c_int as libc::c_ulong,
                                    );
                                    memcpy(
                                        src
                                            .as_mut_ptr()
                                            .offset(ulen as isize)
                                            .offset(1 as libc::c_int as isize) as *mut libc::c_void,
                                        pass as *const libc::c_void,
                                        passlen,
                                    );
                                    src[ulen
                                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                        .wrapping_add(passlen)
                                        as usize] = 0 as libc::c_int as libc::c_char;
                                    encode_base_64(
                                        src.as_mut_ptr(),
                                        dst.as_mut_ptr(),
                                        ::std::mem::size_of::<[libc::c_char; 2048]>()
                                            as libc::c_ulong as libc::c_int,
                                    );
                                    strcat(
                                        buff.as_mut_ptr() as *mut libc::c_char,
                                        b"Proxy-Authorization: Basic \0" as *const u8
                                            as *const libc::c_char,
                                    );
                                    strcat(
                                        buff.as_mut_ptr() as *mut libc::c_char,
                                        dst.as_mut_ptr(),
                                    );
                                    strcat(
                                        buff.as_mut_ptr() as *mut libc::c_char,
                                        b"\r\n\r\n\0" as *const u8 as *const libc::c_char,
                                    );
                                } else {
                                    strcat(
                                        buff.as_mut_ptr() as *mut libc::c_char,
                                        b"\r\n\0" as *const u8 as *const libc::c_char,
                                    );
                                }
                                len = strlen(buff.as_mut_ptr() as *mut libc::c_char);
                                if !(len
                                    != send(
                                        sock,
                                        buff.as_mut_ptr() as *const libc::c_void,
                                        len,
                                        0 as libc::c_int,
                                    ) as size_t)
                                {
                                    len = 0 as libc::c_int as size_t;
                                    loop {
                                        if !(len
                                            < (8 as libc::c_int * 1024 as libc::c_int) as libc::c_ulong)
                                        {
                                            current_block = 18153031941552419006;
                                            break;
                                        }
                                        if !(1 as libc::c_int as libc::c_ulong
                                            == read_n_bytes(
                                                sock,
                                                buff.as_mut_ptr().offset(len as isize) as *mut libc::c_char,
                                                1 as libc::c_int as size_t,
                                            ))
                                        {
                                            current_block = 12289599622484323274;
                                            break;
                                        }
                                        len = len.wrapping_add(1);
                                        len;
                                        if len > 4 as libc::c_int as libc::c_ulong
                                            && buff[len.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                as usize] as libc::c_int == '\n' as i32
                                            && buff[len.wrapping_sub(2 as libc::c_int as libc::c_ulong)
                                                as usize] as libc::c_int == '\r' as i32
                                            && buff[len.wrapping_sub(3 as libc::c_int as libc::c_ulong)
                                                as usize] as libc::c_int == '\n' as i32
                                            && buff[len.wrapping_sub(4 as libc::c_int as libc::c_ulong)
                                                as usize] as libc::c_int == '\r' as i32
                                        {
                                            current_block = 18153031941552419006;
                                            break;
                                        }
                                    }
                                    match current_block {
                                        12289599622484323274 => {}
                                        _ => {
                                            if len
                                                == (8 as libc::c_int * 1024 as libc::c_int) as libc::c_ulong
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
                            _ => {
                                if !user.is_null() {
                                    buff[0 as libc::c_int
                                        as usize] = 5 as libc::c_int as libc::c_uchar;
                                    buff[1 as libc::c_int
                                        as usize] = 2 as libc::c_int as libc::c_uchar;
                                    buff[2 as libc::c_int
                                        as usize] = 0 as libc::c_int as libc::c_uchar;
                                    buff[3 as libc::c_int
                                        as usize] = 2 as libc::c_int as libc::c_uchar;
                                    if 4 as libc::c_int as libc::c_ulong
                                        != write_n_bytes(
                                            sock,
                                            buff.as_mut_ptr() as *mut libc::c_char,
                                            4 as libc::c_int as size_t,
                                        )
                                    {
                                        current_block = 12289599622484323274;
                                    } else {
                                        current_block = 3580086814630675314;
                                    }
                                } else {
                                    buff[0 as libc::c_int
                                        as usize] = 5 as libc::c_int as libc::c_uchar;
                                    buff[1 as libc::c_int
                                        as usize] = 1 as libc::c_int as libc::c_uchar;
                                    buff[2 as libc::c_int
                                        as usize] = 0 as libc::c_int as libc::c_uchar;
                                    if 3 as libc::c_int as libc::c_ulong
                                        != write_n_bytes(
                                            sock,
                                            buff.as_mut_ptr() as *mut libc::c_char,
                                            3 as libc::c_int as size_t,
                                        )
                                    {
                                        current_block = 12289599622484323274;
                                    } else {
                                        current_block = 3580086814630675314;
                                    }
                                }
                                match current_block {
                                    12289599622484323274 => {}
                                    _ => {
                                        if !(2 as libc::c_int as libc::c_ulong
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
                                                    let mut c: libc::c_int = 0;
                                                    let fresh14 = cur;
                                                    cur = cur.offset(1);
                                                    *fresh14 = 1 as libc::c_int as libc::c_char;
                                                    c = (ulen & 0xff as libc::c_int as libc::c_ulong)
                                                        as libc::c_int;
                                                    let fresh15 = cur;
                                                    cur = cur.offset(1);
                                                    *fresh15 = c as libc::c_char;
                                                    memcpy(
                                                        cur as *mut libc::c_void,
                                                        user as *const libc::c_void,
                                                        c as size_t,
                                                    );
                                                    cur = cur.offset(c as isize);
                                                    c = (passlen & 0xff as libc::c_int as libc::c_ulong)
                                                        as libc::c_int;
                                                    let fresh16 = cur;
                                                    cur = cur.offset(1);
                                                    *fresh16 = c as libc::c_char;
                                                    memcpy(
                                                        cur as *mut libc::c_void,
                                                        pass as *const libc::c_void,
                                                        c as size_t,
                                                    );
                                                    cur = cur.offset(c as isize);
                                                    if cur.offset_from(out.as_mut_ptr()) as libc::c_long
                                                        as size_t
                                                        != write_n_bytes(
                                                            sock,
                                                            out.as_mut_ptr(),
                                                            cur.offset_from(out.as_mut_ptr()) as libc::c_long as size_t,
                                                        )
                                                    {
                                                        current_block = 12289599622484323274;
                                                    } else if 2 as libc::c_int as libc::c_ulong
                                                        != read_n_bytes(
                                                            sock,
                                                            in_0.as_mut_ptr(),
                                                            2 as libc::c_int as size_t,
                                                        )
                                                    {
                                                        current_block = 12289599622484323274;
                                                    } else if in_0[0 as libc::c_int as usize] as libc::c_int
                                                        != 1 as libc::c_int
                                                        || in_0[1 as libc::c_int as usize] as libc::c_int
                                                            != 0 as libc::c_int
                                                    {
                                                        if in_0[0 as libc::c_int as usize] as libc::c_int
                                                            != 1 as libc::c_int
                                                        {
                                                            current_block = 12289599622484323274;
                                                        } else {
                                                            return BLOCKED as libc::c_int
                                                        }
                                                    } else {
                                                        current_block = 2472048668343472511;
                                                    }
                                                } else {
                                                    current_block = 2472048668343472511;
                                                }
                                                match current_block {
                                                    12289599622484323274 => {}
                                                    _ => {
                                                        let mut buff_iter: size_t = 0 as libc::c_int as size_t;
                                                        let fresh17 = buff_iter;
                                                        buff_iter = buff_iter.wrapping_add(1);
                                                        buff[fresh17 as usize] = 5 as libc::c_int as libc::c_uchar;
                                                        let fresh18 = buff_iter;
                                                        buff_iter = buff_iter.wrapping_add(1);
                                                        buff[fresh18 as usize] = 1 as libc::c_int as libc::c_uchar;
                                                        let fresh19 = buff_iter;
                                                        buff_iter = buff_iter.wrapping_add(1);
                                                        buff[fresh19 as usize] = 0 as libc::c_int as libc::c_uchar;
                                                        if dns_len == 0 {
                                                            let fresh20 = buff_iter;
                                                            buff_iter = buff_iter.wrapping_add(1);
                                                            buff[fresh20 as usize] = 1 as libc::c_int as libc::c_uchar;
                                                            memcpy(
                                                                buff.as_mut_ptr().offset(buff_iter as isize)
                                                                    as *mut libc::c_void,
                                                                &mut ip as *mut ip_type as *const libc::c_void,
                                                                4 as libc::c_int as libc::c_ulong,
                                                            );
                                                            buff_iter = (buff_iter as libc::c_ulong)
                                                                .wrapping_add(4 as libc::c_int as libc::c_ulong) as size_t
                                                                as size_t;
                                                        } else {
                                                            let fresh21 = buff_iter;
                                                            buff_iter = buff_iter.wrapping_add(1);
                                                            buff[fresh21 as usize] = 3 as libc::c_int as libc::c_uchar;
                                                            let fresh22 = buff_iter;
                                                            buff_iter = buff_iter.wrapping_add(1);
                                                            buff[fresh22
                                                                as usize] = (dns_len & 0xff as libc::c_int as libc::c_ulong)
                                                                as libc::c_uchar;
                                                            memcpy(
                                                                buff.as_mut_ptr().offset(buff_iter as isize)
                                                                    as *mut libc::c_void,
                                                                dns_name as *const libc::c_void,
                                                                dns_len,
                                                            );
                                                            buff_iter = (buff_iter as libc::c_ulong)
                                                                .wrapping_add(dns_len) as size_t as size_t;
                                                        }
                                                        memcpy(
                                                            buff.as_mut_ptr().offset(buff_iter as isize)
                                                                as *mut libc::c_void,
                                                            &mut port as *mut libc::c_ushort as *const libc::c_void,
                                                            2 as libc::c_int as libc::c_ulong,
                                                        );
                                                        buff_iter = (buff_iter as libc::c_ulong)
                                                            .wrapping_add(2 as libc::c_int as libc::c_ulong) as size_t
                                                            as size_t;
                                                        if !(buff_iter
                                                            != write_n_bytes(
                                                                sock,
                                                                buff.as_mut_ptr() as *mut libc::c_char,
                                                                buff_iter,
                                                            ))
                                                        {
                                                            if !(4 as libc::c_int as libc::c_ulong
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
                                                                            current_block = 2824739507554377001;
                                                                            match current_block {
                                                                                13060446176355317402 => {
                                                                                    len = 16 as libc::c_int as size_t;
                                                                                    current_block = 13003737910779602957;
                                                                                }
                                                                                2824739507554377001 => {
                                                                                    len = 4 as libc::c_int as size_t;
                                                                                    current_block = 13003737910779602957;
                                                                                }
                                                                                _ => {
                                                                                    len = 0 as libc::c_int as size_t;
                                                                                    if 1 as libc::c_int as libc::c_ulong
                                                                                        != read_n_bytes(
                                                                                            sock,
                                                                                            &mut len as *mut size_t as *mut libc::c_char,
                                                                                            1 as libc::c_int as size_t,
                                                                                        )
                                                                                    {
                                                                                        current_block = 12289599622484323274;
                                                                                    } else {
                                                                                        current_block = 13003737910779602957;
                                                                                    }
                                                                                }
                                                                            }
                                                                            match current_block {
                                                                                12289599622484323274 => {}
                                                                                _ => {
                                                                                    if !(len.wrapping_add(2 as libc::c_int as libc::c_ulong)
                                                                                        != read_n_bytes(
                                                                                            sock,
                                                                                            buff.as_mut_ptr() as *mut libc::c_char,
                                                                                            len.wrapping_add(2 as libc::c_int as libc::c_ulong),
                                                                                        ))
                                                                                    {
                                                                                        return SUCCESS as libc::c_int;
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                        4 => {
                                                                            current_block = 13060446176355317402;
                                                                            match current_block {
                                                                                13060446176355317402 => {
                                                                                    len = 16 as libc::c_int as size_t;
                                                                                    current_block = 13003737910779602957;
                                                                                }
                                                                                2824739507554377001 => {
                                                                                    len = 4 as libc::c_int as size_t;
                                                                                    current_block = 13003737910779602957;
                                                                                }
                                                                                _ => {
                                                                                    len = 0 as libc::c_int as size_t;
                                                                                    if 1 as libc::c_int as libc::c_ulong
                                                                                        != read_n_bytes(
                                                                                            sock,
                                                                                            &mut len as *mut size_t as *mut libc::c_char,
                                                                                            1 as libc::c_int as size_t,
                                                                                        )
                                                                                    {
                                                                                        current_block = 12289599622484323274;
                                                                                    } else {
                                                                                        current_block = 13003737910779602957;
                                                                                    }
                                                                                }
                                                                            }
                                                                            match current_block {
                                                                                12289599622484323274 => {}
                                                                                _ => {
                                                                                    if !(len.wrapping_add(2 as libc::c_int as libc::c_ulong)
                                                                                        != read_n_bytes(
                                                                                            sock,
                                                                                            buff.as_mut_ptr() as *mut libc::c_char,
                                                                                            len.wrapping_add(2 as libc::c_int as libc::c_ulong),
                                                                                        ))
                                                                                    {
                                                                                        return SUCCESS as libc::c_int;
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                        3 => {
                                                                            current_block = 5664113730932431836;
                                                                            match current_block {
                                                                                13060446176355317402 => {
                                                                                    len = 16 as libc::c_int as size_t;
                                                                                    current_block = 13003737910779602957;
                                                                                }
                                                                                2824739507554377001 => {
                                                                                    len = 4 as libc::c_int as size_t;
                                                                                    current_block = 13003737910779602957;
                                                                                }
                                                                                _ => {
                                                                                    len = 0 as libc::c_int as size_t;
                                                                                    if 1 as libc::c_int as libc::c_ulong
                                                                                        != read_n_bytes(
                                                                                            sock,
                                                                                            &mut len as *mut size_t as *mut libc::c_char,
                                                                                            1 as libc::c_int as size_t,
                                                                                        )
                                                                                    {
                                                                                        current_block = 12289599622484323274;
                                                                                    } else {
                                                                                        current_block = 13003737910779602957;
                                                                                    }
                                                                                }
                                                                            }
                                                                            match current_block {
                                                                                12289599622484323274 => {}
                                                                                _ => {
                                                                                    if !(len.wrapping_add(2 as libc::c_int as libc::c_ulong)
                                                                                        != read_n_bytes(
                                                                                            sock,
                                                                                            buff.as_mut_ptr() as *mut libc::c_char,
                                                                                            len.wrapping_add(2 as libc::c_int as libc::c_ulong),
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
                    }
                    3 => {
                        current_block = 12829669402821218572;
                        match current_block {
                            4650557577079045330 => return SUCCESS as libc::c_int,
                            6417057564578538666 => {
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
                                        .octet[0 as libc::c_int
                                        as usize] = 0 as libc::c_int as libc::c_uchar;
                                    ip
                                        .octet[1 as libc::c_int
                                        as usize] = 0 as libc::c_int as libc::c_uchar;
                                    ip
                                        .octet[2 as libc::c_int
                                        as usize] = 0 as libc::c_int as libc::c_uchar;
                                    ip
                                        .octet[3 as libc::c_int
                                        as usize] = 1 as libc::c_int as libc::c_uchar;
                                }
                                memcpy(
                                    &mut *buff.as_mut_ptr().offset(4 as libc::c_int as isize)
                                        as *mut libc::c_uchar as *mut libc::c_void,
                                    &mut ip as *mut ip_type as *const libc::c_void,
                                    4 as libc::c_int as libc::c_ulong,
                                );
                                len = ulen.wrapping_add(1 as libc::c_int as libc::c_ulong);
                                if len > 1 as libc::c_int as libc::c_ulong {
                                    memcpy(
                                        &mut *buff.as_mut_ptr().offset(8 as libc::c_int as isize)
                                            as *mut libc::c_uchar as *mut libc::c_void,
                                        user as *const libc::c_void,
                                        len,
                                    );
                                } else {
                                    buff[8 as libc::c_int
                                        as usize] = 0 as libc::c_int as libc::c_uchar;
                                }
                                if dns_len != 0 {
                                    memcpy(
                                        &mut *buff
                                            .as_mut_ptr()
                                            .offset(
                                                (8 as libc::c_int as libc::c_ulong).wrapping_add(len)
                                                    as isize,
                                            ) as *mut libc::c_uchar as *mut libc::c_void,
                                        dns_name as *const libc::c_void,
                                        dns_len.wrapping_add(1 as libc::c_int as libc::c_ulong),
                                    );
                                    len = (len as libc::c_ulong)
                                        .wrapping_add(
                                            dns_len.wrapping_add(1 as libc::c_int as libc::c_ulong),
                                        ) as size_t as size_t;
                                }
                                if !(len.wrapping_add(8 as libc::c_int as libc::c_ulong)
                                    != write_n_bytes(
                                        sock,
                                        buff.as_mut_ptr() as *mut libc::c_char,
                                        (8 as libc::c_int as libc::c_ulong).wrapping_add(len),
                                    ))
                                {
                                    if !(8 as libc::c_int as libc::c_ulong
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
                            13056961889198038528 => {
                                if dns_len == 0 {
                                    inet_ntop(
                                        2 as libc::c_int,
                                        &mut *(ip.octet)
                                            .as_mut_ptr()
                                            .offset(0 as libc::c_int as isize) as *mut libc::c_uchar
                                            as *const libc::c_void,
                                        ip_buf.as_mut_ptr(),
                                        ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong
                                            as socklen_t,
                                    );
                                    dns_name = ip_buf.as_mut_ptr();
                                }
                                snprintf(
                                    buff.as_mut_ptr() as *mut libc::c_char,
                                    ::std::mem::size_of::<[libc::c_uchar; 8192]>()
                                        as libc::c_ulong,
                                    b"CONNECT %s:%d HTTP/1.0\r\n\0" as *const u8
                                        as *const libc::c_char,
                                    dns_name,
                                    __bswap_16(port) as libc::c_int,
                                );
                                if *user.offset(0 as libc::c_int as isize) != 0 {
                                    let mut src: [libc::c_char; 512] = [0; 512];
                                    let mut dst: [libc::c_char; 2048] = [0; 2048];
                                    memcpy(
                                        src.as_mut_ptr() as *mut libc::c_void,
                                        user as *const libc::c_void,
                                        ulen,
                                    );
                                    memcpy(
                                        src.as_mut_ptr().offset(ulen as isize) as *mut libc::c_void,
                                        b":\0" as *const u8 as *const libc::c_char
                                            as *const libc::c_void,
                                        1 as libc::c_int as libc::c_ulong,
                                    );
                                    memcpy(
                                        src
                                            .as_mut_ptr()
                                            .offset(ulen as isize)
                                            .offset(1 as libc::c_int as isize) as *mut libc::c_void,
                                        pass as *const libc::c_void,
                                        passlen,
                                    );
                                    src[ulen
                                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                        .wrapping_add(passlen)
                                        as usize] = 0 as libc::c_int as libc::c_char;
                                    encode_base_64(
                                        src.as_mut_ptr(),
                                        dst.as_mut_ptr(),
                                        ::std::mem::size_of::<[libc::c_char; 2048]>()
                                            as libc::c_ulong as libc::c_int,
                                    );
                                    strcat(
                                        buff.as_mut_ptr() as *mut libc::c_char,
                                        b"Proxy-Authorization: Basic \0" as *const u8
                                            as *const libc::c_char,
                                    );
                                    strcat(
                                        buff.as_mut_ptr() as *mut libc::c_char,
                                        dst.as_mut_ptr(),
                                    );
                                    strcat(
                                        buff.as_mut_ptr() as *mut libc::c_char,
                                        b"\r\n\r\n\0" as *const u8 as *const libc::c_char,
                                    );
                                } else {
                                    strcat(
                                        buff.as_mut_ptr() as *mut libc::c_char,
                                        b"\r\n\0" as *const u8 as *const libc::c_char,
                                    );
                                }
                                len = strlen(buff.as_mut_ptr() as *mut libc::c_char);
                                if !(len
                                    != send(
                                        sock,
                                        buff.as_mut_ptr() as *const libc::c_void,
                                        len,
                                        0 as libc::c_int,
                                    ) as size_t)
                                {
                                    len = 0 as libc::c_int as size_t;
                                    loop {
                                        if !(len
                                            < (8 as libc::c_int * 1024 as libc::c_int) as libc::c_ulong)
                                        {
                                            current_block = 18153031941552419006;
                                            break;
                                        }
                                        if !(1 as libc::c_int as libc::c_ulong
                                            == read_n_bytes(
                                                sock,
                                                buff.as_mut_ptr().offset(len as isize) as *mut libc::c_char,
                                                1 as libc::c_int as size_t,
                                            ))
                                        {
                                            current_block = 12289599622484323274;
                                            break;
                                        }
                                        len = len.wrapping_add(1);
                                        len;
                                        if len > 4 as libc::c_int as libc::c_ulong
                                            && buff[len.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                as usize] as libc::c_int == '\n' as i32
                                            && buff[len.wrapping_sub(2 as libc::c_int as libc::c_ulong)
                                                as usize] as libc::c_int == '\r' as i32
                                            && buff[len.wrapping_sub(3 as libc::c_int as libc::c_ulong)
                                                as usize] as libc::c_int == '\n' as i32
                                            && buff[len.wrapping_sub(4 as libc::c_int as libc::c_ulong)
                                                as usize] as libc::c_int == '\r' as i32
                                        {
                                            current_block = 18153031941552419006;
                                            break;
                                        }
                                    }
                                    match current_block {
                                        12289599622484323274 => {}
                                        _ => {
                                            if len
                                                == (8 as libc::c_int * 1024 as libc::c_int) as libc::c_ulong
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
                            _ => {
                                if !user.is_null() {
                                    buff[0 as libc::c_int
                                        as usize] = 5 as libc::c_int as libc::c_uchar;
                                    buff[1 as libc::c_int
                                        as usize] = 2 as libc::c_int as libc::c_uchar;
                                    buff[2 as libc::c_int
                                        as usize] = 0 as libc::c_int as libc::c_uchar;
                                    buff[3 as libc::c_int
                                        as usize] = 2 as libc::c_int as libc::c_uchar;
                                    if 4 as libc::c_int as libc::c_ulong
                                        != write_n_bytes(
                                            sock,
                                            buff.as_mut_ptr() as *mut libc::c_char,
                                            4 as libc::c_int as size_t,
                                        )
                                    {
                                        current_block = 12289599622484323274;
                                    } else {
                                        current_block = 3580086814630675314;
                                    }
                                } else {
                                    buff[0 as libc::c_int
                                        as usize] = 5 as libc::c_int as libc::c_uchar;
                                    buff[1 as libc::c_int
                                        as usize] = 1 as libc::c_int as libc::c_uchar;
                                    buff[2 as libc::c_int
                                        as usize] = 0 as libc::c_int as libc::c_uchar;
                                    if 3 as libc::c_int as libc::c_ulong
                                        != write_n_bytes(
                                            sock,
                                            buff.as_mut_ptr() as *mut libc::c_char,
                                            3 as libc::c_int as size_t,
                                        )
                                    {
                                        current_block = 12289599622484323274;
                                    } else {
                                        current_block = 3580086814630675314;
                                    }
                                }
                                match current_block {
                                    12289599622484323274 => {}
                                    _ => {
                                        if !(2 as libc::c_int as libc::c_ulong
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
                                                    let mut c: libc::c_int = 0;
                                                    let fresh14 = cur;
                                                    cur = cur.offset(1);
                                                    *fresh14 = 1 as libc::c_int as libc::c_char;
                                                    c = (ulen & 0xff as libc::c_int as libc::c_ulong)
                                                        as libc::c_int;
                                                    let fresh15 = cur;
                                                    cur = cur.offset(1);
                                                    *fresh15 = c as libc::c_char;
                                                    memcpy(
                                                        cur as *mut libc::c_void,
                                                        user as *const libc::c_void,
                                                        c as size_t,
                                                    );
                                                    cur = cur.offset(c as isize);
                                                    c = (passlen & 0xff as libc::c_int as libc::c_ulong)
                                                        as libc::c_int;
                                                    let fresh16 = cur;
                                                    cur = cur.offset(1);
                                                    *fresh16 = c as libc::c_char;
                                                    memcpy(
                                                        cur as *mut libc::c_void,
                                                        pass as *const libc::c_void,
                                                        c as size_t,
                                                    );
                                                    cur = cur.offset(c as isize);
                                                    if cur.offset_from(out.as_mut_ptr()) as libc::c_long
                                                        as size_t
                                                        != write_n_bytes(
                                                            sock,
                                                            out.as_mut_ptr(),
                                                            cur.offset_from(out.as_mut_ptr()) as libc::c_long as size_t,
                                                        )
                                                    {
                                                        current_block = 12289599622484323274;
                                                    } else if 2 as libc::c_int as libc::c_ulong
                                                        != read_n_bytes(
                                                            sock,
                                                            in_0.as_mut_ptr(),
                                                            2 as libc::c_int as size_t,
                                                        )
                                                    {
                                                        current_block = 12289599622484323274;
                                                    } else if in_0[0 as libc::c_int as usize] as libc::c_int
                                                        != 1 as libc::c_int
                                                        || in_0[1 as libc::c_int as usize] as libc::c_int
                                                            != 0 as libc::c_int
                                                    {
                                                        if in_0[0 as libc::c_int as usize] as libc::c_int
                                                            != 1 as libc::c_int
                                                        {
                                                            current_block = 12289599622484323274;
                                                        } else {
                                                            return BLOCKED as libc::c_int
                                                        }
                                                    } else {
                                                        current_block = 2472048668343472511;
                                                    }
                                                } else {
                                                    current_block = 2472048668343472511;
                                                }
                                                match current_block {
                                                    12289599622484323274 => {}
                                                    _ => {
                                                        let mut buff_iter: size_t = 0 as libc::c_int as size_t;
                                                        let fresh17 = buff_iter;
                                                        buff_iter = buff_iter.wrapping_add(1);
                                                        buff[fresh17 as usize] = 5 as libc::c_int as libc::c_uchar;
                                                        let fresh18 = buff_iter;
                                                        buff_iter = buff_iter.wrapping_add(1);
                                                        buff[fresh18 as usize] = 1 as libc::c_int as libc::c_uchar;
                                                        let fresh19 = buff_iter;
                                                        buff_iter = buff_iter.wrapping_add(1);
                                                        buff[fresh19 as usize] = 0 as libc::c_int as libc::c_uchar;
                                                        if dns_len == 0 {
                                                            let fresh20 = buff_iter;
                                                            buff_iter = buff_iter.wrapping_add(1);
                                                            buff[fresh20 as usize] = 1 as libc::c_int as libc::c_uchar;
                                                            memcpy(
                                                                buff.as_mut_ptr().offset(buff_iter as isize)
                                                                    as *mut libc::c_void,
                                                                &mut ip as *mut ip_type as *const libc::c_void,
                                                                4 as libc::c_int as libc::c_ulong,
                                                            );
                                                            buff_iter = (buff_iter as libc::c_ulong)
                                                                .wrapping_add(4 as libc::c_int as libc::c_ulong) as size_t
                                                                as size_t;
                                                        } else {
                                                            let fresh21 = buff_iter;
                                                            buff_iter = buff_iter.wrapping_add(1);
                                                            buff[fresh21 as usize] = 3 as libc::c_int as libc::c_uchar;
                                                            let fresh22 = buff_iter;
                                                            buff_iter = buff_iter.wrapping_add(1);
                                                            buff[fresh22
                                                                as usize] = (dns_len & 0xff as libc::c_int as libc::c_ulong)
                                                                as libc::c_uchar;
                                                            memcpy(
                                                                buff.as_mut_ptr().offset(buff_iter as isize)
                                                                    as *mut libc::c_void,
                                                                dns_name as *const libc::c_void,
                                                                dns_len,
                                                            );
                                                            buff_iter = (buff_iter as libc::c_ulong)
                                                                .wrapping_add(dns_len) as size_t as size_t;
                                                        }
                                                        memcpy(
                                                            buff.as_mut_ptr().offset(buff_iter as isize)
                                                                as *mut libc::c_void,
                                                            &mut port as *mut libc::c_ushort as *const libc::c_void,
                                                            2 as libc::c_int as libc::c_ulong,
                                                        );
                                                        buff_iter = (buff_iter as libc::c_ulong)
                                                            .wrapping_add(2 as libc::c_int as libc::c_ulong) as size_t
                                                            as size_t;
                                                        if !(buff_iter
                                                            != write_n_bytes(
                                                                sock,
                                                                buff.as_mut_ptr() as *mut libc::c_char,
                                                                buff_iter,
                                                            ))
                                                        {
                                                            if !(4 as libc::c_int as libc::c_ulong
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
                                                                            current_block = 2824739507554377001;
                                                                            match current_block {
                                                                                13060446176355317402 => {
                                                                                    len = 16 as libc::c_int as size_t;
                                                                                    current_block = 13003737910779602957;
                                                                                }
                                                                                2824739507554377001 => {
                                                                                    len = 4 as libc::c_int as size_t;
                                                                                    current_block = 13003737910779602957;
                                                                                }
                                                                                _ => {
                                                                                    len = 0 as libc::c_int as size_t;
                                                                                    if 1 as libc::c_int as libc::c_ulong
                                                                                        != read_n_bytes(
                                                                                            sock,
                                                                                            &mut len as *mut size_t as *mut libc::c_char,
                                                                                            1 as libc::c_int as size_t,
                                                                                        )
                                                                                    {
                                                                                        current_block = 12289599622484323274;
                                                                                    } else {
                                                                                        current_block = 13003737910779602957;
                                                                                    }
                                                                                }
                                                                            }
                                                                            match current_block {
                                                                                12289599622484323274 => {}
                                                                                _ => {
                                                                                    if !(len.wrapping_add(2 as libc::c_int as libc::c_ulong)
                                                                                        != read_n_bytes(
                                                                                            sock,
                                                                                            buff.as_mut_ptr() as *mut libc::c_char,
                                                                                            len.wrapping_add(2 as libc::c_int as libc::c_ulong),
                                                                                        ))
                                                                                    {
                                                                                        return SUCCESS as libc::c_int;
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                        4 => {
                                                                            current_block = 13060446176355317402;
                                                                            match current_block {
                                                                                13060446176355317402 => {
                                                                                    len = 16 as libc::c_int as size_t;
                                                                                    current_block = 13003737910779602957;
                                                                                }
                                                                                2824739507554377001 => {
                                                                                    len = 4 as libc::c_int as size_t;
                                                                                    current_block = 13003737910779602957;
                                                                                }
                                                                                _ => {
                                                                                    len = 0 as libc::c_int as size_t;
                                                                                    if 1 as libc::c_int as libc::c_ulong
                                                                                        != read_n_bytes(
                                                                                            sock,
                                                                                            &mut len as *mut size_t as *mut libc::c_char,
                                                                                            1 as libc::c_int as size_t,
                                                                                        )
                                                                                    {
                                                                                        current_block = 12289599622484323274;
                                                                                    } else {
                                                                                        current_block = 13003737910779602957;
                                                                                    }
                                                                                }
                                                                            }
                                                                            match current_block {
                                                                                12289599622484323274 => {}
                                                                                _ => {
                                                                                    if !(len.wrapping_add(2 as libc::c_int as libc::c_ulong)
                                                                                        != read_n_bytes(
                                                                                            sock,
                                                                                            buff.as_mut_ptr() as *mut libc::c_char,
                                                                                            len.wrapping_add(2 as libc::c_int as libc::c_ulong),
                                                                                        ))
                                                                                    {
                                                                                        return SUCCESS as libc::c_int;
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                        3 => {
                                                                            current_block = 5664113730932431836;
                                                                            match current_block {
                                                                                13060446176355317402 => {
                                                                                    len = 16 as libc::c_int as size_t;
                                                                                    current_block = 13003737910779602957;
                                                                                }
                                                                                2824739507554377001 => {
                                                                                    len = 4 as libc::c_int as size_t;
                                                                                    current_block = 13003737910779602957;
                                                                                }
                                                                                _ => {
                                                                                    len = 0 as libc::c_int as size_t;
                                                                                    if 1 as libc::c_int as libc::c_ulong
                                                                                        != read_n_bytes(
                                                                                            sock,
                                                                                            &mut len as *mut size_t as *mut libc::c_char,
                                                                                            1 as libc::c_int as size_t,
                                                                                        )
                                                                                    {
                                                                                        current_block = 12289599622484323274;
                                                                                    } else {
                                                                                        current_block = 13003737910779602957;
                                                                                    }
                                                                                }
                                                                            }
                                                                            match current_block {
                                                                                12289599622484323274 => {}
                                                                                _ => {
                                                                                    if !(len.wrapping_add(2 as libc::c_int as libc::c_ulong)
                                                                                        != read_n_bytes(
                                                                                            sock,
                                                                                            buff.as_mut_ptr() as *mut libc::c_char,
                                                                                            len.wrapping_add(2 as libc::c_int as libc::c_ulong),
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
    let mut addr: sockaddr_in = sockaddr_in {
        sin_family: 0,
        sin_port: 0,
        sin_addr: in_addr { s_addr: 0 },
        sin_zero: [0; 8],
    };
    let mut ip_buf: [libc::c_char; 16] = [0; 16];
    *fd = socket(2 as libc::c_int, SOCK_STREAM as libc::c_int, 0 as libc::c_int);
    if !(*fd == -(1 as libc::c_int)) {
        inet_ntop(
            2 as libc::c_int,
            &mut *((*pd).ip.octet).as_mut_ptr().offset(0 as libc::c_int as isize)
                as *mut libc::c_uchar as *const libc::c_void,
            ip_buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong as socklen_t,
        );
        proxychains_write_log(
            b"[proxychains] %s  ...  %s:%d \0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            begin_mark,
            ip_buf.as_mut_ptr(),
            __bswap_16((*pd).port) as libc::c_int,
        );
        (*pd).ps = PLAY_STATE;
        memset(
            &mut addr as *mut sockaddr_in as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong,
        );
        addr.sin_family = 2 as libc::c_int as sa_family_t;
        addr.sin_addr.s_addr = (*pd).ip.as_int;
        addr.sin_port = (*pd).port;
        if timed_connect(
            *fd,
            &mut addr as *mut sockaddr_in as *mut sockaddr,
            ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as socklen_t,
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
    if *fd != -(1 as libc::c_int) {
        close(*fd);
    }
    return SOCKET_ERROR as libc::c_int;
}
pub unsafe extern "C" fn get_rand_int(mut range: libc::c_uint) -> libc::c_uint {
    static mut fp: *mut FILE = 0 as *const FILE as *mut FILE;
    let mut randval: libc::c_uint = 0;
    if fp.is_null() {
        fp = fopen(
            b"/dev/urandom\0" as *const u8 as *const libc::c_char,
            b"r\0" as *const u8 as *const libc::c_char,
        );
    }
    if fread(
        &mut randval as *mut libc::c_uint as *mut libc::c_void,
        ::std::mem::size_of::<libc::c_uint>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        fp,
    ) != 0
    {
        return randval.wrapping_rem(range)
    } else {
        srand(time(0 as *mut time_t) as libc::c_uint);
        return (rand() as libc::c_uint).wrapping_rem(range);
    };
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
                i = (0 as libc::c_int as libc::c_uint)
                    .wrapping_add(get_rand_int(proxy_count));
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
    let mut alive_count: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    release_busy(pd, proxy_count);
    i = 0 as libc::c_int as libc::c_uint;
    while i < proxy_count {
        if (*pd.offset(i as isize)).ps as libc::c_uint
            == PLAY_STATE as libc::c_int as libc::c_uint
        {
            alive_count = alive_count.wrapping_add(1);
            alive_count;
        }
        i = i.wrapping_add(1);
        i;
    }
    return alive_count;
}
unsafe extern "C" fn chain_step(
    mut ns: libc::c_int,
    mut pfrom: *mut proxy_data,
    mut pto: *mut proxy_data,
) -> libc::c_int {
    let mut retcode: libc::c_int = -(1 as libc::c_int);
    let mut hostname_0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ip_buf: [libc::c_char; 16] = [0; 16];
    let mut current_block_4: u64;
    if (*pto).ip.octet[0 as libc::c_int as usize] as libc::c_uint == remote_dns_subnet {
        hostname_0 = string_from_internal_ip((*pto).ip);
        if hostname_0.is_null() {
            current_block_4 = 10317748377739400690;
        } else {
            current_block_4 = 14523784380283086299;
        }
    } else {
        current_block_4 = 10317748377739400690;
    }
    match current_block_4 {
        10317748377739400690 => {
            inet_ntop(
                2 as libc::c_int,
                &mut *((*pto).ip.octet).as_mut_ptr().offset(0 as libc::c_int as isize)
                    as *mut libc::c_uchar as *const libc::c_void,
                ip_buf.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong as socklen_t,
            );
            hostname_0 = ip_buf.as_mut_ptr();
        }
        _ => {}
    }
    proxychains_write_log(
        b" ...  %s:%d \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        hostname_0,
        __bswap_16((*pto).port) as libc::c_int,
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
        ip: ip_type { octet: [0; 4] },
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
    let mut offset: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut alive_count: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut curr_len: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    p3 = &mut p4;
    '_again: loop {
        match ct as libc::c_uint {
            0 => {
                calc_alive(pd, proxy_count);
                offset = 0 as libc::c_int as libc::c_uint;
                loop {
                    p1 = select_proxy(FIFOLY, pd, proxy_count, &mut offset);
                    if p1.is_null() {
                        current_block = 8039577134148111679;
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
                    current_block = 3603246499773290836;
                    break;
                } else {
                    current_block = 5892776923941496671;
                    break;
                }
            }
            1 => {
                calc_alive(pd, proxy_count);
                offset = 0 as libc::c_int as libc::c_uint;
                p1 = select_proxy(FIFOLY, pd, proxy_count, &mut offset);
                if p1.is_null() {
                    current_block = 2706659501864706830;
                    break;
                } else {
                    current_block = 2719512138335094285;
                    break;
                }
            }
            2 => {
                alive_count = calc_alive(pd, proxy_count);
                if alive_count < max_chain {
                    current_block = 8039577134148111679;
                    break;
                }
                offset = 0 as libc::c_int as libc::c_uint;
                curr_len = offset;
                loop {
                    p1 = select_proxy(RANDOMLY, pd, proxy_count, &mut offset);
                    if p1.is_null() {
                        current_block = 8039577134148111679;
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
                        current_block = 8039577134148111679;
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
                    current_block = 3603246499773290836;
                    break;
                } else {
                    current_block = 5892776923941496671;
                    break;
                }
            }
            _ => {
                current_block = 5892776923941496671;
                break;
            }
        }
    }
    match current_block {
        2719512138335094285 => {
            if SUCCESS as libc::c_int
                != start_chain(
                    &mut ns,
                    p1,
                    b"Strict chain\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                )
            {
                current_block = 2706659501864706830;
            } else {
                loop {
                    if !(offset < proxy_count) {
                        current_block = 9520865839495247062;
                        break;
                    }
                    p2 = select_proxy(FIFOLY, pd, proxy_count, &mut offset);
                    if p2.is_null() {
                        current_block = 9520865839495247062;
                        break;
                    }
                    if SUCCESS as libc::c_int != chain_step(ns, p1, p2) {
                        current_block = 2706659501864706830;
                        break;
                    }
                    p1 = p2;
                }
                match current_block {
                    2706659501864706830 => {}
                    _ => {
                        (*p3).ip = target_ip;
                        (*p3).port = target_port;
                        if SUCCESS as libc::c_int != chain_step(ns, p1, p3) {
                            current_block = 3603246499773290836;
                        } else {
                            current_block = 5892776923941496671;
                        }
                    }
                }
            }
        }
        8039577134148111679 => {
            proxychains_write_log(
                b"\n!!!need more proxies!!!\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            current_block = 2706659501864706830;
        }
        _ => {}
    }
    match current_block {
        2706659501864706830 => {
            release_all(pd, proxy_count);
            if ns != -(1 as libc::c_int) {
                close(ns);
            }
            *__errno_location() = 110 as libc::c_int;
            return -(1 as libc::c_int);
        }
        3603246499773290836 => {
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
static mut local_host: ip_type = ip_type {
    octet: [
        127 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        1 as libc::c_int as libc::c_uchar,
    ],
};
pub static mut hostname: [libc::c_char; 256] = [0; 256];
pub unsafe extern "C" fn proxy_gethostbyname(
    mut name: *const libc::c_char,
    mut data: *mut gethostbyname_data,
) -> *mut hostent {
    let mut current_block: u64;
    let mut buff: [libc::c_char; 256] = [0; 256];
    let mut i: uint32_t = 0;
    let mut hash: uint32_t = 0;
    let mut new_mem: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut l: size_t = 0;
    let mut hp: *mut hostent = 0 as *mut hostent;
    (*data)
        .resolved_addr_p[0 as libc::c_int
        as usize] = &mut (*data).resolved_addr as *mut in_addr_t as *mut libc::c_char;
    (*data).resolved_addr_p[1 as libc::c_int as usize] = 0 as *mut libc::c_char;
    (*data).hostent_space.h_addr_list = ((*data).resolved_addr_p).as_mut_ptr();
    (*data).resolved_addr = 0 as libc::c_int as in_addr_t;
    gethostname(
        buff.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
    );
    if strcmp(buff.as_mut_ptr(), name) == 0 {
        (*data).resolved_addr = inet_addr(buff.as_mut_ptr());
        if (*data).resolved_addr == -(1 as libc::c_int) as in_addr_t {
            (*data).resolved_addr = local_host.as_int;
        }
        snprintf(
            hostname.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
            b"%s\0" as *const u8 as *const libc::c_char,
            name,
        );
        (*data).hostent_space.h_name = hostname.as_mut_ptr();
        (*data)
            .hostent_space
            .h_length = ::std::mem::size_of::<in_addr_t>() as libc::c_ulong
            as libc::c_int;
        (*data).hostent_space.h_addrtype = 2 as libc::c_int;
        return &mut (*data).hostent_space;
    }
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
    hash = dalias_hash(name as *mut libc::c_char);
    pthread_mutex_lock(&mut internal_ips_lock);
    if internal_ips.counter != 0 {
        i = 0 as libc::c_int as uint32_t;
        loop {
            if !(i < internal_ips.counter) {
                current_block = 10652014663920648156;
                break;
            }
            if (**(internal_ips.list).offset(i as isize)).hash == hash
                && strcmp(name, (**(internal_ips.list).offset(i as isize)).string) == 0
            {
                (*data).resolved_addr = make_internal_ip(i);
                current_block = 16062556756544833697;
                break;
            } else {
                i = i.wrapping_add(1);
                i;
            }
        }
    } else {
        current_block = 10652014663920648156;
    }
    match current_block {
        10652014663920648156 => {
            if internal_ips.capa
                < (internal_ips.counter).wrapping_add(1 as libc::c_int as libc::c_uint)
            {
                new_mem = realloc(
                    internal_ips.list as *mut libc::c_void,
                    ((internal_ips.capa).wrapping_add(16 as libc::c_int as libc::c_uint)
                        as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
                        ),
                );
                if !new_mem.is_null() {
                    internal_ips
                        .capa = (internal_ips.capa as libc::c_uint)
                        .wrapping_add(16 as libc::c_int as libc::c_uint) as uint32_t
                        as uint32_t;
                    internal_ips.list = new_mem as *mut *mut string_hash_tuple;
                    current_block = 3275366147856559585;
                } else {
                    current_block = 14112873171500882133;
                }
            } else {
                current_block = 3275366147856559585;
            }
            match current_block {
                3275366147856559585 => {
                    (*data).resolved_addr = make_internal_ip(internal_ips.counter);
                    if (*data).resolved_addr == -(1 as libc::c_int) as in_addr_t {
                        current_block = 4328957238102952938;
                    } else {
                        l = strlen(name);
                        new_mem = malloc(
                            (::std::mem::size_of::<string_hash_tuple>() as libc::c_ulong)
                                .wrapping_add(l)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong),
                        );
                        if new_mem.is_null() {
                            current_block = 14112873171500882133;
                        } else {
                            let ref mut fresh23 = *(internal_ips.list)
                                .offset(internal_ips.counter as isize);
                            *fresh23 = new_mem as *mut string_hash_tuple;
                            (**(internal_ips.list).offset(internal_ips.counter as isize))
                                .hash = hash;
                            let ref mut fresh24 = (**(internal_ips.list)
                                .offset(internal_ips.counter as isize))
                                .string;
                            *fresh24 = (new_mem as *mut libc::c_char)
                                .offset(
                                    ::std::mem::size_of::<string_hash_tuple>() as libc::c_ulong
                                        as isize,
                                );
                            memcpy(
                                (**(internal_ips.list)
                                    .offset(internal_ips.counter as isize))
                                    .string as *mut libc::c_void,
                                name as *const libc::c_void,
                                l.wrapping_add(1 as libc::c_int as libc::c_ulong),
                            );
                            internal_ips
                                .counter = (internal_ips.counter as libc::c_uint)
                                .wrapping_add(1 as libc::c_int as libc::c_uint) as uint32_t
                                as uint32_t;
                            current_block = 16062556756544833697;
                        }
                    }
                }
                _ => {}
            }
            match current_block {
                16062556756544833697 => {}
                _ => {
                    match current_block {
                        14112873171500882133 => {
                            proxychains_write_log(
                                b"out of mem\n\0" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char,
                            );
                        }
                        _ => {}
                    }
                    pthread_mutex_unlock(&mut internal_ips_lock);
                    return 0 as *mut hostent;
                }
            }
        }
        _ => {}
    }
    pthread_mutex_unlock(&mut internal_ips_lock);
    strncpy(
        ((*data).addr_name).as_mut_ptr(),
        name,
        (::std::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    );
    (*data).hostent_space.h_name = ((*data).addr_name).as_mut_ptr();
    (*data)
        .hostent_space
        .h_length = ::std::mem::size_of::<in_addr_t>() as libc::c_ulong as libc::c_int;
    (*data).hostent_space.h_addrtype = 2 as libc::c_int;
    return &mut (*data).hostent_space;
}
pub unsafe extern "C" fn proxy_freeaddrinfo(mut res: *mut addrinfo) {
    free(res as *mut libc::c_void);
}
pub unsafe extern "C" fn proxy_getservbyname(
    mut service: *const libc::c_char,
    mut se_buf: *mut servent,
    mut buf: *mut libc::c_char,
    mut buf_len: size_t,
    mut se_result: *mut *mut servent,
) {
    getservbyname_r(service, 0 as *const libc::c_char, se_buf, buf, buf_len, se_result);
}
pub unsafe extern "C" fn proxy_getaddrinfo(
    mut node: *const libc::c_char,
    mut service: *const libc::c_char,
    mut hints: *const addrinfo,
    mut res: *mut *mut addrinfo,
) -> libc::c_int {
    let mut ghdata: gethostbyname_data = gethostbyname_data {
        hostent_space: hostent {
            h_name: 0 as *mut libc::c_char,
            h_aliases: 0 as *mut *mut libc::c_char,
            h_addrtype: 0,
            h_length: 0,
            h_addr_list: 0 as *mut *mut libc::c_char,
        },
        resolved_addr: 0,
        resolved_addr_p: [0 as *mut libc::c_char; 2],
        addr_name: [0; 8192],
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
    space = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<addrinfo_data>() as libc::c_ulong,
    ) as *mut addrinfo_data;
    if space.is_null() {
        return 1 as libc::c_int;
    }
    if !node.is_null()
        && inet_aton(
            node,
            &mut (*(&mut (*space).sockaddr_space as *mut sockaddr as *mut sockaddr_in))
                .sin_addr,
        ) == 0
    {
        hp = proxy_gethostbyname(node, &mut ghdata);
        if !hp.is_null() {
            memcpy(
                &mut (*(&mut (*space).sockaddr_space as *mut sockaddr
                    as *mut sockaddr_in))
                    .sin_addr as *mut in_addr as *mut libc::c_void,
                *(*hp).h_addr_list as *const libc::c_void,
                ::std::mem::size_of::<in_addr_t>() as libc::c_ulong,
            );
        } else {
            free(space as *mut libc::c_void);
            return 1 as libc::c_int;
        }
    }
    if !service.is_null() {
        proxy_getservbyname(
            service,
            &mut se_buf,
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
            &mut se,
        );
    }
    port = if !se.is_null() {
        (*se).s_port
    } else {
        __bswap_16(
            atoi(
                if !service.is_null() {
                    service
                } else {
                    b"0\0" as *const u8 as *const libc::c_char
                },
            ) as uint16_t,
        ) as libc::c_int
    };
    (*(&mut (*space).sockaddr_space as *mut sockaddr as *mut sockaddr_in))
        .sin_port = port as in_port_t;
    p = &mut (*space).addrinfo_space;
    *res = p;
    if p as size_t == space as size_t {} else {
        __assert_fail(
            b"(size_t)p == (size_t) space\0" as *const u8 as *const libc::c_char,
            b"src/core.c\0" as *const u8 as *const libc::c_char,
            913 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 95],
                &[libc::c_char; 95],
            >(
                b"int proxy_getaddrinfo(const char *, const char *, const struct addrinfo *, struct addrinfo **)\0",
            ))
                .as_ptr(),
        );
    };
    (*p).ai_addr = &mut (*space).sockaddr_space;
    if !node.is_null() {
        strncpy(
            ((*space).addr_name).as_mut_ptr(),
            node,
            (::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
    }
    (*p).ai_canonname = ((*space).addr_name).as_mut_ptr();
    (*p).ai_next = 0 as *mut addrinfo;
    (*space).sockaddr_space.sa_family = 2 as libc::c_int as sa_family_t;
    (*p).ai_family = (*space).sockaddr_space.sa_family as libc::c_int;
    (*p).ai_addrlen = ::std::mem::size_of::<sockaddr>() as libc::c_ulong as socklen_t;
    if !hints.is_null() {
        (*p).ai_socktype = (*hints).ai_socktype;
        (*p).ai_flags = (*hints).ai_flags;
        (*p).ai_protocol = (*hints).ai_protocol;
    } else {
        (*p).ai_flags = 0x20 as libc::c_int;
    }
    return 0 as libc::c_int;
}
