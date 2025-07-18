use ::libc;
extern "C" {
    pub type libusb_device_handle;
    pub type libusb_context;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn usleep(__useconds: __useconds_t) -> libc::c_int;
    fn fn_log(
        ctx: *mut freenect_context,
        level: freenect_loglevel,
        fmt: *const libc::c_char,
        _: ...
    );
    fn fnusb_control(
        dev: *mut fnusb_dev,
        bmRequestType: uint8_t,
        bRequest: uint8_t,
        wValue: uint16_t,
        wIndex: uint16_t,
        data: *mut uint8_t,
        wLength: uint16_t,
    ) -> libc::c_int;
}
pub type __int8_t = libc::c_schar;
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __useconds_t = libc::c_uint;
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
pub type freenect_flag = libc::c_uint;
pub const FREENECT_NEAR_MODE: freenect_flag = 262144;
pub const FREENECT_MIRROR_VIDEO: freenect_flag = 131072;
pub const FREENECT_MIRROR_DEPTH: freenect_flag = 65536;
pub const FREENECT_RAW_COLOR: freenect_flag = 16;
pub const FREENECT_AUTO_WHITE_BALANCE: freenect_flag = 2;
pub const FREENECT_AUTO_EXPOSURE: freenect_flag = 16384;
pub type freenect_flag_value = libc::c_uint;
pub const FREENECT_ON: freenect_flag_value = 1;
pub const FREENECT_OFF: freenect_flag_value = 0;
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
pub struct cam_hdr {
    pub magic: [uint8_t; 2],
    pub len: uint16_t,
    pub cmd: uint16_t,
    pub tag: uint16_t,
}
pub unsafe extern "C" fn register_for_flag(mut flag: libc::c_int) -> libc::c_int {
    match flag {
        65536 => return 0x17 as libc::c_int,
        131072 => return 0x47 as libc::c_int,
        _ => return -(1 as libc::c_int),
    };
}
pub unsafe extern "C" fn freenect_set_flag(
    mut dev: *mut freenect_device,
    mut flag: freenect_flag,
    mut value: freenect_flag_value,
) -> libc::c_int {
    let mut ctx: *mut freenect_context = (*dev).parent;
    if flag as libc::c_uint == FREENECT_NEAR_MODE as libc::c_int as libc::c_uint {
        if (*dev).usb_cam.PID != 0x2bf as libc::c_int {
            fn_log(
                ctx,
                FREENECT_LOG_WARNING,
                b"Near mode is only supported by K4W\0" as *const u8
                    as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
        if value as libc::c_uint == FREENECT_ON as libc::c_int as libc::c_uint {
            let mut ret: libc::c_int = write_register(
                dev,
                0x15 as libc::c_int as uint16_t,
                0x7 as libc::c_int as uint16_t,
            );
            if ret < 0 as libc::c_int {
                return ret;
            }
            usleep(100000 as libc::c_int as __useconds_t);
            return write_register(
                dev,
                0x2ef as libc::c_int as uint16_t,
                0 as libc::c_int as uint16_t,
            );
        } else {
            let mut ret_0: libc::c_int = write_register(
                dev,
                0x15 as libc::c_int as uint16_t,
                0x1e as libc::c_int as uint16_t,
            );
            if ret_0 < 0 as libc::c_int {
                return ret_0;
            }
            usleep(100000 as libc::c_int as __useconds_t);
            return write_register(
                dev,
                0x2ef as libc::c_int as uint16_t,
                0x190 as libc::c_int as uint16_t,
            );
        }
    }
    if flag as libc::c_uint >= ((1 as libc::c_int) << 16 as libc::c_int) as libc::c_uint
    {
        let mut reg: libc::c_int = register_for_flag(flag as libc::c_int);
        if reg < 0 as libc::c_int {
            return reg;
        }
        return write_register(dev, reg as uint16_t, value as uint16_t);
    }
    let mut cmos_value: uint16_t = read_cmos_register(
        dev,
        0x106 as libc::c_int as uint16_t,
    );
    if cmos_value as libc::c_int == 65535 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if value as libc::c_uint == FREENECT_ON as libc::c_int as libc::c_uint {
        cmos_value = (cmos_value as libc::c_uint | flag as libc::c_uint) as uint16_t;
    } else {
        cmos_value = (cmos_value as libc::c_uint & !(flag as libc::c_uint)) as uint16_t;
    }
    return write_cmos_register(dev, 0x106 as libc::c_int as uint16_t, cmos_value);
}
pub unsafe extern "C" fn freenect_get_ir_brightness(
    mut dev: *mut freenect_device,
) -> libc::c_int {
    let mut ctx: *mut freenect_context = (*dev).parent;
    let brightness: uint16_t = read_register(dev, 0x15 as libc::c_int as uint16_t);
    if brightness as libc::c_int == 65535 as libc::c_int {
        fn_log(
            ctx,
            FREENECT_LOG_WARNING,
            b"Failed to get IR brightness!\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    return brightness as libc::c_int;
}
pub unsafe extern "C" fn freenect_set_ir_brightness(
    mut dev: *mut freenect_device,
    mut brightness: uint16_t,
) -> libc::c_int {
    let mut ctx: *mut freenect_context = (*dev).parent;
    if (brightness as libc::c_int) < 1 as libc::c_int {
        brightness = 1 as libc::c_int as uint16_t;
    }
    if brightness as libc::c_int > 50 as libc::c_int {
        brightness = 50 as libc::c_int as uint16_t;
    }
    let ret: libc::c_int = write_register(
        dev,
        0x15 as libc::c_int as uint16_t,
        brightness,
    );
    if ret < 0 as libc::c_int {
        fn_log(
            ctx,
            FREENECT_LOG_WARNING,
            b"Failed to set IR brightness\0" as *const u8 as *const libc::c_char,
        );
    }
    return ret;
}
pub unsafe extern "C" fn send_cmd(
    mut dev: *mut freenect_device,
    mut cmd: uint16_t,
    mut cmdbuf: *mut libc::c_void,
    mut cmd_len: libc::c_uint,
    mut replybuf: *mut libc::c_void,
    mut reply_len: libc::c_int,
) -> libc::c_int {
    let mut ctx: *mut freenect_context = (*dev).parent;
    let mut res: libc::c_int = 0;
    let mut actual_len: libc::c_int = 0;
    let mut obuf: [uint8_t; 1024] = [0; 1024];
    let mut ibuf: [uint8_t; 512] = [0; 512];
    let mut chdr: *mut cam_hdr = obuf.as_mut_ptr() as *mut cam_hdr;
    let mut rhdr: *mut cam_hdr = ibuf.as_mut_ptr() as *mut cam_hdr;
    if cmd_len & 1 as libc::c_int as libc::c_uint != 0
        || cmd_len as libc::c_ulong
            > (0x400 as libc::c_int as libc::c_ulong)
                .wrapping_sub(::std::mem::size_of::<cam_hdr>() as libc::c_ulong)
    {
        fn_log(
            ctx,
            FREENECT_LOG_ERROR,
            b"send_cmd: Invalid command length (0x%x)\n\0" as *const u8
                as *const libc::c_char,
            cmd_len,
        );
        return -(1 as libc::c_int);
    }
    (*chdr).magic[0 as libc::c_int as usize] = 0x47 as libc::c_int as uint8_t;
    (*chdr).magic[1 as libc::c_int as usize] = 0x4d as libc::c_int as uint8_t;
    (*chdr).cmd = cmd;
    (*chdr).tag = (*dev).cam_tag;
    (*chdr).len = cmd_len.wrapping_div(2 as libc::c_int as libc::c_uint) as uint16_t;
    memcpy(
        obuf
            .as_mut_ptr()
            .offset(::std::mem::size_of::<cam_hdr>() as libc::c_ulong as isize)
            as *mut libc::c_void,
        cmdbuf,
        cmd_len as libc::c_ulong,
    );
    res = fnusb_control(
        &mut (*dev).usb_cam,
        0x40 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
        0 as libc::c_int as uint16_t,
        0 as libc::c_int as uint16_t,
        obuf.as_mut_ptr(),
        (cmd_len as libc::c_ulong)
            .wrapping_add(::std::mem::size_of::<cam_hdr>() as libc::c_ulong) as uint16_t,
    );
    fn_log(
        ctx,
        FREENECT_LOG_SPEW,
        b"send_cmd: cmd=%04x tag=%04x len=%04x: %d\n\0" as *const u8
            as *const libc::c_char,
        cmd as libc::c_int,
        (*dev).cam_tag as libc::c_int,
        cmd_len,
        res,
    );
    if res < 0 as libc::c_int {
        fn_log(
            ctx,
            FREENECT_LOG_ERROR,
            b"send_cmd: Output control transfer failed (%d)\n\0" as *const u8
                as *const libc::c_char,
            res,
        );
        return res;
    }
    loop {
        actual_len = fnusb_control(
            &mut (*dev).usb_cam,
            0xc0 as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
            0 as libc::c_int as uint16_t,
            0 as libc::c_int as uint16_t,
            ibuf.as_mut_ptr(),
            0x200 as libc::c_int as uint16_t,
        );
        fn_log(
            ctx,
            FREENECT_LOG_FLOOD,
            b"send_cmd: actual length = %d\n\0" as *const u8 as *const libc::c_char,
            actual_len,
        );
        usleep(1 as libc::c_int as __useconds_t);
        if !(actual_len == 0 as libc::c_int || actual_len == 0x200 as libc::c_int) {
            break;
        }
    }
    fn_log(
        ctx,
        FREENECT_LOG_SPEW,
        b"Control reply: %d\n\0" as *const u8 as *const libc::c_char,
        res,
    );
    if actual_len < ::std::mem::size_of::<cam_hdr>() as libc::c_ulong as libc::c_int {
        fn_log(
            ctx,
            FREENECT_LOG_ERROR,
            b"send_cmd: Input control transfer failed (%d)\n\0" as *const u8
                as *const libc::c_char,
            res,
        );
        return res;
    }
    actual_len = (actual_len as libc::c_ulong)
        .wrapping_sub(::std::mem::size_of::<cam_hdr>() as libc::c_ulong) as libc::c_int
        as libc::c_int;
    if (*rhdr).magic[0 as libc::c_int as usize] as libc::c_int != 0x52 as libc::c_int
        || (*rhdr).magic[1 as libc::c_int as usize] as libc::c_int != 0x42 as libc::c_int
    {
        fn_log(
            ctx,
            FREENECT_LOG_ERROR,
            b"send_cmd: Bad magic %02x %02x\n\0" as *const u8 as *const libc::c_char,
            (*rhdr).magic[0 as libc::c_int as usize] as libc::c_int,
            (*rhdr).magic[1 as libc::c_int as usize] as libc::c_int,
        );
        return -(1 as libc::c_int);
    }
    if (*rhdr).cmd as libc::c_int != (*chdr).cmd as libc::c_int {
        fn_log(
            ctx,
            FREENECT_LOG_ERROR,
            b"send_cmd: Bad cmd %02x != %02x\n\0" as *const u8 as *const libc::c_char,
            (*rhdr).cmd as libc::c_int,
            (*chdr).cmd as libc::c_int,
        );
        return -(1 as libc::c_int);
    }
    if (*rhdr).tag as libc::c_int != (*chdr).tag as libc::c_int {
        fn_log(
            ctx,
            FREENECT_LOG_ERROR,
            b"send_cmd: Bad tag %04x != %04x\n\0" as *const u8 as *const libc::c_char,
            (*rhdr).tag as libc::c_int,
            (*chdr).tag as libc::c_int,
        );
        return -(1 as libc::c_int);
    }
    if (*rhdr).len as libc::c_int != actual_len / 2 as libc::c_int {
        fn_log(
            ctx,
            FREENECT_LOG_ERROR,
            b"send_cmd: Bad len %04x != %04x\n\0" as *const u8 as *const libc::c_char,
            (*rhdr).len as libc::c_int,
            actual_len / 2 as libc::c_int,
        );
        return -(1 as libc::c_int);
    }
    if actual_len > reply_len {
        fn_log(
            ctx,
            FREENECT_LOG_WARNING,
            b"send_cmd: Data buffer is %d bytes long, but got %d bytes\n\0" as *const u8
                as *const libc::c_char,
            reply_len,
            actual_len,
        );
        memcpy(
            replybuf,
            ibuf
                .as_mut_ptr()
                .offset(::std::mem::size_of::<cam_hdr>() as libc::c_ulong as isize)
                as *const libc::c_void,
            reply_len as libc::c_ulong,
        );
    } else {
        memcpy(
            replybuf,
            ibuf
                .as_mut_ptr()
                .offset(::std::mem::size_of::<cam_hdr>() as libc::c_ulong as isize)
                as *const libc::c_void,
            actual_len as libc::c_ulong,
        );
    }
    (*dev).cam_tag = ((*dev).cam_tag).wrapping_add(1);
    (*dev).cam_tag;
    return actual_len;
}
pub unsafe extern "C" fn read_register(
    mut dev: *mut freenect_device,
    mut reg: uint16_t,
) -> uint16_t {
    let mut ctx: *mut freenect_context = (*dev).parent;
    let mut reply: [uint16_t; 2] = [0; 2];
    let mut cmd: uint16_t = reg;
    let mut res: libc::c_int = send_cmd(
        dev,
        0x2 as libc::c_int as uint16_t,
        &mut cmd as *mut uint16_t as *mut libc::c_void,
        2 as libc::c_int as libc::c_uint,
        reply.as_mut_ptr() as *mut libc::c_void,
        4 as libc::c_int,
    );
    if res < 0 as libc::c_int {
        fn_log(
            ctx,
            FREENECT_LOG_ERROR,
            b"read_register: send_cmd() failed: %d\n\0" as *const u8
                as *const libc::c_char,
            res,
        );
        return 65535 as libc::c_int as uint16_t;
    }
    if res != 4 as libc::c_int {
        fn_log(
            ctx,
            FREENECT_LOG_WARNING,
            b"read_register: send_cmd() returned %d [%04x %04x], 0000 expected\n\0"
                as *const u8 as *const libc::c_char,
            res,
            reply[0 as libc::c_int as usize] as libc::c_int,
            reply[1 as libc::c_int as usize] as libc::c_int,
        );
    }
    fn_log(
        ctx,
        FREENECT_LOG_DEBUG,
        b"read_register: 0x%04x => 0x%04x\n\0" as *const u8 as *const libc::c_char,
        reg as libc::c_int,
        reply[1 as libc::c_int as usize] as libc::c_int,
    );
    return reply[1 as libc::c_int as usize];
}
pub unsafe extern "C" fn write_register(
    mut dev: *mut freenect_device,
    mut reg: uint16_t,
    mut data: uint16_t,
) -> libc::c_int {
    let mut ctx: *mut freenect_context = (*dev).parent;
    let mut reply: [uint16_t; 2] = [0; 2];
    let mut cmd: [uint16_t; 2] = [0; 2];
    cmd[0 as libc::c_int as usize] = reg;
    cmd[1 as libc::c_int as usize] = data;
    fn_log(
        ctx,
        FREENECT_LOG_DEBUG,
        b"write_register: 0x%04x <= 0x%02x\n\0" as *const u8 as *const libc::c_char,
        reg as libc::c_int,
        data as libc::c_int,
    );
    let mut res: libc::c_int = send_cmd(
        dev,
        0x3 as libc::c_int as uint16_t,
        cmd.as_mut_ptr() as *mut libc::c_void,
        4 as libc::c_int as libc::c_uint,
        reply.as_mut_ptr() as *mut libc::c_void,
        4 as libc::c_int,
    );
    if res < 0 as libc::c_int {
        fn_log(
            ctx,
            FREENECT_LOG_ERROR,
            b"write_register: send_cmd() returned %d\n\0" as *const u8
                as *const libc::c_char,
            res,
        );
        return res;
    }
    if res != 2 as libc::c_int {
        fn_log(
            ctx,
            FREENECT_LOG_WARNING,
            b"write_register: send_cmd() returned %d [%04x %04x], 0000 expected\n\0"
                as *const u8 as *const libc::c_char,
            res,
            reply[0 as libc::c_int as usize] as libc::c_int,
            reply[1 as libc::c_int as usize] as libc::c_int,
        );
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn read_cmos_register(
    mut dev: *mut freenect_device,
    mut reg: uint16_t,
) -> uint16_t {
    let mut ctx: *mut freenect_context = (*dev).parent;
    let mut replybuf: [uint16_t; 512] = [0; 512];
    let mut cmdbuf: [uint16_t; 3] = [0; 3];
    cmdbuf[0 as libc::c_int as usize] = 1 as libc::c_int as uint16_t;
    cmdbuf[1 as libc::c_int
        as usize] = (reg as libc::c_int & 0x7fff as libc::c_int) as uint16_t;
    cmdbuf[2 as libc::c_int as usize] = 0 as libc::c_int as uint16_t;
    let mut res: libc::c_int = send_cmd(
        dev,
        0x95 as libc::c_int as uint16_t,
        cmdbuf.as_mut_ptr() as *mut libc::c_void,
        6 as libc::c_int as libc::c_uint,
        replybuf.as_mut_ptr() as *mut libc::c_void,
        6 as libc::c_int,
    );
    if res < 0 as libc::c_int {
        fn_log(
            ctx,
            FREENECT_LOG_ERROR,
            b"read_cmos_register: send_cmd() returned %d\n\0" as *const u8
                as *const libc::c_char,
            res,
        );
        return 65535 as libc::c_int as uint16_t;
    }
    fn_log(
        ctx,
        FREENECT_LOG_DEBUG,
        b"read_cmos_register: 0x%04x => 0x%04x\n\0" as *const u8 as *const libc::c_char,
        reg as libc::c_int,
        replybuf[2 as libc::c_int as usize] as libc::c_int,
    );
    return replybuf[2 as libc::c_int as usize];
}
pub unsafe extern "C" fn write_cmos_register(
    mut dev: *mut freenect_device,
    mut reg: uint16_t,
    mut value: uint16_t,
) -> libc::c_int {
    let mut ctx: *mut freenect_context = (*dev).parent;
    let mut replybuf: [uint16_t; 512] = [0; 512];
    let mut cmdbuf: [uint16_t; 3] = [0; 3];
    cmdbuf[0 as libc::c_int as usize] = 1 as libc::c_int as uint16_t;
    cmdbuf[1 as libc::c_int
        as usize] = (reg as libc::c_int | 0x8000 as libc::c_int) as uint16_t;
    cmdbuf[2 as libc::c_int as usize] = value;
    fn_log(
        ctx,
        FREENECT_LOG_DEBUG,
        b"write_cmos_register: 0x%04x <= 0x%04x\n\0" as *const u8 as *const libc::c_char,
        reg as libc::c_int,
        value as libc::c_int,
    );
    let mut res: libc::c_int = send_cmd(
        dev,
        0x95 as libc::c_int as uint16_t,
        cmdbuf.as_mut_ptr() as *mut libc::c_void,
        6 as libc::c_int as libc::c_uint,
        replybuf.as_mut_ptr() as *mut libc::c_void,
        6 as libc::c_int,
    );
    if res < 0 as libc::c_int {
        fn_log(
            ctx,
            FREENECT_LOG_ERROR,
            b"write_cmos_register: send_cmd() returned %d\n\0" as *const u8
                as *const libc::c_char,
            res,
        );
    }
    return res;
}
