use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type logfile;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn LayerCleanupMemory(layer: *mut layer);
    fn RethinkViewportOffsets(_: *mut canvas);
    fn RethinkDisplayViewports() -> libc::c_int;
    fn WindowChanged(_: *mut win, _: libc::c_int);
    fn WListLinkChanged();
    fn ReleaseAutoWritelock(_: *mut display, _: *mut win) -> libc::c_int;
    fn ObtainAutoWritelock(_: *mut display, _: *mut win) -> libc::c_int;
    fn RefreshLine(_: libc::c_int, _: libc::c_int, _: libc::c_int, _: libc::c_int);
    fn GotoPos(_: libc::c_int, _: libc::c_int);
    fn ResizeLayersToCanvases();
    fn evenq(_: *mut event);
    fn evdeq(_: *mut event);
    fn SetTimeout(_: *mut event, _: libc::c_int);
    fn KillLayerChain(_: *mut layer);
    fn display_windows(onblank: libc::c_int, order: libc::c_int, group: *mut win);
    static mut display: *mut display;
    static mut fore: *mut win;
    static mut windows: *mut win;
    static mut flayer: *mut layer;
    static mut captionalways: libc::c_int;
    static mut BlankLf: LayFuncs;
    static mut focusminwidth: libc::c_int;
    static mut focusminheight: libc::c_int;
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
unsafe extern "C" fn CanvasInitBlank(mut cv: *mut canvas) {
    (*cv).c_blank.l_cvlist = cv;
    (*cv).c_blank.l_width = (*cv).c_xe - (*cv).c_xs + 1 as libc::c_int;
    (*cv).c_blank.l_height = (*cv).c_ye - (*cv).c_ys + 1 as libc::c_int;
    (*cv).c_blank.l_y = 0 as libc::c_int;
    (*cv).c_blank.l_x = (*cv).c_blank.l_y;
    (*cv).c_blank.l_layfn = &mut BlankLf;
    (*cv).c_blank.l_data = 0 as *mut libc::c_void;
    (*cv).c_blank.l_next = 0 as *mut layer;
    (*cv).c_blank.l_bottom = &mut (*cv).c_blank;
    (*cv).c_blank.l_blocking = 0 as libc::c_int;
    (*cv).c_layer = &mut (*cv).c_blank;
}
unsafe extern "C" fn FreePerp(mut pcv: *mut canvas) {
    let mut cv: *mut canvas = 0 as *mut canvas;
    if ((*pcv).c_slperp).is_null() {
        return;
    }
    cv = (*pcv).c_slperp;
    (*cv).c_slprev = (*pcv).c_slprev;
    if !((*cv).c_slprev).is_null() {
        (*(*cv).c_slprev).c_slnext = cv;
    }
    (*cv).c_slback = (*pcv).c_slback;
    if !((*cv).c_slback).is_null() && (*(*cv).c_slback).c_slperp == pcv {
        (*(*cv).c_slback).c_slperp = cv;
    }
    (*cv).c_slorient = (*pcv).c_slorient;
    (*cv).c_slweight = (*pcv).c_slweight;
    while !((*cv).c_slnext).is_null() {
        cv = (*cv).c_slnext;
        (*cv).c_slorient = (*pcv).c_slorient;
        (*cv).c_slback = (*pcv).c_slback;
        (*cv).c_slweight = (*pcv).c_slweight;
    }
    (*cv).c_slnext = (*pcv).c_slnext;
    if !((*cv).c_slnext).is_null() {
        (*(*cv).c_slnext).c_slprev = cv;
    }
    LayerCleanupMemory(&mut (*pcv).c_blank);
    free(pcv as *mut libc::c_void);
}
pub unsafe extern "C" fn FreeCanvas(mut cv: *mut canvas) {
    let mut vp: *mut viewport = 0 as *mut viewport;
    let mut nvp: *mut viewport = 0 as *mut viewport;
    let mut cvp: *mut *mut canvas = 0 as *mut *mut canvas;
    let mut p: *mut win = 0 as *mut win;
    if !((*cv).c_slprev).is_null() {
        (*(*cv).c_slprev).c_slnext = (*cv).c_slnext;
    }
    if !((*cv).c_slnext).is_null() {
        (*(*cv).c_slnext).c_slprev = (*cv).c_slprev;
    }
    if !((*cv).c_slback).is_null() && (*(*cv).c_slback).c_slperp == cv {
        (*(*cv).c_slback)
            .c_slperp = if !((*cv).c_slnext).is_null() {
            (*cv).c_slnext
        } else {
            (*cv).c_slprev
        };
    }
    if !((*cv).c_slperp).is_null() {
        while !((*cv).c_slperp).is_null() {
            FreeCanvas((*cv).c_slperp);
        }
        LayerCleanupMemory(&mut (*cv).c_blank);
        free(cv as *mut libc::c_void);
        return;
    }
    if !display.is_null() {
        if (*display).d_forecv == cv {
            (*display).d_forecv = 0 as *mut canvas;
        }
        cvp = &mut (*display).d_cvlist;
        while !(*cvp).is_null() {
            if *cvp == cv {
                *cvp = (*cv).c_next;
                break;
            } else {
                cvp = &mut (**cvp).c_next;
            }
        }
    }
    p = if !((*cv).c_layer).is_null() {
        (*(*(*cv).c_layer).l_bottom).l_data as *mut win
    } else {
        0 as *mut win
    };
    SetCanvasWindow(cv, 0 as *mut win);
    if !p.is_null() {
        WindowChanged(p, 'u' as i32);
    }
    if flayer == (*cv).c_layer {
        flayer = 0 as *mut layer;
    }
    vp = (*cv).c_vplist;
    while !vp.is_null() {
        (*vp).v_canvas = 0 as *mut canvas;
        nvp = (*vp).v_next;
        (*vp).v_next = 0 as *mut viewport;
        free(vp as *mut libc::c_void);
        vp = nvp;
    }
    evdeq(&mut (*cv).c_captev);
    LayerCleanupMemory(&mut (*cv).c_blank);
    free(cv as *mut libc::c_void);
}
pub unsafe extern "C" fn CountCanvas(mut cv: *mut canvas) -> libc::c_int {
    let mut num: libc::c_int = 0 as libc::c_int;
    while !cv.is_null() {
        if !((*cv).c_slperp).is_null() {
            let mut cvp: *mut canvas = 0 as *mut canvas;
            let mut nump: libc::c_int = 1 as libc::c_int;
            let mut n: libc::c_int = 0;
            cvp = (*cv).c_slperp;
            while !cvp.is_null() {
                if !((*cvp).c_slperp).is_null() {
                    n = CountCanvas((*cvp).c_slperp);
                    if n > nump {
                        nump = n;
                    }
                }
                cvp = (*cvp).c_slnext;
            }
            num += nump;
        } else {
            num += 1;
            num;
        }
        cv = (*cv).c_slnext;
    }
    return num;
}
pub unsafe extern "C" fn CountCanvasPerp(mut cv: *mut canvas) -> libc::c_int {
    let mut cvp: *mut canvas = 0 as *mut canvas;
    let mut num: libc::c_int = 1 as libc::c_int;
    let mut n: libc::c_int = 0;
    cvp = (*cv).c_slperp;
    while !cvp.is_null() {
        if !((*cvp).c_slperp).is_null() {
            n = CountCanvas((*cvp).c_slperp);
            if n > num {
                num = n;
            }
        }
        cvp = (*cvp).c_slnext;
    }
    return num;
}
pub unsafe extern "C" fn FindCanvas(
    mut x: libc::c_int,
    mut y: libc::c_int,
) -> *mut canvas {
    let mut cv: *mut canvas = 0 as *mut canvas;
    let mut mcv: *mut canvas = 0 as *mut canvas;
    let mut m: libc::c_int = 0;
    let mut mm: libc::c_int = 0 as libc::c_int;
    let mut current_block_15: u64;
    cv = (*display).d_cvlist;
    while !cv.is_null() {
        if x >= (*cv).c_xs && x <= (*cv).c_xe && y >= (*cv).c_ys
            && y <= (*cv).c_ye + 1 as libc::c_int
        {
            return cv;
        }
        if !(cv == (*display).d_forecv) {
            m = 0 as libc::c_int;
            if x >= (*(*display).d_forecv).c_xs && x <= (*(*display).d_forecv).c_xe {
                if x < (*cv).c_xs || x > (*cv).c_xe {
                    current_block_15 = 16668937799742929182;
                } else if y < (*(*display).d_forecv).c_ys && y < (*cv).c_ys {
                    current_block_15 = 16668937799742929182;
                } else if y > (*(*display).d_forecv).c_ye + 1 as libc::c_int
                    && y > (*cv).c_ye + 1 as libc::c_int
                {
                    current_block_15 = 16668937799742929182;
                } else {
                    if y < (*cv).c_ys {
                        m = (*cv).c_ys - y;
                    }
                    if y > (*cv).c_ye + 1 as libc::c_int {
                        m = y - ((*cv).c_ye + 1 as libc::c_int);
                    }
                    current_block_15 = 11050875288958768710;
                }
            } else {
                current_block_15 = 11050875288958768710;
            }
            match current_block_15 {
                16668937799742929182 => {}
                _ => {
                    if y >= (*(*display).d_forecv).c_ys
                        && y <= (*(*display).d_forecv).c_ye + 1 as libc::c_int
                    {
                        if y < (*cv).c_ys || y > (*cv).c_ye + 1 as libc::c_int {
                            current_block_15 = 16668937799742929182;
                        } else if x < (*(*display).d_forecv).c_xs && x < (*cv).c_xs {
                            current_block_15 = 16668937799742929182;
                        } else if x > (*(*display).d_forecv).c_xe && x > (*cv).c_xe {
                            current_block_15 = 16668937799742929182;
                        } else {
                            if x < (*cv).c_xs {
                                m = (*cv).c_xs - x;
                            }
                            if x > (*cv).c_xe {
                                m = x - (*cv).c_xe;
                            }
                            current_block_15 = 11584701595673473500;
                        }
                    } else {
                        current_block_15 = 11584701595673473500;
                    }
                    match current_block_15 {
                        16668937799742929182 => {}
                        _ => {
                            if m != 0 && (mm == 0 || m < mm) {
                                mcv = cv;
                                mm = m;
                            }
                        }
                    }
                }
            }
        }
        cv = (*cv).c_next;
    }
    return if !mcv.is_null() { mcv } else { (*display).d_forecv };
}
pub unsafe extern "C" fn SetCanvasWindow(mut cv: *mut canvas, mut wi: *mut win) {
    let mut p: *mut win = 0 as *mut win;
    let mut pp: *mut *mut win = 0 as *mut *mut win;
    let mut l: *mut layer = 0 as *mut layer;
    let mut cvp: *mut canvas = 0 as *mut canvas;
    let mut cvpp: *mut *mut canvas = 0 as *mut *mut canvas;
    l = (*cv).c_layer;
    display = (*cv).c_display;
    if !l.is_null() {
        cvpp = &mut (*l).l_cvlist;
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
        *cvpp = (*cvp).c_lnext;
        p = (*(*l).l_bottom).l_data as *mut win;
        l = (*cv).c_layer;
        (*cv).c_layer = 0 as *mut layer;
        if !p.is_null() && cv == (*display).d_forecv {
            ReleaseAutoWritelock(display, p);
            if (*p).w_silence != 0 {
                SetTimeout(
                    &mut (*p).w_silenceev,
                    (*p).w_silencewait * 1000 as libc::c_int,
                );
                evenq(&mut (*p).w_silenceev);
            }
            (*display).d_other = fore;
            (*display).d_fore = 0 as *mut win;
        }
        if ((*l).l_cvlist).is_null() && (p.is_null() || l != (*p).w_savelayer) {
            KillLayerChain(l);
        }
    }
    if !wi.is_null() && (*wi).w_type != 3 as libc::c_int {
        l = &mut (*wi).w_layer;
        if !((*wi).w_savelayer).is_null()
            && ((*wi).w_blocked != 0 || ((*(*wi).w_savelayer).l_cvlist).is_null())
        {
            l = (*wi).w_savelayer;
        }
    } else {
        l = &mut (*cv).c_blank;
        if !wi.is_null() {
            (*l).l_data = wi as *mut libc::c_char as *mut libc::c_void;
        } else {
            (*l).l_data = 0 as *mut libc::c_void;
        }
    }
    (*cv).c_lnext = (*l).l_cvlist;
    (*l).l_cvlist = cv;
    (*cv).c_layer = l;
    (*cv).c_xoff = (*cv).c_xs;
    (*cv).c_yoff = (*cv).c_ys;
    RethinkViewportOffsets(cv);
    if flayer.is_null() {
        flayer = l;
    }
    if !wi.is_null() && (*wi).w_type == 3 as libc::c_int {
        let mut d: *mut display = display;
        let mut oldflayer: *mut layer = flayer;
        flayer = l;
        display_windows(0 as libc::c_int, 0 as libc::c_int, wi);
        flayer = oldflayer;
        display = d;
    }
    if !wi.is_null() && (*display).d_other == wi {
        (*display).d_other = (*wi).w_next;
    }
    if cv == (*display).d_forecv {
        (*display).d_fore = wi;
        fore = (*display).d_fore;
        if !wi.is_null() {
            ObtainAutoWritelock(display, wi);
            if windows != wi {
                pp = &mut windows;
                loop {
                    p = *pp;
                    if p.is_null() {
                        break;
                    }
                    if p == wi {
                        break;
                    }
                    pp = &mut (*p).w_next;
                }
                *pp = (*p).w_next;
                (*p).w_next = windows;
                windows = p;
                WListLinkChanged();
            }
        }
    }
}
unsafe extern "C" fn cv_winid_fn(mut ev: *mut event, mut data: *mut libc::c_char) {
    let mut ox: libc::c_int = 0;
    let mut oy: libc::c_int = 0;
    let mut cv: *mut canvas = data as *mut canvas;
    display = (*cv).c_display;
    if (*display).d_status == 1 as libc::c_int {
        SetTimeout(ev, 1 as libc::c_int);
        evenq(ev);
        return;
    }
    ox = (*display).d_x;
    oy = (*display).d_y;
    if ((*cv).c_ye + 1 as libc::c_int) < (*display).d_height {
        RefreshLine(
            (*cv).c_ye + 1 as libc::c_int,
            0 as libc::c_int,
            (*display).d_width - 1 as libc::c_int,
            0 as libc::c_int,
        );
    }
    if ox != -(1 as libc::c_int) && oy != -(1 as libc::c_int) {
        GotoPos(ox, oy);
    }
}
pub unsafe extern "C" fn MakeDefaultCanvas() -> libc::c_int {
    let mut cv: *mut canvas = 0 as *mut canvas;
    cv = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<canvas>() as libc::c_ulong,
    ) as *mut canvas;
    if cv.is_null() {
        return -(1 as libc::c_int);
    }
    (*cv).c_xs = 0 as libc::c_int;
    (*cv).c_xe = (*display).d_width - 1 as libc::c_int;
    (*cv).c_ys = ((*display).d_has_hstatus == 4 as libc::c_int) as libc::c_int;
    (*cv)
        .c_ye = (*display).d_height - 1 as libc::c_int
        - ((*display).d_has_hstatus == 1 as libc::c_int) as libc::c_int - captionalways;
    (*cv).c_xoff = 0 as libc::c_int;
    (*cv).c_yoff = 0 as libc::c_int;
    (*cv).c_next = 0 as *mut canvas;
    (*cv).c_display = display;
    (*cv).c_vplist = 0 as *mut viewport;
    (*cv).c_slnext = 0 as *mut canvas;
    (*cv).c_slprev = 0 as *mut canvas;
    (*cv).c_slperp = 0 as *mut canvas;
    (*cv).c_slweight = 1 as libc::c_int;
    (*cv).c_slback = &mut (*display).d_canvas;
    (*display).d_canvas.c_slperp = cv;
    (*display).d_canvas.c_xs = (*cv).c_xs;
    (*display).d_canvas.c_xe = (*cv).c_xe;
    (*display).d_canvas.c_ys = (*cv).c_ys;
    (*display).d_canvas.c_ye = (*cv).c_ye;
    (*cv).c_slorient = 0 as libc::c_int;
    (*cv).c_captev.type_0 = 0 as libc::c_int;
    (*cv).c_captev.data = cv as *mut libc::c_char;
    (*cv)
        .c_captev
        .handler = ::std::mem::transmute::<
        Option::<unsafe extern "C" fn() -> ()>,
        Option::<unsafe extern "C" fn(*mut event, *mut libc::c_char) -> ()>,
    >(
        Some(
            ::std::mem::transmute::<
                unsafe extern "C" fn(*mut event, *mut libc::c_char) -> (),
                unsafe extern "C" fn() -> (),
            >(cv_winid_fn),
        ),
    );
    CanvasInitBlank(cv);
    (*cv).c_lnext = 0 as *mut canvas;
    (*display).d_cvlist = cv;
    RethinkDisplayViewports();
    (*display).d_forecv = cv;
    return 0 as libc::c_int;
}
unsafe extern "C" fn CreateCanvasChainRec(
    mut cv: *mut canvas,
    mut cvp: *mut *mut canvas,
) -> *mut *mut canvas {
    while !cv.is_null() {
        if !((*cv).c_slperp).is_null() {
            cvp = CreateCanvasChainRec((*cv).c_slperp, cvp);
        } else {
            *cvp = cv;
            cvp = &mut (*cv).c_next;
        }
        cv = (*cv).c_slnext;
    }
    return cvp;
}
pub unsafe extern "C" fn RecreateCanvasChain() {
    let mut cvp: *mut *mut canvas = 0 as *mut *mut canvas;
    cvp = CreateCanvasChainRec((*display).d_canvas.c_slperp, &mut (*display).d_cvlist);
    *cvp = 0 as *mut canvas;
}
pub unsafe extern "C" fn EqualizeCanvas(mut cv: *mut canvas, mut gflag: libc::c_int) {
    let mut cv2: *mut canvas = 0 as *mut canvas;
    while !cv.is_null() {
        if !((*cv).c_slperp).is_null() && gflag != 0 {
            (*cv).c_slweight = CountCanvasPerp(cv);
            cv2 = (*cv).c_slperp;
            while !cv2.is_null() {
                if !((*cv2).c_slperp).is_null() {
                    EqualizeCanvas((*cv2).c_slperp, gflag);
                }
                cv2 = (*cv2).c_slnext;
            }
        } else {
            (*cv).c_slweight = 1 as libc::c_int;
        }
        cv = (*cv).c_slnext;
    }
}
pub unsafe extern "C" fn ResizeCanvas(mut cv: *mut canvas) {
    let mut cv2: *mut canvas = 0 as *mut canvas;
    let mut cvn: *mut canvas = 0 as *mut canvas;
    let mut fcv: *mut canvas = 0 as *mut canvas;
    let mut nh: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut maxi: libc::c_int = 0;
    let mut hh: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    let mut w: libc::c_int = 0;
    let mut wsum: libc::c_int = 0;
    let mut need: libc::c_int = 0;
    let mut got: libc::c_int = 0;
    let mut xs: libc::c_int = 0;
    let mut ys: libc::c_int = 0;
    let mut xe: libc::c_int = 0;
    let mut ye: libc::c_int = 0;
    let mut focusmin: libc::c_int = 0 as libc::c_int;
    xs = (*cv).c_xs;
    ys = (*cv).c_ys;
    xe = (*cv).c_xe;
    ye = (*cv).c_ye;
    cv = (*cv).c_slperp;
    if cv.is_null() {
        return;
    }
    if (*cv).c_slorient == 0 as libc::c_int {
        (*cv).c_xs = xs;
        (*cv).c_xe = xe;
        (*cv).c_ys = ys;
        (*cv).c_ye = ye;
        (*cv).c_xoff = (*cv).c_xs;
        (*cv).c_yoff = (*cv).c_ys;
        (*cv).c_blank.l_width = (*cv).c_xe - (*cv).c_xs + 1 as libc::c_int;
        (*cv).c_blank.l_height = (*cv).c_ye - (*cv).c_ys + 1 as libc::c_int;
        return;
    }
    fcv = 0 as *mut canvas;
    if focusminwidth != 0 || focusminheight != 0 {
        cv2 = (*display).d_forecv;
        while !((*cv2).c_slback).is_null() {
            if (*cv2).c_slback == (*cv).c_slback {
                fcv = cv2;
                focusmin = if (*cv).c_slorient == (1 as libc::c_int) << 0 as libc::c_int
                {
                    focusminheight
                } else {
                    focusminwidth
                };
                if focusmin > 0 as libc::c_int {
                    focusmin -= 1;
                    focusmin;
                } else if focusmin < 0 as libc::c_int {
                    focusmin = if (*cv).c_slorient
                        == (1 as libc::c_int) << 0 as libc::c_int
                    {
                        ye - ys + 2 as libc::c_int
                    } else {
                        xe - xs + 2 as libc::c_int
                    };
                }
            }
            cv2 = (*cv2).c_slback;
        }
    }
    if focusmin != 0 {
        m = CountCanvas(cv) * 2 as libc::c_int;
        nh = if (*cv).c_slorient == (1 as libc::c_int) << 0 as libc::c_int {
            ye - ys + 2 as libc::c_int
        } else {
            xe - xs + 2 as libc::c_int
        };
        nh -= m;
        if nh < 0 as libc::c_int {
            nh = 0 as libc::c_int;
        }
        if focusmin > nh {
            focusmin = nh;
        }
    }
    cv2 = cv;
    wsum = 0 as libc::c_int;
    while !cv2.is_null() {
        wsum += (*cv2).c_slweight;
        cv2 = (*cv2).c_slnext;
    }
    if wsum == 0 as libc::c_int {
        wsum = 1 as libc::c_int;
    }
    w = wsum;
    nh = if (*cv).c_slorient == (1 as libc::c_int) << 0 as libc::c_int {
        ye - ys + 2 as libc::c_int
    } else {
        xe - xs + 2 as libc::c_int
    };
    cv2 = cv;
    got = 0 as libc::c_int;
    need = got;
    while !cv2.is_null() {
        m = if !((*cv2).c_slperp).is_null() {
            CountCanvasPerp(cv2) * 2 as libc::c_int - 1 as libc::c_int
        } else {
            1 as libc::c_int
        };
        if cv2 == fcv {
            m += focusmin;
        }
        hh = if (*cv2).c_slweight != 0 {
            nh * (*cv2).c_slweight / w
        } else {
            0 as libc::c_int
        };
        w -= (*cv2).c_slweight;
        nh -= hh;
        if hh <= m + 1 as libc::c_int {
            need += m + 1 as libc::c_int - hh;
        } else {
            got += hh - m - 1 as libc::c_int;
        }
        cv2 = (*cv2).c_slnext;
    }
    if need > got {
        need = got;
    }
    nh = if (*cv).c_slorient == (1 as libc::c_int) << 0 as libc::c_int {
        ye - ys + 2 as libc::c_int
    } else {
        xe - xs + 2 as libc::c_int
    };
    i = if (*cv).c_slorient == (1 as libc::c_int) << 0 as libc::c_int { ys } else { xs };
    maxi = if (*cv).c_slorient == (1 as libc::c_int) << 0 as libc::c_int {
        ye
    } else {
        xe
    };
    w = wsum;
    while !cv.is_null() {
        cvn = (*cv).c_slnext;
        if i > maxi {
            if !((*cv).c_slprev).is_null() && ((*(*cv).c_slback).c_slback).is_null()
                && ((*(*cv).c_slprev).c_slperp).is_null()
                && ((*(*cv).c_slprev).c_slprev).is_null()
            {
                (*(*cv).c_slprev).c_slorient = 0 as libc::c_int;
                if captionalways == 0 {
                    (*(*cv).c_slback).c_ye += 1;
                    (*(*cv).c_slback).c_ye;
                    (*(*cv).c_slprev).c_ye += 1;
                    (*(*cv).c_slprev).c_ye;
                }
            }
            SetCanvasWindow(cv, 0 as *mut win);
            FreeCanvas(cv);
        } else {
            m = if !((*cv).c_slperp).is_null() {
                CountCanvasPerp(cv) * 2 as libc::c_int - 1 as libc::c_int
            } else {
                1 as libc::c_int
            };
            if cv == fcv {
                m += focusmin;
            }
            hh = if (*cv).c_slweight != 0 {
                nh * (*cv).c_slweight / w
            } else {
                0 as libc::c_int
            };
            w -= (*cv).c_slweight;
            nh -= hh;
            if hh <= m + 1 as libc::c_int {
                hh = m + 1 as libc::c_int;
            } else {
                let mut hx: libc::c_int = 1 as libc::c_int;
                if got != 0 as libc::c_int {
                    hx = need * (hh - m - 1 as libc::c_int) / got;
                }
                got -= hh - m - 1 as libc::c_int;
                hh -= hx;
                need -= hx;
            }
            if i + hh > maxi + 2 as libc::c_int {
                hh = maxi + 2 as libc::c_int - i;
            }
            if i + hh == maxi + 1 as libc::c_int {
                hh += 1;
                hh;
            }
            if (*cv).c_slorient == (1 as libc::c_int) << 0 as libc::c_int {
                (*cv).c_xs = xs;
                (*cv).c_xe = xe;
                (*cv).c_ys = i;
                (*cv).c_ye = i + hh - 2 as libc::c_int;
                (*cv).c_xoff = xs;
                (*cv).c_yoff = i;
            } else {
                (*cv).c_xs = i;
                (*cv).c_xe = i + hh - 2 as libc::c_int;
                (*cv).c_ys = ys;
                (*cv).c_ye = ye;
                (*cv).c_xoff = i;
                (*cv).c_yoff = ys;
            }
            (*cv).c_xoff = (*cv).c_xs;
            (*cv).c_yoff = (*cv).c_ys;
            (*cv).c_blank.l_width = (*cv).c_xe - (*cv).c_xs + 1 as libc::c_int;
            (*cv).c_blank.l_height = (*cv).c_ye - (*cv).c_ys + 1 as libc::c_int;
            if !((*cv).c_slperp).is_null() {
                ResizeCanvas(cv);
                if ((*(*cv).c_slperp).c_slnext).is_null() {
                    FreePerp((*cv).c_slperp);
                    FreePerp(cv);
                }
            }
            i += hh;
        }
        cv = cvn;
    }
}
unsafe extern "C" fn AddPerp(mut cv: *mut canvas) -> *mut canvas {
    let mut pcv: *mut canvas = 0 as *mut canvas;
    pcv = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<canvas>() as libc::c_ulong,
    ) as *mut canvas;
    if pcv.is_null() {
        return 0 as *mut canvas;
    }
    (*pcv).c_next = 0 as *mut canvas;
    (*pcv).c_display = (*cv).c_display;
    (*pcv).c_slnext = (*cv).c_slnext;
    (*pcv).c_slprev = (*cv).c_slprev;
    (*pcv).c_slperp = cv;
    (*pcv).c_slback = (*cv).c_slback;
    if !((*cv).c_slback).is_null() && (*(*cv).c_slback).c_slperp == cv {
        (*(*cv).c_slback).c_slperp = pcv;
    }
    (*pcv).c_slorient = (*cv).c_slorient;
    (*pcv).c_xoff = 0 as libc::c_int;
    (*pcv).c_yoff = 0 as libc::c_int;
    (*pcv).c_xs = (*cv).c_xs;
    (*pcv).c_xe = (*cv).c_xe;
    (*pcv).c_ys = (*cv).c_ys;
    (*pcv).c_ye = (*cv).c_ye;
    if !((*pcv).c_slnext).is_null() {
        (*(*pcv).c_slnext).c_slprev = pcv;
    }
    if !((*pcv).c_slprev).is_null() {
        (*(*pcv).c_slprev).c_slnext = pcv;
    }
    (*pcv).c_slweight = (*cv).c_slweight;
    CanvasInitBlank(pcv);
    (*cv).c_slweight = 1 as libc::c_int;
    (*cv).c_slnext = 0 as *mut canvas;
    (*cv).c_slprev = 0 as *mut canvas;
    (*cv).c_slperp = 0 as *mut canvas;
    (*cv).c_slback = pcv;
    (*cv).c_slorient = 0 as libc::c_int;
    return pcv;
}
pub unsafe extern "C" fn AddCanvas(mut orient: libc::c_int) -> libc::c_int {
    let mut cv: *mut canvas = 0 as *mut canvas;
    let mut xs: libc::c_int = 0;
    let mut xe: libc::c_int = 0;
    let mut ys: libc::c_int = 0;
    let mut ye: libc::c_int = 0;
    let mut h: libc::c_int = 0;
    let mut num: libc::c_int = 0;
    cv = (*display).d_forecv;
    if (*cv).c_slorient != 0 as libc::c_int && (*cv).c_slorient != orient {
        if (AddPerp(cv)).is_null() {
            return -(1 as libc::c_int);
        }
    }
    cv = (*display).d_forecv;
    xs = (*(*cv).c_slback).c_xs;
    xe = (*(*cv).c_slback).c_xe;
    ys = (*(*cv).c_slback).c_ys;
    ye = (*(*cv).c_slback).c_ye;
    if captionalways == 0 && cv == (*display).d_canvas.c_slperp
        && ((*cv).c_slnext).is_null()
    {
        ye -= 1;
        ye;
    }
    num = CountCanvas((*(*cv).c_slback).c_slperp) + 1 as libc::c_int;
    if orient == (1 as libc::c_int) << 0 as libc::c_int {
        h = ye - ys + 1 as libc::c_int;
    } else {
        h = xe - xs + 1 as libc::c_int;
    }
    h -= 2 as libc::c_int * num - 1 as libc::c_int;
    if h < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    cv = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<canvas>() as libc::c_ulong,
    ) as *mut canvas;
    if cv.is_null() {
        return -(1 as libc::c_int);
    }
    (*(*(*display).d_forecv).c_slback).c_ye = ye;
    (*(*display).d_forecv).c_slorient = orient;
    (*cv).c_slnext = (*(*display).d_forecv).c_slnext;
    (*cv).c_slprev = (*display).d_forecv;
    (*(*display).d_forecv).c_slnext = cv;
    if !((*cv).c_slnext).is_null() {
        (*(*cv).c_slnext).c_slprev = cv;
    }
    (*cv).c_slorient = orient;
    (*cv).c_slback = (*(*display).d_forecv).c_slback;
    (*cv).c_xs = xs;
    (*cv).c_xe = xe;
    (*cv).c_ys = ys;
    (*cv).c_ye = ye;
    (*cv).c_xoff = 0 as libc::c_int;
    (*cv).c_yoff = 0 as libc::c_int;
    (*cv).c_display = display;
    (*cv).c_vplist = 0 as *mut viewport;
    (*cv).c_captev.type_0 = 0 as libc::c_int;
    (*cv).c_captev.data = cv as *mut libc::c_char;
    (*cv)
        .c_captev
        .handler = ::std::mem::transmute::<
        Option::<unsafe extern "C" fn() -> ()>,
        Option::<unsafe extern "C" fn(*mut event, *mut libc::c_char) -> ()>,
    >(
        Some(
            ::std::mem::transmute::<
                unsafe extern "C" fn(*mut event, *mut libc::c_char) -> (),
                unsafe extern "C" fn() -> (),
            >(cv_winid_fn),
        ),
    );
    CanvasInitBlank(cv);
    (*cv).c_lnext = 0 as *mut canvas;
    (*cv).c_next = 0 as *mut canvas;
    cv = (*cv).c_slback;
    EqualizeCanvas((*cv).c_slperp, 0 as libc::c_int);
    ResizeCanvas(cv);
    RecreateCanvasChain();
    RethinkDisplayViewports();
    ResizeLayersToCanvases();
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn RemCanvas() {
    let mut ye: libc::c_int = 0;
    let mut cv: *mut canvas = 0 as *mut canvas;
    cv = (*display).d_forecv;
    ye = (*(*cv).c_slback).c_ye;
    if (*cv).c_slorient == 0 as libc::c_int {
        return;
    }
    while !((*cv).c_slprev).is_null() {
        cv = (*cv).c_slprev;
    }
    if ((*cv).c_slnext).is_null() {
        return;
    }
    if ((*(*cv).c_slnext).c_slnext).is_null() && !((*(*cv).c_slback).c_slback).is_null()
    {
        cv = (*display).d_forecv;
        FreePerp(
            if !((*cv).c_slprev).is_null() { (*cv).c_slprev } else { (*cv).c_slnext },
        );
        FreePerp((*cv).c_slback);
    }
    cv = (*display).d_forecv;
    (*display).d_forecv = (*cv).c_slprev;
    if ((*display).d_forecv).is_null() {
        (*display).d_forecv = (*cv).c_slnext;
    }
    FreeCanvas(cv);
    cv = (*display).d_forecv;
    while !((*(*display).d_forecv).c_slperp).is_null() {
        (*display).d_forecv = (*(*display).d_forecv).c_slperp;
    }
    if ((*cv).c_slnext).is_null() && ((*cv).c_slprev).is_null()
        && ((*(*cv).c_slback).c_slback).is_null() && ((*cv).c_slperp).is_null()
    {
        (*cv).c_slorient = 0 as libc::c_int;
        if captionalways == 0 {
            ye += 1;
            (*(*cv).c_slback).c_ye = ye;
        }
    }
    cv = (*cv).c_slback;
    EqualizeCanvas((*cv).c_slperp, 0 as libc::c_int);
    ResizeCanvas(cv);
    (*display).d_fore = (*(*(*(*display).d_forecv).c_layer).l_bottom).l_data as *mut win;
    flayer = (*(*display).d_forecv).c_layer;
    RecreateCanvasChain();
    RethinkDisplayViewports();
    ResizeLayersToCanvases();
}
pub unsafe extern "C" fn OneCanvas() {
    let mut cv: *mut canvas = (*display).d_forecv;
    let mut ocv: *mut canvas = 0 as *mut canvas;
    if !((*cv).c_slprev).is_null() {
        ocv = (*cv).c_slprev;
        (*(*cv).c_slprev).c_slnext = (*cv).c_slnext;
    }
    if !((*cv).c_slnext).is_null() {
        ocv = (*cv).c_slnext;
        (*(*cv).c_slnext).c_slprev = (*cv).c_slprev;
    }
    if ocv.is_null() {
        return;
    }
    if !((*cv).c_slback).is_null() && (*(*cv).c_slback).c_slperp == cv {
        (*(*cv).c_slback).c_slperp = ocv;
    }
    (*cv).c_slorient = 0 as libc::c_int;
    while !((*display).d_canvas.c_slperp).is_null() {
        FreeCanvas((*display).d_canvas.c_slperp);
    }
    cv = (*display).d_forecv;
    (*display).d_canvas.c_slperp = cv;
    (*cv).c_slback = &mut (*display).d_canvas;
    (*cv).c_slnext = 0 as *mut canvas;
    (*cv).c_slprev = 0 as *mut canvas;
    if captionalways == 0 {
        (*display).d_canvas.c_ye += 1;
        (*display).d_canvas.c_ye;
    }
    ResizeCanvas(&mut (*display).d_canvas);
    RecreateCanvasChain();
    RethinkDisplayViewports();
    ResizeLayersToCanvases();
}
pub unsafe extern "C" fn DupLayoutCv(
    mut cvf: *mut canvas,
    mut cvt: *mut canvas,
    mut save: libc::c_int,
) {
    while !cvf.is_null() {
        (*cvt).c_slorient = (*cvf).c_slorient;
        (*cvt).c_slweight = (*cvf).c_slweight;
        if cvf == (*display).d_forecv {
            (*display).d_forecv = cvt;
        }
        if save == 0 {
            (*cvt).c_display = display;
            if ((*cvf).c_slperp).is_null() {
                (*cvt).c_captev.type_0 = 0 as libc::c_int;
                (*cvt).c_captev.data = cvt as *mut libc::c_char;
                (*cvt)
                    .c_captev
                    .handler = ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn() -> ()>,
                    Option::<unsafe extern "C" fn(*mut event, *mut libc::c_char) -> ()>,
                >(
                    Some(
                        ::std::mem::transmute::<
                            unsafe extern "C" fn(*mut event, *mut libc::c_char) -> (),
                            unsafe extern "C" fn() -> (),
                        >(cv_winid_fn),
                    ),
                );
                (*cvt).c_blank.l_cvlist = 0 as *mut canvas;
                (*cvt).c_blank.l_layfn = &mut BlankLf;
                (*cvt).c_blank.l_bottom = &mut (*cvt).c_blank;
            }
            (*cvt).c_layer = (*cvf).c_layer;
        } else {
            let mut p: *mut win = if !((*cvf).c_layer).is_null() {
                (*(*(*cvf).c_layer).l_bottom).l_data as *mut win
            } else {
                0 as *mut win
            };
            (*cvt)
                .c_layer = if !p.is_null() {
                &mut (*p).w_layer
            } else {
                0 as *mut layer
            };
        }
        if !((*cvf).c_slperp).is_null() {
            (*cvt)
                .c_slperp = calloc(
                1 as libc::c_int as libc::c_ulong,
                ::std::mem::size_of::<canvas>() as libc::c_ulong,
            ) as *mut canvas;
            (*(*cvt).c_slperp).c_slback = cvt;
            CanvasInitBlank((*cvt).c_slperp);
            DupLayoutCv((*cvf).c_slperp, (*cvt).c_slperp, save);
        }
        if !((*cvf).c_slnext).is_null() {
            (*cvt)
                .c_slnext = calloc(
                1 as libc::c_int as libc::c_ulong,
                ::std::mem::size_of::<canvas>() as libc::c_ulong,
            ) as *mut canvas;
            (*(*cvt).c_slnext).c_slprev = cvt;
            (*(*cvt).c_slnext).c_slback = (*cvt).c_slback;
            CanvasInitBlank((*cvt).c_slnext);
        }
        cvf = (*cvf).c_slnext;
        cvt = (*cvt).c_slnext;
    }
}
pub unsafe extern "C" fn PutWindowCv(mut cv: *mut canvas) {
    let mut p: *mut win = 0 as *mut win;
    while !cv.is_null() {
        if !((*cv).c_slperp).is_null() {
            PutWindowCv((*cv).c_slperp);
        } else {
            p = if !((*cv).c_layer).is_null() {
                (*(*cv).c_layer).l_data as *mut win
            } else {
                0 as *mut win
            };
            (*cv).c_layer = 0 as *mut layer;
            SetCanvasWindow(cv, p);
        }
        cv = (*cv).c_slnext;
    }
}
