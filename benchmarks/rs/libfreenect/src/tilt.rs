use ::libc;
extern "C" {
    pub type libusb_device_handle;
    pub type libusb_context;
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
    fn libusb_error_name(errcode: libc::c_int) -> *const libc::c_char;
    fn fnusb_control(
        dev: *mut fnusb_dev,
        bmRequestType: uint8_t,
        bRequest: uint8_t,
        wValue: uint16_t,
        wIndex: uint16_t,
        data: *mut uint8_t,
        wLength: uint16_t,
    ) -> libc::c_int;
    fn fn_log(
        ctx: *mut freenect_context,
        level: freenect_loglevel,
        fmt: *const libc::c_char,
        _: ...
    );
    fn libusb_bulk_transfer(
        dev_handle: *mut libusb_device_handle,
        endpoint: libc::c_uchar,
        data: *mut libc::c_uchar,
        length: libc::c_int,
        actual_length: *mut libc::c_int,
        timeout: libc::c_uint,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fn_alt_motor_reply {
    pub magic: uint32_t,
    pub tag: uint32_t,
    pub status: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub x: int32_t,
    pub y: int32_t,
    pub z: int32_t,
    pub tilt: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fn_alt_motor_command {
    pub magic: uint32_t,
    pub tag: uint32_t,
    pub arg1: uint32_t,
    pub cmd: uint32_t,
    pub arg2: uint32_t,
}
pub const LED_ALT_BLINK_GREEN: C2RustUnnamed_0 = 2;
pub const LED_ALT_OFF: C2RustUnnamed_0 = 1;
pub const LED_ALT_SOLID_GREEN: C2RustUnnamed_0 = 3;
pub const LED_ALT_SOLID_RED: C2RustUnnamed_0 = 4;
pub type C2RustUnnamed_0 = libc::c_uint;
static mut tag_seq: libc::c_int = 0 as libc::c_int;
static mut tag_next_ack: libc::c_int = 0 as libc::c_int;
pub unsafe extern "C" fn get_reply(
    mut dev: *mut libusb_device_handle,
    mut ctx: *mut freenect_context,
) -> libc::c_int {
    let mut buffer: [libc::c_uchar; 512] = [0; 512];
    memset(
        buffer.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        512 as libc::c_int as libc::c_ulong,
    );
    let mut transferred: libc::c_int = 0 as libc::c_int;
    let mut res: libc::c_int = 0 as libc::c_int;
    res = libusb_bulk_transfer(
        dev,
        0x81 as libc::c_int as libc::c_uchar,
        buffer.as_mut_ptr(),
        512 as libc::c_int,
        &mut transferred,
        100 as libc::c_int as libc::c_uint,
    );
    if res != 0 as libc::c_int {
        fn_log(
            ctx,
            FREENECT_LOG_ERROR,
            b"get_reply(): libusb_bulk_transfer failed: %s (transferred = %d)\n\0"
                as *const u8 as *const libc::c_char,
            libusb_error_name(res),
            transferred,
        );
    } else if transferred != 12 as libc::c_int {
        fn_log(
            ctx,
            FREENECT_LOG_ERROR,
            b"get_reply(): weird - got %d bytes (expected 12)\n\0" as *const u8
                as *const libc::c_char,
            transferred,
        );
    } else {
        let mut reply: fn_alt_motor_reply = fn_alt_motor_reply {
            magic: 0,
            tag: 0,
            status: 0,
        };
        memcpy(
            &mut reply as *mut fn_alt_motor_reply as *mut libc::c_void,
            buffer.as_mut_ptr() as *const libc::c_void,
            ::std::mem::size_of::<fn_alt_motor_reply>() as libc::c_ulong,
        );
        if reply.magic != 0xa6fe000 as libc::c_int as libc::c_uint {
            fn_log(
                ctx,
                FREENECT_LOG_ERROR,
                b"Bad magic: %08X (expected 0A6FE000\n\0" as *const u8
                    as *const libc::c_char,
                reply.magic,
            );
            res = -(1 as libc::c_int);
        }
        if reply.status != 0 as libc::c_int as libc::c_uint {
            fn_log(
                ctx,
                FREENECT_LOG_ERROR,
                b"reply status != 0: failure?\n\0" as *const u8 as *const libc::c_char,
            );
            res = -(1 as libc::c_int);
        }
        tag_next_ack += 1;
        tag_next_ack;
    }
    return res;
}
pub unsafe extern "C" fn freenect_get_tilt_state(
    mut dev: *mut freenect_device,
) -> *mut freenect_raw_tilt_state {
    return &mut (*dev).raw_state;
}
pub unsafe extern "C" fn update_tilt_state_alt(
    mut dev: *mut freenect_device,
) -> libc::c_int {
    let mut ctx: *mut freenect_context = (*dev).parent;
    if ((*dev).usb_audio.dev).is_null() {
        fn_log(
            ctx,
            FREENECT_LOG_WARNING,
            b"Motor control failed: audio device missing\0" as *const u8
                as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    let mut transferred: libc::c_int = 0 as libc::c_int;
    let mut res: libc::c_int = 0 as libc::c_int;
    let mut cmd: fn_alt_motor_command = fn_alt_motor_command {
        magic: 0,
        tag: 0,
        arg1: 0,
        cmd: 0,
        arg2: 0,
    };
    cmd.magic = 0x6022009 as libc::c_int as uint32_t;
    let fresh0 = tag_seq;
    tag_seq = tag_seq + 1;
    cmd.tag = fresh0 as uint32_t;
    cmd.arg1 = 0x68 as libc::c_int as uint32_t;
    cmd.cmd = 0x8032 as libc::c_int as uint32_t;
    let mut buffer: [libc::c_uchar; 256] = [0; 256];
    memcpy(
        buffer.as_mut_ptr() as *mut libc::c_void,
        &mut cmd as *mut fn_alt_motor_command as *const libc::c_void,
        16 as libc::c_int as libc::c_ulong,
    );
    res = libusb_bulk_transfer(
        (*dev).usb_audio.dev,
        0x1 as libc::c_int as libc::c_uchar,
        buffer.as_mut_ptr(),
        16 as libc::c_int,
        &mut transferred,
        250 as libc::c_int as libc::c_uint,
    );
    if res != 0 as libc::c_int {
        return res;
    }
    res = libusb_bulk_transfer(
        (*dev).usb_audio.dev,
        0x81 as libc::c_int as libc::c_uchar,
        buffer.as_mut_ptr(),
        256 as libc::c_int,
        &mut transferred,
        250 as libc::c_int as libc::c_uint,
    );
    if res != 0 as libc::c_int {
        return res;
    }
    let mut accel_and_tilt: C2RustUnnamed = C2RustUnnamed {
        x: 0,
        y: 0,
        z: 0,
        tilt: 0,
    };
    memcpy(
        &mut accel_and_tilt as *mut C2RustUnnamed as *mut libc::c_void,
        buffer.as_mut_ptr().offset(16 as libc::c_int as isize) as *const libc::c_void,
        ::std::mem::size_of::<C2RustUnnamed>() as libc::c_ulong,
    );
    fn_log(
        ctx,
        FREENECT_LOG_SPEW,
        b"Accelerometer state: X == %d \t Y == %d \t Z == %d \t Tilt == %d\n\0"
            as *const u8 as *const libc::c_char,
        accel_and_tilt.x,
        accel_and_tilt.y,
        accel_and_tilt.z,
        accel_and_tilt.tilt,
    );
    (*dev).raw_state.accelerometer_x = accel_and_tilt.x as int16_t;
    (*dev).raw_state.accelerometer_y = accel_and_tilt.y as int16_t;
    (*dev).raw_state.accelerometer_z = accel_and_tilt.z as int16_t;
    (*dev)
        .raw_state
        .tilt_angle = (accel_and_tilt.tilt as int8_t as libc::c_int * 2 as libc::c_int)
        as int8_t;
    return get_reply((*dev).usb_audio.dev, ctx);
}
pub unsafe extern "C" fn freenect_update_tilt_state(
    mut dev: *mut freenect_device,
) -> libc::c_int {
    let mut ctx: *mut freenect_context = (*dev).parent;
    if (*dev).motor_control_with_audio_enabled != 0 {
        return update_tilt_state_alt(dev);
    }
    if (*ctx).enabled_subdevices as libc::c_uint
        & FREENECT_DEVICE_MOTOR as libc::c_int as libc::c_uint == 0
    {
        return 0 as libc::c_int;
    }
    let mut buf: [uint8_t; 10] = [0; 10];
    let mut ux: uint16_t = 0;
    let mut uy: uint16_t = 0;
    let mut uz: uint16_t = 0;
    let mut ret: libc::c_int = fnusb_control(
        &mut (*dev).usb_motor,
        0xc0 as libc::c_int as uint8_t,
        0x32 as libc::c_int as uint8_t,
        0 as libc::c_int as uint16_t,
        0 as libc::c_int as uint16_t,
        buf.as_mut_ptr(),
        10 as libc::c_int as uint16_t,
    );
    if ret != 10 as libc::c_int {
        fn_log(
            ctx,
            FREENECT_LOG_ERROR,
            b"Error in accelerometer reading, libusb_control_transfer returned %d\n\0"
                as *const u8 as *const libc::c_char,
            ret,
        );
        return if ret < 0 as libc::c_int { ret } else { -(1 as libc::c_int) };
    }
    ux = ((buf[2 as libc::c_int as usize] as uint16_t as libc::c_int) << 8 as libc::c_int
        | buf[3 as libc::c_int as usize] as libc::c_int) as uint16_t;
    uy = ((buf[4 as libc::c_int as usize] as uint16_t as libc::c_int) << 8 as libc::c_int
        | buf[5 as libc::c_int as usize] as libc::c_int) as uint16_t;
    uz = ((buf[6 as libc::c_int as usize] as uint16_t as libc::c_int) << 8 as libc::c_int
        | buf[7 as libc::c_int as usize] as libc::c_int) as uint16_t;
    (*dev).raw_state.accelerometer_x = ux as int16_t;
    (*dev).raw_state.accelerometer_y = uy as int16_t;
    (*dev).raw_state.accelerometer_z = uz as int16_t;
    (*dev).raw_state.tilt_angle = buf[8 as libc::c_int as usize] as int8_t;
    (*dev)
        .raw_state
        .tilt_status = buf[9 as libc::c_int as usize] as freenect_tilt_status_code;
    return ret;
}
pub unsafe extern "C" fn freenect_set_tilt_degs_alt(
    mut dev: *mut freenect_device,
    mut tilt_degrees: libc::c_int,
) -> libc::c_int {
    let mut ctx: *mut freenect_context = (*dev).parent;
    if tilt_degrees > 31 as libc::c_int || tilt_degrees < -(31 as libc::c_int) {
        fn_log(
            ctx,
            FREENECT_LOG_WARNING,
            b"set_tilt(): degrees %d out of safe range [-31, 31]\n\0" as *const u8
                as *const libc::c_char,
            tilt_degrees,
        );
        return -(1 as libc::c_int);
    }
    if ((*dev).usb_audio.dev).is_null() {
        fn_log(
            ctx,
            FREENECT_LOG_WARNING,
            b"Motor control failed: audio device missing\0" as *const u8
                as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    let mut cmd: fn_alt_motor_command = fn_alt_motor_command {
        magic: 0,
        tag: 0,
        arg1: 0,
        cmd: 0,
        arg2: 0,
    };
    cmd.magic = 0x6022009 as libc::c_int as uint32_t;
    let fresh1 = tag_seq;
    tag_seq = tag_seq + 1;
    cmd.tag = fresh1 as uint32_t;
    cmd.arg1 = 0 as libc::c_int as uint32_t;
    cmd.cmd = 0x803b as libc::c_int as uint32_t;
    cmd.arg2 = tilt_degrees as uint32_t;
    let mut transferred: libc::c_int = 0 as libc::c_int;
    let mut res: libc::c_int = 0 as libc::c_int;
    let mut buffer: [libc::c_uchar; 20] = [0; 20];
    memcpy(
        buffer.as_mut_ptr() as *mut libc::c_void,
        &mut cmd as *mut fn_alt_motor_command as *const libc::c_void,
        20 as libc::c_int as libc::c_ulong,
    );
    res = libusb_bulk_transfer(
        (*dev).usb_audio.dev,
        0x1 as libc::c_int as libc::c_uchar,
        buffer.as_mut_ptr(),
        20 as libc::c_int,
        &mut transferred,
        250 as libc::c_int as libc::c_uint,
    );
    if res != 0 as libc::c_int {
        fn_log(
            ctx,
            FREENECT_LOG_ERROR,
            b"freenect_set_tilt_alt(): libusb_bulk_transfer failed: %s (transferred = %d)\n\0"
                as *const u8 as *const libc::c_char,
            libusb_error_name(res),
            transferred,
        );
        return res;
    }
    return get_reply((*dev).usb_audio.dev, ctx);
}
pub unsafe extern "C" fn freenect_set_tilt_degs(
    mut dev: *mut freenect_device,
    mut angle: libc::c_double,
) -> libc::c_int {
    let mut ctx: *mut freenect_context = (*dev).parent;
    if (*dev).motor_control_with_audio_enabled != 0 {
        return freenect_set_tilt_degs_alt(dev, angle as libc::c_int);
    }
    if (*ctx).enabled_subdevices as libc::c_uint
        & FREENECT_DEVICE_MOTOR as libc::c_int as libc::c_uint == 0
    {
        return 0 as libc::c_int;
    }
    let mut ret: libc::c_int = 0;
    let mut empty: [uint8_t; 1] = [0; 1];
    angle = if angle < -(31 as libc::c_int) as libc::c_double {
        -(31 as libc::c_int) as libc::c_double
    } else if angle > 31 as libc::c_int as libc::c_double {
        31 as libc::c_int as libc::c_double
    } else {
        angle
    };
    angle = angle * 2 as libc::c_int as libc::c_double;
    ret = fnusb_control(
        &mut (*dev).usb_motor,
        0x40 as libc::c_int as uint8_t,
        0x31 as libc::c_int as uint8_t,
        angle as int16_t as uint16_t,
        0 as libc::c_int as uint16_t,
        empty.as_mut_ptr(),
        0 as libc::c_int as uint16_t,
    );
    return ret;
}
pub unsafe extern "C" fn fnusb_set_led_alt(
    mut dev: *mut libusb_device_handle,
    mut ctx: *mut freenect_context,
    mut state: freenect_led_options,
) -> libc::c_int {
    if state as libc::c_uint == LED_GREEN as libc::c_int as libc::c_uint {
        state = LED_ALT_SOLID_GREEN as libc::c_int as freenect_led_options;
    } else if state as libc::c_uint == LED_RED as libc::c_int as libc::c_uint {
        state = LED_ALT_SOLID_RED as libc::c_int as freenect_led_options;
    } else if state as libc::c_uint == LED_YELLOW as libc::c_int as libc::c_uint {
        state = LED_ALT_SOLID_GREEN as libc::c_int as freenect_led_options;
    } else if state as libc::c_uint == LED_OFF as libc::c_int as libc::c_uint {
        state = LED_ALT_OFF as libc::c_int as freenect_led_options;
    } else if state as libc::c_uint == LED_BLINK_GREEN as libc::c_int as libc::c_uint {
        state = LED_ALT_BLINK_GREEN as libc::c_int as freenect_led_options;
    } else {
        state = LED_GREEN;
    }
    let mut cmd: fn_alt_motor_command = fn_alt_motor_command {
        magic: 0,
        tag: 0,
        arg1: 0,
        cmd: 0,
        arg2: 0,
    };
    cmd.magic = 0x6022009 as libc::c_int as uint32_t;
    let fresh2 = tag_seq;
    tag_seq = tag_seq + 1;
    cmd.tag = fresh2 as uint32_t;
    cmd.arg1 = 0 as libc::c_int as uint32_t;
    cmd.cmd = 0x10 as libc::c_int as uint32_t;
    cmd.arg2 = state as int32_t as uint32_t;
    let mut buffer: [libc::c_uchar; 20] = [0; 20];
    memcpy(
        buffer.as_mut_ptr() as *mut libc::c_void,
        &mut cmd as *mut fn_alt_motor_command as *const libc::c_void,
        20 as libc::c_int as libc::c_ulong,
    );
    let mut transferred: libc::c_int = 0 as libc::c_int;
    let mut res: libc::c_int = libusb_bulk_transfer(
        dev,
        0x1 as libc::c_int as libc::c_uchar,
        buffer.as_mut_ptr(),
        20 as libc::c_int,
        &mut transferred,
        100 as libc::c_int as libc::c_uint,
    );
    if res != 0 as libc::c_int {
        fn_log(
            ctx,
            FREENECT_LOG_WARNING,
            b"fnusb_set_led_alt(): libusb_bulk_transfer failed: %s (transferred = %d)\n\0"
                as *const u8 as *const libc::c_char,
            libusb_error_name(res),
            transferred,
        );
        return res;
    }
    return get_reply(dev, ctx);
}
pub unsafe extern "C" fn freenect_set_led_alt(
    mut dev: *mut freenect_device,
    mut state: freenect_led_options,
) -> libc::c_int {
    let mut ctx: *mut freenect_context = (*dev).parent;
    if ((*dev).usb_audio.dev).is_null() {
        fn_log(
            ctx,
            FREENECT_LOG_WARNING,
            b"Motor control failed: audio device missing\0" as *const u8
                as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    return fnusb_set_led_alt((*dev).usb_audio.dev, ctx, state);
}
pub unsafe extern "C" fn freenect_set_led(
    mut dev: *mut freenect_device,
    mut option: freenect_led_options,
) -> libc::c_int {
    let mut ctx: *mut freenect_context = (*dev).parent;
    if (*dev).motor_control_with_audio_enabled != 0 {
        return freenect_set_led_alt(dev, option);
    }
    if (*ctx).enabled_subdevices as libc::c_uint
        & FREENECT_DEVICE_MOTOR as libc::c_int as libc::c_uint == 0
    {
        return 0 as libc::c_int;
    }
    let mut ret: libc::c_int = 0;
    let mut empty: [uint8_t; 1] = [0; 1];
    ret = fnusb_control(
        &mut (*dev).usb_motor,
        0x40 as libc::c_int as uint8_t,
        0x6 as libc::c_int as uint8_t,
        option as uint16_t,
        0 as libc::c_int as uint16_t,
        empty.as_mut_ptr(),
        0 as libc::c_int as uint16_t,
    );
    return ret;
}
pub unsafe extern "C" fn freenect_get_tilt_degs(
    mut state: *mut freenect_raw_tilt_state,
) -> libc::c_double {
    return (*state).tilt_angle as libc::c_double / 2.0f64;
}
pub unsafe extern "C" fn freenect_get_tilt_status(
    mut state: *mut freenect_raw_tilt_state,
) -> freenect_tilt_status_code {
    return (*state).tilt_status;
}
pub unsafe extern "C" fn freenect_get_mks_accel(
    mut state: *mut freenect_raw_tilt_state,
    mut x: *mut libc::c_double,
    mut y: *mut libc::c_double,
    mut z: *mut libc::c_double,
) {
    *x = (*state).accelerometer_x as libc::c_double
        / 819 as libc::c_int as libc::c_double * 9.80665f64;
    *y = (*state).accelerometer_y as libc::c_double
        / 819 as libc::c_int as libc::c_double * 9.80665f64;
    *z = (*state).accelerometer_z as libc::c_double
        / 819 as libc::c_int as libc::c_double * 9.80665f64;
}
