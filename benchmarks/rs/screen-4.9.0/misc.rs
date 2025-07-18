use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type logfile;
    fn poll(__fds: *mut pollfd, __nfds: nfds_t, __timeout: libc::c_int) -> libc::c_int;
    fn select(
        __nfds: libc::c_int,
        __readfds: *mut fd_set,
        __writefds: *mut fd_set,
        __exceptfds: *mut fd_set,
        __timeout: *mut timeval,
    ) -> libc::c_int;
    fn kill(__pid: __pid_t, __sig: libc::c_int) -> libc::c_int;
    fn sigemptyset(__set: *mut sigset_t) -> libc::c_int;
    fn sigaction(
        __sig: libc::c_int,
        __act: *const sigaction,
        __oact: *mut sigaction,
    ) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn getuid() -> __uid_t;
    fn geteuid() -> __uid_t;
    fn getgid() -> __gid_t;
    fn getegid() -> __gid_t;
    fn setreuid(__ruid: __uid_t, __euid: __uid_t) -> libc::c_int;
    fn setregid(__rgid: __gid_t, __egid: __gid_t) -> libc::c_int;
    fn getdtablesize() -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn setenv(
        __name: *const libc::c_char,
        __value: *const libc::c_char,
        __replace: libc::c_int,
    ) -> libc::c_int;
    fn bcopy(__src: *const libc::c_void, __dest: *mut libc::c_void, __n: size_t);
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    static mut strnomem: [libc::c_char; 0];
    fn Panic(_: libc::c_int, _: *const libc::c_char, _: ...) -> !;
    fn LPutChar(_: *mut layer, _: *mut mchar, _: libc::c_int, _: libc::c_int);
    fn LPutStr(
        _: *mut layer,
        _: *mut libc::c_char,
        _: libc::c_int,
        _: *mut mchar,
        _: libc::c_int,
        _: libc::c_int,
    );
    static mut flayer: *mut layer;
    static mut eff_uid: libc::c_int;
    static mut real_uid: libc::c_int;
    static mut eff_gid: libc::c_int;
    static mut real_gid: libc::c_int;
    static mut mchar_blank: mchar;
    static mut blank: *mut libc::c_uchar;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __pid_t = libc::c_int;
pub type __clock_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type nfds_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pollfd {
    pub fd: libc::c_int,
    pub events: libc::c_short,
    pub revents: libc::c_short,
}
pub type pid_t = __pid_t;
pub type size_t = libc::c_ulong;
pub type int32_t = __int32_t;
pub type sigset_t = __sigset_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type __fd_mask = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fd_set {
    pub fds_bits: [__fd_mask; 16],
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
    pub ut_tv: C2RustUnnamed_10,
    pub ut_addr_v6: [int32_t; 4],
    pub __glibc_reserved: [libc::c_char; 20],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
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
    pub l_mouseevent: C2RustUnnamed_12,
    pub l_pause: C2RustUnnamed_11,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct C2RustUnnamed_11 {
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
pub struct C2RustUnnamed_12 {
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
    pub w_alt: C2RustUnnamed_13,
    pub w_destroyev: event,
    pub w_exitstatus: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_13 {
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
pub unsafe extern "C" fn SaveStr(mut str: *const libc::c_char) -> *mut libc::c_char {
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    cp = malloc((strlen(str)).wrapping_add(1 as libc::c_int as libc::c_ulong))
        as *mut libc::c_char;
    if cp.is_null() {
        Panic(
            0 as libc::c_int,
            b"%s\0" as *const u8 as *const libc::c_char,
            strnomem.as_mut_ptr(),
        );
    } else {
        strcpy(cp, str);
    }
    return cp;
}
pub unsafe extern "C" fn SaveStrn(
    mut str: *const libc::c_char,
    mut n: libc::c_int,
) -> *mut libc::c_char {
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    cp = malloc((n + 1 as libc::c_int) as libc::c_ulong) as *mut libc::c_char;
    if cp.is_null() {
        Panic(
            0 as libc::c_int,
            b"%s\0" as *const u8 as *const libc::c_char,
            strnomem.as_mut_ptr(),
        );
    } else {
        bcopy(
            str as *mut libc::c_char as *const libc::c_void,
            cp as *mut libc::c_void,
            n as size_t,
        );
        *cp.offset(n as isize) = 0 as libc::c_int as libc::c_char;
    }
    return cp;
}
pub unsafe extern "C" fn InStr(
    mut str: *mut libc::c_char,
    mut pat: *const libc::c_char,
) -> *mut libc::c_char {
    let mut npat: libc::c_int = strlen(pat) as libc::c_int;
    while *str != 0 {
        if strncmp(str, pat, npat as libc::c_ulong) == 0 {
            return str;
        }
        str = str.offset(1);
        str;
    }
    return 0 as *mut libc::c_char;
}
pub unsafe extern "C" fn centerline(mut str: *mut libc::c_char, mut y: libc::c_int) {
    let mut l: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    n = strlen(str) as libc::c_int;
    if n > (*flayer).l_width - 1 as libc::c_int {
        n = (*flayer).l_width - 1 as libc::c_int;
    }
    l = ((*flayer).l_width - 1 as libc::c_int - n) / 2 as libc::c_int;
    LPutStr(flayer, str, n, &mut mchar_blank, l, y);
}
pub unsafe extern "C" fn leftline(
    mut str: *mut libc::c_char,
    mut y: libc::c_int,
    mut rend: *mut mchar,
) {
    let mut l: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut mchar_dol: mchar = mchar {
        image: 0,
        attr: 0,
        font: 0,
        fontx: 0,
        color: 0,
        mbcs: 0,
    };
    mchar_dol = mchar_blank;
    mchar_dol.image = '$' as i32 as libc::c_uchar;
    n = strlen(str) as libc::c_int;
    l = n;
    if n > (*flayer).l_width - 1 as libc::c_int {
        n = (*flayer).l_width - 1 as libc::c_int;
    }
    LPutStr(
        flayer,
        str,
        n,
        if !rend.is_null() { rend } else { &mut mchar_blank },
        0 as libc::c_int,
        y,
    );
    if n != l {
        LPutChar(flayer, &mut mchar_dol, n, y);
    }
}
pub unsafe extern "C" fn Filename(mut s: *mut libc::c_char) -> *mut libc::c_char {
    let mut p: *mut libc::c_char = s;
    if !p.is_null() {
        while *p != 0 {
            let fresh0 = p;
            p = p.offset(1);
            if *fresh0 as libc::c_int == '/' as i32 {
                s = p;
            }
        }
    }
    return s;
}
pub unsafe extern "C" fn stripdev(mut nam: *mut libc::c_char) -> *mut libc::c_char {
    if nam.is_null() {
        return 0 as *mut libc::c_char;
    }
    if strncmp(
        nam,
        b"/dev/\0" as *const u8 as *const libc::c_char,
        5 as libc::c_int as libc::c_ulong,
    ) == 0 as libc::c_int
    {
        return nam.offset(5 as libc::c_int as isize);
    }
    return nam;
}
pub unsafe extern "C" fn xsignal(
    mut sig: libc::c_int,
    mut func: Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
) -> Option::<unsafe extern "C" fn(libc::c_int) -> ()> {
    let mut osa: sigaction = sigaction {
        __sigaction_handler: C2RustUnnamed_9 {
            sa_handler: None,
        },
        sa_mask: __sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    let mut sa: sigaction = sigaction {
        __sigaction_handler: C2RustUnnamed_9 {
            sa_handler: None,
        },
        sa_mask: __sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    sa.__sigaction_handler.sa_handler = func;
    sigemptyset(&mut sa.sa_mask);
    sa
        .sa_flags = if sig == 17 as libc::c_int {
        0x10000000 as libc::c_int
    } else {
        0 as libc::c_int
    };
    if sigaction(sig, &mut sa, &mut osa) != 0 {
        return ::std::mem::transmute::<
            libc::intptr_t,
            Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
        >(-(1 as libc::c_int) as libc::intptr_t);
    }
    return osa.__sigaction_handler.sa_handler;
}
pub unsafe extern "C" fn xseteuid(mut euid: libc::c_int) {
    let mut oeuid: libc::c_int = 0;
    oeuid = geteuid() as libc::c_int;
    if oeuid == euid {
        return;
    }
    if getuid() as libc::c_int != euid {
        oeuid = getuid() as libc::c_int;
    }
    if setreuid(oeuid as __uid_t, euid as __uid_t) != 0 {
        Panic(*__errno_location(), b"setreuid\0" as *const u8 as *const libc::c_char);
    }
}
pub unsafe extern "C" fn xsetegid(mut egid: libc::c_int) {
    let mut oegid: libc::c_int = 0;
    oegid = getegid() as libc::c_int;
    if oegid == egid {
        return;
    }
    if getgid() as libc::c_int != egid {
        oegid = getgid() as libc::c_int;
    }
    if setregid(oegid as __gid_t, egid as __gid_t) != 0 {
        Panic(*__errno_location(), b"setregid\0" as *const u8 as *const libc::c_char);
    }
}
pub unsafe extern "C" fn bclear(mut p: *mut libc::c_char, mut n: libc::c_int) {
    bcopy(
        blank as *mut libc::c_char as *const libc::c_void,
        p as *mut libc::c_void,
        n as size_t,
    );
}
pub unsafe extern "C" fn Kill(mut pid: libc::c_int, mut sig: libc::c_int) {
    if pid < 2 as libc::c_int {
        return;
    }
    kill(pid, sig);
}
pub unsafe extern "C" fn closeallfiles(mut except: libc::c_int) {
    let mut f: libc::c_int = 0;
    let mut pfd: [pollfd; 1024] = [pollfd {
        fd: 0,
        events: 0,
        revents: 0,
    }; 1024];
    let mut maxfd: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut z: libc::c_int = 0;
    i = 3 as libc::c_int;
    maxfd = getdtablesize();
    while i < maxfd {
        memset(
            pfd.as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<[pollfd; 1024]>() as libc::c_ulong,
        );
        z = 0 as libc::c_int;
        f = i;
        while f < maxfd && f < i + 1024 as libc::c_int {
            let fresh1 = z;
            z = z + 1;
            pfd[fresh1 as usize].fd = f;
            f += 1;
            f;
        }
        ret = poll(pfd.as_mut_ptr(), (f - i) as nfds_t, 0 as libc::c_int);
        if ret < 0 as libc::c_int {
            Panic(*__errno_location(), b"poll\0" as *const u8 as *const libc::c_char);
        }
        z = 0 as libc::c_int;
        f = i;
        while f < maxfd && f < i + 1024 as libc::c_int {
            let fresh2 = z;
            z = z + 1;
            if pfd[fresh2 as usize].revents as libc::c_int & 0x20 as libc::c_int == 0
                && f != except
            {
                close(f);
            }
            f += 1;
            f;
        }
        i = f;
    }
}
static mut UserSTAT: libc::c_int = 0;
pub unsafe extern "C" fn UserContext() -> libc::c_int {
    xseteuid(real_uid);
    xsetegid(real_gid);
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn UserReturn(mut val: libc::c_int) {
    xseteuid(eff_uid);
    xsetegid(eff_gid);
    UserSTAT = val;
}
pub unsafe extern "C" fn UserStatus() -> libc::c_int {
    return UserSTAT;
}
pub unsafe extern "C" fn AddXChar(
    mut buf: *mut libc::c_char,
    mut ch: libc::c_int,
) -> libc::c_int {
    let mut p: *mut libc::c_char = buf;
    if ch < ' ' as i32 || ch == 0x7f as libc::c_int {
        let fresh3 = p;
        p = p.offset(1);
        *fresh3 = '^' as i32 as libc::c_char;
        let fresh4 = p;
        p = p.offset(1);
        *fresh4 = (ch ^ 0x40 as libc::c_int) as libc::c_char;
    } else if ch >= 0x80 as libc::c_int {
        let fresh5 = p;
        p = p.offset(1);
        *fresh5 = '\\' as i32 as libc::c_char;
        let fresh6 = p;
        p = p.offset(1);
        *fresh6 = ((ch >> 6 as libc::c_int & 7 as libc::c_int) + '0' as i32)
            as libc::c_char;
        let fresh7 = p;
        p = p.offset(1);
        *fresh7 = ((ch >> 3 as libc::c_int & 7 as libc::c_int) + '0' as i32)
            as libc::c_char;
        let fresh8 = p;
        p = p.offset(1);
        *fresh8 = ((ch >> 0 as libc::c_int & 7 as libc::c_int) + '0' as i32)
            as libc::c_char;
    } else {
        let fresh9 = p;
        p = p.offset(1);
        *fresh9 = ch as libc::c_char;
    }
    return p.offset_from(buf) as libc::c_long as libc::c_int;
}
pub unsafe extern "C" fn AddXChars(
    mut buf: *mut libc::c_char,
    mut len: libc::c_int,
    mut str: *mut libc::c_char,
) -> libc::c_int {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    if str.is_null() {
        *buf = 0 as libc::c_int as libc::c_char;
        return 0 as libc::c_int;
    }
    len -= 4 as libc::c_int;
    p = buf;
    while p < buf.offset(len as isize) && *str as libc::c_int != 0 {
        if *str as libc::c_int == ' ' as i32 {
            let fresh10 = p;
            p = p.offset(1);
            *fresh10 = *str;
        } else {
            p = p.offset(AddXChar(p, *str as libc::c_int) as isize);
        }
        str = str.offset(1);
        str;
    }
    *p = 0 as libc::c_int as libc::c_char;
    return p.offset_from(buf) as libc::c_long as libc::c_int;
}
pub unsafe extern "C" fn sleep1000(mut msec: libc::c_int) {
    let mut t: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    t.tv_sec = (msec / 1000 as libc::c_int) as libc::c_long;
    t.tv_usec = (msec % 1000 as libc::c_int * 1000 as libc::c_int) as libc::c_long;
    select(
        0 as libc::c_int,
        0 as *mut fd_set,
        0 as *mut fd_set,
        0 as *mut fd_set,
        &mut t,
    );
}
pub unsafe extern "C" fn xsetenv(
    mut var: *mut libc::c_char,
    mut value: *mut libc::c_char,
) {
    setenv(var, value, 1 as libc::c_int);
}
pub unsafe extern "C" fn _delay(
    mut delay: libc::c_int,
    mut outc: Option::<unsafe extern "C" fn(libc::c_int) -> libc::c_int>,
) -> libc::c_int {
    let mut pad: libc::c_int = 0;
    extern "C" {
        static mut ospeed: libc::c_short;
    }
    static mut osp2pad: [libc::c_short; 18] = [
        0 as libc::c_int as libc::c_short,
        2000 as libc::c_int as libc::c_short,
        1333 as libc::c_int as libc::c_short,
        909 as libc::c_int as libc::c_short,
        743 as libc::c_int as libc::c_short,
        666 as libc::c_int as libc::c_short,
        500 as libc::c_int as libc::c_short,
        333 as libc::c_int as libc::c_short,
        166 as libc::c_int as libc::c_short,
        83 as libc::c_int as libc::c_short,
        55 as libc::c_int as libc::c_short,
        41 as libc::c_int as libc::c_short,
        20 as libc::c_int as libc::c_short,
        10 as libc::c_int as libc::c_short,
        5 as libc::c_int as libc::c_short,
        2 as libc::c_int as libc::c_short,
        1 as libc::c_int as libc::c_short,
        1 as libc::c_int as libc::c_short,
    ];
    if ospeed as libc::c_int <= 0 as libc::c_int
        || ospeed as libc::c_int
            >= (::std::mem::size_of::<[libc::c_short; 18]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<libc::c_short>() as libc::c_ulong)
                as libc::c_int
    {
        return 0 as libc::c_int;
    }
    pad = osp2pad[ospeed as usize] as libc::c_int;
    delay = (delay + pad / 2 as libc::c_int) / pad;
    loop {
        let fresh11 = delay;
        delay = delay - 1;
        if !(fresh11 > 0 as libc::c_int) {
            break;
        }
        (Some(outc.unwrap())).unwrap()(0 as libc::c_int);
    }
    return 0 as libc::c_int;
}
