use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type logfile;
    fn chmod(__file: *const libc::c_char, __mode: __mode_t) -> libc::c_int;
    fn __xstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn bcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn bcopy(__src: *const libc::c_void, __dest: *mut libc::c_void, __n: size_t);
    fn bzero(_: *mut libc::c_void, _: libc::c_ulong);
    fn time(__timer: *mut time_t) -> time_t;
    fn setutent();
    fn getutline(__line: *const utmp) -> *mut utmp;
    fn pututline(__utmp_ptr: *const utmp) -> *mut utmp;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn Msg(_: libc::c_int, _: *const libc::c_char, _: ...);
    fn WindowChanged(_: *mut win, _: libc::c_int);
    fn CheckTtyname(_: *mut libc::c_char) -> libc::c_int;
    fn GetPtsPathOrSymlink(_: libc::c_int) -> *mut libc::c_char;
    fn stripdev(_: *mut libc::c_char) -> *mut libc::c_char;
    static mut display: *mut display;
    static mut fore: *mut win;
    static mut LoginName: *mut libc::c_char;
    static mut real_uid: libc::c_int;
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
pub type __syscall_slong_t = libc::c_long;
pub type pid_t = __pid_t;
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
unsafe extern "C" fn stat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __xstat(1 as libc::c_int, __path, __statbuf);
}
static mut utmpok: libc::c_int = 0;
static mut UtmpName: [libc::c_char; 14] = unsafe {
    *::std::mem::transmute::<&[u8; 14], &mut [libc::c_char; 14]>(b"/var/run/utmp\0")
};
static mut utmpfd: libc::c_int = -(1 as libc::c_int);
pub unsafe extern "C" fn SlotToggle(mut how: libc::c_int) {
    if (*fore).w_type != 0 as libc::c_int {
        Msg(
            0 as libc::c_int,
            b"Can only work with normal windows.\n\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if how != 0 {
        if (*fore).w_slot == -(1 as libc::c_int) as slot_t || ((*fore).w_slot).is_null()
        {
            if SetUtmp(fore) == 0 as libc::c_int {
                Msg(
                    0 as libc::c_int,
                    b"This window is now logged in.\0" as *const u8
                        as *const libc::c_char,
                );
            } else {
                Msg(
                    0 as libc::c_int,
                    b"This window should now be logged in.\0" as *const u8
                        as *const libc::c_char,
                );
            }
            WindowChanged(fore, 'f' as i32);
        } else {
            Msg(
                0 as libc::c_int,
                b"This window is already logged in.\0" as *const u8
                    as *const libc::c_char,
            );
        }
    } else if (*fore).w_slot == -(1 as libc::c_int) as slot_t {
        Msg(
            0 as libc::c_int,
            b"This window is already logged out\n\0" as *const u8 as *const libc::c_char,
        );
    } else if ((*fore).w_slot).is_null() {
        Msg(
            0 as libc::c_int,
            b"This window is not logged in.\0" as *const u8 as *const libc::c_char,
        );
        (*fore).w_slot = -(1 as libc::c_int) as slot_t;
    } else {
        RemoveUtmp(fore);
        if (*fore).w_slot != -(1 as libc::c_int) as slot_t {
            Msg(
                0 as libc::c_int,
                b"What? Cannot remove Utmp slot?\0" as *const u8 as *const libc::c_char,
            );
        } else {
            Msg(
                0 as libc::c_int,
                b"This window is no longer logged in.\0" as *const u8
                    as *const libc::c_char,
            );
        }
        WindowChanged(fore, 'f' as i32);
    };
}
pub unsafe extern "C" fn InitUtmp() {
    utmpfd = open(UtmpName.as_mut_ptr(), 0o2 as libc::c_int);
    if utmpfd == -(1 as libc::c_int) {
        if *__errno_location() != 13 as libc::c_int {
            Msg(
                *__errno_location(),
                b"%s\0" as *const u8 as *const libc::c_char,
                UtmpName.as_mut_ptr(),
            );
        }
        utmpok = 0 as libc::c_int;
        return;
    }
    close(utmpfd);
    utmpfd = -(1 as libc::c_int);
    utmpok = 1 as libc::c_int;
}
pub unsafe extern "C" fn RemoveLoginSlot() {
    let mut u: utmp = utmp {
        ut_type: 0,
        ut_pid: 0,
        ut_line: [0; 32],
        ut_id: [0; 4],
        ut_user: [0; 32],
        ut_host: [0; 256],
        ut_exit: exit_status {
            e_termination: 0,
            e_exit: 0,
        },
        ut_session: 0,
        ut_tv: C2RustUnnamed {
            tv_sec: 0,
            tv_usec: 0,
        },
        ut_addr_v6: [0; 4],
        __glibc_reserved: [0; 20],
    };
    let mut uu: *mut utmp = 0 as *mut utmp;
    (*display).d_loginslot = TtyNameSlot(((*display).d_usertty).as_mut_ptr());
    if ((*display).d_loginslot).is_null()
        || (*display).d_loginslot == -(1 as libc::c_int) as slot_t
    {
        return;
    }
    if utmpok == 0 {
        (*display).d_loginslot = 0 as slot_t;
    } else {
        uu = getutslot((*display).d_loginslot);
        if uu.is_null() {
            (*display).d_loginslot = 0 as slot_t;
        } else {
            (*display).d_utmp_logintty = *uu;
            u = *uu;
            makedead(&mut u);
            if pututslot(
                (*display).d_loginslot,
                &mut u,
                0 as *mut libc::c_char,
                0 as *mut win,
            ) == 0 as libc::c_int
            {
                (*display).d_loginslot = 0 as slot_t;
            }
        }
    }
    if ((*display).d_loginslot).is_null() {
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
        let mut tty: *mut libc::c_char = 0 as *mut libc::c_char;
        (*display).d_loginttymode = 0 as libc::c_int;
        tty = GetPtsPathOrSymlink((*display).d_userfd);
        if !tty.is_null() && stat(tty, &mut stb) == 0 as libc::c_int
            && stb.st_uid as libc::c_int == real_uid && CheckTtyname(tty) == 0
            && stb.st_mode as libc::c_int & 0o777 as libc::c_int != 0o666 as libc::c_int
        {
            (*display)
                .d_loginttymode = stb.st_mode as libc::c_int & 0o777 as libc::c_int;
            chmod(
                ((*display).d_usertty).as_mut_ptr(),
                stb.st_mode & 0o600 as libc::c_int as libc::c_uint,
            );
        }
    }
}
pub unsafe extern "C" fn RestoreLoginSlot() {
    let mut tty: *mut libc::c_char = 0 as *mut libc::c_char;
    if utmpok != 0 && !((*display).d_loginslot).is_null()
        && (*display).d_loginslot != -(1 as libc::c_int) as slot_t
    {
        if pututslot(
            (*display).d_loginslot,
            &mut (*display).d_utmp_logintty,
            ((*display).d_utmp_logintty.ut_host).as_mut_ptr(),
            0 as *mut win,
        ) == 0 as libc::c_int
        {
            Msg(
                *__errno_location(),
                b"Could not write %s\0" as *const u8 as *const libc::c_char,
                UtmpName.as_mut_ptr(),
            );
        }
    }
    (*display).d_loginslot = 0 as slot_t;
    if (*display).d_loginttymode != 0
        && {
            tty = GetPtsPathOrSymlink((*display).d_userfd);
            !tty.is_null()
        } && CheckTtyname(tty) == 0
    {
        chmod(tty, (*display).d_loginttymode as __mode_t);
    }
}
pub unsafe extern "C" fn SetUtmp(mut wi: *mut win) -> libc::c_int {
    let mut slot: slot_t = 0 as *mut libc::c_char;
    let mut u: utmp = utmp {
        ut_type: 0,
        ut_pid: 0,
        ut_line: [0; 32],
        ut_id: [0; 4],
        ut_user: [0; 32],
        ut_host: [0; 256],
        ut_exit: exit_status {
            e_termination: 0,
            e_exit: 0,
        },
        ut_session: 0,
        ut_tv: C2RustUnnamed {
            tv_sec: 0,
            tv_usec: 0,
        },
        ut_addr_v6: [0; 4],
        __glibc_reserved: [0; 20],
    };
    let mut saved_ut: libc::c_int = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut host: [libc::c_char; 271] = [0; 271];
    (*wi).w_slot = 0 as slot_t;
    if utmpok == 0 || (*wi).w_type != 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    slot = TtyNameSlot(((*wi).w_tty).as_mut_ptr());
    if slot.is_null() {
        return -(1 as libc::c_int);
    }
    bzero(
        &mut u as *mut utmp as *mut libc::c_char as *mut libc::c_void,
        ::std::mem::size_of::<utmp>() as libc::c_ulong,
    );
    saved_ut = bcmp(
        &mut (*wi).w_savut as *mut utmp as *mut libc::c_char as *const libc::c_void,
        &mut u as *mut utmp as *mut libc::c_char as *const libc::c_void,
        ::std::mem::size_of::<utmp>() as libc::c_ulong,
    );
    if saved_ut != 0 {
        bcopy(
            &mut (*wi).w_savut as *mut utmp as *mut libc::c_char as *const libc::c_void,
            &mut u as *mut utmp as *mut libc::c_char as *mut libc::c_void,
            ::std::mem::size_of::<utmp>() as libc::c_ulong,
        );
    }
    if saved_ut == 0 {
        makeuser(&mut u, stripdev(((*wi).w_tty).as_mut_ptr()), LoginName, (*wi).w_pid);
    }
    host[(::std::mem::size_of::<[libc::c_char; 271]>() as libc::c_ulong)
        .wrapping_sub(15 as libc::c_int as libc::c_ulong)
        as usize] = '\0' as i32 as libc::c_char;
    if !display.is_null() {
        strncpy(
            host.as_mut_ptr(),
            ((*display).d_utmp_logintty.ut_host).as_mut_ptr(),
            (::std::mem::size_of::<[libc::c_char; 271]>() as libc::c_ulong)
                .wrapping_sub(15 as libc::c_int as libc::c_ulong),
        );
        if !((*display).d_loginslot).is_null()
            && (*display).d_loginslot != -(1 as libc::c_int) as slot_t
            && host[0 as libc::c_int as usize] as libc::c_int != '\0' as i32
        {
            p = host.as_mut_ptr();
            while *p != 0 {
                if ((*p as libc::c_int) < '0' as i32 || *p as libc::c_int > '9' as i32)
                    && *p as libc::c_int != '.' as i32
                {
                    break;
                }
                p = p.offset(1);
                p;
            }
            if *p != 0 {
                p = host.as_mut_ptr();
                while *p != 0 {
                    if *p as libc::c_int == '.' as i32
                        || *p as libc::c_int == ':' as i32 && p != host.as_mut_ptr()
                    {
                        *p = '\0' as i32 as libc::c_char;
                        break;
                    } else {
                        p = p.offset(1);
                        p;
                    }
                }
            }
        } else {
            strncpy(
                host.as_mut_ptr().offset(1 as libc::c_int as isize),
                stripdev(((*display).d_usertty).as_mut_ptr()),
                (::std::mem::size_of::<[libc::c_char; 271]>() as libc::c_ulong)
                    .wrapping_sub(15 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            );
            host[0 as libc::c_int as usize] = ':' as i32 as libc::c_char;
        }
    } else {
        strncpy(
            host.as_mut_ptr(),
            b"local\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 271]>() as libc::c_ulong)
                .wrapping_sub(15 as libc::c_int as libc::c_ulong),
        );
    }
    sprintf(
        host.as_mut_ptr().offset(strlen(host.as_mut_ptr()) as isize),
        b":S.%d\0" as *const u8 as *const libc::c_char,
        (*wi).w_number,
    );
    strncpy(
        (u.ut_host).as_mut_ptr(),
        host.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
    );
    if pututslot(slot, &mut u, host.as_mut_ptr(), wi) == 0 as libc::c_int {
        Msg(
            *__errno_location(),
            b"Could not write %s\0" as *const u8 as *const libc::c_char,
            UtmpName.as_mut_ptr(),
        );
        return -(1 as libc::c_int);
    }
    (*wi).w_slot = slot;
    bcopy(
        &mut u as *mut utmp as *mut libc::c_char as *const libc::c_void,
        &mut (*wi).w_savut as *mut utmp as *mut libc::c_char as *mut libc::c_void,
        ::std::mem::size_of::<utmp>() as libc::c_ulong,
    );
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn RemoveUtmp(mut wi: *mut win) -> libc::c_int {
    let mut u: utmp = utmp {
        ut_type: 0,
        ut_pid: 0,
        ut_line: [0; 32],
        ut_id: [0; 4],
        ut_user: [0; 32],
        ut_host: [0; 256],
        ut_exit: exit_status {
            e_termination: 0,
            e_exit: 0,
        },
        ut_session: 0,
        ut_tv: C2RustUnnamed {
            tv_sec: 0,
            tv_usec: 0,
        },
        ut_addr_v6: [0; 4],
        __glibc_reserved: [0; 20],
    };
    let mut uu: *mut utmp = 0 as *mut utmp;
    let mut slot: slot_t = 0 as *mut libc::c_char;
    slot = (*wi).w_slot;
    if utmpok == 0 {
        return -(1 as libc::c_int);
    }
    if slot.is_null() || slot == -(1 as libc::c_int) as slot_t {
        (*wi).w_slot = -(1 as libc::c_int) as slot_t;
        return 0 as libc::c_int;
    }
    bzero(
        &mut u as *mut utmp as *mut libc::c_char as *mut libc::c_void,
        ::std::mem::size_of::<utmp>() as libc::c_ulong,
    );
    uu = getutslot(slot);
    if uu.is_null() {
        Msg(
            0 as libc::c_int,
            b"Utmp slot not found -> not removed\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    bcopy(
        uu as *mut libc::c_char as *const libc::c_void,
        &mut (*wi).w_savut as *mut utmp as *mut libc::c_char as *mut libc::c_void,
        ::std::mem::size_of::<utmp>() as libc::c_ulong,
    );
    u = *uu;
    makedead(&mut u);
    if pututslot(slot, &mut u, 0 as *mut libc::c_char, wi) == 0 as libc::c_int {
        Msg(
            *__errno_location(),
            b"Could not write %s\0" as *const u8 as *const libc::c_char,
            UtmpName.as_mut_ptr(),
        );
        return -(1 as libc::c_int);
    }
    (*wi).w_slot = -(1 as libc::c_int) as slot_t;
    return 0 as libc::c_int;
}
unsafe extern "C" fn getutslot(mut slot: slot_t) -> *mut utmp {
    let mut u: utmp = utmp {
        ut_type: 0,
        ut_pid: 0,
        ut_line: [0; 32],
        ut_id: [0; 4],
        ut_user: [0; 32],
        ut_host: [0; 256],
        ut_exit: exit_status {
            e_termination: 0,
            e_exit: 0,
        },
        ut_session: 0,
        ut_tv: C2RustUnnamed {
            tv_sec: 0,
            tv_usec: 0,
        },
        ut_addr_v6: [0; 4],
        __glibc_reserved: [0; 20],
    };
    bzero(
        &mut u as *mut utmp as *mut libc::c_char as *mut libc::c_void,
        ::std::mem::size_of::<utmp>() as libc::c_ulong,
    );
    strncpy(
        (u.ut_line).as_mut_ptr(),
        slot as *const libc::c_char,
        ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
    );
    setutent();
    return getutline(&mut u);
}
unsafe extern "C" fn pututslot(
    mut slot: slot_t,
    mut u: *mut utmp,
    mut host: *mut libc::c_char,
    mut wi: *mut win,
) -> libc::c_int {
    setutent();
    return (xpututline(u) != 0 as *mut utmp) as libc::c_int;
}
unsafe extern "C" fn makedead(mut u: *mut utmp) {
    (*u).ut_type = 8 as libc::c_int as libc::c_short;
    (*u).ut_exit.e_termination = 0 as libc::c_int as libc::c_short;
    (*u).ut_exit.e_exit = 0 as libc::c_int as libc::c_short;
    (*u).ut_user[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
}
unsafe extern "C" fn makeuser(
    mut u: *mut utmp,
    mut line: *mut libc::c_char,
    mut user: *mut libc::c_char,
    mut pid: libc::c_int,
) {
    let mut now: time_t = 0;
    (*u).ut_type = 7 as libc::c_int as libc::c_short;
    strncpy(
        ((*u).ut_user).as_mut_ptr(),
        user,
        ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
    );
    strncpy(
        ((*u).ut_id).as_mut_ptr(),
        line.offset(3 as libc::c_int as isize),
        ::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong,
    );
    strncpy(
        ((*u).ut_line).as_mut_ptr(),
        line,
        ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
    );
    (*u).ut_pid = pid;
    time(&mut now);
    (*u).ut_tv.tv_sec = now as int32_t;
}
unsafe extern "C" fn TtyNameSlot(mut nam: *mut libc::c_char) -> slot_t {
    return stripdev(nam);
}
unsafe extern "C" fn xpututline(mut u: *mut utmp) -> *mut utmp {
    let mut u2: *mut utmp = 0 as *mut utmp;
    pututline(u);
    setutent();
    u2 = getutline(u);
    if u2.is_null() {
        return if (*u).ut_type as libc::c_int == 8 as libc::c_int {
            u
        } else {
            0 as *mut utmp
        };
    }
    return if (*u).ut_type as libc::c_int == (*u2).ut_type as libc::c_int {
        u
    } else {
        0 as *mut utmp
    };
}
