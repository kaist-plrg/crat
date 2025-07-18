use ::libc;
extern "C" {
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn time(__timer: *mut time_t) -> time_t;
    fn strftime(
        __s: *mut libc::c_char,
        __maxsize: size_t,
        __format: *const libc::c_char,
        __tp: *const tm,
    ) -> size_t;
    fn gmtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm;
    fn safe_write(fd: libc::c_int, buf: *const libc::c_void, count: size_t) -> ssize_t;
    fn write_message(fd: libc::c_int, fmt: *const libc::c_char, _: ...) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __time_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: libc::c_int,
    pub tm_min: libc::c_int,
    pub tm_hour: libc::c_int,
    pub tm_mday: libc::c_int,
    pub tm_mon: libc::c_int,
    pub tm_year: libc::c_int,
    pub tm_wday: libc::c_int,
    pub tm_yday: libc::c_int,
    pub tm_isdst: libc::c_int,
    pub tm_gmtoff: libc::c_long,
    pub tm_zone: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct http_message_s {
    pub response: C2RustUnnamed_1,
    pub headers: C2RustUnnamed_0,
    pub body: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub text: *const libc::c_char,
    pub length: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub strings: *mut *const libc::c_char,
    pub total: libc::c_uint,
    pub used: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub string: *const libc::c_char,
    pub code: libc::c_int,
}
pub type http_message_t = *mut http_message_s;
unsafe extern "C" fn is_http_message_valid(mut msg: http_message_t) -> libc::c_int {
    if msg.is_null() {
        return 0 as libc::c_int;
    }
    if ((*msg).headers.strings).is_null() {
        return 0 as libc::c_int;
    }
    if ((*msg).response.string).is_null() {
        return 0 as libc::c_int;
    }
    if (*msg).response.code < 1 as libc::c_int
        || (*msg).response.code > 999 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn http_message_create(
    mut response_code: libc::c_int,
    mut response_string: *const libc::c_char,
) -> http_message_t {
    let mut msg: http_message_t = 0 as *mut http_message_s;
    let mut ret: libc::c_int = 0;
    msg = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<http_message_s>() as libc::c_ulong,
    ) as *mut http_message_s;
    if msg.is_null() {
        return 0 as http_message_t;
    }
    (*msg)
        .headers
        .strings = calloc(
        128 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
    ) as *mut *const libc::c_char;
    if ((*msg).headers.strings).is_null() {
        free(msg as *mut libc::c_void);
        msg = 0 as http_message_t;
        return 0 as http_message_t;
    }
    (*msg).headers.total = 128 as libc::c_int as libc::c_uint;
    ret = http_message_set_response(msg, response_code, response_string);
    if ret < 0 as libc::c_int {
        free((*msg).headers.strings as *mut libc::c_void);
        (*msg).headers.strings = 0 as *mut *const libc::c_char;
        free(msg as *mut libc::c_void);
        msg = 0 as http_message_t;
        return 0 as http_message_t;
    }
    return msg;
}
pub unsafe extern "C" fn http_message_destroy(mut msg: http_message_t) -> libc::c_int {
    if msg.is_null() {
        return -(14 as libc::c_int);
    }
    if !((*msg).headers.strings).is_null() {
        free((*msg).headers.strings as *mut libc::c_void);
        (*msg).headers.strings = 0 as *mut *const libc::c_char;
    }
    free(msg as *mut libc::c_void);
    msg = 0 as http_message_t;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn http_message_set_response(
    mut msg: http_message_t,
    mut response_code: libc::c_int,
    mut response_string: *const libc::c_char,
) -> libc::c_int {
    if msg.is_null() {
        return -(14 as libc::c_int);
    }
    if response_code < 1 as libc::c_int || response_code > 999 as libc::c_int {
        return -(22 as libc::c_int);
    }
    if response_string.is_null() {
        return -(22 as libc::c_int);
    }
    if strlen(response_string) == 0 as libc::c_int as libc::c_ulong {
        return -(22 as libc::c_int);
    }
    (*msg).response.code = response_code;
    (*msg).response.string = response_string;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn http_message_set_body(
    mut msg: http_message_t,
    mut body: *const libc::c_char,
    mut len: size_t,
) -> libc::c_int {
    if msg.is_null() {
        return -(14 as libc::c_int);
    }
    if body.is_null() {
        return -(22 as libc::c_int);
    }
    if len == 0 as libc::c_int as libc::c_ulong {
        return -(22 as libc::c_int);
    }
    (*msg).body.text = body;
    (*msg).body.length = len;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn http_message_add_headers(
    mut msg: http_message_t,
    mut headers: *mut *const libc::c_char,
    mut num_headers: libc::c_uint,
) -> libc::c_int {
    let mut new_headers: *mut *const libc::c_char = 0 as *mut *const libc::c_char;
    let mut i: libc::c_uint = 0;
    if msg.is_null() {
        return -(14 as libc::c_int);
    }
    if headers.is_null() {
        return -(22 as libc::c_int);
    }
    if ((*msg).headers.used).wrapping_add(num_headers) > (*msg).headers.total {
        new_headers = calloc(
            ((*msg).headers.total).wrapping_mul(2 as libc::c_int as libc::c_uint)
                as libc::c_ulong,
            ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
        ) as *mut *const libc::c_char;
        if new_headers.is_null() {
            return -(12 as libc::c_int);
        }
        i = 0 as libc::c_int as libc::c_uint;
        while i != (*msg).headers.used {
            let ref mut fresh0 = *new_headers.offset(i as isize);
            *fresh0 = *((*msg).headers.strings).offset(i as isize);
            i = i.wrapping_add(1);
            i;
        }
        free((*msg).headers.strings as *mut libc::c_void);
        (*msg).headers.strings = 0 as *mut *const libc::c_char;
        (*msg).headers.strings = new_headers;
        (*msg)
            .headers
            .total = ((*msg).headers.total)
            .wrapping_mul(2 as libc::c_int as libc::c_uint);
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i != num_headers {
        let ref mut fresh1 = *((*msg).headers.strings)
            .offset(i.wrapping_add((*msg).headers.used) as isize);
        *fresh1 = *headers.offset(i as isize);
        i = i.wrapping_add(1);
        i;
    }
    (*msg).headers.used = ((*msg).headers.used).wrapping_add(num_headers);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn http_message_send(
    mut msg: http_message_t,
    mut fd: libc::c_int,
) -> libc::c_int {
    let mut timebuf: [libc::c_char; 30] = [0; 30];
    let mut global_time: time_t = 0;
    let mut i: libc::c_uint = 0;
    let mut tm_buf: tm = tm {
        tm_sec: 0,
        tm_min: 0,
        tm_hour: 0,
        tm_mday: 0,
        tm_mon: 0,
        tm_year: 0,
        tm_wday: 0,
        tm_yday: 0,
        tm_isdst: 0,
        tm_gmtoff: 0,
        tm_zone: 0 as *const libc::c_char,
    };
    if msg.is_null() {
        return -(14 as libc::c_int);
    }
    if fd < 1 as libc::c_int {
        return -(9 as libc::c_int);
    }
    if is_http_message_valid(msg) == 0 {
        return -(22 as libc::c_int);
    }
    write_message(
        fd,
        b"HTTP/1.0 %d %s\r\n\0" as *const u8 as *const libc::c_char,
        (*msg).response.code,
        (*msg).response.string,
    );
    i = 0 as libc::c_int as libc::c_uint;
    while i != (*msg).headers.used {
        write_message(
            fd,
            b"%s\r\n\0" as *const u8 as *const libc::c_char,
            *((*msg).headers.strings).offset(i as isize),
        );
        i = i.wrapping_add(1);
        i;
    }
    global_time = time(0 as *mut time_t);
    strftime(
        timebuf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 30]>() as libc::c_ulong,
        b"%a, %d %b %Y %H:%M:%S GMT\0" as *const u8 as *const libc::c_char,
        gmtime_r(&mut global_time, &mut tm_buf),
    );
    write_message(
        fd,
        b"Date: %s\r\n\0" as *const u8 as *const libc::c_char,
        timebuf.as_mut_ptr(),
    );
    write_message(
        fd,
        b"Content-length: %lu\r\n\0" as *const u8 as *const libc::c_char,
        (*msg).body.length,
    );
    safe_write(
        fd,
        b"\r\n\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        2 as libc::c_int as size_t,
    );
    if (*msg).body.length > 0 as libc::c_int as libc::c_ulong {
        safe_write(fd, (*msg).body.text as *const libc::c_void, (*msg).body.length);
    }
    return 0 as libc::c_int;
}
