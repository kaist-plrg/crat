use ::libc;
use ::c2rust_bitfields;
use ::c2rust_asm_casts;
use c2rust_asm_casts::AsmCastTrait;
use std::arch::asm;
extern "C" {
    pub type logfile;
    fn select(
        __nfds: libc::c_int,
        __readfds: *mut fd_set,
        __writefds: *mut fd_set,
        __exceptfds: *mut fd_set,
        __timeout: *mut timeval,
    ) -> libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
    fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, _: ...) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn sleep(__seconds: libc::c_uint) -> libc::c_uint;
    fn dup(__fd: libc::c_int) -> libc::c_int;
    fn execvpe(
        __file: *const libc::c_char,
        __argv: *const *mut libc::c_char,
        __envp: *const *mut libc::c_char,
    ) -> libc::c_int;
    fn setuid(__uid: __uid_t) -> libc::c_int;
    fn setgid(__gid: __gid_t) -> libc::c_int;
    fn fork() -> __pid_t;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn bcopy(__src: *const libc::c_void, __dest: *mut libc::c_void, __n: size_t);
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn gettimeofday(__tv: *mut timeval, __tz: *mut libc::c_void) -> libc::c_int;
    fn cfgetospeed(__termios_p: *const termios) -> speed_t;
    fn tcflush(__fd: libc::c_int, __queue_selector: libc::c_int) -> libc::c_int;
    fn tputs(
        _: *mut libc::c_char,
        _: libc::c_int,
        _: Option::<unsafe extern "C" fn(libc::c_int) -> libc::c_int>,
    );
    fn tgoto(_: *mut libc::c_char, _: libc::c_int, _: libc::c_int) -> *mut libc::c_char;
    fn SetForeCanvas(_: *mut display, _: *mut canvas);
    fn FindCanvas(_: libc::c_int, _: libc::c_int) -> *mut canvas;
    fn FreeCanvas(_: *mut canvas);
    fn OpenDevice(
        _: *mut *mut libc::c_char,
        _: libc::c_int,
        _: *mut libc::c_int,
        _: *mut *mut libc::c_char,
    ) -> libc::c_int;
    fn Hangup();
    fn Kill(_: libc::c_int, _: libc::c_int);
    fn Msg(_: libc::c_int, _: *const libc::c_char, _: ...);
    fn Panic(_: libc::c_int, _: *const libc::c_char, _: ...) -> !;
    fn MakeWinMsgEv(
        _: *mut libc::c_char,
        _: *mut win,
        _: libc::c_int,
        _: libc::c_int,
        _: *mut event,
        _: libc::c_int,
    ) -> *mut libc::c_char;
    fn PutWinMsg(_: *mut libc::c_char, _: libc::c_int, _: libc::c_int);
    fn SetTTY(_: libc::c_int, _: *mut mode);
    fn SetFlow(_: libc::c_int);
    fn fgtty(_: libc::c_int) -> libc::c_int;
    fn brktty(_: libc::c_int);
    fn lookup_baud(bps: libc::c_int) -> *mut baud_values;
    fn zmodem_abort(_: *mut win, _: *mut display);
    fn InitPTY(_: libc::c_int);
    fn ProcessInput(_: *mut libc::c_char, _: libc::c_int);
    fn ProcessInput2(_: *mut libc::c_char, _: libc::c_int);
    fn DoAction(_: *mut action, _: libc::c_int);
    fn Activate(_: libc::c_int);
    fn ApplyAttrColor(_: libc::c_int, _: *mut mchar);
    fn StuffKey(_: libc::c_int) -> libc::c_int;
    fn FreeTransTable();
    fn ChangeScreenSize(_: libc::c_int, _: libc::c_int, _: libc::c_int);
    fn CheckScreenSize(_: libc::c_int);
    fn evenq(_: *mut event);
    fn evdeq(_: *mut event);
    fn SetTimeout(_: *mut event, _: libc::c_int);
    fn closeallfiles(_: libc::c_int);
    fn xsignal(
        _: libc::c_int,
        _: Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
    ) -> Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
    fn sleep1000(_: libc::c_int);
    fn FindUserPtr(_: *mut libc::c_char) -> *mut *mut acluser;
    fn UserAdd(
        _: *mut libc::c_char,
        _: *mut libc::c_char,
        _: *mut *mut acluser,
    ) -> libc::c_int;
    fn LGotoPos(_: *mut layer, _: libc::c_int, _: libc::c_int);
    fn LClearLine(
        _: *mut layer,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: *mut mline,
    );
    fn LSetRendition(_: *mut layer, _: *mut mchar);
    fn LCursorVisibility(_: *mut layer, _: libc::c_int);
    fn LSetFlow(_: *mut layer, _: libc::c_int);
    fn LKeypadMode(_: *mut layer, _: libc::c_int);
    fn LCursorkeysMode(_: *mut layer, _: libc::c_int);
    fn LMouseMode(_: *mut layer, _: libc::c_int);
    fn FromUtf8(_: libc::c_int, _: *mut libc::c_int) -> libc::c_int;
    fn AddUtf8(_: libc::c_int);
    fn utf8_isdouble(_: libc::c_int) -> libc::c_int;
    fn utf8_iscomb(_: libc::c_int) -> libc::c_int;
    fn CanEncodeFont(_: libc::c_int, _: libc::c_int) -> libc::c_int;
    fn DecodeChar(_: libc::c_int, _: libc::c_int, _: *mut libc::c_int) -> libc::c_int;
    fn PrepareEncodedChar(_: libc::c_int) -> libc::c_int;
    fn EncodeChar(
        _: *mut libc::c_char,
        _: libc::c_int,
        _: libc::c_int,
        _: *mut libc::c_int,
    ) -> libc::c_int;
    static mut flayer: *mut layer;
    static mut windows: *mut win;
    static mut fore: *mut win;
    static mut use_hardstatus: libc::c_int;
    static mut MsgWait: libc::c_int;
    static mut MsgMinWait: libc::c_int;
    static Z0width: libc::c_int;
    static Z1width: libc::c_int;
    static mut mline_blank: mline;
    static mut mline_null: mline;
    static mut mline_old: mline;
    static mut mchar_null: mchar;
    static mut mchar_blank: mchar;
    static mut mchar_so: mchar;
    static mut nwin_default: NewWindow;
    static mut idleaction: action;
    static mut hstatusstring: *mut libc::c_char;
    static mut captionstring: *mut libc::c_char;
    static mut pastefont: libc::c_int;
    static mut idletimo: libc::c_int;
    static mut pty_preopen: libc::c_int;
    static mut glwz: winsize;
    static mut NewEnv: *mut *mut libc::c_char;
    static mut real_uid: libc::c_int;
    static mut real_gid: libc::c_int;
    static mut ServerSocket: libc::c_int;
    static mut eff_uid: libc::c_int;
    static mut eff_gid: libc::c_int;
    static mut ospeed: libc::c_short;
}
pub type __int32_t = libc::c_int;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __ssize_t = libc::c_long;
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
pub type __fd_mask = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fd_set {
    pub fds_bits: [__fd_mask; 16],
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
pub type move_t = libc::c_uint;
pub const M_CR: move_t = 10;
pub const M_RW: move_t = 9;
pub const M_CRI: move_t = 8;
pub const M_RI: move_t = 7;
pub const M_CLE: move_t = 6;
pub const M_LE: move_t = 5;
pub const M_CDO: move_t = 4;
pub const M_DO: move_t = 3;
pub const M_CUP: move_t = 2;
pub const M_UP: move_t = 1;
pub const M_NONE: move_t = 0;
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
pub struct baud_values {
    pub idx: libc::c_int,
    pub bps: libc::c_int,
    pub sym: libc::c_int,
}
pub const CSI_INACTIVE: C2RustUnnamed_3 = 6;
pub const CSI_PY: C2RustUnnamed_3 = 2;
pub const CSI_PX: C2RustUnnamed_3 = 1;
pub const CSI_PB: C2RustUnnamed_3 = 0;
pub const CSI_DONE: C2RustUnnamed_3 = 3;
pub const CSI_INVALID: C2RustUnnamed_3 = 7;
pub const CSI_BEGIN: C2RustUnnamed_3 = 5;
pub const CSI_ESC_SEEN: C2RustUnnamed_3 = 4;
pub type C2RustUnnamed_3 = libc::c_uint;
pub static mut display: *mut display = 0 as *const display as *mut display;
pub static mut displays: *mut display = 0 as *const display as *mut display;
pub static mut attr2color: [[libc::c_int; 4]; 8] = [[0; 4]; 8];
pub static mut nattr2color: libc::c_int = 0;
pub static mut defobuflimit: libc::c_int = 256 as libc::c_int;
pub static mut defnonblock: libc::c_int = -(1 as libc::c_int);
pub static mut defmousetrack: libc::c_int = 0 as libc::c_int;
pub static mut defautonuke: libc::c_int = 0 as libc::c_int;
pub static mut captionalways: libc::c_int = 0;
pub static mut hardstatusemu: libc::c_int = 0 as libc::c_int;
pub static mut focusminwidth: libc::c_int = 0;
pub static mut focusminheight: libc::c_int = 0;
pub unsafe extern "C" fn DefProcess(
    mut bufp: *mut *mut libc::c_char,
    mut lenp: *mut libc::c_int,
) {
    *bufp = (*bufp).offset(*lenp as isize);
    *lenp = 0 as libc::c_int;
}
pub unsafe extern "C" fn DefRedisplayLine(
    mut y: libc::c_int,
    mut xs: libc::c_int,
    mut xe: libc::c_int,
    mut isblank: libc::c_int,
) {
    if isblank == 0 as libc::c_int && y >= 0 as libc::c_int {
        DefClearLine(y, xs, xe, 0 as libc::c_int);
    }
}
pub unsafe extern "C" fn DefClearLine(
    mut y: libc::c_int,
    mut xs: libc::c_int,
    mut xe: libc::c_int,
    mut bce: libc::c_int,
) {
    LClearLine(flayer, y, xs, xe, bce, 0 as *mut mline);
}
pub unsafe extern "C" fn DefRewrite(
    mut y: libc::c_int,
    mut xs: libc::c_int,
    mut xe: libc::c_int,
    mut rend: *mut mchar,
    mut doit: libc::c_int,
) -> libc::c_int {
    return 1000 as libc::c_int;
}
pub unsafe extern "C" fn DefResize(
    mut wi: libc::c_int,
    mut he: libc::c_int,
) -> libc::c_int {
    return -(1 as libc::c_int);
}
pub unsafe extern "C" fn DefRestore() {
    let mut olddisplay: *mut display = display;
    let mut cv: *mut canvas = 0 as *mut canvas;
    display = displays;
    while !display.is_null() {
        cv = (*display).d_cvlist;
        while !cv.is_null() {
            if (*cv).c_layer == flayer {
                break;
            }
            cv = (*cv).c_next;
        }
        if !cv.is_null() {
            InsertMode(0 as libc::c_int);
        }
        display = (*display).d_next;
    }
    display = olddisplay;
    LKeypadMode(flayer, 0 as libc::c_int);
    LCursorkeysMode(flayer, 0 as libc::c_int);
    LCursorVisibility(flayer, 0 as libc::c_int);
    LMouseMode(flayer, 0 as libc::c_int);
    LSetRendition(flayer, &mut mchar_null);
    LSetFlow(flayer, nwin_default.flowflag & (1 as libc::c_int) << 0 as libc::c_int);
}
pub static mut BlankLf: LayFuncs = unsafe {
    {
        let mut init = LayFuncs {
            lf_LayProcess: ::std::mem::transmute::<
                Option::<unsafe extern "C" fn() -> ()>,
                Option::<
                    unsafe extern "C" fn(*mut *mut libc::c_char, *mut libc::c_int) -> (),
                >,
            >(
                Some(
                    ::std::mem::transmute::<
                        unsafe extern "C" fn(
                            *mut *mut libc::c_char,
                            *mut libc::c_int,
                        ) -> (),
                        unsafe extern "C" fn() -> (),
                    >(DefProcess),
                ),
            ),
            lf_LayAbort: None,
            lf_LayRedisplayLine: ::std::mem::transmute::<
                Option::<unsafe extern "C" fn() -> ()>,
                Option::<
                    unsafe extern "C" fn(
                        libc::c_int,
                        libc::c_int,
                        libc::c_int,
                        libc::c_int,
                    ) -> (),
                >,
            >(
                Some(
                    ::std::mem::transmute::<
                        unsafe extern "C" fn(
                            libc::c_int,
                            libc::c_int,
                            libc::c_int,
                            libc::c_int,
                        ) -> (),
                        unsafe extern "C" fn() -> (),
                    >(DefRedisplayLine),
                ),
            ),
            lf_LayClearLine: ::std::mem::transmute::<
                Option::<unsafe extern "C" fn() -> ()>,
                Option::<
                    unsafe extern "C" fn(
                        libc::c_int,
                        libc::c_int,
                        libc::c_int,
                        libc::c_int,
                    ) -> (),
                >,
            >(
                Some(
                    ::std::mem::transmute::<
                        unsafe extern "C" fn(
                            libc::c_int,
                            libc::c_int,
                            libc::c_int,
                            libc::c_int,
                        ) -> (),
                        unsafe extern "C" fn() -> (),
                    >(DefClearLine),
                ),
            ),
            lf_LayRewrite: ::std::mem::transmute::<
                Option::<unsafe extern "C" fn() -> libc::c_int>,
                Option::<
                    unsafe extern "C" fn(
                        libc::c_int,
                        libc::c_int,
                        libc::c_int,
                        *mut mchar,
                        libc::c_int,
                    ) -> libc::c_int,
                >,
            >(
                Some(
                    ::std::mem::transmute::<
                        unsafe extern "C" fn(
                            libc::c_int,
                            libc::c_int,
                            libc::c_int,
                            *mut mchar,
                            libc::c_int,
                        ) -> libc::c_int,
                        unsafe extern "C" fn() -> libc::c_int,
                    >(DefRewrite),
                ),
            ),
            lf_LayResize: Some(
                BlankResize
                    as unsafe extern "C" fn(libc::c_int, libc::c_int) -> libc::c_int,
            ),
            lf_LayRestore: Some(DefRestore as unsafe extern "C" fn() -> ()),
            lf_LayFree: None,
        };
        init
    }
};
unsafe extern "C" fn BlankResize(
    mut wi: libc::c_int,
    mut he: libc::c_int,
) -> libc::c_int {
    (*flayer).l_width = wi;
    (*flayer).l_height = he;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn MakeDisplay(
    mut uname: *mut libc::c_char,
    mut utty: *mut libc::c_char,
    mut term: *mut libc::c_char,
    mut fd: libc::c_int,
    mut pid: libc::c_int,
    mut Mode: *mut mode,
) -> *mut display {
    let mut u: *mut *mut acluser = 0 as *mut *mut acluser;
    let mut b: *mut baud_values = 0 as *mut baud_values;
    u = FindUserPtr(uname);
    if (*u).is_null() && UserAdd(uname, 0 as *mut libc::c_char, u) != 0 {
        return 0 as *mut display;
    }
    display = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<display>() as libc::c_ulong,
    ) as *mut display;
    if display.is_null() {
        return 0 as *mut display;
    }
    (*display).d_next = displays;
    displays = display;
    (*display).d_flow = 1 as libc::c_int;
    (*display).d_nonblock = defnonblock;
    (*display).d_userfd = fd;
    (*display).d_writeev.fd = fd;
    (*display).d_readev.fd = (*display).d_writeev.fd;
    (*display).d_readev.type_0 = 1 as libc::c_int;
    (*display).d_writeev.type_0 = 2 as libc::c_int;
    (*display).d_writeev.data = display as *mut libc::c_char;
    (*display).d_readev.data = (*display).d_writeev.data;
    (*display)
        .d_readev
        .handler = Some(
        disp_readev_fn as unsafe extern "C" fn(*mut event, *mut libc::c_char) -> (),
    );
    (*display)
        .d_writeev
        .handler = Some(
        disp_writeev_fn as unsafe extern "C" fn(*mut event, *mut libc::c_char) -> (),
    );
    evenq(&mut (*display).d_readev);
    (*display).d_writeev.condpos = &mut (*display).d_obuflen;
    (*display).d_writeev.condneg = &mut (*display).d_obuffree;
    evenq(&mut (*display).d_writeev);
    (*display).d_statusev.type_0 = 0 as libc::c_int;
    (*display).d_statusev.data = display as *mut libc::c_char;
    (*display)
        .d_statusev
        .handler = Some(
        disp_status_fn as unsafe extern "C" fn(*mut event, *mut libc::c_char) -> (),
    );
    (*display).d_hstatusev.type_0 = 0 as libc::c_int;
    (*display).d_hstatusev.data = display as *mut libc::c_char;
    (*display)
        .d_hstatusev
        .handler = Some(
        disp_hstatus_fn as unsafe extern "C" fn(*mut event, *mut libc::c_char) -> (),
    );
    (*display).d_blockedev.type_0 = 0 as libc::c_int;
    (*display).d_blockedev.data = display as *mut libc::c_char;
    (*display)
        .d_blockedev
        .handler = Some(
        disp_blocked_fn as unsafe extern "C" fn(*mut event, *mut libc::c_char) -> (),
    );
    (*display).d_blockedev.condpos = &mut (*display).d_obuffree;
    (*display).d_blockedev.condneg = &mut (*display).d_obuflenmax;
    (*display)
        .d_hstatusev
        .handler = Some(
        disp_hstatus_fn as unsafe extern "C" fn(*mut event, *mut libc::c_char) -> (),
    );
    (*display).d_mapev.type_0 = 0 as libc::c_int;
    (*display).d_mapev.data = display as *mut libc::c_char;
    (*display)
        .d_mapev
        .handler = Some(
        disp_map_fn as unsafe extern "C" fn(*mut event, *mut libc::c_char) -> (),
    );
    (*display).d_idleev.type_0 = 0 as libc::c_int;
    (*display).d_idleev.data = display as *mut libc::c_char;
    (*display)
        .d_idleev
        .handler = Some(
        disp_idle_fn as unsafe extern "C" fn(*mut event, *mut libc::c_char) -> (),
    );
    (*display).d_blankerev.type_0 = 1 as libc::c_int;
    (*display).d_blankerev.data = display as *mut libc::c_char;
    (*display)
        .d_blankerev
        .handler = Some(
        disp_blanker_fn as unsafe extern "C" fn(*mut event, *mut libc::c_char) -> (),
    );
    (*display).d_blankerev.fd = -(1 as libc::c_int);
    (*display).d_OldMode = *Mode;
    (*display).d_status_obuffree = -(1 as libc::c_int);
    Resize_obuf();
    (*display).d_obufmax = defobuflimit;
    (*display).d_obuflenmax = (*display).d_obuflen - (*display).d_obufmax;
    (*display).d_auto_nuke = defautonuke;
    (*display).d_obufp = (*display).d_obuf;
    (*display).d_printfd = -(1 as libc::c_int);
    (*display).d_userpid = pid;
    b = lookup_baud(cfgetospeed(&mut (*display).d_OldMode.tio) as libc::c_int);
    if !b.is_null() {
        (*display).d_dospeed = (*b).idx as libc::c_short;
    }
    strncpy(
        ((*display).d_usertty).as_mut_ptr(),
        utty,
        (::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    );
    (*display)
        .d_usertty[(::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        as usize] = 0 as libc::c_int as libc::c_char;
    strncpy(
        ((*display).d_termname).as_mut_ptr(),
        term,
        32 as libc::c_int as libc::c_ulong,
    );
    (*display).d_termname[32 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    (*display).d_user = *u;
    (*display)
        .d_processinput = Some(
        ProcessInput as unsafe extern "C" fn(*mut libc::c_char, libc::c_int) -> (),
    );
    (*display).d_mousetrack = defmousetrack;
    return display;
}
pub unsafe extern "C" fn FreeDisplay() {
    let mut p: *mut win = 0 as *mut win;
    let mut d: *mut display = 0 as *mut display;
    let mut dp: *mut *mut display = 0 as *mut *mut display;
    FreeTransTable();
    KillBlanker();
    if (*display).d_userfd >= 0 as libc::c_int {
        Flush(3 as libc::c_int);
        if display.is_null() {
            return;
        }
        SetTTY((*display).d_userfd, &mut (*display).d_OldMode);
        fcntl((*display).d_userfd, 4 as libc::c_int, 0 as libc::c_int);
    }
    freetty();
    if !((*display).d_tentry).is_null() {
        free((*display).d_tentry as *mut libc::c_void);
    }
    (*display).d_tentry = 0 as *mut libc::c_char;
    if !((*display).d_processinputdata).is_null() {
        free((*display).d_processinputdata as *mut libc::c_void);
    }
    (*display).d_processinputdata = 0 as *mut libc::c_char;
    (*display).d_tcinited = 0 as libc::c_int as libc::c_char;
    evdeq(&mut (*display).d_hstatusev);
    evdeq(&mut (*display).d_statusev);
    evdeq(&mut (*display).d_readev);
    evdeq(&mut (*display).d_writeev);
    evdeq(&mut (*display).d_blockedev);
    evdeq(&mut (*display).d_mapev);
    if !((*display).d_kmaps).is_null() {
        free((*display).d_kmaps as *mut libc::c_void);
        (*display).d_kmaps = 0 as *mut libc::c_uchar;
        (*display).d_aseqs = 0 as libc::c_int;
        (*display).d_nseqs = 0 as libc::c_int;
        (*display).d_seqp = 0 as *mut libc::c_uchar;
        (*display).d_seql = 0 as libc::c_int;
        (*display).d_seqh = 0 as *mut libc::c_uchar;
    }
    evdeq(&mut (*display).d_idleev);
    evdeq(&mut (*display).d_blankerev);
    dp = &mut displays;
    loop {
        d = *dp;
        if d.is_null() {
            break;
        }
        if d == display {
            break;
        }
        dp = &mut (*d).d_next;
    }
    if !((*display).d_status_lastmsg).is_null() {
        free((*display).d_status_lastmsg as *mut libc::c_void);
    }
    if !((*display).d_obuf).is_null() {
        free((*display).d_obuf as *mut libc::c_void);
    }
    *dp = (*display).d_next;
    while !((*display).d_canvas.c_slperp).is_null() {
        FreeCanvas((*display).d_canvas.c_slperp);
    }
    (*display).d_cvlist = 0 as *mut canvas;
    p = windows;
    while !p.is_null() {
        if (*p).w_pdisplay == display {
            (*p).w_pdisplay = 0 as *mut display;
        }
        if (*p).w_lastdisp == display {
            (*p).w_lastdisp = 0 as *mut display;
        }
        if (*p).w_readev.condneg == &mut (*display).d_status as *mut libc::c_int
            || (*p).w_readev.condneg == &mut (*display).d_obuflenmax as *mut libc::c_int
        {
            (*p).w_readev.condneg = 0 as *mut libc::c_int;
            (*p).w_readev.condpos = (*p).w_readev.condneg;
        }
        p = (*p).w_next;
    }
    p = windows;
    while !p.is_null() {
        if (*p).w_zdisplay == display {
            zmodem_abort(p, 0 as *mut display);
        }
        p = (*p).w_next;
    }
    if (*display).d_mousetrack != 0 {
        (*display).d_mousetrack = 0 as libc::c_int;
        MouseMode(0 as libc::c_int);
    }
    free(display as *mut libc::c_char as *mut libc::c_void);
    display = 0 as *mut display;
}
pub unsafe extern "C" fn InitTerm(mut adapt: libc::c_int) {
    (*display).d_bot = -(1 as libc::c_int);
    (*display).d_top = (*display).d_bot;
    AddCStr((*display).d_tcs[39 as libc::c_int as usize].str_0);
    AddCStr((*display).d_tcs[40 as libc::c_int as usize].str_0);
    if !((*display).d_tcs[27 as libc::c_int as usize].str_0).is_null()
        && strcmp(
            (*display).d_tcs[27 as libc::c_int as usize].str_0,
            (*display).d_tcs[28 as libc::c_int as usize].str_0,
        ) != 0
    {
        AddCStr((*display).d_tcs[28 as libc::c_int as usize].str_0);
    }
    (*display).d_insert = 0 as libc::c_int;
    AddCStr((*display).d_tcs[69 as libc::c_int as usize].str_0);
    AddCStr((*display).d_tcs[71 as libc::c_int as usize].str_0);
    (*display).d_keypad = 0 as libc::c_int;
    (*display).d_cursorkeys = 0 as libc::c_int;
    AddCStr((*display).d_tcs[55 as libc::c_int as usize].str_0);
    AddCStr((*display).d_tcs[104 as libc::c_int as usize].str_0);
    AddCStr((*display).d_tcs[99 as libc::c_int as usize].str_0);
    (*display).d_rend = mchar_null;
    (*display).d_atyp = 0 as libc::c_int as libc::c_char;
    if adapt == 0 as libc::c_int {
        ResizeDisplay((*display).d_defwidth, (*display).d_defheight);
    }
    ChangeScrollRegion(0 as libc::c_int, (*display).d_height - 1 as libc::c_int);
    (*display).d_y = 0 as libc::c_int;
    (*display).d_x = (*display).d_y;
    Flush(3 as libc::c_int);
    ClearAll();
    CheckScreenSize(if adapt != 0 { 2 as libc::c_int } else { 0 as libc::c_int });
}
pub unsafe extern "C" fn FinitTerm() {
    KillBlanker();
    if (*display).d_tcinited != 0 {
        ResizeDisplay((*display).d_defwidth, (*display).d_defheight);
        InsertMode(0 as libc::c_int);
        ChangeScrollRegion(0 as libc::c_int, (*display).d_height - 1 as libc::c_int);
        KeypadMode(0 as libc::c_int);
        CursorkeysMode(0 as libc::c_int);
        CursorVisibility(0 as libc::c_int);
        if (*display).d_mousetrack != 0 {
            (*display).d_mousetrack = 0 as libc::c_int;
        }
        MouseMode(0 as libc::c_int);
        ExtMouseMode(0 as libc::c_int);
        SetRendition(&mut mchar_null);
        SetFlow((1 as libc::c_int) << 0 as libc::c_int);
        AddCStr((*display).d_tcs[70 as libc::c_int as usize].str_0);
        AddCStr((*display).d_tcs[72 as libc::c_int as usize].str_0);
        if (*display).d_hstatus != 0 {
            ShowHStatus(0 as *mut libc::c_char);
        }
        (*display).d_y = -(1 as libc::c_int);
        (*display).d_x = (*display).d_y;
        GotoPos(0 as libc::c_int, (*display).d_height - 1 as libc::c_int);
        (*display).d_obuffree -= 1;
        if (*display).d_obuffree <= 0 as libc::c_int {
            Resize_obuf();
        }
        let fresh0 = (*display).d_obufp;
        (*display).d_obufp = ((*display).d_obufp).offset(1);
        *fresh0 = '\r' as i32 as libc::c_char;
        (*display).d_obuffree -= 1;
        if (*display).d_obuffree <= 0 as libc::c_int {
            Resize_obuf();
        }
        let fresh1 = (*display).d_obufp;
        (*display).d_obufp = ((*display).d_obufp).offset(1);
        *fresh1 = '\n' as i32 as libc::c_char;
        AddCStr((*display).d_tcs[41 as libc::c_int as usize].str_0);
    }
    Flush(3 as libc::c_int);
}
unsafe extern "C" fn INSERTCHAR(mut c: libc::c_int) {
    if (*display).d_insert == 0 && (*display).d_x < (*display).d_width - 1 as libc::c_int
    {
        if !((*display).d_tcs[29 as libc::c_int as usize].str_0).is_null()
            || !((*display).d_tcs[30 as libc::c_int as usize].str_0).is_null()
        {
            if !((*display).d_tcs[29 as libc::c_int as usize].str_0).is_null() {
                AddCStr((*display).d_tcs[29 as libc::c_int as usize].str_0);
            } else {
                AddCStr2(
                    (*display).d_tcs[30 as libc::c_int as usize].str_0,
                    1 as libc::c_int,
                );
            }
            RAW_PUTCHAR(c);
            return;
        }
        InsertMode(1 as libc::c_int);
        if (*display).d_insert == 0 {
            RefreshLine(
                (*display).d_y,
                (*display).d_x,
                (*display).d_width - 1 as libc::c_int,
                0 as libc::c_int,
            );
            return;
        }
    }
    RAW_PUTCHAR(c);
}
pub unsafe extern "C" fn PUTCHAR(mut c: libc::c_int) {
    if (*display).d_insert != 0 && (*display).d_x < (*display).d_width - 1 as libc::c_int
    {
        InsertMode(0 as libc::c_int);
    }
    RAW_PUTCHAR(c);
}
pub unsafe extern "C" fn PUTCHARLP(mut c: libc::c_int) {
    if (*display).d_x < (*display).d_width - 1 as libc::c_int {
        if (*display).d_insert != 0 {
            InsertMode(0 as libc::c_int);
        }
        RAW_PUTCHAR(c);
        return;
    }
    if (*display).d_tcs[87 as libc::c_int as usize].flg != 0
        || (*display).d_y != (*display).d_bot
    {
        let mut y: libc::c_int = (*display).d_y;
        RAW_PUTCHAR(c);
        if (*display).d_tcs[83 as libc::c_int as usize].flg != 0
            && (*display).d_tcs[87 as libc::c_int as usize].flg == 0
        {
            GotoPos((*display).d_width - 1 as libc::c_int, y);
        }
        return;
    }
    (*display).d_lp_missing = 1 as libc::c_int;
    (*display).d_rend.image = c as libc::c_uchar;
    (*display).d_lpchar = (*display).d_rend;
    if (*display).d_mbcs != 0 {
        (*display).d_lpchar.mbcs = c as libc::c_uchar;
        (*display).d_lpchar.image = (*display).d_mbcs as libc::c_uchar;
        (*display).d_mbcs = 0 as libc::c_int;
        (*display).d_x -= 1;
        (*display).d_x;
    }
}
unsafe extern "C" fn RAW_PUTCHAR(mut c: libc::c_int) {
    let mut current_block: u64;
    if (*display).d_encoding == 8 as libc::c_int {
        c = c & 255 as libc::c_int
            | ((*display).d_rend.font as libc::c_int) << 8 as libc::c_int
            | ((*display).d_rend.fontx as libc::c_int) << 16 as libc::c_int;
        if (*display).d_mbcs != 0 {
            c = (*display).d_mbcs;
            if (*display).d_x == (*display).d_width {
                (*display).d_x
                    += if (*display).d_tcs[83 as libc::c_int as usize].flg != 0 {
                        1 as libc::c_int
                    } else {
                        -(1 as libc::c_int)
                    };
            }
            (*display).d_mbcs = 0 as libc::c_int;
        } else if utf8_isdouble(c) != 0 {
            (*display).d_mbcs = c;
            (*display).d_x += 1;
            (*display).d_x;
            return;
        }
        if c < 32 as libc::c_int {
            AddCStr2((*display).d_tcs[98 as libc::c_int as usize].str_0, '0' as i32);
            (*display).d_obuffree -= 1;
            if (*display).d_obuffree <= 0 as libc::c_int {
                Resize_obuf();
            }
            let fresh2 = (*display).d_obufp;
            (*display).d_obufp = ((*display).d_obufp).offset(1);
            *fresh2 = (c + 0x5f as libc::c_int) as libc::c_char;
            AddCStr((*display).d_tcs[99 as libc::c_int as usize].str_0);
        } else if c < 0x80 as libc::c_int {
            if !((*display).d_xtable).is_null()
                && !(*((*display).d_xtable)
                    .offset((*display).d_rend.font as libc::c_int as isize))
                    .is_null()
                && !(*(*((*display).d_xtable)
                    .offset((*display).d_rend.font as libc::c_int as isize))
                    .offset(c as libc::c_uchar as libc::c_int as isize))
                    .is_null()
            {
                AddStr(
                    *(*((*display).d_xtable)
                        .offset((*display).d_rend.font as libc::c_int as isize))
                        .offset(c as libc::c_uchar as libc::c_int as isize),
                );
            } else {
                (*display).d_obuffree -= 1;
                if (*display).d_obuffree <= 0 as libc::c_int {
                    Resize_obuf();
                }
                let fresh3 = (*display).d_obufp;
                (*display).d_obufp = ((*display).d_obufp).offset(1);
                *fresh3 = c as libc::c_char;
            }
        } else {
            AddUtf8(c);
        }
        current_block = 11195778457473113840;
    } else {
        if (*display).d_rend.font as libc::c_int != 0
            && (*display).d_rend.font as libc::c_int & 0x60 as libc::c_int
                == 0 as libc::c_int
        {
            let mut t: libc::c_int = c;
            if (*display).d_mbcs == 0 as libc::c_int {
                (*display).d_mbcs = c;
                (*display).d_x += 1;
                (*display).d_x;
                return;
            }
            (*display).d_x -= 1;
            (*display).d_x;
            if (*display).d_x == (*display).d_width - 1 as libc::c_int {
                (*display).d_x
                    += if (*display).d_tcs[83 as libc::c_int as usize].flg != 0 {
                        1 as libc::c_int
                    } else {
                        -(1 as libc::c_int)
                    };
            }
            c = (*display).d_mbcs;
            (*display).d_mbcs = t;
        }
        if (*display).d_encoding != 0 {
            c = PrepareEncodedChar(c);
        }
        current_block = 6754251027526096267;
    }
    loop {
        match current_block {
            6754251027526096267 => {
                if !((*display).d_xtable).is_null()
                    && !(*((*display).d_xtable)
                        .offset((*display).d_rend.font as libc::c_int as isize))
                        .is_null()
                    && !(*(*((*display).d_xtable)
                        .offset((*display).d_rend.font as libc::c_int as isize))
                        .offset(c as libc::c_uchar as libc::c_int as isize))
                        .is_null()
                {
                    AddStr(
                        *(*((*display).d_xtable)
                            .offset((*display).d_rend.font as libc::c_int as isize))
                            .offset(c as libc::c_uchar as libc::c_int as isize),
                    );
                } else {
                    (*display).d_obuffree -= 1;
                    if (*display).d_obuffree <= 0 as libc::c_int {
                        Resize_obuf();
                    }
                    let fresh4 = (*display).d_obufp;
                    (*display).d_obufp = ((*display).d_obufp).offset(1);
                    *fresh4 = (if (*display).d_rend.font as libc::c_int != '0' as i32 {
                        c
                    } else {
                        (*display).d_c0_tab[c as libc::c_uchar as libc::c_int as usize]
                            as libc::c_int
                    }) as libc::c_char;
                }
                current_block = 11195778457473113840;
            }
            _ => {
                (*display).d_x += 1;
                if (*display).d_x >= (*display).d_width {
                    if (*display).d_tcs[83 as libc::c_int as usize].flg
                        == 0 as libc::c_int
                    {
                        (*display).d_x = (*display).d_width - 1 as libc::c_int;
                    } else if (*display).d_tcs[87 as libc::c_int as usize].flg == 0
                        || (*display).d_x > (*display).d_width
                    {
                        (*display).d_x -= (*display).d_width;
                        if (*display).d_y < (*display).d_height - 1 as libc::c_int
                            && (*display).d_y != (*display).d_bot
                        {
                            (*display).d_y += 1;
                            (*display).d_y;
                        }
                    }
                }
                if !((*display).d_mbcs != 0) {
                    break;
                }
                c = (*display).d_mbcs;
                (*display).d_mbcs = 0 as libc::c_int;
                current_block = 6754251027526096267;
            }
        }
    };
}
unsafe extern "C" fn DoAddChar(mut c: libc::c_int) -> libc::c_int {
    (*display).d_obuffree -= 1;
    if (*display).d_obuffree <= 0 as libc::c_int {
        Resize_obuf();
    }
    let fresh5 = (*display).d_obufp;
    (*display).d_obufp = ((*display).d_obufp).offset(1);
    *fresh5 = c as libc::c_char;
    return c;
}
pub unsafe extern "C" fn AddCStr(mut s: *mut libc::c_char) {
    if !display.is_null() && !s.is_null() && *s as libc::c_int != 0 {
        ospeed = (*display).d_dospeed;
        tputs(
            s,
            1 as libc::c_int,
            ::std::mem::transmute::<
                Option::<unsafe extern "C" fn() -> libc::c_int>,
                Option::<unsafe extern "C" fn(libc::c_int) -> libc::c_int>,
            >(
                Some(
                    ::std::mem::transmute::<
                        unsafe extern "C" fn(libc::c_int) -> libc::c_int,
                        unsafe extern "C" fn() -> libc::c_int,
                    >(DoAddChar),
                ),
            ),
        );
    }
}
pub unsafe extern "C" fn AddCStr2(mut s: *mut libc::c_char, mut c: libc::c_int) {
    if !display.is_null() && !s.is_null() && *s as libc::c_int != 0 {
        ospeed = (*display).d_dospeed;
        tputs(
            tgoto(s, 0 as libc::c_int, c),
            1 as libc::c_int,
            ::std::mem::transmute::<
                Option::<unsafe extern "C" fn() -> libc::c_int>,
                Option::<unsafe extern "C" fn(libc::c_int) -> libc::c_int>,
            >(
                Some(
                    ::std::mem::transmute::<
                        unsafe extern "C" fn(libc::c_int) -> libc::c_int,
                        unsafe extern "C" fn() -> libc::c_int,
                    >(DoAddChar),
                ),
            ),
        );
    }
}
pub unsafe extern "C" fn InsertMode(mut on: libc::c_int) {
    if !display.is_null() && on != (*display).d_insert
        && !((*display).d_tcs[27 as libc::c_int as usize].str_0).is_null()
    {
        (*display).d_insert = on;
        if on != 0 {
            AddCStr((*display).d_tcs[27 as libc::c_int as usize].str_0);
        } else {
            AddCStr((*display).d_tcs[28 as libc::c_int as usize].str_0);
        }
    }
}
pub unsafe extern "C" fn KeypadMode(mut on: libc::c_int) {
    if !display.is_null() {
        (*display).d_keypad = on;
    }
}
pub unsafe extern "C" fn CursorkeysMode(mut on: libc::c_int) {
    if !display.is_null() {
        (*display).d_cursorkeys = on;
    }
}
pub unsafe extern "C" fn ReverseVideo(mut on: libc::c_int) {
    if !display.is_null() && (*display).d_revvid != on
        && !((*display).d_tcs[93 as libc::c_int as usize].str_0).is_null()
    {
        (*display).d_revvid = on;
        if (*display).d_revvid != 0 {
            AddCStr((*display).d_tcs[93 as libc::c_int as usize].str_0);
        } else {
            AddCStr((*display).d_tcs[94 as libc::c_int as usize].str_0);
        }
    }
}
pub unsafe extern "C" fn CursorVisibility(mut v: libc::c_int) {
    if !display.is_null() && (*display).d_curvis != v {
        if (*display).d_curvis != 0 {
            AddCStr((*display).d_tcs[82 as libc::c_int as usize].str_0);
        }
        (*display).d_curvis = 0 as libc::c_int;
        if v == -(1 as libc::c_int)
            && !((*display).d_tcs[80 as libc::c_int as usize].str_0).is_null()
        {
            AddCStr((*display).d_tcs[80 as libc::c_int as usize].str_0);
        } else if v == 1 as libc::c_int
            && !((*display).d_tcs[81 as libc::c_int as usize].str_0).is_null()
        {
            AddCStr((*display).d_tcs[81 as libc::c_int as usize].str_0);
        } else {
            return
        }
        (*display).d_curvis = v;
    }
}
pub unsafe extern "C" fn MouseMode(mut mode: libc::c_int) {
    if display.is_null() {
        return;
    }
    if mode < (*display).d_mousetrack {
        mode = (*display).d_mousetrack;
    }
    if (*display).d_mouse != mode {
        let mut mousebuf: [libc::c_char; 20] = [0; 20];
        if (*display).d_tcs[96 as libc::c_int as usize].flg == 0 {
            return;
        }
        if (*display).d_mouse != 0 {
            sprintf(
                mousebuf.as_mut_ptr(),
                b"\x1B[?%dl\0" as *const u8 as *const libc::c_char,
                (*display).d_mouse,
            );
            AddStr(mousebuf.as_mut_ptr());
        }
        if mode != 0 {
            sprintf(
                mousebuf.as_mut_ptr(),
                b"\x1B[?%dh\0" as *const u8 as *const libc::c_char,
                mode,
            );
            AddStr(mousebuf.as_mut_ptr());
        }
        (*display).d_mouse = mode;
        (*display).d_mouse_parse.state = CSI_INACTIVE as libc::c_int;
    }
}
pub unsafe extern "C" fn ExtMouseMode(mut mode: libc::c_int) {
    if !display.is_null() && (*display).d_extmouse != mode {
        let mut mousebuf: [libc::c_char; 20] = [0; 20];
        if (*display).d_tcs[96 as libc::c_int as usize].flg == 0 {
            return;
        }
        if (*display).d_extmouse != 0 {
            sprintf(
                mousebuf.as_mut_ptr(),
                b"\x1B[?%dl\0" as *const u8 as *const libc::c_char,
                (*display).d_extmouse,
            );
            AddStr(mousebuf.as_mut_ptr());
        }
        if mode != 0 {
            sprintf(
                mousebuf.as_mut_ptr(),
                b"\x1B[?%dh\0" as *const u8 as *const libc::c_char,
                mode,
            );
            AddStr(mousebuf.as_mut_ptr());
        }
        (*display).d_extmouse = mode;
        (*display).d_mouse_parse.state = CSI_INACTIVE as libc::c_int;
    }
}
static mut StrCost: libc::c_int = 0;
unsafe extern "C" fn CountChars(mut c: libc::c_int) -> libc::c_int {
    StrCost += 1;
    StrCost;
    return c;
}
pub unsafe extern "C" fn CalcCost(mut s: *mut libc::c_char) -> libc::c_int {
    if !s.is_null() {
        StrCost = 0 as libc::c_int;
        ospeed = (*display).d_dospeed;
        tputs(
            s,
            1 as libc::c_int,
            ::std::mem::transmute::<
                Option::<unsafe extern "C" fn() -> libc::c_int>,
                Option::<unsafe extern "C" fn(libc::c_int) -> libc::c_int>,
            >(
                Some(
                    ::std::mem::transmute::<
                        unsafe extern "C" fn(libc::c_int) -> libc::c_int,
                        unsafe extern "C" fn() -> libc::c_int,
                    >(CountChars),
                ),
            ),
        );
        return StrCost;
    } else {
        return 1000 as libc::c_int
    };
}
unsafe extern "C" fn CallRewrite(
    mut y: libc::c_int,
    mut xs: libc::c_int,
    mut xe: libc::c_int,
    mut doit: libc::c_int,
) -> libc::c_int {
    let mut cv: *mut canvas = 0 as *mut canvas;
    let mut cvlist: *mut canvas = 0 as *mut canvas;
    let mut cvlnext: *mut canvas = 0 as *mut canvas;
    let mut vp: *mut viewport = 0 as *mut viewport;
    let mut oldflayer: *mut layer = 0 as *mut layer;
    let mut cost: libc::c_int = 0;
    vp = 0 as *mut viewport;
    cv = (*display).d_cvlist;
    while !cv.is_null() {
        if !(y < (*cv).c_ys || y > (*cv).c_ye || xe < (*cv).c_xs || xs > (*cv).c_xe) {
            vp = (*cv).c_vplist;
            while !vp.is_null() {
                if y >= (*vp).v_ys && y <= (*vp).v_ye && xe >= (*vp).v_xs
                    && xs <= (*vp).v_xe
                {
                    break;
                }
                vp = (*vp).v_next;
            }
            if !vp.is_null() {
                break;
            }
        }
        cv = (*cv).c_next;
    }
    if doit != 0 {
        oldflayer = flayer;
        flayer = (*cv).c_layer;
        cvlist = (*flayer).l_cvlist;
        cvlnext = (*cv).c_lnext;
        (*flayer).l_cvlist = cv;
        (*cv).c_lnext = 0 as *mut canvas;
        (Some(((*(*flayer).l_layfn).lf_LayRewrite).unwrap()))
            .unwrap()(
            y - (*vp).v_yoff,
            xs - (*vp).v_xoff,
            xe - (*vp).v_xoff,
            &mut (*display).d_rend,
            1 as libc::c_int,
        );
        (*flayer).l_cvlist = cvlist;
        (*cv).c_lnext = cvlnext;
        flayer = oldflayer;
        return 0 as libc::c_int;
    }
    if cv.is_null() || ((*cv).c_layer).is_null() {
        return 1000 as libc::c_int;
    }
    if xs < (*vp).v_xs || xe > (*vp).v_xe {
        return 1000 as libc::c_int;
    }
    if y - (*vp).v_yoff < 0 as libc::c_int
        || y - (*vp).v_yoff >= (*(*cv).c_layer).l_height
    {
        return 1000 as libc::c_int;
    }
    if xs - (*vp).v_xoff < 0 as libc::c_int
        || xe - (*vp).v_xoff >= (*(*cv).c_layer).l_width
    {
        return 1000 as libc::c_int;
    }
    if (*display).d_encoding == 8 as libc::c_int {
        (*display).d_rend.font = 0 as libc::c_int as libc::c_uchar;
    }
    oldflayer = flayer;
    flayer = (*cv).c_layer;
    cost = (Some(((*(*flayer).l_layfn).lf_LayRewrite).unwrap()))
        .unwrap()(
        y - (*vp).v_yoff,
        xs - (*vp).v_xoff,
        xe - (*vp).v_xoff,
        &mut (*display).d_rend,
        0 as libc::c_int,
    );
    flayer = oldflayer;
    if (*display).d_insert != 0 {
        cost += (*display).d_EIcost + (*display).d_IMcost;
    }
    return cost;
}
pub unsafe extern "C" fn GotoPos(mut x2: libc::c_int, mut y2: libc::c_int) {
    let mut dy: libc::c_int = 0;
    let mut dx: libc::c_int = 0;
    let mut x1: libc::c_int = 0;
    let mut y1: libc::c_int = 0;
    let mut costx: libc::c_int = 0;
    let mut costy: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut CMcost: libc::c_int = 0;
    let mut xm: move_t = M_NONE;
    let mut ym: move_t = M_NONE;
    if display.is_null() {
        return;
    }
    x1 = (*display).d_x;
    y1 = (*display).d_y;
    if x1 == (*display).d_width {
        if (*display).d_tcs[87 as libc::c_int as usize].flg != 0
            && (*display).d_tcs[83 as libc::c_int as usize].flg != 0
        {
            x1 = -(1 as libc::c_int);
        } else {
            x1 -= 1;
            x1;
        }
    }
    if x2 == (*display).d_width {
        x2 -= 1;
        x2;
    }
    dx = x2 - x1;
    dy = y2 - y1;
    if dy == 0 as libc::c_int && dx == 0 as libc::c_int {
        return;
    }
    if (*display).d_tcs[56 as libc::c_int as usize].flg == 0 {
        SetRendition(&mut mchar_null);
    }
    if !(y1 < 0 as libc::c_int || y2 > (*display).d_bot && y1 <= (*display).d_bot
        || y2 < (*display).d_top && y1 >= (*display).d_top)
    {
        if !(y1 > (*display).d_bot && y2 > y1 || y1 < (*display).d_top && y2 < y1) {
            if !((*display).d_tcs[6 as libc::c_int as usize].str_0).is_null() && x2 == 0
                && y2 == 0
            {
                s = (*display).d_tcs[6 as libc::c_int as usize].str_0;
            } else {
                s = tgoto((*display).d_tcs[5 as libc::c_int as usize].str_0, x2, y2);
            }
            CMcost = CalcCost(s);
            costx = 1000 as libc::c_int;
            if x1 >= 0 as libc::c_int {
                if dx > 0 as libc::c_int {
                    if !((*display).d_tcs[17 as libc::c_int as usize].str_0).is_null()
                        && (dx > 1 as libc::c_int
                            || ((*display).d_tcs[16 as libc::c_int as usize].str_0)
                                .is_null())
                    {
                        costx = CalcCost(
                            tgoto(
                                (*display).d_tcs[17 as libc::c_int as usize].str_0,
                                0 as libc::c_int,
                                dx,
                            ),
                        );
                        xm = M_CRI;
                    }
                    m = (*display).d_NDcost * dx;
                    if m < costx {
                        costx = m;
                        xm = M_RI;
                    }
                    if dx < costx
                        && {
                            m = CallRewrite(
                                y1,
                                x1,
                                x2 - 1 as libc::c_int,
                                0 as libc::c_int,
                            );
                            m < costx
                        }
                    {
                        costx = m;
                        xm = M_RW;
                    }
                } else if dx < 0 as libc::c_int {
                    if !((*display).d_tcs[15 as libc::c_int as usize].str_0).is_null()
                        && (dx < -(1 as libc::c_int)
                            || ((*display).d_tcs[13 as libc::c_int as usize].str_0)
                                .is_null())
                    {
                        costx = CalcCost(
                            tgoto(
                                (*display).d_tcs[15 as libc::c_int as usize].str_0,
                                0 as libc::c_int,
                                -dx,
                            ),
                        );
                        xm = M_CLE;
                    }
                    m = -dx * (*display).d_LEcost;
                    if m < costx {
                        costx = m;
                        xm = M_LE;
                    }
                } else {
                    costx = 0 as libc::c_int;
                }
            }
            if x2 + (*display).d_CRcost < costx
                && {
                    m = (if x2 != 0 {
                        CallRewrite(
                            y1,
                            0 as libc::c_int,
                            x2 - 1 as libc::c_int,
                            0 as libc::c_int,
                        )
                    } else {
                        0 as libc::c_int
                    }) + (*display).d_CRcost;
                    m < costx
                }
            {
                costx = m;
                xm = M_CR;
            }
            if !(costx >= CMcost) {
                costy = 1000 as libc::c_int;
                if dy > 0 as libc::c_int {
                    if !((*display).d_tcs[11 as libc::c_int as usize].str_0).is_null()
                        && dy > 1 as libc::c_int
                    {
                        costy = CalcCost(
                            tgoto(
                                (*display).d_tcs[11 as libc::c_int as usize].str_0,
                                0 as libc::c_int,
                                dy,
                            ),
                        );
                        ym = M_CDO;
                    }
                    m = dy
                        * (if x2 == 0 as libc::c_int {
                            (*display).d_NLcost
                        } else {
                            (*display).d_DOcost
                        });
                    if m < costy {
                        costy = m;
                        ym = M_DO;
                    }
                } else if dy < 0 as libc::c_int {
                    if !((*display).d_tcs[9 as libc::c_int as usize].str_0).is_null()
                        && (dy < -(1 as libc::c_int)
                            || ((*display).d_tcs[8 as libc::c_int as usize].str_0)
                                .is_null())
                    {
                        costy = CalcCost(
                            tgoto(
                                (*display).d_tcs[9 as libc::c_int as usize].str_0,
                                0 as libc::c_int,
                                -dy,
                            ),
                        );
                        ym = M_CUP;
                    }
                    m = -dy * (*display).d_UPcost;
                    if m < costy {
                        costy = m;
                        ym = M_UP;
                    }
                } else {
                    costy = 0 as libc::c_int;
                }
                if !(costx + costy >= CMcost) {
                    let mut current_block_95: u64;
                    match xm as libc::c_uint {
                        5 => {
                            loop {
                                let fresh6 = dx;
                                dx = dx + 1;
                                if !(fresh6 < 0 as libc::c_int) {
                                    break;
                                }
                                AddCStr((*display).d_tcs[13 as libc::c_int as usize].str_0);
                            }
                            current_block_95 = 9199578309995299736;
                        }
                        6 => {
                            AddCStr2(
                                (*display).d_tcs[15 as libc::c_int as usize].str_0,
                                -dx,
                            );
                            current_block_95 = 9199578309995299736;
                        }
                        7 => {
                            loop {
                                let fresh7 = dx;
                                dx = dx - 1;
                                if !(fresh7 > 0 as libc::c_int) {
                                    break;
                                }
                                AddCStr((*display).d_tcs[16 as libc::c_int as usize].str_0);
                            }
                            current_block_95 = 9199578309995299736;
                        }
                        8 => {
                            AddCStr2(
                                (*display).d_tcs[17 as libc::c_int as usize].str_0,
                                dx,
                            );
                            current_block_95 = 9199578309995299736;
                        }
                        10 => {
                            AddCStr((*display).d_tcs[7 as libc::c_int as usize].str_0);
                            (*display).d_x = 0 as libc::c_int;
                            x1 = 0 as libc::c_int;
                            current_block_95 = 13297161352190218458;
                        }
                        9 => {
                            current_block_95 = 13297161352190218458;
                        }
                        _ => {
                            current_block_95 = 9199578309995299736;
                        }
                    }
                    match current_block_95 {
                        13297161352190218458 => {
                            if x1 < x2 {
                                CallRewrite(
                                    y1,
                                    x1,
                                    x2 - 1 as libc::c_int,
                                    1 as libc::c_int,
                                );
                            }
                        }
                        _ => {}
                    }
                    match ym as libc::c_uint {
                        1 => {
                            loop {
                                let fresh8 = dy;
                                dy = dy + 1;
                                if !(fresh8 < 0 as libc::c_int) {
                                    break;
                                }
                                AddCStr((*display).d_tcs[8 as libc::c_int as usize].str_0);
                            }
                        }
                        2 => {
                            AddCStr2(
                                (*display).d_tcs[9 as libc::c_int as usize].str_0,
                                -dy,
                            );
                        }
                        3 => {
                            s = if x2 == 0 as libc::c_int {
                                (*display).d_tcs[19 as libc::c_int as usize].str_0
                            } else {
                                (*display).d_tcs[10 as libc::c_int as usize].str_0
                            };
                            loop {
                                let fresh9 = dy;
                                dy = dy - 1;
                                if !(fresh9 > 0 as libc::c_int) {
                                    break;
                                }
                                AddCStr(s);
                            }
                        }
                        4 => {
                            AddCStr2(
                                (*display).d_tcs[11 as libc::c_int as usize].str_0,
                                dy,
                            );
                        }
                        _ => {}
                    }
                    (*display).d_x = x2;
                    (*display).d_y = y2;
                    return;
                }
            }
        }
    }
    if !((*display).d_tcs[6 as libc::c_int as usize].str_0).is_null() && x2 == 0
        && y2 == 0
    {
        AddCStr((*display).d_tcs[6 as libc::c_int as usize].str_0);
    } else {
        AddCStr(tgoto((*display).d_tcs[5 as libc::c_int as usize].str_0, x2, y2));
    }
    (*display).d_x = x2;
    (*display).d_y = y2;
}
pub unsafe extern "C" fn ClearAll() {
    ClearArea(
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        (*display).d_width - 1 as libc::c_int,
        (*display).d_width - 1 as libc::c_int,
        (*display).d_height - 1 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
    );
}
pub unsafe extern "C" fn ClearArea(
    mut x1: libc::c_int,
    mut y1: libc::c_int,
    mut xs: libc::c_int,
    mut xe: libc::c_int,
    mut x2: libc::c_int,
    mut y2: libc::c_int,
    mut bce: libc::c_int,
    mut uselayfn: libc::c_int,
) {
    let mut y: libc::c_int = 0;
    let mut xxe: libc::c_int = 0;
    let mut cv: *mut canvas = 0 as *mut canvas;
    let mut vp: *mut viewport = 0 as *mut viewport;
    if x1 == (*display).d_width {
        x1 -= 1;
        x1;
    }
    if x2 == (*display).d_width {
        x2 -= 1;
        x2;
    }
    if xs == -(1 as libc::c_int) {
        xs = x1;
    }
    if xe == -(1 as libc::c_int) {
        xe = x2;
    }
    if (*display).d_tcs[33 as libc::c_int as usize].flg != 0 {
        SetRendition(&mut mchar_null);
    }
    if (*display).d_tcs[66 as libc::c_int as usize].flg != 0 {
        SetBackColor(bce);
    }
    if (*display).d_lp_missing != 0 && y1 <= (*display).d_bot
        && xe >= (*display).d_width - 1 as libc::c_int
    {
        if y2 > (*display).d_bot
            || y2 == (*display).d_bot && x2 >= (*display).d_width - 1 as libc::c_int
        {
            (*display).d_lp_missing = 0 as libc::c_int;
        }
    }
    if x2 == (*display).d_width - 1 as libc::c_int
        && (xs == 0 as libc::c_int || y1 == y2)
        && xe == (*display).d_width - 1 as libc::c_int
        && y2 == (*display).d_height - 1 as libc::c_int
        && (bce == 0 || (*display).d_tcs[66 as libc::c_int as usize].flg != 0)
    {
        if x1 == 0 as libc::c_int && y1 == 0 as libc::c_int
            && (*display).d_auto_nuke != 0
        {
            NukePending();
        }
        if x1 == 0 as libc::c_int && y1 == 0 as libc::c_int
            && !((*display).d_tcs[34 as libc::c_int as usize].str_0).is_null()
        {
            AddCStr((*display).d_tcs[34 as libc::c_int as usize].str_0);
            (*display).d_x = 0 as libc::c_int;
            (*display).d_y = (*display).d_x;
            return;
        }
        if !((*display).d_tcs[35 as libc::c_int as usize].str_0).is_null()
            && (y1 < y2
                || ((*display).d_tcs[37 as libc::c_int as usize].str_0).is_null())
        {
            GotoPos(x1, y1);
            AddCStr((*display).d_tcs[35 as libc::c_int as usize].str_0);
            return;
        }
    }
    if x1 == 0 as libc::c_int && xs == 0 as libc::c_int
        && (xe == (*display).d_width - 1 as libc::c_int || y1 == y2)
        && y1 == 0 as libc::c_int
        && !((*display).d_tcs[36 as libc::c_int as usize].str_0).is_null()
        && (bce == 0 || (*display).d_tcs[66 as libc::c_int as usize].flg != 0)
    {
        GotoPos(x1, y1);
        AddCStr((*display).d_tcs[36 as libc::c_int as usize].str_0);
        return;
    }
    xxe = xe;
    let mut current_block_67: u64;
    y = y1;
    while y <= y2 {
        if y == y2 {
            xxe = x2;
        }
        if x1 == 0 as libc::c_int
            && !((*display).d_tcs[38 as libc::c_int as usize].str_0).is_null()
            && (xxe != (*display).d_width - 1 as libc::c_int
                || (*display).d_x == xxe && (*display).d_y == y)
            && (bce == 0 || (*display).d_tcs[66 as libc::c_int as usize].flg != 0)
        {
            GotoPos(xxe, y);
            AddCStr((*display).d_tcs[38 as libc::c_int as usize].str_0);
        } else if xxe == (*display).d_width - 1 as libc::c_int
            && !((*display).d_tcs[37 as libc::c_int as usize].str_0).is_null()
            && (bce == 0 || (*display).d_tcs[66 as libc::c_int as usize].flg != 0)
        {
            GotoPos(x1, y);
            AddCStr((*display).d_tcs[37 as libc::c_int as usize].str_0);
        } else {
            if uselayfn != 0 {
                vp = 0 as *mut viewport;
                cv = (*display).d_cvlist;
                while !cv.is_null() {
                    if !(y < (*cv).c_ys || y > (*cv).c_ye || xxe < (*cv).c_xs
                        || x1 > (*cv).c_xe)
                    {
                        vp = (*cv).c_vplist;
                        while !vp.is_null() {
                            if y >= (*vp).v_ys && y <= (*vp).v_ye && xxe >= (*vp).v_xs
                                && x1 <= (*vp).v_xe
                            {
                                break;
                            }
                            vp = (*vp).v_next;
                        }
                        if !vp.is_null() {
                            break;
                        }
                    }
                    cv = (*cv).c_next;
                }
                if !cv.is_null() && !((*cv).c_layer).is_null() && x1 >= (*vp).v_xs
                    && xxe <= (*vp).v_xe && y - (*vp).v_yoff >= 0 as libc::c_int
                    && y - (*vp).v_yoff < (*(*cv).c_layer).l_height
                    && xxe - (*vp).v_xoff >= 0 as libc::c_int
                    && x1 - (*vp).v_xoff < (*(*cv).c_layer).l_width
                {
                    let mut oldflayer: *mut layer = flayer;
                    let mut cvlist: *mut canvas = 0 as *mut canvas;
                    let mut cvlnext: *mut canvas = 0 as *mut canvas;
                    flayer = (*cv).c_layer;
                    cvlist = (*flayer).l_cvlist;
                    cvlnext = (*cv).c_lnext;
                    (*flayer).l_cvlist = cv;
                    (*cv).c_lnext = 0 as *mut canvas;
                    (Some(((*(*flayer).l_layfn).lf_LayClearLine).unwrap()))
                        .unwrap()(
                        y - (*vp).v_yoff,
                        x1 - (*vp).v_xoff,
                        xxe - (*vp).v_xoff,
                        bce,
                    );
                    (*flayer).l_cvlist = cvlist;
                    (*cv).c_lnext = cvlnext;
                    flayer = oldflayer;
                    current_block_67 = 7659304154607701039;
                } else {
                    current_block_67 = 5597585068398118923;
                }
            } else {
                current_block_67 = 5597585068398118923;
            }
            match current_block_67 {
                7659304154607701039 => {}
                _ => {
                    ClearLine(0 as *mut mline, y, x1, xxe, bce);
                }
            }
        }
        y += 1;
        y;
        x1 = xs;
    }
}
pub unsafe extern "C" fn Redisplay(mut cur_only: libc::c_int) {
    InsertMode(0 as libc::c_int);
    ChangeScrollRegion(0 as libc::c_int, (*display).d_height - 1 as libc::c_int);
    KeypadMode(0 as libc::c_int);
    CursorkeysMode(0 as libc::c_int);
    CursorVisibility(0 as libc::c_int);
    MouseMode(0 as libc::c_int);
    ExtMouseMode(0 as libc::c_int);
    SetRendition(&mut mchar_null);
    SetFlow((1 as libc::c_int) << 0 as libc::c_int);
    ClearAll();
    if cur_only > 0 as libc::c_int && !((*display).d_fore).is_null() {
        RefreshArea(
            0 as libc::c_int,
            (*(*display).d_fore).w_layer.l_y,
            (*display).d_width - 1 as libc::c_int,
            (*(*display).d_fore).w_layer.l_y,
            1 as libc::c_int,
        );
    } else {
        RefreshAll(1 as libc::c_int);
    }
    RefreshHStatus();
    let mut olddisplay: *mut display = display;
    let mut oldflayer: *mut layer = flayer;
    let mut l: *mut layer = (*(*display).d_forecv).c_layer;
    let mut cvlist: *mut canvas = (*l).l_cvlist;
    let mut cvlnext: *mut canvas = (*(*display).d_forecv).c_lnext;
    flayer = l;
    (*l).l_cvlist = (*display).d_forecv;
    (*(*display).d_forecv).c_lnext = 0 as *mut canvas;
    (Some(((*(*flayer).l_layfn).lf_LayRestore).unwrap())).unwrap()();
    LGotoPos(flayer, (*flayer).l_x, (*flayer).l_y);
    flayer = oldflayer;
    (*l).l_cvlist = cvlist;
    (*(*display).d_forecv).c_lnext = cvlnext;
    display = olddisplay;
}
pub unsafe extern "C" fn RedisplayDisplays(mut cur_only: libc::c_int) {
    let mut olddisplay: *mut display = display;
    display = displays;
    while !display.is_null() {
        Redisplay(cur_only);
        display = (*display).d_next;
    }
    display = olddisplay;
}
pub unsafe extern "C" fn ScrollH(
    mut y: libc::c_int,
    mut xs: libc::c_int,
    mut xe: libc::c_int,
    mut n: libc::c_int,
    mut bce: libc::c_int,
    mut oml: *mut mline,
) {
    let mut i: libc::c_int = 0;
    if n == 0 as libc::c_int {
        return;
    }
    if xe != (*display).d_width - 1 as libc::c_int {
        RefreshLine(y, xs, xe, 0 as libc::c_int);
        return;
    }
    GotoPos(xs, y);
    if (*display).d_tcs[33 as libc::c_int as usize].flg != 0 {
        SetRendition(&mut mchar_null);
    }
    if (*display).d_tcs[66 as libc::c_int as usize].flg != 0 {
        SetBackColor(bce);
    }
    if n > 0 as libc::c_int {
        if n >= xe - xs + 1 as libc::c_int {
            n = xe - xs + 1 as libc::c_int;
        }
        if !((*display).d_tcs[32 as libc::c_int as usize].str_0).is_null()
            && !(n == 1 as libc::c_int
                && !((*display).d_tcs[31 as libc::c_int as usize].str_0).is_null())
        {
            AddCStr2((*display).d_tcs[32 as libc::c_int as usize].str_0, n);
        } else if !((*display).d_tcs[31 as libc::c_int as usize].str_0).is_null() {
            i = n;
            loop {
                let fresh10 = i;
                i = i - 1;
                if !(fresh10 != 0) {
                    break;
                }
                AddCStr((*display).d_tcs[31 as libc::c_int as usize].str_0);
            }
        } else {
            RefreshLine(y, xs, xe, 0 as libc::c_int);
            return;
        }
    } else {
        if -n >= xe - xs + 1 as libc::c_int {
            n = -(xe - xs + 1 as libc::c_int);
        }
        if (*display).d_insert == 0 {
            if !((*display).d_tcs[30 as libc::c_int as usize].str_0).is_null()
                && !(n == -(1 as libc::c_int)
                    && !((*display).d_tcs[29 as libc::c_int as usize].str_0).is_null())
            {
                AddCStr2((*display).d_tcs[30 as libc::c_int as usize].str_0, -n);
            } else if !((*display).d_tcs[29 as libc::c_int as usize].str_0).is_null() {
                i = -n;
                loop {
                    let fresh11 = i;
                    i = i - 1;
                    if !(fresh11 != 0) {
                        break;
                    }
                    AddCStr((*display).d_tcs[29 as libc::c_int as usize].str_0);
                }
            } else if !((*display).d_tcs[27 as libc::c_int as usize].str_0).is_null() {
                InsertMode(1 as libc::c_int);
                SetRendition(&mut mchar_null);
                SetBackColor(bce);
                i = -n;
                loop {
                    let fresh12 = i;
                    i = i - 1;
                    if !(fresh12 != 0) {
                        break;
                    }
                    INSERTCHAR(' ' as i32);
                }
                bce = 0 as libc::c_int;
            } else {
                RefreshLine(y, xs, xe, 0 as libc::c_int);
                return;
            }
        } else {
            SetRendition(&mut mchar_null);
            SetBackColor(bce);
            i = -n;
            loop {
                let fresh13 = i;
                i = i - 1;
                if !(fresh13 != 0) {
                    break;
                }
                INSERTCHAR(' ' as i32);
            }
            bce = 0 as libc::c_int;
        }
    }
    if bce != 0 && (*display).d_tcs[66 as libc::c_int as usize].flg == 0 {
        if n > 0 as libc::c_int {
            ClearLine(0 as *mut mline, y, xe - n + 1 as libc::c_int, xe, bce);
        } else {
            ClearLine(0 as *mut mline, y, xs, xs - n - 1 as libc::c_int, bce);
        }
    }
    if (*display).d_lp_missing != 0 && y == (*display).d_bot {
        if n > 0 as libc::c_int {
            WriteLP((*display).d_width - 1 as libc::c_int - n, y);
        }
        (*display).d_lp_missing = 0 as libc::c_int;
    }
}
pub unsafe extern "C" fn ScrollV(
    mut xs: libc::c_int,
    mut ys: libc::c_int,
    mut xe: libc::c_int,
    mut ye: libc::c_int,
    mut n: libc::c_int,
    mut bce: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut up: libc::c_int = 0;
    let mut oldbot: libc::c_int = 0;
    let mut alok: libc::c_int = 0;
    let mut dlok: libc::c_int = 0;
    let mut aldlfaster: libc::c_int = 0;
    let mut missy: libc::c_int = 0 as libc::c_int;
    if n == 0 as libc::c_int {
        return;
    }
    if n >= ye - ys + 1 as libc::c_int || -n >= ye - ys + 1 as libc::c_int {
        ClearArea(xs, ys, xs, xe, xe, ye, bce, 0 as libc::c_int);
        return;
    }
    if xs > (*display).d_vpxmin || xe < (*display).d_vpxmax {
        RefreshArea(xs, ys, xe, ye, 0 as libc::c_int);
        return;
    }
    if (*display).d_lp_missing != 0 {
        if (*display).d_bot > ye || (*display).d_bot < ys {
            missy = (*display).d_bot;
        } else {
            missy = (*display).d_bot - n;
            if missy > ye || missy < ys {
                (*display).d_lp_missing = 0 as libc::c_int;
            }
        }
    }
    up = 1 as libc::c_int;
    if n < 0 as libc::c_int {
        up = 0 as libc::c_int;
        n = -n;
    }
    if n >= ye - ys + 1 as libc::c_int {
        n = ye - ys + 1 as libc::c_int;
    }
    oldbot = (*display).d_bot;
    if ys < (*display).d_top || (*display).d_bot != ye {
        ChangeScrollRegion(ys, ye);
    }
    alok = (!((*display).d_tcs[22 as libc::c_int as usize].str_0).is_null()
        || !((*display).d_tcs[23 as libc::c_int as usize].str_0).is_null()
        || ys >= (*display).d_top && ye == (*display).d_bot && up != 0) as libc::c_int;
    dlok = (!((*display).d_tcs[24 as libc::c_int as usize].str_0).is_null()
        || !((*display).d_tcs[25 as libc::c_int as usize].str_0).is_null()
        || ys >= (*display).d_top && ye == (*display).d_bot && up == 0) as libc::c_int;
    if (*display).d_top != ys && !(alok != 0 && dlok != 0) {
        ChangeScrollRegion(ys, ye);
    }
    if (*display).d_lp_missing != 0
        && (oldbot != (*display).d_bot
            || oldbot == (*display).d_bot && up != 0 && (*display).d_top == ys
                && (*display).d_bot == ye)
    {
        WriteLP((*display).d_width - 1 as libc::c_int, oldbot);
        if oldbot == (*display).d_bot {
            n -= 1;
            if n == 0 as libc::c_int {
                if bce != 0 && (*display).d_tcs[66 as libc::c_int as usize].flg == 0 {
                    ClearLine(0 as *mut mline, ye, xs, xe, bce);
                }
                return;
            }
        }
    }
    if (*display).d_tcs[33 as libc::c_int as usize].flg != 0 {
        SetRendition(&mut mchar_null);
    }
    if (*display).d_tcs[66 as libc::c_int as usize].flg != 0 {
        SetBackColor(bce);
    }
    aldlfaster = (n > 1 as libc::c_int && ys >= (*display).d_top
        && ye == (*display).d_bot
        && (up != 0 && !((*display).d_tcs[25 as libc::c_int as usize].str_0).is_null()
            || up == 0
                && !((*display).d_tcs[23 as libc::c_int as usize].str_0).is_null()))
        as libc::c_int;
    if (up != 0 || !((*display).d_tcs[21 as libc::c_int as usize].str_0).is_null())
        && (*display).d_top == ys && (*display).d_bot == ye && aldlfaster == 0
    {
        if up != 0 {
            GotoPos(0 as libc::c_int, ye);
            i = n;
            loop {
                let fresh14 = i;
                i = i - 1;
                if !(fresh14 > 0 as libc::c_int) {
                    break;
                }
                AddCStr((*display).d_tcs[19 as libc::c_int as usize].str_0);
            }
        } else {
            GotoPos(0 as libc::c_int, ys);
            i = n;
            loop {
                let fresh15 = i;
                i = i - 1;
                if !(fresh15 > 0 as libc::c_int) {
                    break;
                }
                AddCStr((*display).d_tcs[21 as libc::c_int as usize].str_0);
            }
        }
    } else if alok != 0 && dlok != 0 {
        if up != 0 || ye != (*display).d_bot {
            GotoPos(
                0 as libc::c_int,
                if up != 0 { ys } else { ye + 1 as libc::c_int - n },
            );
            if !((*display).d_tcs[25 as libc::c_int as usize].str_0).is_null()
                && !(n == 1 as libc::c_int
                    && !((*display).d_tcs[24 as libc::c_int as usize].str_0).is_null())
            {
                AddCStr2((*display).d_tcs[25 as libc::c_int as usize].str_0, n);
            } else {
                i = n;
                loop {
                    let fresh16 = i;
                    i = i - 1;
                    if !(fresh16 != 0) {
                        break;
                    }
                    AddCStr((*display).d_tcs[24 as libc::c_int as usize].str_0);
                }
            }
        }
        if up == 0 || ye != (*display).d_bot {
            GotoPos(
                0 as libc::c_int,
                if up != 0 { ye + 1 as libc::c_int - n } else { ys },
            );
            if !((*display).d_tcs[23 as libc::c_int as usize].str_0).is_null()
                && !(n == 1 as libc::c_int
                    && !((*display).d_tcs[22 as libc::c_int as usize].str_0).is_null())
            {
                AddCStr2((*display).d_tcs[23 as libc::c_int as usize].str_0, n);
            } else {
                i = n;
                loop {
                    let fresh17 = i;
                    i = i - 1;
                    if !(fresh17 != 0) {
                        break;
                    }
                    AddCStr((*display).d_tcs[22 as libc::c_int as usize].str_0);
                }
            }
        }
    } else {
        RefreshArea(xs, ys, xe, ye, 0 as libc::c_int);
        return;
    }
    if bce != 0 && (*display).d_tcs[66 as libc::c_int as usize].flg == 0 {
        if up != 0 {
            ClearArea(
                xs,
                ye - n + 1 as libc::c_int,
                xs,
                xe,
                xe,
                ye,
                bce,
                0 as libc::c_int,
            );
        } else {
            ClearArea(
                xs,
                ys,
                xs,
                xe,
                xe,
                ys + n - 1 as libc::c_int,
                bce,
                0 as libc::c_int,
            );
        }
    }
    if (*display).d_lp_missing != 0 && missy != (*display).d_bot {
        WriteLP((*display).d_width - 1 as libc::c_int, missy);
    }
}
pub unsafe extern "C" fn SetAttr(mut new: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut old: libc::c_int = 0;
    let mut typ: libc::c_int = 0;
    if display.is_null()
        || {
            old = (*display).d_rend.attr as libc::c_int;
            old == new
        }
    {
        return;
    }
    (*display)
        .d_col16change = (old ^ new)
        & ((1 as libc::c_int) << 6 as libc::c_int
            | (1 as libc::c_int) << 7 as libc::c_int);
    new ^= (*display).d_col16change;
    if old == new {
        return;
    }
    (*display).d_rend.attr = new as libc::c_uchar;
    typ = (*display).d_atyp as libc::c_int;
    if new & old != old {
        if typ & (1 as libc::c_int) << 2 as libc::c_int != 0 {
            AddCStr((*display).d_tcs[53 as libc::c_int as usize].str_0);
        }
        if typ & (1 as libc::c_int) << 1 as libc::c_int != 0 {
            AddCStr((*display).d_tcs[54 as libc::c_int as usize].str_0);
        }
        if typ & (1 as libc::c_int) << 0 as libc::c_int != 0 {
            AddCStr((*display).d_tcs[55 as libc::c_int as usize].str_0);
            if (*display).d_hascolor != 0 {
                (*display).d_rend.color = 0 as libc::c_int as libc::c_uchar;
                (*display)
                    .d_rend
                    .attr = ((*display).d_rend.attr as libc::c_int
                    & !((1 as libc::c_int) << 7 as libc::c_int
                        | (1 as libc::c_int) << 6 as libc::c_int)) as libc::c_uchar;
            }
            if (*display).d_tcs[97 as libc::c_int as usize].flg == 0 {
                (*display).d_rend.font = 0 as libc::c_int as libc::c_uchar;
                (*display).d_realfont = 0 as libc::c_int;
            }
        }
        old = 0 as libc::c_int;
        typ = 0 as libc::c_int;
    }
    old ^= new;
    i = 0 as libc::c_int;
    j = 1 as libc::c_int;
    while old != 0 && i < 6 as libc::c_int {
        if !(old & j == 0 as libc::c_int) {
            old ^= j;
            if !((*display).d_attrtab[i as usize]).is_null() {
                AddCStr((*display).d_attrtab[i as usize]);
                typ |= (*display).d_attrtyp[i as usize] as libc::c_int;
            }
        }
        i += 1;
        i;
        j <<= 1 as libc::c_int;
    }
    (*display).d_atyp = typ as libc::c_char;
}
pub unsafe extern "C" fn SetFont(mut new: libc::c_int) {
    let mut old: libc::c_int = (*display).d_rend.font as libc::c_int;
    if display.is_null() || old == new {
        return;
    }
    (*display).d_rend.font = new as libc::c_uchar;
    if (*display).d_encoding != 0 && CanEncodeFont((*display).d_encoding, new) != 0 {
        return;
    }
    if new == (*display).d_realfont {
        return;
    }
    (*display).d_realfont = new;
    if !((*display).d_xtable).is_null()
        && !(*((*display).d_xtable).offset(new as libc::c_uchar as libc::c_int as isize))
            .is_null()
        && !(*(*((*display).d_xtable)
            .offset(new as libc::c_uchar as libc::c_int as isize))
            .offset(256 as libc::c_int as isize))
            .is_null()
    {
        AddCStr(
            *(*((*display).d_xtable)
                .offset(new as libc::c_uchar as libc::c_int as isize))
                .offset(256 as libc::c_int as isize),
        );
        return;
    }
    if (*display).d_tcs[97 as libc::c_int as usize].flg == 0 && new != '0' as i32 {
        new = 0 as libc::c_int;
        if old == new {
            return;
        }
    }
    if new == 0 as libc::c_int {
        AddCStr((*display).d_tcs[99 as libc::c_int as usize].str_0);
    } else if new < ' ' as i32 {
        AddStr(b"\x1B$\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
        if new > 2 as libc::c_int {
            (*display).d_obuffree -= 1;
            if (*display).d_obuffree <= 0 as libc::c_int {
                Resize_obuf();
            }
            let fresh18 = (*display).d_obufp;
            (*display).d_obufp = ((*display).d_obufp).offset(1);
            *fresh18 = '(' as i32 as libc::c_char;
        }
        (*display).d_obuffree -= 1;
        if (*display).d_obuffree <= 0 as libc::c_int {
            Resize_obuf();
        }
        let fresh19 = (*display).d_obufp;
        (*display).d_obufp = ((*display).d_obufp).offset(1);
        *fresh19 = (new + '@' as i32) as libc::c_char;
    } else {
        AddCStr2((*display).d_tcs[98 as libc::c_int as usize].str_0, new);
    };
}
pub unsafe extern "C" fn color256to16(mut jj: libc::c_int) -> libc::c_int {
    let mut min: libc::c_int = 0;
    let mut max: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut g: libc::c_int = 0;
    let mut b: libc::c_int = 0;
    if jj >= 232 as libc::c_int {
        jj = (jj - 232 as libc::c_int) / 6 as libc::c_int;
        jj = (jj & 1 as libc::c_int) << 3 as libc::c_int
            | (if jj & 2 as libc::c_int != 0 {
                7 as libc::c_int
            } else {
                0 as libc::c_int
            });
    } else if jj >= 16 as libc::c_int {
        jj -= 16 as libc::c_int;
        r = jj / 36 as libc::c_int;
        g = jj / 6 as libc::c_int % 6 as libc::c_int;
        b = jj % 6 as libc::c_int;
        min = if r < g { if r < b { r } else { b } } else if g < b { g } else { b };
        max = if r > g { if r > b { r } else { b } } else if g > b { g } else { b };
        if min == max {
            jj = (max + 1 as libc::c_int & 2 as libc::c_int) << 2 as libc::c_int
                | (if max + 1 as libc::c_int & 4 as libc::c_int != 0 {
                    7 as libc::c_int
                } else {
                    0 as libc::c_int
                });
        } else {
            jj = (b - min) / (max - min) << 2 as libc::c_int
                | (g - min) / (max - min) << 1 as libc::c_int | (r - min) / (max - min)
                | (if max > 3 as libc::c_int {
                    8 as libc::c_int
                } else {
                    0 as libc::c_int
                });
        }
    }
    return jj;
}
pub unsafe extern "C" fn SetColor(mut f: libc::c_int, mut b: libc::c_int) {
    let mut of: libc::c_int = 0;
    let mut ob: libc::c_int = 0;
    static mut sftrans: [libc::c_uchar; 8] = [
        0 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        2 as libc::c_int as libc::c_uchar,
        6 as libc::c_int as libc::c_uchar,
        1 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        7 as libc::c_int as libc::c_uchar,
    ];
    if display.is_null() {
        return;
    }
    of = (*display).d_rend.color as libc::c_int & 0xf as libc::c_int
        | (if (*display).d_rend.attr as libc::c_int
            & (1 as libc::c_int) << 6 as libc::c_int != 0
        {
            0x100 as libc::c_int
        } else {
            0 as libc::c_int
        });
    ob = ((*display).d_rend.color as libc::c_int & 0xf0 as libc::c_int)
        >> 4 as libc::c_int
        | (if (*display).d_rend.attr as libc::c_int
            & (1 as libc::c_int) << 7 as libc::c_int != 0
        {
            0x100 as libc::c_int
        } else {
            0 as libc::c_int
        });
    if f == 0x100 as libc::c_int {
        f = 0 as libc::c_int;
    }
    if b == 0x100 as libc::c_int {
        b = 0 as libc::c_int;
    }
    if (*display).d_tcs[67 as libc::c_int as usize].flg == 0
        && (*display).d_hascolor != 0
        && (f == 0 as libc::c_int && f != of || b == 0 as libc::c_int && b != ob)
    {
        if !((*display).d_tcs[64 as libc::c_int as usize].str_0).is_null() {
            AddCStr((*display).d_tcs[64 as libc::c_int as usize].str_0);
        } else {
            let mut oattr: libc::c_int = 0;
            oattr = (*display).d_rend.attr as libc::c_int;
            AddCStr(
                (if !((*display).d_tcs[55 as libc::c_int as usize].str_0).is_null() {
                    (*display).d_tcs[55 as libc::c_int as usize].str_0
                        as *const libc::c_char
                } else {
                    b"\x1B[m\0" as *const u8 as *const libc::c_char
                }) as *mut libc::c_char,
            );
            if !((*display).d_tcs[55 as libc::c_int as usize].str_0).is_null()
                && (*display).d_tcs[97 as libc::c_int as usize].flg == 0
            {
                (*display).d_rend.font = 0 as libc::c_int as libc::c_uchar;
                (*display).d_realfont = 0 as libc::c_int;
            }
            (*display).d_atyp = 0 as libc::c_int as libc::c_char;
            (*display).d_rend.attr = 0 as libc::c_int as libc::c_uchar;
            SetAttr(oattr);
        }
        ob = 0 as libc::c_int;
        of = ob;
    }
    (*display)
        .d_rend
        .color = ((*display).d_rend.color as libc::c_int & 0xf0 as libc::c_int
        | f & 0xf as libc::c_int) as libc::c_uchar;
    (*display)
        .d_rend
        .attr = (((*display).d_rend.attr as libc::c_int
        | (1 as libc::c_int) << 6 as libc::c_int)
        ^ (if f & 0x100 as libc::c_int != 0 {
            0 as libc::c_int
        } else {
            (1 as libc::c_int) << 6 as libc::c_int
        })) as libc::c_uchar;
    (*display)
        .d_rend
        .color = ((*display).d_rend.color as libc::c_int & 0xf as libc::c_int
        | b << 4 as libc::c_int & 0xf0 as libc::c_int) as libc::c_uchar;
    (*display)
        .d_rend
        .attr = (((*display).d_rend.attr as libc::c_int
        | (1 as libc::c_int) << 7 as libc::c_int)
        ^ (if b & 0x100 as libc::c_int != 0 {
            0 as libc::c_int
        } else {
            (1 as libc::c_int) << 7 as libc::c_int
        })) as libc::c_uchar;
    (*display).d_col16change = 0 as libc::c_int;
    if (*display).d_hascolor == 0 {
        return;
    }
    f = if f != 0 {
        (if f & 0x1f8 as libc::c_int == 0x108 as libc::c_int {
            f ^ 0x108 as libc::c_int
        } else {
            f & 0xff as libc::c_int
        }) ^ 9 as libc::c_int
    } else {
        -(1 as libc::c_int)
    };
    b = if b != 0 {
        (if b & 0x1f8 as libc::c_int == 0x108 as libc::c_int {
            b ^ 0x108 as libc::c_int
        } else {
            b & 0xff as libc::c_int
        }) ^ 9 as libc::c_int
    } else {
        -(1 as libc::c_int)
    };
    of = if of != 0 {
        (if of & 0x1f8 as libc::c_int == 0x108 as libc::c_int {
            of ^ 0x108 as libc::c_int
        } else {
            of & 0xff as libc::c_int
        }) ^ 9 as libc::c_int
    } else {
        -(1 as libc::c_int)
    };
    ob = if ob != 0 {
        (if ob & 0x1f8 as libc::c_int == 0x108 as libc::c_int {
            ob ^ 0x108 as libc::c_int
        } else {
            ob & 0xff as libc::c_int
        }) ^ 9 as libc::c_int
    } else {
        -(1 as libc::c_int)
    };
    if f != of && f != of | 8 as libc::c_int {
        if f == -(1 as libc::c_int) {
            AddCStr(
                b"\x1B[39m\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
        } else if !((*display).d_tcs[60 as libc::c_int as usize].str_0).is_null() {
            AddCStr2(
                (*display).d_tcs[60 as libc::c_int as usize].str_0,
                f & 7 as libc::c_int,
            );
        } else if !((*display).d_tcs[62 as libc::c_int as usize].str_0).is_null() {
            AddCStr2(
                (*display).d_tcs[62 as libc::c_int as usize].str_0,
                sftrans[(f & 7 as libc::c_int) as usize] as libc::c_int,
            );
        }
    }
    if b != ob && b != ob | 8 as libc::c_int {
        if b == -(1 as libc::c_int) {
            AddCStr(
                b"\x1B[49m\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
        } else if !((*display).d_tcs[61 as libc::c_int as usize].str_0).is_null() {
            AddCStr2(
                (*display).d_tcs[61 as libc::c_int as usize].str_0,
                b & 7 as libc::c_int,
            );
        } else if !((*display).d_tcs[63 as libc::c_int as usize].str_0).is_null() {
            AddCStr2(
                (*display).d_tcs[63 as libc::c_int as usize].str_0,
                sftrans[(b & 7 as libc::c_int) as usize] as libc::c_int,
            );
        }
    }
    if f != of && (*display).d_tcs[96 as libc::c_int as usize].flg != 0
        && f & 8 as libc::c_int != 0 as libc::c_int && f != -(1 as libc::c_int)
    {
        AddCStr2(
            b"\x1B[9%p1%dm\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            f & 7 as libc::c_int,
        );
    }
    if b != ob && (*display).d_tcs[96 as libc::c_int as usize].flg != 0
        && b & 8 as libc::c_int != 0 as libc::c_int && b != -(1 as libc::c_int)
    {
        AddCStr2(
            b"\x1B[10%p1%dm\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b & 7 as libc::c_int,
        );
    }
}
unsafe extern "C" fn SetBackColor(mut new: libc::c_int) {
    if display.is_null() {
        return;
    }
    SetColor(
        (*display).d_rend.color as libc::c_int & 0xf as libc::c_int
            | (if (*display).d_rend.attr as libc::c_int
                & (1 as libc::c_int) << 6 as libc::c_int != 0
            {
                0x100 as libc::c_int
            } else {
                0 as libc::c_int
            }),
        new,
    );
}
pub unsafe extern "C" fn SetRendition(mut mc: *mut mchar) {
    if display.is_null() {
        return;
    }
    if nattr2color != 0 && (*display).d_hascolor != 0
        && (*mc).attr as libc::c_int & nattr2color != 0 as libc::c_int
    {
        static mut mmc: mchar = mchar {
            image: 0,
            attr: 0,
            font: 0,
            fontx: 0,
            color: 0,
            mbcs: 0,
        };
        let mut i: libc::c_int = 0;
        mmc = *mc;
        i = 0 as libc::c_int;
        while i < 8 as libc::c_int {
            if !(attr2color[i as usize]).as_mut_ptr().is_null()
                && (*mc).attr as libc::c_int & (1 as libc::c_int) << i
                    != 0 as libc::c_int
            {
                if (*mc).color as libc::c_int == 0 as libc::c_int
                    && attr2color[i as usize][3 as libc::c_int as usize] != 0
                {
                    ApplyAttrColor(
                        attr2color[i as usize][3 as libc::c_int as usize],
                        &mut mmc,
                    );
                } else if (*mc).color as libc::c_int & 0xf as libc::c_int
                    == 0 as libc::c_int
                    && attr2color[i as usize][2 as libc::c_int as usize] != 0
                {
                    ApplyAttrColor(
                        attr2color[i as usize][2 as libc::c_int as usize],
                        &mut mmc,
                    );
                } else if (*mc).color as libc::c_int & 0xf0 as libc::c_int
                    == 0 as libc::c_int
                    && attr2color[i as usize][1 as libc::c_int as usize] != 0
                {
                    ApplyAttrColor(
                        attr2color[i as usize][1 as libc::c_int as usize],
                        &mut mmc,
                    );
                } else {
                    ApplyAttrColor(
                        attr2color[i as usize][0 as libc::c_int as usize],
                        &mut mmc,
                    );
                }
            }
            i += 1;
            i;
        }
        mc = &mut mmc;
    }
    if (*display).d_hascolor != 0
        && (*display).d_tcs[68 as libc::c_int as usize].flg != 0
        && (*mc).attr as libc::c_int
            & ((1 as libc::c_int) << 6 as libc::c_int
                | (1 as libc::c_int) << 7 as libc::c_int) != 0
    {
        let mut a: libc::c_int = (*mc).attr as libc::c_int;
        if (*mc).attr as libc::c_int & (1 as libc::c_int) << 6 as libc::c_int != 0
            && !((*display).d_tcs[49 as libc::c_int as usize].str_0).is_null()
        {
            a |= (1 as libc::c_int) << 2 as libc::c_int;
        }
        if (*mc).attr as libc::c_int & (1 as libc::c_int) << 7 as libc::c_int != 0
            && !((*display).d_tcs[52 as libc::c_int as usize].str_0).is_null()
        {
            a |= (1 as libc::c_int) << 5 as libc::c_int;
        }
        if (*display).d_rend.attr as libc::c_int != a {
            SetAttr(a);
        }
    } else if (*display).d_rend.attr as libc::c_int != (*mc).attr as libc::c_int {
        SetAttr((*mc).attr as libc::c_int);
    }
    if (*display).d_rend.color as libc::c_int != (*mc).color as libc::c_int
        || (*display).d_col16change != 0
    {
        SetColor(
            (*mc).color as libc::c_int & 0xf as libc::c_int
                | (if (*mc).attr as libc::c_int & (1 as libc::c_int) << 6 as libc::c_int
                    != 0
                {
                    0x100 as libc::c_int
                } else {
                    0 as libc::c_int
                }),
            ((*mc).color as libc::c_int & 0xf0 as libc::c_int) >> 4 as libc::c_int
                | (if (*mc).attr as libc::c_int & (1 as libc::c_int) << 7 as libc::c_int
                    != 0
                {
                    0x100 as libc::c_int
                } else {
                    0 as libc::c_int
                }),
        );
    }
    if (*display).d_rend.font as libc::c_int != (*mc).font as libc::c_int {
        SetFont((*mc).font as libc::c_int);
    }
    if (*display).d_encoding == 8 as libc::c_int {
        (*display).d_rend.fontx = (*mc).fontx;
    }
}
pub unsafe extern "C" fn SetRenditionMline(mut ml: *mut mline, mut x: libc::c_int) {
    if display.is_null() {
        return;
    }
    if nattr2color != 0 && (*display).d_hascolor != 0
        && *((*ml).attr).offset(x as isize) as libc::c_int & nattr2color
            != 0 as libc::c_int
    {
        let mut mc: mchar = mchar {
            image: 0,
            attr: 0,
            font: 0,
            fontx: 0,
            color: 0,
            mbcs: 0,
        };
        mc.image = *((*ml).image).offset(x as isize);
        mc.attr = *((*ml).attr).offset(x as isize);
        mc.font = *((*ml).font).offset(x as isize);
        mc.fontx = *((*ml).fontx).offset(x as isize);
        mc.color = *((*ml).color).offset(x as isize);
        mc.mbcs = 0 as libc::c_int as libc::c_uchar;
        SetRendition(&mut mc);
        return;
    }
    if (*display).d_hascolor != 0
        && (*display).d_tcs[68 as libc::c_int as usize].flg != 0
        && *((*ml).attr).offset(x as isize) as libc::c_int
            & ((1 as libc::c_int) << 6 as libc::c_int
                | (1 as libc::c_int) << 7 as libc::c_int) != 0
    {
        let mut a: libc::c_int = *((*ml).attr).offset(x as isize) as libc::c_int;
        if *((*ml).attr).offset(x as isize) as libc::c_int
            & (1 as libc::c_int) << 6 as libc::c_int != 0
            && !((*display).d_tcs[49 as libc::c_int as usize].str_0).is_null()
        {
            a |= (1 as libc::c_int) << 2 as libc::c_int;
        }
        if *((*ml).attr).offset(x as isize) as libc::c_int
            & (1 as libc::c_int) << 7 as libc::c_int != 0
            && !((*display).d_tcs[52 as libc::c_int as usize].str_0).is_null()
        {
            a |= (1 as libc::c_int) << 5 as libc::c_int;
        }
        if (*display).d_rend.attr as libc::c_int != a {
            SetAttr(a);
        }
    } else if (*display).d_rend.attr as libc::c_int
        != *((*ml).attr).offset(x as isize) as libc::c_int
    {
        SetAttr(*((*ml).attr).offset(x as isize) as libc::c_int);
    }
    if (*display).d_rend.color as libc::c_int
        != *((*ml).color).offset(x as isize) as libc::c_int
        || (*display).d_col16change != 0
    {
        let mut mc_0: mchar = mchar {
            image: 0,
            attr: 0,
            font: 0,
            fontx: 0,
            color: 0,
            mbcs: 0,
        };
        mc_0.image = *((*ml).image).offset(x as isize);
        mc_0.attr = *((*ml).attr).offset(x as isize);
        mc_0.font = *((*ml).font).offset(x as isize);
        mc_0.fontx = *((*ml).fontx).offset(x as isize);
        mc_0.color = *((*ml).color).offset(x as isize);
        mc_0.mbcs = 0 as libc::c_int as libc::c_uchar;
        SetColor(
            mc_0.color as libc::c_int & 0xf as libc::c_int
                | (if mc_0.attr as libc::c_int & (1 as libc::c_int) << 6 as libc::c_int
                    != 0
                {
                    0x100 as libc::c_int
                } else {
                    0 as libc::c_int
                }),
            (mc_0.color as libc::c_int & 0xf0 as libc::c_int) >> 4 as libc::c_int
                | (if mc_0.attr as libc::c_int & (1 as libc::c_int) << 7 as libc::c_int
                    != 0
                {
                    0x100 as libc::c_int
                } else {
                    0 as libc::c_int
                }),
        );
    }
    if (*display).d_rend.font as libc::c_int
        != *((*ml).font).offset(x as isize) as libc::c_int
    {
        SetFont(*((*ml).font).offset(x as isize) as libc::c_int);
    }
    if (*display).d_encoding == 8 as libc::c_int {
        (*display).d_rend.fontx = *((*ml).fontx).offset(x as isize);
    }
}
pub unsafe extern "C" fn MakeStatus(mut msg: *mut libc::c_char) {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut max: libc::c_int = 0;
    if display.is_null() {
        return;
    }
    if (*display).d_blocked != 0 {
        return;
    }
    if (*display).d_tcinited == 0 {
        if !((*display).d_processinputdata).is_null() {
            return;
        }
        AddStr(msg);
        AddStr(b"\r\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
        Flush(0 as libc::c_int);
        return;
    }
    if use_hardstatus == 0 || (*display).d_tcs[75 as libc::c_int as usize].flg == 0 {
        max = (*display).d_width;
        if (*display).d_tcs[87 as libc::c_int as usize].flg == 0 as libc::c_int {
            max -= 1;
            max;
        }
    } else {
        max = if (*display).d_tcs[76 as libc::c_int as usize].num > 0 as libc::c_int {
            (*display).d_tcs[76 as libc::c_int as usize].num
        } else {
            (*display).d_width
                - ((*display).d_tcs[87 as libc::c_int as usize].flg == 0) as libc::c_int
        };
    }
    if (*display).d_status != 0 {
        if strcmp(msg, (*display).d_status_lastmsg) == 0 as libc::c_int {
            if (*display).d_status_obufpos == 0 {
                SetTimeout(&mut (*display).d_statusev, MsgWait);
            }
            return;
        }
        RemoveStatusMinWait();
    }
    t = msg;
    s = t;
    while *s as libc::c_int != 0
        && (t.offset_from(msg) as libc::c_long) < max as libc::c_long
    {
        if *s as libc::c_int == 'g' as i32 & 0o37 as libc::c_int {
            AddCStr((*display).d_tcs[42 as libc::c_int as usize].str_0);
        } else if *s as libc::c_uchar as libc::c_int >= ' ' as i32
            && *s as libc::c_int != 0o177 as libc::c_int
        {
            let fresh20 = t;
            t = t.offset(1);
            *fresh20 = *s;
        }
        s = s.offset(1);
        s;
    }
    *t = '\0' as i32 as libc::c_char;
    if t == msg {
        return;
    }
    if t.offset_from(msg) as libc::c_long >= (*display).d_status_buflen as libc::c_long {
        let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
        if !((*display).d_status_lastmsg).is_null() {
            buf = realloc(
                (*display).d_status_lastmsg as *mut libc::c_void,
                (t.offset_from(msg) as libc::c_long + 1 as libc::c_int as libc::c_long)
                    as libc::c_ulong,
            ) as *mut libc::c_char;
        } else {
            buf = malloc(
                (t.offset_from(msg) as libc::c_long + 1 as libc::c_int as libc::c_long)
                    as libc::c_ulong,
            ) as *mut libc::c_char;
        }
        if !buf.is_null() {
            (*display).d_status_lastmsg = buf;
            (*display)
                .d_status_buflen = (t.offset_from(msg) as libc::c_long
                + 1 as libc::c_int as libc::c_long) as libc::c_int;
        }
    }
    if (t.offset_from(msg) as libc::c_long) < (*display).d_status_buflen as libc::c_long
    {
        strcpy((*display).d_status_lastmsg, msg);
    }
    (*display).d_status_len = t.offset_from(msg) as libc::c_long as libc::c_int;
    (*display).d_status_lastx = (*display).d_x;
    (*display).d_status_lasty = (*display).d_y;
    if use_hardstatus == 0 || (*display).d_has_hstatus == 0 as libc::c_int
        || (*display).d_has_hstatus == 2 as libc::c_int
    {
        (*display).d_status = 1 as libc::c_int;
        GotoPos(0 as libc::c_int, (*display).d_height - 1 as libc::c_int);
        SetRendition(&mut mchar_so);
        InsertMode(0 as libc::c_int);
        AddStr(msg);
        if (*display).d_status_len < max {
            (*display).d_status_len += 1;
            (*display).d_status_len;
            SetRendition(&mut mchar_null);
            (*display).d_obuffree -= 1;
            if (*display).d_obuffree <= 0 as libc::c_int {
                Resize_obuf();
            }
            let fresh21 = (*display).d_obufp;
            (*display).d_obufp = ((*display).d_obufp).offset(1);
            *fresh21 = ' ' as i32 as libc::c_char;
            if (*display).d_status_len < max {
                (*display).d_status_len += 1;
                (*display).d_status_len;
                (*display).d_obuffree -= 1;
                if (*display).d_obuffree <= 0 as libc::c_int {
                    Resize_obuf();
                }
                let fresh22 = (*display).d_obufp;
                (*display).d_obufp = ((*display).d_obufp).offset(1);
                *fresh22 = ' ' as i32 as libc::c_char;
                (*display).d_obuffree -= 1;
                if (*display).d_obuffree <= 0 as libc::c_int {
                    Resize_obuf();
                }
                let fresh23 = (*display).d_obufp;
                (*display).d_obufp = ((*display).d_obufp).offset(1);
                *fresh23 = '\u{8}' as i32 as libc::c_char;
            }
            (*display).d_obuffree -= 1;
            if (*display).d_obuffree <= 0 as libc::c_int {
                Resize_obuf();
            }
            let fresh24 = (*display).d_obufp;
            (*display).d_obufp = ((*display).d_obufp).offset(1);
            *fresh24 = '\u{8}' as i32 as libc::c_char;
        }
        (*display).d_x = -(1 as libc::c_int);
    } else {
        (*display).d_status = 2 as libc::c_int;
        ShowHStatus(msg);
    }
    (*display)
        .d_status_obufpos = ((*display).d_obufp).offset_from((*display).d_obuf)
        as libc::c_long as libc::c_int;
    if (*display).d_status == 1 as libc::c_int {
        let mut olddisplay: *mut display = display;
        let mut oldflayer: *mut layer = flayer;
        (*display).d_status = 0 as libc::c_int;
        GotoPos(0 as libc::c_int, (*display).d_height - 1 as libc::c_int);
        RefreshLine(
            (*display).d_height - 1 as libc::c_int,
            0 as libc::c_int,
            (*display).d_status_len - 1 as libc::c_int,
            0 as libc::c_int,
        );
        GotoPos((*display).d_status_lastx, (*display).d_status_lasty);
        flayer = if !((*display).d_forecv).is_null() {
            (*(*display).d_forecv).c_layer
        } else {
            0 as *mut layer
        };
        if !flayer.is_null() {
            LGotoPos(flayer, (*flayer).l_x, (*flayer).l_y);
        }
        display = olddisplay;
        flayer = oldflayer;
        (*display).d_status = 1 as libc::c_int;
    }
}
pub unsafe extern "C" fn RemoveStatus() {
    let mut olddisplay: *mut display = 0 as *mut display;
    let mut oldflayer: *mut layer = 0 as *mut layer;
    let mut where_0: libc::c_int = 0;
    if display.is_null() {
        return;
    }
    where_0 = (*display).d_status;
    if where_0 == 0 {
        return;
    }
    if (*display).d_status_obuffree >= 0 as libc::c_int {
        (*display).d_obuflen = (*display).d_status_obuflen;
        (*display).d_obuffree = (*display).d_status_obuffree;
        (*display).d_status_obuffree = -(1 as libc::c_int);
    }
    (*display).d_status = 0 as libc::c_int;
    (*display).d_status_obufpos = 0 as libc::c_int;
    (*display).d_status_bell = 0 as libc::c_int as libc::c_char;
    evdeq(&mut (*display).d_statusev);
    olddisplay = display;
    oldflayer = flayer;
    if where_0 == 1 as libc::c_int {
        if captionalways != 0
            || !((*display).d_canvas.c_slperp).is_null()
                && !((*(*display).d_canvas.c_slperp).c_slnext).is_null()
        {
            GotoPos(0 as libc::c_int, (*display).d_height - 1 as libc::c_int);
            RefreshLine(
                (*display).d_height - 1 as libc::c_int,
                0 as libc::c_int,
                (*display).d_status_len - 1 as libc::c_int,
                0 as libc::c_int,
            );
            GotoPos((*display).d_status_lastx, (*display).d_status_lasty);
        }
    } else {
        RefreshHStatus();
    }
    flayer = if !((*display).d_forecv).is_null() {
        (*(*display).d_forecv).c_layer
    } else {
        0 as *mut layer
    };
    if !flayer.is_null() {
        LGotoPos(flayer, (*flayer).l_x, (*flayer).l_y);
    }
    display = olddisplay;
    flayer = oldflayer;
}
unsafe extern "C" fn RemoveStatusMinWait() {
    if (*display).d_status_bell == 0 && (*display).d_status_obufpos == 0 {
        let mut now: timeval = timeval { tv_sec: 0, tv_usec: 0 };
        let mut ti: libc::c_int = 0;
        gettimeofday(&mut now, 0 as *mut libc::c_void);
        ti = ((now.tv_sec - (*display).d_status_time.tv_sec)
            * 1000 as libc::c_int as libc::c_long
            + (now.tv_usec - (*display).d_status_time.tv_usec)
                / 1000 as libc::c_int as libc::c_long) as libc::c_int;
        if ti < MsgMinWait {
            DisplaySleep1000(MsgMinWait - ti, 0 as libc::c_int);
        }
    }
    RemoveStatus();
}
unsafe extern "C" fn strlen_onscreen(
    mut c: *mut libc::c_uchar,
    mut end: *mut libc::c_uchar,
) -> libc::c_int {
    let mut len: libc::c_int = 0 as libc::c_int;
    while *c as libc::c_int != 0 && (end.is_null() || c < end) {
        let mut v: libc::c_int = 0;
        let mut dec: libc::c_int = 0 as libc::c_int;
        loop {
            let fresh25 = c;
            c = c.offset(1);
            v = FromUtf8(*fresh25 as libc::c_int, &mut dec);
            if v == -(2 as libc::c_int) {
                c = c.offset(-1);
                c;
            }
            if !(v < 0 as libc::c_int && (end.is_null() || c < end)) {
                break;
            }
        }
        if utf8_iscomb(v) == 0 {
            if utf8_isdouble(v) != 0 {
                len += 1;
                len;
            }
            len += 1;
            len;
        }
    }
    return len;
}
unsafe extern "C" fn PrePutWinMsg(
    mut s: *mut libc::c_char,
    mut start: libc::c_int,
    mut max: libc::c_int,
) -> libc::c_int {
    if (*display).d_encoding == 8 as libc::c_int {
        let mut chars: libc::c_int = strlen_onscreen(
            s.offset(start as isize) as *mut libc::c_uchar,
            s.offset(max as isize) as *mut libc::c_uchar,
        );
        (*display).d_encoding = 0 as libc::c_int;
        PutWinMsg(s, start, max + (max - start - chars));
        (*display).d_encoding = 8 as libc::c_int;
        (*display).d_x -= max - chars;
        return start + chars;
    } else {
        PutWinMsg(s, start, max);
        return max;
    };
}
pub unsafe extern "C" fn ShowHStatus(mut str: *mut libc::c_char) {
    let mut l: libc::c_int = 0;
    let mut ox: libc::c_int = 0;
    let mut oy: libc::c_int = 0;
    let mut max: libc::c_int = 0;
    if (*display).d_status == 1 as libc::c_int
        && (*display).d_has_hstatus == 1 as libc::c_int
        && (*display).d_height - 1 as libc::c_int
            == (*display).d_height - 1 as libc::c_int
    {
        return;
    }
    if (*display).d_blocked != 0 {
        return;
    }
    if (*display).d_tcs[75 as libc::c_int as usize].flg != 0
        && (*display).d_has_hstatus == 3 as libc::c_int
    {
        if (*display).d_hstatus == 0
            && (str.is_null() || *str as libc::c_int == 0 as libc::c_int)
        {
            return;
        }
        SetRendition(&mut mchar_null);
        InsertMode(0 as libc::c_int);
        if (*display).d_hstatus != 0 {
            AddCStr((*display).d_tcs[79 as libc::c_int as usize].str_0);
        }
        (*display).d_hstatus = 0 as libc::c_int;
        if str.is_null() || *str as libc::c_int == 0 as libc::c_int {
            return;
        }
        AddCStr2((*display).d_tcs[77 as libc::c_int as usize].str_0, 0 as libc::c_int);
        max = if (*display).d_tcs[76 as libc::c_int as usize].num > 0 as libc::c_int {
            (*display).d_tcs[76 as libc::c_int as usize].num
        } else {
            (*display).d_width
                - ((*display).d_tcs[87 as libc::c_int as usize].flg == 0) as libc::c_int
        };
        if strlen(str) as libc::c_int > max {
            AddStrn(str, max);
        } else {
            AddStr(str);
        }
        AddCStr((*display).d_tcs[78 as libc::c_int as usize].str_0);
        (*display).d_hstatus = 1 as libc::c_int;
    } else if (*display).d_has_hstatus == 1 as libc::c_int {
        ox = (*display).d_x;
        oy = (*display).d_y;
        str = (if !str.is_null() {
            str as *const libc::c_char
        } else {
            b"\0" as *const u8 as *const libc::c_char
        }) as *mut libc::c_char;
        l = strlen(str) as libc::c_int;
        if l > (*display).d_width {
            l = (*display).d_width;
        }
        GotoPos(0 as libc::c_int, (*display).d_height - 1 as libc::c_int);
        SetRendition(
            if captionalways != 0 || ((*display).d_cvlist).is_null()
                || !((*(*display).d_cvlist).c_next).is_null()
            {
                &mut mchar_null
            } else {
                &mut mchar_so
            },
        );
        l = PrePutWinMsg(str, 0 as libc::c_int, l);
        if captionalways == 0 && !((*display).d_cvlist).is_null()
            && ((*(*display).d_cvlist).c_next).is_null()
        {
            loop {
                let fresh26 = l;
                l = l + 1;
                if !(fresh26 < (*display).d_width) {
                    break;
                }
                PUTCHARLP(' ' as i32);
            }
        }
        if l < (*display).d_width {
            ClearArea(
                l,
                (*display).d_height - 1 as libc::c_int,
                l,
                (*display).d_width - 1 as libc::c_int,
                (*display).d_width - 1 as libc::c_int,
                (*display).d_height - 1 as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
            );
        }
        if ox != -(1 as libc::c_int) && oy != -(1 as libc::c_int) {
            GotoPos(ox, oy);
        }
        (*display)
            .d_hstatus = if *str as libc::c_int != 0 {
            1 as libc::c_int
        } else {
            0 as libc::c_int
        };
        SetRendition(&mut mchar_null);
    } else if (*display).d_has_hstatus == 4 as libc::c_int {
        ox = (*display).d_x;
        oy = (*display).d_y;
        str = (if !str.is_null() {
            str as *const libc::c_char
        } else {
            b"\0" as *const u8 as *const libc::c_char
        }) as *mut libc::c_char;
        l = strlen(str) as libc::c_int;
        if l > (*display).d_width {
            l = (*display).d_width;
        }
        GotoPos(0 as libc::c_int, 0 as libc::c_int);
        SetRendition(
            if captionalways != 0 || ((*display).d_cvlist).is_null()
                || !((*(*display).d_cvlist).c_next).is_null()
            {
                &mut mchar_null
            } else {
                &mut mchar_so
            },
        );
        l = PrePutWinMsg(str, 0 as libc::c_int, l);
        if captionalways == 0
            || !((*display).d_cvlist).is_null()
                && ((*(*display).d_cvlist).c_next).is_null()
        {
            loop {
                let fresh27 = l;
                l = l + 1;
                if !(fresh27 < (*display).d_width) {
                    break;
                }
                PUTCHARLP(' ' as i32);
            }
        }
        if l < (*display).d_width {
            ClearArea(
                l,
                0 as libc::c_int,
                l,
                (*display).d_width - 1 as libc::c_int,
                (*display).d_width - 1 as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
            );
        }
        if ox != -(1 as libc::c_int) && oy != -(1 as libc::c_int) {
            GotoPos(ox, oy);
        }
        (*display)
            .d_hstatus = if *str as libc::c_int != 0 {
            1 as libc::c_int
        } else {
            0 as libc::c_int
        };
        SetRendition(&mut mchar_null);
    } else if !str.is_null() && *str as libc::c_int != 0
        && (*display).d_has_hstatus == 2 as libc::c_int
    {
        Msg(0 as libc::c_int, b"%s\0" as *const u8 as *const libc::c_char, str);
    }
}
pub unsafe extern "C" fn RefreshHStatus() {
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut extrabytes: libc::c_int = (strlen(hstatusstring))
        .wrapping_sub(
            strlen_onscreen(hstatusstring as *mut libc::c_uchar, 0 as *mut libc::c_uchar)
                as libc::c_ulong,
        ) as libc::c_int;
    evdeq(&mut (*display).d_hstatusev);
    if (*display).d_status == 2 as libc::c_int {
        return;
    }
    buf = MakeWinMsgEv(
        hstatusstring,
        (*display).d_fore,
        '%' as i32,
        if (*display).d_tcs[75 as libc::c_int as usize].flg != 0
            && (*display).d_has_hstatus == 3 as libc::c_int
            && (*display).d_tcs[76 as libc::c_int as usize].num > 0 as libc::c_int
        {
            (*display).d_tcs[76 as libc::c_int as usize].num
        } else {
            (*display).d_width
                - ((*display).d_tcs[87 as libc::c_int as usize].flg == 0) as libc::c_int
                + extrabytes
        },
        &mut (*display).d_hstatusev,
        0 as libc::c_int,
    );
    if !buf.is_null() && *buf as libc::c_int != 0 {
        ShowHStatus(buf);
        if (*display).d_has_hstatus != 0 as libc::c_int
            && (*display).d_hstatusev.timeout.tv_sec != 0
        {
            evenq(&mut (*display).d_hstatusev);
        }
    } else {
        ShowHStatus(0 as *mut libc::c_char);
    };
}
pub unsafe extern "C" fn RefreshAll(mut isblank: libc::c_int) {
    let mut cv: *mut canvas = 0 as *mut canvas;
    cv = (*display).d_cvlist;
    while !cv.is_null() {
        let mut olddisplay: *mut display = display;
        let mut oldflayer: *mut layer = flayer;
        let mut l: *mut layer = (*cv).c_layer;
        let mut cvlist: *mut canvas = (*l).l_cvlist;
        let mut cvlnext: *mut canvas = (*cv).c_lnext;
        flayer = l;
        (*l).l_cvlist = cv;
        (*cv).c_lnext = 0 as *mut canvas;
        (Some(((*(*flayer).l_layfn).lf_LayRedisplayLine).unwrap()))
            .unwrap()(
            -(1 as libc::c_int),
            -(1 as libc::c_int),
            -(1 as libc::c_int),
            isblank,
        );
        flayer = oldflayer;
        (*l).l_cvlist = cvlist;
        (*cv).c_lnext = cvlnext;
        display = olddisplay;
        display = (*cv).c_display;
        cv = (*cv).c_next;
    }
    RefreshArea(
        0 as libc::c_int,
        0 as libc::c_int,
        (*display).d_width - 1 as libc::c_int,
        (*display).d_height - 1 as libc::c_int,
        isblank,
    );
}
pub unsafe extern "C" fn RefreshArea(
    mut xs: libc::c_int,
    mut ys: libc::c_int,
    mut xe: libc::c_int,
    mut ye: libc::c_int,
    mut isblank: libc::c_int,
) {
    let mut y: libc::c_int = 0;
    if isblank == 0 && xs == 0 as libc::c_int
        && xe == (*display).d_width - 1 as libc::c_int
        && ye == (*display).d_height - 1 as libc::c_int
        && (ys == 0 as libc::c_int
            || !((*display).d_tcs[35 as libc::c_int as usize].str_0).is_null())
    {
        ClearArea(xs, ys, xs, xe, xe, ye, 0 as libc::c_int, 0 as libc::c_int);
        isblank = 1 as libc::c_int;
    }
    y = ys;
    while y <= ye {
        RefreshLine(y, xs, xe, isblank);
        y += 1;
        y;
    }
}
pub unsafe extern "C" fn RefreshLine(
    mut y: libc::c_int,
    mut from: libc::c_int,
    mut to: libc::c_int,
    mut isblank: libc::c_int,
) {
    let mut vp: *mut viewport = 0 as *mut viewport;
    let mut lvp: *mut viewport = 0 as *mut viewport;
    let mut cv: *mut canvas = 0 as *mut canvas;
    let mut lcv: *mut canvas = 0 as *mut canvas;
    let mut cvlist: *mut canvas = 0 as *mut canvas;
    let mut cvlnext: *mut canvas = 0 as *mut canvas;
    let mut oldflayer: *mut layer = 0 as *mut layer;
    let mut xx: libc::c_int = 0;
    let mut yy: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut win = 0 as *mut win;
    if (*display).d_status == 1 as libc::c_int
        && y == (*display).d_height - 1 as libc::c_int
    {
        if to >= (*display).d_status_len {
            (*display).d_status_len = to + 1 as libc::c_int;
        }
        return;
    }
    if isblank == 0 as libc::c_int
        && !((*display).d_tcs[37 as libc::c_int as usize].str_0).is_null()
        && to == (*display).d_width - 1 as libc::c_int && from < to
        && (*display).d_status != 2 as libc::c_int
    {
        GotoPos(from, y);
        if (*display).d_tcs[33 as libc::c_int as usize].flg != 0
            || (*display).d_tcs[66 as libc::c_int as usize].flg != 0
        {
            SetRendition(&mut mchar_null);
        }
        AddCStr((*display).d_tcs[37 as libc::c_int as usize].str_0);
        isblank = 1 as libc::c_int;
    }
    if y == (*display).d_height - 1 as libc::c_int
        && (*display).d_has_hstatus == 1 as libc::c_int
        || y == 0 as libc::c_int && (*display).d_has_hstatus == 4 as libc::c_int
    {
        RefreshHStatus();
        return;
    }
    while from <= to {
        lcv = 0 as *mut canvas;
        lvp = 0 as *mut viewport;
        cv = (*display).d_cvlist;
        while !cv.is_null() {
            if y == (*cv).c_ye + 1 as libc::c_int && from >= (*cv).c_xs
                && from <= (*cv).c_xe
            {
                let mut extrabytes: libc::c_int = (strlen(captionstring))
                    .wrapping_sub(
                        strlen_onscreen(
                            captionstring as *mut libc::c_uchar,
                            0 as *mut libc::c_uchar,
                        ) as libc::c_ulong,
                    ) as libc::c_int;
                p = (*(*(*cv).c_layer).l_bottom).l_data as *mut win;
                buf = MakeWinMsgEv(
                    captionstring,
                    p,
                    '%' as i32,
                    (*cv).c_xe - (*cv).c_xs
                        + (((*cv).c_xe + 1 as libc::c_int) < (*display).d_width
                            || (*display).d_tcs[87 as libc::c_int as usize].flg != 0)
                            as libc::c_int + extrabytes,
                    &mut (*cv).c_captev,
                    0 as libc::c_int,
                );
                if (*cv).c_captev.timeout.tv_sec != 0 {
                    evenq(&mut (*cv).c_captev);
                }
                xx = if to > (*cv).c_xe { (*cv).c_xe } else { to };
                l = strlen(buf) as libc::c_int;
                GotoPos(from, y);
                SetRendition(&mut mchar_so);
                if l > xx - (*cv).c_xs + 1 as libc::c_int {
                    l = xx - (*cv).c_xs + 1 as libc::c_int;
                }
                l = PrePutWinMsg(buf, from - (*cv).c_xs, l + extrabytes);
                from = (*cv).c_xs + l;
                while from <= xx {
                    PUTCHARLP(' ' as i32);
                    from += 1;
                    from;
                }
                break;
            } else if from == (*cv).c_xe + 1 as libc::c_int && y >= (*cv).c_ys
                && y <= (*cv).c_ye + 1 as libc::c_int
            {
                GotoPos(from, y);
                SetRendition(&mut mchar_so);
                PUTCHARLP(' ' as i32);
                from += 1;
                from;
                break;
            } else {
                if !(y < (*cv).c_ys || y > (*cv).c_ye || to < (*cv).c_xs
                    || from > (*cv).c_xe)
                {
                    vp = (*cv).c_vplist;
                    while !vp.is_null() {
                        if y >= (*vp).v_ys && y <= (*vp).v_ye && from <= (*vp).v_xe
                            && to >= (*vp).v_xs
                            && (lvp.is_null() || (*lvp).v_xs > (*vp).v_xs)
                        {
                            lcv = cv;
                            lvp = vp;
                        }
                        vp = (*vp).v_next;
                    }
                }
                cv = (*cv).c_next;
            }
        }
        if !cv.is_null() {
            continue;
        }
        if lvp.is_null() {
            break;
        }
        if from < (*lvp).v_xs {
            if isblank == 0 {
                DisplayLine(
                    &mut mline_null,
                    &mut mline_blank,
                    y,
                    from,
                    (*lvp).v_xs - 1 as libc::c_int,
                );
            }
            from = (*lvp).v_xs;
        }
        yy = y - (*lvp).v_yoff;
        xx = if to < (*lvp).v_xe { to } else { (*lvp).v_xe };
        if !((*lcv).c_layer).is_null()
            && (*lcv).c_xoff + (*(*lcv).c_layer).l_width == from
        {
            GotoPos(from, y);
            SetRendition(&mut mchar_blank);
            PUTCHARLP('|' as i32);
            from += 1;
            from;
        }
        if !((*lcv).c_layer).is_null() && yy == (*(*lcv).c_layer).l_height {
            GotoPos(from, y);
            SetRendition(&mut mchar_blank);
            while from <= (*lvp).v_xe && from - (*lvp).v_xoff < (*(*lcv).c_layer).l_width
            {
                PUTCHARLP('-' as i32);
                from += 1;
                from;
            }
            if from >= (*lvp).v_xe + 1 as libc::c_int {
                continue;
            }
        }
        if ((*lcv).c_layer).is_null() || yy >= (*(*lcv).c_layer).l_height
            || from - (*lvp).v_xoff >= (*(*lcv).c_layer).l_width
        {
            if isblank == 0 {
                DisplayLine(&mut mline_null, &mut mline_blank, y, from, (*lvp).v_xe);
            }
            from = (*lvp).v_xe + 1 as libc::c_int;
        } else {
            if xx - (*lvp).v_xoff >= (*(*lcv).c_layer).l_width {
                xx = (*(*lcv).c_layer).l_width + (*lvp).v_xoff - 1 as libc::c_int;
            }
            oldflayer = flayer;
            flayer = (*lcv).c_layer;
            cvlist = (*flayer).l_cvlist;
            cvlnext = (*lcv).c_lnext;
            (*flayer).l_cvlist = lcv;
            (*lcv).c_lnext = 0 as *mut canvas;
            (Some(((*(*flayer).l_layfn).lf_LayRedisplayLine).unwrap()))
                .unwrap()(yy, from - (*lvp).v_xoff, xx - (*lvp).v_xoff, isblank);
            (*flayer).l_cvlist = cvlist;
            (*lcv).c_lnext = cvlnext;
            flayer = oldflayer;
            from = xx + 1 as libc::c_int;
        }
    }
    if isblank == 0 && from <= to {
        DisplayLine(&mut mline_null, &mut mline_blank, y, from, to);
    }
}
unsafe extern "C" fn WriteLP(mut x2: libc::c_int, mut y2: libc::c_int) {
    let mut oldrend: mchar = mchar {
        image: 0,
        attr: 0,
        font: 0,
        fontx: 0,
        color: 0,
        mbcs: 0,
    };
    oldrend = (*display).d_rend;
    if (*display).d_lpchar.mbcs != 0 {
        if x2 > 0 as libc::c_int {
            x2 -= 1;
            x2;
        } else {
            (*display).d_lpchar = mchar_blank;
        }
    }
    GotoPos(x2, y2);
    SetRendition(&mut (*display).d_lpchar);
    PUTCHAR((*display).d_lpchar.image as libc::c_int);
    if (*display).d_lpchar.mbcs != 0 {
        PUTCHAR((*display).d_lpchar.mbcs as libc::c_int);
    }
    (*display).d_lp_missing = 0 as libc::c_int;
    SetRendition(&mut oldrend);
}
pub unsafe extern "C" fn ClearLine(
    mut oml: *mut mline,
    mut y: libc::c_int,
    mut from: libc::c_int,
    mut to: libc::c_int,
    mut bce: libc::c_int,
) {
    let mut x: libc::c_int = 0;
    let mut bcechar: mchar = mchar {
        image: 0,
        attr: 0,
        font: 0,
        fontx: 0,
        color: 0,
        mbcs: 0,
    };
    if (*display).d_tcs[33 as libc::c_int as usize].flg != 0 {
        SetRendition(&mut mchar_null);
    }
    if (*display).d_tcs[66 as libc::c_int as usize].flg != 0 {
        SetBackColor(bce);
    }
    if from == 0 as libc::c_int
        && !((*display).d_tcs[38 as libc::c_int as usize].str_0).is_null()
        && (to != (*display).d_width - 1 as libc::c_int
            || (*display).d_x == to && (*display).d_y == y)
        && (bce == 0 || (*display).d_tcs[66 as libc::c_int as usize].flg != 0)
    {
        GotoPos(to, y);
        AddCStr((*display).d_tcs[38 as libc::c_int as usize].str_0);
        return;
    }
    if to == (*display).d_width - 1 as libc::c_int
        && !((*display).d_tcs[37 as libc::c_int as usize].str_0).is_null()
        && (bce == 0 || (*display).d_tcs[66 as libc::c_int as usize].flg != 0)
    {
        GotoPos(from, y);
        AddCStr((*display).d_tcs[37 as libc::c_int as usize].str_0);
        return;
    }
    if oml.is_null() {
        oml = &mut mline_null;
    }
    if bce == 0 {
        DisplayLine(oml, &mut mline_blank, y, from, to);
        return;
    }
    bcechar = mchar_null;
    bcechar
        .color = (bcechar.color as libc::c_int & 0xf as libc::c_int
        | bce << 4 as libc::c_int & 0xf0 as libc::c_int) as libc::c_uchar;
    bcechar
        .attr = ((bcechar.attr as libc::c_int | (1 as libc::c_int) << 7 as libc::c_int)
        ^ (if bce & 0x100 as libc::c_int != 0 {
            0 as libc::c_int
        } else {
            (1 as libc::c_int) << 7 as libc::c_int
        })) as libc::c_uchar;
    x = from;
    while x <= to {
        *(mline_old.image).offset(x as isize) = bcechar.image;
        *(mline_old.attr).offset(x as isize) = bcechar.attr;
        *(mline_old.font).offset(x as isize) = bcechar.font;
        *(mline_old.fontx).offset(x as isize) = bcechar.fontx;
        *(mline_old.color).offset(x as isize) = bcechar.color;
        x += 1;
        x;
    }
    DisplayLine(oml, &mut mline_old, y, from, to);
}
pub unsafe extern "C" fn DisplayLine(
    mut oml: *mut mline,
    mut ml: *mut mline,
    mut y: libc::c_int,
    mut from: libc::c_int,
    mut to: libc::c_int,
) {
    let mut x: libc::c_int = 0;
    let mut last2flag: libc::c_int = 0 as libc::c_int;
    let mut delete_lp: libc::c_int = 0 as libc::c_int;
    if (*display).d_tcs[87 as libc::c_int as usize].flg == 0 && y == (*display).d_bot
        && to == (*display).d_width - 1 as libc::c_int
    {
        if (*display).d_lp_missing != 0
            || !(*((*oml).image).offset(to as isize) as libc::c_int
                == *((*ml).image).offset(to as isize) as libc::c_int
                && *((*oml).attr).offset(to as isize) as libc::c_int
                    == *((*ml).attr).offset(to as isize) as libc::c_int
                && *((*oml).font).offset(to as isize) as libc::c_int
                    == *((*ml).font).offset(to as isize) as libc::c_int
                && *((*oml).fontx).offset(to as isize) as libc::c_int
                    == *((*ml).fontx).offset(to as isize) as libc::c_int
                && *((*oml).color).offset(to as isize) as libc::c_int
                    == *((*ml).color).offset(to as isize) as libc::c_int)
        {
            if (!((*display).d_tcs[29 as libc::c_int as usize].str_0).is_null()
                || !((*display).d_tcs[27 as libc::c_int as usize].str_0).is_null())
                && from < to
                && (if (*display).d_encoding == 8 as libc::c_int {
                    (*((*ml).font).offset((to + 1 as libc::c_int) as isize)
                        as libc::c_int == 0xff as libc::c_int
                        && *((*ml).image).offset((to + 1 as libc::c_int) as isize)
                            as libc::c_int == 0xff as libc::c_int) as libc::c_int
                } else {
                    (*((*ml).font).offset(to as isize) as libc::c_int
                        & 0x1f as libc::c_int != 0 as libc::c_int
                        && *((*ml).font).offset(to as isize) as libc::c_int
                            & 0xe0 as libc::c_int == 0 as libc::c_int) as libc::c_int
                }) == 0
            {
                last2flag = 1 as libc::c_int;
                (*display).d_lp_missing = 0 as libc::c_int;
                to -= 1;
                to;
            } else {
                delete_lp = (!(mchar_blank.image as libc::c_int
                    == *((*oml).image).offset(to as isize) as libc::c_int
                    && mchar_blank.attr as libc::c_int
                        == *((*oml).attr).offset(to as isize) as libc::c_int
                    && mchar_blank.font as libc::c_int
                        == *((*oml).font).offset(to as isize) as libc::c_int
                    && mchar_blank.fontx as libc::c_int
                        == *((*oml).fontx).offset(to as isize) as libc::c_int
                    && mchar_blank.color as libc::c_int
                        == *((*oml).color).offset(to as isize) as libc::c_int)
                    && (!((*display).d_tcs[37 as libc::c_int as usize].str_0).is_null()
                        || !((*display).d_tcs[31 as libc::c_int as usize].str_0)
                            .is_null()
                        || !((*display).d_tcs[32 as libc::c_int as usize].str_0)
                            .is_null())) as libc::c_int;
                (*display)
                    .d_lp_missing = !(mchar_blank.image as libc::c_int
                    == *((*ml).image).offset(to as isize) as libc::c_int
                    && mchar_blank.attr as libc::c_int
                        == *((*ml).attr).offset(to as isize) as libc::c_int
                    && mchar_blank.font as libc::c_int
                        == *((*ml).font).offset(to as isize) as libc::c_int
                    && mchar_blank.fontx as libc::c_int
                        == *((*ml).fontx).offset(to as isize) as libc::c_int
                    && mchar_blank.color as libc::c_int
                        == *((*ml).color).offset(to as isize) as libc::c_int)
                    as libc::c_int;
                (*display).d_lpchar.image = *((*ml).image).offset(to as isize);
                (*display).d_lpchar.attr = *((*ml).attr).offset(to as isize);
                (*display).d_lpchar.font = *((*ml).font).offset(to as isize);
                (*display).d_lpchar.fontx = *((*ml).fontx).offset(to as isize);
                (*display).d_lpchar.color = *((*ml).color).offset(to as isize);
                (*display).d_lpchar.mbcs = 0 as libc::c_int as libc::c_uchar;
            }
        }
        to -= 1;
        to;
    }
    if (*display).d_mbcs != 0 {
        SetRenditionMline(ml, from);
        PUTCHAR(*((*ml).image).offset(from as isize) as libc::c_int);
        from += 1;
        from;
    }
    let mut current_block_47: u64;
    x = from;
    while x <= to {
        if !ml.is_null()
            && (x < to || x != (*display).d_width - 1 as libc::c_int
                || *((*ml).image).offset((x + 1 as libc::c_int) as isize) as libc::c_int
                    != 0)
        {
            if *((*oml).image).offset(x as isize) as libc::c_int
                == *((*ml).image).offset(x as isize) as libc::c_int
                && *((*oml).attr).offset(x as isize) as libc::c_int
                    == *((*ml).attr).offset(x as isize) as libc::c_int
                && *((*oml).font).offset(x as isize) as libc::c_int
                    == *((*ml).font).offset(x as isize) as libc::c_int
                && *((*oml).fontx).offset(x as isize) as libc::c_int
                    == *((*ml).fontx).offset(x as isize) as libc::c_int
                && *((*oml).color).offset(x as isize) as libc::c_int
                    == *((*ml).color).offset(x as isize) as libc::c_int
            {
                current_block_47 = 8693738493027456495;
            } else {
                current_block_47 = 14136749492126903395;
            }
        } else {
            current_block_47 = 14136749492126903395;
        }
        match current_block_47 {
            14136749492126903395 => {
                GotoPos(x, y);
                if if (*display).d_encoding == 8 as libc::c_int {
                    (*((*ml).font).offset(x as isize) as libc::c_int
                        == 0xff as libc::c_int
                        && *((*ml).image).offset(x as isize) as libc::c_int
                            == 0xff as libc::c_int) as libc::c_int
                } else {
                    (*((*ml).font).offset(x as isize) as libc::c_int
                        & 0xe0 as libc::c_int == 0x80 as libc::c_int) as libc::c_int
                } != 0
                {
                    x -= 1;
                    x;
                    GotoPos(x, y);
                }
                if x == to
                    && (if (*display).d_encoding == 8 as libc::c_int {
                        (*((*ml).font).offset((x + 1 as libc::c_int) as isize)
                            as libc::c_int == 0xff as libc::c_int
                            && *((*ml).image).offset((x + 1 as libc::c_int) as isize)
                                as libc::c_int == 0xff as libc::c_int) as libc::c_int
                    } else {
                        (*((*ml).font).offset(x as isize) as libc::c_int
                            & 0x1f as libc::c_int != 0 as libc::c_int
                            && *((*ml).font).offset(x as isize) as libc::c_int
                                & 0xe0 as libc::c_int == 0 as libc::c_int) as libc::c_int
                    }) != 0
                {
                    break;
                }
                SetRenditionMline(ml, x);
                PUTCHAR(*((*ml).image).offset(x as isize) as libc::c_int);
                if if (*display).d_encoding == 8 as libc::c_int {
                    (*((*ml).font).offset((x + 1 as libc::c_int) as isize) as libc::c_int
                        == 0xff as libc::c_int
                        && *((*ml).image).offset((x + 1 as libc::c_int) as isize)
                            as libc::c_int == 0xff as libc::c_int) as libc::c_int
                } else {
                    (*((*ml).font).offset(x as isize) as libc::c_int
                        & 0x1f as libc::c_int != 0 as libc::c_int
                        && *((*ml).font).offset(x as isize) as libc::c_int
                            & 0xe0 as libc::c_int == 0 as libc::c_int) as libc::c_int
                } != 0
                {
                    x += 1;
                    PUTCHAR(*((*ml).image).offset(x as isize) as libc::c_int);
                }
            }
            _ => {}
        }
        x += 1;
        x;
    }
    if last2flag != 0 {
        GotoPos(x, y);
        SetRenditionMline(ml, x + 1 as libc::c_int);
        PUTCHAR(*((*ml).image).offset((x + 1 as libc::c_int) as isize) as libc::c_int);
        GotoPos(x, y);
        SetRenditionMline(ml, x);
        INSERTCHAR(*((*ml).image).offset(x as isize) as libc::c_int);
    } else if delete_lp != 0 {
        if (*display).d_tcs[33 as libc::c_int as usize].flg != 0 {
            SetRendition(&mut mchar_null);
        }
        if !((*display).d_tcs[31 as libc::c_int as usize].str_0).is_null() {
            AddCStr((*display).d_tcs[31 as libc::c_int as usize].str_0);
        } else if !((*display).d_tcs[32 as libc::c_int as usize].str_0).is_null() {
            AddCStr2(
                (*display).d_tcs[32 as libc::c_int as usize].str_0,
                1 as libc::c_int,
            );
        } else if !((*display).d_tcs[37 as libc::c_int as usize].str_0).is_null() {
            AddCStr((*display).d_tcs[37 as libc::c_int as usize].str_0);
        }
    }
}
pub unsafe extern "C" fn PutChar(
    mut c: *mut mchar,
    mut x: libc::c_int,
    mut y: libc::c_int,
) {
    GotoPos(x, y);
    SetRendition(c);
    PUTCHARLP((*c).image as libc::c_int);
    if (*c).mbcs != 0 {
        if (*display).d_encoding == 8 as libc::c_int {
            (*display).d_rend.font = 0 as libc::c_int as libc::c_uchar;
        }
        PUTCHARLP((*c).mbcs as libc::c_int);
    }
}
pub unsafe extern "C" fn InsChar(
    mut c: *mut mchar,
    mut x: libc::c_int,
    mut xe: libc::c_int,
    mut y: libc::c_int,
    mut oml: *mut mline,
) {
    GotoPos(x, y);
    if y == (*display).d_bot && (*display).d_tcs[87 as libc::c_int as usize].flg == 0 {
        if x == (*display).d_width - 1 as libc::c_int {
            (*display).d_lp_missing = 1 as libc::c_int;
            (*display).d_lpchar = *c;
            return;
        }
        if xe == (*display).d_width - 1 as libc::c_int {
            (*display).d_lp_missing = 0 as libc::c_int;
        }
    }
    if x == xe {
        SetRendition(c);
        PUTCHARLP((*c).image as libc::c_int);
        return;
    }
    if !(!((*display).d_tcs[29 as libc::c_int as usize].str_0).is_null()
        || !((*display).d_tcs[30 as libc::c_int as usize].str_0).is_null()
        || !((*display).d_tcs[27 as libc::c_int as usize].str_0).is_null())
        || xe != (*display).d_width - 1 as libc::c_int
    {
        RefreshLine(y, x, xe, 0 as libc::c_int);
        GotoPos(x + 1 as libc::c_int, y);
        return;
    }
    InsertMode(1 as libc::c_int);
    if (*display).d_insert == 0 {
        if (*c).mbcs as libc::c_int != 0
            && !((*display).d_tcs[29 as libc::c_int as usize].str_0).is_null()
        {
            AddCStr((*display).d_tcs[29 as libc::c_int as usize].str_0);
        }
        if !((*display).d_tcs[29 as libc::c_int as usize].str_0).is_null() {
            AddCStr((*display).d_tcs[29 as libc::c_int as usize].str_0);
        } else {
            AddCStr2(
                (*display).d_tcs[30 as libc::c_int as usize].str_0,
                if (*c).mbcs as libc::c_int != 0 {
                    2 as libc::c_int
                } else {
                    1 as libc::c_int
                },
            );
        }
    }
    SetRendition(c);
    RAW_PUTCHAR((*c).image as libc::c_int);
    if (*c).mbcs != 0 {
        if (*display).d_encoding == 8 as libc::c_int {
            (*display).d_rend.font = 0 as libc::c_int as libc::c_uchar;
        }
        if (*display).d_x == (*display).d_width - 1 as libc::c_int {
            PUTCHARLP((*c).mbcs as libc::c_int);
        } else {
            RAW_PUTCHAR((*c).mbcs as libc::c_int);
        }
    }
}
pub unsafe extern "C" fn WrapChar(
    mut c: *mut mchar,
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut xs: libc::c_int,
    mut ys: libc::c_int,
    mut xe: libc::c_int,
    mut ye: libc::c_int,
    mut ins: libc::c_int,
) {
    let mut bce: libc::c_int = 0;
    bce = ((*c).color as libc::c_int & 0xf0 as libc::c_int) >> 4 as libc::c_int
        | (if (*c).attr as libc::c_int & (1 as libc::c_int) << 7 as libc::c_int != 0 {
            0x100 as libc::c_int
        } else {
            0 as libc::c_int
        });
    if xs != 0 as libc::c_int || x != (*display).d_width
        || (*display).d_tcs[83 as libc::c_int as usize].flg == 0
    {
        if y == ye {
            ScrollV(xs, ys, xe, ye, 1 as libc::c_int, bce);
        } else if y < (*display).d_height - 1 as libc::c_int {
            y += 1;
            y;
        }
        if ins != 0 {
            InsChar(c, xs, xe, y, 0 as *mut mline);
        } else {
            PutChar(c, xs, y);
        }
        return;
    }
    if y == ye {
        ChangeScrollRegion(ys, ye);
        if (*display).d_bot != y || (*display).d_x != (*display).d_width
            || bce == 0 && (*display).d_tcs[66 as libc::c_int as usize].flg == 0
        {
            ScrollV(xs, ys, xe, ye, 1 as libc::c_int, bce);
            y -= 1;
            y;
        }
    } else if y == (*display).d_bot {
        ChangeScrollRegion(0 as libc::c_int, (*display).d_height - 1 as libc::c_int);
    }
    if (*display).d_x != (*display).d_width || (*display).d_y != y {
        if (*display).d_tcs[87 as libc::c_int as usize].flg != 0 && y >= 0 as libc::c_int
        {
            RefreshLine(
                y,
                (*display).d_width - 1 as libc::c_int,
                (*display).d_width - 1 as libc::c_int,
                0 as libc::c_int,
            );
        }
        if (*display).d_x != (*display).d_width || (*display).d_y != y {
            if y == ye {
                ScrollV(xs, ys, xe, ye, 1 as libc::c_int, bce);
            }
            GotoPos(
                xs,
                if y == ye || y == (*display).d_height - 1 as libc::c_int {
                    y
                } else {
                    y + 1 as libc::c_int
                },
            );
        }
    }
    if y != ye && y < (*display).d_height - 1 as libc::c_int {
        y += 1;
        y;
    }
    if ins != (*display).d_insert {
        InsertMode(ins);
    }
    if ins != 0 && (*display).d_insert == 0 {
        InsChar(c, 0 as libc::c_int, xe, y, 0 as *mut mline);
        return;
    }
    (*display).d_y = y;
    (*display).d_x = 0 as libc::c_int;
    SetRendition(c);
    RAW_PUTCHAR((*c).image as libc::c_int);
    if (*c).mbcs != 0 {
        if (*display).d_encoding == 8 as libc::c_int {
            (*display).d_rend.font = 0 as libc::c_int as libc::c_uchar;
        }
        RAW_PUTCHAR((*c).mbcs as libc::c_int);
    }
}
pub unsafe extern "C" fn ResizeDisplay(
    mut wi: libc::c_int,
    mut he: libc::c_int,
) -> libc::c_int {
    if (*display).d_width == wi && (*display).d_height == he {
        return 0 as libc::c_int;
    }
    if (*display).d_width != wi
        && ((*display).d_height == he
            || ((*display).d_tcs[44 as libc::c_int as usize].str_0).is_null())
        && !((*display).d_tcs[45 as libc::c_int as usize].str_0).is_null()
        && (wi == Z0width || wi == Z1width)
    {
        AddCStr(
            if wi == Z0width {
                (*display).d_tcs[45 as libc::c_int as usize].str_0
            } else {
                (*display).d_tcs[46 as libc::c_int as usize].str_0
            },
        );
        ChangeScreenSize(wi, (*display).d_height, 0 as libc::c_int);
        return if he == (*display).d_height {
            0 as libc::c_int
        } else {
            -(1 as libc::c_int)
        };
    }
    if !((*display).d_tcs[44 as libc::c_int as usize].str_0).is_null() {
        AddCStr(tgoto((*display).d_tcs[44 as libc::c_int as usize].str_0, wi, he));
        ChangeScreenSize(wi, he, 0 as libc::c_int);
        return 0 as libc::c_int;
    }
    return -(1 as libc::c_int);
}
pub unsafe extern "C" fn ChangeScrollRegion(
    mut newtop: libc::c_int,
    mut newbot: libc::c_int,
) {
    if display.is_null() {
        return;
    }
    if newtop == newbot {
        return;
    }
    if newtop == -(1 as libc::c_int) {
        newtop = 0 as libc::c_int;
    }
    if newbot == -(1 as libc::c_int) {
        newbot = (*display).d_height - 1 as libc::c_int;
    }
    if ((*display).d_tcs[18 as libc::c_int as usize].str_0).is_null() {
        (*display).d_top = 0 as libc::c_int;
        (*display).d_bot = (*display).d_height - 1 as libc::c_int;
        return;
    }
    if (*display).d_top == newtop && (*display).d_bot == newbot {
        return;
    }
    AddCStr(tgoto((*display).d_tcs[18 as libc::c_int as usize].str_0, newbot, newtop));
    (*display).d_top = newtop;
    (*display).d_bot = newbot;
    (*display).d_x = -(1 as libc::c_int);
    (*display).d_y = (*display).d_x;
}
pub unsafe extern "C" fn AddStr(mut str: *mut libc::c_char) {
    let mut c: libc::c_char = 0;
    if (*display).d_encoding == 8 as libc::c_int {
        loop {
            let fresh28 = str;
            str = str.offset(1);
            c = *fresh28;
            if !(c != 0) {
                break;
            }
            AddUtf8(c as libc::c_uchar as libc::c_int);
        }
        return;
    }
    loop {
        let fresh29 = str;
        str = str.offset(1);
        c = *fresh29;
        if !(c != 0) {
            break;
        }
        (*display).d_obuffree -= 1;
        if (*display).d_obuffree <= 0 as libc::c_int {
            Resize_obuf();
        }
        let fresh30 = (*display).d_obufp;
        (*display).d_obufp = ((*display).d_obufp).offset(1);
        *fresh30 = c;
    };
}
pub unsafe extern "C" fn AddStrn(mut str: *mut libc::c_char, mut n: libc::c_int) {
    let mut c: libc::c_char = 0;
    if (*display).d_encoding == 8 as libc::c_int {
        loop {
            let fresh31 = str;
            str = str.offset(1);
            c = *fresh31;
            if !(c as libc::c_int != 0
                && {
                    let fresh32 = n;
                    n = n - 1;
                    fresh32 > 0 as libc::c_int
                })
            {
                break;
            }
            AddUtf8(c as libc::c_uchar as libc::c_int);
        }
    } else {
        loop {
            let fresh33 = str;
            str = str.offset(1);
            c = *fresh33;
            if !(c as libc::c_int != 0
                && {
                    let fresh34 = n;
                    n = n - 1;
                    fresh34 > 0 as libc::c_int
                })
            {
                break;
            }
            (*display).d_obuffree -= 1;
            if (*display).d_obuffree <= 0 as libc::c_int {
                Resize_obuf();
            }
            let fresh35 = (*display).d_obufp;
            (*display).d_obufp = ((*display).d_obufp).offset(1);
            *fresh35 = c;
        }
    }
    loop {
        let fresh36 = n;
        n = n - 1;
        if !(fresh36 > 0 as libc::c_int) {
            break;
        }
        (*display).d_obuffree -= 1;
        if (*display).d_obuffree <= 0 as libc::c_int {
            Resize_obuf();
        }
        let fresh37 = (*display).d_obufp;
        (*display).d_obufp = ((*display).d_obufp).offset(1);
        *fresh37 = ' ' as i32 as libc::c_char;
    };
}
pub unsafe extern "C" fn Flush(mut progress: libc::c_int) {
    let mut l: libc::c_int = 0;
    let mut wr: libc::c_int = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    l = ((*display).d_obufp).offset_from((*display).d_obuf) as libc::c_long
        as libc::c_int;
    if l == 0 as libc::c_int {
        return;
    }
    if (*display).d_userfd < 0 as libc::c_int {
        (*display).d_obuffree += l;
        (*display).d_obufp = (*display).d_obuf;
        return;
    }
    p = (*display).d_obuf;
    if progress == 0 {
        fcntl((*display).d_userfd, 4 as libc::c_int, 0 as libc::c_int) != 0;
    }
    while l != 0 {
        if progress != 0 {
            let mut w: fd_set = fd_set { fds_bits: [0; 16] };
            let mut __d0: libc::c_int = 0;
            let mut __d1: libc::c_int = 0;
            let fresh38 = &mut __d0;
            let fresh39;
            let fresh40 = (::std::mem::size_of::<fd_set>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<__fd_mask>() as libc::c_ulong);
            let fresh41 = &mut __d1;
            let fresh42;
            let fresh43 = &mut *(w.fds_bits)
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize) as *mut __fd_mask;
            asm!(
                "cld; rep; stosq", inlateout("cx")
                c2rust_asm_casts::AsmCast::cast_in(fresh38, fresh40) => fresh39,
                inlateout("di") c2rust_asm_casts::AsmCast::cast_in(fresh41, fresh43) =>
                fresh42, inlateout("ax") 0 as libc::c_int => _, options(preserves_flags,
                att_syntax)
            );
            c2rust_asm_casts::AsmCast::cast_out(fresh38, fresh40, fresh39);
            c2rust_asm_casts::AsmCast::cast_out(fresh41, fresh43, fresh42);
            w
                .fds_bits[((*display).d_userfd
                / (8 as libc::c_int
                    * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                        as libc::c_int)) as usize]
                |= ((1 as libc::c_ulong)
                    << (*display).d_userfd
                        % (8 as libc::c_int
                            * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                as libc::c_int)) as __fd_mask;
            let mut t: timeval = timeval { tv_sec: 0, tv_usec: 0 };
            t.tv_sec = progress as __time_t;
            t.tv_usec = 0 as libc::c_int as __suseconds_t;
            wr = select(
                1024 as libc::c_int,
                0 as *mut fd_set,
                &mut w,
                0 as *mut fd_set,
                &mut t,
            );
            if wr == -(1 as libc::c_int) {
                if *__errno_location() == 4 as libc::c_int {
                    continue;
                } else {
                    break;
                }
            } else if wr == 0 as libc::c_int {
                break;
            }
        }
        wr = write((*display).d_userfd, p as *const libc::c_void, l as size_t)
            as libc::c_int;
        if wr <= 0 as libc::c_int {
            if !(*__errno_location() == 4 as libc::c_int) {
                break;
            }
        } else {
            (*display).d_obuffree += wr;
            p = p.offset(wr as isize);
            l -= wr;
        }
    }
    l != 0;
    (*display).d_obuffree += l;
    (*display).d_obufp = (*display).d_obuf;
    if progress == 0 {
        fcntl((*display).d_userfd, 4 as libc::c_int, 0o4000 as libc::c_int) != 0;
    }
    if (*display).d_blocked == 1 as libc::c_int {
        (*display).d_blocked = 0 as libc::c_int;
    }
    (*display).d_blocked_fuzz = 0 as libc::c_int;
}
pub unsafe extern "C" fn freetty() {
    if (*display).d_userfd >= 0 as libc::c_int {
        close((*display).d_userfd);
    }
    (*display).d_userfd = -(1 as libc::c_int);
    (*display).d_obufp = 0 as *mut libc::c_char;
    (*display).d_obuffree = 0 as libc::c_int;
    if !((*display).d_obuf).is_null() {
        free((*display).d_obuf as *mut libc::c_void);
    }
    (*display).d_obuf = 0 as *mut libc::c_char;
    (*display).d_obuflen = 0 as libc::c_int;
    (*display).d_obuflenmax = -(*display).d_obufmax;
    (*display).d_blocked = 0 as libc::c_int;
    (*display).d_blocked_fuzz = 0 as libc::c_int;
}
pub unsafe extern "C" fn Resize_obuf() {
    let mut ind: libc::c_int = 0;
    if (*display).d_status_obuffree >= 0 as libc::c_int {
        RemoveStatusMinWait();
        (*display).d_obuffree -= 1;
        if (*display).d_obuffree > 0 as libc::c_int {
            return;
        }
    }
    if (*display).d_obuflen != 0 && !((*display).d_obuf).is_null() {
        ind = ((*display).d_obufp).offset_from((*display).d_obuf) as libc::c_long
            as libc::c_int;
        (*display).d_obuflen += 4096 as libc::c_int;
        (*display).d_obuffree += 4096 as libc::c_int;
        (*display)
            .d_obuf = realloc(
            (*display).d_obuf as *mut libc::c_void,
            (*display).d_obuflen as libc::c_ulong,
        ) as *mut libc::c_char;
    } else {
        ind = 0 as libc::c_int;
        (*display).d_obuflen = 4096 as libc::c_int;
        (*display).d_obuffree = 4096 as libc::c_int;
        (*display)
            .d_obuf = malloc((*display).d_obuflen as libc::c_ulong) as *mut libc::c_char;
    }
    if ((*display).d_obuf).is_null() {
        Panic(0 as libc::c_int, b"Out of memory\0" as *const u8 as *const libc::c_char);
    }
    (*display).d_obufp = ((*display).d_obuf).offset(ind as isize);
    (*display).d_obuflenmax = (*display).d_obuflen - (*display).d_obufmax;
}
pub unsafe extern "C" fn DisplaySleep1000(mut n: libc::c_int, mut eat: libc::c_int) {
    let mut buf: libc::c_char = 0;
    let mut r: fd_set = fd_set { fds_bits: [0; 16] };
    let mut t: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    if n <= 0 as libc::c_int {
        return;
    }
    if display.is_null() {
        sleep1000(n);
        return;
    }
    t.tv_usec = (n % 1000 as libc::c_int * 1000 as libc::c_int) as __suseconds_t;
    t.tv_sec = (n / 1000 as libc::c_int) as __time_t;
    let mut __d0: libc::c_int = 0;
    let mut __d1: libc::c_int = 0;
    let fresh44 = &mut __d0;
    let fresh45;
    let fresh46 = (::std::mem::size_of::<fd_set>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<__fd_mask>() as libc::c_ulong);
    let fresh47 = &mut __d1;
    let fresh48;
    let fresh49 = &mut *(r.fds_bits).as_mut_ptr().offset(0 as libc::c_int as isize)
        as *mut __fd_mask;
    asm!(
        "cld; rep; stosq", inlateout("cx") c2rust_asm_casts::AsmCast::cast_in(fresh44,
        fresh46) => fresh45, inlateout("di") c2rust_asm_casts::AsmCast::cast_in(fresh47,
        fresh49) => fresh48, inlateout("ax") 0 as libc::c_int => _,
        options(preserves_flags, att_syntax)
    );
    c2rust_asm_casts::AsmCast::cast_out(fresh44, fresh46, fresh45);
    c2rust_asm_casts::AsmCast::cast_out(fresh47, fresh49, fresh48);
    r
        .fds_bits[((*display).d_userfd
        / (8 as libc::c_int
            * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
        as usize]
        |= ((1 as libc::c_ulong)
            << (*display).d_userfd
                % (8 as libc::c_int
                    * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                        as libc::c_int)) as __fd_mask;
    if select(1024 as libc::c_int, &mut r, 0 as *mut fd_set, 0 as *mut fd_set, &mut t)
        > 0 as libc::c_int
    {
        if eat != 0 {
            read(
                (*display).d_userfd,
                &mut buf as *mut libc::c_char as *mut libc::c_void,
                1 as libc::c_int as size_t,
            );
        }
    }
}
pub unsafe extern "C" fn NukePending() {
    let mut len: libc::c_int = 0;
    let mut oldtop: libc::c_int = (*display).d_top;
    let mut oldbot: libc::c_int = (*display).d_bot;
    let mut oldrend: mchar = mchar {
        image: 0,
        attr: 0,
        font: 0,
        fontx: 0,
        color: 0,
        mbcs: 0,
    };
    let mut oldkeypad: libc::c_int = (*display).d_keypad;
    let mut oldcursorkeys: libc::c_int = (*display).d_cursorkeys;
    let mut oldcurvis: libc::c_int = (*display).d_curvis;
    let mut oldmouse: libc::c_int = (*display).d_mouse;
    let mut oldextmouse: libc::c_int = (*display).d_extmouse;
    oldrend = (*display).d_rend;
    len = ((*display).d_obufp).offset_from((*display).d_obuf) as libc::c_long
        as libc::c_int;
    tcflush((*display).d_userfd, 1 as libc::c_int);
    (*display).d_obufp = (*display).d_obuf;
    (*display).d_obuffree += len;
    (*display).d_bot = -(1 as libc::c_int);
    (*display).d_top = (*display).d_bot;
    AddCStr((*display).d_tcs[39 as libc::c_int as usize].str_0);
    AddCStr((*display).d_tcs[40 as libc::c_int as usize].str_0);
    if !((*display).d_tcs[55 as libc::c_int as usize].str_0).is_null() {
        AddCStr((*display).d_tcs[55 as libc::c_int as usize].str_0);
    } else {
        if (*display).d_hascolor != 0 {
            AddStr(b"\x1B[m\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
        }
        AddCStr((*display).d_tcs[54 as libc::c_int as usize].str_0);
        AddCStr((*display).d_tcs[53 as libc::c_int as usize].str_0);
    }
    if !((*display).d_tcs[27 as libc::c_int as usize].str_0).is_null()
        && strcmp(
            (*display).d_tcs[27 as libc::c_int as usize].str_0,
            (*display).d_tcs[28 as libc::c_int as usize].str_0,
        ) != 0
    {
        AddCStr((*display).d_tcs[28 as libc::c_int as usize].str_0);
    }
    (*display).d_insert = 0 as libc::c_int;
    if !((*display).d_tcs[69 as libc::c_int as usize].str_0).is_null()
        && strcmp(
            (*display).d_tcs[69 as libc::c_int as usize].str_0,
            (*display).d_tcs[70 as libc::c_int as usize].str_0,
        ) != 0
    {
        AddCStr((*display).d_tcs[69 as libc::c_int as usize].str_0);
    }
    if !((*display).d_tcs[71 as libc::c_int as usize].str_0).is_null()
        && strcmp(
            (*display).d_tcs[71 as libc::c_int as usize].str_0,
            (*display).d_tcs[72 as libc::c_int as usize].str_0,
        ) != 0
    {
        AddCStr((*display).d_tcs[71 as libc::c_int as usize].str_0);
    }
    AddCStr((*display).d_tcs[99 as libc::c_int as usize].str_0);
    (*display).d_rend = mchar_null;
    (*display).d_atyp = 0 as libc::c_int as libc::c_char;
    AddCStr((*display).d_tcs[79 as libc::c_int as usize].str_0);
    (*display).d_hstatus = 0 as libc::c_int;
    AddCStr((*display).d_tcs[82 as libc::c_int as usize].str_0);
    (*display).d_curvis = 0 as libc::c_int;
    ChangeScrollRegion(oldtop, oldbot);
    SetRendition(&mut oldrend);
    KeypadMode(oldkeypad);
    CursorkeysMode(oldcursorkeys);
    CursorVisibility(oldcurvis);
    MouseMode(oldmouse);
    ExtMouseMode(oldextmouse);
    if !((*display).d_tcs[44 as libc::c_int as usize].str_0).is_null() {
        AddCStr(
            tgoto(
                (*display).d_tcs[44 as libc::c_int as usize].str_0,
                (*display).d_width,
                (*display).d_height,
            ),
        );
    } else if !((*display).d_tcs[45 as libc::c_int as usize].str_0).is_null()
        && ((*display).d_width == Z0width || (*display).d_width == Z1width)
    {
        AddCStr(
            if (*display).d_width == Z0width {
                (*display).d_tcs[45 as libc::c_int as usize].str_0
            } else {
                (*display).d_tcs[46 as libc::c_int as usize].str_0
            },
        );
    }
}
unsafe extern "C" fn disp_writeev_eagain(
    mut ev: *mut event,
    mut data: *mut libc::c_char,
) {
    display = data as *mut display;
    evdeq(&mut (*display).d_writeev);
    (*display).d_writeev.type_0 = 2 as libc::c_int;
    (*display)
        .d_writeev
        .handler = Some(
        disp_writeev_fn as unsafe extern "C" fn(*mut event, *mut libc::c_char) -> (),
    );
    evenq(&mut (*display).d_writeev);
}
unsafe extern "C" fn disp_writeev_fn(mut ev: *mut event, mut data: *mut libc::c_char) {
    let mut len: libc::c_int = 0;
    let mut size: libc::c_int = 256 as libc::c_int;
    display = data as *mut display;
    len = ((*display).d_obufp).offset_from((*display).d_obuf) as libc::c_long
        as libc::c_int;
    if len < size {
        size = len;
    }
    if (*display).d_status_obufpos != 0 && size > (*display).d_status_obufpos {
        size = (*display).d_status_obufpos;
    }
    size = write(
        (*display).d_userfd,
        (*display).d_obuf as *const libc::c_void,
        size as size_t,
    ) as libc::c_int;
    if size >= 0 as libc::c_int {
        len -= size;
        if len != 0 {
            bcopy(
                ((*display).d_obuf).offset(size as isize) as *const libc::c_void,
                (*display).d_obuf as *mut libc::c_void,
                len as size_t,
            );
        }
        (*display).d_obufp = ((*display).d_obufp).offset(-(size as isize));
        (*display).d_obuffree += size;
        if (*display).d_status_obufpos != 0 {
            (*display).d_status_obufpos -= size;
            if (*display).d_status_obufpos == 0 {
                if (*display).d_status == 1 as libc::c_int {
                    (*display).d_status_obuflen = (*display).d_obuflen;
                    (*display).d_status_obuffree = (*display).d_obuffree;
                    (*display).d_obuflen = 0 as libc::c_int;
                    (*display).d_obuffree = (*display).d_obuflen;
                }
                gettimeofday(&mut (*display).d_status_time, 0 as *mut libc::c_void);
                SetTimeout(&mut (*display).d_statusev, MsgWait);
                evenq(&mut (*display).d_statusev);
            }
        }
        if (*display).d_blocked_fuzz != 0 {
            (*display).d_blocked_fuzz -= size;
            if (*display).d_blocked_fuzz < 0 as libc::c_int {
                (*display).d_blocked_fuzz = 0 as libc::c_int;
            }
        }
        if (*display).d_blockedev.queued != 0 {
            if ((*display).d_obufp).offset_from((*display).d_obuf) as libc::c_long
                > ((*display).d_obufmax / 2 as libc::c_int) as libc::c_long
            {
                SetTimeout(&mut (*display).d_blockedev, (*display).d_nonblock);
            } else {
                evdeq(&mut (*display).d_blockedev);
            }
        }
        if (*display).d_blocked == 1 as libc::c_int
            && (*display).d_obuf == (*display).d_obufp
        {
            (*display).d_blocked = 0 as libc::c_int;
            Activate(
                if !((*display).d_fore).is_null() {
                    (*(*display).d_fore).w_norefresh as libc::c_int
                } else {
                    0 as libc::c_int
                },
            );
            (*display)
                .d_blocked_fuzz = ((*display).d_obufp).offset_from((*display).d_obuf)
                as libc::c_long as libc::c_int;
        }
    } else {
        if *__errno_location() == 11 as libc::c_int {
            evdeq(&mut (*display).d_writeev);
            (*display).d_writeev.type_0 = 0 as libc::c_int;
            (*display)
                .d_writeev
                .handler = ::std::mem::transmute::<
                Option::<unsafe extern "C" fn() -> ()>,
                Option::<unsafe extern "C" fn(*mut event, *mut libc::c_char) -> ()>,
            >(
                Some(
                    ::std::mem::transmute::<
                        unsafe extern "C" fn(*mut event, *mut libc::c_char) -> (),
                        unsafe extern "C" fn() -> (),
                    >(disp_writeev_eagain),
                ),
            );
            SetTimeout(&mut (*display).d_writeev, 100 as libc::c_int);
            evenq(&mut (*display).d_writeev);
        }
        if *__errno_location() != 4 as libc::c_int
            && *__errno_location() != 11 as libc::c_int
        {
            Msg(
                *__errno_location(),
                b"Error writing output to display\0" as *const u8 as *const libc::c_char,
            );
        }
    };
}
unsafe extern "C" fn disp_readev_fn(mut ev: *mut event, mut data: *mut libc::c_char) {
    let mut size: libc::c_int = 0;
    let mut bufspace: [libc::c_char; 4132] = [0; 4132];
    let mut buf: *mut libc::c_uchar = (bufspace.as_mut_ptr() as *mut libc::c_uchar)
        .offset(
            (3 as libc::c_int + 10 as libc::c_int + 1 as libc::c_int + 10 as libc::c_int
                + 1 as libc::c_int + 10 as libc::c_int + 1 as libc::c_int) as isize,
        );
    let mut cv: *mut canvas = 0 as *mut canvas;
    display = data as *mut display;
    if !((*display).d_forecv).is_null() {
        cv = (*(*(*display).d_forecv).c_layer).l_cvlist;
        while !cv.is_null() {
            display = (*cv).c_display;
            if (*display).d_status == 1 as libc::c_int {
                RemoveStatus();
            }
            cv = (*cv).c_lnext;
        }
    }
    display = data as *mut display;
    if ((*display).d_fore).is_null() {
        size = 4096 as libc::c_int;
    } else if !((*(*display).d_fore).w_pwin).is_null()
        && (*(*(*display).d_fore).w_pwin).p_fdpat & 0x1000 as libc::c_int != 0
    {
        size = (::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong)
            .wrapping_sub((*(*(*display).d_fore).w_pwin).p_inlen as libc::c_ulong)
            as libc::c_int;
    } else {
        size = (::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong)
            .wrapping_sub((*(*display).d_fore).w_inlen as libc::c_ulong) as libc::c_int;
    }
    if size > 4096 as libc::c_int {
        size = 4096 as libc::c_int;
    }
    if size <= 0 as libc::c_int {
        size = 1 as libc::c_int;
    }
    size = read((*display).d_userfd, buf as *mut libc::c_void, size as size_t)
        as libc::c_int;
    if size < 0 as libc::c_int {
        if *__errno_location() == 4 as libc::c_int
            || *__errno_location() == 11 as libc::c_int
        {
            return;
        }
        Hangup();
        sleep(1 as libc::c_int as libc::c_uint);
        return;
    } else if size == 0 as libc::c_int {
        Hangup();
        sleep(1 as libc::c_int as libc::c_uint);
        return;
    }
    if (*display).d_blocked == 4 as libc::c_int {
        (*display).d_blocked = 0 as libc::c_int;
        KillBlanker();
        Activate(
            if !((*display).d_fore).is_null() {
                (*(*display).d_fore).w_norefresh as libc::c_int
            } else {
                0 as libc::c_int
            },
        );
        ResetIdle();
        return;
    }
    if (*display).d_blocked > 1 as libc::c_int {
        let mut bufp: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut p: *mut win = 0 as *mut win;
        flayer = 0 as *mut layer;
        p = windows;
        while !p.is_null() {
            if (*p).w_zdisplay == display {
                flayer = &mut (*p).w_layer;
                bufp = buf as *mut libc::c_char;
                while size > 0 as libc::c_int {
                    (Some(((*(*flayer).l_layfn).lf_LayProcess).unwrap()))
                        .unwrap()(&mut bufp, &mut size);
                }
                return;
            }
            p = (*p).w_next;
        }
        zmodem_abort(0 as *mut win, display);
    }
    if idletimo > 0 as libc::c_int {
        ResetIdle();
    }
    if !((*display).d_fore).is_null() {
        (*(*display).d_fore).w_lastdisp = display;
    }
    if (*display).d_mouse != 0 && !((*display).d_forecv).is_null() {
        let mut bp: *mut libc::c_uchar = buf;
        let mut end: *mut libc::c_uchar = bp.offset(size as isize);
        let mut mark: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
        let mut current_block_65: u64;
        match (*display).d_mouse_parse.state {
            5 => {
                buf = buf.offset(-1);
                *buf = '[' as i32 as libc::c_uchar;
                size += 1;
                size;
                current_block_65 = 5994502731804054259;
            }
            4 => {
                current_block_65 = 5994502731804054259;
            }
            2 | 1 | 0 | _ => {
                current_block_65 = 5159818223158340697;
            }
        }
        match current_block_65 {
            5994502731804054259 => {
                buf = buf.offset(-1);
                *buf = '\u{1b}' as i32 as libc::c_uchar;
                size += 1;
                size;
            }
            _ => {}
        }
        while bp != end {
            let fresh50 = bp;
            bp = bp.offset(1);
            let mut c: libc::c_uchar = *fresh50;
            match (*display).d_mouse_parse.state {
                6 => {
                    if c as libc::c_int == '\u{1b}' as i32 {
                        mark = bp.offset(-(1 as libc::c_int as isize));
                        (*display).d_mouse_parse.state = CSI_ESC_SEEN as libc::c_int;
                    }
                }
                4 => {
                    if c as libc::c_int == '[' as i32 {
                        (*display).d_mouse_parse.state = CSI_BEGIN as libc::c_int;
                    } else {
                        (*display).d_mouse_parse.state = CSI_INACTIVE as libc::c_int;
                    }
                }
                5 => {
                    if c as libc::c_int == 'M' as i32 {
                        (*display).d_mouse_parse.state = CSI_PB as libc::c_int;
                        (*display)
                            .d_mouse_parse
                            .sgrmode = 0 as libc::c_int as libc::c_char;
                    } else if c as libc::c_int == '<' as i32 {
                        (*display).d_mouse_parse.state = CSI_PB as libc::c_int;
                        (*display)
                            .d_mouse_parse
                            .params[(*display).d_mouse_parse.state
                            as usize] = 0 as libc::c_int;
                        (*display)
                            .d_mouse_parse
                            .sgrmode = 1 as libc::c_int as libc::c_char;
                    } else {
                        (*display).d_mouse_parse.state = CSI_INACTIVE as libc::c_int;
                    }
                }
                0 | 1 | 2 => {
                    if (*display).d_mouse_parse.sgrmode != 0 {
                        if '0' as i32 <= c as libc::c_int
                            && c as libc::c_int <= '9' as i32
                        {
                            (*display)
                                .d_mouse_parse
                                .params[(*display).d_mouse_parse.state as usize]
                                *= 10 as libc::c_int;
                            (*display)
                                .d_mouse_parse
                                .params[(*display).d_mouse_parse.state as usize]
                                += c as libc::c_int - '0' as i32;
                        } else if (*display).d_mouse_parse.state == CSI_PY as libc::c_int
                        {
                            if c as libc::c_int == 'M' as i32
                                || c as libc::c_int == 'm' as i32
                            {
                                (*display).d_mouse_parse.state = CSI_DONE as libc::c_int;
                            } else {
                                (*display).d_mouse_parse.state = CSI_INVALID as libc::c_int;
                            }
                        } else if c as libc::c_int == ';' as i32 {
                            (*display).d_mouse_parse.state += 1;
                            (*display).d_mouse_parse.state;
                            (*display)
                                .d_mouse_parse
                                .params[(*display).d_mouse_parse.state
                                as usize] = 0 as libc::c_int;
                        } else {
                            (*display).d_mouse_parse.state = CSI_INVALID as libc::c_int;
                        }
                    } else {
                        let fresh51 = (*display).d_mouse_parse.state;
                        (*display)
                            .d_mouse_parse
                            .state = (*display).d_mouse_parse.state + 1;
                        (*display)
                            .d_mouse_parse
                            .params[fresh51 as usize] = c as libc::c_int;
                    }
                }
                _ => {}
            }
            if (*display).d_mouse_parse.state == CSI_INVALID as libc::c_int {
                if buf < mark {
                    disp_processinput(
                        display,
                        buf,
                        mark.offset_from(buf) as libc::c_long as libc::c_int,
                    );
                }
                buf = bp;
                size = end.offset_from(bp) as libc::c_long as libc::c_int;
                (*display).d_mouse_parse.state = CSI_INACTIVE as libc::c_int;
            } else if (*display).d_mouse_parse.state == CSI_DONE as libc::c_int {
                if buf < mark {
                    disp_processinput(
                        display,
                        buf,
                        mark.offset_from(buf) as libc::c_long as libc::c_int,
                    );
                }
                buf = bp;
                size = end.offset_from(bp) as libc::c_long as libc::c_int;
                let mut x: libc::c_int = (*display)
                    .d_mouse_parse
                    .params[CSI_PX as libc::c_int as usize];
                let mut y: libc::c_int = (*display)
                    .d_mouse_parse
                    .params[CSI_PY as libc::c_int as usize];
                let mut bias: libc::c_int = if (*display).d_mouse_parse.sgrmode
                    as libc::c_int != 0
                {
                    1 as libc::c_int
                } else {
                    33 as libc::c_int
                };
                x -= bias;
                y -= bias;
                if x >= (*(*display).d_forecv).c_xs && x <= (*(*display).d_forecv).c_xe
                    && y >= (*(*display).d_forecv).c_ys
                    && y <= (*(*display).d_forecv).c_ye
                {
                    if !((*display).d_fore).is_null()
                        && (*(*display).d_fore).w_mouse != 0
                        || (*display).d_mousetrack != 0
                            && (*(*(*display).d_forecv).c_layer).l_mode
                                == 1 as libc::c_int
                    {
                        x -= (*(*display).d_forecv).c_xoff;
                        y -= (*(*display).d_forecv).c_yoff;
                        if x >= 0 as libc::c_int
                            && x < (*(*(*display).d_forecv).c_layer).l_width
                            && y >= 0 as libc::c_int
                            && y < (*(*(*display).d_forecv).c_layer).l_height
                        {
                            let mut tmp: [libc::c_char; 37] = [0; 37];
                            let mut n: libc::c_int = 0;
                            x += bias;
                            y += bias;
                            if (*display).d_mouse_parse.sgrmode != 0 {
                                n = snprintf(
                                    tmp.as_mut_ptr(),
                                    (3 as libc::c_int + 10 as libc::c_int + 1 as libc::c_int
                                        + 10 as libc::c_int + 1 as libc::c_int + 10 as libc::c_int
                                        + 1 as libc::c_int) as libc::c_ulong,
                                    b"\x1B[<%d;%d;%d%c\0" as *const u8 as *const libc::c_char,
                                    (*display)
                                        .d_mouse_parse
                                        .params[CSI_PB as libc::c_int as usize],
                                    x,
                                    y,
                                    c as libc::c_int,
                                );
                            } else {
                                n = snprintf(
                                    tmp.as_mut_ptr(),
                                    (3 as libc::c_int + 10 as libc::c_int + 1 as libc::c_int
                                        + 10 as libc::c_int + 1 as libc::c_int + 10 as libc::c_int
                                        + 1 as libc::c_int) as libc::c_ulong,
                                    b"\x1B[M%c%c%c\0" as *const u8 as *const libc::c_char,
                                    (*display)
                                        .d_mouse_parse
                                        .params[CSI_PB as libc::c_int as usize],
                                    x,
                                    y,
                                );
                            }
                            if n
                                > 3 as libc::c_int + 10 as libc::c_int + 1 as libc::c_int
                                    + 10 as libc::c_int + 1 as libc::c_int + 10 as libc::c_int
                                    + 1 as libc::c_int
                            {
                                n = 3 as libc::c_int + 10 as libc::c_int + 1 as libc::c_int
                                    + 10 as libc::c_int + 1 as libc::c_int + 10 as libc::c_int
                                    + 1 as libc::c_int;
                            }
                            buf = buf.offset(-(n as isize));
                            size += n;
                            memcpy(
                                buf as *mut libc::c_void,
                                tmp.as_mut_ptr() as *const libc::c_void,
                                n as libc::c_ulong,
                            );
                        }
                    }
                } else if (*display).d_mousetrack != 0 {
                    let mut focus: libc::c_int = 0 as libc::c_int;
                    if (*display).d_mouse_parse.sgrmode != 0 {
                        focus = (c as libc::c_int == 'm' as i32) as libc::c_int;
                    } else {
                        focus = ((*display)
                            .d_mouse_parse
                            .params[CSI_PB as libc::c_int as usize] == '#' as i32)
                            as libc::c_int;
                    }
                    let mut cv_0: *mut canvas = FindCanvas(x, y);
                    if focus != 0 && !cv_0.is_null() {
                        SetForeCanvas(display, cv_0);
                    }
                }
                (*display).d_mouse_parse.state = CSI_INACTIVE as libc::c_int;
            }
        }
        if (*display).d_mouse_parse.state != CSI_INACTIVE as libc::c_int {
            size = (if !mark.is_null() {
                mark.offset_from(buf) as libc::c_long
            } else {
                0 as libc::c_int as libc::c_long
            }) as libc::c_int;
        }
    }
    if size > 0 as libc::c_int {
        disp_processinput(display, buf, size);
    }
}
unsafe extern "C" fn disp_processinput(
    mut display_0: *mut display,
    mut buf: *mut libc::c_uchar,
    mut size: libc::c_int,
) {
    if (*display_0).d_encoding
        != (if !((*display_0).d_forecv).is_null() {
            (*(*(*display_0).d_forecv).c_layer).l_encoding
        } else {
            0 as libc::c_int
        })
    {
        let mut i: libc::c_int = 0;
        let mut j: libc::c_int = 0;
        let mut c: libc::c_int = 0;
        let mut enc: libc::c_int = 0;
        let mut buf2: [libc::c_char; 8202] = [0; 8202];
        enc = if !((*display_0).d_forecv).is_null() {
            (*(*(*display_0).d_forecv).c_layer).l_encoding
        } else {
            0 as libc::c_int
        };
        j = 0 as libc::c_int;
        i = j;
        while i < size {
            c = *buf.offset(i as isize) as libc::c_int;
            c = DecodeChar(c, (*display_0).d_encoding, &mut (*display_0).d_decodestate);
            if c == -(2 as libc::c_int) {
                i -= 1;
                i;
            }
            if !(c < 0 as libc::c_int) {
                if pastefont != 0 {
                    let mut font: libc::c_int = 0 as libc::c_int;
                    j
                        += EncodeChar(
                            buf2.as_mut_ptr().offset(j as isize),
                            c,
                            enc,
                            &mut font,
                        );
                    j
                        += EncodeChar(
                            buf2.as_mut_ptr().offset(j as isize),
                            -(1 as libc::c_int),
                            enc,
                            &mut font,
                        );
                } else {
                    j
                        += EncodeChar(
                            buf2.as_mut_ptr().offset(j as isize),
                            c,
                            enc,
                            0 as *mut libc::c_int,
                        );
                }
                if j
                    > ::std::mem::size_of::<[libc::c_char; 8202]>() as libc::c_ulong
                        as libc::c_int - 10 as libc::c_int
                {
                    break;
                }
            }
            i += 1;
            i;
        }
        (Some(((*display_0).d_processinput).unwrap())).unwrap()(buf2.as_mut_ptr(), j);
        return;
    }
    (Some(((*display_0).d_processinput).unwrap()))
        .unwrap()(buf as *mut libc::c_char, size);
}
unsafe extern "C" fn disp_status_fn(mut ev: *mut event, mut data: *mut libc::c_char) {
    display = data as *mut display;
    if (*display).d_status != 0 {
        RemoveStatus();
    }
}
unsafe extern "C" fn disp_hstatus_fn(mut ev: *mut event, mut data: *mut libc::c_char) {
    display = data as *mut display;
    if (*display).d_status == 2 as libc::c_int {
        SetTimeout(ev, 1 as libc::c_int);
        evenq(ev);
        return;
    }
    RefreshHStatus();
}
unsafe extern "C" fn disp_blocked_fn(mut ev: *mut event, mut data: *mut libc::c_char) {
    let mut p: *mut win = 0 as *mut win;
    display = data as *mut display;
    if ((*display).d_obufp).offset_from((*display).d_obuf) as libc::c_long
        > ((*display).d_obufmax + (*display).d_blocked_fuzz) as libc::c_long
    {
        (*display).d_blocked = 1 as libc::c_int;
        p = windows;
        while !p.is_null() {
            if (*p).w_readev.condneg == &mut (*display).d_obuflenmax as *mut libc::c_int
            {
                (*p).w_readev.condneg = 0 as *mut libc::c_int;
                (*p).w_readev.condpos = (*p).w_readev.condneg;
            }
            p = (*p).w_next;
        }
    }
}
unsafe extern "C" fn disp_map_fn(mut ev: *mut event, mut data: *mut libc::c_char) {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut l: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut q: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    display = data as *mut display;
    l = (*display).d_seql;
    if l == 0 {
        return;
    }
    p = ((*display).d_seqp as *mut libc::c_char).offset(-(l as isize));
    (*display).d_seqp = ((*display).d_kmaps).offset(3 as libc::c_int as isize);
    (*display).d_seql = 0 as libc::c_int;
    q = (*display).d_seqh;
    if !q.is_null() {
        (*display).d_seqh = 0 as *mut libc::c_uchar;
        i = (*q.offset(0 as libc::c_int as isize) as libc::c_int) << 8 as libc::c_int
            | *q.offset(1 as libc::c_int as isize) as libc::c_int;
        i &= !(0x4000 as libc::c_int);
        if StuffKey(i) != 0 {
            ProcessInput2(
                (q as *mut libc::c_char).offset(3 as libc::c_int as isize),
                *q.offset(2 as libc::c_int as isize) as libc::c_int,
            );
        }
        if display.is_null() {
            return;
        }
        l -= *q.offset(2 as libc::c_int as isize) as libc::c_int;
        p = p.offset(*q.offset(2 as libc::c_int as isize) as libc::c_int as isize);
    } else {
        (*display).d_dontmap = 1 as libc::c_int;
    }
    ProcessInput(p, l);
}
unsafe extern "C" fn disp_idle_fn(mut ev: *mut event, mut data: *mut libc::c_char) {
    let mut olddisplay: *mut display = 0 as *mut display;
    display = data as *mut display;
    if idletimo <= 0 as libc::c_int || idleaction.nr == -(1 as libc::c_int) {
        return;
    }
    olddisplay = display;
    flayer = (*(*display).d_forecv).c_layer;
    fore = (*display).d_fore;
    DoAction(&mut idleaction, -(1 as libc::c_int));
    if idleaction.nr == 19 as libc::c_int {
        return;
    }
    display = displays;
    while !display.is_null() {
        if olddisplay == display {
            break;
        }
        display = (*display).d_next;
    }
    if !display.is_null() {
        ResetIdle();
    }
}
pub unsafe extern "C" fn ResetIdle() {
    if idletimo > 0 as libc::c_int {
        SetTimeout(&mut (*display).d_idleev, idletimo);
        if (*display).d_idleev.queued == 0 {
            evenq(&mut (*display).d_idleev);
        }
    } else {
        evdeq(&mut (*display).d_idleev);
    };
}
unsafe extern "C" fn disp_blanker_fn(mut ev: *mut event, mut data: *mut libc::c_char) {
    let mut buf: [libc::c_char; 4096] = [0; 4096];
    let mut b: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut size: libc::c_int = 0;
    display = data as *mut display;
    size = read(
        (*display).d_blankerev.fd,
        buf.as_mut_ptr() as *mut libc::c_void,
        4096 as libc::c_int as size_t,
    ) as libc::c_int;
    if size <= 0 as libc::c_int {
        evdeq(&mut (*display).d_blankerev);
        close((*display).d_blankerev.fd);
        (*display).d_blankerev.fd = -(1 as libc::c_int);
        return;
    }
    b = buf.as_mut_ptr();
    while size != 0 {
        (*display).d_obuffree -= 1;
        if (*display).d_obuffree <= 0 as libc::c_int {
            Resize_obuf();
        }
        let fresh52 = b;
        b = b.offset(1);
        let fresh53 = (*display).d_obufp;
        (*display).d_obufp = ((*display).d_obufp).offset(1);
        *fresh53 = *fresh52;
        size -= 1;
        size;
    }
}
pub unsafe extern "C" fn KillBlanker() {
    let mut oldtop: libc::c_int = (*display).d_top;
    let mut oldbot: libc::c_int = (*display).d_bot;
    let mut oldrend: mchar = mchar {
        image: 0,
        attr: 0,
        font: 0,
        fontx: 0,
        color: 0,
        mbcs: 0,
    };
    if (*display).d_blankerev.fd == -(1 as libc::c_int) {
        return;
    }
    if (*display).d_blocked == 4 as libc::c_int {
        (*display).d_blocked = 0 as libc::c_int;
    }
    evdeq(&mut (*display).d_blankerev);
    close((*display).d_blankerev.fd);
    (*display).d_blankerev.fd = -(1 as libc::c_int);
    Kill((*display).d_blankerpid, 1 as libc::c_int);
    (*display).d_bot = -(1 as libc::c_int);
    (*display).d_top = (*display).d_bot;
    oldrend = (*display).d_rend;
    if !((*display).d_tcs[55 as libc::c_int as usize].str_0).is_null() {
        AddCStr((*display).d_tcs[55 as libc::c_int as usize].str_0);
        AddCStr((*display).d_tcs[55 as libc::c_int as usize].str_0);
    } else {
        if (*display).d_hascolor != 0 {
            AddStr(
                b"\x1B[m\x1B[m\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        }
        AddCStr((*display).d_tcs[54 as libc::c_int as usize].str_0);
        AddCStr((*display).d_tcs[53 as libc::c_int as usize].str_0);
    }
    AddCStr((*display).d_tcs[82 as libc::c_int as usize].str_0);
    AddCStr((*display).d_tcs[99 as libc::c_int as usize].str_0);
    (*display).d_rend = mchar_null;
    (*display).d_atyp = 0 as libc::c_int as libc::c_char;
    (*display).d_curvis = 0 as libc::c_int;
    (*display).d_y = -(1 as libc::c_int);
    (*display).d_x = (*display).d_y;
    ChangeScrollRegion(oldtop, oldbot);
    SetRendition(&mut oldrend);
    ClearAll();
}
pub unsafe extern "C" fn RunBlanker(mut cmdv: *mut *mut libc::c_char) {
    let mut m: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pid: libc::c_int = 0;
    let mut slave: libc::c_int = -(1 as libc::c_int);
    let mut ptype: libc::c_int = 0 as libc::c_int;
    let mut termname: [libc::c_char; 38] = [0; 38];
    let mut np: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    strcpy(termname.as_mut_ptr(), b"TERM=\0" as *const u8 as *const libc::c_char);
    strncpy(
        termname.as_mut_ptr().offset(5 as libc::c_int as isize),
        ((*display).d_termname).as_mut_ptr(),
        (32 as libc::c_int - 6 as libc::c_int) as libc::c_ulong,
    );
    termname[(::std::mem::size_of::<[libc::c_char; 38]>() as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        as usize] = 0 as libc::c_int as libc::c_char;
    KillBlanker();
    (*display).d_blankerpid = -(1 as libc::c_int);
    (*display).d_blankerev.fd = OpenDevice(cmdv, 0 as libc::c_int, &mut ptype, &mut m);
    if (*display).d_blankerev.fd == -(1 as libc::c_int) {
        Msg(
            0 as libc::c_int,
            b"OpenDevice failed\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if pty_preopen != 0 {
        slave = open(m, 0o2 as libc::c_int | 0o400 as libc::c_int);
        if slave == -(1 as libc::c_int) {
            Msg(*__errno_location(), b"%s\0" as *const u8 as *const libc::c_char, m);
            close((*display).d_blankerev.fd);
            (*display).d_blankerev.fd = -(1 as libc::c_int);
            return;
        }
    }
    pid = fork();
    match pid {
        -1 => {
            Msg(*__errno_location(), b"fork\0" as *const u8 as *const libc::c_char);
            close((*display).d_blankerev.fd);
            (*display).d_blankerev.fd = -(1 as libc::c_int);
            close(slave);
            return;
        }
        0 => {
            displays = 0 as *mut display;
            ServerSocket = -(1 as libc::c_int);
            xsignal(13 as libc::c_int, None);
            if setgid(real_gid as __gid_t) != 0 || setuid(real_uid as __uid_t) != 0 {
                Panic(
                    *__errno_location(),
                    b"setuid/setgid\0" as *const u8 as *const libc::c_char,
                );
            }
            eff_uid = real_uid;
            eff_gid = real_gid;
            brktty((*display).d_userfd);
            freetty();
            if slave != -(1 as libc::c_int) {
                close(0 as libc::c_int);
                dup(slave);
                close(slave);
                closeallfiles((*display).d_blankerev.fd);
                slave = dup(0 as libc::c_int);
            } else {
                closeallfiles((*display).d_blankerev.fd);
            }
            close(0 as libc::c_int);
            close(1 as libc::c_int);
            close(2 as libc::c_int);
            if open(m, 0o2 as libc::c_int) != 0 {
                Panic(
                    *__errno_location(),
                    b"Cannot open %s\0" as *const u8 as *const libc::c_char,
                    m,
                );
            }
            dup(0 as libc::c_int);
            dup(0 as libc::c_int);
            close((*display).d_blankerev.fd);
            if slave != -(1 as libc::c_int) {
                close(slave);
            }
            InitPTY(0 as libc::c_int);
            fgtty(0 as libc::c_int);
            SetTTY(0 as libc::c_int, &mut (*display).d_OldMode);
            np = NewEnv.offset(3 as libc::c_int as isize);
            let fresh54 = np;
            np = np.offset(1);
            *fresh54 = *NewEnv.offset(0 as libc::c_int as isize);
            let fresh55 = np;
            np = np.offset(1);
            *fresh55 = termname.as_mut_ptr();
            glwz.ws_col = (*display).d_width as libc::c_ushort;
            glwz.ws_row = (*display).d_height as libc::c_ushort;
            ioctl(
                0 as libc::c_int,
                0x5414 as libc::c_int as libc::c_ulong,
                &mut glwz as *mut winsize as *mut libc::c_char,
            );
            execvpe(
                *cmdv,
                cmdv as *const *mut libc::c_char,
                NewEnv.offset(3 as libc::c_int as isize) as *const *mut libc::c_char,
            );
            Panic(
                *__errno_location(),
                b"Cannot exec '%s'\0" as *const u8 as *const libc::c_char,
                *cmdv,
            );
        }
        _ => {}
    }
    (*display).d_blankerpid = pid;
    evenq(&mut (*display).d_blankerev);
    (*display).d_blocked = 4 as libc::c_int;
    ClearAll();
    close(slave);
}
