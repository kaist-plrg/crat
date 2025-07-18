use ::libc;
extern "C" {
    fn exit(_: libc::c_int) -> !;
    fn perror(__s: *const libc::c_char);
    fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;
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
pub type __sigval_t = sigval;
#[derive(Copy, Clone)]
#[repr(C)]
pub union sigval {
    pub sival_int: libc::c_int,
    pub sival_ptr: *mut libc::c_void,
}
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
pub struct saved_sig_state {
    pub sa: [sigaction; 4],
}
pub static mut got_signal: libc::c_int = 0 as libc::c_int;
unsafe extern "C" fn signal_handler(mut dummy: libc::c_int) {
    got_signal = 1 as libc::c_int;
}
pub unsafe extern "C" fn setup_signal() {
    signal(
        1 as libc::c_int,
        Some(signal_handler as unsafe extern "C" fn(libc::c_int) -> ()),
    );
    signal(
        2 as libc::c_int,
        Some(signal_handler as unsafe extern "C" fn(libc::c_int) -> ()),
    );
    signal(
        15 as libc::c_int,
        Some(signal_handler as unsafe extern "C" fn(libc::c_int) -> ()),
    );
    signal(
        3 as libc::c_int,
        Some(signal_handler as unsafe extern "C" fn(libc::c_int) -> ()),
    );
}
unsafe extern "C" fn _allow_interrupt(
    mut ss: *mut saved_sig_state,
    mut sig: libc::c_int,
    mut slot: libc::c_int,
) {
    let mut new: sigaction = sigaction {
        __sigaction_handler: C2RustUnnamed_9 {
            sa_handler: None,
        },
        sa_mask: __sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    memset(
        &mut new as *mut sigaction as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<sigaction>() as libc::c_ulong,
    );
    new
        .__sigaction_handler
        .sa_handler = Some(signal_handler as unsafe extern "C" fn(libc::c_int) -> ());
    new.sa_flags &= !(0x10000000 as libc::c_int);
    if sigaction(sig, &mut new, &mut *((*ss).sa).as_mut_ptr().offset(slot as isize))
        < 0 as libc::c_int
    {
        perror(b"sigaction\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
}
pub unsafe extern "C" fn allow_interrupts(mut ss: *mut saved_sig_state) {
    _allow_interrupt(ss, 2 as libc::c_int, 0 as libc::c_int);
    _allow_interrupt(ss, 2 as libc::c_int, 1 as libc::c_int);
    _allow_interrupt(ss, 2 as libc::c_int, 2 as libc::c_int);
    _allow_interrupt(ss, 2 as libc::c_int, 3 as libc::c_int);
}
unsafe extern "C" fn _restore_interrupt(
    mut ss: *mut saved_sig_state,
    mut sig: libc::c_int,
    mut slot: libc::c_int,
) {
    if sigaction(
        sig,
        &mut *((*ss).sa).as_mut_ptr().offset(slot as isize),
        0 as *mut sigaction,
    ) < 0 as libc::c_int
    {
        perror(b"restore sigaction\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
}
pub unsafe extern "C" fn restore_interrupts(mut ss: *mut saved_sig_state) {
    _restore_interrupt(ss, 2 as libc::c_int, 0 as libc::c_int);
    _restore_interrupt(ss, 2 as libc::c_int, 1 as libc::c_int);
    _restore_interrupt(ss, 2 as libc::c_int, 2 as libc::c_int);
    _restore_interrupt(ss, 2 as libc::c_int, 3 as libc::c_int);
}
