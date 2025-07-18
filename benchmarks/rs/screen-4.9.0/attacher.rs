use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type logfile;
    fn __xstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn chmod(__file: *const libc::c_char, __mode: __mode_t) -> libc::c_int;
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
    fn kill(__pid: __pid_t, __sig: libc::c_int) -> libc::c_int;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn __errno_location() -> *mut libc::c_int;
    fn access(__name: *const libc::c_char, __type: libc::c_int) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn alarm(__seconds: libc::c_uint) -> libc::c_uint;
    fn sleep(__seconds: libc::c_uint) -> libc::c_uint;
    fn pause() -> libc::c_int;
    fn execl(
        __path: *const libc::c_char,
        __arg: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn getpid() -> __pid_t;
    fn getppid() -> __pid_t;
    fn setuid(__uid: __uid_t) -> libc::c_int;
    fn setgid(__gid: __gid_t) -> libc::c_int;
    fn setresuid(__ruid: __uid_t, __euid: __uid_t, __suid: __uid_t) -> libc::c_int;
    fn fork() -> __pid_t;
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    fn getpass(__prompt: *const libc::c_char) -> *mut libc::c_char;
    fn crypt(
        __key: *const libc::c_char,
        __salt: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    static mut stderr: *mut FILE;
    fn exit(_: libc::c_int) -> !;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn bzero(_: *mut libc::c_void, _: libc::c_ulong);
    fn index(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
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
    fn time(__timer: *mut time_t) -> time_t;
    fn wait(__stat_loc: *mut libc::c_int) -> __pid_t;
    static mut DefaultEsc: libc::c_int;
    static mut DefaultMetaEsc: libc::c_int;
    fn eexit(_: libc::c_int) -> !;
    fn Kill(_: libc::c_int, _: libc::c_int);
    fn Msg(_: libc::c_int, _: *const libc::c_char, _: ...);
    fn Panic(_: libc::c_int, _: *const libc::c_char, _: ...) -> !;
    fn SetTTY(_: libc::c_int, _: *mut mode);
    fn FindSocket(
        _: *mut libc::c_int,
        _: *mut libc::c_int,
        _: *mut libc::c_int,
        _: *mut libc::c_char,
        _: *mut bool,
    ) -> libc::c_int;
    fn MakeClientSocket(_: libc::c_int, _: bool) -> libc::c_int;
    fn MakeServerSocket(_: bool) -> libc::c_int;
    fn SendAttachMsg(_: libc::c_int, _: *mut msg, _: libc::c_int) -> libc::c_int;
    fn ReceiveRaw(_: libc::c_int);
    fn IsSocket(_: *const libc::c_char) -> bool;
    fn SaveStr(_: *const libc::c_char) -> *mut libc::c_char;
    fn closeallfiles(_: libc::c_int);
    fn xsignal(
        _: libc::c_int,
        _: Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
    ) -> Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
    fn xseteuid(_: libc::c_int);
    static mut real_uid: libc::c_int;
    static mut real_gid: libc::c_int;
    static mut eff_uid: libc::c_int;
    static mut eff_gid: libc::c_int;
    static mut ServerSocket: libc::c_int;
    static mut displays: *mut display;
    static mut SockName: *mut libc::c_char;
    static mut SockMatch: *mut libc::c_char;
    static mut SockPath: [libc::c_char; 0];
    static mut HostName: [libc::c_char; 0];
    static mut ppp: *mut passwd;
    static mut attach_tty: *mut libc::c_char;
    static mut attach_term: *mut libc::c_char;
    static mut LoginName: *mut libc::c_char;
    static mut preselect: *mut libc::c_char;
    static mut attach_tty_is_in_new_ns: bool;
    static mut attach_tty_name_in_ns: [libc::c_char; 0];
    static mut xflag: libc::c_int;
    static mut dflag: libc::c_int;
    static mut rflag: libc::c_int;
    static mut quietflag: libc::c_int;
    static mut adaptflag: libc::c_int;
    static mut attach_Mode: mode;
    static mut nwin_options: NewWindow;
    static mut MasterPid: libc::c_int;
    static mut attach_fd: libc::c_int;
    static mut multi: *mut libc::c_char;
    static mut multiattach: libc::c_int;
    static mut multi_uid: libc::c_int;
    static mut own_uid: libc::c_int;
    static mut tty_mode: libc::c_int;
    static mut tty_oldmode: libc::c_int;
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
pub type time_t = __time_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct msg {
    pub protocol_revision: libc::c_int,
    pub type_0: libc::c_int,
    pub m_tty: [libc::c_char; 4096],
    pub m: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub create: C2RustUnnamed_7,
    pub attach: C2RustUnnamed_6,
    pub detach: C2RustUnnamed_5,
    pub command: C2RustUnnamed_4,
    pub message: [libc::c_char; 8192],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub auser: [libc::c_char; 257],
    pub nargs: libc::c_int,
    pub cmd: [libc::c_char; 4096],
    pub apid: libc::c_int,
    pub preselect: [libc::c_char; 20],
    pub writeback: [libc::c_char; 4096],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub duser: [libc::c_char; 257],
    pub dpid: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub auser: [libc::c_char; 257],
    pub apid: libc::c_int,
    pub adaptflag: libc::c_int,
    pub lines: libc::c_int,
    pub columns: libc::c_int,
    pub preselect: [libc::c_char; 20],
    pub esc: libc::c_int,
    pub meta_esc: libc::c_int,
    pub envterm: [libc::c_char; 33],
    pub encoding: libc::c_int,
    pub detachfirst: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub lflag: libc::c_int,
    pub aflag: libc::c_int,
    pub flowflag: libc::c_int,
    pub hheight: libc::c_int,
    pub nargs: libc::c_int,
    pub line: [libc::c_char; 4096],
    pub dir: [libc::c_char; 4096],
    pub screenterm: [libc::c_char; 33],
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
#[inline]
unsafe extern "C" fn stat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __xstat(1 as libc::c_int, __path, __statbuf);
}
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
static mut ContinuePlease: libc::c_int = 0;
unsafe extern "C" fn AttachSigCont(mut sigsig: libc::c_int) {
    ContinuePlease = 1 as libc::c_int;
}
static mut QueryResult: libc::c_int = 0;
unsafe extern "C" fn QueryResultSuccess(mut sigsig: libc::c_int) {
    QueryResult = 1 as libc::c_int;
}
unsafe extern "C" fn QueryResultFail(mut sigsig: libc::c_int) {
    QueryResult = 2 as libc::c_int;
}
unsafe extern "C" fn WriteMessage(mut s: libc::c_int, mut m: *mut msg) -> libc::c_int {
    let mut r: libc::c_int = 0;
    let mut l: libc::c_int = ::std::mem::size_of::<msg>() as libc::c_ulong
        as libc::c_int;
    let mut is_socket: bool = false;
    is_socket = IsSocket(SockPath.as_mut_ptr());
    if is_socket as libc::c_int != 0 && (*m).type_0 == 2 as libc::c_int {
        return SendAttachMsg(s, m, attach_fd);
    }
    while l > 0 as libc::c_int {
        r = write(
            s,
            (m as *mut libc::c_char)
                .offset(
                    (::std::mem::size_of::<msg>() as libc::c_ulong)
                        .wrapping_sub(l as libc::c_ulong) as isize,
                ) as *const libc::c_void,
            l as size_t,
        ) as libc::c_int;
        if r == -(1 as libc::c_int) && *__errno_location() == 4 as libc::c_int {
            continue;
        }
        if r == -(1 as libc::c_int) || r == 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
        l -= r;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn Attach(mut how: libc::c_int) -> libc::c_int {
    let mut n: libc::c_int = 0;
    let mut lasts: libc::c_int = 0;
    let mut m: msg = msg {
        protocol_revision: 0,
        type_0: 0,
        m_tty: [0; 4096],
        m: C2RustUnnamed_3 {
            create: C2RustUnnamed_7 {
                lflag: 0,
                aflag: 0,
                flowflag: 0,
                hheight: 0,
                nargs: 0,
                line: [0; 4096],
                dir: [0; 4096],
                screenterm: [0; 33],
            },
        },
    };
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
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut is_socket: bool = false;
    if (how == 2 as libc::c_int || how == 3 as libc::c_int) && multiattach != 0 {
        real_uid = multi_uid;
        eff_uid = own_uid;
        if setresuid(multi_uid as __uid_t, own_uid as __uid_t, multi_uid as __uid_t) != 0
        {
            Panic(
                *__errno_location(),
                b"setresuid\0" as *const u8 as *const libc::c_char,
            );
        }
        if chmod(attach_tty, 0o666 as libc::c_int as __mode_t) != 0 {
            Panic(
                *__errno_location(),
                b"chmod %s\0" as *const u8 as *const libc::c_char,
                attach_tty,
            );
        }
        tty_oldmode = tty_mode;
    }
    bzero(
        &mut m as *mut msg as *mut libc::c_char as *mut libc::c_void,
        ::std::mem::size_of::<msg>() as libc::c_ulong,
    );
    m.type_0 = how;
    m
        .protocol_revision = ('m' as i32) << 24 as libc::c_int
        | ('s' as i32) << 16 as libc::c_int | ('g' as i32) << 8 as libc::c_int
        | 5 as libc::c_int;
    strncpy(
        (m.m_tty).as_mut_ptr(),
        if attach_tty_is_in_new_ns as libc::c_int != 0 {
            attach_tty_name_in_ns.as_mut_ptr()
        } else {
            attach_tty
        },
        (::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    );
    m
        .m_tty[(::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        as usize] = 0 as libc::c_int as libc::c_char;
    is_socket = IsSocket(SockPath.as_mut_ptr());
    if how == 6 as libc::c_int {
        lasts = MakeClientSocket(0 as libc::c_int, is_socket);
        if lasts >= 0 as libc::c_int {
            WriteMessage(lasts, &mut m);
            close(lasts);
        }
        return 0 as libc::c_int;
    }
    if how == 3 as libc::c_int {
        lasts = MakeClientSocket(0 as libc::c_int, is_socket);
        if lasts < 0 as libc::c_int {
            Panic(
                0 as libc::c_int,
                b"Sorry, cannot contact session \"%s\" again.\r\n\0" as *const u8
                    as *const libc::c_char,
                SockName,
            );
        }
    } else {
        n = FindSocket(
            &mut lasts,
            0 as *mut libc::c_int,
            0 as *mut libc::c_int,
            SockMatch,
            &mut is_socket,
        );
        match n {
            0 => {
                if rflag != 0 && rflag & 1 as libc::c_int == 0 as libc::c_int {
                    return 0 as libc::c_int;
                }
                if quietflag != 0 {
                    eexit(10 as libc::c_int);
                }
                if !SockMatch.is_null() && *SockMatch as libc::c_int != 0 {
                    Panic(
                        0 as libc::c_int,
                        b"There is no screen to be %sed matching %s.\0" as *const u8
                            as *const libc::c_char,
                        if xflag != 0 {
                            b"attach\0" as *const u8 as *const libc::c_char
                        } else if dflag != 0 {
                            b"detach\0" as *const u8 as *const libc::c_char
                        } else {
                            b"resum\0" as *const u8 as *const libc::c_char
                        },
                        SockMatch,
                    );
                } else {
                    Panic(
                        0 as libc::c_int,
                        b"There is no screen to be %sed.\0" as *const u8
                            as *const libc::c_char,
                        if xflag != 0 {
                            b"attach\0" as *const u8 as *const libc::c_char
                        } else if dflag != 0 {
                            b"detach\0" as *const u8 as *const libc::c_char
                        } else {
                            b"resum\0" as *const u8 as *const libc::c_char
                        },
                    );
                }
            }
            1 => {}
            _ => {
                if rflag < 3 as libc::c_int {
                    if quietflag != 0 {
                        eexit(10 as libc::c_int + n);
                    }
                    Panic(
                        0 as libc::c_int,
                        b"Type \"screen [-d] -r [pid.]tty.host\" to resume one of them.\0"
                            as *const u8 as *const libc::c_char,
                    );
                }
            }
        }
    }
    if multiattach == 0 {
        if setuid(real_uid as __uid_t) != 0 {
            Panic(*__errno_location(), b"setuid\0" as *const u8 as *const libc::c_char);
        }
    } else {
        xseteuid(real_uid);
    }
    eff_uid = real_uid;
    if setgid(real_gid as __gid_t) != 0 {
        Panic(*__errno_location(), b"setgid\0" as *const u8 as *const libc::c_char);
    }
    eff_gid = real_gid;
    MasterPid = 0 as libc::c_int;
    s = SockName;
    while *s != 0 {
        if *s as libc::c_int > '9' as i32 || (*s as libc::c_int) < '0' as i32 {
            break;
        }
        MasterPid = 10 as libc::c_int * MasterPid + (*s as libc::c_int - '0' as i32);
        s = s.offset(1);
        s;
    }
    if stat(SockPath.as_mut_ptr(), &mut st) == -(1 as libc::c_int) {
        Panic(
            *__errno_location(),
            b"stat %s\0" as *const u8 as *const libc::c_char,
            SockPath.as_mut_ptr(),
        );
    }
    if st.st_mode & 0o600 as libc::c_int as libc::c_uint
        != 0o600 as libc::c_int as libc::c_uint
    {
        Panic(
            0 as libc::c_int,
            b"Socket is in wrong mode (%03o)\0" as *const u8 as *const libc::c_char,
            st.st_mode as libc::c_int,
        );
    }
    if (xflag != 0 || rflag != 0) && dflag != 0
        && st.st_mode & 0o700 as libc::c_int as libc::c_uint
            == 0o600 as libc::c_int as libc::c_uint
    {
        dflag = 0 as libc::c_int;
    }
    if (dflag != 0 || xflag == 0)
        && st.st_mode & 0o700 as libc::c_int as libc::c_uint
            != (if dflag != 0 { 0o700 as libc::c_int } else { 0o600 as libc::c_int })
                as libc::c_uint
    {
        Panic(
            0 as libc::c_int,
            b"That screen is %sdetached.\0" as *const u8 as *const libc::c_char,
            if dflag != 0 {
                b"already \0" as *const u8 as *const libc::c_char
            } else {
                b"not \0" as *const u8 as *const libc::c_char
            },
        );
    }
    if dflag != 0 && (how == 4 as libc::c_int || how == 5 as libc::c_int) {
        m.m.detach.dpid = getpid();
        strncpy(
            (m.m.detach.duser).as_mut_ptr(),
            LoginName,
            (::std::mem::size_of::<[libc::c_char; 257]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
        m
            .m
            .detach
            .duser[(::std::mem::size_of::<[libc::c_char; 257]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            as usize] = 0 as libc::c_int as libc::c_char;
        if dflag == 2 as libc::c_int {
            m.type_0 = 5 as libc::c_int;
        } else {
            m.type_0 = 4 as libc::c_int;
        }
        xsignal(
            18 as libc::c_int,
            ::std::mem::transmute::<
                Option::<unsafe extern "C" fn() -> ()>,
                Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
            >(
                Some(
                    ::std::mem::transmute::<
                        unsafe extern "C" fn(libc::c_int) -> (),
                        unsafe extern "C" fn() -> (),
                    >(AttachSigCont),
                ),
            ),
        );
        if WriteMessage(lasts, &mut m) != 0 {
            Panic(
                *__errno_location(),
                b"WriteMessage\0" as *const u8 as *const libc::c_char,
            );
        }
        close(lasts);
        while ContinuePlease == 0 {
            pause();
        }
        xsignal(18 as libc::c_int, None);
        ContinuePlease = 0 as libc::c_int;
        if how != 2 as libc::c_int {
            return 0 as libc::c_int;
        }
        sleep(1 as libc::c_int as libc::c_uint);
        lasts = MakeClientSocket(0 as libc::c_int, is_socket);
        if lasts == -(1 as libc::c_int) {
            Panic(
                0 as libc::c_int,
                b"Cannot contact screen again. Sigh.\0" as *const u8
                    as *const libc::c_char,
            );
        }
        m.type_0 = how;
    }
    strncpy(
        (m.m.attach.envterm).as_mut_ptr(),
        attach_term,
        32 as libc::c_int as libc::c_ulong,
    );
    m.m.attach.envterm[32 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    strncpy(
        (m.m.attach.auser).as_mut_ptr(),
        LoginName,
        (::std::mem::size_of::<[libc::c_char; 257]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    );
    m
        .m
        .attach
        .auser[(::std::mem::size_of::<[libc::c_char; 257]>() as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        as usize] = 0 as libc::c_int as libc::c_char;
    m.m.attach.esc = DefaultEsc;
    m.m.attach.meta_esc = DefaultMetaEsc;
    strncpy(
        (m.m.attach.preselect).as_mut_ptr(),
        if !preselect.is_null() {
            preselect as *const libc::c_char
        } else {
            b"\0" as *const u8 as *const libc::c_char
        },
        (::std::mem::size_of::<[libc::c_char; 20]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    );
    m
        .m
        .attach
        .preselect[(::std::mem::size_of::<[libc::c_char; 20]>() as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        as usize] = 0 as libc::c_int as libc::c_char;
    m.m.attach.apid = getpid();
    m.m.attach.adaptflag = adaptflag;
    m.m.attach.columns = 0 as libc::c_int;
    m.m.attach.lines = m.m.attach.columns;
    s = getenv(b"LINES\0" as *const u8 as *const libc::c_char);
    if !s.is_null() {
        m.m.attach.lines = atoi(s);
    }
    s = getenv(b"COLUMNS\0" as *const u8 as *const libc::c_char);
    if !s.is_null() {
        m.m.attach.columns = atoi(s);
    }
    m
        .m
        .attach
        .encoding = if nwin_options.encoding > 0 as libc::c_int {
        nwin_options.encoding + 1 as libc::c_int
    } else {
        0 as libc::c_int
    };
    if dflag == 2 as libc::c_int {
        m.m.attach.detachfirst = 5 as libc::c_int;
    } else if dflag != 0 {
        m.m.attach.detachfirst = 4 as libc::c_int;
    } else {
        m.m.attach.detachfirst = 2 as libc::c_int;
    }
    if !multi.is_null() && (how == 2 as libc::c_int || how == 3 as libc::c_int) {
        xsignal(
            18 as libc::c_int,
            ::std::mem::transmute::<
                Option::<unsafe extern "C" fn() -> ()>,
                Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
            >(
                Some(
                    ::std::mem::transmute::<
                        unsafe extern "C" fn(libc::c_int) -> (),
                        unsafe extern "C" fn() -> (),
                    >(AttachSigCont),
                ),
            ),
        );
    }
    if WriteMessage(lasts, &mut m) != 0 {
        Panic(
            *__errno_location(),
            b"WriteMessage\0" as *const u8 as *const libc::c_char,
        );
    }
    close(lasts);
    if !multi.is_null() && (how == 2 as libc::c_int || how == 3 as libc::c_int) {
        while ContinuePlease == 0 {
            pause();
        }
        xsignal(18 as libc::c_int, None);
        ContinuePlease = 0 as libc::c_int;
        xseteuid(own_uid);
        if tty_oldmode >= 0 as libc::c_int {
            if chmod(attach_tty, tty_oldmode as __mode_t) != 0 {
                Panic(
                    *__errno_location(),
                    b"chmod %s\0" as *const u8 as *const libc::c_char,
                    attach_tty,
                );
            }
        }
        tty_oldmode = -(1 as libc::c_int);
        xseteuid(real_uid);
    }
    rflag = 0 as libc::c_int;
    return 1 as libc::c_int;
}
static mut AttacherPanic: libc::c_int = 0 as libc::c_int;
unsafe extern "C" fn AttacherSigAlarm(mut sigsig: libc::c_int) {}
unsafe extern "C" fn AttacherSigInt(mut sigsig: libc::c_int) {
    xsignal(
        2 as libc::c_int,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> ()>,
            Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
        >(
            Some(
                ::std::mem::transmute::<
                    unsafe extern "C" fn(libc::c_int) -> (),
                    unsafe extern "C" fn() -> (),
                >(AttacherSigInt),
            ),
        ),
    );
    Kill(MasterPid, 2 as libc::c_int);
}
pub unsafe extern "C" fn AttacherFinit(mut sigsig: libc::c_int) {
    let mut statb: stat = stat {
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
    let mut m: msg = msg {
        protocol_revision: 0,
        type_0: 0,
        m_tty: [0; 4096],
        m: C2RustUnnamed_3 {
            create: C2RustUnnamed_7 {
                lflag: 0,
                aflag: 0,
                flowflag: 0,
                hheight: 0,
                nargs: 0,
                line: [0; 4096],
                dir: [0; 4096],
                screenterm: [0; 33],
            },
        },
    };
    let mut s: libc::c_int = 0;
    let mut is_socket: bool = false;
    xsignal(
        1 as libc::c_int,
        ::std::mem::transmute::<
            libc::intptr_t,
            __sighandler_t,
        >(1 as libc::c_int as libc::intptr_t),
    );
    if stat(SockPath.as_mut_ptr(), &mut statb) == 0 as libc::c_int
        && statb.st_mode & 0o777 as libc::c_int as libc::c_uint
            != 0o600 as libc::c_int as libc::c_uint
    {
        bzero(
            &mut m as *mut msg as *mut libc::c_char as *mut libc::c_void,
            ::std::mem::size_of::<msg>() as libc::c_ulong,
        );
        strncpy(
            (m.m_tty).as_mut_ptr(),
            if attach_tty_is_in_new_ns as libc::c_int != 0 {
                attach_tty_name_in_ns.as_mut_ptr()
            } else {
                attach_tty
            },
            (::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
        m
            .m_tty[(::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            as usize] = 0 as libc::c_int as libc::c_char;
        m.m.detach.dpid = getpid();
        m.type_0 = 7 as libc::c_int;
        m
            .protocol_revision = ('m' as i32) << 24 as libc::c_int
            | ('s' as i32) << 16 as libc::c_int | ('g' as i32) << 8 as libc::c_int
            | 5 as libc::c_int;
        is_socket = IsSocket(SockPath.as_mut_ptr());
        s = MakeClientSocket(0 as libc::c_int, is_socket);
        if s >= 0 as libc::c_int {
            WriteMessage(s, &mut m);
            close(s);
        }
    }
    if tty_oldmode >= 0 as libc::c_int {
        if setuid(own_uid as __uid_t) != 0 {
            Panic(*__errno_location(), b"setuid\0" as *const u8 as *const libc::c_char);
        }
        chmod(attach_tty, tty_oldmode as __mode_t);
    }
    exit(0 as libc::c_int);
}
unsafe extern "C" fn AttacherFinitBye(mut sigsig: libc::c_int) {
    let mut ppid: libc::c_int = 0;
    if setgid(real_gid as __gid_t) != 0 {
        Panic(*__errno_location(), b"setgid\0" as *const u8 as *const libc::c_char);
    }
    if setuid(own_uid as __uid_t) != 0 {
        Panic(*__errno_location(), b"setuid\0" as *const u8 as *const libc::c_char);
    }
    ppid = getppid();
    if ppid > 1 as libc::c_int {
        Kill(ppid, 1 as libc::c_int);
    }
    exit(0 as libc::c_int);
}
static mut SuspendPlease: libc::c_int = 0;
unsafe extern "C" fn SigStop(mut sigsig: libc::c_int) {
    SuspendPlease = 1 as libc::c_int;
}
static mut LockPlease: libc::c_int = 0;
unsafe extern "C" fn DoLock(mut sigsig: libc::c_int) {
    LockPlease = 1 as libc::c_int;
}
static mut SigWinchPlease: libc::c_int = 0;
unsafe extern "C" fn AttacherWinch(mut sigsig: libc::c_int) {
    SigWinchPlease = 1 as libc::c_int;
}
pub unsafe extern "C" fn Attacher() {
    xsignal(
        1 as libc::c_int,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> ()>,
            Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
        >(
            Some(
                ::std::mem::transmute::<
                    unsafe extern "C" fn(libc::c_int) -> (),
                    unsafe extern "C" fn() -> (),
                >(AttacherFinit),
            ),
        ),
    );
    xsignal(
        1 as libc::c_int,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> ()>,
            Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
        >(
            Some(
                ::std::mem::transmute::<
                    unsafe extern "C" fn(libc::c_int) -> (),
                    unsafe extern "C" fn() -> (),
                >(AttacherFinit),
            ),
        ),
    );
    xsignal(
        10 as libc::c_int,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> ()>,
            Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
        >(
            Some(
                ::std::mem::transmute::<
                    unsafe extern "C" fn(libc::c_int) -> (),
                    unsafe extern "C" fn() -> (),
                >(AttacherFinitBye),
            ),
        ),
    );
    xsignal(
        12 as libc::c_int,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> ()>,
            Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
        >(
            Some(
                ::std::mem::transmute::<
                    unsafe extern "C" fn(libc::c_int) -> (),
                    unsafe extern "C" fn() -> (),
                >(DoLock),
            ),
        ),
    );
    xsignal(
        2 as libc::c_int,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> ()>,
            Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
        >(
            Some(
                ::std::mem::transmute::<
                    unsafe extern "C" fn(libc::c_int) -> (),
                    unsafe extern "C" fn() -> (),
                >(AttacherSigInt),
            ),
        ),
    );
    xsignal(
        20 as libc::c_int,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> ()>,
            Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
        >(
            Some(
                ::std::mem::transmute::<
                    unsafe extern "C" fn(libc::c_int) -> (),
                    unsafe extern "C" fn() -> (),
                >(SigStop),
            ),
        ),
    );
    xsignal(
        28 as libc::c_int,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> ()>,
            Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
        >(
            Some(
                ::std::mem::transmute::<
                    unsafe extern "C" fn(libc::c_int) -> (),
                    unsafe extern "C" fn() -> (),
                >(AttacherWinch),
            ),
        ),
    );
    dflag = 0 as libc::c_int;
    xflag = 1 as libc::c_int;
    loop {
        xsignal(
            14 as libc::c_int,
            ::std::mem::transmute::<
                Option::<unsafe extern "C" fn() -> ()>,
                Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
            >(
                Some(
                    ::std::mem::transmute::<
                        unsafe extern "C" fn(libc::c_int) -> (),
                        unsafe extern "C" fn() -> (),
                    >(AttacherSigAlarm),
                ),
            ),
        );
        alarm(15 as libc::c_int as libc::c_uint);
        pause();
        alarm(0 as libc::c_int as libc::c_uint);
        if kill(MasterPid, 0 as libc::c_int) < 0 as libc::c_int
            && *__errno_location() != 1 as libc::c_int
        {
            AttacherPanic += 1;
            AttacherPanic;
        }
        if AttacherPanic != 0 {
            fcntl(0 as libc::c_int, 4 as libc::c_int, 0 as libc::c_int);
            SetTTY(0 as libc::c_int, &mut attach_Mode);
            printf(
                b"\nError: Cannot find master process to attach to!\n\0" as *const u8
                    as *const libc::c_char,
            );
            eexit(1 as libc::c_int);
        }
        if SuspendPlease != 0 {
            SuspendPlease = 0 as libc::c_int;
            xsignal(20 as libc::c_int, None);
            kill(getpid(), 20 as libc::c_int);
            xsignal(
                20 as libc::c_int,
                ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn() -> ()>,
                    Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
                >(
                    Some(
                        ::std::mem::transmute::<
                            unsafe extern "C" fn(libc::c_int) -> (),
                            unsafe extern "C" fn() -> (),
                        >(SigStop),
                    ),
                ),
            );
            Attach(3 as libc::c_int);
        }
        if LockPlease != 0 {
            LockPlease = 0 as libc::c_int;
            LockTerminal();
            Attach(3 as libc::c_int);
        }
        if SigWinchPlease != 0 {
            SigWinchPlease = 0 as libc::c_int;
            Attach(6 as libc::c_int);
        }
    };
}
static mut LockEnd: [libc::c_char; 27] = unsafe {
    *::std::mem::transmute::<
        &[u8; 27],
        &mut [libc::c_char; 27],
    >(b"Welcome back to screen !!\n\0")
};
unsafe extern "C" fn LockHup(mut sigsig: libc::c_int) {
    let mut ppid: libc::c_int = getppid();
    if setgid(real_gid as __gid_t) != 0 {
        Panic(*__errno_location(), b"setgid\0" as *const u8 as *const libc::c_char);
    }
    if setuid(own_uid as __uid_t) != 0 {
        Panic(*__errno_location(), b"setuid\0" as *const u8 as *const libc::c_char);
    }
    if ppid > 1 as libc::c_int {
        Kill(ppid, 1 as libc::c_int);
    }
    exit(0 as libc::c_int);
}
unsafe extern "C" fn LockTerminal() {
    let mut prg: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sig: libc::c_int = 0;
    let mut pid: libc::c_int = 0;
    let mut sigs: [Option::<unsafe extern "C" fn(libc::c_int) -> ()>; 65] = [None; 65];
    sig = 1 as libc::c_int;
    while sig < 64 as libc::c_int + 1 as libc::c_int {
        sigs[sig
            as usize] = xsignal(
            sig,
            if sig == 17 as libc::c_int {
                None
            } else {
                ::std::mem::transmute::<
                    libc::intptr_t,
                    __sighandler_t,
                >(1 as libc::c_int as libc::intptr_t)
            },
        );
        sig += 1;
        sig;
    }
    xsignal(
        1 as libc::c_int,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> ()>,
            Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
        >(
            Some(
                ::std::mem::transmute::<
                    unsafe extern "C" fn(libc::c_int) -> (),
                    unsafe extern "C" fn() -> (),
                >(LockHup),
            ),
        ),
    );
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    prg = getenv(b"LOCKPRG\0" as *const u8 as *const libc::c_char);
    if !prg.is_null()
        && strcmp(prg, b"builtin\0" as *const u8 as *const libc::c_char) != 0
        && access(prg, 1 as libc::c_int) == 0
    {
        xsignal(17 as libc::c_int, None);
        pid = fork();
        if pid == 0 as libc::c_int {
            displays = 0 as *mut display;
            ServerSocket = -(1 as libc::c_int);
            if setgid(real_gid as __gid_t) != 0 {
                Panic(
                    *__errno_location(),
                    b"setgid\0" as *const u8 as *const libc::c_char,
                );
            }
            if setuid(own_uid as __uid_t) != 0 {
                Panic(
                    *__errno_location(),
                    b"setuid\0" as *const u8 as *const libc::c_char,
                );
            }
            closeallfiles(0 as libc::c_int);
            execl(
                prg,
                b"SCREEN-LOCK\0" as *const u8 as *const libc::c_char,
                0 as *mut libc::c_void,
            );
            exit(*__errno_location());
        }
        if pid == -(1 as libc::c_int) {
            Msg(
                *__errno_location(),
                b"Cannot lock terminal - fork failed\0" as *const u8
                    as *const libc::c_char,
            );
        } else {
            let mut wstat: libc::c_int = 0;
            let mut wret: libc::c_int = 0;
            *__errno_location() = 0 as libc::c_int;
            loop {
                wret = wait(&mut wstat);
                if !(wret != pid
                    || wret == -(1 as libc::c_int)
                        && *__errno_location() == 4 as libc::c_int)
                {
                    break;
                }
                *__errno_location() = 0 as libc::c_int;
            }
            if *__errno_location() != 0 {
                Msg(*__errno_location(), b"Lock\0" as *const u8 as *const libc::c_char);
                sleep(2 as libc::c_int as libc::c_uint);
            } else if wstat & 0x7f as libc::c_int != 0 as libc::c_int {
                fprintf(
                    stderr,
                    b"Lock: %s: Killed by signal: %d%s\n\0" as *const u8
                        as *const libc::c_char,
                    prg,
                    wstat & 0x7f as libc::c_int,
                    if wstat & 0x80 as libc::c_int != 0 {
                        b" (Core dumped)\0" as *const u8 as *const libc::c_char
                    } else {
                        b"\0" as *const u8 as *const libc::c_char
                    },
                );
                sleep(2 as libc::c_int as libc::c_uint);
            } else if !((wstat & 0xff00 as libc::c_int) >> 8 as libc::c_int != 0) {
                printf(
                    b"%s\0" as *const u8 as *const libc::c_char,
                    LockEnd.as_mut_ptr(),
                );
            }
        }
    } else {
        !prg.is_null();
        screen_builtin_lck();
    }
    sig = 1 as libc::c_int;
    while sig < 64 as libc::c_int + 1 as libc::c_int {
        if sigs[sig as usize]
            != ::std::mem::transmute::<
                libc::intptr_t,
                Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
            >(-(1 as libc::c_int) as libc::intptr_t)
        {
            xsignal(sig, sigs[sig as usize]);
        }
        sig += 1;
        sig;
    }
}
unsafe extern "C" fn screen_builtin_lck() {
    let mut fullname: [libc::c_char; 100] = [0; 100];
    let mut cp1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut message: [libc::c_char; 200] = [0; 200];
    let mut pass: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut mypass: [libc::c_char; 17] = [0; 17];
    let mut salt: [libc::c_char; 3] = [0; 3];
    let mut using_pam: libc::c_int = 1 as libc::c_int;
    using_pam = 0 as libc::c_int;
    pass = (*ppp).pw_passwd;
    if pass.is_null() || *pass as libc::c_int == 0 as libc::c_int {
        pass = getpass(b"Key:   \0" as *const u8 as *const libc::c_char);
        if !pass.is_null() {
            strncpy(
                mypass.as_mut_ptr(),
                pass,
                (::std::mem::size_of::<[libc::c_char; 17]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            );
            mypass[(::std::mem::size_of::<[libc::c_char; 17]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                as usize] = 0 as libc::c_int as libc::c_char;
            if *mypass.as_mut_ptr() as libc::c_int == 0 as libc::c_int {
                return;
            }
            pass = getpass(b"Again: \0" as *const u8 as *const libc::c_char);
            if !pass.is_null() {
                if strcmp(mypass.as_mut_ptr(), pass) != 0 {
                    fprintf(
                        stderr,
                        b"Passwords don't match.\x07\n\0" as *const u8
                            as *const libc::c_char,
                    );
                    sleep(2 as libc::c_int as libc::c_uint);
                    return;
                }
            }
        }
        if pass.is_null() {
            fprintf(
                stderr,
                b"Getpass error.\x07\n\0" as *const u8 as *const libc::c_char,
            );
            sleep(2 as libc::c_int as libc::c_uint);
            return;
        }
        salt[0 as libc::c_int
            as usize] = ('A' as i32
            + (time(0 as *mut time_t) % 26 as libc::c_int as libc::c_long)
                as libc::c_int) as libc::c_char;
        salt[1 as libc::c_int
            as usize] = ('A' as i32
            + ((time(0 as *mut time_t) >> 6 as libc::c_int)
                % 26 as libc::c_int as libc::c_long) as libc::c_int) as libc::c_char;
        salt[2 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        pass = crypt(mypass.as_mut_ptr(), salt.as_mut_ptr());
        if pass.is_null() {
            fprintf(
                stderr,
                b"crypt() error.\x07\n\0" as *const u8 as *const libc::c_char,
            );
            sleep(2 as libc::c_int as libc::c_uint);
            return;
        }
        (*ppp).pw_passwd = SaveStr(pass);
        pass = (*ppp).pw_passwd;
    }
    strncpy(
        fullname.as_mut_ptr(),
        (*ppp).pw_gecos,
        (::std::mem::size_of::<[libc::c_char; 100]>() as libc::c_ulong)
            .wrapping_sub(9 as libc::c_int as libc::c_ulong),
    );
    fullname[(::std::mem::size_of::<[libc::c_char; 100]>() as libc::c_ulong)
        .wrapping_sub(9 as libc::c_int as libc::c_ulong)
        as usize] = 0 as libc::c_int as libc::c_char;
    cp1 = index(fullname.as_mut_ptr(), ',' as i32);
    if !cp1.is_null() {
        *cp1 = '\0' as i32 as libc::c_char;
    }
    cp1 = index(fullname.as_mut_ptr(), '&' as i32);
    if !cp1.is_null() {
        strncpy(cp1, (*ppp).pw_name, 8 as libc::c_int as libc::c_ulong);
        *cp1.offset(8 as libc::c_int as isize) = 0 as libc::c_int as libc::c_char;
        if *cp1 as libc::c_int >= 'a' as i32 && *cp1 as libc::c_int <= 'z' as i32 {
            *cp1 = (*cp1 as libc::c_int - ('a' as i32 - 'A' as i32)) as libc::c_char;
        }
    }
    sprintf(
        message.as_mut_ptr(),
        b"Screen used by %s%s<%s> on %s.\nPassword:\x07\0" as *const u8
            as *const libc::c_char,
        fullname.as_mut_ptr(),
        if fullname[0 as libc::c_int as usize] as libc::c_int != 0 {
            b" \0" as *const u8 as *const libc::c_char
        } else {
            b"\0" as *const u8 as *const libc::c_char
        },
        (*ppp).pw_name,
        HostName.as_mut_ptr(),
    );
    loop {
        *__errno_location() = 0 as libc::c_int;
        cp1 = getpass(message.as_mut_ptr());
        if cp1.is_null() {
            AttacherFinit(0 as libc::c_int);
        }
        if !(using_pam != 0) {
            let mut buf: *mut libc::c_char = crypt(cp1, pass);
            if !buf.is_null() && strncmp(buf, pass, strlen(pass)) == 0 {
                break;
            }
        }
        bzero(cp1 as *mut libc::c_void, strlen(cp1));
    }
    bzero(cp1 as *mut libc::c_void, strlen(cp1));
}
pub unsafe extern "C" fn SendCmdMessage(
    mut sty: *mut libc::c_char,
    mut match_0: *mut libc::c_char,
    mut av: *mut *mut libc::c_char,
    mut query: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut s: libc::c_int = 0;
    let mut m: msg = msg {
        protocol_revision: 0,
        type_0: 0,
        m_tty: [0; 4096],
        m: C2RustUnnamed_3 {
            create: C2RustUnnamed_7 {
                lflag: 0,
                aflag: 0,
                flowflag: 0,
                hheight: 0,
                nargs: 0,
                line: [0; 4096],
                dir: [0; 4096],
                screenterm: [0; 33],
            },
        },
    };
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut is_socket: bool = false;
    if sty.is_null() {
        i = FindSocket(
            &mut s,
            0 as *mut libc::c_int,
            0 as *mut libc::c_int,
            match_0,
            &mut is_socket,
        );
        if i == 0 as libc::c_int {
            Panic(
                0 as libc::c_int,
                b"No screen session found.\0" as *const u8 as *const libc::c_char,
            );
        }
        if i != 1 as libc::c_int {
            Panic(
                0 as libc::c_int,
                b"Use -S to specify a session.\0" as *const u8 as *const libc::c_char,
            );
        }
    } else {
        if strlen(sty) > 255 as libc::c_int as libc::c_ulong {
            *sty.offset(255 as libc::c_int as isize) = 0 as libc::c_int as libc::c_char;
        }
        if strlen(sty)
            > (2 as libc::c_int * 768 as libc::c_int - 1 as libc::c_int) as libc::c_ulong
        {
            *sty
                .offset(
                    (2 as libc::c_int * 768 as libc::c_int - 1 as libc::c_int) as isize,
                ) = 0 as libc::c_int as libc::c_char;
        }
        sprintf(
            SockPath.as_mut_ptr().offset(strlen(SockPath.as_mut_ptr()) as isize),
            b"/%s\0" as *const u8 as *const libc::c_char,
            sty,
        );
        is_socket = IsSocket(SockPath.as_mut_ptr());
        s = MakeClientSocket(1 as libc::c_int, is_socket);
        if s == -(1 as libc::c_int) {
            exit(1 as libc::c_int);
        }
    }
    bzero(
        &mut m as *mut msg as *mut libc::c_char as *mut libc::c_void,
        ::std::mem::size_of::<msg>() as libc::c_ulong,
    );
    m.type_0 = if query != 0 { 9 as libc::c_int } else { 8 as libc::c_int };
    if !attach_tty.is_null() {
        strncpy(
            (m.m_tty).as_mut_ptr(),
            if attach_tty_is_in_new_ns as libc::c_int != 0 {
                attach_tty_name_in_ns.as_mut_ptr()
            } else {
                attach_tty
            },
            (::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
        m
            .m_tty[(::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            as usize] = 0 as libc::c_int as libc::c_char;
    }
    p = (m.m.command.cmd).as_mut_ptr();
    n = 0 as libc::c_int;
    while !(*av).is_null() && n < 64 as libc::c_int - 1 as libc::c_int {
        len = (strlen(*av)).wrapping_add(1 as libc::c_int as libc::c_ulong)
            as libc::c_int;
        if p.offset(len as isize)
            >= (m.m.command.cmd)
                .as_mut_ptr()
                .offset(
                    ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong
                        as isize,
                )
                .offset(-(1 as libc::c_int as isize))
        {
            break;
        }
        strcpy(p, *av);
        p = p.offset(len as isize);
        av = av.offset(1);
        av;
        n += 1;
        n;
    }
    *p = 0 as libc::c_int as libc::c_char;
    m.m.command.nargs = n;
    strncpy(
        (m.m.attach.auser).as_mut_ptr(),
        LoginName,
        (::std::mem::size_of::<[libc::c_char; 257]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    );
    m
        .m
        .command
        .auser[(::std::mem::size_of::<[libc::c_char; 257]>() as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        as usize] = 0 as libc::c_int as libc::c_char;
    m
        .protocol_revision = ('m' as i32) << 24 as libc::c_int
        | ('s' as i32) << 16 as libc::c_int | ('g' as i32) << 8 as libc::c_int
        | 5 as libc::c_int;
    strncpy(
        (m.m.command.preselect).as_mut_ptr(),
        if !preselect.is_null() {
            preselect as *const libc::c_char
        } else {
            b"\0" as *const u8 as *const libc::c_char
        },
        (::std::mem::size_of::<[libc::c_char; 20]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    );
    m
        .m
        .command
        .preselect[(::std::mem::size_of::<[libc::c_char; 20]>() as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        as usize] = 0 as libc::c_int as libc::c_char;
    m.m.command.apid = getpid();
    if query != 0 {
        let mut sp: *mut libc::c_char = SockPath
            .as_mut_ptr()
            .offset(strlen(SockPath.as_mut_ptr()) as isize);
        let mut query_0: [libc::c_char; 8] = *::std::mem::transmute::<
            &[u8; 8],
            &mut [libc::c_char; 8],
        >(b"-queryX\0");
        let mut c: libc::c_char = 0;
        let mut r: libc::c_int = -(1 as libc::c_int);
        c = 'A' as i32 as libc::c_char;
        while c as libc::c_int <= 'Z' as i32 {
            query_0[6 as libc::c_int as usize] = c;
            strcpy(sp, query_0.as_mut_ptr());
            r = MakeServerSocket(is_socket);
            if r >= 0 as libc::c_int {
                break;
            }
            c += 1;
            c;
        }
        if r < 0 as libc::c_int {
            c = '0' as i32 as libc::c_char;
            while c as libc::c_int <= '9' as i32 {
                query_0[6 as libc::c_int as usize] = c;
                strcpy(sp, query_0.as_mut_ptr());
                r = MakeServerSocket(is_socket);
                if r >= 0 as libc::c_int {
                    break;
                }
                c += 1;
                c;
            }
        }
        if r < 0 as libc::c_int {
            Panic(
                0 as libc::c_int,
                b"Could not create a listening socket to read the results.\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        strncpy(
            (m.m.command.writeback).as_mut_ptr(),
            SockPath.as_mut_ptr(),
            (::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
        m
            .m
            .command
            .writeback[(::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            as usize] = '\0' as i32 as libc::c_char;
        xsignal(
            18 as libc::c_int,
            ::std::mem::transmute::<
                Option::<unsafe extern "C" fn() -> ()>,
                Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
            >(
                Some(
                    ::std::mem::transmute::<
                        unsafe extern "C" fn(libc::c_int) -> (),
                        unsafe extern "C" fn() -> (),
                    >(QueryResultSuccess),
                ),
            ),
        );
        xsignal(
            1 as libc::c_int,
            ::std::mem::transmute::<
                Option::<unsafe extern "C" fn() -> ()>,
                Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
            >(
                Some(
                    ::std::mem::transmute::<
                        unsafe extern "C" fn(libc::c_int) -> (),
                        unsafe extern "C" fn() -> (),
                    >(QueryResultFail),
                ),
            ),
        );
        if WriteMessage(s, &mut m) != 0 {
            Msg(*__errno_location(), b"write\0" as *const u8 as *const libc::c_char);
        }
        close(s);
        while QueryResult == 0 {
            pause();
        }
        xsignal(18 as libc::c_int, None);
        xsignal(1 as libc::c_int, None);
        ReceiveRaw(r);
        unlink(SockPath.as_mut_ptr());
        if QueryResult == 2 as libc::c_int {
            exit(1 as libc::c_int);
        }
    } else {
        if WriteMessage(s, &mut m) != 0 {
            Msg(*__errno_location(), b"write\0" as *const u8 as *const libc::c_char);
        }
        close(s);
    };
}
