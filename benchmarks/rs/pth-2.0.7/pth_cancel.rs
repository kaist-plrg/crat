use ::libc;
extern "C" {
    fn __errno_location() -> *mut libc::c_int;
    static mut __pth_DQ: pth_pqueue_t;
    fn __pth_pqueue_insert(_: *mut pth_pqueue_t, _: libc::c_int, _: pth_t);
    fn __pth_tcb_free(_: pth_t);
    fn __pth_thread_cleanup(_: pth_t);
    fn __pth_pqueue_delete(_: *mut pth_pqueue_t, _: pth_t);
    fn __pth_pqueue_contains(_: *mut pth_pqueue_t, _: pth_t) -> libc::c_int;
    static mut __pth_WQ: pth_pqueue_t;
    static mut __pth_RQ: pth_pqueue_t;
    static mut __pth_NQ: pth_pqueue_t;
    static mut __pth_current: pth_t;
    fn pth_join(_: pth_t, _: *mut *mut libc::c_void) -> libc::c_int;
    fn pth_exit(_: *mut libc::c_void);
}
pub type size_t = libc::c_ulong;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
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
    pub ev_args: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub FD: C2RustUnnamed_8,
    pub SELECT: C2RustUnnamed_7,
    pub SIGS: C2RustUnnamed_6,
    pub TIME: C2RustUnnamed_5,
    pub MSG: C2RustUnnamed_4,
    pub MUTEX: C2RustUnnamed_3,
    pub COND: C2RustUnnamed_2,
    pub TID: C2RustUnnamed_1,
    pub FUNC: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub func: pth_event_func_t,
    pub arg: *mut libc::c_void,
    pub tv: pth_time_t,
}
pub type pth_event_func_t = Option::<
    unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub tid: pth_t,
}
pub type pth_t = *mut pth_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
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
pub struct C2RustUnnamed_3 {
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
pub struct C2RustUnnamed_4 {
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
pub struct C2RustUnnamed_5 {
    pub tv: pth_time_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub sigs: *mut sigset_t,
    pub sig: *mut libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub n: *mut libc::c_int,
    pub nfd: libc::c_int,
    pub rfds: *mut fd_set,
    pub wfds: *mut fd_set,
    pub efds: *mut fd_set,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
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
pub type pth_pqueue_t = pth_pqueue_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pth_pqueue_st {
    pub q_head: pth_t,
    pub q_num: libc::c_int,
}
pub unsafe extern "C" fn pth_cancel_state(
    mut newstate: libc::c_int,
    mut oldstate: *mut libc::c_int,
) {
    if !oldstate.is_null() {
        *oldstate = (*__pth_current).cancelstate as libc::c_int;
    }
    if newstate != 0 as libc::c_int {
        (*__pth_current).cancelstate = newstate as libc::c_uint;
    }
}
pub unsafe extern "C" fn pth_cancel_point() {
    if (*__pth_current).cancelreq == (0 as libc::c_int == 0) as libc::c_int
        && (*__pth_current).cancelstate
            & ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint != 0
    {
        (*__pth_current).cancelreq = 0 as libc::c_int;
        pth_exit(-(1 as libc::c_int) as *mut libc::c_void);
    }
}
pub unsafe extern "C" fn pth_cancel(mut thread: pth_t) -> libc::c_int {
    let mut q: *mut pth_pqueue_t = 0 as *mut pth_pqueue_t;
    if thread.is_null() {
        *__errno_location() = 22 as libc::c_int;
        return 0 as libc::c_int;
    }
    if thread == __pth_current {
        *__errno_location() = 22 as libc::c_int;
        return 0 as libc::c_int;
    }
    if (*thread).state as libc::c_uint == PTH_STATE_DEAD as libc::c_int as libc::c_uint {
        *__errno_location() = 1 as libc::c_int;
        return 0 as libc::c_int;
    }
    (*thread).cancelreq = (0 as libc::c_int == 0) as libc::c_int;
    if (*thread).cancelstate & ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint
        != 0
        && (*thread).cancelstate
            & ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uint != 0
    {
        match (*thread).state as libc::c_uint {
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
            *__errno_location() = 3 as libc::c_int;
            return 0 as libc::c_int;
        }
        if __pth_pqueue_contains(q, thread) == 0 {
            *__errno_location() = 3 as libc::c_int;
            return 0 as libc::c_int;
        }
        __pth_pqueue_delete(q, thread);
        __pth_thread_cleanup(thread);
        if (*thread).joinable == 0 {
            __pth_tcb_free(thread);
        } else {
            (*thread).join_arg = -(1 as libc::c_int) as *mut libc::c_void;
            (*thread).state = PTH_STATE_DEAD;
            __pth_pqueue_insert(&mut __pth_DQ, 0 as libc::c_int, thread);
        }
    }
    return (0 as libc::c_int == 0) as libc::c_int;
}
pub unsafe extern "C" fn pth_abort(mut thread: pth_t) -> libc::c_int {
    if thread.is_null() {
        *__errno_location() = 22 as libc::c_int;
        return 0 as libc::c_int;
    }
    if thread == __pth_current {
        *__errno_location() = 22 as libc::c_int;
        return 0 as libc::c_int;
    }
    if (*thread).state as libc::c_uint == PTH_STATE_DEAD as libc::c_int as libc::c_uint
        && (*thread).joinable != 0
    {
        if pth_join(thread, 0 as *mut *mut libc::c_void) == 0 {
            return 0 as libc::c_int;
        }
    } else {
        (*thread).joinable = 0 as libc::c_int;
        (*thread)
            .cancelstate = ((1 as libc::c_int) << 0 as libc::c_int
            | (1 as libc::c_int) << 2 as libc::c_int) as libc::c_uint;
        if pth_cancel(thread) == 0 {
            return 0 as libc::c_int;
        }
    }
    return (0 as libc::c_int == 0) as libc::c_int;
}
