use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type _freenect_context;
    pub type _freenect_device;
    static mut stdout: *mut FILE;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn freenect_init(
        ctx: *mut *mut freenect_context,
        usb_ctx: *mut libc::c_void,
    ) -> libc::c_int;
    fn freenect_shutdown(ctx: *mut freenect_context) -> libc::c_int;
    fn freenect_set_log_level(ctx: *mut freenect_context, level: freenect_loglevel);
    fn freenect_process_events(ctx: *mut freenect_context) -> libc::c_int;
    fn freenect_num_devices(ctx: *mut freenect_context) -> libc::c_int;
    fn freenect_select_subdevices(
        ctx: *mut freenect_context,
        subdevs: freenect_device_flags,
    );
    fn freenect_open_device(
        ctx: *mut freenect_context,
        dev: *mut *mut freenect_device,
        index: libc::c_int,
    ) -> libc::c_int;
    fn freenect_close_device(dev: *mut freenect_device) -> libc::c_int;
    fn freenect_set_depth_callback(dev: *mut freenect_device, cb: freenect_depth_cb);
    fn freenect_set_depth_chunk_callback(
        dev: *mut freenect_device,
        cb: freenect_chunk_cb,
    );
    fn freenect_start_depth(dev: *mut freenect_device) -> libc::c_int;
    fn freenect_stop_depth(dev: *mut freenect_device) -> libc::c_int;
    fn freenect_update_tilt_state(dev: *mut freenect_device) -> libc::c_int;
    fn freenect_get_tilt_state(
        dev: *mut freenect_device,
    ) -> *mut freenect_raw_tilt_state;
    fn freenect_set_tilt_degs(
        dev: *mut freenect_device,
        angle: libc::c_double,
    ) -> libc::c_int;
    fn freenect_set_led(
        dev: *mut freenect_device,
        option: freenect_led_options,
    ) -> libc::c_int;
    fn freenect_get_mks_accel(
        state: *mut freenect_raw_tilt_state,
        x: *mut libc::c_double,
        y: *mut libc::c_double,
        z: *mut libc::c_double,
    );
    fn freenect_find_depth_mode(
        res: freenect_resolution,
        fmt: freenect_depth_format,
    ) -> freenect_frame_mode;
    fn freenect_set_depth_mode(
        dev: *mut freenect_device,
        mode: freenect_frame_mode,
    ) -> libc::c_int;
    fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option::<
            unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
        >,
        __arg: *mut libc::c_void,
    ) -> libc::c_int;
    fn pthread_join(
        __th: pthread_t,
        __thread_return: *mut *mut libc::c_void,
    ) -> libc::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_cond_signal(__cond: *mut pthread_cond_t) -> libc::c_int;
    fn pthread_cond_wait(
        __cond: *mut pthread_cond_t,
        __mutex: *mut pthread_mutex_t,
    ) -> libc::c_int;
    fn glBindTexture(target: GLenum, texture: GLuint);
    fn glGenTextures(n: GLsizei, textures: *mut GLuint);
    fn glTexImage2D(
        target: GLenum,
        level: GLint,
        internalFormat: GLint,
        width: GLsizei,
        height: GLsizei,
        border: GLint,
        format: GLenum,
        type_0: GLenum,
        pixels: *const libc::c_void,
    );
    fn glTexParameteri(target: GLenum, pname: GLenum, param: GLint);
    fn glShadeModel(mode: GLenum);
    fn glTexCoord2f(s: GLfloat, t: GLfloat);
    fn glColor4f(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat);
    fn glVertex3f(x: GLfloat, y: GLfloat, z: GLfloat);
    fn glEnd();
    fn glBegin(mode: GLenum);
    fn glLoadIdentity();
    fn glViewport(x: GLint, y: GLint, width: GLsizei, height: GLsizei);
    fn glOrtho(
        left: GLdouble,
        right: GLdouble,
        bottom: GLdouble,
        top: GLdouble,
        near_val: GLdouble,
        far_val: GLdouble,
    );
    fn glMatrixMode(mode: GLenum);
    fn glDepthMask(flag: GLboolean);
    fn glDepthFunc(func: GLenum);
    fn glClearDepth(depth: GLclampd);
    fn glDisable(cap: GLenum);
    fn glEnable(cap: GLenum);
    fn glBlendFunc(sfactor: GLenum, dfactor: GLenum);
    fn glClearColor(red: GLclampf, green: GLclampf, blue: GLclampf, alpha: GLclampf);
    fn glutInit(pargc: *mut libc::c_int, argv: *mut *mut libc::c_char);
    fn glutInitWindowPosition(x: libc::c_int, y: libc::c_int);
    fn glutInitWindowSize(width: libc::c_int, height: libc::c_int);
    fn glutInitDisplayMode(displayMode: libc::c_uint);
    fn glutMainLoop();
    fn glutCreateWindow(title: *const libc::c_char) -> libc::c_int;
    fn glutDestroyWindow(window_0: libc::c_int);
    fn glutKeyboardFunc(
        callback: Option::<
            unsafe extern "C" fn(libc::c_uchar, libc::c_int, libc::c_int) -> (),
        >,
    );
    fn glutReshapeFunc(
        callback: Option::<unsafe extern "C" fn(libc::c_int, libc::c_int) -> ()>,
    );
    fn glutDisplayFunc(callback: Option::<unsafe extern "C" fn() -> ()>);
    fn glutIdleFunc(callback: Option::<unsafe extern "C" fn() -> ()>);
    fn glutSwapBuffers();
    fn powf(_: libc::c_float, _: libc::c_float) -> libc::c_float;
}
pub type size_t = libc::c_ulong;
pub type __int8_t = libc::c_schar;
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
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
pub type int8_t = __int8_t;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
pub type __pthread_list_t = __pthread_internal_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_mutex_s {
    pub __lock: libc::c_int,
    pub __count: libc::c_uint,
    pub __owner: libc::c_int,
    pub __nusers: libc::c_uint,
    pub __kind: libc::c_int,
    pub __spins: libc::c_short,
    pub __elision: libc::c_short,
    pub __list: __pthread_list_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_cond_s {
    pub c2rust_unnamed: C2RustUnnamed_1,
    pub c2rust_unnamed_0: C2RustUnnamed,
    pub __g_refs: [libc::c_uint; 2],
    pub __g_size: [libc::c_uint; 2],
    pub __g1_orig_size: libc::c_uint,
    pub __wrefs: libc::c_uint,
    pub __g_signals: [libc::c_uint; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub __g1_start: libc::c_ulonglong,
    pub __g1_start32: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub __low: libc::c_uint,
    pub __high: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub __wseq: libc::c_ulonglong,
    pub __wseq32: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub __low: libc::c_uint,
    pub __high: libc::c_uint,
}
pub type pthread_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_attr_t {
    pub __size: [libc::c_char; 56],
    pub __align: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
    pub __align: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_cond_t {
    pub __data: __pthread_cond_s,
    pub __size: [libc::c_char; 48],
    pub __align: libc::c_longlong,
}
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type freenect_device_flags = libc::c_uint;
pub const FREENECT_DEVICE_AUDIO: freenect_device_flags = 4;
pub const FREENECT_DEVICE_CAMERA: freenect_device_flags = 2;
pub const FREENECT_DEVICE_MOTOR: freenect_device_flags = 1;
pub type freenect_resolution = libc::c_uint;
pub const FREENECT_RESOLUTION_DUMMY: freenect_resolution = 2147483647;
pub const FREENECT_RESOLUTION_HIGH: freenect_resolution = 2;
pub const FREENECT_RESOLUTION_MEDIUM: freenect_resolution = 1;
pub const FREENECT_RESOLUTION_LOW: freenect_resolution = 0;
pub type freenect_video_format = libc::c_uint;
pub const FREENECT_VIDEO_DUMMY: freenect_video_format = 2147483647;
pub const FREENECT_VIDEO_YUV_RAW: freenect_video_format = 6;
pub const FREENECT_VIDEO_YUV_RGB: freenect_video_format = 5;
pub const FREENECT_VIDEO_IR_10BIT_PACKED: freenect_video_format = 4;
pub const FREENECT_VIDEO_IR_10BIT: freenect_video_format = 3;
pub const FREENECT_VIDEO_IR_8BIT: freenect_video_format = 2;
pub const FREENECT_VIDEO_BAYER: freenect_video_format = 1;
pub const FREENECT_VIDEO_RGB: freenect_video_format = 0;
pub type freenect_depth_format = libc::c_uint;
pub const FREENECT_DEPTH_DUMMY: freenect_depth_format = 2147483647;
pub const FREENECT_DEPTH_MM: freenect_depth_format = 5;
pub const FREENECT_DEPTH_REGISTERED: freenect_depth_format = 4;
pub const FREENECT_DEPTH_10BIT_PACKED: freenect_depth_format = 3;
pub const FREENECT_DEPTH_11BIT_PACKED: freenect_depth_format = 2;
pub const FREENECT_DEPTH_10BIT: freenect_depth_format = 1;
pub const FREENECT_DEPTH_11BIT: freenect_depth_format = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct freenect_frame_mode {
    pub reserved: uint32_t,
    pub resolution: freenect_resolution,
    pub c2rust_unnamed: C2RustUnnamed_3,
    pub bytes: int32_t,
    pub width: int16_t,
    pub height: int16_t,
    pub data_bits_per_pixel: int8_t,
    pub padding_bits_per_pixel: int8_t,
    pub framerate: int8_t,
    pub is_valid: int8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub dummy: int32_t,
    pub video_format: freenect_video_format,
    pub depth_format: freenect_depth_format,
}
pub type freenect_led_options = libc::c_uint;
pub const LED_BLINK_RED_YELLOW: freenect_led_options = 6;
pub const LED_BLINK_GREEN: freenect_led_options = 4;
pub const LED_YELLOW: freenect_led_options = 3;
pub const LED_RED: freenect_led_options = 2;
pub const LED_GREEN: freenect_led_options = 1;
pub const LED_OFF: freenect_led_options = 0;
pub type freenect_tilt_status_code = libc::c_uint;
pub const TILT_STATUS_MOVING: freenect_tilt_status_code = 4;
pub const TILT_STATUS_LIMIT: freenect_tilt_status_code = 1;
pub const TILT_STATUS_STOPPED: freenect_tilt_status_code = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct freenect_raw_tilt_state {
    pub accelerometer_x: int16_t,
    pub accelerometer_y: int16_t,
    pub accelerometer_z: int16_t,
    pub tilt_angle: int8_t,
    pub tilt_status: freenect_tilt_status_code,
}
pub type freenect_context = _freenect_context;
pub type freenect_device = _freenect_device;
pub type freenect_usb_context = ();
pub type freenect_loglevel = libc::c_uint;
pub const FREENECT_LOG_FLOOD: freenect_loglevel = 7;
pub const FREENECT_LOG_SPEW: freenect_loglevel = 6;
pub const FREENECT_LOG_DEBUG: freenect_loglevel = 5;
pub const FREENECT_LOG_INFO: freenect_loglevel = 4;
pub const FREENECT_LOG_NOTICE: freenect_loglevel = 3;
pub const FREENECT_LOG_WARNING: freenect_loglevel = 2;
pub const FREENECT_LOG_ERROR: freenect_loglevel = 1;
pub const FREENECT_LOG_FATAL: freenect_loglevel = 0;
pub type freenect_depth_cb = Option::<
    unsafe extern "C" fn(*mut freenect_device, *mut libc::c_void, uint32_t) -> (),
>;
pub type freenect_chunk_cb = Option::<
    unsafe extern "C" fn(
        *mut libc::c_void,
        *mut libc::c_void,
        libc::c_int,
        libc::c_int,
        *mut libc::c_void,
    ) -> (),
>;
pub type C2RustUnnamed_4 = libc::c_uint;
pub const PTHREAD_MUTEX_DEFAULT: C2RustUnnamed_4 = 0;
pub const PTHREAD_MUTEX_ERRORCHECK: C2RustUnnamed_4 = 2;
pub const PTHREAD_MUTEX_RECURSIVE: C2RustUnnamed_4 = 1;
pub const PTHREAD_MUTEX_NORMAL: C2RustUnnamed_4 = 0;
pub const PTHREAD_MUTEX_ADAPTIVE_NP: C2RustUnnamed_4 = 3;
pub const PTHREAD_MUTEX_ERRORCHECK_NP: C2RustUnnamed_4 = 2;
pub const PTHREAD_MUTEX_RECURSIVE_NP: C2RustUnnamed_4 = 1;
pub const PTHREAD_MUTEX_TIMED_NP: C2RustUnnamed_4 = 0;
pub type GLenum = libc::c_uint;
pub type GLboolean = libc::c_uchar;
pub type GLvoid = ();
pub type GLint = libc::c_int;
pub type GLuint = libc::c_uint;
pub type GLsizei = libc::c_int;
pub type GLfloat = libc::c_float;
pub type GLclampf = libc::c_float;
pub type GLdouble = libc::c_double;
pub type GLclampd = libc::c_double;
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
pub static mut freenect_thread: pthread_t = 0;
pub static mut die: libc::c_int = 0 as libc::c_int;
pub static mut g_argc: libc::c_int = 0;
pub static mut g_argv: *mut *mut libc::c_char = 0 as *const *mut libc::c_char
    as *mut *mut libc::c_char;
pub static mut window: libc::c_int = 0;
pub static mut gl_backbuf_mutex: pthread_mutex_t = pthread_mutex_t {
    __data: {
        let mut init = __pthread_mutex_s {
            __lock: 0 as libc::c_int,
            __count: 0 as libc::c_int as libc::c_uint,
            __owner: 0 as libc::c_int,
            __nusers: 0 as libc::c_int as libc::c_uint,
            __kind: PTHREAD_MUTEX_TIMED_NP as libc::c_int,
            __spins: 0 as libc::c_int as libc::c_short,
            __elision: 0 as libc::c_int as libc::c_short,
            __list: {
                let mut init = __pthread_internal_list {
                    __prev: 0 as *const __pthread_internal_list
                        as *mut __pthread_internal_list,
                    __next: 0 as *const __pthread_internal_list
                        as *mut __pthread_internal_list,
                };
                init
            },
        };
        init
    },
};
pub static mut depth_mid: *mut uint8_t = 0 as *const uint8_t as *mut uint8_t;
pub static mut depth_front: *mut uint8_t = 0 as *const uint8_t as *mut uint8_t;
pub static mut gl_depth_tex: GLuint = 0;
pub static mut f_ctx: *mut freenect_context = 0 as *const freenect_context
    as *mut freenect_context;
pub static mut f_dev: *mut freenect_device = 0 as *const freenect_device
    as *mut freenect_device;
pub static mut freenect_angle: libc::c_int = 0 as libc::c_int;
pub static mut freenect_led: libc::c_int = 0;
pub static mut gl_frame_cond: pthread_cond_t = pthread_cond_t {
    __data: {
        let mut init = __pthread_cond_s {
            c2rust_unnamed: C2RustUnnamed_1 {
                __wseq: 0 as libc::c_int as libc::c_ulonglong,
            },
            c2rust_unnamed_0: C2RustUnnamed {
                __g1_start: 0 as libc::c_int as libc::c_ulonglong,
            },
            __g_refs: [
                0 as libc::c_int as libc::c_uint,
                0 as libc::c_int as libc::c_uint,
            ],
            __g_size: [
                0 as libc::c_int as libc::c_uint,
                0 as libc::c_int as libc::c_uint,
            ],
            __g1_orig_size: 0 as libc::c_int as libc::c_uint,
            __wrefs: 0 as libc::c_int as libc::c_uint,
            __g_signals: [
                0 as libc::c_int as libc::c_uint,
                0 as libc::c_int as libc::c_uint,
            ],
        };
        init
    },
};
pub static mut got_depth: libc::c_int = 0 as libc::c_int;
pub unsafe extern "C" fn chunk_cb(
    mut buffer: *mut libc::c_void,
    mut pkt_data: *mut libc::c_void,
    mut pkt_num: libc::c_int,
    mut pkt_size: libc::c_int,
    mut ud: *mut libc::c_void,
) {
    if pkt_num == 73 as libc::c_int || pkt_num == 146 as libc::c_int {
        return;
    }
    let mut raw: *mut uint8_t = pkt_data as *mut uint8_t;
    let mut frame: *mut uint16_t = buffer as *mut uint16_t;
    if pkt_num > 219 as libc::c_int {
        raw = raw.offset(((pkt_num - 220 as libc::c_int) * 12 as libc::c_int) as isize);
        frame = frame
            .offset((320 as libc::c_int * (pkt_num - 2 as libc::c_int)) as isize);
    } else if pkt_num > 146 as libc::c_int {
        raw = raw
            .offset(
                ((pkt_num - 147 as libc::c_int) * 12 as libc::c_int + 4 as libc::c_int)
                    as isize,
            );
        frame = frame
            .offset((320 as libc::c_int * (pkt_num - 2 as libc::c_int)) as isize);
    } else if pkt_num > 73 as libc::c_int {
        raw = raw
            .offset(
                ((pkt_num - 74 as libc::c_int) * 12 as libc::c_int + 8 as libc::c_int)
                    as isize,
            );
        frame = frame
            .offset((320 as libc::c_int * (pkt_num - 1 as libc::c_int)) as isize);
    } else {
        raw = raw.offset((pkt_num * 12 as libc::c_int) as isize);
        frame = frame.offset((320 as libc::c_int * pkt_num) as isize);
    }
    let mut n: libc::c_int = 0 as libc::c_int;
    while n != 40 as libc::c_int {
        *frame
            .offset(
                0 as libc::c_int as isize,
            ) = ((*raw.offset(0 as libc::c_int as isize) as libc::c_int)
            << 3 as libc::c_int
            | *raw.offset(1 as libc::c_int as isize) as libc::c_int >> 5 as libc::c_int)
            as uint16_t;
        *frame
            .offset(
                1 as libc::c_int as isize,
            ) = (((*raw.offset(2 as libc::c_int as isize) as libc::c_int)
            << 9 as libc::c_int
            | (*raw.offset(3 as libc::c_int as isize) as libc::c_int) << 1 as libc::c_int
            | *raw.offset(4 as libc::c_int as isize) as libc::c_int >> 7 as libc::c_int)
            & 2047 as libc::c_int) as uint16_t;
        *frame
            .offset(
                2 as libc::c_int as isize,
            ) = (((*raw.offset(5 as libc::c_int as isize) as libc::c_int)
            << 7 as libc::c_int
            | *raw.offset(6 as libc::c_int as isize) as libc::c_int >> 1 as libc::c_int)
            & 2047 as libc::c_int) as uint16_t;
        *frame
            .offset(
                3 as libc::c_int as isize,
            ) = (((*raw.offset(8 as libc::c_int as isize) as libc::c_int)
            << 5 as libc::c_int
            | *raw.offset(9 as libc::c_int as isize) as libc::c_int >> 3 as libc::c_int)
            & 2047 as libc::c_int) as uint16_t;
        *frame
            .offset(
                4 as libc::c_int as isize,
            ) = ((*raw.offset(11 as libc::c_int as isize) as libc::c_int)
            << 3 as libc::c_int
            | *raw.offset(12 as libc::c_int as isize) as libc::c_int >> 5 as libc::c_int)
            as uint16_t;
        *frame
            .offset(
                5 as libc::c_int as isize,
            ) = (((*raw.offset(13 as libc::c_int as isize) as libc::c_int)
            << 9 as libc::c_int
            | (*raw.offset(14 as libc::c_int as isize) as libc::c_int)
                << 1 as libc::c_int
            | *raw.offset(15 as libc::c_int as isize) as libc::c_int >> 7 as libc::c_int)
            & 2047 as libc::c_int) as uint16_t;
        *frame
            .offset(
                6 as libc::c_int as isize,
            ) = (((*raw.offset(16 as libc::c_int as isize) as libc::c_int)
            << 7 as libc::c_int
            | *raw.offset(17 as libc::c_int as isize) as libc::c_int >> 1 as libc::c_int)
            & 2047 as libc::c_int) as uint16_t;
        *frame
            .offset(
                7 as libc::c_int as isize,
            ) = (((*raw.offset(19 as libc::c_int as isize) as libc::c_int)
            << 5 as libc::c_int
            | *raw.offset(20 as libc::c_int as isize) as libc::c_int >> 3 as libc::c_int)
            & 2047 as libc::c_int) as uint16_t;
        frame = frame.offset(8 as libc::c_int as isize);
        raw = raw.offset(22 as libc::c_int as isize);
        n += 1;
        n;
    }
}
pub unsafe extern "C" fn DrawGLScene() {
    pthread_mutex_lock(&mut gl_backbuf_mutex);
    while got_depth == 0 {
        pthread_cond_wait(&mut gl_frame_cond, &mut gl_backbuf_mutex);
    }
    let mut tmp: *mut uint8_t = 0 as *mut uint8_t;
    if got_depth != 0 {
        tmp = depth_front;
        depth_front = depth_mid;
        depth_mid = tmp;
        got_depth = 0 as libc::c_int;
    }
    pthread_mutex_unlock(&mut gl_backbuf_mutex);
    glBindTexture(0xde1 as libc::c_int as GLenum, gl_depth_tex);
    glTexImage2D(
        0xde1 as libc::c_int as GLenum,
        0 as libc::c_int,
        3 as libc::c_int,
        320 as libc::c_int,
        240 as libc::c_int,
        0 as libc::c_int,
        0x1907 as libc::c_int as GLenum,
        0x1401 as libc::c_int as GLenum,
        depth_front as *const libc::c_void,
    );
    glBegin(0x6 as libc::c_int as GLenum);
    glColor4f(1.0f32, 1.0f32, 1.0f32, 1.0f32);
    glTexCoord2f(0 as libc::c_int as GLfloat, 0 as libc::c_int as GLfloat);
    glVertex3f(
        0 as libc::c_int as GLfloat,
        0 as libc::c_int as GLfloat,
        0 as libc::c_int as GLfloat,
    );
    glTexCoord2f(1 as libc::c_int as GLfloat, 0 as libc::c_int as GLfloat);
    glVertex3f(
        320 as libc::c_int as GLfloat,
        0 as libc::c_int as GLfloat,
        0 as libc::c_int as GLfloat,
    );
    glTexCoord2f(1 as libc::c_int as GLfloat, 1 as libc::c_int as GLfloat);
    glVertex3f(
        320 as libc::c_int as GLfloat,
        240 as libc::c_int as GLfloat,
        0 as libc::c_int as GLfloat,
    );
    glTexCoord2f(0 as libc::c_int as GLfloat, 1 as libc::c_int as GLfloat);
    glVertex3f(
        0 as libc::c_int as GLfloat,
        240 as libc::c_int as GLfloat,
        0 as libc::c_int as GLfloat,
    );
    glEnd();
    glutSwapBuffers();
}
pub unsafe extern "C" fn keyPressed(
    mut key: libc::c_uchar,
    mut x: libc::c_int,
    mut y: libc::c_int,
) {
    if key as libc::c_int == 27 as libc::c_int {
        ::std::ptr::write_volatile(&mut die as *mut libc::c_int, 1 as libc::c_int);
        pthread_join(freenect_thread, 0 as *mut *mut libc::c_void);
        glutDestroyWindow(window);
        free(depth_mid as *mut libc::c_void);
        free(depth_front as *mut libc::c_void);
        exit(0 as libc::c_int);
    }
    if key as libc::c_int == 'w' as i32 {
        freenect_angle += 1;
        freenect_angle;
        if freenect_angle > 30 as libc::c_int {
            freenect_angle = 30 as libc::c_int;
        }
    }
    if key as libc::c_int == 's' as i32 {
        freenect_angle = 0 as libc::c_int;
    }
    if key as libc::c_int == 'x' as i32 {
        freenect_angle -= 1;
        freenect_angle;
        if freenect_angle < -(30 as libc::c_int) {
            freenect_angle = -(30 as libc::c_int);
        }
    }
    if key as libc::c_int == '1' as i32 {
        freenect_set_led(f_dev, LED_GREEN);
    }
    if key as libc::c_int == '2' as i32 {
        freenect_set_led(f_dev, LED_RED);
    }
    if key as libc::c_int == '3' as i32 {
        freenect_set_led(f_dev, LED_YELLOW);
    }
    if key as libc::c_int == '4' as i32 {
        freenect_set_led(f_dev, LED_BLINK_GREEN);
    }
    if key as libc::c_int == '5' as i32 {
        freenect_set_led(f_dev, LED_BLINK_GREEN);
    }
    if key as libc::c_int == '6' as i32 {
        freenect_set_led(f_dev, LED_BLINK_RED_YELLOW);
    }
    if key as libc::c_int == '0' as i32 {
        freenect_set_led(f_dev, LED_OFF);
    }
    freenect_set_tilt_degs(f_dev, freenect_angle as libc::c_double);
}
pub unsafe extern "C" fn ReSizeGLScene(mut Width: libc::c_int, mut Height: libc::c_int) {
    glViewport(0 as libc::c_int, 0 as libc::c_int, Width, Height);
    glMatrixMode(0x1701 as libc::c_int as GLenum);
    glLoadIdentity();
    glOrtho(
        0 as libc::c_int as GLdouble,
        320 as libc::c_int as GLdouble,
        240 as libc::c_int as GLdouble,
        0 as libc::c_int as GLdouble,
        -1.0f32 as GLdouble,
        1.0f32 as GLdouble,
    );
    glMatrixMode(0x1700 as libc::c_int as GLenum);
    glLoadIdentity();
}
pub unsafe extern "C" fn InitGL(mut Width: libc::c_int, mut Height: libc::c_int) {
    glClearColor(0.0f32, 0.0f32, 0.0f32, 0.0f32);
    glClearDepth(1.0f64);
    glDepthFunc(0x201 as libc::c_int as GLenum);
    glDepthMask(0 as libc::c_int as GLboolean);
    glDisable(0xb71 as libc::c_int as GLenum);
    glDisable(0xbe2 as libc::c_int as GLenum);
    glDisable(0xbc0 as libc::c_int as GLenum);
    glEnable(0xde1 as libc::c_int as GLenum);
    glBlendFunc(0x302 as libc::c_int as GLenum, 0x303 as libc::c_int as GLenum);
    glShadeModel(0x1d00 as libc::c_int as GLenum);
    glGenTextures(1 as libc::c_int, &mut gl_depth_tex);
    glBindTexture(0xde1 as libc::c_int as GLenum, gl_depth_tex);
    glTexParameteri(
        0xde1 as libc::c_int as GLenum,
        0x2801 as libc::c_int as GLenum,
        0x2601 as libc::c_int,
    );
    glTexParameteri(
        0xde1 as libc::c_int as GLenum,
        0x2800 as libc::c_int as GLenum,
        0x2601 as libc::c_int,
    );
    ReSizeGLScene(Width, Height);
}
pub unsafe extern "C" fn gl_threadfunc(mut arg: *mut libc::c_void) -> *mut libc::c_void {
    printf(b"GL thread\n\0" as *const u8 as *const libc::c_char);
    glutInit(&mut g_argc, g_argv);
    glutInitDisplayMode(
        (0 as libc::c_int | 0x2 as libc::c_int | 0x8 as libc::c_int
            | 0x10 as libc::c_int) as libc::c_uint,
    );
    glutInitWindowSize(320 as libc::c_int, 240 as libc::c_int);
    glutInitWindowPosition(0 as libc::c_int, 0 as libc::c_int);
    window = glutCreateWindow(b"LibFreenect\0" as *const u8 as *const libc::c_char);
    glutDisplayFunc(
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> ()>,
            Option::<unsafe extern "C" fn() -> ()>,
        >(
            Some(
                ::std::mem::transmute::<
                    unsafe extern "C" fn() -> (),
                    unsafe extern "C" fn() -> (),
                >(DrawGLScene),
            ),
        ),
    );
    glutIdleFunc(
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> ()>,
            Option::<unsafe extern "C" fn() -> ()>,
        >(
            Some(
                ::std::mem::transmute::<
                    unsafe extern "C" fn() -> (),
                    unsafe extern "C" fn() -> (),
                >(DrawGLScene),
            ),
        ),
    );
    glutReshapeFunc(
        Some(ReSizeGLScene as unsafe extern "C" fn(libc::c_int, libc::c_int) -> ()),
    );
    glutKeyboardFunc(
        Some(
            keyPressed
                as unsafe extern "C" fn(libc::c_uchar, libc::c_int, libc::c_int) -> (),
        ),
    );
    InitGL(320 as libc::c_int, 240 as libc::c_int);
    glutMainLoop();
    return 0 as *mut libc::c_void;
}
pub static mut t_gamma: [uint16_t; 2048] = [0; 2048];
pub unsafe extern "C" fn depth_cb(
    mut dev: *mut freenect_device,
    mut v_depth: *mut libc::c_void,
    mut timestamp: uint32_t,
) {
    let mut i: libc::c_int = 0;
    let mut depth: *mut uint16_t = v_depth as *mut uint16_t;
    pthread_mutex_lock(&mut gl_backbuf_mutex);
    i = 0 as libc::c_int;
    while i < 320 as libc::c_int * 240 as libc::c_int {
        let mut pval: libc::c_int = t_gamma[*depth.offset(i as isize) as usize]
            as libc::c_int;
        let mut lb: libc::c_int = pval & 0xff as libc::c_int;
        match pval >> 8 as libc::c_int {
            0 => {
                *depth_mid
                    .offset(
                        (3 as libc::c_int * i + 0 as libc::c_int) as isize,
                    ) = 255 as libc::c_int as uint8_t;
                *depth_mid
                    .offset(
                        (3 as libc::c_int * i + 1 as libc::c_int) as isize,
                    ) = (255 as libc::c_int - lb) as uint8_t;
                *depth_mid
                    .offset(
                        (3 as libc::c_int * i + 2 as libc::c_int) as isize,
                    ) = (255 as libc::c_int - lb) as uint8_t;
            }
            1 => {
                *depth_mid
                    .offset(
                        (3 as libc::c_int * i + 0 as libc::c_int) as isize,
                    ) = 255 as libc::c_int as uint8_t;
                *depth_mid
                    .offset(
                        (3 as libc::c_int * i + 1 as libc::c_int) as isize,
                    ) = lb as uint8_t;
                *depth_mid
                    .offset(
                        (3 as libc::c_int * i + 2 as libc::c_int) as isize,
                    ) = 0 as libc::c_int as uint8_t;
            }
            2 => {
                *depth_mid
                    .offset(
                        (3 as libc::c_int * i + 0 as libc::c_int) as isize,
                    ) = (255 as libc::c_int - lb) as uint8_t;
                *depth_mid
                    .offset(
                        (3 as libc::c_int * i + 1 as libc::c_int) as isize,
                    ) = 255 as libc::c_int as uint8_t;
                *depth_mid
                    .offset(
                        (3 as libc::c_int * i + 2 as libc::c_int) as isize,
                    ) = 0 as libc::c_int as uint8_t;
            }
            3 => {
                *depth_mid
                    .offset(
                        (3 as libc::c_int * i + 0 as libc::c_int) as isize,
                    ) = 0 as libc::c_int as uint8_t;
                *depth_mid
                    .offset(
                        (3 as libc::c_int * i + 1 as libc::c_int) as isize,
                    ) = 255 as libc::c_int as uint8_t;
                *depth_mid
                    .offset(
                        (3 as libc::c_int * i + 2 as libc::c_int) as isize,
                    ) = lb as uint8_t;
            }
            4 => {
                *depth_mid
                    .offset(
                        (3 as libc::c_int * i + 0 as libc::c_int) as isize,
                    ) = 0 as libc::c_int as uint8_t;
                *depth_mid
                    .offset(
                        (3 as libc::c_int * i + 1 as libc::c_int) as isize,
                    ) = (255 as libc::c_int - lb) as uint8_t;
                *depth_mid
                    .offset(
                        (3 as libc::c_int * i + 2 as libc::c_int) as isize,
                    ) = 255 as libc::c_int as uint8_t;
            }
            5 => {
                *depth_mid
                    .offset(
                        (3 as libc::c_int * i + 0 as libc::c_int) as isize,
                    ) = 0 as libc::c_int as uint8_t;
                *depth_mid
                    .offset(
                        (3 as libc::c_int * i + 1 as libc::c_int) as isize,
                    ) = 0 as libc::c_int as uint8_t;
                *depth_mid
                    .offset(
                        (3 as libc::c_int * i + 2 as libc::c_int) as isize,
                    ) = (255 as libc::c_int - lb) as uint8_t;
            }
            _ => {
                *depth_mid
                    .offset(
                        (3 as libc::c_int * i + 0 as libc::c_int) as isize,
                    ) = 0 as libc::c_int as uint8_t;
                *depth_mid
                    .offset(
                        (3 as libc::c_int * i + 1 as libc::c_int) as isize,
                    ) = 0 as libc::c_int as uint8_t;
                *depth_mid
                    .offset(
                        (3 as libc::c_int * i + 2 as libc::c_int) as isize,
                    ) = 0 as libc::c_int as uint8_t;
            }
        }
        i += 1;
        i;
    }
    got_depth += 1;
    got_depth;
    pthread_cond_signal(&mut gl_frame_cond);
    pthread_mutex_unlock(&mut gl_backbuf_mutex);
}
pub unsafe extern "C" fn freenect_threadfunc(
    mut arg: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut accelCount: libc::c_int = 0 as libc::c_int;
    freenect_set_tilt_degs(f_dev, freenect_angle as libc::c_double);
    freenect_set_led(f_dev, LED_RED);
    freenect_set_depth_callback(
        f_dev,
        Some(
            depth_cb
                as unsafe extern "C" fn(
                    *mut freenect_device,
                    *mut libc::c_void,
                    uint32_t,
                ) -> (),
        ),
    );
    freenect_set_depth_mode(
        f_dev,
        freenect_find_depth_mode(FREENECT_RESOLUTION_MEDIUM, FREENECT_DEPTH_11BIT_PACKED),
    );
    freenect_set_depth_chunk_callback(
        f_dev,
        Some(
            chunk_cb
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                    libc::c_int,
                    libc::c_int,
                    *mut libc::c_void,
                ) -> (),
        ),
    );
    freenect_start_depth(f_dev);
    printf(
        b"'w'-tilt up, 's'-level, 'x'-tilt down, '0'-'6'-select LED mode\n\0"
            as *const u8 as *const libc::c_char,
    );
    while die == 0 && freenect_process_events(f_ctx) >= 0 as libc::c_int {
        let fresh0 = accelCount;
        accelCount = accelCount + 1;
        if fresh0 >= 2000 as libc::c_int {
            accelCount = 0 as libc::c_int;
            let mut state: *mut freenect_raw_tilt_state = 0
                as *mut freenect_raw_tilt_state;
            freenect_update_tilt_state(f_dev);
            state = freenect_get_tilt_state(f_dev);
            let mut dx: libc::c_double = 0.;
            let mut dy: libc::c_double = 0.;
            let mut dz: libc::c_double = 0.;
            freenect_get_mks_accel(state, &mut dx, &mut dy, &mut dz);
            printf(
                b"\r raw acceleration: %4d %4d %4d  mks acceleration: %4f %4f %4f\0"
                    as *const u8 as *const libc::c_char,
                (*state).accelerometer_x as libc::c_int,
                (*state).accelerometer_y as libc::c_int,
                (*state).accelerometer_z as libc::c_int,
                dx,
                dy,
                dz,
            );
            fflush(stdout);
        }
    }
    printf(b"\nshutting down streams...\n\0" as *const u8 as *const libc::c_char);
    freenect_stop_depth(f_dev);
    freenect_close_device(f_dev);
    freenect_shutdown(f_ctx);
    printf(b"-- done!\n\0" as *const u8 as *const libc::c_char);
    return 0 as *mut libc::c_void;
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut res: libc::c_int = 0;
    depth_mid = malloc(
        (320 as libc::c_int * 240 as libc::c_int * 3 as libc::c_int) as libc::c_ulong,
    ) as *mut uint8_t;
    depth_front = malloc(
        (320 as libc::c_int * 240 as libc::c_int * 3 as libc::c_int) as libc::c_ulong,
    ) as *mut uint8_t;
    printf(b"Kinect camera test\n\0" as *const u8 as *const libc::c_char);
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 2048 as libc::c_int {
        let mut v: libc::c_float = (i as libc::c_double / 2048.0f64) as libc::c_float;
        v = powf(v, 3 as libc::c_int as libc::c_float)
            * 6 as libc::c_int as libc::c_float;
        t_gamma[i
            as usize] = (v * 6 as libc::c_int as libc::c_float
            * 256 as libc::c_int as libc::c_float) as uint16_t;
        i += 1;
        i;
    }
    g_argc = argc;
    g_argv = argv;
    if freenect_init(&mut f_ctx, 0 as *mut libc::c_void) < 0 as libc::c_int {
        printf(b"freenect_init() failed\n\0" as *const u8 as *const libc::c_char);
        return 1 as libc::c_int;
    }
    freenect_set_log_level(f_ctx, FREENECT_LOG_DEBUG);
    freenect_select_subdevices(
        f_ctx,
        (FREENECT_DEVICE_MOTOR as libc::c_int | FREENECT_DEVICE_CAMERA as libc::c_int)
            as freenect_device_flags,
    );
    let mut nr_devices: libc::c_int = freenect_num_devices(f_ctx);
    printf(
        b"Number of devices found: %d\n\0" as *const u8 as *const libc::c_char,
        nr_devices,
    );
    let mut user_device_number: libc::c_int = 0 as libc::c_int;
    if argc > 1 as libc::c_int {
        user_device_number = atoi(*argv.offset(1 as libc::c_int as isize));
    }
    if nr_devices < 1 as libc::c_int {
        freenect_shutdown(f_ctx);
        return 1 as libc::c_int;
    }
    if freenect_open_device(f_ctx, &mut f_dev, user_device_number) < 0 as libc::c_int {
        printf(b"Could not open device\n\0" as *const u8 as *const libc::c_char);
        freenect_shutdown(f_ctx);
        return 1 as libc::c_int;
    }
    res = pthread_create(
        &mut freenect_thread,
        0 as *const pthread_attr_t,
        Some(
            freenect_threadfunc
                as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
        ),
        0 as *mut libc::c_void,
    );
    if res != 0 {
        printf(b"pthread_create failed\n\0" as *const u8 as *const libc::c_char);
        freenect_shutdown(f_ctx);
        return 1 as libc::c_int;
    }
    gl_threadfunc(0 as *mut libc::c_void);
    return 0 as libc::c_int;
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
