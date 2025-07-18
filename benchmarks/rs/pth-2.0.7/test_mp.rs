use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type pth_st;
    pub type pth_attr_st;
    pub type pth_event_st;
    pub type pth_msgport_st;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn perror(__s: *const libc::c_char);
    fn free(__ptr: *mut libc::c_void);
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn exit(_: libc::c_int) -> !;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
    fn time(__timer: *mut time_t) -> time_t;
    fn ctime(__timer: *const time_t) -> *mut libc::c_char;
    fn pth_init() -> libc::c_int;
    fn pth_kill() -> libc::c_int;
    fn pth_attr_new() -> pth_attr_t;
    fn pth_attr_set(_: pth_attr_t, _: libc::c_int, _: ...) -> libc::c_int;
    fn pth_attr_destroy(_: pth_attr_t) -> libc::c_int;
    fn pth_spawn(
        _: pth_attr_t,
        _: Option::<unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void>,
        _: *mut libc::c_void,
    ) -> pth_t;
    fn pth_yield(_: pth_t) -> libc::c_int;
    fn pth_wait(_: pth_event_t) -> libc::c_int;
    fn pth_cancel(_: pth_t) -> libc::c_int;
    fn pth_join(_: pth_t, _: *mut *mut libc::c_void) -> libc::c_int;
    fn pth_timeout(_: libc::c_long, _: libc::c_long) -> pth_time_t;
    fn pth_event(_: libc::c_ulong, _: ...) -> pth_event_t;
    fn pth_event_status(_: pth_event_t) -> pth_status_t;
    fn pth_event_free(_: pth_event_t, _: libc::c_int) -> libc::c_int;
    fn pth_msgport_create(_: *const libc::c_char) -> pth_msgport_t;
    fn pth_msgport_destroy(_: pth_msgport_t);
    fn pth_msgport_find(_: *const libc::c_char) -> pth_msgport_t;
    fn pth_msgport_put(_: pth_msgport_t, _: *mut pth_message_t) -> libc::c_int;
    fn pth_msgport_get(_: pth_msgport_t) -> *mut pth_message_t;
    fn pth_msgport_reply(_: *mut pth_message_t) -> libc::c_int;
    fn pth_cleanup_push(
        _: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
        _: *mut libc::c_void,
    ) -> libc::c_int;
    fn pth_sleep(_: libc::c_uint) -> libc::c_uint;
    fn pth_readline_ev(
        _: libc::c_int,
        _: *mut libc::c_void,
        _: size_t,
        _: pth_event_t,
    ) -> ssize_t;
}
pub type size_t = libc::c_ulong;
pub type __int32_t = libc::c_int;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
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
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type pth_time_t = timeval;
pub type pth_t = *mut pth_st;
pub type pth_attr_t = *mut pth_attr_st;
pub type C2RustUnnamed = libc::c_uint;
pub const PTH_ATTR_BOUND: C2RustUnnamed = 14;
pub const PTH_ATTR_EVENTS: C2RustUnnamed = 13;
pub const PTH_ATTR_STATE: C2RustUnnamed = 12;
pub const PTH_ATTR_START_ARG: C2RustUnnamed = 11;
pub const PTH_ATTR_START_FUNC: C2RustUnnamed = 10;
pub const PTH_ATTR_TIME_RAN: C2RustUnnamed = 9;
pub const PTH_ATTR_TIME_LAST: C2RustUnnamed = 8;
pub const PTH_ATTR_TIME_SPAWN: C2RustUnnamed = 7;
pub const PTH_ATTR_DISPATCHES: C2RustUnnamed = 6;
pub const PTH_ATTR_STACK_ADDR: C2RustUnnamed = 5;
pub const PTH_ATTR_STACK_SIZE: C2RustUnnamed = 4;
pub const PTH_ATTR_CANCEL_STATE: C2RustUnnamed = 3;
pub const PTH_ATTR_JOINABLE: C2RustUnnamed = 2;
pub const PTH_ATTR_NAME: C2RustUnnamed = 1;
pub const PTH_ATTR_PRIO: C2RustUnnamed = 0;
pub type pth_event_t = *mut pth_event_st;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const PTH_FREE_ALL: C2RustUnnamed_0 = 1;
pub const PTH_FREE_THIS: C2RustUnnamed_0 = 0;
pub type pth_status_t = libc::c_uint;
pub const PTH_STATUS_FAILED: pth_status_t = 2;
pub const PTH_STATUS_OCCURRED: pth_status_t = 1;
pub const PTH_STATUS_PENDING: pth_status_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pth_ringnode_st {
    pub rn_next: *mut pth_ringnode_t,
    pub rn_prev: *mut pth_ringnode_t,
}
pub type pth_ringnode_t = pth_ringnode_st;
pub type pth_msgport_t = *mut pth_msgport_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pth_message_st {
    pub m_node: pth_ringnode_t,
    pub m_replyport: pth_msgport_t,
    pub m_size: libc::c_uint,
    pub m_data: *mut libc::c_void,
}
pub type pth_message_t = pth_message_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct query {
    pub head: pth_message_t,
    pub string: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct worker_cleanup_t {
    pub mp: pth_msgport_t,
    pub ev: pth_event_t,
}
#[inline]
unsafe extern "C" fn toupper(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_toupper_loc()).offset(__c as isize)
    } else {
        __c
    };
}
unsafe extern "C" fn worker_cleanup(mut arg: *mut libc::c_void) {
    let mut wc: *mut worker_cleanup_t = arg as *mut worker_cleanup_t;
    pth_event_free((*wc).ev, PTH_FREE_THIS as libc::c_int);
    pth_msgport_destroy((*wc).mp);
}
unsafe extern "C" fn worker(mut _dummy: *mut libc::c_void) -> *mut libc::c_void {
    let mut wc: worker_cleanup_t = worker_cleanup_t {
        mp: 0 as *mut pth_msgport_st,
        ev: 0 as *mut pth_event_st,
    };
    let mut mp: pth_msgport_t = 0 as *mut pth_msgport_st;
    let mut ev: pth_event_t = 0 as *mut pth_event_st;
    let mut q: *mut query = 0 as *mut query;
    let mut i: libc::c_int = 0;
    fprintf(stderr, b"worker: start\n\0" as *const u8 as *const libc::c_char);
    mp = pth_msgport_create(b"worker\0" as *const u8 as *const libc::c_char);
    wc.mp = mp;
    ev = pth_event(((1 as libc::c_int) << 5 as libc::c_int) as libc::c_ulong, mp);
    wc.ev = ev;
    pth_cleanup_push(
        Some(worker_cleanup as unsafe extern "C" fn(*mut libc::c_void) -> ()),
        &mut wc as *mut worker_cleanup_t as *mut libc::c_void,
    );
    loop {
        i = pth_wait(ev);
        if i != 1 as libc::c_int {
            continue;
        }
        loop {
            q = pth_msgport_get(mp) as *mut query;
            if q.is_null() {
                break;
            }
            fprintf(
                stderr,
                b"worker: recv query <%s>\n\0" as *const u8 as *const libc::c_char,
                (*q).string,
            );
            i = 0 as libc::c_int;
            while *((*q).string).offset(i as isize) as libc::c_int != '\0' as i32 {
                *((*q).string)
                    .offset(
                        i as isize,
                    ) = ({
                    let mut __res: libc::c_int = 0;
                    if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                        > 1 as libc::c_int as libc::c_ulong
                    {
                        if 0 != 0 {
                            let mut __c: libc::c_int = *((*q).string).offset(i as isize)
                                as libc::c_int;
                            __res = if __c < -(128 as libc::c_int)
                                || __c > 255 as libc::c_int
                            {
                                __c
                            } else {
                                *(*__ctype_toupper_loc()).offset(__c as isize)
                            };
                        } else {
                            __res = toupper(
                                *((*q).string).offset(i as isize) as libc::c_int,
                            );
                        }
                    } else {
                        __res = *(*__ctype_toupper_loc())
                            .offset(
                                *((*q).string).offset(i as isize) as libc::c_int as isize,
                            );
                    }
                    __res
                }) as libc::c_char;
                i += 1;
                i;
            }
            fprintf(
                stderr,
                b"worker: send reply <%s>\n\0" as *const u8 as *const libc::c_char,
                (*q).string,
            );
            pth_msgport_reply(q as *mut pth_message_t);
        }
    };
}
unsafe extern "C" fn ticker(mut _arg: *mut libc::c_void) -> *mut libc::c_void {
    let mut now: time_t = 0;
    fprintf(stderr, b"ticker: start\n\0" as *const u8 as *const libc::c_char);
    loop {
        pth_sleep(5 as libc::c_int as libc::c_uint);
        now = time(0 as *mut time_t);
        fprintf(
            stderr,
            b"ticker was woken up on %s\0" as *const u8 as *const libc::c_char,
            ctime(&mut now),
        );
    };
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut caLine: [libc::c_char; 1024] = [0; 1024];
    let mut ev: pth_event_t = 0 as pth_event_t;
    let mut evt: pth_event_t = 0 as pth_event_t;
    let mut t_worker: pth_t = 0 as pth_t;
    let mut t_ticker: pth_t = 0 as pth_t;
    let mut t_attr: pth_attr_t = 0 as *mut pth_attr_st;
    let mut mp: pth_msgport_t = 0 as pth_msgport_t;
    let mut mp_worker: pth_msgport_t = 0 as pth_msgport_t;
    let mut q: *mut query = 0 as *mut query;
    let mut n: libc::c_int = 0;
    if pth_init() == 0 {
        perror(b"pth_init\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    fprintf(
        stderr,
        b"This is TEST_MP, a Pth test using message ports.\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        stderr,
        b"Lines on stdin are send to a worker thread via message\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"ports, translated to upper case by the worker thread and\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"send back to the main thread via message ports.\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"Additionally a useless ticker thread awakens every 5s.\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"Enter \"quit\" on stdin for stopping this test.\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
    t_attr = pth_attr_new();
    pth_attr_set(
        t_attr,
        PTH_ATTR_NAME as libc::c_int,
        b"worker\0" as *const u8 as *const libc::c_char,
    );
    pth_attr_set(
        t_attr,
        PTH_ATTR_JOINABLE as libc::c_int,
        (0 as libc::c_int == 0) as libc::c_int,
    );
    pth_attr_set(
        t_attr,
        PTH_ATTR_STACK_SIZE as libc::c_int,
        16 as libc::c_int * 1024 as libc::c_int,
    );
    t_worker = pth_spawn(
        t_attr,
        Some(worker as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void),
        0 as *mut libc::c_void,
    );
    pth_attr_set(
        t_attr,
        PTH_ATTR_NAME as libc::c_int,
        b"ticker\0" as *const u8 as *const libc::c_char,
    );
    t_ticker = pth_spawn(
        t_attr,
        Some(ticker as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void),
        0 as *mut libc::c_void,
    );
    pth_attr_destroy(t_attr);
    pth_yield(0 as pth_t);
    mp_worker = pth_msgport_find(b"worker\0" as *const u8 as *const libc::c_char);
    mp = pth_msgport_create(b"main\0" as *const u8 as *const libc::c_char);
    q = malloc(::std::mem::size_of::<query>() as libc::c_ulong) as *mut query;
    ev = pth_event(((1 as libc::c_int) << 5 as libc::c_int) as libc::c_ulong, mp);
    evt = 0 as pth_event_t;
    loop {
        if evt.is_null() {
            evt = pth_event(
                ((1 as libc::c_int) << 4 as libc::c_int) as libc::c_ulong,
                pth_timeout(
                    20 as libc::c_int as libc::c_long,
                    0 as libc::c_int as libc::c_long,
                ),
            );
        } else {
            evt = pth_event(
                ((1 as libc::c_int) << 4 as libc::c_int
                    | (1 as libc::c_int) << 20 as libc::c_int) as libc::c_ulong,
                evt,
                pth_timeout(
                    20 as libc::c_int as libc::c_long,
                    0 as libc::c_int as libc::c_long,
                ),
            );
        }
        n = pth_readline_ev(
            0 as libc::c_int,
            caLine.as_mut_ptr() as *mut libc::c_void,
            1024 as libc::c_int as size_t,
            evt,
        ) as libc::c_int;
        if n == -(1 as libc::c_int)
            && pth_event_status(evt) as libc::c_uint
                == PTH_STATUS_OCCURRED as libc::c_int as libc::c_uint
        {
            fprintf(
                stderr,
                b"main: Hey, what are you waiting for? Type in something!\n\0"
                    as *const u8 as *const libc::c_char,
            );
        } else if n < 0 as libc::c_int {
            fprintf(
                stderr,
                b"main: I/O read error on stdin\n\0" as *const u8 as *const libc::c_char,
            );
            break;
        } else if n == 0 as libc::c_int {
            fprintf(
                stderr,
                b"main: EOF on stdin\n\0" as *const u8 as *const libc::c_char,
            );
            break;
        } else {
            caLine[(n - 1 as libc::c_int) as usize] = '\0' as i32 as libc::c_char;
            if strcmp(caLine.as_mut_ptr(), b"quit\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                fprintf(stderr, b"main: quit\n\0" as *const u8 as *const libc::c_char);
                break;
            } else {
                fprintf(
                    stderr,
                    b"main: out --> <%s>\n\0" as *const u8 as *const libc::c_char,
                    caLine.as_mut_ptr(),
                );
                (*q).string = caLine.as_mut_ptr();
                (*q).head.m_replyport = mp;
                pth_msgport_put(mp_worker, q as *mut pth_message_t);
                pth_wait(ev);
                q = pth_msgport_get(mp) as *mut query;
                fprintf(
                    stderr,
                    b"main: in <-- <%s>\n\0" as *const u8 as *const libc::c_char,
                    (*q).string,
                );
            }
        }
    }
    free(q as *mut libc::c_void);
    pth_event_free(ev, PTH_FREE_THIS as libc::c_int);
    pth_event_free(evt, PTH_FREE_THIS as libc::c_int);
    pth_msgport_destroy(mp);
    pth_cancel(t_worker);
    pth_join(t_worker, 0 as *mut *mut libc::c_void);
    pth_cancel(t_ticker);
    pth_join(t_ticker, 0 as *mut *mut libc::c_void);
    pth_kill();
    return 0 as libc::c_int;
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
