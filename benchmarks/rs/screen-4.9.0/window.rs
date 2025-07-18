use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn chmod(__file: *const libc::c_char, __mode: __mode_t) -> libc::c_int;
    fn __xstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, _: ...) -> libc::c_int;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn __errno_location() -> *mut libc::c_int;
    fn access(__name: *const libc::c_char, __type: libc::c_int) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn chown(
        __file: *const libc::c_char,
        __owner: __uid_t,
        __group: __gid_t,
    ) -> libc::c_int;
    fn chdir(__path: *const libc::c_char) -> libc::c_int;
    fn dup(__fd: libc::c_int) -> libc::c_int;
    fn execvpe(
        __file: *const libc::c_char,
        __argv: *const *mut libc::c_char,
        __envp: *const *mut libc::c_char,
    ) -> libc::c_int;
    fn setuid(__uid: __uid_t) -> libc::c_int;
    fn setgid(__gid: __gid_t) -> libc::c_int;
    fn fork() -> __pid_t;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    static mut stderr: *mut FILE;
    static mut stdout: *mut FILE;
    fn bcopy(__src: *const libc::c_void, __dest: *mut libc::c_void, __n: size_t);
    fn index(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn rindex(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn tcflush(__fd: libc::c_int, __queue_selector: libc::c_int) -> libc::c_int;
    fn LayPause(layer: *mut layer, pause: libc::c_int);
    fn LayerCleanupMemory(layer: *mut layer);
    fn RethinkViewportOffsets(_: *mut canvas);
    static mut strnomem: [libc::c_char; 0];
    fn Msg(_: libc::c_int, _: *const libc::c_char, _: ...);
    fn Panic(_: libc::c_int, _: *const libc::c_char, _: ...) -> !;
    fn MakeWinMsg(
        _: *mut libc::c_char,
        _: *mut win,
        _: libc::c_int,
    ) -> *mut libc::c_char;
    fn WindowDied(_: *mut win, _: libc::c_int, _: libc::c_int);
    fn ResetWindow(_: *mut win);
    fn WriteString(_: *mut win, _: *mut libc::c_char, _: libc::c_int);
    fn SetCharsets(_: *mut win, _: *mut libc::c_char);
    fn WNewAutoFlow(_: *mut win, _: libc::c_int);
    fn WindowChanged(_: *mut win, _: libc::c_int);
    fn RcLine(_: *mut libc::c_char, _: libc::c_int);
    fn secfopen(_: *mut libc::c_char, _: *mut libc::c_char) -> *mut FILE;
    fn OpenTTY(_: *mut libc::c_char, _: *mut libc::c_char) -> libc::c_int;
    fn InitTTY(_: *mut mode, _: libc::c_int);
    fn SetTTY(_: libc::c_int, _: *mut mode);
    fn SetFlow(_: libc::c_int);
    fn TtyGrabConsole(
        _: libc::c_int,
        _: libc::c_int,
        _: *mut libc::c_char,
    ) -> libc::c_int;
    fn fgtty(_: libc::c_int) -> libc::c_int;
    fn brktty(_: libc::c_int);
    fn FreePaster(_: *mut paster);
    fn Input(
        _: *mut libc::c_char,
        _: libc::c_int,
        _: libc::c_int,
        _: Option::<
            unsafe extern "C" fn(*mut libc::c_char, libc::c_int, *mut libc::c_char) -> (),
        >,
        _: *mut libc::c_char,
        _: libc::c_int,
    );
    fn ZmodemPage();
    fn SetUtmp(_: *mut win) -> libc::c_int;
    fn RemoveUtmp(_: *mut win) -> libc::c_int;
    fn OpenPTY(_: *mut *mut libc::c_char) -> libc::c_int;
    fn InitPTY(_: libc::c_int);
    fn DoProcess(
        _: *mut win,
        _: *mut *mut libc::c_char,
        _: *mut libc::c_int,
        _: *mut paster,
    );
    fn Activate(_: libc::c_int);
    fn KillWindow(_: *mut win);
    fn SetForeWindow(_: *mut win);
    fn MakeTermcap(_: libc::c_int) -> *mut libc::c_char;
    fn PUTCHAR(_: libc::c_int);
    fn ClearAll();
    fn GotoPos(_: libc::c_int, _: libc::c_int);
    fn InsertMode(_: libc::c_int);
    fn KeypadMode(_: libc::c_int);
    fn CursorkeysMode(_: libc::c_int);
    fn ReverseVideo(_: libc::c_int);
    fn CursorVisibility(_: libc::c_int);
    fn MouseMode(_: libc::c_int);
    fn ExtMouseMode(_: libc::c_int);
    fn SetRendition(_: *mut mchar);
    fn RemoveStatus();
    fn AddStr(_: *mut libc::c_char);
    fn Flush(_: libc::c_int);
    fn freetty();
    fn Resize_obuf();
    fn ChangeWindowSize(
        _: *mut win,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
    ) -> libc::c_int;
    fn evenq(_: *mut event);
    fn evdeq(_: *mut event);
    fn SetTimeout(_: *mut event, _: libc::c_int);
    fn SaveStr(_: *const libc::c_char) -> *mut libc::c_char;
    fn Filename(_: *mut libc::c_char) -> *mut libc::c_char;
    fn closeallfiles(_: libc::c_int);
    fn xsignal(
        _: libc::c_int,
        _: Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
    ) -> Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
    fn AddXChar(_: *mut libc::c_char, _: libc::c_int) -> libc::c_int;
    fn AclCheckPermWin(_: *mut acluser, _: libc::c_int, _: *mut win) -> libc::c_int;
    fn AclWinSwap(_: libc::c_int, _: libc::c_int);
    fn NewWindowAcl(_: *mut win, _: *mut acluser) -> libc::c_int;
    fn FreeWindowAcl(_: *mut win);
    fn LClearLine(
        _: *mut layer,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: *mut mline,
    );
    fn LRefreshAll(_: *mut layer, _: libc::c_int);
    fn LCDisplayLine(
        _: *mut layer,
        _: *mut mline,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
    );
    fn LCDisplayLineWrap(
        _: *mut layer,
        _: *mut mline,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
    );
    fn KillLayerChain(_: *mut layer);
    fn ExitOverlayPage();
    fn ContainsSpecialDeffont(
        _: *mut mline,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
    ) -> libc::c_int;
    fn logfopen(name: *mut libc::c_char, fp: *mut FILE) -> *mut logfile;
    fn islogfile(name: *mut libc::c_char) -> libc::c_int;
    fn logfclose(_: *mut logfile) -> libc::c_int;
    static mut displays: *mut display;
    static mut display: *mut display;
    static mut windows: *mut win;
    static mut fore: *mut win;
    static mut console_window: *mut win;
    static mut ShellArgs: [*mut libc::c_char; 0];
    static mut ShellProg: *mut libc::c_char;
    static mut screenterm: [libc::c_char; 0];
    static mut screenlogfile: *mut libc::c_char;
    static mut TtyMode: libc::c_int;
    static mut SilenceWait: libc::c_int;
    static mut ServerSocket: libc::c_int;
    static mut real_uid: libc::c_int;
    static mut real_gid: libc::c_int;
    static mut eff_uid: libc::c_int;
    static mut eff_gid: libc::c_int;
    static mut NewEnv: *mut *mut libc::c_char;
    static mut maxwin: libc::c_int;
    static mut logflushev: event;
    static mut log_flush: libc::c_int;
    static mut logtstamp_after: libc::c_int;
    static mut ZombieKey_destroy: libc::c_int;
    static mut ZombieKey_resurrect: libc::c_int;
    static mut flayer: *mut layer;
    static mut maxusercount: libc::c_int;
    static mut pty_preopen: libc::c_int;
    static mut zmodem_mode: libc::c_int;
    static mut mchar_blank: mchar;
    static mut zmodem_sendcmd: *mut libc::c_char;
    static mut zmodem_recvcmd: *mut libc::c_char;
    static mut glwz: winsize;
    static mut separate_sids: libc::c_int;
}
pub type __int32_t = libc::c_int;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type pid_t = __pid_t;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;
pub type int32_t = __int32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
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
pub type __sighandler_t = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct winsize {
    pub ws_row: libc::c_ushort,
    pub ws_col: libc::c_ushort,
    pub ws_xpixel: libc::c_ushort,
    pub ws_ypixel: libc::c_ushort,
}
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
pub type slot_t = *mut libc::c_char;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct exit_status {
    pub e_termination: libc::c_short,
    pub e_exit: libc::c_short,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct utmp {
    pub ut_type: libc::c_short,
    pub ut_pid: pid_t,
    pub ut_line: [libc::c_char; 32],
    pub ut_id: [libc::c_char; 4],
    pub ut_user: [libc::c_char; 32],
    pub ut_host: [libc::c_char; 256],
    pub ut_exit: exit_status,
    pub ut_session: int32_t,
    pub ut_tv: C2RustUnnamed,
    pub ut_addr_v6: [int32_t; 4],
    pub __glibc_reserved: [libc::c_char; 20],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub tv_sec: int32_t,
    pub tv_usec: int32_t,
}
pub type state_t = libc::c_uint;
pub const PRIN4: state_t = 8;
pub const PRINCSI: state_t = 7;
pub const PRINESC: state_t = 6;
pub const PRIN: state_t = 5;
pub const CSI: state_t = 4;
pub const STRESC: state_t = 3;
pub const ASTR: state_t = 2;
pub const ESC: state_t = 1;
pub const LIT: state_t = 0;
pub type string_t = libc::c_uint;
pub const STATUS: string_t = 7;
pub const GM: string_t = 6;
pub const AKA: string_t = 5;
pub const PM: string_t = 4;
pub const APC: string_t = 3;
pub const OSC: string_t = 2;
pub const DCS: string_t = 1;
pub const NONE: string_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct event {
    pub next: *mut event,
    pub handler: Option::<unsafe extern "C" fn(*mut event, *mut libc::c_char) -> ()>,
    pub data: *mut libc::c_char,
    pub fd: libc::c_int,
    pub type_0: libc::c_int,
    pub pri: libc::c_int,
    pub timeout: timeval,
    pub queued: libc::c_int,
    pub active: libc::c_int,
    pub condpos: *mut libc::c_int,
    pub condneg: *mut libc::c_int,
}
pub type AclBits = *mut libc::c_uchar;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aclusergroup {
    pub u: *mut acluser,
    pub next: *mut aclusergroup,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct acluser {
    pub u_next: *mut acluser,
    pub u_name: [libc::c_char; 257],
    pub u_password: *mut libc::c_char,
    pub u_checkpassword: libc::c_int,
    pub u_detachwin: libc::c_int,
    pub u_detachotherwin: libc::c_int,
    pub u_Esc: libc::c_int,
    pub u_MetaEsc: libc::c_int,
    pub u_plop: plop,
    pub u_id: libc::c_int,
    pub u_umask_w_bits: [AclBits; 3],
    pub u_group: *mut aclusergroup,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct plop {
    pub buf: *mut libc::c_char,
    pub len: libc::c_int,
    pub enc: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct action {
    pub nr: libc::c_int,
    pub args: *mut *mut libc::c_char,
    pub argl: *mut libc::c_int,
    pub quiet: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mchar {
    pub image: libc::c_uchar,
    pub attr: libc::c_uchar,
    pub font: libc::c_uchar,
    pub fontx: libc::c_uchar,
    pub color: libc::c_uchar,
    pub mbcs: libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LayFuncs {
    pub lf_LayProcess: Option::<
        unsafe extern "C" fn(*mut *mut libc::c_char, *mut libc::c_int) -> (),
    >,
    pub lf_LayAbort: Option::<unsafe extern "C" fn() -> ()>,
    pub lf_LayRedisplayLine: Option::<
        unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int, libc::c_int) -> (),
    >,
    pub lf_LayClearLine: Option::<
        unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int, libc::c_int) -> (),
    >,
    pub lf_LayRewrite: Option::<
        unsafe extern "C" fn(
            libc::c_int,
            libc::c_int,
            libc::c_int,
            *mut mchar,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub lf_LayResize: Option::<
        unsafe extern "C" fn(libc::c_int, libc::c_int) -> libc::c_int,
    >,
    pub lf_LayRestore: Option::<unsafe extern "C" fn() -> ()>,
    pub lf_LayFree: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct layer {
    pub l_cvlist: *mut canvas,
    pub l_width: libc::c_int,
    pub l_height: libc::c_int,
    pub l_x: libc::c_int,
    pub l_y: libc::c_int,
    pub l_encoding: libc::c_int,
    pub l_layfn: *mut LayFuncs,
    pub l_data: *mut libc::c_void,
    pub l_next: *mut layer,
    pub l_bottom: *mut layer,
    pub l_blocking: libc::c_int,
    pub l_mode: libc::c_int,
    pub l_mouseevent: C2RustUnnamed_1,
    pub l_pause: C2RustUnnamed_0,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    #[bitfield(name = "d", ty = "libc::c_int", bits = "0..=0")]
    pub d: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
    pub left: *mut libc::c_int,
    pub right: *mut libc::c_int,
    pub top: libc::c_int,
    pub bottom: libc::c_int,
    pub lines: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub buffer: [libc::c_uchar; 3],
    pub len: libc::c_int,
    pub start: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct canvas {
    pub c_next: *mut canvas,
    pub c_display: *mut display,
    pub c_slnext: *mut canvas,
    pub c_slprev: *mut canvas,
    pub c_slperp: *mut canvas,
    pub c_slback: *mut canvas,
    pub c_slorient: libc::c_int,
    pub c_slweight: libc::c_int,
    pub c_vplist: *mut viewport,
    pub c_layer: *mut layer,
    pub c_lnext: *mut canvas,
    pub c_blank: layer,
    pub c_xoff: libc::c_int,
    pub c_yoff: libc::c_int,
    pub c_xs: libc::c_int,
    pub c_xe: libc::c_int,
    pub c_ys: libc::c_int,
    pub c_ye: libc::c_int,
    pub c_captev: event,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct viewport {
    pub v_next: *mut viewport,
    pub v_canvas: *mut canvas,
    pub v_xoff: libc::c_int,
    pub v_yoff: libc::c_int,
    pub v_xs: libc::c_int,
    pub v_xe: libc::c_int,
    pub v_ys: libc::c_int,
    pub v_ye: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct display {
    pub d_next: *mut display,
    pub d_user: *mut acluser,
    pub d_canvas: canvas,
    pub d_cvlist: *mut canvas,
    pub d_forecv: *mut canvas,
    pub d_layout: *mut layout,
    pub d_processinput: Option::<
        unsafe extern "C" fn(*mut libc::c_char, libc::c_int) -> (),
    >,
    pub d_processinputdata: *mut libc::c_char,
    pub d_vpxmin: libc::c_int,
    pub d_vpxmax: libc::c_int,
    pub d_fore: *mut win,
    pub d_other: *mut win,
    pub d_nonblock: libc::c_int,
    pub d_termname: [libc::c_char; 33],
    pub d_tentry: *mut libc::c_char,
    pub d_tcinited: libc::c_char,
    pub d_width: libc::c_int,
    pub d_height: libc::c_int,
    pub d_defwidth: libc::c_int,
    pub d_defheight: libc::c_int,
    pub d_top: libc::c_int,
    pub d_bot: libc::c_int,
    pub d_x: libc::c_int,
    pub d_y: libc::c_int,
    pub d_rend: mchar,
    pub d_col16change: libc::c_int,
    pub d_atyp: libc::c_char,
    pub d_mbcs: libc::c_int,
    pub d_encoding: libc::c_int,
    pub d_decodestate: libc::c_int,
    pub d_realfont: libc::c_int,
    pub d_insert: libc::c_int,
    pub d_keypad: libc::c_int,
    pub d_cursorkeys: libc::c_int,
    pub d_revvid: libc::c_int,
    pub d_curvis: libc::c_int,
    pub d_has_hstatus: libc::c_int,
    pub d_hstatus: libc::c_int,
    pub d_lp_missing: libc::c_int,
    pub d_mouse: libc::c_int,
    pub d_extmouse: libc::c_int,
    pub d_mouse_parse: mouse_parse,
    pub d_mousetrack: libc::c_int,
    pub d_lpchar: mchar,
    pub d_status_time: timeval,
    pub d_status: libc::c_int,
    pub d_status_bell: libc::c_char,
    pub d_status_len: libc::c_int,
    pub d_status_lastmsg: *mut libc::c_char,
    pub d_status_buflen: libc::c_int,
    pub d_status_lastx: libc::c_int,
    pub d_status_lasty: libc::c_int,
    pub d_status_obuflen: libc::c_int,
    pub d_status_obuffree: libc::c_int,
    pub d_status_obufpos: libc::c_int,
    pub d_statusev: event,
    pub d_hstatusev: event,
    pub d_kaablamm: libc::c_int,
    pub d_ESCseen: *mut action,
    pub d_userpid: libc::c_int,
    pub d_usertty: [libc::c_char; 4096],
    pub d_userfd: libc::c_int,
    pub d_readev: event,
    pub d_writeev: event,
    pub d_blockedev: event,
    pub d_OldMode: mode,
    pub d_NewMode: mode,
    pub d_flow: libc::c_int,
    pub d_intrc: libc::c_int,
    pub d_obuf: *mut libc::c_char,
    pub d_obuflen: libc::c_int,
    pub d_obufmax: libc::c_int,
    pub d_obuflenmax: libc::c_int,
    pub d_obufp: *mut libc::c_char,
    pub d_obuffree: libc::c_int,
    pub d_auto_nuke: libc::c_int,
    pub d_nseqs: libc::c_int,
    pub d_aseqs: libc::c_int,
    pub d_kmaps: *mut libc::c_uchar,
    pub d_seqp: *mut libc::c_uchar,
    pub d_seql: libc::c_int,
    pub d_seqh: *mut libc::c_uchar,
    pub d_mapev: event,
    pub d_dontmap: libc::c_int,
    pub d_mapdefault: libc::c_int,
    pub d_tcs: [tcu; 201],
    pub d_attrtab: [*mut libc::c_char; 6],
    pub d_attrtyp: [libc::c_char; 6],
    pub d_hascolor: libc::c_int,
    pub d_dospeed: libc::c_short,
    pub d_c0_tab: [libc::c_char; 256],
    pub d_xtable: *mut *mut *mut libc::c_char,
    pub d_UPcost: libc::c_int,
    pub d_DOcost: libc::c_int,
    pub d_LEcost: libc::c_int,
    pub d_NDcost: libc::c_int,
    pub d_CRcost: libc::c_int,
    pub d_IMcost: libc::c_int,
    pub d_EIcost: libc::c_int,
    pub d_NLcost: libc::c_int,
    pub d_printfd: libc::c_int,
    pub d_loginslot: slot_t,
    pub d_utmp_logintty: utmp,
    pub d_loginttymode: libc::c_int,
    pub d_blocked: libc::c_int,
    pub d_blocked_fuzz: libc::c_int,
    pub d_idleev: event,
    pub d_blankerpid: libc::c_int,
    pub d_blankerev: event,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union tcu {
    pub flg: libc::c_int,
    pub num: libc::c_int,
    pub str_0: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mode {
    pub tio: termios,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mouse_parse {
    pub sgrmode: libc::c_char,
    pub state: libc::c_int,
    pub params: [libc::c_int; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct win {
    pub w_next: *mut win,
    pub w_type: libc::c_int,
    pub w_data: *mut libc::c_void,
    pub w_layer: layer,
    pub w_savelayer: *mut layer,
    pub w_blocked: libc::c_int,
    pub w_pwin: *mut pseudowin,
    pub w_pdisplay: *mut display,
    pub w_lastdisp: *mut display,
    pub w_number: libc::c_int,
    pub w_readev: event,
    pub w_writeev: event,
    pub w_silenceev: event,
    pub w_zombieev: event,
    pub w_poll_zombie_timeout: libc::c_int,
    pub w_ptyfd: libc::c_int,
    pub w_inbuf: [libc::c_char; 4096],
    pub w_inlen: libc::c_int,
    pub w_outbuf: [libc::c_char; 4096],
    pub w_outlen: libc::c_int,
    pub w_aflag: libc::c_int,
    pub w_dynamicaka: libc::c_int,
    pub w_title: *mut libc::c_char,
    pub w_akachange: *mut libc::c_char,
    pub w_akabuf: [libc::c_char; 768],
    pub w_autoaka: libc::c_int,
    pub w_group: *mut win,
    pub w_intermediate: libc::c_int,
    pub w_args: [libc::c_int; 64],
    pub w_NumArgs: libc::c_int,
    pub w_wlock: libc::c_int,
    pub w_wlockuser: *mut acluser,
    pub w_userbits: [AclBits; 3],
    pub w_lio_notify: AclBits,
    pub w_mon_notify: AclBits,
    pub w_state: state_t,
    pub w_StringType: string_t,
    pub w_mlines: *mut mline,
    pub w_rend: mchar,
    pub w_FontL: libc::c_char,
    pub w_FontR: libc::c_char,
    pub w_FontE: libc::c_char,
    pub w_Charset: libc::c_int,
    pub w_CharsetR: libc::c_int,
    pub w_charsets: [libc::c_int; 4],
    pub w_ss: libc::c_int,
    pub w_saved: cursor,
    pub w_top: libc::c_int,
    pub w_bot: libc::c_int,
    pub w_wrap: libc::c_int,
    pub w_origin: libc::c_int,
    pub w_insert: libc::c_int,
    pub w_keypad: libc::c_int,
    pub w_cursorkeys: libc::c_int,
    pub w_revvid: libc::c_int,
    pub w_curinv: libc::c_int,
    pub w_curvvis: libc::c_int,
    pub w_autolf: libc::c_int,
    pub w_hstatus: *mut libc::c_char,
    pub w_gr: libc::c_int,
    pub w_c1: libc::c_int,
    pub w_bce: libc::c_int,
    pub w_decodestate: libc::c_int,
    pub w_mbcs: libc::c_int,
    pub w_string: [libc::c_char; 768],
    pub w_stringp: *mut libc::c_char,
    pub w_tabs: *mut libc::c_char,
    pub w_bell: libc::c_int,
    pub w_flow: libc::c_int,
    pub w_log: *mut logfile,
    pub w_logsilence: libc::c_int,
    pub w_monitor: libc::c_int,
    pub w_silencewait: libc::c_int,
    pub w_silence: libc::c_int,
    pub w_vbwait: libc::c_char,
    pub w_norefresh: libc::c_char,
    pub w_mouse: libc::c_int,
    pub w_extmouse: libc::c_int,
    pub w_slowpaste: libc::c_int,
    pub w_histheight: libc::c_int,
    pub w_histidx: libc::c_int,
    pub w_scrollback_height: libc::c_int,
    pub w_hlines: *mut mline,
    pub w_paster: paster,
    pub w_pid: libc::c_int,
    pub w_deadpid: libc::c_int,
    pub w_cmdargs: [*mut libc::c_char; 64],
    pub w_dir: *mut libc::c_char,
    pub w_term: *mut libc::c_char,
    pub w_lflag: libc::c_int,
    pub w_slot: slot_t,
    pub w_savut: utmp,
    pub w_tty: [libc::c_char; 768],
    pub w_zauto: libc::c_int,
    pub w_zdisplay: *mut display,
    pub w_alt: C2RustUnnamed_2,
    pub w_destroyev: event,
    pub w_exitstatus: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub on: libc::c_int,
    pub mlines: *mut mline,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub histheight: libc::c_int,
    pub hlines: *mut mline,
    pub histidx: libc::c_int,
    pub cursor: cursor,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cursor {
    pub on: libc::c_int,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub Rend: mchar,
    pub Charset: libc::c_int,
    pub CharsetR: libc::c_int,
    pub Charsets: [libc::c_int; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mline {
    pub image: *mut libc::c_uchar,
    pub attr: *mut libc::c_uchar,
    pub font: *mut libc::c_uchar,
    pub fontx: *mut libc::c_uchar,
    pub color: *mut libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct paster {
    pub pa_pastebuf: *mut libc::c_char,
    pub pa_pasteptr: *mut libc::c_char,
    pub pa_pastelen: libc::c_int,
    pub pa_pastelayer: *mut layer,
    pub pa_slowev: event,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct logfile {
    pub next: *mut logfile,
    pub fp: *mut FILE,
    pub name: *mut libc::c_char,
    pub opencount: libc::c_int,
    pub writecount: libc::c_int,
    pub flushcount: libc::c_int,
    pub st: *mut stat,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pseudowin {
    pub p_fdpat: libc::c_int,
    pub p_pid: libc::c_int,
    pub p_ptyfd: libc::c_int,
    pub p_readev: event,
    pub p_writeev: event,
    pub p_cmd: [libc::c_char; 768],
    pub p_tty: [libc::c_char; 768],
    pub p_inbuf: [libc::c_char; 4096],
    pub p_inlen: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct layout {
    pub lay_next: *mut layout,
    pub lay_title: *mut libc::c_char,
    pub lay_number: libc::c_int,
    pub lay_canvas: canvas,
    pub lay_forecv: *mut canvas,
    pub lay_cvlist: *mut canvas,
    pub lay_autosave: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NewWindow {
    pub StartAt: libc::c_int,
    pub aka: *mut libc::c_char,
    pub args: *mut *mut libc::c_char,
    pub dir: *mut libc::c_char,
    pub term: *mut libc::c_char,
    pub aflag: libc::c_int,
    pub dynamicaka: libc::c_int,
    pub flowflag: libc::c_int,
    pub lflag: libc::c_int,
    pub histheight: libc::c_int,
    pub monitor: libc::c_int,
    pub wlock: libc::c_int,
    pub silence: libc::c_int,
    pub wrap: libc::c_int,
    pub Lflag: libc::c_int,
    pub slow: libc::c_int,
    pub gr: libc::c_int,
    pub c1: libc::c_int,
    pub bce: libc::c_int,
    pub encoding: libc::c_int,
    pub hstatus: *mut libc::c_char,
    pub charset: *mut libc::c_char,
    pub poll_zombie_timeout: libc::c_int,
}
#[inline]
unsafe extern "C" fn stat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __xstat(1 as libc::c_int, __path, __statbuf);
}
pub static mut wtab: *mut *mut win = 0 as *const *mut win as *mut *mut win;
pub static mut VerboseCreate: libc::c_int = 0 as libc::c_int;
pub static mut DefaultShell: [libc::c_char; 8] = unsafe {
    *::std::mem::transmute::<&[u8; 8], &mut [libc::c_char; 8]>(b"/bin/sh\0")
};
pub static mut nwin_undef: NewWindow = {
    let mut init = NewWindow {
        StartAt: -(1 as libc::c_int),
        aka: 0 as *const libc::c_char as *mut libc::c_char,
        args: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        dir: 0 as *const libc::c_char as *mut libc::c_char,
        term: 0 as *const libc::c_char as *mut libc::c_char,
        aflag: -(1 as libc::c_int),
        dynamicaka: -(1 as libc::c_int),
        flowflag: -(1 as libc::c_int),
        lflag: -(1 as libc::c_int),
        histheight: -(1 as libc::c_int),
        monitor: -(1 as libc::c_int),
        wlock: -(1 as libc::c_int),
        silence: -(1 as libc::c_int),
        wrap: -(1 as libc::c_int),
        Lflag: -(1 as libc::c_int),
        slow: -(1 as libc::c_int),
        gr: -(1 as libc::c_int),
        c1: -(1 as libc::c_int),
        bce: -(1 as libc::c_int),
        encoding: -(1 as libc::c_int),
        hstatus: 0 as *const libc::c_char as *mut libc::c_char,
        charset: 0 as *const libc::c_char as *mut libc::c_char,
        poll_zombie_timeout: 0 as libc::c_int,
    };
    init
};
pub static mut nwin_default: NewWindow = unsafe {
    {
        let mut init = NewWindow {
            StartAt: 0 as libc::c_int,
            aka: 0 as *const libc::c_char as *mut libc::c_char,
            args: ShellArgs.as_ptr() as *mut _,
            dir: 0 as *const libc::c_char as *mut libc::c_char,
            term: screenterm.as_ptr() as *mut _,
            aflag: 0 as libc::c_int,
            dynamicaka: 1 as libc::c_int,
            flowflag: 1 as libc::c_int * ((1 as libc::c_int) << 0 as libc::c_int),
            lflag: 1 as libc::c_int,
            histheight: 100 as libc::c_int,
            monitor: 0 as libc::c_int,
            wlock: 0 as libc::c_int,
            silence: 0 as libc::c_int,
            wrap: 1 as libc::c_int,
            Lflag: 0 as libc::c_int,
            slow: 0 as libc::c_int,
            gr: 0 as libc::c_int,
            c1: 1 as libc::c_int,
            bce: 0 as libc::c_int,
            encoding: 0 as libc::c_int,
            hstatus: 0 as *const libc::c_char as *mut libc::c_char,
            charset: 0 as *const libc::c_char as *mut libc::c_char,
            poll_zombie_timeout: 0,
        };
        init
    }
};
pub static mut nwin_options: NewWindow = NewWindow {
    StartAt: 0,
    aka: 0 as *const libc::c_char as *mut libc::c_char,
    args: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
    dir: 0 as *const libc::c_char as *mut libc::c_char,
    term: 0 as *const libc::c_char as *mut libc::c_char,
    aflag: 0,
    dynamicaka: 0,
    flowflag: 0,
    lflag: 0,
    histheight: 0,
    monitor: 0,
    wlock: 0,
    silence: 0,
    wrap: 0,
    Lflag: 0,
    slow: 0,
    gr: 0,
    c1: 0,
    bce: 0,
    encoding: 0,
    hstatus: 0 as *const libc::c_char as *mut libc::c_char,
    charset: 0 as *const libc::c_char as *mut libc::c_char,
    poll_zombie_timeout: 0,
};
static mut const_IOSIZE: libc::c_int = 4096 as libc::c_int;
static mut const_one: libc::c_int = 1 as libc::c_int;
pub unsafe extern "C" fn nwin_compose(
    mut def: *mut NewWindow,
    mut new: *mut NewWindow,
    mut res: *mut NewWindow,
) {
    (*res)
        .StartAt = if (*new).StartAt != nwin_undef.StartAt {
        (*new).StartAt
    } else {
        (*def).StartAt
    };
    (*res).aka = if (*new).aka != nwin_undef.aka { (*new).aka } else { (*def).aka };
    (*res).args = if (*new).args != nwin_undef.args { (*new).args } else { (*def).args };
    (*res).dir = if (*new).dir != nwin_undef.dir { (*new).dir } else { (*def).dir };
    (*res).term = if (*new).term != nwin_undef.term { (*new).term } else { (*def).term };
    (*res)
        .aflag = if (*new).aflag != nwin_undef.aflag {
        (*new).aflag
    } else {
        (*def).aflag
    };
    (*res)
        .dynamicaka = if (*new).dynamicaka != nwin_undef.dynamicaka {
        (*new).dynamicaka
    } else {
        (*def).dynamicaka
    };
    (*res)
        .flowflag = if (*new).flowflag != nwin_undef.flowflag {
        (*new).flowflag
    } else {
        (*def).flowflag
    };
    (*res)
        .lflag = if (*new).lflag != nwin_undef.lflag {
        (*new).lflag
    } else {
        (*def).lflag
    };
    (*res)
        .histheight = if (*new).histheight != nwin_undef.histheight {
        (*new).histheight
    } else {
        (*def).histheight
    };
    (*res)
        .monitor = if (*new).monitor != nwin_undef.monitor {
        (*new).monitor
    } else {
        (*def).monitor
    };
    (*res)
        .wlock = if (*new).wlock != nwin_undef.wlock {
        (*new).wlock
    } else {
        (*def).wlock
    };
    (*res)
        .silence = if (*new).silence != nwin_undef.silence {
        (*new).silence
    } else {
        (*def).silence
    };
    (*res).wrap = if (*new).wrap != nwin_undef.wrap { (*new).wrap } else { (*def).wrap };
    (*res)
        .Lflag = if (*new).Lflag != nwin_undef.Lflag {
        (*new).Lflag
    } else {
        (*def).Lflag
    };
    (*res).slow = if (*new).slow != nwin_undef.slow { (*new).slow } else { (*def).slow };
    (*res).gr = if (*new).gr != nwin_undef.gr { (*new).gr } else { (*def).gr };
    (*res).c1 = if (*new).c1 != nwin_undef.c1 { (*new).c1 } else { (*def).c1 };
    (*res).bce = if (*new).bce != nwin_undef.bce { (*new).bce } else { (*def).bce };
    (*res)
        .encoding = if (*new).encoding != nwin_undef.encoding {
        (*new).encoding
    } else {
        (*def).encoding
    };
    (*res)
        .hstatus = if (*new).hstatus != nwin_undef.hstatus {
        (*new).hstatus
    } else {
        (*def).hstatus
    };
    (*res)
        .charset = if (*new).charset != nwin_undef.charset {
        (*new).charset
    } else {
        (*def).charset
    };
    (*res)
        .poll_zombie_timeout = if (*new).poll_zombie_timeout
        != nwin_undef.poll_zombie_timeout
    {
        (*new).poll_zombie_timeout
    } else {
        (*def).poll_zombie_timeout
    };
}
pub static mut WinLf: LayFuncs = unsafe {
    {
        let mut init = LayFuncs {
            lf_LayProcess: Some(
                WinProcess
                    as unsafe extern "C" fn(
                        *mut *mut libc::c_char,
                        *mut libc::c_int,
                    ) -> (),
            ),
            lf_LayAbort: None,
            lf_LayRedisplayLine: Some(
                WinRedisplayLine
                    as unsafe extern "C" fn(
                        libc::c_int,
                        libc::c_int,
                        libc::c_int,
                        libc::c_int,
                    ) -> (),
            ),
            lf_LayClearLine: Some(
                WinClearLine
                    as unsafe extern "C" fn(
                        libc::c_int,
                        libc::c_int,
                        libc::c_int,
                        libc::c_int,
                    ) -> (),
            ),
            lf_LayRewrite: Some(
                WinRewrite
                    as unsafe extern "C" fn(
                        libc::c_int,
                        libc::c_int,
                        libc::c_int,
                        *mut mchar,
                        libc::c_int,
                    ) -> libc::c_int,
            ),
            lf_LayResize: Some(
                WinResize
                    as unsafe extern "C" fn(libc::c_int, libc::c_int) -> libc::c_int,
            ),
            lf_LayRestore: Some(WinRestore as unsafe extern "C" fn() -> ()),
            lf_LayFree: None,
        };
        init
    }
};
unsafe extern "C" fn DoAutolf(
    mut buf: *mut libc::c_char,
    mut lenp: *mut libc::c_int,
    mut fr: libc::c_int,
) -> libc::c_int {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: libc::c_int = *lenp;
    let mut trunc: libc::c_int = 0 as libc::c_int;
    p = buf;
    while len > 0 as libc::c_int {
        if !(*p as libc::c_int != '\r' as i32) {
            let fresh0 = fr;
            fr = fr - 1;
            if fresh0 <= 0 as libc::c_int {
                trunc += 1;
                trunc;
                len -= 1;
                len;
            }
            if len == 0 as libc::c_int {
                break;
            }
            let fresh1 = len;
            len = len + 1;
            bcopy(
                p as *const libc::c_void,
                p.offset(1 as libc::c_int as isize) as *mut libc::c_void,
                fresh1 as size_t,
            );
            *p.offset(1 as libc::c_int as isize) = '\n' as i32 as libc::c_char;
        }
        p = p.offset(1);
        p;
        len -= 1;
        len;
    }
    *lenp = p.offset_from(buf) as libc::c_long as libc::c_int;
    return trunc;
}
unsafe extern "C" fn WinProcess(
    mut bufpp: *mut *mut libc::c_char,
    mut lenp: *mut libc::c_int,
) {
    let mut l2: libc::c_int = 0 as libc::c_int;
    let mut f: libc::c_int = 0;
    let mut ilen: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut l: libc::c_int = *lenp;
    let mut trunc: libc::c_int = 0;
    let mut ibuf: *mut libc::c_char = 0 as *mut libc::c_char;
    fore = (*flayer).l_data as *mut win;
    if (*fore).w_type == 3 as libc::c_int {
        *bufpp = (*bufpp).offset(*lenp as isize);
        *lenp = 0 as libc::c_int;
        return;
    }
    if (*fore).w_ptyfd < 0 as libc::c_int {
        ZombieProcess(bufpp, lenp);
        return;
    }
    if !display.is_null() && (*fore).w_wlock == 1 as libc::c_int
        && ((*fore).w_wlockuser).is_null()
        && AclCheckPermWin((*display).d_user, 1 as libc::c_int, fore) == 0
    {
        (*fore).w_wlockuser = (*display).d_user;
    }
    if !display.is_null()
        && (if (*fore).w_wlock == 0 as libc::c_int {
            AclCheckPermWin((*display).d_user, 1 as libc::c_int, fore)
        } else {
            ((*display).d_user != (*fore).w_wlockuser) as libc::c_int
        }) != 0
    {
        Msg(
            0 as libc::c_int,
            b"write: permission denied (user %s)\0" as *const u8 as *const libc::c_char,
            ((*(*display).d_user).u_name).as_mut_ptr(),
        );
        *bufpp = (*bufpp).offset(*lenp as isize);
        *lenp = 0 as libc::c_int;
        return;
    }
    if !((*fore).w_pwin).is_null()
        && (*(*fore).w_pwin).p_fdpat & 0x1000 as libc::c_int != 0
    {
        ibuf = ((*(*fore).w_pwin).p_inbuf).as_mut_ptr();
        ilen = &mut (*(*fore).w_pwin).p_inlen;
        f = (::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong)
            .wrapping_sub(*ilen as libc::c_ulong) as libc::c_int;
    } else {
        ibuf = ((*fore).w_inbuf).as_mut_ptr();
        ilen = &mut (*fore).w_inlen;
        f = (::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong)
            .wrapping_sub(*ilen as libc::c_ulong) as libc::c_int;
    }
    if l > f {
        l = f;
    }
    if l > 0 as libc::c_int {
        l2 = l;
        bcopy(
            *bufpp as *const libc::c_void,
            ibuf.offset(*ilen as isize) as *mut libc::c_void,
            l2 as size_t,
        );
        if (*fore).w_autolf != 0
            && {
                trunc = DoAutolf(ibuf.offset(*ilen as isize), &mut l2, f - l2);
                trunc != 0
            }
        {
            l -= trunc;
        }
        *ilen += l2;
        *bufpp = (*bufpp).offset(l as isize);
        *lenp -= l;
        return;
    }
}
unsafe extern "C" fn ZombieProcess(
    mut bufpp: *mut *mut libc::c_char,
    mut lenp: *mut libc::c_int,
) {
    let mut l: libc::c_int = *lenp;
    let mut buf: *mut libc::c_char = *bufpp;
    let mut b1: [libc::c_char; 10] = [0; 10];
    let mut b2: [libc::c_char; 10] = [0; 10];
    fore = (*flayer).l_data as *mut win;
    *bufpp = (*bufpp).offset(*lenp as isize);
    *lenp = 0 as libc::c_int;
    loop {
        let fresh2 = l;
        l = l - 1;
        if !(fresh2 > 0 as libc::c_int) {
            break;
        }
        if *(buf as *mut libc::c_uchar) as libc::c_int == ZombieKey_destroy {
            KillWindow(fore);
            return;
        }
        if *(buf as *mut libc::c_uchar) as libc::c_int == ZombieKey_resurrect {
            WriteString(
                fore,
                b"\r\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                2 as libc::c_int,
            );
            RemakeWindow(fore);
            return;
        }
        buf = buf.offset(1);
        buf;
    }
    b1[AddXChar(b1.as_mut_ptr(), ZombieKey_destroy)
        as usize] = '\0' as i32 as libc::c_char;
    b2[AddXChar(b2.as_mut_ptr(), ZombieKey_resurrect)
        as usize] = '\0' as i32 as libc::c_char;
    Msg(
        0 as libc::c_int,
        b"Press %s to destroy or %s to resurrect window\0" as *const u8
            as *const libc::c_char,
        b1.as_mut_ptr(),
        b2.as_mut_ptr(),
    );
}
unsafe extern "C" fn WinRedisplayLine(
    mut y: libc::c_int,
    mut from: libc::c_int,
    mut to: libc::c_int,
    mut isblank: libc::c_int,
) {
    if y < 0 as libc::c_int {
        return;
    }
    fore = (*flayer).l_data as *mut win;
    if from == 0 as libc::c_int && y > 0 as libc::c_int
        && *((*((*fore).w_mlines).offset((y - 1 as libc::c_int) as isize)).image)
            .offset((*fore).w_layer.l_width as isize) as libc::c_int == 0 as libc::c_int
    {
        LCDisplayLineWrap(
            &mut (*fore).w_layer,
            &mut *((*fore).w_mlines).offset(y as isize),
            y,
            from,
            to,
            isblank,
        );
    } else {
        LCDisplayLine(
            &mut (*fore).w_layer,
            &mut *((*fore).w_mlines).offset(y as isize),
            y,
            from,
            to,
            isblank,
        );
    };
}
unsafe extern "C" fn WinRewrite(
    mut y: libc::c_int,
    mut x1: libc::c_int,
    mut x2: libc::c_int,
    mut rend: *mut mchar,
    mut doit: libc::c_int,
) -> libc::c_int {
    let mut cost: libc::c_int = 0;
    let mut dx: libc::c_int = 0;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut i: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut f: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut fx: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut c: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    fore = (*flayer).l_data as *mut win;
    dx = x2 - x1 + 1 as libc::c_int;
    if doit != 0 {
        i = ((*((*fore).w_mlines).offset(y as isize)).image).offset(x1 as isize);
        loop {
            let fresh3 = dx;
            dx = dx - 1;
            if !(fresh3 > 0 as libc::c_int) {
                break;
            }
            let fresh4 = i;
            i = i.offset(1);
            PUTCHAR(*fresh4 as libc::c_int);
        }
        return 0 as libc::c_int;
    }
    p = ((*((*fore).w_mlines).offset(y as isize)).attr).offset(x1 as isize);
    f = ((*((*fore).w_mlines).offset(y as isize)).font).offset(x1 as isize);
    fx = ((*((*fore).w_mlines).offset(y as isize)).fontx).offset(x1 as isize);
    if (*rend).font as libc::c_int != 0
        && (*rend).font as libc::c_int & 0x60 as libc::c_int == 0 as libc::c_int
    {
        return 1000 as libc::c_int;
    }
    if (*fore).w_layer.l_encoding != 0 && (*fore).w_layer.l_encoding != 8 as libc::c_int
        && (*display).d_encoding == 8 as libc::c_int
        && ContainsSpecialDeffont(
            ((*fore).w_mlines).offset(y as isize),
            x1,
            x2,
            (*fore).w_layer.l_encoding,
        ) != 0
    {
        return 1000 as libc::c_int;
    }
    c = ((*((*fore).w_mlines).offset(y as isize)).color).offset(x1 as isize);
    dx = x2 - x1 + 1 as libc::c_int;
    cost = dx;
    loop {
        let fresh5 = dx;
        dx = dx - 1;
        if !(fresh5 > 0 as libc::c_int) {
            break;
        }
        let fresh6 = p;
        p = p.offset(1);
        if *fresh6 as libc::c_int != (*rend).attr as libc::c_int {
            return 1000 as libc::c_int;
        }
        let fresh7 = f;
        f = f.offset(1);
        if *fresh7 as libc::c_int != (*rend).font as libc::c_int {
            return 1000 as libc::c_int;
        }
        let fresh8 = fx;
        fx = fx.offset(1);
        if *fresh8 as libc::c_int != (*rend).fontx as libc::c_int {
            return 1000 as libc::c_int;
        }
        let fresh9 = c;
        c = c.offset(1);
        if *fresh9 as libc::c_int != (*rend).color as libc::c_int {
            return 1000 as libc::c_int;
        }
    }
    return cost;
}
unsafe extern "C" fn WinClearLine(
    mut y: libc::c_int,
    mut xs: libc::c_int,
    mut xe: libc::c_int,
    mut bce: libc::c_int,
) {
    fore = (*flayer).l_data as *mut win;
    LClearLine(flayer, y, xs, xe, bce, &mut *((*fore).w_mlines).offset(y as isize));
}
unsafe extern "C" fn WinResize(mut wi: libc::c_int, mut he: libc::c_int) -> libc::c_int {
    fore = (*flayer).l_data as *mut win;
    ChangeWindowSize(fore, wi, he, (*fore).w_histheight);
    return 0 as libc::c_int;
}
unsafe extern "C" fn WinRestore() {
    let mut cv: *mut canvas = 0 as *mut canvas;
    fore = (*flayer).l_data as *mut win;
    cv = (*flayer).l_cvlist;
    while !cv.is_null() {
        display = (*cv).c_display;
        if !(cv != (*display).d_forecv) {
            KeypadMode((*fore).w_keypad);
            CursorkeysMode((*fore).w_cursorkeys);
            SetFlow((*fore).w_flow & (1 as libc::c_int) << 0 as libc::c_int);
            InsertMode((*fore).w_insert);
            ReverseVideo((*fore).w_revvid);
            CursorVisibility(
                if (*fore).w_curinv != 0 {
                    -(1 as libc::c_int)
                } else {
                    (*fore).w_curvvis
                },
            );
            MouseMode((*fore).w_mouse);
            ExtMouseMode((*fore).w_extmouse);
        }
        cv = (*cv).c_next;
    }
}
pub unsafe extern "C" fn DoStartLog(
    mut w: *mut win,
    mut buf: *mut libc::c_char,
    mut bufsize: libc::c_int,
) -> libc::c_int {
    let mut n: libc::c_int = 0;
    if w.is_null() || buf.is_null() {
        return -(1 as libc::c_int);
    }
    strncpy(
        buf,
        MakeWinMsg(screenlogfile, w, '%' as i32),
        (bufsize - 1 as libc::c_int) as libc::c_ulong,
    );
    *buf
        .offset(
            (bufsize - 1 as libc::c_int) as isize,
        ) = 0 as libc::c_int as libc::c_char;
    if !((*w).w_log).is_null() {
        logfclose((*w).w_log);
    }
    (*w)
        .w_log = logfopen(
        buf,
        if islogfile(buf) != 0 {
            0 as *mut FILE
        } else {
            secfopen(
                buf,
                b"a\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            )
        },
    );
    if ((*w).w_log).is_null() {
        return -(2 as libc::c_int);
    }
    if logflushev.queued == 0 {
        n = if log_flush != 0 {
            log_flush
        } else {
            (logtstamp_after + 4 as libc::c_int) / 5 as libc::c_int
        };
        if n != 0 {
            SetTimeout(&mut logflushev, n * 1000 as libc::c_int);
            evenq(&mut logflushev);
        }
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn MakeWindow(mut newwin: *mut NewWindow) -> libc::c_int {
    let mut pp: *mut *mut win = 0 as *mut *mut win;
    let mut p: *mut win = 0 as *mut win;
    let mut n: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut f: libc::c_int = -(1 as libc::c_int);
    let mut nwin: NewWindow = NewWindow {
        StartAt: 0,
        aka: 0 as *const libc::c_char as *mut libc::c_char,
        args: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        dir: 0 as *const libc::c_char as *mut libc::c_char,
        term: 0 as *const libc::c_char as *mut libc::c_char,
        aflag: 0,
        dynamicaka: 0,
        flowflag: 0,
        lflag: 0,
        histheight: 0,
        monitor: 0,
        wlock: 0,
        silence: 0,
        wrap: 0,
        Lflag: 0,
        slow: 0,
        gr: 0,
        c1: 0,
        bce: 0,
        encoding: 0,
        hstatus: 0 as *const libc::c_char as *mut libc::c_char,
        charset: 0 as *const libc::c_char as *mut libc::c_char,
        poll_zombie_timeout: 0,
    };
    let mut type_0: libc::c_int = 0;
    let mut startat: libc::c_int = 0;
    let mut TtyName: *mut libc::c_char = 0 as *mut libc::c_char;
    extern "C" {
        static mut users: *mut acluser;
    }
    if wtab.is_null() {
        if maxwin == 0 {
            maxwin = 100 as libc::c_int;
        }
        wtab = calloc(
            maxwin as libc::c_ulong,
            ::std::mem::size_of::<*mut win>() as libc::c_ulong,
        ) as *mut *mut win;
    }
    nwin_compose(&mut nwin_default, newwin, &mut nwin);
    startat = if nwin.StartAt < maxwin { nwin.StartAt } else { 0 as libc::c_int };
    pp = wtab.offset(startat as isize);
    while !(*pp).is_null() {
        pp = pp.offset(1);
        if pp == wtab.offset(maxwin as isize) {
            pp = wtab;
        }
        if !(pp != wtab.offset(startat as isize)) {
            break;
        }
    }
    if !(*pp).is_null() {
        Msg(0 as libc::c_int, b"No more windows.\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    n = pp.offset_from(wtab) as libc::c_long as libc::c_int;
    f = OpenDevice(nwin.args, nwin.lflag, &mut type_0, &mut TtyName);
    if f < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if type_0 == 3 as libc::c_int {
        f = -(1 as libc::c_int);
    }
    p = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<win>() as libc::c_ulong,
    ) as *mut win;
    if p.is_null() {
        close(f);
        Msg(
            0 as libc::c_int,
            b"%s\0" as *const u8 as *const libc::c_char,
            strnomem.as_mut_ptr(),
        );
        return -(1 as libc::c_int);
    }
    if type_0 != 0 as libc::c_int {
        nwin.lflag = 0 as libc::c_int;
    }
    (*p).w_type = type_0;
    i = 0 as libc::c_int;
    while !(*(nwin.args).offset(i as isize)).is_null()
        && i < 64 as libc::c_int - 1 as libc::c_int
    {
        (*p).w_cmdargs[i as usize] = SaveStr(*(nwin.args).offset(i as isize));
        i += 1;
        i;
    }
    (*p).w_cmdargs[i as usize] = 0 as *mut libc::c_char;
    if !(nwin.dir).is_null() {
        (*p).w_dir = SaveStr(nwin.dir);
    }
    if !(nwin.term).is_null() {
        (*p).w_term = SaveStr(nwin.term);
    }
    (*p).w_number = n;
    (*p).w_group = 0 as *mut win;
    if !fore.is_null() && (*fore).w_type == 3 as libc::c_int {
        (*p).w_group = fore;
    } else if !fore.is_null() && !((*fore).w_group).is_null() {
        (*p).w_group = (*fore).w_group;
    }
    if NewWindowAcl(p, if !display.is_null() { (*display).d_user } else { users }) != 0 {
        free(p as *mut libc::c_char as *mut libc::c_void);
        close(f);
        Msg(
            0 as libc::c_int,
            b"%s\0" as *const u8 as *const libc::c_char,
            strnomem.as_mut_ptr(),
        );
        return -(1 as libc::c_int);
    }
    (*p).w_layer.l_next = 0 as *mut layer;
    (*p).w_layer.l_bottom = &mut (*p).w_layer;
    (*p).w_layer.l_layfn = &mut WinLf;
    (*p).w_layer.l_data = p as *mut libc::c_char as *mut libc::c_void;
    (*p).w_savelayer = &mut (*p).w_layer;
    (*p).w_pdisplay = 0 as *mut display;
    (*p).w_lastdisp = 0 as *mut display;
    if !display.is_null() && AclCheckPermWin((*display).d_user, 1 as libc::c_int, p) == 0
    {
        (*p).w_wlockuser = (*display).d_user;
    }
    (*p).w_wlock = nwin.wlock;
    (*p).w_ptyfd = f;
    (*p).w_aflag = nwin.aflag;
    (*p).w_dynamicaka = nwin.dynamicaka;
    (*p)
        .w_flow = nwin.flowflag
        | (if nwin.flowflag & (1 as libc::c_int) << 2 as libc::c_int != 0 {
            (1 as libc::c_int) << 1 as libc::c_int
                | (1 as libc::c_int) << 0 as libc::c_int
        } else {
            (1 as libc::c_int) << 1 as libc::c_int
        });
    if (nwin.aka).is_null() {
        nwin.aka = Filename(*(nwin.args).offset(0 as libc::c_int as isize));
    }
    strncpy(
        ((*p).w_akabuf).as_mut_ptr(),
        nwin.aka,
        (::std::mem::size_of::<[libc::c_char; 768]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    );
    nwin.aka = rindex(((*p).w_akabuf).as_mut_ptr(), '|' as i32);
    if !(nwin.aka).is_null() {
        (*p).w_autoaka = 0 as libc::c_int;
        let fresh10 = nwin.aka;
        nwin.aka = (nwin.aka).offset(1);
        *fresh10 = 0 as libc::c_int as libc::c_char;
        (*p).w_title = nwin.aka;
        (*p).w_akachange = (nwin.aka).offset(strlen(nwin.aka) as isize);
    } else {
        (*p).w_akachange = ((*p).w_akabuf).as_mut_ptr();
        (*p).w_title = (*p).w_akachange;
    }
    if !(nwin.hstatus).is_null() {
        (*p).w_hstatus = SaveStr(nwin.hstatus);
    }
    (*p).w_monitor = nwin.monitor;
    if (*p).w_monitor == 1 as libc::c_int {
        i = 0 as libc::c_int;
        while i < maxusercount {
            let ref mut fresh11 = *((*p).w_mon_notify)
                .offset((i >> 3 as libc::c_int) as isize);
            *fresh11 = (*fresh11 as libc::c_int
                | 0x80 as libc::c_int >> (i & 7 as libc::c_int)) as libc::c_uchar;
            i += 1;
            i;
        }
    }
    (*p).w_silence = nwin.silence;
    (*p).w_silencewait = SilenceWait;
    if (*p).w_silence == 1 as libc::c_int {
        i = 0 as libc::c_int;
        while i < maxusercount {
            let ref mut fresh12 = *((*p).w_lio_notify)
                .offset((i >> 3 as libc::c_int) as isize);
            *fresh12 = (*fresh12 as libc::c_int
                | 0x80 as libc::c_int >> (i & 7 as libc::c_int)) as libc::c_uchar;
            i += 1;
            i;
        }
    }
    (*p).w_slowpaste = nwin.slow;
    (*p).w_norefresh = 0 as libc::c_int as libc::c_char;
    strncpy(
        ((*p).w_tty).as_mut_ptr(),
        TtyName,
        (768 as libc::c_int - 1 as libc::c_int) as libc::c_ulong,
    );
    if ChangeWindowSize(
        p,
        if !display.is_null() {
            (*(*display).d_forecv).c_xe - (*(*display).d_forecv).c_xs + 1 as libc::c_int
        } else {
            80 as libc::c_int
        },
        if !display.is_null() {
            (*(*display).d_forecv).c_ye - (*(*display).d_forecv).c_ys + 1 as libc::c_int
        } else {
            24 as libc::c_int
        },
        nwin.histheight,
    ) != 0
    {
        FreeWindow(p);
        return -(1 as libc::c_int);
    }
    (*p).w_layer.l_encoding = nwin.encoding;
    ResetWindow(p);
    if !(nwin.charset).is_null() {
        SetCharsets(p, nwin.charset);
    }
    if VerboseCreate != 0 && type_0 != 3 as libc::c_int {
        let mut d: *mut display = display;
        WriteString(
            p,
            b":screen (\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            9 as libc::c_int,
        );
        WriteString(p, (*p).w_title, strlen((*p).w_title) as libc::c_int);
        WriteString(
            p,
            b"):\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            2 as libc::c_int,
        );
        f = 0 as libc::c_int;
        while !((*p).w_cmdargs[f as usize]).is_null() {
            WriteString(
                p,
                b" \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                1 as libc::c_int,
            );
            WriteString(
                p,
                (*p).w_cmdargs[f as usize],
                strlen((*p).w_cmdargs[f as usize]) as libc::c_int,
            );
            f += 1;
            f;
        }
        WriteString(
            p,
            b"\r\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            2 as libc::c_int,
        );
        display = d;
    }
    (*p).w_deadpid = 0 as libc::c_int;
    (*p).w_pid = 0 as libc::c_int;
    (*p).w_pwin = 0 as *mut pseudowin;
    if type_0 == 0 as libc::c_int {
        (*p).w_pid = ForkWindow(p, nwin.args, TtyName);
        if (*p).w_pid < 0 as libc::c_int {
            FreeWindow(p);
            return -(1 as libc::c_int);
        }
    }
    if !display.is_null() && !((*display).d_fore).is_null() {
        (*display).d_other = (*display).d_fore;
    }
    *pp = p;
    (*p).w_next = windows;
    windows = p;
    if type_0 == 3 as libc::c_int {
        SetForeWindow(p);
        Activate((*p).w_norefresh as libc::c_int);
        WindowChanged(0 as *mut win, 'w' as i32);
        WindowChanged(0 as *mut win, 'W' as i32);
        WindowChanged(0 as *mut win, 0 as libc::c_int);
        return n;
    }
    (*p).w_lflag = nwin.lflag;
    (*p).w_slot = -(1 as libc::c_int) as slot_t;
    if nwin.lflag & 1 as libc::c_int != 0 {
        (*p).w_slot = 0 as slot_t;
        if !display.is_null() || (*p).w_lflag & 2 as libc::c_int != 0 {
            SetUtmp(p);
        }
    }
    if nwin.Lflag != 0 {
        let mut buf: [libc::c_char; 1024] = [0; 1024];
        DoStartLog(
            p,
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
        );
    }
    if nwin.poll_zombie_timeout != 0 {
        (*p).w_poll_zombie_timeout = nwin.poll_zombie_timeout;
    }
    (*p).w_zombieev.type_0 = 0 as libc::c_int;
    (*p).w_zombieev.data = p as *mut libc::c_char;
    (*p)
        .w_zombieev
        .handler = Some(
        win_resurrect_zombie_fn
            as unsafe extern "C" fn(*mut event, *mut libc::c_char) -> (),
    );
    (*p).w_writeev.fd = (*p).w_ptyfd;
    (*p).w_readev.fd = (*p).w_writeev.fd;
    (*p).w_readev.type_0 = 1 as libc::c_int;
    (*p).w_writeev.type_0 = 2 as libc::c_int;
    (*p).w_writeev.data = p as *mut libc::c_char;
    (*p).w_readev.data = (*p).w_writeev.data;
    (*p)
        .w_readev
        .handler = Some(
        win_readev_fn as unsafe extern "C" fn(*mut event, *mut libc::c_char) -> (),
    );
    (*p)
        .w_writeev
        .handler = Some(
        win_writeev_fn as unsafe extern "C" fn(*mut event, *mut libc::c_char) -> (),
    );
    (*p).w_writeev.condpos = &mut (*p).w_inlen;
    evenq(&mut (*p).w_readev);
    evenq(&mut (*p).w_writeev);
    (*p).w_paster.pa_slowev.type_0 = 0 as libc::c_int;
    (*p)
        .w_paster
        .pa_slowev
        .data = &mut (*p).w_paster as *mut paster as *mut libc::c_char;
    (*p)
        .w_paster
        .pa_slowev
        .handler = Some(
        paste_slowev_fn as unsafe extern "C" fn(*mut event, *mut libc::c_char) -> (),
    );
    (*p).w_silenceev.type_0 = 0 as libc::c_int;
    (*p).w_silenceev.data = p as *mut libc::c_char;
    (*p)
        .w_silenceev
        .handler = Some(
        win_silenceev_fn as unsafe extern "C" fn(*mut event, *mut libc::c_char) -> (),
    );
    if (*p).w_silence > 0 as libc::c_int {
        SetTimeout(&mut (*p).w_silenceev, (*p).w_silencewait * 1000 as libc::c_int);
        evenq(&mut (*p).w_silenceev);
    }
    (*p).w_destroyev.type_0 = 0 as libc::c_int;
    (*p).w_destroyev.data = 0 as *mut libc::c_char;
    (*p)
        .w_destroyev
        .handler = Some(
        win_destroyev_fn as unsafe extern "C" fn(*mut event, *mut libc::c_char) -> (),
    );
    SetForeWindow(p);
    Activate((*p).w_norefresh as libc::c_int);
    WindowChanged(0 as *mut win, 'w' as i32);
    WindowChanged(0 as *mut win, 'W' as i32);
    WindowChanged(0 as *mut win, 0 as libc::c_int);
    return n;
}
pub unsafe extern "C" fn RemakeWindow(mut p: *mut win) -> libc::c_int {
    let mut TtyName: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut lflag: libc::c_int = 0;
    let mut f: libc::c_int = 0;
    lflag = nwin_default.lflag;
    f = OpenDevice(((*p).w_cmdargs).as_mut_ptr(), lflag, &mut (*p).w_type, &mut TtyName);
    if f < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    evdeq(&mut (*p).w_destroyev);
    strncpy(
        ((*p).w_tty).as_mut_ptr(),
        if *TtyName as libc::c_int != 0 { TtyName } else { (*p).w_title },
        (768 as libc::c_int - 1 as libc::c_int) as libc::c_ulong,
    );
    (*p).w_ptyfd = f;
    (*p).w_readev.fd = f;
    (*p).w_writeev.fd = f;
    evenq(&mut (*p).w_readev);
    evenq(&mut (*p).w_writeev);
    if VerboseCreate != 0 {
        let mut d: *mut display = display;
        WriteString(
            p,
            b":screen (\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            9 as libc::c_int,
        );
        WriteString(p, (*p).w_title, strlen((*p).w_title) as libc::c_int);
        WriteString(
            p,
            b"):\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            2 as libc::c_int,
        );
        f = 0 as libc::c_int;
        while !((*p).w_cmdargs[f as usize]).is_null() {
            WriteString(
                p,
                b" \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                1 as libc::c_int,
            );
            WriteString(
                p,
                (*p).w_cmdargs[f as usize],
                strlen((*p).w_cmdargs[f as usize]) as libc::c_int,
            );
            f += 1;
            f;
        }
        WriteString(
            p,
            b"\r\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            2 as libc::c_int,
        );
        display = d;
    }
    (*p).w_deadpid = 0 as libc::c_int;
    (*p).w_pid = 0 as libc::c_int;
    if (*p).w_type == 0 as libc::c_int {
        (*p).w_pid = ForkWindow(p, ((*p).w_cmdargs).as_mut_ptr(), TtyName);
        if (*p).w_pid < 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
    }
    if ((*p).w_slot).is_null()
        && (!display.is_null() || (*p).w_lflag & 2 as libc::c_int != 0)
    {
        SetUtmp(p);
    }
    WindowChanged(p, 'f' as i32);
    return (*p).w_number;
}
pub unsafe extern "C" fn CloseDevice(mut wp: *mut win) {
    if (*wp).w_ptyfd < 0 as libc::c_int {
        return;
    }
    if (*wp).w_type == 0 as libc::c_int {
        chmod(((*wp).w_tty).as_mut_ptr(), 0o666 as libc::c_int as __mode_t);
        chown(
            ((*wp).w_tty).as_mut_ptr(),
            0 as libc::c_int as __uid_t,
            0 as libc::c_int as __gid_t,
        );
    }
    close((*wp).w_ptyfd);
    (*wp).w_ptyfd = -(1 as libc::c_int);
    (*wp).w_tty[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    evdeq(&mut (*wp).w_readev);
    evdeq(&mut (*wp).w_writeev);
    (*wp).w_writeev.fd = -(1 as libc::c_int);
    (*wp).w_readev.fd = (*wp).w_writeev.fd;
}
pub unsafe extern "C" fn FreeWindow(mut wp: *mut win) {
    let mut d: *mut display = 0 as *mut display;
    let mut i: libc::c_int = 0;
    let mut cv: *mut canvas = 0 as *mut canvas;
    let mut ncv: *mut canvas = 0 as *mut canvas;
    let mut l: *mut layer = 0 as *mut layer;
    if !((*wp).w_pwin).is_null() {
        FreePseudowin(wp);
    }
    RemoveUtmp(wp);
    CloseDevice(wp);
    if wp == console_window {
        TtyGrabConsole(
            -(1 as libc::c_int),
            -(1 as libc::c_int),
            b"free\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        console_window = 0 as *mut win;
    }
    if !((*wp).w_log).is_null() {
        logfclose((*wp).w_log);
    }
    ChangeWindowSize(wp, 0 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int);
    if (*wp).w_type == 3 as libc::c_int {
        let mut win: *mut win = 0 as *mut win;
        win = windows;
        while !win.is_null() {
            if (*win).w_group == wp {
                (*win).w_group = (*wp).w_group;
            }
            win = (*win).w_next;
        }
    }
    if !((*wp).w_hstatus).is_null() {
        free((*wp).w_hstatus as *mut libc::c_void);
    }
    i = 0 as libc::c_int;
    while !((*wp).w_cmdargs[i as usize]).is_null() {
        free((*wp).w_cmdargs[i as usize] as *mut libc::c_void);
        i += 1;
        i;
    }
    if !((*wp).w_dir).is_null() {
        free((*wp).w_dir as *mut libc::c_void);
    }
    if !((*wp).w_term).is_null() {
        free((*wp).w_term as *mut libc::c_void);
    }
    d = displays;
    while !d.is_null() {
        if (*d).d_other == wp {
            (*d)
                .d_other = if !((*d).d_fore).is_null() && (*(*d).d_fore).w_next != wp {
                (*(*d).d_fore).w_next
            } else {
                (*wp).w_next
            };
        }
        if (*d).d_fore == wp {
            (*d).d_fore = 0 as *mut win;
        }
        cv = (*d).d_cvlist;
        while !cv.is_null() {
            l = (*cv).c_layer;
            while !l.is_null() {
                if (*l).l_layfn == &mut WinLf as *mut LayFuncs {
                    break;
                }
                l = (*l).l_next;
            }
            if !l.is_null() {
                if !((*l).l_data as *mut win != wp) {
                    if (*cv).c_layer == (*wp).w_savelayer {
                        (*wp).w_savelayer = 0 as *mut layer;
                    }
                    KillLayerChain((*cv).c_layer);
                }
            }
            cv = (*cv).c_next;
        }
        d = (*d).d_next;
    }
    if !((*wp).w_savelayer).is_null() {
        KillLayerChain((*wp).w_savelayer);
    }
    cv = (*wp).w_layer.l_cvlist;
    while !cv.is_null() {
        ncv = (*cv).c_lnext;
        (*cv).c_layer = &mut (*cv).c_blank;
        (*cv).c_blank.l_cvlist = cv;
        (*cv).c_lnext = 0 as *mut canvas;
        (*cv).c_xoff = (*cv).c_xs;
        (*cv).c_yoff = (*cv).c_ys;
        RethinkViewportOffsets(cv);
        cv = ncv;
    }
    (*wp).w_layer.l_cvlist = 0 as *mut canvas;
    if flayer == &mut (*wp).w_layer as *mut layer {
        flayer = 0 as *mut layer;
    }
    LayerCleanupMemory(&mut (*wp).w_layer);
    FreeWindowAcl(wp);
    evdeq(&mut (*wp).w_readev);
    evdeq(&mut (*wp).w_writeev);
    evdeq(&mut (*wp).w_silenceev);
    evdeq(&mut (*wp).w_zombieev);
    evdeq(&mut (*wp).w_destroyev);
    FreePaster(&mut (*wp).w_paster);
    free(wp as *mut libc::c_char as *mut libc::c_void);
}
pub unsafe extern "C" fn OpenDevice(
    mut args: *mut *mut libc::c_char,
    mut lflag: libc::c_int,
    mut typep: *mut libc::c_int,
    mut namep: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut arg: *mut libc::c_char = *args.offset(0 as libc::c_int as isize);
    let mut st: stat = stat {
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
    let mut f: libc::c_int = 0;
    if arg.is_null() {
        return -(1 as libc::c_int);
    }
    if strcmp(arg, b"//group\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
    {
        *typep = 3 as libc::c_int;
        *namep = b"telnet\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        return 0 as libc::c_int;
    }
    if strncmp(
        arg,
        b"//\0" as *const u8 as *const libc::c_char,
        2 as libc::c_int as libc::c_ulong,
    ) == 0 as libc::c_int
    {
        Msg(
            0 as libc::c_int,
            b"Invalid argument '%s'\0" as *const u8 as *const libc::c_char,
            arg,
        );
        return -(1 as libc::c_int);
    } else if stat(arg, &mut st) == 0 as libc::c_int
        && st.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o20000 as libc::c_int as libc::c_uint
    {
        if access(arg, 4 as libc::c_int | 2 as libc::c_int) == -(1 as libc::c_int) {
            Msg(
                *__errno_location(),
                b"Cannot access line '%s' for R/W\0" as *const u8 as *const libc::c_char,
                arg,
            );
            return -(1 as libc::c_int);
        }
        f = OpenTTY(arg, *args.offset(1 as libc::c_int as isize));
        if f < 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
        lflag = 0 as libc::c_int;
        *typep = 1 as libc::c_int;
        *namep = arg;
    } else {
        *typep = 0 as libc::c_int;
        f = OpenPTY(namep);
        if f == -(1 as libc::c_int) {
            Msg(
                0 as libc::c_int,
                b"No more PTYs.\0" as *const u8 as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
        let mut flag: libc::c_int = 1 as libc::c_int;
        if ioctl(
            f,
            0x5420 as libc::c_int as libc::c_ulong,
            &mut flag as *mut libc::c_int as *mut libc::c_char,
        ) != 0
        {
            Msg(
                *__errno_location(),
                b"TIOCPKT ioctl\0" as *const u8 as *const libc::c_char,
            );
            close(f);
            return -(1 as libc::c_int);
        }
    }
    fcntl(f, 4 as libc::c_int, 0o4000 as libc::c_int);
    if *typep == 0 as libc::c_int || *typep == 1 as libc::c_int {
        tcflush(f, 2 as libc::c_int);
    }
    if *typep != 0 as libc::c_int {
        return f;
    }
    if chown(*namep, real_uid as __uid_t, 5 as libc::c_int as __gid_t) != 0
        && eff_uid == 0
    {
        Msg(*__errno_location(), b"chown tty\0" as *const u8 as *const libc::c_char);
        close(f);
        return -(1 as libc::c_int);
    }
    if chmod(
        *namep,
        (if lflag != 0 { TtyMode } else { TtyMode & !(0o22 as libc::c_int) }) as __mode_t,
    ) != 0 && eff_uid == 0
    {
        Msg(*__errno_location(), b"chmod tty\0" as *const u8 as *const libc::c_char);
        close(f);
        return -(1 as libc::c_int);
    }
    return f;
}
unsafe extern "C" fn ForkWindow(
    mut win: *mut win,
    mut args: *mut *mut libc::c_char,
    mut ttyn: *mut libc::c_char,
) -> libc::c_int {
    let mut pid: libc::c_int = 0;
    let mut tebuf: [libc::c_char; 38] = [0; 38];
    let mut ebuf: [libc::c_char; 20] = [0; 20];
    let mut shellbuf: [libc::c_char; 4103] = [0; 4103];
    let mut proc_0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut newfd: libc::c_int = 0;
    let mut w: libc::c_int = (*win).w_layer.l_width;
    let mut h: libc::c_int = (*win).w_layer.l_height;
    let mut i: libc::c_int = 0;
    let mut pat: libc::c_int = 0;
    let mut wfdused: libc::c_int = 0;
    let mut pwin: *mut pseudowin = (*win).w_pwin;
    let mut slave: libc::c_int = -(1 as libc::c_int);
    if pty_preopen != 0 {
        slave = open(ttyn, 0o2 as libc::c_int | 0o400 as libc::c_int);
        if slave == -(1 as libc::c_int) {
            Msg(*__errno_location(), b"ttyn\0" as *const u8 as *const libc::c_char);
            return -(1 as libc::c_int);
        }
    }
    proc_0 = *args;
    if proc_0.is_null() {
        args = ShellArgs.as_mut_ptr();
        proc_0 = *args;
    }
    fflush(stdout);
    fflush(stderr);
    pid = fork();
    match pid {
        -1 => {
            Msg(*__errno_location(), b"fork\0" as *const u8 as *const libc::c_char);
        }
        0 => {
            xsignal(1 as libc::c_int, None);
            xsignal(2 as libc::c_int, None);
            xsignal(3 as libc::c_int, None);
            xsignal(15 as libc::c_int, None);
            xsignal(21 as libc::c_int, None);
            xsignal(22 as libc::c_int, None);
            xsignal(13 as libc::c_int, None);
            xsignal(25 as libc::c_int, None);
            displays = 0 as *mut display;
            ServerSocket = -(1 as libc::c_int);
            if setgid(real_gid as __gid_t) != 0 || setuid(real_uid as __uid_t) != 0 {
                Panic(
                    *__errno_location(),
                    b"Setuid/gid\0" as *const u8 as *const libc::c_char,
                );
            }
            eff_uid = real_uid;
            eff_gid = real_gid;
            if pwin.is_null() {
                if !((*win).w_dir).is_null() && *(*win).w_dir as libc::c_int != 0
                    && chdir((*win).w_dir) != 0
                {
                    Panic(
                        *__errno_location(),
                        b"Cannot chdir to %s\0" as *const u8 as *const libc::c_char,
                        (*win).w_dir,
                    );
                }
            }
            if !display.is_null() {
                brktty((*display).d_userfd);
                freetty();
            } else {
                brktty(-(1 as libc::c_int));
            }
            if slave != -(1 as libc::c_int) {
                close(0 as libc::c_int);
                dup(slave);
                close(slave);
                closeallfiles((*win).w_ptyfd);
                slave = dup(0 as libc::c_int);
            } else {
                closeallfiles((*win).w_ptyfd);
            }
            close(0 as libc::c_int);
            close(1 as libc::c_int);
            close(2 as libc::c_int);
            newfd = -(1 as libc::c_int);
            pat = if !pwin.is_null() {
                (*pwin).p_fdpat
            } else {
                (0x1 as libc::c_int) << 2 as libc::c_int * 2 as libc::c_int
                    | (0x1 as libc::c_int) << 2 as libc::c_int | 0x1 as libc::c_int
            };
            wfdused = 0 as libc::c_int;
            i = 0 as libc::c_int;
            while i < 3 as libc::c_int {
                if pat & (0x1 as libc::c_int) << 2 as libc::c_int * i != 0 {
                    if newfd < 0 as libc::c_int {
                        if separate_sids != 0 {
                            newfd = open(ttyn, 0o2 as libc::c_int);
                        } else {
                            newfd = open(
                                ttyn,
                                0o2 as libc::c_int | 0o400 as libc::c_int,
                            );
                        }
                        if newfd < 0 as libc::c_int {
                            Panic(
                                *__errno_location(),
                                b"Cannot open %s\0" as *const u8 as *const libc::c_char,
                                ttyn,
                            );
                        }
                    } else {
                        dup(newfd);
                    }
                } else {
                    dup((*win).w_ptyfd);
                    wfdused = 1 as libc::c_int;
                }
                i += 1;
                i;
            }
            if wfdused != 0 {
                if fcntl((*win).w_ptyfd, 4 as libc::c_int, 0 as libc::c_int) != 0 {
                    Msg(
                        *__errno_location(),
                        b"Warning: clear NBLOCK fcntl failed\0" as *const u8
                            as *const libc::c_char,
                    );
                }
            }
            close((*win).w_ptyfd);
            if slave != -(1 as libc::c_int) {
                close(slave);
            }
            if newfd >= 0 as libc::c_int {
                let mut fakemode: mode = mode {
                    tio: termios {
                        c_iflag: 0,
                        c_oflag: 0,
                        c_cflag: 0,
                        c_lflag: 0,
                        c_line: 0,
                        c_cc: [0; 32],
                        c_ispeed: 0,
                        c_ospeed: 0,
                    },
                };
                let mut modep: *mut mode = 0 as *mut mode;
                InitPTY(newfd);
                if fgtty(newfd) != 0 {
                    Msg(
                        *__errno_location(),
                        b"fgtty\0" as *const u8 as *const libc::c_char,
                    );
                }
                if !display.is_null() {
                    modep = &mut (*display).d_OldMode;
                } else {
                    modep = &mut fakemode;
                    InitTTY(modep, 0 as libc::c_int);
                }
                if !pwin.is_null()
                    && (pat & 0x1000 as libc::c_int == 0
                        || pat & (0x2 as libc::c_int) << 2 as libc::c_int != 0)
                {
                    (*modep).tio.c_lflag &= !(0o10 as libc::c_int) as libc::c_uint;
                    (*modep).tio.c_iflag &= !(0o400 as libc::c_int) as libc::c_uint;
                }
                SetTTY(newfd, modep);
                glwz.ws_col = w as libc::c_ushort;
                glwz.ws_row = h as libc::c_ushort;
                ioctl(
                    newfd,
                    0x5414 as libc::c_int as libc::c_ulong,
                    &mut glwz as *mut winsize as *mut libc::c_char,
                );
                fcntl(newfd, 4 as libc::c_int, 0 as libc::c_int);
            }
            let ref mut fresh13 = *NewEnv.offset(2 as libc::c_int as isize);
            *fresh13 = MakeTermcap(
                (display.is_null() || (*win).w_aflag != 0) as libc::c_int,
            );
            strcpy(
                shellbuf.as_mut_ptr(),
                b"SHELL=\0" as *const u8 as *const libc::c_char,
            );
            strncpy(
                shellbuf.as_mut_ptr().offset(6 as libc::c_int as isize),
                ShellProg
                    .offset(
                        (*ShellProg as libc::c_int == '-' as i32) as libc::c_int as isize,
                    ),
                (::std::mem::size_of::<[libc::c_char; 4103]>() as libc::c_ulong)
                    .wrapping_sub(7 as libc::c_int as libc::c_ulong),
            );
            shellbuf[(::std::mem::size_of::<[libc::c_char; 4103]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                as usize] = 0 as libc::c_int as libc::c_char;
            let ref mut fresh14 = *NewEnv.offset(4 as libc::c_int as isize);
            *fresh14 = shellbuf.as_mut_ptr();
            if !((*win).w_term).is_null() && *(*win).w_term as libc::c_int != 0
                && strcmp(screenterm.as_mut_ptr(), (*win).w_term) != 0
                && strlen((*win).w_term) < 32 as libc::c_int as libc::c_ulong
            {
                let mut s1: *mut libc::c_char = 0 as *mut libc::c_char;
                let mut s2: *mut libc::c_char = 0 as *mut libc::c_char;
                let mut tl: libc::c_char = 0;
                snprintf(
                    tebuf.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 38]>() as libc::c_ulong,
                    b"TERM=%s\0" as *const u8 as *const libc::c_char,
                    (*win).w_term,
                );
                tl = strlen((*win).w_term) as libc::c_char;
                let ref mut fresh15 = *NewEnv.offset(1 as libc::c_int as isize);
                *fresh15 = tebuf.as_mut_ptr();
                s1 = index(*NewEnv.offset(2 as libc::c_int as isize), '|' as i32);
                if !s1.is_null() {
                    s1 = s1.offset(1);
                    s2 = index(s1, '|' as i32);
                    if !s2.is_null() {
                        if (strlen(*NewEnv.offset(2 as libc::c_int as isize)))
                            .wrapping_sub(
                                s2.offset_from(s1) as libc::c_long as libc::c_ulong,
                            )
                            .wrapping_add(tl as libc::c_ulong)
                            < 1024 as libc::c_int as libc::c_ulong
                        {
                            bcopy(
                                s2 as *const libc::c_void,
                                s1.offset(tl as libc::c_int as isize) as *mut libc::c_void,
                                (strlen(s2)).wrapping_add(1 as libc::c_int as libc::c_ulong),
                            );
                            bcopy(
                                (*win).w_term as *const libc::c_void,
                                s1 as *mut libc::c_void,
                                tl as size_t,
                            );
                        }
                    }
                }
            }
            snprintf(
                ebuf.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 20]>() as libc::c_ulong,
                b"WINDOW=%d\0" as *const u8 as *const libc::c_char,
                (*win).w_number,
            );
            let ref mut fresh16 = *NewEnv.offset(3 as libc::c_int as isize);
            *fresh16 = ebuf.as_mut_ptr();
            if *proc_0 as libc::c_int == '-' as i32 {
                proc_0 = proc_0.offset(1);
                proc_0;
            }
            if *proc_0 == 0 {
                proc_0 = DefaultShell.as_mut_ptr();
            }
            execvpe(
                proc_0,
                args as *const *mut libc::c_char,
                NewEnv as *const *mut libc::c_char,
            );
            Panic(
                *__errno_location(),
                b"Cannot exec '%s'\0" as *const u8 as *const libc::c_char,
                proc_0,
            );
        }
        _ => {}
    }
    if slave != -(1 as libc::c_int) {
        close(slave);
    }
    return pid;
}
pub unsafe extern "C" fn winexec(mut av: *mut *mut libc::c_char) -> libc::c_int {
    let mut pp: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut r: libc::c_int = 0 as libc::c_int;
    let mut l: libc::c_int = 0 as libc::c_int;
    let mut w: *mut win = 0 as *mut win;
    extern "C" {
        #[link_name = "display"]
        static mut display_0: *mut display;
    }
    extern "C" {
        #[link_name = "windows"]
        static mut windows_0: *mut win;
    }
    let mut pwin: *mut pseudowin = 0 as *mut pseudowin;
    let mut type_0: libc::c_int = 0;
    w = if !display.is_null() { fore } else { windows };
    if w.is_null() {
        return -(1 as libc::c_int);
    }
    if (*av).is_null() || !((*w).w_pwin).is_null() {
        Msg(
            0 as libc::c_int,
            b"Filter running: %s\0" as *const u8 as *const libc::c_char,
            if !((*w).w_pwin).is_null() {
                ((*(*w).w_pwin).p_cmd).as_mut_ptr() as *const libc::c_char
            } else {
                b"(none)\0" as *const u8 as *const libc::c_char
            },
        );
        return -(1 as libc::c_int);
    }
    if (*w).w_ptyfd < 0 as libc::c_int {
        Msg(
            0 as libc::c_int,
            b"You feel dead inside.\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    pwin = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<pseudowin>() as libc::c_ulong,
    ) as *mut pseudowin;
    if pwin.is_null() {
        Msg(
            0 as libc::c_int,
            b"%s\0" as *const u8 as *const libc::c_char,
            strnomem.as_mut_ptr(),
        );
        return -(1 as libc::c_int);
    }
    s = *av;
    while *s as libc::c_int == ' ' as i32 {
        s = s.offset(1);
        s;
    }
    p = s;
    while *p as libc::c_int == ':' as i32 || *p as libc::c_int == '.' as i32
        || *p as libc::c_int == '!' as i32
    {
        p = p.offset(1);
        p;
    }
    if *p as libc::c_int != '|' as i32 {
        while *p as libc::c_int != 0 && p > s
            && *p.offset(-(1 as libc::c_int) as isize) as libc::c_int == '.' as i32
        {
            p = p.offset(-1);
            p;
        }
    }
    if *p as libc::c_int == '|' as i32 {
        l = 0x1000 as libc::c_int;
        p = p.offset(1);
        p;
    }
    if *p != 0 {
        let ref mut fresh17 = *av.offset(0 as libc::c_int as isize);
        *fresh17 = p;
    } else {
        av = av.offset(1);
        av;
    }
    t = ((*pwin).p_cmd).as_mut_ptr();
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        *t = (if s < p {
            let fresh18 = s;
            s = s.offset(1);
            *fresh18 as libc::c_int
        } else {
            '.' as i32
        }) as libc::c_char;
        let fresh19 = t;
        t = t.offset(1);
        match *fresh19 as libc::c_int {
            46 | 124 => {
                l |= (0x1 as libc::c_int) << i * 2 as libc::c_int;
            }
            33 => {
                l |= (0x2 as libc::c_int) << i * 2 as libc::c_int;
            }
            58 => {
                l |= (0x1 as libc::c_int | 0x2 as libc::c_int) << i * 2 as libc::c_int;
            }
            _ => {}
        }
        i += 1;
        i;
    }
    if l & 0x1000 as libc::c_int != 0 {
        let fresh20 = t;
        t = t.offset(1);
        *fresh20 = '|' as i32 as libc::c_char;
        if l & 0x3 as libc::c_int == 0x1 as libc::c_int {
            *((*pwin).p_cmd).as_mut_ptr() = '!' as i32 as libc::c_char;
            l ^= 0x1 as libc::c_int | 0x2 as libc::c_int;
        }
    }
    if l & 0x2 as libc::c_int == 0 {
        l |= 0x1000 as libc::c_int;
    }
    let fresh21 = t;
    t = t.offset(1);
    *fresh21 = ' ' as i32 as libc::c_char;
    (*pwin).p_fdpat = l;
    l = 768 as libc::c_int - 4 as libc::c_int;
    pp = av;
    while !(*pp).is_null() {
        p = *pp;
        while *p as libc::c_int != 0
            && {
                let fresh22 = l;
                l = l - 1;
                fresh22 > 0 as libc::c_int
            }
        {
            let fresh23 = p;
            p = p.offset(1);
            let fresh24 = t;
            t = t.offset(1);
            *fresh24 = *fresh23;
        }
        if l <= 0 as libc::c_int {
            break;
        }
        let fresh25 = t;
        t = t.offset(1);
        *fresh25 = ' ' as i32 as libc::c_char;
        pp = pp.offset(1);
        pp;
    }
    t = t.offset(-1);
    *t = '\0' as i32 as libc::c_char;
    (*pwin).p_ptyfd = OpenDevice(av, 0 as libc::c_int, &mut type_0, &mut t);
    if (*pwin).p_ptyfd < 0 as libc::c_int {
        free(pwin as *mut libc::c_char as *mut libc::c_void);
        return -(1 as libc::c_int);
    }
    strncpy(
        ((*pwin).p_tty).as_mut_ptr(),
        t,
        (768 as libc::c_int - 1 as libc::c_int) as libc::c_ulong,
    );
    (*w).w_pwin = pwin;
    if type_0 != 0 as libc::c_int {
        FreePseudowin(w);
        Msg(
            0 as libc::c_int,
            b"Cannot only use commands as pseudo win.\0" as *const u8
                as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (*pwin).p_fdpat & 0x1 as libc::c_int == 0 {
        evdeq(&mut (*w).w_readev);
    }
    let mut flag: libc::c_int = 0 as libc::c_int;
    if ioctl(
        (*pwin).p_ptyfd,
        0x5420 as libc::c_int as libc::c_ulong,
        &mut flag as *mut libc::c_int as *mut libc::c_char,
    ) != 0
    {
        Msg(
            *__errno_location(),
            b"TIOCPKT pwin ioctl\0" as *const u8 as *const libc::c_char,
        );
        FreePseudowin(w);
        return -(1 as libc::c_int);
    }
    if (*w).w_type == 0 as libc::c_int && (*pwin).p_fdpat & 0x1 as libc::c_int == 0 {
        if ioctl(
            (*w).w_ptyfd,
            0x5420 as libc::c_int as libc::c_ulong,
            &mut flag as *mut libc::c_int as *mut libc::c_char,
        ) != 0
        {
            Msg(
                *__errno_location(),
                b"TIOCPKT win ioctl\0" as *const u8 as *const libc::c_char,
            );
            FreePseudowin(w);
            return -(1 as libc::c_int);
        }
    }
    (*pwin).p_writeev.fd = (*pwin).p_ptyfd;
    (*pwin).p_readev.fd = (*pwin).p_writeev.fd;
    (*pwin).p_readev.type_0 = 1 as libc::c_int;
    (*pwin).p_writeev.type_0 = 2 as libc::c_int;
    (*pwin).p_writeev.data = w as *mut libc::c_char;
    (*pwin).p_readev.data = (*pwin).p_writeev.data;
    (*pwin)
        .p_readev
        .handler = Some(
        pseu_readev_fn as unsafe extern "C" fn(*mut event, *mut libc::c_char) -> (),
    );
    (*pwin)
        .p_writeev
        .handler = Some(
        pseu_writeev_fn as unsafe extern "C" fn(*mut event, *mut libc::c_char) -> (),
    );
    (*pwin).p_writeev.condpos = &mut (*pwin).p_inlen;
    if (*pwin).p_fdpat
        & ((0x1 as libc::c_int) << 2 as libc::c_int * 2 as libc::c_int
            | (0x1 as libc::c_int) << 2 as libc::c_int) != 0
    {
        evenq(&mut (*pwin).p_readev);
    }
    evenq(&mut (*pwin).p_writeev);
    (*pwin).p_pid = ForkWindow(w, av, t);
    r = (*pwin).p_pid;
    if r < 0 as libc::c_int {
        FreePseudowin(w);
    }
    return r;
}
pub unsafe extern "C" fn FreePseudowin(mut w: *mut win) {
    let mut pwin: *mut pseudowin = (*w).w_pwin;
    if fcntl((*w).w_ptyfd, 4 as libc::c_int, 0o4000 as libc::c_int) != 0 {
        Msg(
            *__errno_location(),
            b"Warning: FreePseudowin: NBLOCK fcntl failed\0" as *const u8
                as *const libc::c_char,
        );
    }
    if (*w).w_type == 0 as libc::c_int && (*pwin).p_fdpat & 0x1 as libc::c_int == 0 {
        let mut flag: libc::c_int = 1 as libc::c_int;
        if ioctl(
            (*w).w_ptyfd,
            0x5420 as libc::c_int as libc::c_ulong,
            &mut flag as *mut libc::c_int as *mut libc::c_char,
        ) != 0
        {
            Msg(
                *__errno_location(),
                b"Warning: FreePseudowin: TIOCPKT win ioctl\0" as *const u8
                    as *const libc::c_char,
            );
        }
    }
    chmod(((*pwin).p_tty).as_mut_ptr(), 0o666 as libc::c_int as __mode_t);
    chown(
        ((*pwin).p_tty).as_mut_ptr(),
        0 as libc::c_int as __uid_t,
        0 as libc::c_int as __gid_t,
    );
    if (*pwin).p_ptyfd >= 0 as libc::c_int {
        close((*pwin).p_ptyfd);
    }
    evdeq(&mut (*pwin).p_readev);
    evdeq(&mut (*pwin).p_writeev);
    if (*w).w_readev.condneg == &mut (*pwin).p_inlen as *mut libc::c_int {
        (*w).w_readev.condneg = 0 as *mut libc::c_int;
        (*w).w_readev.condpos = (*w).w_readev.condneg;
    }
    evenq(&mut (*w).w_readev);
    free(pwin as *mut libc::c_char as *mut libc::c_void);
    (*w).w_pwin = 0 as *mut pseudowin;
}
pub unsafe extern "C" fn ReleaseAutoWritelock(
    mut dis: *mut display,
    mut w: *mut win,
) -> libc::c_int {
    if (*w).w_wlock == 1 as libc::c_int && (*w).w_wlockuser == (*dis).d_user {
        let mut d: *mut display = 0 as *mut display;
        d = displays;
        while !d.is_null() {
            if d != dis && (*d).d_fore == w && (*d).d_user == (*dis).d_user {
                break;
            }
            d = (*d).d_next;
        }
        if d.is_null() {
            (*w).w_wlockuser = 0 as *mut acluser;
            return 0 as libc::c_int;
        }
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn ObtainAutoWritelock(
    mut d: *mut display,
    mut w: *mut win,
) -> libc::c_int {
    if (*w).w_wlock == 1 as libc::c_int
        && AclCheckPermWin((*d).d_user, 1 as libc::c_int, w) == 0
        && ((*w).w_wlockuser).is_null()
    {
        (*w).w_wlockuser = (*d).d_user;
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn paste_slowev_fn(mut ev: *mut event, mut data: *mut libc::c_char) {
    let mut pa: *mut paster = data as *mut paster;
    let mut p: *mut win = 0 as *mut win;
    let mut l: libc::c_int = 1 as libc::c_int;
    flayer = (*pa).pa_pastelayer;
    if flayer.is_null() {
        (*pa).pa_pastelen = 0 as libc::c_int;
    }
    if (*pa).pa_pastelen == 0 {
        return;
    }
    p = (*(*flayer).l_bottom).l_data as *mut win;
    DoProcess(p, &mut (*pa).pa_pasteptr, &mut l, pa);
    (*pa).pa_pastelen -= 1 as libc::c_int - l;
    if (*pa).pa_pastelen > 0 as libc::c_int {
        SetTimeout(&mut (*pa).pa_slowev, (*p).w_slowpaste);
        evenq(&mut (*pa).pa_slowev);
    }
}
unsafe extern "C" fn muchpending(mut p: *mut win, mut ev: *mut event) -> libc::c_int {
    let mut cv: *mut canvas = 0 as *mut canvas;
    cv = (*p).w_layer.l_cvlist;
    while !cv.is_null() {
        display = (*cv).c_display;
        if (*display).d_status == 1 as libc::c_int && (*display).d_status_bell == 0 {
            (*ev).condpos = &mut const_one;
            (*ev).condneg = &mut (*display).d_status;
            return 1 as libc::c_int;
        }
        if !((*display).d_blocked != 0) {
            if ((*display).d_obufp).offset_from((*display).d_obuf) as libc::c_long
                > ((*display).d_obufmax + (*display).d_blocked_fuzz) as libc::c_long
            {
                if (*display).d_nonblock == 0 as libc::c_int {
                    (*display).d_blocked = 1 as libc::c_int;
                } else {
                    (*ev).condpos = &mut (*display).d_obuffree;
                    (*ev).condneg = &mut (*display).d_obuflenmax;
                    if (*display).d_nonblock > 0 as libc::c_int
                        && (*display).d_blockedev.queued == 0
                    {
                        SetTimeout(&mut (*display).d_blockedev, (*display).d_nonblock);
                        evenq(&mut (*display).d_blockedev);
                    }
                    return 1 as libc::c_int;
                }
            }
        }
        cv = (*cv).c_lnext;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn win_readev_fn(mut ev: *mut event, mut data: *mut libc::c_char) {
    let mut p: *mut win = data as *mut win;
    let mut buf: [libc::c_char; 4096] = [0; 4096];
    let mut bp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut size: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut wtop: libc::c_int = 0;
    bp = buf.as_mut_ptr();
    size = 4096 as libc::c_int;
    wtop = (!((*p).w_pwin).is_null()
        && (*(*p).w_pwin).p_fdpat & 0x3 as libc::c_int
            == 0x1 as libc::c_int | 0x2 as libc::c_int) as libc::c_int;
    if wtop != 0 {
        size = 4096 as libc::c_int - (*(*p).w_pwin).p_inlen;
        if size <= 0 as libc::c_int {
            (*ev).condpos = &mut const_IOSIZE;
            (*ev).condneg = &mut (*(*p).w_pwin).p_inlen;
            return;
        }
    }
    if !((*p).w_layer.l_cvlist).is_null() && muchpending(p, ev) != 0 {
        return;
    }
    if ((*p).w_zdisplay).is_null() {
        if (*p).w_blocked != 0 {
            (*ev).condpos = &mut const_one;
            (*ev).condneg = &mut (*p).w_blocked;
            return;
        }
    }
    if !((*ev).condpos).is_null() {
        (*ev).condneg = 0 as *mut libc::c_int;
        (*ev).condpos = (*ev).condneg;
    }
    len = (*p).w_outlen;
    if len != 0 {
        (*p).w_outlen = 0 as libc::c_int;
        WriteString(p, ((*p).w_outbuf).as_mut_ptr(), len);
        return;
    }
    len = read((*ev).fd, buf.as_mut_ptr() as *mut libc::c_void, size as size_t)
        as libc::c_int;
    if len < 0 as libc::c_int {
        if *__errno_location() == 4 as libc::c_int
            || *__errno_location() == 11 as libc::c_int
        {
            return;
        }
        WindowDied(p, 0 as libc::c_int, 0 as libc::c_int);
        return;
    }
    if len == 0 as libc::c_int {
        WindowDied(p, 0 as libc::c_int, 0 as libc::c_int);
        return;
    }
    if (*p).w_type == 0 as libc::c_int {
        if buf[0 as libc::c_int as usize] != 0 {
            if buf[0 as libc::c_int as usize] as libc::c_int & 16 as libc::c_int != 0 {
                WNewAutoFlow(p, 0 as libc::c_int);
            }
            if buf[0 as libc::c_int as usize] as libc::c_int & 32 as libc::c_int != 0 {
                WNewAutoFlow(p, 1 as libc::c_int);
            }
        }
        bp = bp.offset(1);
        bp;
        len -= 1;
        len;
    }
    if len == 0 as libc::c_int {
        return;
    }
    if zmodem_mode != 0 && zmodem_parse(p, bp, len) != 0 {
        return;
    }
    if wtop != 0 {
        bcopy(
            bp as *const libc::c_void,
            ((*(*p).w_pwin).p_inbuf).as_mut_ptr().offset((*(*p).w_pwin).p_inlen as isize)
                as *mut libc::c_void,
            len as size_t,
        );
        (*(*p).w_pwin).p_inlen += len;
    }
    LayPause(&mut (*p).w_layer, 1 as libc::c_int);
    WriteString(p, bp, len);
    LayPause(&mut (*p).w_layer, 0 as libc::c_int);
}
unsafe extern "C" fn win_resurrect_zombie_fn(
    mut ev: *mut event,
    mut data: *mut libc::c_char,
) {
    let mut p: *mut win = data as *mut win;
    if (*p).w_deadpid != (*p).w_pid {
        return;
    }
    WriteString(
        p,
        b"\r\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        2 as libc::c_int,
    );
    RemakeWindow(p);
}
unsafe extern "C" fn win_writeev_fn(mut ev: *mut event, mut data: *mut libc::c_char) {
    let mut p: *mut win = data as *mut win;
    let mut len: libc::c_int = 0;
    if (*p).w_inlen != 0 {
        len = write(
            (*ev).fd,
            ((*p).w_inbuf).as_mut_ptr() as *const libc::c_void,
            (*p).w_inlen as size_t,
        ) as libc::c_int;
        if len <= 0 as libc::c_int {
            len = (*p).w_inlen;
        }
        (*p).w_inlen -= len;
        if (*p).w_inlen != 0 {
            bcopy(
                ((*p).w_inbuf).as_mut_ptr().offset(len as isize) as *const libc::c_void,
                ((*p).w_inbuf).as_mut_ptr() as *mut libc::c_void,
                (*p).w_inlen as size_t,
            );
        }
    }
    if (*p).w_paster.pa_pastelen != 0 && (*p).w_slowpaste == 0 {
        let mut pa: *mut paster = &mut (*p).w_paster;
        flayer = (*pa).pa_pastelayer;
        if !flayer.is_null() {
            DoProcess(p, &mut (*pa).pa_pasteptr, &mut (*pa).pa_pastelen, pa);
        }
    }
}
unsafe extern "C" fn pseu_readev_fn(mut ev: *mut event, mut data: *mut libc::c_char) {
    let mut p: *mut win = data as *mut win;
    let mut buf: [libc::c_char; 4096] = [0; 4096];
    let mut size: libc::c_int = 0;
    let mut ptow: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    size = 4096 as libc::c_int;
    ptow = ((*(*p).w_pwin).p_fdpat & (0x3 as libc::c_int) << 2 as libc::c_int
        == (0x1 as libc::c_int | 0x2 as libc::c_int) << 2 as libc::c_int
        || (*(*p).w_pwin).p_fdpat
            & (0x3 as libc::c_int) << 2 as libc::c_int * 2 as libc::c_int
            == (0x1 as libc::c_int | 0x2 as libc::c_int)
                << 2 as libc::c_int * 2 as libc::c_int) as libc::c_int;
    if ptow != 0 {
        size = 4096 as libc::c_int - (*p).w_inlen;
        if size <= 0 as libc::c_int {
            (*ev).condpos = &mut const_IOSIZE;
            (*ev).condneg = &mut (*p).w_inlen;
            return;
        }
    }
    if !((*p).w_layer.l_cvlist).is_null() && muchpending(p, ev) != 0 {
        return;
    }
    if (*p).w_blocked != 0 {
        (*ev).condpos = &mut const_one;
        (*ev).condneg = &mut (*p).w_blocked;
        return;
    }
    if !((*ev).condpos).is_null() {
        (*ev).condneg = 0 as *mut libc::c_int;
        (*ev).condpos = (*ev).condneg;
    }
    len = (*p).w_outlen;
    if len != 0 {
        (*p).w_outlen = 0 as libc::c_int;
        WriteString(p, ((*p).w_outbuf).as_mut_ptr(), len);
        return;
    }
    len = read((*ev).fd, buf.as_mut_ptr() as *mut libc::c_void, size as size_t)
        as libc::c_int;
    if len <= 0 as libc::c_int {
        if *__errno_location() == 4 as libc::c_int
            || *__errno_location() == 11 as libc::c_int
        {
            return;
        }
        FreePseudowin(p);
        return;
    }
    if ptow != 0 {
        bcopy(
            buf.as_mut_ptr() as *const libc::c_void,
            ((*p).w_inbuf).as_mut_ptr().offset((*p).w_inlen as isize)
                as *mut libc::c_void,
            len as size_t,
        );
        (*p).w_inlen += len;
    }
    WriteString(p, buf.as_mut_ptr(), len);
}
unsafe extern "C" fn pseu_writeev_fn(mut ev: *mut event, mut data: *mut libc::c_char) {
    let mut p: *mut win = data as *mut win;
    let mut pw: *mut pseudowin = (*p).w_pwin;
    let mut len: libc::c_int = 0;
    if (*pw).p_inlen == 0 as libc::c_int {
        return;
    }
    len = write(
        (*ev).fd,
        ((*pw).p_inbuf).as_mut_ptr() as *const libc::c_void,
        (*pw).p_inlen as size_t,
    ) as libc::c_int;
    if len <= 0 as libc::c_int {
        len = (*pw).p_inlen;
    }
    (*(*p).w_pwin).p_inlen -= len;
    if (*(*p).w_pwin).p_inlen != 0 {
        bcopy(
            ((*(*p).w_pwin).p_inbuf).as_mut_ptr().offset(len as isize)
                as *const libc::c_void,
            ((*(*p).w_pwin).p_inbuf).as_mut_ptr() as *mut libc::c_void,
            (*(*p).w_pwin).p_inlen as size_t,
        );
    }
}
unsafe extern "C" fn win_silenceev_fn(mut ev: *mut event, mut data: *mut libc::c_char) {
    let mut p: *mut win = data as *mut win;
    let mut cv: *mut canvas = 0 as *mut canvas;
    display = displays;
    while !display.is_null() {
        cv = (*display).d_cvlist;
        while !cv.is_null() {
            if (*(*cv).c_layer).l_bottom == &mut (*p).w_layer as *mut layer {
                break;
            }
            cv = (*cv).c_next;
        }
        if cv.is_null() {
            if !(*((*p).w_lio_notify)
                .offset(((*(*display).d_user).u_id >> 3 as libc::c_int) as isize)
                as libc::c_int
                & 0x80 as libc::c_int >> ((*(*display).d_user).u_id & 7 as libc::c_int)
                == 0)
            {
                Msg(
                    0 as libc::c_int,
                    b"Window %d: silence for %d seconds\0" as *const u8
                        as *const libc::c_char,
                    (*p).w_number,
                    (*p).w_silencewait,
                );
                (*p).w_silence = 2 as libc::c_int;
                WindowChanged(p, 'f' as i32);
            }
        }
        display = (*display).d_next;
    }
}
unsafe extern "C" fn win_destroyev_fn(mut ev: *mut event, mut data: *mut libc::c_char) {
    let mut p: *mut win = (*ev).data as *mut win;
    WindowDied(p, (*p).w_exitstatus, 1 as libc::c_int);
}
unsafe extern "C" fn zmodem_parse(
    mut p: *mut win,
    mut bp: *mut libc::c_char,
    mut len: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut b2: *mut libc::c_char = bp;
    i = 0 as libc::c_int;
    while i < len {
        if (*p).w_zauto == 0 as libc::c_int {
            while i < len {
                if *b2 as libc::c_int == 0o30 as libc::c_int {
                    break;
                }
                i += 1;
                i;
                b2 = b2.offset(1);
                b2;
            }
            if i == len {
                break;
            }
            if i > 1 as libc::c_int
                && *b2.offset(-(1 as libc::c_int) as isize) as libc::c_int == '*' as i32
                && *b2.offset(-(2 as libc::c_int) as isize) as libc::c_int == '*' as i32
            {
                (*p).w_zauto = 3 as libc::c_int;
            }
        } else if (*p).w_zauto > 5 as libc::c_int
            || *b2 as libc::c_int
                == (*::std::mem::transmute::<
                    &[u8; 7],
                    &[libc::c_char; 7],
                >(b"**\x18B00\0"))[(*p).w_zauto as usize] as libc::c_int
            || (*p).w_zauto == 5 as libc::c_int && *b2 as libc::c_int == '1' as i32
            || (*p).w_zauto == 5 as libc::c_int && !((*p).w_zdisplay).is_null()
                && *b2 as libc::c_int == '8' as i32
        {
            (*p).w_zauto += 1;
            if !((*p).w_zauto < 6 as libc::c_int) {
                if (*p).w_zauto == 6 as libc::c_int {
                    (*p).w_zauto = 0 as libc::c_int;
                }
                if ((*p).w_zdisplay).is_null() {
                    if i > 6 as libc::c_int {
                        WriteString(p, bp, i + 1 as libc::c_int - 6 as libc::c_int);
                    }
                    WriteString(
                        p,
                        b"\r\n\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        2 as libc::c_int,
                    );
                    zmodem_found(
                        p,
                        (*b2 as libc::c_int == '1' as i32) as libc::c_int,
                        b2.offset(1 as libc::c_int as isize),
                        len - i - 1 as libc::c_int,
                    );
                    return 1 as libc::c_int;
                } else if (*p).w_zauto == 7 as libc::c_int
                    || *b2 as libc::c_int == '8' as i32
                {
                    let mut se: libc::c_int = if (*(*p).w_zdisplay).d_blocked
                        == 2 as libc::c_int
                    {
                        'O' as i32
                    } else {
                        -118i32
                    };
                    while i < len {
                        if *b2 as libc::c_int == se {
                            break;
                        }
                        i += 1;
                        i;
                        b2 = b2.offset(1);
                        b2;
                    }
                    if i < len {
                        zmodem_abort(p, 0 as *mut display);
                        (*display).d_blocked = 0 as libc::c_int;
                        (*display).d_readev.condneg = 0 as *mut libc::c_int;
                        (*display).d_readev.condpos = (*display).d_readev.condneg;
                        loop {
                            let fresh26 = len;
                            len = len - 1;
                            if !(fresh26 > 0 as libc::c_int) {
                                break;
                            }
                            (*display).d_obuffree -= 1;
                            if (*display).d_obuffree <= 0 as libc::c_int {
                                Resize_obuf();
                            }
                            let fresh27 = bp;
                            bp = bp.offset(1);
                            let fresh28 = (*display).d_obufp;
                            (*display).d_obufp = ((*display).d_obufp).offset(1);
                            *fresh28 = *fresh27;
                        }
                        Flush(0 as libc::c_int);
                        Activate(
                            if !((*display).d_fore).is_null() {
                                (*(*display).d_fore).w_norefresh as libc::c_int
                            } else {
                                0 as libc::c_int
                            },
                        );
                        return 1 as libc::c_int;
                    }
                    (*p).w_zauto = 6 as libc::c_int;
                }
            }
        } else {
            (*p)
                .w_zauto = if *b2 as libc::c_int == '*' as i32 {
                if (*p).w_zauto == 2 as libc::c_int {
                    2 as libc::c_int
                } else {
                    1 as libc::c_int
                }
            } else {
                0 as libc::c_int
            };
        }
        i += 1;
        i;
        b2 = b2.offset(1);
        b2;
    }
    if (*p).w_zauto == 0 as libc::c_int
        && *bp.offset((len - 1 as libc::c_int) as isize) as libc::c_int == '*' as i32
    {
        (*p)
            .w_zauto = if len > 1 as libc::c_int
            && *bp.offset((len - 2 as libc::c_int) as isize) as libc::c_int == '*' as i32
        {
            2 as libc::c_int
        } else {
            1 as libc::c_int
        };
    }
    if !((*p).w_zdisplay).is_null() {
        display = (*p).w_zdisplay;
        loop {
            let fresh29 = len;
            len = len - 1;
            if !(fresh29 > 0 as libc::c_int) {
                break;
            }
            (*display).d_obuffree -= 1;
            if (*display).d_obuffree <= 0 as libc::c_int {
                Resize_obuf();
            }
            let fresh30 = bp;
            bp = bp.offset(1);
            let fresh31 = (*display).d_obufp;
            (*display).d_obufp = ((*display).d_obufp).offset(1);
            *fresh31 = *fresh30;
        }
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn zmodem_fin(
    mut buf: *mut libc::c_char,
    mut len: libc::c_int,
    mut data: *mut libc::c_char,
) {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut n: libc::c_int = 0;
    if len != 0 {
        RcLine(
            buf,
            (strlen(buf)).wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int,
        );
    } else {
        s = b"\x18\x18\x18\x18\x18\x18\x18\x18\x18\x18\0" as *const u8
            as *const libc::c_char as *mut libc::c_char;
        n = strlen(s) as libc::c_int;
        (Some(((*(*flayer).l_layfn).lf_LayProcess).unwrap())).unwrap()(&mut s, &mut n);
    };
}
unsafe extern "C" fn zmodem_found(
    mut p: *mut win,
    mut send: libc::c_int,
    mut bp: *mut libc::c_char,
    mut len: libc::c_int,
) {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    extern "C" {
        #[link_name = "zmodem_mode"]
        static mut zmodem_mode_0: libc::c_int;
    }
    n = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < len {
        if *bp.offset(i as isize) as libc::c_int != 0o30 as libc::c_int {
            n = 0 as libc::c_int;
        } else {
            n += 1;
            if n > 4 as libc::c_int {
                return;
            }
        }
        i += 1;
        i;
    }
    if zmodem_mode == 3 as libc::c_int
        || zmodem_mode == 1 as libc::c_int && (*p).w_type != 1 as libc::c_int
    {
        let mut d: *mut display = 0 as *mut display;
        let mut olddisplay: *mut display = 0 as *mut display;
        olddisplay = display;
        d = (*p).w_lastdisp;
        if d.is_null() || (*d).d_fore != p {
            d = displays;
            while !d.is_null() {
                if (*d).d_fore == p {
                    break;
                }
                d = (*d).d_next;
            }
        }
        if d.is_null() && !((*p).w_layer.l_cvlist).is_null() {
            d = (*(*p).w_layer.l_cvlist).c_display;
        }
        if d.is_null() {
            d = displays;
        }
        if d.is_null() {
            return;
        }
        display = d;
        RemoveStatus();
        (*p).w_zdisplay = display;
        (*display).d_blocked = 2 as libc::c_int + send;
        flayer = &mut (*p).w_layer;
        ZmodemPage();
        display = d;
        evdeq(&mut (*display).d_blockedev);
        (*display).d_readev.condpos = &mut const_IOSIZE;
        (*display).d_readev.condneg = &mut (*p).w_inlen;
        ClearAll();
        GotoPos(0 as libc::c_int, 0 as libc::c_int);
        SetRendition(&mut mchar_blank);
        AddStr(
            b"Zmodem active\r\n\r\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        AddStr(
            (if send != 0 {
                b"**\x18B01\0" as *const u8 as *const libc::c_char
            } else {
                b"**\x18B00\0" as *const u8 as *const libc::c_char
            }) as *mut libc::c_char,
        );
        loop {
            let fresh32 = len;
            len = len - 1;
            if !(fresh32 > 0 as libc::c_int) {
                break;
            }
            (*display).d_obuffree -= 1;
            if (*display).d_obuffree <= 0 as libc::c_int {
                Resize_obuf();
            }
            let fresh33 = bp;
            bp = bp.offset(1);
            let fresh34 = (*display).d_obufp;
            (*display).d_obufp = ((*display).d_obufp).offset(1);
            *fresh34 = *fresh33;
        }
        display = olddisplay;
        return;
    }
    flayer = &mut (*p).w_layer;
    Input(
        b":\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        768 as libc::c_int,
        0 as libc::c_int,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> ()>,
            Option::<
                unsafe extern "C" fn(
                    *mut libc::c_char,
                    libc::c_int,
                    *mut libc::c_char,
                ) -> (),
            >,
        >(
            Some(
                ::std::mem::transmute::<
                    unsafe extern "C" fn(
                        *mut libc::c_char,
                        libc::c_int,
                        *mut libc::c_char,
                    ) -> (),
                    unsafe extern "C" fn() -> (),
                >(zmodem_fin),
            ),
        ),
        0 as *mut libc::c_char,
        0 as libc::c_int,
    );
    s = if send != 0 { zmodem_sendcmd } else { zmodem_recvcmd };
    n = strlen(s) as libc::c_int;
    (Some(((*(*flayer).l_layfn).lf_LayProcess).unwrap())).unwrap()(&mut s, &mut n);
}
pub unsafe extern "C" fn zmodem_abort(mut p: *mut win, mut d: *mut display) {
    let mut olddisplay: *mut display = display;
    let mut oldflayer: *mut layer = flayer;
    if !p.is_null() {
        if !((*p).w_savelayer).is_null() && !((*(*p).w_savelayer).l_next).is_null() {
            if oldflayer == (*p).w_savelayer {
                oldflayer = (*flayer).l_next;
            }
            flayer = (*p).w_savelayer;
            ExitOverlayPage();
        }
        (*p).w_zdisplay = 0 as *mut display;
        (*p).w_zauto = 0 as libc::c_int;
        LRefreshAll(&mut (*p).w_layer, 0 as libc::c_int);
    }
    if !d.is_null() {
        display = d;
        (*display).d_blocked = 0 as libc::c_int;
        (*display).d_readev.condneg = 0 as *mut libc::c_int;
        (*display).d_readev.condpos = (*display).d_readev.condneg;
        Activate(
            if !((*display).d_fore).is_null() {
                (*(*display).d_fore).w_norefresh as libc::c_int
            } else {
                0 as libc::c_int
            },
        );
    }
    display = olddisplay;
    flayer = oldflayer;
}
pub unsafe extern "C" fn WindowChangeNumber(
    mut old: libc::c_int,
    mut dest: libc::c_int,
) -> libc::c_int {
    let mut p: *mut win = 0 as *mut win;
    let mut win_old: *mut win = 0 as *mut win;
    if dest < 0 as libc::c_int || dest >= maxwin {
        Msg(
            0 as libc::c_int,
            b"Given window position is invalid.\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    win_old = *wtab.offset(old as isize);
    p = *wtab.offset(dest as isize);
    let ref mut fresh35 = *wtab.offset(dest as isize);
    *fresh35 = win_old;
    (*win_old).w_number = dest;
    let ref mut fresh36 = *wtab.offset(old as isize);
    *fresh36 = p;
    if !p.is_null() {
        (*p).w_number = old;
    }
    AclWinSwap(old, dest);
    if (*win_old).w_slot != -(1 as libc::c_int) as slot_t
        && !((*win_old).w_slot).is_null()
    {
        RemoveUtmp(win_old);
        SetUtmp(win_old);
    }
    if !p.is_null() && (*p).w_slot != -(1 as libc::c_int) as slot_t
        && !((*p).w_slot).is_null()
    {
        display = if !((*win_old).w_layer.l_cvlist).is_null() {
            (*(*win_old).w_layer.l_cvlist).c_display
        } else {
            0 as *mut display
        };
        RemoveUtmp(p);
        SetUtmp(p);
    }
    WindowChanged(win_old, 'n' as i32);
    WindowChanged(0 as *mut win, 'w' as i32);
    WindowChanged(0 as *mut win, 'W' as i32);
    WindowChanged(0 as *mut win, 0 as libc::c_int);
    return 1 as libc::c_int;
}
