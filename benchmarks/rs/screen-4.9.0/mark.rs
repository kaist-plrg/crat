use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type logfile;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn bcopy(__src: *const libc::c_void, __dest: *mut libc::c_void, __n: size_t);
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn WindowChanged(_: *mut win, _: libc::c_int);
    fn WriteFile(_: *mut acluser, _: *mut libc::c_char, _: libc::c_int);
    fn Search(_: libc::c_int);
    fn ISearch(_: libc::c_int);
    fn DoProcess(
        _: *mut win,
        _: *mut *mut libc::c_char,
        _: *mut libc::c_int,
        _: *mut paster,
    );
    fn DefClearLine(_: libc::c_int, _: libc::c_int, _: libc::c_int, _: libc::c_int);
    fn DefResize(_: libc::c_int, _: libc::c_int) -> libc::c_int;
    fn DefRestore();
    fn PUTCHAR(_: libc::c_int);
    fn Redisplay(_: libc::c_int);
    fn evdeq(_: *mut event);
    fn UserFreeCopyBuffer(_: *mut acluser) -> libc::c_int;
    fn LGotoPos(_: *mut layer, _: libc::c_int, _: libc::c_int);
    fn LPutChar(_: *mut layer, _: *mut mchar, _: libc::c_int, _: libc::c_int);
    fn LScrollV(
        _: *mut layer,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
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
    fn LMsg(_: libc::c_int, _: *const libc::c_char, _: ...);
    fn InitOverlayPage(_: libc::c_int, _: *mut LayFuncs, _: libc::c_int) -> libc::c_int;
    fn ExitOverlayPage();
    fn LayProcessMouse(_: *mut layer, _: libc::c_uchar) -> libc::c_int;
    fn LayProcessMouseSwitch(_: *mut layer, _: libc::c_int);
    fn ToUtf8_comb(_: *mut libc::c_char, _: libc::c_int) -> libc::c_int;
    fn ContainsSpecialDeffont(
        _: *mut mline,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
    ) -> libc::c_int;
    fn EncodeChar(
        _: *mut libc::c_char,
        _: libc::c_int,
        _: libc::c_int,
        _: *mut libc::c_int,
    ) -> libc::c_int;
    static mut flayer: *mut layer;
    static mut display: *mut display;
    static mut fore: *mut win;
    static mut mchar_so: mchar;
}
pub type __int32_t = libc::c_int;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type pid_t = __pid_t;
pub type size_t = libc::c_ulong;
pub type int32_t = __int32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
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
    pub ut_tv: C2RustUnnamed_0,
    pub ut_addr_v6: [int32_t; 4],
    pub __glibc_reserved: [libc::c_char; 20],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
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
    pub l_mouseevent: C2RustUnnamed_2,
    pub l_pause: C2RustUnnamed_1,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
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
pub struct C2RustUnnamed_2 {
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
    pub w_alt: C2RustUnnamed_3,
    pub w_destroyev: event,
    pub w_exitstatus: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
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
pub struct markdata {
    pub md_window: *mut win,
    pub md_user: *mut acluser,
    pub cx: libc::c_int,
    pub cy: libc::c_int,
    pub x1: libc::c_int,
    pub y1: libc::c_int,
    pub second: libc::c_int,
    pub left_mar: libc::c_int,
    pub right_mar: libc::c_int,
    pub nonl: libc::c_int,
    pub rep_cnt: libc::c_int,
    pub append_mode: libc::c_int,
    pub write_buffer: libc::c_int,
    pub hist_offset: libc::c_int,
    pub isstr: [libc::c_char; 100],
    pub isstrl: libc::c_int,
    pub isistr: [libc::c_char; 200],
    pub isistrl: libc::c_int,
    pub isdir: libc::c_int,
    pub isstartpos: libc::c_int,
    pub isstartdir: libc::c_int,
    pub f_cmd: C2RustUnnamed_4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub flag: libc::c_int,
    pub target: libc::c_int,
    pub direction: libc::c_int,
}
pub static mut pastefont: libc::c_int = 1 as libc::c_int;
pub static mut MarkLf: LayFuncs = unsafe {
    {
        let mut init = LayFuncs {
            lf_LayProcess: Some(
                MarkProcess
                    as unsafe extern "C" fn(
                        *mut *mut libc::c_char,
                        *mut libc::c_int,
                    ) -> (),
            ),
            lf_LayAbort: Some(MarkAbort as unsafe extern "C" fn() -> ()),
            lf_LayRedisplayLine: Some(
                MarkRedisplayLine
                    as unsafe extern "C" fn(
                        libc::c_int,
                        libc::c_int,
                        libc::c_int,
                        libc::c_int,
                    ) -> (),
            ),
            lf_LayClearLine: Some(
                DefClearLine
                    as unsafe extern "C" fn(
                        libc::c_int,
                        libc::c_int,
                        libc::c_int,
                        libc::c_int,
                    ) -> (),
            ),
            lf_LayRewrite: Some(
                MarkRewrite
                    as unsafe extern "C" fn(
                        libc::c_int,
                        libc::c_int,
                        libc::c_int,
                        *mut mchar,
                        libc::c_int,
                    ) -> libc::c_int,
            ),
            lf_LayResize: Some(
                DefResize
                    as unsafe extern "C" fn(libc::c_int, libc::c_int) -> libc::c_int,
            ),
            lf_LayRestore: Some(DefRestore as unsafe extern "C" fn() -> ()),
            lf_LayFree: None,
        };
        init
    }
};
pub static mut join_with_cr: libc::c_int = 0 as libc::c_int;
pub static mut compacthist: libc::c_int = 0 as libc::c_int;
pub static mut mark_key_tab: [libc::c_uchar; 256] = [0; 256];
static mut markdata: *mut markdata = 0 as *const markdata as *mut markdata;
unsafe extern "C" fn is_letter(mut c: libc::c_char) -> libc::c_int {
    if c as libc::c_int >= 'a' as i32 && c as libc::c_int <= 'z' as i32
        || c as libc::c_int >= 'A' as i32 && c as libc::c_int <= 'Z' as i32
        || c as libc::c_int >= '0' as i32 && c as libc::c_int <= '9' as i32
        || c as libc::c_int == '_' as i32 || c as libc::c_int == '.' as i32
        || c as libc::c_int == '@' as i32 || c as libc::c_int == ':' as i32
        || c as libc::c_int == '%' as i32 || c as libc::c_int == '!' as i32
        || c as libc::c_int == '-' as i32 || c as libc::c_int == '+' as i32
    {
        return 1 as libc::c_int
    } else if c as libc::c_int != ' ' as i32 {
        return 2 as libc::c_int
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn linestart(mut y: libc::c_int) -> libc::c_int {
    let mut x: libc::c_int = 0;
    let mut i: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    x = (*markdata).left_mar;
    i = ((*(if y < (*fore).w_histheight {
        &mut *((*fore).w_hlines)
            .offset((((*fore).w_histidx + y) % (*fore).w_histheight) as isize)
            as *mut mline
    } else {
        &mut *((*fore).w_mlines).offset((y - (*fore).w_histheight) as isize)
            as *mut mline
    }))
        .image)
        .offset(x as isize);
    while x < (*fore).w_layer.l_width - 1 as libc::c_int {
        let fresh0 = i;
        i = i.offset(1);
        if *fresh0 as libc::c_int != ' ' as i32 {
            break;
        }
        x += 1;
        x;
    }
    if x == (*fore).w_layer.l_width - 1 as libc::c_int {
        x = (*markdata).left_mar;
    }
    return x;
}
unsafe extern "C" fn lineend(mut y: libc::c_int) -> libc::c_int {
    let mut x: libc::c_int = 0;
    let mut i: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    x = (*markdata).right_mar;
    i = ((*(if y < (*fore).w_histheight {
        &mut *((*fore).w_hlines)
            .offset((((*fore).w_histidx + y) % (*fore).w_histheight) as isize)
            as *mut mline
    } else {
        &mut *((*fore).w_mlines).offset((y - (*fore).w_histheight) as isize)
            as *mut mline
    }))
        .image)
        .offset(x as isize);
    while x >= 0 as libc::c_int {
        let fresh1 = i;
        i = i.offset(-1);
        if *fresh1 as libc::c_int != ' ' as i32 {
            break;
        }
        x -= 1;
        x;
    }
    if x < 0 as libc::c_int {
        x = (*markdata).left_mar;
    }
    return x;
}
unsafe extern "C" fn nextchar(
    mut xp: *mut libc::c_int,
    mut yp: *mut libc::c_int,
    mut direction: libc::c_int,
    mut target: libc::c_char,
    mut num: libc::c_int,
) -> libc::c_int {
    let mut width: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut step: libc::c_int = 0;
    let mut adjust: libc::c_int = 0;
    let mut displayed_line: *mut libc::c_char = 0 as *mut libc::c_char;
    x = *xp;
    step = 1 as libc::c_int;
    adjust = 0 as libc::c_int;
    width = (*fore).w_layer.l_width;
    displayed_line = (*if *yp < (*fore).w_histheight {
        &mut *((*fore).w_hlines)
            .offset((((*fore).w_histidx + *yp) % (*fore).w_histheight) as isize)
            as *mut mline
    } else {
        &mut *((*fore).w_mlines).offset((*yp - (*fore).w_histheight) as isize)
            as *mut mline
    })
        .image as *mut libc::c_char;
    let mut current_block_12: u64;
    match direction {
        116 => {
            adjust = -(1 as libc::c_int);
            current_block_12 = 8238959367098441733;
        }
        102 => {
            current_block_12 = 8238959367098441733;
        }
        84 => {
            adjust = 1 as libc::c_int;
            current_block_12 = 15733963409361362504;
        }
        70 => {
            current_block_12 = 15733963409361362504;
        }
        _ => {
            current_block_12 = 7976072742316086414;
        }
    }
    match current_block_12 {
        8238959367098441733 => {
            step = 1 as libc::c_int;
        }
        15733963409361362504 => {
            step = -(1 as libc::c_int);
        }
        _ => {}
    }
    x += step;
    while x >= 0 as libc::c_int && x <= width {
        if *displayed_line.offset(x as isize) as libc::c_int == target as libc::c_int {
            num -= 1;
            if num == 0 as libc::c_int {
                *xp = x + adjust;
                return 0 as libc::c_int;
            }
        }
        x += step;
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn nextword(
    mut xp: *mut libc::c_int,
    mut yp: *mut libc::c_int,
    mut flags: libc::c_int,
    mut num: libc::c_int,
) {
    let mut xx: libc::c_int = (*fore).w_layer.l_width;
    let mut yy: libc::c_int = (*fore).w_histheight + (*fore).w_layer.l_height;
    let mut sx: libc::c_int = 0;
    let mut oq: libc::c_int = 0;
    let mut q: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut ml: *mut mline = 0 as *mut mline;
    x = *xp;
    y = *yp;
    sx = if flags & (1 as libc::c_int) << 0 as libc::c_int != 0 {
        -(1 as libc::c_int)
    } else {
        1 as libc::c_int
    };
    if flags & (1 as libc::c_int) << 1 as libc::c_int != 0
        && flags & (1 as libc::c_int) << 2 as libc::c_int != 0
    {
        x += sx;
    }
    ml = if y < (*fore).w_histheight {
        &mut *((*fore).w_hlines)
            .offset((((*fore).w_histidx + y) % (*fore).w_histheight) as isize)
            as *mut mline
    } else {
        &mut *((*fore).w_mlines).offset((y - (*fore).w_histheight) as isize)
            as *mut mline
    };
    oq = -(1 as libc::c_int);
    loop {
        if x >= xx || x < 0 as libc::c_int {
            q = 0 as libc::c_int;
        } else if flags & (1 as libc::c_int) << 3 as libc::c_int != 0 {
            q = (*((*ml).image).offset(x as isize) as libc::c_int == ' ' as i32)
                as libc::c_int;
        } else {
            q = is_letter(*((*ml).image).offset(x as isize) as libc::c_char);
        }
        if oq >= 0 as libc::c_int && oq != q {
            if oq == 0 as libc::c_int
                || flags & (1 as libc::c_int) << 1 as libc::c_int == 0
            {
                *xp = x;
            } else {
                *xp = x - sx;
            }
            *yp = y;
            if flags & (1 as libc::c_int) << 1 as libc::c_int == 0 && q != 0
                || flags & (1 as libc::c_int) << 1 as libc::c_int != 0 && oq != 0
            {
                num -= 1;
                if num <= 0 as libc::c_int {
                    return;
                }
            }
        }
        if x == xx {
            x = -(1 as libc::c_int);
            y += 1;
            if y >= yy {
                return;
            }
            ml = if y < (*fore).w_histheight {
                &mut *((*fore).w_hlines)
                    .offset((((*fore).w_histidx + y) % (*fore).w_histheight) as isize)
                    as *mut mline
            } else {
                &mut *((*fore).w_mlines).offset((y - (*fore).w_histheight) as isize)
                    as *mut mline
            };
        } else if x < 0 as libc::c_int {
            x = xx;
            y -= 1;
            if y < 0 as libc::c_int {
                return;
            }
            ml = if y < (*fore).w_histheight {
                &mut *((*fore).w_hlines)
                    .offset((((*fore).w_histidx + y) % (*fore).w_histheight) as isize)
                    as *mut mline
            } else {
                &mut *((*fore).w_mlines).offset((y - (*fore).w_histheight) as isize)
                    as *mut mline
            };
        }
        x += sx;
        oq = q;
    };
}
unsafe extern "C" fn rem(
    mut x1: libc::c_int,
    mut y1: libc::c_int,
    mut x2: libc::c_int,
    mut y2: libc::c_int,
    mut redisplay: libc::c_int,
    mut pt: *mut libc::c_char,
    mut yend: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut from: libc::c_int = 0;
    let mut to: libc::c_int = 0;
    let mut ry: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut l: libc::c_int = 0 as libc::c_int;
    let mut im: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut ml: *mut mline = 0 as *mut mline;
    let mut cf: libc::c_int = 0;
    let mut cfx: libc::c_int = 0;
    let mut font: libc::c_int = 0;
    let mut fo: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut fox: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    (*markdata).second = 0 as libc::c_int;
    if y2 < y1 || y2 == y1 && x2 < x1 {
        i = y2;
        y2 = y1;
        y1 = i;
        i = x2;
        x2 = x1;
        x1 = i;
    }
    ry = y1 - (*markdata).hist_offset;
    i = y1;
    if redisplay != 2 as libc::c_int && pt.is_null() && ry < 0 as libc::c_int {
        i -= ry;
        ry = 0 as libc::c_int;
    }
    while i <= y2 {
        if redisplay != 2 as libc::c_int && pt.is_null() && ry > yend {
            break;
        }
        ml = if i < (*fore).w_histheight {
            &mut *((*fore).w_hlines)
                .offset((((*fore).w_histidx + i) % (*fore).w_histheight) as isize)
                as *mut mline
        } else {
            &mut *((*fore).w_mlines).offset((i - (*fore).w_histheight) as isize)
                as *mut mline
        };
        from = if i == y1 { x1 } else { 0 as libc::c_int };
        if from < (*markdata).left_mar {
            from = (*markdata).left_mar;
        }
        to = (*fore).w_layer.l_width;
        im = ((*ml).image).offset(to as isize);
        while to >= 0 as libc::c_int {
            let fresh2 = im;
            im = im.offset(-1);
            if *fresh2 as libc::c_int != ' ' as i32 {
                break;
            }
            to -= 1;
            to;
        }
        if i == y2 && x2 < to {
            to = x2;
        }
        if to > (*markdata).right_mar {
            to = (*markdata).right_mar;
        }
        if redisplay == 1 as libc::c_int && from <= to && ry >= 0 as libc::c_int
            && ry <= yend
        {
            MarkRedisplayLine(ry, from, to, 0 as libc::c_int);
        }
        if !(redisplay != 2 as libc::c_int && pt.is_null()) {
            j = from;
            if if (*fore).w_layer.l_encoding == 8 as libc::c_int {
                (*((*ml).font).offset(j as isize) as libc::c_int == 0xff as libc::c_int
                    && *((*ml).image).offset(j as isize) as libc::c_int
                        == 0xff as libc::c_int) as libc::c_int
            } else {
                (*((*ml).font).offset(j as isize) as libc::c_int & 0xe0 as libc::c_int
                    == 0x80 as libc::c_int) as libc::c_int
            } != 0
            {
                j -= 1;
                j;
            }
            im = ((*ml).image).offset(j as isize);
            fo = ((*ml).font).offset(j as isize);
            fox = ((*ml).fontx).offset(j as isize);
            font = 0 as libc::c_int;
            while j <= to {
                let fresh3 = im;
                im = im.offset(1);
                c = *fresh3 as libc::c_int;
                let fresh4 = fo;
                fo = fo.offset(1);
                cf = *fresh4 as libc::c_int;
                let fresh5 = fox;
                fox = fox.offset(1);
                cfx = *fresh5 as libc::c_int;
                if (*fore).w_layer.l_encoding == 8 as libc::c_int {
                    c |= cf << 8 as libc::c_int | cfx << 16 as libc::c_int;
                    if !(c == 0xffff as libc::c_int) {
                        c = ToUtf8_comb(pt, c);
                        l += c;
                        if !pt.is_null() {
                            pt = pt.offset(c as isize);
                        }
                    }
                } else {
                    if cf != 0 && cf & 0x60 as libc::c_int == 0 as libc::c_int {
                        let fresh6 = im;
                        im = im.offset(1);
                        c = c << 8 as libc::c_int | *fresh6 as libc::c_int;
                        fo = fo.offset(1);
                        fo;
                        j += 1;
                        j;
                    }
                    if pastefont != 0 {
                        c = EncodeChar(
                            pt,
                            c | cf << 16 as libc::c_int,
                            (*fore).w_layer.l_encoding,
                            &mut font,
                        );
                        l += c;
                        if !pt.is_null() {
                            pt = pt.offset(c as isize);
                        }
                    } else {
                        if !pt.is_null() {
                            let fresh7 = pt;
                            pt = pt.offset(1);
                            *fresh7 = c as libc::c_char;
                        }
                        l += 1;
                        l;
                    }
                }
                j += 1;
                j;
            }
            if pastefont != 0 && font != 0 as libc::c_int {
                if !pt.is_null() {
                    strcpy(pt, b"\x1B(B\0" as *const u8 as *const libc::c_char);
                    pt = pt.offset(3 as libc::c_int as isize);
                }
                l += 3 as libc::c_int;
            }
            if i != y2
                && (to != (*fore).w_layer.l_width - 1 as libc::c_int
                    || *((*ml).image).offset((to + 1 as libc::c_int) as isize)
                        as libc::c_int == ' ' as i32)
            {
                match (*markdata).nonl {
                    0 => {
                        if !pt.is_null() {
                            let fresh8 = pt;
                            pt = pt.offset(1);
                            *fresh8 = '\r' as i32 as libc::c_char;
                        }
                        l += 1;
                        l;
                        if join_with_cr != 0 {
                            if !pt.is_null() {
                                let fresh9 = pt;
                                pt = pt.offset(1);
                                *fresh9 = '\n' as i32 as libc::c_char;
                            }
                            l += 1;
                            l;
                        }
                    }
                    2 => {
                        if !pt.is_null() {
                            let fresh10 = pt;
                            pt = pt.offset(1);
                            *fresh10 = ' ' as i32 as libc::c_char;
                        }
                        l += 1;
                        l;
                    }
                    3 => {
                        if !pt.is_null() {
                            let fresh11 = pt;
                            pt = pt.offset(1);
                            *fresh11 = ',' as i32 as libc::c_char;
                        }
                        l += 1;
                        l;
                    }
                    1 | _ => {}
                }
            }
        }
        i += 1;
        i;
        ry += 1;
        ry;
    }
    return l;
}
unsafe extern "C" fn eq(mut a: libc::c_int, mut b: libc::c_int) -> libc::c_int {
    if a == b {
        return 1 as libc::c_int;
    }
    if a == 0 as libc::c_int || b == 0 as libc::c_int {
        return 1 as libc::c_int;
    }
    if a <= '9' as i32 && a >= '0' as i32 && b <= '9' as i32 && b >= '0' as i32 {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn GetHistory() -> libc::c_int {
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut q: libc::c_int = 0 as libc::c_int;
    let mut xx: libc::c_int = 0;
    let mut yy: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut linep: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut ml: *mut mline = 0 as *mut mline;
    x = (*fore).w_layer.l_x;
    if x >= (*fore).w_layer.l_width {
        x = (*fore).w_layer.l_width - 1 as libc::c_int;
    }
    y = (*fore).w_layer.l_y + (*fore).w_histheight;
    ml = if y < (*fore).w_histheight {
        &mut *((*fore).w_hlines)
            .offset((((*fore).w_histidx + y) % (*fore).w_histheight) as isize)
            as *mut mline
    } else {
        &mut *((*fore).w_mlines).offset((y - (*fore).w_histheight) as isize)
            as *mut mline
    };
    xx = x - 1 as libc::c_int;
    linep = ((*ml).image).offset(xx as isize);
    while xx >= 0 as libc::c_int {
        let fresh12 = linep;
        linep = linep.offset(-1);
        q = *fresh12 as libc::c_int;
        if q != ' ' as i32 {
            break;
        }
        xx -= 1;
        xx;
    }
    yy = y - 1 as libc::c_int;
    while yy >= 0 as libc::c_int {
        ml = if yy < (*fore).w_histheight {
            &mut *((*fore).w_hlines)
                .offset((((*fore).w_histidx + yy) % (*fore).w_histheight) as isize)
                as *mut mline
        } else {
            &mut *((*fore).w_mlines).offset((yy - (*fore).w_histheight) as isize)
                as *mut mline
        };
        linep = (*ml).image;
        if xx < 0 as libc::c_int || eq(*linep.offset(xx as isize) as libc::c_int, q) != 0
        {
            i = (*fore).w_layer.l_width - 1 as libc::c_int;
            linep = linep.offset(i as isize);
            while i >= x {
                let fresh13 = linep;
                linep = linep.offset(-1);
                if *fresh13 as libc::c_int != ' ' as i32 {
                    break;
                }
                i -= 1;
                i;
            }
            if i >= x {
                break;
            }
        }
        yy -= 1;
        yy;
    }
    if yy < 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    if !((*(*display).d_user).u_plop.buf).is_null() {
        UserFreeCopyBuffer((*display).d_user);
    }
    (*(*display).d_user)
        .u_plop
        .buf = malloc((i - x + 2 as libc::c_int) as libc::c_uint as libc::c_ulong)
        as *mut libc::c_char;
    if ((*(*display).d_user).u_plop.buf).is_null() {
        LMsg(
            0 as libc::c_int,
            b"Not enough memory... Sorry.\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    bcopy(
        (linep as *mut libc::c_char)
            .offset(-(i as isize))
            .offset(x as isize)
            .offset(1 as libc::c_int as isize) as *const libc::c_void,
        (*(*display).d_user).u_plop.buf as *mut libc::c_void,
        (i - x + 1 as libc::c_int) as size_t,
    );
    (*(*display).d_user).u_plop.len = i - x + 1 as libc::c_int;
    (*(*display).d_user).u_plop.enc = (*fore).w_layer.l_encoding;
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn MarkRoutine() {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    if InitOverlayPage(
        ::std::mem::size_of::<markdata>() as libc::c_ulong as libc::c_int,
        &mut MarkLf,
        1 as libc::c_int,
    ) != 0
    {
        return;
    }
    (*flayer).l_encoding = (*fore).w_layer.l_encoding;
    (*flayer).l_mode = 1 as libc::c_int;
    markdata = (*flayer).l_data as *mut markdata;
    (*markdata).md_user = (*display).d_user;
    (*markdata).md_window = fore;
    (*markdata).second = 0 as libc::c_int;
    (*markdata).rep_cnt = 0 as libc::c_int;
    (*markdata).append_mode = 0 as libc::c_int;
    (*markdata).write_buffer = 0 as libc::c_int;
    (*markdata).nonl = 0 as libc::c_int;
    (*markdata).left_mar = 0 as libc::c_int;
    (*markdata).right_mar = (*fore).w_layer.l_width - 1 as libc::c_int;
    (*markdata).hist_offset = (*fore).w_histheight;
    x = (*fore).w_layer.l_x;
    y = (*fore).w_layer.l_y + (*markdata).hist_offset;
    if x >= (*fore).w_layer.l_width {
        x = (*fore).w_layer.l_width - 1 as libc::c_int;
    }
    LGotoPos(flayer, x, y - (*markdata).hist_offset);
    LMsg(
        0 as libc::c_int,
        b"Copy mode - Column %d Line %d(+%d) (%d,%d)\0" as *const u8
            as *const libc::c_char,
        x + 1 as libc::c_int,
        y + 1 as libc::c_int - (*markdata).hist_offset,
        (*fore).w_histheight,
        (*fore).w_layer.l_width,
        (*fore).w_layer.l_height,
    );
    (*markdata).x1 = x;
    (*markdata).cx = (*markdata).x1;
    (*markdata).y1 = y;
    (*markdata).cy = (*markdata).y1;
    (*flayer).l_x = x;
    (*flayer).l_y = y - (*markdata).hist_offset;
}
unsafe extern "C" fn MarkProcess(
    mut inbufp: *mut *mut libc::c_char,
    mut inlenp: *mut libc::c_int,
) {
    let mut inbuf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pt: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut inlen: libc::c_int = 0;
    let mut cx: libc::c_int = 0;
    let mut cy: libc::c_int = 0;
    let mut x2: libc::c_int = 0;
    let mut y2: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut yend: libc::c_int = 0;
    let mut newcopylen: libc::c_int = 0 as libc::c_int;
    let mut od: libc::c_int = 0;
    let mut in_mark: libc::c_int = 0;
    let mut rep_cnt: libc::c_int = 0;
    let mut md_user: *mut acluser = 0 as *mut acluser;
    markdata = (*flayer).l_data as *mut markdata;
    fore = (*markdata).md_window;
    md_user = (*markdata).md_user;
    if inbufp.is_null() {
        MarkAbort();
        return;
    }
    LGotoPos(flayer, (*markdata).cx, (*markdata).cy - (*markdata).hist_offset);
    inbuf = *inbufp;
    inlen = *inlenp;
    pt = inbuf;
    in_mark = 1 as libc::c_int;
    let mut current_block_290: u64;
    while in_mark != 0 && inlen != 0 {
        let fresh14 = pt;
        pt = pt.offset(1);
        let mut ch: libc::c_uchar = *fresh14 as libc::c_uchar;
        inlen -= 1;
        inlen;
        if (*flayer).l_mouseevent.start != 0 {
            let mut r: libc::c_int = LayProcessMouse(flayer, ch);
            if r == -(1 as libc::c_int) {
                LayProcessMouseSwitch(flayer, 0 as libc::c_int);
            } else {
                if !(r != 0) {
                    continue;
                }
                ch = 0o222 as libc::c_int as libc::c_uchar;
            }
        }
        od = mark_key_tab[ch as libc::c_int as usize] as libc::c_int;
        rep_cnt = (*markdata).rep_cnt;
        if od >= '0' as i32 && od <= '9' as i32 && (*markdata).f_cmd.flag == 0 {
            if rep_cnt < 1001 as libc::c_int
                && (od != '0' as i32 || rep_cnt != 0 as libc::c_int)
            {
                (*markdata).rep_cnt = 10 as libc::c_int * rep_cnt + od - '0' as i32;
                continue;
            }
        }
        cx = (*markdata).cx;
        cy = (*markdata).cy;
        if (*markdata).f_cmd.flag != 0 {
            (*markdata).f_cmd.flag = 0 as libc::c_int;
            (*markdata).rep_cnt = 0 as libc::c_int;
            if *(*__ctype_b_loc()).offset(od as isize) as libc::c_int
                & _ISgraph as libc::c_int as libc::c_ushort as libc::c_int != 0
            {
                (*markdata).f_cmd.target = od;
                rep_cnt = if rep_cnt != 0 { rep_cnt } else { 1 as libc::c_int };
                nextchar(
                    &mut cx,
                    &mut cy,
                    (*markdata).f_cmd.direction,
                    od as libc::c_char,
                    rep_cnt,
                );
                revto(cx, cy);
                continue;
            }
        }
        loop {
            match od {
                102 => {
                    current_block_290 = 11072814364684381342;
                    break;
                }
                70 => {
                    current_block_290 = 11072814364684381342;
                    break;
                }
                116 | 84 => {
                    current_block_290 = 2757429485286371073;
                    break;
                }
                59 | 44 => {
                    if (*markdata).f_cmd.target == 0 {
                        current_block_290 = 14203403603292796089;
                        break;
                    } else {
                        current_block_290 = 980989089337379490;
                        break;
                    }
                }
                111 | 120 => {
                    if (*markdata).second == 0 {
                        current_block_290 = 14203403603292796089;
                        break;
                    } else {
                        current_block_290 = 1356832168064818221;
                        break;
                    }
                }
                12 => {
                    Redisplay(0 as libc::c_int);
                    LGotoPos(flayer, cx, cy - (*markdata).hist_offset);
                    current_block_290 = 14203403603292796089;
                    break;
                }
                130 => {
                    current_block_290 = 3793519976100317671;
                    break;
                }
                8 | 104 => {
                    current_block_290 = 3793519976100317671;
                    break;
                }
                142 => {
                    current_block_290 = 3389598844371035468;
                    break;
                }
                14 | 106 => {
                    current_block_290 = 3389598844371035468;
                    break;
                }
                43 => {
                    if rep_cnt == 0 as libc::c_int {
                        rep_cnt = 1 as libc::c_int;
                    }
                    j = cy + rep_cnt;
                    if j
                        > (*fore).w_histheight + (*fore).w_layer.l_height
                            - 1 as libc::c_int
                    {
                        j = (*fore).w_histheight + (*fore).w_layer.l_height
                            - 1 as libc::c_int;
                    }
                    revto(linestart(j), j);
                    current_block_290 = 14203403603292796089;
                    break;
                }
                45 => {
                    if rep_cnt == 0 as libc::c_int {
                        rep_cnt = 1 as libc::c_int;
                    }
                    cy -= rep_cnt;
                    if cy < 0 as libc::c_int {
                        cy = 0 as libc::c_int;
                    }
                    revto(linestart(cy), cy);
                    current_block_290 = 14203403603292796089;
                    break;
                }
                94 => {
                    revto(linestart(cy), cy);
                    current_block_290 = 14203403603292796089;
                    break;
                }
                10 => {
                    revto((*markdata).left_mar, cy + 1 as libc::c_int);
                    current_block_290 = 14203403603292796089;
                    break;
                }
                144 => {
                    current_block_290 = 6888191626228944686;
                    break;
                }
                16 | 107 => {
                    current_block_290 = 6888191626228944686;
                    break;
                }
                134 | 108 => {
                    if rep_cnt == 0 as libc::c_int {
                        rep_cnt = 1 as libc::c_int;
                    }
                    revto(cx + rep_cnt, cy);
                    current_block_290 = 14203403603292796089;
                    break;
                }
                1 | 48 => {
                    revto((*markdata).left_mar, cy);
                    current_block_290 = 14203403603292796089;
                    break;
                }
                4 => {
                    if rep_cnt == 0 as libc::c_int {
                        rep_cnt = (*fore).w_layer.l_height + 1 as libc::c_int
                            >> 1 as libc::c_int;
                    }
                    revto_line(cx, cy + rep_cnt, cy - (*markdata).hist_offset);
                    current_block_290 = 14203403603292796089;
                    break;
                }
                36 => {
                    revto(lineend(cy), cy);
                    current_block_290 = 14203403603292796089;
                    break;
                }
                18 => {
                    ISearch(-(1 as libc::c_int));
                    in_mark = 0 as libc::c_int;
                    current_block_290 = 14203403603292796089;
                    break;
                }
                19 => {
                    ISearch(1 as libc::c_int);
                    in_mark = 0 as libc::c_int;
                    current_block_290 = 14203403603292796089;
                    break;
                }
                21 => {
                    if rep_cnt == 0 as libc::c_int {
                        rep_cnt = (*fore).w_layer.l_height + 1 as libc::c_int
                            >> 1 as libc::c_int;
                    }
                    revto_line(cx, cy - rep_cnt, cy - (*markdata).hist_offset);
                    current_block_290 = 14203403603292796089;
                    break;
                }
                7 => {
                    if (*markdata).left_mar == 0 as libc::c_int
                        && (*markdata).right_mar
                            == (*fore).w_layer.l_width - 1 as libc::c_int
                    {
                        LMsg(
                            0 as libc::c_int,
                            b"Column %d Line %d(+%d)\0" as *const u8
                                as *const libc::c_char,
                            cx + 1 as libc::c_int,
                            cy - (*markdata).hist_offset + 1 as libc::c_int,
                            (*markdata).hist_offset,
                        );
                    } else {
                        LMsg(
                            0 as libc::c_int,
                            b"Column %d(%d..%d) Line %d(+%d)\0" as *const u8
                                as *const libc::c_char,
                            cx + 1 as libc::c_int,
                            (*markdata).left_mar + 1 as libc::c_int,
                            (*markdata).right_mar + 1 as libc::c_int,
                            cy - (*markdata).hist_offset + 1 as libc::c_int,
                            (*markdata).hist_offset,
                        );
                    }
                    current_block_290 = 14203403603292796089;
                    break;
                }
                2 => {
                    if rep_cnt == 0 as libc::c_int {
                        rep_cnt = 1 as libc::c_int;
                    }
                    rep_cnt *= (*fore).w_layer.l_height;
                    revto(cx, cy - rep_cnt);
                    current_block_290 = 14203403603292796089;
                    break;
                }
                6 => {
                    if rep_cnt == 0 as libc::c_int {
                        rep_cnt = 1 as libc::c_int;
                    }
                    rep_cnt *= (*fore).w_layer.l_height;
                    revto(cx, cy + rep_cnt);
                    current_block_290 = 14203403603292796089;
                    break;
                }
                5 => {
                    if rep_cnt == 0 as libc::c_int {
                        rep_cnt = 1 as libc::c_int;
                    }
                    rep_cnt = MarkScrollUpDisplay(rep_cnt);
                    if cy < 0 as libc::c_int + (*markdata).hist_offset {
                        revto(cx, 0 as libc::c_int + (*markdata).hist_offset);
                    } else {
                        LGotoPos(flayer, cx, cy - (*markdata).hist_offset);
                    }
                    current_block_290 = 14203403603292796089;
                    break;
                }
                25 => {
                    if rep_cnt == 0 as libc::c_int {
                        rep_cnt = 1 as libc::c_int;
                    }
                    rep_cnt = MarkScrollDownDisplay(rep_cnt);
                    if cy
                        > (*fore).w_layer.l_height - 1 as libc::c_int
                            + (*markdata).hist_offset
                    {
                        revto(
                            cx,
                            (*fore).w_layer.l_height - 1 as libc::c_int
                                + (*markdata).hist_offset,
                        );
                    } else {
                        LGotoPos(flayer, cx, cy - (*markdata).hist_offset);
                    }
                    current_block_290 = 14203403603292796089;
                    break;
                }
                64 => {
                    current_block_290 = 14203403603292796089;
                    break;
                }
                37 => {
                    if rep_cnt < 0 as libc::c_int {
                        rep_cnt = 0 as libc::c_int;
                    }
                    if rep_cnt > 100 as libc::c_int {
                        rep_cnt = 100 as libc::c_int;
                    }
                    revto_line(
                        (*markdata).left_mar,
                        (*fore).w_histheight - (*fore).w_scrollback_height
                            + ((rep_cnt
                                * ((*fore).w_scrollback_height + (*fore).w_layer.l_height))
                                as libc::c_double / 100.0f64) as libc::c_int,
                        ((*fore).w_layer.l_height - 1 as libc::c_int) / 2 as libc::c_int,
                    );
                    current_block_290 = 14203403603292796089;
                    break;
                }
                129 | 103 => {
                    rep_cnt = 1 as libc::c_int;
                    current_block_290 = 9158226259752878651;
                    break;
                }
                133 | 71 => {
                    current_block_290 = 9158226259752878651;
                    break;
                }
                72 => {
                    revto(
                        (*markdata).left_mar,
                        0 as libc::c_int + (*markdata).hist_offset,
                    );
                    current_block_290 = 14203403603292796089;
                    break;
                }
                77 => {
                    revto(
                        (*markdata).left_mar,
                        ((*fore).w_layer.l_height - 1 as libc::c_int) / 2 as libc::c_int
                            + (*markdata).hist_offset,
                    );
                    current_block_290 = 14203403603292796089;
                    break;
                }
                76 => {
                    revto(
                        (*markdata).left_mar,
                        (*fore).w_layer.l_height - 1 as libc::c_int
                            + (*markdata).hist_offset,
                    );
                    current_block_290 = 14203403603292796089;
                    break;
                }
                124 => {
                    rep_cnt -= 1;
                    revto(rep_cnt, cy);
                    current_block_290 = 14203403603292796089;
                    break;
                }
                119 => {
                    if rep_cnt == 0 as libc::c_int {
                        rep_cnt = 1 as libc::c_int;
                    }
                    nextword(
                        &mut cx,
                        &mut cy,
                        (1 as libc::c_int) << 2 as libc::c_int,
                        rep_cnt,
                    );
                    revto(cx, cy);
                    current_block_290 = 14203403603292796089;
                    break;
                }
                101 | 69 => {
                    if rep_cnt == 0 as libc::c_int {
                        rep_cnt = 1 as libc::c_int;
                    }
                    nextword(
                        &mut cx,
                        &mut cy,
                        (1 as libc::c_int) << 1 as libc::c_int
                            | (1 as libc::c_int) << 2 as libc::c_int
                            | (if od == 'E' as i32 {
                                (1 as libc::c_int) << 3 as libc::c_int
                            } else {
                                0 as libc::c_int
                            }),
                        rep_cnt,
                    );
                    revto(cx, cy);
                    current_block_290 = 14203403603292796089;
                    break;
                }
                98 | 66 => {
                    if rep_cnt == 0 as libc::c_int {
                        rep_cnt = 1 as libc::c_int;
                    }
                    nextword(
                        &mut cx,
                        &mut cy,
                        (1 as libc::c_int) << 0 as libc::c_int
                            | (1 as libc::c_int) << 1 as libc::c_int
                            | (1 as libc::c_int) << 2 as libc::c_int
                            | (if od == 'B' as i32 {
                                (1 as libc::c_int) << 3 as libc::c_int
                            } else {
                                0 as libc::c_int
                            }),
                        rep_cnt,
                    );
                    revto(cx, cy);
                    current_block_290 = 14203403603292796089;
                    break;
                }
                97 => {
                    (*markdata).append_mode = 1 as libc::c_int - (*markdata).append_mode;
                    LMsg(
                        0 as libc::c_int,
                        if (*markdata).append_mode != 0 {
                            b":set append\0" as *const u8 as *const libc::c_char
                        } else {
                            b":set noappend\0" as *const u8 as *const libc::c_char
                        },
                    );
                    current_block_290 = 14203403603292796089;
                    break;
                }
                118 | 86 => {
                    if (*markdata).left_mar == 8 as libc::c_int {
                        rep_cnt = 1 as libc::c_int;
                    } else {
                        rep_cnt = 9 as libc::c_int;
                    }
                    current_block_290 = 7725111741901194009;
                    break;
                }
                99 | 67 => {
                    current_block_290 = 7725111741901194009;
                    break;
                }
                74 => {
                    (*markdata)
                        .nonl = ((*markdata).nonl + 1 as libc::c_int) % 4 as libc::c_int;
                    match (*markdata).nonl {
                        0 => {
                            if join_with_cr != 0 {
                                LMsg(
                                    0 as libc::c_int,
                                    b"Multiple lines (CR/LF)\0" as *const u8
                                        as *const libc::c_char,
                                );
                            } else {
                                LMsg(
                                    0 as libc::c_int,
                                    b"Multiple lines (LF)\0" as *const u8 as *const libc::c_char,
                                );
                            }
                        }
                        1 => {
                            LMsg(
                                0 as libc::c_int,
                                b"Lines joined\0" as *const u8 as *const libc::c_char,
                            );
                        }
                        2 => {
                            LMsg(
                                0 as libc::c_int,
                                b"Lines joined with blanks\0" as *const u8
                                    as *const libc::c_char,
                            );
                        }
                        3 => {
                            LMsg(
                                0 as libc::c_int,
                                b"Lines joined with comma\0" as *const u8
                                    as *const libc::c_char,
                            );
                        }
                        _ => {}
                    }
                    current_block_290 = 14203403603292796089;
                    break;
                }
                47 => {
                    Search(1 as libc::c_int);
                    in_mark = 0 as libc::c_int;
                    current_block_290 = 14203403603292796089;
                    break;
                }
                63 => {
                    Search(-(1 as libc::c_int));
                    in_mark = 0 as libc::c_int;
                    current_block_290 = 14203403603292796089;
                    break;
                }
                110 => {
                    Search(0 as libc::c_int);
                    current_block_290 = 14203403603292796089;
                    break;
                }
                78 => {
                    (*markdata).isdir = -(*markdata).isdir;
                    Search(0 as libc::c_int);
                    (*markdata).isdir = -(*markdata).isdir;
                    current_block_290 = 14203403603292796089;
                    break;
                }
                121 | 89 => {
                    if (*markdata).second == 0 as libc::c_int {
                        revto(linestart(cy), cy);
                        (*markdata).second += 1;
                        (*markdata).second;
                        (*markdata).x1 = (*markdata).cx;
                        cx = (*markdata).x1;
                        (*markdata).y1 = (*markdata).cy;
                        cy = (*markdata).y1;
                    }
                    rep_cnt -= 1;
                    if rep_cnt > 0 as libc::c_int {
                        revto(cx, cy + rep_cnt);
                    }
                    revto(lineend((*markdata).cy), (*markdata).cy);
                    if od == 'y' as i32 {
                        current_block_290 = 14203403603292796089;
                        break;
                    } else {
                        current_block_290 = 16313690934366579767;
                        break;
                    }
                }
                87 => {
                    current_block_290 = 16313690934366579767;
                    break;
                }
                65 => {
                    current_block_290 = 14734356657960272782;
                    break;
                }
                62 => {
                    current_block_290 = 6638457178241141462;
                    break;
                }
                32 | 13 => {
                    current_block_290 = 8536254371945452008;
                    break;
                }
                146 => {
                    if (*flayer).l_mouseevent.start != 0 {
                        let mut button: libc::c_int = (*flayer)
                            .l_mouseevent
                            .buffer[0 as libc::c_int as usize] as libc::c_int;
                        if button == 'a' as i32 {
                            od = 'j' as i32;
                        } else if button == '`' as i32 {
                            od = 'k' as i32;
                        } else if button == ' ' as i32 {
                            cx = (*flayer).l_mouseevent.buffer[1 as libc::c_int as usize]
                                as libc::c_int;
                            cy = (*flayer).l_mouseevent.buffer[2 as libc::c_int as usize]
                                as libc::c_int + (*markdata).hist_offset;
                            revto(cx, cy);
                            od = ' ' as i32;
                        } else {
                            od = 0 as libc::c_int;
                        }
                        LayProcessMouseSwitch(flayer, 0 as libc::c_int);
                        if !(od != 0) {
                            current_block_290 = 14203403603292796089;
                            break;
                        }
                    } else {
                        LayProcessMouseSwitch(flayer, 1 as libc::c_int);
                        current_block_290 = 14203403603292796089;
                        break;
                    }
                }
                _ => {
                    MarkAbort();
                    LMsg(
                        0 as libc::c_int,
                        b"Copy mode aborted\0" as *const u8 as *const libc::c_char,
                    );
                    in_mark = 0 as libc::c_int;
                    current_block_290 = 14203403603292796089;
                    break;
                }
            }
        }
        match current_block_290 {
            9158226259752878651 => {
                if rep_cnt == 0 as libc::c_int {
                    rep_cnt = (*fore).w_histheight + (*fore).w_layer.l_height;
                }
                rep_cnt -= 1;
                revto_line(
                    (*markdata).left_mar,
                    rep_cnt,
                    ((*fore).w_layer.l_height - 1 as libc::c_int) / 2 as libc::c_int,
                );
                current_block_290 = 14203403603292796089;
            }
            7725111741901194009 => {
                if (*markdata).second != 0 {
                    rem(
                        (*markdata).x1,
                        (*markdata).y1,
                        cx,
                        cy,
                        1 as libc::c_int,
                        0 as *mut libc::c_char,
                        (*fore).w_layer.l_height - 1 as libc::c_int,
                    );
                    (*markdata).second = 1 as libc::c_int;
                }
                rep_cnt -= 1;
                rep_cnt;
                if rep_cnt < 0 as libc::c_int {
                    rep_cnt = cx;
                }
                if od != 'C' as i32 {
                    (*markdata).left_mar = rep_cnt;
                    if (*markdata).left_mar > (*markdata).right_mar {
                        (*markdata).left_mar = (*markdata).right_mar;
                    }
                } else {
                    (*markdata).right_mar = rep_cnt;
                    if (*markdata).left_mar > (*markdata).right_mar {
                        (*markdata).right_mar = (*markdata).left_mar;
                    }
                }
                if (*markdata).second != 0 {
                    (*markdata).cx = (*markdata).x1;
                    (*markdata).cy = (*markdata).y1;
                    revto(cx, cy);
                }
                if od == 'v' as i32 || od == 'V' as i32 {
                    LMsg(
                        0 as libc::c_int,
                        if (*markdata).left_mar != 8 as libc::c_int {
                            b":set nonu\0" as *const u8 as *const libc::c_char
                        } else {
                            b":set nu\0" as *const u8 as *const libc::c_char
                        },
                    );
                }
                current_block_290 = 14203403603292796089;
            }
            16313690934366579767 => {
                if od == 'W' as i32 {
                    if rep_cnt == 0 as libc::c_int {
                        rep_cnt = 1 as libc::c_int;
                    }
                    if (*markdata).second == 0 {
                        nextword(
                            &mut cx,
                            &mut cy,
                            (1 as libc::c_int) << 0 as libc::c_int
                                | (1 as libc::c_int) << 1 as libc::c_int,
                            1 as libc::c_int,
                        );
                        revto(cx, cy);
                        (*markdata).second += 1;
                        (*markdata).second;
                        (*markdata).x1 = (*markdata).cx;
                        cx = (*markdata).x1;
                        (*markdata).y1 = (*markdata).cy;
                        cy = (*markdata).y1;
                    }
                    nextword(
                        &mut cx,
                        &mut cy,
                        (1 as libc::c_int) << 1 as libc::c_int,
                        rep_cnt,
                    );
                    revto(cx, cy);
                }
                cx = (*markdata).cx;
                cy = (*markdata).cy;
                current_block_290 = 14734356657960272782;
            }
            6888191626228944686 => {
                if rep_cnt == 0 as libc::c_int {
                    rep_cnt = 1 as libc::c_int;
                }
                revto(cx, cy - rep_cnt);
                current_block_290 = 14203403603292796089;
            }
            3389598844371035468 => {
                if rep_cnt == 0 as libc::c_int {
                    rep_cnt = 1 as libc::c_int;
                }
                revto(cx, cy + rep_cnt);
                current_block_290 = 14203403603292796089;
            }
            3793519976100317671 => {
                if rep_cnt == 0 as libc::c_int {
                    rep_cnt = 1 as libc::c_int;
                }
                revto(cx - rep_cnt, cy);
                current_block_290 = 14203403603292796089;
            }
            1356832168064818221 => {
                (*markdata).cx = (*markdata).x1;
                (*markdata).cy = (*markdata).y1;
                (*markdata).x1 = cx;
                (*markdata).y1 = cy;
                revto((*markdata).cx, (*markdata).cy);
                current_block_290 = 14203403603292796089;
            }
            980989089337379490 => {
                if rep_cnt == 0 {
                    rep_cnt = 1 as libc::c_int;
                }
                nextchar(
                    &mut cx,
                    &mut cy,
                    if od == ';' as i32 {
                        (*markdata).f_cmd.direction
                    } else {
                        (*markdata).f_cmd.direction ^ 0x20 as libc::c_int
                    },
                    (*markdata).f_cmd.target as libc::c_char,
                    rep_cnt,
                );
                revto(cx, cy);
                current_block_290 = 14203403603292796089;
            }
            11072814364684381342 => {
                current_block_290 = 2757429485286371073;
            }
            _ => {}
        }
        match current_block_290 {
            2757429485286371073 => {
                (*markdata).f_cmd.flag = 1 as libc::c_int;
                (*markdata).f_cmd.direction = od;
                continue;
            }
            14734356657960272782 => {
                if od == 'A' as i32 {
                    (*markdata).append_mode = 1 as libc::c_int;
                }
                current_block_290 = 6638457178241141462;
            }
            _ => {}
        }
        match current_block_290 {
            6638457178241141462 => {
                if od == '>' as i32 {
                    (*markdata).write_buffer = 1 as libc::c_int;
                }
                current_block_290 = 8536254371945452008;
            }
            _ => {}
        }
        match current_block_290 {
            8536254371945452008 => {
                if (*markdata).second == 0 {
                    (*markdata).second += 1;
                    (*markdata).second;
                    (*markdata).x1 = cx;
                    (*markdata).y1 = cy;
                    revto(cx, cy);
                    LMsg(
                        0 as libc::c_int,
                        b"First mark set - Column %d Line %d\0" as *const u8
                            as *const libc::c_char,
                        cx + 1 as libc::c_int,
                        cy - (*markdata).hist_offset + 1 as libc::c_int,
                    );
                } else {
                    let mut append_mode: libc::c_int = (*markdata).append_mode;
                    let mut write_buffer: libc::c_int = (*markdata).write_buffer;
                    x2 = cx;
                    y2 = cy;
                    newcopylen = rem(
                        (*markdata).x1,
                        (*markdata).y1,
                        x2,
                        y2,
                        2 as libc::c_int,
                        0 as *mut libc::c_char,
                        0 as libc::c_int,
                    );
                    if !((*md_user).u_plop.buf).is_null() && append_mode == 0 {
                        UserFreeCopyBuffer(md_user);
                    }
                    yend = (*fore).w_layer.l_height - 1 as libc::c_int;
                    if (*fore).w_histheight - (*markdata).hist_offset
                        < (*fore).w_layer.l_height
                    {
                        (*markdata).second = 0 as libc::c_int;
                        yend
                            -= MarkScrollUpDisplay(
                                (*fore).w_histheight - (*markdata).hist_offset,
                            );
                    }
                    if newcopylen > 0 as libc::c_int {
                        if !((*md_user).u_plop.buf).is_null() {
                            (*md_user)
                                .u_plop
                                .buf = realloc(
                                (*md_user).u_plop.buf as *mut libc::c_void,
                                ((*md_user).u_plop.len + newcopylen + 3 as libc::c_int)
                                    as libc::c_uint as libc::c_ulong,
                            ) as *mut libc::c_char;
                        } else {
                            (*md_user).u_plop.len = 0 as libc::c_int;
                            (*md_user)
                                .u_plop
                                .buf = malloc(
                                (newcopylen + 3 as libc::c_int) as libc::c_uint
                                    as libc::c_ulong,
                            ) as *mut libc::c_char;
                        }
                        if ((*md_user).u_plop.buf).is_null() {
                            MarkAbort();
                            in_mark = 0 as libc::c_int;
                            LMsg(
                                0 as libc::c_int,
                                b"Not enough memory... Sorry.\0" as *const u8
                                    as *const libc::c_char,
                            );
                            (*md_user).u_plop.len = 0 as libc::c_int;
                            (*md_user).u_plop.buf = 0 as *mut libc::c_char;
                            current_block_290 = 14203403603292796089;
                        } else {
                            if append_mode != 0 {
                                match (*markdata).nonl {
                                    0 => {
                                        if join_with_cr != 0 {
                                            *((*md_user).u_plop.buf)
                                                .offset(
                                                    (*md_user).u_plop.len as isize,
                                                ) = '\r' as i32 as libc::c_char;
                                            (*md_user).u_plop.len += 1;
                                            (*md_user).u_plop.len;
                                        }
                                        *((*md_user).u_plop.buf)
                                            .offset(
                                                (*md_user).u_plop.len as isize,
                                            ) = '\n' as i32 as libc::c_char;
                                        (*md_user).u_plop.len += 1;
                                        (*md_user).u_plop.len;
                                    }
                                    2 => {
                                        *((*md_user).u_plop.buf)
                                            .offset(
                                                (*md_user).u_plop.len as isize,
                                            ) = ' ' as i32 as libc::c_char;
                                        (*md_user).u_plop.len += 1;
                                        (*md_user).u_plop.len;
                                    }
                                    3 => {
                                        *((*md_user).u_plop.buf)
                                            .offset(
                                                (*md_user).u_plop.len as isize,
                                            ) = ',' as i32 as libc::c_char;
                                        (*md_user).u_plop.len += 1;
                                        (*md_user).u_plop.len;
                                    }
                                    1 | _ => {}
                                }
                            }
                            (*md_user).u_plop.len
                                += rem(
                                    (*markdata).x1,
                                    (*markdata).y1,
                                    x2,
                                    y2,
                                    ((*markdata).hist_offset == (*fore).w_histheight)
                                        as libc::c_int,
                                    ((*md_user).u_plop.buf)
                                        .offset((*md_user).u_plop.len as isize),
                                    yend,
                                );
                            (*md_user).u_plop.enc = (*fore).w_layer.l_encoding;
                            current_block_290 = 1707335883933721018;
                        }
                    } else {
                        current_block_290 = 1707335883933721018;
                    }
                    match current_block_290 {
                        14203403603292796089 => {}
                        _ => {
                            if (*markdata).hist_offset != (*fore).w_histheight {
                                let mut oldlay: *mut layer = flayer;
                                let mut oldcvlist: *mut canvas = 0 as *mut canvas;
                                let mut cv: *mut canvas = 0 as *mut canvas;
                                flayer = (*flayer).l_next;
                                oldcvlist = (*flayer).l_cvlist;
                                (*flayer).l_cvlist = (*oldlay).l_cvlist;
                                cv = (*flayer).l_cvlist;
                                while !cv.is_null() {
                                    (*cv).c_layer = flayer;
                                    cv = (*cv).c_lnext;
                                }
                                LRefreshAll(flayer, 0 as libc::c_int);
                                flayer = oldlay;
                                cv = (*flayer).l_cvlist;
                                while !cv.is_null() {
                                    (*cv).c_layer = flayer;
                                    cv = (*cv).c_lnext;
                                }
                                (*(*flayer).l_next).l_cvlist = oldcvlist;
                            }
                            ExitOverlayPage();
                            WindowChanged(fore, 'P' as i32);
                            if append_mode != 0 {
                                LMsg(
                                    0 as libc::c_int,
                                    b"Appended %d characters to buffer\0" as *const u8
                                        as *const libc::c_char,
                                    newcopylen,
                                );
                            } else {
                                LMsg(
                                    0 as libc::c_int,
                                    b"Copied %d characters into buffer\0" as *const u8
                                        as *const libc::c_char,
                                    (*md_user).u_plop.len,
                                );
                            }
                            if write_buffer != 0 {
                                WriteFile(
                                    md_user,
                                    0 as *mut libc::c_char,
                                    2 as libc::c_int,
                                );
                            }
                            in_mark = 0 as libc::c_int;
                        }
                    }
                }
            }
            _ => {}
        }
        if in_mark != 0 {
            (*markdata).rep_cnt = 0 as libc::c_int;
        }
    }
    if in_mark != 0 {
        (*flayer).l_x = (*markdata).cx;
        (*flayer).l_y = (*markdata).cy - (*markdata).hist_offset;
    }
    *inbufp = pt;
    *inlenp = inlen;
}
pub unsafe extern "C" fn revto(mut tx: libc::c_int, mut ty: libc::c_int) {
    revto_line(tx, ty, -(1 as libc::c_int));
}
pub unsafe extern "C" fn revto_line(
    mut tx: libc::c_int,
    mut ty: libc::c_int,
    mut line: libc::c_int,
) {
    let mut fx: libc::c_int = 0;
    let mut fy: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut revst: libc::c_int = 0;
    let mut reven: libc::c_int = 0;
    let mut qq: libc::c_int = 0;
    let mut ff: libc::c_int = 0;
    let mut tt: libc::c_int = 0;
    let mut st: libc::c_int = 0;
    let mut en: libc::c_int = 0;
    let mut ce: libc::c_int = 0 as libc::c_int;
    let mut ystart: libc::c_int = 0 as libc::c_int;
    let mut yend: libc::c_int = (*fore).w_layer.l_height - 1 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut ry: libc::c_int = 0;
    let mut wi: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut ml: *mut mline = 0 as *mut mline;
    let mut mc: mchar = mchar {
        image: 0,
        attr: 0,
        font: 0,
        fontx: 0,
        color: 0,
        mbcs: 0,
    };
    if tx < 0 as libc::c_int {
        tx = 0 as libc::c_int;
    } else if tx > (*fore).w_layer.l_width - 1 as libc::c_int {
        tx = (*fore).w_layer.l_width - 1 as libc::c_int;
    }
    if ty < (*fore).w_histheight - (*fore).w_scrollback_height {
        ty = (*fore).w_histheight - (*fore).w_scrollback_height;
    } else if ty > (*fore).w_histheight + (*fore).w_layer.l_height - 1 as libc::c_int {
        ty = (*fore).w_histheight + (*fore).w_layer.l_height - 1 as libc::c_int;
    }
    fx = (*markdata).cx;
    fy = (*markdata).cy;
    ml = if ty < (*fore).w_histheight {
        &mut *((*fore).w_hlines)
            .offset((((*fore).w_histidx + ty) % (*fore).w_histheight) as isize)
            as *mut mline
    } else {
        &mut *((*fore).w_mlines).offset((ty - (*fore).w_histheight) as isize)
            as *mut mline
    };
    if ty == fy && fx + 1 as libc::c_int == tx
        && (if (*fore).w_layer.l_encoding == 8 as libc::c_int {
            (*((*ml).font).offset(tx as isize) as libc::c_int == 0xff as libc::c_int
                && *((*ml).image).offset(tx as isize) as libc::c_int
                    == 0xff as libc::c_int) as libc::c_int
        } else {
            (*((*ml).font).offset(tx as isize) as libc::c_int & 0xe0 as libc::c_int
                == 0x80 as libc::c_int) as libc::c_int
        }) != 0 && tx < (*display).d_width - 1 as libc::c_int
    {
        tx += 1;
        tx;
    }
    if ty == fy && fx - 1 as libc::c_int == tx
        && (if (*fore).w_layer.l_encoding == 8 as libc::c_int {
            (*((*ml).font).offset(fx as isize) as libc::c_int == 0xff as libc::c_int
                && *((*ml).image).offset(fx as isize) as libc::c_int
                    == 0xff as libc::c_int) as libc::c_int
        } else {
            (*((*ml).font).offset(fx as isize) as libc::c_int & 0xe0 as libc::c_int
                == 0x80 as libc::c_int) as libc::c_int
        }) != 0 && tx != 0
    {
        tx -= 1;
        tx;
    }
    (*markdata).cx = tx;
    (*markdata).cy = ty;
    i = 0 as libc::c_int;
    if line >= 0 as libc::c_int && line < (*fore).w_layer.l_height {
        i = ty - (*markdata).hist_offset - line;
    } else if ty < (*markdata).hist_offset {
        i = ty - (*markdata).hist_offset;
    } else if ty
        > (*markdata).hist_offset + ((*fore).w_layer.l_height - 1 as libc::c_int)
    {
        i = ty - (*markdata).hist_offset - ((*fore).w_layer.l_height - 1 as libc::c_int);
    }
    if i > 0 as libc::c_int {
        yend -= MarkScrollUpDisplay(i);
    } else if i < 0 as libc::c_int {
        ystart += MarkScrollDownDisplay(-i);
    }
    if (*markdata).second == 0 as libc::c_int {
        (*flayer).l_x = tx;
        (*flayer).l_y = ty - (*markdata).hist_offset;
        LGotoPos(flayer, tx, ty - (*markdata).hist_offset);
        return;
    }
    qq = (*markdata).x1 + (*markdata).y1 * (*fore).w_layer.l_width;
    ff = fx + fy * (*fore).w_layer.l_width;
    tt = tx + ty * (*fore).w_layer.l_width;
    if ff > tt {
        st = tt;
        en = ff;
        x = tx;
        y = ty;
    } else {
        st = ff;
        en = tt;
        x = fx;
        y = fy;
    }
    if st > qq {
        st += 1;
        st;
        x += 1;
        x;
    }
    if en < qq {
        en -= 1;
        en;
    }
    if tt > qq {
        revst = qq;
        reven = tt;
    } else {
        revst = tt;
        reven = qq;
    }
    ry = y - (*markdata).hist_offset;
    if ry < ystart {
        y += ystart - ry;
        x = 0 as libc::c_int;
        st = y * (*fore).w_layer.l_width;
        ry = ystart;
    }
    ml = if y < (*fore).w_histheight {
        &mut *((*fore).w_hlines)
            .offset((((*fore).w_histidx + y) % (*fore).w_histheight) as isize)
            as *mut mline
    } else {
        &mut *((*fore).w_mlines).offset((y - (*fore).w_histheight) as isize)
            as *mut mline
    };
    t = st;
    while t <= en {
        if x >= (*fore).w_layer.l_width {
            x = 0 as libc::c_int;
            y += 1;
            y;
            ry += 1;
            ry;
            ml = if y < (*fore).w_histheight {
                &mut *((*fore).w_hlines)
                    .offset((((*fore).w_histidx + y) % (*fore).w_histheight) as isize)
                    as *mut mline
            } else {
                &mut *((*fore).w_mlines).offset((y - (*fore).w_histheight) as isize)
                    as *mut mline
            };
        }
        if ry > yend {
            break;
        }
        if t == st || x == 0 as libc::c_int {
            wi = ((*ml).image).offset((*fore).w_layer.l_width as isize);
            ce = (*fore).w_layer.l_width;
            while ce >= 0 as libc::c_int {
                if *wi as libc::c_int != ' ' as i32 {
                    break;
                }
                ce -= 1;
                ce;
                wi = wi.offset(-1);
                wi;
            }
        }
        if x <= ce && x >= (*markdata).left_mar && x <= (*markdata).right_mar {
            if if (*fore).w_layer.l_encoding == 8 as libc::c_int {
                (*((*ml).font).offset(x as isize) as libc::c_int == 0xff as libc::c_int
                    && *((*ml).image).offset(x as isize) as libc::c_int
                        == 0xff as libc::c_int) as libc::c_int
            } else {
                (*((*ml).font).offset(x as isize) as libc::c_int & 0xe0 as libc::c_int
                    == 0x80 as libc::c_int) as libc::c_int
            } != 0
            {
                if t == revst {
                    revst -= 1;
                    revst;
                }
                t -= 1;
                t;
                x -= 1;
                x;
            }
            if t >= revst && t <= reven {
                mc = mchar_so;
                if pastefont != 0 {
                    mc.font = *((*ml).font).offset(x as isize);
                    mc.fontx = *((*ml).fontx).offset(x as isize);
                }
                mc.image = *((*ml).image).offset(x as isize);
            } else {
                mc.image = *((*ml).image).offset(x as isize);
                mc.attr = *((*ml).attr).offset(x as isize);
                mc.font = *((*ml).font).offset(x as isize);
                mc.fontx = *((*ml).fontx).offset(x as isize);
                mc.color = *((*ml).color).offset(x as isize);
                mc.mbcs = 0 as libc::c_int as libc::c_uchar;
            }
            if if (*fore).w_layer.l_encoding == 8 as libc::c_int {
                (*((*ml).font).offset((x + 1 as libc::c_int) as isize) as libc::c_int
                    == 0xff as libc::c_int
                    && *((*ml).image).offset((x + 1 as libc::c_int) as isize)
                        as libc::c_int == 0xff as libc::c_int) as libc::c_int
            } else {
                (*((*ml).font).offset(x as isize) as libc::c_int & 0x1f as libc::c_int
                    != 0 as libc::c_int
                    && *((*ml).font).offset(x as isize) as libc::c_int
                        & 0xe0 as libc::c_int == 0 as libc::c_int) as libc::c_int
            } != 0
            {
                mc.mbcs = *((*ml).image).offset((x + 1 as libc::c_int) as isize);
                LPutChar(flayer, &mut mc, x, y - (*markdata).hist_offset);
                t += 1;
                t;
            }
            LPutChar(flayer, &mut mc, x, y - (*markdata).hist_offset);
            if if (*fore).w_layer.l_encoding == 8 as libc::c_int {
                (*((*ml).font).offset((x + 1 as libc::c_int) as isize) as libc::c_int
                    == 0xff as libc::c_int
                    && *((*ml).image).offset((x + 1 as libc::c_int) as isize)
                        as libc::c_int == 0xff as libc::c_int) as libc::c_int
            } else {
                (*((*ml).font).offset(x as isize) as libc::c_int & 0x1f as libc::c_int
                    != 0 as libc::c_int
                    && *((*ml).font).offset(x as isize) as libc::c_int
                        & 0xe0 as libc::c_int == 0 as libc::c_int) as libc::c_int
            } != 0
            {
                x += 1;
                x;
            }
        }
        t += 1;
        t;
        x += 1;
        x;
    }
    (*flayer).l_x = tx;
    (*flayer).l_y = ty - (*markdata).hist_offset;
    LGotoPos(flayer, tx, ty - (*markdata).hist_offset);
}
unsafe extern "C" fn MarkAbort() {
    let mut yend: libc::c_int = 0;
    let mut redisp: libc::c_int = 0;
    markdata = (*flayer).l_data as *mut markdata;
    fore = (*markdata).md_window;
    yend = (*fore).w_layer.l_height - 1 as libc::c_int;
    redisp = (*markdata).second;
    if (*fore).w_histheight - (*markdata).hist_offset < (*fore).w_layer.l_height {
        (*markdata).second = 0 as libc::c_int;
        yend -= MarkScrollUpDisplay((*fore).w_histheight - (*markdata).hist_offset);
    }
    if (*markdata).hist_offset != (*fore).w_histheight {
        let mut oldlay: *mut layer = flayer;
        let mut oldcvlist: *mut canvas = 0 as *mut canvas;
        let mut cv: *mut canvas = 0 as *mut canvas;
        flayer = (*flayer).l_next;
        oldcvlist = (*flayer).l_cvlist;
        (*flayer).l_cvlist = (*oldlay).l_cvlist;
        cv = (*flayer).l_cvlist;
        while !cv.is_null() {
            (*cv).c_layer = flayer;
            cv = (*cv).c_lnext;
        }
        LRefreshAll(flayer, 0 as libc::c_int);
        flayer = oldlay;
        cv = (*flayer).l_cvlist;
        while !cv.is_null() {
            (*cv).c_layer = flayer;
            cv = (*cv).c_lnext;
        }
        (*(*flayer).l_next).l_cvlist = oldcvlist;
    } else {
        rem(
            (*markdata).x1,
            (*markdata).y1,
            (*markdata).cx,
            (*markdata).cy,
            redisp,
            0 as *mut libc::c_char,
            yend,
        );
    }
    ExitOverlayPage();
    WindowChanged(fore, 'P' as i32);
}
unsafe extern "C" fn MarkRedisplayLine(
    mut y: libc::c_int,
    mut xs: libc::c_int,
    mut xe: libc::c_int,
    mut isblank: libc::c_int,
) {
    let mut wy: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut rm: libc::c_int = 0;
    let mut sta: libc::c_int = 0;
    let mut sto: libc::c_int = 0;
    let mut cp: libc::c_int = 0;
    let mut wi: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut ml: *mut mline = 0 as *mut mline;
    let mut mchar_marked: mchar = mchar {
        image: 0,
        attr: 0,
        font: 0,
        fontx: 0,
        color: 0,
        mbcs: 0,
    };
    if y < 0 as libc::c_int {
        return;
    }
    markdata = (*flayer).l_data as *mut markdata;
    fore = (*markdata).md_window;
    mchar_marked = mchar_so;
    wy = y + (*markdata).hist_offset;
    ml = if wy < (*fore).w_histheight {
        &mut *((*fore).w_hlines)
            .offset((((*fore).w_histidx + wy) % (*fore).w_histheight) as isize)
            as *mut mline
    } else {
        &mut *((*fore).w_mlines).offset((wy - (*fore).w_histheight) as isize)
            as *mut mline
    };
    if (*markdata).second == 0 as libc::c_int {
        if (if (*fore).w_layer.l_encoding == 8 as libc::c_int {
            (*((*ml).font).offset(xs as isize) as libc::c_int == 0xff as libc::c_int
                && *((*ml).image).offset(xs as isize) as libc::c_int
                    == 0xff as libc::c_int) as libc::c_int
        } else {
            (*((*ml).font).offset(xs as isize) as libc::c_int & 0xe0 as libc::c_int
                == 0x80 as libc::c_int) as libc::c_int
        }) != 0 && xs > 0 as libc::c_int
        {
            xs -= 1;
            xs;
        }
        if (if (*fore).w_layer.l_encoding == 8 as libc::c_int {
            (*((*ml).font).offset((xe + 1 as libc::c_int) as isize) as libc::c_int
                == 0xff as libc::c_int
                && *((*ml).image).offset((xe + 1 as libc::c_int) as isize) as libc::c_int
                    == 0xff as libc::c_int) as libc::c_int
        } else {
            (*((*ml).font).offset(xe as isize) as libc::c_int & 0x1f as libc::c_int
                != 0 as libc::c_int
                && *((*ml).font).offset(xe as isize) as libc::c_int & 0xe0 as libc::c_int
                    == 0 as libc::c_int) as libc::c_int
        }) != 0 && xe < (*fore).w_layer.l_width - 1 as libc::c_int
        {
            xe += 1;
            xe;
        }
        if xs == 0 as libc::c_int && y > 0 as libc::c_int && wy > 0 as libc::c_int
            && *((*(if (wy - 1 as libc::c_int) < (*fore).w_histheight {
                &mut *((*fore).w_hlines)
                    .offset(
                        (((*fore).w_histidx + wy - 1 as libc::c_int)
                            % (*fore).w_histheight) as isize,
                    ) as *mut mline
            } else {
                &mut *((*fore).w_mlines)
                    .offset((wy - 1 as libc::c_int - (*fore).w_histheight) as isize)
                    as *mut mline
            }))
                .image)
                .offset((*flayer).l_width as isize) as libc::c_int == 0 as libc::c_int
        {
            LCDisplayLineWrap(flayer, ml, y, xs, xe, isblank);
        } else {
            LCDisplayLine(flayer, ml, y, xs, xe, isblank);
        }
        return;
    }
    sta = (*markdata).y1 * (*fore).w_layer.l_width + (*markdata).x1;
    sto = (*markdata).cy * (*fore).w_layer.l_width + (*markdata).cx;
    if sta > sto {
        i = sta;
        sta = sto;
        sto = i;
    }
    cp = wy * (*fore).w_layer.l_width + xs;
    rm = (*markdata).right_mar;
    x = (*fore).w_layer.l_width;
    wi = ((*ml).image).offset((*fore).w_layer.l_width as isize);
    while x >= 0 as libc::c_int {
        if *wi as libc::c_int != ' ' as i32 {
            break;
        }
        x -= 1;
        x;
        wi = wi.offset(-1);
        wi;
    }
    if x < rm {
        rm = x;
    }
    x = xs;
    while x <= xe {
        if cp >= sta && x >= (*markdata).left_mar {
            break;
        }
        x += 1;
        x;
        cp += 1;
        cp;
    }
    if if (*fore).w_layer.l_encoding == 8 as libc::c_int {
        (*((*ml).font).offset(x as isize) as libc::c_int == 0xff as libc::c_int
            && *((*ml).image).offset(x as isize) as libc::c_int == 0xff as libc::c_int)
            as libc::c_int
    } else {
        (*((*ml).font).offset(x as isize) as libc::c_int & 0xe0 as libc::c_int
            == 0x80 as libc::c_int) as libc::c_int
    } != 0
    {
        x -= 1;
        x;
    }
    if x > xs {
        LCDisplayLine(flayer, ml, y, xs, x - 1 as libc::c_int, isblank);
    }
    while x <= xe {
        if cp > sto || x > rm {
            break;
        }
        if pastefont != 0 {
            mchar_marked.font = *((*ml).font).offset(x as isize);
            mchar_marked.fontx = *((*ml).fontx).offset(x as isize);
        }
        mchar_marked.image = *((*ml).image).offset(x as isize);
        mchar_marked.mbcs = 0 as libc::c_int as libc::c_uchar;
        if if (*fore).w_layer.l_encoding == 8 as libc::c_int {
            (*((*ml).font).offset((x + 1 as libc::c_int) as isize) as libc::c_int
                == 0xff as libc::c_int
                && *((*ml).image).offset((x + 1 as libc::c_int) as isize) as libc::c_int
                    == 0xff as libc::c_int) as libc::c_int
        } else {
            (*((*ml).font).offset(x as isize) as libc::c_int & 0x1f as libc::c_int
                != 0 as libc::c_int
                && *((*ml).font).offset(x as isize) as libc::c_int & 0xe0 as libc::c_int
                    == 0 as libc::c_int) as libc::c_int
        } != 0
        {
            mchar_marked.mbcs = *((*ml).image).offset((x + 1 as libc::c_int) as isize);
            cp += 1;
            cp;
        }
        LPutChar(flayer, &mut mchar_marked, x, y);
        if if (*fore).w_layer.l_encoding == 8 as libc::c_int {
            (*((*ml).font).offset((x + 1 as libc::c_int) as isize) as libc::c_int
                == 0xff as libc::c_int
                && *((*ml).image).offset((x + 1 as libc::c_int) as isize) as libc::c_int
                    == 0xff as libc::c_int) as libc::c_int
        } else {
            (*((*ml).font).offset(x as isize) as libc::c_int & 0x1f as libc::c_int
                != 0 as libc::c_int
                && *((*ml).font).offset(x as isize) as libc::c_int & 0xe0 as libc::c_int
                    == 0 as libc::c_int) as libc::c_int
        } != 0
        {
            x += 1;
            x;
        }
        x += 1;
        x;
        cp += 1;
        cp;
    }
    if x <= xe {
        LCDisplayLine(flayer, ml, y, x, xe, isblank);
    }
}
unsafe extern "C" fn MarkRewrite(
    mut ry: libc::c_int,
    mut xs: libc::c_int,
    mut xe: libc::c_int,
    mut rend: *mut mchar,
    mut doit: libc::c_int,
) -> libc::c_int {
    let mut dx: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut st: libc::c_int = 0;
    let mut en: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut rm: libc::c_int = 0;
    let mut i: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut ml: *mut mline = 0 as *mut mline;
    let mut mchar_marked: mchar = mchar {
        image: 0,
        attr: 0,
        font: 0,
        fontx: 0,
        color: 0,
        mbcs: 0,
    };
    mchar_marked = mchar_so;
    markdata = (*flayer).l_data as *mut markdata;
    fore = (*markdata).md_window;
    y = ry + (*markdata).hist_offset;
    ml = if y < (*fore).w_histheight {
        &mut *((*fore).w_hlines)
            .offset((((*fore).w_histidx + y) % (*fore).w_histheight) as isize)
            as *mut mline
    } else {
        &mut *((*fore).w_mlines).offset((y - (*fore).w_histheight) as isize)
            as *mut mline
    };
    if (*fore).w_layer.l_encoding != 0 && (*fore).w_layer.l_encoding != 8 as libc::c_int
        && (*display).d_encoding == 8 as libc::c_int
        && ContainsSpecialDeffont(ml, xs, xe, (*fore).w_layer.l_encoding) != 0
    {
        return 1000 as libc::c_int;
    }
    dx = xe - xs + 1 as libc::c_int;
    if doit != 0 {
        i = ((*ml).image).offset(xs as isize);
        loop {
            let fresh15 = dx;
            dx = dx - 1;
            if !(fresh15 != 0) {
                break;
            }
            let fresh16 = i;
            i = i.offset(1);
            PUTCHAR(*fresh16 as libc::c_int);
        }
        return 0 as libc::c_int;
    }
    if (*markdata).second == 0 as libc::c_int {
        en = -(1 as libc::c_int);
        st = en;
    } else {
        st = (*markdata).y1 * (*fore).w_layer.l_width + (*markdata).x1;
        en = (*markdata).cy * (*fore).w_layer.l_width + (*markdata).cx;
        if st > en {
            t = st;
            st = en;
            en = t;
        }
    }
    t = y * (*fore).w_layer.l_width + xs;
    rm = (*fore).w_layer.l_width;
    i = ((*ml).image).offset((*fore).w_layer.l_width as isize);
    while rm >= 0 as libc::c_int {
        let fresh17 = i;
        i = i.offset(-1);
        if *fresh17 as libc::c_int != ' ' as i32 {
            break;
        }
        rm -= 1;
        rm;
    }
    if rm > (*markdata).right_mar {
        rm = (*markdata).right_mar;
    }
    x = xs;
    loop {
        let fresh18 = dx;
        dx = dx - 1;
        if !(fresh18 != 0) {
            break;
        }
        if t >= st && t <= en && x >= (*markdata).left_mar && x <= rm {
            if pastefont != 0 {
                mchar_marked.font = *((*ml).font).offset(x as isize);
                mchar_marked.fontx = *((*ml).fontx).offset(x as isize);
            }
            (*rend).image = mchar_marked.image;
            if !((*rend).image as libc::c_int == mchar_marked.image as libc::c_int
                && (*rend).attr as libc::c_int == mchar_marked.attr as libc::c_int
                && (*rend).font as libc::c_int == mchar_marked.font as libc::c_int
                && (*rend).fontx as libc::c_int == mchar_marked.fontx as libc::c_int
                && (*rend).color as libc::c_int == mchar_marked.color as libc::c_int)
            {
                return 1000 as libc::c_int;
            }
        } else {
            (*rend).image = *((*ml).image).offset(x as isize);
            if !((*rend).image as libc::c_int
                == *((*ml).image).offset(x as isize) as libc::c_int
                && (*rend).attr as libc::c_int
                    == *((*ml).attr).offset(x as isize) as libc::c_int
                && (*rend).font as libc::c_int
                    == *((*ml).font).offset(x as isize) as libc::c_int
                && (*rend).fontx as libc::c_int
                    == *((*ml).fontx).offset(x as isize) as libc::c_int
                && (*rend).color as libc::c_int
                    == *((*ml).color).offset(x as isize) as libc::c_int)
            {
                return 1000 as libc::c_int;
            }
        }
        x += 1;
        x;
    }
    return xe - xs + 1 as libc::c_int;
}
unsafe extern "C" fn MarkScrollUpDisplay(mut n: libc::c_int) -> libc::c_int {
    let mut i: libc::c_int = 0;
    if n <= 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    if n > (*fore).w_histheight - (*markdata).hist_offset {
        n = (*fore).w_histheight - (*markdata).hist_offset;
    }
    (*markdata).hist_offset += n;
    i = if n < (*flayer).l_height { n } else { (*flayer).l_height };
    LScrollV(
        flayer,
        i,
        0 as libc::c_int,
        (*flayer).l_height - 1 as libc::c_int,
        0 as libc::c_int,
    );
    loop {
        let fresh19 = i;
        i = i - 1;
        if !(fresh19 > 0 as libc::c_int) {
            break;
        }
        MarkRedisplayLine(
            (*flayer).l_height - i - 1 as libc::c_int,
            0 as libc::c_int,
            (*flayer).l_width - 1 as libc::c_int,
            1 as libc::c_int,
        );
    }
    return n;
}
unsafe extern "C" fn MarkScrollDownDisplay(mut n: libc::c_int) -> libc::c_int {
    let mut i: libc::c_int = 0;
    if n <= 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    if n > (*markdata).hist_offset {
        n = (*markdata).hist_offset;
    }
    (*markdata).hist_offset -= n;
    i = if n < (*flayer).l_height { n } else { (*flayer).l_height };
    LScrollV(
        flayer,
        -i,
        0 as libc::c_int,
        (*fore).w_layer.l_height - 1 as libc::c_int,
        0 as libc::c_int,
    );
    loop {
        let fresh20 = i;
        i = i - 1;
        if !(fresh20 > 0 as libc::c_int) {
            break;
        }
        MarkRedisplayLine(
            i,
            0 as libc::c_int,
            (*flayer).l_width - 1 as libc::c_int,
            1 as libc::c_int,
        );
    }
    return n;
}
pub unsafe extern "C" fn MakePaster(
    mut pa: *mut paster,
    mut buf: *mut libc::c_char,
    mut len: libc::c_int,
    mut bufiscopy: libc::c_int,
) {
    FreePaster(pa);
    (*pa).pa_pasteptr = buf;
    (*pa).pa_pastelen = len;
    if bufiscopy != 0 {
        (*pa).pa_pastebuf = buf;
    }
    (*pa).pa_pastelayer = flayer;
    DoProcess(
        (*(*flayer).l_bottom).l_data as *mut win,
        &mut (*pa).pa_pasteptr,
        &mut (*pa).pa_pastelen,
        pa,
    );
}
pub unsafe extern "C" fn FreePaster(mut pa: *mut paster) {
    if !((*pa).pa_pastebuf).is_null() {
        free((*pa).pa_pastebuf as *mut libc::c_void);
    }
    (*pa).pa_pastebuf = 0 as *mut libc::c_char;
    (*pa).pa_pasteptr = 0 as *mut libc::c_char;
    (*pa).pa_pastelen = 0 as libc::c_int;
    (*pa).pa_pastelayer = 0 as *mut layer;
    evdeq(&mut (*pa).pa_slowev);
}
