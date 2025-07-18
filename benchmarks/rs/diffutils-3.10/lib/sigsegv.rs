use ::libc;
extern "C" {
    fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;
    fn sigemptyset(__set: *mut sigset_t) -> libc::c_int;
    fn sigaddset(__set: *mut sigset_t, __signo: libc::c_int) -> libc::c_int;
    fn sigaction(
        __sig: libc::c_int,
        __act: *const sigaction,
        __oact: *mut sigaction,
    ) -> libc::c_int;
    fn sigaltstack(__ss: *const stack_t, __oss: *mut stack_t) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn perror(__s: *const libc::c_char);
    fn sigsegv_get_vma(address: uintptr_t, vma: *mut vma_struct) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __pid_t = libc::c_int;
pub type __clock_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sigset_t {
    pub __val: [libc::c_ulong; 16],
}
pub type __sigset_t = sigset_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stack_t {
    pub ss_sp: *mut libc::c_void,
    pub ss_flags: libc::c_int,
    pub ss_size: size_t,
}
pub type greg_t = libc::c_longlong;
pub type gregset_t = [greg_t; 23];
pub type C2RustUnnamed = libc::c_uint;
pub const REG_CR2: C2RustUnnamed = 22;
pub const REG_OLDMASK: C2RustUnnamed = 21;
pub const REG_TRAPNO: C2RustUnnamed = 20;
pub const REG_ERR: C2RustUnnamed = 19;
pub const REG_CSGSFS: C2RustUnnamed = 18;
pub const REG_EFL: C2RustUnnamed = 17;
pub const REG_RIP: C2RustUnnamed = 16;
pub const REG_RSP: C2RustUnnamed = 15;
pub const REG_RCX: C2RustUnnamed = 14;
pub const REG_RAX: C2RustUnnamed = 13;
pub const REG_RDX: C2RustUnnamed = 12;
pub const REG_RBX: C2RustUnnamed = 11;
pub const REG_RBP: C2RustUnnamed = 10;
pub const REG_RSI: C2RustUnnamed = 9;
pub const REG_RDI: C2RustUnnamed = 8;
pub const REG_R15: C2RustUnnamed = 7;
pub const REG_R14: C2RustUnnamed = 6;
pub const REG_R13: C2RustUnnamed = 5;
pub const REG_R12: C2RustUnnamed = 4;
pub const REG_R11: C2RustUnnamed = 3;
pub const REG_R10: C2RustUnnamed = 2;
pub const REG_R9: C2RustUnnamed = 1;
pub const REG_R8: C2RustUnnamed = 0;
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
    pub _sifields: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub _pad: [libc::c_int; 28],
    pub _kill: C2RustUnnamed_9,
    pub _timer: C2RustUnnamed_8,
    pub _rt: C2RustUnnamed_7,
    pub _sigchld: C2RustUnnamed_6,
    pub _sigfault: C2RustUnnamed_3,
    pub _sigpoll: C2RustUnnamed_2,
    pub _sigsys: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub _call_addr: *mut libc::c_void,
    pub _syscall: libc::c_int,
    pub _arch: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub si_band: libc::c_long,
    pub si_fd: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub si_addr: *mut libc::c_void,
    pub si_addr_lsb: libc::c_short,
    pub _bounds: C2RustUnnamed_4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
    pub _addr_bnd: C2RustUnnamed_5,
    pub _pkey: __uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub _lower: *mut libc::c_void,
    pub _upper: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_status: libc::c_int,
    pub si_utime: __clock_t,
    pub si_stime: __clock_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub si_tid: libc::c_int,
    pub si_overrun: libc::c_int,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
}
pub type __sighandler_t = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sigaction {
    pub __sigaction_handler: C2RustUnnamed_10,
    pub sa_mask: __sigset_t,
    pub sa_flags: libc::c_int,
    pub sa_restorer: Option::<unsafe extern "C" fn() -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_10 {
    pub sa_handler: __sighandler_t,
    pub sa_sigaction: Option::<
        unsafe extern "C" fn(libc::c_int, *mut siginfo_t, *mut libc::c_void) -> (),
    >,
}
pub type C2RustUnnamed_11 = libc::c_uint;
pub const SS_DISABLE: C2RustUnnamed_11 = 2;
pub const SS_ONSTACK: C2RustUnnamed_11 = 1;
pub type sigsegv_handler_t = Option::<
    unsafe extern "C" fn(*mut libc::c_void, libc::c_int) -> libc::c_int,
>;
pub type stackoverflow_handler_t = Option::<
    unsafe extern "C" fn(libc::c_int, stackoverflow_context_t) -> (),
>;
pub type stackoverflow_context_t = *mut ucontext_t;
pub type uintptr_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vma_struct {
    pub start: uintptr_t,
    pub end: uintptr_t,
    pub is_near_this: Option::<
        unsafe extern "C" fn(uintptr_t, *mut vma_struct) -> libc::c_int,
    >,
    pub prev_end: uintptr_t,
}
pub static mut libsigsegv_version: libc::c_int = 0x20d as libc::c_int;
unsafe extern "C" fn sigsegv_reset_onstack_flag() {}
static mut stack_top: uintptr_t = 0 as libc::c_int as uintptr_t;
unsafe extern "C" fn remember_stack_top(mut some_variable_on_stack: *mut libc::c_void) {
    let mut vma: vma_struct = vma_struct {
        start: 0,
        end: 0,
        is_near_this: None,
        prev_end: 0,
    };
    if sigsegv_get_vma(some_variable_on_stack as uintptr_t, &mut vma) >= 0 as libc::c_int
    {
        stack_top = (vma.end).wrapping_sub(1 as libc::c_int as libc::c_ulong);
    }
}
static mut stk_user_handler: stackoverflow_handler_t = unsafe {
    ::std::mem::transmute::<
        *mut libc::c_void,
        stackoverflow_handler_t,
    >(0 as *const libc::c_void as *mut libc::c_void)
};
static mut stk_extra_stack: uintptr_t = 0;
static mut stk_extra_stack_size: size_t = 0;
static mut user_handler: sigsegv_handler_t = unsafe {
    ::std::mem::transmute::<
        *mut libc::c_void,
        sigsegv_handler_t,
    >(0 as *const libc::c_void as *mut libc::c_void)
};
unsafe extern "C" fn sigsegv_handler(
    mut sig: libc::c_int,
    mut sip: *mut siginfo_t,
    mut ucp: *mut libc::c_void,
) {
    let mut address: *mut libc::c_void = (*sip)._sifields._sigfault.si_addr;
    if !(user_handler.is_some()
        && (Some(user_handler.unwrap())).unwrap()(address, 0 as libc::c_int) != 0)
    {
        if stk_user_handler.is_some() {
            let mut old_sp: uintptr_t = (*(ucp as *mut ucontext_t))
                .uc_mcontext
                .gregs[REG_RSP as libc::c_int as usize] as uintptr_t;
            if stack_top != 0 {
                let mut saved_errno: libc::c_int = 0;
                let mut vma: vma_struct = vma_struct {
                    start: 0,
                    end: 0,
                    is_near_this: None,
                    prev_end: 0,
                };
                let mut ret: libc::c_int = 0;
                saved_errno = *__errno_location();
                ret = sigsegv_get_vma(stack_top, &mut vma);
                *__errno_location() = saved_errno;
                if ret >= 0 as libc::c_int {
                    let mut addr: uintptr_t = address as uintptr_t;
                    if if addr >= vma.start {
                        (addr
                            <= (vma.end).wrapping_sub(1 as libc::c_int as libc::c_ulong))
                            as libc::c_int
                    } else {
                        (vma.is_near_this).unwrap()(addr, &mut vma)
                    } != 0
                    {
                        let mut emergency: libc::c_int = (old_sp >= stk_extra_stack
                            && old_sp
                                <= stk_extra_stack.wrapping_add(stk_extra_stack_size))
                            as libc::c_int;
                        let mut context: stackoverflow_context_t = ucp
                            as *mut ucontext_t;
                        (Some(stk_user_handler.unwrap())).unwrap()(emergency, context);
                    }
                }
            }
        }
        if !(user_handler.is_some()
            && (Some(user_handler.unwrap())).unwrap()(address, 1 as libc::c_int) != 0)
        {
            let mut signo: libc::c_int = 0;
            signo = 11 as libc::c_int;
            signal(signo, None);
        }
    }
}
unsafe extern "C" fn install_for(mut sig: libc::c_int) {
    let mut action: sigaction = sigaction {
        __sigaction_handler: C2RustUnnamed_10 {
            sa_handler: None,
        },
        sa_mask: __sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    action
        .__sigaction_handler
        .sa_sigaction = ::std::mem::transmute::<
        Option::<
            unsafe extern "C" fn(libc::c_int, *mut siginfo_t, *mut libc::c_void) -> (),
        >,
        Option::<
            unsafe extern "C" fn(libc::c_int, *mut siginfo_t, *mut libc::c_void) -> (),
        >,
    >(
        Some(
            sigsegv_handler
                as unsafe extern "C" fn(
                    libc::c_int,
                    *mut siginfo_t,
                    *mut libc::c_void,
                ) -> (),
        ),
    );
    sigemptyset(&mut action.sa_mask);
    sigaddset(&mut action.sa_mask, 1 as libc::c_int);
    sigaddset(&mut action.sa_mask, 2 as libc::c_int);
    sigaddset(&mut action.sa_mask, 3 as libc::c_int);
    sigaddset(&mut action.sa_mask, 13 as libc::c_int);
    sigaddset(&mut action.sa_mask, 14 as libc::c_int);
    sigaddset(&mut action.sa_mask, 15 as libc::c_int);
    sigaddset(&mut action.sa_mask, 10 as libc::c_int);
    sigaddset(&mut action.sa_mask, 12 as libc::c_int);
    sigaddset(&mut action.sa_mask, 17 as libc::c_int);
    sigaddset(&mut action.sa_mask, 17 as libc::c_int);
    sigaddset(&mut action.sa_mask, 23 as libc::c_int);
    sigaddset(&mut action.sa_mask, 29 as libc::c_int);
    sigaddset(&mut action.sa_mask, 29 as libc::c_int);
    sigaddset(&mut action.sa_mask, 24 as libc::c_int);
    sigaddset(&mut action.sa_mask, 25 as libc::c_int);
    sigaddset(&mut action.sa_mask, 26 as libc::c_int);
    sigaddset(&mut action.sa_mask, 27 as libc::c_int);
    sigaddset(&mut action.sa_mask, 30 as libc::c_int);
    sigaddset(&mut action.sa_mask, 28 as libc::c_int);
    action.sa_flags = 4 as libc::c_int;
    if stk_user_handler.is_some() {
        action.sa_flags |= 0x8000000 as libc::c_int;
    }
    sigaction(sig, &mut action, 0 as *mut libc::c_void as *mut sigaction);
}
pub unsafe extern "C" fn sigsegv_install_handler(
    mut handler: sigsegv_handler_t,
) -> libc::c_int {
    user_handler = handler;
    let mut sig: libc::c_int = 0;
    sig = 11 as libc::c_int;
    install_for(sig);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn sigsegv_deinstall_handler() {
    user_handler = ::std::mem::transmute::<
        *mut libc::c_void,
        sigsegv_handler_t,
    >(0 as *mut libc::c_void);
    if stk_user_handler.is_none() {
        let mut sig: libc::c_int = 0;
        sig = 11 as libc::c_int;
        signal(sig, None);
    }
}
pub unsafe extern "C" fn sigsegv_leave_handler(
    mut continuation: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut libc::c_void,
            *mut libc::c_void,
        ) -> (),
    >,
    mut cont_arg1: *mut libc::c_void,
    mut cont_arg2: *mut libc::c_void,
    mut cont_arg3: *mut libc::c_void,
) -> libc::c_int {
    sigsegv_reset_onstack_flag();
    (Some(continuation.unwrap())).unwrap()(cont_arg1, cont_arg2, cont_arg3);
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn stackoverflow_install_handler(
    mut handler: stackoverflow_handler_t,
    mut extra_stack: *mut libc::c_void,
    mut extra_stack_size: size_t,
) -> libc::c_int {
    if stack_top == 0 {
        let mut dummy: libc::c_int = 0;
        remember_stack_top(&mut dummy as *mut libc::c_int as *mut libc::c_void);
        if stack_top == 0 {
            return -(1 as libc::c_int);
        }
    }
    stk_user_handler = handler;
    stk_extra_stack = extra_stack as uintptr_t;
    stk_extra_stack_size = extra_stack_size;
    let mut ss: stack_t = stack_t {
        ss_sp: 0 as *mut libc::c_void,
        ss_flags: 0,
        ss_size: 0,
    };
    ss.ss_sp = extra_stack;
    ss.ss_size = extra_stack_size;
    ss.ss_flags = 0 as libc::c_int;
    if sigaltstack(&mut ss, 0 as *mut stack_t) < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    let mut sig: libc::c_int = 0;
    sig = 11 as libc::c_int;
    install_for(sig);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn stackoverflow_deinstall_handler() {
    stk_user_handler = ::std::mem::transmute::<
        *mut libc::c_void,
        stackoverflow_handler_t,
    >(0 as *mut libc::c_void);
    if user_handler.is_some() {
        let mut sig: libc::c_int = 0;
        sig = 11 as libc::c_int;
        install_for(sig);
    } else {
        let mut sig_0: libc::c_int = 0;
        sig_0 = 11 as libc::c_int;
        signal(sig_0, None);
    }
    let mut ss: stack_t = stack_t {
        ss_sp: 0 as *mut libc::c_void,
        ss_flags: 0,
        ss_size: 0,
    };
    ss.ss_flags = SS_DISABLE as libc::c_int;
    if sigaltstack(&mut ss, 0 as *mut stack_t) < 0 as libc::c_int {
        perror(
            b"gnulib sigsegv (stackoverflow_deinstall_handler)\0" as *const u8
                as *const libc::c_char,
        );
    }
}
