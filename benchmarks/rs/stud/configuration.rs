use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn vfprintf(
        _: *mut FILE,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn vsnprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn fgets(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncat(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strdup(__s: *const libc::c_char) -> *mut libc::c_char;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcasecmp(__s1: *const libc::c_char, __s2: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn __errno_location() -> *mut libc::c_int;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn geteuid() -> __uid_t;
    static mut optarg: *mut libc::c_char;
    static mut optind: libc::c_int;
    fn getopt_long(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
        __longopts: *const option,
        __longind: *mut libc::c_int,
    ) -> libc::c_int;
    fn __xpg_basename(__path: *mut libc::c_char) -> *mut libc::c_char;
    fn getpwuid(__uid: __uid_t) -> *mut passwd;
    fn getpwnam(__name: *const libc::c_char) -> *mut passwd;
    fn getgrgid(__gid: __gid_t) -> *mut group;
    fn getgrnam(__name: *const libc::c_char) -> *mut group;
    fn __xstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn init_openssl();
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
pub type gid_t = __gid_t;
pub type uid_t = __uid_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const libc::c_char,
    pub has_arg: libc::c_int,
    pub flag: *mut libc::c_int,
    pub val: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct passwd {
    pub pw_name: *mut libc::c_char,
    pub pw_passwd: *mut libc::c_char,
    pub pw_uid: __uid_t,
    pub pw_gid: __gid_t,
    pub pw_gecos: *mut libc::c_char,
    pub pw_dir: *mut libc::c_char,
    pub pw_shell: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct group {
    pub gr_name: *mut libc::c_char,
    pub gr_passwd: *mut libc::c_char,
    pub gr_gid: __gid_t,
    pub gr_mem: *mut *mut libc::c_char,
}
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
pub type ENC_TYPE = libc::c_uint;
pub const ENC_SSL: ENC_TYPE = 1;
pub const ENC_TLS: ENC_TYPE = 0;
pub type PROXY_MODE = libc::c_uint;
pub const SSL_CLIENT: PROXY_MODE = 1;
pub const SSL_SERVER: PROXY_MODE = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cert_files {
    pub CERT_FILE: *mut libc::c_char,
    pub NEXT: *mut cert_files,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __stud_config {
    pub ETYPE: ENC_TYPE,
    pub PMODE: PROXY_MODE,
    pub WRITE_IP_OCTET: libc::c_int,
    pub WRITE_PROXY_LINE: libc::c_int,
    pub PROXY_PROXY_LINE: libc::c_int,
    pub CHROOT: *mut libc::c_char,
    pub UID: uid_t,
    pub GID: gid_t,
    pub FRONT_IP: *mut libc::c_char,
    pub FRONT_PORT: *mut libc::c_char,
    pub BACK_IP: *mut libc::c_char,
    pub BACK_PORT: *mut libc::c_char,
    pub NCORES: libc::c_long,
    pub CERT_FILES: *mut cert_files,
    pub CIPHER_SUITE: *mut libc::c_char,
    pub ENGINE: *mut libc::c_char,
    pub BACKLOG: libc::c_int,
    pub QUIET: libc::c_int,
    pub SYSLOG: libc::c_int,
    pub SYSLOG_FACILITY: libc::c_int,
    pub TCP_KEEPALIVE_TIME: libc::c_int,
    pub DAEMONIZE: libc::c_int,
    pub PREFER_SERVER_CIPHERS: libc::c_int,
}
pub type stud_config = __stud_config;
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
#[inline]
unsafe extern "C" fn atol(mut __nptr: *const libc::c_char) -> libc::c_long {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    );
}
#[inline]
unsafe extern "C" fn stat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __xstat(1 as libc::c_int, __path, __statbuf);
}
static mut var_buf: [libc::c_char; 1024] = [0; 1024];
static mut val_buf: [libc::c_char; 1024] = [0; 1024];
static mut error_buf: [libc::c_char; 1024] = [0; 1024];
static mut tmp_buf: [libc::c_char; 150] = [0; 150];
unsafe extern "C" fn config_error_set(mut fmt: *mut libc::c_char, mut args: ...) {
    memset(
        error_buf.as_mut_ptr() as *mut libc::c_void,
        '\0' as i32,
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
    );
    let mut args_0: ::std::ffi::VaListImpl;
    args_0 = args.clone();
    vsnprintf(
        error_buf.as_mut_ptr(),
        (::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        fmt,
        args_0.as_va_list(),
    );
}
pub unsafe extern "C" fn config_error_get() -> *mut libc::c_char {
    return error_buf.as_mut_ptr();
}
pub unsafe extern "C" fn config_die(mut fmt: *mut libc::c_char, mut args: ...) {
    let mut args_0: ::std::ffi::VaListImpl;
    args_0 = args.clone();
    vfprintf(stderr, fmt, args_0.as_va_list());
    fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
    exit(1 as libc::c_int);
}
pub unsafe extern "C" fn config_new() -> *mut stud_config {
    let mut r: *mut stud_config = 0 as *mut stud_config;
    r = malloc(::std::mem::size_of::<stud_config>() as libc::c_ulong)
        as *mut stud_config;
    if r.is_null() {
        config_error_set(
            b"Unable to allocate memory for configuration structure: %s\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            strerror(*__errno_location()),
        );
        return 0 as *mut stud_config;
    }
    (*r).ETYPE = ENC_TLS;
    (*r).PMODE = SSL_SERVER;
    (*r).WRITE_IP_OCTET = 0 as libc::c_int;
    (*r).WRITE_PROXY_LINE = 0 as libc::c_int;
    (*r).PROXY_PROXY_LINE = 0 as libc::c_int;
    (*r).CHROOT = 0 as *mut libc::c_char;
    (*r).UID = 0 as libc::c_int as uid_t;
    (*r).GID = 0 as libc::c_int as gid_t;
    (*r).FRONT_IP = 0 as *mut libc::c_char;
    (*r).FRONT_PORT = strdup(b"8443\0" as *const u8 as *const libc::c_char);
    (*r).BACK_IP = strdup(b"127.0.0.1\0" as *const u8 as *const libc::c_char);
    (*r).BACK_PORT = strdup(b"8000\0" as *const u8 as *const libc::c_char);
    (*r).NCORES = 1 as libc::c_int as libc::c_long;
    (*r).CERT_FILES = 0 as *mut cert_files;
    (*r).CIPHER_SUITE = 0 as *mut libc::c_char;
    (*r).ENGINE = 0 as *mut libc::c_char;
    (*r).BACKLOG = 100 as libc::c_int;
    (*r).QUIET = 0 as libc::c_int;
    (*r).SYSLOG = 0 as libc::c_int;
    (*r).SYSLOG_FACILITY = (3 as libc::c_int) << 3 as libc::c_int;
    (*r).TCP_KEEPALIVE_TIME = 3600 as libc::c_int;
    (*r).DAEMONIZE = 0 as libc::c_int;
    (*r).PREFER_SERVER_CIPHERS = 0 as libc::c_int;
    return r;
}
pub unsafe extern "C" fn config_destroy(mut cfg: *mut stud_config) {
    if cfg.is_null() {
        return;
    }
    if !((*cfg).CHROOT).is_null() {
        free((*cfg).CHROOT as *mut libc::c_void);
    }
    if !((*cfg).FRONT_IP).is_null() {
        free((*cfg).FRONT_IP as *mut libc::c_void);
    }
    if !((*cfg).FRONT_PORT).is_null() {
        free((*cfg).FRONT_PORT as *mut libc::c_void);
    }
    if !((*cfg).BACK_IP).is_null() {
        free((*cfg).BACK_IP as *mut libc::c_void);
    }
    if !((*cfg).BACK_PORT).is_null() {
        free((*cfg).BACK_PORT as *mut libc::c_void);
    }
    if !((*cfg).CERT_FILES).is_null() {
        let mut curr: *mut cert_files = (*cfg).CERT_FILES;
        let mut next: *mut cert_files = 0 as *mut cert_files;
        while !((*cfg).CERT_FILES).is_null() {
            next = (*curr).NEXT;
            free(curr as *mut libc::c_void);
            curr = next;
        }
    }
    if !((*cfg).CIPHER_SUITE).is_null() {
        free((*cfg).CIPHER_SUITE as *mut libc::c_void);
    }
    if !((*cfg).ENGINE).is_null() {
        free((*cfg).ENGINE as *mut libc::c_void);
    }
    free(cfg as *mut libc::c_void);
}
pub unsafe extern "C" fn config_get_param(
    mut str: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    if str.is_null() {
        return 0 as *mut libc::c_char;
    }
    if strlen(str) < 1 as libc::c_int as libc::c_ulong
        || *str.offset(0 as libc::c_int as isize) as libc::c_int == '\n' as i32
        || strcmp(str, b"\r\n\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
    {
        return 0 as *mut libc::c_char;
    }
    ptr = str;
    if *str.offset(0 as libc::c_int as isize) as libc::c_int == '#' as i32 {
        return 0 as *mut libc::c_char;
    }
    while !ptr.is_null()
        && *(*__ctype_b_loc()).offset(*ptr as libc::c_int as isize) as libc::c_int
            & _ISalpha as libc::c_int as libc::c_ushort as libc::c_int == 0
    {
        ptr = ptr.offset(1);
        ptr;
    }
    memset(
        var_buf.as_mut_ptr() as *mut libc::c_void,
        '\0' as i32,
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
    );
    i = 0 as libc::c_int;
    while !ptr.is_null()
        && (*(*__ctype_b_loc()).offset(*ptr as libc::c_int as isize) as libc::c_int
            & _ISalnum as libc::c_int as libc::c_ushort as libc::c_int != 0
            || *ptr as libc::c_int == '-' as i32)
    {
        var_buf[i as usize] = *ptr;
        i += 1;
        i;
        ptr = ptr.offset(1);
        ptr;
    }
    if strlen(var_buf.as_mut_ptr()) < 1 as libc::c_int as libc::c_ulong {
        return 0 as *mut libc::c_char;
    }
    return var_buf.as_mut_ptr();
}
pub unsafe extern "C" fn config_get_value(
    mut str: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0 as libc::c_int;
    if str.is_null() {
        return 0 as *mut libc::c_char;
    }
    if strlen(str) < 1 as libc::c_int as libc::c_ulong {
        return 0 as *mut libc::c_char;
    }
    ptr = str;
    while !ptr.is_null() && *ptr as libc::c_int != '=' as i32 {
        ptr = ptr.offset(1);
        ptr;
    }
    ptr = ptr.offset(1);
    ptr;
    while !ptr.is_null()
        && *(*__ctype_b_loc()).offset(*ptr as libc::c_int as isize) as libc::c_int
            & _ISgraph as libc::c_int as libc::c_ushort as libc::c_int == 0
    {
        ptr = ptr.offset(1);
        ptr;
    }
    if ptr.is_null() {
        return 0 as *mut libc::c_char;
    }
    memset(
        val_buf.as_mut_ptr() as *mut libc::c_void,
        '\0' as i32,
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
    );
    while !ptr.is_null()
        && *(*__ctype_b_loc()).offset(*ptr as libc::c_int as isize) as libc::c_int
            & _ISgraph as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        let fresh0 = i;
        i = i + 1;
        val_buf[fresh0 as usize] = *ptr;
        ptr = ptr.offset(1);
        ptr;
    }
    if strlen(val_buf.as_mut_ptr()) < 1 as libc::c_int as libc::c_ulong {
        return 0 as *mut libc::c_char;
    }
    return val_buf.as_mut_ptr();
}
pub unsafe extern "C" fn str_rtrim(mut str: *mut libc::c_char) -> *mut libc::c_char {
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: libc::c_int = 0;
    len = strlen(str) as libc::c_int;
    ptr = str.offset(len as isize).offset(-(1 as libc::c_int as isize));
    while ptr >= str
        && (*(*__ctype_b_loc()).offset(*ptr as libc::c_int as isize) as libc::c_int
            & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
            || *ptr as libc::c_int == '"' as i32 || *ptr as libc::c_int == '\'' as i32)
    {
        ptr = ptr.offset(-1);
        ptr;
    }
    *ptr.offset(1 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
    return str;
}
pub unsafe extern "C" fn str_ltrim(mut str: *mut libc::c_char) -> *mut libc::c_char {
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: libc::c_int = 0;
    ptr = str;
    while *ptr as libc::c_int != 0
        && (*(*__ctype_b_loc()).offset(*ptr as libc::c_int as isize) as libc::c_int
            & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
            || *ptr as libc::c_int == '"' as i32 || *ptr as libc::c_int == '\'' as i32)
    {
        ptr = ptr.offset(1);
        ptr;
    }
    len = strlen(ptr) as libc::c_int;
    memmove(
        str as *mut libc::c_void,
        ptr as *const libc::c_void,
        (len + 1 as libc::c_int) as libc::c_ulong,
    );
    return str;
}
pub unsafe extern "C" fn str_trim(mut str: *mut libc::c_char) -> *mut libc::c_char {
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    ptr = str_rtrim(str);
    str = str_ltrim(ptr);
    return str;
}
pub unsafe extern "C" fn config_assign_str(
    mut dst: *mut *mut libc::c_char,
    mut v: *mut libc::c_char,
) -> *mut libc::c_char {
    if (*dst).is_null() {
        if !v.is_null() && strlen(v) > 0 as libc::c_int as libc::c_ulong {
            *dst = strdup(v);
        }
    } else if !v.is_null() && strlen(v) > 0 as libc::c_int as libc::c_ulong {
        memset(
            *dst as *mut libc::c_void,
            '\0' as i32,
            (strlen(v)).wrapping_add(1 as libc::c_int as libc::c_ulong),
        );
        memcpy(*dst as *mut libc::c_void, v as *const libc::c_void, strlen(v));
    } else {
        free(*dst as *mut libc::c_void);
    }
    return *dst;
}
pub unsafe extern "C" fn config_param_val_bool(
    mut val: *mut libc::c_char,
    mut res: *mut libc::c_int,
) -> libc::c_int {
    if val.is_null() {
        return 0 as libc::c_int;
    }
    if strcasecmp(val, b"on\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
        || strcasecmp(val, b"yes\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        || strcasecmp(val, b"y\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        || strcasecmp(val, b"true\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        || strcasecmp(val, b"t\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        || strcasecmp(val, b"1\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
    {
        *res = 1 as libc::c_int;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn config_param_val_str(
    mut val: *mut libc::c_char,
) -> *mut libc::c_char {
    return strdup(val);
}
pub unsafe extern "C" fn config_param_host_port_wildcard(
    mut str: *mut libc::c_char,
    mut addr: *mut *mut libc::c_char,
    mut port: *mut *mut libc::c_char,
    mut wildcard_okay: libc::c_int,
) -> libc::c_int {
    let mut len: libc::c_int = (if !str.is_null() {
        strlen(str)
    } else {
        0 as libc::c_int as libc::c_ulong
    }) as libc::c_int;
    if str.is_null() || len == 0 {
        config_error_set(
            b"Invalid/unset host/port string.\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return 0 as libc::c_int;
    }
    let mut port_buf: [libc::c_char; 6] = [0; 6];
    let mut addr_buf: [libc::c_char; 150] = [0; 150];
    memset(
        port_buf.as_mut_ptr() as *mut libc::c_void,
        '\0' as i32,
        ::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong,
    );
    memset(
        addr_buf.as_mut_ptr() as *mut libc::c_void,
        '\0' as i32,
        ::std::mem::size_of::<[libc::c_char; 150]>() as libc::c_ulong,
    );
    if *str as libc::c_int == '[' as i32 {
        let mut ptr: *mut libc::c_char = str.offset(1 as libc::c_int as isize);
        let mut x: *mut libc::c_char = strrchr(ptr, ']' as i32);
        if x.is_null() {
            config_error_set(
                b"Invalid address '%s'.\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                str,
            );
            return 0 as libc::c_int;
        }
        memcpy(
            addr_buf.as_mut_ptr() as *mut libc::c_void,
            ptr as *const libc::c_void,
            x.offset_from(ptr) as libc::c_long as libc::c_ulong,
        );
        x = x.offset(2 as libc::c_int as isize);
        memcpy(
            port_buf.as_mut_ptr() as *mut libc::c_void,
            x as *const libc::c_void,
            (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
    } else {
        let mut x_0: *mut libc::c_char = strrchr(str, ',' as i32);
        if x_0.is_null() {
            config_error_set(
                b"Invalid address string '%s'\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                str,
            );
            return 0 as libc::c_int;
        }
        let mut addr_len: libc::c_int = x_0.offset_from(str) as libc::c_long
            as libc::c_int;
        memcpy(
            addr_buf.as_mut_ptr() as *mut libc::c_void,
            str as *const libc::c_void,
            addr_len as libc::c_ulong,
        );
        x_0 = x_0.offset(1);
        memcpy(
            port_buf.as_mut_ptr() as *mut libc::c_void,
            x_0 as *const libc::c_void,
            ::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong,
        );
    }
    let mut p: libc::c_int = atoi(port_buf.as_mut_ptr());
    if p < 1 as libc::c_int || p > 65536 as libc::c_int {
        config_error_set(
            b"Invalid port number '%s'\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            port_buf.as_mut_ptr(),
        );
        return 0 as libc::c_int;
    }
    if strcmp(addr_buf.as_mut_ptr(), b"*\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        if wildcard_okay != 0 {
            free(*addr as *mut libc::c_void);
        } else {
            config_error_set(
                b"Invalid address: wildcards are not allowed.\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            );
            return 0 as libc::c_int;
        }
    } else {
        *addr = strdup(addr_buf.as_mut_ptr());
    }
    *port = strdup(port_buf.as_mut_ptr());
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn config_param_host_port(
    mut str: *mut libc::c_char,
    mut addr: *mut *mut libc::c_char,
    mut port: *mut *mut libc::c_char,
) -> libc::c_int {
    return config_param_host_port_wildcard(str, addr, port, 0 as libc::c_int);
}
pub unsafe extern "C" fn config_param_val_int(
    mut str: *mut libc::c_char,
    mut dst: *mut libc::c_int,
) -> libc::c_int {
    *dst = if !str.is_null() { atoi(str) } else { 0 as libc::c_int };
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn config_param_val_int_pos(
    mut str: *mut libc::c_char,
    mut dst: *mut libc::c_int,
) -> libc::c_int {
    let mut num: libc::c_int = 0 as libc::c_int;
    if !str.is_null() {
        num = atoi(str);
    }
    if num < 1 as libc::c_int {
        config_error_set(
            b"Not a positive number.\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return 0 as libc::c_int;
    }
    *dst = num;
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn config_param_val_intl(
    mut str: *mut libc::c_char,
    mut dst: *mut libc::c_long,
) -> libc::c_int {
    *dst = if !str.is_null() { atol(str) } else { 0 as libc::c_int as libc::c_long };
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn config_param_val_intl_pos(
    mut str: *mut libc::c_char,
    mut dst: *mut libc::c_long,
) -> libc::c_int {
    let mut num: libc::c_long = 0 as libc::c_int as libc::c_long;
    if !str.is_null() {
        num = atol(str);
    }
    if num < 1 as libc::c_int as libc::c_long {
        config_error_set(
            b"Not a positive number.\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return 0 as libc::c_int;
    }
    *dst = num;
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn config_param_validate(
    mut k: *mut libc::c_char,
    mut v: *mut libc::c_char,
    mut cfg: *mut stud_config,
    mut file: *mut libc::c_char,
    mut line: libc::c_int,
) {
    let mut r: libc::c_int = 1 as libc::c_int;
    let mut st: stat = stat {
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
    if strcmp(k, b"tls\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        (*cfg).ETYPE = ENC_TLS;
    } else if strcmp(k, b"ssl\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
    {
        (*cfg).ETYPE = ENC_SSL;
    } else if strcmp(k, b"ciphers\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        if !v.is_null() && strlen(v) > 0 as libc::c_int as libc::c_ulong {
            config_assign_str(&mut (*cfg).CIPHER_SUITE, v);
        }
    } else if strcmp(k, b"ssl-engine\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        if !v.is_null() && strlen(v) > 0 as libc::c_int as libc::c_ulong {
            config_assign_str(&mut (*cfg).ENGINE, v);
        }
    } else if strcmp(k, b"prefer-server-ciphers\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        r = config_param_val_bool(v, &mut (*cfg).PREFER_SERVER_CIPHERS);
    } else if strcmp(k, b"frontend\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        r = config_param_host_port_wildcard(
            v,
            &mut (*cfg).FRONT_IP,
            &mut (*cfg).FRONT_PORT,
            1 as libc::c_int,
        );
    } else if strcmp(k, b"backend\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        r = config_param_host_port(v, &mut (*cfg).BACK_IP, &mut (*cfg).BACK_PORT);
    } else if strcmp(k, b"workers\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        r = config_param_val_intl_pos(v, &mut (*cfg).NCORES);
    } else if strcmp(k, b"backlog\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        r = config_param_val_int(v, &mut (*cfg).BACKLOG);
        if r != 0 && (*cfg).BACKLOG < -(1 as libc::c_int) {
            (*cfg).BACKLOG = -(1 as libc::c_int);
        }
    } else if strcmp(k, b"keepalive\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        r = config_param_val_int_pos(v, &mut (*cfg).TCP_KEEPALIVE_TIME);
    } else if strcmp(k, b"chroot\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        if !v.is_null() && strlen(v) > 0 as libc::c_int as libc::c_ulong {
            if stat(v, &mut st) != 0 as libc::c_int {
                config_error_set(
                    b"Unable to stat directory '%s': %s'.\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    v,
                    strerror(*__errno_location()),
                );
                r = 0 as libc::c_int;
            } else if !(st.st_mode & 0o170000 as libc::c_int as libc::c_uint
                == 0o40000 as libc::c_int as libc::c_uint)
            {
                config_error_set(
                    b"Bad chroot directory '%s': Not a directory.\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    v,
                    strerror(*__errno_location()),
                );
                r = 0 as libc::c_int;
            } else {
                config_assign_str(&mut (*cfg).CHROOT, v);
            }
        }
    } else if strcmp(k, b"user\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        if !v.is_null() && strlen(v) > 0 as libc::c_int as libc::c_ulong {
            let mut passwd: *mut passwd = 0 as *mut passwd;
            passwd = getpwnam(v);
            if passwd.is_null() {
                config_error_set(
                    b"Invalid user '%s'.\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    v,
                );
                r = 0 as libc::c_int;
            } else {
                (*cfg).UID = (*passwd).pw_uid;
                (*cfg).GID = (*passwd).pw_gid;
            }
        }
    } else if strcmp(k, b"group\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        if !v.is_null() && strlen(v) > 0 as libc::c_int as libc::c_ulong {
            let mut grp: *mut group = 0 as *mut group;
            grp = getgrnam(v);
            if grp.is_null() {
                config_error_set(
                    b"Invalid group '%s'.\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    v,
                );
                r = 0 as libc::c_int;
            } else {
                (*cfg).GID = (*grp).gr_gid;
            }
        }
    } else if strcmp(k, b"quiet\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        r = config_param_val_bool(v, &mut (*cfg).QUIET);
    } else if strcmp(k, b"syslog\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        r = config_param_val_bool(v, &mut (*cfg).SYSLOG);
    } else if strcmp(k, b"syslog-facility\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        r = 1 as libc::c_int;
        if strcmp(v, b"auth\0" as *const u8 as *const libc::c_char) == 0
            || strcmp(v, b"authpriv\0" as *const u8 as *const libc::c_char) == 0
        {
            (*cfg).SYSLOG_FACILITY = (10 as libc::c_int) << 3 as libc::c_int;
        } else if strcmp(v, b"cron\0" as *const u8 as *const libc::c_char) == 0 {
            (*cfg).SYSLOG_FACILITY = (9 as libc::c_int) << 3 as libc::c_int;
        } else if strcmp(v, b"daemon\0" as *const u8 as *const libc::c_char) == 0 {
            (*cfg).SYSLOG_FACILITY = (3 as libc::c_int) << 3 as libc::c_int;
        } else if strcmp(v, b"ftp\0" as *const u8 as *const libc::c_char) == 0 {
            (*cfg).SYSLOG_FACILITY = (11 as libc::c_int) << 3 as libc::c_int;
        } else if strcmp(v, b"local0\0" as *const u8 as *const libc::c_char) == 0 {
            (*cfg).SYSLOG_FACILITY = (16 as libc::c_int) << 3 as libc::c_int;
        } else if strcmp(v, b"local1\0" as *const u8 as *const libc::c_char) == 0 {
            (*cfg).SYSLOG_FACILITY = (17 as libc::c_int) << 3 as libc::c_int;
        } else if strcmp(v, b"local2\0" as *const u8 as *const libc::c_char) == 0 {
            (*cfg).SYSLOG_FACILITY = (18 as libc::c_int) << 3 as libc::c_int;
        } else if strcmp(v, b"local3\0" as *const u8 as *const libc::c_char) == 0 {
            (*cfg).SYSLOG_FACILITY = (19 as libc::c_int) << 3 as libc::c_int;
        } else if strcmp(v, b"local4\0" as *const u8 as *const libc::c_char) == 0 {
            (*cfg).SYSLOG_FACILITY = (20 as libc::c_int) << 3 as libc::c_int;
        } else if strcmp(v, b"local5\0" as *const u8 as *const libc::c_char) == 0 {
            (*cfg).SYSLOG_FACILITY = (21 as libc::c_int) << 3 as libc::c_int;
        } else if strcmp(v, b"local6\0" as *const u8 as *const libc::c_char) == 0 {
            (*cfg).SYSLOG_FACILITY = (22 as libc::c_int) << 3 as libc::c_int;
        } else if strcmp(v, b"local7\0" as *const u8 as *const libc::c_char) == 0 {
            (*cfg).SYSLOG_FACILITY = (23 as libc::c_int) << 3 as libc::c_int;
        } else if strcmp(v, b"lpr\0" as *const u8 as *const libc::c_char) == 0 {
            (*cfg).SYSLOG_FACILITY = (6 as libc::c_int) << 3 as libc::c_int;
        } else if strcmp(v, b"mail\0" as *const u8 as *const libc::c_char) == 0 {
            (*cfg).SYSLOG_FACILITY = (2 as libc::c_int) << 3 as libc::c_int;
        } else if strcmp(v, b"news\0" as *const u8 as *const libc::c_char) == 0 {
            (*cfg).SYSLOG_FACILITY = (7 as libc::c_int) << 3 as libc::c_int;
        } else if strcmp(v, b"user\0" as *const u8 as *const libc::c_char) == 0 {
            (*cfg).SYSLOG_FACILITY = (1 as libc::c_int) << 3 as libc::c_int;
        } else if strcmp(v, b"uucp\0" as *const u8 as *const libc::c_char) == 0 {
            (*cfg).SYSLOG_FACILITY = (8 as libc::c_int) << 3 as libc::c_int;
        } else {
            config_error_set(
                b"Invalid facility '%s'.\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                v,
            );
            r = 0 as libc::c_int;
        }
    } else if strcmp(k, b"daemon\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        r = config_param_val_bool(v, &mut (*cfg).DAEMONIZE);
    } else if strcmp(k, b"write-ip\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        r = config_param_val_bool(v, &mut (*cfg).WRITE_IP_OCTET);
    } else if strcmp(k, b"write-proxy\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        r = config_param_val_bool(v, &mut (*cfg).WRITE_PROXY_LINE);
    } else if strcmp(k, b"proxy-proxy\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        r = config_param_val_bool(v, &mut (*cfg).PROXY_PROXY_LINE);
    } else if strcmp(k, b"pem-file\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        if !v.is_null() && strlen(v) > 0 as libc::c_int as libc::c_ulong {
            if stat(v, &mut st) != 0 as libc::c_int {
                config_error_set(
                    b"Unable to stat x509 certificate PEM file '%s': \0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    v,
                    strerror(*__errno_location()),
                );
                r = 0 as libc::c_int;
            } else if !(st.st_mode & 0o170000 as libc::c_int as libc::c_uint
                == 0o100000 as libc::c_int as libc::c_uint)
            {
                config_error_set(
                    b"Invalid x509 certificate PEM file '%s': Not a file.\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    v,
                );
                r = 0 as libc::c_int;
            } else {
                let mut cert: *mut cert_files = calloc(
                    1 as libc::c_int as libc::c_ulong,
                    ::std::mem::size_of::<cert_files>() as libc::c_ulong,
                ) as *mut cert_files;
                config_assign_str(&mut (*cert).CERT_FILE, v);
                (*cert).NEXT = (*cfg).CERT_FILES;
                (*cfg).CERT_FILES = cert;
            }
        }
    } else {
        fprintf(
            stderr,
            b"Ignoring unknown configuration key '%s' in configuration file '%s', line %d\n\0"
                as *const u8 as *const libc::c_char,
            k,
            file,
            line,
        );
    }
    if r == 0 {
        if !file.is_null() {
            config_die(
                b"Error in configuration file '%s', line %d: %s\n\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                file,
                line,
                config_error_get(),
            );
        } else {
            config_die(
                b"Invalid parameter '%s': %s\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                k,
                config_error_get(),
            );
        }
    }
}
pub unsafe extern "C" fn config_file_parse(
    mut file: *mut libc::c_char,
    mut cfg: *mut stud_config,
) -> libc::c_int {
    if cfg.is_null() {
        config_die(
            b"Undefined stud options; THIS IS A BUG!\n\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        );
    }
    let mut line: [libc::c_char; 1024] = [0; 1024];
    let mut fd: *mut FILE = 0 as *mut FILE;
    if file.is_null() || strlen(file) < 1 as libc::c_int as libc::c_ulong
        || strcmp(file, b"-\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
    {
        fd = stdin;
    } else {
        fd = fopen(file, b"r\0" as *const u8 as *const libc::c_char);
    }
    if fd.is_null() {
        config_die(
            b"Unable to open configuration file '%s': %s\n\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            file,
            strerror(*__errno_location()),
        );
    }
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 10000 as libc::c_int {
        memset(
            line.as_mut_ptr() as *mut libc::c_void,
            '\0' as i32,
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
        );
        if (fgets(
            line.as_mut_ptr(),
            (::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
            fd,
        ))
            .is_null()
        {
            break;
        }
        i += 1;
        i;
        let mut key: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut val: *mut libc::c_char = 0 as *mut libc::c_char;
        key = config_get_param(line.as_mut_ptr());
        if key.is_null() {
            continue;
        }
        val = config_get_value(line.as_mut_ptr());
        if val.is_null() {
            continue;
        }
        str_trim(val);
        config_param_validate(key, val, cfg, file, i);
    }
    fclose(fd);
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn config_disp_str(
    mut str: *mut libc::c_char,
) -> *mut libc::c_char {
    return (if str.is_null() {
        b"\0" as *const u8 as *const libc::c_char
    } else {
        str as *const libc::c_char
    }) as *mut libc::c_char;
}
pub unsafe extern "C" fn config_disp_bool(mut v: libc::c_int) -> *mut libc::c_char {
    return (if v > 0 as libc::c_int {
        b"on\0" as *const u8 as *const libc::c_char
    } else {
        b"off\0" as *const u8 as *const libc::c_char
    }) as *mut libc::c_char;
}
pub unsafe extern "C" fn config_disp_uid(mut uid: uid_t) -> *mut libc::c_char {
    memset(
        tmp_buf.as_mut_ptr() as *mut libc::c_void,
        '\0' as i32,
        ::std::mem::size_of::<[libc::c_char; 150]>() as libc::c_ulong,
    );
    if uid == 0 as libc::c_int as libc::c_uint
        && geteuid() != 0 as libc::c_int as libc::c_uint
    {
        return tmp_buf.as_mut_ptr();
    }
    let mut pw: *mut passwd = getpwuid(uid);
    if !pw.is_null() {
        memcpy(
            tmp_buf.as_mut_ptr() as *mut libc::c_void,
            (*pw).pw_name as *const libc::c_void,
            strlen((*pw).pw_name),
        );
    }
    return tmp_buf.as_mut_ptr();
}
pub unsafe extern "C" fn config_disp_gid(mut gid: gid_t) -> *mut libc::c_char {
    memset(
        tmp_buf.as_mut_ptr() as *mut libc::c_void,
        '\0' as i32,
        ::std::mem::size_of::<[libc::c_char; 150]>() as libc::c_ulong,
    );
    if gid == 0 as libc::c_int as libc::c_uint
        && geteuid() != 0 as libc::c_int as libc::c_uint
    {
        return tmp_buf.as_mut_ptr();
    }
    let mut gr: *mut group = getgrgid(gid);
    if !gr.is_null() {
        memcpy(
            tmp_buf.as_mut_ptr() as *mut libc::c_void,
            (*gr).gr_name as *const libc::c_void,
            strlen((*gr).gr_name),
        );
    }
    return tmp_buf.as_mut_ptr();
}
pub unsafe extern "C" fn config_disp_hostport(
    mut host: *mut libc::c_char,
    mut port: *mut libc::c_char,
) -> *mut libc::c_char {
    memset(
        tmp_buf.as_mut_ptr() as *mut libc::c_void,
        '\0' as i32,
        ::std::mem::size_of::<[libc::c_char; 150]>() as libc::c_ulong,
    );
    if host.is_null() && port.is_null() {
        return b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    strcat(tmp_buf.as_mut_ptr(), b"[\0" as *const u8 as *const libc::c_char);
    if host.is_null() {
        strcat(tmp_buf.as_mut_ptr(), b"*\0" as *const u8 as *const libc::c_char);
    } else {
        strncat(tmp_buf.as_mut_ptr(), host, 40 as libc::c_int as libc::c_ulong);
    }
    strcat(tmp_buf.as_mut_ptr(), b"]:\0" as *const u8 as *const libc::c_char);
    strncat(tmp_buf.as_mut_ptr(), port, 5 as libc::c_int as libc::c_ulong);
    return tmp_buf.as_mut_ptr();
}
pub unsafe extern "C" fn config_disp_log_facility(
    mut facility: libc::c_int,
) -> *const libc::c_char {
    match facility {
        80 => return b"authpriv\0" as *const u8 as *const libc::c_char,
        72 => return b"cron\0" as *const u8 as *const libc::c_char,
        24 => return b"daemon\0" as *const u8 as *const libc::c_char,
        88 => return b"ftp\0" as *const u8 as *const libc::c_char,
        128 => return b"local0\0" as *const u8 as *const libc::c_char,
        136 => return b"local1\0" as *const u8 as *const libc::c_char,
        144 => return b"local2\0" as *const u8 as *const libc::c_char,
        152 => return b"local3\0" as *const u8 as *const libc::c_char,
        160 => return b"local4\0" as *const u8 as *const libc::c_char,
        168 => return b"local5\0" as *const u8 as *const libc::c_char,
        176 => return b"local6\0" as *const u8 as *const libc::c_char,
        184 => return b"local7\0" as *const u8 as *const libc::c_char,
        48 => return b"lpr\0" as *const u8 as *const libc::c_char,
        16 => return b"mail\0" as *const u8 as *const libc::c_char,
        56 => return b"news\0" as *const u8 as *const libc::c_char,
        8 => return b"user\0" as *const u8 as *const libc::c_char,
        64 => return b"uucp\0" as *const u8 as *const libc::c_char,
        _ => return b"UNKNOWN\0" as *const u8 as *const libc::c_char,
    };
}
pub unsafe extern "C" fn config_print_usage_fd(
    mut prog: *mut libc::c_char,
    mut cfg: *mut stud_config,
    mut out: *mut FILE,
) {
    if out.is_null() {
        out = stderr;
    }
    fprintf(
        out,
        b"Usage: %s [OPTIONS] PEM\n\n\0" as *const u8 as *const libc::c_char,
        __xpg_basename(prog),
    );
    fprintf(
        out,
        b"This is stud, The Scalable TLS Unwrapping Daemon.\n\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(out, b"CONFIGURATION:\n\0" as *const u8 as *const libc::c_char);
    fprintf(out, b"\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        out,
        b"        --config=FILE      Load configuration from specified file.\n\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        out,
        b"        --default-config   Prints default configuration to stdout.\n\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(out, b"\n\0" as *const u8 as *const libc::c_char);
    fprintf(out, b"ENCRYPTION METHODS:\n\0" as *const u8 as *const libc::c_char);
    fprintf(out, b"\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        out,
        b"      --tls                   TLSv1 (default)\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        out,
        b"      --ssl                   SSLv3 (implies no TLSv1)\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        out,
        b"  -c  --ciphers=SUITE         Sets allowed ciphers (Default: \"%s\")\n\0"
            as *const u8 as *const libc::c_char,
        config_disp_str((*cfg).CIPHER_SUITE),
    );
    fprintf(
        out,
        b"  -e  --ssl-engine=NAME       Sets OpenSSL engine (Default: \"%s\")\n\0"
            as *const u8 as *const libc::c_char,
        config_disp_str((*cfg).ENGINE),
    );
    fprintf(
        out,
        b"  -O  --prefer-server-ciphers Prefer server list order\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(out, b"\n\0" as *const u8 as *const libc::c_char);
    fprintf(out, b"SOCKET:\n\0" as *const u8 as *const libc::c_char);
    fprintf(out, b"\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        out,
        b"  --client                    Enable client proxy mode\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        out,
        b"  -b  --backend=HOST,PORT     Backend [connect] (default is \"%s\")\n\0"
            as *const u8 as *const libc::c_char,
        config_disp_hostport((*cfg).BACK_IP, (*cfg).BACK_PORT),
    );
    fprintf(
        out,
        b"  -f  --frontend=HOST,PORT    Frontend [bind] (default is \"%s\")\n\0"
            as *const u8 as *const libc::c_char,
        config_disp_hostport((*cfg).FRONT_IP, (*cfg).FRONT_PORT),
    );
    fprintf(out, b"\n\0" as *const u8 as *const libc::c_char);
    fprintf(out, b"PERFORMANCE:\n\0" as *const u8 as *const libc::c_char);
    fprintf(out, b"\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        out,
        b"  -n  --workers=NUM          Number of worker processes (Default: %ld)\n\0"
            as *const u8 as *const libc::c_char,
        (*cfg).NCORES,
    );
    fprintf(
        out,
        b"  -B  --backlog=NUM          Set listen backlog size (Default: %d)\n\0"
            as *const u8 as *const libc::c_char,
        (*cfg).BACKLOG,
    );
    fprintf(
        out,
        b"  -k  --keepalive=SECS       TCP keepalive on client socket (Default: %d)\n\0"
            as *const u8 as *const libc::c_char,
        (*cfg).TCP_KEEPALIVE_TIME,
    );
    fprintf(out, b"\n\0" as *const u8 as *const libc::c_char);
    fprintf(out, b"SECURITY:\n\0" as *const u8 as *const libc::c_char);
    fprintf(out, b"\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        out,
        b"  -r  --chroot=DIR           Sets chroot directory (Default: \"%s\")\n\0"
            as *const u8 as *const libc::c_char,
        config_disp_str((*cfg).CHROOT),
    );
    fprintf(
        out,
        b"  -u  --user=USER            Set uid/gid after binding the socket (Default: \"%s\")\n\0"
            as *const u8 as *const libc::c_char,
        config_disp_uid((*cfg).UID),
    );
    fprintf(
        out,
        b"  -g  --group=GROUP          Set gid after binding the socket (Default: \"%s\")\n\0"
            as *const u8 as *const libc::c_char,
        config_disp_gid((*cfg).GID),
    );
    fprintf(out, b"\n\0" as *const u8 as *const libc::c_char);
    fprintf(out, b"LOGGING:\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        out,
        b"  -q  --quiet                Be quiet; emit only error messages\n\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        out,
        b"  -s  --syslog               Send log message to syslog in addition to stderr/stdout\n\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        out,
        b"  --syslog-facility=FACILITY Syslog facility to use (Default: \"%s\")\n\0"
            as *const u8 as *const libc::c_char,
        config_disp_log_facility((*cfg).SYSLOG_FACILITY),
    );
    fprintf(out, b"\n\0" as *const u8 as *const libc::c_char);
    fprintf(out, b"OTHER OPTIONS:\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        out,
        b"      --daemon               Fork into background and become a daemon (Default: %s)\n\0"
            as *const u8 as *const libc::c_char,
        config_disp_bool((*cfg).DAEMONIZE),
    );
    fprintf(
        out,
        b"      --write-ip             Write 1 octet with the IP family followed by the IP\n\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        out,
        b"                             address in 4 (IPv4) or 16 (IPv6) octets little-endian\n\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        out,
        b"                             to backend before the actual data\n\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        out,
        b"                             (Default: %s)\n\0" as *const u8
            as *const libc::c_char,
        config_disp_bool((*cfg).WRITE_IP_OCTET),
    );
    fprintf(
        out,
        b"      --write-proxy          Write HaProxy's PROXY (IPv4 or IPv6) protocol line\n\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        out,
        b"                             before actual data\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        out,
        b"                             (Default: %s)\n\0" as *const u8
            as *const libc::c_char,
        config_disp_bool((*cfg).WRITE_PROXY_LINE),
    );
    fprintf(
        out,
        b"      --proxy-proxy          Proxy HaProxy's PROXY (IPv4 or IPv6) protocol line\n\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        out,
        b"                             before actual data\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        out,
        b"                             (Default: %s)\n\0" as *const u8
            as *const libc::c_char,
        config_disp_bool((*cfg).PROXY_PROXY_LINE),
    );
    fprintf(out, b"\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        out,
        b"  -t  --test                 Test configuration and exit\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        out,
        b"  -V  --version              Print program version and exit\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        out,
        b"  -h  --help                 This help message\n\0" as *const u8
            as *const libc::c_char,
    );
}
pub unsafe extern "C" fn config_print_default(
    mut fd: *mut FILE,
    mut cfg: *mut stud_config,
) {
    if fd.is_null() {
        return;
    }
    fprintf(fd, b"#\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        fd,
        b"# stud(8), The Scalable TLS Unwrapping Daemon's configuration\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(fd, b"#\n\0" as *const u8 as *const libc::c_char);
    fprintf(fd, b"\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        fd,
        b"# NOTE: all config file parameters can be overriden\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(fd, b"#       from command line!\n\0" as *const u8 as *const libc::c_char);
    fprintf(fd, b"\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        fd,
        b"# Listening address. REQUIRED.\n\0" as *const u8 as *const libc::c_char,
    );
    fprintf(fd, b"#\n\0" as *const u8 as *const libc::c_char);
    fprintf(fd, b"# type: string\n\0" as *const u8 as *const libc::c_char);
    fprintf(fd, b"# syntax: [HOST]:PORT\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        fd,
        b"%s = \"%s\"\n\0" as *const u8 as *const libc::c_char,
        b"frontend\0" as *const u8 as *const libc::c_char,
        config_disp_hostport((*cfg).FRONT_IP, (*cfg).FRONT_PORT),
    );
    fprintf(fd, b"\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        fd,
        b"# Upstream server address. REQUIRED.\n\0" as *const u8 as *const libc::c_char,
    );
    fprintf(fd, b"#\n\0" as *const u8 as *const libc::c_char);
    fprintf(fd, b"# type: string\n\0" as *const u8 as *const libc::c_char);
    fprintf(fd, b"# syntax: [HOST]:PORT.\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        fd,
        b"%s = \"%s\"\n\0" as *const u8 as *const libc::c_char,
        b"backend\0" as *const u8 as *const libc::c_char,
        config_disp_hostport((*cfg).BACK_IP, (*cfg).BACK_PORT),
    );
    fprintf(fd, b"\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        fd,
        b"# SSL x509 certificate file. REQUIRED.\n\0" as *const u8 as *const libc::c_char,
    );
    fprintf(
        fd,
        b"# List multiple certs to use SNI. Certs are used in the order they\n\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        fd,
        b"# are listed; the last cert listed will be used if none of the others match\n\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(fd, b"#\n\0" as *const u8 as *const libc::c_char);
    fprintf(fd, b"# type: string\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        fd,
        b"%s = \"%s\"\n\0" as *const u8 as *const libc::c_char,
        b"pem-file\0" as *const u8 as *const libc::c_char,
        b"\0" as *const u8 as *const libc::c_char,
    );
    fprintf(fd, b"\n\0" as *const u8 as *const libc::c_char);
    fprintf(fd, b"# SSL protocol.\n\0" as *const u8 as *const libc::c_char);
    fprintf(fd, b"#\n\0" as *const u8 as *const libc::c_char);
    fprintf(fd, b"# tls = on\n\0" as *const u8 as *const libc::c_char);
    fprintf(fd, b"# ssl = off\n\0" as *const u8 as *const libc::c_char);
    fprintf(fd, b"\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        fd,
        b"# List of allowed SSL ciphers.\n\0" as *const u8 as *const libc::c_char,
    );
    fprintf(fd, b"#\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        fd,
        b"# Run openssl ciphers for list of available ciphers.\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(fd, b"# type: string\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        fd,
        b"%s = \"%s\"\n\0" as *const u8 as *const libc::c_char,
        b"ciphers\0" as *const u8 as *const libc::c_char,
        config_disp_str((*cfg).CIPHER_SUITE),
    );
    fprintf(fd, b"\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        fd,
        b"# Enforce server cipher list order\n\0" as *const u8 as *const libc::c_char,
    );
    fprintf(fd, b"#\n\0" as *const u8 as *const libc::c_char);
    fprintf(fd, b"# type: boolean\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        fd,
        b"%s = %s\n\0" as *const u8 as *const libc::c_char,
        b"prefer-server-ciphers\0" as *const u8 as *const libc::c_char,
        config_disp_bool((*cfg).PREFER_SERVER_CIPHERS),
    );
    fprintf(fd, b"\n\0" as *const u8 as *const libc::c_char);
    fprintf(fd, b"# Use specified SSL engine\n\0" as *const u8 as *const libc::c_char);
    fprintf(fd, b"#\n\0" as *const u8 as *const libc::c_char);
    fprintf(fd, b"# type: string\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        fd,
        b"%s = \"%s\"\n\0" as *const u8 as *const libc::c_char,
        b"ssl-engine\0" as *const u8 as *const libc::c_char,
        config_disp_str((*cfg).ENGINE),
    );
    fprintf(fd, b"\n\0" as *const u8 as *const libc::c_char);
    fprintf(fd, b"# Number of worker processes\n\0" as *const u8 as *const libc::c_char);
    fprintf(fd, b"#\n\0" as *const u8 as *const libc::c_char);
    fprintf(fd, b"# type: integer\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        fd,
        b"%s = %d\n\0" as *const u8 as *const libc::c_char,
        b"workers\0" as *const u8 as *const libc::c_char,
        (*cfg).NCORES as libc::c_int,
    );
    fprintf(fd, b"\n\0" as *const u8 as *const libc::c_char);
    fprintf(fd, b"# Listen backlog size\n\0" as *const u8 as *const libc::c_char);
    fprintf(fd, b"#\n\0" as *const u8 as *const libc::c_char);
    fprintf(fd, b"# type: integer\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        fd,
        b"%s = %d\n\0" as *const u8 as *const libc::c_char,
        b"backlog\0" as *const u8 as *const libc::c_char,
        (*cfg).BACKLOG,
    );
    fprintf(fd, b"\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        fd,
        b"# TCP socket keepalive interval in seconds\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(fd, b"#\n\0" as *const u8 as *const libc::c_char);
    fprintf(fd, b"# type: integer\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        fd,
        b"%s = %d\n\0" as *const u8 as *const libc::c_char,
        b"keepalive\0" as *const u8 as *const libc::c_char,
        (*cfg).TCP_KEEPALIVE_TIME,
    );
    fprintf(fd, b"\n\0" as *const u8 as *const libc::c_char);
    fprintf(fd, b"# Chroot directory\n\0" as *const u8 as *const libc::c_char);
    fprintf(fd, b"#\n\0" as *const u8 as *const libc::c_char);
    fprintf(fd, b"# type: string\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        fd,
        b"%s = \"%s\"\n\0" as *const u8 as *const libc::c_char,
        b"chroot\0" as *const u8 as *const libc::c_char,
        config_disp_str((*cfg).CHROOT),
    );
    fprintf(fd, b"\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        fd,
        b"# Set uid after binding a socket\n\0" as *const u8 as *const libc::c_char,
    );
    fprintf(fd, b"#\n\0" as *const u8 as *const libc::c_char);
    fprintf(fd, b"# type: string\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        fd,
        b"%s = \"%s\"\n\0" as *const u8 as *const libc::c_char,
        b"user\0" as *const u8 as *const libc::c_char,
        config_disp_uid((*cfg).UID),
    );
    fprintf(fd, b"\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        fd,
        b"# Set gid after binding a socket\n\0" as *const u8 as *const libc::c_char,
    );
    fprintf(fd, b"#\n\0" as *const u8 as *const libc::c_char);
    fprintf(fd, b"# type: string\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        fd,
        b"%s = \"%s\"\n\0" as *const u8 as *const libc::c_char,
        b"group\0" as *const u8 as *const libc::c_char,
        config_disp_gid((*cfg).GID),
    );
    fprintf(fd, b"\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        fd,
        b"# Quiet execution, report only error messages\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(fd, b"#\n\0" as *const u8 as *const libc::c_char);
    fprintf(fd, b"# type: boolean\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        fd,
        b"%s = %s\n\0" as *const u8 as *const libc::c_char,
        b"quiet\0" as *const u8 as *const libc::c_char,
        config_disp_bool((*cfg).QUIET),
    );
    fprintf(fd, b"\n\0" as *const u8 as *const libc::c_char);
    fprintf(fd, b"# Use syslog for logging\n\0" as *const u8 as *const libc::c_char);
    fprintf(fd, b"#\n\0" as *const u8 as *const libc::c_char);
    fprintf(fd, b"# type: boolean\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        fd,
        b"%s = %s\n\0" as *const u8 as *const libc::c_char,
        b"syslog\0" as *const u8 as *const libc::c_char,
        config_disp_bool((*cfg).SYSLOG),
    );
    fprintf(fd, b"\n\0" as *const u8 as *const libc::c_char);
    fprintf(fd, b"# Syslog facility to use\n\0" as *const u8 as *const libc::c_char);
    fprintf(fd, b"#\n\0" as *const u8 as *const libc::c_char);
    fprintf(fd, b"# type: string\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        fd,
        b"%s = \"%s\"\n\0" as *const u8 as *const libc::c_char,
        b"syslog-facility\0" as *const u8 as *const libc::c_char,
        config_disp_log_facility((*cfg).SYSLOG_FACILITY),
    );
    fprintf(fd, b"\n\0" as *const u8 as *const libc::c_char);
    fprintf(fd, b"# Run as daemon\n\0" as *const u8 as *const libc::c_char);
    fprintf(fd, b"#\n\0" as *const u8 as *const libc::c_char);
    fprintf(fd, b"# type: boolean\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        fd,
        b"%s = %s\n\0" as *const u8 as *const libc::c_char,
        b"daemon\0" as *const u8 as *const libc::c_char,
        config_disp_bool((*cfg).DAEMONIZE),
    );
    fprintf(fd, b"\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        fd,
        b"# Report client address by writing IP before sending data\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(fd, b"#\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        fd,
        b"# NOTE: This option is mutually exclusive with option %s and %s.\n\0"
            as *const u8 as *const libc::c_char,
        b"write-proxy\0" as *const u8 as *const libc::c_char,
        b"proxy-proxy\0" as *const u8 as *const libc::c_char,
    );
    fprintf(fd, b"#\n\0" as *const u8 as *const libc::c_char);
    fprintf(fd, b"# type: boolean\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        fd,
        b"%s = %s\n\0" as *const u8 as *const libc::c_char,
        b"write-ip\0" as *const u8 as *const libc::c_char,
        config_disp_bool((*cfg).WRITE_IP_OCTET),
    );
    fprintf(fd, b"\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        fd,
        b"# Report client address using SENDPROXY protocol, see\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        fd,
        b"# http://haproxy.1wt.eu/download/1.5/doc/proxy-protocol.txt\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(fd, b"# for details.\n\0" as *const u8 as *const libc::c_char);
    fprintf(fd, b"#\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        fd,
        b"# NOTE: This option is mutually exclusive with option %s and %s.\n\0"
            as *const u8 as *const libc::c_char,
        b"write-ip\0" as *const u8 as *const libc::c_char,
        b"proxy-proxy\0" as *const u8 as *const libc::c_char,
    );
    fprintf(fd, b"#\n\0" as *const u8 as *const libc::c_char);
    fprintf(fd, b"# type: boolean\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        fd,
        b"%s = %s\n\0" as *const u8 as *const libc::c_char,
        b"write-proxy\0" as *const u8 as *const libc::c_char,
        config_disp_bool((*cfg).WRITE_PROXY_LINE),
    );
    fprintf(fd, b"\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        fd,
        b"# Proxy an existing SENDPROXY protocol header through this request.\n\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(fd, b"#\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        fd,
        b"# NOTE: This option is mutually exclusive with option %s and %s.\n\0"
            as *const u8 as *const libc::c_char,
        b"write-ip\0" as *const u8 as *const libc::c_char,
        b"write-proxy\0" as *const u8 as *const libc::c_char,
    );
    fprintf(fd, b"#\n\0" as *const u8 as *const libc::c_char);
    fprintf(fd, b"# type: boolean\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        fd,
        b"%s = %s\n\0" as *const u8 as *const libc::c_char,
        b"proxy-proxy\0" as *const u8 as *const libc::c_char,
        config_disp_bool((*cfg).PROXY_PROXY_LINE),
    );
    fprintf(fd, b"\n\0" as *const u8 as *const libc::c_char);
    fprintf(fd, b"# EOF\n\0" as *const u8 as *const libc::c_char);
}
pub unsafe extern "C" fn config_print_usage(
    mut prog: *mut libc::c_char,
    mut cfg: *mut stud_config,
) {
    config_print_usage_fd(prog, cfg, stdout);
}
pub unsafe extern "C" fn config_parse_cli(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut cfg: *mut stud_config,
) {
    static mut tls: libc::c_int = 0 as libc::c_int;
    static mut ssl: libc::c_int = 0 as libc::c_int;
    static mut client: libc::c_int = 0 as libc::c_int;
    let mut c: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut test_only: libc::c_int = 0 as libc::c_int;
    let mut prog: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut long_options: [option; 26] = [
        {
            let mut init = option {
                name: b"config\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 10000 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"default-config\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 10001 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"tls\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: &mut tls,
                val: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"ssl\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: &mut ssl,
                val: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"client\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: &mut client,
                val: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"ciphers\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'c' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"prefer-server-ciphers\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'O' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"backend\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'b' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"frontend\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'f' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"workers\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'n' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"backlog\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'B' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"keepalive\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'k' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"chroot\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'r' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"user\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'u' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"group\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'g' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"quiet\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'q' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"syslog\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 's' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"syslog-facility\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 11015 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"daemon\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: &mut (*cfg).DAEMONIZE,
                val: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"write-ip\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: &mut (*cfg).WRITE_IP_OCTET,
                val: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"write-proxy\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: &mut (*cfg).WRITE_PROXY_LINE,
                val: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"proxy-proxy\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: &mut (*cfg).PROXY_PROXY_LINE,
                val: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"test\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 't' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"version\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'V' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"help\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'h' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: 0 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 0 as libc::c_int,
            };
            init
        },
    ];
    loop {
        let mut option_index: libc::c_int = 0 as libc::c_int;
        c = getopt_long(
            argc,
            argv,
            b"c:e:Ob:f:n:B:C:U:P:M:k:r:u:g:qstVh\0" as *const u8 as *const libc::c_char,
            long_options.as_mut_ptr(),
            &mut option_index,
        );
        if c == -(1 as libc::c_int) {
            break;
        }
        match c {
            0 => {}
            10000 => {
                if config_file_parse(optarg, cfg) == 0 {
                    config_die(
                        b"%s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                        config_error_get(),
                    );
                }
            }
            10001 => {
                config_print_default(stdout, cfg);
                exit(0 as libc::c_int);
            }
            11015 => {
                config_param_validate(
                    b"syslog-facility\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    optarg,
                    cfg,
                    0 as *mut libc::c_char,
                    0 as libc::c_int,
                );
            }
            99 => {
                config_param_validate(
                    b"ciphers\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    optarg,
                    cfg,
                    0 as *mut libc::c_char,
                    0 as libc::c_int,
                );
            }
            101 => {
                config_param_validate(
                    b"ssl-engine\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    optarg,
                    cfg,
                    0 as *mut libc::c_char,
                    0 as libc::c_int,
                );
            }
            79 => {
                config_param_validate(
                    b"prefer-server-ciphers\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    b"on\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    cfg,
                    0 as *mut libc::c_char,
                    0 as libc::c_int,
                );
            }
            98 => {
                config_param_validate(
                    b"backend\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    optarg,
                    cfg,
                    0 as *mut libc::c_char,
                    0 as libc::c_int,
                );
            }
            102 => {
                config_param_validate(
                    b"frontend\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    optarg,
                    cfg,
                    0 as *mut libc::c_char,
                    0 as libc::c_int,
                );
            }
            110 => {
                config_param_validate(
                    b"workers\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    optarg,
                    cfg,
                    0 as *mut libc::c_char,
                    0 as libc::c_int,
                );
            }
            66 => {
                config_param_validate(
                    b"backlog\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    optarg,
                    cfg,
                    0 as *mut libc::c_char,
                    0 as libc::c_int,
                );
            }
            107 => {
                config_param_validate(
                    b"keepalive\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    optarg,
                    cfg,
                    0 as *mut libc::c_char,
                    0 as libc::c_int,
                );
            }
            114 => {
                config_param_validate(
                    b"chroot\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    optarg,
                    cfg,
                    0 as *mut libc::c_char,
                    0 as libc::c_int,
                );
            }
            117 => {
                config_param_validate(
                    b"user\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    optarg,
                    cfg,
                    0 as *mut libc::c_char,
                    0 as libc::c_int,
                );
            }
            103 => {
                config_param_validate(
                    b"group\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    optarg,
                    cfg,
                    0 as *mut libc::c_char,
                    0 as libc::c_int,
                );
            }
            113 => {
                config_param_validate(
                    b"quiet\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    b"on\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    cfg,
                    0 as *mut libc::c_char,
                    0 as libc::c_int,
                );
            }
            115 => {
                config_param_validate(
                    b"syslog\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    b"on\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    cfg,
                    0 as *mut libc::c_char,
                    0 as libc::c_int,
                );
            }
            116 => {
                test_only = 1 as libc::c_int;
            }
            86 => {
                printf(
                    b"%s %s\n\0" as *const u8 as *const libc::c_char,
                    __xpg_basename(*argv.offset(0 as libc::c_int as isize)),
                    b"0.3-dev\0" as *const u8 as *const libc::c_char,
                );
                exit(0 as libc::c_int);
            }
            104 => {
                config_print_usage(*argv.offset(0 as libc::c_int as isize), cfg);
                exit(0 as libc::c_int);
            }
            _ => {
                config_die(
                    b"Invalid command line parameters. Run %s --help for instructions.\0"
                        as *const u8 as *const libc::c_char as *mut libc::c_char,
                    __xpg_basename(*argv.offset(0 as libc::c_int as isize)),
                );
            }
        }
    }
    prog = *argv.offset(0 as libc::c_int as isize);
    if tls != 0 && ssl != 0 {
        config_die(
            b"Options --tls and --ssl are mutually exclusive.\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        );
    } else if ssl != 0 {
        (*cfg).ETYPE = ENC_SSL;
    } else if tls != 0 {
        (*cfg).ETYPE = ENC_TLS;
    }
    if client != 0 {
        (*cfg).PMODE = SSL_CLIENT;
    }
    if (*cfg).WRITE_IP_OCTET != 0 && (*cfg).WRITE_PROXY_LINE != 0 {
        config_die(
            b"Options --write-ip and --write-proxy are mutually exclusive.\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    if (*cfg).WRITE_PROXY_LINE != 0 && (*cfg).PROXY_PROXY_LINE != 0 {
        config_die(
            b"Options --write-proxy and --proxy-proxy are mutually exclusive.\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    if (*cfg).WRITE_IP_OCTET != 0 && (*cfg).PROXY_PROXY_LINE != 0 {
        config_die(
            b"Options --write-ip and --proxy-proxy are mutually exclusive.\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    if (*cfg).DAEMONIZE != 0 {
        (*cfg).SYSLOG = 1 as libc::c_int;
        (*cfg).QUIET = 1 as libc::c_int;
    }
    argc -= optind;
    argv = argv.offset(optind as isize);
    i = 0 as libc::c_int;
    while i < argc {
        config_param_validate(
            b"pem-file\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            *argv.offset(i as isize),
            cfg,
            0 as *mut libc::c_char,
            0 as libc::c_int,
        );
        i += 1;
        i;
    }
    if (*cfg).PMODE as libc::c_uint == SSL_SERVER as libc::c_int as libc::c_uint
        && ((*cfg).CERT_FILES).is_null()
    {
        config_die(
            b"No x509 certificate PEM file specified!\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        );
    }
    if test_only != 0 {
        fprintf(
            stderr,
            b"Trying to initialize SSL contexts with your certificates\0" as *const u8
                as *const libc::c_char,
        );
        init_openssl();
        printf(
            b"%s configuration looks ok.\n\0" as *const u8 as *const libc::c_char,
            __xpg_basename(prog),
        );
        exit(0 as libc::c_int);
    }
}
