use ::libc;
extern "C" {
    fn __errno_location() -> *mut libc::c_int;
    fn pth_yield(_: pth_t) -> libc::c_int;
    fn pth_wait(_: pth_event_t) -> libc::c_int;
    fn pth_cancel_state(_: libc::c_int, _: *mut libc::c_int);
    fn pth_event(_: libc::c_ulong, _: ...) -> pth_event_t;
    fn pth_event_concat(_: pth_event_t, _: ...) -> pth_event_t;
    fn pth_event_isolate(_: pth_event_t) -> pth_event_t;
    fn pth_event_status(_: pth_event_t) -> pth_status_t;
    fn pth_cleanup_push(
        _: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
        _: *mut libc::c_void,
    ) -> libc::c_int;
    fn pth_cleanup_pop(_: libc::c_int) -> libc::c_int;
    static mut __pth_current: pth_t;
    fn __pth_ring_append(_: *mut pth_ring_t, _: *mut pth_ringnode_t);
    fn __pth_ring_delete(_: *mut pth_ring_t, _: *mut pth_ringnode_t);
    static mut __pth_errno_flag: libc::c_int;
    static mut __pth_errno_storage: libc::c_int;
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
pub type pth_key_t = libc::c_int;
pub type C2RustUnnamed_9 = libc::c_uint;
pub const PTH_RWLOCK_RW: C2RustUnnamed_9 = 1;
pub const PTH_RWLOCK_RD: C2RustUnnamed_9 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pth_rwlock_st {
    pub rw_state: libc::c_int,
    pub rw_mode: libc::c_uint,
    pub rw_readers: libc::c_ulong,
    pub rw_mutex_rd: pth_mutex_t,
    pub rw_mutex_rw: pth_mutex_t,
}
pub type pth_rwlock_t = pth_rwlock_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pth_barrier_st {
    pub br_state: libc::c_ulong,
    pub br_threshold: libc::c_int,
    pub br_count: libc::c_int,
    pub br_cycle: libc::c_int,
    pub br_cond: pth_cond_t,
    pub br_mutex: pth_mutex_t,
}
pub type pth_barrier_t = pth_barrier_st;
pub unsafe extern "C" fn pth_mutex_init(mut mutex: *mut pth_mutex_t) -> libc::c_int {
    if mutex.is_null() {
        *__errno_location() = 22 as libc::c_int;
        return 0 as libc::c_int;
    }
    (*mutex).mx_state = (1 as libc::c_int) << 0 as libc::c_int;
    (*mutex).mx_owner = 0 as pth_t;
    (*mutex).mx_count = 0 as libc::c_int as libc::c_ulong;
    return (0 as libc::c_int == 0) as libc::c_int;
}
pub unsafe extern "C" fn pth_mutex_acquire(
    mut mutex: *mut pth_mutex_t,
    mut tryonly: libc::c_int,
    mut ev_extra: pth_event_t,
) -> libc::c_int {
    static mut ev_key: pth_key_t = -(1 as libc::c_int);
    let mut ev: pth_event_t = 0 as *mut pth_event_st;
    if mutex.is_null() {
        *__errno_location() = 22 as libc::c_int;
        return 0 as libc::c_int;
    }
    if (*mutex).mx_state & (1 as libc::c_int) << 0 as libc::c_int == 0 {
        *__errno_location() = 35 as libc::c_int;
        return 0 as libc::c_int;
    }
    if (*mutex).mx_state & (1 as libc::c_int) << 1 as libc::c_int == 0 {
        (*mutex).mx_state |= (1 as libc::c_int) << 1 as libc::c_int;
        (*mutex).mx_owner = __pth_current;
        (*mutex).mx_count = 1 as libc::c_int as libc::c_ulong;
        __pth_ring_append(&mut (*__pth_current).mutexring, &mut (*mutex).mx_node);
        return (0 as libc::c_int == 0) as libc::c_int;
    }
    if (*mutex).mx_count >= 1 as libc::c_int as libc::c_ulong
        && (*mutex).mx_owner == __pth_current
    {
        (*mutex).mx_count = ((*mutex).mx_count).wrapping_add(1);
        (*mutex).mx_count;
        return (0 as libc::c_int == 0) as libc::c_int;
    }
    if tryonly != 0 {
        *__errno_location() = 16 as libc::c_int;
        return 0 as libc::c_int;
    }
    loop {
        ev = pth_event(
            ((1 as libc::c_int) << 6 as libc::c_int
                | (1 as libc::c_int) << 22 as libc::c_int) as libc::c_ulong,
            &mut ev_key as *mut pth_key_t,
            mutex,
        );
        if !ev_extra.is_null() {
            pth_event_concat(ev, ev_extra, 0 as *mut libc::c_void);
        }
        pth_wait(ev);
        if !ev_extra.is_null() {
            pth_event_isolate(ev);
            if pth_event_status(ev) as libc::c_uint
                == PTH_STATUS_PENDING as libc::c_int as libc::c_uint
            {
                *__errno_location() = 4 as libc::c_int;
                return 0 as libc::c_int;
            }
        }
        if (*mutex).mx_state & (1 as libc::c_int) << 1 as libc::c_int == 0 {
            break;
        }
    }
    (*mutex).mx_state |= (1 as libc::c_int) << 1 as libc::c_int;
    (*mutex).mx_owner = __pth_current;
    (*mutex).mx_count = 1 as libc::c_int as libc::c_ulong;
    __pth_ring_append(&mut (*__pth_current).mutexring, &mut (*mutex).mx_node);
    return (0 as libc::c_int == 0) as libc::c_int;
}
pub unsafe extern "C" fn pth_mutex_release(mut mutex: *mut pth_mutex_t) -> libc::c_int {
    if mutex.is_null() {
        *__errno_location() = 22 as libc::c_int;
        return 0 as libc::c_int;
    }
    if (*mutex).mx_state & (1 as libc::c_int) << 0 as libc::c_int == 0 {
        *__errno_location() = 35 as libc::c_int;
        return 0 as libc::c_int;
    }
    if (*mutex).mx_state & (1 as libc::c_int) << 1 as libc::c_int == 0 {
        *__errno_location() = 35 as libc::c_int;
        return 0 as libc::c_int;
    }
    if (*mutex).mx_owner != __pth_current {
        *__errno_location() = 13 as libc::c_int;
        return 0 as libc::c_int;
    }
    (*mutex).mx_count = ((*mutex).mx_count).wrapping_sub(1);
    (*mutex).mx_count;
    if (*mutex).mx_count <= 0 as libc::c_int as libc::c_ulong {
        (*mutex).mx_state &= !((1 as libc::c_int) << 1 as libc::c_int);
        (*mutex).mx_owner = 0 as pth_t;
        (*mutex).mx_count = 0 as libc::c_int as libc::c_ulong;
        __pth_ring_delete(&mut (*__pth_current).mutexring, &mut (*mutex).mx_node);
    }
    return (0 as libc::c_int == 0) as libc::c_int;
}
pub unsafe extern "C" fn __pth_mutex_releaseall(mut thread: pth_t) {
    let mut rn: *mut pth_ringnode_t = 0 as *mut pth_ringnode_t;
    let mut rnf: *mut pth_ringnode_t = 0 as *mut pth_ringnode_t;
    if thread.is_null() {
        return;
    }
    rnf = if (&mut (*thread).mutexring as *mut pth_ring_t).is_null() {
        0 as *mut pth_ringnode_t
    } else {
        (*thread).mutexring.r_hook
    };
    rn = rnf;
    while !rn.is_null() {
        pth_mutex_release(rn as *mut pth_mutex_t);
        rn = if (&mut (*thread).mutexring as *mut pth_ring_t).is_null() || rn.is_null() {
            0 as *mut pth_ringnode_t
        } else if (*rn).rn_next == (*thread).mutexring.r_hook {
            0 as *mut pth_ringnode_t
        } else {
            (*rn).rn_next
        };
        if rn == rnf {
            break;
        }
    }
}
pub unsafe extern "C" fn pth_rwlock_init(mut rwlock: *mut pth_rwlock_t) -> libc::c_int {
    if rwlock.is_null() {
        *__errno_location() = 22 as libc::c_int;
        return 0 as libc::c_int;
    }
    (*rwlock).rw_state = (1 as libc::c_int) << 0 as libc::c_int;
    (*rwlock).rw_readers = 0 as libc::c_int as libc::c_ulong;
    pth_mutex_init(&mut (*rwlock).rw_mutex_rd);
    pth_mutex_init(&mut (*rwlock).rw_mutex_rw);
    return (0 as libc::c_int == 0) as libc::c_int;
}
pub unsafe extern "C" fn pth_rwlock_acquire(
    mut rwlock: *mut pth_rwlock_t,
    mut op: libc::c_int,
    mut tryonly: libc::c_int,
    mut ev_extra: pth_event_t,
) -> libc::c_int {
    if rwlock.is_null() {
        *__errno_location() = 22 as libc::c_int;
        return 0 as libc::c_int;
    }
    if (*rwlock).rw_state & (1 as libc::c_int) << 0 as libc::c_int == 0 {
        *__errno_location() = 35 as libc::c_int;
        return 0 as libc::c_int;
    }
    if op == PTH_RWLOCK_RW as libc::c_int {
        if pth_mutex_acquire(&mut (*rwlock).rw_mutex_rw, tryonly, ev_extra) == 0 {
            return 0 as libc::c_int;
        }
        (*rwlock).rw_mode = PTH_RWLOCK_RW as libc::c_int as libc::c_uint;
    } else {
        if pth_mutex_acquire(&mut (*rwlock).rw_mutex_rd, tryonly, ev_extra) == 0 {
            return 0 as libc::c_int;
        }
        (*rwlock).rw_readers = ((*rwlock).rw_readers).wrapping_add(1);
        (*rwlock).rw_readers;
        if (*rwlock).rw_readers == 1 as libc::c_int as libc::c_ulong {
            if pth_mutex_acquire(&mut (*rwlock).rw_mutex_rw, tryonly, ev_extra) == 0 {
                (*rwlock).rw_readers = ((*rwlock).rw_readers).wrapping_sub(1);
                (*rwlock).rw_readers;
                __pth_errno_storage = *__errno_location();
                __pth_errno_flag = (0 as libc::c_int == 0) as libc::c_int;
                while __pth_errno_flag != 0 {
                    pth_mutex_release(&mut (*rwlock).rw_mutex_rd);
                    *__errno_location() = __pth_errno_storage;
                    __pth_errno_flag = 0 as libc::c_int;
                }
                return 0 as libc::c_int;
            }
        }
        (*rwlock).rw_mode = PTH_RWLOCK_RD as libc::c_int as libc::c_uint;
        pth_mutex_release(&mut (*rwlock).rw_mutex_rd);
    }
    return (0 as libc::c_int == 0) as libc::c_int;
}
pub unsafe extern "C" fn pth_rwlock_release(
    mut rwlock: *mut pth_rwlock_t,
) -> libc::c_int {
    if rwlock.is_null() {
        *__errno_location() = 22 as libc::c_int;
        return 0 as libc::c_int;
    }
    if (*rwlock).rw_state & (1 as libc::c_int) << 0 as libc::c_int == 0 {
        *__errno_location() = 35 as libc::c_int;
        return 0 as libc::c_int;
    }
    if (*rwlock).rw_mode == PTH_RWLOCK_RW as libc::c_int as libc::c_uint {
        if pth_mutex_release(&mut (*rwlock).rw_mutex_rw) == 0 {
            return 0 as libc::c_int;
        }
    } else {
        if pth_mutex_acquire(
            &mut (*rwlock).rw_mutex_rd,
            0 as libc::c_int,
            0 as pth_event_t,
        ) == 0
        {
            return 0 as libc::c_int;
        }
        (*rwlock).rw_readers = ((*rwlock).rw_readers).wrapping_sub(1);
        (*rwlock).rw_readers;
        if (*rwlock).rw_readers == 0 as libc::c_int as libc::c_ulong {
            if pth_mutex_release(&mut (*rwlock).rw_mutex_rw) == 0 {
                (*rwlock).rw_readers = ((*rwlock).rw_readers).wrapping_add(1);
                (*rwlock).rw_readers;
                __pth_errno_storage = *__errno_location();
                __pth_errno_flag = (0 as libc::c_int == 0) as libc::c_int;
                while __pth_errno_flag != 0 {
                    pth_mutex_release(&mut (*rwlock).rw_mutex_rd);
                    *__errno_location() = __pth_errno_storage;
                    __pth_errno_flag = 0 as libc::c_int;
                }
                return 0 as libc::c_int;
            }
        }
        (*rwlock).rw_mode = PTH_RWLOCK_RD as libc::c_int as libc::c_uint;
        pth_mutex_release(&mut (*rwlock).rw_mutex_rd);
    }
    return (0 as libc::c_int == 0) as libc::c_int;
}
pub unsafe extern "C" fn pth_cond_init(mut cond: *mut pth_cond_t) -> libc::c_int {
    if cond.is_null() {
        *__errno_location() = 22 as libc::c_int;
        return 0 as libc::c_int;
    }
    (*cond).cn_state = ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_ulong;
    (*cond).cn_waiters = 0 as libc::c_int as libc::c_uint;
    return (0 as libc::c_int == 0) as libc::c_int;
}
unsafe extern "C" fn pth_cond_cleanup_handler(mut _cleanvec: *mut libc::c_void) {
    let mut mutex: *mut pth_mutex_t = *(_cleanvec as *mut *mut libc::c_void)
        .offset(0 as libc::c_int as isize) as *mut pth_mutex_t;
    let mut cond: *mut pth_cond_t = *(_cleanvec as *mut *mut libc::c_void)
        .offset(1 as libc::c_int as isize) as *mut pth_cond_t;
    pth_mutex_acquire(mutex, 0 as libc::c_int, 0 as pth_event_t);
    (*cond).cn_waiters = ((*cond).cn_waiters).wrapping_sub(1);
    (*cond).cn_waiters;
}
pub unsafe extern "C" fn pth_cond_await(
    mut cond: *mut pth_cond_t,
    mut mutex: *mut pth_mutex_t,
    mut ev_extra: pth_event_t,
) -> libc::c_int {
    static mut ev_key: pth_key_t = -(1 as libc::c_int);
    let mut cleanvec: [*mut libc::c_void; 2] = [0 as *mut libc::c_void; 2];
    let mut ev: pth_event_t = 0 as *mut pth_event_st;
    if cond.is_null() || mutex.is_null() {
        *__errno_location() = 22 as libc::c_int;
        return 0 as libc::c_int;
    }
    if (*cond).cn_state & ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_ulong == 0
    {
        *__errno_location() = 35 as libc::c_int;
        return 0 as libc::c_int;
    }
    if (*cond).cn_state & ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_ulong != 0
        && (*cond).cn_state & ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_ulong
            == 0
    {
        (*cond).cn_state &= !((1 as libc::c_int) << 1 as libc::c_int) as libc::c_ulong;
        (*cond).cn_state &= !((1 as libc::c_int) << 2 as libc::c_int) as libc::c_ulong;
        (*cond).cn_state &= !((1 as libc::c_int) << 3 as libc::c_int) as libc::c_ulong;
        return (0 as libc::c_int == 0) as libc::c_int;
    }
    (*cond).cn_waiters = ((*cond).cn_waiters).wrapping_add(1);
    (*cond).cn_waiters;
    pth_mutex_release(mutex);
    ev = pth_event(
        ((1 as libc::c_int) << 7 as libc::c_int
            | (1 as libc::c_int) << 22 as libc::c_int) as libc::c_ulong,
        &mut ev_key as *mut pth_key_t,
        cond,
    );
    if !ev_extra.is_null() {
        pth_event_concat(ev, ev_extra, 0 as *mut libc::c_void);
    }
    cleanvec[0 as libc::c_int as usize] = mutex as *mut libc::c_void;
    cleanvec[1 as libc::c_int as usize] = cond as *mut libc::c_void;
    pth_cleanup_push(
        Some(pth_cond_cleanup_handler as unsafe extern "C" fn(*mut libc::c_void) -> ()),
        cleanvec.as_mut_ptr() as *mut libc::c_void,
    );
    pth_wait(ev);
    pth_cleanup_pop(0 as libc::c_int);
    if !ev_extra.is_null() {
        pth_event_isolate(ev);
    }
    pth_mutex_acquire(mutex, 0 as libc::c_int, 0 as pth_event_t);
    (*cond).cn_waiters = ((*cond).cn_waiters).wrapping_sub(1);
    (*cond).cn_waiters;
    return (0 as libc::c_int == 0) as libc::c_int;
}
pub unsafe extern "C" fn pth_cond_notify(
    mut cond: *mut pth_cond_t,
    mut broadcast: libc::c_int,
) -> libc::c_int {
    if cond.is_null() {
        *__errno_location() = 22 as libc::c_int;
        return 0 as libc::c_int;
    }
    if (*cond).cn_state & ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_ulong == 0
    {
        *__errno_location() = 35 as libc::c_int;
        return 0 as libc::c_int;
    }
    if (*cond).cn_waiters > 0 as libc::c_int as libc::c_uint {
        (*cond).cn_state |= ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_ulong;
        if broadcast != 0 {
            (*cond).cn_state
                |= ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_ulong;
        } else {
            (*cond).cn_state
                &= !((1 as libc::c_int) << 2 as libc::c_int) as libc::c_ulong;
        }
        (*cond).cn_state &= !((1 as libc::c_int) << 3 as libc::c_int) as libc::c_ulong;
        pth_yield(0 as pth_t);
    }
    return (0 as libc::c_int == 0) as libc::c_int;
}
pub unsafe extern "C" fn pth_barrier_init(
    mut barrier: *mut pth_barrier_t,
    mut threshold: libc::c_int,
) -> libc::c_int {
    if barrier.is_null() || threshold <= 0 as libc::c_int {
        *__errno_location() = 22 as libc::c_int;
        return 0 as libc::c_int;
    }
    if pth_mutex_init(&mut (*barrier).br_mutex) == 0 {
        return 0 as libc::c_int;
    }
    if pth_cond_init(&mut (*barrier).br_cond) == 0 {
        return 0 as libc::c_int;
    }
    (*barrier).br_state = ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_ulong;
    (*barrier).br_threshold = threshold;
    (*barrier).br_count = threshold;
    (*barrier).br_cycle = 0 as libc::c_int;
    return (0 as libc::c_int == 0) as libc::c_int;
}
pub unsafe extern "C" fn pth_barrier_reach(
    mut barrier: *mut pth_barrier_t,
) -> libc::c_int {
    let mut cancel: libc::c_int = 0;
    let mut cycle: libc::c_int = 0;
    let mut rv: libc::c_int = 0;
    if barrier.is_null() {
        *__errno_location() = 22 as libc::c_int;
        return 0 as libc::c_int;
    }
    if (*barrier).br_state & ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_ulong
        == 0
    {
        *__errno_location() = 22 as libc::c_int;
        return 0 as libc::c_int;
    }
    if pth_mutex_acquire(&mut (*barrier).br_mutex, 0 as libc::c_int, 0 as pth_event_t)
        == 0
    {
        return 0 as libc::c_int;
    }
    cycle = (*barrier).br_cycle;
    (*barrier).br_count -= 1;
    if (*barrier).br_count == 0 as libc::c_int {
        (*barrier).br_cycle = ((*barrier).br_cycle == 0) as libc::c_int;
        (*barrier).br_count = (*barrier).br_threshold;
        rv = pth_cond_notify(
            &mut (*barrier).br_cond,
            (0 as libc::c_int == 0) as libc::c_int,
        );
        if rv != 0 {
            rv = -(2 as libc::c_int);
        }
    } else {
        pth_cancel_state((1 as libc::c_int) << 1 as libc::c_int, &mut cancel);
        if (*barrier).br_threshold == (*barrier).br_count {
            rv = -(1 as libc::c_int);
        } else {
            rv = (0 as libc::c_int == 0) as libc::c_int;
        }
        while cycle == (*barrier).br_cycle {
            rv = pth_cond_await(
                &mut (*barrier).br_cond,
                &mut (*barrier).br_mutex,
                0 as pth_event_t,
            );
            if rv == 0 {
                break;
            }
        }
        pth_cancel_state(cancel, 0 as *mut libc::c_int);
    }
    pth_mutex_release(&mut (*barrier).br_mutex);
    return rv;
}
