use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn __errno_location() -> *mut libc::c_int;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn vsnprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn strtoll(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_longlong;
    fn scheme_from_proto(proto: libc::c_int) -> *const libc::c_char;
    fn conn_set(conn: *mut conn_t, set_url: *const libc::c_char) -> libc::c_int;
    fn tcp_write(
        tcp: *mut tcp_t,
        buffer: *mut libc::c_void,
        size: libc::c_int,
    ) -> ssize_t;
    fn tcp_read(
        tcp: *mut tcp_t,
        buffer: *mut libc::c_void,
        size: libc::c_int,
    ) -> ssize_t;
    fn tcp_close(tcp: *mut tcp_t);
    fn tcp_connect(
        tcp: *mut tcp_t,
        hostname: *mut libc::c_char,
        port: libc::c_int,
        secure: libc::c_int,
        local_if: *mut libc::c_char,
        io_timeout: libc::c_uint,
    ) -> libc::c_int;
    fn is_ipv6_addr(hostname: *const libc::c_char) -> libc::c_int;
    fn abuf_strcat(abuf: *mut abuf_t, src: *const libc::c_char) -> libc::c_int;
    fn abuf_setup(abuf: *mut abuf_t, len: size_t) -> libc::c_int;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn strlcpy(_: *mut libc::c_char, _: *const libc::c_char, _: size_t) -> size_t;
    fn strlcat(_: *mut libc::c_char, _: *const libc::c_char, _: size_t) -> size_t;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    fn strpbrk(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strncasecmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
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
pub type size_t = libc::c_ulong;
pub type __uint16_t = libc::c_ushort;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type off_t = __off_t;
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
pub type ssize_t = __ssize_t;
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
unsafe extern "C" fn is_default_port(
    mut proto: libc::c_int,
    mut port: libc::c_int,
) -> libc::c_int {
    return (proto
        == (1 as libc::c_int) << 1 as libc::c_int
            | (0 as libc::c_int) << 0 as libc::c_int && port == 80 as libc::c_int
        || proto
            == (1 as libc::c_int) << 1 as libc::c_int
                | (1 as libc::c_int) << 0 as libc::c_int && port == 443 as libc::c_int)
        as libc::c_int;
}
#[inline]
unsafe extern "C" fn chain_next(mut p: *mut *mut *const libc::c_char) -> libc::c_char {
    while !(**p).is_null() && ***p == 0 {
        *p = (*p).offset(1);
        *p;
    }
    return (if !(**p).is_null() {
        let fresh0 = **p;
        **p = (**p).offset(1);
        *fresh0 as libc::c_int
    } else {
        0 as libc::c_int
    }) as libc::c_char;
}
unsafe extern "C" fn http_auth_token(
    mut token: *mut libc::c_char,
    mut user: *const libc::c_char,
    mut pass: *const libc::c_char,
) {
    let base64_encode: [libc::c_char; 64] = *::std::mem::transmute::<
        &[u8; 64],
        &[libc::c_char; 64],
    >(b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/");
    let mut auth: [*const libc::c_char; 4] = [
        user,
        b":\0" as *const u8 as *const libc::c_char,
        pass,
        0 as *const libc::c_char,
    ];
    let mut p: *mut *const libc::c_char = auth.as_mut_ptr();
    while !(*p).is_null() {
        let mut a: libc::c_char = chain_next(&mut p);
        if a == 0 {
            break;
        }
        let fresh1 = token;
        token = token.offset(1);
        *fresh1 = base64_encode[(a as libc::c_int >> 2 as libc::c_int) as usize];
        let mut b: libc::c_char = chain_next(&mut p);
        let fresh2 = token;
        token = token.offset(1);
        *fresh2 = base64_encode[((a as libc::c_int & 3 as libc::c_int)
            << 4 as libc::c_int | b as libc::c_int >> 4 as libc::c_int) as usize];
        if b == 0 {
            let fresh3 = token;
            token = token.offset(1);
            *fresh3 = '=' as i32 as libc::c_char;
            let fresh4 = token;
            token = token.offset(1);
            *fresh4 = '=' as i32 as libc::c_char;
            break;
        } else {
            let mut c: libc::c_char = chain_next(&mut p);
            let fresh5 = token;
            token = token.offset(1);
            *fresh5 = base64_encode[((b as libc::c_int & 15 as libc::c_int)
                << 2 as libc::c_int | c as libc::c_int >> 6 as libc::c_int) as usize];
            if c == 0 {
                let fresh6 = token;
                token = token.offset(1);
                *fresh6 = '=' as i32 as libc::c_char;
                break;
            } else {
                let fresh7 = token;
                token = token.offset(1);
                *fresh7 = base64_encode[(c as libc::c_int & 63 as libc::c_int) as usize];
            }
        }
    }
}
pub unsafe extern "C" fn http_connect(
    mut conn: *mut http_t,
    mut proto: libc::c_int,
    mut proxy: *mut libc::c_char,
    mut host: *mut libc::c_char,
    mut port: libc::c_int,
    mut user: *mut libc::c_char,
    mut pass: *mut libc::c_char,
    mut io_timeout: libc::c_uint,
) -> libc::c_int {
    let mut puser: *const libc::c_char = 0 as *const libc::c_char;
    let mut ppass: *const libc::c_char = b"\0" as *const u8 as *const libc::c_char;
    let mut tconn: [conn_t; 1] = [conn_t {
        conf: 0 as *mut conf_t,
        proto: 0,
        port: 0,
        proxy: 0,
        host: [0; 1024],
        dir: [0; 1024],
        file: [0; 1024],
        user: [0; 1024],
        pass: [0; 1024],
        output_filename: [0; 1024],
        ftp: [ftp_t {
            cwd: [0; 1024],
            message: 0 as *mut libc::c_char,
            status: 0,
            tcp: tcp_t { fd: 0, ai_family: 0 },
            data_tcp: tcp_t { fd: 0, ai_family: 0 },
            proto: 0,
            ftp_mode: 0,
            local_if: 0 as *mut libc::c_char,
        }; 1],
        http: [http_t {
            host: [0; 1024],
            auth: [0; 1024],
            request: [abuf_t {
                p: 0 as *mut libc::c_char,
                len: 0,
            }; 1],
            headers: [abuf_t {
                p: 0 as *mut libc::c_char,
                len: 0,
            }; 1],
            port: 0,
            proto: 0,
            proxy: 0,
            proxy_auth: [0; 1024],
            firstbyte: 0,
            lastbyte: 0,
            status: 0,
            tcp: tcp_t { fd: 0, ai_family: 0 },
            local_if: 0 as *mut libc::c_char,
        }; 1],
        size: 0,
        currentbyte: 0,
        lastbyte: 0,
        tcp: 0 as *mut tcp_t,
        enabled: false,
        supported: false,
        last_transfer: 0,
        message: 0 as *mut libc::c_char,
        local_if: 0 as *mut libc::c_char,
        state: false,
        setup_thread: [0; 1],
        lock: pthread_mutex_t {
            __data: __pthread_mutex_s {
                __lock: 0,
                __count: 0,
                __owner: 0,
                __nusers: 0,
                __kind: 0,
                __spins: 0,
                __elision: 0,
                __list: __pthread_list_t {
                    __prev: 0 as *mut __pthread_internal_list,
                    __next: 0 as *mut __pthread_internal_list,
                },
            },
        },
    }; 1];
    strlcpy(
        ((*conn).host).as_mut_ptr(),
        host,
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
    );
    (*conn).port = port;
    (*conn).proto = proto;
    if !proxy.is_null() && *proxy as libc::c_int != 0 {
        if conn_set(tconn.as_mut_ptr(), proxy) == 0 {
            fprintf(
                stderr,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Invalid proxy string: %s\n\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                proxy,
            );
            return 0 as libc::c_int;
        }
        host = ((*tconn.as_mut_ptr()).host).as_mut_ptr();
        port = (*tconn.as_mut_ptr()).port;
        proto = (*tconn.as_mut_ptr()).proto;
        puser = ((*tconn.as_mut_ptr()).user).as_mut_ptr();
        ppass = ((*tconn.as_mut_ptr()).pass).as_mut_ptr();
        (*conn).proxy = 1 as libc::c_int;
    }
    if tcp_connect(
        &mut (*conn).tcp,
        host,
        port,
        (proto & (1 as libc::c_int) << 0 as libc::c_int
            == (1 as libc::c_int) << 0 as libc::c_int) as libc::c_int,
        (*conn).local_if,
        io_timeout,
    ) == -(1 as libc::c_int)
    {
        return 0 as libc::c_int;
    }
    if *user as libc::c_int == 0 as libc::c_int {
        *((*conn).auth).as_mut_ptr() = 0 as libc::c_int as libc::c_char;
    } else {
        http_auth_token(((*conn).auth).as_mut_ptr(), user, pass);
    }
    if (*conn).proxy == 0 || puser.is_null() || *puser as libc::c_int == 0 as libc::c_int
    {
        *((*conn).proxy_auth).as_mut_ptr() = 0 as libc::c_int as libc::c_char;
    } else {
        http_auth_token(((*conn).proxy_auth).as_mut_ptr(), puser, ppass);
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn http_disconnect(mut conn: *mut http_t) {
    tcp_close(&mut (*conn).tcp);
}
pub unsafe extern "C" fn http_get(mut conn: *mut http_t, mut lurl: *mut libc::c_char) {
    let mut prefix: *const libc::c_char = b"\0" as *const u8 as *const libc::c_char;
    let mut postfix: *const libc::c_char = b"\0" as *const u8 as *const libc::c_char;
    if is_ipv6_addr(((*conn).host).as_mut_ptr()) != 0 {
        prefix = b"[\0" as *const u8 as *const libc::c_char;
        postfix = b"]\0" as *const u8 as *const libc::c_char;
    }
    *(*((*conn).request).as_mut_ptr()).p = 0 as libc::c_int as libc::c_char;
    if (*conn).proxy != 0 {
        let mut proto: *const libc::c_char = scheme_from_proto((*conn).proto);
        if is_default_port((*conn).proto, (*conn).port) != 0 {
            http_addheader(
                conn,
                b"GET %s%s%s%s%s HTTP/1.0\0" as *const u8 as *const libc::c_char,
                proto,
                prefix,
                ((*conn).host).as_mut_ptr(),
                postfix,
                lurl,
            );
        } else {
            http_addheader(
                conn,
                b"GET %s%s%s%s:%i%s HTTP/1.0\0" as *const u8 as *const libc::c_char,
                proto,
                prefix,
                ((*conn).host).as_mut_ptr(),
                postfix,
                (*conn).port,
                lurl,
            );
        }
    } else {
        http_addheader(
            conn,
            b"GET %s HTTP/1.0\0" as *const u8 as *const libc::c_char,
            lurl,
        );
        if is_default_port((*conn).proto, (*conn).port) != 0 {
            http_addheader(
                conn,
                b"Host: %s%s%s\0" as *const u8 as *const libc::c_char,
                prefix,
                ((*conn).host).as_mut_ptr(),
                postfix,
            );
        } else {
            http_addheader(
                conn,
                b"Host: %s%s%s:%i\0" as *const u8 as *const libc::c_char,
                prefix,
                ((*conn).host).as_mut_ptr(),
                postfix,
                (*conn).port,
            );
        }
    }
    if *((*conn).auth).as_mut_ptr() != 0 {
        http_addheader(
            conn,
            b"Authorization: Basic %s\0" as *const u8 as *const libc::c_char,
            ((*conn).auth).as_mut_ptr(),
        );
    }
    if *((*conn).proxy_auth).as_mut_ptr() != 0 {
        http_addheader(
            conn,
            b"Proxy-Authorization: Basic %s\0" as *const u8 as *const libc::c_char,
            ((*conn).proxy_auth).as_mut_ptr(),
        );
    }
    http_addheader(conn, b"Accept: */*\0" as *const u8 as *const libc::c_char);
    http_addheader(
        conn,
        b"Accept-Encoding: identity\0" as *const u8 as *const libc::c_char,
    );
    if (*conn).lastbyte != 0 && (*conn).firstbyte >= 0 as libc::c_int as libc::c_long {
        http_addheader(
            conn,
            b"Range: bytes=%jd-%jd\0" as *const u8 as *const libc::c_char,
            (*conn).firstbyte,
            (*conn).lastbyte - 1 as libc::c_int as libc::c_long,
        );
    } else if (*conn).firstbyte >= 0 as libc::c_int as libc::c_long {
        http_addheader(
            conn,
            b"Range: bytes=%jd-\0" as *const u8 as *const libc::c_char,
            (*conn).firstbyte,
        );
    }
}
pub unsafe extern "C" fn http_addheader(
    mut conn: *mut http_t,
    mut format: *const libc::c_char,
    mut args: ...
) {
    let mut s: [libc::c_char; 1024] = [0; 1024];
    let mut params: ::std::ffi::VaListImpl;
    params = args.clone();
    vsnprintf(
        s.as_mut_ptr(),
        (::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong)
            .wrapping_sub(3 as libc::c_int as libc::c_ulong),
        format,
        params.as_va_list(),
    );
    strlcat(
        s.as_mut_ptr(),
        b"\r\n\0" as *const u8 as *const libc::c_char,
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
    );
    if abuf_strcat(((*conn).request).as_mut_ptr(), s.as_mut_ptr()) < 0 as libc::c_int {
        fprintf(stderr, b"Out of memory\n\0" as *const u8 as *const libc::c_char);
    }
}
pub unsafe extern "C" fn http_exec(mut conn: *mut http_t) -> libc::c_int {
    let mut s2: *mut libc::c_char = 0 as *mut libc::c_char;
    strlcat(
        (*((*conn).request).as_mut_ptr()).p,
        b"\r\n\0" as *const u8 as *const libc::c_char,
        (*((*conn).request).as_mut_ptr()).len,
    );
    let reqlen: size_t = strlen((*((*conn).request).as_mut_ptr()).p);
    let mut nwrite: size_t = 0 as libc::c_int as size_t;
    while nwrite < reqlen {
        let mut tmp: ssize_t = 0;
        tmp = tcp_write(
            &mut (*conn).tcp,
            ((*((*conn).request).as_mut_ptr()).p).offset(nwrite as isize)
                as *mut libc::c_void,
            reqlen.wrapping_sub(nwrite) as libc::c_int,
        );
        if tmp < 0 as libc::c_int as libc::c_long {
            if *__errno_location() == 4 as libc::c_int
                || *__errno_location() == 11 as libc::c_int
            {
                continue;
            }
            fprintf(
                stderr,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Connection gone while writing.\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            return 0 as libc::c_int;
        } else {
            nwrite = (nwrite as libc::c_ulong).wrapping_add(tmp as libc::c_ulong)
                as size_t as size_t;
        }
    }
    *(*((*conn).headers).as_mut_ptr()).p = 0 as libc::c_int as libc::c_char;
    let mut s: *mut libc::c_char = (*((*conn).headers).as_mut_ptr()).p;
    loop {
        if tcp_read(&mut (*conn).tcp, s as *mut libc::c_void, 1 as libc::c_int)
            <= 0 as libc::c_int as libc::c_long
        {
            fprintf(
                stderr,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Connection gone.\n\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            return 0 as libc::c_int;
        }
        if *s as libc::c_int == '\r' as i32 {
            continue;
        }
        if *s as libc::c_int == '\n' as i32 {
            if s > (*((*conn).headers).as_mut_ptr()).p
                && *s.offset(-(1 as libc::c_int) as isize) as libc::c_int == '\n' as i32
            {
                *s = 0 as libc::c_int as libc::c_char;
                break;
            }
        }
        s = s.offset(1);
        s;
        let mut pos: size_t = s.offset_from((*((*conn).headers).as_mut_ptr()).p)
            as libc::c_long as size_t;
        if pos.wrapping_add(10 as libc::c_int as libc::c_ulong)
            < (*((*conn).headers).as_mut_ptr()).len
        {
            let mut tmp_0: libc::c_int = abuf_setup(
                ((*conn).headers).as_mut_ptr(),
                ((*((*conn).headers).as_mut_ptr()).len)
                    .wrapping_add(512 as libc::c_int as libc::c_ulong),
            );
            if tmp_0 < 0 as libc::c_int {
                fprintf(
                    stderr,
                    b"Out of memory\n\0" as *const u8 as *const libc::c_char,
                );
                return 0 as libc::c_int;
            }
            s = ((*((*conn).headers).as_mut_ptr()).p).offset(pos as isize);
        }
    }
    sscanf(
        (*((*conn).headers).as_mut_ptr()).p,
        b"%*s %3i\0" as *const u8 as *const libc::c_char,
        &mut (*conn).status as *mut libc::c_int,
    );
    s2 = strchr((*((*conn).headers).as_mut_ptr()).p, '\n' as i32);
    if !s2.is_null() {
        *s2 = 0 as libc::c_int as libc::c_char;
    }
    let reslen: size_t = (s2.offset_from((*((*conn).headers).as_mut_ptr()).p)
        as libc::c_long + 1 as libc::c_int as libc::c_long) as size_t;
    if (*((*conn).request).as_mut_ptr()).len < reqlen {
        let mut ret: libc::c_int = abuf_setup(((*conn).request).as_mut_ptr(), reslen);
        if ret < 0 as libc::c_int {
            return 0 as libc::c_int;
        }
    }
    memcpy(
        (*((*conn).request).as_mut_ptr()).p as *mut libc::c_void,
        (*((*conn).headers).as_mut_ptr()).p as *const libc::c_void,
        reslen,
    );
    *s2 = '\n' as i32 as libc::c_char;
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn http_header(
    mut conn: *const http_t,
    mut header: *const libc::c_char,
) -> *const libc::c_char {
    let mut p: *const libc::c_char = (*((*conn).headers).as_ptr()).p;
    let mut hlen: size_t = strlen(header);
    loop {
        if strncasecmp(p, header, hlen) == 0 as libc::c_int {
            return p.offset(hlen as isize);
        }
        while *p as libc::c_int != '\n' as i32 && *p as libc::c_int != 0 {
            p = p.offset(1);
            p;
        }
        if *p as libc::c_int == '\n' as i32 {
            p = p.offset(1);
            p;
        }
        if !(*p != 0) {
            break;
        }
    }
    return 0 as *const libc::c_char;
}
pub unsafe extern "C" fn http_size(mut conn: *mut http_t) -> off_t {
    let mut i: *const libc::c_char = 0 as *const libc::c_char;
    let mut j: off_t = 0;
    i = http_header(conn, b"Content-Length:\0" as *const u8 as *const libc::c_char);
    if i.is_null() {
        return -(2 as libc::c_int) as off_t;
    }
    sscanf(i, b"%jd\0" as *const u8 as *const libc::c_char, &mut j as *mut off_t);
    return j;
}
pub unsafe extern "C" fn http_size_from_range(mut conn: *mut http_t) -> off_t {
    let mut i: *const libc::c_char = 0 as *const libc::c_char;
    i = http_header(conn, b"Content-Range:\0" as *const u8 as *const libc::c_char);
    if i.is_null() {
        return -(2 as libc::c_int) as off_t;
    }
    i = strchr(i, '/' as i32);
    let fresh8 = i;
    i = i.offset(1);
    if fresh8.is_null() {
        return -(2 as libc::c_int) as off_t;
    }
    let mut j: off_t = strtoll(i, 0 as *mut *mut libc::c_char, 10 as libc::c_int)
        as off_t;
    if j == 0 && *i as libc::c_int != '0' as i32 {
        return -(3 as libc::c_int) as off_t;
    }
    return j;
}
pub unsafe extern "C" fn http_filename(
    mut conn: *const http_t,
    mut filename: *mut libc::c_char,
) {
    let mut h: *const libc::c_char = 0 as *const libc::c_char;
    h = http_header(conn, b"Content-Disposition:\0" as *const u8 as *const libc::c_char);
    if !h.is_null() {
        sscanf(
            h,
            b"%*s%*[ \t]filename%*[ \t=\"'-]%254[^\n\"']\0" as *const u8
                as *const libc::c_char,
            filename,
        );
        let space: [libc::c_char; 3] = *::std::mem::transmute::<
            &[u8; 3],
            &[libc::c_char; 3],
        >(b"\t \0");
        let mut n: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut p: *mut libc::c_char = filename;
        loop {
            p = strpbrk(p, space.as_ptr());
            if p.is_null() {
                break;
            }
            n = p.offset(strspn(p, space.as_ptr()) as isize);
            if *n == 0 {
                *p = 0 as libc::c_int as libc::c_char;
                break;
            } else {
                p = n;
            }
        }
        let invalid: [libc::c_char; 10] = *::std::mem::transmute::<
            &[u8; 10],
            &[libc::c_char; 10],
        >(b"/\\?%*:|<>\0");
        let replacement: libc::c_char = '_' as i32 as libc::c_char;
        let mut i: *mut libc::c_char = filename;
        loop {
            i = strpbrk(i, invalid.as_ptr());
            if i.is_null() {
                break;
            }
            *i = replacement;
            i = i.offset(1);
            i;
        }
    }
}
#[inline]
unsafe extern "C" fn decode_nibble(mut n: libc::c_char) -> libc::c_char {
    if n as libc::c_int <= '9' as i32 {
        return (n as libc::c_int - '0' as i32) as libc::c_char;
    }
    if n as libc::c_int >= 'a' as i32 {
        n = (n as libc::c_int - ('a' as i32 - 'A' as i32)) as libc::c_char;
    }
    return (n as libc::c_int - 'A' as i32 + 10 as libc::c_int) as libc::c_char;
}
#[inline]
unsafe extern "C" fn encode_nibble(mut n: libc::c_char) -> libc::c_char {
    return (if n as libc::c_int > 9 as libc::c_int {
        n as libc::c_int + 'a' as i32 - 10 as libc::c_int
    } else {
        n as libc::c_int + '0' as i32
    }) as libc::c_char;
}
#[inline]
unsafe extern "C" fn encode_byte(mut dst: *mut libc::c_char, mut n: libc::c_char) {
    let fresh9 = dst;
    dst = dst.offset(1);
    *fresh9 = '%' as i32 as libc::c_char;
    let fresh10 = dst;
    dst = dst.offset(1);
    *fresh10 = encode_nibble((n as libc::c_int >> 4 as libc::c_int) as libc::c_char);
    *dst = encode_nibble((n as libc::c_int & 15 as libc::c_int) as libc::c_char);
}
pub unsafe extern "C" fn http_decode(mut s: *mut libc::c_char) {
    while *s as libc::c_int != 0 && *s as libc::c_int != '%' as i32 {
        s = s.offset(1);
        s;
    }
    if *s == 0 {
        return;
    }
    let mut p: *mut libc::c_char = s;
    while !(*s.offset(1 as libc::c_int as isize) == 0
        || *s.offset(2 as libc::c_int as isize) == 0)
    {
        let fresh11 = p;
        p = p.offset(1);
        *fresh11 = ((decode_nibble(*s.offset(1 as libc::c_int as isize)) as libc::c_int)
            << 4 as libc::c_int
            | decode_nibble(*s.offset(2 as libc::c_int as isize)) as libc::c_int)
            as libc::c_char;
        s = s.offset(3 as libc::c_int as isize);
        while *s as libc::c_int != 0 && *s as libc::c_int != '%' as i32 {
            let fresh12 = s;
            s = s.offset(1);
            let fresh13 = p;
            p = p.offset(1);
            *fresh13 = *fresh12;
        }
        if !(*s as libc::c_int == '%' as i32) {
            break;
        }
    }
    *p = 0 as libc::c_int as libc::c_char;
}
pub unsafe extern "C" fn http_encode(mut s: *mut libc::c_char, mut len: size_t) {
    let mut t: [libc::c_char; 1024] = [0; 1024];
    let mut i: libc::c_uint = 0;
    let mut j: libc::c_uint = 0;
    j = 0 as libc::c_int as libc::c_uint;
    i = j;
    while *s.offset(i as isize) as libc::c_int != 0
        && (j as libc::c_ulong)
            < (::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
    {
        t[j as usize] = *s.offset(i as isize);
        if *s.offset(i as isize) as libc::c_int <= 0x20 as libc::c_int
            || *s.offset(i as isize) as libc::c_int >= 0x7f as libc::c_int
        {
            if j as libc::c_ulong
                >= (::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong)
                    .wrapping_sub(3 as libc::c_int as libc::c_ulong)
            {
                break;
            }
            encode_byte(t.as_mut_ptr().offset(j as isize), *s.offset(i as isize));
            j = j.wrapping_add(2 as libc::c_int as libc::c_uint);
        }
        i = i.wrapping_add(1);
        i;
        j = j.wrapping_add(1);
        j;
    }
    t[j as usize] = 0 as libc::c_int as libc::c_char;
    strlcpy(s, t.as_mut_ptr(), len);
}
