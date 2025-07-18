use ::libc;
extern "C" {
    fn exit(_: libc::c_int) -> !;
    fn raise(__sig: libc::c_int) -> libc::c_int;
    fn sigemptyset(__set: *mut sigset_t) -> libc::c_int;
    fn sigaction(
        __sig: libc::c_int,
        __act: *const sigaction,
        __oact: *mut sigaction,
    ) -> libc::c_int;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn __errno_location() -> *mut libc::c_int;
    fn log_level_up();
    fn log_level_down();
    fn log_stacktrace();
    fn log_reopen();
    fn log_loggable(level: libc::c_int) -> libc::c_int;
    fn _log(
        file: *const libc::c_char,
        line: libc::c_int,
        panic: libc::c_int,
        fmt: *const libc::c_char,
        _: ...
    );
    fn _log_safe(fmt: *const libc::c_char, _: ...);
}
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
pub type rstatus_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct signal {
    pub signo: libc::c_int,
    pub signame: *mut libc::c_char,
    pub flags: libc::c_int,
    pub handler: Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
}
static mut signals: [signal; 9] = [signal {
    signo: 0,
    signame: 0 as *mut libc::c_char,
    flags: 0,
    handler: None,
}; 9];
pub unsafe extern "C" fn signal_init() -> rstatus_t {
    let mut sig: *const signal = 0 as *const signal;
    sig = signals.as_ptr();
    while (*sig).signo != 0 as libc::c_int {
        let mut status: rstatus_t = 0;
        let mut sa: sigaction = sigaction {
            __sigaction_handler: C2RustUnnamed_9 {
                sa_handler: None,
            },
            sa_mask: __sigset_t { __val: [0; 16] },
            sa_flags: 0,
            sa_restorer: None,
        };
        memset(
            &mut sa as *mut sigaction as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<sigaction>() as libc::c_ulong,
        );
        sa.__sigaction_handler.sa_handler = (*sig).handler;
        sa.sa_flags = (*sig).flags;
        sigemptyset(&mut sa.sa_mask);
        status = sigaction((*sig).signo, &mut sa, 0 as *mut sigaction);
        if status < 0 as libc::c_int {
            if log_loggable(1 as libc::c_int) != 0 as libc::c_int {
                _log(
                    b"nc_signal.c\0" as *const u8 as *const libc::c_char,
                    53 as libc::c_int,
                    0 as libc::c_int,
                    b"sigaction(%s) failed: %s\0" as *const u8 as *const libc::c_char,
                    (*sig).signame,
                    strerror(*__errno_location()),
                );
            }
            return -(1 as libc::c_int);
        }
        sig = sig.offset(1);
        sig;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn signal_deinit() {}
pub unsafe extern "C" fn signal_handler(mut signo: libc::c_int) {
    let mut sig: *const signal = 0 as *const signal;
    let mut action: Option::<unsafe extern "C" fn() -> ()> = None;
    let mut actionstr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut done: bool = false;
    sig = signals.as_ptr();
    while (*sig).signo != 0 as libc::c_int {
        if (*sig).signo == signo {
            break;
        }
        sig = sig.offset(1);
        sig;
    }
    actionstr = b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    action = None;
    done = 0 as libc::c_int != 0;
    match signo {
        21 => {
            actionstr = b", up logging level\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
            action = Some(log_level_up as unsafe extern "C" fn() -> ());
        }
        22 => {
            actionstr = b", down logging level\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
            action = Some(log_level_down as unsafe extern "C" fn() -> ());
        }
        1 => {
            actionstr = b", reopening log file\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
            action = Some(log_reopen as unsafe extern "C" fn() -> ());
        }
        2 => {
            done = 1 as libc::c_int != 0;
            actionstr = b", exiting\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
        }
        11 => {
            log_stacktrace();
            actionstr = b", core dumping\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
            raise(11 as libc::c_int);
        }
        10 | 12 | _ => {}
    }
    _log_safe(
        b"signal %d (%s) received%s\0" as *const u8 as *const libc::c_char,
        signo,
        (*sig).signame,
        actionstr,
    );
    if action.is_some() {
        action.unwrap()();
    }
    if done {
        exit(1 as libc::c_int);
    }
}
unsafe extern "C" fn run_static_initializers() {
    signals = [
        {
            let mut init = signal {
                signo: 10 as libc::c_int,
                signame: b"SIGUSR1\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                flags: 0 as libc::c_int,
                handler: Some(signal_handler as unsafe extern "C" fn(libc::c_int) -> ()),
            };
            init
        },
        {
            let mut init = signal {
                signo: 12 as libc::c_int,
                signame: b"SIGUSR2\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                flags: 0 as libc::c_int,
                handler: Some(signal_handler as unsafe extern "C" fn(libc::c_int) -> ()),
            };
            init
        },
        {
            let mut init = signal {
                signo: 21 as libc::c_int,
                signame: b"SIGTTIN\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                flags: 0 as libc::c_int,
                handler: Some(signal_handler as unsafe extern "C" fn(libc::c_int) -> ()),
            };
            init
        },
        {
            let mut init = signal {
                signo: 22 as libc::c_int,
                signame: b"SIGTTOU\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                flags: 0 as libc::c_int,
                handler: Some(signal_handler as unsafe extern "C" fn(libc::c_int) -> ()),
            };
            init
        },
        {
            let mut init = signal {
                signo: 1 as libc::c_int,
                signame: b"SIGHUP\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                flags: 0 as libc::c_int,
                handler: Some(signal_handler as unsafe extern "C" fn(libc::c_int) -> ()),
            };
            init
        },
        {
            let mut init = signal {
                signo: 2 as libc::c_int,
                signame: b"SIGINT\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                flags: 0 as libc::c_int,
                handler: Some(signal_handler as unsafe extern "C" fn(libc::c_int) -> ()),
            };
            init
        },
        {
            let mut init = signal {
                signo: 11 as libc::c_int,
                signame: b"SIGSEGV\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                flags: 0x80000000 as libc::c_uint as libc::c_int,
                handler: Some(signal_handler as unsafe extern "C" fn(libc::c_int) -> ()),
            };
            init
        },
        {
            let mut init = signal {
                signo: 13 as libc::c_int,
                signame: b"SIGPIPE\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                flags: 0 as libc::c_int,
                handler: ::std::mem::transmute::<
                    libc::intptr_t,
                    __sighandler_t,
                >(1 as libc::c_int as libc::intptr_t),
            };
            init
        },
        {
            let mut init = signal {
                signo: 0 as libc::c_int,
                signame: 0 as *mut libc::c_char,
                flags: 0 as libc::c_int,
                handler: None,
            };
            init
        },
    ];
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
