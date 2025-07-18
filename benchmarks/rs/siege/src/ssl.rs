use ::libc;
extern "C" {
    pub type AUTH_T;
    pub type DIGEST_CRED;
    pub type DIGEST_CHLG;
    pub type ARRAY_T;
    pub type COOKIES_T;
    pub type x509_st;
    pub type ossl_init_settings_st;
    pub type ssl_st;
    pub type ssl_ctx_st;
    pub type ssl_method_st;
    pub type PAGE_T;
    pub type CACHE_T;
    fn pthread_self() -> pthread_t;
    static mut my: CONFIG;
    fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> libc::c_int;
    fn pthread_mutex_destroy(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn CRYPTO_malloc(
        num: size_t,
        file: *const libc::c_char,
        line: libc::c_int,
    ) -> *mut libc::c_void;
    fn CRYPTO_free(ptr: *mut libc::c_void, file: *const libc::c_char, line: libc::c_int);
    fn ERR_remove_state(pid: libc::c_ulong);
    fn ERR_remove_thread_state(_: *mut libc::c_void);
    fn ERR_error_string(e: libc::c_ulong, buf: *mut libc::c_char) -> *mut libc::c_char;
    fn ERR_peek_error() -> libc::c_ulong;
    fn ERR_get_error() -> libc::c_ulong;
    fn OPENSSL_init_ssl(
        opts: uint64_t,
        settings: *const OPENSSL_INIT_SETTINGS,
    ) -> libc::c_int;
    fn TLS_client_method() -> *const SSL_METHOD;
    fn SSL_get_error(s: *const SSL, ret_code: libc::c_int) -> libc::c_int;
    fn SSL_CTX_ctrl(
        ctx: *mut SSL_CTX,
        cmd: libc::c_int,
        larg: libc::c_long,
        parg: *mut libc::c_void,
    ) -> libc::c_long;
    fn SSL_ctrl(
        ssl: *mut SSL,
        cmd: libc::c_int,
        larg: libc::c_long,
        parg: *mut libc::c_void,
    ) -> libc::c_long;
    fn SSL_connect(ssl: *mut SSL) -> libc::c_int;
    fn SSL_new(ctx: *mut SSL_CTX) -> *mut SSL;
    fn SSL_CTX_check_private_key(ctx: *const SSL_CTX) -> libc::c_int;
    fn SSL_CTX_use_certificate_chain_file(
        ctx: *mut SSL_CTX,
        file: *const libc::c_char,
    ) -> libc::c_int;
    fn SSL_CTX_use_PrivateKey_file(
        ctx: *mut SSL_CTX,
        file: *const libc::c_char,
        type_0: libc::c_int,
    ) -> libc::c_int;
    fn SSL_set_fd(s: *mut SSL, fd: libc::c_int) -> libc::c_int;
    fn SSL_CTX_set_timeout(ctx: *mut SSL_CTX, t: libc::c_long) -> libc::c_long;
    fn SSL_CTX_new(meth: *const SSL_METHOD) -> *mut SSL_CTX;
    fn SSL_CTX_set_cipher_list(_: *mut SSL_CTX, str: *const libc::c_char) -> libc::c_int;
    fn SSL_CTX_set_options(ctx: *mut SSL_CTX, op: libc::c_ulong) -> libc::c_ulong;
    fn stralloc(_: *mut libc::c_char) -> *mut libc::c_char;
    fn xfree(ptr: *mut libc::c_void);
    fn NOTIFY(L: LEVEL, fmt: *const libc::c_char, _: ...);
}
pub type size_t = libc::c_ulong;
pub type __uint64_t = libc::c_ulong;
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
pub union pthread_mutexattr_t {
    pub __size: [libc::c_char; 4],
    pub __align: libc::c_int,
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
pub type BOOLEAN = libc::c_uint;
pub const boolean_true: BOOLEAN = 1;
pub const boolean_false: BOOLEAN = 0;
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
pub type uint64_t = __uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pollfd {
    pub fd: libc::c_int,
    pub events: libc::c_short,
    pub revents: libc::c_short,
}
pub type X509 = x509_st;
pub type OPENSSL_INIT_SETTINGS = ossl_init_settings_st;
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
    pub content: C2RustUnnamed_7,
    pub connection: C2RustUnnamed_6,
    pub auth: C2RustUnnamed_4,
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
    pub ftp: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub code: libc::c_int,
    pub host: [libc::c_char; 64],
    pub port: libc::c_int,
    pub size: size_t,
    pub pasv: BOOLEAN,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub wchlg: *mut DCHLG,
    pub wcred: *mut DCRED,
    pub www: libc::c_int,
    pub pchlg: *mut DCHLG,
    pub pcred: *mut DCRED,
    pub proxy: libc::c_int,
    pub type_0: C2RustUnnamed_5,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub www: TYPE,
    pub proxy: TYPE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub max: libc::c_int,
    pub timeout: libc::c_int,
    pub reuse: libc::c_int,
    pub status: libc::c_int,
    pub keepalive: libc::c_int,
    pub tested: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub transfer: libc::c_int,
    pub length: size_t,
}
pub type LEVEL = libc::c_uint;
pub const FATAL: LEVEL = 3;
pub const ERROR: LEVEL = 2;
pub const WARNING: LEVEL = 1;
pub const DEBUG: LEVEL = 0;
static mut lock_cs: *mut pthread_mutex_t = 0 as *const pthread_mutex_t
    as *mut pthread_mutex_t;
static mut lock_count: *mut libc::c_long = 0 as *const libc::c_long as *mut libc::c_long;
pub unsafe extern "C" fn SSL_initialize(
    mut C: *mut CONN,
    mut servername: *const libc::c_char,
) -> BOOLEAN {
    let mut i: libc::c_int = 0;
    let mut serr: libc::c_int = 0;
    if !((*C).ssl).is_null() {
        return boolean_true;
    }
    (*C).ssl = 0 as *mut SSL;
    (*C).ctx = 0 as *mut SSL_CTX;
    (*C).method = 0 as *const SSL_METHOD;
    (*C).cert = 0 as *mut X509;
    if (my.ssl_key).is_null() && !(my.ssl_cert).is_null() {
        my.ssl_key = my.ssl_cert;
    }
    if (my.ssl_ciphers).is_null() {
        my
            .ssl_ciphers = stralloc(
            b"ALL:!COMPLEMENTOFDEFAULT:!eNULL\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    (*C).method = TLS_client_method() as *mut SSL_METHOD;
    if ((*C).method).is_null() {
        SSL_error_stack();
        return boolean_false;
    }
    (*C).ctx = SSL_CTX_new((*C).method);
    if ((*C).ctx).is_null() {
        SSL_error_stack();
        return boolean_false;
    }
    SSL_CTX_ctrl(
        (*C).ctx,
        33 as libc::c_int,
        (0x1 as libc::c_uint | 0x2 as libc::c_uint) as libc::c_long,
        0 as *mut libc::c_void,
    );
    SSL_CTX_set_options(
        (*C).ctx,
        (0 as libc::c_int as libc::c_uint | 0x2000000 as libc::c_uint) as libc::c_ulong,
    );
    SSL_CTX_ctrl(
        (*C).ctx,
        44 as libc::c_int,
        (0x1 as libc::c_int | 0x2 as libc::c_int) as libc::c_long,
        0 as *mut libc::c_void,
    );
    SSL_CTX_set_timeout((*C).ctx, my.ssl_timeout as libc::c_long);
    if !(my.ssl_ciphers).is_null() {
        if SSL_CTX_set_cipher_list((*C).ctx, my.ssl_ciphers) == 0 {
            NOTIFY(
                ERROR,
                b"SSL_CTX_set_cipher_list\0" as *const u8 as *const libc::c_char,
            );
            return boolean_false;
        }
    }
    if !(my.ssl_cert).is_null() {
        if SSL_CTX_use_certificate_chain_file((*C).ctx, my.ssl_cert) == 0 {
            SSL_error_stack();
            NOTIFY(
                ERROR,
                b"Error reading certificate file: %s\0" as *const u8
                    as *const libc::c_char,
                my.ssl_cert,
            );
        }
        i = 0 as libc::c_int;
        while i < 3 as libc::c_int {
            if SSL_CTX_use_PrivateKey_file((*C).ctx, my.ssl_key, 1 as libc::c_int) != 0 {
                break;
            }
            if i < 2 as libc::c_int
                && (ERR_peek_error() & 0xfff as libc::c_long as libc::c_ulong)
                    as libc::c_int == 100 as libc::c_int
            {
                SSL_error_stack();
                NOTIFY(
                    WARNING,
                    b"Wrong pass phrase: retrying\0" as *const u8 as *const libc::c_char,
                );
            }
            i += 1;
            i;
        }
        if SSL_CTX_check_private_key((*C).ctx) == 0 {
            NOTIFY(
                ERROR,
                b"Private key does not match the certificate\0" as *const u8
                    as *const libc::c_char,
            );
            return boolean_false;
        }
    }
    (*C).ssl = SSL_new((*C).ctx);
    SSL_ctrl(
        (*C).ssl,
        55 as libc::c_int,
        0 as libc::c_int as libc::c_long,
        servername as *mut libc::c_char as *mut libc::c_void,
    );
    if ((*C).ssl).is_null() {
        SSL_error_stack();
        return boolean_false;
    }
    SSL_set_fd((*C).ssl, (*C).sock);
    serr = SSL_connect((*C).ssl);
    if serr != 1 as libc::c_int {
        SSL_error_stack();
        NOTIFY(
            ERROR,
            b"Failed to make an SSL connection: %d\0" as *const u8
                as *const libc::c_char,
            SSL_get_error((*C).ssl, serr),
        );
        return boolean_false;
    }
    return boolean_true;
}
pub unsafe extern "C" fn SSL_thread_setup() {
    let mut x: libc::c_int = 0;
    OPENSSL_init_ssl(0 as libc::c_int as uint64_t, 0 as *const OPENSSL_INIT_SETTINGS);
    OPENSSL_init_ssl(
        (0x200000 as libc::c_long | 0x2 as libc::c_long) as uint64_t,
        0 as *const OPENSSL_INIT_SETTINGS,
    );
    lock_cs = CRYPTO_malloc(
        (1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<pthread_mutex_t>() as libc::c_ulong),
        b"ssl.c\0" as *const u8 as *const libc::c_char,
        192 as libc::c_int,
    ) as *mut pthread_mutex_t;
    lock_count = CRYPTO_malloc(
        (1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_long>() as libc::c_ulong),
        b"ssl.c\0" as *const u8 as *const libc::c_char,
        195 as libc::c_int,
    ) as *mut libc::c_long;
    x = 0 as libc::c_int;
    while x < 1 as libc::c_int {
        *lock_count.offset(x as isize) = 0 as libc::c_int as libc::c_long;
        pthread_mutex_init(
            &mut *lock_cs.offset(x as isize),
            0 as *const pthread_mutexattr_t,
        );
        x += 1;
        x;
    }
}
pub unsafe extern "C" fn SSL_thread_cleanup() {
    let mut x: libc::c_int = 0;
    xfree(my.ssl_ciphers as *mut libc::c_void);
    x = 0 as libc::c_int;
    while x < 1 as libc::c_int {
        pthread_mutex_destroy(&mut *lock_cs.offset(x as isize));
        x += 1;
        x;
    }
    if !lock_cs.is_null() {
        CRYPTO_free(
            lock_cs as *mut libc::c_void,
            b"ssl.c\0" as *const u8 as *const libc::c_char,
            217 as libc::c_int,
        );
        lock_cs = 0 as *mut libc::c_void as *mut pthread_mutex_t;
    }
    if !lock_count.is_null() {
        CRYPTO_free(
            lock_count as *mut libc::c_void,
            b"ssl.c\0" as *const u8 as *const libc::c_char,
            221 as libc::c_int,
        );
        lock_count = 0 as *mut libc::c_void as *mut libc::c_long;
    }
    ERR_remove_state(0 as libc::c_int as libc::c_ulong);
    ERR_remove_thread_state(0 as *mut libc::c_void);
}
pub unsafe extern "C" fn SSL_pthreads_thread_id() -> libc::c_ulong {
    let mut ret: libc::c_ulong = 0;
    ret = pthread_self();
    return ret;
}
unsafe extern "C" fn SSL_error_stack() {
    let mut err: libc::c_ulong = 0;
    let mut string: [libc::c_char; 120] = [0; 120];
    err = ERR_get_error();
    if err == 0 {
        return;
    }
    SSL_error_stack();
    ERR_error_string(err, string.as_mut_ptr());
    NOTIFY(
        ERROR,
        b"stack: %lX : %s\0" as *const u8 as *const libc::c_char,
        err,
        string.as_mut_ptr(),
    );
}
