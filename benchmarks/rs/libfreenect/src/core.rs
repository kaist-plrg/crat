use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type libusb_device_handle;
    pub type libusb_context;
    static mut stderr: *mut FILE;
    fn vfprintf(
        _: *mut FILE,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn vsnprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn fnusb_init(ctx: *mut fnusb_ctx, usb_ctx: *mut libc::c_void) -> libc::c_int;
    fn fnusb_shutdown(ctx: *mut fnusb_ctx) -> libc::c_int;
    fn fnusb_close_subdevices(dev: *mut freenect_device) -> libc::c_int;
    fn fnusb_process_events_timeout(
        ctx: *mut fnusb_ctx,
        timeout: *mut timeval,
    ) -> libc::c_int;
    fn freenect_stop_audio(dev: *mut freenect_device) -> libc::c_int;
    fn freenect_stop_depth(dev: *mut freenect_device) -> libc::c_int;
    fn freenect_stop_video(dev: *mut freenect_device) -> libc::c_int;
    fn fnusb_num_devices(ctx: *mut freenect_context) -> libc::c_int;
    fn fnusb_list_device_attributes(
        ctx: *mut freenect_context,
        attribute_list: *mut *mut freenect_device_attributes,
    ) -> libc::c_int;
    fn fnusb_open_subdevices(
        dev: *mut freenect_device,
        index: libc::c_int,
    ) -> libc::c_int;
    fn freenect_camera_init(dev: *mut freenect_device) -> libc::c_int;
    fn freenect_camera_teardown(dev: *mut freenect_device) -> libc::c_int;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type size_t = libc::c_ulong;
pub type va_list = __builtin_va_list;
pub type __int8_t = libc::c_schar;
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
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
pub type int8_t = __int8_t;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type freenect_device_flags = libc::c_uint;
pub const FREENECT_DEVICE_AUDIO: freenect_device_flags = 4;
pub const FREENECT_DEVICE_CAMERA: freenect_device_flags = 2;
pub const FREENECT_DEVICE_MOTOR: freenect_device_flags = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct freenect_device_attributes {
    pub next: *mut freenect_device_attributes,
    pub camera_serial: *const libc::c_char,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _freenect_context {
    pub log_level: freenect_loglevel,
    pub log_cb: freenect_log_cb,
    pub usb: fnusb_ctx,
    pub enabled_subdevices: freenect_device_flags,
    pub first: *mut freenect_device,
    pub zero_plane_res: libc::c_int,
    pub fn_fw_nui_ptr: *mut libc::c_uchar,
    pub fn_fw_nui_size: libc::c_uint,
    pub fn_fw_k4w_ptr: *mut libc::c_uchar,
    pub fn_fw_k4w_size: libc::c_uint,
}
pub type freenect_device = _freenect_device;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _freenect_device {
    pub parent: *mut freenect_context,
    pub next: *mut freenect_device,
    pub user_data: *mut libc::c_void,
    pub usb_cam: fnusb_dev,
    pub depth_isoc: fnusb_isoc_stream,
    pub video_isoc: fnusb_isoc_stream,
    pub depth_cb: freenect_depth_cb,
    pub video_cb: freenect_video_cb,
    pub depth_chunk_cb: freenect_chunk_cb,
    pub video_chunk_cb: freenect_chunk_cb,
    pub video_format: freenect_video_format,
    pub depth_format: freenect_depth_format,
    pub video_resolution: freenect_resolution,
    pub depth_resolution: freenect_resolution,
    pub cam_inited: libc::c_int,
    pub cam_tag: uint16_t,
    pub depth: packet_stream,
    pub video: packet_stream,
    pub registration: freenect_registration,
    pub usb_audio: fnusb_dev,
    pub audio_out_isoc: fnusb_isoc_stream,
    pub audio_in_isoc: fnusb_isoc_stream,
    pub audio_in_cb: freenect_audio_in_cb,
    pub audio_out_cb: freenect_audio_out_cb,
    pub audio: audio_stream,
    pub audio_tag: uint32_t,
    pub usb_motor: fnusb_dev,
    pub raw_state: freenect_raw_tilt_state,
    pub device_does_motor_control_with_audio: libc::c_int,
    pub motor_control_with_audio_enabled: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fnusb_dev {
    pub parent: *mut freenect_device,
    pub dev: *mut libusb_device_handle,
    pub device_dead: libc::c_int,
    pub VID: libc::c_int,
    pub PID: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct audio_stream {
    pub running: libc::c_int,
    pub audio_out_ring: *mut freenect_sample_51,
    pub ring_reader_idx: libc::c_int,
    pub ring_writer_idx: libc::c_int,
    pub out_window: uint16_t,
    pub out_seq: uint8_t,
    pub out_counter_within_window: uint8_t,
    pub out_weird_timestamp: uint16_t,
    pub out_window_parity: uint8_t,
    pub in_window: uint16_t,
    pub last_seen_window: [uint16_t; 10],
    pub in_counter: uint8_t,
    pub mic_buffer: [*mut int32_t; 4],
    pub cancelled_buffer: *mut int16_t,
    pub in_unknown: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct freenect_sample_51 {
    pub left: int16_t,
    pub right: int16_t,
    pub center: int16_t,
    pub lfe: int16_t,
    pub surround_left: int16_t,
    pub surround_right: int16_t,
}
pub type freenect_audio_out_cb = Option::<
    unsafe extern "C" fn(
        *mut freenect_device,
        *mut freenect_sample_51,
        *mut libc::c_int,
    ) -> (),
>;
pub type freenect_audio_in_cb = Option::<
    unsafe extern "C" fn(
        *mut freenect_device,
        libc::c_int,
        *mut int32_t,
        *mut int32_t,
        *mut int32_t,
        *mut int32_t,
        *mut int16_t,
        *mut libc::c_void,
    ) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fnusb_isoc_stream {
    pub parent: *mut fnusb_dev,
    pub xfers: *mut *mut libusb_transfer,
    pub buffer: *mut uint8_t,
    pub cb: fnusb_iso_cb,
    pub num_xfers: libc::c_int,
    pub pkts: libc::c_int,
    pub len: libc::c_int,
    pub dead: libc::c_int,
    pub dead_xfers: libc::c_int,
}
pub type fnusb_iso_cb = Option::<
    unsafe extern "C" fn(*mut freenect_device, *mut uint8_t, libc::c_int) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct libusb_transfer {
    pub dev_handle: *mut libusb_device_handle,
    pub flags: uint8_t,
    pub endpoint: libc::c_uchar,
    pub type_0: libc::c_uchar,
    pub timeout: libc::c_uint,
    pub status: libusb_transfer_status,
    pub length: libc::c_int,
    pub actual_length: libc::c_int,
    pub callback: libusb_transfer_cb_fn,
    pub user_data: *mut libc::c_void,
    pub buffer: *mut libc::c_uchar,
    pub num_iso_packets: libc::c_int,
    pub iso_packet_desc: [libusb_iso_packet_descriptor; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct libusb_iso_packet_descriptor {
    pub length: libc::c_uint,
    pub actual_length: libc::c_uint,
    pub status: libusb_transfer_status,
}
pub type libusb_transfer_status = libc::c_uint;
pub const LIBUSB_TRANSFER_OVERFLOW: libusb_transfer_status = 6;
pub const LIBUSB_TRANSFER_NO_DEVICE: libusb_transfer_status = 5;
pub const LIBUSB_TRANSFER_STALL: libusb_transfer_status = 4;
pub const LIBUSB_TRANSFER_CANCELLED: libusb_transfer_status = 3;
pub const LIBUSB_TRANSFER_TIMED_OUT: libusb_transfer_status = 2;
pub const LIBUSB_TRANSFER_ERROR: libusb_transfer_status = 1;
pub const LIBUSB_TRANSFER_COMPLETED: libusb_transfer_status = 0;
pub type libusb_transfer_cb_fn = Option::<
    unsafe extern "C" fn(*mut libusb_transfer) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct freenect_registration {
    pub reg_info: freenect_reg_info,
    pub reg_pad_info: freenect_reg_pad_info,
    pub zero_plane_info: freenect_zero_plane_info,
    pub const_shift: libc::c_double,
    pub raw_to_mm_shift: *mut uint16_t,
    pub depth_to_rgb_shift: *mut int32_t,
    pub registration_table: *mut [int32_t; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct freenect_zero_plane_info {
    pub dcmos_emitter_dist: libc::c_float,
    pub dcmos_rcmos_dist: libc::c_float,
    pub reference_distance: libc::c_float,
    pub reference_pixel_size: libc::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct freenect_reg_pad_info {
    pub start_lines: uint16_t,
    pub end_lines: uint16_t,
    pub cropping_lines: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct freenect_reg_info {
    pub dx_center: int32_t,
    pub ax: int32_t,
    pub bx: int32_t,
    pub cx: int32_t,
    pub dx: int32_t,
    pub dx_start: int32_t,
    pub ay: int32_t,
    pub by: int32_t,
    pub cy: int32_t,
    pub dy: int32_t,
    pub dy_start: int32_t,
    pub dx_beta_start: int32_t,
    pub dy_beta_start: int32_t,
    pub rollout_blank: int32_t,
    pub rollout_size: int32_t,
    pub dx_beta_inc: int32_t,
    pub dy_beta_inc: int32_t,
    pub dxdx_start: int32_t,
    pub dxdy_start: int32_t,
    pub dydx_start: int32_t,
    pub dydy_start: int32_t,
    pub dxdxdx_start: int32_t,
    pub dydxdx_start: int32_t,
    pub dxdxdy_start: int32_t,
    pub dydxdy_start: int32_t,
    pub back_comp1: int32_t,
    pub dydydx_start: int32_t,
    pub back_comp2: int32_t,
    pub dydydy_start: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct packet_stream {
    pub running: libc::c_int,
    pub flag: uint8_t,
    pub synced: libc::c_int,
    pub seq: uint8_t,
    pub got_pkts: libc::c_int,
    pub pkt_num: libc::c_int,
    pub pkts_per_frame: libc::c_int,
    pub pkt_size: libc::c_int,
    pub frame_size: libc::c_int,
    pub last_pkt_size: libc::c_int,
    pub valid_pkts: libc::c_int,
    pub lost_pkts: libc::c_uint,
    pub valid_frames: libc::c_int,
    pub variable_length: libc::c_int,
    pub last_timestamp: uint32_t,
    pub timestamp: uint32_t,
    pub split_bufs: libc::c_int,
    pub lib_buf: *mut libc::c_void,
    pub usr_buf: *mut libc::c_void,
    pub raw_buf: *mut uint8_t,
    pub proc_buf: *mut libc::c_void,
}
pub type freenect_chunk_cb = Option::<
    unsafe extern "C" fn(
        *mut libc::c_void,
        *mut libc::c_void,
        libc::c_int,
        libc::c_int,
        *mut libc::c_void,
    ) -> (),
>;
pub type freenect_video_cb = Option::<
    unsafe extern "C" fn(*mut freenect_device, *mut libc::c_void, uint32_t) -> (),
>;
pub type freenect_depth_cb = Option::<
    unsafe extern "C" fn(*mut freenect_device, *mut libc::c_void, uint32_t) -> (),
>;
pub type freenect_context = _freenect_context;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fnusb_ctx {
    pub ctx: *mut libusb_context,
    pub should_free_ctx: libc::c_int,
}
pub type freenect_log_cb = Option::<
    unsafe extern "C" fn(
        *mut freenect_context,
        freenect_loglevel,
        *const libc::c_char,
    ) -> (),
>;
pub type freenect_loglevel = libc::c_uint;
pub const FREENECT_LOG_FLOOD: freenect_loglevel = 7;
pub const FREENECT_LOG_SPEW: freenect_loglevel = 6;
pub const FREENECT_LOG_DEBUG: freenect_loglevel = 5;
pub const FREENECT_LOG_INFO: freenect_loglevel = 4;
pub const FREENECT_LOG_NOTICE: freenect_loglevel = 3;
pub const FREENECT_LOG_WARNING: freenect_loglevel = 2;
pub const FREENECT_LOG_ERROR: freenect_loglevel = 1;
pub const FREENECT_LOG_FATAL: freenect_loglevel = 0;
pub type freenect_usb_context = ();
pub unsafe extern "C" fn freenect_init(
    mut ctx: *mut *mut freenect_context,
    mut usb_ctx: *mut libc::c_void,
) -> libc::c_int {
    let mut res: libc::c_int = 0;
    *ctx = malloc(::std::mem::size_of::<freenect_context>() as libc::c_ulong)
        as *mut freenect_context;
    if (*ctx).is_null() {
        return -(1 as libc::c_int);
    }
    memset(
        *ctx as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<freenect_context>() as libc::c_ulong,
    );
    (**ctx).log_level = FREENECT_LOG_NOTICE;
    (**ctx)
        .enabled_subdevices = (FREENECT_DEVICE_MOTOR as libc::c_int
        | FREENECT_DEVICE_CAMERA as libc::c_int) as freenect_device_flags;
    res = fnusb_init(&mut (**ctx).usb, usb_ctx);
    if res < 0 as libc::c_int {
        free(*ctx as *mut libc::c_void);
        *ctx = 0 as *mut freenect_context;
    }
    return res;
}
pub unsafe extern "C" fn freenect_shutdown(
    mut ctx: *mut freenect_context,
) -> libc::c_int {
    while !((*ctx).first).is_null() {
        fn_log(
            ctx,
            FREENECT_LOG_NOTICE,
            b"Device %p open during shutdown, closing...\n\0" as *const u8
                as *const libc::c_char,
            (*ctx).first,
        );
        freenect_close_device((*ctx).first);
    }
    fnusb_shutdown(&mut (*ctx).usb);
    free(ctx as *mut libc::c_void);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn freenect_process_events(
    mut ctx: *mut freenect_context,
) -> libc::c_int {
    let mut timeout: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    timeout.tv_sec = 60 as libc::c_int as __time_t;
    timeout.tv_usec = 0 as libc::c_int as __suseconds_t;
    return freenect_process_events_timeout(ctx, &mut timeout);
}
pub unsafe extern "C" fn freenect_process_events_timeout(
    mut ctx: *mut freenect_context,
    mut timeout: *mut timeval,
) -> libc::c_int {
    let mut res: libc::c_int = fnusb_process_events_timeout(&mut (*ctx).usb, timeout);
    let mut dev: *mut freenect_device = (*ctx).first;
    while !dev.is_null() {
        if (*dev).usb_cam.device_dead != 0 {
            fn_log(
                ctx,
                FREENECT_LOG_ERROR,
                b"USB camera marked dead, stopping streams\n\0" as *const u8
                    as *const libc::c_char,
            );
            res = -(1 as libc::c_int);
            freenect_stop_video(dev);
            freenect_stop_depth(dev);
        }
        if (*dev).usb_audio.device_dead != 0 {
            fn_log(
                ctx,
                FREENECT_LOG_ERROR,
                b"USB audio marked dead, stopping streams\n\0" as *const u8
                    as *const libc::c_char,
            );
            res = -(1 as libc::c_int);
            freenect_stop_audio(dev);
        }
        dev = (*dev).next;
    }
    return res;
}
pub unsafe extern "C" fn freenect_num_devices(
    mut ctx: *mut freenect_context,
) -> libc::c_int {
    return fnusb_num_devices(ctx);
}
pub unsafe extern "C" fn freenect_list_device_attributes(
    mut ctx: *mut freenect_context,
    mut attribute_list: *mut *mut freenect_device_attributes,
) -> libc::c_int {
    return fnusb_list_device_attributes(ctx, attribute_list);
}
pub unsafe extern "C" fn freenect_free_device_attributes(
    mut attribute_list: *mut freenect_device_attributes,
) {
    let mut to_free: *mut freenect_device_attributes = 0
        as *mut freenect_device_attributes;
    while !attribute_list.is_null() {
        to_free = attribute_list;
        if !((*attribute_list).camera_serial).is_null() {
            free(
                (*attribute_list).camera_serial as *mut libc::c_char as *mut libc::c_void,
            );
            (*attribute_list).camera_serial = 0 as *const libc::c_char;
        }
        attribute_list = (*attribute_list).next;
        free(to_free as *mut libc::c_void);
    }
}
pub unsafe extern "C" fn freenect_supported_subdevices() -> libc::c_int {
    return FREENECT_DEVICE_MOTOR as libc::c_int | FREENECT_DEVICE_CAMERA as libc::c_int
        | FREENECT_DEVICE_AUDIO as libc::c_int;
}
pub unsafe extern "C" fn freenect_select_subdevices(
    mut ctx: *mut freenect_context,
    mut subdevs: freenect_device_flags,
) {
    (*ctx)
        .enabled_subdevices = (subdevs as libc::c_uint
        & (FREENECT_DEVICE_MOTOR as libc::c_int | FREENECT_DEVICE_CAMERA as libc::c_int
            | FREENECT_DEVICE_AUDIO as libc::c_int) as libc::c_uint)
        as freenect_device_flags;
}
pub unsafe extern "C" fn freenect_enabled_subdevices(
    mut ctx: *mut freenect_context,
) -> freenect_device_flags {
    return (*ctx).enabled_subdevices;
}
pub unsafe extern "C" fn freenect_open_device(
    mut ctx: *mut freenect_context,
    mut dev: *mut *mut freenect_device,
    mut index: libc::c_int,
) -> libc::c_int {
    let mut res: libc::c_int = 0;
    let mut pdev: *mut freenect_device = malloc(
        ::std::mem::size_of::<freenect_device>() as libc::c_ulong,
    ) as *mut freenect_device;
    if pdev.is_null() {
        return -(1 as libc::c_int);
    }
    memset(
        pdev as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<freenect_device>() as libc::c_ulong,
    );
    (*pdev).parent = ctx;
    res = fnusb_open_subdevices(pdev, index);
    if res < 0 as libc::c_int {
        free(pdev as *mut libc::c_void);
        return res;
    }
    if ((*ctx).first).is_null() {
        (*ctx).first = pdev;
    } else {
        let mut prev: *mut freenect_device = (*ctx).first;
        while !((*prev).next).is_null() {
            prev = (*prev).next;
        }
        (*prev).next = pdev;
    }
    *dev = pdev;
    if !((*pdev).usb_cam.dev).is_null() {
        if freenect_camera_init(pdev) < 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn freenect_open_device_by_camera_serial(
    mut ctx: *mut freenect_context,
    mut dev: *mut *mut freenect_device,
    mut camera_serial: *const libc::c_char,
) -> libc::c_int {
    let mut attrlist: *mut freenect_device_attributes = 0
        as *mut freenect_device_attributes;
    let mut item: *mut freenect_device_attributes = 0 as *mut freenect_device_attributes;
    let mut count: libc::c_int = fnusb_list_device_attributes(ctx, &mut attrlist);
    if count < 0 as libc::c_int {
        fn_log(
            ctx,
            FREENECT_LOG_ERROR,
            b"freenect_open_device_by_camera_serial: Couldn't enumerate serial numbers\n\0"
                as *const u8 as *const libc::c_char,
        );
        return count;
    }
    let mut index: libc::c_int = 0 as libc::c_int;
    item = attrlist;
    while !item.is_null() {
        if strlen((*item).camera_serial) == strlen(camera_serial)
            && strcmp((*item).camera_serial, camera_serial) == 0 as libc::c_int
        {
            freenect_free_device_attributes(attrlist);
            return freenect_open_device(ctx, dev, index);
        }
        item = (*item).next;
        index += 1;
        index;
    }
    freenect_free_device_attributes(attrlist);
    fn_log(
        ctx,
        FREENECT_LOG_ERROR,
        b"freenect_open_device_by_camera_serial: Couldn't find a device with serial %s\n\0"
            as *const u8 as *const libc::c_char,
        camera_serial,
    );
    return -(1 as libc::c_int);
}
pub unsafe extern "C" fn freenect_close_device(
    mut dev: *mut freenect_device,
) -> libc::c_int {
    let mut ctx: *mut freenect_context = (*dev).parent;
    let mut res: libc::c_int = 0;
    if !((*dev).usb_cam.dev).is_null() {
        freenect_camera_teardown(dev);
    }
    res = fnusb_close_subdevices(dev);
    if res < 0 as libc::c_int {
        fn_log(
            ctx,
            FREENECT_LOG_ERROR,
            b"fnusb_close_subdevices failed: %d\n\0" as *const u8 as *const libc::c_char,
            res,
        );
        return res;
    }
    let mut last: *mut freenect_device = 0 as *mut freenect_device;
    let mut cur: *mut freenect_device = (*ctx).first;
    while !cur.is_null() && cur != dev {
        last = cur;
        cur = (*cur).next;
    }
    if cur.is_null() {
        fn_log(
            ctx,
            FREENECT_LOG_ERROR,
            b"device %p not found in linked list for this context!\n\0" as *const u8
                as *const libc::c_char,
            dev,
        );
        return -(1 as libc::c_int);
    }
    if !last.is_null() {
        (*last).next = (*cur).next;
    } else {
        (*ctx).first = (*cur).next;
    }
    free(dev as *mut libc::c_void);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn freenect_set_user(
    mut dev: *mut freenect_device,
    mut user: *mut libc::c_void,
) {
    (*dev).user_data = user;
}
pub unsafe extern "C" fn freenect_get_user(
    mut dev: *mut freenect_device,
) -> *mut libc::c_void {
    return (*dev).user_data;
}
pub unsafe extern "C" fn freenect_set_log_level(
    mut ctx: *mut freenect_context,
    mut level: freenect_loglevel,
) {
    (*ctx).log_level = level;
}
pub unsafe extern "C" fn freenect_set_log_callback(
    mut ctx: *mut freenect_context,
    mut cb: freenect_log_cb,
) {
    (*ctx).log_cb = cb;
}
pub unsafe extern "C" fn fn_log(
    mut ctx: *mut freenect_context,
    mut level: freenect_loglevel,
    mut fmt: *const libc::c_char,
    mut args: ...
) {
    let mut ap: ::std::ffi::VaListImpl;
    if level as libc::c_uint > (*ctx).log_level as libc::c_uint {
        return;
    }
    if ((*ctx).log_cb).is_some() {
        let mut msgbuf: [libc::c_char; 1024] = [0; 1024];
        ap = args.clone();
        vsnprintf(
            msgbuf.as_mut_ptr(),
            1024 as libc::c_int as libc::c_ulong,
            fmt,
            ap.as_va_list(),
        );
        msgbuf[1023 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        ((*ctx).log_cb).unwrap()(ctx, level, msgbuf.as_mut_ptr());
    } else {
        ap = args.clone();
        vfprintf(stderr, fmt, ap.as_va_list());
    };
}
pub unsafe extern "C" fn freenect_set_fw_address_nui(
    mut ctx: *mut freenect_context,
    mut fw_ptr: *mut libc::c_uchar,
    mut num_bytes: libc::c_uint,
) {
    (*ctx).fn_fw_nui_ptr = fw_ptr;
    (*ctx).fn_fw_nui_size = num_bytes;
}
pub unsafe extern "C" fn freenect_set_fw_address_k4w(
    mut ctx: *mut freenect_context,
    mut fw_ptr: *mut libc::c_uchar,
    mut num_bytes: libc::c_uint,
) {
    (*ctx).fn_fw_k4w_ptr = fw_ptr;
    (*ctx).fn_fw_k4w_size = num_bytes;
}
