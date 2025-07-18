use ::libc;
extern "C" {
    pub type libusb_device_handle;
    pub type libusb_context;
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fnusb_bulk(
        dev: *mut fnusb_dev,
        endpoint: uint8_t,
        data: *mut uint8_t,
        len: libc::c_int,
        transferred: *mut libc::c_int,
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
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __errno_location() -> *mut libc::c_int;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fseek(
        __stream: *mut FILE,
        __off: libc::c_long,
        __whence: libc::c_int,
    ) -> libc::c_int;
    fn ftell(__stream: *mut FILE) -> libc::c_long;
    fn rewind(__stream: *mut FILE);
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
}
pub type __int8_t = libc::c_schar;
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
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
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bootloader_command {
    pub magic: uint32_t,
    pub tag: uint32_t,
    pub bytes: uint32_t,
    pub cmd: uint32_t,
    pub addr: uint32_t,
    pub unk: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct firmware_header {
    pub magic: uint32_t,
    pub ver_minor: uint16_t,
    pub ver_major: uint16_t,
    pub ver_release: uint16_t,
    pub ver_patch: uint16_t,
    pub base_addr: uint32_t,
    pub size: uint32_t,
    pub entry_addr: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cemdloader_command {
    pub magic: uint32_t,
    pub tag: uint32_t,
    pub arg1: uint32_t,
    pub cmd: uint32_t,
    pub arg2: uint32_t,
    pub zeros: [uint32_t; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bootloader_status_code {
    pub magic: uint32_t,
    pub tag: uint32_t,
    pub status: uint32_t,
}
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
pub type _IO_lock_t = ();
unsafe extern "C" fn dump_bl_cmd(
    mut ctx: *mut freenect_context,
    mut cmd: bootloader_command,
) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 24 as libc::c_int {
        fn_log(
            ctx,
            FREENECT_LOG_INFO,
            b"%02X \0" as *const u8 as *const libc::c_char,
            *(&mut cmd as *mut bootloader_command as *mut libc::c_uchar)
                .offset(i as isize) as libc::c_int,
        );
        i += 1;
        i;
    }
    fn_log(ctx, FREENECT_LOG_INFO, b"\n\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn dump_cemd_cmd(
    mut ctx: *mut freenect_context,
    mut cmd: cemdloader_command,
) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 24 as libc::c_int {
        fn_log(
            ctx,
            FREENECT_LOG_INFO,
            b"%02X \0" as *const u8 as *const libc::c_char,
            *(&mut cmd as *mut cemdloader_command as *mut libc::c_uchar)
                .offset(i as isize) as libc::c_int,
        );
        i += 1;
        i;
    }
    fn_log(
        ctx,
        FREENECT_LOG_INFO,
        b"(%d more zeros)\n\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<cemdloader_command>() as libc::c_ulong)
            .wrapping_sub(24 as libc::c_int as libc::c_ulong) as libc::c_int,
    );
}
unsafe extern "C" fn get_reply(mut dev: *mut fnusb_dev) -> libc::c_int {
    let mut ctx: *mut freenect_context = (*(*dev).parent).parent;
    let mut dump: [libc::c_uchar; 512] = [0; 512];
    let mut buffer: bootloader_status_code = bootloader_status_code {
        magic: 0,
        tag: 0,
        status: 0,
    };
    let mut res: libc::c_int = 0;
    let mut transferred: libc::c_int = 0;
    res = fnusb_bulk(
        dev,
        0x81 as libc::c_int as uint8_t,
        dump.as_mut_ptr(),
        512 as libc::c_int,
        &mut transferred,
    );
    if res != 0 as libc::c_int
        || transferred as libc::c_ulong
            != ::std::mem::size_of::<bootloader_status_code>() as libc::c_ulong
    {
        fn_log(
            ctx,
            FREENECT_LOG_ERROR,
            b"Error reading reply: %d\ttransferred: %d (expected %d)\n\0" as *const u8
                as *const libc::c_char,
            res,
            transferred,
            ::std::mem::size_of::<bootloader_status_code>() as libc::c_ulong
                as libc::c_int,
        );
        return res;
    }
    memcpy(
        &mut buffer as *mut bootloader_status_code as *mut libc::c_void,
        dump.as_mut_ptr() as *const libc::c_void,
        ::std::mem::size_of::<bootloader_status_code>() as libc::c_ulong,
    );
    if buffer.magic != 0xa6fe000 as libc::c_int as libc::c_uint {
        fn_log(
            ctx,
            FREENECT_LOG_ERROR,
            b"Error reading reply: invalid magic %08X\n\0" as *const u8
                as *const libc::c_char,
            buffer.magic,
        );
        return -(1 as libc::c_int);
    }
    if buffer.tag != (*(*dev).parent).audio_tag {
        fn_log(
            ctx,
            FREENECT_LOG_ERROR,
            b"Error reading reply: non-matching tag number %08X (expected %08X)\n\0"
                as *const u8 as *const libc::c_char,
            buffer.tag,
            (*(*dev).parent).audio_tag,
        );
        return -(1 as libc::c_int);
    }
    if buffer.status != 0 as libc::c_int as libc::c_uint {
        fn_log(
            ctx,
            FREENECT_LOG_ERROR,
            b"Notice reading reply: last uint32_t was nonzero: %d\n\0" as *const u8
                as *const libc::c_char,
            buffer.status,
        );
    }
    fn_log(
        ctx,
        FREENECT_LOG_INFO,
        b"Reading reply: \0" as *const u8 as *const libc::c_char,
    );
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < transferred {
        fn_log(
            ctx,
            FREENECT_LOG_INFO,
            b"%02X \0" as *const u8 as *const libc::c_char,
            *(&mut buffer as *mut bootloader_status_code as *mut libc::c_uchar)
                .offset(i as isize) as libc::c_int,
        );
        i += 1;
        i;
    }
    fn_log(ctx, FREENECT_LOG_INFO, b"\n\0" as *const u8 as *const libc::c_char);
    return res;
}
pub unsafe extern "C" fn upload_firmware(
    mut dev: *mut fnusb_dev,
    mut filename: *mut libc::c_char,
) -> libc::c_int {
    let mut ctx: *mut freenect_context = (*(*dev).parent).parent;
    let mut fw_filename: [libc::c_char; 1024] = [0; 1024];
    sprintf(
        fw_filename.as_mut_ptr(),
        b"/%s\0" as *const u8 as *const libc::c_char,
        filename,
    );
    let mut filenamelen: libc::c_int = strlen(fw_filename.as_mut_ptr()) as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut searchpathcount: libc::c_int = 0;
    let mut fw: *mut FILE = 0 as *mut FILE;
    let mut current_block_27: u64;
    i = 0 as libc::c_int;
    searchpathcount = 6 as libc::c_int;
    while fw.is_null() && i < searchpathcount {
        let mut fwfile: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut needs_free: libc::c_int = 0 as libc::c_int;
        match i {
            0 => {
                let mut envpath: *mut libc::c_char = getenv(
                    b"LIBFREENECT_FIRMWARE_PATH\0" as *const u8 as *const libc::c_char,
                );
                if envpath.is_null() {
                    current_block_27 = 10680521327981672866;
                } else {
                    let mut pathlen: libc::c_int = strlen(envpath) as libc::c_int;
                    fwfile = malloc(
                        (pathlen + filenamelen + 1 as libc::c_int) as libc::c_ulong,
                    ) as *mut libc::c_char;
                    strcpy(fwfile, envpath);
                    strcat(fwfile, fw_filename.as_mut_ptr());
                    needs_free = 1 as libc::c_int;
                    current_block_27 = 7205609094909031804;
                }
            }
            1 => {
                fwfile = malloc(2048 as libc::c_int as libc::c_ulong)
                    as *mut libc::c_char;
                needs_free = 1 as libc::c_int;
                sprintf(
                    fwfile,
                    b".%s\0" as *const u8 as *const libc::c_char,
                    fw_filename.as_mut_ptr(),
                );
                current_block_27 = 7205609094909031804;
            }
            2 => {
                let mut home: *mut libc::c_char = getenv(
                    b"HOME\0" as *const u8 as *const libc::c_char,
                );
                if home.is_null() {
                    current_block_27 = 10680521327981672866;
                } else {
                    let mut homelen: libc::c_int = strlen(home) as libc::c_int;
                    let mut dotfolder: *mut libc::c_char = b"/.libfreenect\0"
                        as *const u8 as *const libc::c_char as *mut libc::c_char;
                    let mut locallen: libc::c_int = strlen(dotfolder) as libc::c_int;
                    fwfile = malloc(
                        (homelen + locallen + filenamelen + 1 as libc::c_int)
                            as libc::c_ulong,
                    ) as *mut libc::c_char;
                    strcpy(fwfile, home);
                    strcat(fwfile, dotfolder);
                    strcat(fwfile, fw_filename.as_mut_ptr());
                    needs_free = 1 as libc::c_int;
                    current_block_27 = 7205609094909031804;
                }
            }
            3 => {
                fwfile = malloc(2048 as libc::c_int as libc::c_ulong)
                    as *mut libc::c_char;
                needs_free = 1 as libc::c_int;
                sprintf(
                    fwfile,
                    b"/usr/local/share/libfreenect%s\0" as *const u8
                        as *const libc::c_char,
                    fw_filename.as_mut_ptr(),
                );
                current_block_27 = 7205609094909031804;
            }
            4 => {
                fwfile = malloc(2048 as libc::c_int as libc::c_ulong)
                    as *mut libc::c_char;
                needs_free = 1 as libc::c_int;
                sprintf(
                    fwfile,
                    b"/usr/share/libfreenect%s\0" as *const u8 as *const libc::c_char,
                    fw_filename.as_mut_ptr(),
                );
                current_block_27 = 7205609094909031804;
            }
            5 => {
                fwfile = malloc(2048 as libc::c_int as libc::c_ulong)
                    as *mut libc::c_char;
                needs_free = 1 as libc::c_int;
                sprintf(
                    fwfile,
                    b"./../Resources%s\0" as *const u8 as *const libc::c_char,
                    fw_filename.as_mut_ptr(),
                );
                current_block_27 = 7205609094909031804;
            }
            _ => {
                current_block_27 = 7205609094909031804;
            }
        }
        match current_block_27 {
            7205609094909031804 => {
                fn_log(
                    ctx,
                    FREENECT_LOG_INFO,
                    b"Trying to open %s as firmware...\n\0" as *const u8
                        as *const libc::c_char,
                    fwfile,
                );
                fw = fopen(fwfile, b"rb\0" as *const u8 as *const libc::c_char);
                if needs_free != 0 {
                    free(fwfile as *mut libc::c_void);
                }
            }
            _ => {}
        }
        i += 1;
        i;
    }
    if fw.is_null() {
        fn_log(
            ctx,
            FREENECT_LOG_ERROR,
            b"upload_firmware: failed to find firmware file.\n\0" as *const u8
                as *const libc::c_char,
        );
        return -*__errno_location();
    }
    fseek(fw, 0 as libc::c_int as libc::c_long, 2 as libc::c_int);
    let mut fw_num_bytes: libc::c_int = ftell(fw) as libc::c_int;
    rewind(fw);
    if fw_num_bytes <= 0 as libc::c_int {
        fn_log(
            ctx,
            FREENECT_LOG_ERROR,
            b"upload_firmware: failed to find file with any data.\n\0" as *const u8
                as *const libc::c_char,
        );
        return -*__errno_location();
    }
    let mut fw_bytes: *mut libc::c_uchar = malloc(fw_num_bytes as libc::c_ulong)
        as *mut libc::c_uchar;
    let mut numRead: libc::c_int = fread(
        fw_bytes as *mut libc::c_void,
        1 as libc::c_int as libc::c_ulong,
        fw_num_bytes as libc::c_ulong,
        fw,
    ) as libc::c_int;
    fw_num_bytes = numRead;
    let mut retVal: libc::c_int = upload_firmware_from_memory(
        dev,
        fw_bytes,
        fw_num_bytes as libc::c_uint,
    );
    fclose(fw);
    fw = 0 as *mut FILE;
    return retVal;
}
pub unsafe extern "C" fn upload_firmware_from_memory(
    mut dev: *mut fnusb_dev,
    mut fw_from_mem: *mut libc::c_uchar,
    mut fw_size_in_btyes: libc::c_uint,
) -> libc::c_int {
    let mut ctx: *mut freenect_context = (*(*dev).parent).parent;
    let mut bootcmd: bootloader_command = bootloader_command {
        magic: 0,
        tag: 0,
        bytes: 0,
        cmd: 0,
        addr: 0,
        unk: 0,
    };
    memset(
        &mut bootcmd as *mut bootloader_command as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<bootloader_command>() as libc::c_ulong,
    );
    bootcmd.magic = 0x6022009 as libc::c_int as uint32_t;
    let mut res: libc::c_int = 0;
    let mut transferred: libc::c_int = 0;
    let mut fwheader: firmware_header = firmware_header {
        magic: 0,
        ver_minor: 0,
        ver_major: 0,
        ver_release: 0,
        ver_patch: 0,
        base_addr: 0,
        size: 0,
        entry_addr: 0,
    };
    let mut read: libc::c_int = 0 as libc::c_int;
    let mut bytesLeft: libc::c_int = fw_size_in_btyes as libc::c_int;
    let mut readPtr: *mut libc::c_uchar = &mut *fw_from_mem
        .offset(0 as libc::c_int as isize) as *mut libc::c_uchar;
    if (fw_size_in_btyes as libc::c_ulong)
        < ::std::mem::size_of::<firmware_header>() as libc::c_ulong
    {
        fn_log(
            ctx,
            FREENECT_LOG_ERROR,
            b"upload_firmware: firmware image too small, has no header?\n\0" as *const u8
                as *const libc::c_char,
        );
        return -*__errno_location();
    }
    memcpy(
        &mut fwheader as *mut firmware_header as *mut libc::c_void,
        readPtr as *const libc::c_void,
        ::std::mem::size_of::<firmware_header>() as libc::c_ulong,
    );
    fwheader.magic = fwheader.magic;
    fwheader.ver_major = fwheader.ver_major;
    fwheader.ver_minor = fwheader.ver_minor;
    fwheader.ver_release = fwheader.ver_release;
    fwheader.ver_patch = fwheader.ver_patch;
    fwheader.base_addr = fwheader.base_addr;
    fwheader.size = fwheader.size;
    fwheader.entry_addr = fwheader.entry_addr;
    fn_log(
        ctx,
        FREENECT_LOG_INFO,
        b"Found firmware image:\n\0" as *const u8 as *const libc::c_char,
    );
    fn_log(
        ctx,
        FREENECT_LOG_INFO,
        b"\tmagic        %08X\n\0" as *const u8 as *const libc::c_char,
        fwheader.magic,
    );
    fn_log(
        ctx,
        FREENECT_LOG_INFO,
        b"\tversion      %02d.%02d.%02d.%02d\n\0" as *const u8 as *const libc::c_char,
        fwheader.ver_major as libc::c_int,
        fwheader.ver_minor as libc::c_int,
        fwheader.ver_release as libc::c_int,
        fwheader.ver_patch as libc::c_int,
    );
    fn_log(
        ctx,
        FREENECT_LOG_INFO,
        b"\tbase address 0x%08x\n\0" as *const u8 as *const libc::c_char,
        fwheader.base_addr,
    );
    fn_log(
        ctx,
        FREENECT_LOG_INFO,
        b"\tsize         0x%08x\n\0" as *const u8 as *const libc::c_char,
        fwheader.size,
    );
    fn_log(
        ctx,
        FREENECT_LOG_INFO,
        b"\tentry point  0x%08x\n\0" as *const u8 as *const libc::c_char,
        fwheader.entry_addr,
    );
    let mut addr: uint32_t = fwheader.base_addr;
    let mut page: [libc::c_uchar; 16384] = [0; 16384];
    let mut readIndex: libc::c_int = 0 as libc::c_int;
    let mut total_bytes_sent: libc::c_int = 0 as libc::c_int;
    loop {
        read = (if 0x4000 as libc::c_int as libc::c_uint
            > (fwheader.size).wrapping_sub(total_bytes_sent as libc::c_uint)
        {
            (fwheader.size).wrapping_sub(total_bytes_sent as libc::c_uint)
        } else {
            0x4000 as libc::c_int as libc::c_uint
        }) as libc::c_int;
        if read > bytesLeft {
            read = bytesLeft;
        }
        if read <= 0 as libc::c_int {
            break;
        }
        memcpy(
            page.as_mut_ptr() as *mut libc::c_void,
            &mut *readPtr.offset(readIndex as isize) as *mut libc::c_uchar
                as *const libc::c_void,
            read as libc::c_ulong,
        );
        readIndex += read;
        bytesLeft -= read;
        bootcmd.tag = (*(*dev).parent).audio_tag;
        bootcmd.bytes = read as uint32_t;
        bootcmd.cmd = 0x3 as libc::c_int as uint32_t;
        bootcmd.addr = addr;
        fn_log(
            ctx,
            FREENECT_LOG_INFO,
            b"About to send: \0" as *const u8 as *const libc::c_char,
        );
        dump_bl_cmd(ctx, bootcmd);
        res = fnusb_bulk(
            dev,
            1 as libc::c_int as uint8_t,
            &mut bootcmd as *mut bootloader_command as *mut libc::c_uchar,
            ::std::mem::size_of::<bootloader_command>() as libc::c_ulong as libc::c_int,
            &mut transferred,
        );
        if res != 0 as libc::c_int
            || transferred as libc::c_ulong
                != ::std::mem::size_of::<bootloader_command>() as libc::c_ulong
        {
            fn_log(
                ctx,
                FREENECT_LOG_ERROR,
                b"upload_firmware(): Error: res: %d\ttransferred: %d (expected %d)\n\0"
                    as *const u8 as *const libc::c_char,
                res,
                transferred,
                ::std::mem::size_of::<bootloader_command>() as libc::c_ulong
                    as libc::c_int,
            );
            return -(1 as libc::c_int);
        }
        let mut bytes_sent: libc::c_int = 0 as libc::c_int;
        while bytes_sent < read {
            let mut to_send: libc::c_int = if read - bytes_sent > 512 as libc::c_int {
                512 as libc::c_int
            } else {
                read - bytes_sent
            };
            res = fnusb_bulk(
                dev,
                1 as libc::c_int as uint8_t,
                &mut *page.as_mut_ptr().offset(bytes_sent as isize),
                to_send,
                &mut transferred,
            );
            if res != 0 as libc::c_int || transferred != to_send {
                fn_log(
                    ctx,
                    FREENECT_LOG_ERROR,
                    b"upload_firmware(): Error: res: %d\ttransferred: %d (expected %d)\n\0"
                        as *const u8 as *const libc::c_char,
                    res,
                    transferred,
                    to_send,
                );
                return -(1 as libc::c_int);
            }
            bytes_sent += to_send;
            total_bytes_sent += to_send;
        }
        res = get_reply(dev);
        addr = (addr as libc::c_uint).wrapping_add(read as uint32_t) as uint32_t
            as uint32_t;
        (*(*dev).parent).audio_tag = ((*(*dev).parent).audio_tag).wrapping_add(1);
        (*(*dev).parent).audio_tag;
        if !(read > 0 as libc::c_int) {
            break;
        }
    }
    if total_bytes_sent as libc::c_uint != fwheader.size {
        fn_log(
            ctx,
            FREENECT_LOG_ERROR,
            b"upload_firmware: firmware image declared %d bytes, but file only contained %d bytes\n\0"
                as *const u8 as *const libc::c_char,
            fwheader.size,
            total_bytes_sent,
        );
        return -(1 as libc::c_int);
    }
    bootcmd.tag = (*(*dev).parent).audio_tag;
    bootcmd.bytes = 0 as libc::c_int as uint32_t;
    bootcmd.cmd = 0x4 as libc::c_int as uint32_t;
    bootcmd.addr = fwheader.entry_addr;
    dump_bl_cmd(ctx, bootcmd);
    res = fnusb_bulk(
        dev,
        1 as libc::c_int as uint8_t,
        &mut bootcmd as *mut bootloader_command as *mut libc::c_uchar,
        ::std::mem::size_of::<bootloader_command>() as libc::c_ulong as libc::c_int,
        &mut transferred,
    );
    if res != 0 as libc::c_int
        || transferred as libc::c_ulong
            != ::std::mem::size_of::<bootloader_command>() as libc::c_ulong
    {
        fn_log(
            ctx,
            FREENECT_LOG_ERROR,
            b"upload_firmware(): Error: res: %d\ttransferred: %d (expected %d)\n\0"
                as *const u8 as *const libc::c_char,
            res,
            transferred,
            ::std::mem::size_of::<bootloader_command>() as libc::c_ulong as libc::c_int,
        );
        return -(1 as libc::c_int);
    }
    res = get_reply(dev);
    (*(*dev).parent).audio_tag = ((*(*dev).parent).audio_tag).wrapping_add(1);
    (*(*dev).parent).audio_tag;
    fn_log(
        ctx,
        FREENECT_LOG_INFO,
        b"Firmware successfully uploaded and launched.  Device will disconnect and reenumerate.\n\0"
            as *const u8 as *const libc::c_char,
    );
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn upload_cemd_data(mut dev: *mut fnusb_dev) -> libc::c_int {
    let mut ctx: *mut freenect_context = (*(*dev).parent).parent;
    let mut cemdcmd: cemdloader_command = cemdloader_command {
        magic: 0,
        tag: 0,
        arg1: 0,
        cmd: 0,
        arg2: 0,
        zeros: [0; 8],
    };
    memset(
        &mut cemdcmd as *mut cemdloader_command as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<cemdloader_command>() as libc::c_ulong,
    );
    cemdcmd.magic = 0x6022009 as libc::c_int as uint32_t;
    cemdcmd.tag = (*(*dev).parent).audio_tag;
    cemdcmd.arg1 = 0 as libc::c_int as uint32_t;
    cemdcmd.cmd = 0x133 as libc::c_int as uint32_t;
    cemdcmd.arg2 = 0x64014 as libc::c_int as uint32_t;
    fn_log(
        ctx,
        FREENECT_LOG_INFO,
        b"Starting CEMD data upload:\n\0" as *const u8 as *const libc::c_char,
    );
    let mut res: libc::c_int = 0;
    let mut transferred: libc::c_int = 0;
    res = fnusb_bulk(
        dev,
        1 as libc::c_int as uint8_t,
        &mut cemdcmd as *mut cemdloader_command as *mut libc::c_uchar,
        ::std::mem::size_of::<cemdloader_command>() as libc::c_ulong as libc::c_int,
        &mut transferred,
    );
    if res != 0 as libc::c_int
        || transferred as libc::c_ulong
            != ::std::mem::size_of::<cemdloader_command>() as libc::c_ulong
    {
        fn_log(
            ctx,
            FREENECT_LOG_ERROR,
            b"Error: res: %d\ttransferred: %d (expected %d)\n\0" as *const u8
                as *const libc::c_char,
            res,
            transferred,
            ::std::mem::size_of::<cemdloader_command>() as libc::c_ulong as libc::c_int,
        );
        return -(1 as libc::c_int);
    }
    res = get_reply(dev);
    (*(*dev).parent).audio_tag = ((*(*dev).parent).audio_tag).wrapping_add(1);
    (*(*dev).parent).audio_tag;
    let mut cemd_filename: *const libc::c_char = b"cemd_data.bin\0" as *const u8
        as *const libc::c_char;
    let mut cf: *mut FILE = fopen(
        cemd_filename,
        b"r\0" as *const u8 as *const libc::c_char,
    );
    if cf.is_null() {
        fn_log(
            ctx,
            FREENECT_LOG_ERROR,
            b"upload_cemd_data: Failed to open %s: error %d\0" as *const u8
                as *const libc::c_char,
            cemd_filename,
            *__errno_location(),
        );
        return *__errno_location();
    }
    let mut addr: uint32_t = 0 as libc::c_int as uint32_t;
    let mut read: libc::c_int = 0 as libc::c_int;
    let mut page: [libc::c_uchar; 16384] = [0; 16384];
    loop {
        read = fread(
            page.as_mut_ptr() as *mut libc::c_void,
            1 as libc::c_int as libc::c_ulong,
            0x4000 as libc::c_int as libc::c_ulong,
            cf,
        ) as libc::c_int;
        if read <= 0 as libc::c_int {
            break;
        }
        cemdcmd.tag = (*(*dev).parent).audio_tag;
        cemdcmd.arg1 = read as uint32_t;
        cemdcmd.cmd = 0x134 as libc::c_int as uint32_t;
        cemdcmd.arg2 = addr;
        fn_log(
            ctx,
            FREENECT_LOG_INFO,
            b"About to send: \0" as *const u8 as *const libc::c_char,
        );
        dump_cemd_cmd(ctx, cemdcmd);
        res = fnusb_bulk(
            dev,
            1 as libc::c_int as uint8_t,
            &mut cemdcmd as *mut cemdloader_command as *mut libc::c_uchar,
            ::std::mem::size_of::<cemdloader_command>() as libc::c_ulong as libc::c_int,
            &mut transferred,
        );
        if res != 0 as libc::c_int
            || transferred as libc::c_ulong
                != ::std::mem::size_of::<cemdloader_command>() as libc::c_ulong
        {
            fn_log(
                ctx,
                FREENECT_LOG_ERROR,
                b"Error: res: %d\ttransferred: %d (expected %d)\n\0" as *const u8
                    as *const libc::c_char,
                res,
                transferred,
                ::std::mem::size_of::<cemdloader_command>() as libc::c_ulong
                    as libc::c_int,
            );
            return -(1 as libc::c_int);
        }
        let mut bytes_sent: libc::c_int = 0 as libc::c_int;
        while bytes_sent < read {
            let mut to_send: libc::c_int = if read - bytes_sent > 512 as libc::c_int {
                512 as libc::c_int
            } else {
                read - bytes_sent
            };
            res = fnusb_bulk(
                dev,
                1 as libc::c_int as uint8_t,
                &mut *page.as_mut_ptr().offset(bytes_sent as isize),
                to_send,
                &mut transferred,
            );
            if res != 0 as libc::c_int || transferred != to_send {
                fn_log(
                    ctx,
                    FREENECT_LOG_ERROR,
                    b"Error: res: %d\ttransferred: %d (expected %d)\n\0" as *const u8
                        as *const libc::c_char,
                    res,
                    transferred,
                    to_send,
                );
                return -(1 as libc::c_int);
            }
            bytes_sent += to_send;
        }
        res = get_reply(dev);
        addr = (addr as libc::c_uint).wrapping_add(read as uint32_t) as uint32_t
            as uint32_t;
        (*(*dev).parent).audio_tag = ((*(*dev).parent).audio_tag).wrapping_add(1);
        (*(*dev).parent).audio_tag;
        if !(read > 0 as libc::c_int) {
            break;
        }
    }
    fclose(cf);
    cf = 0 as *mut FILE;
    cemdcmd.tag = (*(*dev).parent).audio_tag;
    cemdcmd.arg1 = 0 as libc::c_int as uint32_t;
    cemdcmd.cmd = 0x135 as libc::c_int as uint32_t;
    cemdcmd.arg2 = 0x64000 as libc::c_int as uint32_t;
    fn_log(
        ctx,
        FREENECT_LOG_INFO,
        b"Finishing CEMD data upload...\n\0" as *const u8 as *const libc::c_char,
    );
    res = fnusb_bulk(
        dev,
        1 as libc::c_int as uint8_t,
        &mut cemdcmd as *mut cemdloader_command as *mut libc::c_uchar,
        ::std::mem::size_of::<cemdloader_command>() as libc::c_ulong as libc::c_int,
        &mut transferred,
    );
    if res != 0 as libc::c_int
        || transferred as libc::c_ulong
            != ::std::mem::size_of::<cemdloader_command>() as libc::c_ulong
    {
        fn_log(
            ctx,
            FREENECT_LOG_ERROR,
            b"upload_cemd_data(): Error: res: %d\ttransferred: %d (expected %d)\n\0"
                as *const u8 as *const libc::c_char,
            res,
            transferred,
            ::std::mem::size_of::<cemdloader_command>() as libc::c_ulong as libc::c_int,
        );
        return -(1 as libc::c_int);
    }
    res = get_reply(dev);
    (*(*dev).parent).audio_tag = ((*(*dev).parent).audio_tag).wrapping_add(1);
    (*(*dev).parent).audio_tag;
    fn_log(
        ctx,
        FREENECT_LOG_INFO,
        b"CEMD data uploaded successfully.\n\0" as *const u8 as *const libc::c_char,
    );
    return 0 as libc::c_int;
}
