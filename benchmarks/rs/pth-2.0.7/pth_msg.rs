use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn __pth_ring_append(_: *mut pth_ring_t, _: *mut pth_ringnode_t);
    fn __pth_ring_init(_: *mut pth_ring_t);
    static mut __pth_current: pth_t;
    fn __pth_ring_delete(_: *mut pth_ring_t, _: *mut pth_ringnode_t);
    fn __pth_ring_pop(_: *mut pth_ring_t) -> *mut pth_ringnode_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pth_message_st {
    pub m_node: pth_ringnode_t,
    pub m_replyport: pth_msgport_t,
    pub m_size: libc::c_uint,
    pub m_data: *mut libc::c_void,
}
pub type pth_message_t = pth_message_st;
static mut pth_msgport: pth_ring_t = {
    let mut init = pth_ring_st {
        r_hook: 0 as *const pth_ringnode_t as *mut pth_ringnode_t,
        r_nodes: 0,
    };
    init
};
pub unsafe extern "C" fn pth_msgport_create(
    mut name: *const libc::c_char,
) -> pth_msgport_t {
    let mut mp: pth_msgport_t = 0 as *mut pth_msgport_st;
    mp = malloc(::std::mem::size_of::<pth_msgport_st>() as libc::c_ulong)
        as pth_msgport_t;
    if mp.is_null() {
        *__errno_location() = 12 as libc::c_int;
        return 0 as *mut libc::c_void as pth_msgport_t;
    }
    (*mp).mp_name = name;
    (*mp).mp_tid = __pth_current;
    __pth_ring_init(&mut (*mp).mp_queue);
    __pth_ring_append(&mut pth_msgport, &mut (*mp).mp_node);
    return mp;
}
pub unsafe extern "C" fn pth_msgport_destroy(mut mp: pth_msgport_t) {
    let mut m: *mut pth_message_t = 0 as *mut pth_message_t;
    if mp.is_null() {
        return;
    }
    loop {
        m = pth_msgport_get(mp);
        if m.is_null() {
            break;
        }
        pth_msgport_reply(m);
    }
    __pth_ring_delete(&mut pth_msgport, &mut (*mp).mp_node);
    free(mp as *mut libc::c_void);
}
pub unsafe extern "C" fn pth_msgport_find(
    mut name: *const libc::c_char,
) -> pth_msgport_t {
    let mut mp: pth_msgport_t = 0 as *mut pth_msgport_st;
    let mut mpf: pth_msgport_t = 0 as *mut pth_msgport_st;
    if name.is_null() {
        *__errno_location() = 22 as libc::c_int;
        return 0 as *mut libc::c_void as pth_msgport_t;
    }
    mpf = (if (&mut pth_msgport as *mut pth_ring_t).is_null() {
        0 as *mut pth_ringnode_t
    } else {
        pth_msgport.r_hook
    }) as pth_msgport_t;
    mp = mpf;
    while !mp.is_null() {
        if !((*mp).mp_name).is_null() {
            if strcmp((*mp).mp_name, name) == 0 as libc::c_int {
                break;
            }
        }
        mp = (if (&mut pth_msgport as *mut pth_ring_t).is_null()
            || (mp as *mut pth_ringnode_t).is_null()
        {
            0 as *mut pth_ringnode_t
        } else if (*(mp as *mut pth_ringnode_t)).rn_next == pth_msgport.r_hook {
            0 as *mut pth_ringnode_t
        } else {
            (*(mp as *mut pth_ringnode_t)).rn_next
        }) as pth_msgport_t;
        if !(mp == mpf) {
            continue;
        }
        mp = 0 as pth_msgport_t;
        break;
    }
    return mp;
}
pub unsafe extern "C" fn pth_msgport_pending(mut mp: pth_msgport_t) -> libc::c_int {
    if mp.is_null() {
        *__errno_location() = 22 as libc::c_int;
        return -(1 as libc::c_int);
    }
    return (if (&mut (*mp).mp_queue as *mut pth_ring_t).is_null() {
        -(1 as libc::c_int) as libc::c_uint
    } else {
        (*mp).mp_queue.r_nodes
    }) as libc::c_int;
}
pub unsafe extern "C" fn pth_msgport_put(
    mut mp: pth_msgport_t,
    mut m: *mut pth_message_t,
) -> libc::c_int {
    if mp.is_null() {
        *__errno_location() = 22 as libc::c_int;
        return 0 as libc::c_int;
    }
    __pth_ring_append(&mut (*mp).mp_queue, m as *mut pth_ringnode_t);
    return (0 as libc::c_int == 0) as libc::c_int;
}
pub unsafe extern "C" fn pth_msgport_get(mut mp: pth_msgport_t) -> *mut pth_message_t {
    let mut m: *mut pth_message_t = 0 as *mut pth_message_t;
    if mp.is_null() {
        *__errno_location() = 22 as libc::c_int;
        return 0 as *mut libc::c_void as *mut pth_message_t;
    }
    m = __pth_ring_pop(&mut (*mp).mp_queue) as *mut pth_message_t;
    return m;
}
pub unsafe extern "C" fn pth_msgport_reply(mut m: *mut pth_message_t) -> libc::c_int {
    if m.is_null() {
        *__errno_location() = 22 as libc::c_int;
        return 0 as libc::c_int;
    }
    return pth_msgport_put((*m).m_replyport, m);
}
