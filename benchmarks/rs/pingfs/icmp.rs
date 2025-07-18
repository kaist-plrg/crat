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
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn bzero(__s: *mut libc::c_void, __n: size_t);
    fn sendto(
        __fd: libc::c_int,
        __buf: *const libc::c_void,
        __n: size_t,
        __flags: libc::c_int,
        __addr: __CONST_SOCKADDR_ARG,
        __addr_len: socklen_t,
    ) -> ssize_t;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn inet_ntop(
        __af: libc::c_int,
        __cp: *const libc::c_void,
        __buf: *mut libc::c_char,
        __len: socklen_t,
    ) -> *const libc::c_char;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __ssize_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
pub type ssize_t = __ssize_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type socklen_t = __socklen_t;
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
pub type icmp_type = libc::c_uint;
pub const ICMP_REPLY: icmp_type = 1;
pub const ICMP_REQUEST: icmp_type = 0;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct icmp_rule {
    pub request_type: libc::c_int,
    pub reply_type: libc::c_int,
    pub use_checksum: libc::c_int,
    pub strip_iphdr: libc::c_int,
}
static mut icmpv4: icmp_rule = {
    let mut init = icmp_rule {
        request_type: 8 as libc::c_int,
        reply_type: 0 as libc::c_int,
        use_checksum: 1 as libc::c_int,
        strip_iphdr: 1 as libc::c_int,
    };
    init
};
static mut icmpv6: icmp_rule = {
    let mut init = icmp_rule {
        request_type: 128 as libc::c_int,
        reply_type: 129 as libc::c_int,
        use_checksum: 0 as libc::c_int,
        strip_iphdr: 0 as libc::c_int,
    };
    init
};
unsafe extern "C" fn checksum(mut data: *mut uint8_t, mut len: uint32_t) -> uint16_t {
    let mut csum: uint32_t = 0 as libc::c_int as uint32_t;
    let mut i: uint32_t = 0;
    i = 0 as libc::c_int as uint32_t;
    while i < len {
        let mut c: uint16_t = ((*data.offset(i as isize) as libc::c_int)
            << 8 as libc::c_int) as uint16_t;
        if i.wrapping_add(1 as libc::c_int as libc::c_uint) < len {
            c = (c as libc::c_int
                | *data.offset(i.wrapping_add(1 as libc::c_int as libc::c_uint) as isize)
                    as libc::c_int) as uint16_t;
        }
        csum = (csum as libc::c_uint).wrapping_add(c as libc::c_uint) as uint32_t
            as uint32_t;
        i = (i as libc::c_uint).wrapping_add(2 as libc::c_int as libc::c_uint)
            as uint32_t as uint32_t;
    }
    csum = (csum >> 16 as libc::c_int)
        .wrapping_add(csum & 0xffff as libc::c_int as libc::c_uint);
    csum = (csum as libc::c_uint).wrapping_add(csum >> 16 as libc::c_int) as uint32_t
        as uint32_t;
    return !csum as uint16_t;
}
unsafe extern "C" fn read16(mut data: *mut uint8_t) -> uint16_t {
    return ((*data.offset(0 as libc::c_int as isize) as libc::c_int) << 8 as libc::c_int
        | *data.offset(1 as libc::c_int as isize) as libc::c_int) as uint16_t;
}
unsafe extern "C" fn write16(mut data: *mut uint8_t, mut s: uint16_t) {
    *data
        .offset(
            0 as libc::c_int as isize,
        ) = (s as libc::c_int >> 8 as libc::c_int) as uint8_t;
    *data
        .offset(
            1 as libc::c_int as isize,
        ) = (s as libc::c_int & 0xff as libc::c_int) as uint8_t;
}
unsafe extern "C" fn icmp_encode(
    mut pkt: *mut icmp_packet,
    mut len: *mut libc::c_int,
) -> *mut uint8_t {
    let mut rule: *const icmp_rule = if (*pkt).peer.ss_family as libc::c_int
        == 2 as libc::c_int
    {
        &icmpv4
    } else {
        &icmpv6
    };
    let mut data: *mut uint8_t = 0 as *mut uint8_t;
    let mut pktlen: libc::c_int = 0;
    pktlen = (8 as libc::c_int as libc::c_uint).wrapping_add((*pkt).payload_len)
        as libc::c_int;
    data = calloc(1 as libc::c_int as libc::c_ulong, pktlen as libc::c_ulong)
        as *mut uint8_t;
    if data.is_null() {
        return 0 as *mut uint8_t;
    }
    if (*pkt).type_0 as libc::c_uint == ICMP_REQUEST as libc::c_int as libc::c_uint {
        *data.offset(0 as libc::c_int as isize) = (*rule).request_type as uint8_t;
    } else {
        *data.offset(0 as libc::c_int as isize) = (*rule).reply_type as uint8_t;
    }
    write16(&mut *data.offset(4 as libc::c_int as isize), (*pkt).id);
    write16(&mut *data.offset(6 as libc::c_int as isize), (*pkt).seqno);
    if (*pkt).payload_len != 0 {
        memcpy(
            &mut *data.offset(8 as libc::c_int as isize) as *mut uint8_t
                as *mut libc::c_void,
            (*pkt).payload as *const libc::c_void,
            (*pkt).payload_len as libc::c_ulong,
        );
    }
    if (*rule).use_checksum != 0 {
        write16(
            &mut *data.offset(2 as libc::c_int as isize),
            checksum(data, pktlen as uint32_t),
        );
    }
    *len = pktlen;
    return data;
}
pub unsafe extern "C" fn icmp_send(
    mut socket: libc::c_int,
    mut pkt: *mut icmp_packet,
) -> libc::c_int {
    let mut len: libc::c_int = 0;
    let mut icmpdata: *mut uint8_t = 0 as *mut uint8_t;
    icmpdata = icmp_encode(pkt, &mut len);
    if icmpdata.is_null() {
        return 0 as libc::c_int;
    }
    len = sendto(
        socket,
        icmpdata as *const libc::c_void,
        len as size_t,
        0 as libc::c_int,
        __CONST_SOCKADDR_ARG {
            __sockaddr__: &mut (*pkt).peer as *mut sockaddr_storage as *mut sockaddr,
        },
        (*pkt).peer_len,
    ) as libc::c_int;
    free(icmpdata as *mut libc::c_void);
    return len;
}
pub unsafe extern "C" fn icmp_parse(
    mut pkt: *mut icmp_packet,
    mut data: *mut uint8_t,
    mut len: libc::c_int,
) -> libc::c_int {
    let mut rule: *const icmp_rule = if (*pkt).peer.ss_family as libc::c_int
        == 2 as libc::c_int
    {
        &icmpv4
    } else {
        &icmpv6
    };
    if (*rule).strip_iphdr != 0 {
        let mut hdrlen: libc::c_int = 0;
        if len == 0 as libc::c_int {
            return -(3 as libc::c_int);
        }
        hdrlen = (*data.offset(0 as libc::c_int as isize) as libc::c_int
            & 0xf as libc::c_int) << 2 as libc::c_int;
        if len < hdrlen {
            return -(4 as libc::c_int);
        }
        data = data.offset(hdrlen as isize);
        len -= hdrlen;
    }
    if len < 8 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if (*rule).use_checksum != 0 {
        if checksum(data, len as uint32_t) as libc::c_int != 0 as libc::c_int {
            return -(2 as libc::c_int);
        }
    }
    if (*rule).request_type == *data.offset(0 as libc::c_int as isize) as libc::c_int {
        (*pkt).type_0 = ICMP_REQUEST;
    } else if (*rule).reply_type
        == *data.offset(0 as libc::c_int as isize) as libc::c_int
    {
        (*pkt).type_0 = ICMP_REPLY;
    } else {
        return -(5 as libc::c_int)
    }
    (*pkt).id = read16(&mut *data.offset(4 as libc::c_int as isize));
    (*pkt).seqno = read16(&mut *data.offset(6 as libc::c_int as isize));
    (*pkt).payload_len = (len - 8 as libc::c_int) as uint32_t;
    if (*pkt).payload_len != 0 {
        (*pkt).payload = malloc((*pkt).payload_len as libc::c_ulong) as *mut uint8_t;
        memcpy(
            (*pkt).payload as *mut libc::c_void,
            &mut *data.offset(8 as libc::c_int as isize) as *mut uint8_t
                as *const libc::c_void,
            (*pkt).payload_len as libc::c_ulong,
        );
    } else {
        (*pkt).payload = 0 as *mut uint8_t;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn get_in_addr(mut ss: *mut sockaddr_storage) -> *mut libc::c_void {
    if (*ss).ss_family as libc::c_int == 2 as libc::c_int {
        return &mut (*(ss as *mut sockaddr_in)).sin_addr as *mut in_addr
            as *mut libc::c_void
    } else {
        return &mut (*(ss as *mut sockaddr_in6)).sin6_addr as *mut in6_addr
            as *mut libc::c_void
    };
}
unsafe extern "C" fn icmp_type_str(mut pkt: *mut icmp_packet) -> *mut libc::c_char {
    if (*pkt).type_0 as libc::c_uint == ICMP_REPLY as libc::c_int as libc::c_uint {
        return b"Reply from\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    if (*pkt).type_0 as libc::c_uint == ICMP_REQUEST as libc::c_int as libc::c_uint {
        return b"Request to\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    return b"Other\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
}
pub unsafe extern "C" fn icmp_dump(mut pkt: *mut icmp_packet) {
    let mut ipaddr: [libc::c_char; 64] = [0; 64];
    bzero(
        ipaddr.as_mut_ptr() as *mut libc::c_void,
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
    );
    inet_ntop(
        (*pkt).peer.ss_family as libc::c_int,
        get_in_addr(&mut (*pkt).peer),
        ipaddr.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as socklen_t,
    );
    printf(
        b"%s %s, id %04X, seqno %04X, payload %d bytes\n\0" as *const u8
            as *const libc::c_char,
        icmp_type_str(pkt),
        ipaddr.as_mut_ptr(),
        (*pkt).id as libc::c_int,
        (*pkt).seqno as libc::c_int,
        (*pkt).payload_len,
    );
}
