use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type __jmp_buf_tag;
    pub type URL_T;
    pub type AUTH_T;
    pub type DIGEST_CRED;
    pub type DIGEST_CHLG;
    pub type ARRAY_T;
    pub type HASH_T;
    pub type COOKIES_T;
    pub type x509_st;
    pub type ssl_st;
    pub type ssl_ctx_st;
    pub type ssl_method_st;
    pub type PAGE_T;
    pub type DATE_T;
    pub type CACHE_T;
    pub type RESPONSE_T;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn time(__timer: *mut time_t) -> time_t;
    fn strftime(
        __s: *mut libc::c_char,
        __maxsize: size_t,
        __format: *const libc::c_char,
        __tp: *const tm,
    ) -> size_t;
    fn localtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm;
    fn pthread_setcancelstate(
        __state: libc::c_int,
        __oldstate: *mut libc::c_int,
    ) -> libc::c_int;
    fn pthread_setcanceltype(
        __type: libc::c_int,
        __oldtype: *mut libc::c_int,
    ) -> libc::c_int;
    fn pthread_testcancel();
    fn __pthread_register_cancel(__buf: *mut __pthread_unwind_buf_t);
    fn __pthread_unregister_cancel(__buf: *mut __pthread_unwind_buf_t);
    fn __pthread_unwind_next(__buf: *mut __pthread_unwind_buf_t) -> !;
    fn __sigsetjmp(__env: *mut __jmp_buf_tag, __savemask: libc::c_int) -> libc::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn times(__buffer: *mut tms) -> clock_t;
    fn __errno_location() -> *mut libc::c_int;
    fn new_url(str: *mut libc::c_char) -> URL;
    fn url_destroy(this: URL) -> URL;
    fn url_set_ID(this: URL, id: libc::c_int);
    fn url_set_hostname(this: URL, hostname: *mut libc::c_char);
    fn url_set_redirect(this: URL, redir: BOOLEAN);
    fn url_set_conttype(this: URL, type_0: *mut libc::c_char);
    fn url_set_postdata(this: URL, postdata: *mut libc::c_char, postlen: size_t);
    fn url_set_method(this: URL, method: METHOD);
    fn url_get_ID(this: URL) -> libc::c_int;
    fn url_get_method(this: URL) -> METHOD;
    fn url_get_method_name(this: URL) -> *mut libc::c_char;
    fn url_is_redirect(this: URL) -> BOOLEAN;
    fn url_get_absolute(this: URL) -> *mut libc::c_char;
    fn url_get_scheme(this: URL) -> SCHEME;
    fn url_get_username(this: URL) -> *mut libc::c_char;
    fn url_get_password(this: URL) -> *mut libc::c_char;
    fn url_get_hostname(this: URL) -> *mut libc::c_char;
    fn url_get_port(this: URL) -> libc::c_int;
    fn url_get_request(this: URL) -> *mut libc::c_char;
    fn url_get_display(this: URL) -> *mut libc::c_char;
    fn url_get_postlen(this: URL) -> size_t;
    fn url_get_postdata(this: URL) -> *mut libc::c_char;
    fn url_get_conttype(this: URL) -> *mut libc::c_char;
    fn url_set_username(this: URL, username: *mut libc::c_char);
    fn url_set_password(this: URL, password: *mut libc::c_char);
    fn url_normalize(req: URL, location: *mut libc::c_char) -> URL;
    fn auth_set_basic_header(
        this: AUTH,
        scheme: SCHEME,
        realm: *mut libc::c_char,
    ) -> BOOLEAN;
    fn auth_set_ntlm_header(
        this: AUTH,
        scheme: SCHEME,
        header: *mut libc::c_char,
        realm: *mut libc::c_char,
    ) -> BOOLEAN;
    fn auth_set_digest_header(
        this: AUTH,
        ch: *mut *mut DCHLG,
        cr: *mut *mut DCRED,
        rand: *mut libc::c_uint,
        realm: *mut libc::c_char,
        str: *mut libc::c_char,
    ) -> BOOLEAN;
    fn auth_get_proxy_required(this: AUTH) -> BOOLEAN;
    fn auth_get_proxy_host(this: AUTH) -> *mut libc::c_char;
    fn auth_get_proxy_port(this: AUTH) -> libc::c_int;
    fn auth_get_ftp_username(this: AUTH, realm: *mut libc::c_char) -> *mut libc::c_char;
    fn auth_get_ftp_password(this: AUTH, realm: *mut libc::c_char) -> *mut libc::c_char;
    fn new_array() -> ARRAY;
    fn array_destroy(this: ARRAY) -> ARRAY;
    fn array_get(this: ARRAY, index: libc::c_int) -> *mut libc::c_void;
    fn array_remove(this: ARRAY, index: libc::c_int) -> *mut libc::c_void;
    fn array_pop(this: ARRAY) -> *mut libc::c_void;
    fn array_next(this: ARRAY) -> *mut libc::c_void;
    fn array_length(this: ARRAY) -> size_t;
    fn hash_get(this: HASH, key: *mut libc::c_char) -> *mut libc::c_void;
    fn hash_get_keys(this: HASH) -> *mut *mut libc::c_char;
    fn hash_get_entries(this: HASH) -> libc::c_int;
    fn cookies_add(
        this: COOKIES,
        str: *mut libc::c_char,
        host: *mut libc::c_char,
    ) -> BOOLEAN;
    fn cookies_delete_all(this: COOKIES) -> BOOLEAN;
    static mut my: CONFIG;
    fn socket_close(C: *mut CONN);
    fn new_socket(
        conn: *mut CONN,
        hostname: *const libc::c_char,
        port: libc::c_int,
    ) -> libc::c_int;
    fn is_cached(this: CACHE, U: URL) -> BOOLEAN;
    fn cache_destroy(this: CACHE) -> CACHE;
    fn new_cache() -> CACHE;
    fn date_stamp(this: DATE) -> *mut libc::c_char;
    fn date_destroy(this: DATE) -> DATE;
    fn new_date(_: *mut libc::c_char) -> DATE;
    fn page_value(this: PAGE) -> *mut libc::c_char;
    fn page_clear(this: PAGE);
    fn page_destroy(this: PAGE) -> PAGE;
    fn new_page(_: *mut libc::c_char) -> PAGE;
    fn SSL_initialize(C: *mut CONN, servername: *const libc::c_char) -> BOOLEAN;
    fn ftp_login(C: *mut CONN, U: URL) -> BOOLEAN;
    fn ftp_size(C: *mut CONN, U: URL) -> BOOLEAN;
    fn ftp_pasv(C: *mut CONN) -> BOOLEAN;
    fn ftp_stor(C: *mut CONN, U: URL) -> BOOLEAN;
    fn ftp_retr(C: *mut CONN, U: URL) -> BOOLEAN;
    fn ftp_get(D: *mut CONN, U: URL, size: size_t) -> size_t;
    fn ftp_put(D: *mut CONN, U: URL) -> size_t;
    fn ftp_quit(C: *mut CONN) -> BOOLEAN;
    fn new_response() -> RESPONSE;
    fn response_destroy(this: RESPONSE) -> RESPONSE;
    fn response_set_code(this: RESPONSE, line: *mut libc::c_char) -> BOOLEAN;
    fn response_get_code(this: RESPONSE) -> libc::c_int;
    fn response_get_protocol(this: RESPONSE) -> *mut libc::c_char;
    fn response_set_from_cache(this: RESPONSE, cached: BOOLEAN);
    fn response_get_from_cache(this: RESPONSE) -> BOOLEAN;
    fn response_success(this: RESPONSE) -> libc::c_int;
    fn response_failure(this: RESPONSE) -> libc::c_int;
    fn response_get_content_type(this: RESPONSE) -> *mut libc::c_char;
    fn response_get_location(this: RESPONSE) -> *mut libc::c_char;
    fn response_get_proxy_auth_realm(this: RESPONSE) -> *mut libc::c_char;
    fn http_get(C: *mut CONN, U: URL) -> BOOLEAN;
    fn http_post(C: *mut CONN, U: URL) -> BOOLEAN;
    fn http_read_headers(C: *mut CONN, U: URL) -> RESPONSE;
    fn https_tunnel_request(
        C: *mut CONN,
        host: *mut libc::c_char,
        port: libc::c_int,
    ) -> BOOLEAN;
    fn response_get_proxy_auth_type(this: RESPONSE) -> TYPE;
    fn response_get_proxy_auth_challenge(this: RESPONSE) -> *mut libc::c_char;
    fn http_read(C: *mut CONN, R: RESPONSE) -> ssize_t;
    fn https_tunnel_response(C: *mut CONN) -> libc::c_int;
    fn response_get_www_auth_type(this: RESPONSE) -> TYPE;
    fn response_get_www_auth_challenge(this: RESPONSE) -> *mut libc::c_char;
    fn response_get_www_auth_realm(this: RESPONSE) -> *mut libc::c_char;
    fn elapsed_time(time_0: clock_t) -> libc::c_float;
    fn pthread_sleep_np(seconds: libc::c_uint);
    fn pthread_usleep_np(usec: libc::c_ulong);
    fn pthread_rand_np(ctx: *mut libc::c_uint) -> libc::c_int;
    fn urandom() -> libc::c_int;
    fn strmatch(str1: *mut libc::c_char, str2: *mut libc::c_char) -> BOOLEAN;
    fn echo(fmt: *const libc::c_char, _: ...);
    fn debug(fmt: *const libc::c_char, _: ...);
    fn stristr(
        haystack: *const libc::c_char,
        needle: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn html_parser(array: ARRAY, base: URL, page: *mut libc::c_char) -> BOOLEAN;
    fn empty(s: *const libc::c_char) -> BOOLEAN;
    fn xstrdup(str: *const libc::c_char) -> *mut libc::c_char;
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    fn xcalloc(_: size_t, _: size_t) -> *mut libc::c_void;
    fn xfree(ptr: *mut libc::c_void);
    fn DISPLAY(color: libc::c_int, fmt: *const libc::c_char, _: ...);
    fn NOTIFY(L: LEVEL, fmt: *const libc::c_char, _: ...);
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __clock_t = libc::c_long;
pub type __time_t = libc::c_long;
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
pub type clock_t = __clock_t;
pub type time_t = __time_t;
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
pub type __jmp_buf = [libc::c_long; 8];
pub type C2RustUnnamed_3 = libc::c_uint;
pub const PTHREAD_CANCEL_DISABLE: C2RustUnnamed_3 = 1;
pub const PTHREAD_CANCEL_ENABLE: C2RustUnnamed_3 = 0;
pub type C2RustUnnamed_4 = libc::c_uint;
pub const PTHREAD_CANCEL_ASYNCHRONOUS: C2RustUnnamed_4 = 1;
pub const PTHREAD_CANCEL_DEFERRED: C2RustUnnamed_4 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_unwind_buf_t {
    pub __cancel_jmp_buf: [C2RustUnnamed_5; 1],
    pub __pad: [*mut libc::c_void; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub __cancel_jmp_buf: __jmp_buf,
    pub __mask_was_saved: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tms {
    pub tms_utime: clock_t,
    pub tms_stime: clock_t,
    pub tms_cutime: clock_t,
    pub tms_cstime: clock_t,
}
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
pub type HASH = *mut HASH_T;
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
pub type DATE = *mut DATE_T;
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
    pub content: C2RustUnnamed_10,
    pub connection: C2RustUnnamed_9,
    pub auth: C2RustUnnamed_7,
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
    pub ftp: C2RustUnnamed_6,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub code: libc::c_int,
    pub host: [libc::c_char; 64],
    pub port: libc::c_int,
    pub size: size_t,
    pub pasv: BOOLEAN,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub wchlg: *mut DCHLG,
    pub wcred: *mut DCRED,
    pub www: libc::c_int,
    pub pchlg: *mut DCHLG,
    pub pcred: *mut DCRED,
    pub proxy: libc::c_int,
    pub type_0: C2RustUnnamed_8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub www: TYPE,
    pub proxy: TYPE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub max: libc::c_int,
    pub timeout: libc::c_int,
    pub reuse: libc::c_int,
    pub status: libc::c_int,
    pub keepalive: libc::c_int,
    pub tested: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub transfer: libc::c_int,
    pub length: size_t,
}
pub type C2RustUnnamed_11 = libc::c_uint;
pub const TRAILER: C2RustUnnamed_11 = 4;
pub const CHUNKED: C2RustUnnamed_11 = 2;
pub const NONE: C2RustUnnamed_11 = 1;
pub type RESPONSE = *mut RESPONSE_T;
pub type LEVEL = libc::c_uint;
pub const FATAL: LEVEL = 3;
pub const ERROR: LEVEL = 2;
pub const WARNING: LEVEL = 1;
pub const DEBUG: LEVEL = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BROWSER_T {
    pub id: libc::c_int,
    pub tid: size_t,
    pub urls: ARRAY,
    pub parts: ARRAY,
    pub cookies: HASH,
    pub conn: *mut CONN,
    pub type_0: libc::c_int,
    pub state: libc::c_int,
    pub total: libc::c_float,
    pub available: libc::c_float,
    pub lowest: libc::c_float,
    pub highest: libc::c_float,
    pub elapsed: libc::c_float,
    pub time: libc::c_float,
    pub himark: libc::c_float,
    pub lomark: libc::c_float,
    pub start: clock_t,
    pub stop: clock_t,
    pub t_start: tms,
    pub t_stop: tms,
    pub auth: C2RustUnnamed_12,
    pub code: libc::c_uint,
    pub count: libc::c_uint,
    pub okay: libc::c_uint,
    pub fail: libc::c_uint,
    pub hits: libc::c_ulong,
    pub bytes: libc::c_ulonglong,
    pub rseed: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_12 {
    pub wchlg: *mut DCHLG,
    pub wcred: *mut DCRED,
    pub www: libc::c_int,
    pub pchlg: *mut DCHLG,
    pub pcred: *mut DCRED,
    pub proxy: libc::c_int,
    pub bids: C2RustUnnamed_14,
    pub type_0: C2RustUnnamed_13,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_13 {
    pub www: TYPE,
    pub proxy: TYPE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_14 {
    pub www: libc::c_int,
    pub proxy: libc::c_int,
}
pub type BROWSER = *mut BROWSER_T;
pub static mut __himark: libc::c_float = 0 as libc::c_int as libc::c_float;
pub static mut __lomark: libc::c_float = -(1 as libc::c_int) as libc::c_float;
pub static mut BROWSERSIZE: size_t = ::std::mem::size_of::<BROWSER_T>() as libc::c_ulong;
pub unsafe extern "C" fn new_browser(mut id: libc::c_int) -> BROWSER {
    let mut this: BROWSER = 0 as *mut BROWSER_T;
    this = calloc(BROWSERSIZE, 1 as libc::c_int as libc::c_ulong) as BROWSER;
    (*this).id = id;
    (*this).total = 0.0f64 as libc::c_float;
    (*this).available = 0.0f64 as libc::c_float;
    (*this).count = 0.0f64 as libc::c_uint;
    (*this).okay = 0 as libc::c_int as libc::c_uint;
    (*this).fail = 0 as libc::c_int as libc::c_uint;
    (*this).lowest = -(1 as libc::c_int) as libc::c_float;
    (*this).highest = 0.0f64 as libc::c_float;
    (*this).elapsed = 0.0f64 as libc::c_float;
    (*this).bytes = 0.0f64 as libc::c_ulonglong;
    (*this).urls = 0 as ARRAY;
    (*this).parts = new_array();
    (*this).rseed = urandom() as libc::c_uint;
    return this;
}
pub unsafe extern "C" fn browser_destroy(mut this: BROWSER) -> BROWSER {
    if !this.is_null() {
        if !((*this).parts).is_null() {
            let mut u: URL = 0 as *mut URL_T;
            loop {
                u = array_pop((*this).parts) as URL;
                if u.is_null() {
                    break;
                }
                u = url_destroy(u);
            }
            (*this).parts = array_destroy((*this).parts);
        }
        xfree(this as *mut libc::c_void);
    }
    this = 0 as BROWSER;
    return this;
}
pub unsafe extern "C" fn browser_get_hits(mut this: BROWSER) -> libc::c_ulong {
    return (*this).hits;
}
pub unsafe extern "C" fn browser_get_bytes(mut this: BROWSER) -> libc::c_ulonglong {
    return (*this).bytes;
}
pub unsafe extern "C" fn browser_get_time(mut this: BROWSER) -> libc::c_float {
    return (*this).time;
}
pub unsafe extern "C" fn browser_get_code(mut this: BROWSER) -> libc::c_uint {
    return (*this).code;
}
pub unsafe extern "C" fn browser_get_okay(mut this: BROWSER) -> libc::c_uint {
    return (*this).okay;
}
pub unsafe extern "C" fn browser_get_fail(mut this: BROWSER) -> libc::c_uint {
    return (*this).fail;
}
pub unsafe extern "C" fn browser_get_himark(mut this: BROWSER) -> libc::c_float {
    return (*this).himark;
}
pub unsafe extern "C" fn browser_get_lomark(mut this: BROWSER) -> libc::c_float {
    return (*this).lomark;
}
pub unsafe extern "C" fn start(mut this: BROWSER) -> *mut libc::c_void {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut max_y: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    (*this).conn = 0 as *mut CONN;
    (*this)
        .conn = xcalloc(
        ::std::mem::size_of::<CONN>() as libc::c_ulong,
        1 as libc::c_int as size_t,
    ) as *mut CONN;
    (*(*this).conn).sock = -(1 as libc::c_int);
    (*(*this).conn)
        .page = new_page(b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    (*(*this).conn).cache = new_cache();
    let mut __cancel_buf: __pthread_unwind_buf_t = __pthread_unwind_buf_t {
        __cancel_jmp_buf: [C2RustUnnamed_5 {
            __cancel_jmp_buf: [0; 8],
            __mask_was_saved: 0,
        }; 1],
        __pad: [0 as *mut libc::c_void; 4],
    };
    let mut __cancel_routine: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()> = ::std::mem::transmute::<
        *mut libc::c_void,
        Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    >(
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> ()>,
            *mut libc::c_void,
        >(
            Some(
                ::std::mem::transmute::<
                    unsafe extern "C" fn() -> (),
                    unsafe extern "C" fn() -> (),
                >(__signal_cleanup),
            ),
        ),
    );
    let mut __cancel_arg: *mut libc::c_void = (*this).conn as *mut libc::c_void;
    let mut __not_first_call: libc::c_int = __sigsetjmp(
        (__cancel_buf.__cancel_jmp_buf).as_mut_ptr() as *mut libc::c_void
            as *mut __jmp_buf_tag,
        0 as libc::c_int,
    );
    if __not_first_call as libc::c_long != 0 {
        __cancel_routine.unwrap()(__cancel_arg);
        __pthread_unwind_next(&mut __cancel_buf);
    }
    __pthread_register_cancel(&mut __cancel_buf);
    pthread_setcanceltype(
        PTHREAD_CANCEL_ASYNCHRONOUS as libc::c_int,
        &mut (*this).type_0,
    );
    pthread_setcancelstate(PTHREAD_CANCEL_ENABLE as libc::c_int, &mut (*this).state);
    if my.login as libc::c_uint == boolean_true as libc::c_int as libc::c_uint {
        let mut tmp: URL = new_url(array_next(my.lurl) as *mut libc::c_char);
        url_set_ID(tmp, 0 as libc::c_int);
        __request(this, tmp);
    }
    len = if my.reps == -(1 as libc::c_int) {
        array_length((*this).urls) as libc::c_int
    } else {
        my.reps
    };
    y = if my.reps == -(1 as libc::c_int) {
        0 as libc::c_int
    } else {
        (*this).id * (my.length / my.cusers)
    };
    max_y = array_length((*this).urls) as libc::c_int;
    x = 0 as libc::c_int;
    while x < len {
        x = if my.secs > 0 as libc::c_int
            && (my.reps <= 0 as libc::c_int || my.reps == 10301062 as libc::c_int)
        {
            0 as libc::c_int
        } else {
            x
        };
        if my.internet as libc::c_uint == boolean_true as libc::c_int as libc::c_uint {
            y = (pthread_rand_np(&mut (*this).rseed) as libc::c_double
                / (2147483647 as libc::c_int as libc::c_double
                    + 1 as libc::c_int as libc::c_double) * my.length as libc::c_double
                + 0.5f64) as libc::c_uint as libc::c_int;
            y = if y >= my.length { my.length - 1 as libc::c_int } else { y };
            y = if y < 0 as libc::c_int { 0 as libc::c_int } else { y };
        } else if y >= max_y {
            y = 0 as libc::c_int;
            if my.expire as u64 != 0 {
                cookies_delete_all(my.cookies);
            }
        }
        if y >= max_y || y < 0 as libc::c_int {
            y = 0 as libc::c_int;
        }
        let mut tmp_0: URL = array_get((*this).urls, y) as URL;
        if !tmp_0.is_null() && !(url_get_hostname(tmp_0)).is_null() {
            (*this).auth.bids.www = 0 as libc::c_int;
            ret = __request(this, tmp_0) as libc::c_int;
            if ret == boolean_false as libc::c_int {
                __increment_failures();
            }
        }
        if my.parser as libc::c_uint == boolean_true as libc::c_int as libc::c_uint
            && !((*this).parts).is_null()
        {
            let mut u: URL = 0 as *mut URL_T;
            loop {
                u = array_pop((*this).parts) as URL;
                if u.is_null() {
                    break;
                }
                if !(url_get_scheme(u) as libc::c_uint
                    == UNSUPPORTED as libc::c_int as libc::c_uint)
                {
                    if my.cache as libc::c_uint != 0
                        && is_cached((*(*this).conn).cache, u) as libc::c_uint != 0
                    {
                        let mut r: RESPONSE = new_response();
                        response_set_code(
                            r,
                            b"HTTP/1.1 200 OK\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                        );
                        response_set_from_cache(r, boolean_true);
                        __display_result(
                            this,
                            r,
                            u,
                            0 as libc::c_int as libc::c_ulong,
                            0.00f64 as libc::c_float,
                        );
                        r = response_destroy(r);
                    } else {
                        (*this).auth.bids.www = 0 as libc::c_int;
                        if __no_follow(url_get_hostname(u)) as u64 == 0 {
                            ret = __request(this, u) as libc::c_int;
                            if ret == boolean_false as libc::c_int {
                                __increment_failures();
                            }
                        }
                    }
                }
                u = url_destroy(u);
            }
        }
        if my.delay >= 1 as libc::c_int as libc::c_float {
            pthread_sleep_np(
                (pthread_rand_np(&mut (*this).rseed) as libc::c_double
                    / (2147483647 as libc::c_int as libc::c_double
                        + 1 as libc::c_int as libc::c_double)
                    * my.delay as libc::c_double + 0.5f64) as libc::c_uint,
            );
        } else if my.delay as libc::c_double >= 0.001f64 {
            pthread_usleep_np(
                (pthread_rand_np(&mut (*this).rseed) as libc::c_double
                    / (2147483647 as libc::c_int as libc::c_double
                        + 1 as libc::c_int as libc::c_double)
                    * my.delay as libc::c_double
                    * 1000000 as libc::c_int as libc::c_double + 0.0005f64)
                    as libc::c_uint as libc::c_ulong,
            );
        }
        if my.failures > 0 as libc::c_int && my.failed >= my.failures {
            break;
        }
        x += 1;
        x;
        y += 1;
        y;
    }
    __pthread_unregister_cancel(&mut __cancel_buf);
    if (*(*this).conn).sock >= 0 as libc::c_int {
        (*(*this).conn).connection.reuse = 0 as libc::c_int;
        socket_close((*this).conn);
    }
    (*(*this).conn).page = page_destroy((*(*this).conn).page);
    (*(*this).conn).cache = cache_destroy((*(*this).conn).cache);
    xfree((*this).conn as *mut libc::c_void);
    (*this).conn = 0 as *mut CONN;
    return 0 as *mut libc::c_void;
}
pub unsafe extern "C" fn browser_set_urls(mut this: BROWSER, mut urls: ARRAY) {
    (*this).urls = urls;
}
pub unsafe extern "C" fn browser_set_cookies(mut this: BROWSER, mut cookies: HASH) {
    let mut i: libc::c_int = 0 as libc::c_int;
    (*this).cookies = cookies;
    if !((*this).cookies).is_null() {
        let mut keys: *mut *mut libc::c_char = hash_get_keys((*this).cookies);
        i = 0 as libc::c_int;
        while i < hash_get_entries((*this).cookies) {
            let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut len: libc::c_int = strlen(
                hash_get((*this).cookies, *keys.offset(i as isize))
                    as *const libc::c_char,
            ) as libc::c_int;
            tmp = xmalloc((len + 2 as libc::c_int) as size_t) as *mut libc::c_char;
            memset(
                tmp as *mut libc::c_void,
                '\0' as i32,
                (len + 2 as libc::c_int) as libc::c_ulong,
            );
            snprintf(
                tmp,
                (len + 1 as libc::c_int) as libc::c_ulong,
                b"%s\0" as *const u8 as *const libc::c_char,
                hash_get((*this).cookies, *keys.offset(i as isize)) as *mut libc::c_char,
            );
            cookies_add(
                my.cookies,
                tmp,
                b".\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            xfree(tmp as *mut libc::c_void);
            i += 1;
            i;
        }
    }
}
unsafe extern "C" fn __request(mut this: BROWSER, mut U: URL) -> BOOLEAN {
    (*(*this).conn).scheme = url_get_scheme(U);
    match (*(*this).conn).scheme as libc::c_uint {
        3 => return __ftp(this, U),
        1 | 2 | _ => return __http(this, U),
    };
}
unsafe extern "C" fn __http(mut this: BROWSER, mut U: URL) -> BOOLEAN {
    let mut bytes: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    let mut code: libc::c_int = 0;
    let mut okay: libc::c_int = 0;
    let mut fail: libc::c_int = 0;
    let mut etime: libc::c_float = 0.;
    let mut start_0: clock_t = 0;
    let mut stop: clock_t = 0;
    let mut t_start: tms = tms {
        tms_utime: 0,
        tms_stime: 0,
        tms_cutime: 0,
        tms_cstime: 0,
    };
    let mut t_stop: tms = tms {
        tms_utime: 0,
        tms_stime: 0,
        tms_cutime: 0,
        tms_cstime: 0,
    };
    let mut resp: RESPONSE = 0 as *mut RESPONSE_T;
    let mut meta: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut keepsake: tm = tm {
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
    let mut now: time_t = 0;
    let mut tmp: *mut tm = 0 as *mut tm;
    let mut len: size_t = 0;
    let mut fmtime: [libc::c_char; 65] = [0; 65];
    let mut redirect_url: URL = 0 as URL;
    page_clear((*(*this).conn).page);
    if my.csv as u64 != 0 {
        now = time(0 as *mut time_t);
        tmp = localtime_r(&mut now, &mut keepsake);
        if !tmp.is_null() {
            len = strftime(
                fmtime.as_mut_ptr(),
                64 as libc::c_int as size_t,
                b"%Y-%m-%d %H:%M:%S\0" as *const u8 as *const libc::c_char,
                tmp,
            );
            if len == 0 as libc::c_int as libc::c_ulong {
                memset(
                    fmtime.as_mut_ptr() as *mut libc::c_void,
                    '\0' as i32,
                    64 as libc::c_int as libc::c_ulong,
                );
                snprintf(
                    fmtime.as_mut_ptr(),
                    64 as libc::c_int as libc::c_ulong,
                    b"n/a\0" as *const u8 as *const libc::c_char,
                );
            }
        } else {
            snprintf(
                fmtime.as_mut_ptr(),
                64 as libc::c_int as libc::c_ulong,
                b"n/a\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    if url_get_scheme(U) as libc::c_uint == UNSUPPORTED as libc::c_int as libc::c_uint {
        if my.verbose as libc::c_uint != 0 && my.get as u64 == 0 && my.print as u64 == 0
        {
            NOTIFY(
                ERROR,
                b"%s %d %6.2f secs: %7d bytes ==> %s\n\0" as *const u8
                    as *const libc::c_char,
                b"UNSPPRTD\0" as *const u8 as *const libc::c_char,
                501 as libc::c_int,
                0.00f64,
                0 as libc::c_int,
                b"PROTOCOL NOT SUPPORTED BY SIEGE\0" as *const u8 as *const libc::c_char,
            );
        }
        return boolean_false;
    }
    start_0 = times(&mut t_start);
    if __init_connection(this, U) as u64 == 0 {
        return boolean_false;
    }
    if url_get_method(U) as libc::c_uint == POST as libc::c_int as libc::c_uint
        || url_get_method(U) as libc::c_uint == PUT as libc::c_int as libc::c_uint
        || url_get_method(U) as libc::c_uint == PATCH as libc::c_int as libc::c_uint
        || url_get_method(U) as libc::c_uint == DELETE as libc::c_int as libc::c_uint
        || url_get_method(U) as libc::c_uint == OPTIONS as libc::c_int as libc::c_uint
    {
        if http_post((*this).conn, U) as libc::c_uint
            == boolean_false as libc::c_int as libc::c_uint
        {
            (*(*this).conn).connection.reuse = 0 as libc::c_int;
            socket_close((*this).conn);
            return boolean_false;
        }
    } else if http_get((*this).conn, U) as libc::c_uint
        == boolean_false as libc::c_int as libc::c_uint
    {
        (*(*this).conn).connection.reuse = 0 as libc::c_int;
        socket_close((*this).conn);
        return boolean_false;
    }
    resp = http_read_headers((*this).conn, U);
    if resp.is_null() {
        (*(*this).conn).connection.reuse = 0 as libc::c_int;
        socket_close((*this).conn);
        echo(
            b"%s:%d NULL headers\0" as *const u8 as *const libc::c_char,
            b"browser.c\0" as *const u8 as *const libc::c_char,
            496 as libc::c_int,
        );
        return boolean_false;
    }
    code = response_get_code(resp);
    if code == 418 as libc::c_int {
        (*(*this).conn).connection.reuse = 0 as libc::c_int;
        socket_close((*this).conn);
        stop = times(&mut t_stop);
        etime = elapsed_time(stop - start_0);
        (*this).hits = ((*this).hits).wrapping_add(1);
        (*this).hits;
        (*this).time += etime;
        (*this).fail = ((*this).fail).wrapping_add(1 as libc::c_int as libc::c_uint);
        __display_result(this, resp, U, 0 as libc::c_int as libc::c_ulong, etime);
        resp = response_destroy(resp);
        return boolean_false;
    }
    bytes = http_read((*this).conn, resp) as libc::c_ulong;
    if my.print as u64 != 0 {
        printf(
            b"%s\n\0" as *const u8 as *const libc::c_char,
            page_value((*(*this).conn).page),
        );
    }
    if my.parser as libc::c_uint == boolean_true as libc::c_int as libc::c_uint {
        if strmatch(
            response_get_content_type(resp),
            b"text/html\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) as libc::c_uint != 0 && code < 300 as libc::c_int
        {
            let mut i: libc::c_int = 0;
            html_parser((*this).parts, U, page_value((*(*this).conn).page));
            i = 0 as libc::c_int;
            while i < array_length((*this).parts) as libc::c_int {
                let mut url: URL = array_get((*this).parts, i) as URL;
                if url_is_redirect(url) as u64 != 0 {
                    let mut tmp_0: URL = array_remove((*this).parts, i) as URL;
                    meta = xstrdup(url_get_absolute(tmp_0));
                    tmp_0 = url_destroy(tmp_0);
                }
                i += 1;
                i;
            }
        }
    }
    if my.zero_ok as u64 == 0 && bytes < 1 as libc::c_int as libc::c_ulong {
        (*(*this).conn).connection.reuse = 0 as libc::c_int;
        socket_close((*this).conn);
        resp = response_destroy(resp);
        echo(
            b"%s:%d zero bytes back from server\0" as *const u8 as *const libc::c_char,
            b"browser.c\0" as *const u8 as *const libc::c_char,
            544 as libc::c_int,
        );
        return boolean_false;
    }
    stop = times(&mut t_stop);
    etime = elapsed_time(stop - start_0);
    okay = response_success(resp);
    fail = response_failure(resp);
    (*this).bytes = ((*this).bytes).wrapping_add(bytes as libc::c_ulonglong);
    (*this).time += etime;
    (*this).code = ((*this).code).wrapping_add(okay as libc::c_uint);
    (*this).fail = ((*this).fail).wrapping_add(fail as libc::c_uint);
    if code == 200 as libc::c_int {
        (*this).okay = ((*this).okay).wrapping_add(1);
        (*this).okay;
    }
    if etime > __himark {
        __himark = etime;
    }
    if __lomark < 0 as libc::c_int as libc::c_float || etime < __lomark {
        __lomark = etime;
    }
    (*this).himark = __himark;
    (*this).lomark = __lomark;
    __display_result(this, resp, U, bytes, etime);
    if my.keepalive as u64 == 0 {
        socket_close((*this).conn);
    }
    match code {
        200 => {
            if !meta.is_null() && strlen(meta) > 2 as libc::c_int as libc::c_ulong {
                redirect_url = url_normalize(U, meta);
                xfree(meta as *mut libc::c_void);
                meta = 0 as *mut libc::c_char;
                page_clear((*(*this).conn).page);
                if empty(url_get_hostname(redirect_url)) as u64 != 0 {
                    url_set_hostname(redirect_url, url_get_hostname(U));
                }
                url_set_redirect(U, boolean_false);
                url_set_redirect(redirect_url, boolean_false);
                if __request(this, redirect_url) as libc::c_uint
                    == boolean_false as libc::c_int as libc::c_uint
                {
                    redirect_url = url_destroy(redirect_url);
                    return boolean_false;
                }
                redirect_url = url_destroy(redirect_url);
            }
        }
        201 => {
            if my.follow as libc::c_uint != 0 && !(response_get_location(resp)).is_null()
            {
                redirect_url = url_normalize(U, response_get_location(resp));
                if empty(url_get_hostname(redirect_url)) as u64 != 0 {
                    url_set_hostname(redirect_url, url_get_hostname(U));
                }
                if __request(this, redirect_url) as libc::c_uint
                    == boolean_false as libc::c_int as libc::c_uint
                {
                    redirect_url = url_destroy(redirect_url);
                    return boolean_false;
                }
            }
        }
        301 | 302 | 303 | 307 => {
            if my.follow as libc::c_uint != 0 && !(response_get_location(resp)).is_null()
            {
                redirect_url = url_normalize(U, response_get_location(resp));
                if empty(url_get_hostname(redirect_url)) as u64 != 0 {
                    url_set_hostname(redirect_url, url_get_hostname(U));
                }
                if code == 307 as libc::c_int {
                    url_set_conttype(redirect_url, url_get_conttype(U));
                    url_set_method(redirect_url, url_get_method(U));
                    if url_get_method(redirect_url) as libc::c_uint
                        == POST as libc::c_int as libc::c_uint
                        || url_get_method(redirect_url) as libc::c_uint
                            == PUT as libc::c_int as libc::c_uint
                        || url_get_method(redirect_url) as libc::c_uint
                            == PATCH as libc::c_int as libc::c_uint
                        || url_get_method(U) as libc::c_uint
                            == DELETE as libc::c_int as libc::c_uint
                        || url_get_method(U) as libc::c_uint
                            == OPTIONS as libc::c_int as libc::c_uint
                    {
                        url_set_postdata(
                            redirect_url,
                            url_get_postdata(U),
                            url_get_postlen(U),
                        );
                    }
                }
                if __request(this, redirect_url) as libc::c_uint
                    == boolean_false as libc::c_int as libc::c_uint
                {
                    redirect_url = url_destroy(redirect_url);
                    return boolean_false;
                }
            }
            redirect_url = url_destroy(redirect_url);
        }
        401 => {
            (*this)
                .auth
                .www = if (*this).auth.www == 0 as libc::c_int {
                1 as libc::c_int
            } else {
                (*this).auth.www
            };
            let fresh0 = (*this).auth.bids.www;
            (*this).auth.bids.www = (*this).auth.bids.www + 1;
            if fresh0 < my.bids - 1 as libc::c_int {
                let mut b: BOOLEAN = boolean_false;
                if response_get_www_auth_type(resp) as libc::c_uint
                    == DIGEST as libc::c_int as libc::c_uint
                {
                    (*this).auth.type_0.www = DIGEST;
                    b = auth_set_digest_header(
                        my.auth,
                        &mut (*this).auth.wchlg,
                        &mut (*this).auth.wcred,
                        &mut (*this).rseed,
                        response_get_www_auth_realm(resp),
                        response_get_www_auth_challenge(resp),
                    );
                    if b as libc::c_uint == boolean_false as libc::c_int as libc::c_uint
                    {
                        fprintf(
                            stderr,
                            b"ERROR: Unable to respond to an authorization challenge\n\0"
                                as *const u8 as *const libc::c_char,
                        );
                        fprintf(
                            stderr,
                            b"       in the following realm: '%s'\n\0" as *const u8
                                as *const libc::c_char,
                            response_get_www_auth_realm(resp),
                        );
                        fprintf(
                            stderr,
                            b"       Did you set login credentials in the conf file?\n\0"
                                as *const u8 as *const libc::c_char,
                        );
                        resp = response_destroy(resp);
                        return boolean_false;
                    }
                }
                if response_get_www_auth_type(resp) as libc::c_uint
                    == NTLM as libc::c_int as libc::c_uint
                {
                    (*this).auth.type_0.www = NTLM;
                    b = auth_set_ntlm_header(
                        my.auth,
                        HTTP,
                        response_get_www_auth_challenge(resp),
                        response_get_www_auth_realm(resp),
                    );
                }
                if response_get_www_auth_type(resp) as libc::c_uint
                    == BASIC as libc::c_int as libc::c_uint
                {
                    (*this).auth.type_0.www = BASIC;
                    auth_set_basic_header(
                        my.auth,
                        HTTP,
                        response_get_www_auth_realm(resp),
                    );
                }
                if __request(this, U) as libc::c_uint
                    == boolean_false as libc::c_int as libc::c_uint
                {
                    fprintf(
                        stderr,
                        b"ERROR from http_request\n\0" as *const u8
                            as *const libc::c_char,
                    );
                    return boolean_false;
                }
            }
        }
        407 => {
            (*this)
                .auth
                .proxy = if (*this).auth.proxy == 0 as libc::c_int {
                1 as libc::c_int
            } else {
                (*this).auth.proxy
            };
            let fresh1 = (*this).auth.bids.proxy;
            (*this).auth.bids.proxy = (*this).auth.bids.proxy + 1;
            if fresh1 < my.bids - 1 as libc::c_int {
                if response_get_proxy_auth_type(resp) as libc::c_uint
                    == DIGEST as libc::c_int as libc::c_uint
                {
                    let mut b_0: BOOLEAN = boolean_false;
                    (*this).auth.type_0.proxy = DIGEST;
                    b_0 = auth_set_digest_header(
                        my.auth,
                        &mut (*this).auth.pchlg,
                        &mut (*this).auth.pcred,
                        &mut (*this).rseed,
                        response_get_proxy_auth_realm(resp),
                        response_get_proxy_auth_challenge(resp),
                    );
                    if b_0 as libc::c_uint
                        == boolean_false as libc::c_int as libc::c_uint
                    {
                        fprintf(
                            stderr,
                            b"ERROR: Unable to respond to a proxy authorization challenge\n\0"
                                as *const u8 as *const libc::c_char,
                        );
                        fprintf(
                            stderr,
                            b"       in the following HTTP realm: '%s'\n\0" as *const u8
                                as *const libc::c_char,
                            response_get_proxy_auth_realm(resp),
                        );
                        fprintf(
                            stderr,
                            b"       Did you set proxy-login credentials in the conf file?\n\0"
                                as *const u8 as *const libc::c_char,
                        );
                        resp = response_destroy(resp);
                        return boolean_false;
                    }
                }
                if response_get_proxy_auth_type(resp) as libc::c_uint
                    == BASIC as libc::c_int as libc::c_uint
                {
                    (*this).auth.type_0.proxy = BASIC;
                    auth_set_basic_header(
                        my.auth,
                        PROXY,
                        response_get_proxy_auth_realm(resp),
                    );
                }
                if __request(this, U) as libc::c_uint
                    == boolean_false as libc::c_int as libc::c_uint
                {
                    return boolean_false;
                }
            }
        }
        408 | 500 | 501 | 502 | 503 | 504 | 505 | 506 | 507 | 508 | 509 => {
            return boolean_false;
        }
        _ => {}
    }
    (*this).hits = ((*this).hits).wrapping_add(1);
    (*this).hits;
    resp = response_destroy(resp);
    return boolean_true;
}
unsafe extern "C" fn __ftp(mut this: BROWSER, mut U: URL) -> BOOLEAN {
    let mut pass: libc::c_int = 0;
    let mut fail: libc::c_int = 0;
    let mut code: libc::c_int = 0 as libc::c_int;
    let mut etime: libc::c_float = 0.;
    let mut D: *mut CONN = 0 as *mut CONN;
    let mut bytes: size_t = 0 as libc::c_int as size_t;
    let mut start_0: clock_t = 0;
    let mut stop: clock_t = 0;
    let mut t_start: tms = tms {
        tms_utime: 0,
        tms_stime: 0,
        tms_cutime: 0,
        tms_cstime: 0,
    };
    let mut t_stop: tms = tms {
        tms_utime: 0,
        tms_stime: 0,
        tms_cutime: 0,
        tms_cstime: 0,
    };
    D = xcalloc(
        ::std::mem::size_of::<CONN>() as libc::c_ulong,
        1 as libc::c_int as size_t,
    ) as *mut CONN;
    (*D).sock = -(1 as libc::c_int);
    if __init_connection(this, U) as u64 == 0 {
        NOTIFY(
            ERROR,
            b"%s:%d connection failed %s:%d\0" as *const u8 as *const libc::c_char,
            b"browser.c\0" as *const u8 as *const libc::c_char,
            765 as libc::c_int,
            url_get_hostname(U),
            url_get_port(U),
        );
        xfree(D as *mut libc::c_void);
        return boolean_false;
    }
    start_0 = times(&mut t_start);
    if (*(*this).conn).sock < 0 as libc::c_int {
        NOTIFY(
            ERROR,
            b"%s:%d connection failed %s:%d\0" as *const u8 as *const libc::c_char,
            b"browser.c\0" as *const u8 as *const libc::c_char,
            775 as libc::c_int,
            url_get_hostname(U),
            url_get_port(U),
        );
        socket_close((*this).conn);
        xfree(D as *mut libc::c_void);
        return boolean_false;
    }
    if (url_get_username(U)).is_null()
        || strlen(url_get_username(U)) < 1 as libc::c_int as libc::c_ulong
    {
        url_set_username(U, auth_get_ftp_username(my.auth, url_get_hostname(U)));
    }
    if (url_get_password(U)).is_null()
        || strlen(url_get_password(U)) < 1 as libc::c_int as libc::c_ulong
    {
        url_set_password(U, auth_get_ftp_password(my.auth, url_get_hostname(U)));
    }
    if ftp_login((*this).conn, U) as libc::c_uint
        == boolean_false as libc::c_int as libc::c_uint
    {
        if my.verbose as u64 != 0 {
            let mut color: libc::c_int = __select_color((*(*this).conn).ftp.code);
            DISPLAY(
                color,
                b"FTP/%d %6.2f secs: %7lu bytes ==> %-6s %s\0" as *const u8
                    as *const libc::c_char,
                (*(*this).conn).ftp.code,
                0.0f64,
                bytes,
                url_get_method_name(U),
                url_get_request(U),
            );
        }
        xfree(D as *mut libc::c_void);
        (*this).fail = ((*this).fail).wrapping_add(1 as libc::c_int as libc::c_uint);
        return boolean_false;
    }
    ftp_pasv((*this).conn);
    if (*(*this).conn).ftp.pasv as libc::c_uint
        == boolean_true as libc::c_int as libc::c_uint
    {
        debug(
            b"Connecting to: %s:%d\0" as *const u8 as *const libc::c_char,
            ((*(*this).conn).ftp.host).as_mut_ptr(),
            (*(*this).conn).ftp.port,
        );
        (*D)
            .sock = new_socket(
            D,
            ((*(*this).conn).ftp.host).as_mut_ptr(),
            (*(*this).conn).ftp.port,
        );
        if (*D).sock < 0 as libc::c_int {
            debug(
                b"%s:%d connection failed. error %d(%s)\0" as *const u8
                    as *const libc::c_char,
                b"browser.c\0" as *const u8 as *const libc::c_char,
                809 as libc::c_int,
                *__errno_location(),
                strerror(*__errno_location()),
            );
            (*this).fail = ((*this).fail).wrapping_add(1 as libc::c_int as libc::c_uint);
            socket_close(D);
            xfree(D as *mut libc::c_void);
            return boolean_false;
        }
    }
    if url_get_method(U) as libc::c_uint == POST as libc::c_int as libc::c_uint
        || url_get_method(U) as libc::c_uint == PUT as libc::c_int as libc::c_uint
        || url_get_method(U) as libc::c_uint == PATCH as libc::c_int as libc::c_uint
        || url_get_method(U) as libc::c_uint == DELETE as libc::c_int as libc::c_uint
        || url_get_method(U) as libc::c_uint == OPTIONS as libc::c_int as libc::c_uint
    {
        ftp_stor((*this).conn, U);
        bytes = ftp_put(D, U);
        code = (*(*this).conn).ftp.code;
    } else {
        if ftp_size((*this).conn, U) as libc::c_uint
            == boolean_true as libc::c_int as libc::c_uint
        {
            if ftp_retr((*this).conn, U) as libc::c_uint
                == boolean_true as libc::c_int as libc::c_uint
            {
                bytes = ftp_get(D, U, (*(*this).conn).ftp.size);
            }
        }
        code = (*(*this).conn).ftp.code;
    }
    socket_close(D);
    ftp_quit((*this).conn);
    pass = if bytes == (*(*this).conn).ftp.size {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    };
    fail = if pass == 0 as libc::c_int { 1 as libc::c_int } else { 0 as libc::c_int };
    stop = times(&mut t_stop);
    etime = elapsed_time(stop - start_0);
    (*this).bytes = ((*this).bytes).wrapping_add(bytes as libc::c_ulonglong);
    (*this).time += etime;
    (*this).code = ((*this).code).wrapping_add(pass as libc::c_uint);
    (*this).fail = ((*this).fail).wrapping_add(fail as libc::c_uint);
    if etime > __himark {
        __himark = etime;
    }
    if __lomark < 0 as libc::c_int as libc::c_float || etime < __lomark {
        __lomark = etime;
    }
    (*this).himark = __himark;
    (*this).lomark = __lomark;
    if my.verbose as u64 != 0 {
        let mut color_0: libc::c_int = if my.color as libc::c_uint
            == boolean_true as libc::c_int as libc::c_uint
        {
            __select_color(code)
        } else {
            -(1 as libc::c_int)
        };
        DISPLAY(
            color_0,
            b"FTP/%d %6.2f secs: %7lu bytes ==> %-6s %s\0" as *const u8
                as *const libc::c_char,
            code,
            etime as libc::c_double,
            bytes,
            url_get_method_name(U),
            url_get_request(U),
        );
    }
    (*this).hits = ((*this).hits).wrapping_add(1);
    (*this).hits;
    xfree(D as *mut libc::c_void);
    return boolean_true;
}
unsafe extern "C" fn __init_connection(mut this: BROWSER, mut U: URL) -> BOOLEAN {
    (*(*this).conn).pos_ini = 0 as libc::c_int;
    (*(*this).conn).inbuffer = 0 as libc::c_int as size_t;
    (*(*this).conn).content.transfer = NONE as libc::c_int;
    (*(*this).conn).content.length = !(0 as libc::c_long) as size_t;
    (*(*this).conn)
        .connection
        .keepalive = (if (*(*this).conn).connection.max == 1 as libc::c_int {
        0 as libc::c_int as libc::c_uint
    } else {
        my.keepalive as libc::c_uint
    }) as libc::c_int;
    (*(*this).conn)
        .connection
        .reuse = (if (*(*this).conn).connection.max == 1 as libc::c_int {
        0 as libc::c_int as libc::c_uint
    } else {
        my.keepalive as libc::c_uint
    }) as libc::c_int;
    (*(*this).conn)
        .connection
        .tested = if (*(*this).conn).connection.tested == 0 as libc::c_int {
        1 as libc::c_int
    } else {
        (*(*this).conn).connection.tested
    };
    (*(*this).conn).auth.www = (*this).auth.www;
    (*(*this).conn).auth.wchlg = (*this).auth.wchlg;
    (*(*this).conn).auth.wcred = (*this).auth.wcred;
    (*(*this).conn).auth.proxy = (*this).auth.proxy;
    (*(*this).conn).auth.pchlg = (*this).auth.pchlg;
    (*(*this).conn).auth.pcred = (*this).auth.pcred;
    (*(*this).conn).auth.type_0.www = (*this).auth.type_0.www;
    (*(*this).conn).auth.type_0.proxy = (*this).auth.type_0.proxy;
    memset(
        ((*(*this).conn).buffer).as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong,
    );
    debug(
        b"%s:%d attempting connection to %s:%d\0" as *const u8 as *const libc::c_char,
        b"browser.c\0" as *const u8 as *const libc::c_char,
        888 as libc::c_int,
        if auth_get_proxy_required(my.auth) as libc::c_uint != 0 {
            auth_get_proxy_host(my.auth)
        } else {
            url_get_hostname(U)
        },
        if auth_get_proxy_required(my.auth) as libc::c_uint != 0 {
            auth_get_proxy_port(my.auth)
        } else {
            url_get_port(U)
        },
    );
    if (*(*this).conn).connection.reuse == 0
        || (*(*this).conn).connection.status == 0 as libc::c_int
    {
        if auth_get_proxy_required(my.auth) as u64 != 0 {
            debug(
                b"%s:%d creating new socket:     %s:%d\0" as *const u8
                    as *const libc::c_char,
                b"browser.c\0" as *const u8 as *const libc::c_char,
                897 as libc::c_int,
                auth_get_proxy_host(my.auth),
                auth_get_proxy_port(my.auth),
            );
            (*(*this).conn)
                .sock = new_socket(
                (*this).conn,
                auth_get_proxy_host(my.auth),
                auth_get_proxy_port(my.auth),
            );
        } else {
            debug(
                b"%s:%d creating new socket:     %s:%d\0" as *const u8
                    as *const libc::c_char,
                b"browser.c\0" as *const u8 as *const libc::c_char,
                903 as libc::c_int,
                url_get_hostname(U),
                url_get_port(U),
            );
            (*(*this).conn)
                .sock = new_socket((*this).conn, url_get_hostname(U), url_get_port(U));
        }
    }
    if my.keepalive as u64 != 0 {
        (*(*this).conn).connection.reuse = boolean_true as libc::c_int;
    }
    if (*(*this).conn).sock < 0 as libc::c_int {
        debug(
            b"%s:%d connection failed. error %d(%s)\0" as *const u8
                as *const libc::c_char,
            b"browser.c\0" as *const u8 as *const libc::c_char,
            915 as libc::c_int,
            *__errno_location(),
            strerror(*__errno_location()),
        );
        socket_close((*this).conn);
        return boolean_false;
    }
    debug(
        b"%s:%d good socket connection:  %s:%d\0" as *const u8 as *const libc::c_char,
        b"browser.c\0" as *const u8 as *const libc::c_char,
        923 as libc::c_int,
        if auth_get_proxy_required(my.auth) as libc::c_uint != 0 {
            auth_get_proxy_host(my.auth)
        } else {
            url_get_hostname(U)
        },
        if auth_get_proxy_required(my.auth) as libc::c_uint != 0 {
            auth_get_proxy_port(my.auth)
        } else {
            url_get_port(U)
        },
    );
    if url_get_scheme(U) as libc::c_uint == HTTPS as libc::c_int as libc::c_uint {
        if auth_get_proxy_required(my.auth) as u64 != 0 {
            https_tunnel_request((*this).conn, url_get_hostname(U), url_get_port(U));
            https_tunnel_response((*this).conn);
        }
        (*(*this).conn).encrypt = boolean_true;
        if SSL_initialize((*this).conn, url_get_hostname(U)) as libc::c_uint
            == boolean_false as libc::c_int as libc::c_uint
        {
            return boolean_false;
        }
    }
    return boolean_true;
}
unsafe extern "C" fn __display_result(
    mut this: BROWSER,
    mut resp: RESPONSE,
    mut U: URL,
    mut bytes: libc::c_ulong,
    mut etime: libc::c_float,
) {
    let mut fmtime: [libc::c_char; 65] = [0; 65];
    let mut keepsake: tm = tm {
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
    let mut now: time_t = 0;
    let mut tmp: *mut tm = 0 as *mut tm;
    let mut len: size_t = 0;
    if my.csv as u64 != 0 {
        now = time(0 as *mut time_t);
        tmp = localtime_r(&mut now, &mut keepsake);
        if !tmp.is_null() {
            len = strftime(
                fmtime.as_mut_ptr(),
                64 as libc::c_int as size_t,
                b"%Y-%m-%d %H:%M:%S\0" as *const u8 as *const libc::c_char,
                tmp,
            );
            if len == 0 as libc::c_int as libc::c_ulong {
                memset(
                    fmtime.as_mut_ptr() as *mut libc::c_void,
                    '\0' as i32,
                    64 as libc::c_int as libc::c_ulong,
                );
                snprintf(
                    fmtime.as_mut_ptr(),
                    64 as libc::c_int as libc::c_ulong,
                    b"n/a\0" as *const u8 as *const libc::c_char,
                );
            }
        } else {
            snprintf(
                fmtime.as_mut_ptr(),
                64 as libc::c_int as libc::c_ulong,
                b"n/a\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    if my.verbose as libc::c_uint != 0 && my.get as u64 == 0 && my.print as u64 == 0
        && my.debug as u64 == 0
    {
        let mut color: libc::c_int = if my.color as libc::c_uint
            == boolean_true as libc::c_int as libc::c_uint
        {
            __select_color(response_get_code(resp))
        } else {
            -(1 as libc::c_int)
        };
        let mut date: DATE = new_date(0 as *mut libc::c_char);
        let mut stamp: *mut libc::c_char = (if my.timestamp as libc::c_uint != 0 {
            date_stamp(date) as *const libc::c_char
        } else {
            b"\0" as *const u8 as *const libc::c_char
        }) as *mut libc::c_char;
        let mut cached: *mut libc::c_char = (if response_get_from_cache(resp)
            as libc::c_uint != 0
        {
            b"(C)\0" as *const u8 as *const libc::c_char
        } else {
            b"   \0" as *const u8 as *const libc::c_char
        }) as *mut libc::c_char;
        if my.color as libc::c_uint != 0
            && response_get_from_cache(resp) as libc::c_uint
                == boolean_true as libc::c_int as libc::c_uint
        {
            color = 2 as libc::c_int;
        }
        if my.csv as u64 != 0 {
            if my.display as u64 != 0 {
                DISPLAY(
                    color,
                    b"%s%s%s%4d,%s,%d,%6.2f,%7lu,%s,%d,%s\0" as *const u8
                        as *const libc::c_char,
                    stamp,
                    if my.mark as libc::c_uint != 0 {
                        my.markstr as *const libc::c_char
                    } else {
                        b"\0" as *const u8 as *const libc::c_char
                    },
                    if my.mark as libc::c_uint != 0 {
                        b",\0" as *const u8 as *const libc::c_char
                    } else {
                        b"\0" as *const u8 as *const libc::c_char
                    },
                    (*this).id,
                    response_get_protocol(resp),
                    response_get_code(resp),
                    etime as libc::c_double,
                    bytes,
                    url_get_display(U),
                    url_get_ID(U),
                    fmtime.as_mut_ptr(),
                );
            } else {
                DISPLAY(
                    color,
                    b"%s%s%s%s,%d,%6.2f,%7lu,%s,%d,%s\0" as *const u8
                        as *const libc::c_char,
                    stamp,
                    if my.mark as libc::c_uint != 0 {
                        my.markstr as *const libc::c_char
                    } else {
                        b"\0" as *const u8 as *const libc::c_char
                    },
                    if my.mark as libc::c_uint != 0 {
                        b",\0" as *const u8 as *const libc::c_char
                    } else {
                        b"\0" as *const u8 as *const libc::c_char
                    },
                    response_get_protocol(resp),
                    response_get_code(resp),
                    etime as libc::c_double,
                    bytes,
                    url_get_display(U),
                    url_get_ID(U),
                    fmtime.as_mut_ptr(),
                );
            }
        } else if my.display as u64 != 0 {
            DISPLAY(
                color,
                b"%4d) %s %d %6.2f secs: %7lu bytes ==> %-4s %s\0" as *const u8
                    as *const libc::c_char,
                (*this).id,
                response_get_protocol(resp),
                response_get_code(resp),
                etime as libc::c_double,
                bytes,
                url_get_method_name(U),
                url_get_display(U),
            );
        } else {
            DISPLAY(
                color,
                b"%s%s %d%s %5.2f secs: %7lu bytes ==> %-4s %s\0" as *const u8
                    as *const libc::c_char,
                stamp,
                response_get_protocol(resp),
                response_get_code(resp),
                cached,
                etime as libc::c_double,
                bytes,
                url_get_method_name(U),
                url_get_display(U),
            );
        }
        date = date_destroy(date);
    }
}
unsafe extern "C" fn __increment_failures() {
    pthread_mutex_lock(&mut my.lock);
    my.failed += 1;
    my.failed;
    pthread_mutex_unlock(&mut my.lock);
    pthread_testcancel();
}
unsafe extern "C" fn __no_follow(mut hostname: *const libc::c_char) -> BOOLEAN {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*my.nomap).index {
        if !(stristr(hostname, *((*my.nomap).line).offset(i as isize))).is_null() {
            return boolean_true;
        }
        i += 1;
        i;
    }
    return boolean_false;
}
unsafe extern "C" fn __select_color(mut code: libc::c_int) -> libc::c_int {
    match code {
        150 | 200 | 201 | 202 | 203 | 204 | 205 | 206 | 226 => return 4 as libc::c_int,
        300 | 301 | 302 | 303 | 304 | 305 | 306 | 307 => return 6 as libc::c_int,
        400 | 401 | 402 | 403 | 404 | 405 | 406 | 407 | 408 | 409 | 410 | 411 | 412 | 413
        | 414 | 415 | 416 | 417 => return 5 as libc::c_int,
        500 | 501 | 502 | 503 | 504 | 505 | _ => return 1 as libc::c_int,
    };
}
unsafe extern "C" fn __signal_cleanup() {}
