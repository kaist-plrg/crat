use ::libc;
extern "C" {
    pub type sockaddr_x25;
    pub type sockaddr_ns;
    pub type sockaddr_iso;
    pub type sockaddr_ipx;
    pub type sockaddr_inarp;
    pub type sockaddr_eon;
    pub type sockaddr_dl;
    pub type sockaddr_ax25;
    pub type sockaddr_at;
    pub type htab;
    pub type upstream;
    pub type reversepath;
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn socket(
        __domain: libc::c_int,
        __type: libc::c_int,
        __protocol: libc::c_int,
    ) -> libc::c_int;
    fn bind(
        __fd: libc::c_int,
        __addr: __CONST_SOCKADDR_ARG,
        __len: socklen_t,
    ) -> libc::c_int;
    fn getsockname(
        __fd: libc::c_int,
        __addr: __SOCKADDR_ARG,
        __len: *mut socklen_t,
    ) -> libc::c_int;
    fn connect(
        __fd: libc::c_int,
        __addr: __CONST_SOCKADDR_ARG,
        __len: socklen_t,
    ) -> libc::c_int;
    fn setsockopt(
        __fd: libc::c_int,
        __level: libc::c_int,
        __optname: libc::c_int,
        __optval: *const libc::c_void,
        __optlen: socklen_t,
    ) -> libc::c_int;
    fn listen(__fd: libc::c_int, __n: libc::c_int) -> libc::c_int;
    fn ntohs(__netshort: uint16_t) -> uint16_t;
    fn getaddrinfo(
        __name: *const libc::c_char,
        __service: *const libc::c_char,
        __req: *const addrinfo,
        __pai: *mut *mut addrinfo,
    ) -> libc::c_int;
    fn freeaddrinfo(__ai: *mut addrinfo);
    fn gai_strerror(__ecode: libc::c_int) -> *const libc::c_char;
    fn getnameinfo(
        __sa: *const sockaddr,
        __salen: socklen_t,
        __host: *mut libc::c_char,
        __hostlen: socklen_t,
        __serv: *mut libc::c_char,
        __servlen: socklen_t,
        __flags: libc::c_int,
    ) -> libc::c_int;
    fn inet_ntop(
        __af: libc::c_int,
        __cp: *const libc::c_void,
        __buf: *mut libc::c_char,
        __len: socklen_t,
    ) -> *const libc::c_char;
    static mut config: *mut config_s;
    fn log_message(level: libc::c_int, fmt: *const libc::c_char, _: ...);
    fn get_ip_string(
        sa: *mut sockaddr,
        buf: *mut libc::c_char,
        len: size_t,
    ) -> *const libc::c_char;
    fn sblist_get(l: *mut sblist, item: size_t) -> *mut libc::c_void;
    fn sblist_add(l: *mut sblist, item: *mut libc::c_void) -> libc::c_int;
    fn loop_records_add(addr: *mut sockaddr_union);
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
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
pub struct sockaddr_un {
    pub sun_family: sa_family_t,
    pub sun_path: [libc::c_char; 108],
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
pub struct config_s {
    pub basicauth_list: *mut sblist,
    pub logf_name: *mut libc::c_char,
    pub syslog: libc::c_uint,
    pub port: libc::c_uint,
    pub stathost: *mut libc::c_char,
    pub quit: libc::c_uint,
    pub maxclients: libc::c_uint,
    pub user: *mut libc::c_char,
    pub group: *mut libc::c_char,
    pub listen_addrs: *mut sblist,
    pub filter: *mut libc::c_char,
    pub filter_opts: libc::c_uint,
    pub add_xtinyproxy: libc::c_uint,
    pub reversepath_list: *mut reversepath,
    pub reverseonly: libc::c_uint,
    pub reversemagic: libc::c_uint,
    pub reversebaseurl: *mut libc::c_char,
    pub upstream_list: *mut upstream,
    pub pidpath: *mut libc::c_char,
    pub idletimeout: libc::c_uint,
    pub bind_addrs: *mut sblist,
    pub bindsame: libc::c_uint,
    pub via_proxy_name: *mut libc::c_char,
    pub disable_viaheader: libc::c_uint,
    pub errorpages: *mut htab,
    pub errorpage_undef: *mut libc::c_char,
    pub statpage: *mut libc::c_char,
    pub access_list: acl_list_t,
    pub connect_ports: *mut sblist,
    pub anonymous_map: *mut htab,
    pub add_headers: *mut sblist,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sblist {
    pub itemsize: size_t,
    pub blockitems: size_t,
    pub count: size_t,
    pub capa: size_t,
    pub items: *mut libc::c_char,
}
pub type acl_list_t = *mut sblist;
#[derive(Copy, Clone)]
#[repr(C)]
pub union sockaddr_union {
    pub v4: sockaddr_in,
    pub v6: sockaddr_in6,
}
unsafe extern "C" fn get_gai_error(mut n: libc::c_int) -> *const libc::c_char {
    if n == -(11 as libc::c_int) {
        return strerror(*__errno_location())
    } else {
        return gai_strerror(n)
    };
}
unsafe extern "C" fn family_string(mut af: libc::c_int) -> *const libc::c_char {
    match af {
        0 => return b"AF_UNSPEC\0" as *const u8 as *const libc::c_char,
        2 => return b"AF_INET\0" as *const u8 as *const libc::c_char,
        10 => return b"AF_INET6\0" as *const u8 as *const libc::c_char,
        _ => {}
    }
    return b"unknown\0" as *const u8 as *const libc::c_char;
}
unsafe extern "C" fn bind_socket(
    mut sockfd: libc::c_int,
    mut addr: *const libc::c_char,
    mut family: libc::c_int,
) -> libc::c_int {
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
    let mut res: *mut addrinfo = 0 as *mut addrinfo;
    let mut ressave: *mut addrinfo = 0 as *mut addrinfo;
    let mut n: libc::c_int = 0;
    memset(
        &mut hints as *mut addrinfo as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<addrinfo>() as libc::c_ulong,
    );
    hints.ai_family = family;
    hints.ai_socktype = SOCK_STREAM as libc::c_int;
    n = getaddrinfo(addr, 0 as *const libc::c_char, &mut hints, &mut res);
    if n != 0 as libc::c_int {
        log_message(
            6 as libc::c_int,
            b"bind_socket: getaddrinfo failed for %s: %s (af: %s)\0" as *const u8
                as *const libc::c_char,
            addr,
            get_gai_error(n),
            family_string(family),
        );
        return -(1 as libc::c_int);
    }
    ressave = res;
    while !(bind(
        sockfd,
        __CONST_SOCKADDR_ARG {
            __sockaddr__: (*res).ai_addr,
        },
        (*res).ai_addrlen,
    ) == 0 as libc::c_int)
    {
        res = (*res).ai_next;
        if res.is_null() {
            break;
        }
    }
    freeaddrinfo(ressave);
    if res.is_null() {
        return -(1 as libc::c_int);
    }
    return sockfd;
}
unsafe extern "C" fn bind_socket_list(
    mut sockfd: libc::c_int,
    mut addresses: *mut sblist,
    mut family: libc::c_int,
) -> libc::c_int {
    let mut nb_addresses: size_t = (*addresses).count;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < nb_addresses {
        let mut address: *const libc::c_char = *(sblist_get(addresses, i)
            as *mut *const libc::c_char);
        if bind_socket(sockfd, address, family) >= 0 as libc::c_int {
            log_message(
                6 as libc::c_int,
                b"Bound to %s\0" as *const u8 as *const libc::c_char,
                address,
            );
            return 0 as libc::c_int;
        }
        i = i.wrapping_add(1);
        i;
    }
    return -(1 as libc::c_int);
}
pub unsafe extern "C" fn set_socket_timeout(mut fd: libc::c_int) {
    let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    tv.tv_usec = 0 as libc::c_int as __suseconds_t;
    tv.tv_sec = (*config).idletimeout as __time_t;
    setsockopt(
        fd,
        1 as libc::c_int,
        21 as libc::c_int,
        &mut tv as *mut timeval as *mut libc::c_void,
        ::std::mem::size_of::<timeval>() as libc::c_ulong as socklen_t,
    );
    tv.tv_usec = 0 as libc::c_int as __suseconds_t;
    tv.tv_sec = (*config).idletimeout as __time_t;
    setsockopt(
        fd,
        1 as libc::c_int,
        20 as libc::c_int,
        &mut tv as *mut timeval as *mut libc::c_void,
        ::std::mem::size_of::<timeval>() as libc::c_ulong as socklen_t,
    );
}
pub unsafe extern "C" fn opensock(
    mut host: *const libc::c_char,
    mut port: libc::c_int,
    mut bind_to: *const libc::c_char,
) -> libc::c_int {
    let mut sockfd: libc::c_int = 0;
    let mut n: libc::c_int = 0;
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
    let mut res: *mut addrinfo = 0 as *mut addrinfo;
    let mut ressave: *mut addrinfo = 0 as *mut addrinfo;
    let mut portstr: [libc::c_char; 6] = [0; 6];
    log_message(
        6 as libc::c_int,
        b"opensock: opening connection to %s:%d\0" as *const u8 as *const libc::c_char,
        host,
        port,
    );
    memset(
        &mut hints as *mut addrinfo as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<addrinfo>() as libc::c_ulong,
    );
    hints.ai_family = 0 as libc::c_int;
    hints.ai_socktype = SOCK_STREAM as libc::c_int;
    snprintf(
        portstr.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong,
        b"%d\0" as *const u8 as *const libc::c_char,
        port,
    );
    n = getaddrinfo(host, portstr.as_mut_ptr(), &mut hints, &mut res);
    if n != 0 as libc::c_int {
        log_message(
            3 as libc::c_int,
            b"opensock: Could not retrieve address info for %s:%d: %s\0" as *const u8
                as *const libc::c_char,
            host,
            port,
            get_gai_error(n),
        );
        return -(1 as libc::c_int);
    }
    log_message(
        6 as libc::c_int,
        b"opensock: getaddrinfo returned for %s:%d\0" as *const u8
            as *const libc::c_char,
        host,
        port,
    );
    ressave = res;
    let mut current_block_22: u64;
    loop {
        sockfd = socket((*res).ai_family, (*res).ai_socktype, (*res).ai_protocol);
        if !(sockfd < 0 as libc::c_int) {
            if !bind_to.is_null() {
                if bind_socket(sockfd, bind_to, (*res).ai_family) < 0 as libc::c_int {
                    close(sockfd);
                    current_block_22 = 11812396948646013369;
                } else {
                    current_block_22 = 2370887241019905314;
                }
            } else if !((*config).bind_addrs).is_null() {
                if bind_socket_list(sockfd, (*config).bind_addrs, (*res).ai_family)
                    < 0 as libc::c_int
                {
                    close(sockfd);
                    current_block_22 = 11812396948646013369;
                } else {
                    current_block_22 = 2370887241019905314;
                }
            } else {
                current_block_22 = 2370887241019905314;
            }
            match current_block_22 {
                11812396948646013369 => {}
                _ => {
                    set_socket_timeout(sockfd);
                    if connect(
                        sockfd,
                        __CONST_SOCKADDR_ARG {
                            __sockaddr__: (*res).ai_addr,
                        },
                        (*res).ai_addrlen,
                    ) == 0 as libc::c_int
                    {
                        let mut p: *mut sockaddr_union = (*res).ai_addr
                            as *mut libc::c_void as *mut sockaddr_union;
                        let mut u: sockaddr_union = sockaddr_union {
                            v4: sockaddr_in {
                                sin_family: 0,
                                sin_port: 0,
                                sin_addr: in_addr { s_addr: 0 },
                                sin_zero: [0; 8],
                            },
                        };
                        let mut af: libc::c_int = (*(*res).ai_addr).sa_family
                            as libc::c_int;
                        let mut dport: libc::c_uint = ntohs(
                            (if af == 2 as libc::c_int {
                                (*p).v4.sin_port as libc::c_int
                            } else {
                                (*p).v6.sin6_port as libc::c_int
                            }) as uint16_t,
                        ) as libc::c_uint;
                        let mut slen: socklen_t = ::std::mem::size_of::<sockaddr_union>()
                            as libc::c_ulong as socklen_t;
                        if dport == (*config).port {
                            getsockname(
                                sockfd,
                                __SOCKADDR_ARG {
                                    __sockaddr__: &mut u as *mut sockaddr_union
                                        as *mut libc::c_void as *mut sockaddr,
                                },
                                &mut slen,
                            );
                            loop_records_add(&mut u);
                        }
                        break;
                    } else {
                        close(sockfd);
                    }
                }
            }
        }
        res = (*res).ai_next;
        if res.is_null() {
            break;
        }
    }
    freeaddrinfo(ressave);
    if res.is_null() {
        log_message(
            3 as libc::c_int,
            b"opensock: Could not establish a connection to %s:%d\0" as *const u8
                as *const libc::c_char,
            host,
            port,
        );
        return -(1 as libc::c_int);
    }
    return sockfd;
}
pub unsafe extern "C" fn socket_nonblocking(mut sock: libc::c_int) -> libc::c_int {
    let mut flags: libc::c_int = 0;
    flags = fcntl(sock, 3 as libc::c_int, 0 as libc::c_int);
    return fcntl(sock, 4 as libc::c_int, flags | 0o4000 as libc::c_int);
}
pub unsafe extern "C" fn socket_blocking(mut sock: libc::c_int) -> libc::c_int {
    let mut flags: libc::c_int = 0;
    flags = fcntl(sock, 3 as libc::c_int, 0 as libc::c_int);
    return fcntl(sock, 4 as libc::c_int, flags & !(0o4000 as libc::c_int));
}
unsafe extern "C" fn listen_on_one_socket(mut ad: *mut addrinfo) -> libc::c_int {
    let mut listenfd: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let on: libc::c_int = 1 as libc::c_int;
    let mut numerichost: [libc::c_char; 1025] = [0; 1025];
    let mut flags: libc::c_int = 1 as libc::c_int;
    ret = getnameinfo(
        (*ad).ai_addr,
        (*ad).ai_addrlen,
        numerichost.as_mut_ptr(),
        1025 as libc::c_int as socklen_t,
        0 as *mut libc::c_char,
        0 as libc::c_int as socklen_t,
        flags,
    );
    if ret != 0 as libc::c_int {
        log_message(
            3 as libc::c_int,
            b"getnameinfo failed: %s\0" as *const u8 as *const libc::c_char,
            get_gai_error(ret),
        );
        return -(1 as libc::c_int);
    }
    log_message(
        6 as libc::c_int,
        b"trying to listen on host[%s], family[%d], socktype[%d], proto[%d]\0"
            as *const u8 as *const libc::c_char,
        numerichost.as_mut_ptr(),
        (*ad).ai_family,
        (*ad).ai_socktype,
        (*ad).ai_protocol,
    );
    listenfd = socket((*ad).ai_family, (*ad).ai_socktype, (*ad).ai_protocol);
    if listenfd == -(1 as libc::c_int) {
        log_message(
            3 as libc::c_int,
            b"socket() failed: %s\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
        return -(1 as libc::c_int);
    }
    ret = setsockopt(
        listenfd,
        1 as libc::c_int,
        2 as libc::c_int,
        &on as *const libc::c_int as *const libc::c_void,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t,
    );
    if ret != 0 as libc::c_int {
        log_message(
            3 as libc::c_int,
            b"setsockopt failed to set SO_REUSEADDR: %s\0" as *const u8
                as *const libc::c_char,
            strerror(*__errno_location()),
        );
        close(listenfd);
        return -(1 as libc::c_int);
    }
    if (*ad).ai_family == 10 as libc::c_int {
        ret = setsockopt(
            listenfd,
            IPPROTO_IPV6 as libc::c_int,
            26 as libc::c_int,
            &on as *const libc::c_int as *const libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t,
        );
        if ret != 0 as libc::c_int {
            log_message(
                3 as libc::c_int,
                b"setsockopt failed to set IPV6_V6ONLY: %s\0" as *const u8
                    as *const libc::c_char,
                strerror(*__errno_location()),
            );
            close(listenfd);
            return -(1 as libc::c_int);
        }
    }
    ret = bind(
        listenfd,
        __CONST_SOCKADDR_ARG {
            __sockaddr__: (*ad).ai_addr,
        },
        (*ad).ai_addrlen,
    );
    if ret != 0 as libc::c_int {
        log_message(
            3 as libc::c_int,
            b"bind failed: %s\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
        close(listenfd);
        return -(1 as libc::c_int);
    }
    ret = listen(listenfd, 1024 as libc::c_int);
    if ret != 0 as libc::c_int {
        log_message(
            3 as libc::c_int,
            b"listen failed: %s\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
        close(listenfd);
        return -(1 as libc::c_int);
    }
    log_message(
        6 as libc::c_int,
        b"listening on fd [%d]\0" as *const u8 as *const libc::c_char,
        listenfd,
    );
    return listenfd;
}
pub unsafe extern "C" fn listen_sock(
    mut addr: *const libc::c_char,
    mut port: uint16_t,
    mut listen_fds: *mut sblist,
) -> libc::c_int {
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
    let mut result: *mut addrinfo = 0 as *mut addrinfo;
    let mut rp: *mut addrinfo = 0 as *mut addrinfo;
    let mut portstr: [libc::c_char; 6] = [0; 6];
    let mut ret: libc::c_int = -(1 as libc::c_int);
    let mut n: libc::c_int = 0;
    log_message(
        6 as libc::c_int,
        b"listen_sock called with addr = '%s'\0" as *const u8 as *const libc::c_char,
        if addr.is_null() {
            b"(NULL)\0" as *const u8 as *const libc::c_char
        } else {
            addr
        },
    );
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
        ::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong,
        b"%d\0" as *const u8 as *const libc::c_char,
        port as libc::c_int,
    );
    n = getaddrinfo(addr, portstr.as_mut_ptr(), &mut hints, &mut result);
    if n != 0 as libc::c_int {
        log_message(
            3 as libc::c_int,
            b"Unable to getaddrinfo() for %s:%d because of %s\0" as *const u8
                as *const libc::c_char,
            addr,
            port as libc::c_int,
            get_gai_error(n),
        );
        return -(1 as libc::c_int);
    }
    rp = result;
    while !rp.is_null() {
        let mut listenfd: libc::c_int = 0;
        listenfd = listen_on_one_socket(rp);
        if !(listenfd == -(1 as libc::c_int)) {
            sblist_add(
                listen_fds,
                &mut listenfd as *mut libc::c_int as *mut libc::c_void,
            );
            ret = 0 as libc::c_int;
            if !addr.is_null() {
                break;
            }
        }
        rp = (*rp).ai_next;
    }
    if ret != 0 as libc::c_int {
        log_message(
            3 as libc::c_int,
            b"Unable to listen on any address.\0" as *const u8 as *const libc::c_char,
        );
    }
    freeaddrinfo(result);
    return ret;
}
pub unsafe extern "C" fn getsock_ip(
    mut fd: libc::c_int,
    mut ipaddr: *mut libc::c_char,
) -> libc::c_int {
    let mut name: sockaddr_storage = sockaddr_storage {
        ss_family: 0,
        __ss_padding: [0; 118],
        __ss_align: 0,
    };
    let mut namelen: socklen_t = ::std::mem::size_of::<sockaddr_storage>()
        as libc::c_ulong as socklen_t;
    if getsockname(
        fd,
        __SOCKADDR_ARG {
            __sockaddr__: &mut name as *mut sockaddr_storage as *mut sockaddr,
        },
        &mut namelen,
    ) != 0 as libc::c_int
    {
        log_message(
            3 as libc::c_int,
            b"getsock_ip: getsockname() error: %s\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
        return -(1 as libc::c_int);
    }
    if (get_ip_string(
        &mut name as *mut sockaddr_storage as *mut sockaddr,
        ipaddr,
        48 as libc::c_int as size_t,
    ))
        .is_null()
    {
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn getpeer_information(
    mut addr: *mut sockaddr_union,
    mut ipaddr: *mut libc::c_char,
    mut ipaddr_len: size_t,
) {
    let mut af: libc::c_int = (*addr).v4.sin_family as libc::c_int;
    let mut ipdata: *mut libc::c_void = if af == 2 as libc::c_int {
        &mut (*addr).v4.sin_addr as *mut in_addr as *mut libc::c_void
    } else {
        &mut (*addr).v6.sin6_addr as *mut in6_addr as *mut libc::c_void
    };
    inet_ntop(af, ipdata, ipaddr, ipaddr_len as socklen_t);
}
