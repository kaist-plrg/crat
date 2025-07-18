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
    fn socket(
        __domain: libc::c_int,
        __type: libc::c_int,
        __protocol: libc::c_int,
    ) -> libc::c_int;
    fn sendto(
        __fd: libc::c_int,
        __buf: *const libc::c_void,
        __n: size_t,
        __flags: libc::c_int,
        __addr: __CONST_SOCKADDR_ARG,
        __addr_len: socklen_t,
    ) -> ssize_t;
    fn recvfrom(
        __fd: libc::c_int,
        __buf: *mut libc::c_void,
        __n: size_t,
        __flags: libc::c_int,
        __addr: __SOCKADDR_ARG,
        __addr_len: *mut socklen_t,
    ) -> ssize_t;
    fn abort() -> !;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn ntohs(__netshort: uint16_t) -> uint16_t;
    fn htons(__hostshort: uint16_t) -> uint16_t;
    static mut proxychains_resolver: dns_lookup_flavor;
    fn at_init();
    fn at_get_host_for_ip(ip: ip_type4, readbuf: *mut libc::c_char) -> size_t;
    fn at_get_ip_for_host(host: *mut libc::c_char, len: size_t) -> ip_type4;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __ssize_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
pub type ssize_t = __ssize_t;
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
pub type uint32_t = __uint32_t;
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
pub type uint16_t = __uint16_t;
pub type uint8_t = __uint8_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub union ip_type4 {
    pub octet: [libc::c_uchar; 4],
    pub as_int: uint32_t,
}
pub type at_msgtype = libc::c_uint;
pub const ATM_EXIT: at_msgtype = 3;
pub const ATM_FAIL: at_msgtype = 2;
pub const ATM_GETNAME: at_msgtype = 1;
pub const ATM_GETIP: at_msgtype = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct at_msghdr {
    pub msgtype: libc::c_uchar,
    pub reserved: libc::c_char,
    pub datalen: libc::c_ushort,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct at_msg {
    pub h: at_msghdr,
    pub m: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub host: [libc::c_char; 260],
    pub ip: ip_type4,
}
pub type dns_lookup_flavor = libc::c_uint;
pub const DNSLF_RDNS_DAEMON: dns_lookup_flavor = 3;
pub const DNSLF_RDNS_THREAD: dns_lookup_flavor = 2;
pub const DNSLF_RDNS_START: dns_lookup_flavor = 2;
pub const DNSLF_FORKEXEC: dns_lookup_flavor = 1;
pub const DNSLF_LIBC: dns_lookup_flavor = 0;
static mut rdns_server: sockaddr_in = sockaddr_in {
    sin_family: 0,
    sin_port: 0,
    sin_addr: in_addr { s_addr: 0 },
    sin_zero: [0; 8],
};
pub unsafe extern "C" fn rdns_daemon_get_host_for_ip(
    mut ip: ip_type4,
    mut readbuf: *mut libc::c_char,
) -> size_t {
    let mut msg: at_msg = {
        let mut init = at_msg {
            h: {
                let mut init = at_msghdr {
                    msgtype: ATM_GETNAME as libc::c_int as libc::c_uchar,
                    reserved: 0,
                    datalen: htons(4 as libc::c_int as uint16_t),
                };
                init
            },
            m: C2RustUnnamed_0 { ip: ip },
        };
        init
    };
    let mut fd: libc::c_int = socket(
        2 as libc::c_int,
        SOCK_DGRAM as libc::c_int | SOCK_CLOEXEC as libc::c_int,
        0 as libc::c_int,
    );
    sendto(
        fd,
        &mut msg as *mut at_msg as *const libc::c_void,
        (::std::mem::size_of::<at_msghdr>() as libc::c_ulong)
            .wrapping_add(4 as libc::c_int as libc::c_ulong),
        0 as libc::c_int,
        __CONST_SOCKADDR_ARG {
            __sockaddr__: &mut rdns_server as *mut sockaddr_in as *mut libc::c_void
                as *const sockaddr,
        },
        ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as socklen_t,
    );
    recvfrom(
        fd,
        &mut msg as *mut at_msg as *mut libc::c_void,
        ::std::mem::size_of::<at_msg>() as libc::c_ulong,
        0 as libc::c_int,
        __SOCKADDR_ARG {
            __sockaddr__: 0 as *mut libc::c_void as *mut sockaddr,
        },
        0 as *mut socklen_t,
    );
    close(fd);
    msg.h.datalen = ntohs(msg.h.datalen);
    if msg.h.datalen == 0 || msg.h.datalen as libc::c_int > 256 as libc::c_int {
        return 0 as libc::c_int as size_t;
    }
    memcpy(
        readbuf as *mut libc::c_void,
        (msg.m.host).as_mut_ptr() as *const libc::c_void,
        msg.h.datalen as libc::c_ulong,
    );
    return (msg.h.datalen as libc::c_int - 1 as libc::c_int) as size_t;
}
unsafe extern "C" fn rdns_daemon_get_ip_for_host(
    mut host: *mut libc::c_char,
    mut len: size_t,
) -> ip_type4 {
    let mut msg: at_msg = {
        let mut init = at_msg {
            h: {
                let mut init = at_msghdr {
                    msgtype: ATM_GETIP as libc::c_int as libc::c_uchar,
                    reserved: 0,
                    datalen: 0,
                };
                init
            },
            m: C2RustUnnamed_0 { host: [0; 260] },
        };
        init
    };
    if len >= 256 as libc::c_int as libc::c_ulong {
        return ip_type4 {
            as_int: -(1 as libc::c_int) as uint32_t,
        };
    }
    memcpy(
        (msg.m.host).as_mut_ptr() as *mut libc::c_void,
        host as *const libc::c_void,
        len.wrapping_add(1 as libc::c_int as libc::c_ulong),
    );
    msg
        .h
        .datalen = htons(
        len.wrapping_add(1 as libc::c_int as libc::c_ulong) as uint16_t,
    );
    let mut fd: libc::c_int = socket(
        2 as libc::c_int,
        SOCK_DGRAM as libc::c_int | SOCK_CLOEXEC as libc::c_int,
        0 as libc::c_int,
    );
    sendto(
        fd,
        &mut msg as *mut at_msg as *const libc::c_void,
        (::std::mem::size_of::<at_msghdr>() as libc::c_ulong)
            .wrapping_add(len)
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
        0 as libc::c_int,
        __CONST_SOCKADDR_ARG {
            __sockaddr__: &mut rdns_server as *mut sockaddr_in as *mut libc::c_void
                as *const sockaddr,
        },
        ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as socklen_t,
    );
    recvfrom(
        fd,
        &mut msg as *mut at_msg as *mut libc::c_void,
        ::std::mem::size_of::<at_msg>() as libc::c_ulong,
        0 as libc::c_int,
        __SOCKADDR_ARG {
            __sockaddr__: 0 as *mut libc::c_void as *mut sockaddr,
        },
        0 as *mut socklen_t,
    );
    close(fd);
    if ntohs(msg.h.datalen) as libc::c_int != 4 as libc::c_int {
        return ip_type4 {
            as_int: -(1 as libc::c_int) as uint32_t,
        };
    }
    return msg.m.ip;
}
pub unsafe extern "C" fn rdns_resolver_string(
    mut flavor: dns_lookup_flavor,
) -> *const libc::c_char {
    static mut tab: [[libc::c_char; 7]; 4] = unsafe {
        [
            *::std::mem::transmute::<&[u8; 7], &[libc::c_char; 7]>(b"off\0\0\0\0"),
            *::std::mem::transmute::<&[u8; 7], &[libc::c_char; 7]>(b"old\0\0\0\0"),
            *::std::mem::transmute::<&[u8; 7], &[libc::c_char; 7]>(b"thread\0"),
            *::std::mem::transmute::<&[u8; 7], &[libc::c_char; 7]>(b"daemon\0"),
        ]
    };
    return (tab[flavor as usize]).as_ptr();
}
pub unsafe extern "C" fn rdns_init(mut flavor: dns_lookup_flavor) {
    static mut init_done: libc::c_int = 0 as libc::c_int;
    if init_done == 0 {
        match flavor as libc::c_uint {
            2 => {
                at_init();
            }
            3 | _ => {}
        }
    }
    init_done = 1 as libc::c_int;
}
pub unsafe extern "C" fn rdns_set_daemon(mut addr: *mut sockaddr_in) {
    rdns_server = *addr;
}
pub unsafe extern "C" fn rdns_get_host_for_ip(
    mut ip: ip_type4,
    mut readbuf: *mut libc::c_char,
) -> size_t {
    match proxychains_resolver as libc::c_uint {
        2 => return at_get_host_for_ip(ip, readbuf),
        3 => return rdns_daemon_get_host_for_ip(ip, readbuf),
        _ => {
            abort();
        }
    };
}
pub unsafe extern "C" fn rdns_get_ip_for_host(
    mut host: *mut libc::c_char,
    mut len: size_t,
) -> ip_type4 {
    match proxychains_resolver as libc::c_uint {
        2 => return at_get_ip_for_host(host, len),
        3 => return rdns_daemon_get_ip_for_host(host, len),
        _ => {
            abort();
        }
    };
}
