use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type logfile;
    fn getpwnam(__name: *const libc::c_char) -> *mut passwd;
    fn getspnam(__name: *const libc::c_char) -> *mut spwd;
    fn closelog();
    fn openlog(
        __ident: *const libc::c_char,
        __option: libc::c_int,
        __facility: libc::c_int,
    );
    fn syslog(__pri: libc::c_int, __fmt: *const libc::c_char, _: ...);
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn crypt(
        __key: *const libc::c_char,
        __salt: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn free(__ptr: *mut libc::c_void);
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn Detach(_: libc::c_int);
    fn Finit(_: libc::c_int);
    fn FreePaster(_: *mut paster);
    fn FindCommnr(_: *const libc::c_char) -> libc::c_int;
    fn WindowByNoN(_: *mut libc::c_char) -> libc::c_int;
    fn SaveStr(_: *const libc::c_char) -> *mut libc::c_char;
    static mut comms: [comm; 0];
    static mut windows: *mut win;
    static mut wtab: *mut *mut win;
    static mut NullStr: [libc::c_char; 0];
    static mut SockPath: [libc::c_char; 0];
    static mut display: *mut display;
    static mut displays: *mut display;
}
pub type __int32_t = libc::c_int;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct passwd {
    pub pw_name: *mut libc::c_char,
    pub pw_passwd: *mut libc::c_char,
    pub pw_uid: __uid_t,
    pub pw_gid: __gid_t,
    pub pw_gecos: *mut libc::c_char,
    pub pw_dir: *mut libc::c_char,
    pub pw_shell: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spwd {
    pub sp_namp: *mut libc::c_char,
    pub sp_pwdp: *mut libc::c_char,
    pub sp_lstchg: libc::c_long,
    pub sp_min: libc::c_long,
    pub sp_max: libc::c_long,
    pub sp_warn: libc::c_long,
    pub sp_inact: libc::c_long,
    pub sp_expire: libc::c_long,
    pub sp_flag: libc::c_ulong,
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
pub static mut users: *mut acluser = 0 as *const acluser as *mut acluser;
pub static mut maxusercount: libc::c_int = 0 as libc::c_int;
static mut userbits: AclBits = 0 as *const libc::c_uchar as *mut libc::c_uchar;
static mut default_w_bit: [libc::c_char; 3] = [
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
];
static mut default_c_bit: [libc::c_char; 1] = [0 as libc::c_int as libc::c_char];
unsafe extern "C" fn GrowBitfield(
    mut bfp: *mut AclBits,
    mut len: libc::c_int,
    mut delta: libc::c_int,
    mut defaultbit: libc::c_int,
) -> libc::c_int {
    let mut n: AclBits = 0 as *mut libc::c_uchar;
    let mut o: AclBits = *bfp;
    let mut i: libc::c_int = 0;
    n = calloc(
        1 as libc::c_int as libc::c_ulong,
        &mut *(0 as *mut libc::c_char)
            .offset((len + delta + 1 as libc::c_int >> 3 as libc::c_int) as isize)
            as *mut libc::c_char as libc::c_ulong,
    ) as AclBits;
    if n.is_null() {
        return -(1 as libc::c_int);
    }
    i = 0 as libc::c_int;
    while i < len + delta {
        if i < len
            && 0x80 as libc::c_int >> (i & 7 as libc::c_int)
                & *o.offset((i >> 3 as libc::c_int) as isize) as libc::c_int != 0
            || i >= len && defaultbit != 0
        {
            let ref mut fresh0 = *n.offset((i >> 3 as libc::c_int) as isize);
            *fresh0 = (*fresh0 as libc::c_int
                | 0x80 as libc::c_int >> (i & 7 as libc::c_int)) as libc::c_uchar;
        }
        i += 1;
        i;
    }
    if len != 0 {
        free(o as *mut libc::c_char as *mut libc::c_void);
    }
    *bfp = n;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn FindUserPtr(mut name: *mut libc::c_char) -> *mut *mut acluser {
    let mut u: *mut *mut acluser = 0 as *mut *mut acluser;
    u = &mut users;
    while !(*u).is_null() {
        if strcmp(((**u).u_name).as_mut_ptr(), name) == 0 {
            break;
        }
        u = &mut (**u).u_next;
    }
    return u;
}
pub static mut DefaultEsc: libc::c_int = -(1 as libc::c_int);
pub static mut DefaultMetaEsc: libc::c_int = -(1 as libc::c_int);
pub unsafe extern "C" fn UserAdd(
    mut name: *mut libc::c_char,
    mut pass: *mut libc::c_char,
    mut up: *mut *mut acluser,
) -> libc::c_int {
    let mut j: libc::c_int = 0;
    if up.is_null() {
        up = FindUserPtr(name);
    }
    if !(*up).is_null() {
        if !pass.is_null() {
            (**up).u_password = SaveStr(pass);
        }
        return 1 as libc::c_int;
    }
    if strcmp(b"none\0" as *const u8 as *const libc::c_char, name) != 0 {
        *up = calloc(
            1 as libc::c_int as libc::c_ulong,
            ::std::mem::size_of::<acluser>() as libc::c_ulong,
        ) as *mut acluser;
    }
    if (*up).is_null() {
        return -(1 as libc::c_int);
    }
    (**up).u_plop.buf = 0 as *mut libc::c_char;
    (**up).u_plop.len = 0 as libc::c_int;
    (**up).u_plop.enc = 0 as libc::c_int;
    (**up).u_Esc = DefaultEsc;
    (**up).u_MetaEsc = DefaultMetaEsc;
    strncpy(((**up).u_name).as_mut_ptr(), name, 256 as libc::c_int as libc::c_ulong);
    (**up).u_password = 0 as *mut libc::c_char;
    if !pass.is_null() {
        (**up).u_password = SaveStr(pass);
    }
    if ((**up).u_password).is_null() {
        (**up).u_password = NullStr.as_mut_ptr();
    }
    (**up).u_detachwin = -(1 as libc::c_int);
    (**up).u_detachotherwin = -(1 as libc::c_int);
    (**up).u_group = 0 as *mut aclusergroup;
    (**up).u_id = 0 as libc::c_int;
    while (**up).u_id < maxusercount {
        if 0x80 as libc::c_int >> ((**up).u_id & 7 as libc::c_int)
            & *userbits.offset(((**up).u_id >> 3 as libc::c_int) as isize) as libc::c_int
            == 0
        {
            break;
        }
        (**up).u_id += 1;
        (**up).u_id;
    }
    if (**up).u_id == maxusercount {
        let mut j_0: libc::c_int = 0;
        let mut w: *mut win = 0 as *mut win;
        let mut u: *mut acluser = 0 as *mut acluser;
        if GrowBitfield(&mut userbits, maxusercount, 8 as libc::c_int, 0 as libc::c_int)
            != 0
        {
            free(*up as *mut libc::c_char as *mut libc::c_void);
            *up = 0 as *mut acluser;
            return -(1 as libc::c_int);
        }
        j_0 = 0 as libc::c_int;
        while j_0 <= 189 as libc::c_int {
            let mut i: libc::c_int = 0;
            i = 0 as libc::c_int;
            while i < 1 as libc::c_int {
                if GrowBitfield(
                    &mut *((*comms.as_mut_ptr().offset(j_0 as isize)).userbits)
                        .as_mut_ptr()
                        .offset(i as isize),
                    maxusercount,
                    8 as libc::c_int,
                    default_c_bit[i as usize] as libc::c_int,
                ) != 0
                {
                    free(*up as *mut libc::c_char as *mut libc::c_void);
                    *up = 0 as *mut acluser;
                    return -(1 as libc::c_int);
                }
                i += 1;
                i;
            }
            j_0 += 1;
            j_0;
        }
        u = users;
        while u != *up {
            j_0 = 0 as libc::c_int;
            while j_0 < 3 as libc::c_int {
                if GrowBitfield(
                    &mut *((*u).u_umask_w_bits).as_mut_ptr().offset(j_0 as isize),
                    maxusercount,
                    8 as libc::c_int,
                    default_w_bit[j_0 as usize] as libc::c_int,
                ) != 0
                {
                    free(*up as *mut libc::c_char as *mut libc::c_void);
                    *up = 0 as *mut acluser;
                    return -(1 as libc::c_int);
                }
                j_0 += 1;
                j_0;
            }
            u = (*u).u_next;
        }
        w = windows;
        while !w.is_null() {
            j_0 = 0 as libc::c_int;
            while j_0 < 3 as libc::c_int {
                if GrowBitfield(
                    &mut *((*w).w_userbits).as_mut_ptr().offset(j_0 as isize),
                    maxusercount,
                    8 as libc::c_int,
                    default_w_bit[j_0 as usize] as libc::c_int,
                ) != 0
                {
                    free(*up as *mut libc::c_char as *mut libc::c_void);
                    *up = 0 as *mut acluser;
                    return -(1 as libc::c_int);
                }
                j_0 += 1;
                j_0;
            }
            if GrowBitfield(
                &mut (*w).w_mon_notify,
                maxusercount,
                8 as libc::c_int,
                0 as libc::c_int,
            ) != 0
                || GrowBitfield(
                    &mut (*w).w_lio_notify,
                    maxusercount,
                    8 as libc::c_int,
                    0 as libc::c_int,
                ) != 0
            {
                free(*up as *mut libc::c_char as *mut libc::c_void);
                *up = 0 as *mut acluser;
                return -(1 as libc::c_int);
            }
            w = (*w).w_next;
        }
        maxusercount += 8 as libc::c_int;
    }
    let ref mut fresh1 = *userbits.offset(((**up).u_id >> 3 as libc::c_int) as isize);
    *fresh1 = (*fresh1 as libc::c_int
        | 0x80 as libc::c_int >> ((**up).u_id & 7 as libc::c_int)) as libc::c_uchar;
    if (**up).u_id == 0 as libc::c_int {
        AclSetPerm(
            0 as *mut acluser,
            *up,
            b"+a\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"#?\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    if strcmp(
        ((**up).u_name).as_mut_ptr(),
        b"nobody\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
        AclSetPerm(
            0 as *mut acluser,
            *up,
            b"-rwx\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"#?\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        AclSetPerm(
            0 as *mut acluser,
            *up,
            b"+x\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"su\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        AclSetPerm(
            0 as *mut acluser,
            *up,
            b"+x\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"detach\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        AclSetPerm(
            0 as *mut acluser,
            *up,
            b"+x\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"displays\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        AclSetPerm(
            0 as *mut acluser,
            *up,
            b"+x\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"version\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    j = 0 as libc::c_int;
    while j < 3 as libc::c_int {
        if GrowBitfield(
            &mut *((**up).u_umask_w_bits).as_mut_ptr().offset(j as isize),
            0 as libc::c_int,
            maxusercount,
            default_w_bit[j as usize] as libc::c_int,
        ) != 0
        {
            free(*up as *mut libc::c_char as *mut libc::c_void);
            *up = 0 as *mut acluser;
            return -(1 as libc::c_int);
        }
        let ref mut fresh2 = *((**up).u_umask_w_bits[j as usize])
            .offset(((**up).u_id >> 3 as libc::c_int) as isize);
        *fresh2 = (*fresh2 as libc::c_int
            | 0x80 as libc::c_int >> ((**up).u_id & 7 as libc::c_int)) as libc::c_uchar;
        j += 1;
        j;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn UserDel(
    mut name: *mut libc::c_char,
    mut up: *mut *mut acluser,
) -> libc::c_int {
    let mut u: *mut acluser = 0 as *mut acluser;
    let mut i: libc::c_int = 0;
    let mut old: *mut display = 0 as *mut display;
    let mut next: *mut display = 0 as *mut display;
    if up.is_null() {
        up = FindUserPtr(name);
    }
    u = *up;
    if u.is_null() {
        return -(1 as libc::c_int);
    }
    old = display;
    display = displays;
    while !display.is_null() {
        next = (*display).d_next;
        if !((*display).d_user != u) {
            if display == old {
                old = 0 as *mut display;
            }
            Detach(2 as libc::c_int);
        }
        display = next;
    }
    display = old;
    *up = (*u).u_next;
    up = &mut users;
    while !(*up).is_null() {
        let mut g: *mut *mut aclusergroup = &mut (**up).u_group;
        while !(*g).is_null() {
            if (**g).u == u {
                let mut next_0: *mut aclusergroup = (**g).next;
                free(*g as *mut libc::c_char as *mut libc::c_void);
                *g = next_0;
            } else {
                g = &mut (**g).next;
            }
        }
        up = &mut (**up).u_next;
    }
    let ref mut fresh3 = *userbits.offset(((*u).u_id >> 3 as libc::c_int) as isize);
    *fresh3 = (*fresh3 as libc::c_int
        & !(0x80 as libc::c_int >> ((*u).u_id & 7 as libc::c_int))) as libc::c_uchar;
    AclSetPerm(
        0 as *mut acluser,
        u,
        (if default_w_bit[2 as libc::c_int as usize] as libc::c_int != 0 {
            b"+r\0" as *const u8 as *const libc::c_char
        } else {
            b"-r\0" as *const u8 as *const libc::c_char
        }) as *mut libc::c_char,
        b"#\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    AclSetPerm(
        0 as *mut acluser,
        u,
        (if default_w_bit[1 as libc::c_int as usize] as libc::c_int != 0 {
            b"+w\0" as *const u8 as *const libc::c_char
        } else {
            b"-w\0" as *const u8 as *const libc::c_char
        }) as *mut libc::c_char,
        b"#\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    AclSetPerm(
        0 as *mut acluser,
        u,
        (if default_w_bit[0 as libc::c_int as usize] as libc::c_int != 0 {
            b"+x\0" as *const u8 as *const libc::c_char
        } else {
            b"-x\0" as *const u8 as *const libc::c_char
        }) as *mut libc::c_char,
        b"#\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    AclSetPerm(
        0 as *mut acluser,
        u,
        (if default_c_bit[0 as libc::c_int as usize] as libc::c_int != 0 {
            b"+x\0" as *const u8 as *const libc::c_char
        } else {
            b"-x\0" as *const u8 as *const libc::c_char
        }) as *mut libc::c_char,
        b"?\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        free((*u).u_umask_w_bits[i as usize] as *mut libc::c_char as *mut libc::c_void);
        i += 1;
        i;
    }
    UserFreeCopyBuffer(u);
    free(u as *mut libc::c_char as *mut libc::c_void);
    if users.is_null() {
        Finit(0 as libc::c_int);
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn UserFreeCopyBuffer(mut u: *mut acluser) -> libc::c_int {
    let mut w: *mut win = 0 as *mut win;
    let mut pa: *mut paster = 0 as *mut paster;
    if ((*u).u_plop.buf).is_null() {
        return 1 as libc::c_int;
    }
    w = windows;
    while !w.is_null() {
        pa = &mut (*w).w_paster;
        if (*pa).pa_pasteptr >= (*u).u_plop.buf
            && (((*pa).pa_pasteptr).offset_from((*u).u_plop.buf) as libc::c_long)
                < (*u).u_plop.len as libc::c_long
        {
            FreePaster(pa);
        }
        w = (*w).w_next;
    }
    free((*u).u_plop.buf as *mut libc::c_void);
    (*u).u_plop.len = 0 as libc::c_int;
    (*u).u_plop.buf = 0 as *mut libc::c_char;
    return 0 as libc::c_int;
}
unsafe extern "C" fn FindGroupPtr(
    mut gp: *mut *mut aclusergroup,
    mut u: *mut acluser,
    mut recursive: libc::c_int,
) -> *mut *mut aclusergroup {
    let mut g: *mut *mut aclusergroup = 0 as *mut *mut aclusergroup;
    while !(*gp).is_null() {
        if (**gp).u == u {
            return gp;
        }
        if recursive != 0
            && {
                g = FindGroupPtr(
                    &mut (*(**gp).u).u_group,
                    u,
                    recursive + 1 as libc::c_int,
                );
                !(*g).is_null()
            }
        {
            return g;
        }
        gp = &mut (**gp).next;
    }
    return gp;
}
unsafe extern "C" fn PasswordMatches(
    mut pw: *const libc::c_char,
    mut password: *const libc::c_char,
) -> libc::c_int {
    if *password == 0 {
        return 0 as libc::c_int;
    }
    let mut buf: *mut libc::c_char = crypt(
        pw as *mut libc::c_char,
        password as *mut libc::c_char,
    );
    return (!buf.is_null() && strcmp(buf, password) == 0) as libc::c_int;
}
pub unsafe extern "C" fn AclLinkUser(
    mut from: *mut libc::c_char,
    mut to: *mut libc::c_char,
) -> libc::c_int {
    let mut u1: *mut *mut acluser = 0 as *mut *mut acluser;
    let mut u2: *mut *mut acluser = 0 as *mut *mut acluser;
    let mut g: *mut *mut aclusergroup = 0 as *mut *mut aclusergroup;
    u1 = FindUserPtr(from);
    if (*u1).is_null() && UserAdd(from, 0 as *mut libc::c_char, u1) != 0 {
        return -(1 as libc::c_int);
    }
    u2 = FindUserPtr(to);
    if (*u2).is_null() && UserAdd(to, 0 as *mut libc::c_char, u2) != 0 {
        return -(1 as libc::c_int);
    }
    if !(*FindGroupPtr(&mut (**u2).u_group, *u1, 1 as libc::c_int)).is_null() {
        return 1 as libc::c_int;
    }
    g = FindGroupPtr(&mut (**u1).u_group, *u2, 0 as libc::c_int);
    if !(*g).is_null() {
        return 2 as libc::c_int;
    }
    *g = malloc(::std::mem::size_of::<aclusergroup>() as libc::c_ulong)
        as *mut aclusergroup;
    if (*g).is_null() {
        return -(1 as libc::c_int);
    }
    (**g).u = *u2;
    (**g).next = 0 as *mut aclusergroup;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn DoSu(
    mut up: *mut *mut acluser,
    mut name: *mut libc::c_char,
    mut pw1: *mut libc::c_char,
    mut pw2: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut u: *mut acluser = 0 as *mut acluser;
    let mut sorry: libc::c_int = 0 as libc::c_int;
    u = *FindUserPtr(name);
    if u.is_null() {
        sorry += 1;
        sorry;
    } else {
        let mut pp: *mut passwd = 0 as *mut passwd;
        let mut ss: *mut spwd = 0 as *mut spwd;
        let mut t: libc::c_int = 0;
        let mut c: libc::c_int = 0;
        let mut pass: *mut libc::c_char = b"\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char;
        pp = getpwnam(name);
        if pp.is_null() {
            if !(!pw1.is_null() && *pw1 as libc::c_int != 0
                && *pw1 as libc::c_int != -1i32)
            {
                sorry += 1;
                sorry;
            }
        } else {
            pass = (*pp).pw_passwd;
        }
        t = 0 as libc::c_int;
        while t < 13 as libc::c_int {
            c = *pass.offset(t as isize) as libc::c_int;
            if !(c == '.' as i32 || c == '/' as i32 || c >= '0' as i32 && c <= '9' as i32
                || c >= 'a' as i32 && c <= 'z' as i32
                || c >= 'A' as i32 && c <= 'Z' as i32)
            {
                break;
            }
            t += 1;
            t;
        }
        if t < 13 as libc::c_int {
            ss = getspnam(name);
            if ss.is_null() {
                sorry += 1;
                sorry;
            } else {
                pass = (*ss).sp_pwdp;
            }
        }
        if !pw2.is_null() && *pw2 as libc::c_int != 0 && *pw2 as libc::c_int != -1i32 {
            if PasswordMatches(pw2, pass) == 0 {
                sorry += 1;
                sorry;
            }
        } else if *pass != 0 {
            sorry += 1;
            sorry;
        }
        if !pw1.is_null() && *pw1 as libc::c_int != 0 && *pw1 as libc::c_int != -1i32 {
            if PasswordMatches(pw1, (*u).u_password) == 0 {
                sorry += 1;
                sorry;
            }
        } else if *(*u).u_password != 0 {
            sorry += 1;
            sorry;
        }
    }
    openlog(
        b"screen\0" as *const u8 as *const libc::c_char,
        0x1 as libc::c_int,
        (4 as libc::c_int) << 3 as libc::c_int,
    );
    syslog(
        5 as libc::c_int,
        b"%s: \"su %s\" %s for \"%s\"\0" as *const u8 as *const libc::c_char,
        SockPath.as_mut_ptr(),
        name,
        if sorry != 0 {
            b"failed\0" as *const u8 as *const libc::c_char
        } else {
            b"succeeded\0" as *const u8 as *const libc::c_char
        },
        ((**up).u_name).as_mut_ptr(),
    );
    closelog();
    if sorry != 0 {
        return b"Sorry.\0" as *const u8 as *const libc::c_char as *mut libc::c_char
    } else {
        *up = u;
    }
    return 0 as *mut libc::c_char;
}
pub unsafe extern "C" fn NewWindowAcl(
    mut w: *mut win,
    mut u: *mut acluser,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    if GrowBitfield(
        &mut (*w).w_mon_notify,
        0 as libc::c_int,
        maxusercount,
        0 as libc::c_int,
    ) != 0
        || GrowBitfield(
            &mut (*w).w_lio_notify,
            0 as libc::c_int,
            maxusercount,
            0 as libc::c_int,
        ) != 0
    {
        return -(1 as libc::c_int);
    }
    j = 0 as libc::c_int;
    while j < 3 as libc::c_int {
        if GrowBitfield(
            &mut *((*w).w_userbits).as_mut_ptr().offset(j as isize),
            0 as libc::c_int,
            maxusercount,
            0 as libc::c_int,
        ) != 0
        {
            loop {
                j -= 1;
                if !(j >= 0 as libc::c_int) {
                    break;
                }
                free(
                    (*w).w_userbits[j as usize] as *mut libc::c_char as *mut libc::c_void,
                );
            }
            free((*w).w_mon_notify as *mut libc::c_char as *mut libc::c_void);
            free((*w).w_lio_notify as *mut libc::c_char as *mut libc::c_void);
            return -(1 as libc::c_int);
        }
        i = 0 as libc::c_int;
        while i < maxusercount {
            if if !u.is_null() {
                0x80 as libc::c_int >> (i & 7 as libc::c_int)
                    & *((*u).u_umask_w_bits[j as usize])
                        .offset((i >> 3 as libc::c_int) as isize) as libc::c_int
            } else {
                default_w_bit[j as usize] as libc::c_int
            } != 0
            {
                let ref mut fresh4 = *((*w).w_userbits[j as usize])
                    .offset((i >> 3 as libc::c_int) as isize);
                *fresh4 = (*fresh4 as libc::c_int
                    | 0x80 as libc::c_int >> (i & 7 as libc::c_int)) as libc::c_uchar;
            }
            i += 1;
            i;
        }
        j += 1;
        j;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn FreeWindowAcl(mut w: *mut win) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        free((*w).w_userbits[i as usize] as *mut libc::c_char as *mut libc::c_void);
        i += 1;
        i;
    }
    free((*w).w_mon_notify as *mut libc::c_char as *mut libc::c_void);
    free((*w).w_lio_notify as *mut libc::c_char as *mut libc::c_void);
}
unsafe extern "C" fn AclSetPermCmd(
    mut u: *mut acluser,
    mut mode: *mut libc::c_char,
    mut cmd: *mut comm,
) -> libc::c_int {
    let mut neg: libc::c_int = 0 as libc::c_int;
    let mut m: *mut libc::c_char = mode;
    while *m != 0 {
        let fresh5 = m;
        m = m.offset(1);
        match *fresh5 as libc::c_int {
            45 => {
                neg = 1 as libc::c_int;
            }
            43 => {
                neg = 0 as libc::c_int;
            }
            97 | 101 | 120 => {
                if neg != 0 {
                    let ref mut fresh6 = *(*((*cmd).userbits)
                        .as_mut_ptr()
                        .offset(0 as libc::c_int as isize))
                        .offset(((*u).u_id >> 3 as libc::c_int) as isize);
                    *fresh6 = (*fresh6 as libc::c_int
                        & !(0x80 as libc::c_int >> ((*u).u_id & 7 as libc::c_int)))
                        as libc::c_uchar;
                } else {
                    let ref mut fresh7 = *(*((*cmd).userbits)
                        .as_mut_ptr()
                        .offset(0 as libc::c_int as isize))
                        .offset(((*u).u_id >> 3 as libc::c_int) as isize);
                    *fresh7 = (*fresh7 as libc::c_int
                        | 0x80 as libc::c_int >> ((*u).u_id & 7 as libc::c_int))
                        as libc::c_uchar;
                }
            }
            114 | 119 => {}
            _ => return -(1 as libc::c_int),
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn AclSetPermWin(
    mut uu: *mut acluser,
    mut u: *mut acluser,
    mut mode: *mut libc::c_char,
    mut win: *mut win,
) -> libc::c_int {
    let mut neg: libc::c_int = 0 as libc::c_int;
    let mut bit: libc::c_int = 0;
    let mut bits: libc::c_int = 0;
    let mut bitarray: *mut AclBits = 0 as *mut AclBits;
    let mut m: *mut libc::c_char = mode;
    if !uu.is_null() {
        bitarray = ((*uu).u_umask_w_bits).as_mut_ptr();
    } else {
        bitarray = ((*win).w_userbits).as_mut_ptr();
    }
    while *m != 0 {
        let fresh8 = m;
        m = m.offset(1);
        match *fresh8 as libc::c_int {
            45 => {
                neg = 1 as libc::c_int;
                continue;
            }
            43 => {
                neg = 0 as libc::c_int;
                continue;
            }
            114 => {
                bits = (1 as libc::c_int) << 2 as libc::c_int;
            }
            119 => {
                bits = (1 as libc::c_int) << 1 as libc::c_int;
            }
            120 => {
                bits = (1 as libc::c_int) << 0 as libc::c_int;
            }
            97 => {
                bits = ((1 as libc::c_int) << 3 as libc::c_int) - 1 as libc::c_int;
            }
            _ => return -(1 as libc::c_int),
        }
        bit = 0 as libc::c_int;
        while bit < 3 as libc::c_int {
            if !(bits & (1 as libc::c_int) << bit == 0) {
                if neg != 0 {
                    let ref mut fresh9 = *(*bitarray.offset(bit as isize))
                        .offset(((*u).u_id >> 3 as libc::c_int) as isize);
                    *fresh9 = (*fresh9 as libc::c_int
                        & !(0x80 as libc::c_int >> ((*u).u_id & 7 as libc::c_int)))
                        as libc::c_uchar;
                } else {
                    let ref mut fresh10 = *(*bitarray.offset(bit as isize))
                        .offset(((*u).u_id >> 3 as libc::c_int) as isize);
                    *fresh10 = (*fresh10 as libc::c_int
                        | 0x80 as libc::c_int >> ((*u).u_id & 7 as libc::c_int))
                        as libc::c_uchar;
                }
                if uu.is_null() && (*win).w_wlockuser == u && neg != 0
                    && bit == 1 as libc::c_int
                {
                    (*win).w_wlockuser = 0 as *mut acluser;
                    if (*win).w_wlock == 2 as libc::c_int {
                        (*win).w_wlock = 1 as libc::c_int;
                    }
                }
            }
            bit += 1;
            bit;
        }
    }
    if !uu.is_null()
        && (*u).u_name[0 as libc::c_int as usize] as libc::c_int == '?' as i32
        && (*u).u_name[1 as libc::c_int as usize] as libc::c_int == '\0' as i32
    {
        if !win.is_null() {
            bit = 0 as libc::c_int;
            while bit < 3 as libc::c_int {
                default_w_bit[bit
                    as usize] = (if *(*bitarray.offset(bit as isize))
                    .offset(((*u).u_id >> 3 as libc::c_int) as isize) as libc::c_int
                    & 0x80 as libc::c_int >> ((*u).u_id & 7 as libc::c_int) != 0
                {
                    1 as libc::c_int
                } else {
                    0 as libc::c_int
                }) as libc::c_char;
                bit += 1;
                bit;
            }
        } else {
            bit = 0 as libc::c_int;
            while bit < 1 as libc::c_int {
                default_c_bit[bit
                    as usize] = (if *(*bitarray.offset(bit as isize))
                    .offset(((*u).u_id >> 3 as libc::c_int) as isize) as libc::c_int
                    & 0x80 as libc::c_int >> ((*u).u_id & 7 as libc::c_int) != 0
                {
                    1 as libc::c_int
                } else {
                    0 as libc::c_int
                }) as libc::c_char;
                bit += 1;
                bit;
            }
        }
        UserDel(((*u).u_name).as_mut_ptr(), 0 as *mut *mut acluser);
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn AclSetPerm(
    mut uu: *mut acluser,
    mut u: *mut acluser,
    mut mode: *mut libc::c_char,
    mut s: *mut libc::c_char,
) -> libc::c_int {
    let mut w: *mut win = 0 as *mut win;
    let mut i: libc::c_int = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ch: libc::c_char = 0;
    while *s != 0 {
        match *s as libc::c_int {
            42 => {
                return AclSetPerm(
                    uu,
                    u,
                    mode,
                    b"#?\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
            }
            35 => {
                if !uu.is_null() {
                    AclSetPermWin(uu, u, mode, 1 as libc::c_int as *mut win);
                } else {
                    w = windows;
                    while !w.is_null() {
                        AclSetPermWin(0 as *mut acluser, u, mode, w);
                        w = (*w).w_next;
                    }
                }
                s = s.offset(1);
                s;
            }
            63 => {
                if !uu.is_null() {
                    AclSetPermWin(uu, u, mode, 0 as *mut win);
                } else {
                    i = 0 as libc::c_int;
                    while i <= 189 as libc::c_int {
                        AclSetPermCmd(
                            u,
                            mode,
                            &mut *comms.as_mut_ptr().offset(i as isize),
                        );
                        i += 1;
                        i;
                    }
                }
                s = s.offset(1);
                s;
            }
            _ => {
                p = s;
                while *p as libc::c_int != 0 && *p as libc::c_int != ' ' as i32
                    && *p as libc::c_int != '\t' as i32
                    && *p as libc::c_int != ',' as i32
                {
                    p = p.offset(1);
                    p;
                }
                ch = *p;
                if ch != 0 {
                    let fresh11 = p;
                    p = p.offset(1);
                    *fresh11 = '\0' as i32 as libc::c_char;
                }
                i = FindCommnr(s);
                if i != -(1 as libc::c_int) {
                    AclSetPermCmd(u, mode, &mut *comms.as_mut_ptr().offset(i as isize));
                } else {
                    i = WindowByNoN(s);
                    if i >= 0 as libc::c_int && !(*wtab.offset(i as isize)).is_null() {
                        AclSetPermWin(
                            0 as *mut acluser,
                            u,
                            mode,
                            *wtab.offset(i as isize),
                        );
                    } else {
                        return -(1 as libc::c_int)
                    }
                }
                if ch != 0 {
                    *p.offset(-(1 as libc::c_int) as isize) = ch;
                }
                s = p;
            }
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn UserAcl(
    mut uu: *mut acluser,
    mut u: *mut *mut acluser,
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    if !(*u).is_null()
        && strcmp(
            ((**u).u_name).as_mut_ptr(),
            b"nobody\0" as *const u8 as *const libc::c_char,
        ) == 0
        || argc > 1 as libc::c_int
            && strcmp(
                *argv.offset(0 as libc::c_int as isize),
                b"nobody\0" as *const u8 as *const libc::c_char,
            ) == 0
    {
        return -(1 as libc::c_int);
    }
    match argc {
        4 => {
            return (UserAdd(
                *argv.offset(0 as libc::c_int as isize),
                *argv.offset(1 as libc::c_int as isize),
                u,
            ) < 0 as libc::c_int
                || AclSetPerm(
                    uu,
                    *u,
                    *argv.offset(2 as libc::c_int as isize),
                    *argv.offset(3 as libc::c_int as isize),
                ) != 0) as libc::c_int;
        }
        3 => {
            return (UserAdd(
                *argv.offset(0 as libc::c_int as isize),
                0 as *mut libc::c_char,
                u,
            ) < 0 as libc::c_int
                || AclSetPerm(
                    uu,
                    *u,
                    *argv.offset(1 as libc::c_int as isize),
                    *argv.offset(2 as libc::c_int as isize),
                ) != 0) as libc::c_int;
        }
        2 => {
            return (UserAdd(
                *argv.offset(0 as libc::c_int as isize),
                *argv.offset(1 as libc::c_int as isize),
                u,
            ) < 0 as libc::c_int) as libc::c_int;
        }
        1 => {
            return (UserAdd(
                *argv.offset(0 as libc::c_int as isize),
                0 as *mut libc::c_char,
                u,
            ) < 0 as libc::c_int
                || AclSetPerm(
                    uu,
                    *u,
                    b"+a\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    b"#?\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ) != 0) as libc::c_int;
        }
        _ => return -(1 as libc::c_int),
    };
}
unsafe extern "C" fn UserAclCopy(
    mut to_up: *mut *mut acluser,
    mut from_up: *mut *mut acluser,
) -> libc::c_int {
    let mut w: *mut win = 0 as *mut win;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut to_id: libc::c_int = 0;
    let mut from_id: libc::c_int = 0;
    if (*to_up).is_null() || (*from_up).is_null() {
        return -(1 as libc::c_int);
    }
    to_id = (**to_up).u_id;
    from_id = (**from_up).u_id;
    if to_id == from_id {
        return -(1 as libc::c_int);
    }
    w = windows;
    while !w.is_null() {
        i = 0 as libc::c_int;
        while i < 3 as libc::c_int {
            if *((*w).w_userbits[i as usize])
                .offset((from_id >> 3 as libc::c_int) as isize) as libc::c_int
                & 0x80 as libc::c_int >> (from_id & 7 as libc::c_int) != 0
            {
                let ref mut fresh12 = *((*w).w_userbits[i as usize])
                    .offset((to_id >> 3 as libc::c_int) as isize);
                *fresh12 = (*fresh12 as libc::c_int
                    | 0x80 as libc::c_int >> (to_id & 7 as libc::c_int))
                    as libc::c_uchar;
            } else {
                let ref mut fresh13 = *((*w).w_userbits[i as usize])
                    .offset((to_id >> 3 as libc::c_int) as isize);
                *fresh13 = (*fresh13 as libc::c_int
                    & !(0x80 as libc::c_int >> (to_id & 7 as libc::c_int)))
                    as libc::c_uchar;
                if (*w).w_wlockuser == *to_up && i == 1 as libc::c_int {
                    (*w).w_wlockuser = 0 as *mut acluser;
                    if (*w).w_wlock == 2 as libc::c_int {
                        (*w).w_wlock = 1 as libc::c_int;
                    }
                }
            }
            i += 1;
            i;
        }
        w = (*w).w_next;
    }
    j = 0 as libc::c_int;
    while j <= 189 as libc::c_int {
        i = 0 as libc::c_int;
        while i < 1 as libc::c_int {
            if *(*((*comms.as_mut_ptr().offset(j as isize)).userbits)
                .as_mut_ptr()
                .offset(i as isize))
                .offset((from_id >> 3 as libc::c_int) as isize) as libc::c_int
                & 0x80 as libc::c_int >> (from_id & 7 as libc::c_int) != 0
            {
                let ref mut fresh14 = *(*((*comms.as_mut_ptr().offset(j as isize))
                    .userbits)
                    .as_mut_ptr()
                    .offset(i as isize))
                    .offset((to_id >> 3 as libc::c_int) as isize);
                *fresh14 = (*fresh14 as libc::c_int
                    | 0x80 as libc::c_int >> (to_id & 7 as libc::c_int))
                    as libc::c_uchar;
            } else {
                let ref mut fresh15 = *(*((*comms.as_mut_ptr().offset(j as isize))
                    .userbits)
                    .as_mut_ptr()
                    .offset(i as isize))
                    .offset((to_id >> 3 as libc::c_int) as isize);
                *fresh15 = (*fresh15 as libc::c_int
                    & !(0x80 as libc::c_int >> (to_id & 7 as libc::c_int)))
                    as libc::c_uchar;
            }
            i += 1;
            i;
        }
        j += 1;
        j;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn UsersAcl(
    mut uu: *mut acluser,
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut r: libc::c_int = 0;
    let mut cf_u: *mut *mut acluser = 0 as *mut *mut acluser;
    if argc == 1 as libc::c_int {
        let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
        s = *argv.offset(0 as libc::c_int as isize);
        while *s != 0 {
            let fresh16 = s;
            s = s.offset(1);
            if *fresh16 as libc::c_int == '=' as i32 {
                p = s;
            }
        }
        if !p.is_null() {
            *p.offset(-(1 as libc::c_int) as isize) = '\0' as i32 as libc::c_char;
            cf_u = FindUserPtr(p);
        }
    }
    if *(*argv.offset(0 as libc::c_int as isize)).offset(0 as libc::c_int as isize)
        as libc::c_int == '*' as i32
        && *(*argv.offset(0 as libc::c_int as isize)).offset(1 as libc::c_int as isize)
            as libc::c_int == '\0' as i32
    {
        let mut u: *mut *mut acluser = 0 as *mut *mut acluser;
        u = &mut users;
        while !(*u).is_null() {
            if strcmp(
                b"nobody\0" as *const u8 as *const libc::c_char,
                ((**u).u_name).as_mut_ptr(),
            ) != 0
                && (if !cf_u.is_null() {
                    r = UserAclCopy(u, cf_u);
                    (r < 0 as libc::c_int) as libc::c_int
                } else {
                    r = UserAcl(uu, u, argc, argv);
                    (r < 0 as libc::c_int) as libc::c_int
                }) != 0
            {
                return -(1 as libc::c_int);
            }
            u = &mut (**u).u_next;
        }
        return 0 as libc::c_int;
    }
    loop {
        s = *argv.offset(0 as libc::c_int as isize);
        while *s as libc::c_int != 0 && *s as libc::c_int != ' ' as i32
            && *s as libc::c_int != '\t' as i32 && *s as libc::c_int != ',' as i32
            && *s as libc::c_int != '=' as i32
        {
            s = s.offset(1);
            s;
        }
        if *s as libc::c_int != 0 {
            let fresh17 = s;
            s = s.offset(1);
            *fresh17 = '\0' as i32 as libc::c_char;
        } else {
            *s = '\0' as i32 as libc::c_char;
        };
        if if !cf_u.is_null() {
            r = UserAclCopy(FindUserPtr(*argv.offset(0 as libc::c_int as isize)), cf_u);
            (r < 0 as libc::c_int) as libc::c_int
        } else {
            r = UserAcl(
                uu,
                FindUserPtr(*argv.offset(0 as libc::c_int as isize)),
                argc,
                argv,
            );
            (r < 0 as libc::c_int) as libc::c_int
        } != 0
        {
            return -(1 as libc::c_int);
        }
        let ref mut fresh18 = *argv.offset(0 as libc::c_int as isize);
        *fresh18 = s;
        if !(**fresh18 != 0) {
            break;
        }
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn AclUmask(
    mut u: *mut acluser,
    mut str: *mut libc::c_char,
    mut errp: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut mode: [libc::c_char; 16] = [0; 16];
    let mut av: [*mut libc::c_char; 3] = [0 as *mut libc::c_char; 3];
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut c: libc::c_char = '\0' as i32 as libc::c_char;
    p = str;
    while *p != 0 {
        c = *p;
        if c as libc::c_int == '+' as i32 || c as libc::c_int == '-' as i32 {
            break;
        }
        p = p.offset(1);
        p;
    }
    if *p == 0 {
        *errp = b"Bad argument. Should be ``[user[,user...]{+|-}rwxn''.\0" as *const u8
            as *const libc::c_char as *mut libc::c_char;
        return -(1 as libc::c_int);
    }
    strncpy(mode.as_mut_ptr(), p, 15 as libc::c_int as libc::c_ulong);
    mode[15 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
    *p = '\0' as i32 as libc::c_char;
    if strcmp(b"??\0" as *const u8 as *const libc::c_char, str) == 0 {
        str = str.offset(1);
        str;
        av[2 as libc::c_int
            as usize] = b"?\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    } else {
        av[2 as libc::c_int
            as usize] = b"#\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    av[1 as libc::c_int as usize] = mode.as_mut_ptr();
    av[0 as libc::c_int
        as usize] = (if *str as libc::c_int != 0 {
        str as *const libc::c_char
    } else {
        b"*\0" as *const u8 as *const libc::c_char
    }) as *mut libc::c_char;
    if UsersAcl(u, 3 as libc::c_int, av.as_mut_ptr()) != 0 {
        *errp = b"UsersAcl failed. Hmmm.\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char;
        *p = c;
        return -(1 as libc::c_int);
    }
    *p = c;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn AclWinSwap(mut a: libc::c_int, mut b: libc::c_int) {}
pub static mut EffectiveAclUser: *mut acluser = 0 as *const acluser as *mut acluser;
pub unsafe extern "C" fn AclCheckPermWin(
    mut u: *mut acluser,
    mut mode: libc::c_int,
    mut w: *mut win,
) -> libc::c_int {
    let mut ok: libc::c_int = 0;
    if mode < 0 as libc::c_int || mode >= 3 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if !EffectiveAclUser.is_null() {
        u = EffectiveAclUser;
    }
    ok = *((*w).w_userbits[mode as usize])
        .offset(((*u).u_id >> 3 as libc::c_int) as isize) as libc::c_int
        & 0x80 as libc::c_int >> ((*u).u_id & 7 as libc::c_int);
    if ok == 0 {
        let mut g: *mut *mut aclusergroup = &mut (*u).u_group;
        let mut saved_eff: *mut acluser = EffectiveAclUser;
        EffectiveAclUser = 0 as *mut acluser;
        while !(*g).is_null() {
            if AclCheckPermWin((**g).u, mode, w) == 0 {
                break;
            }
            g = &mut (**g).next;
        }
        EffectiveAclUser = saved_eff;
        if !(*g).is_null() {
            ok = 1 as libc::c_int;
        }
    }
    return (ok == 0) as libc::c_int;
}
pub unsafe extern "C" fn AclCheckPermCmd(
    mut u: *mut acluser,
    mut mode: libc::c_int,
    mut c: *mut comm,
) -> libc::c_int {
    let mut ok: libc::c_int = 0;
    if mode < 0 as libc::c_int || mode >= 1 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if !EffectiveAclUser.is_null() {
        u = EffectiveAclUser;
    }
    ok = *(*((*c).userbits).as_mut_ptr().offset(mode as isize))
        .offset(((*u).u_id >> 3 as libc::c_int) as isize) as libc::c_int
        & 0x80 as libc::c_int >> ((*u).u_id & 7 as libc::c_int);
    if ok == 0 {
        let mut g: *mut *mut aclusergroup = &mut (*u).u_group;
        let mut saved_eff: *mut acluser = EffectiveAclUser;
        EffectiveAclUser = 0 as *mut acluser;
        while !(*g).is_null() {
            if AclCheckPermCmd((**g).u, mode, c) == 0 {
                break;
            }
            g = &mut (**g).next;
        }
        EffectiveAclUser = saved_eff;
        if !(*g).is_null() {
            ok = 1 as libc::c_int;
        }
    }
    return (ok == 0) as libc::c_int;
}
