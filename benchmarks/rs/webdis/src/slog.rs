use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type event_base;
    pub type worker;
    pub type acl;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn time(__timer: *mut time_t) -> time_t;
    fn strftime(
        __s: *mut libc::c_char,
        __maxsize: size_t,
        __format: *const libc::c_char,
        __tp: *const tm,
    ) -> size_t;
    fn localtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn getpid() -> __pid_t;
    fn fsync(__fd: libc::c_int) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn event_add(ev: *mut event, timeout: *const timeval) -> libc::c_int;
    fn event_new(
        _: *mut event_base,
        _: libc::c_int,
        _: libc::c_short,
        _: event_callback_fn,
        _: *mut libc::c_void,
    ) -> *mut event;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
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
pub type time_t = __time_t;
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
    pub __tm_gmtoff: libc::c_long,
    pub __tm_zone: *const libc::c_char,
}
pub type pid_t = __pid_t;
pub type gid_t = __gid_t;
pub type uid_t = __uid_t;
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
pub type log_level = libc::c_uint;
pub const WEBDIS_TRACE: log_level = 8;
pub const WEBDIS_DEBUG: log_level = 4;
pub const WEBDIS_INFO: log_level = 3;
pub const WEBDIS_NOTICE: log_level = 2;
pub const WEBDIS_WARNING: log_level = 1;
pub const WEBDIS_ERROR: log_level = 0;
pub type log_fsync_mode = libc::c_uint;
pub const LOG_FSYNC_ALL: log_fsync_mode = 2;
pub const LOG_FSYNC_MILLIS: log_fsync_mode = 1;
pub const LOG_FSYNC_AUTO: log_fsync_mode = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct server {
    pub fd: libc::c_int,
    pub ev: event,
    pub base: *mut event_base,
    pub cfg: *mut conf,
    pub w: *mut *mut worker,
    pub next_worker: libc::c_int,
    pub log: C2RustUnnamed,
    pub auth_log_mutex: pthread_mutex_t,
    pub auth_logged: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub self_0: pid_t,
    pub fd: libc::c_int,
    pub fsync_tv: timeval,
    pub fsync_ev: *mut event,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct event {
    pub ev_evcallback: event_callback,
    pub ev_timeout_pos: C2RustUnnamed_5,
    pub ev_fd: libc::c_int,
    pub ev_base: *mut event_base,
    pub ev_: C2RustUnnamed_0,
    pub ev_events: libc::c_short,
    pub ev_res: libc::c_short,
    pub ev_timeout: timeval,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub ev_io: C2RustUnnamed_3,
    pub ev_signal: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub ev_signal_next: C2RustUnnamed_2,
    pub ev_ncalls: libc::c_short,
    pub ev_pncalls: *mut libc::c_short,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub le_next: *mut event,
    pub le_prev: *mut *mut event,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub ev_io_next: C2RustUnnamed_4,
    pub ev_timeout: timeval,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub le_next: *mut event,
    pub le_prev: *mut *mut event,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_5 {
    pub ev_next_with_common_timeout: C2RustUnnamed_6,
    pub min_heap_idx: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct event_callback {
    pub evcb_active_next: C2RustUnnamed_8,
    pub evcb_flags: libc::c_short,
    pub evcb_pri: uint8_t,
    pub evcb_closure: uint8_t,
    pub evcb_cb_union: C2RustUnnamed_7,
    pub evcb_arg: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_7 {
    pub evcb_callback: Option::<
        unsafe extern "C" fn(libc::c_int, libc::c_short, *mut libc::c_void) -> (),
    >,
    pub evcb_selfcb: Option::<
        unsafe extern "C" fn(*mut event_callback, *mut libc::c_void) -> (),
    >,
    pub evcb_evfinalize: Option::<
        unsafe extern "C" fn(*mut event, *mut libc::c_void) -> (),
    >,
    pub evcb_cbfinalize: Option::<
        unsafe extern "C" fn(*mut event_callback, *mut libc::c_void) -> (),
    >,
}
pub type uint8_t = __uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub tqe_next: *mut event_callback,
    pub tqe_prev: *mut *mut event_callback,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct conf {
    pub redis_host: *mut libc::c_char,
    pub redis_port: libc::c_int,
    pub redis_auth: *mut auth,
    pub http_host: *mut libc::c_char,
    pub http_port: libc::c_int,
    pub http_threads: libc::c_int,
    pub http_max_request_size: size_t,
    pub pool_size_per_thread: libc::c_int,
    pub daemonize: libc::c_int,
    pub pidfile: *mut libc::c_char,
    pub websockets: libc::c_int,
    pub database: libc::c_int,
    pub perms: *mut acl,
    pub user: uid_t,
    pub group: gid_t,
    pub logfile: *mut libc::c_char,
    pub verbosity: log_level,
    pub log_fsync: C2RustUnnamed_10,
    pub hiredis_opts: C2RustUnnamed_9,
    pub default_root: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub keep_alive_sec: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub mode: log_fsync_mode,
    pub period_millis: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct auth {
    pub use_legacy_auth: libc::c_int,
    pub username: *mut libc::c_char,
    pub password: *mut libc::c_char,
}
pub type event_callback_fn = Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_short, *mut libc::c_void) -> (),
>;
pub unsafe extern "C" fn slog_init(mut s: *mut server) {
    (*s).log.self_0 = getpid();
    if !((*(*s).cfg).logfile).is_null() {
        let mut old_fd: libc::c_int = (*s).log.fd;
        (*s)
            .log
            .fd = open(
            (*(*s).cfg).logfile,
            0o1 as libc::c_int | 0o2000 as libc::c_int | 0o100 as libc::c_int,
            0o400 as libc::c_int | 0o200 as libc::c_int,
        );
        if old_fd != -(1 as libc::c_int) {
            close(old_fd);
        }
        if (*s).log.fd != -(1 as libc::c_int) {
            return;
        }
        fprintf(
            stderr,
            b"Could not open %s: %s\n\0" as *const u8 as *const libc::c_char,
            (*(*s).cfg).logfile,
            strerror(*__errno_location()),
        );
    }
    (*s).log.fd = 2 as libc::c_int;
}
unsafe extern "C" fn slog_fsync_tick(
    mut fd: libc::c_int,
    mut event: libc::c_short,
    mut data: *mut libc::c_void,
) {
    let mut s: *mut server = data as *mut server;
    let mut ret: libc::c_int = fsync((*s).log.fd);
}
pub unsafe extern "C" fn slog_fsync_init(mut s: *mut server) {
    if (*(*s).cfg).log_fsync.mode as libc::c_uint
        != LOG_FSYNC_MILLIS as libc::c_int as libc::c_uint
    {
        return;
    }
    (*s)
        .log
        .fsync_ev = event_new(
        (*s).base,
        -(1 as libc::c_int),
        0x10 as libc::c_int as libc::c_short,
        Some(
            slog_fsync_tick
                as unsafe extern "C" fn(
                    libc::c_int,
                    libc::c_short,
                    *mut libc::c_void,
                ) -> (),
        ),
        s as *mut libc::c_void,
    );
    if ((*s).log.fsync_ev).is_null() {
        let evnew_error: [libc::c_char; 33] = *::std::mem::transmute::<
            &[u8; 33],
            &[libc::c_char; 33],
        >(b"fsync timer could not be created\0");
        slog(
            s,
            WEBDIS_ERROR,
            evnew_error.as_ptr(),
            (::std::mem::size_of::<[libc::c_char; 33]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
        return;
    }
    let mut period_usec: libc::c_int = (*(*s).cfg).log_fsync.period_millis
        * 1000 as libc::c_int;
    (*s).log.fsync_tv.tv_sec = (period_usec / 1000000 as libc::c_int) as __time_t;
    (*s).log.fsync_tv.tv_usec = (period_usec % 1000000 as libc::c_int) as __suseconds_t;
    let mut ret_ta: libc::c_int = event_add((*s).log.fsync_ev, &mut (*s).log.fsync_tv);
    if ret_ta != 0 as libc::c_int {
        let reason: [libc::c_char; 35] = *::std::mem::transmute::<
            &[u8; 35],
            &[libc::c_char; 35],
        >(b"fsync timer could not be added: %d\0");
        let mut error_msg: [libc::c_char; 51] = [0; 51];
        let mut error_len: libc::c_int = snprintf(
            error_msg.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 51]>() as libc::c_ulong,
            reason.as_ptr(),
            ret_ta,
        );
        slog(s, WEBDIS_ERROR, error_msg.as_mut_ptr(), error_len as size_t);
    }
}
pub unsafe extern "C" fn slog_enabled(
    mut s: *mut server,
    mut level: log_level,
) -> libc::c_int {
    return if level as libc::c_uint <= (*(*s).cfg).verbosity as libc::c_uint {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    };
}
unsafe extern "C" fn slog_internal(
    mut s: *mut server,
    mut level: log_level,
    mut body: *const libc::c_char,
    mut sz: size_t,
) {
    let mut c: *const libc::c_char = b"EWNIDT\0" as *const u8 as *const libc::c_char;
    let mut now: time_t = 0;
    let mut now_tm: tm = tm {
        tm_sec: 0,
        tm_min: 0,
        tm_hour: 0,
        tm_mday: 0,
        tm_mon: 0,
        tm_year: 0,
        tm_wday: 0,
        tm_yday: 0,
        tm_isdst: 0,
        __tm_gmtoff: 0,
        __tm_zone: 0 as *const libc::c_char,
    };
    let mut lt_ret: *mut tm = 0 as *mut tm;
    let mut time_buf: [libc::c_char; 64] = [0; 64];
    let mut msg: [libc::c_char; 125] = [0; 125];
    let mut line: [libc::c_char; 248] = [0; 248];
    let mut line_sz: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    if (*s).log.fd == 0 {
        return;
    }
    sz = if sz != 0 { sz } else { strlen(body) };
    snprintf(
        msg.as_mut_ptr(),
        if sz.wrapping_add(1 as libc::c_int as libc::c_ulong)
            > ::std::mem::size_of::<[libc::c_char; 125]>() as libc::c_ulong
        {
            ::std::mem::size_of::<[libc::c_char; 125]>() as libc::c_ulong
        } else {
            sz.wrapping_add(1 as libc::c_int as libc::c_ulong)
        },
        b"%s\0" as *const u8 as *const libc::c_char,
        body,
    );
    now = time(0 as *mut time_t);
    lt_ret = localtime_r(&mut now, &mut now_tm);
    if !lt_ret.is_null() {
        strftime(
            time_buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
            b"%d %b %H:%M:%S\0" as *const u8 as *const libc::c_char,
            lt_ret,
        );
    } else {
        let err_msg: [libc::c_char; 20] = *::std::mem::transmute::<
            &[u8; 20],
            &[libc::c_char; 20],
        >(b"(NO TIME AVAILABLE)\0");
        memcpy(
            time_buf.as_mut_ptr() as *mut libc::c_void,
            err_msg.as_ptr() as *const libc::c_void,
            ::std::mem::size_of::<[libc::c_char; 20]>() as libc::c_ulong,
        );
    }
    let mut letter: libc::c_char = (if level as libc::c_uint
        == WEBDIS_TRACE as libc::c_int as libc::c_uint
    {
        *c.offset(5 as libc::c_int as isize) as libc::c_int
    } else {
        *c.offset(level as isize) as libc::c_int
    }) as libc::c_char;
    line_sz = snprintf(
        line.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 248]>() as libc::c_ulong,
        b"[%d] %s %c %s\n\0" as *const u8 as *const libc::c_char,
        (*s).log.self_0,
        time_buf.as_mut_ptr(),
        letter as libc::c_int,
        msg.as_mut_ptr(),
    );
    ret = write((*s).log.fd, line.as_mut_ptr() as *const libc::c_void, line_sz as size_t)
        as libc::c_int;
    if (*(*s).cfg).log_fsync.mode as libc::c_uint
        == LOG_FSYNC_ALL as libc::c_int as libc::c_uint
    {
        ret = fsync((*s).log.fd);
    }
}
pub unsafe extern "C" fn slog(
    mut s: *mut server,
    mut level: log_level,
    mut body: *const libc::c_char,
    mut sz: size_t,
) {
    if level as libc::c_uint <= (*(*s).cfg).verbosity as libc::c_uint {
        slog_internal(s, level, body, sz);
    }
}
