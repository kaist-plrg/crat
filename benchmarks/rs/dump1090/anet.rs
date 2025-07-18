use ::libc;
extern "C" {
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
    fn connect(
        __fd: libc::c_int,
        __addr: *const sockaddr,
        __len: socklen_t,
    ) -> libc::c_int;
    fn getpeername(
        __fd: libc::c_int,
        __addr: *mut sockaddr,
        __len: *mut socklen_t,
    ) -> libc::c_int;
    fn setsockopt(
        __fd: libc::c_int,
        __level: libc::c_int,
        __optname: libc::c_int,
        __optval: *const libc::c_void,
        __optlen: socklen_t,
    ) -> libc::c_int;
    fn listen(__fd: libc::c_int, __n: libc::c_int) -> libc::c_int;
    fn accept(
        __fd: libc::c_int,
        __addr: *mut sockaddr,
        __addr_len: *mut socklen_t,
    ) -> libc::c_int;
    fn chmod(__file: *const libc::c_char, __mode: __mode_t) -> libc::c_int;
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
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn inet_ntoa(__in: in_addr) -> *mut libc::c_char;
    fn inet_aton(__cp: *const libc::c_char, __inp: *mut in_addr) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
    fn gethostbyname(__name: *const libc::c_char) -> *mut hostent;
    fn __errno_location() -> *mut libc::c_int;
    fn vsnprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
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
pub type __mode_t = libc::c_uint;
pub type __ssize_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
pub type mode_t = __mode_t;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;
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
pub struct sockaddr_un {
    pub sun_family: sa_family_t,
    pub sun_path: [libc::c_char; 108],
}
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
pub struct sockaddr_in {
    pub sin_family: sa_family_t,
    pub sin_port: in_port_t,
    pub sin_addr: in_addr,
    pub sin_zero: [libc::c_uchar; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hostent {
    pub h_name: *mut libc::c_char,
    pub h_aliases: *mut *mut libc::c_char,
    pub h_addrtype: libc::c_int,
    pub h_length: libc::c_int,
    pub h_addr_list: *mut *mut libc::c_char,
}
pub type va_list = __builtin_va_list;
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
unsafe extern "C" fn anetSetError(
    mut err: *mut libc::c_char,
    mut fmt: *const libc::c_char,
    mut args: ...
) {
    let mut ap: ::std::ffi::VaListImpl;
    if err.is_null() {
        return;
    }
    ap = args.clone();
    vsnprintf(err, 256 as libc::c_int as libc::c_ulong, fmt, ap.as_va_list());
}
pub unsafe extern "C" fn anetNonBlock(
    mut err: *mut libc::c_char,
    mut fd: libc::c_int,
) -> libc::c_int {
    let mut flags: libc::c_int = 0;
    flags = fcntl(fd, 3 as libc::c_int);
    if flags == -(1 as libc::c_int) {
        anetSetError(
            err,
            b"fcntl(F_GETFL): %s\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
        return -(1 as libc::c_int);
    }
    if fcntl(fd, 4 as libc::c_int, flags | 0o4000 as libc::c_int) == -(1 as libc::c_int)
    {
        anetSetError(
            err,
            b"fcntl(F_SETFL,O_NONBLOCK): %s\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn anetTcpNoDelay(
    mut err: *mut libc::c_char,
    mut fd: libc::c_int,
) -> libc::c_int {
    let mut yes: libc::c_int = 1 as libc::c_int;
    if setsockopt(
        fd,
        IPPROTO_TCP as libc::c_int,
        1 as libc::c_int,
        &mut yes as *mut libc::c_int as *const libc::c_void,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t,
    ) == -(1 as libc::c_int)
    {
        anetSetError(
            err,
            b"setsockopt TCP_NODELAY: %s\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn anetSetSendBuffer(
    mut err: *mut libc::c_char,
    mut fd: libc::c_int,
    mut buffsize: libc::c_int,
) -> libc::c_int {
    if setsockopt(
        fd,
        1 as libc::c_int,
        7 as libc::c_int,
        &mut buffsize as *mut libc::c_int as *const libc::c_void,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t,
    ) == -(1 as libc::c_int)
    {
        anetSetError(
            err,
            b"setsockopt SO_SNDBUF: %s\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn anetTcpKeepAlive(
    mut err: *mut libc::c_char,
    mut fd: libc::c_int,
) -> libc::c_int {
    let mut yes: libc::c_int = 1 as libc::c_int;
    if setsockopt(
        fd,
        1 as libc::c_int,
        9 as libc::c_int,
        &mut yes as *mut libc::c_int as *const libc::c_void,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t,
    ) == -(1 as libc::c_int)
    {
        anetSetError(
            err,
            b"setsockopt SO_KEEPALIVE: %s\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn anetResolve(
    mut err: *mut libc::c_char,
    mut host: *mut libc::c_char,
    mut ipbuf: *mut libc::c_char,
) -> libc::c_int {
    let mut sa: sockaddr_in = sockaddr_in {
        sin_family: 0,
        sin_port: 0,
        sin_addr: in_addr { s_addr: 0 },
        sin_zero: [0; 8],
    };
    sa.sin_family = 2 as libc::c_int as sa_family_t;
    if inet_aton(host, &mut sa.sin_addr) == 0 as libc::c_int {
        let mut he: *mut hostent = 0 as *mut hostent;
        he = gethostbyname(host);
        if he.is_null() {
            anetSetError(
                err,
                b"can't resolve: %s\0" as *const u8 as *const libc::c_char,
                host,
            );
            return -(1 as libc::c_int);
        }
        memcpy(
            &mut sa.sin_addr as *mut in_addr as *mut libc::c_void,
            *((*he).h_addr_list).offset(0 as libc::c_int as isize)
                as *const libc::c_void,
            ::std::mem::size_of::<in_addr>() as libc::c_ulong,
        );
    }
    strcpy(ipbuf, inet_ntoa(sa.sin_addr));
    return 0 as libc::c_int;
}
unsafe extern "C" fn anetCreateSocket(
    mut err: *mut libc::c_char,
    mut domain: libc::c_int,
) -> libc::c_int {
    let mut s: libc::c_int = 0;
    let mut on: libc::c_int = 1 as libc::c_int;
    s = socket(domain, SOCK_STREAM as libc::c_int, 0 as libc::c_int);
    if s == -(1 as libc::c_int) {
        anetSetError(
            err,
            b"creating socket: %s\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
        return -(1 as libc::c_int);
    }
    if setsockopt(
        s,
        1 as libc::c_int,
        2 as libc::c_int,
        &mut on as *mut libc::c_int as *const libc::c_void,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t,
    ) == -(1 as libc::c_int)
    {
        anetSetError(
            err,
            b"setsockopt SO_REUSEADDR: %s\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
        return -(1 as libc::c_int);
    }
    return s;
}
unsafe extern "C" fn anetTcpGenericConnect(
    mut err: *mut libc::c_char,
    mut addr: *mut libc::c_char,
    mut port: libc::c_int,
    mut flags: libc::c_int,
) -> libc::c_int {
    let mut s: libc::c_int = 0;
    let mut sa: sockaddr_in = sockaddr_in {
        sin_family: 0,
        sin_port: 0,
        sin_addr: in_addr { s_addr: 0 },
        sin_zero: [0; 8],
    };
    s = anetCreateSocket(err, 2 as libc::c_int);
    if s == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    sa.sin_family = 2 as libc::c_int as sa_family_t;
    sa.sin_port = __bswap_16(port as __uint16_t);
    if inet_aton(addr, &mut sa.sin_addr) == 0 as libc::c_int {
        let mut he: *mut hostent = 0 as *mut hostent;
        he = gethostbyname(addr);
        if he.is_null() {
            anetSetError(
                err,
                b"can't resolve: %s\0" as *const u8 as *const libc::c_char,
                addr,
            );
            close(s);
            return -(1 as libc::c_int);
        }
        memcpy(
            &mut sa.sin_addr as *mut in_addr as *mut libc::c_void,
            *((*he).h_addr_list).offset(0 as libc::c_int as isize)
                as *const libc::c_void,
            ::std::mem::size_of::<in_addr>() as libc::c_ulong,
        );
    }
    if flags & 1 as libc::c_int != 0 {
        if anetNonBlock(err, s) != 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
    }
    if connect(
        s,
        &mut sa as *mut sockaddr_in as *mut sockaddr,
        ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as socklen_t,
    ) == -(1 as libc::c_int)
    {
        if *__errno_location() == 115 as libc::c_int && flags & 1 as libc::c_int != 0 {
            return s;
        }
        anetSetError(
            err,
            b"connect: %s\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
        close(s);
        return -(1 as libc::c_int);
    }
    return s;
}
pub unsafe extern "C" fn anetTcpConnect(
    mut err: *mut libc::c_char,
    mut addr: *mut libc::c_char,
    mut port: libc::c_int,
) -> libc::c_int {
    return anetTcpGenericConnect(err, addr, port, 0 as libc::c_int);
}
pub unsafe extern "C" fn anetTcpNonBlockConnect(
    mut err: *mut libc::c_char,
    mut addr: *mut libc::c_char,
    mut port: libc::c_int,
) -> libc::c_int {
    return anetTcpGenericConnect(err, addr, port, 1 as libc::c_int);
}
pub unsafe extern "C" fn anetUnixGenericConnect(
    mut err: *mut libc::c_char,
    mut path: *mut libc::c_char,
    mut flags: libc::c_int,
) -> libc::c_int {
    let mut s: libc::c_int = 0;
    let mut sa: sockaddr_un = sockaddr_un {
        sun_family: 0,
        sun_path: [0; 108],
    };
    s = anetCreateSocket(err, 1 as libc::c_int);
    if s == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    sa.sun_family = 1 as libc::c_int as sa_family_t;
    strncpy(
        (sa.sun_path).as_mut_ptr(),
        path,
        (::std::mem::size_of::<[libc::c_char; 108]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    );
    if flags & 1 as libc::c_int != 0 {
        if anetNonBlock(err, s) != 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
    }
    if connect(
        s,
        &mut sa as *mut sockaddr_un as *mut sockaddr,
        ::std::mem::size_of::<sockaddr_un>() as libc::c_ulong as socklen_t,
    ) == -(1 as libc::c_int)
    {
        if *__errno_location() == 115 as libc::c_int && flags & 1 as libc::c_int != 0 {
            return s;
        }
        anetSetError(
            err,
            b"connect: %s\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
        close(s);
        return -(1 as libc::c_int);
    }
    return s;
}
pub unsafe extern "C" fn anetUnixConnect(
    mut err: *mut libc::c_char,
    mut path: *mut libc::c_char,
) -> libc::c_int {
    return anetUnixGenericConnect(err, path, 0 as libc::c_int);
}
pub unsafe extern "C" fn anetUnixNonBlockConnect(
    mut err: *mut libc::c_char,
    mut path: *mut libc::c_char,
) -> libc::c_int {
    return anetUnixGenericConnect(err, path, 1 as libc::c_int);
}
pub unsafe extern "C" fn anetRead(
    mut fd: libc::c_int,
    mut buf: *mut libc::c_char,
    mut count: libc::c_int,
) -> libc::c_int {
    let mut nread: libc::c_int = 0;
    let mut totlen: libc::c_int = 0 as libc::c_int;
    while totlen != count {
        nread = read(fd, buf as *mut libc::c_void, (count - totlen) as size_t)
            as libc::c_int;
        if nread == 0 as libc::c_int {
            return totlen;
        }
        if nread == -(1 as libc::c_int) {
            return -(1 as libc::c_int);
        }
        totlen += nread;
        buf = buf.offset(nread as isize);
    }
    return totlen;
}
pub unsafe extern "C" fn anetWrite(
    mut fd: libc::c_int,
    mut buf: *mut libc::c_char,
    mut count: libc::c_int,
) -> libc::c_int {
    let mut nwritten: libc::c_int = 0;
    let mut totlen: libc::c_int = 0 as libc::c_int;
    while totlen != count {
        nwritten = write(fd, buf as *const libc::c_void, (count - totlen) as size_t)
            as libc::c_int;
        if nwritten == 0 as libc::c_int {
            return totlen;
        }
        if nwritten == -(1 as libc::c_int) {
            return -(1 as libc::c_int);
        }
        totlen += nwritten;
        buf = buf.offset(nwritten as isize);
    }
    return totlen;
}
unsafe extern "C" fn anetListen(
    mut err: *mut libc::c_char,
    mut s: libc::c_int,
    mut sa: *mut sockaddr,
    mut len: socklen_t,
) -> libc::c_int {
    if bind(s, sa, len) == -(1 as libc::c_int) {
        anetSetError(
            err,
            b"bind: %s\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
        close(s);
        return -(1 as libc::c_int);
    }
    if listen(s, 511 as libc::c_int) == -(1 as libc::c_int) {
        anetSetError(
            err,
            b"listen: %s\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
        close(s);
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn anetTcpServer(
    mut err: *mut libc::c_char,
    mut port: libc::c_int,
    mut bindaddr: *mut libc::c_char,
) -> libc::c_int {
    let mut s: libc::c_int = 0;
    let mut sa: sockaddr_in = sockaddr_in {
        sin_family: 0,
        sin_port: 0,
        sin_addr: in_addr { s_addr: 0 },
        sin_zero: [0; 8],
    };
    s = anetCreateSocket(err, 2 as libc::c_int);
    if s == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    memset(
        &mut sa as *mut sockaddr_in as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong,
    );
    sa.sin_family = 2 as libc::c_int as sa_family_t;
    sa.sin_port = __bswap_16(port as __uint16_t);
    sa.sin_addr.s_addr = __bswap_32(0 as libc::c_int as in_addr_t);
    if !bindaddr.is_null() && inet_aton(bindaddr, &mut sa.sin_addr) == 0 as libc::c_int {
        anetSetError(err, b"invalid bind address\0" as *const u8 as *const libc::c_char);
        close(s);
        return -(1 as libc::c_int);
    }
    if anetListen(
        err,
        s,
        &mut sa as *mut sockaddr_in as *mut sockaddr,
        ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as socklen_t,
    ) == -(1 as libc::c_int)
    {
        return -(1 as libc::c_int);
    }
    return s;
}
pub unsafe extern "C" fn anetUnixServer(
    mut err: *mut libc::c_char,
    mut path: *mut libc::c_char,
    mut perm: mode_t,
) -> libc::c_int {
    let mut s: libc::c_int = 0;
    let mut sa: sockaddr_un = sockaddr_un {
        sun_family: 0,
        sun_path: [0; 108],
    };
    s = anetCreateSocket(err, 1 as libc::c_int);
    if s == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    memset(
        &mut sa as *mut sockaddr_un as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<sockaddr_un>() as libc::c_ulong,
    );
    sa.sun_family = 1 as libc::c_int as sa_family_t;
    strncpy(
        (sa.sun_path).as_mut_ptr(),
        path,
        (::std::mem::size_of::<[libc::c_char; 108]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    );
    if anetListen(
        err,
        s,
        &mut sa as *mut sockaddr_un as *mut sockaddr,
        ::std::mem::size_of::<sockaddr_un>() as libc::c_ulong as socklen_t,
    ) == -(1 as libc::c_int)
    {
        return -(1 as libc::c_int);
    }
    if perm != 0 {
        chmod((sa.sun_path).as_mut_ptr(), perm);
    }
    return s;
}
unsafe extern "C" fn anetGenericAccept(
    mut err: *mut libc::c_char,
    mut s: libc::c_int,
    mut sa: *mut sockaddr,
    mut len: *mut socklen_t,
) -> libc::c_int {
    let mut fd: libc::c_int = 0;
    loop {
        fd = accept(s, sa, len);
        if !(fd == -(1 as libc::c_int)) {
            break;
        }
        if *__errno_location() == 4 as libc::c_int {
            continue;
        }
        anetSetError(
            err,
            b"accept: %s\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
        return -(1 as libc::c_int);
    }
    return fd;
}
pub unsafe extern "C" fn anetTcpAccept(
    mut err: *mut libc::c_char,
    mut s: libc::c_int,
    mut ip: *mut libc::c_char,
    mut port: *mut libc::c_int,
) -> libc::c_int {
    let mut fd: libc::c_int = 0;
    let mut sa: sockaddr_in = sockaddr_in {
        sin_family: 0,
        sin_port: 0,
        sin_addr: in_addr { s_addr: 0 },
        sin_zero: [0; 8],
    };
    let mut salen: socklen_t = ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong
        as socklen_t;
    fd = anetGenericAccept(
        err,
        s,
        &mut sa as *mut sockaddr_in as *mut sockaddr,
        &mut salen,
    );
    if fd == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    if !ip.is_null() {
        strcpy(ip, inet_ntoa(sa.sin_addr));
    }
    if !port.is_null() {
        *port = __bswap_16(sa.sin_port) as libc::c_int;
    }
    return fd;
}
pub unsafe extern "C" fn anetUnixAccept(
    mut err: *mut libc::c_char,
    mut s: libc::c_int,
) -> libc::c_int {
    let mut fd: libc::c_int = 0;
    let mut sa: sockaddr_un = sockaddr_un {
        sun_family: 0,
        sun_path: [0; 108],
    };
    let mut salen: socklen_t = ::std::mem::size_of::<sockaddr_un>() as libc::c_ulong
        as socklen_t;
    fd = anetGenericAccept(
        err,
        s,
        &mut sa as *mut sockaddr_un as *mut sockaddr,
        &mut salen,
    );
    if fd == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    return fd;
}
pub unsafe extern "C" fn anetPeerToString(
    mut fd: libc::c_int,
    mut ip: *mut libc::c_char,
    mut port: *mut libc::c_int,
) -> libc::c_int {
    let mut sa: sockaddr_in = sockaddr_in {
        sin_family: 0,
        sin_port: 0,
        sin_addr: in_addr { s_addr: 0 },
        sin_zero: [0; 8],
    };
    let mut salen: socklen_t = ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong
        as socklen_t;
    if getpeername(fd, &mut sa as *mut sockaddr_in as *mut sockaddr, &mut salen)
        == -(1 as libc::c_int)
    {
        *port = 0 as libc::c_int;
        *ip.offset(0 as libc::c_int as isize) = '?' as i32 as libc::c_char;
        *ip.offset(1 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
        return -(1 as libc::c_int);
    }
    if !ip.is_null() {
        strcpy(ip, inet_ntoa(sa.sin_addr));
    }
    if !port.is_null() {
        *port = __bswap_16(sa.sin_port) as libc::c_int;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn anetSockName(
    mut fd: libc::c_int,
    mut ip: *mut libc::c_char,
    mut port: *mut libc::c_int,
) -> libc::c_int {
    let mut sa: sockaddr_in = sockaddr_in {
        sin_family: 0,
        sin_port: 0,
        sin_addr: in_addr { s_addr: 0 },
        sin_zero: [0; 8],
    };
    let mut salen: socklen_t = ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong
        as socklen_t;
    if getsockname(fd, &mut sa as *mut sockaddr_in as *mut sockaddr, &mut salen)
        == -(1 as libc::c_int)
    {
        *port = 0 as libc::c_int;
        *ip.offset(0 as libc::c_int as isize) = '?' as i32 as libc::c_char;
        *ip.offset(1 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
        return -(1 as libc::c_int);
    }
    if !ip.is_null() {
        strcpy(ip, inet_ntoa(sa.sin_addr));
    }
    if !port.is_null() {
        *port = __bswap_16(sa.sin_port) as libc::c_int;
    }
    return 0 as libc::c_int;
}
