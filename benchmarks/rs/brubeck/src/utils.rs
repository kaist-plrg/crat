use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
    fn getaddrinfo(
        __name: *const libc::c_char,
        __service: *const libc::c_char,
        __req: *const addrinfo,
        __pai: *mut *mut addrinfo,
    ) -> libc::c_int;
    fn freeaddrinfo(__ai: *mut addrinfo);
    fn setsockopt(
        __fd: libc::c_int,
        __level: libc::c_int,
        __optname: libc::c_int,
        __optval: *const libc::c_void,
        __optlen: socklen_t,
    ) -> libc::c_int;
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
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    static mut stderr: *mut FILE;
    fn gh_log_die() -> !;
}
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type size_t = libc::c_ulong;
pub type socklen_t = __socklen_t;
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
pub union C2RustUnnamed {
    pub L: libc::c_int,
    pub F: libc::c_float,
}
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
pub unsafe extern "C" fn find_substr(
    mut s: *const libc::c_char,
    mut find: *const libc::c_char,
    mut slen: size_t,
) -> *mut libc::c_char {
    let mut c: libc::c_char = 0;
    let mut sc: libc::c_char = 0;
    let mut len: size_t = 0;
    let fresh0 = find;
    find = find.offset(1);
    c = *fresh0;
    if c as libc::c_int != '\0' as i32 {
        len = strlen(find);
        loop {
            loop {
                let fresh1 = s;
                s = s.offset(1);
                sc = *fresh1;
                if sc as libc::c_int == '\0' as i32
                    || {
                        let fresh2 = slen;
                        slen = slen.wrapping_sub(1);
                        fresh2 < 1 as libc::c_int as libc::c_ulong
                    }
                {
                    return 0 as *mut libc::c_char;
                }
                if !(sc as libc::c_int != c as libc::c_int) {
                    break;
                }
            }
            if len > slen {
                return 0 as *mut libc::c_char;
            }
            if !(strncmp(s, find, len) != 0 as libc::c_int) {
                break;
            }
        }
        s = s.offset(-1);
        s;
    }
    return s as *mut libc::c_char;
}
pub unsafe extern "C" fn sock_setnonblock(mut fd: libc::c_int) {
    let mut flags: libc::c_int = 0;
    flags = fcntl(fd, 3 as libc::c_int);
    flags |= 0o4000 as libc::c_int;
    if fcntl(fd, 4 as libc::c_int, flags) < 0 as libc::c_int {
        fprintf(
            stderr,
            b"[FATAL]: Failed to set O_NONBLOCK\n\0" as *const u8 as *const libc::c_char,
        );
        gh_log_die();
    }
}
pub unsafe extern "C" fn sock_setreuse(mut fd: libc::c_int, mut reuse: libc::c_int) {
    if setsockopt(
        fd,
        1 as libc::c_int,
        2 as libc::c_int,
        &mut reuse as *mut libc::c_int as *const libc::c_void,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t,
    ) == -(1 as libc::c_int)
    {
        fprintf(
            stderr,
            b"[FATAL]: Failed to set SO_REUSEADDR\n\0" as *const u8
                as *const libc::c_char,
        );
        gh_log_die();
    }
}
pub unsafe extern "C" fn sock_enlarge_in(mut fd: libc::c_int) {
    let mut bs: libc::c_int = 33554431 as libc::c_int;
    if setsockopt(
        fd,
        1 as libc::c_int,
        8 as libc::c_int,
        &mut bs as *mut libc::c_int as *const libc::c_void,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t,
    ) == -(1 as libc::c_int)
    {
        fprintf(
            stderr,
            b"[FATAL]: Failed to set SO_RCVBUF\n\0" as *const u8 as *const libc::c_char,
        );
        gh_log_die();
    }
}
pub unsafe extern "C" fn sock_enlarge_out(mut fd: libc::c_int) {
    let mut bs: libc::c_int = 33554431 as libc::c_int;
    if setsockopt(
        fd,
        1 as libc::c_int,
        7 as libc::c_int,
        &mut bs as *mut libc::c_int as *const libc::c_void,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t,
    ) == -(1 as libc::c_int)
    {
        fprintf(
            stderr,
            b"[FATAL]: Failed to set SO_SNDBUF\n\0" as *const u8 as *const libc::c_char,
        );
        gh_log_die();
    }
}
pub unsafe extern "C" fn sock_setreuse_port(
    mut fd: libc::c_int,
    mut reuse: libc::c_int,
) {
    if setsockopt(
        fd,
        1 as libc::c_int,
        15 as libc::c_int,
        &mut reuse as *mut libc::c_int as *const libc::c_void,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t,
    ) == -(1 as libc::c_int)
    {
        fprintf(
            stderr,
            b"[FATAL]: failed to set SO_REUSEPORT\n\0" as *const u8
                as *const libc::c_char,
        );
        gh_log_die();
    }
}
pub unsafe extern "C" fn url_to_inaddr2(
    mut addr: *mut sockaddr_in,
    mut url: *const libc::c_char,
    mut port: libc::c_int,
) {
    memset(
        addr as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong,
    );
    if !url.is_null() {
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
        memset(
            &mut hints as *mut addrinfo as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<addrinfo>() as libc::c_ulong,
        );
        hints.ai_family = 2 as libc::c_int;
        if getaddrinfo(url, 0 as *const libc::c_char, &mut hints, &mut result)
            != 0 as libc::c_int
        {
            fprintf(
                stderr,
                b"[FATAL]: failed to resolve address '%s'\n\0" as *const u8
                    as *const libc::c_char,
                url,
            );
            gh_log_die();
        }
        rp = result;
        while !rp.is_null() {
            if (*result).ai_family == 2 as libc::c_int
                && (*result).ai_addrlen as libc::c_ulong
                    == ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong
            {
                break;
            }
            rp = (*rp).ai_next;
        }
        if rp.is_null() {
            fprintf(
                stderr,
                b"[FATAL]: address format not supported\n\0" as *const u8
                    as *const libc::c_char,
            );
            gh_log_die();
        }
        memcpy(
            addr as *mut libc::c_void,
            (*rp).ai_addr as *const libc::c_void,
            (*rp).ai_addrlen as libc::c_ulong,
        );
        (*addr).sin_port = __bswap_16(port as __uint16_t);
        freeaddrinfo(result);
    } else {
        (*addr).sin_family = 2 as libc::c_int as sa_family_t;
        (*addr).sin_port = __bswap_16(port as __uint16_t);
        (*addr).sin_addr.s_addr = __bswap_32(0 as libc::c_int as in_addr_t);
    };
}
pub unsafe extern "C" fn brubeck_itoa(
    mut ptr: *mut libc::c_char,
    mut number: uint64_t,
) -> libc::c_int {
    let mut origin: *mut libc::c_char = ptr;
    let mut size: libc::c_int = 0;
    loop {
        let fresh3 = ptr;
        ptr = ptr.offset(1);
        *fresh3 = ('0' as i32 as libc::c_ulong)
            .wrapping_add(number.wrapping_rem(10 as libc::c_int as libc::c_ulong))
            as libc::c_char;
        number = (number as libc::c_ulong)
            .wrapping_div(10 as libc::c_int as libc::c_ulong) as uint64_t as uint64_t;
        if !(number != 0) {
            break;
        }
    }
    size = ptr.offset_from(origin) as libc::c_long as libc::c_int;
    ptr = ptr.offset(-1);
    ptr;
    while origin < ptr {
        let mut t: libc::c_char = *ptr;
        let fresh4 = ptr;
        ptr = ptr.offset(-1);
        *fresh4 = *origin;
        let fresh5 = origin;
        origin = origin.offset(1);
        *fresh5 = t;
    }
    return size;
}
pub unsafe extern "C" fn brubeck_ftoa(
    mut outbuf: *mut libc::c_char,
    mut f: libc::c_float,
) -> libc::c_int {
    let mut mantissa: uint64_t = 0;
    let mut int_part: uint64_t = 0;
    let mut frac_part: uint64_t = 0;
    let mut safe_shift: libc::c_int = 0;
    let mut safe_mask: uint64_t = 0;
    let mut exp2: libc::c_short = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut x: C2RustUnnamed = C2RustUnnamed { L: 0 };
    x.F = f;
    p = outbuf;
    exp2 = ((x.L >> 23 as libc::c_int) as libc::c_uchar as libc::c_int
        - 127 as libc::c_int) as libc::c_short;
    mantissa = (x.L & 0xffffff as libc::c_int | 0x800000 as libc::c_int) as uint64_t;
    frac_part = 0 as libc::c_int as uint64_t;
    int_part = 0 as libc::c_int as uint64_t;
    if x.L < 0 as libc::c_int {
        let fresh6 = p;
        p = p.offset(1);
        *fresh6 = '-' as i32 as libc::c_char;
    }
    if (exp2 as libc::c_int) < -(36 as libc::c_int) {
        let fresh7 = p;
        p = p.offset(1);
        *fresh7 = '0' as i32 as libc::c_char;
    } else {
        safe_shift = -(exp2 as libc::c_int + 1 as libc::c_int);
        safe_mask = (0xffffffffffffffff as libc::c_ulonglong
            >> 64 as libc::c_int - 24 as libc::c_int - safe_shift) as uint64_t;
        if exp2 as libc::c_int >= 64 as libc::c_int {
            int_part = (9223372036854775807 as libc::c_long as libc::c_ulong)
                .wrapping_mul(2 as libc::c_ulong)
                .wrapping_add(1 as libc::c_ulong);
        } else if exp2 as libc::c_int >= 23 as libc::c_int {
            int_part = mantissa << exp2 as libc::c_int - 23 as libc::c_int;
        } else if exp2 as libc::c_int >= 0 as libc::c_int {
            int_part = mantissa >> 23 as libc::c_int - exp2 as libc::c_int;
            frac_part = mantissa & safe_mask;
        } else {
            frac_part = mantissa & 0xffffff as libc::c_int as libc::c_ulong;
        }
        if int_part == 0 as libc::c_int as libc::c_ulong {
            let fresh8 = p;
            p = p.offset(1);
            *fresh8 = '0' as i32 as libc::c_char;
        } else {
            p = p.offset(brubeck_itoa(p, int_part) as isize);
        }
        if frac_part != 0 as libc::c_int as libc::c_ulong {
            let mut m: libc::c_int = 0;
            let fresh9 = p;
            p = p.offset(1);
            *fresh9 = '.' as i32 as libc::c_char;
            m = 0 as libc::c_int;
            while m < 4 as libc::c_int {
                frac_part = (frac_part << 3 as libc::c_int)
                    .wrapping_add(frac_part << 1 as libc::c_int);
                let fresh10 = p;
                p = p.offset(1);
                *fresh10 = (frac_part >> 24 as libc::c_int + safe_shift)
                    .wrapping_add('0' as i32 as libc::c_ulong) as libc::c_char;
                frac_part &= safe_mask;
                m += 1;
                m;
            }
            while *p.offset(-(1 as libc::c_int) as isize) as libc::c_int == '0' as i32 {
                p = p.offset(-1);
                p;
            }
            if *p.offset(-(1 as libc::c_int) as isize) as libc::c_int == '.' as i32 {
                p = p.offset(-1);
                p;
            }
        }
    }
    *p = 0 as libc::c_int as libc::c_char;
    return p.offset_from(outbuf) as libc::c_long as libc::c_int;
}
