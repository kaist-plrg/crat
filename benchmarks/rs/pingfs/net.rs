use ::libc;
use ::c2rust_asm_casts;
use c2rust_asm_casts::AsmCastTrait;
use std::arch::asm;
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
    static mut stdout: *mut FILE;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn perror(__s: *const libc::c_char);
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
    fn recvfrom(
        __fd: libc::c_int,
        __buf: *mut libc::c_void,
        __n: size_t,
        __flags: libc::c_int,
        __addr: __SOCKADDR_ARG,
        __addr_len: *mut socklen_t,
    ) -> ssize_t;
    fn setsockopt(
        __fd: libc::c_int,
        __level: libc::c_int,
        __optname: libc::c_int,
        __optval: *const libc::c_void,
        __optlen: socklen_t,
    ) -> libc::c_int;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn icmp_parse(
        pkt: *mut icmp_packet,
        data: *mut uint8_t,
        len: libc::c_int,
    ) -> libc::c_int;
    fn icmp_send(socket_0: libc::c_int, pkt: *mut icmp_packet) -> libc::c_int;
    fn chunk_reply(
        userdata: *mut libc::c_void,
        addr: *mut sockaddr_storage,
        addrlen: size_t,
        id: uint16_t,
        seqno: uint16_t,
        data: *mut *mut uint8_t,
        len: size_t,
    );
    fn pthread_join(
        __th: pthread_t,
        __thread_return: *mut *mut libc::c_void,
    ) -> libc::c_int;
    fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option::<
            unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
        >,
        __arg: *mut libc::c_void,
    ) -> libc::c_int;
    fn nanosleep(
        __requested_time: *const timespec,
        __remaining: *mut timespec,
    ) -> libc::c_int;
    fn pthread_cancel(__th: pthread_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
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
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_storage {
    pub ss_family: sa_family_t,
    pub __ss_padding: [libc::c_char; 118],
    pub __ss_align: libc::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __SOCKADDR_ARG {
    pub __sockaddr__: *mut sockaddr,
    pub __sockaddr_at__: *mut sockaddr_at,
    pub __sockaddr_ax25__: *mut sockaddr_ax25,
    pub __sockaddr_dl__: *mut sockaddr_dl,
    pub __sockaddr_eon__: *mut sockaddr_eon,
    pub __sockaddr_in__: *mut sockaddr_in,
    pub __sockaddr_in6__: *mut sockaddr_in6,
    pub __sockaddr_inarp__: *mut sockaddr_inarp,
    pub __sockaddr_ipx__: *mut sockaddr_ipx,
    pub __sockaddr_iso__: *mut sockaddr_iso,
    pub __sockaddr_ns__: *mut sockaddr_ns,
    pub __sockaddr_un__: *mut sockaddr_un,
    pub __sockaddr_x25__: *mut sockaddr_x25,
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
pub type C2RustUnnamed_1 = libc::c_uint;
pub const IPPROTO_MH: C2RustUnnamed_1 = 135;
pub const IPPROTO_DSTOPTS: C2RustUnnamed_1 = 60;
pub const IPPROTO_NONE: C2RustUnnamed_1 = 59;
pub const IPPROTO_ICMPV6: C2RustUnnamed_1 = 58;
pub const IPPROTO_FRAGMENT: C2RustUnnamed_1 = 44;
pub const IPPROTO_ROUTING: C2RustUnnamed_1 = 43;
pub const IPPROTO_HOPOPTS: C2RustUnnamed_1 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct host {
    pub next: *mut host,
    pub sockaddr: sockaddr_storage,
    pub sockaddr_len: socklen_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct icmp6_filter {
    pub icmp6_filt: [uint32_t; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct icmp_packet {
    pub peer: sockaddr_storage,
    pub peer_len: socklen_t,
    pub type_0: icmp_type,
    pub id: uint16_t,
    pub seqno: uint16_t,
    pub payload: *mut uint8_t,
    pub payload_len: uint32_t,
}
pub type icmp_type = libc::c_uint;
pub const ICMP_REPLY: icmp_type = 1;
pub const ICMP_REQUEST: icmp_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pkt_stats {
    pub packets: libc::c_ulonglong,
    pub bytes: libc::c_ulonglong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct net_data {
    pub responder: pthread_t,
    pub status: pthread_t,
    pub stats_mutex: pthread_mutex_t,
    pub tx: pkt_stats,
    pub rx: pkt_stats,
}
pub type net_recv_fn_t = Option::<
    unsafe extern "C" fn(
        *mut libc::c_void,
        *mut sockaddr_storage,
        size_t,
        uint16_t,
        uint16_t,
        *mut *mut uint8_t,
        size_t,
    ) -> (),
>;
static mut sockv4: libc::c_int = 0;
static mut sockv6: libc::c_int = 0;
static mut netdata: net_data = net_data {
    responder: 0,
    status: 0,
    stats_mutex: pthread_mutex_t {
        __data: __pthread_mutex_s {
            __lock: 0,
            __count: 0,
            __owner: 0,
            __nusers: 0,
            __kind: 0,
            __spins: 0,
            __elision: 0,
            __list: __pthread_list_t {
                __prev: 0 as *const __pthread_internal_list
                    as *mut __pthread_internal_list,
                __next: 0 as *const __pthread_internal_list
                    as *mut __pthread_internal_list,
            },
        },
    },
    tx: pkt_stats { packets: 0, bytes: 0 },
    rx: pkt_stats { packets: 0, bytes: 0 },
};
unsafe extern "C" fn inc_stats(mut stats: *mut pkt_stats, mut packetsize: libc::c_int) {
    pthread_mutex_lock(&mut netdata.stats_mutex);
    (*stats).packets = ((*stats).packets).wrapping_add(1);
    (*stats).packets;
    (*stats)
        .bytes = ((*stats).bytes)
        .wrapping_add((packetsize + 8 as libc::c_int) as libc::c_ulonglong);
    pthread_mutex_unlock(&mut netdata.stats_mutex);
}
unsafe extern "C" fn net_inc_tx(mut packetsize: libc::c_int) {
    inc_stats(&mut netdata.tx, packetsize);
}
pub unsafe extern "C" fn net_inc_rx(mut packetsize: libc::c_int) {
    inc_stats(&mut netdata.rx, packetsize);
}
pub unsafe extern "C" fn net_open_sockets() -> libc::c_int {
    let mut rcvbuf: libc::c_int = 1024 as libc::c_int * 1024 as libc::c_int;
    sockv4 = socket(
        2 as libc::c_int,
        SOCK_RAW as libc::c_int,
        IPPROTO_ICMP as libc::c_int,
    );
    if sockv4 < 0 as libc::c_int {
        perror(b"Failed to open IPv4 socket\0" as *const u8 as *const libc::c_char);
    } else {
        let mut res: libc::c_int = setsockopt(
            sockv4,
            1 as libc::c_int,
            8 as libc::c_int,
            &mut rcvbuf as *mut libc::c_int as *const libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t,
        );
        if res < 0 as libc::c_int {
            perror(
                b"Failed to set receive buffer size on IPv4 socket\0" as *const u8
                    as *const libc::c_char,
            );
        }
    }
    sockv6 = socket(
        10 as libc::c_int,
        SOCK_RAW as libc::c_int,
        IPPROTO_ICMPV6 as libc::c_int,
    );
    if sockv6 >= 0 as libc::c_int {
        let mut filter: icmp6_filter = icmp6_filter { icmp6_filt: [0; 8] };
        let mut res_0: libc::c_int = setsockopt(
            sockv6,
            1 as libc::c_int,
            8 as libc::c_int,
            &mut rcvbuf as *mut libc::c_int as *const libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t,
        );
        if res_0 < 0 as libc::c_int {
            perror(
                b"Failed to set receive buffer size on IPv6 socket\0" as *const u8
                    as *const libc::c_char,
            );
        }
        memset(
            &mut filter as *mut icmp6_filter as *mut libc::c_void,
            0xff as libc::c_int,
            ::std::mem::size_of::<icmp6_filter>() as libc::c_ulong,
        );
        filter.icmp6_filt[(129 as libc::c_int >> 5 as libc::c_int) as usize]
            &= !((1 as libc::c_int) << (129 as libc::c_int & 31 as libc::c_int))
                as libc::c_uint;
        res_0 = setsockopt(
            sockv6,
            IPPROTO_ICMPV6 as libc::c_int,
            1 as libc::c_int,
            &mut filter as *mut icmp6_filter as *const libc::c_void,
            ::std::mem::size_of::<icmp6_filter>() as libc::c_ulong as socklen_t,
        );
        if res_0 < 0 as libc::c_int {
            perror(
                b"Failed to set ICMP filters on IPv6 socket\0" as *const u8
                    as *const libc::c_char,
            );
        }
    } else {
        perror(b"Failed to open IPv6 socket\0" as *const u8 as *const libc::c_char);
    }
    if sockv4 < 0 as libc::c_int && sockv6 < 0 as libc::c_int {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn net_send(
    mut host: *mut host,
    mut id: uint16_t,
    mut seqno: uint16_t,
    mut data: *const uint8_t,
    mut len: size_t,
) {
    let mut sock: libc::c_int = 0;
    let mut pkt: icmp_packet = icmp_packet {
        peer: sockaddr_storage {
            ss_family: 0,
            __ss_padding: [0; 118],
            __ss_align: 0,
        },
        peer_len: 0,
        type_0: ICMP_REQUEST,
        id: 0,
        seqno: 0,
        payload: 0 as *mut uint8_t,
        payload_len: 0,
    };
    memcpy(
        &mut pkt.peer as *mut sockaddr_storage as *mut libc::c_void,
        &mut (*host).sockaddr as *mut sockaddr_storage as *const libc::c_void,
        (*host).sockaddr_len as libc::c_ulong,
    );
    pkt.peer_len = (*host).sockaddr_len;
    pkt.type_0 = ICMP_REQUEST;
    pkt.id = id;
    pkt.seqno = seqno;
    pkt.payload = data as *mut uint8_t;
    pkt.payload_len = len as uint32_t;
    if pkt.peer.ss_family as libc::c_int == 2 as libc::c_int {
        sock = sockv4;
    } else {
        sock = sockv6;
    }
    if sock >= 0 as libc::c_int {
        net_inc_tx(pkt.payload_len as libc::c_int);
        if icmp_send(sock, &mut pkt) == 0 {
            perror(b"Failed sending data packet\0" as *const u8 as *const libc::c_char);
        }
    }
}
unsafe extern "C" fn handle_recv(
    mut sock: libc::c_int,
    mut recv_fn: net_recv_fn_t,
    mut recv_data: *mut libc::c_void,
) {
    let mut mypkt: icmp_packet = icmp_packet {
        peer: sockaddr_storage {
            ss_family: 0,
            __ss_padding: [0; 118],
            __ss_align: 0,
        },
        peer_len: 0,
        type_0: ICMP_REQUEST,
        id: 0,
        seqno: 0,
        payload: 0 as *mut uint8_t,
        payload_len: 0,
    };
    mypkt
        .peer_len = ::std::mem::size_of::<sockaddr_storage>() as libc::c_ulong
        as socklen_t;
    let mut buf: [uint8_t; 8192] = [0; 8192];
    let mut len: libc::c_int = 0;
    len = recvfrom(
        sock,
        buf.as_mut_ptr() as *mut libc::c_void,
        ::std::mem::size_of::<[uint8_t; 8192]>() as libc::c_ulong,
        0 as libc::c_int,
        __SOCKADDR_ARG {
            __sockaddr__: &mut mypkt.peer as *mut sockaddr_storage as *mut sockaddr,
        },
        &mut mypkt.peer_len,
    ) as libc::c_int;
    if len > 0 as libc::c_int
        && icmp_parse(&mut mypkt, buf.as_mut_ptr(), len) == 0 as libc::c_int
    {
        if mypkt.type_0 as libc::c_uint == ICMP_REPLY as libc::c_int as libc::c_uint {
            recv_fn
                .unwrap()(
                recv_data,
                &mut mypkt.peer,
                mypkt.peer_len as size_t,
                mypkt.id,
                mypkt.seqno,
                &mut mypkt.payload,
                mypkt.payload_len as size_t,
            );
        }
        free(mypkt.payload as *mut libc::c_void);
    }
}
pub unsafe extern "C" fn net_recv(
    mut tv: *mut timeval,
    mut recv_fn: net_recv_fn_t,
    mut recv_data: *mut libc::c_void,
) -> libc::c_int {
    let mut maxfd: libc::c_int = 0;
    let mut fds: fd_set = fd_set { fds_bits: [0; 16] };
    let mut i: libc::c_int = 0;
    let mut __d0: libc::c_int = 0;
    let mut __d1: libc::c_int = 0;
    let fresh0 = &mut __d0;
    let fresh1;
    let fresh2 = (::std::mem::size_of::<fd_set>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<__fd_mask>() as libc::c_ulong);
    let fresh3 = &mut __d1;
    let fresh4;
    let fresh5 = &mut *(fds.fds_bits).as_mut_ptr().offset(0 as libc::c_int as isize)
        as *mut __fd_mask;
    asm!(
        "cld; rep; stosq", inlateout("cx") c2rust_asm_casts::AsmCast::cast_in(fresh0,
        fresh2) => fresh1, inlateout("di") c2rust_asm_casts::AsmCast::cast_in(fresh3,
        fresh5) => fresh4, inlateout("ax") 0 as libc::c_int => _,
        options(preserves_flags, att_syntax)
    );
    c2rust_asm_casts::AsmCast::cast_out(fresh0, fresh2, fresh1);
    c2rust_asm_casts::AsmCast::cast_out(fresh3, fresh5, fresh4);
    if sockv4 >= 0 as libc::c_int {
        fds
            .fds_bits[(sockv4
            / (8 as libc::c_int
                * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
            as usize]
            |= ((1 as libc::c_ulong)
                << sockv4
                    % (8 as libc::c_int
                        * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                            as libc::c_int)) as __fd_mask;
    }
    if sockv6 >= 0 as libc::c_int {
        fds
            .fds_bits[(sockv6
            / (8 as libc::c_int
                * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
            as usize]
            |= ((1 as libc::c_ulong)
                << sockv6
                    % (8 as libc::c_int
                        * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                            as libc::c_int)) as __fd_mask;
    }
    maxfd = if sockv4 > sockv6 { sockv4 } else { sockv6 };
    i = select(
        maxfd + 1 as libc::c_int,
        &mut fds,
        0 as *mut fd_set,
        0 as *mut fd_set,
        tv,
    );
    if sockv4 >= 0 as libc::c_int
        && fds
            .fds_bits[(sockv4
            / (8 as libc::c_int
                * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
            as usize]
            & ((1 as libc::c_ulong)
                << sockv4
                    % (8 as libc::c_int
                        * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                            as libc::c_int)) as __fd_mask
            != 0 as libc::c_int as libc::c_long
    {
        handle_recv(sockv4, recv_fn, recv_data);
    }
    if sockv6 >= 0 as libc::c_int
        && fds
            .fds_bits[(sockv6
            / (8 as libc::c_int
                * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
            as usize]
            & ((1 as libc::c_ulong)
                << sockv6
                    % (8 as libc::c_int
                        * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                            as libc::c_int)) as __fd_mask
            != 0 as libc::c_int as libc::c_long
    {
        handle_recv(sockv6, recv_fn, recv_data);
    }
    return i;
}
unsafe extern "C" fn responder_thread(mut arg: *mut libc::c_void) -> *mut libc::c_void {
    loop {
        let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
        tv.tv_sec = 1 as libc::c_int as __time_t;
        tv.tv_usec = 0 as libc::c_int as __suseconds_t;
        net_recv(
            &mut tv,
            Some(
                chunk_reply
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut sockaddr_storage,
                        size_t,
                        uint16_t,
                        uint16_t,
                        *mut *mut uint8_t,
                        size_t,
                    ) -> (),
            ),
            0 as *mut libc::c_void,
        );
    };
}
unsafe extern "C" fn get_stats(mut rx: *mut pkt_stats, mut tx: *mut pkt_stats) {
    pthread_mutex_lock(&mut netdata.stats_mutex);
    memcpy(
        rx as *mut libc::c_void,
        &mut netdata.rx as *mut pkt_stats as *const libc::c_void,
        ::std::mem::size_of::<pkt_stats>() as libc::c_ulong,
    );
    memcpy(
        tx as *mut libc::c_void,
        &mut netdata.tx as *mut pkt_stats as *const libc::c_void,
        ::std::mem::size_of::<pkt_stats>() as libc::c_ulong,
    );
    pthread_mutex_unlock(&mut netdata.stats_mutex);
}
unsafe extern "C" fn format_bytes(
    mut bytes: libc::c_ulonglong,
    mut suffix: *mut *const libc::c_char,
) -> libc::c_float {
    let mut suffixes: [*const libc::c_char; 5] = [
        b"B\0" as *const u8 as *const libc::c_char,
        b"kB\0" as *const u8 as *const libc::c_char,
        b"MB\0" as *const u8 as *const libc::c_char,
        b"GB\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut bps: libc::c_float = bytes as libc::c_float;
    while !(suffixes[(i + 1 as libc::c_int) as usize]).is_null()
        && bps > 1300 as libc::c_int as libc::c_float
    {
        bps /= 1000.0f32;
        i += 1;
        i;
    }
    *suffix = suffixes[i as usize];
    return bps;
}
unsafe extern "C" fn diff_stats(mut new: *mut pkt_stats, mut old: *mut pkt_stats) {
    let mut diff: pkt_stats = pkt_stats { packets: 0, bytes: 0 };
    let mut bytes: libc::c_float = 0.;
    let mut byte_suffix: *const libc::c_char = 0 as *const libc::c_char;
    diff.packets = ((*new).packets).wrapping_sub((*old).packets);
    diff.bytes = ((*new).bytes).wrapping_sub((*old).bytes);
    memcpy(
        old as *mut libc::c_void,
        new as *const libc::c_void,
        ::std::mem::size_of::<pkt_stats>() as libc::c_ulong,
    );
    bytes = format_bytes(diff.bytes, &mut byte_suffix);
    printf(
        b"%6llu pkt/s, %7.01f %2s/s\0" as *const u8 as *const libc::c_char,
        diff.packets,
        bytes as libc::c_double,
        byte_suffix,
    );
}
unsafe extern "C" fn status_thread(mut arg: *mut libc::c_void) -> *mut libc::c_void {
    let status_sleep: timespec = {
        let mut init = timespec {
            tv_sec: 1 as libc::c_int as __time_t,
            tv_nsec: 0 as libc::c_int as __syscall_slong_t,
        };
        init
    };
    static mut prev_rx: pkt_stats = pkt_stats { packets: 0, bytes: 0 };
    static mut prev_tx: pkt_stats = pkt_stats { packets: 0, bytes: 0 };
    get_stats(&mut prev_rx, &mut prev_tx);
    nanosleep(&status_sleep, 0 as *mut timespec);
    loop {
        let mut rx: pkt_stats = pkt_stats { packets: 0, bytes: 0 };
        let mut tx: pkt_stats = pkt_stats { packets: 0, bytes: 0 };
        get_stats(&mut rx, &mut tx);
        printf(b"\rICMP in: \0" as *const u8 as *const libc::c_char);
        diff_stats(&mut rx, &mut prev_rx);
        printf(b"    ICMP out: \0" as *const u8 as *const libc::c_char);
        diff_stats(&mut tx, &mut prev_tx);
        fflush(stdout);
        nanosleep(&status_sleep, 0 as *mut timespec);
    };
}
pub unsafe extern "C" fn net_start() {
    if pthread_mutex_init(&mut netdata.stats_mutex, 0 as *const pthread_mutexattr_t) != 0
    {
        perror(b"Fatal, failed to create a mutex\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    pthread_create(
        &mut netdata.responder,
        0 as *const pthread_attr_t,
        Some(
            responder_thread
                as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
        ),
        0 as *mut libc::c_void,
    );
    pthread_create(
        &mut netdata.status,
        0 as *const pthread_attr_t,
        Some(
            status_thread as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
        ),
        0 as *mut libc::c_void,
    );
}
pub unsafe extern "C" fn net_stop() {
    pthread_cancel(netdata.responder);
    pthread_join(netdata.responder, 0 as *mut *mut libc::c_void);
    pthread_cancel(netdata.status);
    pthread_join(netdata.status, 0 as *mut *mut libc::c_void);
    pthread_mutex_lock(&mut netdata.stats_mutex);
    printf(
        b"\n\nTotal network resources consumed:\nin:  %10llu packets, %10llu bytes\nout: %10llu packets, %10llu bytes\n (bytes counted above IP level)\n\0"
            as *const u8 as *const libc::c_char,
        netdata.rx.packets,
        netdata.rx.bytes,
        netdata.tx.packets,
        netdata.tx.bytes,
    );
    pthread_mutex_unlock(&mut netdata.stats_mutex);
}
