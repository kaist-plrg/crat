use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type logfile;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn getc(__stream: *mut FILE) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn SetCharsets(_: *mut win, _: *mut libc::c_char);
    fn secfopen(_: *mut libc::c_char, _: *mut libc::c_char) -> *mut FILE;
    fn Resize_obuf();
    fn ExitOverlayPage();
    static mut null: *mut libc::c_uchar;
    static mut display: *mut display;
    static mut displays: *mut display;
    static mut flayer: *mut layer;
    static mut screenencodings: *mut libc::c_char;
    static mut cjkwidth: libc::c_int;
}
pub type __int32_t = libc::c_int;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
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
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
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
pub struct recodetab {
    pub tab: *mut [libc::c_ushort; 2],
    pub flags: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct encoding {
    pub name: *mut libc::c_char,
    pub charsets: *mut libc::c_char,
    pub deffont: libc::c_int,
    pub usegr: libc::c_int,
    pub noc1: libc::c_int,
    pub fontlist: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct interval {
    pub first: libc::c_int,
    pub last: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct combchar {
    pub c1: libc::c_uint,
    pub c2: libc::c_uint,
    pub next: libc::c_uint,
    pub prev: libc::c_uint,
}
pub static mut encodings: [encoding; 21] = [
    {
        let mut init = encoding {
            name: b"C\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            charsets: 0 as *const libc::c_char as *mut libc::c_char,
            deffont: 0 as libc::c_int,
            usegr: 0 as libc::c_int,
            noc1: 0 as libc::c_int,
            fontlist: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding {
            name: b"eucJP\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            charsets: b"B\x02I\x0401\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            deffont: 0 as libc::c_int,
            usegr: 1 as libc::c_int,
            noc1: 0 as libc::c_int,
            fontlist: b"\x02\x04I\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding {
            name: b"SJIS\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            charsets: b"BIBB01\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            deffont: 0 as libc::c_int,
            usegr: 1 as libc::c_int,
            noc1: 1 as libc::c_int,
            fontlist: b"\x02I\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding {
            name: b"eucKR\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            charsets: b"B\x03BB01\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            deffont: 0 as libc::c_int,
            usegr: 1 as libc::c_int,
            noc1: 0 as libc::c_int,
            fontlist: b"\x03\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding {
            name: b"eucCN\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            charsets: b"B\x01BB01\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            deffont: 0 as libc::c_int,
            usegr: 1 as libc::c_int,
            noc1: 0 as libc::c_int,
            fontlist: b"\x01\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding {
            name: b"Big5\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            charsets: b"B\x18BB01\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            deffont: 0 as libc::c_int,
            usegr: 1 as libc::c_int,
            noc1: 0 as libc::c_int,
            fontlist: b"\x18\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding {
            name: b"KOI8-R\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            charsets: 0 as *const libc::c_char as *mut libc::c_char,
            deffont: 0x80 as libc::c_int | '!' as i32,
            usegr: 0 as libc::c_int,
            noc1: 1 as libc::c_int,
            fontlist: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding {
            name: b"CP1251\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            charsets: 0 as *const libc::c_char as *mut libc::c_char,
            deffont: 0x80 as libc::c_int | '?' as i32,
            usegr: 0 as libc::c_int,
            noc1: 1 as libc::c_int,
            fontlist: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding {
            name: b"UTF-8\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            charsets: 0 as *const libc::c_char as *mut libc::c_char,
            deffont: -(1 as libc::c_int),
            usegr: 0 as libc::c_int,
            noc1: 0 as libc::c_int,
            fontlist: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding {
            name: b"ISO8859-2\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            charsets: 0 as *const libc::c_char as *mut libc::c_char,
            deffont: 0x80 as libc::c_int | 'B' as i32,
            usegr: 0 as libc::c_int,
            noc1: 0 as libc::c_int,
            fontlist: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding {
            name: b"ISO8859-3\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            charsets: 0 as *const libc::c_char as *mut libc::c_char,
            deffont: 0x80 as libc::c_int | 'C' as i32,
            usegr: 0 as libc::c_int,
            noc1: 0 as libc::c_int,
            fontlist: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding {
            name: b"ISO8859-4\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            charsets: 0 as *const libc::c_char as *mut libc::c_char,
            deffont: 0x80 as libc::c_int | 'D' as i32,
            usegr: 0 as libc::c_int,
            noc1: 0 as libc::c_int,
            fontlist: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding {
            name: b"ISO8859-5\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            charsets: 0 as *const libc::c_char as *mut libc::c_char,
            deffont: 0x80 as libc::c_int | 'L' as i32,
            usegr: 0 as libc::c_int,
            noc1: 0 as libc::c_int,
            fontlist: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding {
            name: b"ISO8859-6\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            charsets: 0 as *const libc::c_char as *mut libc::c_char,
            deffont: 0x80 as libc::c_int | 'G' as i32,
            usegr: 0 as libc::c_int,
            noc1: 0 as libc::c_int,
            fontlist: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding {
            name: b"ISO8859-7\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            charsets: 0 as *const libc::c_char as *mut libc::c_char,
            deffont: 0x80 as libc::c_int | 'F' as i32,
            usegr: 0 as libc::c_int,
            noc1: 0 as libc::c_int,
            fontlist: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding {
            name: b"ISO8859-8\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            charsets: 0 as *const libc::c_char as *mut libc::c_char,
            deffont: 0x80 as libc::c_int | 'H' as i32,
            usegr: 0 as libc::c_int,
            noc1: 0 as libc::c_int,
            fontlist: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding {
            name: b"ISO8859-9\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            charsets: 0 as *const libc::c_char as *mut libc::c_char,
            deffont: 0x80 as libc::c_int | 'M' as i32,
            usegr: 0 as libc::c_int,
            noc1: 0 as libc::c_int,
            fontlist: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding {
            name: b"ISO8859-10\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            charsets: 0 as *const libc::c_char as *mut libc::c_char,
            deffont: 0x80 as libc::c_int | 'V' as i32,
            usegr: 0 as libc::c_int,
            noc1: 0 as libc::c_int,
            fontlist: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding {
            name: b"ISO8859-15\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            charsets: 0 as *const libc::c_char as *mut libc::c_char,
            deffont: 0x80 as libc::c_int | 'b' as i32,
            usegr: 0 as libc::c_int,
            noc1: 0 as libc::c_int,
            fontlist: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding {
            name: b"jis\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            charsets: 0 as *const libc::c_char as *mut libc::c_char,
            deffont: 0 as libc::c_int,
            usegr: 0 as libc::c_int,
            noc1: 0 as libc::c_int,
            fontlist: b"\x02\x04I\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = encoding {
            name: b"GBK\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            charsets: b"B\x19BB01\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            deffont: 0x80 as libc::c_int | 'b' as i32,
            usegr: 1 as libc::c_int,
            noc1: 1 as libc::c_int,
            fontlist: b"\x19\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
];
static mut builtin_tabs: [[libc::c_ushort; 2]; 171] = [
    [0x30 as libc::c_int as libc::c_ushort, 0 as libc::c_int as libc::c_ushort],
    [0x5f as libc::c_int as libc::c_ushort, 0x25ae as libc::c_int as libc::c_ushort],
    [0x60 as libc::c_int as libc::c_ushort, 0x25c6 as libc::c_int as libc::c_ushort],
    [0x61 as libc::c_int as libc::c_ushort, 0x2592 as libc::c_int as libc::c_ushort],
    [0x62 as libc::c_int as libc::c_ushort, 0x2409 as libc::c_int as libc::c_ushort],
    [0x63 as libc::c_int as libc::c_ushort, 0x240c as libc::c_int as libc::c_ushort],
    [0x64 as libc::c_int as libc::c_ushort, 0x240d as libc::c_int as libc::c_ushort],
    [0x65 as libc::c_int as libc::c_ushort, 0x240a as libc::c_int as libc::c_ushort],
    [0x66 as libc::c_int as libc::c_ushort, 0xb0 as libc::c_int as libc::c_ushort],
    [0x67 as libc::c_int as libc::c_ushort, 0xb1 as libc::c_int as libc::c_ushort],
    [0x68 as libc::c_int as libc::c_ushort, 0x2424 as libc::c_int as libc::c_ushort],
    [0x69 as libc::c_int as libc::c_ushort, 0x240b as libc::c_int as libc::c_ushort],
    [0x6a as libc::c_int as libc::c_ushort, 0x2518 as libc::c_int as libc::c_ushort],
    [0x6b as libc::c_int as libc::c_ushort, 0x2510 as libc::c_int as libc::c_ushort],
    [0x6c as libc::c_int as libc::c_ushort, 0x250c as libc::c_int as libc::c_ushort],
    [0x6d as libc::c_int as libc::c_ushort, 0x2514 as libc::c_int as libc::c_ushort],
    [0x6e as libc::c_int as libc::c_ushort, 0x253c as libc::c_int as libc::c_ushort],
    [0x6f as libc::c_int as libc::c_ushort, 0x23ba as libc::c_int as libc::c_ushort],
    [0x70 as libc::c_int as libc::c_ushort, 0x23bb as libc::c_int as libc::c_ushort],
    [0x71 as libc::c_int as libc::c_ushort, 0x2500 as libc::c_int as libc::c_ushort],
    [0x72 as libc::c_int as libc::c_ushort, 0x23bc as libc::c_int as libc::c_ushort],
    [0x73 as libc::c_int as libc::c_ushort, 0x23bd as libc::c_int as libc::c_ushort],
    [0x74 as libc::c_int as libc::c_ushort, 0x251c as libc::c_int as libc::c_ushort],
    [0x75 as libc::c_int as libc::c_ushort, 0x2524 as libc::c_int as libc::c_ushort],
    [0x76 as libc::c_int as libc::c_ushort, 0x2534 as libc::c_int as libc::c_ushort],
    [0x77 as libc::c_int as libc::c_ushort, 0x252c as libc::c_int as libc::c_ushort],
    [0x78 as libc::c_int as libc::c_ushort, 0x2502 as libc::c_int as libc::c_ushort],
    [0x79 as libc::c_int as libc::c_ushort, 0x2264 as libc::c_int as libc::c_ushort],
    [0x7a as libc::c_int as libc::c_ushort, 0x2265 as libc::c_int as libc::c_ushort],
    [0x7b as libc::c_int as libc::c_ushort, 0x3c0 as libc::c_int as libc::c_ushort],
    [0x7c as libc::c_int as libc::c_ushort, 0x2260 as libc::c_int as libc::c_ushort],
    [0x7d as libc::c_int as libc::c_ushort, 0xa3 as libc::c_int as libc::c_ushort],
    [0x7e as libc::c_int as libc::c_ushort, 0xb7 as libc::c_int as libc::c_ushort],
    [0 as libc::c_int as libc::c_ushort, 0 as libc::c_int as libc::c_ushort],
    [0x34 as libc::c_int as libc::c_ushort, 0 as libc::c_int as libc::c_ushort],
    [0x23 as libc::c_int as libc::c_ushort, 0xa3 as libc::c_int as libc::c_ushort],
    [0x40 as libc::c_int as libc::c_ushort, 0xbe as libc::c_int as libc::c_ushort],
    [0x5b as libc::c_int as libc::c_ushort, 0xff as libc::c_int as libc::c_ushort],
    [0x5c as libc::c_int as libc::c_ushort, 0xbd as libc::c_int as libc::c_ushort],
    [0x5d as libc::c_int as libc::c_ushort, 0x7c as libc::c_int as libc::c_ushort],
    [0x7b as libc::c_int as libc::c_ushort, 0xa8 as libc::c_int as libc::c_ushort],
    [0x7c as libc::c_int as libc::c_ushort, 0x66 as libc::c_int as libc::c_ushort],
    [0x7d as libc::c_int as libc::c_ushort, 0xbc as libc::c_int as libc::c_ushort],
    [0x7e as libc::c_int as libc::c_ushort, 0xb4 as libc::c_int as libc::c_ushort],
    [0 as libc::c_int as libc::c_ushort, 0 as libc::c_int as libc::c_ushort],
    [0x35 as libc::c_int as libc::c_ushort, 0 as libc::c_int as libc::c_ushort],
    [0x5b as libc::c_int as libc::c_ushort, 0xc4 as libc::c_int as libc::c_ushort],
    [0x5c as libc::c_int as libc::c_ushort, 0xd6 as libc::c_int as libc::c_ushort],
    [0x5d as libc::c_int as libc::c_ushort, 0xc5 as libc::c_int as libc::c_ushort],
    [0x5e as libc::c_int as libc::c_ushort, 0xdc as libc::c_int as libc::c_ushort],
    [0x60 as libc::c_int as libc::c_ushort, 0xe9 as libc::c_int as libc::c_ushort],
    [0x7b as libc::c_int as libc::c_ushort, 0xe4 as libc::c_int as libc::c_ushort],
    [0x7c as libc::c_int as libc::c_ushort, 0xf6 as libc::c_int as libc::c_ushort],
    [0x7d as libc::c_int as libc::c_ushort, 0xe5 as libc::c_int as libc::c_ushort],
    [0x7e as libc::c_int as libc::c_ushort, 0xfc as libc::c_int as libc::c_ushort],
    [0 as libc::c_int as libc::c_ushort, 0 as libc::c_int as libc::c_ushort],
    [0x36 as libc::c_int as libc::c_ushort, 0 as libc::c_int as libc::c_ushort],
    [0x40 as libc::c_int as libc::c_ushort, 0xc4 as libc::c_int as libc::c_ushort],
    [0x5b as libc::c_int as libc::c_ushort, 0xc6 as libc::c_int as libc::c_ushort],
    [0x5c as libc::c_int as libc::c_ushort, 0xd8 as libc::c_int as libc::c_ushort],
    [0x5d as libc::c_int as libc::c_ushort, 0xc5 as libc::c_int as libc::c_ushort],
    [0x5e as libc::c_int as libc::c_ushort, 0xdc as libc::c_int as libc::c_ushort],
    [0x60 as libc::c_int as libc::c_ushort, 0xe4 as libc::c_int as libc::c_ushort],
    [0x7b as libc::c_int as libc::c_ushort, 0xe6 as libc::c_int as libc::c_ushort],
    [0x7c as libc::c_int as libc::c_ushort, 0xf8 as libc::c_int as libc::c_ushort],
    [0x7d as libc::c_int as libc::c_ushort, 0xe5 as libc::c_int as libc::c_ushort],
    [0x7e as libc::c_int as libc::c_ushort, 0xfc as libc::c_int as libc::c_ushort],
    [0 as libc::c_int as libc::c_ushort, 0 as libc::c_int as libc::c_ushort],
    [0x37 as libc::c_int as libc::c_ushort, 0 as libc::c_int as libc::c_ushort],
    [0x40 as libc::c_int as libc::c_ushort, 0xc9 as libc::c_int as libc::c_ushort],
    [0x5b as libc::c_int as libc::c_ushort, 0xc4 as libc::c_int as libc::c_ushort],
    [0x5c as libc::c_int as libc::c_ushort, 0xd6 as libc::c_int as libc::c_ushort],
    [0x5d as libc::c_int as libc::c_ushort, 0xc5 as libc::c_int as libc::c_ushort],
    [0x5e as libc::c_int as libc::c_ushort, 0xdc as libc::c_int as libc::c_ushort],
    [0x60 as libc::c_int as libc::c_ushort, 0xe9 as libc::c_int as libc::c_ushort],
    [0x7b as libc::c_int as libc::c_ushort, 0xe4 as libc::c_int as libc::c_ushort],
    [0x7c as libc::c_int as libc::c_ushort, 0xf6 as libc::c_int as libc::c_ushort],
    [0x7d as libc::c_int as libc::c_ushort, 0xe5 as libc::c_int as libc::c_ushort],
    [0x7e as libc::c_int as libc::c_ushort, 0xfc as libc::c_int as libc::c_ushort],
    [0 as libc::c_int as libc::c_ushort, 0 as libc::c_int as libc::c_ushort],
    [0x3d as libc::c_int as libc::c_ushort, 0 as libc::c_int as libc::c_ushort],
    [0x23 as libc::c_int as libc::c_ushort, 0xf9 as libc::c_int as libc::c_ushort],
    [0x40 as libc::c_int as libc::c_ushort, 0xe0 as libc::c_int as libc::c_ushort],
    [0x5b as libc::c_int as libc::c_ushort, 0xe9 as libc::c_int as libc::c_ushort],
    [0x5c as libc::c_int as libc::c_ushort, 0xe7 as libc::c_int as libc::c_ushort],
    [0x5d as libc::c_int as libc::c_ushort, 0xea as libc::c_int as libc::c_ushort],
    [0x5e as libc::c_int as libc::c_ushort, 0xee as libc::c_int as libc::c_ushort],
    [0x5f as libc::c_int as libc::c_ushort, 0xe8 as libc::c_int as libc::c_ushort],
    [0x60 as libc::c_int as libc::c_ushort, 0xf4 as libc::c_int as libc::c_ushort],
    [0x7b as libc::c_int as libc::c_ushort, 0xe4 as libc::c_int as libc::c_ushort],
    [0x7c as libc::c_int as libc::c_ushort, 0xf6 as libc::c_int as libc::c_ushort],
    [0x7d as libc::c_int as libc::c_ushort, 0xfc as libc::c_int as libc::c_ushort],
    [0x7e as libc::c_int as libc::c_ushort, 0xfb as libc::c_int as libc::c_ushort],
    [0 as libc::c_int as libc::c_ushort, 0 as libc::c_int as libc::c_ushort],
    [0x41 as libc::c_int as libc::c_ushort, 0 as libc::c_int as libc::c_ushort],
    [0x23 as libc::c_int as libc::c_ushort, 0xa3 as libc::c_int as libc::c_ushort],
    [0 as libc::c_int as libc::c_ushort, 0 as libc::c_int as libc::c_ushort],
    [0x4b as libc::c_int as libc::c_ushort, 0 as libc::c_int as libc::c_ushort],
    [0x40 as libc::c_int as libc::c_ushort, 0xa7 as libc::c_int as libc::c_ushort],
    [0x5b as libc::c_int as libc::c_ushort, 0xc4 as libc::c_int as libc::c_ushort],
    [0x5c as libc::c_int as libc::c_ushort, 0xd6 as libc::c_int as libc::c_ushort],
    [0x5d as libc::c_int as libc::c_ushort, 0xdc as libc::c_int as libc::c_ushort],
    [0x7b as libc::c_int as libc::c_ushort, 0xe4 as libc::c_int as libc::c_ushort],
    [0x7c as libc::c_int as libc::c_ushort, 0xf6 as libc::c_int as libc::c_ushort],
    [0x7d as libc::c_int as libc::c_ushort, 0xfc as libc::c_int as libc::c_ushort],
    [0x7e as libc::c_int as libc::c_ushort, 0xdf as libc::c_int as libc::c_ushort],
    [0 as libc::c_int as libc::c_ushort, 0 as libc::c_int as libc::c_ushort],
    [0x51 as libc::c_int as libc::c_ushort, 0 as libc::c_int as libc::c_ushort],
    [0x40 as libc::c_int as libc::c_ushort, 0xe0 as libc::c_int as libc::c_ushort],
    [0x5b as libc::c_int as libc::c_ushort, 0xe2 as libc::c_int as libc::c_ushort],
    [0x5c as libc::c_int as libc::c_ushort, 0xe7 as libc::c_int as libc::c_ushort],
    [0x5d as libc::c_int as libc::c_ushort, 0xea as libc::c_int as libc::c_ushort],
    [0x5e as libc::c_int as libc::c_ushort, 0xee as libc::c_int as libc::c_ushort],
    [0x60 as libc::c_int as libc::c_ushort, 0xf4 as libc::c_int as libc::c_ushort],
    [0x7b as libc::c_int as libc::c_ushort, 0xe9 as libc::c_int as libc::c_ushort],
    [0x7c as libc::c_int as libc::c_ushort, 0xf9 as libc::c_int as libc::c_ushort],
    [0x7d as libc::c_int as libc::c_ushort, 0xe8 as libc::c_int as libc::c_ushort],
    [0x7e as libc::c_int as libc::c_ushort, 0xfb as libc::c_int as libc::c_ushort],
    [0 as libc::c_int as libc::c_ushort, 0 as libc::c_int as libc::c_ushort],
    [0x52 as libc::c_int as libc::c_ushort, 0 as libc::c_int as libc::c_ushort],
    [0x23 as libc::c_int as libc::c_ushort, 0xa3 as libc::c_int as libc::c_ushort],
    [0x40 as libc::c_int as libc::c_ushort, 0xe0 as libc::c_int as libc::c_ushort],
    [0x5b as libc::c_int as libc::c_ushort, 0xb0 as libc::c_int as libc::c_ushort],
    [0x5c as libc::c_int as libc::c_ushort, 0xe7 as libc::c_int as libc::c_ushort],
    [0x5d as libc::c_int as libc::c_ushort, 0xa7 as libc::c_int as libc::c_ushort],
    [0x7b as libc::c_int as libc::c_ushort, 0xe9 as libc::c_int as libc::c_ushort],
    [0x7c as libc::c_int as libc::c_ushort, 0xf9 as libc::c_int as libc::c_ushort],
    [0x7d as libc::c_int as libc::c_ushort, 0xe8 as libc::c_int as libc::c_ushort],
    [0x7e as libc::c_int as libc::c_ushort, 0xa8 as libc::c_int as libc::c_ushort],
    [0 as libc::c_int as libc::c_ushort, 0 as libc::c_int as libc::c_ushort],
    [0x59 as libc::c_int as libc::c_ushort, 0 as libc::c_int as libc::c_ushort],
    [0x23 as libc::c_int as libc::c_ushort, 0xa3 as libc::c_int as libc::c_ushort],
    [0x40 as libc::c_int as libc::c_ushort, 0xa7 as libc::c_int as libc::c_ushort],
    [0x5b as libc::c_int as libc::c_ushort, 0xb0 as libc::c_int as libc::c_ushort],
    [0x5c as libc::c_int as libc::c_ushort, 0xe7 as libc::c_int as libc::c_ushort],
    [0x5d as libc::c_int as libc::c_ushort, 0xe9 as libc::c_int as libc::c_ushort],
    [0x60 as libc::c_int as libc::c_ushort, 0xf9 as libc::c_int as libc::c_ushort],
    [0x7b as libc::c_int as libc::c_ushort, 0xe0 as libc::c_int as libc::c_ushort],
    [0x7c as libc::c_int as libc::c_ushort, 0xf2 as libc::c_int as libc::c_ushort],
    [0x7d as libc::c_int as libc::c_ushort, 0xe8 as libc::c_int as libc::c_ushort],
    [0x7e as libc::c_int as libc::c_ushort, 0xec as libc::c_int as libc::c_ushort],
    [0 as libc::c_int as libc::c_ushort, 0 as libc::c_int as libc::c_ushort],
    [0x5a as libc::c_int as libc::c_ushort, 0 as libc::c_int as libc::c_ushort],
    [0x23 as libc::c_int as libc::c_ushort, 0xa3 as libc::c_int as libc::c_ushort],
    [0x40 as libc::c_int as libc::c_ushort, 0xa7 as libc::c_int as libc::c_ushort],
    [0x5b as libc::c_int as libc::c_ushort, 0xa1 as libc::c_int as libc::c_ushort],
    [0x5c as libc::c_int as libc::c_ushort, 0xd1 as libc::c_int as libc::c_ushort],
    [0x5d as libc::c_int as libc::c_ushort, 0xbf as libc::c_int as libc::c_ushort],
    [0x7b as libc::c_int as libc::c_ushort, 0xb0 as libc::c_int as libc::c_ushort],
    [0x7c as libc::c_int as libc::c_ushort, 0xf1 as libc::c_int as libc::c_ushort],
    [0x7d as libc::c_int as libc::c_ushort, 0xe7 as libc::c_int as libc::c_ushort],
    [0 as libc::c_int as libc::c_ushort, 0 as libc::c_int as libc::c_ushort],
    [0xe2 as libc::c_int as libc::c_ushort, 0 as libc::c_int as libc::c_ushort],
    [0xa4 as libc::c_int as libc::c_ushort, 0x20ac as libc::c_int as libc::c_ushort],
    [0xa6 as libc::c_int as libc::c_ushort, 0x160 as libc::c_int as libc::c_ushort],
    [0xa8 as libc::c_int as libc::c_ushort, 0x161 as libc::c_int as libc::c_ushort],
    [0xb4 as libc::c_int as libc::c_ushort, 0x17d as libc::c_int as libc::c_ushort],
    [0xb8 as libc::c_int as libc::c_ushort, 0x17e as libc::c_int as libc::c_ushort],
    [0xbc as libc::c_int as libc::c_ushort, 0x152 as libc::c_int as libc::c_ushort],
    [0xbd as libc::c_int as libc::c_ushort, 0x153 as libc::c_int as libc::c_ushort],
    [0xbe as libc::c_int as libc::c_ushort, 0x178 as libc::c_int as libc::c_ushort],
    [0 as libc::c_int as libc::c_ushort, 0 as libc::c_int as libc::c_ushort],
    [0x4a as libc::c_int as libc::c_ushort, 0 as libc::c_int as libc::c_ushort],
    [0x5c as libc::c_int as libc::c_ushort, 0xa5 as libc::c_int as libc::c_ushort],
    [0x7e as libc::c_int as libc::c_ushort, 0x203e as libc::c_int as libc::c_ushort],
    [0 as libc::c_int as libc::c_ushort, 0 as libc::c_int as libc::c_ushort],
    [0x49 as libc::c_int as libc::c_ushort, 0 as libc::c_int as libc::c_ushort],
    [0x21 as libc::c_int as libc::c_ushort, 0xff61 as libc::c_int as libc::c_ushort],
    [
        (0x5f as libc::c_int | 0x8000 as libc::c_int) as libc::c_ushort,
        0xff9f as libc::c_int as libc::c_ushort,
    ],
    [0 as libc::c_int as libc::c_ushort, 0 as libc::c_int as libc::c_ushort],
    [0 as libc::c_int as libc::c_ushort, 0 as libc::c_int as libc::c_ushort],
];
static mut recodetabs: [recodetab; 256] = [recodetab {
    tab: 0 as *const [libc::c_ushort; 2] as *mut [libc::c_ushort; 2],
    flags: 0,
}; 256];
pub unsafe extern "C" fn InitBuiltinTabs() {
    let mut p: *mut [libc::c_ushort; 2] = 0 as *mut [libc::c_ushort; 2];
    p = builtin_tabs.as_mut_ptr();
    while (*p)[0 as libc::c_int as usize] != 0 {
        recodetabs[(*p)[0 as libc::c_int as usize] as usize].flags = 2 as libc::c_int;
        recodetabs[(*p)[0 as libc::c_int as usize] as usize]
            .tab = p.offset(1 as libc::c_int as isize);
        p = p.offset(1);
        p;
        while (*p)[0 as libc::c_int as usize] != 0 {
            p = p.offset(1);
            p;
        }
        p = p.offset(1);
        p;
    }
}
unsafe extern "C" fn recode_char(
    mut c: libc::c_int,
    mut to_utf: libc::c_int,
    mut font: libc::c_int,
) -> libc::c_int {
    let mut f: libc::c_int = 0;
    let mut p: *mut [libc::c_ushort; 2] = 0 as *mut [libc::c_ushort; 2];
    if to_utf != 0 {
        if c < 256 as libc::c_int {
            return c;
        }
        f = c >> 8 as libc::c_int & 0xff as libc::c_int;
        c &= 0xff as libc::c_int;
        match f {
            67 => {
                f ^= 'C' as i32 ^ '5' as i32;
            }
            69 => {
                f ^= 'E' as i32 ^ '6' as i32;
            }
            72 => {
                f ^= 'H' as i32 ^ '7' as i32;
            }
            _ => {}
        }
        p = recodetabs[f as usize].tab;
        if p.is_null() && recodetabs[f as usize].flags == 0 as libc::c_int {
            LoadFontTranslation(f, 0 as *mut libc::c_char);
            p = recodetabs[f as usize].tab;
        }
        if !p.is_null() {
            while (*p)[0 as libc::c_int as usize] != 0 {
                if (*p.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize]
                    as libc::c_int & 0x8000 as libc::c_int != 0
                    && c
                        <= (*p
                            .offset(
                                0 as libc::c_int as isize,
                            ))[0 as libc::c_int as usize] as libc::c_int
                            & 0x7fff as libc::c_int
                    && c
                        >= (*p
                            .offset(
                                -(1 as libc::c_int) as isize,
                            ))[0 as libc::c_int as usize] as libc::c_int
                {
                    return c
                        - (*p
                            .offset(
                                -(1 as libc::c_int) as isize,
                            ))[0 as libc::c_int as usize] as libc::c_int
                        + (*p
                            .offset(
                                -(1 as libc::c_int) as isize,
                            ))[1 as libc::c_int as usize] as libc::c_int;
                }
                if (*p)[0 as libc::c_int as usize] as libc::c_int == c {
                    return (*p)[1 as libc::c_int as usize] as libc::c_int;
                }
                p = p.offset(1);
                p;
            }
        }
        return c & 0xff as libc::c_int;
    }
    if font == -(1 as libc::c_int) {
        if c < 256 as libc::c_int {
            return c;
        }
        font = 32 as libc::c_int;
        while font < 128 as libc::c_int {
            p = recodetabs[font as usize].tab;
            if !p.is_null() {
                while (*p)[1 as libc::c_int as usize] != 0 {
                    if (*p.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize]
                        as libc::c_int & 0x8000 as libc::c_int != 0
                        && c
                            <= (*p
                                .offset(
                                    0 as libc::c_int as isize,
                                ))[1 as libc::c_int as usize] as libc::c_int
                        && c
                            >= (*p
                                .offset(
                                    -(1 as libc::c_int) as isize,
                                ))[1 as libc::c_int as usize] as libc::c_int
                    {
                        return c
                            - (*p
                                .offset(
                                    -(1 as libc::c_int) as isize,
                                ))[1 as libc::c_int as usize] as libc::c_int
                            + (*p
                                .offset(
                                    -(1 as libc::c_int) as isize,
                                ))[0 as libc::c_int as usize] as libc::c_int
                            | font << 8 as libc::c_int;
                    }
                    if (*p)[1 as libc::c_int as usize] as libc::c_int == c {
                        return (*p)[0 as libc::c_int as usize] as libc::c_int
                            | font << 8 as libc::c_int;
                    }
                    p = p.offset(1);
                    p;
                }
            }
            font += 1;
            font;
        }
        return '?' as i32;
    }
    if c < 128 as libc::c_int && font & 128 as libc::c_int != 0 as libc::c_int {
        return c;
    }
    if font >= 32 as libc::c_int {
        p = recodetabs[font as usize].tab;
        if p.is_null() && recodetabs[font as usize].flags == 0 as libc::c_int {
            LoadFontTranslation(font, 0 as *mut libc::c_char);
            p = recodetabs[font as usize].tab;
        }
        if !p.is_null() {
            while (*p)[1 as libc::c_int as usize] != 0 {
                if (*p.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize]
                    as libc::c_int & 0x8000 as libc::c_int != 0
                    && c
                        <= (*p
                            .offset(
                                0 as libc::c_int as isize,
                            ))[1 as libc::c_int as usize] as libc::c_int
                    && c
                        >= (*p
                            .offset(
                                -(1 as libc::c_int) as isize,
                            ))[1 as libc::c_int as usize] as libc::c_int
                {
                    return c
                        - (*p
                            .offset(
                                -(1 as libc::c_int) as isize,
                            ))[1 as libc::c_int as usize] as libc::c_int
                        + (*p
                            .offset(
                                -(1 as libc::c_int) as isize,
                            ))[0 as libc::c_int as usize] as libc::c_int
                        | (if font & 128 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            font << 8 as libc::c_int
                        });
                }
                if (*p)[1 as libc::c_int as usize] as libc::c_int == c {
                    return (*p)[0 as libc::c_int as usize] as libc::c_int
                        | (if font & 128 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            font << 8 as libc::c_int
                        });
                }
                p = p.offset(1);
                p;
            }
        }
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn recode_char_dw(
    mut c: libc::c_int,
    mut c2p: *mut libc::c_int,
    mut to_utf: libc::c_int,
    mut font: libc::c_int,
) -> libc::c_int {
    let mut f: libc::c_int = 0;
    let mut p: *mut [libc::c_ushort; 2] = 0 as *mut [libc::c_ushort; 2];
    if to_utf != 0 {
        f = c >> 8 as libc::c_int & 0xff as libc::c_int;
        c = (c & 255 as libc::c_int) << 8 as libc::c_int | *c2p & 255 as libc::c_int;
        *c2p = 0xffff as libc::c_int;
        p = recodetabs[f as usize].tab;
        if p.is_null() && recodetabs[f as usize].flags == 0 as libc::c_int {
            LoadFontTranslation(f, 0 as *mut libc::c_char);
            p = recodetabs[f as usize].tab;
        }
        if !p.is_null() {
            while (*p)[0 as libc::c_int as usize] != 0 {
                if (*p)[0 as libc::c_int as usize] as libc::c_int == c {
                    if utf8_isdouble((*p)[1 as libc::c_int as usize] as libc::c_int) == 0
                    {
                        *c2p = ' ' as i32;
                    }
                    return (*p)[1 as libc::c_int as usize] as libc::c_int;
                }
                p = p.offset(1);
                p;
            }
        }
        return 0xff1f as libc::c_int;
    }
    if font == -(1 as libc::c_int) {
        font = 0 as libc::c_int;
        while font < 0o30 as libc::c_int {
            p = recodetabs[font as usize].tab;
            if !p.is_null() {
                while (*p)[1 as libc::c_int as usize] != 0 {
                    if (*p)[1 as libc::c_int as usize] as libc::c_int == c {
                        *c2p = (*p)[0 as libc::c_int as usize] as libc::c_int
                            & 255 as libc::c_int | font << 8 as libc::c_int
                            | 0x8000 as libc::c_int;
                        return (*p)[0 as libc::c_int as usize] as libc::c_int
                            >> 8 as libc::c_int | font << 8 as libc::c_int;
                    }
                    p = p.offset(1);
                    p;
                }
            }
            font += 1;
            font;
        }
        *c2p = '?' as i32;
        return '?' as i32;
    }
    if font < 32 as libc::c_int {
        p = recodetabs[font as usize].tab;
        if p.is_null() && recodetabs[font as usize].flags == 0 as libc::c_int {
            LoadFontTranslation(font, 0 as *mut libc::c_char);
            p = recodetabs[font as usize].tab;
        }
        if !p.is_null() {
            while (*p)[1 as libc::c_int as usize] != 0 {
                if (*p)[1 as libc::c_int as usize] as libc::c_int == c {
                    *c2p = (*p)[0 as libc::c_int as usize] as libc::c_int
                        & 255 as libc::c_int | font << 8 as libc::c_int
                        | 0x8000 as libc::c_int;
                    return (*p)[0 as libc::c_int as usize] as libc::c_int
                        >> 8 as libc::c_int | font << 8 as libc::c_int;
                }
                p = p.offset(1);
                p;
            }
        }
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn recode_char_to_encoding(
    mut c: libc::c_int,
    mut encoding: libc::c_int,
) -> libc::c_int {
    let mut fp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut x: libc::c_int = 0;
    if encoding == 8 as libc::c_int {
        return recode_char(c, 1 as libc::c_int, -(1 as libc::c_int));
    }
    fp = encodings[encoding as usize].fontlist;
    if !fp.is_null() {
        while *fp != 0 {
            let fresh0 = fp;
            fp = fp.offset(1);
            x = recode_char(
                c,
                0 as libc::c_int,
                *fresh0 as libc::c_uchar as libc::c_int,
            );
            if x != -(1 as libc::c_int) {
                return x;
            }
        }
    }
    if encodings[encoding as usize].deffont != 0 {
        x = recode_char(c, 0 as libc::c_int, encodings[encoding as usize].deffont);
        if x != -(1 as libc::c_int) {
            return x;
        }
    }
    return recode_char(c, 0 as libc::c_int, -(1 as libc::c_int));
}
unsafe extern "C" fn recode_char_dw_to_encoding(
    mut c: libc::c_int,
    mut c2p: *mut libc::c_int,
    mut encoding: libc::c_int,
) -> libc::c_int {
    let mut fp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut x: libc::c_int = 0;
    if encoding == 8 as libc::c_int {
        return recode_char_dw(c, c2p, 1 as libc::c_int, -(1 as libc::c_int));
    }
    fp = encodings[encoding as usize].fontlist;
    if !fp.is_null() {
        while *fp != 0 {
            let fresh1 = fp;
            fp = fp.offset(1);
            x = recode_char_dw(
                c,
                c2p,
                0 as libc::c_int,
                *fresh1 as libc::c_uchar as libc::c_int,
            );
            if x != -(1 as libc::c_int) {
                return x;
            }
        }
    }
    if encodings[encoding as usize].deffont != 0 {
        x = recode_char_dw(
            c,
            c2p,
            0 as libc::c_int,
            encodings[encoding as usize].deffont,
        );
        if x != -(1 as libc::c_int) {
            return x;
        }
    }
    return recode_char_dw(c, c2p, 0 as libc::c_int, -(1 as libc::c_int));
}
pub unsafe extern "C" fn recode_mchar(
    mut mc: *mut mchar,
    mut from: libc::c_int,
    mut to: libc::c_int,
) -> *mut mchar {
    static mut rmc: mchar = mchar {
        image: 0,
        attr: 0,
        font: 0,
        fontx: 0,
        color: 0,
        mbcs: 0,
    };
    let mut c: libc::c_int = 0;
    if from == to || from != 8 as libc::c_int && to != 8 as libc::c_int {
        return mc;
    }
    rmc = *mc;
    if rmc.font as libc::c_int == 0 as libc::c_int && from != 8 as libc::c_int {
        rmc.font = encodings[from as usize].deffont as libc::c_uchar;
    }
    if rmc.font as libc::c_int == 0 as libc::c_int {
        return mc;
    }
    c = rmc.image as libc::c_int | (rmc.font as libc::c_int) << 8 as libc::c_int;
    if from == 8 as libc::c_int {
        c |= (rmc.fontx as libc::c_int) << 16 as libc::c_int;
    }
    if rmc.mbcs != 0 {
        let mut c2: libc::c_int = rmc.mbcs as libc::c_int;
        c = recode_char_dw_to_encoding(c, &mut c2, to);
        rmc.mbcs = c2 as libc::c_uchar;
    } else {
        c = recode_char_to_encoding(c, to);
    }
    rmc.image = (c & 255 as libc::c_int) as libc::c_uchar;
    rmc.font = (c >> 8 as libc::c_int & 255 as libc::c_int) as libc::c_uchar;
    if to == 8 as libc::c_int {
        rmc.fontx = (c >> 16 as libc::c_int & 255 as libc::c_int) as libc::c_uchar;
    }
    return &mut rmc;
}
pub unsafe extern "C" fn recode_mline(
    mut ml: *mut mline,
    mut w: libc::c_int,
    mut from: libc::c_int,
    mut to: libc::c_int,
) -> *mut mline {
    static mut maxlen: libc::c_int = 0;
    static mut last: libc::c_int = 0;
    static mut rml: [mline; 2] = [mline {
        image: 0 as *const libc::c_uchar as *mut libc::c_uchar,
        attr: 0 as *const libc::c_uchar as *mut libc::c_uchar,
        font: 0 as *const libc::c_uchar as *mut libc::c_uchar,
        fontx: 0 as *const libc::c_uchar as *mut libc::c_uchar,
        color: 0 as *const libc::c_uchar as *mut libc::c_uchar,
    }; 2];
    static mut rl: *mut mline = 0 as *const mline as *mut mline;
    let mut i: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    if from == to || from != 8 as libc::c_int && to != 8 as libc::c_int
        || w == 0 as libc::c_int
    {
        return ml;
    }
    if (*ml).font == null && (*ml).fontx == null
        && encodings[from as usize].deffont == 0 as libc::c_int
    {
        return ml;
    }
    if w > maxlen {
        i = 0 as libc::c_int;
        while i < 2 as libc::c_int {
            if (rml[i as usize].image).is_null() {
                rml[i as usize].image = malloc(w as libc::c_ulong) as *mut libc::c_uchar;
            } else {
                rml[i as usize]
                    .image = realloc(
                    rml[i as usize].image as *mut libc::c_void,
                    w as libc::c_ulong,
                ) as *mut libc::c_uchar;
            }
            if (rml[i as usize].font).is_null() {
                rml[i as usize].font = malloc(w as libc::c_ulong) as *mut libc::c_uchar;
            } else {
                rml[i as usize]
                    .font = realloc(
                    rml[i as usize].font as *mut libc::c_void,
                    w as libc::c_ulong,
                ) as *mut libc::c_uchar;
            }
            if (rml[i as usize].fontx).is_null() {
                rml[i as usize].fontx = malloc(w as libc::c_ulong) as *mut libc::c_uchar;
            } else {
                rml[i as usize]
                    .fontx = realloc(
                    rml[i as usize].fontx as *mut libc::c_void,
                    w as libc::c_ulong,
                ) as *mut libc::c_uchar;
            }
            if (rml[i as usize].image).is_null() || (rml[i as usize].font).is_null()
                || (rml[i as usize].fontx).is_null()
            {
                maxlen = 0 as libc::c_int;
                return ml;
            }
            i += 1;
            i;
        }
        maxlen = w;
    }
    i = 0 as libc::c_int;
    while i < w {
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < w {
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < w {
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < w {
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < w {
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < w {
        i += 1;
        i;
    }
    rl = rml.as_mut_ptr().offset(last as isize);
    (*rl).attr = (*ml).attr;
    (*rl).color = (*ml).color;
    i = 0 as libc::c_int;
    while i < w {
        c = *((*ml).image).offset(i as isize) as libc::c_int
            | (*((*ml).font).offset(i as isize) as libc::c_int) << 8 as libc::c_int;
        if from == 8 as libc::c_int {
            c |= (*((*ml).fontx).offset(i as isize) as libc::c_int) << 16 as libc::c_int;
        }
        if from != 8 as libc::c_int && c < 256 as libc::c_int {
            c |= encodings[from as usize].deffont << 8 as libc::c_int;
        }
        if from != 8 as libc::c_int && c & 0x1f00 as libc::c_int != 0 as libc::c_int
            && c & 0xe000 as libc::c_int == 0 as libc::c_int
            || from == 8 as libc::c_int && utf8_isdouble(c) != 0
        {
            if i + 1 as libc::c_int == w {
                c = '?' as i32;
            } else {
                let mut c2: libc::c_int = 0;
                i += 1;
                i;
                c2 = *((*ml).image).offset(i as isize) as libc::c_int
                    | (*((*ml).font).offset(i as isize) as libc::c_int)
                        << 8 as libc::c_int;
                c = recode_char_dw_to_encoding(c, &mut c2, to);
                if to == 8 as libc::c_int {
                    *((*rl).fontx)
                        .offset(
                            (i - 1 as libc::c_int) as isize,
                        ) = (c >> 16 as libc::c_int & 255 as libc::c_int)
                        as libc::c_uchar;
                }
                *((*rl).font)
                    .offset(
                        (i - 1 as libc::c_int) as isize,
                    ) = (c >> 8 as libc::c_int & 255 as libc::c_int) as libc::c_uchar;
                *((*rl).image)
                    .offset(
                        (i - 1 as libc::c_int) as isize,
                    ) = (c & 255 as libc::c_int) as libc::c_uchar;
                c = c2;
            }
        } else {
            c = recode_char_to_encoding(c, to);
        }
        *((*rl).image).offset(i as isize) = (c & 255 as libc::c_int) as libc::c_uchar;
        *((*rl).font)
            .offset(
                i as isize,
            ) = (c >> 8 as libc::c_int & 255 as libc::c_int) as libc::c_uchar;
        if to == 8 as libc::c_int {
            *((*rl).fontx)
                .offset(
                    i as isize,
                ) = (c >> 16 as libc::c_int & 255 as libc::c_int) as libc::c_uchar;
        }
        i += 1;
        i;
    }
    last ^= 1 as libc::c_int;
    i = 0 as libc::c_int;
    while i < w {
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < w {
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < w {
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < w {
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < w {
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < w {
        i += 1;
        i;
    }
    return rl;
}
pub static mut combchars: *mut *mut combchar = 0 as *const *mut combchar
    as *mut *mut combchar;
pub unsafe extern "C" fn AddUtf8(mut c: libc::c_int) {
    if c >= 0xd800 as libc::c_int && c < 0xe000 as libc::c_int && !combchars.is_null()
        && !(*combchars.offset((c - 0xd800 as libc::c_int) as isize)).is_null()
    {
        AddUtf8(
            (**combchars.offset((c - 0xd800 as libc::c_int) as isize)).c1 as libc::c_int,
        );
        c = (**combchars.offset((c - 0xd800 as libc::c_int) as isize)).c2 as libc::c_int;
    }
    if c >= 0x10000 as libc::c_int {
        if c >= 0x200000 as libc::c_int {
            (*display).d_obuffree -= 1;
            if (*display).d_obuffree <= 0 as libc::c_int {
                Resize_obuf();
            }
            let fresh2 = (*display).d_obufp;
            (*display).d_obufp = ((*display).d_obufp).offset(1);
            *fresh2 = ((c & 0x3000000 as libc::c_int) >> 12 as libc::c_int
                ^ 0xf8 as libc::c_int) as libc::c_char;
            c = c & 0xffffff as libc::c_int
                ^ (0xf0 as libc::c_int ^ 0x80 as libc::c_int) << 18 as libc::c_int;
        }
        (*display).d_obuffree -= 1;
        if (*display).d_obuffree <= 0 as libc::c_int {
            Resize_obuf();
        }
        let fresh3 = (*display).d_obufp;
        (*display).d_obufp = ((*display).d_obufp).offset(1);
        *fresh3 = ((c & 0x1fc0000 as libc::c_int) >> 18 as libc::c_int
            ^ 0xf0 as libc::c_int) as libc::c_char;
        c = c & 0x3ffff as libc::c_int
            ^ (0xe0 as libc::c_int ^ 0x80 as libc::c_int) << 12 as libc::c_int;
    }
    if c >= 0x800 as libc::c_int {
        (*display).d_obuffree -= 1;
        if (*display).d_obuffree <= 0 as libc::c_int {
            Resize_obuf();
        }
        let fresh4 = (*display).d_obufp;
        (*display).d_obufp = ((*display).d_obufp).offset(1);
        *fresh4 = ((c & 0x7f000 as libc::c_int) >> 12 as libc::c_int
            ^ 0xe0 as libc::c_int) as libc::c_char;
        c = c & 0xfff as libc::c_int
            ^ (0xc0 as libc::c_int ^ 0x80 as libc::c_int) << 6 as libc::c_int;
    }
    if c >= 0x80 as libc::c_int {
        (*display).d_obuffree -= 1;
        if (*display).d_obuffree <= 0 as libc::c_int {
            Resize_obuf();
        }
        let fresh5 = (*display).d_obufp;
        (*display).d_obufp = ((*display).d_obufp).offset(1);
        *fresh5 = ((c & 0x1fc0 as libc::c_int) >> 6 as libc::c_int ^ 0xc0 as libc::c_int)
            as libc::c_char;
        c = c & 0x3f as libc::c_int | 0x80 as libc::c_int;
    }
    (*display).d_obuffree -= 1;
    if (*display).d_obuffree <= 0 as libc::c_int {
        Resize_obuf();
    }
    let fresh6 = (*display).d_obufp;
    (*display).d_obufp = ((*display).d_obufp).offset(1);
    *fresh6 = c as libc::c_char;
}
pub unsafe extern "C" fn ToUtf8_comb(
    mut p: *mut libc::c_char,
    mut c: libc::c_int,
) -> libc::c_int {
    let mut l: libc::c_int = 0;
    if c >= 0xd800 as libc::c_int && c < 0xe000 as libc::c_int && !combchars.is_null()
        && !(*combchars.offset((c - 0xd800 as libc::c_int) as isize)).is_null()
    {
        l = ToUtf8_comb(
            p,
            (**combchars.offset((c - 0xd800 as libc::c_int) as isize)).c1 as libc::c_int,
        );
        return l
            + ToUtf8(
                (if !p.is_null() {
                    p.offset(l as isize)
                } else {
                    0 as *mut libc::c_char
                }),
                (**combchars.offset((c - 0xd800 as libc::c_int) as isize)).c2
                    as libc::c_int,
            );
    }
    return ToUtf8(p, c);
}
pub unsafe extern "C" fn ToUtf8(
    mut p: *mut libc::c_char,
    mut c: libc::c_int,
) -> libc::c_int {
    let mut l: libc::c_int = 1 as libc::c_int;
    if c >= 0x10000 as libc::c_int {
        if c >= 0x200000 as libc::c_int {
            if !p.is_null() {
                let fresh7 = p;
                p = p.offset(1);
                *fresh7 = ((c & 0x3000000 as libc::c_int) >> 12 as libc::c_int
                    ^ 0xf8 as libc::c_int) as libc::c_char;
            }
            l += 1;
            l;
            c = c & 0xffffff as libc::c_int
                ^ (0xf0 as libc::c_int ^ 0x80 as libc::c_int) << 18 as libc::c_int;
        }
        if !p.is_null() {
            let fresh8 = p;
            p = p.offset(1);
            *fresh8 = ((c & 0x1fc0000 as libc::c_int) >> 18 as libc::c_int
                ^ 0xf0 as libc::c_int) as libc::c_char;
        }
        l += 1;
        l;
        c = c & 0x3ffff as libc::c_int
            ^ (0xe0 as libc::c_int ^ 0x80 as libc::c_int) << 12 as libc::c_int;
    }
    if c >= 0x800 as libc::c_int {
        if !p.is_null() {
            let fresh9 = p;
            p = p.offset(1);
            *fresh9 = ((c & 0x7f000 as libc::c_int) >> 12 as libc::c_int
                ^ 0xe0 as libc::c_int) as libc::c_char;
        }
        l += 1;
        l;
        c = c & 0xfff as libc::c_int | 0x1000 as libc::c_int;
    }
    if c >= 0x80 as libc::c_int {
        if !p.is_null() {
            let fresh10 = p;
            p = p.offset(1);
            *fresh10 = ((c & 0x1fc0 as libc::c_int) >> 6 as libc::c_int
                ^ 0xc0 as libc::c_int) as libc::c_char;
        }
        l += 1;
        l;
        c = c & 0x3f as libc::c_int | 0x80 as libc::c_int;
    }
    if !p.is_null() {
        let fresh11 = p;
        p = p.offset(1);
        *fresh11 = c as libc::c_char;
    }
    return l;
}
pub unsafe extern "C" fn FromUtf8(
    mut c: libc::c_int,
    mut utf8charp: *mut libc::c_int,
) -> libc::c_int {
    let mut utf8char: libc::c_int = *utf8charp;
    if utf8char != 0 {
        if c & 0xc0 as libc::c_int != 0x80 as libc::c_int {
            *utf8charp = 0 as libc::c_int;
            return -(2 as libc::c_int);
        } else {
            c = c & 0x3f as libc::c_int | utf8char << 6 as libc::c_int;
        }
        if utf8char & 0x40000000 as libc::c_int == 0 {
            if c as libc::c_uint & 0x820823e0 as libc::c_uint
                == 0x80000000 as libc::c_uint
            {
                c = 0xfdffffff as libc::c_uint as libc::c_int;
            } else if c & 0x20821f0 as libc::c_int == 0x2000000 as libc::c_int {
                c = 0xfff7ffff as libc::c_uint as libc::c_int;
            } else if c & 0x820f8 as libc::c_int == 0x80000 as libc::c_int {
                c = 0xffffd000 as libc::c_uint as libc::c_int;
            } else if c & 0x207c as libc::c_int == 0x2000 as libc::c_int {
                c = 0xffffff70 as libc::c_uint as libc::c_int;
            }
        }
    } else if c >= 0xfe as libc::c_int {
        c = 0xfffd as libc::c_int;
    } else if c >= 0xfc as libc::c_int {
        c = ((c & 0x1 as libc::c_int) as libc::c_uint | 0xbffffffc as libc::c_uint)
            as libc::c_int;
    } else if c >= 0xf8 as libc::c_int {
        c = ((c & 0x3 as libc::c_int) as libc::c_uint | 0xbfffff00 as libc::c_uint)
            as libc::c_int;
    } else if c >= 0xf0 as libc::c_int {
        c = ((c & 0x7 as libc::c_int) as libc::c_uint | 0xbfffc000 as libc::c_uint)
            as libc::c_int;
    } else if c >= 0xe0 as libc::c_int {
        c = ((c & 0xf as libc::c_int) as libc::c_uint | 0xbff00000 as libc::c_uint)
            as libc::c_int;
    } else if c >= 0xc2 as libc::c_int {
        c = ((c & 0x1f as libc::c_int) as libc::c_uint | 0xfc000000 as libc::c_uint)
            as libc::c_int;
    } else if c >= 0xc0 as libc::c_int {
        c = 0xfdffffff as libc::c_uint as libc::c_int;
    } else if c >= 0x80 as libc::c_int {
        c = 0xfffd as libc::c_int;
    }
    utf8char = if c as libc::c_uint & 0x80000000 as libc::c_uint != 0 {
        c
    } else {
        0 as libc::c_int
    };
    *utf8charp = utf8char;
    if utf8char != 0 {
        return -(1 as libc::c_int);
    }
    if c as libc::c_uint & 0xff800000 as libc::c_uint != 0 {
        c = 0xfffd as libc::c_int;
    }
    if c >= 0xd800 as libc::c_int
        && (c <= 0xdfff as libc::c_int || c == 0xfffe as libc::c_int
            || c == 0xffff as libc::c_int)
    {
        c = 0xfffd as libc::c_int;
    }
    return c;
}
pub unsafe extern "C" fn WinSwitchEncoding(mut p: *mut win, mut encoding: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut ml: *mut mline = 0 as *mut mline;
    let mut d: *mut display = 0 as *mut display;
    let mut cv: *mut canvas = 0 as *mut canvas;
    let mut oldflayer: *mut layer = 0 as *mut layer;
    if ((*p).w_layer.l_encoding == 8 as libc::c_int) as libc::c_int
        == (encoding == 8 as libc::c_int) as libc::c_int
    {
        (*p).w_layer.l_encoding = encoding;
        return;
    }
    oldflayer = flayer;
    d = displays;
    while !d.is_null() {
        cv = (*d).d_cvlist;
        while !cv.is_null() {
            if p == (*(*(*cv).c_layer).l_bottom).l_data as *mut win {
                flayer = (*cv).c_layer;
                while !((*flayer).l_next).is_null() {
                    if oldflayer == flayer {
                        oldflayer = (*flayer).l_next;
                    }
                    ExitOverlayPage();
                }
            }
            cv = (*cv).c_next;
        }
        d = (*d).d_next;
    }
    flayer = oldflayer;
    j = 0 as libc::c_int;
    while j < (*p).w_layer.l_height + (*p).w_histheight {
        ml = if j < (*p).w_layer.l_height {
            &mut *((*p).w_mlines).offset(j as isize) as *mut mline
        } else {
            &mut *((*p).w_hlines).offset((j - (*p).w_layer.l_height) as isize)
                as *mut mline
        };
        if !((*ml).font == null && ((*ml).fontx).is_null()
            && encodings[(*p).w_layer.l_encoding as usize].deffont == 0 as libc::c_int)
        {
            i = 0 as libc::c_int;
            while i < (*p).w_layer.l_width {
                c = *((*ml).image).offset(i as isize) as libc::c_int
                    | (*((*ml).font).offset(i as isize) as libc::c_int)
                        << 8 as libc::c_int;
                if (*p).w_layer.l_encoding == 8 as libc::c_int {
                    c
                        |= (*((*ml).fontx).offset(i as isize) as libc::c_int)
                            << 16 as libc::c_int;
                }
                if (*p).w_layer.l_encoding != 8 as libc::c_int && c < 256 as libc::c_int
                {
                    c
                        |= encodings[(*p).w_layer.l_encoding as usize].deffont
                            << 8 as libc::c_int;
                }
                if !(c < 256 as libc::c_int) {
                    if (*ml).font == null {
                        (*ml)
                            .font = calloc(
                            ((*p).w_layer.l_width + 1 as libc::c_int) as libc::c_ulong,
                            1 as libc::c_int as libc::c_ulong,
                        ) as *mut libc::c_uchar;
                        if ((*ml).font).is_null() {
                            (*ml).font = null;
                            break;
                        }
                    }
                    if (*p).w_layer.l_encoding != 8 as libc::c_int
                        && c & 0x1f00 as libc::c_int != 0 as libc::c_int
                        && c & 0xe000 as libc::c_int == 0 as libc::c_int
                        || (*p).w_layer.l_encoding == 8 as libc::c_int
                            && utf8_isdouble(c) != 0
                    {
                        if i + 1 as libc::c_int == (*p).w_layer.l_width {
                            c = '?' as i32;
                        } else {
                            let mut c2: libc::c_int = 0;
                            i += 1;
                            i;
                            c2 = *((*ml).image).offset(i as isize) as libc::c_int
                                | (*((*ml).font).offset(i as isize) as libc::c_int)
                                    << 8 as libc::c_int
                                | (*((*ml).fontx).offset(i as isize) as libc::c_int)
                                    << 16 as libc::c_int;
                            c = recode_char_dw_to_encoding(c, &mut c2, encoding);
                            if encoding == 8 as libc::c_int {
                                if c > 0x10000 as libc::c_int && (*ml).fontx == null {
                                    (*ml)
                                        .fontx = calloc(
                                        ((*p).w_layer.l_width + 1 as libc::c_int) as libc::c_ulong,
                                        1 as libc::c_int as libc::c_ulong,
                                    ) as *mut libc::c_uchar;
                                    if ((*ml).fontx).is_null() {
                                        (*ml).fontx = null;
                                        break;
                                    }
                                }
                                *((*ml).fontx)
                                    .offset(
                                        (i - 1 as libc::c_int) as isize,
                                    ) = (c >> 16 as libc::c_int & 255 as libc::c_int)
                                    as libc::c_uchar;
                            } else {
                                (*ml).fontx = null;
                            }
                            *((*ml).font)
                                .offset(
                                    (i - 1 as libc::c_int) as isize,
                                ) = (c >> 8 as libc::c_int & 255 as libc::c_int)
                                as libc::c_uchar;
                            *((*ml).image)
                                .offset(
                                    (i - 1 as libc::c_int) as isize,
                                ) = (c & 255 as libc::c_int) as libc::c_uchar;
                            c = c2;
                        }
                    } else {
                        c = recode_char_to_encoding(c, encoding);
                    }
                    *((*ml).image)
                        .offset(i as isize) = (c & 255 as libc::c_int) as libc::c_uchar;
                    *((*ml).font)
                        .offset(
                            i as isize,
                        ) = (c >> 8 as libc::c_int & 255 as libc::c_int)
                        as libc::c_uchar;
                    if encoding == 8 as libc::c_int {
                        if c > 0x10000 as libc::c_int && (*ml).fontx == null {
                            (*ml)
                                .fontx = calloc(
                                ((*p).w_layer.l_width + 1 as libc::c_int) as libc::c_ulong,
                                1 as libc::c_int as libc::c_ulong,
                            ) as *mut libc::c_uchar;
                            if ((*ml).fontx).is_null() {
                                (*ml).fontx = null;
                                break;
                            }
                        }
                        *((*ml).fontx)
                            .offset(
                                i as isize,
                            ) = (c >> 16 as libc::c_int & 255 as libc::c_int)
                            as libc::c_uchar;
                    } else {
                        (*ml).fontx = null;
                    }
                }
                i += 1;
                i;
            }
        }
        j += 1;
        j;
    }
    (*p).w_layer.l_encoding = encoding;
}
unsafe extern "C" fn bisearch(
    mut ucs: libc::c_int,
    mut table: *const interval,
    mut max: libc::c_int,
) -> libc::c_int {
    let mut min: libc::c_int = 0 as libc::c_int;
    let mut mid: libc::c_int = 0;
    if ucs < (*table.offset(0 as libc::c_int as isize)).first
        || ucs > (*table.offset(max as isize)).last
    {
        return 0 as libc::c_int;
    }
    while max >= min {
        mid = (min + max) / 2 as libc::c_int;
        if ucs > (*table.offset(mid as isize)).last {
            min = mid + 1 as libc::c_int;
        } else if ucs < (*table.offset(mid as isize)).first {
            max = mid - 1 as libc::c_int;
        } else {
            return 1 as libc::c_int
        }
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn utf8_isdouble(mut c: libc::c_int) -> libc::c_int {
    static mut ambiguous: [interval; 179] = [
        {
            let mut init = interval {
                first: 0xa1 as libc::c_int,
                last: 0xa1 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xa4 as libc::c_int,
                last: 0xa4 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xa7 as libc::c_int,
                last: 0xa8 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xaa as libc::c_int,
                last: 0xaa as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xad as libc::c_int,
                last: 0xae as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xb0 as libc::c_int,
                last: 0xb4 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xb6 as libc::c_int,
                last: 0xba as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xbc as libc::c_int,
                last: 0xbf as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xc6 as libc::c_int,
                last: 0xc6 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xd0 as libc::c_int,
                last: 0xd0 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xd7 as libc::c_int,
                last: 0xd8 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xde as libc::c_int,
                last: 0xe1 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xe6 as libc::c_int,
                last: 0xe6 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xe8 as libc::c_int,
                last: 0xea as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xec as libc::c_int,
                last: 0xed as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xf0 as libc::c_int,
                last: 0xf0 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xf2 as libc::c_int,
                last: 0xf3 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xf7 as libc::c_int,
                last: 0xfa as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xfc as libc::c_int,
                last: 0xfc as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xfe as libc::c_int,
                last: 0xfe as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x101 as libc::c_int,
                last: 0x101 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x111 as libc::c_int,
                last: 0x111 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x113 as libc::c_int,
                last: 0x113 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x11b as libc::c_int,
                last: 0x11b as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x126 as libc::c_int,
                last: 0x127 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x12b as libc::c_int,
                last: 0x12b as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x131 as libc::c_int,
                last: 0x133 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x138 as libc::c_int,
                last: 0x138 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x13f as libc::c_int,
                last: 0x142 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x144 as libc::c_int,
                last: 0x144 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x148 as libc::c_int,
                last: 0x14b as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x14d as libc::c_int,
                last: 0x14d as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x152 as libc::c_int,
                last: 0x153 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x166 as libc::c_int,
                last: 0x167 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x16b as libc::c_int,
                last: 0x16b as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1ce as libc::c_int,
                last: 0x1ce as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1d0 as libc::c_int,
                last: 0x1d0 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1d2 as libc::c_int,
                last: 0x1d2 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1d4 as libc::c_int,
                last: 0x1d4 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1d6 as libc::c_int,
                last: 0x1d6 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1d8 as libc::c_int,
                last: 0x1d8 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1da as libc::c_int,
                last: 0x1da as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1dc as libc::c_int,
                last: 0x1dc as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x251 as libc::c_int,
                last: 0x251 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x261 as libc::c_int,
                last: 0x261 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2c4 as libc::c_int,
                last: 0x2c4 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2c7 as libc::c_int,
                last: 0x2c7 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2c9 as libc::c_int,
                last: 0x2cb as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2cd as libc::c_int,
                last: 0x2cd as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2d0 as libc::c_int,
                last: 0x2d0 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2d8 as libc::c_int,
                last: 0x2db as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2dd as libc::c_int,
                last: 0x2dd as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2df as libc::c_int,
                last: 0x2df as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x300 as libc::c_int,
                last: 0x36f as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x391 as libc::c_int,
                last: 0x3a1 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x3a3 as libc::c_int,
                last: 0x3a9 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x3b1 as libc::c_int,
                last: 0x3c1 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x3c3 as libc::c_int,
                last: 0x3c9 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x401 as libc::c_int,
                last: 0x401 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x410 as libc::c_int,
                last: 0x44f as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x451 as libc::c_int,
                last: 0x451 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2010 as libc::c_int,
                last: 0x2010 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2013 as libc::c_int,
                last: 0x2016 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2018 as libc::c_int,
                last: 0x2019 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x201c as libc::c_int,
                last: 0x201d as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2020 as libc::c_int,
                last: 0x2022 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2024 as libc::c_int,
                last: 0x2027 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2030 as libc::c_int,
                last: 0x2030 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2032 as libc::c_int,
                last: 0x2033 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2035 as libc::c_int,
                last: 0x2035 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x203b as libc::c_int,
                last: 0x203b as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x203e as libc::c_int,
                last: 0x203e as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2074 as libc::c_int,
                last: 0x2074 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x207f as libc::c_int,
                last: 0x207f as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2081 as libc::c_int,
                last: 0x2084 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x20ac as libc::c_int,
                last: 0x20ac as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2103 as libc::c_int,
                last: 0x2103 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2105 as libc::c_int,
                last: 0x2105 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2109 as libc::c_int,
                last: 0x2109 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2113 as libc::c_int,
                last: 0x2113 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2116 as libc::c_int,
                last: 0x2116 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2121 as libc::c_int,
                last: 0x2122 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2126 as libc::c_int,
                last: 0x2126 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x212b as libc::c_int,
                last: 0x212b as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2153 as libc::c_int,
                last: 0x2154 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x215b as libc::c_int,
                last: 0x215e as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2160 as libc::c_int,
                last: 0x216b as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2170 as libc::c_int,
                last: 0x2179 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2189 as libc::c_int,
                last: 0x2189 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2190 as libc::c_int,
                last: 0x2199 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x21b8 as libc::c_int,
                last: 0x21b9 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x21d2 as libc::c_int,
                last: 0x21d2 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x21d4 as libc::c_int,
                last: 0x21d4 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x21e7 as libc::c_int,
                last: 0x21e7 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2200 as libc::c_int,
                last: 0x2200 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2202 as libc::c_int,
                last: 0x2203 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2207 as libc::c_int,
                last: 0x2208 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x220b as libc::c_int,
                last: 0x220b as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x220f as libc::c_int,
                last: 0x220f as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2211 as libc::c_int,
                last: 0x2211 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2215 as libc::c_int,
                last: 0x2215 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x221a as libc::c_int,
                last: 0x221a as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x221d as libc::c_int,
                last: 0x2220 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2223 as libc::c_int,
                last: 0x2223 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2225 as libc::c_int,
                last: 0x2225 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2227 as libc::c_int,
                last: 0x222c as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x222e as libc::c_int,
                last: 0x222e as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2234 as libc::c_int,
                last: 0x2237 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x223c as libc::c_int,
                last: 0x223d as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2248 as libc::c_int,
                last: 0x2248 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x224c as libc::c_int,
                last: 0x224c as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2252 as libc::c_int,
                last: 0x2252 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2260 as libc::c_int,
                last: 0x2261 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2264 as libc::c_int,
                last: 0x2267 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x226a as libc::c_int,
                last: 0x226b as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x226e as libc::c_int,
                last: 0x226f as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2282 as libc::c_int,
                last: 0x2283 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2286 as libc::c_int,
                last: 0x2287 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2295 as libc::c_int,
                last: 0x2295 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2299 as libc::c_int,
                last: 0x2299 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x22a5 as libc::c_int,
                last: 0x22a5 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x22bf as libc::c_int,
                last: 0x22bf as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2312 as libc::c_int,
                last: 0x2312 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2460 as libc::c_int,
                last: 0x24e9 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x24eb as libc::c_int,
                last: 0x254b as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2550 as libc::c_int,
                last: 0x2573 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2580 as libc::c_int,
                last: 0x258f as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2592 as libc::c_int,
                last: 0x2595 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x25a0 as libc::c_int,
                last: 0x25a1 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x25a3 as libc::c_int,
                last: 0x25a9 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x25b2 as libc::c_int,
                last: 0x25b3 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x25b6 as libc::c_int,
                last: 0x25b7 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x25bc as libc::c_int,
                last: 0x25bd as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x25c0 as libc::c_int,
                last: 0x25c1 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x25c6 as libc::c_int,
                last: 0x25c8 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x25cb as libc::c_int,
                last: 0x25cb as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x25ce as libc::c_int,
                last: 0x25d1 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x25e2 as libc::c_int,
                last: 0x25e5 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x25ef as libc::c_int,
                last: 0x25ef as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2605 as libc::c_int,
                last: 0x2606 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2609 as libc::c_int,
                last: 0x2609 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x260e as libc::c_int,
                last: 0x260f as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x261c as libc::c_int,
                last: 0x261c as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x261e as libc::c_int,
                last: 0x261e as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2640 as libc::c_int,
                last: 0x2640 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2642 as libc::c_int,
                last: 0x2642 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2660 as libc::c_int,
                last: 0x2661 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2663 as libc::c_int,
                last: 0x2665 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2667 as libc::c_int,
                last: 0x266a as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x266c as libc::c_int,
                last: 0x266d as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x266f as libc::c_int,
                last: 0x266f as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x269e as libc::c_int,
                last: 0x269f as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x26bf as libc::c_int,
                last: 0x26bf as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x26c6 as libc::c_int,
                last: 0x26cd as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x26cf as libc::c_int,
                last: 0x26d3 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x26d5 as libc::c_int,
                last: 0x26e1 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x26e3 as libc::c_int,
                last: 0x26e3 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x26e8 as libc::c_int,
                last: 0x26e9 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x26eb as libc::c_int,
                last: 0x26f1 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x26f4 as libc::c_int,
                last: 0x26f4 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x26f6 as libc::c_int,
                last: 0x26f9 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x26fb as libc::c_int,
                last: 0x26fc as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x26fe as libc::c_int,
                last: 0x26ff as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x273d as libc::c_int,
                last: 0x273d as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2776 as libc::c_int,
                last: 0x277f as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2b56 as libc::c_int,
                last: 0x2b59 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x3248 as libc::c_int,
                last: 0x324f as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xe000 as libc::c_int,
                last: 0xf8ff as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xfe00 as libc::c_int,
                last: 0xfe0f as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xfffd as libc::c_int,
                last: 0xfffd as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f100 as libc::c_int,
                last: 0x1f10a as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f110 as libc::c_int,
                last: 0x1f12d as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f130 as libc::c_int,
                last: 0x1f169 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f170 as libc::c_int,
                last: 0x1f18d as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f18f as libc::c_int,
                last: 0x1f190 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f19b as libc::c_int,
                last: 0x1f1ac as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xe0100 as libc::c_int,
                last: 0xe01ef as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xf0000 as libc::c_int,
                last: 0xffffd as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x100000 as libc::c_int,
                last: 0x10fffd as libc::c_int,
            };
            init
        },
    ];
    static mut wide: [interval; 113] = [
        {
            let mut init = interval {
                first: 0x1100 as libc::c_int,
                last: 0x115f as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x231a as libc::c_int,
                last: 0x231b as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2329 as libc::c_int,
                last: 0x232a as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x23e9 as libc::c_int,
                last: 0x23ec as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x23f0 as libc::c_int,
                last: 0x23f0 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x23f3 as libc::c_int,
                last: 0x23f3 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x25fd as libc::c_int,
                last: 0x25fe as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2614 as libc::c_int,
                last: 0x2615 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2648 as libc::c_int,
                last: 0x2653 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x267f as libc::c_int,
                last: 0x267f as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2693 as libc::c_int,
                last: 0x2693 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x26a1 as libc::c_int,
                last: 0x26a1 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x26aa as libc::c_int,
                last: 0x26ab as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x26bd as libc::c_int,
                last: 0x26be as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x26c4 as libc::c_int,
                last: 0x26c5 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x26ce as libc::c_int,
                last: 0x26ce as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x26d4 as libc::c_int,
                last: 0x26d4 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x26ea as libc::c_int,
                last: 0x26ea as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x26f2 as libc::c_int,
                last: 0x26f3 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x26f5 as libc::c_int,
                last: 0x26f5 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x26fa as libc::c_int,
                last: 0x26fa as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x26fd as libc::c_int,
                last: 0x26fd as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2705 as libc::c_int,
                last: 0x2705 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x270a as libc::c_int,
                last: 0x270b as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2728 as libc::c_int,
                last: 0x2728 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x274c as libc::c_int,
                last: 0x274c as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x274e as libc::c_int,
                last: 0x274e as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2753 as libc::c_int,
                last: 0x2755 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2757 as libc::c_int,
                last: 0x2757 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2795 as libc::c_int,
                last: 0x2797 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x27b0 as libc::c_int,
                last: 0x27b0 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x27bf as libc::c_int,
                last: 0x27bf as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2b1b as libc::c_int,
                last: 0x2b1c as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2b50 as libc::c_int,
                last: 0x2b50 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2b55 as libc::c_int,
                last: 0x2b55 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2e80 as libc::c_int,
                last: 0x2e99 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2e9b as libc::c_int,
                last: 0x2ef3 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2f00 as libc::c_int,
                last: 0x2fd5 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2ff0 as libc::c_int,
                last: 0x2ffb as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x3000 as libc::c_int,
                last: 0x303e as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x3041 as libc::c_int,
                last: 0x3096 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x3099 as libc::c_int,
                last: 0x30ff as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x3105 as libc::c_int,
                last: 0x312f as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x3131 as libc::c_int,
                last: 0x318e as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x3190 as libc::c_int,
                last: 0x31ba as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x31c0 as libc::c_int,
                last: 0x31e3 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x31f0 as libc::c_int,
                last: 0x321e as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x3220 as libc::c_int,
                last: 0x3247 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x3250 as libc::c_int,
                last: 0x4dbf as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x4e00 as libc::c_int,
                last: 0xa48c as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xa490 as libc::c_int,
                last: 0xa4c6 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xa960 as libc::c_int,
                last: 0xa97c as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xac00 as libc::c_int,
                last: 0xd7a3 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xf900 as libc::c_int,
                last: 0xfaff as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xfe10 as libc::c_int,
                last: 0xfe19 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xfe30 as libc::c_int,
                last: 0xfe52 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xfe54 as libc::c_int,
                last: 0xfe66 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xfe68 as libc::c_int,
                last: 0xfe6b as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xff01 as libc::c_int,
                last: 0xff60 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xffe0 as libc::c_int,
                last: 0xffe6 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x16fe0 as libc::c_int,
                last: 0x16fe3 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x17000 as libc::c_int,
                last: 0x187f7 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x18800 as libc::c_int,
                last: 0x18af2 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1b000 as libc::c_int,
                last: 0x1b11e as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1b150 as libc::c_int,
                last: 0x1b152 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1b164 as libc::c_int,
                last: 0x1b167 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1b170 as libc::c_int,
                last: 0x1b2fb as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f004 as libc::c_int,
                last: 0x1f004 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f0cf as libc::c_int,
                last: 0x1f0cf as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f18e as libc::c_int,
                last: 0x1f18e as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f191 as libc::c_int,
                last: 0x1f19a as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f200 as libc::c_int,
                last: 0x1f202 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f210 as libc::c_int,
                last: 0x1f23b as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f240 as libc::c_int,
                last: 0x1f248 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f250 as libc::c_int,
                last: 0x1f251 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f260 as libc::c_int,
                last: 0x1f265 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f300 as libc::c_int,
                last: 0x1f320 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f32d as libc::c_int,
                last: 0x1f335 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f337 as libc::c_int,
                last: 0x1f37c as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f37e as libc::c_int,
                last: 0x1f393 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f3a0 as libc::c_int,
                last: 0x1f3ca as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f3cf as libc::c_int,
                last: 0x1f3d3 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f3e0 as libc::c_int,
                last: 0x1f3f0 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f3f4 as libc::c_int,
                last: 0x1f3f4 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f3f8 as libc::c_int,
                last: 0x1f43e as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f440 as libc::c_int,
                last: 0x1f440 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f442 as libc::c_int,
                last: 0x1f4fc as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f4ff as libc::c_int,
                last: 0x1f53d as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f54b as libc::c_int,
                last: 0x1f54e as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f550 as libc::c_int,
                last: 0x1f567 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f57a as libc::c_int,
                last: 0x1f57a as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f595 as libc::c_int,
                last: 0x1f596 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f5a4 as libc::c_int,
                last: 0x1f5a4 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f5fb as libc::c_int,
                last: 0x1f64f as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f680 as libc::c_int,
                last: 0x1f6c5 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f6cc as libc::c_int,
                last: 0x1f6cc as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f6d0 as libc::c_int,
                last: 0x1f6d2 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f6d5 as libc::c_int,
                last: 0x1f6d5 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f6eb as libc::c_int,
                last: 0x1f6ec as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f6f4 as libc::c_int,
                last: 0x1f6fa as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f7e0 as libc::c_int,
                last: 0x1f7eb as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f90d as libc::c_int,
                last: 0x1f971 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f973 as libc::c_int,
                last: 0x1f976 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f97a as libc::c_int,
                last: 0x1f9a2 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f9a5 as libc::c_int,
                last: 0x1f9aa as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f9ae as libc::c_int,
                last: 0x1f9ca as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1f9cd as libc::c_int,
                last: 0x1f9ff as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1fa70 as libc::c_int,
                last: 0x1fa73 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1fa78 as libc::c_int,
                last: 0x1fa7a as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1fa80 as libc::c_int,
                last: 0x1fa82 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1fa90 as libc::c_int,
                last: 0x1fa95 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x20000 as libc::c_int,
                last: 0x2fffd as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x30000 as libc::c_int,
                last: 0x3fffd as libc::c_int,
            };
            init
        },
    ];
    if c >= 0xdf00 as libc::c_int && c <= 0xdfff as libc::c_int {
        return 1 as libc::c_int;
    }
    return (bisearch(
        c,
        wide.as_ptr(),
        (::std::mem::size_of::<[interval; 113]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<interval>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
    ) != 0
        || cjkwidth != 0
            && bisearch(
                c,
                ambiguous.as_ptr(),
                (::std::mem::size_of::<[interval; 179]>() as libc::c_ulong)
                    .wrapping_div(::std::mem::size_of::<interval>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
            ) != 0) as libc::c_int;
}
pub unsafe extern "C" fn utf8_iscomb(mut c: libc::c_int) -> libc::c_int {
    static mut combining: [interval; 142] = [
        {
            let mut init = interval {
                first: 0x300 as libc::c_int,
                last: 0x36f as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x483 as libc::c_int,
                last: 0x486 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x488 as libc::c_int,
                last: 0x489 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x591 as libc::c_int,
                last: 0x5bd as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x5bf as libc::c_int,
                last: 0x5bf as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x5c1 as libc::c_int,
                last: 0x5c2 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x5c4 as libc::c_int,
                last: 0x5c5 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x5c7 as libc::c_int,
                last: 0x5c7 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x600 as libc::c_int,
                last: 0x603 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x610 as libc::c_int,
                last: 0x615 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x64b as libc::c_int,
                last: 0x65e as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x670 as libc::c_int,
                last: 0x670 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x6d6 as libc::c_int,
                last: 0x6e4 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x6e7 as libc::c_int,
                last: 0x6e8 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x6ea as libc::c_int,
                last: 0x6ed as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x70f as libc::c_int,
                last: 0x70f as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x711 as libc::c_int,
                last: 0x711 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x730 as libc::c_int,
                last: 0x74a as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x7a6 as libc::c_int,
                last: 0x7b0 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x7eb as libc::c_int,
                last: 0x7f3 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x901 as libc::c_int,
                last: 0x902 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x93c as libc::c_int,
                last: 0x93c as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x941 as libc::c_int,
                last: 0x948 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x94d as libc::c_int,
                last: 0x94d as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x951 as libc::c_int,
                last: 0x954 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x962 as libc::c_int,
                last: 0x963 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x981 as libc::c_int,
                last: 0x981 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x9bc as libc::c_int,
                last: 0x9bc as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x9c1 as libc::c_int,
                last: 0x9c4 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x9cd as libc::c_int,
                last: 0x9cd as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x9e2 as libc::c_int,
                last: 0x9e3 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xa01 as libc::c_int,
                last: 0xa02 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xa3c as libc::c_int,
                last: 0xa3c as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xa41 as libc::c_int,
                last: 0xa42 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xa47 as libc::c_int,
                last: 0xa48 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xa4b as libc::c_int,
                last: 0xa4d as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xa70 as libc::c_int,
                last: 0xa71 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xa81 as libc::c_int,
                last: 0xa82 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xabc as libc::c_int,
                last: 0xabc as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xac1 as libc::c_int,
                last: 0xac5 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xac7 as libc::c_int,
                last: 0xac8 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xacd as libc::c_int,
                last: 0xacd as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xae2 as libc::c_int,
                last: 0xae3 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xb01 as libc::c_int,
                last: 0xb01 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xb3c as libc::c_int,
                last: 0xb3c as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xb3f as libc::c_int,
                last: 0xb3f as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xb41 as libc::c_int,
                last: 0xb43 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xb4d as libc::c_int,
                last: 0xb4d as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xb56 as libc::c_int,
                last: 0xb56 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xb82 as libc::c_int,
                last: 0xb82 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xbc0 as libc::c_int,
                last: 0xbc0 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xbcd as libc::c_int,
                last: 0xbcd as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xc3e as libc::c_int,
                last: 0xc40 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xc46 as libc::c_int,
                last: 0xc48 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xc4a as libc::c_int,
                last: 0xc4d as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xc55 as libc::c_int,
                last: 0xc56 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xcbc as libc::c_int,
                last: 0xcbc as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xcbf as libc::c_int,
                last: 0xcbf as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xcc6 as libc::c_int,
                last: 0xcc6 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xccc as libc::c_int,
                last: 0xccd as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xce2 as libc::c_int,
                last: 0xce3 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xd41 as libc::c_int,
                last: 0xd43 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xd4d as libc::c_int,
                last: 0xd4d as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xdca as libc::c_int,
                last: 0xdca as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xdd2 as libc::c_int,
                last: 0xdd4 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xdd6 as libc::c_int,
                last: 0xdd6 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xe31 as libc::c_int,
                last: 0xe31 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xe34 as libc::c_int,
                last: 0xe3a as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xe47 as libc::c_int,
                last: 0xe4e as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xeb1 as libc::c_int,
                last: 0xeb1 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xeb4 as libc::c_int,
                last: 0xeb9 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xebb as libc::c_int,
                last: 0xebc as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xec8 as libc::c_int,
                last: 0xecd as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xf18 as libc::c_int,
                last: 0xf19 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xf35 as libc::c_int,
                last: 0xf35 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xf37 as libc::c_int,
                last: 0xf37 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xf39 as libc::c_int,
                last: 0xf39 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xf71 as libc::c_int,
                last: 0xf7e as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xf80 as libc::c_int,
                last: 0xf84 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xf86 as libc::c_int,
                last: 0xf87 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xf90 as libc::c_int,
                last: 0xf97 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xf99 as libc::c_int,
                last: 0xfbc as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xfc6 as libc::c_int,
                last: 0xfc6 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x102d as libc::c_int,
                last: 0x1030 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1032 as libc::c_int,
                last: 0x1032 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1036 as libc::c_int,
                last: 0x1037 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1039 as libc::c_int,
                last: 0x1039 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1058 as libc::c_int,
                last: 0x1059 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1160 as libc::c_int,
                last: 0x11ff as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x135f as libc::c_int,
                last: 0x135f as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1712 as libc::c_int,
                last: 0x1714 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1732 as libc::c_int,
                last: 0x1734 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1752 as libc::c_int,
                last: 0x1753 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1772 as libc::c_int,
                last: 0x1773 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x17b4 as libc::c_int,
                last: 0x17b5 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x17b7 as libc::c_int,
                last: 0x17bd as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x17c6 as libc::c_int,
                last: 0x17c6 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x17c9 as libc::c_int,
                last: 0x17d3 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x17dd as libc::c_int,
                last: 0x17dd as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x180b as libc::c_int,
                last: 0x180d as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x18a9 as libc::c_int,
                last: 0x18a9 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1920 as libc::c_int,
                last: 0x1922 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1927 as libc::c_int,
                last: 0x1928 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1932 as libc::c_int,
                last: 0x1932 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1939 as libc::c_int,
                last: 0x193b as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1a17 as libc::c_int,
                last: 0x1a18 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1b00 as libc::c_int,
                last: 0x1b03 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1b34 as libc::c_int,
                last: 0x1b34 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1b36 as libc::c_int,
                last: 0x1b3a as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1b3c as libc::c_int,
                last: 0x1b3c as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1b42 as libc::c_int,
                last: 0x1b42 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1b6b as libc::c_int,
                last: 0x1b73 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1dc0 as libc::c_int,
                last: 0x1dca as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1dfe as libc::c_int,
                last: 0x1dff as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x200b as libc::c_int,
                last: 0x200f as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x202a as libc::c_int,
                last: 0x202e as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x2060 as libc::c_int,
                last: 0x2063 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x206a as libc::c_int,
                last: 0x206f as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x20d0 as libc::c_int,
                last: 0x20ef as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x302a as libc::c_int,
                last: 0x302f as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x3099 as libc::c_int,
                last: 0x309a as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xa806 as libc::c_int,
                last: 0xa806 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xa80b as libc::c_int,
                last: 0xa80b as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xa825 as libc::c_int,
                last: 0xa826 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xfb1e as libc::c_int,
                last: 0xfb1e as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xfe00 as libc::c_int,
                last: 0xfe0f as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xfe20 as libc::c_int,
                last: 0xfe23 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xfeff as libc::c_int,
                last: 0xfeff as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xfff9 as libc::c_int,
                last: 0xfffb as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x10a01 as libc::c_int,
                last: 0x10a03 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x10a05 as libc::c_int,
                last: 0x10a06 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x10a0c as libc::c_int,
                last: 0x10a0f as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x10a38 as libc::c_int,
                last: 0x10a3a as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x10a3f as libc::c_int,
                last: 0x10a3f as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1d167 as libc::c_int,
                last: 0x1d169 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1d173 as libc::c_int,
                last: 0x1d182 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1d185 as libc::c_int,
                last: 0x1d18b as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1d1aa as libc::c_int,
                last: 0x1d1ad as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0x1d242 as libc::c_int,
                last: 0x1d244 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xe0001 as libc::c_int,
                last: 0xe0001 as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xe0020 as libc::c_int,
                last: 0xe007f as libc::c_int,
            };
            init
        },
        {
            let mut init = interval {
                first: 0xe0100 as libc::c_int,
                last: 0xe01ef as libc::c_int,
            };
            init
        },
    ];
    return bisearch(
        c,
        combining.as_ptr(),
        (::std::mem::size_of::<[interval; 142]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<interval>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
    );
}
unsafe extern "C" fn comb_tofront(mut i: libc::c_int) {
    loop {
        let mut root: libc::c_int = if i >= 0x700 as libc::c_int {
            0x801 as libc::c_int
        } else {
            0x800 as libc::c_int
        };
        (**combchars.offset((**combchars.offset(i as isize)).prev as isize))
            .next = (**combchars.offset(i as isize)).next;
        (**combchars.offset((**combchars.offset(i as isize)).next as isize))
            .prev = (**combchars.offset(i as isize)).prev;
        (**combchars.offset(i as isize)).next = (**combchars.offset(root as isize)).next;
        (**combchars.offset(i as isize)).prev = root as libc::c_uint;
        (**combchars.offset((**combchars.offset(root as isize)).next as isize))
            .prev = i as libc::c_uint;
        (**combchars.offset(root as isize)).next = i as libc::c_uint;
        i = (**combchars.offset(i as isize)).c1 as libc::c_int;
        if i < 0xd800 as libc::c_int || i >= 0xe000 as libc::c_int {
            return;
        }
        i -= 0xd800 as libc::c_int;
    };
}
pub unsafe extern "C" fn utf8_handle_comb(mut c: libc::c_int, mut mc: *mut mchar) {
    let mut root: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut c1: libc::c_int = 0;
    let mut isdouble: libc::c_int = 0;
    c1 = (*mc).image as libc::c_int | ((*mc).font as libc::c_int) << 8 as libc::c_int
        | ((*mc).fontx as libc::c_int) << 16 as libc::c_int;
    isdouble = (c1 >= 0x1100 as libc::c_int && utf8_isdouble(c1) != 0) as libc::c_int;
    if combchars.is_null() {
        combchars = calloc(
            0x802 as libc::c_int as libc::c_ulong,
            ::std::mem::size_of::<*mut combchar>() as libc::c_ulong,
        ) as *mut *mut combchar;
        if combchars.is_null() {
            return;
        }
        let ref mut fresh12 = *combchars.offset(0x800 as libc::c_int as isize);
        *fresh12 = malloc(::std::mem::size_of::<combchar>() as libc::c_ulong)
            as *mut combchar;
        let ref mut fresh13 = *combchars.offset(0x801 as libc::c_int as isize);
        *fresh13 = malloc(::std::mem::size_of::<combchar>() as libc::c_ulong)
            as *mut combchar;
        if (*combchars.offset(0x800 as libc::c_int as isize)).is_null()
            || (*combchars.offset(0x801 as libc::c_int as isize)).is_null()
        {
            if !(*combchars.offset(0x800 as libc::c_int as isize)).is_null() {
                free(
                    *combchars.offset(0x800 as libc::c_int as isize) as *mut libc::c_void,
                );
            }
            if !(*combchars.offset(0x801 as libc::c_int as isize)).is_null() {
                free(
                    *combchars.offset(0x801 as libc::c_int as isize) as *mut libc::c_void,
                );
            }
            free(combchars as *mut libc::c_void);
            return;
        }
        (**combchars.offset(0x800 as libc::c_int as isize))
            .c1 = 0 as libc::c_int as libc::c_uint;
        (**combchars.offset(0x800 as libc::c_int as isize))
            .c2 = 0x700 as libc::c_int as libc::c_uint;
        (**combchars.offset(0x800 as libc::c_int as isize))
            .next = 0x800 as libc::c_int as libc::c_uint;
        (**combchars.offset(0x800 as libc::c_int as isize))
            .prev = 0x800 as libc::c_int as libc::c_uint;
        (**combchars.offset(0x801 as libc::c_int as isize))
            .c1 = 0x700 as libc::c_int as libc::c_uint;
        (**combchars.offset(0x801 as libc::c_int as isize))
            .c2 = 0x800 as libc::c_int as libc::c_uint;
        (**combchars.offset(0x801 as libc::c_int as isize))
            .next = 0x801 as libc::c_int as libc::c_uint;
        (**combchars.offset(0x801 as libc::c_int as isize))
            .prev = 0x801 as libc::c_int as libc::c_uint;
    }
    root = if isdouble != 0 { 0x801 as libc::c_int } else { 0x800 as libc::c_int };
    i = (**combchars.offset(root as isize)).c1 as libc::c_int;
    while (i as libc::c_uint) < (**combchars.offset(root as isize)).c2 {
        if (*combchars.offset(i as isize)).is_null() {
            break;
        }
        if (**combchars.offset(i as isize)).c1 == c1 as libc::c_uint
            && (**combchars.offset(i as isize)).c2 == c as libc::c_uint
        {
            break;
        }
        i += 1;
        i;
    }
    if i as libc::c_uint == (**combchars.offset(root as isize)).c2 {
        if c1 >= 0xd800 as libc::c_int && c1 < 0xe000 as libc::c_int {
            comb_tofront(c1 - 0xd800 as libc::c_int);
        }
        i = (**combchars.offset(root as isize)).prev as libc::c_int;
        if i == 0x800 as libc::c_int || i == 0x801 as libc::c_int
            || c1 == i + 0xd800 as libc::c_int
        {
            (*mc).image = '?' as i32 as libc::c_uchar;
            (*mc).font = 0 as libc::c_int as libc::c_uchar;
            return;
        }
    } else if (*combchars.offset(i as isize)).is_null() {
        let ref mut fresh14 = *combchars.offset(i as isize);
        *fresh14 = malloc(::std::mem::size_of::<combchar>() as libc::c_ulong)
            as *mut combchar;
        if (*combchars.offset(i as isize)).is_null() {
            return;
        }
        (**combchars.offset(i as isize)).prev = i as libc::c_uint;
        (**combchars.offset(i as isize)).next = i as libc::c_uint;
    }
    (**combchars.offset(i as isize)).c1 = c1 as libc::c_uint;
    (**combchars.offset(i as isize)).c2 = c as libc::c_uint;
    (*mc).image = (i & 0xff as libc::c_int) as libc::c_uchar;
    (*mc).font = ((i >> 8 as libc::c_int) + 0xd8 as libc::c_int) as libc::c_uchar;
    (*mc).fontx = 0 as libc::c_int as libc::c_uchar;
    comb_tofront(i);
}
unsafe extern "C" fn encmatch(
    mut s1: *mut libc::c_char,
    mut s2: *mut libc::c_char,
) -> libc::c_int {
    let mut c1: libc::c_int = 0;
    let mut c2: libc::c_int = 0;
    loop {
        c1 = *s1 as libc::c_uchar as libc::c_int;
        if c1 >= 'A' as i32 && c1 <= 'Z' as i32 {
            c1 += 'a' as i32 - 'A' as i32;
        }
        if !(c1 >= 'a' as i32 && c1 <= 'z' as i32)
            && !(c1 >= '0' as i32 && c1 <= '9' as i32)
        {
            s1 = s1.offset(1);
            s1;
        } else {
            c2 = *s2 as libc::c_uchar as libc::c_int;
            if c2 >= 'A' as i32 && c2 <= 'Z' as i32 {
                c2 += 'a' as i32 - 'A' as i32;
            }
            if !(c2 >= 'a' as i32 && c2 <= 'z' as i32)
                && !(c2 >= '0' as i32 && c2 <= '9' as i32)
            {
                s2 = s2.offset(1);
                s2;
            } else {
                if c1 != c2 {
                    return 0 as libc::c_int;
                }
                s1 = s1.offset(1);
                s1;
                s2 = s2.offset(1);
                s2;
            }
        }
        if !(c1 != 0) {
            break;
        }
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn FindEncoding(mut name: *mut libc::c_char) -> libc::c_int {
    let mut encoding: libc::c_int = 0;
    if name.is_null() || *name as libc::c_int == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    if encmatch(name, b"euc\0" as *const u8 as *const libc::c_char as *mut libc::c_char)
        != 0
    {
        name = b"eucJP\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    if encmatch(name, b"off\0" as *const u8 as *const libc::c_char as *mut libc::c_char)
        != 0
        || encmatch(
            name,
            b"iso8859-1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) != 0
    {
        return 0 as libc::c_int;
    }
    encoding = 0 as libc::c_int;
    while encoding
        < (::std::mem::size_of::<[encoding; 21]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<encoding>() as libc::c_ulong)
            as libc::c_int
    {
        if encmatch(name, encodings[encoding as usize].name) != 0 {
            LoadFontTranslationsForEncoding(encoding);
            return encoding;
        }
        encoding += 1;
        encoding;
    }
    return -(1 as libc::c_int);
}
pub unsafe extern "C" fn EncodingName(mut encoding: libc::c_int) -> *mut libc::c_char {
    if encoding
        >= (::std::mem::size_of::<[encoding; 21]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<encoding>() as libc::c_ulong)
            as libc::c_int
    {
        return 0 as *mut libc::c_char;
    }
    return encodings[encoding as usize].name;
}
pub unsafe extern "C" fn EncodingDefFont(mut encoding: libc::c_int) -> libc::c_int {
    return encodings[encoding as usize].deffont;
}
pub unsafe extern "C" fn ResetEncoding(mut p: *mut win) {
    let mut c: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut encoding: libc::c_int = (*p).w_layer.l_encoding;
    c = encodings[encoding as usize].charsets;
    if !c.is_null() {
        SetCharsets(p, c);
    }
    LoadFontTranslationsForEncoding(encoding);
    if encodings[encoding as usize].usegr != 0 {
        (*p).w_gr = 2 as libc::c_int;
        (*p)
            .w_FontE = *(encodings[encoding as usize].charsets)
            .offset(1 as libc::c_int as isize);
    } else {
        (*p).w_FontE = 0 as libc::c_int as libc::c_char;
    }
    if encodings[encoding as usize].noc1 != 0 {
        (*p).w_c1 = 0 as libc::c_int;
    }
}
pub unsafe extern "C" fn DecodeChar(
    mut c: libc::c_int,
    mut encoding: libc::c_int,
    mut statep: *mut libc::c_int,
) -> libc::c_int {
    let mut t: libc::c_int = 0;
    if encoding == 8 as libc::c_int {
        c = FromUtf8(c, statep);
        if c >= 0x10000 as libc::c_int {
            c = (c & 0x7f0000 as libc::c_int) << 8 as libc::c_int
                | c & 0xffff as libc::c_int;
        }
        return c;
    }
    if encoding == 2 as libc::c_int {
        if *statep == 0 {
            if 0x81 as libc::c_int <= c && c <= 0x9f as libc::c_int
                || 0xe0 as libc::c_int <= c && c <= 0xef as libc::c_int
            {
                *statep = c;
                return -(1 as libc::c_int);
            }
            if c < 0x80 as libc::c_int {
                return c;
            }
            return c | ('I' as i32) << 16 as libc::c_int;
        }
        t = c;
        c = *statep;
        *statep = 0 as libc::c_int;
        if 0x40 as libc::c_int <= t && t <= 0xfc as libc::c_int
            && t != 0x7f as libc::c_int
        {
            if c <= 0x9f as libc::c_int {
                c = (c - 0x81 as libc::c_int) * 2 as libc::c_int + 0x21 as libc::c_int;
            } else {
                c = (c - 0xc1 as libc::c_int) * 2 as libc::c_int + 0x21 as libc::c_int;
            }
            if t <= 0x7e as libc::c_int {
                t -= 0x1f as libc::c_int;
            } else if t <= 0x9e as libc::c_int {
                t -= 0x20 as libc::c_int;
            } else {
                t -= 0x7e as libc::c_int;
                c += 1;
                c;
            }
            return c << 8 as libc::c_int | t
                | ('B' as i32 & 0o37 as libc::c_int) << 16 as libc::c_int;
        }
        return t;
    }
    if encoding == 1 as libc::c_int || encoding == 3 as libc::c_int
        || encoding == 4 as libc::c_int
    {
        if *statep == 0 {
            if c & 0x80 as libc::c_int != 0 {
                *statep = c;
                return -(1 as libc::c_int);
            }
            return c;
        }
        t = c;
        c = *statep;
        *statep = 0 as libc::c_int;
        if encoding == 1 as libc::c_int {
            if c == 0x8e as libc::c_int {
                return t | ('I' as i32) << 16 as libc::c_int;
            }
            if c == 0x8f as libc::c_int {
                *statep = t | ('D' as i32 & 0o37 as libc::c_int) << 8 as libc::c_int;
                return -(1 as libc::c_int);
            }
        }
        c &= 0xff7f as libc::c_int;
        t &= 0x7f as libc::c_int;
        c = c << 8 as libc::c_int | t;
        if encoding == 3 as libc::c_int {
            return c | (3 as libc::c_int) << 16 as libc::c_int;
        }
        if encoding == 4 as libc::c_int {
            return c | (1 as libc::c_int) << 16 as libc::c_int;
        }
        if c & ('D' as i32 & 0o37 as libc::c_int) << 16 as libc::c_int != 0 {
            return c
        } else {
            return c | ('B' as i32 & 0o37 as libc::c_int) << 16 as libc::c_int
        }
    }
    if encoding == 5 as libc::c_int || encoding == 20 as libc::c_int {
        if *statep == 0 {
            if c & 0x80 as libc::c_int != 0 {
                if encoding == 20 as libc::c_int && c == 0x80 as libc::c_int {
                    return 0xa4 as libc::c_int
                        | ('b' as i32 | 0x80 as libc::c_int) << 16 as libc::c_int;
                }
                *statep = c;
                return -(1 as libc::c_int);
            }
            return c;
        }
        t = c;
        c = *statep;
        *statep = 0 as libc::c_int;
        c &= 0x7f as libc::c_int;
        return c << 8 as libc::c_int | t
            | (if encoding == 5 as libc::c_int {
                (0o30 as libc::c_int) << 16 as libc::c_int
            } else {
                (0o31 as libc::c_int) << 16 as libc::c_int
            });
    }
    return c | encodings[encoding as usize].deffont << 16 as libc::c_int;
}
pub unsafe extern "C" fn EncodeChar(
    mut bp: *mut libc::c_char,
    mut c: libc::c_int,
    mut encoding: libc::c_int,
    mut fontp: *mut libc::c_int,
) -> libc::c_int {
    let mut t: libc::c_int = 0;
    let mut f: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    if c == -(1 as libc::c_int) && !fontp.is_null() {
        if *fontp == 0 as libc::c_int {
            return 0 as libc::c_int;
        }
        if !bp.is_null() {
            let fresh15 = bp;
            bp = bp.offset(1);
            *fresh15 = 0o33 as libc::c_int as libc::c_char;
            let fresh16 = bp;
            bp = bp.offset(1);
            *fresh16 = '(' as i32 as libc::c_char;
            let fresh17 = bp;
            bp = bp.offset(1);
            *fresh17 = 'B' as i32 as libc::c_char;
        }
        return 3 as libc::c_int;
    }
    f = c >> 16 as libc::c_int & 0xff as libc::c_int;
    if encoding == 8 as libc::c_int {
        if f != 0 {
            if f != 0 && f & 0x60 as libc::c_int == 0 as libc::c_int {
                let mut c2: libc::c_int = c & 0xff as libc::c_int;
                c = c >> 8 as libc::c_int & 0xff as libc::c_int | f << 8 as libc::c_int;
                c = recode_char_dw_to_encoding(c, &mut c2, encoding);
            } else {
                c = c & 0xff as libc::c_int | f << 8 as libc::c_int;
                c = recode_char_to_encoding(c, encoding);
            }
        }
        return ToUtf8(bp, c);
    }
    if f == 0 as libc::c_int && c & 0x7f00ff00 as libc::c_int != 0 as libc::c_int {
        if c >= 0x10000 as libc::c_int {
            c = (c & 0x7f0000 as libc::c_int) >> 8 as libc::c_int
                | c & 0xffff as libc::c_int;
        }
        if utf8_isdouble(c) != 0 {
            let mut c2_0: libc::c_int = 0xffff as libc::c_int;
            c = recode_char_dw_to_encoding(c, &mut c2_0, encoding);
            c = c << 8 as libc::c_int | c2_0 & 0xff as libc::c_int;
        } else {
            c = recode_char_to_encoding(c, encoding);
            c = (c & 0xff00 as libc::c_int) << 8 as libc::c_int
                | c & 0xff as libc::c_int;
        }
        f = c >> 16 as libc::c_int;
    }
    if f & 0x80 as libc::c_int != 0 {
        f = 0 as libc::c_int;
    }
    if encoding == 2 as libc::c_int {
        if f == 'I' as i32 {
            c = c & 0xff as libc::c_int | 0x80 as libc::c_int;
        } else if f == 'B' as i32 & 0o37 as libc::c_int {
            if bp.is_null() {
                return 2 as libc::c_int;
            }
            t = c & 0xff as libc::c_int;
            c = c >> 8 as libc::c_int & 0xff as libc::c_int;
            t
                += if c & 1 as libc::c_int != 0 {
                    if t <= 0x5f as libc::c_int {
                        0x1f as libc::c_int
                    } else {
                        0x20 as libc::c_int
                    }
                } else {
                    0x7e as libc::c_int
                };
            c = (c - 0x21 as libc::c_int) / 2 as libc::c_int
                + (if c < 0x5f as libc::c_int {
                    0x81 as libc::c_int
                } else {
                    0xc1 as libc::c_int
                });
            let fresh18 = bp;
            bp = bp.offset(1);
            *fresh18 = c as libc::c_char;
            let fresh19 = bp;
            bp = bp.offset(1);
            *fresh19 = t as libc::c_char;
            return 2 as libc::c_int;
        }
    }
    if encoding == 1 as libc::c_int {
        if f == 'I' as i32 {
            if !bp.is_null() {
                let fresh20 = bp;
                bp = bp.offset(1);
                *fresh20 = 0x8e as libc::c_int as libc::c_char;
                let fresh21 = bp;
                bp = bp.offset(1);
                *fresh21 = c as libc::c_char;
            }
            return 2 as libc::c_int;
        }
        if f == 'B' as i32 & 0o37 as libc::c_int {
            if !bp.is_null() {
                let fresh22 = bp;
                bp = bp.offset(1);
                *fresh22 = (c >> 8 as libc::c_int | 0x80 as libc::c_int) as libc::c_char;
                let fresh23 = bp;
                bp = bp.offset(1);
                *fresh23 = (c | 0x80 as libc::c_int) as libc::c_char;
            }
            return 2 as libc::c_int;
        }
        if f == 'D' as i32 & 0o37 as libc::c_int {
            if !bp.is_null() {
                let fresh24 = bp;
                bp = bp.offset(1);
                *fresh24 = 0x8f as libc::c_int as libc::c_char;
                let fresh25 = bp;
                bp = bp.offset(1);
                *fresh25 = (c >> 8 as libc::c_int) as libc::c_char;
                let fresh26 = bp;
                bp = bp.offset(1);
                *fresh26 = c as libc::c_char;
            }
            return 3 as libc::c_int;
        }
    }
    if encoding == 3 as libc::c_int && f == 3 as libc::c_int
        || encoding == 4 as libc::c_int && f == 1 as libc::c_int
    {
        if !bp.is_null() {
            let fresh27 = bp;
            bp = bp.offset(1);
            *fresh27 = (c >> 8 as libc::c_int | 0x80 as libc::c_int) as libc::c_char;
            let fresh28 = bp;
            bp = bp.offset(1);
            *fresh28 = (c | 0x80 as libc::c_int) as libc::c_char;
        }
        return 2 as libc::c_int;
    }
    if encoding == 5 as libc::c_int && f == 0o30 as libc::c_int
        || encoding == 20 as libc::c_int && f == 0o31 as libc::c_int
    {
        if !bp.is_null() {
            let fresh29 = bp;
            bp = bp.offset(1);
            *fresh29 = (c >> 8 as libc::c_int | 0x80 as libc::c_int) as libc::c_char;
            let fresh30 = bp;
            bp = bp.offset(1);
            *fresh30 = c as libc::c_char;
        }
        return 2 as libc::c_int;
    }
    if encoding == 20 as libc::c_int && f == 0 as libc::c_int && c == 0xa4 as libc::c_int
    {
        c = 0x80 as libc::c_int;
    }
    l = 0 as libc::c_int;
    if !fontp.is_null() && f != *fontp {
        *fontp = f;
        if f != 0 && f < ' ' as i32 {
            if !bp.is_null() {
                let fresh31 = bp;
                bp = bp.offset(1);
                *fresh31 = 0o33 as libc::c_int as libc::c_char;
                let fresh32 = bp;
                bp = bp.offset(1);
                *fresh32 = '$' as i32 as libc::c_char;
                if f > 2 as libc::c_int {
                    let fresh33 = bp;
                    bp = bp.offset(1);
                    *fresh33 = '(' as i32 as libc::c_char;
                }
                let fresh34 = bp;
                bp = bp.offset(1);
                *fresh34 = ('@' as i32 + f) as libc::c_char;
            }
            l += if f > 2 as libc::c_int { 4 as libc::c_int } else { 3 as libc::c_int };
        } else if f < 128 as libc::c_int {
            if f == 0 as libc::c_int {
                f = 'B' as i32;
            }
            if !bp.is_null() {
                let fresh35 = bp;
                bp = bp.offset(1);
                *fresh35 = 0o33 as libc::c_int as libc::c_char;
                let fresh36 = bp;
                bp = bp.offset(1);
                *fresh36 = '(' as i32 as libc::c_char;
                let fresh37 = bp;
                bp = bp.offset(1);
                *fresh37 = f as libc::c_char;
            }
            l += 3 as libc::c_int;
        }
    }
    if c & 0xff00 as libc::c_int != 0 {
        if !bp.is_null() {
            let fresh38 = bp;
            bp = bp.offset(1);
            *fresh38 = (c >> 8 as libc::c_int) as libc::c_char;
        }
        l += 1;
        l;
    }
    if !bp.is_null() {
        let fresh39 = bp;
        bp = bp.offset(1);
        *fresh39 = c as libc::c_char;
    }
    return l + 1 as libc::c_int;
}
pub unsafe extern "C" fn CanEncodeFont(
    mut encoding: libc::c_int,
    mut f: libc::c_int,
) -> libc::c_int {
    match encoding {
        8 => return 1 as libc::c_int,
        2 => {
            return (f == 'B' as i32 & 0o37 as libc::c_int || f == 'I' as i32)
                as libc::c_int;
        }
        1 => {
            return (f == 'B' as i32 & 0o37 as libc::c_int || f == 'I' as i32
                || f == 'D' as i32 & 0o37 as libc::c_int) as libc::c_int;
        }
        3 => return (f == 3 as libc::c_int) as libc::c_int,
        4 => return (f == 1 as libc::c_int) as libc::c_int,
        5 => return (f == 0o30 as libc::c_int) as libc::c_int,
        20 => return (f == 0o31 as libc::c_int) as libc::c_int,
        _ => {}
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn PrepareEncodedChar(mut c: libc::c_int) -> libc::c_int {
    let mut encoding: libc::c_int = 0;
    let mut t: libc::c_int = 0 as libc::c_int;
    let mut f: libc::c_int = 0;
    encoding = (*display).d_encoding;
    f = (*display).d_rend.font as libc::c_int;
    t = (*display).d_mbcs;
    if encoding == 2 as libc::c_int {
        if f == 'I' as i32 {
            return c | 0x80 as libc::c_int
        } else if f == 'B' as i32 & 0o37 as libc::c_int {
            t
                += if c & 1 as libc::c_int != 0 {
                    if t <= 0x5f as libc::c_int {
                        0x1f as libc::c_int
                    } else {
                        0x20 as libc::c_int
                    }
                } else {
                    0x7e as libc::c_int
                };
            c = (c - 0x21 as libc::c_int) / 2 as libc::c_int
                + (if c < 0x5f as libc::c_int {
                    0x81 as libc::c_int
                } else {
                    0xc1 as libc::c_int
                });
            (*display).d_mbcs = t;
        }
        return c;
    }
    if encoding == 1 as libc::c_int {
        if f == 'I' as i32 {
            (*display).d_obuffree -= 1;
            if (*display).d_obuffree <= 0 as libc::c_int {
                Resize_obuf();
            }
            let fresh40 = (*display).d_obufp;
            (*display).d_obufp = ((*display).d_obufp).offset(1);
            *fresh40 = 0x8e as libc::c_int as libc::c_char;
            return c | 0x80 as libc::c_int;
        }
        if f == 'B' as i32 & 0o37 as libc::c_int {
            (*display).d_mbcs = t | 0x80 as libc::c_int;
            return c | 0x80 as libc::c_int;
        }
        if f == 'D' as i32 & 0o37 as libc::c_int {
            (*display).d_obuffree -= 1;
            if (*display).d_obuffree <= 0 as libc::c_int {
                Resize_obuf();
            }
            let fresh41 = (*display).d_obufp;
            (*display).d_obufp = ((*display).d_obufp).offset(1);
            *fresh41 = 0x8f as libc::c_int as libc::c_char;
            (*display).d_mbcs = t | 0x80 as libc::c_int;
            return c | 0x80 as libc::c_int;
        }
    }
    if encoding == 3 as libc::c_int && f == 3 as libc::c_int
        || encoding == 4 as libc::c_int && f == 1 as libc::c_int
    {
        (*display).d_mbcs = t | 0x80 as libc::c_int;
        return c | 0x80 as libc::c_int;
    }
    if encoding == 5 as libc::c_int && f == 0o30 as libc::c_int
        || encoding == 20 as libc::c_int && f == 0o31 as libc::c_int
    {
        return c | 0x80 as libc::c_int;
    }
    return c;
}
pub unsafe extern "C" fn RecodeBuf(
    mut fbuf: *mut libc::c_uchar,
    mut flen: libc::c_int,
    mut fenc: libc::c_int,
    mut tenc: libc::c_int,
    mut tbuf: *mut libc::c_uchar,
) -> libc::c_int {
    let mut c: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut decstate: libc::c_int = 0 as libc::c_int;
    let mut font: libc::c_int = 0 as libc::c_int;
    j = 0 as libc::c_int;
    i = j;
    while i < flen {
        c = *fbuf.offset(i as isize) as libc::c_int;
        c = DecodeChar(c, fenc, &mut decstate);
        if c == -(2 as libc::c_int) {
            i -= 1;
            i;
        }
        if !(c < 0 as libc::c_int) {
            j
                += EncodeChar(
                    if !tbuf.is_null() {
                        (tbuf as *mut libc::c_char).offset(j as isize)
                    } else {
                        0 as *mut libc::c_char
                    },
                    c,
                    tenc,
                    &mut font,
                );
        }
        i += 1;
        i;
    }
    j
        += EncodeChar(
            if !tbuf.is_null() {
                (tbuf as *mut libc::c_char).offset(j as isize)
            } else {
                0 as *mut libc::c_char
            },
            -(1 as libc::c_int),
            tenc,
            &mut font,
        );
    return j;
}
pub unsafe extern "C" fn ContainsSpecialDeffont(
    mut ml: *mut mline,
    mut xs: libc::c_int,
    mut xe: libc::c_int,
    mut encoding: libc::c_int,
) -> libc::c_int {
    let mut f: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut i: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut c: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut dx: libc::c_int = 0;
    if encoding == 8 as libc::c_int
        || encodings[encoding as usize].deffont == 0 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    i = ((*ml).image).offset(xs as isize);
    f = ((*ml).font).offset(xs as isize);
    dx = xe - xs + 1 as libc::c_int;
    loop {
        let fresh42 = dx;
        dx = dx - 1;
        if !(fresh42 > 0 as libc::c_int) {
            break;
        }
        let fresh43 = f;
        f = f.offset(1);
        if *fresh43 != 0 {
            continue;
        }
        let fresh44 = i;
        i = i.offset(1);
        c = *fresh44 as libc::c_int;
        x = recode_char_to_encoding(
            c | encodings[encoding as usize].deffont << 8 as libc::c_int,
            8 as libc::c_int,
        );
        if c != x {
            return 1 as libc::c_int;
        }
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn LoadFontTranslation(
    mut font: libc::c_int,
    mut file: *mut libc::c_char,
) -> libc::c_int {
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut myfile: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut f: *mut FILE = 0 as *mut FILE;
    let mut i: libc::c_int = 0;
    let mut fo: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut u: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut ok: libc::c_int = 0;
    let mut p: *mut [libc::c_ushort; 2] = 0 as *mut [libc::c_ushort; 2];
    let mut tab: *mut [libc::c_ushort; 2] = 0 as *mut [libc::c_ushort; 2];
    myfile = file;
    if myfile.is_null() {
        if font == 0 as libc::c_int || screenencodings.is_null() {
            return -(1 as libc::c_int);
        }
        if strlen(screenencodings)
            > (::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong)
                .wrapping_sub(10 as libc::c_int as libc::c_ulong)
        {
            return -(1 as libc::c_int);
        }
        sprintf(
            buf.as_mut_ptr(),
            b"%s/%02x\0" as *const u8 as *const libc::c_char,
            screenencodings,
            font & 0xff as libc::c_int,
        );
        myfile = buf.as_mut_ptr();
    }
    f = secfopen(
        myfile,
        b"r\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if f.is_null() {
        return -(1 as libc::c_int);
    }
    ok = 0 as libc::c_int;
    i = ok;
    loop {
        while i < 12 as libc::c_int {
            if getc(f)
                != (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"ScreenI2UTF8\0"))[i as usize] as libc::c_int
            {
                break;
            }
            i += 1;
            i;
        }
        if getc(f) != 0 as libc::c_int {
            break;
        }
        fo = getc(f);
        if fo == -(1 as libc::c_int) {
            break;
        }
        if font != -(1 as libc::c_int) && font != fo {
            break;
        }
        i = getc(f);
        x = getc(f);
        if x == -(1 as libc::c_int) {
            break;
        }
        i = i << 8 as libc::c_int | x;
        getc(f);
        loop {
            x = getc(f);
            if !(x != 0 && x != -(1 as libc::c_int)) {
                break;
            }
            getc(f);
        }
        p = malloc(
            (::std::mem::size_of::<[libc::c_ushort; 2]>() as libc::c_ulong)
                .wrapping_mul((i + 1 as libc::c_int) as libc::c_ulong),
        ) as *mut [libc::c_ushort; 2];
        if p.is_null() {
            break;
        }
        tab = p;
        while i > 0 as libc::c_int {
            x = getc(f);
            x = x << 8 as libc::c_int | getc(f);
            u = getc(f);
            c = getc(f);
            u = u << 8 as libc::c_int | c;
            if c == -(1 as libc::c_int) {
                break;
            }
            (*p)[0 as libc::c_int as usize] = x as libc::c_ushort;
            (*p)[1 as libc::c_int as usize] = u as libc::c_ushort;
            p = p.offset(1);
            p;
            i -= 1;
            i;
        }
        (*p)[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_ushort;
        (*p)[1 as libc::c_int as usize] = 0 as libc::c_int as libc::c_ushort;
        if i != 0
            || (*tab.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize]
                as libc::c_int & 0x8000 as libc::c_int != 0
        {
            free(tab as *mut libc::c_void);
            break;
        } else {
            if !(recodetabs[fo as usize].tab).is_null()
                && recodetabs[fo as usize].flags & 1 as libc::c_int != 0 as libc::c_int
            {
                free(recodetabs[fo as usize].tab as *mut libc::c_void);
            }
            recodetabs[fo as usize].tab = tab;
            recodetabs[fo as usize].flags = 1 as libc::c_int;
            c = getc(f);
            if c == -(1 as libc::c_int) {
                ok = 1 as libc::c_int;
                break;
            } else {
                if c != 'S' as i32 {
                    break;
                }
                i = 1 as libc::c_int;
            }
        }
    }
    fclose(f);
    if font != -(1 as libc::c_int) && file.is_null()
        && recodetabs[font as usize].flags == 0 as libc::c_int
    {
        recodetabs[font as usize].flags = 4 as libc::c_int;
    }
    return if ok != 0 { 0 as libc::c_int } else { -(1 as libc::c_int) };
}
pub unsafe extern "C" fn LoadFontTranslationsForEncoding(mut encoding: libc::c_int) {
    let mut c: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut f: libc::c_int = 0;
    c = encodings[encoding as usize].fontlist;
    if !c.is_null() {
        loop {
            let fresh45 = c;
            c = c.offset(1);
            f = *fresh45 as libc::c_uchar as libc::c_int;
            if !(f != 0 as libc::c_int) {
                break;
            }
            if recodetabs[f as usize].flags == 0 as libc::c_int {
                LoadFontTranslation(f, 0 as *mut libc::c_char);
            }
        }
    }
    f = encodings[encoding as usize].deffont;
    if f > 0 as libc::c_int && recodetabs[f as usize].flags == 0 as libc::c_int {
        LoadFontTranslation(f, 0 as *mut libc::c_char);
    }
}
