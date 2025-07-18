use ::libc;
extern "C" {
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn __errno_location() -> *mut libc::c_int;
    fn send(
        __fd: libc::c_int,
        __buf: *const libc::c_void,
        __n: size_t,
        __flags: libc::c_int,
    ) -> ssize_t;
    fn log_message(level: libc::c_int, fmt: *const libc::c_char, _: ...);
}
pub type size_t = libc::c_ulong;
pub type __ssize_t = libc::c_long;
pub type ssize_t = __ssize_t;
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
pub struct buffer_s {
    pub head: *mut bufline_s,
    pub tail: *mut bufline_s,
    pub size: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bufline_s {
    pub string: *mut libc::c_uchar,
    pub next: *mut bufline_s,
    pub length: size_t,
    pub pos: size_t,
}
unsafe extern "C" fn makenewline(
    mut data: *mut libc::c_uchar,
    mut length: size_t,
) -> *mut bufline_s {
    let mut newline: *mut bufline_s = 0 as *mut bufline_s;
    newline = malloc(::std::mem::size_of::<bufline_s>() as libc::c_ulong)
        as *mut bufline_s;
    if newline.is_null() {
        return 0 as *mut bufline_s;
    }
    (*newline).string = malloc(length) as *mut libc::c_uchar;
    if ((*newline).string).is_null() {
        free(newline as *mut libc::c_void);
        newline = 0 as *mut bufline_s;
        return 0 as *mut bufline_s;
    }
    memcpy((*newline).string as *mut libc::c_void, data as *const libc::c_void, length);
    (*newline).next = 0 as *mut bufline_s;
    (*newline).length = length;
    (*newline).pos = 0 as libc::c_int as size_t;
    return newline;
}
unsafe extern "C" fn free_line(mut line: *mut bufline_s) {
    if line.is_null() {
        return;
    }
    if !((*line).string).is_null() {
        free((*line).string as *mut libc::c_void);
        (*line).string = 0 as *mut libc::c_uchar;
    }
    free(line as *mut libc::c_void);
    line = 0 as *mut bufline_s;
}
pub unsafe extern "C" fn new_buffer() -> *mut buffer_s {
    let mut buffptr: *mut buffer_s = 0 as *mut buffer_s;
    buffptr = malloc(::std::mem::size_of::<buffer_s>() as libc::c_ulong)
        as *mut buffer_s;
    if buffptr.is_null() {
        return 0 as *mut buffer_s;
    }
    (*buffptr).tail = 0 as *mut bufline_s;
    (*buffptr).head = (*buffptr).tail;
    (*buffptr).size = 0 as libc::c_int as size_t;
    return buffptr;
}
pub unsafe extern "C" fn delete_buffer(mut buffptr: *mut buffer_s) {
    let mut next: *mut bufline_s = 0 as *mut bufline_s;
    while !((*buffptr).head).is_null() {
        next = (*(*buffptr).head).next;
        free_line((*buffptr).head);
        (*buffptr).head = next;
    }
    free(buffptr as *mut libc::c_void);
    buffptr = 0 as *mut buffer_s;
}
pub unsafe extern "C" fn buffer_size(mut buffptr: *mut buffer_s) -> size_t {
    return (*buffptr).size;
}
pub unsafe extern "C" fn add_to_buffer(
    mut buffptr: *mut buffer_s,
    mut data: *mut libc::c_uchar,
    mut length: size_t,
) -> libc::c_int {
    let mut newline: *mut bufline_s = 0 as *mut bufline_s;
    ((*buffptr).head).is_null();
    newline = makenewline(data, length);
    if newline.is_null() {
        return -(1 as libc::c_int);
    }
    if (*buffptr).size == 0 as libc::c_int as libc::c_ulong {
        (*buffptr).tail = newline;
        (*buffptr).head = (*buffptr).tail;
    } else {
        (*(*buffptr).tail).next = newline;
        (*buffptr).tail = newline;
    }
    (*buffptr)
        .size = ((*buffptr).size as libc::c_ulong).wrapping_add(length) as size_t
        as size_t;
    return 0 as libc::c_int;
}
unsafe extern "C" fn remove_from_buffer(mut buffptr: *mut buffer_s) -> *mut bufline_s {
    let mut line: *mut bufline_s = 0 as *mut bufline_s;
    line = (*buffptr).head;
    (*buffptr).head = (*line).next;
    (*buffptr)
        .size = ((*buffptr).size as libc::c_ulong).wrapping_sub((*line).length) as size_t
        as size_t;
    return line;
}
pub unsafe extern "C" fn read_buffer(
    mut fd: libc::c_int,
    mut buffptr: *mut buffer_s,
) -> ssize_t {
    let mut bytesin: ssize_t = 0;
    let mut buffer: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    if (*buffptr).size >= (1024 as libc::c_int * 96 as libc::c_int) as size_t {
        return 0 as libc::c_int as ssize_t;
    }
    buffer = malloc((1024 as libc::c_int * 2 as libc::c_int) as libc::c_ulong)
        as *mut libc::c_uchar;
    if buffer.is_null() {
        return -(12 as libc::c_int) as ssize_t;
    }
    bytesin = read(
        fd,
        buffer as *mut libc::c_void,
        (1024 as libc::c_int * 2 as libc::c_int) as size_t,
    );
    if bytesin > 0 as libc::c_int as libc::c_long {
        if add_to_buffer(buffptr, buffer, bytesin as size_t) < 0 as libc::c_int {
            log_message(
                3 as libc::c_int,
                b"readbuff: add_to_buffer() error.\0" as *const u8 as *const libc::c_char,
            );
            bytesin = -(1 as libc::c_int) as ssize_t;
        }
    } else if bytesin == 0 as libc::c_int as libc::c_long {
        bytesin = -(1 as libc::c_int) as ssize_t;
    } else {
        match *__errno_location() {
            11 | 4 => {
                bytesin = 0 as libc::c_int as ssize_t;
            }
            _ => {
                log_message(
                    3 as libc::c_int,
                    b"read_buffer: read() failed on fd %d: %s\0" as *const u8
                        as *const libc::c_char,
                    fd,
                    strerror(*__errno_location()),
                );
                bytesin = -(1 as libc::c_int) as ssize_t;
            }
        }
    }
    free(buffer as *mut libc::c_void);
    buffer = 0 as *mut libc::c_uchar;
    return bytesin;
}
pub unsafe extern "C" fn write_buffer(
    mut fd: libc::c_int,
    mut buffptr: *mut buffer_s,
) -> ssize_t {
    let mut bytessent: ssize_t = 0;
    let mut line: *mut bufline_s = 0 as *mut bufline_s;
    if (*buffptr).size == 0 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int as ssize_t;
    }
    line = (*buffptr).head;
    bytessent = send(
        fd,
        ((*line).string).offset((*line).pos as isize) as *const libc::c_void,
        ((*line).length).wrapping_sub((*line).pos),
        MSG_NOSIGNAL as libc::c_int,
    );
    if bytessent >= 0 as libc::c_int as libc::c_long {
        (*line)
            .pos = ((*line).pos as libc::c_ulong)
            .wrapping_add(bytessent as libc::c_ulong) as size_t as size_t;
        if (*line).pos == (*line).length {
            free_line(remove_from_buffer(buffptr));
        }
        return bytessent;
    } else {
        match *__errno_location() {
            11 | 4 => return 0 as libc::c_int as ssize_t,
            105 | 12 => {
                log_message(
                    3 as libc::c_int,
                    b"writebuff: write() error [NOBUFS/NOMEM] \"%s\" on file descriptor %d\0"
                        as *const u8 as *const libc::c_char,
                    strerror(*__errno_location()),
                    fd,
                );
                return 0 as libc::c_int as ssize_t;
            }
            _ => {
                log_message(
                    3 as libc::c_int,
                    b"writebuff: write() error \"%s\" on file descriptor %d\0"
                        as *const u8 as *const libc::c_char,
                    strerror(*__errno_location()),
                    fd,
                );
                return -(1 as libc::c_int) as ssize_t;
            }
        }
    };
}
