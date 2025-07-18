use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn exit(_: libc::c_int) -> !;
    fn system(__command: *const libc::c_char) -> libc::c_int;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn vsnprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn getaddrinfo(
        __name: *const libc::c_char,
        __service: *const libc::c_char,
        __req: *const addrinfo,
        __pai: *mut *mut addrinfo,
    ) -> libc::c_int;
    fn freeaddrinfo(__ai: *mut addrinfo);
    fn gai_strerror(__ecode: libc::c_int) -> *const libc::c_char;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    static mut debug: libc::c_int;
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
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
pub type va_list = __builtin_va_list;
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
pub unsafe extern "C" fn run_cmd(
    mut cmd: *mut libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut ap: ::std::ffi::VaListImpl;
    let mut buf: [libc::c_char; 100] = [0; 100];
    ap = args.clone();
    vsnprintf(
        buf.as_mut_ptr(),
        100 as libc::c_int as libc::c_ulong,
        cmd,
        ap.as_va_list(),
    );
    if debug != 0 {
        printf(b"EXEC: %s\n\0" as *const u8 as *const libc::c_char, buf.as_mut_ptr());
    }
    return system(buf.as_mut_ptr());
}
pub unsafe extern "C" fn sum_every_16bits(
    mut addr: *mut libc::c_void,
    mut count: libc::c_int,
) -> uint32_t {
    let mut sum: uint32_t = 0 as libc::c_int as uint32_t;
    let mut ptr: *mut uint16_t = addr as *mut uint16_t;
    while count > 1 as libc::c_int {
        let fresh0 = ptr;
        ptr = ptr.offset(1);
        sum = (sum as libc::c_uint).wrapping_add(*fresh0 as libc::c_uint) as uint32_t
            as uint32_t;
        count -= 2 as libc::c_int;
    }
    if count > 0 as libc::c_int {
        sum = (sum as libc::c_uint).wrapping_add(*(ptr as *mut uint8_t) as libc::c_uint)
            as uint32_t as uint32_t;
    }
    return sum;
}
pub unsafe extern "C" fn checksum(
    mut addr: *mut libc::c_void,
    mut count: libc::c_int,
    mut start_sum: libc::c_int,
) -> uint16_t {
    let mut sum: uint32_t = start_sum as uint32_t;
    sum = (sum as libc::c_uint).wrapping_add(sum_every_16bits(addr, count)) as uint32_t
        as uint32_t;
    while sum >> 16 as libc::c_int != 0 {
        sum = (sum & 0xffff as libc::c_int as libc::c_uint)
            .wrapping_add(sum >> 16 as libc::c_int);
    }
    return !sum as uint16_t;
}
pub unsafe extern "C" fn get_address(
    mut host: *mut libc::c_char,
    mut port: *mut libc::c_char,
    mut addr: *mut sockaddr,
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
    let mut s: libc::c_int = 0;
    memset(
        &mut hints as *mut addrinfo as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<addrinfo>() as libc::c_ulong,
    );
    hints.ai_family = 2 as libc::c_int;
    hints.ai_socktype = SOCK_STREAM as libc::c_int;
    s = getaddrinfo(host, port, &mut hints, &mut result);
    if s != 0 as libc::c_int {
        fprintf(
            stderr,
            b"getaddrinfo: %s\n\0" as *const u8 as *const libc::c_char,
            gai_strerror(s),
        );
        exit(1 as libc::c_int);
    }
    rp = result;
    if !rp.is_null() {
        *addr = *(*rp).ai_addr;
        freeaddrinfo(result);
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn parse_ipv4_string(mut addr: *mut libc::c_char) -> uint32_t {
    let mut addr_bytes: [uint8_t; 4] = [0; 4];
    sscanf(
        addr,
        b"%hhu.%hhu.%hhu.%hhu\0" as *const u8 as *const libc::c_char,
        &mut *addr_bytes.as_mut_ptr().offset(3 as libc::c_int as isize) as *mut uint8_t,
        &mut *addr_bytes.as_mut_ptr().offset(2 as libc::c_int as isize) as *mut uint8_t,
        &mut *addr_bytes.as_mut_ptr().offset(1 as libc::c_int as isize) as *mut uint8_t,
        &mut *addr_bytes.as_mut_ptr().offset(0 as libc::c_int as isize) as *mut uint8_t,
    );
    return (addr_bytes[0 as libc::c_int as usize] as libc::c_int
        | (addr_bytes[1 as libc::c_int as usize] as libc::c_int) << 8 as libc::c_int
        | (addr_bytes[2 as libc::c_int as usize] as libc::c_int) << 16 as libc::c_int
        | (addr_bytes[3 as libc::c_int as usize] as libc::c_int) << 24 as libc::c_int)
        as uint32_t;
}
pub unsafe extern "C" fn min(mut x: uint32_t, mut y: uint32_t) -> uint32_t {
    return if x > y { y } else { x };
}
