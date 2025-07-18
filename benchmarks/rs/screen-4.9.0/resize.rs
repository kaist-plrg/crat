use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type logfile;
    fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, _: ...) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn bcopy(__src: *const libc::c_void, __dest: *mut libc::c_void, __n: size_t);
    fn bzero(_: *mut libc::c_void, _: libc::c_ulong);
    static mut strnomem: [libc::c_char; 0];
    fn ResizeCanvas(_: *mut canvas);
    fn RecreateCanvasChain();
    fn RethinkViewportOffsets(_: *mut canvas);
    fn RethinkDisplayViewports() -> libc::c_int;
    fn Msg(_: libc::c_int, _: *const libc::c_char, _: ...);
    fn Panic(_: libc::c_int, _: *const libc::c_char, _: ...) -> !;
    fn KillWindow(_: *mut win);
    fn RefreshArea(
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
    );
    fn Redisplay(_: libc::c_int);
    fn ResetIdle();
    fn KillBlanker();
    fn ExitOverlayPage();
    static mut flayer: *mut layer;
    static mut display: *mut display;
    static mut displays: *mut display;
    static mut blank: *mut libc::c_uchar;
    static mut null: *mut libc::c_uchar;
    static mut mline_blank: mline;
    static mut mline_null: mline;
    static mut mline_old: mline;
    static mut windows: *mut win;
    static Z0width: libc::c_int;
    static Z1width: libc::c_int;
    static mut captionalways: libc::c_int;
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
pub static mut glwz: winsize = winsize {
    ws_row: 0,
    ws_col: 0,
    ws_xpixel: 0,
    ws_ypixel: 0,
};
static mut mline_zero: mline = {
    let mut init = mline {
        image: 0 as *const libc::c_uchar as *mut libc::c_uchar,
        attr: 0 as *const libc::c_uchar as *mut libc::c_uchar,
        font: 0 as *const libc::c_uchar as *mut libc::c_uchar,
        fontx: 0 as *const libc::c_uchar as *mut libc::c_uchar,
        color: 0 as *const libc::c_uchar as *mut libc::c_uchar,
    };
    init
};
pub unsafe extern "C" fn CheckScreenSize(mut change_flag: libc::c_int) {
    let mut wi: libc::c_int = 0;
    let mut he: libc::c_int = 0;
    if display.is_null() {
        return;
    }
    if ioctl(
        (*display).d_userfd,
        0x5413 as libc::c_int as libc::c_ulong,
        &mut glwz as *mut winsize as *mut libc::c_char,
    ) != 0 as libc::c_int
    {
        wi = (*display).d_tcs[1 as libc::c_int as usize].num;
        he = (*display).d_tcs[0 as libc::c_int as usize].num;
    } else {
        wi = glwz.ws_col as libc::c_int;
        he = glwz.ws_row as libc::c_int;
        if wi == 0 as libc::c_int {
            wi = (*display).d_tcs[1 as libc::c_int as usize].num;
        }
        if he == 0 as libc::c_int {
            he = (*display).d_tcs[0 as libc::c_int as usize].num;
        }
    }
    if (*display).d_width == wi && (*display).d_height == he {
        return;
    }
    KillBlanker();
    ResetIdle();
    ChangeScreenSize(wi, he, change_flag);
}
pub unsafe extern "C" fn ChangeScreenSize(
    mut wi: libc::c_int,
    mut he: libc::c_int,
    mut change_fore: libc::c_int,
) {
    let mut p: *mut win = 0 as *mut win;
    let mut cv: *mut canvas = 0 as *mut canvas;
    let mut wwi: libc::c_int = 0;
    cv = &mut (*display).d_canvas;
    (*cv).c_xe = wi - 1 as libc::c_int;
    (*cv).c_ys = ((*display).d_has_hstatus == 4 as libc::c_int) as libc::c_int;
    (*cv)
        .c_ye = he - 1 as libc::c_int
        - (!((*cv).c_slperp).is_null() && !((*(*cv).c_slperp).c_slnext).is_null()
            || captionalways != 0) as libc::c_int
        - ((*display).d_has_hstatus == 1 as libc::c_int) as libc::c_int;
    (*cv).c_blank.l_height = (*cv).c_ye - (*cv).c_ys + 1 as libc::c_int;
    if !((*cv).c_slperp).is_null() {
        ResizeCanvas(cv);
        RecreateCanvasChain();
        RethinkDisplayViewports();
    }
    if ((*display).d_forecv).is_null() {
        (*display).d_forecv = (*display).d_cvlist;
    }
    if !((*display).d_forecv).is_null() {
        (*display)
            .d_fore = (*(*(*(*display).d_forecv).c_layer).l_bottom).l_data as *mut win;
    }
    (*display).d_width = wi;
    (*display).d_height = he;
    CheckMaxSize(wi);
    if !((*display).d_tcs[44 as libc::c_int as usize].str_0).is_null() {
        (*display).d_defwidth = (*display).d_tcs[1 as libc::c_int as usize].num;
        (*display).d_defheight = (*display).d_tcs[0 as libc::c_int as usize].num;
    } else {
        if !((*display).d_tcs[45 as libc::c_int as usize].str_0).is_null()
            && (wi == Z0width || wi == Z1width)
            && ((*display).d_tcs[1 as libc::c_int as usize].num == Z0width
                || (*display).d_tcs[1 as libc::c_int as usize].num == Z1width)
        {
            (*display).d_defwidth = (*display).d_tcs[1 as libc::c_int as usize].num;
        } else {
            (*display).d_defwidth = wi;
        }
        (*display).d_defheight = he;
    }
    if change_fore != 0 {
        ResizeLayersToCanvases();
    }
    if change_fore == 2 as libc::c_int
        && ((*display).d_tcs[44 as libc::c_int as usize].str_0).is_null()
        && ((*displays).d_next).is_null()
    {
        p = windows;
        while !p.is_null() {
            wwi = wi;
            if !((*p).w_savelayer).is_null() && ((*(*p).w_savelayer).l_cvlist).is_null()
            {
                ResizeLayer((*p).w_savelayer, wwi, he, 0 as *mut display);
            }
            p = (*p).w_next;
        }
    }
}
pub unsafe extern "C" fn ResizeLayersToCanvases() {
    let mut cv: *mut canvas = 0 as *mut canvas;
    let mut l: *mut layer = 0 as *mut layer;
    let mut lx: libc::c_int = 0;
    let mut ly: libc::c_int = 0;
    (*display).d_kaablamm = 0 as libc::c_int;
    cv = (*display).d_cvlist;
    while !cv.is_null() {
        l = (*cv).c_layer;
        if !l.is_null() {
            if !((*l).l_width == (*cv).c_xe - (*cv).c_xs + 1 as libc::c_int
                && (*l).l_height == (*cv).c_ye - (*cv).c_ys + 1 as libc::c_int)
            {
                if !(MayResizeLayer(l) == 0) {
                    ResizeLayer(
                        l,
                        (*cv).c_xe - (*cv).c_xs + 1 as libc::c_int,
                        (*cv).c_ye - (*cv).c_ys + 1 as libc::c_int,
                        display,
                    );
                }
                lx = (*(*cv).c_layer).l_x;
                ly = (*(*cv).c_layer).l_y;
                if ly + (*cv).c_yoff < (*cv).c_ys {
                    (*cv).c_yoff = (*cv).c_ys - ly;
                    RethinkViewportOffsets(cv);
                } else if ly + (*cv).c_yoff > (*cv).c_ye {
                    (*cv).c_yoff = (*cv).c_ye - ly;
                    RethinkViewportOffsets(cv);
                }
                if lx + (*cv).c_xoff < (*cv).c_xs {
                    let mut n: libc::c_int = (*cv).c_xs - (lx + (*cv).c_xoff);
                    if n
                        < ((*cv).c_xe - (*cv).c_xs + 1 as libc::c_int) / 2 as libc::c_int
                    {
                        n = ((*cv).c_xe - (*cv).c_xs + 1 as libc::c_int)
                            / 2 as libc::c_int;
                    }
                    if (*cv).c_xoff + n > (*cv).c_xs {
                        n = (*cv).c_xs - (*cv).c_xoff;
                    }
                    (*cv).c_xoff += n;
                    RethinkViewportOffsets(cv);
                } else if lx + (*cv).c_xoff > (*cv).c_xe {
                    let mut n_0: libc::c_int = lx + (*cv).c_xoff - (*cv).c_xe;
                    if n_0
                        < ((*cv).c_xe - (*cv).c_xs + 1 as libc::c_int) / 2 as libc::c_int
                    {
                        n_0 = ((*cv).c_xe - (*cv).c_xs + 1 as libc::c_int)
                            / 2 as libc::c_int;
                    }
                    if ((*cv).c_xoff - n_0 + (*(*cv).c_layer).l_width - 1 as libc::c_int)
                        < (*cv).c_xe
                    {
                        n_0 = (*cv).c_xoff + (*(*cv).c_layer).l_width - 1 as libc::c_int
                            - (*cv).c_xe;
                    }
                    (*cv).c_xoff -= n_0;
                    RethinkViewportOffsets(cv);
                }
            }
        }
        cv = (*cv).c_next;
    }
    Redisplay(0 as libc::c_int);
    if (*display).d_kaablamm != 0 {
        kaablamm();
        (*display).d_kaablamm = 0 as libc::c_int;
    }
}
pub unsafe extern "C" fn MayResizeLayer(mut l: *mut layer) -> libc::c_int {
    let mut cvs: libc::c_int = 0 as libc::c_int;
    while !l.is_null() {
        if !((*l).l_cvlist).is_null() {
            cvs += 1;
            if cvs > 1 as libc::c_int || !((*(*l).l_cvlist).c_lnext).is_null() {
                return 0 as libc::c_int;
            }
        }
        l = (*l).l_next;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn kaablamm() {
    Msg(
        0 as libc::c_int,
        b"Aborted because of window size change.\0" as *const u8 as *const libc::c_char,
    );
}
pub unsafe extern "C" fn ResizeLayer(
    mut l: *mut layer,
    mut wi: libc::c_int,
    mut he: libc::c_int,
    mut norefdisp: *mut display,
) {
    let mut p: *mut win = 0 as *mut win;
    let mut cv: *mut canvas = 0 as *mut canvas;
    let mut oldflayer: *mut layer = flayer;
    let mut d: *mut display = 0 as *mut display;
    let mut olddisplay: *mut display = display;
    if (*l).l_width == wi && (*l).l_height == he {
        return;
    }
    p = (*(*l).l_bottom).l_data as *mut win;
    if !oldflayer.is_null()
        && (l == oldflayer || (*(*oldflayer).l_bottom).l_data as *mut win == p)
    {
        oldflayer = 0 as *mut layer;
    }
    flayer = l;
    if !p.is_null() {
        d = displays;
        while !d.is_null() {
            cv = (*d).d_cvlist;
            while !cv.is_null() {
                if p == (*(*(*cv).c_layer).l_bottom).l_data as *mut win {
                    let mut _last: *mut layer = 0 as *mut layer;
                    flayer = (*cv).c_layer;
                    while !((*flayer).l_next).is_null() {
                        if (Some(((*(*flayer).l_layfn).lf_LayResize).unwrap()))
                            .unwrap()(wi, he) == 0 as libc::c_int
                        {
                            _last = flayer;
                            flayer = (*flayer).l_next;
                        } else {
                            let mut _cv: *mut canvas = 0 as *mut canvas;
                            _cv = (*flayer).l_cvlist;
                            while !_cv.is_null() {
                                (*(*_cv).c_display).d_kaablamm = 1 as libc::c_int;
                                _cv = (*_cv).c_lnext;
                            }
                            ExitOverlayPage();
                            if !_last.is_null() {
                                (*_last).l_next = flayer;
                            }
                        }
                    }
                    (Some(((*(*flayer).l_layfn).lf_LayResize).unwrap()))
                        .unwrap()(wi, he);
                }
                cv = (*cv).c_next;
            }
            d = (*d).d_next;
        }
    } else {
        let mut _last_0: *mut layer = 0 as *mut layer;
        flayer = flayer;
        while !((*flayer).l_next).is_null() {
            if (Some(((*(*flayer).l_layfn).lf_LayResize).unwrap())).unwrap()(wi, he)
                == 0 as libc::c_int
            {
                _last_0 = flayer;
                flayer = (*flayer).l_next;
            } else {
                let mut _cv_0: *mut canvas = 0 as *mut canvas;
                _cv_0 = (*flayer).l_cvlist;
                while !_cv_0.is_null() {
                    (*(*_cv_0).c_display).d_kaablamm = 1 as libc::c_int;
                    _cv_0 = (*_cv_0).c_lnext;
                }
                ExitOverlayPage();
                if !_last_0.is_null() {
                    (*_last_0).l_next = flayer;
                }
            }
        }
        (Some(((*(*flayer).l_layfn).lf_LayResize).unwrap())).unwrap()(wi, he);
    }
    display = displays;
    while !display.is_null() {
        if !(display == norefdisp) {
            cv = (*display).d_cvlist;
            while !cv.is_null() {
                if (*(*(*cv).c_layer).l_bottom).l_data as *mut win == p {
                    let mut olddisplay_0: *mut display = display;
                    let mut oldflayer_0: *mut layer = flayer;
                    let mut l_0: *mut layer = (*cv).c_layer;
                    let mut cvlist: *mut canvas = (*l_0).l_cvlist;
                    let mut cvlnext: *mut canvas = (*cv).c_lnext;
                    flayer = l_0;
                    (*l_0).l_cvlist = cv;
                    (*cv).c_lnext = 0 as *mut canvas;
                    (Some(((*(*flayer).l_layfn).lf_LayRedisplayLine).unwrap()))
                        .unwrap()(
                        -(1 as libc::c_int),
                        -(1 as libc::c_int),
                        -(1 as libc::c_int),
                        0 as libc::c_int,
                    );
                    flayer = oldflayer_0;
                    (*l_0).l_cvlist = cvlist;
                    (*cv).c_lnext = cvlnext;
                    display = olddisplay_0;
                    RefreshArea(
                        (*cv).c_xs,
                        (*cv).c_ys,
                        (*cv).c_xe,
                        (*cv).c_ye,
                        0 as libc::c_int,
                    );
                }
                cv = (*cv).c_next;
            }
            if (*display).d_kaablamm != 0 {
                kaablamm();
                (*display).d_kaablamm = 0 as libc::c_int;
            }
        }
        display = (*display).d_next;
    }
    if !oldflayer.is_null() {
        flayer = oldflayer;
    }
    display = olddisplay;
}
unsafe extern "C" fn FreeMline(mut ml: *mut mline) {
    if !((*ml).image).is_null() {
        free((*ml).image as *mut libc::c_void);
    }
    if !((*ml).attr).is_null() && (*ml).attr != null {
        free((*ml).attr as *mut libc::c_void);
    }
    if !((*ml).font).is_null() && (*ml).font != null {
        free((*ml).font as *mut libc::c_void);
    }
    if !((*ml).fontx).is_null() && (*ml).fontx != null {
        free((*ml).fontx as *mut libc::c_void);
    }
    if !((*ml).color).is_null() && (*ml).color != null {
        free((*ml).color as *mut libc::c_void);
    }
    *ml = mline_zero;
}
unsafe extern "C" fn AllocMline(mut ml: *mut mline, mut w: libc::c_int) -> libc::c_int {
    (*ml).image = malloc(w as libc::c_ulong) as *mut libc::c_uchar;
    (*ml).attr = null;
    (*ml).font = null;
    (*ml).fontx = null;
    (*ml).color = null;
    if ((*ml).image).is_null() {
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn BcopyMline(
    mut mlf: *mut mline,
    mut xf: libc::c_int,
    mut mlt: *mut mline,
    mut xt: libc::c_int,
    mut l: libc::c_int,
    mut w: libc::c_int,
) -> libc::c_int {
    let mut r: libc::c_int = 0 as libc::c_int;
    bcopy(
        ((*mlf).image as *mut libc::c_char).offset(xf as isize) as *const libc::c_void,
        ((*mlt).image as *mut libc::c_char).offset(xt as isize) as *mut libc::c_void,
        l as size_t,
    );
    if (*mlf).attr != null && (*mlt).attr == null {
        (*mlt)
            .attr = calloc(w as libc::c_ulong, 1 as libc::c_int as libc::c_ulong)
            as *mut libc::c_uchar;
        if ((*mlt).attr).is_null() {
            (*mlt).attr = null;
            r = -(1 as libc::c_int);
        }
    }
    if (*mlt).attr != null {
        bcopy(
            ((*mlf).attr as *mut libc::c_char).offset(xf as isize)
                as *const libc::c_void,
            ((*mlt).attr as *mut libc::c_char).offset(xt as isize) as *mut libc::c_void,
            l as size_t,
        );
    }
    if (*mlf).font != null && (*mlt).font == null {
        (*mlt)
            .font = calloc(w as libc::c_ulong, 1 as libc::c_int as libc::c_ulong)
            as *mut libc::c_uchar;
        if ((*mlt).font).is_null() {
            (*mlt).font = null;
            r = -(1 as libc::c_int);
        }
    }
    if (*mlt).font != null {
        bcopy(
            ((*mlf).font as *mut libc::c_char).offset(xf as isize)
                as *const libc::c_void,
            ((*mlt).font as *mut libc::c_char).offset(xt as isize) as *mut libc::c_void,
            l as size_t,
        );
    }
    if (*mlf).fontx != null && (*mlt).fontx == null {
        (*mlt)
            .fontx = calloc(w as libc::c_ulong, 1 as libc::c_int as libc::c_ulong)
            as *mut libc::c_uchar;
        if ((*mlt).fontx).is_null() {
            (*mlt).fontx = null;
            r = -(1 as libc::c_int);
        }
    }
    if (*mlt).fontx != null {
        bcopy(
            ((*mlf).fontx as *mut libc::c_char).offset(xf as isize)
                as *const libc::c_void,
            ((*mlt).fontx as *mut libc::c_char).offset(xt as isize) as *mut libc::c_void,
            l as size_t,
        );
    }
    if (*mlf).color != null && (*mlt).color == null {
        (*mlt)
            .color = calloc(w as libc::c_ulong, 1 as libc::c_int as libc::c_ulong)
            as *mut libc::c_uchar;
        if ((*mlt).color).is_null() {
            (*mlt).color = null;
            r = -(1 as libc::c_int);
        }
    }
    if (*mlt).color != null {
        bcopy(
            ((*mlf).color as *mut libc::c_char).offset(xf as isize)
                as *const libc::c_void,
            ((*mlt).color as *mut libc::c_char).offset(xt as isize) as *mut libc::c_void,
            l as size_t,
        );
    }
    return r;
}
static mut maxwidth: libc::c_int = 0;
unsafe extern "C" fn CheckMaxSize(mut wi: libc::c_int) {
    let mut oldnull: *mut libc::c_uchar = null;
    let mut oldblank: *mut libc::c_uchar = blank;
    let mut p: *mut win = 0 as *mut win;
    let mut i: libc::c_int = 0;
    let mut ml: *mut mline = 0 as *mut mline;
    if wi > 1000 as libc::c_int {
        wi = 1000 as libc::c_int;
    }
    if wi <= maxwidth {
        return;
    }
    maxwidth = wi + 1 as libc::c_int;
    blank = xrealloc(blank as *mut libc::c_char, maxwidth) as *mut libc::c_uchar;
    null = xrealloc(null as *mut libc::c_char, maxwidth) as *mut libc::c_uchar;
    mline_old
        .image = xrealloc(mline_old.image as *mut libc::c_char, maxwidth)
        as *mut libc::c_uchar;
    mline_old
        .attr = xrealloc(mline_old.attr as *mut libc::c_char, maxwidth)
        as *mut libc::c_uchar;
    mline_old
        .font = xrealloc(mline_old.font as *mut libc::c_char, maxwidth)
        as *mut libc::c_uchar;
    mline_old
        .fontx = xrealloc(mline_old.fontx as *mut libc::c_char, maxwidth)
        as *mut libc::c_uchar;
    mline_old
        .color = xrealloc(mline_old.color as *mut libc::c_char, maxwidth)
        as *mut libc::c_uchar;
    if !(!blank.is_null() && !null.is_null() && !(mline_old.image).is_null()
        && !(mline_old.attr).is_null() && !(mline_old.font).is_null()
        && !(mline_old.fontx).is_null() && !(mline_old.color).is_null())
    {
        Panic(
            0 as libc::c_int,
            b"%s\0" as *const u8 as *const libc::c_char,
            strnomem.as_mut_ptr(),
        );
    }
    MakeBlankLine(blank, maxwidth);
    bzero(null as *mut libc::c_char as *mut libc::c_void, maxwidth as libc::c_ulong);
    mline_blank.image = blank;
    mline_blank.attr = null;
    mline_null.image = null;
    mline_null.attr = null;
    mline_blank.font = null;
    mline_null.font = null;
    mline_blank.fontx = null;
    mline_null.fontx = null;
    mline_blank.color = null;
    mline_null.color = null;
    p = windows;
    while !p.is_null() {
        ml = (*p).w_mlines;
        i = 0 as libc::c_int;
        while i < (*p).w_layer.l_height {
            if (*ml).image == oldblank {
                (*ml).image = blank;
            }
            if (*ml).attr == oldnull {
                (*ml).attr = null;
            }
            if (*ml).font == oldnull {
                (*ml).font = null;
            }
            if (*ml).fontx == oldnull {
                (*ml).fontx = null;
            }
            if (*ml).color == oldnull {
                (*ml).color = null;
            }
            i += 1;
            i;
            ml = ml.offset(1);
            ml;
        }
        ml = (*p).w_hlines;
        i = 0 as libc::c_int;
        while i < (*p).w_histheight {
            if (*ml).image == oldblank {
                (*ml).image = blank;
            }
            if (*ml).attr == oldnull {
                (*ml).attr = null;
            }
            if (*ml).font == oldnull {
                (*ml).font = null;
            }
            if (*ml).fontx == oldnull {
                (*ml).fontx = null;
            }
            if (*ml).color == oldnull {
                (*ml).color = null;
            }
            i += 1;
            i;
            ml = ml.offset(1);
            ml;
        }
        ml = (*p).w_alt.hlines;
        i = 0 as libc::c_int;
        while i < (*p).w_alt.histheight {
            if (*ml).image == oldblank {
                (*ml).image = blank;
            }
            if (*ml).attr == oldnull {
                (*ml).attr = null;
            }
            if (*ml).font == oldnull {
                (*ml).font = null;
            }
            if (*ml).fontx == oldnull {
                (*ml).fontx = null;
            }
            if (*ml).color == oldnull {
                (*ml).color = null;
            }
            i += 1;
            i;
            ml = ml.offset(1);
            ml;
        }
        ml = (*p).w_alt.mlines;
        i = 0 as libc::c_int;
        while i < (*p).w_alt.height {
            if (*ml).image == oldblank {
                (*ml).image = blank;
            }
            if (*ml).attr == oldnull {
                (*ml).attr = null;
            }
            if (*ml).font == oldnull {
                (*ml).font = null;
            }
            if (*ml).fontx == oldnull {
                (*ml).fontx = null;
            }
            if (*ml).color == oldnull {
                (*ml).color = null;
            }
            i += 1;
            i;
            ml = ml.offset(1);
            ml;
        }
        p = (*p).w_next;
    }
}
pub unsafe extern "C" fn xrealloc(
    mut mem: *mut libc::c_char,
    mut len: libc::c_int,
) -> *mut libc::c_char {
    let mut nmem: *mut libc::c_char = 0 as *mut libc::c_char;
    if mem.is_null() {
        return malloc(len as libc::c_ulong) as *mut libc::c_char;
    }
    nmem = realloc(mem as *mut libc::c_void, len as libc::c_ulong) as *mut libc::c_char;
    if !nmem.is_null() {
        return nmem;
    }
    free(mem as *mut libc::c_void);
    return 0 as *mut libc::c_char;
}
unsafe extern "C" fn MakeBlankLine(mut p: *mut libc::c_uchar, mut n: libc::c_int) {
    loop {
        let fresh0 = n;
        n = n - 1;
        if !(fresh0 != 0) {
            break;
        }
        let fresh1 = p;
        p = p.offset(1);
        *fresh1 = ' ' as i32 as libc::c_uchar;
    };
}
pub unsafe extern "C" fn ChangeWindowSize(
    mut p: *mut win,
    mut wi: libc::c_int,
    mut he: libc::c_int,
    mut hi: libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut mlf: *mut mline = 0 as *mut mline;
    let mut mlt: *mut mline = 0 as *mut mline;
    let mut ml: *mut mline = 0 as *mut mline;
    let mut nmlines: *mut mline = 0 as *mut mline;
    let mut nhlines: *mut mline = 0 as *mut mline;
    let mut fy: libc::c_int = 0;
    let mut ty: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut lx: libc::c_int = 0;
    let mut lf: libc::c_int = 0;
    let mut lt: libc::c_int = 0;
    let mut yy: libc::c_int = 0;
    let mut oty: libc::c_int = 0;
    let mut addone: libc::c_int = 0;
    let mut ncx: libc::c_int = 0;
    let mut ncy: libc::c_int = 0;
    let mut naka: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut shift: libc::c_int = 0;
    if wi <= 0 as libc::c_int || he <= 0 as libc::c_int {
        hi = 0 as libc::c_int;
        he = hi;
        wi = he;
    }
    if (*p).w_type == 3 as libc::c_int {
        return 0 as libc::c_int;
    }
    if wi > 1000 as libc::c_int {
        Msg(
            0 as libc::c_int,
            b"Window width too large. Truncated to %d.\0" as *const u8
                as *const libc::c_char,
            1000 as libc::c_int,
        );
        wi = 1000 as libc::c_int;
    }
    if he > 1000 as libc::c_int {
        Msg(
            0 as libc::c_int,
            b"Window height too large. Truncated to %d.\0" as *const u8
                as *const libc::c_char,
            1000 as libc::c_int,
        );
        he = 1000 as libc::c_int;
    }
    if (*p).w_layer.l_width == wi && (*p).w_layer.l_height == he
        && (*p).w_histheight == hi
    {
        return 0 as libc::c_int;
    }
    CheckMaxSize(wi);
    fy = (*p).w_histheight + (*p).w_layer.l_height - 1 as libc::c_int;
    ty = hi + he - 1 as libc::c_int;
    nhlines = 0 as *mut mline;
    nmlines = nhlines;
    ncx = 0 as libc::c_int;
    ncy = 0 as libc::c_int;
    naka = 0 as libc::c_int;
    if wi != 0 {
        if wi != (*p).w_layer.l_width || he != (*p).w_layer.l_height {
            nmlines = calloc(
                he as libc::c_ulong,
                ::std::mem::size_of::<mline>() as libc::c_ulong,
            ) as *mut mline;
            if nmlines.is_null() {
                KillWindow(p);
                Msg(
                    0 as libc::c_int,
                    b"%s\0" as *const u8 as *const libc::c_char,
                    strnomem.as_mut_ptr(),
                );
                return -(1 as libc::c_int);
            }
        } else {
            nmlines = (*p).w_mlines;
            fy -= he;
            ty -= he;
            ncx = (*p).w_layer.l_x;
            ncy = (*p).w_layer.l_y;
            naka = (*p).w_autoaka;
        }
    }
    if hi != 0 {
        nhlines = calloc(
            hi as libc::c_ulong,
            ::std::mem::size_of::<mline>() as libc::c_ulong,
        ) as *mut mline;
        if nhlines.is_null() {
            Msg(
                0 as libc::c_int,
                b"No memory for history buffer - turned off\0" as *const u8
                    as *const libc::c_char,
            );
            hi = 0 as libc::c_int;
            ty = he - 1 as libc::c_int;
        }
    }
    addone = 0 as libc::c_int;
    if (*p).w_layer.l_width != 0 && (*p).w_layer.l_x == (*p).w_layer.l_width {
        addone = 1 as libc::c_int;
        (*p).w_layer.l_x -= 1;
        (*p).w_layer.l_x;
    }
    if (*p).w_layer.l_width == wi {
        ncx = (*p).w_layer.l_x + addone;
        ncy = (*p).w_layer.l_y + he - (*p).w_layer.l_height;
        shift = -ncy;
        yy = (*p).w_layer.l_y + (*p).w_histheight - 1 as libc::c_int;
        while yy >= 0 as libc::c_int && ncy + shift < he {
            ml = if yy < (*p).w_histheight {
                &mut *((*p).w_hlines)
                    .offset((((*p).w_histidx + yy) % (*p).w_histheight) as isize)
                    as *mut mline
            } else {
                &mut *((*p).w_mlines).offset((yy - (*p).w_histheight) as isize)
                    as *mut mline
            };
            if ((*ml).image).is_null() {
                break;
            }
            if *((*ml).image).offset((*p).w_layer.l_width as isize) as libc::c_int
                == ' ' as i32
            {
                break;
            }
            shift += 1;
            shift;
            yy -= 1;
            yy;
        }
        if shift < 0 as libc::c_int {
            shift = 0 as libc::c_int;
        }
        ncy += shift;
        if (*p).w_autoaka > 0 as libc::c_int {
            naka = (*p).w_autoaka + he - (*p).w_layer.l_height + shift;
            if naka < 1 as libc::c_int || naka > he {
                naka = 0 as libc::c_int;
            }
        }
        loop {
            let fresh2 = shift;
            shift = shift - 1;
            if !(fresh2 > 0 as libc::c_int) {
                break;
            }
            ml = if fy < (*p).w_histheight {
                &mut *((*p).w_hlines)
                    .offset((((*p).w_histidx + fy) % (*p).w_histheight) as isize)
                    as *mut mline
            } else {
                &mut *((*p).w_mlines).offset((fy - (*p).w_histheight) as isize)
                    as *mut mline
            };
            FreeMline(ml);
            fy -= 1;
            fy;
        }
    }
    if fy >= 0 as libc::c_int {
        mlf = if fy < (*p).w_histheight {
            &mut *((*p).w_hlines)
                .offset((((*p).w_histidx + fy) % (*p).w_histheight) as isize)
                as *mut mline
        } else {
            &mut *((*p).w_mlines).offset((fy - (*p).w_histheight) as isize) as *mut mline
        };
    }
    if ty >= 0 as libc::c_int {
        mlt = if ty < hi {
            &mut *nhlines.offset(ty as isize) as *mut mline
        } else {
            &mut *nmlines.offset((ty - hi) as isize) as *mut mline
        };
    }
    's_382: loop {
        if !(fy >= 0 as libc::c_int && ty >= 0 as libc::c_int) {
            current_block = 7157669805658135323;
            break;
        }
        if (*p).w_layer.l_width == wi {
            *mlt = *mlf;
            *mlf = mline_zero;
            fy -= 1;
            if fy >= 0 as libc::c_int {
                mlf = if fy < (*p).w_histheight {
                    &mut *((*p).w_hlines)
                        .offset((((*p).w_histidx + fy) % (*p).w_histheight) as isize)
                        as *mut mline
                } else {
                    &mut *((*p).w_mlines).offset((fy - (*p).w_histheight) as isize)
                        as *mut mline
                };
            }
            ty -= 1;
            if ty >= 0 as libc::c_int {
                mlt = if ty < hi {
                    &mut *nhlines.offset(ty as isize) as *mut mline
                } else {
                    &mut *nmlines.offset((ty - hi) as isize) as *mut mline
                };
            }
        } else {
            l = (*p).w_layer.l_width - 1 as libc::c_int;
            while l > 0 as libc::c_int {
                if *((*mlf).image).offset(l as isize) as libc::c_int != ' ' as i32
                    || *((*mlf).attr).offset(l as isize) as libc::c_int != 0
                {
                    break;
                }
                l -= 1;
                l;
            }
            if fy == (*p).w_layer.l_y + (*p).w_histheight && l < (*p).w_layer.l_x {
                l = (*p).w_layer.l_x;
            }
            l += 1;
            l;
            lf = l;
            yy = fy - 1 as libc::c_int;
            while yy >= 0 as libc::c_int {
                ml = if yy < (*p).w_histheight {
                    &mut *((*p).w_hlines)
                        .offset((((*p).w_histidx + yy) % (*p).w_histheight) as isize)
                        as *mut mline
                } else {
                    &mut *((*p).w_mlines).offset((yy - (*p).w_histheight) as isize)
                        as *mut mline
                };
                if *((*ml).image).offset((*p).w_layer.l_width as isize) as libc::c_int
                    == ' ' as i32
                {
                    break;
                }
                l += (*p).w_layer.l_width;
                yy -= 1;
                yy;
            }
            lt = (l - 1 as libc::c_int) % wi + 1 as libc::c_int;
            oty = ty;
            while l > 0 as libc::c_int && fy >= 0 as libc::c_int
                && ty >= 0 as libc::c_int
            {
                lx = if lt > lf { lf } else { lt };
                if ((*mlt).image).is_null() {
                    if AllocMline(mlt, wi + 1 as libc::c_int) != 0 {
                        current_block = 1434134386094521330;
                        break 's_382;
                    }
                    MakeBlankLine(((*mlt).image).offset(lt as isize), wi - lt);
                    *((*mlt).image)
                        .offset(
                            wi as isize,
                        ) = (if oty == ty { ' ' as i32 } else { 0 as libc::c_int })
                        as libc::c_uchar;
                }
                if BcopyMline(mlf, lf - lx, mlt, lt - lx, lx, wi + 1 as libc::c_int) != 0
                {
                    current_block = 1434134386094521330;
                    break 's_382;
                }
                if fy == (*p).w_layer.l_y + (*p).w_histheight
                    && lf - lx <= (*p).w_layer.l_x && lf > (*p).w_layer.l_x
                {
                    ncx = (*p).w_layer.l_x + lt - lf + addone;
                    ncy = ty - hi;
                    shift = if wi != 0 {
                        -ncy + (l - lx) / wi
                    } else {
                        0 as libc::c_int
                    };
                    if ty + shift > hi + he - 1 as libc::c_int {
                        shift = hi + he - 1 as libc::c_int - ty;
                    }
                    if shift > 0 as libc::c_int {
                        y = hi + he - 1 as libc::c_int;
                        while y >= ty {
                            mlt = if y < hi {
                                &mut *nhlines.offset(y as isize) as *mut mline
                            } else {
                                &mut *nmlines.offset((y - hi) as isize) as *mut mline
                            };
                            FreeMline(mlt);
                            if !(y - shift < ty) {
                                ml = if y - shift < hi {
                                    &mut *nhlines.offset((y - shift) as isize) as *mut mline
                                } else {
                                    &mut *nmlines.offset((y - shift - hi) as isize)
                                        as *mut mline
                                };
                                *mlt = *ml;
                                *ml = mline_zero;
                            }
                            y -= 1;
                            y;
                        }
                        ncy += shift;
                        ty += shift;
                        mlt = if ty < hi {
                            &mut *nhlines.offset(ty as isize) as *mut mline
                        } else {
                            &mut *nmlines.offset((ty - hi) as isize) as *mut mline
                        };
                        if naka > 0 as libc::c_int {
                            naka = if naka + shift > he {
                                0 as libc::c_int
                            } else {
                                naka + shift
                            };
                        }
                    }
                }
                if (*p).w_autoaka > 0 as libc::c_int
                    && fy == (*p).w_autoaka - 1 as libc::c_int + (*p).w_histheight
                    && lf - lx <= 0 as libc::c_int
                {
                    naka = if ty - hi >= 0 as libc::c_int {
                        1 as libc::c_int + ty - hi
                    } else {
                        0 as libc::c_int
                    };
                }
                lf -= lx;
                lt -= lx;
                l -= lx;
                if lf == 0 as libc::c_int {
                    FreeMline(mlf);
                    lf = (*p).w_layer.l_width;
                    fy -= 1;
                    if fy >= 0 as libc::c_int {
                        mlf = if fy < (*p).w_histheight {
                            &mut *((*p).w_hlines)
                                .offset(
                                    (((*p).w_histidx + fy) % (*p).w_histheight) as isize,
                                ) as *mut mline
                        } else {
                            &mut *((*p).w_mlines)
                                .offset((fy - (*p).w_histheight) as isize) as *mut mline
                        };
                    }
                }
                if lt == 0 as libc::c_int {
                    lt = wi;
                    ty -= 1;
                    if ty >= 0 as libc::c_int {
                        mlt = if ty < hi {
                            &mut *nhlines.offset(ty as isize) as *mut mline
                        } else {
                            &mut *nmlines.offset((ty - hi) as isize) as *mut mline
                        };
                    }
                }
            }
        }
    }
    match current_block {
        7157669805658135323 => {
            while fy >= 0 as libc::c_int {
                FreeMline(mlf);
                fy -= 1;
                if fy >= 0 as libc::c_int {
                    mlf = if fy < (*p).w_histheight {
                        &mut *((*p).w_hlines)
                            .offset((((*p).w_histidx + fy) % (*p).w_histheight) as isize)
                            as *mut mline
                    } else {
                        &mut *((*p).w_mlines).offset((fy - (*p).w_histheight) as isize)
                            as *mut mline
                    };
                }
            }
            loop {
                if !(ty >= 0 as libc::c_int) {
                    current_block = 2884634553824165030;
                    break;
                }
                if AllocMline(mlt, wi + 1 as libc::c_int) != 0 {
                    current_block = 1434134386094521330;
                    break;
                }
                MakeBlankLine((*mlt).image, wi + 1 as libc::c_int);
                ty -= 1;
                if ty >= 0 as libc::c_int {
                    mlt = if ty < hi {
                        &mut *nhlines.offset(ty as isize) as *mut mline
                    } else {
                        &mut *nmlines.offset((ty - hi) as isize) as *mut mline
                    };
                }
            }
            match current_block {
                1434134386094521330 => {}
                _ => {
                    if !((*p).w_mlines).is_null() && (*p).w_mlines != nmlines {
                        free((*p).w_mlines as *mut libc::c_char as *mut libc::c_void);
                    }
                    (*p).w_mlines = nmlines;
                    if !((*p).w_hlines).is_null() && (*p).w_hlines != nhlines {
                        free((*p).w_hlines as *mut libc::c_char as *mut libc::c_void);
                    }
                    (*p).w_hlines = nhlines;
                    nhlines = 0 as *mut mline;
                    nmlines = nhlines;
                    if (*p).w_layer.l_width != wi {
                        if wi != 0 {
                            t = if !((*p).w_tabs).is_null() {
                                (*p).w_layer.l_width
                            } else {
                                0 as libc::c_int
                            };
                            (*p).w_tabs = xrealloc((*p).w_tabs, wi + 1 as libc::c_int);
                            if ((*p).w_tabs).is_null() {
                                current_block = 1434134386094521330;
                            } else {
                                while t < wi {
                                    *((*p).w_tabs)
                                        .offset(
                                            t as isize,
                                        ) = (if t != 0 && t & 7 as libc::c_int == 0 {
                                        1 as libc::c_int
                                    } else {
                                        0 as libc::c_int
                                    }) as libc::c_char;
                                    t += 1;
                                    t;
                                }
                                *((*p).w_tabs)
                                    .offset(wi as isize) = 0 as libc::c_int as libc::c_char;
                                current_block = 11359721434352816539;
                            }
                        } else {
                            if !((*p).w_tabs).is_null() {
                                free((*p).w_tabs as *mut libc::c_void);
                            }
                            (*p).w_tabs = 0 as *mut libc::c_char;
                            current_block = 11359721434352816539;
                        }
                    } else {
                        current_block = 11359721434352816539;
                    }
                    match current_block {
                        1434134386094521330 => {}
                        _ => {
                            (*p).w_saved.y += ncy - (*p).w_layer.l_y;
                            (*p).w_layer.l_x = ncx;
                            (*p).w_layer.l_y = ncy;
                            if (*p).w_autoaka > 0 as libc::c_int {
                                (*p).w_autoaka = naka;
                            }
                            if (*p).w_layer.l_x > wi {
                                (*p).w_layer.l_x = wi;
                            }
                            if (*p).w_layer.l_y >= he {
                                (*p).w_layer.l_y = he - 1 as libc::c_int;
                            }
                            if (*p).w_saved.x > wi {
                                (*p).w_saved.x = wi;
                            }
                            if (*p).w_saved.y >= he {
                                (*p).w_saved.y = he - 1 as libc::c_int;
                            }
                            if (*p).w_saved.y < 0 as libc::c_int {
                                (*p).w_saved.y = 0 as libc::c_int;
                            }
                            if (*p).w_alt.cursor.x > wi {
                                (*p).w_alt.cursor.x = wi;
                            }
                            if (*p).w_alt.cursor.y >= he {
                                (*p).w_alt.cursor.y = he - 1 as libc::c_int;
                            }
                            if (*p).w_alt.cursor.y < 0 as libc::c_int {
                                (*p).w_alt.cursor.y = 0 as libc::c_int;
                            }
                            (*p).w_top = 0 as libc::c_int;
                            (*p).w_bot = he - 1 as libc::c_int;
                            if wi != 0
                                && ((*p).w_layer.l_width != wi
                                    || (*p).w_layer.l_height != he)
                                && (*p).w_layer.l_width != 0 as libc::c_int
                                && (*p).w_layer.l_height != 0 as libc::c_int
                                && (*p).w_ptyfd >= 0 as libc::c_int && (*p).w_pid != 0
                            {
                                glwz.ws_col = wi as libc::c_ushort;
                                glwz.ws_row = he as libc::c_ushort;
                                ioctl(
                                    (*p).w_ptyfd,
                                    0x5414 as libc::c_int as libc::c_ulong,
                                    &mut glwz as *mut winsize as *mut libc::c_char,
                                ) != 0;
                            }
                            (*p).w_layer.l_width = wi;
                            (*p).w_layer.l_height = he;
                            if (*p).w_scrollback_height > hi {
                                (*p).w_scrollback_height = hi;
                            }
                            (*p).w_histidx = 0 as libc::c_int;
                            (*p).w_histheight = hi;
                            return 0 as libc::c_int;
                        }
                    }
                }
            }
        }
        _ => {}
    }
    if !nmlines.is_null() || !nhlines.is_null() {
        ty = he + hi - 1 as libc::c_int;
        while ty >= 0 as libc::c_int {
            mlt = if ty < hi {
                &mut *nhlines.offset(ty as isize) as *mut mline
            } else {
                &mut *nmlines.offset((ty - hi) as isize) as *mut mline
            };
            FreeMline(mlt);
            ty -= 1;
            ty;
        }
        if !nmlines.is_null() && (*p).w_mlines != nmlines {
            free(nmlines as *mut libc::c_char as *mut libc::c_void);
        }
        if !nhlines.is_null() && (*p).w_hlines != nhlines {
            free(nhlines as *mut libc::c_char as *mut libc::c_void);
        }
    }
    KillWindow(p);
    Msg(
        0 as libc::c_int,
        b"%s\0" as *const u8 as *const libc::c_char,
        strnomem.as_mut_ptr(),
    );
    return -(1 as libc::c_int);
}
pub unsafe extern "C" fn FreeAltScreen(mut p: *mut win) {
    let mut i: libc::c_int = 0;
    if !((*p).w_alt.mlines).is_null() {
        i = 0 as libc::c_int;
        while i < (*p).w_alt.height {
            FreeMline(((*p).w_alt.mlines).offset(i as isize));
            i += 1;
            i;
        }
        free((*p).w_alt.mlines as *mut libc::c_void);
    }
    (*p).w_alt.mlines = 0 as *mut mline;
    (*p).w_alt.width = 0 as libc::c_int;
    (*p).w_alt.height = 0 as libc::c_int;
    if !((*p).w_alt.hlines).is_null() {
        i = 0 as libc::c_int;
        while i < (*p).w_alt.histheight {
            FreeMline(((*p).w_alt.hlines).offset(i as isize));
            i += 1;
            i;
        }
        free((*p).w_alt.hlines as *mut libc::c_void);
    }
    (*p).w_alt.hlines = 0 as *mut mline;
    (*p).w_alt.histidx = 0 as libc::c_int;
    (*p).w_alt.histheight = 0 as libc::c_int;
}
unsafe extern "C" fn SwapAltScreen(mut p: *mut win) {
    let mut ml: *mut mline = 0 as *mut mline;
    let mut t: libc::c_int = 0;
    ml = (*p).w_alt.mlines;
    (*p).w_alt.mlines = (*p).w_mlines;
    (*p).w_mlines = ml;
    t = (*p).w_alt.width;
    (*p).w_alt.width = (*p).w_layer.l_width;
    (*p).w_layer.l_width = t;
    t = (*p).w_alt.height;
    (*p).w_alt.height = (*p).w_layer.l_height;
    (*p).w_layer.l_height = t;
    t = (*p).w_alt.histheight;
    (*p).w_alt.histheight = (*p).w_histheight;
    (*p).w_histheight = t;
    ml = (*p).w_alt.hlines;
    (*p).w_alt.hlines = (*p).w_hlines;
    (*p).w_hlines = ml;
    t = (*p).w_alt.histidx;
    (*p).w_alt.histidx = (*p).w_histidx;
    (*p).w_histidx = t;
}
pub unsafe extern "C" fn EnterAltScreen(mut p: *mut win) {
    if (*p).w_alt.on == 0 {
        FreeAltScreen(p);
        SwapAltScreen(p);
    } else {
        (*p).w_layer.l_height = 0 as libc::c_int;
        (*p).w_histheight = 0 as libc::c_int;
    }
    ChangeWindowSize(p, (*p).w_alt.width, (*p).w_alt.height, (*p).w_alt.histheight);
    (*p).w_alt.on = 1 as libc::c_int;
}
pub unsafe extern "C" fn LeaveAltScreen(mut p: *mut win) {
    if (*p).w_alt.on == 0 {
        return;
    }
    SwapAltScreen(p);
    ChangeWindowSize(p, (*p).w_alt.width, (*p).w_alt.height, (*p).w_alt.histheight);
    FreeAltScreen(p);
    (*p).w_alt.on = 0 as libc::c_int;
}
