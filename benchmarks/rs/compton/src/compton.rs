use ::libc;
use ::c2rust_asm_casts;
use c2rust_asm_casts::AsmCastTrait;
use std::arch::asm;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type _XGC;
    pub type _XDisplay;
    pub type _XPrivate;
    pub type _XrmHashBucketRec;
    pub type _XRRScreenConfiguration;
    pub type _glx_texture;
    pub type _c2_lptr;
    fn c2_parsed(
        ps: *mut session_t,
        pcondlst: *mut *mut c2_lptr_t,
        pattern: *const libc::c_char,
        data: *mut libc::c_void,
    ) -> *mut c2_lptr_t;
    fn c2_free_lptr(lp: *mut c2_lptr_t) -> *mut c2_lptr_t;
    fn c2_matchd(
        ps: *mut session_t,
        w: *mut win,
        condlst: *const c2_lptr_t,
        cache: *mut *const c2_lptr_t,
        pdata: *mut *mut libc::c_void,
    ) -> bool;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_double;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn select(
        __nfds: libc::c_int,
        __readfds: *mut fd_set,
        __writefds: *mut fd_set,
        __exceptfds: *mut fd_set,
        __timeout: *mut timeval,
    ) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strcasecmp(__s1: *const libc::c_char, __s2: *const libc::c_char) -> libc::c_int;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn gettimeofday(__tv: *mut timeval, __tz: *mut libc::c_void) -> libc::c_int;
    fn XCreateImage(
        _: *mut Display,
        _: *mut Visual,
        _: libc::c_uint,
        _: libc::c_int,
        _: libc::c_int,
        _: *mut libc::c_char,
        _: libc::c_uint,
        _: libc::c_uint,
        _: libc::c_int,
        _: libc::c_int,
    ) -> *mut XImage;
    fn XOpenDisplay(_: *const libc::c_char) -> *mut Display;
    fn XSynchronize(
        _: *mut Display,
        _: libc::c_int,
    ) -> Option::<unsafe extern "C" fn(*mut Display) -> libc::c_int>;
    fn XInternAtom(_: *mut Display, _: *const libc::c_char, _: libc::c_int) -> Atom;
    fn XCreateGC(
        _: *mut Display,
        _: Drawable,
        _: libc::c_ulong,
        _: *mut XGCValues,
    ) -> GC;
    fn XCreatePixmap(
        _: *mut Display,
        _: Drawable,
        _: libc::c_uint,
        _: libc::c_uint,
        _: libc::c_uint,
    ) -> Pixmap;
    fn XCreateSimpleWindow(
        _: *mut Display,
        _: Window,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_uint,
        _: libc::c_uint,
        _: libc::c_uint,
        _: libc::c_ulong,
        _: libc::c_ulong,
    ) -> Window;
    fn XSetErrorHandler(_: XErrorHandler) -> XErrorHandler;
    fn XFreeStringList(_: *mut *mut libc::c_char);
    fn XChangeProperty(
        _: *mut Display,
        _: Window,
        _: Atom,
        _: Atom,
        _: libc::c_int,
        _: libc::c_int,
        _: *const libc::c_uchar,
        _: libc::c_int,
    ) -> libc::c_int;
    fn XClearArea(
        _: *mut Display,
        _: Window,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_uint,
        _: libc::c_uint,
        _: libc::c_int,
    ) -> libc::c_int;
    fn XDeleteProperty(_: *mut Display, _: Window, _: Atom) -> libc::c_int;
    fn XDestroyWindow(_: *mut Display, _: Window) -> libc::c_int;
    fn XEventsQueued(_: *mut Display, _: libc::c_int) -> libc::c_int;
    fn XFlush(_: *mut Display) -> libc::c_int;
    fn XFree(_: *mut libc::c_void) -> libc::c_int;
    fn XFreeGC(_: *mut Display, _: GC) -> libc::c_int;
    fn XFreePixmap(_: *mut Display, _: Pixmap) -> libc::c_int;
    fn XGetErrorText(
        _: *mut Display,
        _: libc::c_int,
        _: *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_int;
    fn XGetGeometry(
        _: *mut Display,
        _: Drawable,
        _: *mut Window,
        _: *mut libc::c_int,
        _: *mut libc::c_int,
        _: *mut libc::c_uint,
        _: *mut libc::c_uint,
        _: *mut libc::c_uint,
        _: *mut libc::c_uint,
    ) -> libc::c_int;
    fn XGetInputFocus(
        _: *mut Display,
        _: *mut Window,
        _: *mut libc::c_int,
    ) -> libc::c_int;
    fn XGetWindowProperty(
        _: *mut Display,
        _: Window,
        _: Atom,
        _: libc::c_long,
        _: libc::c_long,
        _: libc::c_int,
        _: Atom,
        _: *mut Atom,
        _: *mut libc::c_int,
        _: *mut libc::c_ulong,
        _: *mut libc::c_ulong,
        _: *mut *mut libc::c_uchar,
    ) -> libc::c_int;
    fn XGetWindowAttributes(
        _: *mut Display,
        _: Window,
        _: *mut XWindowAttributes,
    ) -> libc::c_int;
    fn XGrabServer(_: *mut Display) -> libc::c_int;
    fn XMapWindow(_: *mut Display, _: Window) -> libc::c_int;
    fn XNextEvent(_: *mut Display, _: *mut XEvent) -> libc::c_int;
    fn XPutImage(
        _: *mut Display,
        _: Drawable,
        _: GC,
        _: *mut XImage,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_uint,
        _: libc::c_uint,
    ) -> libc::c_int;
    fn XQueryExtension(
        _: *mut Display,
        _: *const libc::c_char,
        _: *mut libc::c_int,
        _: *mut libc::c_int,
        _: *mut libc::c_int,
    ) -> libc::c_int;
    fn XQueryTree(
        _: *mut Display,
        _: Window,
        _: *mut Window,
        _: *mut Window,
        _: *mut *mut Window,
        _: *mut libc::c_uint,
    ) -> libc::c_int;
    fn XSelectInput(_: *mut Display, _: Window, _: libc::c_long) -> libc::c_int;
    fn XSetSelectionOwner(_: *mut Display, _: Atom, _: Window, _: Time) -> libc::c_int;
    fn XSync(_: *mut Display, _: libc::c_int) -> libc::c_int;
    fn XUngrabServer(_: *mut Display) -> libc::c_int;
    fn XUnmapWindow(_: *mut Display, _: Window) -> libc::c_int;
    fn XAllocClassHint() -> *mut XClassHint;
    fn XGetTextProperty(
        _: *mut Display,
        _: Window,
        _: *mut XTextProperty,
        _: Atom,
    ) -> libc::c_int;
    fn XGetWMName(_: *mut Display, _: Window, _: *mut XTextProperty) -> libc::c_int;
    fn XSetTextProperty(_: *mut Display, _: Window, _: *mut XTextProperty, _: Atom);
    fn Xutf8SetWMProperties(
        _: *mut Display,
        _: Window,
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
        _: *mut XSizeHints,
        _: *mut XWMHints,
        _: *mut XClassHint,
    );
    fn XmbTextListToTextProperty(
        display: *mut Display,
        list: *mut *mut libc::c_char,
        count: libc::c_int,
        style: XICCEncodingStyle,
        text_prop_return: *mut XTextProperty,
    ) -> libc::c_int;
    fn XmbTextPropertyToTextList(
        display: *mut Display,
        text_prop: *const XTextProperty,
        list_return: *mut *mut *mut libc::c_char,
        count_return: *mut libc::c_int,
    ) -> libc::c_int;
    fn XFixesQueryExtension(
        dpy: *mut Display,
        event_base_return: *mut libc::c_int,
        error_base_return: *mut libc::c_int,
    ) -> libc::c_int;
    fn XFixesCreateRegion(
        dpy: *mut Display,
        rectangles: *mut XRectangle,
        nrectangles: libc::c_int,
    ) -> XserverRegion;
    fn XFixesCreateRegionFromWindow(
        dpy: *mut Display,
        window: Window,
        kind: libc::c_int,
    ) -> XserverRegion;
    fn XFixesDestroyRegion(dpy: *mut Display, region: XserverRegion);
    fn XFixesSetRegion(
        dpy: *mut Display,
        region: XserverRegion,
        rectangles: *mut XRectangle,
        nrectangles: libc::c_int,
    );
    fn XFixesCopyRegion(dpy: *mut Display, dst: XserverRegion, src: XserverRegion);
    fn XFixesUnionRegion(
        dpy: *mut Display,
        dst: XserverRegion,
        src1: XserverRegion,
        src2: XserverRegion,
    );
    fn XFixesIntersectRegion(
        dpy: *mut Display,
        dst: XserverRegion,
        src1: XserverRegion,
        src2: XserverRegion,
    );
    fn XFixesSubtractRegion(
        dpy: *mut Display,
        dst: XserverRegion,
        src1: XserverRegion,
        src2: XserverRegion,
    );
    fn XFixesTranslateRegion(
        dpy: *mut Display,
        region: XserverRegion,
        dx: libc::c_int,
        dy: libc::c_int,
    );
    fn XFixesFetchRegion(
        dpy: *mut Display,
        region: XserverRegion,
        nrectanglesRet: *mut libc::c_int,
    ) -> *mut XRectangle;
    fn XFixesSetWindowShapeRegion(
        dpy: *mut Display,
        win: Window,
        shape_kind: libc::c_int,
        x_off: libc::c_int,
        y_off: libc::c_int,
        region: XserverRegion,
    );
    fn XFixesSetPictureClipRegion(
        dpy: *mut Display,
        picture: XID,
        clip_x_origin: libc::c_int,
        clip_y_origin: libc::c_int,
        region: XserverRegion,
    );
    fn XCompositeQueryVersion(
        dpy: *mut Display,
        major_version_return: *mut libc::c_int,
        minor_version_return: *mut libc::c_int,
    ) -> libc::c_int;
    fn XCompositeRedirectSubwindows(
        dpy: *mut Display,
        window: Window,
        update: libc::c_int,
    );
    fn XCompositeUnredirectWindow(
        dpy: *mut Display,
        window: Window,
        update: libc::c_int,
    );
    fn XCompositeUnredirectSubwindows(
        dpy: *mut Display,
        window: Window,
        update: libc::c_int,
    );
    fn XCompositeNameWindowPixmap(dpy: *mut Display, window: Window) -> Pixmap;
    fn XCompositeGetOverlayWindow(dpy: *mut Display, window: Window) -> Window;
    fn XCompositeReleaseOverlayWindow(dpy: *mut Display, window: Window);
    fn XDamageQueryExtension(
        dpy: *mut Display,
        event_base_return: *mut libc::c_int,
        error_base_return: *mut libc::c_int,
    ) -> libc::c_int;
    fn XDamageCreate(
        dpy: *mut Display,
        drawable: Drawable,
        level: libc::c_int,
    ) -> Damage;
    fn XDamageDestroy(dpy: *mut Display, damage: Damage);
    fn XDamageSubtract(
        dpy: *mut Display,
        damage: Damage,
        repair: XserverRegion,
        parts: XserverRegion,
    );
    fn XRenderQueryExtension(
        dpy: *mut Display,
        event_basep: *mut libc::c_int,
        error_basep: *mut libc::c_int,
    ) -> libc::c_int;
    fn XRenderFindVisualFormat(
        dpy: *mut Display,
        visual: *const Visual,
    ) -> *mut XRenderPictFormat;
    fn XRenderFindStandardFormat(
        dpy: *mut Display,
        format: libc::c_int,
    ) -> *mut XRenderPictFormat;
    fn XRenderCreatePicture(
        dpy: *mut Display,
        drawable: Drawable,
        format: *const XRenderPictFormat,
        valuemask: libc::c_ulong,
        attributes: *const XRenderPictureAttributes,
    ) -> Picture;
    fn XRenderFreePicture(dpy: *mut Display, picture: Picture);
    fn XRenderComposite(
        dpy: *mut Display,
        op: libc::c_int,
        src: Picture,
        mask: Picture,
        dst: Picture,
        src_x: libc::c_int,
        src_y: libc::c_int,
        mask_x: libc::c_int,
        mask_y: libc::c_int,
        dst_x: libc::c_int,
        dst_y: libc::c_int,
        width: libc::c_uint,
        height: libc::c_uint,
    );
    fn XRenderFillRectangle(
        dpy: *mut Display,
        op: libc::c_int,
        dst: Picture,
        color: *const XRenderColor,
        x: libc::c_int,
        y: libc::c_int,
        width: libc::c_uint,
        height: libc::c_uint,
    );
    fn XRenderFillRectangles(
        dpy: *mut Display,
        op: libc::c_int,
        dst: Picture,
        color: *const XRenderColor,
        rectangles: *const XRectangle,
        n_rects: libc::c_int,
    );
    fn XRenderQueryFilters(dpy: *mut Display, drawable: Drawable) -> *mut XFilters;
    fn XRenderSetPictureFilter(
        dpy: *mut Display,
        picture: Picture,
        filter: *const libc::c_char,
        params: *mut XFixed,
        nparams: libc::c_int,
    );
    fn XShapeQueryExtension(
        _: *mut Display,
        _: *mut libc::c_int,
        _: *mut libc::c_int,
    ) -> libc::c_int;
    fn XShapeQueryExtents(
        _: *mut Display,
        _: Window,
        _: *mut libc::c_int,
        _: *mut libc::c_int,
        _: *mut libc::c_int,
        _: *mut libc::c_uint,
        _: *mut libc::c_uint,
        _: *mut libc::c_int,
        _: *mut libc::c_int,
        _: *mut libc::c_int,
        _: *mut libc::c_uint,
        _: *mut libc::c_uint,
    ) -> libc::c_int;
    fn XShapeSelectInput(_: *mut Display, _: Window, _: libc::c_ulong);
    fn XRRQueryExtension(
        dpy: *mut Display,
        event_base_return: *mut libc::c_int,
        error_base_return: *mut libc::c_int,
    ) -> libc::c_int;
    fn XRRGetScreenInfo(
        dpy: *mut Display,
        window: Window,
    ) -> *mut XRRScreenConfiguration;
    fn XRRFreeScreenConfigInfo(config: *mut XRRScreenConfiguration);
    fn XRRConfigCurrentRate(config: *mut XRRScreenConfiguration) -> libc::c_short;
    fn XRRSelectInput(dpy: *mut Display, window: Window, mask: libc::c_int);
    fn XdbeQueryExtension(
        _: *mut Display,
        _: *mut libc::c_int,
        _: *mut libc::c_int,
    ) -> libc::c_int;
    fn XdbeAllocateBackBufferName(
        _: *mut Display,
        _: Window,
        _: XdbeSwapAction,
    ) -> XdbeBackBuffer;
    fn XdbeDeallocateBackBufferName(_: *mut Display, _: XdbeBackBuffer) -> libc::c_int;
    fn XdbeSwapBuffers(
        _: *mut Display,
        _: *mut XdbeSwapInfo,
        _: libc::c_int,
    ) -> libc::c_int;
    fn XSyncQueryExtension(
        _: *mut Display,
        _: *mut libc::c_int,
        _: *mut libc::c_int,
    ) -> libc::c_int;
    fn XSyncInitialize(
        _: *mut Display,
        _: *mut libc::c_int,
        _: *mut libc::c_int,
    ) -> libc::c_int;
    fn XSyncCreateFence(_: *mut Display, _: Drawable, _: libc::c_int) -> XSyncFence;
    fn XSyncTriggerFence(_: *mut Display, _: XSyncFence) -> libc::c_int;
    fn XSyncResetFence(_: *mut Display, _: XSyncFence) -> libc::c_int;
    fn XSyncDestroyFence(_: *mut Display, _: XSyncFence) -> libc::c_int;
    fn XSyncAwaitFence(
        _: *mut Display,
        _: *const XSyncFence,
        _: libc::c_int,
    ) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn freopen(
        __filename: *const libc::c_char,
        __modes: *const libc::c_char,
        __stream: *mut FILE,
    ) -> *mut FILE;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    static mut stderr: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stdin: *mut FILE;
    fn exp(_: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn ceil(_: libc::c_double) -> libc::c_double;
    fn round(_: libc::c_double) -> libc::c_double;
    fn _exit(__status: libc::c_int) -> !;
    fn getpid() -> __pid_t;
    fn getppid() -> __pid_t;
    fn setsid() -> __pid_t;
    fn fork() -> __pid_t;
    static mut optarg: *mut libc::c_char;
    static mut optind: libc::c_int;
    fn getopt_long(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
        __longopts: *const option,
        __longind: *mut libc::c_int,
    ) -> libc::c_int;
    fn setlocale(
        __category: libc::c_int,
        __locale: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn sigemptyset(__set: *mut sigset_t) -> libc::c_int;
    fn sigaction(
        __sig: libc::c_int,
        __act: *const sigaction,
        __oact: *mut sigaction,
    ) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __uint32_t = libc::c_uint;
pub type __uid_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __clock_t = libc::c_long;
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
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
pub type sigset_t = __sigset_t;
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
pub type __sigval_t = sigval;
#[derive(Copy, Clone)]
#[repr(C)]
pub union sigval {
    pub sival_int: libc::c_int,
    pub sival_ptr: *mut libc::c_void,
}
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
pub type Time = libc::c_ulong;
pub type Window = XID;
pub type Drawable = XID;
pub type Font = XID;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XGCValues {
    pub function: libc::c_int,
    pub plane_mask: libc::c_ulong,
    pub foreground: libc::c_ulong,
    pub background: libc::c_ulong,
    pub line_width: libc::c_int,
    pub line_style: libc::c_int,
    pub cap_style: libc::c_int,
    pub join_style: libc::c_int,
    pub fill_style: libc::c_int,
    pub fill_rule: libc::c_int,
    pub arc_mode: libc::c_int,
    pub tile: Pixmap,
    pub stipple: Pixmap,
    pub ts_x_origin: libc::c_int,
    pub ts_y_origin: libc::c_int,
    pub font: Font,
    pub subwindow_mode: libc::c_int,
    pub graphics_exposures: libc::c_int,
    pub clip_x_origin: libc::c_int,
    pub clip_y_origin: libc::c_int,
    pub clip_mask: Pixmap,
    pub dash_offset: libc::c_int,
    pub dashes: libc::c_char,
}
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
pub struct ScreenFormat {
    pub ext_data: *mut XExtData,
    pub depth: libc::c_int,
    pub bits_per_pixel: libc::c_int,
    pub scanline_pad: libc::c_int,
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
pub struct _XImage {
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub xoffset: libc::c_int,
    pub format: libc::c_int,
    pub data: *mut libc::c_char,
    pub byte_order: libc::c_int,
    pub bitmap_unit: libc::c_int,
    pub bitmap_bit_order: libc::c_int,
    pub bitmap_pad: libc::c_int,
    pub depth: libc::c_int,
    pub bytes_per_line: libc::c_int,
    pub bits_per_pixel: libc::c_int,
    pub red_mask: libc::c_ulong,
    pub green_mask: libc::c_ulong,
    pub blue_mask: libc::c_ulong,
    pub obdata: XPointer,
    pub f: funcs,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct funcs {
    pub create_image: Option::<
        unsafe extern "C" fn(
            *mut _XDisplay,
            *mut Visual,
            libc::c_uint,
            libc::c_int,
            libc::c_int,
            *mut libc::c_char,
            libc::c_uint,
            libc::c_uint,
            libc::c_int,
            libc::c_int,
        ) -> *mut _XImage,
    >,
    pub destroy_image: Option::<unsafe extern "C" fn(*mut _XImage) -> libc::c_int>,
    pub get_pixel: Option::<
        unsafe extern "C" fn(*mut _XImage, libc::c_int, libc::c_int) -> libc::c_ulong,
    >,
    pub put_pixel: Option::<
        unsafe extern "C" fn(
            *mut _XImage,
            libc::c_int,
            libc::c_int,
            libc::c_ulong,
        ) -> libc::c_int,
    >,
    pub sub_image: Option::<
        unsafe extern "C" fn(
            *mut _XImage,
            libc::c_int,
            libc::c_int,
            libc::c_uint,
            libc::c_uint,
        ) -> *mut _XImage,
    >,
    pub add_pixel: Option::<
        unsafe extern "C" fn(*mut _XImage, libc::c_long) -> libc::c_int,
    >,
}
pub type XImage = _XImage;
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
pub struct C2RustUnnamed_0 {
    pub ext_data: *mut XExtData,
    pub private1: *mut _XPrivate,
    pub fd: libc::c_int,
    pub private2: libc::c_int,
    pub proto_major_version: libc::c_int,
    pub proto_minor_version: libc::c_int,
    pub vendor: *mut libc::c_char,
    pub private3: XID,
    pub private4: XID,
    pub private5: XID,
    pub private6: libc::c_int,
    pub resource_alloc: Option::<unsafe extern "C" fn(*mut _XDisplay) -> XID>,
    pub byte_order: libc::c_int,
    pub bitmap_unit: libc::c_int,
    pub bitmap_pad: libc::c_int,
    pub bitmap_bit_order: libc::c_int,
    pub nformats: libc::c_int,
    pub pixmap_format: *mut ScreenFormat,
    pub private8: libc::c_int,
    pub release: libc::c_int,
    pub private9: *mut _XPrivate,
    pub private10: *mut _XPrivate,
    pub qlen: libc::c_int,
    pub last_request_read: libc::c_ulong,
    pub request: libc::c_ulong,
    pub private11: XPointer,
    pub private12: XPointer,
    pub private13: XPointer,
    pub private14: XPointer,
    pub max_request_size: libc::c_uint,
    pub db: *mut _XrmHashBucketRec,
    pub private15: Option::<unsafe extern "C" fn(*mut _XDisplay) -> libc::c_int>,
    pub display_name: *mut libc::c_char,
    pub default_screen: libc::c_int,
    pub nscreens: libc::c_int,
    pub screens: *mut Screen,
    pub motion_buffer: libc::c_ulong,
    pub private16: libc::c_ulong,
    pub min_keycode: libc::c_int,
    pub max_keycode: libc::c_int,
    pub private17: XPointer,
    pub private18: XPointer,
    pub private19: libc::c_int,
    pub xdefaults: *mut libc::c_char,
}
pub type _XPrivDisplay = *mut C2RustUnnamed_0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XKeyEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub root: Window,
    pub subwindow: Window,
    pub time: Time,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub x_root: libc::c_int,
    pub y_root: libc::c_int,
    pub state: libc::c_uint,
    pub keycode: libc::c_uint,
    pub same_screen: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XButtonEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub root: Window,
    pub subwindow: Window,
    pub time: Time,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub x_root: libc::c_int,
    pub y_root: libc::c_int,
    pub state: libc::c_uint,
    pub button: libc::c_uint,
    pub same_screen: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XMotionEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub root: Window,
    pub subwindow: Window,
    pub time: Time,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub x_root: libc::c_int,
    pub y_root: libc::c_int,
    pub state: libc::c_uint,
    pub is_hint: libc::c_char,
    pub same_screen: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XCrossingEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub root: Window,
    pub subwindow: Window,
    pub time: Time,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub x_root: libc::c_int,
    pub y_root: libc::c_int,
    pub mode: libc::c_int,
    pub detail: libc::c_int,
    pub same_screen: libc::c_int,
    pub focus: libc::c_int,
    pub state: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XFocusChangeEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub mode: libc::c_int,
    pub detail: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XKeymapEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub key_vector: [libc::c_char; 32],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XExposeEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub count: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XGraphicsExposeEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub drawable: Drawable,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub count: libc::c_int,
    pub major_code: libc::c_int,
    pub minor_code: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XNoExposeEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub drawable: Drawable,
    pub major_code: libc::c_int,
    pub minor_code: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XVisibilityEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub state: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XCreateWindowEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub parent: Window,
    pub window: Window,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub border_width: libc::c_int,
    pub override_redirect: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XDestroyWindowEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XUnmapEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub from_configure: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XMapEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub override_redirect: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XMapRequestEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub parent: Window,
    pub window: Window,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XReparentEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub parent: Window,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub override_redirect: libc::c_int,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XGravityEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub x: libc::c_int,
    pub y: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XResizeRequestEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub width: libc::c_int,
    pub height: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XConfigureRequestEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub parent: Window,
    pub window: Window,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub border_width: libc::c_int,
    pub above: Window,
    pub detail: libc::c_int,
    pub value_mask: libc::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XCirculateEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub place: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XCirculateRequestEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub parent: Window,
    pub window: Window,
    pub place: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XPropertyEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub atom: Atom,
    pub time: Time,
    pub state: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XSelectionClearEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub selection: Atom,
    pub time: Time,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XSelectionRequestEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub owner: Window,
    pub requestor: Window,
    pub selection: Atom,
    pub target: Atom,
    pub property: Atom,
    pub time: Time,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XSelectionEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub requestor: Window,
    pub selection: Atom,
    pub target: Atom,
    pub property: Atom,
    pub time: Time,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XColormapEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub colormap: Colormap,
    pub new: libc::c_int,
    pub state: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XClientMessageEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub message_type: Atom,
    pub format: libc::c_int,
    pub data: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub b: [libc::c_char; 20],
    pub s: [libc::c_short; 10],
    pub l: [libc::c_long; 5],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XMappingEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub request: libc::c_int,
    pub first_keycode: libc::c_int,
    pub count: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XErrorEvent {
    pub type_0: libc::c_int,
    pub display: *mut Display,
    pub resourceid: XID,
    pub serial: libc::c_ulong,
    pub error_code: libc::c_uchar,
    pub request_code: libc::c_uchar,
    pub minor_code: libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XAnyEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XGenericEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub extension: libc::c_int,
    pub evtype: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XGenericEventCookie {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub extension: libc::c_int,
    pub evtype: libc::c_int,
    pub cookie: libc::c_uint,
    pub data: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union _XEvent {
    pub type_0: libc::c_int,
    pub xany: XAnyEvent,
    pub xkey: XKeyEvent,
    pub xbutton: XButtonEvent,
    pub xmotion: XMotionEvent,
    pub xcrossing: XCrossingEvent,
    pub xfocus: XFocusChangeEvent,
    pub xexpose: XExposeEvent,
    pub xgraphicsexpose: XGraphicsExposeEvent,
    pub xnoexpose: XNoExposeEvent,
    pub xvisibility: XVisibilityEvent,
    pub xcreatewindow: XCreateWindowEvent,
    pub xdestroywindow: XDestroyWindowEvent,
    pub xunmap: XUnmapEvent,
    pub xmap: XMapEvent,
    pub xmaprequest: XMapRequestEvent,
    pub xreparent: XReparentEvent,
    pub xconfigure: XConfigureEvent,
    pub xgravity: XGravityEvent,
    pub xresizerequest: XResizeRequestEvent,
    pub xconfigurerequest: XConfigureRequestEvent,
    pub xcirculate: XCirculateEvent,
    pub xcirculaterequest: XCirculateRequestEvent,
    pub xproperty: XPropertyEvent,
    pub xselectionclear: XSelectionClearEvent,
    pub xselectionrequest: XSelectionRequestEvent,
    pub xselection: XSelectionEvent,
    pub xcolormap: XColormapEvent,
    pub xclient: XClientMessageEvent,
    pub xmapping: XMappingEvent,
    pub xerror: XErrorEvent,
    pub xkeymap: XKeymapEvent,
    pub xgeneric: XGenericEvent,
    pub xcookie: XGenericEventCookie,
    pub pad: [libc::c_long; 24],
}
pub type XEvent = _XEvent;
pub type XErrorHandler = Option::<
    unsafe extern "C" fn(*mut Display, *mut XErrorEvent) -> libc::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XSizeHints {
    pub flags: libc::c_long,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub min_width: libc::c_int,
    pub min_height: libc::c_int,
    pub max_width: libc::c_int,
    pub max_height: libc::c_int,
    pub width_inc: libc::c_int,
    pub height_inc: libc::c_int,
    pub min_aspect: C2RustUnnamed_2,
    pub max_aspect: C2RustUnnamed_2,
    pub base_width: libc::c_int,
    pub base_height: libc::c_int,
    pub win_gravity: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub x: libc::c_int,
    pub y: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XWMHints {
    pub flags: libc::c_long,
    pub input: libc::c_int,
    pub initial_state: libc::c_int,
    pub icon_pixmap: Pixmap,
    pub icon_window: Window,
    pub icon_x: libc::c_int,
    pub icon_y: libc::c_int,
    pub icon_mask: Pixmap,
    pub window_group: XID,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XTextProperty {
    pub value: *mut libc::c_uchar,
    pub encoding: Atom,
    pub format: libc::c_int,
    pub nitems: libc::c_ulong,
}
pub type XICCEncodingStyle = libc::c_uint;
pub const XUTF8StringStyle: XICCEncodingStyle = 4;
pub const XStdICCTextStyle: XICCEncodingStyle = 3;
pub const XTextStyle: XICCEncodingStyle = 2;
pub const XCompoundTextStyle: XICCEncodingStyle = 1;
pub const XStringStyle: XICCEncodingStyle = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XClassHint {
    pub res_name: *mut libc::c_char,
    pub res_class: *mut libc::c_char,
}
pub type XserverRegion = XID;
pub type Damage = XID;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XDamageNotifyEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub drawable: Drawable,
    pub damage: Damage,
    pub level: libc::c_int,
    pub more: libc::c_int,
    pub timestamp: Time,
    pub area: XRectangle,
    pub geometry: XRectangle,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _XRenderPictureAttributes {
    pub repeat: libc::c_int,
    pub alpha_map: Picture,
    pub alpha_x_origin: libc::c_int,
    pub alpha_y_origin: libc::c_int,
    pub clip_x_origin: libc::c_int,
    pub clip_y_origin: libc::c_int,
    pub clip_mask: Pixmap,
    pub graphics_exposures: libc::c_int,
    pub subwindow_mode: libc::c_int,
    pub poly_edge: libc::c_int,
    pub poly_mode: libc::c_int,
    pub dither: Atom,
    pub component_alpha: libc::c_int,
}
pub type XRenderPictureAttributes = _XRenderPictureAttributes;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XRenderColor {
    pub red: libc::c_ushort,
    pub green: libc::c_ushort,
    pub blue: libc::c_ushort,
    pub alpha: libc::c_ushort,
}
pub type XDouble = libc::c_double;
pub type XFixed = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _XFilters {
    pub nfilter: libc::c_int,
    pub filter: *mut *mut libc::c_char,
    pub nalias: libc::c_int,
    pub alias: *mut libc::c_short,
}
pub type XFilters = _XFilters;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XShapeEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub kind: libc::c_int,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub width: libc::c_uint,
    pub height: libc::c_uint,
    pub time: Time,
    pub shaped: libc::c_int,
}
pub type Rotation = libc::c_ushort;
pub type SizeID = libc::c_ushort;
pub type SubpixelOrder = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XRRScreenChangeNotifyEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub window: Window,
    pub root: Window,
    pub timestamp: Time,
    pub config_timestamp: Time,
    pub size_index: SizeID,
    pub subpixel_order: SubpixelOrder,
    pub rotation: Rotation,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub mwidth: libc::c_int,
    pub mheight: libc::c_int,
}
pub type XRRScreenConfiguration = _XRRScreenConfiguration;
pub type XdbeBackBuffer = Drawable;
pub type XdbeSwapAction = libc::c_uchar;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XdbeSwapInfo {
    pub swap_window: Window,
    pub swap_action: XdbeSwapAction,
}
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
    pub data: C2RustUnnamed_3,
    pub nitems: libc::c_ulong,
    pub type_0: Atom,
    pub format: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
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
pub struct reg_data_t {
    pub rects: *mut XRectangle,
    pub nrects: libc::c_int,
}
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
pub struct options_tmp {
    pub no_dock_shadow: bool,
    pub no_dnd_shadow: bool,
    pub menu_opacity: libc::c_double,
}
pub type timeout_t = _timeout_t;
pub type win_evmode_t = libc::c_uint;
pub const WIN_EVMODE_CLIENT: win_evmode_t = 2;
pub const WIN_EVMODE_FRAME: win_evmode_t = 1;
pub const WIN_EVMODE_UNKNOWN: win_evmode_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const libc::c_char,
    pub has_arg: libc::c_int,
    pub flag: *mut libc::c_int,
    pub val: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct siginfo_t {
    pub si_signo: libc::c_int,
    pub si_errno: libc::c_int,
    pub si_code: libc::c_int,
    pub __pad0: libc::c_int,
    pub _sifields: C2RustUnnamed_4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
    pub _pad: [libc::c_int; 28],
    pub _kill: C2RustUnnamed_13,
    pub _timer: C2RustUnnamed_12,
    pub _rt: C2RustUnnamed_11,
    pub _sigchld: C2RustUnnamed_10,
    pub _sigfault: C2RustUnnamed_7,
    pub _sigpoll: C2RustUnnamed_6,
    pub _sigsys: C2RustUnnamed_5,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub _call_addr: *mut libc::c_void,
    pub _syscall: libc::c_int,
    pub _arch: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub si_band: libc::c_long,
    pub si_fd: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub si_addr: *mut libc::c_void,
    pub si_addr_lsb: libc::c_short,
    pub _bounds: C2RustUnnamed_8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_8 {
    pub _addr_bnd: C2RustUnnamed_9,
    pub _pkey: __uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub _lower: *mut libc::c_void,
    pub _upper: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_status: libc::c_int,
    pub si_utime: __clock_t,
    pub si_stime: __clock_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_11 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_12 {
    pub si_tid: libc::c_int,
    pub si_overrun: libc::c_int,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_13 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
}
pub type __sighandler_t = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sigaction {
    pub __sigaction_handler: C2RustUnnamed_14,
    pub sa_mask: __sigset_t,
    pub sa_flags: libc::c_int,
    pub sa_restorer: Option::<unsafe extern "C" fn() -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_14 {
    pub sa_handler: __sighandler_t,
    pub sa_sigaction: Option::<
        unsafe extern "C" fn(libc::c_int, *mut siginfo_t, *mut libc::c_void) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_15 {
    pub name: *const libc::c_char,
    pub kern_str: *const libc::c_char,
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
unsafe extern "C" fn timeval_isempty(mut ptv: *mut timeval) -> bool {
    if ptv.is_null() {
        return 0 as libc::c_int != 0;
    }
    return (*ptv).tv_sec <= 0 as libc::c_int as libc::c_long
        && (*ptv).tv_usec <= 0 as libc::c_int as libc::c_long;
}
#[inline]
unsafe extern "C" fn timeval_ms_cmp(
    mut ptv: *mut timeval,
    mut ms: time_ms_t,
) -> libc::c_int {
    let mut sec: libc::c_long = ms / 1000 as libc::c_int as libc::c_long;
    if (*ptv).tv_sec > sec {
        return 1 as libc::c_int;
    }
    if (*ptv).tv_sec < sec {
        return -(1 as libc::c_int);
    }
    let mut usec: libc::c_long = ms % 1000 as libc::c_int as libc::c_long
        * (1000000 as libc::c_long / 1000 as libc::c_int as libc::c_long);
    if (*ptv).tv_usec > usec {
        return 1 as libc::c_int;
    }
    if (*ptv).tv_usec < usec {
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn timeval_subtract(
    mut result: *mut timeval,
    mut x: *mut timeval,
    mut y: *mut timeval,
) -> libc::c_int {
    if (*x).tv_usec < (*y).tv_usec {
        let mut nsec: libc::c_long = ((*y).tv_usec - (*x).tv_usec)
            / 1000000 as libc::c_int as libc::c_long + 1 as libc::c_int as libc::c_long;
        (*y).tv_usec -= 1000000 as libc::c_int as libc::c_long * nsec;
        (*y).tv_sec += nsec;
    }
    if (*x).tv_usec - (*y).tv_usec > 1000000 as libc::c_int as libc::c_long {
        let mut nsec_0: libc::c_long = ((*x).tv_usec - (*y).tv_usec)
            / 1000000 as libc::c_int as libc::c_long;
        (*y).tv_usec += 1000000 as libc::c_int as libc::c_long * nsec_0;
        (*y).tv_sec -= nsec_0;
    }
    (*result).tv_sec = (*x).tv_sec - (*y).tv_sec;
    (*result).tv_usec = (*x).tv_usec - (*y).tv_usec;
    return ((*x).tv_sec < (*y).tv_sec) as libc::c_int;
}
#[inline]
unsafe extern "C" fn get_time_timeval() -> timeval {
    let mut tv: timeval = {
        let mut init = timeval {
            tv_sec: 0 as libc::c_int as __time_t,
            tv_usec: 0 as libc::c_int as __suseconds_t,
        };
        init
    };
    gettimeofday(&mut tv, 0 as *mut libc::c_void);
    return tv;
}
#[inline]
unsafe extern "C" fn print_timestamp(mut ps: *mut session_t) {
    let mut tm: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut diff: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    if gettimeofday(&mut tm, 0 as *mut libc::c_void) != 0 {
        return;
    }
    timeval_subtract(&mut diff, &mut tm, &mut (*ps).time_start);
    printf(
        b"[ %5ld.%02ld ] \0" as *const u8 as *const libc::c_char,
        diff.tv_sec,
        diff.tv_usec / 10000 as libc::c_int as libc::c_long,
    );
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
unsafe extern "C" fn normalize_i_range(
    mut i: libc::c_int,
    mut min: libc::c_int,
    mut max: libc::c_int,
) -> libc::c_int {
    if i > max {
        return max;
    }
    if i < min {
        return min;
    }
    return i;
}
#[inline]
unsafe extern "C" fn max_i(mut a: libc::c_int, mut b: libc::c_int) -> libc::c_int {
    return if a > b { a } else { b };
}
#[inline]
unsafe extern "C" fn min_i(mut a: libc::c_int, mut b: libc::c_int) -> libc::c_int {
    return if a > b { b } else { a };
}
#[inline]
unsafe extern "C" fn max_l(mut a: libc::c_long, mut b: libc::c_long) -> libc::c_long {
    return if a > b { a } else { b };
}
#[inline]
unsafe extern "C" fn normalize_d_range(
    mut d: libc::c_double,
    mut min: libc::c_double,
    mut max: libc::c_double,
) -> libc::c_double {
    if d > max {
        return max;
    }
    if d < min {
        return min;
    }
    return d;
}
#[inline]
unsafe extern "C" fn normalize_d(mut d: libc::c_double) -> libc::c_double {
    return normalize_d_range(d, 0.0f64, 1.0f64);
}
#[inline]
unsafe extern "C" fn parse_vsync(
    mut ps: *mut session_t,
    mut str: *const libc::c_char,
) -> bool {
    let mut i: vsync_t = VSYNC_NONE;
    while !(VSYNC_STRS[i as usize]).is_null() {
        if strcasecmp(str, VSYNC_STRS[i as usize]) == 0 {
            (*ps).o.vsync = i;
            return 1 as libc::c_int != 0;
        }
        i += 1;
        i;
    }
    fprintf(
        stderr,
        b"%s(\"%s\"): Invalid vsync argument.\n\0" as *const u8 as *const libc::c_char,
        (*::std::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"parse_vsync\0"))
            .as_ptr(),
        str,
    );
    return 0 as libc::c_int != 0;
}
#[inline]
unsafe extern "C" fn parse_backend(
    mut ps: *mut session_t,
    mut str: *const libc::c_char,
) -> bool {
    let mut i: backend = BKEND_XRENDER;
    while !(BACKEND_STRS[i as usize]).is_null() {
        if strcasecmp(str, BACKEND_STRS[i as usize]) == 0 {
            (*ps).o.backend = i;
            return 1 as libc::c_int != 0;
        }
        i += 1;
        i;
    }
    if strcasecmp(str, b"xr_glx_hybird\0" as *const u8 as *const libc::c_char) == 0 {
        (*ps).o.backend = BKEND_XR_GLX_HYBRID;
        return 1 as libc::c_int != 0;
    }
    if strcasecmp(str, b"xr-glx-hybrid\0" as *const u8 as *const libc::c_char) == 0 {
        (*ps).o.backend = BKEND_XR_GLX_HYBRID;
        return 1 as libc::c_int != 0;
    }
    fprintf(
        stderr,
        b"%s(\"%s\"): Invalid backend argument.\n\0" as *const u8 as *const libc::c_char,
        (*::std::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"parse_backend\0"))
            .as_ptr(),
        str,
    );
    return 0 as libc::c_int != 0;
}
#[inline]
unsafe extern "C" fn parse_glx_swap_method(
    mut ps: *mut session_t,
    mut str: *const libc::c_char,
) -> bool {
    if strcmp(b"undefined\0" as *const u8 as *const libc::c_char, str) == 0 {
        (*ps).o.glx_swap_method = 0 as libc::c_int;
        return 1 as libc::c_int != 0;
    }
    if strcmp(b"copy\0" as *const u8 as *const libc::c_char, str) == 0 {
        (*ps).o.glx_swap_method = 1 as libc::c_int;
        return 1 as libc::c_int != 0;
    }
    if strcmp(b"exchange\0" as *const u8 as *const libc::c_char, str) == 0 {
        (*ps).o.glx_swap_method = 2 as libc::c_int;
        return 1 as libc::c_int != 0;
    }
    if strcmp(b"buffer-age\0" as *const u8 as *const libc::c_char, str) == 0 {
        (*ps).o.glx_swap_method = -(1 as libc::c_int);
        return 1 as libc::c_int != 0;
    }
    let mut pc: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut age: libc::c_int = strtol(str, &mut pc, 0 as libc::c_int) as libc::c_int;
    if pc.is_null() || str == pc as *const libc::c_char {
        fprintf(
            stderr,
            b"%s(\"%s\"): Invalid number.\n\0" as *const u8 as *const libc::c_char,
            (*::std::mem::transmute::<
                &[u8; 22],
                &[libc::c_char; 22],
            >(b"parse_glx_swap_method\0"))
                .as_ptr(),
            str,
        );
        return 0 as libc::c_int != 0;
    }
    while *pc != 0 {
        if *(*__ctype_b_loc()).offset(*pc as libc::c_int as isize) as libc::c_int
            & _ISspace as libc::c_int as libc::c_ushort as libc::c_int == 0
        {
            fprintf(
                stderr,
                b"%s(\"%s\"): Trailing characters.\n\0" as *const u8
                    as *const libc::c_char,
                (*::std::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"parse_glx_swap_method\0"))
                    .as_ptr(),
                str,
            );
            return 0 as libc::c_int != 0;
        }
        pc = pc.offset(1);
        pc;
    }
    if age > 5 as libc::c_int + 1 as libc::c_int || age < -(1 as libc::c_int) {
        fprintf(
            stderr,
            b"%s(\"%s\"): Number too large / too small.\n\0" as *const u8
                as *const libc::c_char,
            (*::std::mem::transmute::<
                &[u8; 22],
                &[libc::c_char; 22],
            >(b"parse_glx_swap_method\0"))
                .as_ptr(),
            str,
        );
        return 0 as libc::c_int != 0;
    }
    (*ps).o.glx_swap_method = age;
    return 1 as libc::c_int != 0;
}
#[inline]
unsafe extern "C" fn fds_insert_select(
    mut ppfds: *mut *mut fd_set,
    mut fd: libc::c_int,
) -> bool {
    if (*ppfds).is_null() {
        *ppfds = malloc(::std::mem::size_of::<fd_set>() as libc::c_ulong) as *mut fd_set;
        if !(*ppfds).is_null() {
            let mut __d0: libc::c_int = 0;
            let mut __d1: libc::c_int = 0;
            let fresh0 = &mut __d0;
            let fresh1;
            let fresh2 = (::std::mem::size_of::<fd_set>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<__fd_mask>() as libc::c_ulong);
            let fresh3 = &mut __d1;
            let fresh4;
            let fresh5 = &mut *((**ppfds).fds_bits)
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize) as *mut __fd_mask;
            asm!(
                "cld; rep; stosq", inlateout("cx")
                c2rust_asm_casts::AsmCast::cast_in(fresh0, fresh2) => fresh1,
                inlateout("di") c2rust_asm_casts::AsmCast::cast_in(fresh3, fresh5) =>
                fresh4, inlateout("ax") 0 as libc::c_int => _, options(preserves_flags,
                att_syntax)
            );
            c2rust_asm_casts::AsmCast::cast_out(fresh0, fresh2, fresh1);
            c2rust_asm_casts::AsmCast::cast_out(fresh3, fresh5, fresh4);
        } else {
            fprintf(
                stderr,
                b"Failed to allocate memory for select() fdset.\n\0" as *const u8
                    as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
    }
    (**ppfds)
        .fds_bits[(fd
        / (8 as libc::c_int
            * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
        as usize]
        |= ((1 as libc::c_ulong)
            << fd
                % (8 as libc::c_int
                    * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                        as libc::c_int)) as __fd_mask;
    return 1 as libc::c_int != 0;
}
#[inline]
unsafe extern "C" fn fds_insert(
    mut ps: *mut session_t,
    mut fd: libc::c_int,
    mut events: libc::c_short,
) -> bool {
    let mut result: bool = 1 as libc::c_int != 0;
    (*ps).nfds_max = max_i(fd + 1 as libc::c_int, (*ps).nfds_max);
    if 0x1 as libc::c_int & events as libc::c_int != 0 {
        result = fds_insert_select(&mut (*ps).pfds_read, fd) as libc::c_int != 0
            && result as libc::c_int != 0;
    }
    if 0x4 as libc::c_int & events as libc::c_int != 0 {
        result = fds_insert_select(&mut (*ps).pfds_write, fd) as libc::c_int != 0
            && result as libc::c_int != 0;
    }
    if 0x2 as libc::c_int & events as libc::c_int != 0 {
        result = fds_insert_select(&mut (*ps).pfds_except, fd) as libc::c_int != 0
            && result as libc::c_int != 0;
    }
    return result;
}
#[inline]
unsafe extern "C" fn fds_poll(
    mut ps: *mut session_t,
    mut ptv: *mut timeval,
) -> libc::c_int {
    let mut pfds_read: *mut fd_set = 0 as *mut fd_set;
    if !((*ps).pfds_read).is_null() {
        pfds_read = malloc(::std::mem::size_of::<fd_set>() as libc::c_ulong)
            as *mut fd_set;
        memcpy(
            pfds_read as *mut libc::c_void,
            (*ps).pfds_read as *const libc::c_void,
            ::std::mem::size_of::<fd_set>() as libc::c_ulong,
        );
        if pfds_read.is_null() {
            fprintf(
                stderr,
                b"Failed to allocate memory for copying select() fdset.\n\0" as *const u8
                    as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
    }
    let mut pfds_write: *mut fd_set = 0 as *mut fd_set;
    if !((*ps).pfds_write).is_null() {
        pfds_write = malloc(::std::mem::size_of::<fd_set>() as libc::c_ulong)
            as *mut fd_set;
        memcpy(
            pfds_write as *mut libc::c_void,
            (*ps).pfds_write as *const libc::c_void,
            ::std::mem::size_of::<fd_set>() as libc::c_ulong,
        );
        if pfds_write.is_null() {
            fprintf(
                stderr,
                b"Failed to allocate memory for copying select() fdset.\n\0" as *const u8
                    as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
    }
    let mut pfds_except: *mut fd_set = 0 as *mut fd_set;
    if !((*ps).pfds_except).is_null() {
        pfds_except = malloc(::std::mem::size_of::<fd_set>() as libc::c_ulong)
            as *mut fd_set;
        memcpy(
            pfds_except as *mut libc::c_void,
            (*ps).pfds_except as *const libc::c_void,
            ::std::mem::size_of::<fd_set>() as libc::c_ulong,
        );
        if pfds_except.is_null() {
            fprintf(
                stderr,
                b"Failed to allocate memory for copying select() fdset.\n\0" as *const u8
                    as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
    }
    let mut ret: libc::c_int = select(
        (*ps).nfds_max,
        pfds_read,
        pfds_write,
        pfds_except,
        ptv,
    );
    free(pfds_read as *mut libc::c_void);
    free(pfds_write as *mut libc::c_void);
    free(pfds_except as *mut libc::c_void);
    return ret;
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
unsafe extern "C" fn get_tgt_window(mut ps: *mut session_t) -> Window {
    return if (*ps).o.paint_on_overlay as libc::c_int != 0 {
        (*ps).overlay
    } else {
        (*ps).root
    };
}
#[inline]
unsafe extern "C" fn find_win(mut ps: *mut session_t, mut id: Window) -> *mut win {
    if id == 0 {
        return 0 as *mut win;
    }
    let mut w: *mut win = 0 as *mut win;
    w = (*ps).list;
    while !w.is_null() {
        if (*w).id == id && !(*w).destroyed {
            return w;
        }
        w = (*w).next;
    }
    return 0 as *mut win;
}
#[inline]
unsafe extern "C" fn find_toplevel(mut ps: *mut session_t, mut id: Window) -> *mut win {
    if id == 0 {
        return 0 as *mut win;
    }
    let mut w: *mut win = (*ps).list;
    while !w.is_null() {
        if (*w).client_win == id && !(*w).destroyed {
            return w;
        }
        w = (*w).next;
    }
    return 0 as *mut win;
}
#[inline]
unsafe extern "C" fn bkend_use_xrender(mut ps: *mut session_t) -> bool {
    return BKEND_XRENDER as libc::c_int as libc::c_uint
        == (*ps).o.backend as libc::c_uint
        || BKEND_XR_GLX_HYBRID as libc::c_int as libc::c_uint
            == (*ps).o.backend as libc::c_uint;
}
#[inline]
unsafe extern "C" fn bkend_use_glx(mut ps: *mut session_t) -> bool {
    return BKEND_GLX as libc::c_int as libc::c_uint == (*ps).o.backend as libc::c_uint
        || BKEND_XR_GLX_HYBRID as libc::c_int as libc::c_uint
            == (*ps).o.backend as libc::c_uint;
}
#[inline]
unsafe extern "C" fn win_is_focused_real(
    mut ps: *mut session_t,
    mut w: *const win,
) -> bool {
    return 2 as libc::c_int == (*w).a.map_state && (*ps).active_win == w as *mut _win;
}
#[inline]
unsafe extern "C" fn copy_region(
    mut ps: *const session_t,
    mut oldregion: XserverRegion,
) -> XserverRegion {
    if oldregion == 0 {
        return 0 as libc::c_long as XserverRegion;
    }
    let mut region: XserverRegion = XFixesCreateRegion(
        (*ps).dpy,
        0 as *mut XRectangle,
        0 as libc::c_int,
    );
    XFixesCopyRegion((*ps).dpy, region, oldregion);
    return region;
}
#[inline]
unsafe extern "C" fn free_region(mut ps: *mut session_t, mut p: *mut XserverRegion) {
    if *p != 0 {
        XFixesDestroyRegion((*ps).dpy, *p);
        *p = 0 as libc::c_long as XserverRegion;
    }
}
#[inline]
unsafe extern "C" fn free_all_damage_last(mut ps: *mut session_t) {
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 5 as libc::c_int {
        free_region(ps, &mut *((*ps).all_damage_last).as_mut_ptr().offset(i as isize));
        i += 1;
        i;
    }
}
#[inline]
unsafe extern "C" fn free_fence(mut ps: *mut session_t, mut pfence: *mut XSyncFence) {
    if *pfence != 0 {
        XSyncDestroyFence((*ps).dpy, *pfence);
    }
    *pfence = 0 as libc::c_long as XSyncFence;
}
#[inline]
unsafe extern "C" fn rect_crop(
    mut pdst: *mut XRectangle,
    mut psrc: *const XRectangle,
    mut pbound: *const XRectangle,
) {
    (*pdst)
        .x = max_i((*psrc).x as libc::c_int, (*pbound).x as libc::c_int)
        as libc::c_short;
    (*pdst)
        .y = max_i((*psrc).y as libc::c_int, (*pbound).y as libc::c_int)
        as libc::c_short;
    (*pdst)
        .width = max_i(
        0 as libc::c_int,
        min_i(
            (*psrc).x as libc::c_int + (*psrc).width as libc::c_int,
            (*pbound).x as libc::c_int + (*pbound).width as libc::c_int,
        ) - (*pdst).x as libc::c_int,
    ) as libc::c_ushort;
    (*pdst)
        .height = max_i(
        0 as libc::c_int,
        min_i(
            (*psrc).y as libc::c_int + (*psrc).height as libc::c_int,
            (*pbound).y as libc::c_int + (*pbound).height as libc::c_int,
        ) - (*pdst).y as libc::c_int,
    ) as libc::c_ushort;
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
unsafe extern "C" fn win_is_solid(mut ps: *mut session_t, mut w: *const win) -> bool {
    return WMODE_SOLID as libc::c_int as libc::c_uint == (*w).mode as libc::c_uint
        && !(*ps).o.force_win_blend;
}
#[inline]
unsafe extern "C" fn wid_has_prop(
    mut ps: *const session_t,
    mut w: Window,
    mut atom: Atom,
) -> bool {
    let mut type_0: Atom = 0 as libc::c_long as Atom;
    let mut format: libc::c_int = 0;
    let mut nitems: libc::c_ulong = 0;
    let mut after: libc::c_ulong = 0;
    let mut data: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    if 0 as libc::c_int
        == XGetWindowProperty(
            (*ps).dpy,
            w,
            atom,
            0 as libc::c_int as libc::c_long,
            0 as libc::c_int as libc::c_long,
            0 as libc::c_int,
            0 as libc::c_long as Atom,
            &mut type_0,
            &mut format,
            &mut nitems,
            &mut after,
            &mut data,
        )
    {
        cxfree(data as *mut libc::c_void);
        if type_0 != 0 {
            return 1 as libc::c_int != 0;
        }
    }
    return 0 as libc::c_int != 0;
}
#[inline]
unsafe extern "C" fn wid_get_prop(
    mut ps: *const session_t,
    mut wid: Window,
    mut atom: Atom,
    mut length: libc::c_long,
    mut rtype: Atom,
    mut rformat: libc::c_int,
) -> winprop_t {
    return wid_get_prop_adv(ps, wid, atom, 0 as libc::c_long, length, rtype, rformat);
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
unsafe extern "C" fn free_texture(
    mut ps: *mut session_t,
    mut pptex: *mut *mut glx_texture_t,
) {
    let mut ptex: *mut glx_texture_t = *pptex;
    if ptex.is_null() {
        return;
    }
}
#[inline]
unsafe extern "C" fn free_paint_glx(mut ps: *mut session_t, mut ppaint: *mut paint_t) {
    free_texture(ps, &mut (*ppaint).ptex);
}
#[inline]
unsafe extern "C" fn free_win_res_glx(mut ps: *mut session_t, mut w: *mut win) {
    free_paint_glx(ps, &mut (*w).paint);
    free_paint_glx(ps, &mut (*w).shadow_paint);
}
#[inline]
unsafe extern "C" fn glx_mark_(
    mut ps: *mut session_t,
    mut func: *const libc::c_char,
    mut xid: XID,
    mut start: bool,
) {}
#[inline]
unsafe extern "C" fn glx_mark_frame(mut ps: *mut session_t) {}
#[inline]
unsafe extern "C" fn xr_sync_(
    mut ps: *mut session_t,
    mut d: Drawable,
    mut pfence: *mut XSyncFence,
) {
    if !(*ps).o.xrender_sync {
        return;
    }
    XSync((*ps).dpy, 0 as libc::c_int);
    if (*ps).o.xrender_sync_fence as libc::c_int != 0
        && (*ps).xsync_exists as libc::c_int != 0
    {
        let mut tmp_fence: XSyncFence = 0 as libc::c_long as XSyncFence;
        if pfence.is_null() {
            pfence = &mut tmp_fence;
        }
        if *pfence == 0 {
            *pfence = XSyncCreateFence((*ps).dpy, d, 0 as libc::c_int);
        }
        if *pfence != 0 {
            let mut triggered: libc::c_int = 0 as libc::c_int;
            XSyncTriggerFence((*ps).dpy, *pfence);
            XSyncAwaitFence((*ps).dpy, pfence, 1 as libc::c_int);
        } else {
            fprintf(
                stderr,
                b"%s(%#010lx): Failed to create X Sync fence.\n\0" as *const u8
                    as *const libc::c_char,
                (*::std::mem::transmute::<&[u8; 9], &[libc::c_char; 9]>(b"xr_sync_\0"))
                    .as_ptr(),
                d,
            );
        }
        free_fence(ps, &mut tmp_fence);
        if *pfence != 0 {
            XSyncResetFence((*ps).dpy, *pfence);
        }
    }
}
#[inline]
unsafe extern "C" fn atof(mut __nptr: *const libc::c_char) -> libc::c_double {
    return strtod(__nptr, 0 as *mut libc::c_void as *mut *mut libc::c_char);
}
#[inline]
unsafe extern "C" fn get_time_ms() -> time_ms_t {
    let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    gettimeofday(&mut tv, 0 as *mut libc::c_void);
    return tv.tv_sec
        % (15 as libc::c_long * 24 as libc::c_long * 60 as libc::c_long
            * 60 as libc::c_long) * 1000 as libc::c_int as libc::c_long
        + tv.tv_usec / 1000 as libc::c_int as libc::c_long;
}
#[inline]
unsafe extern "C" fn set_ignore_next(mut ps: *mut session_t) {
    set_ignore(
        ps,
        ((*((*ps).dpy as _XPrivDisplay)).request)
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
    );
}
#[inline]
unsafe extern "C" fn xrfilter_reset(mut ps: *mut session_t, mut p: Picture) {
    XRenderSetPictureFilter(
        (*ps).dpy,
        p,
        b"Nearest\0" as *const u8 as *const libc::c_char,
        0 as *mut XFixed,
        0 as libc::c_int,
    );
}
#[inline]
unsafe extern "C" fn wintype_arr_enable(mut arr: *mut bool) {
    let mut i: wintype_t = WINTYPE_UNKNOWN;
    i = WINTYPE_UNKNOWN;
    while (i as libc::c_uint) < NUM_WINTYPES as libc::c_int as libc::c_uint {
        *arr.offset(i as isize) = 1 as libc::c_int != 0;
        i += 1;
        i;
    }
}
#[inline]
unsafe extern "C" fn geom_to_rect(
    mut ps: *mut session_t,
    mut src: *const geometry_t,
    mut def: *const XRectangle,
) -> XRectangle {
    let mut rect_def: XRectangle = {
        let mut init = XRectangle {
            x: 0 as libc::c_int as libc::c_short,
            y: 0 as libc::c_int as libc::c_short,
            width: (*ps).root_width as libc::c_ushort,
            height: (*ps).root_height as libc::c_ushort,
        };
        init
    };
    if def.is_null() {
        def = &mut rect_def;
    }
    let mut rect: XRectangle = {
        let mut init = XRectangle {
            x: (*src).x as libc::c_short,
            y: (*src).y as libc::c_short,
            width: (*src).wid as libc::c_ushort,
            height: (*src).hei as libc::c_ushort,
        };
        init
    };
    if (*src).wid < 0 as libc::c_int {
        rect.width = (*def).width;
    }
    if (*src).hei < 0 as libc::c_int {
        rect.height = (*def).height;
    }
    if -(1 as libc::c_int) == (*src).x {
        rect.x = (*def).x;
    } else if (*src).x < 0 as libc::c_int {
        rect
            .x = ((*ps).root_width + rect.x as libc::c_int + 2 as libc::c_int
            - rect.width as libc::c_int) as libc::c_short;
    }
    if -(1 as libc::c_int) == (*src).y {
        rect.y = (*def).y;
    } else if (*src).y < 0 as libc::c_int {
        rect
            .y = ((*ps).root_height + rect.y as libc::c_int + 2 as libc::c_int
            - rect.height as libc::c_int) as libc::c_short;
    }
    return rect;
}
#[inline]
unsafe extern "C" fn rect_to_reg(
    mut ps: *mut session_t,
    mut src: *const XRectangle,
) -> XserverRegion {
    if src.is_null() {
        return 0 as libc::c_long as XserverRegion;
    }
    let mut bound: XRectangle = {
        let mut init = XRectangle {
            x: 0 as libc::c_int as libc::c_short,
            y: 0 as libc::c_int as libc::c_short,
            width: (*ps).root_width as libc::c_ushort,
            height: (*ps).root_height as libc::c_ushort,
        };
        init
    };
    let mut res: XRectangle = {
        let mut init = XRectangle {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        };
        init
    };
    rect_crop(&mut res, src, &mut bound);
    if res.width as libc::c_int != 0 && res.height as libc::c_int != 0 {
        return XFixesCreateRegion((*ps).dpy, &mut res, 1 as libc::c_int);
    }
    return 0 as libc::c_long as XserverRegion;
}
#[inline]
unsafe extern "C" fn free_picture(mut ps: *mut session_t, mut p: *mut Picture) {
    if *p != 0 {
        XRenderFreePicture((*ps).dpy, *p);
        *p = 0 as libc::c_long as Picture;
    }
}
#[inline]
unsafe extern "C" fn free_pixmap(mut ps: *mut session_t, mut p: *mut Pixmap) {
    if *p != 0 {
        XFreePixmap((*ps).dpy, *p);
        *p = 0 as libc::c_long as Pixmap;
    }
}
#[inline]
unsafe extern "C" fn free_damage(mut ps: *mut session_t, mut p: *mut Damage) {
    if *p != 0 {
        set_ignore_next(ps);
        XDamageDestroy((*ps).dpy, *p);
        *p = 0 as libc::c_long as Damage;
    }
}
#[inline]
unsafe extern "C" fn free_wincondlst(mut pcondlst: *mut *mut c2_lptr_t) {
    loop {
        *pcondlst = c2_free_lptr(*pcondlst);
        if (*pcondlst).is_null() {
            break;
        }
    };
}
#[inline]
unsafe extern "C" fn free_xinerama_info(mut ps: *mut session_t) {}
#[inline]
unsafe extern "C" fn paint_isvalid(
    mut ps: *mut session_t,
    mut ppaint: *const paint_t,
) -> bool {
    if ppaint.is_null() {
        return 0 as libc::c_int != 0;
    }
    if bkend_use_xrender(ps) as libc::c_int != 0 && (*ppaint).pict == 0 {
        return 0 as libc::c_int != 0;
    }
    return 1 as libc::c_int != 0;
}
#[inline]
unsafe extern "C" fn paint_bind_tex_real(
    mut ps: *mut session_t,
    mut ppaint: *mut paint_t,
    mut wid: libc::c_uint,
    mut hei: libc::c_uint,
    mut depth: libc::c_uint,
    mut force: bool,
) -> bool {
    return 1 as libc::c_int != 0;
}
#[inline]
unsafe extern "C" fn paint_bind_tex(
    mut ps: *mut session_t,
    mut ppaint: *mut paint_t,
    mut wid: libc::c_uint,
    mut hei: libc::c_uint,
    mut depth: libc::c_uint,
    mut force: bool,
) -> bool {
    if BKEND_GLX as libc::c_int as libc::c_uint == (*ps).o.backend as libc::c_uint {
        return paint_bind_tex_real(ps, ppaint, wid, hei, depth, force);
    }
    return 1 as libc::c_int != 0;
}
#[inline]
unsafe extern "C" fn free_reg_data(mut pregd: *mut reg_data_t) {
    cxfree((*pregd).rects as *mut libc::c_void);
    (*pregd).rects = 0 as *mut XRectangle;
    (*pregd).nrects = 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn free_paint(mut ps: *mut session_t, mut ppaint: *mut paint_t) {
    free_paint_glx(ps, ppaint);
    free_picture(ps, &mut (*ppaint).pict);
    free_pixmap(ps, &mut (*ppaint).pixmap);
}
#[inline]
unsafe extern "C" fn free_wpaint(mut ps: *mut session_t, mut w: *mut win) {
    free_paint(ps, &mut (*w).paint);
    free_fence(ps, &mut (*w).fence);
}
#[inline]
unsafe extern "C" fn free_win_res(mut ps: *mut session_t, mut w: *mut win) {
    free_win_res_glx(ps, w);
    free_region(ps, &mut (*w).extents);
    free_paint(ps, &mut (*w).paint);
    free_region(ps, &mut (*w).border_size);
    free_paint(ps, &mut (*w).shadow_paint);
    free_damage(ps, &mut (*w).damage);
    free_region(ps, &mut (*w).reg_ignore);
    free((*w).name as *mut libc::c_void);
    free((*w).class_instance as *mut libc::c_void);
    free((*w).class_general as *mut libc::c_void);
    free((*w).role as *mut libc::c_void);
}
#[inline]
unsafe extern "C" fn free_root_tile(mut ps: *mut session_t) {
    free_picture(ps, &mut (*ps).root_tile_paint.pict);
    free_texture(ps, &mut (*ps).root_tile_paint.ptex);
    if (*ps).root_tile_fill {
        free_pixmap(ps, &mut (*ps).root_tile_paint.pixmap);
    }
    (*ps).root_tile_paint.pixmap = 0 as libc::c_long as Pixmap;
    (*ps).root_tile_fill = 0 as libc::c_int != 0;
}
#[inline]
unsafe extern "C" fn ms_to_tv(mut timeout: libc::c_int) -> timeval {
    return {
        let mut init = timeval {
            tv_sec: (timeout / 1000 as libc::c_int) as __time_t,
            tv_usec: (timeout % 1000 as libc::c_int) as libc::c_long
                * (1000000 as libc::c_long / 1000 as libc::c_int as libc::c_long),
        };
        init
    };
}
#[inline]
unsafe extern "C" fn isdamagenotify(
    mut ps: *mut session_t,
    mut ev: *const XEvent,
) -> bool {
    return (*ps).damage_event + 0 as libc::c_int == (*ev).type_0;
}
#[inline]
unsafe extern "C" fn make_text_prop(
    mut ps: *mut session_t,
    mut str: *mut libc::c_char,
) -> *mut XTextProperty {
    let mut pprop: *mut XTextProperty = allocchk_(
        (*::std::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"make_text_prop\0"))
            .as_ptr(),
        calloc(
            1 as libc::c_int as libc::c_ulong,
            ::std::mem::size_of::<XTextProperty>() as libc::c_ulong,
        ),
    ) as *mut XTextProperty;
    if XmbTextListToTextProperty(
        (*ps).dpy,
        &mut str,
        1 as libc::c_int,
        XStringStyle,
        pprop,
    ) != 0
    {
        cxfree((*pprop).value as *mut libc::c_void);
        free(pprop as *mut libc::c_void);
        pprop = 0 as *mut XTextProperty;
    }
    return pprop;
}
#[inline]
unsafe extern "C" fn wid_set_text_prop(
    mut ps: *mut session_t,
    mut wid: Window,
    mut prop_atom: Atom,
    mut str: *mut libc::c_char,
) -> bool {
    let mut pprop: *mut XTextProperty = make_text_prop(ps, str);
    if pprop.is_null() {
        fprintf(
            stderr,
            b"%s(\"%s\"): Failed to make text property.\n\0" as *const u8
                as *const libc::c_char,
            (*::std::mem::transmute::<
                &[u8; 18],
                &[libc::c_char; 18],
            >(b"wid_set_text_prop\0"))
                .as_ptr(),
            str,
        );
        return 0 as libc::c_int != 0;
    }
    XSetTextProperty((*ps).dpy, wid, pprop, prop_atom);
    cxfree((*pprop).value as *mut libc::c_void);
    cxfree(pprop as *mut libc::c_void);
    return 1 as libc::c_int != 0;
}
#[inline]
unsafe extern "C" fn check_fade_fin(mut ps: *mut session_t, mut w: *mut win) {
    if ((*w).fade_callback).is_some() && (*w).opacity == (*w).opacity_tgt {
        set_fade_callback(ps, w, None, 1 as libc::c_int != 0);
    }
}
#[inline]
unsafe extern "C" fn win_ev_stop(mut ps: *mut session_t, mut w: *mut win) {
    set_ignore_next(ps);
    XSelectInput((*ps).dpy, (*w).id, 0 as libc::c_int as libc::c_long);
    if (*w).client_win != 0 {
        set_ignore_next(ps);
        XSelectInput((*ps).dpy, (*w).client_win, 0 as libc::c_int as libc::c_long);
    }
    if (*ps).shape_exists {
        set_ignore_next(ps);
        XShapeSelectInput((*ps).dpy, (*w).id, 0 as libc::c_int as libc::c_ulong);
    }
}
#[inline]
unsafe extern "C" fn wid_get_children(
    mut ps: *mut session_t,
    mut w: Window,
    mut children: *mut *mut Window,
    mut nchildren: *mut libc::c_uint,
) -> bool {
    let mut troot: Window = 0;
    let mut tparent: Window = 0;
    if XQueryTree((*ps).dpy, w, &mut troot, &mut tparent, children, nchildren) == 0 {
        *nchildren = 0 as libc::c_int as libc::c_uint;
        return 0 as libc::c_int != 0;
    }
    return 1 as libc::c_int != 0;
}
#[inline]
unsafe extern "C" fn wid_bounding_shaped(
    mut ps: *const session_t,
    mut wid: Window,
) -> bool {
    if (*ps).shape_exists {
        let mut bounding_shaped: libc::c_int = 0 as libc::c_int;
        let mut clip_shaped: libc::c_int = 0 as libc::c_int;
        let mut x_bounding: libc::c_int = 0;
        let mut y_bounding: libc::c_int = 0;
        let mut x_clip: libc::c_int = 0;
        let mut y_clip: libc::c_int = 0;
        let mut w_bounding: libc::c_uint = 0;
        let mut h_bounding: libc::c_uint = 0;
        let mut w_clip: libc::c_uint = 0;
        let mut h_clip: libc::c_uint = 0;
        XShapeQueryExtents(
            (*ps).dpy,
            wid,
            &mut bounding_shaped,
            &mut x_bounding,
            &mut y_bounding,
            &mut w_bounding,
            &mut h_bounding,
            &mut clip_shaped,
            &mut x_clip,
            &mut y_clip,
            &mut w_clip,
            &mut h_clip,
        );
        return bounding_shaped != 0;
    }
    return 0 as libc::c_int != 0;
}
#[inline]
unsafe extern "C" fn update_reg_ignore_expire(
    mut ps: *mut session_t,
    mut w: *const win,
) {
    if (*w).to_paint as libc::c_int != 0
        && WMODE_SOLID as libc::c_int as libc::c_uint == (*w).mode as libc::c_uint
    {
        (*ps).reg_ignore_expire = 1 as libc::c_int != 0;
    }
}
#[inline]
unsafe extern "C" fn win_has_frame(mut w: *const win) -> bool {
    return (*w).a.border_width != 0 || (*w).frame_extents.top != 0
        || (*w).frame_extents.left != 0 || (*w).frame_extents.right != 0
        || (*w).frame_extents.bottom != 0;
}
#[inline]
unsafe extern "C" fn win_calc_frame_extents(
    mut ps: *mut session_t,
    mut w: *const win,
) -> margin_t {
    let mut result: margin_t = (*w).frame_extents;
    result.top = max_i(result.top, (*w).a.border_width);
    result.left = max_i(result.left, (*w).a.border_width);
    result.bottom = max_i(result.bottom, (*w).a.border_width);
    result.right = max_i(result.right, (*w).a.border_width);
    return result;
}
#[inline]
unsafe extern "C" fn wid_set_opacity_prop(
    mut ps: *mut session_t,
    mut wid: Window,
    mut val: opacity_t,
) {
    let v: libc::c_ulong = val as libc::c_ulong;
    XChangeProperty(
        (*ps).dpy,
        wid,
        (*ps).atom_opacity,
        6 as libc::c_int as Atom,
        32 as libc::c_int,
        0 as libc::c_int,
        &v as *const libc::c_ulong as *mut libc::c_uchar,
        1 as libc::c_int,
    );
}
#[inline]
unsafe extern "C" fn wid_rm_opacity_prop(mut ps: *mut session_t, mut wid: Window) {
    XDeleteProperty((*ps).dpy, wid, (*ps).atom_opacity);
}
#[inline]
unsafe extern "C" fn normalize_conv_kern(
    mut wid: libc::c_int,
    mut hei: libc::c_int,
    mut kern: *mut XFixed,
) {
    let mut sum: libc::c_double = 0.0f64;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < wid * hei {
        sum
            += *kern.offset(i as isize) as XDouble
                / 65536 as libc::c_int as libc::c_double;
        i += 1;
        i;
    }
    let mut factor: libc::c_double = 1.0f64 / sum;
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < wid * hei {
        *kern
            .offset(
                i_0 as isize,
            ) = (*kern.offset(i_0 as isize) as XDouble
            / 65536 as libc::c_int as libc::c_double * factor
            * 65536 as libc::c_int as libc::c_double) as XFixed;
        i_0 += 1;
        i_0;
    }
}
#[inline]
unsafe extern "C" fn set_tgt_clip(
    mut ps: *mut session_t,
    mut reg: XserverRegion,
    mut pcache_reg: *const reg_data_t,
) {
    match (*ps).o.backend as libc::c_uint {
        0 | 2 => {
            XFixesSetPictureClipRegion(
                (*ps).dpy,
                (*ps).tgt_buffer.pict,
                0 as libc::c_int,
                0 as libc::c_int,
                reg,
            );
        }
        _ => {}
    };
}
#[inline]
unsafe extern "C" fn win_update_opacity_prop(mut ps: *mut session_t, mut w: *mut win) {
    (*w).opacity_prop = wid_get_opacity_prop(ps, (*w).id, 0xffffffff as libc::c_uint);
    if !(*ps).o.detect_client_opacity || (*w).client_win == 0
        || (*w).id == (*w).client_win
    {
        (*w).opacity_prop_client = 0xffffffff as libc::c_uint;
    } else {
        (*w)
            .opacity_prop_client = wid_get_opacity_prop(
            ps,
            (*w).client_win,
            0xffffffff as libc::c_uint,
        );
    };
}
#[inline]
unsafe extern "C" fn validate_pixmap(mut ps: *mut session_t, mut pxmap: Pixmap) -> bool {
    if pxmap == 0 {
        return 0 as libc::c_int != 0;
    }
    let mut rroot: Window = 0 as libc::c_long as Window;
    let mut rx: libc::c_int = 0 as libc::c_int;
    let mut ry: libc::c_int = 0 as libc::c_int;
    let mut rwid: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut rhei: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut rborder: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut rdepth: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    return XGetGeometry(
        (*ps).dpy,
        pxmap,
        &mut rroot,
        &mut rx,
        &mut ry,
        &mut rwid,
        &mut rhei,
        &mut rborder,
        &mut rdepth,
    ) != 0 && rwid != 0 && rhei != 0;
}
#[inline]
unsafe extern "C" fn win_validate_pixmap(mut ps: *mut session_t, mut w: *mut win) {
    if !validate_pixmap(ps, (*w).paint.pixmap) {
        free_paint(ps, &mut (*w).paint);
    }
}
#[inline]
unsafe extern "C" fn win_match(
    mut ps: *mut session_t,
    mut w: *mut win,
    mut condlst: *mut c2_lptr_t,
    mut cache: *mut *const c2_lptr_t,
) -> bool {
    return c2_matchd(ps, w, condlst, cache, 0 as *mut *mut libc::c_void);
}
unsafe extern "C" fn clear_cache_win_leaders(mut ps: *mut session_t) {
    let mut w: *mut win = (*ps).list;
    while !w.is_null() {
        (*w).cache_leader = 0 as libc::c_long as Window;
        w = (*w).next;
    }
}
#[inline]
unsafe extern "C" fn find_win_all(mut ps: *mut session_t, wid: Window) -> *mut win {
    if wid == 0 || 1 as libc::c_long as libc::c_ulong == wid || wid == (*ps).root
        || wid == (*ps).overlay
    {
        return 0 as *mut win;
    }
    let mut w: *mut win = find_win(ps, wid);
    if w.is_null() {
        w = find_toplevel(ps, wid);
    }
    if w.is_null() {
        w = find_toplevel2(ps, wid);
    }
    return w;
}
#[inline]
unsafe extern "C" fn win_get_leader(mut ps: *mut session_t, mut w: *mut win) -> Window {
    return win_get_leader_raw(ps, w, 0 as libc::c_int);
}
#[inline]
unsafe extern "C" fn group_is_focused(
    mut ps: *mut session_t,
    mut leader: Window,
) -> bool {
    if leader == 0 {
        return 0 as libc::c_int != 0;
    }
    let mut w: *mut win = (*ps).list;
    while !w.is_null() {
        if win_get_leader(ps, w) == leader && !(*w).destroyed
            && win_is_focused_real(ps, w) as libc::c_int != 0
        {
            return 1 as libc::c_int != 0;
        }
        w = (*w).next;
    }
    return 0 as libc::c_int != 0;
}
#[inline]
unsafe extern "C" fn group_update_focused(mut ps: *mut session_t, mut leader: Window) {
    if leader == 0 {
        return;
    }
    let mut w: *mut win = (*ps).list;
    while !w.is_null() {
        if win_get_leader(ps, w) == leader && !(*w).destroyed {
            win_update_focused(ps, w);
        }
        w = (*w).next;
    }
}
#[inline]
unsafe extern "C" fn win_render(
    mut ps: *mut session_t,
    mut w: *mut win,
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut wid: libc::c_int,
    mut hei: libc::c_int,
    mut opacity: libc::c_double,
    mut reg_paint: XserverRegion,
    mut pcache_reg: *const reg_data_t,
    mut pict: Picture,
) {
    let dx: libc::c_int = (if !w.is_null() { (*w).a.x } else { 0 as libc::c_int }) + x;
    let dy: libc::c_int = (if !w.is_null() { (*w).a.y } else { 0 as libc::c_int }) + y;
    let argb: bool = !w.is_null()
        && (WMODE_ARGB as libc::c_int as libc::c_uint == (*w).mode as libc::c_uint
            || (*ps).o.force_win_blend as libc::c_int != 0);
    let neg: bool = !w.is_null() && (*w).invert_color as libc::c_int != 0;
    render_(
        ps,
        x,
        y,
        dx,
        dy,
        wid,
        hei,
        opacity,
        argb,
        neg,
        pict,
        if !w.is_null() { (*w).paint.ptex } else { (*ps).root_tile_paint.ptex },
        reg_paint,
        pcache_reg,
    );
}
#[inline]
unsafe extern "C" fn win_get_role(
    mut ps: *mut session_t,
    mut w: *mut win,
) -> libc::c_int {
    let mut ret: libc::c_int = win_get_prop_str(
        ps,
        w,
        &mut (*w).role,
        Some(
            wid_get_role
                as unsafe extern "C" fn(
                    *mut session_t,
                    Window,
                    *mut *mut libc::c_char,
                ) -> bool,
        ),
    );
    return ret;
}
#[inline]
unsafe extern "C" fn get_screen_region(mut ps: *mut session_t) -> XserverRegion {
    let mut r: XRectangle = XRectangle {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    };
    r.x = 0 as libc::c_int as libc::c_short;
    r.y = 0 as libc::c_int as libc::c_short;
    r.width = (*ps).root_width as libc::c_ushort;
    r.height = (*ps).root_height as libc::c_ushort;
    return XFixesCreateRegion((*ps).dpy, &mut r, 1 as libc::c_int);
}
#[inline]
unsafe extern "C" fn is_region_empty(
    mut ps: *const session_t,
    mut region: XserverRegion,
    mut pcache_reg: *mut reg_data_t,
) -> bool {
    let mut nrects: libc::c_int = 0 as libc::c_int;
    let mut rects: *mut XRectangle = XFixesFetchRegion((*ps).dpy, region, &mut nrects);
    if !pcache_reg.is_null() {
        (*pcache_reg).rects = rects;
        (*pcache_reg).nrects = nrects;
    } else {
        cxfree(rects as *mut libc::c_void);
    }
    return nrects == 0;
}
#[inline]
unsafe extern "C" fn resize_region(
    mut ps: *mut session_t,
    mut region: XserverRegion,
    mut mod_0: libc::c_short,
) {
    if mod_0 == 0 || region == 0 {
        return;
    }
    let mut nrects: libc::c_int = 0 as libc::c_int;
    let mut nnewrects: libc::c_int = 0 as libc::c_int;
    let mut newrects: *mut XRectangle = 0 as *mut XRectangle;
    let mut rects: *mut XRectangle = XFixesFetchRegion((*ps).dpy, region, &mut nrects);
    if !(rects.is_null() || nrects == 0) {
        newrects = calloc(
            nrects as libc::c_ulong,
            ::std::mem::size_of::<XRectangle>() as libc::c_ulong,
        ) as *mut XRectangle;
        if newrects.is_null() {
            fprintf(
                stderr,
                b"%s(): Failed to allocate memory.\n\0" as *const u8
                    as *const libc::c_char,
                (*::std::mem::transmute::<
                    &[u8; 14],
                    &[libc::c_char; 14],
                >(b"resize_region\0"))
                    .as_ptr(),
            );
            exit(1 as libc::c_int);
        }
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < nrects {
            let mut x1: libc::c_int = max_i(
                (*rects.offset(i as isize)).x as libc::c_int - mod_0 as libc::c_int,
                0 as libc::c_int,
            );
            let mut y1: libc::c_int = max_i(
                (*rects.offset(i as isize)).y as libc::c_int - mod_0 as libc::c_int,
                0 as libc::c_int,
            );
            let mut x2: libc::c_int = min_i(
                (*rects.offset(i as isize)).x as libc::c_int
                    + (*rects.offset(i as isize)).width as libc::c_int
                    + mod_0 as libc::c_int,
                (*ps).root_width,
            );
            let mut y2: libc::c_int = min_i(
                (*rects.offset(i as isize)).y as libc::c_int
                    + (*rects.offset(i as isize)).height as libc::c_int
                    + mod_0 as libc::c_int,
                (*ps).root_height,
            );
            let mut wid: libc::c_int = x2 - x1;
            let mut hei: libc::c_int = y2 - y1;
            if !(wid <= 0 as libc::c_int || hei <= 0 as libc::c_int) {
                (*newrects.offset(nnewrects as isize)).x = x1 as libc::c_short;
                (*newrects.offset(nnewrects as isize)).y = y1 as libc::c_short;
                (*newrects.offset(nnewrects as isize)).width = wid as libc::c_ushort;
                (*newrects.offset(nnewrects as isize)).height = hei as libc::c_ushort;
                nnewrects += 1;
                nnewrects;
            }
            i += 1;
            i;
        }
        XFixesSetRegion((*ps).dpy, region, newrects, nnewrects);
    }
    cxfree(rects as *mut libc::c_void);
    free(newrects as *mut libc::c_void);
}
#[inline]
unsafe extern "C" fn win_get_name(
    mut ps: *mut session_t,
    mut w: *mut win,
) -> libc::c_int {
    let mut ret: libc::c_int = win_get_prop_str(
        ps,
        w,
        &mut (*w).name,
        Some(
            wid_get_name
                as unsafe extern "C" fn(
                    *mut session_t,
                    Window,
                    *mut *mut libc::c_char,
                ) -> bool,
        ),
    );
    return ret;
}
#[inline]
unsafe extern "C" fn add_damage_win(mut ps: *mut session_t, mut w: *mut win) {
    if (*w).extents != 0 {
        add_damage(ps, copy_region(ps, (*w).extents));
    }
}
#[inline]
unsafe extern "C" fn timeout_get_newrun(mut ptmout: *const timeout_t) -> time_ms_t {
    return (*ptmout).firstrun
        + (max_l(
            ((*ptmout).lastrun
                + ((*ptmout).interval as libc::c_double * 0.05f64) as time_ms_t
                - (*ptmout).firstrun) / (*ptmout).interval,
            ((*ptmout).lastrun
                + ((*ptmout).interval as libc::c_double
                    * (1 as libc::c_int as libc::c_double - 0.05f64)) as time_ms_t
                - (*ptmout).firstrun) / (*ptmout).interval,
        ) + 1 as libc::c_int as libc::c_long) * (*ptmout).interval;
}
#[inline]
unsafe extern "C" fn cxinerama_win_upd_scr(mut ps: *mut session_t, mut w: *mut win) {}
pub static mut WINTYPES: [*const libc::c_char; 15] = [
    b"unknown\0" as *const u8 as *const libc::c_char,
    b"desktop\0" as *const u8 as *const libc::c_char,
    b"dock\0" as *const u8 as *const libc::c_char,
    b"toolbar\0" as *const u8 as *const libc::c_char,
    b"menu\0" as *const u8 as *const libc::c_char,
    b"utility\0" as *const u8 as *const libc::c_char,
    b"splash\0" as *const u8 as *const libc::c_char,
    b"dialog\0" as *const u8 as *const libc::c_char,
    b"normal\0" as *const u8 as *const libc::c_char,
    b"dropdown_menu\0" as *const u8 as *const libc::c_char,
    b"popup_menu\0" as *const u8 as *const libc::c_char,
    b"tooltip\0" as *const u8 as *const libc::c_char,
    b"notify\0" as *const u8 as *const libc::c_char,
    b"combo\0" as *const u8 as *const libc::c_char,
    b"dnd\0" as *const u8 as *const libc::c_char,
];
pub static mut VSYNC_STRS: [*const libc::c_char; 7] = [
    b"none\0" as *const u8 as *const libc::c_char,
    b"drm\0" as *const u8 as *const libc::c_char,
    b"opengl\0" as *const u8 as *const libc::c_char,
    b"opengl-oml\0" as *const u8 as *const libc::c_char,
    b"opengl-swc\0" as *const u8 as *const libc::c_char,
    b"opengl-mswc\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
pub static mut BACKEND_STRS: [*const libc::c_char; 4] = [
    b"xrender\0" as *const u8 as *const libc::c_char,
    b"glx\0" as *const u8 as *const libc::c_char,
    b"xr_glx_hybrid\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
static mut VSYNC_FUNCS_INIT: [Option::<
    unsafe extern "C" fn(*mut session_t) -> bool,
>; 6] = unsafe {
    [
        None,
        Some(vsync_drm_init as unsafe extern "C" fn(*mut session_t) -> bool),
        Some(vsync_opengl_init as unsafe extern "C" fn(*mut session_t) -> bool),
        Some(vsync_opengl_oml_init as unsafe extern "C" fn(*mut session_t) -> bool),
        Some(vsync_opengl_swc_init as unsafe extern "C" fn(*mut session_t) -> bool),
        Some(vsync_opengl_mswc_init as unsafe extern "C" fn(*mut session_t) -> bool),
    ]
};
static mut VSYNC_FUNCS_WAIT: [Option::<
    unsafe extern "C" fn(*mut session_t) -> libc::c_int,
>; 6] = [None, None, None, None, None, None];
static mut VSYNC_FUNCS_DEINIT: [Option::<
    unsafe extern "C" fn(*mut session_t) -> (),
>; 6] = [None, None, None, None, None, None];
static mut background_props_str: [*const libc::c_char; 3] = [
    b"_XROOTPMAP_ID\0" as *const u8 as *const libc::c_char,
    b"_XSETROOT_ID\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
pub static mut ps_g: *mut session_t = 0 as *const session_t as *mut session_t;
unsafe extern "C" fn fade_timeout(mut ps: *mut session_t) -> libc::c_int {
    let mut diff: libc::c_int = ((*ps).o.fade_delta - get_time_ms() + (*ps).fade_time)
        as libc::c_int;
    diff = normalize_i_range(
        diff,
        0 as libc::c_int,
        ((*ps).o.fade_delta * 2 as libc::c_int as libc::c_long) as libc::c_int,
    );
    return diff;
}
unsafe extern "C" fn run_fade(
    mut ps: *mut session_t,
    mut w: *mut win,
    mut steps: libc::c_uint,
) {
    if (*w).opacity == (*w).opacity_tgt {
        return;
    }
    if !(*w).fade {
        (*w).opacity = (*w).opacity_tgt;
    } else if steps != 0 {
        if (*w).opacity < (*w).opacity_tgt {
            (*w)
                .opacity = normalize_d_range(
                (*w).opacity as libc::c_double
                    + (*ps).o.fade_in_step as libc::c_double * steps as libc::c_double,
                0.0f64,
                (*w).opacity_tgt as libc::c_double,
            ) as opacity_t;
        } else {
            (*w)
                .opacity = normalize_d_range(
                (*w).opacity as libc::c_double
                    - (*ps).o.fade_out_step as libc::c_double * steps as libc::c_double,
                (*w).opacity_tgt as libc::c_double,
                0xffffffff as libc::c_uint as libc::c_double,
            ) as opacity_t;
        }
    }
    if (*w).opacity != (*w).opacity_tgt {
        (*ps).idling = 0 as libc::c_int != 0;
    }
}
unsafe extern "C" fn set_fade_callback(
    mut ps: *mut session_t,
    mut w: *mut win,
    mut callback: Option::<unsafe extern "C" fn(*mut session_t, *mut win) -> ()>,
    mut exec_callback: bool,
) {
    let mut old_callback: Option::<
        unsafe extern "C" fn(*mut session_t, *mut win) -> (),
    > = (*w).fade_callback;
    (*w).fade_callback = callback;
    if exec_callback as libc::c_int != 0 && old_callback.is_some() {
        old_callback.unwrap()(ps, w);
        (*ps).idling = 0 as libc::c_int != 0;
    }
}
unsafe extern "C" fn gaussian(
    mut r: libc::c_double,
    mut x: libc::c_double,
    mut y: libc::c_double,
) -> libc::c_double {
    return 1 as libc::c_int as libc::c_double
        / sqrt(2 as libc::c_int as libc::c_double * 3.14159265358979323846f64 * r)
        * exp(-(x * x + y * y) / (2 as libc::c_int as libc::c_double * r * r));
}
unsafe extern "C" fn make_gaussian_map(mut r: libc::c_double) -> *mut conv {
    let mut c: *mut conv = 0 as *mut conv;
    let mut size: libc::c_int = ceil(r * 3 as libc::c_int as libc::c_double)
        as libc::c_int + 1 as libc::c_int & !(1 as libc::c_int);
    let mut center: libc::c_int = size / 2 as libc::c_int;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut t: libc::c_double = 0.;
    let mut g: libc::c_double = 0.;
    c = malloc(
        (::std::mem::size_of::<conv>() as libc::c_ulong)
            .wrapping_add(
                ((size * size) as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
                    ),
            ),
    ) as *mut conv;
    (*c).size = size;
    (*c).data = c.offset(1 as libc::c_int as isize) as *mut libc::c_double;
    t = 0.0f64;
    y = 0 as libc::c_int;
    while y < size {
        x = 0 as libc::c_int;
        while x < size {
            g = gaussian(
                r,
                (x - center) as libc::c_double,
                (y - center) as libc::c_double,
            );
            t += g;
            *((*c).data).offset((y * size + x) as isize) = g;
            x += 1;
            x;
        }
        y += 1;
        y;
    }
    y = 0 as libc::c_int;
    while y < size {
        x = 0 as libc::c_int;
        while x < size {
            *((*c).data).offset((y * size + x) as isize) /= t;
            x += 1;
            x;
        }
        y += 1;
        y;
    }
    return c;
}
unsafe extern "C" fn sum_gaussian(
    mut map: *mut conv,
    mut opacity: libc::c_double,
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut width: libc::c_int,
    mut height: libc::c_int,
) -> libc::c_uchar {
    let mut fx: libc::c_int = 0;
    let mut fy: libc::c_int = 0;
    let mut g_data: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut g_line: *mut libc::c_double = (*map).data;
    let mut g_size: libc::c_int = (*map).size;
    let mut center: libc::c_int = g_size / 2 as libc::c_int;
    let mut fx_start: libc::c_int = 0;
    let mut fx_end: libc::c_int = 0;
    let mut fy_start: libc::c_int = 0;
    let mut fy_end: libc::c_int = 0;
    let mut v: libc::c_double = 0.;
    fx_start = center - x;
    if fx_start < 0 as libc::c_int {
        fx_start = 0 as libc::c_int;
    }
    fx_end = width + center - x;
    if fx_end > g_size {
        fx_end = g_size;
    }
    fy_start = center - y;
    if fy_start < 0 as libc::c_int {
        fy_start = 0 as libc::c_int;
    }
    fy_end = height + center - y;
    if fy_end > g_size {
        fy_end = g_size;
    }
    g_line = g_line.offset((fy_start * g_size) as isize).offset(fx_start as isize);
    v = 0 as libc::c_int as libc::c_double;
    fy = fy_start;
    while fy < fy_end {
        g_data = g_line;
        g_line = g_line.offset(g_size as isize);
        fx = fx_start;
        while fx < fx_end {
            let fresh6 = g_data;
            g_data = g_data.offset(1);
            v += *fresh6;
            fx += 1;
            fx;
        }
        fy += 1;
        fy;
    }
    if v > 1 as libc::c_int as libc::c_double {
        v = 1 as libc::c_int as libc::c_double;
    }
    return (v * opacity * 255.0f64) as libc::c_uchar;
}
unsafe extern "C" fn presum_gaussian(mut ps: *mut session_t, mut map: *mut conv) {
    let mut center: libc::c_int = (*map).size / 2 as libc::c_int;
    let mut opacity: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    (*ps).cgsize = (*map).size;
    if !((*ps).shadow_corner).is_null() {
        free((*ps).shadow_corner as *mut libc::c_void);
    }
    if !((*ps).shadow_top).is_null() {
        free((*ps).shadow_top as *mut libc::c_void);
    }
    (*ps)
        .shadow_corner = malloc(
        (((*ps).cgsize + 1 as libc::c_int) * ((*ps).cgsize + 1 as libc::c_int)
            * 26 as libc::c_int) as libc::c_ulong,
    ) as *mut libc::c_uchar;
    (*ps)
        .shadow_top = malloc(
        (((*ps).cgsize + 1 as libc::c_int) * 26 as libc::c_int) as libc::c_ulong,
    ) as *mut libc::c_uchar;
    x = 0 as libc::c_int;
    while x <= (*ps).cgsize {
        *((*ps).shadow_top)
            .offset(
                (25 as libc::c_int * ((*ps).cgsize + 1 as libc::c_int) + x) as isize,
            ) = sum_gaussian(
            map,
            1 as libc::c_int as libc::c_double,
            x - center,
            center,
            (*ps).cgsize * 2 as libc::c_int,
            (*ps).cgsize * 2 as libc::c_int,
        );
        opacity = 0 as libc::c_int;
        while opacity < 25 as libc::c_int {
            *((*ps).shadow_top)
                .offset(
                    (opacity * ((*ps).cgsize + 1 as libc::c_int) + x) as isize,
                ) = (*((*ps).shadow_top)
                .offset(
                    (25 as libc::c_int * ((*ps).cgsize + 1 as libc::c_int) + x) as isize,
                ) as libc::c_int * opacity / 25 as libc::c_int) as libc::c_uchar;
            opacity += 1;
            opacity;
        }
        y = 0 as libc::c_int;
        while y <= x {
            *((*ps).shadow_corner)
                .offset(
                    (25 as libc::c_int * ((*ps).cgsize + 1 as libc::c_int)
                        * ((*ps).cgsize + 1 as libc::c_int)
                        + y * ((*ps).cgsize + 1 as libc::c_int) + x) as isize,
                ) = sum_gaussian(
                map,
                1 as libc::c_int as libc::c_double,
                x - center,
                y - center,
                (*ps).cgsize * 2 as libc::c_int,
                (*ps).cgsize * 2 as libc::c_int,
            );
            *((*ps).shadow_corner)
                .offset(
                    (25 as libc::c_int * ((*ps).cgsize + 1 as libc::c_int)
                        * ((*ps).cgsize + 1 as libc::c_int)
                        + x * ((*ps).cgsize + 1 as libc::c_int) + y) as isize,
                ) = *((*ps).shadow_corner)
                .offset(
                    (25 as libc::c_int * ((*ps).cgsize + 1 as libc::c_int)
                        * ((*ps).cgsize + 1 as libc::c_int)
                        + y * ((*ps).cgsize + 1 as libc::c_int) + x) as isize,
                );
            opacity = 0 as libc::c_int;
            while opacity < 25 as libc::c_int {
                let ref mut fresh7 = *((*ps).shadow_corner)
                    .offset(
                        (opacity * ((*ps).cgsize + 1 as libc::c_int)
                            * ((*ps).cgsize + 1 as libc::c_int)
                            + x * ((*ps).cgsize + 1 as libc::c_int) + y) as isize,
                    );
                *fresh7 = (*((*ps).shadow_corner)
                    .offset(
                        (25 as libc::c_int * ((*ps).cgsize + 1 as libc::c_int)
                            * ((*ps).cgsize + 1 as libc::c_int)
                            + y * ((*ps).cgsize + 1 as libc::c_int) + x) as isize,
                    ) as libc::c_int * opacity / 25 as libc::c_int) as libc::c_uchar;
                *((*ps).shadow_corner)
                    .offset(
                        (opacity * ((*ps).cgsize + 1 as libc::c_int)
                            * ((*ps).cgsize + 1 as libc::c_int)
                            + y * ((*ps).cgsize + 1 as libc::c_int) + x) as isize,
                    ) = *fresh7;
                opacity += 1;
                opacity;
            }
            y += 1;
            y;
        }
        x += 1;
        x;
    }
}
unsafe extern "C" fn make_shadow(
    mut ps: *mut session_t,
    mut opacity: libc::c_double,
    mut width: libc::c_int,
    mut height: libc::c_int,
) -> *mut XImage {
    let mut ximage: *mut XImage = 0 as *mut XImage;
    let mut data: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut ylimit: libc::c_int = 0;
    let mut xlimit: libc::c_int = 0;
    let mut swidth: libc::c_int = width + (*ps).cgsize;
    let mut sheight: libc::c_int = height + (*ps).cgsize;
    let mut center: libc::c_int = (*ps).cgsize / 2 as libc::c_int;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut d: libc::c_uchar = 0;
    let mut x_diff: libc::c_int = 0;
    let mut opacity_int: libc::c_int = (opacity * 25 as libc::c_int as libc::c_double)
        as libc::c_int;
    data = malloc(
        ((swidth * sheight) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong),
    ) as *mut libc::c_uchar;
    if data.is_null() {
        return 0 as *mut XImage;
    }
    ximage = XCreateImage(
        (*ps).dpy,
        (*ps).vis,
        8 as libc::c_int as libc::c_uint,
        2 as libc::c_int,
        0 as libc::c_int,
        data as *mut libc::c_char,
        swidth as libc::c_uint,
        sheight as libc::c_uint,
        8 as libc::c_int,
        (swidth as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
            as libc::c_int,
    );
    if ximage.is_null() {
        free(data as *mut libc::c_void);
        return 0 as *mut XImage;
    }
    if (*ps).cgsize > 0 as libc::c_int {
        d = *((*ps).shadow_top)
            .offset(
                (opacity_int * ((*ps).cgsize + 1 as libc::c_int) + (*ps).cgsize) as isize,
            );
    } else {
        d = sum_gaussian((*ps).gaussian_map, opacity, center, center, width, height);
    }
    memset(
        data as *mut libc::c_void,
        d as libc::c_int,
        (sheight * swidth) as libc::c_ulong,
    );
    ylimit = (*ps).cgsize;
    if ylimit > sheight / 2 as libc::c_int {
        ylimit = (sheight + 1 as libc::c_int) / 2 as libc::c_int;
    }
    xlimit = (*ps).cgsize;
    if xlimit > swidth / 2 as libc::c_int {
        xlimit = (swidth + 1 as libc::c_int) / 2 as libc::c_int;
    }
    y = 0 as libc::c_int;
    while y < ylimit {
        x = 0 as libc::c_int;
        while x < xlimit {
            if xlimit == (*ps).cgsize && ylimit == (*ps).cgsize {
                d = *((*ps).shadow_corner)
                    .offset(
                        (opacity_int * ((*ps).cgsize + 1 as libc::c_int)
                            * ((*ps).cgsize + 1 as libc::c_int)
                            + y * ((*ps).cgsize + 1 as libc::c_int) + x) as isize,
                    );
            } else {
                d = sum_gaussian(
                    (*ps).gaussian_map,
                    opacity,
                    x - center,
                    y - center,
                    width,
                    height,
                );
            }
            *data.offset((y * swidth + x) as isize) = d;
            *data.offset(((sheight - y - 1 as libc::c_int) * swidth + x) as isize) = d;
            *data
                .offset(
                    ((sheight - y - 1 as libc::c_int) * swidth
                        + (swidth - x - 1 as libc::c_int)) as isize,
                ) = d;
            *data.offset((y * swidth + (swidth - x - 1 as libc::c_int)) as isize) = d;
            x += 1;
            x;
        }
        y += 1;
        y;
    }
    x_diff = swidth - (*ps).cgsize * 2 as libc::c_int;
    if x_diff > 0 as libc::c_int && ylimit > 0 as libc::c_int {
        y = 0 as libc::c_int;
        while y < ylimit {
            if ylimit == (*ps).cgsize {
                d = *((*ps).shadow_top)
                    .offset(
                        (opacity_int * ((*ps).cgsize + 1 as libc::c_int) + y) as isize,
                    );
            } else {
                d = sum_gaussian(
                    (*ps).gaussian_map,
                    opacity,
                    center,
                    y - center,
                    width,
                    height,
                );
            }
            memset(
                &mut *data.offset((y * swidth + (*ps).cgsize) as isize)
                    as *mut libc::c_uchar as *mut libc::c_void,
                d as libc::c_int,
                x_diff as libc::c_ulong,
            );
            memset(
                &mut *data
                    .offset(
                        ((sheight - y - 1 as libc::c_int) * swidth + (*ps).cgsize)
                            as isize,
                    ) as *mut libc::c_uchar as *mut libc::c_void,
                d as libc::c_int,
                x_diff as libc::c_ulong,
            );
            y += 1;
            y;
        }
    }
    x = 0 as libc::c_int;
    while x < xlimit {
        if xlimit == (*ps).cgsize {
            d = *((*ps).shadow_top)
                .offset((opacity_int * ((*ps).cgsize + 1 as libc::c_int) + x) as isize);
        } else {
            d = sum_gaussian(
                (*ps).gaussian_map,
                opacity,
                x - center,
                center,
                width,
                height,
            );
        }
        y = (*ps).cgsize;
        while y < sheight - (*ps).cgsize {
            *data.offset((y * swidth + x) as isize) = d;
            *data.offset((y * swidth + (swidth - x - 1 as libc::c_int)) as isize) = d;
            y += 1;
            y;
        }
        x += 1;
        x;
    }
    return ximage;
}
unsafe extern "C" fn win_build_shadow(
    mut ps: *mut session_t,
    mut w: *mut win,
    mut opacity: libc::c_double,
) -> bool {
    let width: libc::c_int = (*w).widthb;
    let height: libc::c_int = (*w).heightb;
    let mut shadow_image: *mut XImage = 0 as *mut XImage;
    let mut shadow_pixmap: Pixmap = 0 as libc::c_long as Pixmap;
    let mut shadow_pixmap_argb: Pixmap = 0 as libc::c_long as Pixmap;
    let mut shadow_picture: Picture = 0 as libc::c_long as Picture;
    let mut shadow_picture_argb: Picture = 0 as libc::c_long as Picture;
    let mut gc: GC = 0 as GC;
    shadow_image = make_shadow(ps, opacity, width, height);
    if shadow_image.is_null() {
        return 0 as libc::c_long != 0;
    }
    shadow_pixmap = XCreatePixmap(
        (*ps).dpy,
        (*ps).root,
        (*shadow_image).width as libc::c_uint,
        (*shadow_image).height as libc::c_uint,
        8 as libc::c_int as libc::c_uint,
    );
    shadow_pixmap_argb = XCreatePixmap(
        (*ps).dpy,
        (*ps).root,
        (*shadow_image).width as libc::c_uint,
        (*shadow_image).height as libc::c_uint,
        32 as libc::c_int as libc::c_uint,
    );
    if !(shadow_pixmap == 0 || shadow_pixmap_argb == 0) {
        shadow_picture = XRenderCreatePicture(
            (*ps).dpy,
            shadow_pixmap,
            XRenderFindStandardFormat((*ps).dpy, 2 as libc::c_int),
            0 as libc::c_int as libc::c_ulong,
            0 as *const XRenderPictureAttributes,
        );
        shadow_picture_argb = XRenderCreatePicture(
            (*ps).dpy,
            shadow_pixmap_argb,
            XRenderFindStandardFormat((*ps).dpy, 0 as libc::c_int),
            0 as libc::c_int as libc::c_ulong,
            0 as *const XRenderPictureAttributes,
        );
        if !(shadow_picture == 0 || shadow_picture_argb == 0) {
            gc = XCreateGC(
                (*ps).dpy,
                shadow_pixmap,
                0 as libc::c_int as libc::c_ulong,
                0 as *mut XGCValues,
            );
            if !gc.is_null() {
                XPutImage(
                    (*ps).dpy,
                    shadow_pixmap,
                    gc,
                    shadow_image,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    (*shadow_image).width as libc::c_uint,
                    (*shadow_image).height as libc::c_uint,
                );
                XRenderComposite(
                    (*ps).dpy,
                    1 as libc::c_int,
                    (*ps).cshadow_picture,
                    shadow_picture,
                    shadow_picture_argb,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    (*shadow_image).width as libc::c_uint,
                    (*shadow_image).height as libc::c_uint,
                );
                (*w).shadow_paint.pixmap = shadow_pixmap_argb;
                (*w).shadow_paint.pict = shadow_picture_argb;
                xr_sync_(ps, (*w).shadow_paint.pixmap, 0 as *mut XSyncFence);
                XFreeGC((*ps).dpy, gc);
                (Some(((*shadow_image).f.destroy_image).unwrap()))
                    .unwrap()(shadow_image);
                XFreePixmap((*ps).dpy, shadow_pixmap);
                XRenderFreePicture((*ps).dpy, shadow_picture);
                return 1 as libc::c_int != 0;
            }
        }
    }
    if !shadow_image.is_null() {
        (Some(((*shadow_image).f.destroy_image).unwrap())).unwrap()(shadow_image);
    }
    if shadow_pixmap != 0 {
        XFreePixmap((*ps).dpy, shadow_pixmap);
    }
    if shadow_pixmap_argb != 0 {
        XFreePixmap((*ps).dpy, shadow_pixmap_argb);
    }
    if shadow_picture != 0 {
        XRenderFreePicture((*ps).dpy, shadow_picture);
    }
    if shadow_picture_argb != 0 {
        XRenderFreePicture((*ps).dpy, shadow_picture_argb);
    }
    if !gc.is_null() {
        XFreeGC((*ps).dpy, gc);
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn solid_picture(
    mut ps: *mut session_t,
    mut argb: bool,
    mut a: libc::c_double,
    mut r: libc::c_double,
    mut g: libc::c_double,
    mut b: libc::c_double,
) -> Picture {
    let mut pixmap: Pixmap = 0;
    let mut picture: Picture = 0;
    let mut pa: XRenderPictureAttributes = XRenderPictureAttributes {
        repeat: 0,
        alpha_map: 0,
        alpha_x_origin: 0,
        alpha_y_origin: 0,
        clip_x_origin: 0,
        clip_y_origin: 0,
        clip_mask: 0,
        graphics_exposures: 0,
        subwindow_mode: 0,
        poly_edge: 0,
        poly_mode: 0,
        dither: 0,
        component_alpha: 0,
    };
    let mut c: XRenderColor = XRenderColor {
        red: 0,
        green: 0,
        blue: 0,
        alpha: 0,
    };
    pixmap = XCreatePixmap(
        (*ps).dpy,
        (*ps).root,
        1 as libc::c_int as libc::c_uint,
        1 as libc::c_int as libc::c_uint,
        (if argb as libc::c_int != 0 { 32 as libc::c_int } else { 8 as libc::c_int })
            as libc::c_uint,
    );
    if pixmap == 0 {
        return 0 as libc::c_long as Picture;
    }
    pa.repeat = 1 as libc::c_int;
    picture = XRenderCreatePicture(
        (*ps).dpy,
        pixmap,
        XRenderFindStandardFormat(
            (*ps).dpy,
            if argb as libc::c_int != 0 { 0 as libc::c_int } else { 2 as libc::c_int },
        ),
        ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_ulong,
        &mut pa,
    );
    if picture == 0 {
        XFreePixmap((*ps).dpy, pixmap);
        return 0 as libc::c_long as Picture;
    }
    c.alpha = (a * 0xffff as libc::c_int as libc::c_double) as libc::c_ushort;
    c.red = (r * 0xffff as libc::c_int as libc::c_double) as libc::c_ushort;
    c.green = (g * 0xffff as libc::c_int as libc::c_double) as libc::c_ushort;
    c.blue = (b * 0xffff as libc::c_int as libc::c_double) as libc::c_ushort;
    XRenderFillRectangle(
        (*ps).dpy,
        1 as libc::c_int,
        picture,
        &mut c,
        0 as libc::c_int,
        0 as libc::c_int,
        1 as libc::c_int as libc::c_uint,
        1 as libc::c_int as libc::c_uint,
    );
    XFreePixmap((*ps).dpy, pixmap);
    return picture;
}
unsafe extern "C" fn discard_ignore(
    mut ps: *mut session_t,
    mut sequence: libc::c_ulong,
) {
    while !((*ps).ignore_head).is_null() {
        if !(sequence.wrapping_sub((*(*ps).ignore_head).sequence) as libc::c_long
            > 0 as libc::c_int as libc::c_long)
        {
            break;
        }
        let mut next: *mut ignore_t = (*(*ps).ignore_head).next;
        free((*ps).ignore_head as *mut libc::c_void);
        (*ps).ignore_head = next;
        if ((*ps).ignore_head).is_null() {
            (*ps).ignore_tail = &mut (*ps).ignore_head;
        }
    }
}
unsafe extern "C" fn set_ignore(mut ps: *mut session_t, mut sequence: libc::c_ulong) {
    if (*ps).o.show_all_xerrors {
        return;
    }
    let mut i: *mut ignore_t = malloc(::std::mem::size_of::<ignore_t>() as libc::c_ulong)
        as *mut ignore_t;
    if i.is_null() {
        return;
    }
    (*i).sequence = sequence;
    (*i).next = 0 as *mut _ignore;
    *(*ps).ignore_tail = i;
    (*ps).ignore_tail = &mut (*i).next;
}
unsafe extern "C" fn should_ignore(
    mut ps: *mut session_t,
    mut sequence: libc::c_ulong,
) -> libc::c_int {
    discard_ignore(ps, sequence);
    return (!((*ps).ignore_head).is_null() && (*(*ps).ignore_head).sequence == sequence)
        as libc::c_int;
}
pub unsafe extern "C" fn wid_get_prop_adv(
    mut ps: *const session_t,
    mut w: Window,
    mut atom: Atom,
    mut offset: libc::c_long,
    mut length: libc::c_long,
    mut rtype: Atom,
    mut rformat: libc::c_int,
) -> winprop_t {
    let mut type_0: Atom = 0 as libc::c_long as Atom;
    let mut format: libc::c_int = 0 as libc::c_int;
    let mut nitems: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    let mut after: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    let mut data: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    if 0 as libc::c_int
        == XGetWindowProperty(
            (*ps).dpy,
            w,
            atom,
            offset,
            length,
            0 as libc::c_int,
            rtype,
            &mut type_0,
            &mut format,
            &mut nitems,
            &mut after,
            &mut data,
        ) && nitems != 0
        && (0 as libc::c_long as libc::c_ulong == type_0 || type_0 == rtype)
        && (rformat == 0 || format == rformat)
        && (8 as libc::c_int == format || 16 as libc::c_int == format
            || 32 as libc::c_int == format)
    {
        return {
            let mut init = winprop_t {
                data: C2RustUnnamed_3 { p8: data },
                nitems: nitems,
                type_0: type_0,
                format: format,
            };
            init
        };
    }
    cxfree(data as *mut libc::c_void);
    return {
        let mut init = winprop_t {
            data: C2RustUnnamed_3 {
                p8: 0 as *mut libc::c_uchar,
            },
            nitems: 0 as libc::c_int as libc::c_ulong,
            type_0: 0 as libc::c_long as Atom,
            format: 0 as libc::c_int,
        };
        init
    };
}
unsafe extern "C" fn win_rounded_corners(mut ps: *mut session_t, mut w: *mut win) {
    (*w).rounded_corners = 0 as libc::c_int != 0;
    if !(*w).bounding_shaped {
        return;
    }
    if (*w).border_size == 0 {
        (*w).border_size = border_size(ps, w, 1 as libc::c_int != 0);
    }
    if (*w).border_size == 0 {
        return;
    }
    let mut minwidth: libc::c_ushort = max_i(
        ((*w).widthb as libc::c_double * (1 as libc::c_int as libc::c_double - 0.05f64))
            as libc::c_int,
        (*w).widthb - 10 as libc::c_int,
    ) as libc::c_ushort;
    let mut minheight: libc::c_ushort = max_i(
        ((*w).heightb as libc::c_double * (1 as libc::c_int as libc::c_double - 0.05f64))
            as libc::c_int,
        (*w).heightb - 10 as libc::c_int,
    ) as libc::c_ushort;
    let mut nrects: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut rects: *mut XRectangle = XFixesFetchRegion(
        (*ps).dpy,
        (*w).border_size,
        &mut nrects,
    );
    if rects.is_null() {
        return;
    }
    i = 0 as libc::c_int;
    while i < nrects {
        if (*rects.offset(i as isize)).width as libc::c_int >= minwidth as libc::c_int
            && (*rects.offset(i as isize)).height as libc::c_int
                >= minheight as libc::c_int
        {
            (*w).rounded_corners = 1 as libc::c_int != 0;
            break;
        } else {
            i += 1;
            i;
        }
    }
    cxfree(rects as *mut libc::c_void);
}
unsafe extern "C" fn condlst_add(
    mut ps: *mut session_t,
    mut pcondlst: *mut *mut c2_lptr_t,
    mut pattern: *const libc::c_char,
) -> bool {
    if pattern.is_null() {
        return 0 as libc::c_int != 0;
    }
    if (c2_parsed(ps, pcondlst, pattern, 0 as *mut libc::c_void)).is_null() {
        exit(1 as libc::c_int);
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn determine_evmask(
    mut ps: *mut session_t,
    mut wid: Window,
    mut mode: win_evmode_t,
) -> libc::c_long {
    let mut evmask: libc::c_long = 0 as libc::c_long;
    let mut w: *mut win = 0 as *mut win;
    if WIN_EVMODE_FRAME as libc::c_int as libc::c_uint == mode as libc::c_uint
        || {
            w = find_win(ps, wid);
            !w.is_null() && 2 as libc::c_int == (*w).a.map_state
        }
    {
        evmask |= (1 as libc::c_long) << 22 as libc::c_int;
        if (*ps).o.track_focus as libc::c_int != 0 && !(*ps).o.use_ewmh_active_win {
            evmask |= (1 as libc::c_long) << 21 as libc::c_int;
        }
    }
    if WIN_EVMODE_CLIENT as libc::c_int as libc::c_uint == mode as libc::c_uint
        || {
            w = find_toplevel(ps, wid);
            !w.is_null() && 2 as libc::c_int == (*w).a.map_state
        }
    {
        if (*ps).o.frame_opacity != 0. || (*ps).o.track_wdata as libc::c_int != 0
            || !((*ps).track_atom_lst).is_null()
            || (*ps).o.detect_client_opacity as libc::c_int != 0
        {
            evmask |= (1 as libc::c_long) << 22 as libc::c_int;
        }
    }
    return evmask;
}
unsafe extern "C" fn find_toplevel2(
    mut ps: *mut session_t,
    mut wid: Window,
) -> *mut win {
    let mut w: *mut win = 0 as *mut win;
    while wid != 0 && wid != (*ps).root
        && {
            w = find_win(ps, wid);
            w.is_null()
        }
    {
        let mut troot: Window = 0;
        let mut parent: Window = 0;
        let mut tchildren: *mut Window = 0 as *mut Window;
        let mut tnchildren: libc::c_uint = 0;
        if XQueryTree(
            (*ps).dpy,
            wid,
            &mut troot,
            &mut parent,
            &mut tchildren,
            &mut tnchildren,
        ) == 0
        {
            parent = 0 as libc::c_int as Window;
            break;
        } else {
            cxfree(tchildren as *mut libc::c_void);
            wid = parent;
        }
    }
    return w;
}
unsafe extern "C" fn recheck_focus(mut ps: *mut session_t) -> *mut win {
    if (*ps).o.use_ewmh_active_win {
        update_ewmh_active_win(ps);
        return (*ps).active_win;
    }
    let mut wid: Window = 0 as libc::c_int as Window;
    let mut revert_to: libc::c_int = 0;
    XGetInputFocus((*ps).dpy, &mut wid, &mut revert_to);
    let mut w: *mut win = find_win_all(ps, wid);
    if !w.is_null() {
        win_set_focused(ps, w, 1 as libc::c_int != 0);
        return w;
    }
    return 0 as *mut win;
}
unsafe extern "C" fn get_root_tile(mut ps: *mut session_t) -> bool {
    (*ps).root_tile_fill = 0 as libc::c_int != 0;
    let mut fill: bool = 0 as libc::c_int != 0;
    let mut pixmap: Pixmap = 0 as libc::c_long as Pixmap;
    let mut p: libc::c_int = 0 as libc::c_int;
    while !(background_props_str[p as usize]).is_null() {
        let mut prop: winprop_t = wid_get_prop(
            ps,
            (*ps).root,
            get_atom(ps, background_props_str[p as usize]),
            1 as libc::c_long,
            20 as libc::c_int as Atom,
            32 as libc::c_int,
        );
        if prop.nitems != 0 {
            pixmap = *prop.data.p32 as Pixmap;
            fill = 0 as libc::c_int != 0;
            free_winprop(&mut prop);
            break;
        } else {
            free_winprop(&mut prop);
            p += 1;
            p;
        }
    }
    if pixmap != 0 && !validate_pixmap(ps, pixmap) {
        pixmap = 0 as libc::c_long as Pixmap;
    }
    if pixmap == 0 {
        pixmap = XCreatePixmap(
            (*ps).dpy,
            (*ps).root,
            1 as libc::c_int as libc::c_uint,
            1 as libc::c_int as libc::c_uint,
            (*ps).depth as libc::c_uint,
        );
        fill = 1 as libc::c_int != 0;
    }
    let mut pa: XRenderPictureAttributes = {
        let mut init = _XRenderPictureAttributes {
            repeat: 1 as libc::c_int,
            alpha_map: 0,
            alpha_x_origin: 0,
            alpha_y_origin: 0,
            clip_x_origin: 0,
            clip_y_origin: 0,
            clip_mask: 0,
            graphics_exposures: 0,
            subwindow_mode: 0,
            poly_edge: 0,
            poly_mode: 0,
            dither: 0,
            component_alpha: 0,
        };
        init
    };
    (*ps)
        .root_tile_paint
        .pict = XRenderCreatePicture(
        (*ps).dpy,
        pixmap,
        XRenderFindVisualFormat((*ps).dpy, (*ps).vis),
        ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_ulong,
        &mut pa,
    );
    if fill {
        let mut c: XRenderColor = XRenderColor {
            red: 0,
            green: 0,
            blue: 0,
            alpha: 0,
        };
        c.blue = 0x8080 as libc::c_int as libc::c_ushort;
        c.green = c.blue;
        c.red = c.green;
        c.alpha = 0xffff as libc::c_int as libc::c_ushort;
        XRenderFillRectangle(
            (*ps).dpy,
            1 as libc::c_int,
            (*ps).root_tile_paint.pict,
            &mut c,
            0 as libc::c_int,
            0 as libc::c_int,
            1 as libc::c_int as libc::c_uint,
            1 as libc::c_int as libc::c_uint,
        );
    }
    (*ps).root_tile_fill = fill;
    (*ps).root_tile_paint.pixmap = pixmap;
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn paint_root(mut ps: *mut session_t, mut reg_paint: XserverRegion) {
    if (*ps).root_tile_paint.pixmap == 0 {
        get_root_tile(ps);
    }
    win_render(
        ps,
        0 as *mut win,
        0 as libc::c_int,
        0 as libc::c_int,
        (*ps).root_width,
        (*ps).root_height,
        1.0f64,
        reg_paint,
        0 as *const reg_data_t,
        (*ps).root_tile_paint.pict,
    );
}
unsafe extern "C" fn win_get_region(
    mut ps: *mut session_t,
    mut w: *mut win,
    mut use_offset: bool,
) -> XserverRegion {
    let mut r: XRectangle = XRectangle {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    };
    r
        .x = (if use_offset as libc::c_int != 0 { (*w).a.x } else { 0 as libc::c_int })
        as libc::c_short;
    r
        .y = (if use_offset as libc::c_int != 0 { (*w).a.y } else { 0 as libc::c_int })
        as libc::c_short;
    r.width = (*w).widthb as libc::c_ushort;
    r.height = (*w).heightb as libc::c_ushort;
    return XFixesCreateRegion((*ps).dpy, &mut r, 1 as libc::c_int);
}
unsafe extern "C" fn win_get_region_noframe(
    mut ps: *mut session_t,
    mut w: *mut win,
    mut use_offset: bool,
) -> XserverRegion {
    let extents: margin_t = win_calc_frame_extents(ps, w);
    let mut r: XRectangle = XRectangle {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    };
    r
        .x = ((if use_offset as libc::c_int != 0 { (*w).a.x } else { 0 as libc::c_int })
        + extents.left) as libc::c_short;
    r
        .y = ((if use_offset as libc::c_int != 0 { (*w).a.y } else { 0 as libc::c_int })
        + extents.top) as libc::c_short;
    r
        .width = max_i((*w).a.width - extents.left - extents.right, 0 as libc::c_int)
        as libc::c_ushort;
    r
        .height = max_i((*w).a.height - extents.top - extents.bottom, 0 as libc::c_int)
        as libc::c_ushort;
    if r.width as libc::c_int > 0 as libc::c_int
        && r.height as libc::c_int > 0 as libc::c_int
    {
        return XFixesCreateRegion((*ps).dpy, &mut r, 1 as libc::c_int)
    } else {
        return XFixesCreateRegion((*ps).dpy, 0 as *mut XRectangle, 0 as libc::c_int)
    };
}
unsafe extern "C" fn win_extents(
    mut ps: *mut session_t,
    mut w: *mut win,
) -> XserverRegion {
    let mut r: XRectangle = XRectangle {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    };
    r.x = (*w).a.x as libc::c_short;
    r.y = (*w).a.y as libc::c_short;
    r.width = (*w).widthb as libc::c_ushort;
    r.height = (*w).heightb as libc::c_ushort;
    if (*w).shadow {
        let mut sr: XRectangle = XRectangle {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        };
        sr.x = ((*w).a.x + (*w).shadow_dx) as libc::c_short;
        sr.y = ((*w).a.y + (*w).shadow_dy) as libc::c_short;
        sr.width = (*w).shadow_width as libc::c_ushort;
        sr.height = (*w).shadow_height as libc::c_ushort;
        if (sr.x as libc::c_int) < r.x as libc::c_int {
            r
                .width = (r.x as libc::c_int + r.width as libc::c_int
                - sr.x as libc::c_int) as libc::c_ushort;
            r.x = sr.x;
        }
        if (sr.y as libc::c_int) < r.y as libc::c_int {
            r
                .height = (r.y as libc::c_int + r.height as libc::c_int
                - sr.y as libc::c_int) as libc::c_ushort;
            r.y = sr.y;
        }
        if sr.x as libc::c_int + sr.width as libc::c_int
            > r.x as libc::c_int + r.width as libc::c_int
        {
            r
                .width = (sr.x as libc::c_int + sr.width as libc::c_int
                - r.x as libc::c_int) as libc::c_ushort;
        }
        if sr.y as libc::c_int + sr.height as libc::c_int
            > r.y as libc::c_int + r.height as libc::c_int
        {
            r
                .height = (sr.y as libc::c_int + sr.height as libc::c_int
                - r.y as libc::c_int) as libc::c_ushort;
        }
    }
    return XFixesCreateRegion((*ps).dpy, &mut r, 1 as libc::c_int);
}
unsafe extern "C" fn border_size(
    mut ps: *mut session_t,
    mut w: *mut win,
    mut use_offset: bool,
) -> XserverRegion {
    let mut fin: XserverRegion = win_get_region(ps, w, use_offset);
    if (*w).bounding_shaped {
        let mut border: XserverRegion = XFixesCreateRegionFromWindow(
            (*ps).dpy,
            (*w).id,
            0 as libc::c_int,
        );
        if border == 0 {
            return fin;
        }
        if use_offset {
            XFixesTranslateRegion(
                (*ps).dpy,
                border,
                (*w).a.x + (*w).a.border_width,
                (*w).a.y + (*w).a.border_width,
            );
        }
        XFixesIntersectRegion((*ps).dpy, fin, fin, border);
        XFixesDestroyRegion((*ps).dpy, border);
    }
    return fin;
}
unsafe extern "C" fn find_client_win(mut ps: *mut session_t, mut w: Window) -> Window {
    if wid_has_prop(ps, w, (*ps).atom_client) {
        return w;
    }
    let mut children: *mut Window = 0 as *mut Window;
    let mut nchildren: libc::c_uint = 0;
    let mut i: libc::c_uint = 0;
    let mut ret: Window = 0 as libc::c_int as Window;
    if !wid_get_children(ps, w, &mut children, &mut nchildren) {
        return 0 as libc::c_int as Window;
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < nchildren {
        ret = find_client_win(ps, *children.offset(i as isize));
        if ret != 0 {
            break;
        }
        i = i.wrapping_add(1);
        i;
    }
    cxfree(children as *mut libc::c_void);
    return ret;
}
unsafe extern "C" fn get_frame_extents(
    mut ps: *mut session_t,
    mut w: *mut win,
    mut client: Window,
) {
    memset(
        &mut (*w).frame_extents as *mut margin_t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<margin_t>() as libc::c_ulong,
    );
    let mut prop: winprop_t = wid_get_prop(
        ps,
        client,
        (*ps).atom_frame_extents,
        4 as libc::c_long,
        6 as libc::c_int as Atom,
        32 as libc::c_int,
    );
    if 4 as libc::c_int as libc::c_ulong == prop.nitems {
        let extents: *const libc::c_long = prop.data.p32;
        (*w)
            .frame_extents
            .left = *extents.offset(0 as libc::c_int as isize) as libc::c_int;
        (*w)
            .frame_extents
            .right = *extents.offset(1 as libc::c_int as isize) as libc::c_int;
        (*w)
            .frame_extents
            .top = *extents.offset(2 as libc::c_int as isize) as libc::c_int;
        (*w)
            .frame_extents
            .bottom = *extents.offset(3 as libc::c_int as isize) as libc::c_int;
        if (*ps).o.frame_opacity != 0. {
            update_reg_ignore_expire(ps, w);
        }
    }
    free_winprop(&mut prop);
}
#[inline]
unsafe extern "C" fn get_alpha_pict_d(
    mut ps: *mut session_t,
    mut o: libc::c_double,
) -> Picture {
    return *((*ps).alpha_picts)
        .offset(round(normalize_d(o) / (*ps).o.alpha_step) as libc::c_int as isize);
}
#[inline]
unsafe extern "C" fn get_alpha_pict_o(
    mut ps: *mut session_t,
    mut o: opacity_t,
) -> Picture {
    return get_alpha_pict_d(
        ps,
        o as libc::c_double / 0xffffffff as libc::c_uint as libc::c_double,
    );
}
unsafe extern "C" fn paint_preprocess(
    mut ps: *mut session_t,
    mut list: *mut win,
) -> *mut win {
    let mut t: *mut win = 0 as *mut win;
    let mut next: *mut win = 0 as *mut win;
    let mut steps: time_ms_t = 0 as libc::c_long;
    if (*ps).fade_time != 0 {
        steps = (((get_time_ms() - (*ps).fade_time) as libc::c_double
            + 0.2f64 * (*ps).o.fade_delta as libc::c_double)
            / (*ps).o.fade_delta as libc::c_double) as time_ms_t;
    }
    if (*ps).fade_time == 0 || steps < 0 as libc::c_long {
        (*ps).fade_time = get_time_ms();
        steps = 0 as libc::c_long;
    }
    (*ps).fade_time += steps * (*ps).o.fade_delta;
    let mut last_reg_ignore: XserverRegion = 0 as libc::c_long as XserverRegion;
    let mut unredir_possible: bool = 0 as libc::c_int != 0;
    let mut is_highest: bool = 1 as libc::c_int != 0;
    let mut w: *mut win = list;
    while !w.is_null() {
        let mut to_paint: bool = 1 as libc::c_int != 0;
        let mode_old: winmode_t = (*w).mode;
        next = (*w).next;
        let mut opacity_old: opacity_t = (*w).opacity;
        if (*w).flags & 0x1 as libc::c_int as libc::c_long != 0 {
            free_paint(ps, &mut (*w).shadow_paint);
        }
        if (*ps).reg_ignore_expire {
            free_region(ps, &mut (*w).reg_ignore);
        }
        if 0 as libc::c_int == (*w).a.map_state {
            win_set_shadow(ps, w, (*w).shadow_last);
            (*w).fade = (*w).fade_last;
            win_set_invert_color(ps, w, (*w).invert_color_last);
            win_set_blur_background(ps, w, (*w).blur_background_last);
        }
        if 0x4 as libc::c_int as libc::c_long & (*w).flags != 0 {
            calc_opacity(ps, w);
            calc_dim(ps, w);
        }
        run_fade(ps, w, steps as libc::c_uint);
        if !(*w).damaged || (*w).a.x + (*w).a.width < 1 as libc::c_int
            || (*w).a.y + (*w).a.height < 1 as libc::c_int
            || (*w).a.x >= (*ps).root_width || (*w).a.y >= (*ps).root_height
            || (0 as libc::c_int == (*w).a.map_state
                || (*w).destroyed as libc::c_int != 0) && (*w).paint.pixmap == 0
            || get_alpha_pict_o(ps, (*w).opacity)
                == *((*ps).alpha_picts).offset(0 as libc::c_int as isize)
            || (*w).paint_excluded as libc::c_int != 0
        {
            to_paint = 0 as libc::c_int != 0;
        }
        if to_paint as libc::c_int != 0
            && (!(*w).to_paint || (*w).opacity != opacity_old)
        {
            win_determine_mode(ps, w);
        }
        if to_paint {
            if (*w).border_size == 0 {
                (*w).border_size = border_size(ps, w, 1 as libc::c_int != 0);
            }
            if (*w).extents == 0 {
                (*w).extents = win_extents(ps, w);
            }
            let mut frame_opacity_old: libc::c_double = (*w).frame_opacity;
            if (*ps).o.frame_opacity != 0. && 1.0f64 != (*ps).o.frame_opacity
                && win_has_frame(w) as libc::c_int != 0
            {
                (*w).frame_opacity = get_opacity_percent(w) * (*ps).o.frame_opacity;
            } else {
                (*w).frame_opacity = 0.0f64;
            }
            if (*w).to_paint as libc::c_int != 0
                && WMODE_SOLID as libc::c_int as libc::c_uint == mode_old as libc::c_uint
                && (0.0f64 == frame_opacity_old) as libc::c_int
                    != (0.0f64 == (*w).frame_opacity) as libc::c_int
            {
                (*ps).reg_ignore_expire = 1 as libc::c_int != 0;
            }
            if (*w).frame_opacity != 0. {
                (*w).shadow_opacity = (*ps).o.shadow_opacity * (*w).frame_opacity;
            } else {
                (*w).shadow_opacity = (*ps).o.shadow_opacity * get_opacity_percent(w);
            }
        }
        if to_paint as libc::c_int != (*w).to_paint as libc::c_int
            || (*w).opacity != opacity_old
        {
            add_damage_win(ps, w);
        }
        if (to_paint as libc::c_int != 0
            && WMODE_SOLID as libc::c_int as libc::c_uint == (*w).mode as libc::c_uint)
            as libc::c_int
            != ((*w).to_paint as libc::c_int != 0
                && WMODE_SOLID as libc::c_int as libc::c_uint
                    == mode_old as libc::c_uint) as libc::c_int
        {
            (*ps).reg_ignore_expire = 1 as libc::c_int != 0;
        }
        if to_paint {
            if (*ps).reg_ignore_expire as libc::c_int != 0 || !(*w).to_paint {
                free_region(ps, &mut (*w).reg_ignore);
                if win_is_solid(ps, w) {
                    if (*w).frame_opacity == 0. {
                        if (*w).border_size != 0 {
                            (*w).reg_ignore = copy_region(ps, (*w).border_size);
                        } else {
                            (*w)
                                .reg_ignore = win_get_region(ps, w, 1 as libc::c_int != 0);
                        }
                    } else {
                        (*w)
                            .reg_ignore = win_get_region_noframe(
                            ps,
                            w,
                            1 as libc::c_int != 0,
                        );
                        if (*w).border_size != 0 {
                            XFixesIntersectRegion(
                                (*ps).dpy,
                                (*w).reg_ignore,
                                (*w).reg_ignore,
                                (*w).border_size,
                            );
                        }
                    }
                    if last_reg_ignore != 0 {
                        XFixesUnionRegion(
                            (*ps).dpy,
                            (*w).reg_ignore,
                            (*w).reg_ignore,
                            last_reg_ignore,
                        );
                    }
                } else if last_reg_ignore != 0 {
                    (*w).reg_ignore = copy_region(ps, last_reg_ignore);
                } else {
                    (*w).reg_ignore = 0 as libc::c_long as XserverRegion;
                }
            }
            last_reg_ignore = (*w).reg_ignore;
            if (*ps).o.unredir_if_possible as libc::c_int != 0
                && is_highest as libc::c_int != 0 && to_paint as libc::c_int != 0
            {
                is_highest = 0 as libc::c_int != 0;
                if win_is_solid(ps, w) as libc::c_int != 0
                    && ((*w).frame_opacity == 0. || !win_has_frame(w))
                    && win_is_fullscreen(ps, w) as libc::c_int != 0
                    && !(*w).unredir_if_possible_excluded
                {
                    unredir_possible = 1 as libc::c_int != 0;
                }
            }
            (*w).flags = 0 as libc::c_int as int_fast16_t;
        }
        let mut destroyed: bool = (*w).opacity_tgt == (*w).opacity
            && (*w).destroyed as libc::c_int != 0;
        if to_paint {
            (*w).prev_trans = t;
            t = w;
        } else {
            check_fade_fin(ps, w);
        }
        if !destroyed {
            (*w).to_paint = to_paint;
            if (*w).to_paint {
                (*w).shadow_last = (*w).shadow;
                (*w).fade_last = (*w).fade;
                (*w).invert_color_last = (*w).invert_color;
                (*w).blur_background_last = (*w).blur_background;
            }
        }
        w = next;
    }
    if UNSET as libc::c_int as libc::c_uint != (*ps).o.redirected_force as libc::c_uint {
        unredir_possible = (*ps).o.redirected_force as u64 == 0;
    }
    if (*ps).o.unredir_if_possible as libc::c_int != 0 && is_highest as libc::c_int != 0
        && !(*ps).redirected
    {
        unredir_possible = 1 as libc::c_int != 0;
    }
    if unredir_possible {
        if (*ps).redirected {
            if (*ps).o.unredir_if_possible_delay == 0
                || (*ps).tmout_unredir_hit as libc::c_int != 0
            {
                redir_stop(ps);
            } else if !(*(*ps).tmout_unredir).enabled {
                timeout_reset(ps, (*ps).tmout_unredir);
                (*(*ps).tmout_unredir).enabled = 1 as libc::c_int != 0;
            }
        }
    } else {
        (*(*ps).tmout_unredir).enabled = 0 as libc::c_int != 0;
        redir_start(ps);
    }
    return t;
}
#[inline]
unsafe extern "C" fn win_paint_shadow(
    mut ps: *mut session_t,
    mut w: *mut win,
    mut reg_paint: XserverRegion,
    mut pcache_reg: *const reg_data_t,
) {
    paint_bind_tex(
        ps,
        &mut (*w).shadow_paint,
        0 as libc::c_int as libc::c_uint,
        0 as libc::c_int as libc::c_uint,
        32 as libc::c_int as libc::c_uint,
        0 as libc::c_int != 0,
    );
    if !paint_isvalid(ps, &mut (*w).shadow_paint) {
        fprintf(
            stderr,
            b"%s(%#010lx): Missing painting data. This is a bad sign.\n\0" as *const u8
                as *const libc::c_char,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"win_paint_shadow\0"))
                .as_ptr(),
            (*w).id,
        );
        return;
    }
    render_(
        ps,
        0 as libc::c_int,
        0 as libc::c_int,
        (*w).a.x + (*w).shadow_dx,
        (*w).a.y + (*w).shadow_dy,
        (*w).shadow_width,
        (*w).shadow_height,
        (*w).shadow_opacity,
        1 as libc::c_int != 0,
        0 as libc::c_int != 0,
        (*w).shadow_paint.pict,
        (*w).shadow_paint.ptex,
        reg_paint,
        pcache_reg,
    );
}
#[inline]
unsafe extern "C" fn xr_build_picture(
    mut ps: *mut session_t,
    mut wid: libc::c_int,
    mut hei: libc::c_int,
    mut pictfmt: *mut XRenderPictFormat,
) -> Picture {
    if pictfmt.is_null() {
        pictfmt = XRenderFindVisualFormat((*ps).dpy, (*ps).vis);
    }
    let mut depth: libc::c_int = (*pictfmt).depth;
    let mut tmp_pixmap: Pixmap = XCreatePixmap(
        (*ps).dpy,
        (*ps).root,
        wid as libc::c_uint,
        hei as libc::c_uint,
        depth as libc::c_uint,
    );
    if tmp_pixmap == 0 {
        return 0 as libc::c_long as Picture;
    }
    let mut tmp_picture: Picture = XRenderCreatePicture(
        (*ps).dpy,
        tmp_pixmap,
        pictfmt,
        0 as libc::c_int as libc::c_ulong,
        0 as *const XRenderPictureAttributes,
    );
    free_pixmap(ps, &mut tmp_pixmap);
    return tmp_picture;
}
unsafe extern "C" fn xr_blur_dst(
    mut ps: *mut session_t,
    mut tgt_buffer: Picture,
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut wid: libc::c_int,
    mut hei: libc::c_int,
    mut blur_kerns: *mut *mut XFixed,
    mut reg_clip: XserverRegion,
) -> bool {
    let mut tmp_picture: Picture = xr_build_picture(
        ps,
        wid,
        hei,
        0 as *mut XRenderPictFormat,
    );
    if tmp_picture == 0 {
        fprintf(
            stderr,
            b"%s(): Failed to build intermediate Picture.\n\0" as *const u8
                as *const libc::c_char,
            (*::std::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"xr_blur_dst\0"))
                .as_ptr(),
        );
        return 0 as libc::c_int != 0;
    }
    if reg_clip != 0 && tmp_picture != 0 {
        XFixesSetPictureClipRegion(
            (*ps).dpy,
            tmp_picture,
            reg_clip as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int as XserverRegion,
        );
    }
    let mut src_pict: Picture = tgt_buffer;
    let mut dst_pict: Picture = tmp_picture;
    let mut i: libc::c_int = 0 as libc::c_int;
    while !(*blur_kerns.offset(i as isize)).is_null() {
        let mut convolution_blur: *mut XFixed = *blur_kerns.offset(i as isize);
        let mut kwid: libc::c_int = (*convolution_blur.offset(0 as libc::c_int as isize)
            as XDouble / 65536 as libc::c_int as libc::c_double) as libc::c_int;
        let mut khei: libc::c_int = (*convolution_blur.offset(1 as libc::c_int as isize)
            as XDouble / 65536 as libc::c_int as libc::c_double) as libc::c_int;
        let mut rd_from_tgt: bool = tgt_buffer == src_pict;
        XRenderSetPictureFilter(
            (*ps).dpy,
            src_pict,
            b"convolution\0" as *const u8 as *const libc::c_char,
            convolution_blur,
            kwid * khei + 2 as libc::c_int,
        );
        XRenderComposite(
            (*ps).dpy,
            1 as libc::c_int,
            src_pict,
            0 as libc::c_long as Picture,
            dst_pict,
            if rd_from_tgt as libc::c_int != 0 { x } else { 0 as libc::c_int },
            if rd_from_tgt as libc::c_int != 0 { y } else { 0 as libc::c_int },
            0 as libc::c_int,
            0 as libc::c_int,
            if rd_from_tgt as libc::c_int != 0 { 0 as libc::c_int } else { x },
            if rd_from_tgt as libc::c_int != 0 { 0 as libc::c_int } else { y },
            wid as libc::c_uint,
            hei as libc::c_uint,
        );
        xrfilter_reset(ps, src_pict);
        let mut tmp: XserverRegion = src_pict;
        src_pict = dst_pict;
        dst_pict = tmp;
        i += 1;
        i;
    }
    if src_pict != tgt_buffer {
        XRenderComposite(
            (*ps).dpy,
            1 as libc::c_int,
            src_pict,
            0 as libc::c_long as Picture,
            tgt_buffer,
            0 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
            x,
            y,
            wid as libc::c_uint,
            hei as libc::c_uint,
        );
    }
    free_picture(ps, &mut tmp_picture);
    return 1 as libc::c_int != 0;
}
#[inline]
unsafe extern "C" fn win_blur_background(
    mut ps: *mut session_t,
    mut w: *mut win,
    mut tgt_buffer: Picture,
    mut reg_paint: XserverRegion,
    mut pcache_reg: *const reg_data_t,
) {
    let x: libc::c_int = (*w).a.x;
    let y: libc::c_int = (*w).a.y;
    let wid: libc::c_int = (*w).widthb;
    let hei: libc::c_int = (*w).heightb;
    let mut factor_center: libc::c_double = 1.0f64;
    if !(*ps).o.blur_background_fixed {
        let mut pct: libc::c_double = 1.0f64
            - get_opacity_percent(w) * (1.0f64 - 1.0f64 / 9.0f64);
        factor_center = pct * 8.0f64 / (1.1f64 - pct);
    }
    match (*ps).o.backend as libc::c_uint {
        0 | 2 => {
            let mut i: libc::c_int = 0 as libc::c_int;
            while i < 5 as libc::c_int {
                let mut kern_src: *mut XFixed = (*ps).o.blur_kerns[i as usize];
                let mut kern_dst: *mut XFixed = (*ps).blur_kerns_cache[i as usize];
                if kern_src.is_null() {
                    break;
                }
                if !((*ps).o.blur_background_fixed as libc::c_int != 0
                    && !kern_dst.is_null())
                {
                    let mut kwid: libc::c_int = (*kern_src
                        .offset(0 as libc::c_int as isize) as XDouble
                        / 65536 as libc::c_int as libc::c_double) as libc::c_int;
                    let mut khei: libc::c_int = (*kern_src
                        .offset(1 as libc::c_int as isize) as XDouble
                        / 65536 as libc::c_int as libc::c_double) as libc::c_int;
                    if kern_dst.is_null() {
                        kern_dst = malloc(
                            ((kwid * khei + 2 as libc::c_int) as libc::c_ulong)
                                .wrapping_mul(
                                    ::std::mem::size_of::<XFixed>() as libc::c_ulong,
                                ),
                        ) as *mut XFixed;
                        if kern_dst.is_null() {
                            fprintf(
                                stderr,
                                b"%s(): Failed to allocate memory for blur kernel.\n\0"
                                    as *const u8 as *const libc::c_char,
                                (*::std::mem::transmute::<
                                    &[u8; 20],
                                    &[libc::c_char; 20],
                                >(b"win_blur_background\0"))
                                    .as_ptr(),
                            );
                            return;
                        }
                        (*ps).blur_kerns_cache[i as usize] = kern_dst;
                    }
                    *kern_src
                        .offset(
                            (2 as libc::c_int + khei / 2 as libc::c_int * kwid
                                + kwid / 2 as libc::c_int) as isize,
                        ) = (factor_center * 65536 as libc::c_int as libc::c_double)
                        as XFixed;
                    memcpy(
                        kern_dst as *mut libc::c_void,
                        kern_src as *const libc::c_void,
                        ((kwid * khei + 2 as libc::c_int) as libc::c_ulong)
                            .wrapping_mul(
                                ::std::mem::size_of::<XFixed>() as libc::c_ulong,
                            ),
                    );
                    normalize_conv_kern(
                        kwid,
                        khei,
                        kern_dst.offset(2 as libc::c_int as isize),
                    );
                }
                i += 1;
                i;
            }
            let mut reg_noframe: XserverRegion = 0 as libc::c_long as XserverRegion;
            if win_is_solid(ps, w) {
                let mut reg_all: XserverRegion = border_size(
                    ps,
                    w,
                    0 as libc::c_int != 0,
                );
                reg_noframe = win_get_region_noframe(ps, w, 0 as libc::c_int != 0);
                XFixesSubtractRegion((*ps).dpy, reg_noframe, reg_all, reg_noframe);
                free_region(ps, &mut reg_all);
            }
            xr_blur_dst(
                ps,
                tgt_buffer,
                x,
                y,
                wid,
                hei,
                ((*ps).blur_kerns_cache).as_mut_ptr(),
                reg_noframe,
            );
            free_region(ps, &mut reg_noframe);
        }
        _ => {}
    };
}
unsafe extern "C" fn render_(
    mut ps: *mut session_t,
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut dx: libc::c_int,
    mut dy: libc::c_int,
    mut wid: libc::c_int,
    mut hei: libc::c_int,
    mut opacity: libc::c_double,
    mut argb: bool,
    mut neg: bool,
    mut pict: Picture,
    mut ptex: *mut glx_texture_t,
    mut reg_paint: XserverRegion,
    mut pcache_reg: *const reg_data_t,
) {
    match (*ps).o.backend as libc::c_uint {
        0 | 2 => {
            let mut alpha_pict: Picture = get_alpha_pict_d(ps, opacity);
            if alpha_pict != *((*ps).alpha_picts).offset(0 as libc::c_int as isize) {
                let mut op: libc::c_int = if !argb && alpha_pict == 0 {
                    1 as libc::c_int
                } else {
                    3 as libc::c_int
                };
                XRenderComposite(
                    (*ps).dpy,
                    op,
                    pict,
                    alpha_pict,
                    (*ps).tgt_buffer.pict,
                    x,
                    y,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    dx,
                    dy,
                    wid as libc::c_uint,
                    hei as libc::c_uint,
                );
            }
        }
        _ => {}
    };
}
#[inline]
unsafe extern "C" fn win_paint_win(
    mut ps: *mut session_t,
    mut w: *mut win,
    mut reg_paint: XserverRegion,
    mut pcache_reg: *const reg_data_t,
) {
    glx_mark_(
        ps,
        (*::std::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"win_paint_win\0"))
            .as_ptr(),
        (*w).id,
        1 as libc::c_int != 0,
    );
    if (*w).paint.pixmap == 0 && (*ps).has_name_pixmap as libc::c_int != 0 {
        set_ignore_next(ps);
        (*w).paint.pixmap = XCompositeNameWindowPixmap((*ps).dpy, (*w).id);
        if (*w).paint.pixmap != 0 {
            free_fence(ps, &mut (*w).fence);
        }
    }
    let mut draw: Drawable = (*w).paint.pixmap;
    if draw == 0 {
        draw = (*w).id;
    }
    if bkend_use_xrender(ps) as libc::c_int != 0 && (*w).paint.pict == 0 {
        let mut pa: XRenderPictureAttributes = {
            let mut init = _XRenderPictureAttributes {
                repeat: 0,
                alpha_map: 0,
                alpha_x_origin: 0,
                alpha_y_origin: 0,
                clip_x_origin: 0,
                clip_y_origin: 0,
                clip_mask: 0,
                graphics_exposures: 0,
                subwindow_mode: 1 as libc::c_int,
                poly_edge: 0,
                poly_mode: 0,
                dither: 0,
                component_alpha: 0,
            };
            init
        };
        (*w)
            .paint
            .pict = XRenderCreatePicture(
            (*ps).dpy,
            draw,
            (*w).pictfmt,
            ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_ulong,
            &mut pa,
        );
    }
    if 2 as libc::c_int == (*w).a.map_state {
        xr_sync_(ps, draw, &mut (*w).fence);
    }
    if !paint_bind_tex(
        ps,
        &mut (*w).paint,
        0 as libc::c_int as libc::c_uint,
        0 as libc::c_int as libc::c_uint,
        0 as libc::c_int as libc::c_uint,
        !(*ps).o.glx_no_rebind_pixmap && (*w).pixmap_damaged as libc::c_int != 0,
    ) {
        fprintf(
            stderr,
            b"%s(%#010lx): Failed to bind texture. Expect troubles.\n\0" as *const u8
                as *const libc::c_char,
            (*::std::mem::transmute::<
                &[u8; 14],
                &[libc::c_char; 14],
            >(b"win_paint_win\0"))
                .as_ptr(),
            (*w).id,
        );
    }
    (*w).pixmap_damaged = 0 as libc::c_int != 0;
    if !paint_isvalid(ps, &mut (*w).paint) {
        fprintf(
            stderr,
            b"%s(%#010lx): Missing painting data. This is a bad sign.\n\0" as *const u8
                as *const libc::c_char,
            (*::std::mem::transmute::<
                &[u8; 14],
                &[libc::c_char; 14],
            >(b"win_paint_win\0"))
                .as_ptr(),
            (*w).id,
        );
        return;
    }
    let x: libc::c_int = (*w).a.x;
    let y: libc::c_int = (*w).a.y;
    let wid: libc::c_int = (*w).widthb;
    let hei: libc::c_int = (*w).heightb;
    let mut pict: Picture = (*w).paint.pict;
    if bkend_use_xrender(ps) as libc::c_int != 0 && (*w).invert_color as libc::c_int != 0
    {
        let mut newpict: Picture = xr_build_picture(ps, wid, hei, (*w).pictfmt);
        if newpict != 0 {
            if reg_paint != 0 {
                let mut reg: XserverRegion = copy_region(ps, reg_paint);
                XFixesTranslateRegion((*ps).dpy, reg, -x, -y);
                XFixesSetPictureClipRegion(
                    (*ps).dpy,
                    newpict,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    reg,
                );
                free_region(ps, &mut reg);
            }
            XRenderComposite(
                (*ps).dpy,
                1 as libc::c_int,
                pict,
                0 as libc::c_long as Picture,
                newpict,
                0 as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
                wid as libc::c_uint,
                hei as libc::c_uint,
            );
            XRenderComposite(
                (*ps).dpy,
                0x39 as libc::c_int,
                (*ps).white_picture,
                0 as libc::c_long as Picture,
                newpict,
                0 as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
                wid as libc::c_uint,
                hei as libc::c_uint,
            );
            if WMODE_ARGB as libc::c_int as libc::c_uint == (*w).mode as libc::c_uint {
                XRenderComposite(
                    (*ps).dpy,
                    6 as libc::c_int,
                    pict,
                    0 as libc::c_long as Picture,
                    newpict,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    wid as libc::c_uint,
                    hei as libc::c_uint,
                );
            }
            pict = newpict;
        }
    }
    let dopacity: libc::c_double = get_opacity_percent(w);
    if (*w).frame_opacity == 0. {
        win_render(
            ps,
            w,
            0 as libc::c_int,
            0 as libc::c_int,
            wid,
            hei,
            dopacity,
            reg_paint,
            pcache_reg,
            pict,
        );
    } else {
        let extents: margin_t = win_calc_frame_extents(ps, w);
        let t: libc::c_int = extents.top;
        let l: libc::c_int = extents.left;
        let b: libc::c_int = extents.bottom;
        let r: libc::c_int = extents.right;
        let mut phei: libc::c_int = min_i(t, hei);
        if phei > 0 as libc::c_int {
            win_render(
                ps,
                w,
                0 as libc::c_int,
                0 as libc::c_int,
                wid,
                phei,
                (*w).frame_opacity,
                reg_paint,
                pcache_reg,
                pict,
            );
        }
        if hei > t {
            phei = min_i(hei - t, b);
            if phei > 0 as libc::c_int {
                win_render(
                    ps,
                    w,
                    0 as libc::c_int,
                    hei - phei,
                    wid,
                    phei,
                    (*w).frame_opacity,
                    reg_paint,
                    pcache_reg,
                    pict,
                );
            }
            phei = hei - t - phei;
            if phei > 0 as libc::c_int {
                let mut pwid: libc::c_int = min_i(l, wid);
                if pwid > 0 as libc::c_int {
                    win_render(
                        ps,
                        w,
                        0 as libc::c_int,
                        t,
                        pwid,
                        phei,
                        (*w).frame_opacity,
                        reg_paint,
                        pcache_reg,
                        pict,
                    );
                }
                if wid > l {
                    pwid = min_i(wid - l, r);
                    if pwid > 0 as libc::c_int {
                        win_render(
                            ps,
                            w,
                            wid - pwid,
                            t,
                            pwid,
                            phei,
                            (*w).frame_opacity,
                            reg_paint,
                            pcache_reg,
                            pict,
                        );
                    }
                    pwid = wid - l - pwid;
                    if pwid > 0 as libc::c_int {
                        win_render(
                            ps,
                            w,
                            l,
                            t,
                            pwid,
                            phei,
                            dopacity,
                            reg_paint,
                            pcache_reg,
                            pict,
                        );
                    }
                }
            }
        }
    }
    if pict != (*w).paint.pict {
        free_picture(ps, &mut pict);
    }
    if (*w).dim {
        let mut dim_opacity: libc::c_double = (*ps).o.inactive_dim;
        if !(*ps).o.inactive_dim_fixed {
            dim_opacity *= get_opacity_percent(w);
        }
        match (*ps).o.backend as libc::c_uint {
            0 | 2 => {
                let mut cval: libc::c_ushort = (0xffff as libc::c_int as libc::c_double
                    * dim_opacity) as libc::c_ushort;
                let mut color: XRenderColor = {
                    let mut init = XRenderColor {
                        red: 0 as libc::c_int as libc::c_ushort,
                        green: 0 as libc::c_int as libc::c_ushort,
                        blue: 0 as libc::c_int as libc::c_ushort,
                        alpha: cval,
                    };
                    init
                };
                let mut rect: XRectangle = {
                    let mut init = XRectangle {
                        x: x as libc::c_short,
                        y: y as libc::c_short,
                        width: wid as libc::c_ushort,
                        height: hei as libc::c_ushort,
                    };
                    init
                };
                XRenderFillRectangles(
                    (*ps).dpy,
                    3 as libc::c_int,
                    (*ps).tgt_buffer.pict,
                    &mut color,
                    &mut rect,
                    1 as libc::c_int,
                );
            }
            _ => {}
        }
    }
    glx_mark_(
        ps,
        (*::std::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"win_paint_win\0"))
            .as_ptr(),
        (*w).id,
        0 as libc::c_int != 0,
    );
}
unsafe extern "C" fn rebuild_screen_reg(mut ps: *mut session_t) {
    if (*ps).screen_reg != 0 {
        XFixesDestroyRegion((*ps).dpy, (*ps).screen_reg);
    }
    (*ps).screen_reg = get_screen_region(ps);
}
unsafe extern "C" fn rebuild_shadow_exclude_reg(mut ps: *mut session_t) {
    free_region(ps, &mut (*ps).shadow_exclude_reg);
    let mut rect: XRectangle = geom_to_rect(
        ps,
        &mut (*ps).o.shadow_exclude_reg_geom,
        0 as *const XRectangle,
    );
    (*ps).shadow_exclude_reg = rect_to_reg(ps, &mut rect);
}
unsafe extern "C" fn paint_all(
    mut ps: *mut session_t,
    mut region: XserverRegion,
    mut region_real: XserverRegion,
    mut t: *mut win,
) {
    if region_real == 0 {
        region_real = region;
    }
    let mut reg_paint: XserverRegion = 0 as libc::c_long as XserverRegion;
    let mut reg_tmp: XserverRegion = 0 as libc::c_long as XserverRegion;
    let mut reg_tmp2: XserverRegion = 0 as libc::c_long as XserverRegion;
    if region == 0 {
        region = get_screen_region(ps);
        region_real = region;
    } else {
        XFixesIntersectRegion((*ps).dpy, region, region, (*ps).screen_reg);
    }
    if !paint_isvalid(ps, &mut (*ps).tgt_buffer) {
        if BKEND_XRENDER as libc::c_int as libc::c_uint
            == (*ps).o.backend as libc::c_uint && (*ps).o.dbe as libc::c_int != 0
        {
            (*ps)
                .tgt_buffer
                .pict = XRenderCreatePicture(
                (*ps).dpy,
                (*ps).root_dbe,
                XRenderFindVisualFormat((*ps).dpy, (*ps).vis),
                0 as libc::c_int as libc::c_ulong,
                0 as *const XRenderPictureAttributes,
            );
        } else {
            if (*ps).tgt_buffer.pixmap == 0 {
                free_paint(ps, &mut (*ps).tgt_buffer);
                (*ps)
                    .tgt_buffer
                    .pixmap = XCreatePixmap(
                    (*ps).dpy,
                    (*ps).root,
                    (*ps).root_width as libc::c_uint,
                    (*ps).root_height as libc::c_uint,
                    (*ps).depth as libc::c_uint,
                );
            }
            if BKEND_GLX as libc::c_int as libc::c_uint
                != (*ps).o.backend as libc::c_uint
            {
                (*ps)
                    .tgt_buffer
                    .pict = XRenderCreatePicture(
                    (*ps).dpy,
                    (*ps).tgt_buffer.pixmap,
                    XRenderFindVisualFormat((*ps).dpy, (*ps).vis),
                    0 as libc::c_int as libc::c_ulong,
                    0 as *const XRenderPictureAttributes,
                );
            }
        }
    }
    if BKEND_XRENDER as libc::c_int as libc::c_uint == (*ps).o.backend as libc::c_uint {
        XFixesSetPictureClipRegion(
            (*ps).dpy,
            (*ps).tgt_picture,
            0 as libc::c_int,
            0 as libc::c_int,
            region_real,
        );
    }
    if !t.is_null() && (*t).reg_ignore != 0 {
        reg_tmp = XFixesCreateRegion((*ps).dpy, 0 as *mut XRectangle, 0 as libc::c_int);
        reg_paint = reg_tmp;
        XFixesSubtractRegion((*ps).dpy, reg_paint, region, (*t).reg_ignore);
    } else {
        reg_paint = region;
    }
    set_tgt_clip(ps, reg_paint, 0 as *const reg_data_t);
    paint_root(ps, reg_paint);
    if reg_tmp == 0 {
        reg_tmp = XFixesCreateRegion((*ps).dpy, 0 as *mut XRectangle, 0 as libc::c_int);
    }
    reg_tmp2 = XFixesCreateRegion((*ps).dpy, 0 as *mut XRectangle, 0 as libc::c_int);
    let mut w: *mut win = t;
    while !w.is_null() {
        if (*w).shadow {
            if (*w).shadow_paint.pixmap == 0 {
                win_build_shadow(ps, w, 1 as libc::c_int as libc::c_double);
            }
            if (*w).reg_ignore != 0 {
                if w == t {
                    reg_paint = reg_tmp;
                    XFixesSubtractRegion((*ps).dpy, reg_paint, region, (*w).reg_ignore);
                } else {
                    reg_paint = reg_tmp2;
                }
                XFixesIntersectRegion((*ps).dpy, reg_paint, reg_paint, (*w).extents);
            } else {
                reg_paint = reg_tmp;
                XFixesIntersectRegion((*ps).dpy, reg_paint, region, (*w).extents);
            }
            if (*ps).shadow_exclude_reg != 0 {
                XFixesSubtractRegion(
                    (*ps).dpy,
                    reg_paint,
                    reg_paint,
                    (*ps).shadow_exclude_reg,
                );
            }
            let mut rec_shadow_border: XRectangle = {
                let mut init = XRectangle {
                    x: ((*w).a.x + (*w).shadow_dx) as libc::c_short,
                    y: ((*w).a.y + (*w).shadow_dy) as libc::c_short,
                    width: (*w).shadow_width as libc::c_ushort,
                    height: (*w).shadow_height as libc::c_ushort,
                };
                init
            };
            let mut reg_shadow: XserverRegion = XFixesCreateRegion(
                (*ps).dpy,
                &mut rec_shadow_border,
                1 as libc::c_int,
            );
            XFixesIntersectRegion((*ps).dpy, reg_paint, reg_paint, reg_shadow);
            free_region(ps, &mut reg_shadow);
            if (*ps).o.clear_shadow as libc::c_int != 0 && (*w).border_size != 0 {
                XFixesSubtractRegion((*ps).dpy, reg_paint, reg_paint, (*w).border_size);
            }
            let mut cache_reg: reg_data_t = {
                let mut init = reg_data_t {
                    rects: 0 as *mut XRectangle,
                    nrects: 0 as libc::c_int,
                };
                init
            };
            if region == reg_paint || !is_region_empty(ps, reg_paint, &mut cache_reg) {
                set_tgt_clip(ps, reg_paint, &mut cache_reg);
                win_paint_shadow(ps, w, reg_paint, &mut cache_reg);
            }
            free_reg_data(&mut cache_reg);
        }
        reg_paint = reg_tmp;
        if !((*w).prev_trans).is_null() && (*(*w).prev_trans).reg_ignore != 0 {
            XFixesSubtractRegion(
                (*ps).dpy,
                reg_paint,
                region,
                (*(*w).prev_trans).reg_ignore,
            );
            XFixesCopyRegion((*ps).dpy, reg_tmp2, reg_paint);
            if (*w).border_size != 0 {
                XFixesIntersectRegion((*ps).dpy, reg_paint, reg_paint, (*w).border_size);
            }
        } else if (*w).border_size != 0 {
            XFixesIntersectRegion((*ps).dpy, reg_paint, region, (*w).border_size);
        } else {
            reg_paint = region;
        }
        let mut cache_reg_0: reg_data_t = {
            let mut init = reg_data_t {
                rects: 0 as *mut XRectangle,
                nrects: 0 as libc::c_int,
            };
            init
        };
        if !is_region_empty(ps, reg_paint, &mut cache_reg_0) {
            set_tgt_clip(ps, reg_paint, &mut cache_reg_0);
            if (*w).blur_background as libc::c_int != 0
                && (!win_is_solid(ps, w)
                    || (*ps).o.blur_background_frame as libc::c_int != 0
                        && (*w).frame_opacity != 0.)
            {
                win_blur_background(
                    ps,
                    w,
                    (*ps).tgt_buffer.pict,
                    reg_paint,
                    &mut cache_reg_0,
                );
            }
            win_paint_win(ps, w, reg_paint, &mut cache_reg_0);
        }
        free_reg_data(&mut cache_reg_0);
        w = (*w).prev_trans;
    }
    XFixesDestroyRegion((*ps).dpy, reg_tmp);
    XFixesDestroyRegion((*ps).dpy, reg_tmp2);
    if !(*ps).o.dbe {
        set_tgt_clip(ps, 0 as libc::c_long as XserverRegion, 0 as *const reg_data_t);
    }
    if (*ps).o.vsync as u64 != 0 {
        XSync((*ps).dpy, 0 as libc::c_int);
    }
    if !(*ps).o.vsync_aggressive {
        vsync_wait(ps);
    }
    match (*ps).o.backend as libc::c_uint {
        0 => {
            if (*ps).o.dbe {
                let mut swap_info: XdbeSwapInfo = {
                    let mut init = XdbeSwapInfo {
                        swap_window: get_tgt_window(ps),
                        swap_action: 3 as libc::c_int as XdbeSwapAction,
                    };
                    init
                };
                XdbeSwapBuffers((*ps).dpy, &mut swap_info, 1 as libc::c_int);
            } else if (*ps).tgt_buffer.pict != (*ps).tgt_picture {
                XRenderComposite(
                    (*ps).dpy,
                    1 as libc::c_int,
                    (*ps).tgt_buffer.pict,
                    0 as libc::c_long as Picture,
                    (*ps).tgt_picture,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    (*ps).root_width as libc::c_uint,
                    (*ps).root_height as libc::c_uint,
                );
            }
        }
        _ => {}
    }
    glx_mark_frame(ps);
    if (*ps).o.vsync_aggressive {
        vsync_wait(ps);
    }
    XFlush((*ps).dpy);
    XFixesDestroyRegion((*ps).dpy, region);
    let mut pprev: *mut win = 0 as *mut win;
    let mut w_0: *mut win = t;
    while !w_0.is_null() {
        pprev = (*w_0).prev_trans;
        check_fade_fin(ps, w_0);
        w_0 = pprev;
    }
}
unsafe extern "C" fn add_damage(mut ps: *mut session_t, mut damage: XserverRegion) {
    if !(*ps).redirected {
        free_region(ps, &mut damage);
    }
    if damage == 0 {
        return;
    }
    if (*ps).all_damage != 0 {
        XFixesUnionRegion((*ps).dpy, (*ps).all_damage, (*ps).all_damage, damage);
        XFixesDestroyRegion((*ps).dpy, damage);
    } else {
        (*ps).all_damage = damage;
    };
}
unsafe extern "C" fn repair_win(mut ps: *mut session_t, mut w: *mut win) {
    if 2 as libc::c_int != (*w).a.map_state {
        return;
    }
    let mut parts: XserverRegion = 0;
    if !(*w).damaged {
        parts = win_extents(ps, w);
        set_ignore_next(ps);
        XDamageSubtract(
            (*ps).dpy,
            (*w).damage,
            0 as libc::c_long as XserverRegion,
            0 as libc::c_long as XserverRegion,
        );
    } else {
        parts = XFixesCreateRegion((*ps).dpy, 0 as *mut XRectangle, 0 as libc::c_int);
        set_ignore_next(ps);
        XDamageSubtract(
            (*ps).dpy,
            (*w).damage,
            0 as libc::c_long as XserverRegion,
            parts,
        );
        XFixesTranslateRegion(
            (*ps).dpy,
            parts,
            (*w).a.x + (*w).a.border_width,
            (*w).a.y + (*w).a.border_width,
        );
    }
    (*w).damaged = 1 as libc::c_int != 0;
    (*w).pixmap_damaged = 1 as libc::c_int != 0;
    if !(*ps).redirected {
        free_region(ps, &mut parts);
        return;
    }
    if !(*ps).reg_ignore_expire && !((*w).prev_trans).is_null()
        && (*(*w).prev_trans).reg_ignore != 0
    {
        XFixesSubtractRegion((*ps).dpy, parts, parts, (*(*w).prev_trans).reg_ignore);
    }
    add_damage(ps, parts);
}
unsafe extern "C" fn wid_get_prop_wintype(
    mut ps: *mut session_t,
    mut wid: Window,
) -> wintype_t {
    set_ignore_next(ps);
    let mut prop: winprop_t = wid_get_prop(
        ps,
        wid,
        (*ps).atom_win_type,
        32 as libc::c_long,
        4 as libc::c_int as Atom,
        32 as libc::c_int,
    );
    let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong) < prop.nitems {
        let mut j: wintype_t = WINTYPE_DESKTOP;
        while (j as libc::c_uint) < NUM_WINTYPES as libc::c_int as libc::c_uint {
            if (*ps).atoms_wintypes[j as usize]
                == *(prop.data.p32).offset(i as isize) as Atom
            {
                free_winprop(&mut prop);
                return j;
            }
            j += 1;
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    free_winprop(&mut prop);
    return WINTYPE_UNKNOWN;
}
unsafe extern "C" fn map_win(mut ps: *mut session_t, mut id: Window) {
    if (*ps).overlay != 0 && id == (*ps).overlay && !(*ps).redirected {
        XUnmapWindow((*ps).dpy, (*ps).overlay);
        XFlush((*ps).dpy);
    }
    let mut w: *mut win = find_win(ps, id);
    if w.is_null() || 2 as libc::c_int == (*w).a.class
        || 2 as libc::c_int == (*w).a.map_state
    {
        return;
    }
    (*w).a.map_state = 2 as libc::c_int;
    cxinerama_win_upd_scr(ps, w);
    XSelectInput((*ps).dpy, id, determine_evmask(ps, id, WIN_EVMODE_FRAME));
    if (*ps).shape_exists {
        XShapeSelectInput(
            (*ps).dpy,
            id,
            ((1 as libc::c_long) << 0 as libc::c_int) as libc::c_ulong,
        );
    }
    XFlush((*ps).dpy);
    win_determine_mode(ps, w);
    if (*w).client_win == 0 {
        win_recheck_client(ps, w);
    } else {
        win_mark_client(ps, w, (*w).client_win);
    }
    win_update_shape_raw(ps, w);
    if (*ps).o.track_focus {
        recheck_focus(ps);
    }
    win_update_focused(ps, w);
    win_update_opacity_prop(ps, w);
    (*w).flags |= 0x4 as libc::c_int as libc::c_long;
    if (*ps).o.respect_prop_shadow {
        win_update_prop_shadow_raw(ps, w);
    }
    win_determine_shadow(ps, w);
    (*w).in_openclose = 1 as libc::c_int != 0;
    set_fade_callback(
        ps,
        w,
        Some(finish_map_win as unsafe extern "C" fn(*mut session_t, *mut win) -> ()),
        1 as libc::c_int != 0,
    );
    win_determine_fade(ps, w);
    win_determine_blur_background(ps, w);
    (*w).damaged = 0 as libc::c_int != 0;
    if (*w).need_configure {
        configure_win(ps, &mut (*w).queue_configure);
    }
}
unsafe extern "C" fn finish_map_win(mut ps: *mut session_t, mut w: *mut win) {
    (*w).in_openclose = 0 as libc::c_int != 0;
    if (*ps).o.no_fading_openclose {
        win_determine_fade(ps, w);
    }
}
unsafe extern "C" fn finish_unmap_win(mut ps: *mut session_t, mut w: *mut win) {
    (*w).damaged = 0 as libc::c_int != 0;
    (*w).in_openclose = 0 as libc::c_int != 0;
    update_reg_ignore_expire(ps, w);
    if (*w).extents != 0 as libc::c_long as libc::c_ulong {
        add_damage(ps, (*w).extents);
        (*w).extents = 0 as libc::c_long as XserverRegion;
    }
    free_wpaint(ps, w);
    free_region(ps, &mut (*w).border_size);
    free_paint(ps, &mut (*w).shadow_paint);
}
unsafe extern "C" fn unmap_callback(mut ps: *mut session_t, mut w: *mut win) {
    finish_unmap_win(ps, w);
}
unsafe extern "C" fn unmap_win(mut ps: *mut session_t, mut w: *mut win) {
    if w.is_null() || 0 as libc::c_int == (*w).a.map_state {
        return;
    }
    if (*w).paint.pixmap != 0 {
        xr_sync_(ps, (*w).paint.pixmap, &mut (*w).fence);
    }
    free_fence(ps, &mut (*w).fence);
    win_set_focused(ps, w, 0 as libc::c_int != 0);
    (*w).a.map_state = 0 as libc::c_int;
    (*w).flags |= 0x4 as libc::c_int as libc::c_long;
    set_fade_callback(
        ps,
        w,
        Some(unmap_callback as unsafe extern "C" fn(*mut session_t, *mut win) -> ()),
        0 as libc::c_int != 0,
    );
    (*w).in_openclose = 1 as libc::c_int != 0;
    win_determine_fade(ps, w);
    if (*w).fade {
        win_validate_pixmap(ps, w);
    }
    win_ev_stop(ps, w);
}
unsafe extern "C" fn wid_get_opacity_prop(
    mut ps: *mut session_t,
    mut wid: Window,
    mut def: opacity_t,
) -> opacity_t {
    let mut val: opacity_t = def;
    let mut prop: winprop_t = wid_get_prop(
        ps,
        wid,
        (*ps).atom_opacity,
        1 as libc::c_long,
        6 as libc::c_int as Atom,
        32 as libc::c_int,
    );
    if prop.nitems != 0 {
        val = *prop.data.p32 as opacity_t;
    }
    free_winprop(&mut prop);
    return val;
}
unsafe extern "C" fn get_opacity_percent(mut w: *mut win) -> libc::c_double {
    return (*w).opacity as libc::c_double / 0xffffffff as libc::c_uint as libc::c_double;
}
unsafe extern "C" fn win_determine_mode(mut ps: *mut session_t, mut w: *mut win) {
    let mut mode: winmode_t = WMODE_SOLID;
    if !((*w).pictfmt).is_null() && (*(*w).pictfmt).type_0 == 1 as libc::c_int
        && (*(*w).pictfmt).direct.alphaMask as libc::c_int != 0
    {
        mode = WMODE_ARGB;
    } else if (*w).opacity != 0xffffffff as libc::c_uint {
        mode = WMODE_TRANS;
    } else {
        mode = WMODE_SOLID;
    }
    (*w).mode = mode;
}
unsafe extern "C" fn calc_opacity(mut ps: *mut session_t, mut w: *mut win) {
    let mut opacity: opacity_t = 0xffffffff as libc::c_uint;
    if (*w).destroyed as libc::c_int != 0 || 2 as libc::c_int != (*w).a.map_state {
        opacity = 0 as libc::c_int as opacity_t;
    } else {
        opacity = (*w).opacity_prop;
        if 0xffffffff as libc::c_uint == opacity
            && {
                opacity = (*w).opacity_prop_client;
                0xffffffff as libc::c_uint == opacity
            }
        {
            opacity = ((*ps).o.wintype_opacity[(*w).window_type as usize]
                * 0xffffffff as libc::c_uint as libc::c_double) as opacity_t;
        }
        if (*ps).o.inactive_opacity != 0
            && 0 as libc::c_int == (*w).focused as libc::c_int
            && (0xffffffff as libc::c_uint == opacity
                || (*ps).o.inactive_opacity_override as libc::c_int != 0)
        {
            opacity = (*ps).o.inactive_opacity;
        }
        if 0xffffffff as libc::c_uint == opacity && (*ps).o.active_opacity != 0
            && win_is_focused_real(ps, w) as libc::c_int != 0
        {
            opacity = (*ps).o.active_opacity;
        }
    }
    (*w).opacity_tgt = opacity;
}
unsafe extern "C" fn calc_dim(mut ps: *mut session_t, mut w: *mut win) {
    let mut dim: bool = false;
    if (*w).destroyed as libc::c_int != 0 || 2 as libc::c_int != (*w).a.map_state {
        return;
    }
    if (*ps).o.inactive_dim != 0. && !(*w).focused {
        dim = 1 as libc::c_int != 0;
    } else {
        dim = 0 as libc::c_int != 0;
    }
    if dim as libc::c_int != (*w).dim as libc::c_int {
        (*w).dim = dim;
        add_damage_win(ps, w);
    }
}
unsafe extern "C" fn win_determine_fade(mut ps: *mut session_t, mut w: *mut win) {
    if UNSET as libc::c_int as libc::c_uint != (*w).fade_force as libc::c_uint {
        (*w).fade = (*w).fade_force as u64 != 0;
        (*w).fade_last = (*w).fade;
    } else if (*ps).o.no_fading_openclose as libc::c_int != 0
        && (*w).in_openclose as libc::c_int != 0
    {
        (*w).fade = 0 as libc::c_int != 0;
        (*w).fade_last = (*w).fade;
    } else if (*ps).o.no_fading_destroyed_argb as libc::c_int != 0
        && (*w).destroyed as libc::c_int != 0
        && WMODE_ARGB as libc::c_int as libc::c_uint == (*w).mode as libc::c_uint
        && (*w).client_win != 0 && (*w).client_win != (*w).id
    {
        (*w).fade = 0 as libc::c_int != 0;
        (*w).fade_last = (*w).fade;
    } else if !(2 as libc::c_int != (*w).a.map_state) {
        if win_match(ps, w, (*ps).o.fade_blacklist, &mut (*w).cache_fblst) {
            (*w).fade = 0 as libc::c_int != 0;
        } else {
            (*w).fade = (*ps).o.wintype_fade[(*w).window_type as usize];
        }
    }
}
unsafe extern "C" fn win_update_shape_raw(mut ps: *mut session_t, mut w: *mut win) {
    if (*ps).shape_exists {
        (*w).bounding_shaped = wid_bounding_shaped(ps, (*w).id);
        if (*w).bounding_shaped as libc::c_int != 0
            && (*ps).o.detect_rounded_corners as libc::c_int != 0
        {
            win_rounded_corners(ps, w);
        }
    }
}
unsafe extern "C" fn win_update_shape(mut ps: *mut session_t, mut w: *mut win) {
    if (*ps).shape_exists {
        win_update_shape_raw(ps, w);
        win_on_factor_change(ps, w);
    }
}
unsafe extern "C" fn win_update_prop_shadow_raw(
    mut ps: *mut session_t,
    mut w: *mut win,
) {
    let mut prop: winprop_t = wid_get_prop(
        ps,
        (*w).id,
        (*ps).atom_compton_shadow,
        1 as libc::c_int as libc::c_long,
        6 as libc::c_int as Atom,
        32 as libc::c_int,
    );
    if prop.nitems == 0 {
        (*w).prop_shadow = -(1 as libc::c_int) as libc::c_long;
    } else {
        (*w).prop_shadow = *prop.data.p32;
    }
    free_winprop(&mut prop);
}
unsafe extern "C" fn win_update_prop_shadow(mut ps: *mut session_t, mut w: *mut win) {
    let mut attr_shadow_old: libc::c_long = (*w).prop_shadow;
    win_update_prop_shadow_raw(ps, w);
    if (*w).prop_shadow != attr_shadow_old {
        win_determine_shadow(ps, w);
    }
}
unsafe extern "C" fn win_set_shadow(
    mut ps: *mut session_t,
    mut w: *mut win,
    mut shadow_new: bool,
) {
    if (*w).shadow as libc::c_int == shadow_new as libc::c_int {
        return;
    }
    (*w).shadow = shadow_new;
    if (*w).extents != 0 {
        if !(*w).shadow {
            add_damage(ps, (*w).extents);
        } else {
            free_region(ps, &mut (*w).extents);
        }
        (*w).extents = win_extents(ps, w);
        if (*w).shadow {
            add_damage_win(ps, w);
        }
    }
}
unsafe extern "C" fn win_determine_shadow(mut ps: *mut session_t, mut w: *mut win) {
    let mut shadow_new: bool = (*w).shadow;
    if UNSET as libc::c_int as libc::c_uint != (*w).shadow_force as libc::c_uint {
        shadow_new = (*w).shadow_force as u64 != 0;
    } else if 2 as libc::c_int == (*w).a.map_state {
        shadow_new = (*ps).o.wintype_shadow[(*w).window_type as usize] as libc::c_int
            != 0 && !win_match(ps, w, (*ps).o.shadow_blacklist, &mut (*w).cache_sblst)
            && !((*ps).o.shadow_ignore_shaped as libc::c_int != 0
                && (*w).bounding_shaped as libc::c_int != 0 && !(*w).rounded_corners)
            && !((*ps).o.respect_prop_shadow as libc::c_int != 0
                && 0 as libc::c_int as libc::c_long == (*w).prop_shadow);
    }
    win_set_shadow(ps, w, shadow_new);
}
unsafe extern "C" fn win_set_invert_color(
    mut ps: *mut session_t,
    mut w: *mut win,
    mut invert_color_new: bool,
) {
    if (*w).invert_color as libc::c_int == invert_color_new as libc::c_int {
        return;
    }
    (*w).invert_color = invert_color_new;
    add_damage_win(ps, w);
}
unsafe extern "C" fn win_determine_invert_color(
    mut ps: *mut session_t,
    mut w: *mut win,
) {
    let mut invert_color_new: bool = (*w).invert_color;
    if UNSET as libc::c_int as libc::c_uint != (*w).invert_color_force as libc::c_uint {
        invert_color_new = (*w).invert_color_force as u64 != 0;
    } else if 2 as libc::c_int == (*w).a.map_state {
        invert_color_new = win_match(
            ps,
            w,
            (*ps).o.invert_color_list,
            &mut (*w).cache_ivclst,
        );
    }
    win_set_invert_color(ps, w, invert_color_new);
}
unsafe extern "C" fn win_set_blur_background(
    mut ps: *mut session_t,
    mut w: *mut win,
    mut blur_background_new: bool,
) {
    if (*w).blur_background as libc::c_int == blur_background_new as libc::c_int {
        return;
    }
    (*w).blur_background = blur_background_new;
    if !win_is_solid(ps, w)
        || (*ps).o.blur_background_frame as libc::c_int != 0 && (*w).frame_opacity != 0.
    {
        add_damage_win(ps, w);
    }
}
unsafe extern "C" fn win_determine_blur_background(
    mut ps: *mut session_t,
    mut w: *mut win,
) {
    if 2 as libc::c_int != (*w).a.map_state {
        return;
    }
    let mut blur_background_new: bool = (*ps).o.blur_background as libc::c_int != 0
        && !win_match(ps, w, (*ps).o.blur_background_blacklist, &mut (*w).cache_bbblst);
    win_set_blur_background(ps, w, blur_background_new);
}
unsafe extern "C" fn win_update_opacity_rule(mut ps: *mut session_t, mut w: *mut win) {
    if 2 as libc::c_int != (*w).a.map_state {
        return;
    }
    let mut opacity: opacity_t = 0xffffffff as libc::c_uint;
    let mut val: *mut libc::c_void = 0 as *mut libc::c_void;
    if c2_matchd(ps, w, (*ps).o.opacity_rules, &mut (*w).cache_oparule, &mut val) {
        opacity = (val as libc::c_long as libc::c_double / 100.0f64
            * 0xffffffff as libc::c_uint as libc::c_double) as opacity_t;
    }
    if opacity == (*w).opacity_set {
        return;
    }
    if 0xffffffff as libc::c_uint != opacity {
        wid_set_opacity_prop(ps, (*w).id, opacity);
    } else if 0xffffffff as libc::c_uint != (*w).opacity_set {
        wid_rm_opacity_prop(ps, (*w).id);
    }
    (*w).opacity_set = opacity;
}
unsafe extern "C" fn win_on_wtype_change(mut ps: *mut session_t, mut w: *mut win) {
    win_determine_shadow(ps, w);
    win_determine_fade(ps, w);
    win_update_focused(ps, w);
    if !((*ps).o.invert_color_list).is_null() {
        win_determine_invert_color(ps, w);
    }
    if !((*ps).o.opacity_rules).is_null() {
        win_update_opacity_rule(ps, w);
    }
}
unsafe extern "C" fn win_on_factor_change(mut ps: *mut session_t, mut w: *mut win) {
    if !((*ps).o.shadow_blacklist).is_null() {
        win_determine_shadow(ps, w);
    }
    if !((*ps).o.fade_blacklist).is_null() {
        win_determine_fade(ps, w);
    }
    if !((*ps).o.invert_color_list).is_null() {
        win_determine_invert_color(ps, w);
    }
    if !((*ps).o.focus_blacklist).is_null() {
        win_update_focused(ps, w);
    }
    if !((*ps).o.blur_background_blacklist).is_null() {
        win_determine_blur_background(ps, w);
    }
    if !((*ps).o.opacity_rules).is_null() {
        win_update_opacity_rule(ps, w);
    }
    if 2 as libc::c_int == (*w).a.map_state && !((*ps).o.paint_blacklist).is_null() {
        (*w)
            .paint_excluded = win_match(
            ps,
            w,
            (*ps).o.paint_blacklist,
            &mut (*w).cache_pblst,
        );
    }
    if 2 as libc::c_int == (*w).a.map_state
        && !((*ps).o.unredir_if_possible_blacklist).is_null()
    {
        (*w)
            .unredir_if_possible_excluded = win_match(
            ps,
            w,
            (*ps).o.unredir_if_possible_blacklist,
            &mut (*w).cache_uipblst,
        );
    }
}
unsafe extern "C" fn calc_win_size(mut ps: *mut session_t, mut w: *mut win) {
    (*w).widthb = (*w).a.width + (*w).a.border_width * 2 as libc::c_int;
    (*w).heightb = (*w).a.height + (*w).a.border_width * 2 as libc::c_int;
    calc_shadow_geometry(ps, w);
    (*w).flags |= 0x1 as libc::c_int as libc::c_long;
}
unsafe extern "C" fn calc_shadow_geometry(mut ps: *mut session_t, mut w: *mut win) {
    (*w).shadow_dx = (*ps).o.shadow_offset_x;
    (*w).shadow_dy = (*ps).o.shadow_offset_y;
    (*w).shadow_width = (*w).widthb + (*(*ps).gaussian_map).size;
    (*w).shadow_height = (*w).heightb + (*(*ps).gaussian_map).size;
}
unsafe extern "C" fn win_upd_wintype(mut ps: *mut session_t, mut w: *mut win) {
    let wtype_old: wintype_t = (*w).window_type;
    (*w).window_type = wid_get_prop_wintype(ps, (*w).client_win);
    if WINTYPE_UNKNOWN as libc::c_int as libc::c_uint == (*w).window_type as libc::c_uint
    {
        if (*w).a.override_redirect != 0
            || !wid_has_prop(ps, (*w).client_win, (*ps).atom_transient)
        {
            (*w).window_type = WINTYPE_NORMAL;
        } else {
            (*w).window_type = WINTYPE_DIALOG;
        }
    }
    if (*w).window_type as libc::c_uint != wtype_old as libc::c_uint {
        win_on_wtype_change(ps, w);
    }
}
unsafe extern "C" fn win_mark_client(
    mut ps: *mut session_t,
    mut w: *mut win,
    mut client: Window,
) {
    (*w).client_win = client;
    if 2 as libc::c_int != (*w).a.map_state {
        return;
    }
    XSelectInput((*ps).dpy, client, determine_evmask(ps, client, WIN_EVMODE_CLIENT));
    XFlush((*ps).dpy);
    win_upd_wintype(ps, w);
    if (*ps).o.frame_opacity != 0. {
        get_frame_extents(ps, w, client);
    }
    if (*ps).o.track_leader {
        win_update_leader(ps, w);
    }
    if (*ps).o.track_wdata {
        win_get_name(ps, w);
        win_get_class(ps, w);
        win_get_role(ps, w);
    }
    win_on_factor_change(ps, w);
    win_update_focused(ps, w);
}
unsafe extern "C" fn win_unmark_client(mut ps: *mut session_t, mut w: *mut win) {
    let mut client: Window = (*w).client_win;
    (*w).client_win = 0 as libc::c_long as Window;
    XSelectInput((*ps).dpy, client, determine_evmask(ps, client, WIN_EVMODE_UNKNOWN));
}
unsafe extern "C" fn win_recheck_client(mut ps: *mut session_t, mut w: *mut win) {
    (*w).wmwin = 0 as libc::c_int != 0;
    let mut cw: Window = find_client_win(ps, (*w).id);
    if cw == 0 {
        cw = (*w).id;
        (*w).wmwin = (*w).a.override_redirect == 0;
    }
    if (*w).client_win != 0 && (*w).client_win != cw {
        win_unmark_client(ps, w);
    }
    win_mark_client(ps, w, cw);
}
unsafe extern "C" fn add_win(
    mut ps: *mut session_t,
    mut id: Window,
    mut prev: Window,
) -> bool {
    static mut win_def: win = {
        let mut init = _win {
            next: 0 as *const _win as *mut _win,
            prev_trans: 0 as *const _win as *mut _win,
            id: 0 as libc::c_long as Window,
            a: {
                let mut init = XWindowAttributes {
                    x: 0,
                    y: 0,
                    width: 0,
                    height: 0,
                    border_width: 0,
                    depth: 0,
                    visual: 0 as *const Visual as *mut Visual,
                    root: 0,
                    class: 0,
                    bit_gravity: 0,
                    win_gravity: 0,
                    backing_store: 0,
                    backing_planes: 0,
                    backing_pixel: 0,
                    save_under: 0,
                    colormap: 0,
                    map_installed: 0,
                    map_state: 0,
                    all_event_masks: 0,
                    your_event_mask: 0,
                    do_not_propagate_mask: 0,
                    override_redirect: 0,
                    screen: 0 as *const Screen as *mut Screen,
                };
                init
            },
            pictfmt: 0 as *const XRenderPictFormat as *mut XRenderPictFormat,
            mode: WMODE_TRANS,
            damaged: 0 as libc::c_int != 0,
            fence: 0,
            pixmap_damaged: 0 as libc::c_int != 0,
            damage: 0 as libc::c_long as Damage,
            paint: {
                let mut init = paint_t {
                    pixmap: 0 as libc::c_long as Pixmap,
                    pict: 0 as libc::c_long as Picture,
                    ptex: 0 as *const glx_texture_t as *mut glx_texture_t,
                };
                init
            },
            border_size: 0 as libc::c_long as XserverRegion,
            extents: 0 as libc::c_long as XserverRegion,
            flags: 0 as libc::c_int as int_fast16_t,
            need_configure: 0 as libc::c_int != 0,
            queue_configure: {
                let mut init = XConfigureEvent {
                    type_0: 0,
                    serial: 0,
                    send_event: 0,
                    display: 0 as *const Display as *mut Display,
                    event: 0,
                    window: 0,
                    x: 0,
                    y: 0,
                    width: 0,
                    height: 0,
                    border_width: 0,
                    above: 0,
                    override_redirect: 0,
                };
                init
            },
            reg_ignore: 0 as libc::c_long as XserverRegion,
            widthb: 0 as libc::c_int,
            heightb: 0 as libc::c_int,
            destroyed: 0 as libc::c_int != 0,
            bounding_shaped: 0 as libc::c_int != 0,
            rounded_corners: 0 as libc::c_int != 0,
            to_paint: 0 as libc::c_int != 0,
            paint_excluded: false,
            unredir_if_possible_excluded: false,
            in_openclose: 0 as libc::c_int != 0,
            client_win: 0 as libc::c_long as Window,
            window_type: WINTYPE_UNKNOWN,
            wmwin: 0 as libc::c_int != 0,
            leader: 0 as libc::c_long as Window,
            cache_leader: 0 as libc::c_long as Window,
            focused: 0 as libc::c_int != 0,
            focused_force: UNSET,
            name: 0 as *const libc::c_char as *mut libc::c_char,
            class_instance: 0 as *const libc::c_char as *mut libc::c_char,
            class_general: 0 as *const libc::c_char as *mut libc::c_char,
            role: 0 as *const libc::c_char as *mut libc::c_char,
            cache_sblst: 0 as *const c2_lptr_t,
            cache_fblst: 0 as *const c2_lptr_t,
            cache_fcblst: 0 as *const c2_lptr_t,
            cache_ivclst: 0 as *const c2_lptr_t,
            cache_bbblst: 0 as *const c2_lptr_t,
            cache_oparule: 0 as *const c2_lptr_t,
            cache_pblst: 0 as *const c2_lptr_t,
            cache_uipblst: 0 as *const c2_lptr_t,
            opacity: 0 as libc::c_int as opacity_t,
            opacity_tgt: 0 as libc::c_int as opacity_t,
            opacity_prop: 0xffffffff as libc::c_uint,
            opacity_prop_client: 0xffffffff as libc::c_uint,
            opacity_set: 0xffffffff as libc::c_uint,
            fade: 0 as libc::c_int != 0,
            fade_last: false,
            fade_force: UNSET,
            fade_callback: None,
            frame_opacity: 0.0f64,
            frame_extents: {
                let mut init = margin_t {
                    top: 0 as libc::c_int,
                    left: 0 as libc::c_int,
                    bottom: 0 as libc::c_int,
                    right: 0 as libc::c_int,
                };
                init
            },
            shadow: 0 as libc::c_int != 0,
            shadow_last: false,
            shadow_force: UNSET,
            shadow_opacity: 0.0f64,
            shadow_dx: 0 as libc::c_int,
            shadow_dy: 0 as libc::c_int,
            shadow_width: 0 as libc::c_int,
            shadow_height: 0 as libc::c_int,
            shadow_paint: {
                let mut init = paint_t {
                    pixmap: 0 as libc::c_long as Pixmap,
                    pict: 0 as libc::c_long as Picture,
                    ptex: 0 as *const glx_texture_t as *mut glx_texture_t,
                };
                init
            },
            prop_shadow: -(1 as libc::c_int) as libc::c_long,
            dim: 0 as libc::c_int != 0,
            invert_color: 0 as libc::c_int != 0,
            invert_color_last: false,
            invert_color_force: UNSET,
            blur_background: 0 as libc::c_int != 0,
            blur_background_last: false,
        };
        init
    };
    if id == (*ps).overlay || !(find_win(ps, id)).is_null() {
        return 0 as libc::c_int != 0;
    }
    let mut new: *mut win = malloc(::std::mem::size_of::<win>() as libc::c_ulong)
        as *mut win;
    if new.is_null() {
        fprintf(
            stderr,
            b"%s(%#010lx): Failed to allocate memory for the new window.\n\0"
                as *const u8 as *const libc::c_char,
            (*::std::mem::transmute::<&[u8; 8], &[libc::c_char; 8]>(b"add_win\0"))
                .as_ptr(),
            id,
        );
        return 0 as libc::c_int != 0;
    }
    memcpy(
        new as *mut libc::c_void,
        &win_def as *const win as *const libc::c_void,
        ::std::mem::size_of::<win>() as libc::c_ulong,
    );
    let mut p: *mut *mut win = 0 as *mut *mut win;
    if prev != 0 {
        p = &mut (*ps).list;
        while !(*p).is_null() {
            if (**p).id == prev && !(**p).destroyed {
                break;
            }
            p = &mut (**p).next;
        }
    } else {
        p = &mut (*ps).list;
    }
    (*new).id = id;
    set_ignore_next(ps);
    if XGetWindowAttributes((*ps).dpy, id, &mut (*new).a) == 0
        || 1 as libc::c_int == (*new).a.map_state
    {
        free(new as *mut libc::c_void);
        return 0 as libc::c_int != 0;
    }
    let mut map_state: libc::c_int = (*new).a.map_state;
    (*new).a.map_state = 0 as libc::c_int;
    if 1 as libc::c_int == (*new).a.class {
        (*new).pictfmt = XRenderFindVisualFormat((*ps).dpy, (*new).a.visual);
        set_ignore_next(ps);
        (*new).damage = XDamageCreate((*ps).dpy, id, 3 as libc::c_int);
    }
    calc_win_size(ps, new);
    (*new).next = *p;
    *p = new;
    if 2 as libc::c_int == map_state {
        map_win(ps, id);
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn restack_win(
    mut ps: *mut session_t,
    mut w: *mut win,
    mut new_above: Window,
) {
    let mut old_above: Window = 0;
    update_reg_ignore_expire(ps, w);
    if !((*w).next).is_null() {
        old_above = (*(*w).next).id;
    } else {
        old_above = 0 as libc::c_long as Window;
    }
    if old_above != new_above {
        let mut prev: *mut *mut win = 0 as *mut *mut win;
        let mut prev_old: *mut *mut win = 0 as *mut *mut win;
        prev = &mut (*ps).list;
        while !(*prev).is_null() {
            if *prev == w {
                break;
            }
            prev = &mut (**prev).next;
        }
        prev_old = prev;
        let mut found: bool = 0 as libc::c_int != 0;
        prev = &mut (*ps).list;
        while !(*prev).is_null() {
            if (**prev).id == new_above && !(**prev).destroyed {
                found = 1 as libc::c_int != 0;
                break;
            } else {
                prev = &mut (**prev).next;
            }
        }
        if new_above != 0 && !found {
            fprintf(
                stderr,
                b"%s(%#010lx, %#010lx): Failed to found new above window.\n\0"
                    as *const u8 as *const libc::c_char,
                (*::std::mem::transmute::<
                    &[u8; 12],
                    &[libc::c_char; 12],
                >(b"restack_win\0"))
                    .as_ptr(),
                (*w).id,
                new_above,
            );
            return;
        }
        *prev_old = (*w).next;
        (*w).next = *prev;
        *prev = w;
    }
}
unsafe extern "C" fn configure_win(
    mut ps: *mut session_t,
    mut ce: *mut XConfigureEvent,
) {
    if (*ce).window == (*ps).root {
        free_paint(ps, &mut (*ps).tgt_buffer);
        (*ps).root_width = (*ce).width;
        (*ps).root_height = (*ce).height;
        rebuild_screen_reg(ps);
        rebuild_shadow_exclude_reg(ps);
        free_all_damage_last(ps);
        if (*ps).o.reredir_on_root_change as libc::c_int != 0
            && (*ps).redirected as libc::c_int != 0
        {
            redir_stop(ps);
            redir_start(ps);
        }
        force_repaint(ps);
        return;
    }
    let mut w: *mut win = find_win(ps, (*ce).window);
    let mut damage: XserverRegion = 0 as libc::c_long as XserverRegion;
    if w.is_null() {
        return;
    }
    if (*w).a.map_state == 0 as libc::c_int {
        (*w).need_configure = 1 as libc::c_int != 0;
        (*w).queue_configure = *ce;
        restack_win(ps, w, (*ce).above);
    } else {
        if !(*w).need_configure {
            restack_win(ps, w, (*ce).above);
        }
        let mut factor_change: bool = 0 as libc::c_int != 0;
        (*ps).reg_ignore_expire = 1 as libc::c_int != 0;
        (*w).need_configure = 0 as libc::c_int != 0;
        damage = XFixesCreateRegion((*ps).dpy, 0 as *mut XRectangle, 0 as libc::c_int);
        if (*w).extents != 0 as libc::c_long as libc::c_ulong {
            XFixesCopyRegion((*ps).dpy, damage, (*w).extents);
        }
        if (*w).a.x != (*ce).x || (*w).a.y != (*ce).y || (*w).a.width != (*ce).width
            || (*w).a.height != (*ce).height || (*w).a.border_width != (*ce).border_width
        {
            factor_change = 1 as libc::c_int != 0;
            free_region(ps, &mut (*w).extents);
            free_region(ps, &mut (*w).border_size);
        }
        (*w).a.x = (*ce).x;
        (*w).a.y = (*ce).y;
        if (*w).a.width != (*ce).width || (*w).a.height != (*ce).height
            || (*w).a.border_width != (*ce).border_width
        {
            free_wpaint(ps, w);
        }
        if (*w).a.width != (*ce).width || (*w).a.height != (*ce).height
            || (*w).a.border_width != (*ce).border_width
        {
            (*w).a.width = (*ce).width;
            (*w).a.height = (*ce).height;
            (*w).a.border_width = (*ce).border_width;
            calc_win_size(ps, w);
            if (*ps).shape_exists as libc::c_int != 0
                && (*ps).o.shadow_ignore_shaped as libc::c_int != 0
                && (*ps).o.detect_rounded_corners as libc::c_int != 0
                && (*w).bounding_shaped as libc::c_int != 0
            {
                win_update_shape(ps, w);
            }
        }
        if damage != 0 {
            let mut extents: XserverRegion = win_extents(ps, w);
            XFixesUnionRegion((*ps).dpy, damage, damage, extents);
            XFixesDestroyRegion((*ps).dpy, extents);
            add_damage(ps, damage);
        }
        if factor_change {
            cxinerama_win_upd_scr(ps, w);
            win_on_factor_change(ps, w);
        }
    }
    (*w).a.override_redirect = (*ce).override_redirect;
}
unsafe extern "C" fn circulate_win(
    mut ps: *mut session_t,
    mut ce: *mut XCirculateEvent,
) {
    let mut w: *mut win = find_win(ps, (*ce).window);
    let mut new_above: Window = 0;
    if w.is_null() {
        return;
    }
    if (*ce).place == 0 as libc::c_int {
        new_above = (*(*ps).list).id;
    } else {
        new_above = 0 as libc::c_long as Window;
    }
    restack_win(ps, w, new_above);
}
unsafe extern "C" fn finish_destroy_win(mut ps: *mut session_t, mut id: Window) {
    let mut prev: *mut *mut win = 0 as *mut *mut win;
    let mut w: *mut win = 0 as *mut win;
    prev = &mut (*ps).list;
    loop {
        w = *prev;
        if w.is_null() {
            break;
        }
        if (*w).id == id && (*w).destroyed as libc::c_int != 0 {
            finish_unmap_win(ps, w);
            *prev = (*w).next;
            if w == (*ps).active_win {
                (*ps).active_win = 0 as *mut _win;
            }
            free_win_res(ps, w);
            let mut w2: *mut win = (*ps).list;
            while !w2.is_null() {
                if w == (*w2).prev_trans {
                    (*w2).prev_trans = 0 as *mut _win;
                }
                w2 = (*w2).next;
            }
            free(w as *mut libc::c_void);
            break;
        } else {
            prev = &mut (*w).next;
        }
    };
}
unsafe extern "C" fn destroy_callback(mut ps: *mut session_t, mut w: *mut win) {
    finish_destroy_win(ps, (*w).id);
}
unsafe extern "C" fn destroy_win(mut ps: *mut session_t, mut id: Window) {
    let mut w: *mut win = find_win(ps, id);
    if !w.is_null() {
        unmap_win(ps, w);
        (*w).destroyed = 1 as libc::c_int != 0;
        if (*ps).o.no_fading_destroyed_argb {
            win_determine_fade(ps, w);
        }
        set_fade_callback(
            ps,
            w,
            Some(
                destroy_callback as unsafe extern "C" fn(*mut session_t, *mut win) -> (),
            ),
            0 as libc::c_int != 0,
        );
    }
}
#[inline]
unsafe extern "C" fn root_damaged(mut ps: *mut session_t) {
    if (*ps).root_tile_paint.pixmap != 0 {
        XClearArea(
            (*ps).dpy,
            (*ps).root,
            0 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int as libc::c_uint,
            0 as libc::c_int as libc::c_uint,
            1 as libc::c_int,
        );
        free_root_tile(ps);
    }
    force_repaint(ps);
}
unsafe extern "C" fn damage_win(
    mut ps: *mut session_t,
    mut de: *mut XDamageNotifyEvent,
) {
    let mut w: *mut win = find_win(ps, (*de).drawable);
    if w.is_null() {
        return;
    }
    repair_win(ps, w);
}
unsafe extern "C" fn xerror(
    mut dpy: *mut Display,
    mut ev: *mut XErrorEvent,
) -> libc::c_int {
    let ps: *mut session_t = ps_g;
    let mut o: libc::c_int = 0 as libc::c_int;
    let mut name: *const libc::c_char = b"Unknown\0" as *const u8 as *const libc::c_char;
    if should_ignore(ps, (*ev).serial) != 0 {
        return 0 as libc::c_int;
    }
    if (*ev).request_code as libc::c_int == (*ps).composite_opcode
        && (*ev).minor_code as libc::c_int == 2 as libc::c_int
    {
        fprintf(
            stderr,
            b"Another composite manager is already running\n\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    o = (*ev).error_code as libc::c_int - (*ps).xfixes_error;
    match o {
        0 => {
            name = b"BadRegion\0" as *const u8 as *const libc::c_char;
        }
        _ => {}
    }
    o = (*ev).error_code as libc::c_int - (*ps).damage_error;
    match o {
        0 => {
            name = b"BadDamage\0" as *const u8 as *const libc::c_char;
        }
        _ => {}
    }
    o = (*ev).error_code as libc::c_int - (*ps).render_error;
    match o {
        0 => {
            name = b"BadPictFormat\0" as *const u8 as *const libc::c_char;
        }
        1 => {
            name = b"BadPicture\0" as *const u8 as *const libc::c_char;
        }
        2 => {
            name = b"BadPictOp\0" as *const u8 as *const libc::c_char;
        }
        3 => {
            name = b"BadGlyphSet\0" as *const u8 as *const libc::c_char;
        }
        4 => {
            name = b"BadGlyph\0" as *const u8 as *const libc::c_char;
        }
        _ => {}
    }
    if (*ps).xsync_exists {
        o = (*ev).error_code as libc::c_int - (*ps).xsync_error;
        match o {
            0 => {
                name = b"XSyncBadCounter\0" as *const u8 as *const libc::c_char;
            }
            1 => {
                name = b"XSyncBadAlarm\0" as *const u8 as *const libc::c_char;
            }
            2 => {
                name = b"XSyncBadFence\0" as *const u8 as *const libc::c_char;
            }
            _ => {}
        }
    }
    match (*ev).error_code as libc::c_int {
        10 => {
            name = b"BadAccess\0" as *const u8 as *const libc::c_char;
        }
        11 => {
            name = b"BadAlloc\0" as *const u8 as *const libc::c_char;
        }
        5 => {
            name = b"BadAtom\0" as *const u8 as *const libc::c_char;
        }
        12 => {
            name = b"BadColor\0" as *const u8 as *const libc::c_char;
        }
        6 => {
            name = b"BadCursor\0" as *const u8 as *const libc::c_char;
        }
        9 => {
            name = b"BadDrawable\0" as *const u8 as *const libc::c_char;
        }
        7 => {
            name = b"BadFont\0" as *const u8 as *const libc::c_char;
        }
        13 => {
            name = b"BadGC\0" as *const u8 as *const libc::c_char;
        }
        14 => {
            name = b"BadIDChoice\0" as *const u8 as *const libc::c_char;
        }
        17 => {
            name = b"BadImplementation\0" as *const u8 as *const libc::c_char;
        }
        16 => {
            name = b"BadLength\0" as *const u8 as *const libc::c_char;
        }
        8 => {
            name = b"BadMatch\0" as *const u8 as *const libc::c_char;
        }
        15 => {
            name = b"BadName\0" as *const u8 as *const libc::c_char;
        }
        4 => {
            name = b"BadPixmap\0" as *const u8 as *const libc::c_char;
        }
        1 => {
            name = b"BadRequest\0" as *const u8 as *const libc::c_char;
        }
        2 => {
            name = b"BadValue\0" as *const u8 as *const libc::c_char;
        }
        3 => {
            name = b"BadWindow\0" as *const u8 as *const libc::c_char;
        }
        _ => {}
    }
    print_timestamp(ps);
    let mut buf: [libc::c_char; 80] = *::std::mem::transmute::<
        &[u8; 80],
        &mut [libc::c_char; 80],
    >(
        b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    );
    XGetErrorText(
        (*ps).dpy,
        (*ev).error_code as libc::c_int,
        buf.as_mut_ptr(),
        80 as libc::c_int,
    );
    printf(
        b"error %4d %-12s request %4d minor %4d serial %6lu: \"%s\"\n\0" as *const u8
            as *const libc::c_char,
        (*ev).error_code as libc::c_int,
        name,
        (*ev).request_code as libc::c_int,
        (*ev).minor_code as libc::c_int,
        (*ev).serial,
        buf.as_mut_ptr(),
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn expose_root(
    mut ps: *mut session_t,
    mut rects: *mut XRectangle,
    mut nrects: libc::c_int,
) {
    free_all_damage_last(ps);
    let mut region: XserverRegion = XFixesCreateRegion((*ps).dpy, rects, nrects);
    add_damage(ps, region);
}
unsafe extern "C" fn wid_get_prop_window(
    mut ps: *mut session_t,
    mut wid: Window,
    mut aprop: Atom,
) -> Window {
    let mut p: Window = 0 as libc::c_long as Window;
    let mut prop: winprop_t = wid_get_prop(
        ps,
        wid,
        aprop,
        1 as libc::c_long,
        33 as libc::c_int as Atom,
        32 as libc::c_int,
    );
    if prop.nitems != 0 {
        p = *prop.data.p32 as Window;
    }
    free_winprop(&mut prop);
    return p;
}
unsafe extern "C" fn win_update_focused(mut ps: *mut session_t, mut w: *mut win) {
    let mut focused_old: bool = (*w).focused;
    if UNSET as libc::c_int as libc::c_uint != (*w).focused_force as libc::c_uint {
        (*w).focused = (*w).focused_force as u64 != 0;
    } else {
        (*w).focused = win_is_focused_real(ps, w);
        if (*ps).o.wintype_focus[(*w).window_type as usize] as libc::c_int != 0
            || (*ps).o.mark_wmwin_focused as libc::c_int != 0
                && (*w).wmwin as libc::c_int != 0
            || (*ps).o.mark_ovredir_focused as libc::c_int != 0
                && (*w).id == (*w).client_win && !(*w).wmwin
            || 2 as libc::c_int == (*w).a.map_state
                && win_match(ps, w, (*ps).o.focus_blacklist, &mut (*w).cache_fcblst)
                    as libc::c_int != 0
        {
            (*w).focused = 1 as libc::c_int != 0;
        }
        if (*ps).o.track_leader as libc::c_int != 0 && (*ps).active_leader != 0
            && win_get_leader(ps, w) == (*ps).active_leader
        {
            (*w).focused = 1 as libc::c_int != 0;
        }
    }
    (*w).flags |= 0x4 as libc::c_int as libc::c_long;
}
unsafe extern "C" fn win_set_focused(
    mut ps: *mut session_t,
    mut w: *mut win,
    mut focused: bool,
) {
    if 0 as libc::c_int == (*w).a.map_state {
        return;
    }
    if win_is_focused_real(ps, w) as libc::c_int == focused as libc::c_int {
        return;
    }
    if focused {
        if !((*ps).active_win).is_null() {
            win_set_focused(ps, (*ps).active_win, 0 as libc::c_int != 0);
        }
        (*ps).active_win = w;
    } else if w == (*ps).active_win {
        (*ps).active_win = 0 as *mut _win;
    }
    win_on_focus_change(ps, w);
}
unsafe extern "C" fn win_on_focus_change(mut ps: *mut session_t, mut w: *mut win) {
    if (*ps).o.track_leader {
        let mut leader: Window = win_get_leader(ps, w);
        if win_is_focused_real(ps, w) as libc::c_int != 0
            && leader != (*ps).active_leader
        {
            let mut active_leader_old: Window = (*ps).active_leader;
            (*ps).active_leader = leader;
            group_update_focused(ps, active_leader_old);
            group_update_focused(ps, leader);
        } else if !win_is_focused_real(ps, w) && leader != 0
            && leader == (*ps).active_leader && !group_is_focused(ps, leader)
        {
            (*ps).active_leader = 0 as libc::c_long as Window;
            group_update_focused(ps, leader);
        }
        win_update_focused(ps, w);
    } else {
        win_update_focused(ps, w);
    }
    win_on_factor_change(ps, w);
}
unsafe extern "C" fn win_update_leader(mut ps: *mut session_t, mut w: *mut win) {
    let mut leader: Window = 0 as libc::c_long as Window;
    if (*ps).o.detect_transient as libc::c_int != 0 && leader == 0 {
        leader = wid_get_prop_window(ps, (*w).client_win, (*ps).atom_transient);
    }
    if (*ps).o.detect_client_leader as libc::c_int != 0 && leader == 0 {
        leader = wid_get_prop_window(ps, (*w).client_win, (*ps).atom_client_leader);
    }
    win_set_leader(ps, w, leader);
}
unsafe extern "C" fn win_set_leader(
    mut ps: *mut session_t,
    mut w: *mut win,
    mut nleader: Window,
) {
    if (*w).leader != nleader {
        let mut cache_leader_old: Window = win_get_leader(ps, w);
        (*w).leader = nleader;
        clear_cache_win_leaders(ps);
        let mut cache_leader: Window = win_get_leader(ps, w);
        if win_is_focused_real(ps, w) as libc::c_int != 0
            && cache_leader_old != cache_leader
        {
            (*ps).active_leader = cache_leader;
            group_update_focused(ps, cache_leader_old);
            group_update_focused(ps, cache_leader);
        } else {
            win_update_focused(ps, w);
        }
        win_on_factor_change(ps, w);
    }
}
unsafe extern "C" fn win_get_leader_raw(
    mut ps: *mut session_t,
    mut w: *mut win,
    mut recursions: libc::c_int,
) -> Window {
    if (*w).cache_leader == 0 && ((*w).client_win != 0 || (*w).leader != 0) {
        (*w).cache_leader = (*w).leader;
        if (*w).cache_leader == 0 {
            (*w).cache_leader = (*w).client_win;
        }
        if (*w).cache_leader != 0 && (*w).cache_leader != (*w).client_win {
            let mut wp: *mut win = find_toplevel(ps, (*w).cache_leader);
            if !wp.is_null() {
                if recursions > 20 as libc::c_int {
                    return 0 as libc::c_long as Window;
                }
                (*w)
                    .cache_leader = win_get_leader_raw(
                    ps,
                    wp,
                    recursions + 1 as libc::c_int,
                );
            }
        }
    }
    return (*w).cache_leader;
}
pub unsafe extern "C" fn wid_get_text_prop(
    mut ps: *mut session_t,
    mut wid: Window,
    mut prop: Atom,
    mut pstrlst: *mut *mut *mut libc::c_char,
    mut pnstr: *mut libc::c_int,
) -> bool {
    let mut text_prop: XTextProperty = {
        let mut init = XTextProperty {
            value: 0 as *mut libc::c_uchar,
            encoding: 0 as libc::c_long as Atom,
            format: 0 as libc::c_int,
            nitems: 0 as libc::c_int as libc::c_ulong,
        };
        init
    };
    if !(XGetTextProperty((*ps).dpy, wid, &mut text_prop, prop) != 0
        && !(text_prop.value).is_null())
    {
        return 0 as libc::c_int != 0;
    }
    if 0 as libc::c_int
        != XmbTextPropertyToTextList((*ps).dpy, &mut text_prop, pstrlst, pnstr)
        || *pnstr == 0
    {
        *pnstr = 0 as libc::c_int;
        if !(*pstrlst).is_null() {
            XFreeStringList(*pstrlst);
        }
        cxfree(text_prop.value as *mut libc::c_void);
        return 0 as libc::c_int != 0;
    }
    cxfree(text_prop.value as *mut libc::c_void);
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn wid_get_name(
    mut ps: *mut session_t,
    mut wid: Window,
    mut name: *mut *mut libc::c_char,
) -> bool {
    let mut text_prop: XTextProperty = {
        let mut init = XTextProperty {
            value: 0 as *mut libc::c_uchar,
            encoding: 0 as libc::c_long as Atom,
            format: 0 as libc::c_int,
            nitems: 0 as libc::c_int as libc::c_ulong,
        };
        init
    };
    let mut strlst: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut nstr: libc::c_int = 0 as libc::c_int;
    if !wid_get_text_prop(ps, wid, (*ps).atom_name_ewmh, &mut strlst, &mut nstr) {
        if !(XGetWMName((*ps).dpy, wid, &mut text_prop) != 0
            && !(text_prop.value).is_null())
        {
            return 0 as libc::c_int != 0;
        }
        if 0 as libc::c_int
            != XmbTextPropertyToTextList(
                (*ps).dpy,
                &mut text_prop,
                &mut strlst,
                &mut nstr,
            ) || nstr == 0 || strlst.is_null()
        {
            if !strlst.is_null() {
                XFreeStringList(strlst);
            }
            cxfree(text_prop.value as *mut libc::c_void);
            return 0 as libc::c_int != 0;
        }
        cxfree(text_prop.value as *mut libc::c_void);
    }
    *name = mstrcpy(*strlst.offset(0 as libc::c_int as isize));
    XFreeStringList(strlst);
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn wid_get_role(
    mut ps: *mut session_t,
    mut wid: Window,
    mut role: *mut *mut libc::c_char,
) -> bool {
    let mut strlst: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut nstr: libc::c_int = 0 as libc::c_int;
    if !wid_get_text_prop(ps, wid, (*ps).atom_role, &mut strlst, &mut nstr) {
        return 0 as libc::c_int != 0;
    }
    *role = mstrcpy(*strlst.offset(0 as libc::c_int as isize));
    XFreeStringList(strlst);
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn win_get_prop_str(
    mut ps: *mut session_t,
    mut w: *mut win,
    mut tgt: *mut *mut libc::c_char,
    mut func_wid_get_prop_str: Option::<
        unsafe extern "C" fn(*mut session_t, Window, *mut *mut libc::c_char) -> bool,
    >,
) -> libc::c_int {
    let mut ret: libc::c_int = -(1 as libc::c_int);
    let mut prop_old: *mut libc::c_char = *tgt;
    if (*w).client_win == 0 {
        return 0 as libc::c_int;
    }
    ret = func_wid_get_prop_str.unwrap()(ps, (*w).client_win, tgt) as libc::c_int;
    if ret == 0 {
        ret = -(1 as libc::c_int);
    } else if !prop_old.is_null() && strcmp(*tgt, prop_old) == 0 {
        ret = 0 as libc::c_int;
    } else {
        ret = 1 as libc::c_int;
    }
    if *tgt != prop_old {
        free(prop_old as *mut libc::c_void);
    }
    return ret;
}
unsafe extern "C" fn win_get_class(mut ps: *mut session_t, mut w: *mut win) -> bool {
    let mut strlst: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut nstr: libc::c_int = 0 as libc::c_int;
    if (*w).client_win == 0 {
        return 0 as libc::c_int != 0;
    }
    free((*w).class_instance as *mut libc::c_void);
    free((*w).class_general as *mut libc::c_void);
    (*w).class_instance = 0 as *mut libc::c_char;
    (*w).class_general = 0 as *mut libc::c_char;
    if !wid_get_text_prop(
        ps,
        (*w).client_win,
        (*ps).atom_class,
        &mut strlst,
        &mut nstr,
    ) {
        return 0 as libc::c_int != 0;
    }
    (*w).class_instance = mstrcpy(*strlst.offset(0 as libc::c_int as isize));
    if nstr > 1 as libc::c_int {
        (*w).class_general = mstrcpy(*strlst.offset(1 as libc::c_int as isize));
    }
    XFreeStringList(strlst);
    return 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn force_repaint(mut ps: *mut session_t) {
    let mut reg: XserverRegion = 0 as libc::c_long as XserverRegion;
    if (*ps).screen_reg != 0
        && {
            reg = copy_region(ps, (*ps).screen_reg);
            reg != 0
        }
    {
        (*ps).ev_received = 1 as libc::c_int != 0;
        add_damage(ps, reg);
    }
}
#[inline]
unsafe extern "C" fn ev_focus_in(
    mut ps: *mut session_t,
    mut ev: *mut XFocusChangeEvent,
) {
    recheck_focus(ps);
}
#[inline]
unsafe extern "C" fn ev_focus_out(
    mut ps: *mut session_t,
    mut ev: *mut XFocusChangeEvent,
) {
    recheck_focus(ps);
}
#[inline]
unsafe extern "C" fn ev_create_notify(
    mut ps: *mut session_t,
    mut ev: *mut XCreateWindowEvent,
) {
    add_win(ps, (*ev).window, 0 as libc::c_int as Window);
}
#[inline]
unsafe extern "C" fn ev_configure_notify(
    mut ps: *mut session_t,
    mut ev: *mut XConfigureEvent,
) {
    configure_win(ps, ev);
}
#[inline]
unsafe extern "C" fn ev_destroy_notify(
    mut ps: *mut session_t,
    mut ev: *mut XDestroyWindowEvent,
) {
    destroy_win(ps, (*ev).window);
}
#[inline]
unsafe extern "C" fn ev_map_notify(mut ps: *mut session_t, mut ev: *mut XMapEvent) {
    map_win(ps, (*ev).window);
}
#[inline]
unsafe extern "C" fn ev_unmap_notify(mut ps: *mut session_t, mut ev: *mut XUnmapEvent) {
    let mut w: *mut win = find_win(ps, (*ev).window);
    if !w.is_null() {
        unmap_win(ps, w);
    }
}
#[inline]
unsafe extern "C" fn ev_reparent_notify(
    mut ps: *mut session_t,
    mut ev: *mut XReparentEvent,
) {
    if (*ev).parent == (*ps).root {
        add_win(ps, (*ev).window, 0 as libc::c_int as Window);
    } else {
        destroy_win(ps, (*ev).window);
        XSelectInput(
            (*ps).dpy,
            (*ev).window,
            determine_evmask(ps, (*ev).window, WIN_EVMODE_UNKNOWN),
        );
        if (find_toplevel(ps, (*ev).window)).is_null() {
            let mut w_top: *mut win = find_toplevel2(ps, (*ev).parent);
            if !w_top.is_null()
                && ((*w_top).client_win == 0 || (*w_top).client_win == (*w_top).id)
            {
                if wid_has_prop(ps, (*ev).window, (*ps).atom_client) {
                    (*w_top).wmwin = 0 as libc::c_int != 0;
                    win_unmark_client(ps, w_top);
                    win_mark_client(ps, w_top, (*ev).window);
                } else {
                    XSelectInput(
                        (*ps).dpy,
                        (*ev).window,
                        determine_evmask(ps, (*ev).window, WIN_EVMODE_UNKNOWN)
                            | (1 as libc::c_long) << 22 as libc::c_int,
                    );
                }
            }
        }
    };
}
#[inline]
unsafe extern "C" fn ev_circulate_notify(
    mut ps: *mut session_t,
    mut ev: *mut XCirculateEvent,
) {
    circulate_win(ps, ev);
}
#[inline]
unsafe extern "C" fn ev_expose(mut ps: *mut session_t, mut ev: *mut XExposeEvent) {
    if (*ev).window == (*ps).root || (*ps).overlay != 0 && (*ev).window == (*ps).overlay
    {
        let mut more: libc::c_int = (*ev).count + 1 as libc::c_int;
        if (*ps).n_expose == (*ps).size_expose {
            if !((*ps).expose_rects).is_null() {
                (*ps)
                    .expose_rects = realloc(
                    (*ps).expose_rects as *mut libc::c_void,
                    (((*ps).size_expose + more) as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<XRectangle>() as libc::c_ulong,
                        ),
                ) as *mut XRectangle;
                (*ps).size_expose += more;
            } else {
                (*ps)
                    .expose_rects = malloc(
                    (more as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<XRectangle>() as libc::c_ulong,
                        ),
                ) as *mut XRectangle;
                (*ps).size_expose = more;
            }
        }
        (*((*ps).expose_rects).offset((*ps).n_expose as isize))
            .x = (*ev).x as libc::c_short;
        (*((*ps).expose_rects).offset((*ps).n_expose as isize))
            .y = (*ev).y as libc::c_short;
        (*((*ps).expose_rects).offset((*ps).n_expose as isize))
            .width = (*ev).width as libc::c_ushort;
        (*((*ps).expose_rects).offset((*ps).n_expose as isize))
            .height = (*ev).height as libc::c_ushort;
        (*ps).n_expose += 1;
        (*ps).n_expose;
        if (*ev).count == 0 as libc::c_int {
            expose_root(ps, (*ps).expose_rects, (*ps).n_expose);
            (*ps).n_expose = 0 as libc::c_int;
        }
    }
}
unsafe extern "C" fn update_ewmh_active_win(mut ps: *mut session_t) {
    let mut wid: Window = wid_get_prop_window(
        ps,
        (*ps).root,
        (*ps).atom_ewmh_active_win,
    );
    let mut w: *mut win = find_win_all(ps, wid);
    if !w.is_null() {
        win_set_focused(ps, w, 1 as libc::c_int != 0);
    }
}
#[inline]
unsafe extern "C" fn ev_property_notify(
    mut ps: *mut session_t,
    mut ev: *mut XPropertyEvent,
) {
    if (*ps).root == (*ev).window {
        if (*ps).o.track_focus as libc::c_int != 0
            && (*ps).o.use_ewmh_active_win as libc::c_int != 0
            && (*ps).atom_ewmh_active_win == (*ev).atom
        {
            update_ewmh_active_win(ps);
        } else {
            let mut p: libc::c_int = 0 as libc::c_int;
            while !(background_props_str[p as usize]).is_null() {
                if (*ev).atom == get_atom(ps, background_props_str[p as usize]) {
                    root_damaged(ps);
                    break;
                } else {
                    p += 1;
                    p;
                }
            }
        }
        return;
    }
    if (*ev).atom == (*ps).atom_client {
        if (find_toplevel(ps, (*ev).window)).is_null() {
            XSelectInput(
                (*ps).dpy,
                (*ev).window,
                determine_evmask(ps, (*ev).window, WIN_EVMODE_UNKNOWN),
            );
            let mut w_top: *mut win = find_toplevel2(ps, (*ev).window);
            if !w_top.is_null()
                && ((*w_top).client_win == 0 || (*w_top).client_win == (*w_top).id)
                && wid_has_prop(ps, (*ev).window, (*ps).atom_client) as libc::c_int != 0
            {
                (*w_top).wmwin = 0 as libc::c_int != 0;
                win_unmark_client(ps, w_top);
                win_mark_client(ps, w_top, (*ev).window);
            }
        }
    }
    if (*ev).atom == (*ps).atom_win_type {
        let mut w: *mut win = 0 as *mut win;
        w = find_toplevel(ps, (*ev).window);
        if !w.is_null() {
            win_upd_wintype(ps, w);
        }
    }
    if (*ev).atom == (*ps).atom_opacity {
        let mut w_0: *mut win = 0 as *mut win;
        w_0 = find_win(ps, (*ev).window);
        if !w_0.is_null() {
            (*w_0)
                .opacity_prop = wid_get_opacity_prop(
                ps,
                (*w_0).id,
                0xffffffff as libc::c_uint,
            );
        } else if (*ps).o.detect_client_opacity as libc::c_int != 0
            && {
                w_0 = find_toplevel(ps, (*ev).window);
                !w_0.is_null()
            }
        {
            (*w_0)
                .opacity_prop_client = wid_get_opacity_prop(
                ps,
                (*w_0).client_win,
                0xffffffff as libc::c_uint,
            );
        }
        if !w_0.is_null() {
            (*w_0).flags |= 0x4 as libc::c_int as libc::c_long;
        }
    }
    if (*ps).o.frame_opacity != 0. && (*ev).atom == (*ps).atom_frame_extents {
        let mut w_1: *mut win = find_toplevel(ps, (*ev).window);
        if !w_1.is_null() {
            get_frame_extents(ps, w_1, (*ev).window);
            add_damage_win(ps, w_1);
        }
    }
    if (*ps).o.track_wdata as libc::c_int != 0
        && ((*ps).atom_name == (*ev).atom || (*ps).atom_name_ewmh == (*ev).atom)
    {
        let mut w_2: *mut win = find_toplevel(ps, (*ev).window);
        if !w_2.is_null() && 1 as libc::c_int == win_get_name(ps, w_2) {
            win_on_factor_change(ps, w_2);
        }
    }
    if (*ps).o.track_wdata as libc::c_int != 0 && (*ps).atom_class == (*ev).atom {
        let mut w_3: *mut win = find_toplevel(ps, (*ev).window);
        if !w_3.is_null() {
            win_get_class(ps, w_3);
            win_on_factor_change(ps, w_3);
        }
    }
    if (*ps).o.track_wdata as libc::c_int != 0 && (*ps).atom_role == (*ev).atom {
        let mut w_4: *mut win = find_toplevel(ps, (*ev).window);
        if !w_4.is_null() && 1 as libc::c_int == win_get_role(ps, w_4) {
            win_on_factor_change(ps, w_4);
        }
    }
    if (*ps).o.respect_prop_shadow as libc::c_int != 0
        && (*ps).atom_compton_shadow == (*ev).atom
    {
        let mut w_5: *mut win = find_win(ps, (*ev).window);
        if !w_5.is_null() {
            win_update_prop_shadow(ps, w_5);
        }
    }
    if (*ps).o.detect_transient as libc::c_int != 0 && (*ps).atom_transient == (*ev).atom
        || (*ps).o.detect_client_leader as libc::c_int != 0
            && (*ps).atom_client_leader == (*ev).atom
    {
        let mut w_6: *mut win = find_toplevel(ps, (*ev).window);
        if !w_6.is_null() {
            win_update_leader(ps, w_6);
        }
    }
    let mut platom: *mut latom_t = (*ps).track_atom_lst;
    while !platom.is_null() {
        if (*platom).atom == (*ev).atom {
            let mut w_7: *mut win = find_win(ps, (*ev).window);
            if w_7.is_null() {
                w_7 = find_toplevel(ps, (*ev).window);
            }
            if !w_7.is_null() {
                win_on_factor_change(ps, w_7);
            }
            break;
        } else {
            platom = (*platom).next;
        }
    }
}
#[inline]
unsafe extern "C" fn ev_damage_notify(
    mut ps: *mut session_t,
    mut ev: *mut XDamageNotifyEvent,
) {
    damage_win(ps, ev);
}
#[inline]
unsafe extern "C" fn ev_shape_notify(mut ps: *mut session_t, mut ev: *mut XShapeEvent) {
    let mut w: *mut win = find_win(ps, (*ev).window);
    if w.is_null() || 0 as libc::c_int == (*w).a.map_state {
        return;
    }
    if (*w).border_size != 0 {
        add_damage(ps, (*w).border_size);
        (*w).border_size = border_size(ps, w, 1 as libc::c_int != 0);
        add_damage(ps, copy_region(ps, (*w).border_size));
    }
    win_update_shape(ps, w);
    update_reg_ignore_expire(ps, w);
}
unsafe extern "C" fn ev_screen_change_notify(
    mut ps: *mut session_t,
    mut ev: *mut XRRScreenChangeNotifyEvent,
) {
    if (*ps).o.xinerama_shadow_crop {
        cxinerama_upd_scrs(ps);
    }
    if (*ps).o.sw_opti as libc::c_int != 0 && (*ps).o.refresh_rate == 0 {
        update_refresh_rate(ps);
        if (*ps).refresh_rate == 0 {
            fprintf(
                stderr,
                b"ev_screen_change_notify(): Refresh rate detection failed, --sw-opti disabled.\0"
                    as *const u8 as *const libc::c_char,
            );
            (*ps).o.sw_opti = 0 as libc::c_int != 0;
        }
    }
}
unsafe extern "C" fn ev_handle(mut ps: *mut session_t, mut ev: *mut XEvent) {
    if (*ev).type_0 & 0x7f as libc::c_int != 11 as libc::c_int {
        discard_ignore(ps, (*ev).xany.serial);
    }
    match (*ev).type_0 {
        9 => {
            ev_focus_in(ps, ev as *mut XFocusChangeEvent);
        }
        10 => {
            ev_focus_out(ps, ev as *mut XFocusChangeEvent);
        }
        16 => {
            ev_create_notify(ps, ev as *mut XCreateWindowEvent);
        }
        22 => {
            ev_configure_notify(ps, ev as *mut XConfigureEvent);
        }
        17 => {
            ev_destroy_notify(ps, ev as *mut XDestroyWindowEvent);
        }
        19 => {
            ev_map_notify(ps, ev as *mut XMapEvent);
        }
        18 => {
            ev_unmap_notify(ps, ev as *mut XUnmapEvent);
        }
        21 => {
            ev_reparent_notify(ps, ev as *mut XReparentEvent);
        }
        26 => {
            ev_circulate_notify(ps, ev as *mut XCirculateEvent);
        }
        12 => {
            ev_expose(ps, ev as *mut XExposeEvent);
        }
        28 => {
            ev_property_notify(ps, ev as *mut XPropertyEvent);
        }
        _ => {
            if (*ps).shape_exists as libc::c_int != 0
                && (*ev).type_0 == (*ps).shape_event
            {
                ev_shape_notify(ps, ev as *mut XShapeEvent);
            } else if (*ps).randr_exists as libc::c_int != 0
                && (*ev).type_0 == (*ps).randr_event + 0 as libc::c_int
            {
                ev_screen_change_notify(ps, ev as *mut XRRScreenChangeNotifyEvent);
            } else if isdamagenotify(ps, ev) {
                ev_damage_notify(ps, ev as *mut XDamageNotifyEvent);
            }
        }
    };
}
unsafe extern "C" fn usage(mut ret: libc::c_int) -> ! {
    static mut usage_text: *const libc::c_char = b"compton (git--)\nusage: compton [options]\nOptions:\n\n-d display\n  Which display should be managed.\n\n-r radius\n  The blur radius for shadows. (default 12)\n\n-o opacity\n  The translucency for shadows. (default .75)\n\n-l left-offset\n  The left offset for shadows. (default -15)\n\n-t top-offset\n  The top offset for shadows. (default -15)\n\n-I fade-in-step\n  Opacity change between steps while fading in. (default 0.028)\n\n-O fade-out-step\n  Opacity change between steps while fading out. (default 0.03)\n\n-D fade-delta-time\n  The time between steps in a fade in milliseconds. (default 10)\n\n-m opacity\n  The opacity for menus. (default 1.0)\n\n-c\n  Enabled client-side shadows on windows.\n\n-C\n  Avoid drawing shadows on dock/panel windows.\n\n-z\n  Zero the part of the shadow's mask behind the window.\n\n-f\n  Fade windows in/out when opening/closing and when opacity\n  changes, unless --no-fading-openclose is used.\n\n-F\n  Equals to -f. Deprecated.\n\n-i opacity\n  Opacity of inactive windows. (0.1 - 1.0)\n\n-e opacity\n  Opacity of window titlebars and borders. (0.1 - 1.0)\n\n-G\n  Don't draw shadows on DND windows\n\n-b\n  Daemonize process.\n\n-S\n  Enable synchronous operation (for debugging).\n\n--show-all-xerrors\n  Show all X errors (for debugging).\n\n--config path\n  Look for configuration file at the path. Use /dev/null to avoid\n  loading configuration file. (DISABLED AT COMPILE TIME)\n\n--write-pid-path path\n  Write process ID to a file.\n\n--shadow-red value\n  Red color value of shadow (0.0 - 1.0, defaults to 0).\n\n--shadow-green value\n  Green color value of shadow (0.0 - 1.0, defaults to 0).\n\n--shadow-blue value\n  Blue color value of shadow (0.0 - 1.0, defaults to 0).\n\n--inactive-opacity-override\n  Inactive opacity set by -i overrides value of _NET_WM_OPACITY.\n\n--inactive-dim value\n  Dim inactive windows. (0.0 - 1.0, defaults to 0)\n\n--active-opacity opacity\n  Default opacity for active windows. (0.0 - 1.0)\n\n--mark-wmwin-focused\n  Try to detect WM windows and mark them as active.\n\n--shadow-exclude condition\n  Exclude conditions for shadows.\n\n--fade-exclude condition\n  Exclude conditions for fading.\n\n--mark-ovredir-focused\n  Mark windows that have no WM frame as active.\n\n--no-fading-openclose\n  Do not fade on window open/close.\n\n--no-fading-destroyed-argb\n  Do not fade destroyed ARGB windows with WM frame. Workaround of bugs\n  in Openbox, Fluxbox, etc.\n\n--shadow-ignore-shaped\n  Do not paint shadows on shaped windows. (Deprecated, use\n  --shadow-exclude 'bounding_shaped' or\n  --shadow-exclude 'bounding_shaped && !rounded_corners' instead.)\n\n--detect-rounded-corners\n  Try to detect windows with rounded corners and don't consider\n  them shaped windows. Affects --shadow-ignore-shaped,\n  --unredir-if-possible, and possibly others. You need to turn this\n  on manually if you want to match against rounded_corners in\n  conditions.\n\n--detect-client-opacity\n  Detect _NET_WM_OPACITY on client windows, useful for window\n  managers not passing _NET_WM_OPACITY of client windows to frame\n  windows.\n\n--refresh-rate val\n  Specify refresh rate of the screen. If not specified or 0, compton\n  will try detecting this with X RandR extension.\n\n--vsync vsync-method\n  Set VSync method. There are (up to) 5 VSync methods currently\n  available:\n    none = No VSync\n    drm = VSync with DRM_IOCTL_WAIT_VBLANK. May only work on some\n      (DRI-based) drivers. (DISABLED AT COMPILE TIME)\n    opengl = Try to VSync with SGI_video_sync OpenGL extension. Only\n      work on some drivers. (DISABLED AT COMPILE TIME)\n    opengl-oml = Try to VSync with OML_sync_control OpenGL extension.\n      Only work on some drivers. (DISABLED AT COMPILE TIME)\n    opengl-swc = Try to VSync with SGI_swap_control OpenGL extension.\n      Only work on some drivers. Works only with GLX backend. (DISABLED AT COMPILE TIME)\n    opengl-mswc = Try to VSync with MESA_swap_control OpenGL\n      extension. Basically the same as opengl-swc above, except the\n      extension we use. (DISABLED AT COMPILE TIME)\n\n--vsync-aggressive\n  Attempt to send painting request before VBlank and do XFlush()\n  during VBlank. This switch may be lifted out at any moment.\n\n--alpha-step val\n  X Render backend: Step for pregenerating alpha pictures. \n  0.01 - 1.0. Defaults to 0.03.\n\n--dbe\n  Enable DBE painting mode, intended to use with VSync to\n  (hopefully) eliminate tearing.\n\n--paint-on-overlay\n  Painting on X Composite overlay window.\n\n--sw-opti\n  Limit compton to repaint at most once every 1 / refresh_rate\n  second to boost performance.\n\n--use-ewmh-active-win\n  Use _NET_WM_ACTIVE_WINDOW on the root window to determine which\n  window is focused instead of using FocusIn/Out events.\n\n--respect-prop-shadow\n  Respect _COMPTON_SHADOW. This a prototype-level feature, which\n  you must not rely on.\n\n--unredir-if-possible\n  Unredirect all windows if a full-screen opaque window is\n  detected, to maximize performance for full-screen windows.\n\n--unredir-if-possible-delay ms\n  Delay before unredirecting the window, in milliseconds.\n  Defaults to 0.\n\n--unredir-if-possible-exclude condition\n  Conditions of windows that shouldn't be considered full-screen\n  for unredirecting screen.\n\n--focus-exclude condition\n  Specify a list of conditions of windows that should always be\n  considered focused.\n\n--inactive-dim-fixed\n  Use fixed inactive dim value.\n\n--detect-transient\n  Use WM_TRANSIENT_FOR to group windows, and consider windows in\n  the same group focused at the same time.\n\n--detect-client-leader\n  Use WM_CLIENT_LEADER to group windows, and consider windows in\n  the same group focused at the same time. WM_TRANSIENT_FOR has\n  higher priority if --detect-transient is enabled, too.\n\n--blur-background\n  Blur background of semi-transparent / ARGB windows. Bad in\n  performance. The switch name may change without prior\n  notifications.\n\n--blur-background-frame\n  Blur background of windows when the window frame is not opaque.\n  Implies --blur-background. Bad in performance. The switch name\n  may change.\n\n--blur-background-fixed\n  Use fixed blur strength instead of adjusting according to window\n  opacity.\n\n--blur-kern matrix\n  Specify the blur convolution kernel, with the following format:\n    WIDTH,HEIGHT,ELE1,ELE2,ELE3,ELE4,ELE5...\n  The element in the center must not be included, it will be forever\n  1.0 or changing based on opacity, depending on whether you have\n  --blur-background-fixed.\n  A 7x7 Gaussian blur kernel looks like:\n    --blur-kern '7,7,0.000003,0.000102,0.000849,0.001723,0.000849,0.000102,0.000003,0.000102,0.003494,0.029143,0.059106,0.029143,0.003494,0.000102,0.000849,0.029143,0.243117,0.493069,0.243117,0.029143,0.000849,0.001723,0.059106,0.493069,0.493069,0.059106,0.001723,0.000849,0.029143,0.243117,0.493069,0.243117,0.029143,0.000849,0.000102,0.003494,0.029143,0.059106,0.029143,0.003494,0.000102,0.000003,0.000102,0.000849,0.001723,0.000849,0.000102,0.000003'\n  Up to 4 blur kernels may be specified, separated with semicolon, for\n  multi-pass blur.\n  May also be one the predefined kernels: 3x3box (default), 5x5box,\n  7x7box, 3x3gaussian, 5x5gaussian, 7x7gaussian, 9x9gaussian,\n  11x11gaussian.\n\n--blur-background-exclude condition\n  Exclude conditions for background blur.\n\n--resize-damage integer\n  Resize damaged region by a specific number of pixels. A positive\n  value enlarges it while a negative one shrinks it. Useful for\n  fixing the line corruption issues of blur. May or may not\n  work with --glx-no-stencil. Shrinking doesn't function correctly.\n\n--invert-color-include condition\n  Specify a list of conditions of windows that should be painted with\n  inverted color. Resource-hogging, and is not well tested.\n\n--opacity-rule opacity:condition\n  Specify a list of opacity rules, in the format \"PERCENT:PATTERN\",\n  like '50:name *= \"Firefox\"'. compton-trans is recommended over\n  this. Note we do not distinguish 100% and unset, and we don't make\n  any guarantee about possible conflicts with other programs that set\n  _NET_WM_WINDOW_OPACITY on frame or client windows.\n\n--shadow-exclude-reg geometry\n  Specify a X geometry that describes the region in which shadow\n  should not be painted in, such as a dock window region.\n  Use --shadow-exclude-reg 'x10+0-0', for example, if the 10 pixels\n  on the bottom of the screen should not have shadows painted on.\n\n--xinerama-shadow-crop\n  Crop shadow of a window fully on a particular Xinerama screen to the\n  screen. (DISABLED AT COMPILE TIME)\n\n--backend backend\n  Choose backend. Possible choices are xrender, glx, and\n  xr_glx_hybrid(GLX BACKENDS DISABLED AT COMPILE TIME).\n\n--glx-no-stencil\n  GLX backend: Avoid using stencil buffer. Might cause issues\n  when rendering transparent content. My tests show a 15% performance\n  boost.\n\n--glx-copy-from-front\n  GLX backend: Copy unmodified regions from front buffer instead of\n  redrawing them all. My tests with nvidia-drivers show a 5% decrease\n  in performance when the whole screen is modified, but a 30% increase\n  when only 1/4 is. My tests on nouveau show terrible slowdown. Could\n  work with --glx-swap-method but not --glx-use-copysubbuffermesa.\n\n--glx-use-copysubbuffermesa\n  GLX backend: Use MESA_copy_sub_buffer to do partial screen update.\n  My tests on nouveau shows a 200% performance boost when only 1/4 of\n  the screen is updated. May break VSync and is not available on some\n  drivers. Overrides --glx-copy-from-front.\n\n--glx-no-rebind-pixmap\n  GLX backend: Avoid rebinding pixmap on window damage. Probably\n  could improve performance on rapid window content changes, but is\n  known to break things on some drivers (LLVMpipe, xf86-video-intel,\n  etc.).\n\n--glx-swap-method undefined/copy/exchange/3/4/5/6/buffer-age\n  GLX backend: GLX buffer swap method we assume. Could be\n  undefined (0), copy (1), exchange (2), 3-6, or buffer-age (-1).\n  \"undefined\" is the slowest and the safest, and the default value.\n  1 is fastest, but may fail on some drivers, 2-6 are gradually slower\n  but safer (6 is still faster than 0). -1 means auto-detect using\n  GLX_EXT_buffer_age, supported by some drivers. Useless with\n  --glx-use-copysubbuffermesa.\n\n--glx-use-gpushader4\n  GLX backend: Use GL_EXT_gpu_shader4 for some optimization on blur\n  GLSL code. My tests on GTX 670 show no noticeable effect.\n\n--xrender-sync\n  Attempt to synchronize client applications' draw calls with XSync(),\n  used on GLX backend to ensure up-to-date window content is painted.\n\n--xrender-sync-fence\n  Additionally use X Sync fence to sync clients' draw calls. Needed\n  on nvidia-drivers with GLX backend for some users.\n\n--glx-fshader-win shader\n  GLX backend: Use specified GLSL fragment shader for rendering window\n  contents.\n\n--force-win-blend\n  Force all windows to be painted with blending. Useful if you have a\n  --glx-fshader-win that could turn opaque pixels transparent.\n\n--dbus\n  Enable remote control via D-Bus. See the D-BUS API section in the\n  man page for more details. (DISABLED AT COMPILE TIME)\n\n--benchmark cycles\n  Benchmark mode. Repeatedly paint until reaching the specified cycles.\n\n--benchmark-wid window-id\n  Specify window ID to repaint in benchmark mode. If omitted or is 0,\n  the whole screen is repainted.\n\0"
        as *const u8 as *const libc::c_char;
    let mut f: *mut FILE = if ret != 0 { stderr } else { stdout };
    fputs(usage_text, f);
    exit(ret);
}
unsafe extern "C" fn register_cm(mut ps: *mut session_t) -> bool {
    (*ps)
        .reg_win = XCreateSimpleWindow(
        (*ps).dpy,
        (*ps).root,
        0 as libc::c_int,
        0 as libc::c_int,
        1 as libc::c_int as libc::c_uint,
        1 as libc::c_int as libc::c_uint,
        0 as libc::c_int as libc::c_uint,
        0 as libc::c_long as libc::c_ulong,
        0 as libc::c_long as libc::c_ulong,
    );
    if (*ps).reg_win == 0 {
        fprintf(
            stderr,
            b"%s(): Failed to create window.\n\0" as *const u8 as *const libc::c_char,
            (*::std::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"register_cm\0"))
                .as_ptr(),
        );
        return 0 as libc::c_int != 0;
    }
    if (*ps).redirected {
        XCompositeUnredirectWindow((*ps).dpy, (*ps).reg_win, 1 as libc::c_int);
    }
    let mut h: *mut XClassHint = XAllocClassHint();
    if !h.is_null() {
        (*h)
            .res_name = b"compton\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char;
        (*h)
            .res_class = b"xcompmgr\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char;
    }
    Xutf8SetWMProperties(
        (*ps).dpy,
        (*ps).reg_win,
        b"xcompmgr\0" as *const u8 as *const libc::c_char,
        b"xcompmgr\0" as *const u8 as *const libc::c_char,
        0 as *mut *mut libc::c_char,
        0 as libc::c_int,
        0 as *mut XSizeHints,
        0 as *mut XWMHints,
        h,
    );
    cxfree(h as *mut libc::c_void);
    let mut pid: libc::c_long = getpid() as libc::c_long;
    if XChangeProperty(
        (*ps).dpy,
        (*ps).reg_win,
        get_atom(ps, b"_NET_WM_PID\0" as *const u8 as *const libc::c_char),
        6 as libc::c_int as Atom,
        32 as libc::c_int,
        0 as libc::c_int,
        &mut pid as *mut libc::c_long as *mut libc::c_uchar,
        1 as libc::c_int,
    ) == 0
    {
        fprintf(
            stderr,
            b"%s(): Failed to set _NET_WM_PID.\n\0" as *const u8 as *const libc::c_char,
            (*::std::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"register_cm\0"))
                .as_ptr(),
        );
    }
    if !wid_set_text_prop(
        ps,
        (*ps).reg_win,
        get_atom(ps, b"COMPTON_VERSION\0" as *const u8 as *const libc::c_char),
        b"git--\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) {
        fprintf(
            stderr,
            b"%s(): Failed to set COMPTON_VERSION.\n\0" as *const u8
                as *const libc::c_char,
            (*::std::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"register_cm\0"))
                .as_ptr(),
        );
    }
    if !(*ps).o.no_x_selection {
        let mut len: libc::c_uint = (strlen(
            b"_NET_WM_CM_S\0" as *const u8 as *const libc::c_char,
        ))
            .wrapping_add(2 as libc::c_int as libc::c_ulong) as libc::c_uint;
        let mut s: libc::c_int = (*ps).scr;
        while s >= 10 as libc::c_int {
            len = len.wrapping_add(1);
            len;
            s /= 10 as libc::c_int;
        }
        let mut buf: *mut libc::c_char = malloc(len as libc::c_ulong)
            as *mut libc::c_char;
        snprintf(
            buf,
            len as libc::c_ulong,
            b"_NET_WM_CM_S%d\0" as *const u8 as *const libc::c_char,
            (*ps).scr,
        );
        *buf
            .offset(
                len.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
            ) = '\0' as i32 as libc::c_char;
        XSetSelectionOwner(
            (*ps).dpy,
            get_atom(ps, buf),
            (*ps).reg_win,
            0 as libc::c_int as Time,
        );
        free(buf as *mut libc::c_void);
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn ostream_reopen(
    mut ps: *mut session_t,
    mut path: *const libc::c_char,
) -> bool {
    if path.is_null() {
        path = (*ps).o.logpath;
    }
    if path.is_null() {
        path = b"/dev/null\0" as *const u8 as *const libc::c_char;
    }
    let mut success: bool = !(freopen(
        path,
        b"a\0" as *const u8 as *const libc::c_char,
        stdout,
    ))
        .is_null();
    success = !(freopen(path, b"a\0" as *const u8 as *const libc::c_char, stderr))
        .is_null() && success as libc::c_int != 0;
    if !success {
        fprintf(
            stderr,
            b"%s(%s): freopen() failed.\n\0" as *const u8 as *const libc::c_char,
            (*::std::mem::transmute::<
                &[u8; 15],
                &[libc::c_char; 15],
            >(b"ostream_reopen\0"))
                .as_ptr(),
            path,
        );
        exit(1 as libc::c_int);
    }
    return success;
}
#[inline]
unsafe extern "C" fn fork_after(mut ps: *mut session_t) -> bool {
    if getppid() == 1 as libc::c_int {
        return 1 as libc::c_int != 0;
    }
    let mut pid: libc::c_int = fork();
    if -(1 as libc::c_int) == pid {
        fprintf(
            stderr,
            b"%s(): fork() failed.\n\0" as *const u8 as *const libc::c_char,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"fork_after\0"))
                .as_ptr(),
        );
        return 0 as libc::c_int != 0;
    }
    if pid > 0 as libc::c_int {
        _exit(0 as libc::c_int);
    }
    setsid();
    let mut success: bool = !(freopen(
        b"/dev/null\0" as *const u8 as *const libc::c_char,
        b"r\0" as *const u8 as *const libc::c_char,
        stdin,
    ))
        .is_null();
    if !success {
        fprintf(
            stderr,
            b"%s(): freopen() failed.\n\0" as *const u8 as *const libc::c_char,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"fork_after\0"))
                .as_ptr(),
        );
        return 0 as libc::c_int != 0;
    }
    return success;
}
#[inline]
unsafe extern "C" fn write_pid(mut ps: *mut session_t) -> bool {
    if ((*ps).o.write_pid_path).is_null() {
        return 1 as libc::c_int != 0;
    }
    let mut f: *mut FILE = fopen(
        (*ps).o.write_pid_path,
        b"w\0" as *const u8 as *const libc::c_char,
    );
    if f.is_null() as libc::c_int as libc::c_long != 0 {
        fprintf(
            stderr,
            b"%s(): Failed to write PID to \"%s\".\n\0" as *const u8
                as *const libc::c_char,
            (*::std::mem::transmute::<&[u8; 10], &[libc::c_char; 10]>(b"write_pid\0"))
                .as_ptr(),
            (*ps).o.write_pid_path,
        );
        return 0 as libc::c_int != 0;
    }
    fprintf(f, b"%ld\n\0" as *const u8 as *const libc::c_char, getpid() as libc::c_long);
    fclose(f);
    return 1 as libc::c_int != 0;
}
#[inline]
unsafe extern "C" fn parse_long(
    mut s: *const libc::c_char,
    mut dest: *mut libc::c_long,
) -> bool {
    let mut endptr: *const libc::c_char = 0 as *const libc::c_char;
    let mut val: libc::c_long = strtol(
        s,
        &mut endptr as *mut *const libc::c_char as *mut *mut libc::c_char,
        0 as libc::c_int,
    );
    if endptr.is_null() || endptr == s {
        fprintf(
            stderr,
            b"%s(\"%s\"): Invalid number.\n\0" as *const u8 as *const libc::c_char,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"parse_long\0"))
                .as_ptr(),
            s,
        );
        return 0 as libc::c_int != 0;
    }
    while *(*__ctype_b_loc()).offset(*endptr as libc::c_int as isize) as libc::c_int
        & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        endptr = endptr.offset(1);
        endptr;
    }
    if *endptr != 0 {
        fprintf(
            stderr,
            b"%s(\"%s\"): Trailing characters.\n\0" as *const u8 as *const libc::c_char,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"parse_long\0"))
                .as_ptr(),
            s,
        );
        return 0 as libc::c_int != 0;
    }
    *dest = val;
    return 1 as libc::c_int != 0;
}
#[inline]
unsafe extern "C" fn parse_matrix_readnum(
    mut src: *const libc::c_char,
    mut dest: *mut libc::c_double,
) -> *const libc::c_char {
    let mut pc: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut val: libc::c_double = strtod(src, &mut pc);
    if pc.is_null() || pc == src as *mut libc::c_char {
        fprintf(
            stderr,
            b"%s(\"%s\"): No number found.\n\0" as *const u8 as *const libc::c_char,
            (*::std::mem::transmute::<
                &[u8; 21],
                &[libc::c_char; 21],
            >(b"parse_matrix_readnum\0"))
                .as_ptr(),
            src,
        );
        return src;
    }
    while *pc as libc::c_int != 0
        && (*(*__ctype_b_loc()).offset(*pc as libc::c_int as isize) as libc::c_int
            & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
            || ',' as i32 == *pc as libc::c_int)
    {
        pc = pc.offset(1);
        pc;
    }
    *dest = val;
    return pc;
}
#[inline]
unsafe extern "C" fn parse_matrix(
    mut ps: *mut session_t,
    mut src: *const libc::c_char,
    mut endptr: *mut *const libc::c_char,
) -> *mut XFixed {
    let mut current_block: u64;
    let mut wid: libc::c_int = 0 as libc::c_int;
    let mut hei: libc::c_int = 0 as libc::c_int;
    let mut pc: *const libc::c_char = 0 as *const libc::c_char;
    let mut matrix: *mut XFixed = 0 as *mut XFixed;
    let mut val: libc::c_double = 0.0f64;
    pc = parse_matrix_readnum(src, &mut val);
    if !(src == pc) {
        src = pc;
        wid = val as libc::c_int;
        pc = parse_matrix_readnum(src, &mut val);
        if !(src == pc) {
            src = pc;
            hei = val as libc::c_int;
            if wid <= 0 as libc::c_int || hei <= 0 as libc::c_int {
                fprintf(
                    stderr,
                    b"%s(): Invalid matrix width/height.\n\0" as *const u8
                        as *const libc::c_char,
                    (*::std::mem::transmute::<
                        &[u8; 13],
                        &[libc::c_char; 13],
                    >(b"parse_matrix\0"))
                        .as_ptr(),
                );
            } else if !(wid % 2 as libc::c_int != 0 && hei % 2 as libc::c_int != 0) {
                fprintf(
                    stderr,
                    b"%s(): Width/height not odd.\n\0" as *const u8
                        as *const libc::c_char,
                    (*::std::mem::transmute::<
                        &[u8; 13],
                        &[libc::c_char; 13],
                    >(b"parse_matrix\0"))
                        .as_ptr(),
                );
            } else if wid > 16 as libc::c_int || hei > 16 as libc::c_int {
                fprintf(
                    stderr,
                    b"%s(): Matrix width/height too large.\n\0" as *const u8
                        as *const libc::c_char,
                    (*::std::mem::transmute::<
                        &[u8; 13],
                        &[libc::c_char; 13],
                    >(b"parse_matrix\0"))
                        .as_ptr(),
                );
            } else {
                matrix = calloc(
                    (wid * hei + 2 as libc::c_int) as libc::c_ulong,
                    ::std::mem::size_of::<XFixed>() as libc::c_ulong,
                ) as *mut XFixed;
                if matrix.is_null() {
                    fprintf(
                        stderr,
                        b"%s(): Failed to allocate memory for matrix.\n\0" as *const u8
                            as *const libc::c_char,
                        (*::std::mem::transmute::<
                            &[u8; 13],
                            &[libc::c_char; 13],
                        >(b"parse_matrix\0"))
                            .as_ptr(),
                    );
                } else {
                    let mut skip: libc::c_int = hei / 2 as libc::c_int * wid
                        + wid / 2 as libc::c_int;
                    let mut hasneg: bool = 0 as libc::c_int != 0;
                    let mut i: libc::c_int = 0 as libc::c_int;
                    loop {
                        if !(i < wid * hei) {
                            current_block = 17478428563724192186;
                            break;
                        }
                        if i == skip {
                            *matrix
                                .offset(
                                    (2 as libc::c_int + i) as isize,
                                ) = 0 as libc::c_int * 65536 as libc::c_int;
                        } else {
                            let mut val_0: libc::c_double = 0 as libc::c_int
                                as libc::c_double;
                            pc = parse_matrix_readnum(src, &mut val_0);
                            if src == pc {
                                current_block = 4983819030718313881;
                                break;
                            }
                            src = pc;
                            if val_0 < 0 as libc::c_int as libc::c_double {
                                hasneg = 1 as libc::c_int != 0;
                            }
                            *matrix
                                .offset(
                                    (2 as libc::c_int + i) as isize,
                                ) = (val_0 * 65536 as libc::c_int as libc::c_double)
                                as XFixed;
                        }
                        i += 1;
                        i;
                    }
                    match current_block {
                        4983819030718313881 => {}
                        _ => {
                            if BKEND_XRENDER as libc::c_int as libc::c_uint
                                == (*ps).o.backend as libc::c_uint
                                && hasneg as libc::c_int != 0
                            {
                                fprintf(
                                    stderr,
                                    b"%s(): A convolution kernel with negative values may not work properly under X Render backend.\n\0"
                                        as *const u8 as *const libc::c_char,
                                    (*::std::mem::transmute::<
                                        &[u8; 13],
                                        &[libc::c_char; 13],
                                    >(b"parse_matrix\0"))
                                        .as_ptr(),
                                );
                            }
                            loop {
                                if !(*pc as libc::c_int != 0
                                    && ';' as i32 != *pc as libc::c_int)
                                {
                                    current_block = 7205609094909031804;
                                    break;
                                }
                                if *(*__ctype_b_loc()).offset(*pc as libc::c_int as isize)
                                    as libc::c_int
                                    & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                                    == 0 && ',' as i32 != *pc as libc::c_int
                                {
                                    fprintf(
                                        stderr,
                                        b"%s(): Trailing characters in matrix string.\n\0"
                                            as *const u8 as *const libc::c_char,
                                        (*::std::mem::transmute::<
                                            &[u8; 13],
                                            &[libc::c_char; 13],
                                        >(b"parse_matrix\0"))
                                            .as_ptr(),
                                    );
                                    current_block = 4983819030718313881;
                                    break;
                                } else {
                                    pc = pc.offset(1);
                                    pc;
                                }
                            }
                            match current_block {
                                4983819030718313881 => {}
                                _ => {
                                    if ';' as i32 == *pc as libc::c_int {
                                        pc = pc.offset(1);
                                        pc;
                                        while *pc as libc::c_int != 0
                                            && *(*__ctype_b_loc()).offset(*pc as libc::c_int as isize)
                                                as libc::c_int
                                                & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                                                != 0
                                        {
                                            pc = pc.offset(1);
                                            pc;
                                        }
                                    }
                                    if !endptr.is_null() {
                                        *endptr = pc;
                                        current_block = 14832935472441733737;
                                    } else if *pc != 0 {
                                        fprintf(
                                            stderr,
                                            b"%s(): Only one matrix expected.\n\0" as *const u8
                                                as *const libc::c_char,
                                            (*::std::mem::transmute::<
                                                &[u8; 13],
                                                &[libc::c_char; 13],
                                            >(b"parse_matrix\0"))
                                                .as_ptr(),
                                        );
                                        current_block = 4983819030718313881;
                                    } else {
                                        current_block = 14832935472441733737;
                                    }
                                    match current_block {
                                        4983819030718313881 => {}
                                        _ => {
                                            *matrix
                                                .offset(
                                                    0 as libc::c_int as isize,
                                                ) = wid * 65536 as libc::c_int;
                                            *matrix
                                                .offset(
                                                    1 as libc::c_int as isize,
                                                ) = hei * 65536 as libc::c_int;
                                            return matrix;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    free(matrix as *mut libc::c_void);
    return 0 as *mut XFixed;
}
#[inline]
unsafe extern "C" fn parse_conv_kern(
    mut ps: *mut session_t,
    mut src: *const libc::c_char,
    mut endptr: *mut *const libc::c_char,
) -> *mut XFixed {
    return parse_matrix(ps, src, endptr);
}
unsafe extern "C" fn parse_conv_kern_lst(
    mut ps: *mut session_t,
    mut src: *const libc::c_char,
    mut dest: *mut *mut XFixed,
    mut max: libc::c_int,
) -> bool {
    static mut CONV_KERN_PREDEF: [C2RustUnnamed_15; 8] = [
        {
            let mut init = C2RustUnnamed_15 {
                name: b"3x3box\0" as *const u8 as *const libc::c_char,
                kern_str: b"3,3,1,1,1,1,1,1,1,1,\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_15 {
                name: b"5x5box\0" as *const u8 as *const libc::c_char,
                kern_str: b"5,5,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,\0"
                    as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_15 {
                name: b"7x7box\0" as *const u8 as *const libc::c_char,
                kern_str: b"7,7,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,\0"
                    as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_15 {
                name: b"3x3gaussian\0" as *const u8 as *const libc::c_char,
                kern_str: b"3,3,0.243117,0.493069,0.243117,0.493069,0.493069,0.243117,0.493069,0.243117,\0"
                    as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_15 {
                name: b"5x5gaussian\0" as *const u8 as *const libc::c_char,
                kern_str: b"5,5,0.003493,0.029143,0.059106,0.029143,0.003493,0.029143,0.243117,0.493069,0.243117,0.029143,0.059106,0.493069,0.493069,0.059106,0.029143,0.243117,0.493069,0.243117,0.029143,0.003493,0.029143,0.059106,0.029143,0.003493,\0"
                    as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_15 {
                name: b"7x7gaussian\0" as *const u8 as *const libc::c_char,
                kern_str: b"7,7,0.000003,0.000102,0.000849,0.001723,0.000849,0.000102,0.000003,0.000102,0.003493,0.029143,0.059106,0.029143,0.003493,0.000102,0.000849,0.029143,0.243117,0.493069,0.243117,0.029143,0.000849,0.001723,0.059106,0.493069,0.493069,0.059106,0.001723,0.000849,0.029143,0.243117,0.493069,0.243117,0.029143,0.000849,0.000102,0.003493,0.029143,0.059106,0.029143,0.003493,0.000102,0.000003,0.000102,0.000849,0.001723,0.000849,0.000102,0.000003,\0"
                    as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_15 {
                name: b"9x9gaussian\0" as *const u8 as *const libc::c_char,
                kern_str: b"9,9,0.000000,0.000000,0.000001,0.000006,0.000012,0.000006,0.000001,0.000000,0.000000,0.000000,0.000003,0.000102,0.000849,0.001723,0.000849,0.000102,0.000003,0.000000,0.000001,0.000102,0.003493,0.029143,0.059106,0.029143,0.003493,0.000102,0.000001,0.000006,0.000849,0.029143,0.243117,0.493069,0.243117,0.029143,0.000849,0.000006,0.000012,0.001723,0.059106,0.493069,0.493069,0.059106,0.001723,0.000012,0.000006,0.000849,0.029143,0.243117,0.493069,0.243117,0.029143,0.000849,0.000006,0.000001,0.000102,0.003493,0.029143,0.059106,0.029143,0.003493,0.000102,0.000001,0.000000,0.000003,0.000102,0.000849,0.001723,0.000849,0.000102,0.000003,0.000000,0.000000,0.000000,0.000001,0.000006,0.000012,0.000006,0.000001,0.000000,0.000000,\0"
                    as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_15 {
                name: b"11x11gaussian\0" as *const u8 as *const libc::c_char,
                kern_str: b"11,11,0.000000,0.000000,0.000000,0.000000,0.000000,0.000000,0.000000,0.000000,0.000000,0.000000,0.000000,0.000000,0.000000,0.000000,0.000001,0.000006,0.000012,0.000006,0.000001,0.000000,0.000000,0.000000,0.000000,0.000000,0.000003,0.000102,0.000849,0.001723,0.000849,0.000102,0.000003,0.000000,0.000000,0.000000,0.000001,0.000102,0.003493,0.029143,0.059106,0.029143,0.003493,0.000102,0.000001,0.000000,0.000000,0.000006,0.000849,0.029143,0.243117,0.493069,0.243117,0.029143,0.000849,0.000006,0.000000,0.000000,0.000012,0.001723,0.059106,0.493069,0.493069,0.059106,0.001723,0.000012,0.000000,0.000000,0.000006,0.000849,0.029143,0.243117,0.493069,0.243117,0.029143,0.000849,0.000006,0.000000,0.000000,0.000001,0.000102,0.003493,0.029143,0.059106,0.029143,0.003493,0.000102,0.000001,0.000000,0.000000,0.000000,0.000003,0.000102,0.000849,0.001723,0.000849,0.000102,0.000003,0.000000,0.000000,0.000000,0.000000,0.000000,0.000001,0.000006,0.000012,0.000006,0.000001,0.000000,0.000000,0.000000,0.000000,0.000000,0.000000,0.000000,0.000000,0.000000,0.000000,0.000000,0.000000,0.000000,0.000000,\0"
                    as *const u8 as *const libc::c_char,
            };
            init
        },
    ];
    let mut i: libc::c_int = 0 as libc::c_int;
    while (i as libc::c_ulong)
        < (::std::mem::size_of::<[C2RustUnnamed_15; 8]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<C2RustUnnamed_15>() as libc::c_ulong)
    {
        if strcmp(CONV_KERN_PREDEF[i as usize].name, src) == 0 {
            return parse_conv_kern_lst(
                ps,
                CONV_KERN_PREDEF[i as usize].kern_str,
                dest,
                max,
            );
        }
        i += 1;
        i;
    }
    let mut i_0: libc::c_int = 0 as libc::c_int;
    let mut pc: *const libc::c_char = src;
    i_0 = 0 as libc::c_int;
    while i_0 < max {
        free(*dest.offset(i_0 as isize) as *mut libc::c_void);
        let ref mut fresh8 = *dest.offset(i_0 as isize);
        *fresh8 = 0 as *mut XFixed;
        i_0 += 1;
        i_0;
    }
    i_0 = 0 as libc::c_int;
    while !pc.is_null() && *pc as libc::c_int != 0 && i_0 < max - 1 as libc::c_int {
        let fresh9 = i_0;
        i_0 = i_0 + 1;
        let ref mut fresh10 = *dest.offset(fresh9 as isize);
        *fresh10 = parse_conv_kern(ps, pc, &mut pc);
        if (*fresh10).is_null() {
            return 0 as libc::c_int != 0;
        }
    }
    if *pc != 0 {
        fprintf(
            stderr,
            b"%s(): Too many blur kernels!\n\0" as *const u8 as *const libc::c_char,
            (*::std::mem::transmute::<
                &[u8; 20],
                &[libc::c_char; 20],
            >(b"parse_conv_kern_lst\0"))
                .as_ptr(),
        );
        return 0 as libc::c_int != 0;
    }
    return 1 as libc::c_int != 0;
}
#[inline]
unsafe extern "C" fn parse_geometry(
    mut ps: *mut session_t,
    mut src: *const libc::c_char,
    mut dest: *mut geometry_t,
) -> bool {
    let mut current_block: u64;
    let mut geom: geometry_t = {
        let mut init = geometry_t {
            wid: -(1 as libc::c_int),
            hei: -(1 as libc::c_int),
            x: -(1 as libc::c_int),
            y: -(1 as libc::c_int),
        };
        init
    };
    let mut val: libc::c_long = 0 as libc::c_long;
    let mut endptr: *mut libc::c_char = 0 as *mut libc::c_char;
    while *src as libc::c_int != 0
        && *(*__ctype_b_loc()).offset(*src as libc::c_int as isize) as libc::c_int
            & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        src = src.offset(1);
        src;
    }
    if !(*src == 0) {
        if !('+' as i32 == *src as libc::c_int || '-' as i32 == *src as libc::c_int) {
            val = strtol(src, &mut endptr, 10 as libc::c_int);
            if !endptr.is_null() && src != endptr as *const libc::c_char {
                geom.wid = val as libc::c_int;
                src = endptr;
            }
            while *src as libc::c_int != 0
                && *(*__ctype_b_loc()).offset(*src as libc::c_int as isize)
                    as libc::c_int
                    & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
            {
                src = src.offset(1);
                src;
            }
            if *src == 0 {
                current_block = 631107397986101845;
            } else {
                current_block = 9606288038608642794;
            }
        } else {
            current_block = 9606288038608642794;
        }
        match current_block {
            631107397986101845 => {}
            _ => {
                if 'x' as i32 == *src as libc::c_int {
                    src = src.offset(1);
                    src;
                    val = strtol(src, &mut endptr, 10 as libc::c_int);
                    if !endptr.is_null() && src != endptr as *const libc::c_char {
                        geom.hei = val as libc::c_int;
                        if geom.hei < 0 as libc::c_int {
                            fprintf(
                                stderr,
                                b"%s(\"%s\"): Invalid height.\n\0" as *const u8
                                    as *const libc::c_char,
                                (*::std::mem::transmute::<
                                    &[u8; 15],
                                    &[libc::c_char; 15],
                                >(b"parse_geometry\0"))
                                    .as_ptr(),
                                src,
                            );
                            return 0 as libc::c_int != 0;
                        }
                        src = endptr;
                    }
                    while *src as libc::c_int != 0
                        && *(*__ctype_b_loc()).offset(*src as libc::c_int as isize)
                            as libc::c_int
                            & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                            != 0
                    {
                        src = src.offset(1);
                        src;
                    }
                    if *src == 0 {
                        current_block = 631107397986101845;
                    } else {
                        current_block = 15345278821338558188;
                    }
                } else {
                    current_block = 15345278821338558188;
                }
                match current_block {
                    631107397986101845 => {}
                    _ => {
                        if '+' as i32 == *src as libc::c_int
                            || '-' as i32 == *src as libc::c_int
                        {
                            val = strtol(src, &mut endptr, 10 as libc::c_int);
                            if !endptr.is_null() && src != endptr as *const libc::c_char
                            {
                                geom.x = val as libc::c_int;
                                if '-' as i32 == *src as libc::c_int
                                    && geom.x <= 0 as libc::c_int
                                {
                                    geom.x -= 2 as libc::c_int;
                                }
                                src = endptr;
                            }
                            while *src as libc::c_int != 0
                                && *(*__ctype_b_loc()).offset(*src as libc::c_int as isize)
                                    as libc::c_int
                                    & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                                    != 0
                            {
                                src = src.offset(1);
                                src;
                            }
                            if *src == 0 {
                                current_block = 631107397986101845;
                            } else {
                                current_block = 14072441030219150333;
                            }
                        } else {
                            current_block = 14072441030219150333;
                        }
                        match current_block {
                            631107397986101845 => {}
                            _ => {
                                if '+' as i32 == *src as libc::c_int
                                    || '-' as i32 == *src as libc::c_int
                                {
                                    val = strtol(src, &mut endptr, 10 as libc::c_int);
                                    if !endptr.is_null() && src != endptr as *const libc::c_char
                                    {
                                        geom.y = val as libc::c_int;
                                        if '-' as i32 == *src as libc::c_int
                                            && geom.y <= 0 as libc::c_int
                                        {
                                            geom.y -= 2 as libc::c_int;
                                        }
                                        src = endptr;
                                    }
                                    while *src as libc::c_int != 0
                                        && *(*__ctype_b_loc()).offset(*src as libc::c_int as isize)
                                            as libc::c_int
                                            & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                                            != 0
                                    {
                                        src = src.offset(1);
                                        src;
                                    }
                                    if *src == 0 {
                                        current_block = 631107397986101845;
                                    } else {
                                        current_block = 13678349939556791712;
                                    }
                                } else {
                                    current_block = 13678349939556791712;
                                }
                                match current_block {
                                    631107397986101845 => {}
                                    _ => {
                                        if *src != 0 {
                                            fprintf(
                                                stderr,
                                                b"%s(\"%s\"): Trailing characters.\n\0" as *const u8
                                                    as *const libc::c_char,
                                                (*::std::mem::transmute::<
                                                    &[u8; 15],
                                                    &[libc::c_char; 15],
                                                >(b"parse_geometry\0"))
                                                    .as_ptr(),
                                                src,
                                            );
                                            return 0 as libc::c_int != 0;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    *dest = geom;
    return 1 as libc::c_int != 0;
}
#[inline]
unsafe extern "C" fn parse_rule_opacity(
    mut ps: *mut session_t,
    mut src: *const libc::c_char,
) -> bool {
    let mut endptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut val: libc::c_long = strtol(src, &mut endptr, 0 as libc::c_int);
    if endptr.is_null() || endptr == src as *mut libc::c_char {
        fprintf(
            stderr,
            b"%s(\"%s\"): No opacity specified?\n\0" as *const u8 as *const libc::c_char,
            (*::std::mem::transmute::<
                &[u8; 19],
                &[libc::c_char; 19],
            >(b"parse_rule_opacity\0"))
                .as_ptr(),
            src,
        );
        return 0 as libc::c_int != 0;
    }
    if val > 100 as libc::c_int as libc::c_long || val < 0 as libc::c_int as libc::c_long
    {
        fprintf(
            stderr,
            b"%s(\"%s\"): Opacity %ld invalid.\n\0" as *const u8 as *const libc::c_char,
            (*::std::mem::transmute::<
                &[u8; 19],
                &[libc::c_char; 19],
            >(b"parse_rule_opacity\0"))
                .as_ptr(),
            src,
            val,
        );
        return 0 as libc::c_int != 0;
    }
    while *endptr as libc::c_int != 0
        && *(*__ctype_b_loc()).offset(*endptr as libc::c_int as isize) as libc::c_int
            & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        endptr = endptr.offset(1);
        endptr;
    }
    if ':' as i32 != *endptr as libc::c_int {
        fprintf(
            stderr,
            b"%s(\"%s\"): Opacity terminator not found.\n\0" as *const u8
                as *const libc::c_char,
            (*::std::mem::transmute::<
                &[u8; 19],
                &[libc::c_char; 19],
            >(b"parse_rule_opacity\0"))
                .as_ptr(),
            src,
        );
        return 0 as libc::c_int != 0;
    }
    endptr = endptr.offset(1);
    endptr;
    return !(c2_parsed(ps, &mut (*ps).o.opacity_rules, endptr, val as *mut libc::c_void))
        .is_null();
}
unsafe extern "C" fn get_cfg(
    mut ps: *mut session_t,
    mut argc: libc::c_int,
    mut argv: *const *mut libc::c_char,
    mut first_pass: bool,
) {
    static mut shortopts: *const libc::c_char = b"D:I:O:d:r:o:m:l:t:i:e:hscnfFCaSzGb\0"
        as *const u8 as *const libc::c_char;
    static mut longopts: [option; 85] = [
        {
            let mut init = option {
                name: b"help\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'h' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"config\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 256 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"shadow-radius\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'r' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"shadow-opacity\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'o' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"shadow-offset-x\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'l' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"shadow-offset-y\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 't' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"fade-in-step\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'I' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"fade-out-step\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'O' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"fade-delta\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'D' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"menu-opacity\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'm' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"shadow\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'c' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"no-dock-shadow\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'C' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"clear-shadow\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'z' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"fading\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'f' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"inactive-opacity\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'i' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"frame-opacity\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'e' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"daemon\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'b' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"no-dnd-shadow\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'G' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"shadow-red\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 257 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"shadow-green\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 258 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"shadow-blue\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 259 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"inactive-opacity-override\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 260 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"inactive-dim\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 261 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"mark-wmwin-focused\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 262 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"shadow-exclude\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 263 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"mark-ovredir-focused\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 264 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"no-fading-openclose\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 265 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"shadow-ignore-shaped\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 266 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"detect-rounded-corners\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 267 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"detect-client-opacity\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 268 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"refresh-rate\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 269 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"vsync\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 270 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"alpha-step\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 271 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"dbe\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 272 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"paint-on-overlay\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 273 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"sw-opti\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 274 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"vsync-aggressive\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 275 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"use-ewmh-active-win\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 276 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"respect-prop-shadow\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 277 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"unredir-if-possible\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 278 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"focus-exclude\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 279 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"inactive-dim-fixed\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 280 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"detect-transient\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 281 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"detect-client-leader\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 282 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"blur-background\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 283 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"blur-background-frame\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 284 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"blur-background-fixed\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 285 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"dbus\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 286 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"logpath\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 287 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"invert-color-include\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 288 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"opengl\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 289 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"backend\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 290 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"glx-no-stencil\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 291 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"glx-copy-from-front\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 292 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"benchmark\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 293 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"benchmark-wid\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 294 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"glx-use-copysubbuffermesa\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 295 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"blur-background-exclude\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 296 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"active-opacity\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 297 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"glx-no-rebind-pixmap\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 298 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"glx-swap-method\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 299 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"fade-exclude\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 300 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"blur-kern\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 301 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"resize-damage\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 302 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"glx-use-gpushader4\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 303 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"opacity-rule\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 304 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"shadow-exclude-reg\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 305 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"paint-exclude\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 306 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"xinerama-shadow-crop\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 307 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"unredir-if-possible-exclude\0" as *const u8
                    as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 308 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"unredir-if-possible-delay\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 309 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"write-pid-path\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 310 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"vsync-use-glfinish\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 311 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"xrender-sync\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 312 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"xrender-sync-fence\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 313 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"show-all-xerrors\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 314 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"no-fading-destroyed-argb\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 315 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"force-win-blend\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 316 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"glx-fshader-win\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 317 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"version\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 318 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"no-x-selection\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 319 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"no-name-pixmap\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 320 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"reredir-on-root-change\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 731 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"glx-reinit-on-root-change\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 732 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: 0 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 0 as libc::c_int,
            };
            init
        },
    ];
    let mut o: libc::c_int = 0 as libc::c_int;
    let mut longopt_idx: libc::c_int = -(1 as libc::c_int);
    let mut i: libc::c_int = 0 as libc::c_int;
    if first_pass {
        optind = 1 as libc::c_int;
        loop {
            o = getopt_long(argc, argv, shortopts, longopts.as_ptr(), &mut longopt_idx);
            if !(-(1 as libc::c_int) != o) {
                break;
            }
            if 256 as libc::c_int == o {
                (*ps).o.config_file = mstrcpy(optarg);
            } else if 'd' as i32 == o {
                (*ps).o.display = mstrcpy(optarg);
            } else if 'S' as i32 == o {
                (*ps).o.synchronize = 1 as libc::c_int != 0;
            } else if 314 as libc::c_int == o {
                (*ps).o.show_all_xerrors = 1 as libc::c_int != 0;
            } else if 318 as libc::c_int == o {
                printf(
                    b"%s\n\0" as *const u8 as *const libc::c_char,
                    b"git--\0" as *const u8 as *const libc::c_char,
                );
                exit(0 as libc::c_int);
            } else if 320 as libc::c_int == o {
                (*ps).o.no_name_pixmap = 1 as libc::c_int != 0;
            } else if '?' as i32 == o || ':' as i32 == o {
                usage(1 as libc::c_int);
            }
        }
        if optind < argc {
            fprintf(
                stderr,
                b"%s(): compton doesn't accept positional arguments.\n\0" as *const u8
                    as *const libc::c_char,
                (*::std::mem::transmute::<&[u8; 8], &[libc::c_char; 8]>(b"get_cfg\0"))
                    .as_ptr(),
            );
            exit(1 as libc::c_int);
        }
        return;
    }
    let mut cfgtmp: options_tmp = {
        let mut init = options_tmp {
            no_dock_shadow: 0 as libc::c_int != 0,
            no_dnd_shadow: 0 as libc::c_int != 0,
            menu_opacity: 1.0f64,
        };
        init
    };
    let mut shadow_enable: bool = 0 as libc::c_int != 0;
    let mut fading_enable: bool = 0 as libc::c_int != 0;
    let mut lc_numeric_old: *mut libc::c_char = mstrcpy(
        setlocale(1 as libc::c_int, 0 as *const libc::c_char),
    );
    i = 0 as libc::c_int;
    while i < NUM_WINTYPES as libc::c_int {
        (*ps).o.wintype_fade[i as usize] = 0 as libc::c_int != 0;
        (*ps).o.wintype_shadow[i as usize] = 0 as libc::c_int != 0;
        (*ps).o.wintype_opacity[i as usize] = 1.0f64;
        i += 1;
        i;
    }
    setlocale(1 as libc::c_int, b"C\0" as *const u8 as *const libc::c_char);
    optind = 1 as libc::c_int;
    loop {
        o = getopt_long(argc, argv, shortopts, longopts.as_ptr(), &mut longopt_idx);
        if !(-(1 as libc::c_int) != o) {
            break;
        }
        let mut val: libc::c_long = 0 as libc::c_int as libc::c_long;
        match o {
            104 => {
                usage(0 as libc::c_int);
            }
            68 => {
                if !parse_long(optarg, &mut val) {
                    exit(1 as libc::c_int);
                }
                (*ps).o.fade_delta = val;
            }
            73 => {
                (*ps)
                    .o
                    .fade_in_step = (normalize_d(atof(optarg))
                    * 0xffffffff as libc::c_uint as libc::c_double) as opacity_t;
            }
            79 => {
                (*ps)
                    .o
                    .fade_out_step = (normalize_d(atof(optarg))
                    * 0xffffffff as libc::c_uint as libc::c_double) as opacity_t;
            }
            99 => {
                shadow_enable = 1 as libc::c_int != 0;
            }
            67 => {
                cfgtmp.no_dock_shadow = 1 as libc::c_int != 0;
            }
            71 => {
                cfgtmp.no_dnd_shadow = 1 as libc::c_int != 0;
            }
            109 => {
                cfgtmp.menu_opacity = atof(optarg);
            }
            102 | 70 => {
                fading_enable = 1 as libc::c_int != 0;
            }
            114 => {
                if !parse_long(optarg, &mut val) {
                    exit(1 as libc::c_int);
                }
                (*ps).o.shadow_radius = val as libc::c_int;
            }
            111 => {
                (*ps).o.shadow_opacity = atof(optarg);
            }
            108 => {
                if !parse_long(optarg, &mut val) {
                    exit(1 as libc::c_int);
                }
                (*ps).o.shadow_offset_x = val as libc::c_int;
            }
            116 => {
                if !parse_long(optarg, &mut val) {
                    exit(1 as libc::c_int);
                }
                (*ps).o.shadow_offset_y = val as libc::c_int;
            }
            105 => {
                (*ps)
                    .o
                    .inactive_opacity = (normalize_d(atof(optarg))
                    * 0xffffffff as libc::c_uint as libc::c_double) as opacity_t;
            }
            101 => {
                (*ps).o.frame_opacity = atof(optarg);
            }
            122 => {
                (*ps).o.clear_shadow = 1 as libc::c_int != 0;
            }
            110 | 97 | 115 => {
                fprintf(
                    stderr,
                    b"%s(): -n, -a, and -s have been removed.\n\0" as *const u8
                        as *const libc::c_char,
                    (*::std::mem::transmute::<
                        &[u8; 8],
                        &[libc::c_char; 8],
                    >(b"get_cfg\0"))
                        .as_ptr(),
                );
                exit(1 as libc::c_int);
            }
            98 => {
                (*ps).o.fork_after_register = 1 as libc::c_int != 0;
            }
            100 | 83 | 314 | 318 | 320 | 256 => {}
            257 => {
                (*ps).o.shadow_red = atof(optarg);
            }
            258 => {
                (*ps).o.shadow_green = atof(optarg);
            }
            259 => {
                (*ps).o.shadow_blue = atof(optarg);
            }
            260 => {
                (*ps).o.inactive_opacity_override = 1 as libc::c_int != 0;
            }
            261 => {
                (*ps).o.inactive_dim = atof(optarg);
            }
            262 => {
                (*ps).o.mark_wmwin_focused = 1 as libc::c_int != 0;
            }
            263 => {
                condlst_add(ps, &mut (*ps).o.shadow_blacklist, optarg);
            }
            264 => {
                (*ps).o.mark_ovredir_focused = 1 as libc::c_int != 0;
            }
            265 => {
                (*ps).o.no_fading_openclose = 1 as libc::c_int != 0;
            }
            266 => {
                (*ps).o.shadow_ignore_shaped = 1 as libc::c_int != 0;
            }
            267 => {
                (*ps).o.detect_rounded_corners = 1 as libc::c_int != 0;
            }
            268 => {
                (*ps).o.detect_client_opacity = 1 as libc::c_int != 0;
            }
            269 => {
                if !parse_long(optarg, &mut val) {
                    exit(1 as libc::c_int);
                }
                (*ps).o.refresh_rate = val as libc::c_int;
            }
            270 => {
                if !parse_vsync(ps, optarg) {
                    exit(1 as libc::c_int);
                }
            }
            271 => {
                (*ps).o.alpha_step = atof(optarg);
            }
            272 => {
                (*ps).o.dbe = 1 as libc::c_int != 0;
            }
            273 => {
                (*ps).o.paint_on_overlay = 1 as libc::c_int != 0;
            }
            274 => {
                (*ps).o.sw_opti = 1 as libc::c_int != 0;
            }
            275 => {
                (*ps).o.vsync_aggressive = 1 as libc::c_int != 0;
            }
            276 => {
                (*ps).o.use_ewmh_active_win = 1 as libc::c_int != 0;
            }
            277 => {
                (*ps).o.respect_prop_shadow = 1 as libc::c_int != 0;
            }
            278 => {
                (*ps).o.unredir_if_possible = 1 as libc::c_int != 0;
            }
            279 => {
                condlst_add(ps, &mut (*ps).o.focus_blacklist, optarg);
            }
            280 => {
                (*ps).o.inactive_dim_fixed = 1 as libc::c_int != 0;
            }
            281 => {
                (*ps).o.detect_transient = 1 as libc::c_int != 0;
            }
            282 => {
                (*ps).o.detect_client_leader = 1 as libc::c_int != 0;
            }
            283 => {
                (*ps).o.blur_background = 1 as libc::c_int != 0;
            }
            284 => {
                (*ps).o.blur_background_frame = 1 as libc::c_int != 0;
            }
            285 => {
                (*ps).o.blur_background_fixed = 1 as libc::c_int != 0;
            }
            286 => {
                (*ps).o.dbus = 1 as libc::c_int != 0;
            }
            287 => {
                (*ps).o.logpath = mstrcpy(optarg);
            }
            288 => {
                condlst_add(ps, &mut (*ps).o.invert_color_list, optarg);
            }
            289 => {
                (*ps).o.backend = BKEND_GLX;
            }
            290 => {
                if !parse_backend(ps, optarg) {
                    exit(1 as libc::c_int);
                }
            }
            291 => {
                (*ps).o.glx_no_stencil = 1 as libc::c_int != 0;
            }
            292 => {
                (*ps).o.glx_copy_from_front = 1 as libc::c_int != 0;
            }
            293 => {
                if !parse_long(optarg, &mut val) {
                    exit(1 as libc::c_int);
                }
                (*ps).o.benchmark = val as libc::c_int;
            }
            294 => {
                (*ps)
                    .o
                    .benchmark_wid = strtol(
                    optarg,
                    0 as *mut *mut libc::c_char,
                    0 as libc::c_int,
                ) as Window;
            }
            295 => {
                (*ps).o.glx_use_copysubbuffermesa = 1 as libc::c_int != 0;
            }
            296 => {
                condlst_add(ps, &mut (*ps).o.blur_background_blacklist, optarg);
            }
            297 => {
                (*ps)
                    .o
                    .active_opacity = (normalize_d(atof(optarg))
                    * 0xffffffff as libc::c_uint as libc::c_double) as opacity_t;
            }
            298 => {
                (*ps).o.glx_no_rebind_pixmap = 1 as libc::c_int != 0;
            }
            299 => {
                if !parse_glx_swap_method(ps, optarg) {
                    exit(1 as libc::c_int);
                }
            }
            300 => {
                condlst_add(ps, &mut (*ps).o.fade_blacklist, optarg);
            }
            301 => {
                if !parse_conv_kern_lst(
                    ps,
                    optarg,
                    ((*ps).o.blur_kerns).as_mut_ptr(),
                    5 as libc::c_int,
                ) {
                    exit(1 as libc::c_int);
                }
            }
            302 => {
                if !parse_long(optarg, &mut val) {
                    exit(1 as libc::c_int);
                }
                (*ps).o.resize_damage = val as libc::c_int;
            }
            303 => {
                (*ps).o.glx_use_gpushader4 = 1 as libc::c_int != 0;
            }
            304 => {
                if !parse_rule_opacity(ps, optarg) {
                    exit(1 as libc::c_int);
                }
            }
            305 => {
                if !parse_geometry(ps, optarg, &mut (*ps).o.shadow_exclude_reg_geom) {
                    exit(1 as libc::c_int);
                }
            }
            306 => {
                condlst_add(ps, &mut (*ps).o.paint_blacklist, optarg);
            }
            307 => {
                (*ps).o.xinerama_shadow_crop = 1 as libc::c_int != 0;
            }
            308 => {
                condlst_add(ps, &mut (*ps).o.unredir_if_possible_blacklist, optarg);
            }
            309 => {
                if !parse_long(optarg, &mut val) {
                    exit(1 as libc::c_int);
                }
                (*ps).o.unredir_if_possible_delay = val;
            }
            310 => {
                (*ps).o.write_pid_path = mstrcpy(optarg);
            }
            311 => {
                (*ps).o.vsync_use_glfinish = 1 as libc::c_int != 0;
            }
            312 => {
                (*ps).o.xrender_sync = 1 as libc::c_int != 0;
            }
            313 => {
                (*ps).o.xrender_sync_fence = 1 as libc::c_int != 0;
            }
            315 => {
                (*ps).o.no_fading_destroyed_argb = 1 as libc::c_int != 0;
            }
            316 => {
                (*ps).o.force_win_blend = 1 as libc::c_int != 0;
            }
            317 => {
                (*ps).o.glx_fshader_win_str = mstrcpy(optarg);
            }
            319 => {
                (*ps).o.no_x_selection = 1 as libc::c_int != 0;
            }
            731 => {
                (*ps).o.reredir_on_root_change = 1 as libc::c_int != 0;
            }
            732 => {
                (*ps).o.glx_reinit_on_root_change = 1 as libc::c_int != 0;
            }
            _ => {
                usage(1 as libc::c_int);
            }
        }
    }
    setlocale(1 as libc::c_int, lc_numeric_old);
    free(lc_numeric_old as *mut libc::c_void);
    (*ps)
        .o
        .fade_delta = max_i((*ps).o.fade_delta as libc::c_int, 1 as libc::c_int)
        as time_ms_t;
    (*ps).o.shadow_radius = max_i((*ps).o.shadow_radius, 1 as libc::c_int);
    (*ps).o.shadow_red = normalize_d((*ps).o.shadow_red);
    (*ps).o.shadow_green = normalize_d((*ps).o.shadow_green);
    (*ps).o.shadow_blue = normalize_d((*ps).o.shadow_blue);
    (*ps).o.inactive_dim = normalize_d((*ps).o.inactive_dim);
    (*ps).o.frame_opacity = normalize_d((*ps).o.frame_opacity);
    (*ps).o.shadow_opacity = normalize_d((*ps).o.shadow_opacity);
    cfgtmp.menu_opacity = normalize_d(cfgtmp.menu_opacity);
    (*ps)
        .o
        .refresh_rate = normalize_i_range(
        (*ps).o.refresh_rate,
        0 as libc::c_int,
        300 as libc::c_int,
    );
    (*ps).o.alpha_step = normalize_d_range((*ps).o.alpha_step, 0.01f64, 1.0f64);
    if 0xffffffff as libc::c_uint == (*ps).o.inactive_opacity {
        (*ps).o.inactive_opacity = 0 as libc::c_int as opacity_t;
    }
    if 0xffffffff as libc::c_uint == (*ps).o.active_opacity {
        (*ps).o.active_opacity = 0 as libc::c_int as opacity_t;
    }
    if shadow_enable {
        wintype_arr_enable(((*ps).o.wintype_shadow).as_mut_ptr());
    }
    (*ps)
        .o
        .wintype_shadow[WINTYPE_DESKTOP as libc::c_int as usize] = 0 as libc::c_int != 0;
    if cfgtmp.no_dock_shadow {
        (*ps)
            .o
            .wintype_shadow[WINTYPE_DOCK as libc::c_int
            as usize] = 0 as libc::c_int != 0;
    }
    if cfgtmp.no_dnd_shadow {
        (*ps)
            .o
            .wintype_shadow[WINTYPE_DND as libc::c_int as usize] = 0 as libc::c_int != 0;
    }
    if fading_enable {
        wintype_arr_enable(((*ps).o.wintype_fade).as_mut_ptr());
    }
    if 1.0f64 != cfgtmp.menu_opacity {
        (*ps)
            .o
            .wintype_opacity[WINTYPE_DROPDOWN_MENU as libc::c_int
            as usize] = cfgtmp.menu_opacity;
        (*ps)
            .o
            .wintype_opacity[WINTYPE_POPUP_MENU as libc::c_int
            as usize] = cfgtmp.menu_opacity;
    }
    if (*ps).o.blur_background_frame {
        (*ps).o.blur_background = 1 as libc::c_int != 0;
    }
    if (*ps).o.xrender_sync_fence {
        (*ps).o.xrender_sync = 1 as libc::c_int != 0;
    }
    if (*ps).o.inactive_opacity != 0 || (*ps).o.active_opacity != 0
        || (*ps).o.inactive_dim != 0.
    {
        (*ps).o.track_focus = 1 as libc::c_int != 0;
    }
    if (*ps).o.detect_transient as libc::c_int != 0
        || (*ps).o.detect_client_leader as libc::c_int != 0
    {
        (*ps).o.track_leader = 1 as libc::c_int != 0;
    }
    if (*ps).o.blur_background as libc::c_int != 0
        && ((*ps).o.blur_kerns[0 as libc::c_int as usize]).is_null()
    {
        static mut convolution_blur: [XFixed; 11] = [
            3 as libc::c_int * 65536 as libc::c_int,
            3 as libc::c_int * 65536 as libc::c_int,
            1 as libc::c_int * 65536 as libc::c_int,
            1 as libc::c_int * 65536 as libc::c_int,
            1 as libc::c_int * 65536 as libc::c_int,
            1 as libc::c_int * 65536 as libc::c_int,
            1 as libc::c_int * 65536 as libc::c_int,
            1 as libc::c_int * 65536 as libc::c_int,
            1 as libc::c_int * 65536 as libc::c_int,
            1 as libc::c_int * 65536 as libc::c_int,
            1 as libc::c_int * 65536 as libc::c_int,
        ];
        (*ps)
            .o
            .blur_kerns[0 as libc::c_int
            as usize] = malloc(::std::mem::size_of::<[XFixed; 11]>() as libc::c_ulong)
            as *mut XFixed;
        if ((*ps).o.blur_kerns[0 as libc::c_int as usize]).is_null() {
            fprintf(
                stderr,
                b"%s(): Failed to allocate memory for convolution kernel.\n\0"
                    as *const u8 as *const libc::c_char,
                (*::std::mem::transmute::<&[u8; 8], &[libc::c_char; 8]>(b"get_cfg\0"))
                    .as_ptr(),
            );
            exit(1 as libc::c_int);
        }
        memcpy(
            (*ps).o.blur_kerns[0 as libc::c_int as usize] as *mut libc::c_void,
            &convolution_blur as *const [XFixed; 11] as *const libc::c_void,
            ::std::mem::size_of::<[XFixed; 11]>() as libc::c_ulong,
        );
    }
    rebuild_shadow_exclude_reg(ps);
    if (*ps).o.resize_damage < 0 as libc::c_int {
        fprintf(
            stderr,
            b"%s(): Negative --resize-damage does not work correctly.\n\0" as *const u8
                as *const libc::c_char,
            (*::std::mem::transmute::<&[u8; 8], &[libc::c_char; 8]>(b"get_cfg\0"))
                .as_ptr(),
        );
    }
}
unsafe extern "C" fn init_atoms(mut ps: *mut session_t) {
    (*ps)
        .atom_opacity = get_atom(
        ps,
        b"_NET_WM_WINDOW_OPACITY\0" as *const u8 as *const libc::c_char,
    );
    (*ps)
        .atom_frame_extents = get_atom(
        ps,
        b"_NET_FRAME_EXTENTS\0" as *const u8 as *const libc::c_char,
    );
    (*ps).atom_client = get_atom(ps, b"WM_STATE\0" as *const u8 as *const libc::c_char);
    (*ps).atom_name = 39 as libc::c_int as Atom;
    (*ps)
        .atom_name_ewmh = get_atom(
        ps,
        b"_NET_WM_NAME\0" as *const u8 as *const libc::c_char,
    );
    (*ps).atom_class = 67 as libc::c_int as Atom;
    (*ps)
        .atom_role = get_atom(
        ps,
        b"WM_WINDOW_ROLE\0" as *const u8 as *const libc::c_char,
    );
    (*ps).atom_transient = 68 as libc::c_int as Atom;
    (*ps)
        .atom_client_leader = get_atom(
        ps,
        b"WM_CLIENT_LEADER\0" as *const u8 as *const libc::c_char,
    );
    (*ps)
        .atom_ewmh_active_win = get_atom(
        ps,
        b"_NET_ACTIVE_WINDOW\0" as *const u8 as *const libc::c_char,
    );
    (*ps)
        .atom_compton_shadow = get_atom(
        ps,
        b"_COMPTON_SHADOW\0" as *const u8 as *const libc::c_char,
    );
    (*ps)
        .atom_win_type = get_atom(
        ps,
        b"_NET_WM_WINDOW_TYPE\0" as *const u8 as *const libc::c_char,
    );
    (*ps)
        .atoms_wintypes[WINTYPE_UNKNOWN as libc::c_int
        as usize] = 0 as libc::c_int as Atom;
    (*ps)
        .atoms_wintypes[WINTYPE_DESKTOP as libc::c_int
        as usize] = get_atom(
        ps,
        b"_NET_WM_WINDOW_TYPE_DESKTOP\0" as *const u8 as *const libc::c_char,
    );
    (*ps)
        .atoms_wintypes[WINTYPE_DOCK as libc::c_int
        as usize] = get_atom(
        ps,
        b"_NET_WM_WINDOW_TYPE_DOCK\0" as *const u8 as *const libc::c_char,
    );
    (*ps)
        .atoms_wintypes[WINTYPE_TOOLBAR as libc::c_int
        as usize] = get_atom(
        ps,
        b"_NET_WM_WINDOW_TYPE_TOOLBAR\0" as *const u8 as *const libc::c_char,
    );
    (*ps)
        .atoms_wintypes[WINTYPE_MENU as libc::c_int
        as usize] = get_atom(
        ps,
        b"_NET_WM_WINDOW_TYPE_MENU\0" as *const u8 as *const libc::c_char,
    );
    (*ps)
        .atoms_wintypes[WINTYPE_UTILITY as libc::c_int
        as usize] = get_atom(
        ps,
        b"_NET_WM_WINDOW_TYPE_UTILITY\0" as *const u8 as *const libc::c_char,
    );
    (*ps)
        .atoms_wintypes[WINTYPE_SPLASH as libc::c_int
        as usize] = get_atom(
        ps,
        b"_NET_WM_WINDOW_TYPE_SPLASH\0" as *const u8 as *const libc::c_char,
    );
    (*ps)
        .atoms_wintypes[WINTYPE_DIALOG as libc::c_int
        as usize] = get_atom(
        ps,
        b"_NET_WM_WINDOW_TYPE_DIALOG\0" as *const u8 as *const libc::c_char,
    );
    (*ps)
        .atoms_wintypes[WINTYPE_NORMAL as libc::c_int
        as usize] = get_atom(
        ps,
        b"_NET_WM_WINDOW_TYPE_NORMAL\0" as *const u8 as *const libc::c_char,
    );
    (*ps)
        .atoms_wintypes[WINTYPE_DROPDOWN_MENU as libc::c_int
        as usize] = get_atom(
        ps,
        b"_NET_WM_WINDOW_TYPE_DROPDOWN_MENU\0" as *const u8 as *const libc::c_char,
    );
    (*ps)
        .atoms_wintypes[WINTYPE_POPUP_MENU as libc::c_int
        as usize] = get_atom(
        ps,
        b"_NET_WM_WINDOW_TYPE_POPUP_MENU\0" as *const u8 as *const libc::c_char,
    );
    (*ps)
        .atoms_wintypes[WINTYPE_TOOLTIP as libc::c_int
        as usize] = get_atom(
        ps,
        b"_NET_WM_WINDOW_TYPE_TOOLTIP\0" as *const u8 as *const libc::c_char,
    );
    (*ps)
        .atoms_wintypes[WINTYPE_NOTIFY as libc::c_int
        as usize] = get_atom(
        ps,
        b"_NET_WM_WINDOW_TYPE_NOTIFICATION\0" as *const u8 as *const libc::c_char,
    );
    (*ps)
        .atoms_wintypes[WINTYPE_COMBO as libc::c_int
        as usize] = get_atom(
        ps,
        b"_NET_WM_WINDOW_TYPE_COMBO\0" as *const u8 as *const libc::c_char,
    );
    (*ps)
        .atoms_wintypes[WINTYPE_DND as libc::c_int
        as usize] = get_atom(
        ps,
        b"_NET_WM_WINDOW_TYPE_DND\0" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn update_refresh_rate(mut ps: *mut session_t) {
    let mut randr_info: *mut XRRScreenConfiguration = 0 as *mut XRRScreenConfiguration;
    randr_info = XRRGetScreenInfo((*ps).dpy, (*ps).root);
    if randr_info.is_null() {
        return;
    }
    (*ps).refresh_rate = XRRConfigCurrentRate(randr_info);
    XRRFreeScreenConfigInfo(randr_info);
    if (*ps).refresh_rate != 0 {
        (*ps)
            .refresh_intv = 1000000 as libc::c_long / (*ps).refresh_rate as libc::c_long;
    } else {
        (*ps).refresh_intv = 0 as libc::c_int as libc::c_long;
    };
}
unsafe extern "C" fn swopti_init(mut ps: *mut session_t) -> bool {
    (*ps).refresh_rate = (*ps).o.refresh_rate as libc::c_short;
    if (*ps).refresh_rate != 0 {
        (*ps)
            .refresh_intv = 1000000 as libc::c_long / (*ps).refresh_rate as libc::c_long;
    }
    if (*ps).refresh_rate == 0 && (*ps).randr_exists as libc::c_int != 0 {
        update_refresh_rate(ps);
    }
    if (*ps).refresh_rate == 0 {
        return 0 as libc::c_int != 0;
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn swopti_handle_timeout(
    mut ps: *mut session_t,
    mut ptv: *mut timeval,
) {
    if ptv.is_null() {
        return;
    }
    let mut offset: libc::c_long = ((*ptv).tv_usec + (get_time_timeval()).tv_usec
        - (*ps).paint_tm_offset) % (*ps).refresh_intv;
    if offset < 0 as libc::c_int as libc::c_long {
        offset += (*ps).refresh_intv;
    }
    if offset < 3000 as libc::c_int as libc::c_long
        || offset > (*ps).refresh_intv - 3000 as libc::c_int as libc::c_long
    {
        return;
    }
    (*ptv).tv_usec += (*ps).refresh_intv - offset;
    if (*ptv).tv_usec > 1000000 as libc::c_long {
        (*ptv).tv_usec -= 1000000 as libc::c_long;
        (*ptv).tv_sec += 1;
        (*ptv).tv_sec;
    }
}
unsafe extern "C" fn vsync_drm_init(mut ps: *mut session_t) -> bool {
    fprintf(
        stderr,
        b"%s(): Program not compiled with DRM VSync support.\n\0" as *const u8
            as *const libc::c_char,
        (*::std::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"vsync_drm_init\0"))
            .as_ptr(),
    );
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn vsync_opengl_init(mut ps: *mut session_t) -> bool {
    fprintf(
        stderr,
        b"%s(): Program not compiled with OpenGL VSync support.\n\0" as *const u8
            as *const libc::c_char,
        (*::std::mem::transmute::<
            &[u8; 18],
            &[libc::c_char; 18],
        >(b"vsync_opengl_init\0"))
            .as_ptr(),
    );
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn vsync_opengl_oml_init(mut ps: *mut session_t) -> bool {
    fprintf(
        stderr,
        b"%s(): Program not compiled with OpenGL VSync support.\n\0" as *const u8
            as *const libc::c_char,
        (*::std::mem::transmute::<
            &[u8; 22],
            &[libc::c_char; 22],
        >(b"vsync_opengl_oml_init\0"))
            .as_ptr(),
    );
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn vsync_opengl_swc_init(mut ps: *mut session_t) -> bool {
    fprintf(
        stderr,
        b"%s(): Program not compiled with OpenGL VSync support.\n\0" as *const u8
            as *const libc::c_char,
        (*::std::mem::transmute::<
            &[u8; 22],
            &[libc::c_char; 22],
        >(b"vsync_opengl_swc_init\0"))
            .as_ptr(),
    );
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn vsync_opengl_mswc_init(mut ps: *mut session_t) -> bool {
    fprintf(
        stderr,
        b"%s(): Program not compiled with OpenGL VSync support.\n\0" as *const u8
            as *const libc::c_char,
        (*::std::mem::transmute::<
            &[u8; 23],
            &[libc::c_char; 23],
        >(b"vsync_opengl_mswc_init\0"))
            .as_ptr(),
    );
    return 0 as libc::c_int != 0;
}
pub unsafe extern "C" fn vsync_init(mut ps: *mut session_t) -> bool {
    if (*ps).o.vsync as libc::c_uint != 0
        && (VSYNC_FUNCS_INIT[(*ps).o.vsync as usize]).is_some()
        && !(VSYNC_FUNCS_INIT[(*ps).o.vsync as usize]).unwrap()(ps)
    {
        (*ps).o.vsync = VSYNC_NONE;
        return 0 as libc::c_int != 0;
    } else {
        return 1 as libc::c_int != 0
    };
}
unsafe extern "C" fn vsync_wait(mut ps: *mut session_t) {
    if (*ps).o.vsync as u64 == 0 {
        return;
    }
    if (VSYNC_FUNCS_WAIT[(*ps).o.vsync as usize]).is_some() {
        (VSYNC_FUNCS_WAIT[(*ps).o.vsync as usize]).unwrap()(ps);
    }
}
pub unsafe extern "C" fn vsync_deinit(mut ps: *mut session_t) {
    if (*ps).o.vsync as libc::c_uint != 0
        && (VSYNC_FUNCS_DEINIT[(*ps).o.vsync as usize]).is_some()
    {
        (VSYNC_FUNCS_DEINIT[(*ps).o.vsync as usize]).unwrap()(ps);
    }
}
unsafe extern "C" fn init_alpha_picts(mut ps: *mut session_t) {
    let mut i: libc::c_int = 0;
    let mut num: libc::c_int = (round(1.0f64 / (*ps).o.alpha_step)
        + 1 as libc::c_int as libc::c_double) as libc::c_int;
    (*ps)
        .alpha_picts = malloc(
        (::std::mem::size_of::<Picture>() as libc::c_ulong)
            .wrapping_mul(num as libc::c_ulong),
    ) as *mut Picture;
    i = 0 as libc::c_int;
    while i < num {
        let mut o: libc::c_double = i as libc::c_double * (*ps).o.alpha_step;
        if 1.0f64 - o > (*ps).o.alpha_step {
            *((*ps).alpha_picts)
                .offset(
                    i as isize,
                ) = solid_picture(
                ps,
                0 as libc::c_int != 0,
                o,
                0 as libc::c_int as libc::c_double,
                0 as libc::c_int as libc::c_double,
                0 as libc::c_int as libc::c_double,
            );
        } else {
            *((*ps).alpha_picts).offset(i as isize) = 0 as libc::c_long as Picture;
        }
        i += 1;
        i;
    }
}
unsafe extern "C" fn init_dbe(mut ps: *mut session_t) -> bool {
    (*ps)
        .root_dbe = XdbeAllocateBackBufferName(
        (*ps).dpy,
        if (*ps).o.paint_on_overlay as libc::c_int != 0 {
            (*ps).overlay
        } else {
            (*ps).root
        },
        3 as libc::c_int as XdbeSwapAction,
    );
    if (*ps).root_dbe == 0 {
        fprintf(
            stderr,
            b"%s(): Failed to create double buffer. Double buffering cannot work.\n\0"
                as *const u8 as *const libc::c_char,
            (*::std::mem::transmute::<&[u8; 9], &[libc::c_char; 9]>(b"init_dbe\0"))
                .as_ptr(),
        );
        return 0 as libc::c_int != 0;
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn init_overlay(mut ps: *mut session_t) -> bool {
    (*ps).overlay = XCompositeGetOverlayWindow((*ps).dpy, (*ps).root);
    if (*ps).overlay != 0 {
        let mut region: XserverRegion = XFixesCreateRegion(
            (*ps).dpy,
            0 as *mut XRectangle,
            0 as libc::c_int,
        );
        XFixesSetWindowShapeRegion(
            (*ps).dpy,
            (*ps).overlay,
            0 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int as XserverRegion,
        );
        XFixesSetWindowShapeRegion(
            (*ps).dpy,
            (*ps).overlay,
            2 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
            region,
        );
        XFixesDestroyRegion((*ps).dpy, region);
        XSelectInput((*ps).dpy, (*ps).overlay, (1 as libc::c_long) << 15 as libc::c_int);
    } else {
        fprintf(
            stderr,
            b"Cannot get X Composite overlay window. Falling back to painting on root window.\n\0"
                as *const u8 as *const libc::c_char,
        );
        (*ps).o.paint_on_overlay = 0 as libc::c_int != 0;
    }
    return (*ps).overlay != 0;
}
unsafe extern "C" fn init_filters(mut ps: *mut session_t) -> bool {
    if (*ps).o.blur_background as libc::c_int != 0
        || (*ps).o.blur_background_frame as libc::c_int != 0
    {
        match (*ps).o.backend as libc::c_uint {
            0 | 2 => {
                let mut pf: *mut XFilters = XRenderQueryFilters(
                    (*ps).dpy,
                    get_tgt_window(ps),
                );
                if !pf.is_null() {
                    let mut i: libc::c_int = 0 as libc::c_int;
                    while i < (*pf).nfilter {
                        if strcmp(
                            *((*pf).filter).offset(i as isize),
                            b"convolution\0" as *const u8 as *const libc::c_char,
                        ) == 0
                        {
                            (*ps).xrfilter_convolution_exists = 1 as libc::c_int != 0;
                        }
                        i += 1;
                        i;
                    }
                }
                cxfree(pf as *mut libc::c_void);
                if !(*ps).xrfilter_convolution_exists {
                    fprintf(
                        stderr,
                        b"%s(): X Render convolution filter unsupported by your X server. Background blur is not possible.\n\0"
                            as *const u8 as *const libc::c_char,
                        (*::std::mem::transmute::<
                            &[u8; 13],
                            &[libc::c_char; 13],
                        >(b"init_filters\0"))
                            .as_ptr(),
                    );
                    return 0 as libc::c_int != 0;
                }
            }
            _ => {}
        }
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn redir_start(mut ps: *mut session_t) {
    if !(*ps).redirected {
        if (*ps).overlay != 0 {
            XMapWindow((*ps).dpy, (*ps).overlay);
        }
        XCompositeRedirectSubwindows((*ps).dpy, (*ps).root, 1 as libc::c_int);
        XSync((*ps).dpy, 0 as libc::c_int);
        (*ps).redirected = 1 as libc::c_int != 0;
        force_repaint(ps);
    }
}
unsafe extern "C" fn timeout_get_poll_time(mut ps: *mut session_t) -> time_ms_t {
    let now: time_ms_t = get_time_ms();
    let mut wait: time_ms_t = 9223372036854775807 as libc::c_long;
    let mut ptmout: *mut timeout_t = (*ps).tmout_lst;
    while !ptmout.is_null() {
        if (*ptmout).enabled {
            let mut newrun: time_ms_t = timeout_get_newrun(ptmout);
            if newrun <= now {
                wait = 0 as libc::c_int as time_ms_t;
                break;
            } else {
                let mut newwait: time_ms_t = newrun - now;
                if newwait < wait {
                    wait = newwait;
                }
            }
        }
        ptmout = (*ptmout).next;
    }
    return wait;
}
pub unsafe extern "C" fn timeout_insert(
    mut ps: *mut session_t,
    mut interval: time_ms_t,
    mut callback: Option::<unsafe extern "C" fn(*mut session_t, *mut timeout_t) -> bool>,
    mut data: *mut libc::c_void,
) -> *mut timeout_t {
    static mut tmout_def: timeout_t = {
        let mut init = _timeout_t {
            enabled: 1 as libc::c_int != 0,
            data: 0 as *const libc::c_void as *mut libc::c_void,
            callback: None,
            interval: 0 as libc::c_long,
            firstrun: 0 as libc::c_long,
            lastrun: 0 as libc::c_long,
            next: 0 as *const _timeout_t as *mut _timeout_t,
        };
        init
    };
    let now: time_ms_t = get_time_ms();
    let mut ptmout: *mut timeout_t = malloc(
        ::std::mem::size_of::<timeout_t>() as libc::c_ulong,
    ) as *mut timeout_t;
    if ptmout.is_null() {
        fprintf(
            stderr,
            b"%s(): Failed to allocate memory for timeout.\n\0" as *const u8
                as *const libc::c_char,
            (*::std::mem::transmute::<
                &[u8; 15],
                &[libc::c_char; 15],
            >(b"timeout_insert\0"))
                .as_ptr(),
        );
        exit(1 as libc::c_int);
    }
    memcpy(
        ptmout as *mut libc::c_void,
        &tmout_def as *const timeout_t as *const libc::c_void,
        ::std::mem::size_of::<timeout_t>() as libc::c_ulong,
    );
    (*ptmout).interval = interval;
    (*ptmout).firstrun = now;
    (*ptmout).lastrun = now;
    (*ptmout).data = data;
    (*ptmout).callback = callback;
    (*ptmout).next = (*ps).tmout_lst;
    (*ps).tmout_lst = ptmout;
    return ptmout;
}
pub unsafe extern "C" fn timeout_drop(
    mut ps: *mut session_t,
    mut prm: *mut timeout_t,
) -> bool {
    let mut pplast: *mut *mut timeout_t = &mut (*ps).tmout_lst;
    let mut ptmout: *mut timeout_t = (*ps).tmout_lst;
    while !ptmout.is_null() {
        if prm == ptmout {
            *pplast = (*ptmout).next;
            free(ptmout as *mut libc::c_void);
            return 1 as libc::c_int != 0;
        }
        pplast = &mut (*ptmout).next;
        ptmout = (*ptmout).next;
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn timeout_clear(mut ps: *mut session_t) {
    let mut ptmout: *mut timeout_t = (*ps).tmout_lst;
    let mut next: *mut timeout_t = 0 as *mut timeout_t;
    while !ptmout.is_null() {
        next = (*ptmout).next;
        free(ptmout as *mut libc::c_void);
        ptmout = next;
    }
}
unsafe extern "C" fn timeout_run(mut ps: *mut session_t) -> bool {
    let now: time_ms_t = get_time_ms();
    let mut ret: bool = 0 as libc::c_int != 0;
    let mut pnext: *mut timeout_t = 0 as *mut timeout_t;
    let mut ptmout: *mut timeout_t = (*ps).tmout_lst;
    while !ptmout.is_null() {
        pnext = (*ptmout).next;
        if (*ptmout).enabled {
            let max: time_ms_t = now
                + ((*ptmout).interval as libc::c_double * 0.05f64) as time_ms_t;
            let mut newrun: time_ms_t = timeout_get_newrun(ptmout);
            if newrun <= max {
                ret = 1 as libc::c_int != 0;
                timeout_invoke(ps, ptmout);
            }
        }
        ptmout = pnext;
    }
    return ret;
}
pub unsafe extern "C" fn timeout_invoke(
    mut ps: *mut session_t,
    mut ptmout: *mut timeout_t,
) {
    let now: time_ms_t = get_time_ms();
    (*ptmout).lastrun = now;
    if ((*ptmout).callback).is_some() {
        ((*ptmout).callback).unwrap()(ps, ptmout);
    }
}
pub unsafe extern "C" fn timeout_reset(
    mut ps: *mut session_t,
    mut ptmout: *mut timeout_t,
) {
    (*ptmout).lastrun = get_time_ms();
    (*ptmout).firstrun = (*ptmout).lastrun;
}
unsafe extern "C" fn redir_stop(mut ps: *mut session_t) {
    if (*ps).redirected {
        let mut w: *mut win = (*ps).list;
        while !w.is_null() {
            free_wpaint(ps, w);
            w = (*w).next;
        }
        XCompositeUnredirectSubwindows((*ps).dpy, (*ps).root, 1 as libc::c_int);
        if (*ps).overlay != 0 {
            XUnmapWindow((*ps).dpy, (*ps).overlay);
        }
        XSync((*ps).dpy, 0 as libc::c_int);
        (*ps).redirected = 0 as libc::c_int != 0;
    }
}
unsafe extern "C" fn tmout_unredir_callback(
    mut ps: *mut session_t,
    mut tmout: *mut timeout_t,
) -> bool {
    (*ps).tmout_unredir_hit = 1 as libc::c_int != 0;
    (*tmout).enabled = 0 as libc::c_int != 0;
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn mainloop(mut ps: *mut session_t) -> bool {
    timeout_run(ps);
    if XEventsQueued((*ps).dpy, 1 as libc::c_int) != 0 {
        let mut ev: XEvent = _XEvent { type_0: 0 };
        XNextEvent((*ps).dpy, &mut ev);
        ev_handle(ps, &mut ev);
        (*ps).ev_received = 1 as libc::c_int != 0;
        return 1 as libc::c_int != 0;
    }
    if (*ps).reset {
        return 0 as libc::c_int != 0;
    }
    let mut ptv: *mut timeval = 0 as *mut timeval;
    if (*ps).ev_received as libc::c_int != 0 || (*ps).o.benchmark != 0 {
        ptv = malloc(::std::mem::size_of::<timeval>() as libc::c_ulong) as *mut timeval;
        (*ptv).tv_sec = 0 as libc::c_long;
        (*ptv).tv_usec = 0 as libc::c_long;
    } else if !(*ps).idling {
        ptv = malloc(::std::mem::size_of::<timeval>() as libc::c_ulong) as *mut timeval;
        *ptv = ms_to_tv(fade_timeout(ps));
    }
    if !ptv.is_null() && (*ps).o.sw_opti as libc::c_int != 0 {
        swopti_handle_timeout(ps, ptv);
    }
    if !ptv.is_null() && timeval_isempty(ptv) as libc::c_int != 0 {
        free(ptv as *mut libc::c_void);
        return 0 as libc::c_int != 0;
    }
    let mut tmout_ms: time_ms_t = timeout_get_poll_time(ps);
    if tmout_ms < 9223372036854775807 as libc::c_long {
        if ptv.is_null() {
            ptv = malloc(::std::mem::size_of::<timeval>() as libc::c_ulong)
                as *mut timeval;
            *ptv = ms_to_tv(tmout_ms as libc::c_int);
        } else if timeval_ms_cmp(ptv, tmout_ms) > 0 as libc::c_int {
            *ptv = ms_to_tv(tmout_ms as libc::c_int);
        }
    }
    if !ptv.is_null() && timeval_isempty(ptv) as libc::c_int != 0 {
        free(ptv as *mut libc::c_void);
        return 0 as libc::c_int != 0;
    }
    fds_poll(ps, ptv);
    free(ptv as *mut libc::c_void);
    ptv = 0 as *mut timeval;
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn cxinerama_upd_scrs(mut ps: *mut session_t) {}
unsafe extern "C" fn session_init(
    mut ps_old: *mut session_t,
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> *mut session_t {
    static mut s_def: session_t = {
        let mut init = _session_t {
            dpy: 0 as *const Display as *mut Display,
            scr: 0 as libc::c_int,
            vis: 0 as *const Visual as *mut Visual,
            depth: 0 as libc::c_int,
            root: 0 as libc::c_long as Window,
            root_height: 0 as libc::c_int,
            root_width: 0 as libc::c_int,
            overlay: 0 as libc::c_long as Window,
            root_tile_fill: 0 as libc::c_int != 0,
            root_tile_paint: {
                let mut init = paint_t {
                    pixmap: 0 as libc::c_long as Pixmap,
                    pict: 0 as libc::c_long as Picture,
                    ptex: 0 as *const glx_texture_t as *mut glx_texture_t,
                };
                init
            },
            screen_reg: 0 as libc::c_long as XserverRegion,
            root_picture: 0,
            tgt_picture: 0 as libc::c_long as Picture,
            tgt_buffer: {
                let mut init = paint_t {
                    pixmap: 0 as libc::c_long as Pixmap,
                    pict: 0 as libc::c_long as Picture,
                    ptex: 0 as *const glx_texture_t as *mut glx_texture_t,
                };
                init
            },
            tgt_buffer_fence: 0,
            root_dbe: 0 as libc::c_long as XdbeBackBuffer,
            reg_win: 0 as libc::c_long as Window,
            o: {
                let mut init = _options_t {
                    config_file: 0 as *const libc::c_char as *mut libc::c_char,
                    write_pid_path: 0 as *const libc::c_char as *mut libc::c_char,
                    display: 0 as *const libc::c_char as *mut libc::c_char,
                    display_repr: 0 as *const libc::c_char as *mut libc::c_char,
                    backend: BKEND_XRENDER,
                    xrender_sync: false,
                    xrender_sync_fence: false,
                    glx_no_stencil: 0 as libc::c_int != 0,
                    glx_copy_from_front: 0 as libc::c_int != 0,
                    glx_use_copysubbuffermesa: false,
                    glx_no_rebind_pixmap: false,
                    glx_swap_method: 0,
                    glx_use_gpushader4: false,
                    glx_fshader_win_str: 0 as *const libc::c_char as *mut libc::c_char,
                    fork_after_register: 0 as libc::c_int != 0,
                    detect_rounded_corners: 0 as libc::c_int != 0,
                    paint_on_overlay: 0 as libc::c_int != 0,
                    force_win_blend: false,
                    resize_damage: 0 as libc::c_int,
                    unredir_if_possible: 0 as libc::c_int != 0,
                    unredir_if_possible_blacklist: 0 as *const c2_lptr_t
                        as *mut c2_lptr_t,
                    unredir_if_possible_delay: 0 as libc::c_int as time_ms_t,
                    redirected_force: UNSET,
                    stoppaint_force: UNSET,
                    reredir_on_root_change: false,
                    glx_reinit_on_root_change: false,
                    dbus: 0 as libc::c_int != 0,
                    logpath: 0 as *const libc::c_char as *mut libc::c_char,
                    benchmark: 0 as libc::c_int,
                    benchmark_wid: 0 as libc::c_long as Window,
                    paint_blacklist: 0 as *const c2_lptr_t as *mut c2_lptr_t,
                    no_name_pixmap: false,
                    synchronize: 0 as libc::c_int != 0,
                    show_all_xerrors: false,
                    no_x_selection: false,
                    refresh_rate: 0 as libc::c_int,
                    sw_opti: 0 as libc::c_int != 0,
                    vsync: VSYNC_NONE,
                    dbe: 0 as libc::c_int != 0,
                    vsync_aggressive: 0 as libc::c_int != 0,
                    vsync_use_glfinish: false,
                    wintype_shadow: [
                        0 as libc::c_int != 0,
                        false,
                        false,
                        false,
                        false,
                        false,
                        false,
                        false,
                        false,
                        false,
                        false,
                        false,
                        false,
                        false,
                        false,
                    ],
                    shadow_red: 0.0f64,
                    shadow_green: 0.0f64,
                    shadow_blue: 0.0f64,
                    shadow_radius: 12 as libc::c_int,
                    shadow_offset_x: -(15 as libc::c_int),
                    shadow_offset_y: -(15 as libc::c_int),
                    shadow_opacity: 0.75f64,
                    clear_shadow: 0 as libc::c_int != 0,
                    shadow_exclude_reg_geom: geometry_t {
                        wid: 0,
                        hei: 0,
                        x: 0,
                        y: 0,
                    },
                    shadow_blacklist: 0 as *const c2_lptr_t as *mut c2_lptr_t,
                    shadow_ignore_shaped: 0 as libc::c_int != 0,
                    respect_prop_shadow: 0 as libc::c_int != 0,
                    xinerama_shadow_crop: 0 as libc::c_int != 0,
                    wintype_fade: [
                        0 as libc::c_int != 0,
                        false,
                        false,
                        false,
                        false,
                        false,
                        false,
                        false,
                        false,
                        false,
                        false,
                        false,
                        false,
                        false,
                        false,
                    ],
                    fade_in_step: (0.028f64
                        * 0xffffffff as libc::c_uint as libc::c_double) as opacity_t,
                    fade_out_step: (0.03f64
                        * 0xffffffff as libc::c_uint as libc::c_double) as opacity_t,
                    fade_delta: 10 as libc::c_int as time_ms_t,
                    no_fading_openclose: 0 as libc::c_int != 0,
                    no_fading_destroyed_argb: 0 as libc::c_int != 0,
                    fade_blacklist: 0 as *const c2_lptr_t as *mut c2_lptr_t,
                    wintype_opacity: [
                        0.0f64,
                        0.,
                        0.,
                        0.,
                        0.,
                        0.,
                        0.,
                        0.,
                        0.,
                        0.,
                        0.,
                        0.,
                        0.,
                        0.,
                        0.,
                    ],
                    inactive_opacity: 0 as libc::c_int as opacity_t,
                    active_opacity: 0 as libc::c_int as opacity_t,
                    inactive_opacity_override: 0 as libc::c_int != 0,
                    frame_opacity: 0.0f64,
                    detect_client_opacity: 0 as libc::c_int != 0,
                    alpha_step: 0.03f64,
                    blur_background: 0 as libc::c_int != 0,
                    blur_background_frame: 0 as libc::c_int != 0,
                    blur_background_fixed: 0 as libc::c_int != 0,
                    blur_background_blacklist: 0 as *const c2_lptr_t as *mut c2_lptr_t,
                    blur_kerns: [
                        0 as *const XFixed as *mut XFixed,
                        0 as *const XFixed as *mut XFixed,
                        0 as *const XFixed as *mut XFixed,
                        0 as *const XFixed as *mut XFixed,
                        0 as *const XFixed as *mut XFixed,
                    ],
                    inactive_dim: 0.0f64,
                    inactive_dim_fixed: 0 as libc::c_int != 0,
                    invert_color_list: 0 as *const c2_lptr_t as *mut c2_lptr_t,
                    opacity_rules: 0 as *const c2_lptr_t as *mut c2_lptr_t,
                    wintype_focus: [
                        0 as libc::c_int != 0,
                        false,
                        false,
                        false,
                        false,
                        false,
                        false,
                        false,
                        false,
                        false,
                        false,
                        false,
                        false,
                        false,
                        false,
                    ],
                    mark_wmwin_focused: 0 as libc::c_int != 0,
                    mark_ovredir_focused: 0 as libc::c_int != 0,
                    use_ewmh_active_win: 0 as libc::c_int != 0,
                    focus_blacklist: 0 as *const c2_lptr_t as *mut c2_lptr_t,
                    detect_transient: 0 as libc::c_int != 0,
                    detect_client_leader: 0 as libc::c_int != 0,
                    track_focus: 0 as libc::c_int != 0,
                    track_wdata: 0 as libc::c_int != 0,
                    track_leader: 0 as libc::c_int != 0,
                };
                init
            },
            pfds_read: 0 as *const fd_set as *mut fd_set,
            pfds_write: 0 as *const fd_set as *mut fd_set,
            pfds_except: 0 as *const fd_set as *mut fd_set,
            nfds_max: 0 as libc::c_int,
            tmout_lst: 0 as *const _timeout_t as *mut _timeout_t,
            tmout_unredir: 0 as *const _timeout_t as *mut _timeout_t,
            tmout_unredir_hit: false,
            ev_received: false,
            idling: 0 as libc::c_int != 0,
            time_start: {
                let mut init = timeval {
                    tv_sec: 0 as libc::c_int as __time_t,
                    tv_usec: 0 as libc::c_int as __suseconds_t,
                };
                init
            },
            all_damage: 0 as libc::c_long as XserverRegion,
            all_damage_last: [0 as libc::c_long as XserverRegion, 0, 0, 0, 0],
            redirected: 0 as libc::c_int != 0,
            alpha_picts: 0 as *const Picture as *mut Picture,
            reg_ignore_expire: 0 as libc::c_int != 0,
            fade_time: 0 as libc::c_long,
            ignore_head: 0 as *const ignore_t as *mut ignore_t,
            ignore_tail: 0 as *const *mut ignore_t as *mut *mut ignore_t,
            blur_kerns_cache: [0 as *const XFixed as *mut XFixed; 5],
            reset: 0 as libc::c_int != 0,
            expose_rects: 0 as *const XRectangle as *mut XRectangle,
            size_expose: 0 as libc::c_int,
            n_expose: 0 as libc::c_int,
            list: 0 as *const _win as *mut _win,
            active_win: 0 as *const _win as *mut _win,
            active_leader: 0 as libc::c_long as Window,
            black_picture: 0 as libc::c_long as Picture,
            cshadow_picture: 0 as libc::c_long as Picture,
            white_picture: 0 as libc::c_long as Picture,
            gaussian_map: 0 as *const conv as *mut conv,
            cgsize: 0 as libc::c_int,
            shadow_corner: 0 as *const libc::c_uchar as *mut libc::c_uchar,
            shadow_top: 0 as *const libc::c_uchar as *mut libc::c_uchar,
            shadow_exclude_reg: 0,
            refresh_rate: 0 as libc::c_int as libc::c_short,
            refresh_intv: 0 as libc::c_ulong as libc::c_long,
            paint_tm_offset: 0 as libc::c_long,
            xfixes_event: 0 as libc::c_int,
            xfixes_error: 0 as libc::c_int,
            damage_event: 0 as libc::c_int,
            damage_error: 0 as libc::c_int,
            render_event: 0 as libc::c_int,
            render_error: 0 as libc::c_int,
            composite_event: 0 as libc::c_int,
            composite_error: 0 as libc::c_int,
            composite_opcode: 0 as libc::c_int,
            has_name_pixmap: 0 as libc::c_int != 0,
            shape_exists: 0 as libc::c_int != 0,
            shape_event: 0 as libc::c_int,
            shape_error: 0 as libc::c_int,
            randr_exists: 0 as libc::c_int != 0,
            randr_event: 0 as libc::c_int,
            randr_error: 0 as libc::c_int,
            dbe_exists: 0 as libc::c_int != 0,
            xsync_exists: false,
            xsync_event: 0,
            xsync_error: 0,
            xrfilter_convolution_exists: 0 as libc::c_int != 0,
            atom_opacity: 0 as libc::c_long as Atom,
            atom_frame_extents: 0 as libc::c_long as Atom,
            atom_client: 0 as libc::c_long as Atom,
            atom_name: 0 as libc::c_long as Atom,
            atom_name_ewmh: 0 as libc::c_long as Atom,
            atom_class: 0 as libc::c_long as Atom,
            atom_role: 0 as libc::c_long as Atom,
            atom_transient: 0 as libc::c_long as Atom,
            atom_client_leader: 0,
            atom_ewmh_active_win: 0 as libc::c_long as Atom,
            atom_compton_shadow: 0 as libc::c_long as Atom,
            atom_win_type: 0 as libc::c_long as Atom,
            atoms_wintypes: [
                0 as libc::c_int as Atom,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            track_atom_lst: 0 as *const latom_t as *mut latom_t,
        };
        init
    };
    let mut ps: *mut session_t = malloc(
        ::std::mem::size_of::<session_t>() as libc::c_ulong,
    ) as *mut session_t;
    memcpy(
        ps as *mut libc::c_void,
        &s_def as *const session_t as *const libc::c_void,
        ::std::mem::size_of::<session_t>() as libc::c_ulong,
    );
    ps_g = ps;
    (*ps).ignore_tail = &mut (*ps).ignore_head;
    gettimeofday(&mut (*ps).time_start, 0 as *mut libc::c_void);
    wintype_arr_enable(((*ps).o.wintype_focus).as_mut_ptr());
    (*ps)
        .o
        .wintype_focus[WINTYPE_UNKNOWN as libc::c_int as usize] = 0 as libc::c_int != 0;
    (*ps)
        .o
        .wintype_focus[WINTYPE_NORMAL as libc::c_int as usize] = 0 as libc::c_int != 0;
    (*ps)
        .o
        .wintype_focus[WINTYPE_UTILITY as libc::c_int as usize] = 0 as libc::c_int != 0;
    get_cfg(ps, argc, argv, 1 as libc::c_int != 0);
    if !ps_old.is_null() && !((*ps_old).dpy).is_null() {
        (*ps).dpy = (*ps_old).dpy;
    }
    if ((*ps).dpy).is_null() {
        (*ps).dpy = XOpenDisplay((*ps).o.display);
        if ((*ps).dpy).is_null() {
            fprintf(
                stderr,
                b"%s(): Can't open display.\n\0" as *const u8 as *const libc::c_char,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"session_init\0"))
                    .as_ptr(),
            );
            exit(1 as libc::c_int);
        }
    }
    XSetErrorHandler(
        Some(
            xerror as unsafe extern "C" fn(*mut Display, *mut XErrorEvent) -> libc::c_int,
        ),
    );
    if (*ps).o.synchronize {
        XSynchronize((*ps).dpy, 1 as libc::c_int);
    }
    (*ps).scr = (*((*ps).dpy as _XPrivDisplay)).default_screen;
    (*ps)
        .root = (*((*((*ps).dpy as _XPrivDisplay)).screens).offset((*ps).scr as isize))
        .root;
    (*ps)
        .vis = (*((*((*ps).dpy as _XPrivDisplay)).screens).offset((*ps).scr as isize))
        .root_visual;
    (*ps)
        .depth = (*((*((*ps).dpy as _XPrivDisplay)).screens).offset((*ps).scr as isize))
        .root_depth;
    XSelectInput(
        (*ps).dpy,
        (*ps).root,
        (1 as libc::c_long) << 19 as libc::c_int
            | (1 as libc::c_long) << 15 as libc::c_int
            | (1 as libc::c_long) << 17 as libc::c_int
            | (1 as libc::c_long) << 22 as libc::c_int,
    );
    XFlush((*ps).dpy);
    (*ps)
        .root_width = (*((*((*ps).dpy as _XPrivDisplay)).screens)
        .offset((*ps).scr as isize))
        .width;
    (*ps)
        .root_height = (*((*((*ps).dpy as _XPrivDisplay)).screens)
        .offset((*ps).scr as isize))
        .height;
    if XRenderQueryExtension((*ps).dpy, &mut (*ps).render_event, &mut (*ps).render_error)
        == 0
    {
        fprintf(stderr, b"No render extension\n\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    if XQueryExtension(
        (*ps).dpy,
        b"Composite\0" as *const u8 as *const libc::c_char,
        &mut (*ps).composite_opcode,
        &mut (*ps).composite_event,
        &mut (*ps).composite_error,
    ) == 0
    {
        fprintf(
            stderr,
            b"No composite extension\n\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    let mut composite_major: libc::c_int = 0 as libc::c_int;
    let mut composite_minor: libc::c_int = 0 as libc::c_int;
    XCompositeQueryVersion((*ps).dpy, &mut composite_major, &mut composite_minor);
    if !(*ps).o.no_name_pixmap
        && (composite_major > 0 as libc::c_int || composite_minor >= 2 as libc::c_int)
    {
        (*ps).has_name_pixmap = 1 as libc::c_int != 0;
    }
    if XDamageQueryExtension((*ps).dpy, &mut (*ps).damage_event, &mut (*ps).damage_error)
        == 0
    {
        fprintf(stderr, b"No damage extension\n\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    if XFixesQueryExtension((*ps).dpy, &mut (*ps).xfixes_event, &mut (*ps).xfixes_error)
        == 0
    {
        fprintf(stderr, b"No XFixes extension\n\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    let mut display_repr: *mut libc::c_char = (*((*ps).dpy as _XPrivDisplay))
        .display_name;
    if display_repr.is_null() {
        display_repr = b"unknown\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char;
    }
    display_repr = mstrcpy(display_repr);
    let mut pdisp: *mut libc::c_char = display_repr;
    while *pdisp != 0 {
        if *(*__ctype_b_loc()).offset(*pdisp as libc::c_int as isize) as libc::c_int
            & _ISalnum as libc::c_int as libc::c_ushort as libc::c_int == 0
        {
            *pdisp = '_' as i32 as libc::c_char;
        }
        pdisp = pdisp.offset(1);
        pdisp;
    }
    (*ps).o.display_repr = display_repr;
    get_cfg(ps, argc, argv, 0 as libc::c_int != 0);
    if XShapeQueryExtension((*ps).dpy, &mut (*ps).shape_event, &mut (*ps).shape_error)
        != 0
    {
        (*ps).shape_exists = 1 as libc::c_int != 0;
    }
    if (*ps).o.xrender_sync_fence {
        if XSyncQueryExtension((*ps).dpy, &mut (*ps).xsync_event, &mut (*ps).xsync_error)
            != 0
        {
            let mut major_version_return: libc::c_int = 0 as libc::c_int;
            let mut minor_version_return: libc::c_int = 0 as libc::c_int;
            if XSyncInitialize(
                (*ps).dpy,
                &mut major_version_return,
                &mut minor_version_return,
            ) != 0
            {
                (*ps).xsync_exists = 1 as libc::c_int != 0;
            }
        }
        if !(*ps).xsync_exists {
            fprintf(
                stderr,
                b"%s(): X Sync extension not found. No X Sync fence sync is possible.\n\0"
                    as *const u8 as *const libc::c_char,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"session_init\0"))
                    .as_ptr(),
            );
            exit(1 as libc::c_int);
        }
    }
    if (*ps).o.sw_opti as libc::c_int != 0 && (*ps).o.refresh_rate == 0
        || (*ps).o.xinerama_shadow_crop as libc::c_int != 0
    {
        if XRRQueryExtension((*ps).dpy, &mut (*ps).randr_event, &mut (*ps).randr_error)
            != 0
        {
            (*ps).randr_exists = 1 as libc::c_int != 0;
        } else {
            fprintf(
                stderr,
                b"%s(): No XRandR extension, automatic screen change detection impossible.\n\0"
                    as *const u8 as *const libc::c_char,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"session_init\0"))
                    .as_ptr(),
            );
        }
    }
    if (*ps).o.dbe {
        let mut dbe_ver_major: libc::c_int = 0 as libc::c_int;
        let mut dbe_ver_minor: libc::c_int = 0 as libc::c_int;
        if XdbeQueryExtension((*ps).dpy, &mut dbe_ver_major, &mut dbe_ver_minor) != 0 {
            if dbe_ver_major >= 1 as libc::c_int {
                (*ps).dbe_exists = 1 as libc::c_int != 0;
            } else {
                fprintf(
                    stderr,
                    b"DBE extension version too low. Double buffering impossible.\n\0"
                        as *const u8 as *const libc::c_char,
                );
            }
        } else {
            fprintf(
                stderr,
                b"No DBE extension. Double buffering impossible.\n\0" as *const u8
                    as *const libc::c_char,
            );
        }
        if !(*ps).dbe_exists {
            (*ps).o.dbe = 0 as libc::c_int != 0;
        }
    }
    if (*ps).o.xinerama_shadow_crop {
        fprintf(
            stderr,
            b"%s(): Xinerama support not compiled in.\n\0" as *const u8
                as *const libc::c_char,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"session_init\0"))
                .as_ptr(),
        );
    }
    rebuild_screen_reg(ps);
    if (*ps).o.paint_on_overlay {
        init_overlay(ps);
    }
    if (*ps).o.dbe as libc::c_int != 0
        && BKEND_XRENDER as libc::c_int as libc::c_uint
            != (*ps).o.backend as libc::c_uint
    {
        fprintf(
            stderr,
            b"%s(): DBE couldn't be used on GLX backend.\n\0" as *const u8
                as *const libc::c_char,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"session_init\0"))
                .as_ptr(),
        );
        (*ps).o.dbe = 0 as libc::c_int != 0;
    }
    if (*ps).o.dbe as libc::c_int != 0 && !init_dbe(ps) {
        exit(1 as libc::c_int);
    }
    if bkend_use_glx(ps) {
        fprintf(
            stderr,
            b"%s(): GLX backend support not compiled in.\n\0" as *const u8
                as *const libc::c_char,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"session_init\0"))
                .as_ptr(),
        );
        exit(1 as libc::c_int);
    }
    if BKEND_GLX as libc::c_int as libc::c_uint == (*ps).o.backend as libc::c_uint
        && !((*ps).o.glx_fshader_win_str).is_null()
    {
        fprintf(
            stderr,
            b"%s(): GLSL supported not compiled in, can't load shader.\n\0" as *const u8
                as *const libc::c_char,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"session_init\0"))
                .as_ptr(),
        );
        exit(1 as libc::c_int);
    }
    if (*ps).o.sw_opti {
        (*ps).o.sw_opti = swopti_init(ps);
    }
    if (*ps).randr_exists as libc::c_int != 0
        && ((*ps).o.sw_opti as libc::c_int != 0 && (*ps).o.refresh_rate == 0
            || (*ps).o.xinerama_shadow_crop as libc::c_int != 0)
    {
        XRRSelectInput(
            (*ps).dpy,
            (*ps).root,
            ((1 as libc::c_long) << 0 as libc::c_int) as libc::c_int,
        );
    }
    if !vsync_init(ps) {
        exit(1 as libc::c_int);
    }
    cxinerama_upd_scrs(ps);
    if (*ps).reg_win == 0 && !register_cm(ps) {
        exit(1 as libc::c_int);
    }
    init_atoms(ps);
    init_alpha_picts(ps);
    (*ps).gaussian_map = make_gaussian_map((*ps).o.shadow_radius as libc::c_double);
    presum_gaussian(ps, (*ps).gaussian_map);
    let mut pa: XRenderPictureAttributes = XRenderPictureAttributes {
        repeat: 0,
        alpha_map: 0,
        alpha_x_origin: 0,
        alpha_y_origin: 0,
        clip_x_origin: 0,
        clip_y_origin: 0,
        clip_mask: 0,
        graphics_exposures: 0,
        subwindow_mode: 0,
        poly_edge: 0,
        poly_mode: 0,
        dither: 0,
        component_alpha: 0,
    };
    pa.subwindow_mode = 1 as libc::c_int;
    (*ps)
        .root_picture = XRenderCreatePicture(
        (*ps).dpy,
        (*ps).root,
        XRenderFindVisualFormat((*ps).dpy, (*ps).vis),
        ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_ulong,
        &mut pa,
    );
    if (*ps).o.paint_on_overlay {
        (*ps)
            .tgt_picture = XRenderCreatePicture(
            (*ps).dpy,
            (*ps).overlay,
            XRenderFindVisualFormat((*ps).dpy, (*ps).vis),
            ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_ulong,
            &mut pa,
        );
    } else {
        (*ps).tgt_picture = (*ps).root_picture;
    }
    if !init_filters(ps) {
        exit(1 as libc::c_int);
    }
    (*ps)
        .black_picture = solid_picture(
        ps,
        1 as libc::c_int != 0,
        1 as libc::c_int as libc::c_double,
        0 as libc::c_int as libc::c_double,
        0 as libc::c_int as libc::c_double,
        0 as libc::c_int as libc::c_double,
    );
    (*ps)
        .white_picture = solid_picture(
        ps,
        1 as libc::c_int != 0,
        1 as libc::c_int as libc::c_double,
        1 as libc::c_int as libc::c_double,
        1 as libc::c_int as libc::c_double,
        1 as libc::c_int as libc::c_double,
    );
    if (*ps).o.shadow_red == 0. && (*ps).o.shadow_green == 0.
        && (*ps).o.shadow_blue == 0.
    {
        (*ps).cshadow_picture = (*ps).black_picture;
    } else {
        (*ps)
            .cshadow_picture = solid_picture(
            ps,
            1 as libc::c_int != 0,
            1 as libc::c_int as libc::c_double,
            (*ps).o.shadow_red,
            (*ps).o.shadow_green,
            (*ps).o.shadow_blue,
        );
    }
    fds_insert(
        ps,
        (*((*ps).dpy as _XPrivDisplay)).fd,
        0x1 as libc::c_int as libc::c_short,
    );
    (*ps)
        .tmout_unredir = timeout_insert(
        ps,
        (*ps).o.unredir_if_possible_delay,
        Some(
            tmout_unredir_callback
                as unsafe extern "C" fn(*mut session_t, *mut timeout_t) -> bool,
        ),
        0 as *mut libc::c_void,
    );
    (*(*ps).tmout_unredir).enabled = 0 as libc::c_int != 0;
    XGrabServer((*ps).dpy);
    let mut root_return: Window = 0;
    let mut parent_return: Window = 0;
    let mut children: *mut Window = 0 as *mut Window;
    let mut nchildren: libc::c_uint = 0;
    XQueryTree(
        (*ps).dpy,
        (*ps).root,
        &mut root_return,
        &mut parent_return,
        &mut children,
        &mut nchildren,
    );
    let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while i < nchildren {
        add_win(
            ps,
            *children.offset(i as isize),
            if i != 0 {
                *children
                    .offset(i.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize)
            } else {
                0 as libc::c_long as libc::c_ulong
            },
        );
        i = i.wrapping_add(1);
        i;
    }
    cxfree(children as *mut libc::c_void);
    if (*ps).o.track_focus {
        recheck_focus(ps);
    }
    XUngrabServer((*ps).dpy);
    XFlush((*ps).dpy);
    if (*ps).o.dbus {
        fprintf(
            stderr,
            b"%s(): DBus support not compiled in!\n\0" as *const u8
                as *const libc::c_char,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"session_init\0"))
                .as_ptr(),
        );
        exit(1 as libc::c_int);
    }
    if (*ps).o.fork_after_register {
        if !fork_after(ps) {
            session_destroy(ps);
            return 0 as *mut session_t;
        }
    }
    if (*ps).o.fork_after_register as libc::c_int != 0 || !((*ps).o.logpath).is_null() {
        ostream_reopen(ps, 0 as *const libc::c_char);
    }
    write_pid(ps);
    if !ps_old.is_null() {
        free(ps_old as *mut libc::c_void);
    }
    return ps;
}
unsafe extern "C" fn session_destroy(mut ps: *mut session_t) {
    redir_stop(ps);
    XSelectInput((*ps).dpy, (*ps).root, 0 as libc::c_int as libc::c_long);
    let mut next: *mut win = 0 as *mut win;
    let mut w: *mut win = (*ps).list;
    while !w.is_null() {
        next = (*w).next;
        if 2 as libc::c_int == (*w).a.map_state && !(*w).destroyed {
            win_ev_stop(ps, w);
        }
        free_win_res(ps, w);
        free(w as *mut libc::c_void);
        w = next;
    }
    (*ps).list = 0 as *mut _win;
    let max: libc::c_int = (round(1.0f64 / (*ps).o.alpha_step)
        + 1 as libc::c_int as libc::c_double) as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < max {
        free_picture(ps, &mut *((*ps).alpha_picts).offset(i as isize));
        i += 1;
        i;
    }
    free((*ps).alpha_picts as *mut libc::c_void);
    (*ps).alpha_picts = 0 as *mut Picture;
    free_wincondlst(&mut (*ps).o.shadow_blacklist);
    free_wincondlst(&mut (*ps).o.fade_blacklist);
    free_wincondlst(&mut (*ps).o.focus_blacklist);
    free_wincondlst(&mut (*ps).o.invert_color_list);
    free_wincondlst(&mut (*ps).o.blur_background_blacklist);
    free_wincondlst(&mut (*ps).o.opacity_rules);
    free_wincondlst(&mut (*ps).o.paint_blacklist);
    free_wincondlst(&mut (*ps).o.unredir_if_possible_blacklist);
    let mut next_0: *mut latom_t = 0 as *mut latom_t;
    let mut this: *mut latom_t = (*ps).track_atom_lst;
    while !this.is_null() {
        next_0 = (*this).next;
        free(this as *mut libc::c_void);
        this = next_0;
    }
    (*ps).track_atom_lst = 0 as *mut latom_t;
    let mut next_1: *mut ignore_t = 0 as *mut ignore_t;
    let mut ign: *mut ignore_t = (*ps).ignore_head;
    while !ign.is_null() {
        next_1 = (*ign).next;
        free(ign as *mut libc::c_void);
        ign = next_1;
    }
    (*ps).ignore_head = 0 as *mut ignore_t;
    (*ps).ignore_tail = &mut (*ps).ignore_head;
    if (*ps).cshadow_picture == (*ps).black_picture {
        (*ps).cshadow_picture = 0 as libc::c_long as Picture;
    } else {
        free_picture(ps, &mut (*ps).cshadow_picture);
    }
    free_picture(ps, &mut (*ps).black_picture);
    free_picture(ps, &mut (*ps).white_picture);
    if (*ps).tgt_buffer.pict == (*ps).tgt_picture {
        (*ps).tgt_buffer.pict = 0 as libc::c_long as Picture;
    }
    if (*ps).tgt_picture == (*ps).root_picture {
        (*ps).tgt_picture = 0 as libc::c_long as Picture;
    } else {
        free_picture(ps, &mut (*ps).tgt_picture);
    }
    free_fence(ps, &mut (*ps).tgt_buffer_fence);
    free_picture(ps, &mut (*ps).root_picture);
    free_paint(ps, &mut (*ps).tgt_buffer);
    free_root_tile(ps);
    free_region(ps, &mut (*ps).screen_reg);
    free_region(ps, &mut (*ps).all_damage);
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < 5 as libc::c_int {
        free_region(ps, &mut *((*ps).all_damage_last).as_mut_ptr().offset(i_0 as isize));
        i_0 += 1;
        i_0;
    }
    free((*ps).expose_rects as *mut libc::c_void);
    free((*ps).shadow_corner as *mut libc::c_void);
    free((*ps).shadow_top as *mut libc::c_void);
    free((*ps).gaussian_map as *mut libc::c_void);
    free((*ps).o.config_file as *mut libc::c_void);
    free((*ps).o.write_pid_path as *mut libc::c_void);
    free((*ps).o.display as *mut libc::c_void);
    free((*ps).o.display_repr as *mut libc::c_void);
    free((*ps).o.logpath as *mut libc::c_void);
    let mut i_1: libc::c_int = 0 as libc::c_int;
    while i_1 < 5 as libc::c_int {
        free((*ps).o.blur_kerns[i_1 as usize] as *mut libc::c_void);
        free((*ps).blur_kerns_cache[i_1 as usize] as *mut libc::c_void);
        i_1 += 1;
        i_1;
    }
    free((*ps).pfds_read as *mut libc::c_void);
    free((*ps).pfds_write as *mut libc::c_void);
    free((*ps).pfds_except as *mut libc::c_void);
    free((*ps).o.glx_fshader_win_str as *mut libc::c_void);
    free_xinerama_info(ps);
    if (*ps).root_dbe != 0 {
        XdbeDeallocateBackBufferName((*ps).dpy, (*ps).root_dbe);
        (*ps).root_dbe = 0 as libc::c_long as XdbeBackBuffer;
    }
    if (*ps).overlay != 0 {
        XCompositeReleaseOverlayWindow((*ps).dpy, (*ps).overlay);
        (*ps).overlay = 0 as libc::c_long as Window;
    }
    if (*ps).reg_win != 0 {
        XDestroyWindow((*ps).dpy, (*ps).reg_win);
        (*ps).reg_win = 0 as libc::c_long as Window;
    }
    XSync((*ps).dpy, 1 as libc::c_int);
    (*ps).tmout_unredir = 0 as *mut _timeout_t;
    timeout_clear(ps);
    if ps == ps_g {
        ps_g = 0 as *mut session_t;
    }
}
unsafe extern "C" fn session_run(mut ps: *mut session_t) {
    let mut t: *mut win = 0 as *mut win;
    if (*ps).o.sw_opti {
        (*ps).paint_tm_offset = (get_time_timeval()).tv_usec;
    }
    (*ps).reg_ignore_expire = 1 as libc::c_int != 0;
    t = paint_preprocess(ps, (*ps).list);
    if (*ps).redirected {
        paint_all(
            ps,
            0 as libc::c_long as XserverRegion,
            0 as libc::c_long as XserverRegion,
            t,
        );
    }
    (*ps).idling = 0 as libc::c_int != 0;
    while !(*ps).reset {
        (*ps).ev_received = 0 as libc::c_int != 0;
        while mainloop(ps) {}
        if (*ps).o.benchmark != 0 {
            if (*ps).o.benchmark_wid != 0 {
                let mut w: *mut win = find_win(ps, (*ps).o.benchmark_wid);
                if w.is_null() {
                    fprintf(
                        stderr,
                        b"%s(): Couldn't find specified benchmark window.\n\0"
                            as *const u8 as *const libc::c_char,
                        (*::std::mem::transmute::<
                            &[u8; 12],
                            &[libc::c_char; 12],
                        >(b"session_run\0"))
                            .as_ptr(),
                    );
                    session_destroy(ps);
                    exit(1 as libc::c_int);
                }
                add_damage_win(ps, w);
            } else {
                force_repaint(ps);
            }
        }
        (*ps).idling = 1 as libc::c_int != 0;
        t = paint_preprocess(ps, (*ps).list);
        (*ps).tmout_unredir_hit = 0 as libc::c_int != 0;
        if !(*ps).redirected
            || ON as libc::c_int as libc::c_uint
                == (*ps).o.stoppaint_force as libc::c_uint
        {
            free_region(ps, &mut (*ps).all_damage);
        }
        let mut all_damage_orig: XserverRegion = 0 as libc::c_long as XserverRegion;
        if (*ps).o.resize_damage > 0 as libc::c_int {
            all_damage_orig = copy_region(ps, (*ps).all_damage);
        }
        resize_region(ps, (*ps).all_damage, (*ps).o.resize_damage as libc::c_short);
        if (*ps).all_damage != 0
            && !is_region_empty(ps, (*ps).all_damage, 0 as *mut reg_data_t)
        {
            static mut paint: libc::c_int = 0 as libc::c_int;
            paint_all(ps, (*ps).all_damage, all_damage_orig, t);
            (*ps).reg_ignore_expire = 0 as libc::c_int != 0;
            paint += 1;
            paint;
            if (*ps).o.benchmark != 0 && paint >= (*ps).o.benchmark {
                exit(0 as libc::c_int);
            }
            XSync((*ps).dpy, 0 as libc::c_int);
            (*ps).all_damage = 0 as libc::c_long as XserverRegion;
        }
        free_region(ps, &mut all_damage_orig);
        if (*ps).idling {
            (*ps).fade_time = 0 as libc::c_long;
        }
    }
}
unsafe extern "C" fn reset_enable(mut signum: libc::c_int) {
    let ps: *mut session_t = ps_g;
    (*ps).reset = 1 as libc::c_int != 0;
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    setlocale(6 as libc::c_int, b"\0" as *const u8 as *const libc::c_char);
    let mut block_mask: sigset_t = sigset_t { __val: [0; 16] };
    sigemptyset(&mut block_mask);
    let action: sigaction = {
        let mut init = sigaction {
            __sigaction_handler: C2RustUnnamed_14 {
                sa_handler: Some(reset_enable as unsafe extern "C" fn(libc::c_int) -> ()),
            },
            sa_mask: block_mask,
            sa_flags: 0 as libc::c_int,
            sa_restorer: None,
        };
        init
    };
    sigaction(10 as libc::c_int, &action, 0 as *mut sigaction);
    let mut ps_old: *mut session_t = ps_g;
    loop {
        ps_g = session_init(ps_old, argc, argv);
        if ps_g.is_null() {
            fprintf(
                stderr,
                b"%s(): Failed to create new session.\n\0" as *const u8
                    as *const libc::c_char,
                (*::std::mem::transmute::<&[u8; 5], &[libc::c_char; 5]>(b"main\0"))
                    .as_ptr(),
            );
            return 1 as libc::c_int;
        }
        session_run(ps_g);
        ps_old = ps_g;
        session_destroy(ps_g);
    };
}
pub fn main() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args_os() {
        args.push(
            (::std::ffi::CString::new(
                ::std::os::unix::ffi::OsStrExt::as_bytes(arg.as_os_str()),
            ))
                .unwrap()
                .into_raw(),
        );
    }
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as libc::c_int,
                args.as_mut_ptr() as *mut *mut libc::c_char,
            ) as i32,
        )
    }
}
