use ::libc;
extern "C" {
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn vsnprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memchr(
        _: *const libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn __errno_location() -> *mut libc::c_int;
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
    fn inet_pton(
        __af: libc::c_int,
        __cp: *const libc::c_char,
        __buf: *mut libc::c_void,
    ) -> libc::c_int;
    fn inet_ntop(
        __af: libc::c_int,
        __cp: *const libc::c_void,
        __buf: *mut libc::c_char,
        __len: socklen_t,
    ) -> *const libc::c_char;
    fn inet_aton(__cp: *const libc::c_char, __inp: *mut in_addr) -> libc::c_int;
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
pub type __ssize_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type va_list = __builtin_va_list;
pub type ssize_t = __ssize_t;
pub type socklen_t = __socklen_t;
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
pub const MSG_TRYHARD: C2RustUnnamed = 4;
pub const MSG_DONTROUTE: C2RustUnnamed = 4;
pub const MSG_PEEK: C2RustUnnamed = 2;
pub const MSG_OOB: C2RustUnnamed = 1;
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
    pub __in6_u: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
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
pub struct read_lines_s {
    pub data: *mut libc::c_char,
    pub len: size_t,
    pub next: *mut read_lines_s,
}
pub unsafe extern "C" fn safe_write(
    mut fd: libc::c_int,
    mut buf: *const libc::c_void,
    mut count: size_t,
) -> ssize_t {
    let mut len: ssize_t = 0;
    let mut bytestosend: size_t = 0;
    let mut buffer: *const libc::c_char = buf as *const libc::c_char;
    bytestosend = count;
    loop {
        len = send(
            fd,
            buffer as *const libc::c_void,
            bytestosend,
            MSG_NOSIGNAL as libc::c_int,
        );
        if len < 0 as libc::c_int as libc::c_long {
            if *__errno_location() == 4 as libc::c_int {
                continue;
            }
            return -*__errno_location() as ssize_t;
        } else {
            if len as size_t == bytestosend {
                break;
            }
            buffer = buffer.offset(len as isize);
            bytestosend = (bytestosend as libc::c_ulong)
                .wrapping_sub(len as libc::c_ulong) as size_t as size_t;
        }
    }
    return count as ssize_t;
}
pub unsafe extern "C" fn safe_read(
    mut fd: libc::c_int,
    mut buffer: *mut libc::c_void,
    mut count: size_t,
) -> ssize_t {
    let mut len: ssize_t = 0;
    loop {
        len = read(fd, buffer, count);
        if !(len < 0 as libc::c_int as libc::c_long
            && *__errno_location() == 4 as libc::c_int)
        {
            break;
        }
    }
    return len;
}
pub unsafe extern "C" fn write_message(
    mut fd: libc::c_int,
    mut fmt: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut n: ssize_t = 0;
    let mut size: size_t = (1024 as libc::c_int * 8 as libc::c_int) as size_t;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmpbuf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ap: ::std::ffi::VaListImpl;
    buf = malloc(size) as *mut libc::c_char;
    if buf.is_null() {
        return -(1 as libc::c_int);
    }
    loop {
        ap = args.clone();
        n = vsnprintf(buf, size, fmt, ap.as_va_list()) as ssize_t;
        if n > -(1 as libc::c_int) as libc::c_long && (n as size_t) < size {
            break;
        }
        if n > -(1 as libc::c_int) as libc::c_long {
            size = (n + 1 as libc::c_int as libc::c_long) as size_t;
        } else {
            size = (size as libc::c_ulong)
                .wrapping_mul(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
        }
        tmpbuf = realloc(buf as *mut libc::c_void, size) as *mut libc::c_char;
        if tmpbuf.is_null() {
            free(buf as *mut libc::c_void);
            buf = 0 as *mut libc::c_char;
            return -(1 as libc::c_int);
        } else {
            buf = tmpbuf;
        }
    }
    if safe_write(fd, buf as *const libc::c_void, n as size_t)
        < 0 as libc::c_int as libc::c_long
    {
        free(buf as *mut libc::c_void);
        buf = 0 as *mut libc::c_char;
        return -(1 as libc::c_int);
    }
    free(buf as *mut libc::c_void);
    buf = 0 as *mut libc::c_char;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn readline(
    mut fd: libc::c_int,
    mut whole_buffer: *mut *mut libc::c_char,
) -> ssize_t {
    let mut current_block: u64;
    let mut whole_buffer_len: ssize_t = 0;
    let mut buffer: [libc::c_char; 512] = [0; 512];
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ret: ssize_t = 0;
    let mut diff: ssize_t = 0;
    let mut first_line: *mut read_lines_s = 0 as *mut read_lines_s;
    let mut line_ptr: *mut read_lines_s = 0 as *mut read_lines_s;
    first_line = calloc(
        ::std::mem::size_of::<read_lines_s>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
    ) as *mut read_lines_s;
    if first_line.is_null() {
        return -(12 as libc::c_int) as ssize_t;
    }
    line_ptr = first_line;
    whole_buffer_len = 0 as libc::c_int as ssize_t;
    loop {
        ret = recv(
            fd,
            buffer.as_mut_ptr() as *mut libc::c_void,
            512 as libc::c_int as size_t,
            MSG_PEEK as libc::c_int,
        );
        if ret <= 0 as libc::c_int as libc::c_long {
            current_block = 8447755360592948704;
            break;
        }
        ptr = memchr(
            buffer.as_mut_ptr() as *const libc::c_void,
            '\n' as i32,
            ret as libc::c_ulong,
        ) as *mut libc::c_char;
        if !ptr.is_null() {
            diff = ptr.offset_from(buffer.as_mut_ptr()) as libc::c_long
                + 1 as libc::c_int as libc::c_long;
        } else {
            diff = ret;
        }
        whole_buffer_len += diff;
        if whole_buffer_len > (128 as libc::c_int * 1024 as libc::c_int) as libc::c_long
        {
            ret = -(34 as libc::c_int) as ssize_t;
            current_block = 8447755360592948704;
            break;
        } else {
            (*line_ptr).data = malloc(diff as libc::c_ulong) as *mut libc::c_char;
            if ((*line_ptr).data).is_null() {
                ret = -(12 as libc::c_int) as ssize_t;
                current_block = 8447755360592948704;
                break;
            } else {
                ret = recv(
                    fd,
                    (*line_ptr).data as *mut libc::c_void,
                    diff as size_t,
                    0 as libc::c_int,
                );
                if ret == -(1 as libc::c_int) as libc::c_long {
                    current_block = 8447755360592948704;
                    break;
                }
                (*line_ptr).len = diff as size_t;
                if !ptr.is_null() {
                    (*line_ptr).next = 0 as *mut read_lines_s;
                    current_block = 16203760046146113240;
                    break;
                } else {
                    (*line_ptr)
                        .next = calloc(
                        ::std::mem::size_of::<read_lines_s>() as libc::c_ulong,
                        1 as libc::c_int as libc::c_ulong,
                    ) as *mut read_lines_s;
                    if ((*line_ptr).next).is_null() {
                        ret = -(12 as libc::c_int) as ssize_t;
                        current_block = 8447755360592948704;
                        break;
                    } else {
                        line_ptr = (*line_ptr).next;
                    }
                }
            }
        }
    }
    match current_block {
        16203760046146113240 => {
            *whole_buffer = malloc(
                (whole_buffer_len + 1 as libc::c_int as libc::c_long) as libc::c_ulong,
            ) as *mut libc::c_char;
            if (*whole_buffer).is_null() {
                ret = -(12 as libc::c_int) as ssize_t;
            } else {
                *(*whole_buffer)
                    .offset(whole_buffer_len as isize) = '\0' as i32 as libc::c_char;
                whole_buffer_len = 0 as libc::c_int as ssize_t;
                line_ptr = first_line;
                while !line_ptr.is_null() {
                    memcpy(
                        (*whole_buffer).offset(whole_buffer_len as isize)
                            as *mut libc::c_void,
                        (*line_ptr).data as *const libc::c_void,
                        (*line_ptr).len,
                    );
                    whole_buffer_len = (whole_buffer_len as libc::c_ulong)
                        .wrapping_add((*line_ptr).len) as ssize_t as ssize_t;
                    line_ptr = (*line_ptr).next;
                }
                ret = whole_buffer_len;
            }
        }
        _ => {}
    }
    loop {
        line_ptr = (*first_line).next;
        if !((*first_line).data).is_null() {
            free((*first_line).data as *mut libc::c_void);
            (*first_line).data = 0 as *mut libc::c_char;
        }
        free(first_line as *mut libc::c_void);
        first_line = 0 as *mut read_lines_s;
        first_line = line_ptr;
        if first_line.is_null() {
            break;
        }
    }
    return ret;
}
pub unsafe extern "C" fn get_ip_string(
    mut sa: *mut sockaddr,
    mut buf: *mut libc::c_char,
    mut buflen: size_t,
) -> *const libc::c_char {
    let mut result: *const libc::c_char = 0 as *const libc::c_char;
    *buf.offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
    match (*sa).sa_family as libc::c_int {
        2 => {
            let mut sa_in: *mut sockaddr_in = sa as *mut sockaddr_in;
            result = inet_ntop(
                2 as libc::c_int,
                &mut (*sa_in).sin_addr as *mut in_addr as *const libc::c_void,
                buf,
                buflen as socklen_t,
            );
        }
        10 => {
            let mut sa_in6: *mut sockaddr_in6 = sa as *mut sockaddr_in6;
            result = inet_ntop(
                10 as libc::c_int,
                &mut (*sa_in6).sin6_addr as *mut in6_addr as *const libc::c_void,
                buf,
                buflen as socklen_t,
            );
        }
        _ => return 0 as *const libc::c_char,
    }
    return result;
}
pub unsafe extern "C" fn full_inet_pton(
    mut ip: *const libc::c_char,
    mut dst: *mut libc::c_void,
) -> libc::c_int {
    let mut buf: [libc::c_char; 24] = [0; 24];
    let mut tmp: [libc::c_char; 24] = [0; 24];
    let mut n: libc::c_int = 0;
    n = inet_aton(ip, dst as *mut in_addr);
    if n == 0 as libc::c_int {
        return inet_pton(10 as libc::c_int, ip, dst);
    }
    snprintf(
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 24]>() as libc::c_ulong,
        b"::ffff:%s\0" as *const u8 as *const libc::c_char,
        inet_ntop(
            2 as libc::c_int,
            dst,
            tmp.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 24]>() as libc::c_ulong as socklen_t,
        ),
    );
    return inet_pton(10 as libc::c_int, buf.as_mut_ptr(), dst);
}
