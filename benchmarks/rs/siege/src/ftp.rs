use ::libc;
extern "C" {
    pub type URL_T;
    pub type AUTH_T;
    pub type DIGEST_CRED;
    pub type DIGEST_CHLG;
    pub type ARRAY_T;
    pub type COOKIES_T;
    pub type x509_st;
    pub type ssl_st;
    pub type ssl_ctx_st;
    pub type ssl_method_st;
    pub type PAGE_T;
    pub type CACHE_T;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
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
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    static mut my: CONFIG;
    fn pthread_self() -> pthread_t;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn url_get_username(this: URL) -> *mut libc::c_char;
    fn url_get_password(this: URL) -> *mut libc::c_char;
    fn url_get_path(this: URL) -> *mut libc::c_char;
    fn url_get_file(this: URL) -> *mut libc::c_char;
    fn url_get_postlen(this: URL) -> size_t;
    fn url_get_postdata(this: URL) -> *mut libc::c_char;
    fn okay(code: libc::c_int) -> BOOLEAN;
    fn echo(fmt: *const libc::c_char, _: ...);
    fn debug(fmt: *const libc::c_char, _: ...);
    fn socket_read(conn: *mut CONN, buf: *mut libc::c_void, len: size_t) -> ssize_t;
    fn socket_write(conn: *mut CONN, b: *const libc::c_void, n: size_t) -> libc::c_int;
    fn write_file(U: URL, buf: *mut libc::c_char, len: size_t);
    fn chomp(str: *mut libc::c_char) -> *mut libc::c_char;
    fn split(
        pattern: libc::c_char,
        s: *mut libc::c_char,
        n_words: *mut libc::c_int,
    ) -> *mut *mut libc::c_char;
    fn split_free(split_0: *mut *mut libc::c_char, length: libc::c_int);
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    fn xfree(ptr: *mut libc::c_void);
    fn NOTIFY(L: LEVEL, fmt: *const libc::c_char, _: ...);
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
pub type va_list = __builtin_va_list;
pub type __ssize_t = libc::c_long;
pub type ssize_t = __ssize_t;
pub type __fd_mask = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fd_set {
    pub fds_bits: [__fd_mask; 16],
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_cond_s {
    pub c2rust_unnamed: C2RustUnnamed_1,
    pub c2rust_unnamed_0: C2RustUnnamed,
    pub __g_refs: [libc::c_uint; 2],
    pub __g_size: [libc::c_uint; 2],
    pub __g1_orig_size: libc::c_uint,
    pub __wrefs: libc::c_uint,
    pub __g_signals: [libc::c_uint; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub __g1_start: libc::c_ulonglong,
    pub __g1_start32: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub __low: libc::c_uint,
    pub __high: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub __wseq: libc::c_ulonglong,
    pub __wseq32: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub __low: libc::c_uint,
    pub __high: libc::c_uint,
}
pub type pthread_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
    pub __align: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_cond_t {
    pub __data: __pthread_cond_s,
    pub __size: [libc::c_char; 48],
    pub __align: libc::c_longlong,
}
pub type C2RustUnnamed_3 = libc::c_uint;
pub const _ISalnum: C2RustUnnamed_3 = 8;
pub const _ISpunct: C2RustUnnamed_3 = 4;
pub const _IScntrl: C2RustUnnamed_3 = 2;
pub const _ISblank: C2RustUnnamed_3 = 1;
pub const _ISgraph: C2RustUnnamed_3 = 32768;
pub const _ISprint: C2RustUnnamed_3 = 16384;
pub const _ISspace: C2RustUnnamed_3 = 8192;
pub const _ISxdigit: C2RustUnnamed_3 = 4096;
pub const _ISdigit: C2RustUnnamed_3 = 2048;
pub const _ISalpha: C2RustUnnamed_3 = 1024;
pub const _ISlower: C2RustUnnamed_3 = 512;
pub const _ISupper: C2RustUnnamed_3 = 256;
pub type BOOLEAN = libc::c_uint;
pub const boolean_true: BOOLEAN = 1;
pub const boolean_false: BOOLEAN = 0;
pub type URL = *mut URL_T;
pub type METHOD = libc::c_uint;
pub const PATCH: METHOD = 9;
pub const CONNECT: METHOD = 8;
pub const OPTIONS: METHOD = 7;
pub const TRACE: METHOD = 6;
pub const DELETE: METHOD = 5;
pub const PUT: METHOD = 4;
pub const POST: METHOD = 3;
pub const GET: METHOD = 2;
pub const HEAD: METHOD = 1;
pub const NOMETHOD: METHOD = 0;
pub type SCHEME = libc::c_uint;
pub const PROXY: SCHEME = 4;
pub const FTP: SCHEME = 3;
pub const HTTPS: SCHEME = 2;
pub const HTTP: SCHEME = 1;
pub const UNSUPPORTED: SCHEME = 0;
pub type AUTH = *mut AUTH_T;
pub type DCRED = DIGEST_CRED;
pub type DCHLG = DIGEST_CHLG;
pub type TYPE = libc::c_uint;
pub const NTLM: TYPE = 2;
pub const DIGEST: TYPE = 1;
pub const BASIC: TYPE = 0;
pub type ARRAY = *mut ARRAY_T;
pub type COOKIES = *mut COOKIES_T;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LINES {
    pub index: libc::c_int,
    pub line: *mut *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CONFIG {
    pub logging: BOOLEAN,
    pub shlog: BOOLEAN,
    pub limit: libc::c_int,
    pub url: *mut libc::c_char,
    pub logfile: [libc::c_char; 128],
    pub verbose: BOOLEAN,
    pub quiet: BOOLEAN,
    pub parser: BOOLEAN,
    pub csv: BOOLEAN,
    pub fullurl: BOOLEAN,
    pub display: BOOLEAN,
    pub config: BOOLEAN,
    pub color: BOOLEAN,
    pub cusers: libc::c_int,
    pub delay: libc::c_float,
    pub timeout: libc::c_int,
    pub bench: BOOLEAN,
    pub internet: BOOLEAN,
    pub timestamp: BOOLEAN,
    pub time: libc::c_int,
    pub secs: libc::c_int,
    pub reps: libc::c_int,
    pub file: [libc::c_char; 128],
    pub length: libc::c_int,
    pub nomap: *mut LINES,
    pub debug: BOOLEAN,
    pub chunked: BOOLEAN,
    pub unique: BOOLEAN,
    pub get: BOOLEAN,
    pub print: BOOLEAN,
    pub mark: BOOLEAN,
    pub markstr: *mut libc::c_char,
    pub protocol: libc::c_int,
    pub cookies: COOKIES,
    pub uagent: [libc::c_char; 256],
    pub encoding: [libc::c_char; 256],
    pub conttype: [libc::c_char; 256],
    pub bids: libc::c_int,
    pub auth: AUTH,
    pub keepalive: BOOLEAN,
    pub signaled: libc::c_int,
    pub extra: [libc::c_char; 2048],
    pub login: BOOLEAN,
    pub loginurl: *mut libc::c_char,
    pub lurl: ARRAY,
    pub failures: libc::c_int,
    pub failed: libc::c_int,
    pub escape: BOOLEAN,
    pub expire: BOOLEAN,
    pub follow: BOOLEAN,
    pub zero_ok: BOOLEAN,
    pub spinner: BOOLEAN,
    pub cache: BOOLEAN,
    pub rc: [libc::c_char; 256],
    pub ssl_timeout: libc::c_int,
    pub ssl_cert: *mut libc::c_char,
    pub ssl_key: *mut libc::c_char,
    pub ssl_ciphers: *mut libc::c_char,
    pub method: METHOD,
    pub json_output: BOOLEAN,
    pub cond: pthread_cond_t,
    pub lock: pthread_mutex_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pollfd {
    pub fd: libc::c_int,
    pub events: libc::c_short,
    pub revents: libc::c_short,
}
pub type X509 = x509_st;
pub type SSL = ssl_st;
pub type SSL_CTX = ssl_ctx_st;
pub type SSL_METHOD = ssl_method_st;
pub type PAGE = *mut PAGE_T;
pub type CACHE = *mut CACHE_T;
pub type S_STATUS = libc::c_uint;
pub const S_DONE: S_STATUS = 8;
pub const S_WRITING: S_STATUS = 4;
pub const S_READING: S_STATUS = 2;
pub const S_CONNECTING: S_STATUS = 1;
pub type SDSET = libc::c_uint;
pub const RDWR: SDSET = 3;
pub const WRITE: SDSET = 2;
pub const READ: SDSET = 1;
pub const UNDEF: SDSET = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CONN {
    pub sock: libc::c_int,
    pub status: S_STATUS,
    pub encrypt: BOOLEAN,
    pub scheme: SCHEME,
    pub page: PAGE,
    pub cache: CACHE,
    pub content: C2RustUnnamed_8,
    pub connection: C2RustUnnamed_7,
    pub auth: C2RustUnnamed_5,
    pub ssl: *mut SSL,
    pub ctx: *mut SSL_CTX,
    pub method: *const SSL_METHOD,
    pub cert: *mut X509,
    pub inbuffer: size_t,
    pub pos_ini: libc::c_int,
    pub buffer: [libc::c_char; 4096],
    pub chkbuf: [libc::c_char; 1024],
    pub pfd: [pollfd; 1],
    pub ws: *mut fd_set,
    pub rs: *mut fd_set,
    pub state: SDSET,
    pub ftp: C2RustUnnamed_4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub code: libc::c_int,
    pub host: [libc::c_char; 64],
    pub port: libc::c_int,
    pub size: size_t,
    pub pasv: BOOLEAN,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub wchlg: *mut DCHLG,
    pub wcred: *mut DCRED,
    pub www: libc::c_int,
    pub pchlg: *mut DCHLG,
    pub pcred: *mut DCRED,
    pub proxy: libc::c_int,
    pub type_0: C2RustUnnamed_6,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub www: TYPE,
    pub proxy: TYPE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub max: libc::c_int,
    pub timeout: libc::c_int,
    pub reuse: libc::c_int,
    pub status: libc::c_int,
    pub keepalive: libc::c_int,
    pub tested: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub transfer: libc::c_int,
    pub length: size_t,
}
pub type LEVEL = libc::c_uint;
pub const FATAL: LEVEL = 3;
pub const ERROR: LEVEL = 2;
pub const WARNING: LEVEL = 1;
pub const DEBUG: LEVEL = 0;
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
pub unsafe extern "C" fn ftp_login(mut C: *mut CONN, mut U: URL) -> BOOLEAN {
    let mut code: libc::c_int = 120 as libc::c_int;
    let mut tmp: [libc::c_char; 128] = [0; 128];
    code = __response(C);
    if okay(code) as u64 == 0 {
        NOTIFY(
            ERROR,
            b"FTP: Server responded: %d\0" as *const u8 as *const libc::c_char,
            code,
        );
        return boolean_false;
    }
    snprintf(
        tmp.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
        b"%s\0" as *const u8 as *const libc::c_char,
        if (url_get_username(U)).is_null() {
            b"anonymous\0" as *const u8 as *const libc::c_char
        } else {
            url_get_username(U) as *const libc::c_char
        },
    );
    code = __request(
        C,
        b"USER %s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        tmp.as_mut_ptr(),
    );
    if code != 331 as libc::c_int {
        if okay(code) as u64 != 0 {
            return boolean_true;
        }
    }
    memset(
        tmp.as_mut_ptr() as *mut libc::c_void,
        '\0' as i32,
        ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
    );
    snprintf(
        tmp.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
        b"%s\0" as *const u8 as *const libc::c_char,
        if (url_get_password(U)).is_null() {
            b"siege@joedog.org\0" as *const u8 as *const libc::c_char
        } else {
            url_get_password(U) as *const libc::c_char
        },
    );
    code = __request(
        C,
        b"PASS %s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        tmp.as_mut_ptr(),
    );
    return __in_range(code, 200 as libc::c_int, 299 as libc::c_int);
}
pub unsafe extern "C" fn ftp_pasv(mut C: *mut CONN) -> BOOLEAN {
    let mut i: libc::c_int = 0;
    let mut code: libc::c_int = 0;
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut addr: [libc::c_uchar; 6] = [0; 6];
    code = __request(
        C,
        b"PASV\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if okay(code) as u64 == 0 {
        return boolean_false;
    }
    ptr = ((*C).chkbuf).as_mut_ptr();
    ptr = ptr.offset(4 as libc::c_int as isize);
    while *ptr as libc::c_int != 0
        && *(*__ctype_b_loc()).offset(*ptr as libc::c_int as isize) as libc::c_int
            & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int == 0
    {
        ptr = ptr.offset(1);
        ptr;
    }
    if *ptr == 0 {
        return boolean_false;
    }
    i = 0 as libc::c_int;
    while i < 6 as libc::c_int {
        addr[i as usize] = 0 as libc::c_int as libc::c_uchar;
        while *(*__ctype_b_loc()).offset(*ptr as libc::c_int as isize) as libc::c_int
            & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
        {
            addr[i
                as usize] = (*ptr as libc::c_int - '0' as i32
                + 10 as libc::c_int * addr[i as usize] as libc::c_int) as libc::c_uchar;
            ptr = ptr.offset(1);
            ptr;
        }
        if *ptr as libc::c_int == ',' as i32 {
            ptr = ptr.offset(1);
            ptr;
        } else if i < 5 as libc::c_int {
            return boolean_false
        }
        i += 1;
        i;
    }
    snprintf(
        ((*C).ftp.host).as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
        b"%d.%d.%d.%d\0" as *const u8 as *const libc::c_char,
        addr[0 as libc::c_int as usize] as libc::c_int,
        addr[1 as libc::c_int as usize] as libc::c_int,
        addr[2 as libc::c_int as usize] as libc::c_int,
        addr[3 as libc::c_int as usize] as libc::c_int,
    );
    (*C)
        .ftp
        .port = ((addr[4 as libc::c_int as usize] as libc::c_int) << 8 as libc::c_int)
        + addr[5 as libc::c_int as usize] as libc::c_int;
    return boolean_true;
}
pub unsafe extern "C" fn ftp_cwd(mut C: *mut CONN, mut U: URL) -> BOOLEAN {
    let mut code: libc::c_int = 0;
    code = __request(
        C,
        b"CWD %s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        url_get_path(U),
    );
    return okay(code);
}
pub unsafe extern "C" fn ftp_ascii(mut C: *mut CONN) -> BOOLEAN {
    (*C)
        .ftp
        .code = __request(
        C,
        b"TYPE A\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    return okay((*C).ftp.code);
}
pub unsafe extern "C" fn ftp_binary(mut C: *mut CONN) -> BOOLEAN {
    (*C)
        .ftp
        .code = __request(
        C,
        b"TYPE I\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    return okay((*C).ftp.code);
}
pub unsafe extern "C" fn ftp_quit(mut C: *mut CONN) -> BOOLEAN {
    (*C)
        .ftp
        .code = __request(
        C,
        b"QUIT\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    return okay((*C).ftp.code);
}
pub unsafe extern "C" fn ftp_size(mut C: *mut CONN, mut U: URL) -> BOOLEAN {
    let mut size: libc::c_int = 0;
    let mut resp: libc::c_int = 0;
    if ftp_binary(C) as libc::c_uint != boolean_true as libc::c_int as libc::c_uint {
        return boolean_false;
    }
    (*C)
        .ftp
        .code = __request(
        C,
        b"SIZE %s%s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        url_get_path(U),
        url_get_file(U),
    );
    if okay((*C).ftp.code) as u64 == 0 {
        return boolean_false
    } else if sscanf(
        ((*C).chkbuf).as_mut_ptr(),
        b"%d %d\0" as *const u8 as *const libc::c_char,
        &mut resp as *mut libc::c_int,
        &mut size as *mut libc::c_int,
    ) == 2 as libc::c_int
    {
        (*C).ftp.size = size as size_t;
        return boolean_true;
    } else {
        return boolean_false
    };
}
pub unsafe extern "C" fn ftp_stor(mut C: *mut CONN, mut U: URL) -> BOOLEAN {
    let mut len: size_t = 0;
    let mut file: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut id: size_t = pthread_self();
    let mut num: libc::c_int = 2 as libc::c_int;
    let mut parts: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    if (id as libc::c_double) < 0.0f64 {
        id = id.wrapping_neg();
    }
    len = (strlen(url_get_file(U))).wrapping_add(17 as libc::c_int as libc::c_ulong);
    parts = split('.' as i32 as libc::c_char, url_get_file(U), &mut num);
    file = xmalloc(len) as *mut libc::c_char;
    memset(file as *mut libc::c_void, '\0' as i32, len);
    snprintf(
        file,
        len,
        b"%s-%zu.%s\0" as *const u8 as *const libc::c_char,
        *parts.offset(0 as libc::c_int as isize),
        id,
        if (*parts.offset(1 as libc::c_int as isize)).is_null() {
            b"\0" as *const u8 as *const libc::c_char
        } else {
            *parts.offset(1 as libc::c_int as isize) as *const libc::c_char
        },
    );
    if my.unique as u64 != 0 {
        (*C)
            .ftp
            .code = __request(
            C,
            b"STOR %s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            file,
        );
    } else {
        (*C)
            .ftp
            .code = __request(
            C,
            b"STOR %s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            url_get_file(U),
        );
    }
    xfree(file as *mut libc::c_void);
    split_free(parts, num);
    return okay((*C).ftp.code);
}
pub unsafe extern "C" fn ftp_retr(mut C: *mut CONN, mut U: URL) -> BOOLEAN {
    (*C)
        .ftp
        .code = __request(
        C,
        b"RETR %s%s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        url_get_path(U),
        url_get_file(U),
    );
    return okay((*C).ftp.code);
}
pub unsafe extern "C" fn ftp_put(mut D: *mut CONN, mut U: URL) -> size_t {
    let mut n: size_t = 0;
    n = socket_write(D, url_get_postdata(U) as *const libc::c_void, url_get_postlen(U))
        as size_t;
    if n != url_get_postlen(U) {
        NOTIFY(
            ERROR,
            b"HTTP: unable to write to socket.\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int) as size_t;
    }
    return url_get_postlen(U);
}
pub unsafe extern "C" fn ftp_get(
    mut D: *mut CONN,
    mut U: URL,
    mut size: size_t,
) -> size_t {
    let mut n: libc::c_int = 0;
    let mut c: libc::c_char = 0;
    let mut bytes: size_t = 0 as libc::c_int as size_t;
    let mut file: *mut libc::c_char = 0 as *mut libc::c_char;
    file = xmalloc(size) as *mut libc::c_char;
    memset(file as *mut libc::c_void, '\0' as i32, size);
    loop {
        n = socket_read(
            D,
            &mut c as *mut libc::c_char as *mut libc::c_void,
            1 as libc::c_int as size_t,
        ) as libc::c_int;
        if n == 0 as libc::c_int {
            break;
        }
        *file.offset(bytes as isize) = c;
        bytes = (bytes as libc::c_ulong).wrapping_add(n as libc::c_ulong) as size_t
            as size_t;
        if !(bytes < size) {
            break;
        }
    }
    if my.get as u64 != 0 {
        write_file(U, file, size);
    }
    xfree(file as *mut libc::c_void);
    return bytes;
}
pub unsafe extern "C" fn ftp_list(
    mut C: *mut CONN,
    mut D: *mut CONN,
    mut U: URL,
) -> BOOLEAN {
    let mut n: libc::c_int = 0;
    let mut c: libc::c_char = 0;
    let mut bytes: libc::c_int = 0;
    (*C)
        .ftp
        .code = __request(
        C,
        b"LIST %s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        if (url_get_file(U)).is_null() { url_get_path(U) } else { url_get_file(U) },
    );
    if (*C).ftp.code == 150 as libc::c_int {
        if (*D).sock < 1 as libc::c_int {
            NOTIFY(
                ERROR,
                b"unable to read from socket: %s:%d\0" as *const u8
                    as *const libc::c_char,
                ((*C).ftp.host).as_mut_ptr(),
                (*C).ftp.port,
            );
            return boolean_false;
        }
        loop {
            n = socket_read(
                D,
                &mut c as *mut libc::c_char as *mut libc::c_void,
                1 as libc::c_int as size_t,
            ) as libc::c_int;
            if n == 0 as libc::c_int {
                break;
            }
            if my.verbose as u64 != 0 {
                printf(b"%c\0" as *const u8 as *const libc::c_char, c as libc::c_int);
            }
            bytes += n;
            if !(boolean_true as libc::c_int != 0) {
                break;
            }
        }
    }
    return boolean_true;
}
unsafe extern "C" fn __request(
    mut C: *mut CONN,
    mut fmt: *mut libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut code: libc::c_int = 0 as libc::c_int;
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut cmd: [libc::c_char; 1032] = [0; 1032];
    let mut len: size_t = 0;
    let mut n: size_t = 0;
    let mut ap: ::std::ffi::VaListImpl;
    memset(
        buf.as_mut_ptr() as *mut libc::c_void,
        '\0' as i32,
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
    );
    memset(
        cmd.as_mut_ptr() as *mut libc::c_void,
        '\0' as i32,
        ::std::mem::size_of::<[libc::c_char; 1032]>() as libc::c_ulong,
    );
    ap = args.clone();
    vsnprintf(
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
        fmt,
        ap.as_va_list(),
    );
    len = snprintf(
        cmd.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1032]>() as libc::c_ulong,
        b"%s\r\n\0" as *const u8 as *const libc::c_char,
        buf.as_mut_ptr(),
    ) as size_t;
    n = socket_write(C, cmd.as_mut_ptr() as *const libc::c_void, len) as size_t;
    if n != len {
        NOTIFY(
            ERROR,
            b"FTP: unable to write to socket.\0" as *const u8 as *const libc::c_char,
        );
        code = 500 as libc::c_int;
    }
    debug(chomp(cmd.as_mut_ptr()));
    if code == 500 as libc::c_int {
        (*C).ftp.code = 500 as libc::c_int;
        return (*C).ftp.code;
    } else {
        (*C).ftp.code = __response(C);
        return (*C).ftp.code;
    };
}
unsafe extern "C" fn __response(mut C: *mut CONN) -> libc::c_int {
    let mut n: libc::c_int = 0;
    let mut c: libc::c_char = 0;
    let mut code: libc::c_int = 120 as libc::c_int;
    let mut cont: BOOLEAN = boolean_true;
    while cont as u64 != 0 {
        let mut x: libc::c_int = 0;
        while boolean_true as libc::c_int != 0 {
            x = 0 as libc::c_int;
            memset(
                ((*C).chkbuf).as_mut_ptr() as *mut libc::c_void,
                '\0' as i32,
                ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
            );
            loop {
                n = socket_read(
                    C,
                    &mut c as *mut libc::c_char as *mut libc::c_void,
                    1 as libc::c_int as size_t,
                ) as libc::c_int;
                if !(n == 1 as libc::c_int) {
                    break;
                }
                echo(b"%c\0" as *const u8 as *const libc::c_char, c as libc::c_int);
                (*C).chkbuf[x as usize] = c;
                if (*C).chkbuf[x as usize] as libc::c_int == '\n' as i32 {
                    break;
                }
                x += 1;
                x;
            }
            if *(*__ctype_b_loc())
                .offset((*C).chkbuf[0 as libc::c_int as usize] as libc::c_int as isize)
                as libc::c_int & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
                != 0
                && (*C).chkbuf[3 as libc::c_int as usize] as libc::c_int != '-' as i32
            {
                break;
            }
        }
        code = __response_code(((*C).chkbuf).as_mut_ptr());
        if (*C).chkbuf[3 as libc::c_int as usize] as libc::c_int == ' ' as i32 {
            cont = boolean_false;
        }
    }
    if code > 499 as libc::c_int && my.quiet as u64 == 0 {
        printf(
            b"%s\n\0" as *const u8 as *const libc::c_char,
            chomp(((*C).chkbuf).as_mut_ptr()),
        );
    }
    return code;
}
unsafe extern "C" fn __response_code(mut buf: *const libc::c_char) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut code: [libc::c_char; 4] = [0; 4];
    memset(
        code.as_mut_ptr() as *mut libc::c_void,
        '\0' as i32,
        ::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong,
    );
    strncpy(code.as_mut_ptr(), buf, 3 as libc::c_int as libc::c_ulong);
    code[3 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
    ret = atoi(code.as_mut_ptr());
    return ret;
}
unsafe extern "C" fn __in_range(
    mut code: libc::c_int,
    mut lower: libc::c_int,
    mut upper: libc::c_int,
) -> BOOLEAN {
    return (code >= lower && code <= upper) as libc::c_int as BOOLEAN;
}
