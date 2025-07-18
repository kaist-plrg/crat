use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn http_encode(s: *mut libc::c_char, len: size_t);
    fn http_size_from_range(conn: *mut http_t) -> off_t;
    fn http_size(conn: *mut http_t) -> off_t;
    fn http_filename(conn: *const http_t, filename: *mut libc::c_char);
    fn http_header(
        conn: *const http_t,
        header: *const libc::c_char,
    ) -> *const libc::c_char;
    fn http_exec(conn: *mut http_t) -> libc::c_int;
    fn http_addheader(conn: *mut http_t, format: *const libc::c_char, _: ...);
    fn http_get(conn: *mut http_t, lurl: *mut libc::c_char);
    fn http_disconnect(conn: *mut http_t);
    fn http_connect(
        conn: *mut http_t,
        proto: libc::c_int,
        proxy: *mut libc::c_char,
        host: *mut libc::c_char,
        port: libc::c_int,
        user: *mut libc::c_char,
        pass: *mut libc::c_char,
        io_timeout: libc::c_uint,
    ) -> libc::c_int;
    fn ftp_size(
        conn: *mut ftp_t,
        file: *mut libc::c_char,
        maxredir: libc::c_int,
        io_timeout: libc::c_uint,
    ) -> off_t;
    fn ftp_data(conn: *mut ftp_t, io_timeout: libc::c_uint) -> libc::c_int;
    fn ftp_cwd(conn: *mut ftp_t, cwd: *mut libc::c_char) -> libc::c_int;
    fn ftp_command(conn: *mut ftp_t, format: *const libc::c_char, _: ...) -> libc::c_int;
    fn ftp_wait(conn: *mut ftp_t) -> libc::c_int;
    fn ftp_disconnect(conn: *mut ftp_t);
    fn ftp_connect(
        conn: *mut ftp_t,
        proto: libc::c_int,
        host: *mut libc::c_char,
        port: libc::c_int,
        user: *mut libc::c_char,
        pass: *mut libc::c_char,
        io_timeout: libc::c_uint,
    ) -> libc::c_int;
    fn is_ipv6_addr(hostname: *const libc::c_char) -> libc::c_int;
    fn abuf_printf(abuf: *mut abuf_t, fmt: *const libc::c_char, _: ...) -> libc::c_int;
    fn abuf_setup(abuf: *mut abuf_t, len: size_t) -> libc::c_int;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn strlcpy(_: *mut libc::c_char, _: *const libc::c_char, _: size_t) -> size_t;
    fn strlcat(_: *mut libc::c_char, _: *const libc::c_char, _: size_t) -> size_t;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strcspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __uint16_t = libc::c_ushort;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type off_t = __off_t;
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
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
pub type __pthread_list_t = __pthread_internal_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_mutex_s {
    pub __lock: libc::c_int,
    pub __count: libc::c_uint,
    pub __owner: libc::c_int,
    pub __nusers: libc::c_uint,
    pub __kind: libc::c_int,
    pub __spins: libc::c_short,
    pub __elision: libc::c_short,
    pub __list: __pthread_list_t,
}
pub type pthread_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
    pub __align: libc::c_long,
}
pub type uint16_t = __uint16_t;
pub type sa_family_t = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct message_t {
    pub next: *mut libc::c_void,
    pub text: [libc::c_char; 1024],
}
pub type if_t = message_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct abuf_t {
    pub p: *mut libc::c_char,
    pub len: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct conf_t {
    pub default_filename: [libc::c_char; 1024],
    pub http_proxy: [libc::c_char; 1024],
    pub no_proxy: [libc::c_char; 1024],
    pub num_connections: uint16_t,
    pub strip_cgi_parameters: libc::c_int,
    pub save_state_interval: libc::c_int,
    pub connection_timeout: libc::c_int,
    pub reconnect_delay: libc::c_int,
    pub max_redirect: libc::c_int,
    pub buffer_size: libc::c_int,
    pub max_speed: libc::c_ulonglong,
    pub verbose: libc::c_int,
    pub alternate_output: libc::c_int,
    pub insecure: libc::c_int,
    pub no_clobber: libc::c_int,
    pub percentage: libc::c_int,
    pub interfaces: *mut if_t,
    pub ai_family: sa_family_t,
    pub search_timeout: libc::c_int,
    pub search_threads: libc::c_int,
    pub search_amount: libc::c_int,
    pub search_top: libc::c_int,
    pub io_timeout: libc::c_uint,
    pub add_header_count: libc::c_int,
    pub add_header: [[libc::c_char; 1024]; 10],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tcp_t {
    pub fd: libc::c_int,
    pub ai_family: sa_family_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ftp_t {
    pub cwd: [libc::c_char; 1024],
    pub message: *mut libc::c_char,
    pub status: libc::c_int,
    pub tcp: tcp_t,
    pub data_tcp: tcp_t,
    pub proto: libc::c_int,
    pub ftp_mode: libc::c_int,
    pub local_if: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct http_t {
    pub host: [libc::c_char; 1024],
    pub auth: [libc::c_char; 1024],
    pub request: [abuf_t; 1],
    pub headers: [abuf_t; 1],
    pub port: libc::c_int,
    pub proto: libc::c_int,
    pub proxy: libc::c_int,
    pub proxy_auth: [libc::c_char; 1024],
    pub firstbyte: off_t,
    pub lastbyte: off_t,
    pub status: libc::c_int,
    pub tcp: tcp_t,
    pub local_if: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct conn_t {
    pub conf: *mut conf_t,
    pub proto: libc::c_int,
    pub port: libc::c_int,
    pub proxy: libc::c_int,
    pub host: [libc::c_char; 1024],
    pub dir: [libc::c_char; 1024],
    pub file: [libc::c_char; 1024],
    pub user: [libc::c_char; 1024],
    pub pass: [libc::c_char; 1024],
    pub output_filename: [libc::c_char; 1024],
    pub ftp: [ftp_t; 1],
    pub http: [http_t; 1],
    pub size: off_t,
    pub currentbyte: off_t,
    pub lastbyte: off_t,
    pub tcp: *mut tcp_t,
    pub enabled: bool,
    pub supported: bool,
    pub last_transfer: libc::c_int,
    pub message: *mut libc::c_char,
    pub local_if: *mut libc::c_char,
    pub state: bool,
    pub setup_thread: [pthread_t; 1],
    pub lock: pthread_mutex_t,
}
#[inline]
unsafe extern "C" fn is_proto_http(mut proto: libc::c_int) -> libc::c_int {
    return (proto & (1 as libc::c_int) << 1 as libc::c_int
        == (1 as libc::c_int) << 1 as libc::c_int) as libc::c_int;
}
pub unsafe extern "C" fn conn_set(
    mut conn: *mut conn_t,
    mut set_url: *const libc::c_char,
) -> libc::c_int {
    let mut url: [libc::c_char; 1024] = [0; 1024];
    let mut i: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut j: *mut libc::c_char = 0 as *mut libc::c_char;
    i = strstr(set_url, b"://\0" as *const u8 as *const libc::c_char);
    if i.is_null() {
        (*conn)
            .proto = (1 as libc::c_int) << 1 as libc::c_int
            | (0 as libc::c_int) << 0 as libc::c_int;
        (*conn).port = 80 as libc::c_int;
        strlcpy(
            url.as_mut_ptr(),
            set_url,
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
        );
    } else {
        let mut proto_len: libc::c_int = i.offset_from(set_url) as libc::c_long
            as libc::c_int;
        if strncmp(
            set_url,
            b"ftp\0" as *const u8 as *const libc::c_char,
            proto_len as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            (*conn)
                .proto = (0 as libc::c_int) << 1 as libc::c_int
                | (0 as libc::c_int) << 0 as libc::c_int;
            (*conn).port = 21 as libc::c_int;
        } else if strncmp(
            set_url,
            b"http\0" as *const u8 as *const libc::c_char,
            proto_len as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            (*conn)
                .proto = (1 as libc::c_int) << 1 as libc::c_int
                | (0 as libc::c_int) << 0 as libc::c_int;
            (*conn).port = 80 as libc::c_int;
        } else if strncmp(
            set_url,
            b"ftps\0" as *const u8 as *const libc::c_char,
            proto_len as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            (*conn)
                .proto = (0 as libc::c_int) << 1 as libc::c_int
                | (1 as libc::c_int) << 0 as libc::c_int;
            (*conn).port = 990 as libc::c_int;
        } else if strncmp(
            set_url,
            b"https\0" as *const u8 as *const libc::c_char,
            proto_len as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            (*conn)
                .proto = (1 as libc::c_int) << 1 as libc::c_int
                | (1 as libc::c_int) << 0 as libc::c_int;
            (*conn).port = 443 as libc::c_int;
        } else {
            fprintf(
                stderr,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Unsupported protocol\n\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            return 0 as libc::c_int;
        }
        if (*conn).proto & (1 as libc::c_int) << 0 as libc::c_int
            == (1 as libc::c_int) << 0 as libc::c_int
        {
            fprintf(
                stderr,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Secure protocol is not supported\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            return 0 as libc::c_int;
        }
        strlcpy(
            url.as_mut_ptr(),
            i.offset(3 as libc::c_int as isize),
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
        );
    }
    i = strchr(url.as_mut_ptr(), '/' as i32);
    if i.is_null() {
        strcpy(((*conn).dir).as_mut_ptr(), b"/\0" as *const u8 as *const libc::c_char);
    } else {
        *i = 0 as libc::c_int as libc::c_char;
        snprintf(
            ((*conn).dir).as_mut_ptr(),
            1024 as libc::c_int as size_t,
            b"/%s\0" as *const u8 as *const libc::c_char,
            i.offset(1 as libc::c_int as isize),
        );
        if (*conn).proto
            == (1 as libc::c_int) << 1 as libc::c_int
                | (0 as libc::c_int) << 0 as libc::c_int
            || (*conn).proto
                == (1 as libc::c_int) << 1 as libc::c_int
                    | (1 as libc::c_int) << 0 as libc::c_int
        {
            http_encode(
                ((*conn).dir).as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
            );
        }
    }
    j = strchr(((*conn).dir).as_mut_ptr(), '?' as i32);
    if !j.is_null() {
        *j = 0 as libc::c_int as libc::c_char;
    }
    i = strrchr(((*conn).dir).as_mut_ptr(), '/' as i32);
    if !i.is_null() {
        *i = 0 as libc::c_int as libc::c_char;
    }
    if !j.is_null() {
        *j = '?' as i32 as libc::c_char;
    }
    if i.is_null() {
        strlcpy(
            ((*conn).file).as_mut_ptr(),
            ((*conn).dir).as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
        );
        strcpy(((*conn).dir).as_mut_ptr(), b"/\0" as *const u8 as *const libc::c_char);
    } else {
        strlcpy(
            ((*conn).file).as_mut_ptr(),
            i.offset(1 as libc::c_int as isize),
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
        );
        strlcat(
            ((*conn).dir).as_mut_ptr(),
            b"/\0" as *const u8 as *const libc::c_char,
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
        );
    }
    if !(strrchr(url.as_mut_ptr(), '@' as i32)).is_null() {
        strlcpy(
            ((*conn).user).as_mut_ptr(),
            url.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
        );
        i = strrchr(((*conn).user).as_mut_ptr(), '@' as i32);
        *i = 0 as libc::c_int as libc::c_char;
        strlcpy(
            url.as_mut_ptr(),
            i.offset(1 as libc::c_int as isize),
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
        );
        *((*conn).pass).as_mut_ptr() = 0 as libc::c_int as libc::c_char;
    } else if (*conn).proto & (1 as libc::c_int) << 1 as libc::c_int
        == (0 as libc::c_int) << 1 as libc::c_int
    {
        strcpy(
            ((*conn).user).as_mut_ptr(),
            b"anonymous\0" as *const u8 as *const libc::c_char,
        );
        strcpy(
            ((*conn).pass).as_mut_ptr(),
            b"mailto:axel@axel.project\0" as *const u8 as *const libc::c_char,
        );
    } else {
        let ref mut fresh0 = *((*conn).pass).as_mut_ptr();
        *fresh0 = 0 as libc::c_int as libc::c_char;
        *((*conn).user).as_mut_ptr() = *fresh0;
    }
    i = strchr(((*conn).user).as_mut_ptr(), ':' as i32);
    if !i.is_null() {
        *i = 0 as libc::c_int as libc::c_char;
        strlcpy(
            ((*conn).pass).as_mut_ptr(),
            i.offset(1 as libc::c_int as isize),
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
        );
    }
    if *url.as_mut_ptr() as libc::c_int == '[' as i32 {
        strlcpy(
            ((*conn).host).as_mut_ptr(),
            url.as_mut_ptr().offset(1 as libc::c_int as isize),
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
        );
        i = strrchr(((*conn).host).as_mut_ptr(), ']' as i32);
        if !i.is_null() {
            let fresh1 = i;
            i = i.offset(1);
            *fresh1 = 0 as libc::c_int as libc::c_char;
        } else {
            return 0 as libc::c_int
        }
    } else {
        strlcpy(
            ((*conn).host).as_mut_ptr(),
            url.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
        );
        i = ((*conn).host).as_mut_ptr();
        while *i as libc::c_int != 0 && *i as libc::c_int != ':' as i32 {
            i = i.offset(1);
            i;
        }
    }
    if *i as libc::c_int == ':' as i32 {
        *i = 0 as libc::c_int as libc::c_char;
        sscanf(
            i.offset(1 as libc::c_int as isize),
            b"%i\0" as *const u8 as *const libc::c_char,
            &mut (*conn).port as *mut libc::c_int,
        );
        i = ((*conn).host).as_mut_ptr();
    }
    return ((*conn).port > 0 as libc::c_int) as libc::c_int;
}
pub unsafe extern "C" fn scheme_from_proto(
    mut proto: libc::c_int,
) -> *const libc::c_char {
    match proto {
        0 => return b"ftp://\0" as *const u8 as *const libc::c_char,
        1 => return b"ftps://\0" as *const u8 as *const libc::c_char,
        3 => return b"https://\0" as *const u8 as *const libc::c_char,
        2 | _ => return b"http://\0" as *const u8 as *const libc::c_char,
    };
}
pub unsafe extern "C" fn conn_url(
    mut dst: *mut libc::c_char,
    mut len: size_t,
    mut conn: *mut conn_t,
) -> *mut libc::c_char {
    let mut prefix: *const libc::c_char = b"\0" as *const u8 as *const libc::c_char;
    let mut postfix: *const libc::c_char = b"\0" as *const u8 as *const libc::c_char;
    let mut scheme: *const libc::c_char = scheme_from_proto((*conn).proto);
    let mut scheme_len: size_t = strlcpy(dst, scheme, len);
    if scheme_len > len {
        return 0 as *mut libc::c_char;
    }
    len = (len as libc::c_ulong).wrapping_sub(scheme_len) as size_t as size_t;
    let mut p: *mut libc::c_char = dst.offset(scheme_len as isize);
    if *((*conn).user).as_mut_ptr() as libc::c_int != 0 as libc::c_int
        && strcmp(
            ((*conn).user).as_mut_ptr(),
            b"anonymous\0" as *const u8 as *const libc::c_char,
        ) != 0 as libc::c_int
    {
        let mut plen: libc::c_int = snprintf(
            p,
            len,
            b"%s:%s@\0" as *const u8 as *const libc::c_char,
            ((*conn).user).as_mut_ptr(),
            ((*conn).pass).as_mut_ptr(),
        );
        if plen < 0 as libc::c_int {
            return 0 as *mut libc::c_char;
        }
        len = (len as libc::c_ulong).wrapping_sub(plen as libc::c_ulong) as size_t
            as size_t;
        p = p.offset(plen as isize);
    }
    if is_ipv6_addr(((*conn).host).as_mut_ptr()) != 0 {
        prefix = b"[\0" as *const u8 as *const libc::c_char;
        postfix = b"]\0" as *const u8 as *const libc::c_char;
    }
    let mut plen_0: libc::c_int = 0;
    plen_0 = snprintf(
        p,
        len,
        b"%s%s%s:%i%s%s\0" as *const u8 as *const libc::c_char,
        prefix,
        ((*conn).host).as_mut_ptr(),
        postfix,
        (*conn).port,
        ((*conn).dir).as_mut_ptr(),
        ((*conn).file).as_mut_ptr(),
    );
    return if plen_0 < 0 as libc::c_int { 0 as *mut libc::c_char } else { dst };
}
pub unsafe extern "C" fn conn_disconnect(mut conn: *mut conn_t) {
    if (*conn).proto & (1 as libc::c_int) << 1 as libc::c_int
        == (0 as libc::c_int) << 1 as libc::c_int && (*conn).proxy == 0
    {
        ftp_disconnect(((*conn).ftp).as_mut_ptr());
    } else {
        http_disconnect(((*conn).http).as_mut_ptr());
    }
    (*conn).tcp = 0 as *mut tcp_t;
    (*conn).enabled = 0 as libc::c_int != 0;
}
pub unsafe extern "C" fn conn_init(mut conn: *mut conn_t) -> libc::c_int {
    let mut proxy: *mut libc::c_char = ((*(*conn).conf).http_proxy).as_mut_ptr();
    let mut host: *mut libc::c_char = ((*(*conn).conf).no_proxy).as_mut_ptr();
    let mut i: libc::c_int = 0;
    if *((*(*conn).conf).http_proxy).as_mut_ptr() as libc::c_int == 0 as libc::c_int {
        proxy = 0 as *mut libc::c_char;
    } else if *((*(*conn).conf).no_proxy).as_mut_ptr() as libc::c_int != 0 as libc::c_int
    {
        i = 0 as libc::c_int;
        loop {
            if (*(*conn).conf).no_proxy[i as usize] as libc::c_int == 0 as libc::c_int {
                if !(strstr(((*conn).host).as_mut_ptr(), host)).is_null() {
                    proxy = 0 as *mut libc::c_char;
                }
                host = &mut *((*(*conn).conf).no_proxy)
                    .as_mut_ptr()
                    .offset((i + 1 as libc::c_int) as isize) as *mut libc::c_char;
                if (*(*conn).conf).no_proxy[(i + 1 as libc::c_int) as usize]
                    as libc::c_int == 0 as libc::c_int
                {
                    break;
                }
            }
            i += 1;
            i;
        }
    }
    (*conn)
        .proxy = (proxy != 0 as *mut libc::c_void as *mut libc::c_char) as libc::c_int;
    if (*conn).proto & (1 as libc::c_int) << 1 as libc::c_int
        == (0 as libc::c_int) << 1 as libc::c_int && (*conn).proxy == 0
    {
        let ref mut fresh2 = (*((*conn).ftp).as_mut_ptr()).local_if;
        *fresh2 = (*conn).local_if;
        (*((*conn).ftp).as_mut_ptr()).ftp_mode = 1 as libc::c_int;
        (*((*conn).ftp).as_mut_ptr()).tcp.ai_family = (*(*conn).conf).ai_family;
        if ftp_connect(
            ((*conn).ftp).as_mut_ptr(),
            (*conn).proto,
            ((*conn).host).as_mut_ptr(),
            (*conn).port,
            ((*conn).user).as_mut_ptr(),
            ((*conn).pass).as_mut_ptr(),
            (*(*conn).conf).io_timeout,
        ) == 0
        {
            (*conn).message = (*((*conn).ftp).as_mut_ptr()).message;
            conn_disconnect(conn);
            return 0 as libc::c_int;
        }
        (*conn).message = (*((*conn).ftp).as_mut_ptr()).message;
        if ftp_cwd(((*conn).ftp).as_mut_ptr(), ((*conn).dir).as_mut_ptr()) == 0 {
            conn_disconnect(conn);
            return 0 as libc::c_int;
        }
    } else {
        let ref mut fresh3 = (*((*conn).http).as_mut_ptr()).local_if;
        *fresh3 = (*conn).local_if;
        (*((*conn).http).as_mut_ptr()).tcp.ai_family = (*(*conn).conf).ai_family;
        if http_connect(
            ((*conn).http).as_mut_ptr(),
            (*conn).proto,
            proxy,
            ((*conn).host).as_mut_ptr(),
            (*conn).port,
            ((*conn).user).as_mut_ptr(),
            ((*conn).pass).as_mut_ptr(),
            (*(*conn).conf).io_timeout,
        ) == 0
        {
            (*conn).message = (*((*((*conn).http).as_mut_ptr()).headers).as_mut_ptr()).p;
            conn_disconnect(conn);
            return 0 as libc::c_int;
        }
        (*conn).message = (*((*((*conn).http).as_mut_ptr()).headers).as_mut_ptr()).p;
        (*conn).tcp = &mut (*((*conn).http).as_mut_ptr()).tcp;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn conn_setup(mut conn: *mut conn_t) -> libc::c_int {
    if (*((*conn).ftp).as_mut_ptr()).tcp.fd <= 0 as libc::c_int
        && (*((*conn).http).as_mut_ptr()).tcp.fd <= 0 as libc::c_int
    {
        if conn_init(conn) == 0 {
            return 0 as libc::c_int;
        }
    }
    if (*conn).proto & (1 as libc::c_int) << 1 as libc::c_int
        == (0 as libc::c_int) << 1 as libc::c_int && (*conn).proxy == 0
    {
        if ftp_data(((*conn).ftp).as_mut_ptr(), (*(*conn).conf).io_timeout) == 0 {
            return 0 as libc::c_int;
        }
        (*conn).tcp = &mut (*((*conn).ftp).as_mut_ptr()).data_tcp;
        if (*conn).currentbyte != 0 {
            ftp_command(
                ((*conn).ftp).as_mut_ptr(),
                b"REST %jd\0" as *const u8 as *const libc::c_char,
                (*conn).currentbyte,
            );
            if ftp_wait(((*conn).ftp).as_mut_ptr()) / 100 as libc::c_int
                != 3 as libc::c_int
                && (*((*conn).ftp).as_mut_ptr()).status / 100 as libc::c_int
                    != 2 as libc::c_int
            {
                return 0 as libc::c_int;
            }
        }
    } else {
        let mut s: [libc::c_char; 2048] = [0; 2048];
        let mut i: libc::c_int = 0;
        snprintf(
            s.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 2048]>() as libc::c_ulong,
            b"%s%s\0" as *const u8 as *const libc::c_char,
            ((*conn).dir).as_mut_ptr(),
            ((*conn).file).as_mut_ptr(),
        );
        (*((*conn).http).as_mut_ptr())
            .firstbyte = if (*conn).supported as libc::c_int != 0 {
            (*conn).currentbyte
        } else {
            -(1 as libc::c_int) as libc::c_long
        };
        (*((*conn).http).as_mut_ptr()).lastbyte = (*conn).lastbyte;
        abuf_setup(
            ((*((*conn).http).as_mut_ptr()).request).as_mut_ptr(),
            2048 as libc::c_int as size_t,
        );
        http_get(((*conn).http).as_mut_ptr(), s.as_mut_ptr());
        i = 0 as libc::c_int;
        while i < (*(*conn).conf).add_header_count {
            http_addheader(
                ((*conn).http).as_mut_ptr(),
                b"%s\0" as *const u8 as *const libc::c_char,
                ((*(*conn).conf).add_header[i as usize]).as_mut_ptr(),
            );
            i += 1;
            i;
        }
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn conn_exec(mut conn: *mut conn_t) -> libc::c_int {
    if (*conn).proto & (1 as libc::c_int) << 1 as libc::c_int
        == (0 as libc::c_int) << 1 as libc::c_int && (*conn).proxy == 0
    {
        if ftp_command(
            ((*conn).ftp).as_mut_ptr(),
            b"RETR %s\0" as *const u8 as *const libc::c_char,
            ((*conn).file).as_mut_ptr(),
        ) == 0
        {
            return 0 as libc::c_int;
        }
        return (ftp_wait(((*conn).ftp).as_mut_ptr()) / 100 as libc::c_int
            == 1 as libc::c_int) as libc::c_int;
    } else {
        abuf_setup(
            ((*((*conn).http).as_mut_ptr()).headers).as_mut_ptr(),
            1024 as libc::c_int as size_t,
        );
        if http_exec(((*conn).http).as_mut_ptr()) == 0 {
            return 0 as libc::c_int;
        }
        return ((*((*conn).http).as_mut_ptr()).status / 100 as libc::c_int
            == 2 as libc::c_int) as libc::c_int;
    };
}
unsafe extern "C" fn conn_info_ftp(mut conn: *mut conn_t) -> libc::c_int {
    ftp_command(
        ((*conn).ftp).as_mut_ptr(),
        b"REST %d\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
    );
    if ftp_wait(((*conn).ftp).as_mut_ptr()) / 100 as libc::c_int == 3 as libc::c_int
        || (*((*conn).ftp).as_mut_ptr()).status / 100 as libc::c_int == 2 as libc::c_int
    {
        (*conn).supported = 1 as libc::c_int != 0;
        ftp_command(
            ((*conn).ftp).as_mut_ptr(),
            b"REST %d\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
        ftp_wait(((*conn).ftp).as_mut_ptr());
    } else {
        (*conn).supported = 0 as libc::c_int != 0;
    }
    if ftp_cwd(((*conn).ftp).as_mut_ptr(), ((*conn).dir).as_mut_ptr()) == 0 {
        return 0 as libc::c_int;
    }
    (*conn)
        .size = ftp_size(
        ((*conn).ftp).as_mut_ptr(),
        ((*conn).file).as_mut_ptr(),
        (*(*conn).conf).max_redirect,
        (*(*conn).conf).io_timeout,
    );
    if (*conn).size < 0 as libc::c_int as libc::c_long {
        (*conn).supported = 0 as libc::c_int != 0;
    }
    if (*conn).size == -(1 as libc::c_int) as libc::c_long {
        return 0 as libc::c_int
    } else if (*conn).size == -(2 as libc::c_int) as libc::c_long {
        (*conn).size = 9223372036854775807 as libc::c_longlong as off_t;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn conn_info(mut conn: *mut conn_t) -> libc::c_int {
    if (*conn).proto & (1 as libc::c_int) << 1 as libc::c_int
        == (0 as libc::c_int) << 1 as libc::c_int && (*conn).proxy == 0
    {
        return conn_info_ftp(conn);
    }
    let mut s: [libc::c_char; 1005] = [0; 1005];
    let mut i: libc::c_longlong = 0 as libc::c_int as libc::c_longlong;
    loop {
        let mut t: *const libc::c_char = 0 as *const libc::c_char;
        (*conn).supported = 1 as libc::c_int != 0;
        (*conn).currentbyte = 0 as libc::c_int as off_t;
        pthread_mutex_lock(&mut (*conn).lock);
        let mut setup_ret: libc::c_int = conn_setup(conn);
        pthread_mutex_unlock(&mut (*conn).lock);
        if setup_ret == 0 {
            return 0 as libc::c_int;
        }
        conn_exec(conn);
        conn_disconnect(conn);
        http_filename(
            ((*conn).http).as_mut_ptr(),
            ((*conn).output_filename).as_mut_ptr(),
        );
        if (*((*conn).http).as_mut_ptr()).status / 100 as libc::c_int != 3 as libc::c_int
        {
            break;
        }
        t = http_header(
            ((*conn).http).as_mut_ptr(),
            b"location:\0" as *const u8 as *const libc::c_char,
        );
        if t.is_null() {
            return 0 as libc::c_int;
        }
        sscanf(t, b"%1000s\0" as *const u8 as *const libc::c_char, s.as_mut_ptr());
        if s[0 as libc::c_int as usize] as libc::c_int == '/' as i32 {
            abuf_printf(
                ((*((*conn).http).as_mut_ptr()).headers).as_mut_ptr(),
                b"%s%s:%i%s\0" as *const u8 as *const libc::c_char,
                scheme_from_proto((*conn).proto),
                ((*conn).host).as_mut_ptr(),
                (*conn).port,
                s.as_mut_ptr(),
            );
            strlcpy(
                s.as_mut_ptr(),
                (*((*((*conn).http).as_mut_ptr()).headers).as_mut_ptr()).p,
                ::std::mem::size_of::<[libc::c_char; 1005]>() as libc::c_ulong,
            );
        } else if (strstr(s.as_mut_ptr(), b"://\0" as *const u8 as *const libc::c_char))
            .is_null()
        {
            conn_url(
                (*((*((*conn).http).as_mut_ptr()).headers).as_mut_ptr()).p,
                (*((*((*conn).http).as_mut_ptr()).headers).as_mut_ptr()).len,
                conn,
            );
            strlcat(
                (*((*((*conn).http).as_mut_ptr()).headers).as_mut_ptr()).p,
                s.as_mut_ptr(),
                (*((*((*conn).http).as_mut_ptr()).headers).as_mut_ptr()).len,
            );
            strlcpy(
                s.as_mut_ptr(),
                (*((*((*conn).http).as_mut_ptr()).headers).as_mut_ptr()).p,
                ::std::mem::size_of::<[libc::c_char; 1005]>() as libc::c_ulong,
            );
        }
        if conn_set(conn, s.as_mut_ptr()) == 0 {
            return 0 as libc::c_int;
        }
        if (*conn).proto & (1 as libc::c_int) << 1 as libc::c_int
            == (0 as libc::c_int) << 1 as libc::c_int && (*conn).proxy == 0
        {
            return -(1 as libc::c_int);
        }
        i += 1;
        if i >= (*(*conn).conf).max_redirect as libc::c_longlong {
            fprintf(
                stderr,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Too many redirects.\n\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            return 0 as libc::c_int;
        }
        if !((*((*conn).http).as_mut_ptr()).status / 100 as libc::c_int
            == 3 as libc::c_int)
        {
            break;
        }
    }
    if (*((*conn).http).as_mut_ptr()).status != 416 as libc::c_int
        && (*((*conn).http).as_mut_ptr()).status / 100 as libc::c_int != 2 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    (*conn).size = http_size_from_range(((*conn).http).as_mut_ptr());
    (*conn)
        .supported = (*((*conn).http).as_mut_ptr()).status == 206 as libc::c_int
        || (*conn).size > 0 as libc::c_int as libc::c_long;
    if (*conn).size <= 0 as libc::c_int as libc::c_long {
        match (*((*conn).http).as_mut_ptr()).status {
            200 | 416 => {
                (*conn).supported = 0 as libc::c_int != 0;
            }
            206 => {}
            _ => return 0 as libc::c_int,
        }
    }
    (*conn)
        .size = ({
        let mut __a: off_t = (*conn).size;
        let mut __b: off_t = http_size(((*conn).http).as_mut_ptr());
        if __a > __b { __a } else { __b }
    });
    if (*conn).size <= 0 as libc::c_int as libc::c_long {
        (*conn).size = 9223372036854775807 as libc::c_longlong as off_t;
        (*conn).supported = 0 as libc::c_int != 0;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn conn_info_status_get(
    mut msg: *mut libc::c_char,
    mut size: size_t,
    mut conn: *mut conn_t,
) -> libc::c_int {
    if is_proto_http((*conn).proto) != 0 {
        let mut p: *mut libc::c_char = (*((*((*conn).http).as_mut_ptr()).headers)
            .as_mut_ptr())
            .p;
        loop {
            let fresh4 = p;
            p = p.offset(1);
            if !(*fresh4 as libc::c_int != ' ' as i32) {
                break;
            }
        }
        loop {
            let fresh5 = p;
            p = p.offset(1);
            if !(*fresh5 as libc::c_int != ' ' as i32) {
                break;
            }
        }
        let mut len: size_t = strcspn(p, b"\r\n\0" as *const u8 as *const libc::c_char);
        if len != 0 {
            strlcpy(
                msg,
                p,
                ({
                    let mut __a: libc::c_ulong = len
                        .wrapping_add(1 as libc::c_int as libc::c_ulong);
                    let mut __b: size_t = size;
                    if __a < __b { __a } else { __b }
                }),
            );
            return (*((*conn).http).as_mut_ptr()).status;
        }
    }
    strlcpy(
        msg,
        dcgettext(
            0 as *const libc::c_char,
            b"Unknown Error\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        size,
    );
    return -(1 as libc::c_int);
}
