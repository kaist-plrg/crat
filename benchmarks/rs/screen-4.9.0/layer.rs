use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type logfile;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn vsnprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn Msg(_: libc::c_int, _: *const libc::c_char, _: ...);
    fn PutWinMsg(_: *mut libc::c_char, _: libc::c_int, _: libc::c_int);
    fn SetFlow(_: libc::c_int);
    fn PUTCHARLP(_: libc::c_int);
    fn ClearArea(
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
    );
    fn ClearLine(
        _: *mut mline,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
    );
    fn RefreshArea(
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
    );
    fn RefreshLine(_: libc::c_int, _: libc::c_int, _: libc::c_int, _: libc::c_int);
    fn DisplayLine(
        _: *mut mline,
        _: *mut mline,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
    );
    fn GotoPos(_: libc::c_int, _: libc::c_int);
    fn ScrollH(
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: *mut mline,
    );
    fn ScrollV(
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
    );
    fn PutChar(_: *mut mchar, _: libc::c_int, _: libc::c_int);
    fn InsChar(
        _: *mut mchar,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: *mut mline,
    );
    fn WrapChar(
        _: *mut mchar,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
    );
    fn KeypadMode(_: libc::c_int);
    fn CursorkeysMode(_: libc::c_int);
    fn CursorVisibility(_: libc::c_int);
    fn MouseMode(_: libc::c_int);
    fn ExtMouseMode(_: libc::c_int);
    fn SetRendition(_: *mut mchar);
    fn MakeStatus(_: *mut libc::c_char);
    fn RemoveStatus();
    fn DoNLS(_: *const libc::c_char) -> *const libc::c_char;
    fn recode_mchar(_: *mut mchar, _: libc::c_int, _: libc::c_int) -> *mut mchar;
    fn recode_mline(
        _: *mut mline,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
    ) -> *mut mline;
    static mut display: *mut display;
    static mut displays: *mut display;
    static mut mline_blank: mline;
    static mut mline_null: mline;
    static mut mchar_blank: mchar;
    static mut flayer: *mut layer;
    static mut WinLf: LayFuncs;
    static mut BlankLf: LayFuncs;
    static mut layouts: *mut layout;
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
pub type __int32_t = libc::c_int;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type pid_t = __pid_t;
pub type int32_t = __int32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type va_list = __builtin_va_list;
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
unsafe extern "C" fn mloff(mut ml: *mut mline, mut off: libc::c_int) -> *mut mline {
    static mut mml: mline = mline {
        image: 0 as *const libc::c_uchar as *mut libc::c_uchar,
        attr: 0 as *const libc::c_uchar as *mut libc::c_uchar,
        font: 0 as *const libc::c_uchar as *mut libc::c_uchar,
        fontx: 0 as *const libc::c_uchar as *mut libc::c_uchar,
        color: 0 as *const libc::c_uchar as *mut libc::c_uchar,
    };
    if ml.is_null() {
        return 0 as *mut mline;
    }
    mml.image = ((*ml).image).offset(off as isize);
    mml.attr = ((*ml).attr).offset(off as isize);
    mml.font = ((*ml).font).offset(off as isize);
    mml.fontx = ((*ml).fontx).offset(off as isize);
    mml.color = ((*ml).color).offset(off as isize);
    return &mut mml;
}
pub unsafe extern "C" fn LGotoPos(
    mut l: *mut layer,
    mut x: libc::c_int,
    mut y: libc::c_int,
) {
    let mut cv: *mut canvas = 0 as *mut canvas;
    let mut vp: *mut viewport = 0 as *mut viewport;
    let mut x2: libc::c_int = 0;
    let mut y2: libc::c_int = 0;
    if ((*l).l_pause).d() != 0 {
        LayPauseUpdateRegion(l, x, x, y, y);
    }
    cv = (*l).l_cvlist;
    while !cv.is_null() {
        if !(((*l).l_pause).d() != 0 && (*cv).c_slorient != 0) {
            display = (*cv).c_display;
            if !((*display).d_blocked != 0) {
                if !(cv != (*display).d_forecv) {
                    x2 = x + (*cv).c_xoff;
                    y2 = y + (*cv).c_yoff;
                    if x2 < (*cv).c_xs {
                        x2 = (*cv).c_xs;
                    }
                    if y2 < (*cv).c_ys {
                        y2 = (*cv).c_ys;
                    }
                    if x2 > (*cv).c_xe {
                        x2 = (*cv).c_xe;
                    }
                    if y2 > (*cv).c_ye {
                        y2 = (*cv).c_ye;
                    }
                    vp = (*cv).c_vplist;
                    while !vp.is_null() {
                        if !(x2 < (*vp).v_xs || x2 > (*vp).v_xe) {
                            if !(y2 < (*vp).v_ys || y2 > (*vp).v_ye) {
                                GotoPos(x2, y2);
                                break;
                            }
                        }
                        vp = (*vp).v_next;
                    }
                }
            }
        }
        cv = (*cv).c_lnext;
    }
}
pub unsafe extern "C" fn LScrollH(
    mut l: *mut layer,
    mut n: libc::c_int,
    mut y: libc::c_int,
    mut xs: libc::c_int,
    mut xe: libc::c_int,
    mut bce: libc::c_int,
    mut ol: *mut mline,
) {
    let mut cv: *mut canvas = 0 as *mut canvas;
    let mut vp: *mut viewport = 0 as *mut viewport;
    let mut y2: libc::c_int = 0;
    let mut xs2: libc::c_int = 0;
    let mut xe2: libc::c_int = 0;
    if n == 0 as libc::c_int {
        return;
    }
    if ((*l).l_pause).d() != 0 {
        LayPauseUpdateRegion(l, xs, xe, y, y);
    }
    cv = (*l).l_cvlist;
    while !cv.is_null() {
        if !(((*l).l_pause).d() != 0 && (*cv).c_slorient != 0) {
            vp = (*cv).c_vplist;
            while !vp.is_null() {
                y2 = y + (*vp).v_yoff;
                if !(y2 < (*vp).v_ys || y2 > (*vp).v_ye) {
                    xs2 = xs + (*vp).v_xoff;
                    xe2 = xe + (*vp).v_xoff;
                    if xs2 < (*vp).v_xs {
                        xs2 = (*vp).v_xs;
                    }
                    if xe2 > (*vp).v_xe {
                        xe2 = (*vp).v_xe;
                    }
                    if !(xs2 > xe2) {
                        display = (*cv).c_display;
                        if !((*display).d_blocked != 0) {
                            ScrollH(
                                y2,
                                xs2,
                                xe2,
                                n,
                                bce,
                                if !ol.is_null() {
                                    mloff(ol, -(*vp).v_xoff)
                                } else {
                                    0 as *mut mline
                                },
                            );
                            if !(xe2 - xs2 == xe - xs) {
                                if n > 0 as libc::c_int {
                                    xs2 = xe2 + 1 as libc::c_int - n;
                                    xe2 = xe + (*vp).v_xoff - n;
                                } else {
                                    xe2 = xs2 - 1 as libc::c_int - n;
                                    xs2 = xs + (*vp).v_xoff - n;
                                }
                                if xs2 < (*vp).v_xs {
                                    xs2 = (*vp).v_xs;
                                }
                                if xe2 > (*vp).v_xe {
                                    xe2 = (*vp).v_xe;
                                }
                                if xs2 <= xe2 {
                                    RefreshArea(xs2, y2, xe2, y2, 1 as libc::c_int);
                                }
                            }
                        }
                    }
                }
                vp = (*vp).v_next;
            }
        }
        cv = (*cv).c_lnext;
    }
}
pub unsafe extern "C" fn LScrollV(
    mut l: *mut layer,
    mut n: libc::c_int,
    mut ys: libc::c_int,
    mut ye: libc::c_int,
    mut bce: libc::c_int,
) {
    let mut cv: *mut canvas = 0 as *mut canvas;
    let mut vp: *mut viewport = 0 as *mut viewport;
    let mut ys2: libc::c_int = 0;
    let mut ye2: libc::c_int = 0;
    let mut xs2: libc::c_int = 0;
    let mut xe2: libc::c_int = 0;
    if n == 0 as libc::c_int {
        return;
    }
    if ((*l).l_pause).d() != 0 {
        LayPauseUpdateRegion(
            l,
            0 as libc::c_int,
            (*l).l_width - 1 as libc::c_int,
            ys,
            ye,
        );
    }
    cv = (*l).l_cvlist;
    while !cv.is_null() {
        if !(((*l).l_pause).d() != 0 && (*cv).c_slorient != 0) {
            vp = (*cv).c_vplist;
            while !vp.is_null() {
                xs2 = (*vp).v_xoff;
                xe2 = (*l).l_width - 1 as libc::c_int + (*vp).v_xoff;
                ys2 = ys + (*vp).v_yoff;
                ye2 = ye + (*vp).v_yoff;
                if xs2 < (*vp).v_xs {
                    xs2 = (*vp).v_xs;
                }
                if xe2 > (*vp).v_xe {
                    xe2 = (*vp).v_xe;
                }
                if ys2 < (*vp).v_ys {
                    ys2 = (*vp).v_ys;
                }
                if ye2 > (*vp).v_ye {
                    ye2 = (*vp).v_ye;
                }
                if !(ys2 > ye2 || xs2 > xe2) {
                    display = (*cv).c_display;
                    if !((*display).d_blocked != 0) {
                        ScrollV((*vp).v_xs, ys2, (*vp).v_xe, ye2, n, bce);
                        if !(ye2 - ys2 == ye - ys) {
                            if n > 0 as libc::c_int {
                                ys2 = ye2 + 1 as libc::c_int - n;
                                ye2 = ye + (*vp).v_yoff - n;
                            } else {
                                ye2 = ys2 - 1 as libc::c_int - n;
                                ys2 = ys + (*vp).v_yoff - n;
                            }
                            if ys2 < (*vp).v_ys {
                                ys2 = (*vp).v_ys;
                            }
                            if ye2 > (*vp).v_ye {
                                ye2 = (*vp).v_ye;
                            }
                            if ys2 <= ye2 {
                                RefreshArea(xs2, ys2, xe2, ye2, 1 as libc::c_int);
                            }
                        }
                    }
                }
                vp = (*vp).v_next;
            }
        }
        cv = (*cv).c_lnext;
    }
}
pub unsafe extern "C" fn LInsChar(
    mut l: *mut layer,
    mut c: *mut mchar,
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut ol: *mut mline,
) {
    let mut cv: *mut canvas = 0 as *mut canvas;
    let mut vp: *mut viewport = 0 as *mut viewport;
    let mut xs2: libc::c_int = 0;
    let mut xe2: libc::c_int = 0;
    let mut y2: libc::c_int = 0;
    let mut f: libc::c_int = 0;
    let mut c2: *mut mchar = 0 as *mut mchar;
    let mut cc: mchar = mchar {
        image: 0,
        attr: 0,
        font: 0,
        fontx: 0,
        color: 0,
        mbcs: 0,
    };
    let mut rol: *mut mline = 0 as *mut mline;
    if ((*l).l_pause).d() != 0 {
        LayPauseUpdateRegion(l, x, (*l).l_width - 1 as libc::c_int, y, y);
    }
    cv = (*l).l_cvlist;
    while !cv.is_null() {
        if !(((*l).l_pause).d() != 0 && (*cv).c_slorient != 0) {
            vp = (*cv).c_vplist;
            while !vp.is_null() {
                y2 = y + (*vp).v_yoff;
                if !(y2 < (*vp).v_ys || y2 > (*vp).v_ye) {
                    xs2 = x + (*vp).v_xoff;
                    xe2 = (*l).l_width - 1 as libc::c_int + (*vp).v_xoff;
                    c2 = c;
                    f = 0 as libc::c_int;
                    if xs2 < (*vp).v_xs {
                        xs2 = (*vp).v_xs;
                        c2 = &mut mchar_blank;
                        if !ol.is_null() {
                            let mut i: libc::c_int = xs2 - (*vp).v_xoff
                                - 1 as libc::c_int;
                            if i >= 0 as libc::c_int && i < (*l).l_width {
                                cc.image = *((*ol).image).offset(i as isize);
                                cc.attr = *((*ol).attr).offset(i as isize);
                                cc.font = *((*ol).font).offset(i as isize);
                                cc.fontx = *((*ol).fontx).offset(i as isize);
                                cc.color = *((*ol).color).offset(i as isize);
                                cc.mbcs = 0 as libc::c_int as libc::c_uchar;
                                c2 = &mut cc;
                            }
                        } else {
                            f = 1 as libc::c_int;
                        }
                    }
                    if xe2 > (*vp).v_xe {
                        xe2 = (*vp).v_xe;
                    }
                    if !(xs2 > xe2) {
                        display = (*cv).c_display;
                        if !((*display).d_blocked != 0) {
                            rol = if ((*l).l_encoding == 8 as libc::c_int) as libc::c_int
                                != ((*display).d_encoding == 8 as libc::c_int)
                                    as libc::c_int
                            {
                                recode_mline(
                                    ol,
                                    (*l).l_width,
                                    (*l).l_encoding,
                                    (*display).d_encoding,
                                )
                            } else {
                                ol
                            };
                            InsChar(
                                if ((*l).l_encoding == 8 as libc::c_int) as libc::c_int
                                    != ((*display).d_encoding == 8 as libc::c_int)
                                        as libc::c_int
                                {
                                    recode_mchar(c2, (*l).l_encoding, (*display).d_encoding)
                                } else {
                                    c2
                                },
                                xs2,
                                xe2,
                                y2,
                                mloff(rol, -(*vp).v_xoff),
                            );
                            if f != 0 {
                                RefreshArea(xs2, y2, xs2, y2, 1 as libc::c_int);
                            }
                        }
                    }
                }
                vp = (*vp).v_next;
            }
        }
        cv = (*cv).c_lnext;
    }
}
pub unsafe extern "C" fn LPutChar(
    mut l: *mut layer,
    mut c: *mut mchar,
    mut x: libc::c_int,
    mut y: libc::c_int,
) {
    let mut cv: *mut canvas = 0 as *mut canvas;
    let mut vp: *mut viewport = 0 as *mut viewport;
    let mut x2: libc::c_int = 0;
    let mut y2: libc::c_int = 0;
    if ((*l).l_pause).d() != 0 {
        LayPauseUpdateRegion(
            l,
            x,
            x
                + (if (*c).mbcs as libc::c_int != 0 {
                    1 as libc::c_int
                } else {
                    0 as libc::c_int
                }),
            y,
            y,
        );
    }
    cv = (*l).l_cvlist;
    while !cv.is_null() {
        if !(((*l).l_pause).d() != 0 && (*cv).c_slorient != 0) {
            display = (*cv).c_display;
            if !((*display).d_blocked != 0) {
                vp = (*cv).c_vplist;
                while !vp.is_null() {
                    y2 = y + (*vp).v_yoff;
                    if !(y2 < (*vp).v_ys || y2 > (*vp).v_ye) {
                        x2 = x + (*vp).v_xoff;
                        if !(x2 < (*vp).v_xs || x2 > (*vp).v_xe) {
                            PutChar(
                                if ((*l).l_encoding == 8 as libc::c_int) as libc::c_int
                                    != ((*display).d_encoding == 8 as libc::c_int)
                                        as libc::c_int
                                {
                                    recode_mchar(c, (*l).l_encoding, (*display).d_encoding)
                                } else {
                                    c
                                },
                                x2,
                                y2,
                            );
                            break;
                        }
                    }
                    vp = (*vp).v_next;
                }
            }
        }
        cv = (*cv).c_lnext;
    }
}
pub unsafe extern "C" fn LPutStr(
    mut l: *mut layer,
    mut s: *mut libc::c_char,
    mut n: libc::c_int,
    mut r: *mut mchar,
    mut x: libc::c_int,
    mut y: libc::c_int,
) {
    let mut cv: *mut canvas = 0 as *mut canvas;
    let mut vp: *mut viewport = 0 as *mut viewport;
    let mut s2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut xs2: libc::c_int = 0;
    let mut xe2: libc::c_int = 0;
    let mut y2: libc::c_int = 0;
    if x + n > (*l).l_width {
        n = (*l).l_width - x;
    }
    if ((*l).l_pause).d() != 0 {
        LayPauseUpdateRegion(l, x, x + n - 1 as libc::c_int, y, y);
    }
    cv = (*l).l_cvlist;
    while !cv.is_null() {
        if !(((*l).l_pause).d() != 0 && (*cv).c_slorient != 0) {
            vp = (*cv).c_vplist;
            while !vp.is_null() {
                y2 = y + (*vp).v_yoff;
                if !(y2 < (*vp).v_ys || y2 > (*vp).v_ye) {
                    xs2 = x + (*vp).v_xoff;
                    xe2 = xs2 + n - 1 as libc::c_int;
                    if xs2 < (*vp).v_xs {
                        xs2 = (*vp).v_xs;
                    }
                    if xe2 > (*vp).v_xe {
                        xe2 = (*vp).v_xe;
                    }
                    if !(xs2 > xe2) {
                        display = (*cv).c_display;
                        if !((*display).d_blocked != 0) {
                            GotoPos(xs2, y2);
                            SetRendition(r);
                            s2 = s
                                .offset(xs2 as isize)
                                .offset(-(x as isize))
                                .offset(-((*vp).v_xoff as isize));
                            if (*display).d_encoding == 8 as libc::c_int
                                && (*l).l_encoding != 8 as libc::c_int
                                && ((*r).font as libc::c_int != 0
                                    || (*r).fontx as libc::c_int != 0 || (*l).l_encoding != 0)
                            {
                                let mut mc: mchar = mchar {
                                    image: 0,
                                    attr: 0,
                                    font: 0,
                                    fontx: 0,
                                    color: 0,
                                    mbcs: 0,
                                };
                                mc = *r;
                                while xs2 <= xe2 {
                                    let fresh0 = s2;
                                    s2 = s2.offset(1);
                                    mc.image = *fresh0 as libc::c_uchar;
                                    let fresh1 = xs2;
                                    xs2 = xs2 + 1;
                                    PutChar(
                                        if ((*l).l_encoding == 8 as libc::c_int) as libc::c_int
                                            != ((*display).d_encoding == 8 as libc::c_int)
                                                as libc::c_int
                                        {
                                            recode_mchar(
                                                &mut mc,
                                                (*l).l_encoding,
                                                (*display).d_encoding,
                                            )
                                        } else {
                                            &mut mc
                                        },
                                        fresh1,
                                        y2,
                                    );
                                }
                            } else {
                                loop {
                                    let fresh2 = xs2;
                                    xs2 = xs2 + 1;
                                    if !(fresh2 <= xe2) {
                                        break;
                                    }
                                    let fresh3 = s2;
                                    s2 = s2.offset(1);
                                    PUTCHARLP(*fresh3 as libc::c_int);
                                }
                            }
                        }
                    }
                }
                vp = (*vp).v_next;
            }
        }
        cv = (*cv).c_lnext;
    }
}
pub unsafe extern "C" fn LPutWinMsg(
    mut l: *mut layer,
    mut s: *mut libc::c_char,
    mut n: libc::c_int,
    mut r: *mut mchar,
    mut x: libc::c_int,
    mut y: libc::c_int,
) {
    let mut cv: *mut canvas = 0 as *mut canvas;
    let mut vp: *mut viewport = 0 as *mut viewport;
    let mut xs2: libc::c_int = 0;
    let mut xe2: libc::c_int = 0;
    let mut y2: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut len2: libc::c_int = 0;
    let mut or: mchar = mchar {
        image: 0,
        attr: 0,
        font: 0,
        fontx: 0,
        color: 0,
        mbcs: 0,
    };
    if x + n > (*l).l_width {
        n = (*l).l_width - x;
    }
    if ((*l).l_pause).d() != 0 {
        LayPauseUpdateRegion(l, x, x + n - 1 as libc::c_int, y, y);
    }
    len = strlen(s) as libc::c_int;
    if len > n {
        len = n;
    }
    cv = (*l).l_cvlist;
    while !cv.is_null() {
        if !(((*l).l_pause).d() != 0 && (*cv).c_slorient != 0) {
            vp = (*cv).c_vplist;
            while !vp.is_null() {
                y2 = y + (*vp).v_yoff;
                if !(y2 < (*vp).v_ys || y2 > (*vp).v_ye) {
                    xs2 = x + (*vp).v_xoff;
                    xe2 = xs2 + n - 1 as libc::c_int;
                    if xs2 < (*vp).v_xs {
                        xs2 = (*vp).v_xs;
                    }
                    if xe2 > (*vp).v_xe {
                        xe2 = (*vp).v_xe;
                    }
                    if !(xs2 > xe2) {
                        display = (*cv).c_display;
                        if !((*display).d_blocked != 0) {
                            GotoPos(xs2, y2);
                            SetRendition(r);
                            len2 = xe2 - (x + (*vp).v_xoff) + 1 as libc::c_int;
                            if len2 > len {
                                len2 = len;
                            }
                            PutWinMsg(s, xs2 - x - (*vp).v_xoff, len2);
                            xs2 = x + (*vp).v_xoff + len2;
                            if xs2 < (*vp).v_xs {
                                xs2 = (*vp).v_xs;
                            }
                            or = (*display).d_rend;
                            GotoPos(xs2, y2);
                            SetRendition(&mut or);
                            loop {
                                let fresh4 = xs2;
                                xs2 = xs2 + 1;
                                if !(fresh4 <= xe2) {
                                    break;
                                }
                                PUTCHARLP(' ' as i32);
                            }
                        }
                    }
                }
                vp = (*vp).v_next;
            }
        }
        cv = (*cv).c_lnext;
    }
}
pub unsafe extern "C" fn LClearLine(
    mut l: *mut layer,
    mut y: libc::c_int,
    mut xs: libc::c_int,
    mut xe: libc::c_int,
    mut bce: libc::c_int,
    mut ol: *mut mline,
) {
    let mut cv: *mut canvas = 0 as *mut canvas;
    let mut vp: *mut viewport = 0 as *mut viewport;
    let mut y2: libc::c_int = 0;
    let mut xs2: libc::c_int = 0;
    let mut xe2: libc::c_int = 0;
    if xs >= (*l).l_width {
        xs = (*l).l_width - 1 as libc::c_int;
    }
    if xe >= (*l).l_width {
        xe = (*l).l_width - 1 as libc::c_int;
    }
    if ((*l).l_pause).d() != 0 {
        LayPauseUpdateRegion(l, xs, xe, y, y);
    }
    cv = (*l).l_cvlist;
    while !cv.is_null() {
        if !(((*l).l_pause).d() != 0 && (*cv).c_slorient != 0) {
            vp = (*cv).c_vplist;
            while !vp.is_null() {
                xs2 = xs + (*vp).v_xoff;
                xe2 = xe + (*vp).v_xoff;
                y2 = y + (*vp).v_yoff;
                if !(y2 < (*vp).v_ys || y2 > (*vp).v_ye) {
                    if xs2 < (*vp).v_xs {
                        xs2 = (*vp).v_xs;
                    }
                    if xe2 > (*vp).v_xe {
                        xe2 = (*vp).v_xe;
                    }
                    if !(xs2 > xe2) {
                        display = (*cv).c_display;
                        if !((*display).d_blocked != 0) {
                            ClearLine(
                                if !ol.is_null() {
                                    mloff(
                                        if ((*l).l_encoding == 8 as libc::c_int) as libc::c_int
                                            != ((*display).d_encoding == 8 as libc::c_int)
                                                as libc::c_int
                                        {
                                            recode_mline(
                                                ol,
                                                (*l).l_width,
                                                (*l).l_encoding,
                                                (*display).d_encoding,
                                            )
                                        } else {
                                            ol
                                        },
                                        -(*vp).v_xoff,
                                    )
                                } else {
                                    0 as *mut mline
                                },
                                y2,
                                xs2,
                                xe2,
                                bce,
                            );
                        }
                    }
                }
                vp = (*vp).v_next;
            }
        }
        cv = (*cv).c_lnext;
    }
}
pub unsafe extern "C" fn LClearArea(
    mut l: *mut layer,
    mut xs: libc::c_int,
    mut ys: libc::c_int,
    mut xe: libc::c_int,
    mut ye: libc::c_int,
    mut bce: libc::c_int,
    mut uself: libc::c_int,
) {
    let mut cv: *mut canvas = 0 as *mut canvas;
    let mut vp: *mut viewport = 0 as *mut viewport;
    let mut xs2: libc::c_int = 0;
    let mut ys2: libc::c_int = 0;
    let mut xe2: libc::c_int = 0;
    let mut ye2: libc::c_int = 0;
    if ys < 0 as libc::c_int || ye < ys {
        return;
    }
    if xs >= (*l).l_width {
        xs = (*l).l_width - 1 as libc::c_int;
    }
    if xe >= (*l).l_width {
        xe = (*l).l_width - 1 as libc::c_int;
    }
    if ((*l).l_pause).d() != 0 {
        LayPauseUpdateRegion(l, xs, xe, ys, ye);
    }
    cv = (*l).l_cvlist;
    while !cv.is_null() {
        if !(((*l).l_pause).d() != 0 && (*cv).c_slorient != 0) {
            display = (*cv).c_display;
            if !((*display).d_blocked != 0) {
                vp = (*cv).c_vplist;
                while !vp.is_null() {
                    xs2 = xs + (*vp).v_xoff;
                    xe2 = xe + (*vp).v_xoff;
                    ys2 = ys + (*vp).v_yoff;
                    ye2 = ye + (*vp).v_yoff;
                    if xs2 < (*vp).v_xs {
                        xs2 = (*vp).v_xs;
                    }
                    if xe2 > (*vp).v_xe {
                        xe2 = (*vp).v_xe;
                    }
                    if xs2 > (*vp).v_xe {
                        ys2 += 1;
                        ys2;
                    }
                    if xe2 < (*vp).v_xs {
                        ye2 -= 1;
                        ye2;
                    }
                    if ys2 < (*vp).v_ys {
                        ys2 = (*vp).v_ys;
                    }
                    if ye2 > (*vp).v_ye {
                        ye2 = (*vp).v_ye;
                    }
                    if !(ys2 > ye2) {
                        if xs == 0 as libc::c_int || ys2 != ys + (*vp).v_yoff {
                            xs2 = (*vp).v_xs;
                        }
                        if xe == (*l).l_width - 1 as libc::c_int
                            || ye2 != ye + (*vp).v_yoff
                        {
                            xe2 = (*vp).v_xe;
                        }
                        display = (*cv).c_display;
                        ClearArea(
                            xs2,
                            ys2,
                            (*vp).v_xs,
                            (*vp).v_xe,
                            xe2,
                            ye2,
                            bce,
                            uself,
                        );
                        if xe == (*l).l_width - 1 as libc::c_int
                            && xe2 > (*vp).v_xoff + xe
                        {
                            let mut y: libc::c_int = 0;
                            SetRendition(&mut mchar_blank);
                            y = ys2;
                            while y <= ye2 {
                                GotoPos(xe + (*vp).v_xoff + 1 as libc::c_int, y);
                                PUTCHARLP('|' as i32);
                                y += 1;
                                y;
                            }
                        }
                    }
                    vp = (*vp).v_next;
                }
            }
        }
        cv = (*cv).c_lnext;
    }
}
pub unsafe extern "C" fn LCDisplayLine(
    mut l: *mut layer,
    mut ml: *mut mline,
    mut y: libc::c_int,
    mut xs: libc::c_int,
    mut xe: libc::c_int,
    mut isblank: libc::c_int,
) {
    let mut cv: *mut canvas = 0 as *mut canvas;
    let mut vp: *mut viewport = 0 as *mut viewport;
    let mut xs2: libc::c_int = 0;
    let mut xe2: libc::c_int = 0;
    let mut y2: libc::c_int = 0;
    if ((*l).l_pause).d() != 0 {
        LayPauseUpdateRegion(l, xs, xe, y, y);
    }
    cv = (*l).l_cvlist;
    while !cv.is_null() {
        if !(((*l).l_pause).d() != 0 && (*cv).c_slorient != 0) {
            display = (*cv).c_display;
            if !((*display).d_blocked != 0) {
                vp = (*cv).c_vplist;
                while !vp.is_null() {
                    xs2 = xs + (*vp).v_xoff;
                    xe2 = xe + (*vp).v_xoff;
                    y2 = y + (*vp).v_yoff;
                    if !(y2 < (*vp).v_ys || y2 > (*vp).v_ye) {
                        if xs2 < (*vp).v_xs {
                            xs2 = (*vp).v_xs;
                        }
                        if xe2 > (*vp).v_xe {
                            xe2 = (*vp).v_xe;
                        }
                        if !(xs2 > xe2) {
                            display = (*cv).c_display;
                            DisplayLine(
                                if isblank != 0 {
                                    &mut mline_blank
                                } else {
                                    &mut mline_null
                                },
                                mloff(
                                    if ((*l).l_encoding == 8 as libc::c_int) as libc::c_int
                                        != ((*display).d_encoding == 8 as libc::c_int)
                                            as libc::c_int
                                    {
                                        recode_mline(
                                            ml,
                                            (*l).l_width,
                                            (*l).l_encoding,
                                            (*display).d_encoding,
                                        )
                                    } else {
                                        ml
                                    },
                                    -(*vp).v_xoff,
                                ),
                                y2,
                                xs2,
                                xe2,
                            );
                        }
                    }
                    vp = (*vp).v_next;
                }
            }
        }
        cv = (*cv).c_lnext;
    }
}
pub unsafe extern "C" fn LCDisplayLineWrap(
    mut l: *mut layer,
    mut ml: *mut mline,
    mut y: libc::c_int,
    mut from: libc::c_int,
    mut to: libc::c_int,
    mut isblank: libc::c_int,
) {
    let mut nc: mchar = mchar {
        image: 0,
        attr: 0,
        font: 0,
        fontx: 0,
        color: 0,
        mbcs: 0,
    };
    nc.image = *((*ml).image).offset(0 as libc::c_int as isize);
    nc.attr = *((*ml).attr).offset(0 as libc::c_int as isize);
    nc.font = *((*ml).font).offset(0 as libc::c_int as isize);
    nc.fontx = *((*ml).fontx).offset(0 as libc::c_int as isize);
    nc.color = *((*ml).color).offset(0 as libc::c_int as isize);
    nc.mbcs = 0 as libc::c_int as libc::c_uchar;
    if if (*l).l_encoding == 8 as libc::c_int {
        (*((*ml).font).offset((0 as libc::c_int + 1 as libc::c_int) as isize)
            as libc::c_int == 0xff as libc::c_int
            && *((*ml).image).offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                as libc::c_int == 0xff as libc::c_int) as libc::c_int
    } else {
        (*((*ml).font).offset(0 as libc::c_int as isize) as libc::c_int
            & 0x1f as libc::c_int != 0 as libc::c_int
            && *((*ml).font).offset(0 as libc::c_int as isize) as libc::c_int
                & 0xe0 as libc::c_int == 0 as libc::c_int) as libc::c_int
    } != 0
    {
        nc.mbcs = *((*ml).image).offset(1 as libc::c_int as isize);
        from += 1;
        from;
    }
    LWrapChar(
        l,
        &mut nc,
        y - 1 as libc::c_int,
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        0 as libc::c_int,
    );
    from += 1;
    from;
    if from <= to {
        LCDisplayLine(l, ml, y, from, to, isblank);
    }
}
pub unsafe extern "C" fn LSetRendition(mut l: *mut layer, mut r: *mut mchar) {
    let mut cv: *mut canvas = 0 as *mut canvas;
    cv = (*l).l_cvlist;
    while !cv.is_null() {
        display = (*cv).c_display;
        if !((*display).d_blocked != 0) {
            SetRendition(r);
        }
        cv = (*cv).c_lnext;
    }
}
pub unsafe extern "C" fn LWrapChar(
    mut l: *mut layer,
    mut c: *mut mchar,
    mut y: libc::c_int,
    mut top: libc::c_int,
    mut bot: libc::c_int,
    mut ins: libc::c_int,
) {
    let mut cv: *mut canvas = 0 as *mut canvas;
    let mut cvlist: *mut canvas = 0 as *mut canvas;
    let mut cvlnext: *mut canvas = 0 as *mut canvas;
    let mut vp: *mut viewport = 0 as *mut viewport;
    let mut evp: *mut viewport = 0 as *mut viewport;
    let mut vpp: *mut *mut viewport = 0 as *mut *mut viewport;
    let mut yy: libc::c_int = 0;
    let mut y2: libc::c_int = 0;
    let mut yy2: libc::c_int = 0;
    let mut top2: libc::c_int = 0;
    let mut bot2: libc::c_int = 0;
    let mut bce: libc::c_int = 0;
    if ((*l).l_pause).d() != 0 {
        LayPauseUpdateRegion(
            l,
            0 as libc::c_int,
            (*l).l_width - 1 as libc::c_int,
            top,
            bot,
        );
    }
    bce = ((*c).color as libc::c_int & 0xf0 as libc::c_int) >> 4 as libc::c_int
        | (if (*c).attr as libc::c_int & (1 as libc::c_int) << 7 as libc::c_int != 0 {
            0x100 as libc::c_int
        } else {
            0 as libc::c_int
        });
    if y != bot {
        yy = if y == (*l).l_height - 1 as libc::c_int {
            y
        } else {
            y + 1 as libc::c_int
        };
        cv = (*l).l_cvlist;
        while !cv.is_null() {
            if !(((*l).l_pause).d() != 0 && (*cv).c_slorient != 0) {
                y2 = 0 as libc::c_int;
                display = (*cv).c_display;
                if !((*display).d_blocked != 0) {
                    vp = (*cv).c_vplist;
                    while !vp.is_null() {
                        y2 = y + (*vp).v_yoff;
                        yy2 = yy + (*vp).v_yoff;
                        if yy2 >= (*vp).v_ys && yy2 <= (*vp).v_ye
                            && (*vp).v_xoff >= (*vp).v_xs && (*vp).v_xoff <= (*vp).v_xe
                        {
                            break;
                        }
                        vp = (*vp).v_next;
                    }
                    if !vp.is_null() {
                        evp = (*cv).c_vplist;
                        while !evp.is_null() {
                            if y2 >= (*evp).v_ys && y2 <= (*evp).v_ye
                                && (*evp).v_xoff + (*l).l_width - 1 as libc::c_int
                                    >= (*evp).v_xs
                                && (*evp).v_xoff + (*l).l_width - 1 as libc::c_int
                                    <= (*evp).v_xe
                            {
                                break;
                            }
                            evp = (*evp).v_next;
                        }
                        if evp.is_null()
                            || ins != 0
                                && (*vp).v_xoff + (*l).l_width - 1 as libc::c_int
                                    > (*vp).v_ye
                        {
                            cvlist = (*l).l_cvlist;
                            cvlnext = (*cv).c_lnext;
                            (*l).l_cvlist = cv;
                            (*cv).c_lnext = 0 as *mut canvas;
                            if ins != 0 {
                                LInsChar(l, c, 0 as libc::c_int, yy, 0 as *mut mline);
                            } else {
                                LPutChar(l, c, 0 as libc::c_int, yy);
                            }
                            (*l).l_cvlist = cvlist;
                            (*cv).c_lnext = cvlnext;
                        } else {
                            WrapChar(
                                if ((*l).l_encoding == 8 as libc::c_int) as libc::c_int
                                    != ((*display).d_encoding == 8 as libc::c_int)
                                        as libc::c_int
                                {
                                    recode_mchar(c, (*l).l_encoding, (*display).d_encoding)
                                } else {
                                    c
                                },
                                (*vp).v_xoff + (*l).l_width,
                                y2,
                                (*vp).v_xoff,
                                -(1 as libc::c_int),
                                (*vp).v_xoff + (*l).l_width - 1 as libc::c_int,
                                -(1 as libc::c_int),
                                ins,
                            );
                        }
                    }
                }
            }
            cv = (*cv).c_lnext;
        }
    } else {
        cv = (*l).l_cvlist;
        while !cv.is_null() {
            if !(((*l).l_pause).d() != 0 && (*cv).c_slorient != 0) {
                display = (*cv).c_display;
                if !((*display).d_blocked != 0) {
                    vpp = &mut (*cv).c_vplist;
                    loop {
                        vp = *vpp;
                        if vp.is_null() {
                            break;
                        }
                        yy2 = bot + (*vp).v_yoff;
                        if yy2 >= (*vp).v_ys && yy2 <= (*vp).v_ye
                            && (*vp).v_xoff >= (*vp).v_xs
                            && (*vp).v_xoff + (*l).l_width - 1 as libc::c_int
                                <= (*vp).v_xe
                        {
                            break;
                        }
                        vpp = &mut (*vp).v_next;
                    }
                    if !vp.is_null() {
                        *vpp = (*vp).v_next;
                    }
                    if !((*cv).c_vplist).is_null() {
                        cvlist = (*l).l_cvlist;
                        cvlnext = (*cv).c_lnext;
                        (*l).l_cvlist = cv;
                        (*cv).c_lnext = 0 as *mut canvas;
                        LScrollV(l, 1 as libc::c_int, top, bot, bce);
                        if vp.is_null() {
                            if ins != 0 {
                                LInsChar(l, c, 0 as libc::c_int, bot, 0 as *mut mline);
                            } else {
                                LPutChar(l, c, 0 as libc::c_int, bot);
                            }
                        }
                        (*l).l_cvlist = cvlist;
                        (*cv).c_lnext = cvlnext;
                    }
                    if !vp.is_null() {
                        *vpp = vp;
                        top2 = top + (*vp).v_yoff;
                        bot2 = bot + (*vp).v_yoff;
                        if top2 < (*vp).v_ys {
                            top2 = (*vp).v_ys;
                        }
                        WrapChar(
                            if ((*l).l_encoding == 8 as libc::c_int) as libc::c_int
                                != ((*display).d_encoding == 8 as libc::c_int)
                                    as libc::c_int
                            {
                                recode_mchar(c, (*l).l_encoding, (*display).d_encoding)
                            } else {
                                c
                            },
                            (*vp).v_xoff + (*l).l_width,
                            bot2,
                            (*vp).v_xoff,
                            top2,
                            (*vp).v_xoff + (*l).l_width - 1 as libc::c_int,
                            bot2,
                            ins,
                        );
                    }
                }
            }
            cv = (*cv).c_lnext;
        }
    };
}
pub unsafe extern "C" fn LCursorVisibility(mut l: *mut layer, mut vis: libc::c_int) {
    let mut cv: *mut canvas = 0 as *mut canvas;
    cv = (*l).l_cvlist;
    while !cv.is_null() {
        display = (*cv).c_display;
        if !((*display).d_blocked != 0) {
            if !(cv != (*display).d_forecv) {
                CursorVisibility(vis);
            }
        }
        cv = (*cv).c_lnext;
    }
}
pub unsafe extern "C" fn LSetFlow(mut l: *mut layer, mut flow: libc::c_int) {
    let mut cv: *mut canvas = 0 as *mut canvas;
    cv = (*l).l_cvlist;
    while !cv.is_null() {
        display = (*cv).c_display;
        if !(cv != (*display).d_forecv) {
            SetFlow(flow);
        }
        cv = (*cv).c_lnext;
    }
}
pub unsafe extern "C" fn LKeypadMode(mut l: *mut layer, mut on: libc::c_int) {
    let mut cv: *mut canvas = 0 as *mut canvas;
    cv = (*l).l_cvlist;
    while !cv.is_null() {
        display = (*cv).c_display;
        if !((*display).d_blocked != 0) {
            if !(cv != (*display).d_forecv) {
                KeypadMode(on);
            }
        }
        cv = (*cv).c_lnext;
    }
}
pub unsafe extern "C" fn LCursorkeysMode(mut l: *mut layer, mut on: libc::c_int) {
    let mut cv: *mut canvas = 0 as *mut canvas;
    cv = (*l).l_cvlist;
    while !cv.is_null() {
        display = (*cv).c_display;
        if !((*display).d_blocked != 0) {
            if !(cv != (*display).d_forecv) {
                CursorkeysMode(on);
            }
        }
        cv = (*cv).c_lnext;
    }
}
pub unsafe extern "C" fn LMouseMode(mut l: *mut layer, mut on: libc::c_int) {
    let mut cv: *mut canvas = 0 as *mut canvas;
    cv = (*l).l_cvlist;
    while !cv.is_null() {
        display = (*cv).c_display;
        if !((*display).d_blocked != 0) {
            if !(cv != (*display).d_forecv) {
                MouseMode(on);
            }
        }
        cv = (*cv).c_lnext;
    }
}
pub unsafe extern "C" fn LClearAll(mut l: *mut layer, mut uself: libc::c_int) {
    LClearArea(
        l,
        0 as libc::c_int,
        0 as libc::c_int,
        (*l).l_width - 1 as libc::c_int,
        (*l).l_height - 1 as libc::c_int,
        0 as libc::c_int,
        uself,
    );
}
pub unsafe extern "C" fn LRefreshAll(mut l: *mut layer, mut isblank: libc::c_int) {
    let mut oldflayer: *mut layer = 0 as *mut layer;
    let mut y: libc::c_int = 0;
    oldflayer = flayer;
    flayer = l;
    if isblank == 0 {
        LClearArea(
            l,
            0 as libc::c_int,
            0 as libc::c_int,
            (*l).l_width - 1 as libc::c_int,
            (*l).l_height - 1 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
        );
    }
    (Some(((*(*flayer).l_layfn).lf_LayRedisplayLine).unwrap()))
        .unwrap()(
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        1 as libc::c_int,
    );
    y = 0 as libc::c_int;
    while y < (*l).l_height {
        (Some(((*(*flayer).l_layfn).lf_LayRedisplayLine).unwrap()))
            .unwrap()(
            y,
            0 as libc::c_int,
            (*l).l_width - 1 as libc::c_int,
            1 as libc::c_int,
        );
        y += 1;
        y;
    }
    flayer = oldflayer;
}
pub unsafe extern "C" fn LMsg(
    mut err: libc::c_int,
    mut fmt: *const libc::c_char,
    mut args: ...
) {
    let mut ap: ::std::ffi::VaListImpl;
    let mut buf: [libc::c_char; 8192] = [0; 8192];
    let mut p: *mut libc::c_char = buf.as_mut_ptr();
    let mut cv: *mut canvas = 0 as *mut canvas;
    ap = args.clone();
    fmt = DoNLS(fmt);
    vsnprintf(
        p,
        (::std::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong)
            .wrapping_sub(100 as libc::c_int as libc::c_ulong),
        fmt,
        ap.as_va_list(),
    );
    if err != 0 {
        p = p.offset(strlen(p) as isize);
        let fresh5 = p;
        p = p.offset(1);
        *fresh5 = ':' as i32 as libc::c_char;
        let fresh6 = p;
        p = p.offset(1);
        *fresh6 = ' ' as i32 as libc::c_char;
        strncpy(
            p,
            strerror(err),
            (buf
                .as_mut_ptr()
                .offset(
                    ::std::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong
                        as isize,
                )
                .offset_from(p) as libc::c_long - 1 as libc::c_int as libc::c_long)
                as libc::c_ulong,
        );
        buf[(::std::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            as usize] = 0 as libc::c_int as libc::c_char;
    }
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
            MakeStatus(buf.as_mut_ptr());
        }
        display = (*display).d_next;
    }
}
pub unsafe extern "C" fn LExtMouseMode(mut l: *mut layer, mut on: libc::c_int) {
    let mut cv: *mut canvas = 0 as *mut canvas;
    cv = (*l).l_cvlist;
    while !cv.is_null() {
        display = (*cv).c_display;
        if !((*display).d_blocked != 0) {
            if !(cv != (*display).d_forecv) {
                ExtMouseMode(on);
            }
        }
        cv = (*cv).c_lnext;
    }
}
pub unsafe extern "C" fn KillLayerChain(mut lay: *mut layer) {
    let mut cv: *mut canvas = 0 as *mut canvas;
    let mut ncv: *mut canvas = 0 as *mut canvas;
    let mut l: *mut layer = 0 as *mut layer;
    let mut oldflayer: *mut layer = 0 as *mut layer;
    oldflayer = flayer;
    l = lay;
    while !l.is_null() {
        if (*l).l_layfn == &mut WinLf as *mut LayFuncs
            || (*l).l_layfn == &mut BlankLf as *mut LayFuncs
        {
            break;
        }
        if oldflayer == l {
            oldflayer = 0 as *mut layer;
        }
        cv = (*l).l_cvlist;
        while !cv.is_null() {
            ncv = (*cv).c_lnext;
            (*cv).c_layer = 0 as *mut layer;
            (*cv).c_lnext = 0 as *mut canvas;
            cv = ncv;
        }
        l = (*l).l_next;
    }
    flayer = lay;
    while flayer != l {
        ExitOverlayPage();
    }
    flayer = oldflayer;
}
pub unsafe extern "C" fn InitOverlayPage(
    mut datasize: libc::c_int,
    mut lf: *mut LayFuncs,
    mut block: libc::c_int,
) -> libc::c_int {
    let mut data: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut newlay: *mut layer = 0 as *mut layer;
    let mut cv: *mut canvas = 0 as *mut canvas;
    let mut cvp: *mut canvas = 0 as *mut canvas;
    let mut cvpp: *mut *mut canvas = 0 as *mut *mut canvas;
    let mut p: *mut win = 0 as *mut win;
    cv = 0 as *mut canvas;
    if !display.is_null() && (*(*display).d_forecv).c_layer == flayer {
        cv = (*display).d_forecv;
    }
    newlay = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<layer>() as libc::c_ulong,
    ) as *mut layer;
    if newlay.is_null() {
        Msg(
            0 as libc::c_int,
            b"No memory for layer struct\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    data = 0 as *mut libc::c_char;
    if datasize != 0 {
        data = calloc(1 as libc::c_int as libc::c_ulong, datasize as libc::c_ulong)
            as *mut libc::c_char;
        if data.is_null() {
            free(newlay as *mut libc::c_char as *mut libc::c_void);
            Msg(
                0 as libc::c_int,
                b"No memory for layer data\0" as *const u8 as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
    }
    p = (*(*flayer).l_bottom).l_data as *mut win;
    if !p.is_null()
        && ((*p).w_savelayer == flayer || block != 0 && ((*flayer).l_next).is_null())
    {
        if !((*p).w_savelayer).is_null() && (*p).w_savelayer != flayer
            && ((*(*p).w_savelayer).l_cvlist).is_null()
        {
            KillLayerChain((*p).w_savelayer);
        }
        (*p).w_savelayer = newlay;
    }
    if !cv.is_null() && ((*flayer).l_next).is_null() && block == 0 {
        let mut olddisplay: *mut display = display;
        display = (*cv).c_display;
        RemoveStatus();
        display = olddisplay;
        cvpp = &mut (*flayer).l_cvlist;
        loop {
            cvp = *cvpp;
            if cvp.is_null() {
                break;
            }
            if cvp == cv {
                break;
            }
            cvpp = &mut (*cvp).c_lnext;
        }
        *cvpp = (*cv).c_lnext;
        (*newlay).l_cvlist = cv;
        (*cv).c_lnext = 0 as *mut canvas;
        (*cv).c_layer = newlay;
    } else {
        let mut olddisplay_0: *mut display = display;
        let mut cv_0: *mut canvas = 0 as *mut canvas;
        display = displays;
        while !display.is_null() {
            cv_0 = (*display).d_cvlist;
            while !cv_0.is_null() {
                if (*cv_0).c_layer == flayer {
                    break;
                }
                cv_0 = (*cv_0).c_next;
            }
            if !cv_0.is_null() {
                RemoveStatus();
            }
            display = (*display).d_next;
        }
        display = olddisplay_0;
        block != 0;
        if block != 0 && (*flayer).l_layfn == &mut WinLf as *mut LayFuncs {
            (*p).w_blocked += 1;
            (*p).w_blocked;
            (*newlay).l_blocking = 1 as libc::c_int;
        }
        (*newlay).l_cvlist = (*flayer).l_cvlist;
        cvp = (*newlay).l_cvlist;
        while !cvp.is_null() {
            (*cvp).c_layer = newlay;
            cvp = (*cvp).c_lnext;
        }
        (*flayer).l_cvlist = 0 as *mut canvas;
    }
    (*newlay).l_width = (*flayer).l_width;
    (*newlay).l_height = (*flayer).l_height;
    (*newlay).l_encoding = 0 as libc::c_int;
    (*newlay).l_layfn = lf;
    (*newlay).l_data = data as *mut libc::c_void;
    (*newlay).l_next = flayer;
    (*newlay).l_bottom = (*flayer).l_bottom;
    flayer = newlay;
    (Some(((*(*flayer).l_layfn).lf_LayRestore).unwrap())).unwrap()();
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn ExitOverlayPage() {
    let mut oldlay: *mut layer = 0 as *mut layer;
    let mut p: *mut win = 0 as *mut win;
    let mut doredisplay: libc::c_int = 0 as libc::c_int;
    let mut cv: *mut canvas = 0 as *mut canvas;
    let mut ocv: *mut canvas = 0 as *mut canvas;
    let mut lay: *mut layout = 0 as *mut layout;
    oldlay = flayer;
    if !((*oldlay).l_data).is_null() {
        if ((*(*oldlay).l_layfn).lf_LayFree).is_some() {
            (Some(((*(*flayer).l_layfn).lf_LayFree).unwrap()))
                .unwrap()((*oldlay).l_data);
        }
        free((*oldlay).l_data);
    }
    p = (*(*flayer).l_bottom).l_data as *mut win;
    flayer = (*oldlay).l_next;
    if (*flayer).l_layfn == &mut WinLf as *mut LayFuncs {
        if (*oldlay).l_blocking != 0 {
            (*p).w_blocked -= 1;
            (*p).w_blocked;
        }
        if (*p).w_blocked != 0 && !((*p).w_savelayer).is_null()
            && (*p).w_savelayer != flayer && !((*oldlay).l_cvlist).is_null()
        {
            flayer = (*p).w_savelayer;
            doredisplay = 1 as libc::c_int;
        }
    }
    if !p.is_null() && (*p).w_savelayer == oldlay {
        (*p).w_savelayer = flayer;
    }
    if !p.is_null() && oldlay == (*p).w_paster.pa_pastelayer {
        (*p).w_paster.pa_pastelayer = 0 as *mut layer;
    }
    lay = layouts;
    while !lay.is_null() {
        cv = (*lay).lay_cvlist;
        while !cv.is_null() {
            if (*cv).c_layer == oldlay {
                (*cv).c_layer = flayer;
            }
            cv = (*cv).c_next;
        }
        lay = (*lay).lay_next;
    }
    ocv = 0 as *mut canvas;
    cv = (*oldlay).l_cvlist;
    while !cv.is_null() {
        (*cv).c_layer = flayer;
        ocv = cv;
        cv = (*cv).c_lnext;
    }
    if !ocv.is_null() {
        cv = (*flayer).l_cvlist;
        (*ocv).c_lnext = 0 as *mut canvas;
        (*flayer).l_cvlist = (*oldlay).l_cvlist;
        if doredisplay != 0 {
            LRefreshAll(flayer, 0 as libc::c_int);
        }
        (*ocv).c_lnext = cv;
    }
    (*oldlay).l_cvlist = 0 as *mut canvas;
    LayerCleanupMemory(oldlay);
    free(oldlay as *mut libc::c_char as *mut libc::c_void);
    (Some(((*(*flayer).l_layfn).lf_LayRestore).unwrap())).unwrap()();
    LGotoPos(flayer, (*flayer).l_x, (*flayer).l_y);
}
pub unsafe extern "C" fn LayProcessMouse(
    mut l: *mut layer,
    mut ch: libc::c_uchar,
) -> libc::c_int {
    let mut len: libc::c_int = 0;
    if (*l).l_mouseevent.len as libc::c_ulong
        >= ::std::mem::size_of::<[libc::c_uchar; 3]>() as libc::c_ulong
    {
        return -(1 as libc::c_int);
    }
    let fresh7 = (*l).l_mouseevent.len;
    (*l).l_mouseevent.len = (*l).l_mouseevent.len + 1;
    len = fresh7;
    (*l)
        .l_mouseevent
        .buffer[len
        as usize] = (if len > 0 as libc::c_int {
        ch as libc::c_int - 33 as libc::c_int
    } else {
        ch as libc::c_int
    }) as libc::c_uchar;
    return ((*l).l_mouseevent.len as libc::c_ulong
        == ::std::mem::size_of::<[libc::c_uchar; 3]>() as libc::c_ulong) as libc::c_int;
}
pub unsafe extern "C" fn LayProcessMouseSwitch(mut l: *mut layer, mut s: libc::c_int) {
    (*l).l_mouseevent.start = s;
    if (*l).l_mouseevent.start != 0 {
        (*l).l_mouseevent.len = 0 as libc::c_int;
    }
}
pub unsafe extern "C" fn LayPause(mut layer: *mut layer, mut pause: libc::c_int) {
    let mut cv: *mut canvas = 0 as *mut canvas;
    let mut line: libc::c_int = 0;
    let mut win: *mut win = 0 as *mut win;
    pause = (pause != 0) as libc::c_int;
    if ((*layer).l_pause).d() == pause {
        return;
    }
    ((*layer).l_pause).set_d(pause);
    if ((*layer).l_pause).d() != 0 {
        (*layer).l_pause.bottom = -(1 as libc::c_int);
        (*layer).l_pause.top = (*layer).l_pause.bottom;
        return;
    }
    if (*layer).l_pause.top == -(1 as libc::c_int)
        && (*layer).l_pause.bottom == -(1 as libc::c_int)
    {
        return;
    }
    if (*layer).l_layfn == &mut WinLf as *mut LayFuncs {
        win = (*layer).l_data as *mut win;
    } else {
        win = 0 as *mut win;
    }
    cv = (*layer).l_cvlist;
    while !cv.is_null() {
        let mut vp: *mut viewport = 0 as *mut viewport;
        if !((*cv).c_slorient == 0) {
            display = (*cv).c_display;
            vp = (*cv).c_vplist;
            while !vp.is_null() {
                line = (*layer).l_pause.top;
                while line <= (*layer).l_pause.bottom {
                    let mut xs: libc::c_int = 0;
                    let mut xe: libc::c_int = 0;
                    if line + (*vp).v_yoff >= (*vp).v_ys
                        && line + (*vp).v_yoff <= (*vp).v_ye
                        && {
                            xs = *((*layer).l_pause.left).offset(line as isize);
                            xs >= 0 as libc::c_int
                        }
                        && {
                            xe = *((*layer).l_pause.right).offset(line as isize);
                            xe >= 0 as libc::c_int
                        }
                    {
                        xs += (*vp).v_xoff;
                        xe += (*vp).v_xoff;
                        if xs < (*vp).v_xs {
                            xs = (*vp).v_xs;
                        }
                        if xe > (*vp).v_xe {
                            xe = (*vp).v_xe;
                        }
                        if (*layer).l_encoding == 8 as libc::c_int && xe < (*vp).v_xe
                            && !win.is_null()
                        {
                            let mut ml: *mut mline = ((*win).w_mlines)
                                .offset(line as isize);
                            if if 8 as libc::c_int == 8 as libc::c_int {
                                (*((*ml).font).offset((xe + 1 as libc::c_int) as isize)
                                    as libc::c_int == 0xff as libc::c_int
                                    && *((*ml).image).offset((xe + 1 as libc::c_int) as isize)
                                        as libc::c_int == 0xff as libc::c_int) as libc::c_int
                            } else {
                                (*((*ml).font).offset(xe as isize) as libc::c_int
                                    & 0x1f as libc::c_int != 0 as libc::c_int
                                    && *((*ml).font).offset(xe as isize) as libc::c_int
                                        & 0xe0 as libc::c_int == 0 as libc::c_int) as libc::c_int
                            } != 0
                            {
                                xe += 1;
                                xe;
                            }
                        }
                        if xs <= xe {
                            RefreshLine(line + (*vp).v_yoff, xs, xe, 0 as libc::c_int);
                        }
                    }
                    line += 1;
                    line;
                }
                vp = (*vp).v_next;
            }
            if cv == (*display).d_forecv {
                let mut cx: libc::c_int = (*layer).l_x + (*cv).c_xoff;
                let mut cy: libc::c_int = (*layer).l_y + (*cv).c_yoff;
                if cx < (*cv).c_xs {
                    cx = (*cv).c_xs;
                }
                if cy < (*cv).c_ys {
                    cy = (*cv).c_ys;
                }
                if cx > (*cv).c_xe {
                    cx = (*cv).c_xe;
                }
                if cy > (*cv).c_ye {
                    cy = (*cv).c_ye;
                }
                GotoPos(cx, cy);
            }
        }
        cv = (*cv).c_lnext;
    }
    line = (*layer).l_pause.top;
    while line <= (*layer).l_pause.bottom {
        let ref mut fresh8 = *((*layer).l_pause.right).offset(line as isize);
        *fresh8 = -(1 as libc::c_int);
        *((*layer).l_pause.left).offset(line as isize) = *fresh8;
        line += 1;
        line;
    }
}
pub unsafe extern "C" fn LayPauseUpdateRegion(
    mut layer: *mut layer,
    mut xs: libc::c_int,
    mut xe: libc::c_int,
    mut ys: libc::c_int,
    mut ye: libc::c_int,
) {
    if ((*layer).l_pause).d() == 0 {
        return;
    }
    if ys < 0 as libc::c_int {
        ys = 0 as libc::c_int;
    }
    if ye >= (*layer).l_height {
        ye = (*layer).l_height - 1 as libc::c_int;
    }
    if xe >= (*layer).l_width {
        xe = (*layer).l_width - 1 as libc::c_int;
    }
    if (*layer).l_pause.top == -(1 as libc::c_int) || (*layer).l_pause.top > ys {
        (*layer).l_pause.top = ys;
    }
    if (*layer).l_pause.bottom < ye {
        (*layer).l_pause.bottom = ye;
        if (*layer).l_pause.lines <= ye {
            let mut o: libc::c_int = (*layer).l_pause.lines;
            (*layer).l_pause.lines = ye + 32 as libc::c_int;
            (*layer)
                .l_pause
                .left = realloc(
                (*layer).l_pause.left as *mut libc::c_void,
                (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                    .wrapping_mul((*layer).l_pause.lines as libc::c_ulong),
            ) as *mut libc::c_int;
            (*layer)
                .l_pause
                .right = realloc(
                (*layer).l_pause.right as *mut libc::c_void,
                (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                    .wrapping_mul((*layer).l_pause.lines as libc::c_ulong),
            ) as *mut libc::c_int;
            while o < (*layer).l_pause.lines {
                let ref mut fresh9 = *((*layer).l_pause.right).offset(o as isize);
                *fresh9 = -(1 as libc::c_int);
                *((*layer).l_pause.left).offset(o as isize) = *fresh9;
                o += 1;
                o;
            }
        }
    }
    while ys <= ye {
        if *((*layer).l_pause.left).offset(ys as isize) == -(1 as libc::c_int)
            || *((*layer).l_pause.left).offset(ys as isize) > xs
        {
            *((*layer).l_pause.left).offset(ys as isize) = xs;
        }
        if *((*layer).l_pause.right).offset(ys as isize) < xe {
            *((*layer).l_pause.right).offset(ys as isize) = xe;
        }
        ys += 1;
        ys;
    }
}
pub unsafe extern "C" fn LayerCleanupMemory(mut layer: *mut layer) {
    if !((*layer).l_pause.left).is_null() {
        free((*layer).l_pause.left as *mut libc::c_void);
    }
    if !((*layer).l_pause.right).is_null() {
        free((*layer).l_pause.right as *mut libc::c_void);
    }
}
