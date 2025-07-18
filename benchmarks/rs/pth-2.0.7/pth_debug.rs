use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn getpid() -> __pid_t;
    fn __errno_location() -> *mut libc::c_int;
    static mut __pth_errno_storage: libc::c_int;
    static mut __pth_errno_flag: libc::c_int;
    static mut __pth_current: pth_t;
    static mut __pth_NQ: pth_pqueue_t;
    static mut __pth_RQ: pth_pqueue_t;
    static mut __pth_WQ: pth_pqueue_t;
    static mut __pth_SQ: pth_pqueue_t;
    static mut __pth_DQ: pth_pqueue_t;
    static mut __pth_loadval: libc::c_float;
    fn __pth_vsnprintf(
        _: *mut libc::c_char,
        _: size_t,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn __pth_util_cpystrn(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: size_t,
    ) -> *mut libc::c_char;
    fn __pth_snprintf(
        _: *mut libc::c_char,
        _: size_t,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn __pth_pqueue_walk(_: *mut pth_pqueue_t, _: pth_t, _: libc::c_int) -> pth_t;
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
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
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
pub struct pth_pqueue_st {
    pub q_head: pth_t,
    pub q_num: libc::c_int,
}
pub type pth_pqueue_t = pth_pqueue_st;
pub unsafe extern "C" fn __pth_debug(
    mut file: *const libc::c_char,
    mut line: libc::c_int,
    mut argc: libc::c_int,
    mut fmt: *const libc::c_char,
    mut args: ...
) {
    let mut ap: ::std::ffi::VaListImpl;
    static mut str: [libc::c_char; 1024] = [0; 1024];
    let mut n: size_t = 0;
    __pth_errno_storage = *__errno_location();
    __pth_errno_flag = (0 as libc::c_int == 0) as libc::c_int;
    while __pth_errno_flag != 0 {
        ap = args.clone();
        if !file.is_null() {
            __pth_snprintf(
                str.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
                b"%d:%s:%04d: \0" as *const u8 as *const libc::c_char,
                getpid(),
                file,
                line,
            );
        } else {
            str[0 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
        }
        n = strlen(str.as_mut_ptr());
        if argc == 1 as libc::c_int {
            __pth_util_cpystrn(
                str.as_mut_ptr().offset(n as isize),
                fmt,
                (::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong)
                    .wrapping_sub(n),
            );
        } else {
            __pth_vsnprintf(
                str.as_mut_ptr().offset(n as isize),
                (::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong)
                    .wrapping_sub(n),
                fmt,
                ap.as_va_list(),
            );
        }
        n = strlen(str.as_mut_ptr());
        let fresh0 = n;
        n = n.wrapping_add(1);
        str[fresh0 as usize] = '\n' as i32 as libc::c_char;
        write(2 as libc::c_int, str.as_mut_ptr() as *const libc::c_void, n);
        *__errno_location() = __pth_errno_storage;
        __pth_errno_flag = 0 as libc::c_int;
    }
}
pub unsafe extern "C" fn __pth_dumpstate(mut fp: *mut FILE) {
    fprintf(
        fp,
        b"+----------------------------------------------------------------------\n\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        fp,
        b"| Pth Version: %s\n\0" as *const u8 as *const libc::c_char,
        b"2.0.7 (08-Jun-2006)\0" as *const u8 as *const libc::c_char,
    );
    fprintf(
        fp,
        b"| Load Average: %.2f\n\0" as *const u8 as *const libc::c_char,
        __pth_loadval as libc::c_double,
    );
    __pth_dumpqueue(fp, b"NEW\0" as *const u8 as *const libc::c_char, &mut __pth_NQ);
    __pth_dumpqueue(fp, b"READY\0" as *const u8 as *const libc::c_char, &mut __pth_RQ);
    fprintf(fp, b"| Thread Queue RUNNING:\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        fp,
        b"|   1. thread 0x%lx (\"%s\")\n\0" as *const u8 as *const libc::c_char,
        __pth_current as libc::c_ulong,
        ((*__pth_current).name).as_mut_ptr(),
    );
    __pth_dumpqueue(fp, b"WAITING\0" as *const u8 as *const libc::c_char, &mut __pth_WQ);
    __pth_dumpqueue(
        fp,
        b"SUSPENDED\0" as *const u8 as *const libc::c_char,
        &mut __pth_SQ,
    );
    __pth_dumpqueue(fp, b"DEAD\0" as *const u8 as *const libc::c_char, &mut __pth_DQ);
    fprintf(
        fp,
        b"+----------------------------------------------------------------------\n\0"
            as *const u8 as *const libc::c_char,
    );
}
pub unsafe extern "C" fn __pth_dumpqueue(
    mut fp: *mut FILE,
    mut qn: *const libc::c_char,
    mut q: *mut pth_pqueue_t,
) {
    let mut t: pth_t = 0 as *mut pth_st;
    let mut n: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    fprintf(fp, b"| Thread Queue %s:\n\0" as *const u8 as *const libc::c_char, qn);
    n = if q.is_null() { -(1 as libc::c_int) } else { (*q).q_num };
    if n == 0 as libc::c_int {
        fprintf(fp, b"|   no threads\n\0" as *const u8 as *const libc::c_char);
    }
    i = 1 as libc::c_int;
    t = if q.is_null() { 0 as pth_t } else { (*q).q_head };
    while !t.is_null() {
        let fresh1 = i;
        i = i + 1;
        fprintf(
            fp,
            b"|   %d. thread 0x%lx (\"%s\")\n\0" as *const u8 as *const libc::c_char,
            fresh1,
            t as libc::c_ulong,
            ((*t).name).as_mut_ptr(),
        );
        t = __pth_pqueue_walk(q, t, (1 as libc::c_int) << 1 as libc::c_int);
    }
}
