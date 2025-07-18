use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type logfile;
    fn exit(_: libc::c_int) -> !;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn index(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn DefProcess(_: *mut *mut libc::c_char, _: *mut libc::c_int);
    fn DefRedisplayLine(_: libc::c_int, _: libc::c_int, _: libc::c_int, _: libc::c_int);
    fn DefClearLine(_: libc::c_int, _: libc::c_int, _: libc::c_int, _: libc::c_int);
    fn DefRewrite(
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: *mut mchar,
        _: libc::c_int,
    ) -> libc::c_int;
    fn DefResize(_: libc::c_int, _: libc::c_int) -> libc::c_int;
    fn DefRestore();
    fn centerline(_: *mut libc::c_char, _: libc::c_int);
    fn AddXChar(_: *mut libc::c_char, _: libc::c_int) -> libc::c_int;
    fn LGotoPos(_: *mut layer, _: libc::c_int, _: libc::c_int);
    fn LPutChar(_: *mut layer, _: *mut mchar, _: libc::c_int, _: libc::c_int);
    fn LPutStr(
        _: *mut layer,
        _: *mut libc::c_char,
        _: libc::c_int,
        _: *mut mchar,
        _: libc::c_int,
        _: libc::c_int,
    );
    fn LClearAll(_: *mut layer, _: libc::c_int);
    fn LClearArea(
        _: *mut layer,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
    );
    fn LRefreshAll(_: *mut layer, _: libc::c_int);
    fn LMsg(_: libc::c_int, _: *const libc::c_char, _: ...);
    fn InitOverlayPage(_: libc::c_int, _: *mut LayFuncs, _: libc::c_int) -> libc::c_int;
    fn ExitOverlayPage();
    static mut flayer: *mut layer;
    static mut noargs: [*mut libc::c_char; 0];
    static mut mchar_blank: mchar;
    static mut blank: *mut libc::c_uchar;
    static mut term: [term; 0];
    static mut comms: [comm; 0];
    static mut ktab: [action; 0];
    static mut kmap_exts: *mut kmap_ext;
    static mut kmap_extn: libc::c_int;
    static mut dmtab: [action; 0];
    static mut mmtab: [action; 0];
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
pub struct comm {
    pub name: *mut libc::c_char,
    pub flags: libc::c_int,
    pub userbits: [AclBits; 1],
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
pub struct term {
    pub tcname: *mut libc::c_char,
    pub type_0: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kmap_ext {
    pub str_0: *mut libc::c_char,
    pub fl: libc::c_int,
    pub um: action,
    pub dm: action,
    pub mm: action,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct helpdata {
    pub class: *mut libc::c_char,
    pub ktabp: *mut action,
    pub maxrow: libc::c_int,
    pub grow: libc::c_int,
    pub numcols: libc::c_int,
    pub numrows: libc::c_int,
    pub num_names: libc::c_int,
    pub numskip: libc::c_int,
    pub numpages: libc::c_int,
    pub command_search: libc::c_int,
    pub command_bindings: libc::c_int,
    pub refgrow: libc::c_int,
    pub refcommand_search: libc::c_int,
    pub inter: libc::c_int,
    pub mcom: libc::c_int,
    pub mkey: libc::c_int,
    pub nact: [libc::c_int; 190],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct copydata {
    pub cps: *mut libc::c_char,
    pub savedcps: *mut libc::c_char,
    pub refcps: *mut libc::c_char,
    pub refsavedcps: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bindkeydata {
    pub title: *mut libc::c_char,
    pub tab: *mut action,
    pub pos: libc::c_int,
    pub last: libc::c_int,
    pub page: libc::c_int,
    pub pages: libc::c_int,
}
pub static mut version: [libc::c_char; 60] = [0; 60];
pub unsafe extern "C" fn exit_with_usage(
    mut myname: *mut libc::c_char,
    mut message: *mut libc::c_char,
    mut arg: *mut libc::c_char,
) {
    printf(
        b"Use: %s [-opts] [cmd [args]]\n\0" as *const u8 as *const libc::c_char,
        myname,
    );
    printf(
        b" or: %s -r [host.tty]\n\nOptions:\n\0" as *const u8 as *const libc::c_char,
        myname,
    );
    printf(
        b"-a            Force all capabilities into each window's termcap.\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"-A -[r|R]     Adapt all windows to the new display width & height.\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"-c file       Read configuration file instead of '.screenrc'.\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"-d (-r)       Detach the elsewhere running screen (and reattach here).\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"-dmS name     Start as daemon: Screen session in detached mode.\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"-D (-r)       Detach and logout remote (and reattach here).\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"-D -RR        Do whatever is needed to get a screen session.\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"-e xy         Change command characters.\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"-f            Flow control on, -fn = off, -fa = auto.\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"-h lines      Set the size of the scrollback history buffer.\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"-i            Interrupt output sooner when flow control is on.\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"-l            Login mode on (update %s), -ln = off.\n\0" as *const u8
            as *const libc::c_char,
        b"/var/run/utmp\0" as *const u8 as *const libc::c_char,
    );
    printf(b"-ls [match]   or\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"-list         Do nothing, just list our SockDir [on possible matches].\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"-L            Turn on output logging.\n\0" as *const u8 as *const libc::c_char,
    );
    printf(b"-Logfile file Set logfile name.\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"-m            ignore $STY variable, do create a new screen session.\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"-O            Choose optimal output rather than exact vt100 emulation.\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"-p window     Preselect the named window if it exists.\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"-q            Quiet startup. Exits with non-zero return code if unsuccessful.\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"-Q            Commands will send the response to the stdout of the querying process.\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"-r [session]  Reattach to a detached screen process.\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"-R            Reattach if possible, otherwise start a new session.\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"-s shell      Shell to execute rather than $SHELL.\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"-S sockname   Name this session <pid>.sockname instead of <pid>.<tty>.<host>.\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"-t title      Set title. (window's name).\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"-T term       Use term as $TERM for windows, rather than \"screen\".\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"-U            Tell screen to use UTF-8 encoding.\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"-v            Print \"Screen version %s\".\n\0" as *const u8
            as *const libc::c_char,
        version.as_mut_ptr(),
    );
    printf(
        b"-wipe [match] Do nothing, just clean up SockDir [on possible matches].\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"-x            Attach to a not detached screen. (Multi display mode).\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"-X            Execute <cmd> as a screen command in the specified session.\n\0"
            as *const u8 as *const libc::c_char,
    );
    if !message.is_null() && *message as libc::c_int != 0 {
        printf(b"\nError: \0" as *const u8 as *const libc::c_char);
        printf(message, arg);
        printf(b"\n\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    exit(0 as libc::c_int);
}
static mut HelpLf: LayFuncs = unsafe {
    {
        let mut init = LayFuncs {
            lf_LayProcess: Some(
                HelpProcess
                    as unsafe extern "C" fn(
                        *mut *mut libc::c_char,
                        *mut libc::c_int,
                    ) -> (),
            ),
            lf_LayAbort: Some(HelpAbort as unsafe extern "C" fn() -> ()),
            lf_LayRedisplayLine: Some(
                HelpRedisplayLine
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
                DefRewrite
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
pub unsafe extern "C" fn display_help(
    mut class: *mut libc::c_char,
    mut ktabp: *mut action,
) {
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut key: libc::c_int = 0;
    let mut mcom: libc::c_int = 0;
    let mut mkey: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut used: [libc::c_int; 190] = [0; 190];
    let mut helpdata: *mut helpdata = 0 as *mut helpdata;
    if (*flayer).l_height < 6 as libc::c_int {
        LMsg(
            0 as libc::c_int,
            b"Window height too small for help page\0" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    if InitOverlayPage(
        ::std::mem::size_of::<helpdata>() as libc::c_ulong as libc::c_int,
        &mut HelpLf,
        0 as libc::c_int,
    ) != 0
    {
        return;
    }
    helpdata = (*flayer).l_data as *mut helpdata;
    (*helpdata).class = class;
    (*helpdata).ktabp = ktabp;
    (*helpdata).command_bindings = 0 as libc::c_int;
    (*helpdata).num_names = (*helpdata).command_bindings;
    (*helpdata).command_search = 0 as libc::c_int;
    n = 0 as libc::c_int;
    while n <= 189 as libc::c_int {
        used[n as usize] = 0 as libc::c_int;
        n += 1;
        n;
    }
    mcom = 0 as libc::c_int;
    mkey = 0 as libc::c_int;
    key = 0 as libc::c_int;
    while key < 256 as libc::c_int + (188 as libc::c_int - 106 as libc::c_int) {
        n = (*ktabp.offset(key as isize)).nr;
        if !(n == -(1 as libc::c_int)) {
            if (*ktabp.offset(key as isize)).args == noargs.as_mut_ptr() {
                used[n as usize]
                    += if key <= ' ' as i32 || key == 0x7f as libc::c_int {
                        3 as libc::c_int
                    } else if key > 0x7f as libc::c_int {
                        5 as libc::c_int
                    } else {
                        2 as libc::c_int
                    };
            } else {
                (*helpdata).command_bindings += 1;
                (*helpdata).command_bindings;
            }
        }
        key += 1;
        key;
    }
    i = 0 as libc::c_int;
    n = i;
    while n <= 189 as libc::c_int {
        if used[n as usize] != 0 {
            l = strlen((*comms.as_mut_ptr().offset(n as isize)).name) as libc::c_int;
            if l > mcom {
                mcom = l;
            }
            if used[n as usize] > mkey {
                mkey = used[n as usize];
            }
            let fresh0 = i;
            i = i + 1;
            (*helpdata).nact[fresh0 as usize] = n;
        }
        n += 1;
        n;
    }
    (*helpdata).num_names = i;
    if mkey > 256 as libc::c_int {
        mkey = 256 as libc::c_int;
    }
    (*helpdata).numcols = (*flayer).l_width / (mcom + mkey + 1 as libc::c_int);
    if (*helpdata).numcols == 0 as libc::c_int {
        HelpAbort();
        LMsg(0 as libc::c_int, b"Width too small\0" as *const u8 as *const libc::c_char);
        return;
    }
    (*helpdata)
        .inter = ((*flayer).l_width - (mcom + mkey) * (*helpdata).numcols)
        / ((*helpdata).numcols + 1 as libc::c_int);
    if (*helpdata).inter <= 0 as libc::c_int {
        (*helpdata).inter = 1 as libc::c_int;
    }
    (*helpdata).mcom = mcom;
    (*helpdata).mkey = mkey;
    (*helpdata)
        .numrows = ((*helpdata).num_names + (*helpdata).numcols - 1 as libc::c_int)
        / (*helpdata).numcols;
    (*helpdata)
        .numskip = (*flayer).l_height - 5 as libc::c_int
        - (2 as libc::c_int + (*helpdata).numrows);
    while (*helpdata).numskip < 0 as libc::c_int {
        (*helpdata).numskip += (*flayer).l_height - 5 as libc::c_int;
    }
    (*helpdata).numskip %= (*flayer).l_height - 5 as libc::c_int;
    if (*helpdata).numskip > (*flayer).l_height / 3 as libc::c_int
        || (*helpdata).numskip > (*helpdata).command_bindings
    {
        (*helpdata).numskip = 1 as libc::c_int;
    }
    (*helpdata)
        .maxrow = 2 as libc::c_int + (*helpdata).numrows + (*helpdata).numskip
        + (*helpdata).command_bindings;
    (*helpdata).grow = 0 as libc::c_int;
    (*helpdata)
        .numpages = ((*helpdata).maxrow + (*flayer).l_height - 6 as libc::c_int)
        / ((*flayer).l_height - 5 as libc::c_int);
    (*flayer).l_x = 0 as libc::c_int;
    (*flayer).l_y = (*flayer).l_height - 1 as libc::c_int;
    helppage();
}
unsafe extern "C" fn HelpProcess(
    mut ppbuf: *mut *mut libc::c_char,
    mut plen: *mut libc::c_int,
) {
    let mut done: libc::c_int = 0 as libc::c_int;
    while done == 0 && *plen > 0 as libc::c_int {
        let mut current_block_0: u64;
        match **ppbuf as libc::c_int {
            32 => {
                if helppage() == 0 as libc::c_int {
                    current_block_0 = 15427931788582360902;
                } else {
                    current_block_0 = 1073789639873248462;
                }
            }
            13 | 10 => {
                current_block_0 = 1073789639873248462;
            }
            _ => {
                current_block_0 = 15427931788582360902;
            }
        }
        match current_block_0 {
            1073789639873248462 => {
                done = 1 as libc::c_int;
            }
            _ => {}
        }
        *ppbuf = (*ppbuf).offset(1);
        *ppbuf;
        *plen -= 1;
        *plen;
    }
    if done != 0 {
        HelpAbort();
    }
}
unsafe extern "C" fn HelpAbort() {
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
    ExitOverlayPage();
}
unsafe extern "C" fn helppage() -> libc::c_int {
    let mut helpdata: *mut helpdata = 0 as *mut helpdata;
    let mut col: libc::c_int = 0;
    let mut crow: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut key: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut buf: [libc::c_char; 256] = [0; 256];
    let mut Esc_buf: [libc::c_char; 5] = [0; 5];
    let mut cbuf: [libc::c_char; 256] = [0; 256];
    let mut ktabp: *mut action = 0 as *mut action;
    helpdata = (*flayer).l_data as *mut helpdata;
    ktabp = (*helpdata).ktabp;
    if (*helpdata).grow >= (*helpdata).maxrow {
        return -(1 as libc::c_int);
    }
    (*helpdata).refgrow = (*helpdata).grow;
    (*helpdata).refcommand_search = (*helpdata).command_search;
    LClearAll(flayer, 0 as libc::c_int);
    sprintf(
        cbuf.as_mut_ptr(),
        b"Screen key bindings, page %d of %d.\0" as *const u8 as *const libc::c_char,
        (*helpdata).grow / ((*flayer).l_height - 5 as libc::c_int) + 1 as libc::c_int,
        (*helpdata).numpages,
    );
    centerline(cbuf.as_mut_ptr(), 0 as libc::c_int);
    crow = 2 as libc::c_int;
    *Esc_buf.as_mut_ptr() = '\0' as i32 as libc::c_char;
    *buf.as_mut_ptr() = '\0' as i32 as libc::c_char;
    if !((*flayer).l_cvlist).is_null() && !((*(*flayer).l_cvlist).c_display).is_null() {
        add_key_to_buf(
            buf.as_mut_ptr(),
            (*(*(*(*flayer).l_cvlist).c_display).d_user).u_MetaEsc,
        );
        add_key_to_buf(
            Esc_buf.as_mut_ptr(),
            (*(*(*(*flayer).l_cvlist).c_display).d_user).u_Esc,
        );
    } else {
        strcpy(Esc_buf.as_mut_ptr(), b"??\0" as *const u8 as *const libc::c_char);
        strcpy(buf.as_mut_ptr(), b"??\0" as *const u8 as *const libc::c_char);
    }
    while crow < (*flayer).l_height - 3 as libc::c_int {
        if (*helpdata).grow < 1 as libc::c_int {
            if ktabp == ktab.as_mut_ptr() {
                sprintf(
                    cbuf.as_mut_ptr(),
                    b"Command key:  %s   Literal %s:  %s\0" as *const u8
                        as *const libc::c_char,
                    Esc_buf.as_mut_ptr(),
                    Esc_buf.as_mut_ptr(),
                    buf.as_mut_ptr(),
                );
            } else {
                sprintf(
                    cbuf.as_mut_ptr(),
                    b"Command class: '%.80s'\0" as *const u8 as *const libc::c_char,
                    (*helpdata).class,
                );
            }
            centerline(cbuf.as_mut_ptr(), crow);
            (*helpdata).grow += 1;
            (*helpdata).grow;
        } else if (*helpdata).grow >= 2 as libc::c_int
            && ((*helpdata).grow - 2 as libc::c_int) < (*helpdata).numrows
        {
            x = 0 as libc::c_int;
            col = 0 as libc::c_int;
            while col < (*helpdata).numcols
                && {
                    n = (*helpdata).numrows * col
                        + ((*helpdata).grow - 2 as libc::c_int);
                    n < (*helpdata).num_names
                }
            {
                x += (*helpdata).inter - (col == 0) as libc::c_int;
                n = (*helpdata).nact[n as usize];
                buf[0 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
                key = 0 as libc::c_int;
                while key
                    < 256 as libc::c_int + (188 as libc::c_int - 106 as libc::c_int)
                {
                    if (*ktabp.offset(key as isize)).nr == n
                        && (*ktabp.offset(key as isize)).args == noargs.as_mut_ptr()
                        && strlen(buf.as_mut_ptr())
                            < (::std::mem::size_of::<[libc::c_char; 256]>()
                                as libc::c_ulong)
                                .wrapping_sub(7 as libc::c_int as libc::c_ulong)
                    {
                        strcat(
                            buf.as_mut_ptr(),
                            b" \0" as *const u8 as *const libc::c_char,
                        );
                        add_key_to_buf(buf.as_mut_ptr(), key);
                    }
                    key += 1;
                    key;
                }
                PadStr(
                    (*comms.as_mut_ptr().offset(n as isize)).name,
                    (*helpdata).mcom,
                    x,
                    crow,
                );
                x += (*helpdata).mcom;
                PadStr(buf.as_mut_ptr(), (*helpdata).mkey, x, crow);
                x += (*helpdata).mkey;
                col += 1;
                col;
            }
            (*helpdata).grow += 1;
            (*helpdata).grow;
        } else if (*helpdata).grow - 2 as libc::c_int - (*helpdata).numrows
            >= (*helpdata).numskip
            && (*helpdata).grow - 2 as libc::c_int - (*helpdata).numrows
                - (*helpdata).numskip < (*helpdata).command_bindings
        {
            loop {
                n = (*ktabp.offset((*helpdata).command_search as isize)).nr;
                if !(n == -(1 as libc::c_int)
                    || (*ktabp.offset((*helpdata).command_search as isize)).args
                        == noargs.as_mut_ptr())
                {
                    break;
                }
                (*helpdata).command_search += 1;
                if (*helpdata).command_search
                    >= 256 as libc::c_int + (188 as libc::c_int - 106 as libc::c_int)
                {
                    return -(1 as libc::c_int);
                }
            }
            buf[0 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
            add_key_to_buf(buf.as_mut_ptr(), (*helpdata).command_search);
            PadStr(buf.as_mut_ptr(), 5 as libc::c_int, 0 as libc::c_int, crow);
            let fresh1 = (*helpdata).command_search;
            (*helpdata).command_search = (*helpdata).command_search + 1;
            AddAction(&mut *ktabp.offset(fresh1 as isize), 5 as libc::c_int, crow);
            (*helpdata).grow += 1;
            (*helpdata).grow;
        } else {
            (*helpdata).grow += 1;
            (*helpdata).grow;
        }
        crow += 1;
        crow;
    }
    sprintf(
        cbuf.as_mut_ptr(),
        b"[Press Space %s Return to end.]\0" as *const u8 as *const libc::c_char,
        if (*helpdata).grow < (*helpdata).maxrow {
            b"for next page;\0" as *const u8 as *const libc::c_char
        } else {
            b"or\0" as *const u8 as *const libc::c_char
        },
    );
    centerline(cbuf.as_mut_ptr(), (*flayer).l_height - 2 as libc::c_int);
    LGotoPos(flayer, (*flayer).l_x, (*flayer).l_y);
    return 0 as libc::c_int;
}
unsafe extern "C" fn AddAction(
    mut act: *mut action,
    mut x: libc::c_int,
    mut y: libc::c_int,
) {
    let mut buf: [libc::c_char; 256] = [0; 256];
    let mut del: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut fr: libc::c_int = 0;
    let mut bp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pp: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut lp: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut ll: libc::c_int = 0;
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
    fr = (*flayer).l_width - 1 as libc::c_int - x;
    if fr <= 0 as libc::c_int {
        return;
    }
    l = strlen((*comms.as_mut_ptr().offset((*act).nr as isize)).name) as libc::c_int;
    if l + 1 as libc::c_int > fr {
        l = fr - 1 as libc::c_int;
    }
    PadStr((*comms.as_mut_ptr().offset((*act).nr as isize)).name, l, x, y);
    x += l;
    fr -= l + 1 as libc::c_int;
    let fresh2 = x;
    x = x + 1;
    LPutChar(flayer, if fr != 0 { &mut mchar_blank } else { &mut mchar_dol }, fresh2, y);
    pp = (*act).args;
    lp = (*act).argl;
    while !pp.is_null()
        && {
            cp = *pp;
            !cp.is_null()
        }
    {
        del = 0 as libc::c_int;
        bp = buf.as_mut_ptr();
        let fresh3 = lp;
        lp = lp.offset(1);
        ll = *fresh3;
        if ll == 0 || !(index(cp, ' ' as i32)).is_null() {
            if !(index(cp, '\'' as i32)).is_null() {
                del = '"' as i32;
                let fresh4 = bp;
                bp = bp.offset(1);
                *fresh4 = del as libc::c_char;
            } else {
                del = '\'' as i32;
                let fresh5 = bp;
                bp = bp.offset(1);
                *fresh5 = del as libc::c_char;
            }
        }
        loop {
            let fresh6 = ll;
            ll = ll - 1;
            if !(fresh6 != 0
                && bp < buf.as_mut_ptr().offset(250 as libc::c_int as isize))
            {
                break;
            }
            let fresh7 = cp;
            cp = cp.offset(1);
            bp = bp
                .offset(
                    AddXChar(bp, *(fresh7 as *mut libc::c_uchar) as libc::c_int) as isize,
                );
        }
        if del != 0 {
            let fresh8 = bp;
            bp = bp.offset(1);
            *fresh8 = del as libc::c_char;
        }
        *bp = 0 as libc::c_int as libc::c_char;
        fr = (fr as libc::c_long
            - (bp.offset_from(buf.as_mut_ptr()) as libc::c_long
                + 1 as libc::c_int as libc::c_long)) as libc::c_int;
        if fr < 0 as libc::c_int {
            fr = (fr as libc::c_long + bp.offset_from(buf.as_mut_ptr()) as libc::c_long)
                as libc::c_int;
            if fr > 0 as libc::c_int {
                PadStr(buf.as_mut_ptr(), fr, x, y);
            }
            if fr == 0 as libc::c_int {
                LPutChar(flayer, &mut mchar_dol, x, y);
            }
            return;
        }
        PadStr(buf.as_mut_ptr(), strlen(buf.as_mut_ptr()) as libc::c_int, x, y);
        x = (x as libc::c_ulong).wrapping_add(strlen(buf.as_mut_ptr())) as libc::c_int
            as libc::c_int;
        pp = pp.offset(1);
        pp;
        if !(*pp).is_null() {
            let fresh9 = x;
            x = x + 1;
            LPutChar(
                flayer,
                if fr != 0 { &mut mchar_blank } else { &mut mchar_dol },
                fresh9,
                y,
            );
        }
    }
}
unsafe extern "C" fn add_key_to_buf(mut buf: *mut libc::c_char, mut key: libc::c_int) {
    buf = buf.offset(strlen(buf) as isize);
    if key < 0 as libc::c_int {
        strcpy(buf, b"unset\0" as *const u8 as *const libc::c_char);
    } else if key == ' ' as i32 {
        strcpy(buf, b"sp\0" as *const u8 as *const libc::c_char);
    } else if key >= 256 as libc::c_int {
        key = key - 256 as libc::c_int + 106 as libc::c_int;
        *buf.offset(0 as libc::c_int as isize) = ':' as i32 as libc::c_char;
        *buf
            .offset(
                1 as libc::c_int as isize,
            ) = *((*term.as_mut_ptr().offset(key as isize)).tcname)
            .offset(0 as libc::c_int as isize);
        *buf
            .offset(
                2 as libc::c_int as isize,
            ) = *((*term.as_mut_ptr().offset(key as isize)).tcname)
            .offset(1 as libc::c_int as isize);
        *buf.offset(3 as libc::c_int as isize) = ':' as i32 as libc::c_char;
        *buf.offset(4 as libc::c_int as isize) = 0 as libc::c_int as libc::c_char;
    } else {
        *buf.offset(AddXChar(buf, key) as isize) = 0 as libc::c_int as libc::c_char;
    };
}
unsafe extern "C" fn HelpRedisplayLine(
    mut y: libc::c_int,
    mut xs: libc::c_int,
    mut xe: libc::c_int,
    mut isblank: libc::c_int,
) {
    if y < 0 as libc::c_int {
        let mut helpdata: *mut helpdata = 0 as *mut helpdata;
        helpdata = (*flayer).l_data as *mut helpdata;
        (*helpdata).grow = (*helpdata).refgrow;
        (*helpdata).command_search = (*helpdata).refcommand_search;
        helppage();
        return;
    }
    if y != 0 as libc::c_int && y != (*flayer).l_height - 1 as libc::c_int {
        return;
    }
    if isblank == 0 {
        LClearArea(flayer, xs, y, xe, y, 0 as libc::c_int, 0 as libc::c_int);
    }
}
static mut CopyrightLf: LayFuncs = unsafe {
    {
        let mut init = LayFuncs {
            lf_LayProcess: Some(
                CopyrightProcess
                    as unsafe extern "C" fn(
                        *mut *mut libc::c_char,
                        *mut libc::c_int,
                    ) -> (),
            ),
            lf_LayAbort: Some(CopyrightAbort as unsafe extern "C" fn() -> ()),
            lf_LayRedisplayLine: Some(
                CopyrightRedisplayLine
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
                DefRewrite
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
static mut cpmsg: [libc::c_char; 1354] = unsafe {
    *::std::mem::transmute::<
        &[u8; 1354],
        &[libc::c_char; 1354],
    >(
        b"\nGNU Screen version %v\n\nCopyright (c) 2018-2020 Alexander Naumov, Amadeusz Slawinski\nCopyright (c) 2015-2017 Juergen Weigert, Alexander Naumov, Amadeusz Slawinski\nCopyright (c) 2010-2014 Juergen Weigert, Sadrul Habib Chowdhury\nCopyright (c) 2008-2009 Juergen Weigert, Michael Schroeder, Micah Cowan, Sadrul Habib Chowdhury\nCopyright (c) 1993-2007 Juergen Weigert, Michael Schroeder\nCopyright (c) 1987 Oliver Laumann\n\nThis program is free software; you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation; either version 3, or (at your option) any later version.\n\nThis program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.\n\nYou should have received a copy of the GNU General Public License along with this program (see the file COPYING); if not, see https://www.gnu.org/licenses/, or contact Free Software Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02111-1301  USA.\n\nSend bugreports, fixes, enhancements, t-shirts, money, beer & pizza to screen-devel@gnu.org\n\n\nCapabilities:\n+copy +remote-detach +power-detach +multi-attach +multi-user +font +color-16 +utf8 -rxvt -builtin-telnet \0",
    )
};
unsafe extern "C" fn CopyrightProcess(
    mut ppbuf: *mut *mut libc::c_char,
    mut plen: *mut libc::c_int,
) {
    let mut done: libc::c_int = 0 as libc::c_int;
    let mut copydata: *mut copydata = 0 as *mut copydata;
    copydata = (*flayer).l_data as *mut copydata;
    while done == 0 && *plen > 0 as libc::c_int {
        let mut current_block_3: u64;
        match **ppbuf as libc::c_int {
            32 => {
                if *(*copydata).cps != 0 {
                    copypage();
                    current_block_3 = 14523784380283086299;
                } else {
                    current_block_3 = 3142581409041571743;
                }
            }
            13 | 10 => {
                current_block_3 = 3142581409041571743;
            }
            _ => {
                current_block_3 = 14523784380283086299;
            }
        }
        match current_block_3 {
            3142581409041571743 => {
                CopyrightAbort();
                done = 1 as libc::c_int;
            }
            _ => {}
        }
        *ppbuf = (*ppbuf).offset(1);
        *ppbuf;
        *plen -= 1;
        *plen;
    }
}
unsafe extern "C" fn CopyrightAbort() {
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
    ExitOverlayPage();
}
pub unsafe extern "C" fn display_copyright() {
    let mut copydata: *mut copydata = 0 as *mut copydata;
    if (*flayer).l_width < 10 as libc::c_int || (*flayer).l_height < 5 as libc::c_int {
        LMsg(
            0 as libc::c_int,
            b"Window size too small for copyright page\0" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    if InitOverlayPage(
        ::std::mem::size_of::<copydata>() as libc::c_ulong as libc::c_int,
        &mut CopyrightLf,
        0 as libc::c_int,
    ) != 0
    {
        return;
    }
    copydata = (*flayer).l_data as *mut copydata;
    (*copydata).cps = cpmsg.as_ptr() as *mut libc::c_char;
    (*copydata).savedcps = 0 as *mut libc::c_char;
    (*flayer).l_x = 0 as libc::c_int;
    (*flayer).l_y = (*flayer).l_height - 1 as libc::c_int;
    copypage();
}
unsafe extern "C" fn copypage() {
    let mut cps: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ws: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut cbuf: [libc::c_char; 80] = [0; 80];
    let mut copydata: *mut copydata = 0 as *mut copydata;
    copydata = (*flayer).l_data as *mut copydata;
    LClearAll(flayer, 0 as libc::c_int);
    y = 0 as libc::c_int;
    x = y;
    cps = (*copydata).cps;
    (*copydata).refcps = cps;
    (*copydata).refsavedcps = (*copydata).savedcps;
    while *cps as libc::c_int != 0 && y < (*flayer).l_height - 3 as libc::c_int {
        ws = cps;
        while *cps as libc::c_int == ' ' as i32 {
            cps = cps.offset(1);
            cps;
        }
        if strncmp(
            cps,
            b"%v\0" as *const u8 as *const libc::c_char,
            2 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            (*copydata).savedcps = cps.offset(2 as libc::c_int as isize);
            cps = version.as_mut_ptr();
        } else {
            while *cps as libc::c_int != 0 && *cps as libc::c_int != ' ' as i32
                && *cps as libc::c_int != '\n' as i32
            {
                cps = cps.offset(1);
                cps;
            }
            l = cps.offset_from(ws) as libc::c_long as libc::c_int;
            cps = ws;
            if l > (*flayer).l_width - 1 as libc::c_int {
                l = (*flayer).l_width - 1 as libc::c_int;
            }
            if x != 0 && x + l >= (*flayer).l_width - 2 as libc::c_int {
                x = 0 as libc::c_int;
                y += 1;
                y;
            } else {
                if x != 0 {
                    LPutChar(flayer, &mut mchar_blank, x, y);
                    x += 1;
                    x;
                }
                if l != 0 {
                    LPutStr(flayer, ws, l, &mut mchar_blank, x, y);
                }
                x += l;
                cps = cps.offset(l as isize);
                if *cps as libc::c_int == 0 as libc::c_int
                    && !((*copydata).savedcps).is_null()
                {
                    cps = (*copydata).savedcps;
                    (*copydata).savedcps = 0 as *mut libc::c_char;
                }
                if *cps as libc::c_int == '\n' as i32 {
                    x = 0 as libc::c_int;
                    y += 1;
                    y;
                }
                if *cps as libc::c_int == ' ' as i32
                    || *cps as libc::c_int == '\n' as i32
                {
                    cps = cps.offset(1);
                    cps;
                }
            }
        }
    }
    while *cps as libc::c_int == '\n' as i32 {
        cps = cps.offset(1);
        cps;
    }
    sprintf(
        cbuf.as_mut_ptr(),
        b"[Press Space %s Return to end.]\0" as *const u8 as *const libc::c_char,
        if *cps as libc::c_int != 0 {
            b"for next page;\0" as *const u8 as *const libc::c_char
        } else {
            b"or\0" as *const u8 as *const libc::c_char
        },
    );
    centerline(cbuf.as_mut_ptr(), (*flayer).l_height - 2 as libc::c_int);
    (*copydata).cps = cps;
    LGotoPos(flayer, (*flayer).l_x, (*flayer).l_y);
}
unsafe extern "C" fn CopyrightRedisplayLine(
    mut y: libc::c_int,
    mut xs: libc::c_int,
    mut xe: libc::c_int,
    mut isblank: libc::c_int,
) {
    if y < 0 as libc::c_int {
        let mut copydata: *mut copydata = 0 as *mut copydata;
        copydata = (*flayer).l_data as *mut copydata;
        (*copydata).cps = (*copydata).refcps;
        (*copydata).savedcps = (*copydata).refsavedcps;
        copypage();
        return;
    }
    if y != 0 as libc::c_int && y != (*flayer).l_height - 1 as libc::c_int {
        return;
    }
    if isblank != 0 {
        return;
    }
    LClearArea(flayer, xs, y, xe, y, 0 as libc::c_int, 0 as libc::c_int);
}
static mut BindkeyLf: LayFuncs = unsafe {
    {
        let mut init = LayFuncs {
            lf_LayProcess: Some(
                BindkeyProcess
                    as unsafe extern "C" fn(
                        *mut *mut libc::c_char,
                        *mut libc::c_int,
                    ) -> (),
            ),
            lf_LayAbort: Some(BindkeyAbort as unsafe extern "C" fn() -> ()),
            lf_LayRedisplayLine: Some(
                BindkeyRedisplayLine
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
                DefRewrite
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
pub unsafe extern "C" fn display_bindkey(
    mut title: *mut libc::c_char,
    mut tab: *mut action,
) {
    let mut bindkeydata: *mut bindkeydata = 0 as *mut bindkeydata;
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    if (*flayer).l_height < 6 as libc::c_int {
        LMsg(
            0 as libc::c_int,
            b"Window height too small for bindkey page\0" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    if InitOverlayPage(
        ::std::mem::size_of::<bindkeydata>() as libc::c_ulong as libc::c_int,
        &mut BindkeyLf,
        0 as libc::c_int,
    ) != 0
    {
        return;
    }
    bindkeydata = (*flayer).l_data as *mut bindkeydata;
    (*bindkeydata).title = title;
    (*bindkeydata).tab = tab;
    n = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i
        < 188 as libc::c_int - 106 as libc::c_int
            + (188 as libc::c_int - 166 as libc::c_int) + kmap_extn
    {
        if (*tab.offset(i as isize)).nr != -(1 as libc::c_int) {
            n += 1;
            n;
        }
        i += 1;
        i;
    }
    (*bindkeydata).pos = 0 as libc::c_int;
    (*bindkeydata).page = 1 as libc::c_int;
    (*bindkeydata)
        .pages = (n + (*flayer).l_height - 6 as libc::c_int)
        / ((*flayer).l_height - 5 as libc::c_int);
    if (*bindkeydata).pages == 0 as libc::c_int {
        (*bindkeydata).pages = 1 as libc::c_int;
    }
    (*flayer).l_x = 0 as libc::c_int;
    (*flayer).l_y = (*flayer).l_height - 1 as libc::c_int;
    bindkeypage();
}
unsafe extern "C" fn BindkeyAbort() {
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
    ExitOverlayPage();
}
unsafe extern "C" fn bindkeypage() {
    let mut bindkeydata: *mut bindkeydata = 0 as *mut bindkeydata;
    let mut kme: *mut kmap_ext = 0 as *mut kmap_ext;
    let mut tbuf: [libc::c_char; 256] = [0; 256];
    let mut del: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut sl: libc::c_int = 0;
    let mut act: *mut action = 0 as *mut action;
    let mut xch: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    bindkeydata = (*flayer).l_data as *mut bindkeydata;
    LClearAll(flayer, 0 as libc::c_int);
    sprintf(
        tbuf.as_mut_ptr(),
        b"%s key bindings, page %d of %d.\0" as *const u8 as *const libc::c_char,
        (*bindkeydata).title,
        (*bindkeydata).page,
        (*bindkeydata).pages,
    );
    centerline(tbuf.as_mut_ptr(), 0 as libc::c_int);
    y = 2 as libc::c_int;
    let mut current_block_39: u64;
    i = (*bindkeydata).pos;
    while i
        < 188 as libc::c_int - 106 as libc::c_int
            + (188 as libc::c_int - 166 as libc::c_int) + kmap_extn
        && y < (*flayer).l_height - 3 as libc::c_int
    {
        p = tbuf.as_mut_ptr();
        xch = b"   \0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        if i < 188 as libc::c_int - 106 as libc::c_int {
            act = &mut *((*bindkeydata).tab).offset(i as isize) as *mut action;
            if (*act).nr == -(1 as libc::c_int) {
                current_block_39 = 11875828834189669668;
            } else {
                let fresh10 = p;
                p = p.offset(1);
                *fresh10 = ':' as i32 as libc::c_char;
                del = *fresh10 as libc::c_int;
                s = (*term.as_mut_ptr().offset((i + 106 as libc::c_int) as isize))
                    .tcname;
                sl = (if !s.is_null() {
                    strlen(s)
                } else {
                    0 as libc::c_int as libc::c_ulong
                }) as libc::c_int;
                current_block_39 = 15768484401365413375;
            }
        } else if i
            < 188 as libc::c_int - 106 as libc::c_int
                + (188 as libc::c_int - 166 as libc::c_int)
        {
            act = &mut *((*bindkeydata).tab).offset(i as isize) as *mut action;
            if (*act).nr == -(1 as libc::c_int) {
                current_block_39 = 11875828834189669668;
            } else {
                let fresh11 = p;
                p = p.offset(1);
                *fresh11 = ':' as i32 as libc::c_char;
                del = *fresh11 as libc::c_int;
                s = (*term
                    .as_mut_ptr()
                    .offset(
                        (i
                            + (106 as libc::c_int - 188 as libc::c_int
                                + 166 as libc::c_int)) as isize,
                    ))
                    .tcname;
                sl = (if !s.is_null() {
                    strlen(s)
                } else {
                    0 as libc::c_int as libc::c_ulong
                }) as libc::c_int;
                xch = b"[A]\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
                current_block_39 = 15768484401365413375;
            }
        } else {
            kme = kmap_exts
                .offset(
                    (i
                        - (188 as libc::c_int - 106 as libc::c_int
                            + (188 as libc::c_int - 166 as libc::c_int))) as isize,
                );
            del = 0 as libc::c_int;
            s = (*kme).str_0;
            sl = (*kme).fl & !(0x4000 as libc::c_int);
            if (*kme).fl & 0x4000 as libc::c_int != 0 as libc::c_int {
                xch = b"[T]\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
            }
            act = if (*bindkeydata).tab == dmtab.as_mut_ptr() {
                &mut (*kme).dm
            } else if (*bindkeydata).tab == mmtab.as_mut_ptr() {
                &mut (*kme).mm
            } else {
                &mut (*kme).um
            };
            if (*act).nr == -(1 as libc::c_int) {
                current_block_39 = 11875828834189669668;
            } else {
                current_block_39 = 15768484401365413375;
            }
        }
        match current_block_39 {
            15768484401365413375 => {
                loop {
                    let fresh12 = sl;
                    sl = sl - 1;
                    if !(fresh12 > 0 as libc::c_int) {
                        break;
                    }
                    let fresh13 = s;
                    s = s.offset(1);
                    p = p
                        .offset(
                            AddXChar(p, *(fresh13 as *mut libc::c_uchar) as libc::c_int)
                                as isize,
                        );
                }
                if del != 0 {
                    let fresh14 = p;
                    p = p.offset(1);
                    *fresh14 = del as libc::c_char;
                }
                let fresh15 = p;
                p = p.offset(1);
                *fresh15 = ' ' as i32 as libc::c_char;
                while p < tbuf.as_mut_ptr().offset(15 as libc::c_int as isize) {
                    let fresh16 = p;
                    p = p.offset(1);
                    *fresh16 = ' ' as i32 as libc::c_char;
                }
                sprintf(p, b"%s -> \0" as *const u8 as *const libc::c_char, xch);
                p = p.offset(7 as libc::c_int as isize);
                if p.offset_from(tbuf.as_mut_ptr()) as libc::c_long
                    > ((*flayer).l_width - 1 as libc::c_int) as libc::c_long
                {
                    tbuf[((*flayer).l_width - 2 as libc::c_int)
                        as usize] = '$' as i32 as libc::c_char;
                    tbuf[((*flayer).l_width - 1 as libc::c_int)
                        as usize] = 0 as libc::c_int as libc::c_char;
                }
                PadStr(
                    tbuf.as_mut_ptr(),
                    strlen(tbuf.as_mut_ptr()) as libc::c_int,
                    0 as libc::c_int,
                    y,
                );
                AddAction(act, strlen(tbuf.as_mut_ptr()) as libc::c_int, y);
                y += 1;
                y;
            }
            _ => {}
        }
        i += 1;
        i;
    }
    y += 1;
    y;
    (*bindkeydata).last = i;
    sprintf(
        tbuf.as_mut_ptr(),
        b"[Press Space %s Return to end.]\0" as *const u8 as *const libc::c_char,
        if (*bindkeydata).page < (*bindkeydata).pages {
            b"for next page;\0" as *const u8 as *const libc::c_char
        } else {
            b"or\0" as *const u8 as *const libc::c_char
        },
    );
    centerline(tbuf.as_mut_ptr(), (*flayer).l_height - 2 as libc::c_int);
    LGotoPos(flayer, (*flayer).l_x, (*flayer).l_y);
}
unsafe extern "C" fn BindkeyProcess(
    mut ppbuf: *mut *mut libc::c_char,
    mut plen: *mut libc::c_int,
) {
    let mut done: libc::c_int = 0 as libc::c_int;
    let mut bindkeydata: *mut bindkeydata = 0 as *mut bindkeydata;
    bindkeydata = (*flayer).l_data as *mut bindkeydata;
    while done == 0 && *plen > 0 as libc::c_int {
        let mut current_block_4: u64;
        match **ppbuf as libc::c_int {
            32 => {
                if (*bindkeydata).page < (*bindkeydata).pages {
                    (*bindkeydata).pos = (*bindkeydata).last;
                    (*bindkeydata).page += 1;
                    (*bindkeydata).page;
                    bindkeypage();
                    current_block_4 = 13513818773234778473;
                } else {
                    current_block_4 = 9550830170014250711;
                }
            }
            13 | 10 => {
                current_block_4 = 9550830170014250711;
            }
            _ => {
                current_block_4 = 13513818773234778473;
            }
        }
        match current_block_4 {
            9550830170014250711 => {
                done = 1 as libc::c_int;
            }
            _ => {}
        }
        *ppbuf = (*ppbuf).offset(1);
        *ppbuf;
        *plen -= 1;
        *plen;
    }
    if done != 0 {
        BindkeyAbort();
    }
}
unsafe extern "C" fn BindkeyRedisplayLine(
    mut y: libc::c_int,
    mut xs: libc::c_int,
    mut xe: libc::c_int,
    mut isblank: libc::c_int,
) {
    if y < 0 as libc::c_int {
        bindkeypage();
        return;
    }
    if y != 0 as libc::c_int && y != (*flayer).l_height - 1 as libc::c_int {
        return;
    }
    if isblank == 0 {
        LClearArea(flayer, xs, y, xe, y, 0 as libc::c_int, 0 as libc::c_int);
    }
}
static mut ZmodemLf: LayFuncs = unsafe {
    {
        let mut init = LayFuncs {
            lf_LayProcess: Some(
                DefProcess
                    as unsafe extern "C" fn(
                        *mut *mut libc::c_char,
                        *mut libc::c_int,
                    ) -> (),
            ),
            lf_LayAbort: None,
            lf_LayRedisplayLine: Some(
                ZmodemRedisplayLine
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
                DefRewrite
                    as unsafe extern "C" fn(
                        libc::c_int,
                        libc::c_int,
                        libc::c_int,
                        *mut mchar,
                        libc::c_int,
                    ) -> libc::c_int,
            ),
            lf_LayResize: Some(
                ZmodemResize
                    as unsafe extern "C" fn(libc::c_int, libc::c_int) -> libc::c_int,
            ),
            lf_LayRestore: Some(DefRestore as unsafe extern "C" fn() -> ()),
            lf_LayFree: None,
        };
        init
    }
};
unsafe extern "C" fn ZmodemResize(
    mut wi: libc::c_int,
    mut he: libc::c_int,
) -> libc::c_int {
    (*flayer).l_width = wi;
    (*flayer).l_height = he;
    (*flayer)
        .l_x = if (*flayer).l_width > 32 as libc::c_int {
        32 as libc::c_int
    } else {
        0 as libc::c_int
    };
    return 0 as libc::c_int;
}
unsafe extern "C" fn ZmodemRedisplayLine(
    mut y: libc::c_int,
    mut xs: libc::c_int,
    mut xe: libc::c_int,
    mut isblank: libc::c_int,
) {
    DefRedisplayLine(y, xs, xe, isblank);
    if y == 0 as libc::c_int && xs == 0 as libc::c_int {
        LPutStr(
            flayer,
            b"Zmodem active on another display\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            if (*flayer).l_width > 32 as libc::c_int {
                32 as libc::c_int
            } else {
                (*flayer).l_width
            },
            &mut mchar_blank,
            0 as libc::c_int,
            0 as libc::c_int,
        );
    }
}
pub unsafe extern "C" fn ZmodemPage() {
    if InitOverlayPage(1 as libc::c_int, &mut ZmodemLf, 1 as libc::c_int) != 0 {
        return;
    }
    LRefreshAll(flayer, 0 as libc::c_int);
    (*flayer)
        .l_x = if (*flayer).l_width > 32 as libc::c_int {
        32 as libc::c_int
    } else {
        0 as libc::c_int
    };
    (*flayer).l_y = 0 as libc::c_int;
}
unsafe extern "C" fn PadStr(
    mut str: *mut libc::c_char,
    mut n: libc::c_int,
    mut x: libc::c_int,
    mut y: libc::c_int,
) {
    let mut l: libc::c_int = 0;
    l = strlen(str) as libc::c_int;
    if l > n {
        l = n;
    }
    LPutStr(flayer, str, l, &mut mchar_blank, x, y);
    if l < n {
        LPutStr(flayer, blank as *mut libc::c_char, n - l, &mut mchar_blank, x + l, y);
    }
}
