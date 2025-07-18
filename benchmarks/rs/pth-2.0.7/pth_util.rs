use ::libc;
extern "C" {
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
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __uint32_t = libc::c_uint;
pub type __uid_t = libc::c_uint;
pub type __pid_t = libc::c_int;
pub type __clock_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
pub type sigset_t = __sigset_t;
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
unsafe extern "C" fn pth_util_sigdelete_sighandler(mut _sig: libc::c_int) {}
pub unsafe extern "C" fn __pth_util_sigdelete(mut sig: libc::c_int) -> libc::c_int {
    let mut ss: sigset_t = sigset_t { __val: [0; 16] };
    let mut oss: sigset_t = sigset_t { __val: [0; 16] };
    let mut sa: sigaction = sigaction {
        __sigaction_handler: C2RustUnnamed_9 {
            sa_handler: None,
        },
        sa_mask: sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    let mut osa: sigaction = sigaction {
        __sigaction_handler: C2RustUnnamed_9 {
            sa_handler: None,
        },
        sa_mask: sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    sigpending(&mut ss);
    if sigismember(&mut ss, sig) == 0 {
        return 0 as libc::c_int;
    }
    sigemptyset(&mut ss);
    sigaddset(&mut ss, sig);
    sigprocmask(0 as libc::c_int, &mut ss, &mut oss);
    sa
        .__sigaction_handler
        .sa_handler = Some(
        pth_util_sigdelete_sighandler as unsafe extern "C" fn(libc::c_int) -> (),
    );
    sigfillset(&mut sa.sa_mask);
    sa.sa_flags = 0 as libc::c_int;
    if sigaction(sig, &mut sa, &mut osa) != 0 as libc::c_int {
        sigprocmask(2 as libc::c_int, &mut oss, 0 as *mut sigset_t);
        return 0 as libc::c_int;
    }
    sigfillset(&mut ss);
    sigdelset(&mut ss, sig);
    sigsuspend(&mut ss);
    sigaction(sig, &mut osa, 0 as *mut sigaction);
    sigprocmask(2 as libc::c_int, &mut oss, 0 as *mut sigset_t);
    return (0 as libc::c_int == 0) as libc::c_int;
}
pub unsafe extern "C" fn __pth_util_cpystrn(
    mut dst: *mut libc::c_char,
    mut src: *const libc::c_char,
    mut dst_size: size_t,
) -> *mut libc::c_char {
    let mut d: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
    if dst_size == 0 as libc::c_int as libc::c_ulong {
        return dst;
    }
    d = dst;
    end = dst.offset(dst_size as isize).offset(-(1 as libc::c_int as isize));
    while d < end {
        *d = *src;
        if *d as libc::c_int == '\0' as i32 {
            return d;
        }
        d = d.offset(1);
        d;
        src = src.offset(1);
        src;
    }
    *d = '\0' as i32 as libc::c_char;
    return d;
}
pub unsafe extern "C" fn __pth_util_fd_valid(mut fd: libc::c_int) -> libc::c_int {
    if fd < 0 as libc::c_int || fd >= 1024 as libc::c_int {
        return 0 as libc::c_int;
    }
    if fcntl(fd, 3 as libc::c_int) == -(1 as libc::c_int)
        && *__errno_location() == 9 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    return (0 as libc::c_int == 0) as libc::c_int;
}
pub unsafe extern "C" fn __pth_util_fds_merge(
    mut nfd: libc::c_int,
    mut ifds1: *mut fd_set,
    mut ofds1: *mut fd_set,
    mut ifds2: *mut fd_set,
    mut ofds2: *mut fd_set,
    mut ifds3: *mut fd_set,
    mut ofds3: *mut fd_set,
) {
    let mut s: libc::c_int = 0;
    s = 0 as libc::c_int;
    while s < nfd {
        if !ifds1.is_null() {
            if (*ifds1)
                .__fds_bits[(s
                / (8 as libc::c_int
                    * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                        as libc::c_int)) as usize]
                & ((1 as libc::c_ulong)
                    << s
                        % (8 as libc::c_int
                            * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                as libc::c_int)) as __fd_mask
                != 0 as libc::c_int as libc::c_long
            {
                (*ofds1)
                    .__fds_bits[(s
                    / (8 as libc::c_int
                        * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                            as libc::c_int)) as usize]
                    |= ((1 as libc::c_ulong)
                        << s
                            % (8 as libc::c_int
                                * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                    as libc::c_int)) as __fd_mask;
            }
        }
        if !ifds2.is_null() {
            if (*ifds2)
                .__fds_bits[(s
                / (8 as libc::c_int
                    * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                        as libc::c_int)) as usize]
                & ((1 as libc::c_ulong)
                    << s
                        % (8 as libc::c_int
                            * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                as libc::c_int)) as __fd_mask
                != 0 as libc::c_int as libc::c_long
            {
                (*ofds2)
                    .__fds_bits[(s
                    / (8 as libc::c_int
                        * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                            as libc::c_int)) as usize]
                    |= ((1 as libc::c_ulong)
                        << s
                            % (8 as libc::c_int
                                * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                    as libc::c_int)) as __fd_mask;
            }
        }
        if !ifds3.is_null() {
            if (*ifds3)
                .__fds_bits[(s
                / (8 as libc::c_int
                    * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                        as libc::c_int)) as usize]
                & ((1 as libc::c_ulong)
                    << s
                        % (8 as libc::c_int
                            * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                as libc::c_int)) as __fd_mask
                != 0 as libc::c_int as libc::c_long
            {
                (*ofds3)
                    .__fds_bits[(s
                    / (8 as libc::c_int
                        * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                            as libc::c_int)) as usize]
                    |= ((1 as libc::c_ulong)
                        << s
                            % (8 as libc::c_int
                                * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                    as libc::c_int)) as __fd_mask;
            }
        }
        s += 1;
        s;
    }
}
pub unsafe extern "C" fn __pth_util_fds_test(
    mut nfd: libc::c_int,
    mut ifds1: *mut fd_set,
    mut ofds1: *mut fd_set,
    mut ifds2: *mut fd_set,
    mut ofds2: *mut fd_set,
    mut ifds3: *mut fd_set,
    mut ofds3: *mut fd_set,
) -> libc::c_int {
    let mut s: libc::c_int = 0;
    s = 0 as libc::c_int;
    while s < nfd {
        if !ifds1.is_null() {
            if (*ifds1)
                .__fds_bits[(s
                / (8 as libc::c_int
                    * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                        as libc::c_int)) as usize]
                & ((1 as libc::c_ulong)
                    << s
                        % (8 as libc::c_int
                            * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                as libc::c_int)) as __fd_mask
                != 0 as libc::c_int as libc::c_long
                && (*ofds1)
                    .__fds_bits[(s
                    / (8 as libc::c_int
                        * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                            as libc::c_int)) as usize]
                    & ((1 as libc::c_ulong)
                        << s
                            % (8 as libc::c_int
                                * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                    as libc::c_int)) as __fd_mask
                    != 0 as libc::c_int as libc::c_long
            {
                return (0 as libc::c_int == 0) as libc::c_int;
            }
        }
        if !ifds2.is_null() {
            if (*ifds2)
                .__fds_bits[(s
                / (8 as libc::c_int
                    * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                        as libc::c_int)) as usize]
                & ((1 as libc::c_ulong)
                    << s
                        % (8 as libc::c_int
                            * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                as libc::c_int)) as __fd_mask
                != 0 as libc::c_int as libc::c_long
                && (*ofds2)
                    .__fds_bits[(s
                    / (8 as libc::c_int
                        * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                            as libc::c_int)) as usize]
                    & ((1 as libc::c_ulong)
                        << s
                            % (8 as libc::c_int
                                * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                    as libc::c_int)) as __fd_mask
                    != 0 as libc::c_int as libc::c_long
            {
                return (0 as libc::c_int == 0) as libc::c_int;
            }
        }
        if !ifds3.is_null() {
            if (*ifds3)
                .__fds_bits[(s
                / (8 as libc::c_int
                    * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                        as libc::c_int)) as usize]
                & ((1 as libc::c_ulong)
                    << s
                        % (8 as libc::c_int
                            * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                as libc::c_int)) as __fd_mask
                != 0 as libc::c_int as libc::c_long
                && (*ofds3)
                    .__fds_bits[(s
                    / (8 as libc::c_int
                        * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                            as libc::c_int)) as usize]
                    & ((1 as libc::c_ulong)
                        << s
                            % (8 as libc::c_int
                                * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                    as libc::c_int)) as __fd_mask
                    != 0 as libc::c_int as libc::c_long
            {
                return (0 as libc::c_int == 0) as libc::c_int;
            }
        }
        s += 1;
        s;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn __pth_util_fds_select(
    mut nfd: libc::c_int,
    mut ifds1: *mut fd_set,
    mut ofds1: *mut fd_set,
    mut ifds2: *mut fd_set,
    mut ofds2: *mut fd_set,
    mut ifds3: *mut fd_set,
    mut ofds3: *mut fd_set,
) -> libc::c_int {
    let mut s: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    n = 0 as libc::c_int;
    s = 0 as libc::c_int;
    while s < nfd {
        if !ifds1.is_null()
            && (*ifds1)
                .__fds_bits[(s
                / (8 as libc::c_int
                    * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                        as libc::c_int)) as usize]
                & ((1 as libc::c_ulong)
                    << s
                        % (8 as libc::c_int
                            * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                as libc::c_int)) as __fd_mask
                != 0 as libc::c_int as libc::c_long
        {
            if !((*ofds1)
                .__fds_bits[(s
                / (8 as libc::c_int
                    * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                        as libc::c_int)) as usize]
                & ((1 as libc::c_ulong)
                    << s
                        % (8 as libc::c_int
                            * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                as libc::c_int)) as __fd_mask
                != 0 as libc::c_int as libc::c_long)
            {
                (*ifds1)
                    .__fds_bits[(s
                    / (8 as libc::c_int
                        * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                            as libc::c_int)) as usize]
                    &= !(((1 as libc::c_ulong)
                        << s
                            % (8 as libc::c_int
                                * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                    as libc::c_int)) as __fd_mask);
            } else {
                n += 1;
                n;
            }
        }
        if !ifds2.is_null()
            && (*ifds2)
                .__fds_bits[(s
                / (8 as libc::c_int
                    * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                        as libc::c_int)) as usize]
                & ((1 as libc::c_ulong)
                    << s
                        % (8 as libc::c_int
                            * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                as libc::c_int)) as __fd_mask
                != 0 as libc::c_int as libc::c_long
        {
            if !((*ofds2)
                .__fds_bits[(s
                / (8 as libc::c_int
                    * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                        as libc::c_int)) as usize]
                & ((1 as libc::c_ulong)
                    << s
                        % (8 as libc::c_int
                            * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                as libc::c_int)) as __fd_mask
                != 0 as libc::c_int as libc::c_long)
            {
                (*ifds2)
                    .__fds_bits[(s
                    / (8 as libc::c_int
                        * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                            as libc::c_int)) as usize]
                    &= !(((1 as libc::c_ulong)
                        << s
                            % (8 as libc::c_int
                                * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                    as libc::c_int)) as __fd_mask);
            } else {
                n += 1;
                n;
            }
        }
        if !ifds3.is_null()
            && (*ifds3)
                .__fds_bits[(s
                / (8 as libc::c_int
                    * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                        as libc::c_int)) as usize]
                & ((1 as libc::c_ulong)
                    << s
                        % (8 as libc::c_int
                            * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                as libc::c_int)) as __fd_mask
                != 0 as libc::c_int as libc::c_long
        {
            if !((*ofds3)
                .__fds_bits[(s
                / (8 as libc::c_int
                    * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                        as libc::c_int)) as usize]
                & ((1 as libc::c_ulong)
                    << s
                        % (8 as libc::c_int
                            * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                as libc::c_int)) as __fd_mask
                != 0 as libc::c_int as libc::c_long)
            {
                (*ifds3)
                    .__fds_bits[(s
                    / (8 as libc::c_int
                        * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                            as libc::c_int)) as usize]
                    &= !(((1 as libc::c_ulong)
                        << s
                            % (8 as libc::c_int
                                * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                    as libc::c_int)) as __fd_mask);
            } else {
                n += 1;
                n;
            }
        }
        s += 1;
        s;
    }
    return n;
}
