use ::libc;
extern "C" {
    pub type libusb_device_handle;
    pub type libusb_context;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn fn_log(
        ctx: *mut freenect_context,
        level: freenect_loglevel,
        fmt: *const libc::c_char,
        _: ...
    );
    fn fnusb_get_max_iso_packet_size(
        dev: *mut fnusb_dev,
        endpoint: libc::c_uchar,
        default_size: libc::c_int,
    ) -> libc::c_int;
    fn fnusb_start_iso(
        dev: *mut fnusb_dev,
        strm: *mut fnusb_isoc_stream,
        cb: fnusb_iso_cb,
        endpoint: libc::c_uchar,
        xfers: libc::c_int,
        pkts: libc::c_int,
        len: libc::c_int,
    ) -> libc::c_int;
    fn freenect_destroy_registration(reg: *mut freenect_registration) -> libc::c_int;
    fn fnusb_stop_iso(dev: *mut fnusb_dev, strm: *mut fnusb_isoc_stream) -> libc::c_int;
    fn freenect_apply_registration(
        dev: *mut freenect_device,
        input: *mut uint8_t,
        output_mm: *mut uint16_t,
        unpacked: bool,
    ) -> libc::c_int;
    fn freenect_init_registration(dev: *mut freenect_device) -> libc::c_int;
    fn freenect_apply_depth_to_mm(
        dev: *mut freenect_device,
        input_packed: *mut uint8_t,
        output_mm: *mut uint16_t,
    ) -> libc::c_int;
    fn send_cmd(
        dev: *mut freenect_device,
        cmd: uint16_t,
        cmdbuf: *mut libc::c_void,
        cmd_len: libc::c_uint,
        replybuf: *mut libc::c_void,
        reply_len: libc::c_int,
    ) -> libc::c_int;
    fn write_register(
        dev: *mut freenect_device,
        reg: uint16_t,
        data: uint16_t,
    ) -> libc::c_int;
}
pub type __int8_t = libc::c_schar;
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type int8_t = __int8_t;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pkt_hdr {
    pub magic: [uint8_t; 2],
    pub pad: uint8_t,
    pub flag: uint8_t,
    pub unk1: uint8_t,
    pub seq: uint8_t,
    pub unk2: uint8_t,
    pub unk3: uint8_t,
    pub timestamp: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub ui: uint32_t,
    pub f: libc::c_float,
}
static mut supported_video_modes: [freenect_frame_mode; 12] = [
    {
        let mut init = freenect_frame_mode {
            reserved: ((FREENECT_RESOLUTION_HIGH as libc::c_int & 0xff as libc::c_int)
                << 8 as libc::c_int
                | FREENECT_VIDEO_RGB as libc::c_int & 0xff as libc::c_int) as uint32_t,
            resolution: FREENECT_RESOLUTION_HIGH,
            c2rust_unnamed: C2RustUnnamed {
                dummy: FREENECT_VIDEO_RGB as libc::c_int,
            },
            bytes: 1280 as libc::c_int * 1024 as libc::c_int * 3 as libc::c_int,
            width: 1280 as libc::c_int as int16_t,
            height: 1024 as libc::c_int as int16_t,
            data_bits_per_pixel: 24 as libc::c_int as int8_t,
            padding_bits_per_pixel: 0 as libc::c_int as int8_t,
            framerate: 10 as libc::c_int as int8_t,
            is_valid: 1 as libc::c_int as int8_t,
        };
        init
    },
    {
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
    },
    {
        let mut init = freenect_frame_mode {
            reserved: ((FREENECT_RESOLUTION_HIGH as libc::c_int & 0xff as libc::c_int)
                << 8 as libc::c_int
                | FREENECT_VIDEO_BAYER as libc::c_int & 0xff as libc::c_int) as uint32_t,
            resolution: FREENECT_RESOLUTION_HIGH,
            c2rust_unnamed: C2RustUnnamed {
                dummy: FREENECT_VIDEO_BAYER as libc::c_int,
            },
            bytes: 1280 as libc::c_int * 1024 as libc::c_int,
            width: 1280 as libc::c_int as int16_t,
            height: 1024 as libc::c_int as int16_t,
            data_bits_per_pixel: 8 as libc::c_int as int8_t,
            padding_bits_per_pixel: 0 as libc::c_int as int8_t,
            framerate: 10 as libc::c_int as int8_t,
            is_valid: 1 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = freenect_frame_mode {
            reserved: ((FREENECT_RESOLUTION_MEDIUM as libc::c_int & 0xff as libc::c_int)
                << 8 as libc::c_int
                | FREENECT_VIDEO_BAYER as libc::c_int & 0xff as libc::c_int) as uint32_t,
            resolution: FREENECT_RESOLUTION_MEDIUM,
            c2rust_unnamed: C2RustUnnamed {
                dummy: FREENECT_VIDEO_BAYER as libc::c_int,
            },
            bytes: 640 as libc::c_int * 480 as libc::c_int,
            width: 640 as libc::c_int as int16_t,
            height: 480 as libc::c_int as int16_t,
            data_bits_per_pixel: 8 as libc::c_int as int8_t,
            padding_bits_per_pixel: 0 as libc::c_int as int8_t,
            framerate: 30 as libc::c_int as int8_t,
            is_valid: 1 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = freenect_frame_mode {
            reserved: ((FREENECT_RESOLUTION_HIGH as libc::c_int & 0xff as libc::c_int)
                << 8 as libc::c_int
                | FREENECT_VIDEO_IR_8BIT as libc::c_int & 0xff as libc::c_int)
                as uint32_t,
            resolution: FREENECT_RESOLUTION_HIGH,
            c2rust_unnamed: C2RustUnnamed {
                dummy: FREENECT_VIDEO_IR_8BIT as libc::c_int,
            },
            bytes: 1280 as libc::c_int * 1024 as libc::c_int,
            width: 1280 as libc::c_int as int16_t,
            height: 1024 as libc::c_int as int16_t,
            data_bits_per_pixel: 8 as libc::c_int as int8_t,
            padding_bits_per_pixel: 0 as libc::c_int as int8_t,
            framerate: 10 as libc::c_int as int8_t,
            is_valid: 1 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = freenect_frame_mode {
            reserved: ((FREENECT_RESOLUTION_MEDIUM as libc::c_int & 0xff as libc::c_int)
                << 8 as libc::c_int
                | FREENECT_VIDEO_IR_8BIT as libc::c_int & 0xff as libc::c_int)
                as uint32_t,
            resolution: FREENECT_RESOLUTION_MEDIUM,
            c2rust_unnamed: C2RustUnnamed {
                dummy: FREENECT_VIDEO_IR_8BIT as libc::c_int,
            },
            bytes: 640 as libc::c_int * 488 as libc::c_int,
            width: 640 as libc::c_int as int16_t,
            height: 488 as libc::c_int as int16_t,
            data_bits_per_pixel: 8 as libc::c_int as int8_t,
            padding_bits_per_pixel: 0 as libc::c_int as int8_t,
            framerate: 30 as libc::c_int as int8_t,
            is_valid: 1 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = freenect_frame_mode {
            reserved: ((FREENECT_RESOLUTION_HIGH as libc::c_int & 0xff as libc::c_int)
                << 8 as libc::c_int
                | FREENECT_VIDEO_IR_10BIT as libc::c_int & 0xff as libc::c_int)
                as uint32_t,
            resolution: FREENECT_RESOLUTION_HIGH,
            c2rust_unnamed: C2RustUnnamed {
                dummy: FREENECT_VIDEO_IR_10BIT as libc::c_int,
            },
            bytes: 1280 as libc::c_int * 1024 as libc::c_int * 2 as libc::c_int,
            width: 1280 as libc::c_int as int16_t,
            height: 1024 as libc::c_int as int16_t,
            data_bits_per_pixel: 10 as libc::c_int as int8_t,
            padding_bits_per_pixel: 6 as libc::c_int as int8_t,
            framerate: 10 as libc::c_int as int8_t,
            is_valid: 1 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = freenect_frame_mode {
            reserved: ((FREENECT_RESOLUTION_MEDIUM as libc::c_int & 0xff as libc::c_int)
                << 8 as libc::c_int
                | FREENECT_VIDEO_IR_10BIT as libc::c_int & 0xff as libc::c_int)
                as uint32_t,
            resolution: FREENECT_RESOLUTION_MEDIUM,
            c2rust_unnamed: C2RustUnnamed {
                dummy: FREENECT_VIDEO_IR_10BIT as libc::c_int,
            },
            bytes: 640 as libc::c_int * 488 as libc::c_int * 2 as libc::c_int,
            width: 640 as libc::c_int as int16_t,
            height: 488 as libc::c_int as int16_t,
            data_bits_per_pixel: 10 as libc::c_int as int8_t,
            padding_bits_per_pixel: 6 as libc::c_int as int8_t,
            framerate: 30 as libc::c_int as int8_t,
            is_valid: 1 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = freenect_frame_mode {
            reserved: ((FREENECT_RESOLUTION_HIGH as libc::c_int & 0xff as libc::c_int)
                << 8 as libc::c_int
                | FREENECT_VIDEO_IR_10BIT_PACKED as libc::c_int & 0xff as libc::c_int)
                as uint32_t,
            resolution: FREENECT_RESOLUTION_HIGH,
            c2rust_unnamed: C2RustUnnamed {
                dummy: FREENECT_VIDEO_IR_10BIT_PACKED as libc::c_int,
            },
            bytes: 1280 as libc::c_int * 1024 as libc::c_int * 10 as libc::c_int
                / 8 as libc::c_int,
            width: 1280 as libc::c_int as int16_t,
            height: 1024 as libc::c_int as int16_t,
            data_bits_per_pixel: 10 as libc::c_int as int8_t,
            padding_bits_per_pixel: 0 as libc::c_int as int8_t,
            framerate: 10 as libc::c_int as int8_t,
            is_valid: 1 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = freenect_frame_mode {
            reserved: ((FREENECT_RESOLUTION_MEDIUM as libc::c_int & 0xff as libc::c_int)
                << 8 as libc::c_int
                | FREENECT_VIDEO_IR_10BIT_PACKED as libc::c_int & 0xff as libc::c_int)
                as uint32_t,
            resolution: FREENECT_RESOLUTION_MEDIUM,
            c2rust_unnamed: C2RustUnnamed {
                dummy: FREENECT_VIDEO_IR_10BIT_PACKED as libc::c_int,
            },
            bytes: 640 as libc::c_int * 488 as libc::c_int * 10 as libc::c_int
                / 8 as libc::c_int,
            width: 640 as libc::c_int as int16_t,
            height: 488 as libc::c_int as int16_t,
            data_bits_per_pixel: 10 as libc::c_int as int8_t,
            padding_bits_per_pixel: 0 as libc::c_int as int8_t,
            framerate: 30 as libc::c_int as int8_t,
            is_valid: 1 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = freenect_frame_mode {
            reserved: ((FREENECT_RESOLUTION_MEDIUM as libc::c_int & 0xff as libc::c_int)
                << 8 as libc::c_int
                | FREENECT_VIDEO_YUV_RGB as libc::c_int & 0xff as libc::c_int)
                as uint32_t,
            resolution: FREENECT_RESOLUTION_MEDIUM,
            c2rust_unnamed: C2RustUnnamed {
                dummy: FREENECT_VIDEO_YUV_RGB as libc::c_int,
            },
            bytes: 640 as libc::c_int * 480 as libc::c_int * 3 as libc::c_int,
            width: 640 as libc::c_int as int16_t,
            height: 480 as libc::c_int as int16_t,
            data_bits_per_pixel: 24 as libc::c_int as int8_t,
            padding_bits_per_pixel: 0 as libc::c_int as int8_t,
            framerate: 15 as libc::c_int as int8_t,
            is_valid: 1 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = freenect_frame_mode {
            reserved: ((FREENECT_RESOLUTION_MEDIUM as libc::c_int & 0xff as libc::c_int)
                << 8 as libc::c_int
                | FREENECT_VIDEO_YUV_RAW as libc::c_int & 0xff as libc::c_int)
                as uint32_t,
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
    },
];
static mut supported_depth_modes: [freenect_frame_mode; 6] = [
    {
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
    },
    {
        let mut init = freenect_frame_mode {
            reserved: ((FREENECT_RESOLUTION_MEDIUM as libc::c_int & 0xff as libc::c_int)
                << 8 as libc::c_int
                | FREENECT_DEPTH_10BIT as libc::c_int & 0xff as libc::c_int) as uint32_t,
            resolution: FREENECT_RESOLUTION_MEDIUM,
            c2rust_unnamed: C2RustUnnamed {
                dummy: FREENECT_DEPTH_10BIT as libc::c_int,
            },
            bytes: 640 as libc::c_int * 480 as libc::c_int * 2 as libc::c_int,
            width: 640 as libc::c_int as int16_t,
            height: 480 as libc::c_int as int16_t,
            data_bits_per_pixel: 10 as libc::c_int as int8_t,
            padding_bits_per_pixel: 6 as libc::c_int as int8_t,
            framerate: 30 as libc::c_int as int8_t,
            is_valid: 1 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = freenect_frame_mode {
            reserved: ((FREENECT_RESOLUTION_MEDIUM as libc::c_int & 0xff as libc::c_int)
                << 8 as libc::c_int
                | FREENECT_DEPTH_11BIT_PACKED as libc::c_int & 0xff as libc::c_int)
                as uint32_t,
            resolution: FREENECT_RESOLUTION_MEDIUM,
            c2rust_unnamed: C2RustUnnamed {
                dummy: FREENECT_DEPTH_11BIT_PACKED as libc::c_int,
            },
            bytes: 640 as libc::c_int * 480 as libc::c_int * 11 as libc::c_int
                / 8 as libc::c_int,
            width: 640 as libc::c_int as int16_t,
            height: 480 as libc::c_int as int16_t,
            data_bits_per_pixel: 11 as libc::c_int as int8_t,
            padding_bits_per_pixel: 0 as libc::c_int as int8_t,
            framerate: 30 as libc::c_int as int8_t,
            is_valid: 1 as libc::c_int as int8_t,
        };
        init
    },
    {
        let mut init = freenect_frame_mode {
            reserved: ((FREENECT_RESOLUTION_MEDIUM as libc::c_int & 0xff as libc::c_int)
                << 8 as libc::c_int
                | FREENECT_DEPTH_10BIT_PACKED as libc::c_int & 0xff as libc::c_int)
                as uint32_t,
            resolution: FREENECT_RESOLUTION_MEDIUM,
            c2rust_unnamed: C2RustUnnamed {
                dummy: FREENECT_DEPTH_10BIT_PACKED as libc::c_int,
            },
            bytes: 640 as libc::c_int * 480 as libc::c_int * 10 as libc::c_int
                / 8 as libc::c_int,
            width: 640 as libc::c_int as int16_t,
            height: 480 as libc::c_int as int16_t,
            data_bits_per_pixel: 10 as libc::c_int as int8_t,
            padding_bits_per_pixel: 0 as libc::c_int as int8_t,
            framerate: 30 as libc::c_int as int8_t,
            is_valid: 1 as libc::c_int as int8_t,
        };
        init
    },
    {
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
    },
    {
        let mut init = freenect_frame_mode {
            reserved: ((FREENECT_RESOLUTION_MEDIUM as libc::c_int & 0xff as libc::c_int)
                << 8 as libc::c_int
                | FREENECT_DEPTH_MM as libc::c_int & 0xff as libc::c_int) as uint32_t,
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
    },
];
unsafe extern "C" fn stream_process(
    mut ctx: *mut freenect_context,
    mut strm: *mut packet_stream,
    mut pkt: *mut uint8_t,
    mut len: libc::c_int,
    mut cb: freenect_chunk_cb,
    mut user_data: *mut libc::c_void,
) -> libc::c_int {
    if len < 12 as libc::c_int {
        return 0 as libc::c_int;
    }
    let mut hdr: *mut pkt_hdr = pkt as *mut pkt_hdr;
    let mut data: *mut uint8_t = pkt
        .offset(::std::mem::size_of::<pkt_hdr>() as libc::c_ulong as isize);
    let mut datalen: libc::c_int = (len as libc::c_ulong)
        .wrapping_sub(::std::mem::size_of::<pkt_hdr>() as libc::c_ulong) as libc::c_int;
    let mut l_info: freenect_loglevel = FREENECT_LOG_INFO;
    let mut l_notice: freenect_loglevel = FREENECT_LOG_NOTICE;
    let mut l_warning: freenect_loglevel = FREENECT_LOG_WARNING;
    if (*strm).valid_frames < 2 as libc::c_int {
        l_warning = FREENECT_LOG_SPEW;
        l_notice = l_warning;
        l_info = l_notice;
    }
    if (*hdr).magic[0 as libc::c_int as usize] as libc::c_int != 'R' as i32
        || (*hdr).magic[1 as libc::c_int as usize] as libc::c_int != 'B' as i32
    {
        fn_log(
            ctx,
            l_notice,
            b"[Stream %02x] Invalid magic %02x%02x\n\0" as *const u8
                as *const libc::c_char,
            (*strm).flag as libc::c_int,
            (*hdr).magic[0 as libc::c_int as usize] as libc::c_int,
            (*hdr).magic[1 as libc::c_int as usize] as libc::c_int,
        );
        return 0 as libc::c_int;
    }
    fn_log(
        ctx,
        FREENECT_LOG_FLOOD,
        b"[Stream %02x] Packet with flag: %02x\n\0" as *const u8 as *const libc::c_char,
        (*strm).flag as libc::c_int,
        (*hdr).flag as libc::c_int,
    );
    let mut sof: uint8_t = ((*strm).flag as libc::c_int | 1 as libc::c_int) as uint8_t;
    let mut mof: uint8_t = ((*strm).flag as libc::c_int | 2 as libc::c_int) as uint8_t;
    let mut eof: uint8_t = ((*strm).flag as libc::c_int | 5 as libc::c_int) as uint8_t;
    if (*strm).synced == 0 {
        if (*hdr).flag as libc::c_int != sof as libc::c_int {
            fn_log(
                ctx,
                FREENECT_LOG_SPEW,
                b"[Stream %02x] Not synced yet...\n\0" as *const u8
                    as *const libc::c_char,
                (*strm).flag as libc::c_int,
            );
            return 0 as libc::c_int;
        }
        (*strm).synced = 1 as libc::c_int;
        (*strm).seq = (*hdr).seq;
        (*strm).pkt_num = 0 as libc::c_int;
        (*strm).valid_pkts = 0 as libc::c_int;
        (*strm).got_pkts = 0 as libc::c_int;
    }
    let mut got_frame_size: libc::c_int = 0 as libc::c_int;
    if (*strm).seq as libc::c_int != (*hdr).seq as libc::c_int {
        let mut lost: uint8_t = ((*hdr).seq as libc::c_int - (*strm).seq as libc::c_int)
            as uint8_t;
        (*strm).lost_pkts = ((*strm).lost_pkts).wrapping_add(lost as libc::c_uint);
        fn_log(
            ctx,
            l_info,
            b"[Stream %02x] Lost %d packets\n\0" as *const u8 as *const libc::c_char,
            (*strm).flag as libc::c_int,
            lost as libc::c_int,
        );
        fn_log(
            ctx,
            FREENECT_LOG_DEBUG,
            b"[Stream %02x] Lost %d total packets in %d frames (%f lppf)\n\0"
                as *const u8 as *const libc::c_char,
            (*strm).flag as libc::c_int,
            (*strm).lost_pkts,
            (*strm).valid_frames,
            ((*strm).lost_pkts as libc::c_float / (*strm).valid_frames as libc::c_float)
                as libc::c_double,
        );
        if lost as libc::c_int > 5 as libc::c_int || (*strm).variable_length != 0 {
            fn_log(
                ctx,
                l_notice,
                b"[Stream %02x] Lost too many packets, resyncing...\n\0" as *const u8
                    as *const libc::c_char,
                (*strm).flag as libc::c_int,
            );
            (*strm).synced = 0 as libc::c_int;
            return 0 as libc::c_int;
        }
        (*strm).seq = (*hdr).seq;
        let mut left: libc::c_int = (*strm).pkts_per_frame - (*strm).pkt_num;
        if left <= lost as libc::c_int {
            (*strm).pkt_num = lost as libc::c_int - left;
            (*strm).valid_pkts = (*strm).got_pkts;
            (*strm).got_pkts = 0 as libc::c_int;
            got_frame_size = (*strm).frame_size;
            (*strm).timestamp = (*strm).last_timestamp;
            (*strm).valid_frames += 1;
            (*strm).valid_frames;
        } else {
            (*strm).pkt_num += lost as libc::c_int;
        }
    }
    let mut expected_pkt_size: libc::c_int = if (*strm).pkt_num
        == (*strm).pkts_per_frame - 1 as libc::c_int
    {
        (*strm).last_pkt_size
    } else {
        (*strm).pkt_size
    };
    if (*strm).variable_length == 0 {
        if !((*strm).pkt_num == 0 as libc::c_int
            && (*hdr).flag as libc::c_int == sof as libc::c_int)
            && !((*strm).pkt_num == (*strm).pkts_per_frame - 1 as libc::c_int
                && (*hdr).flag as libc::c_int == eof as libc::c_int)
            && !((*strm).pkt_num > 0 as libc::c_int
                && (*strm).pkt_num < (*strm).pkts_per_frame - 1 as libc::c_int
                && (*hdr).flag as libc::c_int == mof as libc::c_int)
        {
            fn_log(
                ctx,
                l_notice,
                b"[Stream %02x] Inconsistent flag %02x with %d packets in buf (%d total), resyncing...\n\0"
                    as *const u8 as *const libc::c_char,
                (*strm).flag as libc::c_int,
                (*hdr).flag as libc::c_int,
                (*strm).pkt_num,
                (*strm).pkts_per_frame,
            );
            (*strm).synced = 0 as libc::c_int;
            return got_frame_size;
        }
        if datalen > expected_pkt_size {
            fn_log(
                ctx,
                l_warning,
                b"[Stream %02x] Expected max %d data bytes, but got %d. Dropping...\n\0"
                    as *const u8 as *const libc::c_char,
                (*strm).flag as libc::c_int,
                expected_pkt_size,
                datalen,
            );
            return got_frame_size;
        }
        if datalen < expected_pkt_size {
            fn_log(
                ctx,
                l_warning,
                b"[Stream %02x] Expected %d data bytes, but got %d\n\0" as *const u8
                    as *const libc::c_char,
                (*strm).flag as libc::c_int,
                expected_pkt_size,
                datalen,
            );
        }
    } else {
        if !((*strm).pkt_num == 0 as libc::c_int
            && (*hdr).flag as libc::c_int == sof as libc::c_int)
            && !((*strm).pkt_num < (*strm).pkts_per_frame
                && ((*hdr).flag as libc::c_int == eof as libc::c_int
                    || (*hdr).flag as libc::c_int == mof as libc::c_int))
        {
            fn_log(
                ctx,
                l_notice,
                b"[Stream %02x] Inconsistent flag %02x with %d packets in buf (%d total), resyncing...\n\0"
                    as *const u8 as *const libc::c_char,
                (*strm).flag as libc::c_int,
                (*hdr).flag as libc::c_int,
                (*strm).pkt_num,
                (*strm).pkts_per_frame,
            );
            (*strm).synced = 0 as libc::c_int;
            return got_frame_size;
        }
        if datalen > expected_pkt_size {
            fn_log(
                ctx,
                l_warning,
                b"[Stream %02x] Expected max %d data bytes, but got %d. Resyncng...\n\0"
                    as *const u8 as *const libc::c_char,
                (*strm).flag as libc::c_int,
                expected_pkt_size,
                datalen,
            );
            (*strm).synced = 0 as libc::c_int;
            return got_frame_size;
        }
        if datalen < expected_pkt_size
            && (*hdr).flag as libc::c_int != eof as libc::c_int
        {
            fn_log(
                ctx,
                l_warning,
                b"[Stream %02x] Expected %d data bytes, but got %d. Resyncing...\n\0"
                    as *const u8 as *const libc::c_char,
                (*strm).flag as libc::c_int,
                expected_pkt_size,
                datalen,
            );
            (*strm).synced = 0 as libc::c_int;
            return got_frame_size;
        }
    }
    let mut dbuf: *mut uint8_t = ((*strm).raw_buf)
        .offset(((*strm).pkt_num * (*strm).pkt_size) as isize);
    if cb.is_some() {
        cb
            .unwrap()(
            (*strm).raw_buf as *mut libc::c_void,
            data as *mut libc::c_void,
            (*strm).pkt_num,
            datalen,
            user_data,
        );
    } else {
        memcpy(
            dbuf as *mut libc::c_void,
            data as *const libc::c_void,
            datalen as libc::c_ulong,
        );
    }
    (*strm).pkt_num += 1;
    (*strm).pkt_num;
    (*strm).seq = ((*strm).seq).wrapping_add(1);
    (*strm).seq;
    (*strm).got_pkts += 1;
    (*strm).got_pkts;
    (*strm).last_timestamp = (*hdr).timestamp;
    if (*hdr).flag as libc::c_int == eof as libc::c_int {
        if (*strm).variable_length != 0 {
            got_frame_size = (dbuf.offset_from((*strm).raw_buf) as libc::c_long
                + datalen as libc::c_long) as libc::c_int;
        } else {
            got_frame_size = (dbuf.offset_from((*strm).raw_buf) as libc::c_long
                + (*strm).last_pkt_size as libc::c_long) as libc::c_int;
        }
        (*strm).pkt_num = 0 as libc::c_int;
        (*strm).valid_pkts = (*strm).got_pkts;
        (*strm).got_pkts = 0 as libc::c_int;
        (*strm).timestamp = (*strm).last_timestamp;
        (*strm).valid_frames += 1;
        (*strm).valid_frames;
    }
    return got_frame_size;
}
unsafe extern "C" fn stream_init(
    mut ctx: *mut freenect_context,
    mut strm: *mut packet_stream,
    mut rlen: libc::c_int,
    mut plen: libc::c_int,
) {
    (*strm).valid_frames = 0 as libc::c_int;
    (*strm).synced = 0 as libc::c_int;
    if !((*strm).usr_buf).is_null() {
        (*strm).lib_buf = 0 as *mut libc::c_void;
        (*strm).proc_buf = (*strm).usr_buf;
    } else {
        (*strm).lib_buf = malloc(plen as libc::c_ulong);
        (*strm).proc_buf = (*strm).lib_buf;
    }
    if rlen == 0 as libc::c_int {
        (*strm).split_bufs = 0 as libc::c_int;
        (*strm).raw_buf = (*strm).proc_buf as *mut uint8_t;
        (*strm).frame_size = plen;
    } else {
        (*strm).split_bufs = 1 as libc::c_int;
        (*strm).raw_buf = malloc(rlen as libc::c_ulong) as *mut uint8_t;
        (*strm).frame_size = rlen;
    }
    (*strm).last_pkt_size = (*strm).frame_size % (*strm).pkt_size;
    if (*strm).last_pkt_size == 0 as libc::c_int {
        (*strm).last_pkt_size = (*strm).pkt_size;
    }
    (*strm)
        .pkts_per_frame = ((*strm).frame_size + (*strm).pkt_size - 1 as libc::c_int)
        / (*strm).pkt_size;
}
unsafe extern "C" fn stream_freebufs(
    mut ctx: *mut freenect_context,
    mut strm: *mut packet_stream,
) {
    if (*strm).split_bufs != 0 {
        free((*strm).raw_buf as *mut libc::c_void);
    }
    if !((*strm).lib_buf).is_null() {
        free((*strm).lib_buf);
    }
    (*strm).raw_buf = 0 as *mut uint8_t;
    (*strm).proc_buf = 0 as *mut libc::c_void;
    (*strm).lib_buf = 0 as *mut libc::c_void;
}
unsafe extern "C" fn stream_setbuf(
    mut ctx: *mut freenect_context,
    mut strm: *mut packet_stream,
    mut pbuf: *mut libc::c_void,
) -> libc::c_int {
    if (*strm).running == 0 {
        (*strm).usr_buf = pbuf;
        return 0 as libc::c_int;
    } else {
        if pbuf.is_null() && ((*strm).lib_buf).is_null() {
            fn_log(
                ctx,
                FREENECT_LOG_ERROR,
                b"Attempted to set buffer to NULL but stream was started with no internal buffer\n\0"
                    as *const u8 as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
        (*strm).usr_buf = pbuf;
        if pbuf.is_null() {
            (*strm).proc_buf = (*strm).lib_buf;
        } else {
            (*strm).proc_buf = pbuf;
        }
        if (*strm).split_bufs == 0 {
            (*strm).raw_buf = (*strm).proc_buf as *mut uint8_t;
        }
        return 0 as libc::c_int;
    };
}
#[inline]
unsafe extern "C" fn convert_packed_to_16bit(
    mut src: *mut uint8_t,
    mut dest: *mut uint16_t,
    mut vw: libc::c_int,
    mut n: libc::c_int,
) {
    let mut mask: libc::c_uint = (((1 as libc::c_int) << vw) - 1 as libc::c_int)
        as libc::c_uint;
    let mut buffer: uint32_t = 0 as libc::c_int as uint32_t;
    let mut bitsIn: libc::c_int = 0 as libc::c_int;
    loop {
        let fresh0 = n;
        n = n - 1;
        if !(fresh0 != 0) {
            break;
        }
        while bitsIn < vw {
            let fresh1 = src;
            src = src.offset(1);
            buffer = buffer << 8 as libc::c_int | *fresh1 as libc::c_uint;
            bitsIn += 8 as libc::c_int;
        }
        bitsIn -= vw;
        let fresh2 = dest;
        dest = dest.offset(1);
        *fresh2 = (buffer >> bitsIn & mask) as uint16_t;
    };
}
#[inline]
unsafe extern "C" fn convert_packed_to_8bit(
    mut src: *mut uint8_t,
    mut dest: *mut uint8_t,
    mut vw: libc::c_int,
    mut n: libc::c_int,
) {
    let mut buffer: uint32_t = 0 as libc::c_int as uint32_t;
    let mut bitsIn: libc::c_int = 0 as libc::c_int;
    loop {
        let fresh3 = n;
        n = n - 1;
        if !(fresh3 != 0) {
            break;
        }
        while bitsIn < vw {
            let fresh4 = src;
            src = src.offset(1);
            buffer = buffer << 8 as libc::c_int | *fresh4 as libc::c_uint;
            bitsIn += 8 as libc::c_int;
        }
        bitsIn -= vw;
        let fresh5 = dest;
        dest = dest.offset(1);
        *fresh5 = (buffer >> bitsIn + vw - 8 as libc::c_int) as uint8_t;
    };
}
unsafe extern "C" fn convert_packed11_to_16bit(
    mut raw: *mut uint8_t,
    mut frame: *mut uint16_t,
    mut n: libc::c_int,
) {
    let mut baseMask: uint16_t = (((1 as libc::c_int) << 11 as libc::c_int)
        - 1 as libc::c_int) as uint16_t;
    while n >= 8 as libc::c_int {
        let mut r0: uint8_t = *raw.offset(0 as libc::c_int as isize);
        let mut r1: uint8_t = *raw.offset(1 as libc::c_int as isize);
        let mut r2: uint8_t = *raw.offset(2 as libc::c_int as isize);
        let mut r3: uint8_t = *raw.offset(3 as libc::c_int as isize);
        let mut r4: uint8_t = *raw.offset(4 as libc::c_int as isize);
        let mut r5: uint8_t = *raw.offset(5 as libc::c_int as isize);
        let mut r6: uint8_t = *raw.offset(6 as libc::c_int as isize);
        let mut r7: uint8_t = *raw.offset(7 as libc::c_int as isize);
        let mut r8: uint8_t = *raw.offset(8 as libc::c_int as isize);
        let mut r9: uint8_t = *raw.offset(9 as libc::c_int as isize);
        let mut r10: uint8_t = *raw.offset(10 as libc::c_int as isize);
        *frame
            .offset(
                0 as libc::c_int as isize,
            ) = ((r0 as libc::c_int) << 3 as libc::c_int
            | r1 as libc::c_int >> 5 as libc::c_int) as uint16_t;
        *frame
            .offset(
                1 as libc::c_int as isize,
            ) = (((r1 as libc::c_int) << 6 as libc::c_int
            | r2 as libc::c_int >> 2 as libc::c_int) & baseMask as libc::c_int)
            as uint16_t;
        *frame
            .offset(
                2 as libc::c_int as isize,
            ) = (((r2 as libc::c_int) << 9 as libc::c_int
            | (r3 as libc::c_int) << 1 as libc::c_int
            | r4 as libc::c_int >> 7 as libc::c_int) & baseMask as libc::c_int)
            as uint16_t;
        *frame
            .offset(
                3 as libc::c_int as isize,
            ) = (((r4 as libc::c_int) << 4 as libc::c_int
            | r5 as libc::c_int >> 4 as libc::c_int) & baseMask as libc::c_int)
            as uint16_t;
        *frame
            .offset(
                4 as libc::c_int as isize,
            ) = (((r5 as libc::c_int) << 7 as libc::c_int
            | r6 as libc::c_int >> 1 as libc::c_int) & baseMask as libc::c_int)
            as uint16_t;
        *frame
            .offset(
                5 as libc::c_int as isize,
            ) = (((r6 as libc::c_int) << 10 as libc::c_int
            | (r7 as libc::c_int) << 2 as libc::c_int
            | r8 as libc::c_int >> 6 as libc::c_int) & baseMask as libc::c_int)
            as uint16_t;
        *frame
            .offset(
                6 as libc::c_int as isize,
            ) = (((r8 as libc::c_int) << 5 as libc::c_int
            | r9 as libc::c_int >> 3 as libc::c_int) & baseMask as libc::c_int)
            as uint16_t;
        *frame
            .offset(
                7 as libc::c_int as isize,
            ) = (((r9 as libc::c_int) << 8 as libc::c_int | r10 as libc::c_int)
            & baseMask as libc::c_int) as uint16_t;
        n -= 8 as libc::c_int;
        raw = raw.offset(11 as libc::c_int as isize);
        frame = frame.offset(8 as libc::c_int as isize);
    }
}
unsafe extern "C" fn depth_process(
    mut dev: *mut freenect_device,
    mut pkt: *mut uint8_t,
    mut len: libc::c_int,
) {
    let mut ctx: *mut freenect_context = (*dev).parent;
    if len == 0 as libc::c_int {
        return;
    }
    if (*dev).depth.running == 0 {
        return;
    }
    let mut got_frame_size: libc::c_int = stream_process(
        ctx,
        &mut (*dev).depth,
        pkt,
        len,
        (*dev).depth_chunk_cb,
        (*dev).user_data,
    );
    if got_frame_size == 0 {
        return;
    }
    fn_log(
        ctx,
        FREENECT_LOG_SPEW,
        b"Got depth frame of size %d/%d, %d/%d packets arrived, TS %08x\n\0" as *const u8
            as *const libc::c_char,
        got_frame_size,
        (*dev).depth.frame_size,
        (*dev).depth.valid_pkts,
        (*dev).depth.pkts_per_frame,
        (*dev).depth.timestamp,
    );
    match (*dev).depth_format as libc::c_uint {
        0 => {
            convert_packed11_to_16bit(
                (*dev).depth.raw_buf,
                (*dev).depth.proc_buf as *mut uint16_t,
                640 as libc::c_int * 480 as libc::c_int,
            );
        }
        4 => {
            freenect_apply_registration(
                dev,
                (*dev).depth.raw_buf,
                (*dev).depth.proc_buf as *mut uint16_t,
                0 as libc::c_int != 0,
            );
        }
        5 => {
            freenect_apply_depth_to_mm(
                dev,
                (*dev).depth.raw_buf,
                (*dev).depth.proc_buf as *mut uint16_t,
            );
        }
        1 => {
            convert_packed_to_16bit(
                (*dev).depth.raw_buf,
                (*dev).depth.proc_buf as *mut uint16_t,
                10 as libc::c_int,
                640 as libc::c_int * 480 as libc::c_int,
            );
        }
        3 | 2 => {}
        _ => {
            fn_log(
                ctx,
                FREENECT_LOG_ERROR,
                b"depth_process() was called, but an invalid depth_format is set\n\0"
                    as *const u8 as *const libc::c_char,
            );
        }
    }
    if ((*dev).depth_cb).is_some() {
        ((*dev).depth_cb).unwrap()(dev, (*dev).depth.proc_buf, (*dev).depth.timestamp);
    }
}
unsafe extern "C" fn convert_uyvy_to_rgb(
    mut raw_buf: *mut uint8_t,
    mut proc_buf: *mut uint8_t,
    mut frame_mode: freenect_frame_mode,
) {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    y = 0 as libc::c_int;
    while y < frame_mode.height as libc::c_int {
        x = 0 as libc::c_int;
        while x < frame_mode.width as libc::c_int {
            let mut i: libc::c_int = frame_mode.width as libc::c_int * y + x;
            let mut u: libc::c_int = *raw_buf.offset((2 as libc::c_int * i) as isize)
                as libc::c_int;
            let mut y1: libc::c_int = *raw_buf
                .offset((2 as libc::c_int * i + 1 as libc::c_int) as isize)
                as libc::c_int;
            let mut v: libc::c_int = *raw_buf
                .offset((2 as libc::c_int * i + 2 as libc::c_int) as isize)
                as libc::c_int;
            let mut y2: libc::c_int = *raw_buf
                .offset((2 as libc::c_int * i + 3 as libc::c_int) as isize)
                as libc::c_int;
            let mut r1: libc::c_int = (y1 - 16 as libc::c_int) * 1164 as libc::c_int
                / 1000 as libc::c_int
                + (v - 128 as libc::c_int) * 1596 as libc::c_int / 1000 as libc::c_int;
            let mut g1: libc::c_int = (y1 - 16 as libc::c_int) * 1164 as libc::c_int
                / 1000 as libc::c_int
                - (v - 128 as libc::c_int) * 813 as libc::c_int / 1000 as libc::c_int
                - (u - 128 as libc::c_int) * 391 as libc::c_int / 1000 as libc::c_int;
            let mut b1: libc::c_int = (y1 - 16 as libc::c_int) * 1164 as libc::c_int
                / 1000 as libc::c_int
                + (u - 128 as libc::c_int) * 2018 as libc::c_int / 1000 as libc::c_int;
            let mut r2: libc::c_int = (y2 - 16 as libc::c_int) * 1164 as libc::c_int
                / 1000 as libc::c_int
                + (v - 128 as libc::c_int) * 1596 as libc::c_int / 1000 as libc::c_int;
            let mut g2: libc::c_int = (y2 - 16 as libc::c_int) * 1164 as libc::c_int
                / 1000 as libc::c_int
                - (v - 128 as libc::c_int) * 813 as libc::c_int / 1000 as libc::c_int
                - (u - 128 as libc::c_int) * 391 as libc::c_int / 1000 as libc::c_int;
            let mut b2: libc::c_int = (y2 - 16 as libc::c_int) * 1164 as libc::c_int
                / 1000 as libc::c_int
                + (u - 128 as libc::c_int) * 2018 as libc::c_int / 1000 as libc::c_int;
            if r1 < 0 as libc::c_int {
                r1 = 0 as libc::c_int;
            }
            if r1 > 255 as libc::c_int {
                r1 = 255 as libc::c_int;
            }
            if g1 < 0 as libc::c_int {
                g1 = 0 as libc::c_int;
            }
            if g1 > 255 as libc::c_int {
                g1 = 255 as libc::c_int;
            }
            if b1 < 0 as libc::c_int {
                b1 = 0 as libc::c_int;
            }
            if b1 > 255 as libc::c_int {
                b1 = 255 as libc::c_int;
            }
            if r2 < 0 as libc::c_int {
                r2 = 0 as libc::c_int;
            }
            if r2 > 255 as libc::c_int {
                r2 = 255 as libc::c_int;
            }
            if g2 < 0 as libc::c_int {
                g2 = 0 as libc::c_int;
            }
            if g2 > 255 as libc::c_int {
                g2 = 255 as libc::c_int;
            }
            if b2 < 0 as libc::c_int {
                b2 = 0 as libc::c_int;
            }
            if b2 > 255 as libc::c_int {
                b2 = 255 as libc::c_int;
            }
            *proc_buf.offset((3 as libc::c_int * i) as isize) = r1 as uint8_t;
            *proc_buf
                .offset(
                    (3 as libc::c_int * i + 1 as libc::c_int) as isize,
                ) = g1 as uint8_t;
            *proc_buf
                .offset(
                    (3 as libc::c_int * i + 2 as libc::c_int) as isize,
                ) = b1 as uint8_t;
            *proc_buf
                .offset(
                    (3 as libc::c_int * i + 3 as libc::c_int) as isize,
                ) = r2 as uint8_t;
            *proc_buf
                .offset(
                    (3 as libc::c_int * i + 4 as libc::c_int) as isize,
                ) = g2 as uint8_t;
            *proc_buf
                .offset(
                    (3 as libc::c_int * i + 5 as libc::c_int) as isize,
                ) = b2 as uint8_t;
            x += 2 as libc::c_int;
        }
        y += 1;
        y;
    }
}
unsafe extern "C" fn convert_bayer_to_rgb(
    mut raw_buf: *mut uint8_t,
    mut proc_buf: *mut uint8_t,
    mut frame_mode: freenect_frame_mode,
) {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut dst: *mut uint8_t = proc_buf;
    let mut prevLine: *mut uint8_t = 0 as *mut uint8_t;
    let mut curLine: *mut uint8_t = 0 as *mut uint8_t;
    let mut nextLine: *mut uint8_t = 0 as *mut uint8_t;
    let mut hVals: uint32_t = 0;
    let mut vSums: uint32_t = 0;
    curLine = raw_buf;
    nextLine = curLine.offset(frame_mode.width as libc::c_int as isize);
    y = 0 as libc::c_int;
    while y < frame_mode.height as libc::c_int {
        if y > 0 as libc::c_int
            && y < frame_mode.height as libc::c_int - 1 as libc::c_int
        {
            prevLine = curLine.offset(-(frame_mode.width as libc::c_int as isize));
        } else if y == 0 as libc::c_int {
            prevLine = nextLine;
        } else {
            nextLine = prevLine;
        }
        let fresh6 = curLine;
        curLine = curLine.offset(1);
        hVals = ((*fresh6 as libc::c_int) << 8 as libc::c_int) as uint32_t;
        hVals |= ((*curLine as libc::c_int) << 16 as libc::c_int) as libc::c_uint;
        let fresh7 = prevLine;
        prevLine = prevLine.offset(1);
        let fresh8 = nextLine;
        nextLine = nextLine.offset(1);
        vSums = ((*fresh7 as libc::c_int + *fresh8 as libc::c_int) << 7 as libc::c_int
            & 0xff00 as libc::c_int) as uint32_t;
        vSums
            |= ((*prevLine as libc::c_int + *nextLine as libc::c_int)
                << 15 as libc::c_int & 0xff0000 as libc::c_int) as libc::c_uint;
        let mut yOdd: uint8_t = (y & 1 as libc::c_int) as uint8_t;
        x = 0 as libc::c_int;
        while x < frame_mode.width as libc::c_int - 1 as libc::c_int {
            let fresh9 = curLine;
            curLine = curLine.offset(1);
            hVals |= *fresh9 as libc::c_uint;
            let fresh10 = prevLine;
            prevLine = prevLine.offset(1);
            let fresh11 = nextLine;
            nextLine = nextLine.offset(1);
            vSums
                |= (*fresh10 as libc::c_int + *fresh11 as libc::c_int
                    >> 1 as libc::c_int) as libc::c_uint;
            let mut hSum: uint8_t = ((hVals >> 16 as libc::c_int) as uint8_t
                as libc::c_int + hVals as uint8_t as libc::c_int >> 1 as libc::c_int)
                as uint8_t;
            if yOdd as libc::c_int == 0 as libc::c_int {
                if x & 1 as libc::c_int == 0 as libc::c_int {
                    let fresh12 = dst;
                    dst = dst.offset(1);
                    *fresh12 = hSum;
                    let fresh13 = dst;
                    dst = dst.offset(1);
                    *fresh13 = (hVals >> 8 as libc::c_int) as uint8_t;
                    let fresh14 = dst;
                    dst = dst.offset(1);
                    *fresh14 = (vSums >> 8 as libc::c_int) as uint8_t;
                } else {
                    let fresh15 = dst;
                    dst = dst.offset(1);
                    *fresh15 = (hVals >> 8 as libc::c_int) as uint8_t;
                    let fresh16 = dst;
                    dst = dst.offset(1);
                    *fresh16 = (hSum as libc::c_int
                        + (vSums >> 8 as libc::c_int) as uint8_t as libc::c_int
                        >> 1 as libc::c_int) as uint8_t;
                    let fresh17 = dst;
                    dst = dst.offset(1);
                    *fresh17 = ((vSums >> 16 as libc::c_int) as uint8_t as libc::c_int
                        + vSums as uint8_t as libc::c_int >> 1 as libc::c_int)
                        as uint8_t;
                }
            } else if x & 1 as libc::c_int == 0 as libc::c_int {
                let fresh18 = dst;
                dst = dst.offset(1);
                *fresh18 = ((vSums >> 16 as libc::c_int) as uint8_t as libc::c_int
                    + vSums as uint8_t as libc::c_int >> 1 as libc::c_int) as uint8_t;
                let fresh19 = dst;
                dst = dst.offset(1);
                *fresh19 = (hSum as libc::c_int
                    + (vSums >> 8 as libc::c_int) as uint8_t as libc::c_int
                    >> 1 as libc::c_int) as uint8_t;
                let fresh20 = dst;
                dst = dst.offset(1);
                *fresh20 = (hVals >> 8 as libc::c_int) as uint8_t;
            } else {
                let fresh21 = dst;
                dst = dst.offset(1);
                *fresh21 = (vSums >> 8 as libc::c_int) as uint8_t;
                let fresh22 = dst;
                dst = dst.offset(1);
                *fresh22 = (hVals >> 8 as libc::c_int) as uint8_t;
                let fresh23 = dst;
                dst = dst.offset(1);
                *fresh23 = hSum;
            }
            hVals <<= 8 as libc::c_int;
            vSums <<= 8 as libc::c_int;
            x += 1;
            x;
        }
        hVals |= (hVals >> 16 as libc::c_int) as uint8_t as libc::c_uint;
        vSums |= (vSums >> 16 as libc::c_int) as uint8_t as libc::c_uint;
        let mut hSum_0: uint8_t = hVals as uint8_t;
        if yOdd as libc::c_int == 0 as libc::c_int {
            if x & 1 as libc::c_int == 0 as libc::c_int {
                let fresh24 = dst;
                dst = dst.offset(1);
                *fresh24 = hSum_0;
                let fresh25 = dst;
                dst = dst.offset(1);
                *fresh25 = (hVals >> 8 as libc::c_int) as uint8_t;
                let fresh26 = dst;
                dst = dst.offset(1);
                *fresh26 = (vSums >> 8 as libc::c_int) as uint8_t;
            } else {
                let fresh27 = dst;
                dst = dst.offset(1);
                *fresh27 = (hVals >> 8 as libc::c_int) as uint8_t;
                let fresh28 = dst;
                dst = dst.offset(1);
                *fresh28 = (hSum_0 as libc::c_int
                    + (vSums >> 8 as libc::c_int) as uint8_t as libc::c_int
                    >> 1 as libc::c_int) as uint8_t;
                let fresh29 = dst;
                dst = dst.offset(1);
                *fresh29 = vSums as uint8_t;
            }
        } else if x & 1 as libc::c_int == 0 as libc::c_int {
            let fresh30 = dst;
            dst = dst.offset(1);
            *fresh30 = vSums as uint8_t;
            let fresh31 = dst;
            dst = dst.offset(1);
            *fresh31 = (hSum_0 as libc::c_int
                + (vSums >> 8 as libc::c_int) as uint8_t as libc::c_int
                >> 1 as libc::c_int) as uint8_t;
            let fresh32 = dst;
            dst = dst.offset(1);
            *fresh32 = (hVals >> 8 as libc::c_int) as uint8_t;
        } else {
            let fresh33 = dst;
            dst = dst.offset(1);
            *fresh33 = (vSums >> 8 as libc::c_int) as uint8_t;
            let fresh34 = dst;
            dst = dst.offset(1);
            *fresh34 = (hVals >> 8 as libc::c_int) as uint8_t;
            let fresh35 = dst;
            dst = dst.offset(1);
            *fresh35 = hSum_0;
        }
        y += 1;
        y;
    }
}
unsafe extern "C" fn video_process(
    mut dev: *mut freenect_device,
    mut pkt: *mut uint8_t,
    mut len: libc::c_int,
) {
    let mut ctx: *mut freenect_context = (*dev).parent;
    if len == 0 as libc::c_int {
        return;
    }
    if (*dev).video.running == 0 {
        return;
    }
    let mut got_frame_size: libc::c_int = stream_process(
        ctx,
        &mut (*dev).video,
        pkt,
        len,
        (*dev).video_chunk_cb,
        (*dev).user_data,
    );
    if got_frame_size == 0 {
        return;
    }
    fn_log(
        ctx,
        FREENECT_LOG_SPEW,
        b"Got video frame of size %d/%d, %d/%d packets arrived, TS %08x\n\0" as *const u8
            as *const libc::c_char,
        got_frame_size,
        (*dev).video.frame_size,
        (*dev).video.valid_pkts,
        (*dev).video.pkts_per_frame,
        (*dev).video.timestamp,
    );
    let mut frame_mode: freenect_frame_mode = freenect_get_current_video_mode(dev);
    match (*dev).video_format as libc::c_uint {
        0 => {
            convert_bayer_to_rgb(
                (*dev).video.raw_buf,
                (*dev).video.proc_buf as *mut uint8_t,
                frame_mode,
            );
        }
        3 => {
            convert_packed_to_16bit(
                (*dev).video.raw_buf,
                (*dev).video.proc_buf as *mut uint16_t,
                10 as libc::c_int,
                frame_mode.width as libc::c_int * frame_mode.height as libc::c_int,
            );
        }
        2 => {
            convert_packed_to_8bit(
                (*dev).video.raw_buf,
                (*dev).video.proc_buf as *mut uint8_t,
                10 as libc::c_int,
                frame_mode.width as libc::c_int * frame_mode.height as libc::c_int,
            );
        }
        5 => {
            convert_uyvy_to_rgb(
                (*dev).video.raw_buf,
                (*dev).video.proc_buf as *mut uint8_t,
                frame_mode,
            );
        }
        1 | 4 | 6 => {}
        _ => {
            fn_log(
                ctx,
                FREENECT_LOG_ERROR,
                b"video_process() was called, but an invalid video_format is set\n\0"
                    as *const u8 as *const libc::c_char,
            );
        }
    }
    if ((*dev).video_cb).is_some() {
        ((*dev).video_cb).unwrap()(dev, (*dev).video.proc_buf, (*dev).video.timestamp);
    }
}
unsafe extern "C" fn freenect_fetch_reg_info(
    mut dev: *mut freenect_device,
) -> libc::c_int {
    let mut ctx: *mut freenect_context = (*dev).parent;
    let mut reply: [libc::c_char; 512] = [0; 512];
    let mut cmd: [uint16_t; 5] = [0; 5];
    let mut mode: freenect_frame_mode = freenect_get_current_video_mode(dev);
    cmd[0 as libc::c_int as usize] = 0x40 as libc::c_int as uint16_t;
    cmd[1 as libc::c_int as usize] = 0 as libc::c_int as uint16_t;
    cmd[2 as libc::c_int as usize] = mode.resolution as uint16_t;
    cmd[3 as libc::c_int as usize] = mode.framerate as uint16_t;
    cmd[4 as libc::c_int as usize] = 0 as libc::c_int as uint16_t;
    let mut res: libc::c_int = 0;
    res = send_cmd(
        dev,
        0x16 as libc::c_int as uint16_t,
        cmd.as_mut_ptr() as *mut libc::c_void,
        10 as libc::c_int as libc::c_uint,
        reply.as_mut_ptr() as *mut libc::c_void,
        118 as libc::c_int,
    );
    if res != 118 as libc::c_int {
        fn_log(
            ctx,
            FREENECT_LOG_ERROR,
            b"freenect_fetch_reg_info: send_cmd read %d bytes (expected 118)\n\0"
                as *const u8 as *const libc::c_char,
            res,
        );
        return -(1 as libc::c_int);
    }
    memcpy(
        &mut (*dev).registration.reg_info as *mut freenect_reg_info as *mut libc::c_void,
        reply.as_mut_ptr().offset(2 as libc::c_int as isize) as *const libc::c_void,
        ::std::mem::size_of::<freenect_reg_info>() as libc::c_ulong,
    );
    (*dev).registration.reg_info.ax = (*dev).registration.reg_info.ax;
    (*dev).registration.reg_info.bx = (*dev).registration.reg_info.bx;
    (*dev).registration.reg_info.cx = (*dev).registration.reg_info.cx;
    (*dev).registration.reg_info.dx = (*dev).registration.reg_info.dx;
    (*dev).registration.reg_info.ay = (*dev).registration.reg_info.ay;
    (*dev).registration.reg_info.by = (*dev).registration.reg_info.by;
    (*dev).registration.reg_info.cy = (*dev).registration.reg_info.cy;
    (*dev).registration.reg_info.dy = (*dev).registration.reg_info.dy;
    (*dev).registration.reg_info.dx_start = (*dev).registration.reg_info.dx_start;
    (*dev).registration.reg_info.dy_start = (*dev).registration.reg_info.dy_start;
    (*dev)
        .registration
        .reg_info
        .dx_beta_start = (*dev).registration.reg_info.dx_beta_start;
    (*dev)
        .registration
        .reg_info
        .dy_beta_start = (*dev).registration.reg_info.dy_beta_start;
    (*dev).registration.reg_info.dx_beta_inc = (*dev).registration.reg_info.dx_beta_inc;
    (*dev).registration.reg_info.dy_beta_inc = (*dev).registration.reg_info.dy_beta_inc;
    (*dev).registration.reg_info.dxdx_start = (*dev).registration.reg_info.dxdx_start;
    (*dev).registration.reg_info.dxdy_start = (*dev).registration.reg_info.dxdy_start;
    (*dev).registration.reg_info.dydx_start = (*dev).registration.reg_info.dydx_start;
    (*dev).registration.reg_info.dydy_start = (*dev).registration.reg_info.dydy_start;
    (*dev)
        .registration
        .reg_info
        .dxdxdx_start = (*dev).registration.reg_info.dxdxdx_start;
    (*dev)
        .registration
        .reg_info
        .dydxdx_start = (*dev).registration.reg_info.dydxdx_start;
    (*dev)
        .registration
        .reg_info
        .dxdxdy_start = (*dev).registration.reg_info.dxdxdy_start;
    (*dev)
        .registration
        .reg_info
        .dydxdy_start = (*dev).registration.reg_info.dydxdy_start;
    (*dev)
        .registration
        .reg_info
        .dydydx_start = (*dev).registration.reg_info.dydydx_start;
    (*dev)
        .registration
        .reg_info
        .dydydy_start = (*dev).registration.reg_info.dydydy_start;
    fn_log(
        ctx,
        FREENECT_LOG_SPEW,
        b"ax:                %d\n\0" as *const u8 as *const libc::c_char,
        (*dev).registration.reg_info.ax,
    );
    fn_log(
        ctx,
        FREENECT_LOG_SPEW,
        b"bx:                %d\n\0" as *const u8 as *const libc::c_char,
        (*dev).registration.reg_info.bx,
    );
    fn_log(
        ctx,
        FREENECT_LOG_SPEW,
        b"cx:                %d\n\0" as *const u8 as *const libc::c_char,
        (*dev).registration.reg_info.cx,
    );
    fn_log(
        ctx,
        FREENECT_LOG_SPEW,
        b"dx:                %d\n\0" as *const u8 as *const libc::c_char,
        (*dev).registration.reg_info.dx,
    );
    fn_log(
        ctx,
        FREENECT_LOG_SPEW,
        b"ay:                %d\n\0" as *const u8 as *const libc::c_char,
        (*dev).registration.reg_info.ay,
    );
    fn_log(
        ctx,
        FREENECT_LOG_SPEW,
        b"by:                %d\n\0" as *const u8 as *const libc::c_char,
        (*dev).registration.reg_info.by,
    );
    fn_log(
        ctx,
        FREENECT_LOG_SPEW,
        b"cy:                %d\n\0" as *const u8 as *const libc::c_char,
        (*dev).registration.reg_info.cy,
    );
    fn_log(
        ctx,
        FREENECT_LOG_SPEW,
        b"dy:                %d\n\0" as *const u8 as *const libc::c_char,
        (*dev).registration.reg_info.dy,
    );
    fn_log(
        ctx,
        FREENECT_LOG_SPEW,
        b"dx_start:          %d\n\0" as *const u8 as *const libc::c_char,
        (*dev).registration.reg_info.dx_start,
    );
    fn_log(
        ctx,
        FREENECT_LOG_SPEW,
        b"dy_start:          %d\n\0" as *const u8 as *const libc::c_char,
        (*dev).registration.reg_info.dy_start,
    );
    fn_log(
        ctx,
        FREENECT_LOG_SPEW,
        b"dx_beta_start:     %d\n\0" as *const u8 as *const libc::c_char,
        (*dev).registration.reg_info.dx_beta_start,
    );
    fn_log(
        ctx,
        FREENECT_LOG_SPEW,
        b"dy_beta_start:     %d\n\0" as *const u8 as *const libc::c_char,
        (*dev).registration.reg_info.dy_beta_start,
    );
    fn_log(
        ctx,
        FREENECT_LOG_SPEW,
        b"dx_beta_inc:       %d\n\0" as *const u8 as *const libc::c_char,
        (*dev).registration.reg_info.dx_beta_inc,
    );
    fn_log(
        ctx,
        FREENECT_LOG_SPEW,
        b"dy_beta_inc:       %d\n\0" as *const u8 as *const libc::c_char,
        (*dev).registration.reg_info.dy_beta_inc,
    );
    fn_log(
        ctx,
        FREENECT_LOG_SPEW,
        b"dxdx_start:        %d\n\0" as *const u8 as *const libc::c_char,
        (*dev).registration.reg_info.dxdx_start,
    );
    fn_log(
        ctx,
        FREENECT_LOG_SPEW,
        b"dxdy_start:        %d\n\0" as *const u8 as *const libc::c_char,
        (*dev).registration.reg_info.dxdy_start,
    );
    fn_log(
        ctx,
        FREENECT_LOG_SPEW,
        b"dydx_start:        %d\n\0" as *const u8 as *const libc::c_char,
        (*dev).registration.reg_info.dydx_start,
    );
    fn_log(
        ctx,
        FREENECT_LOG_SPEW,
        b"dydy_start:        %d\n\0" as *const u8 as *const libc::c_char,
        (*dev).registration.reg_info.dydy_start,
    );
    fn_log(
        ctx,
        FREENECT_LOG_SPEW,
        b"dxdxdx_start:      %d\n\0" as *const u8 as *const libc::c_char,
        (*dev).registration.reg_info.dxdxdx_start,
    );
    fn_log(
        ctx,
        FREENECT_LOG_SPEW,
        b"dydxdx_start:      %d\n\0" as *const u8 as *const libc::c_char,
        (*dev).registration.reg_info.dydxdx_start,
    );
    fn_log(
        ctx,
        FREENECT_LOG_SPEW,
        b"dxdxdy_start:      %d\n\0" as *const u8 as *const libc::c_char,
        (*dev).registration.reg_info.dxdxdy_start,
    );
    fn_log(
        ctx,
        FREENECT_LOG_SPEW,
        b"dydxdy_start:      %d\n\0" as *const u8 as *const libc::c_char,
        (*dev).registration.reg_info.dydxdy_start,
    );
    fn_log(
        ctx,
        FREENECT_LOG_SPEW,
        b"dydydx_start:      %d\n\0" as *const u8 as *const libc::c_char,
        (*dev).registration.reg_info.dydydx_start,
    );
    fn_log(
        ctx,
        FREENECT_LOG_SPEW,
        b"dydydy_start:      %d\n\0" as *const u8 as *const libc::c_char,
        (*dev).registration.reg_info.dydydy_start,
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn freenect_fetch_reg_pad_info(
    mut dev: *mut freenect_device,
) -> libc::c_int {
    let mut ctx: *mut freenect_context = (*dev).parent;
    let mut reply: [libc::c_char; 512] = [0; 512];
    let mut cmd: [uint16_t; 5] = [0; 5];
    let mut mode: freenect_frame_mode = freenect_get_current_video_mode(dev);
    cmd[0 as libc::c_int as usize] = 0x41 as libc::c_int as uint16_t;
    cmd[1 as libc::c_int as usize] = 0 as libc::c_int as uint16_t;
    cmd[2 as libc::c_int as usize] = mode.resolution as uint16_t;
    cmd[3 as libc::c_int as usize] = mode.framerate as uint16_t;
    cmd[4 as libc::c_int as usize] = 0 as libc::c_int as uint16_t;
    let mut res: libc::c_int = 0;
    res = send_cmd(
        dev,
        0x16 as libc::c_int as uint16_t,
        cmd.as_mut_ptr() as *mut libc::c_void,
        10 as libc::c_int as libc::c_uint,
        reply.as_mut_ptr() as *mut libc::c_void,
        8 as libc::c_int,
    );
    if res != 8 as libc::c_int {
        fn_log(
            ctx,
            FREENECT_LOG_ERROR,
            b"freenect_fetch_reg_pad_info: send_cmd read %d bytes (expected 8)\n\0"
                as *const u8 as *const libc::c_char,
            res,
        );
        return -(1 as libc::c_int);
    }
    memcpy(
        &mut (*dev).registration.reg_pad_info as *mut freenect_reg_pad_info
            as *mut libc::c_void,
        reply.as_mut_ptr().offset(2 as libc::c_int as isize) as *const libc::c_void,
        ::std::mem::size_of::<freenect_reg_pad_info>() as libc::c_ulong,
    );
    (*dev)
        .registration
        .reg_pad_info
        .start_lines = (*dev).registration.reg_pad_info.start_lines;
    (*dev)
        .registration
        .reg_pad_info
        .end_lines = (*dev).registration.reg_pad_info.end_lines;
    (*dev)
        .registration
        .reg_pad_info
        .cropping_lines = (*dev).registration.reg_pad_info.cropping_lines;
    fn_log(
        ctx,
        FREENECT_LOG_SPEW,
        b"start_lines:    %u\n\0" as *const u8 as *const libc::c_char,
        (*dev).registration.reg_pad_info.start_lines as libc::c_int,
    );
    fn_log(
        ctx,
        FREENECT_LOG_SPEW,
        b"end_lines:      %u\n\0" as *const u8 as *const libc::c_char,
        (*dev).registration.reg_pad_info.end_lines as libc::c_int,
    );
    fn_log(
        ctx,
        FREENECT_LOG_SPEW,
        b"cropping_lines: %u\n\0" as *const u8 as *const libc::c_char,
        (*dev).registration.reg_pad_info.cropping_lines as libc::c_int,
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn freenect_fetch_reg_const_shift(
    mut dev: *mut freenect_device,
) -> libc::c_int {
    let mut ctx: *mut freenect_context = (*dev).parent;
    let mut reply: [libc::c_char; 512] = [0; 512];
    let mut cmd: [uint16_t; 5] = [0; 5];
    let mut mode: freenect_frame_mode = freenect_get_current_video_mode(dev);
    cmd[0 as libc::c_int as usize] = 0 as libc::c_int as uint16_t;
    cmd[1 as libc::c_int as usize] = 0 as libc::c_int as uint16_t;
    cmd[2 as libc::c_int as usize] = mode.resolution as uint16_t;
    cmd[3 as libc::c_int as usize] = mode.framerate as uint16_t;
    cmd[4 as libc::c_int as usize] = 0 as libc::c_int as uint16_t;
    let mut res: libc::c_int = 0;
    res = send_cmd(
        dev,
        0x16 as libc::c_int as uint16_t,
        cmd.as_mut_ptr() as *mut libc::c_void,
        10 as libc::c_int as libc::c_uint,
        reply.as_mut_ptr() as *mut libc::c_void,
        4 as libc::c_int,
    );
    if res != 4 as libc::c_int {
        fn_log(
            ctx,
            FREENECT_LOG_ERROR,
            b"freenect_fetch_reg_const_shift: send_cmd read %d bytes (expected 8)\n\0"
                as *const u8 as *const libc::c_char,
            res,
        );
        return -(1 as libc::c_int);
    }
    let mut shift: uint16_t = 0;
    memcpy(
        &mut shift as *mut uint16_t as *mut libc::c_void,
        reply.as_mut_ptr().offset(2 as libc::c_int as isize) as *const libc::c_void,
        ::std::mem::size_of::<uint16_t>() as libc::c_ulong,
    );
    shift = shift;
    (*dev).registration.const_shift = shift as libc::c_double;
    fn_log(
        ctx,
        FREENECT_LOG_SPEW,
        b"const_shift: %f\n\0" as *const u8 as *const libc::c_char,
        (*dev).registration.const_shift,
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn freenect_fetch_zero_plane_info(
    mut dev: *mut freenect_device,
) -> libc::c_int {
    let mut ctx: *mut freenect_context = (*dev).parent;
    let mut reply: [libc::c_char; 512] = [0; 512];
    let mut cmd: [uint16_t; 5] = [0 as libc::c_int as uint16_t, 0, 0, 0, 0];
    let mut res: libc::c_int = 0;
    res = send_cmd(
        dev,
        0x4 as libc::c_int as uint16_t,
        cmd.as_mut_ptr() as *mut libc::c_void,
        10 as libc::c_int as libc::c_uint,
        reply.as_mut_ptr() as *mut libc::c_void,
        (*ctx).zero_plane_res,
    );
    if res != (*ctx).zero_plane_res {
        fn_log(
            ctx,
            FREENECT_LOG_ERROR,
            b"freenect_fetch_zero_plane_info: send_cmd read %d bytes (expected %d)\n\0"
                as *const u8 as *const libc::c_char,
            res,
            (*ctx).zero_plane_res,
        );
        return -(1 as libc::c_int);
    }
    memcpy(
        &mut (*dev).registration.zero_plane_info as *mut freenect_zero_plane_info
            as *mut libc::c_void,
        reply.as_mut_ptr().offset(94 as libc::c_int as isize) as *const libc::c_void,
        ::std::mem::size_of::<freenect_zero_plane_info>() as libc::c_ulong,
    );
    let mut conversion_union: C2RustUnnamed_0 = C2RustUnnamed_0 { ui: 0 };
    conversion_union.f = (*dev).registration.zero_plane_info.dcmos_emitter_dist;
    conversion_union.ui = conversion_union.ui;
    (*dev).registration.zero_plane_info.dcmos_emitter_dist = conversion_union.f;
    conversion_union.f = (*dev).registration.zero_plane_info.dcmos_rcmos_dist;
    conversion_union.ui = conversion_union.ui;
    (*dev).registration.zero_plane_info.dcmos_rcmos_dist = conversion_union.f;
    conversion_union.f = (*dev).registration.zero_plane_info.reference_distance;
    conversion_union.ui = conversion_union.ui;
    (*dev).registration.zero_plane_info.reference_distance = conversion_union.f;
    conversion_union.f = (*dev).registration.zero_plane_info.reference_pixel_size;
    conversion_union.ui = conversion_union.ui;
    (*dev).registration.zero_plane_info.reference_pixel_size = conversion_union.f;
    fn_log(
        ctx,
        FREENECT_LOG_SPEW,
        b"dcmos_emitter_distance: %f\n\0" as *const u8 as *const libc::c_char,
        (*dev).registration.zero_plane_info.dcmos_emitter_dist as libc::c_double,
    );
    fn_log(
        ctx,
        FREENECT_LOG_SPEW,
        b"dcmos_rcmos_distance:   %f\n\0" as *const u8 as *const libc::c_char,
        (*dev).registration.zero_plane_info.dcmos_rcmos_dist as libc::c_double,
    );
    fn_log(
        ctx,
        FREENECT_LOG_SPEW,
        b"reference_distance:     %f\n\0" as *const u8 as *const libc::c_char,
        (*dev).registration.zero_plane_info.reference_distance as libc::c_double,
    );
    fn_log(
        ctx,
        FREENECT_LOG_SPEW,
        b"reference_pixel_size:   %f\n\0" as *const u8 as *const libc::c_char,
        (*dev).registration.zero_plane_info.reference_pixel_size as libc::c_double,
    );
    (*dev).registration.zero_plane_info.dcmos_rcmos_dist = 2.4f32;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn freenect_start_depth(
    mut dev: *mut freenect_device,
) -> libc::c_int {
    let mut ctx: *mut freenect_context = (*dev).parent;
    if (*dev).depth.running != 0 {
        return -(1 as libc::c_int);
    }
    (*dev).depth.pkt_size = 1760 as libc::c_int - 12 as libc::c_int;
    (*dev).depth.flag = 0x70 as libc::c_int as uint8_t;
    (*dev).depth.variable_length = 0 as libc::c_int;
    let mut current_block_10: u64;
    match (*dev).depth_format as libc::c_uint {
        4 | 5 => {
            freenect_init_registration(dev);
            current_block_10 = 18344302610087699094;
        }
        0 => {
            current_block_10 = 18344302610087699094;
        }
        1 => {
            stream_init(
                ctx,
                &mut (*dev).depth,
                (freenect_find_depth_mode(
                    (*dev).depth_resolution,
                    FREENECT_DEPTH_10BIT_PACKED,
                ))
                    .bytes,
                (freenect_find_depth_mode((*dev).depth_resolution, FREENECT_DEPTH_10BIT))
                    .bytes,
            );
            current_block_10 = 12209867499936983673;
        }
        2 | 3 => {
            stream_init(
                ctx,
                &mut (*dev).depth,
                0 as libc::c_int,
                (freenect_find_depth_mode((*dev).depth_resolution, (*dev).depth_format))
                    .bytes,
            );
            current_block_10 = 12209867499936983673;
        }
        _ => {
            fn_log(
                ctx,
                FREENECT_LOG_ERROR,
                b"freenect_start_depth() called with invalid depth format %d\n\0"
                    as *const u8 as *const libc::c_char,
                (*dev).depth_format as libc::c_uint,
            );
            return -(1 as libc::c_int);
        }
    }
    match current_block_10 {
        18344302610087699094 => {
            stream_init(
                ctx,
                &mut (*dev).depth,
                (freenect_find_depth_mode(
                    (*dev).depth_resolution,
                    FREENECT_DEPTH_11BIT_PACKED,
                ))
                    .bytes,
                (freenect_find_depth_mode((*dev).depth_resolution, FREENECT_DEPTH_11BIT))
                    .bytes,
            );
        }
        _ => {}
    }
    let depth_endpoint: libc::c_uchar = 0x82 as libc::c_int as libc::c_uchar;
    let mut packet_size: libc::c_int = fnusb_get_max_iso_packet_size(
        &mut (*dev).usb_cam,
        depth_endpoint,
        1920 as libc::c_int,
    );
    fn_log(
        ctx,
        FREENECT_LOG_INFO,
        b"[Stream 70] Negotiated packet size %d\n\0" as *const u8 as *const libc::c_char,
        packet_size,
    );
    let mut res: libc::c_int = fnusb_start_iso(
        &mut (*dev).usb_cam,
        &mut (*dev).depth_isoc,
        Some(
            depth_process
                as unsafe extern "C" fn(
                    *mut freenect_device,
                    *mut uint8_t,
                    libc::c_int,
                ) -> (),
        ),
        depth_endpoint,
        16 as libc::c_int,
        16 as libc::c_int,
        packet_size,
    );
    if res < 0 as libc::c_int {
        return res;
    }
    write_register(dev, 0x105 as libc::c_int as uint16_t, 0 as libc::c_int as uint16_t);
    write_register(dev, 0x6 as libc::c_int as uint16_t, 0 as libc::c_int as uint16_t);
    match (*dev).depth_format as libc::c_uint {
        0 | 2 | 4 | 5 => {
            write_register(
                dev,
                0x12 as libc::c_int as uint16_t,
                0x3 as libc::c_int as uint16_t,
            );
        }
        1 | 3 => {
            write_register(
                dev,
                0x12 as libc::c_int as uint16_t,
                0x2 as libc::c_int as uint16_t,
            );
        }
        2147483647 | _ => {}
    }
    write_register(dev, 0x13 as libc::c_int as uint16_t, 0x1 as libc::c_int as uint16_t);
    write_register(
        dev,
        0x14 as libc::c_int as uint16_t,
        0x1e as libc::c_int as uint16_t,
    );
    write_register(dev, 0x6 as libc::c_int as uint16_t, 0x2 as libc::c_int as uint16_t);
    write_register(dev, 0x17 as libc::c_int as uint16_t, 0 as libc::c_int as uint16_t);
    (*dev).depth.running = 1 as libc::c_int;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn freenect_start_video(
    mut dev: *mut freenect_device,
) -> libc::c_int {
    let mut ctx: *mut freenect_context = (*dev).parent;
    if (*dev).video.running != 0 {
        return -(1 as libc::c_int);
    }
    (*dev).video.pkt_size = 1920 as libc::c_int - 12 as libc::c_int;
    (*dev).video.flag = 0x80 as libc::c_int as uint8_t;
    (*dev).video.variable_length = 0 as libc::c_int;
    let mut mode_reg: uint16_t = 0;
    let mut mode_value: uint16_t = 0;
    let mut res_reg: uint16_t = 0;
    let mut res_value: uint16_t = 0;
    let mut fps_reg: uint16_t = 0;
    let mut fps_value: uint16_t = 0;
    let mut hflip_reg: uint16_t = 0;
    match (*dev).video_format as libc::c_uint {
        0 | 1 => {
            if (*dev).video_resolution as libc::c_uint
                == FREENECT_RESOLUTION_HIGH as libc::c_int as libc::c_uint
            {
                mode_value = 0 as libc::c_int as uint16_t;
                res_value = 0x2 as libc::c_int as uint16_t;
                fps_value = 0xf as libc::c_int as uint16_t;
            } else if (*dev).video_resolution as libc::c_uint
                == FREENECT_RESOLUTION_MEDIUM as libc::c_int as libc::c_uint
            {
                mode_value = 0 as libc::c_int as uint16_t;
                res_value = 0x1 as libc::c_int as uint16_t;
                fps_value = 0x1e as libc::c_int as uint16_t;
            } else {
                fn_log(
                    ctx,
                    FREENECT_LOG_ERROR,
                    b"freenect_start_video(): called with invalid format/resolution combination\n\0"
                        as *const u8 as *const libc::c_char,
                );
                return -(1 as libc::c_int);
            }
            mode_reg = 0xc as libc::c_int as uint16_t;
            res_reg = 0xd as libc::c_int as uint16_t;
            fps_reg = 0xe as libc::c_int as uint16_t;
            hflip_reg = 0x47 as libc::c_int as uint16_t;
        }
        2 | 3 | 4 => {
            if (*dev).video_resolution as libc::c_uint
                == FREENECT_RESOLUTION_HIGH as libc::c_int as libc::c_uint
            {
                if (*dev).depth.running != 0 {
                    fn_log(
                        ctx,
                        FREENECT_LOG_ERROR,
                        b"freenect_start_video(): cannot stream high-resolution IR at same time as depth stream\n\0"
                            as *const u8 as *const libc::c_char,
                    );
                    return -(1 as libc::c_int);
                }
                write_register(
                    dev,
                    0x13 as libc::c_int as uint16_t,
                    0x1 as libc::c_int as uint16_t,
                );
                write_register(
                    dev,
                    0x14 as libc::c_int as uint16_t,
                    0x1e as libc::c_int as uint16_t,
                );
                write_register(
                    dev,
                    0x6 as libc::c_int as uint16_t,
                    0x2 as libc::c_int as uint16_t,
                );
                write_register(
                    dev,
                    0x6 as libc::c_int as uint16_t,
                    0 as libc::c_int as uint16_t,
                );
                mode_value = 0 as libc::c_int as uint16_t;
                res_value = 0x2 as libc::c_int as uint16_t;
                fps_value = 0xf as libc::c_int as uint16_t;
            } else if (*dev).video_resolution as libc::c_uint
                == FREENECT_RESOLUTION_MEDIUM as libc::c_int as libc::c_uint
            {
                mode_value = 0 as libc::c_int as uint16_t;
                res_value = 0x1 as libc::c_int as uint16_t;
                fps_value = 0x1e as libc::c_int as uint16_t;
            } else {
                fn_log(
                    ctx,
                    FREENECT_LOG_ERROR,
                    b"freenect_start_video(): called with invalid format/resolution combination\n\0"
                        as *const u8 as *const libc::c_char,
                );
                return -(1 as libc::c_int);
            }
            mode_reg = 0x19 as libc::c_int as uint16_t;
            res_reg = 0x1a as libc::c_int as uint16_t;
            fps_reg = 0x1b as libc::c_int as uint16_t;
            hflip_reg = 0x48 as libc::c_int as uint16_t;
        }
        5 | 6 => {
            if (*dev).video_resolution as libc::c_uint
                == FREENECT_RESOLUTION_MEDIUM as libc::c_int as libc::c_uint
            {
                mode_value = 0x5 as libc::c_int as uint16_t;
                res_value = 0x1 as libc::c_int as uint16_t;
                fps_value = 0xf as libc::c_int as uint16_t;
            } else {
                fn_log(
                    ctx,
                    FREENECT_LOG_ERROR,
                    b"freenect_start_video(): called with invalid format/resolution combination\n\0"
                        as *const u8 as *const libc::c_char,
                );
                return -(1 as libc::c_int);
            }
            mode_reg = 0xc as libc::c_int as uint16_t;
            res_reg = 0xd as libc::c_int as uint16_t;
            fps_reg = 0xe as libc::c_int as uint16_t;
            hflip_reg = 0x47 as libc::c_int as uint16_t;
        }
        _ => {
            fn_log(
                ctx,
                FREENECT_LOG_ERROR,
                b"freenect_start_video(): called with invalid video format %d\n\0"
                    as *const u8 as *const libc::c_char,
                (*dev).video_format as libc::c_uint,
            );
            return -(1 as libc::c_int);
        }
    }
    let mut frame_mode: freenect_frame_mode = freenect_get_current_video_mode(dev);
    match (*dev).video_format as libc::c_uint {
        0 => {
            stream_init(
                ctx,
                &mut (*dev).video,
                (freenect_find_video_mode((*dev).video_resolution, FREENECT_VIDEO_BAYER))
                    .bytes,
                frame_mode.bytes,
            );
        }
        1 => {
            stream_init(ctx, &mut (*dev).video, 0 as libc::c_int, frame_mode.bytes);
        }
        2 => {
            stream_init(
                ctx,
                &mut (*dev).video,
                (freenect_find_video_mode(
                    (*dev).video_resolution,
                    FREENECT_VIDEO_IR_10BIT_PACKED,
                ))
                    .bytes,
                frame_mode.bytes,
            );
        }
        3 => {
            stream_init(
                ctx,
                &mut (*dev).video,
                (freenect_find_video_mode(
                    (*dev).video_resolution,
                    FREENECT_VIDEO_IR_10BIT_PACKED,
                ))
                    .bytes,
                frame_mode.bytes,
            );
        }
        4 => {
            stream_init(ctx, &mut (*dev).video, 0 as libc::c_int, frame_mode.bytes);
        }
        5 => {
            stream_init(
                ctx,
                &mut (*dev).video,
                (freenect_find_video_mode(
                    (*dev).video_resolution,
                    FREENECT_VIDEO_YUV_RAW,
                ))
                    .bytes,
                frame_mode.bytes,
            );
        }
        6 => {
            stream_init(ctx, &mut (*dev).video, 0 as libc::c_int, frame_mode.bytes);
        }
        2147483647 | _ => {}
    }
    let video_endpoint: libc::c_uchar = 0x81 as libc::c_int as libc::c_uchar;
    let mut packet_size: libc::c_int = fnusb_get_max_iso_packet_size(
        &mut (*dev).usb_cam,
        video_endpoint,
        1920 as libc::c_int,
    );
    fn_log(
        ctx,
        FREENECT_LOG_INFO,
        b"[Stream 80] Negotiated packet size %d\n\0" as *const u8 as *const libc::c_char,
        packet_size,
    );
    let mut res: libc::c_int = fnusb_start_iso(
        &mut (*dev).usb_cam,
        &mut (*dev).video_isoc,
        Some(
            video_process
                as unsafe extern "C" fn(
                    *mut freenect_device,
                    *mut uint8_t,
                    libc::c_int,
                ) -> (),
        ),
        video_endpoint,
        16 as libc::c_int,
        16 as libc::c_int,
        packet_size,
    );
    if res < 0 as libc::c_int {
        return res;
    }
    write_register(dev, mode_reg, mode_value);
    write_register(dev, res_reg, res_value);
    write_register(dev, fps_reg, fps_value);
    match (*dev).video_format as libc::c_uint {
        0 | 1 | 5 | 6 => {
            write_register(
                dev,
                0x5 as libc::c_int as uint16_t,
                0x1 as libc::c_int as uint16_t,
            );
        }
        2 | 3 | 4 => {
            write_register(
                dev,
                0x105 as libc::c_int as uint16_t,
                0 as libc::c_int as uint16_t,
            );
            write_register(
                dev,
                0x5 as libc::c_int as uint16_t,
                0x3 as libc::c_int as uint16_t,
            );
        }
        2147483647 | _ => {}
    }
    write_register(dev, hflip_reg, 0 as libc::c_int as uint16_t);
    (*dev).video.running = 1 as libc::c_int;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn freenect_stop_depth(
    mut dev: *mut freenect_device,
) -> libc::c_int {
    let mut ctx: *mut freenect_context = (*dev).parent;
    let mut res: libc::c_int = 0;
    if (*dev).depth.running == 0 {
        return -(1 as libc::c_int);
    }
    (*dev).depth.running = 0 as libc::c_int;
    write_register(dev, 0x6 as libc::c_int as uint16_t, 0 as libc::c_int as uint16_t);
    res = fnusb_stop_iso(&mut (*dev).usb_cam, &mut (*dev).depth_isoc);
    if res < 0 as libc::c_int {
        fn_log(
            ctx,
            FREENECT_LOG_ERROR,
            b"Failed to stop depth isochronous stream: %d\n\0" as *const u8
                as *const libc::c_char,
            res,
        );
        return res;
    }
    freenect_destroy_registration(&mut (*dev).registration);
    stream_freebufs(ctx, &mut (*dev).depth);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn freenect_stop_video(
    mut dev: *mut freenect_device,
) -> libc::c_int {
    let mut ctx: *mut freenect_context = (*dev).parent;
    let mut res: libc::c_int = 0;
    if (*dev).video.running == 0 {
        return -(1 as libc::c_int);
    }
    (*dev).video.running = 0 as libc::c_int;
    write_register(dev, 0x5 as libc::c_int as uint16_t, 0 as libc::c_int as uint16_t);
    res = fnusb_stop_iso(&mut (*dev).usb_cam, &mut (*dev).video_isoc);
    if res < 0 as libc::c_int {
        fn_log(
            ctx,
            FREENECT_LOG_ERROR,
            b"Failed to stop RGB isochronous stream: %d\n\0" as *const u8
                as *const libc::c_char,
            res,
        );
        return res;
    }
    stream_freebufs(ctx, &mut (*dev).video);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn freenect_set_depth_callback(
    mut dev: *mut freenect_device,
    mut cb: freenect_depth_cb,
) {
    (*dev).depth_cb = cb;
}
pub unsafe extern "C" fn freenect_set_video_callback(
    mut dev: *mut freenect_device,
    mut cb: freenect_video_cb,
) {
    (*dev).video_cb = cb;
}
pub unsafe extern "C" fn freenect_set_depth_chunk_callback(
    mut dev: *mut freenect_device,
    mut cb: freenect_chunk_cb,
) {
    (*dev).depth_chunk_cb = cb;
}
pub unsafe extern "C" fn freenect_set_video_chunk_callback(
    mut dev: *mut freenect_device,
    mut cb: freenect_chunk_cb,
) {
    (*dev).video_chunk_cb = cb;
}
pub unsafe extern "C" fn freenect_get_video_mode_count() -> libc::c_int {
    return 12 as libc::c_int;
}
pub unsafe extern "C" fn freenect_get_video_mode(
    mut mode_num: libc::c_int,
) -> freenect_frame_mode {
    if mode_num >= 0 as libc::c_int && mode_num < 12 as libc::c_int {
        return supported_video_modes[mode_num as usize];
    }
    let mut retval: freenect_frame_mode = freenect_frame_mode {
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
    retval.is_valid = 0 as libc::c_int as int8_t;
    return retval;
}
pub unsafe extern "C" fn freenect_get_current_video_mode(
    mut dev: *mut freenect_device,
) -> freenect_frame_mode {
    return freenect_find_video_mode((*dev).video_resolution, (*dev).video_format);
}
pub unsafe extern "C" fn freenect_find_video_mode(
    mut res: freenect_resolution,
    mut fmt: freenect_video_format,
) -> freenect_frame_mode {
    let mut unique_id: uint32_t = (res as libc::c_uint
        & 0xff as libc::c_int as libc::c_uint) << 8 as libc::c_int
        | fmt as libc::c_uint & 0xff as libc::c_int as libc::c_uint;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 12 as libc::c_int {
        if supported_video_modes[i as usize].reserved == unique_id {
            return supported_video_modes[i as usize];
        }
        i += 1;
        i;
    }
    let mut retval: freenect_frame_mode = freenect_frame_mode {
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
    retval.is_valid = 0 as libc::c_int as int8_t;
    return retval;
}
pub unsafe extern "C" fn freenect_set_video_mode(
    mut dev: *mut freenect_device,
    mode: freenect_frame_mode,
) -> libc::c_int {
    let mut ctx: *mut freenect_context = (*dev).parent;
    if (*dev).video.running != 0 {
        fn_log(
            ctx,
            FREENECT_LOG_ERROR,
            b"Tried to set video mode while stream is active\n\0" as *const u8
                as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    let mut found: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 12 as libc::c_int {
        if supported_video_modes[i as usize].reserved == mode.reserved {
            found = 1 as libc::c_int;
            break;
        } else {
            i += 1;
            i;
        }
    }
    if found == 0 {
        fn_log(
            ctx,
            FREENECT_LOG_ERROR,
            b"freenect_set_video_mode: freenect_frame_mode provided is invalid\n\0"
                as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    let mut res: freenect_resolution = (mode.reserved >> 8 as libc::c_int
        & 0xff as libc::c_int as libc::c_uint) as freenect_resolution;
    let mut fmt: freenect_video_format = (mode.reserved
        & 0xff as libc::c_int as libc::c_uint) as freenect_video_format;
    (*dev).video_format = fmt;
    (*dev).video_resolution = res;
    freenect_fetch_reg_info(dev);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn freenect_get_depth_mode_count() -> libc::c_int {
    return 6 as libc::c_int;
}
pub unsafe extern "C" fn freenect_get_depth_mode(
    mut mode_num: libc::c_int,
) -> freenect_frame_mode {
    if mode_num >= 0 as libc::c_int && mode_num < 6 as libc::c_int {
        return supported_depth_modes[mode_num as usize];
    }
    let mut retval: freenect_frame_mode = freenect_frame_mode {
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
    retval.is_valid = 0 as libc::c_int as int8_t;
    return retval;
}
pub unsafe extern "C" fn freenect_get_current_depth_mode(
    mut dev: *mut freenect_device,
) -> freenect_frame_mode {
    return freenect_find_depth_mode((*dev).depth_resolution, (*dev).depth_format);
}
pub unsafe extern "C" fn freenect_find_depth_mode(
    mut res: freenect_resolution,
    mut fmt: freenect_depth_format,
) -> freenect_frame_mode {
    let mut unique_id: uint32_t = (res as libc::c_uint
        & 0xff as libc::c_int as libc::c_uint) << 8 as libc::c_int
        | fmt as libc::c_uint & 0xff as libc::c_int as libc::c_uint;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 6 as libc::c_int {
        if supported_depth_modes[i as usize].reserved == unique_id {
            return supported_depth_modes[i as usize];
        }
        i += 1;
        i;
    }
    let mut retval: freenect_frame_mode = freenect_frame_mode {
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
    retval.is_valid = 0 as libc::c_int as int8_t;
    return retval;
}
pub unsafe extern "C" fn freenect_set_depth_mode(
    mut dev: *mut freenect_device,
    mode: freenect_frame_mode,
) -> libc::c_int {
    let mut ctx: *mut freenect_context = (*dev).parent;
    if (*dev).depth.running != 0 {
        fn_log(
            ctx,
            FREENECT_LOG_ERROR,
            b"Tried to set depth mode while stream is active\n\0" as *const u8
                as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    let mut found: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 6 as libc::c_int {
        if supported_depth_modes[i as usize].reserved == mode.reserved {
            found = 1 as libc::c_int;
            break;
        } else {
            i += 1;
            i;
        }
    }
    if found == 0 {
        fn_log(
            ctx,
            FREENECT_LOG_ERROR,
            b"freenect_set_depth_mode: freenect_frame_mode provided is invalid\n\0"
                as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    let mut res: freenect_resolution = (mode.reserved >> 8 as libc::c_int
        & 0xff as libc::c_int as libc::c_uint) as freenect_resolution;
    let mut fmt: freenect_depth_format = (mode.reserved
        & 0xff as libc::c_int as libc::c_uint) as freenect_depth_format;
    (*dev).depth_format = fmt;
    (*dev).depth_resolution = res;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn freenect_set_depth_buffer(
    mut dev: *mut freenect_device,
    mut buf: *mut libc::c_void,
) -> libc::c_int {
    return stream_setbuf((*dev).parent, &mut (*dev).depth, buf);
}
pub unsafe extern "C" fn freenect_set_video_buffer(
    mut dev: *mut freenect_device,
    mut buf: *mut libc::c_void,
) -> libc::c_int {
    return stream_setbuf((*dev).parent, &mut (*dev).video, buf);
}
pub unsafe extern "C" fn freenect_camera_init(
    mut dev: *mut freenect_device,
) -> libc::c_int {
    let mut ctx: *mut freenect_context = (*dev).parent;
    let mut res: libc::c_int = 0;
    res = freenect_fetch_reg_pad_info(dev);
    if res < 0 as libc::c_int {
        fn_log(
            ctx,
            FREENECT_LOG_ERROR,
            b"freenect_camera_init(): Failed to fetch registration pad info for device\n\0"
                as *const u8 as *const libc::c_char,
        );
        return res;
    }
    res = freenect_fetch_zero_plane_info(dev);
    if res < 0 as libc::c_int {
        fn_log(
            ctx,
            FREENECT_LOG_ERROR,
            b"freenect_camera_init(): Failed to fetch zero plane info for device\n\0"
                as *const u8 as *const libc::c_char,
        );
        return res;
    }
    res = freenect_set_video_mode(
        dev,
        freenect_find_video_mode(FREENECT_RESOLUTION_MEDIUM, FREENECT_VIDEO_RGB),
    );
    res = freenect_set_depth_mode(
        dev,
        freenect_find_depth_mode(FREENECT_RESOLUTION_MEDIUM, FREENECT_DEPTH_11BIT),
    );
    res = freenect_fetch_reg_const_shift(dev);
    if res < 0 as libc::c_int {
        fn_log(
            ctx,
            FREENECT_LOG_ERROR,
            b"freenect_camera_init(): Failed to fetch const shift for device\n\0"
                as *const u8 as *const libc::c_char,
        );
        return res;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn freenect_camera_teardown(
    mut dev: *mut freenect_device,
) -> libc::c_int {
    let mut ctx: *mut freenect_context = (*dev).parent;
    let mut res: libc::c_int = 0 as libc::c_int;
    if (*dev).depth.running != 0 {
        res = freenect_stop_depth(dev);
        if res < 0 as libc::c_int {
            fn_log(
                ctx,
                FREENECT_LOG_ERROR,
                b"freenect_camera_teardown(): Failed to stop depth camera\n\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        return res;
    }
    if (*dev).video.running != 0 {
        res = freenect_stop_video(dev);
        if res < 0 as libc::c_int {
            fn_log(
                ctx,
                FREENECT_LOG_ERROR,
                b"freenect_camera_teardown(): Failed to stop video camera\n\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        return res;
    }
    freenect_destroy_registration(&mut (*dev).registration);
    return 0 as libc::c_int;
}
