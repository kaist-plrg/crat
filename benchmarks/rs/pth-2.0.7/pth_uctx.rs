use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn sigprocmask(
        __how: libc::c_int,
        __set: *const sigset_t,
        __oset: *mut sigset_t,
    ) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn swapcontext(__oucp: *mut ucontext_t, __ucp: *const ucontext_t) -> libc::c_int;
    fn setcontext(__ucp: *const ucontext_t) -> libc::c_int;
    fn __pth_mctx_set(
        _: *mut pth_mctx_t,
        _: Option::<unsafe extern "C" fn() -> ()>,
        _: *mut libc::c_char,
        _: *mut libc::c_char,
    ) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
pub type sigset_t = __sigset_t;
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
pub type pth_mctx_t = pth_mctx_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pth_mctx_st {
    pub uc: ucontext_t,
    pub restored: libc::c_int,
    pub sigs: sigset_t,
    pub error: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pth_uctx_st {
    pub uc_stack_own: libc::c_int,
    pub uc_stack_ptr: *mut libc::c_char,
    pub uc_stack_len: size_t,
    pub uc_mctx_set: libc::c_int,
    pub uc_mctx: pth_mctx_t,
}
pub type pth_uctx_t = *mut pth_uctx_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pth_uctx_trampoline_t {
    pub mctx_parent: *mut pth_mctx_t,
    pub uctx_this: pth_uctx_t,
    pub uctx_after: pth_uctx_t,
    pub start_func: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub start_arg: *mut libc::c_void,
}
pub unsafe extern "C" fn pth_uctx_create(mut puctx: *mut pth_uctx_t) -> libc::c_int {
    let mut uctx: pth_uctx_t = 0 as *mut pth_uctx_st;
    if puctx.is_null() {
        *__errno_location() = 22 as libc::c_int;
        return 0 as libc::c_int;
    }
    uctx = malloc(::std::mem::size_of::<pth_uctx_st>() as libc::c_ulong) as pth_uctx_t;
    if uctx.is_null() {
        *__errno_location() = *__errno_location();
        return 0 as libc::c_int;
    }
    (*uctx).uc_stack_own = 0 as libc::c_int;
    (*uctx).uc_stack_ptr = 0 as *mut libc::c_char;
    (*uctx).uc_stack_len = 0 as libc::c_int as size_t;
    (*uctx).uc_mctx_set = 0 as libc::c_int;
    memset(
        &mut (*uctx).uc_mctx as *mut pth_mctx_t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<pth_mctx_t>() as libc::c_ulong,
    );
    *puctx = uctx;
    return (0 as libc::c_int == 0) as libc::c_int;
}
pub static mut pth_uctx_trampoline_ctx: pth_uctx_trampoline_t = pth_uctx_trampoline_t {
    mctx_parent: 0 as *const pth_mctx_t as *mut pth_mctx_t,
    uctx_this: 0 as *const pth_uctx_st as *mut pth_uctx_st,
    uctx_after: 0 as *const pth_uctx_st as *mut pth_uctx_st,
    start_func: None,
    start_arg: 0 as *const libc::c_void as *mut libc::c_void,
};
unsafe extern "C" fn pth_uctx_trampoline() {
    let mut ctx: pth_uctx_trampoline_t = pth_uctx_trampoline_t {
        mctx_parent: 0 as *const pth_mctx_t as *mut pth_mctx_t,
        uctx_this: 0 as *const pth_uctx_st as *mut pth_uctx_st,
        uctx_after: 0 as *const pth_uctx_st as *mut pth_uctx_st,
        start_func: None,
        start_arg: 0 as *const libc::c_void as *mut libc::c_void,
    };
    ::std::ptr::write_volatile(
        &mut ctx.mctx_parent as *mut *mut pth_mctx_t,
        pth_uctx_trampoline_ctx.mctx_parent,
    );
    ::std::ptr::write_volatile(
        &mut ctx.uctx_this as *mut pth_uctx_t,
        pth_uctx_trampoline_ctx.uctx_this,
    );
    ::std::ptr::write_volatile(
        &mut ctx.uctx_after as *mut pth_uctx_t,
        pth_uctx_trampoline_ctx.uctx_after,
    );
    ::std::ptr::write_volatile(
        &mut ctx.start_func
            as *mut Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
        pth_uctx_trampoline_ctx.start_func,
    );
    ::std::ptr::write_volatile(
        &mut ctx.start_arg as *mut *mut libc::c_void,
        pth_uctx_trampoline_ctx.start_arg,
    );
    swapcontext(&mut (*ctx.uctx_this).uc_mctx.uc, &mut (*ctx.mctx_parent).uc);
    (Some((ctx.start_func).unwrap())).unwrap()(ctx.start_arg);
    if !(ctx.uctx_after).is_null() {
        *__errno_location() = (*ctx.uctx_after).uc_mctx.error;
        (*ctx.uctx_after).uc_mctx.restored = 1 as libc::c_int;
        setcontext(&mut (*ctx.uctx_after).uc_mctx.uc);
    }
    exit(0 as libc::c_int);
}
pub unsafe extern "C" fn pth_uctx_make(
    mut uctx: pth_uctx_t,
    mut sk_addr: *mut libc::c_char,
    mut sk_size: size_t,
    mut sigmask: *const sigset_t,
    mut start_func: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    mut start_arg: *mut libc::c_void,
    mut uctx_after: pth_uctx_t,
) -> libc::c_int {
    let mut mctx_parent: pth_mctx_t = pth_mctx_t {
        uc: ucontext_t {
            uc_flags: 0,
            uc_link: 0 as *mut ucontext_t,
            uc_stack: stack_t {
                ss_sp: 0 as *mut libc::c_void,
                ss_flags: 0,
                ss_size: 0,
            },
            uc_mcontext: mcontext_t {
                gregs: [0; 23],
                fpregs: 0 as *mut _libc_fpstate,
                __reserved1: [0; 8],
            },
            uc_sigmask: sigset_t { __val: [0; 16] },
            __fpregs_mem: _libc_fpstate {
                cwd: 0,
                swd: 0,
                ftw: 0,
                fop: 0,
                rip: 0,
                rdp: 0,
                mxcsr: 0,
                mxcr_mask: 0,
                _st: [_libc_fpxreg {
                    significand: [0; 4],
                    exponent: 0,
                    __glibc_reserved1: [0; 3],
                }; 8],
                _xmm: [_libc_xmmreg { element: [0; 4] }; 16],
                __glibc_reserved1: [0; 24],
            },
            __ssp: [0; 4],
        },
        restored: 0,
        sigs: sigset_t { __val: [0; 16] },
        error: 0,
    };
    let mut ss: sigset_t = sigset_t { __val: [0; 16] };
    if uctx.is_null() || start_func.is_none()
        || sk_size < (16 as libc::c_int * 1024 as libc::c_int) as libc::c_ulong
    {
        *__errno_location() = 22 as libc::c_int;
        return 0 as libc::c_int;
    }
    if sk_addr.is_null() {
        sk_addr = malloc(sk_size) as *mut libc::c_char;
        if sk_addr.is_null() {
            *__errno_location() = *__errno_location();
            return 0 as libc::c_int;
        }
        (*uctx).uc_stack_own = (0 as libc::c_int == 0) as libc::c_int;
    } else {
        (*uctx).uc_stack_own = 0 as libc::c_int;
    }
    (*uctx).uc_stack_ptr = sk_addr;
    (*uctx).uc_stack_len = sk_size;
    if __pth_mctx_set(
        &mut (*uctx).uc_mctx,
        Some(pth_uctx_trampoline as unsafe extern "C" fn() -> ()),
        (*uctx).uc_stack_ptr,
        ((*uctx).uc_stack_ptr).offset((*uctx).uc_stack_len as isize),
    ) == 0
    {
        *__errno_location() = *__errno_location();
        return 0 as libc::c_int;
    }
    pth_uctx_trampoline_ctx.mctx_parent = &mut mctx_parent;
    pth_uctx_trampoline_ctx.uctx_this = uctx;
    pth_uctx_trampoline_ctx.uctx_after = uctx_after;
    pth_uctx_trampoline_ctx.start_func = start_func;
    pth_uctx_trampoline_ctx.start_arg = start_arg;
    if !sigmask.is_null() {
        sigprocmask(2 as libc::c_int, sigmask, &mut ss);
    }
    swapcontext(&mut mctx_parent.uc, &mut (*uctx).uc_mctx.uc);
    if !sigmask.is_null() {
        sigprocmask(2 as libc::c_int, &mut ss, 0 as *mut sigset_t);
    }
    (*uctx).uc_mctx_set = (0 as libc::c_int == 0) as libc::c_int;
    return (0 as libc::c_int == 0) as libc::c_int;
}
pub unsafe extern "C" fn pth_uctx_switch(
    mut uctx_from: pth_uctx_t,
    mut uctx_to: pth_uctx_t,
) -> libc::c_int {
    if uctx_from.is_null() || uctx_to.is_null() {
        *__errno_location() = 22 as libc::c_int;
        return 0 as libc::c_int;
    }
    if (*uctx_to).uc_mctx_set == 0 {
        *__errno_location() = 1 as libc::c_int;
        return 0 as libc::c_int;
    }
    (*uctx_from).uc_mctx_set = (0 as libc::c_int == 0) as libc::c_int;
    swapcontext(&mut (*uctx_from).uc_mctx.uc, &mut (*uctx_to).uc_mctx.uc);
    return (0 as libc::c_int == 0) as libc::c_int;
}
pub unsafe extern "C" fn pth_uctx_destroy(mut uctx: pth_uctx_t) -> libc::c_int {
    if uctx.is_null() {
        *__errno_location() = 22 as libc::c_int;
        return 0 as libc::c_int;
    }
    if (*uctx).uc_stack_own != 0 && !((*uctx).uc_stack_ptr).is_null() {
        free((*uctx).uc_stack_ptr as *mut libc::c_void);
    }
    free(uctx as *mut libc::c_void);
    return (0 as libc::c_int == 0) as libc::c_int;
}
