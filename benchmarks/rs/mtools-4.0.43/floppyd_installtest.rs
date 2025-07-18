use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn free(__ptr: *mut libc::c_void);
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn puts(__s: *const libc::c_char) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn __errno_location() -> *mut libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn safe_malloc(size: size_t) -> *mut libc::c_void;
    fn atou16(str: *const libc::c_char) -> uint16_t;
    fn safePopenOut(
        command: *const *const libc::c_char,
        output: *mut libc::c_char,
        len: size_t,
    ) -> ssize_t;
    fn socket(
        __domain: libc::c_int,
        __type: libc::c_int,
        __protocol: libc::c_int,
    ) -> libc::c_int;
    fn connect(
        __fd: libc::c_int,
        __addr: *const sockaddr,
        __len: socklen_t,
    ) -> libc::c_int;
    fn setsockopt(
        __fd: libc::c_int,
        __level: libc::c_int,
        __optname: libc::c_int,
        __optval: *const libc::c_void,
        __optlen: socklen_t,
    ) -> libc::c_int;
    fn inet_addr(__cp: *const libc::c_char) -> in_addr_t;
    fn endhostent();
    fn gethostbyname(__name: *const libc::c_char) -> *mut hostent;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
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
pub type Byte = uint8_t;
pub type Dword = uint32_t;
pub type FloppydOpcodes = libc::c_uint;
pub const OP_SEEK64: FloppydOpcodes = 8;
pub const OP_OPRW: FloppydOpcodes = 7;
pub const OP_OPRO: FloppydOpcodes = 6;
pub const OP_IOCTL: FloppydOpcodes = 5;
pub const OP_CLOSE: FloppydOpcodes = 4;
pub const OP_FLUSH: FloppydOpcodes = 3;
pub const OP_SEEK: FloppydOpcodes = 2;
pub const OP_WRITE: FloppydOpcodes = 1;
pub const OP_READ: FloppydOpcodes = 0;
pub type AuthErrorsEnum = libc::c_uint;
pub const AUTH_IO_ERROR: AuthErrorsEnum = 6;
pub const AUTH_BADPACKET: AuthErrorsEnum = 5;
pub const AUTH_DEVLOCKED: AuthErrorsEnum = 4;
pub const AUTH_WRONGVERSION: AuthErrorsEnum = 3;
pub const AUTH_AUTHFAILED: AuthErrorsEnum = 2;
pub const AUTH_PACKETOVERSIZE: AuthErrorsEnum = 1;
pub const AUTH_SUCCESS: AuthErrorsEnum = 0;
#[inline]
unsafe extern "C" fn __bswap_16(mut __bsx: __uint16_t) -> __uint16_t {
    return (__bsx as libc::c_int >> 8 as libc::c_int & 0xff as libc::c_int
        | (__bsx as libc::c_int & 0xff as libc::c_int) << 8 as libc::c_int)
        as __uint16_t;
}
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
static mut AuthErrors: [*const libc::c_char; 5] = [
    b"Auth success!\0" as *const u8 as *const libc::c_char,
    b"Auth failed: Packet oversized!\0" as *const u8 as *const libc::c_char,
    b"Auth failed: X-Cookie doesn't match!\0" as *const u8 as *const libc::c_char,
    b"Auth failed: Wrong transmission protocol version!\0" as *const u8
        as *const libc::c_char,
    b"Auth failed: Device locked!\0" as *const u8 as *const libc::c_char,
];
#[inline]
unsafe extern "C" fn byte2dword(mut val: *mut Byte) -> Dword {
    let mut l: Dword = 0;
    l = (((*val.offset(0 as libc::c_int as isize) as libc::c_int) << 24 as libc::c_int)
        + ((*val.offset(1 as libc::c_int as isize) as libc::c_int) << 16 as libc::c_int)
        + ((*val.offset(2 as libc::c_int as isize) as libc::c_int) << 8 as libc::c_int)
        + *val.offset(3 as libc::c_int as isize) as libc::c_int) as Dword;
    return l;
}
unsafe extern "C" fn dword2byte(mut parm: Dword, mut rval: *mut Byte) {
    *rval
        .offset(
            0 as libc::c_int as isize,
        ) = (parm >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_uint) as Byte;
    *rval
        .offset(
            1 as libc::c_int as isize,
        ) = (parm >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint) as Byte;
    *rval
        .offset(
            2 as libc::c_int as isize,
        ) = (parm >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint) as Byte;
    *rval
        .offset(
            3 as libc::c_int as isize,
        ) = (parm & 0xff as libc::c_int as libc::c_uint) as Byte;
}
#[inline]
unsafe extern "C" fn read_dword(mut handle: libc::c_int) -> Dword {
    let mut val: [Byte; 4] = [0; 4];
    if read(
        handle,
        val.as_mut_ptr() as *mut libc::c_char as *mut libc::c_void,
        4 as libc::c_int as size_t,
    ) < 4 as libc::c_int as libc::c_long
    {
        return -(1 as libc::c_int) as Dword;
    }
    return byte2dword(val.as_mut_ptr());
}
unsafe extern "C" fn write_dword(
    mut handle: libc::c_int,
    mut parm: Dword,
) -> libc::c_int {
    let mut val: [Byte; 4] = [0; 4];
    dword2byte(parm, val.as_mut_ptr());
    if write(handle, val.as_mut_ptr() as *const libc::c_void, 4 as libc::c_int as size_t)
        < 4 as libc::c_int as libc::c_long
    {
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn authenticate_to_floppyd(
    mut fullauth: libc::c_char,
    mut sock: libc::c_int,
    mut display: *mut libc::c_char,
    mut protoversion: uint32_t,
) -> uint32_t {
    let mut filelen: uint32_t = 0 as libc::c_int as uint32_t;
    let mut buf: [Byte; 16] = [0; 16];
    let mut command: [*const libc::c_char; 6] = [
        b"xauth\0" as *const u8 as *const libc::c_char,
        b"xauth\0" as *const u8 as *const libc::c_char,
        b"extract\0" as *const u8 as *const libc::c_char,
        b"-\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut xcookie: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut errcode: Dword = 0;
    let mut bytesRead: uint32_t = 0;
    let mut cap: uint32_t = 0 as libc::c_int as uint32_t;
    if fullauth != 0 {
        command[4 as libc::c_int as usize] = display;
        filelen = strlen(display) as uint32_t;
        filelen = (filelen as libc::c_uint)
            .wrapping_add(100 as libc::c_int as libc::c_uint) as uint32_t as uint32_t;
        xcookie = safe_malloc(
            filelen.wrapping_add(4 as libc::c_int as libc::c_uint) as size_t,
        ) as *mut libc::c_char;
        filelen = safePopenOut(
            command.as_mut_ptr(),
            xcookie.offset(4 as libc::c_int as isize),
            filelen as size_t,
        ) as uint32_t;
        if filelen < 1 as libc::c_int as libc::c_uint {
            return AUTH_AUTHFAILED as libc::c_int as uint32_t;
        }
    }
    dword2byte(4 as libc::c_int as Dword, buf.as_mut_ptr());
    dword2byte(protoversion, buf.as_mut_ptr().offset(4 as libc::c_int as isize));
    if write(sock, buf.as_mut_ptr() as *const libc::c_void, 8 as libc::c_int as size_t)
        < 8 as libc::c_int as libc::c_long
    {
        return AUTH_IO_ERROR as libc::c_int as uint32_t;
    }
    bytesRead = read_dword(sock);
    if bytesRead != 4 as libc::c_int as libc::c_uint
        && bytesRead != 12 as libc::c_int as libc::c_uint
    {
        return AUTH_WRONGVERSION as libc::c_int as uint32_t;
    }
    errcode = read_dword(sock);
    if errcode != AUTH_SUCCESS as libc::c_int as libc::c_uint {
        return errcode;
    }
    protoversion = 10 as libc::c_int as uint32_t;
    if bytesRead >= 12 as libc::c_int as libc::c_uint {
        protoversion = read_dword(sock);
        cap = read_dword(sock);
    }
    fprintf(
        stderr,
        b"Protocol Version=%d\n\0" as *const u8 as *const libc::c_char,
        protoversion,
    );
    if protoversion >= 11 as libc::c_int as libc::c_uint {
        fprintf(
            stderr,
            b"Capabilities:%s%s\n\0" as *const u8 as *const libc::c_char,
            if cap & 1 as libc::c_int as libc::c_uint != 0 {
                b" ExplicitOpen\0" as *const u8 as *const libc::c_char
            } else {
                b"\0" as *const u8 as *const libc::c_char
            },
            if cap & 2 as libc::c_int as libc::c_uint != 0 {
                b" LargeFiles\0" as *const u8 as *const libc::c_char
            } else {
                b"\0" as *const u8 as *const libc::c_char
            },
        );
    }
    if fullauth != 0 {
        dword2byte(filelen, xcookie as *mut Byte);
        if write(
            sock,
            xcookie as *const libc::c_void,
            filelen.wrapping_add(4 as libc::c_int as libc::c_uint) as size_t,
        ) < filelen.wrapping_add(4 as libc::c_int as libc::c_uint) as ssize_t
        {
            return AUTH_IO_ERROR as libc::c_int as uint32_t;
        }
        if read_dword(sock) != 4 as libc::c_int as libc::c_uint {
            return AUTH_PACKETOVERSIZE as libc::c_int as uint32_t;
        }
        errcode = read_dword(sock);
    }
    return errcode;
}
unsafe extern "C" fn get_host_and_port(
    mut name: *const libc::c_char,
    mut hostname: *mut *mut libc::c_char,
    mut display: *mut *mut libc::c_char,
    mut port: *mut uint16_t,
) -> libc::c_int {
    let mut newname: *mut libc::c_char = strdup(name);
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p2: *mut libc::c_char = 0 as *mut libc::c_char;
    p = newname;
    while *p as libc::c_int != '/' as i32 && *p as libc::c_int != 0 {
        p = p.offset(1);
        p;
    }
    p2 = p;
    if *p != 0 {
        p = p.offset(1);
        p;
    }
    *p2 = 0 as libc::c_int as libc::c_char;
    *port = atou16(p);
    if *port as libc::c_int == 0 as libc::c_int {
        *port = 5703 as libc::c_int as uint16_t;
    }
    *display = strdup(newname);
    p = newname;
    while *p as libc::c_int != ':' as i32 && *p as libc::c_int != 0 {
        p = p.offset(1);
        p;
    }
    p2 = p;
    if *p != 0 {
        p = p.offset(1);
        p;
    }
    *p2 = 0 as libc::c_int as libc::c_char;
    *port = (*port as libc::c_int + atoi(p)) as uint16_t;
    if *newname == 0
        || strcmp(newname, b"unix\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
    {
        free(newname as *mut libc::c_void);
        newname = strdup(b"localhost\0" as *const u8 as *const libc::c_char);
    }
    *hostname = newname;
    return 1 as libc::c_int;
}
unsafe extern "C" fn getipaddress(mut ipaddr: *mut libc::c_char) -> in_addr_t {
    let mut host: *mut hostent = 0 as *mut hostent;
    let mut ip: in_addr_t = 0;
    ip = inet_addr(ipaddr);
    if ip == 0xffffffff as libc::c_uint
        && strcmp(ipaddr, b"255.255.255.255\0" as *const u8 as *const libc::c_char)
            != 0 as libc::c_int
    {
        host = gethostbyname(ipaddr);
        if !host.is_null() {
            memcpy(
                &mut ip as *mut in_addr_t as *mut libc::c_void,
                *((*host).h_addr_list).offset(0 as libc::c_int as isize)
                    as *const libc::c_void,
                ::std::mem::size_of::<in_addr_t>() as libc::c_ulong,
            );
        }
        endhostent();
    }
    return ip;
}
unsafe extern "C" fn connect_to_server(
    mut ip: in_addr_t,
    mut port: uint16_t,
) -> libc::c_int {
    let mut addr: sockaddr_in = sockaddr_in {
        sin_family: 0,
        sin_port: 0,
        sin_addr: in_addr { s_addr: 0 },
        sin_zero: [0; 8],
    };
    let mut sock: libc::c_int = 0;
    sock = socket(2 as libc::c_int, SOCK_STREAM as libc::c_int, 0 as libc::c_int);
    if sock < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    addr.sin_family = 2 as libc::c_int as sa_family_t;
    addr.sin_port = __bswap_16(port);
    addr.sin_addr.s_addr = ip;
    if connect(
        sock,
        &mut addr as *mut sockaddr_in as *mut sockaddr,
        ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as socklen_t,
    ) < 0 as libc::c_int
    {
        return -(1 as libc::c_int);
    }
    let mut on: libc::c_int = 1 as libc::c_int;
    setsockopt(
        0 as libc::c_int,
        1 as libc::c_int,
        9 as libc::c_int,
        &mut on as *mut libc::c_int as *mut libc::c_char as *const libc::c_void,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t,
    );
    return sock;
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut hostname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut display: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut port: uint16_t = 0;
    let mut sock: libc::c_int = 0;
    let mut reply: uint32_t = 0;
    let mut rval: libc::c_int = 0;
    let mut protoversion: uint32_t = 0;
    let mut fullauth: libc::c_char = 0 as libc::c_int as libc::c_char;
    let mut opcode: Byte = OP_CLOSE as libc::c_int as Byte;
    if argc < 2 as libc::c_int {
        puts(
            b"Usage: floppyd_installtest [-f] Connect-String\n-f\tDo full X-Cookie-Authentication\0"
                as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    name = *argv.offset(1 as libc::c_int as isize);
    if strcmp(name, b"-f\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        fullauth = 1 as libc::c_int as libc::c_char;
        name = *argv.offset(2 as libc::c_int as isize);
    }
    rval = get_host_and_port(name, &mut hostname, &mut display, &mut port);
    if rval == 0 {
        return -(1 as libc::c_int);
    }
    sock = connect_to_server(getipaddress(hostname), port);
    if sock == -(1 as libc::c_int) {
        fprintf(
            stderr,
            b"Can't connect to floppyd server on %s, port %i!\n\0" as *const u8
                as *const libc::c_char,
            hostname,
            port as libc::c_int,
        );
        return -(1 as libc::c_int);
    }
    protoversion = 11 as libc::c_int as uint32_t;
    loop {
        reply = authenticate_to_floppyd(fullauth, sock, display, protoversion);
        if protoversion == 10 as libc::c_int as libc::c_uint {
            break;
        }
        if !(reply == AUTH_WRONGVERSION as libc::c_int as libc::c_uint) {
            break;
        }
        protoversion = 10 as libc::c_int as uint32_t;
    }
    if reply != 0 as libc::c_int as libc::c_uint {
        fprintf(
            stderr,
            b"Connection to floppyd failed:\n%s\n\0" as *const u8 as *const libc::c_char,
            AuthErrors[reply as usize],
        );
        return -(1 as libc::c_int);
    }
    free(hostname as *mut libc::c_void);
    free(display as *mut libc::c_void);
    if write_dword(sock, 1 as libc::c_int as Dword) < 0 as libc::c_int {
        fprintf(
            stderr,
            b"Short write to floppyd:\n%s\n\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
    }
    if write(
        sock,
        &mut opcode as *mut Byte as *const libc::c_void,
        1 as libc::c_int as size_t,
    ) < 0 as libc::c_int as libc::c_long
    {
        fprintf(
            stderr,
            b"Short write to floppyd:\n%s\n\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
    }
    close(sock);
    return 0 as libc::c_int;
}
pub fn main() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args_os() {
        args.push(
            (::std::ffi::CString::new(
                ::std::os::unix::ffi::OsStrExt::as_bytes(arg.as_os_str()),
            ))
                .unwrap()
                .into_raw(),
        );
    }
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as libc::c_int,
                args.as_mut_ptr() as *mut *mut libc::c_char,
            ) as i32,
        )
    }
}
