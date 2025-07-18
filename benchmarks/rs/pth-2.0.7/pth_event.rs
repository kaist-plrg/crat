use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn __errno_location() -> *mut libc::c_int;
    static mut __pth_current: pth_t;
    fn __pth_util_fd_valid(_: libc::c_int) -> libc::c_int;
    fn pth_yield(_: pth_t) -> libc::c_int;
    fn pth_cancel_point();
    fn pth_key_setdata(_: pth_key_t, _: *const libc::c_void) -> libc::c_int;
    fn pth_key_getdata(_: pth_key_t) -> *mut libc::c_void;
    fn pth_key_create(
        _: *mut pth_key_t,
        _: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    ) -> libc::c_int;
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
pub type C2RustUnnamed_9 = libc::c_uint;
pub const PTH_FREE_ALL: C2RustUnnamed_9 = 1;
pub const PTH_FREE_THIS: C2RustUnnamed_9 = 0;
pub type pth_key_t = libc::c_int;
unsafe extern "C" fn pth_event_destructor(mut vp: *mut libc::c_void) {
    pth_event_free(vp as pth_event_t, PTH_FREE_THIS as libc::c_int);
}
pub unsafe extern "C" fn pth_event(
    mut spec: libc::c_ulong,
    mut args: ...
) -> pth_event_t {
    let mut ev: pth_event_t = 0 as *mut pth_event_st;
    let mut ev_key: *mut pth_key_t = 0 as *mut pth_key_t;
    let mut ap: ::std::ffi::VaListImpl;
    ap = args.clone();
    if spec & ((1 as libc::c_int) << 20 as libc::c_int) as libc::c_ulong != 0 {
        ev = ap.arg::<pth_event_t>();
    } else if spec & ((1 as libc::c_int) << 22 as libc::c_int) as libc::c_ulong != 0 {
        ev_key = ap.arg::<*mut pth_key_t>();
        if *ev_key == -(1 as libc::c_int) {
            pth_key_create(
                ev_key,
                Some(
                    pth_event_destructor as unsafe extern "C" fn(*mut libc::c_void) -> (),
                ),
            );
        }
        ev = pth_key_getdata(*ev_key) as pth_event_t;
        if ev.is_null() {
            ev = malloc(::std::mem::size_of::<pth_event_st>() as libc::c_ulong)
                as pth_event_t;
            pth_key_setdata(*ev_key, ev as *const libc::c_void);
        }
    } else {
        ev = malloc(::std::mem::size_of::<pth_event_st>() as libc::c_ulong)
            as pth_event_t;
    }
    if ev.is_null() {
        *__errno_location() = *__errno_location();
        return 0 as *mut libc::c_void as pth_event_t;
    }
    if spec & ((1 as libc::c_int) << 21 as libc::c_int) as libc::c_ulong != 0 {
        let mut ch: pth_event_t = ap.arg::<pth_event_t>();
        (*ev).ev_prev = (*ch).ev_prev;
        (*ev).ev_next = ch;
        (*(*ev).ev_prev).ev_next = ev;
        (*(*ev).ev_next).ev_prev = ev;
    } else {
        (*ev).ev_prev = ev;
        (*ev).ev_next = ev;
    }
    (*ev).ev_status = PTH_STATUS_PENDING;
    if spec & ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_ulong != 0 {
        let mut fd: libc::c_int = ap.arg::<libc::c_int>();
        if __pth_util_fd_valid(fd) == 0 {
            *__errno_location() = 9 as libc::c_int;
            return 0 as *mut libc::c_void as pth_event_t;
        }
        (*ev).ev_type = (1 as libc::c_int) << 1 as libc::c_int;
        (*ev)
            .ev_goal = (spec
            & ((1 as libc::c_int) << 12 as libc::c_int
                | (1 as libc::c_int) << 13 as libc::c_int
                | (1 as libc::c_int) << 14 as libc::c_int) as libc::c_ulong)
            as libc::c_int;
        (*ev).ev_args.FD.fd = fd;
    } else if spec & ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_ulong != 0 {
        let mut n: *mut libc::c_int = ap.arg::<*mut libc::c_int>();
        let mut nfd: libc::c_int = ap.arg::<libc::c_int>();
        let mut rfds: *mut fd_set = ap.arg::<*mut fd_set>();
        let mut wfds: *mut fd_set = ap.arg::<*mut fd_set>();
        let mut efds: *mut fd_set = ap.arg::<*mut fd_set>();
        (*ev).ev_type = (1 as libc::c_int) << 2 as libc::c_int;
        (*ev)
            .ev_goal = (spec
            & ((1 as libc::c_int) << 11 as libc::c_int) as libc::c_ulong) as libc::c_int;
        (*ev).ev_args.SELECT.n = n;
        (*ev).ev_args.SELECT.nfd = nfd;
        (*ev).ev_args.SELECT.rfds = rfds;
        (*ev).ev_args.SELECT.wfds = wfds;
        (*ev).ev_args.SELECT.efds = efds;
    } else if spec & ((1 as libc::c_int) << 3 as libc::c_int) as libc::c_ulong != 0 {
        let mut sigs: *mut sigset_t = ap.arg::<*mut sigset_t>();
        let mut sig: *mut libc::c_int = ap.arg::<*mut libc::c_int>();
        (*ev).ev_type = (1 as libc::c_int) << 3 as libc::c_int;
        (*ev)
            .ev_goal = (spec
            & ((1 as libc::c_int) << 11 as libc::c_int) as libc::c_ulong) as libc::c_int;
        (*ev).ev_args.SIGS.sigs = sigs;
        (*ev).ev_args.SIGS.sig = sig;
    } else if spec & ((1 as libc::c_int) << 4 as libc::c_int) as libc::c_ulong != 0 {
        let mut tv: pth_time_t = timeval { tv_sec: 0, tv_usec: 0 };
        (*ev).ev_type = (1 as libc::c_int) << 4 as libc::c_int;
        (*ev)
            .ev_goal = (spec
            & ((1 as libc::c_int) << 11 as libc::c_int) as libc::c_ulong) as libc::c_int;
        (*ev).ev_args.TIME.tv = tv;
    } else if spec & ((1 as libc::c_int) << 5 as libc::c_int) as libc::c_ulong != 0 {
        let mut mp: pth_msgport_t = ap.arg::<pth_msgport_t>();
        (*ev).ev_type = (1 as libc::c_int) << 5 as libc::c_int;
        (*ev)
            .ev_goal = (spec
            & ((1 as libc::c_int) << 11 as libc::c_int) as libc::c_ulong) as libc::c_int;
        (*ev).ev_args.MSG.mp = mp;
    } else if spec & ((1 as libc::c_int) << 6 as libc::c_int) as libc::c_ulong != 0 {
        let mut mutex: *mut pth_mutex_t = ap.arg::<*mut pth_mutex_t>();
        (*ev).ev_type = (1 as libc::c_int) << 6 as libc::c_int;
        (*ev)
            .ev_goal = (spec
            & ((1 as libc::c_int) << 11 as libc::c_int) as libc::c_ulong) as libc::c_int;
        (*ev).ev_args.MUTEX.mutex = mutex;
    } else if spec & ((1 as libc::c_int) << 7 as libc::c_int) as libc::c_ulong != 0 {
        let mut cond: *mut pth_cond_t = ap.arg::<*mut pth_cond_t>();
        (*ev).ev_type = (1 as libc::c_int) << 7 as libc::c_int;
        (*ev)
            .ev_goal = (spec
            & ((1 as libc::c_int) << 11 as libc::c_int) as libc::c_ulong) as libc::c_int;
        (*ev).ev_args.COND.cond = cond;
    } else if spec & ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_ulong != 0 {
        let mut tid: pth_t = ap.arg::<pth_t>();
        let mut goal: libc::c_int = 0;
        (*ev).ev_type = (1 as libc::c_int) << 8 as libc::c_int;
        if spec & ((1 as libc::c_int) << 15 as libc::c_int) as libc::c_ulong != 0 {
            goal = PTH_STATE_NEW as libc::c_int;
        } else if spec & ((1 as libc::c_int) << 16 as libc::c_int) as libc::c_ulong != 0
        {
            goal = PTH_STATE_READY as libc::c_int;
        } else if spec & ((1 as libc::c_int) << 17 as libc::c_int) as libc::c_ulong != 0
        {
            goal = PTH_STATE_WAITING as libc::c_int;
        } else if spec & ((1 as libc::c_int) << 18 as libc::c_int) as libc::c_ulong != 0
        {
            goal = PTH_STATE_DEAD as libc::c_int;
        } else {
            goal = PTH_STATE_READY as libc::c_int;
        }
        (*ev).ev_goal = goal;
        (*ev).ev_args.TID.tid = tid;
    } else if spec & ((1 as libc::c_int) << 9 as libc::c_int) as libc::c_ulong != 0 {
        (*ev).ev_type = (1 as libc::c_int) << 9 as libc::c_int;
        (*ev)
            .ev_goal = (spec
            & ((1 as libc::c_int) << 11 as libc::c_int) as libc::c_ulong) as libc::c_int;
        (*ev)
            .ev_args
            .FUNC
            .func = ::std::mem::transmute(
            ap.arg::<*mut unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int>(),
        );
        (*ev).ev_args.FUNC.arg = ap.arg::<*mut libc::c_void>();
        (*ev).ev_args.FUNC.tv = timeval { tv_sec: 0, tv_usec: 0 };
    } else {
        *__errno_location() = 22 as libc::c_int;
        return 0 as *mut libc::c_void as pth_event_t;
    }
    return ev;
}
pub unsafe extern "C" fn pth_event_typeof(mut ev: pth_event_t) -> libc::c_ulong {
    if ev.is_null() {
        *__errno_location() = 22 as libc::c_int;
        return 0 as libc::c_int as libc::c_ulong;
    }
    return ((*ev).ev_type | (*ev).ev_goal) as libc::c_ulong;
}
pub unsafe extern "C" fn pth_event_extract(
    mut ev: pth_event_t,
    mut args: ...
) -> libc::c_int {
    let mut ap: ::std::ffi::VaListImpl;
    if ev.is_null() {
        *__errno_location() = 22 as libc::c_int;
        return 0 as libc::c_int;
    }
    ap = args.clone();
    if (*ev).ev_type & (1 as libc::c_int) << 1 as libc::c_int != 0 {
        let mut fd: *mut libc::c_int = ap.arg::<*mut libc::c_int>();
        *fd = (*ev).ev_args.FD.fd;
    } else if (*ev).ev_type & (1 as libc::c_int) << 3 as libc::c_int != 0 {
        let mut sigs: *mut *mut sigset_t = ap.arg::<*mut *mut sigset_t>();
        let mut sig: *mut *mut libc::c_int = ap.arg::<*mut *mut libc::c_int>();
        *sigs = (*ev).ev_args.SIGS.sigs;
        *sig = (*ev).ev_args.SIGS.sig;
    } else if (*ev).ev_type & (1 as libc::c_int) << 4 as libc::c_int != 0 {
        let mut tv: *mut pth_time_t = ap.arg::<*mut pth_time_t>();
        *tv = (*ev).ev_args.TIME.tv;
    } else if (*ev).ev_type & (1 as libc::c_int) << 5 as libc::c_int != 0 {
        let mut mp: *mut pth_msgport_t = ap.arg::<*mut pth_msgport_t>();
        *mp = (*ev).ev_args.MSG.mp;
    } else if (*ev).ev_type & (1 as libc::c_int) << 6 as libc::c_int != 0 {
        let mut mutex: *mut *mut pth_mutex_t = ap.arg::<*mut *mut pth_mutex_t>();
        *mutex = (*ev).ev_args.MUTEX.mutex;
    } else if (*ev).ev_type & (1 as libc::c_int) << 7 as libc::c_int != 0 {
        let mut cond: *mut *mut pth_cond_t = ap.arg::<*mut *mut pth_cond_t>();
        *cond = (*ev).ev_args.COND.cond;
    } else if (*ev).ev_type & (1 as libc::c_int) << 8 as libc::c_int != 0 {
        let mut tid: *mut pth_t = ap.arg::<*mut pth_t>();
        *tid = (*ev).ev_args.TID.tid;
    } else if (*ev).ev_type & (1 as libc::c_int) << 9 as libc::c_int != 0 {
        let mut func: *mut pth_event_func_t = ap.arg::<*mut pth_event_func_t>();
        let mut arg: *mut *mut libc::c_void = ap.arg::<*mut *mut libc::c_void>();
        let mut tv_0: *mut pth_time_t = ap.arg::<*mut pth_time_t>();
        *func = (*ev).ev_args.FUNC.func;
        *arg = (*ev).ev_args.FUNC.arg;
        *tv_0 = (*ev).ev_args.FUNC.tv;
    } else {
        *__errno_location() = 22 as libc::c_int;
        return 0 as libc::c_int;
    }
    return (0 as libc::c_int == 0) as libc::c_int;
}
pub unsafe extern "C" fn pth_event_concat(
    mut evf: pth_event_t,
    mut args: ...
) -> pth_event_t {
    let mut evc: pth_event_t = 0 as *mut pth_event_st;
    let mut evn: pth_event_t = 0 as *mut pth_event_st;
    let mut evl: pth_event_t = 0 as *mut pth_event_st;
    let mut evt: pth_event_t = 0 as *mut pth_event_st;
    let mut ap: ::std::ffi::VaListImpl;
    if evf.is_null() {
        *__errno_location() = 22 as libc::c_int;
        return 0 as *mut libc::c_void as pth_event_t;
    }
    ap = args.clone();
    evc = evf;
    evl = (*evc).ev_next;
    loop {
        evn = ap.arg::<pth_event_t>();
        if evn.is_null() {
            break;
        }
        (*evc).ev_next = evn;
        evt = (*evn).ev_prev;
        (*evn).ev_prev = evc;
        evc = evt;
    }
    (*evc).ev_next = evl;
    (*evl).ev_prev = evc;
    return evf;
}
pub unsafe extern "C" fn pth_event_isolate(mut ev: pth_event_t) -> pth_event_t {
    let mut ring: pth_event_t = 0 as *mut pth_event_st;
    if ev.is_null() {
        *__errno_location() = 22 as libc::c_int;
        return 0 as *mut libc::c_void as pth_event_t;
    }
    ring = 0 as pth_event_t;
    if !((*ev).ev_next == ev && (*ev).ev_prev == ev) {
        ring = (*ev).ev_next;
        (*(*ev).ev_prev).ev_next = (*ev).ev_next;
        (*(*ev).ev_next).ev_prev = (*ev).ev_prev;
        (*ev).ev_prev = ev;
        (*ev).ev_next = ev;
    }
    return ring;
}
pub unsafe extern "C" fn pth_event_status(mut ev: pth_event_t) -> pth_status_t {
    if ev.is_null() {
        *__errno_location() = 22 as libc::c_int;
        return 0 as libc::c_int as pth_status_t;
    }
    return (*ev).ev_status;
}
pub unsafe extern "C" fn pth_event_walk(
    mut ev: pth_event_t,
    mut direction: libc::c_uint,
) -> pth_event_t {
    if ev.is_null() {
        *__errno_location() = 22 as libc::c_int;
        return 0 as *mut libc::c_void as pth_event_t;
    }
    loop {
        if direction & ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_uint != 0 {
            ev = (*ev).ev_next;
        } else if direction & ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uint
            != 0
        {
            ev = (*ev).ev_prev;
        } else {
            *__errno_location() = 22 as libc::c_int;
            return 0 as *mut libc::c_void as pth_event_t;
        }
        if !(direction & ((1 as libc::c_int) << 11 as libc::c_int) as libc::c_uint != 0
            && (*ev).ev_status as libc::c_uint
                != PTH_STATUS_OCCURRED as libc::c_int as libc::c_uint)
        {
            break;
        }
    }
    return ev;
}
pub unsafe extern "C" fn pth_event_free(
    mut ev: pth_event_t,
    mut mode: libc::c_int,
) -> libc::c_int {
    let mut evc: pth_event_t = 0 as *mut pth_event_st;
    let mut evn: pth_event_t = 0 as *mut pth_event_st;
    if ev.is_null() {
        *__errno_location() = 22 as libc::c_int;
        return 0 as libc::c_int;
    }
    if mode == PTH_FREE_THIS as libc::c_int {
        (*(*ev).ev_prev).ev_next = (*ev).ev_next;
        (*(*ev).ev_next).ev_prev = (*ev).ev_prev;
        free(ev as *mut libc::c_void);
    } else if mode == PTH_FREE_ALL as libc::c_int {
        evc = ev;
        loop {
            evn = (*evc).ev_next;
            free(evc as *mut libc::c_void);
            evc = evn;
            if !(evc != ev) {
                break;
            }
        }
    }
    return (0 as libc::c_int == 0) as libc::c_int;
}
pub unsafe extern "C" fn pth_wait(mut ev_ring: pth_event_t) -> libc::c_int {
    let mut nonpending: libc::c_int = 0;
    let mut ev: pth_event_t = 0 as *mut pth_event_st;
    if ev_ring.is_null() {
        *__errno_location() = 22 as libc::c_int;
        return -(1 as libc::c_int);
    }
    ev = ev_ring;
    loop {
        (*ev).ev_status = PTH_STATUS_PENDING;
        ev = (*ev).ev_next;
        if !(ev != ev_ring) {
            break;
        }
    }
    (*__pth_current).events = ev_ring;
    (*__pth_current).state = PTH_STATE_WAITING;
    pth_yield(0 as pth_t);
    pth_cancel_point();
    (*__pth_current).events = 0 as pth_event_t;
    ev = ev_ring;
    nonpending = 0 as libc::c_int;
    loop {
        if (*ev).ev_status as libc::c_uint
            != PTH_STATUS_PENDING as libc::c_int as libc::c_uint
        {
            nonpending += 1;
            nonpending;
        }
        ev = (*ev).ev_next;
        if !(ev != ev_ring) {
            break;
        }
    }
    return nonpending;
}
