use ::libc;
use ::c2rust_asm_casts;
use c2rust_asm_casts::AsmCastTrait;
use std::arch::asm;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn pipe(__pipedes: *mut libc::c_int) -> libc::c_int;
    fn getpid() -> __pid_t;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn select(
        __nfds: libc::c_int,
        __readfds: *mut fd_set,
        __writefds: *mut fd_set,
        __exceptfds: *mut fd_set,
        __timeout: *mut timeval,
    ) -> libc::c_int;
    fn abort() -> !;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn kill(__pid: __pid_t, __sig: libc::c_int) -> libc::c_int;
    fn sigemptyset(__set: *mut sigset_t) -> libc::c_int;
    fn sigfillset(__set: *mut sigset_t) -> libc::c_int;
    fn sigaddset(__set: *mut sigset_t, __signo: libc::c_int) -> libc::c_int;
    fn sigdelset(__set: *mut sigset_t, __signo: libc::c_int) -> libc::c_int;
    fn sigismember(__set: *const sigset_t, __signo: libc::c_int) -> libc::c_int;
    fn sigprocmask(
        __how: libc::c_int,
        __set: *const sigset_t,
        __oset: *mut sigset_t,
    ) -> libc::c_int;
    fn sigsuspend(__set: *const sigset_t) -> libc::c_int;
    fn sigaction(
        __sig: libc::c_int,
        __act: *const sigaction,
        __oact: *mut sigaction,
    ) -> libc::c_int;
    fn sigpending(__set: *mut sigset_t) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn gettimeofday(__tv: *mut timeval, __tz: *mut libc::c_void) -> libc::c_int;
    fn pth_fdmode(_: libc::c_int, _: libc::c_int) -> libc::c_int;
    fn swapcontext(__oucp: *mut ucontext_t, __ucp: *const ucontext_t) -> libc::c_int;
    static mut __pth_time_zero: pth_time_t;
    fn __pth_time_cmp(_: *mut pth_time_t, _: *mut pth_time_t) -> libc::c_int;
    fn __pth_tcb_free(_: pth_t);
    fn __pth_util_sigdelete(_: libc::c_int) -> libc::c_int;
    fn __pth_util_fds_merge(
        _: libc::c_int,
        _: *mut fd_set,
        _: *mut fd_set,
        _: *mut fd_set,
        _: *mut fd_set,
        _: *mut fd_set,
        _: *mut fd_set,
    );
    fn __pth_util_fds_test(
        _: libc::c_int,
        _: *mut fd_set,
        _: *mut fd_set,
        _: *mut fd_set,
        _: *mut fd_set,
        _: *mut fd_set,
        _: *mut fd_set,
    ) -> libc::c_int;
    fn __pth_util_fds_select(
        _: libc::c_int,
        _: *mut fd_set,
        _: *mut fd_set,
        _: *mut fd_set,
        _: *mut fd_set,
        _: *mut fd_set,
        _: *mut fd_set,
    ) -> libc::c_int;
    fn __pth_pqueue_init(_: *mut pth_pqueue_t);
    fn __pth_pqueue_insert(_: *mut pth_pqueue_t, _: libc::c_int, _: pth_t);
    fn __pth_pqueue_delmax(_: *mut pth_pqueue_t) -> pth_t;
    fn __pth_pqueue_delete(_: *mut pth_pqueue_t, _: pth_t);
    fn __pth_pqueue_increase(_: *mut pth_pqueue_t);
    fn __pth_pqueue_tail(_: *mut pth_pqueue_t) -> pth_t;
    fn __pth_pqueue_walk(_: *mut pth_pqueue_t, _: pth_t, _: libc::c_int) -> pth_t;
}
pub type size_t = libc::c_ulong;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __clock_t = libc::c_long;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
pub type sigset_t = __sigset_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type __fd_mask = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fd_set {
    pub __fds_bits: [__fd_mask; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union sigval {
    pub sival_int: libc::c_int,
    pub sival_ptr: *mut libc::c_void,
}
pub type __sigval_t = sigval;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct siginfo_t {
    pub si_signo: libc::c_int,
    pub si_errno: libc::c_int,
    pub si_code: libc::c_int,
    pub __pad0: libc::c_int,
    pub _sifields: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub _pad: [libc::c_int; 28],
    pub _kill: C2RustUnnamed_8,
    pub _timer: C2RustUnnamed_7,
    pub _rt: C2RustUnnamed_6,
    pub _sigchld: C2RustUnnamed_5,
    pub _sigfault: C2RustUnnamed_2,
    pub _sigpoll: C2RustUnnamed_1,
    pub _sigsys: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub _call_addr: *mut libc::c_void,
    pub _syscall: libc::c_int,
    pub _arch: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub si_band: libc::c_long,
    pub si_fd: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub si_addr: *mut libc::c_void,
    pub si_addr_lsb: libc::c_short,
    pub _bounds: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub _addr_bnd: C2RustUnnamed_4,
    pub _pkey: __uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub _lower: *mut libc::c_void,
    pub _upper: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_status: libc::c_int,
    pub si_utime: __clock_t,
    pub si_stime: __clock_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub si_tid: libc::c_int,
    pub si_overrun: libc::c_int,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
}
pub type __sighandler_t = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sigaction {
    pub __sigaction_handler: C2RustUnnamed_9,
    pub sa_mask: __sigset_t,
    pub sa_flags: libc::c_int,
    pub sa_restorer: Option::<unsafe extern "C" fn() -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_9 {
    pub sa_handler: __sighandler_t,
    pub sa_sigaction: Option::<
        unsafe extern "C" fn(libc::c_int, *mut siginfo_t, *mut libc::c_void) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stack_t {
    pub ss_sp: *mut libc::c_void,
    pub ss_flags: libc::c_int,
    pub ss_size: size_t,
}
pub type greg_t = libc::c_longlong;
pub type gregset_t = [greg_t; 23];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _libc_fpxreg {
    pub significand: [libc::c_ushort; 4],
    pub exponent: libc::c_ushort,
    pub __glibc_reserved1: [libc::c_ushort; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _libc_xmmreg {
    pub element: [__uint32_t; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _libc_fpstate {
    pub cwd: __uint16_t,
    pub swd: __uint16_t,
    pub ftw: __uint16_t,
    pub fop: __uint16_t,
    pub rip: __uint64_t,
    pub rdp: __uint64_t,
    pub mxcsr: __uint32_t,
    pub mxcr_mask: __uint32_t,
    pub _st: [_libc_fpxreg; 8],
    pub _xmm: [_libc_xmmreg; 16],
    pub __glibc_reserved1: [__uint32_t; 24],
}
pub type fpregset_t = *mut _libc_fpstate;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mcontext_t {
    pub gregs: gregset_t,
    pub fpregs: fpregset_t,
    pub __reserved1: [libc::c_ulonglong; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ucontext_t {
    pub uc_flags: libc::c_ulong,
    pub uc_link: *mut ucontext_t,
    pub uc_stack: stack_t,
    pub uc_mcontext: mcontext_t,
    pub uc_sigmask: sigset_t,
    pub __fpregs_mem: _libc_fpstate,
    pub __ssp: [libc::c_ulonglong; 4],
}
pub type pth_time_t = timeval;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pth_st {
    pub q_next: pth_t,
    pub q_prev: pth_t,
    pub q_prio: libc::c_int,
    pub prio: libc::c_int,
    pub name: [libc::c_char; 40],
    pub dispatches: libc::c_int,
    pub state: pth_state_t,
    pub spawned: pth_time_t,
    pub lastran: pth_time_t,
    pub running: pth_time_t,
    pub events: pth_event_t,
    pub sigpending: sigset_t,
    pub sigpendcnt: libc::c_int,
    pub mctx: pth_mctx_t,
    pub stack: *mut libc::c_char,
    pub stacksize: libc::c_uint,
    pub stackguard: *mut libc::c_long,
    pub stackloan: libc::c_int,
    pub start_func: Option::<
        unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
    >,
    pub start_arg: *mut libc::c_void,
    pub joinable: libc::c_int,
    pub join_arg: *mut libc::c_void,
    pub data_value: *mut *const libc::c_void,
    pub data_count: libc::c_int,
    pub cancelreq: libc::c_int,
    pub cancelstate: libc::c_uint,
    pub cleanups: *mut pth_cleanup_t,
    pub mutexring: pth_ring_t,
}
pub type pth_ring_t = pth_ring_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pth_ring_st {
    pub r_hook: *mut pth_ringnode_t,
    pub r_nodes: libc::c_uint,
}
pub type pth_ringnode_t = pth_ringnode_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pth_ringnode_st {
    pub rn_next: *mut pth_ringnode_t,
    pub rn_prev: *mut pth_ringnode_t,
}
pub type pth_cleanup_t = pth_cleanup_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pth_cleanup_st {
    pub next: *mut pth_cleanup_t,
    pub func: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub arg: *mut libc::c_void,
}
pub type pth_mctx_t = pth_mctx_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pth_mctx_st {
    pub uc: ucontext_t,
    pub restored: libc::c_int,
    pub sigs: sigset_t,
    pub error: libc::c_int,
}
pub type pth_event_t = *mut pth_event_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pth_event_st {
    pub ev_next: *mut pth_event_st,
    pub ev_prev: *mut pth_event_st,
    pub ev_status: pth_status_t,
    pub ev_type: libc::c_int,
    pub ev_goal: libc::c_int,
    pub ev_args: C2RustUnnamed_10,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_10 {
    pub FD: C2RustUnnamed_19,
    pub SELECT: C2RustUnnamed_18,
    pub SIGS: C2RustUnnamed_17,
    pub TIME: C2RustUnnamed_16,
    pub MSG: C2RustUnnamed_15,
    pub MUTEX: C2RustUnnamed_14,
    pub COND: C2RustUnnamed_13,
    pub TID: C2RustUnnamed_12,
    pub FUNC: C2RustUnnamed_11,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_11 {
    pub func: pth_event_func_t,
    pub arg: *mut libc::c_void,
    pub tv: pth_time_t,
}
pub type pth_event_func_t = Option::<
    unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_12 {
    pub tid: pth_t,
}
pub type pth_t = *mut pth_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_13 {
    pub cond: *mut pth_cond_t,
}
pub type pth_cond_t = pth_cond_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pth_cond_st {
    pub cn_state: libc::c_ulong,
    pub cn_waiters: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_14 {
    pub mutex: *mut pth_mutex_t,
}
pub type pth_mutex_t = pth_mutex_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pth_mutex_st {
    pub mx_node: pth_ringnode_t,
    pub mx_state: libc::c_int,
    pub mx_owner: pth_t,
    pub mx_count: libc::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_15 {
    pub mp: pth_msgport_t,
}
pub type pth_msgport_t = *mut pth_msgport_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pth_msgport_st {
    pub mp_node: pth_ringnode_t,
    pub mp_name: *const libc::c_char,
    pub mp_tid: pth_t,
    pub mp_queue: pth_ring_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_16 {
    pub tv: pth_time_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_17 {
    pub sigs: *mut sigset_t,
    pub sig: *mut libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_18 {
    pub n: *mut libc::c_int,
    pub nfd: libc::c_int,
    pub rfds: *mut fd_set,
    pub wfds: *mut fd_set,
    pub efds: *mut fd_set,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_19 {
    pub fd: libc::c_int,
}
pub type pth_status_t = libc::c_uint;
pub const PTH_STATUS_FAILED: pth_status_t = 2;
pub const PTH_STATUS_OCCURRED: pth_status_t = 1;
pub const PTH_STATUS_PENDING: pth_status_t = 0;
pub type pth_state_t = pth_state_en;
pub type pth_state_en = libc::c_uint;
pub const PTH_STATE_DEAD: pth_state_en = 4;
pub const PTH_STATE_WAITING: pth_state_en = 3;
pub const PTH_STATE_READY: pth_state_en = 2;
pub const PTH_STATE_NEW: pth_state_en = 1;
pub const PTH_STATE_SCHEDULER: pth_state_en = 0;
pub type C2RustUnnamed_20 = libc::c_int;
pub const PTH_FDMODE_NONBLOCK: C2RustUnnamed_20 = 2;
pub const PTH_FDMODE_BLOCK: C2RustUnnamed_20 = 1;
pub const PTH_FDMODE_POLL: C2RustUnnamed_20 = 0;
pub const PTH_FDMODE_ERROR: C2RustUnnamed_20 = -1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pth_pqueue_st {
    pub q_head: pth_t,
    pub q_num: libc::c_int,
}
pub type pth_pqueue_t = pth_pqueue_st;
pub static mut __pth_main: pth_t = 0 as *const pth_st as *mut pth_st;
pub static mut __pth_sched: pth_t = 0 as *const pth_st as *mut pth_st;
pub static mut __pth_current: pth_t = 0 as *const pth_st as *mut pth_st;
pub static mut __pth_NQ: pth_pqueue_t = pth_pqueue_t {
    q_head: 0 as *const pth_st as *mut pth_st,
    q_num: 0,
};
pub static mut __pth_RQ: pth_pqueue_t = pth_pqueue_t {
    q_head: 0 as *const pth_st as *mut pth_st,
    q_num: 0,
};
pub static mut __pth_WQ: pth_pqueue_t = pth_pqueue_t {
    q_head: 0 as *const pth_st as *mut pth_st,
    q_num: 0,
};
pub static mut __pth_SQ: pth_pqueue_t = pth_pqueue_t {
    q_head: 0 as *const pth_st as *mut pth_st,
    q_num: 0,
};
pub static mut __pth_DQ: pth_pqueue_t = pth_pqueue_t {
    q_head: 0 as *const pth_st as *mut pth_st,
    q_num: 0,
};
pub static mut __pth_favournew: libc::c_int = 0;
pub static mut __pth_loadval: libc::c_float = 0.;
static mut pth_sigpipe: [libc::c_int; 2] = [0; 2];
static mut pth_sigpending: sigset_t = sigset_t { __val: [0; 16] };
static mut pth_sigblock: sigset_t = sigset_t { __val: [0; 16] };
static mut pth_sigcatch: sigset_t = sigset_t { __val: [0; 16] };
static mut pth_sigraised: sigset_t = sigset_t { __val: [0; 16] };
static mut pth_loadticknext: pth_time_t = pth_time_t {
    tv_sec: 0,
    tv_usec: 0,
};
static mut pth_loadtickgap: pth_time_t = {
    let mut init = timeval {
        tv_sec: 1 as libc::c_int as __time_t,
        tv_usec: 0 as libc::c_int as __suseconds_t,
    };
    init
};
pub unsafe extern "C" fn __pth_scheduler_init() -> libc::c_int {
    if pipe(pth_sigpipe.as_mut_ptr()) == -(1 as libc::c_int) {
        *__errno_location() = *__errno_location();
        return 0 as libc::c_int;
    }
    if pth_fdmode(
        pth_sigpipe[0 as libc::c_int as usize],
        PTH_FDMODE_NONBLOCK as libc::c_int,
    ) == PTH_FDMODE_ERROR as libc::c_int
    {
        *__errno_location() = *__errno_location();
        return 0 as libc::c_int;
    }
    if pth_fdmode(
        pth_sigpipe[1 as libc::c_int as usize],
        PTH_FDMODE_NONBLOCK as libc::c_int,
    ) == PTH_FDMODE_ERROR as libc::c_int
    {
        *__errno_location() = *__errno_location();
        return 0 as libc::c_int;
    }
    __pth_sched = 0 as pth_t;
    __pth_current = 0 as pth_t;
    __pth_pqueue_init(&mut __pth_NQ);
    __pth_pqueue_init(&mut __pth_RQ);
    __pth_pqueue_init(&mut __pth_WQ);
    __pth_pqueue_init(&mut __pth_SQ);
    __pth_pqueue_init(&mut __pth_DQ);
    __pth_favournew = 1 as libc::c_int;
    __pth_loadval = 1.0f64 as libc::c_float;
    if (0 as *mut pth_time_t).is_null() {
        gettimeofday(&mut pth_loadticknext, 0 as *mut libc::c_void);
    } else {
        pth_loadticknext.tv_sec = (*(0 as *mut pth_time_t)).tv_sec;
        pth_loadticknext.tv_usec = (*(0 as *mut pth_time_t)).tv_usec;
    }
    return (0 as libc::c_int == 0) as libc::c_int;
}
pub unsafe extern "C" fn __pth_scheduler_drop() {
    let mut t: pth_t = 0 as *mut pth_st;
    loop {
        t = __pth_pqueue_delmax(&mut __pth_NQ);
        if t.is_null() {
            break;
        }
        __pth_tcb_free(t);
    }
    __pth_pqueue_init(&mut __pth_NQ);
    loop {
        t = __pth_pqueue_delmax(&mut __pth_RQ);
        if t.is_null() {
            break;
        }
        __pth_tcb_free(t);
    }
    __pth_pqueue_init(&mut __pth_RQ);
    loop {
        t = __pth_pqueue_delmax(&mut __pth_WQ);
        if t.is_null() {
            break;
        }
        __pth_tcb_free(t);
    }
    __pth_pqueue_init(&mut __pth_WQ);
    loop {
        t = __pth_pqueue_delmax(&mut __pth_SQ);
        if t.is_null() {
            break;
        }
        __pth_tcb_free(t);
    }
    __pth_pqueue_init(&mut __pth_SQ);
    loop {
        t = __pth_pqueue_delmax(&mut __pth_DQ);
        if t.is_null() {
            break;
        }
        __pth_tcb_free(t);
    }
    __pth_pqueue_init(&mut __pth_DQ);
}
pub unsafe extern "C" fn __pth_scheduler_kill() {
    __pth_scheduler_drop();
    close(pth_sigpipe[0 as libc::c_int as usize]);
    close(pth_sigpipe[1 as libc::c_int as usize]);
}
pub unsafe extern "C" fn __pth_scheduler(
    mut dummy: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut sigs: sigset_t = sigset_t { __val: [0; 16] };
    let mut running: pth_time_t = pth_time_t {
        tv_sec: 0,
        tv_usec: 0,
    };
    let mut snapshot: pth_time_t = pth_time_t {
        tv_sec: 0,
        tv_usec: 0,
    };
    let mut sa: sigaction = sigaction {
        __sigaction_handler: C2RustUnnamed_9 {
            sa_handler: None,
        },
        sa_mask: sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    let mut ss: sigset_t = sigset_t { __val: [0; 16] };
    let mut sig: libc::c_int = 0;
    let mut t: pth_t = 0 as *mut pth_st;
    (*__pth_sched).state = PTH_STATE_SCHEDULER;
    sigfillset(&mut sigs);
    sigprocmask(2 as libc::c_int, &mut sigs, 0 as *mut sigset_t);
    if (0 as *mut pth_time_t).is_null() {
        gettimeofday(&mut snapshot, 0 as *mut libc::c_void);
    } else {
        snapshot.tv_sec = (*(0 as *mut pth_time_t)).tv_sec;
        snapshot.tv_usec = (*(0 as *mut pth_time_t)).tv_usec;
    }
    loop {
        loop {
            t = __pth_pqueue_tail(&mut __pth_NQ);
            if t.is_null() {
                break;
            }
            __pth_pqueue_delete(&mut __pth_NQ, t);
            (*t).state = PTH_STATE_READY;
            if __pth_favournew != 0 {
                __pth_pqueue_insert(
                    &mut __pth_RQ,
                    if !(__pth_RQ.q_head).is_null() {
                        (*__pth_RQ.q_head).q_prio + 1 as libc::c_int
                    } else {
                        5 as libc::c_int
                    },
                    t,
                );
            } else {
                __pth_pqueue_insert(&mut __pth_RQ, 0 as libc::c_int, t);
            }
        }
        if __pth_time_cmp(&mut snapshot, &mut pth_loadticknext) >= 0 as libc::c_int {
            let mut ttmp: pth_time_t = pth_time_t {
                tv_sec: 0,
                tv_usec: 0,
            };
            let mut numready: libc::c_int = 0;
            numready = if (&mut __pth_RQ as *mut pth_pqueue_t).is_null() {
                -(1 as libc::c_int)
            } else {
                __pth_RQ.q_num
            };
            if (&mut snapshot as *mut pth_time_t).is_null() {
                gettimeofday(&mut ttmp, 0 as *mut libc::c_void);
            } else {
                ttmp.tv_sec = snapshot.tv_sec;
                ttmp.tv_usec = snapshot.tv_usec;
            }
            loop {
                __pth_loadval = (numready as libc::c_double * 0.25f64
                    + __pth_loadval as libc::c_double * 0.75f64) as libc::c_float;
                ttmp.tv_sec -= pth_loadtickgap.tv_sec;
                ttmp.tv_usec -= pth_loadtickgap.tv_usec;
                if ttmp.tv_usec < 0 as libc::c_int as libc::c_long {
                    ttmp.tv_sec -= 1 as libc::c_int as libc::c_long;
                    ttmp.tv_usec += 1000000 as libc::c_int as libc::c_long;
                }
                if !(__pth_time_cmp(&mut ttmp, &mut pth_loadticknext)
                    >= 0 as libc::c_int)
                {
                    break;
                }
            }
            if (&mut snapshot as *mut pth_time_t).is_null() {
                gettimeofday(&mut pth_loadticknext, 0 as *mut libc::c_void);
            } else {
                pth_loadticknext.tv_sec = snapshot.tv_sec;
                pth_loadticknext.tv_usec = snapshot.tv_usec;
            }
            pth_loadticknext.tv_sec += pth_loadtickgap.tv_sec;
            pth_loadticknext.tv_usec += pth_loadtickgap.tv_usec;
            if pth_loadticknext.tv_usec > 1000000 as libc::c_int as libc::c_long {
                pth_loadticknext.tv_sec += 1 as libc::c_int as libc::c_long;
                pth_loadticknext.tv_usec -= 1000000 as libc::c_int as libc::c_long;
            }
        }
        __pth_current = __pth_pqueue_delmax(&mut __pth_RQ);
        if __pth_current.is_null() {
            fprintf(
                stderr,
                b"**Pth** SCHEDULER INTERNAL ERROR: no more thread(s) available to schedule!?!?\n\0"
                    as *const u8 as *const libc::c_char,
            );
            abort();
        }
        if (*__pth_current).sigpendcnt > 0 as libc::c_int {
            sigpending(&mut pth_sigpending);
            sig = 1 as libc::c_int;
            while sig < 65 as libc::c_int {
                if sigismember(&mut (*__pth_current).sigpending, sig) != 0 {
                    if sigismember(&mut pth_sigpending, sig) == 0 {
                        kill(getpid(), sig);
                    }
                }
                sig += 1;
                sig;
            }
        }
        if (0 as *mut pth_time_t).is_null() {
            gettimeofday(&mut (*__pth_current).lastran, 0 as *mut libc::c_void);
        } else {
            (*__pth_current).lastran.tv_sec = (*(0 as *mut pth_time_t)).tv_sec;
            (*__pth_current).lastran.tv_usec = (*(0 as *mut pth_time_t)).tv_usec;
        }
        if (&mut (*__pth_current).lastran as *mut pth_time_t).is_null() {
            gettimeofday(&mut running, 0 as *mut libc::c_void);
        } else {
            running.tv_sec = (*__pth_current).lastran.tv_sec;
            running.tv_usec = (*__pth_current).lastran.tv_usec;
        }
        running.tv_sec -= snapshot.tv_sec;
        running.tv_usec -= snapshot.tv_usec;
        if running.tv_usec < 0 as libc::c_int as libc::c_long {
            running.tv_sec -= 1 as libc::c_int as libc::c_long;
            running.tv_usec += 1000000 as libc::c_int as libc::c_long;
        }
        (*__pth_sched).running.tv_sec += running.tv_sec;
        (*__pth_sched).running.tv_usec += running.tv_usec;
        if (*__pth_sched).running.tv_usec > 1000000 as libc::c_int as libc::c_long {
            (*__pth_sched).running.tv_sec += 1 as libc::c_int as libc::c_long;
            (*__pth_sched).running.tv_usec -= 1000000 as libc::c_int as libc::c_long;
        }
        (*__pth_current).dispatches += 1;
        (*__pth_current).dispatches;
        swapcontext(&mut (*__pth_sched).mctx.uc, &mut (*__pth_current).mctx.uc);
        if (0 as *mut pth_time_t).is_null() {
            gettimeofday(&mut snapshot, 0 as *mut libc::c_void);
        } else {
            snapshot.tv_sec = (*(0 as *mut pth_time_t)).tv_sec;
            snapshot.tv_usec = (*(0 as *mut pth_time_t)).tv_usec;
        }
        if (&mut snapshot as *mut pth_time_t).is_null() {
            gettimeofday(&mut running, 0 as *mut libc::c_void);
        } else {
            running.tv_sec = snapshot.tv_sec;
            running.tv_usec = snapshot.tv_usec;
        }
        running.tv_sec -= (*__pth_current).lastran.tv_sec;
        running.tv_usec -= (*__pth_current).lastran.tv_usec;
        if running.tv_usec < 0 as libc::c_int as libc::c_long {
            running.tv_sec -= 1 as libc::c_int as libc::c_long;
            running.tv_usec += 1000000 as libc::c_int as libc::c_long;
        }
        (*__pth_current).running.tv_sec += running.tv_sec;
        (*__pth_current).running.tv_usec += running.tv_usec;
        if (*__pth_current).running.tv_usec > 1000000 as libc::c_int as libc::c_long {
            (*__pth_current).running.tv_sec += 1 as libc::c_int as libc::c_long;
            (*__pth_current).running.tv_usec -= 1000000 as libc::c_int as libc::c_long;
        }
        if (*__pth_current).sigpendcnt > 0 as libc::c_int {
            let mut sigstillpending: sigset_t = sigset_t { __val: [0; 16] };
            sigpending(&mut sigstillpending);
            sig = 1 as libc::c_int;
            while sig < 65 as libc::c_int {
                if sigismember(&mut (*__pth_current).sigpending, sig) != 0 {
                    if sigismember(&mut sigstillpending, sig) == 0 {
                        sigdelset(&mut (*__pth_current).sigpending, sig);
                        (*__pth_current).sigpendcnt -= 1;
                        (*__pth_current).sigpendcnt;
                    } else if sigismember(&mut pth_sigpending, sig) == 0 {
                        __pth_util_sigdelete(sig);
                    }
                }
                sig += 1;
                sig;
            }
        }
        if !((*__pth_current).stackguard).is_null() {
            if *(*__pth_current).stackguard != 0xdead as libc::c_int as libc::c_long {
                if sigaction(11 as libc::c_int, 0 as *const sigaction, &mut sa)
                    == 0 as libc::c_int
                {
                    if (sa.__sigaction_handler.sa_handler).is_none() {
                        fprintf(
                            stderr,
                            b"**Pth** STACK OVERFLOW: thread pid_t=0x%lx, name=\"%s\"\n\0"
                                as *const u8 as *const libc::c_char,
                            __pth_current as libc::c_ulong,
                            ((*__pth_current).name).as_mut_ptr(),
                        );
                        kill(getpid(), 11 as libc::c_int);
                        sigfillset(&mut ss);
                        sigdelset(&mut ss, 11 as libc::c_int);
                        sigsuspend(&mut ss);
                        abort();
                    }
                }
                (*__pth_current).join_arg = 0xdead as libc::c_int as *mut libc::c_void;
                (*__pth_current).state = PTH_STATE_DEAD;
                kill(getpid(), 11 as libc::c_int);
            }
        }
        if (*__pth_current).state as libc::c_uint
            == PTH_STATE_DEAD as libc::c_int as libc::c_uint
        {
            if (*__pth_current).joinable == 0 {
                __pth_tcb_free(__pth_current);
            } else {
                __pth_pqueue_insert(&mut __pth_DQ, 0 as libc::c_int, __pth_current);
            }
            __pth_current = 0 as pth_t;
        }
        if !__pth_current.is_null()
            && (*__pth_current).state as libc::c_uint
                == PTH_STATE_WAITING as libc::c_int as libc::c_uint
        {
            __pth_pqueue_insert(&mut __pth_WQ, (*__pth_current).prio, __pth_current);
            __pth_current = 0 as pth_t;
        }
        __pth_pqueue_increase(&mut __pth_RQ);
        if !__pth_current.is_null() {
            __pth_pqueue_insert(&mut __pth_RQ, (*__pth_current).prio, __pth_current);
        }
        if (if (&mut __pth_RQ as *mut pth_pqueue_t).is_null() {
            -(1 as libc::c_int)
        } else {
            __pth_RQ.q_num
        }) == 0 as libc::c_int
            && (if (&mut __pth_NQ as *mut pth_pqueue_t).is_null() {
                -(1 as libc::c_int)
            } else {
                __pth_NQ.q_num
            }) == 0 as libc::c_int
        {
            __pth_sched_eventmanager(&mut snapshot, 0 as libc::c_int);
        } else {
            __pth_sched_eventmanager(
                &mut snapshot,
                (0 as libc::c_int == 0) as libc::c_int,
            );
        }
    };
}
pub unsafe extern "C" fn __pth_sched_eventmanager(
    mut now: *mut pth_time_t,
    mut dopoll: libc::c_int,
) {
    let mut nexttimer_thread: pth_t = 0 as *mut pth_st;
    let mut nexttimer_ev: pth_event_t = 0 as *mut pth_event_st;
    let mut nexttimer_value: pth_time_t = pth_time_t {
        tv_sec: 0,
        tv_usec: 0,
    };
    let mut evh: pth_event_t = 0 as *mut pth_event_st;
    let mut ev: pth_event_t = 0 as *mut pth_event_st;
    let mut t: pth_t = 0 as *mut pth_st;
    let mut tlast: pth_t = 0 as *mut pth_st;
    let mut this_occurred: libc::c_int = 0;
    let mut any_occurred: libc::c_int = 0;
    let mut rfds: fd_set = fd_set { __fds_bits: [0; 16] };
    let mut wfds: fd_set = fd_set { __fds_bits: [0; 16] };
    let mut efds: fd_set = fd_set { __fds_bits: [0; 16] };
    let mut delay: timeval = pth_time_t {
        tv_sec: 0,
        tv_usec: 0,
    };
    let mut pdelay: *mut timeval = 0 as *mut timeval;
    let mut oss: sigset_t = sigset_t { __val: [0; 16] };
    let mut sa: sigaction = sigaction {
        __sigaction_handler: C2RustUnnamed_9 {
            sa_handler: None,
        },
        sa_mask: sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    let mut osa: [sigaction; 66] = [sigaction {
        __sigaction_handler: C2RustUnnamed_9 {
            sa_handler: None,
        },
        sa_mask: sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    }; 66];
    let mut minibuf: [libc::c_char; 128] = [0; 128];
    let mut loop_repeat: libc::c_int = 0;
    let mut fdmax: libc::c_int = 0;
    let mut rc: libc::c_int = 0;
    let mut sig: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    loop {
        loop_repeat = 0 as libc::c_int;
        let mut __d0: libc::c_int = 0;
        let mut __d1: libc::c_int = 0;
        let fresh0 = &mut __d0;
        let fresh1;
        let fresh2 = (::std::mem::size_of::<fd_set>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<__fd_mask>() as libc::c_ulong);
        let fresh3 = &mut __d1;
        let fresh4;
        let fresh5 = &mut *(rfds.__fds_bits)
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize) as *mut __fd_mask;
        asm!(
            "cld; rep; stosq", inlateout("cx") c2rust_asm_casts::AsmCast::cast_in(fresh0,
            fresh2) => fresh1, inlateout("di") c2rust_asm_casts::AsmCast::cast_in(fresh3,
            fresh5) => fresh4, inlateout("ax") 0 as libc::c_int => _,
            options(preserves_flags, att_syntax)
        );
        c2rust_asm_casts::AsmCast::cast_out(fresh0, fresh2, fresh1);
        c2rust_asm_casts::AsmCast::cast_out(fresh3, fresh5, fresh4);
        let mut __d0_0: libc::c_int = 0;
        let mut __d1_0: libc::c_int = 0;
        let fresh6 = &mut __d0_0;
        let fresh7;
        let fresh8 = (::std::mem::size_of::<fd_set>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<__fd_mask>() as libc::c_ulong);
        let fresh9 = &mut __d1_0;
        let fresh10;
        let fresh11 = &mut *(wfds.__fds_bits)
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
        let mut __d0_1: libc::c_int = 0;
        let mut __d1_1: libc::c_int = 0;
        let fresh12 = &mut __d0_1;
        let fresh13;
        let fresh14 = (::std::mem::size_of::<fd_set>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<__fd_mask>() as libc::c_ulong);
        let fresh15 = &mut __d1_1;
        let fresh16;
        let fresh17 = &mut *(efds.__fds_bits)
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize) as *mut __fd_mask;
        asm!(
            "cld; rep; stosq", inlateout("cx")
            c2rust_asm_casts::AsmCast::cast_in(fresh12, fresh14) => fresh13,
            inlateout("di") c2rust_asm_casts::AsmCast::cast_in(fresh15, fresh17) =>
            fresh16, inlateout("ax") 0 as libc::c_int => _, options(preserves_flags,
            att_syntax)
        );
        c2rust_asm_casts::AsmCast::cast_out(fresh12, fresh14, fresh13);
        c2rust_asm_casts::AsmCast::cast_out(fresh15, fresh17, fresh16);
        fdmax = -(1 as libc::c_int);
        sigpending(&mut pth_sigpending);
        sigfillset(&mut pth_sigblock);
        sigemptyset(&mut pth_sigcatch);
        sigemptyset(&mut pth_sigraised);
        if (&mut __pth_time_zero as *mut pth_time_t).is_null() {
            gettimeofday(&mut nexttimer_value, 0 as *mut libc::c_void);
        } else {
            nexttimer_value.tv_sec = __pth_time_zero.tv_sec;
            nexttimer_value.tv_usec = __pth_time_zero.tv_usec;
        }
        nexttimer_thread = 0 as pth_t;
        nexttimer_ev = 0 as pth_event_t;
        any_occurred = 0 as libc::c_int;
        t = if (&mut __pth_WQ as *mut pth_pqueue_t).is_null() {
            0 as pth_t
        } else {
            __pth_WQ.q_head
        };
        while !t.is_null() {
            sig = 1 as libc::c_int;
            while sig < 65 as libc::c_int {
                if sigismember(&mut (*t).mctx.sigs, sig) == 0 {
                    sigdelset(&mut pth_sigblock, sig);
                }
                sig += 1;
                sig;
            }
            if (*t).cancelreq == (0 as libc::c_int == 0) as libc::c_int {
                any_occurred = (0 as libc::c_int == 0) as libc::c_int;
            }
            if !((*t).events).is_null() {
                evh = (*t).events;
                ev = evh;
                loop {
                    if (*ev).ev_status as libc::c_uint
                        == PTH_STATUS_PENDING as libc::c_int as libc::c_uint
                    {
                        this_occurred = 0 as libc::c_int;
                        if (*ev).ev_type == (1 as libc::c_int) << 1 as libc::c_int {
                            if (*ev).ev_goal & (1 as libc::c_int) << 12 as libc::c_int
                                != 0
                            {
                                rfds
                                    .__fds_bits[((*ev).ev_args.FD.fd
                                    / (8 as libc::c_int
                                        * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                            as libc::c_int)) as usize]
                                    |= ((1 as libc::c_ulong)
                                        << (*ev).ev_args.FD.fd
                                            % (8 as libc::c_int
                                                * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                                    as libc::c_int)) as __fd_mask;
                            }
                            if (*ev).ev_goal & (1 as libc::c_int) << 13 as libc::c_int
                                != 0
                            {
                                wfds
                                    .__fds_bits[((*ev).ev_args.FD.fd
                                    / (8 as libc::c_int
                                        * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                            as libc::c_int)) as usize]
                                    |= ((1 as libc::c_ulong)
                                        << (*ev).ev_args.FD.fd
                                            % (8 as libc::c_int
                                                * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                                    as libc::c_int)) as __fd_mask;
                            }
                            if (*ev).ev_goal & (1 as libc::c_int) << 14 as libc::c_int
                                != 0
                            {
                                efds
                                    .__fds_bits[((*ev).ev_args.FD.fd
                                    / (8 as libc::c_int
                                        * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                            as libc::c_int)) as usize]
                                    |= ((1 as libc::c_ulong)
                                        << (*ev).ev_args.FD.fd
                                            % (8 as libc::c_int
                                                * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                                    as libc::c_int)) as __fd_mask;
                            }
                            if fdmax < (*ev).ev_args.FD.fd {
                                fdmax = (*ev).ev_args.FD.fd;
                            }
                        } else if (*ev).ev_type == (1 as libc::c_int) << 2 as libc::c_int
                        {
                            __pth_util_fds_merge(
                                (*ev).ev_args.SELECT.nfd,
                                (*ev).ev_args.SELECT.rfds,
                                &mut rfds,
                                (*ev).ev_args.SELECT.wfds,
                                &mut wfds,
                                (*ev).ev_args.SELECT.efds,
                                &mut efds,
                            );
                            if fdmax < (*ev).ev_args.SELECT.nfd - 1 as libc::c_int {
                                fdmax = (*ev).ev_args.SELECT.nfd - 1 as libc::c_int;
                            }
                        } else if (*ev).ev_type == (1 as libc::c_int) << 3 as libc::c_int
                        {
                            sig = 1 as libc::c_int;
                            while sig < 65 as libc::c_int {
                                if sigismember((*ev).ev_args.SIGS.sigs, sig) != 0 {
                                    if sigismember(&mut (*t).sigpending, sig) != 0 {
                                        *(*ev).ev_args.SIGS.sig = sig;
                                        sigdelset(&mut (*t).sigpending, sig);
                                        (*t).sigpendcnt -= 1;
                                        (*t).sigpendcnt;
                                        this_occurred = (0 as libc::c_int == 0) as libc::c_int;
                                    }
                                    if sigismember(&mut pth_sigpending, sig) != 0 {
                                        if !((*ev).ev_args.SIGS.sig).is_null() {
                                            *(*ev).ev_args.SIGS.sig = sig;
                                        }
                                        __pth_util_sigdelete(sig);
                                        sigdelset(&mut pth_sigpending, sig);
                                        this_occurred = (0 as libc::c_int == 0) as libc::c_int;
                                    } else {
                                        sigdelset(&mut pth_sigblock, sig);
                                        sigaddset(&mut pth_sigcatch, sig);
                                    }
                                }
                                sig += 1;
                                sig;
                            }
                        } else if (*ev).ev_type == (1 as libc::c_int) << 4 as libc::c_int
                        {
                            if __pth_time_cmp(&mut (*ev).ev_args.TIME.tv, now)
                                < 0 as libc::c_int
                            {
                                this_occurred = (0 as libc::c_int == 0) as libc::c_int;
                            } else if nexttimer_thread.is_null()
                                && nexttimer_ev.is_null()
                                || __pth_time_cmp(
                                    &mut (*ev).ev_args.TIME.tv,
                                    &mut nexttimer_value,
                                ) < 0 as libc::c_int
                            {
                                nexttimer_thread = t;
                                nexttimer_ev = ev;
                                if (&mut (*ev).ev_args.TIME.tv as *mut pth_time_t).is_null()
                                {
                                    gettimeofday(&mut nexttimer_value, 0 as *mut libc::c_void);
                                } else {
                                    nexttimer_value.tv_sec = (*ev).ev_args.TIME.tv.tv_sec;
                                    nexttimer_value.tv_usec = (*ev).ev_args.TIME.tv.tv_usec;
                                }
                            }
                        } else if (*ev).ev_type == (1 as libc::c_int) << 5 as libc::c_int
                        {
                            if (if (&mut (*(*ev).ev_args.MSG.mp).mp_queue
                                as *mut pth_ring_t)
                                .is_null()
                            {
                                -(1 as libc::c_int) as libc::c_uint
                            } else {
                                (*(*ev).ev_args.MSG.mp).mp_queue.r_nodes
                            }) > 0 as libc::c_int as libc::c_uint
                            {
                                this_occurred = (0 as libc::c_int == 0) as libc::c_int;
                            }
                        } else if (*ev).ev_type == (1 as libc::c_int) << 6 as libc::c_int
                        {
                            if (*(*ev).ev_args.MUTEX.mutex).mx_state
                                & (1 as libc::c_int) << 1 as libc::c_int == 0
                            {
                                this_occurred = (0 as libc::c_int == 0) as libc::c_int;
                            }
                        } else if (*ev).ev_type == (1 as libc::c_int) << 7 as libc::c_int
                        {
                            if (*(*ev).ev_args.COND.cond).cn_state
                                & ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_ulong
                                != 0
                            {
                                if (*(*ev).ev_args.COND.cond).cn_state
                                    & ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_ulong
                                    != 0
                                {
                                    this_occurred = (0 as libc::c_int == 0) as libc::c_int;
                                } else if (*(*ev).ev_args.COND.cond).cn_state
                                    & ((1 as libc::c_int) << 3 as libc::c_int) as libc::c_ulong
                                    == 0
                                {
                                    (*(*ev).ev_args.COND.cond).cn_state
                                        |= ((1 as libc::c_int) << 3 as libc::c_int)
                                            as libc::c_ulong;
                                    this_occurred = (0 as libc::c_int == 0) as libc::c_int;
                                }
                            }
                        } else if (*ev).ev_type == (1 as libc::c_int) << 8 as libc::c_int
                        {
                            if ((*ev).ev_args.TID.tid).is_null()
                                && (if (&mut __pth_DQ as *mut pth_pqueue_t).is_null() {
                                    -(1 as libc::c_int)
                                } else {
                                    __pth_DQ.q_num
                                }) > 0 as libc::c_int
                                || !((*ev).ev_args.TID.tid).is_null()
                                    && (*(*ev).ev_args.TID.tid).state as libc::c_uint
                                        == (*ev).ev_goal as libc::c_uint
                            {
                                this_occurred = (0 as libc::c_int == 0) as libc::c_int;
                            }
                        } else if (*ev).ev_type == (1 as libc::c_int) << 9 as libc::c_int
                        {
                            if ((*ev).ev_args.FUNC.func).unwrap()((*ev).ev_args.FUNC.arg)
                                != 0
                            {
                                this_occurred = (0 as libc::c_int == 0) as libc::c_int;
                            } else {
                                let mut tv: pth_time_t = pth_time_t {
                                    tv_sec: 0,
                                    tv_usec: 0,
                                };
                                if now.is_null() {
                                    gettimeofday(&mut tv, 0 as *mut libc::c_void);
                                } else {
                                    tv.tv_sec = (*now).tv_sec;
                                    tv.tv_usec = (*now).tv_usec;
                                }
                                tv.tv_sec += (*ev).ev_args.FUNC.tv.tv_sec;
                                tv.tv_usec += (*ev).ev_args.FUNC.tv.tv_usec;
                                if tv.tv_usec > 1000000 as libc::c_int as libc::c_long {
                                    tv.tv_sec += 1 as libc::c_int as libc::c_long;
                                    tv.tv_usec -= 1000000 as libc::c_int as libc::c_long;
                                }
                                if nexttimer_thread.is_null() && nexttimer_ev.is_null()
                                    || __pth_time_cmp(&mut tv, &mut nexttimer_value)
                                        < 0 as libc::c_int
                                {
                                    nexttimer_thread = t;
                                    nexttimer_ev = ev;
                                    if (&mut tv as *mut pth_time_t).is_null() {
                                        gettimeofday(&mut nexttimer_value, 0 as *mut libc::c_void);
                                    } else {
                                        nexttimer_value.tv_sec = tv.tv_sec;
                                        nexttimer_value.tv_usec = tv.tv_usec;
                                    }
                                }
                            }
                        }
                        if this_occurred != 0 {
                            (*ev).ev_status = PTH_STATUS_OCCURRED;
                            any_occurred = (0 as libc::c_int == 0) as libc::c_int;
                        }
                    }
                    ev = (*ev).ev_next;
                    if !(ev != evh) {
                        break;
                    }
                }
            }
            t = __pth_pqueue_walk(
                &mut __pth_WQ,
                t,
                (1 as libc::c_int) << 1 as libc::c_int,
            );
        }
        if any_occurred != 0 {
            dopoll = (0 as libc::c_int == 0) as libc::c_int;
        }
        if dopoll != 0 {
            if (&mut __pth_time_zero as *mut pth_time_t).is_null() {
                gettimeofday(&mut delay, 0 as *mut libc::c_void);
            } else {
                delay.tv_sec = __pth_time_zero.tv_sec;
                delay.tv_usec = __pth_time_zero.tv_usec;
            }
            pdelay = &mut delay;
        } else if !nexttimer_ev.is_null() {
            if (&mut nexttimer_value as *mut pth_time_t).is_null() {
                gettimeofday(&mut delay, 0 as *mut libc::c_void);
            } else {
                delay.tv_sec = nexttimer_value.tv_sec;
                delay.tv_usec = nexttimer_value.tv_usec;
            }
            delay.tv_sec -= (*now).tv_sec;
            delay.tv_usec -= (*now).tv_usec;
            if delay.tv_usec < 0 as libc::c_int as libc::c_long {
                delay.tv_sec -= 1 as libc::c_int as libc::c_long;
                delay.tv_usec += 1000000 as libc::c_int as libc::c_long;
            }
            pdelay = &mut delay;
        } else {
            pdelay = 0 as *mut timeval;
        }
        while read(
            pth_sigpipe[0 as libc::c_int as usize],
            minibuf.as_mut_ptr() as *mut libc::c_void,
            ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
        ) > 0 as libc::c_int as libc::c_long
        {}
        rfds
            .__fds_bits[(pth_sigpipe[0 as libc::c_int as usize]
            / (8 as libc::c_int
                * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
            as usize]
            |= ((1 as libc::c_ulong)
                << pth_sigpipe[0 as libc::c_int as usize]
                    % (8 as libc::c_int
                        * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                            as libc::c_int)) as __fd_mask;
        if fdmax < pth_sigpipe[0 as libc::c_int as usize] {
            fdmax = pth_sigpipe[0 as libc::c_int as usize];
        }
        sig = 1 as libc::c_int;
        while sig < 65 as libc::c_int {
            if sigismember(&mut pth_sigcatch, sig) != 0 {
                sa
                    .__sigaction_handler
                    .sa_handler = Some(
                    __pth_sched_eventmanager_sighandler
                        as unsafe extern "C" fn(libc::c_int) -> (),
                );
                sigfillset(&mut sa.sa_mask);
                sa.sa_flags = 0 as libc::c_int;
                sigaction(sig, &mut sa, &mut *osa.as_mut_ptr().offset(sig as isize));
            }
            sig += 1;
            sig;
        }
        sigprocmask(2 as libc::c_int, &mut pth_sigblock, &mut oss);
        rc = -(1 as libc::c_int);
        if !(dopoll != 0 && fdmax == -(1 as libc::c_int)) {
            loop {
                rc = select(
                    fdmax + 1 as libc::c_int,
                    &mut rfds,
                    &mut wfds,
                    &mut efds,
                    pdelay,
                );
                if !(rc < 0 as libc::c_int && *__errno_location() == 4 as libc::c_int) {
                    break;
                }
            }
        }
        sigprocmask(2 as libc::c_int, &mut oss, 0 as *mut sigset_t);
        sig = 1 as libc::c_int;
        while sig < 65 as libc::c_int {
            if sigismember(&mut pth_sigcatch, sig) != 0 {
                sigaction(
                    sig,
                    &mut *osa.as_mut_ptr().offset(sig as isize),
                    0 as *mut sigaction,
                );
            }
            sig += 1;
            sig;
        }
        if dopoll == 0 && rc == 0 as libc::c_int && !nexttimer_ev.is_null() {
            if (*nexttimer_ev).ev_type == (1 as libc::c_int) << 9 as libc::c_int {
                loop_repeat = (0 as libc::c_int == 0) as libc::c_int;
            } else {
                (*nexttimer_ev).ev_status = PTH_STATUS_OCCURRED;
            }
        }
        if dopoll == 0 && rc > 0 as libc::c_int
            && rfds
                .__fds_bits[(pth_sigpipe[0 as libc::c_int as usize]
                / (8 as libc::c_int
                    * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                        as libc::c_int)) as usize]
                & ((1 as libc::c_ulong)
                    << pth_sigpipe[0 as libc::c_int as usize]
                        % (8 as libc::c_int
                            * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                as libc::c_int)) as __fd_mask
                != 0 as libc::c_int as libc::c_long
        {
            rfds
                .__fds_bits[(pth_sigpipe[0 as libc::c_int as usize]
                / (8 as libc::c_int
                    * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                        as libc::c_int)) as usize]
                &= !(((1 as libc::c_ulong)
                    << pth_sigpipe[0 as libc::c_int as usize]
                        % (8 as libc::c_int
                            * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                as libc::c_int)) as __fd_mask);
            rc -= 1;
            rc;
        }
        if rc <= 0 as libc::c_int {
            let mut __d0_2: libc::c_int = 0;
            let mut __d1_2: libc::c_int = 0;
            let fresh18 = &mut __d0_2;
            let fresh19;
            let fresh20 = (::std::mem::size_of::<fd_set>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<__fd_mask>() as libc::c_ulong);
            let fresh21 = &mut __d1_2;
            let fresh22;
            let fresh23 = &mut *(rfds.__fds_bits)
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize) as *mut __fd_mask;
            asm!(
                "cld; rep; stosq", inlateout("cx")
                c2rust_asm_casts::AsmCast::cast_in(fresh18, fresh20) => fresh19,
                inlateout("di") c2rust_asm_casts::AsmCast::cast_in(fresh21, fresh23) =>
                fresh22, inlateout("ax") 0 as libc::c_int => _, options(preserves_flags,
                att_syntax)
            );
            c2rust_asm_casts::AsmCast::cast_out(fresh18, fresh20, fresh19);
            c2rust_asm_casts::AsmCast::cast_out(fresh21, fresh23, fresh22);
            let mut __d0_3: libc::c_int = 0;
            let mut __d1_3: libc::c_int = 0;
            let fresh24 = &mut __d0_3;
            let fresh25;
            let fresh26 = (::std::mem::size_of::<fd_set>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<__fd_mask>() as libc::c_ulong);
            let fresh27 = &mut __d1_3;
            let fresh28;
            let fresh29 = &mut *(wfds.__fds_bits)
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize) as *mut __fd_mask;
            asm!(
                "cld; rep; stosq", inlateout("cx")
                c2rust_asm_casts::AsmCast::cast_in(fresh24, fresh26) => fresh25,
                inlateout("di") c2rust_asm_casts::AsmCast::cast_in(fresh27, fresh29) =>
                fresh28, inlateout("ax") 0 as libc::c_int => _, options(preserves_flags,
                att_syntax)
            );
            c2rust_asm_casts::AsmCast::cast_out(fresh24, fresh26, fresh25);
            c2rust_asm_casts::AsmCast::cast_out(fresh27, fresh29, fresh28);
            let mut __d0_4: libc::c_int = 0;
            let mut __d1_4: libc::c_int = 0;
            let fresh30 = &mut __d0_4;
            let fresh31;
            let fresh32 = (::std::mem::size_of::<fd_set>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<__fd_mask>() as libc::c_ulong);
            let fresh33 = &mut __d1_4;
            let fresh34;
            let fresh35 = &mut *(efds.__fds_bits)
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize) as *mut __fd_mask;
            asm!(
                "cld; rep; stosq", inlateout("cx")
                c2rust_asm_casts::AsmCast::cast_in(fresh30, fresh32) => fresh31,
                inlateout("di") c2rust_asm_casts::AsmCast::cast_in(fresh33, fresh35) =>
                fresh34, inlateout("ax") 0 as libc::c_int => _, options(preserves_flags,
                att_syntax)
            );
            c2rust_asm_casts::AsmCast::cast_out(fresh30, fresh32, fresh31);
            c2rust_asm_casts::AsmCast::cast_out(fresh33, fresh35, fresh34);
        }
        t = if (&mut __pth_WQ as *mut pth_pqueue_t).is_null() {
            0 as pth_t
        } else {
            __pth_WQ.q_head
        };
        while !t.is_null() {
            any_occurred = 0 as libc::c_int;
            if !((*t).events).is_null() {
                evh = (*t).events;
                ev = evh;
                loop {
                    if (*ev).ev_status as libc::c_uint
                        == PTH_STATUS_PENDING as libc::c_int as libc::c_uint
                    {
                        if (*ev).ev_type == (1 as libc::c_int) << 1 as libc::c_int {
                            if (*ev).ev_goal & (1 as libc::c_int) << 12 as libc::c_int
                                != 0
                                && rfds
                                    .__fds_bits[((*ev).ev_args.FD.fd
                                    / (8 as libc::c_int
                                        * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                            as libc::c_int)) as usize]
                                    & ((1 as libc::c_ulong)
                                        << (*ev).ev_args.FD.fd
                                            % (8 as libc::c_int
                                                * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                                    as libc::c_int)) as __fd_mask
                                    != 0 as libc::c_int as libc::c_long
                                || (*ev).ev_goal & (1 as libc::c_int) << 13 as libc::c_int
                                    != 0
                                    && wfds
                                        .__fds_bits[((*ev).ev_args.FD.fd
                                        / (8 as libc::c_int
                                            * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                                as libc::c_int)) as usize]
                                        & ((1 as libc::c_ulong)
                                            << (*ev).ev_args.FD.fd
                                                % (8 as libc::c_int
                                                    * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                                        as libc::c_int)) as __fd_mask
                                        != 0 as libc::c_int as libc::c_long
                                || (*ev).ev_goal & (1 as libc::c_int) << 14 as libc::c_int
                                    != 0
                                    && efds
                                        .__fds_bits[((*ev).ev_args.FD.fd
                                        / (8 as libc::c_int
                                            * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                                as libc::c_int)) as usize]
                                        & ((1 as libc::c_ulong)
                                            << (*ev).ev_args.FD.fd
                                                % (8 as libc::c_int
                                                    * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                                        as libc::c_int)) as __fd_mask
                                        != 0 as libc::c_int as libc::c_long
                            {
                                (*ev).ev_status = PTH_STATUS_OCCURRED;
                            } else if rc < 0 as libc::c_int {
                                let mut rc2: libc::c_int = 0;
                                if (*ev).ev_goal & (1 as libc::c_int) << 12 as libc::c_int
                                    != 0
                                {
                                    rfds
                                        .__fds_bits[((*ev).ev_args.FD.fd
                                        / (8 as libc::c_int
                                            * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                                as libc::c_int)) as usize]
                                        |= ((1 as libc::c_ulong)
                                            << (*ev).ev_args.FD.fd
                                                % (8 as libc::c_int
                                                    * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                                        as libc::c_int)) as __fd_mask;
                                }
                                if (*ev).ev_goal & (1 as libc::c_int) << 13 as libc::c_int
                                    != 0
                                {
                                    wfds
                                        .__fds_bits[((*ev).ev_args.FD.fd
                                        / (8 as libc::c_int
                                            * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                                as libc::c_int)) as usize]
                                        |= ((1 as libc::c_ulong)
                                            << (*ev).ev_args.FD.fd
                                                % (8 as libc::c_int
                                                    * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                                        as libc::c_int)) as __fd_mask;
                                }
                                if (*ev).ev_goal & (1 as libc::c_int) << 14 as libc::c_int
                                    != 0
                                {
                                    efds
                                        .__fds_bits[((*ev).ev_args.FD.fd
                                        / (8 as libc::c_int
                                            * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                                as libc::c_int)) as usize]
                                        |= ((1 as libc::c_ulong)
                                            << (*ev).ev_args.FD.fd
                                                % (8 as libc::c_int
                                                    * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                                        as libc::c_int)) as __fd_mask;
                                }
                                if (&mut __pth_time_zero as *mut pth_time_t).is_null() {
                                    gettimeofday(&mut delay, 0 as *mut libc::c_void);
                                } else {
                                    delay.tv_sec = __pth_time_zero.tv_sec;
                                    delay.tv_usec = __pth_time_zero.tv_usec;
                                }
                                loop {
                                    rc2 = select(
                                        (*ev).ev_args.FD.fd + 1 as libc::c_int,
                                        &mut rfds,
                                        &mut wfds,
                                        &mut efds,
                                        &mut delay,
                                    );
                                    if !(rc2 < 0 as libc::c_int
                                        && *__errno_location() == 4 as libc::c_int)
                                    {
                                        break;
                                    }
                                }
                                if rc2 > 0 as libc::c_int {
                                    rfds
                                        .__fds_bits[((*ev).ev_args.FD.fd
                                        / (8 as libc::c_int
                                            * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                                as libc::c_int)) as usize]
                                        &= !(((1 as libc::c_ulong)
                                            << (*ev).ev_args.FD.fd
                                                % (8 as libc::c_int
                                                    * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                                        as libc::c_int)) as __fd_mask);
                                    wfds
                                        .__fds_bits[((*ev).ev_args.FD.fd
                                        / (8 as libc::c_int
                                            * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                                as libc::c_int)) as usize]
                                        &= !(((1 as libc::c_ulong)
                                            << (*ev).ev_args.FD.fd
                                                % (8 as libc::c_int
                                                    * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                                        as libc::c_int)) as __fd_mask);
                                    efds
                                        .__fds_bits[((*ev).ev_args.FD.fd
                                        / (8 as libc::c_int
                                            * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                                as libc::c_int)) as usize]
                                        &= !(((1 as libc::c_ulong)
                                            << (*ev).ev_args.FD.fd
                                                % (8 as libc::c_int
                                                    * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                                        as libc::c_int)) as __fd_mask);
                                } else if rc2 < 0 as libc::c_int {
                                    let mut __d0_5: libc::c_int = 0;
                                    let mut __d1_5: libc::c_int = 0;
                                    let fresh36 = &mut __d0_5;
                                    let fresh37;
                                    let fresh38 = (::std::mem::size_of::<fd_set>()
                                        as libc::c_ulong)
                                        .wrapping_div(
                                            ::std::mem::size_of::<__fd_mask>() as libc::c_ulong,
                                        );
                                    let fresh39 = &mut __d1_5;
                                    let fresh40;
                                    let fresh41 = &mut *(rfds.__fds_bits)
                                        .as_mut_ptr()
                                        .offset(0 as libc::c_int as isize) as *mut __fd_mask;
                                    asm!(
                                        "cld; rep; stosq", inlateout("cx")
                                        c2rust_asm_casts::AsmCast::cast_in(fresh36, fresh38) =>
                                        fresh37, inlateout("di")
                                        c2rust_asm_casts::AsmCast::cast_in(fresh39, fresh41) =>
                                        fresh40, inlateout("ax") 0 as libc::c_int => _,
                                        options(preserves_flags, att_syntax)
                                    );
                                    c2rust_asm_casts::AsmCast::cast_out(
                                        fresh36,
                                        fresh38,
                                        fresh37,
                                    );
                                    c2rust_asm_casts::AsmCast::cast_out(
                                        fresh39,
                                        fresh41,
                                        fresh40,
                                    );
                                    let mut __d0_6: libc::c_int = 0;
                                    let mut __d1_6: libc::c_int = 0;
                                    let fresh42 = &mut __d0_6;
                                    let fresh43;
                                    let fresh44 = (::std::mem::size_of::<fd_set>()
                                        as libc::c_ulong)
                                        .wrapping_div(
                                            ::std::mem::size_of::<__fd_mask>() as libc::c_ulong,
                                        );
                                    let fresh45 = &mut __d1_6;
                                    let fresh46;
                                    let fresh47 = &mut *(wfds.__fds_bits)
                                        .as_mut_ptr()
                                        .offset(0 as libc::c_int as isize) as *mut __fd_mask;
                                    asm!(
                                        "cld; rep; stosq", inlateout("cx")
                                        c2rust_asm_casts::AsmCast::cast_in(fresh42, fresh44) =>
                                        fresh43, inlateout("di")
                                        c2rust_asm_casts::AsmCast::cast_in(fresh45, fresh47) =>
                                        fresh46, inlateout("ax") 0 as libc::c_int => _,
                                        options(preserves_flags, att_syntax)
                                    );
                                    c2rust_asm_casts::AsmCast::cast_out(
                                        fresh42,
                                        fresh44,
                                        fresh43,
                                    );
                                    c2rust_asm_casts::AsmCast::cast_out(
                                        fresh45,
                                        fresh47,
                                        fresh46,
                                    );
                                    let mut __d0_7: libc::c_int = 0;
                                    let mut __d1_7: libc::c_int = 0;
                                    let fresh48 = &mut __d0_7;
                                    let fresh49;
                                    let fresh50 = (::std::mem::size_of::<fd_set>()
                                        as libc::c_ulong)
                                        .wrapping_div(
                                            ::std::mem::size_of::<__fd_mask>() as libc::c_ulong,
                                        );
                                    let fresh51 = &mut __d1_7;
                                    let fresh52;
                                    let fresh53 = &mut *(efds.__fds_bits)
                                        .as_mut_ptr()
                                        .offset(0 as libc::c_int as isize) as *mut __fd_mask;
                                    asm!(
                                        "cld; rep; stosq", inlateout("cx")
                                        c2rust_asm_casts::AsmCast::cast_in(fresh48, fresh50) =>
                                        fresh49, inlateout("di")
                                        c2rust_asm_casts::AsmCast::cast_in(fresh51, fresh53) =>
                                        fresh52, inlateout("ax") 0 as libc::c_int => _,
                                        options(preserves_flags, att_syntax)
                                    );
                                    c2rust_asm_casts::AsmCast::cast_out(
                                        fresh48,
                                        fresh50,
                                        fresh49,
                                    );
                                    c2rust_asm_casts::AsmCast::cast_out(
                                        fresh51,
                                        fresh53,
                                        fresh52,
                                    );
                                    (*ev).ev_status = PTH_STATUS_FAILED;
                                }
                            }
                        } else if (*ev).ev_type == (1 as libc::c_int) << 2 as libc::c_int
                        {
                            if __pth_util_fds_test(
                                (*ev).ev_args.SELECT.nfd,
                                (*ev).ev_args.SELECT.rfds,
                                &mut rfds,
                                (*ev).ev_args.SELECT.wfds,
                                &mut wfds,
                                (*ev).ev_args.SELECT.efds,
                                &mut efds,
                            ) != 0
                            {
                                n = __pth_util_fds_select(
                                    (*ev).ev_args.SELECT.nfd,
                                    (*ev).ev_args.SELECT.rfds,
                                    &mut rfds,
                                    (*ev).ev_args.SELECT.wfds,
                                    &mut wfds,
                                    (*ev).ev_args.SELECT.efds,
                                    &mut efds,
                                );
                                if !((*ev).ev_args.SELECT.n).is_null() {
                                    *(*ev).ev_args.SELECT.n = n;
                                }
                                (*ev).ev_status = PTH_STATUS_OCCURRED;
                            } else if rc < 0 as libc::c_int {
                                let mut rc2_0: libc::c_int = 0;
                                let mut prfds: *mut fd_set = 0 as *mut fd_set;
                                let mut pwfds: *mut fd_set = 0 as *mut fd_set;
                                let mut pefds: *mut fd_set = 0 as *mut fd_set;
                                let mut trfds: fd_set = fd_set { __fds_bits: [0; 16] };
                                let mut twfds: fd_set = fd_set { __fds_bits: [0; 16] };
                                let mut tefds: fd_set = fd_set { __fds_bits: [0; 16] };
                                if !((*ev).ev_args.SELECT.rfds).is_null() {
                                    memcpy(
                                        &mut trfds as *mut fd_set as *mut libc::c_void,
                                        (*ev).ev_args.SELECT.rfds as *const libc::c_void,
                                        ::std::mem::size_of::<fd_set>() as libc::c_ulong,
                                    );
                                    prfds = &mut trfds;
                                }
                                if !((*ev).ev_args.SELECT.wfds).is_null() {
                                    memcpy(
                                        &mut twfds as *mut fd_set as *mut libc::c_void,
                                        (*ev).ev_args.SELECT.wfds as *const libc::c_void,
                                        ::std::mem::size_of::<fd_set>() as libc::c_ulong,
                                    );
                                    pwfds = &mut twfds;
                                }
                                if !((*ev).ev_args.SELECT.efds).is_null() {
                                    memcpy(
                                        &mut tefds as *mut fd_set as *mut libc::c_void,
                                        (*ev).ev_args.SELECT.efds as *const libc::c_void,
                                        ::std::mem::size_of::<fd_set>() as libc::c_ulong,
                                    );
                                    pefds = &mut tefds;
                                }
                                if (&mut __pth_time_zero as *mut pth_time_t).is_null() {
                                    gettimeofday(&mut delay, 0 as *mut libc::c_void);
                                } else {
                                    delay.tv_sec = __pth_time_zero.tv_sec;
                                    delay.tv_usec = __pth_time_zero.tv_usec;
                                }
                                loop {
                                    rc2_0 = select(
                                        (*ev).ev_args.SELECT.nfd + 1 as libc::c_int,
                                        prfds,
                                        pwfds,
                                        pefds,
                                        &mut delay,
                                    );
                                    if !(rc2_0 < 0 as libc::c_int
                                        && *__errno_location() == 4 as libc::c_int)
                                    {
                                        break;
                                    }
                                }
                                if rc2_0 < 0 as libc::c_int {
                                    (*ev).ev_status = PTH_STATUS_FAILED;
                                }
                            }
                        } else if (*ev).ev_type == (1 as libc::c_int) << 3 as libc::c_int
                        {
                            sig = 1 as libc::c_int;
                            while sig < 65 as libc::c_int {
                                if sigismember((*ev).ev_args.SIGS.sigs, sig) != 0 {
                                    if sigismember(&mut pth_sigraised, sig) != 0 {
                                        if !((*ev).ev_args.SIGS.sig).is_null() {
                                            *(*ev).ev_args.SIGS.sig = sig;
                                        }
                                        sigdelset(&mut pth_sigraised, sig);
                                        (*ev).ev_status = PTH_STATUS_OCCURRED;
                                    }
                                }
                                sig += 1;
                                sig;
                            }
                        }
                    } else if (*ev).ev_type == (1 as libc::c_int) << 7 as libc::c_int {
                        if (*(*ev).ev_args.COND.cond).cn_state
                            & ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_ulong
                            != 0
                        {
                            (*(*ev).ev_args.COND.cond).cn_state
                                &= !((1 as libc::c_int) << 1 as libc::c_int)
                                    as libc::c_ulong;
                            (*(*ev).ev_args.COND.cond).cn_state
                                &= !((1 as libc::c_int) << 2 as libc::c_int)
                                    as libc::c_ulong;
                            (*(*ev).ev_args.COND.cond).cn_state
                                &= !((1 as libc::c_int) << 3 as libc::c_int)
                                    as libc::c_ulong;
                        }
                    }
                    if (*ev).ev_status as libc::c_uint
                        != PTH_STATUS_PENDING as libc::c_int as libc::c_uint
                    {
                        any_occurred = (0 as libc::c_int == 0) as libc::c_int;
                    }
                    ev = (*ev).ev_next;
                    if !(ev != evh) {
                        break;
                    }
                }
            }
            if (*t).cancelreq == (0 as libc::c_int == 0) as libc::c_int {
                any_occurred = (0 as libc::c_int == 0) as libc::c_int;
            }
            tlast = t;
            t = __pth_pqueue_walk(
                &mut __pth_WQ,
                t,
                (1 as libc::c_int) << 1 as libc::c_int,
            );
            if any_occurred != 0 {
                __pth_pqueue_delete(&mut __pth_WQ, tlast);
                (*tlast).state = PTH_STATE_READY;
                __pth_pqueue_insert(
                    &mut __pth_RQ,
                    (*tlast).prio + 1 as libc::c_int,
                    tlast,
                );
            }
        }
        if !(loop_repeat != 0) {
            break;
        }
        if (0 as *mut pth_time_t).is_null() {
            gettimeofday(now, 0 as *mut libc::c_void);
        } else {
            (*now).tv_sec = (*(0 as *mut pth_time_t)).tv_sec;
            (*now).tv_usec = (*(0 as *mut pth_time_t)).tv_usec;
        }
    };
}
pub unsafe extern "C" fn __pth_sched_eventmanager_sighandler(mut sig: libc::c_int) {
    let mut c: libc::c_char = 0;
    sigaddset(&mut pth_sigraised, sig);
    c = sig as libc::c_char;
    write(
        pth_sigpipe[1 as libc::c_int as usize],
        &mut c as *mut libc::c_char as *const libc::c_void,
        ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
    );
}
