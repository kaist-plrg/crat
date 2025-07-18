use ::libc;
extern "C" {
    fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option::<
            unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
        >,
        __arg: *mut libc::c_void,
    ) -> libc::c_int;
    fn pthread_join(
        __th: pthread_t,
        __thread_return: *mut *mut libc::c_void,
    ) -> libc::c_int;
    fn pthread_kill(__threadid: pthread_t, __signo: libc::c_int) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
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
    fn socket(
        __domain: libc::c_int,
        __type: libc::c_int,
        __protocol: libc::c_int,
    ) -> libc::c_int;
    fn bind(__fd: libc::c_int, __addr: *const sockaddr, __len: socklen_t) -> libc::c_int;
    fn getsockname(
        __fd: libc::c_int,
        __addr: *mut sockaddr,
        __len: *mut socklen_t,
    ) -> libc::c_int;
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
    fn getaddrinfo(
        __name: *const libc::c_char,
        __service: *const libc::c_char,
        __req: *const addrinfo,
        __pai: *mut *mut addrinfo,
    ) -> libc::c_int;
    fn freeaddrinfo(__ai: *mut addrinfo);
    fn gai_strerror(__ecode: libc::c_int) -> *const libc::c_char;
    fn debug(level: libc::c_int, format: *mut libc::c_char, _: ...);
    fn warn(format: *mut libc::c_char, _: ...);
    fn die(format: *mut libc::c_char, _: ...);
    fn player_put_packet(seqno: seq_t, data: *mut uint8_t, len: libc::c_int);
}
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __ssize_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_attr_t {
    pub __size: [libc::c_char; 56],
    pub __align: libc::c_long,
}
pub type pthread_t = libc::c_ulong;
pub type ssize_t = __ssize_t;
pub type socklen_t = __socklen_t;
pub type int16_t = __int16_t;
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
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
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
pub struct sockaddr_in6 {
    pub sin6_family: sa_family_t,
    pub sin6_port: in_port_t,
    pub sin6_flowinfo: uint32_t,
    pub sin6_addr: in6_addr,
    pub sin6_scope_id: uint32_t,
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
pub type seq_t = uint16_t;
#[inline]
unsafe extern "C" fn __bswap_16(mut __bsx: __uint16_t) -> __uint16_t {
    return (__bsx as libc::c_int >> 8 as libc::c_int & 0xff as libc::c_int
        | (__bsx as libc::c_int & 0xff as libc::c_int) << 8 as libc::c_int)
        as __uint16_t;
}
#[inline]
unsafe extern "C" fn seq_diff(mut a: seq_t, mut b: seq_t) -> uint16_t {
    let mut diff: int16_t = (b as libc::c_int - a as libc::c_int) as int16_t;
    return diff as uint16_t;
}
static mut running: libc::c_int = 0 as libc::c_int;
static mut please_shutdown: libc::c_int = 0;
static mut rtp_client: sockaddr_storage = sockaddr_storage {
    ss_family: 0,
    __ss_padding: [0; 118],
    __ss_align: 0,
};
static mut sock: libc::c_int = 0;
static mut rtp_thread: pthread_t = 0;
unsafe extern "C" fn rtp_receiver(mut arg: *mut libc::c_void) -> *mut libc::c_void {
    let mut packet: [uint8_t; 2048] = [0; 2048];
    let mut pktp: *mut uint8_t = 0 as *mut uint8_t;
    let mut nread: ssize_t = 0;
    while !(please_shutdown != 0) {
        nread = recv(
            sock,
            packet.as_mut_ptr() as *mut libc::c_void,
            ::std::mem::size_of::<[uint8_t; 2048]>() as libc::c_ulong,
            0 as libc::c_int,
        );
        if nread < 0 as libc::c_int as libc::c_long {
            break;
        }
        let mut plen: ssize_t = nread;
        let mut type_0: uint8_t = (packet[1 as libc::c_int as usize] as libc::c_int
            & !(0x80 as libc::c_int)) as uint8_t;
        if type_0 as libc::c_int == 0x54 as libc::c_int {
            continue;
        }
        if type_0 as libc::c_int == 0x60 as libc::c_int
            || type_0 as libc::c_int == 0x56 as libc::c_int
        {
            pktp = packet.as_mut_ptr();
            if type_0 as libc::c_int == 0x56 as libc::c_int {
                pktp = pktp.offset(4 as libc::c_int as isize);
                plen -= 4 as libc::c_int as libc::c_long;
            }
            let mut seqno: seq_t = __bswap_16(
                *(pktp.offset(2 as libc::c_int as isize) as *mut libc::c_ushort),
            );
            pktp = pktp.offset(12 as libc::c_int as isize);
            plen -= 12 as libc::c_int as libc::c_long;
            if plen >= 16 as libc::c_int as libc::c_long {
                player_put_packet(seqno, pktp, plen as libc::c_int);
            } else if type_0 as libc::c_int == 0x56 as libc::c_int
                && seqno as libc::c_int == 0 as libc::c_int
            {
                debug(
                    2 as libc::c_int,
                    b"resend-related request packet received, ignoring.\n\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                );
            } else {
                debug(
                    1 as libc::c_int,
                    b"Unknown RTP packet of type 0x%02X length %d seqno %d\n\0"
                        as *const u8 as *const libc::c_char as *mut libc::c_char,
                    type_0 as libc::c_int,
                    nread,
                    seqno as libc::c_int,
                );
            }
        } else {
            warn(
                b"Unknown RTP packet of type 0x%02X length %d\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                type_0 as libc::c_int,
                nread,
            );
        }
    }
    debug(
        1 as libc::c_int,
        b"RTP thread interrupted. terminating.\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    close(sock);
    return 0 as *mut libc::c_void;
}
unsafe extern "C" fn bind_port(mut remote: *mut sockaddr_storage) -> libc::c_int {
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
    let mut info: *mut addrinfo = 0 as *mut addrinfo;
    memset(
        &mut hints as *mut addrinfo as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<addrinfo>() as libc::c_ulong,
    );
    hints.ai_family = (*remote).ss_family as libc::c_int;
    hints.ai_socktype = SOCK_DGRAM as libc::c_int;
    hints.ai_flags = 0x1 as libc::c_int;
    let mut ret: libc::c_int = getaddrinfo(
        0 as *const libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char,
        &mut hints,
        &mut info,
    );
    if ret < 0 as libc::c_int {
        die(
            b"failed to get usable addrinfo?! %s\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            gai_strerror(ret),
        );
    }
    sock = socket(
        (*remote).ss_family as libc::c_int,
        SOCK_DGRAM as libc::c_int,
        IPPROTO_UDP as libc::c_int,
    );
    ret = bind(sock, (*info).ai_addr, (*info).ai_addrlen);
    freeaddrinfo(info);
    if ret < 0 as libc::c_int {
        die(
            b"could not bind a UDP port!\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    let mut sport: libc::c_int = 0;
    let mut local: sockaddr_storage = sockaddr_storage {
        ss_family: 0,
        __ss_padding: [0; 118],
        __ss_align: 0,
    };
    let mut local_len: socklen_t = ::std::mem::size_of::<sockaddr_storage>()
        as libc::c_ulong as socklen_t;
    getsockname(
        sock,
        &mut local as *mut sockaddr_storage as *mut sockaddr,
        &mut local_len,
    );
    if local.ss_family as libc::c_int == 10 as libc::c_int {
        let mut sa6: *mut sockaddr_in6 = &mut local as *mut sockaddr_storage
            as *mut sockaddr_in6;
        sport = __bswap_16((*sa6).sin6_port) as libc::c_int;
    } else {
        let mut sa: *mut sockaddr_in = &mut local as *mut sockaddr_storage
            as *mut sockaddr_in;
        sport = __bswap_16((*sa).sin_port) as libc::c_int;
    }
    return sport;
}
pub unsafe extern "C" fn rtp_setup(
    mut remote: *mut sockaddr_storage,
    mut cport: libc::c_int,
    mut tport: libc::c_int,
) -> libc::c_int {
    if running != 0 {
        die(
            b"rtp_setup called with active stream!\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    debug(
        1 as libc::c_int,
        b"rtp_setup: cport=%d tport=%d\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        cport,
        tport,
    );
    memcpy(
        &mut rtp_client as *mut sockaddr_storage as *mut libc::c_void,
        remote as *const libc::c_void,
        ::std::mem::size_of::<sockaddr_storage>() as libc::c_ulong,
    );
    if rtp_client.ss_family as libc::c_int == 10 as libc::c_int {
        let mut sa6: *mut sockaddr_in6 = &mut rtp_client as *mut sockaddr_storage
            as *mut sockaddr_in6;
        (*sa6).sin6_port = __bswap_16(cport as __uint16_t);
    } else {
        let mut sa: *mut sockaddr_in = &mut rtp_client as *mut sockaddr_storage
            as *mut sockaddr_in;
        (*sa).sin_port = __bswap_16(cport as __uint16_t);
    }
    let mut sport: libc::c_int = bind_port(remote);
    debug(
        1 as libc::c_int,
        b"rtp listening on port %d\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        sport,
    );
    please_shutdown = 0 as libc::c_int;
    pthread_create(
        &mut rtp_thread,
        0 as *const pthread_attr_t,
        Some(
            rtp_receiver as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
        ),
        0 as *mut libc::c_void,
    );
    running = 1 as libc::c_int;
    return sport;
}
pub unsafe extern "C" fn rtp_shutdown() {
    if running == 0 {
        die(
            b"rtp_shutdown called without active stream!\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        );
    }
    debug(
        2 as libc::c_int,
        b"shutting down RTP thread\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    please_shutdown = 1 as libc::c_int;
    pthread_kill(rtp_thread, 10 as libc::c_int);
    let mut retval: *mut libc::c_void = 0 as *mut libc::c_void;
    pthread_join(rtp_thread, &mut retval);
    running = 0 as libc::c_int;
}
pub unsafe extern "C" fn rtp_request_resend(mut first: seq_t, mut last: seq_t) {
    if running == 0 {
        die(
            b"rtp_request_resend called without active stream!\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        );
    }
    debug(
        1 as libc::c_int,
        b"requesting resend on %d packets (%04X:%04X)\n\0" as *const u8
            as *const libc::c_char as *mut libc::c_char,
        seq_diff(first, last) as libc::c_int + 1 as libc::c_int,
        first as libc::c_int,
        last as libc::c_int,
    );
    let mut req: [libc::c_char; 8] = [0; 8];
    req[0 as libc::c_int as usize] = 0x80 as libc::c_int as libc::c_char;
    req[1 as libc::c_int
        as usize] = (0x55 as libc::c_int | 0x80 as libc::c_int) as libc::c_char;
    *(req.as_mut_ptr().offset(2 as libc::c_int as isize)
        as *mut libc::c_ushort) = __bswap_16(1 as libc::c_int as __uint16_t);
    *(req.as_mut_ptr().offset(4 as libc::c_int as isize)
        as *mut libc::c_ushort) = __bswap_16(first);
    *(req.as_mut_ptr().offset(6 as libc::c_int as isize)
        as *mut libc::c_ushort) = __bswap_16(
        (last as libc::c_int - first as libc::c_int + 1 as libc::c_int) as __uint16_t,
    );
    sendto(
        sock,
        req.as_mut_ptr() as *const libc::c_void,
        ::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong,
        0 as libc::c_int,
        &mut rtp_client as *mut sockaddr_storage as *mut sockaddr,
        ::std::mem::size_of::<sockaddr_storage>() as libc::c_ulong as socklen_t,
    );
}
