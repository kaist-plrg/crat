use ::libc;
extern "C" {
    pub type libusb_device_handle;
    pub type libusb_context;
    pub type json_value_t;
    pub type json_object_t;
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn gettimeofday(__tv: *mut timeval, __tz: *mut libc::c_void) -> libc::c_int;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn sleep(__seconds: libc::c_uint) -> libc::c_uint;
    fn usleep(__useconds: __useconds_t) -> libc::c_int;
    fn json_parse_file(filename: *const libc::c_char) -> *mut JSON_Value;
    fn json_object_get_object(
        object: *const JSON_Object,
        name: *const libc::c_char,
    ) -> *mut JSON_Object;
    fn json_object_get_number(
        object: *const JSON_Object,
        name: *const libc::c_char,
    ) -> libc::c_double;
    fn json_value_free(value: *mut JSON_Value);
    fn json_object(value: *const JSON_Value) -> *mut JSON_Object;
    fn freenect_init_registration(dev: *mut freenect_device) -> libc::c_int;
    fn freenect_apply_registration(
        dev: *mut freenect_device,
        input: *mut uint8_t,
        output_mm: *mut uint16_t,
        unpacked: bool,
    ) -> libc::c_int;
    fn freenect_apply_depth_unpacked_to_mm(
        dev: *mut freenect_device,
        input: *mut uint16_t,
        output_mm: *mut uint16_t,
    ) -> libc::c_int;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fgetc(__stream: *mut FILE) -> libc::c_int;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn fseek(
        __stream: *mut FILE,
        __off: libc::c_long,
        __whence: libc::c_int,
    ) -> libc::c_int;
    fn ftell(__stream: *mut FILE) -> libc::c_long;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn __ctype_tolower_loc() -> *mut *const __int32_t;
}
pub type __int8_t = libc::c_schar;
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __useconds_t = libc::c_uint;
pub type __suseconds_t = libc::c_long;
pub type int8_t = __int8_t;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
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
    pub c2rust_unnamed: C2RustUnnamed,
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
pub union C2RustUnnamed {
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
pub type JSON_Value = json_value_t;
pub type JSON_Object = json_object_t;
pub type FILE = _IO_FILE;
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
pub type size_t = libc::c_ulong;
pub type _IO_lock_t = ();
pub unsafe extern "C" fn get_time() -> libc::c_double {
    let mut cur: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    gettimeofday(&mut cur, 0 as *mut libc::c_void);
    return cur.tv_sec as libc::c_double + cur.tv_usec as libc::c_double / 1000000.0f64;
}
pub unsafe extern "C" fn sleep_highres(mut tm: libc::c_double) {
    let mut sec: libc::c_int = floor(tm) as libc::c_int;
    let mut usec: libc::c_int = ((tm - sec as libc::c_double)
        * 1000000 as libc::c_int as libc::c_double) as libc::c_int;
    if tm > 0 as libc::c_int as libc::c_double {
        sleep(sec as libc::c_uint);
        usleep(usec as __useconds_t);
    }
}
#[inline]
unsafe extern "C" fn tolower(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_tolower_loc()).offset(__c as isize)
    } else {
        __c
    };
}
static mut fake_dev_singleton: freenect_device = freenect_device {
    parent: 0 as *const freenect_context as *mut freenect_context,
    next: 0 as *const freenect_device as *mut freenect_device,
    user_data: 0 as *const libc::c_void as *mut libc::c_void,
    usb_cam: fnusb_dev {
        parent: 0 as *const freenect_device as *mut freenect_device,
        dev: 0 as *const libusb_device_handle as *mut libusb_device_handle,
        device_dead: 0,
        VID: 0,
        PID: 0,
    },
    depth_isoc: fnusb_isoc_stream {
        parent: 0 as *const fnusb_dev as *mut fnusb_dev,
        xfers: 0 as *const *mut libusb_transfer as *mut *mut libusb_transfer,
        buffer: 0 as *const uint8_t as *mut uint8_t,
        cb: None,
        num_xfers: 0,
        pkts: 0,
        len: 0,
        dead: 0,
        dead_xfers: 0,
    },
    video_isoc: fnusb_isoc_stream {
        parent: 0 as *const fnusb_dev as *mut fnusb_dev,
        xfers: 0 as *const *mut libusb_transfer as *mut *mut libusb_transfer,
        buffer: 0 as *const uint8_t as *mut uint8_t,
        cb: None,
        num_xfers: 0,
        pkts: 0,
        len: 0,
        dead: 0,
        dead_xfers: 0,
    },
    depth_cb: None,
    video_cb: None,
    depth_chunk_cb: None,
    video_chunk_cb: None,
    video_format: FREENECT_VIDEO_RGB,
    depth_format: FREENECT_DEPTH_11BIT,
    video_resolution: FREENECT_RESOLUTION_LOW,
    depth_resolution: FREENECT_RESOLUTION_LOW,
    cam_inited: 0,
    cam_tag: 0,
    depth: packet_stream {
        running: 0,
        flag: 0,
        synced: 0,
        seq: 0,
        got_pkts: 0,
        pkt_num: 0,
        pkts_per_frame: 0,
        pkt_size: 0,
        frame_size: 0,
        last_pkt_size: 0,
        valid_pkts: 0,
        lost_pkts: 0,
        valid_frames: 0,
        variable_length: 0,
        last_timestamp: 0,
        timestamp: 0,
        split_bufs: 0,
        lib_buf: 0 as *const libc::c_void as *mut libc::c_void,
        usr_buf: 0 as *const libc::c_void as *mut libc::c_void,
        raw_buf: 0 as *const uint8_t as *mut uint8_t,
        proc_buf: 0 as *const libc::c_void as *mut libc::c_void,
    },
    video: packet_stream {
        running: 0,
        flag: 0,
        synced: 0,
        seq: 0,
        got_pkts: 0,
        pkt_num: 0,
        pkts_per_frame: 0,
        pkt_size: 0,
        frame_size: 0,
        last_pkt_size: 0,
        valid_pkts: 0,
        lost_pkts: 0,
        valid_frames: 0,
        variable_length: 0,
        last_timestamp: 0,
        timestamp: 0,
        split_bufs: 0,
        lib_buf: 0 as *const libc::c_void as *mut libc::c_void,
        usr_buf: 0 as *const libc::c_void as *mut libc::c_void,
        raw_buf: 0 as *const uint8_t as *mut uint8_t,
        proc_buf: 0 as *const libc::c_void as *mut libc::c_void,
    },
    registration: freenect_registration {
        reg_info: freenect_reg_info {
            dx_center: 0,
            ax: 0,
            bx: 0,
            cx: 0,
            dx: 0,
            dx_start: 0,
            ay: 0,
            by: 0,
            cy: 0,
            dy: 0,
            dy_start: 0,
            dx_beta_start: 0,
            dy_beta_start: 0,
            rollout_blank: 0,
            rollout_size: 0,
            dx_beta_inc: 0,
            dy_beta_inc: 0,
            dxdx_start: 0,
            dxdy_start: 0,
            dydx_start: 0,
            dydy_start: 0,
            dxdxdx_start: 0,
            dydxdx_start: 0,
            dxdxdy_start: 0,
            dydxdy_start: 0,
            back_comp1: 0,
            dydydx_start: 0,
            back_comp2: 0,
            dydydy_start: 0,
        },
        reg_pad_info: freenect_reg_pad_info {
            start_lines: 0,
            end_lines: 0,
            cropping_lines: 0,
        },
        zero_plane_info: freenect_zero_plane_info {
            dcmos_emitter_dist: 0.,
            dcmos_rcmos_dist: 0.,
            reference_distance: 0.,
            reference_pixel_size: 0.,
        },
        const_shift: 0.,
        raw_to_mm_shift: 0 as *const uint16_t as *mut uint16_t,
        depth_to_rgb_shift: 0 as *const int32_t as *mut int32_t,
        registration_table: 0 as *const [int32_t; 2] as *mut [int32_t; 2],
    },
    usb_audio: fnusb_dev {
        parent: 0 as *const freenect_device as *mut freenect_device,
        dev: 0 as *const libusb_device_handle as *mut libusb_device_handle,
        device_dead: 0,
        VID: 0,
        PID: 0,
    },
    audio_out_isoc: fnusb_isoc_stream {
        parent: 0 as *const fnusb_dev as *mut fnusb_dev,
        xfers: 0 as *const *mut libusb_transfer as *mut *mut libusb_transfer,
        buffer: 0 as *const uint8_t as *mut uint8_t,
        cb: None,
        num_xfers: 0,
        pkts: 0,
        len: 0,
        dead: 0,
        dead_xfers: 0,
    },
    audio_in_isoc: fnusb_isoc_stream {
        parent: 0 as *const fnusb_dev as *mut fnusb_dev,
        xfers: 0 as *const *mut libusb_transfer as *mut *mut libusb_transfer,
        buffer: 0 as *const uint8_t as *mut uint8_t,
        cb: None,
        num_xfers: 0,
        pkts: 0,
        len: 0,
        dead: 0,
        dead_xfers: 0,
    },
    audio_in_cb: None,
    audio_out_cb: None,
    audio: audio_stream {
        running: 0,
        audio_out_ring: 0 as *const freenect_sample_51 as *mut freenect_sample_51,
        ring_reader_idx: 0,
        ring_writer_idx: 0,
        out_window: 0,
        out_seq: 0,
        out_counter_within_window: 0,
        out_weird_timestamp: 0,
        out_window_parity: 0,
        in_window: 0,
        last_seen_window: [0; 10],
        in_counter: 0,
        mic_buffer: [0 as *const int32_t as *mut int32_t; 4],
        cancelled_buffer: 0 as *const int16_t as *mut int16_t,
        in_unknown: 0 as *const libc::c_void as *mut libc::c_void,
    },
    audio_tag: 0,
    usb_motor: fnusb_dev {
        parent: 0 as *const freenect_device as *mut freenect_device,
        dev: 0 as *const libusb_device_handle as *mut libusb_device_handle,
        device_dead: 0,
        VID: 0,
        PID: 0,
    },
    raw_state: freenect_raw_tilt_state {
        accelerometer_x: 0,
        accelerometer_y: 0,
        accelerometer_z: 0,
        tilt_angle: 0,
        tilt_status: TILT_STATUS_STOPPED,
    },
    device_does_motor_control_with_audio: 0,
    motor_control_with_audio_enabled: 0,
};
static mut fake_dev: *mut freenect_device = unsafe {
    &fake_dev_singleton as *const freenect_device as *mut freenect_device
};
static mut fake_ctx: *mut freenect_context = unsafe {
    5678 as libc::c_int as *mut freenect_context
};
static mut cur_depth_cb: freenect_depth_cb = None;
static mut cur_video_cb: freenect_video_cb = None;
static mut input_path: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut index_fp: *mut FILE = 0 as *const FILE as *mut FILE;
static mut state: freenect_raw_tilt_state = {
    let mut init = freenect_raw_tilt_state {
        accelerometer_x: 0 as libc::c_int as int16_t,
        accelerometer_y: 0,
        accelerometer_z: 0,
        tilt_angle: 0,
        tilt_status: TILT_STATUS_STOPPED,
    };
    init
};
static mut ir_brightness: uint16_t = 25 as libc::c_int as uint16_t;
static mut already_warned: libc::c_int = 0 as libc::c_int;
static mut playback_prev_time: libc::c_double = 0.0f64;
static mut record_prev_time: libc::c_double = 0.0f64;
static mut user_depth_buf: *mut libc::c_void = 0 as *const libc::c_void
    as *mut libc::c_void;
static mut user_video_buf: *mut libc::c_void = 0 as *const libc::c_void
    as *mut libc::c_void;
static mut depth_running: libc::c_int = 0 as libc::c_int;
static mut rgb_running: libc::c_int = 0 as libc::c_int;
static mut user_ptr: *mut libc::c_void = 0 as *const libc::c_void as *mut libc::c_void;
static mut loop_playback: bool = 1 as libc::c_int != 0;
static mut rgb_video_mode: freenect_frame_mode = {
    let mut init = freenect_frame_mode {
        reserved: ((FREENECT_RESOLUTION_MEDIUM as libc::c_int & 0xff as libc::c_int)
            << 8 as libc::c_int
            | FREENECT_VIDEO_RGB as libc::c_int & 0xff as libc::c_int) as uint32_t,
        resolution: FREENECT_RESOLUTION_MEDIUM,
        c2rust_unnamed: C2RustUnnamed {
            dummy: FREENECT_VIDEO_RGB as libc::c_int,
        },
        bytes: 640 as libc::c_int * 480 as libc::c_int * 3 as libc::c_int,
        width: 640 as libc::c_int as int16_t,
        height: 480 as libc::c_int as int16_t,
        data_bits_per_pixel: 24 as libc::c_int as int8_t,
        padding_bits_per_pixel: 0 as libc::c_int as int8_t,
        framerate: 30 as libc::c_int as int8_t,
        is_valid: 1 as libc::c_int as int8_t,
    };
    init
};
static mut yuv_video_mode: freenect_frame_mode = {
    let mut init = freenect_frame_mode {
        reserved: ((FREENECT_RESOLUTION_MEDIUM as libc::c_int & 0xff as libc::c_int)
            << 8 as libc::c_int
            | FREENECT_VIDEO_YUV_RAW as libc::c_int & 0xff as libc::c_int) as uint32_t,
        resolution: FREENECT_RESOLUTION_MEDIUM,
        c2rust_unnamed: C2RustUnnamed {
            dummy: FREENECT_VIDEO_YUV_RAW as libc::c_int,
        },
        bytes: 640 as libc::c_int * 480 as libc::c_int * 2 as libc::c_int,
        width: 640 as libc::c_int as int16_t,
        height: 480 as libc::c_int as int16_t,
        data_bits_per_pixel: 16 as libc::c_int as int8_t,
        padding_bits_per_pixel: 0 as libc::c_int as int8_t,
        framerate: 15 as libc::c_int as int8_t,
        is_valid: 1 as libc::c_int as int8_t,
    };
    init
};
static mut video_mode: freenect_frame_mode = freenect_frame_mode {
    reserved: 0,
    resolution: FREENECT_RESOLUTION_LOW,
    c2rust_unnamed: C2RustUnnamed { dummy: 0 },
    bytes: 0,
    width: 0,
    height: 0,
    data_bits_per_pixel: 0,
    padding_bits_per_pixel: 0,
    framerate: 0,
    is_valid: 0,
};
static mut depth_11_mode: freenect_frame_mode = {
    let mut init = freenect_frame_mode {
        reserved: ((FREENECT_RESOLUTION_MEDIUM as libc::c_int & 0xff as libc::c_int)
            << 8 as libc::c_int
            | FREENECT_DEPTH_11BIT as libc::c_int & 0xff as libc::c_int) as uint32_t,
        resolution: FREENECT_RESOLUTION_MEDIUM,
        c2rust_unnamed: C2RustUnnamed {
            dummy: FREENECT_DEPTH_11BIT as libc::c_int,
        },
        bytes: 640 as libc::c_int * 480 as libc::c_int * 2 as libc::c_int,
        width: 640 as libc::c_int as int16_t,
        height: 480 as libc::c_int as int16_t,
        data_bits_per_pixel: 11 as libc::c_int as int8_t,
        padding_bits_per_pixel: 5 as libc::c_int as int8_t,
        framerate: 30 as libc::c_int as int8_t,
        is_valid: 1 as libc::c_int as int8_t,
    };
    init
};
static mut depth_registered_mode: freenect_frame_mode = {
    let mut init = freenect_frame_mode {
        reserved: ((FREENECT_RESOLUTION_MEDIUM as libc::c_int & 0xff as libc::c_int)
            << 8 as libc::c_int
            | FREENECT_DEPTH_REGISTERED as libc::c_int & 0xff as libc::c_int)
            as uint32_t,
        resolution: FREENECT_RESOLUTION_MEDIUM,
        c2rust_unnamed: C2RustUnnamed {
            dummy: FREENECT_DEPTH_REGISTERED as libc::c_int,
        },
        bytes: 640 as libc::c_int * 480 as libc::c_int * 2 as libc::c_int,
        width: 640 as libc::c_int as int16_t,
        height: 480 as libc::c_int as int16_t,
        data_bits_per_pixel: 16 as libc::c_int as int8_t,
        padding_bits_per_pixel: 0 as libc::c_int as int8_t,
        framerate: 30 as libc::c_int as int8_t,
        is_valid: 1 as libc::c_int as int8_t,
    };
    init
};
static mut depth_mm_mode: freenect_frame_mode = {
    let mut init = freenect_frame_mode {
        reserved: ((FREENECT_RESOLUTION_MEDIUM as libc::c_int & 0xff as libc::c_int)
            << 8 as libc::c_int | FREENECT_DEPTH_MM as libc::c_int & 0xff as libc::c_int)
            as uint32_t,
        resolution: FREENECT_RESOLUTION_MEDIUM,
        c2rust_unnamed: C2RustUnnamed {
            dummy: FREENECT_DEPTH_MM as libc::c_int,
        },
        bytes: 640 as libc::c_int * 480 as libc::c_int * 2 as libc::c_int,
        width: 640 as libc::c_int as int16_t,
        height: 480 as libc::c_int as int16_t,
        data_bits_per_pixel: 16 as libc::c_int as int8_t,
        padding_bits_per_pixel: 0 as libc::c_int as int8_t,
        framerate: 30 as libc::c_int as int8_t,
        is_valid: 1 as libc::c_int as int8_t,
    };
    init
};
static mut depth_mode: freenect_frame_mode = freenect_frame_mode {
    reserved: 0,
    resolution: FREENECT_RESOLUTION_LOW,
    c2rust_unnamed: C2RustUnnamed { dummy: 0 },
    bytes: 0,
    width: 0,
    height: 0,
    data_bits_per_pixel: 0,
    padding_bits_per_pixel: 0,
    framerate: 0,
    is_valid: 0,
};
static mut default_video_back: *mut libc::c_void = 0 as *const libc::c_void
    as *mut libc::c_void;
static mut default_depth_back: *mut libc::c_void = 0 as *const libc::c_void
    as *mut libc::c_void;
unsafe extern "C" fn one_line(mut fp: *mut FILE) -> *mut libc::c_char {
    let mut c: libc::c_int = 0;
    let mut pos: libc::c_int = 0 as libc::c_int;
    let mut out: *mut libc::c_char = 0 as *mut libc::c_char;
    c = fgetc(fp);
    while !(c == '\n' as i32 || c == -(1 as libc::c_int)) {
        out = realloc(
            out as *mut libc::c_void,
            (pos + 1 as libc::c_int) as libc::c_ulong,
        ) as *mut libc::c_char;
        let fresh0 = pos;
        pos = pos + 1;
        *out.offset(fresh0 as isize) = c as libc::c_char;
        c = fgetc(fp);
    }
    if !out.is_null() {
        out = realloc(
            out as *mut libc::c_void,
            (pos + 1 as libc::c_int) as libc::c_ulong,
        ) as *mut libc::c_char;
        *out.offset(pos as isize) = '\0' as i32 as libc::c_char;
    }
    return out;
}
unsafe extern "C" fn get_data_size(mut fp: *mut FILE) -> libc::c_int {
    let mut orig: libc::c_int = ftell(fp) as libc::c_int;
    fseek(fp, 0 as libc::c_long, 2 as libc::c_int);
    let mut out: libc::c_int = ftell(fp) as libc::c_int;
    fseek(fp, orig as libc::c_long, 0 as libc::c_int);
    return out;
}
unsafe extern "C" fn parse_line(
    mut type_0: *mut libc::c_char,
    mut cur_time: *mut libc::c_double,
    mut timestamp: *mut libc::c_uint,
    mut data_size: *mut libc::c_uint,
    mut data: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut line: *mut libc::c_char = one_line(index_fp);
    if line.is_null() {
        printf(
            b"Warning: No more lines in [%s]\n\0" as *const u8 as *const libc::c_char,
            input_path,
        );
        return -(1 as libc::c_int);
    }
    let mut file_path_size: libc::c_int = (strlen(input_path))
        .wrapping_add(strlen(line))
        .wrapping_add(50 as libc::c_int as libc::c_ulong) as libc::c_int;
    let mut file_path: *mut libc::c_char = malloc(file_path_size as libc::c_ulong)
        as *mut libc::c_char;
    snprintf(
        file_path,
        file_path_size as libc::c_ulong,
        b"%s/%s\0" as *const u8 as *const libc::c_char,
        input_path,
        line,
    );
    let mut cur_fp: *mut FILE = fopen(
        file_path,
        b"rb\0" as *const u8 as *const libc::c_char,
    );
    if cur_fp.is_null() {
        printf(
            b"Error: Cannot open file [%s]\n\0" as *const u8 as *const libc::c_char,
            file_path,
        );
        exit(1 as libc::c_int);
    }
    let mut ret: libc::c_int = 0 as libc::c_int;
    *data_size = get_data_size(cur_fp) as libc::c_uint;
    sscanf(
        line,
        b"%c-%lf-%u-%*s\0" as *const u8 as *const libc::c_char,
        type_0,
        cur_time,
        timestamp,
    );
    *data = malloc(*data_size as libc::c_ulong) as *mut libc::c_char;
    if fread(
        *data as *mut libc::c_void,
        *data_size as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        cur_fp,
    ) != 1 as libc::c_int as libc::c_ulong
    {
        printf(
            b"Error: Couldn't read entire file.\n\0" as *const u8 as *const libc::c_char,
        );
        ret = -(1 as libc::c_int);
    }
    fclose(cur_fp);
    free(line as *mut libc::c_void);
    free(file_path as *mut libc::c_void);
    return ret;
}
unsafe extern "C" fn open_index() {
    let mut index_path_size: libc::c_int = (strlen(input_path))
        .wrapping_add(50 as libc::c_int as libc::c_ulong) as libc::c_int;
    let mut index_path: *mut libc::c_char = malloc(index_path_size as libc::c_ulong)
        as *mut libc::c_char;
    snprintf(
        index_path,
        index_path_size as libc::c_ulong,
        b"%s/INDEX.txt\0" as *const u8 as *const libc::c_char,
        input_path,
    );
    index_fp = fopen(index_path, b"rb\0" as *const u8 as *const libc::c_char);
    if index_fp.is_null() {
        printf(
            b"Error: Cannot open file [%s]\n\0" as *const u8 as *const libc::c_char,
            index_path,
        );
        exit(1 as libc::c_int);
    }
    free(index_path as *mut libc::c_void);
}
unsafe extern "C" fn close_index() {
    fclose(index_fp);
    index_fp = 0 as *mut FILE;
    record_prev_time = 0 as libc::c_int as libc::c_double;
    playback_prev_time = 0 as libc::c_int as libc::c_double;
}
unsafe extern "C" fn skip_line(mut str: *mut libc::c_char) -> *mut libc::c_char {
    let mut out: *mut libc::c_char = strchr(str, '\n' as i32);
    if out.is_null() {
        printf(
            b"Error: PGM/PPM has incorrect formatting, expected a header on one line followed by a newline\n\0"
                as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    return out.offset(1 as libc::c_int as isize);
}
unsafe extern "C" fn convert_rgb_to_uyvy(
    mut rgb_buffer: *mut uint8_t,
    mut yuv_buffer: *mut uint8_t,
    mut mode: freenect_frame_mode,
) {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    y = 0 as libc::c_int;
    while y < mode.height as libc::c_int {
        x = 0 as libc::c_int;
        while x < mode.width as libc::c_int {
            let mut pos: libc::c_int = y * mode.width as libc::c_int + x;
            let mut rgb0: *mut uint8_t = rgb_buffer
                .offset((pos * 3 as libc::c_int) as isize);
            let mut rgb1: *mut uint8_t = rgb_buffer
                .offset(((pos + 1 as libc::c_int) * 3 as libc::c_int) as isize);
            let mut y0: libc::c_float = 0.257f32
                * *rgb0.offset(0 as libc::c_int as isize) as libc::c_int as libc::c_float
                + 0.504f32
                    * *rgb0.offset(1 as libc::c_int as isize) as libc::c_int
                        as libc::c_float
                + 0.098f32
                    * *rgb0.offset(2 as libc::c_int as isize) as libc::c_int
                        as libc::c_float + 16 as libc::c_int as libc::c_float;
            let mut u0: libc::c_float = -(0.148f32
                * *rgb0.offset(0 as libc::c_int as isize) as libc::c_int
                    as libc::c_float)
                - 0.291f32
                    * *rgb0.offset(1 as libc::c_int as isize) as libc::c_int
                        as libc::c_float
                + 0.439f32
                    * *rgb0.offset(2 as libc::c_int as isize) as libc::c_int
                        as libc::c_float + 128 as libc::c_int as libc::c_float;
            let mut v0: libc::c_float = 0.439f32
                * *rgb0.offset(0 as libc::c_int as isize) as libc::c_int as libc::c_float
                - 0.368f32
                    * *rgb0.offset(1 as libc::c_int as isize) as libc::c_int
                        as libc::c_float
                - 0.071f32
                    * *rgb0.offset(2 as libc::c_int as isize) as libc::c_int
                        as libc::c_float + 128 as libc::c_int as libc::c_float;
            let mut y1: libc::c_float = 0.257f32
                * *rgb1.offset(0 as libc::c_int as isize) as libc::c_int as libc::c_float
                + 0.504f32
                    * *rgb1.offset(1 as libc::c_int as isize) as libc::c_int
                        as libc::c_float
                + 0.098f32
                    * *rgb1.offset(2 as libc::c_int as isize) as libc::c_int
                        as libc::c_float + 16 as libc::c_int as libc::c_float;
            let mut u1: libc::c_float = -(0.148f32
                * *rgb1.offset(0 as libc::c_int as isize) as libc::c_int
                    as libc::c_float)
                - 0.291f32
                    * *rgb1.offset(1 as libc::c_int as isize) as libc::c_int
                        as libc::c_float
                + 0.439f32
                    * *rgb1.offset(2 as libc::c_int as isize) as libc::c_int
                        as libc::c_float + 128 as libc::c_int as libc::c_float;
            let mut v1: libc::c_float = 0.439f32
                * *rgb1.offset(0 as libc::c_int as isize) as libc::c_int as libc::c_float
                - 0.368f32
                    * *rgb1.offset(1 as libc::c_int as isize) as libc::c_int
                        as libc::c_float
                - 0.071f32
                    * *rgb1.offset(2 as libc::c_int as isize) as libc::c_int
                        as libc::c_float + 128 as libc::c_int as libc::c_float;
            let mut uyvy: *mut uint8_t = yuv_buffer
                .offset((pos * 2 as libc::c_int) as isize);
            *uyvy.offset(0 as libc::c_int as isize) = ((u0 + u1) / 2.0f32) as uint8_t;
            *uyvy.offset(1 as libc::c_int as isize) = y0 as uint8_t;
            *uyvy.offset(2 as libc::c_int as isize) = ((v0 + v1) / 2.0f32) as uint8_t;
            *uyvy.offset(3 as libc::c_int as isize) = y1 as uint8_t;
            x += 2 as libc::c_int;
        }
        y += 1;
        y;
    }
}
pub unsafe extern "C" fn freenect_process_events(
    mut ctx: *mut freenect_context,
) -> libc::c_int {
    if index_fp.is_null() {
        open_index();
    }
    let mut type_0: libc::c_char = 0;
    let mut record_cur_time: libc::c_double = 0.;
    let mut timestamp: libc::c_uint = 0;
    let mut data_size: libc::c_uint = 0;
    let mut data: *mut libc::c_char = 0 as *mut libc::c_char;
    if parse_line(
        &mut type_0,
        &mut record_cur_time,
        &mut timestamp,
        &mut data_size,
        &mut data,
    ) != 0
    {
        if loop_playback {
            close_index();
            return 0 as libc::c_int;
        } else {
            return -(1 as libc::c_int)
        }
    }
    if record_prev_time != 0.0f64 && playback_prev_time != 0.0f64 {
        sleep_highres(
            record_cur_time - record_prev_time - (get_time() - playback_prev_time),
        );
    }
    record_prev_time = record_cur_time;
    match type_0 as libc::c_int {
        100 => {
            if cur_depth_cb.is_some() && depth_running != 0 {
                let mut mode: freenect_frame_mode = freenect_get_current_depth_mode(
                    fake_dev,
                );
                let mut cur_depth: *mut libc::c_void = skip_line(data)
                    as *mut libc::c_void;
                let mut depth_buffer: *mut libc::c_void = if !user_depth_buf.is_null() {
                    user_depth_buf
                } else {
                    default_depth_back
                };
                match mode.c2rust_unnamed.depth_format as libc::c_uint {
                    0 => {
                        memcpy(depth_buffer, cur_depth, mode.bytes as libc::c_ulong);
                    }
                    4 => {
                        freenect_apply_registration(
                            fake_dev,
                            cur_depth as *mut uint8_t,
                            depth_buffer as *mut uint16_t,
                            1 as libc::c_int != 0,
                        );
                    }
                    5 => {
                        freenect_apply_depth_unpacked_to_mm(
                            fake_dev,
                            cur_depth as *mut uint16_t,
                            depth_buffer as *mut uint16_t,
                        );
                    }
                    _ => {
                        __assert_fail(
                            b"0\0" as *const u8 as *const libc::c_char,
                            b"../../fakenect/fakenect.c\0" as *const u8
                                as *const libc::c_char,
                            260 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 48],
                                &[libc::c_char; 48],
                            >(b"int freenect_process_events(freenect_context *)\0"))
                                .as_ptr(),
                        );
                        'c_2585: {
                            __assert_fail(
                                b"0\0" as *const u8 as *const libc::c_char,
                                b"../../fakenect/fakenect.c\0" as *const u8
                                    as *const libc::c_char,
                                260 as libc::c_int as libc::c_uint,
                                (*::std::mem::transmute::<
                                    &[u8; 48],
                                    &[libc::c_char; 48],
                                >(b"int freenect_process_events(freenect_context *)\0"))
                                    .as_ptr(),
                            );
                        };
                    }
                }
                cur_depth_cb.unwrap()(fake_dev, depth_buffer, timestamp);
            }
        }
        114 => {
            if cur_video_cb.is_some() && rgb_running != 0 {
                let mut cur_video: *mut libc::c_void = skip_line(data)
                    as *mut libc::c_void;
                let mut video_buffer: *mut libc::c_void = if !user_video_buf.is_null() {
                    user_video_buf
                } else {
                    default_video_back
                };
                let mut mode_0: freenect_frame_mode = freenect_get_current_video_mode(
                    fake_dev,
                );
                match mode_0.c2rust_unnamed.video_format as libc::c_uint {
                    0 => {
                        memcpy(video_buffer, cur_video, mode_0.bytes as libc::c_ulong);
                    }
                    6 => {
                        convert_rgb_to_uyvy(
                            cur_video as *mut uint8_t,
                            video_buffer as *mut uint8_t,
                            mode_0,
                        );
                    }
                    _ => {
                        __assert_fail(
                            b"0\0" as *const u8 as *const libc::c_char,
                            b"../../fakenect/fakenect.c\0" as *const u8
                                as *const libc::c_char,
                            282 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 48],
                                &[libc::c_char; 48],
                            >(b"int freenect_process_events(freenect_context *)\0"))
                                .as_ptr(),
                        );
                        'c_2040: {
                            __assert_fail(
                                b"0\0" as *const u8 as *const libc::c_char,
                                b"../../fakenect/fakenect.c\0" as *const u8
                                    as *const libc::c_char,
                                282 as libc::c_int as libc::c_uint,
                                (*::std::mem::transmute::<
                                    &[u8; 48],
                                    &[libc::c_char; 48],
                                >(b"int freenect_process_events(freenect_context *)\0"))
                                    .as_ptr(),
                            );
                        };
                    }
                }
                cur_video_cb.unwrap()(fake_dev, video_buffer, timestamp);
            }
        }
        97 => {
            if data_size as libc::c_ulong
                == ::std::mem::size_of::<freenect_raw_tilt_state>() as libc::c_ulong
            {
                memcpy(
                    &mut state as *mut freenect_raw_tilt_state as *mut libc::c_void,
                    data as *const libc::c_void,
                    ::std::mem::size_of::<freenect_raw_tilt_state>() as libc::c_ulong,
                );
            } else if already_warned == 0 {
                already_warned = 1 as libc::c_int;
                printf(
                    b"\n\nWarning: Accelerometer data has an unexpected size [%u] instead of [%u].  The acceleration and tilt data will be substituted for dummy values.  This data was probably made with an older version of record (the upstream interface changed).\n\n\0"
                        as *const u8 as *const libc::c_char,
                    data_size,
                    ::std::mem::size_of::<freenect_raw_tilt_state>() as libc::c_ulong
                        as libc::c_uint,
                );
            }
        }
        _ => {}
    }
    free(data as *mut libc::c_void);
    playback_prev_time = get_time();
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn freenect_process_events_timeout(
    mut ctx: *mut freenect_context,
    mut timeout: *mut timeval,
) -> libc::c_int {
    return freenect_process_events(ctx);
}
pub unsafe extern "C" fn freenect_get_tilt_degs(
    mut state_0: *mut freenect_raw_tilt_state,
) -> libc::c_double {
    return (*state_0).tilt_angle as libc::c_double / 2.0f64;
}
pub unsafe extern "C" fn freenect_get_tilt_state(
    mut dev: *mut freenect_device,
) -> *mut freenect_raw_tilt_state {
    return &mut state;
}
pub unsafe extern "C" fn freenect_get_tilt_status(
    mut state_0: *mut freenect_raw_tilt_state,
) -> freenect_tilt_status_code {
    return (*state_0).tilt_status;
}
pub unsafe extern "C" fn freenect_get_mks_accel(
    mut state_0: *mut freenect_raw_tilt_state,
    mut x: *mut libc::c_double,
    mut y: *mut libc::c_double,
    mut z: *mut libc::c_double,
) {
    *x = (*state_0).accelerometer_x as libc::c_double
        / 819 as libc::c_int as libc::c_double * 9.80665f64;
    *y = (*state_0).accelerometer_y as libc::c_double
        / 819 as libc::c_int as libc::c_double * 9.80665f64;
    *z = (*state_0).accelerometer_z as libc::c_double
        / 819 as libc::c_int as libc::c_double * 9.80665f64;
}
pub unsafe extern "C" fn freenect_set_depth_callback(
    mut dev: *mut freenect_device,
    mut cb: freenect_depth_cb,
) {
    cur_depth_cb = cb;
}
pub unsafe extern "C" fn freenect_set_video_callback(
    mut dev: *mut freenect_device,
    mut cb: freenect_video_cb,
) {
    cur_video_cb = cb;
}
pub unsafe extern "C" fn freenect_set_video_mode(
    mut dev: *mut freenect_device,
    mode: freenect_frame_mode,
) -> libc::c_int {
    video_mode = mode;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn freenect_set_depth_mode(
    mut dev: *mut freenect_device,
    mode: freenect_frame_mode,
) -> libc::c_int {
    depth_mode = mode;
    if (mode.c2rust_unnamed.depth_format as libc::c_uint
        == FREENECT_DEPTH_MM as libc::c_int as libc::c_uint
        || mode.c2rust_unnamed.depth_format as libc::c_uint
            == FREENECT_DEPTH_REGISTERED as libc::c_int as libc::c_uint)
        && (*dev).registration.zero_plane_info.reference_distance
            == 0 as libc::c_int as libc::c_float
    {
        printf(
            b"Warning: older fakenect recording doesn't contain registration info for mapping depth to MM units\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn freenect_find_video_mode(
    mut res: freenect_resolution,
    mut fmt: freenect_video_format,
) -> freenect_frame_mode {
    if FREENECT_RESOLUTION_MEDIUM as libc::c_int as libc::c_uint == res as libc::c_uint
    {} else {
        __assert_fail(
            b"FREENECT_RESOLUTION_MEDIUM == res\0" as *const u8 as *const libc::c_char,
            b"../../fakenect/fakenect.c\0" as *const u8 as *const libc::c_char,
            374 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 89],
                &[libc::c_char; 89],
            >(
                b"freenect_frame_mode freenect_find_video_mode(freenect_resolution, freenect_video_format)\0",
            ))
                .as_ptr(),
        );
    }
    'c_3835: {
        if FREENECT_RESOLUTION_MEDIUM as libc::c_int as libc::c_uint
            == res as libc::c_uint
        {} else {
            __assert_fail(
                b"FREENECT_RESOLUTION_MEDIUM == res\0" as *const u8
                    as *const libc::c_char,
                b"../../fakenect/fakenect.c\0" as *const u8 as *const libc::c_char,
                374 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 89],
                    &[libc::c_char; 89],
                >(
                    b"freenect_frame_mode freenect_find_video_mode(freenect_resolution, freenect_video_format)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    match fmt as libc::c_uint {
        0 => return rgb_video_mode,
        6 => return yuv_video_mode,
        _ => {
            __assert_fail(
                b"0\0" as *const u8 as *const libc::c_char,
                b"../../fakenect/fakenect.c\0" as *const u8 as *const libc::c_char,
                382 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 89],
                    &[libc::c_char; 89],
                >(
                    b"freenect_frame_mode freenect_find_video_mode(freenect_resolution, freenect_video_format)\0",
                ))
                    .as_ptr(),
            );
            'c_3790: {
                __assert_fail(
                    b"0\0" as *const u8 as *const libc::c_char,
                    b"../../fakenect/fakenect.c\0" as *const u8 as *const libc::c_char,
                    382 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 89],
                        &[libc::c_char; 89],
                    >(
                        b"freenect_frame_mode freenect_find_video_mode(freenect_resolution, freenect_video_format)\0",
                    ))
                        .as_ptr(),
                );
            };
        }
    }
    let mut invalid: freenect_frame_mode = {
        let mut init = freenect_frame_mode {
            reserved: 0 as libc::c_int as uint32_t,
            resolution: FREENECT_RESOLUTION_LOW,
            c2rust_unnamed: C2RustUnnamed { dummy: 0 },
            bytes: 0,
            width: 0,
            height: 0,
            data_bits_per_pixel: 0,
            padding_bits_per_pixel: 0,
            framerate: 0,
            is_valid: 0,
        };
        init
    };
    return invalid;
}
pub unsafe extern "C" fn freenect_get_video_mode_count() -> libc::c_int {
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn freenect_get_video_mode(
    mut mode_num: libc::c_int,
) -> freenect_frame_mode {
    if mode_num == 0 as libc::c_int {
        return rgb_video_mode
    } else if mode_num == 1 as libc::c_int {
        return yuv_video_mode
    } else {
        let mut invalid: freenect_frame_mode = {
            let mut init = freenect_frame_mode {
                reserved: 0 as libc::c_int as uint32_t,
                resolution: FREENECT_RESOLUTION_LOW,
                c2rust_unnamed: C2RustUnnamed { dummy: 0 },
                bytes: 0,
                width: 0,
                height: 0,
                data_bits_per_pixel: 0,
                padding_bits_per_pixel: 0,
                framerate: 0,
                is_valid: 0,
            };
            init
        };
        return invalid;
    };
}
pub unsafe extern "C" fn freenect_get_current_video_mode(
    mut dev: *mut freenect_device,
) -> freenect_frame_mode {
    return video_mode;
}
pub unsafe extern "C" fn freenect_find_depth_mode(
    mut res: freenect_resolution,
    mut fmt: freenect_depth_format,
) -> freenect_frame_mode {
    if FREENECT_RESOLUTION_MEDIUM as libc::c_int as libc::c_uint == res as libc::c_uint
    {} else {
        __assert_fail(
            b"FREENECT_RESOLUTION_MEDIUM == res\0" as *const u8 as *const libc::c_char,
            b"../../fakenect/fakenect.c\0" as *const u8 as *const libc::c_char,
            413 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 89],
                &[libc::c_char; 89],
            >(
                b"freenect_frame_mode freenect_find_depth_mode(freenect_resolution, freenect_depth_format)\0",
            ))
                .as_ptr(),
        );
    }
    'c_4103: {
        if FREENECT_RESOLUTION_MEDIUM as libc::c_int as libc::c_uint
            == res as libc::c_uint
        {} else {
            __assert_fail(
                b"FREENECT_RESOLUTION_MEDIUM == res\0" as *const u8
                    as *const libc::c_char,
                b"../../fakenect/fakenect.c\0" as *const u8 as *const libc::c_char,
                413 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 89],
                    &[libc::c_char; 89],
                >(
                    b"freenect_frame_mode freenect_find_depth_mode(freenect_resolution, freenect_depth_format)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    match fmt as libc::c_uint {
        0 => return depth_11_mode,
        4 => return depth_registered_mode,
        5 => return depth_mm_mode,
        _ => {
            __assert_fail(
                b"0\0" as *const u8 as *const libc::c_char,
                b"../../fakenect/fakenect.c\0" as *const u8 as *const libc::c_char,
                423 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 89],
                    &[libc::c_char; 89],
                >(
                    b"freenect_frame_mode freenect_find_depth_mode(freenect_resolution, freenect_depth_format)\0",
                ))
                    .as_ptr(),
            );
            'c_4053: {
                __assert_fail(
                    b"0\0" as *const u8 as *const libc::c_char,
                    b"../../fakenect/fakenect.c\0" as *const u8 as *const libc::c_char,
                    423 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 89],
                        &[libc::c_char; 89],
                    >(
                        b"freenect_frame_mode freenect_find_depth_mode(freenect_resolution, freenect_depth_format)\0",
                    ))
                        .as_ptr(),
                );
            };
        }
    }
    let mut invalid: freenect_frame_mode = {
        let mut init = freenect_frame_mode {
            reserved: 0 as libc::c_int as uint32_t,
            resolution: FREENECT_RESOLUTION_LOW,
            c2rust_unnamed: C2RustUnnamed { dummy: 0 },
            bytes: 0,
            width: 0,
            height: 0,
            data_bits_per_pixel: 0,
            padding_bits_per_pixel: 0,
            framerate: 0,
            is_valid: 0,
        };
        init
    };
    return invalid;
}
pub unsafe extern "C" fn freenect_get_depth_mode_count() -> libc::c_int {
    return 3 as libc::c_int;
}
pub unsafe extern "C" fn freenect_get_depth_mode(
    mut mode_num: libc::c_int,
) -> freenect_frame_mode {
    if mode_num == 0 as libc::c_int {
        return depth_11_mode
    } else if mode_num == 1 as libc::c_int {
        return depth_mm_mode
    } else if mode_num == 2 as libc::c_int {
        return depth_registered_mode
    } else {
        let mut invalid: freenect_frame_mode = {
            let mut init = freenect_frame_mode {
                reserved: 0 as libc::c_int as uint32_t,
                resolution: FREENECT_RESOLUTION_LOW,
                c2rust_unnamed: C2RustUnnamed { dummy: 0 },
                bytes: 0,
                width: 0,
                height: 0,
                data_bits_per_pixel: 0,
                padding_bits_per_pixel: 0,
                framerate: 0,
                is_valid: 0,
            };
            init
        };
        return invalid;
    };
}
pub unsafe extern "C" fn freenect_get_current_depth_mode(
    mut dev: *mut freenect_device,
) -> freenect_frame_mode {
    return depth_mode;
}
pub unsafe extern "C" fn freenect_num_devices(
    mut ctx: *mut freenect_context,
) -> libc::c_int {
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn freenect_open_device(
    mut ctx: *mut freenect_context,
    mut dev: *mut *mut freenect_device,
    mut index: libc::c_int,
) -> libc::c_int {
    *dev = fake_dev;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn freenect_open_device_by_camera_serial(
    mut ctx: *mut freenect_context,
    mut dev: *mut *mut freenect_device,
    mut camera_serial: *const libc::c_char,
) -> libc::c_int {
    *dev = fake_dev;
    return 0 as libc::c_int;
}
unsafe extern "C" fn read_device_info(mut dev: *mut freenect_device) {
    let mut fn_0: [libc::c_char; 512] = [0; 512];
    snprintf(
        fn_0.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong,
        b"%s/device.json\0" as *const u8 as *const libc::c_char,
        input_path,
    );
    let mut js: *mut JSON_Value = json_parse_file(fn_0.as_mut_ptr());
    if js.is_null() {
        return;
    }
    let mut reg_info: *mut JSON_Object = json_object_get_object(
        json_object(js),
        b"reg_info\0" as *const u8 as *const libc::c_char,
    );
    let mut pad_info: *mut JSON_Object = json_object_get_object(
        reg_info,
        b"pad_info\0" as *const u8 as *const libc::c_char,
    );
    let mut zp_info: *mut JSON_Object = json_object_get_object(
        reg_info,
        b"zero_plane_info\0" as *const u8 as *const libc::c_char,
    );
    (*dev)
        .registration
        .reg_info
        .ax = json_object_get_number(
        reg_info,
        b"ax\0" as *const u8 as *const libc::c_char,
    ) as int32_t;
    (*dev)
        .registration
        .reg_info
        .bx = json_object_get_number(
        reg_info,
        b"bx\0" as *const u8 as *const libc::c_char,
    ) as int32_t;
    (*dev)
        .registration
        .reg_info
        .cx = json_object_get_number(
        reg_info,
        b"cx\0" as *const u8 as *const libc::c_char,
    ) as int32_t;
    (*dev)
        .registration
        .reg_info
        .dx = json_object_get_number(
        reg_info,
        b"dx\0" as *const u8 as *const libc::c_char,
    ) as int32_t;
    (*dev)
        .registration
        .reg_info
        .ay = json_object_get_number(
        reg_info,
        b"ay\0" as *const u8 as *const libc::c_char,
    ) as int32_t;
    (*dev)
        .registration
        .reg_info
        .by = json_object_get_number(
        reg_info,
        b"by\0" as *const u8 as *const libc::c_char,
    ) as int32_t;
    (*dev)
        .registration
        .reg_info
        .cy = json_object_get_number(
        reg_info,
        b"cy\0" as *const u8 as *const libc::c_char,
    ) as int32_t;
    (*dev)
        .registration
        .reg_info
        .dy = json_object_get_number(
        reg_info,
        b"dy\0" as *const u8 as *const libc::c_char,
    ) as int32_t;
    (*dev)
        .registration
        .reg_info
        .dx_start = json_object_get_number(
        reg_info,
        b"dx_start\0" as *const u8 as *const libc::c_char,
    ) as int32_t;
    (*dev)
        .registration
        .reg_info
        .dy_start = json_object_get_number(
        reg_info,
        b"dy_start\0" as *const u8 as *const libc::c_char,
    ) as int32_t;
    (*dev)
        .registration
        .reg_info
        .dx_beta_start = json_object_get_number(
        reg_info,
        b"dx_beta_start\0" as *const u8 as *const libc::c_char,
    ) as int32_t;
    (*dev)
        .registration
        .reg_info
        .dy_beta_start = json_object_get_number(
        reg_info,
        b"dy_beta_start\0" as *const u8 as *const libc::c_char,
    ) as int32_t;
    (*dev)
        .registration
        .reg_info
        .dx_beta_inc = json_object_get_number(
        reg_info,
        b"dx_beta_inc\0" as *const u8 as *const libc::c_char,
    ) as int32_t;
    (*dev)
        .registration
        .reg_info
        .dy_beta_inc = json_object_get_number(
        reg_info,
        b"dy_beta_inc\0" as *const u8 as *const libc::c_char,
    ) as int32_t;
    (*dev)
        .registration
        .reg_info
        .dxdx_start = json_object_get_number(
        reg_info,
        b"dxdx_start\0" as *const u8 as *const libc::c_char,
    ) as int32_t;
    (*dev)
        .registration
        .reg_info
        .dxdy_start = json_object_get_number(
        reg_info,
        b"dxdy_start\0" as *const u8 as *const libc::c_char,
    ) as int32_t;
    (*dev)
        .registration
        .reg_info
        .dydx_start = json_object_get_number(
        reg_info,
        b"dydx_start\0" as *const u8 as *const libc::c_char,
    ) as int32_t;
    (*dev)
        .registration
        .reg_info
        .dydy_start = json_object_get_number(
        reg_info,
        b"dydy_start\0" as *const u8 as *const libc::c_char,
    ) as int32_t;
    (*dev)
        .registration
        .reg_info
        .dxdxdx_start = json_object_get_number(
        reg_info,
        b"dxdxdx_start\0" as *const u8 as *const libc::c_char,
    ) as int32_t;
    (*dev)
        .registration
        .reg_info
        .dydxdx_start = json_object_get_number(
        reg_info,
        b"dydxdx_start\0" as *const u8 as *const libc::c_char,
    ) as int32_t;
    (*dev)
        .registration
        .reg_info
        .dxdxdy_start = json_object_get_number(
        reg_info,
        b"dxdxdy_start\0" as *const u8 as *const libc::c_char,
    ) as int32_t;
    (*dev)
        .registration
        .reg_info
        .dydxdy_start = json_object_get_number(
        reg_info,
        b"dydxdy_start\0" as *const u8 as *const libc::c_char,
    ) as int32_t;
    (*dev)
        .registration
        .reg_info
        .dydydx_start = json_object_get_number(
        reg_info,
        b"dydydx_start\0" as *const u8 as *const libc::c_char,
    ) as int32_t;
    (*dev)
        .registration
        .reg_info
        .dydydy_start = json_object_get_number(
        reg_info,
        b"dydydy_start\0" as *const u8 as *const libc::c_char,
    ) as int32_t;
    (*dev)
        .registration
        .reg_pad_info
        .start_lines = json_object_get_number(
        pad_info,
        b"start_lines\0" as *const u8 as *const libc::c_char,
    ) as uint16_t;
    (*dev)
        .registration
        .reg_pad_info
        .end_lines = json_object_get_number(
        pad_info,
        b"end_lines\0" as *const u8 as *const libc::c_char,
    ) as uint16_t;
    (*dev)
        .registration
        .reg_pad_info
        .cropping_lines = json_object_get_number(
        pad_info,
        b"cropping_lines\0" as *const u8 as *const libc::c_char,
    ) as uint16_t;
    (*dev)
        .registration
        .zero_plane_info
        .dcmos_emitter_dist = json_object_get_number(
        zp_info,
        b"dcmos_emitter_distance\0" as *const u8 as *const libc::c_char,
    ) as libc::c_float;
    (*dev)
        .registration
        .zero_plane_info
        .dcmos_rcmos_dist = json_object_get_number(
        zp_info,
        b"dcmos_rcmos_distance\0" as *const u8 as *const libc::c_char,
    ) as libc::c_float;
    (*dev)
        .registration
        .zero_plane_info
        .reference_distance = json_object_get_number(
        zp_info,
        b"reference_distance\0" as *const u8 as *const libc::c_char,
    ) as libc::c_float;
    (*dev)
        .registration
        .zero_plane_info
        .reference_pixel_size = json_object_get_number(
        zp_info,
        b"reference_pixel_size\0" as *const u8 as *const libc::c_char,
    ) as libc::c_float;
    (*dev)
        .registration
        .const_shift = json_object_get_number(
        json_object(js),
        b"const_shift\0" as *const u8 as *const libc::c_char,
    );
    json_value_free(js);
    freenect_init_registration(fake_dev);
}
pub unsafe extern "C" fn freenect_init(
    mut ctx: *mut *mut freenect_context,
    mut usb_ctx: *mut libc::c_void,
) -> libc::c_int {
    input_path = getenv(b"FAKENECT_PATH\0" as *const u8 as *const libc::c_char);
    if input_path.is_null() {
        printf(
            b"Error: Environmental variable FAKENECT_PATH is not set.  Set it to a path that was created using the 'record' utility.\n\0"
                as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    let mut var: *mut libc::c_char = getenv(
        b"FAKENECT_LOOP\0" as *const u8 as *const libc::c_char,
    );
    if !var.is_null() {
        let len: libc::c_int = strlen(var) as libc::c_int;
        let mut tmp: *mut libc::c_char = malloc(
            ((len + 1 as libc::c_int) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
        ) as *mut libc::c_char;
        let mut i: libc::c_int = 0;
        i = 0 as libc::c_int;
        while i < len {
            *tmp
                .offset(
                    i as isize,
                ) = ({
                let mut __res: libc::c_int = 0;
                if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                    > 1 as libc::c_int as libc::c_ulong
                {
                    if 0 != 0 {
                        let mut __c: libc::c_int = *var.offset(i as isize)
                            as libc::c_int;
                        __res = if __c < -(128 as libc::c_int)
                            || __c > 255 as libc::c_int
                        {
                            __c
                        } else {
                            *(*__ctype_tolower_loc()).offset(__c as isize)
                        };
                    } else {
                        __res = tolower(*var.offset(i as isize) as libc::c_int);
                    }
                } else {
                    __res = *(*__ctype_tolower_loc())
                        .offset(*var.offset(i as isize) as libc::c_int as isize);
                }
                __res
            }) as libc::c_char;
            i += 1;
            i;
        }
        *tmp.offset(len as isize) = '\0' as i32 as libc::c_char;
        if strcmp(tmp, b"0\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
            || strcmp(tmp, b"false\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            || strcmp(tmp, b"no\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            || strcmp(tmp, b"off\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
        {
            loop_playback = 0 as libc::c_int != 0;
        }
        free(tmp as *mut libc::c_void);
    }
    *ctx = fake_ctx;
    read_device_info(fake_dev);
    video_mode = rgb_video_mode;
    depth_mode = depth_11_mode;
    default_video_back = malloc(
        (640 as libc::c_int * 480 as libc::c_int * 3 as libc::c_int) as libc::c_ulong,
    );
    default_depth_back = malloc(
        (640 as libc::c_int * 480 as libc::c_int * 2 as libc::c_int) as libc::c_ulong,
    );
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn freenect_supported_subdevices() -> libc::c_int {
    return FREENECT_DEVICE_MOTOR as libc::c_int | FREENECT_DEVICE_CAMERA as libc::c_int;
}
pub unsafe extern "C" fn freenect_select_subdevices(
    mut ctx: *mut freenect_context,
    mut subdevs: freenect_device_flags,
) {}
pub unsafe extern "C" fn freenect_set_depth_buffer(
    mut dev: *mut freenect_device,
    mut buf: *mut libc::c_void,
) -> libc::c_int {
    user_depth_buf = buf;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn freenect_set_video_buffer(
    mut dev: *mut freenect_device,
    mut buf: *mut libc::c_void,
) -> libc::c_int {
    user_video_buf = buf;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn freenect_set_user(
    mut dev: *mut freenect_device,
    mut user: *mut libc::c_void,
) {
    user_ptr = user;
}
pub unsafe extern "C" fn freenect_get_user(
    mut dev: *mut freenect_device,
) -> *mut libc::c_void {
    return user_ptr;
}
pub unsafe extern "C" fn freenect_start_depth(
    mut dev: *mut freenect_device,
) -> libc::c_int {
    depth_running = 1 as libc::c_int;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn freenect_start_video(
    mut dev: *mut freenect_device,
) -> libc::c_int {
    rgb_running = 1 as libc::c_int;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn freenect_stop_depth(
    mut dev: *mut freenect_device,
) -> libc::c_int {
    depth_running = 0 as libc::c_int;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn freenect_stop_video(
    mut dev: *mut freenect_device,
) -> libc::c_int {
    rgb_running = 0 as libc::c_int;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn freenect_set_video_format(
    mut dev: *mut freenect_device,
    mut fmt: freenect_video_format,
) -> libc::c_int {
    if fmt as libc::c_uint == FREENECT_VIDEO_RGB as libc::c_int as libc::c_uint
        || fmt as libc::c_uint == FREENECT_VIDEO_YUV_RAW as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"fmt == FREENECT_VIDEO_RGB || fmt == FREENECT_VIDEO_YUV_RAW\0" as *const u8
                as *const libc::c_char,
            b"../../fakenect/fakenect.c\0" as *const u8 as *const libc::c_char,
            628 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 72],
                &[libc::c_char; 72],
            >(
                b"int freenect_set_video_format(freenect_device *, freenect_video_format)\0",
            ))
                .as_ptr(),
        );
    }
    'c_10196: {
        if fmt as libc::c_uint == FREENECT_VIDEO_RGB as libc::c_int as libc::c_uint
            || fmt as libc::c_uint
                == FREENECT_VIDEO_YUV_RAW as libc::c_int as libc::c_uint
        {} else {
            __assert_fail(
                b"fmt == FREENECT_VIDEO_RGB || fmt == FREENECT_VIDEO_YUV_RAW\0"
                    as *const u8 as *const libc::c_char,
                b"../../fakenect/fakenect.c\0" as *const u8 as *const libc::c_char,
                628 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 72],
                    &[libc::c_char; 72],
                >(
                    b"int freenect_set_video_format(freenect_device *, freenect_video_format)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn freenect_set_depth_format(
    mut dev: *mut freenect_device,
    mut fmt: freenect_depth_format,
) -> libc::c_int {
    if fmt as libc::c_uint == FREENECT_DEPTH_11BIT as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"fmt == FREENECT_DEPTH_11BIT\0" as *const u8 as *const libc::c_char,
            b"../../fakenect/fakenect.c\0" as *const u8 as *const libc::c_char,
            633 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 72],
                &[libc::c_char; 72],
            >(
                b"int freenect_set_depth_format(freenect_device *, freenect_depth_format)\0",
            ))
                .as_ptr(),
        );
    }
    'c_10260: {
        if fmt as libc::c_uint == FREENECT_DEPTH_11BIT as libc::c_int as libc::c_uint
        {} else {
            __assert_fail(
                b"fmt == FREENECT_DEPTH_11BIT\0" as *const u8 as *const libc::c_char,
                b"../../fakenect/fakenect.c\0" as *const u8 as *const libc::c_char,
                633 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 72],
                    &[libc::c_char; 72],
                >(
                    b"int freenect_set_depth_format(freenect_device *, freenect_depth_format)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn freenect_set_log_callback(
    mut ctx: *mut freenect_context,
    mut cb: freenect_log_cb,
) {}
pub unsafe extern "C" fn freenect_set_log_level(
    mut ctx: *mut freenect_context,
    mut level: freenect_loglevel,
) {}
pub unsafe extern "C" fn freenect_shutdown(
    mut ctx: *mut freenect_context,
) -> libc::c_int {
    free(default_video_back);
    free(default_depth_back);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn freenect_close_device(
    mut dev: *mut freenect_device,
) -> libc::c_int {
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn freenect_set_tilt_degs(
    mut dev: *mut freenect_device,
    mut angle: libc::c_double,
) -> libc::c_int {
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn freenect_set_led(
    mut dev: *mut freenect_device,
    mut option: freenect_led_options,
) -> libc::c_int {
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn freenect_update_tilt_state(
    mut dev: *mut freenect_device,
) -> libc::c_int {
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn freenect_get_ir_brightness(
    mut dev: *mut freenect_device,
) -> libc::c_int {
    return ir_brightness as libc::c_int;
}
pub unsafe extern "C" fn freenect_set_ir_brightness(
    mut dev: *mut freenect_device,
    mut brightness: uint16_t,
) -> libc::c_int {
    ir_brightness = (brightness as libc::c_int % 50 as libc::c_int) as uint16_t;
    return 0 as libc::c_int;
}
