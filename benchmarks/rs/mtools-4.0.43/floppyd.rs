use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type _XDisplay;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn lseek(__fd: libc::c_int, __offset: __off_t, __whence: libc::c_int) -> __off_t;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn alarm(__seconds: libc::c_uint) -> libc::c_uint;
    fn setpgrp() -> libc::c_int;
    fn setsid() -> __pid_t;
    fn setuid(__uid: __uid_t) -> libc::c_int;
    fn setgid(__gid: __gid_t) -> libc::c_int;
    fn fork() -> __pid_t;
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    static mut optarg: *mut libc::c_char;
    static mut optind: libc::c_int;
    fn getopt(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
    ) -> libc::c_int;
    fn gethostname(__name: *mut libc::c_char, __len: size_t) -> libc::c_int;
    fn fsync(__fd: libc::c_int) -> libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn umask(__mask: __mode_t) -> __mode_t;
    fn __errno_location() -> *mut libc::c_int;
    fn endpwent();
    fn getpwuid(__uid: __uid_t) -> *mut passwd;
    fn getpwnam(__name: *const libc::c_char) -> *mut passwd;
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
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn free(__ptr: *mut libc::c_void);
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn setenv(
        __name: *const libc::c_char,
        __value: *const libc::c_char,
        __replace: libc::c_int,
    ) -> libc::c_int;
    fn mkstemp(__template: *mut libc::c_char) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    static mut stderr: *mut FILE;
    fn exit(_: libc::c_int) -> !;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;
    fn perror(__s: *const libc::c_char);
    fn mt_lseek(fd: libc::c_int, where_0: mt_off_t, whence: libc::c_int) -> libc::c_int;
    fn initgroups(__user: *const libc::c_char, __group: __gid_t) -> libc::c_int;
    fn lock_dev(fd: libc::c_int, mode: libc::c_int, dev: *mut device) -> libc::c_int;
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
    fn inet_addr(__cp: *const libc::c_char) -> in_addr_t;
    fn endhostent();
    fn gethostbyname(__name: *const libc::c_char) -> *mut hostent;
    fn endservent();
    fn getservbyname(
        __name: *const libc::c_char,
        __proto: *const libc::c_char,
    ) -> *mut servent;
    fn XauFileName() -> *mut libc::c_char;
    fn XOpenDisplay(_: *const libc::c_char) -> *mut Display;
    fn XCloseDisplay(_: *mut Display) -> libc::c_int;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __mode_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __ssize_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
pub type gid_t = __gid_t;
pub type uid_t = __uid_t;
pub type off_t = __off_t;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;
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
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
pub type socklen_t = __socklen_t;
pub type __sighandler_t = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct passwd {
    pub pw_name: *mut libc::c_char,
    pub pw_passwd: *mut libc::c_char,
    pub pw_uid: __uid_t,
    pub pw_gid: __gid_t,
    pub pw_gecos: *mut libc::c_char,
    pub pw_dir: *mut libc::c_char,
    pub pw_shell: *mut libc::c_char,
}
pub type mt_off_t = off_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct servent {
    pub s_name: *mut libc::c_char,
    pub s_aliases: *mut *mut libc::c_char,
    pub s_port: libc::c_int,
    pub s_proto: *mut libc::c_char,
}
pub type Byte = uint8_t;
pub type Dword = uint32_t;
pub type Qword = uint64_t;
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
pub type Display = _XDisplay;
pub type Packet = *mut Packet_0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Packet_0 {
    pub data: *mut Byte,
    pub len: Dword,
    pub alloc_size: Dword,
}
pub type io_buffer = *mut io_buffer_0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct io_buffer_0 {
    pub out_buffer: [Byte; 16348],
    pub in_buffer: [Byte; 16348],
    pub in_valid: size_t,
    pub in_start: size_t,
    pub out_valid: size_t,
    pub handle: libc::c_int,
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
unsafe extern "C" fn __bswap_16(mut __bsx: __uint16_t) -> __uint16_t {
    return (__bsx as libc::c_int >> 8 as libc::c_int & 0xff as libc::c_int
        | (__bsx as libc::c_int & 0xff as libc::c_int) << 8 as libc::c_int)
        as __uint16_t;
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
pub static mut mtools_lock_timeout: libc::c_uint = 30 as libc::c_int as libc::c_uint;
unsafe extern "C" fn new_io_buffer(mut _handle: libc::c_int) -> io_buffer {
    let mut buffer: io_buffer = 0 as *mut io_buffer_0;
    buffer = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<io_buffer_0>() as libc::c_ulong,
    ) as *mut io_buffer_0;
    (*buffer).handle = _handle;
    (*buffer).in_start = 0 as libc::c_int as size_t;
    (*buffer).in_valid = (*buffer).in_start;
    (*buffer).out_valid = 0 as libc::c_int as size_t;
    return buffer;
}
unsafe extern "C" fn flush(mut buffer: io_buffer) {
    if (*buffer).out_valid != 0 {
        if write(
            (*buffer).handle,
            ((*buffer).out_buffer).as_mut_ptr() as *const libc::c_void,
            (*buffer).out_valid,
        ) < 0 as libc::c_int as libc::c_long
        {
            perror(b"floppyd flush\0" as *const u8 as *const libc::c_char);
        }
        (*buffer).out_valid = 0 as libc::c_int as size_t;
    }
}
unsafe extern "C" fn free_io_buffer(mut buffer: io_buffer) {
    flush(buffer);
    free(buffer as *mut libc::c_void);
}
unsafe extern "C" fn buf_read(
    mut buf: io_buffer,
    mut buffer: *mut Byte,
    mut nbytes: size_t,
) -> size_t {
    let mut ret: size_t = 0;
    if nbytes <= (*buf).in_valid {
        memcpy(
            buffer as *mut libc::c_void,
            ((*buf).in_buffer).as_mut_ptr().offset((*buf).in_start as isize)
                as *const libc::c_void,
            nbytes,
        );
        (*buf)
            .in_valid = ((*buf).in_valid as libc::c_ulong).wrapping_sub(nbytes) as size_t
            as size_t;
        (*buf)
            .in_start = ((*buf).in_start as libc::c_ulong).wrapping_add(nbytes) as size_t
            as size_t;
        ret = nbytes;
    } else {
        if (*buf).in_valid != 0 {
            memcpy(
                buffer as *mut libc::c_void,
                ((*buf).in_buffer).as_mut_ptr().offset((*buf).in_start as isize)
                    as *const libc::c_void,
                (*buf).in_valid,
            );
        }
        nbytes = (nbytes as libc::c_ulong).wrapping_sub((*buf).in_valid) as size_t
            as size_t;
        buffer = buffer.offset((*buf).in_valid as isize);
        if nbytes > 16348 as libc::c_int as libc::c_ulong {
            let mut rval: ssize_t = read(
                (*buf).handle,
                buffer as *mut libc::c_void,
                nbytes,
            );
            if rval >= 0 as libc::c_int as libc::c_long {
                ret = (rval as size_t).wrapping_add((*buf).in_valid);
            } else {
                perror(b"read error\0" as *const u8 as *const libc::c_char);
                exit(1 as libc::c_int);
            }
            (*buf).in_start = 0 as libc::c_int as size_t;
            (*buf).in_valid = (*buf).in_start;
        } else {
            let mut rval_0: ssize_t = read(
                (*buf).handle,
                ((*buf).in_buffer).as_mut_ptr() as *mut libc::c_void,
                16348 as libc::c_int as size_t,
            );
            if rval_0 >= 0 as libc::c_int as libc::c_long {
                if rval_0 < nbytes as ssize_t {
                    memcpy(
                        buffer as *mut libc::c_void,
                        ((*buf).in_buffer).as_mut_ptr() as *const libc::c_void,
                        rval_0 as size_t,
                    );
                    ret = (rval_0 as size_t).wrapping_add((*buf).in_valid);
                    (*buf).in_start = 0 as libc::c_int as size_t;
                    (*buf).in_valid = (*buf).in_start;
                } else {
                    let mut a: size_t = 0;
                    memcpy(
                        buffer as *mut libc::c_void,
                        ((*buf).in_buffer).as_mut_ptr() as *const libc::c_void,
                        nbytes,
                    );
                    (*buf).in_start = nbytes;
                    a = (*buf).in_valid;
                    (*buf).in_valid = (rval_0 as size_t).wrapping_sub(nbytes);
                    ret = a.wrapping_add(nbytes);
                }
            } else {
                perror(b"read error\0" as *const u8 as *const libc::c_char);
                exit(1 as libc::c_int);
            }
        }
    }
    return ret;
}
unsafe extern "C" fn buf_write(
    mut buf: io_buffer,
    mut buffer: *mut libc::c_void,
    mut nbytes: size_t,
) -> ssize_t {
    if ((*buf).out_valid).wrapping_add(nbytes) > 16348 as libc::c_int as libc::c_ulong {
        flush(buf);
        return write((*buf).handle, buffer, nbytes);
    }
    memcpy(
        ((*buf).out_buffer).as_mut_ptr().offset((*buf).out_valid as isize)
            as *mut libc::c_void,
        buffer,
        nbytes,
    );
    (*buf)
        .out_valid = ((*buf).out_valid as libc::c_ulong).wrapping_add(nbytes) as size_t
        as size_t;
    return nbytes as ssize_t;
}
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
unsafe extern "C" fn read_dword(mut fp: io_buffer) -> Dword {
    let mut val: [Byte; 4] = [0; 4];
    if buf_read(fp, val.as_mut_ptr(), 4 as libc::c_int as size_t)
        < 4 as libc::c_int as libc::c_ulong
    {
        return 0xffffffff as libc::c_uint;
    }
    return byte2dword(val.as_mut_ptr());
}
unsafe extern "C" fn write_dword(mut fp: io_buffer, mut parm: Dword) {
    let mut val: [Byte; 4] = [0; 4];
    dword2byte(parm, val.as_mut_ptr());
    buf_write(fp, val.as_mut_ptr() as *mut libc::c_void, 4 as libc::c_int as size_t);
}
unsafe extern "C" fn newPacket() -> Packet {
    let mut packet: Packet = 0 as *mut Packet_0;
    packet = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<Packet_0>() as libc::c_ulong,
    ) as *mut Packet_0;
    (*packet).data = 0 as *mut Byte;
    (*packet).alloc_size = 0 as libc::c_int as Dword;
    (*packet).len = (*packet).alloc_size;
    return packet;
}
unsafe extern "C" fn destroyPacket(mut packet: Packet) {
    if !((*packet).data).is_null() {
        free((*packet).data as *mut libc::c_void);
    }
    free(packet as *mut libc::c_void);
}
unsafe extern "C" fn kill_packet(mut packet: Packet) {
    if !((*packet).data).is_null() {
        free((*packet).data as *mut libc::c_void);
    }
    (*packet).data = 0 as *mut Byte;
    (*packet).len = 0 as libc::c_int as Dword;
    (*packet).alloc_size = 0 as libc::c_int as Dword;
}
unsafe extern "C" fn make_new(mut packet: Packet, mut l: Dword) {
    if l < (*packet).alloc_size {
        (*packet).len = l;
        return;
    }
    kill_packet(packet);
    (*packet).alloc_size = l;
    (*packet).len = (*packet).alloc_size;
    (*packet).data = malloc(l as libc::c_ulong) as *mut Byte;
    memset((*packet).data as *mut libc::c_void, 0 as libc::c_int, l as libc::c_ulong);
}
unsafe extern "C" fn send_packet(mut packet: Packet, mut fp: io_buffer) -> libc::c_char {
    if !((*packet).data).is_null() {
        write_dword(fp, (*packet).len);
        buf_write(fp, (*packet).data as *mut libc::c_void, (*packet).len as size_t);
        flush(fp);
    }
    return ((*packet).data != 0 as *mut libc::c_void as *mut Byte) as libc::c_int
        as libc::c_char;
}
unsafe extern "C" fn recv_packet(
    mut packet: Packet,
    mut fp: io_buffer,
    mut maxlength: Dword,
) -> libc::c_char {
    let mut start: Dword = 0;
    let mut l: size_t = 0;
    let mut length: Dword = read_dword(fp);
    if length > maxlength || length == 0xffffffff as libc::c_uint {
        return 0 as libc::c_int as libc::c_char;
    }
    make_new(packet, length);
    l = 0 as libc::c_int as size_t;
    start = 0 as libc::c_int as Dword;
    while start < length {
        l = buf_read(
            fp,
            ((*packet).data).offset(start as isize),
            length.wrapping_sub(start) as size_t,
        );
        if l == 0 as libc::c_int as libc::c_ulong {
            return 0 as libc::c_int as libc::c_char;
        }
        start = (start as libc::c_ulong).wrapping_add(l) as Dword as Dword;
    }
    if (*packet).len == 0 as libc::c_int as libc::c_uint {
        return 0 as libc::c_int as libc::c_char;
    }
    return 1 as libc::c_int as libc::c_char;
}
unsafe extern "C" fn read_packet(
    mut packet: Packet,
    mut fd: libc::c_int,
    mut length: Dword,
) -> ssize_t {
    let mut ret: ssize_t = 0;
    make_new(packet, length);
    ret = read(fd, (*packet).data as *mut libc::c_void, (*packet).len as size_t);
    if ret < 0 as libc::c_int as libc::c_long {
        return ret;
    }
    (*packet).len = ret as Dword;
    return 0 as libc::c_int as ssize_t;
}
unsafe extern "C" fn write_packet(
    mut packet: Packet,
    mut fd: libc::c_int,
) -> libc::c_int {
    return write(fd, (*packet).data as *const libc::c_void, (*packet).len as size_t)
        as libc::c_int;
}
unsafe extern "C" fn put_dword(
    mut packet: Packet,
    mut my_index: libc::c_int,
    mut val: Dword,
) {
    dword2byte(val, ((*packet).data).offset(my_index as isize));
}
unsafe extern "C" fn put_qword(
    mut packet: Packet,
    mut my_index: libc::c_int,
    mut val: Qword,
) {
    qword2byte(val, ((*packet).data).offset(my_index as isize));
}
unsafe extern "C" fn get_dword(mut packet: Packet, mut my_index: libc::c_int) -> Dword {
    return byte2dword(((*packet).data).offset(my_index as isize));
}
unsafe extern "C" fn get_qword(mut packet: Packet, mut my_index: libc::c_int) -> Qword {
    return byte2qword(((*packet).data).offset(my_index as isize));
}
unsafe extern "C" fn get_length(mut packet: Packet) -> Dword {
    return (*packet).len;
}
unsafe extern "C" fn eat(
    mut ptr: *mut *mut libc::c_uchar,
    mut len: *mut size_t,
    mut c: libc::c_uchar,
) -> libc::c_int {
    if *len < (c as libc::c_uint).wrapping_add(3 as libc::c_uint) as libc::c_ulong {
        return -(1 as libc::c_int);
    }
    *ptr = (*ptr).offset((c as libc::c_int + 2 as libc::c_int) as isize);
    *len = (*len as libc::c_ulong)
        .wrapping_sub((c as libc::c_int + 2 as libc::c_int) as libc::c_ulong) as size_t
        as size_t;
    return 0 as libc::c_int;
}
static mut dispName: *const libc::c_char = 0 as *const libc::c_char;
static mut XAUTHORITY: [libc::c_char; 11] = unsafe {
    *::std::mem::transmute::<&[u8; 11], &mut [libc::c_char; 11]>(b"XAUTHORITY\0")
};
unsafe extern "C" fn do_auth(
    mut sock: io_buffer,
    mut version: *mut libc::c_uint,
) -> libc::c_char {
    let mut fd: libc::c_int = 0;
    let mut displ: *mut Display = 0 as *mut Display;
    let mut proto_version: Packet = newPacket();
    let mut mit_cookie: Packet = 0 as *mut Packet_0;
    let mut ptr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut len: size_t = 0;
    let mut authFile: [libc::c_char; 41] = *::std::mem::transmute::<
        &[u8; 41],
        &mut [libc::c_char; 41],
    >(b"/tmp/floppyd.XXXXXX\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0");
    let mut template: [libc::c_uchar; 4096] = [0; 4096];
    let mut reply: Packet = newPacket();
    make_new(reply, 4 as libc::c_int as Dword);
    if recv_packet(proto_version, sock, 4 as libc::c_int as Dword) == 0 {
        put_dword(reply, 0 as libc::c_int, AUTH_PACKETOVERSIZE as libc::c_int as Dword);
        send_packet(reply, sock);
        destroyPacket(reply);
        destroyPacket(proto_version);
        return 0 as libc::c_int as libc::c_char;
    }
    *version = get_dword(proto_version, 0 as libc::c_int);
    if *version > 11 as libc::c_int as libc::c_uint
        || *version < 10 as libc::c_int as libc::c_uint
    {
        put_dword(reply, 0 as libc::c_int, AUTH_WRONGVERSION as libc::c_int as Dword);
        send_packet(reply, sock);
        destroyPacket(reply);
        destroyPacket(proto_version);
        return 0 as libc::c_int as libc::c_char;
    }
    if *version == 10 as libc::c_int as libc::c_uint {
        put_dword(reply, 0 as libc::c_int, AUTH_SUCCESS as libc::c_int as Dword);
    } else {
        let mut cap: Dword = 1 as libc::c_int as Dword;
        if ::std::mem::size_of::<mt_off_t>() as libc::c_ulong
            >= 8 as libc::c_int as libc::c_ulong
        {
            cap |= 2 as libc::c_int as libc::c_uint;
        }
        make_new(reply, 12 as libc::c_int as Dword);
        put_dword(reply, 0 as libc::c_int, AUTH_SUCCESS as libc::c_int as Dword);
        put_dword(reply, 4 as libc::c_int, 11 as libc::c_int as Dword);
        put_dword(reply, 8 as libc::c_int, cap);
    }
    send_packet(reply, sock);
    destroyPacket(proto_version);
    make_new(reply, 4 as libc::c_int as Dword);
    mit_cookie = newPacket();
    if recv_packet(mit_cookie, sock, 3000 as libc::c_int as Dword) == 0 {
        put_dword(reply, 0 as libc::c_int, AUTH_PACKETOVERSIZE as libc::c_int as Dword);
        send_packet(reply, sock);
        destroyPacket(reply);
        destroyPacket(mit_cookie);
        return 0 as libc::c_int as libc::c_char;
    }
    umask(0o77 as libc::c_int as __mode_t);
    fd = mkstemp(authFile.as_mut_ptr());
    if fd == -(1 as libc::c_int) {
        put_dword(reply, 0 as libc::c_int, AUTH_DEVLOCKED as libc::c_int as Dword);
        send_packet(reply, sock);
        close(fd);
        destroyPacket(reply);
        destroyPacket(mit_cookie);
        return 0 as libc::c_int as libc::c_char;
    }
    setenv(XAUTHORITY.as_mut_ptr(), authFile.as_mut_ptr(), 1 as libc::c_int);
    ptr = template.as_mut_ptr();
    *ptr.offset(4095 as libc::c_int as isize) = 0 as libc::c_int as libc::c_uchar;
    let fresh0 = ptr;
    ptr = ptr.offset(1);
    *fresh0 = 1 as libc::c_int as libc::c_uchar;
    let fresh1 = ptr;
    ptr = ptr.offset(1);
    *fresh1 = 0 as libc::c_int as libc::c_uchar;
    let fresh2 = ptr;
    ptr = ptr.offset(1);
    *fresh2 = 0 as libc::c_int as libc::c_uchar;
    gethostname(
        (ptr as *mut libc::c_char).offset(1 as libc::c_int as isize),
        4088 as libc::c_int as size_t,
    );
    len = strlen((ptr as *mut libc::c_char).offset(1 as libc::c_int as isize));
    let fresh3 = ptr;
    ptr = ptr.offset(1);
    *fresh3 = len as libc::c_uchar;
    ptr = ptr.offset(len as isize);
    let fresh4 = ptr;
    ptr = ptr.offset(1);
    *fresh4 = 0 as libc::c_int as libc::c_uchar;
    let fresh5 = ptr;
    ptr = ptr.offset(1);
    *fresh5 = 1 as libc::c_int as libc::c_uchar;
    let fresh6 = ptr;
    ptr = ptr.offset(1);
    *fresh6 = '0' as i32 as libc::c_uchar;
    let fresh7 = ptr;
    ptr = ptr.offset(1);
    *fresh7 = '\0' as i32 as libc::c_uchar;
    if write(
        fd,
        template.as_mut_ptr() as *const libc::c_void,
        len.wrapping_add(8 as libc::c_int as libc::c_ulong),
    ) < len.wrapping_add(8 as libc::c_int as libc::c_ulong) as ssize_t
    {
        close(fd);
        return 0 as libc::c_int as libc::c_char;
    }
    ptr = (*mit_cookie).data;
    len = (*mit_cookie).len as size_t;
    if eat(&mut ptr, &mut len, 1 as libc::c_int as libc::c_uchar) != 0
        || eat(&mut ptr, &mut len, *ptr) != 0 || eat(&mut ptr, &mut len, *ptr) != 0
    {
        destroyPacket(mit_cookie);
        unlink(XauFileName());
        put_dword(reply, 0 as libc::c_int, AUTH_BADPACKET as libc::c_int as Dword);
        send_packet(reply, sock);
        destroyPacket(reply);
        return 0 as libc::c_int as libc::c_char;
    }
    if write(fd, ptr as *const libc::c_void, len) < len as ssize_t {
        close(fd);
        return 0 as libc::c_int as libc::c_char;
    }
    close(fd);
    destroyPacket(mit_cookie);
    displ = XOpenDisplay(dispName);
    if displ.is_null() {
        unlink(XauFileName());
        put_dword(reply, 0 as libc::c_int, AUTH_AUTHFAILED as libc::c_int as Dword);
        send_packet(reply, sock);
        destroyPacket(reply);
        return 0 as libc::c_int as libc::c_char;
    }
    XCloseDisplay(displ);
    put_dword(reply, 0 as libc::c_int, AUTH_SUCCESS as libc::c_int as Dword);
    send_packet(reply, sock);
    destroyPacket(reply);
    unlink(XauFileName());
    return 1 as libc::c_int as libc::c_char;
}
unsafe extern "C" fn getportnum(mut portnum: *mut libc::c_char) -> uint16_t {
    let mut digits: *mut libc::c_char = portnum;
    let mut serv: *mut servent = 0 as *mut servent;
    let mut port: uint16_t = 0;
    port = 0 as libc::c_int as uint16_t;
    while *(*__ctype_b_loc()).offset(*digits as libc::c_int as isize) as libc::c_int
        & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        port = (port as libc::c_int * 10 as libc::c_int
            + (*digits as libc::c_int - '0' as i32) as uint8_t as libc::c_int)
            as uint16_t;
        digits = digits.offset(1);
        digits;
    }
    if *digits as libc::c_int != '\0' as i32 || port as libc::c_int <= 0 as libc::c_int {
        serv = getservbyname(portnum, b"tcp\0" as *const u8 as *const libc::c_char);
        if !serv.is_null() {
            port = __bswap_16((*serv).s_port as uint16_t);
        } else {
            port = 0 as libc::c_int as uint16_t;
        }
        endservent();
    }
    return port;
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
unsafe extern "C" fn getuserid(mut user: *mut libc::c_char) -> uid_t {
    let mut pw: *mut passwd = 0 as *mut passwd;
    let mut uid: uid_t = 0;
    pw = getpwnam(user);
    if !pw.is_null() {
        uid = (*pw).pw_uid;
    } else if *user as libc::c_int == '#' as i32 {
        uid = atoi(&mut *user.offset(1 as libc::c_int as isize)) as uid_t;
    } else {
        uid = 65535 as libc::c_int as uid_t;
    }
    endpwent();
    return uid;
}
unsafe extern "C" fn getgroupid(mut uid: uid_t) -> uid_t {
    let mut pw: *mut passwd = 0 as *mut passwd;
    let mut gid: gid_t = 0;
    pw = getpwuid(uid);
    if !pw.is_null() {
        gid = (*pw).pw_gid;
    } else {
        gid = 65535 as libc::c_int as gid_t;
    }
    endpwent();
    return gid;
}
unsafe extern "C" fn bind_to_port(
    mut bind_ip: in_addr_t,
    mut bind_port: uint16_t,
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
        perror(b"socket()\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    let mut on: libc::c_int = 1 as libc::c_int;
    if setsockopt(
        sock,
        1 as libc::c_int,
        2 as libc::c_int,
        &mut on as *mut libc::c_int as *mut libc::c_char as *const libc::c_void,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t,
    ) < 0 as libc::c_int
    {
        perror(b"setsockopt\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    addr.sin_family = 2 as libc::c_int as sa_family_t;
    addr.sin_port = __bswap_16(bind_port);
    addr.sin_addr.s_addr = bind_ip;
    if bind(
        sock,
        &mut addr as *mut sockaddr_in as *mut sockaddr,
        ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as socklen_t,
    ) < 0 as libc::c_int
    {
        perror(b"bind()\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    if listen(sock, 4096 as libc::c_int) < 0 as libc::c_int {
        perror(b"listen()\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    return sock;
}
static mut sockethandle_now: libc::c_int = -(1 as libc::c_int);
unsafe extern "C" fn alarm_signal(mut a: libc::c_int) -> ! {
    if sockethandle_now != -(1 as libc::c_int) {
        close(sockethandle_now);
        sockethandle_now = -(1 as libc::c_int);
        unlink(XauFileName());
    }
    exit(1 as libc::c_int);
}
unsafe extern "C" fn server_main_loop(
    mut sock: libc::c_int,
    mut device_name: *const *const libc::c_char,
    mut n_dev: libc::c_uint,
) -> ! {
    let mut addr: sockaddr_in = sockaddr_in {
        sin_family: 0,
        sin_port: 0,
        sin_addr: in_addr { s_addr: 0 },
        sin_zero: [0; 8],
    };
    let mut len: libc::c_uint = 0;
    signal(
        17 as libc::c_int,
        ::std::mem::transmute::<
            libc::intptr_t,
            __sighandler_t,
        >(1 as libc::c_int as libc::intptr_t),
    );
    loop {
        let mut new_sock: libc::c_int = 0;
        len = ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as libc::c_uint;
        loop {
            new_sock = accept(
                sock,
                &mut addr as *mut sockaddr_in as *mut sockaddr,
                &mut len,
            );
            if !(new_sock < 0 as libc::c_int) {
                break;
            }
        }
        match fork() {
            0 => {
                serve_client(new_sock, device_name, n_dev, 0 as libc::c_int);
                exit(0 as libc::c_int);
            }
            -1 | _ => {}
        }
        close(new_sock);
        new_sock = -(1 as libc::c_int);
    };
}
unsafe extern "C" fn usage(
    mut prog: *mut libc::c_char,
    mut opt: *const libc::c_char,
    mut ret: libc::c_int,
) -> ! {
    if !opt.is_null() {
        fprintf(stderr, b"%s: %s\n\0" as *const u8 as *const libc::c_char, prog, opt);
    }
    fprintf(
        stderr,
        b"usage: %s [-s port [-r user] [-b ipaddr]] devicename [Names of local host]\n\0"
            as *const u8 as *const libc::c_char,
        prog,
    );
    fprintf(
        stderr,
        b"    -d          Run as a server (default port 5703 + DISPLAY)\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"    -s port     Run as a server bound to the specified port.\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"    -r user     Run as the specified user in server mode.\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"    -b ipaddr   Bind to the specified ipaddr in server mode.\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"    -l          Do not attempt to connect to localhost:0 to validate connection\n\0"
            as *const u8 as *const libc::c_char,
    );
    exit(ret);
}
unsafe extern "C" fn makeDisplayName(mut dispNr: libc::c_int) -> *mut libc::c_char {
    let mut result: [libc::c_char; 80] = [0; 80];
    sprintf(result.as_mut_ptr(), b":%d.0\0" as *const u8 as *const libc::c_char, dispNr);
    return strdup(result.as_mut_ptr());
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut sockfd: libc::c_int = 0 as libc::c_int;
    let mut arg: libc::c_int = 0;
    let mut run_as_server: libc::c_int = 0 as libc::c_int;
    let mut bind_ip: in_addr_t = 0 as libc::c_int as in_addr_t;
    let mut bind_port: uint16_t = 0 as libc::c_int as uint16_t;
    let mut run_uid: uid_t = 65535 as libc::c_int as uid_t;
    let mut run_gid: gid_t = 65535 as libc::c_int as gid_t;
    let mut username: *mut libc::c_char = strdup(
        b"nobody\0" as *const u8 as *const libc::c_char,
    );
    let mut sock: libc::c_int = 0;
    let mut device_name: *const *const libc::c_char = 0 as *const *const libc::c_char;
    let mut floppy0: *const libc::c_char = b"/dev/fd0\0" as *const u8
        as *const libc::c_char;
    let mut n_dev: libc::c_uint = 0;
    if argc > 1 as libc::c_int
        && strcmp(
            *argv.offset(0 as libc::c_int as isize),
            b"--help\0" as *const u8 as *const libc::c_char,
        ) == 0
    {
        usage(
            *argv.offset(0 as libc::c_int as isize),
            0 as *const libc::c_char,
            0 as libc::c_int,
        );
    }
    loop {
        arg = getopt(argc, argv, b"ds:r:b:x:h\0" as *const u8 as *const libc::c_char);
        if !(arg != -(1 as libc::c_int)) {
            break;
        }
        match arg {
            100 => {
                run_as_server = 1 as libc::c_int;
            }
            115 => {
                run_as_server = 1 as libc::c_int;
                bind_port = getportnum(optarg);
            }
            114 => {
                free(username as *mut libc::c_void);
                username = strdup(optarg);
                run_uid = getuserid(optarg);
                run_gid = getgroupid(run_uid);
            }
            98 => {
                run_as_server = 1 as libc::c_int;
                bind_ip = getipaddress(optarg);
            }
            120 => {
                dispName = strdup(optarg);
            }
            104 => {
                usage(
                    *argv.offset(0 as libc::c_int as isize),
                    0 as *const libc::c_char,
                    0 as libc::c_int,
                );
            }
            63 => {
                usage(
                    *argv.offset(0 as libc::c_int as isize),
                    0 as *const libc::c_char,
                    1 as libc::c_int,
                );
            }
            _ => {}
        }
    }
    if optind < argc {
        device_name = (argv as *const *const libc::c_char).offset(optind as isize);
        n_dev = (argc - optind) as libc::c_uint;
    } else {
        device_name = &mut floppy0;
        n_dev = 1 as libc::c_int as libc::c_uint;
    }
    if dispName.is_null() {
        dispName = getenv(b"DISPLAY\0" as *const u8 as *const libc::c_char);
    }
    if dispName.is_null() && bind_port as libc::c_int != 0 as libc::c_int {
        dispName = makeDisplayName(
            (bind_port as libc::c_int - 5703 as libc::c_int) as libc::c_ushort
                as libc::c_int,
        );
    }
    if dispName.is_null() {
        dispName = b":0\0" as *const u8 as *const libc::c_char;
    }
    if bind_port as libc::c_int == 0 as libc::c_int {
        let mut p: *mut libc::c_char = strchr(dispName, ':' as i32);
        bind_port = 5703 as libc::c_int as uint16_t;
        if !p.is_null() {
            bind_port = (bind_port as libc::c_int
                + atoi(p.offset(1 as libc::c_int as isize))) as uint16_t;
        }
    }
    if run_as_server == 0 {
        let mut addr: sockaddr_in = sockaddr_in {
            sin_family: 0,
            sin_port: 0,
            sin_addr: in_addr { s_addr: 0 },
            sin_zero: [0; 8],
        };
        let mut len: libc::c_uint = ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong
            as libc::c_uint;
        if getsockname(
            0 as libc::c_int,
            &mut addr as *mut sockaddr_in as *mut sockaddr,
            &mut len,
        ) >= 0 as libc::c_int
            && len as libc::c_ulong
                == ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong
        {
            bind_port = __bswap_16(addr.sin_port);
        }
    }
    umask(0o77 as libc::c_int as __mode_t);
    if run_as_server != 0 && bind_ip == 0xffffffff as libc::c_uint {
        usage(
            *argv.offset(0 as libc::c_int as isize),
            b"The server ipaddr is invalid.\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int,
        );
    }
    if run_as_server != 0 && bind_port as libc::c_int == 0 as libc::c_int {
        usage(
            *argv.offset(0 as libc::c_int as isize),
            b"No server port was specified (or it was invalid).\0" as *const u8
                as *const libc::c_char,
            1 as libc::c_int,
        );
    }
    if run_as_server != 0 {
        sock = bind_to_port(bind_ip, bind_port);
        match fork() {
            -1 => {
                perror(b"fork()\0" as *const u8 as *const libc::c_char);
                exit(1 as libc::c_int);
            }
            0 => {
                signal(
                    1 as libc::c_int,
                    ::std::mem::transmute::<
                        libc::intptr_t,
                        __sighandler_t,
                    >(1 as libc::c_int as libc::intptr_t),
                );
                signal(
                    3 as libc::c_int,
                    ::std::mem::transmute::<
                        libc::intptr_t,
                        __sighandler_t,
                    >(1 as libc::c_int as libc::intptr_t),
                );
                signal(
                    20 as libc::c_int,
                    ::std::mem::transmute::<
                        libc::intptr_t,
                        __sighandler_t,
                    >(1 as libc::c_int as libc::intptr_t),
                );
                signal(
                    18 as libc::c_int,
                    ::std::mem::transmute::<
                        libc::intptr_t,
                        __sighandler_t,
                    >(1 as libc::c_int as libc::intptr_t),
                );
                signal(
                    13 as libc::c_int,
                    ::std::mem::transmute::<
                        Option::<unsafe extern "C" fn(libc::c_int) -> !>,
                        __sighandler_t,
                    >(Some(alarm_signal as unsafe extern "C" fn(libc::c_int) -> !)),
                );
                setgid(run_gid);
                initgroups(username, run_gid);
                setuid(run_uid);
                setsid();
                setpgrp();
                server_main_loop(sock, device_name, n_dev);
            }
            _ => {}
        }
        exit(0 as libc::c_int);
    }
    signal(
        1 as libc::c_int,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(libc::c_int) -> !>,
            __sighandler_t,
        >(Some(alarm_signal as unsafe extern "C" fn(libc::c_int) -> !)),
    );
    signal(
        2 as libc::c_int,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(libc::c_int) -> !>,
            __sighandler_t,
        >(Some(alarm_signal as unsafe extern "C" fn(libc::c_int) -> !)),
    );
    signal(
        3 as libc::c_int,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(libc::c_int) -> !>,
            __sighandler_t,
        >(Some(alarm_signal as unsafe extern "C" fn(libc::c_int) -> !)),
    );
    signal(
        15 as libc::c_int,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(libc::c_int) -> !>,
            __sighandler_t,
        >(Some(alarm_signal as unsafe extern "C" fn(libc::c_int) -> !)),
    );
    signal(
        20 as libc::c_int,
        ::std::mem::transmute::<
            libc::intptr_t,
            __sighandler_t,
        >(1 as libc::c_int as libc::intptr_t),
    );
    signal(
        18 as libc::c_int,
        ::std::mem::transmute::<
            libc::intptr_t,
            __sighandler_t,
        >(1 as libc::c_int as libc::intptr_t),
    );
    signal(
        13 as libc::c_int,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(libc::c_int) -> !>,
            __sighandler_t,
        >(Some(alarm_signal as unsafe extern "C" fn(libc::c_int) -> !)),
    );
    serve_client(sockfd, device_name, n_dev, 1 as libc::c_int);
    return 0 as libc::c_int;
}
unsafe extern "C" fn send_reply(
    mut rval: libc::c_int,
    mut sock: io_buffer,
    mut len: Dword,
) {
    let mut reply: Packet = newPacket();
    make_new(reply, 8 as libc::c_int as Dword);
    put_dword(reply, 0 as libc::c_int, len);
    if rval == -(1 as libc::c_int) {
        put_dword(reply, 4 as libc::c_int, 0 as libc::c_int as Dword);
    } else {
        put_dword(reply, 4 as libc::c_int, *__errno_location() as Dword);
    }
    send_packet(reply, sock);
    destroyPacket(reply);
}
unsafe extern "C" fn send_reply64(
    mut rval: libc::c_int,
    mut sock: io_buffer,
    mut len: mt_off_t,
) {
    let mut reply: Packet = newPacket();
    make_new(reply, 12 as libc::c_int as Dword);
    put_qword(reply, 0 as libc::c_int, len as Qword);
    if rval == -(1 as libc::c_int) {
        put_dword(reply, 8 as libc::c_int, 0 as libc::c_int as Dword);
    } else {
        put_dword(reply, 8 as libc::c_int, *__errno_location() as Dword);
    }
    send_packet(reply, sock);
    destroyPacket(reply);
}
unsafe extern "C" fn cleanup(mut x: libc::c_int) -> ! {
    unlink(XauFileName());
    exit(-(1 as libc::c_int));
}
pub unsafe extern "C" fn serve_client(
    mut sockhandle: libc::c_int,
    mut device_name: *const *const libc::c_char,
    mut n_dev: libc::c_uint,
    mut close_stderr: libc::c_int,
) {
    let mut opcode: Packet = 0 as *mut Packet_0;
    let mut parm: Packet = 0 as *mut Packet_0;
    let mut readOnly: libc::c_int = 0;
    let mut devFd: libc::c_int = 0;
    let mut sock: io_buffer = 0 as *mut io_buffer_0;
    let mut stopLoop: libc::c_int = 0;
    let mut version: libc::c_uint = 0;
    let mut needSendReply: libc::c_int = 0 as libc::c_int;
    let mut rval: libc::c_int = 0 as libc::c_int;
    let mut on: libc::c_int = 1 as libc::c_int;
    if setsockopt(
        sockhandle,
        1 as libc::c_int,
        9 as libc::c_int,
        &mut on as *mut libc::c_int as *mut libc::c_char as *const libc::c_void,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t,
    ) < 0 as libc::c_int
    {
        perror(b"setsockopt\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    if close_stderr != 0 {
        close(2 as libc::c_int);
        open(b"/dev/null\0" as *const u8 as *const libc::c_char, 0o1 as libc::c_int);
    }
    sock = new_io_buffer(sockhandle);
    alarm(60 as libc::c_int as libc::c_uint);
    version = 0 as libc::c_int as libc::c_uint;
    if do_auth(sock, &mut version) == 0 {
        free_io_buffer(sock);
        return;
    }
    alarm(0 as libc::c_int as libc::c_uint);
    signal(
        15 as libc::c_int,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(libc::c_int) -> !>,
            __sighandler_t,
        >(Some(cleanup as unsafe extern "C" fn(libc::c_int) -> !)),
    );
    signal(
        14 as libc::c_int,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(libc::c_int) -> !>,
            __sighandler_t,
        >(Some(cleanup as unsafe extern "C" fn(libc::c_int) -> !)),
    );
    sockethandle_now = sockhandle;
    opcode = newPacket();
    parm = newPacket();
    devFd = -(1 as libc::c_int);
    readOnly = 1 as libc::c_int;
    stopLoop = 0 as libc::c_int;
    if version == 10 as libc::c_int as libc::c_uint {
        readOnly = 0 as libc::c_int;
        devFd = open(
            *device_name.offset(0 as libc::c_int as isize),
            0o2 as libc::c_int | 0 as libc::c_int,
        );
        if devFd < 0 as libc::c_int {
            readOnly = 1 as libc::c_int;
            devFd = open(
                *device_name.offset(0 as libc::c_int as isize),
                0 as libc::c_int | 0 as libc::c_int,
            );
        }
        if devFd < 0 as libc::c_int {
            send_reply(
                0 as libc::c_int,
                sock,
                if devFd >= 0 as libc::c_int {
                    0 as libc::c_int as libc::c_uint
                } else {
                    -(1 as libc::c_int) as Dword
                },
            );
            stopLoop = 1 as libc::c_int;
        }
        lock_dev(devFd, (readOnly == 0) as libc::c_int, 0 as *mut device);
    }
    while stopLoop == 0 {
        let mut dev_nr: uint32_t = 0 as libc::c_int as uint32_t;
        if recv_packet(opcode, sock, 1 as libc::c_int as Dword) == 0 {
            break;
        }
        recv_packet(parm, sock, 3000000 as libc::c_int as Dword);
        cork((*sock).handle, 1 as libc::c_int);
        match *((*opcode).data).offset(0 as libc::c_int as isize) as libc::c_int {
            6 => {
                if get_length(parm) >= 4 as libc::c_int as libc::c_uint {
                    dev_nr = get_dword(parm, 0 as libc::c_int);
                } else {
                    dev_nr = 0 as libc::c_int as uint32_t;
                }
                if dev_nr >= n_dev {
                    send_reply(0 as libc::c_int, sock, -(1 as libc::c_int) as Dword);
                } else {
                    devFd = open(
                        *device_name.offset(dev_nr as isize),
                        0 as libc::c_int | 0 as libc::c_int,
                    );
                    if devFd >= 0 as libc::c_int
                        && lock_dev(devFd, 0 as libc::c_int, 0 as *mut device) != 0
                    {
                        send_reply(0 as libc::c_int, sock, -(1 as libc::c_int) as Dword);
                    } else {
                        send_reply(
                            0 as libc::c_int,
                            sock,
                            if devFd >= 0 as libc::c_int {
                                0 as libc::c_int as libc::c_uint
                            } else {
                                -(1 as libc::c_int) as Dword
                            },
                        );
                        readOnly = 1 as libc::c_int;
                    }
                }
            }
            7 => {
                if get_length(parm) >= 4 as libc::c_int as libc::c_uint {
                    dev_nr = get_dword(parm, 0 as libc::c_int);
                } else {
                    dev_nr = 0 as libc::c_int as uint32_t;
                }
                if dev_nr >= n_dev {
                    send_reply(0 as libc::c_int, sock, -(1 as libc::c_int) as Dword);
                } else {
                    devFd = open(
                        *device_name.offset(dev_nr as isize),
                        0o2 as libc::c_int,
                    );
                    if devFd >= 0 as libc::c_int
                        && lock_dev(devFd, 1 as libc::c_int, 0 as *mut device) != 0
                    {
                        send_reply(0 as libc::c_int, sock, -(1 as libc::c_int) as Dword);
                    } else {
                        send_reply(
                            0 as libc::c_int,
                            sock,
                            if devFd >= 0 as libc::c_int {
                                0 as libc::c_int as libc::c_uint
                            } else {
                                -(1 as libc::c_int) as Dword
                            },
                        );
                        readOnly = 0 as libc::c_int;
                    }
                }
            }
            0 => {
                if read_packet(parm, devFd, get_dword(parm, 0 as libc::c_int))
                    < 0 as libc::c_int as libc::c_long
                {
                    send_reply(devFd, sock, -(1 as libc::c_int) as Dword);
                } else {
                    send_reply(devFd, sock, get_length(parm));
                    send_packet(parm, sock);
                }
            }
            1 => {
                if readOnly != 0 {
                    *__errno_location() = -(30 as libc::c_int);
                    rval = -(1 as libc::c_int);
                } else {
                    rval = write_packet(parm, devFd);
                }
                send_reply(devFd, sock, rval as Dword);
            }
            2 => {
                lseek(
                    devFd,
                    get_dword(parm, 0 as libc::c_int) as off_t,
                    get_dword(parm, 4 as libc::c_int) as libc::c_int,
                );
                send_reply(
                    devFd,
                    sock,
                    lseek(devFd, 0 as libc::c_int as __off_t, 1 as libc::c_int) as Dword,
                );
            }
            8 => {
                if (::std::mem::size_of::<mt_off_t>() as libc::c_ulong)
                    < 8 as libc::c_int as libc::c_ulong
                {
                    *__errno_location() = 22 as libc::c_int;
                    send_reply(devFd, sock, -(1 as libc::c_int) as Dword);
                } else {
                    mt_lseek(
                        devFd,
                        get_qword(parm, 0 as libc::c_int) as mt_off_t,
                        get_dword(parm, 8 as libc::c_int) as libc::c_int,
                    );
                    send_reply64(
                        devFd,
                        sock,
                        mt_lseek(devFd, 0 as libc::c_int as mt_off_t, 1 as libc::c_int)
                            as mt_off_t,
                    );
                }
            }
            3 => {
                fsync(devFd);
                send_reply(devFd, sock, 0 as libc::c_int as Dword);
            }
            4 => {
                close(devFd);
                needSendReply = 1 as libc::c_int;
                rval = devFd;
                devFd = -(1 as libc::c_int);
                stopLoop = 1 as libc::c_int;
            }
            5 => {}
            _ => {
                *__errno_location() = 22 as libc::c_int;
                send_reply(devFd, sock, -(1 as libc::c_int) as Dword);
            }
        }
        cork((*sock).handle, 0 as libc::c_int);
        kill_packet(parm);
        alarm(0 as libc::c_int as libc::c_uint);
    }
    if devFd >= 0 as libc::c_int {
        close(devFd);
        devFd = -(1 as libc::c_int);
    }
    free_io_buffer(sock);
    unlink(XauFileName());
    if needSendReply != 0 {
        send_reply(rval, sock, 0 as libc::c_int as Dword);
    }
    destroyPacket(opcode);
    destroyPacket(parm);
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
