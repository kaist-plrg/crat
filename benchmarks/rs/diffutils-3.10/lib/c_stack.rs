use ::libc;
extern "C" {
    fn __errno_location() -> *mut libc::c_int;
    fn raise(__sig: libc::c_int) -> libc::c_int;
    fn abort() -> !;
    fn getprogname() -> *const libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn mempcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn _exit(_: libc::c_int) -> !;
    fn stackoverflow_install_handler(
        handler: stackoverflow_handler_t,
        extra_stack: *mut libc::c_void,
        extra_stack_size: size_t,
    ) -> libc::c_int;
    fn sigsegv_install_handler(handler: sigsegv_handler_t) -> libc::c_int;
    static mut exit_failure: libc::c_int;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
}
pub type ssize_t = __ssize_t;
pub type __ssize_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type idx_t = ptrdiff_t;
pub type ptrdiff_t = libc::c_long;
pub type sigsegv_handler_t = Option::<
    unsafe extern "C" fn(*mut libc::c_void, libc::c_int) -> libc::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct max_align_t {
    pub __clang_max_align_nonce1: libc::c_longlong,
    pub __clang_max_align_nonce2: f128::f128,
}
pub type stackoverflow_context_t = *mut ucontext_t;
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
pub type __uint32_t = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _libc_xmmreg {
    pub element: [__uint32_t; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _libc_fpxreg {
    pub significand: [libc::c_ushort; 4],
    pub exponent: libc::c_ushort,
    pub __glibc_reserved1: [libc::c_ushort; 3],
}
pub type __uint64_t = libc::c_ulong;
pub type __uint16_t = libc::c_ushort;
pub type __sigset_t = sigset_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sigset_t {
    pub __val: [libc::c_ulong; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mcontext_t {
    pub gregs: gregset_t,
    pub fpregs: fpregset_t,
    pub __reserved1: [libc::c_ulonglong; 8],
}
pub type fpregset_t = *mut _libc_fpstate;
pub type gregset_t = [greg_t; 23];
pub type greg_t = libc::c_longlong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stack_t {
    pub ss_sp: *mut libc::c_void,
    pub ss_flags: libc::c_int,
    pub ss_size: size_t,
}
pub type stackoverflow_handler_t = Option::<
    unsafe extern "C" fn(libc::c_int, stackoverflow_context_t) -> (),
>;
static mut alternate_signal_stack: [max_align_t; 2048] = [max_align_t {
    __clang_max_align_nonce1: 0,
    __clang_max_align_nonce2: f128::f128::ZERO,
}; 2048];
static mut segv_action: Option::<unsafe extern "C" fn(libc::c_int) -> ()> = None;
static mut program_error_message: *const libc::c_char = 0 as *const libc::c_char;
static mut stack_overflow_message: *const libc::c_char = 0 as *const libc::c_char;
static mut progname: *const libc::c_char = 0 as *const libc::c_char;
unsafe extern "C" fn die(mut signo: libc::c_int) {
    segv_action.unwrap()(signo);
    let mut message: *const libc::c_char = if signo != 0 {
        program_error_message
    } else {
        stack_overflow_message
    };
    let mut prognamelen: size_t = strlen(progname);
    let mut messagelen: size_t = strlen(message);
    static mut separator: [libc::c_char; 2] = [
        ':' as i32 as libc::c_char,
        ' ' as i32 as libc::c_char,
    ];
    let mut buf: [libc::c_char; 4098] = [0; 4098];
    let mut buflen: idx_t = 0;
    if prognamelen.wrapping_add(messagelen)
        < (::std::mem::size_of::<[libc::c_char; 4098]>() as libc::c_ulong)
            .wrapping_sub(::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong)
    {
        let mut p: *mut libc::c_char = mempcpy(
            buf.as_mut_ptr() as *mut libc::c_void,
            progname as *const libc::c_void,
            prognamelen,
        ) as *mut libc::c_char;
        p = mempcpy(
            p as *mut libc::c_void,
            separator.as_ptr() as *const libc::c_void,
            ::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong,
        ) as *mut libc::c_char;
        p = mempcpy(p as *mut libc::c_void, message as *const libc::c_void, messagelen)
            as *mut libc::c_char;
        let fresh0 = p;
        p = p.offset(1);
        *fresh0 = '\n' as i32 as libc::c_char;
        buflen = p.offset_from(buf.as_mut_ptr()) as libc::c_long;
    } else {
        write(2 as libc::c_int, progname as *const libc::c_void, prognamelen);
        write(
            2 as libc::c_int,
            separator.as_ptr() as *const libc::c_void,
            ::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong,
        );
        write(2 as libc::c_int, message as *const libc::c_void, messagelen);
        buf[0 as libc::c_int as usize] = '\n' as i32 as libc::c_char;
        buflen = 1 as libc::c_int as idx_t;
    }
    write(2 as libc::c_int, buf.as_mut_ptr() as *const libc::c_void, buflen as size_t);
    if signo == 0 {
        _exit(exit_failure);
    }
    raise(signo);
    abort();
}
unsafe extern "C" fn null_action(mut signo: libc::c_int) {}
static mut segv_handler_missing: libc::c_int = 0;
unsafe extern "C" fn segv_handler(
    mut address: *mut libc::c_void,
    mut serious: libc::c_int,
) -> libc::c_int {
    if serious == 0 {
        return 0 as libc::c_int;
    }
    die(11 as libc::c_int);
    panic!();
}
unsafe extern "C" fn overflow_handler(
    mut emergency: libc::c_int,
    mut context: stackoverflow_context_t,
) {
    die(
        if emergency == 0 || segv_handler_missing != 0 {
            0 as libc::c_int
        } else {
            11 as libc::c_int
        },
    );
}
pub unsafe extern "C" fn c_stack_action(
    mut action: Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
) -> libc::c_int {
    ::std::ptr::write_volatile(
        &mut segv_action as *mut Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
        if action.is_some() {
            action
        } else {
            Some(null_action as unsafe extern "C" fn(libc::c_int) -> ())
        },
    );
    ::std::ptr::write_volatile(
        &mut program_error_message as *mut *const libc::c_char,
        dcgettext(
            0 as *const libc::c_char,
            b"program error\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    ::std::ptr::write_volatile(
        &mut stack_overflow_message as *mut *const libc::c_char,
        dcgettext(
            0 as *const libc::c_char,
            b"stack overflow\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    ::std::ptr::write_volatile(&mut progname as *mut *const libc::c_char, getprogname());
    if stackoverflow_install_handler(
        Some(
            overflow_handler
                as unsafe extern "C" fn(libc::c_int, stackoverflow_context_t) -> (),
        ),
        alternate_signal_stack.as_mut_ptr() as *mut libc::c_void,
        ::std::mem::size_of::<[max_align_t; 2048]>() as libc::c_ulong,
    ) != 0
    {
        *__errno_location() = 95 as libc::c_int;
        return -(1 as libc::c_int);
    }
    ::std::ptr::write_volatile(
        &mut segv_handler_missing as *mut libc::c_int,
        sigsegv_install_handler(
            Some(
                segv_handler
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        libc::c_int,
                    ) -> libc::c_int,
            ),
        ),
    );
    return 0 as libc::c_int;
}
