use ::libc;
extern "C" {
    pub type libusb_device_handle;
    pub type libusb_context;
    pub type json_object_t;
    pub type json_value_t;
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn freenect_set_video_callback(dev: *mut freenect_device, cb: freenect_video_cb);
    fn gettimeofday(__tv: *mut timeval, __tz: *mut libc::c_void) -> libc::c_int;
    fn freenect_set_depth_callback(dev: *mut freenect_device, cb: freenect_depth_cb);
    fn freenect_start_depth(dev: *mut freenect_device) -> libc::c_int;
    fn freenect_start_video(dev: *mut freenect_device) -> libc::c_int;
    fn freenect_stop_depth(dev: *mut freenect_device) -> libc::c_int;
    fn freenect_stop_video(dev: *mut freenect_device) -> libc::c_int;
    fn freenect_update_tilt_state(dev: *mut freenect_device) -> libc::c_int;
    fn freenect_get_tilt_state(
        dev: *mut freenect_device,
    ) -> *mut freenect_raw_tilt_state;
    fn freenect_get_current_video_mode(dev: *mut freenect_device) -> freenect_frame_mode;
    fn freenect_find_video_mode(
        res: freenect_resolution,
        fmt: freenect_video_format,
    ) -> freenect_frame_mode;
    fn freenect_set_video_mode(
        dev: *mut freenect_device,
        mode: freenect_frame_mode,
    ) -> libc::c_int;
    fn freenect_get_current_depth_mode(dev: *mut freenect_device) -> freenect_frame_mode;
    fn freenect_find_depth_mode(
        res: freenect_resolution,
        fmt: freenect_depth_format,
    ) -> freenect_frame_mode;
    fn freenect_set_depth_mode(
        dev: *mut freenect_device,
        mode: freenect_frame_mode,
    ) -> libc::c_int;
    fn freenect_init(
        ctx: *mut *mut freenect_context,
        usb_ctx: *mut libc::c_void,
    ) -> libc::c_int;
    fn freenect_shutdown(ctx: *mut freenect_context) -> libc::c_int;
    fn freenect_process_events(ctx: *mut freenect_context) -> libc::c_int;
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
    fn floor(_: libc::c_double) -> libc::c_double;
    fn sleep(__seconds: libc::c_uint) -> libc::c_uint;
    fn usleep(__useconds: __useconds_t) -> libc::c_int;
    fn json_serialize_to_file_pretty(
        value: *const JSON_Value,
        filename: *const libc::c_char,
    ) -> JSON_Status;
    fn json_object_set_value(
        object: *mut JSON_Object,
        name: *const libc::c_char,
        value: *mut JSON_Value,
    ) -> JSON_Status;
    fn json_object_set_number(
        object: *mut JSON_Object,
        name: *const libc::c_char,
        number: libc::c_double,
    ) -> JSON_Status;
    fn json_value_init_object() -> *mut JSON_Value;
    fn json_value_free(value: *mut JSON_Value);
    fn json_object(value: *const JSON_Value) -> *mut JSON_Object;
    fn mkdir(__path: *const libc::c_char, __mode: __mode_t) -> libc::c_int;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn popen(__command: *const libc::c_char, __modes: *const libc::c_char) -> *mut FILE;
    fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
}
pub type __int8_t = libc::c_schar;
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __mode_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __useconds_t = libc::c_uint;
pub type __suseconds_t = libc::c_long;
pub type __sig_atomic_t = libc::c_int;
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
pub type size_t = libc::c_ulong;
pub type JSON_Object = json_object_t;
pub type JSON_Value = json_value_t;
pub type JSON_Status = libc::c_int;
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
pub type sig_atomic_t = __sig_atomic_t;
pub type __sighandler_t = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
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
pub static mut out_dir: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut running: sig_atomic_t = 1 as libc::c_int;
pub static mut last_timestamp: uint32_t = 0 as libc::c_int as uint32_t;
pub static mut index_fp: *mut FILE = 0 as *const FILE as *mut FILE;
pub static mut use_ffmpeg: libc::c_int = 0 as libc::c_int;
pub static mut ffmpeg_opts: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut depth_name: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut rgb_name: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut depth_stream: *mut FILE = 0 as *const FILE as *mut FILE;
pub static mut rgb_stream: *mut FILE = 0 as *const FILE as *mut FILE;
pub unsafe extern "C" fn dump_depth(
    mut fp: *mut FILE,
    mut data: *mut libc::c_void,
    mut data_size: libc::c_int,
) {
    fprintf(
        fp,
        b"P5 %d %d 65535\n\0" as *const u8 as *const libc::c_char,
        640 as libc::c_int,
        480 as libc::c_int,
    );
    fwrite(data, data_size as libc::c_ulong, 1 as libc::c_int as libc::c_ulong, fp);
}
pub unsafe extern "C" fn dump_rgb(
    mut fp: *mut FILE,
    mut data: *mut libc::c_void,
    mut data_size: libc::c_int,
) {
    fprintf(
        fp,
        b"P6 %d %d 255\n\0" as *const u8 as *const libc::c_char,
        640 as libc::c_int,
        480 as libc::c_int,
    );
    fwrite(data, data_size as libc::c_ulong, 1 as libc::c_int as libc::c_ulong, fp);
}
pub unsafe extern "C" fn open_dump(
    mut type_0: libc::c_char,
    mut cur_time: libc::c_double,
    mut timestamp: uint32_t,
    mut data_size: libc::c_int,
    mut extension: *const libc::c_char,
) -> *mut FILE {
    let mut fn_0: *mut libc::c_char = malloc(
        (strlen(out_dir)).wrapping_add(50 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    sprintf(
        fn_0,
        b"%c-%f-%u.%s\0" as *const u8 as *const libc::c_char,
        type_0 as libc::c_int,
        cur_time,
        timestamp,
        extension,
    );
    fprintf(index_fp, b"%s\n\0" as *const u8 as *const libc::c_char, fn_0);
    sprintf(
        fn_0,
        b"%s/%c-%f-%u.%s\0" as *const u8 as *const libc::c_char,
        out_dir,
        type_0 as libc::c_int,
        cur_time,
        timestamp,
        extension,
    );
    let mut fp: *mut FILE = fopen(fn_0, b"wb\0" as *const u8 as *const libc::c_char);
    if fp.is_null() {
        printf(
            b"Error: Cannot open file [%s]\n\0" as *const u8 as *const libc::c_char,
            fn_0,
        );
        exit(1 as libc::c_int);
    }
    printf(b"%s\n\0" as *const u8 as *const libc::c_char, fn_0);
    free(fn_0 as *mut libc::c_void);
    return fp;
}
pub unsafe extern "C" fn open_ffmpeg(
    mut output_filename: *mut libc::c_char,
) -> *mut FILE {
    let mut cmd: [libc::c_char; 1024] = [0; 1024];
    if ffmpeg_opts.is_null() {
        ffmpeg_opts = b"-aspect 4:3 -r 20 -vcodec msmpeg4 -b 30000k\0" as *const u8
            as *const libc::c_char as *mut libc::c_char;
    }
    snprintf(
        cmd.as_mut_ptr(),
        1024 as libc::c_int as libc::c_ulong,
        b"ffmpeg -pix_fmt rgb24 -s %dx%d -f rawvideo -i /dev/stdin %s %s\0" as *const u8
            as *const libc::c_char,
        640 as libc::c_int,
        480 as libc::c_int,
        ffmpeg_opts,
        output_filename,
    );
    fprintf(stderr, b"%s\n\0" as *const u8 as *const libc::c_char, cmd.as_mut_ptr());
    let mut proc_0: *mut FILE = popen(
        cmd.as_mut_ptr(),
        b"w\0" as *const u8 as *const libc::c_char,
    );
    if proc_0.is_null() {
        printf(b"Error: Cannot run ffmpeg\n\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    return proc_0;
}
pub unsafe extern "C" fn dump(
    mut type_0: libc::c_char,
    mut timestamp: uint32_t,
    mut data: *mut libc::c_void,
    mut data_size: libc::c_int,
) {
    let mut cur_time: libc::c_double = get_time();
    let mut fp: *mut FILE = 0 as *mut FILE;
    last_timestamp = timestamp;
    match type_0 as libc::c_int {
        100 => {
            fp = open_dump(
                type_0,
                cur_time,
                timestamp,
                data_size,
                b"pgm\0" as *const u8 as *const libc::c_char,
            );
            dump_depth(fp, data, data_size);
            fclose(fp);
        }
        114 => {
            fp = open_dump(
                type_0,
                cur_time,
                timestamp,
                data_size,
                b"ppm\0" as *const u8 as *const libc::c_char,
            );
            dump_rgb(fp, data, data_size);
            fclose(fp);
        }
        97 => {
            fp = open_dump(
                type_0,
                cur_time,
                timestamp,
                data_size,
                b"dump\0" as *const u8 as *const libc::c_char,
            );
            fwrite(
                data,
                data_size as libc::c_ulong,
                1 as libc::c_int as libc::c_ulong,
                fp,
            );
            fclose(fp);
        }
        _ => {}
    };
}
pub unsafe extern "C" fn dump_ffmpeg_24(
    mut stream: *mut FILE,
    mut timestamp: uint32_t,
    mut data: *mut libc::c_void,
    mut data_size: libc::c_int,
) {
    fwrite(data, data_size as libc::c_ulong, 1 as libc::c_int as libc::c_ulong, stream);
}
pub unsafe extern "C" fn dump_ffmpeg_pad16(
    mut stream: *mut FILE,
    mut timestamp: uint32_t,
    mut data: *mut libc::c_void,
    mut data_size: libc::c_int,
) {
    let mut z: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut data_ptr: *mut uint16_t = data as *mut uint16_t;
    let mut end: *mut uint16_t = data_ptr.offset(data_size as isize);
    while data_ptr < end {
        z = *data_ptr as libc::c_uint;
        fwrite(
            &mut z as *mut libc::c_uint as *mut libc::c_char as *const libc::c_void,
            3 as libc::c_int as libc::c_ulong,
            1 as libc::c_int as libc::c_ulong,
            stream,
        );
        data_ptr = data_ptr.offset(2 as libc::c_int as isize);
    }
}
pub unsafe extern "C" fn snapshot_accel(mut dev: *mut freenect_device) {
    let mut state: *mut freenect_raw_tilt_state = 0 as *mut freenect_raw_tilt_state;
    if last_timestamp == 0 {
        return;
    }
    freenect_update_tilt_state(dev);
    state = freenect_get_tilt_state(dev);
    dump(
        'a' as i32 as libc::c_char,
        last_timestamp,
        state as *mut libc::c_void,
        ::std::mem::size_of::<freenect_raw_tilt_state>() as libc::c_ulong as libc::c_int,
    );
}
pub unsafe extern "C" fn depth_cb(
    mut dev: *mut freenect_device,
    mut depth: *mut libc::c_void,
    mut timestamp: uint32_t,
) {
    dump(
        'd' as i32 as libc::c_char,
        timestamp,
        depth,
        (freenect_get_current_depth_mode(dev)).bytes,
    );
}
pub unsafe extern "C" fn rgb_cb(
    mut dev: *mut freenect_device,
    mut rgb: *mut libc::c_void,
    mut timestamp: uint32_t,
) {
    dump(
        'r' as i32 as libc::c_char,
        timestamp,
        rgb,
        (freenect_get_current_video_mode(dev)).bytes,
    );
}
pub unsafe extern "C" fn depth_cb_ffmpeg(
    mut dev: *mut freenect_device,
    mut depth: *mut libc::c_void,
    mut timestamp: uint32_t,
) {
    let mut cur_time: libc::c_double = get_time();
    fprintf(
        index_fp,
        b"d-%f-%u\n\0" as *const u8 as *const libc::c_char,
        cur_time,
        timestamp,
    );
    dump_ffmpeg_pad16(
        depth_stream,
        timestamp,
        depth,
        (freenect_find_depth_mode(FREENECT_RESOLUTION_MEDIUM, FREENECT_DEPTH_11BIT))
            .bytes,
    );
}
pub unsafe extern "C" fn rgb_cb_ffmpeg(
    mut dev: *mut freenect_device,
    mut rgb: *mut libc::c_void,
    mut timestamp: uint32_t,
) {
    let mut cur_time: libc::c_double = get_time();
    fprintf(
        index_fp,
        b"d-%f-%u\n\0" as *const u8 as *const libc::c_char,
        cur_time,
        timestamp,
    );
    dump_ffmpeg_24(
        rgb_stream,
        timestamp,
        rgb,
        (freenect_get_current_video_mode(dev)).bytes,
    );
}
pub unsafe extern "C" fn init_ffmpeg_streams() {
    depth_stream = open_ffmpeg(depth_name);
    rgb_stream = open_ffmpeg(rgb_name);
}
pub unsafe extern "C" fn print_mode(
    mut name: *const libc::c_char,
    mut mode: freenect_frame_mode,
) {
    printf(
        b"%s Mode: {%d, %d, {%d}, %d, %d, %d, %d, %d, %d, %d}\n\0" as *const u8
            as *const libc::c_char,
        name,
        mode.reserved,
        mode.resolution as libc::c_int,
        mode.c2rust_unnamed.video_format as libc::c_int,
        mode.bytes,
        mode.width as libc::c_int,
        mode.height as libc::c_int,
        mode.data_bits_per_pixel as libc::c_int,
        mode.padding_bits_per_pixel as libc::c_int,
        mode.framerate as libc::c_int,
        mode.is_valid as libc::c_int,
    );
}
unsafe extern "C" fn write_device_info(mut dev: *mut freenect_device) {
    let mut js: *mut JSON_Value = json_value_init_object();
    let mut dev_js: *mut JSON_Object = json_object(js);
    let mut reg_info_val: *mut JSON_Value = json_value_init_object();
    let mut reg_info: *mut JSON_Object = json_object(reg_info_val);
    json_object_set_value(
        dev_js,
        b"reg_info\0" as *const u8 as *const libc::c_char,
        reg_info_val,
    );
    json_object_set_number(
        reg_info,
        b"ax\0" as *const u8 as *const libc::c_char,
        (*dev).registration.reg_info.ax as libc::c_double,
    );
    json_object_set_number(
        reg_info,
        b"bx\0" as *const u8 as *const libc::c_char,
        (*dev).registration.reg_info.bx as libc::c_double,
    );
    json_object_set_number(
        reg_info,
        b"cx\0" as *const u8 as *const libc::c_char,
        (*dev).registration.reg_info.cx as libc::c_double,
    );
    json_object_set_number(
        reg_info,
        b"dx\0" as *const u8 as *const libc::c_char,
        (*dev).registration.reg_info.dx as libc::c_double,
    );
    json_object_set_number(
        reg_info,
        b"ay\0" as *const u8 as *const libc::c_char,
        (*dev).registration.reg_info.ay as libc::c_double,
    );
    json_object_set_number(
        reg_info,
        b"by\0" as *const u8 as *const libc::c_char,
        (*dev).registration.reg_info.by as libc::c_double,
    );
    json_object_set_number(
        reg_info,
        b"cy\0" as *const u8 as *const libc::c_char,
        (*dev).registration.reg_info.cy as libc::c_double,
    );
    json_object_set_number(
        reg_info,
        b"dy\0" as *const u8 as *const libc::c_char,
        (*dev).registration.reg_info.dy as libc::c_double,
    );
    json_object_set_number(
        reg_info,
        b"dx_start\0" as *const u8 as *const libc::c_char,
        (*dev).registration.reg_info.dx_start as libc::c_double,
    );
    json_object_set_number(
        reg_info,
        b"dy_start\0" as *const u8 as *const libc::c_char,
        (*dev).registration.reg_info.dy_start as libc::c_double,
    );
    json_object_set_number(
        reg_info,
        b"dx_beta_start\0" as *const u8 as *const libc::c_char,
        (*dev).registration.reg_info.dx_beta_start as libc::c_double,
    );
    json_object_set_number(
        reg_info,
        b"dy_beta_start\0" as *const u8 as *const libc::c_char,
        (*dev).registration.reg_info.dy_beta_start as libc::c_double,
    );
    json_object_set_number(
        reg_info,
        b"dx_beta_inc\0" as *const u8 as *const libc::c_char,
        (*dev).registration.reg_info.dx_beta_inc as libc::c_double,
    );
    json_object_set_number(
        reg_info,
        b"dy_beta_inc\0" as *const u8 as *const libc::c_char,
        (*dev).registration.reg_info.dy_beta_inc as libc::c_double,
    );
    json_object_set_number(
        reg_info,
        b"dxdx_start\0" as *const u8 as *const libc::c_char,
        (*dev).registration.reg_info.dxdx_start as libc::c_double,
    );
    json_object_set_number(
        reg_info,
        b"dxdy_start\0" as *const u8 as *const libc::c_char,
        (*dev).registration.reg_info.dxdy_start as libc::c_double,
    );
    json_object_set_number(
        reg_info,
        b"dydx_start\0" as *const u8 as *const libc::c_char,
        (*dev).registration.reg_info.dydx_start as libc::c_double,
    );
    json_object_set_number(
        reg_info,
        b"dydy_start\0" as *const u8 as *const libc::c_char,
        (*dev).registration.reg_info.dydy_start as libc::c_double,
    );
    json_object_set_number(
        reg_info,
        b"dxdxdx_start\0" as *const u8 as *const libc::c_char,
        (*dev).registration.reg_info.dxdxdx_start as libc::c_double,
    );
    json_object_set_number(
        reg_info,
        b"dydxdx_start\0" as *const u8 as *const libc::c_char,
        (*dev).registration.reg_info.dydxdx_start as libc::c_double,
    );
    json_object_set_number(
        reg_info,
        b"dxdxdy_start\0" as *const u8 as *const libc::c_char,
        (*dev).registration.reg_info.dxdxdy_start as libc::c_double,
    );
    json_object_set_number(
        reg_info,
        b"dydxdy_start\0" as *const u8 as *const libc::c_char,
        (*dev).registration.reg_info.dydxdy_start as libc::c_double,
    );
    json_object_set_number(
        reg_info,
        b"dydydx_start\0" as *const u8 as *const libc::c_char,
        (*dev).registration.reg_info.dydydx_start as libc::c_double,
    );
    json_object_set_number(
        reg_info,
        b"dydydy_start\0" as *const u8 as *const libc::c_char,
        (*dev).registration.reg_info.dydydy_start as libc::c_double,
    );
    let mut pad_info_val: *mut JSON_Value = json_value_init_object();
    let mut pad_info: *mut JSON_Object = json_object(pad_info_val);
    json_object_set_value(
        reg_info,
        b"pad_info\0" as *const u8 as *const libc::c_char,
        pad_info_val,
    );
    json_object_set_number(
        pad_info,
        b"start_lines\0" as *const u8 as *const libc::c_char,
        (*dev).registration.reg_pad_info.start_lines as libc::c_double,
    );
    json_object_set_number(
        pad_info,
        b"end_lines\0" as *const u8 as *const libc::c_char,
        (*dev).registration.reg_pad_info.end_lines as libc::c_double,
    );
    json_object_set_number(
        pad_info,
        b"cropping_lines\0" as *const u8 as *const libc::c_char,
        (*dev).registration.reg_pad_info.cropping_lines as libc::c_double,
    );
    let mut zp_info_val: *mut JSON_Value = json_value_init_object();
    let mut zp_info: *mut JSON_Object = json_object(zp_info_val);
    json_object_set_value(
        reg_info,
        b"zero_plane_info\0" as *const u8 as *const libc::c_char,
        zp_info_val,
    );
    json_object_set_number(
        zp_info,
        b"dcmos_emitter_distance\0" as *const u8 as *const libc::c_char,
        (*dev).registration.zero_plane_info.dcmos_emitter_dist as libc::c_double,
    );
    json_object_set_number(
        zp_info,
        b"dcmos_rcmos_distance\0" as *const u8 as *const libc::c_char,
        (*dev).registration.zero_plane_info.dcmos_rcmos_dist as libc::c_double,
    );
    json_object_set_number(
        zp_info,
        b"reference_distance\0" as *const u8 as *const libc::c_char,
        (*dev).registration.zero_plane_info.reference_distance as libc::c_double,
    );
    json_object_set_number(
        zp_info,
        b"reference_pixel_size\0" as *const u8 as *const libc::c_char,
        (*dev).registration.zero_plane_info.reference_pixel_size as libc::c_double,
    );
    json_object_set_number(
        dev_js,
        b"const_shift\0" as *const u8 as *const libc::c_char,
        (*dev).registration.const_shift,
    );
    let mut fn_0: [libc::c_char; 512] = [0; 512];
    snprintf(
        fn_0.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong,
        b"%s/device.json\0" as *const u8 as *const libc::c_char,
        out_dir,
    );
    json_serialize_to_file_pretty(js, fn_0.as_mut_ptr());
    json_value_free(js);
}
pub unsafe extern "C" fn init() {
    let mut ctx: *mut freenect_context = 0 as *mut freenect_context;
    let mut dev: *mut freenect_device = 0 as *mut freenect_device;
    if freenect_init(&mut ctx, 0 as *mut libc::c_void) != 0 {
        printf(b"Error: Cannot get context\n\0" as *const u8 as *const libc::c_char);
        return;
    }
    freenect_select_subdevices(
        ctx,
        (FREENECT_DEVICE_MOTOR as libc::c_int | FREENECT_DEVICE_CAMERA as libc::c_int)
            as freenect_device_flags,
    );
    if freenect_open_device(ctx, &mut dev, 0 as libc::c_int) != 0 {
        printf(b"Error: Cannot get device\n\0" as *const u8 as *const libc::c_char);
        return;
    }
    print_mode(
        b"Depth\0" as *const u8 as *const libc::c_char,
        freenect_find_depth_mode(FREENECT_RESOLUTION_MEDIUM, FREENECT_DEPTH_11BIT),
    );
    print_mode(
        b"Video\0" as *const u8 as *const libc::c_char,
        freenect_find_video_mode(FREENECT_RESOLUTION_MEDIUM, FREENECT_VIDEO_RGB),
    );
    freenect_set_depth_mode(
        dev,
        freenect_find_depth_mode(FREENECT_RESOLUTION_MEDIUM, FREENECT_DEPTH_11BIT),
    );
    freenect_start_depth(dev);
    freenect_set_video_mode(
        dev,
        freenect_find_video_mode(FREENECT_RESOLUTION_MEDIUM, FREENECT_VIDEO_RGB),
    );
    freenect_start_video(dev);
    write_device_info(dev);
    if use_ffmpeg != 0 {
        init_ffmpeg_streams();
        freenect_set_depth_callback(
            dev,
            Some(
                depth_cb_ffmpeg
                    as unsafe extern "C" fn(
                        *mut freenect_device,
                        *mut libc::c_void,
                        uint32_t,
                    ) -> (),
            ),
        );
        freenect_set_video_callback(
            dev,
            Some(
                rgb_cb_ffmpeg
                    as unsafe extern "C" fn(
                        *mut freenect_device,
                        *mut libc::c_void,
                        uint32_t,
                    ) -> (),
            ),
        );
    } else {
        freenect_set_depth_callback(
            dev,
            Some(
                depth_cb
                    as unsafe extern "C" fn(
                        *mut freenect_device,
                        *mut libc::c_void,
                        uint32_t,
                    ) -> (),
            ),
        );
        freenect_set_video_callback(
            dev,
            Some(
                rgb_cb
                    as unsafe extern "C" fn(
                        *mut freenect_device,
                        *mut libc::c_void,
                        uint32_t,
                    ) -> (),
            ),
        );
    }
    while running != 0 && freenect_process_events(ctx) >= 0 as libc::c_int {
        snapshot_accel(dev);
    }
    freenect_stop_depth(dev);
    freenect_stop_video(dev);
    freenect_close_device(dev);
    freenect_shutdown(ctx);
}
pub unsafe extern "C" fn open_index(mut fn_0: *const libc::c_char) -> *mut FILE {
    let mut fp: *mut FILE = fopen(fn_0, b"r\0" as *const u8 as *const libc::c_char);
    if !fp.is_null() {
        fclose(fp);
        printf(
            b"Error: Index already exists, to avoid overwriting use a different directory.\n\0"
                as *const u8 as *const libc::c_char,
        );
        return 0 as *mut FILE;
    }
    fp = fopen(fn_0, b"wb\0" as *const u8 as *const libc::c_char);
    if fp.is_null() {
        printf(
            b"Error: Cannot open file [%s]\n\0" as *const u8 as *const libc::c_char,
            fn_0,
        );
        return 0 as *mut FILE;
    }
    return fp;
}
pub unsafe extern "C" fn signal_cleanup(mut num: libc::c_int) {
    ::std::ptr::write_volatile(&mut running as *mut sig_atomic_t, 0 as libc::c_int);
    printf(b"Caught signal, cleaning up\n\0" as *const u8 as *const libc::c_char);
    signal(
        2 as libc::c_int,
        Some(signal_cleanup as unsafe extern "C" fn(libc::c_int) -> ()),
    );
}
pub unsafe extern "C" fn usage() {
    printf(
        b"Records the Kinect sensor data to a directory\nResult can be used as input to Fakenect\nUsage:\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"  record [-h] [-ffmpeg] [-ffmpeg-opts <options>] <target basename>\n\0"
            as *const u8 as *const libc::c_char,
    );
    exit(0 as libc::c_int);
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut c: libc::c_int = 1 as libc::c_int;
    while c < argc {
        if strcmp(
            *argv.offset(c as isize),
            b"-ffmpeg\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            use_ffmpeg = 1 as libc::c_int;
        } else if strcmp(
            *argv.offset(c as isize),
            b"-ffmpeg-opts\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            c += 1;
            if c < argc {
                ffmpeg_opts = *argv.offset(c as isize);
            }
        } else if strcmp(
            *argv.offset(c as isize),
            b"-h\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            usage();
        } else {
            out_dir = *argv.offset(c as isize);
        }
        c += 1;
        c;
    }
    if out_dir.is_null() {
        usage();
    }
    signal(
        2 as libc::c_int,
        Some(signal_cleanup as unsafe extern "C" fn(libc::c_int) -> ()),
    );
    if use_ffmpeg != 0 {
        let mut f: *mut FILE = 0 as *mut FILE;
        let mut index_fn: *mut libc::c_char = malloc(
            (strlen(out_dir)).wrapping_add(50 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_char;
        sprintf(
            index_fn,
            b"%s-index.txt\0" as *const u8 as *const libc::c_char,
            out_dir,
        );
        index_fp = open_index(index_fn);
        free(index_fn as *mut libc::c_void);
        if index_fp.is_null() {
            return 1 as libc::c_int;
        }
        depth_name = malloc(
            (strlen(out_dir)).wrapping_add(50 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_char;
        rgb_name = malloc(
            (strlen(out_dir)).wrapping_add(50 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_char;
        sprintf(
            depth_name,
            b"%s-depth.avi\0" as *const u8 as *const libc::c_char,
            out_dir,
        );
        sprintf(rgb_name, b"%s-rgb.avi\0" as *const u8 as *const libc::c_char, out_dir);
        f = fopen(depth_name, b"r\0" as *const u8 as *const libc::c_char);
        if !f.is_null() {
            printf(
                b"Error: %s already exists, to avoid overwriting use a different name.\n\0"
                    as *const u8 as *const libc::c_char,
                depth_name,
            );
            fclose(f);
            exit(1 as libc::c_int);
        }
        f = fopen(rgb_name, b"r\0" as *const u8 as *const libc::c_char);
        if !f.is_null() {
            printf(
                b"Error: %s already exists, to avoid overwriting use a different name.\n\0"
                    as *const u8 as *const libc::c_char,
                depth_name,
            );
            fclose(f);
            exit(1 as libc::c_int);
        }
        init();
        free(depth_name as *mut libc::c_void);
        free(rgb_name as *mut libc::c_void);
        if !depth_stream.is_null() {
            fclose(depth_stream);
        }
        if !rgb_stream.is_null() {
            fclose(rgb_stream);
        }
        fclose(index_fp);
    } else {
        mkdir(
            out_dir,
            (0o400 as libc::c_int | 0o200 as libc::c_int | 0o100 as libc::c_int
                | (0o400 as libc::c_int | 0o200 as libc::c_int | 0o100 as libc::c_int)
                    >> 3 as libc::c_int
                | (0o400 as libc::c_int | 0o200 as libc::c_int | 0o100 as libc::c_int)
                    >> 3 as libc::c_int >> 3 as libc::c_int) as __mode_t,
        );
        let mut fn_0: *mut libc::c_char = malloc(
            (strlen(out_dir)).wrapping_add(50 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_char;
        sprintf(fn_0, b"%s/INDEX.txt\0" as *const u8 as *const libc::c_char, out_dir);
        index_fp = open_index(fn_0);
        free(fn_0 as *mut libc::c_void);
        if index_fp.is_null() {
            fclose(index_fp);
            return 1 as libc::c_int;
        }
        init();
        fclose(index_fp);
    }
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
