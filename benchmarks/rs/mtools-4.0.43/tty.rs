use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn atexit(__func: Option::<unsafe extern "C" fn() -> ()>) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    static mut stdin: *mut FILE;
    static mut stderr: *mut FILE;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fdopen(__fd: libc::c_int, __modes: *const libc::c_char) -> *mut FILE;
    fn vfprintf(
        _: *mut FILE,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn fgetc(__stream: *mut FILE) -> libc::c_int;
    fn fgets(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn isatty(__fd: libc::c_int) -> libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn tcgetattr(__fd: libc::c_int, __termios_p: *mut termios) -> libc::c_int;
    fn tcsetattr(
        __fd: libc::c_int,
        __optional_actions: libc::c_int,
        __termios_p: *const termios,
    ) -> libc::c_int;
    fn tcflush(__fd: libc::c_int, __queue_selector: libc::c_int) -> libc::c_int;
    fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;
    fn setup_signal();
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type va_list = __builtin_va_list;
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
pub type __sighandler_t = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
pub type Terminal = termios;
static mut tty: *mut FILE = 0 as *const FILE as *mut FILE;
static mut notty: libc::c_int = 0 as libc::c_int;
static mut ttyfd: libc::c_int = -(1 as libc::c_int);
pub static mut mtools_raw_tty: libc::c_int = 1 as libc::c_int;
static mut tty_mode: libc::c_int = -(1 as libc::c_int);
static mut need_tty_reset: libc::c_int = 0 as libc::c_int;
static mut handlerIsSet: libc::c_int = 0 as libc::c_int;
static mut in_orig: Terminal = Terminal {
    c_iflag: 0,
    c_oflag: 0,
    c_cflag: 0,
    c_lflag: 0,
    c_line: 0,
    c_cc: [0; 32],
    c_ispeed: 0,
    c_ospeed: 0,
};
unsafe extern "C" fn tty_time_out(mut dummy: libc::c_int) -> ! {
    let mut exit_code: libc::c_int = 0;
    signal(
        14 as libc::c_int,
        ::std::mem::transmute::<
            libc::intptr_t,
            __sighandler_t,
        >(1 as libc::c_int as libc::intptr_t),
    );
    if !tty.is_null() && need_tty_reset != 0 {
        tcsetattr(ttyfd, 0 as libc::c_int, &mut in_orig);
    }
    exit_code = 0 as libc::c_int;
    exit(exit_code);
}
unsafe extern "C" fn cleanup_tty() {
    if !tty.is_null() && need_tty_reset != 0 {
        tcsetattr(ttyfd, 0 as libc::c_int, &mut in_orig);
        setup_signal();
    }
}
unsafe extern "C" fn set_raw_tty(mut mode: libc::c_int) {
    let mut in_raw: Terminal = Terminal {
        c_iflag: 0,
        c_oflag: 0,
        c_cflag: 0,
        c_lflag: 0,
        c_line: 0,
        c_cc: [0; 32],
        c_ispeed: 0,
        c_ospeed: 0,
    };
    if mode != tty_mode && mode != -(1 as libc::c_int) {
        if handlerIsSet == 0 {
            tcgetattr(ttyfd, &mut in_orig);
            need_tty_reset = 1 as libc::c_int;
            atexit(Some(cleanup_tty as unsafe extern "C" fn() -> ()));
            handlerIsSet = 1 as libc::c_int;
        }
        setup_signal();
        signal(
            14 as libc::c_int,
            ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(libc::c_int) -> !>,
                __sighandler_t,
            >(Some(tty_time_out as unsafe extern "C" fn(libc::c_int) -> !)),
        );
        tcgetattr(ttyfd, &mut in_raw);
        if mode != 0 {
            in_raw.c_lflag &= !(0 as libc::c_uint) ^ 0o2 as libc::c_int as libc::c_uint;
            in_raw.c_cc[6 as libc::c_int as usize] = 1 as libc::c_int as cc_t;
            in_raw.c_cc[5 as libc::c_int as usize] = 0 as libc::c_int as cc_t;
            tcsetattr(ttyfd, 0 as libc::c_int, &mut in_raw);
        } else {
            in_raw.c_lflag |= 0o2 as libc::c_int as libc::c_uint;
            tcsetattr(ttyfd, 0 as libc::c_int, &mut in_raw);
        }
        tty_mode = mode;
        tcflush(ttyfd, 0 as libc::c_int);
    }
}
pub unsafe extern "C" fn opentty(mut mode: libc::c_int) -> *mut FILE {
    if notty != 0 {
        return 0 as *mut FILE;
    }
    if tty.is_null() {
        ttyfd = open(
            b"/dev/tty\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
        if ttyfd >= 0 as libc::c_int {
            tty = fdopen(ttyfd, b"r\0" as *const u8 as *const libc::c_char);
        }
    }
    if tty.is_null() {
        if isatty(0 as libc::c_int) == 0 {
            notty = 1 as libc::c_int;
            return 0 as *mut FILE;
        }
        ttyfd = 0 as libc::c_int;
        tty = stdin;
    }
    if mtools_raw_tty != 0 {
        set_raw_tty(mode);
    }
    return tty;
}
pub unsafe extern "C" fn ask_confirmation(
    mut format: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut ans: [libc::c_char; 10] = [0; 10];
    let mut ap: ::std::ffi::VaListImpl;
    if (opentty(-(1 as libc::c_int))).is_null() {
        return 0 as libc::c_int;
    }
    loop {
        ap = args.clone();
        vfprintf(stderr, format, ap.as_va_list());
        fflush(stderr);
        fflush(opentty(-(1 as libc::c_int)));
        if mtools_raw_tty != 0 {
            let mut c: libc::c_int = fgetc(opentty(1 as libc::c_int));
            if c < 0 as libc::c_int {
                ans[0 as libc::c_int as usize] = 'n' as i32 as libc::c_char;
            } else {
                ans[0 as libc::c_int as usize] = c as libc::c_char;
            }
            fputs(b"\n\0" as *const u8 as *const libc::c_char, stderr);
        } else if (fgets(ans.as_mut_ptr(), 9 as libc::c_int, opentty(0 as libc::c_int)))
            .is_null()
        {
            ans[0 as libc::c_int as usize] = 'n' as i32 as libc::c_char;
        }
        if ans[0 as libc::c_int as usize] as libc::c_int == 'y' as i32
            || ans[0 as libc::c_int as usize] as libc::c_int == 'Y' as i32
        {
            return 0 as libc::c_int;
        }
        if ans[0 as libc::c_int as usize] as libc::c_int == 'n' as i32
            || ans[0 as libc::c_int as usize] as libc::c_int == 'N' as i32
        {
            return -(1 as libc::c_int);
        }
    };
}
