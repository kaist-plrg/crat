use ::libc;
extern "C" {
    fn raise(__sig: libc::c_int) -> libc::c_int;
    fn sigemptyset(__set: *mut sigset_t) -> libc::c_int;
    fn sigaddset(__set: *mut sigset_t, __signo: libc::c_int) -> libc::c_int;
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
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn abort() -> !;
}
pub type __uint32_t = libc::c_uint;
pub type __uid_t = libc::c_uint;
pub type __pid_t = libc::c_int;
pub type __clock_t = libc::c_long;
pub type __sig_atomic_t = libc::c_int;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
pub type sigset_t = __sigset_t;
pub type sig_atomic_t = __sig_atomic_t;
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
pub type action_t = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct actions_entry_t {
    pub action: action_t,
}
pub type sa_handler_t = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
pub type gl_once_t = libc::c_int;
#[inline]
unsafe extern "C" fn get_handler(mut a: *const sigaction) -> sa_handler_t {
    return (*a).__sigaction_handler.sa_handler;
}
static mut fatal_signals: [libc::c_int; 7] = [
    2 as libc::c_int,
    15 as libc::c_int,
    1 as libc::c_int,
    13 as libc::c_int,
    24 as libc::c_int,
    25 as libc::c_int,
    0 as libc::c_int,
];
unsafe extern "C" fn init_fatal_signals() {
    static mut fatal_signals_initialized: bool = 0 as libc::c_int != 0;
    if !fatal_signals_initialized {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i
            < (::std::mem::size_of::<[libc::c_int; 7]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        {
            let mut action: sigaction = sigaction {
                __sigaction_handler: C2RustUnnamed_9 {
                    sa_handler: None,
                },
                sa_mask: __sigset_t { __val: [0; 16] },
                sa_flags: 0,
                sa_restorer: None,
            };
            if sigaction(fatal_signals[i as usize], 0 as *const sigaction, &mut action)
                >= 0 as libc::c_int
                && get_handler(&mut action)
                    == ::std::mem::transmute::<
                        libc::intptr_t,
                        __sighandler_t,
                    >(1 as libc::c_int as libc::intptr_t)
            {
                fatal_signals[i as usize] = -(1 as libc::c_int);
            }
            i = i.wrapping_add(1);
            i;
        }
        fatal_signals_initialized = 1 as libc::c_int != 0;
    }
}
static mut static_actions: [actions_entry_t; 32] = [actions_entry_t {
    action: None,
}; 32];
static mut actions: *mut actions_entry_t = unsafe { static_actions.as_ptr() as *mut _ };
static mut actions_count: sig_atomic_t = 0 as libc::c_int;
static mut actions_allocated: size_t = 0;
static mut saved_sigactions: [sigaction; 64] = [sigaction {
    __sigaction_handler: C2RustUnnamed_9 {
        sa_handler: None,
    },
    sa_mask: __sigset_t { __val: [0; 16] },
    sa_flags: 0,
    sa_restorer: None,
}; 64];
unsafe extern "C" fn uninstall_handlers() {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i
        < (::std::mem::size_of::<[libc::c_int; 7]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
    {
        if fatal_signals[i as usize] >= 0 as libc::c_int {
            let mut sig: libc::c_int = fatal_signals[i as usize];
            if saved_sigactions[sig as usize].__sigaction_handler.sa_handler
                == ::std::mem::transmute::<
                    libc::intptr_t,
                    __sighandler_t,
                >(1 as libc::c_int as libc::intptr_t)
            {
                saved_sigactions[sig as usize].__sigaction_handler.sa_handler = None;
            }
            sigaction(
                sig,
                &mut *saved_sigactions.as_mut_ptr().offset(sig as isize),
                0 as *mut sigaction,
            );
        }
        i = i.wrapping_add(1);
        i;
    }
}
unsafe extern "C" fn fatal_signal_handler(mut sig: libc::c_int) {
    loop {
        let mut action: action_t = None;
        let mut n: size_t = actions_count as size_t;
        if n == 0 as libc::c_int as libc::c_ulong {
            break;
        }
        n = n.wrapping_sub(1);
        n;
        ::std::ptr::write_volatile(
            &mut actions_count as *mut sig_atomic_t,
            n as sig_atomic_t,
        );
        action = (*actions.offset(n as isize)).action;
        action.unwrap()(sig);
    }
    uninstall_handlers();
    raise(sig);
}
unsafe extern "C" fn install_handlers() {
    let mut i: size_t = 0;
    let mut action: sigaction = sigaction {
        __sigaction_handler: C2RustUnnamed_9 {
            sa_handler: None,
        },
        sa_mask: __sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    action
        .__sigaction_handler
        .sa_handler = Some(
        fatal_signal_handler as unsafe extern "C" fn(libc::c_int) -> (),
    );
    action.sa_flags = 0x40000000 as libc::c_int;
    sigemptyset(&mut action.sa_mask);
    i = 0 as libc::c_int as size_t;
    while i
        < (::std::mem::size_of::<[libc::c_int; 7]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
    {
        if fatal_signals[i as usize] >= 0 as libc::c_int {
            let mut sig: libc::c_int = fatal_signals[i as usize];
            if !((sig as libc::c_ulong)
                < (::std::mem::size_of::<[sigaction; 64]>() as libc::c_ulong)
                    .wrapping_div(::std::mem::size_of::<sigaction>() as libc::c_ulong))
            {
                abort();
            }
            sigaction(
                sig,
                &mut action,
                &mut *saved_sigactions.as_mut_ptr().offset(sig as isize),
            );
        }
        i = i.wrapping_add(1);
        i;
    }
}
pub unsafe extern "C" fn at_fatal_signal(mut action: action_t) -> libc::c_int {
    let mut current_block: u64;
    let mut mt: bool = 1 as libc::c_int != 0;
    mt;
    static mut cleanup_initialized: bool = 0 as libc::c_int != 0;
    if !cleanup_initialized {
        init_fatal_signals();
        install_handlers();
        cleanup_initialized = 1 as libc::c_int != 0;
    }
    let mut ret: libc::c_int = 0 as libc::c_int;
    if actions_count as libc::c_ulong == actions_allocated {
        let mut old_actions: *mut actions_entry_t = actions;
        let mut old_actions_allocated: size_t = actions_allocated;
        let mut new_actions_allocated: size_t = (2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(actions_allocated);
        let mut new_actions: *mut actions_entry_t = malloc(
            new_actions_allocated
                .wrapping_mul(::std::mem::size_of::<actions_entry_t>() as libc::c_ulong),
        ) as *mut actions_entry_t;
        if new_actions.is_null() {
            ret = -(1 as libc::c_int);
            current_block = 9846966827070660599;
        } else {
            let mut k: size_t = 0;
            k = 0 as libc::c_int as size_t;
            while k < old_actions_allocated {
                *new_actions.offset(k as isize) = *old_actions.offset(k as isize);
                k = k.wrapping_add(1);
                k;
            }
            ::std::ptr::write_volatile(
                &mut actions as *mut *mut actions_entry_t,
                new_actions,
            );
            actions_allocated = new_actions_allocated;
            current_block = 5948590327928692120;
        }
    } else {
        current_block = 5948590327928692120;
    }
    match current_block {
        5948590327928692120 => {
            let ref mut fresh0 = (*actions.offset(actions_count as isize)).action;
            ::std::ptr::write_volatile(fresh0, action);
            ::std::ptr::write_volatile(
                &mut actions_count as *mut sig_atomic_t,
                ::std::ptr::read_volatile::<
                    sig_atomic_t,
                >(&actions_count as *const sig_atomic_t) + 1,
            );
            ::std::ptr::read_volatile::<
                sig_atomic_t,
            >(&actions_count as *const sig_atomic_t);
        }
        _ => {}
    }
    mt;
    return ret;
}
static mut fatal_signal_set: sigset_t = __sigset_t { __val: [0; 16] };
unsafe extern "C" fn do_init_fatal_signal_set() {
    let mut i: size_t = 0;
    init_fatal_signals();
    sigemptyset(&mut fatal_signal_set);
    i = 0 as libc::c_int as size_t;
    while i
        < (::std::mem::size_of::<[libc::c_int; 7]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
    {
        if fatal_signals[i as usize] >= 0 as libc::c_int {
            sigaddset(&mut fatal_signal_set, fatal_signals[i as usize]);
        }
        i = i.wrapping_add(1);
        i;
    }
}
static mut fatal_signal_set_once: gl_once_t = 0 as libc::c_int;
unsafe extern "C" fn init_fatal_signal_set() {
    if if fatal_signal_set_once == 0 as libc::c_int {
        fatal_signal_set_once = !(0 as libc::c_int);
        do_init_fatal_signal_set();
        0 as libc::c_int
    } else {
        0 as libc::c_int
    } != 0
    {
        abort();
    }
}
static mut fatal_signals_block_counter: libc::c_uint = 0 as libc::c_int as libc::c_uint;
pub unsafe extern "C" fn block_fatal_signals() {
    let mut mt: bool = 1 as libc::c_int != 0;
    mt;
    let fresh1 = fatal_signals_block_counter;
    fatal_signals_block_counter = fatal_signals_block_counter.wrapping_add(1);
    if fresh1 == 0 as libc::c_int as libc::c_uint {
        init_fatal_signal_set();
        sigprocmask(0 as libc::c_int, &mut fatal_signal_set, 0 as *mut sigset_t);
    }
    mt;
}
pub unsafe extern "C" fn unblock_fatal_signals() {
    let mut mt: bool = 1 as libc::c_int != 0;
    mt;
    if fatal_signals_block_counter == 0 as libc::c_int as libc::c_uint {
        abort();
    }
    fatal_signals_block_counter = fatal_signals_block_counter.wrapping_sub(1);
    if fatal_signals_block_counter == 0 as libc::c_int as libc::c_uint {
        init_fatal_signal_set();
        sigprocmask(1 as libc::c_int, &mut fatal_signal_set, 0 as *mut sigset_t);
    }
    mt;
}
pub unsafe extern "C" fn get_fatal_signals(
    mut signals: *mut libc::c_int,
) -> libc::c_uint {
    init_fatal_signal_set();
    let mut p: *mut libc::c_int = signals;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i
        < (::std::mem::size_of::<[libc::c_int; 7]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
    {
        if fatal_signals[i as usize] >= 0 as libc::c_int {
            let fresh2 = p;
            p = p.offset(1);
            *fresh2 = fatal_signals[i as usize];
        }
        i = i.wrapping_add(1);
        i;
    }
    return p.offset_from(signals) as libc::c_long as libc::c_uint;
}
pub unsafe extern "C" fn get_fatal_signal_set() -> *const sigset_t {
    init_fatal_signal_set();
    return &mut fatal_signal_set;
}
unsafe extern "C" fn run_static_initializers() {
    actions_allocated = (::std::mem::size_of::<[actions_entry_t; 32]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<actions_entry_t>() as libc::c_ulong);
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
