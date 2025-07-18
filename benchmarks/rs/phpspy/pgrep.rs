use ::libc;
use ::c2rust_asm_casts;
use c2rust_asm_casts::AsmCastTrait;
use std::arch::asm;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
    fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> libc::c_int;
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
    fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> libc::c_int;
    fn pthread_mutex_destroy(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_cond_init(
        __cond: *mut pthread_cond_t,
        __cond_attr: *const pthread_condattr_t,
    ) -> libc::c_int;
    fn pthread_cond_destroy(__cond: *mut pthread_cond_t) -> libc::c_int;
    fn pthread_cond_signal(__cond: *mut pthread_cond_t) -> libc::c_int;
    fn pthread_cond_broadcast(__cond: *mut pthread_cond_t) -> libc::c_int;
    fn pthread_cond_timedwait(
        __cond: *mut pthread_cond_t,
        __mutex: *mut pthread_mutex_t,
        __abstime: *const timespec,
    ) -> libc::c_int;
    fn sigfillset(__set: *mut sigset_t) -> libc::c_int;
    fn sigprocmask(
        __how: libc::c_int,
        __set: *const sigset_t,
        __oset: *mut sigset_t,
    ) -> libc::c_int;
    fn sigaction(
        __sig: libc::c_int,
        __act: *const sigaction,
        __oact: *mut sigaction,
    ) -> libc::c_int;
    fn asprintf(
        __ptr: *mut *mut libc::c_char,
        __fmt: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn fgets(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
    fn perror(__s: *const libc::c_char);
    fn popen(__command: *const libc::c_char, __modes: *const libc::c_char) -> *mut FILE;
    fn pclose(__stream: *mut FILE) -> libc::c_int;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn select(
        __nfds: libc::c_int,
        __readfds: *mut fd_set,
        __writefds: *mut fd_set,
        __exceptfds: *mut fd_set,
        __timeout: *mut timeval,
    ) -> libc::c_int;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __errno_location() -> *mut libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn pipe(__pipedes: *mut libc::c_int) -> libc::c_int;
    fn alarm(__seconds: libc::c_uint) -> libc::c_uint;
    static mut opt_pgrep_args: *mut libc::c_char;
    static mut done: libc::c_int;
    static mut opt_num_workers: libc::c_int;
    static mut opt_time_limit_ms: libc::c_long;
    fn log_error(fmt: *const libc::c_char, _: ...);
    fn main_pid(pid: pid_t) -> libc::c_int;
}
pub type __uint32_t = libc::c_uint;
pub type __uid_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __clock_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __clockid_t = libc::c_int;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type pid_t = __pid_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type clockid_t = __clockid_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_attr_t {
    pub __size: [libc::c_char; 56],
    pub __align: libc::c_long,
}
pub type __sigval_t = sigval;
#[derive(Copy, Clone)]
#[repr(C)]
pub union sigval {
    pub sival_int: libc::c_int,
    pub sival_ptr: *mut libc::c_void,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
pub type sigset_t = __sigset_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct siginfo_t {
    pub si_signo: libc::c_int,
    pub si_errno: libc::c_int,
    pub si_code: libc::c_int,
    pub __pad0: libc::c_int,
    pub _sifields: C2RustUnnamed_4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
    pub _pad: [libc::c_int; 28],
    pub _kill: C2RustUnnamed_13,
    pub _timer: C2RustUnnamed_12,
    pub _rt: C2RustUnnamed_11,
    pub _sigchld: C2RustUnnamed_10,
    pub _sigfault: C2RustUnnamed_7,
    pub _sigpoll: C2RustUnnamed_6,
    pub _sigsys: C2RustUnnamed_5,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub _call_addr: *mut libc::c_void,
    pub _syscall: libc::c_int,
    pub _arch: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub si_band: libc::c_long,
    pub si_fd: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub si_addr: *mut libc::c_void,
    pub si_addr_lsb: libc::c_short,
    pub _bounds: C2RustUnnamed_8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_8 {
    pub _addr_bnd: C2RustUnnamed_9,
    pub _pkey: __uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub _lower: *mut libc::c_void,
    pub _upper: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_status: libc::c_int,
    pub si_utime: __clock_t,
    pub si_stime: __clock_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_11 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_12 {
    pub si_tid: libc::c_int,
    pub si_overrun: libc::c_int,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_13 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
}
pub type __sighandler_t = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sigaction {
    pub __sigaction_handler: C2RustUnnamed_14,
    pub sa_mask: __sigset_t,
    pub sa_flags: libc::c_int,
    pub sa_restorer: Option::<unsafe extern "C" fn() -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_14 {
    pub sa_handler: __sighandler_t,
    pub sa_sigaction: Option::<
        unsafe extern "C" fn(libc::c_int, *mut siginfo_t, *mut libc::c_void) -> (),
    >,
}
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
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
static mut avail_pids: *mut libc::c_int = 0 as *const libc::c_int as *mut libc::c_int;
static mut attached_pids: *mut libc::c_int = 0 as *const libc::c_int as *mut libc::c_int;
static mut work_threads: *mut pthread_t = 0 as *const pthread_t as *mut pthread_t;
static mut signal_thread: pthread_t = 0;
static mut avail_pids_count: libc::c_int = 0 as libc::c_int;
static mut mutex: pthread_mutex_t = pthread_mutex_t {
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
static mut can_produce: pthread_cond_t = pthread_cond_t {
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
static mut can_consume: pthread_cond_t = pthread_cond_t {
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
static mut done_pipe: [libc::c_int; 2] = [-(1 as libc::c_int), -(1 as libc::c_int)];
pub unsafe extern "C" fn main_pgrep() -> libc::c_int {
    let mut i: libc::c_long = 0;
    if opt_num_workers < 1 as libc::c_int {
        log_error(
            b"Expected max concurrent workers (-T) > 0\n\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    pthread_create(
        &mut signal_thread,
        0 as *const pthread_attr_t,
        Some(
            run_signal_thread
                as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
        ),
        0 as *mut libc::c_void,
    );
    block_all_signals();
    init_work_threads();
    i = 0 as libc::c_int as libc::c_long;
    while i < opt_num_workers as libc::c_long {
        pthread_create(
            &mut *work_threads.offset(i as isize),
            0 as *const pthread_attr_t,
            Some(
                run_work_thread
                    as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
            ),
            i as *mut libc::c_void,
        );
        i += 1;
        i;
    }
    if opt_time_limit_ms > 0 as libc::c_int as libc::c_long {
        alarm(
            (if 1 as libc::c_int as libc::c_long
                > opt_time_limit_ms / 1000 as libc::c_int as libc::c_long
            {
                1 as libc::c_int as libc::c_long
            } else {
                opt_time_limit_ms / 1000 as libc::c_int as libc::c_long
            }) as libc::c_uint,
        );
    }
    pgrep_for_pids();
    i = 0 as libc::c_int as libc::c_long;
    while i < opt_num_workers as libc::c_long {
        pthread_join(*work_threads.offset(i as isize), 0 as *mut *mut libc::c_void);
        i += 1;
        i;
    }
    pthread_join(signal_thread, 0 as *mut *mut libc::c_void);
    deinit_work_threads();
    log_error(b"main_pgrep finished gracefully\n\0" as *const u8 as *const libc::c_char);
    return 0 as libc::c_int;
}
unsafe extern "C" fn wait_for_turn(
    mut producer_or_consumer: libc::c_char,
) -> libc::c_int {
    let mut timeout: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    pthread_mutex_lock(&mut mutex);
    while done == 0 {
        if producer_or_consumer as libc::c_int == 'p' as i32
            && avail_pids_count < opt_num_workers
        {
            break;
        }
        if avail_pids_count > 0 as libc::c_int {
            break;
        }
        clock_gettime(0 as libc::c_int, &mut timeout);
        timeout.tv_sec += 2 as libc::c_int as libc::c_long;
        pthread_cond_timedwait(
            if producer_or_consumer as libc::c_int == 'p' as i32 {
                &mut can_produce
            } else {
                &mut can_consume
            },
            &mut mutex,
            &mut timeout,
        );
    }
    if done != 0 {
        pthread_mutex_unlock(&mut mutex);
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn pgrep_for_pids() {
    let mut pcmd: *mut FILE = 0 as *mut FILE;
    let mut pgrep_cmd: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut line: [libc::c_char; 64] = [0; 64];
    let mut pid: libc::c_int = 0;
    let mut found: libc::c_int = 0;
    let mut timeout: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    if asprintf(
        &mut pgrep_cmd as *mut *mut libc::c_char,
        b"pgrep %s\0" as *const u8 as *const libc::c_char,
        opt_pgrep_args,
    ) < 0 as libc::c_int
    {
        *__errno_location() = 12 as libc::c_int;
        perror(b"asprintf\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    while done == 0 {
        if wait_for_turn('p' as i32 as libc::c_char) != 0 {
            break;
        }
        found = 0 as libc::c_int;
        pcmd = popen(pgrep_cmd, b"r\0" as *const u8 as *const libc::c_char);
        if !pcmd.is_null() {
            while avail_pids_count < opt_num_workers
                && !(fgets(
                    line.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong
                        as libc::c_int,
                    pcmd,
                ))
                    .is_null()
            {
                if strlen(line.as_mut_ptr()) < 1 as libc::c_int as libc::c_ulong
                    || *line.as_mut_ptr() as libc::c_int == '\n' as i32
                {
                    continue;
                }
                pid = atoi(line.as_mut_ptr());
                if is_already_attached(pid) != 0 {
                    continue;
                }
                let fresh0 = avail_pids_count;
                avail_pids_count = avail_pids_count + 1;
                *avail_pids.offset(fresh0 as isize) = pid;
                found += 1 as libc::c_int;
            }
            pclose(pcmd);
        }
        if found > 0 as libc::c_int {
            pthread_cond_broadcast(&mut can_consume);
        } else {
            clock_gettime(0 as libc::c_int, &mut timeout);
            timeout.tv_sec += 2 as libc::c_int as libc::c_long;
            pthread_cond_timedwait(&mut can_produce, &mut mutex, &mut timeout);
        }
        pthread_mutex_unlock(&mut mutex);
    }
    free(pgrep_cmd as *mut libc::c_void);
}
unsafe extern "C" fn run_work_thread(mut arg: *mut libc::c_void) -> *mut libc::c_void {
    let mut worker_num: libc::c_int = 0;
    worker_num = arg as libc::c_long as libc::c_int;
    while done == 0 {
        if wait_for_turn('c' as i32 as libc::c_char) != 0 {
            break;
        }
        avail_pids_count -= 1;
        *attached_pids
            .offset(worker_num as isize) = *avail_pids.offset(avail_pids_count as isize);
        pthread_cond_signal(&mut can_produce);
        pthread_mutex_unlock(&mut mutex);
        main_pid(*attached_pids.offset(worker_num as isize));
        *attached_pids.offset(worker_num as isize) = 0 as libc::c_int;
    }
    return 0 as *mut libc::c_void;
}
unsafe extern "C" fn is_already_attached(mut pid: libc::c_int) -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < opt_num_workers {
        if *attached_pids.offset(i as isize) == pid {
            return 1 as libc::c_int
        } else if i < avail_pids_count && *avail_pids.offset(i as isize) == pid {
            return 1 as libc::c_int
        }
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn init_work_threads() {
    avail_pids = calloc(
        opt_num_workers as libc::c_ulong,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    attached_pids = calloc(
        opt_num_workers as libc::c_ulong,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    work_threads = calloc(
        opt_num_workers as libc::c_ulong,
        ::std::mem::size_of::<pthread_t>() as libc::c_ulong,
    ) as *mut pthread_t;
    if avail_pids.is_null() || attached_pids.is_null() || work_threads.is_null() {
        *__errno_location() = 12 as libc::c_int;
        perror(b"calloc\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    pthread_mutex_init(&mut mutex, 0 as *const pthread_mutexattr_t);
    pthread_cond_init(&mut can_produce, 0 as *const pthread_condattr_t);
    pthread_cond_init(&mut can_consume, 0 as *const pthread_condattr_t);
}
unsafe extern "C" fn deinit_work_threads() {
    free(avail_pids as *mut libc::c_void);
    free(attached_pids as *mut libc::c_void);
    free(work_threads as *mut libc::c_void);
    pthread_mutex_destroy(&mut mutex);
    pthread_cond_destroy(&mut can_produce);
    pthread_cond_destroy(&mut can_consume);
}
unsafe extern "C" fn block_all_signals() -> libc::c_int {
    let mut rv: libc::c_int = 0;
    let mut set: sigset_t = sigset_t { __val: [0; 16] };
    rv = sigfillset(&mut set);
    if rv != 0 as libc::c_int {
        return rv;
    }
    rv = sigprocmask(0 as libc::c_int, &mut set, 0 as *mut sigset_t);
    if rv != 0 as libc::c_int {
        return rv;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn write_done_pipe() {
    let mut rv: libc::c_int = 0;
    let mut ignore: libc::c_int = 0;
    if done_pipe[1 as libc::c_int as usize] >= 0 as libc::c_int {
        ignore = 1 as libc::c_int;
        rv = write(
            done_pipe[1 as libc::c_int as usize],
            &mut ignore as *mut libc::c_int as *const libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
        ) as libc::c_int;
    }
}
unsafe extern "C" fn handle_signal(mut signum: libc::c_int) {
    write_done_pipe();
}
unsafe extern "C" fn run_signal_thread(mut arg: *mut libc::c_void) -> *mut libc::c_void {
    let mut rv: libc::c_int = 0;
    let mut ignore: libc::c_int = 0;
    let mut rfds: fd_set = fd_set { fds_bits: [0; 16] };
    let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut sa: sigaction = sigaction {
        __sigaction_handler: C2RustUnnamed_14 {
            sa_handler: None,
        },
        sa_mask: sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    rv = pipe(done_pipe.as_mut_ptr());
    rv = fcntl(
        done_pipe[1 as libc::c_int as usize],
        4 as libc::c_int,
        0o4000 as libc::c_int,
    );
    memset(
        &mut sa as *mut sigaction as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<sigaction>() as libc::c_ulong,
    );
    sa
        .__sigaction_handler
        .sa_handler = Some(handle_signal as unsafe extern "C" fn(libc::c_int) -> ());
    sigaction(2 as libc::c_int, &mut sa, 0 as *mut sigaction);
    sigaction(15 as libc::c_int, &mut sa, 0 as *mut sigaction);
    sigaction(1 as libc::c_int, &mut sa, 0 as *mut sigaction);
    sigaction(14 as libc::c_int, &mut sa, 0 as *mut sigaction);
    sa
        .__sigaction_handler
        .sa_handler = ::std::mem::transmute::<
        libc::intptr_t,
        __sighandler_t,
    >(1 as libc::c_int as libc::intptr_t);
    sigaction(13 as libc::c_int, &mut sa, 0 as *mut sigaction);
    loop {
        let mut __d0: libc::c_int = 0;
        let mut __d1: libc::c_int = 0;
        let fresh1 = &mut __d0;
        let fresh2;
        let fresh3 = (::std::mem::size_of::<fd_set>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<__fd_mask>() as libc::c_ulong);
        let fresh4 = &mut __d1;
        let fresh5;
        let fresh6 = &mut *(rfds.fds_bits).as_mut_ptr().offset(0 as libc::c_int as isize)
            as *mut __fd_mask;
        asm!(
            "cld; rep; stosq", inlateout("cx") c2rust_asm_casts::AsmCast::cast_in(fresh1,
            fresh3) => fresh2, inlateout("di") c2rust_asm_casts::AsmCast::cast_in(fresh4,
            fresh6) => fresh5, inlateout("ax") 0 as libc::c_int => _,
            options(preserves_flags, att_syntax)
        );
        c2rust_asm_casts::AsmCast::cast_out(fresh1, fresh3, fresh2);
        c2rust_asm_casts::AsmCast::cast_out(fresh4, fresh6, fresh5);
        rfds
            .fds_bits[(done_pipe[0 as libc::c_int as usize]
            / (8 as libc::c_int
                * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
            as usize]
            |= ((1 as libc::c_ulong)
                << done_pipe[0 as libc::c_int as usize]
                    % (8 as libc::c_int
                        * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                            as libc::c_int)) as __fd_mask;
        tv.tv_sec = 1 as libc::c_int as __time_t;
        tv.tv_usec = 0 as libc::c_int as __suseconds_t;
        rv = select(
            done_pipe[0 as libc::c_int as usize] + 1 as libc::c_int,
            &mut rfds,
            0 as *mut fd_set,
            0 as *mut fd_set,
            &mut tv,
        );
        if !(rv < 1 as libc::c_int) {
            break;
        }
    }
    rv = read(
        done_pipe[0 as libc::c_int as usize],
        &mut ignore as *mut libc::c_int as *mut libc::c_void,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as libc::c_int;
    done = 1 as libc::c_int;
    pthread_mutex_lock(&mut mutex);
    pthread_cond_broadcast(&mut can_consume);
    pthread_cond_broadcast(&mut can_produce);
    pthread_mutex_unlock(&mut mutex);
    return 0 as *mut libc::c_void;
}
