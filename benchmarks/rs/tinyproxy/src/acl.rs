use ::libc;
extern "C" {
    fn free(__ptr: *mut libc::c_void);
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strcasecmp(__s1: *const libc::c_char, __s2: *const libc::c_char) -> libc::c_int;
    fn getaddrinfo(
        __name: *const libc::c_char,
        __service: *const libc::c_char,
        __req: *const addrinfo,
        __pai: *mut *mut addrinfo,
    ) -> libc::c_int;
    fn freeaddrinfo(__ai: *mut addrinfo);
    fn getnameinfo(
        __sa: *const sockaddr,
        __salen: socklen_t,
        __host: *mut libc::c_char,
        __hostlen: socklen_t,
        __serv: *mut libc::c_char,
        __servlen: socklen_t,
        __flags: libc::c_int,
    ) -> libc::c_int;
    fn sblist_free(l: *mut sblist);
    fn sblist_get(l: *mut sblist, item: size_t) -> *mut libc::c_void;
    fn sblist_add(l: *mut sblist, item: *mut libc::c_void) -> libc::c_int;
    fn sblist_new(itemsize: size_t, blockitems: size_t) -> *mut sblist;
    fn log_message(level: libc::c_int, fmt: *const libc::c_char, _: ...);
    fn get_ip_string(
        sa: *mut sockaddr,
        buf: *mut libc::c_char,
        len: size_t,
    ) -> *const libc::c_char;
    fn full_inet_pton(ip: *const libc::c_char, dst: *mut libc::c_void) -> libc::c_int;
    fn hostspec_parse(domain: *mut libc::c_char, h: *mut hostspec) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __socklen_t = libc::c_uint;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
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
pub struct sblist {
    pub itemsize: size_t,
    pub blockitems: size_t,
    pub count: size_t,
    pub capa: size_t,
    pub items: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union sockaddr_union {
    pub v4: sockaddr_in,
    pub v6: sockaddr_in6,
}
pub type acl_access_t = libc::c_uint;
pub const ACL_DENY: acl_access_t = 1;
pub const ACL_ALLOW: acl_access_t = 0;
pub type acl_list_t = *mut sblist;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct acl_s {
    pub access: acl_access_t,
    pub h: hostspec,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hostspec {
    pub type_0: hostspec_type,
    pub address: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub string: *mut libc::c_char,
    pub ip: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub network: [libc::c_uchar; 16],
    pub mask: [libc::c_uchar; 16],
}
pub type hostspec_type = libc::c_uint;
pub const HST_NUMERIC: hostspec_type = 2;
pub const HST_STRING: hostspec_type = 1;
pub const HST_NONE: hostspec_type = 0;
unsafe extern "C" fn init_access_list(mut access_list: *mut acl_list_t) -> libc::c_int {
    if (*access_list).is_null() {
        *access_list = sblist_new(
            ::std::mem::size_of::<acl_s>() as libc::c_ulong,
            16 as libc::c_int as size_t,
        );
        if (*access_list).is_null() {
            log_message(
                3 as libc::c_int,
                b"Unable to allocate memory for access list\0" as *const u8
                    as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn insert_acl(
    mut location: *mut libc::c_char,
    mut access_type: acl_access_t,
    mut access_list: *mut acl_list_t,
) -> libc::c_int {
    let mut acl: acl_s = acl_s {
        access: ACL_ALLOW,
        h: hostspec {
            type_0: HST_NONE,
            address: C2RustUnnamed_0 {
                string: 0 as *mut libc::c_char,
            },
        },
    };
    if init_access_list(access_list) != 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    memset(
        &mut acl as *mut acl_s as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<acl_s>() as libc::c_ulong,
    );
    acl.access = access_type;
    if hostspec_parse(location, &mut acl.h) != 0
        || acl.h.type_0 as libc::c_uint == HST_NONE as libc::c_int as libc::c_uint
    {
        return -(1 as libc::c_int);
    }
    if sblist_add(*access_list, &mut acl as *mut acl_s as *mut libc::c_void) == 0 {
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn acl_string_processing(
    mut acl: *mut acl_s,
    mut ip_address: *const libc::c_char,
    mut addr: *mut sockaddr_union,
    mut string_addr: *mut libc::c_char,
) -> libc::c_int {
    let mut match_0: libc::c_int = 0;
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
    let mut test_length: size_t = 0;
    let mut match_length: size_t = 0;
    let mut ipbuf: [libc::c_char; 512] = [0; 512];
    if *((*acl).h.address.string).offset(0 as libc::c_int as isize) as libc::c_int
        != '.' as i32
    {
        memset(
            &mut hints as *mut addrinfo as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<addrinfo>() as libc::c_ulong,
        );
        hints.ai_family = 0 as libc::c_int;
        hints.ai_socktype = SOCK_STREAM as libc::c_int;
        if !(getaddrinfo(
            (*acl).h.address.string,
            0 as *const libc::c_char,
            &mut hints,
            &mut res,
        ) != 0 as libc::c_int)
        {
            ressave = res;
            match_0 = 0 as libc::c_int;
            loop {
                get_ip_string(
                    (*res).ai_addr,
                    ipbuf.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong,
                );
                if strcmp(ip_address, ipbuf.as_mut_ptr()) == 0 as libc::c_int {
                    match_0 = (0 as libc::c_int == 0) as libc::c_int;
                    break;
                } else {
                    res = (*res).ai_next;
                    if res.is_null() {
                        break;
                    }
                }
            }
            freeaddrinfo(ressave);
            if match_0 != 0 {
                if (*acl).access as libc::c_uint
                    == ACL_DENY as libc::c_int as libc::c_uint
                {
                    return 0 as libc::c_int
                } else {
                    return 1 as libc::c_int
                }
            }
        }
    }
    if *string_addr.offset(0 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int
    {
        if getnameinfo(
            addr as *mut libc::c_void as *const sockaddr,
            ::std::mem::size_of::<sockaddr_union>() as libc::c_ulong as socklen_t,
            string_addr,
            1024 as libc::c_int as socklen_t,
            0 as *mut libc::c_char,
            0 as libc::c_int as socklen_t,
            0 as libc::c_int,
        ) != 0 as libc::c_int
        {
            return -(1 as libc::c_int);
        }
    }
    test_length = strlen(string_addr);
    match_length = strlen((*acl).h.address.string);
    if test_length < match_length {
        return -(1 as libc::c_int);
    }
    if strcasecmp(
        string_addr.offset(test_length.wrapping_sub(match_length) as isize),
        (*acl).h.address.string,
    ) == 0 as libc::c_int
    {
        if (*acl).access as libc::c_uint == ACL_DENY as libc::c_int as libc::c_uint {
            return 0 as libc::c_int
        } else {
            return 1 as libc::c_int
        }
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn check_numeric_acl(
    mut acl: *const acl_s,
    mut addr: *mut uint8_t,
) -> libc::c_int {
    let mut x: uint8_t = 0;
    let mut y: uint8_t = 0;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i != 16 as libc::c_int {
        x = (*addr.offset(i as isize) as libc::c_int
            & (*acl).h.address.ip.mask[i as usize] as libc::c_int) as uint8_t;
        y = (*acl).h.address.ip.network[i as usize];
        if x as libc::c_int != y as libc::c_int {
            return -(1 as libc::c_int);
        }
        i += 1;
        i;
    }
    return ((*acl).access as libc::c_uint == ACL_ALLOW as libc::c_int as libc::c_uint)
        as libc::c_int;
}
pub unsafe extern "C" fn check_acl(
    mut ip: *const libc::c_char,
    mut addr: *mut sockaddr_union,
    mut access_list: acl_list_t,
) -> libc::c_int {
    let mut acl: *mut acl_s = 0 as *mut acl_s;
    let mut perm: libc::c_int = 0 as libc::c_int;
    let mut is_numeric_addr: libc::c_int = 0;
    let mut i: size_t = 0;
    let mut string_addr: [libc::c_char; 1024] = [0; 1024];
    let mut numeric_addr: [uint8_t; 16] = [0; 16];
    string_addr[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    if access_list.is_null() {
        return 1 as libc::c_int;
    }
    is_numeric_addr = (full_inet_pton(
        ip,
        &mut numeric_addr as *mut [uint8_t; 16] as *mut libc::c_void,
    ) > 0 as libc::c_int) as libc::c_int;
    let mut current_block_12: u64;
    i = 0 as libc::c_int as size_t;
    while i < (*access_list).count {
        acl = sblist_get(access_list, i) as *mut acl_s;
        match (*acl).h.type_0 as libc::c_uint {
            1 => {
                perm = acl_string_processing(acl, ip, addr, string_addr.as_mut_ptr());
                current_block_12 = 12599329904712511516;
            }
            2 => {
                if *ip.offset(0 as libc::c_int as isize) as libc::c_int == '\0' as i32 {
                    current_block_12 = 6483416627284290920;
                } else {
                    perm = if is_numeric_addr != 0 {
                        check_numeric_acl(acl, numeric_addr.as_mut_ptr())
                    } else {
                        -(1 as libc::c_int)
                    };
                    current_block_12 = 12599329904712511516;
                }
            }
            0 => {
                perm = -(1 as libc::c_int);
                current_block_12 = 12599329904712511516;
            }
            _ => {
                current_block_12 = 12599329904712511516;
            }
        }
        match current_block_12 {
            12599329904712511516 => {
                if perm == 0 as libc::c_int {
                    break;
                }
                if perm == 1 as libc::c_int {
                    return perm;
                }
            }
            _ => {}
        }
        i = i.wrapping_add(1);
        i;
    }
    log_message(
        5 as libc::c_int,
        b"Unauthorized connection from \"%s\".\0" as *const u8 as *const libc::c_char,
        ip,
    );
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn flush_access_list(mut access_list: acl_list_t) {
    let mut acl: *mut acl_s = 0 as *mut acl_s;
    let mut i: size_t = 0;
    if access_list.is_null() {
        return;
    }
    i = 0 as libc::c_int as size_t;
    while i < (*access_list).count {
        acl = sblist_get(access_list, i) as *mut acl_s;
        if (*acl).h.type_0 as libc::c_uint == HST_STRING as libc::c_int as libc::c_uint {
            free((*acl).h.address.string as *mut libc::c_void);
            (*acl).h.address.string = 0 as *mut libc::c_char;
        }
        i = i.wrapping_add(1);
        i;
    }
    sblist_free(access_list);
}
