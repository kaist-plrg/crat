use ::libc;
use ::c2rust_asm_casts;
use c2rust_asm_casts::AsmCastTrait;
use std::arch::asm;
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
    fn bind(
        __fd: libc::c_int,
        __addr: __CONST_SOCKADDR_ARG,
        __len: socklen_t,
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
    fn getaddrinfo(
        __name: *const libc::c_char,
        __service: *const libc::c_char,
        __req: *const addrinfo,
        __pai: *mut *mut addrinfo,
    ) -> libc::c_int;
    fn freeaddrinfo(__ai: *mut addrinfo);
    fn gai_strerror(__ecode: libc::c_int) -> *const libc::c_char;
    fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, _: ...) -> libc::c_int;
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn inet_addr(__cp: *const libc::c_char) -> in_addr_t;
    fn inet_ntoa(__in: in_addr) -> *mut libc::c_char;
    fn inet_pton(
        __af: libc::c_int,
        __cp: *const libc::c_char,
        __buf: *mut libc::c_void,
    ) -> libc::c_int;
    fn strlcpy(_: *mut libc::c_char, _: *const libc::c_char, _: size_t) -> size_t;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __caddr_t = *mut libc::c_char;
pub type __socklen_t = libc::c_uint;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ifmap {
    pub mem_start: libc::c_ulong,
    pub mem_end: libc::c_ulong,
    pub base_addr: libc::c_ushort,
    pub irq: libc::c_uchar,
    pub dma: libc::c_uchar,
    pub port: libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ifreq {
    pub ifr_ifrn: C2RustUnnamed_2,
    pub ifr_ifru: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub ifru_addr: sockaddr,
    pub ifru_dstaddr: sockaddr,
    pub ifru_broadaddr: sockaddr,
    pub ifru_netmask: sockaddr,
    pub ifru_hwaddr: sockaddr,
    pub ifru_flags: libc::c_short,
    pub ifru_ivalue: libc::c_int,
    pub ifru_mtu: libc::c_int,
    pub ifru_map: ifmap,
    pub ifru_slave: [libc::c_char; 16],
    pub ifru_newname: [libc::c_char; 16],
    pub ifru_data: __caddr_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
    pub ifrn_name: [libc::c_char; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tcp_t {
    pub fd: libc::c_int,
    pub ai_family: sa_family_t,
}
pub unsafe extern "C" fn is_ipv6_addr(mut hostname: *const libc::c_char) -> libc::c_int {
    let mut buf: [libc::c_char; 16] = [0; 16];
    return (!hostname.is_null()
        && 1 as libc::c_int
            == inet_pton(
                10 as libc::c_int,
                hostname,
                buf.as_mut_ptr() as *mut libc::c_void,
            )) as libc::c_int;
}
#[inline]
unsafe extern "C" fn tcp_error(
    mut hostname: *mut libc::c_char,
    mut port: libc::c_int,
    mut reason: *const libc::c_char,
) {
    fprintf(
        stderr,
        dcgettext(
            0 as *const libc::c_char,
            b"Unable to connect to server %s:%i: %s\n\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        ),
        hostname,
        port,
        reason,
    );
}
pub unsafe extern "C" fn tcp_connect(
    mut tcp: *mut tcp_t,
    mut hostname: *mut libc::c_char,
    mut port: libc::c_int,
    mut secure: libc::c_int,
    mut local_if: *mut libc::c_char,
    mut io_timeout: libc::c_uint,
) -> libc::c_int {
    let mut local_addr: sockaddr_in = sockaddr_in {
        sin_family: 0,
        sin_port: 0,
        sin_addr: in_addr { s_addr: 0 },
        sin_zero: [0; 8],
    };
    let mut portstr: [libc::c_char; 10] = [0; 10];
    let mut ai_hints: addrinfo = addrinfo {
        ai_flags: 0,
        ai_family: 0,
        ai_socktype: 0,
        ai_protocol: 0,
        ai_addrlen: 0,
        ai_addr: 0 as *mut sockaddr,
        ai_canonname: 0 as *mut libc::c_char,
        ai_next: 0 as *mut addrinfo,
    };
    let mut gai_results: *mut addrinfo = 0 as *mut addrinfo;
    let mut gai_result: *mut addrinfo = 0 as *mut addrinfo;
    let mut ret: libc::c_int = 0;
    let mut sock_fd: libc::c_int = -(1 as libc::c_int);
    memset(
        &mut local_addr as *mut sockaddr_in as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong,
    );
    if !local_if.is_null() {
        if *local_if == 0 || (*tcp).ai_family as libc::c_int != 2 as libc::c_int {
            local_if = 0 as *mut libc::c_char;
        } else {
            local_addr.sin_family = 2 as libc::c_int as sa_family_t;
            local_addr.sin_port = 0 as libc::c_int as in_port_t;
            local_addr.sin_addr.s_addr = inet_addr(local_if);
        }
    }
    snprintf(
        portstr.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong,
        b"%d\0" as *const u8 as *const libc::c_char,
        port,
    );
    memset(
        &mut ai_hints as *mut addrinfo as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<addrinfo>() as libc::c_ulong,
    );
    ai_hints.ai_family = (*tcp).ai_family as libc::c_int;
    ai_hints.ai_socktype = SOCK_STREAM as libc::c_int;
    ai_hints.ai_flags = 0x20 as libc::c_int;
    ai_hints.ai_protocol = 0 as libc::c_int;
    ret = getaddrinfo(hostname, portstr.as_mut_ptr(), &mut ai_hints, &mut gai_results);
    if ret != 0 as libc::c_int {
        tcp_error(hostname, port, gai_strerror(ret));
        return -(1 as libc::c_int);
    }
    gai_result = gai_results;
    loop {
        let mut tcp_fastopen: libc::c_int = -(1 as libc::c_int);
        if sock_fd != -(1 as libc::c_int) {
            close(sock_fd);
            sock_fd = -(1 as libc::c_int);
        }
        sock_fd = socket(
            (*gai_result).ai_family,
            (*gai_result).ai_socktype,
            (*gai_result).ai_protocol,
        );
        if !(sock_fd == -(1 as libc::c_int)) {
            if !local_if.is_null() && (*gai_result).ai_family == 2 as libc::c_int {
                bind(
                    sock_fd,
                    __CONST_SOCKADDR_ARG {
                        __sockaddr__: &mut local_addr as *mut sockaddr_in
                            as *mut sockaddr,
                    },
                    ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as socklen_t,
                );
            }
            tcp_fastopen = setsockopt(
                sock_fd,
                IPPROTO_TCP as libc::c_int,
                30 as libc::c_int,
                0 as *const libc::c_void,
                0 as libc::c_int as socklen_t,
            );
            ret = connect(
                sock_fd,
                __CONST_SOCKADDR_ARG {
                    __sockaddr__: (*gai_result).ai_addr,
                },
                (*gai_result).ai_addrlen,
            );
            if ret != -(1 as libc::c_int) {
                break;
            }
            if !(*__errno_location() != 115 as libc::c_int) {
                if tcp_fastopen != -(1 as libc::c_int) {
                    break;
                }
                let mut fdset: fd_set = fd_set { fds_bits: [0; 16] };
                let mut __d0: libc::c_int = 0;
                let mut __d1: libc::c_int = 0;
                let fresh0 = &mut __d0;
                let fresh1;
                let fresh2 = (::std::mem::size_of::<fd_set>() as libc::c_ulong)
                    .wrapping_div(::std::mem::size_of::<__fd_mask>() as libc::c_ulong);
                let fresh3 = &mut __d1;
                let fresh4;
                let fresh5 = &mut *(fdset.fds_bits)
                    .as_mut_ptr()
                    .offset(0 as libc::c_int as isize) as *mut __fd_mask;
                asm!(
                    "cld; rep; stosq", inlateout("cx")
                    c2rust_asm_casts::AsmCast::cast_in(fresh0, fresh2) => fresh1,
                    inlateout("di") c2rust_asm_casts::AsmCast::cast_in(fresh3, fresh5) =>
                    fresh4, inlateout("ax") 0 as libc::c_int => _,
                    options(preserves_flags, att_syntax)
                );
                c2rust_asm_casts::AsmCast::cast_out(fresh0, fresh2, fresh1);
                c2rust_asm_casts::AsmCast::cast_out(fresh3, fresh5, fresh4);
                fdset
                    .fds_bits[(sock_fd
                    / (8 as libc::c_int
                        * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                            as libc::c_int)) as usize]
                    |= ((1 as libc::c_ulong)
                        << sock_fd
                            % (8 as libc::c_int
                                * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                    as libc::c_int)) as __fd_mask;
                let mut tout: timeval = {
                    let mut init = timeval {
                        tv_sec: io_timeout as __time_t,
                        tv_usec: 0,
                    };
                    init
                };
                ret = select(
                    sock_fd + 1 as libc::c_int,
                    0 as *mut fd_set,
                    &mut fdset,
                    0 as *mut fd_set,
                    &mut tout,
                );
                if ret != -(1 as libc::c_int) {
                    break;
                }
            }
        }
        gai_result = (*gai_result).ai_next;
        if gai_result.is_null() {
            break;
        }
    }
    freeaddrinfo(gai_results);
    if sock_fd == -(1 as libc::c_int) {
        tcp_error(hostname, port, strerror(*__errno_location()));
        return -(1 as libc::c_int);
    }
    fcntl(sock_fd, 4 as libc::c_int, 0 as libc::c_int);
    (*tcp).fd = sock_fd;
    let mut tout_0: timeval = {
        let mut init = timeval {
            tv_sec: io_timeout as __time_t,
            tv_usec: 0,
        };
        init
    };
    setsockopt(
        sock_fd,
        1 as libc::c_int,
        20 as libc::c_int,
        &mut tout_0 as *mut timeval as *const libc::c_void,
        ::std::mem::size_of::<timeval>() as libc::c_ulong as socklen_t,
    );
    setsockopt(
        sock_fd,
        1 as libc::c_int,
        21 as libc::c_int,
        &mut tout_0 as *mut timeval as *const libc::c_void,
        ::std::mem::size_of::<timeval>() as libc::c_ulong as socklen_t,
    );
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn tcp_read(
    mut tcp: *mut tcp_t,
    mut buffer: *mut libc::c_void,
    mut size: libc::c_int,
) -> ssize_t {
    return read((*tcp).fd, buffer, size as size_t);
}
pub unsafe extern "C" fn tcp_write(
    mut tcp: *mut tcp_t,
    mut buffer: *mut libc::c_void,
    mut size: libc::c_int,
) -> ssize_t {
    return write((*tcp).fd, buffer, size as size_t);
}
pub unsafe extern "C" fn tcp_close(mut tcp: *mut tcp_t) {
    if (*tcp).fd > 0 as libc::c_int {
        close((*tcp).fd);
        (*tcp).fd = -(1 as libc::c_int);
    }
}
pub unsafe extern "C" fn get_if_ip(
    mut dst: *mut libc::c_char,
    mut len: size_t,
    mut iface: *const libc::c_char,
) -> libc::c_int {
    let mut ifr: ifreq = ifreq {
        ifr_ifrn: C2RustUnnamed_2 {
            ifrn_name: [0; 16],
        },
        ifr_ifru: C2RustUnnamed_1 {
            ifru_addr: sockaddr {
                sa_family: 0,
                sa_data: [0; 14],
            },
        },
    };
    let mut ret: libc::c_int = 0;
    let mut fd: libc::c_int = socket(
        2 as libc::c_int,
        SOCK_DGRAM as libc::c_int,
        IPPROTO_IP as libc::c_int,
    );
    if fd < 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    memset(
        &mut ifr as *mut ifreq as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<ifreq>() as libc::c_ulong,
    );
    strlcpy(
        (ifr.ifr_ifrn.ifrn_name).as_mut_ptr(),
        iface,
        ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong,
    );
    ifr.ifr_ifru.ifru_addr.sa_family = 2 as libc::c_int as sa_family_t;
    ret = (ioctl(fd, 0x8915 as libc::c_int as libc::c_ulong, &mut ifr as *mut ifreq)
        == 0) as libc::c_int;
    if ret != 0 {
        let mut x: *mut sockaddr_in = &mut ifr.ifr_ifru.ifru_addr as *mut sockaddr
            as *mut sockaddr_in;
        strlcpy(dst, inet_ntoa((*x).sin_addr), len);
    }
    close(fd);
    return ret;
}
