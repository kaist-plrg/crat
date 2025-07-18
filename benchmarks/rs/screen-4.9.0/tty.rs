use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type logfile;
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn __lxstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, _: ...) -> libc::c_int;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn __errno_location() -> *mut libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn alarm(__seconds: libc::c_uint) -> libc::c_uint;
    fn getpid() -> __pid_t;
    fn setsid() -> __pid_t;
    fn isatty(__fd: libc::c_int) -> libc::c_int;
    fn tcsetpgrp(__fd: libc::c_int, __pgrp_id: __pid_t) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn realpath(
        __name: *const libc::c_char,
        __resolved: *mut libc::c_char,
    ) -> *mut libc::c_char;
    fn bzero(_: *mut libc::c_void, _: libc::c_ulong);
    fn index(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn cfsetospeed(__termios_p: *mut termios, __speed: speed_t) -> libc::c_int;
    fn cfsetispeed(__termios_p: *mut termios, __speed: speed_t) -> libc::c_int;
    fn tcgetattr(__fd: libc::c_int, __termios_p: *mut termios) -> libc::c_int;
    fn tcsetattr(
        __fd: libc::c_int,
        __optional_actions: libc::c_int,
        __termios_p: *const termios,
    ) -> libc::c_int;
    fn tcsendbreak(__fd: libc::c_int, __duration: libc::c_int) -> libc::c_int;
    fn tcflush(__fd: libc::c_int, __queue_selector: libc::c_int) -> libc::c_int;
    fn tcflow(__fd: libc::c_int, __action: libc::c_int) -> libc::c_int;
    fn Msg(_: libc::c_int, _: *const libc::c_char, _: ...);
    fn WriteString(_: *mut win, _: *mut libc::c_char, _: libc::c_int);
    fn secopen(_: *mut libc::c_char, _: libc::c_int, _: libc::c_int) -> libc::c_int;
    fn OpenPTY(_: *mut *mut libc::c_char) -> libc::c_int;
    fn evenq(_: *mut event);
    fn evdeq(_: *mut event);
    fn UserContext() -> libc::c_int;
    fn UserReturn(_: libc::c_int);
    fn UserStatus() -> libc::c_int;
    fn xsignal(
        _: libc::c_int,
        _: Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
    ) -> Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
    fn sleep1000(_: libc::c_int);
    static mut display: *mut display;
    static mut displays: *mut display;
    static mut iflag: libc::c_int;
    static mut console_window: *mut win;
}
pub type __int32_t = libc::c_int;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
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
pub struct baud_values {
    pub idx: libc::c_int,
    pub bps: libc::c_int,
    pub sym: libc::c_int,
}
#[inline]
unsafe extern "C" fn lstat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __lxstat(1 as libc::c_int, __path, __statbuf);
}
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
pub static mut separate_sids: libc::c_int = 1 as libc::c_int;
unsafe extern "C" fn SigAlrmDummy(mut sigsig: libc::c_int) {}
pub unsafe extern "C" fn OpenTTY(
    mut line: *mut libc::c_char,
    mut opt: *mut libc::c_char,
) -> libc::c_int {
    let mut f: libc::c_int = 0;
    let mut Mode: mode = mode {
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
    let mut sigalrm: Option::<unsafe extern "C" fn(libc::c_int) -> ()> = None;
    sigalrm = xsignal(
        14 as libc::c_int,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> ()>,
            Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
        >(
            Some(
                ::std::mem::transmute::<
                    unsafe extern "C" fn(libc::c_int) -> (),
                    unsafe extern "C" fn() -> (),
                >(SigAlrmDummy),
            ),
        ),
    );
    alarm(2 as libc::c_int as libc::c_uint);
    f = secopen(
        line,
        0o2 as libc::c_int | 0o4000 as libc::c_int | 0o400 as libc::c_int,
        0 as libc::c_int,
    );
    if f == -(1 as libc::c_int) {
        if *__errno_location() == 4 as libc::c_int {
            Msg(
                0 as libc::c_int,
                b"Cannot open line '%s' for R/W: open() blocked, aborted.\0" as *const u8
                    as *const libc::c_char,
                line,
            );
        } else {
            Msg(
                *__errno_location(),
                b"Cannot open line '%s' for R/W\0" as *const u8 as *const libc::c_char,
                line,
            );
        }
        alarm(0 as libc::c_int as libc::c_uint);
        xsignal(14 as libc::c_int, sigalrm);
        return -(1 as libc::c_int);
    }
    if isatty(f) == 0 {
        Msg(
            0 as libc::c_int,
            b"'%s' is not a tty\0" as *const u8 as *const libc::c_char,
            line,
        );
        alarm(0 as libc::c_int as libc::c_uint);
        xsignal(14 as libc::c_int, sigalrm);
        close(f);
        return -(1 as libc::c_int);
    }
    *__errno_location() = 0 as libc::c_int;
    if ioctl(f, 0x540c as libc::c_int as libc::c_ulong, 0 as *mut libc::c_char)
        < 0 as libc::c_int
    {
        Msg(
            *__errno_location(),
            b"%s: ioctl TIOCEXCL failed\0" as *const u8 as *const libc::c_char,
            line,
        );
    }
    InitTTY(&mut Mode, 1 as libc::c_int);
    SttyMode(&mut Mode, opt);
    SetTTY(f, &mut Mode);
    let mut mcs: libc::c_int = 0 as libc::c_int;
    ioctl(f, 0x5415 as libc::c_int as libc::c_ulong, &mut mcs as *mut libc::c_int);
    mcs |= 0x4 as libc::c_int;
    ioctl(f, 0x5418 as libc::c_int as libc::c_ulong, &mut mcs as *mut libc::c_int);
    brktty(f);
    alarm(0 as libc::c_int as libc::c_uint);
    xsignal(14 as libc::c_int, sigalrm);
    return f;
}
pub unsafe extern "C" fn InitTTY(mut m: *mut mode, mut ttyflag: libc::c_int) {
    bzero(
        m as *mut libc::c_char as *mut libc::c_void,
        ::std::mem::size_of::<mode>() as libc::c_ulong,
    );
    (*m).tio.c_iflag |= 0o2 as libc::c_int as libc::c_uint;
    (*m).tio.c_iflag |= 0o4 as libc::c_int as libc::c_uint;
    (*m).tio.c_iflag |= 0o2000 as libc::c_int as libc::c_uint;
    if ttyflag == 0 {
        (*m).tio.c_iflag |= 0o400 as libc::c_int as libc::c_uint;
        (*m).tio.c_oflag |= 0o4 as libc::c_int as libc::c_uint;
        (*m).tio.c_oflag |= 0o14000 as libc::c_int as libc::c_uint;
        (*m).tio.c_oflag |= 0o1 as libc::c_int as libc::c_uint;
    }
    cfsetospeed(&mut (*m).tio, 0o15 as libc::c_int as speed_t);
    cfsetispeed(&mut (*m).tio, 0o15 as libc::c_int as speed_t);
    (*m).tio.c_cflag |= 0o60 as libc::c_int as libc::c_uint;
    (*m).tio.c_cflag |= 0o200 as libc::c_int as libc::c_uint;
    (*m).tio.c_cflag |= 0o4000 as libc::c_int as libc::c_uint;
    (*m).tio.c_lflag |= 0o1000 as libc::c_int as libc::c_uint;
    (*m).tio.c_lflag |= 0o4000 as libc::c_int as libc::c_uint;
    if ttyflag == 0 {
        (*m).tio.c_lflag |= 0o1 as libc::c_int as libc::c_uint;
        (*m).tio.c_lflag |= 0o2 as libc::c_int as libc::c_uint;
        (*m).tio.c_lflag |= 0o10 as libc::c_int as libc::c_uint;
    }
    (*m).tio.c_lflag |= 0o20 as libc::c_int as libc::c_uint;
    (*m).tio.c_lflag |= 0o40 as libc::c_int as libc::c_uint;
    (*m).tio.c_lflag |= 0o100000 as libc::c_int as libc::c_uint;
    (*m)
        .tio
        .c_cc[0 as libc::c_int as usize] = ('C' as i32 & 0o37 as libc::c_int) as cc_t;
    (*m)
        .tio
        .c_cc[1 as libc::c_int as usize] = ('\\' as i32 & 0o37 as libc::c_int) as cc_t;
    (*m).tio.c_cc[2 as libc::c_int as usize] = 0x7f as libc::c_int as cc_t;
    (*m)
        .tio
        .c_cc[3 as libc::c_int as usize] = ('U' as i32 & 0o37 as libc::c_int) as cc_t;
    (*m)
        .tio
        .c_cc[4 as libc::c_int as usize] = ('D' as i32 & 0o37 as libc::c_int) as cc_t;
    (*m).tio.c_cc[11 as libc::c_int as usize] = '\0' as i32 as cc_t;
    (*m).tio.c_cc[16 as libc::c_int as usize] = '\0' as i32 as cc_t;
    (*m)
        .tio
        .c_cc[8 as libc::c_int as usize] = ('Q' as i32 & 0o37 as libc::c_int) as cc_t;
    (*m)
        .tio
        .c_cc[9 as libc::c_int as usize] = ('S' as i32 & 0o37 as libc::c_int) as cc_t;
    (*m)
        .tio
        .c_cc[10 as libc::c_int as usize] = ('Z' as i32 & 0o37 as libc::c_int) as cc_t;
    (*m)
        .tio
        .c_cc[12 as libc::c_int as usize] = ('R' as i32 & 0o37 as libc::c_int) as cc_t;
    (*m)
        .tio
        .c_cc[13 as libc::c_int as usize] = ('O' as i32 & 0o37 as libc::c_int) as cc_t;
    (*m)
        .tio
        .c_cc[14 as libc::c_int as usize] = ('W' as i32 & 0o37 as libc::c_int) as cc_t;
    (*m)
        .tio
        .c_cc[15 as libc::c_int as usize] = ('V' as i32 & 0o37 as libc::c_int) as cc_t;
    if ttyflag != 0 {
        (*m).tio.c_cc[6 as libc::c_int as usize] = 100 as libc::c_int as cc_t;
        (*m).tio.c_cc[5 as libc::c_int as usize] = 2 as libc::c_int as cc_t;
    }
}
pub unsafe extern "C" fn SetTTY(mut fd: libc::c_int, mut mp: *mut mode) {
    *__errno_location() = 0 as libc::c_int;
    tcsetattr(fd, 1 as libc::c_int, &mut (*mp).tio);
    if *__errno_location() != 0 {
        Msg(
            *__errno_location(),
            b"SetTTY (fd %d): ioctl failed\0" as *const u8 as *const libc::c_char,
            fd,
        );
    }
}
pub unsafe extern "C" fn GetTTY(mut fd: libc::c_int, mut mp: *mut mode) {
    *__errno_location() = 0 as libc::c_int;
    tcgetattr(fd, &mut (*mp).tio);
    if *__errno_location() != 0 {
        Msg(
            *__errno_location(),
            b"GetTTY (fd %d): ioctl failed\0" as *const u8 as *const libc::c_char,
            fd,
        );
    }
}
pub unsafe extern "C" fn SetMode(
    mut op: *mut mode,
    mut np: *mut mode,
    mut flow: libc::c_int,
    mut interrupt: libc::c_int,
) {
    *np = *op;
    (*np).tio.c_iflag &= !(0o400 as libc::c_int) as libc::c_uint;
    (*np).tio.c_iflag &= !(0o40 as libc::c_int) as libc::c_uint;
    (*np).tio.c_oflag &= !(0o4 as libc::c_int) as libc::c_uint;
    (*np).tio.c_lflag &= !(0o2 as libc::c_int | 0o10 as libc::c_int) as libc::c_uint;
    (*np).tio.c_lflag &= !(0o100000 as libc::c_int) as libc::c_uint;
    if interrupt != 0 {
        (*np).tio.c_lflag |= 0o1 as libc::c_int as libc::c_uint;
    } else {
        (*np).tio.c_lflag &= !(0o1 as libc::c_int) as libc::c_uint;
    }
    (*np).tio.c_cc[6 as libc::c_int as usize] = 1 as libc::c_int as cc_t;
    (*np).tio.c_cc[5 as libc::c_int as usize] = 0 as libc::c_int as cc_t;
    if interrupt == 0 || flow == 0 {
        (*np).tio.c_cc[0 as libc::c_int as usize] = '\0' as i32 as cc_t;
    }
    (*np).tio.c_cc[1 as libc::c_int as usize] = '\0' as i32 as cc_t;
    if flow == 0 as libc::c_int {
        (*np).tio.c_cc[8 as libc::c_int as usize] = '\0' as i32 as cc_t;
        (*np).tio.c_cc[9 as libc::c_int as usize] = '\0' as i32 as cc_t;
        (*np).tio.c_iflag &= !(0o2000 as libc::c_int) as libc::c_uint;
    }
    (*np).tio.c_cc[13 as libc::c_int as usize] = '\0' as i32 as cc_t;
    (*np).tio.c_cc[15 as libc::c_int as usize] = '\0' as i32 as cc_t;
    (*np).tio.c_cc[10 as libc::c_int as usize] = '\0' as i32 as cc_t;
    (*np).tio.c_cc[2 as libc::c_int as usize] = 0x7f as libc::c_int as cc_t;
    (*np).tio.c_cc[3 as libc::c_int as usize] = '\0' as i32 as cc_t;
    (*np).tio.c_cc[12 as libc::c_int as usize] = '\0' as i32 as cc_t;
    (*np).tio.c_cc[14 as libc::c_int as usize] = '\0' as i32 as cc_t;
}
pub unsafe extern "C" fn SetFlow(mut on: libc::c_int) {
    if (*display).d_flow == on {
        return;
    }
    if on != 0 {
        (*display)
            .d_NewMode
            .tio
            .c_cc[0 as libc::c_int
            as usize] = (if iflag != 0 {
            (*display).d_OldMode.tio.c_cc[0 as libc::c_int as usize] as libc::c_int
        } else {
            '\0' as i32
        }) as cc_t;
        (*display)
            .d_NewMode
            .tio
            .c_cc[8 as libc::c_int
            as usize] = (*display).d_OldMode.tio.c_cc[8 as libc::c_int as usize];
        (*display)
            .d_NewMode
            .tio
            .c_cc[9 as libc::c_int
            as usize] = (*display).d_OldMode.tio.c_cc[9 as libc::c_int as usize];
        (*display).d_NewMode.tio.c_iflag
            |= (*display).d_OldMode.tio.c_iflag & 0o2000 as libc::c_int as libc::c_uint;
    } else {
        (*display).d_NewMode.tio.c_cc[0 as libc::c_int as usize] = '\0' as i32 as cc_t;
        (*display).d_NewMode.tio.c_cc[8 as libc::c_int as usize] = '\0' as i32 as cc_t;
        (*display).d_NewMode.tio.c_cc[9 as libc::c_int as usize] = '\0' as i32 as cc_t;
        (*display).d_NewMode.tio.c_iflag &= !(0o2000 as libc::c_int) as libc::c_uint;
    }
    if on == 0 {
        tcflow((*display).d_userfd, 1 as libc::c_int);
    }
    tcsetattr((*display).d_userfd, 0 as libc::c_int, &mut (*display).d_NewMode.tio) != 0;
    (*display).d_flow = on;
}
pub unsafe extern "C" fn SttyMode(
    mut m: *mut mode,
    mut opt: *mut libc::c_char,
) -> libc::c_int {
    static mut sep: [libc::c_char; 6] = unsafe {
        *::std::mem::transmute::<&[u8; 6], &[libc::c_char; 6]>(b" \t:;,\0")
    };
    if opt.is_null() {
        return 0 as libc::c_int;
    }
    while *opt != 0 {
        while !(index(sep.as_ptr(), *opt as libc::c_int)).is_null() {
            opt = opt.offset(1);
            opt;
        }
        if *opt as libc::c_int >= '0' as i32 && *opt as libc::c_int <= '9' as i32 {
            if SetBaud(m, atoi(opt), atoi(opt)) != 0 {
                return -(1 as libc::c_int);
            }
        } else if strncmp(
            b"cs7\0" as *const u8 as *const libc::c_char,
            opt,
            3 as libc::c_int as libc::c_ulong,
        ) == 0
        {
            (*m).tio.c_cflag &= !(0o60 as libc::c_int) as libc::c_uint;
            (*m).tio.c_cflag |= 0o40 as libc::c_int as libc::c_uint;
        } else if strncmp(
            b"cs8\0" as *const u8 as *const libc::c_char,
            opt,
            3 as libc::c_int as libc::c_ulong,
        ) == 0
        {
            (*m).tio.c_cflag &= !(0o60 as libc::c_int) as libc::c_uint;
            (*m).tio.c_cflag |= 0o60 as libc::c_int as libc::c_uint;
        } else if strncmp(
            b"istrip\0" as *const u8 as *const libc::c_char,
            opt,
            6 as libc::c_int as libc::c_ulong,
        ) == 0
        {
            (*m).tio.c_iflag |= 0o40 as libc::c_int as libc::c_uint;
        } else if strncmp(
            b"-istrip\0" as *const u8 as *const libc::c_char,
            opt,
            7 as libc::c_int as libc::c_ulong,
        ) == 0
        {
            (*m).tio.c_iflag &= !(0o40 as libc::c_int) as libc::c_uint;
        } else if strncmp(
            b"ixon\0" as *const u8 as *const libc::c_char,
            opt,
            4 as libc::c_int as libc::c_ulong,
        ) == 0
        {
            (*m).tio.c_iflag |= 0o2000 as libc::c_int as libc::c_uint;
        } else if strncmp(
            b"-ixon\0" as *const u8 as *const libc::c_char,
            opt,
            5 as libc::c_int as libc::c_ulong,
        ) == 0
        {
            (*m).tio.c_iflag &= !(0o2000 as libc::c_int) as libc::c_uint;
        } else if strncmp(
            b"ixoff\0" as *const u8 as *const libc::c_char,
            opt,
            5 as libc::c_int as libc::c_ulong,
        ) == 0
        {
            (*m).tio.c_iflag |= 0o10000 as libc::c_int as libc::c_uint;
        } else if strncmp(
            b"-ixoff\0" as *const u8 as *const libc::c_char,
            opt,
            6 as libc::c_int as libc::c_ulong,
        ) == 0
        {
            (*m).tio.c_iflag &= !(0o10000 as libc::c_int) as libc::c_uint;
        } else if strncmp(
            b"crtscts\0" as *const u8 as *const libc::c_char,
            opt,
            7 as libc::c_int as libc::c_ulong,
        ) == 0
        {
            (*m).tio.c_cflag |= 0o20000000000 as libc::c_uint;
        } else if strncmp(
            b"-crtscts\0" as *const u8 as *const libc::c_char,
            opt,
            8 as libc::c_int as libc::c_ulong,
        ) == 0
        {
            (*m).tio.c_cflag &= !(0o20000000000 as libc::c_uint);
        } else {
            return -(1 as libc::c_int)
        }
        while *opt as libc::c_int != 0
            && (index(sep.as_ptr(), *opt as libc::c_int)).is_null()
        {
            opt = opt.offset(1);
            opt;
        }
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn brktty(mut fd: libc::c_int) {
    if separate_sids != 0 {
        setsid();
    }
}
pub unsafe extern "C" fn fgtty(mut fd: libc::c_int) -> libc::c_int {
    let mut mypid: libc::c_int = 0;
    mypid = getpid();
    if separate_sids != 0 {
        if tcsetpgrp(fd, mypid) != 0 {
            return -(1 as libc::c_int);
        }
    }
    return 0 as libc::c_int;
}
pub static mut breaktype: libc::c_int = 2 as libc::c_int;
unsafe extern "C" fn DoSendBreak(
    mut fd: libc::c_int,
    mut n: libc::c_int,
    mut type_0: libc::c_int,
) {
    match type_0 {
        2 => {
            let mut i: libc::c_int = 0;
            if n == 0 {
                n += 1;
                n;
            }
            i = 0 as libc::c_int;
            while i < n {
                if tcsendbreak(fd, 0 as libc::c_int) < 0 as libc::c_int {
                    Msg(
                        *__errno_location(),
                        b"cannot send BREAK (tcsendbreak SVR4)\0" as *const u8
                            as *const libc::c_char,
                    );
                    return;
                }
                i += 1;
                i;
            }
        }
        1 => {
            if n == 0 {
                n += 1;
                n;
            }
            let mut i_0: libc::c_int = 0;
            i_0 = 0 as libc::c_int;
            while i_0 < n {
                if ioctl(
                    fd,
                    0x5409 as libc::c_int as libc::c_ulong,
                    0 as *mut libc::c_char,
                ) < 0 as libc::c_int
                {
                    Msg(
                        *__errno_location(),
                        b"Cannot send BREAK (TCSBRK)\0" as *const u8
                            as *const libc::c_char,
                    );
                    return;
                }
                i_0 += 1;
                i_0;
            }
        }
        0 => {
            if ioctl(fd, 0x5427 as libc::c_int as libc::c_ulong, 0 as *mut libc::c_char)
                < 0 as libc::c_int
            {
                Msg(
                    *__errno_location(),
                    b"Can't send BREAK (TIOCSBRK)\0" as *const u8 as *const libc::c_char,
                );
                return;
            }
            sleep1000(if n != 0 { n * 250 as libc::c_int } else { 250 as libc::c_int });
            if ioctl(fd, 0x5428 as libc::c_int as libc::c_ulong, 0 as *mut libc::c_char)
                < 0 as libc::c_int
            {
                Msg(
                    *__errno_location(),
                    b"BREAK stuck!!! -- HELP! (TIOCCBRK)\0" as *const u8
                        as *const libc::c_char,
                );
                return;
            }
        }
        _ => {
            Msg(
                0 as libc::c_int,
                b"Internal SendBreak error: method %d unknown\0" as *const u8
                    as *const libc::c_char,
                type_0,
            );
        }
    };
}
pub unsafe extern "C" fn SendBreak(
    mut wp: *mut win,
    mut n: libc::c_int,
    mut closeopen: libc::c_int,
) {
    let mut sigalrm: Option::<unsafe extern "C" fn(libc::c_int) -> ()> = None;
    if (*wp).w_type != 1 as libc::c_int {
        return;
    }
    tcflush((*wp).w_ptyfd, 2 as libc::c_int);
    if closeopen != 0 {
        close((*wp).w_ptyfd);
        sleep1000(if n != 0 { n * 250 as libc::c_int } else { 250 as libc::c_int });
        (*wp)
            .w_ptyfd = OpenTTY(
            ((*wp).w_tty).as_mut_ptr(),
            (*wp).w_cmdargs[1 as libc::c_int as usize],
        );
        if (*wp).w_ptyfd < 1 as libc::c_int {
            Msg(
                0 as libc::c_int,
                b"Ouch, cannot reopen line %s, please try harder\0" as *const u8
                    as *const libc::c_char,
                ((*wp).w_tty).as_mut_ptr(),
            );
            return;
        }
        fcntl((*wp).w_ptyfd, 4 as libc::c_int, 0o4000 as libc::c_int);
    } else {
        sigalrm = xsignal(
            14 as libc::c_int,
            ::std::mem::transmute::<
                Option::<unsafe extern "C" fn() -> ()>,
                Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
            >(
                Some(
                    ::std::mem::transmute::<
                        unsafe extern "C" fn(libc::c_int) -> (),
                        unsafe extern "C" fn() -> (),
                    >(SigAlrmDummy),
                ),
            ),
        );
        alarm(15 as libc::c_int as libc::c_uint);
        DoSendBreak((*wp).w_ptyfd, n, breaktype);
        alarm(0 as libc::c_int as libc::c_uint);
        xsignal(14 as libc::c_int, sigalrm);
    };
}
static mut consredir_ev: event = event {
    next: 0 as *const event as *mut event,
    handler: None,
    data: 0 as *const libc::c_char as *mut libc::c_char,
    fd: 0,
    type_0: 0,
    pri: 0,
    timeout: timeval { tv_sec: 0, tv_usec: 0 },
    queued: 0,
    active: 0,
    condpos: 0 as *const libc::c_int as *mut libc::c_int,
    condneg: 0 as *const libc::c_int as *mut libc::c_int,
};
static mut consredirfd: [libc::c_int; 2] = [-(1 as libc::c_int), -(1 as libc::c_int)];
unsafe extern "C" fn consredir_readev_fn(
    mut ev: *mut event,
    mut data: *mut libc::c_char,
) {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut n: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut buf: [libc::c_char; 256] = [0; 256];
    let mut l: libc::c_int = 0;
    if console_window.is_null()
        || {
            l = read(
                consredirfd[0 as libc::c_int as usize],
                buf.as_mut_ptr() as *mut libc::c_void,
                ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
            ) as libc::c_int;
            l <= 0 as libc::c_int
        }
    {
        close(consredirfd[0 as libc::c_int as usize]);
        close(consredirfd[1 as libc::c_int as usize]);
        consredirfd[1 as libc::c_int as usize] = -(1 as libc::c_int);
        consredirfd[0 as libc::c_int as usize] = consredirfd[1 as libc::c_int as usize];
        evdeq(ev);
        return;
    }
    n = buf.as_mut_ptr();
    p = n;
    while l > 0 as libc::c_int {
        if *n as libc::c_int == '\n' as i32 {
            if n > p {
                WriteString(
                    console_window,
                    p,
                    n.offset_from(p) as libc::c_long as libc::c_int,
                );
            }
            WriteString(
                console_window,
                b"\r\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                2 as libc::c_int,
            );
            p = n.offset(1 as libc::c_int as isize);
        }
        n = n.offset(1);
        n;
        l -= 1;
        l;
    }
    if n > p {
        WriteString(console_window, p, n.offset_from(p) as libc::c_long as libc::c_int);
    }
}
pub unsafe extern "C" fn TtyGrabConsole(
    mut fd: libc::c_int,
    mut on: libc::c_int,
    mut rc_name: *mut libc::c_char,
) -> libc::c_int {
    let mut d: *mut display = 0 as *mut display;
    let mut new1: mode = mode {
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
    let mut new2: mode = mode {
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
    let mut slave: *mut libc::c_char = 0 as *mut libc::c_char;
    if on > 0 as libc::c_int {
        if displays.is_null() {
            Msg(
                0 as libc::c_int,
                b"I need a display\0" as *const u8 as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
        d = displays;
        while !d.is_null() {
            if strcmp(
                ((*d).d_usertty).as_mut_ptr(),
                b"/dev/console\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                break;
            }
            d = (*d).d_next;
        }
        if !d.is_null() {
            Msg(
                0 as libc::c_int,
                b"too dangerous - screen is running on /dev/console\0" as *const u8
                    as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
    }
    if consredirfd[0 as libc::c_int as usize] >= 0 as libc::c_int {
        evdeq(&mut consredir_ev);
        close(consredirfd[0 as libc::c_int as usize]);
        close(consredirfd[1 as libc::c_int as usize]);
        consredirfd[1 as libc::c_int as usize] = -(1 as libc::c_int);
        consredirfd[0 as libc::c_int as usize] = consredirfd[1 as libc::c_int as usize];
    }
    if on <= 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    consredirfd[0 as libc::c_int as usize] = OpenPTY(&mut slave);
    if consredirfd[0 as libc::c_int as usize] < 0 as libc::c_int {
        Msg(
            *__errno_location(),
            b"%s: could not open detach pty master\0" as *const u8
                as *const libc::c_char,
            rc_name,
        );
        return -(1 as libc::c_int);
    }
    consredirfd[1 as libc::c_int
        as usize] = open(slave, 0o2 as libc::c_int | 0o400 as libc::c_int);
    if consredirfd[1 as libc::c_int as usize] < 0 as libc::c_int {
        Msg(
            *__errno_location(),
            b"%s: could not open detach pty slave\0" as *const u8 as *const libc::c_char,
            rc_name,
        );
        close(consredirfd[0 as libc::c_int as usize]);
        return -(1 as libc::c_int);
    }
    InitTTY(&mut new1, 0 as libc::c_int);
    SetMode(&mut new1, &mut new2, 0 as libc::c_int, 0 as libc::c_int);
    SetTTY(consredirfd[1 as libc::c_int as usize], &mut new2);
    if UserContext() == 1 as libc::c_int {
        UserReturn(
            ioctl(
                consredirfd[1 as libc::c_int as usize],
                0x541d as libc::c_int as libc::c_ulong,
                &mut on as *mut libc::c_int as *mut libc::c_char,
            ),
        );
    }
    if UserStatus() != 0 {
        Msg(
            *__errno_location(),
            b"%s: ioctl TIOCCONS failed\0" as *const u8 as *const libc::c_char,
            rc_name,
        );
        close(consredirfd[0 as libc::c_int as usize]);
        close(consredirfd[1 as libc::c_int as usize]);
        return -(1 as libc::c_int);
    }
    consredir_ev.fd = consredirfd[0 as libc::c_int as usize];
    consredir_ev.type_0 = 1 as libc::c_int;
    consredir_ev
        .handler = Some(
        consredir_readev_fn as unsafe extern "C" fn(*mut event, *mut libc::c_char) -> (),
    );
    evenq(&mut consredir_ev);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn TtyGetModemStatus(
    mut fd: libc::c_int,
    mut buf: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut p: *mut libc::c_char = buf;
    let mut softcar: libc::c_uint = 0;
    let mut mflags: libc::c_uint = 0;
    let mut mtio: mode = mode {
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
    let mut rtscts: libc::c_int = 0;
    let mut clocal: libc::c_int = 0;
    GetTTY(fd, &mut mtio);
    clocal = 0 as libc::c_int;
    if mtio.tio.c_cflag & 0o4000 as libc::c_int as libc::c_uint != 0 {
        clocal = 1 as libc::c_int;
        let fresh0 = p;
        p = p.offset(1);
        *fresh0 = '{' as i32 as libc::c_char;
    }
    if mtio.tio.c_cflag & 0o20000000000 as libc::c_uint == 0 {
        rtscts = 0 as libc::c_int;
    } else {
        rtscts = 1 as libc::c_int;
    }
    if ioctl(
        fd,
        0x5419 as libc::c_int as libc::c_ulong,
        &mut softcar as *mut libc::c_uint as *mut libc::c_char,
    ) < 0 as libc::c_int
    {
        softcar = 0 as libc::c_int as libc::c_uint;
    }
    if ioctl(
        fd,
        0x5415 as libc::c_int as libc::c_ulong,
        &mut mflags as *mut libc::c_uint as *mut libc::c_char,
    ) < 0 as libc::c_int
    {
        sprintf(
            p,
            b"NO-TTY? %s\0" as *const u8 as *const libc::c_char,
            if softcar != 0 {
                b"(CD)\0" as *const u8 as *const libc::c_char
            } else {
                b"CD\0" as *const u8 as *const libc::c_char
            },
        );
        p = p.offset(strlen(p) as isize);
    } else {
        let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
        s = b"!RTS \0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        if mflags & 0x4 as libc::c_int as libc::c_uint != 0 {
            s = s.offset(1);
            s;
        }
        while *s != 0 {
            let fresh1 = s;
            s = s.offset(1);
            let fresh2 = p;
            p = p.offset(1);
            *fresh2 = *fresh1;
        }
        s = b"!CTS \0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        if rtscts == 0 {
            let fresh3 = p;
            p = p.offset(1);
            *fresh3 = '(' as i32 as libc::c_char;
            s = b"!CTS) \0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
        if mflags & 0x20 as libc::c_int as libc::c_uint != 0 {
            s = s.offset(1);
            s;
        }
        while *s != 0 {
            let fresh4 = s;
            s = s.offset(1);
            let fresh5 = p;
            p = p.offset(1);
            *fresh5 = *fresh4;
        }
        s = b"!DTR \0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        if mflags & 0x2 as libc::c_int as libc::c_uint != 0 {
            s = s.offset(1);
            s;
        }
        while *s != 0 {
            let fresh6 = s;
            s = s.offset(1);
            let fresh7 = p;
            p = p.offset(1);
            *fresh7 = *fresh6;
        }
        s = b"!DSR \0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        if mflags & 0x100 as libc::c_int as libc::c_uint != 0 {
            s = s.offset(1);
            s;
        }
        while *s != 0 {
            let fresh8 = s;
            s = s.offset(1);
            let fresh9 = p;
            p = p.offset(1);
            *fresh9 = *fresh8;
        }
        s = b"!CD \0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        if softcar != 0 {
            let fresh10 = p;
            p = p.offset(1);
            *fresh10 = '(' as i32 as libc::c_char;
            s = b"!CD) \0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
        if mflags & 0x40 as libc::c_int as libc::c_uint != 0 {
            s = s.offset(1);
            s;
        }
        while *s != 0 {
            let fresh11 = s;
            s = s.offset(1);
            let fresh12 = p;
            p = p.offset(1);
            *fresh12 = *fresh11;
        }
        if mflags & 0x80 as libc::c_int as libc::c_uint != 0 {
            s = b"RI \0" as *const u8 as *const libc::c_char as *mut libc::c_char;
            while *s != 0 {
                let fresh13 = s;
                s = s.offset(1);
                let fresh14 = p;
                p = p.offset(1);
                *fresh14 = *fresh13;
            }
        }
        if p > buf
            && *p.offset(-(1 as libc::c_int) as isize) as libc::c_int == ' ' as i32
        {
            p = p.offset(-1);
            p;
        }
        *p = '\0' as i32 as libc::c_char;
    }
    if clocal != 0 {
        let fresh15 = p;
        p = p.offset(1);
        *fresh15 = '}' as i32 as libc::c_char;
    }
    *p = '\0' as i32 as libc::c_char;
    return buf;
}
static mut btable: [baud_values; 34] = [
    {
        let mut init = baud_values {
            idx: 33 as libc::c_int,
            bps: 4000000 as libc::c_int,
            sym: 0o10017 as libc::c_int,
        };
        init
    },
    {
        let mut init = baud_values {
            idx: 32 as libc::c_int,
            bps: 3500000 as libc::c_int,
            sym: 0o10016 as libc::c_int,
        };
        init
    },
    {
        let mut init = baud_values {
            idx: 31 as libc::c_int,
            bps: 3000000 as libc::c_int,
            sym: 0o10015 as libc::c_int,
        };
        init
    },
    {
        let mut init = baud_values {
            idx: 30 as libc::c_int,
            bps: 2500000 as libc::c_int,
            sym: 0o10014 as libc::c_int,
        };
        init
    },
    {
        let mut init = baud_values {
            idx: 29 as libc::c_int,
            bps: 2000000 as libc::c_int,
            sym: 0o10013 as libc::c_int,
        };
        init
    },
    {
        let mut init = baud_values {
            idx: 28 as libc::c_int,
            bps: 1500000 as libc::c_int,
            sym: 0o10012 as libc::c_int,
        };
        init
    },
    {
        let mut init = baud_values {
            idx: 27 as libc::c_int,
            bps: 1152000 as libc::c_int,
            sym: 0o10011 as libc::c_int,
        };
        init
    },
    {
        let mut init = baud_values {
            idx: 26 as libc::c_int,
            bps: 1000000 as libc::c_int,
            sym: 0o10010 as libc::c_int,
        };
        init
    },
    {
        let mut init = baud_values {
            idx: 25 as libc::c_int,
            bps: 921600 as libc::c_int,
            sym: 0o10007 as libc::c_int,
        };
        init
    },
    {
        let mut init = baud_values {
            idx: 24 as libc::c_int,
            bps: 576000 as libc::c_int,
            sym: 0o10006 as libc::c_int,
        };
        init
    },
    {
        let mut init = baud_values {
            idx: 23 as libc::c_int,
            bps: 500000 as libc::c_int,
            sym: 0o10005 as libc::c_int,
        };
        init
    },
    {
        let mut init = baud_values {
            idx: 22 as libc::c_int,
            bps: 460800 as libc::c_int,
            sym: 0o10004 as libc::c_int,
        };
        init
    },
    {
        let mut init = baud_values {
            idx: 21 as libc::c_int,
            bps: 230400 as libc::c_int,
            sym: 0o10003 as libc::c_int,
        };
        init
    },
    {
        let mut init = baud_values {
            idx: 20 as libc::c_int,
            bps: 115200 as libc::c_int,
            sym: 0o10002 as libc::c_int,
        };
        init
    },
    {
        let mut init = baud_values {
            idx: 19 as libc::c_int,
            bps: 57600 as libc::c_int,
            sym: 0o10001 as libc::c_int,
        };
        init
    },
    {
        let mut init = baud_values {
            idx: 18 as libc::c_int,
            bps: 38400 as libc::c_int,
            sym: 0o17 as libc::c_int,
        };
        init
    },
    {
        let mut init = baud_values {
            idx: 18 as libc::c_int,
            bps: 38400 as libc::c_int,
            sym: 0o17 as libc::c_int,
        };
        init
    },
    {
        let mut init = baud_values {
            idx: 17 as libc::c_int,
            bps: 19200 as libc::c_int,
            sym: 0o16 as libc::c_int,
        };
        init
    },
    {
        let mut init = baud_values {
            idx: 17 as libc::c_int,
            bps: 19200 as libc::c_int,
            sym: 0o16 as libc::c_int,
        };
        init
    },
    {
        let mut init = baud_values {
            idx: 16 as libc::c_int,
            bps: 9600 as libc::c_int,
            sym: 0o15 as libc::c_int,
        };
        init
    },
    {
        let mut init = baud_values {
            idx: 14 as libc::c_int,
            bps: 4800 as libc::c_int,
            sym: 0o14 as libc::c_int,
        };
        init
    },
    {
        let mut init = baud_values {
            idx: 12 as libc::c_int,
            bps: 2400 as libc::c_int,
            sym: 0o13 as libc::c_int,
        };
        init
    },
    {
        let mut init = baud_values {
            idx: 11 as libc::c_int,
            bps: 1800 as libc::c_int,
            sym: 0o12 as libc::c_int,
        };
        init
    },
    {
        let mut init = baud_values {
            idx: 10 as libc::c_int,
            bps: 1200 as libc::c_int,
            sym: 0o11 as libc::c_int,
        };
        init
    },
    {
        let mut init = baud_values {
            idx: 8 as libc::c_int,
            bps: 600 as libc::c_int,
            sym: 0o10 as libc::c_int,
        };
        init
    },
    {
        let mut init = baud_values {
            idx: 7 as libc::c_int,
            bps: 300 as libc::c_int,
            sym: 0o7 as libc::c_int,
        };
        init
    },
    {
        let mut init = baud_values {
            idx: 6 as libc::c_int,
            bps: 200 as libc::c_int,
            sym: 0o6 as libc::c_int,
        };
        init
    },
    {
        let mut init = baud_values {
            idx: 5 as libc::c_int,
            bps: 150 as libc::c_int,
            sym: 0o5 as libc::c_int,
        };
        init
    },
    {
        let mut init = baud_values {
            idx: 4 as libc::c_int,
            bps: 134 as libc::c_int,
            sym: 0o4 as libc::c_int,
        };
        init
    },
    {
        let mut init = baud_values {
            idx: 3 as libc::c_int,
            bps: 110 as libc::c_int,
            sym: 0o3 as libc::c_int,
        };
        init
    },
    {
        let mut init = baud_values {
            idx: 2 as libc::c_int,
            bps: 75 as libc::c_int,
            sym: 0o2 as libc::c_int,
        };
        init
    },
    {
        let mut init = baud_values {
            idx: 1 as libc::c_int,
            bps: 50 as libc::c_int,
            sym: 0o1 as libc::c_int,
        };
        init
    },
    {
        let mut init = baud_values {
            idx: 0 as libc::c_int,
            bps: 0 as libc::c_int,
            sym: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = baud_values {
            idx: -(1 as libc::c_int),
            bps: -(1 as libc::c_int),
            sym: -(1 as libc::c_int),
        };
        init
    },
];
pub unsafe extern "C" fn lookup_baud(mut baud: libc::c_int) -> *mut baud_values {
    let mut p: *mut baud_values = 0 as *mut baud_values;
    p = btable.as_mut_ptr();
    while (*p).idx >= 0 as libc::c_int {
        if baud == (*p).bps || baud == (*p).sym {
            return p;
        }
        p = p.offset(1);
        p;
    }
    return 0 as *mut baud_values;
}
pub unsafe extern "C" fn SetBaud(
    mut m: *mut mode,
    mut ibaud: libc::c_int,
    mut obaud: libc::c_int,
) -> libc::c_int {
    let mut ip: *mut baud_values = 0 as *mut baud_values;
    let mut op: *mut baud_values = 0 as *mut baud_values;
    ip = lookup_baud(ibaud);
    if ip.is_null() && ibaud != -(1 as libc::c_int)
        || {
            op = lookup_baud(obaud);
            op.is_null() && obaud != -(1 as libc::c_int)
        }
    {
        return -(1 as libc::c_int);
    }
    if !ip.is_null() {
        cfsetispeed(&mut (*m).tio, (*ip).sym as speed_t);
    }
    if !op.is_null() {
        cfsetospeed(&mut (*m).tio, (*op).sym as speed_t);
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn CheckTtyname(mut tty: *mut libc::c_char) -> libc::c_int {
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
    let mut realbuf: [libc::c_char; 4096] = [0; 4096];
    let mut real: *const libc::c_char = 0 as *const libc::c_char;
    let mut rc: libc::c_int = 0;
    real = realpath(tty, realbuf.as_mut_ptr());
    if real.is_null() {
        return -(1 as libc::c_int);
    }
    realbuf[(::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        as usize] = '\0' as i32 as libc::c_char;
    if lstat(real, &mut st) != 0
        || !(st.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o20000 as libc::c_int as libc::c_uint)
        || st.st_nlink > 1 as libc::c_int as libc::c_ulong
            && strncmp(
                real,
                b"/dev\0" as *const u8 as *const libc::c_char,
                4 as libc::c_int as libc::c_ulong,
            ) != 0
    {
        rc = -(1 as libc::c_int);
    } else {
        rc = 0 as libc::c_int;
    }
    return rc;
}
