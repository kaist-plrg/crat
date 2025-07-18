use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn __errno_location() -> *mut libc::c_int;
    fn gettimeofday(__tv: *mut timeval, __tz: *mut libc::c_void) -> libc::c_int;
    fn __pth_util_cpystrn(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: size_t,
    ) -> *mut libc::c_char;
    static mut __pth_time_zero: pth_time_t;
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
pub type C2RustUnnamed_9 = libc::c_uint;
pub const PTH_ATTR_BOUND: C2RustUnnamed_9 = 14;
pub const PTH_ATTR_EVENTS: C2RustUnnamed_9 = 13;
pub const PTH_ATTR_STATE: C2RustUnnamed_9 = 12;
pub const PTH_ATTR_START_ARG: C2RustUnnamed_9 = 11;
pub const PTH_ATTR_START_FUNC: C2RustUnnamed_9 = 10;
pub const PTH_ATTR_TIME_RAN: C2RustUnnamed_9 = 9;
pub const PTH_ATTR_TIME_LAST: C2RustUnnamed_9 = 8;
pub const PTH_ATTR_TIME_SPAWN: C2RustUnnamed_9 = 7;
pub const PTH_ATTR_DISPATCHES: C2RustUnnamed_9 = 6;
pub const PTH_ATTR_STACK_ADDR: C2RustUnnamed_9 = 5;
pub const PTH_ATTR_STACK_SIZE: C2RustUnnamed_9 = 4;
pub const PTH_ATTR_CANCEL_STATE: C2RustUnnamed_9 = 3;
pub const PTH_ATTR_JOINABLE: C2RustUnnamed_9 = 2;
pub const PTH_ATTR_NAME: C2RustUnnamed_9 = 1;
pub const PTH_ATTR_PRIO: C2RustUnnamed_9 = 0;
pub const PTH_ATTR_SET: C2RustUnnamed_10 = 1;
pub const PTH_ATTR_GET: C2RustUnnamed_10 = 0;
pub type C2RustUnnamed_10 = libc::c_uint;
pub unsafe extern "C" fn pth_attr_of(mut t: pth_t) -> pth_attr_t {
    let mut a: pth_attr_t = 0 as *mut pth_attr_st;
    if t.is_null() {
        *__errno_location() = 22 as libc::c_int;
        return 0 as *mut libc::c_void as pth_attr_t;
    }
    a = malloc(::std::mem::size_of::<pth_attr_st>() as libc::c_ulong) as pth_attr_t;
    if a.is_null() {
        *__errno_location() = 12 as libc::c_int;
        return 0 as *mut libc::c_void as pth_attr_t;
    }
    (*a).a_tid = t;
    return a;
}
pub unsafe extern "C" fn pth_attr_new() -> pth_attr_t {
    let mut a: pth_attr_t = 0 as *mut pth_attr_st;
    a = malloc(::std::mem::size_of::<pth_attr_st>() as libc::c_ulong) as pth_attr_t;
    if a.is_null() {
        *__errno_location() = 12 as libc::c_int;
        return 0 as *mut libc::c_void as pth_attr_t;
    }
    (*a).a_tid = 0 as pth_t;
    pth_attr_init(a);
    return a;
}
pub unsafe extern "C" fn pth_attr_destroy(mut a: pth_attr_t) -> libc::c_int {
    if a.is_null() {
        *__errno_location() = 22 as libc::c_int;
        return 0 as libc::c_int;
    }
    free(a as *mut libc::c_void);
    return (0 as libc::c_int == 0) as libc::c_int;
}
pub unsafe extern "C" fn pth_attr_init(mut a: pth_attr_t) -> libc::c_int {
    if a.is_null() {
        *__errno_location() = 22 as libc::c_int;
        return 0 as libc::c_int;
    }
    if !((*a).a_tid).is_null() {
        *__errno_location() = 1 as libc::c_int;
        return 0 as libc::c_int;
    }
    (*a).a_prio = 0 as libc::c_int;
    __pth_util_cpystrn(
        ((*a).a_name).as_mut_ptr(),
        b"unknown\0" as *const u8 as *const libc::c_char,
        40 as libc::c_int as size_t,
    );
    (*a).a_dispatches = 0 as libc::c_int;
    (*a).a_joinable = (0 as libc::c_int == 0) as libc::c_int;
    (*a)
        .a_cancelstate = ((1 as libc::c_int) << 0 as libc::c_int
        | (1 as libc::c_int) << 3 as libc::c_int) as libc::c_uint;
    (*a).a_stacksize = (64 as libc::c_int * 1024 as libc::c_int) as libc::c_uint;
    (*a).a_stackaddr = 0 as *mut libc::c_char;
    return (0 as libc::c_int == 0) as libc::c_int;
}
pub unsafe extern "C" fn pth_attr_get(
    mut a: pth_attr_t,
    mut op: libc::c_int,
    mut args: ...
) -> libc::c_int {
    let mut ap: ::std::ffi::VaListImpl;
    let mut rc: libc::c_int = 0;
    ap = args.clone();
    rc = __pth_attr_ctrl(PTH_ATTR_GET as libc::c_int, a, op, ap.as_va_list());
    return rc;
}
pub unsafe extern "C" fn pth_attr_set(
    mut a: pth_attr_t,
    mut op: libc::c_int,
    mut args: ...
) -> libc::c_int {
    let mut ap: ::std::ffi::VaListImpl;
    let mut rc: libc::c_int = 0;
    ap = args.clone();
    rc = __pth_attr_ctrl(PTH_ATTR_SET as libc::c_int, a, op, ap.as_va_list());
    return rc;
}
pub unsafe extern "C" fn __pth_attr_ctrl(
    mut cmd: libc::c_int,
    mut a: pth_attr_t,
    mut op: libc::c_int,
    mut ap: ::std::ffi::VaList,
) -> libc::c_int {
    if a.is_null() {
        *__errno_location() = 22 as libc::c_int;
        return 0 as libc::c_int;
    }
    match op {
        0 => {
            let mut val: libc::c_int = 0;
            let mut src: *mut libc::c_int = 0 as *mut libc::c_int;
            let mut dst: *mut libc::c_int = 0 as *mut libc::c_int;
            if cmd == PTH_ATTR_SET as libc::c_int {
                src = &mut val;
                val = ap.arg::<libc::c_int>();
                dst = if !((*a).a_tid).is_null() {
                    &mut (*(*a).a_tid).prio
                } else {
                    &mut (*a).a_prio
                };
            } else {
                src = if !((*a).a_tid).is_null() {
                    &mut (*(*a).a_tid).prio
                } else {
                    &mut (*a).a_prio
                };
                dst = ap.arg::<*mut libc::c_int>();
            }
            *dst = *src;
        }
        1 => {
            if cmd == PTH_ATTR_SET as libc::c_int {
                let mut src_0: *mut libc::c_char = 0 as *mut libc::c_char;
                let mut dst_0: *mut libc::c_char = 0 as *mut libc::c_char;
                src_0 = ap.arg::<*mut libc::c_char>();
                dst_0 = if !((*a).a_tid).is_null() {
                    ((*(*a).a_tid).name).as_mut_ptr()
                } else {
                    ((*a).a_name).as_mut_ptr()
                };
                __pth_util_cpystrn(dst_0, src_0, 40 as libc::c_int as size_t);
            } else {
                let mut src_1: *mut libc::c_char = 0 as *mut libc::c_char;
                let mut dst_1: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
                src_1 = if !((*a).a_tid).is_null() {
                    ((*(*a).a_tid).name).as_mut_ptr()
                } else {
                    ((*a).a_name).as_mut_ptr()
                };
                dst_1 = ap.arg::<*mut *mut libc::c_char>();
                *dst_1 = src_1;
            }
        }
        6 => {
            let mut val_0: libc::c_int = 0;
            let mut src_2: *mut libc::c_int = 0 as *mut libc::c_int;
            let mut dst_2: *mut libc::c_int = 0 as *mut libc::c_int;
            if cmd == PTH_ATTR_SET as libc::c_int {
                src_2 = &mut val_0;
                val_0 = ap.arg::<libc::c_int>();
                dst_2 = if !((*a).a_tid).is_null() {
                    &mut (*(*a).a_tid).dispatches
                } else {
                    &mut (*a).a_dispatches
                };
            } else {
                src_2 = if !((*a).a_tid).is_null() {
                    &mut (*(*a).a_tid).dispatches
                } else {
                    &mut (*a).a_dispatches
                };
                dst_2 = ap.arg::<*mut libc::c_int>();
            }
            *dst_2 = *src_2;
        }
        2 => {
            let mut val_1: libc::c_int = 0;
            let mut src_3: *mut libc::c_int = 0 as *mut libc::c_int;
            let mut dst_3: *mut libc::c_int = 0 as *mut libc::c_int;
            if cmd == PTH_ATTR_SET as libc::c_int {
                src_3 = &mut val_1;
                val_1 = ap.arg::<libc::c_int>();
                dst_3 = if !((*a).a_tid).is_null() {
                    &mut (*(*a).a_tid).joinable
                } else {
                    &mut (*a).a_joinable
                };
            } else {
                src_3 = if !((*a).a_tid).is_null() {
                    &mut (*(*a).a_tid).joinable
                } else {
                    &mut (*a).a_joinable
                };
                dst_3 = ap.arg::<*mut libc::c_int>();
            }
            *dst_3 = *src_3;
        }
        3 => {
            let mut val_2: libc::c_uint = 0;
            let mut src_4: *mut libc::c_uint = 0 as *mut libc::c_uint;
            let mut dst_4: *mut libc::c_uint = 0 as *mut libc::c_uint;
            if cmd == PTH_ATTR_SET as libc::c_int {
                src_4 = &mut val_2;
                val_2 = ap.arg::<libc::c_uint>();
                dst_4 = if !((*a).a_tid).is_null() {
                    &mut (*(*a).a_tid).cancelstate
                } else {
                    &mut (*a).a_cancelstate
                };
            } else {
                src_4 = if !((*a).a_tid).is_null() {
                    &mut (*(*a).a_tid).cancelstate
                } else {
                    &mut (*a).a_cancelstate
                };
                dst_4 = ap.arg::<*mut libc::c_uint>();
            }
            *dst_4 = *src_4;
        }
        4 => {
            let mut val_3: libc::c_uint = 0;
            let mut src_5: *mut libc::c_uint = 0 as *mut libc::c_uint;
            let mut dst_5: *mut libc::c_uint = 0 as *mut libc::c_uint;
            if cmd == PTH_ATTR_SET as libc::c_int {
                if !((*a).a_tid).is_null() {
                    *__errno_location() = 1 as libc::c_int;
                    return 0 as libc::c_int;
                }
                src_5 = &mut val_3;
                val_3 = ap.arg::<libc::c_uint>();
                dst_5 = &mut (*a).a_stacksize;
            } else {
                src_5 = if !((*a).a_tid).is_null() {
                    &mut (*(*a).a_tid).stacksize
                } else {
                    &mut (*a).a_stacksize
                };
                dst_5 = ap.arg::<*mut libc::c_uint>();
            }
            *dst_5 = *src_5;
        }
        5 => {
            let mut val_4: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut src_6: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
            let mut dst_6: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
            if cmd == PTH_ATTR_SET as libc::c_int {
                if !((*a).a_tid).is_null() {
                    *__errno_location() = 1 as libc::c_int;
                    return 0 as libc::c_int;
                }
                src_6 = &mut val_4;
                val_4 = ap.arg::<*mut libc::c_char>();
                dst_6 = &mut (*a).a_stackaddr;
            } else {
                src_6 = if !((*a).a_tid).is_null() {
                    &mut (*(*a).a_tid).stack
                } else {
                    &mut (*a).a_stackaddr
                };
                dst_6 = ap.arg::<*mut *mut libc::c_char>();
            }
            *dst_6 = *src_6;
        }
        7 => {
            let mut dst_7: *mut pth_time_t = 0 as *mut pth_time_t;
            if cmd == PTH_ATTR_SET as libc::c_int {
                *__errno_location() = 1 as libc::c_int;
                return 0 as libc::c_int;
            }
            dst_7 = ap.arg::<*mut pth_time_t>();
            if !((*a).a_tid).is_null() {
                if (&mut (*(*a).a_tid).spawned as *mut pth_time_t).is_null() {
                    gettimeofday(dst_7, 0 as *mut libc::c_void);
                } else {
                    (*dst_7).tv_sec = (*(*a).a_tid).spawned.tv_sec;
                    (*dst_7).tv_usec = (*(*a).a_tid).spawned.tv_usec;
                }
            } else if (&mut __pth_time_zero as *mut pth_time_t).is_null() {
                gettimeofday(dst_7, 0 as *mut libc::c_void);
            } else {
                (*dst_7).tv_sec = __pth_time_zero.tv_sec;
                (*dst_7).tv_usec = __pth_time_zero.tv_usec;
            }
        }
        8 => {
            let mut dst_8: *mut pth_time_t = 0 as *mut pth_time_t;
            if cmd == PTH_ATTR_SET as libc::c_int {
                *__errno_location() = 1 as libc::c_int;
                return 0 as libc::c_int;
            }
            dst_8 = ap.arg::<*mut pth_time_t>();
            if !((*a).a_tid).is_null() {
                if (&mut (*(*a).a_tid).lastran as *mut pth_time_t).is_null() {
                    gettimeofday(dst_8, 0 as *mut libc::c_void);
                } else {
                    (*dst_8).tv_sec = (*(*a).a_tid).lastran.tv_sec;
                    (*dst_8).tv_usec = (*(*a).a_tid).lastran.tv_usec;
                }
            } else if (&mut __pth_time_zero as *mut pth_time_t).is_null() {
                gettimeofday(dst_8, 0 as *mut libc::c_void);
            } else {
                (*dst_8).tv_sec = __pth_time_zero.tv_sec;
                (*dst_8).tv_usec = __pth_time_zero.tv_usec;
            }
        }
        9 => {
            let mut dst_9: *mut pth_time_t = 0 as *mut pth_time_t;
            if cmd == PTH_ATTR_SET as libc::c_int {
                *__errno_location() = 1 as libc::c_int;
                return 0 as libc::c_int;
            }
            dst_9 = ap.arg::<*mut pth_time_t>();
            if !((*a).a_tid).is_null() {
                if (&mut (*(*a).a_tid).running as *mut pth_time_t).is_null() {
                    gettimeofday(dst_9, 0 as *mut libc::c_void);
                } else {
                    (*dst_9).tv_sec = (*(*a).a_tid).running.tv_sec;
                    (*dst_9).tv_usec = (*(*a).a_tid).running.tv_usec;
                }
            } else if (&mut __pth_time_zero as *mut pth_time_t).is_null() {
                gettimeofday(dst_9, 0 as *mut libc::c_void);
            } else {
                (*dst_9).tv_sec = __pth_time_zero.tv_sec;
                (*dst_9).tv_usec = __pth_time_zero.tv_usec;
            }
        }
        10 => {
            let mut dst_10: *mut Option::<
                unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
            > = 0
                as *mut Option::<
                    unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
                >;
            if cmd == PTH_ATTR_SET as libc::c_int {
                *__errno_location() = 1 as libc::c_int;
                return 0 as libc::c_int;
            }
            if ((*a).a_tid).is_null() {
                *__errno_location() = 13 as libc::c_int;
                return 0 as libc::c_int;
            }
            dst_10 = ap.arg::<*mut libc::c_void>()
                as *mut Option::<
                    unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
                >;
            *dst_10 = (*(*a).a_tid).start_func;
        }
        11 => {
            let mut dst_11: *mut *mut libc::c_void = 0 as *mut *mut libc::c_void;
            if cmd == PTH_ATTR_SET as libc::c_int {
                *__errno_location() = 1 as libc::c_int;
                return 0 as libc::c_int;
            }
            if ((*a).a_tid).is_null() {
                *__errno_location() = 13 as libc::c_int;
                return 0 as libc::c_int;
            }
            dst_11 = ap.arg::<*mut *mut libc::c_void>();
            *dst_11 = (*(*a).a_tid).start_arg;
        }
        12 => {
            let mut dst_12: *mut pth_state_t = 0 as *mut pth_state_t;
            if cmd == PTH_ATTR_SET as libc::c_int {
                *__errno_location() = 1 as libc::c_int;
                return 0 as libc::c_int;
            }
            if ((*a).a_tid).is_null() {
                *__errno_location() = 13 as libc::c_int;
                return 0 as libc::c_int;
            }
            dst_12 = ap.arg::<*mut pth_state_t>();
            *dst_12 = (*(*a).a_tid).state;
        }
        13 => {
            let mut dst_13: *mut pth_event_t = 0 as *mut pth_event_t;
            if cmd == PTH_ATTR_SET as libc::c_int {
                *__errno_location() = 1 as libc::c_int;
                return 0 as libc::c_int;
            }
            if ((*a).a_tid).is_null() {
                *__errno_location() = 13 as libc::c_int;
                return 0 as libc::c_int;
            }
            dst_13 = ap.arg::<*mut pth_event_t>();
            *dst_13 = (*(*a).a_tid).events;
        }
        14 => {
            let mut dst_14: *mut libc::c_int = 0 as *mut libc::c_int;
            if cmd == PTH_ATTR_SET as libc::c_int {
                *__errno_location() = 1 as libc::c_int;
                return 0 as libc::c_int;
            }
            dst_14 = ap.arg::<*mut libc::c_int>();
            *dst_14 = if !((*a).a_tid).is_null() {
                (0 as libc::c_int == 0) as libc::c_int
            } else {
                0 as libc::c_int
            };
        }
        _ => {
            *__errno_location() = 22 as libc::c_int;
            return 0 as libc::c_int;
        }
    }
    return (0 as libc::c_int == 0) as libc::c_int;
}
