use ::libc;
extern "C" {
    pub type ARRAY_T;
    pub type AUTH_T;
    pub type COOKIES_T;
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type HASH_T;
    pub type CREDS_T;
    fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_double;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn system(__command: *const libc::c_char) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    static mut my: CONFIG;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn hash_destroy(this: HASH) -> HASH;
    fn hash_add(this: HASH, key: *mut libc::c_char, value: *mut libc::c_void);
    fn strncasecmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn new_creds(scheme: SCHEME, str: *mut libc::c_char) -> CREDS;
    fn auth_add(this: AUTH, creds: CREDS);
    fn auth_set_proxy_port(this: AUTH, port: libc::c_int);
    fn auth_set_proxy_host(this: AUTH, host: *mut libc::c_char);
    fn array_push(this: ARRAY, thing: *mut libc::c_void);
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn fgetc(__stream: *mut FILE) -> libc::c_int;
    fn new_hash() -> HASH;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn pthread_cond_init(
        __cond: *mut pthread_cond_t,
        __cond_attr: *const pthread_condattr_t,
    ) -> libc::c_int;
    fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> libc::c_int;
    fn new_cookies() -> COOKIES;
    fn new_array() -> ARRAY;
    fn auth_set_proxy_required(this: AUTH, required: BOOLEAN);
    fn new_auth() -> AUTH;
    fn __errno_location() -> *mut libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    fn cookies_destroy(this: COOKIES) -> COOKIES;
    fn array_destroy(this: ARRAY) -> ARRAY;
    fn auth_destroy(this: AUTH) -> AUTH;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn auth_display(this: AUTH, scheme: SCHEME);
    fn auth_get_proxy_port(this: AUTH) -> libc::c_int;
    fn auth_get_proxy_host(this: AUTH) -> *mut libc::c_char;
    fn auth_get_proxy_required(this: AUTH) -> BOOLEAN;
    fn parse_time(p: *mut libc::c_char);
    fn strmatch(str1: *mut libc::c_char, str2: *mut libc::c_char) -> BOOLEAN;
    fn evaluate(hash: HASH, buf: *mut libc::c_char) -> *mut libc::c_char;
    fn trim(str: *mut libc::c_char) -> *mut libc::c_char;
    fn xstrdup(str: *const libc::c_char) -> *mut libc::c_char;
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    fn xcalloc(_: size_t, _: size_t) -> *mut libc::c_void;
    fn xfree(ptr: *mut libc::c_void);
    fn NOTIFY(L: LEVEL, fmt: *const libc::c_char, _: ...);
    fn stralloc(_: *mut libc::c_char) -> *mut libc::c_char;
    static mut version_string: *const libc::c_char;
    fn mkdir(__path: *const libc::c_char, __mode: __mode_t) -> libc::c_int;
    fn __xstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
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
pub union pthread_mutexattr_t {
    pub __size: [libc::c_char; 4],
    pub __align: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_condattr_t {
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
pub type ARRAY = *mut ARRAY_T;
pub type AUTH = *mut AUTH_T;
pub type COOKIES = *mut COOKIES_T;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LINES {
    pub index: libc::c_int,
    pub line: *mut *mut libc::c_char,
}
pub type FILE = _IO_FILE;
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
pub type HASH = *mut HASH_T;
pub type LEVEL = libc::c_uint;
pub const FATAL: LEVEL = 3;
pub const ERROR: LEVEL = 2;
pub const WARNING: LEVEL = 1;
pub const DEBUG: LEVEL = 0;
pub type CREDS = *mut CREDS_T;
pub type SCHEME = libc::c_uint;
pub const PROXY: SCHEME = 4;
pub const FTP: SCHEME = 3;
pub const HTTPS: SCHEME = 2;
pub const HTTP: SCHEME = 1;
pub const UNSUPPORTED: SCHEME = 0;
pub const _ISspace: C2RustUnnamed_3 = 8192;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: libc::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
pub type C2RustUnnamed_3 = libc::c_uint;
pub const _ISalnum: C2RustUnnamed_3 = 8;
pub const _ISpunct: C2RustUnnamed_3 = 4;
pub const _IScntrl: C2RustUnnamed_3 = 2;
pub const _ISblank: C2RustUnnamed_3 = 1;
pub const _ISgraph: C2RustUnnamed_3 = 32768;
pub const _ISprint: C2RustUnnamed_3 = 16384;
pub const _ISxdigit: C2RustUnnamed_3 = 4096;
pub const _ISdigit: C2RustUnnamed_3 = 2048;
pub const _ISalpha: C2RustUnnamed_3 = 1024;
pub const _ISlower: C2RustUnnamed_3 = 512;
pub const _ISupper: C2RustUnnamed_3 = 256;
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
#[inline]
unsafe extern "C" fn stat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __xstat(1 as libc::c_int, __path, __statbuf);
}
pub unsafe extern "C" fn init_config() -> libc::c_int {
    let mut e: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut dir: [libc::c_char; 256] = [0; 256];
    let mut res: libc::c_int = 0;
    let mut buf: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    let mut needed: BOOLEAN = boolean_false;
    memset(
        dir.as_mut_ptr() as *mut libc::c_void,
        '\0' as i32,
        ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
    );
    snprintf(
        dir.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
        b"%s/.siege\0" as *const u8 as *const libc::c_char,
        getenv(b"HOME\0" as *const u8 as *const libc::c_char),
    );
    if stat(dir.as_mut_ptr(), &mut buf) < 0 as libc::c_int
        && 2 as libc::c_int == *__errno_location()
    {
        needed = boolean_true;
    } else if !(buf.st_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o40000 as libc::c_int as libc::c_uint)
    {
        if unlink(dir.as_mut_ptr()) == 0 as libc::c_int {
            needed = boolean_true;
        }
    }
    while needed as u64 != 0 {
        let mut ret: libc::c_int = 0;
        mkdir(dir.as_mut_ptr(), 0o750 as libc::c_int as __mode_t);
        ret = system(b"siege.config\0" as *const u8 as *const libc::c_char);
        if ((ret & 0x7f as libc::c_int) + 1 as libc::c_int) as libc::c_schar
            as libc::c_int >> 1 as libc::c_int > 0 as libc::c_int
            && (ret & 0x7f as libc::c_int == 2 as libc::c_int
                || ret & 0x7f as libc::c_int == 3 as libc::c_int)
        {
            break;
        }
        needed = boolean_false;
    }
    if strcmp((my.rc).as_mut_ptr(), b"\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        e = getenv(b"SIEGERC\0" as *const u8 as *const libc::c_char);
        if !e.is_null() {
            snprintf(
                (my.rc).as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
                b"%s\0" as *const u8 as *const libc::c_char,
                e,
            );
        } else {
            snprintf(
                (my.rc).as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
                b"%s/.siege/siege.conf\0" as *const u8 as *const libc::c_char,
                getenv(b"HOME\0" as *const u8 as *const libc::c_char),
            );
            if stat((my.rc).as_mut_ptr(), &mut buf) < 0 as libc::c_int
                && *__errno_location() == 2 as libc::c_int
            {
                snprintf(
                    (my.rc).as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
                    b"/usr/local/etc/siegerc\0" as *const u8 as *const libc::c_char,
                );
            }
        }
    }
    my.debug = boolean_false;
    my.quiet = boolean_false;
    my.color = boolean_true;
    my.internet = boolean_false;
    my.config = boolean_false;
    my.csv = boolean_false;
    my.fullurl = boolean_false;
    my.escape = boolean_true;
    my.parser = boolean_false;
    my.secs = -(1 as libc::c_int);
    my.limit = 255 as libc::c_int;
    my.reps = 10301062 as libc::c_int;
    my.bids = 5 as libc::c_int;
    my.login = boolean_false;
    my.failures = 1024 as libc::c_int;
    my.failed = 0 as libc::c_int;
    my.auth = new_auth();
    auth_set_proxy_required(my.auth, boolean_false);
    auth_set_proxy_port(my.auth, 3128 as libc::c_int);
    my.timeout = 30 as libc::c_int;
    my.timestamp = boolean_false;
    my.chunked = boolean_false;
    my.unique = boolean_true;
    my.json_output = boolean_false;
    my.extra[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    my.follow = boolean_true;
    my.zero_ok = boolean_true;
    my.signaled = 0 as libc::c_int;
    my.ssl_timeout = 300 as libc::c_int;
    my.ssl_cert = 0 as *mut libc::c_char;
    my.ssl_key = 0 as *mut libc::c_char;
    my.ssl_ciphers = 0 as *mut libc::c_char;
    my.lurl = new_array();
    my.cookies = new_cookies();
    my
        .nomap = xcalloc(
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<LINES>() as libc::c_ulong,
    ) as *mut LINES;
    (*my.nomap).index = 0 as libc::c_int;
    (*my.nomap).line = 0 as *mut *mut libc::c_char;
    res = pthread_mutex_init(&mut my.lock, 0 as *const pthread_mutexattr_t);
    if res != 0 as libc::c_int {
        NOTIFY(FATAL, b"unable to initiate lock\0" as *const u8 as *const libc::c_char);
    }
    res = pthread_cond_init(&mut my.cond, 0 as *const pthread_condattr_t);
    if res != 0 as libc::c_int {
        NOTIFY(
            FATAL,
            b"unable to initiate condition\0" as *const u8 as *const libc::c_char,
        );
    }
    if load_conf((my.rc).as_mut_ptr()) < 0 as libc::c_int {
        fprintf(
            stderr,
            b"**************************************************\n\0" as *const u8
                as *const libc::c_char,
        );
        fprintf(
            stderr,
            b"siege: could not open %s\n\0" as *const u8 as *const libc::c_char,
            (my.rc).as_mut_ptr(),
        );
        fprintf(
            stderr,
            b"run 'siege.config' to generate a new config file\n\0" as *const u8
                as *const libc::c_char,
        );
        fprintf(
            stderr,
            b"**************************************************\n\0" as *const u8
                as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if strlen((my.file).as_mut_ptr()) < 1 as libc::c_int as libc::c_ulong {
        snprintf(
            (my.file).as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
            b"%s\0" as *const u8 as *const libc::c_char,
            b"/usr/local/etc/urls.txt\0" as *const u8 as *const libc::c_char,
        );
    }
    if strlen((my.uagent).as_mut_ptr()) < 1 as libc::c_int as libc::c_ulong {
        snprintf(
            (my.uagent).as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
            b"Mozilla/5.0 (%s) Siege/%s\0" as *const u8 as *const libc::c_char,
            b"pc-x86_64-linux-gnu\0" as *const u8 as *const libc::c_char,
            version_string,
        );
    }
    if strlen((my.conttype).as_mut_ptr()) < 1 as libc::c_int as libc::c_ulong {
        snprintf(
            (my.conttype).as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
            b"application/x-www-form-urlencoded\0" as *const u8 as *const libc::c_char,
        );
    }
    if strlen((my.encoding).as_mut_ptr()) < 1 as libc::c_int as libc::c_ulong {
        snprintf(
            (my.encoding).as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
            b"*\0" as *const u8 as *const libc::c_char,
        );
    }
    if strlen((my.logfile).as_mut_ptr()) < 1 as libc::c_int as libc::c_ulong {
        snprintf(
            (my.logfile).as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
            b"%s\0" as *const u8 as *const libc::c_char,
            b"/usr/local/var/log/siege.log\0" as *const u8 as *const libc::c_char,
        );
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn show_config(mut EXIT: libc::c_int) -> libc::c_int {
    let mut method: *mut libc::c_char = 0 as *mut libc::c_char;
    match my.method as libc::c_uint {
        2 => {
            method = strdup(b"GET\0" as *const u8 as *const libc::c_char);
        }
        1 | _ => {
            method = strdup(b"HEAD\0" as *const u8 as *const libc::c_char);
        }
    }
    printf(b"CURRENT  SIEGE  CONFIGURATION\n\0" as *const u8 as *const libc::c_char);
    printf(b"%s\n\0" as *const u8 as *const libc::c_char, (my.uagent).as_mut_ptr());
    printf(
        b"Edit the resource file to change the settings.\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"----------------------------------------------\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"version:                        %s\n\0" as *const u8 as *const libc::c_char,
        version_string,
    );
    printf(
        b"verbose:                        %s\n\0" as *const u8 as *const libc::c_char,
        if my.verbose as libc::c_uint != 0 {
            b"true\0" as *const u8 as *const libc::c_char
        } else {
            b"false\0" as *const u8 as *const libc::c_char
        },
    );
    printf(
        b"color:                          %s\n\0" as *const u8 as *const libc::c_char,
        if my.color as libc::c_uint != 0 {
            b"true\0" as *const u8 as *const libc::c_char
        } else {
            b"false\0" as *const u8 as *const libc::c_char
        },
    );
    printf(
        b"quiet:                          %s\n\0" as *const u8 as *const libc::c_char,
        if my.quiet as libc::c_uint != 0 {
            b"true\0" as *const u8 as *const libc::c_char
        } else {
            b"false\0" as *const u8 as *const libc::c_char
        },
    );
    printf(
        b"debug:                          %s\n\0" as *const u8 as *const libc::c_char,
        if my.debug as libc::c_uint != 0 {
            b"true\0" as *const u8 as *const libc::c_char
        } else {
            b"false\0" as *const u8 as *const libc::c_char
        },
    );
    printf(
        b"protocol:                       %s\n\0" as *const u8 as *const libc::c_char,
        if my.protocol != 0 {
            b"HTTP/1.1\0" as *const u8 as *const libc::c_char
        } else {
            b"HTTP/1.0\0" as *const u8 as *const libc::c_char
        },
    );
    printf(
        b"HTML parser:                    %s\n\0" as *const u8 as *const libc::c_char,
        if my.parser as libc::c_uint != 0 {
            b"enabled\0" as *const u8 as *const libc::c_char
        } else {
            b"disabled\0" as *const u8 as *const libc::c_char
        },
    );
    printf(
        b"get method:                     %s\n\0" as *const u8 as *const libc::c_char,
        method,
    );
    if auth_get_proxy_required(my.auth) as u64 != 0 {
        printf(
            b"proxy-host:                     %s\n\0" as *const u8
                as *const libc::c_char,
            auth_get_proxy_host(my.auth),
        );
        printf(
            b"proxy-port:                     %d\n\0" as *const u8
                as *const libc::c_char,
            auth_get_proxy_port(my.auth),
        );
    }
    printf(
        b"connection:                     %s\n\0" as *const u8 as *const libc::c_char,
        if my.keepalive as libc::c_uint != 0 {
            b"keep-alive\0" as *const u8 as *const libc::c_char
        } else {
            b"close\0" as *const u8 as *const libc::c_char
        },
    );
    printf(
        b"concurrent users:               %d\n\0" as *const u8 as *const libc::c_char,
        my.cusers,
    );
    if my.secs > 0 as libc::c_int {
        printf(
            b"time to run:                    %d seconds\n\0" as *const u8
                as *const libc::c_char,
            my.secs,
        );
    } else {
        printf(
            b"time to run:                    n/a\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    if my.reps > 0 as libc::c_int && my.reps != 10301062 as libc::c_int {
        printf(
            b"repetitions:                    %d\n\0" as *const u8
                as *const libc::c_char,
            my.reps,
        );
    } else {
        printf(
            b"repetitions:                    n/a\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    printf(
        b"socket timeout:                 %d\n\0" as *const u8 as *const libc::c_char,
        my.timeout,
    );
    printf(
        b"cache enabled:                  %s\n\0" as *const u8 as *const libc::c_char,
        if my.cache as libc::c_uint == boolean_true as libc::c_int as libc::c_uint {
            b"true\0" as *const u8 as *const libc::c_char
        } else {
            b"false\0" as *const u8 as *const libc::c_char
        },
    );
    printf(
        b"accept-encoding:                %s\n\0" as *const u8 as *const libc::c_char,
        (my.encoding).as_mut_ptr(),
    );
    printf(
        b"delay:                          %.3f sec%s\n\0" as *const u8
            as *const libc::c_char,
        my.delay as libc::c_double,
        if my.delay > 1 as libc::c_int as libc::c_float {
            b"s\0" as *const u8 as *const libc::c_char
        } else {
            b"\0" as *const u8 as *const libc::c_char
        },
    );
    printf(
        b"internet simulation:            %s\n\0" as *const u8 as *const libc::c_char,
        if my.internet as libc::c_uint != 0 {
            b"true\0" as *const u8 as *const libc::c_char
        } else {
            b"false\0" as *const u8 as *const libc::c_char
        },
    );
    printf(
        b"benchmark mode:                 %s\n\0" as *const u8 as *const libc::c_char,
        if my.bench as libc::c_uint != 0 {
            b"true\0" as *const u8 as *const libc::c_char
        } else {
            b"false\0" as *const u8 as *const libc::c_char
        },
    );
    printf(
        b"failures until abort:           %d\n\0" as *const u8 as *const libc::c_char,
        my.failures,
    );
    printf(
        b"named URL:                      %s\n\0" as *const u8 as *const libc::c_char,
        if (my.url).is_null() || strlen(my.url) < 2 as libc::c_int as libc::c_ulong {
            b"none\0" as *const u8 as *const libc::c_char
        } else {
            my.url as *const libc::c_char
        },
    );
    printf(
        b"URLs file:                      %s\n\0" as *const u8 as *const libc::c_char,
        if strlen((my.file).as_mut_ptr()) > 1 as libc::c_int as libc::c_ulong {
            (my.file).as_mut_ptr() as *const libc::c_char
        } else {
            b"/usr/local/etc/urls.txt\0" as *const u8 as *const libc::c_char
        },
    );
    printf(
        b"thread limit:                   %d\n\0" as *const u8 as *const libc::c_char,
        if my.limit < 1 as libc::c_int { 255 as libc::c_int } else { my.limit },
    );
    printf(
        b"logging:                        %s\n\0" as *const u8 as *const libc::c_char,
        if my.logging as libc::c_uint != 0 {
            b"true\0" as *const u8 as *const libc::c_char
        } else {
            b"false\0" as *const u8 as *const libc::c_char
        },
    );
    printf(
        b"log file:                       %s\n\0" as *const u8 as *const libc::c_char,
        if (my.logfile).as_mut_ptr().is_null() {
            b"/usr/local/var/log/siege.log\0" as *const u8 as *const libc::c_char
        } else {
            (my.logfile).as_mut_ptr() as *const libc::c_char
        },
    );
    printf(
        b"resource file:                  %s\n\0" as *const u8 as *const libc::c_char,
        (my.rc).as_mut_ptr(),
    );
    printf(
        b"timestamped output:             %s\n\0" as *const u8 as *const libc::c_char,
        if my.timestamp as libc::c_uint != 0 {
            b"true\0" as *const u8 as *const libc::c_char
        } else {
            b"false\0" as *const u8 as *const libc::c_char
        },
    );
    printf(
        b"comma separated output:         %s\n\0" as *const u8 as *const libc::c_char,
        if my.csv as libc::c_uint != 0 {
            b"true\0" as *const u8 as *const libc::c_char
        } else {
            b"false\0" as *const u8 as *const libc::c_char
        },
    );
    printf(
        b"allow redirects:                %s\n\0" as *const u8 as *const libc::c_char,
        if my.follow as libc::c_uint != 0 {
            b"true\0" as *const u8 as *const libc::c_char
        } else {
            b"false\0" as *const u8 as *const libc::c_char
        },
    );
    printf(
        b"allow zero byte data:           %s\n\0" as *const u8 as *const libc::c_char,
        if my.zero_ok as libc::c_uint != 0 {
            b"true\0" as *const u8 as *const libc::c_char
        } else {
            b"false\0" as *const u8 as *const libc::c_char
        },
    );
    printf(
        b"allow chunked encoding:         %s\n\0" as *const u8 as *const libc::c_char,
        if my.chunked as libc::c_uint != 0 {
            b"true\0" as *const u8 as *const libc::c_char
        } else {
            b"false\0" as *const u8 as *const libc::c_char
        },
    );
    printf(
        b"upload unique files:            %s\n\0" as *const u8 as *const libc::c_char,
        if my.unique as libc::c_uint != 0 {
            b"true\0" as *const u8 as *const libc::c_char
        } else {
            b"false\0" as *const u8 as *const libc::c_char
        },
    );
    printf(
        b"json output:                    %s\n\0" as *const u8 as *const libc::c_char,
        if my.json_output as libc::c_uint != 0 {
            b"true\0" as *const u8 as *const libc::c_char
        } else {
            b"false\0" as *const u8 as *const libc::c_char
        },
    );
    if my.parser as libc::c_uint == boolean_true as libc::c_int as libc::c_uint
        && (*my.nomap).index > 0 as libc::c_int
    {
        let mut i: libc::c_int = 0;
        printf(b"no-follow:\n\0" as *const u8 as *const libc::c_char);
        i = 0 as libc::c_int;
        while i < (*my.nomap).index {
            printf(
                b" - %s\n\0" as *const u8 as *const libc::c_char,
                *((*my.nomap).line).offset(i as isize),
            );
            i += 1;
            i;
        }
    }
    printf(b"proxy auth:                     \0" as *const u8 as *const libc::c_char);
    auth_display(my.auth, PROXY);
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    printf(b"www auth:                       \0" as *const u8 as *const libc::c_char);
    auth_display(my.auth, HTTP);
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    xfree(method as *mut libc::c_void);
    my.auth = auth_destroy(my.auth);
    my.lurl = array_destroy(my.lurl);
    my.cookies = cookies_destroy(my.cookies);
    if EXIT != 0 {
        exit(0 as libc::c_int);
    } else {
        return 0 as libc::c_int
    };
}
pub unsafe extern "C" fn readline(
    mut s: *mut *mut libc::c_char,
    mut fp: *mut FILE,
) -> libc::c_int {
    let mut c: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut len: libc::c_int = 0 as libc::c_int;
    let mut size: libc::c_int = 0 as libc::c_int;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut txt: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut lop: BOOLEAN = boolean_true;
    ptr = xmalloc(1024 as libc::c_int as size_t) as *mut libc::c_char;
    size = 1024 as libc::c_int;
    loop {
        c = fgetc(fp);
        if !(c != -(1 as libc::c_int) && c != '\n' as i32) {
            break;
        }
        if len >= size {
            tmp = realloc(
                ptr as *mut libc::c_void,
                (size + 1024 as libc::c_int) as libc::c_ulong,
            ) as *mut libc::c_char;
            if tmp.is_null() {
                free(ptr as *mut libc::c_void);
                return -(1 as libc::c_int);
            }
            ptr = tmp;
            size += 1024 as libc::c_int;
        }
        let fresh0 = len;
        len = len + 1;
        *ptr.offset(fresh0 as isize) = c as libc::c_char;
    }
    if len == 0 as libc::c_int {
        if c == -(1 as libc::c_int) {
            free(ptr as *mut libc::c_void);
            return -(1 as libc::c_int);
        } else {
            *ptr.offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
            *s = ptr;
            return 0 as libc::c_int;
        }
    }
    if len + 1 as libc::c_int != size {
        tmp = realloc(
            ptr as *mut libc::c_void,
            (len + 1 as libc::c_int) as libc::c_ulong,
        ) as *mut libc::c_char;
        if tmp.is_null() {
            free(ptr as *mut libc::c_void);
            return -(1 as libc::c_int);
        }
        ptr = tmp;
    }
    *ptr.offset(len as isize) = '\0' as i32 as libc::c_char;
    txt = strdup(ptr);
    trim(txt);
    if strncmp(
        txt,
        b"login\0" as *const u8 as *const libc::c_char,
        5 as libc::c_int as libc::c_ulong,
    ) == 0 as libc::c_int
    {
        lop = boolean_false;
    }
    xfree(txt as *mut libc::c_void);
    i = 0 as libc::c_int;
    while *ptr.offset(i as isize) as libc::c_int != '\0' as i32 {
        if *ptr.offset(i as isize) as libc::c_int == '#' as i32
            && lop as libc::c_uint == boolean_true as libc::c_int as libc::c_uint
        {
            *ptr.offset(i as isize) = '\0' as i32 as libc::c_char;
        }
        i += 1;
        i;
    }
    *s = ptr;
    return len;
}
pub unsafe extern "C" fn load_conf(mut filename: *mut libc::c_char) -> libc::c_int {
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut H: HASH = 0 as *mut HASH_T;
    let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut option: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut optionptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut zlib: BOOLEAN = boolean_true;
    fp = fopen(filename, b"r\0" as *const u8 as *const libc::c_char);
    if fp.is_null() {
        return -(1 as libc::c_int);
    }
    H = new_hash();
    while readline(&mut line, fp) != -(1 as libc::c_int) {
        let mut tmp: *mut libc::c_char = line;
        line = trim(line);
        if *line as libc::c_int == '#' as i32 || *line as libc::c_int == '\0' as i32
            || strlen(line) < 1 as libc::c_int as libc::c_ulong
        {
            free(line as *mut libc::c_void);
        } else {
            option = xstrdup(line);
            optionptr = option;
            while *optionptr as libc::c_int != 0
                && *(*__ctype_b_loc())
                    .offset(
                        *optionptr as libc::c_int as libc::c_uchar as libc::c_int
                            as isize,
                    ) as libc::c_int
                    & _ISspace as libc::c_int as libc::c_ushort as libc::c_int == 0
                && !('=' as i32 == *optionptr as libc::c_int
                    || ':' as i32 == *optionptr as libc::c_int)
            {
                optionptr = optionptr.offset(1);
                optionptr;
            }
            let fresh1 = optionptr;
            optionptr = optionptr.offset(1);
            *fresh1 = '\0' as i32 as libc::c_char;
            while *(*__ctype_b_loc())
                .offset(
                    *optionptr as libc::c_int as libc::c_uchar as libc::c_int as isize,
                ) as libc::c_int
                & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
                || ('=' as i32 == *optionptr as libc::c_int
                    || ':' as i32 == *optionptr as libc::c_int)
            {
                optionptr = optionptr.offset(1);
                optionptr;
            }
            value = xstrdup(optionptr);
            while *line != 0 {
                line = line.offset(1);
                line;
            }
            while !(strstr(option, b"$\0" as *const u8 as *const libc::c_char)).is_null()
            {
                option = evaluate(H, option);
            }
            while !(strstr(value, b"$\0" as *const u8 as *const libc::c_char)).is_null()
            {
                value = evaluate(H, value);
            }
            if strmatch(
                option,
                b"verbose\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) as u64 != 0
            {
                if strncasecmp(
                    value,
                    b"true\0" as *const u8 as *const libc::c_char,
                    4 as libc::c_int as libc::c_ulong,
                ) == 0
                {
                    my.verbose = boolean_true;
                } else {
                    my.verbose = boolean_false;
                }
            } else if strmatch(
                option,
                b"color\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) as u64 != 0
            {
                if strmatch(
                    value,
                    b"false\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ) as libc::c_uint != 0
                    || strmatch(
                        value,
                        b"off\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    ) as libc::c_uint != 0
                {
                    my.color = boolean_false;
                } else {
                    my.color = boolean_true;
                }
            } else if strmatch(
                option,
                b"quiet\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) as u64 != 0
            {
                if strncasecmp(
                    value,
                    b"true\0" as *const u8 as *const libc::c_char,
                    4 as libc::c_int as libc::c_ulong,
                ) == 0
                {
                    my.quiet = boolean_true;
                } else {
                    my.quiet = boolean_false;
                }
            } else if strmatch(
                option,
                b"parser\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) as u64 != 0
            {
                if strncasecmp(
                    value,
                    b"true\0" as *const u8 as *const libc::c_char,
                    4 as libc::c_int as libc::c_ulong,
                ) == 0
                {
                    my.parser = boolean_true;
                } else {
                    my.parser = boolean_false;
                }
            } else if strmatch(
                option,
                b"nofollow\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) as u64 != 0
            {
                if !value.is_null() && strlen(value) > 3 as libc::c_int as libc::c_ulong
                {
                    (*my.nomap)
                        .line = realloc(
                        (*my.nomap).line as *mut libc::c_void,
                        (::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
                            .wrapping_mul(
                                ((*my.nomap).index + 1 as libc::c_int) as libc::c_ulong,
                            ),
                    ) as *mut *mut libc::c_char;
                    let ref mut fresh2 = *((*my.nomap).line)
                        .offset((*my.nomap).index as isize);
                    *fresh2 = strdup(value);
                    (*my.nomap).index += 1;
                    (*my.nomap).index;
                }
            } else if strmatch(
                option,
                b"csv\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) as u64 != 0
            {
                if strncasecmp(
                    value,
                    b"true\0" as *const u8 as *const libc::c_char,
                    4 as libc::c_int as libc::c_ulong,
                ) == 0
                {
                    my.csv = boolean_true;
                } else {
                    my.csv = boolean_false;
                }
            } else if strmatch(
                option,
                b"fullurl\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) as u64 != 0
            {
                if strncasecmp(
                    value,
                    b"true\0" as *const u8 as *const libc::c_char,
                    4 as libc::c_int as libc::c_ulong,
                ) == 0
                {
                    my.fullurl = boolean_true;
                } else {
                    my.fullurl = boolean_false;
                }
            } else if strmatch(
                option,
                b"display-id\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) as u64 != 0
            {
                if strncasecmp(
                    value,
                    b"true\0" as *const u8 as *const libc::c_char,
                    4 as libc::c_int as libc::c_ulong,
                ) == 0
                {
                    my.display = boolean_true;
                } else {
                    my.display = boolean_false;
                }
            } else if strmatch(
                option,
                b"logging\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) as u64 != 0
            {
                if strncasecmp(
                    value,
                    b"true\0" as *const u8 as *const libc::c_char,
                    4 as libc::c_int as libc::c_ulong,
                ) == 0
                {
                    my.logging = boolean_true;
                } else {
                    my.logging = boolean_false;
                }
            } else if strmatch(
                option,
                b"show-logfile\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            ) as u64 != 0
            {
                if strncasecmp(
                    value,
                    b"true\0" as *const u8 as *const libc::c_char,
                    4 as libc::c_int as libc::c_ulong,
                ) == 0
                {
                    my.shlog = boolean_true;
                } else {
                    my.shlog = boolean_false;
                }
            } else if strmatch(
                option,
                b"logfile\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) as u64 != 0
            {
                strncpy(
                    (my.logfile).as_mut_ptr(),
                    value,
                    ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
                );
            } else if strmatch(
                option,
                b"concurrent\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) as u64 != 0
            {
                if !value.is_null() {
                    my.cusers = atoi(value);
                } else {
                    my.cusers = 10 as libc::c_int;
                }
            } else if strmatch(
                option,
                b"reps\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) as u64 != 0
            {
                if !value.is_null() {
                    my.reps = atoi(value);
                } else {
                    my.reps = 5 as libc::c_int;
                }
            } else if strmatch(
                option,
                b"limit\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) as u64 != 0
            {
                if !value.is_null() {
                    my.limit = atoi(value);
                } else {
                    my.limit = 255 as libc::c_int;
                }
            } else if strmatch(
                option,
                b"time\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) as u64 != 0
            {
                parse_time(value);
            } else if strmatch(
                option,
                b"delay\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) as u64 != 0
            {
                if !value.is_null() {
                    my.delay = atof(value) as libc::c_float;
                } else {
                    my.delay = 1 as libc::c_int as libc::c_float;
                }
            } else if strmatch(
                option,
                b"timeout\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) as u64 != 0
            {
                if !value.is_null() {
                    my.timeout = atoi(value);
                } else {
                    my.timeout = 15 as libc::c_int;
                }
            } else if strmatch(
                option,
                b"timestamp\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) as u64 != 0
            {
                if strncasecmp(
                    value,
                    b"true\0" as *const u8 as *const libc::c_char,
                    4 as libc::c_int as libc::c_ulong,
                ) == 0
                {
                    my.timestamp = boolean_true;
                } else {
                    my.timestamp = boolean_false;
                }
            } else if strmatch(
                option,
                b"internet\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) as u64 != 0
            {
                if strncasecmp(
                    value,
                    b"true\0" as *const u8 as *const libc::c_char,
                    4 as libc::c_int as libc::c_ulong,
                ) == 0
                {
                    my.internet = boolean_true;
                } else {
                    my.internet = boolean_false;
                }
            } else if strmatch(
                option,
                b"benchmark\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) as u64 != 0
            {
                if strncasecmp(
                    value,
                    b"true\0" as *const u8 as *const libc::c_char,
                    4 as libc::c_int as libc::c_ulong,
                ) == 0
                {
                    my.bench = boolean_true;
                } else {
                    my.bench = boolean_false;
                }
            } else if strmatch(
                option,
                b"cache\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) as u64 != 0
            {
                if strncasecmp(
                    value,
                    b"true\0" as *const u8 as *const libc::c_char,
                    4 as libc::c_int as libc::c_ulong,
                ) == 0
                {
                    my.cache = boolean_true;
                } else {
                    my.cache = boolean_false;
                }
            } else if strmatch(
                option,
                b"debug\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) as u64 != 0
            {
                if strncasecmp(
                    value,
                    b"true\0" as *const u8 as *const libc::c_char,
                    4 as libc::c_int as libc::c_ulong,
                ) == 0
                {
                    my.debug = boolean_true;
                } else {
                    my.debug = boolean_false;
                }
            } else if strmatch(
                option,
                b"gmethod\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) as u64 != 0
            {
                if strmatch(
                    value,
                    b"GET\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ) as u64 != 0
                {
                    my.method = GET;
                } else {
                    my.method = HEAD;
                }
            } else if strmatch(
                option,
                b"file\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) as u64 != 0
            {
                memset(
                    (my.file).as_mut_ptr() as *mut libc::c_void,
                    0 as libc::c_int,
                    ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
                );
                strncpy(
                    (my.file).as_mut_ptr(),
                    value,
                    ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
                );
            } else if strmatch(
                option,
                b"url\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) as u64 != 0
            {
                my.url = stralloc(value);
            } else if strmatch(
                option,
                b"user-agent\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) as u64 != 0
            {
                strncpy(
                    (my.uagent).as_mut_ptr(),
                    value,
                    ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
                );
            } else if strmatch(
                option,
                b"accept-encoding\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            ) as u64 != 0
            {
                let mut compress: BOOLEAN = boolean_false;
                if !(strstr(value, b"gzip\0" as *const u8 as *const libc::c_char))
                    .is_null()
                    || !(strstr(
                        value,
                        b"compress\0" as *const u8 as *const libc::c_char,
                    ))
                        .is_null()
                {
                    compress = boolean_true;
                }
                if compress as libc::c_uint
                    == boolean_true as libc::c_int as libc::c_uint
                    && zlib as libc::c_uint
                        == boolean_false as libc::c_int as libc::c_uint
                {
                    NOTIFY(
                        WARNING,
                        b"Zip encoding disabled; siege requires zlib support to enable it\0"
                            as *const u8 as *const libc::c_char,
                    );
                } else {
                    strncpy(
                        (my.encoding).as_mut_ptr(),
                        value,
                        ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
                    );
                }
            } else if strncasecmp(
                option,
                b"login\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int as libc::c_ulong,
            ) == 0
            {
                if strmatch(
                    option,
                    b"login-url\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                ) as u64 != 0
                {
                    my.login = boolean_true;
                    array_push(my.lurl, value as *mut libc::c_void);
                } else {
                    auth_add(my.auth, new_creds(HTTP, value));
                }
            } else if strmatch(
                option,
                b"attempts\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) as u64 != 0
            {
                if !value.is_null() {
                    my.bids = atoi(value);
                } else {
                    my.bids = 3 as libc::c_int;
                }
            } else if strmatch(
                option,
                b"connection\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) as u64 != 0
            {
                if strncasecmp(
                    value,
                    b"keep-alive\0" as *const u8 as *const libc::c_char,
                    10 as libc::c_int as libc::c_ulong,
                ) == 0
                {
                    my.keepalive = boolean_true;
                } else {
                    my.keepalive = boolean_false;
                }
            } else if strmatch(
                option,
                b"protocol\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) as u64 != 0
            {
                if strncasecmp(
                    value,
                    b"HTTP/1.1\0" as *const u8 as *const libc::c_char,
                    8 as libc::c_int as libc::c_ulong,
                ) == 0
                {
                    my.protocol = boolean_true as libc::c_int;
                } else {
                    my.protocol = boolean_false as libc::c_int;
                }
            } else if strmatch(
                option,
                b"proxy-host\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) as u64 != 0
            {
                auth_set_proxy_host(my.auth, trim(value));
            } else if strmatch(
                option,
                b"proxy-port\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) as u64 != 0
            {
                if !value.is_null() {
                    auth_set_proxy_port(my.auth, atoi(value));
                } else {
                    auth_set_proxy_port(my.auth, 3128 as libc::c_int);
                }
            } else if strmatch(
                option,
                b"ftp-login\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) as u64 != 0
            {
                auth_add(my.auth, new_creds(FTP, value));
            } else if strmatch(
                option,
                b"proxy-login\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) as u64 != 0
            {
                auth_add(my.auth, new_creds(PROXY, value));
            } else if strmatch(
                option,
                b"failures\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) as u64 != 0
            {
                if !value.is_null() {
                    my.failures = atoi(value);
                } else {
                    my.failures = 30 as libc::c_int;
                }
            } else if strmatch(
                option,
                b"chunked\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) as u64 != 0
            {
                if strncasecmp(
                    value,
                    b"true\0" as *const u8 as *const libc::c_char,
                    4 as libc::c_int as libc::c_ulong,
                ) == 0
                {
                    my.chunked = boolean_true;
                } else {
                    my.chunked = boolean_false;
                }
            } else if strmatch(
                option,
                b"unique\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) as u64 != 0
            {
                if strncasecmp(
                    value,
                    b"true\0" as *const u8 as *const libc::c_char,
                    4 as libc::c_int as libc::c_ulong,
                ) == 0
                {
                    my.unique = boolean_true;
                } else {
                    my.unique = boolean_false;
                }
            } else if strmatch(
                option,
                b"json_output\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) as u64 != 0
            {
                if strncasecmp(
                    value,
                    b"true\0" as *const u8 as *const libc::c_char,
                    4 as libc::c_int as libc::c_ulong,
                ) == 0
                {
                    my.json_output = boolean_true;
                } else {
                    my.json_output = boolean_false;
                }
            } else if strmatch(
                option,
                b"header\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) as u64 != 0
            {
                if (strchr(value, ':' as i32)).is_null() {
                    NOTIFY(
                        FATAL,
                        b"no ':' in http-header\0" as *const u8 as *const libc::c_char,
                    );
                }
                if (strlen(value))
                    .wrapping_add(strlen((my.extra).as_mut_ptr()))
                    .wrapping_add(3 as libc::c_int as libc::c_ulong)
                    > 512 as libc::c_int as libc::c_ulong
                {
                    NOTIFY(
                        FATAL,
                        b"too many headers\0" as *const u8 as *const libc::c_char,
                    );
                }
                strcat((my.extra).as_mut_ptr(), value);
                strcat(
                    (my.extra).as_mut_ptr(),
                    b"\r\n\0" as *const u8 as *const libc::c_char,
                );
            } else if strmatch(
                option,
                b"expire-session\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            ) as u64 != 0
            {
                if strncasecmp(
                    value,
                    b"true\0" as *const u8 as *const libc::c_char,
                    4 as libc::c_int as libc::c_ulong,
                ) == 0
                {
                    my.expire = boolean_true;
                } else {
                    my.expire = boolean_false;
                }
            } else if strmatch(
                option,
                b"follow-location\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            ) as u64 != 0
            {
                if strncasecmp(
                    value,
                    b"true\0" as *const u8 as *const libc::c_char,
                    4 as libc::c_int as libc::c_ulong,
                ) == 0
                {
                    my.follow = boolean_true;
                } else {
                    my.follow = boolean_false;
                }
            } else if strmatch(
                option,
                b"url-escaping\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            ) as u64 != 0
            {
                if strncasecmp(
                    value,
                    b"false\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int as libc::c_ulong,
                ) == 0
                {
                    my.escape = boolean_false;
                } else {
                    my.escape = boolean_true;
                }
            } else if strmatch(
                option,
                b"zero-data-ok\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            ) as u64 != 0
            {
                if strncasecmp(
                    value,
                    b"true\0" as *const u8 as *const libc::c_char,
                    4 as libc::c_int as libc::c_ulong,
                ) == 0
                {
                    my.zero_ok = boolean_true;
                } else {
                    my.zero_ok = boolean_false;
                }
            } else if strmatch(
                option,
                b"ssl-cert\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) as u64 != 0
            {
                my.ssl_cert = stralloc(value);
            } else if strmatch(
                option,
                b"ssl-key\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) as u64 != 0
            {
                my.ssl_key = stralloc(value);
            } else if strmatch(
                option,
                b"ssl-timeout\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) as u64 != 0
            {
                if !value.is_null() {
                    my.ssl_timeout = atoi(value);
                } else {
                    my.ssl_timeout = 15 as libc::c_int;
                }
            } else if strmatch(
                option,
                b"ssl-ciphers\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) as u64 != 0
            {
                my.ssl_ciphers = stralloc(value);
            } else if strmatch(
                option,
                b"spinner\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) as u64 != 0
            {
                if strncasecmp(
                    value,
                    b"true\0" as *const u8 as *const libc::c_char,
                    4 as libc::c_int as libc::c_ulong,
                ) == 0
                {
                    my.spinner = boolean_true;
                } else {
                    my.spinner = boolean_false;
                }
            } else {
                hash_add(H, option, value as *mut libc::c_void);
            }
            xfree(tmp as *mut libc::c_void);
            xfree(value as *mut libc::c_void);
            xfree(option as *mut libc::c_void);
        }
    }
    H = hash_destroy(H);
    fclose(fp);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn ds_module_check() {
    if my.bench as u64 != 0 {
        my.delay = 0 as libc::c_int as libc::c_float;
    }
    if my.secs > 0 as libc::c_int
        && (my.reps > 0 as libc::c_int && my.reps != 10301062 as libc::c_int)
    {
        NOTIFY(
            ERROR,
            b"CONFIG conflict: selected time and repetition based testing\0" as *const u8
                as *const libc::c_char,
        );
        fprintf(
            stderr,
            b"defaulting to time-based testing: %d seconds\n\0" as *const u8
                as *const libc::c_char,
            my.secs,
        );
        my.reps = 10301062 as libc::c_int;
    }
    if my.cusers <= 0 as libc::c_int {
        my.cusers = 1 as libc::c_int;
    }
    if my.get as u64 != 0 {
        my.cusers = 1 as libc::c_int;
        my.reps = 1 as libc::c_int;
        my.logging = boolean_false;
        my.bench = boolean_true;
    }
    if my.json_output as u64 != 0 {
        my.quiet = boolean_true;
    }
    if my.quiet as u64 != 0 {
        my.verbose = boolean_false;
        my.debug = boolean_false;
    }
}
