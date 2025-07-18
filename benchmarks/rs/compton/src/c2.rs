use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type _XGC;
    pub type _XDisplay;
    pub type _glx_texture;
    fn XGetAtomName(_: *mut Display, _: Atom) -> *mut libc::c_char;
    fn XInternAtom(_: *mut Display, _: *const libc::c_char, _: libc::c_int) -> Atom;
    fn XFreeStringList(_: *mut *mut libc::c_char);
    fn XFree(_: *mut libc::c_void) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcasestr(
        __haystack: *const libc::c_char,
        __needle: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strcasecmp(__s1: *const libc::c_char, __s2: *const libc::c_char) -> libc::c_int;
    fn strncasecmp(
        __s1: *const libc::c_char,
        __s2: *const libc::c_char,
        __n: size_t,
    ) -> libc::c_int;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __ctype_tolower_loc() -> *mut *const __int32_t;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    static mut stderr: *mut FILE;
    fn exit(_: libc::c_int) -> !;
    fn free(__ptr: *mut libc::c_void);
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    static WINTYPES: [*const libc::c_char; 15];
    fn wid_get_prop_adv(
        ps: *const session_t,
        w: Window,
        atom: Atom,
        offset: libc::c_long,
        length: libc::c_long,
        rtype: Atom,
        rformat: libc::c_int,
    ) -> winprop_t;
    fn wid_get_text_prop(
        ps: *mut session_t,
        wid: Window,
        prop: Atom,
        pstrlst: *mut *mut *mut libc::c_char,
        pnstr: *mut libc::c_int,
    ) -> bool;
    fn fnmatch(
        __pattern: *const libc::c_char,
        __name: *const libc::c_char,
        __flags: libc::c_int,
    ) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type __fd_mask = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fd_set {
    pub fds_bits: [__fd_mask; 16],
}
pub type uint32_t = __uint32_t;
pub type int_fast16_t = libc::c_long;
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
pub type XID = libc::c_ulong;
pub type Atom = libc::c_ulong;
pub type VisualID = libc::c_ulong;
pub type Window = XID;
pub type Drawable = XID;
pub type Pixmap = XID;
pub type Colormap = XID;
pub type XPointer = *mut libc::c_char;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _XExtData {
    pub number: libc::c_int,
    pub next: *mut _XExtData,
    pub free_private: Option::<unsafe extern "C" fn(*mut _XExtData) -> libc::c_int>,
    pub private_data: XPointer,
}
pub type XExtData = _XExtData;
pub type GC = *mut _XGC;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Visual {
    pub ext_data: *mut XExtData,
    pub visualid: VisualID,
    pub class: libc::c_int,
    pub red_mask: libc::c_ulong,
    pub green_mask: libc::c_ulong,
    pub blue_mask: libc::c_ulong,
    pub bits_per_rgb: libc::c_int,
    pub map_entries: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Depth {
    pub depth: libc::c_int,
    pub nvisuals: libc::c_int,
    pub visuals: *mut Visual,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Screen {
    pub ext_data: *mut XExtData,
    pub display: *mut _XDisplay,
    pub root: Window,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub mwidth: libc::c_int,
    pub mheight: libc::c_int,
    pub ndepths: libc::c_int,
    pub depths: *mut Depth,
    pub root_depth: libc::c_int,
    pub root_visual: *mut Visual,
    pub default_gc: GC,
    pub cmap: Colormap,
    pub white_pixel: libc::c_ulong,
    pub black_pixel: libc::c_ulong,
    pub max_maps: libc::c_int,
    pub min_maps: libc::c_int,
    pub backing_store: libc::c_int,
    pub save_unders: libc::c_int,
    pub root_input_mask: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XWindowAttributes {
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub border_width: libc::c_int,
    pub depth: libc::c_int,
    pub visual: *mut Visual,
    pub root: Window,
    pub class: libc::c_int,
    pub bit_gravity: libc::c_int,
    pub win_gravity: libc::c_int,
    pub backing_store: libc::c_int,
    pub backing_planes: libc::c_ulong,
    pub backing_pixel: libc::c_ulong,
    pub save_under: libc::c_int,
    pub colormap: Colormap,
    pub map_installed: libc::c_int,
    pub map_state: libc::c_int,
    pub all_event_masks: libc::c_long,
    pub your_event_mask: libc::c_long,
    pub do_not_propagate_mask: libc::c_long,
    pub override_redirect: libc::c_int,
    pub screen: *mut Screen,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XRectangle {
    pub x: libc::c_short,
    pub y: libc::c_short,
    pub width: libc::c_ushort,
    pub height: libc::c_ushort,
}
pub type Display = _XDisplay;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XConfigureEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub border_width: libc::c_int,
    pub above: Window,
    pub override_redirect: libc::c_int,
}
pub type XserverRegion = XID;
pub type Damage = XID;
pub type Picture = XID;
pub type PictFormat = XID;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XRenderDirectFormat {
    pub red: libc::c_short,
    pub redMask: libc::c_short,
    pub green: libc::c_short,
    pub greenMask: libc::c_short,
    pub blue: libc::c_short,
    pub blueMask: libc::c_short,
    pub alpha: libc::c_short,
    pub alphaMask: libc::c_short,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XRenderPictFormat {
    pub id: PictFormat,
    pub type_0: libc::c_int,
    pub depth: libc::c_int,
    pub direct: XRenderDirectFormat,
    pub colormap: Colormap,
}
pub type XFixed = libc::c_int;
pub type XdbeBackBuffer = Drawable;
pub type XSyncFence = XID;
pub type opacity_t = uint32_t;
pub type time_ms_t = libc::c_long;
pub type wintype_t = libc::c_uint;
pub const NUM_WINTYPES: wintype_t = 15;
pub const WINTYPE_DND: wintype_t = 14;
pub const WINTYPE_COMBO: wintype_t = 13;
pub const WINTYPE_NOTIFY: wintype_t = 12;
pub const WINTYPE_TOOLTIP: wintype_t = 11;
pub const WINTYPE_POPUP_MENU: wintype_t = 10;
pub const WINTYPE_DROPDOWN_MENU: wintype_t = 9;
pub const WINTYPE_NORMAL: wintype_t = 8;
pub const WINTYPE_DIALOG: wintype_t = 7;
pub const WINTYPE_SPLASH: wintype_t = 6;
pub const WINTYPE_UTILITY: wintype_t = 5;
pub const WINTYPE_MENU: wintype_t = 4;
pub const WINTYPE_TOOLBAR: wintype_t = 3;
pub const WINTYPE_DOCK: wintype_t = 2;
pub const WINTYPE_DESKTOP: wintype_t = 1;
pub const WINTYPE_UNKNOWN: wintype_t = 0;
pub type switch_t = libc::c_uint;
pub const UNSET: switch_t = 2;
pub const ON: switch_t = 1;
pub const OFF: switch_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct geometry_t {
    pub wid: libc::c_int,
    pub hei: libc::c_int,
    pub x: libc::c_int,
    pub y: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct margin_t {
    pub top: libc::c_int,
    pub left: libc::c_int,
    pub bottom: libc::c_int,
    pub right: libc::c_int,
}
pub type winmode_t = libc::c_uint;
pub const WMODE_ARGB: winmode_t = 2;
pub const WMODE_SOLID: winmode_t = 1;
pub const WMODE_TRANS: winmode_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct winprop_t {
    pub data: C2RustUnnamed_0,
    pub nitems: libc::c_ulong,
    pub type_0: Atom,
    pub format: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub p8: *mut libc::c_uchar,
    pub p16: *mut libc::c_short,
    pub p32: *mut libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _ignore {
    pub next: *mut _ignore,
    pub sequence: libc::c_ulong,
}
pub type ignore_t = _ignore;
pub type vsync_t = libc::c_uint;
pub const NUM_VSYNC: vsync_t = 6;
pub const VSYNC_OPENGL_MSWC: vsync_t = 5;
pub const VSYNC_OPENGL_SWC: vsync_t = 4;
pub const VSYNC_OPENGL_OML: vsync_t = 3;
pub const VSYNC_OPENGL: vsync_t = 2;
pub const VSYNC_DRM: vsync_t = 1;
pub const VSYNC_NONE: vsync_t = 0;
pub type backend = libc::c_uint;
pub const NUM_BKEND: backend = 3;
pub const BKEND_XR_GLX_HYBRID: backend = 2;
pub const BKEND_GLX: backend = 1;
pub const BKEND_XRENDER: backend = 0;
pub type glx_texture_t = _glx_texture;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct paint_t {
    pub pixmap: Pixmap,
    pub pict: Picture,
    pub ptex: *mut glx_texture_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct conv {
    pub size: libc::c_int,
    pub data: *mut libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _latom {
    pub atom: Atom,
    pub next: *mut _latom,
}
pub type latom_t = _latom;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _timeout_t {
    pub enabled: bool,
    pub data: *mut libc::c_void,
    pub callback: Option::<
        unsafe extern "C" fn(*mut session_t, *mut _timeout_t) -> bool,
    >,
    pub interval: time_ms_t,
    pub firstrun: time_ms_t,
    pub lastrun: time_ms_t,
    pub next: *mut _timeout_t,
}
pub type session_t = _session_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _session_t {
    pub dpy: *mut Display,
    pub scr: libc::c_int,
    pub vis: *mut Visual,
    pub depth: libc::c_int,
    pub root: Window,
    pub root_height: libc::c_int,
    pub root_width: libc::c_int,
    pub overlay: Window,
    pub root_tile_fill: bool,
    pub root_tile_paint: paint_t,
    pub screen_reg: XserverRegion,
    pub root_picture: Picture,
    pub tgt_picture: Picture,
    pub tgt_buffer: paint_t,
    pub tgt_buffer_fence: XSyncFence,
    pub root_dbe: XdbeBackBuffer,
    pub reg_win: Window,
    pub o: options_t,
    pub pfds_read: *mut fd_set,
    pub pfds_write: *mut fd_set,
    pub pfds_except: *mut fd_set,
    pub nfds_max: libc::c_int,
    pub tmout_lst: *mut _timeout_t,
    pub tmout_unredir: *mut _timeout_t,
    pub tmout_unredir_hit: bool,
    pub ev_received: bool,
    pub idling: bool,
    pub time_start: timeval,
    pub all_damage: XserverRegion,
    pub all_damage_last: [XserverRegion; 5],
    pub redirected: bool,
    pub alpha_picts: *mut Picture,
    pub reg_ignore_expire: bool,
    pub fade_time: time_ms_t,
    pub ignore_head: *mut ignore_t,
    pub ignore_tail: *mut *mut ignore_t,
    pub blur_kerns_cache: [*mut XFixed; 5],
    pub reset: bool,
    pub expose_rects: *mut XRectangle,
    pub size_expose: libc::c_int,
    pub n_expose: libc::c_int,
    pub list: *mut _win,
    pub active_win: *mut _win,
    pub active_leader: Window,
    pub black_picture: Picture,
    pub cshadow_picture: Picture,
    pub white_picture: Picture,
    pub gaussian_map: *mut conv,
    pub cgsize: libc::c_int,
    pub shadow_corner: *mut libc::c_uchar,
    pub shadow_top: *mut libc::c_uchar,
    pub shadow_exclude_reg: XserverRegion,
    pub refresh_rate: libc::c_short,
    pub refresh_intv: libc::c_long,
    pub paint_tm_offset: libc::c_long,
    pub xfixes_event: libc::c_int,
    pub xfixes_error: libc::c_int,
    pub damage_event: libc::c_int,
    pub damage_error: libc::c_int,
    pub render_event: libc::c_int,
    pub render_error: libc::c_int,
    pub composite_event: libc::c_int,
    pub composite_error: libc::c_int,
    pub composite_opcode: libc::c_int,
    pub has_name_pixmap: bool,
    pub shape_exists: bool,
    pub shape_event: libc::c_int,
    pub shape_error: libc::c_int,
    pub randr_exists: bool,
    pub randr_event: libc::c_int,
    pub randr_error: libc::c_int,
    pub dbe_exists: bool,
    pub xsync_exists: bool,
    pub xsync_event: libc::c_int,
    pub xsync_error: libc::c_int,
    pub xrfilter_convolution_exists: bool,
    pub atom_opacity: Atom,
    pub atom_frame_extents: Atom,
    pub atom_client: Atom,
    pub atom_name: Atom,
    pub atom_name_ewmh: Atom,
    pub atom_class: Atom,
    pub atom_role: Atom,
    pub atom_transient: Atom,
    pub atom_client_leader: Atom,
    pub atom_ewmh_active_win: Atom,
    pub atom_compton_shadow: Atom,
    pub atom_win_type: Atom,
    pub atoms_wintypes: [Atom; 15],
    pub track_atom_lst: *mut latom_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _win {
    pub next: *mut _win,
    pub prev_trans: *mut _win,
    pub id: Window,
    pub a: XWindowAttributes,
    pub pictfmt: *mut XRenderPictFormat,
    pub mode: winmode_t,
    pub damaged: bool,
    pub fence: XSyncFence,
    pub pixmap_damaged: bool,
    pub damage: Damage,
    pub paint: paint_t,
    pub border_size: XserverRegion,
    pub extents: XserverRegion,
    pub flags: int_fast16_t,
    pub need_configure: bool,
    pub queue_configure: XConfigureEvent,
    pub reg_ignore: XserverRegion,
    pub widthb: libc::c_int,
    pub heightb: libc::c_int,
    pub destroyed: bool,
    pub bounding_shaped: bool,
    pub rounded_corners: bool,
    pub to_paint: bool,
    pub paint_excluded: bool,
    pub unredir_if_possible_excluded: bool,
    pub in_openclose: bool,
    pub client_win: Window,
    pub window_type: wintype_t,
    pub wmwin: bool,
    pub leader: Window,
    pub cache_leader: Window,
    pub focused: bool,
    pub focused_force: switch_t,
    pub name: *mut libc::c_char,
    pub class_instance: *mut libc::c_char,
    pub class_general: *mut libc::c_char,
    pub role: *mut libc::c_char,
    pub cache_sblst: *const c2_lptr_t,
    pub cache_fblst: *const c2_lptr_t,
    pub cache_fcblst: *const c2_lptr_t,
    pub cache_ivclst: *const c2_lptr_t,
    pub cache_bbblst: *const c2_lptr_t,
    pub cache_oparule: *const c2_lptr_t,
    pub cache_pblst: *const c2_lptr_t,
    pub cache_uipblst: *const c2_lptr_t,
    pub opacity: opacity_t,
    pub opacity_tgt: opacity_t,
    pub opacity_prop: opacity_t,
    pub opacity_prop_client: opacity_t,
    pub opacity_set: opacity_t,
    pub fade: bool,
    pub fade_last: bool,
    pub fade_force: switch_t,
    pub fade_callback: Option::<unsafe extern "C" fn(*mut session_t, *mut _win) -> ()>,
    pub frame_opacity: libc::c_double,
    pub frame_extents: margin_t,
    pub shadow: bool,
    pub shadow_last: bool,
    pub shadow_force: switch_t,
    pub shadow_opacity: libc::c_double,
    pub shadow_dx: libc::c_int,
    pub shadow_dy: libc::c_int,
    pub shadow_width: libc::c_int,
    pub shadow_height: libc::c_int,
    pub shadow_paint: paint_t,
    pub prop_shadow: libc::c_long,
    pub dim: bool,
    pub invert_color: bool,
    pub invert_color_last: bool,
    pub invert_color_force: switch_t,
    pub blur_background: bool,
    pub blur_background_last: bool,
}
pub type c2_lptr_t = _c2_lptr;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _c2_lptr {
    pub ptr: c2_ptr_t,
    pub data: *mut libc::c_void,
    pub next: *mut _c2_lptr,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct c2_ptr_t {
    #[bitfield(name = "isbranch", ty = "bool", bits = "0..=0")]
    pub isbranch: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
    pub c2rust_unnamed: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub b: *mut c2_b_t,
    pub l: *mut c2_l_t,
}
pub type c2_l_t = _c2_l;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct _c2_l {
    #[bitfield(name = "neg", ty = "bool", bits = "0..=0")]
    #[bitfield(name = "op", ty = "C2RustUnnamed_5", bits = "1..=3")]
    #[bitfield(name = "match_0", ty = "C2RustUnnamed_4", bits = "4..=6")]
    #[bitfield(name = "match_ignorecase", ty = "bool", bits = "7..=7")]
    pub neg_op_match_0_match_ignorecase: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
    pub tgt: *mut libc::c_char,
    pub tgtatom: Atom,
    pub tgt_onframe: bool,
    pub index: libc::c_int,
    pub predef: C2RustUnnamed_3,
    pub type_0: c2_l_type,
    pub format: libc::c_int,
    pub ptntype: C2RustUnnamed_2,
    pub ptnstr: *mut libc::c_char,
    pub ptnint: libc::c_long,
}
pub type C2RustUnnamed_2 = libc::c_uint;
pub const C2_L_PTINT: C2RustUnnamed_2 = 2;
pub const C2_L_PTSTRING: C2RustUnnamed_2 = 1;
pub const C2_L_PTUNDEFINED: C2RustUnnamed_2 = 0;
pub type c2_l_type = libc::c_uint;
pub const C2_L_TDRAWABLE: c2_l_type = 5;
pub const C2_L_TATOM: c2_l_type = 4;
pub const C2_L_TWINDOW: c2_l_type = 3;
pub const C2_L_TCARDINAL: c2_l_type = 2;
pub const C2_L_TSTRING: c2_l_type = 1;
pub const C2_L_TUNDEFINED: c2_l_type = 0;
pub type C2RustUnnamed_3 = libc::c_uint;
pub const C2_L_PROLE: C2RustUnnamed_3 = 24;
pub const C2_L_PCLASSI: C2RustUnnamed_3 = 23;
pub const C2_L_PCLASSG: C2RustUnnamed_3 = 22;
pub const C2_L_PNAME: C2RustUnnamed_3 = 21;
pub const C2_L_PLEADER: C2RustUnnamed_3 = 20;
pub const C2_L_PWINDOWTYPE: C2RustUnnamed_3 = 19;
pub const C2_L_PCLIENT: C2RustUnnamed_3 = 18;
pub const C2_L_PROUNDED: C2RustUnnamed_3 = 17;
pub const C2_L_PBSHAPED: C2RustUnnamed_3 = 16;
pub const C2_L_PWMWIN: C2RustUnnamed_3 = 15;
pub const C2_L_PFOCUSED: C2RustUnnamed_3 = 14;
pub const C2_L_PARGB: C2RustUnnamed_3 = 13;
pub const C2_L_POVREDIR: C2RustUnnamed_3 = 12;
pub const C2_L_PFULLSCREEN: C2RustUnnamed_3 = 11;
pub const C2_L_PBDW: C2RustUnnamed_3 = 10;
pub const C2_L_PHEIGHTB: C2RustUnnamed_3 = 9;
pub const C2_L_PWIDTHB: C2RustUnnamed_3 = 8;
pub const C2_L_PHEIGHT: C2RustUnnamed_3 = 7;
pub const C2_L_PWIDTH: C2RustUnnamed_3 = 6;
pub const C2_L_PY2: C2RustUnnamed_3 = 5;
pub const C2_L_PX2: C2RustUnnamed_3 = 4;
pub const C2_L_PY: C2RustUnnamed_3 = 3;
pub const C2_L_PX: C2RustUnnamed_3 = 2;
pub const C2_L_PID: C2RustUnnamed_3 = 1;
pub const C2_L_PUNDEFINED: C2RustUnnamed_3 = 0;
pub type C2RustUnnamed_4 = libc::c_uint;
pub const C2_L_MPCRE: C2RustUnnamed_4 = 4;
pub const C2_L_MWILDCARD: C2RustUnnamed_4 = 3;
pub const C2_L_MCONTAINS: C2RustUnnamed_4 = 2;
pub const C2_L_MSTART: C2RustUnnamed_4 = 1;
pub const C2_L_MEXACT: C2RustUnnamed_4 = 0;
pub type C2RustUnnamed_5 = libc::c_uint;
pub const C2_L_OLTEQ: C2RustUnnamed_5 = 5;
pub const C2_L_OLT: C2RustUnnamed_5 = 4;
pub const C2_L_OGTEQ: C2RustUnnamed_5 = 3;
pub const C2_L_OGT: C2RustUnnamed_5 = 2;
pub const C2_L_OEQ: C2RustUnnamed_5 = 1;
pub const C2_L_OEXISTS: C2RustUnnamed_5 = 0;
pub type c2_b_t = _c2_b;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct _c2_b {
    #[bitfield(name = "neg", ty = "bool", bits = "0..=0")]
    pub neg: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
    pub op: c2_b_op_t,
    pub opr1: c2_ptr_t,
    pub opr2: c2_ptr_t,
}
pub type c2_b_op_t = libc::c_uint;
pub const C2_B_OXOR: c2_b_op_t = 3;
pub const C2_B_OOR: c2_b_op_t = 2;
pub const C2_B_OAND: c2_b_op_t = 1;
pub const C2_B_OUNDEFINED: c2_b_op_t = 0;
pub type options_t = _options_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _options_t {
    pub config_file: *mut libc::c_char,
    pub write_pid_path: *mut libc::c_char,
    pub display: *mut libc::c_char,
    pub display_repr: *mut libc::c_char,
    pub backend: backend,
    pub xrender_sync: bool,
    pub xrender_sync_fence: bool,
    pub glx_no_stencil: bool,
    pub glx_copy_from_front: bool,
    pub glx_use_copysubbuffermesa: bool,
    pub glx_no_rebind_pixmap: bool,
    pub glx_swap_method: libc::c_int,
    pub glx_use_gpushader4: bool,
    pub glx_fshader_win_str: *mut libc::c_char,
    pub fork_after_register: bool,
    pub detect_rounded_corners: bool,
    pub paint_on_overlay: bool,
    pub force_win_blend: bool,
    pub resize_damage: libc::c_int,
    pub unredir_if_possible: bool,
    pub unredir_if_possible_blacklist: *mut c2_lptr_t,
    pub unredir_if_possible_delay: time_ms_t,
    pub redirected_force: switch_t,
    pub stoppaint_force: switch_t,
    pub reredir_on_root_change: bool,
    pub glx_reinit_on_root_change: bool,
    pub dbus: bool,
    pub logpath: *mut libc::c_char,
    pub benchmark: libc::c_int,
    pub benchmark_wid: Window,
    pub paint_blacklist: *mut c2_lptr_t,
    pub no_name_pixmap: bool,
    pub synchronize: bool,
    pub show_all_xerrors: bool,
    pub no_x_selection: bool,
    pub refresh_rate: libc::c_int,
    pub sw_opti: bool,
    pub vsync: vsync_t,
    pub dbe: bool,
    pub vsync_aggressive: bool,
    pub vsync_use_glfinish: bool,
    pub wintype_shadow: [bool; 15],
    pub shadow_red: libc::c_double,
    pub shadow_green: libc::c_double,
    pub shadow_blue: libc::c_double,
    pub shadow_radius: libc::c_int,
    pub shadow_offset_x: libc::c_int,
    pub shadow_offset_y: libc::c_int,
    pub shadow_opacity: libc::c_double,
    pub clear_shadow: bool,
    pub shadow_exclude_reg_geom: geometry_t,
    pub shadow_blacklist: *mut c2_lptr_t,
    pub shadow_ignore_shaped: bool,
    pub respect_prop_shadow: bool,
    pub xinerama_shadow_crop: bool,
    pub wintype_fade: [bool; 15],
    pub fade_in_step: opacity_t,
    pub fade_out_step: opacity_t,
    pub fade_delta: time_ms_t,
    pub no_fading_openclose: bool,
    pub no_fading_destroyed_argb: bool,
    pub fade_blacklist: *mut c2_lptr_t,
    pub wintype_opacity: [libc::c_double; 15],
    pub inactive_opacity: opacity_t,
    pub active_opacity: opacity_t,
    pub inactive_opacity_override: bool,
    pub frame_opacity: libc::c_double,
    pub detect_client_opacity: bool,
    pub alpha_step: libc::c_double,
    pub blur_background: bool,
    pub blur_background_frame: bool,
    pub blur_background_fixed: bool,
    pub blur_background_blacklist: *mut c2_lptr_t,
    pub blur_kerns: [*mut XFixed; 5],
    pub inactive_dim: libc::c_double,
    pub inactive_dim_fixed: bool,
    pub invert_color_list: *mut c2_lptr_t,
    pub opacity_rules: *mut c2_lptr_t,
    pub wintype_focus: [bool; 15],
    pub mark_wmwin_focused: bool,
    pub mark_ovredir_focused: bool,
    pub use_ewmh_active_win: bool,
    pub focus_blacklist: *mut c2_lptr_t,
    pub detect_transient: bool,
    pub detect_client_leader: bool,
    pub track_focus: bool,
    pub track_wdata: bool,
    pub track_leader: bool,
}
pub type win = _win;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct c2_predef_t {
    pub name: *const libc::c_char,
    pub type_0: c2_l_type,
    pub format: libc::c_int,
}
#[inline]
unsafe extern "C" fn tolower(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_tolower_loc()).offset(__c as isize)
    } else {
        __c
    };
}
#[inline]
unsafe extern "C" fn allocchk_(
    mut func_name: *const libc::c_char,
    mut ptr: *mut libc::c_void,
) -> *mut libc::c_void {
    if ptr.is_null() {
        fprintf(
            stderr,
            b"%s(): Failed to allocate memory.\n\0" as *const u8 as *const libc::c_char,
            func_name,
        );
        exit(1 as libc::c_int);
    }
    return ptr;
}
#[inline]
unsafe extern "C" fn mstrcpy(mut src: *const libc::c_char) -> *mut libc::c_char {
    let mut str: *mut libc::c_char = allocchk_(
        (*::std::mem::transmute::<&[u8; 8], &[libc::c_char; 8]>(b"mstrcpy\0")).as_ptr(),
        malloc(
            (strlen(src))
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
        ),
    ) as *mut libc::c_char;
    strcpy(str, src);
    return str;
}
#[inline]
unsafe extern "C" fn mstrncpy(
    mut src: *const libc::c_char,
    mut len: libc::c_uint,
) -> *mut libc::c_char {
    let mut str: *mut libc::c_char = allocchk_(
        (*::std::mem::transmute::<&[u8; 9], &[libc::c_char; 9]>(b"mstrncpy\0")).as_ptr(),
        malloc(
            (len.wrapping_add(1 as libc::c_int as libc::c_uint) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
        ),
    ) as *mut libc::c_char;
    strncpy(str, src, len as libc::c_ulong);
    *str.offset(len as isize) = '\0' as i32 as libc::c_char;
    return str;
}
#[inline]
unsafe extern "C" fn cxfree(mut data: *mut libc::c_void) {
    if !data.is_null() {
        XFree(data);
    }
}
#[inline]
unsafe extern "C" fn get_atom(
    mut ps: *mut session_t,
    mut atom_name: *const libc::c_char,
) -> Atom {
    return XInternAtom((*ps).dpy, atom_name, 0 as libc::c_int);
}
#[inline]
unsafe extern "C" fn win_is_focused_real(
    mut ps: *mut session_t,
    mut w: *const win,
) -> bool {
    return 2 as libc::c_int == (*w).a.map_state && (*ps).active_win == w as *mut _win;
}
#[inline]
unsafe extern "C" fn rect_is_fullscreen(
    mut ps: *mut session_t,
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut wid: libc::c_uint,
    mut hei: libc::c_uint,
) -> bool {
    return x <= 0 as libc::c_int && y <= 0 as libc::c_int
        && (x as libc::c_uint).wrapping_add(wid) >= (*ps).root_width as libc::c_uint
        && (y as libc::c_uint).wrapping_add(hei) >= (*ps).root_height as libc::c_uint;
}
#[inline]
unsafe extern "C" fn win_is_fullscreen(
    mut ps: *mut session_t,
    mut w: *const win,
) -> bool {
    return rect_is_fullscreen(
        ps,
        (*w).a.x,
        (*w).a.y,
        (*w).widthb as libc::c_uint,
        (*w).heightb as libc::c_uint,
    ) as libc::c_int != 0
        && (!(*w).bounding_shaped || (*w).rounded_corners as libc::c_int != 0);
}
#[inline]
unsafe extern "C" fn winprop_get_int(mut prop: winprop_t) -> libc::c_long {
    let mut tgt: libc::c_long = 0 as libc::c_int as libc::c_long;
    if prop.nitems == 0 {
        return 0 as libc::c_int as libc::c_long;
    }
    match prop.format {
        8 => {
            tgt = *prop.data.p8 as libc::c_long;
        }
        16 => {
            tgt = *prop.data.p16 as libc::c_long;
        }
        32 => {
            tgt = *prop.data.p32;
        }
        _ => {}
    }
    return tgt;
}
#[inline]
unsafe extern "C" fn free_winprop(mut pprop: *mut winprop_t) {
    if !((*pprop).data.p8).is_null() {
        cxfree((*pprop).data.p8 as *mut libc::c_void);
        (*pprop).data.p8 = 0 as *mut libc::c_uchar;
    }
    (*pprop).nitems = 0 as libc::c_int as libc::c_ulong;
}
#[inline]
unsafe extern "C" fn c2_freep(mut pp: *mut c2_ptr_t) {
    if !pp.is_null() {
        c2_free(*pp);
        c2_ptr_reset(pp);
    }
}
#[inline]
unsafe extern "C" fn c2_ptr_reset(mut pp: *mut c2_ptr_t) {
    if !pp.is_null() {
        memcpy(
            pp as *mut libc::c_void,
            &C2_PTR_NULL as *const c2_ptr_t as *const libc::c_void,
            ::std::mem::size_of::<c2_ptr_t>() as libc::c_ulong,
        );
    }
}
static mut C2_PTR_NULL: c2_ptr_t = c2_ptr_t {
    isbranch: [0; 1],
    c2rust_padding: [0; 7],
    c2rust_unnamed: C2RustUnnamed_1 {
        b: 0 as *const c2_b_t as *mut c2_b_t,
    },
};
#[inline]
unsafe extern "C" fn c2h_comb_tree(
    mut op: c2_b_op_t,
    mut p1: c2_ptr_t,
    mut p2: c2_ptr_t,
) -> c2_ptr_t {
    let mut p: c2_ptr_t = {
        let mut init = c2_ptr_t {
            isbranch: [0; 1],
            c2rust_padding: [0; 7],
            c2rust_unnamed: C2RustUnnamed_1 {
                b: malloc(::std::mem::size_of::<c2_b_t>() as libc::c_ulong)
                    as *mut c2_b_t,
            },
        };
        init.set_isbranch(1 as libc::c_int != 0);
        init
    };
    (*p.c2rust_unnamed.b).opr1 = p1;
    (*p.c2rust_unnamed.b).opr2 = p2;
    (*p.c2rust_unnamed.b).op = op;
    return p;
}
#[inline]
unsafe extern "C" fn strcmp_wd(
    mut needle: *const libc::c_char,
    mut src: *const libc::c_char,
) -> libc::c_int {
    let mut ret: libc::c_int = strncmp(needle, src, strlen(needle));
    if ret != 0 {
        return ret;
    }
    let mut c: libc::c_char = *src.offset(strlen(needle) as isize);
    if *(*__ctype_b_loc()).offset(c as libc::c_int as isize) as libc::c_int
        & _ISalnum as libc::c_int as libc::c_ushort as libc::c_int != 0
        || '_' as i32 == c as libc::c_int
    {
        return 1 as libc::c_int
    } else {
        return 0 as libc::c_int
    };
}
static mut C2_PREDEFS: [c2_predef_t; 25] = [
    c2_predef_t {
        name: 0 as *const libc::c_char,
        type_0: C2_L_TUNDEFINED,
        format: 0,
    },
    {
        let mut init = c2_predef_t {
            name: b"id\0" as *const u8 as *const libc::c_char,
            type_0: C2_L_TCARDINAL,
            format: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = c2_predef_t {
            name: b"x\0" as *const u8 as *const libc::c_char,
            type_0: C2_L_TCARDINAL,
            format: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = c2_predef_t {
            name: b"y\0" as *const u8 as *const libc::c_char,
            type_0: C2_L_TCARDINAL,
            format: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = c2_predef_t {
            name: b"x2\0" as *const u8 as *const libc::c_char,
            type_0: C2_L_TCARDINAL,
            format: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = c2_predef_t {
            name: b"y2\0" as *const u8 as *const libc::c_char,
            type_0: C2_L_TCARDINAL,
            format: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = c2_predef_t {
            name: b"width\0" as *const u8 as *const libc::c_char,
            type_0: C2_L_TCARDINAL,
            format: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = c2_predef_t {
            name: b"height\0" as *const u8 as *const libc::c_char,
            type_0: C2_L_TCARDINAL,
            format: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = c2_predef_t {
            name: b"widthb\0" as *const u8 as *const libc::c_char,
            type_0: C2_L_TCARDINAL,
            format: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = c2_predef_t {
            name: b"heightb\0" as *const u8 as *const libc::c_char,
            type_0: C2_L_TCARDINAL,
            format: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = c2_predef_t {
            name: b"border_width\0" as *const u8 as *const libc::c_char,
            type_0: C2_L_TCARDINAL,
            format: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = c2_predef_t {
            name: b"fullscreen\0" as *const u8 as *const libc::c_char,
            type_0: C2_L_TCARDINAL,
            format: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = c2_predef_t {
            name: b"override_redirect\0" as *const u8 as *const libc::c_char,
            type_0: C2_L_TCARDINAL,
            format: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = c2_predef_t {
            name: b"argb\0" as *const u8 as *const libc::c_char,
            type_0: C2_L_TCARDINAL,
            format: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = c2_predef_t {
            name: b"focused\0" as *const u8 as *const libc::c_char,
            type_0: C2_L_TCARDINAL,
            format: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = c2_predef_t {
            name: b"wmwin\0" as *const u8 as *const libc::c_char,
            type_0: C2_L_TCARDINAL,
            format: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = c2_predef_t {
            name: b"bounding_shaped\0" as *const u8 as *const libc::c_char,
            type_0: C2_L_TCARDINAL,
            format: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = c2_predef_t {
            name: b"rounded_corners\0" as *const u8 as *const libc::c_char,
            type_0: C2_L_TCARDINAL,
            format: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = c2_predef_t {
            name: b"client\0" as *const u8 as *const libc::c_char,
            type_0: C2_L_TWINDOW,
            format: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = c2_predef_t {
            name: b"window_type\0" as *const u8 as *const libc::c_char,
            type_0: C2_L_TSTRING,
            format: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = c2_predef_t {
            name: b"leader\0" as *const u8 as *const libc::c_char,
            type_0: C2_L_TWINDOW,
            format: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = c2_predef_t {
            name: b"name\0" as *const u8 as *const libc::c_char,
            type_0: C2_L_TSTRING,
            format: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = c2_predef_t {
            name: b"class_g\0" as *const u8 as *const libc::c_char,
            type_0: C2_L_TSTRING,
            format: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = c2_predef_t {
            name: b"class_i\0" as *const u8 as *const libc::c_char,
            type_0: C2_L_TSTRING,
            format: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = c2_predef_t {
            name: b"role\0" as *const u8 as *const libc::c_char,
            type_0: C2_L_TSTRING,
            format: 0 as libc::c_int,
        };
        init
    },
];
static mut leaf_def: c2_l_t = c2_l_t {
    neg_op_match_0_match_ignorecase: [0; 1],
    c2rust_padding: [0; 7],
    tgt: 0 as *const libc::c_char as *mut libc::c_char,
    tgtatom: 0,
    tgt_onframe: false,
    index: 0,
    predef: C2_L_PUNDEFINED,
    type_0: C2_L_TUNDEFINED,
    format: 0,
    ptntype: C2_L_PTUNDEFINED,
    ptnstr: 0 as *const libc::c_char as *mut libc::c_char,
    ptnint: 0,
};
#[inline]
unsafe extern "C" fn c2h_b_opcmp(mut op1: c2_b_op_t, mut op2: c2_b_op_t) -> libc::c_int {
    return c2h_b_opp(op1) - c2h_b_opp(op2);
}
#[inline]
unsafe extern "C" fn c2h_b_opp(mut op: c2_b_op_t) -> libc::c_int {
    match op as libc::c_uint {
        1 => return 2 as libc::c_int,
        2 => return 1 as libc::c_int,
        3 => return 1 as libc::c_int,
        _ => {}
    }
    return 0 as libc::c_int;
}
static mut lptr_def: c2_lptr_t = c2_lptr_t {
    ptr: c2_ptr_t {
        isbranch: [0; 1],
        c2rust_padding: [0; 7],
        c2rust_unnamed: C2RustUnnamed_1 {
            b: 0 as *const c2_b_t as *mut c2_b_t,
        },
    },
    data: 0 as *const libc::c_void as *mut libc::c_void,
    next: 0 as *const _c2_lptr as *mut _c2_lptr,
};
pub unsafe extern "C" fn c2_parsed(
    mut ps: *mut session_t,
    mut pcondlst: *mut *mut c2_lptr_t,
    mut pattern: *const libc::c_char,
    mut data: *mut libc::c_void,
) -> *mut c2_lptr_t {
    if pattern.is_null() {
        return 0 as *mut c2_lptr_t;
    }
    let mut result: c2_ptr_t = {
        let mut init = c2_ptr_t {
            isbranch: [0; 1],
            c2rust_padding: [0; 7],
            c2rust_unnamed: C2RustUnnamed_1 {
                l: 0 as *mut c2_l_t,
            },
        };
        init.set_isbranch(0 as libc::c_int != 0);
        init
    };
    let mut offset: libc::c_int = -(1 as libc::c_int);
    if strlen(pattern) >= 2 as libc::c_int as libc::c_ulong
        && ':' as i32 == *pattern.offset(1 as libc::c_int as isize) as libc::c_int
    {
        offset = c2_parse_legacy(ps, pattern, 0 as libc::c_int, &mut result);
    } else {
        offset = c2_parse_grp(
            ps,
            pattern,
            0 as libc::c_int,
            &mut result,
            0 as libc::c_int,
        );
    }
    if offset < 0 as libc::c_int {
        c2_freep(&mut result);
        return 0 as *mut c2_lptr_t;
    }
    let mut plptr: *mut c2_lptr_t = malloc(
        ::std::mem::size_of::<c2_lptr_t>() as libc::c_ulong,
    ) as *mut c2_lptr_t;
    if plptr.is_null() {
        fprintf(
            stderr,
            b"%s(): Failed to allocate memory for new condition linked list element.\n\0"
                as *const u8 as *const libc::c_char,
            (*::std::mem::transmute::<&[u8; 10], &[libc::c_char; 10]>(b"c2_parsed\0"))
                .as_ptr(),
        );
        exit(1 as libc::c_int);
    }
    memcpy(
        plptr as *mut libc::c_void,
        &lptr_def as *const c2_lptr_t as *const libc::c_void,
        ::std::mem::size_of::<c2_lptr_t>() as libc::c_ulong,
    );
    (*plptr).ptr = result;
    (*plptr).data = data;
    if !pcondlst.is_null() {
        (*plptr).next = *pcondlst;
        *pcondlst = plptr;
    }
    return plptr;
}
unsafe extern "C" fn c2_parse_grp(
    mut ps: *mut session_t,
    mut pattern: *const libc::c_char,
    mut offset: libc::c_int,
    mut presult: *mut c2_ptr_t,
    mut level: libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    if level > 10 as libc::c_int {
        fprintf(
            stderr,
            b"Pattern \"%s\" pos %d: Exceeded maximum recursion levels.\n\0" as *const u8
                as *const libc::c_char,
            pattern,
            offset,
        );
        return -(1 as libc::c_int);
    }
    if pattern.is_null() {
        return -(1 as libc::c_int);
    }
    let endchar: libc::c_char = (if offset != 0 { ')' as i32 } else { '\0' as i32 })
        as libc::c_char;
    let mut ops: [c2_b_op_t; 3] = [C2_B_OUNDEFINED, C2_B_OUNDEFINED, C2_B_OUNDEFINED];
    let mut eles: [c2_ptr_t; 2] = [
        {
            let mut init = c2_ptr_t {
                isbranch: [0; 1],
                c2rust_padding: [0; 7],
                c2rust_unnamed: C2RustUnnamed_1 {
                    l: 0 as *mut c2_l_t,
                },
            };
            init.set_isbranch(0 as libc::c_int != 0);
            init
        },
        {
            let mut init = c2_ptr_t {
                isbranch: [0; 1],
                c2rust_padding: [0; 7],
                c2rust_unnamed: C2RustUnnamed_1 {
                    l: 0 as *mut c2_l_t,
                },
            };
            init.set_isbranch(0 as libc::c_int != 0);
            init
        },
    ];
    let mut elei: libc::c_int = 0 as libc::c_int;
    let mut pele: *mut c2_ptr_t = eles.as_mut_ptr();
    let mut neg: bool = 0 as libc::c_int != 0;
    let mut next_expected: bool = 1 as libc::c_int != 0;
    loop {
        if !(*pattern.offset(offset as isize) != 0) {
            current_block = 168769493162332264;
            break;
        }
        if !(*(*__ctype_b_loc())
            .offset(*pattern.offset(offset as isize) as libc::c_int as isize)
            as libc::c_int & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
            != 0)
        {
            if ')' as i32 == *pattern.offset(offset as isize) as libc::c_int {
                current_block = 168769493162332264;
                break;
            }
            if '!' as i32 == *pattern.offset(offset as isize) as libc::c_int {
                if !next_expected {
                    fprintf(
                        stderr,
                        b"Pattern \"%s\" pos %d: Unexpected \"!\".\n\0" as *const u8
                            as *const libc::c_char,
                        pattern,
                        offset,
                    );
                    current_block = 3631272411116450638;
                    break;
                } else {
                    neg = !neg;
                }
            } else if '&' as i32 == *pattern.offset(offset as isize) as libc::c_int
                || '|' as i32 == *pattern.offset(offset as isize) as libc::c_int
            {
                if next_expected {
                    fprintf(
                        stderr,
                        b"Pattern \"%s\" pos %d: Unexpected logical operator.\n\0"
                            as *const u8 as *const libc::c_char,
                        pattern,
                        offset,
                    );
                    current_block = 3631272411116450638;
                    break;
                } else {
                    next_expected = 1 as libc::c_int != 0;
                    if strncmp(
                        b"&&\0" as *const u8 as *const libc::c_char,
                        pattern.offset(offset as isize),
                        strlen(b"&&\0" as *const u8 as *const libc::c_char),
                    ) == 0
                    {
                        ops[elei as usize] = C2_B_OAND;
                        offset += 1;
                        offset;
                    } else if strncmp(
                        b"||\0" as *const u8 as *const libc::c_char,
                        pattern.offset(offset as isize),
                        strlen(b"||\0" as *const u8 as *const libc::c_char),
                    ) == 0
                    {
                        ops[elei as usize] = C2_B_OOR;
                        offset += 1;
                        offset;
                    } else {
                        fprintf(
                            stderr,
                            b"Pattern \"%s\" pos %d: Illegal logical operator.\n\0"
                                as *const u8 as *const libc::c_char,
                            pattern,
                            offset,
                        );
                        current_block = 3631272411116450638;
                        break;
                    }
                }
            } else if !next_expected {
                fprintf(
                    stderr,
                    b"Pattern \"%s\" pos %d: Unexpected expression.\n\0" as *const u8
                        as *const libc::c_char,
                    pattern,
                    offset,
                );
                current_block = 3631272411116450638;
                break;
            } else {
                if 2 as libc::c_int == elei {
                    elei -= 1;
                    elei;
                    if c2h_b_opcmp(
                        ops[1 as libc::c_int as usize],
                        ops[2 as libc::c_int as usize],
                    ) >= 0 as libc::c_int
                    {
                        eles[0 as libc::c_int
                            as usize] = c2h_comb_tree(
                            ops[1 as libc::c_int as usize],
                            eles[0 as libc::c_int as usize],
                            eles[1 as libc::c_int as usize],
                        );
                        c2_ptr_reset(
                            &mut *eles.as_mut_ptr().offset(1 as libc::c_int as isize),
                        );
                        pele = &mut *eles.as_mut_ptr().offset(elei as isize)
                            as *mut c2_ptr_t;
                        ops[1 as libc::c_int as usize] = ops[2 as libc::c_int as usize];
                    } else {
                        eles[1 as libc::c_int
                            as usize] = c2h_comb_tree(
                            ops[2 as libc::c_int as usize],
                            eles[1 as libc::c_int as usize],
                            C2_PTR_NULL,
                        );
                        pele = &mut (*(*eles
                            .as_mut_ptr()
                            .offset(1 as libc::c_int as isize))
                            .c2rust_unnamed
                            .b)
                            .opr2;
                    }
                    ops[2 as libc::c_int as usize] = C2_B_OUNDEFINED;
                }
                if '(' as i32 == *pattern.offset(offset as isize) as libc::c_int {
                    offset = c2_parse_grp(
                        ps,
                        pattern,
                        offset + 1 as libc::c_int,
                        pele,
                        level + 1 as libc::c_int,
                    );
                    if offset < 0 as libc::c_int {
                        current_block = 3631272411116450638;
                        break;
                    }
                } else {
                    offset = c2_parse_target(ps, pattern, offset, pele);
                    if offset < 0 as libc::c_int {
                        current_block = 3631272411116450638;
                        break;
                    }
                    offset = c2_parse_op(pattern, offset, pele);
                    if offset < 0 as libc::c_int {
                        current_block = 3631272411116450638;
                        break;
                    }
                    offset = c2_parse_pattern(ps, pattern, offset, pele);
                    if offset < 0 as libc::c_int {
                        current_block = 3631272411116450638;
                        break;
                    }
                    if !c2_l_postprocess(ps, (*pele).c2rust_unnamed.l) {
                        current_block = 3631272411116450638;
                        break;
                    }
                }
                offset -= 1;
                offset;
                if neg {
                    neg = 0 as libc::c_int != 0;
                    if (*pele).isbranch() {
                        (*(*pele).c2rust_unnamed.b)
                            .set_neg(!(*(*pele).c2rust_unnamed.b).neg());
                    } else {
                        (*(*pele).c2rust_unnamed.l)
                            .set_neg(!(*(*pele).c2rust_unnamed.l).neg());
                    }
                }
                next_expected = 0 as libc::c_int != 0;
                elei += 1;
                elei;
                pele = &mut *eles.as_mut_ptr().offset(elei as isize) as *mut c2_ptr_t;
            }
        }
        offset += 1;
        offset;
    }
    match current_block {
        168769493162332264 => {
            if *pattern.offset(offset as isize) as libc::c_int != 0 && endchar == 0 {
                fprintf(
                    stderr,
                    b"Pattern \"%s\" pos %d: Expected end of string but found '%c'.\n\0"
                        as *const u8 as *const libc::c_char,
                    pattern,
                    offset,
                    *pattern.offset(offset as isize) as libc::c_int,
                );
            } else if *pattern.offset(offset as isize) == 0
                && endchar as libc::c_int != 0
            {
                fprintf(
                    stderr,
                    b"Pattern \"%s\" pos %d: Expected '%c' but found end of string.\n\0"
                        as *const u8 as *const libc::c_char,
                    pattern,
                    offset,
                    endchar as libc::c_int,
                );
            } else if elei == 0 {
                fprintf(
                    stderr,
                    b"Pattern \"%s\" pos %d: Empty group.\n\0" as *const u8
                        as *const libc::c_char,
                    pattern,
                    offset,
                );
            } else if next_expected {
                fprintf(
                    stderr,
                    b"Pattern \"%s\" pos %d: Missing rule before end of group.\n\0"
                        as *const u8 as *const libc::c_char,
                    pattern,
                    offset,
                );
            } else {
                if elei > 1 as libc::c_int {
                    eles[0 as libc::c_int
                        as usize] = c2h_comb_tree(
                        ops[1 as libc::c_int as usize],
                        eles[0 as libc::c_int as usize],
                        eles[1 as libc::c_int as usize],
                    );
                    c2_ptr_reset(
                        &mut *eles.as_mut_ptr().offset(1 as libc::c_int as isize),
                    );
                }
                *presult = eles[0 as libc::c_int as usize];
                if ')' as i32 == *pattern.offset(offset as isize) as libc::c_int {
                    offset += 1;
                    offset;
                }
                return offset;
            }
        }
        _ => {}
    }
    c2_freep(&mut *eles.as_mut_ptr().offset(0 as libc::c_int as isize));
    c2_freep(&mut *eles.as_mut_ptr().offset(1 as libc::c_int as isize));
    return -(1 as libc::c_int);
}
unsafe extern "C" fn c2_parse_target(
    mut ps: *mut session_t,
    mut pattern: *const libc::c_char,
    mut offset: libc::c_int,
    mut presult: *mut c2_ptr_t,
) -> libc::c_int {
    (*presult).set_isbranch(0 as libc::c_int != 0);
    (*presult)
        .c2rust_unnamed
        .l = malloc(::std::mem::size_of::<c2_l_t>() as libc::c_ulong) as *mut c2_l_t;
    if ((*presult).c2rust_unnamed.l).is_null() {
        fprintf(
            stderr,
            b"Pattern \"%s\" pos %d: Failed to allocate memory for new leaf.\n\0"
                as *const u8 as *const libc::c_char,
            pattern,
            offset,
        );
        return -(1 as libc::c_int);
    }
    let pleaf: *mut c2_l_t = (*presult).c2rust_unnamed.l;
    memcpy(
        pleaf as *mut libc::c_void,
        &leaf_def as *const c2_l_t as *const libc::c_void,
        ::std::mem::size_of::<c2_l_t>() as libc::c_ulong,
    );
    while '!' as i32 == *pattern.offset(offset as isize) as libc::c_int {
        (*pleaf).set_neg(!(*pleaf).neg());
        offset += 1;
        offset;
        while *(*__ctype_b_loc())
            .offset(*pattern.offset(offset as isize) as libc::c_int as isize)
            as libc::c_int & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
            != 0
        {
            offset += 1;
            offset;
        }
    }
    let mut tgtlen: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while *pattern.offset(offset as isize) as libc::c_int != 0
        && (*(*__ctype_b_loc())
            .offset(*pattern.offset(offset as isize) as libc::c_int as isize)
            as libc::c_int & _ISalnum as libc::c_int as libc::c_ushort as libc::c_int
            != 0 || '_' as i32 == *pattern.offset(offset as isize) as libc::c_int)
    {
        tgtlen = tgtlen.wrapping_add(1);
        tgtlen;
        offset += 1;
        offset;
    }
    if tgtlen == 0 {
        fprintf(
            stderr,
            b"Pattern \"%s\" pos %d: Empty target.\n\0" as *const u8
                as *const libc::c_char,
            pattern,
            offset,
        );
        return -(1 as libc::c_int);
    }
    (*pleaf)
        .tgt = mstrncpy(
        &*pattern.offset((offset as libc::c_uint).wrapping_sub(tgtlen) as isize),
        tgtlen,
    );
    let mut i: libc::c_uint = 1 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong)
        < (::std::mem::size_of::<[c2_predef_t; 25]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<c2_predef_t>() as libc::c_ulong)
    {
        if strcmp(C2_PREDEFS[i as usize].name, (*pleaf).tgt) == 0 {
            (*pleaf).predef = i as C2RustUnnamed_3;
            (*pleaf).type_0 = C2_PREDEFS[i as usize].type_0;
            (*pleaf).format = C2_PREDEFS[i as usize].format;
            break;
        } else {
            i = i.wrapping_add(1);
            i;
        }
    }
    (*pleaf).predef as u64 == 0;
    while *(*__ctype_b_loc())
        .offset(*pattern.offset(offset as isize) as libc::c_int as isize) as libc::c_int
        & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        offset += 1;
        offset;
    }
    if '@' as i32 == *pattern.offset(offset as isize) as libc::c_int {
        (*pleaf).tgt_onframe = 1 as libc::c_int != 0;
        offset += 1;
        offset;
        while *(*__ctype_b_loc())
            .offset(*pattern.offset(offset as isize) as libc::c_int as isize)
            as libc::c_int & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
            != 0
        {
            offset += 1;
            offset;
        }
    }
    if '[' as i32 == *pattern.offset(offset as isize) as libc::c_int {
        offset += 1;
        offset;
        while *(*__ctype_b_loc())
            .offset(*pattern.offset(offset as isize) as libc::c_int as isize)
            as libc::c_int & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
            != 0
        {
            offset += 1;
            offset;
        }
        let mut index: libc::c_int = -(1 as libc::c_int);
        let mut endptr: *mut libc::c_char = 0 as *mut libc::c_char;
        index = strtol(pattern.offset(offset as isize), &mut endptr, 0 as libc::c_int)
            as libc::c_int;
        if endptr.is_null()
            || pattern.offset(offset as isize) == endptr as *const libc::c_char
        {
            fprintf(
                stderr,
                b"Pattern \"%s\" pos %d: No index number found after bracket.\n\0"
                    as *const u8 as *const libc::c_char,
                pattern,
                offset,
            );
            return -(1 as libc::c_int);
        }
        if index < 0 as libc::c_int {
            fprintf(
                stderr,
                b"Pattern \"%s\" pos %d: Index number invalid.\n\0" as *const u8
                    as *const libc::c_char,
                pattern,
                offset,
            );
            return -(1 as libc::c_int);
        }
        if (*pleaf).predef as u64 != 0 {
            fprintf(
                stderr,
                b"Pattern \"%s\" pos %d: Predefined targets can't have index.\n\0"
                    as *const u8 as *const libc::c_char,
                pattern,
                offset,
            );
            return -(1 as libc::c_int);
        }
        (*pleaf).index = index;
        offset = endptr.offset_from(pattern) as libc::c_long as libc::c_int;
        while *(*__ctype_b_loc())
            .offset(*pattern.offset(offset as isize) as libc::c_int as isize)
            as libc::c_int & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
            != 0
        {
            offset += 1;
            offset;
        }
        if ']' as i32 != *pattern.offset(offset as isize) as libc::c_int {
            fprintf(
                stderr,
                b"Pattern \"%s\" pos %d: Index end marker not found.\n\0" as *const u8
                    as *const libc::c_char,
                pattern,
                offset,
            );
            return -(1 as libc::c_int);
        }
        offset += 1;
        offset;
        while *(*__ctype_b_loc())
            .offset(*pattern.offset(offset as isize) as libc::c_int as isize)
            as libc::c_int & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
            != 0
        {
            offset += 1;
            offset;
        }
    }
    if ':' as i32 == *pattern.offset(offset as isize) as libc::c_int {
        offset += 1;
        offset;
        while *(*__ctype_b_loc())
            .offset(*pattern.offset(offset as isize) as libc::c_int as isize)
            as libc::c_int & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
            != 0
        {
            offset += 1;
            offset;
        }
        let mut hasformat: bool = 0 as libc::c_int != 0;
        let mut format: libc::c_int = 0 as libc::c_int;
        let mut endptr_0: *mut libc::c_char = 0 as *mut libc::c_char;
        format = strtol(pattern.offset(offset as isize), &mut endptr_0, 0 as libc::c_int)
            as libc::c_int;
        hasformat = !endptr_0.is_null()
            && endptr_0 != pattern.offset(offset as isize) as *mut libc::c_char;
        if hasformat {
            offset = endptr_0.offset_from(pattern) as libc::c_long as libc::c_int;
        }
        while *(*__ctype_b_loc())
            .offset(*pattern.offset(offset as isize) as libc::c_int as isize)
            as libc::c_int & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
            != 0
        {
            offset += 1;
            offset;
        }
        let mut type_0: c2_l_type = C2_L_TUNDEFINED;
        match *pattern.offset(offset as isize) as libc::c_int {
            119 => {
                type_0 = C2_L_TWINDOW;
            }
            100 => {
                type_0 = C2_L_TDRAWABLE;
            }
            99 => {
                type_0 = C2_L_TCARDINAL;
            }
            115 => {
                type_0 = C2_L_TSTRING;
            }
            97 => {
                type_0 = C2_L_TATOM;
            }
            _ => {
                fprintf(
                    stderr,
                    b"Pattern \"%s\" pos %d: Invalid type character.\n\0" as *const u8
                        as *const libc::c_char,
                    pattern,
                    offset,
                );
                return -(1 as libc::c_int);
            }
        }
        if type_0 as u64 != 0 {
            if (*pleaf).predef as u64 != 0 {
                fprintf(
                    stderr,
                    b"%s(): Warning: Type specified for a default target will be ignored.\n\0"
                        as *const u8 as *const libc::c_char,
                    (*::std::mem::transmute::<
                        &[u8; 16],
                        &[libc::c_char; 16],
                    >(b"c2_parse_target\0"))
                        .as_ptr(),
                );
            } else {
                if (*pleaf).type_0 as libc::c_uint != 0
                    && type_0 as libc::c_uint != (*pleaf).type_0 as libc::c_uint
                {
                    fprintf(
                        stderr,
                        b"%s(): Warning: Default type overridden on target.\n\0"
                            as *const u8 as *const libc::c_char,
                        (*::std::mem::transmute::<
                            &[u8; 16],
                            &[libc::c_char; 16],
                        >(b"c2_parse_target\0"))
                            .as_ptr(),
                    );
                }
                (*pleaf).type_0 = type_0;
            }
        }
        offset += 1;
        offset;
        while *(*__ctype_b_loc())
            .offset(*pattern.offset(offset as isize) as libc::c_int as isize)
            as libc::c_int & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
            != 0
        {
            offset += 1;
            offset;
        }
        if (*pleaf).format == 0 {
            match (*pleaf).type_0 as libc::c_uint {
                3 | 5 | 4 => {
                    (*pleaf).format = 32 as libc::c_int;
                }
                1 => {
                    (*pleaf).format = 8 as libc::c_int;
                }
                _ => {}
            }
        }
        if hasformat {
            if (*pleaf).predef as u64 != 0 {
                fprintf(
                    stderr,
                    b"%s(): Warning: Format \"%d\" specified on a default target will be ignored.\n\0"
                        as *const u8 as *const libc::c_char,
                    (*::std::mem::transmute::<
                        &[u8; 16],
                        &[libc::c_char; 16],
                    >(b"c2_parse_target\0"))
                        .as_ptr(),
                    format,
                );
            } else if C2_L_TSTRING as libc::c_int as libc::c_uint
                == (*pleaf).type_0 as libc::c_uint
            {
                fprintf(
                    stderr,
                    b"%s(): Warning: Format \"%d\" specified on a string target will be ignored.\n\0"
                        as *const u8 as *const libc::c_char,
                    (*::std::mem::transmute::<
                        &[u8; 16],
                        &[libc::c_char; 16],
                    >(b"c2_parse_target\0"))
                        .as_ptr(),
                    format,
                );
            } else {
                if (*pleaf).format != 0 && (*pleaf).format != format {
                    fprintf(
                        stderr,
                        b"Warning: Default format %d overridden on target.\n\0"
                            as *const u8 as *const libc::c_char,
                        (*pleaf).format,
                    );
                }
                (*pleaf).format = format;
            }
        }
    }
    if (*pleaf).type_0 as u64 == 0 {
        fprintf(
            stderr,
            b"Pattern \"%s\" pos %d: Target type cannot be determined.\n\0" as *const u8
                as *const libc::c_char,
            pattern,
            offset,
        );
        return -(1 as libc::c_int);
    }
    if (*pleaf).format != 0 && 8 as libc::c_int != (*pleaf).format
        && 16 as libc::c_int != (*pleaf).format && 32 as libc::c_int != (*pleaf).format
    {
        fprintf(
            stderr,
            b"Pattern \"%s\" pos %d: Invalid format.\n\0" as *const u8
                as *const libc::c_char,
            pattern,
            offset,
        );
        return -(1 as libc::c_int);
    }
    return offset;
}
unsafe extern "C" fn c2_parse_op(
    mut pattern: *const libc::c_char,
    mut offset: libc::c_int,
    mut presult: *mut c2_ptr_t,
) -> libc::c_int {
    let pleaf: *mut c2_l_t = (*presult).c2rust_unnamed.l;
    while *(*__ctype_b_loc())
        .offset(*pattern.offset(offset as isize) as libc::c_int as isize) as libc::c_int
        & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        offset += 1;
        offset;
    }
    while '!' as i32 == *pattern.offset(offset as isize) as libc::c_int {
        (*pleaf).set_neg(!(*pleaf).neg());
        offset += 1;
        offset;
        while *(*__ctype_b_loc())
            .offset(*pattern.offset(offset as isize) as libc::c_int as isize)
            as libc::c_int & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
            != 0
        {
            offset += 1;
            offset;
        }
    }
    if '*' as i32 == *pattern.offset(offset as isize) as libc::c_int
        || '^' as i32 == *pattern.offset(offset as isize) as libc::c_int
        || '%' as i32 == *pattern.offset(offset as isize) as libc::c_int
        || '~' as i32 == *pattern.offset(offset as isize) as libc::c_int
    {
        match *pattern.offset(offset as isize) as libc::c_int {
            42 => {
                (*pleaf).set_match_0(C2_L_MCONTAINS);
            }
            94 => {
                (*pleaf).set_match_0(C2_L_MSTART);
            }
            37 => {
                (*pleaf).set_match_0(C2_L_MWILDCARD);
            }
            126 => {
                (*pleaf).set_match_0(C2_L_MPCRE);
            }
            _ => {}
        }
        offset += 1;
        offset;
        while *(*__ctype_b_loc())
            .offset(*pattern.offset(offset as isize) as libc::c_int as isize)
            as libc::c_int & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
            != 0
        {
            offset += 1;
            offset;
        }
    }
    while '?' as i32 == *pattern.offset(offset as isize) as libc::c_int {
        (*pleaf).set_match_ignorecase(1 as libc::c_int != 0);
        offset += 1;
        offset;
        while *(*__ctype_b_loc())
            .offset(*pattern.offset(offset as isize) as libc::c_int as isize)
            as libc::c_int & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
            != 0
        {
            offset += 1;
            offset;
        }
    }
    while '=' as i32 == *pattern.offset(offset as isize) as libc::c_int
        || '>' as i32 == *pattern.offset(offset as isize) as libc::c_int
        || '<' as i32 == *pattern.offset(offset as isize) as libc::c_int
    {
        if '=' as i32 == *pattern.offset(offset as isize) as libc::c_int
            && C2_L_OGT as libc::c_int == (*pleaf).op() as libc::c_int
        {
            (*pleaf).set_op(C2_L_OGTEQ);
        } else if '=' as i32 == *pattern.offset(offset as isize) as libc::c_int
            && C2_L_OLT as libc::c_int == (*pleaf).op() as libc::c_int
        {
            (*pleaf).set_op(C2_L_OLTEQ);
        } else if (*pleaf).op() as u64 != 0 {
            fprintf(
                stderr,
                b"Pattern \"%s\" pos %d: Duplicate operator.\n\0" as *const u8
                    as *const libc::c_char,
                pattern,
                offset,
            );
            return -(1 as libc::c_int);
        } else {
            match *pattern.offset(offset as isize) as libc::c_int {
                61 => {
                    (*pleaf).set_op(C2_L_OEQ);
                }
                62 => {
                    (*pleaf).set_op(C2_L_OGT);
                }
                60 => {
                    (*pleaf).set_op(C2_L_OLT);
                }
                _ => {}
            }
        }
        offset += 1;
        offset;
        while *(*__ctype_b_loc())
            .offset(*pattern.offset(offset as isize) as libc::c_int as isize)
            as libc::c_int & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
            != 0
        {
            offset += 1;
            offset;
        }
    }
    if C2_L_OEQ as libc::c_int != (*pleaf).op() as libc::c_int
        && ((*pleaf).match_0() as libc::c_int != 0
            || (*pleaf).match_ignorecase() as libc::c_int != 0)
    {
        fprintf(
            stderr,
            b"Pattern \"%s\" pos %d: Exists/greater-than/less-than operators cannot have a qualifier.\n\0"
                as *const u8 as *const libc::c_char,
            pattern,
            offset,
        );
        return -(1 as libc::c_int);
    }
    return offset;
}
unsafe extern "C" fn c2_parse_pattern(
    mut ps: *mut session_t,
    mut pattern: *const libc::c_char,
    mut offset: libc::c_int,
    mut presult: *mut c2_ptr_t,
) -> libc::c_int {
    let pleaf: *mut c2_l_t = (*presult).c2rust_unnamed.l;
    if (*pleaf).op() as u64 == 0 {
        return offset;
    }
    while *(*__ctype_b_loc())
        .offset(*pattern.offset(offset as isize) as libc::c_int as isize) as libc::c_int
        & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        offset += 1;
        offset;
    }
    let mut endptr: *mut libc::c_char = 0 as *mut libc::c_char;
    if strcmp_wd(
        b"true\0" as *const u8 as *const libc::c_char,
        &*pattern.offset(offset as isize),
    ) == 0
    {
        (*pleaf).ptntype = C2_L_PTINT;
        (*pleaf).ptnint = 1 as libc::c_int as libc::c_long;
        offset = (offset as libc::c_ulong)
            .wrapping_add(strlen(b"true\0" as *const u8 as *const libc::c_char))
            as libc::c_int as libc::c_int;
    } else if strcmp_wd(
        b"false\0" as *const u8 as *const libc::c_char,
        &*pattern.offset(offset as isize),
    ) == 0
    {
        (*pleaf).ptntype = C2_L_PTINT;
        (*pleaf).ptnint = 0 as libc::c_int as libc::c_long;
        offset = (offset as libc::c_ulong)
            .wrapping_add(strlen(b"false\0" as *const u8 as *const libc::c_char))
            as libc::c_int as libc::c_int;
    } else {
        (*pleaf)
            .ptnint = strtol(
            pattern.offset(offset as isize),
            &mut endptr,
            0 as libc::c_int,
        );
        if pattern.offset(offset as isize) != endptr as *const libc::c_char {
            (*pleaf).ptntype = C2_L_PTINT;
            offset = endptr.offset_from(pattern) as libc::c_long as libc::c_int;
            if *(*__ctype_b_loc())
                .offset(*pattern.offset(offset as isize) as libc::c_int as isize)
                as libc::c_int & _ISalnum as libc::c_int as libc::c_ushort as libc::c_int
                != 0
            {
                fprintf(
                    stderr,
                    b"Pattern \"%s\" pos %d: Trailing characters after a numeric pattern.\n\0"
                        as *const u8 as *const libc::c_char,
                    pattern,
                    offset,
                );
                return -(1 as libc::c_int);
            }
        } else {
            let mut raw: bool = 0 as libc::c_int != 0;
            let mut delim: libc::c_char = '\0' as i32 as libc::c_char;
            if 'r' as i32
                == ({
                    let mut __res: libc::c_int = 0;
                    if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                        > 1 as libc::c_int as libc::c_ulong
                    {
                        if 0 != 0 {
                            let mut __c: libc::c_int = *pattern.offset(offset as isize)
                                as libc::c_int;
                            __res = (if __c < -(128 as libc::c_int)
                                || __c > 255 as libc::c_int
                            {
                                __c
                            } else {
                                *(*__ctype_tolower_loc()).offset(__c as isize)
                            });
                        } else {
                            __res = tolower(
                                *pattern.offset(offset as isize) as libc::c_int,
                            );
                        }
                    } else {
                        __res = *(*__ctype_tolower_loc())
                            .offset(
                                *pattern.offset(offset as isize) as libc::c_int as isize,
                            );
                    }
                    __res
                })
            {
                raw = 1 as libc::c_int != 0;
                offset += 1;
                offset;
                while *(*__ctype_b_loc())
                    .offset(*pattern.offset(offset as isize) as libc::c_int as isize)
                    as libc::c_int
                    & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
                {
                    offset += 1;
                    offset;
                }
            }
            if '"' as i32 == *pattern.offset(offset as isize) as libc::c_int
                || '\'' as i32 == *pattern.offset(offset as isize) as libc::c_int
            {
                (*pleaf).ptntype = C2_L_PTSTRING;
                delim = *pattern.offset(offset as isize);
                offset += 1;
                offset;
            }
            if C2_L_PTSTRING as libc::c_int as libc::c_uint
                != (*pleaf).ptntype as libc::c_uint
            {
                fprintf(
                    stderr,
                    b"Pattern \"%s\" pos %d: Invalid pattern type.\n\0" as *const u8
                        as *const libc::c_char,
                    pattern,
                    offset,
                );
                return -(1 as libc::c_int);
            }
            let mut tptnstr: *mut libc::c_char = malloc(
                (strlen(pattern.offset(offset as isize)))
                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
            ) as *mut libc::c_char;
            let mut ptptnstr: *mut libc::c_char = tptnstr;
            (*pleaf).ptnstr = tptnstr;
            while *pattern.offset(offset as isize) as libc::c_int != 0
                && delim as libc::c_int
                    != *pattern.offset(offset as isize) as libc::c_int
            {
                if '\\' as i32 == *pattern.offset(offset as isize) as libc::c_int && !raw
                {
                    offset += 1;
                    match *pattern.offset(offset as isize) as libc::c_int {
                        92 => {
                            let fresh0 = ptptnstr;
                            ptptnstr = ptptnstr.offset(1);
                            *fresh0 = '\\' as i32 as libc::c_char;
                        }
                        39 => {
                            let fresh1 = ptptnstr;
                            ptptnstr = ptptnstr.offset(1);
                            *fresh1 = '\'' as i32 as libc::c_char;
                        }
                        34 => {
                            let fresh2 = ptptnstr;
                            ptptnstr = ptptnstr.offset(1);
                            *fresh2 = '"' as i32 as libc::c_char;
                        }
                        97 => {
                            let fresh3 = ptptnstr;
                            ptptnstr = ptptnstr.offset(1);
                            *fresh3 = '\u{7}' as i32 as libc::c_char;
                        }
                        98 => {
                            let fresh4 = ptptnstr;
                            ptptnstr = ptptnstr.offset(1);
                            *fresh4 = '\u{8}' as i32 as libc::c_char;
                        }
                        102 => {
                            let fresh5 = ptptnstr;
                            ptptnstr = ptptnstr.offset(1);
                            *fresh5 = '\u{c}' as i32 as libc::c_char;
                        }
                        110 => {
                            let fresh6 = ptptnstr;
                            ptptnstr = ptptnstr.offset(1);
                            *fresh6 = '\n' as i32 as libc::c_char;
                        }
                        114 => {
                            let fresh7 = ptptnstr;
                            ptptnstr = ptptnstr.offset(1);
                            *fresh7 = '\r' as i32 as libc::c_char;
                        }
                        116 => {
                            let fresh8 = ptptnstr;
                            ptptnstr = ptptnstr.offset(1);
                            *fresh8 = '\t' as i32 as libc::c_char;
                        }
                        118 => {
                            let fresh9 = ptptnstr;
                            ptptnstr = ptptnstr.offset(1);
                            *fresh9 = '\u{b}' as i32 as libc::c_char;
                        }
                        111 | 120 => {
                            let mut tstr: *mut libc::c_char = mstrncpy(
                                pattern
                                    .offset(offset as isize)
                                    .offset(1 as libc::c_int as isize),
                                2 as libc::c_int as libc::c_uint,
                            );
                            let mut pstr: *mut libc::c_char = 0 as *mut libc::c_char;
                            let mut val: libc::c_long = strtol(
                                tstr,
                                &mut pstr,
                                if 'o' as i32
                                    == *pattern.offset(offset as isize) as libc::c_int
                                {
                                    8 as libc::c_int
                                } else {
                                    16 as libc::c_int
                                },
                            );
                            free(tstr as *mut libc::c_void);
                            if pstr
                                != &mut *tstr.offset(2 as libc::c_int as isize)
                                    as *mut libc::c_char
                                || val <= 0 as libc::c_int as libc::c_long
                            {
                                fprintf(
                                    stderr,
                                    b"Pattern \"%s\" pos %d: Invalid octal/hex escape sequence.\n\0"
                                        as *const u8 as *const libc::c_char,
                                    pattern,
                                    offset,
                                );
                                return -(1 as libc::c_int);
                            }
                            let fresh10 = ptptnstr;
                            ptptnstr = ptptnstr.offset(1);
                            *fresh10 = val as libc::c_char;
                            offset += 2 as libc::c_int;
                        }
                        _ => {
                            fprintf(
                                stderr,
                                b"Pattern \"%s\" pos %d: Invalid escape sequence.\n\0"
                                    as *const u8 as *const libc::c_char,
                                pattern,
                                offset,
                            );
                            return -(1 as libc::c_int);
                        }
                    }
                } else {
                    let fresh11 = ptptnstr;
                    ptptnstr = ptptnstr.offset(1);
                    *fresh11 = *pattern.offset(offset as isize);
                }
                offset += 1;
                offset;
            }
            if *pattern.offset(offset as isize) == 0 {
                fprintf(
                    stderr,
                    b"Pattern \"%s\" pos %d: Premature end of pattern string.\n\0"
                        as *const u8 as *const libc::c_char,
                    pattern,
                    offset,
                );
                return -(1 as libc::c_int);
            }
            offset += 1;
            offset;
            *ptptnstr = '\0' as i32 as libc::c_char;
            (*pleaf).ptnstr = mstrcpy(tptnstr);
            free(tptnstr as *mut libc::c_void);
        }
    }
    while *(*__ctype_b_loc())
        .offset(*pattern.offset(offset as isize) as libc::c_int as isize) as libc::c_int
        & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        offset += 1;
        offset;
    }
    if (*pleaf).ptntype as u64 == 0 {
        fprintf(
            stderr,
            b"Pattern \"%s\" pos %d: Invalid pattern type.\n\0" as *const u8
                as *const libc::c_char,
            pattern,
            offset,
        );
        return -(1 as libc::c_int);
    }
    if !((C2_L_TSTRING as libc::c_int as libc::c_uint == (*pleaf).type_0 as libc::c_uint
        || C2_L_TATOM as libc::c_int as libc::c_uint == (*pleaf).type_0 as libc::c_uint)
        && C2_L_PTSTRING as libc::c_int as libc::c_uint
            == (*pleaf).ptntype as libc::c_uint
        || (C2_L_TCARDINAL as libc::c_int as libc::c_uint
            == (*pleaf).type_0 as libc::c_uint
            || C2_L_TWINDOW as libc::c_int as libc::c_uint
                == (*pleaf).type_0 as libc::c_uint
            || C2_L_TDRAWABLE as libc::c_int as libc::c_uint
                == (*pleaf).type_0 as libc::c_uint)
            && C2_L_PTINT as libc::c_int as libc::c_uint
                == (*pleaf).ptntype as libc::c_uint)
    {
        fprintf(
            stderr,
            b"Pattern \"%s\" pos %d: Pattern type incompatible with target type.\n\0"
                as *const u8 as *const libc::c_char,
            pattern,
            offset,
        );
        return -(1 as libc::c_int);
    }
    if C2_L_PTINT as libc::c_int as libc::c_uint == (*pleaf).ptntype as libc::c_uint
        && (*pleaf).match_0() as libc::c_int != 0
    {
        fprintf(
            stderr,
            b"Pattern \"%s\" pos %d: Integer/boolean pattern cannot have operator qualifiers.\n\0"
                as *const u8 as *const libc::c_char,
            pattern,
            offset,
        );
        return -(1 as libc::c_int);
    }
    if C2_L_PTINT as libc::c_int as libc::c_uint == (*pleaf).ptntype as libc::c_uint
        && (*pleaf).match_ignorecase() as libc::c_int != 0
    {
        fprintf(
            stderr,
            b"Pattern \"%s\" pos %d: Integer/boolean pattern cannot have flags.\n\0"
                as *const u8 as *const libc::c_char,
            pattern,
            offset,
        );
        return -(1 as libc::c_int);
    }
    if C2_L_PTSTRING as libc::c_int as libc::c_uint == (*pleaf).ptntype as libc::c_uint
        && (C2_L_OGT as libc::c_int == (*pleaf).op() as libc::c_int
            || C2_L_OGTEQ as libc::c_int == (*pleaf).op() as libc::c_int
            || C2_L_OLT as libc::c_int == (*pleaf).op() as libc::c_int
            || C2_L_OLTEQ as libc::c_int == (*pleaf).op() as libc::c_int)
    {
        fprintf(
            stderr,
            b"Pattern \"%s\" pos %d: String pattern cannot have an arithmetic operator.\n\0"
                as *const u8 as *const libc::c_char,
            pattern,
            offset,
        );
        return -(1 as libc::c_int);
    }
    return offset;
}
unsafe extern "C" fn c2_parse_legacy(
    mut ps: *mut session_t,
    mut pattern: *const libc::c_char,
    mut offset: libc::c_int,
    mut presult: *mut c2_ptr_t,
) -> libc::c_int {
    let mut plen: libc::c_uint = strlen(pattern.offset(offset as isize)) as libc::c_uint;
    if plen < 4 as libc::c_int as libc::c_uint
        || ':' as i32
            != *pattern.offset((offset + 1 as libc::c_int) as isize) as libc::c_int
        || (strchr(
            pattern.offset(offset as isize).offset(2 as libc::c_int as isize),
            ':' as i32,
        ))
            .is_null()
    {
        fprintf(
            stderr,
            b"Pattern \"%s\" pos %d: Legacy parser: Invalid format.\n\0" as *const u8
                as *const libc::c_char,
            pattern,
            offset,
        );
        return -(1 as libc::c_int);
    }
    let mut pleaf: *mut c2_l_t = malloc(::std::mem::size_of::<c2_l_t>() as libc::c_ulong)
        as *mut c2_l_t;
    if pleaf.is_null() {
        fprintf(
            stderr,
            b"%s(): Failed to allocate memory for new leaf.\n\0" as *const u8
                as *const libc::c_char,
            (*::std::mem::transmute::<
                &[u8; 16],
                &[libc::c_char; 16],
            >(b"c2_parse_legacy\0"))
                .as_ptr(),
        );
        exit(1 as libc::c_int);
    }
    (*presult).set_isbranch(0 as libc::c_int != 0);
    (*presult).c2rust_unnamed.l = pleaf;
    memcpy(
        pleaf as *mut libc::c_void,
        &leaf_def as *const c2_l_t as *const libc::c_void,
        ::std::mem::size_of::<c2_l_t>() as libc::c_ulong,
    );
    (*pleaf).type_0 = C2_L_TSTRING;
    (*pleaf).set_op(C2_L_OEQ);
    (*pleaf).ptntype = C2_L_PTSTRING;
    match *pattern.offset(offset as isize) as libc::c_int {
        110 => {
            (*pleaf).predef = C2_L_PNAME;
            (*pleaf).type_0 = C2_PREDEFS[C2_L_PNAME as libc::c_int as usize].type_0;
            (*pleaf).format = C2_PREDEFS[C2_L_PNAME as libc::c_int as usize].format;
        }
        105 => {
            (*pleaf).predef = C2_L_PCLASSI;
            (*pleaf).type_0 = C2_PREDEFS[C2_L_PCLASSI as libc::c_int as usize].type_0;
            (*pleaf).format = C2_PREDEFS[C2_L_PCLASSI as libc::c_int as usize].format;
        }
        103 => {
            (*pleaf).predef = C2_L_PCLASSG;
            (*pleaf).type_0 = C2_PREDEFS[C2_L_PCLASSG as libc::c_int as usize].type_0;
            (*pleaf).format = C2_PREDEFS[C2_L_PCLASSG as libc::c_int as usize].format;
        }
        114 => {
            (*pleaf).predef = C2_L_PROLE;
            (*pleaf).type_0 = C2_PREDEFS[C2_L_PROLE as libc::c_int as usize].type_0;
            (*pleaf).format = C2_PREDEFS[C2_L_PROLE as libc::c_int as usize].format;
        }
        _ => {
            fprintf(
                stderr,
                b"Pattern \"%s\" pos %d: Target \"%c\" invalid.\n\n\0" as *const u8
                    as *const libc::c_char,
                pattern,
                offset,
                *pattern.offset(offset as isize) as libc::c_int,
            );
            return -(1 as libc::c_int);
        }
    }
    offset += 2 as libc::c_int;
    match *pattern.offset(offset as isize) as libc::c_int {
        101 => {
            (*pleaf).set_match_0(C2_L_MEXACT);
        }
        97 => {
            (*pleaf).set_match_0(C2_L_MCONTAINS);
        }
        115 => {
            (*pleaf).set_match_0(C2_L_MSTART);
        }
        119 => {
            (*pleaf).set_match_0(C2_L_MWILDCARD);
        }
        112 => {
            (*pleaf).set_match_0(C2_L_MPCRE);
        }
        _ => {
            fprintf(
                stderr,
                b"Pattern \"%s\" pos %d: Type \"%c\" invalid.\n\n\0" as *const u8
                    as *const libc::c_char,
                pattern,
                offset,
                *pattern.offset(offset as isize) as libc::c_int,
            );
            return -(1 as libc::c_int);
        }
    }
    offset += 1;
    offset;
    while ':' as i32 != *pattern.offset(offset as isize) as libc::c_int {
        match *pattern.offset(offset as isize) as libc::c_int {
            105 => {
                (*pleaf).set_match_ignorecase(1 as libc::c_int != 0);
            }
            _ => {
                fprintf(
                    stderr,
                    b"Pattern \"%s\" pos %d: Flag \"%c\" invalid.\n\0" as *const u8
                        as *const libc::c_char,
                    pattern,
                    offset,
                    *pattern.offset(offset as isize) as libc::c_int,
                );
                return -(1 as libc::c_int);
            }
        }
        offset += 1;
        offset;
    }
    offset += 1;
    offset;
    (*pleaf).ptnstr = mstrcpy(pattern.offset(offset as isize));
    if !c2_l_postprocess(ps, pleaf) {
        return -(1 as libc::c_int);
    }
    return offset;
}
unsafe extern "C" fn c2_l_postprocess(
    mut ps: *mut session_t,
    mut pleaf: *mut c2_l_t,
) -> bool {
    if C2_L_OEXISTS as libc::c_int == (*pleaf).op() as libc::c_int
        && (*pleaf).ptntype as u64 == 0
    {
        (*pleaf)
            .ptntype = (if C2_L_TSTRING as libc::c_int as libc::c_uint
            == (*pleaf).type_0 as libc::c_uint
        {
            C2_L_PTSTRING as libc::c_int
        } else {
            C2_L_PTINT as libc::c_int
        }) as C2RustUnnamed_2;
    }
    if (*pleaf).predef as u64 == 0 {
        (*pleaf).tgtatom = get_atom(ps, (*pleaf).tgt);
        if (*pleaf).tgtatom == 0 {
            fprintf(
                stderr,
                b"Failed to get atom for target \"%s\".\n\0" as *const u8
                    as *const libc::c_char,
                (*pleaf).tgt,
            );
            return 0 as libc::c_int != 0;
        }
    }
    if (*pleaf).tgtatom != 0 {
        let mut found: bool = 0 as libc::c_int != 0;
        let mut platom: *mut latom_t = (*ps).track_atom_lst;
        while !platom.is_null() {
            if (*pleaf).tgtatom == (*platom).atom {
                found = 1 as libc::c_int != 0;
                break;
            } else {
                platom = (*platom).next;
            }
        }
        if !found {
            let mut pnew: *mut latom_t = malloc(
                ::std::mem::size_of::<latom_t>() as libc::c_ulong,
            ) as *mut latom_t;
            if pnew.is_null() {
                fprintf(
                    stderr,
                    b"%s(): Failed to allocate memory for new track atom.\n\0"
                        as *const u8 as *const libc::c_char,
                    (*::std::mem::transmute::<
                        &[u8; 17],
                        &[libc::c_char; 17],
                    >(b"c2_l_postprocess\0"))
                        .as_ptr(),
                );
                exit(1 as libc::c_int);
            }
            (*pnew).next = (*ps).track_atom_lst;
            (*pnew).atom = (*pleaf).tgtatom;
            (*ps).track_atom_lst = pnew;
        }
    }
    if (*pleaf).predef as u64 != 0 {
        match (*pleaf).predef as libc::c_uint {
            14 => {
                (*ps).o.track_focus = 1 as libc::c_int != 0;
            }
            21 | 22 | 23 | 24 => {
                (*ps).o.track_wdata = 1 as libc::c_int != 0;
            }
            _ => {}
        }
    }
    if (*pleaf).predef as u64 == 0 {
        let mut pc: *const libc::c_char = (*pleaf).tgt;
        while *pc != 0 {
            if *(*__ctype_b_loc()).offset(*pc as libc::c_int as isize) as libc::c_int
                & _ISlower as libc::c_int as libc::c_ushort as libc::c_int != 0
            {
                fprintf(
                    stderr,
                    b"%s(): Warning: Lowercase character in target name \"%s\".\n\0"
                        as *const u8 as *const libc::c_char,
                    (*::std::mem::transmute::<
                        &[u8; 17],
                        &[libc::c_char; 17],
                    >(b"c2_l_postprocess\0"))
                        .as_ptr(),
                    (*pleaf).tgt,
                );
                break;
            } else {
                pc = pc.offset(1);
                pc;
            }
        }
    }
    if C2_L_PTSTRING as libc::c_int as libc::c_uint == (*pleaf).ptntype as libc::c_uint
        && C2_L_MPCRE as libc::c_int == (*pleaf).match_0() as libc::c_int
    {
        fprintf(
            stderr,
            b"PCRE regular expression support not compiled in.\n\0" as *const u8
                as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn c2_free(mut p: c2_ptr_t) {
    if p.isbranch() {
        let pbranch: *mut c2_b_t = p.c2rust_unnamed.b;
        if pbranch.is_null() {
            return;
        }
        c2_free((*pbranch).opr1);
        c2_free((*pbranch).opr2);
        free(pbranch as *mut libc::c_void);
    } else {
        let pleaf: *mut c2_l_t = p.c2rust_unnamed.l;
        if pleaf.is_null() {
            return;
        }
        free((*pleaf).tgt as *mut libc::c_void);
        free((*pleaf).ptnstr as *mut libc::c_void);
        free(pleaf as *mut libc::c_void);
    };
}
pub unsafe extern "C" fn c2_free_lptr(mut lp: *mut c2_lptr_t) -> *mut c2_lptr_t {
    if lp.is_null() {
        return 0 as *mut c2_lptr_t;
    }
    let mut pnext: *mut c2_lptr_t = (*lp).next;
    c2_free((*lp).ptr);
    free(lp as *mut libc::c_void);
    return pnext;
}
unsafe extern "C" fn c2_get_atom_type(mut pleaf: *const c2_l_t) -> Atom {
    match (*pleaf).type_0 as libc::c_uint {
        2 => return 6 as libc::c_int as Atom,
        3 => return 33 as libc::c_int as Atom,
        1 => return 31 as libc::c_int as Atom,
        4 => return 4 as libc::c_int as Atom,
        5 => return 17 as libc::c_int as Atom,
        _ => {}
    }
    return 0 as libc::c_long as Atom;
}
#[inline]
unsafe extern "C" fn c2_match_once_leaf(
    mut ps: *mut session_t,
    mut w: *mut win,
    mut pleaf: *const c2_l_t,
    mut pres: *mut bool,
    mut perr: *mut bool,
) {
    let wid: Window = if (*pleaf).tgt_onframe as libc::c_int != 0 {
        (*w).client_win
    } else {
        (*w).id
    };
    if (*pleaf).predef as u64 == 0 && wid == 0 {
        return;
    }
    let idx: libc::c_int = if (*pleaf).index < 0 as libc::c_int {
        0 as libc::c_int
    } else {
        (*pleaf).index
    };
    match (*pleaf).ptntype as libc::c_uint {
        2 => {
            let mut tgt: libc::c_long = 0 as libc::c_int as libc::c_long;
            if (*pleaf).predef as u64 != 0 {
                *perr = 0 as libc::c_int != 0;
                match (*pleaf).predef as libc::c_uint {
                    1 => {
                        tgt = wid as libc::c_long;
                    }
                    2 => {
                        tgt = (*w).a.x as libc::c_long;
                    }
                    3 => {
                        tgt = (*w).a.y as libc::c_long;
                    }
                    4 => {
                        tgt = ((*w).a.x + (*w).widthb) as libc::c_long;
                    }
                    5 => {
                        tgt = ((*w).a.y + (*w).heightb) as libc::c_long;
                    }
                    6 => {
                        tgt = (*w).a.width as libc::c_long;
                    }
                    7 => {
                        tgt = (*w).a.height as libc::c_long;
                    }
                    8 => {
                        tgt = (*w).widthb as libc::c_long;
                    }
                    9 => {
                        tgt = (*w).heightb as libc::c_long;
                    }
                    10 => {
                        tgt = (*w).a.border_width as libc::c_long;
                    }
                    11 => {
                        tgt = win_is_fullscreen(ps, w) as libc::c_long;
                    }
                    12 => {
                        tgt = (*w).a.override_redirect as libc::c_long;
                    }
                    13 => {
                        tgt = (WMODE_ARGB as libc::c_int as libc::c_uint
                            == (*w).mode as libc::c_uint) as libc::c_int as libc::c_long;
                    }
                    14 => {
                        tgt = win_is_focused_real(ps, w) as libc::c_long;
                    }
                    15 => {
                        tgt = (*w).wmwin as libc::c_long;
                    }
                    16 => {
                        tgt = (*w).bounding_shaped as libc::c_long;
                    }
                    17 => {
                        tgt = (*w).rounded_corners as libc::c_long;
                    }
                    18 => {
                        tgt = (*w).client_win as libc::c_long;
                    }
                    20 => {
                        tgt = (*w).leader as libc::c_long;
                    }
                    _ => {
                        *perr = 1 as libc::c_int != 0;
                    }
                }
            } else {
                let mut prop: winprop_t = wid_get_prop_adv(
                    ps,
                    wid,
                    (*pleaf).tgtatom,
                    idx as libc::c_long,
                    1 as libc::c_long,
                    c2_get_atom_type(pleaf),
                    (*pleaf).format,
                );
                if prop.nitems != 0 {
                    *perr = 0 as libc::c_int != 0;
                    tgt = winprop_get_int(prop);
                }
                free_winprop(&mut prop);
            }
            if *perr {
                return;
            }
            match (*pleaf).op() as libc::c_int {
                0 => {
                    *pres = if (*pleaf).predef as libc::c_uint != 0 {
                        tgt
                    } else {
                        1 as libc::c_int as libc::c_long
                    } != 0;
                }
                1 => {
                    *pres = tgt == (*pleaf).ptnint;
                }
                2 => {
                    *pres = tgt > (*pleaf).ptnint;
                }
                3 => {
                    *pres = tgt >= (*pleaf).ptnint;
                }
                4 => {
                    *pres = tgt < (*pleaf).ptnint;
                }
                5 => {
                    *pres = tgt <= (*pleaf).ptnint;
                }
                _ => {
                    *perr = 1 as libc::c_int != 0;
                }
            }
        }
        1 => {
            let mut tgt_0: *const libc::c_char = 0 as *const libc::c_char;
            let mut tgt_free: *mut libc::c_char = 0 as *mut libc::c_char;
            if (*pleaf).predef as u64 != 0 {
                match (*pleaf).predef as libc::c_uint {
                    19 => {
                        tgt_0 = WINTYPES[(*w).window_type as usize];
                    }
                    21 => {
                        tgt_0 = (*w).name;
                    }
                    22 => {
                        tgt_0 = (*w).class_general;
                    }
                    23 => {
                        tgt_0 = (*w).class_instance;
                    }
                    24 => {
                        tgt_0 = (*w).role;
                    }
                    _ => {}
                }
            } else if C2_L_TATOM as libc::c_int as libc::c_uint
                == (*pleaf).type_0 as libc::c_uint
            {
                let mut prop_0: winprop_t = wid_get_prop_adv(
                    ps,
                    wid,
                    (*pleaf).tgtatom,
                    idx as libc::c_long,
                    1 as libc::c_long,
                    c2_get_atom_type(pleaf),
                    (*pleaf).format,
                );
                let mut atom: Atom = winprop_get_int(prop_0) as Atom;
                if atom != 0 {
                    tgt_free = XGetAtomName((*ps).dpy, atom);
                }
                if !tgt_free.is_null() {
                    tgt_0 = tgt_free;
                }
                free_winprop(&mut prop_0);
            } else {
                let mut strlst: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
                let mut nstr: libc::c_int = 0;
                if wid_get_text_prop(ps, wid, (*pleaf).tgtatom, &mut strlst, &mut nstr)
                    as libc::c_int != 0 && nstr > idx
                {
                    tgt_free = mstrcpy(*strlst.offset(idx as isize));
                    tgt_0 = tgt_free;
                }
                if !strlst.is_null() {
                    XFreeStringList(strlst);
                }
            }
            if !tgt_0.is_null() {
                *perr = 0 as libc::c_int != 0;
            } else {
                return
            }
            match (*pleaf).op() as libc::c_int {
                0 => {
                    *pres = 1 as libc::c_int != 0;
                }
                1 => {
                    match (*pleaf).match_0() as libc::c_int {
                        0 => {
                            if (*pleaf).match_ignorecase() {
                                *pres = strcasecmp(tgt_0, (*pleaf).ptnstr) == 0;
                            } else {
                                *pres = strcmp(tgt_0, (*pleaf).ptnstr) == 0;
                            }
                        }
                        2 => {
                            if (*pleaf).match_ignorecase() {
                                *pres = !(strcasestr(tgt_0, (*pleaf).ptnstr)).is_null();
                            } else {
                                *pres = !(strstr(tgt_0, (*pleaf).ptnstr)).is_null();
                            }
                        }
                        1 => {
                            if (*pleaf).match_ignorecase() {
                                *pres = strncasecmp(
                                    tgt_0,
                                    (*pleaf).ptnstr,
                                    strlen((*pleaf).ptnstr),
                                ) == 0;
                            } else {
                                *pres = strncmp(
                                    tgt_0,
                                    (*pleaf).ptnstr,
                                    strlen((*pleaf).ptnstr),
                                ) == 0;
                            }
                        }
                        3 => {
                            let mut flags: libc::c_int = 0 as libc::c_int;
                            if (*pleaf).match_ignorecase() {
                                flags |= (1 as libc::c_int) << 4 as libc::c_int;
                            }
                            *pres = fnmatch((*pleaf).ptnstr, tgt_0, flags) == 0;
                        }
                        4 | _ => {}
                    }
                }
                _ => {
                    *perr = 1 as libc::c_int != 0;
                }
            }
            if !tgt_free.is_null() {
                if C2_L_TATOM as libc::c_int as libc::c_uint
                    == (*pleaf).type_0 as libc::c_uint
                {
                    cxfree(tgt_free as *mut libc::c_void);
                } else {
                    free(tgt_free as *mut libc::c_void);
                }
            }
        }
        _ => {}
    };
}
unsafe extern "C" fn c2_match_once(
    mut ps: *mut session_t,
    mut w: *mut win,
    cond: c2_ptr_t,
) -> bool {
    let mut result: bool = 0 as libc::c_int != 0;
    let mut error: bool = 1 as libc::c_int != 0;
    if cond.isbranch() {
        let mut pb: *const c2_b_t = cond.c2rust_unnamed.b;
        if pb.is_null() {
            return 0 as libc::c_int != 0;
        }
        error = 0 as libc::c_int != 0;
        match (*pb).op as libc::c_uint {
            1 => {
                result = c2_match_once(ps, w, (*pb).opr1) as libc::c_int != 0
                    && c2_match_once(ps, w, (*pb).opr2) as libc::c_int != 0;
            }
            2 => {
                result = c2_match_once(ps, w, (*pb).opr1) as libc::c_int != 0
                    || c2_match_once(ps, w, (*pb).opr2) as libc::c_int != 0;
            }
            3 => {
                result = c2_match_once(ps, w, (*pb).opr1) as libc::c_int
                    != c2_match_once(ps, w, (*pb).opr2) as libc::c_int;
            }
            _ => {
                error = 1 as libc::c_int != 0;
            }
        }
    } else {
        let mut pleaf: *const c2_l_t = cond.c2rust_unnamed.l;
        if pleaf.is_null() {
            return 0 as libc::c_int != 0;
        }
        c2_match_once_leaf(ps, w, pleaf, &mut result, &mut error);
        if C2_L_OEXISTS as libc::c_int == (*pleaf).op() as libc::c_int
            && error as libc::c_int != 0
        {
            result = 0 as libc::c_int != 0;
            error = 0 as libc::c_int != 0;
        }
    }
    if error {
        result = 0 as libc::c_int != 0;
    }
    if if cond.isbranch() as libc::c_int != 0 {
        (*cond.c2rust_unnamed.b).neg() as libc::c_int
    } else {
        (*cond.c2rust_unnamed.l).neg() as libc::c_int
    } != 0
    {
        result = !result;
    }
    return result;
}
pub unsafe extern "C" fn c2_matchd(
    mut ps: *mut session_t,
    mut w: *mut win,
    mut condlst: *const c2_lptr_t,
    mut cache: *mut *const c2_lptr_t,
    mut pdata: *mut *mut libc::c_void,
) -> bool {
    if !cache.is_null() && !(*cache).is_null()
        && c2_match_once(ps, w, (**cache).ptr) as libc::c_int != 0
    {
        if !pdata.is_null() {
            *pdata = (**cache).data;
        }
        return 1 as libc::c_int != 0;
    }
    while !condlst.is_null() {
        if c2_match_once(ps, w, (*condlst).ptr) {
            if !cache.is_null() {
                *cache = condlst;
            }
            if !pdata.is_null() {
                *pdata = (*condlst).data;
            }
            return 1 as libc::c_int != 0;
        }
        condlst = (*condlst).next;
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn run_static_initializers() {
    C2_PTR_NULL = {
        let mut init = c2_ptr_t {
            isbranch: [0; 1],
            c2rust_padding: [0; 7],
            c2rust_unnamed: C2RustUnnamed_1 {
                l: 0 as *mut c2_l_t,
            },
        };
        init.set_isbranch(0 as libc::c_int != 0);
        init
    };
    leaf_def = {
        let mut init = _c2_l {
            neg_op_match_0_match_ignorecase: [0; 1],
            c2rust_padding: [0; 7],
            tgt: 0 as *mut libc::c_char,
            tgtatom: 0 as libc::c_int as Atom,
            tgt_onframe: 0 as libc::c_int != 0,
            index: -(1 as libc::c_int),
            predef: C2_L_PUNDEFINED,
            type_0: C2_L_TUNDEFINED,
            format: 0 as libc::c_int,
            ptntype: C2_L_PTUNDEFINED,
            ptnstr: 0 as *mut libc::c_char,
            ptnint: 0 as libc::c_int as libc::c_long,
        };
        init.set_neg(0 as libc::c_int != 0);
        init.set_op(C2_L_OEXISTS);
        init.set_match_0(C2_L_MEXACT);
        init.set_match_ignorecase(0 as libc::c_int != 0);
        init
    };
    lptr_def = {
        let mut init = _c2_lptr {
            ptr: {
                let mut init = c2_ptr_t {
                    isbranch: [0; 1],
                    c2rust_padding: [0; 7],
                    c2rust_unnamed: C2RustUnnamed_1 {
                        l: 0 as *const c2_l_t as *mut c2_l_t,
                    },
                };
                init.set_isbranch(0 as libc::c_int != 0);
                init
            },
            data: 0 as *const libc::c_void as *mut libc::c_void,
            next: 0 as *const _c2_lptr as *mut _c2_lptr,
        };
        init
    };
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
