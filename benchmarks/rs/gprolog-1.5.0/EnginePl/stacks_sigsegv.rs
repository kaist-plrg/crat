use ::libc;
extern "C" {
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn sigemptyset(__set: *mut sigset_t) -> libc::c_int;
    fn sigaction(
        __sig: libc::c_int,
        __act: *const sigaction,
        __oact: *mut sigaction,
    ) -> libc::c_int;
    fn getpagesize() -> libc::c_int;
    fn Pl_Fatal_Error(format: *mut libc::c_char, _: ...);
    fn Pl_M_Sys_Err_String(ret_val: libc::c_int) -> *mut libc::c_char;
    static mut pl_fixed_sizes: PlLong;
    static mut pl_stk_tbl: [InfStack; 0];
    fn mmap(
        __addr: *mut libc::c_void,
        __len: size_t,
        __prot: libc::c_int,
        __flags: libc::c_int,
        __fd: libc::c_int,
        __offset: __off_t,
    ) -> *mut libc::c_void;
    fn munmap(__addr: *mut libc::c_void, __len: size_t) -> libc::c_int;
    fn mprotect(
        __addr: *mut libc::c_void,
        __len: size_t,
        __prot: libc::c_int,
    ) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __uint32_t = libc::c_uint;
pub type __uid_t = libc::c_uint;
pub type __off_t = libc::c_long;
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
pub type intptr_t = libc::c_long;
pub type uintptr_t = libc::c_ulong;
pub type PlLong = intptr_t;
pub type PlULong = uintptr_t;
pub type WamWord = intptr_t;
pub type WamWordP = *mut WamWord;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct InfStack {
    pub name: *mut libc::c_char,
    pub desc: *mut libc::c_char,
    pub env_var_name: *mut libc::c_char,
    pub p_def_size: *mut PlLong,
    pub default_size: libc::c_int,
    pub size: libc::c_int,
    pub stack: *mut WamWord,
}
pub type SegvHdlr = Option::<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int>;
pub static mut pl_reg_bank: *mut WamWord = 0 as *const WamWord as *mut WamWord;
pub static mut TR: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut B: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut H: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut pl_def_max_atom: PlLong = 0;
static mut page_size: libc::c_int = 0;
static mut tbl_handler: [SegvHdlr; 10] = [None; 10];
static mut nb_handler: libc::c_int = 0 as libc::c_int;
unsafe extern "C" fn Virtual_Mem_Alloc(
    mut addr: *mut libc::c_void,
    mut length: libc::c_int,
) -> *mut libc::c_void {
    addr = mmap(
        addr,
        length as size_t,
        0x1 as libc::c_int | 0x2 as libc::c_int,
        0x2 as libc::c_int | 0x20 as libc::c_int,
        -(1 as libc::c_int),
        0 as libc::c_int as __off_t,
    );
    if addr == -(1 as libc::c_int) as *mut libc::c_void {
        addr = 0 as *mut libc::c_void;
    }
    return addr;
}
unsafe extern "C" fn Virtual_Mem_Protect(
    mut addr: *mut libc::c_void,
    mut length: libc::c_int,
) {
    let mut end: *mut WamWord = addr as *mut WamWord;
    if mprotect(addr, length as size_t, 0 as libc::c_int) == -(1 as libc::c_int) {
        if munmap(addr, length as size_t) == -(1 as libc::c_int) {
            Pl_Fatal_Error(
                b"unmap failed : %s\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                Pl_M_Sys_Err_String(-(1 as libc::c_int)),
            );
        }
    }
    *end.offset(-(16 as libc::c_int) as isize) = 0x12345678 as libc::c_int as WamWord;
    *end.offset(-(32 as libc::c_int) as isize) = 0xdeadbeef as libc::c_uint as WamWord;
    *end.offset(-(33 as libc::c_int) as isize) = 0 as libc::c_int as WamWord;
}
pub unsafe extern "C" fn Pl_Allocate_Stacks() {
    let mut length: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut stk_sz: libc::c_uint = 0;
    let mut addr: *mut WamWord = 0 as *mut WamWord;
    let mut i: libc::c_int = 0;
    let mut addr_to_try: [*mut WamWord; 3] = [
        0 as *mut WamWord,
        0x4000000000 as libc::c_ulonglong as *mut WamWord,
        -(1 as libc::c_int) as *mut WamWord,
    ];
    page_size = (getpagesize() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<WamWord>() as libc::c_ulong) as libc::c_int;
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        let ref mut fresh0 = (*pl_stk_tbl.as_mut_ptr().offset(i as isize)).size;
        *fresh0 = ((*pl_stk_tbl.as_mut_ptr().offset(i as isize)).size + page_size
            - 1 as libc::c_int) / page_size * page_size;
        stk_sz = *fresh0 as libc::c_uint;
        if stk_sz == 0 as libc::c_int as libc::c_uint {
            stk_sz = page_size as libc::c_uint;
        }
        length = length.wrapping_add(stk_sz.wrapping_add(page_size as libc::c_uint));
        i += 1;
        i;
    }
    length = (length as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<WamWord>() as libc::c_ulong) as libc::c_uint
        as libc::c_uint;
    addr = 0 as *mut WamWord;
    i = 0 as libc::c_int;
    while addr.is_null()
        && addr_to_try[i as usize] != -(1 as libc::c_int) as *mut WamWord
    {
        addr = addr_to_try[i as usize];
        if !addr.is_null() {
            addr = (addr as PlULong)
                .wrapping_div(getpagesize() as libc::c_ulong)
                .wrapping_mul(getpagesize() as libc::c_ulong) as *mut WamWord;
            addr = (addr as PlULong).wrapping_sub(length as libc::c_ulong)
                as *mut WamWord;
        }
        addr = Virtual_Mem_Alloc(addr as *mut libc::c_void, length as libc::c_int)
            as *mut WamWord;
        i += 1;
        i;
    }
    if addr.is_null() {
        Pl_Fatal_Error(
            b"Memory allocation fault\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        let ref mut fresh1 = (*pl_stk_tbl.as_mut_ptr().offset(i as isize)).stack;
        *fresh1 = addr;
        stk_sz = (*pl_stk_tbl.as_mut_ptr().offset(i as isize)).size as libc::c_uint;
        if stk_sz == 0 as libc::c_int as libc::c_uint {
            stk_sz = page_size as libc::c_uint;
        }
        addr = addr.offset(stk_sz as isize);
        Virtual_Mem_Protect(
            addr as *mut libc::c_void,
            (page_size as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<WamWord>() as libc::c_ulong)
                as libc::c_int,
        );
        addr = addr.offset(page_size as isize);
        i += 1;
        i;
    }
    Install_SIGSEGV_Handler();
    Pl_Push_SIGSEGV_Handler(
        Some(
            Default_SIGSEGV_Handler
                as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int,
        ),
    );
}
unsafe extern "C" fn SIGSEGV_Handler(mut sig: libc::c_int, mut sip: *mut siginfo_t) {
    let mut addr: *mut libc::c_void = (*sip)._sifields._sigfault.si_addr;
    Handle_Bad_Address(addr);
}
unsafe extern "C" fn Install_SIGSEGV_Handler() {
    let mut act: sigaction = sigaction {
        __sigaction_handler: C2RustUnnamed_9 {
            sa_handler: None,
        },
        sa_mask: __sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    act
        .__sigaction_handler
        .sa_sigaction = ::std::mem::transmute::<
        Option::<unsafe extern "C" fn() -> ()>,
        Option::<
            unsafe extern "C" fn(libc::c_int, *mut siginfo_t, *mut libc::c_void) -> (),
        >,
    >(
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(libc::c_int, *mut siginfo_t) -> ()>,
            Option::<unsafe extern "C" fn() -> ()>,
        >(
            Some(
                SIGSEGV_Handler
                    as unsafe extern "C" fn(libc::c_int, *mut siginfo_t) -> (),
            ),
        ),
    );
    sigemptyset(&mut act.sa_mask);
    act.sa_flags = 4 as libc::c_int | 0x10000000 as libc::c_int;
    sigaction(11 as libc::c_int, &mut act, 0 as *mut sigaction);
    sigaction(7 as libc::c_int, &mut act, 0 as *mut sigaction);
}
unsafe extern "C" fn Handle_Bad_Address(mut bad_addr: *mut libc::c_void) {
    let mut i: libc::c_int = nb_handler;
    loop {
        i -= 1;
        if !(i >= 0 as libc::c_int) {
            break;
        }
        if (Some((*tbl_handler.as_mut_ptr().offset(i as isize)).unwrap()))
            .unwrap()(bad_addr) != 0
        {
            return;
        }
    }
    Pl_Fatal_Error(
        b"Segmentation Violation at: %p\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        bad_addr,
    );
    exit(1 as libc::c_int);
}
pub unsafe extern "C" fn Pl_Push_SIGSEGV_Handler(mut handler: SegvHdlr) {
    if nb_handler >= 10 as libc::c_int {
        Pl_Fatal_Error(
            b"too many SIGSEGV handlers (max: %d)\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            10 as libc::c_int,
        );
    }
    let fresh2 = nb_handler;
    nb_handler = nb_handler + 1;
    tbl_handler[fresh2 as usize] = handler;
}
pub unsafe extern "C" fn Pl_Pop_SIGSEGV_Handler() {
    if nb_handler > 0 as libc::c_int {
        nb_handler -= 1;
        nb_handler;
    }
}
unsafe extern "C" fn Default_SIGSEGV_Handler(
    mut bad_addr: *mut libc::c_void,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut addr: *mut WamWord = bad_addr as *mut WamWord;
    i = 4 as libc::c_int - 1 as libc::c_int;
    if addr
        < ((*pl_stk_tbl.as_mut_ptr().offset(i as isize)).stack)
            .offset((*pl_stk_tbl.as_mut_ptr().offset(i as isize)).size as isize)
            .offset(page_size as isize)
    {
        while i >= 0 as libc::c_int {
            if addr
                >= ((*pl_stk_tbl.as_mut_ptr().offset(i as isize)).stack)
                    .offset((*pl_stk_tbl.as_mut_ptr().offset(i as isize)).size as isize)
            {
                Pl_Fatal_Error(Stack_Overflow_Err_Msg(i));
            }
            i -= 1;
            i;
        }
    }
    Pl_Fatal_Error(
        b"Segmentation Violation (bad address: %p)\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        addr,
    );
    return 1 as libc::c_int;
}
unsafe extern "C" fn Stack_Overflow_Err_Msg(
    mut stk_nb: libc::c_int,
) -> *mut libc::c_char {
    let mut s: *mut InfStack = pl_stk_tbl.as_mut_ptr().offset(stk_nb as isize);
    let mut var: *mut libc::c_char = (*s).env_var_name;
    let mut size: libc::c_int = (*s).size;
    let mut usage: libc::c_int = (if stk_nb == 0 as libc::c_int {
        TR
    } else {
        (if stk_nb == 1 as libc::c_int {
            *(pl_reg_bank as *mut WamWordP)
                .offset((256 as libc::c_int + 3 as libc::c_int) as isize)
        } else {
            (if stk_nb == 2 as libc::c_int {
                H
            } else {
                (if B
                    >= *(pl_reg_bank as *mut WamWordP)
                        .offset((256 as libc::c_int + 2 as libc::c_int) as isize)
                {
                    B
                } else {
                    *(pl_reg_bank as *mut WamWordP)
                        .offset((256 as libc::c_int + 2 as libc::c_int) as isize)
                })
            })
        })
    })
        .offset_from((*s).stack) as libc::c_long as libc::c_int;
    static mut msg: [libc::c_char; 256] = [0; 256];
    if (*s).stack == (*pl_stk_tbl.as_mut_ptr().offset(2 as libc::c_int as isize)).stack {
        size += 256 as libc::c_int + 8 as libc::c_int;
    }
    size = (size as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<WamWord>() as libc::c_ulong)
        .wrapping_div(1024 as libc::c_int as libc::c_ulong) as libc::c_int;
    usage = (usage as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<WamWord>() as libc::c_ulong)
        .wrapping_div(1024 as libc::c_int as libc::c_ulong) as libc::c_int;
    if pl_fixed_sizes != 0
        || *var.offset(0 as libc::c_int as isize) as libc::c_int == '\0' as i32
    {
        sprintf(
            msg.as_mut_ptr(),
            b"%s stack overflow (size: %d Kb, reached: %d Kb - fixed size)\0"
                as *const u8 as *const libc::c_char,
            (*s).name,
            size,
            usage,
        );
    } else {
        sprintf(
            msg.as_mut_ptr(),
            b"%s stack overflow (size: %d Kb, reached: %d Kb, environment variable used: %s)\0"
                as *const u8 as *const libc::c_char,
            (*s).name,
            size,
            usage,
            var,
        );
    }
    return msg.as_mut_ptr();
}
