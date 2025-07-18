use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type URL_T;
    pub type AUTH_T;
    pub type ARRAY_T;
    pub type HASH_T;
    pub type COOKIES_T;
    pub type CREW_T;
    pub type BROWSER_T;
    pub type DATA_T;
    fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option::<
            unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
        >,
        __arg: *mut libc::c_void,
    ) -> libc::c_int;
    fn pthread_attr_init(__attr: *mut pthread_attr_t) -> libc::c_int;
    fn pthread_attr_setscope(
        __attr: *mut pthread_attr_t,
        __scope: libc::c_int,
    ) -> libc::c_int;
    fn sigemptyset(__set: *mut sigset_t) -> libc::c_int;
    fn sigaddset(__set: *mut sigset_t, __signo: libc::c_int) -> libc::c_int;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn puts(__s: *const libc::c_char) -> libc::c_int;
    fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_double;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn exit(_: libc::c_int) -> !;
    fn sigprocmask(
        __how: libc::c_int,
        __set: *const sigset_t,
        __oset: *mut sigset_t,
    ) -> libc::c_int;
    fn sleep(__seconds: libc::c_uint) -> libc::c_uint;
    static mut optarg: *mut libc::c_char;
    static mut optind: libc::c_int;
    fn load_cookies(this: COOKIES) -> HASH;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    static mut URLSIZE: size_t;
    fn new_url(str: *mut libc::c_char) -> URL;
    fn url_destroy(this: URL) -> URL;
    fn url_set_ID(this: URL, id: libc::c_int);
    fn url_set_method(this: URL, method: METHOD);
    fn url_get_method(this: URL) -> METHOD;
    fn url_get_hostname(this: URL) -> *mut libc::c_char;
    fn new_array() -> ARRAY;
    fn array_destroyer(this: ARRAY, m: method) -> ARRAY;
    fn array_npush(this: ARRAY, thing: *mut libc::c_void, len: size_t);
    fn array_get(this: ARRAY, index: libc::c_int) -> *mut libc::c_void;
    fn array_length(this: ARRAY) -> size_t;
    fn hash_get(this: HASH, key: *mut libc::c_char) -> *mut libc::c_void;
    fn hash_destroy(this: HASH) -> HASH;
    fn cookies_destroy(this: COOKIES) -> COOKIES;
    fn new_crew(size: libc::c_int, maxsize: libc::c_int, block: BOOLEAN) -> CREW;
    fn crew_add(
        this: CREW,
        routine: Option::<unsafe extern "C" fn() -> ()>,
        arg: *mut libc::c_void,
    ) -> BOOLEAN;
    fn crew_join(
        this: CREW,
        finish: BOOLEAN,
        payload: *mut *mut libc::c_void,
    ) -> BOOLEAN;
    fn crew_destroy(this: CREW);
    fn crew_get_total(this: CREW) -> libc::c_int;
    fn crew_get_shutdown(this: CREW) -> BOOLEAN;
    fn sig_handler(crew: CREW);
    fn siege_timer(handler: pthread_t);
    static mut BROWSERSIZE: size_t;
    fn new_browser(id: libc::c_int) -> BROWSER;
    fn browser_destroy(this: BROWSER) -> BROWSER;
    fn start(this: BROWSER) -> *mut libc::c_void;
    fn browser_set_urls(this: BROWSER, urls: ARRAY);
    fn browser_set_cookies(this: BROWSER, cookies: HASH);
    fn browser_get_hits(this: BROWSER) -> libc::c_ulong;
    fn browser_get_bytes(this: BROWSER) -> libc::c_ulonglong;
    fn browser_get_time(this: BROWSER) -> libc::c_float;
    fn browser_get_code(this: BROWSER) -> libc::c_uint;
    fn browser_get_okay(this: BROWSER) -> libc::c_uint;
    fn browser_get_fail(this: BROWSER) -> libc::c_uint;
    fn browser_get_himark(this: BROWSER) -> libc::c_float;
    fn browser_get_lomark(this: BROWSER) -> libc::c_float;
    fn parse_time(p: *mut libc::c_char);
    fn pthread_usleep_np(usec: libc::c_ulong);
    fn strmatch(str1: *mut libc::c_char, str2: *mut libc::c_char) -> BOOLEAN;
    fn uppercase(s: *mut libc::c_char, len: size_t) -> *mut libc::c_char;
    fn new_data() -> DATA;
    fn data_destroy(this: DATA) -> DATA;
    fn data_set_start(this: DATA);
    fn data_set_stop(this: DATA);
    fn data_set_highest(this: DATA, highest: libc::c_float);
    fn data_set_lowest(this: DATA, lowest: libc::c_float);
    fn data_increment_bytes(this: DATA, bytes: libc::c_ulong);
    fn data_increment_count(this: DATA, count: libc::c_ulong);
    fn data_increment_total(this: DATA, total: libc::c_float);
    fn data_increment_code(this: DATA, code: libc::c_int);
    fn data_increment_fail(this: DATA, fail: libc::c_int);
    fn data_increment_okay(this: DATA, ok200: libc::c_int);
    fn data_get_megabytes(this: DATA) -> libc::c_float;
    fn data_get_throughput(this: DATA) -> libc::c_float;
    fn data_get_concurrency(this: DATA) -> libc::c_float;
    fn data_get_count(this: DATA) -> libc::c_uint;
    fn data_get_code(this: DATA) -> libc::c_uint;
    fn data_get_okay(this: DATA) -> libc::c_uint;
    fn log_transaction(D: DATA);
    fn mark_log_file(message: *mut libc::c_char);
    fn data_get_highest(this: DATA) -> libc::c_float;
    fn data_get_lowest(this: DATA) -> libc::c_float;
    fn data_get_elapsed(this: DATA) -> libc::c_float;
    fn data_get_response_time(this: DATA) -> libc::c_float;
    fn data_get_transaction_rate(this: DATA) -> libc::c_float;
    fn init_config() -> libc::c_int;
    fn show_config(EXIT: libc::c_int) -> libc::c_int;
    fn ds_module_check();
    fn read_cfg_file(l: *mut LINES, filename: *mut libc::c_char) -> libc::c_int;
    fn SSL_thread_cleanup();
    fn SSL_thread_setup();
    static mut version_string: *const libc::c_char;
    static mut program_name: *const libc::c_char;
    static mut copyright: *const libc::c_char;
    fn xstrdup(str: *const libc::c_char) -> *mut libc::c_char;
    fn xcalloc(_: size_t, _: size_t) -> *mut libc::c_void;
    fn xfree(ptr: *mut libc::c_void);
    fn NOTIFY(L: LEVEL, fmt: *const libc::c_char, _: ...);
    fn getopt_long(
        __argc: libc::c_int,
        __argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
        __longopts: *const option,
        __longind: *mut libc::c_int,
    ) -> libc::c_int;
}
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_attr_t {
    pub __size: [libc::c_char; 56],
    pub __align: libc::c_long,
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
pub const PTHREAD_SCOPE_PROCESS: C2RustUnnamed_3 = 1;
pub const PTHREAD_SCOPE_SYSTEM: C2RustUnnamed_3 = 0;
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
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
pub type sigset_t = __sigset_t;
pub type method = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
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
pub type AUTH = *mut AUTH_T;
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
pub type CREW = *mut CREW_T;
pub type BROWSER = *mut BROWSER_T;
pub type DATA = *mut DATA_T;
pub type LEVEL = libc::c_uint;
pub const FATAL: LEVEL = 3;
pub const ERROR: LEVEL = 2;
pub const WARNING: LEVEL = 1;
pub const DEBUG: LEVEL = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const libc::c_char,
    pub has_arg: libc::c_int,
    pub flag: *mut libc::c_int,
    pub val: libc::c_int,
}
#[inline]
unsafe extern "C" fn atof(mut __nptr: *const libc::c_char) -> libc::c_double {
    return strtod(__nptr, 0 as *mut libc::c_void as *mut *mut libc::c_char);
}
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
pub static mut my: CONFIG = CONFIG {
    logging: boolean_false,
    shlog: boolean_false,
    limit: 0,
    url: 0 as *const libc::c_char as *mut libc::c_char,
    logfile: [0; 128],
    verbose: boolean_false,
    quiet: boolean_false,
    parser: boolean_false,
    csv: boolean_false,
    fullurl: boolean_false,
    display: boolean_false,
    config: boolean_false,
    color: boolean_false,
    cusers: 0,
    delay: 0.,
    timeout: 0,
    bench: boolean_false,
    internet: boolean_false,
    timestamp: boolean_false,
    time: 0,
    secs: 0,
    reps: 0,
    file: [0; 128],
    length: 0,
    nomap: 0 as *const LINES as *mut LINES,
    debug: boolean_false,
    chunked: boolean_false,
    unique: boolean_false,
    get: boolean_false,
    print: boolean_false,
    mark: boolean_false,
    markstr: 0 as *const libc::c_char as *mut libc::c_char,
    protocol: 0,
    cookies: 0 as *const COOKIES_T as *mut COOKIES_T,
    uagent: [0; 256],
    encoding: [0; 256],
    conttype: [0; 256],
    bids: 0,
    auth: 0 as *const AUTH_T as *mut AUTH_T,
    keepalive: boolean_false,
    signaled: 0,
    extra: [0; 2048],
    login: boolean_false,
    loginurl: 0 as *const libc::c_char as *mut libc::c_char,
    lurl: 0 as *const ARRAY_T as *mut ARRAY_T,
    failures: 0,
    failed: 0,
    escape: boolean_false,
    expire: boolean_false,
    follow: boolean_false,
    zero_ok: boolean_false,
    spinner: boolean_false,
    cache: boolean_false,
    rc: [0; 256],
    ssl_timeout: 0,
    ssl_cert: 0 as *const libc::c_char as *mut libc::c_char,
    ssl_key: 0 as *const libc::c_char as *mut libc::c_char,
    ssl_ciphers: 0 as *const libc::c_char as *mut libc::c_char,
    method: NOMETHOD,
    json_output: boolean_false,
    cond: pthread_cond_t {
        __data: __pthread_cond_s {
            c2rust_unnamed: C2RustUnnamed_1 { __wseq: 0 },
            c2rust_unnamed_0: C2RustUnnamed { __g1_start: 0 },
            __g_refs: [0; 2],
            __g_size: [0; 2],
            __g1_orig_size: 0,
            __wrefs: 0,
            __g_signals: [0; 2],
        },
    },
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
                __prev: 0 as *const __pthread_internal_list
                    as *mut __pthread_internal_list,
                __next: 0 as *const __pthread_internal_list
                    as *mut __pthread_internal_list,
            },
        },
    },
};
static mut long_options: [option; 25] = [
    {
        let mut init = option {
            name: b"version\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'V' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"help\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'h' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"verbose\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'v' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"quiet\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'q' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"config\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'C' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"debug\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'D' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"get\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'g' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"print\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'p' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"concurrent\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'c' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"no-parser\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'N' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"no-follow\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'F' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"internet\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'i' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"benchmark\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'b' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"reps\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'r' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"time\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 't' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"delay\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'd' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"log\0" as *const u8 as *const libc::c_char,
            has_arg: 2 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'l' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"file\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'f' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"rc\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'R' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"mark\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'm' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"header\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'H' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"user-agent\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'A' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"content-type\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'T' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"json-output\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'j' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: 0 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
];
pub unsafe extern "C" fn display_version(mut b: BOOLEAN) {
    let mut name: [libc::c_char; 128] = [0; 128];
    memset(
        name.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
    );
    strncpy(name.as_mut_ptr(), program_name, strlen(program_name));
    if my.debug as u64 != 0 {
        fprintf(
            stderr,
            b"%s %s: debugging enabled\n\n%s\n\0" as *const u8 as *const libc::c_char,
            uppercase(name.as_mut_ptr(), strlen(name.as_mut_ptr())),
            version_string,
            copyright,
        );
    } else if b as libc::c_uint == boolean_true as libc::c_int as libc::c_uint {
        fprintf(
            stderr,
            b"%s %s\n\n%s\n\0" as *const u8 as *const libc::c_char,
            uppercase(name.as_mut_ptr(), strlen(name.as_mut_ptr())),
            version_string,
            copyright,
        );
        exit(0 as libc::c_int);
    } else {
        fprintf(
            stderr,
            b"%s %s\n\0" as *const u8 as *const libc::c_char,
            uppercase(name.as_mut_ptr(), strlen(name.as_mut_ptr())),
            version_string,
        );
    };
}
pub unsafe extern "C" fn display_help() {
    display_version(boolean_false);
    printf(b"Usage: %s [options]\n\0" as *const u8 as *const libc::c_char, program_name);
    printf(
        b"       %s [options] URL\n\0" as *const u8 as *const libc::c_char,
        program_name,
    );
    printf(b"       %s -g URL\n\0" as *const u8 as *const libc::c_char, program_name);
    printf(b"Options:\n\0" as *const u8 as *const libc::c_char);
    puts(
        b"  -V, --version             VERSION, prints the version number.\0" as *const u8
            as *const libc::c_char,
    );
    puts(
        b"  -h, --help                HELP, prints this section.\0" as *const u8
            as *const libc::c_char,
    );
    puts(
        b"  -C, --config              CONFIGURATION, show the current config.\0"
            as *const u8 as *const libc::c_char,
    );
    puts(
        b"  -v, --verbose             VERBOSE, prints notification to screen.\0"
            as *const u8 as *const libc::c_char,
    );
    puts(
        b"  -q, --quiet               QUIET turns verbose off and suppresses output.\0"
            as *const u8 as *const libc::c_char,
    );
    puts(
        b"  -g, --get                 GET, pull down HTTP headers and display the\0"
            as *const u8 as *const libc::c_char,
    );
    puts(
        b"                            transaction. Great for application debugging.\0"
            as *const u8 as *const libc::c_char,
    );
    puts(
        b"  -p, --print               PRINT, like GET only it prints the entire page.\0"
            as *const u8 as *const libc::c_char,
    );
    puts(
        b"  -c, --concurrent=NUM      CONCURRENT users, default is 10\0" as *const u8
            as *const libc::c_char,
    );
    puts(
        b"  -r, --reps=NUM            REPS, number of times to run the test.\0"
            as *const u8 as *const libc::c_char,
    );
    puts(
        b"  -t, --time=NUMm           TIMED testing where \"m\" is modifier S, M, or H\0"
            as *const u8 as *const libc::c_char,
    );
    puts(
        b"                            ex: --time=1H, one hour test.\0" as *const u8
            as *const libc::c_char,
    );
    puts(
        b"  -d, --delay=NUM           Time DELAY, random delay before each request\0"
            as *const u8 as *const libc::c_char,
    );
    puts(
        b"  -b, --benchmark           BENCHMARK: no delays between requests.\0"
            as *const u8 as *const libc::c_char,
    );
    puts(
        b"  -i, --internet            INTERNET user simulation, hits URLs randomly.\0"
            as *const u8 as *const libc::c_char,
    );
    puts(
        b"  -f, --file=FILE           FILE, select a specific URLS FILE.\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"  -R, --rc=FILE             RC, specify an %src file\n\0" as *const u8
            as *const libc::c_char,
        program_name,
    );
    puts(
        b"  -l, --log[=FILE]          LOG to FILE. If FILE is not specified, the\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"                            default is used: PREFIX/var/%s.log\n\0"
            as *const u8 as *const libc::c_char,
        program_name,
    );
    puts(
        b"  -m, --mark=\"text\"         MARK, mark the log file with a string.\0"
            as *const u8 as *const libc::c_char,
    );
    puts(
        b"                            between .001 and NUM. (NOT COUNTED IN STATS)\0"
            as *const u8 as *const libc::c_char,
    );
    puts(
        b"  -H, --header=\"text\"       Add a header to request (can be many)\0"
            as *const u8 as *const libc::c_char,
    );
    puts(
        b"  -A, --user-agent=\"text\"   Sets User-Agent in request\0" as *const u8
            as *const libc::c_char,
    );
    puts(
        b"  -T, --content-type=\"text\" Sets Content-Type in request\0" as *const u8
            as *const libc::c_char,
    );
    puts(
        b"  -j, --json-output         JSON OUTPUT, print final stats to stdout as JSON\0"
            as *const u8 as *const libc::c_char,
    );
    puts(
        b"      --no-parser           NO PARSER, turn off the HTML page parser\0"
            as *const u8 as *const libc::c_char,
    );
    puts(
        b"      --no-follow           NO FOLLOW, do not follow HTTP redirects\0"
            as *const u8 as *const libc::c_char,
    );
    puts(b"\0" as *const u8 as *const libc::c_char);
    puts(copyright);
    exit(0 as libc::c_int);
}
pub unsafe extern "C" fn parse_rc_cmdline(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) {
    let mut a: libc::c_int = 0 as libc::c_int;
    strcpy((my.rc).as_mut_ptr(), b"\0" as *const u8 as *const libc::c_char);
    while a > -(1 as libc::c_int) {
        a = getopt_long(
            argc,
            argv as *const *mut libc::c_char,
            b"VhvqCDNFpgl::ibr:t:f:d:c:m:H:R:A:T:j\0" as *const u8
                as *const libc::c_char,
            long_options.as_mut_ptr(),
            0 as *mut libc::c_int,
        );
        if a == 'R' as i32 {
            strcpy((my.rc).as_mut_ptr(), optarg);
            a = -(1 as libc::c_int);
        }
    }
    optind = 0 as libc::c_int;
}
pub unsafe extern "C" fn parse_cmdline(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) {
    let mut c: libc::c_int = 0 as libc::c_int;
    let mut nargs: libc::c_int = 0;
    loop {
        c = getopt_long(
            argc,
            argv as *const *mut libc::c_char,
            b"VhvqCDNFpgl::ibr:t:f:d:c:m:H:R:A:T:j\0" as *const u8
                as *const libc::c_char,
            long_options.as_mut_ptr(),
            0 as *mut libc::c_int,
        );
        if !(c != -(1 as libc::c_int)) {
            break;
        }
        match c {
            86 => {
                display_version(boolean_true);
            }
            104 => {
                display_help();
                exit(0 as libc::c_int);
            }
            68 => {
                my.debug = boolean_true;
            }
            67 => {
                my.config = boolean_true;
                my.get = boolean_false;
            }
            99 => {
                my.cusers = atoi(optarg);
            }
            105 => {
                my.internet = boolean_true;
            }
            98 => {
                my.bench = boolean_true;
            }
            100 => {
                my.delay = atof(optarg) as libc::c_float;
                if my.delay < 0 as libc::c_int as libc::c_float {
                    my.delay = 0 as libc::c_int as libc::c_float;
                }
            }
            103 => {
                my.get = boolean_true;
            }
            112 => {
                my.print = boolean_true;
                my.cusers = 1 as libc::c_int;
                my.reps = 1 as libc::c_int;
            }
            108 => {
                my.logging = boolean_true;
                if !optarg.is_null() {
                    my.logfile[strlen(optarg) as usize] = '\0' as i32 as libc::c_char;
                    strncpy((my.logfile).as_mut_ptr(), optarg, strlen(optarg));
                }
            }
            109 => {
                my.mark = boolean_true;
                my.markstr = optarg;
                my.logging = boolean_true;
            }
            113 => {
                my.quiet = boolean_true;
            }
            118 => {
                my.verbose = boolean_true;
            }
            114 => {
                if strmatch(
                    optarg,
                    b"once\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ) as u64 != 0
                {
                    my.reps = -(1 as libc::c_int);
                } else {
                    my.reps = atoi(optarg);
                }
            }
            116 => {
                parse_time(optarg);
            }
            102 => {
                memset(
                    (my.file).as_mut_ptr() as *mut libc::c_void,
                    0 as libc::c_int,
                    ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
                );
                if !optarg.is_null() {
                    strncpy((my.file).as_mut_ptr(), optarg, strlen(optarg));
                }
            }
            65 => {
                strncpy(
                    (my.uagent).as_mut_ptr(),
                    optarg,
                    255 as libc::c_int as libc::c_ulong,
                );
            }
            84 => {
                strncpy(
                    (my.conttype).as_mut_ptr(),
                    optarg,
                    255 as libc::c_int as libc::c_ulong,
                );
            }
            78 => {
                my.parser = boolean_false;
            }
            70 => {
                my.follow = boolean_false;
            }
            72 => {
                if (strchr(optarg, ':' as i32)).is_null() {
                    NOTIFY(
                        FATAL,
                        b"no ':' in http-header\0" as *const u8 as *const libc::c_char,
                    );
                }
                if (strlen(optarg))
                    .wrapping_add(strlen((my.extra).as_mut_ptr()))
                    .wrapping_add(3 as libc::c_int as libc::c_ulong)
                    > 2048 as libc::c_int as libc::c_ulong
                {
                    NOTIFY(
                        FATAL,
                        b"header is too large\0" as *const u8 as *const libc::c_char,
                    );
                }
                strcat((my.extra).as_mut_ptr(), optarg);
                strcat(
                    (my.extra).as_mut_ptr(),
                    b"\r\n\0" as *const u8 as *const libc::c_char,
                );
            }
            106 => {
                my.json_output = boolean_true;
            }
            82 | _ => {}
        }
    }
    nargs = argc - optind;
    if nargs != 0 {
        my.url = xstrdup(*argv.offset((argc - 1 as libc::c_int) as isize));
    }
    if my.get as libc::c_uint != 0 && (my.url).is_null() {
        puts(
            b"ERROR: -g/--get requires a commandline URL\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
}
unsafe extern "C" fn __signal_setup() {
    let mut sigs: sigset_t = sigset_t { __val: [0; 16] };
    sigemptyset(&mut sigs);
    sigaddset(&mut sigs, 1 as libc::c_int);
    sigaddset(&mut sigs, 2 as libc::c_int);
    sigaddset(&mut sigs, 14 as libc::c_int);
    sigaddset(&mut sigs, 15 as libc::c_int);
    sigaddset(&mut sigs, 13 as libc::c_int);
    sigprocmask(0 as libc::c_int, &mut sigs, 0 as *mut sigset_t);
}
unsafe extern "C" fn __config_setup(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) {
    memset(
        &mut my as *mut CONFIG as *mut libc::c_void,
        '\0' as i32,
        ::std::mem::size_of::<CONFIG>() as libc::c_ulong,
    );
    parse_rc_cmdline(argc, argv);
    if init_config() < 0 as libc::c_int {
        exit(1 as libc::c_int);
    }
    parse_cmdline(argc, argv);
    ds_module_check();
    if my.config as u64 != 0 {
        show_config(boolean_true as libc::c_int);
    }
    if my.cusers > my.limit {
        printf(b"\n\0" as *const u8 as *const libc::c_char);
        printf(
            b"================================================================\n\0"
                as *const u8 as *const libc::c_char,
        );
        printf(
            b"WARNING: The number of users is capped at %d.%sTo increase this\n\0"
                as *const u8 as *const libc::c_char,
            my.limit,
            if my.limit > 999 as libc::c_int {
                b" \0" as *const u8 as *const libc::c_char
            } else {
                b"  \0" as *const u8 as *const libc::c_char
            },
        );
        printf(
            b"         limit, search your .siegerc file for 'limit' and change\n\0"
                as *const u8 as *const libc::c_char,
        );
        printf(
            b"         its value. Make sure you read the instructions there...\n\0"
                as *const u8 as *const libc::c_char,
        );
        printf(
            b"================================================================\n\0"
                as *const u8 as *const libc::c_char,
        );
        sleep(10 as libc::c_int as libc::c_uint);
        my.cusers = my.limit;
    }
}
unsafe extern "C" fn __urls_setup() -> *mut LINES {
    let mut lines: *mut LINES = 0 as *mut LINES;
    lines = xcalloc(
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<LINES>() as libc::c_ulong,
    ) as *mut LINES;
    (*lines).index = 0 as libc::c_int;
    (*lines).line = 0 as *mut *mut libc::c_char;
    if !(my.url).is_null() {
        my.length = 1 as libc::c_int;
    } else {
        my.length = read_cfg_file(lines, (my.file).as_mut_ptr());
    }
    if my.length == 0 as libc::c_int {
        display_help();
    }
    return lines;
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut result: libc::c_int = 0 as libc::c_int;
    let mut status: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut lines: *mut LINES = 0 as *mut LINES;
    let mut crew: CREW = 0 as CREW;
    let mut data: DATA = 0 as DATA;
    let mut cookies: HASH = 0 as HASH;
    let mut urls: ARRAY = new_array();
    let mut browsers: ARRAY = new_array();
    let mut cease: pthread_t = 0;
    let mut timer: pthread_t = 0;
    let mut scope_attr: pthread_attr_t = pthread_attr_t { __size: [0; 56] };
    __signal_setup();
    __config_setup(argc, argv);
    lines = __urls_setup();
    pthread_attr_init(&mut scope_attr);
    pthread_attr_setscope(&mut scope_attr, PTHREAD_SCOPE_SYSTEM as libc::c_int);
    SSL_thread_setup();
    if !(my.url).is_null() {
        let mut tmp: URL = new_url(my.url);
        url_set_ID(tmp, 0 as libc::c_int);
        if my.get as libc::c_uint != 0
            && url_get_method(tmp) as libc::c_uint != POST as libc::c_int as libc::c_uint
            && url_get_method(tmp) as libc::c_uint != PUT as libc::c_int as libc::c_uint
        {
            url_set_method(tmp, my.method);
        }
        array_npush(urls, tmp as *mut libc::c_void, URLSIZE);
    } else {
        i = 0 as libc::c_int;
        while i < my.length {
            let mut tmp_0: URL = new_url(*((*lines).line).offset(i as isize));
            url_set_ID(tmp_0, i);
            array_npush(urls, tmp_0 as *mut libc::c_void, URLSIZE);
            i += 1;
            i;
        }
    }
    cookies = load_cookies(my.cookies);
    i = 0 as libc::c_int;
    while i < my.cusers {
        let mut tmp_1: [libc::c_char; 4096] = [0; 4096];
        let mut B: BROWSER = new_browser(i);
        memset(
            tmp_1.as_mut_ptr() as *mut libc::c_void,
            '\0' as i32,
            ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong,
        );
        snprintf(
            tmp_1.as_mut_ptr(),
            4096 as libc::c_int as libc::c_ulong,
            b"%d\0" as *const u8 as *const libc::c_char,
            i,
        );
        if !cookies.is_null() {
            if !(hash_get(cookies, tmp_1.as_mut_ptr())).is_null() {
                browser_set_cookies(B, hash_get(cookies, tmp_1.as_mut_ptr()) as HASH);
            }
        }
        if my.reps > 0 as libc::c_int {
            browser_set_urls(B, urls);
        } else {
            let mut n_urls: libc::c_int = array_length(urls) as libc::c_int;
            let mut per_user: libc::c_int = n_urls / my.cusers;
            let mut remainder: libc::c_int = n_urls % my.cusers;
            let mut begin_url: libc::c_int = i * per_user
                + (if i < remainder { i } else { remainder });
            let mut end_url: libc::c_int = (i + 1 as libc::c_int) * per_user
                + (if i < remainder { i + 1 as libc::c_int } else { remainder });
            let mut url_slice: ARRAY = new_array();
            j = begin_url;
            while j < end_url && j < n_urls {
                let mut u: URL = array_get(urls, j) as URL;
                if !u.is_null() && !(url_get_hostname(u)).is_null()
                    && strlen(url_get_hostname(u)) > 1 as libc::c_int as libc::c_ulong
                {
                    array_npush(url_slice, u as *mut libc::c_void, URLSIZE);
                }
                j += 1;
                j;
            }
            browser_set_urls(B, url_slice);
        }
        array_npush(browsers, B as *mut libc::c_void, BROWSERSIZE);
        i += 1;
        i;
    }
    crew = new_crew(my.cusers, my.cusers, boolean_false);
    if crew.is_null() {
        NOTIFY(
            FATAL,
            b"unable to allocate memory for %d simulated browser\0" as *const u8
                as *const libc::c_char,
            my.cusers,
        );
    }
    result = pthread_create(
        &mut cease,
        0 as *const pthread_attr_t,
        ::std::mem::transmute::<
            *mut libc::c_void,
            Option::<unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void>,
        >(
            ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(CREW) -> ()>,
                *mut libc::c_void,
            >(Some(sig_handler as unsafe extern "C" fn(CREW) -> ())),
        ),
        crew as *mut libc::c_void,
    );
    if result < 0 as libc::c_int {
        NOTIFY(
            FATAL,
            b"failed to create handler: %d\n\0" as *const u8 as *const libc::c_char,
            result,
        );
    }
    if my.secs > 0 as libc::c_int {
        result = pthread_create(
            &mut timer,
            0 as *const pthread_attr_t,
            ::std::mem::transmute::<
                *mut libc::c_void,
                Option::<unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void>,
            >(
                ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(pthread_t) -> ()>,
                    *mut libc::c_void,
                >(Some(siege_timer as unsafe extern "C" fn(pthread_t) -> ())),
            ),
            cease as *mut libc::c_void,
        );
        if result < 0 as libc::c_int {
            NOTIFY(
                FATAL,
                b"failed to create handler: %d\n\0" as *const u8 as *const libc::c_char,
                result,
            );
        }
    }
    if my.get as u64 == 0 && my.quiet as u64 == 0 {
        fprintf(stderr, b"** \0" as *const u8 as *const libc::c_char);
        display_version(boolean_false);
        fprintf(
            stderr,
            b"** Preparing %d concurrent users for battle.\n\0" as *const u8
                as *const libc::c_char,
            my.cusers,
        );
        fprintf(
            stderr,
            b"The server is now under siege...\0" as *const u8 as *const libc::c_char,
        );
        if my.verbose as u64 != 0 {
            fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
        }
    }
    data = new_data();
    data_set_start(data);
    i = 0 as libc::c_int;
    while i < my.cusers
        && crew_get_shutdown(crew) as libc::c_uint
            != boolean_true as libc::c_int as libc::c_uint
    {
        let mut B_0: BROWSER = array_get(browsers, i) as BROWSER;
        result = crew_add(
            crew,
            ::std::mem::transmute::<
                *mut libc::c_void,
                Option::<unsafe extern "C" fn() -> ()>,
            >(
                ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(BROWSER) -> *mut libc::c_void>,
                    *mut libc::c_void,
                >(Some(start as unsafe extern "C" fn(BROWSER) -> *mut libc::c_void)),
            ),
            B_0 as *mut libc::c_void,
        ) as libc::c_int;
        if result == boolean_false as libc::c_int {
            my.verbose = boolean_false;
            fprintf(
                stderr,
                b"Unable to spawn additional threads; you may need to\n\0" as *const u8
                    as *const libc::c_char,
            );
            fprintf(
                stderr,
                b"upgrade your libraries or tune your system in order\n\0" as *const u8
                    as *const libc::c_char,
            );
            fprintf(
                stderr,
                b"to exceed %d users.\n\0" as *const u8 as *const libc::c_char,
                my.cusers,
            );
            NOTIFY(
                FATAL,
                b"system resources exhausted\0" as *const u8 as *const libc::c_char,
            );
        }
        i += 1;
        i;
    }
    crew_join(crew, boolean_true, &mut status);
    data_set_stop(data);
    SSL_thread_cleanup();
    i = 0 as libc::c_int;
    while i
        < (if crew_get_total(crew) > my.cusers
            || crew_get_total(crew) == 0 as libc::c_int
        {
            my.cusers
        } else {
            crew_get_total(crew)
        })
    {
        let mut B_1: BROWSER = array_get(browsers, i) as BROWSER;
        data_increment_count(data, browser_get_hits(B_1));
        data_increment_bytes(data, browser_get_bytes(B_1) as libc::c_ulong);
        data_increment_total(data, browser_get_time(B_1));
        data_increment_code(data, browser_get_code(B_1) as libc::c_int);
        data_increment_okay(data, browser_get_okay(B_1) as libc::c_int);
        data_increment_fail(data, browser_get_fail(B_1) as libc::c_int);
        data_set_highest(data, browser_get_himark(B_1));
        data_set_lowest(data, browser_get_lomark(B_1));
        i += 1;
        i;
    }
    crew_destroy(crew);
    pthread_usleep_np(10000 as libc::c_int as libc::c_ulong);
    if my.quiet as u64 == 0 {
        if my.failures > 0 as libc::c_int && my.failed >= my.failures {
            fprintf(
                stderr,
                b"%s aborted due to excessive socket failure; you\n\0" as *const u8
                    as *const libc::c_char,
                program_name,
            );
            fprintf(
                stderr,
                b"can change the failure threshold in $HOME/.%src\n\0" as *const u8
                    as *const libc::c_char,
                program_name,
            );
        }
        fprintf(
            stderr,
            b"\nTransactions:\t\t%12u hits\n\0" as *const u8 as *const libc::c_char,
            data_get_count(data),
        );
        fprintf(
            stderr,
            b"Availability:\t\t%12.2f %%\n\0" as *const u8 as *const libc::c_char,
            if data_get_count(data) == 0 as libc::c_int as libc::c_uint {
                0 as libc::c_int as libc::c_double
            } else {
                data_get_count(data) as libc::c_double
                    / (data_get_count(data)).wrapping_add(my.failed as libc::c_uint)
                        as libc::c_double * 100 as libc::c_int as libc::c_double
            },
        );
        fprintf(
            stderr,
            b"Elapsed time:\t\t%12.2f secs\n\0" as *const u8 as *const libc::c_char,
            data_get_elapsed(data) as libc::c_double,
        );
        fprintf(
            stderr,
            b"Data transferred:\t%12.2f MB\n\0" as *const u8 as *const libc::c_char,
            data_get_megabytes(data) as libc::c_double,
        );
        fprintf(
            stderr,
            b"Response time:\t\t%12.2f secs\n\0" as *const u8 as *const libc::c_char,
            data_get_response_time(data) as libc::c_double,
        );
        fprintf(
            stderr,
            b"Transaction rate:\t%12.2f trans/sec\n\0" as *const u8
                as *const libc::c_char,
            data_get_transaction_rate(data) as libc::c_double,
        );
        fprintf(
            stderr,
            b"Throughput:\t\t%12.2f MB/sec\n\0" as *const u8 as *const libc::c_char,
            data_get_throughput(data) as libc::c_double,
        );
        fprintf(
            stderr,
            b"Concurrency:\t\t%12.2f\n\0" as *const u8 as *const libc::c_char,
            data_get_concurrency(data) as libc::c_double,
        );
        fprintf(
            stderr,
            b"Successful transactions:%12u\n\0" as *const u8 as *const libc::c_char,
            data_get_code(data),
        );
        if my.debug as u64 != 0 {
            fprintf(
                stderr,
                b"HTTP OK received:\t%12u\n\0" as *const u8 as *const libc::c_char,
                data_get_okay(data),
            );
        }
        fprintf(
            stderr,
            b"Failed transactions:\t%12u\n\0" as *const u8 as *const libc::c_char,
            my.failed,
        );
        fprintf(
            stderr,
            b"Longest transaction:\t%12.2f\n\0" as *const u8 as *const libc::c_char,
            data_get_highest(data) as libc::c_double,
        );
        fprintf(
            stderr,
            b"Shortest transaction:\t%12.2f\n\0" as *const u8 as *const libc::c_char,
            data_get_lowest(data) as libc::c_double,
        );
        fprintf(stderr, b" \n\0" as *const u8 as *const libc::c_char);
    }
    if my.json_output as u64 != 0 {
        fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
        printf(b"{\n\0" as *const u8 as *const libc::c_char);
        printf(
            b"\t\"transactions\":\t\t\t%12u,\n\0" as *const u8 as *const libc::c_char,
            data_get_count(data),
        );
        let mut availability: libc::c_double = 0.;
        if data_get_count(data) == 0 as libc::c_int as libc::c_uint {
            availability = 0 as libc::c_int as libc::c_double;
        } else {
            availability = data_get_count(data) as libc::c_double
                / (data_get_count(data)).wrapping_add(my.failed as libc::c_uint)
                    as libc::c_double * 100 as libc::c_int as libc::c_double;
        }
        printf(
            b"\t\"availability\":\t\t\t%12.2f,\n\0" as *const u8 as *const libc::c_char,
            availability,
        );
        printf(
            b"\t\"elapsed_time\":\t\t\t%12.2f,\n\0" as *const u8 as *const libc::c_char,
            data_get_elapsed(data) as libc::c_double,
        );
        printf(
            b"\t\"data_transferred\":\t\t%12.2f,\n\0" as *const u8
                as *const libc::c_char,
            data_get_megabytes(data) as libc::c_double,
        );
        printf(
            b"\t\"response_time\":\t\t%12.2f,\n\0" as *const u8 as *const libc::c_char,
            data_get_response_time(data) as libc::c_double,
        );
        printf(
            b"\t\"transaction_rate\":\t\t%12.2f,\n\0" as *const u8
                as *const libc::c_char,
            data_get_transaction_rate(data) as libc::c_double,
        );
        printf(
            b"\t\"throughput\":\t\t\t%12.2f,\n\0" as *const u8 as *const libc::c_char,
            data_get_throughput(data) as libc::c_double,
        );
        printf(
            b"\t\"concurrency\":\t\t\t%12.2f,\n\0" as *const u8 as *const libc::c_char,
            data_get_concurrency(data) as libc::c_double,
        );
        printf(
            b"\t\"successful_transactions\":\t%12u,\n\0" as *const u8
                as *const libc::c_char,
            data_get_code(data),
        );
        if my.debug as u64 != 0 {
            printf(
                b"\t\"http_ok_received\":\t\t%12u,\n\0" as *const u8
                    as *const libc::c_char,
                data_get_okay(data),
            );
        }
        printf(
            b"\t\"failed_transactions\":\t\t%12u,\n\0" as *const u8
                as *const libc::c_char,
            my.failed,
        );
        printf(
            b"\t\"longest_transaction\":\t\t%12.2f,\n\0" as *const u8
                as *const libc::c_char,
            data_get_highest(data) as libc::c_double,
        );
        printf(
            b"\t\"shortest_transaction\":\t\t%12.2f\n\0" as *const u8
                as *const libc::c_char,
            data_get_lowest(data) as libc::c_double,
        );
        puts(b"}\0" as *const u8 as *const libc::c_char);
    }
    if my.mark as u64 != 0 {
        mark_log_file(my.markstr);
    }
    if my.logging as u64 != 0 {
        log_transaction(data);
        if my.failures > 0 as libc::c_int && my.failed >= my.failures {
            mark_log_file(
                b"siege aborted due to excessive socket failure.\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            );
        }
    }
    data = data_destroy(data);
    urls = array_destroyer(
        urls,
        ::std::mem::transmute::<
            *mut libc::c_void,
            method,
        >(
            ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(URL) -> URL>,
                *mut libc::c_void,
            >(Some(url_destroy as unsafe extern "C" fn(URL) -> URL)),
        ),
    );
    browsers = array_destroyer(
        browsers,
        ::std::mem::transmute::<
            *mut libc::c_void,
            method,
        >(
            ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(BROWSER) -> BROWSER>,
                *mut libc::c_void,
            >(Some(browser_destroy as unsafe extern "C" fn(BROWSER) -> BROWSER)),
        ),
    );
    cookies = hash_destroy(cookies);
    my.cookies = cookies_destroy(my.cookies);
    if (my.url).is_null() {
        i = 0 as libc::c_int;
        while i < my.length {
            xfree(*((*lines).line).offset(i as isize) as *mut libc::c_void);
            i += 1;
            i;
        }
        xfree((*lines).line as *mut libc::c_void);
        xfree(lines as *mut libc::c_void);
    } else {
        xfree((*lines).line as *mut libc::c_void);
        xfree(lines as *mut libc::c_void);
    }
    exit(0 as libc::c_int);
}
pub fn main() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args_os() {
        args.push(
            (::std::ffi::CString::new(
                ::std::os::unix::ffi::OsStrExt::as_bytes(arg.as_os_str()),
            ))
                .unwrap()
                .into_raw(),
        );
    }
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as libc::c_int,
                args.as_mut_ptr() as *mut *mut libc::c_char,
            ) as i32,
        )
    }
}
