use ::libc;
extern "C" {
    fn __errno_location() -> *mut libc::c_int;
    fn longjmp(_: *mut __jmp_buf_tag, _: libc::c_int) -> !;
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
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn exit(_: libc::c_int) -> !;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn isatty(__fd: libc::c_int) -> libc::c_int;
    fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, _: ...) -> libc::c_int;
    fn last_addr() -> libc::c_int;
    fn modified() -> bool_0;
    fn path_max(filename: *const libc::c_char) -> libc::c_int;
    fn write_file(
        filename: *const libc::c_char,
        mode: *const libc::c_char,
        from: libc::c_int,
        to: libc::c_int,
    ) -> libc::c_int;
    fn show_strerror(filename: *const libc::c_char, errcode: libc::c_int);
    fn set_error_msg(msg: *const libc::c_char);
}
pub type __uint32_t = libc::c_uint;
pub type __uid_t = libc::c_uint;
pub type __pid_t = libc::c_int;
pub type __clock_t = libc::c_long;
pub type __jmp_buf = [libc::c_long; 8];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __jmp_buf_tag {
    pub __jmpbuf: __jmp_buf,
    pub __mask_was_saved: libc::c_int,
    pub __saved_mask: __sigset_t,
}
pub type jmp_buf = [__jmp_buf_tag; 1];
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct winsize {
    pub ws_row: libc::c_ushort,
    pub ws_col: libc::c_ushort,
    pub ws_xpixel: libc::c_ushort,
    pub ws_ypixel: libc::c_ushort,
}
pub type Bool = libc::c_uint;
pub const true_0: Bool = 1;
pub const false_0: Bool = 0;
pub type bool_0 = Bool;
static mut mem_msg: *const libc::c_char = b"Memory exhausted\0" as *const u8
    as *const libc::c_char;
pub static mut jmp_state: jmp_buf = [__jmp_buf_tag {
    __jmpbuf: [0; 8],
    __mask_was_saved: 0,
    __saved_mask: __sigset_t { __val: [0; 16] },
}; 1];
static mut mutex: libc::c_int = 0 as libc::c_int;
static mut window_lines_: libc::c_int = 22 as libc::c_int;
static mut window_columns_: libc::c_int = 72 as libc::c_int;
static mut sighup_pending: bool_0 = false_0;
static mut sigint_pending: bool_0 = false_0;
unsafe extern "C" fn sighup_handler(mut signum: libc::c_int) {
    signum != 0;
    if mutex != 0 {
        sighup_pending = true_0;
        return;
    }
    sighup_pending = false_0;
    let hb: [libc::c_char; 7] = *::std::mem::transmute::<
        &[u8; 7],
        &[libc::c_char; 7],
    >(b"ed.hup\0");
    if last_addr() <= 0 as libc::c_int || modified() as u64 == 0
        || write_file(
            hb.as_ptr(),
            b"w\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int,
            last_addr(),
        ) >= 0 as libc::c_int
    {
        exit(0 as libc::c_int);
    }
    let s: *mut libc::c_char = getenv(b"HOME\0" as *const u8 as *const libc::c_char);
    if s.is_null() || *s.offset(0 as libc::c_int as isize) == 0 {
        exit(1 as libc::c_int);
    }
    let len: libc::c_int = strlen(s) as libc::c_int;
    let need_slash: libc::c_int = (*s.offset((len - 1 as libc::c_int) as isize)
        as libc::c_int != '/' as i32) as libc::c_int;
    let hup: *mut libc::c_char = if (len + need_slash
        + ::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong as libc::c_int)
        < path_max(0 as *const libc::c_char)
    {
        malloc(
            ((len + need_slash) as libc::c_ulong)
                .wrapping_add(
                    ::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong,
                ),
        ) as *mut libc::c_char
    } else {
        0 as *mut libc::c_char
    };
    if hup.is_null() {
        exit(1 as libc::c_int);
    }
    memcpy(hup as *mut libc::c_void, s as *const libc::c_void, len as libc::c_ulong);
    if need_slash != 0 {
        *hup.offset(len as isize) = '/' as i32 as libc::c_char;
    }
    memcpy(
        hup.offset(len as isize).offset(need_slash as isize) as *mut libc::c_void,
        hb.as_ptr() as *const libc::c_void,
        ::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong,
    );
    if write_file(
        hup,
        b"w\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
        last_addr(),
    ) >= 0 as libc::c_int
    {
        exit(0 as libc::c_int);
    }
    exit(1 as libc::c_int);
}
unsafe extern "C" fn sigint_handler(mut signum: libc::c_int) {
    if mutex != 0 {
        sigint_pending = true_0;
    } else {
        let mut set: sigset_t = __sigset_t { __val: [0; 16] };
        sigint_pending = false_0;
        sigemptyset(&mut set);
        sigaddset(&mut set, signum);
        sigprocmask(1 as libc::c_int, &mut set, 0 as *mut sigset_t);
        longjmp(jmp_state.as_mut_ptr(), -(1 as libc::c_int));
    };
}
unsafe extern "C" fn sigwinch_handler(mut signum: libc::c_int) {
    let mut ws: winsize = winsize {
        ws_row: 0,
        ws_col: 0,
        ws_xpixel: 0,
        ws_ypixel: 0,
    };
    if ioctl(
        0 as libc::c_int,
        0x5413 as libc::c_int as libc::c_ulong,
        &mut ws as *mut winsize as *mut libc::c_char,
    ) >= 0 as libc::c_int
    {
        if ws.ws_row as libc::c_int > 2 as libc::c_int
            && (ws.ws_row as libc::c_int) < 600 as libc::c_int
        {
            window_lines_ = ws.ws_row as libc::c_int - 2 as libc::c_int;
        }
        if ws.ws_col as libc::c_int > 8 as libc::c_int
            && (ws.ws_col as libc::c_int) < 1800 as libc::c_int
        {
            window_columns_ = ws.ws_col as libc::c_int - 8 as libc::c_int;
        }
    }
    signum != 0;
}
unsafe extern "C" fn set_signal(
    signum: libc::c_int,
    mut handler: Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
) -> libc::c_int {
    let mut new_action: sigaction = sigaction {
        __sigaction_handler: C2RustUnnamed_9 {
            sa_handler: None,
        },
        sa_mask: __sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    new_action.__sigaction_handler.sa_handler = handler;
    sigemptyset(&mut new_action.sa_mask);
    new_action.sa_flags = 0x10000000 as libc::c_int;
    return sigaction(signum, &mut new_action, 0 as *mut sigaction);
}
pub unsafe extern "C" fn enable_interrupts() {
    mutex -= 1;
    if mutex <= 0 as libc::c_int {
        mutex = 0 as libc::c_int;
        if sighup_pending as u64 != 0 {
            sighup_handler(1 as libc::c_int);
        }
        if sigint_pending as u64 != 0 {
            sigint_handler(2 as libc::c_int);
        }
    }
}
pub unsafe extern "C" fn disable_interrupts() {
    mutex += 1;
    mutex;
}
pub unsafe extern "C" fn set_signals() {
    sigwinch_handler(28 as libc::c_int);
    if isatty(0 as libc::c_int) != 0 {
        set_signal(
            28 as libc::c_int,
            Some(sigwinch_handler as unsafe extern "C" fn(libc::c_int) -> ()),
        );
    }
    set_signal(
        1 as libc::c_int,
        Some(sighup_handler as unsafe extern "C" fn(libc::c_int) -> ()),
    );
    set_signal(
        3 as libc::c_int,
        ::std::mem::transmute::<
            libc::intptr_t,
            __sighandler_t,
        >(1 as libc::c_int as libc::intptr_t),
    );
    set_signal(
        2 as libc::c_int,
        Some(sigint_handler as unsafe extern "C" fn(libc::c_int) -> ()),
    );
}
pub unsafe extern "C" fn set_window_lines(lines: libc::c_int) {
    window_lines_ = lines;
}
pub unsafe extern "C" fn window_columns() -> libc::c_int {
    return window_columns_;
}
pub unsafe extern "C" fn window_lines() -> libc::c_int {
    return window_lines_;
}
pub unsafe extern "C" fn resize_buffer(
    buf: *mut *mut libc::c_char,
    size: *mut libc::c_int,
    min_size: libc::c_uint,
) -> bool_0 {
    if (*size as libc::c_uint) < min_size {
        if min_size >= 2147483647 as libc::c_int as libc::c_uint {
            set_error_msg(b"Line too long\0" as *const u8 as *const libc::c_char);
            return false_0;
        }
        let new_size: libc::c_int = (if min_size < 512 as libc::c_int as libc::c_uint {
            512 as libc::c_int as libc::c_uint
        } else if min_size
            >= (2147483647 as libc::c_int / 2 as libc::c_int) as libc::c_uint
        {
            (2147483647 as libc::c_int - 1 as libc::c_int) as libc::c_uint
        } else {
            min_size
                .wrapping_div(512 as libc::c_int as libc::c_uint)
                .wrapping_mul(1024 as libc::c_int as libc::c_uint)
        }) as libc::c_int;
        let mut new_buf: *mut libc::c_void = 0 as *mut libc::c_void;
        disable_interrupts();
        if !(*buf).is_null() {
            new_buf = realloc(*buf as *mut libc::c_void, new_size as libc::c_ulong);
        } else {
            new_buf = malloc(new_size as libc::c_ulong);
        }
        if new_buf.is_null() {
            show_strerror(0 as *const libc::c_char, *__errno_location());
            set_error_msg(mem_msg);
            enable_interrupts();
            return false_0;
        }
        *size = new_size;
        *buf = new_buf as *mut libc::c_char;
        enable_interrupts();
    }
    return true_0;
}
