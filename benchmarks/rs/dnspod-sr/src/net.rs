use ::libc;
extern "C" {
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn socket(
        __domain: libc::c_int,
        __type: libc::c_int,
        __protocol: libc::c_int,
    ) -> libc::c_int;
    fn bind(__fd: libc::c_int, __addr: *const sockaddr, __len: socklen_t) -> libc::c_int;
    fn connect(
        __fd: libc::c_int,
        __addr: *const sockaddr,
        __len: socklen_t,
    ) -> libc::c_int;
    fn send(
        __fd: libc::c_int,
        __buf: *const libc::c_void,
        __n: size_t,
        __flags: libc::c_int,
    ) -> ssize_t;
    fn recv(
        __fd: libc::c_int,
        __buf: *mut libc::c_void,
        __n: size_t,
        __flags: libc::c_int,
    ) -> ssize_t;
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
    fn listen(__fd: libc::c_int, __n: libc::c_int) -> libc::c_int;
    fn ntohs(__netshort: uint16_t) -> uint16_t;
    fn htonl(__hostlong: uint32_t) -> uint32_t;
    fn htons(__hostshort: uint16_t) -> uint16_t;
    fn epoll_create(__size: libc::c_int) -> libc::c_int;
    fn epoll_ctl(
        __epfd: libc::c_int,
        __op: libc::c_int,
        __fd: libc::c_int,
        __event: *mut epoll_event,
    ) -> libc::c_int;
    fn inet_pton(
        __af: libc::c_int,
        __cp: *const libc::c_char,
        __buf: *mut libc::c_void,
    ) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn perror(__s: *const libc::c_char);
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn dns_error(_: libc::c_int, _: *mut libc::c_char);
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;
pub type ushort = libc::c_ushort;
pub type uint = libc::c_uint;
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
pub type C2RustUnnamed = libc::c_uint;
pub const MSG_CMSG_CLOEXEC: C2RustUnnamed = 1073741824;
pub const MSG_FASTOPEN: C2RustUnnamed = 536870912;
pub const MSG_ZEROCOPY: C2RustUnnamed = 67108864;
pub const MSG_BATCH: C2RustUnnamed = 262144;
pub const MSG_WAITFORONE: C2RustUnnamed = 65536;
pub const MSG_MORE: C2RustUnnamed = 32768;
pub const MSG_NOSIGNAL: C2RustUnnamed = 16384;
pub const MSG_ERRQUEUE: C2RustUnnamed = 8192;
pub const MSG_RST: C2RustUnnamed = 4096;
pub const MSG_CONFIRM: C2RustUnnamed = 2048;
pub const MSG_SYN: C2RustUnnamed = 1024;
pub const MSG_FIN: C2RustUnnamed = 512;
pub const MSG_WAITALL: C2RustUnnamed = 256;
pub const MSG_EOR: C2RustUnnamed = 128;
pub const MSG_DONTWAIT: C2RustUnnamed = 64;
pub const MSG_TRUNC: C2RustUnnamed = 32;
pub const MSG_PROXY: C2RustUnnamed = 16;
pub const MSG_CTRUNC: C2RustUnnamed = 8;
pub const MSG_DONTROUTE: C2RustUnnamed = 4;
pub const MSG_PEEK: C2RustUnnamed = 2;
pub const MSG_OOB: C2RustUnnamed = 1;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type in_addr_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in_addr {
    pub s_addr: in_addr_t,
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
pub type EPOLL_EVENTS = libc::c_uint;
pub const EPOLLET: EPOLL_EVENTS = 2147483648;
pub const EPOLLONESHOT: EPOLL_EVENTS = 1073741824;
pub const EPOLLWAKEUP: EPOLL_EVENTS = 536870912;
pub const EPOLLEXCLUSIVE: EPOLL_EVENTS = 268435456;
pub const EPOLLRDHUP: EPOLL_EVENTS = 8192;
pub const EPOLLHUP: EPOLL_EVENTS = 16;
pub const EPOLLERR: EPOLL_EVENTS = 8;
pub const EPOLLMSG: EPOLL_EVENTS = 1024;
pub const EPOLLWRBAND: EPOLL_EVENTS = 512;
pub const EPOLLWRNORM: EPOLL_EVENTS = 256;
pub const EPOLLRDBAND: EPOLL_EVENTS = 128;
pub const EPOLLRDNORM: EPOLL_EVENTS = 64;
pub const EPOLLOUT: EPOLL_EVENTS = 4;
pub const EPOLLPRI: EPOLL_EVENTS = 2;
pub const EPOLLIN: EPOLL_EVENTS = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub union epoll_data {
    pub ptr: *mut libc::c_void,
    pub fd: libc::c_int,
    pub u32_0: uint32_t,
    pub u64_0: uint64_t,
}
pub type epoll_data_t = epoll_data;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct epoll_event {
    pub events: uint32_t,
    pub data: epoll_data_t,
}
pub type uchar = libc::c_uchar;
pub type hashval_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _packet_type {
    pub label_count: uint8_t,
    pub domain: [uchar; 256],
    pub label: [*mut uint8_t; 64],
    pub label_offsets: [uint8_t; 64],
    pub label_len: [uint8_t; 64],
    pub hash: [hashval_t; 64],
}
pub type packet_type = _packet_type;
pub type rrtype = libc::c_uint;
pub const ANY: rrtype = 255;
pub const MAILA: rrtype = 254;
pub const MAILB: rrtype = 253;
pub const AXFR: rrtype = 252;
pub const TKEY: rrtype = 249;
pub const DHCID: rrtype = 49;
pub const DNSKEY: rrtype = 48;
pub const NSEC: rrtype = 47;
pub const RRSIG: rrtype = 46;
pub const DS: rrtype = 43;
pub const APL: rrtype = 42;
pub const OPT: rrtype = 41;
pub const DNAME: rrtype = 39;
pub const A6: rrtype = 38;
pub const CERT: rrtype = 37;
pub const SRV: rrtype = 33;
pub const NXT: rrtype = 30;
pub const AAAA: rrtype = 28;
pub const KEY: rrtype = 25;
pub const SIG: rrtype = 24;
pub const AFSDB: rrtype = 18;
pub const RP: rrtype = 17;
pub const TXT: rrtype = 16;
pub const MX: rrtype = 15;
pub const MINFO: rrtype = 14;
pub const HINFO: rrtype = 13;
pub const PTR: rrtype = 12;
pub const WKS: rrtype = 11;
pub const NUL: rrtype = 10;
pub const MR: rrtype = 9;
pub const MG: rrtype = 8;
pub const MB: rrtype = 7;
pub const SOA: rrtype = 6;
pub const CNAME: rrtype = 5;
pub const MF: rrtype = 4;
pub const MD: rrtype = 3;
pub const NS: rrtype = 2;
pub const A: rrtype = 1;
pub const BEGIN_TYPE: rrtype = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mbuf_ring {
    pub prod: prod,
    pub cons: cons,
    pub ring: [*mut libc::c_void; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cons {
    pub sc_dequeue: uint32_t,
    pub size: uint32_t,
    pub mask: uint32_t,
    pub head: uint32_t,
    pub tail: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct prod {
    pub watermark: uint32_t,
    pub sp_enqueue: uint32_t,
    pub size: uint32_t,
    pub mask: uint32_t,
    pub head: uint32_t,
    pub tail: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _mem_buf {
    pub mbuf: *mut mbuf_ring,
    pub fetch_len: uint,
    pub socktype: uint,
    pub fd: libc::c_int,
    pub addr: *mut sockaddr_in,
    pub caddr: sockaddr_in,
    pub aaddr: sockaddr_in,
    pub data: [uchar; 4096],
    pub qtype: rrtype,
    pub err: libc::c_int,
    pub dlen: libc::c_int,
    pub id: ushort,
    pub lowerdomain: packet_type,
    pub origindomain: *mut uchar,
    pub buflen: libc::c_int,
    pub buf: *mut uchar,
    pub td: *mut uchar,
    pub cid: ushort,
    pub qlen: ushort,
    pub lables: ushort,
    pub qing: *mut uchar,
    pub qhash: *mut hashval_t,
    pub backid: ushort,
    pub aid: ushort,
    pub mask: ushort,
    pub qname: ushort,
    pub sq: ushort,
    pub qtimes: ushort,
    pub auth_socktype: ushort,
    pub stat: ushort,
    pub qbuffer: [uchar; 256],
    pub qbuffer_hash: hashval_t,
    pub tdbuffer: *mut uchar,
    pub tempbuffer: *mut uchar,
    pub dmbuffer: *mut uchar,
    pub ipbuffer: *mut uchar,
    pub hascname: ushort,
    pub tcpfd: libc::c_int,
    pub tcpnums: libc::c_int,
    pub mxtry: libc::c_int,
    pub qns: libc::c_int,
    pub stime: uint64_t,
}
pub type mbuf_type = _mem_buf;
pub type SA = sockaddr;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockinfo {
    pub addr: sockaddr_in,
    pub fd: libc::c_int,
    pub buflen: libc::c_int,
    pub socktype: libc::c_int,
    pub buf: *mut uchar,
    pub lowerdomain: *mut packet_type,
    pub mbuf: *mut mbuf_type,
}
pub unsafe extern "C" fn check_client_addr(mut addr: *mut sockaddr_in) -> libc::c_int {
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn add_backdoor(mut fd: libc::c_int) -> libc::c_int {
    let mut epfd: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut ev: epoll_event = {
        let mut init = epoll_event {
            events: 0 as libc::c_int as uint32_t,
            data: epoll_data {
                ptr: 0 as *mut libc::c_void,
            },
        };
        init
    };
    epfd = epoll_create(1000 as libc::c_int);
    if epfd < 0 as libc::c_int {
        dns_error(
            0 as libc::c_int,
            b"epoll bd\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    ev.data.fd = fd;
    ev.events = EPOLLIN as libc::c_int as uint32_t;
    ret = epoll_ctl(epfd, 1 as libc::c_int, ev.data.fd, &mut ev);
    if ret < 0 as libc::c_int {
        dns_error(
            0 as libc::c_int,
            b"epoll add udp backdoor\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    return epfd;
}
pub unsafe extern "C" fn set_recv_timeout(
    mut fd: libc::c_int,
    mut sec: libc::c_int,
    mut usec: libc::c_int,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    tv.tv_sec = sec as __time_t;
    tv.tv_usec = usec as __suseconds_t;
    ret = setsockopt(
        fd,
        1 as libc::c_int,
        20 as libc::c_int,
        &mut tv as *mut timeval as *mut libc::c_char as *const libc::c_void,
        ::std::mem::size_of::<timeval>() as libc::c_ulong as socklen_t,
    );
    return ret;
}
pub unsafe extern "C" fn create_socket(
    mut port: libc::c_int,
    mut proto: libc::c_int,
    mut addr: *mut uchar,
) -> libc::c_int {
    let mut fd: libc::c_int = -(1 as libc::c_int);
    let mut pt: libc::c_int = -(1 as libc::c_int);
    let mut srv: sockaddr_in = sockaddr_in {
        sin_family: 0,
        sin_port: 0,
        sin_addr: in_addr { s_addr: 0 },
        sin_zero: [0; 8],
    };
    if proto == SOCK_DGRAM as libc::c_int {
        pt = SOCK_DGRAM as libc::c_int;
    } else if proto == SOCK_STREAM as libc::c_int {
        pt = SOCK_STREAM as libc::c_int;
    }
    fd = socket(2 as libc::c_int, pt, 0 as libc::c_int);
    if fd <= 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    srv.sin_family = 2 as libc::c_int as sa_family_t;
    if addr.is_null() {
        srv.sin_addr.s_addr = htonl(0 as libc::c_int as in_addr_t);
    } else {
        inet_pton(
            2 as libc::c_int,
            addr as *const libc::c_char,
            &mut srv.sin_addr as *mut in_addr as *mut libc::c_void,
        );
    }
    srv.sin_port = htons(port as uint16_t);
    if bind(
        fd,
        &mut srv as *mut sockaddr_in as *mut SA,
        ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as socklen_t,
    ) < 0 as libc::c_int
    {
        perror(b"bind:\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    if proto == SOCK_STREAM as libc::c_int {
        listen(fd, 512 as libc::c_int);
    }
    return fd;
}
pub unsafe extern "C" fn connect_to(mut si: *mut sockinfo) -> libc::c_int {
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut len: socklen_t = ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong
        as socklen_t;
    ret = connect((*si).fd, &mut (*si).addr as *mut sockaddr_in as *mut SA, len);
    if ret < 0 as libc::c_int {
        if *__errno_location() == 115 as libc::c_int {
            return 0 as libc::c_int;
        }
        printf(
            b"%d,%d,\0" as *const u8 as *const libc::c_char,
            (*si).fd,
            *__errno_location(),
        );
        perror(b"conn\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn tcp_write_info(
    mut mbuf: *mut mbuf_type,
    mut vi: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    ret = send(
        (*mbuf).fd,
        (*mbuf).buf as *const libc::c_void,
        (*mbuf).buflen as size_t,
        MSG_NOSIGNAL as libc::c_int,
    ) as libc::c_int;
    if ret < 0 as libc::c_int {
        printf(b"%d,\0" as *const u8 as *const libc::c_char, *__errno_location());
        perror(b"tcp send\0" as *const u8 as *const libc::c_char);
    }
    if vi == 1 as libc::c_int {
        printf(b"fd is %d\n\0" as *const u8 as *const libc::c_char, (*mbuf).fd);
        i = 0 as libc::c_int;
        while i < (*mbuf).buflen {
            printf(
                b"%x,\0" as *const u8 as *const libc::c_char,
                *((*mbuf).buf).offset(i as isize) as libc::c_int,
            );
            i += 1;
            i;
        }
        printf(b"\n\0" as *const u8 as *const libc::c_char);
    }
    return ret;
}
pub unsafe extern "C" fn udp_write_info(
    mut mbuf: *mut mbuf_type,
    mut vi: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut len: socklen_t = 0;
    if vi != 0 {
        dbg_print_addr((*mbuf).addr);
        i = 0 as libc::c_int;
        while i < (*mbuf).buflen {
            if i % 16 as libc::c_int == 0 as libc::c_int {
                printf(b"\n\0" as *const u8 as *const libc::c_char);
            }
            printf(
                b"%02x,\0" as *const u8 as *const libc::c_char,
                *((*mbuf).buf).offset(i as isize) as libc::c_int,
            );
            i += 1;
            i;
        }
        printf(b"\n\0" as *const u8 as *const libc::c_char);
    }
    len = ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as socklen_t;
    (*(*mbuf).addr).sin_family = 2 as libc::c_int as sa_family_t;
    ret = sendto(
        (*mbuf).fd,
        (*mbuf).buf as *const libc::c_void,
        (*mbuf).buflen as size_t,
        0 as libc::c_int,
        (*mbuf).addr as *mut SA,
        len,
    ) as libc::c_int;
    return ret;
}
pub unsafe extern "C" fn tcp_read_dns_msg(
    mut mbuf: *mut mbuf_type,
    mut max: uint,
    mut vi: libc::c_int,
) -> libc::c_int {
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut tp: libc::c_int = 0;
    let mut rcvnum: libc::c_int = 0;
    let mut buf: [uchar; 4] = [0 as libc::c_int as uchar, 0, 0, 0];
    let mut le: ushort = 0 as libc::c_int as ushort;
    tp = recv(
        (*mbuf).fd,
        buf.as_mut_ptr() as *mut libc::c_void,
        2 as libc::c_int as size_t,
        0 as libc::c_int,
    ) as libc::c_int;
    if tp < 0 as libc::c_int {
        printf(b"%d,\0" as *const u8 as *const libc::c_char, (*mbuf).fd);
        perror(b"tp\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    if tp == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    memcpy(
        &mut le as *mut ushort as *mut libc::c_void,
        buf.as_mut_ptr() as *const libc::c_void,
        ::std::mem::size_of::<ushort>() as libc::c_ulong,
    );
    le = ntohs(le);
    if le as libc::c_uint > max {
        printf(
            b"too large %d,%u,%d\n\0" as *const u8 as *const libc::c_char,
            (*mbuf).fd,
            le as libc::c_int,
            max,
        );
        return -(1 as libc::c_int);
    }
    while ret < le as libc::c_int {
        rcvnum = recv(
            (*mbuf).fd,
            ((*mbuf).buf).offset(ret as isize) as *mut libc::c_void,
            ((*mbuf).buflen - ret) as size_t,
            0 as libc::c_int,
        ) as libc::c_int;
        if rcvnum < 0 as libc::c_int {
            if *__errno_location() == 11 as libc::c_int
                || *__errno_location() == 11 as libc::c_int
            {
                continue;
            }
            printf(
                b"tcp data %d,%d,%d\0" as *const u8 as *const libc::c_char,
                (*mbuf).fd,
                le as libc::c_int,
                ret,
            );
            perror(b"tcp read\0" as *const u8 as *const libc::c_char);
            return -(1 as libc::c_int);
        } else if rcvnum == 0 as libc::c_int {
            ret = -(1 as libc::c_int);
            break;
        } else {
            ret += rcvnum;
        }
    }
    return ret;
}
pub unsafe extern "C" fn udp_read_msg(
    mut mbuf: *mut mbuf_type,
    mut visible: libc::c_int,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut len: socklen_t = ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong
        as socklen_t;
    ret = recvfrom(
        (*mbuf).fd,
        (*mbuf).buf as *mut libc::c_void,
        (*mbuf).buflen as size_t,
        0 as libc::c_int,
        (*mbuf).addr as *mut SA,
        &mut len,
    ) as libc::c_int;
    if ret < 0 as libc::c_int {
        return ret;
    }
    if visible != 0 {
        i = 0 as libc::c_int;
        while i < ret {
            printf(
                b"%x,\0" as *const u8 as *const libc::c_char,
                *((*mbuf).buf).offset(i as isize) as libc::c_int,
            );
            i += 1;
            i;
        }
        printf(b"\n\0" as *const u8 as *const libc::c_char);
    }
    return ret;
}
pub unsafe extern "C" fn set_sock_buff(
    mut fd: libc::c_int,
    mut m: libc::c_int,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut bufsize: libc::c_int = m * 1024 as libc::c_int * 1024 as libc::c_int;
    if fd <= 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    ret = setsockopt(
        fd,
        1 as libc::c_int,
        8 as libc::c_int,
        &mut bufsize as *mut libc::c_int as *const uchar as *const libc::c_void,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t,
    );
    return ret;
}
pub unsafe extern "C" fn set_non_block(mut fd: libc::c_int) -> libc::c_int {
    let mut opt: libc::c_int = fcntl(fd, 3 as libc::c_int, 0 as libc::c_int);
    if opt < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    opt |= 0o4000 as libc::c_int;
    return fcntl(fd, 4 as libc::c_int, opt);
}
pub unsafe extern "C" fn make_bin_from_str(
    mut bin: *mut uchar,
    mut str: *const libc::c_char,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut val: uchar = 0 as libc::c_int as uchar;
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        while *str.offset(0 as libc::c_int as isize) as libc::c_int != '.' as i32
            && *str.offset(0 as libc::c_int as isize) as libc::c_int != 0 as libc::c_int
        {
            val = (val as libc::c_int * 10 as libc::c_int
                + *str.offset(0 as libc::c_int as isize) as libc::c_int - '0' as i32)
                as uchar;
            str = str.offset(1);
            str;
        }
        str = str.offset(1);
        str;
        *bin.offset(0 as libc::c_int as isize) = val;
        val = 0 as libc::c_int as uchar;
        bin = bin.offset(1);
        bin;
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn make_addr_from_bin(
    mut addr: *mut sockaddr_in,
    mut data: *mut uchar,
) -> libc::c_int {
    let mut ipv4: [uchar; 16] = [
        0 as libc::c_int as uchar,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    let mut idx: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut val: ushort = 0 as libc::c_int as ushort;
    if *data.offset(0 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int
        && *data.offset(1 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int
        && *data.offset(2 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int
        && *data.offset(3 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int
    {
        return -(1 as libc::c_int);
    }
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        val = *data.offset(i as isize) as ushort;
        if val as libc::c_int > 99 as libc::c_int {
            ipv4[idx
                as usize] = (val as libc::c_int / 100 as libc::c_int + '0' as i32)
                as uchar;
            idx += 1;
            idx;
        }
        if val as libc::c_int > 9 as libc::c_int {
            ipv4[idx
                as usize] = (val as libc::c_int % 100 as libc::c_int / 10 as libc::c_int
                + '0' as i32) as uchar;
            idx += 1;
            idx;
        }
        ipv4[idx
            as usize] = (val as libc::c_int % 10 as libc::c_int + '0' as i32) as uchar;
        idx += 1;
        idx;
        ipv4[idx as usize] = '.' as i32 as uchar;
        idx += 1;
        idx;
        i += 1;
        i;
    }
    ipv4[(idx - 1 as libc::c_int) as usize] = 0 as libc::c_int as uchar;
    i = inet_pton(
        2 as libc::c_int,
        ipv4.as_mut_ptr() as *const libc::c_char,
        &mut (*addr).sin_addr as *mut in_addr as *mut libc::c_void,
    );
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn dbg_print_addr(mut addr: *mut sockaddr_in) -> libc::c_int {
    let mut i: uint = 0;
    if addr.is_null() {
        printf(b"null addr\n\0" as *const u8 as *const libc::c_char);
        return 0 as libc::c_int;
    }
    i = (*addr).sin_addr.s_addr;
    printf(
        b"%u.%u.%u.%u\n\0" as *const u8 as *const libc::c_char,
        i.wrapping_rem(256 as libc::c_int as libc::c_uint),
        i
            .wrapping_div(256 as libc::c_int as libc::c_uint)
            .wrapping_rem(256 as libc::c_int as libc::c_uint),
        i
            .wrapping_div(256 as libc::c_int as libc::c_uint)
            .wrapping_div(256 as libc::c_int as libc::c_uint)
            .wrapping_rem(256 as libc::c_int as libc::c_uint),
        i
            .wrapping_div(256 as libc::c_int as libc::c_uint)
            .wrapping_div(256 as libc::c_int as libc::c_uint)
            .wrapping_div(256 as libc::c_int as libc::c_uint),
    );
    return 0 as libc::c_int;
}
