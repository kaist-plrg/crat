use ::libc;
use ::c2rust_asm_casts;
use c2rust_asm_casts::AsmCastTrait;
use std::arch::asm;
extern "C" {
    fn lseek(__fd: libc::c_int, __offset: __off_t, __whence: libc::c_int) -> __off_t;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn execl(
        __path: *const libc::c_char,
        __arg: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn select(
        __nfds: libc::c_int,
        __readfds: *mut fd_set,
        __writefds: *mut fd_set,
        __exceptfds: *mut fd_set,
        __timeout: *mut timeval,
    ) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn sigemptyset(__set: *mut sigset_t) -> libc::c_int;
    fn sigaddset(__set: *mut sigset_t, __signo: libc::c_int) -> libc::c_int;
    fn sigismember(__set: *const sigset_t, __signo: libc::c_int) -> libc::c_int;
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
    fn sigpending(__set: *mut sigset_t) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn gettimeofday(__tv: *mut timeval, __tz: *mut libc::c_void) -> libc::c_int;
    fn waitpid(
        __pid: __pid_t,
        __stat_loc: *mut libc::c_int,
        __options: libc::c_int,
    ) -> __pid_t;
    fn __xstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn connect(
        __fd: libc::c_int,
        __addr: *const sockaddr,
        __len: socklen_t,
    ) -> libc::c_int;
    fn recv(
        __fd: libc::c_int,
        __buf: *mut libc::c_void,
        __n: size_t,
        __flags: libc::c_int,
    ) -> ssize_t;
    fn sendto(
        __fd: libc::c_int,
        __buf: *const libc::c_void,
        __n: size_t,
        __flags: libc::c_int,
        __addr: *const sockaddr,
        __addr_len: socklen_t,
    ) -> ssize_t;
    fn recvfrom(
        __fd: libc::c_int,
        __buf: *mut libc::c_void,
        __n: size_t,
        __flags: libc::c_int,
        __addr: *mut sockaddr,
        __addr_len: *mut socklen_t,
    ) -> ssize_t;
    fn getsockopt(
        __fd: libc::c_int,
        __level: libc::c_int,
        __optname: libc::c_int,
        __optval: *mut libc::c_void,
        __optlen: *mut socklen_t,
    ) -> libc::c_int;
    fn accept(
        __fd: libc::c_int,
        __addr: *mut sockaddr,
        __addr_len: *mut socklen_t,
    ) -> libc::c_int;
    fn readv(__fd: libc::c_int, __iovec: *const iovec, __count: libc::c_int) -> ssize_t;
    fn writev(__fd: libc::c_int, __iovec: *const iovec, __count: libc::c_int) -> ssize_t;
    fn pth_init() -> libc::c_int;
    fn pth_wait(_: pth_event_t) -> libc::c_int;
    fn pth_fdmode(_: libc::c_int, _: libc::c_int) -> libc::c_int;
    fn pth_time(_: libc::c_long, _: libc::c_long) -> pth_time_t;
    fn pth_timeout(_: libc::c_long, _: libc::c_long) -> pth_time_t;
    fn pth_event(_: libc::c_ulong, _: ...) -> pth_event_t;
    fn pth_event_concat(_: pth_event_t, _: ...) -> pth_event_t;
    fn pth_event_isolate(_: pth_event_t) -> pth_event_t;
    fn pth_event_status(_: pth_event_t) -> pth_status_t;
    fn pth_fork() -> pid_t;
    fn pth_mutex_acquire(
        _: *mut pth_mutex_t,
        _: libc::c_int,
        _: pth_event_t,
    ) -> libc::c_int;
    fn pth_mutex_release(_: *mut pth_mutex_t) -> libc::c_int;
    fn __pth_util_sigdelete(_: libc::c_int) -> libc::c_int;
    static mut __pth_errno_flag: libc::c_int;
    static mut __pth_errno_storage: libc::c_int;
    fn __pth_util_fd_valid(_: libc::c_int) -> libc::c_int;
    static mut __pth_initialized: libc::c_int;
    static mut __pth_time_zero: pth_time_t;
    fn __pth_time_cmp(_: *mut pth_time_t, _: *mut pth_time_t) -> libc::c_int;
    fn __pth_scheduler_kill();
    static mut __pth_current: pth_t;
}
pub type size_t = libc::c_ulong;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __clock_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
pub type off_t = __off_t;
pub type ssize_t = __ssize_t;
pub type pid_t = __pid_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
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
pub type socklen_t = __socklen_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: libc::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iovec {
    pub iov_base: *mut libc::c_void,
    pub iov_len: size_t,
}
pub type sa_family_t = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [libc::c_char; 14],
}
pub type C2RustUnnamed_10 = libc::c_uint;
pub const MSG_CMSG_CLOEXEC: C2RustUnnamed_10 = 1073741824;
pub const MSG_FASTOPEN: C2RustUnnamed_10 = 536870912;
pub const MSG_ZEROCOPY: C2RustUnnamed_10 = 67108864;
pub const MSG_BATCH: C2RustUnnamed_10 = 262144;
pub const MSG_WAITFORONE: C2RustUnnamed_10 = 65536;
pub const MSG_MORE: C2RustUnnamed_10 = 32768;
pub const MSG_NOSIGNAL: C2RustUnnamed_10 = 16384;
pub const MSG_ERRQUEUE: C2RustUnnamed_10 = 8192;
pub const MSG_RST: C2RustUnnamed_10 = 4096;
pub const MSG_CONFIRM: C2RustUnnamed_10 = 2048;
pub const MSG_SYN: C2RustUnnamed_10 = 1024;
pub const MSG_FIN: C2RustUnnamed_10 = 512;
pub const MSG_WAITALL: C2RustUnnamed_10 = 256;
pub const MSG_EOR: C2RustUnnamed_10 = 128;
pub const MSG_DONTWAIT: C2RustUnnamed_10 = 64;
pub const MSG_TRUNC: C2RustUnnamed_10 = 32;
pub const MSG_PROXY: C2RustUnnamed_10 = 16;
pub const MSG_CTRUNC: C2RustUnnamed_10 = 8;
pub const MSG_DONTROUTE: C2RustUnnamed_10 = 4;
pub const MSG_PEEK: C2RustUnnamed_10 = 2;
pub const MSG_OOB: C2RustUnnamed_10 = 1;
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
    pub ev_args: C2RustUnnamed_11,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_11 {
    pub FD: C2RustUnnamed_20,
    pub SELECT: C2RustUnnamed_19,
    pub SIGS: C2RustUnnamed_18,
    pub TIME: C2RustUnnamed_17,
    pub MSG: C2RustUnnamed_16,
    pub MUTEX: C2RustUnnamed_15,
    pub COND: C2RustUnnamed_14,
    pub TID: C2RustUnnamed_13,
    pub FUNC: C2RustUnnamed_12,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_12 {
    pub func: pth_event_func_t,
    pub arg: *mut libc::c_void,
    pub tv: pth_time_t,
}
pub type pth_event_func_t = Option::<
    unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_13 {
    pub tid: pth_t,
}
pub type pth_t = *mut pth_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_14 {
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
pub struct C2RustUnnamed_15 {
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
pub struct C2RustUnnamed_16 {
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
pub struct C2RustUnnamed_17 {
    pub tv: pth_time_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_18 {
    pub sigs: *mut sigset_t,
    pub sig: *mut libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_19 {
    pub n: *mut libc::c_int,
    pub nfd: libc::c_int,
    pub rfds: *mut fd_set,
    pub wfds: *mut fd_set,
    pub efds: *mut fd_set,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_20 {
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
pub type C2RustUnnamed_21 = libc::c_int;
pub const PTH_FDMODE_NONBLOCK: C2RustUnnamed_21 = 2;
pub const PTH_FDMODE_BLOCK: C2RustUnnamed_21 = 1;
pub const PTH_FDMODE_POLL: C2RustUnnamed_21 = 0;
pub const PTH_FDMODE_ERROR: C2RustUnnamed_21 = -1;
pub type nfds_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pollfd {
    pub fd: libc::c_int,
    pub events: libc::c_short,
    pub revents: libc::c_short,
}
#[inline]
unsafe extern "C" fn stat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __xstat(1 as libc::c_int, __path, __statbuf);
}
pub unsafe extern "C" fn pth_nanosleep(
    mut rqtp: *const timespec,
    mut rmtp: *mut timespec,
) -> libc::c_int {
    let mut until: pth_time_t = pth_time_t {
        tv_sec: 0,
        tv_usec: 0,
    };
    let mut offset: pth_time_t = pth_time_t {
        tv_sec: 0,
        tv_usec: 0,
    };
    let mut now: pth_time_t = pth_time_t {
        tv_sec: 0,
        tv_usec: 0,
    };
    let mut ev: pth_event_t = 0 as *mut pth_event_st;
    static mut ev_key: pth_key_t = -(1 as libc::c_int);
    if rqtp.is_null() {
        *__errno_location() = 14 as libc::c_int;
        return -(1 as libc::c_int);
    }
    if (*rqtp).tv_nsec < 0 as libc::c_int as libc::c_long
        || (*rqtp).tv_nsec
            > (1000 as libc::c_int * 1000000 as libc::c_int) as libc::c_long
    {
        *__errno_location() = 22 as libc::c_int;
        return -(1 as libc::c_int);
    }
    if (*rqtp).tv_sec == 0 as libc::c_int as libc::c_long
        && (*rqtp).tv_nsec == 0 as libc::c_int as libc::c_long
    {
        return 0 as libc::c_int;
    }
    offset = pth_time(
        (*rqtp).tv_sec,
        (*rqtp).tv_nsec / 1000 as libc::c_int as libc::c_long,
    );
    if (0 as *mut pth_time_t).is_null() {
        gettimeofday(&mut until, 0 as *mut libc::c_void);
    } else {
        until.tv_sec = (*(0 as *mut pth_time_t)).tv_sec;
        until.tv_usec = (*(0 as *mut pth_time_t)).tv_usec;
    }
    until.tv_sec += offset.tv_sec;
    until.tv_usec += offset.tv_usec;
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
    if ev.is_null() {
        *__errno_location() = *__errno_location();
        return -(1 as libc::c_int);
    }
    pth_wait(ev);
    if !rmtp.is_null() {
        if (0 as *mut pth_time_t).is_null() {
            gettimeofday(&mut now, 0 as *mut libc::c_void);
        } else {
            now.tv_sec = (*(0 as *mut pth_time_t)).tv_sec;
            now.tv_usec = (*(0 as *mut pth_time_t)).tv_usec;
        }
        until.tv_sec -= now.tv_sec;
        until.tv_usec -= now.tv_usec;
        if until.tv_usec < 0 as libc::c_int as libc::c_long {
            until.tv_sec -= 1 as libc::c_int as libc::c_long;
            until.tv_usec += 1000000 as libc::c_int as libc::c_long;
        }
        (*rmtp).tv_sec = until.tv_sec;
        (*rmtp).tv_nsec = until.tv_usec * 1000 as libc::c_int as libc::c_long;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn pth_usleep(mut usec: libc::c_uint) -> libc::c_int {
    let mut until: pth_time_t = pth_time_t {
        tv_sec: 0,
        tv_usec: 0,
    };
    let mut offset: pth_time_t = pth_time_t {
        tv_sec: 0,
        tv_usec: 0,
    };
    let mut ev: pth_event_t = 0 as *mut pth_event_st;
    static mut ev_key: pth_key_t = -(1 as libc::c_int);
    if usec == 0 as libc::c_int as libc::c_uint {
        return 0 as libc::c_int;
    }
    offset = pth_time(
        usec.wrapping_div(1000000 as libc::c_int as libc::c_uint) as libc::c_long,
        usec.wrapping_rem(1000000 as libc::c_int as libc::c_uint) as libc::c_long,
    );
    if (0 as *mut pth_time_t).is_null() {
        gettimeofday(&mut until, 0 as *mut libc::c_void);
    } else {
        until.tv_sec = (*(0 as *mut pth_time_t)).tv_sec;
        until.tv_usec = (*(0 as *mut pth_time_t)).tv_usec;
    }
    until.tv_sec += offset.tv_sec;
    until.tv_usec += offset.tv_usec;
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
    if ev.is_null() {
        *__errno_location() = *__errno_location();
        return -(1 as libc::c_int);
    }
    pth_wait(ev);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn pth_sleep(mut sec: libc::c_uint) -> libc::c_uint {
    let mut until: pth_time_t = pth_time_t {
        tv_sec: 0,
        tv_usec: 0,
    };
    let mut offset: pth_time_t = pth_time_t {
        tv_sec: 0,
        tv_usec: 0,
    };
    let mut ev: pth_event_t = 0 as *mut pth_event_st;
    static mut ev_key: pth_key_t = -(1 as libc::c_int);
    if sec == 0 as libc::c_int as libc::c_uint {
        return 0 as libc::c_int as libc::c_uint;
    }
    offset = pth_time(sec as libc::c_long, 0 as libc::c_int as libc::c_long);
    if (0 as *mut pth_time_t).is_null() {
        gettimeofday(&mut until, 0 as *mut libc::c_void);
    } else {
        until.tv_sec = (*(0 as *mut pth_time_t)).tv_sec;
        until.tv_usec = (*(0 as *mut pth_time_t)).tv_usec;
    }
    until.tv_sec += offset.tv_sec;
    until.tv_usec += offset.tv_usec;
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
    if ev.is_null() {
        return sec;
    }
    pth_wait(ev);
    return 0 as libc::c_int as libc::c_uint;
}
pub unsafe extern "C" fn pth_sigmask(
    mut how: libc::c_int,
    mut set: *const sigset_t,
    mut oset: *mut sigset_t,
) -> libc::c_int {
    let mut rv: libc::c_int = 0;
    if !set.is_null() {
        sigprocmask(how, &mut (*__pth_current).mctx.sigs, 0 as *mut sigset_t);
    }
    rv = sigprocmask(how, set, oset);
    return rv;
}
pub unsafe extern "C" fn pth_sigwait(
    mut set: *const sigset_t,
    mut sigp: *mut libc::c_int,
) -> libc::c_int {
    return pth_sigwait_ev(set, sigp, 0 as pth_event_t);
}
pub unsafe extern "C" fn pth_sigwait_ev(
    mut set: *const sigset_t,
    mut sigp: *mut libc::c_int,
    mut ev_extra: pth_event_t,
) -> libc::c_int {
    let mut ev: pth_event_t = 0 as *mut pth_event_st;
    static mut ev_key: pth_key_t = -(1 as libc::c_int);
    let mut pending: sigset_t = sigset_t { __val: [0; 16] };
    let mut sig: libc::c_int = 0;
    if set.is_null() || sigp.is_null() {
        *__errno_location() = 22 as libc::c_int;
        return 22 as libc::c_int;
    }
    if sigpending(&mut pending) < 0 as libc::c_int {
        sigemptyset(&mut pending);
    }
    sig = 1 as libc::c_int;
    while sig < 65 as libc::c_int {
        if sigismember(set, sig) != 0 && sigismember(&mut pending, sig) != 0 {
            __pth_util_sigdelete(sig);
            *sigp = sig;
            return 0 as libc::c_int;
        }
        sig += 1;
        sig;
    }
    ev = pth_event(
        ((1 as libc::c_int) << 3 as libc::c_int
            | (1 as libc::c_int) << 22 as libc::c_int) as libc::c_ulong,
        &mut ev_key as *mut pth_key_t,
        set,
        sigp,
    );
    if ev.is_null() {
        *__errno_location() = *__errno_location();
        return *__errno_location();
    }
    if !ev_extra.is_null() {
        pth_event_concat(ev, ev_extra, 0 as *mut libc::c_void);
    }
    pth_wait(ev);
    if !ev_extra.is_null() {
        pth_event_isolate(ev);
        if pth_event_status(ev) as libc::c_uint
            != PTH_STATUS_OCCURRED as libc::c_int as libc::c_uint
        {
            *__errno_location() = 4 as libc::c_int;
            return 4 as libc::c_int;
        }
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn pth_waitpid(
    mut wpid: pid_t,
    mut status: *mut libc::c_int,
    mut options: libc::c_int,
) -> pid_t {
    let mut ev: pth_event_t = 0 as *mut pth_event_st;
    static mut ev_key: pth_key_t = -(1 as libc::c_int);
    let mut pid: pid_t = 0;
    loop {
        loop {
            pid = waitpid(wpid, status, options | 1 as libc::c_int);
            if !(pid < 0 as libc::c_int && *__errno_location() == 4 as libc::c_int) {
                break;
            }
        }
        if pid == -(1 as libc::c_int) || pid > 0 as libc::c_int
            || pid == 0 as libc::c_int && options & 1 as libc::c_int != 0
        {
            break;
        }
        ev = pth_event(
            ((1 as libc::c_int) << 4 as libc::c_int
                | (1 as libc::c_int) << 22 as libc::c_int) as libc::c_ulong,
            &mut ev_key as *mut pth_key_t,
            pth_timeout(
                0 as libc::c_int as libc::c_long,
                250000 as libc::c_int as libc::c_long,
            ),
        );
        pth_wait(ev);
    }
    return pid;
}
pub unsafe extern "C" fn pth_system(mut cmd: *const libc::c_char) -> libc::c_int {
    let mut sa_ign: sigaction = sigaction {
        __sigaction_handler: C2RustUnnamed_9 {
            sa_handler: None,
        },
        sa_mask: sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    let mut sa_int: sigaction = sigaction {
        __sigaction_handler: C2RustUnnamed_9 {
            sa_handler: None,
        },
        sa_mask: sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    let mut sa_quit: sigaction = sigaction {
        __sigaction_handler: C2RustUnnamed_9 {
            sa_handler: None,
        },
        sa_mask: sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    let mut ss_block: sigset_t = sigset_t { __val: [0; 16] };
    let mut ss_old: sigset_t = sigset_t { __val: [0; 16] };
    let mut sb: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    let mut pid: pid_t = 0;
    let mut pstat: libc::c_int = 0;
    if cmd.is_null() {
        if stat(b"/bin/sh\0" as *const u8 as *const libc::c_char, &mut sb)
            == -(1 as libc::c_int)
        {
            return 0 as libc::c_int;
        }
        return 1 as libc::c_int;
    }
    sa_ign
        .__sigaction_handler
        .sa_handler = ::std::mem::transmute::<
        libc::intptr_t,
        __sighandler_t,
    >(1 as libc::c_int as libc::intptr_t);
    sigemptyset(&mut sa_ign.sa_mask);
    sa_ign.sa_flags = 0 as libc::c_int;
    sigaction(2 as libc::c_int, &mut sa_ign, &mut sa_int);
    sigaction(3 as libc::c_int, &mut sa_ign, &mut sa_quit);
    sigemptyset(&mut ss_block);
    sigaddset(&mut ss_block, 17 as libc::c_int);
    sigprocmask(0 as libc::c_int, &mut ss_block, &mut ss_old);
    pstat = -(1 as libc::c_int);
    pid = pth_fork();
    match pid {
        -1 => {}
        0 => {
            sigaction(2 as libc::c_int, &mut sa_int, 0 as *mut sigaction);
            sigaction(3 as libc::c_int, &mut sa_quit, 0 as *mut sigaction);
            sigprocmask(2 as libc::c_int, &mut ss_old, 0 as *mut sigset_t);
            __pth_scheduler_kill();
            execl(
                b"/bin/sh\0" as *const u8 as *const libc::c_char,
                b"sh\0" as *const u8 as *const libc::c_char,
                b"-c\0" as *const u8 as *const libc::c_char,
                cmd,
                0 as *mut libc::c_void as *mut libc::c_char,
            );
            exit(127 as libc::c_int);
        }
        _ => {
            pid = pth_waitpid(pid, &mut pstat, 0 as libc::c_int);
        }
    }
    sigaction(2 as libc::c_int, &mut sa_int, 0 as *mut sigaction);
    sigaction(3 as libc::c_int, &mut sa_quit, 0 as *mut sigaction);
    sigprocmask(2 as libc::c_int, &mut ss_old, 0 as *mut sigset_t);
    return if pid == -(1 as libc::c_int) { -(1 as libc::c_int) } else { pstat };
}
pub unsafe extern "C" fn pth_select(
    mut nfds: libc::c_int,
    mut rfds: *mut fd_set,
    mut wfds: *mut fd_set,
    mut efds: *mut fd_set,
    mut timeout: *mut timeval,
) -> libc::c_int {
    return pth_select_ev(nfds, rfds, wfds, efds, timeout, 0 as pth_event_t);
}
pub unsafe extern "C" fn pth_select_ev(
    mut nfd: libc::c_int,
    mut rfds: *mut fd_set,
    mut wfds: *mut fd_set,
    mut efds: *mut fd_set,
    mut timeout: *mut timeval,
    mut ev_extra: pth_event_t,
) -> libc::c_int {
    let mut delay: timeval = pth_time_t {
        tv_sec: 0,
        tv_usec: 0,
    };
    let mut ev: pth_event_t = 0 as *mut pth_event_st;
    let mut ev_select: pth_event_t = 0 as *mut pth_event_st;
    let mut ev_timeout: pth_event_t = 0 as *mut pth_event_st;
    static mut ev_key_select: pth_key_t = -(1 as libc::c_int);
    static mut ev_key_timeout: pth_key_t = -(1 as libc::c_int);
    let mut rspare: fd_set = fd_set { __fds_bits: [0; 16] };
    let mut wspare: fd_set = fd_set { __fds_bits: [0; 16] };
    let mut espare: fd_set = fd_set { __fds_bits: [0; 16] };
    let mut rtmp: *mut fd_set = 0 as *mut fd_set;
    let mut wtmp: *mut fd_set = 0 as *mut fd_set;
    let mut etmp: *mut fd_set = 0 as *mut fd_set;
    let mut selected: libc::c_int = 0;
    let mut rc: libc::c_int = 0;
    if __pth_initialized == 0 {
        pth_init();
    }
    if nfd < 0 as libc::c_int || nfd > 1024 as libc::c_int {
        *__errno_location() = 22 as libc::c_int;
        return -(1 as libc::c_int);
    }
    if !timeout.is_null() {
        if (*timeout).tv_sec < 0 as libc::c_int as libc::c_long
            || (*timeout).tv_usec < 0 as libc::c_int as libc::c_long
            || (*timeout).tv_usec >= 1000000 as libc::c_int as libc::c_long
        {
            *__errno_location() = 22 as libc::c_int;
            return -(1 as libc::c_int);
        }
        if (*timeout).tv_sec
            > (31 as libc::c_int * 24 as libc::c_int * 60 as libc::c_int
                * 60 as libc::c_int) as libc::c_long
        {
            (*timeout)
                .tv_sec = (31 as libc::c_int * 24 as libc::c_int * 60 as libc::c_int
                * 60 as libc::c_int) as __time_t;
        }
    }
    if nfd == 0 as libc::c_int && rfds.is_null() && wfds.is_null() && efds.is_null()
        && !timeout.is_null()
    {
        if (*timeout).tv_sec == 0 as libc::c_int as libc::c_long
            && (*timeout).tv_usec <= 10000 as libc::c_int as libc::c_long
        {
            while select(
                0 as libc::c_int,
                0 as *mut fd_set,
                0 as *mut fd_set,
                0 as *mut fd_set,
                timeout,
            ) < 0 as libc::c_int && *__errno_location() == 4 as libc::c_int
            {}
        } else {
            ev = pth_event(
                ((1 as libc::c_int) << 4 as libc::c_int
                    | (1 as libc::c_int) << 22 as libc::c_int) as libc::c_ulong,
                &mut ev_key_timeout as *mut pth_key_t,
                pth_timeout((*timeout).tv_sec, (*timeout).tv_usec),
            );
            if !ev_extra.is_null() {
                pth_event_concat(ev, ev_extra, 0 as *mut libc::c_void);
            }
            pth_wait(ev);
            if !ev_extra.is_null() {
                pth_event_isolate(ev);
                if pth_event_status(ev) as libc::c_uint
                    != PTH_STATUS_OCCURRED as libc::c_int as libc::c_uint
                {
                    *__errno_location() = 4 as libc::c_int;
                    return -(1 as libc::c_int);
                }
            }
        }
        if !rfds.is_null() {
            let mut __d0: libc::c_int = 0;
            let mut __d1: libc::c_int = 0;
            let fresh0 = &mut __d0;
            let fresh1;
            let fresh2 = (::std::mem::size_of::<fd_set>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<__fd_mask>() as libc::c_ulong);
            let fresh3 = &mut __d1;
            let fresh4;
            let fresh5 = &mut *((*rfds).__fds_bits)
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize) as *mut __fd_mask;
            asm!(
                "cld; rep; stosq", inlateout("cx")
                c2rust_asm_casts::AsmCast::cast_in(fresh0, fresh2) => fresh1,
                inlateout("di") c2rust_asm_casts::AsmCast::cast_in(fresh3, fresh5) =>
                fresh4, inlateout("ax") 0 as libc::c_int => _, options(preserves_flags,
                att_syntax)
            );
            c2rust_asm_casts::AsmCast::cast_out(fresh0, fresh2, fresh1);
            c2rust_asm_casts::AsmCast::cast_out(fresh3, fresh5, fresh4);
        }
        if !wfds.is_null() {
            let mut __d0_0: libc::c_int = 0;
            let mut __d1_0: libc::c_int = 0;
            let fresh6 = &mut __d0_0;
            let fresh7;
            let fresh8 = (::std::mem::size_of::<fd_set>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<__fd_mask>() as libc::c_ulong);
            let fresh9 = &mut __d1_0;
            let fresh10;
            let fresh11 = &mut *((*wfds).__fds_bits)
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize) as *mut __fd_mask;
            asm!(
                "cld; rep; stosq", inlateout("cx")
                c2rust_asm_casts::AsmCast::cast_in(fresh6, fresh8) => fresh7,
                inlateout("di") c2rust_asm_casts::AsmCast::cast_in(fresh9, fresh11) =>
                fresh10, inlateout("ax") 0 as libc::c_int => _, options(preserves_flags,
                att_syntax)
            );
            c2rust_asm_casts::AsmCast::cast_out(fresh6, fresh8, fresh7);
            c2rust_asm_casts::AsmCast::cast_out(fresh9, fresh11, fresh10);
        }
        if !efds.is_null() {
            let mut __d0_1: libc::c_int = 0;
            let mut __d1_1: libc::c_int = 0;
            let fresh12 = &mut __d0_1;
            let fresh13;
            let fresh14 = (::std::mem::size_of::<fd_set>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<__fd_mask>() as libc::c_ulong);
            let fresh15 = &mut __d1_1;
            let fresh16;
            let fresh17 = &mut *((*efds).__fds_bits)
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
        }
        return 0 as libc::c_int;
    }
    delay.tv_sec = 0 as libc::c_int as __time_t;
    delay.tv_usec = 0 as libc::c_int as __suseconds_t;
    rtmp = 0 as *mut fd_set;
    if !rfds.is_null() {
        memcpy(
            &mut rspare as *mut fd_set as *mut libc::c_void,
            rfds as *const libc::c_void,
            ::std::mem::size_of::<fd_set>() as libc::c_ulong,
        );
        rtmp = &mut rspare;
    }
    wtmp = 0 as *mut fd_set;
    if !wfds.is_null() {
        memcpy(
            &mut wspare as *mut fd_set as *mut libc::c_void,
            wfds as *const libc::c_void,
            ::std::mem::size_of::<fd_set>() as libc::c_ulong,
        );
        wtmp = &mut wspare;
    }
    etmp = 0 as *mut fd_set;
    if !efds.is_null() {
        memcpy(
            &mut espare as *mut fd_set as *mut libc::c_void,
            efds as *const libc::c_void,
            ::std::mem::size_of::<fd_set>() as libc::c_ulong,
        );
        etmp = &mut espare;
    }
    loop {
        rc = select(nfd, rtmp, wtmp, etmp, &mut delay);
        if !(rc < 0 as libc::c_int && *__errno_location() == 4 as libc::c_int) {
            break;
        }
    }
    if rc < 0 as libc::c_int {
        *__errno_location() = *__errno_location();
        return -(1 as libc::c_int);
    } else if rc > 0 as libc::c_int
        || rc == 0 as libc::c_int && !timeout.is_null()
            && __pth_time_cmp(timeout, &mut __pth_time_zero) == 0 as libc::c_int
    {
        if !rfds.is_null() {
            memcpy(
                rfds as *mut libc::c_void,
                &mut rspare as *mut fd_set as *const libc::c_void,
                ::std::mem::size_of::<fd_set>() as libc::c_ulong,
            );
        }
        if !wfds.is_null() {
            memcpy(
                wfds as *mut libc::c_void,
                &mut wspare as *mut fd_set as *const libc::c_void,
                ::std::mem::size_of::<fd_set>() as libc::c_ulong,
            );
        }
        if !efds.is_null() {
            memcpy(
                efds as *mut libc::c_void,
                &mut espare as *mut fd_set as *const libc::c_void,
                ::std::mem::size_of::<fd_set>() as libc::c_ulong,
            );
        }
        return rc;
    }
    rc = -(1 as libc::c_int);
    ev_select = pth_event(
        ((1 as libc::c_int) << 2 as libc::c_int
            | (1 as libc::c_int) << 22 as libc::c_int) as libc::c_ulong,
        &mut ev_key_select as *mut pth_key_t,
        &mut rc as *mut libc::c_int,
        nfd,
        rfds,
        wfds,
        efds,
    );
    ev = ev_select;
    ev_timeout = 0 as pth_event_t;
    if !timeout.is_null() {
        ev_timeout = pth_event(
            ((1 as libc::c_int) << 4 as libc::c_int
                | (1 as libc::c_int) << 22 as libc::c_int) as libc::c_ulong,
            &mut ev_key_timeout as *mut pth_key_t,
            pth_timeout((*timeout).tv_sec, (*timeout).tv_usec),
        );
        pth_event_concat(ev, ev_timeout, 0 as *mut libc::c_void);
    }
    if !ev_extra.is_null() {
        pth_event_concat(ev, ev_extra, 0 as *mut libc::c_void);
    }
    pth_wait(ev);
    if !ev_extra.is_null() {
        pth_event_isolate(ev_extra);
    }
    if !timeout.is_null() {
        pth_event_isolate(ev_timeout);
    }
    if pth_event_status(ev_select) as libc::c_uint
        == PTH_STATUS_FAILED as libc::c_int as libc::c_uint
    {
        *__errno_location() = 9 as libc::c_int;
        return -(1 as libc::c_int);
    }
    selected = 0 as libc::c_int;
    if pth_event_status(ev_select) as libc::c_uint
        == PTH_STATUS_OCCURRED as libc::c_int as libc::c_uint
    {
        selected = (0 as libc::c_int == 0) as libc::c_int;
    }
    if !timeout.is_null()
        && pth_event_status(ev_timeout) as libc::c_uint
            == PTH_STATUS_OCCURRED as libc::c_int as libc::c_uint
    {
        selected = (0 as libc::c_int == 0) as libc::c_int;
        if !rfds.is_null() {
            let mut __d0_2: libc::c_int = 0;
            let mut __d1_2: libc::c_int = 0;
            let fresh18 = &mut __d0_2;
            let fresh19;
            let fresh20 = (::std::mem::size_of::<fd_set>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<__fd_mask>() as libc::c_ulong);
            let fresh21 = &mut __d1_2;
            let fresh22;
            let fresh23 = &mut *((*rfds).__fds_bits)
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
        }
        if !wfds.is_null() {
            let mut __d0_3: libc::c_int = 0;
            let mut __d1_3: libc::c_int = 0;
            let fresh24 = &mut __d0_3;
            let fresh25;
            let fresh26 = (::std::mem::size_of::<fd_set>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<__fd_mask>() as libc::c_ulong);
            let fresh27 = &mut __d1_3;
            let fresh28;
            let fresh29 = &mut *((*wfds).__fds_bits)
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
        }
        if !efds.is_null() {
            let mut __d0_4: libc::c_int = 0;
            let mut __d1_4: libc::c_int = 0;
            let fresh30 = &mut __d0_4;
            let fresh31;
            let fresh32 = (::std::mem::size_of::<fd_set>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<__fd_mask>() as libc::c_ulong);
            let fresh33 = &mut __d1_4;
            let fresh34;
            let fresh35 = &mut *((*efds).__fds_bits)
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
        rc = 0 as libc::c_int;
    }
    if !ev_extra.is_null() && selected == 0 {
        *__errno_location() = 4 as libc::c_int;
        return -(1 as libc::c_int);
    }
    return rc;
}
pub unsafe extern "C" fn pth_pselect(
    mut nfds: libc::c_int,
    mut rfds: *mut fd_set,
    mut wfds: *mut fd_set,
    mut efds: *mut fd_set,
    mut ts: *const timespec,
    mut mask: *const sigset_t,
) -> libc::c_int {
    let mut omask: sigset_t = sigset_t { __val: [0; 16] };
    let mut tv: timeval = pth_time_t {
        tv_sec: 0,
        tv_usec: 0,
    };
    let mut tvp: *mut timeval = 0 as *mut timeval;
    let mut rv: libc::c_int = 0;
    if !ts.is_null() {
        tv.tv_sec = (*ts).tv_sec;
        tv.tv_usec = (*ts).tv_nsec / 1000 as libc::c_int as libc::c_long;
        tvp = &mut tv;
    } else {
        tvp = 0 as *mut timeval;
    }
    if !mask.is_null() {
        if sigprocmask(2 as libc::c_int, mask, &mut omask) < 0 as libc::c_int {
            *__errno_location() = *__errno_location();
            return -(1 as libc::c_int);
        }
    }
    rv = pth_select(nfds, rfds, wfds, efds, tvp);
    if !mask.is_null() {
        __pth_errno_storage = *__errno_location();
        __pth_errno_flag = (0 as libc::c_int == 0) as libc::c_int;
        while __pth_errno_flag != 0 {
            sigprocmask(2 as libc::c_int, &mut omask, 0 as *mut sigset_t);
            *__errno_location() = __pth_errno_storage;
            __pth_errno_flag = 0 as libc::c_int;
        }
    }
    return rv;
}
pub unsafe extern "C" fn pth_poll(
    mut pfd: *mut pollfd,
    mut nfd: nfds_t,
    mut timeout: libc::c_int,
) -> libc::c_int {
    return pth_poll_ev(pfd, nfd, timeout, 0 as pth_event_t);
}
pub unsafe extern "C" fn pth_poll_ev(
    mut pfd: *mut pollfd,
    mut nfd: nfds_t,
    mut timeout: libc::c_int,
    mut ev_extra: pth_event_t,
) -> libc::c_int {
    let mut rfds: fd_set = fd_set { __fds_bits: [0; 16] };
    let mut wfds: fd_set = fd_set { __fds_bits: [0; 16] };
    let mut efds: fd_set = fd_set { __fds_bits: [0; 16] };
    let mut xfds: fd_set = fd_set { __fds_bits: [0; 16] };
    let mut tv: timeval = pth_time_t {
        tv_sec: 0,
        tv_usec: 0,
    };
    let mut ptv: *mut timeval = 0 as *mut timeval;
    let mut maxfd: libc::c_int = 0;
    let mut rc: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut i: libc::c_uint = 0;
    let mut data: [libc::c_char; 64] = [0; 64];
    if __pth_initialized == 0 {
        pth_init();
    }
    if pfd.is_null() {
        *__errno_location() = 14 as libc::c_int;
        return -(1 as libc::c_int);
    }
    if nfd < 0 as libc::c_int as libc::c_ulong
        || nfd > 1024 as libc::c_int as libc::c_ulong
    {
        *__errno_location() = 22 as libc::c_int;
        return -(1 as libc::c_int);
    }
    ptv = &mut tv;
    if timeout == 0 as libc::c_int {
        (*ptv).tv_sec = 0 as libc::c_int as __time_t;
        (*ptv).tv_usec = 0 as libc::c_int as __suseconds_t;
    } else if timeout == -(1 as libc::c_int) {
        ptv = 0 as *mut timeval;
    } else if timeout > 0 as libc::c_int {
        (*ptv).tv_sec = (timeout / 1000 as libc::c_int) as __time_t;
        (*ptv)
            .tv_usec = (timeout % 1000 as libc::c_int * 1000 as libc::c_int)
            as __suseconds_t;
    } else {
        *__errno_location() = 22 as libc::c_int;
        return -(1 as libc::c_int);
    }
    maxfd = -(1 as libc::c_int);
    let mut __d0: libc::c_int = 0;
    let mut __d1: libc::c_int = 0;
    let fresh36 = &mut __d0;
    let fresh37;
    let fresh38 = (::std::mem::size_of::<fd_set>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<__fd_mask>() as libc::c_ulong);
    let fresh39 = &mut __d1;
    let fresh40;
    let fresh41 = &mut *(rfds.__fds_bits).as_mut_ptr().offset(0 as libc::c_int as isize)
        as *mut __fd_mask;
    asm!(
        "cld; rep; stosq", inlateout("cx") c2rust_asm_casts::AsmCast::cast_in(fresh36,
        fresh38) => fresh37, inlateout("di") c2rust_asm_casts::AsmCast::cast_in(fresh39,
        fresh41) => fresh40, inlateout("ax") 0 as libc::c_int => _,
        options(preserves_flags, att_syntax)
    );
    c2rust_asm_casts::AsmCast::cast_out(fresh36, fresh38, fresh37);
    c2rust_asm_casts::AsmCast::cast_out(fresh39, fresh41, fresh40);
    let mut __d0_0: libc::c_int = 0;
    let mut __d1_0: libc::c_int = 0;
    let fresh42 = &mut __d0_0;
    let fresh43;
    let fresh44 = (::std::mem::size_of::<fd_set>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<__fd_mask>() as libc::c_ulong);
    let fresh45 = &mut __d1_0;
    let fresh46;
    let fresh47 = &mut *(wfds.__fds_bits).as_mut_ptr().offset(0 as libc::c_int as isize)
        as *mut __fd_mask;
    asm!(
        "cld; rep; stosq", inlateout("cx") c2rust_asm_casts::AsmCast::cast_in(fresh42,
        fresh44) => fresh43, inlateout("di") c2rust_asm_casts::AsmCast::cast_in(fresh45,
        fresh47) => fresh46, inlateout("ax") 0 as libc::c_int => _,
        options(preserves_flags, att_syntax)
    );
    c2rust_asm_casts::AsmCast::cast_out(fresh42, fresh44, fresh43);
    c2rust_asm_casts::AsmCast::cast_out(fresh45, fresh47, fresh46);
    let mut __d0_1: libc::c_int = 0;
    let mut __d1_1: libc::c_int = 0;
    let fresh48 = &mut __d0_1;
    let fresh49;
    let fresh50 = (::std::mem::size_of::<fd_set>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<__fd_mask>() as libc::c_ulong);
    let fresh51 = &mut __d1_1;
    let fresh52;
    let fresh53 = &mut *(efds.__fds_bits).as_mut_ptr().offset(0 as libc::c_int as isize)
        as *mut __fd_mask;
    asm!(
        "cld; rep; stosq", inlateout("cx") c2rust_asm_casts::AsmCast::cast_in(fresh48,
        fresh50) => fresh49, inlateout("di") c2rust_asm_casts::AsmCast::cast_in(fresh51,
        fresh53) => fresh52, inlateout("ax") 0 as libc::c_int => _,
        options(preserves_flags, att_syntax)
    );
    c2rust_asm_casts::AsmCast::cast_out(fresh48, fresh50, fresh49);
    c2rust_asm_casts::AsmCast::cast_out(fresh51, fresh53, fresh52);
    let mut __d0_2: libc::c_int = 0;
    let mut __d1_2: libc::c_int = 0;
    let fresh54 = &mut __d0_2;
    let fresh55;
    let fresh56 = (::std::mem::size_of::<fd_set>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<__fd_mask>() as libc::c_ulong);
    let fresh57 = &mut __d1_2;
    let fresh58;
    let fresh59 = &mut *(xfds.__fds_bits).as_mut_ptr().offset(0 as libc::c_int as isize)
        as *mut __fd_mask;
    asm!(
        "cld; rep; stosq", inlateout("cx") c2rust_asm_casts::AsmCast::cast_in(fresh54,
        fresh56) => fresh55, inlateout("di") c2rust_asm_casts::AsmCast::cast_in(fresh57,
        fresh59) => fresh58, inlateout("ax") 0 as libc::c_int => _,
        options(preserves_flags, att_syntax)
    );
    c2rust_asm_casts::AsmCast::cast_out(fresh54, fresh56, fresh55);
    c2rust_asm_casts::AsmCast::cast_out(fresh57, fresh59, fresh58);
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong) < nfd {
        if __pth_util_fd_valid((*pfd.offset(i as isize)).fd) == 0 {
            xfds
                .__fds_bits[((*pfd.offset(i as isize)).fd
                / (8 as libc::c_int
                    * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                        as libc::c_int)) as usize]
                |= ((1 as libc::c_ulong)
                    << (*pfd.offset(i as isize)).fd
                        % (8 as libc::c_int
                            * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                as libc::c_int)) as __fd_mask;
        } else {
            if (*pfd.offset(i as isize)).events as libc::c_int
                & (0x1 as libc::c_int | 0x40 as libc::c_int) != 0
            {
                rfds
                    .__fds_bits[((*pfd.offset(i as isize)).fd
                    / (8 as libc::c_int
                        * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                            as libc::c_int)) as usize]
                    |= ((1 as libc::c_ulong)
                        << (*pfd.offset(i as isize)).fd
                            % (8 as libc::c_int
                                * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                    as libc::c_int)) as __fd_mask;
            }
            if (*pfd.offset(i as isize)).events as libc::c_int
                & (0x4 as libc::c_int | 0x100 as libc::c_int | 0x200 as libc::c_int) != 0
            {
                wfds
                    .__fds_bits[((*pfd.offset(i as isize)).fd
                    / (8 as libc::c_int
                        * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                            as libc::c_int)) as usize]
                    |= ((1 as libc::c_ulong)
                        << (*pfd.offset(i as isize)).fd
                            % (8 as libc::c_int
                                * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                    as libc::c_int)) as __fd_mask;
            }
            if (*pfd.offset(i as isize)).events as libc::c_int
                & (0x2 as libc::c_int | 0x80 as libc::c_int) != 0
            {
                efds
                    .__fds_bits[((*pfd.offset(i as isize)).fd
                    / (8 as libc::c_int
                        * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                            as libc::c_int)) as usize]
                    |= ((1 as libc::c_ulong)
                        << (*pfd.offset(i as isize)).fd
                            % (8 as libc::c_int
                                * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                    as libc::c_int)) as __fd_mask;
            }
            if (*pfd.offset(i as isize)).fd >= maxfd
                && (*pfd.offset(i as isize)).events as libc::c_int
                    & (0x1 as libc::c_int | 0x4 as libc::c_int | 0x2 as libc::c_int
                        | 0x40 as libc::c_int | 0x80 as libc::c_int
                        | 0x100 as libc::c_int | 0x200 as libc::c_int) != 0
            {
                maxfd = (*pfd.offset(i as isize)).fd;
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    rc = -(1 as libc::c_int);
    if maxfd != -(1 as libc::c_int) {
        rc = pth_select_ev(
            maxfd + 1 as libc::c_int,
            &mut rfds,
            &mut wfds,
            &mut efds,
            ptv,
            ev_extra,
        );
        if rc < 0 as libc::c_int {
            *__errno_location() = *__errno_location();
            return -(1 as libc::c_int);
        } else if rc == 0 as libc::c_int {
            return 0 as libc::c_int
        }
    }
    n = 0 as libc::c_int;
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong) < nfd {
        (*pfd.offset(i as isize)).revents = 0 as libc::c_int as libc::c_short;
        if xfds
            .__fds_bits[((*pfd.offset(i as isize)).fd
            / (8 as libc::c_int
                * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
            as usize]
            & ((1 as libc::c_ulong)
                << (*pfd.offset(i as isize)).fd
                    % (8 as libc::c_int
                        * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                            as libc::c_int)) as __fd_mask
            != 0 as libc::c_int as libc::c_long
        {
            if (*pfd.offset(i as isize)).fd >= 0 as libc::c_int {
                let ref mut fresh60 = (*pfd.offset(i as isize)).revents;
                *fresh60 = (*fresh60 as libc::c_int | 0x20 as libc::c_int)
                    as libc::c_short;
                n += 1;
                n;
            }
        } else if !(maxfd == -(1 as libc::c_int)) {
            if rfds
                .__fds_bits[((*pfd.offset(i as isize)).fd
                / (8 as libc::c_int
                    * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                        as libc::c_int)) as usize]
                & ((1 as libc::c_ulong)
                    << (*pfd.offset(i as isize)).fd
                        % (8 as libc::c_int
                            * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                as libc::c_int)) as __fd_mask
                != 0 as libc::c_int as libc::c_long
            {
                if (*pfd.offset(i as isize)).events as libc::c_int & 0x1 as libc::c_int
                    != 0
                {
                    let ref mut fresh61 = (*pfd.offset(i as isize)).revents;
                    *fresh61 = (*fresh61 as libc::c_int | 0x1 as libc::c_int)
                        as libc::c_short;
                }
                if (*pfd.offset(i as isize)).events as libc::c_int & 0x40 as libc::c_int
                    != 0
                {
                    let ref mut fresh62 = (*pfd.offset(i as isize)).revents;
                    *fresh62 = (*fresh62 as libc::c_int | 0x40 as libc::c_int)
                        as libc::c_short;
                }
                n += 1;
                n;
                if recv(
                    (*pfd.offset(i as isize)).fd,
                    data.as_mut_ptr() as *mut libc::c_void,
                    ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
                    MSG_PEEK as libc::c_int,
                ) == -(1 as libc::c_int) as libc::c_long
                    && (*__errno_location() == 108 as libc::c_int
                        || *__errno_location() == 104 as libc::c_int
                        || *__errno_location() == 103 as libc::c_int
                        || *__errno_location() == 102 as libc::c_int)
                {
                    let ref mut fresh63 = (*pfd.offset(i as isize)).revents;
                    *fresh63 = (*fresh63 as libc::c_int & !(0x1 as libc::c_int))
                        as libc::c_short;
                    let ref mut fresh64 = (*pfd.offset(i as isize)).revents;
                    *fresh64 = (*fresh64 as libc::c_int & !(0x40 as libc::c_int))
                        as libc::c_short;
                    let ref mut fresh65 = (*pfd.offset(i as isize)).revents;
                    *fresh65 = (*fresh65 as libc::c_int | 0x10 as libc::c_int)
                        as libc::c_short;
                }
            } else if wfds
                .__fds_bits[((*pfd.offset(i as isize)).fd
                / (8 as libc::c_int
                    * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                        as libc::c_int)) as usize]
                & ((1 as libc::c_ulong)
                    << (*pfd.offset(i as isize)).fd
                        % (8 as libc::c_int
                            * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                as libc::c_int)) as __fd_mask
                != 0 as libc::c_int as libc::c_long
            {
                if (*pfd.offset(i as isize)).events as libc::c_int & 0x4 as libc::c_int
                    != 0
                {
                    let ref mut fresh66 = (*pfd.offset(i as isize)).revents;
                    *fresh66 = (*fresh66 as libc::c_int | 0x4 as libc::c_int)
                        as libc::c_short;
                }
                if (*pfd.offset(i as isize)).events as libc::c_int & 0x100 as libc::c_int
                    != 0
                {
                    let ref mut fresh67 = (*pfd.offset(i as isize)).revents;
                    *fresh67 = (*fresh67 as libc::c_int | 0x100 as libc::c_int)
                        as libc::c_short;
                }
                if (*pfd.offset(i as isize)).events as libc::c_int & 0x200 as libc::c_int
                    != 0
                {
                    let ref mut fresh68 = (*pfd.offset(i as isize)).revents;
                    *fresh68 = (*fresh68 as libc::c_int | 0x200 as libc::c_int)
                        as libc::c_short;
                }
                n += 1;
                n;
            } else if efds
                .__fds_bits[((*pfd.offset(i as isize)).fd
                / (8 as libc::c_int
                    * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                        as libc::c_int)) as usize]
                & ((1 as libc::c_ulong)
                    << (*pfd.offset(i as isize)).fd
                        % (8 as libc::c_int
                            * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                as libc::c_int)) as __fd_mask
                != 0 as libc::c_int as libc::c_long
            {
                if (*pfd.offset(i as isize)).events as libc::c_int & 0x2 as libc::c_int
                    != 0
                {
                    let ref mut fresh69 = (*pfd.offset(i as isize)).revents;
                    *fresh69 = (*fresh69 as libc::c_int | 0x2 as libc::c_int)
                        as libc::c_short;
                }
                if (*pfd.offset(i as isize)).events as libc::c_int & 0x80 as libc::c_int
                    != 0
                {
                    let ref mut fresh70 = (*pfd.offset(i as isize)).revents;
                    *fresh70 = (*fresh70 as libc::c_int | 0x80 as libc::c_int)
                        as libc::c_short;
                }
                n += 1;
                n;
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    return n;
}
pub unsafe extern "C" fn pth_connect(
    mut s: libc::c_int,
    mut addr: *const sockaddr,
    mut addrlen: socklen_t,
) -> libc::c_int {
    return pth_connect_ev(s, addr, addrlen, 0 as pth_event_t);
}
pub unsafe extern "C" fn pth_connect_ev(
    mut s: libc::c_int,
    mut addr: *const sockaddr,
    mut addrlen: socklen_t,
    mut ev_extra: pth_event_t,
) -> libc::c_int {
    let mut ev: pth_event_t = 0 as *mut pth_event_st;
    static mut ev_key: pth_key_t = -(1 as libc::c_int);
    let mut rv: libc::c_int = 0;
    let mut err: libc::c_int = 0;
    let mut errlen: socklen_t = 0;
    let mut fdmode: libc::c_int = 0;
    if __pth_initialized == 0 {
        pth_init();
    }
    if __pth_util_fd_valid(s) == 0 {
        *__errno_location() = 9 as libc::c_int;
        return -(1 as libc::c_int);
    }
    fdmode = pth_fdmode(s, PTH_FDMODE_NONBLOCK as libc::c_int);
    if fdmode == PTH_FDMODE_ERROR as libc::c_int {
        *__errno_location() = 9 as libc::c_int;
        return -(1 as libc::c_int);
    }
    loop {
        rv = connect(s, addr as *mut sockaddr, addrlen);
        if !(rv == -(1 as libc::c_int) && *__errno_location() == 4 as libc::c_int) {
            break;
        }
    }
    __pth_errno_storage = *__errno_location();
    __pth_errno_flag = (0 as libc::c_int == 0) as libc::c_int;
    while __pth_errno_flag != 0 {
        pth_fdmode(s, fdmode);
        *__errno_location() = __pth_errno_storage;
        __pth_errno_flag = 0 as libc::c_int;
    }
    if rv == -(1 as libc::c_int) && *__errno_location() == 115 as libc::c_int
        && fdmode != PTH_FDMODE_NONBLOCK as libc::c_int
    {
        ev = pth_event(
            ((1 as libc::c_int) << 1 as libc::c_int
                | (1 as libc::c_int) << 13 as libc::c_int
                | (1 as libc::c_int) << 22 as libc::c_int) as libc::c_ulong,
            &mut ev_key as *mut pth_key_t,
            s,
        );
        if ev.is_null() {
            *__errno_location() = *__errno_location();
            return -(1 as libc::c_int);
        }
        if !ev_extra.is_null() {
            pth_event_concat(ev, ev_extra, 0 as *mut libc::c_void);
        }
        pth_wait(ev);
        if !ev_extra.is_null() {
            pth_event_isolate(ev);
            if pth_event_status(ev) as libc::c_uint
                != PTH_STATUS_OCCURRED as libc::c_int as libc::c_uint
            {
                *__errno_location() = 4 as libc::c_int;
                return -(1 as libc::c_int);
            }
        }
        errlen = ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t;
        if getsockopt(
            s,
            1 as libc::c_int,
            4 as libc::c_int,
            &mut err as *mut libc::c_int as *mut libc::c_void,
            &mut errlen,
        ) == -(1 as libc::c_int)
        {
            return -(1 as libc::c_int);
        }
        if err == 0 as libc::c_int {
            return 0 as libc::c_int;
        }
        *__errno_location() = err;
        return rv;
    }
    return rv;
}
pub unsafe extern "C" fn pth_accept(
    mut s: libc::c_int,
    mut addr: *mut sockaddr,
    mut addrlen: *mut socklen_t,
) -> libc::c_int {
    return pth_accept_ev(s, addr, addrlen, 0 as pth_event_t);
}
pub unsafe extern "C" fn pth_accept_ev(
    mut s: libc::c_int,
    mut addr: *mut sockaddr,
    mut addrlen: *mut socklen_t,
    mut ev_extra: pth_event_t,
) -> libc::c_int {
    let mut ev: pth_event_t = 0 as *mut pth_event_st;
    static mut ev_key: pth_key_t = -(1 as libc::c_int);
    let mut fdmode: libc::c_int = 0;
    let mut rv: libc::c_int = 0;
    if __pth_initialized == 0 {
        pth_init();
    }
    if __pth_util_fd_valid(s) == 0 {
        *__errno_location() = 9 as libc::c_int;
        return -(1 as libc::c_int);
    }
    fdmode = pth_fdmode(s, PTH_FDMODE_NONBLOCK as libc::c_int);
    if fdmode == PTH_FDMODE_ERROR as libc::c_int {
        *__errno_location() = 9 as libc::c_int;
        return -(1 as libc::c_int);
    }
    ev = 0 as pth_event_t;
    loop {
        rv = accept(s, addr, addrlen);
        if !(rv == -(1 as libc::c_int)
            && (*__errno_location() == 11 as libc::c_int
                || *__errno_location() == 11 as libc::c_int)
            && fdmode != PTH_FDMODE_NONBLOCK as libc::c_int)
        {
            break;
        }
        if ev.is_null() {
            ev = pth_event(
                ((1 as libc::c_int) << 1 as libc::c_int
                    | (1 as libc::c_int) << 12 as libc::c_int
                    | (1 as libc::c_int) << 22 as libc::c_int) as libc::c_ulong,
                &mut ev_key as *mut pth_key_t,
                s,
            );
            if ev.is_null() {
                *__errno_location() = *__errno_location();
                return -(1 as libc::c_int);
            }
            if !ev_extra.is_null() {
                pth_event_concat(ev, ev_extra, 0 as *mut libc::c_void);
            }
        }
        pth_wait(ev);
        if !ev_extra.is_null() {
            pth_event_isolate(ev);
            if pth_event_status(ev) as libc::c_uint
                != PTH_STATUS_OCCURRED as libc::c_int as libc::c_uint
            {
                pth_fdmode(s, fdmode);
                *__errno_location() = 4 as libc::c_int;
                return -(1 as libc::c_int);
            }
        }
    }
    __pth_errno_storage = *__errno_location();
    __pth_errno_flag = (0 as libc::c_int == 0) as libc::c_int;
    while __pth_errno_flag != 0 {
        pth_fdmode(s, fdmode);
        if rv != -(1 as libc::c_int) {
            pth_fdmode(rv, fdmode);
        }
        *__errno_location() = __pth_errno_storage;
        __pth_errno_flag = 0 as libc::c_int;
    }
    return rv;
}
pub unsafe extern "C" fn pth_read(
    mut fd: libc::c_int,
    mut buf: *mut libc::c_void,
    mut nbytes: size_t,
) -> ssize_t {
    return pth_read_ev(fd, buf, nbytes, 0 as pth_event_t);
}
pub unsafe extern "C" fn pth_read_ev(
    mut fd: libc::c_int,
    mut buf: *mut libc::c_void,
    mut nbytes: size_t,
    mut ev_extra: pth_event_t,
) -> ssize_t {
    let mut delay: timeval = pth_time_t {
        tv_sec: 0,
        tv_usec: 0,
    };
    let mut ev: pth_event_t = 0 as *mut pth_event_st;
    static mut ev_key: pth_key_t = -(1 as libc::c_int);
    let mut fds: fd_set = fd_set { __fds_bits: [0; 16] };
    let mut fdmode: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    if __pth_initialized == 0 {
        pth_init();
    }
    if nbytes == 0 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int as ssize_t;
    }
    if __pth_util_fd_valid(fd) == 0 {
        *__errno_location() = 9 as libc::c_int;
        return -(1 as libc::c_int) as ssize_t;
    }
    fdmode = pth_fdmode(fd, PTH_FDMODE_POLL as libc::c_int);
    if fdmode == PTH_FDMODE_ERROR as libc::c_int {
        *__errno_location() = 9 as libc::c_int;
        return -(1 as libc::c_int) as ssize_t;
    }
    if fdmode == PTH_FDMODE_BLOCK as libc::c_int {
        let mut __d0: libc::c_int = 0;
        let mut __d1: libc::c_int = 0;
        let fresh71 = &mut __d0;
        let fresh72;
        let fresh73 = (::std::mem::size_of::<fd_set>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<__fd_mask>() as libc::c_ulong);
        let fresh74 = &mut __d1;
        let fresh75;
        let fresh76 = &mut *(fds.__fds_bits)
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize) as *mut __fd_mask;
        asm!(
            "cld; rep; stosq", inlateout("cx")
            c2rust_asm_casts::AsmCast::cast_in(fresh71, fresh73) => fresh72,
            inlateout("di") c2rust_asm_casts::AsmCast::cast_in(fresh74, fresh76) =>
            fresh75, inlateout("ax") 0 as libc::c_int => _, options(preserves_flags,
            att_syntax)
        );
        c2rust_asm_casts::AsmCast::cast_out(fresh71, fresh73, fresh72);
        c2rust_asm_casts::AsmCast::cast_out(fresh74, fresh76, fresh75);
        fds
            .__fds_bits[(fd
            / (8 as libc::c_int
                * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
            as usize]
            |= ((1 as libc::c_ulong)
                << fd
                    % (8 as libc::c_int
                        * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                            as libc::c_int)) as __fd_mask;
        delay.tv_sec = 0 as libc::c_int as __time_t;
        delay.tv_usec = 0 as libc::c_int as __suseconds_t;
        loop {
            n = select(
                fd + 1 as libc::c_int,
                &mut fds,
                0 as *mut fd_set,
                0 as *mut fd_set,
                &mut delay,
            );
            if !(n < 0 as libc::c_int && *__errno_location() == 4 as libc::c_int) {
                break;
            }
        }
        if n < 0 as libc::c_int
            && (*__errno_location() == 22 as libc::c_int
                || *__errno_location() == 9 as libc::c_int)
        {
            *__errno_location() = *__errno_location();
            return -(1 as libc::c_int) as ssize_t;
        }
        if n == 0 as libc::c_int {
            ev = pth_event(
                ((1 as libc::c_int) << 1 as libc::c_int
                    | (1 as libc::c_int) << 12 as libc::c_int
                    | (1 as libc::c_int) << 22 as libc::c_int) as libc::c_ulong,
                &mut ev_key as *mut pth_key_t,
                fd,
            );
            if !ev_extra.is_null() {
                pth_event_concat(ev, ev_extra, 0 as *mut libc::c_void);
            }
            n = pth_wait(ev);
            if !ev_extra.is_null() {
                pth_event_isolate(ev);
                if pth_event_status(ev) as libc::c_uint
                    != PTH_STATUS_OCCURRED as libc::c_int as libc::c_uint
                {
                    *__errno_location() = 4 as libc::c_int;
                    return -(1 as libc::c_int) as ssize_t;
                }
            }
        }
    }
    loop {
        n = read(fd, buf, nbytes) as libc::c_int;
        if !(n < 0 as libc::c_int && *__errno_location() == 4 as libc::c_int) {
            break;
        }
    }
    return n as ssize_t;
}
pub unsafe extern "C" fn pth_write(
    mut fd: libc::c_int,
    mut buf: *const libc::c_void,
    mut nbytes: size_t,
) -> ssize_t {
    return pth_write_ev(fd, buf, nbytes, 0 as pth_event_t);
}
pub unsafe extern "C" fn pth_write_ev(
    mut fd: libc::c_int,
    mut buf: *const libc::c_void,
    mut nbytes: size_t,
    mut ev_extra: pth_event_t,
) -> ssize_t {
    let mut delay: timeval = pth_time_t {
        tv_sec: 0,
        tv_usec: 0,
    };
    let mut ev: pth_event_t = 0 as *mut pth_event_st;
    static mut ev_key: pth_key_t = -(1 as libc::c_int);
    let mut fds: fd_set = fd_set { __fds_bits: [0; 16] };
    let mut fdmode: libc::c_int = 0;
    let mut rv: ssize_t = 0;
    let mut s: ssize_t = 0;
    let mut n: libc::c_int = 0;
    if __pth_initialized == 0 {
        pth_init();
    }
    if nbytes == 0 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int as ssize_t;
    }
    if __pth_util_fd_valid(fd) == 0 {
        *__errno_location() = 9 as libc::c_int;
        return -(1 as libc::c_int) as ssize_t;
    }
    fdmode = pth_fdmode(fd, PTH_FDMODE_NONBLOCK as libc::c_int);
    if fdmode == PTH_FDMODE_ERROR as libc::c_int {
        *__errno_location() = 9 as libc::c_int;
        return -(1 as libc::c_int) as ssize_t;
    }
    if fdmode != PTH_FDMODE_NONBLOCK as libc::c_int {
        let mut __d0: libc::c_int = 0;
        let mut __d1: libc::c_int = 0;
        let fresh77 = &mut __d0;
        let fresh78;
        let fresh79 = (::std::mem::size_of::<fd_set>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<__fd_mask>() as libc::c_ulong);
        let fresh80 = &mut __d1;
        let fresh81;
        let fresh82 = &mut *(fds.__fds_bits)
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize) as *mut __fd_mask;
        asm!(
            "cld; rep; stosq", inlateout("cx")
            c2rust_asm_casts::AsmCast::cast_in(fresh77, fresh79) => fresh78,
            inlateout("di") c2rust_asm_casts::AsmCast::cast_in(fresh80, fresh82) =>
            fresh81, inlateout("ax") 0 as libc::c_int => _, options(preserves_flags,
            att_syntax)
        );
        c2rust_asm_casts::AsmCast::cast_out(fresh77, fresh79, fresh78);
        c2rust_asm_casts::AsmCast::cast_out(fresh80, fresh82, fresh81);
        fds
            .__fds_bits[(fd
            / (8 as libc::c_int
                * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
            as usize]
            |= ((1 as libc::c_ulong)
                << fd
                    % (8 as libc::c_int
                        * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                            as libc::c_int)) as __fd_mask;
        delay.tv_sec = 0 as libc::c_int as __time_t;
        delay.tv_usec = 0 as libc::c_int as __suseconds_t;
        loop {
            n = select(
                fd + 1 as libc::c_int,
                0 as *mut fd_set,
                &mut fds,
                0 as *mut fd_set,
                &mut delay,
            );
            if !(n < 0 as libc::c_int && *__errno_location() == 4 as libc::c_int) {
                break;
            }
        }
        if n < 0 as libc::c_int
            && (*__errno_location() == 22 as libc::c_int
                || *__errno_location() == 9 as libc::c_int)
        {
            *__errno_location() = *__errno_location();
            return -(1 as libc::c_int) as ssize_t;
        }
        rv = 0 as libc::c_int as ssize_t;
        loop {
            if n < 1 as libc::c_int {
                ev = pth_event(
                    ((1 as libc::c_int) << 1 as libc::c_int
                        | (1 as libc::c_int) << 13 as libc::c_int
                        | (1 as libc::c_int) << 22 as libc::c_int) as libc::c_ulong,
                    &mut ev_key as *mut pth_key_t,
                    fd,
                );
                if !ev_extra.is_null() {
                    pth_event_concat(ev, ev_extra, 0 as *mut libc::c_void);
                }
                pth_wait(ev);
                if !ev_extra.is_null() {
                    pth_event_isolate(ev);
                    if pth_event_status(ev) as libc::c_uint
                        != PTH_STATUS_OCCURRED as libc::c_int as libc::c_uint
                    {
                        pth_fdmode(fd, fdmode);
                        *__errno_location() = 4 as libc::c_int;
                        return -(1 as libc::c_int) as ssize_t;
                    }
                }
            }
            loop {
                s = write(fd, buf, nbytes);
                if !(s < 0 as libc::c_int as libc::c_long
                    && *__errno_location() == 4 as libc::c_int)
                {
                    break;
                }
            }
            if s > 0 as libc::c_int as libc::c_long {
                rv += s;
            }
            if s > 0 as libc::c_int as libc::c_long && s < nbytes as ssize_t {
                nbytes = (nbytes as libc::c_ulong).wrapping_sub(s as libc::c_ulong)
                    as size_t as size_t;
                buf = (buf as *mut libc::c_char).offset(s as isize) as *mut libc::c_void;
                n = 0 as libc::c_int;
            } else {
                if s < 0 as libc::c_int as libc::c_long
                    && rv == 0 as libc::c_int as libc::c_long
                {
                    rv = -(1 as libc::c_int) as ssize_t;
                }
                break;
            }
        }
    } else {
        loop {
            rv = write(fd, buf, nbytes);
            if !(rv < 0 as libc::c_int as libc::c_long
                && *__errno_location() == 4 as libc::c_int)
            {
                break;
            }
        }
    }
    __pth_errno_storage = *__errno_location();
    __pth_errno_flag = (0 as libc::c_int == 0) as libc::c_int;
    while __pth_errno_flag != 0 {
        pth_fdmode(fd, fdmode);
        *__errno_location() = __pth_errno_storage;
        __pth_errno_flag = 0 as libc::c_int;
    }
    return rv;
}
pub unsafe extern "C" fn pth_readv(
    mut fd: libc::c_int,
    mut iov: *const iovec,
    mut iovcnt: libc::c_int,
) -> ssize_t {
    return pth_readv_ev(fd, iov, iovcnt, 0 as pth_event_t);
}
pub unsafe extern "C" fn pth_readv_ev(
    mut fd: libc::c_int,
    mut iov: *const iovec,
    mut iovcnt: libc::c_int,
    mut ev_extra: pth_event_t,
) -> ssize_t {
    let mut delay: timeval = pth_time_t {
        tv_sec: 0,
        tv_usec: 0,
    };
    let mut ev: pth_event_t = 0 as *mut pth_event_st;
    static mut ev_key: pth_key_t = -(1 as libc::c_int);
    let mut fds: fd_set = fd_set { __fds_bits: [0; 16] };
    let mut fdmode: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    if __pth_initialized == 0 {
        pth_init();
    }
    if iovcnt <= 0 as libc::c_int || iovcnt > 1024 as libc::c_int {
        *__errno_location() = 22 as libc::c_int;
        return -(1 as libc::c_int) as ssize_t;
    }
    if __pth_util_fd_valid(fd) == 0 {
        *__errno_location() = 9 as libc::c_int;
        return -(1 as libc::c_int) as ssize_t;
    }
    fdmode = pth_fdmode(fd, PTH_FDMODE_POLL as libc::c_int);
    if fdmode == PTH_FDMODE_ERROR as libc::c_int {
        *__errno_location() = 9 as libc::c_int;
        return -(1 as libc::c_int) as ssize_t;
    }
    if fdmode == PTH_FDMODE_BLOCK as libc::c_int {
        let mut __d0: libc::c_int = 0;
        let mut __d1: libc::c_int = 0;
        let fresh83 = &mut __d0;
        let fresh84;
        let fresh85 = (::std::mem::size_of::<fd_set>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<__fd_mask>() as libc::c_ulong);
        let fresh86 = &mut __d1;
        let fresh87;
        let fresh88 = &mut *(fds.__fds_bits)
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize) as *mut __fd_mask;
        asm!(
            "cld; rep; stosq", inlateout("cx")
            c2rust_asm_casts::AsmCast::cast_in(fresh83, fresh85) => fresh84,
            inlateout("di") c2rust_asm_casts::AsmCast::cast_in(fresh86, fresh88) =>
            fresh87, inlateout("ax") 0 as libc::c_int => _, options(preserves_flags,
            att_syntax)
        );
        c2rust_asm_casts::AsmCast::cast_out(fresh83, fresh85, fresh84);
        c2rust_asm_casts::AsmCast::cast_out(fresh86, fresh88, fresh87);
        fds
            .__fds_bits[(fd
            / (8 as libc::c_int
                * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
            as usize]
            |= ((1 as libc::c_ulong)
                << fd
                    % (8 as libc::c_int
                        * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                            as libc::c_int)) as __fd_mask;
        delay.tv_sec = 0 as libc::c_int as __time_t;
        delay.tv_usec = 0 as libc::c_int as __suseconds_t;
        loop {
            n = select(
                fd + 1 as libc::c_int,
                &mut fds,
                0 as *mut fd_set,
                0 as *mut fd_set,
                &mut delay,
            );
            if !(n < 0 as libc::c_int && *__errno_location() == 4 as libc::c_int) {
                break;
            }
        }
        if n < 1 as libc::c_int {
            ev = pth_event(
                ((1 as libc::c_int) << 1 as libc::c_int
                    | (1 as libc::c_int) << 12 as libc::c_int
                    | (1 as libc::c_int) << 22 as libc::c_int) as libc::c_ulong,
                &mut ev_key as *mut pth_key_t,
                fd,
            );
            if !ev_extra.is_null() {
                pth_event_concat(ev, ev_extra, 0 as *mut libc::c_void);
            }
            n = pth_wait(ev);
            if !ev_extra.is_null() {
                pth_event_isolate(ev);
                if pth_event_status(ev) as libc::c_uint
                    != PTH_STATUS_OCCURRED as libc::c_int as libc::c_uint
                {
                    *__errno_location() = 4 as libc::c_int;
                    return -(1 as libc::c_int) as ssize_t;
                }
            }
        }
    }
    loop {
        n = readv(fd, iov, iovcnt) as libc::c_int;
        if !(n < 0 as libc::c_int && *__errno_location() == 4 as libc::c_int) {
            break;
        }
    }
    return n as ssize_t;
}
pub unsafe extern "C" fn __pth_readv_faked(
    mut fd: libc::c_int,
    mut iov: *const iovec,
    mut iovcnt: libc::c_int,
) -> ssize_t {
    let mut buffer: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut bytes: size_t = 0;
    let mut copy: size_t = 0;
    let mut rv: size_t = 0;
    let mut i: libc::c_int = 0;
    bytes = 0 as libc::c_int as size_t;
    i = 0 as libc::c_int;
    while i < iovcnt {
        if (*iov.offset(i as isize)).iov_len <= 0 as libc::c_int as libc::c_ulong {
            *__errno_location() = 22 as libc::c_int;
            return -(1 as libc::c_int) as ssize_t;
        }
        bytes = (bytes as libc::c_ulong).wrapping_add((*iov.offset(i as isize)).iov_len)
            as size_t as size_t;
        i += 1;
        i;
    }
    if bytes <= 0 as libc::c_int as libc::c_ulong {
        *__errno_location() = 22 as libc::c_int;
        return -(1 as libc::c_int) as ssize_t;
    }
    buffer = malloc(bytes) as *mut libc::c_char;
    if buffer.is_null() {
        return -(1 as libc::c_int) as ssize_t;
    }
    rv = read(fd, buffer as *mut libc::c_void, bytes) as size_t;
    if rv > 0 as libc::c_int as libc::c_ulong {
        bytes = rv;
        i = 0 as libc::c_int;
        while i < iovcnt {
            copy = if (*iov.offset(i as isize)).iov_len > bytes {
                bytes
            } else {
                (*iov.offset(i as isize)).iov_len
            };
            memcpy(
                (*iov.offset(i as isize)).iov_base,
                buffer as *const libc::c_void,
                copy,
            );
            buffer = buffer.offset(copy as isize);
            bytes = (bytes as libc::c_ulong).wrapping_sub(copy) as size_t as size_t;
            if bytes <= 0 as libc::c_int as libc::c_ulong {
                break;
            }
            i += 1;
            i;
        }
    }
    __pth_errno_storage = *__errno_location();
    __pth_errno_flag = (0 as libc::c_int == 0) as libc::c_int;
    while __pth_errno_flag != 0 {
        free(buffer as *mut libc::c_void);
        *__errno_location() = __pth_errno_storage;
        __pth_errno_flag = 0 as libc::c_int;
    }
    return rv as ssize_t;
}
pub unsafe extern "C" fn pth_writev(
    mut fd: libc::c_int,
    mut iov: *const iovec,
    mut iovcnt: libc::c_int,
) -> ssize_t {
    return pth_writev_ev(fd, iov, iovcnt, 0 as pth_event_t);
}
pub unsafe extern "C" fn pth_writev_ev(
    mut fd: libc::c_int,
    mut iov: *const iovec,
    mut iovcnt: libc::c_int,
    mut ev_extra: pth_event_t,
) -> ssize_t {
    let mut delay: timeval = pth_time_t {
        tv_sec: 0,
        tv_usec: 0,
    };
    let mut ev: pth_event_t = 0 as *mut pth_event_st;
    static mut ev_key: pth_key_t = -(1 as libc::c_int);
    let mut fds: fd_set = fd_set { __fds_bits: [0; 16] };
    let mut fdmode: libc::c_int = 0;
    let mut liov: *mut iovec = 0 as *mut iovec;
    let mut liovcnt: libc::c_int = 0;
    let mut nbytes: size_t = 0;
    let mut rv: ssize_t = 0;
    let mut s: ssize_t = 0;
    let mut n: libc::c_int = 0;
    let mut tiov_stack: [iovec; 32] = [iovec {
        iov_base: 0 as *mut libc::c_void,
        iov_len: 0,
    }; 32];
    let mut tiov: *mut iovec = 0 as *mut iovec;
    let mut tiovcnt: libc::c_int = 0;
    if __pth_initialized == 0 {
        pth_init();
    }
    if iovcnt <= 0 as libc::c_int || iovcnt > 1024 as libc::c_int {
        *__errno_location() = 22 as libc::c_int;
        return -(1 as libc::c_int) as ssize_t;
    }
    if __pth_util_fd_valid(fd) == 0 {
        *__errno_location() = 9 as libc::c_int;
        return -(1 as libc::c_int) as ssize_t;
    }
    fdmode = pth_fdmode(fd, PTH_FDMODE_NONBLOCK as libc::c_int);
    if fdmode == PTH_FDMODE_ERROR as libc::c_int {
        *__errno_location() = 9 as libc::c_int;
        return -(1 as libc::c_int) as ssize_t;
    }
    if fdmode != PTH_FDMODE_NONBLOCK as libc::c_int {
        if iovcnt as libc::c_ulong
            > ::std::mem::size_of::<[iovec; 32]>() as libc::c_ulong
        {
            tiovcnt = (::std::mem::size_of::<iovec>() as libc::c_ulong)
                .wrapping_mul(1024 as libc::c_int as libc::c_ulong) as libc::c_int;
            tiov = malloc(tiovcnt as libc::c_ulong) as *mut iovec;
            if tiov.is_null() {
                *__errno_location() = *__errno_location();
                return -(1 as libc::c_int) as ssize_t;
            }
        } else {
            tiovcnt = ::std::mem::size_of::<[iovec; 32]>() as libc::c_ulong
                as libc::c_int;
            tiov = tiov_stack.as_mut_ptr();
        }
        rv = 0 as libc::c_int as ssize_t;
        nbytes = __pth_writev_iov_bytes(iov, iovcnt) as size_t;
        liov = 0 as *mut iovec;
        liovcnt = 0 as libc::c_int;
        __pth_writev_iov_advance(
            iov,
            iovcnt,
            0 as libc::c_int as size_t,
            &mut liov,
            &mut liovcnt,
            tiov,
            tiovcnt,
        );
        let mut __d0: libc::c_int = 0;
        let mut __d1: libc::c_int = 0;
        let fresh89 = &mut __d0;
        let fresh90;
        let fresh91 = (::std::mem::size_of::<fd_set>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<__fd_mask>() as libc::c_ulong);
        let fresh92 = &mut __d1;
        let fresh93;
        let fresh94 = &mut *(fds.__fds_bits)
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize) as *mut __fd_mask;
        asm!(
            "cld; rep; stosq", inlateout("cx")
            c2rust_asm_casts::AsmCast::cast_in(fresh89, fresh91) => fresh90,
            inlateout("di") c2rust_asm_casts::AsmCast::cast_in(fresh92, fresh94) =>
            fresh93, inlateout("ax") 0 as libc::c_int => _, options(preserves_flags,
            att_syntax)
        );
        c2rust_asm_casts::AsmCast::cast_out(fresh89, fresh91, fresh90);
        c2rust_asm_casts::AsmCast::cast_out(fresh92, fresh94, fresh93);
        fds
            .__fds_bits[(fd
            / (8 as libc::c_int
                * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
            as usize]
            |= ((1 as libc::c_ulong)
                << fd
                    % (8 as libc::c_int
                        * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                            as libc::c_int)) as __fd_mask;
        delay.tv_sec = 0 as libc::c_int as __time_t;
        delay.tv_usec = 0 as libc::c_int as __suseconds_t;
        loop {
            n = select(
                fd + 1 as libc::c_int,
                0 as *mut fd_set,
                &mut fds,
                0 as *mut fd_set,
                &mut delay,
            );
            if !(n < 0 as libc::c_int && *__errno_location() == 4 as libc::c_int) {
                break;
            }
        }
        loop {
            if n < 1 as libc::c_int {
                ev = pth_event(
                    ((1 as libc::c_int) << 1 as libc::c_int
                        | (1 as libc::c_int) << 13 as libc::c_int
                        | (1 as libc::c_int) << 22 as libc::c_int) as libc::c_ulong,
                    &mut ev_key as *mut pth_key_t,
                    fd,
                );
                if !ev_extra.is_null() {
                    pth_event_concat(ev, ev_extra, 0 as *mut libc::c_void);
                }
                pth_wait(ev);
                if !ev_extra.is_null() {
                    pth_event_isolate(ev);
                    if pth_event_status(ev) as libc::c_uint
                        != PTH_STATUS_OCCURRED as libc::c_int as libc::c_uint
                    {
                        pth_fdmode(fd, fdmode);
                        if iovcnt as libc::c_ulong
                            > ::std::mem::size_of::<[iovec; 32]>() as libc::c_ulong
                        {
                            free(tiov as *mut libc::c_void);
                        }
                        *__errno_location() = 4 as libc::c_int;
                        return -(1 as libc::c_int) as ssize_t;
                    }
                }
            }
            loop {
                s = writev(fd, liov, liovcnt);
                if !(s < 0 as libc::c_int as libc::c_long
                    && *__errno_location() == 4 as libc::c_int)
                {
                    break;
                }
            }
            if s > 0 as libc::c_int as libc::c_long {
                rv += s;
            }
            if s > 0 as libc::c_int as libc::c_long && s < nbytes as ssize_t {
                nbytes = (nbytes as libc::c_ulong).wrapping_sub(s as libc::c_ulong)
                    as size_t as size_t;
                __pth_writev_iov_advance(
                    iov,
                    iovcnt,
                    s as size_t,
                    &mut liov,
                    &mut liovcnt,
                    tiov,
                    tiovcnt,
                );
                n = 0 as libc::c_int;
            } else {
                if s < 0 as libc::c_int as libc::c_long
                    && rv == 0 as libc::c_int as libc::c_long
                {
                    rv = -(1 as libc::c_int) as ssize_t;
                }
                break;
            }
        }
        if iovcnt as libc::c_ulong
            > ::std::mem::size_of::<[iovec; 32]>() as libc::c_ulong
        {
            free(tiov as *mut libc::c_void);
        }
    } else {
        loop {
            rv = writev(fd, iov, iovcnt);
            if !(rv < 0 as libc::c_int as libc::c_long
                && *__errno_location() == 4 as libc::c_int)
            {
                break;
            }
        }
    }
    __pth_errno_storage = *__errno_location();
    __pth_errno_flag = (0 as libc::c_int == 0) as libc::c_int;
    while __pth_errno_flag != 0 {
        pth_fdmode(fd, fdmode);
        *__errno_location() = __pth_errno_storage;
        __pth_errno_flag = 0 as libc::c_int;
    }
    return rv;
}
pub unsafe extern "C" fn __pth_writev_iov_bytes(
    mut iov: *const iovec,
    mut iovcnt: libc::c_int,
) -> ssize_t {
    let mut bytes: ssize_t = 0;
    let mut i: libc::c_int = 0;
    bytes = 0 as libc::c_int as ssize_t;
    i = 0 as libc::c_int;
    while i < iovcnt {
        if !((*iov.offset(i as isize)).iov_len <= 0 as libc::c_int as libc::c_ulong) {
            bytes = (bytes as libc::c_ulong)
                .wrapping_add((*iov.offset(i as isize)).iov_len) as ssize_t as ssize_t;
        }
        i += 1;
        i;
    }
    return bytes;
}
pub unsafe extern "C" fn __pth_writev_iov_advance(
    mut riov: *const iovec,
    mut riovcnt: libc::c_int,
    mut advance: size_t,
    mut liov: *mut *mut iovec,
    mut liovcnt: *mut libc::c_int,
    mut tiov: *mut iovec,
    mut tiovcnt: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    if (*liov).is_null() && *liovcnt == 0 as libc::c_int {
        *liov = riov as *mut iovec;
        *liovcnt = riovcnt;
    }
    if advance > 0 as libc::c_int as libc::c_ulong {
        if *liov == riov as *mut iovec && *liovcnt == riovcnt {
            *liov = &mut *tiov.offset(0 as libc::c_int as isize) as *mut iovec;
            i = 0 as libc::c_int;
            while i < riovcnt {
                let ref mut fresh95 = (*tiov.offset(i as isize)).iov_base;
                *fresh95 = (*riov.offset(i as isize)).iov_base;
                (*tiov.offset(i as isize)).iov_len = (*riov.offset(i as isize)).iov_len;
                i += 1;
                i;
            }
        }
        while *liovcnt > 0 as libc::c_int && advance > 0 as libc::c_int as libc::c_ulong
        {
            if (**liov).iov_len > advance {
                (**liov)
                    .iov_base = ((**liov).iov_base as *mut libc::c_char)
                    .offset(advance as isize) as *mut libc::c_void;
                (**liov)
                    .iov_len = ((**liov).iov_len as libc::c_ulong).wrapping_sub(advance)
                    as size_t as size_t;
                break;
            } else {
                advance = (advance as libc::c_ulong).wrapping_sub((**liov).iov_len)
                    as size_t as size_t;
                *liovcnt -= 1;
                *liovcnt;
                *liov = (*liov).offset(1);
                *liov;
            }
        }
    }
}
pub unsafe extern "C" fn __pth_writev_faked(
    mut fd: libc::c_int,
    mut iov: *const iovec,
    mut iovcnt: libc::c_int,
) -> ssize_t {
    let mut buffer: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut bytes: size_t = 0;
    let mut to_copy: size_t = 0;
    let mut copy: size_t = 0;
    let mut rv: size_t = 0;
    let mut i: libc::c_int = 0;
    bytes = 0 as libc::c_int as size_t;
    i = 0 as libc::c_int;
    while i < iovcnt {
        if (*iov.offset(i as isize)).iov_len <= 0 as libc::c_int as libc::c_ulong {
            *__errno_location() = 22 as libc::c_int;
            return -(1 as libc::c_int) as ssize_t;
        }
        bytes = (bytes as libc::c_ulong).wrapping_add((*iov.offset(i as isize)).iov_len)
            as size_t as size_t;
        i += 1;
        i;
    }
    if bytes <= 0 as libc::c_int as libc::c_ulong {
        *__errno_location() = 22 as libc::c_int;
        return -(1 as libc::c_int) as ssize_t;
    }
    buffer = malloc(bytes) as *mut libc::c_char;
    if buffer.is_null() {
        return -(1 as libc::c_int) as ssize_t;
    }
    to_copy = bytes;
    cp = buffer;
    i = 0 as libc::c_int;
    while i < iovcnt {
        copy = if (*iov.offset(i as isize)).iov_len > to_copy {
            to_copy
        } else {
            (*iov.offset(i as isize)).iov_len
        };
        memcpy(cp as *mut libc::c_void, (*iov.offset(i as isize)).iov_base, copy);
        to_copy = (to_copy as libc::c_ulong).wrapping_sub(copy) as size_t as size_t;
        if to_copy <= 0 as libc::c_int as libc::c_ulong {
            break;
        }
        i += 1;
        i;
    }
    rv = write(fd, buffer as *const libc::c_void, bytes) as size_t;
    __pth_errno_storage = *__errno_location();
    __pth_errno_flag = (0 as libc::c_int == 0) as libc::c_int;
    while __pth_errno_flag != 0 {
        free(buffer as *mut libc::c_void);
        *__errno_location() = __pth_errno_storage;
        __pth_errno_flag = 0 as libc::c_int;
    }
    return rv as ssize_t;
}
pub unsafe extern "C" fn pth_pread(
    mut fd: libc::c_int,
    mut buf: *mut libc::c_void,
    mut nbytes: size_t,
    mut offset: off_t,
) -> ssize_t {
    static mut mutex: pth_mutex_t = {
        let mut init = pth_mutex_st {
            mx_node: {
                let mut init = pth_ringnode_st {
                    rn_next: 0 as *const pth_ringnode_t as *mut pth_ringnode_t,
                    rn_prev: 0 as *const pth_ringnode_t as *mut pth_ringnode_t,
                };
                init
            },
            mx_state: (1 as libc::c_int) << 0 as libc::c_int,
            mx_owner: 0 as *const pth_st as pth_t,
            mx_count: 0 as libc::c_int as libc::c_ulong,
        };
        init
    };
    let mut old_offset: off_t = 0;
    let mut rc: ssize_t = 0;
    if pth_mutex_acquire(&mut mutex, 0 as libc::c_int, 0 as pth_event_t) == 0 {
        return -(1 as libc::c_int) as ssize_t;
    }
    old_offset = lseek(fd, 0 as libc::c_int as __off_t, 1 as libc::c_int);
    if old_offset == -(1 as libc::c_int) as off_t {
        pth_mutex_release(&mut mutex);
        return -(1 as libc::c_int) as ssize_t;
    }
    if lseek(fd, offset, 0 as libc::c_int) == -(1 as libc::c_int) as off_t {
        pth_mutex_release(&mut mutex);
        return -(1 as libc::c_int) as ssize_t;
    }
    rc = pth_read(fd, buf, nbytes);
    __pth_errno_storage = *__errno_location();
    __pth_errno_flag = (0 as libc::c_int == 0) as libc::c_int;
    while __pth_errno_flag != 0 {
        lseek(fd, old_offset, 0 as libc::c_int);
        *__errno_location() = __pth_errno_storage;
        __pth_errno_flag = 0 as libc::c_int;
    }
    pth_mutex_release(&mut mutex);
    return rc;
}
pub unsafe extern "C" fn pth_pwrite(
    mut fd: libc::c_int,
    mut buf: *const libc::c_void,
    mut nbytes: size_t,
    mut offset: off_t,
) -> ssize_t {
    static mut mutex: pth_mutex_t = {
        let mut init = pth_mutex_st {
            mx_node: {
                let mut init = pth_ringnode_st {
                    rn_next: 0 as *const pth_ringnode_t as *mut pth_ringnode_t,
                    rn_prev: 0 as *const pth_ringnode_t as *mut pth_ringnode_t,
                };
                init
            },
            mx_state: (1 as libc::c_int) << 0 as libc::c_int,
            mx_owner: 0 as *const pth_st as pth_t,
            mx_count: 0 as libc::c_int as libc::c_ulong,
        };
        init
    };
    let mut old_offset: off_t = 0;
    let mut rc: ssize_t = 0;
    if pth_mutex_acquire(&mut mutex, 0 as libc::c_int, 0 as pth_event_t) == 0 {
        return -(1 as libc::c_int) as ssize_t;
    }
    old_offset = lseek(fd, 0 as libc::c_int as __off_t, 1 as libc::c_int);
    if old_offset == -(1 as libc::c_int) as off_t {
        pth_mutex_release(&mut mutex);
        return -(1 as libc::c_int) as ssize_t;
    }
    if lseek(fd, offset, 0 as libc::c_int) == -(1 as libc::c_int) as off_t {
        pth_mutex_release(&mut mutex);
        return -(1 as libc::c_int) as ssize_t;
    }
    rc = pth_write(fd, buf, nbytes);
    __pth_errno_storage = *__errno_location();
    __pth_errno_flag = (0 as libc::c_int == 0) as libc::c_int;
    while __pth_errno_flag != 0 {
        lseek(fd, old_offset, 0 as libc::c_int);
        *__errno_location() = __pth_errno_storage;
        __pth_errno_flag = 0 as libc::c_int;
    }
    pth_mutex_release(&mut mutex);
    return rc;
}
pub unsafe extern "C" fn pth_recv(
    mut s: libc::c_int,
    mut buf: *mut libc::c_void,
    mut len: size_t,
    mut flags: libc::c_int,
) -> ssize_t {
    return pth_recv_ev(s, buf, len, flags, 0 as pth_event_t);
}
pub unsafe extern "C" fn pth_recv_ev(
    mut s: libc::c_int,
    mut buf: *mut libc::c_void,
    mut len: size_t,
    mut flags: libc::c_int,
    mut ev: pth_event_t,
) -> ssize_t {
    return pth_recvfrom_ev(
        s,
        buf,
        len,
        flags,
        0 as *mut sockaddr,
        0 as *mut socklen_t,
        ev,
    );
}
pub unsafe extern "C" fn pth_recvfrom(
    mut s: libc::c_int,
    mut buf: *mut libc::c_void,
    mut len: size_t,
    mut flags: libc::c_int,
    mut from: *mut sockaddr,
    mut fromlen: *mut socklen_t,
) -> ssize_t {
    return pth_recvfrom_ev(s, buf, len, flags, from, fromlen, 0 as pth_event_t);
}
pub unsafe extern "C" fn pth_recvfrom_ev(
    mut fd: libc::c_int,
    mut buf: *mut libc::c_void,
    mut nbytes: size_t,
    mut flags: libc::c_int,
    mut from: *mut sockaddr,
    mut fromlen: *mut socklen_t,
    mut ev_extra: pth_event_t,
) -> ssize_t {
    let mut delay: timeval = pth_time_t {
        tv_sec: 0,
        tv_usec: 0,
    };
    let mut ev: pth_event_t = 0 as *mut pth_event_st;
    static mut ev_key: pth_key_t = -(1 as libc::c_int);
    let mut fds: fd_set = fd_set { __fds_bits: [0; 16] };
    let mut fdmode: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    if __pth_initialized == 0 {
        pth_init();
    }
    if nbytes == 0 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int as ssize_t;
    }
    if __pth_util_fd_valid(fd) == 0 {
        *__errno_location() = 9 as libc::c_int;
        return -(1 as libc::c_int) as ssize_t;
    }
    fdmode = pth_fdmode(fd, PTH_FDMODE_POLL as libc::c_int);
    if fdmode == PTH_FDMODE_ERROR as libc::c_int {
        *__errno_location() = 9 as libc::c_int;
        return -(1 as libc::c_int) as ssize_t;
    }
    if fdmode == PTH_FDMODE_BLOCK as libc::c_int {
        if __pth_util_fd_valid(fd) == 0 {
            *__errno_location() = 9 as libc::c_int;
            return -(1 as libc::c_int) as ssize_t;
        }
        let mut __d0: libc::c_int = 0;
        let mut __d1: libc::c_int = 0;
        let fresh96 = &mut __d0;
        let fresh97;
        let fresh98 = (::std::mem::size_of::<fd_set>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<__fd_mask>() as libc::c_ulong);
        let fresh99 = &mut __d1;
        let fresh100;
        let fresh101 = &mut *(fds.__fds_bits)
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize) as *mut __fd_mask;
        asm!(
            "cld; rep; stosq", inlateout("cx")
            c2rust_asm_casts::AsmCast::cast_in(fresh96, fresh98) => fresh97,
            inlateout("di") c2rust_asm_casts::AsmCast::cast_in(fresh99, fresh101) =>
            fresh100, inlateout("ax") 0 as libc::c_int => _, options(preserves_flags,
            att_syntax)
        );
        c2rust_asm_casts::AsmCast::cast_out(fresh96, fresh98, fresh97);
        c2rust_asm_casts::AsmCast::cast_out(fresh99, fresh101, fresh100);
        fds
            .__fds_bits[(fd
            / (8 as libc::c_int
                * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
            as usize]
            |= ((1 as libc::c_ulong)
                << fd
                    % (8 as libc::c_int
                        * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                            as libc::c_int)) as __fd_mask;
        delay.tv_sec = 0 as libc::c_int as __time_t;
        delay.tv_usec = 0 as libc::c_int as __suseconds_t;
        loop {
            n = select(
                fd + 1 as libc::c_int,
                &mut fds,
                0 as *mut fd_set,
                0 as *mut fd_set,
                &mut delay,
            );
            if !(n < 0 as libc::c_int && *__errno_location() == 4 as libc::c_int) {
                break;
            }
        }
        if n < 0 as libc::c_int
            && (*__errno_location() == 22 as libc::c_int
                || *__errno_location() == 9 as libc::c_int)
        {
            *__errno_location() = *__errno_location();
            return -(1 as libc::c_int) as ssize_t;
        }
        if n == 0 as libc::c_int {
            ev = pth_event(
                ((1 as libc::c_int) << 1 as libc::c_int
                    | (1 as libc::c_int) << 12 as libc::c_int
                    | (1 as libc::c_int) << 22 as libc::c_int) as libc::c_ulong,
                &mut ev_key as *mut pth_key_t,
                fd,
            );
            if !ev_extra.is_null() {
                pth_event_concat(ev, ev_extra, 0 as *mut libc::c_void);
            }
            n = pth_wait(ev);
            if !ev_extra.is_null() {
                pth_event_isolate(ev);
                if pth_event_status(ev) as libc::c_uint
                    != PTH_STATUS_OCCURRED as libc::c_int as libc::c_uint
                {
                    *__errno_location() = 4 as libc::c_int;
                    return -(1 as libc::c_int) as ssize_t;
                }
            }
        }
    }
    loop {
        n = recvfrom(fd, buf, nbytes, flags, from, fromlen) as libc::c_int;
        if !(n < 0 as libc::c_int && *__errno_location() == 4 as libc::c_int) {
            break;
        }
    }
    return n as ssize_t;
}
pub unsafe extern "C" fn pth_send(
    mut s: libc::c_int,
    mut buf: *const libc::c_void,
    mut len: size_t,
    mut flags: libc::c_int,
) -> ssize_t {
    return pth_send_ev(s, buf, len, flags, 0 as pth_event_t);
}
pub unsafe extern "C" fn pth_send_ev(
    mut s: libc::c_int,
    mut buf: *const libc::c_void,
    mut len: size_t,
    mut flags: libc::c_int,
    mut ev: pth_event_t,
) -> ssize_t {
    return pth_sendto_ev(
        s,
        buf,
        len,
        flags,
        0 as *const sockaddr,
        0 as libc::c_int as socklen_t,
        ev,
    );
}
pub unsafe extern "C" fn pth_sendto(
    mut s: libc::c_int,
    mut buf: *const libc::c_void,
    mut len: size_t,
    mut flags: libc::c_int,
    mut to: *const sockaddr,
    mut tolen: socklen_t,
) -> ssize_t {
    return pth_sendto_ev(s, buf, len, flags, to, tolen, 0 as pth_event_t);
}
pub unsafe extern "C" fn pth_sendto_ev(
    mut fd: libc::c_int,
    mut buf: *const libc::c_void,
    mut nbytes: size_t,
    mut flags: libc::c_int,
    mut to: *const sockaddr,
    mut tolen: socklen_t,
    mut ev_extra: pth_event_t,
) -> ssize_t {
    let mut delay: timeval = pth_time_t {
        tv_sec: 0,
        tv_usec: 0,
    };
    let mut ev: pth_event_t = 0 as *mut pth_event_st;
    static mut ev_key: pth_key_t = -(1 as libc::c_int);
    let mut fds: fd_set = fd_set { __fds_bits: [0; 16] };
    let mut fdmode: libc::c_int = 0;
    let mut rv: ssize_t = 0;
    let mut s: ssize_t = 0;
    let mut n: libc::c_int = 0;
    if __pth_initialized == 0 {
        pth_init();
    }
    if nbytes == 0 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int as ssize_t;
    }
    if __pth_util_fd_valid(fd) == 0 {
        *__errno_location() = 9 as libc::c_int;
        return -(1 as libc::c_int) as ssize_t;
    }
    fdmode = pth_fdmode(fd, PTH_FDMODE_NONBLOCK as libc::c_int);
    if fdmode == PTH_FDMODE_ERROR as libc::c_int {
        *__errno_location() = 9 as libc::c_int;
        return -(1 as libc::c_int) as ssize_t;
    }
    if fdmode != PTH_FDMODE_NONBLOCK as libc::c_int {
        if __pth_util_fd_valid(fd) == 0 {
            pth_fdmode(fd, fdmode);
            *__errno_location() = 9 as libc::c_int;
            return -(1 as libc::c_int) as ssize_t;
        }
        let mut __d0: libc::c_int = 0;
        let mut __d1: libc::c_int = 0;
        let fresh102 = &mut __d0;
        let fresh103;
        let fresh104 = (::std::mem::size_of::<fd_set>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<__fd_mask>() as libc::c_ulong);
        let fresh105 = &mut __d1;
        let fresh106;
        let fresh107 = &mut *(fds.__fds_bits)
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize) as *mut __fd_mask;
        asm!(
            "cld; rep; stosq", inlateout("cx")
            c2rust_asm_casts::AsmCast::cast_in(fresh102, fresh104) => fresh103,
            inlateout("di") c2rust_asm_casts::AsmCast::cast_in(fresh105, fresh107) =>
            fresh106, inlateout("ax") 0 as libc::c_int => _, options(preserves_flags,
            att_syntax)
        );
        c2rust_asm_casts::AsmCast::cast_out(fresh102, fresh104, fresh103);
        c2rust_asm_casts::AsmCast::cast_out(fresh105, fresh107, fresh106);
        fds
            .__fds_bits[(fd
            / (8 as libc::c_int
                * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
            as usize]
            |= ((1 as libc::c_ulong)
                << fd
                    % (8 as libc::c_int
                        * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                            as libc::c_int)) as __fd_mask;
        delay.tv_sec = 0 as libc::c_int as __time_t;
        delay.tv_usec = 0 as libc::c_int as __suseconds_t;
        loop {
            n = select(
                fd + 1 as libc::c_int,
                0 as *mut fd_set,
                &mut fds,
                0 as *mut fd_set,
                &mut delay,
            );
            if !(n < 0 as libc::c_int && *__errno_location() == 4 as libc::c_int) {
                break;
            }
        }
        if n < 0 as libc::c_int
            && (*__errno_location() == 22 as libc::c_int
                || *__errno_location() == 9 as libc::c_int)
        {
            *__errno_location() = *__errno_location();
            return -(1 as libc::c_int) as ssize_t;
        }
        rv = 0 as libc::c_int as ssize_t;
        loop {
            if n == 0 as libc::c_int {
                ev = pth_event(
                    ((1 as libc::c_int) << 1 as libc::c_int
                        | (1 as libc::c_int) << 13 as libc::c_int
                        | (1 as libc::c_int) << 22 as libc::c_int) as libc::c_ulong,
                    &mut ev_key as *mut pth_key_t,
                    fd,
                );
                if !ev_extra.is_null() {
                    pth_event_concat(ev, ev_extra, 0 as *mut libc::c_void);
                }
                pth_wait(ev);
                if !ev_extra.is_null() {
                    pth_event_isolate(ev);
                    if pth_event_status(ev) as libc::c_uint
                        != PTH_STATUS_OCCURRED as libc::c_int as libc::c_uint
                    {
                        pth_fdmode(fd, fdmode);
                        *__errno_location() = 4 as libc::c_int;
                        return -(1 as libc::c_int) as ssize_t;
                    }
                }
            }
            loop {
                s = sendto(fd, buf, nbytes, flags, to, tolen);
                if !(s < 0 as libc::c_int as libc::c_long
                    && *__errno_location() == 4 as libc::c_int)
                {
                    break;
                }
            }
            if s > 0 as libc::c_int as libc::c_long {
                rv += s;
            }
            if s > 0 as libc::c_int as libc::c_long && s < nbytes as ssize_t {
                nbytes = (nbytes as libc::c_ulong).wrapping_sub(s as libc::c_ulong)
                    as size_t as size_t;
                buf = (buf as *mut libc::c_char).offset(s as isize) as *mut libc::c_void;
                n = 0 as libc::c_int;
            } else {
                if s < 0 as libc::c_int as libc::c_long
                    && rv == 0 as libc::c_int as libc::c_long
                {
                    rv = -(1 as libc::c_int) as ssize_t;
                }
                break;
            }
        }
    } else {
        loop {
            rv = sendto(fd, buf, nbytes, flags, to, tolen);
            if !(rv < 0 as libc::c_int as libc::c_long
                && *__errno_location() == 4 as libc::c_int)
            {
                break;
            }
        }
    }
    __pth_errno_storage = *__errno_location();
    __pth_errno_flag = (0 as libc::c_int == 0) as libc::c_int;
    while __pth_errno_flag != 0 {
        pth_fdmode(fd, fdmode);
        *__errno_location() = __pth_errno_storage;
        __pth_errno_flag = 0 as libc::c_int;
    }
    return rv;
}
