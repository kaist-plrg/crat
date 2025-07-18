use ::libc;
use ::c2rust_asm_casts;
use c2rust_asm_casts::AsmCastTrait;
use std::arch::asm;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn access(__name: *const libc::c_char, __type: libc::c_int) -> libc::c_int;
    fn lseek(__fd: libc::c_int, __offset: __off_t, __whence: libc::c_int) -> __off_t;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
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
    fn perror(__s: *const libc::c_char);
    fn select(
        __nfds: libc::c_int,
        __readfds: *mut fd_set,
        __writefds: *mut fd_set,
        __exceptfds: *mut fd_set,
        __timeout: *mut timeval,
    ) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn abort() -> !;
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
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strcspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn gettimeofday(__tv: *mut timeval, __tz: *mut libc::c_void) -> libc::c_int;
    fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option::<
            unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
        >,
        __arg: *mut libc::c_void,
    ) -> libc::c_int;
    fn pthread_join(
        __th: pthread_t,
        __thread_return: *mut *mut libc::c_void,
    ) -> libc::c_int;
    fn pthread_setcancelstate(
        __state: libc::c_int,
        __oldstate: *mut libc::c_int,
    ) -> libc::c_int;
    fn pthread_setcanceltype(
        __type: libc::c_int,
        __oldtype: *mut libc::c_int,
    ) -> libc::c_int;
    fn pthread_cancel(__th: pthread_t) -> libc::c_int;
    fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> libc::c_int;
    fn pthread_mutex_trylock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn strlcpy(_: *mut libc::c_char, _: *const libc::c_char, _: size_t) -> size_t;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn abuf_setup(abuf: *mut abuf_t, len: size_t) -> libc::c_int;
    fn tcp_read(
        tcp: *mut tcp_t,
        buffer_0: *mut libc::c_void,
        size: libc::c_int,
    ) -> ssize_t;
    fn http_decode(s: *mut libc::c_char);
    fn conn_set(conn: *mut conn_t, set_url: *const libc::c_char) -> libc::c_int;
    fn conn_url(
        dst: *mut libc::c_char,
        len: size_t,
        conn: *mut conn_t,
    ) -> *mut libc::c_char;
    fn conn_disconnect(conn: *mut conn_t);
    fn conn_init(conn: *mut conn_t) -> libc::c_int;
    fn conn_setup(conn: *mut conn_t) -> libc::c_int;
    fn conn_exec(conn: *mut conn_t) -> libc::c_int;
    fn conn_info(conn: *mut conn_t) -> libc::c_int;
    fn conn_info_status_get(
        msg: *mut libc::c_char,
        size: size_t,
        conn: *mut conn_t,
    ) -> libc::c_int;
    fn print_messages(axel: *mut axel_t);
    fn axel_size_human(
        dst: *mut libc::c_char,
        len: size_t,
        value: size_t,
    ) -> *mut libc::c_char;
    fn axel_sleep(delay: timespec) -> libc::c_int;
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
pub type __uint64_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_attr_t {
    pub __size: [libc::c_char; 56],
    pub __align: libc::c_long,
}
pub type mode_t = __mode_t;
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
pub type uint16_t = __uint16_t;
pub type uint64_t = __uint64_t;
pub type sa_family_t = libc::c_ushort;
pub type C2RustUnnamed = libc::c_uint;
pub const PTHREAD_CANCEL_DISABLE: C2RustUnnamed = 1;
pub const PTHREAD_CANCEL_ENABLE: C2RustUnnamed = 0;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const PTHREAD_CANCEL_ASYNCHRONOUS: C2RustUnnamed_0 = 1;
pub const PTHREAD_CANCEL_DEFERRED: C2RustUnnamed_0 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct message_t {
    pub next: *mut libc::c_void,
    pub text: [libc::c_char; 1024],
}
pub type url_t = message_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct search_t {
    pub url: [libc::c_char; 1024],
    pub speed_start_time: libc::c_double,
    pub speed: off_t,
    pub size: off_t,
    pub speed_thread: [pthread_t; 1],
    pub conf: *mut conf_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct axel_t {
    pub conn: *mut conn_t,
    pub conf: *mut conf_t,
    pub filename: [libc::c_char; 1024],
    pub start_time: libc::c_double,
    pub next_state: libc::c_int,
    pub finish_time: libc::c_int,
    pub bytes_done: off_t,
    pub start_byte: off_t,
    pub size: off_t,
    pub bytes_per_second: libc::c_longlong,
    pub delay_time: timespec,
    pub outfd: libc::c_int,
    pub ready: libc::c_int,
    pub message: *mut message_t,
    pub last_message: *mut message_t,
    pub url: *mut url_t,
}
#[inline]
unsafe extern "C" fn vprintf(
    mut __fmt: *const libc::c_char,
    mut __arg: ::std::ffi::VaList,
) -> libc::c_int {
    return vfprintf(stdout, __fmt, __arg.as_va_list());
}
static mut buffer: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
unsafe extern "C" fn stfile_makename(
    mut bname: *const libc::c_char,
) -> *mut libc::c_char {
    let suffix: [libc::c_char; 4] = *::std::mem::transmute::<
        &[u8; 4],
        &[libc::c_char; 4],
    >(b".st\0");
    let bname_len: size_t = strlen(bname);
    let mut buf: *mut libc::c_char = malloc(
        bname_len
            .wrapping_add(::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong),
    ) as *mut libc::c_char;
    if buf.is_null() {
        perror(b"stfile_open\0" as *const u8 as *const libc::c_char);
        abort();
    }
    memcpy(buf as *mut libc::c_void, bname as *const libc::c_void, bname_len);
    memcpy(
        buf.offset(bname_len as isize) as *mut libc::c_void,
        suffix.as_ptr() as *const libc::c_void,
        ::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong,
    );
    return buf;
}
unsafe extern "C" fn stfile_unlink(mut bname: *const libc::c_char) -> libc::c_int {
    let mut stname: *mut libc::c_char = stfile_makename(bname);
    let mut ret: libc::c_int = unlink(stname);
    free(stname as *mut libc::c_void);
    return ret;
}
unsafe extern "C" fn stfile_access(
    mut bname: *const libc::c_char,
    mut mode: libc::c_int,
) -> libc::c_int {
    let mut stname: *mut libc::c_char = stfile_makename(bname);
    let mut ret: libc::c_int = access(stname, mode);
    free(stname as *mut libc::c_void);
    return ret;
}
unsafe extern "C" fn stfile_open(
    mut bname: *const libc::c_char,
    mut flags: libc::c_int,
    mut mode: mode_t,
) -> libc::c_int {
    let mut stname: *mut libc::c_char = stfile_makename(bname);
    let mut fd: libc::c_int = open(stname, flags, mode);
    free(stname as *mut libc::c_void);
    return fd;
}
pub unsafe extern "C" fn axel_new(
    mut conf: *mut conf_t,
    mut count: libc::c_int,
    mut res: *const search_t,
) -> *mut axel_t {
    let mut current_block: u64;
    let mut axel: *mut axel_t = 0 as *mut axel_t;
    let mut status: libc::c_int = 0;
    let mut delay: uint64_t = 0;
    let mut u: *mut url_t = 0 as *mut url_t;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    if count == 0 || res.is_null() {
        return 0 as *mut axel_t;
    }
    axel = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<axel_t>() as libc::c_ulong,
    ) as *mut axel_t;
    if !axel.is_null() {
        (*axel).conf = conf;
        (*axel)
            .conn = calloc(
            (*(*axel).conf).num_connections as libc::c_ulong,
            ::std::mem::size_of::<conn_t>() as libc::c_ulong,
        ) as *mut conn_t;
        if !((*axel).conn).is_null() {
            i = 0 as libc::c_int;
            while i < (*(*axel).conf).num_connections as libc::c_int {
                pthread_mutex_init(
                    &mut (*((*axel).conn).offset(i as isize)).lock,
                    0 as *const pthread_mutexattr_t,
                );
                i += 1;
                i;
            }
            if (*(*axel).conf).max_speed > 0 as libc::c_int as libc::c_ulonglong {
                if (16 as libc::c_int as libc::c_ulonglong)
                    .wrapping_mul((*(*axel).conf).max_speed)
                    .wrapping_div((*(*axel).conf).buffer_size as libc::c_ulonglong)
                    < 8 as libc::c_int as libc::c_ulonglong
                {
                    if (*(*axel).conf).verbose >= 2 as libc::c_int {
                        axel_message(
                            axel,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"Buffer resized for this speed.\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                        );
                    }
                    (*(*axel).conf)
                        .buffer_size = (*(*axel).conf).max_speed as libc::c_int;
                }
                delay = ((1000000000 as libc::c_int * (*(*axel).conf).buffer_size
                    * (*(*axel).conf).num_connections as libc::c_int)
                    as libc::c_ulonglong)
                    .wrapping_div((*(*axel).conf).max_speed) as uint64_t;
                (*axel)
                    .delay_time
                    .tv_sec = delay
                    .wrapping_div(1000000000 as libc::c_int as libc::c_ulong)
                    as __time_t;
                (*axel)
                    .delay_time
                    .tv_nsec = delay
                    .wrapping_rem(1000000000 as libc::c_int as libc::c_ulong)
                    as __syscall_slong_t;
            }
            if buffer.is_null() {
                buffer = malloc((*(*axel).conf).buffer_size as libc::c_ulong)
                    as *mut libc::c_char;
                if buffer.is_null() {
                    current_block = 2915222835799028199;
                } else {
                    current_block = 12147880666119273379;
                }
            } else {
                current_block = 12147880666119273379;
            }
            match current_block {
                2915222835799028199 => {}
                _ => {
                    u = malloc(
                        (::std::mem::size_of::<url_t>() as libc::c_ulong)
                            .wrapping_mul(count as libc::c_ulong),
                    ) as *mut url_t;
                    if !u.is_null() {
                        (*axel).url = u;
                        i = 0 as libc::c_int;
                        while i < count {
                            strlcpy(
                                ((*u.offset(i as isize)).text).as_mut_ptr(),
                                ((*res.offset(i as isize)).url).as_ptr(),
                                ::std::mem::size_of::<[libc::c_char; 1024]>()
                                    as libc::c_ulong,
                            );
                            let ref mut fresh0 = (*u.offset(i as isize)).next;
                            *fresh0 = &mut *u.offset((i + 1 as libc::c_int) as isize)
                                as *mut url_t as *mut libc::c_void;
                            i += 1;
                            i;
                        }
                        let ref mut fresh1 = (*u
                            .offset((count - 1 as libc::c_int) as isize))
                            .next;
                        *fresh1 = u as *mut libc::c_void;
                        let ref mut fresh2 = (*((*axel).conn)
                            .offset(0 as libc::c_int as isize))
                            .conf;
                        *fresh2 = (*axel).conf;
                        if conn_set(
                            &mut *((*axel).conn).offset(0 as libc::c_int as isize),
                            ((*(*axel).url).text).as_mut_ptr(),
                        ) == 0
                        {
                            axel_message(
                                axel,
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"Could not parse URL.\n\0" as *const u8
                                        as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                            );
                            (*axel).ready = -(1 as libc::c_int);
                            return axel;
                        }
                        let ref mut fresh3 = (*((*axel).conn)
                            .offset(0 as libc::c_int as isize))
                            .local_if;
                        *fresh3 = ((*(*(*axel).conf).interfaces).text).as_mut_ptr();
                        (*(*axel).conf)
                            .interfaces = (*(*(*axel).conf).interfaces).next
                            as *mut if_t;
                        strlcpy(
                            ((*axel).filename).as_mut_ptr(),
                            ((*((*axel).conn).offset(0 as libc::c_int as isize)).file)
                                .as_mut_ptr(),
                            ::std::mem::size_of::<[libc::c_char; 1024]>()
                                as libc::c_ulong,
                        );
                        http_decode(((*axel).filename).as_mut_ptr());
                        s = strchr(((*axel).filename).as_mut_ptr(), '?' as i32);
                        if !s.is_null() && (*(*axel).conf).strip_cgi_parameters != 0 {
                            *s = 0 as libc::c_int as libc::c_char;
                        }
                        if *((*axel).filename).as_mut_ptr() as libc::c_int
                            == 0 as libc::c_int
                        {
                            strlcpy(
                                ((*axel).filename).as_mut_ptr(),
                                ((*(*axel).conf).default_filename).as_mut_ptr(),
                                ::std::mem::size_of::<[libc::c_char; 1024]>()
                                    as libc::c_ulong,
                            );
                        }
                        if (*(*axel).conf).no_clobber != 0
                            && access(((*axel).filename).as_mut_ptr(), 0 as libc::c_int)
                                == 0 as libc::c_int
                        {
                            let mut ret: libc::c_int = stfile_access(
                                ((*axel).filename).as_mut_ptr(),
                                0 as libc::c_int,
                            );
                            if ret != 0 {
                                printf(
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"File '%s' already there; not retrieving.\n\0" as *const u8
                                            as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                    ((*axel).filename).as_mut_ptr(),
                                );
                                (*axel).ready = -(1 as libc::c_int);
                                return axel;
                            }
                            printf(
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"Incomplete download found, ignoring no-clobber option\n\0"
                                        as *const u8 as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                            );
                        }
                        loop {
                            if conn_init(
                                &mut *((*axel).conn).offset(0 as libc::c_int as isize),
                            ) == 0
                            {
                                axel_message(
                                    axel,
                                    b"%s\0" as *const u8 as *const libc::c_char,
                                    (*((*axel).conn).offset(0 as libc::c_int as isize)).message,
                                );
                                (*axel).ready = -(1 as libc::c_int);
                                return axel;
                            }
                            status = conn_info(
                                &mut *((*axel).conn).offset(0 as libc::c_int as isize),
                            );
                            if status == 0 {
                                let mut msg: [libc::c_char; 80] = [0; 80];
                                let mut code: libc::c_int = conn_info_status_get(
                                    msg.as_mut_ptr(),
                                    ::std::mem::size_of::<[libc::c_char; 80]>()
                                        as libc::c_ulong,
                                    (*axel).conn,
                                );
                                fprintf(
                                    stderr,
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"ERROR %d: %s.\n\0" as *const u8 as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                    code,
                                    msg.as_mut_ptr(),
                                );
                                (*axel).ready = -(1 as libc::c_int);
                                return axel;
                            }
                            if !(status == -(1 as libc::c_int)) {
                                break;
                            }
                        }
                        conn_url(
                            ((*(*axel).url).text).as_mut_ptr(),
                            (::std::mem::size_of::<[libc::c_char; 1024]>()
                                as libc::c_ulong)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                            (*axel).conn,
                        );
                        (*axel)
                            .size = (*((*axel).conn).offset(0 as libc::c_int as isize))
                            .size;
                        if (*(*axel).conf).verbose > 0 as libc::c_int {
                            if (*axel).size as libc::c_longlong
                                != 9223372036854775807 as libc::c_longlong
                            {
                                let mut hsize: [libc::c_char; 32] = [0; 32];
                                axel_size_human(
                                    hsize.as_mut_ptr(),
                                    ::std::mem::size_of::<[libc::c_char; 32]>()
                                        as libc::c_ulong,
                                    (*axel).size as size_t,
                                );
                                axel_message(
                                    axel,
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"File size: %s (%jd bytes)\0" as *const u8
                                            as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                    hsize.as_mut_ptr(),
                                    (*axel).size,
                                );
                            } else {
                                axel_message(
                                    axel,
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"File size: unavailable\0" as *const u8
                                            as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                );
                            }
                        }
                        if (*axel)
                            .filename[strcspn(
                            ((*axel).filename).as_mut_ptr(),
                            b"*?\0" as *const u8 as *const libc::c_char,
                        ) as usize] != 0
                        {
                            strlcpy(
                                ((*axel).filename).as_mut_ptr(),
                                ((*((*axel).conn).offset(0 as libc::c_int as isize)).file)
                                    .as_mut_ptr(),
                                ::std::mem::size_of::<[libc::c_char; 1024]>()
                                    as libc::c_ulong,
                            );
                        }
                        if *((*((*axel).conn).offset(0 as libc::c_int as isize))
                            .output_filename)
                            .as_mut_ptr() as libc::c_int != 0 as libc::c_int
                        {
                            strlcpy(
                                ((*axel).filename).as_mut_ptr(),
                                ((*((*axel).conn).offset(0 as libc::c_int as isize))
                                    .output_filename)
                                    .as_mut_ptr(),
                                ::std::mem::size_of::<[libc::c_char; 1024]>()
                                    as libc::c_ulong,
                            );
                        }
                        return axel;
                    }
                }
            }
        }
    }
    axel_close(axel);
    printf(b"%s\n\0" as *const u8 as *const libc::c_char, strerror(*__errno_location()));
    return 0 as *mut axel_t;
}
pub unsafe extern "C" fn axel_open(mut axel: *mut axel_t) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut fd: libc::c_int = 0;
    let mut nread: ssize_t = 0;
    if (*(*axel).conf).verbose > 0 as libc::c_int {
        axel_message(
            axel,
            dcgettext(
                0 as *const libc::c_char,
                b"Opening output file %s\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            ((*axel).filename).as_mut_ptr(),
        );
    }
    (*axel).outfd = -(1 as libc::c_int);
    if !(*((*axel).conn).offset(0 as libc::c_int as isize)).supported {
        axel_message(
            axel,
            dcgettext(
                0 as *const libc::c_char,
                b"Server unsupported, starting from scratch with one connection.\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        (*(*axel).conf).num_connections = 1 as libc::c_int as uint16_t;
        let mut new_conn: *mut libc::c_void = realloc(
            (*axel).conn as *mut libc::c_void,
            ::std::mem::size_of::<conn_t>() as libc::c_ulong,
        );
        if new_conn.is_null() {
            return 0 as libc::c_int;
        }
        (*axel).conn = new_conn as *mut conn_t;
        axel_divide(axel);
    } else {
        fd = stfile_open(
            ((*axel).filename).as_mut_ptr(),
            0 as libc::c_int,
            0 as libc::c_int as mode_t,
        );
        if fd != -(1 as libc::c_int) {
            let mut old_format: libc::c_int = 0 as libc::c_int;
            let mut stsize: off_t = lseek(
                fd,
                0 as libc::c_int as __off_t,
                2 as libc::c_int,
            );
            lseek(fd, 0 as libc::c_int as __off_t, 0 as libc::c_int);
            nread = read(
                fd,
                &mut (*(*axel).conf).num_connections as *mut uint16_t
                    as *mut libc::c_void,
                ::std::mem::size_of::<uint16_t>() as libc::c_ulong,
            );
            if nread as libc::c_ulong
                != ::std::mem::size_of::<uint16_t>() as libc::c_ulong
            {
                printf(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%s.st: Error, truncated state file\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    ((*axel).filename).as_mut_ptr(),
                );
                close(fd);
                return 0 as libc::c_int;
            }
            if ((*(*axel).conf).num_connections as libc::c_int) < 1 as libc::c_int {
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Bogus number of connections stored in state file\n\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                close(fd);
                return 0 as libc::c_int;
            }
            if stsize
                < (::std::mem::size_of::<uint16_t>() as libc::c_ulong)
                    .wrapping_add(::std::mem::size_of::<off_t>() as libc::c_ulong)
                    .wrapping_add(
                        ((2 as libc::c_int
                            * (*(*axel).conf).num_connections as libc::c_int)
                            as libc::c_ulong)
                            .wrapping_mul(
                                ::std::mem::size_of::<off_t>() as libc::c_ulong,
                            ),
                    ) as off_t
            {
                old_format = 1 as libc::c_int;
            }
            let mut new_conn_0: *mut libc::c_void = realloc(
                (*axel).conn as *mut libc::c_void,
                (::std::mem::size_of::<conn_t>() as libc::c_ulong)
                    .wrapping_mul((*(*axel).conf).num_connections as libc::c_ulong),
            );
            if new_conn_0.is_null() {
                close(fd);
                return 0 as libc::c_int;
            }
            (*axel).conn = new_conn_0 as *mut conn_t;
            memset(
                ((*axel).conn).offset(1 as libc::c_int as isize) as *mut libc::c_void,
                0 as libc::c_int,
                (::std::mem::size_of::<conn_t>() as libc::c_ulong)
                    .wrapping_mul(
                        ((*(*axel).conf).num_connections as libc::c_int
                            - 1 as libc::c_int) as libc::c_ulong,
                    ),
            );
            if old_format != 0 {
                axel_divide(axel);
            }
            nread = read(
                fd,
                &mut (*axel).bytes_done as *mut off_t as *mut libc::c_void,
                ::std::mem::size_of::<off_t>() as libc::c_ulong,
            );
            i = 0 as libc::c_int;
            while i < (*(*axel).conf).num_connections as libc::c_int {
                nread = read(
                    fd,
                    &mut (*((*axel).conn).offset(i as isize)).currentbyte as *mut off_t
                        as *mut libc::c_void,
                    ::std::mem::size_of::<off_t>() as libc::c_ulong,
                );
                if old_format == 0 {
                    nread = read(
                        fd,
                        &mut (*((*axel).conn).offset(i as isize)).lastbyte as *mut off_t
                            as *mut libc::c_void,
                        ::std::mem::size_of::<off_t>() as libc::c_ulong,
                    );
                }
                i += 1;
                i;
            }
            axel_message(
                axel,
                dcgettext(
                    0 as *const libc::c_char,
                    b"State file found: %jd bytes downloaded, %jd to go.\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*axel).bytes_done,
                (*axel).size - (*axel).bytes_done,
            );
            close(fd);
            (*axel)
                .outfd = open(
                ((*axel).filename).as_mut_ptr(),
                0o1 as libc::c_int,
                0o666 as libc::c_int,
            );
            if (*axel).outfd == -(1 as libc::c_int) {
                axel_message(
                    axel,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Error opening local file\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                return 0 as libc::c_int;
            }
        }
    }
    if (*axel).outfd == -(1 as libc::c_int) {
        axel_divide(axel);
        (*axel)
            .outfd = open(
            ((*axel).filename).as_mut_ptr(),
            0o100 as libc::c_int | 0o1 as libc::c_int,
            0o666 as libc::c_int,
        );
        if (*axel).outfd == -(1 as libc::c_int) {
            axel_message(
                axel,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Error opening local file\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            return 0 as libc::c_int;
        }
        if lseek((*axel).outfd, (*axel).size, 0 as libc::c_int)
            == -(1 as libc::c_int) as libc::c_long
            && (*(*axel).conf).num_connections as libc::c_int > 1 as libc::c_int
        {
            axel_message(
                axel,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Crappy filesystem/OS.. Working around. :-(\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            lseek((*axel).outfd, 0 as libc::c_int as __off_t, 0 as libc::c_int);
            memset(
                buffer as *mut libc::c_void,
                0 as libc::c_int,
                (*(*axel).conf).buffer_size as libc::c_ulong,
            );
            let mut j: off_t = (*axel).size;
            while j > 0 as libc::c_int as libc::c_long {
                let mut nwrite: ssize_t = 0;
                nwrite = write(
                    (*axel).outfd,
                    buffer as *const libc::c_void,
                    ({
                        let mut __a: off_t = j;
                        let mut __b: libc::c_int = (*(*axel).conf).buffer_size;
                        (if __a < __b as libc::c_long {
                            __a
                        } else {
                            __b as libc::c_long
                        })
                    }) as size_t,
                );
                if nwrite < 0 as libc::c_int as libc::c_long {
                    if *__errno_location() == 4 as libc::c_int
                        || *__errno_location() == 11 as libc::c_int
                    {
                        continue;
                    }
                    axel_message(
                        axel,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"Error creating local file\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                    return 0 as libc::c_int;
                } else {
                    j -= nwrite;
                }
            }
        }
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn reactivate_connection(
    mut axel: *mut axel_t,
    mut thread: libc::c_int,
) {
    let mut max_remaining: off_t = (100 as libc::c_int * 1024 as libc::c_int
        - 1 as libc::c_int) as off_t;
    let mut idx: libc::c_int = -(1 as libc::c_int);
    if (*((*axel).conn).offset(thread as isize)).enabled as libc::c_int != 0
        || (*((*axel).conn).offset(thread as isize)).currentbyte
            < (*((*axel).conn).offset(thread as isize)).lastbyte
    {
        return;
    }
    let mut j: libc::c_int = 0 as libc::c_int;
    while j < (*(*axel).conf).num_connections as libc::c_int {
        let mut remaining: off_t = (*((*axel).conn).offset(j as isize)).lastbyte
            - (*((*axel).conn).offset(j as isize)).currentbyte;
        if remaining > max_remaining {
            max_remaining = remaining;
            idx = j;
        }
        j += 1;
        j;
    }
    if idx == -(1 as libc::c_int) {
        return;
    }
    (*((*axel).conn).offset(thread as isize))
        .lastbyte = (*((*axel).conn).offset(idx as isize)).lastbyte;
    (*((*axel).conn).offset(idx as isize))
        .lastbyte = (*((*axel).conn).offset(idx as isize)).currentbyte
        + max_remaining / 2 as libc::c_int as libc::c_long;
    (*((*axel).conn).offset(thread as isize))
        .currentbyte = (*((*axel).conn).offset(idx as isize)).lastbyte;
}
pub unsafe extern "C" fn axel_start(mut axel: *mut axel_t) {
    let mut i: libc::c_int = 0;
    let mut url_ptr: *mut url_t = 0 as *mut url_t;
    url_ptr = (*axel).url;
    i = 0 as libc::c_int;
    while i < (*(*axel).conf).num_connections as libc::c_int {
        conn_set(
            &mut *((*axel).conn).offset(i as isize),
            ((*url_ptr).text).as_mut_ptr(),
        );
        url_ptr = (*url_ptr).next as *mut url_t;
        let ref mut fresh4 = (*((*axel).conn).offset(i as isize)).local_if;
        *fresh4 = ((*(*(*axel).conf).interfaces).text).as_mut_ptr();
        (*(*axel).conf).interfaces = (*(*(*axel).conf).interfaces).next as *mut if_t;
        let ref mut fresh5 = (*((*axel).conn).offset(i as isize)).conf;
        *fresh5 = (*axel).conf;
        if i != 0 {
            (*((*axel).conn).offset(i as isize)).supported = 1 as libc::c_int != 0;
        }
        i += 1;
        i;
    }
    if (*(*axel).conf).verbose > 0 as libc::c_int {
        axel_message(
            axel,
            dcgettext(
                0 as *const libc::c_char,
                b"Starting download\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    i = 0 as libc::c_int;
    while i < (*(*axel).conf).num_connections as libc::c_int {
        if (*((*axel).conn).offset(i as isize)).currentbyte
            >= (*((*axel).conn).offset(i as isize)).lastbyte
        {
            pthread_mutex_lock(&mut (*((*axel).conn).offset(i as isize)).lock);
            reactivate_connection(axel, i);
            pthread_mutex_unlock(&mut (*((*axel).conn).offset(i as isize)).lock);
        } else if (*((*axel).conn).offset(i as isize)).currentbyte
            < (*((*axel).conn).offset(i as isize)).lastbyte
        {
            if (*(*axel).conf).verbose >= 2 as libc::c_int {
                axel_message(
                    axel,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Connection %i downloading from %s:%i using interface %s\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    i,
                    ((*((*axel).conn).offset(i as isize)).host).as_mut_ptr(),
                    (*((*axel).conn).offset(i as isize)).port,
                    (*((*axel).conn).offset(i as isize)).local_if,
                );
            }
            (*((*axel).conn).offset(i as isize)).state = 1 as libc::c_int != 0;
            if pthread_create(
                ((*((*axel).conn).offset(i as isize)).setup_thread).as_mut_ptr(),
                0 as *const pthread_attr_t,
                Some(
                    setup_thread
                        as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
                ),
                &mut *((*axel).conn).offset(i as isize) as *mut conn_t
                    as *mut libc::c_void,
            ) != 0 as libc::c_int
            {
                axel_message(
                    axel,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"pthread error!!!\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                (*axel).ready = -(1 as libc::c_int);
            }
        }
        i += 1;
        i;
    }
    (*axel).start_time = axel_gettime();
    (*axel).ready = 0 as libc::c_int;
}
pub unsafe extern "C" fn axel_do(mut axel: *mut axel_t) {
    let mut fds: [fd_set; 1] = [fd_set { fds_bits: [0; 16] }; 1];
    let mut hifd: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut remaining: off_t = 0;
    let mut size: off_t = 0;
    let mut timeval: [timeval; 1] = [timeval { tv_sec: 0, tv_usec: 0 }; 1];
    let mut url_ptr: *mut url_t = 0 as *mut url_t;
    let mut delay: timespec = {
        let mut init = timespec {
            tv_sec: 0 as libc::c_int as __time_t,
            tv_nsec: 100000000 as libc::c_int as __syscall_slong_t,
        };
        init
    };
    let mut max_speed_ratio: libc::c_ulonglong = 0;
    if axel_gettime() > (*axel).next_state as libc::c_double {
        save_state(axel);
        (*axel)
            .next_state = (axel_gettime()
            + (*(*axel).conf).save_state_interval as libc::c_double) as libc::c_int;
    }
    let mut __d0: libc::c_int = 0;
    let mut __d1: libc::c_int = 0;
    let fresh6 = &mut __d0;
    let fresh7;
    let fresh8 = (::std::mem::size_of::<fd_set>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<__fd_mask>() as libc::c_ulong);
    let fresh9 = &mut __d1;
    let fresh10;
    let fresh11 = &mut *((*fds.as_mut_ptr()).fds_bits)
        .as_mut_ptr()
        .offset(0 as libc::c_int as isize) as *mut __fd_mask;
    asm!(
        "cld; rep; stosq", inlateout("cx") c2rust_asm_casts::AsmCast::cast_in(fresh6,
        fresh8) => fresh7, inlateout("di") c2rust_asm_casts::AsmCast::cast_in(fresh9,
        fresh11) => fresh10, inlateout("ax") 0 as libc::c_int => _,
        options(preserves_flags, att_syntax)
    );
    c2rust_asm_casts::AsmCast::cast_out(fresh6, fresh8, fresh7);
    c2rust_asm_casts::AsmCast::cast_out(fresh9, fresh11, fresh10);
    hifd = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < (*(*axel).conf).num_connections as libc::c_int {
        if pthread_mutex_trylock(&mut (*((*axel).conn).offset(i as isize)).lock) == 0 {
            if (*((*axel).conn).offset(i as isize)).enabled {
                let ref mut fresh12 = (*fds.as_mut_ptr())
                    .fds_bits[((*(*((*axel).conn).offset(i as isize)).tcp).fd
                    / (8 as libc::c_int
                        * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                            as libc::c_int)) as usize];
                *fresh12
                    |= ((1 as libc::c_ulong)
                        << (*(*((*axel).conn).offset(i as isize)).tcp).fd
                            % (8 as libc::c_int
                                * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                    as libc::c_int)) as __fd_mask;
                hifd = ({
                    let mut __a: libc::c_int = hifd;
                    let mut __b: libc::c_int = (*(*((*axel).conn).offset(i as isize))
                        .tcp)
                        .fd;
                    if __a > __b { __a } else { __b }
                });
            }
            pthread_mutex_unlock(&mut (*((*axel).conn).offset(i as isize)).lock);
        }
        i += 1;
        i;
    }
    if hifd == 0 as libc::c_int {
        if axel_sleep(delay) < 0 as libc::c_int {
            axel_message(
                axel,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Error while waiting for connection: %s\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                strerror(*__errno_location()),
            );
            (*axel).ready = -(1 as libc::c_int);
            return;
        }
    } else {
        (*timeval.as_mut_ptr()).tv_sec = 0 as libc::c_int as __time_t;
        (*timeval.as_mut_ptr()).tv_usec = 100000 as libc::c_int as __suseconds_t;
        if select(
            hifd + 1 as libc::c_int,
            fds.as_mut_ptr(),
            0 as *mut fd_set,
            0 as *mut fd_set,
            timeval.as_mut_ptr(),
        ) == -(1 as libc::c_int)
        {
            (*axel).ready = -(1 as libc::c_int);
            return;
        }
        i = 0 as libc::c_int;
        while i < (*(*axel).conf).num_connections as libc::c_int {
            if !(pthread_mutex_trylock(&mut (*((*axel).conn).offset(i as isize)).lock)
                != 0)
            {
                if (*((*axel).conn).offset(i as isize)).enabled {
                    if !((*fds.as_mut_ptr())
                        .fds_bits[((*(*((*axel).conn).offset(i as isize)).tcp).fd
                        / (8 as libc::c_int
                            * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                as libc::c_int)) as usize]
                        & ((1 as libc::c_ulong)
                            << (*(*((*axel).conn).offset(i as isize)).tcp).fd
                                % (8 as libc::c_int
                                    * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                        as libc::c_int)) as __fd_mask
                        != 0 as libc::c_int as libc::c_long)
                    {
                        let mut timeout: time_t = ((*((*axel).conn).offset(i as isize))
                            .last_transfer + (*(*axel).conf).connection_timeout)
                            as time_t;
                        if axel_gettime() > timeout as libc::c_double {
                            if (*(*axel).conf).verbose != 0 {
                                axel_message(
                                    axel,
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"Connection %i timed out\0" as *const u8
                                            as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                    i,
                                );
                            }
                            conn_disconnect(&mut *((*axel).conn).offset(i as isize));
                        }
                    } else {
                        (*((*axel).conn).offset(i as isize))
                            .last_transfer = axel_gettime() as libc::c_int;
                        size = tcp_read(
                            (*((*axel).conn).offset(i as isize)).tcp,
                            buffer as *mut libc::c_void,
                            (*(*axel).conf).buffer_size,
                        );
                        if size == -(1 as libc::c_int) as libc::c_long {
                            if (*(*axel).conf).verbose != 0 {
                                axel_message(
                                    axel,
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"Error on connection %i! Connection closed\0" as *const u8
                                            as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                    i,
                                );
                            }
                            conn_disconnect(&mut *((*axel).conn).offset(i as isize));
                        } else if size == 0 as libc::c_int as libc::c_long {
                            if (*(*axel).conf).verbose != 0 {
                                if (*((*axel).conn).offset(i as isize)).currentbyte
                                    < (*((*axel).conn).offset(i as isize)).lastbyte
                                    && (*axel).size as libc::c_longlong
                                        != 9223372036854775807 as libc::c_longlong
                                {
                                    axel_message(
                                        axel,
                                        dcgettext(
                                            0 as *const libc::c_char,
                                            b"Connection %i unexpectedly closed\0" as *const u8
                                                as *const libc::c_char,
                                            5 as libc::c_int,
                                        ),
                                        i,
                                    );
                                } else {
                                    axel_message(
                                        axel,
                                        dcgettext(
                                            0 as *const libc::c_char,
                                            b"Connection %i finished\0" as *const u8
                                                as *const libc::c_char,
                                            5 as libc::c_int,
                                        ),
                                        i,
                                    );
                                }
                            }
                            if !(*((*axel).conn).offset(0 as libc::c_int as isize))
                                .supported
                            {
                                (*axel).ready = 1 as libc::c_int;
                            }
                            conn_disconnect(&mut *((*axel).conn).offset(i as isize));
                            reactivate_connection(axel, i);
                        } else {
                            remaining = (*((*axel).conn).offset(i as isize)).lastbyte
                                - (*((*axel).conn).offset(i as isize)).currentbyte;
                            if remaining < size {
                                if (*(*axel).conf).verbose != 0 {
                                    axel_message(
                                        axel,
                                        dcgettext(
                                            0 as *const libc::c_char,
                                            b"Connection %i finished\0" as *const u8
                                                as *const libc::c_char,
                                            5 as libc::c_int,
                                        ),
                                        i,
                                    );
                                }
                                conn_disconnect(&mut *((*axel).conn).offset(i as isize));
                                size = remaining;
                            }
                            lseek(
                                (*axel).outfd,
                                (*((*axel).conn).offset(i as isize)).currentbyte,
                                0 as libc::c_int,
                            );
                            if write(
                                (*axel).outfd,
                                buffer as *const libc::c_void,
                                size as size_t,
                            ) != size
                            {
                                axel_message(
                                    axel,
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"Write error!\0" as *const u8 as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                );
                                (*axel).ready = -(1 as libc::c_int);
                                pthread_mutex_unlock(
                                    &mut (*((*axel).conn).offset(i as isize)).lock,
                                );
                                return;
                            }
                            let ref mut fresh13 = (*((*axel).conn).offset(i as isize))
                                .currentbyte;
                            *fresh13 += size;
                            (*axel).bytes_done += size;
                            if remaining == size {
                                reactivate_connection(axel, i);
                            }
                        }
                    }
                }
                pthread_mutex_unlock(&mut (*((*axel).conn).offset(i as isize)).lock);
            }
            i += 1;
            i;
        }
        if (*axel).ready != 0 {
            return;
        }
    }
    url_ptr = (*axel).url;
    i = 0 as libc::c_int;
    while i < (*(*axel).conf).num_connections as libc::c_int {
        if !(pthread_mutex_trylock(&mut (*((*axel).conn).offset(i as isize)).lock) != 0)
        {
            if !(*((*axel).conn).offset(i as isize)).enabled
                && (*((*axel).conn).offset(i as isize)).currentbyte
                    < (*((*axel).conn).offset(i as isize)).lastbyte
            {
                if !(*((*axel).conn).offset(i as isize)).state {
                    pthread_join(
                        *((*((*axel).conn).offset(i as isize)).setup_thread)
                            .as_mut_ptr(),
                        0 as *mut *mut libc::c_void,
                    );
                    conn_set(
                        &mut *((*axel).conn).offset(i as isize),
                        ((*url_ptr).text).as_mut_ptr(),
                    );
                    url_ptr = (*url_ptr).next as *mut url_t;
                    if (*(*axel).conf).verbose >= 2 as libc::c_int {
                        axel_message(
                            axel,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"Connection %i downloading from %s:%i using interface %s\0"
                                    as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            i,
                            ((*((*axel).conn).offset(i as isize)).host).as_mut_ptr(),
                            (*((*axel).conn).offset(i as isize)).port,
                            (*((*axel).conn).offset(i as isize)).local_if,
                        );
                    }
                    (*((*axel).conn).offset(i as isize)).state = 1 as libc::c_int != 0;
                    if pthread_create(
                        ((*((*axel).conn).offset(i as isize)).setup_thread).as_mut_ptr(),
                        0 as *const pthread_attr_t,
                        Some(
                            setup_thread
                                as unsafe extern "C" fn(
                                    *mut libc::c_void,
                                ) -> *mut libc::c_void,
                        ),
                        &mut *((*axel).conn).offset(i as isize) as *mut conn_t
                            as *mut libc::c_void,
                    ) == 0 as libc::c_int
                    {
                        (*((*axel).conn).offset(i as isize))
                            .last_transfer = axel_gettime() as libc::c_int;
                    } else {
                        axel_message(
                            axel,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"pthread error!!!\0" as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                        );
                        (*axel).ready = -(1 as libc::c_int);
                    }
                } else if axel_gettime()
                    > ((*((*axel).conn).offset(i as isize)).last_transfer
                        + (*(*axel).conf).reconnect_delay) as libc::c_double
                {
                    pthread_cancel(
                        *((*((*axel).conn).offset(i as isize)).setup_thread).as_mut_ptr(),
                    );
                    (*((*axel).conn).offset(i as isize)).state = 0 as libc::c_int != 0;
                    pthread_join(
                        *((*((*axel).conn).offset(i as isize)).setup_thread)
                            .as_mut_ptr(),
                        0 as *mut *mut libc::c_void,
                    );
                }
            }
            pthread_mutex_unlock(&mut (*((*axel).conn).offset(i as isize)).lock);
        }
        i += 1;
        i;
    }
    (*axel)
        .bytes_per_second = (((*axel).bytes_done - (*axel).start_byte) as libc::c_double
        / (axel_gettime() - (*axel).start_time)) as off_t as libc::c_longlong;
    if (*axel).bytes_per_second != 0 as libc::c_int as libc::c_longlong {
        (*axel)
            .finish_time = ((*axel).start_time
            + ((*axel).size - (*axel).start_byte) as libc::c_double
                / (*axel).bytes_per_second as libc::c_double) as libc::c_int;
    } else {
        (*axel).finish_time = 2147483647 as libc::c_int;
    }
    if (*(*axel).conf).max_speed > 0 as libc::c_int as libc::c_ulonglong {
        max_speed_ratio = ((1000 as libc::c_int as libc::c_longlong
            * (*axel).bytes_per_second) as libc::c_ulonglong)
            .wrapping_div((*(*axel).conf).max_speed);
        if max_speed_ratio > 1050 as libc::c_int as libc::c_ulonglong {
            (*axel).delay_time.tv_nsec += 10000000 as libc::c_int as libc::c_long;
            if (*axel).delay_time.tv_nsec >= 1000000000 as libc::c_int as libc::c_long {
                (*axel).delay_time.tv_sec += 1;
                (*axel).delay_time.tv_sec;
                (*axel).delay_time.tv_nsec -= 1000000000 as libc::c_int as libc::c_long;
            }
        } else if max_speed_ratio < 950 as libc::c_int as libc::c_ulonglong {
            if (*axel).delay_time.tv_nsec >= 10000000 as libc::c_int as libc::c_long {
                (*axel).delay_time.tv_nsec -= 10000000 as libc::c_int as libc::c_long;
            } else if (*axel).delay_time.tv_sec > 0 as libc::c_int as libc::c_long {
                (*axel).delay_time.tv_sec -= 1;
                (*axel).delay_time.tv_sec;
                (*axel).delay_time.tv_nsec += 999000000 as libc::c_int as libc::c_long;
            } else {
                (*axel).delay_time.tv_sec = 0 as libc::c_int as __time_t;
                (*axel).delay_time.tv_nsec = 0 as libc::c_int as __syscall_slong_t;
            }
        }
        if axel_sleep((*axel).delay_time) < 0 as libc::c_int {
            axel_message(
                axel,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Error while enforcing throttling: %s\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                strerror(*__errno_location()),
            );
            (*axel).ready = -(1 as libc::c_int);
            return;
        }
    }
    if (*axel).bytes_done == (*axel).size {
        (*axel).ready = 1 as libc::c_int;
    }
}
pub unsafe extern "C" fn axel_close(mut axel: *mut axel_t) {
    if axel.is_null() {
        return;
    }
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < (*(*axel).conf).num_connections as libc::c_int {
        if *((*((*axel).conn).offset(i as isize)).setup_thread).as_mut_ptr()
            != 0 as libc::c_int as libc::c_ulong
        {
            pthread_cancel(
                *((*((*axel).conn).offset(i as isize)).setup_thread).as_mut_ptr(),
            );
            pthread_join(
                *((*((*axel).conn).offset(i as isize)).setup_thread).as_mut_ptr(),
                0 as *mut *mut libc::c_void,
            );
        }
        conn_disconnect(&mut *((*axel).conn).offset(i as isize));
        i += 1;
        i;
    }
    free((*axel).url as *mut libc::c_void);
    if (*axel).ready == 1 as libc::c_int {
        stfile_unlink(((*axel).filename).as_mut_ptr());
    } else if (*axel).bytes_done > 0 as libc::c_int as libc::c_long {
        save_state(axel);
    }
    print_messages(axel);
    close((*axel).outfd);
    if !((*(*axel).conn).proto & (1 as libc::c_int) << 1 as libc::c_int
        == (0 as libc::c_int) << 1 as libc::c_int) || (*(*axel).conn).proxy != 0
    {
        abuf_setup(
            ((*((*(*axel).conn).http).as_mut_ptr()).request).as_mut_ptr(),
            0 as libc::c_int as size_t,
        );
        abuf_setup(
            ((*((*(*axel).conn).http).as_mut_ptr()).headers).as_mut_ptr(),
            0 as libc::c_int as size_t,
        );
    }
    free((*axel).conn as *mut libc::c_void);
    free(axel as *mut libc::c_void);
    free(buffer as *mut libc::c_void);
}
pub unsafe extern "C" fn axel_gettime() -> libc::c_double {
    let mut time: [timeval; 1] = [timeval { tv_sec: 0, tv_usec: 0 }; 1];
    gettimeofday(time.as_mut_ptr(), 0 as *mut libc::c_void);
    return (*time.as_mut_ptr()).tv_sec as libc::c_double
        + (*time.as_mut_ptr()).tv_usec as libc::c_double
            / 1000000 as libc::c_int as libc::c_double;
}
unsafe extern "C" fn save_state(mut axel: *mut axel_t) {
    if !(*((*axel).conn).offset(0 as libc::c_int as isize)).supported {
        return;
    }
    let mut fd: libc::c_int = 0;
    fd = stfile_open(
        ((*axel).filename).as_mut_ptr(),
        0o100 as libc::c_int | 0o1000 as libc::c_int | 0o1 as libc::c_int,
        0o666 as libc::c_int as mode_t,
    );
    if fd == -(1 as libc::c_int) {
        return;
    }
    let mut nwrite: ssize_t = 0;
    nwrite = write(
        fd,
        &mut (*(*axel).conf).num_connections as *mut uint16_t as *const libc::c_void,
        ::std::mem::size_of::<uint16_t>() as libc::c_ulong,
    );
    nwrite = write(
        fd,
        &mut (*axel).bytes_done as *mut off_t as *const libc::c_void,
        ::std::mem::size_of::<off_t>() as libc::c_ulong,
    );
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < (*(*axel).conf).num_connections as libc::c_int {
        nwrite = write(
            fd,
            &mut (*((*axel).conn).offset(i as isize)).currentbyte as *mut off_t
                as *const libc::c_void,
            ::std::mem::size_of::<off_t>() as libc::c_ulong,
        );
        nwrite = write(
            fd,
            &mut (*((*axel).conn).offset(i as isize)).lastbyte as *mut off_t
                as *const libc::c_void,
            ::std::mem::size_of::<off_t>() as libc::c_ulong,
        );
        i += 1;
        i;
    }
    close(fd);
}
unsafe extern "C" fn setup_thread(mut c: *mut libc::c_void) -> *mut libc::c_void {
    let mut current_block: u64;
    let mut conn: *mut conn_t = c as *mut conn_t;
    let mut oldstate: libc::c_int = 0;
    pthread_setcancelstate(PTHREAD_CANCEL_ENABLE as libc::c_int, &mut oldstate);
    pthread_setcanceltype(PTHREAD_CANCEL_ASYNCHRONOUS as libc::c_int, &mut oldstate);
    pthread_mutex_lock(&mut (*conn).lock);
    if conn_setup(conn) != 0 {
        (*conn).last_transfer = axel_gettime() as libc::c_int;
        if conn_exec(conn) != 0 {
            (*conn).last_transfer = axel_gettime() as libc::c_int;
            (*conn).enabled = 1 as libc::c_int != 0;
            current_block = 1021413192665002230;
        } else {
            current_block = 10879442775620481940;
        }
    } else {
        current_block = 10879442775620481940;
    }
    match current_block {
        10879442775620481940 => {
            conn_disconnect(conn);
        }
        _ => {}
    }
    (*conn).state = 0 as libc::c_int != 0;
    pthread_mutex_unlock(&mut (*conn).lock);
    return 0 as *mut libc::c_void;
}
unsafe extern "C" fn axel_message(
    mut axel: *mut axel_t,
    mut format: *const libc::c_char,
    mut args: ...
) {
    let mut m: *mut message_t = 0 as *mut message_t;
    let mut params: ::std::ffi::VaListImpl;
    if !axel.is_null() {
        m = calloc(
            1 as libc::c_int as libc::c_ulong,
            ::std::mem::size_of::<message_t>() as libc::c_ulong,
        ) as *mut message_t;
        if !m.is_null() {
            params = args.clone();
            vsnprintf(
                ((*m).text).as_mut_ptr(),
                1024 as libc::c_int as size_t,
                format,
                params.as_va_list(),
            );
            if ((*axel).message).is_null() {
                (*axel).last_message = m;
                (*axel).message = (*axel).last_message;
            } else {
                (*(*axel).last_message).next = m as *mut libc::c_void;
                (*axel).last_message = m;
            }
            return;
        }
    }
    print_messages(axel);
    params = args.clone();
    vprintf(format, params.as_va_list());
}
unsafe extern "C" fn axel_divide(mut axel: *mut axel_t) {
    let mut maxconns: off_t = ({
        let mut __a: libc::c_uint = 1 as libc::c_uint;
        let mut __b: libc::c_long = (*axel).size
            / (100 as libc::c_int * 1024 as libc::c_int) as libc::c_long;
        if __a as libc::c_long > __b { __a as libc::c_long } else { __b }
    });
    if maxconns < (*(*axel).conf).num_connections as libc::c_long {
        (*(*axel).conf).num_connections = maxconns as uint16_t;
    }
    let mut seg_len: off_t = (*axel).size
        / (*(*axel).conf).num_connections as libc::c_long;
    if seg_len == 0 {
        printf(
            dcgettext(
                0 as *const libc::c_char,
                b"Too few bytes remaining, forcing a single connection\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        (*(*axel).conf).num_connections = 1 as libc::c_int as uint16_t;
        seg_len = (*axel).size;
        let mut new_conn: *mut conn_t = realloc(
            (*axel).conn as *mut libc::c_void,
            ::std::mem::size_of::<conn_t>() as libc::c_ulong,
        ) as *mut conn_t;
        if !new_conn.is_null() {
            (*axel).conn = new_conn;
        }
    }
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < (*(*axel).conf).num_connections as libc::c_int {
        (*((*axel).conn).offset(i as isize)).currentbyte = seg_len * i as libc::c_long;
        (*((*axel).conn).offset(i as isize))
            .lastbyte = seg_len * i as libc::c_long + seg_len;
        i += 1;
        i;
    }
    let mut tail: size_t = ((*axel).size % seg_len) as size_t;
    let ref mut fresh14 = (*((*axel).conn)
        .offset(
            ((*(*axel).conf).num_connections as libc::c_int - 1 as libc::c_int) as isize,
        ))
        .lastbyte;
    *fresh14 = (*fresh14 as libc::c_ulong).wrapping_add(tail) as off_t as off_t;
}
