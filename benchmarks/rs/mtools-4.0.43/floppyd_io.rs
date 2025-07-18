use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type doscp_t;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn perror(__s: *const libc::c_char);
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
    fn strtosi(
        nptr: *const libc::c_char,
        endptr: *mut *mut libc::c_char,
        base: libc::c_int,
    ) -> libc::c_int;
    fn strtou16(
        nptr: *const libc::c_char,
        endptr: *mut *mut libc::c_char,
        base: libc::c_int,
    ) -> uint16_t;
    fn printOom();
    fn init_head(Stream: *mut Stream_t, Class: *mut Class_t, Next: *mut Stream_t);
    static max_off_t_31: mt_off_t;
    static max_off_t_seek: mt_off_t;
    fn safePopenOut(
        command: *const *const libc::c_char,
        output: *mut libc::c_char,
        len: size_t,
    ) -> ssize_t;
    fn set_geom_noop(
        Stream: *mut Stream_t,
        dev: *mut device_t,
        orig_dev: *mut device_t,
    ) -> libc::c_int;
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
    fn shutdown(__fd: libc::c_int, __how: libc::c_int) -> libc::c_int;
    fn inet_addr(__cp: *const libc::c_char) -> in_addr_t;
    fn endhostent();
    fn gethostbyname(__name: *const libc::c_char) -> *mut hostent;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
pub type off_t = __off_t;
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
pub type size_t = libc::c_ulong;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Stream_t {
    pub Class: *mut Class_t,
    pub refs: libc::c_int,
    pub Next: *mut Stream_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Class_t {
    pub read: Option::<
        unsafe extern "C" fn(*mut Stream_t, *mut libc::c_char, size_t) -> ssize_t,
    >,
    pub write: Option::<
        unsafe extern "C" fn(*mut Stream_t, *mut libc::c_char, size_t) -> ssize_t,
    >,
    pub pread: Option::<
        unsafe extern "C" fn(
            *mut Stream_t,
            *mut libc::c_char,
            mt_off_t,
            size_t,
        ) -> ssize_t,
    >,
    pub pwrite: Option::<
        unsafe extern "C" fn(
            *mut Stream_t,
            *mut libc::c_char,
            mt_off_t,
            size_t,
        ) -> ssize_t,
    >,
    pub flush: Option::<unsafe extern "C" fn(*mut Stream_t) -> libc::c_int>,
    pub freeFunc: Option::<unsafe extern "C" fn(*mut Stream_t) -> libc::c_int>,
    pub set_geom: Option::<
        unsafe extern "C" fn(*mut Stream_t, *mut device_t, *mut device_t) -> libc::c_int,
    >,
    pub get_data: Option::<
        unsafe extern "C" fn(
            *mut Stream_t,
            *mut time_t,
            *mut mt_off_t,
            *mut libc::c_int,
            *mut uint32_t,
        ) -> libc::c_int,
    >,
    pub pre_allocate: Option::<
        unsafe extern "C" fn(*mut Stream_t, mt_off_t) -> libc::c_int,
    >,
    pub get_dosConvert: Option::<unsafe extern "C" fn(*mut Stream_t) -> *mut doscp_t>,
    pub discard: Option::<unsafe extern "C" fn(*mut Stream_t) -> libc::c_int>,
}
pub type mt_off_t = off_t;
pub type device_t = device;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct device {
    pub name: *const libc::c_char,
    pub drive: libc::c_char,
    pub fat_bits: libc::c_int,
    pub mode: libc::c_int,
    pub tracks: libc::c_uint,
    pub heads: uint16_t,
    pub sectors: uint16_t,
    pub hidden: libc::c_uint,
    pub offset: off_t,
    pub partition: libc::c_uint,
    pub misc_flags: libc::c_uint,
    pub ssize: uint8_t,
    pub use_2m: libc::c_uint,
    pub precmd: *mut libc::c_char,
    pub file_nr: libc::c_int,
    pub blocksize: libc::c_uint,
    pub codepage: libc::c_uint,
    pub data_map: *const libc::c_char,
    pub tot_sectors: uint32_t,
    pub sector_size: uint16_t,
    pub postcmd: *mut libc::c_char,
    pub cfg_filename: *const libc::c_char,
}
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
pub type Byte = uint8_t;
pub type Dword = uint32_t;
pub type Qword = uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RemoteFile_t {
    pub head: Stream_t,
    pub fd: libc::c_int,
    pub offset: mt_off_t,
    pub lastwhere: mt_off_t,
    pub size: mt_off_t,
    pub version: libc::c_uint,
    pub capabilities: libc::c_uint,
    pub drive: libc::c_int,
}
pub const AUTH_IO_ERROR: AuthErrorsEnum = 6;
pub const OP_OPRW: FloppydOpcodes = 7;
pub const OP_OPRO: FloppydOpcodes = 6;
pub const AUTH_WRONGVERSION: AuthErrorsEnum = 3;
pub const AUTH_PACKETOVERSIZE: AuthErrorsEnum = 1;
pub const AUTH_SUCCESS: AuthErrorsEnum = 0;
pub const AUTH_AUTHFAILED: AuthErrorsEnum = 2;
pub const OP_CLOSE: FloppydOpcodes = 4;
pub const OP_FLUSH: FloppydOpcodes = 3;
pub const OP_WRITE: FloppydOpcodes = 1;
pub type iofn = Option::<
    unsafe extern "C" fn(libc::c_int, *mut libc::c_char, uint32_t) -> ssize_t,
>;
pub const OP_SEEK: FloppydOpcodes = 2;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SQwordRet {
    pub v: int64_t,
    pub err: libc::c_int,
}
pub const OP_SEEK64: FloppydOpcodes = 8;
pub const OP_READ: FloppydOpcodes = 0;
pub type FloppydOpcodes = libc::c_uint;
pub const OP_IOCTL: FloppydOpcodes = 5;
pub type AuthErrorsEnum = libc::c_uint;
pub const AUTH_BADPACKET: AuthErrorsEnum = 5;
pub const AUTH_DEVLOCKED: AuthErrorsEnum = 4;
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
#[inline]
unsafe extern "C" fn cork(mut sockhandle: libc::c_int, mut on: libc::c_int) {
    if setsockopt(
        sockhandle,
        IPPROTO_TCP as libc::c_int,
        3 as libc::c_int,
        &mut on as *mut libc::c_int as *mut libc::c_char as *const libc::c_void,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t,
    ) < 0 as libc::c_int
    {
        perror(b"setsockopt cork\0" as *const u8 as *const libc::c_char);
    }
}
static mut AuthErrors: [*const libc::c_char; 7] = [
    b"Auth success\0" as *const u8 as *const libc::c_char,
    b"Auth failed: Packet oversized\0" as *const u8 as *const libc::c_char,
    b"Auth failed: X-Cookie doesn't match\0" as *const u8 as *const libc::c_char,
    b"Auth failed: Wrong transmission protocol version\0" as *const u8
        as *const libc::c_char,
    b"Auth failed: Device locked\0" as *const u8 as *const libc::c_char,
    b"Auth failed: Bad packet\0" as *const u8 as *const libc::c_char,
    b"Auth failed: I/O Error\0" as *const u8 as *const libc::c_char,
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
#[inline]
unsafe extern "C" fn byte2sdword(mut val: *mut Byte) -> int32_t {
    let mut l: int32_t = 0;
    l = ((*val.offset(0 as libc::c_int as isize) as libc::c_int) << 24 as libc::c_int)
        + ((*val.offset(1 as libc::c_int as isize) as libc::c_int) << 16 as libc::c_int)
        + ((*val.offset(2 as libc::c_int as isize) as libc::c_int) << 8 as libc::c_int)
        + *val.offset(3 as libc::c_int as isize) as libc::c_int;
    return l;
}
#[inline]
unsafe extern "C" fn byte2qword(mut val: *mut Byte) -> Qword {
    let mut l: Qword = 0;
    l = *val.offset(0 as libc::c_int as isize) as Qword;
    l = l << 8 as libc::c_int | *val.offset(1 as libc::c_int as isize) as libc::c_ulong;
    l = l << 8 as libc::c_int | *val.offset(2 as libc::c_int as isize) as libc::c_ulong;
    l = l << 8 as libc::c_int | *val.offset(3 as libc::c_int as isize) as libc::c_ulong;
    l = l << 8 as libc::c_int | *val.offset(4 as libc::c_int as isize) as libc::c_ulong;
    l = l << 8 as libc::c_int | *val.offset(5 as libc::c_int as isize) as libc::c_ulong;
    l = l << 8 as libc::c_int | *val.offset(6 as libc::c_int as isize) as libc::c_ulong;
    l = l << 8 as libc::c_int | *val.offset(7 as libc::c_int as isize) as libc::c_ulong;
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
unsafe extern "C" fn sdword2byte(mut parm: int32_t, mut rval: *mut Byte) {
    *rval
        .offset(
            0 as libc::c_int as isize,
        ) = (parm >> 24 as libc::c_int & 0xff as libc::c_int) as Byte;
    *rval
        .offset(
            1 as libc::c_int as isize,
        ) = (parm >> 16 as libc::c_int & 0xff as libc::c_int) as Byte;
    *rval
        .offset(
            2 as libc::c_int as isize,
        ) = (parm >> 8 as libc::c_int & 0xff as libc::c_int) as Byte;
    *rval.offset(3 as libc::c_int as isize) = (parm & 0xff as libc::c_int) as Byte;
}
#[inline]
unsafe extern "C" fn qword2byte(mut parm: Qword, mut rval: *mut Byte) {
    *rval
        .offset(
            0 as libc::c_int as isize,
        ) = (parm >> 56 as libc::c_int & 0xff as libc::c_int as libc::c_ulong) as Byte;
    *rval
        .offset(
            1 as libc::c_int as isize,
        ) = (parm >> 48 as libc::c_int & 0xff as libc::c_int as libc::c_ulong) as Byte;
    *rval
        .offset(
            2 as libc::c_int as isize,
        ) = (parm >> 40 as libc::c_int & 0xff as libc::c_int as libc::c_ulong) as Byte;
    *rval
        .offset(
            3 as libc::c_int as isize,
        ) = (parm >> 32 as libc::c_int & 0xff as libc::c_int as libc::c_ulong) as Byte;
    *rval
        .offset(
            4 as libc::c_int as isize,
        ) = (parm >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_ulong) as Byte;
    *rval
        .offset(
            5 as libc::c_int as isize,
        ) = (parm >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_ulong) as Byte;
    *rval
        .offset(
            6 as libc::c_int as isize,
        ) = (parm >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_ulong) as Byte;
    *rval
        .offset(
            7 as libc::c_int as isize,
        ) = (parm & 0xff as libc::c_int as libc::c_ulong) as Byte;
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
#[inline]
unsafe extern "C" fn read_sdword(mut handle: libc::c_int) -> int32_t {
    let mut val: [Byte; 4] = [0; 4];
    if read(
        handle,
        val.as_mut_ptr() as *mut libc::c_char as *mut libc::c_void,
        4 as libc::c_int as size_t,
    ) < 4 as libc::c_int as libc::c_long
    {
        return -(1 as libc::c_int);
    }
    return byte2sdword(val.as_mut_ptr());
}
#[inline]
unsafe extern "C" fn read_sqword(mut handle: libc::c_int) -> SQwordRet {
    let mut val: [Byte; 8] = [0; 8];
    let mut ret: SQwordRet = SQwordRet { v: 0, err: 0 };
    if read(
        handle,
        val.as_mut_ptr() as *mut libc::c_char as *mut libc::c_void,
        8 as libc::c_int as size_t,
    ) < 8 as libc::c_int as libc::c_long
    {
        ret.err = -(1 as libc::c_int);
    } else {
        ret.v = byte2qword(val.as_mut_ptr()) as int64_t;
        ret.err = 0 as libc::c_int;
    }
    return ret;
}
unsafe extern "C" fn authenticate_to_floppyd(
    mut floppyd: *mut RemoteFile_t,
    mut sock: libc::c_int,
    mut display: *mut libc::c_char,
) -> libc::c_uint {
    let mut cookielen: size_t = 0;
    let mut filelen: uint16_t = 0;
    let mut newlen: ssize_t = 0;
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
    let mut l: libc::c_int = 0;
    command[4 as libc::c_int as usize] = display;
    cookielen = strlen(display);
    cookielen = (cookielen as libc::c_ulong)
        .wrapping_add(100 as libc::c_int as libc::c_ulong) as size_t as size_t;
    xcookie = safe_malloc(cookielen.wrapping_add(4 as libc::c_int as libc::c_ulong))
        as *mut libc::c_char;
    newlen = safePopenOut(
        command.as_mut_ptr(),
        xcookie.offset(4 as libc::c_int as isize),
        cookielen,
    );
    if newlen < 1 as libc::c_int as libc::c_long
        || newlen > 65535 as libc::c_int as libc::c_long
    {
        return AUTH_AUTHFAILED as libc::c_int as libc::c_uint;
    }
    filelen = newlen as uint16_t;
    dword2byte(4 as libc::c_int as Dword, buf.as_mut_ptr());
    dword2byte((*floppyd).version, buf.as_mut_ptr().offset(4 as libc::c_int as isize));
    if write(sock, buf.as_mut_ptr() as *const libc::c_void, 8 as libc::c_int as size_t)
        < 8 as libc::c_int as libc::c_long
    {
        return AUTH_IO_ERROR as libc::c_int as libc::c_uint;
    }
    l = read_dword(sock) as libc::c_int;
    if l < 4 as libc::c_int {
        return AUTH_WRONGVERSION as libc::c_int as libc::c_uint;
    }
    errcode = read_dword(sock);
    if errcode != AUTH_SUCCESS as libc::c_int as libc::c_uint {
        return errcode;
    }
    if l >= 8 as libc::c_int {
        (*floppyd).version = read_dword(sock);
    }
    if l >= 12 as libc::c_int {
        (*floppyd).capabilities = read_dword(sock);
    }
    dword2byte(filelen as Dword, xcookie as *mut Byte);
    if write(
        sock,
        xcookie as *const libc::c_void,
        (filelen as libc::c_int + 4 as libc::c_int) as size_t,
    ) < (filelen as libc::c_int + 4 as libc::c_int) as ssize_t
    {
        return AUTH_IO_ERROR as libc::c_int as libc::c_uint;
    }
    if read_dword(sock) != 4 as libc::c_int as libc::c_uint {
        return AUTH_PACKETOVERSIZE as libc::c_int as libc::c_uint;
    }
    errcode = read_dword(sock);
    return errcode;
}
unsafe extern "C" fn floppyd_reader(
    mut fd: libc::c_int,
    mut buffer: *mut libc::c_char,
    mut len: uint32_t,
) -> ssize_t {
    let mut errcode: Dword = 0;
    let mut gotlen: Dword = 0;
    let mut buf: [Byte; 16] = [0; 16];
    dword2byte(1 as libc::c_int as Dword, buf.as_mut_ptr());
    buf[4 as libc::c_int as usize] = OP_READ as libc::c_int as Byte;
    dword2byte(
        4 as libc::c_int as Dword,
        buf.as_mut_ptr().offset(5 as libc::c_int as isize),
    );
    dword2byte(len, buf.as_mut_ptr().offset(9 as libc::c_int as isize));
    if write(fd, buf.as_mut_ptr() as *const libc::c_void, 13 as libc::c_int as size_t)
        < 13 as libc::c_int as libc::c_long
    {
        return AUTH_IO_ERROR as libc::c_int as ssize_t;
    }
    if read_dword(fd) != 8 as libc::c_int as libc::c_uint {
        *__errno_location() = 5 as libc::c_int;
        return -(1 as libc::c_int) as ssize_t;
    }
    gotlen = read_dword(fd);
    errcode = read_dword(fd);
    if gotlen != -(1 as libc::c_int) as Dword {
        let mut l: size_t = 0;
        let mut start: libc::c_uint = 0;
        if read_dword(fd) != gotlen {
            *__errno_location() = 5 as libc::c_int;
            return -(1 as libc::c_int) as ssize_t;
        }
        start = 0 as libc::c_int as libc::c_uint;
        l = 0 as libc::c_int as size_t;
        while start < gotlen {
            let mut ret: ssize_t = read(
                fd,
                buffer.offset(start as isize) as *mut libc::c_void,
                gotlen.wrapping_sub(start) as size_t,
            );
            if ret < 0 as libc::c_int as libc::c_long {
                return -(1 as libc::c_int) as ssize_t;
            }
            if ret == 0 as libc::c_int as libc::c_long {
                *__errno_location() = 5 as libc::c_int;
                return -(1 as libc::c_int) as ssize_t;
            }
            l = ret as size_t;
            start = (start as libc::c_ulong).wrapping_add(l) as libc::c_uint
                as libc::c_uint;
        }
    } else {
        *__errno_location() = errcode as libc::c_int;
    }
    return gotlen as ssize_t;
}
unsafe extern "C" fn floppyd_writer(
    mut fd: libc::c_int,
    mut buffer: *mut libc::c_char,
    mut len: uint32_t,
) -> ssize_t {
    let mut errcode: libc::c_int = 0;
    let mut gotlen: int32_t = 0;
    let mut buf: [Byte; 16] = [0; 16];
    let mut ret: ssize_t = 0;
    dword2byte(1 as libc::c_int as Dword, buf.as_mut_ptr());
    buf[4 as libc::c_int as usize] = OP_WRITE as libc::c_int as Byte;
    dword2byte(len, buf.as_mut_ptr().offset(5 as libc::c_int as isize));
    cork(fd, 1 as libc::c_int);
    if write(fd, buf.as_mut_ptr() as *const libc::c_void, 9 as libc::c_int as size_t)
        < 9 as libc::c_int as libc::c_long
    {
        return AUTH_IO_ERROR as libc::c_int as ssize_t;
    }
    ret = write(fd, buffer as *const libc::c_void, len as size_t);
    if ret == -(1 as libc::c_int) as libc::c_long
        || (ret as size_t) < len as libc::c_ulong
    {
        return AUTH_IO_ERROR as libc::c_int as ssize_t;
    }
    cork(fd, 0 as libc::c_int);
    if read_dword(fd) != 8 as libc::c_int as libc::c_uint {
        *__errno_location() = 5 as libc::c_int;
        return -(1 as libc::c_int) as ssize_t;
    }
    gotlen = read_sdword(fd);
    errcode = read_sdword(fd);
    *__errno_location() = errcode;
    if *__errno_location() != 0 as libc::c_int && gotlen == 0 as libc::c_int {
        if *__errno_location() == 9 as libc::c_int {
            *__errno_location() = 30 as libc::c_int;
        }
        gotlen = -(1 as libc::c_int);
    }
    return gotlen as ssize_t;
}
unsafe extern "C" fn floppyd_lseek(
    mut fd: libc::c_int,
    mut offset: int32_t,
    mut whence: libc::c_int,
) -> libc::c_int {
    let mut errcode: libc::c_int = 0;
    let mut gotlen: libc::c_int = 0;
    let mut buf: [Byte; 32] = [0; 32];
    dword2byte(1 as libc::c_int as Dword, buf.as_mut_ptr());
    buf[4 as libc::c_int as usize] = OP_SEEK as libc::c_int as Byte;
    dword2byte(
        8 as libc::c_int as Dword,
        buf.as_mut_ptr().offset(5 as libc::c_int as isize),
    );
    sdword2byte(offset, buf.as_mut_ptr().offset(9 as libc::c_int as isize));
    sdword2byte(whence, buf.as_mut_ptr().offset(13 as libc::c_int as isize));
    if write(fd, buf.as_mut_ptr() as *const libc::c_void, 17 as libc::c_int as size_t)
        < 17 as libc::c_int as libc::c_long
    {
        return AUTH_IO_ERROR as libc::c_int;
    }
    if read_dword(fd) != 8 as libc::c_int as libc::c_uint {
        *__errno_location() = 5 as libc::c_int;
        return -(1 as libc::c_int);
    }
    gotlen = read_sdword(fd);
    errcode = read_sdword(fd);
    *__errno_location() = errcode;
    return gotlen;
}
unsafe extern "C" fn floppyd_lseek64(
    mut fd: libc::c_int,
    mut offset: mt_off_t,
    mut whence: libc::c_int,
) -> mt_off_t {
    let mut errcode: libc::c_int = 0;
    let mut gotlen: SQwordRet = SQwordRet { v: 0, err: 0 };
    let mut buf: [Byte; 32] = [0; 32];
    dword2byte(1 as libc::c_int as Dword, buf.as_mut_ptr());
    buf[4 as libc::c_int as usize] = OP_SEEK64 as libc::c_int as Byte;
    dword2byte(
        12 as libc::c_int as Dword,
        buf.as_mut_ptr().offset(5 as libc::c_int as isize),
    );
    qword2byte(
        offset as uint32_t as Qword,
        buf.as_mut_ptr().offset(9 as libc::c_int as isize),
    );
    sdword2byte(whence, buf.as_mut_ptr().offset(17 as libc::c_int as isize));
    if write(fd, buf.as_mut_ptr() as *const libc::c_void, 21 as libc::c_int as size_t)
        < 21 as libc::c_int as libc::c_long
    {
        return AUTH_IO_ERROR as libc::c_int as mt_off_t;
    }
    if read_dword(fd) != 12 as libc::c_int as libc::c_uint {
        *__errno_location() = 5 as libc::c_int;
        return -(1 as libc::c_int) as mt_off_t;
    }
    gotlen = read_sqword(fd);
    errcode = read_sdword(fd);
    *__errno_location() = errcode;
    return gotlen.v;
}
unsafe extern "C" fn floppyd_open(
    mut This: *mut RemoteFile_t,
    mut mode: libc::c_int,
) -> libc::c_int {
    let mut errcode: libc::c_int = 0;
    let mut gotlen: libc::c_int = 0;
    let mut buf: [Byte; 16] = [0; 16];
    if (*This).capabilities & 1 as libc::c_int as libc::c_uint == 0 {
        return 0 as libc::c_int;
    }
    dword2byte(1 as libc::c_int as Dword, buf.as_mut_ptr());
    if mode & 0o3 as libc::c_int == 0 as libc::c_int {
        buf[4 as libc::c_int as usize] = OP_OPRO as libc::c_int as Byte;
    } else {
        buf[4 as libc::c_int as usize] = OP_OPRW as libc::c_int as Byte;
    }
    dword2byte(
        4 as libc::c_int as Dword,
        buf.as_mut_ptr().offset(5 as libc::c_int as isize),
    );
    sdword2byte((*This).drive, buf.as_mut_ptr().offset(9 as libc::c_int as isize));
    if write(
        (*This).fd,
        buf.as_mut_ptr() as *const libc::c_void,
        13 as libc::c_int as size_t,
    ) < 13 as libc::c_int as libc::c_long
    {
        return AUTH_IO_ERROR as libc::c_int;
    }
    if read_dword((*This).fd) != 8 as libc::c_int as libc::c_uint {
        *__errno_location() = 5 as libc::c_int;
        return -(1 as libc::c_int);
    }
    gotlen = read_sdword((*This).fd);
    errcode = read_sdword((*This).fd);
    *__errno_location() = errcode;
    return gotlen;
}
unsafe extern "C" fn floppyd_io(
    mut Stream: *mut Stream_t,
    mut buf: *mut libc::c_char,
    mut where_0: mt_off_t,
    mut len: size_t,
    mut io: iofn,
) -> ssize_t {
    let mut This: *mut RemoteFile_t = Stream as *mut RemoteFile_t;
    let mut ret: ssize_t = 0;
    where_0 += (*This).offset;
    if where_0 != (*This).lastwhere {
        if (*This).capabilities & 2 as libc::c_int as libc::c_uint != 0 {
            if floppyd_lseek64((*This).fd, where_0, 0 as libc::c_int)
                < 0 as libc::c_int as libc::c_long
            {
                perror(b"floppyd_lseek64\0" as *const u8 as *const libc::c_char);
                (*This).lastwhere = -(1 as libc::c_int) as mt_off_t;
                return -(1 as libc::c_int) as ssize_t;
            }
        } else {
            if where_0 > 2147483647 as libc::c_int as libc::c_long
                || where_0
                    < (-(2147483647 as libc::c_int) - 1 as libc::c_int) as libc::c_long
            {
                fprintf(
                    stderr,
                    b"Seek position out of range\n\0" as *const u8 as *const libc::c_char,
                );
                return -(1 as libc::c_int) as ssize_t;
            }
            if floppyd_lseek((*This).fd, where_0 as int32_t, 0 as libc::c_int)
                < 0 as libc::c_int
            {
                perror(b"floppyd_lseek\0" as *const u8 as *const libc::c_char);
                (*This).lastwhere = -(1 as libc::c_int) as mt_off_t;
                return -(1 as libc::c_int) as ssize_t;
            }
        }
    }
    ret = io
        .unwrap()(
        (*This).fd,
        buf,
        if len > 2147483647 as libc::c_int as libc::c_ulong {
            (2147483647 as libc::c_int as uint32_t)
                .wrapping_add(1 as libc::c_int as libc::c_uint)
        } else {
            len as uint32_t
        },
    );
    if ret == -(1 as libc::c_int) as libc::c_long {
        perror(b"floppyd_io\0" as *const u8 as *const libc::c_char);
        (*This).lastwhere = -(1 as libc::c_int) as mt_off_t;
        return -(1 as libc::c_int) as ssize_t;
    }
    (*This).lastwhere = where_0 + ret;
    return ret;
}
unsafe extern "C" fn floppyd_pread(
    mut Stream: *mut Stream_t,
    mut buf: *mut libc::c_char,
    mut where_0: mt_off_t,
    mut len: size_t,
) -> ssize_t {
    return floppyd_io(
        Stream,
        buf,
        where_0,
        len,
        Some(
            floppyd_reader
                as unsafe extern "C" fn(
                    libc::c_int,
                    *mut libc::c_char,
                    uint32_t,
                ) -> ssize_t,
        ),
    );
}
unsafe extern "C" fn floppyd_pwrite(
    mut Stream: *mut Stream_t,
    mut buf: *mut libc::c_char,
    mut where_0: mt_off_t,
    mut len: size_t,
) -> ssize_t {
    return floppyd_io(
        Stream,
        buf,
        where_0,
        len,
        Some(
            floppyd_writer
                as unsafe extern "C" fn(
                    libc::c_int,
                    *mut libc::c_char,
                    uint32_t,
                ) -> ssize_t,
        ),
    );
}
unsafe extern "C" fn floppyd_flush(mut Stream: *mut Stream_t) -> libc::c_int {
    let mut buf: [Byte; 16] = [0; 16];
    let mut This: *mut RemoteFile_t = Stream as *mut RemoteFile_t;
    dword2byte(1 as libc::c_int as Dword, buf.as_mut_ptr());
    buf[4 as libc::c_int as usize] = OP_FLUSH as libc::c_int as Byte;
    dword2byte(
        1 as libc::c_int as Dword,
        buf.as_mut_ptr().offset(5 as libc::c_int as isize),
    );
    buf[9 as libc::c_int as usize] = '\0' as i32 as Byte;
    if write(
        (*This).fd,
        buf.as_mut_ptr() as *const libc::c_void,
        10 as libc::c_int as size_t,
    ) < 10 as libc::c_int as libc::c_long
    {
        return AUTH_IO_ERROR as libc::c_int;
    }
    if read_dword((*This).fd) != 8 as libc::c_int as libc::c_uint {
        *__errno_location() = 5 as libc::c_int;
        return -(1 as libc::c_int);
    }
    read_dword((*This).fd);
    read_dword((*This).fd);
    return 0 as libc::c_int;
}
unsafe extern "C" fn floppyd_free(mut Stream: *mut Stream_t) -> libc::c_int {
    let mut buf: [Byte; 16] = [0; 16];
    let mut gotlen: libc::c_int = 0;
    let mut errcode: libc::c_int = 0;
    let mut This: *mut RemoteFile_t = Stream as *mut RemoteFile_t;
    if (*This).fd > 2 as libc::c_int {
        dword2byte(1 as libc::c_int as Dword, buf.as_mut_ptr());
        buf[4 as libc::c_int as usize] = OP_CLOSE as libc::c_int as Byte;
        if write(
            (*This).fd,
            buf.as_mut_ptr() as *const libc::c_void,
            5 as libc::c_int as size_t,
        ) < 5 as libc::c_int as libc::c_long
        {
            return AUTH_IO_ERROR as libc::c_int;
        }
        shutdown((*This).fd, 1 as libc::c_int);
        if read_dword((*This).fd) != 8 as libc::c_int as libc::c_uint {
            *__errno_location() = 5 as libc::c_int;
            return -(1 as libc::c_int);
        }
        gotlen = read_sdword((*This).fd);
        errcode = read_sdword((*This).fd);
        *__errno_location() = errcode;
        close((*This).fd);
        return gotlen;
    } else {
        return 0 as libc::c_int
    };
}
unsafe extern "C" fn floppyd_data(
    mut Stream: *mut Stream_t,
    mut date: *mut time_t,
    mut size: *mut mt_off_t,
    mut type_0: *mut libc::c_int,
    mut address: *mut uint32_t,
) -> libc::c_int {
    let mut This: *mut RemoteFile_t = Stream as *mut RemoteFile_t;
    if !date.is_null() {
        *date = 0 as libc::c_int as time_t;
    }
    if !size.is_null() {
        *size = (*This).size;
    }
    if !type_0.is_null() {
        *type_0 = 0 as libc::c_int;
    }
    if !address.is_null() {
        *address = 0 as libc::c_int as uint32_t;
    }
    return 0 as libc::c_int;
}
static mut FloppydFileClass: Class_t = unsafe {
    {
        let mut init = Class_t {
            read: None,
            write: None,
            pread: Some(
                floppyd_pread
                    as unsafe extern "C" fn(
                        *mut Stream_t,
                        *mut libc::c_char,
                        mt_off_t,
                        size_t,
                    ) -> ssize_t,
            ),
            pwrite: Some(
                floppyd_pwrite
                    as unsafe extern "C" fn(
                        *mut Stream_t,
                        *mut libc::c_char,
                        mt_off_t,
                        size_t,
                    ) -> ssize_t,
            ),
            flush: Some(
                floppyd_flush as unsafe extern "C" fn(*mut Stream_t) -> libc::c_int,
            ),
            freeFunc: Some(
                floppyd_free as unsafe extern "C" fn(*mut Stream_t) -> libc::c_int,
            ),
            set_geom: Some(
                set_geom_noop
                    as unsafe extern "C" fn(
                        *mut Stream_t,
                        *mut device_t,
                        *mut device_t,
                    ) -> libc::c_int,
            ),
            get_data: Some(
                floppyd_data
                    as unsafe extern "C" fn(
                        *mut Stream_t,
                        *mut time_t,
                        *mut mt_off_t,
                        *mut libc::c_int,
                        *mut uint32_t,
                    ) -> libc::c_int,
            ),
            pre_allocate: None,
            get_dosConvert: None,
            discard: None,
        };
        init
    }
};
unsafe extern "C" fn get_host_and_port_and_drive(
    mut name: *const libc::c_char,
    mut hostname: *mut *mut libc::c_char,
    mut display: *mut *mut libc::c_char,
    mut port: *mut uint16_t,
    mut drive: *mut libc::c_int,
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
    *port = 5703 as libc::c_int as uint16_t;
    if *p as libc::c_int >= '0' as i32 && *p as libc::c_int <= '9' as i32 {
        *port = strtou16(p, &mut p, 0 as libc::c_int);
    }
    if *p as libc::c_int == '/' as i32 {
        p = p.offset(1);
        p;
    }
    *drive = 0 as libc::c_int;
    if *p as libc::c_int >= '0' as i32 && *p as libc::c_int <= '9' as i32 {
        *drive = strtosi(p, &mut p, 0 as libc::c_int);
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
        sock,
        1 as libc::c_int,
        9 as libc::c_int,
        &mut on as *mut libc::c_int as *mut libc::c_char as *const libc::c_void,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t,
    );
    return sock;
}
pub unsafe extern "C" fn FloppydOpen(
    mut dev: *mut device,
    mut name: *const libc::c_char,
    mut mode: libc::c_int,
    mut errmsg: *mut libc::c_char,
    mut maxSize: *mut mt_off_t,
) -> *mut Stream_t {
    let mut This: *mut RemoteFile_t = 0 as *mut RemoteFile_t;
    if dev.is_null() || (*dev).misc_flags & 0x40 as libc::c_uint == 0 {
        return 0 as *mut Stream_t;
    }
    This = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<RemoteFile_t>() as libc::c_ulong,
    ) as *mut RemoteFile_t;
    if This.is_null() {
        printOom();
        return 0 as *mut Stream_t;
    }
    init_head(&mut (*This).head, &mut FloppydFileClass, 0 as *mut Stream_t);
    (*This).offset = 0 as libc::c_int as mt_off_t;
    (*This).lastwhere = 0 as libc::c_int as mt_off_t;
    (*This).fd = ConnectToFloppyd(This, name, errmsg);
    if (*This).fd == -(1 as libc::c_int) {
        free(This as *mut libc::c_char as *mut libc::c_void);
        return 0 as *mut Stream_t;
    }
    if floppyd_open(This, mode) < 0 as libc::c_int {
        sprintf(
            errmsg,
            b"Can't open remote drive: %s\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
        close((*This).fd);
        free(This as *mut libc::c_char as *mut libc::c_void);
        return 0 as *mut Stream_t;
    }
    if !maxSize.is_null() {
        *maxSize = if (*This).capabilities & 2 as libc::c_int as libc::c_uint != 0 {
            max_off_t_seek
        } else {
            max_off_t_31
        };
    }
    return &mut (*This).head;
}
unsafe extern "C" fn ConnectToFloppyd(
    mut floppyd: *mut RemoteFile_t,
    mut name: *const libc::c_char,
    mut errmsg: *mut libc::c_char,
) -> libc::c_int {
    let mut hostname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut display: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut port: uint16_t = 0;
    let mut rval: libc::c_int = get_host_and_port_and_drive(
        name,
        &mut hostname,
        &mut display,
        &mut port,
        &mut (*floppyd).drive,
    );
    let mut sock: libc::c_int = 0;
    let mut reply: libc::c_uint = 0;
    if rval == 0 {
        return -(1 as libc::c_int);
    }
    (*floppyd).version = 11 as libc::c_int as libc::c_uint;
    (*floppyd).capabilities = 0 as libc::c_int as libc::c_uint;
    loop {
        sock = connect_to_server(getipaddress(hostname), port);
        if sock == -(1 as libc::c_int) {
            snprintf(
                errmsg,
                200 as libc::c_int as libc::c_ulong,
                b"Can't connect to floppyd server on %s, port %i (%s)!\0" as *const u8
                    as *const libc::c_char,
                hostname,
                port as libc::c_int,
                strerror(*__errno_location()),
            );
            return -(1 as libc::c_int);
        }
        reply = authenticate_to_floppyd(floppyd, sock, display);
        if (*floppyd).version == 10 as libc::c_int as libc::c_uint {
            break;
        }
        if !(reply == AUTH_WRONGVERSION as libc::c_int as libc::c_uint) {
            break;
        }
        (*floppyd).version = 10 as libc::c_int as libc::c_uint;
    }
    if reply != 0 as libc::c_int as libc::c_uint {
        fprintf(
            stderr,
            b"Permission denied, authentication failed!\n%s\n\0" as *const u8
                as *const libc::c_char,
            AuthErrors[reply as usize],
        );
        return -(1 as libc::c_int);
    }
    free(hostname as *mut libc::c_void);
    free(display as *mut libc::c_void);
    return sock;
}
