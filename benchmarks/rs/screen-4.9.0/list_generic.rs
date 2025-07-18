use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type logfile;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn abort() -> !;
    fn Input(
        _: *mut libc::c_char,
        _: libc::c_int,
        _: libc::c_int,
        _: Option::<
            unsafe extern "C" fn(*mut libc::c_char, libc::c_int, *mut libc::c_char) -> (),
        >,
        _: *mut libc::c_char,
        _: libc::c_int,
    );
    fn DefClearLine(_: libc::c_int, _: libc::c_int, _: libc::c_int, _: libc::c_int);
    fn DefRestore();
    fn SaveStr(_: *const libc::c_char) -> *mut libc::c_char;
    fn LGotoPos(_: *mut layer, _: libc::c_int, _: libc::c_int);
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
    fn InitOverlayPage(_: libc::c_int, _: *mut LayFuncs, _: libc::c_int) -> libc::c_int;
    fn ExitOverlayPage();
    fn LayProcessMouse(_: *mut layer, _: libc::c_uchar) -> libc::c_int;
    fn LayProcessMouseSwitch(_: *mut layer, _: libc::c_int);
    static mut flayer: *mut layer;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ListData {
    pub name: *const libc::c_char,
    pub root: *mut ListRow,
    pub selected: *mut ListRow,
    pub top: *mut ListRow,
    pub list_fn: *mut GenericList,
    pub search: *mut libc::c_char,
    pub data: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GenericList {
    pub gl_printheader: Option::<unsafe extern "C" fn(*mut ListData) -> libc::c_int>,
    pub gl_printfooter: Option::<unsafe extern "C" fn(*mut ListData) -> libc::c_int>,
    pub gl_printrow: Option::<
        unsafe extern "C" fn(*mut ListData, *mut ListRow) -> libc::c_int,
    >,
    pub gl_pinput: Option::<
        unsafe extern "C" fn(
            *mut ListData,
            *mut *mut libc::c_char,
            *mut libc::c_int,
        ) -> libc::c_int,
    >,
    pub gl_freerow: Option::<
        unsafe extern "C" fn(*mut ListData, *mut ListRow) -> libc::c_int,
    >,
    pub gl_free: Option::<unsafe extern "C" fn(*mut ListData) -> libc::c_int>,
    pub gl_matchrow: Option::<
        unsafe extern "C" fn(
            *mut ListData,
            *mut ListRow,
            *const libc::c_char,
        ) -> libc::c_int,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ListRow {
    pub data: *mut libc::c_void,
    pub next: *mut ListRow,
    pub prev: *mut ListRow,
    pub y: libc::c_int,
}
pub static mut ListLf: LayFuncs = unsafe {
    {
        let mut init = LayFuncs {
            lf_LayProcess: Some(
                ListProcess
                    as unsafe extern "C" fn(
                        *mut *mut libc::c_char,
                        *mut libc::c_int,
                    ) -> (),
            ),
            lf_LayAbort: Some(ListAbort as unsafe extern "C" fn() -> ()),
            lf_LayRedisplayLine: Some(
                ListRedisplayLine
                    as unsafe extern "C" fn(
                        libc::c_int,
                        libc::c_int,
                        libc::c_int,
                        libc::c_int,
                    ) -> (),
            ),
            lf_LayClearLine: Some(
                ListClearLine
                    as unsafe extern "C" fn(
                        libc::c_int,
                        libc::c_int,
                        libc::c_int,
                        libc::c_int,
                    ) -> (),
            ),
            lf_LayRewrite: Some(
                ListRewrite
                    as unsafe extern "C" fn(
                        libc::c_int,
                        libc::c_int,
                        libc::c_int,
                        *mut mchar,
                        libc::c_int,
                    ) -> libc::c_int,
            ),
            lf_LayResize: Some(
                ListResize
                    as unsafe extern "C" fn(libc::c_int, libc::c_int) -> libc::c_int,
            ),
            lf_LayRestore: Some(ListRestore as unsafe extern "C" fn() -> ()),
            lf_LayFree: Some(ListFree as unsafe extern "C" fn(*mut libc::c_void) -> ()),
        };
        init
    }
};
pub unsafe extern "C" fn glist_display(
    mut list: *mut GenericList,
    mut name: *const libc::c_char,
) -> *mut ListData {
    let mut ldata: *mut ListData = 0 as *mut ListData;
    if InitOverlayPage(
        ::std::mem::size_of::<ListData>() as libc::c_ulong as libc::c_int,
        &mut ListLf,
        0 as libc::c_int,
    ) != 0
    {
        return 0 as *mut ListData;
    }
    ldata = (*flayer).l_data as *mut ListData;
    (*ldata).name = name;
    (*ldata).list_fn = list;
    (*flayer).l_mode = 1 as libc::c_int;
    (*flayer).l_x = 0 as libc::c_int;
    (*flayer).l_y = (*flayer).l_height - 1 as libc::c_int;
    return ldata;
}
unsafe extern "C" fn glist_decide_top(mut ldata: *mut ListData) {
    let mut count: libc::c_int = (*flayer).l_height - 5 as libc::c_int;
    let mut top: *mut ListRow = (*ldata).selected;
    while count != 0 && top != (*ldata).root {
        top = (*top).prev;
        count -= 1;
        count;
    }
    (*ldata).top = top;
}
unsafe extern "C" fn glist_search_dir(
    mut ldata: *mut ListData,
    mut start: *mut ListRow,
    mut dir: libc::c_int,
) -> *mut ListRow {
    let mut row: *mut ListRow = if dir == 1 as libc::c_int {
        (*start).next
    } else {
        (*start).prev
    };
    while !row.is_null() {
        if ((*(*ldata).list_fn).gl_matchrow).unwrap()(ldata, row, (*ldata).search) != 0 {
            return row;
        }
        row = if dir == 1 as libc::c_int { (*row).next } else { (*row).prev };
    }
    if dir == 1 as libc::c_int {
        row = (*ldata).root;
    } else if ((*start).next).is_null() {
        row = start;
    } else {
        row = (*start).next;
        while !((*row).next).is_null() {
            row = (*row).next;
        }
    }
    while row != start {
        if ((*(*ldata).list_fn).gl_matchrow).unwrap()(ldata, row, (*ldata).search) != 0 {
            break;
        }
        row = if dir == 1 as libc::c_int { (*row).next } else { (*row).prev };
    }
    return row;
}
unsafe extern "C" fn glist_search(
    mut buf: *mut libc::c_char,
    mut len: libc::c_int,
    mut data: *mut libc::c_char,
) {
    let mut ldata: *mut ListData = data as *mut ListData;
    let mut row: *mut ListRow = 0 as *mut ListRow;
    if !((*ldata).search).is_null() {
        if ((*ldata).search).is_null() {
            abort();
        } else {
            free((*ldata).search as *mut libc::c_void);
        }
        (*ldata).search = 0 as *mut libc::c_char;
    }
    if len > 0 as libc::c_int {
        (*ldata).search = SaveStr(buf);
    } else {
        return
    }
    row = (*ldata).selected;
    while !row.is_null() {
        if ((*(*ldata).list_fn).gl_matchrow).unwrap()(ldata, row, (*ldata).search) != 0 {
            break;
        }
        row = (*row).next;
    }
    if row.is_null() {
        row = (*ldata).root;
        while row != (*ldata).selected {
            if ((*(*ldata).list_fn).gl_matchrow).unwrap()(ldata, row, (*ldata).search)
                != 0
            {
                break;
            }
            row = (*row).next;
        }
    }
    if row == (*ldata).selected {
        return;
    }
    (*ldata).selected = row;
    if (*(*ldata).selected).y == -(1 as libc::c_int) {
        glist_decide_top(ldata);
    }
    glist_display_all(ldata);
}
unsafe extern "C" fn ListProcess(
    mut ppbuf: *mut *mut libc::c_char,
    mut plen: *mut libc::c_int,
) {
    let mut ldata: *mut ListData = (*flayer).l_data as *mut ListData;
    let mut count: libc::c_int = 0 as libc::c_int;
    while *plen > 0 as libc::c_int {
        let mut old: *mut ListRow = 0 as *mut ListRow;
        let mut ch: libc::c_uchar = 0;
        if (*flayer).l_mouseevent.start == 0 && ((*(*ldata).list_fn).gl_pinput).is_some()
            && ((*(*ldata).list_fn).gl_pinput).unwrap()(ldata, ppbuf, plen) != 0
        {
            continue;
        }
        ch = **ppbuf as libc::c_uchar;
        *ppbuf = (*ppbuf).offset(1);
        *ppbuf;
        *plen -= 1;
        *plen;
        if (*flayer).l_mouseevent.start != 0 {
            let mut r: libc::c_int = LayProcessMouse(flayer, ch);
            if r == -(1 as libc::c_int) {
                LayProcessMouseSwitch(flayer, 0 as libc::c_int);
                continue;
            } else {
                if !(r != 0) {
                    continue;
                }
                ch = 0o222 as libc::c_int as libc::c_uchar;
            }
        }
        if ((*ldata).selected).is_null() {
            *plen = 0 as libc::c_int;
            break;
        } else {
            old = (*ldata).selected;
            let mut current_block_49: u64;
            loop {
                match ch as libc::c_int {
                    144 => {
                        current_block_49 = 10366477599799017231;
                        break;
                    }
                    16 | 107 => {
                        current_block_49 = 10366477599799017231;
                        break;
                    }
                    142 => {
                        current_block_49 = 17444474504745170610;
                        break;
                    }
                    14 | 106 => {
                        current_block_49 = 17444474504745170610;
                        break;
                    }
                    27 | 7 => {
                        ListAbort();
                        *plen = 0 as libc::c_int;
                        return;
                    }
                    129 | 1 => {
                        (*ldata).selected = (*ldata).root;
                        current_block_49 = 15462640364611497761;
                        break;
                    }
                    133 | 5 => {
                        while !((*(*ldata).selected).next).is_null() {
                            (*ldata).selected = (*(*ldata).selected).next;
                        }
                        (*(*ldata).selected).y != -(1 as libc::c_int);
                        current_block_49 = 15462640364611497761;
                        break;
                    }
                    4 | 6 => {
                        count = (*flayer).l_height - 4 as libc::c_int
                            >> (ch as libc::c_int == 0o4 as libc::c_int) as libc::c_int;
                        while !((*(*ldata).selected).next).is_null()
                            && {
                                count -= 1;
                                count != 0
                            }
                        {
                            (*ldata).selected = (*(*ldata).selected).next;
                        }
                        current_block_49 = 15462640364611497761;
                        break;
                    }
                    21 | 2 => {
                        count = (*flayer).l_height - 4 as libc::c_int
                            >> (ch as libc::c_int == 0o25 as libc::c_int) as libc::c_int;
                        while !((*(*ldata).selected).prev).is_null()
                            && {
                                count -= 1;
                                count != 0
                            }
                        {
                            (*ldata).selected = (*(*ldata).selected).prev;
                        }
                        current_block_49 = 15462640364611497761;
                        break;
                    }
                    47 => {
                        if ((*(*ldata).list_fn).gl_matchrow).is_some() {
                            let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
                            Input(
                                b"Search: \0" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char,
                                80 as libc::c_int,
                                0 as libc::c_int,
                                Some(
                                    glist_search
                                        as unsafe extern "C" fn(
                                            *mut libc::c_char,
                                            libc::c_int,
                                            *mut libc::c_char,
                                        ) -> (),
                                ),
                                ldata as *mut libc::c_char,
                                0 as libc::c_int,
                            );
                            s = (*ldata).search;
                            if !s.is_null() {
                                while *s != 0 {
                                    let mut ss: *mut libc::c_char = s;
                                    let mut n: libc::c_int = 1 as libc::c_int;
                                    (Some(((*(*flayer).l_layfn).lf_LayProcess).unwrap()))
                                        .unwrap()(&mut ss, &mut n);
                                    s = s.offset(1);
                                    s;
                                }
                            }
                        }
                        current_block_49 = 15462640364611497761;
                        break;
                    }
                    110 => {
                        if ((*(*ldata).list_fn).gl_matchrow).is_some()
                            && !((*ldata).search).is_null()
                        {
                            (*ldata)
                                .selected = glist_search_dir(
                                ldata,
                                (*ldata).selected,
                                1 as libc::c_int,
                            );
                        }
                        current_block_49 = 15462640364611497761;
                        break;
                    }
                    78 => {
                        if ((*(*ldata).list_fn).gl_matchrow).is_some()
                            && !((*ldata).search).is_null()
                        {
                            (*ldata)
                                .selected = glist_search_dir(
                                ldata,
                                (*ldata).selected,
                                -(1 as libc::c_int),
                            );
                        }
                        current_block_49 = 15462640364611497761;
                        break;
                    }
                    146 => {
                        if (*flayer).l_mouseevent.start != 0 {
                            let mut button: libc::c_int = (*flayer)
                                .l_mouseevent
                                .buffer[0 as libc::c_int as usize] as libc::c_int;
                            if button == 'a' as i32 {
                                ch = 'j' as i32 as libc::c_uchar;
                            } else if button == '`' as i32 {
                                ch = 'k' as i32 as libc::c_uchar;
                            } else if button == ' ' as i32 {
                                let mut y: libc::c_int = (*flayer)
                                    .l_mouseevent
                                    .buffer[2 as libc::c_int as usize] as libc::c_int;
                                let mut r_0: *mut ListRow = (*ldata).top;
                                r_0 = (*ldata).top;
                                while !r_0.is_null() && (*r_0).y != -(1 as libc::c_int)
                                    && (*r_0).y != y
                                {
                                    r_0 = (*r_0).next;
                                }
                                if !r_0.is_null() && (*r_0).y == y {
                                    (*ldata).selected = r_0;
                                }
                                ch = 0 as libc::c_int as libc::c_uchar;
                            } else {
                                ch = 0 as libc::c_int as libc::c_uchar;
                            }
                            LayProcessMouseSwitch(flayer, 0 as libc::c_int);
                            if !(ch != 0) {
                                current_block_49 = 15462640364611497761;
                                break;
                            }
                        } else {
                            LayProcessMouseSwitch(flayer, 1 as libc::c_int);
                            current_block_49 = 15462640364611497761;
                            break;
                        }
                    }
                    32 | 13 | 10 | _ => {
                        current_block_49 = 15462640364611497761;
                        break;
                    }
                }
            }
            match current_block_49 {
                17444474504745170610 => {
                    if !((*(*ldata).selected).next).is_null() {
                        (*ldata).selected = (*old).next;
                    }
                }
                10366477599799017231 => {
                    if !((*(*ldata).selected).prev).is_null() {
                        (*ldata).selected = (*old).prev;
                    }
                }
                _ => {}
            }
            if old == (*ldata).selected {
                continue;
            }
            if (*(*ldata).selected).y == -(1 as libc::c_int) {
                glist_decide_top(ldata);
                glist_display_all(ldata);
            } else {
                ((*(*ldata).list_fn).gl_printrow).unwrap()(ldata, old);
                ((*(*ldata).list_fn).gl_printrow).unwrap()(ldata, (*ldata).selected);
                (*flayer).l_y = (*(*ldata).selected).y;
                LGotoPos(flayer, (*flayer).l_x, (*flayer).l_y);
            }
        }
    }
}
unsafe extern "C" fn ListAbort() {
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
unsafe extern "C" fn ListFree(mut d: *mut libc::c_void) {
    let mut ldata: *mut ListData = d as *mut ListData;
    glist_remove_rows(ldata);
    if ((*(*ldata).list_fn).gl_free).is_some() {
        ((*(*ldata).list_fn).gl_free).unwrap()(ldata);
    }
    if !((*ldata).search).is_null() {
        if ((*ldata).search).is_null() {
            abort();
        } else {
            free((*ldata).search as *mut libc::c_void);
        }
        (*ldata).search = 0 as *mut libc::c_char;
    }
}
unsafe extern "C" fn ListRedisplayLine(
    mut y: libc::c_int,
    mut xs: libc::c_int,
    mut xe: libc::c_int,
    mut isblank: libc::c_int,
) {
    let mut ldata: *mut ListData = 0 as *mut ListData;
    ldata = (*flayer).l_data as *mut ListData;
    if y < 0 as libc::c_int {
        glist_display_all(ldata);
        return;
    }
    if isblank == 0 {
        LClearArea(flayer, xs, y, xe, y, 0 as libc::c_int, 0 as libc::c_int);
    }
    if !((*ldata).top).is_null() && y < (*(*ldata).top).y {
        ((*(*ldata).list_fn).gl_printheader).unwrap()(ldata);
    } else if y + 1 as libc::c_int == (*flayer).l_height {
        ((*(*ldata).list_fn).gl_printfooter).unwrap()(ldata);
    } else {
        let mut row: *mut ListRow = 0 as *mut ListRow;
        row = (*ldata).top;
        while !row.is_null() && (*row).y != -(1 as libc::c_int) {
            if (*row).y == y {
                ((*(*ldata).list_fn).gl_printrow).unwrap()(ldata, row);
                break;
            } else {
                row = (*row).next;
            }
        }
    };
}
unsafe extern "C" fn ListClearLine(
    mut y: libc::c_int,
    mut xs: libc::c_int,
    mut xe: libc::c_int,
    mut bce: libc::c_int,
) {
    DefClearLine(y, xs, xe, bce);
}
unsafe extern "C" fn ListRewrite(
    mut y: libc::c_int,
    mut xs: libc::c_int,
    mut xe: libc::c_int,
    mut rend: *mut mchar,
    mut doit: libc::c_int,
) -> libc::c_int {
    return 1000 as libc::c_int;
}
unsafe extern "C" fn ListResize(
    mut wi: libc::c_int,
    mut he: libc::c_int,
) -> libc::c_int {
    if wi < 10 as libc::c_int || he < 5 as libc::c_int {
        return -(1 as libc::c_int);
    }
    (*flayer).l_width = wi;
    (*flayer).l_height = he;
    (*flayer).l_y = he - 1 as libc::c_int;
    return 0 as libc::c_int;
}
unsafe extern "C" fn ListRestore() {
    DefRestore();
}
pub unsafe extern "C" fn glist_add_row(
    mut ldata: *mut ListData,
    mut data: *mut libc::c_void,
    mut after: *mut ListRow,
) -> *mut ListRow {
    let mut r: *mut ListRow = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<ListRow>() as libc::c_ulong,
    ) as *mut ListRow;
    (*r).data = data;
    if !after.is_null() {
        (*r).next = (*after).next;
        (*r).prev = after;
        (*after).next = r;
        if !((*r).next).is_null() {
            (*(*r).next).prev = r;
        }
    } else {
        (*r).next = (*ldata).root;
        if !((*ldata).root).is_null() {
            (*(*ldata).root).prev = r;
        }
        (*ldata).root = r;
    }
    return r;
}
pub unsafe extern "C" fn glist_remove_rows(mut ldata: *mut ListData) {
    let mut row: *mut ListRow = 0 as *mut ListRow;
    row = (*ldata).root;
    while !row.is_null() {
        let mut r: *mut ListRow = row;
        row = (*row).next;
        ((*(*ldata).list_fn).gl_freerow).unwrap()(ldata, r);
        free(r as *mut libc::c_void);
    }
    (*ldata).top = 0 as *mut ListRow;
    (*ldata).selected = (*ldata).top;
    (*ldata).root = (*ldata).selected;
}
pub unsafe extern "C" fn glist_display_all(mut list: *mut ListData) {
    let mut y: libc::c_int = 0;
    let mut row: *mut ListRow = 0 as *mut ListRow;
    LClearAll(flayer, 0 as libc::c_int);
    y = ((*(*list).list_fn).gl_printheader).unwrap()(list);
    if ((*list).top).is_null() {
        (*list).top = (*list).root;
    }
    if ((*list).selected).is_null() {
        (*list).selected = (*list).root;
    }
    row = (*list).root;
    while row != (*list).top {
        (*row).y = -(1 as libc::c_int);
        row = (*row).next;
    }
    row = (*list).top;
    while !row.is_null() {
        let fresh0 = y;
        y = y + 1;
        (*row).y = fresh0;
        if ((*(*list).list_fn).gl_printrow).unwrap()(list, row) == 0 {
            (*row).y = -(1 as libc::c_int);
            y -= 1;
            y;
        }
        if y + 1 as libc::c_int == (*flayer).l_height {
            break;
        }
        row = (*row).next;
    }
    while !row.is_null() {
        (*row).y = -(1 as libc::c_int);
        row = (*row).next;
    }
    ((*(*list).list_fn).gl_printfooter).unwrap()(list);
    if !((*list).selected).is_null() && (*(*list).selected).y != -(1 as libc::c_int) {
        (*flayer).l_y = (*(*list).selected).y;
    } else {
        (*flayer).l_y = (*flayer).l_height - 1 as libc::c_int;
    }
    LGotoPos(flayer, (*flayer).l_x, (*flayer).l_y);
}
pub unsafe extern "C" fn glist_abort() {
    ListAbort();
}
