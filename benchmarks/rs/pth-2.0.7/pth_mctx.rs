use ::libc;
extern "C" {
    fn getcontext(_: *mut ucontext_t) -> libc::c_int;
    fn makecontext(
        __ucp: *mut ucontext_t,
        __func: Option::<unsafe extern "C" fn() -> ()>,
        __argc: libc::c_int,
        _: ...
    );
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
pub unsafe extern "C" fn __pth_mctx_set(
    mut mctx: *mut pth_mctx_t,
    mut func: Option::<unsafe extern "C" fn() -> ()>,
    mut sk_addr_lo: *mut libc::c_char,
    mut sk_addr_hi: *mut libc::c_char,
) -> libc::c_int {
    if getcontext(&mut (*mctx).uc) != 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    (*mctx).uc.uc_link = 0 as *mut ucontext_t;
    (*mctx).uc.uc_stack.ss_sp = sk_addr_lo as *mut libc::c_void;
    (*mctx)
        .uc
        .uc_stack
        .ss_size = sk_addr_hi.offset_from(sk_addr_lo) as libc::c_long as size_t;
    (*mctx).uc.uc_stack.ss_flags = 0 as libc::c_int;
    makecontext(
        &mut (*mctx).uc as *mut ucontext_t,
        func,
        0 as libc::c_int + 1 as libc::c_int,
    );
    return (0 as libc::c_int == 0) as libc::c_int;
}
