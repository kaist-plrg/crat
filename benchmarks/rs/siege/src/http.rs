use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
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
    pub type RESPONSE_T;
    pub type internal_state;
    static mut stdout: *mut FILE;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn strtoul(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_ulong;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
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
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strncasecmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn url_get_method_name(this: URL) -> *mut libc::c_char;
    fn url_get_scheme(this: URL) -> SCHEME;
    fn url_get_hostname(this: URL) -> *mut libc::c_char;
    fn url_get_port(this: URL) -> libc::c_int;
    fn url_get_request(this: URL) -> *mut libc::c_char;
    fn url_get_postlen(this: URL) -> size_t;
    fn url_get_postdata(this: URL) -> *mut libc::c_char;
    fn url_get_conttype(this: URL) -> *mut libc::c_char;
    fn auth_get_basic_header(this: AUTH, scheme: SCHEME) -> *mut libc::c_char;
    fn auth_get_ntlm_header(this: AUTH, scheme: SCHEME) -> *mut libc::c_char;
    fn auth_get_digest_header(
        this: AUTH,
        scheme: SCHEME,
        chlg: *mut DCHLG,
        cred: *mut DCRED,
        meth: *const libc::c_char,
        uri: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn auth_get_proxy_required(this: AUTH) -> BOOLEAN;
    fn cookies_add(
        this: COOKIES,
        str: *mut libc::c_char,
        host: *mut libc::c_char,
    ) -> BOOLEAN;
    fn cookies_header(
        this: COOKIES,
        host: *mut libc::c_char,
        newton: *mut libc::c_char,
    ) -> *mut libc::c_char;
    static mut my: CONFIG;
    fn response_set_proxy_authenticate(
        this: RESPONSE,
        line: *mut libc::c_char,
    ) -> BOOLEAN;
    fn response_set_www_authenticate(this: RESPONSE, line: *mut libc::c_char) -> BOOLEAN;
    fn response_get_transfer_encoding(this: RESPONSE) -> HTTP_TE;
    fn response_set_transfer_encoding(
        this: RESPONSE,
        line: *mut libc::c_char,
    ) -> BOOLEAN;
    fn response_get_content_encoding(this: RESPONSE) -> HTTP_CE;
    fn response_set_content_encoding(this: RESPONSE, line: *mut libc::c_char) -> BOOLEAN;
    fn response_set_last_modified(this: RESPONSE, line: *mut libc::c_char) -> BOOLEAN;
    fn response_set_location(this: RESPONSE, line: *mut libc::c_char) -> BOOLEAN;
    fn response_get_keepalive_max(this: RESPONSE) -> libc::c_int;
    fn response_get_keepalive_timeout(this: RESPONSE) -> libc::c_int;
    fn response_set_keepalive(this: RESPONSE, line: *mut libc::c_char) -> BOOLEAN;
    fn response_set_connection(this: RESPONSE, line: *mut libc::c_char) -> BOOLEAN;
    fn response_set_content_length(this: RESPONSE, line: *mut libc::c_char) -> BOOLEAN;
    fn response_set_content_type(this: RESPONSE, line: *mut libc::c_char) -> BOOLEAN;
    fn response_set_code(this: RESPONSE, line: *mut libc::c_char) -> BOOLEAN;
    fn response_destroy(this: RESPONSE) -> RESPONSE;
    fn new_response() -> RESPONSE;
    fn socket_readline(C: *mut CONN, ptr: *mut libc::c_char, maxlen: size_t) -> ssize_t;
    fn socket_read(conn: *mut CONN, buf: *mut libc::c_void, len: size_t) -> ssize_t;
    fn socket_write(conn: *mut CONN, b: *const libc::c_void, n: size_t) -> libc::c_int;
    fn cache_get_header(this: CACHE, type_0: CTYPE, U: URL) -> *mut libc::c_char;
    fn cache_add(this: CACHE, type_0: CTYPE, U: URL, date: *mut libc::c_char);
    fn page_concat(this: PAGE, str: *const libc::c_char, len: libc::c_int);
    fn inflate(strm: z_streamp, flush: libc::c_int) -> libc::c_int;
    fn inflateEnd(strm: z_streamp) -> libc::c_int;
    fn inflateInit2_(
        strm: z_streamp,
        windowBits: libc::c_int,
        version: *const libc::c_char,
        stream_size: libc::c_int,
    ) -> libc::c_int;
    fn echo(fmt: *const libc::c_char, _: ...);
    fn strncasestr(
        str1: *const libc::c_char,
        str2: *const libc::c_char,
        len: size_t,
    ) -> *const libc::c_char;
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    fn xfree(ptr: *mut libc::c_void);
    fn NOTIFY(L: LEVEL, fmt: *const libc::c_char, _: ...);
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __ssize_t = libc::c_long;
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
pub const PTHREAD_MUTEX_FAST_NP: C2RustUnnamed_3 = 0;
pub const PTHREAD_MUTEX_DEFAULT: C2RustUnnamed_3 = 0;
pub const PTHREAD_MUTEX_ERRORCHECK: C2RustUnnamed_3 = 2;
pub const PTHREAD_MUTEX_RECURSIVE: C2RustUnnamed_3 = 1;
pub const PTHREAD_MUTEX_NORMAL: C2RustUnnamed_3 = 0;
pub const PTHREAD_MUTEX_ADAPTIVE_NP: C2RustUnnamed_3 = 3;
pub const PTHREAD_MUTEX_ERRORCHECK_NP: C2RustUnnamed_3 = 2;
pub const PTHREAD_MUTEX_RECURSIVE_NP: C2RustUnnamed_3 = 1;
pub const PTHREAD_MUTEX_TIMED_NP: C2RustUnnamed_3 = 0;
pub type C2RustUnnamed_4 = libc::c_uint;
pub const _ISalnum: C2RustUnnamed_4 = 8;
pub const _ISpunct: C2RustUnnamed_4 = 4;
pub const _IScntrl: C2RustUnnamed_4 = 2;
pub const _ISblank: C2RustUnnamed_4 = 1;
pub const _ISgraph: C2RustUnnamed_4 = 32768;
pub const _ISprint: C2RustUnnamed_4 = 16384;
pub const _ISspace: C2RustUnnamed_4 = 8192;
pub const _ISxdigit: C2RustUnnamed_4 = 4096;
pub const _ISdigit: C2RustUnnamed_4 = 2048;
pub const _ISalpha: C2RustUnnamed_4 = 1024;
pub const _ISlower: C2RustUnnamed_4 = 512;
pub const _ISupper: C2RustUnnamed_4 = 256;
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
pub type CTYPE = libc::c_uint;
pub const C_EXPIRES: CTYPE = 2;
pub const C_LAST: CTYPE = 1;
pub const C_ETAG: CTYPE = 0;
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
    pub content: C2RustUnnamed_9,
    pub connection: C2RustUnnamed_8,
    pub auth: C2RustUnnamed_6,
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
    pub ftp: C2RustUnnamed_5,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub code: libc::c_int,
    pub host: [libc::c_char; 64],
    pub port: libc::c_int,
    pub size: size_t,
    pub pasv: BOOLEAN,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub wchlg: *mut DCHLG,
    pub wcred: *mut DCRED,
    pub www: libc::c_int,
    pub pchlg: *mut DCHLG,
    pub pcred: *mut DCRED,
    pub proxy: libc::c_int,
    pub type_0: C2RustUnnamed_7,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub www: TYPE,
    pub proxy: TYPE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub max: libc::c_int,
    pub timeout: libc::c_int,
    pub reuse: libc::c_int,
    pub status: libc::c_int,
    pub keepalive: libc::c_int,
    pub tested: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub transfer: libc::c_int,
    pub length: size_t,
}
pub type HTTP_TE = libc::c_uint;
pub const TRAILER: HTTP_TE = 4;
pub const CHUNKED: HTTP_TE = 2;
pub const NONE: HTTP_TE = 1;
pub type HTTP_CE = libc::c_uint;
pub const BZIP2: HTTP_CE = 8;
pub const GZIP: HTTP_CE = 4;
pub const DEFLATE: HTTP_CE = 2;
pub const COMPRESS: HTTP_CE = 1;
pub type RESPONSE = *mut RESPONSE_T;
pub type LEVEL = libc::c_uint;
pub const FATAL: LEVEL = 3;
pub const ERROR: LEVEL = 2;
pub const WARNING: LEVEL = 1;
pub const DEBUG: LEVEL = 0;
pub type z_stream = z_stream_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct z_stream_s {
    pub next_in: *mut Bytef,
    pub avail_in: uInt,
    pub total_in: uLong,
    pub next_out: *mut Bytef,
    pub avail_out: uInt,
    pub total_out: uLong,
    pub msg: *mut libc::c_char,
    pub state: *mut internal_state,
    pub zalloc: alloc_func,
    pub zfree: free_func,
    pub opaque: voidpf,
    pub data_type: libc::c_int,
    pub adler: uLong,
    pub reserved: uLong,
}
pub type uLong = libc::c_ulong;
pub type voidpf = *mut libc::c_void;
pub type free_func = Option::<unsafe extern "C" fn(voidpf, voidpf) -> ()>;
pub type alloc_func = Option::<unsafe extern "C" fn(voidpf, uInt, uInt) -> voidpf>;
pub type uInt = libc::c_uint;
pub type Bytef = Byte;
pub type Byte = libc::c_uchar;
pub type z_streamp = *mut z_stream;
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
pub static mut __mutex: pthread_mutex_t = pthread_mutex_t {
    __data: {
        let mut init = __pthread_mutex_s {
            __lock: 0 as libc::c_int,
            __count: 0 as libc::c_int as libc::c_uint,
            __owner: 0 as libc::c_int,
            __nusers: 0 as libc::c_int as libc::c_uint,
            __kind: PTHREAD_MUTEX_TIMED_NP as libc::c_int,
            __spins: 0 as libc::c_int as libc::c_short,
            __elision: 0 as libc::c_int as libc::c_short,
            __list: {
                let mut init = __pthread_internal_list {
                    __prev: 0 as *const __pthread_internal_list
                        as *mut __pthread_internal_list,
                    __next: 0 as *const __pthread_internal_list
                        as *mut __pthread_internal_list,
                };
                init
            },
        };
        init
    },
};
pub static mut __cond: pthread_cond_t = pthread_cond_t {
    __data: {
        let mut init = __pthread_cond_s {
            c2rust_unnamed: C2RustUnnamed_1 {
                __wseq: 0 as libc::c_int as libc::c_ulonglong,
            },
            c2rust_unnamed_0: C2RustUnnamed {
                __g1_start: 0 as libc::c_int as libc::c_ulonglong,
            },
            __g_refs: [
                0 as libc::c_int as libc::c_uint,
                0 as libc::c_int as libc::c_uint,
            ],
            __g_size: [
                0 as libc::c_int as libc::c_uint,
                0 as libc::c_int as libc::c_uint,
            ],
            __g1_orig_size: 0 as libc::c_int as libc::c_uint,
            __wrefs: 0 as libc::c_int as libc::c_uint,
            __g_signals: [
                0 as libc::c_int as libc::c_uint,
                0 as libc::c_int as libc::c_uint,
            ],
        };
        init
    },
};
pub unsafe extern "C" fn https_tunnel_request(
    mut C: *mut CONN,
    mut host: *mut libc::c_char,
    mut port: libc::c_int,
) -> BOOLEAN {
    let mut rlen: size_t = 0;
    let mut n: size_t = 0;
    let mut request: [libc::c_char; 256] = [0; 256];
    if (*C).encrypt as libc::c_uint == boolean_true as libc::c_int as libc::c_uint
        && auth_get_proxy_required(my.auth) as libc::c_uint != 0
    {
        snprintf(
            request.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
            b"CONNECT %s:%d HTTP/1.0\r\nUser-agent: Proxy-User\r\n\r\n\0" as *const u8
                as *const libc::c_char,
            host,
            port,
        );
        rlen = strlen(request.as_mut_ptr());
        echo(b"%s\0" as *const u8 as *const libc::c_char, request.as_mut_ptr());
        (*C).encrypt = boolean_false;
        n = socket_write(C, request.as_mut_ptr() as *const libc::c_void, rlen) as size_t;
        if n != rlen {
            NOTIFY(
                ERROR,
                b"HTTP: unable to write to socket.\0" as *const u8 as *const libc::c_char,
            );
            return boolean_false;
        }
    } else {
        return boolean_false
    }
    return boolean_true;
}
pub unsafe extern "C" fn https_tunnel_response(mut C: *mut CONN) -> libc::c_int {
    let mut x: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut c: libc::c_char = 0;
    let mut line: [libc::c_char; 256] = [0; 256];
    let mut code: libc::c_int = 100 as libc::c_int;
    while boolean_true as libc::c_int != 0 {
        x = 0 as libc::c_int;
        memset(
            &mut line as *mut [libc::c_char; 256] as *mut libc::c_void,
            '\0' as i32,
            ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
        );
        loop {
            n = read(
                (*C).sock,
                &mut c as *mut libc::c_char as *mut libc::c_void,
                1 as libc::c_int as size_t,
            ) as libc::c_int;
            if !(n == 1 as libc::c_int) {
                break;
            }
            line[x as usize] = c;
            echo(b"%c\0" as *const u8 as *const libc::c_char, c as libc::c_int);
            if line[0 as libc::c_int as usize] as libc::c_int == '\n' as i32
                || line[1 as libc::c_int as usize] as libc::c_int == '\n' as i32
            {
                return code;
            }
            if line[x as usize] as libc::c_int == '\n' as i32 {
                break;
            }
            x += 1;
            x;
        }
        line[x as usize] = 0 as libc::c_int as libc::c_char;
        if strncasecmp(
            line.as_mut_ptr(),
            b"http\0" as *const u8 as *const libc::c_char,
            4 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            code = atoi(line.as_mut_ptr().offset(9 as libc::c_int as isize));
        }
    }
    panic!();
}
pub unsafe extern "C" fn http_get(mut C: *mut CONN, mut U: URL) -> BOOLEAN {
    let mut rlen: size_t = 0;
    let mut mlen: size_t = 0;
    let mut protocol: [libc::c_char; 16] = [0; 16];
    let mut keepalive: [libc::c_char; 16] = [0; 16];
    let mut hoststr: [libc::c_char; 512] = [0; 512];
    let mut authwww: [libc::c_char; 512] = [0; 512];
    let mut authpxy: [libc::c_char; 512] = [0; 512];
    let mut accept: [libc::c_char; 14] = *::std::mem::transmute::<
        &[u8; 14],
        &mut [libc::c_char; 14],
    >(b"Accept: */*\r\n\0");
    let mut encoding: [libc::c_char; 512] = [0; 512];
    let mut request: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut portstr: [libc::c_char; 16] = [0; 16];
    let mut fullpath: [libc::c_char; 8192] = [0; 8192];
    let mut cookie: [libc::c_char; 4104] = [0; 4104];
    let mut ifnon: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ifmod: *mut libc::c_char = 0 as *mut libc::c_char;
    memset(
        hoststr.as_mut_ptr() as *mut libc::c_void,
        '\0' as i32,
        ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong,
    );
    memset(
        cookie.as_mut_ptr() as *mut libc::c_void,
        '\0' as i32,
        ::std::mem::size_of::<[libc::c_char; 4104]>() as libc::c_ulong,
    );
    memset(
        portstr.as_mut_ptr() as *mut libc::c_void,
        '\0' as i32,
        ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong,
    );
    ifnon = cache_get_header((*C).cache, C_ETAG, U);
    ifmod = cache_get_header((*C).cache, C_LAST, U);
    if auth_get_proxy_required(my.auth) as u64 != 0 {
        sprintf(
            fullpath.as_mut_ptr(),
            b"%s://%s:%d%s\0" as *const u8 as *const libc::c_char,
            if (*C).encrypt as libc::c_uint
                == boolean_false as libc::c_int as libc::c_uint
            {
                b"http\0" as *const u8 as *const libc::c_char
            } else {
                b"https\0" as *const u8 as *const libc::c_char
            },
            url_get_hostname(U),
            url_get_port(U),
            url_get_request(U),
        );
    } else {
        sprintf(
            fullpath.as_mut_ptr(),
            b"%s\0" as *const u8 as *const libc::c_char,
            url_get_request(U),
        );
    }
    if url_get_port(U) == 80 as libc::c_int
        && (*C).encrypt as libc::c_uint == boolean_false as libc::c_int as libc::c_uint
        || url_get_port(U) == 443 as libc::c_int
            && (*C).encrypt as libc::c_uint
                == boolean_false as libc::c_int as libc::c_uint
    {
        portstr[0 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
    } else {
        snprintf(
            portstr.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong,
            b":%d\0" as *const u8 as *const libc::c_char,
            url_get_port(U),
        );
    }
    if my.protocol == boolean_false as libc::c_int
        || my.get as libc::c_uint == boolean_true as libc::c_int as libc::c_uint
        || my.print as libc::c_uint == boolean_true as libc::c_int as libc::c_uint
    {
        snprintf(
            protocol.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong,
            b"HTTP/1.0\0" as *const u8 as *const libc::c_char,
        );
    } else {
        snprintf(
            protocol.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong,
            b"HTTP/1.1\0" as *const u8 as *const libc::c_char,
        );
    }
    if (*C).connection.keepalive == boolean_true as libc::c_int {
        snprintf(
            keepalive.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong,
            b"keep-alive\0" as *const u8 as *const libc::c_char,
        );
    } else {
        snprintf(
            keepalive.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong,
            b"close\0" as *const u8 as *const libc::c_char,
        );
    }
    cookies_header(my.cookies, url_get_hostname(U), cookie.as_mut_ptr());
    if (*C).auth.www != 0 {
        if (*C).auth.type_0.www as libc::c_uint == DIGEST as libc::c_int as libc::c_uint
        {
            snprintf(
                authwww.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong,
                b"%s\0" as *const u8 as *const libc::c_char,
                auth_get_digest_header(
                    my.auth,
                    HTTP,
                    (*C).auth.wchlg,
                    (*C).auth.wcred,
                    url_get_method_name(U),
                    fullpath.as_mut_ptr(),
                ),
            );
        } else if (*C).auth.type_0.www as libc::c_uint
            == NTLM as libc::c_int as libc::c_uint
        {
            snprintf(
                authwww.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong,
                b"%s\0" as *const u8 as *const libc::c_char,
                auth_get_ntlm_header(my.auth, HTTP),
            );
        } else {
            snprintf(
                authwww.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong,
                b"%s\0" as *const u8 as *const libc::c_char,
                auth_get_basic_header(my.auth, HTTP),
            );
        }
    }
    if (*C).auth.proxy != 0 {
        if (*C).auth.type_0.proxy as libc::c_uint
            == DIGEST as libc::c_int as libc::c_uint
        {
            snprintf(
                authpxy.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong,
                b"%s\0" as *const u8 as *const libc::c_char,
                auth_get_digest_header(
                    my.auth,
                    PROXY,
                    (*C).auth.pchlg,
                    (*C).auth.pcred,
                    url_get_method_name(U),
                    fullpath.as_mut_ptr(),
                ),
            );
        } else {
            snprintf(
                authpxy.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong,
                b"%s\0" as *const u8 as *const libc::c_char,
                auth_get_basic_header(my.auth, PROXY),
            );
        }
    }
    if (strncasestr(
        (my.extra).as_mut_ptr(),
        b"host:\0" as *const u8 as *const libc::c_char,
        ::std::mem::size_of::<[libc::c_char; 2048]>() as libc::c_ulong,
    ))
        .is_null()
    {
        if url_get_scheme(U) as libc::c_uint == HTTP as libc::c_int as libc::c_uint
            && url_get_port(U) != 80 as libc::c_int
            || url_get_scheme(U) as libc::c_uint == HTTPS as libc::c_int as libc::c_uint
                && url_get_port(U) != 443 as libc::c_int
        {
            rlen = snprintf(
                hoststr.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong,
                b"Host: %s:%d\r\n\0" as *const u8 as *const libc::c_char,
                url_get_hostname(U),
                url_get_port(U),
            ) as size_t;
        } else {
            rlen = snprintf(
                hoststr.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong,
                b"Host: %s\r\n\0" as *const u8 as *const libc::c_char,
                url_get_hostname(U),
            ) as size_t;
        }
    }
    mlen = (strlen(url_get_method_name(U)))
        .wrapping_add(strlen(fullpath.as_mut_ptr()))
        .wrapping_add(strlen(protocol.as_mut_ptr()))
        .wrapping_add(strlen(hoststr.as_mut_ptr()))
        .wrapping_add(
            strlen(
                (if (*C).auth.www == boolean_true as libc::c_int {
                    authwww.as_mut_ptr() as *const libc::c_char
                } else {
                    b"\0" as *const u8 as *const libc::c_char
                }),
            ),
        )
        .wrapping_add(
            strlen(
                (if (*C).auth.proxy == boolean_true as libc::c_int {
                    authpxy.as_mut_ptr() as *const libc::c_char
                } else {
                    b"\0" as *const u8 as *const libc::c_char
                }),
            ),
        )
        .wrapping_add(strlen(cookie.as_mut_ptr()))
        .wrapping_add(
            strlen(
                (if !ifmod.is_null() {
                    ifmod as *const libc::c_char
                } else {
                    b"\0" as *const u8 as *const libc::c_char
                }),
            ),
        )
        .wrapping_add(
            strlen(
                (if !ifnon.is_null() {
                    ifnon as *const libc::c_char
                } else {
                    b"\0" as *const u8 as *const libc::c_char
                }),
            ),
        )
        .wrapping_add(
            strlen(
                (if strncasecmp(
                    (my.extra).as_mut_ptr(),
                    b"Accept:\0" as *const u8 as *const libc::c_char,
                    7 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    b"\0" as *const u8 as *const libc::c_char
                } else {
                    accept.as_mut_ptr() as *const libc::c_char
                }),
            ),
        )
        .wrapping_add(::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong)
        .wrapping_add(strlen((my.uagent).as_mut_ptr()))
        .wrapping_add(strlen((my.extra).as_mut_ptr()))
        .wrapping_add(strlen(keepalive.as_mut_ptr()))
        .wrapping_add(128 as libc::c_int as libc::c_ulong);
    request = xmalloc(mlen) as *mut libc::c_char;
    memset(request as *mut libc::c_void, '\0' as i32, mlen);
    memset(
        encoding.as_mut_ptr() as *mut libc::c_void,
        '\0' as i32,
        ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong,
    );
    if my.get as u64 == 0 || my.print as u64 == 0 {
        snprintf(
            encoding.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong,
            b"Accept-Encoding: %s\r\n\0" as *const u8 as *const libc::c_char,
            (my.encoding).as_mut_ptr(),
        );
    }
    rlen = snprintf(
        request,
        mlen,
        b"%s %s %s\r\n%s%s%s%s%s%s%s%sUser-Agent: %s\r\n%sConnection: %s\r\n\r\n\0"
            as *const u8 as *const libc::c_char,
        url_get_method_name(U),
        fullpath.as_mut_ptr(),
        protocol.as_mut_ptr(),
        hoststr.as_mut_ptr(),
        if (*C).auth.www == boolean_true as libc::c_int {
            authwww.as_mut_ptr() as *const libc::c_char
        } else {
            b"\0" as *const u8 as *const libc::c_char
        },
        if (*C).auth.proxy == boolean_true as libc::c_int {
            authpxy.as_mut_ptr() as *const libc::c_char
        } else {
            b"\0" as *const u8 as *const libc::c_char
        },
        if strlen(cookie.as_mut_ptr()) > 8 as libc::c_int as libc::c_ulong {
            cookie.as_mut_ptr() as *const libc::c_char
        } else {
            b"\0" as *const u8 as *const libc::c_char
        },
        if !ifmod.is_null() {
            ifmod as *const libc::c_char
        } else {
            b"\0" as *const u8 as *const libc::c_char
        },
        if !ifnon.is_null() {
            ifnon as *const libc::c_char
        } else {
            b"\0" as *const u8 as *const libc::c_char
        },
        if strncasecmp(
            (my.extra).as_mut_ptr(),
            b"Accept:\0" as *const u8 as *const libc::c_char,
            7 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            b"\0" as *const u8 as *const libc::c_char
        } else {
            accept.as_mut_ptr() as *const libc::c_char
        },
        encoding.as_mut_ptr(),
        (my.uagent).as_mut_ptr(),
        (my.extra).as_mut_ptr(),
        keepalive.as_mut_ptr(),
    ) as size_t;
    if (my.debug as libc::c_uint != 0 || my.get as libc::c_uint != 0
        || my.print as libc::c_uint != 0) && my.quiet as u64 == 0
    {
        printf(b"%s\n\0" as *const u8 as *const libc::c_char, request);
        fflush(stdout);
    }
    if rlen == 0 as libc::c_int as libc::c_ulong || rlen > mlen {
        NOTIFY(
            FATAL,
            b"HTTP %s: request buffer overrun!\0" as *const u8 as *const libc::c_char,
            url_get_method_name(U),
        );
    }
    if socket_write(C, request as *const libc::c_void, rlen) < 0 as libc::c_int {
        xfree(ifmod as *mut libc::c_void);
        xfree(ifnon as *mut libc::c_void);
        return boolean_false;
    }
    xfree(ifmod as *mut libc::c_void);
    xfree(ifnon as *mut libc::c_void);
    xfree(request as *mut libc::c_void);
    return boolean_true;
}
pub unsafe extern "C" fn http_post(mut C: *mut CONN, mut U: URL) -> BOOLEAN {
    let mut rlen: size_t = 0;
    let mut mlen: size_t = 0;
    let mut hoststr: [libc::c_char; 128] = [0; 128];
    let mut authwww: [libc::c_char; 128] = [0; 128];
    let mut authpxy: [libc::c_char; 128] = [0; 128];
    let mut accept: [libc::c_char; 14] = *::std::mem::transmute::<
        &[u8; 14],
        &mut [libc::c_char; 14],
    >(b"Accept: */*\r\n\0");
    let mut encoding: [libc::c_char; 512] = [0; 512];
    let mut request: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut portstr: [libc::c_char; 16] = [0; 16];
    let mut protocol: [libc::c_char; 16] = [0; 16];
    let mut keepalive: [libc::c_char; 16] = [0; 16];
    let mut cookie: [libc::c_char; 4096] = [0; 4096];
    let mut fullpath: [libc::c_char; 8192] = [0; 8192];
    memset(
        hoststr.as_mut_ptr() as *mut libc::c_void,
        '\0' as i32,
        ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
    );
    memset(
        cookie.as_mut_ptr() as *mut libc::c_void,
        '\0' as i32,
        4096 as libc::c_int as libc::c_ulong,
    );
    memset(
        portstr.as_mut_ptr() as *mut libc::c_void,
        '\0' as i32,
        ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong,
    );
    memset(
        protocol.as_mut_ptr() as *mut libc::c_void,
        '\0' as i32,
        ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong,
    );
    memset(
        keepalive.as_mut_ptr() as *mut libc::c_void,
        '\0' as i32,
        ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong,
    );
    if auth_get_proxy_required(my.auth) as u64 != 0 {
        sprintf(
            fullpath.as_mut_ptr(),
            b"%s://%s:%d%s\0" as *const u8 as *const libc::c_char,
            if (*C).encrypt as libc::c_uint
                == boolean_false as libc::c_int as libc::c_uint
            {
                b"http\0" as *const u8 as *const libc::c_char
            } else {
                b"https\0" as *const u8 as *const libc::c_char
            },
            url_get_hostname(U),
            url_get_port(U),
            url_get_request(U),
        );
    } else {
        sprintf(
            fullpath.as_mut_ptr(),
            b"%s\0" as *const u8 as *const libc::c_char,
            url_get_request(U),
        );
    }
    if url_get_port(U) == 80 as libc::c_int
        && (*C).encrypt as libc::c_uint == boolean_false as libc::c_int as libc::c_uint
        || url_get_port(U) == 443 as libc::c_int
            && (*C).encrypt as libc::c_uint
                == boolean_true as libc::c_int as libc::c_uint
    {
        portstr[0 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
    } else {
        snprintf(
            portstr.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong,
            b":%d\0" as *const u8 as *const libc::c_char,
            url_get_port(U),
        );
    }
    if my.protocol == boolean_false as libc::c_int
        || my.get as libc::c_uint == boolean_true as libc::c_int as libc::c_uint
        || my.print as libc::c_uint == boolean_true as libc::c_int as libc::c_uint
    {
        snprintf(
            protocol.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong,
            b"HTTP/1.0\0" as *const u8 as *const libc::c_char,
        );
    } else {
        snprintf(
            protocol.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong,
            b"HTTP/1.1\0" as *const u8 as *const libc::c_char,
        );
    }
    if (*C).connection.keepalive == boolean_true as libc::c_int {
        snprintf(
            keepalive.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong,
            b"keep-alive\0" as *const u8 as *const libc::c_char,
        );
    } else {
        snprintf(
            keepalive.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong,
            b"close\0" as *const u8 as *const libc::c_char,
        );
    }
    cookies_header(my.cookies, url_get_hostname(U), cookie.as_mut_ptr());
    if (*C).auth.www != 0 {
        if (*C).auth.type_0.www as libc::c_uint == DIGEST as libc::c_int as libc::c_uint
        {
            snprintf(
                authwww.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
                b"%s\0" as *const u8 as *const libc::c_char,
                auth_get_digest_header(
                    my.auth,
                    HTTP,
                    (*C).auth.wchlg,
                    (*C).auth.wcred,
                    url_get_method_name(U),
                    fullpath.as_mut_ptr(),
                ),
            );
        } else if (*C).auth.type_0.www as libc::c_uint
            == NTLM as libc::c_int as libc::c_uint
        {
            snprintf(
                authwww.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
                b"%s\0" as *const u8 as *const libc::c_char,
                auth_get_ntlm_header(my.auth, HTTP),
            );
        } else {
            snprintf(
                authwww.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
                b"%s\0" as *const u8 as *const libc::c_char,
                auth_get_basic_header(my.auth, HTTP),
            );
        }
    }
    if (*C).auth.proxy != 0 {
        if (*C).auth.type_0.proxy as libc::c_uint
            == DIGEST as libc::c_int as libc::c_uint
        {
            snprintf(
                authpxy.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
                b"%s\0" as *const u8 as *const libc::c_char,
                auth_get_digest_header(
                    my.auth,
                    HTTP,
                    (*C).auth.pchlg,
                    (*C).auth.pcred,
                    url_get_method_name(U),
                    fullpath.as_mut_ptr(),
                ),
            );
        } else {
            snprintf(
                authpxy.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
                b"%s\0" as *const u8 as *const libc::c_char,
                auth_get_basic_header(my.auth, PROXY),
            );
        }
    }
    if (strncasestr(
        (my.extra).as_mut_ptr(),
        b"host:\0" as *const u8 as *const libc::c_char,
        ::std::mem::size_of::<[libc::c_char; 2048]>() as libc::c_ulong,
    ))
        .is_null()
    {
        if url_get_scheme(U) as libc::c_uint == HTTP as libc::c_int as libc::c_uint
            && url_get_port(U) != 80 as libc::c_int
            || url_get_scheme(U) as libc::c_uint == HTTPS as libc::c_int as libc::c_uint
                && url_get_port(U) != 443 as libc::c_int
        {
            rlen = snprintf(
                hoststr.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
                b"Host: %s:%d\r\n\0" as *const u8 as *const libc::c_char,
                url_get_hostname(U),
                url_get_port(U),
            ) as size_t;
        } else {
            rlen = snprintf(
                hoststr.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
                b"Host: %s\r\n\0" as *const u8 as *const libc::c_char,
                url_get_hostname(U),
            ) as size_t;
        }
    }
    mlen = (strlen(url_get_method_name(U)))
        .wrapping_add(strlen(fullpath.as_mut_ptr()))
        .wrapping_add(strlen(protocol.as_mut_ptr()))
        .wrapping_add(strlen(hoststr.as_mut_ptr()))
        .wrapping_add(
            strlen(
                (if (*C).auth.www == boolean_true as libc::c_int {
                    authwww.as_mut_ptr() as *const libc::c_char
                } else {
                    b"\0" as *const u8 as *const libc::c_char
                }),
            ),
        )
        .wrapping_add(
            strlen(
                (if (*C).auth.proxy == boolean_true as libc::c_int {
                    authpxy.as_mut_ptr() as *const libc::c_char
                } else {
                    b"\0" as *const u8 as *const libc::c_char
                }),
            ),
        )
        .wrapping_add(strlen(cookie.as_mut_ptr()))
        .wrapping_add(
            strlen(
                (if strncasecmp(
                    (my.extra).as_mut_ptr(),
                    b"Accept:\0" as *const u8 as *const libc::c_char,
                    7 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    b"\0" as *const u8 as *const libc::c_char
                } else {
                    accept.as_mut_ptr() as *const libc::c_char
                }),
            ),
        )
        .wrapping_add(::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong)
        .wrapping_add(strlen((my.uagent).as_mut_ptr()))
        .wrapping_add(strlen(url_get_conttype(U)))
        .wrapping_add(strlen((my.extra).as_mut_ptr()))
        .wrapping_add(strlen(keepalive.as_mut_ptr()))
        .wrapping_add(url_get_postlen(U))
        .wrapping_add(128 as libc::c_int as libc::c_ulong);
    request = xmalloc(mlen) as *mut libc::c_char;
    memset(request as *mut libc::c_void, '\0' as i32, mlen);
    memset(
        encoding.as_mut_ptr() as *mut libc::c_void,
        '\0' as i32,
        ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong,
    );
    if my.get as u64 == 0 || my.print as u64 == 0 {
        snprintf(
            encoding.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong,
            b"Accept-Encoding: %s\r\n\0" as *const u8 as *const libc::c_char,
            (my.encoding).as_mut_ptr(),
        );
    }
    rlen = snprintf(
        request,
        mlen,
        b"%s %s %s\r\n%s%s%s%s%s%sUser-Agent: %s\r\n%sConnection: %s\r\nContent-Type: %s\r\nContent-Length: %ld\r\n\r\n\0"
            as *const u8 as *const libc::c_char,
        url_get_method_name(U),
        fullpath.as_mut_ptr(),
        protocol.as_mut_ptr(),
        hoststr.as_mut_ptr(),
        if (*C).auth.www == boolean_true as libc::c_int {
            authwww.as_mut_ptr() as *const libc::c_char
        } else {
            b"\0" as *const u8 as *const libc::c_char
        },
        if (*C).auth.proxy == boolean_true as libc::c_int {
            authpxy.as_mut_ptr() as *const libc::c_char
        } else {
            b"\0" as *const u8 as *const libc::c_char
        },
        if strlen(cookie.as_mut_ptr()) > 8 as libc::c_int as libc::c_ulong {
            cookie.as_mut_ptr() as *const libc::c_char
        } else {
            b"\0" as *const u8 as *const libc::c_char
        },
        if strncasecmp(
            (my.extra).as_mut_ptr(),
            b"Accept:\0" as *const u8 as *const libc::c_char,
            7 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            b"\0" as *const u8 as *const libc::c_char
        } else {
            accept.as_mut_ptr() as *const libc::c_char
        },
        encoding.as_mut_ptr(),
        (my.uagent).as_mut_ptr(),
        (my.extra).as_mut_ptr(),
        keepalive.as_mut_ptr(),
        url_get_conttype(U),
        url_get_postlen(U) as libc::c_long,
    ) as size_t;
    if rlen < mlen {
        memcpy(
            request.offset(rlen as isize) as *mut libc::c_void,
            url_get_postdata(U) as *const libc::c_void,
            url_get_postlen(U),
        );
        *request
            .offset(
                rlen.wrapping_add(url_get_postlen(U)) as isize,
            ) = 0 as libc::c_int as libc::c_char;
    }
    rlen = (rlen as libc::c_ulong).wrapping_add(url_get_postlen(U)) as size_t as size_t;
    if my.get as libc::c_uint != 0 || my.debug as libc::c_uint != 0
        || my.print as libc::c_uint != 0
    {
        printf(b"%s\n\n\0" as *const u8 as *const libc::c_char, request);
    }
    if rlen == 0 as libc::c_int as libc::c_ulong || rlen > mlen {
        NOTIFY(
            FATAL,
            b"HTTP %s: request buffer overrun! Unable to continue...\0" as *const u8
                as *const libc::c_char,
            url_get_method_name(U),
        );
    }
    if socket_write(C, request as *const libc::c_void, rlen) < 0 as libc::c_int {
        return boolean_false;
    }
    xfree(request as *mut libc::c_void);
    return boolean_true;
}
pub unsafe extern "C" fn http_read_headers(mut C: *mut CONN, mut U: URL) -> RESPONSE {
    let mut x: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut c: libc::c_char = 0;
    let mut line: [libc::c_char; 4096] = [0; 4096];
    let mut resp: RESPONSE = new_response();
    while boolean_true as libc::c_int != 0 {
        x = 0 as libc::c_int;
        loop {
            n = socket_read(
                C,
                &mut c as *mut libc::c_char as *mut libc::c_void,
                1 as libc::c_int as size_t,
            ) as libc::c_int;
            if !(n == 1 as libc::c_int) {
                break;
            }
            if x < 4096 as libc::c_int - 1 as libc::c_int {
                line[x as usize] = c;
            } else {
                line[x as usize] = '\n' as i32 as libc::c_char;
            }
            echo(b"%c\0" as *const u8 as *const libc::c_char, c as libc::c_int);
            if x <= 1 as libc::c_int && line[x as usize] as libc::c_int == '\n' as i32 {
                return resp;
            }
            if line[x as usize] as libc::c_int == '\n' as i32 {
                break;
            }
            x += 1;
            x;
        }
        line[x as usize] = '\0' as i32 as libc::c_char;
        if x > 0 as libc::c_int
            && line[(x - 1 as libc::c_int) as usize] as libc::c_int == '\r' as i32
        {
            line[(x - 1 as libc::c_int) as usize] = '\0' as i32 as libc::c_char;
        }
        if strncasecmp(
            line.as_mut_ptr(),
            b"http\0" as *const u8 as *const libc::c_char,
            4 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            response_set_code(resp, line.as_mut_ptr());
        }
        if strncasecmp(
            line.as_mut_ptr(),
            b"content-type\0" as *const u8 as *const libc::c_char,
            strlen(b"content-type\0" as *const u8 as *const libc::c_char),
        ) == 0 as libc::c_int
        {
            response_set_content_type(resp, line.as_mut_ptr());
        }
        if strncasecmp(
            line.as_mut_ptr(),
            b"content-encoding\0" as *const u8 as *const libc::c_char,
            strlen(b"content-encoding\0" as *const u8 as *const libc::c_char),
        ) == 0 as libc::c_int
        {
            response_set_content_encoding(resp, line.as_mut_ptr());
        }
        if strncasecmp(
            line.as_mut_ptr(),
            b"content-length\0" as *const u8 as *const libc::c_char,
            strlen(b"content-length\0" as *const u8 as *const libc::c_char),
        ) == 0 as libc::c_int
        {
            response_set_content_length(resp, line.as_mut_ptr());
            (*C)
                .content
                .length = atoi(line.as_mut_ptr().offset(16 as libc::c_int as isize))
                as size_t;
        }
        if strncasecmp(
            line.as_mut_ptr(),
            b"set-cookie\0" as *const u8 as *const libc::c_char,
            strlen(b"set-cookie\0" as *const u8 as *const libc::c_char),
        ) == 0 as libc::c_int
        {
            if !(my.cookies).is_null() {
                let mut tmp: [libc::c_char; 4096] = [0; 4096];
                memset(
                    tmp.as_mut_ptr() as *mut libc::c_void,
                    '\0' as i32,
                    4096 as libc::c_int as libc::c_ulong,
                );
                strncpy(
                    tmp.as_mut_ptr(),
                    line.as_mut_ptr().offset(12 as libc::c_int as isize),
                    strlen(line.as_mut_ptr()),
                );
                cookies_add(my.cookies, tmp.as_mut_ptr(), url_get_hostname(U));
            }
        }
        if strncasecmp(
            line.as_mut_ptr(),
            b"connection\0" as *const u8 as *const libc::c_char,
            strlen(b"connection\0" as *const u8 as *const libc::c_char),
        ) == 0 as libc::c_int
        {
            response_set_connection(resp, line.as_mut_ptr());
        }
        if strncasecmp(
            line.as_mut_ptr(),
            b"keep-alive: \0" as *const u8 as *const libc::c_char,
            12 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            if response_set_keepalive(resp, line.as_mut_ptr()) as libc::c_uint
                == boolean_true as libc::c_int as libc::c_uint
            {
                (*C).connection.timeout = response_get_keepalive_timeout(resp);
                (*C).connection.max = response_get_keepalive_max(resp);
            }
        }
        if strncasecmp(
            line.as_mut_ptr(),
            b"location\0" as *const u8 as *const libc::c_char,
            strlen(b"location\0" as *const u8 as *const libc::c_char),
        ) == 0 as libc::c_int
        {
            response_set_location(resp, line.as_mut_ptr());
        }
        if strncasecmp(
            line.as_mut_ptr(),
            b"content-location\0" as *const u8 as *const libc::c_char,
            strlen(b"content-location\0" as *const u8 as *const libc::c_char),
        ) == 0 as libc::c_int
        {
            response_set_location(resp, line.as_mut_ptr());
        }
        if strncasecmp(
            line.as_mut_ptr(),
            b"last-modified\0" as *const u8 as *const libc::c_char,
            strlen(b"last-modified\0" as *const u8 as *const libc::c_char),
        ) == 0 as libc::c_int
        {
            response_set_last_modified(resp, line.as_mut_ptr());
            let mut date: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut len: size_t = strlen(line.as_mut_ptr());
            if my.cache as u64 != 0 {
                date = xmalloc(len) as *mut libc::c_char;
                memcpy(
                    date as *mut libc::c_void,
                    line.as_mut_ptr().offset(15 as libc::c_int as isize)
                        as *const libc::c_void,
                    len.wrapping_sub(14 as libc::c_int as libc::c_ulong),
                );
                cache_add((*C).cache, C_LAST, U, date);
                xfree(date as *mut libc::c_void);
            }
        }
        if strncasecmp(
            line.as_mut_ptr(),
            b"etag\0" as *const u8 as *const libc::c_char,
            strlen(b"etag\0" as *const u8 as *const libc::c_char),
        ) == 0 as libc::c_int
        {
            let mut etag: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut len_0: size_t = strlen(line.as_mut_ptr());
            if my.cache as u64 != 0 {
                etag = xmalloc(len_0) as *mut libc::c_char;
                memset(etag as *mut libc::c_void, '\0' as i32, len_0);
                memcpy(
                    etag as *mut libc::c_void,
                    line.as_mut_ptr().offset(6 as libc::c_int as isize)
                        as *const libc::c_void,
                    len_0.wrapping_sub(5 as libc::c_int as libc::c_ulong),
                );
                cache_add((*C).cache, C_ETAG, U, etag);
                xfree(etag as *mut libc::c_void);
            }
        }
        if strncasecmp(
            line.as_mut_ptr(),
            b"www-authenticate\0" as *const u8 as *const libc::c_char,
            strlen(b"www-authenticate\0" as *const u8 as *const libc::c_char),
        ) == 0 as libc::c_int
        {
            response_set_www_authenticate(resp, line.as_mut_ptr());
        }
        if strncasecmp(
            line.as_mut_ptr(),
            b"proxy-authenticate\0" as *const u8 as *const libc::c_char,
            strlen(b"proxy-authenticate\0" as *const u8 as *const libc::c_char),
        ) == 0 as libc::c_int
        {
            response_set_proxy_authenticate(resp, line.as_mut_ptr());
        }
        if strncasecmp(
            line.as_mut_ptr(),
            b"transfer-encoding\0" as *const u8 as *const libc::c_char,
            strlen(b"transfer-encoding\0" as *const u8 as *const libc::c_char),
        ) == 0 as libc::c_int
        {
            response_set_transfer_encoding(resp, line.as_mut_ptr());
        }
        if strncasecmp(
            line.as_mut_ptr(),
            b"expires\0" as *const u8 as *const libc::c_char,
            strlen(b"expires\0" as *const u8 as *const libc::c_char),
        ) == 0 as libc::c_int
        {
            let mut expires: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut len_1: size_t = strlen(line.as_mut_ptr());
            if my.cache as u64 != 0 {
                expires = xmalloc(len_1) as *mut libc::c_char;
                memset(expires as *mut libc::c_void, '\0' as i32, len_1);
                memcpy(
                    expires as *mut libc::c_void,
                    line.as_mut_ptr().offset(9 as libc::c_int as isize)
                        as *const libc::c_void,
                    len_1.wrapping_sub(8 as libc::c_int as libc::c_ulong),
                );
                cache_add((*C).cache, C_EXPIRES, U, expires);
                xfree(expires as *mut libc::c_void);
            }
        }
        strncasecmp(
            line.as_mut_ptr(),
            b"cache-control: \0" as *const u8 as *const libc::c_char,
            15 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int;
        if n <= 0 as libc::c_int {
            echo(
                b"read error: %s:%d\0" as *const u8 as *const libc::c_char,
                b"http.c\0" as *const u8 as *const libc::c_char,
                543 as libc::c_int,
            );
            resp = response_destroy(resp);
            return resp;
        }
    }
    return resp;
}
pub unsafe extern "C" fn http_chunk_size(mut C: *mut CONN) -> libc::c_int {
    let mut n: libc::c_int = 0;
    let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut length: size_t = 0;
    memset(
        ((*C).chkbuf).as_mut_ptr() as *mut libc::c_void,
        '\0' as i32,
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
    );
    n = socket_readline(
        C,
        ((*C).chkbuf).as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
    ) as libc::c_int;
    if n < 1 as libc::c_int {
        NOTIFY(
            WARNING,
            b"HTTP: unable to determine chunk size\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (*C).chkbuf[0 as libc::c_int as usize] as libc::c_int == '\n' as i32
        || strlen(((*C).chkbuf).as_mut_ptr()) == 0 as libc::c_int as libc::c_ulong
        || (*C).chkbuf[0 as libc::c_int as usize] as libc::c_int == '\r' as i32
    {
        return -(1 as libc::c_int);
    }
    *__errno_location() = 0 as libc::c_int;
    if *(*__ctype_b_loc())
        .offset(*((*C).chkbuf).as_mut_ptr() as libc::c_uint as libc::c_int as isize)
        as libc::c_int & _ISxdigit as libc::c_int as libc::c_ushort as libc::c_int == 0
    {
        return -(1 as libc::c_int);
    }
    length = strtoul(((*C).chkbuf).as_mut_ptr(), &mut end, 16 as libc::c_int);
    if *__errno_location() == 34 as libc::c_int || end == ((*C).chkbuf).as_mut_ptr() {
        NOTIFY(
            WARNING,
            b"HTTP: invalid chunk line %s\n\0" as *const u8 as *const libc::c_char,
            ((*C).chkbuf).as_mut_ptr(),
        );
        return 0 as libc::c_int;
    } else {
        return length as libc::c_int
    };
}
pub unsafe extern "C" fn http_read(mut C: *mut CONN, mut resp: RESPONSE) -> ssize_t {
    let mut n: libc::c_int = 0 as libc::c_int;
    let mut chunk: libc::c_int = 0 as libc::c_int;
    let mut bytes: size_t = 0 as libc::c_int as size_t;
    let mut length: size_t = 0 as libc::c_int as size_t;
    let mut dest: [libc::c_char; 393216] = [0; 393216];
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut size: size_t = 0x10000 as libc::c_int as size_t;
    if C.is_null() {
        NOTIFY(
            FATAL,
            b"Connection is NULL! Unable to proceed\0" as *const u8
                as *const libc::c_char,
        );
        return 0 as libc::c_int as ssize_t;
    }
    if (*C).content.length == 0 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int as ssize_t
    } else if (*C).content.length == !(0 as libc::c_long) as size_t {
        (*C).content.length = 0 as libc::c_int as size_t;
    }
    memset(
        dest.as_mut_ptr() as *mut libc::c_void,
        '\0' as i32,
        ::std::mem::size_of::<[libc::c_char; 393216]>() as libc::c_ulong,
    );
    if (*C).content.length > 0 as libc::c_int as libc::c_ulong {
        length = (*C).content.length;
        ptr = xmalloc(length.wrapping_add(1 as libc::c_int as libc::c_ulong))
            as *mut libc::c_char;
        memset(
            ptr as *mut libc::c_void,
            '\0' as i32,
            length.wrapping_add(1 as libc::c_int as libc::c_ulong),
        );
        loop {
            n = socket_read(C, ptr as *mut libc::c_void, length) as libc::c_int;
            if n == 0 as libc::c_int {
                break;
            }
            bytes = (bytes as libc::c_ulong).wrapping_add(n as libc::c_ulong) as size_t
                as size_t;
            if !(bytes < length) {
                break;
            }
        }
    } else if my.chunked as libc::c_uint != 0
        && response_get_transfer_encoding(resp) as libc::c_uint
            == CHUNKED as libc::c_int as libc::c_uint
    {
        let mut r: libc::c_int = 0 as libc::c_int;
        bytes = 0 as libc::c_int as size_t;
        let mut done: BOOLEAN = boolean_false;
        ptr = xmalloc(size) as *mut libc::c_char;
        memset(ptr as *mut libc::c_void, '\0' as i32, size);
        loop {
            chunk = http_chunk_size(C);
            if chunk == 0 as libc::c_int {
                socket_readline(
                    C,
                    ((*C).chkbuf).as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
                );
                break;
            } else {
                if chunk < 0 as libc::c_int {
                    chunk = 0 as libc::c_int;
                } else {
                    while n < chunk {
                        let mut remaining_in_chunk: libc::c_int = chunk - n;
                        let mut space_in_buf: libc::c_int = size.wrapping_sub(bytes)
                            as libc::c_int;
                        let mut to_read: libc::c_int = if remaining_in_chunk
                            < space_in_buf
                        {
                            remaining_in_chunk
                        } else {
                            space_in_buf
                        };
                        r = socket_read(
                            C,
                            &mut *ptr.offset(bytes as isize) as *mut libc::c_char
                                as *mut libc::c_void,
                            to_read as size_t,
                        ) as libc::c_int;
                        bytes = (bytes as libc::c_ulong).wrapping_add(r as libc::c_ulong)
                            as size_t as size_t;
                        if r <= 0 as libc::c_int {
                            done = boolean_true;
                            break;
                        } else {
                            n += r;
                            if bytes >= size {
                                tmp = realloc(
                                    ptr as *mut libc::c_void,
                                    size.wrapping_mul(2 as libc::c_int as libc::c_ulong),
                                ) as *mut libc::c_char;
                                if tmp.is_null() {
                                    free(ptr as *mut libc::c_void);
                                    return -(1 as libc::c_int) as ssize_t;
                                }
                                ptr = tmp;
                                size = (size as libc::c_ulong)
                                    .wrapping_mul(2 as libc::c_int as libc::c_ulong) as size_t
                                    as size_t;
                            }
                        }
                    }
                    n = 0 as libc::c_int;
                }
                if !(done as u64 == 0) {
                    break;
                }
            }
        }
        *ptr.offset(bytes as isize) = '\0' as i32 as libc::c_char;
    } else {
        ptr = xmalloc(size) as *mut libc::c_char;
        memset(ptr as *mut libc::c_void, '\0' as i32, size);
        loop {
            n = socket_read(C, ptr as *mut libc::c_void, size) as libc::c_int;
            bytes = (bytes as libc::c_ulong).wrapping_add(n as libc::c_ulong) as size_t
                as size_t;
            if n <= 0 as libc::c_int {
                break;
            }
            if !(boolean_true as libc::c_int != 0) {
                break;
            }
        }
    }
    if response_get_content_encoding(resp) as libc::c_uint
        == GZIP as libc::c_int as libc::c_uint
    {
        __gzip_inflate(
            15 as libc::c_int + 32 as libc::c_int,
            ptr,
            bytes as libc::c_int,
            dest.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 393216]>() as libc::c_ulong
                as libc::c_int,
        );
    }
    if response_get_content_encoding(resp) as libc::c_uint
        == DEFLATE as libc::c_int as libc::c_uint
    {
        __gzip_inflate(
            -(15 as libc::c_int),
            ptr,
            bytes as libc::c_int,
            dest.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 393216]>() as libc::c_ulong
                as libc::c_int,
        );
    }
    if strlen(dest.as_mut_ptr()) > 0 as libc::c_int as libc::c_ulong {
        page_concat(
            (*C).page,
            dest.as_mut_ptr(),
            strlen(dest.as_mut_ptr()) as libc::c_int,
        );
    } else {
        page_concat((*C).page, ptr, strlen(ptr) as libc::c_int);
    }
    xfree(ptr as *mut libc::c_void);
    echo(b"\n\0" as *const u8 as *const libc::c_char);
    return bytes as ssize_t;
}
unsafe extern "C" fn __gzip_inflate(
    mut window: libc::c_int,
    mut src: *const libc::c_char,
    mut srcLen: libc::c_int,
    mut dst: *const libc::c_char,
    mut dstLen: libc::c_int,
) -> libc::c_int {
    let mut err: libc::c_int = -(1 as libc::c_int);
    let mut strm: z_stream = z_stream {
        next_in: 0 as *mut Bytef,
        avail_in: 0,
        total_in: 0,
        next_out: 0 as *mut Bytef,
        avail_out: 0,
        total_out: 0,
        msg: 0 as *mut libc::c_char,
        state: 0 as *mut internal_state,
        zalloc: None,
        zfree: None,
        opaque: 0 as *mut libc::c_void,
        data_type: 0,
        adler: 0,
        reserved: 0,
    };
    let mut ret: libc::c_int = -(1 as libc::c_int);
    strm.zalloc = None;
    strm.zfree = None;
    strm.opaque = 0 as voidpf;
    strm.avail_in = srcLen as uInt;
    strm.avail_out = dstLen as uInt;
    strm.next_in = src as *mut Bytef;
    strm.next_out = dst as *mut Bytef;
    err = inflateInit2_(
        &mut strm,
        window,
        b"1.2.11\0" as *const u8 as *const libc::c_char,
        ::std::mem::size_of::<z_stream>() as libc::c_ulong as libc::c_int,
    );
    if err == 0 as libc::c_int {
        err = inflate(&mut strm, 4 as libc::c_int);
        if err == 1 as libc::c_int {
            ret = strm.total_out as libc::c_int;
        } else {
            inflateEnd(&mut strm);
            return err;
        }
    } else {
        inflateEnd(&mut strm);
        return err;
    }
    inflateEnd(&mut strm);
    return ret;
}
