use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type logfile;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn __fxstat(
        __ver: libc::c_int,
        __fildes: libc::c_int,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn __lxstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn getpwnam(__name: *const libc::c_char) -> *mut passwd;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fdopen(__fd: libc::c_int, __modes: *const libc::c_char) -> *mut FILE;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn putc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn fgets(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn access(__name: *const libc::c_char, __type: libc::c_int) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn pipe(__pipedes: *mut libc::c_int) -> libc::c_int;
    fn dup(__fd: libc::c_int) -> libc::c_int;
    fn execl(
        __path: *const libc::c_char,
        __arg: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn execvp(
        __file: *const libc::c_char,
        __argv: *const *mut libc::c_char,
    ) -> libc::c_int;
    fn setuid(__uid: __uid_t) -> libc::c_int;
    fn setgid(__gid: __gid_t) -> libc::c_int;
    fn fork() -> __pid_t;
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    fn ftruncate(__fd: libc::c_int, __length: __off_t) -> libc::c_int;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn abort() -> !;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
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
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    static mut strnomem: [libc::c_char; 0];
    fn Msg(_: libc::c_int, _: *const libc::c_char, _: ...);
    fn Panic(_: libc::c_int, _: *const libc::c_char, _: ...) -> !;
    fn WMsg(_: *mut win, _: libc::c_int, _: *mut libc::c_char);
    fn DoCommand(_: *mut *mut libc::c_char, _: *mut libc::c_int);
    fn Parse(
        _: *mut libc::c_char,
        _: libc::c_int,
        _: *mut *mut libc::c_char,
        _: *mut libc::c_int,
    ) -> libc::c_int;
    fn DumpTermcap(_: libc::c_int, _: *mut FILE);
    fn AddStr(_: *mut libc::c_char);
    fn Flush(_: libc::c_int);
    fn DisplaySleep1000(_: libc::c_int, _: libc::c_int);
    fn SaveStr(_: *const libc::c_char) -> *mut libc::c_char;
    fn closeallfiles(_: libc::c_int);
    fn UserContext() -> libc::c_int;
    fn UserReturn(_: libc::c_int);
    fn UserStatus() -> libc::c_int;
    fn xsignal(
        _: libc::c_int,
        _: Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
    ) -> Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
    fn xseteuid(_: libc::c_int);
    fn xsetegid(_: libc::c_int);
    static mut display: *mut display;
    static mut displays: *mut display;
    static mut fore: *mut win;
    static mut flayer: *mut layer;
    static mut ServerSocket: libc::c_int;
    static mut real_uid: libc::c_int;
    static mut eff_uid: libc::c_int;
    static mut real_gid: libc::c_int;
    static mut eff_gid: libc::c_int;
    static mut extra_incap: *mut libc::c_char;
    static mut extra_outcap: *mut libc::c_char;
    static mut home: *mut libc::c_char;
    static mut RcFileName: *mut libc::c_char;
    static mut SockPath: [libc::c_char; 0];
    static mut SockName: *mut libc::c_char;
    static mut BufferFile: *mut libc::c_char;
    static mut hardcopy_append: libc::c_int;
    static mut hardcopydir: *mut libc::c_char;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct passwd {
    pub pw_name: *mut libc::c_char,
    pub pw_passwd: *mut libc::c_char,
    pub pw_uid: __uid_t,
    pub pw_gid: __gid_t,
    pub pw_gecos: *mut libc::c_char,
    pub pw_dir: *mut libc::c_char,
    pub pw_shell: *mut libc::c_char,
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
pub type __sighandler_t = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
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
#[inline]
unsafe extern "C" fn lstat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __lxstat(1 as libc::c_int, __path, __statbuf);
}
#[inline]
unsafe extern "C" fn fstat(
    mut __fd: libc::c_int,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __fxstat(1 as libc::c_int, __fd, __statbuf);
}
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
pub static mut rc_name: *mut libc::c_char = b"\0" as *const u8 as *const libc::c_char
    as *mut libc::c_char;
pub static mut rc_recursion: libc::c_int = 0 as libc::c_int;
unsafe extern "C" fn CatExtra(
    mut str1: *mut libc::c_char,
    mut str2: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len1: libc::c_int = 0;
    let mut len2: libc::c_int = 0;
    let mut add_colon: libc::c_int = 0;
    len1 = strlen(str1) as libc::c_int;
    if len1 == 0 as libc::c_int {
        return str2;
    }
    add_colon = (*str1.offset((len1 - 1 as libc::c_int) as isize) as libc::c_int
        != ':' as i32) as libc::c_int;
    if !str2.is_null() {
        len2 = strlen(str2) as libc::c_int;
        cp = realloc(
            str2 as *mut libc::c_void,
            (len1 as libc::c_uint)
                .wrapping_add(len2 as libc::c_uint)
                .wrapping_add(add_colon as libc::c_uint)
                .wrapping_add(1 as libc::c_int as libc::c_uint) as libc::c_ulong,
        ) as *mut libc::c_char;
        if cp.is_null() {
            Panic(
                0 as libc::c_int,
                b"%s\0" as *const u8 as *const libc::c_char,
                strnomem.as_mut_ptr(),
            );
        }
        bcopy(
            cp as *const libc::c_void,
            cp.offset(len1 as isize).offset(add_colon as isize) as *mut libc::c_void,
            (len2 + 1 as libc::c_int) as size_t,
        );
    } else {
        cp = malloc(
            (len1 as libc::c_uint)
                .wrapping_add(add_colon as libc::c_uint)
                .wrapping_add(1 as libc::c_int as libc::c_uint) as libc::c_ulong,
        ) as *mut libc::c_char;
        if cp.is_null() {
            Panic(
                0 as libc::c_int,
                b"%s\0" as *const u8 as *const libc::c_char,
                strnomem.as_mut_ptr(),
            );
        }
        *cp.offset((len1 + add_colon) as isize) = '\0' as i32 as libc::c_char;
    }
    bcopy(str1 as *const libc::c_void, cp as *mut libc::c_void, len1 as size_t);
    if add_colon != 0 {
        *cp.offset(len1 as isize) = ':' as i32 as libc::c_char;
    }
    return cp;
}
unsafe extern "C" fn findrcfile(mut rcfile: *mut libc::c_char) -> *mut libc::c_char {
    let mut buf: [libc::c_char; 256] = [0; 256];
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    if !rcfile.is_null() && *rcfile as libc::c_int == '~' as i32 {
        static mut rcfilename_tilde_exp: [libc::c_char; 4097] = [0; 4097];
        let mut slash_position: *mut libc::c_char = strchr(rcfile, '/' as i32);
        if slash_position == rcfile.offset(1 as libc::c_int as isize) {
            let mut home_0: *mut libc::c_char = getenv(
                b"HOME\0" as *const u8 as *const libc::c_char,
            );
            if home_0.is_null() {
                Msg(
                    0 as libc::c_int,
                    b"%s: source: tilde expansion failed\0" as *const u8
                        as *const libc::c_char,
                    rc_name,
                );
                return 0 as *mut libc::c_char;
            }
            snprintf(
                rcfilename_tilde_exp.as_mut_ptr(),
                4096 as libc::c_int as libc::c_ulong,
                b"%s/%s\0" as *const u8 as *const libc::c_char,
                home_0,
                rcfile.offset(2 as libc::c_int as isize),
            );
        } else if !slash_position.is_null() {
            let mut p_0: *mut passwd = 0 as *mut passwd;
            *slash_position = 0 as libc::c_int as libc::c_char;
            p_0 = getpwnam(rcfile.offset(1 as libc::c_int as isize));
            if p_0.is_null() {
                Msg(
                    0 as libc::c_int,
                    b"%s: source: tilde expansion failed for user %s\0" as *const u8
                        as *const libc::c_char,
                    rc_name,
                    rcfile.offset(1 as libc::c_int as isize),
                );
                return 0 as *mut libc::c_char;
            }
            snprintf(
                rcfilename_tilde_exp.as_mut_ptr(),
                4096 as libc::c_int as libc::c_ulong,
                b"%s/%s\0" as *const u8 as *const libc::c_char,
                (*p_0).pw_dir,
                slash_position.offset(1 as libc::c_int as isize),
            );
        } else {
            Msg(
                0 as libc::c_int,
                b"%s: source: illegal tilde expression.\0" as *const u8
                    as *const libc::c_char,
                rc_name,
            );
            return 0 as *mut libc::c_char;
        }
        rcfile = rcfilename_tilde_exp.as_mut_ptr();
    }
    if !rcfile.is_null() {
        let mut rcend: *mut libc::c_char = rindex(rc_name, '/' as i32);
        if *rcfile as libc::c_int != '/' as i32 && !rcend.is_null()
            && (rcend.offset_from(rc_name) as libc::c_long as libc::c_ulong)
                .wrapping_add(strlen(rcfile))
                .wrapping_add(2 as libc::c_int as libc::c_ulong)
                < ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong
        {
            strncpy(
                buf.as_mut_ptr(),
                rc_name,
                (rcend.offset_from(rc_name) as libc::c_long
                    + 1 as libc::c_int as libc::c_long) as libc::c_ulong,
            );
            strcpy(
                buf
                    .as_mut_ptr()
                    .offset(rcend.offset_from(rc_name) as libc::c_long as isize)
                    .offset(1 as libc::c_int as isize),
                rcfile,
            );
            if access(buf.as_mut_ptr(), 4 as libc::c_int) == 0 as libc::c_int {
                return SaveStr(buf.as_mut_ptr());
            }
        }
        return SaveStr(rcfile);
    }
    p = getenv(b"SCREENRC\0" as *const u8 as *const libc::c_char);
    if !p.is_null() && *p as libc::c_int != '\0' as i32 {
        return SaveStr(p)
    } else {
        if strlen(home)
            > (::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong)
                .wrapping_sub(12 as libc::c_int as libc::c_ulong)
        {
            Panic(
                0 as libc::c_int,
                b"Rc: home too large\0" as *const u8 as *const libc::c_char,
            );
        }
        sprintf(
            buf.as_mut_ptr(),
            b"%s/.screenrc\0" as *const u8 as *const libc::c_char,
            home,
        );
        return SaveStr(buf.as_mut_ptr());
    };
}
pub unsafe extern "C" fn StartRc(
    mut rcfilename: *mut libc::c_char,
    mut nopanic: libc::c_int,
) -> libc::c_int {
    let mut argc: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut buf: [libc::c_char; 2048] = [0; 2048];
    let mut args: [*mut libc::c_char; 64] = [0 as *mut libc::c_char; 64];
    let mut argl: [libc::c_int; 64] = [0; 64];
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut oldrc_name: *mut libc::c_char = rc_name;
    extra_incap = CatExtra(
        b"TF\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        extra_incap,
    );
    if !display.is_null()
        && (strncmp(
            ((*display).d_termname).as_mut_ptr(),
            b"vt\0" as *const u8 as *const libc::c_char,
            2 as libc::c_int as libc::c_ulong,
        ) == 0
            || strncmp(
                ((*display).d_termname).as_mut_ptr(),
                b"xterm\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int as libc::c_ulong,
            ) == 0)
    {
        extra_incap = CatExtra(
            b"xn:f0=\x1BOp:f1=\x1BOq:f2=\x1BOr:f3=\x1BOs:f4=\x1BOt:f5=\x1BOu:f6=\x1BOv:f7=\x1BOw:f8=\x1BOx:f9=\x1BOy:f.=\x1BOn:f,=\x1BOl:fe=\x1BOM:f+=\x1BOk:f-=\x1BOm:f*=\x1BOj:f/=\x1BOo:fq=\x1BOX\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            extra_incap,
        );
    }
    rc_name = findrcfile(rcfilename);
    if rc_name.is_null()
        || {
            fp = secfopen(
                rc_name,
                b"r\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            fp.is_null()
        }
    {
        let mut rc_nonnull: *const libc::c_char = if !rc_name.is_null() {
            rc_name
        } else {
            rcfilename
        };
        if rc_recursion == 0 && !RcFileName.is_null()
            && strcmp(RcFileName, rc_nonnull) == 0
        {
            if nopanic == 0 {
                Panic(
                    0 as libc::c_int,
                    b"Unable to open \"%s\".\0" as *const u8 as *const libc::c_char,
                    rc_nonnull,
                );
            }
        }
        if !rc_name.is_null() {
            if rc_name.is_null() {
                abort();
            } else {
                free(rc_name as *mut libc::c_void);
            }
            rc_name = 0 as *mut libc::c_char;
        }
        rc_name = oldrc_name;
        return 1 as libc::c_int;
    }
    while !(fgets(
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 2048]>() as libc::c_ulong as libc::c_int,
        fp,
    ))
        .is_null()
    {
        p = rindex(buf.as_mut_ptr(), '\n' as i32);
        if !p.is_null() {
            *p = '\0' as i32 as libc::c_char;
        }
        argc = Parse(
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 2048]>() as libc::c_ulong
                as libc::c_int,
            args.as_mut_ptr(),
            argl.as_mut_ptr(),
        );
        if argc == 0 as libc::c_int {
            continue;
        }
        if strcmp(
            args[0 as libc::c_int as usize],
            b"echo\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            if display.is_null() {
                continue;
            }
            if argc < 2 as libc::c_int
                || argc == 3 as libc::c_int
                    && strcmp(
                        args[1 as libc::c_int as usize],
                        b"-n\0" as *const u8 as *const libc::c_char,
                    ) != 0 || argc > 3 as libc::c_int
            {
                Msg(
                    0 as libc::c_int,
                    b"%s: 'echo [-n] \"string\"' expected.\0" as *const u8
                        as *const libc::c_char,
                    rc_name,
                );
            } else {
                AddStr(args[(argc - 1 as libc::c_int) as usize]);
                if argc != 3 as libc::c_int {
                    AddStr(
                        b"\r\n\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    );
                    Flush(0 as libc::c_int);
                }
            }
        } else if strcmp(
            args[0 as libc::c_int as usize],
            b"sleep\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            if display.is_null() {
                continue;
            }
            if argc != 2 as libc::c_int {
                Msg(
                    0 as libc::c_int,
                    b"%s: sleep: one numeric argument expected.\0" as *const u8
                        as *const libc::c_char,
                    rc_name,
                );
            } else {
                DisplaySleep1000(
                    1000 as libc::c_int * atoi(args[1 as libc::c_int as usize]),
                    1 as libc::c_int,
                );
            }
        } else if strcmp(
            args[0 as libc::c_int as usize],
            b"termcapinfo\0" as *const u8 as *const libc::c_char,
        ) == 0
            || strcmp(
                args[0 as libc::c_int as usize],
                b"terminfo\0" as *const u8 as *const libc::c_char,
            ) == 0
        {
            if display.is_null() {
                continue;
            }
            if argc < 3 as libc::c_int || argc > 4 as libc::c_int {
                Msg(
                    0 as libc::c_int,
                    b"%s: %s: incorrect number of arguments.\0" as *const u8
                        as *const libc::c_char,
                    rc_name,
                    args[0 as libc::c_int as usize],
                );
            } else {
                p = args[1 as libc::c_int as usize];
                while !p.is_null() && *p as libc::c_int != 0 {
                    cp = index(p, '|' as i32);
                    if !cp.is_null() {
                        let fresh0 = cp;
                        cp = cp.offset(1);
                        *fresh0 = '\0' as i32 as libc::c_char;
                    }
                    len = strlen(p) as libc::c_int;
                    if *p.offset((len - 1 as libc::c_int) as isize) as libc::c_int
                        == '*' as i32
                    {
                        if len - 1 as libc::c_int == 0
                            || strncmp(
                                p,
                                ((*display).d_termname).as_mut_ptr(),
                                (len - 1 as libc::c_int) as libc::c_ulong,
                            ) == 0
                        {
                            break;
                        }
                    } else if strcmp(p, ((*display).d_termname).as_mut_ptr()) == 0 {
                        break;
                    }
                    p = cp;
                }
                if !(!p.is_null() && *p as libc::c_int != 0) {
                    continue;
                }
                extra_incap = CatExtra(args[2 as libc::c_int as usize], extra_incap);
                if argc == 4 as libc::c_int {
                    extra_outcap = CatExtra(
                        args[3 as libc::c_int as usize],
                        extra_outcap,
                    );
                }
            }
        } else if strcmp(
            args[0 as libc::c_int as usize],
            b"source\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            if rc_recursion <= 10 as libc::c_int {
                rc_recursion += 1;
                rc_recursion;
                StartRc(args[1 as libc::c_int as usize], 0 as libc::c_int);
                rc_recursion -= 1;
                rc_recursion;
            }
        }
    }
    fclose(fp);
    if rc_name.is_null() {
        abort();
    } else {
        free(rc_name as *mut libc::c_void);
    }
    rc_name = 0 as *mut libc::c_char;
    rc_name = oldrc_name;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn FinishRc(mut rcfilename: *mut libc::c_char) {
    let mut buf: [libc::c_char; 2048] = [0; 2048];
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut oldrc_name: *mut libc::c_char = rc_name;
    rc_name = findrcfile(rcfilename);
    if rc_name.is_null()
        || {
            fp = secfopen(
                rc_name,
                b"r\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            fp.is_null()
        }
    {
        let mut rc_nonnull: *const libc::c_char = if !rc_name.is_null() {
            rc_name
        } else {
            rcfilename
        };
        if rc_recursion != 0 {
            Msg(
                *__errno_location(),
                b"%s: source %s\0" as *const u8 as *const libc::c_char,
                oldrc_name,
                rc_nonnull,
            );
        } else if !RcFileName.is_null() && strcmp(RcFileName, rc_nonnull) == 0 {
            Panic(
                0 as libc::c_int,
                b"Unable to open \"%s\".\0" as *const u8 as *const libc::c_char,
                rc_nonnull,
            );
        }
        if !rc_name.is_null() {
            if rc_name.is_null() {
                abort();
            } else {
                free(rc_name as *mut libc::c_void);
            }
            rc_name = 0 as *mut libc::c_char;
        }
        rc_name = oldrc_name;
        return;
    }
    while !(fgets(
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 2048]>() as libc::c_ulong as libc::c_int,
        fp,
    ))
        .is_null()
    {
        RcLine(
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 2048]>() as libc::c_ulong as libc::c_int,
        );
    }
    fclose(fp);
    if rc_name.is_null() {
        abort();
    } else {
        free(rc_name as *mut libc::c_void);
    }
    rc_name = 0 as *mut libc::c_char;
    rc_name = oldrc_name;
}
pub unsafe extern "C" fn do_source(mut rcfilename: *mut libc::c_char) {
    if rc_recursion > 10 as libc::c_int {
        Msg(
            0 as libc::c_int,
            b"%s: source: recursion limit reached\0" as *const u8 as *const libc::c_char,
            rc_name,
        );
        return;
    }
    rc_recursion += 1;
    rc_recursion;
    FinishRc(rcfilename);
    rc_recursion -= 1;
    rc_recursion;
}
pub unsafe extern "C" fn RcLine(mut ubuf: *mut libc::c_char, mut ubufl: libc::c_int) {
    let mut args: [*mut libc::c_char; 64] = [0 as *mut libc::c_char; 64];
    let mut argl: [libc::c_int; 64] = [0; 64];
    extern "C" {
        static mut EffectiveAclUser: *mut acluser;
    }
    extern "C" {
        static mut users: *mut acluser;
    }
    if !display.is_null() {
        fore = (*display).d_fore;
        flayer = (*(*display).d_forecv).c_layer;
    } else {
        flayer = if !fore.is_null() { (*fore).w_savelayer } else { 0 as *mut layer };
    }
    if Parse(ubuf, ubufl, args.as_mut_ptr(), argl.as_mut_ptr()) <= 0 as libc::c_int {
        return;
    }
    if display.is_null() {
        EffectiveAclUser = users;
    }
    DoCommand(args.as_mut_ptr(), argl.as_mut_ptr());
    EffectiveAclUser = 0 as *mut acluser;
}
pub unsafe extern "C" fn WriteFile(
    mut user: *mut acluser,
    mut fn_0: *mut libc::c_char,
    mut dump: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut f: *mut FILE = 0 as *mut FILE;
    let mut fnbuf: [libc::c_char; 1024] = [0; 1024];
    let mut mode: *mut libc::c_char = b"w\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    let mut public: libc::c_int = 0 as libc::c_int;
    let mut stb: stat = stat {
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
    let mut stb2: stat = stat {
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
    let mut fd: libc::c_int = 0;
    let mut exists: libc::c_int = 0 as libc::c_int;
    match dump {
        0 => {
            if fn_0.is_null() {
                i = SockName.offset_from(SockPath.as_mut_ptr()) as libc::c_long
                    as libc::c_int;
                if i
                    > ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong
                        as libc::c_int - 9 as libc::c_int
                {
                    i = 0 as libc::c_int;
                }
                strncpy(fnbuf.as_mut_ptr(), SockPath.as_mut_ptr(), i as libc::c_ulong);
                strcpy(
                    fnbuf.as_mut_ptr().offset(i as isize),
                    b".termcap\0" as *const u8 as *const libc::c_char,
                );
                fn_0 = fnbuf.as_mut_ptr();
            }
        }
        1 | 3 => {
            if fn_0.is_null() {
                if fore.is_null() {
                    return;
                }
                if !hardcopydir.is_null() && *hardcopydir as libc::c_int != 0
                    && strlen(hardcopydir)
                        < (::std::mem::size_of::<[libc::c_char; 1024]>()
                            as libc::c_ulong)
                            .wrapping_sub(21 as libc::c_int as libc::c_ulong)
                {
                    sprintf(
                        fnbuf.as_mut_ptr(),
                        b"%s/hardcopy.%d\0" as *const u8 as *const libc::c_char,
                        hardcopydir,
                        (*fore).w_number,
                    );
                } else {
                    sprintf(
                        fnbuf.as_mut_ptr(),
                        b"hardcopy.%d\0" as *const u8 as *const libc::c_char,
                        (*fore).w_number,
                    );
                }
                fn_0 = fnbuf.as_mut_ptr();
            }
            if hardcopy_append != 0 && access(fn_0, 2 as libc::c_int) == 0 {
                mode = b"a\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
            }
        }
        2 => {
            if fn_0.is_null() {
                strncpy(
                    fnbuf.as_mut_ptr(),
                    BufferFile,
                    (::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                );
                fnbuf[(::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    as usize] = 0 as libc::c_int as libc::c_char;
                fn_0 = fnbuf.as_mut_ptr();
            }
            public = (strcmp(
                fn_0,
                b"/tmp/screen-exchange\0" as *const u8 as *const libc::c_char,
            ) == 0) as libc::c_int;
            exists = (lstat(fn_0, &mut stb) == 0) as libc::c_int;
            if public != 0 && exists != 0
                && (stb.st_mode & 0o170000 as libc::c_int as libc::c_uint
                    == 0o120000 as libc::c_int as libc::c_uint
                    || stb.st_nlink > 1 as libc::c_int as libc::c_ulong)
            {
                Msg(
                    0 as libc::c_int,
                    b"No write to links, please.\0" as *const u8 as *const libc::c_char,
                );
                return;
            }
        }
        _ => {}
    }
    if UserContext() > 0 as libc::c_int {
        if dump == 2 as libc::c_int && public != 0 {
            if exists != 0 {
                fd = open(fn_0, 0o1 as libc::c_int, 0o666 as libc::c_int);
                if fd >= 0 as libc::c_int {
                    if fstat(fd, &mut stb2) == 0 as libc::c_int
                        && stb.st_dev == stb2.st_dev && stb.st_ino == stb2.st_ino
                    {
                        ftruncate(fd, 0 as libc::c_int as __off_t);
                    } else {
                        close(fd);
                        fd = -(1 as libc::c_int);
                    }
                }
            } else {
                fd = open(
                    fn_0,
                    0o1 as libc::c_int | 0o100 as libc::c_int | 0o200 as libc::c_int,
                    0o666 as libc::c_int,
                );
            }
            f = if fd >= 0 as libc::c_int { fdopen(fd, mode) } else { 0 as *mut FILE };
        } else {
            f = fopen(fn_0, mode);
        }
        if f.is_null() {
            UserReturn(0 as libc::c_int);
        } else {
            match dump {
                1 | 3 => {
                    if !fore.is_null() {
                        if *mode as libc::c_int == 'a' as i32 {
                            putc('>' as i32, f);
                            j = (*fore).w_layer.l_width - 2 as libc::c_int;
                            while j > 0 as libc::c_int {
                                putc('=' as i32, f);
                                j -= 1;
                                j;
                            }
                            fputs(b"<\n\0" as *const u8 as *const libc::c_char, f);
                        }
                        if dump == 3 as libc::c_int {
                            i = (*fore).w_histheight - (*fore).w_scrollback_height;
                            while i < (*fore).w_histheight {
                                p = (*if i < (*fore).w_histheight {
                                    &mut *((*fore).w_hlines)
                                        .offset(
                                            (((*fore).w_histidx + i) % (*fore).w_histheight) as isize,
                                        ) as *mut mline
                                } else {
                                    &mut *((*fore).w_mlines)
                                        .offset((i - (*fore).w_histheight) as isize) as *mut mline
                                })
                                    .image as *mut libc::c_char;
                                k = (*fore).w_layer.l_width - 1 as libc::c_int;
                                while k >= 0 as libc::c_int
                                    && *p.offset(k as isize) as libc::c_int == ' ' as i32
                                {
                                    k -= 1;
                                    k;
                                }
                                j = 0 as libc::c_int;
                                while j <= k {
                                    putc(*p.offset(j as isize) as libc::c_int, f);
                                    j += 1;
                                    j;
                                }
                                putc('\n' as i32, f);
                                i += 1;
                                i;
                            }
                        }
                        i = 0 as libc::c_int;
                        while i < (*fore).w_layer.l_height {
                            p = (*((*fore).w_mlines).offset(i as isize)).image
                                as *mut libc::c_char;
                            k = (*fore).w_layer.l_width - 1 as libc::c_int;
                            while k >= 0 as libc::c_int
                                && *p.offset(k as isize) as libc::c_int == ' ' as i32
                            {
                                k -= 1;
                                k;
                            }
                            j = 0 as libc::c_int;
                            while j <= k {
                                putc(*p.offset(j as isize) as libc::c_int, f);
                                j += 1;
                                j;
                            }
                            putc('\n' as i32, f);
                            i += 1;
                            i;
                        }
                    }
                }
                0 => {
                    DumpTermcap((*fore).w_aflag, f);
                }
                2 => {
                    p = (*user).u_plop.buf;
                    i = (*user).u_plop.len;
                    loop {
                        let fresh1 = i;
                        i = i - 1;
                        if !(fresh1 > 0 as libc::c_int) {
                            break;
                        }
                        if *p as libc::c_int == '\r' as i32
                            && (i == 0 as libc::c_int
                                || *p.offset(1 as libc::c_int as isize) as libc::c_int
                                    != '\n' as i32)
                        {
                            putc('\n' as i32, f);
                        } else {
                            putc(*p as libc::c_int, f);
                        }
                        p = p.offset(1);
                        p;
                    }
                }
                _ => {}
            }
            fclose(f);
            UserReturn(1 as libc::c_int);
        }
    }
    if UserStatus() <= 0 as libc::c_int {
        Msg(
            0 as libc::c_int,
            b"Cannot open \"%s\"\0" as *const u8 as *const libc::c_char,
            fn_0,
        );
    } else if !display.is_null() && *rc_name == 0 {
        match dump {
            0 => {
                Msg(
                    0 as libc::c_int,
                    b"Termcap entry written to \"%s\".\0" as *const u8
                        as *const libc::c_char,
                    fn_0,
                );
            }
            1 | 3 => {
                Msg(
                    0 as libc::c_int,
                    b"Screen image %s to \"%s\".\0" as *const u8 as *const libc::c_char,
                    if *mode as libc::c_int == 'a' as i32 {
                        b"appended\0" as *const u8 as *const libc::c_char
                    } else {
                        b"written\0" as *const u8 as *const libc::c_char
                    },
                    fn_0,
                );
            }
            2 => {
                Msg(
                    0 as libc::c_int,
                    b"Copybuffer written to \"%s\".\0" as *const u8
                        as *const libc::c_char,
                    fn_0,
                );
            }
            _ => {}
        }
    }
}
pub unsafe extern "C" fn ReadFile(
    mut fn_0: *mut libc::c_char,
    mut lenp: *mut libc::c_int,
) -> *mut libc::c_char {
    let mut i: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    let mut c: libc::c_char = 0;
    let mut bp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut stb: stat = stat {
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
    i = secopen(fn_0, 0 as libc::c_int, 0 as libc::c_int);
    if i < 0 as libc::c_int {
        Msg(
            *__errno_location(),
            b"no %s -- no slurp\0" as *const u8 as *const libc::c_char,
            fn_0,
        );
        return 0 as *mut libc::c_char;
    }
    if fstat(i, &mut stb) != 0 {
        Msg(
            *__errno_location(),
            b"no good %s -- no slurp\0" as *const u8 as *const libc::c_char,
            fn_0,
        );
        close(i);
        return 0 as *mut libc::c_char;
    }
    size = stb.st_size as libc::c_int;
    buf = malloc(size as libc::c_ulong) as *mut libc::c_char;
    if buf.is_null() {
        close(i);
        Msg(
            0 as libc::c_int,
            b"%s\0" as *const u8 as *const libc::c_char,
            strnomem.as_mut_ptr(),
        );
        return 0 as *mut libc::c_char;
    }
    *__errno_location() = 0 as libc::c_int;
    l = read(i, buf as *mut libc::c_void, size as size_t) as libc::c_int;
    if l != size {
        if l < 0 as libc::c_int {
            l = 0 as libc::c_int;
        }
        Msg(
            *__errno_location(),
            b"Got only %d bytes from %s\0" as *const u8 as *const libc::c_char,
            l,
            fn_0,
        );
    } else if read(
        i,
        &mut c as *mut libc::c_char as *mut libc::c_void,
        1 as libc::c_int as size_t,
    ) > 0 as libc::c_int as libc::c_long
    {
        Msg(
            0 as libc::c_int,
            b"Slurped only %d characters (of %d) into buffer - try again\0" as *const u8
                as *const libc::c_char,
            l,
            size,
        );
    } else {
        Msg(
            0 as libc::c_int,
            b"Slurped %d characters into buffer\0" as *const u8 as *const libc::c_char,
            l,
        );
    }
    close(i);
    *lenp = l;
    bp = buf;
    loop {
        let fresh2 = l;
        l = l - 1;
        if !(fresh2 > 0 as libc::c_int) {
            break;
        }
        if *bp as libc::c_int == '\n' as i32
            && (bp == buf
                || *bp.offset(-(1 as libc::c_int) as isize) as libc::c_int
                    != '\r' as i32)
        {
            *bp = '\r' as i32 as libc::c_char;
        }
        bp = bp.offset(1);
        bp;
    }
    return buf;
}
pub unsafe extern "C" fn KillBuffers() {
    if UserContext() > 0 as libc::c_int {
        UserReturn(
            if unlink(BufferFile) != 0 { *__errno_location() } else { 0 as libc::c_int },
        );
    }
    *__errno_location() = UserStatus();
    Msg(
        *__errno_location(),
        b"%s %sremoved\0" as *const u8 as *const libc::c_char,
        BufferFile,
        if *__errno_location() != 0 {
            b"not \0" as *const u8 as *const libc::c_char
        } else {
            b"\0" as *const u8 as *const libc::c_char
        },
    );
}
pub unsafe extern "C" fn secfopen(
    mut name: *mut libc::c_char,
    mut mode: *mut libc::c_char,
) -> *mut FILE {
    let mut fi: *mut FILE = 0 as *mut FILE;
    xseteuid(real_uid);
    xsetegid(real_gid);
    fi = fopen(name, mode);
    xseteuid(eff_uid);
    xsetegid(eff_gid);
    return fi;
}
pub unsafe extern "C" fn secopen(
    mut name: *mut libc::c_char,
    mut flags: libc::c_int,
    mut mode: libc::c_int,
) -> libc::c_int {
    let mut fd: libc::c_int = 0;
    xseteuid(real_uid);
    xsetegid(real_gid);
    fd = open(name, flags, mode);
    xseteuid(eff_uid);
    xsetegid(eff_gid);
    return fd;
}
pub unsafe extern "C" fn printpipe(
    mut p: *mut win,
    mut cmd: *mut libc::c_char,
) -> libc::c_int {
    let mut pi: [libc::c_int; 2] = [0; 2];
    if pipe(pi.as_mut_ptr()) != 0 {
        WMsg(
            p,
            *__errno_location(),
            b"printing pipe\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    match fork() {
        -1 => {
            WMsg(
                p,
                *__errno_location(),
                b"printing fork\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            return -(1 as libc::c_int);
        }
        0 => {
            display = (*p).w_pdisplay;
            displays = 0 as *mut display;
            ServerSocket = -(1 as libc::c_int);
            close(0 as libc::c_int);
            dup(pi[0 as libc::c_int as usize]);
            closeallfiles(0 as libc::c_int);
            if setgid(real_gid as __gid_t) != 0 || setuid(real_uid as __uid_t) != 0 {
                Panic(
                    *__errno_location(),
                    b"printpipe setuid\0" as *const u8 as *const libc::c_char,
                );
            }
            eff_uid = real_uid;
            eff_gid = real_gid;
            xsignal(13 as libc::c_int, None);
            execl(
                b"/bin/sh\0" as *const u8 as *const libc::c_char,
                b"sh\0" as *const u8 as *const libc::c_char,
                b"-c\0" as *const u8 as *const libc::c_char,
                cmd,
                0 as *mut libc::c_char,
            );
            Panic(*__errno_location(), b"/bin/sh\0" as *const u8 as *const libc::c_char);
        }
        _ => {}
    }
    close(pi[0 as libc::c_int as usize]);
    return pi[1 as libc::c_int as usize];
}
pub unsafe extern "C" fn readpipe(mut cmdv: *mut *mut libc::c_char) -> libc::c_int {
    let mut pi: [libc::c_int; 2] = [0; 2];
    if pipe(pi.as_mut_ptr()) != 0 {
        Msg(*__errno_location(), b"pipe\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    match fork() {
        -1 => {
            Msg(*__errno_location(), b"fork\0" as *const u8 as *const libc::c_char);
            return -(1 as libc::c_int);
        }
        0 => {
            displays = 0 as *mut display;
            ServerSocket = -(1 as libc::c_int);
            close(1 as libc::c_int);
            if dup(pi[1 as libc::c_int as usize]) != 1 as libc::c_int {
                close(pi[1 as libc::c_int as usize]);
                Panic(0 as libc::c_int, b"dup\0" as *const u8 as *const libc::c_char);
            }
            closeallfiles(1 as libc::c_int);
            if setgid(real_gid as __gid_t) != 0 || setuid(real_uid as __uid_t) != 0 {
                close(1 as libc::c_int);
                Panic(
                    *__errno_location(),
                    b"setuid/setgid\0" as *const u8 as *const libc::c_char,
                );
            }
            eff_uid = real_uid;
            eff_gid = real_gid;
            xsignal(13 as libc::c_int, None);
            execvp(*cmdv, cmdv as *const *mut libc::c_char);
            close(1 as libc::c_int);
            Panic(
                *__errno_location(),
                b"%s\0" as *const u8 as *const libc::c_char,
                *cmdv,
            );
        }
        _ => {}
    }
    close(pi[1 as libc::c_int as usize]);
    return pi[0 as libc::c_int as usize];
}
