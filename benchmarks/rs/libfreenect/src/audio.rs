use ::libc;
extern "C" {
    pub type libusb_device_handle;
    pub type libusb_context;
    fn fnusb_stop_iso(dev: *mut fnusb_dev, strm: *mut fnusb_isoc_stream) -> libc::c_int;
    fn fnusb_start_iso(
        dev: *mut fnusb_dev,
        strm: *mut fnusb_isoc_stream,
        cb: fnusb_iso_cb,
        endpoint: libc::c_uchar,
        xfers: libc::c_int,
        pkts: libc::c_int,
        len: libc::c_int,
    ) -> libc::c_int;
    fn fn_log(
        ctx: *mut freenect_context,
        level: freenect_loglevel,
        fmt: *const libc::c_char,
        _: ...
    );
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
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
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
pub struct audio_in_block {
    pub magic: uint32_t,
    pub channel: uint16_t,
    pub len: uint16_t,
    pub window: uint16_t,
    pub unknown: uint16_t,
    pub samples: [int32_t; 0],
}
unsafe extern "C" fn prepare_iso_out_data(
    mut dev: *mut freenect_device,
    mut buffer: *mut uint8_t,
) {
    let mut stream: *mut audio_stream = &mut (*dev).audio;
    if !((*dev).audio_out_cb).is_some() {
        memset(
            buffer as *mut libc::c_void,
            0 as libc::c_int,
            76 as libc::c_int as libc::c_ulong,
        );
    }
    *(buffer as *mut uint16_t).offset(0 as libc::c_int as isize) = (*stream).out_window;
    *buffer.offset(2 as libc::c_int as isize) = (*stream).out_seq;
    if (*stream).out_window_parity as libc::c_int == 0 as libc::c_int {
        if ((*stream).out_counter_within_window as libc::c_int) < 4 as libc::c_int {
            *buffer
                .offset(
                    3 as libc::c_int as isize,
                ) = (((*stream).out_weird_timestamp as libc::c_int
                >> (*stream).out_counter_within_window as libc::c_int * 4 as libc::c_int
                & 0xf as libc::c_int) << 4 as libc::c_int | 0x5 as libc::c_int)
                as uint8_t;
        } else if ((*stream).out_counter_within_window as libc::c_int) < 8 as libc::c_int
        {
            *buffer
                .offset(
                    3 as libc::c_int as isize,
                ) = (((*stream).out_weird_timestamp as libc::c_int + 23 as libc::c_int
                >> ((*stream).out_counter_within_window as libc::c_int
                    - 4 as libc::c_int) * 4 as libc::c_int & 0xf as libc::c_int)
                << 4 as libc::c_int | 0x5 as libc::c_int) as uint8_t;
        } else {
            *buffer.offset(3 as libc::c_int as isize) = 0x1 as libc::c_int as uint8_t;
        }
    } else if ((*stream).out_counter_within_window as libc::c_int) < 4 as libc::c_int {
        *buffer
            .offset(
                3 as libc::c_int as isize,
            ) = (((*stream).out_weird_timestamp as libc::c_int
            >> (*stream).out_counter_within_window as libc::c_int * 4 as libc::c_int
            & 0xf as libc::c_int) << 4 as libc::c_int | 0x5 as libc::c_int) as uint8_t;
    } else {
        *buffer.offset(3 as libc::c_int as isize) = 0x1 as libc::c_int as uint8_t;
    }
    (*stream).out_seq = ((*stream).out_seq).wrapping_add(1);
    (*stream).out_seq;
    (*stream)
        .out_counter_within_window = ((*stream).out_counter_within_window)
        .wrapping_add(1);
    (*stream).out_counter_within_window;
    (*stream)
        .out_weird_timestamp = ((*stream).out_weird_timestamp as libc::c_int
        + if (*stream).out_window_parity as libc::c_int == 1 as libc::c_int {
            6 as libc::c_int
        } else {
            5 as libc::c_int
        }) as uint16_t;
    let mut current_block_28: u64;
    match (*stream).out_seq as libc::c_int {
        128 => {
            (*stream).out_seq = 0 as libc::c_int as uint8_t;
            current_block_28 = 1890451546187205982;
        }
        43 | 86 => {
            current_block_28 = 1890451546187205982;
        }
        _ => {
            current_block_28 = 15768484401365413375;
        }
    }
    match current_block_28 {
        1890451546187205982 => {
            (*stream).out_counter_within_window = 0 as libc::c_int as uint8_t;
            (*stream).out_window = ((*stream).out_window).wrapping_add(1);
            (*stream).out_window;
            (*stream).out_window_parity = ((*stream).out_window_parity).wrapping_add(1);
            (*stream).out_window_parity;
        }
        _ => {}
    }
    if (*stream).out_window_parity as libc::c_int == 3 as libc::c_int {
        (*stream).out_window_parity = 0 as libc::c_int as uint8_t;
    }
}
unsafe extern "C" fn iso_out_callback(
    mut dev: *mut freenect_device,
    mut pkt: *mut uint8_t,
    mut len: libc::c_int,
) {
    prepare_iso_out_data(dev, pkt);
}
unsafe extern "C" fn iso_in_callback(
    mut dev: *mut freenect_device,
    mut pkt: *mut uint8_t,
    mut len: libc::c_int,
) {
    let mut ctx: *mut freenect_context = (*dev).parent;
    if len == 524 as libc::c_int {
        let mut block: *mut audio_in_block = pkt as *mut audio_in_block;
        if (*block).magic != 0x80000080 as libc::c_uint {
            fn_log(
                ctx,
                FREENECT_LOG_ERROR,
                b"audio: invalid magic in iso IN packet: %08X\n\0" as *const u8
                    as *const libc::c_char,
                (*block).magic,
            );
            return;
        }
        if (*block).window as libc::c_int != (*dev).audio.in_window as libc::c_int {
            fn_log(
                ctx,
                FREENECT_LOG_SPEW,
                b"audio: IN window changed: was %04X now %04X\n\0" as *const u8
                    as *const libc::c_char,
                (*dev).audio.in_window as libc::c_int,
                (*block).window as libc::c_int,
            );
            if ((*dev).audio_in_cb).is_some() {
                ((*dev).audio_in_cb)
                    .unwrap()(
                    dev,
                    256 as libc::c_int,
                    (*dev).audio.mic_buffer[0 as libc::c_int as usize],
                    (*dev).audio.mic_buffer[1 as libc::c_int as usize],
                    (*dev).audio.mic_buffer[2 as libc::c_int as usize],
                    (*dev).audio.mic_buffer[3 as libc::c_int as usize],
                    (*dev).audio.cancelled_buffer,
                    (*dev).audio.in_unknown,
                );
            }
            let mut t: libc::c_int = 0;
            t = 0 as libc::c_int;
            while t < 10 as libc::c_int {
                if (*dev).audio.last_seen_window[t as usize] as libc::c_int
                    != (*dev).audio.in_window as libc::c_int
                {
                    fn_log(
                        ctx,
                        FREENECT_LOG_SPEW,
                        b"audio: did not receive data for channel 0x%02x\n\0"
                            as *const u8 as *const libc::c_char,
                        t + 1 as libc::c_int,
                    );
                }
                t += 1;
                t;
            }
            if (*block).window as libc::c_int - (*dev).audio.in_window as libc::c_int
                > 3 as libc::c_int
            {
                fn_log(
                    ctx,
                    FREENECT_LOG_SPEW,
                    b"audio: packet loss, dropped %d windows\n\0" as *const u8
                        as *const libc::c_char,
                    ((*block).window as libc::c_int
                        - (*dev).audio.in_window as libc::c_int - 3 as libc::c_int)
                        / 3 as libc::c_int,
                );
            }
            (*dev).audio.in_window = (*block).window;
        }
        match (*block).channel as libc::c_int {
            1 => {
                memcpy(
                    (*dev).audio.cancelled_buffer as *mut libc::c_void,
                    &mut (*block).samples as *mut [int32_t; 0] as *const libc::c_void,
                    512 as libc::c_int as libc::c_ulong,
                );
            }
            2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 => {
                if (*block).channel as libc::c_int & 1 as libc::c_int != 0 {
                    memcpy(
                        &mut *(*((*dev).audio.mic_buffer)
                            .as_mut_ptr()
                            .offset(
                                (((*block).channel as libc::c_int - 2 as libc::c_int)
                                    / 2 as libc::c_int) as isize,
                            ))
                            .offset(128 as libc::c_int as isize) as *mut int32_t
                            as *mut libc::c_void,
                        &mut (*block).samples as *mut [int32_t; 0]
                            as *const libc::c_void,
                        512 as libc::c_int as libc::c_ulong,
                    );
                } else {
                    memcpy(
                        (*dev)
                            .audio
                            .mic_buffer[(((*block).channel as libc::c_int
                            - 2 as libc::c_int) / 2 as libc::c_int) as usize]
                            as *mut libc::c_void,
                        &mut (*block).samples as *mut [int32_t; 0]
                            as *const libc::c_void,
                        512 as libc::c_int as libc::c_ulong,
                    );
                }
            }
            _ => {
                fn_log(
                    ctx,
                    FREENECT_LOG_ERROR,
                    b"audio: invalid channel in iso IN packet: %d\n\0" as *const u8
                        as *const libc::c_char,
                    (*block).channel as libc::c_int,
                );
            }
        }
        (*dev)
            .audio
            .last_seen_window[((*block).channel as libc::c_int - 1 as libc::c_int)
            as usize] = (*block).window;
    } else if !(len == 60 as libc::c_int) {
        if len != 0 as libc::c_int {
            fn_log(
                ctx,
                FREENECT_LOG_ERROR,
                b"audio: received an iso IN packet of strange length: %d\n\0"
                    as *const u8 as *const libc::c_char,
                len,
            );
        }
    }
}
pub unsafe extern "C" fn freenect_set_audio_in_callback(
    mut dev: *mut freenect_device,
    mut callback: freenect_audio_in_cb,
) {
    (*dev).audio_in_cb = callback;
}
pub unsafe extern "C" fn freenect_set_audio_out_callback(
    mut dev: *mut freenect_device,
    mut callback: freenect_audio_out_cb,
) {
    (*dev).audio_out_cb = callback;
}
pub unsafe extern "C" fn freenect_start_audio(
    mut dev: *mut freenect_device,
) -> libc::c_int {
    let mut ctx: *mut freenect_context = (*dev).parent;
    let mut res: libc::c_int = 0;
    if (*dev).audio.running != 0 {
        return -(1 as libc::c_int);
    }
    (*dev)
        .audio
        .audio_out_ring = malloc(
        (256 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<freenect_sample_51>() as libc::c_ulong),
    ) as *mut freenect_sample_51;
    memset(
        (*dev).audio.audio_out_ring as *mut libc::c_void,
        0 as libc::c_int,
        (256 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<freenect_sample_51>() as libc::c_ulong),
    );
    (*dev)
        .audio
        .cancelled_buffer = malloc(
        (256 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<int16_t>() as libc::c_ulong),
    ) as *mut int16_t;
    memset(
        (*dev).audio.cancelled_buffer as *mut libc::c_void,
        0 as libc::c_int,
        (256 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<int16_t>() as libc::c_ulong),
    );
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        (*dev)
            .audio
            .mic_buffer[i
            as usize] = malloc(
            (256 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<int32_t>() as libc::c_ulong),
        ) as *mut int32_t;
        memset(
            (*dev).audio.mic_buffer[i as usize] as *mut libc::c_void,
            0 as libc::c_int,
            (256 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<int32_t>() as libc::c_ulong),
        );
        i += 1;
        i;
    }
    (*dev).audio.in_unknown = malloc(48 as libc::c_int as libc::c_ulong);
    (*dev).audio.ring_reader_idx = 0 as libc::c_int;
    (*dev).audio.ring_writer_idx = 0 as libc::c_int;
    (*dev).audio.out_window = 0 as libc::c_int as uint16_t;
    (*dev).audio.out_seq = 0 as libc::c_int as uint8_t;
    (*dev).audio.out_counter_within_window = 0 as libc::c_int as uint8_t;
    (*dev).audio.out_weird_timestamp = 0 as libc::c_int as uint16_t;
    (*dev).audio.out_window_parity = 0 as libc::c_int as uint8_t;
    (*dev).audio.in_window = 0 as libc::c_int as uint16_t;
    (*dev).audio.in_counter = 0 as libc::c_int as uint8_t;
    i = 0 as libc::c_int;
    while i < 10 as libc::c_int {
        (*dev).audio.last_seen_window[i as usize] = 0 as libc::c_int as uint16_t;
        i += 1;
        i;
    }
    res = fnusb_start_iso(
        &mut (*dev).usb_audio,
        &mut (*dev).audio_in_isoc,
        Some(
            iso_in_callback
                as unsafe extern "C" fn(
                    *mut freenect_device,
                    *mut uint8_t,
                    libc::c_int,
                ) -> (),
        ),
        0x82 as libc::c_int as libc::c_uchar,
        16 as libc::c_int,
        16 as libc::c_int,
        524 as libc::c_int,
    );
    if res < 0 as libc::c_int {
        fn_log(
            ctx,
            FREENECT_LOG_ERROR,
            b"audio: failed to start isochronous IN stream: %d\n\0" as *const u8
                as *const libc::c_char,
            res,
        );
        return res;
    }
    res = fnusb_start_iso(
        &mut (*dev).usb_audio,
        &mut (*dev).audio_out_isoc,
        Some(
            iso_out_callback
                as unsafe extern "C" fn(
                    *mut freenect_device,
                    *mut uint8_t,
                    libc::c_int,
                ) -> (),
        ),
        0x2 as libc::c_int as libc::c_uchar,
        16 as libc::c_int,
        16 as libc::c_int,
        76 as libc::c_int,
    );
    if res < 0 as libc::c_int {
        fn_log(
            ctx,
            FREENECT_LOG_ERROR,
            b"audio: failed to start isochronous OUT stream: %d\n\0" as *const u8
                as *const libc::c_char,
            res,
        );
        return res;
    }
    (*dev).audio.running = 1 as libc::c_int;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn freenect_stop_audio(
    mut dev: *mut freenect_device,
) -> libc::c_int {
    let mut ctx: *mut freenect_context = (*dev).parent;
    let mut res: libc::c_int = 0;
    let mut ret: libc::c_int = 0 as libc::c_int;
    if (*dev).audio.running == 0 {
        return -(1 as libc::c_int);
    }
    (*dev).audio.running = 0 as libc::c_int;
    res = fnusb_stop_iso(&mut (*dev).usb_audio, &mut (*dev).audio_in_isoc);
    if res < 0 as libc::c_int {
        fn_log(
            ctx,
            FREENECT_LOG_ERROR,
            b"audio: failed to stop isochronous IN stream: %d\n\0" as *const u8
                as *const libc::c_char,
            res,
        );
        return res;
    }
    res = fnusb_stop_iso(&mut (*dev).usb_audio, &mut (*dev).audio_out_isoc);
    if res < 0 as libc::c_int {
        fn_log(
            ctx,
            FREENECT_LOG_ERROR,
            b"audio: failed to stop isochronous OUT stream: %d\n\0" as *const u8
                as *const libc::c_char,
            res,
        );
        return res;
    }
    if !((*dev).audio.audio_out_ring).is_null() {
        free((*dev).audio.audio_out_ring as *mut libc::c_void);
    }
    if !((*dev).audio.cancelled_buffer).is_null() {
        free((*dev).audio.cancelled_buffer as *mut libc::c_void);
    }
    if !((*dev).audio.in_unknown).is_null() {
        free((*dev).audio.in_unknown);
    }
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        if !((*dev).audio.mic_buffer[i as usize]).is_null() {
            free((*dev).audio.mic_buffer[i as usize] as *mut libc::c_void);
        }
        (*dev).audio.mic_buffer[i as usize] = 0 as *mut int32_t;
        i += 1;
        i;
    }
    (*dev).audio.audio_out_ring = 0 as *mut freenect_sample_51;
    (*dev).audio.cancelled_buffer = 0 as *mut int16_t;
    (*dev).audio.in_unknown = 0 as *mut libc::c_void;
    return ret;
}
