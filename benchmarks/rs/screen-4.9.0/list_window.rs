use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type logfile;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn abort() -> !;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn WindowChangeNumber(_: libc::c_int, _: libc::c_int) -> libc::c_int;
    fn MakeWinMsgEv(
        _: *mut libc::c_char,
        _: *mut win,
        _: libc::c_int,
        _: libc::c_int,
        _: *mut event,
        _: libc::c_int,
    ) -> *mut libc::c_char;
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
    fn DoAction(_: *mut action, _: libc::c_int);
    fn Activate(_: libc::c_int);
    fn SetForeWindow(_: *mut win);
    fn ApplyAttrColor(_: libc::c_int, _: *mut mchar);
    fn SwitchWindow(_: libc::c_int);
    fn InStr(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn AclCheckPermWin(_: *mut acluser, _: libc::c_int, _: *mut win) -> libc::c_int;
    fn LGotoPos(_: *mut layer, _: libc::c_int, _: libc::c_int);
    fn LPutWinMsg(
        _: *mut layer,
        _: *mut libc::c_char,
        _: libc::c_int,
        _: *mut mchar,
        _: libc::c_int,
        _: libc::c_int,
    );
    fn LMsg(_: libc::c_int, _: *const libc::c_char, _: ...);
    static mut ListLf: LayFuncs;
    fn glist_add_row(
        ldata: *mut ListData,
        data: *mut libc::c_void,
        after: *mut ListRow,
    ) -> *mut ListRow;
    fn glist_remove_rows(ldata: *mut ListData);
    fn glist_display_all(list: *mut ListData);
    fn glist_display(list: *mut GenericList, name: *const libc::c_char) -> *mut ListData;
    fn glist_abort();
    static mut flayer: *mut layer;
    static mut display: *mut display;
    static mut displays: *mut display;
    static mut wlisttit: *mut libc::c_char;
    static mut wliststr: *mut libc::c_char;
    static mut mchar_blank: mchar;
    static mut mchar_so: mchar;
    static mut renditions: [libc::c_int; 0];
    static mut wtab: *mut *mut win;
    static mut windows: *mut win;
    static mut fore: *mut win;
    static mut maxwin: libc::c_int;
    static mut noargs: [*mut libc::c_char; 0];
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
pub type C2RustUnnamed_3 = libc::c_uint;
pub const NUM_RENDS: C2RustUnnamed_3 = 3;
pub const REND_SILENCE: C2RustUnnamed_3 = 2;
pub const REND_MONITOR: C2RustUnnamed_3 = 1;
pub const REND_BELL: C2RustUnnamed_3 = 0;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gl_Window_Data {
    pub group: *mut win,
    pub order: libc::c_int,
    pub onblank: libc::c_int,
    pub nested: libc::c_int,
    pub fore: *mut win,
}
static mut ListID: [libc::c_char; 7] = unsafe {
    *::std::mem::transmute::<&[u8; 7], &mut [libc::c_char; 7]>(b"window\0")
};
unsafe extern "C" fn window_ancestor(mut a: *mut win, mut d: *mut win) -> libc::c_int {
    if a.is_null() {
        return 1 as libc::c_int;
    }
    while !d.is_null() {
        if (*d).w_group == a {
            return 1 as libc::c_int;
        }
        d = (*d).w_group;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn window_kill_confirm(
    mut buf: *mut libc::c_char,
    mut len: libc::c_int,
    mut data: *mut libc::c_char,
) {
    let mut w: *mut win = windows;
    let mut act: action = action {
        nr: 0,
        args: 0 as *mut *mut libc::c_char,
        argl: 0 as *mut libc::c_int,
        quiet: 0,
    };
    if len != 0 || *buf as libc::c_int != 'y' as i32 && *buf as libc::c_int != 'Y' as i32
    {
        memset(buf as *mut libc::c_void, 0 as libc::c_int, len as libc::c_ulong);
        return;
    }
    while !w.is_null() {
        if w == data as *mut win {
            break;
        }
        w = (*w).w_next;
    }
    if w.is_null() {
        return;
    }
    fore = w;
    act.nr = 96 as libc::c_int;
    act.args = noargs.as_mut_ptr();
    act.argl = 0 as *mut libc::c_int;
    act.quiet = 0 as libc::c_int;
    DoAction(&mut act, -(1 as libc::c_int));
}
unsafe extern "C" fn gl_Window_add_group(
    mut ldata: *mut ListData,
    mut row: *mut ListRow,
) -> *mut ListRow {
    let mut wdata: *mut gl_Window_Data = (*ldata).data as *mut gl_Window_Data;
    let mut group: *mut win = (*row).data as *mut win;
    let mut w: *mut win = 0 as *mut win;
    let mut cur: *mut ListRow = row;
    if (*wdata).order == 1 as libc::c_int {
        let mut _ww: *mut win = 0 as *mut win;
        _ww = windows;
        while !_ww.is_null() {
            w = _ww;
            if !((*w).w_group != group) {
                cur = glist_add_row(ldata, w as *mut libc::c_void, cur);
                if w == (*wdata).fore {
                    (*ldata).selected = cur;
                }
                if (*w).w_type == 3 as libc::c_int {
                    cur = gl_Window_add_group(ldata, cur);
                }
            }
            _ww = (*_ww).w_next;
        }
    } else {
        let mut _ww_0: *mut *mut win = 0 as *mut *mut win;
        let mut _witer: *mut win = 0 as *mut win;
        _ww_0 = wtab;
        _witer = windows;
        while !_witer.is_null()
            && (_ww_0.offset_from(wtab) as libc::c_long) < maxwin as libc::c_long
        {
            w = *_ww_0;
            if !w.is_null() {
                if !((*w).w_group != group) {
                    cur = glist_add_row(ldata, w as *mut libc::c_void, cur);
                    if w == (*wdata).fore {
                        (*ldata).selected = cur;
                    }
                    if (*w).w_type == 3 as libc::c_int {
                        cur = gl_Window_add_group(ldata, cur);
                    }
                    _witer = (*_witer).w_next;
                }
            }
            _ww_0 = _ww_0.offset(1);
            _ww_0;
        }
    }
    return cur;
}
unsafe extern "C" fn gl_Window_rebuild(mut ldata: *mut ListData) {
    let mut row: *mut ListRow = 0 as *mut ListRow;
    let mut wdata: *mut gl_Window_Data = (*ldata).data as *mut gl_Window_Data;
    let mut w: *mut win = 0 as *mut win;
    if (*wdata).order == 1 as libc::c_int {
        let mut _ww: *mut win = 0 as *mut win;
        _ww = windows;
        while !_ww.is_null() {
            w = _ww;
            if !((*w).w_group != (*wdata).group) {
                row = glist_add_row(ldata, w as *mut libc::c_void, row);
                if w == (*wdata).fore {
                    (*ldata).selected = row;
                }
                if (*w).w_type == 3 as libc::c_int && (*wdata).nested != 0 {
                    row = gl_Window_add_group(ldata, row);
                }
            }
            _ww = (*_ww).w_next;
        }
    } else {
        let mut _ww_0: *mut *mut win = 0 as *mut *mut win;
        let mut _witer: *mut win = 0 as *mut win;
        _ww_0 = wtab;
        _witer = windows;
        while !_witer.is_null()
            && (_ww_0.offset_from(wtab) as libc::c_long) < maxwin as libc::c_long
        {
            w = *_ww_0;
            if !w.is_null() {
                if !((*w).w_group != (*wdata).group) {
                    row = glist_add_row(ldata, w as *mut libc::c_void, row);
                    if w == (*wdata).fore {
                        (*ldata).selected = row;
                    }
                    if (*w).w_type == 3 as libc::c_int && (*wdata).nested != 0 {
                        row = gl_Window_add_group(ldata, row);
                    }
                    _witer = (*_witer).w_next;
                }
            }
            _ww_0 = _ww_0.offset(1);
            _ww_0;
        }
    }
    glist_display_all(ldata);
}
unsafe extern "C" fn gl_Window_findrow(
    mut ldata: *mut ListData,
    mut p: *mut win,
) -> *mut ListRow {
    let mut row: *mut ListRow = (*ldata).root;
    while !row.is_null() {
        if (*row).data == p as *mut libc::c_void {
            break;
        }
        row = (*row).next;
    }
    return row;
}
unsafe extern "C" fn gl_Window_remove(
    mut ldata: *mut ListData,
    mut p: *mut win,
) -> libc::c_int {
    let mut row: *mut ListRow = gl_Window_findrow(ldata, p);
    if row.is_null() {
        return 0 as libc::c_int;
    }
    if !((*row).next).is_null() {
        (*(*row).next).prev = (*row).prev;
    }
    if !((*row).prev).is_null() {
        (*(*row).prev).next = (*row).next;
    }
    if (*ldata).selected == row {
        (*ldata)
            .selected = if !((*row).prev).is_null() { (*row).prev } else { (*row).next };
    }
    if (*ldata).top == row {
        (*ldata).top = if !((*row).prev).is_null() { (*row).prev } else { (*row).next };
    }
    if (*ldata).root == row {
        (*ldata).root = (*row).next;
    }
    ((*(*ldata).list_fn).gl_freerow).unwrap()(ldata, row);
    free(row as *mut libc::c_void);
    return 1 as libc::c_int;
}
unsafe extern "C" fn gl_Window_header(mut ldata: *mut ListData) -> libc::c_int {
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut wdata: *mut gl_Window_Data = (*ldata).data as *mut gl_Window_Data;
    let mut g: libc::c_int = 0;
    g = ((*wdata).group != 0 as *mut libc::c_void as *mut win) as libc::c_int;
    if g != 0 {
        LPutWinMsg(
            flayer,
            b"Group: \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            7 as libc::c_int,
            &mut mchar_blank,
            0 as libc::c_int,
            0 as libc::c_int,
        );
        LPutWinMsg(
            flayer,
            (*(*wdata).group).w_title,
            strlen((*(*wdata).group).w_title) as libc::c_int,
            &mut mchar_blank,
            7 as libc::c_int,
            0 as libc::c_int,
        );
    }
    str = MakeWinMsgEv(
        wlisttit,
        0 as *mut win,
        '%' as i32,
        (*flayer).l_width,
        0 as *mut event,
        0 as libc::c_int,
    );
    LPutWinMsg(
        flayer,
        str,
        strlen(str) as libc::c_int,
        &mut mchar_blank,
        0 as libc::c_int,
        g,
    );
    return 2 as libc::c_int + g;
}
unsafe extern "C" fn gl_Window_footer(mut ldata: *mut ListData) -> libc::c_int {
    return 0 as libc::c_int;
}
unsafe extern "C" fn gl_Window_row(
    mut ldata: *mut ListData,
    mut lrow: *mut ListRow,
) -> libc::c_int {
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut w: *mut win = 0 as *mut win;
    let mut g: *mut win = 0 as *mut win;
    let mut xoff: libc::c_int = 0;
    let mut mchar: *mut mchar = 0 as *mut mchar;
    let mut mchar_rend: mchar = mchar_blank;
    let mut wdata: *mut gl_Window_Data = (*ldata).data as *mut gl_Window_Data;
    w = (*lrow).data as *mut win;
    xoff = 0 as libc::c_int;
    g = (*w).w_group;
    while g != (*wdata).group {
        xoff += 2 as libc::c_int;
        g = (*g).w_group;
    }
    str = MakeWinMsgEv(
        wliststr,
        w,
        '%' as i32,
        (*flayer).l_width - xoff,
        0 as *mut event,
        0 as libc::c_int,
    );
    if (*ldata).selected == lrow {
        mchar = &mut mchar_so;
    } else if (*w).w_monitor == 3 as libc::c_int
        && *renditions.as_mut_ptr().offset(REND_MONITOR as libc::c_int as isize)
            != -(1 as libc::c_int)
    {
        mchar = &mut mchar_rend;
        ApplyAttrColor(
            *renditions.as_mut_ptr().offset(REND_MONITOR as libc::c_int as isize),
            mchar,
        );
    } else if ((*w).w_bell == 2 as libc::c_int || (*w).w_bell == 1 as libc::c_int)
        && *renditions.as_mut_ptr().offset(REND_BELL as libc::c_int as isize)
            != -(1 as libc::c_int)
    {
        mchar = &mut mchar_rend;
        ApplyAttrColor(
            *renditions.as_mut_ptr().offset(REND_BELL as libc::c_int as isize),
            mchar,
        );
    } else if ((*w).w_silence == 2 as libc::c_int || (*w).w_silence == 3 as libc::c_int)
        && *renditions.as_mut_ptr().offset(REND_SILENCE as libc::c_int as isize)
            != -(1 as libc::c_int)
    {
        mchar = &mut mchar_rend;
        ApplyAttrColor(
            *renditions.as_mut_ptr().offset(REND_SILENCE as libc::c_int as isize),
            mchar,
        );
    } else {
        mchar = &mut mchar_blank;
    }
    LPutWinMsg(flayer, str, (*flayer).l_width, mchar, xoff, (*lrow).y);
    if xoff != 0 {
        LPutWinMsg(
            flayer,
            b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            xoff,
            mchar,
            0 as libc::c_int,
            (*lrow).y,
        );
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn gl_Window_input(
    mut ldata: *mut ListData,
    mut inp: *mut *mut libc::c_char,
    mut len: *mut libc::c_int,
) -> libc::c_int {
    let mut win: *mut win = 0 as *mut win;
    let mut ch: libc::c_uchar = 0;
    let mut cd: *mut display = display;
    let mut wdata: *mut gl_Window_Data = (*ldata).data as *mut gl_Window_Data;
    if ((*ldata).selected).is_null() {
        return 0 as libc::c_int;
    }
    ch = **inp as libc::c_uchar;
    *inp = (*inp).offset(1);
    *inp;
    *len -= 1;
    *len;
    win = (*(*ldata).selected).data as *mut win;
    let mut current_block_71: u64;
    match ch as libc::c_int {
        32 | 10 | 13 => {
            if win.is_null() {
                current_block_71 = 11052029508375673978;
            } else {
                if !display.is_null()
                    && AclCheckPermWin((*display).d_user, 2 as libc::c_int, win) != 0
                {
                    return 0 as libc::c_int;
                }
                if !((*wdata).group).is_null() && (*wdata).onblank == 0
                    && !((*(*flayer).l_bottom).l_data as *mut win).is_null()
                    && (*((*(*flayer).l_bottom).l_data as *mut win)).w_type
                        == 3 as libc::c_int
                {
                    SwitchWindow((*win).w_number);
                } else {
                    glist_abort();
                    display = cd;
                    if (*display).d_fore != win {
                        SwitchWindow((*win).w_number);
                    }
                }
                *len = 0 as libc::c_int;
                current_block_71 = 11052029508375673978;
            }
        }
        109 => {
            (*wdata)
                .order = if (*wdata).order == 1 as libc::c_int {
                0 as libc::c_int
            } else {
                1 as libc::c_int
            };
            glist_remove_rows(ldata);
            gl_Window_rebuild(ldata);
            current_block_71 = 11052029508375673978;
        }
        103 => {
            (*wdata).nested = ((*wdata).nested == 0) as libc::c_int;
            glist_remove_rows(ldata);
            gl_Window_rebuild(ldata);
            current_block_71 = 11052029508375673978;
        }
        97 => {
            if !((*wdata).group).is_null() {
                let mut order: libc::c_int = (*wdata).order
                    | (if (*wdata).nested != 0 {
                        2 as libc::c_int
                    } else {
                        0 as libc::c_int
                    });
                glist_abort();
                display = cd;
                display_windows(1 as libc::c_int, order, 0 as *mut win);
                *len = 0 as libc::c_int;
            } else if (*wdata).nested == 0 {
                (*wdata).nested = 1 as libc::c_int;
                glist_remove_rows(ldata);
                gl_Window_rebuild(ldata);
            }
            current_block_71 = 11052029508375673978;
        }
        8 => {
            current_block_71 = 15109860607434291925;
        }
        127 => {
            current_block_71 = 15109860607434291925;
        }
        44 => {
            if (*wdata).order == 0 as libc::c_int
                && !((*(*ldata).selected).prev).is_null()
            {
                let mut pw: *mut win = (*(*(*ldata).selected).prev).data as *mut win;
                if (*win).w_group != (*pw).w_group {
                    current_block_71 = 11052029508375673978;
                } else {
                    (*wdata).fore = win;
                    WindowChangeNumber((*win).w_number, (*pw).w_number);
                    current_block_71 = 11052029508375673978;
                }
            } else {
                current_block_71 = 11052029508375673978;
            }
        }
        46 => {
            if (*wdata).order == 0 as libc::c_int
                && !((*(*ldata).selected).next).is_null()
            {
                let mut nw: *mut win = (*(*(*ldata).selected).next).data as *mut win;
                if (*win).w_group != (*nw).w_group {
                    current_block_71 = 11052029508375673978;
                } else {
                    (*wdata).fore = win;
                    WindowChangeNumber((*win).w_number, (*nw).w_number);
                    current_block_71 = 11052029508375673978;
                }
            } else {
                current_block_71 = 11052029508375673978;
            }
        }
        75 => {
            let mut str: [libc::c_char; 768] = [0; 768];
            snprintf(
                str.as_mut_ptr(),
                (::std::mem::size_of::<[libc::c_char; 768]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                b"Really kill window %d (%s) [y/n]\0" as *const u8
                    as *const libc::c_char,
                (*win).w_number,
                (*win).w_title,
            );
            Input(
                str.as_mut_ptr(),
                1 as libc::c_int,
                2 as libc::c_int,
                Some(
                    window_kill_confirm
                        as unsafe extern "C" fn(
                            *mut libc::c_char,
                            libc::c_int,
                            *mut libc::c_char,
                        ) -> (),
                ),
                win as *mut libc::c_char,
                0 as libc::c_int,
            );
            current_block_71 = 11052029508375673978;
        }
        27 | 7 => {
            if !(!((*wdata).group).is_null() && (*wdata).onblank == 0
                && !((*(*flayer).l_bottom).l_data as *mut win).is_null()
                && (*((*(*flayer).l_bottom).l_data as *mut win)).w_type
                    == 3 as libc::c_int)
            {
                let mut fnumber: libc::c_int = if (*wdata).onblank != 0 {
                    (*(*wdata).fore).w_number
                } else {
                    -(1 as libc::c_int)
                };
                glist_abort();
                display = cd;
                if fnumber >= 0 as libc::c_int {
                    SwitchWindow(fnumber);
                }
                *len = 0 as libc::c_int;
            }
            current_block_71 = 11052029508375673978;
        }
        _ => {
            if ch as libc::c_int >= '0' as i32 && ch as libc::c_int <= '9' as i32 {
                let mut row: *mut ListRow = (*ldata).root;
                while !row.is_null() {
                    let mut w: *mut win = (*row).data as *mut win;
                    if (*w).w_number == ch as libc::c_int - '0' as i32 {
                        let mut old: *mut ListRow = (*ldata).selected;
                        if old == row {
                            break;
                        }
                        (*ldata).selected = row;
                        if (*(*ldata).selected).y == -(1 as libc::c_int) {
                            (*ldata).top = row;
                            glist_display_all(ldata);
                        } else {
                            ((*(*ldata).list_fn).gl_printrow).unwrap()(ldata, old);
                            ((*(*ldata).list_fn).gl_printrow)
                                .unwrap()(ldata, (*ldata).selected);
                            (*flayer).l_y = (*(*ldata).selected).y;
                            LGotoPos(flayer, (*flayer).l_x, (*flayer).l_y);
                        }
                        break;
                    } else {
                        row = (*row).next;
                    }
                }
            } else {
                *inp = (*inp).offset(-1);
                *inp;
                *len += 1;
                *len;
                return 0 as libc::c_int;
            }
            current_block_71 = 11052029508375673978;
        }
    }
    match current_block_71 {
        15109860607434291925 => {
            if !((*wdata).group).is_null() {
                if !((*(*wdata).group).w_group).is_null() {
                    let mut g: *mut win = (*(*wdata).group).w_group;
                    glist_abort();
                    display = cd;
                    SetForeWindow(g);
                    *len = 0 as libc::c_int;
                } else {
                    let mut order_0: libc::c_int = (*wdata).order
                        | (if (*wdata).nested != 0 {
                            2 as libc::c_int
                        } else {
                            0 as libc::c_int
                        });
                    glist_abort();
                    display = cd;
                    display_windows(1 as libc::c_int, order_0, 0 as *mut win);
                    *len = 0 as libc::c_int;
                }
            }
        }
        _ => {}
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn gl_Window_freerow(
    mut ldata: *mut ListData,
    mut row: *mut ListRow,
) -> libc::c_int {
    return 0 as libc::c_int;
}
unsafe extern "C" fn gl_Window_free(mut ldata: *mut ListData) -> libc::c_int {
    if ((*ldata).data).is_null() {
        abort();
    } else {
        free((*ldata).data);
    }
    (*ldata).data = 0 as *mut libc::c_void;
    return 0 as libc::c_int;
}
unsafe extern "C" fn gl_Window_match(
    mut ldata: *mut ListData,
    mut row: *mut ListRow,
    mut needle: *const libc::c_char,
) -> libc::c_int {
    let mut w: *mut win = (*row).data as *mut win;
    if !(InStr((*w).w_title, needle)).is_null() {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
static mut gl_Window: GenericList = unsafe {
    {
        let mut init = GenericList {
            gl_printheader: Some(
                gl_Window_header as unsafe extern "C" fn(*mut ListData) -> libc::c_int,
            ),
            gl_printfooter: Some(
                gl_Window_footer as unsafe extern "C" fn(*mut ListData) -> libc::c_int,
            ),
            gl_printrow: Some(
                gl_Window_row
                    as unsafe extern "C" fn(*mut ListData, *mut ListRow) -> libc::c_int,
            ),
            gl_pinput: Some(
                gl_Window_input
                    as unsafe extern "C" fn(
                        *mut ListData,
                        *mut *mut libc::c_char,
                        *mut libc::c_int,
                    ) -> libc::c_int,
            ),
            gl_freerow: Some(
                gl_Window_freerow
                    as unsafe extern "C" fn(*mut ListData, *mut ListRow) -> libc::c_int,
            ),
            gl_free: Some(
                gl_Window_free as unsafe extern "C" fn(*mut ListData) -> libc::c_int,
            ),
            gl_matchrow: Some(
                gl_Window_match
                    as unsafe extern "C" fn(
                        *mut ListData,
                        *mut ListRow,
                        *const libc::c_char,
                    ) -> libc::c_int,
            ),
        };
        init
    }
};
pub unsafe extern "C" fn display_windows(
    mut onblank: libc::c_int,
    mut order: libc::c_int,
    mut group: *mut win,
) {
    let mut p: *mut win = 0 as *mut win;
    let mut ldata: *mut ListData = 0 as *mut ListData;
    let mut wdata: *mut gl_Window_Data = 0 as *mut gl_Window_Data;
    if (*flayer).l_width < 10 as libc::c_int || (*flayer).l_height < 6 as libc::c_int {
        LMsg(
            0 as libc::c_int,
            b"Window size too small for window list page\0" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    if !group.is_null() {
        onblank = 0 as libc::c_int;
    }
    if onblank != 0 {
        if display.is_null() {
            LMsg(
                0 as libc::c_int,
                b"windowlist -b: display required\0" as *const u8 as *const libc::c_char,
            );
            return;
        }
        p = (*display).d_fore;
        if !p.is_null() {
            SetForeWindow(0 as *mut win);
            if !((*p).w_group).is_null() {
                (*display).d_fore = (*p).w_group;
                (*flayer)
                    .l_data = (*p).w_group as *mut libc::c_char as *mut libc::c_void;
            }
            Activate(0 as libc::c_int);
        }
        if (*flayer).l_width < 10 as libc::c_int || (*flayer).l_height < 6 as libc::c_int
        {
            LMsg(
                0 as libc::c_int,
                b"Window size too small for window list page\0" as *const u8
                    as *const libc::c_char,
            );
            return;
        }
    } else {
        p = (*(*flayer).l_bottom).l_data as *mut win;
    }
    if group.is_null() && !p.is_null() {
        group = (*p).w_group;
    }
    ldata = glist_display(&mut gl_Window, ListID.as_mut_ptr());
    if ldata.is_null() {
        if onblank != 0 && !p.is_null() {
            SetForeWindow(p);
            Activate(1 as libc::c_int);
        }
        return;
    }
    wdata = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<gl_Window_Data>() as libc::c_ulong,
    ) as *mut gl_Window_Data;
    (*wdata).group = group;
    (*wdata).order = order & !(2 as libc::c_int);
    (*wdata).nested = (order & 2 as libc::c_int != 0) as libc::c_int;
    (*wdata).onblank = onblank;
    (*wdata).fore = windows;
    while !((*wdata).fore).is_null() && (*(*wdata).fore).w_group != group {
        (*wdata).fore = (*(*wdata).fore).w_next;
    }
    (*ldata).data = wdata as *mut libc::c_void;
    gl_Window_rebuild(ldata);
}
unsafe extern "C" fn WListUpdate(mut p: *mut win, mut ldata: *mut ListData) {
    let mut wdata: *mut gl_Window_Data = (*ldata).data as *mut gl_Window_Data;
    let mut row: *mut ListRow = 0 as *mut ListRow;
    let mut rbefore: *mut ListRow = 0 as *mut ListRow;
    let mut before: *mut win = 0 as *mut win;
    let mut d: libc::c_int = 0 as libc::c_int;
    let mut sel: libc::c_int = 0 as libc::c_int;
    if p.is_null() {
        if !((*ldata).selected).is_null() {
            (*wdata).fore = (*(*ldata).selected).data as *mut win;
        }
        glist_remove_rows(ldata);
        gl_Window_rebuild(ldata);
        return;
    }
    d = 1 as libc::c_int;
    if (*wdata).order == 0 as libc::c_int || (*wdata).order == 1 as libc::c_int {
        if (*p).w_group != (*wdata).group {
            if (*wdata).nested == 0 {
                d = 0 as libc::c_int;
            } else {
                d = window_ancestor((*wdata).group, p);
            }
        }
    }
    if d == 0 {
        if gl_Window_remove(ldata, p) != 0 {
            glist_display_all(ldata);
        }
        return;
    }
    before = 0 as *mut win;
    if (*wdata).order == 1 as libc::c_int {
        if windows != p {
            before = windows;
            while !before.is_null() {
                if (*before).w_next == p {
                    break;
                }
                before = (*before).w_next;
            }
        }
    } else if (*wdata).order == 0 as libc::c_int {
        if (*p).w_number != 0 as libc::c_int {
            let mut w: *mut *mut win = wtab
                .offset((*p).w_number as isize)
                .offset(-(1 as libc::c_int as isize));
            while w >= wtab {
                if !(*w).is_null() && (**w).w_group == (*wdata).group {
                    before = *w;
                    break;
                } else {
                    w = w.offset(-1);
                    w;
                }
            }
        }
    }
    if !before.is_null() {
        rbefore = gl_Window_findrow(ldata, before);
    } else if (*wdata).nested != 0 && !((*p).w_group).is_null() {
        rbefore = gl_Window_findrow(ldata, (*p).w_group);
    } else {
        rbefore = 0 as *mut ListRow;
    }
    row = gl_Window_findrow(ldata, p);
    if !row.is_null() {
        if (*row).prev != rbefore {
            sel = ((*(*ldata).selected).data == p as *mut libc::c_void) as libc::c_int;
            gl_Window_remove(ldata, p);
        } else {
            p = 0 as *mut win;
        }
    }
    if !p.is_null() {
        row = glist_add_row(ldata, p as *mut libc::c_void, rbefore);
        if sel != 0 {
            (*ldata).selected = row;
        }
    }
    glist_display_all(ldata);
}
pub unsafe extern "C" fn WListUpdatecv(mut cv: *mut canvas, mut p: *mut win) {
    let mut ldata: *mut ListData = 0 as *mut ListData;
    let mut wdata: *mut gl_Window_Data = 0 as *mut gl_Window_Data;
    if (*(*cv).c_layer).l_layfn != &mut ListLf as *mut LayFuncs {
        return;
    }
    ldata = (*(*cv).c_layer).l_data as *mut ListData;
    if (*ldata).name != ListID.as_mut_ptr() as *const libc::c_char {
        return;
    }
    wdata = (*ldata).data as *mut gl_Window_Data;
    let mut olddisplay: *mut display = display;
    let mut oldflayer: *mut layer = flayer;
    let mut l: *mut layer = (*cv).c_layer;
    let mut cvlist: *mut canvas = (*l).l_cvlist;
    let mut cvlnext: *mut canvas = (*cv).c_lnext;
    flayer = l;
    (*l).l_cvlist = cv;
    (*cv).c_lnext = 0 as *mut canvas;
    WListUpdate(p, ldata);
    flayer = oldflayer;
    (*l).l_cvlist = cvlist;
    (*cv).c_lnext = cvlnext;
    display = olddisplay;
}
pub unsafe extern "C" fn WListLinkChanged() {
    let mut olddisplay: *mut display = display;
    let mut cv: *mut canvas = 0 as *mut canvas;
    let mut ldata: *mut ListData = 0 as *mut ListData;
    let mut wdata: *mut gl_Window_Data = 0 as *mut gl_Window_Data;
    display = displays;
    while !display.is_null() {
        cv = (*display).d_cvlist;
        while !cv.is_null() {
            if !(((*cv).c_layer).is_null()
                || (*(*cv).c_layer).l_layfn != &mut ListLf as *mut LayFuncs)
            {
                ldata = (*(*cv).c_layer).l_data as *mut ListData;
                if !((*ldata).name != ListID.as_mut_ptr() as *const libc::c_char) {
                    wdata = (*ldata).data as *mut gl_Window_Data;
                    if !((*wdata).order & 1 as libc::c_int == 0) {
                        let mut olddisplay_0: *mut display = display;
                        let mut oldflayer: *mut layer = flayer;
                        let mut l: *mut layer = (*cv).c_layer;
                        let mut cvlist: *mut canvas = (*l).l_cvlist;
                        let mut cvlnext: *mut canvas = (*cv).c_lnext;
                        flayer = l;
                        (*l).l_cvlist = cv;
                        (*cv).c_lnext = 0 as *mut canvas;
                        WListUpdate(0 as *mut win, ldata);
                        flayer = oldflayer;
                        (*l).l_cvlist = cvlist;
                        (*cv).c_lnext = cvlnext;
                        display = olddisplay_0;
                    }
                }
            }
            cv = (*cv).c_next;
        }
        display = (*display).d_next;
    }
    display = olddisplay;
}
