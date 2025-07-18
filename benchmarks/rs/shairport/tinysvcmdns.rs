use ::libc;
use ::c2rust_asm_casts;
use c2rust_asm_casts::AsmCastTrait;
use std::arch::asm;
extern "C" {
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memchr(
        _: *const libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn select(
        __nfds: libc::c_int,
        __readfds: *mut fd_set,
        __writefds: *mut fd_set,
        __exceptfds: *mut fd_set,
        __timeout: *mut timeval,
    ) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn socket(
        __domain: libc::c_int,
        __type: libc::c_int,
        __protocol: libc::c_int,
    ) -> libc::c_int;
    fn bind(__fd: libc::c_int, __addr: *const sockaddr, __len: socklen_t) -> libc::c_int;
    fn sendto(
        __fd: libc::c_int,
        __buf: *const libc::c_void,
        __n: size_t,
        __flags: libc::c_int,
        __addr: *const sockaddr,
        __addr_len: socklen_t,
    ) -> ssize_t;
    fn recvfrom(
        __fd: libc::c_int,
        __buf: *mut libc::c_void,
        __n: size_t,
        __flags: libc::c_int,
        __addr: *mut sockaddr,
        __addr_len: *mut socklen_t,
    ) -> ssize_t;
    fn setsockopt(
        __fd: libc::c_int,
        __level: libc::c_int,
        __optname: libc::c_int,
        __optval: *const libc::c_void,
        __optlen: socklen_t,
    ) -> libc::c_int;
    fn inet_addr(__cp: *const libc::c_char) -> in_addr_t;
    fn inet_ntoa(__in: in_addr) -> *mut libc::c_char;
    fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option::<
            unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
        >,
        __arg: *mut libc::c_void,
    ) -> libc::c_int;
    fn pthread_attr_init(__attr: *mut pthread_attr_t) -> libc::c_int;
    fn pthread_attr_setdetachstate(
        __attr: *mut pthread_attr_t,
        __detachstate: libc::c_int,
    ) -> libc::c_int;
    fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> libc::c_int;
    fn pthread_mutex_destroy(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn debug(level: libc::c_int, format: *mut libc::c_char, _: ...);
    fn nanosleep(
        __requested_time: *const timespec,
        __remaining: *mut timespec,
    ) -> libc::c_int;
    fn warn(format: *mut libc::c_char, _: ...);
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn pipe(__pipedes: *mut libc::c_int) -> libc::c_int;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type size_t = libc::c_ulong;
pub type ssize_t = __ssize_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
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
pub union pthread_mutexattr_t {
    pub __size: [libc::c_char; 4],
    pub __align: libc::c_int,
}
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
pub struct ip_mreq {
    pub imr_multiaddr: in_addr,
    pub imr_interface: in_addr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rr_data_srv {
    pub priority: uint16_t,
    pub weight: uint16_t,
    pub port: uint16_t,
    pub target: *mut uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rr_data_txt {
    pub next: *mut rr_data_txt,
    pub txt: *mut uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rr_data_nsec {
    pub bitmap: [uint8_t; 5],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rr_data_ptr {
    pub name: *mut uint8_t,
    pub entry: *mut rr_entry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rr_entry {
    pub name: *mut uint8_t,
    pub type_0: rr_type,
    pub ttl: uint32_t,
    pub unicast_query: libc::c_char,
    pub cache_flush: libc::c_char,
    pub rr_class: uint16_t,
    pub data: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub NSEC: rr_data_nsec,
    pub SRV: rr_data_srv,
    pub TXT: rr_data_txt,
    pub PTR: rr_data_ptr,
    pub A: rr_data_a,
    pub AAAA: rr_data_aaaa,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rr_data_aaaa {
    pub addr: *mut in6_addr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rr_data_a {
    pub addr: uint32_t,
}
pub type rr_type = libc::c_uint;
pub const RR_ANY: rr_type = 255;
pub const RR_NSEC: rr_type = 47;
pub const RR_SRV: rr_type = 33;
pub const RR_AAAA: rr_type = 28;
pub const RR_TXT: rr_type = 16;
pub const RR_PTR: rr_type = 12;
pub const RR_A: rr_type = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rr_list {
    pub e: *mut rr_entry,
    pub next: *mut rr_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rr_group {
    pub name: *mut uint8_t,
    pub rr: *mut rr_list,
    pub next: *mut rr_group,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mdns_pkt {
    pub id: uint16_t,
    pub flags: uint16_t,
    pub num_qn: uint16_t,
    pub num_ans_rr: uint16_t,
    pub num_auth_rr: uint16_t,
    pub num_add_rr: uint16_t,
    pub rr_qn: *mut rr_list,
    pub rr_ans: *mut rr_list,
    pub rr_auth: *mut rr_list,
    pub rr_add: *mut rr_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct name_comp {
    pub label: *mut uint8_t,
    pub pos: size_t,
    pub next: *mut name_comp,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mdnsd {
    pub data_lock: pthread_mutex_t,
    pub sockfd: libc::c_int,
    pub notify_pipe: [libc::c_int; 2],
    pub stop_flag: libc::c_int,
    pub group: *mut rr_group,
    pub announce: *mut rr_list,
    pub services: *mut rr_list,
    pub hostname: *mut uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mdns_service {
    pub entries: *mut rr_list,
}
pub const PTHREAD_CREATE_DETACHED: C2RustUnnamed_2 = 1;
pub type C2RustUnnamed_2 = libc::c_uint;
pub const PTHREAD_CREATE_JOINABLE: C2RustUnnamed_2 = 0;
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
unsafe extern "C" fn cmp_nlabel(
    mut L1: *const uint8_t,
    mut L2: *const uint8_t,
) -> libc::c_int {
    return strcmp(L1 as *mut libc::c_char, L2 as *mut libc::c_char);
}
#[inline]
unsafe extern "C" fn dup_nlabel(mut n: *const uint8_t) -> *mut uint8_t {
    if *n.offset(0 as libc::c_int as isize) as libc::c_int <= 63 as libc::c_int {} else {
        __assert_fail(
            b"n[0] <= 63\0" as *const u8 as *const libc::c_char,
            b"tinysvcmdns.c\0" as *const u8 as *const libc::c_char,
            81 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 37],
                &[libc::c_char; 37],
            >(b"uint8_t *dup_nlabel(const uint8_t *)\0"))
                .as_ptr(),
        );
    }
    'c_6152: {
        if *n.offset(0 as libc::c_int as isize) as libc::c_int <= 63 as libc::c_int
        {} else {
            __assert_fail(
                b"n[0] <= 63\0" as *const u8 as *const libc::c_char,
                b"tinysvcmdns.c\0" as *const u8 as *const libc::c_char,
                81 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 37],
                    &[libc::c_char; 37],
                >(b"uint8_t *dup_nlabel(const uint8_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    return strdup(n as *mut libc::c_char) as *mut uint8_t;
}
pub unsafe extern "C" fn dup_label(mut label: *const uint8_t) -> *mut uint8_t {
    let mut len: libc::c_int = *label as libc::c_int + 1 as libc::c_int;
    if len > 63 as libc::c_int {
        return 0 as *mut uint8_t;
    }
    let mut newlabel: *mut uint8_t = malloc((len + 1 as libc::c_int) as libc::c_ulong)
        as *mut uint8_t;
    strncpy(
        newlabel as *mut libc::c_char,
        label as *mut libc::c_char,
        len as libc::c_ulong,
    );
    *newlabel.offset(len as isize) = '\0' as i32 as uint8_t;
    return newlabel;
}
pub unsafe extern "C" fn join_nlabel(
    mut n1: *const uint8_t,
    mut n2: *const uint8_t,
) -> *mut uint8_t {
    let mut len1: libc::c_int = 0;
    let mut len2: libc::c_int = 0;
    let mut s: *mut uint8_t = 0 as *mut uint8_t;
    if *n1.offset(0 as libc::c_int as isize) as libc::c_int <= 63 as libc::c_int
        && *n2.offset(0 as libc::c_int as isize) as libc::c_int <= 63 as libc::c_int
    {} else {
        __assert_fail(
            b"n1[0] <= 63 && n2[0] <= 63\0" as *const u8 as *const libc::c_char,
            b"tinysvcmdns.c\0" as *const u8 as *const libc::c_char,
            100 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 55],
                &[libc::c_char; 55],
            >(b"uint8_t *join_nlabel(const uint8_t *, const uint8_t *)\0"))
                .as_ptr(),
        );
    }
    'c_7541: {
        if *n1.offset(0 as libc::c_int as isize) as libc::c_int <= 63 as libc::c_int
            && *n2.offset(0 as libc::c_int as isize) as libc::c_int <= 63 as libc::c_int
        {} else {
            __assert_fail(
                b"n1[0] <= 63 && n2[0] <= 63\0" as *const u8 as *const libc::c_char,
                b"tinysvcmdns.c\0" as *const u8 as *const libc::c_char,
                100 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 55],
                    &[libc::c_char; 55],
                >(b"uint8_t *join_nlabel(const uint8_t *, const uint8_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    len1 = strlen(n1 as *mut libc::c_char) as libc::c_int;
    len2 = strlen(n2 as *mut libc::c_char) as libc::c_int;
    s = malloc((len1 + len2 + 1 as libc::c_int) as libc::c_ulong) as *mut uint8_t;
    strncpy(s as *mut libc::c_char, n1 as *mut libc::c_char, len1 as libc::c_ulong);
    strncpy(
        (s as *mut libc::c_char).offset(len1 as isize),
        n2 as *mut libc::c_char,
        len2 as libc::c_ulong,
    );
    *s.offset((len1 + len2) as isize) = '\0' as i32 as uint8_t;
    return s;
}
pub unsafe extern "C" fn nlabel_to_str(mut name: *const uint8_t) -> *mut libc::c_char {
    let mut label: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut labelp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *const uint8_t = 0 as *const uint8_t;
    if !name.is_null() {} else {
        __assert_fail(
            b"name != NULL\0" as *const u8 as *const libc::c_char,
            b"tinysvcmdns.c\0" as *const u8 as *const libc::c_char,
            117 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 37],
                &[libc::c_char; 37],
            >(b"char *nlabel_to_str(const uint8_t *)\0"))
                .as_ptr(),
        );
    }
    'c_7405: {
        if !name.is_null() {} else {
            __assert_fail(
                b"name != NULL\0" as *const u8 as *const libc::c_char,
                b"tinysvcmdns.c\0" as *const u8 as *const libc::c_char,
                117 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 37],
                    &[libc::c_char; 37],
                >(b"char *nlabel_to_str(const uint8_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    labelp = malloc(256 as libc::c_int as libc::c_ulong) as *mut libc::c_char;
    label = labelp;
    p = name;
    while *p != 0 {
        strncpy(
            labelp,
            (p as *mut libc::c_char).offset(1 as libc::c_int as isize),
            *p as libc::c_ulong,
        );
        labelp = labelp.offset(*p as libc::c_int as isize);
        *labelp = '.' as i32 as libc::c_char;
        labelp = labelp.offset(1);
        labelp;
        p = p.offset(*p as libc::c_int as isize);
        p = p.offset(1);
        p;
    }
    *labelp = '\0' as i32 as libc::c_char;
    return label;
}
unsafe extern "C" fn label_len(
    mut pkt_buf: *mut uint8_t,
    mut pkt_len: size_t,
    mut off: size_t,
) -> size_t {
    let mut p: *mut uint8_t = 0 as *mut uint8_t;
    let mut e: *mut uint8_t = pkt_buf.offset(pkt_len as isize);
    let mut len: size_t = 0 as libc::c_int as size_t;
    p = pkt_buf.offset(off as isize);
    while p < e {
        if *p as libc::c_int == 0 as libc::c_int {
            return len.wrapping_add(1 as libc::c_int as libc::c_ulong)
        } else if *p as libc::c_int & 0xc0 as libc::c_int == 0xc0 as libc::c_int {
            return len.wrapping_add(2 as libc::c_int as libc::c_ulong)
        } else {
            len = (len as libc::c_ulong)
                .wrapping_add((*p as libc::c_int + 1 as libc::c_int) as libc::c_ulong)
                as size_t as size_t;
            p = p.offset(*p as libc::c_int as isize);
        }
        p = p.offset(1);
        p;
    }
    return len;
}
pub unsafe extern "C" fn create_label(mut txt: *const libc::c_char) -> *mut uint8_t {
    let mut len: libc::c_int = 0;
    let mut s: *mut uint8_t = 0 as *mut uint8_t;
    if !txt.is_null() {} else {
        __assert_fail(
            b"txt != NULL\0" as *const u8 as *const libc::c_char,
            b"tinysvcmdns.c\0" as *const u8 as *const libc::c_char,
            163 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 36],
                &[libc::c_char; 36],
            >(b"uint8_t *create_label(const char *)\0"))
                .as_ptr(),
        );
    }
    'c_2929: {
        if !txt.is_null() {} else {
            __assert_fail(
                b"txt != NULL\0" as *const u8 as *const libc::c_char,
                b"tinysvcmdns.c\0" as *const u8 as *const libc::c_char,
                163 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 36],
                    &[libc::c_char; 36],
                >(b"uint8_t *create_label(const char *)\0"))
                    .as_ptr(),
            );
        }
    };
    len = strlen(txt) as libc::c_int;
    if len > 63 as libc::c_int {
        return 0 as *mut uint8_t;
    }
    s = malloc((len + 2 as libc::c_int) as libc::c_ulong) as *mut uint8_t;
    *s.offset(0 as libc::c_int as isize) = len as uint8_t;
    strncpy(
        (s as *mut libc::c_char).offset(1 as libc::c_int as isize),
        txt,
        len as libc::c_ulong,
    );
    *s.offset((len + 1 as libc::c_int) as isize) = '\0' as i32 as uint8_t;
    return s;
}
pub unsafe extern "C" fn create_nlabel(mut name: *const libc::c_char) -> *mut uint8_t {
    let mut label: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut e: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut lenpos: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: libc::c_int = 0 as libc::c_int;
    if !name.is_null() {} else {
        __assert_fail(
            b"name != NULL\0" as *const u8 as *const libc::c_char,
            b"tinysvcmdns.c\0" as *const u8 as *const libc::c_char,
            183 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 37],
                &[libc::c_char; 37],
            >(b"uint8_t *create_nlabel(const char *)\0"))
                .as_ptr(),
        );
    }
    'c_7275: {
        if !name.is_null() {} else {
            __assert_fail(
                b"name != NULL\0" as *const u8 as *const libc::c_char,
                b"tinysvcmdns.c\0" as *const u8 as *const libc::c_char,
                183 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 37],
                    &[libc::c_char; 37],
                >(b"uint8_t *create_nlabel(const char *)\0"))
                    .as_ptr(),
            );
        }
    };
    len = strlen(name) as libc::c_int;
    label = malloc((len + 1 as libc::c_int + 1 as libc::c_int) as libc::c_ulong)
        as *mut libc::c_char;
    if label.is_null() {
        return 0 as *mut uint8_t;
    }
    strncpy(label.offset(1 as libc::c_int as isize), name, len as libc::c_ulong);
    *label.offset((len + 1 as libc::c_int) as isize) = '\0' as i32 as libc::c_char;
    p = label;
    e = p.offset(len as isize);
    lenpos = p;
    while p < e {
        *lenpos = 0 as libc::c_int as libc::c_char;
        let mut dot: *mut libc::c_char = memchr(
            p.offset(1 as libc::c_int as isize) as *const libc::c_void,
            '.' as i32,
            (e.offset_from(p) as libc::c_long - 1 as libc::c_int as libc::c_long)
                as libc::c_ulong,
        ) as *mut libc::c_char;
        if dot.is_null() {
            dot = e.offset(1 as libc::c_int as isize);
        }
        *lenpos = (dot.offset_from(p) as libc::c_long - 1 as libc::c_int as libc::c_long)
            as libc::c_char;
        p = dot;
        lenpos = dot;
    }
    return label as *mut uint8_t;
}
unsafe extern "C" fn copy_label(
    mut pkt_buf: *mut uint8_t,
    mut pkt_len: size_t,
    mut off: size_t,
) -> *mut uint8_t {
    let mut len: libc::c_int = 0;
    if off > pkt_len {
        return 0 as *mut uint8_t;
    }
    len = *pkt_buf.offset(off as isize) as libc::c_int + 1 as libc::c_int;
    if off.wrapping_add(len as libc::c_ulong) > pkt_len {
        debug(
            1 as libc::c_int,
            b"label length exceeds packet buffer\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return 0 as *mut uint8_t;
    }
    return dup_label(pkt_buf.offset(off as isize));
}
unsafe extern "C" fn uncompress_nlabel(
    mut pkt_buf: *mut uint8_t,
    mut pkt_len: size_t,
    mut off: size_t,
) -> *mut uint8_t {
    let mut p: *mut uint8_t = 0 as *mut uint8_t;
    let mut e: *mut uint8_t = pkt_buf.offset(pkt_len as isize);
    let mut len: size_t = 0 as libc::c_int as size_t;
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sp: *mut libc::c_char = 0 as *mut libc::c_char;
    if off >= pkt_len {
        return 0 as *mut uint8_t;
    }
    p = pkt_buf.offset(off as isize);
    while *p as libc::c_int != 0 && p < e {
        let mut llen: size_t = 0 as libc::c_int as size_t;
        if *p as libc::c_int & 0xc0 as libc::c_int == 0xc0 as libc::c_int {
            let mut p2: *mut uint8_t = pkt_buf
                .offset(
                    ((*p.offset(0 as libc::c_int as isize) as libc::c_int
                        & !(0xc0 as libc::c_int)) << 8 as libc::c_int
                        | *p.offset(1 as libc::c_int as isize) as libc::c_int) as isize,
                );
            llen = (*p2 as libc::c_int + 1 as libc::c_int) as size_t;
            p = p2.offset(llen as isize).offset(-(1 as libc::c_int as isize));
        } else {
            llen = (*p as libc::c_int + 1 as libc::c_int) as size_t;
            p = p.offset(llen.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize);
        }
        len = (len as libc::c_ulong).wrapping_add(llen) as size_t as size_t;
        p = p.offset(1);
        p;
    }
    sp = malloc(len.wrapping_add(1 as libc::c_int as libc::c_ulong))
        as *mut libc::c_char;
    str = sp;
    if str.is_null() {
        return 0 as *mut uint8_t;
    }
    p = pkt_buf.offset(off as isize);
    while *p as libc::c_int != 0 && p < e {
        let mut llen_0: size_t = 0 as libc::c_int as size_t;
        if *p as libc::c_int & 0xc0 as libc::c_int == 0xc0 as libc::c_int {
            let mut p2_0: *mut uint8_t = pkt_buf
                .offset(
                    ((*p.offset(0 as libc::c_int as isize) as libc::c_int
                        & !(0xc0 as libc::c_int)) << 8 as libc::c_int
                        | *p.offset(1 as libc::c_int as isize) as libc::c_int) as isize,
                );
            llen_0 = (*p2_0 as libc::c_int + 1 as libc::c_int) as size_t;
            strncpy(sp, p2_0 as *mut libc::c_char, llen_0);
            p = p2_0.offset(llen_0 as isize).offset(-(1 as libc::c_int as isize));
        } else {
            llen_0 = (*p as libc::c_int + 1 as libc::c_int) as size_t;
            strncpy(sp, p as *mut libc::c_char, llen_0);
            p = p
                .offset(llen_0.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize);
        }
        sp = sp.offset(llen_0 as isize);
        p = p.offset(1);
        p;
    }
    *sp = '\0' as i32 as libc::c_char;
    return str as *mut uint8_t;
}
pub unsafe extern "C" fn rr_get_type_name(mut type_0: rr_type) -> *const libc::c_char {
    match type_0 as libc::c_uint {
        1 => return b"A\0" as *const u8 as *const libc::c_char,
        12 => return b"PTR\0" as *const u8 as *const libc::c_char,
        16 => return b"TXT\0" as *const u8 as *const libc::c_char,
        28 => return b"AAAA\0" as *const u8 as *const libc::c_char,
        33 => return b"SRV\0" as *const u8 as *const libc::c_char,
        47 => return b"NSEC\0" as *const u8 as *const libc::c_char,
        255 => return b"ANY\0" as *const u8 as *const libc::c_char,
        _ => {}
    }
    return 0 as *const libc::c_char;
}
pub unsafe extern "C" fn rr_entry_destroy(mut rr: *mut rr_entry) {
    let mut txt_rec: *mut rr_data_txt = 0 as *mut rr_data_txt;
    if !rr.is_null() {} else {
        __assert_fail(
            b"rr\0" as *const u8 as *const libc::c_char,
            b"tinysvcmdns.c\0" as *const u8 as *const libc::c_char,
            293 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 41],
                &[libc::c_char; 41],
            >(b"void rr_entry_destroy(struct rr_entry *)\0"))
                .as_ptr(),
        );
    }
    'c_2532: {
        if !rr.is_null() {} else {
            __assert_fail(
                b"rr\0" as *const u8 as *const libc::c_char,
                b"tinysvcmdns.c\0" as *const u8 as *const libc::c_char,
                293 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 41],
                    &[libc::c_char; 41],
                >(b"void rr_entry_destroy(struct rr_entry *)\0"))
                    .as_ptr(),
            );
        }
    };
    match (*rr).type_0 as libc::c_uint {
        12 => {
            if !((*rr).data.PTR.name).is_null() {
                free((*rr).data.PTR.name as *mut libc::c_void);
            }
        }
        16 => {
            txt_rec = &mut (*rr).data.TXT;
            while !txt_rec.is_null() {
                let mut next: *mut rr_data_txt = (*txt_rec).next;
                if !((*txt_rec).txt).is_null() {
                    free((*txt_rec).txt as *mut libc::c_void);
                }
                if txt_rec != &mut (*rr).data.TXT as *mut rr_data_txt {
                    free(txt_rec as *mut libc::c_void);
                }
                txt_rec = next;
            }
        }
        33 => {
            if !((*rr).data.SRV.target).is_null() {
                free((*rr).data.SRV.target as *mut libc::c_void);
            }
        }
        _ => {}
    }
    free((*rr).name as *mut libc::c_void);
    free(rr as *mut libc::c_void);
}
pub unsafe extern "C" fn rr_list_destroy(
    mut rr: *mut rr_list,
    mut destroy_items: libc::c_char,
) {
    let mut rr_next: *mut rr_list = 0 as *mut rr_list;
    while !rr.is_null() {
        rr_next = (*rr).next;
        if destroy_items != 0 {
            rr_entry_destroy((*rr).e);
        }
        free(rr as *mut libc::c_void);
        rr = rr_next;
    }
}
pub unsafe extern "C" fn rr_list_count(mut rr: *mut rr_list) -> libc::c_int {
    let mut i: libc::c_int = 0 as libc::c_int;
    while !rr.is_null() {
        i += 1;
        i;
        rr = (*rr).next;
    }
    return i;
}
pub unsafe extern "C" fn rr_list_remove(
    mut rr_head: *mut *mut rr_list,
    mut rr: *mut rr_entry,
) -> *mut rr_entry {
    let mut le: *mut rr_list = *rr_head;
    let mut pe: *mut rr_list = 0 as *mut rr_list;
    while !le.is_null() {
        if (*le).e == rr {
            if pe.is_null() {
                *rr_head = (*le).next;
                free(le as *mut libc::c_void);
                return rr;
            } else {
                (*pe).next = (*le).next;
                free(le as *mut libc::c_void);
                return rr;
            }
        }
        pe = le;
        le = (*le).next;
    }
    return 0 as *mut rr_entry;
}
pub unsafe extern "C" fn rr_list_append(
    mut rr_head: *mut *mut rr_list,
    mut rr: *mut rr_entry,
) -> libc::c_int {
    let mut node: *mut rr_list = malloc(
        ::std::mem::size_of::<rr_list>() as libc::c_ulong,
    ) as *mut rr_list;
    (*node).e = rr;
    (*node).next = 0 as *mut rr_list;
    if (*rr_head).is_null() {
        *rr_head = node;
    } else {
        let mut e: *mut rr_list = *rr_head;
        let mut taile: *mut rr_list = 0 as *mut rr_list;
        while !e.is_null() {
            if (*e).e == rr {
                free(node as *mut libc::c_void);
                return 0 as libc::c_int;
            }
            if ((*e).next).is_null() {
                taile = e;
            }
            e = (*e).next;
        }
        (*taile).next = node;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn rr_create_a(
    mut name: *mut uint8_t,
    mut addr: uint32_t,
) -> *mut rr_entry {
    let mut rr: *mut rr_entry = malloc(
        ::std::mem::size_of::<rr_entry>() as libc::c_ulong,
    ) as *mut rr_entry;
    memset(
        rr as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<rr_entry>() as libc::c_ulong,
    );
    (*rr).name = name;
    (*rr).type_0 = RR_A;
    (*rr).ttl = 120 as libc::c_int as uint32_t;
    (*rr).cache_flush = 1 as libc::c_int as libc::c_char;
    (*rr).rr_class = 1 as libc::c_int as uint16_t;
    (*rr).data.A.addr = addr;
    return rr;
}
pub unsafe extern "C" fn rr_create_aaaa(
    mut name: *mut uint8_t,
    mut addr: *mut in6_addr,
) -> *mut rr_entry {
    let mut rr: *mut rr_entry = malloc(
        ::std::mem::size_of::<rr_entry>() as libc::c_ulong,
    ) as *mut rr_entry;
    memset(
        rr as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<rr_entry>() as libc::c_ulong,
    );
    (*rr).name = name;
    (*rr).type_0 = RR_AAAA;
    (*rr).ttl = 120 as libc::c_int as uint32_t;
    (*rr).cache_flush = 1 as libc::c_int as libc::c_char;
    (*rr).rr_class = 1 as libc::c_int as uint16_t;
    (*rr).data.AAAA.addr = addr;
    return rr;
}
pub unsafe extern "C" fn rr_create_srv(
    mut name: *mut uint8_t,
    mut port: uint16_t,
    mut target: *mut uint8_t,
) -> *mut rr_entry {
    let mut rr: *mut rr_entry = malloc(
        ::std::mem::size_of::<rr_entry>() as libc::c_ulong,
    ) as *mut rr_entry;
    memset(
        rr as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<rr_entry>() as libc::c_ulong,
    );
    (*rr).name = name;
    (*rr).type_0 = RR_SRV;
    (*rr).ttl = 120 as libc::c_int as uint32_t;
    (*rr).cache_flush = 1 as libc::c_int as libc::c_char;
    (*rr).rr_class = 1 as libc::c_int as uint16_t;
    (*rr).data.SRV.port = port;
    (*rr).data.SRV.target = target;
    return rr;
}
pub unsafe extern "C" fn rr_create_ptr(
    mut name: *mut uint8_t,
    mut d_rr: *mut rr_entry,
) -> *mut rr_entry {
    let mut rr: *mut rr_entry = malloc(
        ::std::mem::size_of::<rr_entry>() as libc::c_ulong,
    ) as *mut rr_entry;
    memset(
        rr as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<rr_entry>() as libc::c_ulong,
    );
    (*rr).name = name;
    (*rr).type_0 = RR_PTR;
    (*rr).ttl = 120 as libc::c_int as uint32_t;
    (*rr).cache_flush = 1 as libc::c_int as libc::c_char;
    (*rr).rr_class = 1 as libc::c_int as uint16_t;
    (*rr).cache_flush = 0 as libc::c_int as libc::c_char;
    (*rr).data.PTR.entry = d_rr;
    return rr;
}
pub unsafe extern "C" fn rr_create(
    mut name: *mut uint8_t,
    mut type_0: rr_type,
) -> *mut rr_entry {
    let mut rr: *mut rr_entry = malloc(
        ::std::mem::size_of::<rr_entry>() as libc::c_ulong,
    ) as *mut rr_entry;
    memset(
        rr as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<rr_entry>() as libc::c_ulong,
    );
    (*rr).name = name;
    (*rr).type_0 = type_0;
    (*rr).ttl = 120 as libc::c_int as uint32_t;
    (*rr).cache_flush = 1 as libc::c_int as libc::c_char;
    (*rr).rr_class = 1 as libc::c_int as uint16_t;
    return rr;
}
pub unsafe extern "C" fn rr_set_nsec(mut rr_nsec: *mut rr_entry, mut type_0: rr_type) {
    (*rr_nsec).type_0 = RR_NSEC;
    if (*rr_nsec).type_0 as u64 != 0 {} else {
        __assert_fail(
            b"rr_nsec->type = RR_NSEC\0" as *const u8 as *const libc::c_char,
            b"tinysvcmdns.c\0" as *const u8 as *const libc::c_char,
            440 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 50],
                &[libc::c_char; 50],
            >(b"void rr_set_nsec(struct rr_entry *, enum rr_type)\0"))
                .as_ptr(),
        );
    }
    'c_6864: {
        (*rr_nsec).type_0 = RR_NSEC;
        if (*rr_nsec).type_0 as u64 != 0 {} else {
            __assert_fail(
                b"rr_nsec->type = RR_NSEC\0" as *const u8 as *const libc::c_char,
                b"tinysvcmdns.c\0" as *const u8 as *const libc::c_char,
                440 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 50],
                    &[libc::c_char; 50],
                >(b"void rr_set_nsec(struct rr_entry *, enum rr_type)\0"))
                    .as_ptr(),
            );
        }
    };
    if ((type_0 as libc::c_uint).wrapping_div(8 as libc::c_int as libc::c_uint)
        as libc::c_ulong) < ::std::mem::size_of::<[uint8_t; 5]>() as libc::c_ulong
    {} else {
        __assert_fail(
            b"(type / 8) < sizeof(rr_nsec->data.NSEC.bitmap)\0" as *const u8
                as *const libc::c_char,
            b"tinysvcmdns.c\0" as *const u8 as *const libc::c_char,
            441 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 50],
                &[libc::c_char; 50],
            >(b"void rr_set_nsec(struct rr_entry *, enum rr_type)\0"))
                .as_ptr(),
        );
    }
    'c_6804: {
        if ((type_0 as libc::c_uint).wrapping_div(8 as libc::c_int as libc::c_uint)
            as libc::c_ulong) < ::std::mem::size_of::<[uint8_t; 5]>() as libc::c_ulong
        {} else {
            __assert_fail(
                b"(type / 8) < sizeof(rr_nsec->data.NSEC.bitmap)\0" as *const u8
                    as *const libc::c_char,
                b"tinysvcmdns.c\0" as *const u8 as *const libc::c_char,
                441 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 50],
                    &[libc::c_char; 50],
                >(b"void rr_set_nsec(struct rr_entry *, enum rr_type)\0"))
                    .as_ptr(),
            );
        }
    };
    (*rr_nsec)
        .data
        .NSEC
        .bitmap[(type_0 as libc::c_uint).wrapping_div(8 as libc::c_int as libc::c_uint)
        as usize] = ((1 as libc::c_int)
        << (7 as libc::c_int as libc::c_uint)
            .wrapping_sub(
                (type_0 as libc::c_uint).wrapping_rem(8 as libc::c_int as libc::c_uint),
            )) as uint8_t;
}
pub unsafe extern "C" fn rr_add_txt(
    mut rr_txt: *mut rr_entry,
    mut txt: *const libc::c_char,
) {
    let mut txt_rec: *mut rr_data_txt = 0 as *mut rr_data_txt;
    if (*rr_txt).type_0 as libc::c_uint == RR_TXT as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"rr_txt->type == RR_TXT\0" as *const u8 as *const libc::c_char,
            b"tinysvcmdns.c\0" as *const u8 as *const libc::c_char,
            448 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 49],
                &[libc::c_char; 49],
            >(b"void rr_add_txt(struct rr_entry *, const char *)\0"))
                .as_ptr(),
        );
    }
    'c_6992: {
        if (*rr_txt).type_0 as libc::c_uint == RR_TXT as libc::c_int as libc::c_uint
        {} else {
            __assert_fail(
                b"rr_txt->type == RR_TXT\0" as *const u8 as *const libc::c_char,
                b"tinysvcmdns.c\0" as *const u8 as *const libc::c_char,
                448 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 49],
                    &[libc::c_char; 49],
                >(b"void rr_add_txt(struct rr_entry *, const char *)\0"))
                    .as_ptr(),
            );
        }
    };
    txt_rec = &mut (*rr_txt).data.TXT;
    if ((*txt_rec).txt).is_null() {
        (*txt_rec).txt = create_label(txt);
        return;
    }
    while !((*txt_rec).next).is_null() {
        txt_rec = (*txt_rec).next;
    }
    (*txt_rec)
        .next = malloc(::std::mem::size_of::<rr_data_txt>() as libc::c_ulong)
        as *mut rr_data_txt;
    txt_rec = (*txt_rec).next;
    (*txt_rec).txt = create_label(txt);
    (*txt_rec).next = 0 as *mut rr_data_txt;
}
pub unsafe extern "C" fn rr_group_add(
    mut group: *mut *mut rr_group,
    mut rr: *mut rr_entry,
) {
    let mut g: *mut rr_group = 0 as *mut rr_group;
    if !rr.is_null() {} else {
        __assert_fail(
            b"rr != NULL\0" as *const u8 as *const libc::c_char,
            b"tinysvcmdns.c\0" as *const u8 as *const libc::c_char,
            473 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 57],
                &[libc::c_char; 57],
            >(b"void rr_group_add(struct rr_group **, struct rr_entry *)\0"))
                .as_ptr(),
        );
    }
    'c_6249: {
        if !rr.is_null() {} else {
            __assert_fail(
                b"rr != NULL\0" as *const u8 as *const libc::c_char,
                b"tinysvcmdns.c\0" as *const u8 as *const libc::c_char,
                473 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 57],
                    &[libc::c_char; 57],
                >(b"void rr_group_add(struct rr_group **, struct rr_entry *)\0"))
                    .as_ptr(),
            );
        }
    };
    if !(*group).is_null() {
        g = rr_group_find(*group, (*rr).name);
        if !g.is_null() {
            rr_list_append(&mut (*g).rr, rr);
            return;
        }
    }
    g = malloc(::std::mem::size_of::<rr_group>() as libc::c_ulong) as *mut rr_group;
    memset(
        g as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<rr_group>() as libc::c_ulong,
    );
    (*g).name = dup_nlabel((*rr).name);
    rr_list_append(&mut (*g).rr, rr);
    (*g).next = *group;
    *group = g;
}
pub unsafe extern "C" fn rr_group_find(
    mut g: *mut rr_group,
    mut name: *mut uint8_t,
) -> *mut rr_group {
    while !g.is_null() {
        if cmp_nlabel((*g).name, name) == 0 as libc::c_int {
            return g;
        }
        g = (*g).next;
    }
    return 0 as *mut rr_group;
}
pub unsafe extern "C" fn rr_entry_find(
    mut rr_list: *mut rr_list,
    mut name: *mut uint8_t,
    mut type_0: uint16_t,
) -> *mut rr_entry {
    let mut rr: *mut rr_list = rr_list;
    while !rr.is_null() {
        if (*(*rr).e).type_0 as libc::c_uint == type_0 as libc::c_uint
            && cmp_nlabel((*(*rr).e).name, name) == 0 as libc::c_int
        {
            return (*rr).e;
        }
        rr = (*rr).next;
    }
    return 0 as *mut rr_entry;
}
pub unsafe extern "C" fn rr_entry_match(
    mut rr_list: *mut rr_list,
    mut entry: *mut rr_entry,
) -> *mut rr_entry {
    let mut rr: *mut rr_list = rr_list;
    while !rr.is_null() {
        if (*(*rr).e).type_0 as libc::c_uint == (*entry).type_0 as libc::c_uint
            && cmp_nlabel((*(*rr).e).name, (*entry).name) == 0 as libc::c_int
        {
            if (*entry).type_0 as libc::c_uint != RR_PTR as libc::c_int as libc::c_uint {
                return (*rr).e
            } else if cmp_nlabel(
                (if !((*entry).data.PTR.name).is_null() {
                    (*entry).data.PTR.name
                } else {
                    (*(*entry).data.PTR.entry).name
                }),
                (if !((*(*rr).e).data.PTR.name).is_null() {
                    (*(*rr).e).data.PTR.name
                } else {
                    (*(*(*rr).e).data.PTR.entry).name
                }),
            ) == 0 as libc::c_int
            {
                return (*rr).e
            }
        }
        rr = (*rr).next;
    }
    return 0 as *mut rr_entry;
}
pub unsafe extern "C" fn rr_group_destroy(mut group: *mut rr_group) {
    let mut g: *mut rr_group = group;
    while !g.is_null() {
        let mut nextg: *mut rr_group = (*g).next;
        free((*g).name as *mut libc::c_void);
        rr_list_destroy((*g).rr, 1 as libc::c_int as libc::c_char);
        free(g as *mut libc::c_void);
        g = nextg;
    }
}
pub unsafe extern "C" fn mdns_write_u16(
    mut ptr: *mut uint8_t,
    v: uint16_t,
) -> *mut uint8_t {
    let fresh0 = ptr;
    ptr = ptr.offset(1);
    *fresh0 = ((v as libc::c_int >> 8 as libc::c_int) as uint8_t as libc::c_int
        & 0xff as libc::c_int) as uint8_t;
    let fresh1 = ptr;
    ptr = ptr.offset(1);
    *fresh1 = ((v as libc::c_int >> 0 as libc::c_int) as uint8_t as libc::c_int
        & 0xff as libc::c_int) as uint8_t;
    return ptr;
}
pub unsafe extern "C" fn mdns_write_u32(
    mut ptr: *mut uint8_t,
    v: uint32_t,
) -> *mut uint8_t {
    let fresh2 = ptr;
    ptr = ptr.offset(1);
    *fresh2 = ((v >> 24 as libc::c_int) as uint8_t as libc::c_int & 0xff as libc::c_int)
        as uint8_t;
    let fresh3 = ptr;
    ptr = ptr.offset(1);
    *fresh3 = ((v >> 16 as libc::c_int) as uint8_t as libc::c_int & 0xff as libc::c_int)
        as uint8_t;
    let fresh4 = ptr;
    ptr = ptr.offset(1);
    *fresh4 = ((v >> 8 as libc::c_int) as uint8_t as libc::c_int & 0xff as libc::c_int)
        as uint8_t;
    let fresh5 = ptr;
    ptr = ptr.offset(1);
    *fresh5 = ((v >> 0 as libc::c_int) as uint8_t as libc::c_int & 0xff as libc::c_int)
        as uint8_t;
    return ptr;
}
pub unsafe extern "C" fn mdns_read_u16(mut ptr: *const uint8_t) -> uint16_t {
    return ((*ptr.offset(0 as libc::c_int as isize) as libc::c_int & 0xff as libc::c_int)
        << 8 as libc::c_int
        | (*ptr.offset(1 as libc::c_int as isize) as libc::c_int & 0xff as libc::c_int)
            << 0 as libc::c_int) as uint16_t;
}
pub unsafe extern "C" fn mdns_read_u32(mut ptr: *const uint8_t) -> uint32_t {
    return ((*ptr.offset(0 as libc::c_int as isize) as libc::c_int & 0xff as libc::c_int)
        << 24 as libc::c_int
        | (*ptr.offset(1 as libc::c_int as isize) as libc::c_int & 0xff as libc::c_int)
            << 16 as libc::c_int
        | (*ptr.offset(2 as libc::c_int as isize) as libc::c_int & 0xff as libc::c_int)
            << 8 as libc::c_int
        | (*ptr.offset(3 as libc::c_int as isize) as libc::c_int & 0xff as libc::c_int)
            << 0 as libc::c_int) as uint32_t;
}
pub unsafe extern "C" fn mdns_init_reply(mut pkt: *mut mdns_pkt, mut id: uint16_t) {
    (*pkt).id = id;
    (*pkt)
        .flags = ((1 as libc::c_int) << 15 as libc::c_int
        | (1 as libc::c_int) << 10 as libc::c_int) as uint16_t;
    rr_list_destroy((*pkt).rr_qn, 0 as libc::c_int as libc::c_char);
    rr_list_destroy((*pkt).rr_ans, 0 as libc::c_int as libc::c_char);
    rr_list_destroy((*pkt).rr_auth, 0 as libc::c_int as libc::c_char);
    rr_list_destroy((*pkt).rr_add, 0 as libc::c_int as libc::c_char);
    (*pkt).rr_qn = 0 as *mut rr_list;
    (*pkt).rr_ans = 0 as *mut rr_list;
    (*pkt).rr_auth = 0 as *mut rr_list;
    (*pkt).rr_add = 0 as *mut rr_list;
    (*pkt).num_qn = 0 as libc::c_int as uint16_t;
    (*pkt).num_ans_rr = 0 as libc::c_int as uint16_t;
    (*pkt).num_auth_rr = 0 as libc::c_int as uint16_t;
    (*pkt).num_add_rr = 0 as libc::c_int as uint16_t;
}
pub unsafe extern "C" fn mdns_pkt_destroy(mut p: *mut mdns_pkt) {
    rr_list_destroy((*p).rr_qn, 1 as libc::c_int as libc::c_char);
    rr_list_destroy((*p).rr_ans, 1 as libc::c_int as libc::c_char);
    rr_list_destroy((*p).rr_auth, 1 as libc::c_int as libc::c_char);
    rr_list_destroy((*p).rr_add, 1 as libc::c_int as libc::c_char);
    free(p as *mut libc::c_void);
}
unsafe extern "C" fn mdns_parse_qn(
    mut pkt_buf: *mut uint8_t,
    mut pkt_len: size_t,
    mut off: size_t,
    mut pkt: *mut mdns_pkt,
) -> size_t {
    let mut p: *const uint8_t = pkt_buf.offset(off as isize);
    let mut rr: *mut rr_entry = 0 as *mut rr_entry;
    let mut name: *mut uint8_t = 0 as *mut uint8_t;
    if !pkt.is_null() {} else {
        __assert_fail(
            b"pkt != NULL\0" as *const u8 as *const libc::c_char,
            b"tinysvcmdns.c\0" as *const u8 as *const libc::c_char,
            609 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 67],
                &[libc::c_char; 67],
            >(b"size_t mdns_parse_qn(uint8_t *, size_t, size_t, struct mdns_pkt *)\0"))
                .as_ptr(),
        );
    }
    'c_4167: {
        if !pkt.is_null() {} else {
            __assert_fail(
                b"pkt != NULL\0" as *const u8 as *const libc::c_char,
                b"tinysvcmdns.c\0" as *const u8 as *const libc::c_char,
                609 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 67],
                    &[libc::c_char; 67],
                >(
                    b"size_t mdns_parse_qn(uint8_t *, size_t, size_t, struct mdns_pkt *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    rr = malloc(::std::mem::size_of::<rr_entry>() as libc::c_ulong) as *mut rr_entry;
    memset(
        rr as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<rr_entry>() as libc::c_ulong,
    );
    name = uncompress_nlabel(pkt_buf, pkt_len, off);
    p = p.offset(label_len(pkt_buf, pkt_len, off) as isize);
    (*rr).name = name;
    (*rr).type_0 = mdns_read_u16(p) as rr_type;
    p = p.offset(::std::mem::size_of::<uint16_t>() as libc::c_ulong as isize);
    (*rr)
        .unicast_query = (*p as libc::c_int & 0x80 as libc::c_int == 0x80 as libc::c_int)
        as libc::c_int as libc::c_char;
    (*rr)
        .rr_class = (mdns_read_u16(p) as libc::c_int & !(0x80 as libc::c_int))
        as uint16_t;
    p = p.offset(::std::mem::size_of::<uint16_t>() as libc::c_ulong as isize);
    rr_list_append(&mut (*pkt).rr_qn, rr);
    return p.offset_from(pkt_buf.offset(off as isize)) as libc::c_long as size_t;
}
unsafe extern "C" fn mdns_parse_rr(
    mut pkt_buf: *mut uint8_t,
    mut pkt_len: size_t,
    mut off: size_t,
    mut pkt: *mut mdns_pkt,
) -> size_t {
    let mut p: *const uint8_t = pkt_buf.offset(off as isize);
    let mut e: *const uint8_t = pkt_buf.offset(pkt_len as isize);
    let mut rr: *mut rr_entry = 0 as *mut rr_entry;
    let mut name: *mut uint8_t = 0 as *mut uint8_t;
    let mut rr_data_len: size_t = 0 as libc::c_int as size_t;
    let mut txt_rec: *mut rr_data_txt = 0 as *mut rr_data_txt;
    let mut parse_error: libc::c_int = 0 as libc::c_int;
    if !pkt.is_null() {} else {
        __assert_fail(
            b"pkt != NULL\0" as *const u8 as *const libc::c_char,
            b"tinysvcmdns.c\0" as *const u8 as *const libc::c_char,
            642 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 67],
                &[libc::c_char; 67],
            >(b"size_t mdns_parse_rr(uint8_t *, size_t, size_t, struct mdns_pkt *)\0"))
                .as_ptr(),
        );
    }
    'c_3832: {
        if !pkt.is_null() {} else {
            __assert_fail(
                b"pkt != NULL\0" as *const u8 as *const libc::c_char,
                b"tinysvcmdns.c\0" as *const u8 as *const libc::c_char,
                642 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 67],
                    &[libc::c_char; 67],
                >(
                    b"size_t mdns_parse_rr(uint8_t *, size_t, size_t, struct mdns_pkt *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if off > pkt_len {
        return 0 as libc::c_int as size_t;
    }
    rr = malloc(::std::mem::size_of::<rr_entry>() as libc::c_ulong) as *mut rr_entry;
    memset(
        rr as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<rr_entry>() as libc::c_ulong,
    );
    name = uncompress_nlabel(pkt_buf, pkt_len, off);
    p = p.offset(label_len(pkt_buf, pkt_len, off) as isize);
    (*rr).name = name;
    (*rr).type_0 = mdns_read_u16(p) as rr_type;
    p = p.offset(::std::mem::size_of::<uint16_t>() as libc::c_ulong as isize);
    (*rr)
        .cache_flush = (*p as libc::c_int & 0x80 as libc::c_int == 0x80 as libc::c_int)
        as libc::c_int as libc::c_char;
    (*rr)
        .rr_class = (mdns_read_u16(p) as libc::c_int & !(0x80 as libc::c_int))
        as uint16_t;
    p = p.offset(::std::mem::size_of::<uint16_t>() as libc::c_ulong as isize);
    (*rr).ttl = mdns_read_u32(p);
    p = p.offset(::std::mem::size_of::<uint32_t>() as libc::c_ulong as isize);
    rr_data_len = mdns_read_u16(p) as size_t;
    p = p.offset(::std::mem::size_of::<uint16_t>() as libc::c_ulong as isize);
    if p.offset(rr_data_len as isize) > e {
        debug(
            1 as libc::c_int,
            b"rr_data_len goes beyond packet buffer: %lu > %lu\n\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            rr_data_len,
            e.offset_from(p) as libc::c_long,
        );
        rr_entry_destroy(rr);
        return 0 as libc::c_int as size_t;
    }
    e = p.offset(rr_data_len as isize);
    let mut i: libc::c_int = 0;
    match (*rr).type_0 as libc::c_uint {
        1 => {
            if rr_data_len < ::std::mem::size_of::<uint32_t>() as libc::c_ulong {
                debug(
                    1 as libc::c_int,
                    b"invalid rr_data_len=%lu for A record\n\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    rr_data_len,
                );
                parse_error = 1 as libc::c_int;
            } else {
                (*rr).data.A.addr = __bswap_32(mdns_read_u32(p));
                p = p
                    .offset(::std::mem::size_of::<uint32_t>() as libc::c_ulong as isize);
            }
        }
        28 => {
            if rr_data_len < ::std::mem::size_of::<in6_addr>() as libc::c_ulong {
                debug(
                    1 as libc::c_int,
                    b"invalid rr_data_len=%lu for AAAA record\n\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    rr_data_len,
                );
                parse_error = 1 as libc::c_int;
            } else {
                (*rr)
                    .data
                    .AAAA
                    .addr = malloc(::std::mem::size_of::<in6_addr>() as libc::c_ulong)
                    as *mut in6_addr;
                i = 0;
                i = 0 as libc::c_int;
                while (i as libc::c_ulong)
                    < ::std::mem::size_of::<in6_addr>() as libc::c_ulong
                {
                    (*(*rr).data.AAAA.addr)
                        .__in6_u
                        .__u6_addr8[i as usize] = *p.offset(i as isize);
                    i += 1;
                    i;
                }
                p = p
                    .offset(::std::mem::size_of::<in6_addr>() as libc::c_ulong as isize);
            }
        }
        12 => {
            (*rr)
                .data
                .PTR
                .name = uncompress_nlabel(
                pkt_buf,
                pkt_len,
                p.offset_from(pkt_buf) as libc::c_long as size_t,
            );
            if ((*rr).data.PTR.name).is_null() {
                debug(
                    1 as libc::c_int,
                    b"unable to parse/uncompress label for PTR name\n\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                );
                parse_error = 1 as libc::c_int;
            } else {
                p = p.offset(rr_data_len as isize);
            }
        }
        16 => {
            txt_rec = &mut (*rr).data.TXT;
            if rr_data_len == 0 as libc::c_int as libc::c_ulong {
                debug(
                    1 as libc::c_int,
                    b"WARN: rr_data_len for TXT is 0\n\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                );
                (*txt_rec).txt = create_label(b"\0" as *const u8 as *const libc::c_char);
            } else {
                loop {
                    (*txt_rec)
                        .txt = copy_label(
                        pkt_buf,
                        pkt_len,
                        p.offset_from(pkt_buf) as libc::c_long as size_t,
                    );
                    if ((*txt_rec).txt).is_null() {
                        debug(
                            1 as libc::c_int,
                            b"unable to copy label for TXT record\n\0" as *const u8
                                as *const libc::c_char as *mut libc::c_char,
                        );
                        parse_error = 1 as libc::c_int;
                        break;
                    } else {
                        p = p
                            .offset(
                                (*((*txt_rec).txt).offset(0 as libc::c_int as isize)
                                    as libc::c_int + 1 as libc::c_int) as isize,
                            );
                        if p >= e {
                            break;
                        }
                        (*txt_rec)
                            .next = malloc(
                            ::std::mem::size_of::<rr_data_txt>() as libc::c_ulong,
                        ) as *mut rr_data_txt;
                        txt_rec = (*txt_rec).next;
                        (*txt_rec).next = 0 as *mut rr_data_txt;
                    }
                }
            }
        }
        _ => {
            p = e;
        }
    }
    if parse_error != 0 {
        rr_entry_destroy(rr);
        return 0 as libc::c_int as size_t;
    }
    rr_list_append(&mut (*pkt).rr_ans, rr);
    return p.offset_from(pkt_buf.offset(off as isize)) as libc::c_long as size_t;
}
pub unsafe extern "C" fn mdns_parse_pkt(
    mut pkt_buf: *mut uint8_t,
    mut pkt_len: size_t,
) -> *mut mdns_pkt {
    let mut p: *mut uint8_t = pkt_buf;
    let mut off: size_t = 0;
    let mut pkt: *mut mdns_pkt = 0 as *mut mdns_pkt;
    let mut i: libc::c_int = 0;
    if pkt_len < 12 as libc::c_int as libc::c_ulong {
        return 0 as *mut mdns_pkt;
    }
    pkt = malloc(::std::mem::size_of::<mdns_pkt>() as libc::c_ulong) as *mut mdns_pkt;
    memset(
        pkt as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<mdns_pkt>() as libc::c_ulong,
    );
    (*pkt).id = mdns_read_u16(p);
    p = p.offset(::std::mem::size_of::<uint16_t>() as libc::c_ulong as isize);
    (*pkt).flags = mdns_read_u16(p);
    p = p.offset(::std::mem::size_of::<uint16_t>() as libc::c_ulong as isize);
    (*pkt).num_qn = mdns_read_u16(p);
    p = p.offset(::std::mem::size_of::<uint16_t>() as libc::c_ulong as isize);
    (*pkt).num_ans_rr = mdns_read_u16(p);
    p = p.offset(::std::mem::size_of::<uint16_t>() as libc::c_ulong as isize);
    (*pkt).num_auth_rr = mdns_read_u16(p);
    p = p.offset(::std::mem::size_of::<uint16_t>() as libc::c_ulong as isize);
    (*pkt).num_add_rr = mdns_read_u16(p);
    p = p.offset(::std::mem::size_of::<uint16_t>() as libc::c_ulong as isize);
    off = p.offset_from(pkt_buf) as libc::c_long as size_t;
    i = 0 as libc::c_int;
    while i < (*pkt).num_qn as libc::c_int {
        let mut l: size_t = mdns_parse_qn(pkt_buf, pkt_len, off, pkt);
        if l == 0 {
            debug(
                1 as libc::c_int,
                b"error parsing question #%d\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                i,
            );
            mdns_pkt_destroy(pkt);
            return 0 as *mut mdns_pkt;
        }
        off = (off as libc::c_ulong).wrapping_add(l) as size_t as size_t;
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < (*pkt).num_ans_rr as libc::c_int {
        let mut l_0: size_t = mdns_parse_rr(pkt_buf, pkt_len, off, pkt);
        if l_0 == 0 {
            debug(
                1 as libc::c_int,
                b"error parsing answer #%d\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                i,
            );
            mdns_pkt_destroy(pkt);
            return 0 as *mut mdns_pkt;
        }
        off = (off as libc::c_ulong).wrapping_add(l_0) as size_t as size_t;
        i += 1;
        i;
    }
    return pkt;
}
unsafe extern "C" fn mdns_encode_name(
    mut pkt_buf: *mut uint8_t,
    mut pkt_len: size_t,
    mut off: size_t,
    mut name: *const uint8_t,
    mut comp: *mut name_comp,
) -> size_t {
    let mut c: *mut name_comp = 0 as *mut name_comp;
    let mut c_tail: *mut name_comp = 0 as *mut name_comp;
    let mut p: *mut uint8_t = pkt_buf.offset(off as isize);
    let mut len: size_t = 0 as libc::c_int as size_t;
    if !name.is_null() {
        while *name != 0 {
            c = comp;
            while !c.is_null() {
                if cmp_nlabel(name, (*c).label) == 0 as libc::c_int {
                    mdns_write_u16(
                        p,
                        (0xc000 as libc::c_int as libc::c_ulong
                            | (*c).pos & !(0xc000 as libc::c_int) as libc::c_ulong)
                            as uint16_t,
                    );
                    return len
                        .wrapping_add(
                            ::std::mem::size_of::<uint16_t>() as libc::c_ulong,
                        );
                }
                if ((*c).next).is_null() {
                    c_tail = c;
                }
                c = (*c).next;
            }
            let mut segment_len: libc::c_int = *name as libc::c_int + 1 as libc::c_int;
            strncpy(
                p as *mut libc::c_char,
                name as *mut libc::c_char,
                segment_len as libc::c_ulong,
            );
            let mut new_c: *mut name_comp = malloc(
                ::std::mem::size_of::<name_comp>() as libc::c_ulong,
            ) as *mut name_comp;
            memset(
                new_c as *mut libc::c_void,
                0 as libc::c_int,
                ::std::mem::size_of::<name_comp>() as libc::c_ulong,
            );
            (*new_c).label = name as *mut uint8_t;
            (*new_c).pos = p.offset_from(pkt_buf) as libc::c_long as size_t;
            (*c_tail).next = new_c;
            p = p.offset(segment_len as isize);
            len = (len as libc::c_ulong).wrapping_add(segment_len as libc::c_ulong)
                as size_t as size_t;
            name = name.offset(segment_len as isize);
        }
    }
    *p = '\0' as i32 as uint8_t;
    len = (len as libc::c_ulong).wrapping_add(1 as libc::c_int as libc::c_ulong)
        as size_t as size_t;
    return len;
}
unsafe extern "C" fn mdns_encode_rr(
    mut pkt_buf: *mut uint8_t,
    mut pkt_len: size_t,
    mut off: size_t,
    mut rr: *mut rr_entry,
    mut comp: *mut name_comp,
) -> size_t {
    let mut p: *mut uint8_t = pkt_buf.offset(off as isize);
    let mut p_data: *mut uint8_t = 0 as *mut uint8_t;
    let mut l: size_t = 0;
    let mut txt_rec: *mut rr_data_txt = 0 as *mut rr_data_txt;
    let mut label: *mut uint8_t = 0 as *mut uint8_t;
    let mut i: libc::c_int = 0;
    if off < pkt_len {} else {
        __assert_fail(
            b"off < pkt_len\0" as *const u8 as *const libc::c_char,
            b"tinysvcmdns.c\0" as *const u8 as *const libc::c_char,
            862 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 88],
                &[libc::c_char; 88],
            >(
                b"size_t mdns_encode_rr(uint8_t *, size_t, size_t, struct rr_entry *, struct name_comp *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_5484: {
        if off < pkt_len {} else {
            __assert_fail(
                b"off < pkt_len\0" as *const u8 as *const libc::c_char,
                b"tinysvcmdns.c\0" as *const u8 as *const libc::c_char,
                862 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 88],
                    &[libc::c_char; 88],
                >(
                    b"size_t mdns_encode_rr(uint8_t *, size_t, size_t, struct rr_entry *, struct name_comp *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    l = mdns_encode_name(pkt_buf, pkt_len, off, (*rr).name, comp);
    if l != 0 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"l != 0\0" as *const u8 as *const libc::c_char,
            b"tinysvcmdns.c\0" as *const u8 as *const libc::c_char,
            866 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 88],
                &[libc::c_char; 88],
            >(
                b"size_t mdns_encode_rr(uint8_t *, size_t, size_t, struct rr_entry *, struct name_comp *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_5426: {
        if l != 0 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"l != 0\0" as *const u8 as *const libc::c_char,
                b"tinysvcmdns.c\0" as *const u8 as *const libc::c_char,
                866 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 88],
                    &[libc::c_char; 88],
                >(
                    b"size_t mdns_encode_rr(uint8_t *, size_t, size_t, struct rr_entry *, struct name_comp *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    p = p.offset(l as isize);
    p = mdns_write_u16(p, (*rr).type_0 as uint16_t);
    p = mdns_write_u16(
        p,
        ((*rr).rr_class as libc::c_int & !(0x8000 as libc::c_int)
            | ((*rr).cache_flush as libc::c_int) << 15 as libc::c_int) as uint16_t,
    );
    p = mdns_write_u32(p, (*rr).ttl);
    p = p.offset(::std::mem::size_of::<uint16_t>() as libc::c_ulong as isize);
    p_data = p;
    match (*rr).type_0 as libc::c_uint {
        1 => {
            p = mdns_write_u32(p, __bswap_32((*rr).data.A.addr));
        }
        28 => {
            i = 0 as libc::c_int;
            while (i as libc::c_ulong)
                < ::std::mem::size_of::<in6_addr>() as libc::c_ulong
            {
                let fresh6 = p;
                p = p.offset(1);
                *fresh6 = (*(*rr).data.AAAA.addr).__in6_u.__u6_addr8[i as usize];
                i += 1;
                i;
            }
        }
        12 => {
            label = if !((*rr).data.PTR.name).is_null() {
                (*rr).data.PTR.name
            } else {
                (*(*rr).data.PTR.entry).name
            };
            p = p
                .offset(
                    mdns_encode_name(
                        pkt_buf,
                        pkt_len,
                        p.offset_from(pkt_buf) as libc::c_long as size_t,
                        label,
                        comp,
                    ) as isize,
                );
        }
        16 => {
            txt_rec = &mut (*rr).data.TXT;
            while !txt_rec.is_null() {
                let mut len: libc::c_int = *((*txt_rec).txt)
                    .offset(0 as libc::c_int as isize) as libc::c_int + 1 as libc::c_int;
                strncpy(
                    p as *mut libc::c_char,
                    (*txt_rec).txt as *mut libc::c_char,
                    len as libc::c_ulong,
                );
                p = p.offset(len as isize);
                txt_rec = (*txt_rec).next;
            }
        }
        33 => {
            p = mdns_write_u16(p, (*rr).data.SRV.priority);
            p = mdns_write_u16(p, (*rr).data.SRV.weight);
            p = mdns_write_u16(p, (*rr).data.SRV.port);
            p = p
                .offset(
                    mdns_encode_name(
                        pkt_buf,
                        pkt_len,
                        p.offset_from(pkt_buf) as libc::c_long as size_t,
                        (*rr).data.SRV.target,
                        comp,
                    ) as isize,
                );
        }
        47 => {
            p = p
                .offset(
                    mdns_encode_name(
                        pkt_buf,
                        pkt_len,
                        p.offset_from(pkt_buf) as libc::c_long as size_t,
                        (*rr).name,
                        comp,
                    ) as isize,
                );
            let fresh7 = p;
            p = p.offset(1);
            *fresh7 = 0 as libc::c_int as uint8_t;
            let fresh8 = p;
            p = p.offset(1);
            *fresh8 = ::std::mem::size_of::<[uint8_t; 5]>() as libc::c_ulong as uint8_t;
            i = 0 as libc::c_int;
            while (i as libc::c_ulong)
                < ::std::mem::size_of::<[uint8_t; 5]>() as libc::c_ulong
            {
                let fresh9 = p;
                p = p.offset(1);
                *fresh9 = (*rr).data.NSEC.bitmap[i as usize];
                i += 1;
                i;
            }
        }
        _ => {
            debug(
                1 as libc::c_int,
                b"unhandled rr type 0x%02x\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                (*rr).type_0 as libc::c_uint,
            );
        }
    }
    l = p.offset_from(p_data) as libc::c_long as size_t;
    mdns_write_u16(
        p
            .offset(-(l as isize))
            .offset(-(::std::mem::size_of::<uint16_t>() as libc::c_ulong as isize)),
        l as uint16_t,
    );
    return (p.offset_from(pkt_buf) as libc::c_long as libc::c_ulong).wrapping_sub(off);
}
pub unsafe extern "C" fn mdns_encode_pkt(
    mut answer: *mut mdns_pkt,
    mut pkt_buf: *mut uint8_t,
    mut pkt_len: size_t,
) -> size_t {
    let mut comp: *mut name_comp = 0 as *mut name_comp;
    let mut p: *mut uint8_t = pkt_buf;
    let mut off: size_t = 0;
    let mut i: libc::c_int = 0;
    if !answer.is_null() {} else {
        __assert_fail(
            b"answer != NULL\0" as *const u8 as *const libc::c_char,
            b"tinysvcmdns.c\0" as *const u8 as *const libc::c_char,
            957 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 61],
                &[libc::c_char; 61],
            >(b"size_t mdns_encode_pkt(struct mdns_pkt *, uint8_t *, size_t)\0"))
                .as_ptr(),
        );
    }
    'c_5754: {
        if !answer.is_null() {} else {
            __assert_fail(
                b"answer != NULL\0" as *const u8 as *const libc::c_char,
                b"tinysvcmdns.c\0" as *const u8 as *const libc::c_char,
                957 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 61],
                    &[libc::c_char; 61],
                >(b"size_t mdns_encode_pkt(struct mdns_pkt *, uint8_t *, size_t)\0"))
                    .as_ptr(),
            );
        }
    };
    if pkt_len >= 12 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"pkt_len >= 12\0" as *const u8 as *const libc::c_char,
            b"tinysvcmdns.c\0" as *const u8 as *const libc::c_char,
            958 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 61],
                &[libc::c_char; 61],
            >(b"size_t mdns_encode_pkt(struct mdns_pkt *, uint8_t *, size_t)\0"))
                .as_ptr(),
        );
    }
    'c_5716: {
        if pkt_len >= 12 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"pkt_len >= 12\0" as *const u8 as *const libc::c_char,
                b"tinysvcmdns.c\0" as *const u8 as *const libc::c_char,
                958 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 61],
                    &[libc::c_char; 61],
                >(b"size_t mdns_encode_pkt(struct mdns_pkt *, uint8_t *, size_t)\0"))
                    .as_ptr(),
            );
        }
    };
    if p.is_null() {
        return -(1 as libc::c_int) as size_t;
    }
    if (*answer).num_qn as libc::c_int == 0 as libc::c_int {} else {
        __assert_fail(
            b"answer->num_qn == 0\0" as *const u8 as *const libc::c_char,
            b"tinysvcmdns.c\0" as *const u8 as *const libc::c_char,
            964 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 61],
                &[libc::c_char; 61],
            >(b"size_t mdns_encode_pkt(struct mdns_pkt *, uint8_t *, size_t)\0"))
                .as_ptr(),
        );
    }
    'c_5661: {
        if (*answer).num_qn as libc::c_int == 0 as libc::c_int {} else {
            __assert_fail(
                b"answer->num_qn == 0\0" as *const u8 as *const libc::c_char,
                b"tinysvcmdns.c\0" as *const u8 as *const libc::c_char,
                964 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 61],
                    &[libc::c_char; 61],
                >(b"size_t mdns_encode_pkt(struct mdns_pkt *, uint8_t *, size_t)\0"))
                    .as_ptr(),
            );
        }
    };
    p = mdns_write_u16(p, (*answer).id);
    p = mdns_write_u16(p, (*answer).flags);
    p = mdns_write_u16(p, (*answer).num_qn);
    p = mdns_write_u16(p, (*answer).num_ans_rr);
    p = mdns_write_u16(p, (*answer).num_auth_rr);
    p = mdns_write_u16(p, (*answer).num_add_rr);
    off = p.offset_from(pkt_buf) as libc::c_long as size_t;
    comp = malloc(::std::mem::size_of::<name_comp>() as libc::c_ulong) as *mut name_comp;
    if comp.is_null() {
        return -(1 as libc::c_int) as size_t;
    }
    memset(
        comp as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<name_comp>() as libc::c_ulong,
    );
    (*comp).label = b"\0" as *const u8 as *const libc::c_char as *mut uint8_t;
    (*comp).pos = 0 as libc::c_int as size_t;
    let mut rr_set: [*mut rr_list; 3] = [
        (*answer).rr_ans,
        (*answer).rr_auth,
        (*answer).rr_add,
    ];
    i = 0 as libc::c_int;
    while (i as libc::c_ulong)
        < (::std::mem::size_of::<[*mut rr_list; 3]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<*mut rr_list>() as libc::c_ulong)
    {
        let mut rr: *mut rr_list = rr_set[i as usize];
        while !rr.is_null() {
            let mut l: size_t = mdns_encode_rr(pkt_buf, pkt_len, off, (*rr).e, comp);
            off = (off as libc::c_ulong).wrapping_add(l) as size_t as size_t;
            if off >= pkt_len {
                debug(
                    1 as libc::c_int,
                    b"packet buffer too small\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
                return -(1 as libc::c_int) as size_t;
            }
            rr = (*rr).next;
        }
        i += 1;
        i;
    }
    while !comp.is_null() {
        let mut c: *mut name_comp = (*comp).next;
        free(comp as *mut libc::c_void);
        comp = c;
    }
    return off;
}
unsafe extern "C" fn create_recv_sock() -> libc::c_int {
    let mut sd: libc::c_int = socket(
        2 as libc::c_int,
        SOCK_DGRAM as libc::c_int,
        0 as libc::c_int,
    );
    if sd < 0 as libc::c_int {
        match 3 as libc::c_int {
            3 => {
                warn(
                    b"recv socket(): %m\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
            }
            _ => {
                debug(
                    1 as libc::c_int,
                    b"recv socket(): %m\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
            }
        }
        return sd;
    }
    let mut r: libc::c_int = -(1 as libc::c_int);
    let mut on: libc::c_int = 1 as libc::c_int;
    r = setsockopt(
        sd,
        1 as libc::c_int,
        2 as libc::c_int,
        &mut on as *mut libc::c_int as *mut libc::c_char as *const libc::c_void,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t,
    );
    if r < 0 as libc::c_int {
        match 3 as libc::c_int {
            3 => {
                warn(
                    b"recv setsockopt(SO_REUSEADDR): %m\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                );
            }
            _ => {
                debug(
                    1 as libc::c_int,
                    b"recv setsockopt(SO_REUSEADDR): %m\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                );
            }
        }
        return r;
    }
    let mut serveraddr: sockaddr_in = sockaddr_in {
        sin_family: 0,
        sin_port: 0,
        sin_addr: in_addr { s_addr: 0 },
        sin_zero: [0; 8],
    };
    memset(
        &mut serveraddr as *mut sockaddr_in as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong,
    );
    serveraddr.sin_family = 2 as libc::c_int as sa_family_t;
    serveraddr.sin_port = __bswap_16(5353 as libc::c_int as __uint16_t);
    serveraddr.sin_addr.s_addr = __bswap_32(0 as libc::c_int as in_addr_t);
    r = bind(
        sd,
        &mut serveraddr as *mut sockaddr_in as *mut sockaddr,
        ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as socklen_t,
    );
    if r < 0 as libc::c_int {
        match 3 as libc::c_int {
            3 => {
                warn(
                    b"recv bind(): %m\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
            }
            _ => {
                debug(
                    1 as libc::c_int,
                    b"recv bind(): %m\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
            }
        }
    }
    let mut mreq: ip_mreq = ip_mreq {
        imr_multiaddr: in_addr { s_addr: 0 },
        imr_interface: in_addr { s_addr: 0 },
    };
    memset(
        &mut mreq as *mut ip_mreq as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<ip_mreq>() as libc::c_ulong,
    );
    mreq.imr_interface.s_addr = __bswap_32(0 as libc::c_int as in_addr_t);
    mreq
        .imr_multiaddr
        .s_addr = inet_addr(b"224.0.0.251\0" as *const u8 as *const libc::c_char);
    r = setsockopt(
        sd,
        IPPROTO_IP as libc::c_int,
        35 as libc::c_int,
        &mut mreq as *mut ip_mreq as *mut libc::c_char as *const libc::c_void,
        ::std::mem::size_of::<ip_mreq>() as libc::c_ulong as socklen_t,
    );
    if r < 0 as libc::c_int {
        match 3 as libc::c_int {
            3 => {
                warn(
                    b"recv setsockopt(IP_ADD_MEMBERSHIP): %m\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                );
            }
            _ => {
                debug(
                    1 as libc::c_int,
                    b"recv setsockopt(IP_ADD_MEMBERSHIP): %m\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                );
            }
        }
        return r;
    }
    r = setsockopt(
        sd,
        IPPROTO_IP as libc::c_int,
        34 as libc::c_int,
        &mut on as *mut libc::c_int as *mut libc::c_char as *const libc::c_void,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t,
    );
    if r < 0 as libc::c_int {
        match 3 as libc::c_int {
            3 => {
                warn(
                    b"recv setsockopt(IP_MULTICAST_LOOP): %m\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                );
            }
            _ => {
                debug(
                    1 as libc::c_int,
                    b"recv setsockopt(IP_MULTICAST_LOOP): %m\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                );
            }
        }
        return r;
    }
    r = setsockopt(
        sd,
        0 as libc::c_int,
        8 as libc::c_int,
        &mut on as *mut libc::c_int as *mut libc::c_char as *const libc::c_void,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t,
    );
    if r < 0 as libc::c_int {
        match 3 as libc::c_int {
            3 => {
                warn(
                    b"recv setsockopt(IP_PKTINFO): %m\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                );
            }
            _ => {
                debug(
                    1 as libc::c_int,
                    b"recv setsockopt(IP_PKTINFO): %m\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                );
            }
        }
        return r;
    }
    return sd;
}
unsafe extern "C" fn send_packet(
    mut fd: libc::c_int,
    mut data: *const libc::c_void,
    mut len: size_t,
) -> ssize_t {
    static mut toaddr: sockaddr_in = sockaddr_in {
        sin_family: 0,
        sin_port: 0,
        sin_addr: in_addr { s_addr: 0 },
        sin_zero: [0; 8],
    };
    if toaddr.sin_family as libc::c_int != 2 as libc::c_int {
        memset(
            &mut toaddr as *mut sockaddr_in as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong,
        );
        toaddr.sin_family = 2 as libc::c_int as sa_family_t;
        toaddr.sin_port = __bswap_16(5353 as libc::c_int as __uint16_t);
        toaddr
            .sin_addr
            .s_addr = inet_addr(b"224.0.0.251\0" as *const u8 as *const libc::c_char);
    }
    return sendto(
        fd,
        data,
        len,
        0 as libc::c_int,
        &mut toaddr as *mut sockaddr_in as *mut sockaddr,
        ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as socklen_t,
    );
}
unsafe extern "C" fn populate_answers(
    mut svr: *mut mdnsd,
    mut rr_head: *mut *mut rr_list,
    mut name: *mut uint8_t,
    mut type_0: rr_type,
) -> libc::c_int {
    let mut num_ans: libc::c_int = 0 as libc::c_int;
    pthread_mutex_lock(&mut (*svr).data_lock);
    let mut ans_grp: *mut rr_group = rr_group_find((*svr).group, name);
    if ans_grp.is_null() {
        pthread_mutex_unlock(&mut (*svr).data_lock);
        return num_ans;
    }
    let mut n: *mut rr_list = (*ans_grp).rr;
    while !n.is_null() {
        if !(type_0 as libc::c_uint == RR_ANY as libc::c_int as libc::c_uint
            && (*(*n).e).type_0 as libc::c_uint
                == RR_NSEC as libc::c_int as libc::c_uint)
        {
            if (type_0 as libc::c_uint == (*(*n).e).type_0 as libc::c_uint
                || type_0 as libc::c_uint == RR_ANY as libc::c_int as libc::c_uint)
                && cmp_nlabel(name, (*(*n).e).name) == 0 as libc::c_int
            {
                num_ans += rr_list_append(rr_head, (*n).e);
            }
        }
        n = (*n).next;
    }
    pthread_mutex_unlock(&mut (*svr).data_lock);
    return num_ans;
}
unsafe extern "C" fn add_related_rr(
    mut svr: *mut mdnsd,
    mut list: *mut rr_list,
    mut reply: *mut mdns_pkt,
) {
    while !list.is_null() {
        let mut ans: *mut rr_entry = (*list).e;
        match (*ans).type_0 as libc::c_uint {
            12 => {
                (*reply)
                    .num_add_rr = ((*reply).num_add_rr as libc::c_int
                    + populate_answers(
                        svr,
                        &mut (*reply).rr_add,
                        if !((*ans).data.PTR.name).is_null() {
                            (*ans).data.PTR.name
                        } else {
                            (*(*ans).data.PTR.entry).name
                        },
                        RR_ANY,
                    )) as uint16_t;
            }
            33 => {
                (*reply)
                    .num_add_rr = ((*reply).num_add_rr as libc::c_int
                    + populate_answers(
                        svr,
                        &mut (*reply).rr_add,
                        (*ans).data.SRV.target,
                        RR_ANY,
                    )) as uint16_t;
                (*reply)
                    .num_add_rr = ((*reply).num_add_rr as libc::c_int
                    + populate_answers(svr, &mut (*reply).rr_add, (*ans).name, RR_TXT))
                    as uint16_t;
            }
            1 | 28 => {
                (*reply)
                    .num_add_rr = ((*reply).num_add_rr as libc::c_int
                    + populate_answers(svr, &mut (*reply).rr_add, (*ans).name, RR_NSEC))
                    as uint16_t;
            }
            _ => {}
        }
        list = (*list).next;
    }
}
unsafe extern "C" fn announce_srv(
    mut svr: *mut mdnsd,
    mut reply: *mut mdns_pkt,
    mut name: *mut uint8_t,
) {
    mdns_init_reply(reply, 0 as libc::c_int as uint16_t);
    (*reply)
        .num_ans_rr = ((*reply).num_ans_rr as libc::c_int
        + populate_answers(svr, &mut (*reply).rr_ans, name, RR_PTR)) as uint16_t;
    (*reply)
        .num_ans_rr = ((*reply).num_ans_rr as libc::c_int
        + populate_answers(
            svr,
            &mut (*reply).rr_ans,
            b"\t_services\x07_dns-sd\x04_udp\x05local\0" as *const u8
                as *const libc::c_char as *mut uint8_t,
            RR_PTR,
        )) as uint16_t;
    add_related_rr(svr, (*reply).rr_ans, reply);
    add_related_rr(svr, (*reply).rr_add, reply);
}
unsafe extern "C" fn process_mdns_pkt(
    mut svr: *mut mdnsd,
    mut pkt: *mut mdns_pkt,
    mut reply: *mut mdns_pkt,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    if !pkt.is_null() {} else {
        __assert_fail(
            b"pkt != NULL\0" as *const u8 as *const libc::c_char,
            b"tinysvcmdns.c\0" as *const u8 as *const libc::c_char,
            1237 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 75],
                &[libc::c_char; 75],
            >(
                b"int process_mdns_pkt(struct mdnsd *, struct mdns_pkt *, struct mdns_pkt *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_8959: {
        if !pkt.is_null() {} else {
            __assert_fail(
                b"pkt != NULL\0" as *const u8 as *const libc::c_char,
                b"tinysvcmdns.c\0" as *const u8 as *const libc::c_char,
                1237 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 75],
                    &[libc::c_char; 75],
                >(
                    b"int process_mdns_pkt(struct mdnsd *, struct mdns_pkt *, struct mdns_pkt *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if (*pkt).flags as libc::c_int & (1 as libc::c_int) << 15 as libc::c_int
        == 0 as libc::c_int
        && (*pkt).flags as libc::c_int >> 11 as libc::c_int & 0xf as libc::c_int
            == 0 as libc::c_int
    {
        mdns_init_reply(reply, (*pkt).id);
        debug(
            1 as libc::c_int,
            b"flags = %04x, qn = %d, ans = %d, add = %d\n\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            (*pkt).flags as libc::c_int,
            (*pkt).num_qn as libc::c_int,
            (*pkt).num_ans_rr as libc::c_int,
            (*pkt).num_add_rr as libc::c_int,
        );
        let mut qnl: *mut rr_list = (*pkt).rr_qn;
        i = 0 as libc::c_int;
        while i < (*pkt).num_qn as libc::c_int {
            let mut qn: *mut rr_entry = (*qnl).e;
            let mut num_ans_added: libc::c_int = 0 as libc::c_int;
            let mut namestr: *mut libc::c_char = nlabel_to_str((*qn).name);
            debug(
                1 as libc::c_int,
                b"qn #%d: type %s (%02x) %s - \0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                i,
                rr_get_type_name((*qn).type_0),
                (*qn).type_0 as libc::c_uint,
                namestr,
            );
            free(namestr as *mut libc::c_void);
            if (*qn).unicast_query != 0 {
                debug(
                    1 as libc::c_int,
                    b"skipping unicast query\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
            } else {
                num_ans_added = populate_answers(
                    svr,
                    &mut (*reply).rr_ans,
                    (*qn).name,
                    (*qn).type_0,
                );
                (*reply)
                    .num_ans_rr = ((*reply).num_ans_rr as libc::c_int + num_ans_added)
                    as uint16_t;
                debug(
                    1 as libc::c_int,
                    b"added %d answers\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    num_ans_added,
                );
            }
            i += 1;
            i;
            qnl = (*qnl).next;
        }
        let mut ans: *mut rr_list = 0 as *mut rr_list;
        let mut prev_ans: *mut rr_list = 0 as *mut rr_list;
        ans = (*reply).rr_ans;
        while !ans.is_null() {
            let mut next_ans: *mut rr_list = (*ans).next;
            let mut known_ans: *mut rr_entry = rr_entry_match((*pkt).rr_ans, (*ans).e);
            if !known_ans.is_null()
                && (*known_ans).ttl
                    >= ((*(*ans).e).ttl).wrapping_div(2 as libc::c_int as libc::c_uint)
            {
                let mut namestr_0: *mut libc::c_char = nlabel_to_str((*(*ans).e).name);
                debug(
                    1 as libc::c_int,
                    b"removing answer for %s\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    namestr_0,
                );
                free(namestr_0 as *mut libc::c_void);
                if prev_ans.is_null() {
                    (*reply).rr_ans = (*ans).next;
                } else {
                    (*prev_ans).next = (*ans).next;
                }
                free(ans as *mut libc::c_void);
                ans = prev_ans;
                (*reply).num_ans_rr = ((*reply).num_ans_rr).wrapping_sub(1);
                (*reply).num_ans_rr;
            }
            prev_ans = ans;
            ans = next_ans;
        }
        add_related_rr(svr, (*reply).rr_ans, reply);
        add_related_rr(svr, (*reply).rr_add, reply);
        debug(
            1 as libc::c_int,
            b"\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        return (*reply).num_ans_rr as libc::c_int;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn create_pipe(mut handles: *mut libc::c_int) -> libc::c_int {
    return pipe(handles);
}
pub unsafe extern "C" fn read_pipe(
    mut s: libc::c_int,
    mut buf: *mut libc::c_char,
    mut len: libc::c_int,
) -> libc::c_int {
    return read(s, buf as *mut libc::c_void, len as size_t) as libc::c_int;
}
pub unsafe extern "C" fn write_pipe(
    mut s: libc::c_int,
    mut buf: *mut libc::c_char,
    mut len: libc::c_int,
) -> libc::c_int {
    return write(s, buf as *const libc::c_void, len as size_t) as libc::c_int;
}
pub unsafe extern "C" fn close_pipe(mut s: libc::c_int) -> libc::c_int {
    return close(s);
}
unsafe extern "C" fn main_loop(mut svr: *mut mdnsd) {
    let mut sockfd_set: fd_set = fd_set { __fds_bits: [0; 16] };
    let mut max_fd: libc::c_int = (*svr).sockfd;
    let mut notify_buf: [libc::c_char; 2] = [0; 2];
    let mut pkt_buffer: *mut libc::c_void = malloc(
        65536 as libc::c_int as libc::c_ulong,
    );
    if (*svr).notify_pipe[0 as libc::c_int as usize] > max_fd {
        max_fd = (*svr).notify_pipe[0 as libc::c_int as usize];
    }
    let mut mdns_reply: *mut mdns_pkt = malloc(
        ::std::mem::size_of::<mdns_pkt>() as libc::c_ulong,
    ) as *mut mdns_pkt;
    memset(
        mdns_reply as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<mdns_pkt>() as libc::c_ulong,
    );
    while (*svr).stop_flag == 0 {
        let mut __d0: libc::c_int = 0;
        let mut __d1: libc::c_int = 0;
        let fresh10 = &mut __d0;
        let fresh11;
        let fresh12 = (::std::mem::size_of::<fd_set>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<__fd_mask>() as libc::c_ulong);
        let fresh13 = &mut __d1;
        let fresh14;
        let fresh15 = &mut *(sockfd_set.__fds_bits)
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize) as *mut __fd_mask;
        asm!(
            "cld; rep; stosq", inlateout("cx")
            c2rust_asm_casts::AsmCast::cast_in(fresh10, fresh12) => fresh11,
            inlateout("di") c2rust_asm_casts::AsmCast::cast_in(fresh13, fresh15) =>
            fresh14, inlateout("ax") 0 as libc::c_int => _, options(preserves_flags,
            att_syntax)
        );
        c2rust_asm_casts::AsmCast::cast_out(fresh10, fresh12, fresh11);
        c2rust_asm_casts::AsmCast::cast_out(fresh13, fresh15, fresh14);
        sockfd_set
            .__fds_bits[((*svr).sockfd
            / (8 as libc::c_int
                * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
            as usize]
            |= ((1 as libc::c_ulong)
                << (*svr).sockfd
                    % (8 as libc::c_int
                        * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                            as libc::c_int)) as __fd_mask;
        sockfd_set
            .__fds_bits[((*svr).notify_pipe[0 as libc::c_int as usize]
            / (8 as libc::c_int
                * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
            as usize]
            |= ((1 as libc::c_ulong)
                << (*svr).notify_pipe[0 as libc::c_int as usize]
                    % (8 as libc::c_int
                        * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                            as libc::c_int)) as __fd_mask;
        select(
            max_fd + 1 as libc::c_int,
            &mut sockfd_set,
            0 as *mut fd_set,
            0 as *mut fd_set,
            0 as *mut timeval,
        );
        if sockfd_set
            .__fds_bits[((*svr).notify_pipe[0 as libc::c_int as usize]
            / (8 as libc::c_int
                * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
            as usize]
            & ((1 as libc::c_ulong)
                << (*svr).notify_pipe[0 as libc::c_int as usize]
                    % (8 as libc::c_int
                        * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                            as libc::c_int)) as __fd_mask
            != 0 as libc::c_int as libc::c_long
        {
            read_pipe(
                (*svr).notify_pipe[0 as libc::c_int as usize],
                &mut notify_buf as *mut [libc::c_char; 2] as *mut libc::c_char,
                1 as libc::c_int,
            );
        } else if sockfd_set
            .__fds_bits[((*svr).sockfd
            / (8 as libc::c_int
                * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
            as usize]
            & ((1 as libc::c_ulong)
                << (*svr).sockfd
                    % (8 as libc::c_int
                        * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                            as libc::c_int)) as __fd_mask
            != 0 as libc::c_int as libc::c_long
        {
            let mut fromaddr: sockaddr_in = sockaddr_in {
                sin_family: 0,
                sin_port: 0,
                sin_addr: in_addr { s_addr: 0 },
                sin_zero: [0; 8],
            };
            let mut sockaddr_size: socklen_t = ::std::mem::size_of::<sockaddr_in>()
                as libc::c_ulong as socklen_t;
            let mut recvsize: ssize_t = recvfrom(
                (*svr).sockfd,
                pkt_buffer,
                65536 as libc::c_int as size_t,
                0 as libc::c_int,
                &mut fromaddr as *mut sockaddr_in as *mut sockaddr,
                &mut sockaddr_size,
            );
            if recvsize < 0 as libc::c_int as libc::c_long {
                match 3 as libc::c_int {
                    3 => {
                        warn(
                            b"recv(): %m\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                        );
                    }
                    _ => {
                        debug(
                            1 as libc::c_int,
                            b"recv(): %m\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                        );
                    }
                }
            }
            debug(
                1 as libc::c_int,
                b"data from=%s size=%ld\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                inet_ntoa(fromaddr.sin_addr),
                recvsize,
            );
            let mut mdns: *mut mdns_pkt = mdns_parse_pkt(
                pkt_buffer as *mut uint8_t,
                recvsize as size_t,
            );
            if !mdns.is_null() {
                if process_mdns_pkt(svr, mdns, mdns_reply) != 0 {
                    let mut replylen: size_t = mdns_encode_pkt(
                        mdns_reply,
                        pkt_buffer as *mut uint8_t,
                        65536 as libc::c_int as size_t,
                    );
                    send_packet((*svr).sockfd, pkt_buffer, replylen);
                } else if (*mdns).num_qn as libc::c_int == 0 as libc::c_int {
                    debug(
                        1 as libc::c_int,
                        b"(no questions in packet)\n\n\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                    );
                }
                mdns_pkt_destroy(mdns);
            }
        }
        loop {
            let mut ann_e: *mut rr_entry = 0 as *mut rr_entry;
            pthread_mutex_lock(&mut (*svr).data_lock);
            if !((*svr).announce).is_null() {
                ann_e = rr_list_remove(&mut (*svr).announce, (*(*svr).announce).e);
            }
            pthread_mutex_unlock(&mut (*svr).data_lock);
            if ann_e.is_null() {
                break;
            }
            let mut namestr: *mut libc::c_char = nlabel_to_str((*ann_e).name);
            debug(
                1 as libc::c_int,
                b"sending announce for %s\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                namestr,
            );
            free(namestr as *mut libc::c_void);
            announce_srv(svr, mdns_reply, (*ann_e).name);
            if (*mdns_reply).num_ans_rr as libc::c_int > 0 as libc::c_int {
                let mut replylen_0: size_t = mdns_encode_pkt(
                    mdns_reply,
                    pkt_buffer as *mut uint8_t,
                    65536 as libc::c_int as size_t,
                );
                send_packet((*svr).sockfd, pkt_buffer, replylen_0);
            }
        }
    }
    mdns_init_reply(mdns_reply, 0 as libc::c_int as uint16_t);
    pthread_mutex_lock(&mut (*svr).data_lock);
    let mut svc_le: *mut rr_list = (*svr).services;
    while !svc_le.is_null() {
        (*(*svc_le).e).ttl = 0 as libc::c_int as uint32_t;
        (*mdns_reply)
            .num_ans_rr = ((*mdns_reply).num_ans_rr as libc::c_int
            + rr_list_append(&mut (*mdns_reply).rr_ans, (*svc_le).e)) as uint16_t;
        svc_le = (*svc_le).next;
    }
    pthread_mutex_unlock(&mut (*svr).data_lock);
    if (*mdns_reply).num_ans_rr as libc::c_int > 0 as libc::c_int {
        let mut replylen_1: size_t = mdns_encode_pkt(
            mdns_reply,
            pkt_buffer as *mut uint8_t,
            65536 as libc::c_int as size_t,
        );
        send_packet((*svr).sockfd, pkt_buffer, replylen_1);
    }
    mdns_init_reply(mdns_reply, 0 as libc::c_int as uint16_t);
    free(mdns_reply as *mut libc::c_void);
    free(pkt_buffer);
    close_pipe((*svr).sockfd);
    (*svr).stop_flag = 2 as libc::c_int;
}
pub unsafe extern "C" fn mdnsd_set_hostname(
    mut svr: *mut mdnsd,
    mut hostname: *const libc::c_char,
    mut ip: uint32_t,
) {
    let mut a_e: *mut rr_entry = 0 as *mut rr_entry;
    let mut nsec_e: *mut rr_entry = 0 as *mut rr_entry;
    if ((*svr).hostname).is_null() {} else {
        __assert_fail(
            b"svr->hostname == NULL\0" as *const u8 as *const libc::c_char,
            b"tinysvcmdns.c\0" as *const u8 as *const libc::c_char,
            1501 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 64],
                &[libc::c_char; 64],
            >(b"void mdnsd_set_hostname(struct mdnsd *, const char *, uint32_t)\0"))
                .as_ptr(),
        );
    }
    'c_10253: {
        if ((*svr).hostname).is_null() {} else {
            __assert_fail(
                b"svr->hostname == NULL\0" as *const u8 as *const libc::c_char,
                b"tinysvcmdns.c\0" as *const u8 as *const libc::c_char,
                1501 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 64],
                    &[libc::c_char; 64],
                >(b"void mdnsd_set_hostname(struct mdnsd *, const char *, uint32_t)\0"))
                    .as_ptr(),
            );
        }
    };
    a_e = rr_create_a(create_nlabel(hostname), ip);
    nsec_e = rr_create(create_nlabel(hostname), RR_NSEC);
    rr_set_nsec(nsec_e, RR_A);
    pthread_mutex_lock(&mut (*svr).data_lock);
    (*svr).hostname = create_nlabel(hostname);
    rr_group_add(&mut (*svr).group, a_e);
    rr_group_add(&mut (*svr).group, nsec_e);
    pthread_mutex_unlock(&mut (*svr).data_lock);
}
pub unsafe extern "C" fn mdnsd_set_hostname_v6(
    mut svr: *mut mdnsd,
    mut hostname: *const libc::c_char,
    mut addr: *mut in6_addr,
) {
    let mut aaaa_e: *mut rr_entry = 0 as *mut rr_entry;
    let mut nsec_e: *mut rr_entry = 0 as *mut rr_entry;
    if ((*svr).hostname).is_null() {} else {
        __assert_fail(
            b"svr->hostname == NULL\0" as *const u8 as *const libc::c_char,
            b"tinysvcmdns.c\0" as *const u8 as *const libc::c_char,
            1522 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 76],
                &[libc::c_char; 76],
            >(
                b"void mdnsd_set_hostname_v6(struct mdnsd *, const char *, struct in6_addr *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_10400: {
        if ((*svr).hostname).is_null() {} else {
            __assert_fail(
                b"svr->hostname == NULL\0" as *const u8 as *const libc::c_char,
                b"tinysvcmdns.c\0" as *const u8 as *const libc::c_char,
                1522 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 76],
                    &[libc::c_char; 76],
                >(
                    b"void mdnsd_set_hostname_v6(struct mdnsd *, const char *, struct in6_addr *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    aaaa_e = rr_create_aaaa(create_nlabel(hostname), addr);
    nsec_e = rr_create(create_nlabel(hostname), RR_NSEC);
    rr_set_nsec(nsec_e, RR_AAAA);
    pthread_mutex_lock(&mut (*svr).data_lock);
    (*svr).hostname = create_nlabel(hostname);
    rr_group_add(&mut (*svr).group, aaaa_e);
    rr_group_add(&mut (*svr).group, nsec_e);
    pthread_mutex_unlock(&mut (*svr).data_lock);
}
pub unsafe extern "C" fn mdnsd_add_rr(mut svr: *mut mdnsd, mut rr: *mut rr_entry) {
    pthread_mutex_lock(&mut (*svr).data_lock);
    rr_group_add(&mut (*svr).group, rr);
    pthread_mutex_unlock(&mut (*svr).data_lock);
}
pub unsafe extern "C" fn mdnsd_register_svc(
    mut svr: *mut mdnsd,
    mut instance_name: *const libc::c_char,
    mut type_0: *const libc::c_char,
    mut port: uint16_t,
    mut hostname: *const libc::c_char,
    mut txt: *mut *const libc::c_char,
) -> *mut mdns_service {
    let mut txt_e: *mut rr_entry = 0 as *mut rr_entry;
    let mut srv_e: *mut rr_entry = 0 as *mut rr_entry;
    let mut ptr_e: *mut rr_entry = 0 as *mut rr_entry;
    let mut bptr_e: *mut rr_entry = 0 as *mut rr_entry;
    let mut target: *mut uint8_t = 0 as *mut uint8_t;
    let mut inst_nlabel: *mut uint8_t = 0 as *mut uint8_t;
    let mut type_nlabel: *mut uint8_t = 0 as *mut uint8_t;
    let mut nlabel: *mut uint8_t = 0 as *mut uint8_t;
    let mut service: *mut mdns_service = malloc(
        ::std::mem::size_of::<mdns_service>() as libc::c_ulong,
    ) as *mut mdns_service;
    memset(
        service as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<mdns_service>() as libc::c_ulong,
    );
    type_nlabel = create_nlabel(type_0);
    inst_nlabel = create_label(instance_name);
    nlabel = join_nlabel(inst_nlabel, type_nlabel);
    if !txt.is_null() && !(*txt).is_null() {
        txt_e = rr_create(dup_nlabel(nlabel), RR_TXT);
        rr_list_append(&mut (*service).entries, txt_e);
        while !(*txt).is_null() {
            rr_add_txt(txt_e, *txt);
            txt = txt.offset(1);
            txt;
        }
    }
    if !hostname.is_null() || !((*svr).hostname).is_null() {} else {
        __assert_fail(
            b"hostname || svr->hostname\0" as *const u8 as *const libc::c_char,
            b"tinysvcmdns.c\0" as *const u8 as *const libc::c_char,
            1569 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 123],
                &[libc::c_char; 123],
            >(
                b"struct mdns_service *mdnsd_register_svc(struct mdnsd *, const char *, const char *, uint16_t, const char *, const char **)\0",
            ))
                .as_ptr(),
        );
    }
    'c_10716: {
        if !hostname.is_null() || !((*svr).hostname).is_null() {} else {
            __assert_fail(
                b"hostname || svr->hostname\0" as *const u8 as *const libc::c_char,
                b"tinysvcmdns.c\0" as *const u8 as *const libc::c_char,
                1569 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 123],
                    &[libc::c_char; 123],
                >(
                    b"struct mdns_service *mdnsd_register_svc(struct mdnsd *, const char *, const char *, uint16_t, const char *, const char **)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    target = if !hostname.is_null() {
        create_nlabel(hostname)
    } else {
        dup_nlabel((*svr).hostname)
    };
    srv_e = rr_create_srv(dup_nlabel(nlabel), port, target);
    rr_list_append(&mut (*service).entries, srv_e);
    ptr_e = rr_create_ptr(type_nlabel, srv_e);
    bptr_e = rr_create_ptr(
        dup_nlabel(
            b"\t_services\x07_dns-sd\x04_udp\x05local\0" as *const u8
                as *const libc::c_char as *mut uint8_t,
        ),
        ptr_e,
    );
    pthread_mutex_lock(&mut (*svr).data_lock);
    if !txt_e.is_null() {
        rr_group_add(&mut (*svr).group, txt_e);
    }
    rr_group_add(&mut (*svr).group, srv_e);
    rr_group_add(&mut (*svr).group, ptr_e);
    rr_group_add(&mut (*svr).group, bptr_e);
    rr_list_append(&mut (*svr).announce, ptr_e);
    rr_list_append(&mut (*svr).services, ptr_e);
    pthread_mutex_unlock(&mut (*svr).data_lock);
    free(nlabel as *mut libc::c_void);
    free(inst_nlabel as *mut libc::c_void);
    write_pipe(
        (*svr).notify_pipe[1 as libc::c_int as usize],
        b".\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        1 as libc::c_int,
    );
    return service;
}
pub unsafe extern "C" fn mdns_service_destroy(mut srv: *mut mdns_service) {
    if !srv.is_null() {} else {
        __assert_fail(
            b"srv != NULL\0" as *const u8 as *const libc::c_char,
            b"tinysvcmdns.c\0" as *const u8 as *const libc::c_char,
            1610 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 49],
                &[libc::c_char; 49],
            >(b"void mdns_service_destroy(struct mdns_service *)\0"))
                .as_ptr(),
        );
    }
    'c_10865: {
        if !srv.is_null() {} else {
            __assert_fail(
                b"srv != NULL\0" as *const u8 as *const libc::c_char,
                b"tinysvcmdns.c\0" as *const u8 as *const libc::c_char,
                1610 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 49],
                    &[libc::c_char; 49],
                >(b"void mdns_service_destroy(struct mdns_service *)\0"))
                    .as_ptr(),
            );
        }
    };
    rr_list_destroy((*srv).entries, 0 as libc::c_int as libc::c_char);
    free(srv as *mut libc::c_void);
}
pub unsafe extern "C" fn mdnsd_start() -> *mut mdnsd {
    let mut tid: pthread_t = 0;
    let mut attr: pthread_attr_t = pthread_attr_t { __size: [0; 56] };
    let mut server: *mut mdnsd = malloc(::std::mem::size_of::<mdnsd>() as libc::c_ulong)
        as *mut mdnsd;
    memset(
        server as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<mdnsd>() as libc::c_ulong,
    );
    if create_pipe(((*server).notify_pipe).as_mut_ptr()) != 0 as libc::c_int {
        match 3 as libc::c_int {
            3 => {
                warn(
                    b"pipe(): %m\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
            }
            _ => {
                debug(
                    1 as libc::c_int,
                    b"pipe(): %m\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
            }
        }
        free(server as *mut libc::c_void);
        return 0 as *mut mdnsd;
    }
    (*server).sockfd = create_recv_sock();
    if (*server).sockfd < 0 as libc::c_int {
        match 3 as libc::c_int {
            3 => {
                warn(
                    b"unable to create recv socket\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
            }
            _ => {
                debug(
                    1 as libc::c_int,
                    b"unable to create recv socket\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
            }
        }
        free(server as *mut libc::c_void);
        return 0 as *mut mdnsd;
    }
    pthread_mutex_init(&mut (*server).data_lock, 0 as *const pthread_mutexattr_t);
    pthread_attr_init(&mut attr);
    pthread_attr_setdetachstate(&mut attr, PTHREAD_CREATE_DETACHED as libc::c_int);
    if pthread_create(
        &mut tid,
        &mut attr,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut mdnsd) -> ()>,
            Option::<unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void>,
        >(Some(main_loop as unsafe extern "C" fn(*mut mdnsd) -> ())),
        server as *mut libc::c_void,
    ) != 0 as libc::c_int
    {
        pthread_mutex_destroy(&mut (*server).data_lock);
        free(server as *mut libc::c_void);
        return 0 as *mut mdnsd;
    }
    return server;
}
pub unsafe extern "C" fn mdnsd_stop(mut s: *mut mdnsd) {
    if !s.is_null() {} else {
        __assert_fail(
            b"s != NULL\0" as *const u8 as *const libc::c_char,
            b"tinysvcmdns.c\0" as *const u8 as *const libc::c_char,
            1651 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 32],
                &[libc::c_char; 32],
            >(b"void mdnsd_stop(struct mdnsd *)\0"))
                .as_ptr(),
        );
    }
    'c_10108: {
        if !s.is_null() {} else {
            __assert_fail(
                b"s != NULL\0" as *const u8 as *const libc::c_char,
                b"tinysvcmdns.c\0" as *const u8 as *const libc::c_char,
                1651 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 32],
                    &[libc::c_char; 32],
                >(b"void mdnsd_stop(struct mdnsd *)\0"))
                    .as_ptr(),
            );
        }
    };
    let mut ts: timespec = {
        let mut init = timespec {
            tv_sec: 0 as libc::c_int as __time_t,
            tv_nsec: (500 as libc::c_int * 1000000 as libc::c_int) as __syscall_slong_t,
        };
        init
    };
    (*s).stop_flag = 1 as libc::c_int;
    write_pipe(
        (*s).notify_pipe[1 as libc::c_int as usize],
        b".\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        1 as libc::c_int,
    );
    while (*s).stop_flag != 2 as libc::c_int {
        nanosleep(&mut ts, 0 as *mut timespec);
    }
    close_pipe((*s).notify_pipe[0 as libc::c_int as usize]);
    close_pipe((*s).notify_pipe[1 as libc::c_int as usize]);
    pthread_mutex_destroy(&mut (*s).data_lock);
    rr_group_destroy((*s).group);
    rr_list_destroy((*s).announce, 0 as libc::c_int as libc::c_char);
    rr_list_destroy((*s).services, 0 as libc::c_int as libc::c_char);
    if !((*s).hostname).is_null() {
        free((*s).hostname as *mut libc::c_void);
    }
    free(s as *mut libc::c_void);
}
