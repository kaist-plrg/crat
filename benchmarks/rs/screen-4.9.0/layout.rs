use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type logfile;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn SetCanvasWindow(_: *mut canvas, _: *mut win);
    fn MakeDefaultCanvas() -> libc::c_int;
    fn FreeCanvas(_: *mut canvas);
    fn ResizeCanvas(_: *mut canvas);
    fn RecreateCanvasChain();
    fn DupLayoutCv(_: *mut canvas, _: *mut canvas, _: libc::c_int);
    fn PutWindowCv(_: *mut canvas);
    fn RethinkDisplayViewports() -> libc::c_int;
    fn Msg(_: libc::c_int, _: *const libc::c_char, _: ...);
    fn secfopen(_: *mut libc::c_char, _: *mut libc::c_char) -> *mut FILE;
    fn Activate(_: libc::c_int);
    fn ResizeLayersToCanvases();
    fn SaveStr(_: *const libc::c_char) -> *mut libc::c_char;
    fn KillLayerChain(_: *mut layer);
    static mut display: *mut display;
    static mut captionalways: libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __int32_t = libc::c_int;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
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
pub static mut layouts: *mut layout = 0 as *const layout as *mut layout;
pub static mut laytab: [*mut layout; 10] = [0 as *const layout as *mut layout; 10];
pub static mut layout_last: *mut layout = 0 as *const layout as *mut layout;
pub static mut layout_last_marker: layout = layout {
    lay_next: 0 as *const layout as *mut layout,
    lay_title: 0 as *const libc::c_char as *mut libc::c_char,
    lay_number: 0,
    lay_canvas: canvas {
        c_next: 0 as *const canvas as *mut canvas,
        c_display: 0 as *const display as *mut display,
        c_slnext: 0 as *const canvas as *mut canvas,
        c_slprev: 0 as *const canvas as *mut canvas,
        c_slperp: 0 as *const canvas as *mut canvas,
        c_slback: 0 as *const canvas as *mut canvas,
        c_slorient: 0,
        c_slweight: 0,
        c_vplist: 0 as *const viewport as *mut viewport,
        c_layer: 0 as *const layer as *mut layer,
        c_lnext: 0 as *const canvas as *mut canvas,
        c_blank: layer {
            l_cvlist: 0 as *const canvas as *mut canvas,
            l_width: 0,
            l_height: 0,
            l_x: 0,
            l_y: 0,
            l_encoding: 0,
            l_layfn: 0 as *const LayFuncs as *mut LayFuncs,
            l_data: 0 as *const libc::c_void as *mut libc::c_void,
            l_next: 0 as *const layer as *mut layer,
            l_bottom: 0 as *const layer as *mut layer,
            l_blocking: 0,
            l_mode: 0,
            l_mouseevent: C2RustUnnamed_1 {
                buffer: [0; 3],
                len: 0,
                start: 0,
            },
            l_pause: C2RustUnnamed_0 {
                d: [0; 1],
                c2rust_padding: [0; 7],
                left: 0 as *const libc::c_int as *mut libc::c_int,
                right: 0 as *const libc::c_int as *mut libc::c_int,
                top: 0,
                bottom: 0,
                lines: 0,
            },
        },
        c_xoff: 0,
        c_yoff: 0,
        c_xs: 0,
        c_xe: 0,
        c_ys: 0,
        c_ye: 0,
        c_captev: event {
            next: 0 as *const event as *mut event,
            handler: None,
            data: 0 as *const libc::c_char as *mut libc::c_char,
            fd: 0,
            type_0: 0,
            pri: 0,
            timeout: timeval { tv_sec: 0, tv_usec: 0 },
            queued: 0,
            active: 0,
            condpos: 0 as *const libc::c_int as *mut libc::c_int,
            condneg: 0 as *const libc::c_int as *mut libc::c_int,
        },
    },
    lay_forecv: 0 as *const canvas as *mut canvas,
    lay_cvlist: 0 as *const canvas as *mut canvas,
    lay_autosave: 0,
};
pub static mut layout_attach: *mut layout = unsafe {
    &layout_last_marker as *const layout as *mut layout
};
pub unsafe extern "C" fn FreeLayoutCv(mut cv: *mut canvas) {
    let mut cnext: *mut canvas = 0 as *mut canvas;
    let mut c: *mut canvas = cv;
    while !cv.is_null() {
        if !((*cv).c_slperp).is_null() {
            FreeLayoutCv((*cv).c_slperp);
            free((*cv).c_slperp as *mut libc::c_void);
            (*cv).c_slperp = 0 as *mut canvas;
        }
        cnext = (*cv).c_slnext;
        (*cv).c_slnext = 0 as *mut canvas;
        if cv != c {
            free(cv as *mut libc::c_void);
        }
        cv = cnext;
    }
}
pub unsafe extern "C" fn CreateLayout(
    mut title: *mut libc::c_char,
    mut startat: libc::c_int,
) -> *mut layout {
    let mut lay: *mut layout = 0 as *mut layout;
    let mut pl: *mut *mut layout = 0 as *mut *mut layout;
    let mut i: libc::c_int = 0;
    if startat >= 10 as libc::c_int || startat < 0 as libc::c_int {
        startat = 0 as libc::c_int;
    }
    i = startat;
    while !(laytab[i as usize]).is_null() {
        i += 1;
        if i == 10 as libc::c_int {
            i = 0 as libc::c_int;
        }
        if i == startat {
            Msg(
                0 as libc::c_int,
                b"No more layouts\n\0" as *const u8 as *const libc::c_char,
            );
            return 0 as *mut layout;
        }
    }
    lay = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<layout>() as libc::c_ulong,
    ) as *mut layout;
    (*lay).lay_title = SaveStr(title);
    (*lay).lay_autosave = 1 as libc::c_int;
    (*lay).lay_number = i;
    laytab[i as usize] = lay;
    (*lay).lay_next = 0 as *mut layout;
    pl = &mut layouts;
    while !(*pl).is_null() {
        pl = &mut (**pl).lay_next;
    }
    *pl = lay;
    return lay;
}
pub unsafe extern "C" fn SaveLayout(mut name: *mut libc::c_char, mut cv: *mut canvas) {
    let mut lay: *mut layout = 0 as *mut layout;
    let mut fcv: *mut canvas = 0 as *mut canvas;
    lay = layouts;
    while !lay.is_null() {
        if strcmp((*lay).lay_title, name) == 0 {
            break;
        }
        lay = (*lay).lay_next;
    }
    if !lay.is_null() {
        FreeLayoutCv(&mut (*lay).lay_canvas);
    } else {
        lay = CreateLayout(name, 0 as libc::c_int);
    }
    if lay.is_null() {
        return;
    }
    fcv = (*display).d_forecv;
    DupLayoutCv(cv, &mut (*lay).lay_canvas, 1 as libc::c_int);
    (*lay).lay_forecv = (*display).d_forecv;
    (*display).d_forecv = fcv;
    (*display).d_layout = lay;
}
pub unsafe extern "C" fn AutosaveLayout(mut lay: *mut layout) {
    let mut fcv: *mut canvas = 0 as *mut canvas;
    if lay.is_null() || (*lay).lay_autosave == 0 {
        return;
    }
    FreeLayoutCv(&mut (*lay).lay_canvas);
    fcv = (*display).d_forecv;
    DupLayoutCv(&mut (*display).d_canvas, &mut (*lay).lay_canvas, 1 as libc::c_int);
    (*lay).lay_forecv = (*display).d_forecv;
    (*display).d_forecv = fcv;
}
pub unsafe extern "C" fn FindLayout(mut name: *mut libc::c_char) -> *mut layout {
    let mut lay: *mut layout = 0 as *mut layout;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    s = name;
    while *s as libc::c_int >= '0' as i32 && *s as libc::c_int <= '9' as i32 {
        i = i * 10 as libc::c_int + (*s as libc::c_int - '0' as i32);
        s = s.offset(1);
        s;
    }
    if *s == 0 && s != name && i >= 0 as libc::c_int && i < 10 as libc::c_int {
        return laytab[i as usize];
    }
    lay = layouts;
    while !lay.is_null() {
        if strcmp((*lay).lay_title, name) == 0 {
            break;
        }
        lay = (*lay).lay_next;
    }
    return lay;
}
pub unsafe extern "C" fn LoadLayout(mut lay: *mut layout, mut cv: *mut canvas) {
    AutosaveLayout((*display).d_layout);
    if lay.is_null() {
        while !((*display).d_canvas.c_slperp).is_null() {
            FreeCanvas((*display).d_canvas.c_slperp);
        }
        MakeDefaultCanvas();
        SetCanvasWindow((*display).d_forecv, 0 as *mut win);
        (*display).d_layout = 0 as *mut layout;
        return;
    }
    while !((*display).d_canvas.c_slperp).is_null() {
        FreeCanvas((*display).d_canvas.c_slperp);
    }
    (*display).d_cvlist = 0 as *mut canvas;
    (*display).d_forecv = (*lay).lay_forecv;
    if ((*display).d_forecv).is_null() {
        MakeDefaultCanvas();
    }
    DupLayoutCv(&mut (*lay).lay_canvas, &mut (*display).d_canvas, 0 as libc::c_int);
    (*display)
        .d_canvas
        .c_ys = ((*display).d_has_hstatus == 4 as libc::c_int) as libc::c_int;
    (*display)
        .d_canvas
        .c_ye = (*display).d_height - 1 as libc::c_int
        - (!((*display).d_canvas.c_slperp).is_null()
            && !((*(*display).d_canvas.c_slperp).c_slnext).is_null()
            || captionalways != 0) as libc::c_int
        - ((*display).d_has_hstatus == 1 as libc::c_int) as libc::c_int;
    ResizeCanvas(&mut (*display).d_canvas);
    RecreateCanvasChain();
    RethinkDisplayViewports();
    PutWindowCv(&mut (*display).d_canvas);
    ResizeLayersToCanvases();
    (*display).d_layout = lay;
}
pub unsafe extern "C" fn NewLayout(
    mut title: *mut libc::c_char,
    mut startat: libc::c_int,
) {
    let mut lay: *mut layout = 0 as *mut layout;
    let mut fcv: *mut canvas = 0 as *mut canvas;
    lay = CreateLayout(title, startat);
    if lay.is_null() {
        return;
    }
    if !display.is_null() {
        LoadLayout(0 as *mut layout, &mut (*display).d_canvas);
        fcv = (*display).d_forecv;
        DupLayoutCv(&mut (*display).d_canvas, &mut (*lay).lay_canvas, 1 as libc::c_int);
        (*lay).lay_forecv = (*display).d_forecv;
        (*display).d_forecv = fcv;
        (*display).d_layout = lay;
    } else {
        layout_attach = lay;
    }
    (*lay).lay_autosave = 1 as libc::c_int;
}
unsafe extern "C" fn AddLayoutsInfo(
    mut buf: *mut libc::c_char,
    mut len: libc::c_int,
    mut where_0: libc::c_int,
) -> *mut libc::c_char {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ss: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut layout = 0 as *mut layout;
    let mut pp: *mut *mut layout = 0 as *mut *mut layout;
    let mut l: libc::c_int = 0;
    ss = buf;
    s = ss;
    pp = laytab.as_mut_ptr();
    while pp < laytab.as_mut_ptr().offset(10 as libc::c_int as isize) {
        if pp.offset_from(laytab.as_mut_ptr()) as libc::c_long == where_0 as libc::c_long
            && ss == buf
        {
            ss = s;
        }
        p = *pp;
        if !p.is_null() {
            t = (*p).lay_title;
            l = strlen(t) as libc::c_int;
            if l > 20 as libc::c_int {
                l = 20 as libc::c_int;
            }
            if s.offset_from(buf) as libc::c_long + l as libc::c_long
                > (len - 24 as libc::c_int) as libc::c_long
            {
                break;
            }
            if s > buf {
                let fresh0 = s;
                s = s.offset(1);
                *fresh0 = ' ' as i32 as libc::c_char;
                let fresh1 = s;
                s = s.offset(1);
                *fresh1 = ' ' as i32 as libc::c_char;
            }
            sprintf(s, b"%d\0" as *const u8 as *const libc::c_char, (*p).lay_number);
            if (*p).lay_number == where_0 {
                ss = s;
            }
            s = s.offset(strlen(s) as isize);
            if !display.is_null() && p == (*display).d_layout {
                let fresh2 = s;
                s = s.offset(1);
                *fresh2 = '*' as i32 as libc::c_char;
            }
            let fresh3 = s;
            s = s.offset(1);
            *fresh3 = ' ' as i32 as libc::c_char;
            strncpy(s, t, l as libc::c_ulong);
            s = s.offset(l as isize);
        }
        pp = pp.offset(1);
        pp;
    }
    *s = 0 as libc::c_int as libc::c_char;
    return ss;
}
pub unsafe extern "C" fn ShowLayouts(mut where_0: libc::c_int) {
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ss: *mut libc::c_char = 0 as *mut libc::c_char;
    if display.is_null() {
        return;
    }
    if layouts.is_null() {
        Msg(
            0 as libc::c_int,
            b"No layouts defined\n\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if where_0 == -(1 as libc::c_int) && !((*display).d_layout).is_null() {
        where_0 = (*(*display).d_layout).lay_number;
    }
    ss = AddLayoutsInfo(
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
        where_0,
    );
    s = buf.as_mut_ptr().offset(strlen(buf.as_mut_ptr()) as isize);
    if ss.offset_from(buf.as_mut_ptr()) as libc::c_long
        > ((*display).d_width / 2 as libc::c_int) as libc::c_long
    {
        ss = ss.offset(-(((*display).d_width / 2 as libc::c_int) as isize));
        if (s.offset_from(ss) as libc::c_long) < (*display).d_width as libc::c_long {
            ss = s.offset(-((*display).d_width as isize));
            if ss < buf.as_mut_ptr() {
                ss = buf.as_mut_ptr();
            }
        }
    } else {
        ss = buf.as_mut_ptr();
    }
    Msg(0 as libc::c_int, b"%s\0" as *const u8 as *const libc::c_char, ss);
}
pub unsafe extern "C" fn RemoveLayout(mut lay: *mut layout) {
    let mut layp: *mut *mut layout = &mut layouts;
    while !(*layp).is_null() {
        if *layp == lay {
            *layp = (*lay).lay_next;
            break;
        } else {
            layp = &mut (**layp).lay_next;
        }
    }
    laytab[(*lay).lay_number as usize] = 0 as *mut layout;
    if !display.is_null() && (*display).d_layout == lay {
        (*display).d_layout = 0 as *mut layout;
    }
    FreeLayoutCv(&mut (*lay).lay_canvas);
    if !((*lay).lay_title).is_null() {
        free((*lay).lay_title as *mut libc::c_void);
    }
    free(lay as *mut libc::c_void);
    if !layouts.is_null() {
        LoadLayout(
            if !display.is_null() && !((*display).d_layout).is_null() {
                (*display).d_layout
            } else if !(*layp).is_null() {
                *layp
            } else {
                layouts
            },
            if !display.is_null() { &mut (*display).d_canvas } else { 0 as *mut canvas },
        );
    }
    Activate(0 as libc::c_int);
}
pub unsafe extern "C" fn UpdateLayoutCanvas(mut cv: *mut canvas, mut wi: *mut win) {
    while !cv.is_null() {
        if !((*cv).c_layer).is_null()
            && (*(*(*cv).c_layer).l_bottom).l_data as *mut win == wi
        {
            let mut l: *mut layer = (*cv).c_layer;
            (*cv).c_layer = 0 as *mut layer;
            if ((*l).l_cvlist).is_null() && (wi.is_null() || l != (*wi).w_savelayer) {
                KillLayerChain(l);
            }
            l = &mut (*cv).c_blank;
            (*l).l_data = 0 as *mut libc::c_void;
            if (*l).l_cvlist != cv {
                (*cv).c_lnext = (*l).l_cvlist;
                (*l).l_cvlist = cv;
            }
            (*cv).c_layer = l;
        }
        if !((*cv).c_slperp).is_null() {
            UpdateLayoutCanvas((*cv).c_slperp, wi);
        }
        cv = (*cv).c_slnext;
    }
}
unsafe extern "C" fn dump_canvas(mut cv: *mut canvas, mut file: *mut FILE) {
    let mut c: *mut canvas = 0 as *mut canvas;
    c = (*cv).c_slperp;
    while !c.is_null() && !((*c).c_slnext).is_null() {
        fprintf(
            file,
            b"split%s\n\0" as *const u8 as *const libc::c_char,
            if (*c).c_slorient == (1 as libc::c_int) << 1 as libc::c_int {
                b" -v\0" as *const u8 as *const libc::c_char
            } else {
                b"\0" as *const u8 as *const libc::c_char
            },
        );
        c = (*c).c_slnext;
    }
    c = (*cv).c_slperp;
    while !c.is_null() {
        if !((*c).c_slperp).is_null() {
            dump_canvas(c, file);
        } else {
            fprintf(file, b"focus\n\0" as *const u8 as *const libc::c_char);
        }
        c = (*c).c_slnext;
    }
}
pub unsafe extern "C" fn LayoutDumpCanvas(
    mut cv: *mut canvas,
    mut filename: *mut libc::c_char,
) -> libc::c_int {
    let mut file: *mut FILE = secfopen(
        filename,
        b"a\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if file.is_null() {
        return 0 as libc::c_int;
    }
    dump_canvas(cv, file);
    fclose(file);
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn RenameLayout(
    mut layout: *mut layout,
    mut name: *const libc::c_char,
) {
    free((*layout).lay_title as *mut libc::c_void);
    (*layout).lay_title = SaveStr(name);
}
pub unsafe extern "C" fn RenumberLayout(
    mut layout: *mut layout,
    mut number: libc::c_int,
) -> libc::c_int {
    let mut old: libc::c_int = 0;
    let mut lay: *mut layout = 0 as *mut layout;
    old = (*layout).lay_number;
    if number < 0 as libc::c_int || number >= 10 as libc::c_int {
        return 0 as libc::c_int;
    }
    lay = laytab[number as usize];
    laytab[number as usize] = layout;
    (*layout).lay_number = number;
    laytab[old as usize] = lay;
    if !lay.is_null() {
        (*lay).lay_number = old;
    }
    return 1 as libc::c_int;
}
