use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn abort() -> !;
    fn exit(_: libc::c_int) -> !;
    fn sigemptyset(__set: *mut sigset_t) -> libc::c_int;
    fn sigaddset(__set: *mut sigset_t, __signo: libc::c_int) -> libc::c_int;
    fn sigismember(__set: *const sigset_t, __signo: libc::c_int) -> libc::c_int;
    fn sigaction(
        __sig: libc::c_int,
        __act: *const sigaction,
        __oact: *mut sigaction,
    ) -> libc::c_int;
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn gettimeofday(__tv: *mut timeval, __tz: *mut libc::c_void) -> libc::c_int;
    fn time(__timer: *mut time_t) -> time_t;
    static mut __pth_sched: pth_t;
    static mut __pth_main: pth_t;
    fn swapcontext(__oucp: *mut ucontext_t, __ucp: *const ucontext_t) -> libc::c_int;
    static mut __pth_current: pth_t;
    fn pth_attr_destroy(_: pth_attr_t) -> libc::c_int;
    fn __pth_syscall_kill();
    fn __pth_scheduler_kill();
    static mut __pth_errno_flag: libc::c_int;
    static mut __pth_errno_storage: libc::c_int;
    static mut __pth_NQ: pth_pqueue_t;
    fn __pth_pqueue_insert(_: *mut pth_pqueue_t, _: libc::c_int, _: pth_t);
    fn __pth_scheduler(_: *mut libc::c_void) -> *mut libc::c_void;
    fn __pth_tcb_free(_: pth_t);
    fn __pth_mutex_releaseall(_: pth_t);
    fn __pth_key_destroydata(_: pth_t);
    fn __pth_cleanup_popall(_: pth_t, _: libc::c_int);
    fn pth_event_free(_: pth_event_t, _: libc::c_int) -> libc::c_int;
    fn pth_wait(_: pth_event_t) -> libc::c_int;
    static mut __pth_SQ: pth_pqueue_t;
    static mut __pth_WQ: pth_pqueue_t;
    static mut __pth_RQ: pth_pqueue_t;
    fn pth_event(_: libc::c_ulong, _: ...) -> pth_event_t;
    fn __pth_mctx_set(
        _: *mut pth_mctx_t,
        _: Option::<unsafe extern "C" fn() -> ()>,
        _: *mut libc::c_char,
        _: *mut libc::c_char,
    ) -> libc::c_int;
    fn __pth_ring_init(_: *mut pth_ring_t);
    static mut __pth_time_zero: pth_time_t;
    fn __pth_snprintf(
        _: *mut libc::c_char,
        _: size_t,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn __pth_util_cpystrn(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: size_t,
    ) -> *mut libc::c_char;
    fn __pth_tcb_alloc(_: libc::c_uint, _: *mut libc::c_void) -> pth_t;
    fn pth_attr_set(_: pth_attr_t, _: libc::c_int, _: ...) -> libc::c_int;
    fn pth_attr_new() -> pth_attr_t;
    fn __pth_scheduler_init() -> libc::c_int;
    fn __pth_syscall_init();
    static mut __pth_favournew: libc::c_int;
    fn __pth_dumpstate(_: *mut FILE);
    static mut __pth_loadval: libc::c_float;
    static mut __pth_DQ: pth_pqueue_t;
    fn __pth_pqueue_delete(_: *mut pth_pqueue_t, _: pth_t);
    fn __pth_pqueue_contains(_: *mut pth_pqueue_t, _: pth_t) -> libc::c_int;
    fn __pth_pqueue_favorite(_: *mut pth_pqueue_t, _: pth_t) -> libc::c_int;
    fn __pth_time_cmp(_: *mut pth_time_t, _: *mut pth_time_t) -> libc::c_int;
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
pub type time_t = __time_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pth_attr_st {
    pub a_tid: pth_t,
    pub a_prio: libc::c_int,
    pub a_dispatches: libc::c_int,
    pub a_name: [libc::c_char; 40],
    pub a_joinable: libc::c_int,
    pub a_cancelstate: libc::c_uint,
    pub a_stacksize: libc::c_uint,
    pub a_stackaddr: *mut libc::c_char,
}
pub type pth_attr_t = *mut pth_attr_st;
pub type C2RustUnnamed_20 = libc::c_uint;
pub const PTH_ATTR_BOUND: C2RustUnnamed_20 = 14;
pub const PTH_ATTR_EVENTS: C2RustUnnamed_20 = 13;
pub const PTH_ATTR_STATE: C2RustUnnamed_20 = 12;
pub const PTH_ATTR_START_ARG: C2RustUnnamed_20 = 11;
pub const PTH_ATTR_START_FUNC: C2RustUnnamed_20 = 10;
pub const PTH_ATTR_TIME_RAN: C2RustUnnamed_20 = 9;
pub const PTH_ATTR_TIME_LAST: C2RustUnnamed_20 = 8;
pub const PTH_ATTR_TIME_SPAWN: C2RustUnnamed_20 = 7;
pub const PTH_ATTR_DISPATCHES: C2RustUnnamed_20 = 6;
pub const PTH_ATTR_STACK_ADDR: C2RustUnnamed_20 = 5;
pub const PTH_ATTR_STACK_SIZE: C2RustUnnamed_20 = 4;
pub const PTH_ATTR_CANCEL_STATE: C2RustUnnamed_20 = 3;
pub const PTH_ATTR_JOINABLE: C2RustUnnamed_20 = 2;
pub const PTH_ATTR_NAME: C2RustUnnamed_20 = 1;
pub const PTH_ATTR_PRIO: C2RustUnnamed_20 = 0;
pub type C2RustUnnamed_21 = libc::c_uint;
pub const PTH_FREE_ALL: C2RustUnnamed_21 = 1;
pub const PTH_FREE_THIS: C2RustUnnamed_21 = 0;
pub type pth_key_t = libc::c_int;
pub type pth_once_t = libc::c_int;
pub type C2RustUnnamed_22 = libc::c_int;
pub const PTH_FDMODE_NONBLOCK: C2RustUnnamed_22 = 2;
pub const PTH_FDMODE_BLOCK: C2RustUnnamed_22 = 1;
pub const PTH_FDMODE_POLL: C2RustUnnamed_22 = 0;
pub const PTH_FDMODE_ERROR: C2RustUnnamed_22 = -1;
pub type pth_pqueue_t = pth_pqueue_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pth_pqueue_st {
    pub q_head: pth_t,
    pub q_num: libc::c_int,
}
pub unsafe extern "C" fn pth_version() -> libc::c_long {
    return 0x200207 as libc::c_int as libc::c_long;
}
pub static mut __pth_initialized: libc::c_int = 0 as libc::c_int;
pub unsafe extern "C" fn pth_init() -> libc::c_int {
    let mut t_attr: pth_attr_t = 0 as *mut pth_attr_st;
    if __pth_initialized != 0 {
        *__errno_location() = 1 as libc::c_int;
        return 0 as libc::c_int;
    } else {
        __pth_initialized = (0 as libc::c_int == 0) as libc::c_int;
    }
    __pth_syscall_init();
    if __pth_scheduler_init() == 0 {
        __pth_errno_storage = *__errno_location();
        __pth_errno_flag = (0 as libc::c_int == 0) as libc::c_int;
        while __pth_errno_flag != 0 {
            __pth_syscall_kill();
            *__errno_location() = __pth_errno_storage;
            __pth_errno_flag = 0 as libc::c_int;
        }
        *__errno_location() = 11 as libc::c_int;
        return 0 as libc::c_int;
    }
    t_attr = pth_attr_new();
    pth_attr_set(t_attr, PTH_ATTR_PRIO as libc::c_int, 5 as libc::c_int);
    pth_attr_set(
        t_attr,
        PTH_ATTR_NAME as libc::c_int,
        b"**SCHEDULER**\0" as *const u8 as *const libc::c_char,
    );
    pth_attr_set(t_attr, PTH_ATTR_JOINABLE as libc::c_int, 0 as libc::c_int);
    pth_attr_set(
        t_attr,
        PTH_ATTR_CANCEL_STATE as libc::c_int,
        (1 as libc::c_int) << 1 as libc::c_int,
    );
    pth_attr_set(
        t_attr,
        PTH_ATTR_STACK_SIZE as libc::c_int,
        64 as libc::c_int * 1024 as libc::c_int,
    );
    pth_attr_set(t_attr, PTH_ATTR_STACK_ADDR as libc::c_int, 0 as *mut libc::c_void);
    __pth_sched = pth_spawn(
        t_attr,
        Some(
            __pth_scheduler
                as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
        ),
        0 as *mut libc::c_void,
    );
    if __pth_sched.is_null() {
        __pth_errno_storage = *__errno_location();
        __pth_errno_flag = (0 as libc::c_int == 0) as libc::c_int;
        while __pth_errno_flag != 0 {
            pth_attr_destroy(t_attr);
            __pth_scheduler_kill();
            __pth_syscall_kill();
            *__errno_location() = __pth_errno_storage;
            __pth_errno_flag = 0 as libc::c_int;
        }
        return 0 as libc::c_int;
    }
    pth_attr_set(t_attr, PTH_ATTR_PRIO as libc::c_int, 0 as libc::c_int);
    pth_attr_set(
        t_attr,
        PTH_ATTR_NAME as libc::c_int,
        b"main\0" as *const u8 as *const libc::c_char,
    );
    pth_attr_set(
        t_attr,
        PTH_ATTR_JOINABLE as libc::c_int,
        (0 as libc::c_int == 0) as libc::c_int,
    );
    pth_attr_set(
        t_attr,
        PTH_ATTR_CANCEL_STATE as libc::c_int,
        (1 as libc::c_int) << 0 as libc::c_int | (1 as libc::c_int) << 3 as libc::c_int,
    );
    pth_attr_set(t_attr, PTH_ATTR_STACK_SIZE as libc::c_int, 0 as libc::c_int);
    pth_attr_set(t_attr, PTH_ATTR_STACK_ADDR as libc::c_int, 0 as *mut libc::c_void);
    __pth_main = pth_spawn(
        t_attr,
        ::std::mem::transmute::<
            libc::intptr_t,
            Option::<unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void>,
        >(-(1 as libc::c_int) as libc::intptr_t),
        0 as *mut libc::c_void,
    );
    if __pth_main.is_null() {
        __pth_errno_storage = *__errno_location();
        __pth_errno_flag = (0 as libc::c_int == 0) as libc::c_int;
        while __pth_errno_flag != 0 {
            pth_attr_destroy(t_attr);
            __pth_scheduler_kill();
            __pth_syscall_kill();
            *__errno_location() = __pth_errno_storage;
            __pth_errno_flag = 0 as libc::c_int;
        }
        return 0 as libc::c_int;
    }
    pth_attr_destroy(t_attr);
    __pth_current = __pth_sched;
    swapcontext(&mut (*__pth_main).mctx.uc, &mut (*__pth_sched).mctx.uc);
    return (0 as libc::c_int == 0) as libc::c_int;
}
pub unsafe extern "C" fn pth_kill() -> libc::c_int {
    if __pth_initialized == 0 {
        *__errno_location() = 22 as libc::c_int;
        return 0 as libc::c_int;
    }
    if __pth_current != __pth_main {
        *__errno_location() = 1 as libc::c_int;
        return 0 as libc::c_int;
    }
    __pth_thread_cleanup(__pth_main);
    __pth_scheduler_kill();
    __pth_initialized = 0 as libc::c_int;
    __pth_tcb_free(__pth_sched);
    __pth_tcb_free(__pth_main);
    __pth_syscall_kill();
    return (0 as libc::c_int == 0) as libc::c_int;
}
pub unsafe extern "C" fn pth_ctrl(
    mut query: libc::c_ulong,
    mut args: ...
) -> libc::c_long {
    let mut rc: libc::c_long = 0;
    let mut ap: ::std::ffi::VaListImpl;
    rc = 0 as libc::c_int as libc::c_long;
    ap = args.clone();
    if query
        & ((1 as libc::c_int) << 4 as libc::c_int
            | (1 as libc::c_int) << 5 as libc::c_int
            | (1 as libc::c_int) << 6 as libc::c_int
            | (1 as libc::c_int) << 7 as libc::c_int
            | (1 as libc::c_int) << 8 as libc::c_int
            | (1 as libc::c_int) << 9 as libc::c_int) as libc::c_ulong != 0
    {
        if query & ((1 as libc::c_int) << 4 as libc::c_int) as libc::c_ulong != 0 {
            rc
                += (if (&mut __pth_NQ as *mut pth_pqueue_t).is_null() {
                    -(1 as libc::c_int)
                } else {
                    __pth_NQ.q_num
                }) as libc::c_long;
        }
        if query & ((1 as libc::c_int) << 5 as libc::c_int) as libc::c_ulong != 0 {
            rc
                += (if (&mut __pth_RQ as *mut pth_pqueue_t).is_null() {
                    -(1 as libc::c_int)
                } else {
                    __pth_RQ.q_num
                }) as libc::c_long;
        }
        if query & ((1 as libc::c_int) << 6 as libc::c_int) as libc::c_ulong != 0 {
            rc += 1 as libc::c_int as libc::c_long;
        }
        if query & ((1 as libc::c_int) << 7 as libc::c_int) as libc::c_ulong != 0 {
            rc
                += (if (&mut __pth_WQ as *mut pth_pqueue_t).is_null() {
                    -(1 as libc::c_int)
                } else {
                    __pth_WQ.q_num
                }) as libc::c_long;
        }
        if query & ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_ulong != 0 {
            rc
                += (if (&mut __pth_SQ as *mut pth_pqueue_t).is_null() {
                    -(1 as libc::c_int)
                } else {
                    __pth_SQ.q_num
                }) as libc::c_long;
        }
        if query & ((1 as libc::c_int) << 9 as libc::c_int) as libc::c_ulong != 0 {
            rc
                += (if (&mut __pth_DQ as *mut pth_pqueue_t).is_null() {
                    -(1 as libc::c_int)
                } else {
                    __pth_DQ.q_num
                }) as libc::c_long;
        }
    } else if query & ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_ulong != 0 {
        let mut pload: *mut libc::c_float = ap.arg::<*mut libc::c_float>();
        *pload = __pth_loadval;
    } else if query & ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_ulong != 0 {
        let mut t: pth_t = ap.arg::<pth_t>();
        rc = (*t).prio as libc::c_long;
    } else if query & ((1 as libc::c_int) << 3 as libc::c_int) as libc::c_ulong != 0 {
        let mut t_0: pth_t = ap.arg::<pth_t>();
        rc = ((*t_0).name).as_mut_ptr() as libc::c_long;
    } else if query & ((1 as libc::c_int) << 10 as libc::c_int) as libc::c_ulong != 0 {
        let mut fp: *mut FILE = ap.arg::<*mut FILE>();
        __pth_dumpstate(fp);
    } else if query & ((1 as libc::c_int) << 11 as libc::c_int) as libc::c_ulong != 0 {
        let mut favournew: libc::c_int = ap.arg::<libc::c_int>();
        __pth_favournew = if favournew != 0 {
            1 as libc::c_int
        } else {
            0 as libc::c_int
        };
    } else {
        rc = -(1 as libc::c_int) as libc::c_long;
    }
    if rc == -(1 as libc::c_int) as libc::c_long {
        *__errno_location() = 22 as libc::c_int;
        return -(1 as libc::c_int) as libc::c_long;
    }
    return rc;
}
unsafe extern "C" fn pth_spawn_trampoline() {
    let mut data: *mut libc::c_void = 0 as *mut libc::c_void;
    data = (Some(((*__pth_current).start_func).unwrap()))
        .unwrap()((*__pth_current).start_arg);
    pth_exit(data);
    abort();
}
pub unsafe extern "C" fn pth_spawn(
    mut attr: pth_attr_t,
    mut func: Option::<unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void>,
    mut arg: *mut libc::c_void,
) -> pth_t {
    let mut t: pth_t = 0 as *mut pth_st;
    let mut stacksize: libc::c_uint = 0;
    let mut stackaddr: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut ts: pth_time_t = pth_time_t {
        tv_sec: 0,
        tv_usec: 0,
    };
    if func.is_none() {
        *__errno_location() = 22 as libc::c_int;
        return 0 as *mut libc::c_void as pth_t;
    }
    if func
        == ::std::mem::transmute::<
            libc::intptr_t,
            Option::<unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void>,
        >(-(1 as libc::c_int) as libc::intptr_t)
    {
        func = None;
    }
    stacksize = if attr.is_null() {
        (64 as libc::c_int * 1024 as libc::c_int) as libc::c_uint
    } else {
        (*attr).a_stacksize
    };
    stackaddr = (if attr.is_null() {
        0 as *mut libc::c_char
    } else {
        (*attr).a_stackaddr
    }) as *mut libc::c_void;
    t = __pth_tcb_alloc(stacksize, stackaddr);
    if t.is_null() {
        *__errno_location() = *__errno_location();
        return 0 as *mut libc::c_void as pth_t;
    }
    if !attr.is_null() {
        (*t).prio = (*attr).a_prio;
        (*t).joinable = (*attr).a_joinable;
        (*t).cancelstate = (*attr).a_cancelstate;
        (*t).dispatches = (*attr).a_dispatches;
        __pth_util_cpystrn(
            ((*t).name).as_mut_ptr(),
            ((*attr).a_name).as_mut_ptr(),
            40 as libc::c_int as size_t,
        );
    } else if !__pth_current.is_null() {
        (*t).prio = (*__pth_current).prio;
        (*t).joinable = (*__pth_current).joinable;
        (*t).cancelstate = (*__pth_current).cancelstate;
        (*t).dispatches = 0 as libc::c_int;
        __pth_snprintf(
            ((*t).name).as_mut_ptr(),
            40 as libc::c_int as size_t,
            b"%s.child@%d=0x%lx\0" as *const u8 as *const libc::c_char,
            ((*__pth_current).name).as_mut_ptr(),
            time(0 as *mut time_t) as libc::c_uint,
            __pth_current as libc::c_ulong,
        );
    } else {
        (*t).prio = 0 as libc::c_int;
        (*t).joinable = (0 as libc::c_int == 0) as libc::c_int;
        (*t)
            .cancelstate = ((1 as libc::c_int) << 0 as libc::c_int
            | (1 as libc::c_int) << 3 as libc::c_int) as libc::c_uint;
        (*t).dispatches = 0 as libc::c_int;
        __pth_snprintf(
            ((*t).name).as_mut_ptr(),
            40 as libc::c_int as size_t,
            b"user/%x\0" as *const u8 as *const libc::c_char,
            time(0 as *mut time_t) as libc::c_uint,
        );
    }
    if (0 as *mut pth_time_t).is_null() {
        gettimeofday(&mut ts, 0 as *mut libc::c_void);
    } else {
        ts.tv_sec = (*(0 as *mut pth_time_t)).tv_sec;
        ts.tv_usec = (*(0 as *mut pth_time_t)).tv_usec;
    }
    if (&mut ts as *mut pth_time_t).is_null() {
        gettimeofday(&mut (*t).spawned, 0 as *mut libc::c_void);
    } else {
        (*t).spawned.tv_sec = ts.tv_sec;
        (*t).spawned.tv_usec = ts.tv_usec;
    }
    if (&mut ts as *mut pth_time_t).is_null() {
        gettimeofday(&mut (*t).lastran, 0 as *mut libc::c_void);
    } else {
        (*t).lastran.tv_sec = ts.tv_sec;
        (*t).lastran.tv_usec = ts.tv_usec;
    }
    if (&mut __pth_time_zero as *mut pth_time_t).is_null() {
        gettimeofday(&mut (*t).running, 0 as *mut libc::c_void);
    } else {
        (*t).running.tv_sec = __pth_time_zero.tv_sec;
        (*t).running.tv_usec = __pth_time_zero.tv_usec;
    }
    (*t).events = 0 as pth_event_t;
    sigemptyset(&mut (*t).sigpending);
    (*t).sigpendcnt = 0 as libc::c_int;
    (*t).start_func = func;
    (*t).start_arg = arg;
    (*t).join_arg = 0 as *mut libc::c_void;
    (*t).data_value = 0 as *mut *const libc::c_void;
    (*t).data_count = 0 as libc::c_int;
    (*t).cancelreq = 0 as libc::c_int;
    (*t).cleanups = 0 as *mut pth_cleanup_t;
    __pth_ring_init(&mut (*t).mutexring);
    if (*t).stacksize > 0 as libc::c_int as libc::c_uint {
        if __pth_mctx_set(
            &mut (*t).mctx,
            Some(pth_spawn_trampoline as unsafe extern "C" fn() -> ()),
            (*t).stack,
            ((*t).stack).offset((*t).stacksize as isize),
        ) == 0
        {
            __pth_errno_storage = *__errno_location();
            __pth_errno_flag = (0 as libc::c_int == 0) as libc::c_int;
            while __pth_errno_flag != 0 {
                __pth_tcb_free(t);
                *__errno_location() = __pth_errno_storage;
                __pth_errno_flag = 0 as libc::c_int;
            }
            *__errno_location() = *__errno_location();
            return 0 as *mut libc::c_void as pth_t;
        }
    }
    if func
        != Some(
            __pth_scheduler
                as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
        )
    {
        (*t).state = PTH_STATE_NEW;
        __pth_pqueue_insert(&mut __pth_NQ, (*t).prio, t);
    }
    return t;
}
pub unsafe extern "C" fn pth_self() -> pth_t {
    return __pth_current;
}
pub unsafe extern "C" fn pth_raise(mut t: pth_t, mut sig: libc::c_int) -> libc::c_int {
    let mut sa: sigaction = sigaction {
        __sigaction_handler: C2RustUnnamed_9 {
            sa_handler: None,
        },
        sa_mask: __sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    if t.is_null() || t == __pth_current
        || (sig < 0 as libc::c_int || sig > 65 as libc::c_int)
    {
        *__errno_location() = 22 as libc::c_int;
        return 0 as libc::c_int;
    }
    if sig == 0 as libc::c_int {
        return __pth_thread_exists(t)
    } else {
        if sigaction(sig, 0 as *const sigaction, &mut sa) != 0 as libc::c_int {
            return 0 as libc::c_int;
        }
        if sa.__sigaction_handler.sa_handler
            == ::std::mem::transmute::<
                libc::intptr_t,
                __sighandler_t,
            >(1 as libc::c_int as libc::intptr_t)
        {
            return (0 as libc::c_int == 0) as libc::c_int;
        }
        if sigismember(&mut (*t).sigpending, sig) == 0 {
            sigaddset(&mut (*t).sigpending, sig);
            (*t).sigpendcnt += 1;
            (*t).sigpendcnt;
        }
        pth_yield(t);
        return (0 as libc::c_int == 0) as libc::c_int;
    };
}
pub unsafe extern "C" fn __pth_thread_exists(mut t: pth_t) -> libc::c_int {
    if __pth_pqueue_contains(&mut __pth_NQ, t) == 0 {
        if __pth_pqueue_contains(&mut __pth_RQ, t) == 0 {
            if __pth_pqueue_contains(&mut __pth_WQ, t) == 0 {
                if __pth_pqueue_contains(&mut __pth_SQ, t) == 0 {
                    if __pth_pqueue_contains(&mut __pth_DQ, t) == 0 {
                        *__errno_location() = 3 as libc::c_int;
                        return 0 as libc::c_int;
                    }
                }
            }
        }
    }
    return (0 as libc::c_int == 0) as libc::c_int;
}
pub unsafe extern "C" fn __pth_thread_cleanup(mut thread: pth_t) {
    if !((*thread).cleanups).is_null() {
        __pth_cleanup_popall(thread, (0 as libc::c_int == 0) as libc::c_int);
    }
    if !((*thread).data_value).is_null() {
        __pth_key_destroydata(thread);
    }
    __pth_mutex_releaseall(thread);
}
unsafe extern "C" fn pth_exit_cb(mut arg: *mut libc::c_void) -> libc::c_int {
    let mut rc: libc::c_int = 0;
    rc = 0 as libc::c_int;
    rc
        += if (&mut __pth_NQ as *mut pth_pqueue_t).is_null() {
            -(1 as libc::c_int)
        } else {
            __pth_NQ.q_num
        };
    rc
        += if (&mut __pth_RQ as *mut pth_pqueue_t).is_null() {
            -(1 as libc::c_int)
        } else {
            __pth_RQ.q_num
        };
    rc
        += if (&mut __pth_WQ as *mut pth_pqueue_t).is_null() {
            -(1 as libc::c_int)
        } else {
            __pth_WQ.q_num
        };
    rc
        += if (&mut __pth_SQ as *mut pth_pqueue_t).is_null() {
            -(1 as libc::c_int)
        } else {
            __pth_SQ.q_num
        };
    if rc == 1 as libc::c_int {
        return (0 as libc::c_int == 0) as libc::c_int
    } else {
        return 0 as libc::c_int
    };
}
pub unsafe extern "C" fn pth_exit(mut value: *mut libc::c_void) {
    let mut ev: pth_event_t = 0 as *mut pth_event_st;
    if __pth_current == __pth_main {
        if pth_exit_cb(0 as *mut libc::c_void) == 0 {
            ev = pth_event(
                ((1 as libc::c_int) << 9 as libc::c_int) as libc::c_ulong,
                Some(
                    pth_exit_cb as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int,
                ),
            );
            pth_wait(ev);
            pth_event_free(ev, PTH_FREE_THIS as libc::c_int);
        }
    }
    __pth_thread_cleanup(__pth_current);
    if __pth_current != __pth_main {
        (*__pth_current).join_arg = value;
        (*__pth_current).state = PTH_STATE_DEAD;
        swapcontext(&mut (*__pth_current).mctx.uc, &mut (*__pth_sched).mctx.uc);
    } else {
        pth_kill();
        exit(value as libc::c_long as libc::c_int);
    }
    abort();
}
pub unsafe extern "C" fn pth_join(
    mut tid: pth_t,
    mut value: *mut *mut libc::c_void,
) -> libc::c_int {
    let mut ev: pth_event_t = 0 as *mut pth_event_st;
    static mut ev_key: pth_key_t = -(1 as libc::c_int);
    if tid == __pth_current {
        *__errno_location() = 35 as libc::c_int;
        return 0 as libc::c_int;
    }
    if !tid.is_null() && (*tid).joinable == 0 {
        *__errno_location() = 22 as libc::c_int;
        return 0 as libc::c_int;
    }
    if pth_ctrl(
        ((1 as libc::c_int) << 4 as libc::c_int | (1 as libc::c_int) << 5 as libc::c_int
            | (1 as libc::c_int) << 6 as libc::c_int
            | (1 as libc::c_int) << 7 as libc::c_int
            | (1 as libc::c_int) << 8 as libc::c_int
            | (1 as libc::c_int) << 9 as libc::c_int) as libc::c_ulong,
    ) == 1 as libc::c_int as libc::c_long
    {
        *__errno_location() = 35 as libc::c_int;
        return 0 as libc::c_int;
    }
    if tid.is_null() {
        tid = if (&mut __pth_DQ as *mut pth_pqueue_t).is_null() {
            0 as pth_t
        } else {
            __pth_DQ.q_head
        };
    }
    if tid.is_null()
        || !tid.is_null()
            && (*tid).state as libc::c_uint
                != PTH_STATE_DEAD as libc::c_int as libc::c_uint
    {
        ev = pth_event(
            ((1 as libc::c_int) << 8 as libc::c_int
                | (1 as libc::c_int) << 18 as libc::c_int
                | (1 as libc::c_int) << 22 as libc::c_int) as libc::c_ulong,
            &mut ev_key as *mut pth_key_t,
            tid,
        );
        pth_wait(ev);
    }
    if tid.is_null() {
        tid = if (&mut __pth_DQ as *mut pth_pqueue_t).is_null() {
            0 as pth_t
        } else {
            __pth_DQ.q_head
        };
    }
    if tid.is_null()
        || !tid.is_null()
            && (*tid).state as libc::c_uint
                != PTH_STATE_DEAD as libc::c_int as libc::c_uint
    {
        *__errno_location() = 5 as libc::c_int;
        return 0 as libc::c_int;
    }
    if !value.is_null() {
        *value = (*tid).join_arg;
    }
    __pth_pqueue_delete(&mut __pth_DQ, tid);
    __pth_tcb_free(tid);
    return (0 as libc::c_int == 0) as libc::c_int;
}
pub unsafe extern "C" fn pth_yield(mut to: pth_t) -> libc::c_int {
    let mut q: *mut pth_pqueue_t = 0 as *mut pth_pqueue_t;
    if !to.is_null() {
        match (*to).state as libc::c_uint {
            1 => {
                q = &mut __pth_NQ;
            }
            2 => {
                q = &mut __pth_RQ;
            }
            _ => {
                q = 0 as *mut pth_pqueue_t;
            }
        }
        if q.is_null() || __pth_pqueue_contains(q, to) == 0 {
            *__errno_location() = 22 as libc::c_int;
            return 0 as libc::c_int;
        }
    }
    if !to.is_null() && !q.is_null() {
        __pth_pqueue_favorite(q, to);
    }
    !to.is_null();
    swapcontext(&mut (*__pth_current).mctx.uc, &mut (*__pth_sched).mctx.uc);
    return (0 as libc::c_int == 0) as libc::c_int;
}
pub unsafe extern "C" fn pth_suspend(mut t: pth_t) -> libc::c_int {
    let mut q: *mut pth_pqueue_t = 0 as *mut pth_pqueue_t;
    if t.is_null() {
        *__errno_location() = 22 as libc::c_int;
        return 0 as libc::c_int;
    }
    if t == __pth_sched || t == __pth_current {
        *__errno_location() = 1 as libc::c_int;
        return 0 as libc::c_int;
    }
    match (*t).state as libc::c_uint {
        1 => {
            q = &mut __pth_NQ;
        }
        2 => {
            q = &mut __pth_RQ;
        }
        3 => {
            q = &mut __pth_WQ;
        }
        _ => {
            q = 0 as *mut pth_pqueue_t;
        }
    }
    if q.is_null() {
        *__errno_location() = 1 as libc::c_int;
        return 0 as libc::c_int;
    }
    if __pth_pqueue_contains(q, t) == 0 {
        *__errno_location() = 3 as libc::c_int;
        return 0 as libc::c_int;
    }
    __pth_pqueue_delete(q, t);
    __pth_pqueue_insert(&mut __pth_SQ, 0 as libc::c_int, t);
    return (0 as libc::c_int == 0) as libc::c_int;
}
pub unsafe extern "C" fn pth_resume(mut t: pth_t) -> libc::c_int {
    let mut q: *mut pth_pqueue_t = 0 as *mut pth_pqueue_t;
    if t.is_null() {
        *__errno_location() = 22 as libc::c_int;
        return 0 as libc::c_int;
    }
    if t == __pth_sched || t == __pth_current {
        *__errno_location() = 1 as libc::c_int;
        return 0 as libc::c_int;
    }
    if __pth_pqueue_contains(&mut __pth_SQ, t) == 0 {
        *__errno_location() = 1 as libc::c_int;
        return 0 as libc::c_int;
    }
    __pth_pqueue_delete(&mut __pth_SQ, t);
    match (*t).state as libc::c_uint {
        1 => {
            q = &mut __pth_NQ;
        }
        2 => {
            q = &mut __pth_RQ;
        }
        3 => {
            q = &mut __pth_WQ;
        }
        _ => {
            q = 0 as *mut pth_pqueue_t;
        }
    }
    __pth_pqueue_insert(q, 0 as libc::c_int, t);
    return (0 as libc::c_int == 0) as libc::c_int;
}
pub unsafe extern "C" fn pth_fdmode(
    mut fd: libc::c_int,
    mut newmode: libc::c_int,
) -> libc::c_int {
    let mut fdmode: libc::c_int = 0;
    let mut oldmode: libc::c_int = 0;
    fdmode = fcntl(fd, 3 as libc::c_int, 0 as *mut libc::c_void);
    if fdmode == -(1 as libc::c_int) {
        oldmode = PTH_FDMODE_ERROR as libc::c_int;
    } else if fdmode & 0o4000 as libc::c_int != 0 {
        oldmode = PTH_FDMODE_NONBLOCK as libc::c_int;
    } else {
        oldmode = PTH_FDMODE_BLOCK as libc::c_int;
    }
    if oldmode == PTH_FDMODE_BLOCK as libc::c_int
        && newmode == PTH_FDMODE_NONBLOCK as libc::c_int
    {
        fcntl(fd, 4 as libc::c_int, fdmode | 0o4000 as libc::c_int);
    }
    if oldmode == PTH_FDMODE_NONBLOCK as libc::c_int
        && newmode == PTH_FDMODE_BLOCK as libc::c_int
    {
        fcntl(fd, 4 as libc::c_int, fdmode & !(0o4000 as libc::c_int));
    }
    return oldmode;
}
pub unsafe extern "C" fn pth_nap(mut naptime: pth_time_t) -> libc::c_int {
    let mut until: pth_time_t = pth_time_t {
        tv_sec: 0,
        tv_usec: 0,
    };
    let mut ev: pth_event_t = 0 as *mut pth_event_st;
    static mut ev_key: pth_key_t = -(1 as libc::c_int);
    if __pth_time_cmp(&mut naptime, &mut __pth_time_zero) == 0 as libc::c_int {
        *__errno_location() = 22 as libc::c_int;
        return 0 as libc::c_int;
    }
    if (0 as *mut pth_time_t).is_null() {
        gettimeofday(&mut until, 0 as *mut libc::c_void);
    } else {
        until.tv_sec = (*(0 as *mut pth_time_t)).tv_sec;
        until.tv_usec = (*(0 as *mut pth_time_t)).tv_usec;
    }
    until.tv_sec += naptime.tv_sec;
    until.tv_usec += naptime.tv_usec;
    if until.tv_usec > 1000000 as libc::c_int as libc::c_long {
        until.tv_sec += 1 as libc::c_int as libc::c_long;
        until.tv_usec -= 1000000 as libc::c_int as libc::c_long;
    }
    ev = pth_event(
        ((1 as libc::c_int) << 4 as libc::c_int
            | (1 as libc::c_int) << 22 as libc::c_int) as libc::c_ulong,
        &mut ev_key as *mut pth_key_t,
        until,
    );
    pth_wait(ev);
    return (0 as libc::c_int == 0) as libc::c_int;
}
pub unsafe extern "C" fn pth_once(
    mut oncectrl: *mut pth_once_t,
    mut constructor: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    mut arg: *mut libc::c_void,
) -> libc::c_int {
    if oncectrl.is_null() || constructor.is_none() {
        *__errno_location() = 22 as libc::c_int;
        return 0 as libc::c_int;
    }
    if *oncectrl != (0 as libc::c_int == 0) as libc::c_int {
        constructor.unwrap()(arg);
    }
    *oncectrl = (0 as libc::c_int == 0) as libc::c_int;
    return (0 as libc::c_int == 0) as libc::c_int;
}
