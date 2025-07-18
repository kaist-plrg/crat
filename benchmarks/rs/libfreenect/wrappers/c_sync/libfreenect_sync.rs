use ::libc;
extern "C" {
    pub type _freenect_context;
    pub type _freenect_device;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
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
    fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> libc::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_cond_init(
        __cond: *mut pthread_cond_t,
        __cond_attr: *const pthread_condattr_t,
    ) -> libc::c_int;
    fn pthread_cond_signal(__cond: *mut pthread_cond_t) -> libc::c_int;
    fn pthread_cond_wait(
        __cond: *mut pthread_cond_t,
        __mutex: *mut pthread_mutex_t,
    ) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn freenect_init(
        ctx_0: *mut *mut freenect_context,
        usb_ctx: *mut libc::c_void,
    ) -> libc::c_int;
    fn freenect_shutdown(ctx_0: *mut freenect_context) -> libc::c_int;
    fn freenect_process_events(ctx_0: *mut freenect_context) -> libc::c_int;
    fn freenect_select_subdevices(
        ctx_0: *mut freenect_context,
        subdevs: freenect_device_flags,
    );
    fn freenect_open_device(
        ctx_0: *mut freenect_context,
        dev: *mut *mut freenect_device,
        index: libc::c_int,
    ) -> libc::c_int;
    fn freenect_close_device(dev: *mut freenect_device) -> libc::c_int;
    fn freenect_set_user(dev: *mut freenect_device, user: *mut libc::c_void);
    fn freenect_get_user(dev: *mut freenect_device) -> *mut libc::c_void;
    fn freenect_set_depth_callback(dev: *mut freenect_device, cb: freenect_depth_cb);
    fn freenect_set_video_callback(dev: *mut freenect_device, cb: freenect_video_cb);
    fn freenect_set_depth_buffer(
        dev: *mut freenect_device,
        buf: *mut libc::c_void,
    ) -> libc::c_int;
    fn freenect_set_video_buffer(
        dev: *mut freenect_device,
        buf: *mut libc::c_void,
    ) -> libc::c_int;
    fn freenect_start_depth(dev: *mut freenect_device) -> libc::c_int;
    fn freenect_start_video(dev: *mut freenect_device) -> libc::c_int;
    fn freenect_stop_depth(dev: *mut freenect_device) -> libc::c_int;
    fn freenect_stop_video(dev: *mut freenect_device) -> libc::c_int;
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
    fn freenect_find_video_mode(
        res: freenect_resolution,
        fmt: freenect_video_format,
    ) -> freenect_frame_mode;
    fn freenect_set_video_mode(
        dev: *mut freenect_device,
        mode: freenect_frame_mode,
    ) -> libc::c_int;
    fn freenect_find_depth_mode(
        res: freenect_resolution,
        fmt: freenect_depth_format,
    ) -> freenect_frame_mode;
    fn freenect_set_depth_mode(
        dev: *mut freenect_device,
        mode: freenect_frame_mode,
    ) -> libc::c_int;
    fn freenect_camera_to_world(
        dev: *mut freenect_device,
        cx: libc::c_int,
        cy: libc::c_int,
        wz: libc::c_int,
        wx: *mut libc::c_double,
        wy: *mut libc::c_double,
    );
}
pub type __int8_t = libc::c_schar;
pub type __int16_t = libc::c_short;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
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
pub union pthread_mutexattr_t {
    pub __size: [libc::c_char; 4],
    pub __align: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_condattr_t {
    pub __size: [libc::c_char; 4],
    pub __align: libc::c_int,
}
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
pub type C2RustUnnamed_3 = libc::c_uint;
pub const PTHREAD_MUTEX_DEFAULT: C2RustUnnamed_3 = 0;
pub const PTHREAD_MUTEX_ERRORCHECK: C2RustUnnamed_3 = 2;
pub const PTHREAD_MUTEX_RECURSIVE: C2RustUnnamed_3 = 1;
pub const PTHREAD_MUTEX_NORMAL: C2RustUnnamed_3 = 0;
pub const PTHREAD_MUTEX_ADAPTIVE_NP: C2RustUnnamed_3 = 3;
pub const PTHREAD_MUTEX_ERRORCHECK_NP: C2RustUnnamed_3 = 2;
pub const PTHREAD_MUTEX_RECURSIVE_NP: C2RustUnnamed_3 = 1;
pub const PTHREAD_MUTEX_TIMED_NP: C2RustUnnamed_3 = 0;
pub type int8_t = __int8_t;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
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
    pub c2rust_unnamed: C2RustUnnamed_4,
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
pub union C2RustUnnamed_4 {
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
pub type freenect_depth_cb = Option::<
    unsafe extern "C" fn(*mut freenect_device, *mut libc::c_void, uint32_t) -> (),
>;
pub type freenect_video_cb = Option::<
    unsafe extern "C" fn(*mut freenect_device, *mut libc::c_void, uint32_t) -> (),
>;
pub type buffer_ring_t = buffer_ring;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct buffer_ring {
    pub lock: pthread_mutex_t,
    pub cb_cond: pthread_cond_t,
    pub bufs: [*mut libc::c_void; 3],
    pub timestamp: uint32_t,
    pub valid: libc::c_int,
    pub fmt: libc::c_int,
    pub res: libc::c_int,
}
pub type sync_kinect_t = sync_kinect;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sync_kinect {
    pub dev: *mut freenect_device,
    pub video: buffer_ring_t,
    pub depth: buffer_ring_t,
}
pub type set_buffer_t = Option::<
    unsafe extern "C" fn(*mut freenect_device, *mut libc::c_void) -> libc::c_int,
>;
static mut kinects: [*mut sync_kinect_t; 64] = [
    0 as *const sync_kinect_t as *mut sync_kinect_t,
    0 as *const sync_kinect_t as *mut sync_kinect_t,
    0 as *const sync_kinect_t as *mut sync_kinect_t,
    0 as *const sync_kinect_t as *mut sync_kinect_t,
    0 as *const sync_kinect_t as *mut sync_kinect_t,
    0 as *const sync_kinect_t as *mut sync_kinect_t,
    0 as *const sync_kinect_t as *mut sync_kinect_t,
    0 as *const sync_kinect_t as *mut sync_kinect_t,
    0 as *const sync_kinect_t as *mut sync_kinect_t,
    0 as *const sync_kinect_t as *mut sync_kinect_t,
    0 as *const sync_kinect_t as *mut sync_kinect_t,
    0 as *const sync_kinect_t as *mut sync_kinect_t,
    0 as *const sync_kinect_t as *mut sync_kinect_t,
    0 as *const sync_kinect_t as *mut sync_kinect_t,
    0 as *const sync_kinect_t as *mut sync_kinect_t,
    0 as *const sync_kinect_t as *mut sync_kinect_t,
    0 as *const sync_kinect_t as *mut sync_kinect_t,
    0 as *const sync_kinect_t as *mut sync_kinect_t,
    0 as *const sync_kinect_t as *mut sync_kinect_t,
    0 as *const sync_kinect_t as *mut sync_kinect_t,
    0 as *const sync_kinect_t as *mut sync_kinect_t,
    0 as *const sync_kinect_t as *mut sync_kinect_t,
    0 as *const sync_kinect_t as *mut sync_kinect_t,
    0 as *const sync_kinect_t as *mut sync_kinect_t,
    0 as *const sync_kinect_t as *mut sync_kinect_t,
    0 as *const sync_kinect_t as *mut sync_kinect_t,
    0 as *const sync_kinect_t as *mut sync_kinect_t,
    0 as *const sync_kinect_t as *mut sync_kinect_t,
    0 as *const sync_kinect_t as *mut sync_kinect_t,
    0 as *const sync_kinect_t as *mut sync_kinect_t,
    0 as *const sync_kinect_t as *mut sync_kinect_t,
    0 as *const sync_kinect_t as *mut sync_kinect_t,
    0 as *const sync_kinect_t as *mut sync_kinect_t,
    0 as *const sync_kinect_t as *mut sync_kinect_t,
    0 as *const sync_kinect_t as *mut sync_kinect_t,
    0 as *const sync_kinect_t as *mut sync_kinect_t,
    0 as *const sync_kinect_t as *mut sync_kinect_t,
    0 as *const sync_kinect_t as *mut sync_kinect_t,
    0 as *const sync_kinect_t as *mut sync_kinect_t,
    0 as *const sync_kinect_t as *mut sync_kinect_t,
    0 as *const sync_kinect_t as *mut sync_kinect_t,
    0 as *const sync_kinect_t as *mut sync_kinect_t,
    0 as *const sync_kinect_t as *mut sync_kinect_t,
    0 as *const sync_kinect_t as *mut sync_kinect_t,
    0 as *const sync_kinect_t as *mut sync_kinect_t,
    0 as *const sync_kinect_t as *mut sync_kinect_t,
    0 as *const sync_kinect_t as *mut sync_kinect_t,
    0 as *const sync_kinect_t as *mut sync_kinect_t,
    0 as *const sync_kinect_t as *mut sync_kinect_t,
    0 as *const sync_kinect_t as *mut sync_kinect_t,
    0 as *const sync_kinect_t as *mut sync_kinect_t,
    0 as *const sync_kinect_t as *mut sync_kinect_t,
    0 as *const sync_kinect_t as *mut sync_kinect_t,
    0 as *const sync_kinect_t as *mut sync_kinect_t,
    0 as *const sync_kinect_t as *mut sync_kinect_t,
    0 as *const sync_kinect_t as *mut sync_kinect_t,
    0 as *const sync_kinect_t as *mut sync_kinect_t,
    0 as *const sync_kinect_t as *mut sync_kinect_t,
    0 as *const sync_kinect_t as *mut sync_kinect_t,
    0 as *const sync_kinect_t as *mut sync_kinect_t,
    0 as *const sync_kinect_t as *mut sync_kinect_t,
    0 as *const sync_kinect_t as *mut sync_kinect_t,
    0 as *const sync_kinect_t as *mut sync_kinect_t,
    0 as *const sync_kinect_t as *mut sync_kinect_t,
];
static mut ctx: *mut freenect_context = 0 as *const freenect_context
    as *mut freenect_context;
static mut thread_running: libc::c_int = 0 as libc::c_int;
static mut thread: pthread_t = 0;
static mut runloop_lock: pthread_mutex_t = pthread_mutex_t {
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
static mut pending_runloop_tasks: libc::c_int = 0 as libc::c_int;
static mut pending_runloop_tasks_lock: pthread_mutex_t = pthread_mutex_t {
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
static mut pending_runloop_tasks_cond: pthread_cond_t = pthread_cond_t {
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
unsafe extern "C" fn alloc_buffer_ring_video(
    mut res: freenect_resolution,
    mut fmt: freenect_video_format,
    mut buf: *mut buffer_ring_t,
) -> libc::c_int {
    let mut sz: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    match fmt as libc::c_uint {
        0 | 1 | 2 | 3 | 4 => {
            sz = (freenect_find_video_mode(res, fmt)).bytes;
        }
        _ => {
            printf(
                b"Invalid video format %d\n\0" as *const u8 as *const libc::c_char,
                fmt as libc::c_uint,
            );
            return -(1 as libc::c_int);
        }
    }
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        (*buf).bufs[i as usize] = malloc(sz as libc::c_ulong);
        i += 1;
        i;
    }
    (*buf).timestamp = 0 as libc::c_int as uint32_t;
    (*buf).valid = 0 as libc::c_int;
    (*buf).fmt = fmt as libc::c_int;
    (*buf).res = res as libc::c_int;
    return 0 as libc::c_int;
}
unsafe extern "C" fn alloc_buffer_ring_depth(
    mut res: freenect_resolution,
    mut fmt: freenect_depth_format,
    mut buf: *mut buffer_ring_t,
) -> libc::c_int {
    let mut sz: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    match fmt as libc::c_uint {
        0 | 1 | 2 | 3 | 4 | 5 => {
            sz = (freenect_find_depth_mode(res, fmt)).bytes;
        }
        _ => {
            printf(
                b"Invalid depth format %d\n\0" as *const u8 as *const libc::c_char,
                fmt as libc::c_uint,
            );
            return -(1 as libc::c_int);
        }
    }
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        (*buf).bufs[i as usize] = malloc(sz as libc::c_ulong);
        i += 1;
        i;
    }
    (*buf).timestamp = 0 as libc::c_int as uint32_t;
    (*buf).valid = 0 as libc::c_int;
    (*buf).fmt = fmt as libc::c_int;
    (*buf).res = res as libc::c_int;
    return 0 as libc::c_int;
}
unsafe extern "C" fn free_buffer_ring(mut buf: *mut buffer_ring_t) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        free((*buf).bufs[i as usize]);
        (*buf).bufs[i as usize] = 0 as *mut libc::c_void;
        i += 1;
        i;
    }
    (*buf).timestamp = 0 as libc::c_int as uint32_t;
    (*buf).valid = 0 as libc::c_int;
    (*buf).fmt = -(1 as libc::c_int);
    (*buf).res = -(1 as libc::c_int);
}
unsafe extern "C" fn producer_cb_inner(
    mut dev: *mut freenect_device,
    mut data: *mut libc::c_void,
    mut timestamp: uint32_t,
    mut buf: *mut buffer_ring_t,
    mut set_buffer: set_buffer_t,
) {
    pthread_mutex_lock(&mut (*buf).lock);
    if data == (*buf).bufs[2 as libc::c_int as usize] {} else {
        __assert_fail(
            b"data == buf->bufs[2]\0" as *const u8 as *const libc::c_char,
            b"../../../wrappers/c_sync/libfreenect_sync.c\0" as *const u8
                as *const libc::c_char,
            140 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 91],
                &[libc::c_char; 91],
            >(
                b"void producer_cb_inner(freenect_device *, void *, uint32_t, buffer_ring_t *, set_buffer_t)\0",
            ))
                .as_ptr(),
        );
    }
    'c_4600: {
        if data == (*buf).bufs[2 as libc::c_int as usize] {} else {
            __assert_fail(
                b"data == buf->bufs[2]\0" as *const u8 as *const libc::c_char,
                b"../../../wrappers/c_sync/libfreenect_sync.c\0" as *const u8
                    as *const libc::c_char,
                140 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 91],
                    &[libc::c_char; 91],
                >(
                    b"void producer_cb_inner(freenect_device *, void *, uint32_t, buffer_ring_t *, set_buffer_t)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    let mut temp_buf: *mut libc::c_void = (*buf).bufs[1 as libc::c_int as usize];
    (*buf).bufs[1 as libc::c_int as usize] = (*buf).bufs[2 as libc::c_int as usize];
    (*buf).bufs[2 as libc::c_int as usize] = temp_buf;
    set_buffer.unwrap()(dev, temp_buf);
    (*buf).timestamp = timestamp;
    (*buf).valid = 1 as libc::c_int;
    pthread_cond_signal(&mut (*buf).cb_cond);
    pthread_mutex_unlock(&mut (*buf).lock);
}
unsafe extern "C" fn video_producer_cb(
    mut dev: *mut freenect_device,
    mut data: *mut libc::c_void,
    mut timestamp: uint32_t,
) {
    producer_cb_inner(
        dev,
        data,
        timestamp,
        &mut (*((freenect_get_user
            as unsafe extern "C" fn(*mut freenect_device) -> *mut libc::c_void)(dev)
            as *mut sync_kinect_t))
            .video,
        Some(
            freenect_set_video_buffer
                as unsafe extern "C" fn(
                    *mut freenect_device,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
    );
}
unsafe extern "C" fn depth_producer_cb(
    mut dev: *mut freenect_device,
    mut data: *mut libc::c_void,
    mut timestamp: uint32_t,
) {
    producer_cb_inner(
        dev,
        data,
        timestamp,
        &mut (*((freenect_get_user
            as unsafe extern "C" fn(*mut freenect_device) -> *mut libc::c_void)(dev)
            as *mut sync_kinect_t))
            .depth,
        Some(
            freenect_set_depth_buffer
                as unsafe extern "C" fn(
                    *mut freenect_device,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
    );
}
unsafe extern "C" fn pending_runloop_tasks_inc() {
    pthread_mutex_lock(&mut pending_runloop_tasks_lock);
    if pending_runloop_tasks >= 0 as libc::c_int {} else {
        __assert_fail(
            b"pending_runloop_tasks >= 0\0" as *const u8 as *const libc::c_char,
            b"../../../wrappers/c_sync/libfreenect_sync.c\0" as *const u8
                as *const libc::c_char,
            165 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 37],
                &[libc::c_char; 37],
            >(b"void pending_runloop_tasks_inc(void)\0"))
                .as_ptr(),
        );
    }
    'c_5120: {
        if pending_runloop_tasks >= 0 as libc::c_int {} else {
            __assert_fail(
                b"pending_runloop_tasks >= 0\0" as *const u8 as *const libc::c_char,
                b"../../../wrappers/c_sync/libfreenect_sync.c\0" as *const u8
                    as *const libc::c_char,
                165 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 37],
                    &[libc::c_char; 37],
                >(b"void pending_runloop_tasks_inc(void)\0"))
                    .as_ptr(),
            );
        }
    };
    pending_runloop_tasks += 1;
    pending_runloop_tasks;
    pthread_mutex_unlock(&mut pending_runloop_tasks_lock);
}
unsafe extern "C" fn pending_runloop_tasks_dec() {
    pthread_mutex_lock(&mut pending_runloop_tasks_lock);
    pending_runloop_tasks -= 1;
    pending_runloop_tasks;
    if pending_runloop_tasks >= 0 as libc::c_int {} else {
        __assert_fail(
            b"pending_runloop_tasks >= 0\0" as *const u8 as *const libc::c_char,
            b"../../../wrappers/c_sync/libfreenect_sync.c\0" as *const u8
                as *const libc::c_char,
            174 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 37],
                &[libc::c_char; 37],
            >(b"void pending_runloop_tasks_dec(void)\0"))
                .as_ptr(),
        );
    }
    'c_3665: {
        if pending_runloop_tasks >= 0 as libc::c_int {} else {
            __assert_fail(
                b"pending_runloop_tasks >= 0\0" as *const u8 as *const libc::c_char,
                b"../../../wrappers/c_sync/libfreenect_sync.c\0" as *const u8
                    as *const libc::c_char,
                174 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 37],
                    &[libc::c_char; 37],
                >(b"void pending_runloop_tasks_dec(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if pending_runloop_tasks == 0 {
        pthread_cond_signal(&mut pending_runloop_tasks_cond);
    }
    pthread_mutex_unlock(&mut pending_runloop_tasks_lock);
}
unsafe extern "C" fn pending_runloop_tasks_wait_zero() {
    pthread_mutex_lock(&mut pending_runloop_tasks_lock);
    while pending_runloop_tasks != 0 {
        pthread_cond_wait(
            &mut pending_runloop_tasks_cond,
            &mut pending_runloop_tasks_lock,
        );
    }
    pthread_mutex_unlock(&mut pending_runloop_tasks_lock);
}
unsafe extern "C" fn init(mut unused: *mut libc::c_void) -> *mut libc::c_void {
    pending_runloop_tasks_wait_zero();
    pthread_mutex_lock(&mut runloop_lock);
    while thread_running != 0 && freenect_process_events(ctx) >= 0 as libc::c_int {
        pthread_mutex_unlock(&mut runloop_lock);
        pending_runloop_tasks_wait_zero();
        pthread_mutex_lock(&mut runloop_lock);
    }
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 64 as libc::c_int {
        if !(kinects[i as usize]).is_null() {
            freenect_stop_video((*kinects[i as usize]).dev);
            freenect_stop_depth((*kinects[i as usize]).dev);
            freenect_set_user((*kinects[i as usize]).dev, 0 as *mut libc::c_void);
            freenect_close_device((*kinects[i as usize]).dev);
            free_buffer_ring(&mut (**kinects.as_mut_ptr().offset(i as isize)).video);
            free_buffer_ring(&mut (**kinects.as_mut_ptr().offset(i as isize)).depth);
            free(kinects[i as usize] as *mut libc::c_void);
            kinects[i as usize] = 0 as *mut sync_kinect_t;
        }
        i += 1;
        i;
    }
    freenect_shutdown(ctx);
    pthread_mutex_unlock(&mut runloop_lock);
    return 0 as *mut libc::c_void;
}
unsafe extern "C" fn init_thread() -> libc::c_int {
    thread_running = 1 as libc::c_int;
    let mut ret: libc::c_int = freenect_init(&mut ctx, 0 as *mut libc::c_void);
    if ret != 0 as libc::c_int {
        return ret;
    }
    freenect_select_subdevices(
        ctx,
        (FREENECT_DEVICE_MOTOR as libc::c_int | FREENECT_DEVICE_CAMERA as libc::c_int)
            as freenect_device_flags,
    );
    ret = pthread_create(
        &mut thread,
        0 as *const pthread_attr_t,
        Some(init as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void),
        0 as *mut libc::c_void,
    );
    if ret != 0 as libc::c_int {
        return ret;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn change_video_format(
    mut kinect: *mut sync_kinect_t,
    mut res: freenect_resolution,
    mut fmt: freenect_video_format,
) -> libc::c_int {
    freenect_stop_video((*kinect).dev);
    free_buffer_ring(&mut (*kinect).video);
    if alloc_buffer_ring_video(res, fmt, &mut (*kinect).video) != 0 {
        return -(1 as libc::c_int);
    }
    freenect_set_video_mode((*kinect).dev, freenect_find_video_mode(res, fmt));
    freenect_set_video_buffer(
        (*kinect).dev,
        (*kinect).video.bufs[2 as libc::c_int as usize],
    );
    freenect_start_video((*kinect).dev);
    return 0 as libc::c_int;
}
unsafe extern "C" fn change_depth_format(
    mut kinect: *mut sync_kinect_t,
    mut res: freenect_resolution,
    mut fmt: freenect_depth_format,
) -> libc::c_int {
    freenect_stop_depth((*kinect).dev);
    free_buffer_ring(&mut (*kinect).depth);
    if alloc_buffer_ring_depth(res, fmt, &mut (*kinect).depth) != 0 {
        return -(1 as libc::c_int);
    }
    freenect_set_depth_mode((*kinect).dev, freenect_find_depth_mode(res, fmt));
    freenect_set_depth_buffer(
        (*kinect).dev,
        (*kinect).depth.bufs[2 as libc::c_int as usize],
    );
    freenect_start_depth((*kinect).dev);
    return 0 as libc::c_int;
}
unsafe extern "C" fn alloc_kinect(mut index: libc::c_int) -> *mut sync_kinect_t {
    let mut kinect: *mut sync_kinect_t = malloc(
        ::std::mem::size_of::<sync_kinect_t>() as libc::c_ulong,
    ) as *mut sync_kinect_t;
    if freenect_open_device(ctx, &mut (*kinect).dev, index) < 0 as libc::c_int {
        free(kinect as *mut libc::c_void);
        return 0 as *mut sync_kinect_t;
    }
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        (*kinect).video.bufs[i as usize] = 0 as *mut libc::c_void;
        (*kinect).depth.bufs[i as usize] = 0 as *mut libc::c_void;
        i += 1;
        i;
    }
    (*kinect).video.fmt = -(1 as libc::c_int);
    (*kinect).video.res = -(1 as libc::c_int);
    (*kinect).depth.fmt = -(1 as libc::c_int);
    (*kinect).depth.res = -(1 as libc::c_int);
    freenect_set_video_callback(
        (*kinect).dev,
        Some(
            video_producer_cb
                as unsafe extern "C" fn(
                    *mut freenect_device,
                    *mut libc::c_void,
                    uint32_t,
                ) -> (),
        ),
    );
    freenect_set_depth_callback(
        (*kinect).dev,
        Some(
            depth_producer_cb
                as unsafe extern "C" fn(
                    *mut freenect_device,
                    *mut libc::c_void,
                    uint32_t,
                ) -> (),
        ),
    );
    pthread_mutex_init(&mut (*kinect).video.lock, 0 as *const pthread_mutexattr_t);
    pthread_mutex_init(&mut (*kinect).depth.lock, 0 as *const pthread_mutexattr_t);
    pthread_cond_init(&mut (*kinect).video.cb_cond, 0 as *const pthread_condattr_t);
    pthread_cond_init(&mut (*kinect).depth.cb_cond, 0 as *const pthread_condattr_t);
    return kinect;
}
unsafe extern "C" fn setup_kinect(
    mut index: libc::c_int,
    mut res: libc::c_int,
    mut fmt: libc::c_int,
    mut is_depth: libc::c_int,
) -> libc::c_int {
    pending_runloop_tasks_inc();
    pthread_mutex_lock(&mut runloop_lock);
    let mut thread_running_prev: libc::c_int = thread_running;
    if thread_running == 0 {
        let mut ret: libc::c_int = init_thread();
        if ret != 0 as libc::c_int {
            pthread_mutex_unlock(&mut runloop_lock);
            pending_runloop_tasks_dec();
            return ret;
        }
    }
    if (kinects[index as usize]).is_null() {
        kinects[index as usize] = alloc_kinect(index);
    }
    if (kinects[index as usize]).is_null() {
        printf(
            b"Error: Invalid index [%d]\n\0" as *const u8 as *const libc::c_char,
            index,
        );
        if thread_running_prev == 0 {
            thread_running = 0 as libc::c_int;
            pthread_mutex_unlock(&mut runloop_lock);
            pending_runloop_tasks_dec();
            pthread_join(thread, 0 as *mut *mut libc::c_void);
        } else {
            pthread_mutex_unlock(&mut runloop_lock);
            pending_runloop_tasks_dec();
        }
        return -(1 as libc::c_int);
    }
    freenect_set_user(
        (*kinects[index as usize]).dev,
        kinects[index as usize] as *mut libc::c_void,
    );
    let mut buf: *mut buffer_ring_t = 0 as *mut buffer_ring_t;
    if is_depth != 0 {
        buf = &mut (**kinects.as_mut_ptr().offset(index as isize)).depth;
    } else {
        buf = &mut (**kinects.as_mut_ptr().offset(index as isize)).video;
    }
    pthread_mutex_lock(&mut (*buf).lock);
    if (*buf).fmt != fmt || (*buf).res != res {
        if is_depth != 0 {
            change_depth_format(
                kinects[index as usize],
                res as freenect_resolution,
                fmt as freenect_depth_format,
            );
        } else {
            change_video_format(
                kinects[index as usize],
                res as freenect_resolution,
                fmt as freenect_video_format,
            );
        }
    }
    pthread_mutex_unlock(&mut (*buf).lock);
    pthread_mutex_unlock(&mut runloop_lock);
    pending_runloop_tasks_dec();
    return 0 as libc::c_int;
}
unsafe extern "C" fn sync_get(
    mut data: *mut *mut libc::c_void,
    mut timestamp: *mut uint32_t,
    mut buf: *mut buffer_ring_t,
) -> libc::c_int {
    pthread_mutex_lock(&mut (*buf).lock);
    while (*buf).valid == 0 {
        pthread_cond_wait(&mut (*buf).cb_cond, &mut (*buf).lock);
    }
    let mut temp_buf: *mut libc::c_void = (*buf).bufs[0 as libc::c_int as usize];
    (*buf).bufs[0 as libc::c_int as usize] = (*buf).bufs[1 as libc::c_int as usize];
    *data = (*buf).bufs[0 as libc::c_int as usize];
    (*buf).bufs[1 as libc::c_int as usize] = temp_buf;
    (*buf).valid = 0 as libc::c_int;
    *timestamp = (*buf).timestamp;
    pthread_mutex_unlock(&mut (*buf).lock);
    return 0 as libc::c_int;
}
unsafe extern "C" fn runloop_enter(mut index: libc::c_int) -> libc::c_int {
    if index < 0 as libc::c_int || index >= 64 as libc::c_int {
        printf(
            b"Error: Invalid index [%d]\n\0" as *const u8 as *const libc::c_char,
            index,
        );
        return -(1 as libc::c_int);
    }
    if thread_running == 0 || (kinects[index as usize]).is_null() {
        if setup_kinect(
            index,
            FREENECT_RESOLUTION_MEDIUM as libc::c_int,
            FREENECT_DEPTH_11BIT as libc::c_int,
            1 as libc::c_int,
        ) != 0
        {
            return -(1 as libc::c_int);
        }
    }
    pending_runloop_tasks_inc();
    pthread_mutex_lock(&mut runloop_lock);
    return 0 as libc::c_int;
}
unsafe extern "C" fn runloop_exit() {
    pthread_mutex_unlock(&mut runloop_lock);
    pending_runloop_tasks_dec();
}
pub unsafe extern "C" fn freenect_sync_get_video_with_res(
    mut video: *mut *mut libc::c_void,
    mut timestamp: *mut uint32_t,
    mut index: libc::c_int,
    mut res: freenect_resolution,
    mut fmt: freenect_video_format,
) -> libc::c_int {
    if index < 0 as libc::c_int || index >= 64 as libc::c_int {
        printf(
            b"Error: Invalid index [%d]\n\0" as *const u8 as *const libc::c_char,
            index,
        );
        return -(1 as libc::c_int);
    }
    if thread_running == 0 || (kinects[index as usize]).is_null()
        || (*kinects[index as usize]).video.fmt as libc::c_uint != fmt as libc::c_uint
        || (*kinects[index as usize]).video.res as libc::c_uint != res as libc::c_uint
    {
        if setup_kinect(index, res as libc::c_int, fmt as libc::c_int, 0 as libc::c_int)
            != 0
        {
            return -(1 as libc::c_int);
        }
    }
    sync_get(
        video,
        timestamp,
        &mut (**kinects.as_mut_ptr().offset(index as isize)).video,
    );
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn freenect_sync_get_video(
    mut video: *mut *mut libc::c_void,
    mut timestamp: *mut uint32_t,
    mut index: libc::c_int,
    mut fmt: freenect_video_format,
) -> libc::c_int {
    return freenect_sync_get_video_with_res(
        video,
        timestamp,
        index,
        FREENECT_RESOLUTION_MEDIUM,
        fmt,
    );
}
pub unsafe extern "C" fn freenect_sync_get_depth_with_res(
    mut depth: *mut *mut libc::c_void,
    mut timestamp: *mut uint32_t,
    mut index: libc::c_int,
    mut res: freenect_resolution,
    mut fmt: freenect_depth_format,
) -> libc::c_int {
    if index < 0 as libc::c_int || index >= 64 as libc::c_int {
        printf(
            b"Error: Invalid index [%d]\n\0" as *const u8 as *const libc::c_char,
            index,
        );
        return -(1 as libc::c_int);
    }
    if thread_running == 0 || (kinects[index as usize]).is_null()
        || (*kinects[index as usize]).depth.fmt as libc::c_uint != fmt as libc::c_uint
        || (*kinects[index as usize]).depth.res as libc::c_uint != res as libc::c_uint
    {
        if setup_kinect(index, res as libc::c_int, fmt as libc::c_int, 1 as libc::c_int)
            != 0
        {
            return -(1 as libc::c_int);
        }
    }
    sync_get(
        depth,
        timestamp,
        &mut (**kinects.as_mut_ptr().offset(index as isize)).depth,
    );
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn freenect_sync_get_depth(
    mut depth: *mut *mut libc::c_void,
    mut timestamp: *mut uint32_t,
    mut index: libc::c_int,
    mut fmt: freenect_depth_format,
) -> libc::c_int {
    return freenect_sync_get_depth_with_res(
        depth,
        timestamp,
        index,
        FREENECT_RESOLUTION_MEDIUM,
        fmt,
    );
}
pub unsafe extern "C" fn freenect_sync_get_tilt_state(
    mut state: *mut *mut freenect_raw_tilt_state,
    mut index: libc::c_int,
) -> libc::c_int {
    if runloop_enter(index) != 0 {
        return -(1 as libc::c_int);
    }
    freenect_update_tilt_state((*kinects[index as usize]).dev);
    *state = freenect_get_tilt_state((*kinects[index as usize]).dev);
    runloop_exit();
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn freenect_sync_set_tilt_degs(
    mut angle: libc::c_int,
    mut index: libc::c_int,
) -> libc::c_int {
    if runloop_enter(index) != 0 {
        return -(1 as libc::c_int);
    }
    freenect_set_tilt_degs((*kinects[index as usize]).dev, angle as libc::c_double);
    runloop_exit();
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn freenect_sync_set_led(
    mut led: freenect_led_options,
    mut index: libc::c_int,
) -> libc::c_int {
    if runloop_enter(index) != 0 {
        return -(1 as libc::c_int);
    }
    freenect_set_led((*kinects[index as usize]).dev, led);
    runloop_exit();
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn freenect_sync_camera_to_world(
    mut cx: libc::c_int,
    mut cy: libc::c_int,
    mut wz: libc::c_int,
    mut wx: *mut libc::c_double,
    mut wy: *mut libc::c_double,
    mut index: libc::c_int,
) -> libc::c_int {
    if runloop_enter(index) != 0 {
        return -(1 as libc::c_int);
    }
    freenect_camera_to_world((*kinects[index as usize]).dev, cx, cy, wz, wx, wy);
    runloop_exit();
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn freenect_sync_stop() {
    if thread_running != 0 {
        thread_running = 0 as libc::c_int;
        pthread_join(thread, 0 as *mut *mut libc::c_void);
    }
}
