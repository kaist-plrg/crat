use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type htab;
    pub type upstream;
    pub type reversepath;
    static mut stdout: *mut FILE;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
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
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn getpid() -> __pid_t;
    fn fsync(__fd: libc::c_int) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn closelog();
    fn openlog(
        __ident: *const libc::c_char,
        __option: libc::c_int,
        __facility: libc::c_int,
    );
    fn syslog(__pri: libc::c_int, __fmt: *const libc::c_char, _: ...);
    fn strftime(
        __s: *mut libc::c_char,
        __maxsize: size_t,
        __format: *const libc::c_char,
        __tp: *const tm,
    ) -> size_t;
    fn localtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm;
    fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> libc::c_int;
    static mut config: *mut config_s;
    fn create_file_safely(
        filename: *const libc::c_char,
        truncate_file: libc::c_uint,
    ) -> libc::c_int;
    fn sblist_new(itemsize: size_t, blockitems: size_t) -> *mut sblist;
    fn sblist_free(l: *mut sblist);
    fn sblist_get(l: *mut sblist, item: size_t) -> *mut libc::c_void;
    fn sblist_add(l: *mut sblist, item: *mut libc::c_void) -> libc::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
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
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __clockid_t = libc::c_int;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
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
pub type clockid_t = __clockid_t;
pub type time_t = __time_t;
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
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
    pub __align: libc::c_long,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct config_s {
    pub basicauth_list: *mut sblist,
    pub logf_name: *mut libc::c_char,
    pub syslog: libc::c_uint,
    pub port: libc::c_uint,
    pub stathost: *mut libc::c_char,
    pub quit: libc::c_uint,
    pub maxclients: libc::c_uint,
    pub user: *mut libc::c_char,
    pub group: *mut libc::c_char,
    pub listen_addrs: *mut sblist,
    pub filter: *mut libc::c_char,
    pub filter_opts: libc::c_uint,
    pub add_xtinyproxy: libc::c_uint,
    pub reversepath_list: *mut reversepath,
    pub reverseonly: libc::c_uint,
    pub reversemagic: libc::c_uint,
    pub reversebaseurl: *mut libc::c_char,
    pub upstream_list: *mut upstream,
    pub pidpath: *mut libc::c_char,
    pub idletimeout: libc::c_uint,
    pub bind_addrs: *mut sblist,
    pub bindsame: libc::c_uint,
    pub via_proxy_name: *mut libc::c_char,
    pub disable_viaheader: libc::c_uint,
    pub errorpages: *mut htab,
    pub errorpage_undef: *mut libc::c_char,
    pub statpage: *mut libc::c_char,
    pub access_list: acl_list_t,
    pub connect_ports: *mut sblist,
    pub anonymous_map: *mut htab,
    pub add_headers: *mut sblist,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sblist {
    pub itemsize: size_t,
    pub blockitems: size_t,
    pub count: size_t,
    pub capa: size_t,
    pub items: *mut libc::c_char,
}
pub type acl_list_t = *mut sblist;
pub const PTHREAD_MUTEX_TIMED_NP: C2RustUnnamed = 0;
pub type C2RustUnnamed = libc::c_uint;
pub const PTHREAD_MUTEX_FAST_NP: C2RustUnnamed = 0;
pub const PTHREAD_MUTEX_DEFAULT: C2RustUnnamed = 0;
pub const PTHREAD_MUTEX_ERRORCHECK: C2RustUnnamed = 2;
pub const PTHREAD_MUTEX_RECURSIVE: C2RustUnnamed = 1;
pub const PTHREAD_MUTEX_NORMAL: C2RustUnnamed = 0;
pub const PTHREAD_MUTEX_ADAPTIVE_NP: C2RustUnnamed = 3;
pub const PTHREAD_MUTEX_ERRORCHECK_NP: C2RustUnnamed = 2;
pub const PTHREAD_MUTEX_RECURSIVE_NP: C2RustUnnamed = 1;
static mut syslog_level: [*const libc::c_char; 9] = [
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    b"CRITICAL\0" as *const u8 as *const libc::c_char,
    b"ERROR\0" as *const u8 as *const libc::c_char,
    b"WARNING\0" as *const u8 as *const libc::c_char,
    b"NOTICE\0" as *const u8 as *const libc::c_char,
    b"INFO\0" as *const u8 as *const libc::c_char,
    b"DEBUG\0" as *const u8 as *const libc::c_char,
    b"CONNECT\0" as *const u8 as *const libc::c_char,
];
static mut log_mutex: pthread_mutex_t = pthread_mutex_t {
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
pub static mut log_file_fd: libc::c_int = -(1 as libc::c_int);
static mut log_level: libc::c_int = 6 as libc::c_int;
static mut log_message_storage: *mut sblist = 0 as *const sblist as *mut sblist;
static mut logging_initialized: libc::c_uint = 0 as libc::c_int as libc::c_uint;
pub unsafe extern "C" fn open_log_file(
    mut log_file_name: *const libc::c_char,
) -> libc::c_int {
    if log_file_name.is_null() {
        log_file_fd = fileno(stdout);
    } else {
        log_file_fd = create_file_safely(
            log_file_name,
            0 as libc::c_int as libc::c_uint,
        );
    }
    return log_file_fd;
}
pub unsafe extern "C" fn close_log_file() {
    if log_file_fd < 0 as libc::c_int || log_file_fd == fileno(stdout) {
        return;
    }
    close(log_file_fd);
    log_file_fd = -(1 as libc::c_int);
}
pub unsafe extern "C" fn set_log_level(mut level: libc::c_int) {
    log_level = level;
}
pub unsafe extern "C" fn log_message(
    mut level: libc::c_int,
    mut fmt: *const libc::c_char,
    mut args: ...
) {
    let mut current_block: u64;
    let mut args_0: ::std::ffi::VaListImpl;
    let mut nowtime: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
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
    let mut time_string: [libc::c_char; 16] = [0; 16];
    let mut str: [libc::c_char; 800] = [0; 800];
    let mut ret: ssize_t = 0;
    if log_level == 8 as libc::c_int {
        if level == 6 as libc::c_int {
            return;
        }
    } else if log_level == 6 as libc::c_int {
        if level > 6 as libc::c_int && level != 8 as libc::c_int {
            return;
        }
    } else if level > log_level {
        return
    }
    if !config.is_null() && (*config).syslog != 0 && level == 8 as libc::c_int {
        level = 6 as libc::c_int;
    }
    args_0 = args.clone();
    if logging_initialized == 0 {
        let mut entry_buffer: *mut libc::c_char = 0 as *mut libc::c_char;
        if log_message_storage.is_null() {
            log_message_storage = sblist_new(
                ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                64 as libc::c_int as size_t,
            );
            if log_message_storage.is_null() {
                current_block = 4567019141635105728;
            } else {
                current_block = 7149356873433890176;
            }
        } else {
            current_block = 7149356873433890176;
        }
        match current_block {
            4567019141635105728 => {}
            _ => {
                vsnprintf(
                    str.as_mut_ptr(),
                    800 as libc::c_int as libc::c_ulong,
                    fmt,
                    args_0.as_va_list(),
                );
                entry_buffer = malloc(
                    (strlen(str.as_mut_ptr()))
                        .wrapping_add(6 as libc::c_int as libc::c_ulong),
                ) as *mut libc::c_char;
                if !entry_buffer.is_null() {
                    sprintf(
                        entry_buffer,
                        b"%d %s\0" as *const u8 as *const libc::c_char,
                        level,
                        str.as_mut_ptr(),
                    );
                    if sblist_add(
                        log_message_storage,
                        &mut entry_buffer as *mut *mut libc::c_char as *mut libc::c_void,
                    ) == 0
                    {
                        free(entry_buffer as *mut libc::c_void);
                        entry_buffer = 0 as *mut libc::c_char;
                    }
                }
            }
        }
    } else if !((*config).syslog == 0 && log_file_fd == -(1 as libc::c_int)) {
        if (*config).syslog != 0 {
            pthread_mutex_lock(&mut log_mutex);
            vsnprintf(
                str.as_mut_ptr(),
                800 as libc::c_int as libc::c_ulong,
                fmt,
                args_0.as_va_list(),
            );
            syslog(level, b"%s\0" as *const u8 as *const libc::c_char, str.as_mut_ptr());
            pthread_mutex_unlock(&mut log_mutex);
        } else {
            let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
            clock_gettime(0 as libc::c_int, &mut nowtime);
            strftime(
                time_string.as_mut_ptr(),
                16 as libc::c_int as size_t,
                b"%b %d %H:%M:%S\0" as *const u8 as *const libc::c_char,
                localtime_r(&mut nowtime.tv_sec, &mut tm_buf),
            );
            snprintf(
                str.as_mut_ptr(),
                800 as libc::c_int as libc::c_ulong,
                b"%-9s %s.%03lu [%ld]: \0" as *const u8 as *const libc::c_char,
                syslog_level[level as usize],
                time_string.as_mut_ptr(),
                (nowtime.tv_nsec as libc::c_ulong)
                    .wrapping_div(1000000 as libc::c_ulong),
                getpid() as libc::c_long,
            );
            p = str.as_mut_ptr().offset(strlen(str.as_mut_ptr()) as isize);
            vsnprintf(
                p,
                (800 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(strlen(str.as_mut_ptr()))
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                fmt,
                args_0.as_va_list(),
            );
            p = str.as_mut_ptr().offset(strlen(str.as_mut_ptr()) as isize);
            *p = '\n' as i32 as libc::c_char;
            *p.offset(1 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
            pthread_mutex_lock(&mut log_mutex);
            ret = write(
                log_file_fd,
                str.as_mut_ptr() as *const libc::c_void,
                strlen(str.as_mut_ptr()),
            );
            pthread_mutex_unlock(&mut log_mutex);
            if ret == -(1 as libc::c_int) as libc::c_long {
                (*config)
                    .syslog = (0 as libc::c_int == 0) as libc::c_int as libc::c_uint;
                log_message(
                    2 as libc::c_int,
                    b"ERROR: Could not write to log file %s: %s.\0" as *const u8
                        as *const libc::c_char,
                    (*config).logf_name,
                    strerror(*__errno_location()),
                );
                log_message(
                    2 as libc::c_int,
                    b"Falling back to syslog logging\0" as *const u8
                        as *const libc::c_char,
                );
            }
            pthread_mutex_lock(&mut log_mutex);
            fsync(log_file_fd);
            pthread_mutex_unlock(&mut log_mutex);
        }
    }
}
unsafe extern "C" fn send_stored_logs() {
    let mut string: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut level: libc::c_int = 0;
    let mut i: size_t = 0;
    if log_message_storage.is_null() {
        return;
    }
    log_message(
        7 as libc::c_int,
        b"sending stored logs\0" as *const u8 as *const libc::c_char,
    );
    let mut current_block_8: u64;
    i = 0 as libc::c_int as size_t;
    while i < (*log_message_storage).count {
        string = sblist_get(log_message_storage, i) as *mut *mut libc::c_char;
        if !(string.is_null() || (*string).is_null()) {
            ptr = (strchr(*string, ' ' as i32)).offset(1 as libc::c_int as isize);
            level = atoi(*string);
            if !(log_level == 8 as libc::c_int && level == 6 as libc::c_int) {
                if log_level == 6 as libc::c_int {
                    if level > 6 as libc::c_int && level != 8 as libc::c_int {
                        current_block_8 = 17179679302217393232;
                    } else {
                        current_block_8 = 2979737022853876585;
                    }
                } else if level > log_level {
                    current_block_8 = 17179679302217393232;
                } else {
                    current_block_8 = 2979737022853876585;
                }
                match current_block_8 {
                    17179679302217393232 => {}
                    _ => {
                        log_message(
                            level,
                            b"%s\0" as *const u8 as *const libc::c_char,
                            ptr,
                        );
                        free(*string as *mut libc::c_void);
                        *string = 0 as *mut libc::c_char;
                    }
                }
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    sblist_free(log_message_storage);
    log_message_storage = 0 as *mut sblist;
    log_message(
        7 as libc::c_int,
        b"done sending stored logs\0" as *const u8 as *const libc::c_char,
    );
}
pub unsafe extern "C" fn setup_logging() -> libc::c_int {
    if (*config).syslog == 0 {
        if open_log_file((*config).logf_name) < 0 as libc::c_int {
            (*config).syslog = (0 as libc::c_int == 0) as libc::c_int as libc::c_uint;
            log_message(
                2 as libc::c_int,
                b"ERROR: Could not create log file %s: %s.\0" as *const u8
                    as *const libc::c_char,
                (*config).logf_name,
                strerror(*__errno_location()),
            );
            log_message(
                2 as libc::c_int,
                b"Falling back to syslog logging.\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    if (*config).syslog != 0 {
        openlog(
            b"tinyproxy\0" as *const u8 as *const libc::c_char,
            0x1 as libc::c_int,
            (1 as libc::c_int) << 3 as libc::c_int,
        );
    }
    logging_initialized = (0 as libc::c_int == 0) as libc::c_int as libc::c_uint;
    send_stored_logs();
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn shutdown_logging() {
    if logging_initialized == 0 {
        return;
    }
    if (*config).syslog != 0 {
        closelog();
    } else {
        close_log_file();
    }
    logging_initialized = 0 as libc::c_int as libc::c_uint;
}
