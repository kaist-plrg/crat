use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type logfile;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn abort() -> !;
    fn bcopy(__src: *const libc::c_void, __dest: *mut libc::c_void, __n: size_t);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
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
    fn SaveStr(_: *const libc::c_char) -> *mut libc::c_char;
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
    fn LClearArea(
        _: *mut layer,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
    );
    fn LMsg(_: libc::c_int, _: *const libc::c_char, _: ...);
    fn InitOverlayPage(_: libc::c_int, _: *mut LayFuncs, _: libc::c_int) -> libc::c_int;
    fn ExitOverlayPage();
    static mut flayer: *mut layer;
    static mut display: *mut display;
    static mut mchar_blank: mchar;
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
pub struct inpline {
    pub buf: [libc::c_char; 769],
    pub len: libc::c_int,
    pub pos: libc::c_int,
    pub next: *mut inpline,
    pub prev: *mut inpline,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct inpdata {
    pub inp: inpline,
    pub inpmaxlen: libc::c_int,
    pub inpstring: *mut libc::c_char,
    pub inpstringlen: libc::c_int,
    pub inpmode: libc::c_int,
    pub inpfinfunc: Option::<
        unsafe extern "C" fn(*mut libc::c_char, libc::c_int, *mut libc::c_char) -> (),
    >,
    pub priv_0: *mut libc::c_char,
    pub privdata: libc::c_int,
    pub search: *mut libc::c_char,
}
static mut inphist: inpline = inpline {
    buf: [0; 769],
    len: 0,
    pos: 0,
    next: 0 as *const inpline as *mut inpline,
    prev: 0 as *const inpline as *mut inpline,
};
static mut InpLf: LayFuncs = unsafe {
    {
        let mut init = LayFuncs {
            lf_LayProcess: Some(
                InpProcess
                    as unsafe extern "C" fn(
                        *mut *mut libc::c_char,
                        *mut libc::c_int,
                    ) -> (),
            ),
            lf_LayAbort: Some(InpAbort as unsafe extern "C" fn() -> ()),
            lf_LayRedisplayLine: Some(
                InpRedisplayLine
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
pub unsafe extern "C" fn inp_setprompt(
    mut p: *mut libc::c_char,
    mut s: *mut libc::c_char,
) {
    let mut inpdata: *mut inpdata = 0 as *mut inpdata;
    inpdata = (*flayer).l_data as *mut inpdata;
    if !p.is_null() {
        (*inpdata).inpstringlen = strlen(p) as libc::c_int;
        (*inpdata).inpstring = p;
    }
    if !s.is_null() {
        if s != ((*inpdata).inp.buf).as_mut_ptr() {
            strncpy(
                ((*inpdata).inp.buf).as_mut_ptr(),
                s,
                (::std::mem::size_of::<[libc::c_char; 769]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            );
        }
        (*inpdata)
            .inp
            .buf[(::std::mem::size_of::<[libc::c_char; 769]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            as usize] = 0 as libc::c_int as libc::c_char;
        (*inpdata).inp.len = strlen(((*inpdata).inp.buf).as_mut_ptr()) as libc::c_int;
        (*inpdata).inp.pos = (*inpdata).inp.len;
    }
    InpRedisplayLine(
        (*flayer).l_height - 1 as libc::c_int,
        0 as libc::c_int,
        (*flayer).l_width - 1 as libc::c_int,
        0 as libc::c_int,
    );
    (*flayer)
        .l_x = (*inpdata).inpstringlen
        + (if (*inpdata).inpmode & 1 as libc::c_int != 0 {
            0 as libc::c_int
        } else {
            (*inpdata).inp.pos
        });
    (*flayer).l_y = (*flayer).l_height - 1 as libc::c_int;
}
pub unsafe extern "C" fn Input(
    mut istr: *mut libc::c_char,
    mut len: libc::c_int,
    mut mode: libc::c_int,
    mut finfunc: Option::<
        unsafe extern "C" fn(*mut libc::c_char, libc::c_int, *mut libc::c_char) -> (),
    >,
    mut priv_0: *mut libc::c_char,
    mut data: libc::c_int,
) {
    let mut maxlen: libc::c_int = 0;
    let mut inpdata: *mut inpdata = 0 as *mut inpdata;
    if flayer.is_null() {
        return;
    }
    if len > 768 as libc::c_int {
        len = 768 as libc::c_int;
    }
    if mode & 1 as libc::c_int == 0 {
        maxlen = (((*flayer).l_width - 1 as libc::c_int) as libc::c_ulong)
            .wrapping_sub(strlen(istr)) as libc::c_int;
        if len > maxlen {
            len = maxlen;
        }
    }
    if len < 0 as libc::c_int {
        LMsg(
            0 as libc::c_int,
            b"Width %d chars too small\0" as *const u8 as *const libc::c_char,
            -len,
        );
        return;
    }
    if InitOverlayPage(
        ::std::mem::size_of::<inpdata>() as libc::c_ulong as libc::c_int,
        &mut InpLf,
        1 as libc::c_int,
    ) != 0
    {
        return;
    }
    (*flayer).l_mode = 1 as libc::c_int;
    inpdata = (*flayer).l_data as *mut inpdata;
    (*inpdata).inpmaxlen = len;
    (*inpdata).inpfinfunc = finfunc;
    (*inpdata).inp.len = 0 as libc::c_int;
    (*inpdata).inp.pos = (*inpdata).inp.len;
    (*inpdata).inp.prev = inphist.prev;
    (*inpdata).inpmode = mode;
    (*inpdata).privdata = data;
    if priv_0.is_null() {
        priv_0 = &mut (*inpdata).privdata as *mut libc::c_int as *mut libc::c_char;
    }
    (*inpdata).priv_0 = priv_0;
    (*inpdata).inpstringlen = 0 as libc::c_int;
    (*inpdata).inpstring = 0 as *mut libc::c_char;
    (*inpdata).search = 0 as *mut libc::c_char;
    if !istr.is_null() {
        inp_setprompt(istr, 0 as *mut libc::c_void as *mut libc::c_char);
    }
}
unsafe extern "C" fn erase_chars(
    mut inpdata: *mut inpdata,
    mut from: *mut libc::c_char,
    mut to: *mut libc::c_char,
    mut x: libc::c_int,
    mut mv: libc::c_int,
) {
    let mut chng: libc::c_int = 0;
    if (*inpdata).inp.len as libc::c_long
        > to.offset_from(((*inpdata).inp.buf).as_mut_ptr()) as libc::c_long
    {
        bcopy(
            to as *const libc::c_void,
            from as *mut libc::c_void,
            ((*inpdata).inp.len as libc::c_long
                - to.offset_from(((*inpdata).inp.buf).as_mut_ptr()) as libc::c_long)
                as size_t,
        );
    }
    chng = to.offset_from(from) as libc::c_long as libc::c_int;
    if mv != 0 {
        x -= chng;
        (*inpdata).inp.pos -= chng;
    }
    (*inpdata).inp.len -= chng;
    if (*inpdata).inpmode & 1 as libc::c_int == 0 {
        let mut mc: mchar = mchar {
            image: 0,
            attr: 0,
            font: 0,
            fontx: 0,
            color: 0,
            mbcs: 0,
        };
        let mut s: *mut libc::c_char = if from < to { from } else { to };
        mc = mchar_so;
        while s < ((*inpdata).inp.buf).as_mut_ptr().offset((*inpdata).inp.len as isize) {
            let fresh0 = s;
            s = s.offset(1);
            mc.image = *fresh0 as libc::c_uchar;
            let fresh1 = x;
            x = x + 1;
            LPutChar(flayer, &mut mc, fresh1, (*flayer).l_height - 1 as libc::c_int);
        }
        loop {
            let fresh2 = chng;
            chng = chng - 1;
            if !(fresh2 != 0) {
                break;
            }
            let fresh3 = x;
            x = x + 1;
            LPutChar(
                flayer,
                &mut mchar_blank,
                fresh3,
                (*flayer).l_height - 1 as libc::c_int,
            );
        }
        x = (*inpdata).inpstringlen + (*inpdata).inp.pos;
        LGotoPos(flayer, x, (*flayer).l_height - 1 as libc::c_int);
    }
}
unsafe extern "C" fn InpProcess(
    mut ppbuf: *mut *mut libc::c_char,
    mut plen: *mut libc::c_int,
) {
    let mut len: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut pbuf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ch: libc::c_char = 0;
    let mut inpdata: *mut inpdata = 0 as *mut inpdata;
    let mut inpdisplay: *mut display = 0 as *mut display;
    let mut prev: libc::c_int = 0;
    let mut next: libc::c_int = 0;
    let mut search: libc::c_int = 0 as libc::c_int;
    inpdata = (*flayer).l_data as *mut inpdata;
    inpdisplay = display;
    LGotoPos(
        flayer,
        (*inpdata).inpstringlen
            + (if (*inpdata).inpmode & 1 as libc::c_int != 0 {
                0 as libc::c_int
            } else {
                (*inpdata).inp.pos
            }),
        (*flayer).l_height - 1 as libc::c_int,
    );
    if ppbuf.is_null() {
        InpAbort();
        return;
    }
    x = (*inpdata).inpstringlen + (*inpdata).inp.pos;
    len = *plen;
    pbuf = *ppbuf;
    while len != 0 {
        let mut p: *mut libc::c_char = ((*inpdata).inp.buf)
            .as_mut_ptr()
            .offset((*inpdata).inp.pos as isize);
        let fresh4 = pbuf;
        pbuf = pbuf.offset(1);
        ch = *fresh4;
        len -= 1;
        len;
        if (*inpdata).inpmode & 4 as libc::c_int != 0 {
            (*inpdata).inp.buf[(*inpdata).inp.len as usize] = ch;
            if ch != 0 {
                display = inpdisplay;
                (Some(((*inpdata).inpfinfunc).unwrap()))
                    .unwrap()(
                    ((*inpdata).inp.buf).as_mut_ptr(),
                    (*inpdata).inp.len,
                    (*inpdata).priv_0,
                );
                ch = (*inpdata).inp.buf[(*inpdata).inp.len as usize];
            }
        } else if (*inpdata).inpmode & 2 as libc::c_int != 0 {
            display = inpdisplay;
            (Some(((*inpdata).inpfinfunc).unwrap()))
                .unwrap()(&mut ch, 1 as libc::c_int, (*inpdata).priv_0);
            if ch != 0 {
                continue;
            }
        }
        if ch as libc::c_uchar as libc::c_int & 0o177 as libc::c_int >= ' ' as i32
            && ch as libc::c_int != 0o177 as libc::c_int
            && (*inpdata).inp.len < (*inpdata).inpmaxlen
        {
            if (*inpdata).inp.len > (*inpdata).inp.pos {
                bcopy(
                    p as *const libc::c_void,
                    p.offset(1 as libc::c_int as isize) as *mut libc::c_void,
                    ((*inpdata).inp.len - (*inpdata).inp.pos) as size_t,
                );
            }
            let fresh5 = (*inpdata).inp.pos;
            (*inpdata).inp.pos = (*inpdata).inp.pos + 1;
            (*inpdata).inp.buf[fresh5 as usize] = ch;
            (*inpdata).inp.len += 1;
            (*inpdata).inp.len;
            if (*inpdata).inpmode & 1 as libc::c_int == 0 {
                let mut mc: mchar = mchar {
                    image: 0,
                    attr: 0,
                    font: 0,
                    fontx: 0,
                    color: 0,
                    mbcs: 0,
                };
                mc = mchar_so;
                let fresh6 = p;
                p = p.offset(1);
                mc.image = *fresh6 as libc::c_uchar;
                LPutChar(flayer, &mut mc, x, (*flayer).l_height - 1 as libc::c_int);
                x += 1;
                x;
                if p
                    < ((*inpdata).inp.buf)
                        .as_mut_ptr()
                        .offset((*inpdata).inp.len as isize)
                {
                    while p
                        < ((*inpdata).inp.buf)
                            .as_mut_ptr()
                            .offset((*inpdata).inp.len as isize)
                    {
                        let fresh7 = p;
                        p = p.offset(1);
                        mc.image = *fresh7 as libc::c_uchar;
                        let fresh8 = x;
                        x = x + 1;
                        LPutChar(
                            flayer,
                            &mut mc,
                            fresh8,
                            (*flayer).l_height - 1 as libc::c_int,
                        );
                    }
                    x = (*inpdata).inpstringlen + (*inpdata).inp.pos;
                    LGotoPos(flayer, x, (*flayer).l_height - 1 as libc::c_int);
                }
            }
            if !((*inpdata).search).is_null() {
                if ((*inpdata).search).is_null() {
                    abort();
                } else {
                    free((*inpdata).search as *mut libc::c_void);
                }
                (*inpdata).search = 0 as *mut libc::c_char;
            }
        } else if (ch as libc::c_int == '\u{8}' as i32
            || ch as libc::c_int == 0o177 as libc::c_int)
            && (*inpdata).inp.pos > 0 as libc::c_int
        {
            erase_chars(
                inpdata,
                p.offset(-(1 as libc::c_int as isize)),
                p,
                x,
                1 as libc::c_int,
            );
            if !((*inpdata).search).is_null() {
                if ((*inpdata).search).is_null() {
                    abort();
                } else {
                    free((*inpdata).search as *mut libc::c_void);
                }
                (*inpdata).search = 0 as *mut libc::c_char;
            }
        } else if ch as libc::c_int == '\u{15}' as i32 {
            x = (*inpdata).inpstringlen;
            if (*inpdata).inp.len != 0 && (*inpdata).inpmode & 1 as libc::c_int == 0 {
                LClearArea(
                    flayer,
                    x,
                    (*flayer).l_height - 1 as libc::c_int,
                    x + (*inpdata).inp.len - 1 as libc::c_int,
                    (*flayer).l_height - 1 as libc::c_int,
                    0 as libc::c_int,
                    0 as libc::c_int,
                );
                LGotoPos(flayer, x, (*flayer).l_height - 1 as libc::c_int);
            }
            (*inpdata).inp.pos = 0 as libc::c_int;
            (*inpdata).inp.len = (*inpdata).inp.pos;
        } else if ch as libc::c_int == '\u{b}' as i32 {
            x = (*inpdata).inpstringlen + (*inpdata).inp.pos;
            if (*inpdata).inp.len > (*inpdata).inp.pos
                && (*inpdata).inpmode & 1 as libc::c_int == 0
            {
                LClearArea(
                    flayer,
                    x,
                    (*flayer).l_height - 1 as libc::c_int,
                    x + (*inpdata).inp.len - (*inpdata).inp.pos - 1 as libc::c_int,
                    (*flayer).l_height - 1 as libc::c_int,
                    0 as libc::c_int,
                    0 as libc::c_int,
                );
                LGotoPos(flayer, x, (*flayer).l_height - 1 as libc::c_int);
            }
            (*inpdata).inp.len = (*inpdata).inp.pos;
        } else if ch as libc::c_int == '\u{17}' as i32
            && (*inpdata).inp.pos > 0 as libc::c_int
        {
            let fresh9 = p;
            p = p.offset(-1);
            let mut oldp: *mut libc::c_char = fresh9;
            while p > ((*inpdata).inp.buf).as_mut_ptr()
                && *p as libc::c_int == ' ' as i32
            {
                p = p.offset(-1);
                p;
            }
            while p > ((*inpdata).inp.buf).as_mut_ptr()
                && *p.offset(-(1 as libc::c_int as isize)) as libc::c_int != ' ' as i32
            {
                p = p.offset(-1);
                p;
            }
            erase_chars(inpdata, p, oldp, x, 1 as libc::c_int);
            if !((*inpdata).search).is_null() {
                if ((*inpdata).search).is_null() {
                    abort();
                } else {
                    free((*inpdata).search as *mut libc::c_void);
                }
                (*inpdata).search = 0 as *mut libc::c_char;
            }
        } else if ch as libc::c_int == '\u{4}' as i32
            && (*inpdata).inp.pos < (*inpdata).inp.len
        {
            erase_chars(
                inpdata,
                p,
                p.offset(1 as libc::c_int as isize),
                x,
                0 as libc::c_int,
            );
            if !((*inpdata).search).is_null() {
                if ((*inpdata).search).is_null() {
                    abort();
                } else {
                    free((*inpdata).search as *mut libc::c_void);
                }
                (*inpdata).search = 0 as *mut libc::c_char;
            }
        } else if ch as libc::c_int == '\u{1}' as i32
            || ch as libc::c_uchar as libc::c_int == 0o201 as libc::c_int
        {
            x -= (*inpdata).inp.pos;
            LGotoPos(flayer, x, (*flayer).l_height - 1 as libc::c_int);
            (*inpdata).inp.pos = 0 as libc::c_int;
        } else if (ch as libc::c_int == '\u{2}' as i32
            || ch as libc::c_uchar as libc::c_int == 0o202 as libc::c_int)
            && (*inpdata).inp.pos > 0 as libc::c_int
        {
            x -= 1;
            LGotoPos(flayer, x, (*flayer).l_height - 1 as libc::c_int);
            (*inpdata).inp.pos -= 1;
            (*inpdata).inp.pos;
        } else if ch as libc::c_int == '\u{5}' as i32
            || ch as libc::c_uchar as libc::c_int == 0o205 as libc::c_int
        {
            x += (*inpdata).inp.len - (*inpdata).inp.pos;
            LGotoPos(flayer, x, (*flayer).l_height - 1 as libc::c_int);
            (*inpdata).inp.pos = (*inpdata).inp.len;
        } else if (ch as libc::c_int == '\u{6}' as i32
            || ch as libc::c_uchar as libc::c_int == 0o206 as libc::c_int)
            && (*inpdata).inp.pos < (*inpdata).inp.len
        {
            x += 1;
            LGotoPos(flayer, x, (*flayer).l_height - 1 as libc::c_int);
            (*inpdata).inp.pos += 1;
            (*inpdata).inp.pos;
        } else {
            prev = ((ch as libc::c_int == '\u{10}' as i32
                || ch as libc::c_uchar as libc::c_int == 0o220 as libc::c_int)
                && !((*inpdata).inp.prev).is_null()) as libc::c_int;
            if prev != 0
                || {
                    next = ((ch as libc::c_int == '\u{e}' as i32
                        || ch as libc::c_uchar as libc::c_int == 0o216 as libc::c_int)
                        && !((*inpdata).inp.next).is_null()) as libc::c_int;
                    next != 0
                }
                || {
                    search = ((ch as libc::c_int == '\u{12}' as i32
                        || ch as libc::c_uchar as libc::c_int == 0o222 as libc::c_int)
                        && !((*inpdata).inp.prev).is_null()) as libc::c_int;
                    search != 0
                }
            {
                let mut mc_0: mchar = mchar {
                    image: 0,
                    attr: 0,
                    font: 0,
                    fontx: 0,
                    color: 0,
                    mbcs: 0,
                };
                let mut sel: *mut inpline = 0 as *mut inpline;
                let mut pos: libc::c_int = -(1 as libc::c_int);
                mc_0 = mchar_so;
                if prev != 0 {
                    sel = (*inpdata).inp.prev;
                } else if next != 0 {
                    sel = (*inpdata).inp.next;
                } else {
                    (*inpdata)
                        .inp
                        .buf[(*inpdata).inp.len
                        as usize] = 0 as libc::c_int as libc::c_char;
                    if ((*inpdata).search).is_null() {
                        (*inpdata).search = SaveStr(((*inpdata).inp.buf).as_mut_ptr());
                    }
                    sel = (*inpdata).inp.prev;
                    while !sel.is_null() {
                        let mut f: *mut libc::c_char = 0 as *mut libc::c_char;
                        f = strstr(((*sel).buf).as_mut_ptr(), (*inpdata).search);
                        if !f.is_null() {
                            pos = f.offset_from(((*sel).buf).as_mut_ptr())
                                as libc::c_long as libc::c_int;
                            break;
                        } else {
                            sel = (*sel).prev;
                        }
                    }
                    if sel.is_null() {
                        continue;
                    }
                }
                if (*inpdata).inp.len != 0 && (*inpdata).inpmode & 1 as libc::c_int == 0
                {
                    LClearArea(
                        flayer,
                        (*inpdata).inpstringlen,
                        (*flayer).l_height - 1 as libc::c_int,
                        (*inpdata).inpstringlen + (*inpdata).inp.len - 1 as libc::c_int,
                        (*flayer).l_height - 1 as libc::c_int,
                        0 as libc::c_int,
                        0 as libc::c_int,
                    );
                }
                if (prev != 0 || search != 0) && ((*inpdata).inp.next).is_null() {
                    inphist = (*inpdata).inp;
                }
                memcpy(
                    &mut (*inpdata).inp as *mut inpline as *mut libc::c_void,
                    sel as *const libc::c_void,
                    ::std::mem::size_of::<inpline>() as libc::c_ulong,
                );
                if pos != -(1 as libc::c_int) {
                    (*inpdata).inp.pos = pos;
                }
                if (*inpdata).inp.len > (*inpdata).inpmaxlen {
                    (*inpdata).inp.len = (*inpdata).inpmaxlen;
                }
                if (*inpdata).inp.pos > (*inpdata).inp.len {
                    (*inpdata).inp.pos = (*inpdata).inp.len;
                }
                x = (*inpdata).inpstringlen;
                p = ((*inpdata).inp.buf).as_mut_ptr();
                if (*inpdata).inpmode & 1 as libc::c_int == 0 {
                    while p
                        < ((*inpdata).inp.buf)
                            .as_mut_ptr()
                            .offset((*inpdata).inp.len as isize)
                    {
                        let fresh10 = p;
                        p = p.offset(1);
                        mc_0.image = *fresh10 as libc::c_uchar;
                        let fresh11 = x;
                        x = x + 1;
                        LPutChar(
                            flayer,
                            &mut mc_0,
                            fresh11,
                            (*flayer).l_height - 1 as libc::c_int,
                        );
                    }
                }
                x = (*inpdata).inpstringlen + (*inpdata).inp.pos;
                LGotoPos(flayer, x, (*flayer).l_height - 1 as libc::c_int);
            } else if ch as libc::c_int == '\u{3}' as i32
                || ch as libc::c_int == '\u{7}' as i32
                || ch as libc::c_int == '\u{1b}' as i32
                || ch as libc::c_int == '\0' as i32 || ch as libc::c_int == '\n' as i32
                || ch as libc::c_int == '\r' as i32
            {
                if ch as libc::c_int != '\n' as i32 && ch as libc::c_int != '\r' as i32 {
                    (*inpdata).inp.len = 0 as libc::c_int;
                }
                (*inpdata)
                    .inp
                    .buf[(*inpdata).inp.len as usize] = 0 as libc::c_int as libc::c_char;
                if (*inpdata).inp.len != 0
                    && (*inpdata).inpmode & (1 as libc::c_int | 2 as libc::c_int) == 0
                {
                    let mut store: *mut inpline = 0 as *mut inpline;
                    store = inphist.prev;
                    while !store.is_null() {
                        if strcmp(
                            ((*store).buf).as_mut_ptr(),
                            ((*inpdata).inp.buf).as_mut_ptr(),
                        ) == 0 as libc::c_int
                        {
                            if !((*store).next).is_null() {
                                (*(*store).next).prev = (*store).prev;
                            }
                            if !((*store).prev).is_null() {
                                (*(*store).prev).next = (*store).next;
                            }
                            (*store).pos = (*inpdata).inp.pos;
                            break;
                        } else {
                            store = (*store).prev;
                        }
                    }
                    if store.is_null() {
                        store = malloc(::std::mem::size_of::<inpline>() as libc::c_ulong)
                            as *mut inpline;
                        memcpy(
                            store as *mut libc::c_void,
                            &mut (*inpdata).inp as *mut inpline as *const libc::c_void,
                            ::std::mem::size_of::<inpline>() as libc::c_ulong,
                        );
                    }
                    (*store).next = &mut inphist;
                    (*store).prev = inphist.prev;
                    if !(inphist.prev).is_null() {
                        (*inphist.prev).next = store;
                    }
                    inphist.prev = store;
                }
                (*flayer).l_data = 0 as *mut libc::c_void;
                InpAbort();
                *ppbuf = pbuf;
                *plen = len;
                display = inpdisplay;
                if (*inpdata).inpmode & 2 as libc::c_int == 0 as libc::c_int {
                    (Some(((*inpdata).inpfinfunc).unwrap()))
                        .unwrap()(
                        ((*inpdata).inp.buf).as_mut_ptr(),
                        (*inpdata).inp.len,
                        (*inpdata).priv_0,
                    );
                } else {
                    (Some(((*inpdata).inpfinfunc).unwrap()))
                        .unwrap()(
                        pbuf.offset(-(1 as libc::c_int as isize)),
                        0 as libc::c_int,
                        (*inpdata).priv_0,
                    );
                }
                if !((*inpdata).search).is_null() {
                    free((*inpdata).search as *mut libc::c_void);
                }
                free(inpdata as *mut libc::c_void);
                return;
            } else if !((*inpdata).search).is_null() {
                if ((*inpdata).search).is_null() {
                    abort();
                } else {
                    free((*inpdata).search as *mut libc::c_void);
                }
                (*inpdata).search = 0 as *mut libc::c_char;
            }
        }
    }
    if (*inpdata).inpmode & 2 as libc::c_int == 0 {
        (*flayer)
            .l_x = (*inpdata).inpstringlen
            + (if (*inpdata).inpmode & 1 as libc::c_int != 0 {
                0 as libc::c_int
            } else {
                (*inpdata).inp.pos
            });
        (*flayer).l_y = (*flayer).l_height - 1 as libc::c_int;
    }
    *ppbuf = pbuf;
    *plen = len;
}
unsafe extern "C" fn InpAbort() {
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
    (Some(((*(*flayer).l_layfn).lf_LayRedisplayLine).unwrap()))
        .unwrap()(
        (*flayer).l_height - 1 as libc::c_int,
        0 as libc::c_int,
        (*flayer).l_width - 1 as libc::c_int,
        0 as libc::c_int,
    );
    flayer = oldlay;
    cv = (*flayer).l_cvlist;
    while !cv.is_null() {
        (*cv).c_layer = flayer;
        cv = (*cv).c_lnext;
    }
    (*(*flayer).l_next).l_cvlist = oldcvlist;
    ExitOverlayPage();
}
unsafe extern "C" fn InpRedisplayLine(
    mut y: libc::c_int,
    mut xs: libc::c_int,
    mut xe: libc::c_int,
    mut isblank: libc::c_int,
) {
    let mut q: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut s: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut v: libc::c_int = 0;
    let mut inpdata: *mut inpdata = 0 as *mut inpdata;
    inpdata = (*flayer).l_data as *mut inpdata;
    if y != (*flayer).l_height - 1 as libc::c_int {
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
        (Some(((*(*flayer).l_layfn).lf_LayRedisplayLine).unwrap()))
            .unwrap()(y, xs, xe, isblank);
        flayer = oldlay;
        cv = (*flayer).l_cvlist;
        while !cv.is_null() {
            (*cv).c_layer = flayer;
            cv = (*cv).c_lnext;
        }
        (*(*flayer).l_next).l_cvlist = oldcvlist;
        return;
    }
    (*inpdata).inp.buf[(*inpdata).inp.len as usize] = 0 as libc::c_int as libc::c_char;
    q = xs;
    v = xe - xs + 1 as libc::c_int;
    s = 0 as libc::c_int;
    r = (*inpdata).inpstringlen;
    if v > 0 as libc::c_int && q < r {
        l = v;
        if l > r - q {
            l = r - q;
        }
        LPutStr(
            flayer,
            ((*inpdata).inpstring).offset(q as isize).offset(-(s as isize)),
            l,
            &mut mchar_so,
            q,
            y,
        );
        q += l;
        v -= l;
    }
    s = r;
    r += (*inpdata).inp.len;
    if (*inpdata).inpmode & 1 as libc::c_int == 0 && v > 0 as libc::c_int && q < r {
        l = v;
        if l > r - q {
            l = r - q;
        }
        LPutStr(
            flayer,
            ((*inpdata).inp.buf).as_mut_ptr().offset(q as isize).offset(-(s as isize)),
            l,
            &mut mchar_so,
            q,
            y,
        );
        q += l;
        v -= l;
    }
    s = r;
    r = (*flayer).l_width;
    if isblank == 0 && v > 0 as libc::c_int && q < r {
        l = v;
        if l > r - q {
            l = r - q;
        }
        LClearArea(
            flayer,
            q,
            y,
            q + l - 1 as libc::c_int,
            y,
            0 as libc::c_int,
            0 as libc::c_int,
        );
        q += l;
    }
}
