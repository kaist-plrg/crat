use ::libc;
extern "C" {
    pub type libusb_device_handle;
    pub type libusb_context;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
}
pub type __int8_t = libc::c_schar;
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type int8_t = __int8_t;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
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
pub type size_t = libc::c_ulong;
unsafe extern "C" fn freenect_init_depth_to_rgb(
    mut depth_to_rgb: *mut int32_t,
    mut zpi: *mut freenect_zero_plane_info,
) {
    let mut i: uint32_t = 0;
    let mut x_scale: uint32_t = (1280 as libc::c_int / 640 as libc::c_int) as uint32_t;
    let mut pixel_size: libc::c_double = 1.0f64
        / ((*zpi).reference_pixel_size * x_scale as libc::c_float
            * 10 as libc::c_int as libc::c_float) as libc::c_double;
    let mut pixels_between_rgb_and_ir_cmos: libc::c_double = (*zpi).dcmos_rcmos_dist
        as libc::c_double * pixel_size * 10 as libc::c_int as libc::c_double;
    let mut reference_distance_in_pixels: libc::c_double = (*zpi).reference_distance
        as libc::c_double * pixel_size * 10 as libc::c_int as libc::c_double;
    memset(
        depth_to_rgb as *mut libc::c_void,
        0 as libc::c_int,
        (10000 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<int32_t>() as libc::c_ulong),
    );
    i = 0 as libc::c_int as uint32_t;
    while i < 10000 as libc::c_int as libc::c_uint {
        let mut current_depth_in_pixels: libc::c_double = i as libc::c_double
            * pixel_size;
        *depth_to_rgb
            .offset(
                i as isize,
            ) = ((pixels_between_rgb_and_ir_cmos
            * (current_depth_in_pixels - reference_distance_in_pixels)
            / current_depth_in_pixels + 0.375f64) * 256 as libc::c_int as libc::c_double)
            as int32_t;
        i = i.wrapping_add(1);
        i;
    }
}
#[inline]
unsafe extern "C" fn unpack_8_pixels(mut raw: *mut uint8_t, mut frame: *mut uint16_t) {
    let mut baseMask: uint16_t = 0x7ff as libc::c_int as uint16_t;
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
        | r2 as libc::c_int >> 2 as libc::c_int) & baseMask as libc::c_int) as uint16_t;
    *frame
        .offset(
            2 as libc::c_int as isize,
        ) = (((r2 as libc::c_int) << 9 as libc::c_int
        | (r3 as libc::c_int) << 1 as libc::c_int
        | r4 as libc::c_int >> 7 as libc::c_int) & baseMask as libc::c_int) as uint16_t;
    *frame
        .offset(
            3 as libc::c_int as isize,
        ) = (((r4 as libc::c_int) << 4 as libc::c_int
        | r5 as libc::c_int >> 4 as libc::c_int) & baseMask as libc::c_int) as uint16_t;
    *frame
        .offset(
            4 as libc::c_int as isize,
        ) = (((r5 as libc::c_int) << 7 as libc::c_int
        | r6 as libc::c_int >> 1 as libc::c_int) & baseMask as libc::c_int) as uint16_t;
    *frame
        .offset(
            5 as libc::c_int as isize,
        ) = (((r6 as libc::c_int) << 10 as libc::c_int
        | (r7 as libc::c_int) << 2 as libc::c_int
        | r8 as libc::c_int >> 6 as libc::c_int) & baseMask as libc::c_int) as uint16_t;
    *frame
        .offset(
            6 as libc::c_int as isize,
        ) = (((r8 as libc::c_int) << 5 as libc::c_int
        | r9 as libc::c_int >> 3 as libc::c_int) & baseMask as libc::c_int) as uint16_t;
    *frame
        .offset(
            7 as libc::c_int as isize,
        ) = (((r9 as libc::c_int) << 8 as libc::c_int | r10 as libc::c_int)
        & baseMask as libc::c_int) as uint16_t;
}
pub unsafe extern "C" fn freenect_apply_registration(
    mut dev: *mut freenect_device,
    mut input: *mut uint8_t,
    mut output_mm: *mut uint16_t,
    mut unpacked: bool,
) -> libc::c_int {
    let mut reg: *mut freenect_registration = &mut (*dev).registration;
    let mut i: size_t = 0;
    let mut wipe: *mut size_t = output_mm as *mut size_t;
    i = 0 as libc::c_int as size_t;
    while i
        < ((640 as libc::c_int * 480 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<uint16_t>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<size_t>() as libc::c_ulong)
    {
        *wipe.offset(i as isize) = 0 as libc::c_int as size_t;
        i = i.wrapping_add(1);
        i;
    }
    let mut unpack: [uint16_t; 8] = [0; 8];
    let mut target_offset: uint32_t = (480 as libc::c_int
        * (*reg).reg_pad_info.start_lines as libc::c_int) as uint32_t;
    let mut x: uint32_t = 0;
    let mut y: uint32_t = 0;
    let mut source_index: uint32_t = 8 as libc::c_int as uint32_t;
    y = 0 as libc::c_int as uint32_t;
    while y < 480 as libc::c_int as libc::c_uint {
        x = 0 as libc::c_int as uint32_t;
        while x < 640 as libc::c_int as libc::c_uint {
            let mut metric_depth: uint16_t = 0;
            if unpacked {
                let mut buf_index: uint32_t = y
                    .wrapping_mul(640 as libc::c_int as libc::c_uint)
                    .wrapping_add(x);
                metric_depth = *((*reg).raw_to_mm_shift)
                    .offset(
                        *(input as *mut uint16_t).offset(buf_index as isize) as isize,
                    );
            } else {
                if source_index == 8 as libc::c_int as libc::c_uint {
                    unpack_8_pixels(input, unpack.as_mut_ptr());
                    source_index = 0 as libc::c_int as uint32_t;
                    input = input.offset(11 as libc::c_int as isize);
                }
                let fresh0 = source_index;
                source_index = source_index.wrapping_add(1);
                metric_depth = *((*reg).raw_to_mm_shift)
                    .offset(unpack[fresh0 as usize] as isize);
            }
            if !(metric_depth as libc::c_int == 0 as libc::c_int) {
                if !(metric_depth as libc::c_int >= 10000 as libc::c_int) {
                    let mut reg_index: uint32_t = if 0 as libc::c_int != 0 {
                        y.wrapping_add(1 as libc::c_int as libc::c_uint)
                            .wrapping_mul(640 as libc::c_int as libc::c_uint)
                            .wrapping_sub(x)
                            .wrapping_sub(1 as libc::c_int as libc::c_uint)
                    } else {
                        y.wrapping_mul(640 as libc::c_int as libc::c_uint)
                            .wrapping_add(x)
                    };
                    let mut nx: uint32_t = (((*((*reg).registration_table)
                        .offset(reg_index as isize))[0 as libc::c_int as usize]
                        + *((*reg).depth_to_rgb_shift).offset(metric_depth as isize))
                        / 256 as libc::c_int) as uint32_t;
                    let mut ny: uint32_t = (*((*reg).registration_table)
                        .offset(reg_index as isize))[1 as libc::c_int as usize]
                        as uint32_t;
                    if !(nx >= 640 as libc::c_int as libc::c_uint) {
                        let mut target_index: uint32_t = (if 0 as libc::c_int != 0 {
                            ny.wrapping_add(1 as libc::c_int as libc::c_uint)
                                .wrapping_mul(640 as libc::c_int as libc::c_uint)
                                .wrapping_sub(nx)
                                .wrapping_sub(1 as libc::c_int as libc::c_uint)
                        } else {
                            ny.wrapping_mul(640 as libc::c_int as libc::c_uint)
                                .wrapping_add(nx)
                        })
                            .wrapping_sub(target_offset);
                        let mut current_depth: uint16_t = *output_mm
                            .offset(target_index as isize);
                        if current_depth as libc::c_int == 0 as libc::c_int
                            || current_depth as libc::c_int > metric_depth as libc::c_int
                        {
                            *output_mm.offset(target_index as isize) = metric_depth;
                        }
                    }
                }
            }
            x = x.wrapping_add(1);
            x;
        }
        y = y.wrapping_add(1);
        y;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn freenect_apply_depth_to_mm(
    mut dev: *mut freenect_device,
    mut input_packed: *mut uint8_t,
    mut output_mm: *mut uint16_t,
) -> libc::c_int {
    let mut reg: *mut freenect_registration = &mut (*dev).registration;
    let mut unpack: [uint16_t; 8] = [0; 8];
    let mut x: uint32_t = 0;
    let mut y: uint32_t = 0;
    let mut source_index: uint32_t = 8 as libc::c_int as uint32_t;
    y = 0 as libc::c_int as uint32_t;
    while y < 480 as libc::c_int as libc::c_uint {
        x = 0 as libc::c_int as uint32_t;
        while x < 640 as libc::c_int as libc::c_uint {
            if source_index == 8 as libc::c_int as libc::c_uint {
                unpack_8_pixels(input_packed, unpack.as_mut_ptr());
                source_index = 0 as libc::c_int as uint32_t;
                input_packed = input_packed.offset(11 as libc::c_int as isize);
            }
            let fresh1 = source_index;
            source_index = source_index.wrapping_add(1);
            let mut metric_depth: uint16_t = *((*reg).raw_to_mm_shift)
                .offset(unpack[fresh1 as usize] as isize);
            *output_mm
                .offset(
                    y.wrapping_mul(640 as libc::c_int as libc::c_uint).wrapping_add(x)
                        as isize,
                ) = (if (metric_depth as libc::c_int) < 10000 as libc::c_int {
                metric_depth as libc::c_int
            } else {
                10000 as libc::c_int
            }) as uint16_t;
            x = x.wrapping_add(1);
            x;
        }
        y = y.wrapping_add(1);
        y;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn freenect_apply_depth_unpacked_to_mm(
    mut dev: *mut freenect_device,
    mut input: *mut uint16_t,
    mut output_mm: *mut uint16_t,
) -> libc::c_int {
    let mut reg: *mut freenect_registration = &mut (*dev).registration;
    let mut x: uint32_t = 0;
    let mut y: uint32_t = 0;
    y = 0 as libc::c_int as uint32_t;
    while y < 480 as libc::c_int as libc::c_uint {
        x = 0 as libc::c_int as uint32_t;
        while x < 640 as libc::c_int as libc::c_uint {
            let mut buf_index: uint32_t = y
                .wrapping_mul(640 as libc::c_int as libc::c_uint)
                .wrapping_add(x);
            let mut metric_depth: uint16_t = *((*reg).raw_to_mm_shift)
                .offset(*input.offset(buf_index as isize) as isize);
            *output_mm
                .offset(
                    buf_index as isize,
                ) = (if (metric_depth as libc::c_int) < 10000 as libc::c_int {
                metric_depth as libc::c_int
            } else {
                10000 as libc::c_int
            }) as uint16_t;
            x = x.wrapping_add(1);
            x;
        }
        y = y.wrapping_add(1);
        y;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn freenect_create_dxdy_tables(
    mut reg_x_table: *mut libc::c_double,
    mut reg_y_table: *mut libc::c_double,
    mut resolution_x: int32_t,
    mut resolution_y: int32_t,
    mut regdata: *mut freenect_reg_info,
) {
    let mut AX6: int64_t = (*regdata).ax as int64_t;
    let mut BX6: int64_t = (*regdata).bx as int64_t;
    let mut CX2: int64_t = (*regdata).cx as int64_t;
    let mut DX2: int64_t = (*regdata).dx as int64_t;
    let mut AY6: int64_t = (*regdata).ay as int64_t;
    let mut BY6: int64_t = (*regdata).by as int64_t;
    let mut CY2: int64_t = (*regdata).cy as int64_t;
    let mut DY2: int64_t = (*regdata).dy as int64_t;
    let mut dX0: int64_t = ((*regdata).dx_start << 13 as libc::c_int >> 4 as libc::c_int)
        as int64_t;
    let mut dY0: int64_t = ((*regdata).dy_start << 13 as libc::c_int >> 4 as libc::c_int)
        as int64_t;
    let mut dXdX0: int64_t = ((*regdata).dxdx_start << 11 as libc::c_int
        >> 3 as libc::c_int) as int64_t;
    let mut dXdY0: int64_t = ((*regdata).dxdy_start << 11 as libc::c_int
        >> 3 as libc::c_int) as int64_t;
    let mut dYdX0: int64_t = ((*regdata).dydx_start << 11 as libc::c_int
        >> 3 as libc::c_int) as int64_t;
    let mut dYdY0: int64_t = ((*regdata).dydy_start << 11 as libc::c_int
        >> 3 as libc::c_int) as int64_t;
    let mut dXdXdX0: int64_t = (((*regdata).dxdxdx_start << 5 as libc::c_int)
        << 3 as libc::c_int) as int64_t;
    let mut dYdXdX0: int64_t = (((*regdata).dydxdx_start << 5 as libc::c_int)
        << 3 as libc::c_int) as int64_t;
    let mut dYdXdY0: int64_t = (((*regdata).dydxdy_start << 5 as libc::c_int)
        << 3 as libc::c_int) as int64_t;
    let mut dXdXdY0: int64_t = (((*regdata).dxdxdy_start << 5 as libc::c_int)
        << 3 as libc::c_int) as int64_t;
    let mut dYdYdX0: int64_t = (((*regdata).dydydx_start << 5 as libc::c_int)
        << 3 as libc::c_int) as int64_t;
    let mut dYdYdY0: int64_t = (((*regdata).dydydy_start << 5 as libc::c_int)
        << 3 as libc::c_int) as int64_t;
    let mut row: int32_t = 0;
    let mut col: int32_t = 0;
    let mut tOffs: int32_t = 0 as libc::c_int;
    row = 0 as libc::c_int;
    while row < resolution_y {
        dXdXdX0 += CX2;
        dXdX0 += dYdXdX0 >> 8 as libc::c_int;
        dYdXdX0 += DX2;
        dX0 += dYdX0 >> 6 as libc::c_int;
        dYdX0 += dYdYdX0 >> 8 as libc::c_int;
        dYdYdX0 += BX6;
        dXdXdY0 += CY2;
        dXdY0 += dYdXdY0 >> 8 as libc::c_int;
        dYdXdY0 += DY2;
        dY0 += dYdY0 >> 6 as libc::c_int;
        dYdY0 += dYdYdY0 >> 8 as libc::c_int;
        dYdYdY0 += BY6;
        let mut coldXdXdY0: int64_t = dXdXdY0;
        let mut coldXdY0: int64_t = dXdY0;
        let mut coldY0: int64_t = dY0;
        let mut coldXdXdX0: int64_t = dXdXdX0;
        let mut coldXdX0: int64_t = dXdX0;
        let mut coldX0: int64_t = dX0;
        col = 0 as libc::c_int;
        while col < resolution_x {
            *reg_x_table
                .offset(
                    tOffs as isize,
                ) = coldX0 as libc::c_double
                * (1.0f64 / ((1 as libc::c_int) << 17 as libc::c_int) as libc::c_double);
            *reg_y_table
                .offset(
                    tOffs as isize,
                ) = coldY0 as libc::c_double
                * (1.0f64 / ((1 as libc::c_int) << 17 as libc::c_int) as libc::c_double);
            coldX0 += coldXdX0 >> 6 as libc::c_int;
            coldXdX0 += coldXdXdX0 >> 8 as libc::c_int;
            coldXdXdX0 += AX6;
            coldY0 += coldXdY0 >> 6 as libc::c_int;
            coldXdY0 += coldXdXdY0 >> 8 as libc::c_int;
            coldXdXdY0 += AY6;
            col += 1;
            col;
            tOffs += 1;
            tOffs;
        }
        row += 1;
        row;
    }
}
unsafe extern "C" fn freenect_init_registration_table(
    mut registration_table: *mut [int32_t; 2],
    mut reg_info: *mut freenect_reg_info,
) {
    let mut regtable_dx: *mut libc::c_double = malloc(
        ((640 as libc::c_int * 480 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    let mut regtable_dy: *mut libc::c_double = malloc(
        ((640 as libc::c_int * 480 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    memset(
        regtable_dx as *mut libc::c_void,
        0 as libc::c_int,
        ((640 as libc::c_int * 480 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    );
    memset(
        regtable_dy as *mut libc::c_void,
        0 as libc::c_int,
        ((640 as libc::c_int * 480 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    );
    let mut x: int32_t = 0;
    let mut y: int32_t = 0;
    let mut index: int32_t = 0 as libc::c_int;
    freenect_create_dxdy_tables(
        regtable_dx,
        regtable_dy,
        640 as libc::c_int,
        480 as libc::c_int,
        reg_info,
    );
    y = 0 as libc::c_int;
    while y < 480 as libc::c_int {
        x = 0 as libc::c_int;
        while x < 640 as libc::c_int {
            let mut new_x: libc::c_double = x as libc::c_double
                + *regtable_dx.offset(index as isize)
                + 1 as libc::c_int as libc::c_double;
            let mut new_y: libc::c_double = y as libc::c_double
                + *regtable_dy.offset(index as isize)
                + 1 as libc::c_int as libc::c_double;
            if new_x < 0 as libc::c_int as libc::c_double
                || new_y < 0 as libc::c_int as libc::c_double
                || new_x >= 640 as libc::c_int as libc::c_double
                || new_y >= 480 as libc::c_int as libc::c_double
            {
                new_x = (2 as libc::c_int * 640 as libc::c_int) as libc::c_double;
            }
            (*registration_table
                .offset(
                    index as isize,
                ))[0 as libc::c_int
                as usize] = (new_x * 256 as libc::c_int as libc::c_double) as int32_t;
            (*registration_table
                .offset(index as isize))[1 as libc::c_int as usize] = new_y as int32_t;
            x += 1;
            x;
            index += 1;
            index;
        }
        y += 1;
        y;
    }
    free(regtable_dx as *mut libc::c_void);
    free(regtable_dy as *mut libc::c_void);
}
static mut parameter_coefficient: libc::c_double = 4 as libc::c_int as libc::c_double;
static mut shift_scale: libc::c_double = 10 as libc::c_int as libc::c_double;
static mut pixel_size_factor: libc::c_double = 1 as libc::c_int as libc::c_double;
unsafe extern "C" fn freenect_raw_to_mm(
    mut raw: uint16_t,
    mut reg: *mut freenect_registration,
) -> uint16_t {
    let mut zpi: *mut freenect_zero_plane_info = &mut (*reg).zero_plane_info;
    let mut fixed_ref_x: libc::c_double = (raw as libc::c_int as libc::c_double
        - parameter_coefficient * (*reg).const_shift / pixel_size_factor)
        / parameter_coefficient - 0.375f64;
    let mut metric: libc::c_double = fixed_ref_x
        * (*zpi).reference_pixel_size as libc::c_double * pixel_size_factor;
    return (shift_scale
        * (metric * (*zpi).reference_distance as libc::c_double
            / ((*zpi).dcmos_emitter_dist as libc::c_double - metric)
            + (*zpi).reference_distance as libc::c_double)) as uint16_t;
}
unsafe extern "C" fn complete_tables(mut reg: *mut freenect_registration) {
    let mut i: uint16_t = 0;
    i = 0 as libc::c_int as uint16_t;
    while (i as libc::c_int) < 2048 as libc::c_int {
        *((*reg).raw_to_mm_shift).offset(i as isize) = freenect_raw_to_mm(i, reg);
        i = i.wrapping_add(1);
        i;
    }
    *((*reg).raw_to_mm_shift)
        .offset(2047 as libc::c_int as isize) = 0 as libc::c_int as uint16_t;
    freenect_init_depth_to_rgb((*reg).depth_to_rgb_shift, &mut (*reg).zero_plane_info);
    freenect_init_registration_table((*reg).registration_table, &mut (*reg).reg_info);
}
pub unsafe extern "C" fn freenect_camera_to_world(
    mut dev: *mut freenect_device,
    mut cx: libc::c_int,
    mut cy: libc::c_int,
    mut wz: libc::c_int,
    mut wx: *mut libc::c_double,
    mut wy: *mut libc::c_double,
) {
    let mut ref_pix_size: libc::c_double = (*dev)
        .registration
        .zero_plane_info
        .reference_pixel_size as libc::c_double;
    let mut ref_distance: libc::c_double = (*dev)
        .registration
        .zero_plane_info
        .reference_distance as libc::c_double;
    let mut factor: libc::c_double = 2 as libc::c_int as libc::c_double * ref_pix_size
        * wz as libc::c_double / ref_distance;
    *wx = (cx - 640 as libc::c_int / 2 as libc::c_int) as libc::c_double * factor;
    *wy = (cy - 480 as libc::c_int / 2 as libc::c_int) as libc::c_double * factor;
}
pub unsafe extern "C" fn freenect_map_rgb_to_depth(
    mut dev: *mut freenect_device,
    mut depth_mm: *mut uint16_t,
    mut rgb_raw: *mut uint8_t,
    mut rgb_registered: *mut uint8_t,
) {
    let mut target_offset: uint32_t = ((*dev).registration.reg_pad_info.start_lines
        as libc::c_int * 480 as libc::c_int) as uint32_t;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut map: *mut libc::c_int = malloc(
        ((480 as libc::c_int * 640 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    let mut zBuffer: *mut libc::c_ushort = malloc(
        ((480 as libc::c_int * 640 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_ushort>() as libc::c_ulong),
    ) as *mut libc::c_ushort;
    memset(
        zBuffer as *mut libc::c_void,
        0 as libc::c_int,
        ((640 as libc::c_int * 480 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_ushort>() as libc::c_ulong),
    );
    y = 0 as libc::c_int;
    while y < 480 as libc::c_int {
        x = 0 as libc::c_int;
        while x < 640 as libc::c_int {
            let mut index: uint32_t = (y * 640 as libc::c_int + x) as uint32_t;
            let mut cx: uint32_t = 0;
            let mut cy: uint32_t = 0;
            let mut cindex: uint32_t = 0;
            *map.offset(index as isize) = -(1 as libc::c_int);
            let mut wz: libc::c_int = *depth_mm.offset(index as isize) as libc::c_int;
            if !(wz == 0 as libc::c_int) {
                cx = (((*((*dev).registration.registration_table)
                    .offset(index as isize))[0 as libc::c_int as usize]
                    + *((*dev).registration.depth_to_rgb_shift).offset(wz as isize))
                    / 256 as libc::c_int) as uint32_t;
                cy = ((*((*dev).registration.registration_table)
                    .offset(index as isize))[1 as libc::c_int as usize] as libc::c_uint)
                    .wrapping_sub(target_offset);
                if !(cx >= 640 as libc::c_int as libc::c_uint) {
                    cindex = cy
                        .wrapping_mul(640 as libc::c_int as libc::c_uint)
                        .wrapping_add(cx);
                    *map.offset(index as isize) = cindex as libc::c_int;
                    if *zBuffer.offset(cindex as isize) as libc::c_int
                        == 0 as libc::c_int
                        || *zBuffer.offset(cindex as isize) as libc::c_int > wz
                    {
                        *zBuffer.offset(cindex as isize) = wz as libc::c_ushort;
                    }
                }
            }
            x += 1;
            x;
        }
        y += 1;
        y;
    }
    y = 0 as libc::c_int;
    while y < 480 as libc::c_int {
        x = 0 as libc::c_int;
        while x < 640 as libc::c_int {
            let mut index_0: uint32_t = (y * 640 as libc::c_int + x) as uint32_t;
            let mut cindex_0: uint32_t = *map.offset(index_0 as isize) as uint32_t;
            if cindex_0 == -(1 as libc::c_int) as libc::c_uint {
                index_0 = (index_0 as libc::c_uint)
                    .wrapping_mul(3 as libc::c_int as libc::c_uint) as uint32_t
                    as uint32_t;
                *rgb_registered
                    .offset(
                        index_0.wrapping_add(0 as libc::c_int as libc::c_uint) as isize,
                    ) = 0 as libc::c_int as uint8_t;
                *rgb_registered
                    .offset(
                        index_0.wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
                    ) = 0 as libc::c_int as uint8_t;
                *rgb_registered
                    .offset(
                        index_0.wrapping_add(2 as libc::c_int as libc::c_uint) as isize,
                    ) = 0 as libc::c_int as uint8_t;
            } else {
                let mut currentDepth: libc::c_ushort = *depth_mm
                    .offset(index_0 as isize);
                let mut minDepth: libc::c_ushort = *zBuffer.offset(cindex_0 as isize);
                if currentDepth as libc::c_int <= minDepth as libc::c_int {
                    index_0 = (index_0 as libc::c_uint)
                        .wrapping_mul(3 as libc::c_int as libc::c_uint) as uint32_t
                        as uint32_t;
                    cindex_0 = (cindex_0 as libc::c_uint)
                        .wrapping_mul(3 as libc::c_int as libc::c_uint) as uint32_t
                        as uint32_t;
                    *rgb_registered
                        .offset(
                            index_0.wrapping_add(0 as libc::c_int as libc::c_uint)
                                as isize,
                        ) = *rgb_raw
                        .offset(
                            cindex_0.wrapping_add(0 as libc::c_int as libc::c_uint)
                                as isize,
                        );
                    *rgb_registered
                        .offset(
                            index_0.wrapping_add(1 as libc::c_int as libc::c_uint)
                                as isize,
                        ) = *rgb_raw
                        .offset(
                            cindex_0.wrapping_add(1 as libc::c_int as libc::c_uint)
                                as isize,
                        );
                    *rgb_registered
                        .offset(
                            index_0.wrapping_add(2 as libc::c_int as libc::c_uint)
                                as isize,
                        ) = *rgb_raw
                        .offset(
                            cindex_0.wrapping_add(2 as libc::c_int as libc::c_uint)
                                as isize,
                        );
                }
            }
            x += 1;
            x;
        }
        y += 1;
        y;
    }
    free(zBuffer as *mut libc::c_void);
    free(map as *mut libc::c_void);
}
pub unsafe extern "C" fn freenect_init_registration(
    mut dev: *mut freenect_device,
) -> libc::c_int {
    let mut reg: *mut freenect_registration = &mut (*dev).registration;
    freenect_destroy_registration(&mut (*dev).registration);
    (*reg)
        .raw_to_mm_shift = malloc(
        (::std::mem::size_of::<uint16_t>() as libc::c_ulong)
            .wrapping_mul(2048 as libc::c_int as libc::c_ulong),
    ) as *mut uint16_t;
    (*reg)
        .depth_to_rgb_shift = malloc(
        (::std::mem::size_of::<int32_t>() as libc::c_ulong)
            .wrapping_mul(10000 as libc::c_int as libc::c_ulong),
    ) as *mut int32_t;
    (*reg)
        .registration_table = malloc(
        (::std::mem::size_of::<int32_t>() as libc::c_ulong)
            .wrapping_mul(640 as libc::c_int as libc::c_ulong)
            .wrapping_mul(480 as libc::c_int as libc::c_ulong)
            .wrapping_mul(2 as libc::c_int as libc::c_ulong),
    ) as *mut [int32_t; 2];
    complete_tables(reg);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn freenect_copy_registration(
    mut dev: *mut freenect_device,
) -> freenect_registration {
    let mut retval: freenect_registration = freenect_registration {
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
        raw_to_mm_shift: 0 as *mut uint16_t,
        depth_to_rgb_shift: 0 as *mut int32_t,
        registration_table: 0 as *mut [int32_t; 2],
    };
    retval.reg_info = (*dev).registration.reg_info;
    retval.reg_pad_info = (*dev).registration.reg_pad_info;
    retval.zero_plane_info = (*dev).registration.zero_plane_info;
    retval.const_shift = (*dev).registration.const_shift;
    retval
        .raw_to_mm_shift = malloc(
        (::std::mem::size_of::<uint16_t>() as libc::c_ulong)
            .wrapping_mul(2048 as libc::c_int as libc::c_ulong),
    ) as *mut uint16_t;
    retval
        .depth_to_rgb_shift = malloc(
        (::std::mem::size_of::<int32_t>() as libc::c_ulong)
            .wrapping_mul(10000 as libc::c_int as libc::c_ulong),
    ) as *mut int32_t;
    retval
        .registration_table = malloc(
        (::std::mem::size_of::<int32_t>() as libc::c_ulong)
            .wrapping_mul(640 as libc::c_int as libc::c_ulong)
            .wrapping_mul(480 as libc::c_int as libc::c_ulong)
            .wrapping_mul(2 as libc::c_int as libc::c_ulong),
    ) as *mut [int32_t; 2];
    complete_tables(&mut retval);
    return retval;
}
pub unsafe extern "C" fn freenect_destroy_registration(
    mut reg: *mut freenect_registration,
) -> libc::c_int {
    if !((*reg).raw_to_mm_shift).is_null() {
        free((*reg).raw_to_mm_shift as *mut libc::c_void);
        (*reg).raw_to_mm_shift = 0 as *mut uint16_t;
    }
    if !((*reg).depth_to_rgb_shift).is_null() {
        free((*reg).depth_to_rgb_shift as *mut libc::c_void);
        (*reg).depth_to_rgb_shift = 0 as *mut int32_t;
    }
    if !((*reg).registration_table).is_null() {
        free((*reg).registration_table as *mut libc::c_void);
        (*reg).registration_table = 0 as *mut [int32_t; 2];
    }
    return 0 as libc::c_int;
}
