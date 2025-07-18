use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn sigemptyset(__set: *mut sigset_t) -> libc::c_int;
    fn sigaction(
        __sig: libc::c_int,
        __act: *const sigaction,
        __oact: *mut sigaction,
    ) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn ttyname(__fd: libc::c_int) -> *mut libc::c_char;
    fn isatty(__fd: libc::c_int) -> libc::c_int;
    fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, _: ...) -> libc::c_int;
    fn __fxstat(
        __ver: libc::c_int,
        __fildes: libc::c_int,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn tcgetattr(__fd: libc::c_int, __termios_p: *mut termios) -> libc::c_int;
    fn tcsetattr(
        __fd: libc::c_int,
        __optional_actions: libc::c_int,
        __termios_p: *const termios,
    ) -> libc::c_int;
    fn Pl_LE_Get_Prompt_Length() -> libc::c_int;
    fn Pl_LE_Get_Current_Position() -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __uint32_t = libc::c_uint;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __clock_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type ssize_t = __ssize_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
pub type sigset_t = __sigset_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: libc::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct winsize {
    pub ws_row: libc::c_ushort,
    pub ws_col: libc::c_ushort,
    pub ws_xpixel: libc::c_ushort,
    pub ws_ypixel: libc::c_ushort,
}
pub type cc_t = libc::c_uchar;
pub type speed_t = libc::c_uint;
pub type tcflag_t = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct termios {
    pub c_iflag: tcflag_t,
    pub c_oflag: tcflag_t,
    pub c_cflag: tcflag_t,
    pub c_lflag: tcflag_t,
    pub c_line: cc_t,
    pub c_cc: [cc_t; 32],
    pub c_ispeed: speed_t,
    pub c_ospeed: speed_t,
}
pub type TermIO = termios;
pub type C2RustUnnamed_11 = libc::c_uint;
pub const LE_MODE_HOOK: C2RustUnnamed_11 = 2;
pub const LE_MODE_TTY: C2RustUnnamed_11 = 1;
pub const LE_MODE_DEACTIVATED: C2RustUnnamed_11 = 0;
#[inline]
unsafe extern "C" fn fstat(
    mut __fd: libc::c_int,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __fxstat(1 as libc::c_int, __fd, __statbuf);
}
pub static mut pl_le_hook_start: Option::<unsafe extern "C" fn() -> ()> = None;
pub static mut pl_le_hook_put_char: Option::<unsafe extern "C" fn() -> ()> = None;
pub static mut pl_le_hook_get_char0: Option::<unsafe extern "C" fn() -> libc::c_int> = None;
pub static mut pl_le_hook_emit_beep: Option::<unsafe extern "C" fn() -> ()> = None;
pub static mut pl_le_hook_ins_mode: Option::<unsafe extern "C" fn() -> ()> = None;
pub static mut pl_le_hook_screen_size: Option::<unsafe extern "C" fn() -> ()> = None;
pub static mut pl_le_hook_kbd_is_not_empty: Option::<
    unsafe extern "C" fn() -> libc::c_int,
> = None;
pub static mut pl_le_hook_backd: Option::<unsafe extern "C" fn() -> ()> = None;
pub static mut pl_le_hook_forwd: Option::<unsafe extern "C" fn() -> ()> = None;
pub static mut pl_le_hook_displ: Option::<unsafe extern "C" fn() -> ()> = None;
pub static mut pl_le_hook_displ_str: Option::<unsafe extern "C" fn() -> ()> = None;
pub static mut pl_le_hook_erase: Option::<unsafe extern "C" fn() -> ()> = None;
pub static mut pl_le_hook_set_line_buffering: Option::<unsafe extern "C" fn() -> ()> = None;
pub static mut pl_le_hook_get_line_buffering: Option::<
    unsafe extern "C" fn() -> libc::c_int,
> = None;
pub static mut pl_le_hook_flush: Option::<unsafe extern "C" fn() -> ()> = None;
pub static mut pl_le_hook_confirm_box: Option::<unsafe extern "C" fn() -> libc::c_int> = None;
pub static mut pl_le_hook_message_box: Option::<unsafe extern "C" fn() -> ()> = None;
pub static mut pl_le_hook_exit_process: Option::<unsafe extern "C" fn() -> ()> = None;
pub static mut pl_le_initialize: Option::<unsafe extern "C" fn() -> libc::c_int> = unsafe {
    ::std::mem::transmute::<
        Option::<unsafe extern "C" fn() -> libc::c_int>,
        Option::<unsafe extern "C" fn() -> libc::c_int>,
    >(Some(Pl_LE_Initialize as unsafe extern "C" fn() -> libc::c_int))
};
static mut use_linedit: libc::c_int = 0;
static mut use_gui: libc::c_int = 0;
static mut use_ansi: libc::c_int = 0;
static mut fd_in: libc::c_int = 0 as libc::c_int;
static mut fd_out: libc::c_int = -(1 as libc::c_int);
static mut file_dbg: *mut FILE = 0 as *const FILE as *mut FILE;
static mut is_tty_in: libc::c_int = 0;
static mut is_tty_out: libc::c_int = 0;
static mut same_tty: libc::c_int = 0;
static mut old_stty_in: TermIO = TermIO {
    c_iflag: 0,
    c_oflag: 0,
    c_cflag: 0,
    c_lflag: 0,
    c_line: 0,
    c_cc: [0; 32],
    c_ispeed: 0,
    c_ospeed: 0,
};
static mut new_stty_in: TermIO = TermIO {
    c_iflag: 0,
    c_oflag: 0,
    c_cflag: 0,
    c_lflag: 0,
    c_line: 0,
    c_cc: [0; 32],
    c_ispeed: 0,
    c_ospeed: 0,
};
static mut old_stty_out: TermIO = TermIO {
    c_iflag: 0,
    c_oflag: 0,
    c_cflag: 0,
    c_lflag: 0,
    c_line: 0,
    c_cc: [0; 32],
    c_ispeed: 0,
    c_ospeed: 0,
};
static mut new_stty_out: TermIO = TermIO {
    c_iflag: 0,
    c_oflag: 0,
    c_cflag: 0,
    c_lflag: 0,
    c_line: 0,
    c_cc: [0; 32],
    c_ispeed: 0,
    c_ospeed: 0,
};
static mut nb_rows: libc::c_int = 0;
static mut nb_cols: libc::c_int = 0;
static mut term_pos: libc::c_int = 0;
static mut interrupt_key: libc::c_int = 0;
pub unsafe extern "C" fn Pl_LE_Initialize() -> libc::c_int {
    static mut initialized: libc::c_int = 0 as libc::c_int;
    static mut le_mode: libc::c_int = 0;
    if initialized != 0 {
        return le_mode;
    }
    initialized = 1 as libc::c_int;
    Parse_Env_Var();
    if use_linedit == 0 {
        le_mode = LE_MODE_DEACTIVATED as libc::c_int;
        return le_mode;
    }
    le_mode = LE_MODE_TTY as libc::c_int;
    Choose_Fd_Out();
    if pl_le_hook_start.is_some() && use_gui != 0 {
        ::std::mem::transmute::<
            _,
            fn(_),
        >(
            (Some(pl_le_hook_start.unwrap())).unwrap(),
        )((use_gui == 2 as libc::c_int) as libc::c_int);
    }
    if pl_le_hook_put_char.is_some() && pl_le_hook_get_char0.is_some()
        && pl_le_hook_kbd_is_not_empty.is_some() && pl_le_hook_screen_size.is_some()
    {
        le_mode = LE_MODE_HOOK as libc::c_int;
    } else {
        pl_le_hook_put_char = None;
        pl_le_hook_get_char0 = None;
        pl_le_hook_kbd_is_not_empty = None;
        pl_le_hook_screen_size = None;
    }
    if pl_le_hook_screen_size.is_none() {
        pl_le_hook_screen_size = ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut libc::c_int, *mut libc::c_int) -> ()>,
            Option::<unsafe extern "C" fn() -> ()>,
        >(
            Some(
                Pl_LE_Screen_Size
                    as unsafe extern "C" fn(*mut libc::c_int, *mut libc::c_int) -> (),
            ),
        );
    }
    if pl_le_hook_kbd_is_not_empty.is_none() {
        pl_le_hook_kbd_is_not_empty = ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> libc::c_int>,
            Option::<unsafe extern "C" fn() -> libc::c_int>,
        >(Some(Pl_LE_Kbd_Is_Not_Empty as unsafe extern "C" fn() -> libc::c_int));
    }
    if pl_le_hook_put_char.is_none() {
        pl_le_hook_put_char = ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
            Option::<unsafe extern "C" fn() -> ()>,
        >(Some(Pl_LE_Put_Char as unsafe extern "C" fn(libc::c_int) -> ()));
    }
    if pl_le_hook_get_char0.is_none() {
        pl_le_hook_get_char0 = ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> libc::c_int>,
            Option::<unsafe extern "C" fn() -> libc::c_int>,
        >(Some(LE_Get_Char0 as unsafe extern "C" fn() -> libc::c_int));
    }
    if pl_le_hook_ins_mode.is_none() {
        pl_le_hook_ins_mode = ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
            Option::<unsafe extern "C" fn() -> ()>,
        >(Some(Pl_LE_Ins_Mode as unsafe extern "C" fn(libc::c_int) -> ()));
    }
    if pl_le_hook_emit_beep.is_none() {
        pl_le_hook_emit_beep = ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> ()>,
            Option::<unsafe extern "C" fn() -> ()>,
        >(Some(Pl_LE_Emit_Beep as unsafe extern "C" fn() -> ()));
    }
    if pl_le_hook_backd.is_none() {
        pl_le_hook_backd = ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
            Option::<unsafe extern "C" fn() -> ()>,
        >(Some(Backd as unsafe extern "C" fn(libc::c_int) -> ()));
    }
    if pl_le_hook_forwd.is_none() {
        pl_le_hook_forwd = ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(libc::c_int, *mut libc::c_char) -> ()>,
            Option::<unsafe extern "C" fn() -> ()>,
        >(Some(Forwd as unsafe extern "C" fn(libc::c_int, *mut libc::c_char) -> ()));
    }
    if pl_le_hook_displ.is_none() {
        pl_le_hook_displ = ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(libc::c_int, *mut libc::c_char) -> ()>,
            Option::<unsafe extern "C" fn() -> ()>,
        >(Some(Displ as unsafe extern "C" fn(libc::c_int, *mut libc::c_char) -> ()));
    }
    if pl_le_hook_erase.is_none() {
        pl_le_hook_erase = ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
            Option::<unsafe extern "C" fn() -> ()>,
        >(Some(Erase as unsafe extern "C" fn(libc::c_int) -> ()));
    }
    if pl_le_hook_displ_str.is_none() {
        pl_le_hook_displ_str = ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut libc::c_char) -> ()>,
            Option::<unsafe extern "C" fn() -> ()>,
        >(Some(Displ_Str as unsafe extern "C" fn(*mut libc::c_char) -> ()));
    }
    return le_mode;
}
unsafe extern "C" fn Parse_Env_Var() {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut r: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut buff: [libc::c_char; 1024] = [0; 1024];
    use_ansi = 1 as libc::c_int;
    use_gui = use_ansi;
    use_linedit = use_gui;
    p = getenv(b"LINEDIT\0" as *const u8 as *const libc::c_char);
    if p.is_null() {
        return;
    }
    if strncmp(
        p,
        b"no\0" as *const u8 as *const libc::c_char,
        2 as libc::c_int as libc::c_ulong,
    ) == 0 as libc::c_int
    {
        use_linedit = 0 as libc::c_int;
        return;
    }
    if !(strstr(p, b"gui=n\0" as *const u8 as *const libc::c_char)).is_null() {
        use_gui = 0 as libc::c_int;
    }
    if !(strstr(p, b"gui=s\0" as *const u8 as *const libc::c_char)).is_null() {
        use_gui = 2 as libc::c_int;
    }
    if !(strstr(p, b"ansi=n\0" as *const u8 as *const libc::c_char)).is_null() {
        use_ansi = 0 as libc::c_int;
    }
    q = strstr(p, b"out=\0" as *const u8 as *const libc::c_char);
    if !q.is_null() {
        q = q.offset(4 as libc::c_int as isize);
        if *(*__ctype_b_loc()).offset(*p as libc::c_int as isize) as libc::c_int
            & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
        {
            fd_out = strtol(p, 0 as *mut *mut libc::c_char, 10 as libc::c_int)
                as libc::c_int;
        } else {
            r = buff.as_mut_ptr();
            while *q as libc::c_int != 0
                && *(*__ctype_b_loc()).offset(*q as libc::c_int as isize) as libc::c_int
                    & _ISprint as libc::c_int as libc::c_ushort as libc::c_int != 0
                && *(*__ctype_b_loc()).offset(*q as libc::c_int as isize) as libc::c_int
                    & _ISspace as libc::c_int as libc::c_ushort as libc::c_int == 0
            {
                let fresh0 = r;
                r = r.offset(1);
                let fresh1 = q;
                q = q.offset(1);
                *fresh1 = *fresh0;
            }
            *r = '\0' as i32 as libc::c_char;
            fd_out = open(buff.as_mut_ptr(), 0o1 as libc::c_int);
        }
    }
    q = strstr(p, b"dbg=\0" as *const u8 as *const libc::c_char);
    if !q.is_null() {
        q = q.offset(4 as libc::c_int as isize);
        r = buff.as_mut_ptr();
        while *q as libc::c_int != 0
            && *(*__ctype_b_loc()).offset(*q as libc::c_int as isize) as libc::c_int
                & _ISprint as libc::c_int as libc::c_ushort as libc::c_int != 0
            && *(*__ctype_b_loc()).offset(*q as libc::c_int as isize) as libc::c_int
                & _ISspace as libc::c_int as libc::c_ushort as libc::c_int == 0
        {
            let fresh2 = q;
            q = q.offset(1);
            let fresh3 = r;
            r = r.offset(1);
            *fresh3 = *fresh2;
        }
        *r = '\0' as i32 as libc::c_char;
        file_dbg = fopen(buff.as_mut_ptr(), b"wt\0" as *const u8 as *const libc::c_char);
    }
}
pub unsafe extern "C" fn Pl_LE_Open_Terminal() {
    fflush(stdout);
    fflush(stderr);
    is_tty_in = (tcgetattr(fd_in, &mut old_stty_in) == 0) as libc::c_int;
    is_tty_out = (tcgetattr(fd_out, &mut old_stty_out) == 0) as libc::c_int;
    if is_tty_in != 0 {
        interrupt_key = old_stty_in.c_cc[0 as libc::c_int as usize] as libc::c_int;
        Set_TTY_Mode(&mut old_stty_in, &mut new_stty_in);
        tcsetattr(fd_in, 0 as libc::c_int, &mut new_stty_in);
    } else {
        interrupt_key = 'C' as i32 & 0x1f as libc::c_int;
    }
    if is_tty_out != 0 {
        Set_TTY_Mode(&mut old_stty_out, &mut new_stty_out);
        tcsetattr(fd_out, 0 as libc::c_int, &mut new_stty_out);
    }
    Pl_LE_Screen_Size(&mut nb_rows, &mut nb_cols);
    same_tty = Same_File(fd_in, fd_out);
    Install_Resize_Handler();
    term_pos = 0 as libc::c_int;
}
pub unsafe extern "C" fn Pl_LE_Close_Terminal() {
    if is_tty_in != 0 {
        tcsetattr(fd_in, 0 as libc::c_int, &mut old_stty_in);
    }
    if is_tty_out != 0 {
        tcsetattr(fd_out, 0 as libc::c_int, &mut old_stty_out);
    }
}
unsafe extern "C" fn Choose_Fd_Out() {
    let mut fd: [libc::c_int; 3] = [
        1 as libc::c_int,
        0 as libc::c_int,
        2 as libc::c_int,
    ];
    let mut i: libc::c_int = 0;
    let mut try_0: libc::c_int = 0;
    let mut mask: libc::c_int = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int && fd_out < 0 as libc::c_int {
        try_0 = fd[i as usize];
        if !(isatty(try_0) == 0) {
            mask = fcntl(try_0, 3 as libc::c_int);
            if mask & 0o1 as libc::c_int == 0o1 as libc::c_int
                || mask & 0o2 as libc::c_int == 0o2 as libc::c_int
            {
                fd_out = try_0;
                break;
            } else {
                p = ttyname(try_0);
                if !p.is_null() {
                    fd_out = open(p, 0o1 as libc::c_int);
                    break;
                }
            }
        }
        i += 1;
        i;
    }
    if fd_out < 0 as libc::c_int {
        fd_out = 1 as libc::c_int;
    }
}
unsafe extern "C" fn Same_File(
    mut fd1: libc::c_int,
    mut fd2: libc::c_int,
) -> libc::c_int {
    let mut stat1: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    let mut stat2: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    return (fstat(fd1, &mut stat1) != -(1 as libc::c_int)
        && fstat(fd2, &mut stat2) != -(1 as libc::c_int) && stat1.st_dev == stat2.st_dev
        && stat1.st_ino == stat2.st_ino) as libc::c_int;
}
unsafe extern "C" fn Set_TTY_Mode(mut old: *mut TermIO, mut new: *mut TermIO) {
    *new = *old;
    (*new).c_iflag
        &= !(0o100 as libc::c_int | 0o200 as libc::c_int | 0o400 as libc::c_int
            | 0o2000 as libc::c_int | 0o10000 as libc::c_int) as libc::c_uint;
    (*new).c_oflag = (0o1 as libc::c_int | 0o4 as libc::c_int) as tcflag_t;
    (*new).c_lflag
        &= !(0o2 as libc::c_int | 0o10 as libc::c_int | 0o100 as libc::c_int)
            as libc::c_uint;
    (*new).c_cc[6 as libc::c_int as usize] = 1 as libc::c_int as cc_t;
    (*new).c_cc[5 as libc::c_int as usize] = 1 as libc::c_int as cc_t;
    (*new).c_cc[0 as libc::c_int as usize] = -(1 as libc::c_int) as cc_t;
}
unsafe extern "C" fn Install_Resize_Handler() {
    let mut act: sigaction = sigaction {
        __sigaction_handler: C2RustUnnamed_10 {
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
            Option::<unsafe extern "C" fn() -> ()>,
            Option::<unsafe extern "C" fn() -> ()>,
        >(
            Some(
                ::std::mem::transmute::<
                    unsafe extern "C" fn() -> (),
                    unsafe extern "C" fn() -> (),
                >(Resize_Handler),
            ),
        ),
    );
    sigemptyset(&mut act.sa_mask);
    act.sa_flags = 0x10000000 as libc::c_int;
    sigaction(28 as libc::c_int, &mut act, 0 as *mut sigaction);
}
unsafe extern "C" fn Resize_Handler() {
    let mut r: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    Pl_LE_Screen_Size(&mut r, &mut c);
    if c > 0 as libc::c_int && r > 0 as libc::c_int && (r != nb_rows || c != nb_cols) {
        nb_rows = r;
        nb_cols = c;
        term_pos = (Pl_LE_Get_Current_Position() + Pl_LE_Get_Prompt_Length()) % nb_cols;
    }
}
pub unsafe extern "C" fn Pl_LE_Screen_Size(
    mut row: *mut libc::c_int,
    mut col: *mut libc::c_int,
) {
    let mut ws: winsize = winsize {
        ws_row: 0,
        ws_col: 0,
        ws_xpixel: 0,
        ws_ypixel: 0,
    };
    if is_tty_out == 0 {
        *col = 0 as libc::c_int;
        *row = *col;
        return;
    }
    ws.ws_col = 0 as libc::c_int as libc::c_ushort;
    ws.ws_row = ws.ws_col;
    if ioctl(fd_out, 0x5413 as libc::c_int as libc::c_ulong, &mut ws as *mut winsize)
        == -(1 as libc::c_int) || ws.ws_row as libc::c_int == 0 as libc::c_int
        || ws.ws_col as libc::c_int == 0 as libc::c_int
    {
        let mut fd: libc::c_int = open(
            b"/dev/tty\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
        if fd != -(1 as libc::c_int) {
            ioctl(fd, 0x5413 as libc::c_int as libc::c_ulong, &mut ws as *mut winsize);
            close(fd);
        }
    }
    *row = ws.ws_row as libc::c_int;
    *col = ws.ws_col as libc::c_int;
}
pub unsafe extern "C" fn Pl_LE_Is_Interrupt_Key(mut c: libc::c_int) -> libc::c_int {
    return (c == interrupt_key) as libc::c_int;
}
pub unsafe extern "C" fn Pl_LE_Kbd_Is_Not_Empty() -> libc::c_int {
    let mut nb_not_read: libc::c_int = 0;
    ioctl(
        fd_in,
        0x541b as libc::c_int as libc::c_ulong,
        &mut nb_not_read as *mut libc::c_int,
    );
    return (nb_not_read != 0 as libc::c_int) as libc::c_int;
}
pub unsafe extern "C" fn Pl_LE_Ins_Mode(mut ins_mode: libc::c_int) {}
pub unsafe extern "C" fn Pl_LE_Emit_Beep() {
    Pl_LE_Put_Char('\u{7}' as i32);
}
pub unsafe extern "C" fn Pl_LE_Put_Char(mut c: libc::c_int) {
    static mut buf: [libc::c_char; 20] = [0; 20];
    let mut n: libc::c_int = 0;
    buf[0 as libc::c_int as usize] = c as libc::c_char;
    buf[1 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
    if use_ansi != 0 {
        if same_tty == 0 {
            Resize_Handler();
        }
        match c {
            8 => {
                if term_pos == 0 as libc::c_int {
                    term_pos = nb_cols - 1 as libc::c_int;
                    sprintf(
                        buf.as_mut_ptr(),
                        b"\x1B[A\x1B[%dC\0" as *const u8 as *const libc::c_char,
                        term_pos,
                    );
                } else {
                    term_pos -= 1;
                    term_pos;
                }
            }
            7 => {}
            10 => {
                term_pos = 0 as libc::c_int;
            }
            _ => {
                term_pos += 1;
                if term_pos >= nb_cols && nb_cols > 0 as libc::c_int {
                    buf[1 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
                    buf[2 as libc::c_int as usize] = '\u{8}' as i32 as libc::c_char;
                    buf[3 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
                    term_pos = 0 as libc::c_int;
                }
            }
        }
    }
    n = strlen(buf.as_mut_ptr()) as libc::c_int;
    write(fd_out, buf.as_mut_ptr() as *const libc::c_void, strlen(buf.as_mut_ptr()))
        != n as libc::c_long;
}
pub unsafe extern "C" fn Pl_LE_Get_Char() -> libc::c_int {
    let mut c: libc::c_int = ::std::mem::transmute::<
        _,
        fn() -> libc::c_int,
    >((Some(pl_le_hook_get_char0.unwrap())).unwrap())();
    if c == 0x1b as libc::c_int {
        let mut esc_c: libc::c_int = ::std::mem::transmute::<
            _,
            fn() -> libc::c_int,
        >((Some(pl_le_hook_get_char0.unwrap())).unwrap())();
        let mut modif: libc::c_int = 0 as libc::c_int;
        let mut double_bracket: libc::c_int = 0 as libc::c_int;
        let mut number: [libc::c_int; 2] = [0 as libc::c_int, 0 as libc::c_int];
        let mut idx_number: libc::c_int = 0 as libc::c_int;
        if esc_c == 0x1b as libc::c_int {
            modif = 2 as libc::c_int;
            esc_c = ::std::mem::transmute::<
                _,
                fn() -> libc::c_int,
            >((Some(pl_le_hook_get_char0.unwrap())).unwrap())();
        }
        if esc_c == '[' as i32 || esc_c == 'O' as i32 {
            esc_c = ::std::mem::transmute::<
                _,
                fn() -> libc::c_int,
            >((Some(pl_le_hook_get_char0.unwrap())).unwrap())();
            if esc_c == '[' as i32 {
                esc_c = ::std::mem::transmute::<
                    _,
                    fn() -> libc::c_int,
                >((Some(pl_le_hook_get_char0.unwrap())).unwrap())();
                double_bracket = 1 as libc::c_int;
            }
            while *(*__ctype_b_loc()).offset(esc_c as isize) as libc::c_int
                & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
                || esc_c == ';' as i32
            {
                if esc_c == ';' as i32 {
                    idx_number = 1 as libc::c_int - idx_number;
                } else {
                    number[idx_number
                        as usize] = number[idx_number as usize] * 10 as libc::c_int
                        + esc_c - '0' as i32;
                }
                esc_c = ::std::mem::transmute::<
                    _,
                    fn() -> libc::c_int,
                >((Some(pl_le_hook_get_char0.unwrap())).unwrap())();
            }
            c = number[0 as libc::c_int as usize];
            if number[1 as libc::c_int as usize] != 0 {
                modif |= number[1 as libc::c_int as usize] - 1 as libc::c_int;
            }
            if *(*__ctype_b_loc()).offset(esc_c as isize) as libc::c_int
                & _ISupper as libc::c_int as libc::c_ushort as libc::c_int != 0
            {
                if double_bracket != 0 {
                    c = esc_c - 'A' as i32 + 11 as libc::c_int;
                } else if esc_c >= 'P' as i32 {
                    c = esc_c - 'P' as i32 + 11 as libc::c_int;
                } else {
                    c = esc_c;
                }
            } else if esc_c == '^' as i32 {
                modif |= 4 as libc::c_int;
            } else if esc_c == '$' as i32 {
                modif |= 1 as libc::c_int;
            }
            if c == 1 as libc::c_int {
                c = 'H' as i32;
            } else if c == 4 as libc::c_int {
                c = 'F' as i32;
            } else if c >= 25 as libc::c_int && c <= 36 as libc::c_int {
                c = c
                    - (if c <= 26 as libc::c_int || c == 29 as libc::c_int {
                        12 as libc::c_int
                    } else {
                        13 as libc::c_int
                    });
                modif |= 1 as libc::c_int;
            }
            c = modif << 9 as libc::c_int
                | ((1 as libc::c_int) << 8 as libc::c_int | c & 0x7f as libc::c_int);
        } else {
            c = (0 as libc::c_int) << 9 as libc::c_int
                | ((1 as libc::c_int) << 8 as libc::c_int
                    | ((1 as libc::c_int) << 7 as libc::c_int | esc_c
                        | 0x20 as libc::c_int) & 0x7f as libc::c_int);
        }
    }
    return c;
}
unsafe extern "C" fn LE_Get_Char0() -> libc::c_int {
    let mut c: libc::c_uchar = 0;
    if read(
        fd_in,
        &mut c as *mut libc::c_uchar as *mut libc::c_void,
        1 as libc::c_int as size_t,
    ) != 1 as libc::c_int as libc::c_long
    {
        return 'D' as i32 & 0x1f as libc::c_int;
    }
    return c as libc::c_int;
}
unsafe extern "C" fn Backd(mut n: libc::c_int) {
    loop {
        let fresh4 = n;
        n = n - 1;
        if !(fresh4 != 0) {
            break;
        }
        ::std::mem::transmute::<
            _,
            fn(_),
        >((Some(pl_le_hook_put_char.unwrap())).unwrap())('\u{8}' as i32);
    };
}
unsafe extern "C" fn Forwd(mut n: libc::c_int, mut str: *mut libc::c_char) {
    loop {
        let fresh5 = n;
        n = n - 1;
        if !(fresh5 != 0) {
            break;
        }
        let fresh6 = str;
        str = str.offset(1);
        ::std::mem::transmute::<
            _,
            fn(_),
        >((Some(pl_le_hook_put_char.unwrap())).unwrap())(*fresh6 as libc::c_int);
    };
}
unsafe extern "C" fn Displ(mut n: libc::c_int, mut str: *mut libc::c_char) {
    loop {
        let fresh7 = n;
        n = n - 1;
        if !(fresh7 != 0) {
            break;
        }
        let fresh8 = str;
        str = str.offset(1);
        ::std::mem::transmute::<
            _,
            fn(_),
        >((Some(pl_le_hook_put_char.unwrap())).unwrap())(*fresh8 as libc::c_int);
    };
}
unsafe extern "C" fn Erase(mut n: libc::c_int) {
    let mut n0: libc::c_int = n;
    loop {
        let fresh9 = n;
        n = n - 1;
        if !(fresh9 != 0) {
            break;
        }
        ::std::mem::transmute::<
            _,
            fn(_),
        >((Some(pl_le_hook_put_char.unwrap())).unwrap())(' ' as i32);
    }
    ::std::mem::transmute::<_, fn(_)>((Some(pl_le_hook_backd.unwrap())).unwrap())(n0);
}
unsafe extern "C" fn Displ_Str(mut str: *mut libc::c_char) {
    while *str != 0 {
        let fresh10 = str;
        str = str.offset(1);
        ::std::mem::transmute::<
            _,
            fn(_),
        >((Some(pl_le_hook_put_char.unwrap())).unwrap())(*fresh10 as libc::c_int);
    }
}
